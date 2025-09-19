// API Types for The Hot Pot Spot

export interface ApiResponse<T = any> {
  success: boolean;
  data?: T;
  error?: string;
  message?: string;
}

// Menu Types
export interface MenuItem {
  id: string;
  name: string;
  description: string;
  price_subunits: number;
  availability: number;
  priority_rank: number;
  cooking_time_minutes: number;
  ingredients: Ingredient[];
  suggested_by: string;
  votes_for: number;
  votes_against: number;
  is_available_for_voting: boolean;
}

export interface Ingredient {
  name: string;
  amount: string;
  unit: string;
}

// Order Types
export interface Order {
  id: string;
  customer_wallet: string;
  items: OrderItem[];
  total_amount: number;
  delivery_time_minutes: number;
  status: OrderStatus;
  created_at: string;
  confirmed_at?: string;
}

export interface OrderItem {
  menu_item_id: string;
  quantity: number;
  price_subunits: number;
}

export type OrderStatus = 'pending' | 'confirmed' | 'cooking' | 'ready' | 'delivered' | 'cancelled';

// Wallet Types
export interface WalletBalance {
  wallet: string;
  security_tokens: number;
  utility_tokens: number;
}

// Blockchain Types
export interface BlockchainOrderRecord {
  order_id: string;
  customer_wallet: string;
  total_amount: number;
  tokens_issued: number;
  timestamp: number;
}

export interface VotingRecord {
  voter_wallet: string;
  menu_item_id: string;
  menu_item_name: string;
  vote_weight: number;
  vote_for: boolean;
  timestamp: number;
}

// Franchise Types
export interface FranchiseNode {
  node_id: string;
  owner_address: string;
  city: string;
  active: boolean;
  registered_at: string;
  pos_systems: string[];
}

export interface SaleItem {
  item_id: string;
  name: string;
  price_subunits: number;
  quantity: number;
}

// KYC/AML Types
export interface User {
  id: string;
  email: string;
  phone?: string;
  first_name: string;
  last_name: string;
  date_of_birth?: string;
  nationality?: string;
  address?: Address;
  kyc_level: KYCLevel;
  kyc_status: KYCStatus;
  roles: UserRole[];
  created_at: string;
  updated_at: string;
}

export interface Address {
  street: string;
  city: string;
  state: string;
  country: string;
  postal_code: string;
}

export type KYCLevel = 'basic' | 'enhanced' | 'premium';
export type KYCStatus = 'pending' | 'verified' | 'rejected' | 'expired';

export interface UserRole {
  role: string;
  assigned_by: string;
  assigned_at: string;
  expires_at?: string;
}

// Tokenomics Types
export interface TokenomicsConfig {
  security_token: SecurityTokenConfig;
  utility_token: UtilityTokenConfig;
  conversion_pool: ConversionPoolConfig;
}

export interface SecurityTokenConfig {
  thp_per_gel: number;
  min_st_for_kyc: number;
  max_st_per_transaction: number;
}

export interface UtilityTokenConfig {
  ut_per_fifth_visit: number;
  ut_per_streaming_session: number;
  ut_per_2_hours_viewing: number;
  ut_per_repost: number;
  ut_per_popular_comment: number;
  max_streaming_session_minutes: number;
  max_ut_per_day: number;
  non_transferable: boolean;
  min_likes_for_popular_comment: number;
}

export interface ConversionPoolConfig {
  conversion_pool_share: number;
}

// Video Streaming Types
export interface VideoStream {
  stream_id: string;
  name: string;
  source: VideoSource;
  is_active: boolean;
  created_at: string;
  last_updated: string;
}

export type VideoSource = {
  type: 'camera';
  value: string;
} | {
  type: 'youtube';
  value: string;
};

// UT to ST Conversion Types
export interface UTHolder {
  address: string;
  balance: number;
  name?: string;
}

export interface ConversionDetail {
  holder_address: string;
  ut_converted: number;
  st_issued: number;
  conversion_rate: number;
}

export interface ConversionResult {
  success: boolean;
  converted_holders: ConversionDetail[];
  total_ut_converted: number;
  total_st_issued: number;
}

// API Request Types
export interface CreateOrderRequest {
  customer_wallet: string;
  items: OrderItem[];
  delivery_time_minutes: number;
}

export interface VoteRequest {
  voter_wallet: string;
  menu_item_id: string;
  vote_for: boolean;
}

export interface ConvertUTToSTRequest {
  holders: string[];
  exchange_rate: number;
}

// Error Types
export interface ApiError {
  message: string;
  code?: string;
  details?: any;
}
