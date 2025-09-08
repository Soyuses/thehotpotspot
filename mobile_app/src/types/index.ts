// Основные типы для мобильного приложения

export interface User {
  id: string;
  walletAddress: string;
  phoneNumber?: string;
  role: 'owner' | 'franchise' | 'customer';
  isAuthorized: boolean;
  createdAt: number;
}

export interface TokenBalance {
  securityTokens: number;
  utilityTokens: number;
  totalBalance: number;
  ownershipPercentage: number;
}

export interface MenuItem {
  id: string;
  name: string;
  description: string;
  price: number;
  availability: number;
  priorityRank: number;
  cookingTimeMinutes: number;
  totalCalories: number;
  ingredients: Ingredient[];
  isAvailableForVoting: boolean;
  status: 'Proposed' | 'Voting' | 'Approved' | 'Rejected' | 'Active';
}

export interface Ingredient {
  name: string;
  amountGrams: number;
  calories: number;
}

export interface Order {
  id: string;
  customerWallet: string;
  items: OrderItem[];
  totalAmount: number;
  deliveryTimeMinutes: number;
  status: 'Pending' | 'Confirmed' | 'Cancelled' | 'Completed';
  createdAt: number;
  confirmedAt?: number;
  tokensIssued?: number;
}

export interface OrderItem {
  menuItemId: string;
  quantity: number;
  price: number;
}

export interface Check {
  id: string;
  customerWallet: string;
  foodTruck: string;
  amount: number;
  tokens: number;
  status: 'Pending' | 'Confirmed' | 'Cancelled';
  createdAt: number;
  phoneNumber?: string;
  isAuthorized: boolean;
}

export interface Vote {
  id: string;
  voterWallet: string;
  menuItemId: string;
  menuItemName: string;
  voteFor: boolean;
  voteWeight: number;
  timestamp: number;
}

export interface VotingItem {
  id: string;
  name: string;
  description: string;
  votesFor: number;
  votesAgainst: number;
  isActive: boolean;
  endTime?: number;
}

export interface FranchiseNode {
  nodeId: string;
  owner: string;
  isActive: boolean;
  createdAt: number;
  totalRevenue: number;
  totalOrders: number;
}

export interface Alert {
  id: string;
  type: 'OwnerExceedsLimit' | 'FranchiseExceedsLimit' | 'CustomerExceedsLimit' | 'TokenConcentration' | 'CharityFundLow';
  severity: 'Critical' | 'High' | 'Medium' | 'Low';
  message: string;
  timestamp: number;
  address?: string;
  percentage?: number;
}

export interface UnclaimedToken {
  checkId: string;
  amount: number;
  createdAt: number;
  expiryTime: number;
  isDistributed: boolean;
  distributedAt?: number;
}

export interface AnnualDistribution {
  year: number;
  totalUnclaimedTokens: number;
  distributionTimestamp: number;
  distributions: TokenDistribution[];
}

export interface TokenDistribution {
  recipientAddress: string;
  recipientType: 'Owner' | 'Franchise' | 'Customer' | 'CharityFund';
  amount: number;
  percentage: number;
}

export interface NetworkStats {
  totalNodes: number;
  totalOrders: number;
  totalTokens: number;
  activeUsers: number;
  totalRevenue: number;
  averageOrderValue: number;
}

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
  message?: string;
}

// Навигационные типы
export type RootStackParamList = {
  Login: undefined;
  Main: undefined;
  OrderDetails: { orderId: string };
  MenuItemDetails: { itemId: string };
  VotingDetails: { itemId: string };
  CheckTransfer: { checkId?: string };
  Profile: undefined;
  Settings: undefined;
};

export type CustomerTabParamList = {
  Menu: undefined;
  Orders: undefined;
  Voting: undefined;
  Wallet: undefined;
  Profile: undefined;
};

export type OwnerTabParamList = {
  Dashboard: undefined;
  Franchises: undefined;
  Alerts: undefined;
  Analytics: undefined;
  Settings: undefined;
};

export type FranchiseTabParamList = {
  Overview: undefined;
  Orders: undefined;
  Analytics: undefined;
  Settings: undefined;
};

// Состояние приложения
export interface AppState {
  user: User | null;
  tokenBalance: TokenBalance | null;
  menuItems: MenuItem[];
  orders: Order[];
  alerts: Alert[];
  networkStats: NetworkStats | null;
  isLoading: boolean;
  error: string | null;
}

// Настройки приложения
export interface AppSettings {
  notifications: {
    newOrders: boolean;
    payments: boolean;
    alerts: boolean;
    voting: boolean;
  };
  theme: 'light' | 'dark' | 'auto';
  language: 'ru' | 'en';
  biometricAuth: boolean;
  autoSync: boolean;
}
