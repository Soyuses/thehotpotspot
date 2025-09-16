import AsyncStorage from '@react-native-async-storage/async-storage';

// Base API configuration
const API_BASE_URL = 'https://api.thehotpotspot.com'; // Replace with actual API URL
const API_TIMEOUT = 10000; // 10 seconds

// API Response types
interface APIResponse<T = any> {
  success: boolean;
  data?: T;
  error?: string;
  message?: string;
}

// Auth API
export const authAPI = {
  async login(phone: string, password: string): Promise<APIResponse<{ token: string; user: any }>> {
    try {
      const response = await fetch(`${API_BASE_URL}/auth/login`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ phone, password }),
      });

      const data = await response.json();
      return data;
    } catch (error) {
      return {
        success: false,
        error: 'Network error during login',
      };
    }
  },

  async register(userData: any): Promise<APIResponse<{ token: string; user: any }>> {
    try {
      const response = await fetch(`${API_BASE_URL}/auth/register`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(userData),
      });

      const data = await response.json();
      return data;
    } catch (error) {
      return {
        success: false,
        error: 'Network error during registration',
      };
    }
  },

  async verifyPhone(phone: string, code: string): Promise<APIResponse> {
    try {
      const response = await fetch(`${API_BASE_URL}/auth/verify-phone`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ phone, code }),
      });

      const data = await response.json();
      return data;
    } catch (error) {
      return {
        success: false,
        error: 'Network error during phone verification',
      };
    }
  },

  async sendVerificationCode(phone: string): Promise<APIResponse> {
    try {
      const response = await fetch(`${API_BASE_URL}/auth/send-verification-code`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ phone }),
      });

      const data = await response.json();
      return data;
    } catch (error) {
      return {
        success: false,
        error: 'Network error during code sending',
      };
    }
  },

  async updateProfile(userData: any): Promise<APIResponse> {
    try {
      const token = await AsyncStorage.getItem('auth_token');
      const response = await fetch(`${API_BASE_URL}/auth/profile`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`,
        },
        body: JSON.stringify(userData),
      });

      const data = await response.json();
      return data;
    } catch (error) {
      return {
        success: false,
        error: 'Network error during profile update',
      };
    }
  },

  async checkKYCStatus(): Promise<APIResponse<{ status: string; required: boolean }>> {
    try {
      const token = await AsyncStorage.getItem('auth_token');
      const response = await fetch(`${API_BASE_URL}/auth/kyc-status`, {
        method: 'GET',
        headers: {
          'Authorization': `Bearer ${token}`,
        },
      });

      const data = await response.json();
      return data;
    } catch (error) {
      return {
        success: false,
        error: 'Network error during KYC status check',
      };
    }
  },
};

// Wallet API
export const walletAPI = {
  async getBalance(): Promise<APIResponse<{ stTokens: number; utTokens: number; totalValue: number }>> {
    try {
      const token = await AsyncStorage.getItem('auth_token');
      const response = await fetch(`${API_BASE_URL}/wallet/balance`, {
        method: 'GET',
        headers: {
          'Authorization': `Bearer ${token}`,
        },
      });

      const data = await response.json();
      return data;
    } catch (error) {
      return {
        success: false,
        error: 'Network error during balance fetch',
      };
    }
  },

  async getTransactions(): Promise<APIResponse<any[]>> {
    try {
      const token = await AsyncStorage.getItem('auth_token');
      const response = await fetch(`${API_BASE_URL}/wallet/transactions`, {
        method: 'GET',
        headers: {
          'Authorization': `Bearer ${token}`,
        },
      });

      const data = await response.json();
      return data;
    } catch (error) {
      return {
        success: false,
        error: 'Network error during transactions fetch',
      };
    }
  },

  async claimCheck(qrData: string): Promise<APIResponse<{ transferredTokens: number }>> {
    try {
      const token = await AsyncStorage.getItem('auth_token');
      const response = await fetch(`${API_BASE_URL}/wallet/claim-check`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`,
        },
        body: JSON.stringify({ qrData }),
      });

      const data = await response.json();
      return data;
    } catch (error) {
      return {
        success: false,
        error: 'Network error during check claim',
      };
    }
  },

  async transferTokens(toAddress: string, amount: number, type: 'st' | 'ut'): Promise<APIResponse> {
    try {
      const token = await AsyncStorage.getItem('auth_token');
      const response = await fetch(`${API_BASE_URL}/wallet/transfer`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`,
        },
        body: JSON.stringify({ toAddress, amount, type }),
      });

      const data = await response.json();
      return data;
    } catch (error) {
      return {
        success: false,
        error: 'Network error during token transfer',
      };
    }
  },

  async getTransactionHistory(limit: number = 50, offset: number = 0): Promise<APIResponse<any[]>> {
    try {
      const token = await AsyncStorage.getItem('auth_token');
      const response = await fetch(`${API_BASE_URL}/wallet/transactions?limit=${limit}&offset=${offset}`, {
        method: 'GET',
        headers: {
          'Authorization': `Bearer ${token}`,
        },
      });

      const data = await response.json();
      return data;
    } catch (error) {
      return {
        success: false,
        error: 'Network error during transaction history fetch',
      };
    }
  },
};

// Menu API
export const menuAPI = {
  async getMenu(): Promise<APIResponse<any[]>> {
    try {
      const response = await fetch(`${API_BASE_URL}/menu`, {
        method: 'GET',
      });

      const data = await response.json();
      return data;
    } catch (error) {
      return {
        success: false,
        error: 'Network error during menu fetch',
      };
    }
  },

  async getDishDetails(dishId: string): Promise<APIResponse<any>> {
    try {
      const response = await fetch(`${API_BASE_URL}/menu/${dishId}`, {
        method: 'GET',
      });

      const data = await response.json();
      return data;
    } catch (error) {
      return {
        success: false,
        error: 'Network error during dish details fetch',
      };
    }
  },

  async placeOrder(orderData: any): Promise<APIResponse<{ orderId: string; qrCode: string }>> {
    try {
      const token = await AsyncStorage.getItem('auth_token');
      const response = await fetch(`${API_BASE_URL}/orders`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`,
        },
        body: JSON.stringify(orderData),
      });

      const data = await response.json();
      return data;
    } catch (error) {
      return {
        success: false,
        error: 'Network error during order placement',
      };
    }
  },
};

// Voting API
export const votingAPI = {
  async getProposals(): Promise<APIResponse<any[]>> {
    try {
      const token = await AsyncStorage.getItem('auth_token');
      const response = await fetch(`${API_BASE_URL}/voting/proposals`, {
        method: 'GET',
        headers: {
          'Authorization': `Bearer ${token}`,
        },
      });

      const data = await response.json();
      return data;
    } catch (error) {
      return {
        success: false,
        error: 'Network error during proposals fetch',
      };
    }
  },

  async getProposalDetails(proposalId: string): Promise<APIResponse<any>> {
    try {
      const token = await AsyncStorage.getItem('auth_token');
      const response = await fetch(`${API_BASE_URL}/voting/proposals/${proposalId}`, {
        method: 'GET',
        headers: {
          'Authorization': `Bearer ${token}`,
        },
      });

      const data = await response.json();
      return data;
    } catch (error) {
      return {
        success: false,
        error: 'Network error during proposal details fetch',
      };
    }
  },

  async castVote(proposalId: string, choice: 'yes' | 'no' | 'abstain'): Promise<APIResponse> {
    try {
      const token = await AsyncStorage.getItem('auth_token');
      const response = await fetch(`${API_BASE_URL}/voting/vote`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`,
        },
        body: JSON.stringify({ proposalId, choice }),
      });

      const data = await response.json();
      return data;
    } catch (error) {
      return {
        success: false,
        error: 'Network error during vote casting',
      };
    }
  },

  async getVotingHistory(): Promise<APIResponse<any[]>> {
    try {
      const token = await AsyncStorage.getItem('auth_token');
      const response = await fetch(`${API_BASE_URL}/voting/history`, {
        method: 'GET',
        headers: {
          'Authorization': `Bearer ${token}`,
        },
      });

      const data = await response.json();
      return data;
    } catch (error) {
      return {
        success: false,
        error: 'Network error during voting history fetch',
      };
    }
  },
};

// Streaming API
export const streamingAPI = {
  async getStreams(): Promise<APIResponse<any[]>> {
    try {
      const response = await fetch(`${API_BASE_URL}/streaming/streams`, {
        method: 'GET',
      });

      const data = await response.json();
      return data;
    } catch (error) {
      return {
        success: false,
        error: 'Network error during streams fetch',
      };
    }
  },

  async getStreamDetails(streamId: string): Promise<APIResponse<any>> {
    try {
      const response = await fetch(`${API_BASE_URL}/streaming/streams/${streamId}`, {
        method: 'GET',
      });

      const data = await response.json();
      return data;
    } catch (error) {
      return {
        success: false,
        error: 'Network error during stream details fetch',
      };
    }
  },

  async startStreamingSession(streamId: string): Promise<APIResponse<{ sessionId: string }>> {
    try {
      const token = await AsyncStorage.getItem('auth_token');
      const response = await fetch(`${API_BASE_URL}/streaming/sessions`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`,
        },
        body: JSON.stringify({ streamId }),
      });

      const data = await response.json();
      return data;
    } catch (error) {
      return {
        success: false,
        error: 'Network error during streaming session start',
      };
    }
  },

  async endStreamingSession(sessionId: string): Promise<APIResponse> {
    try {
      const token = await AsyncStorage.getItem('auth_token');
      const response = await fetch(`${API_BASE_URL}/streaming/sessions/${sessionId}`, {
        method: 'DELETE',
        headers: {
          'Authorization': `Bearer ${token}`,
        },
      });

      const data = await response.json();
      return data;
    } catch (error) {
      return {
        success: false,
        error: 'Network error during streaming session end',
      };
    }
  },

  async recordActivity(sessionId: string, activityType: string, data: any): Promise<APIResponse> {
    try {
      const token = await AsyncStorage.getItem('auth_token');
      const response = await fetch(`${API_BASE_URL}/streaming/activity`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`,
        },
        body: JSON.stringify({ sessionId, activityType, data }),
      });

      const data = await response.json();
      return data;
    } catch (error) {
      return {
        success: false,
        error: 'Network error during activity recording',
      };
    }
  },
};