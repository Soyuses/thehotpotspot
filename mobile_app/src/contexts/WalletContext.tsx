import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import AsyncStorage from '@react-native-async-storage/async-storage';
import { Alert } from 'react-native';
import { walletAPI } from '../services/api';

interface WalletBalance {
  stTokens: number;
  utTokens: number;
  totalValue: number; // in GEL
}

interface Transaction {
  id: string;
  type: 'st_received' | 'ut_earned' | 'st_transferred' | 'dividend_received';
  amount: number;
  description: string;
  timestamp: string;
  status: 'pending' | 'completed' | 'failed';
}

interface WalletContextType {
  balance: WalletBalance;
  transactions: Transaction[];
  isLoading: boolean;
  refreshBalance: () => Promise<void>;
  refreshTransactions: () => Promise<void>;
  claimCheck: (qrData: string) => Promise<boolean>;
  transferTokens: (toAddress: string, amount: number, type: 'st' | 'ut') => Promise<boolean>;
  getTransactionHistory: (limit?: number, offset?: number) => Promise<Transaction[]>;
}

const WalletContext = createContext<WalletContextType | undefined>(undefined);

export const useWallet = () => {
  const context = useContext(WalletContext);
  if (!context) {
    throw new Error('useWallet must be used within a WalletProvider');
  }
  return context;
};

interface WalletProviderProps {
  children: ReactNode;
}

export const WalletProvider: React.FC<WalletProviderProps> = ({ children }) => {
  const [balance, setBalance] = useState<WalletBalance>({
    stTokens: 0,
    utTokens: 0,
    totalValue: 0,
  });
  const [transactions, setTransactions] = useState<Transaction[]>([]);
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    loadStoredWalletData();
  }, []);

  const loadStoredWalletData = async () => {
    try {
      const storedBalance = await AsyncStorage.getItem('wallet_balance');
      const storedTransactions = await AsyncStorage.getItem('wallet_transactions');

      if (storedBalance) {
        setBalance(JSON.parse(storedBalance));
      }
      if (storedTransactions) {
        setTransactions(JSON.parse(storedTransactions));
      }
    } catch (error) {
      console.error('Error loading stored wallet data:', error);
    }
  };

  const refreshBalance = async (): Promise<void> => {
    try {
      setIsLoading(true);
      const response = await walletAPI.getBalance();
      
      if (response.success) {
        const newBalance = response.balance;
        setBalance(newBalance);
        await AsyncStorage.setItem('wallet_balance', JSON.stringify(newBalance));
      } else {
        console.error('Failed to refresh balance:', response.error);
      }
    } catch (error) {
      console.error('Error refreshing balance:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const refreshTransactions = async (): Promise<void> => {
    try {
      setIsLoading(true);
      const response = await walletAPI.getTransactions();
      
      if (response.success) {
        const newTransactions = response.transactions;
        setTransactions(newTransactions);
        await AsyncStorage.setItem('wallet_transactions', JSON.stringify(newTransactions));
      } else {
        console.error('Failed to refresh transactions:', response.error);
      }
    } catch (error) {
      console.error('Error refreshing transactions:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const claimCheck = async (qrData: string): Promise<boolean> => {
    try {
      setIsLoading(true);
      const response = await walletAPI.claimCheck(qrData);
      
      if (response.success) {
        Alert.alert(
          'Успешно!', 
          `Получено ${response.transferredTokens} ST токенов`
        );
        
        // Refresh balance and transactions
        await refreshBalance();
        await refreshTransactions();
        
        return true;
      } else {
        Alert.alert('Ошибка', response.error || 'Не удалось активировать чек');
        return false;
      }
    } catch (error) {
      console.error('Error claiming check:', error);
      Alert.alert('Ошибка', 'Произошла ошибка при активации чека');
      return false;
    } finally {
      setIsLoading(false);
    }
  };

  const transferTokens = async (
    toAddress: string, 
    amount: number, 
    type: 'st' | 'ut'
  ): Promise<boolean> => {
    try {
      setIsLoading(true);
      const response = await walletAPI.transferTokens(toAddress, amount, type);
      
      if (response.success) {
        Alert.alert('Успешно!', 'Токены успешно переведены');
        
        // Refresh balance and transactions
        await refreshBalance();
        await refreshTransactions();
        
        return true;
      } else {
        Alert.alert('Ошибка', response.error || 'Не удалось перевести токены');
        return false;
      }
    } catch (error) {
      console.error('Error transferring tokens:', error);
      Alert.alert('Ошибка', 'Произошла ошибка при переводе токенов');
      return false;
    } finally {
      setIsLoading(false);
    }
  };

  const getTransactionHistory = async (
    limit: number = 50, 
    offset: number = 0
  ): Promise<Transaction[]> => {
    try {
      const response = await walletAPI.getTransactionHistory(limit, offset);
      
      if (response.success) {
        return response.transactions;
      } else {
        console.error('Failed to get transaction history:', response.error);
        return [];
      }
    } catch (error) {
      console.error('Error getting transaction history:', error);
      return [];
    }
  };

  const value: WalletContextType = {
    balance,
    transactions,
    isLoading,
    refreshBalance,
    refreshTransactions,
    claimCheck,
    transferTokens,
    getTransactionHistory,
  };

  return (
    <WalletContext.Provider value={value}>
      {children}
    </WalletContext.Provider>
  );
};

