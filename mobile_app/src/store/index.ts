import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';
import AsyncStorage from '@react-native-async-storage/async-storage';
import {
  User,
  TokenBalance,
  MenuItem,
  Order,
  Alert,
  NetworkStats,
  AppSettings,
  AppState,
} from '../types';
import apiService from '../services/api';

interface AppStore extends AppState {
  // Actions
  setUser: (user: User | null) => void;
  setTokenBalance: (balance: TokenBalance | null) => void;
  setMenuItems: (items: MenuItem[]) => void;
  setOrders: (orders: Order[]) => void;
  setAlerts: (alerts: Alert[]) => void;
  setNetworkStats: (stats: NetworkStats | null) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  
  // Async actions
  loadUserData: (walletAddress: string) => Promise<void>;
  loadMenuItems: () => Promise<void>;
  loadOrders: (customerWallet: string) => Promise<void>;
  loadAlerts: () => Promise<void>;
  loadNetworkStats: () => Promise<void>;
  loadTokenBalance: (walletAddress: string) => Promise<void>;
  
  // User actions
  registerUser: (phoneNumber: string, walletAddress: string) => Promise<boolean>;
  verifyUser: (phoneNumber: string, code: string) => Promise<boolean>;
  createOrder: (orderData: any) => Promise<boolean>;
  voteOnMenuItem: (menuItemId: string, voteFor: boolean) => Promise<boolean>;
  transferFromCheck: (checkId: string, phoneNumber: string) => Promise<boolean>;
  
  // Owner actions
  addFranchiseNode: (nodeId: string, franchiseOwner: string) => Promise<boolean>;
  emitTokensForInvestors: (investorAddress: string, amount: number, reason: string) => Promise<boolean>;
  distributeUnclaimedTokens: () => Promise<boolean>;
  
  // Franchise actions
  confirmOrder: (orderId: string) => Promise<boolean>;
  cancelOrder: (orderId: string, reason: string) => Promise<boolean>;
  
  // Utility actions
  clearError: () => void;
  reset: () => void;
}

const useAppStore = create<AppStore>()(
  persist(
    (set, get) => ({
      // Initial state
      user: null,
      tokenBalance: null,
      menuItems: [],
      orders: [],
      alerts: [],
      networkStats: null,
      isLoading: false,
      error: null,

      // Basic setters
      setUser: (user) => set({ user }),
      setTokenBalance: (tokenBalance) => set({ tokenBalance }),
      setMenuItems: (menuItems) => set({ menuItems }),
      setOrders: (orders) => set({ orders }),
      setAlerts: (alerts) => set({ alerts }),
      setNetworkStats: (networkStats) => set({ networkStats }),
      setLoading: (isLoading) => set({ isLoading }),
      setError: (error) => set({ error }),

      // Async actions
      loadUserData: async (walletAddress: string) => {
        set({ isLoading: true, error: null });
        try {
          const response = await apiService.getUserByWallet(walletAddress);
          if (response.success && response.data) {
            set({ user: response.data });
          } else {
            set({ error: response.error || 'Failed to load user data' });
          }
        } catch (error) {
          set({ error: 'Network error while loading user data' });
        } finally {
          set({ isLoading: false });
        }
      },

      loadMenuItems: async () => {
        set({ isLoading: true, error: null });
        try {
          const response = await apiService.getMenu();
          if (response.success && response.data) {
            set({ menuItems: response.data });
          } else {
            set({ error: response.error || 'Failed to load menu items' });
          }
        } catch (error) {
          set({ error: 'Network error while loading menu items' });
        } finally {
          set({ isLoading: false });
        }
      },

      loadOrders: async (customerWallet: string) => {
        set({ isLoading: true, error: null });
        try {
          const response = await apiService.getOrderHistory(customerWallet);
          if (response.success && response.data) {
            set({ orders: response.data });
          } else {
            set({ error: response.error || 'Failed to load orders' });
          }
        } catch (error) {
          set({ error: 'Network error while loading orders' });
        } finally {
          set({ isLoading: false });
        }
      },

      loadAlerts: async () => {
        set({ isLoading: true, error: null });
        try {
          const response = await apiService.getMonitoringAlerts();
          if (response.success && response.data) {
            set({ alerts: response.data });
          } else {
            set({ error: response.error || 'Failed to load alerts' });
          }
        } catch (error) {
          set({ error: 'Network error while loading alerts' });
        } finally {
          set({ isLoading: false });
        }
      },

      loadNetworkStats: async () => {
        set({ isLoading: true, error: null });
        try {
          const response = await apiService.getNetworkStats();
          if (response.success && response.data) {
            set({ networkStats: response.data });
          } else {
            set({ error: response.error || 'Failed to load network stats' });
          }
        } catch (error) {
          set({ error: 'Network error while loading network stats' });
        } finally {
          set({ isLoading: false });
        }
      },

      loadTokenBalance: async (walletAddress: string) => {
        set({ isLoading: true, error: null });
        try {
          const response = await apiService.getWalletBalance(walletAddress);
          if (response.success && response.data) {
            set({ tokenBalance: response.data });
          } else {
            set({ error: response.error || 'Failed to load token balance' });
          }
        } catch (error) {
          set({ error: 'Network error while loading token balance' });
        } finally {
          set({ isLoading: false });
        }
      },

      // User actions
      registerUser: async (phoneNumber: string, walletAddress: string) => {
        set({ isLoading: true, error: null });
        try {
          const response = await apiService.registerUser(phoneNumber, walletAddress);
          if (response.success && response.data) {
            set({ user: response.data });
            return true;
          } else {
            set({ error: response.error || 'Failed to register user' });
            return false;
          }
        } catch (error) {
          set({ error: 'Network error while registering user' });
          return false;
        } finally {
          set({ isLoading: false });
        }
      },

      verifyUser: async (phoneNumber: string, code: string) => {
        set({ isLoading: true, error: null });
        try {
          const response = await apiService.verifyUser(phoneNumber, code);
          if (response.success && response.data) {
            set({ user: response.data });
            return true;
          } else {
            set({ error: response.error || 'Failed to verify user' });
            return false;
          }
        } catch (error) {
          set({ error: 'Network error while verifying user' });
          return false;
        } finally {
          set({ isLoading: false });
        }
      },

      createOrder: async (orderData: any) => {
        set({ isLoading: true, error: null });
        try {
          const response = await apiService.createOrder(orderData);
          if (response.success) {
            // Reload orders after creating new one
            const { user } = get();
            if (user) {
              await get().loadOrders(user.walletAddress);
            }
            return true;
          } else {
            set({ error: response.error || 'Failed to create order' });
            return false;
          }
        } catch (error) {
          set({ error: 'Network error while creating order' });
          return false;
        } finally {
          set({ isLoading: false });
        }
      },

      voteOnMenuItem: async (menuItemId: string, voteFor: boolean) => {
        set({ isLoading: true, error: null });
        try {
          const { user } = get();
          if (!user) {
            set({ error: 'User not logged in' });
            return false;
          }

          const response = await apiService.voteOnMenuItem(user.walletAddress, menuItemId, voteFor);
          if (response.success) {
            return true;
          } else {
            set({ error: response.error || 'Failed to vote on menu item' });
            return false;
          }
        } catch (error) {
          set({ error: 'Network error while voting' });
          return false;
        } finally {
          set({ isLoading: false });
        }
      },

      transferFromCheck: async (checkId: string, phoneNumber: string) => {
        set({ isLoading: true, error: null });
        try {
          const { user } = get();
          if (!user) {
            set({ error: 'User not logged in' });
            return false;
          }

          const response = await apiService.transferBalanceFromCheck(checkId, user.walletAddress, phoneNumber);
          if (response.success) {
            // Reload token balance after transfer
            await get().loadTokenBalance(user.walletAddress);
            return true;
          } else {
            set({ error: response.error || 'Failed to transfer from check' });
            return false;
          }
        } catch (error) {
          set({ error: 'Network error while transferring from check' });
          return false;
        } finally {
          set({ isLoading: false });
        }
      },

      // Owner actions
      addFranchiseNode: async (nodeId: string, franchiseOwner: string) => {
        set({ isLoading: true, error: null });
        try {
          const response = await apiService.addFranchiseNode(nodeId, franchiseOwner);
          if (response.success) {
            return true;
          } else {
            set({ error: response.error || 'Failed to add franchise node' });
            return false;
          }
        } catch (error) {
          set({ error: 'Network error while adding franchise node' });
          return false;
        } finally {
          set({ isLoading: false });
        }
      },

      emitTokensForInvestors: async (investorAddress: string, amount: number, reason: string) => {
        set({ isLoading: true, error: null });
        try {
          const response = await apiService.emitTokensForInvestors(investorAddress, amount, reason);
          if (response.success) {
            return true;
          } else {
            set({ error: response.error || 'Failed to emit tokens for investors' });
            return false;
          }
        } catch (error) {
          set({ error: 'Network error while emitting tokens' });
          return false;
        } finally {
          set({ isLoading: false });
        }
      },

      distributeUnclaimedTokens: async () => {
        set({ isLoading: true, error: null });
        try {
          const response = await apiService.distributeUnclaimedTokensAnnually();
          if (response.success) {
            return true;
          } else {
            set({ error: response.error || 'Failed to distribute unclaimed tokens' });
            return false;
          }
        } catch (error) {
          set({ error: 'Network error while distributing tokens' });
          return false;
        } finally {
          set({ isLoading: false });
        }
      },

      // Franchise actions
      confirmOrder: async (orderId: string) => {
        set({ isLoading: true, error: null });
        try {
          const response = await apiService.confirmOrder(orderId);
          if (response.success) {
            // Reload orders after confirming
            const { user } = get();
            if (user) {
              await get().loadOrders(user.walletAddress);
            }
            return true;
          } else {
            set({ error: response.error || 'Failed to confirm order' });
            return false;
          }
        } catch (error) {
          set({ error: 'Network error while confirming order' });
          return false;
        } finally {
          set({ isLoading: false });
        }
      },

      cancelOrder: async (orderId: string, reason: string) => {
        set({ isLoading: true, error: null });
        try {
          const response = await apiService.cancelOrder(orderId, reason);
          if (response.success) {
            // Reload orders after cancelling
            const { user } = get();
            if (user) {
              await get().loadOrders(user.walletAddress);
            }
            return true;
          } else {
            set({ error: response.error || 'Failed to cancel order' });
            return false;
          }
        } catch (error) {
          set({ error: 'Network error while cancelling order' });
          return false;
        } finally {
          set({ isLoading: false });
        }
      },

      // Utility actions
      clearError: () => set({ error: null }),
      reset: () => set({
        user: null,
        tokenBalance: null,
        menuItems: [],
        orders: [],
        alerts: [],
        networkStats: null,
        isLoading: false,
        error: null,
      }),
    }),
    {
      name: 'app-storage',
      storage: createJSONStorage(() => AsyncStorage),
      partialize: (state) => ({
        user: state.user,
        // Не сохраняем временные данные как isLoading, error
      }),
    }
  )
);

export default useAppStore;
