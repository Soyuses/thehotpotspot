import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import AsyncStorage from '@react-native-async-storage/async-storage';
import { Alert } from 'react-native';
import { authAPI } from '../services/api';

interface User {
  id: string;
  phone: string;
  name: string;
  email: string;
  tshirtSize: string;
  favoriteDish: string;
  kycStatus: 'pending' | 'approved' | 'rejected';
  createdAt: string;
}

interface AuthContextType {
  user: User | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  login: (phone: string, password: string) => Promise<boolean>;
  register: (userData: RegisterData) => Promise<boolean>;
  logout: () => Promise<void>;
  updateProfile: (userData: Partial<User>) => Promise<boolean>;
  checkKYCStatus: () => Promise<void>;
}

interface RegisterData {
  phone: string;
  name: string;
  email: string;
  tshirtSize: string;
  favoriteDish: string;
  password: string;
  confirmPassword: string;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};

interface AuthProviderProps {
  children: ReactNode;
}

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  const isAuthenticated = !!user;

  useEffect(() => {
    loadStoredUser();
  }, []);

  const loadStoredUser = async () => {
    try {
      const storedUser = await AsyncStorage.getItem('user');
      if (storedUser) {
        setUser(JSON.parse(storedUser));
      }
    } catch (error) {
      console.error('Error loading stored user:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const login = async (phone: string, password: string): Promise<boolean> => {
    try {
      setIsLoading(true);
      const response = await authAPI.login(phone, password);
      
      if (response.success) {
        const userData = response.user;
        setUser(userData);
        await AsyncStorage.setItem('user', JSON.stringify(userData));
        return true;
      } else {
        Alert.alert('Ошибка входа', response.error || 'Неверный номер телефона или пароль');
        return false;
      }
    } catch (error) {
      console.error('Login error:', error);
      Alert.alert('Ошибка', 'Произошла ошибка при входе в систему');
      return false;
    } finally {
      setIsLoading(false);
    }
  };

  const register = async (userData: RegisterData): Promise<boolean> => {
    try {
      setIsLoading(true);
      
      // Validate password confirmation
      if (userData.password !== userData.confirmPassword) {
        Alert.alert('Ошибка', 'Пароли не совпадают');
        return false;
      }

      // Validate password strength
      if (userData.password.length < 8) {
        Alert.alert('Ошибка', 'Пароль должен содержать минимум 8 символов');
        return false;
      }

      const response = await authAPI.register({
        phone: userData.phone,
        name: userData.name,
        email: userData.email,
        tshirtSize: userData.tshirtSize,
        favoriteDish: userData.favoriteDish,
        password: userData.password,
      });

      if (response.success) {
        const newUser = response.user;
        setUser(newUser);
        await AsyncStorage.setItem('user', JSON.stringify(newUser));
        return true;
      } else {
        Alert.alert('Ошибка регистрации', response.error || 'Не удалось создать аккаунт');
        return false;
      }
    } catch (error) {
      console.error('Registration error:', error);
      Alert.alert('Ошибка', 'Произошла ошибка при регистрации');
      return false;
    } finally {
      setIsLoading(false);
    }
  };

  const logout = async (): Promise<void> => {
    try {
      setUser(null);
      await AsyncStorage.removeItem('user');
      await AsyncStorage.removeItem('wallet_data');
    } catch (error) {
      console.error('Logout error:', error);
    }
  };

  const updateProfile = async (userData: Partial<User>): Promise<boolean> => {
    try {
      if (!user) return false;

      setIsLoading(true);
      const response = await authAPI.updateProfile(user.id, userData);

      if (response.success) {
        const updatedUser = { ...user, ...userData };
        setUser(updatedUser);
        await AsyncStorage.setItem('user', JSON.stringify(updatedUser));
        return true;
      } else {
        Alert.alert('Ошибка', response.error || 'Не удалось обновить профиль');
        return false;
      }
    } catch (error) {
      console.error('Update profile error:', error);
      Alert.alert('Ошибка', 'Произошла ошибка при обновлении профиля');
      return false;
    } finally {
      setIsLoading(false);
    }
  };

  const checkKYCStatus = async (): Promise<void> => {
    try {
      if (!user) return;

      const response = await authAPI.checkKYCStatus(user.id);
      if (response.success && response.kycStatus !== user.kycStatus) {
        const updatedUser = { ...user, kycStatus: response.kycStatus };
        setUser(updatedUser);
        await AsyncStorage.setItem('user', JSON.stringify(updatedUser));
      }
    } catch (error) {
      console.error('KYC status check error:', error);
    }
  };

  const value: AuthContextType = {
    user,
    isAuthenticated,
    isLoading,
    login,
    register,
    logout,
    updateProfile,
    checkKYCStatus,
  };

  return (
    <AuthContext.Provider value={value}>
      {children}
    </AuthContext.Provider>
  );
};

