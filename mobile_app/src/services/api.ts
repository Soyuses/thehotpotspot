import axios, { AxiosInstance, AxiosResponse } from 'axios';
import {
  User,
  TokenBalance,
  MenuItem,
  Order,
  Check,
  Vote,
  VotingItem,
  FranchiseNode,
  Alert,
  UnclaimedToken,
  AnnualDistribution,
  NetworkStats,
  ApiResponse,
} from '../types';

class ApiService {
  private api: AxiosInstance;
  private baseUrl: string;

  constructor() {
    this.baseUrl = 'http://127.0.0.1:3000';
    this.api = axios.create({
      baseURL: this.baseUrl,
      timeout: 10000,
      headers: {
        'Content-Type': 'application/json',
      },
    });

    // Добавляем интерцепторы для обработки ошибок
    this.api.interceptors.response.use(
      (response) => response,
      (error) => {
        console.error('API Error:', error);
        return Promise.reject(error);
      }
    );
  }

  // Методы для работы с пользователями
  async registerUser(phoneNumber: string, walletAddress: string): Promise<ApiResponse<User>> {
    try {
      const response = await this.api.post('/', {
        RegisterUser: {
          phone_number: phoneNumber,
          wallet_address: walletAddress,
        },
      });
      return this.handleResponse(response);
    } catch (error) {
      return this.handleError(error);
    }
  }

  async verifyUser(phoneNumber: string, verificationCode: string): Promise<ApiResponse<User>> {
    try {
      const response = await this.api.post('/', {
        VerifyUser: {
          phone_number: phoneNumber,
          verification_code: verificationCode,
        },
      });
      return this.handleResponse(response);
    } catch (error) {
      return this.handleError(error);
    }
  }

  async getUserByWallet(walletAddress: string): Promise<ApiResponse<User>> {
    try {
      const response = await this.api.post('/', {
        GetUserByWallet: {
          wallet_address: walletAddress,
        },
      });
      return this.handleResponse(response);
    } catch (error) {
      return this.handleError(error);
    }
  }

  // Методы для работы с токенами
  async getWalletBalance(walletAddress: string): Promise<ApiResponse<TokenBalance>> {
    try {
      const response = await this.api.post('/', {
        GetWalletBalance: {
          wallet_address: walletAddress,
        },
      });
      return this.handleResponse(response);
    } catch (error) {
      return this.handleError(error);
    }
  }

  // Методы для работы с меню
  async getMenu(): Promise<ApiResponse<MenuItem[]>> {
    try {
      const response = await this.api.post('/', {
        GetMenu: {},
      });
      return this.handleResponse(response);
    } catch (error) {
      return this.handleError(error);
    }
  }

  async addMenuItem(menuItem: Partial<MenuItem>): Promise<ApiResponse<MenuItem>> {
    try {
      const response = await this.api.post('/', {
        AddMenuItem: menuItem,
      });
      return this.handleResponse(response);
    } catch (error) {
      return this.handleError(error);
    }
  }

  // Методы для работы с заказами
  async createOrder(orderData: {
    customerWallet: string;
    items: Array<{ menuItemId: string; quantity: number }>;
    deliveryTimeMinutes: number;
  }): Promise<ApiResponse<Order>> {
    try {
      const response = await this.api.post('/', {
        CreateOrder: orderData,
      });
      return this.handleResponse(response);
    } catch (error) {
      return this.handleError(error);
    }
  }

  async getOrderHistory(customerWallet: string, limit?: number): Promise<ApiResponse<Order[]>> {
    try {
      const response = await this.api.post('/', {
        GetOrderHistory: {
          customer_wallet: customerWallet,
          limit: limit || 50,
        },
      });
      return this.handleResponse(response);
    } catch (error) {
      return this.handleError(error);
    }
  }

  async confirmOrder(orderId: string): Promise<ApiResponse<boolean>> {
    try {
      const response = await this.api.post('/', {
        ConfirmOrder: {
          order_id: orderId,
        },
      });
      return this.handleResponse(response);
    } catch (error) {
      return this.handleError(error);
    }
  }

  async cancelOrder(orderId: string, reason: string): Promise<ApiResponse<boolean>> {
    try {
      const response = await this.api.post('/', {
        CancelOrder: {
          order_id: orderId,
          reason,
          customer_wallet: 'Customer_Wallet', // В реальном приложении это будет из контекста
        },
      });
      return this.handleResponse(response);
    } catch (error) {
      return this.handleError(error);
    }
  }

  // Методы для работы с чеками
  async transferBalanceFromCheck(checkId: string, customerWallet: string, phoneNumber: string): Promise<ApiResponse<boolean>> {
    try {
      const response = await this.api.post('/', {
        TransferBalanceFromCheck: {
          check_id: checkId,
          customer_wallet: customerWallet,
          phone_number: phoneNumber,
        },
      });
      return this.handleResponse(response);
    } catch (error) {
      return this.handleError(error);
    }
  }

  async getCustomerChecks(customerWallet: string): Promise<ApiResponse<Check[]>> {
    try {
      const response = await this.api.post('/', {
        GetCustomerChecks: {
          customer_wallet: customerWallet,
        },
      });
      return this.handleResponse(response);
    } catch (error) {
      return this.handleError(error);
    }
  }

  // Методы для работы с голосованием
  async getVotingItems(): Promise<ApiResponse<VotingItem[]>> {
    try {
      const response = await this.api.post('/', {
        GetVotingItems: {},
      });
      return this.handleResponse(response);
    } catch (error) {
      return this.handleError(error);
    }
  }

  async voteOnMenuItem(voterWallet: string, menuItemId: string, voteFor: boolean): Promise<ApiResponse<boolean>> {
    try {
      const response = await this.api.post('/', {
        VoteOnMenuItem: {
          voter_wallet: voterWallet,
          menu_item_id: menuItemId,
          vote_for: voteFor,
        },
      });
      return this.handleResponse(response);
    } catch (error) {
      return this.handleError(error);
    }
  }

  async getVotingHistory(voterWallet?: string): Promise<ApiResponse<Vote[]>> {
    try {
      const response = await this.api.post('/', {
        GetVotingHistory: voterWallet ? { voter_wallet: voterWallet } : {},
      });
      return this.handleResponse(response);
    } catch (error) {
      return this.handleError(error);
    }
  }

  // Методы для владельца сети
  async getOwnerStats(): Promise<ApiResponse<any>> {
    try {
      const response = await this.api.post('/', {
        GetOwnerStats: {},
      });
      return this.handleResponse(response);
    } catch (error) {
      return this.handleError(error);
    }
  }

  async getMonitoringAlerts(limit?: number): Promise<ApiResponse<Alert[]>> {
    try {
      const response = await this.api.post('/', {
        GetMonitoringAlerts: {
          limit: limit || 10,
        },
      });
      return this.handleResponse(response);
    } catch (error) {
      return this.handleError(error);
    }
  }

  async addFranchiseNode(nodeId: string, franchiseOwner: string): Promise<ApiResponse<boolean>> {
    try {
      const response = await this.api.post('/', {
        AddFranchiseNode: {
          node_id: nodeId,
          franchise_owner: franchiseOwner,
        },
      });
      return this.handleResponse(response);
    } catch (error) {
      return this.handleError(error);
    }
  }

  async getFranchiseNodes(): Promise<ApiResponse<FranchiseNode[]>> {
    try {
      const response = await this.api.post('/', {
        GetFranchiseNodes: {},
      });
      return this.handleResponse(response);
    } catch (error) {
      return this.handleError(error);
    }
  }

  async emitTokensForInvestors(investorAddress: string, amount: number, reason: string): Promise<ApiResponse<boolean>> {
    try {
      const response = await this.api.post('/', {
        EmitTokensForInvestors: {
          investor_address: investorAddress,
          amount,
          reason,
        },
      });
      return this.handleResponse(response);
    } catch (error) {
      return this.handleError(error);
    }
  }

  // Методы для работы с невостребованными токенами
  async getUnclaimedTokens(limit?: number): Promise<ApiResponse<UnclaimedToken[]>> {
    try {
      const response = await this.api.post('/', {
        GetUnclaimedTokens: {
          limit: limit || 50,
        },
      });
      return this.handleResponse(response);
    } catch (error) {
      return this.handleError(error);
    }
  }

  async distributeUnclaimedTokensAnnually(): Promise<ApiResponse<boolean>> {
    try {
      const response = await this.api.post('/', {
        DistributeUnclaimedTokensAnnually: {},
      });
      return this.handleResponse(response);
    } catch (error) {
      return this.handleError(error);
    }
  }

  async getAnnualDistributions(limit?: number): Promise<ApiResponse<AnnualDistribution[]>> {
    try {
      const response = await this.api.post('/', {
        GetAnnualDistributions: {
          limit: limit || 10,
        },
      });
      return this.handleResponse(response);
    } catch (error) {
      return this.handleError(error);
    }
  }

  // Методы для получения статистики сети
  async getNetworkStats(): Promise<ApiResponse<NetworkStats>> {
    try {
      const response = await this.api.post('/', {
        GetNetworkStats: {},
      });
      return this.handleResponse(response);
    } catch (error) {
      return this.handleError(error);
    }
  }

  // Методы для владельца франшизы
  async getFranchiseStats(franchiseNodeId: string): Promise<ApiResponse<any>> {
    try {
      const response = await this.api.post('/', {
        GetFranchiseStats: {
          franchise_node_id: franchiseNodeId,
        },
      });
      return this.handleResponse(response);
    } catch (error) {
      return this.handleError(error);
    }
  }

  async getFranchiseOrders(franchiseNodeId: string, limit?: number): Promise<ApiResponse<Order[]>> {
    try {
      const response = await this.api.post('/', {
        GetFranchiseOrders: {
          franchise_node_id: franchiseNodeId,
          limit: limit || 100,
        },
      });
      return this.handleResponse(response);
    } catch (error) {
      return this.handleError(error);
    }
  }

  // Вспомогательные методы
  private handleResponse<T>(response: AxiosResponse): ApiResponse<T> {
    const data = response.data;
    
    // Проверяем, есть ли ошибка в ответе
    if (data.Error) {
      return {
        success: false,
        error: data.Error.message || 'Unknown error',
      };
    }

    // Извлекаем данные из ответа
    const responseKey = Object.keys(data)[0];
    const responseData = data[responseKey];

    return {
      success: true,
      data: responseData,
    };
  }

  private handleError(error: any): ApiResponse<any> {
    let errorMessage = 'Network error';
    
    if (error.response) {
      errorMessage = error.response.data?.message || `HTTP ${error.response.status}`;
    } else if (error.request) {
      errorMessage = 'No response from server';
    } else {
      errorMessage = error.message || 'Unknown error';
    }

    return {
      success: false,
      error: errorMessage,
    };
  }

  // Метод для проверки соединения с сервером
  async checkConnection(): Promise<boolean> {
    try {
      await this.api.post('/', { GetNetworkStats: {} });
      return true;
    } catch (error) {
      return false;
    }
  }
}

export default new ApiService();
