import axios, { AxiosInstance, AxiosResponse } from 'axios';
import type {
  ApiResponse,
  MenuItem,
  Order,
  CreateOrderRequest,
  WalletBalance,
  BlockchainOrderRecord,
  VotingRecord,
  FranchiseNode,
  SaleItem,
  User,
  TokenomicsConfig,
  VideoStream,
  UTHolder,
  ConversionResult,
  ConvertUTToSTRequest,
  VoteRequest
} from '@/types/api';

class ApiClient {
  private client: AxiosInstance;

  constructor(baseURL: string = 'http://localhost:3000') {
    this.client = axios.create({
      baseURL,
      timeout: 10000,
      headers: {
        'Content-Type': 'application/json',
      },
    });

    // Request interceptor
    this.client.interceptors.request.use(
      (config) => {
        console.log(`API Request: ${config.method?.toUpperCase()} ${config.url}`);
        return config;
      },
      (error) => {
        console.error('API Request Error:', error);
        return Promise.reject(error);
      }
    );

    // Response interceptor
    this.client.interceptors.response.use(
      (response: AxiosResponse) => {
        console.log(`API Response: ${response.status} ${response.config.url}`);
        return response;
      },
      (error) => {
        console.error('API Response Error:', error.response?.data || error.message);
        return Promise.reject(error);
      }
    );
  }

  // Menu API
  async getMenu(): Promise<MenuItem[]> {
    const response = await this.client.post<ApiResponse<{ items: MenuItem[] }>>('/', {
      GetMenu: {}
    });
    return response.data.data?.items || [];
  }

  async getMenuItem(id: string): Promise<MenuItem | null> {
    try {
      const response = await this.client.post<ApiResponse<{ item: MenuItem }>>('/', {
        GetMenuItem: { id }
      });
      return response.data.data?.item || null;
    } catch (error) {
      return null;
    }
  }

  async addMenuItem(item: Omit<MenuItem, 'id' | 'votes_for' | 'votes_against' | 'is_available_for_voting'>): Promise<boolean> {
    try {
      const response = await this.client.post<ApiResponse<{ success: boolean }>>('/', {
        AddMenuItem: item
      });
      return response.data.data?.success || false;
    } catch (error) {
      return false;
    }
  }

  // Order API
  async createOrder(order: CreateOrderRequest): Promise<Order | null> {
    try {
      const response = await this.client.post<ApiResponse<{ order: Order }>>('/', {
        CreateOrder: order
      });
      return response.data.data?.order || null;
    } catch (error) {
      return null;
    }
  }

  async getOrderStatus(orderId: string): Promise<Order | null> {
    try {
      const response = await this.client.post<ApiResponse<{ order: Order }>>('/', {
        GetOrderStatus: { order_id: orderId }
      });
      return response.data.data?.order || null;
    } catch (error) {
      return null;
    }
  }

  async confirmOrder(orderId: string): Promise<boolean> {
    try {
      const response = await this.client.post<ApiResponse<{ success: boolean }>>('/', {
        ConfirmOrder: { order_id: orderId }
      });
      return response.data.data?.success || false;
    } catch (error) {
      return false;
    }
  }

  async cancelOrder(orderId: string, reason: string, customerWallet: string): Promise<boolean> {
    try {
      const response = await this.client.post<ApiResponse<{ success: boolean }>>('/', {
        CancelOrder: { 
          order_id: orderId, 
          reason, 
          customer_wallet: customerWallet 
        }
      });
      return response.data.data?.success || false;
    } catch (error) {
      return false;
    }
  }

  // Wallet API
  async getWalletBalance(wallet: string): Promise<WalletBalance | null> {
    try {
      const response = await this.client.post<ApiResponse<WalletBalance>>('/', {
        GetWalletBalance: { wallet }
      });
      return response.data.data || null;
    } catch (error) {
      return null;
    }
  }

  // Blockchain API
  async getBlockchainHistory(limit?: number): Promise<BlockchainOrderRecord[]> {
    try {
      const response = await this.client.post<ApiResponse<{ orders: BlockchainOrderRecord[] }>>('/', {
        GetBlockchainHistory: { limit }
      });
      return response.data.data?.orders || [];
    } catch (error) {
      return [];
    }
  }

  async getVotingHistory(): Promise<VotingRecord[]> {
    try {
      const response = await this.client.post<ApiResponse<{ votes: VotingRecord[] }>>('/', {
        GetVotingHistory: {}
      });
      return response.data.data?.votes || [];
    } catch (error) {
      return [];
    }
  }

  async voteOnMenuItem(vote: VoteRequest): Promise<boolean> {
    try {
      const response = await this.client.post<ApiResponse<{ success: boolean }>>('/', {
        VoteOnMenuItem: vote
      });
      return response.data.data?.success || false;
    } catch (error) {
      return false;
    }
  }

  // Franchise API
  async addFranchiseNode(nodeId: string, franchiseOwner: string): Promise<boolean> {
    try {
      const response = await this.client.post<ApiResponse<{ success: boolean }>>('/', {
        AddFranchiseNode: { node_id: nodeId, franchise_owner: franchiseOwner }
      });
      return response.data.data?.success || false;
    } catch (error) {
      return false;
    }
  }

  // KYC/AML API
  async registerUser(user: Omit<User, 'id' | 'kyc_status' | 'roles' | 'created_at' | 'updated_at'>): Promise<string | null> {
    try {
      const response = await this.client.post<ApiResponse<{ verification_code: string }>>('/', {
        RegisterUser: user
      });
      return response.data.data?.verification_code || null;
    } catch (error) {
      return null;
    }
  }

  async startKYCProcess(userId: string, kycLevel: string): Promise<boolean> {
    try {
      const response = await this.client.post<ApiResponse<{ success: boolean }>>('/', {
        StartKYCProcess: { user_id: userId, kyc_level: kycLevel }
      });
      return response.data.data?.success || false;
    } catch (error) {
      return false;
    }
  }

  // Video Streaming API
  async getVideoStreams(): Promise<VideoStream[]> {
    try {
      const response = await this.client.get<VideoStream[]>('/streams');
      return response.data;
    } catch (error) {
      return [];
    }
  }

  async createVideoStream(name: string, sourceType: string, sourceValue: string): Promise<VideoStream | null> {
    try {
      const response = await this.client.post<VideoStream>('/streams', {
        name,
        source_type: sourceType,
        source_value: sourceValue
      });
      return response.data;
    } catch (error) {
      return null;
    }
  }

  async activateVideoStream(streamId: string): Promise<boolean> {
    try {
      await this.client.put(`/streams/${streamId}/activate`);
      return true;
    } catch (error) {
      return false;
    }
  }

  async deactivateVideoStream(streamId: string): Promise<boolean> {
    try {
      await this.client.put(`/streams/${streamId}/deactivate`);
      return true;
    } catch (error) {
      return false;
    }
  }

  // UT to ST Conversion API
  async getUTHolders(): Promise<UTHolder[]> {
    try {
      const response = await this.client.post<ApiResponse<{ holders: UTHolder[] }>>('/', {
        GetUTHolders: {}
      });
      return response.data.data?.holders || [];
    } catch (error) {
      return [];
    }
  }

  async convertUTToST(request: ConvertUTToSTRequest): Promise<ConversionResult | null> {
    try {
      const response = await this.client.post<ApiResponse<ConversionResult>>('/', {
        ConvertUTToST: request
      });
      return response.data.data || null;
    } catch (error) {
      return null;
    }
  }

  // Utility methods
  async healthCheck(): Promise<boolean> {
    try {
      const response = await this.client.get('/health');
      return response.status === 200;
    } catch (error) {
      return false;
    }
  }

  setBaseURL(url: string): void {
    this.client.defaults.baseURL = url;
  }

  getBaseURL(): string {
    return this.client.defaults.baseURL || '';
  }
}

// Export singleton instance
export const apiClient = new ApiClient();
export default apiClient;
