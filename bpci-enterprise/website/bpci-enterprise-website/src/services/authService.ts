import axios from 'axios';

const API_BASE_URL = 'http://127.0.0.1:8081/api';

export interface DeveloperLoginRequest {
  email: string;
  password: string;
}

export interface DeveloperSignupRequest {
  name: string;
  email: string;
  password: string;
  company?: string;
  role?: string;
}

export interface WalletActivationRequest {
  developer_id: string;
  wallet_type: 'Community' | 'Investor' | 'Government' | 'Bank' | 'Owner' | 'ESOP' | 'Treasury' | 'Company';
  owner_type?: 1 | 2 | 3 | 4 | 5; // Only for Owner wallet type
  network_type: 'Testnet' | 'Mainnet';
  stamp_type?: 'Government' | 'Bank' | 'Community' | 'Enterprise';
}

export interface AuthResponse {
  success: boolean;
  message: string;
  data?: {
    developer_id: string;
    email: string;
    session_token: string;
    profile_complete: boolean;
    wallet_activated: boolean;
  };
}

export interface WalletActivationResponse {
  success: boolean;
  message: string;
  data?: {
    registration_id: string;
    wallet_address: string;
    wallet_type: string;
    network_type: string;
    mother_coin_allocation: number;
    baby_coin_balance: number;
  };
}

export interface DeveloperProfile {
  developer_id: string;
  name: string;
  email: string;
  company?: string;
  role?: string;
  profile_complete: boolean;
  wallet_activated: boolean;
  wallet_info?: {
    registration_id: string;
    wallet_address: string;
    wallet_type: string;
    network_type: string;
    mother_coin_allocation: number;
    baby_coin_balance: number;
    poe_mining_enabled: boolean;
  };
  created_at: string;
  last_active: string;
}

export interface DeveloperProfile {
  developer_id: string;
  name: string;
  email: string;
  company?: string;
  role?: string;
  profile_complete: boolean;
  wallet_activated: boolean;
  wallet_info?: {
    registration_id: string;
    wallet_address: string;
    wallet_type: string;
    network_type: string;
    mother_coin_allocation: number;
    baby_coin_balance: number;
    poe_mining_enabled: boolean;
  };
  created_at: string;
  last_active: string;
}

class AuthService {
  private currentDeveloper: DeveloperProfile | null = null;
  private sessionToken: string | null = null;

  constructor() {
    // Check for existing session on initialization
    this.sessionToken = localStorage.getItem('developer_session_token');
    const developerData = localStorage.getItem('developer_profile');
    if (developerData) {
      try {
        this.currentDeveloper = JSON.parse(developerData);
      } catch (error) {
        console.error('Failed to parse stored developer data:', error);
        this.clearSession();
      }
    }
  }

  async login(credentials: DeveloperLoginRequest): Promise<AuthResponse> {
    try {
      const response = await axios.post<AuthResponse>(`${API_BASE_URL}/developer/login`, credentials);
      
      if (response.data.success && response.data.data) {
        this.sessionToken = response.data.data.session_token;
        localStorage.setItem('developer_session_token', this.sessionToken);
        
        // Fetch developer profile after successful login
        await this.fetchDeveloperProfile();
      }
      
      return response.data;
    } catch (error: any) {
      console.error('Developer login failed:', error);
      return {
        success: false,
        message: error.response?.data?.message || 'Login failed. Please check your email and password.'
      };
    }
  }

  async signup(userData: DeveloperSignupRequest): Promise<AuthResponse> {
    try {
      const response = await axios.post<AuthResponse>(`${API_BASE_URL}/developer/signup`, userData);
      
      if (response.data.success && response.data.data) {
        this.sessionToken = response.data.data.session_token;
        localStorage.setItem('developer_session_token', this.sessionToken);
        
        // Fetch developer profile after successful signup
        await this.fetchDeveloperProfile();
      }
      
      return response.data;
    } catch (error: any) {
      console.error('Developer signup failed:', error);
      return {
        success: false,
        message: error.response?.data?.message || 'Signup failed. Please try again.'
      };
    }
  }

  async activateWallet(walletData: WalletActivationRequest): Promise<WalletActivationResponse> {
    if (!this.sessionToken) {
      throw new Error('No session token available. Please login first.');
    }

    try {
      const response = await axios.post<WalletActivationResponse>(
        `${API_BASE_URL}/wallet/activate`, 
        walletData,
        {
          headers: {
            'Authorization': `Bearer ${this.sessionToken}`
          }
        }
      );
      
      if (response.data.success) {
        // Refresh developer profile to get updated wallet info
        await this.fetchDeveloperProfile();
      }
      
      return response.data;
    } catch (error: any) {
      console.error('Wallet activation failed:', error);
      return {
        success: false,
        message: error.response?.data?.message || 'Wallet activation failed. Please try again.'
      };
    }
  }

  async fetchDeveloperProfile(): Promise<void> {
    if (!this.sessionToken) {
      throw new Error('No session token available');
    }

    try {
      const response = await axios.get<{ success: boolean; data: DeveloperProfile }>(`${API_BASE_URL}/developer/profile`, {
        headers: {
          'Authorization': `Bearer ${this.sessionToken}`
        }
      });

      if (response.data.success) {
        this.currentDeveloper = response.data.data;
        localStorage.setItem('developer_profile', JSON.stringify(this.currentDeveloper));
      }
    } catch (error) {
      console.error('Failed to fetch developer profile:', error);
      this.clearSession();
      throw error;
    }
  }

  getCurrentDeveloper(): DeveloperProfile | null {
    return this.currentDeveloper;
  }

  getSessionToken(): string | null {
    return this.sessionToken;
  }

  isAuthenticated(): boolean {
    return this.sessionToken !== null && this.currentDeveloper !== null;
  }

  hasWalletActivated(): boolean {
    return this.currentDeveloper?.wallet_activated === true;
  }

  isProfileComplete(): boolean {
    return this.currentDeveloper?.profile_complete === true;
  }

  getWalletInfo() {
    return this.currentDeveloper?.wallet_info || null;
  }

  logout(): void {
    this.clearSession();
  }

  private clearSession(): void {
    this.currentDeveloper = null;
    this.sessionToken = null;
    localStorage.removeItem('developer_session_token');
    localStorage.removeItem('developer_profile');
  }
}

export const authService = new AuthService();
