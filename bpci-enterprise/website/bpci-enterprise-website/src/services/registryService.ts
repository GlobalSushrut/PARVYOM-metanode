import axios from 'axios';
import { authService } from './authService';

const API_BASE_URL = 'http://127.0.0.1:8081/api';

export interface RegistryStats {
  total_nodes: number;
  active_nodes: number;
  community_nodes: number;
  enterprise_nodes: number;
  bank_nodes: number;
  government_nodes: number;
  hybrid_nodes: number;
  total_validators: number;
  total_miners: number;
  total_notaries: number;
  network_health: 'Excellent' | 'Good' | 'Fair' | 'Poor';
  last_updated: string;
}

export interface NodeInfo {
  node_id: string;
  node_type: string;
  status: 'Active' | 'Inactive' | 'Maintenance';
  identity: {
    name: string;
    verification_level: string;
  };
  authority: {
    level: string;
    trust_score: number;
    compliance_level: string;
  };
  endpoints: {
    primary: string;
    api_endpoint?: string;
    mining_endpoint?: string;
    wallet_endpoint?: string;
  };
  capabilities: string[];
  created_at: string;
  last_active: string;
  uptime_percentage: number;
}

export interface WalletInfo {
  wallet_id: string;
  wallet_type: 'Normal' | 'Compliance' | 'Regulated' | 'Government' | 'Emergency' | 'Bank' | 'Community';
  stamp_type?: 'Government' | 'Bank' | 'Community' | 'Enterprise';
  balance: {
    gen: number;
    nex: number;
    flx: number;
    aur: number;
  };
  mining_sessions: number;
  total_rewards: number;
  status: 'Active' | 'Suspended' | 'Locked';
  created_at: string;
  last_transaction: string;
}

export interface InstallerStatus {
  installer_id: string;
  status: 'Installing' | 'Completed' | 'Failed' | 'Pending';
  progress: number;
  node_type: string;
  estimated_completion: string;
  logs: string[];
  created_at: string;
}

class RegistryService {
  async getRegistryStats(): Promise<RegistryStats | null> {
    try {
      const response = await axios.get(`${API_BASE_URL}/registry/stats`, {
        headers: authService.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      console.error('Failed to fetch registry stats:', error);
      return null;
    }
  }

  async getNodes(page: number = 1, limit: number = 10, nodeType?: string): Promise<{ nodes: NodeInfo[], total: number } | null> {
    try {
      const params = new URLSearchParams({
        page: page.toString(),
        limit: limit.toString(),
        ...(nodeType && { node_type: nodeType })
      });
      
      const response = await axios.get(`${API_BASE_URL}/registry/nodes?${params}`, {
        headers: authService.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      console.error('Failed to fetch nodes:', error);
      return null;
    }
  }

  async getNodeById(nodeId: string): Promise<NodeInfo | null> {
    try {
      const response = await axios.get(`${API_BASE_URL}/registry/nodes/${nodeId}`, {
        headers: authService.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      console.error('Failed to fetch node:', error);
      return null;
    }
  }

  async getWallets(page: number = 1, limit: number = 10): Promise<{ wallets: WalletInfo[], total: number } | null> {
    try {
      const params = new URLSearchParams({
        page: page.toString(),
        limit: limit.toString()
      });
      
      const response = await axios.get(`${API_BASE_URL}/wallet/list?${params}`, {
        headers: authService.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      console.error('Failed to fetch wallets:', error);
      return null;
    }
  }

  async getWalletById(walletId: string): Promise<WalletInfo | null> {
    try {
      const response = await axios.get(`${API_BASE_URL}/wallet/${walletId}`, {
        headers: authService.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      console.error('Failed to fetch wallet:', error);
      return null;
    }
  }

  async createBPCIWallet(walletData: {
    wallet_type: string;
    initial_stake?: number;
  }): Promise<{ success: boolean; wallet_id?: string; message: string }> {
    try {
      const response = await axios.post(`${API_BASE_URL}/wallet/create`, walletData, {
        headers: authService.getAuthHeaders()
      });
      return response.data;
    } catch (error: any) {
      return {
        success: false,
        message: error.response?.data?.message || 'Failed to create wallet'
      };
    }
  }

  async installBPINode(installerData: {
    node_type: string;
    installation_path: string;
    github_repo?: string;
    community_config?: any;
  }): Promise<{ success: boolean; installer_id?: string; message: string }> {
    try {
      const response = await axios.post(`${API_BASE_URL}/installer/start`, installerData, {
        headers: authService.getAuthHeaders()
      });
      return response.data;
    } catch (error: any) {
      return {
        success: false,
        message: error.response?.data?.message || 'Failed to start installation'
      };
    }
  }

  async getInstallerStatus(installerId: string): Promise<InstallerStatus | null> {
    try {
      const response = await axios.get(`${API_BASE_URL}/installer/status/${installerId}`, {
        headers: authService.getAuthHeaders()
      });
      return response.data;
    } catch (error) {
      console.error('Failed to fetch installer status:', error);
      return null;
    }
  }

  async getInstallerLogs(installerId: string): Promise<string[] | null> {
    try {
      const response = await axios.get(`${API_BASE_URL}/installer/logs/${installerId}`, {
        headers: authService.getAuthHeaders()
      });
      return response.data.logs;
    } catch (error) {
      console.error('Failed to fetch installer logs:', error);
      return null;
    }
  }

  async stampWallet(walletId: string, stampType: 'Government' | 'Bank' | 'Community' | 'Enterprise'): Promise<{ success: boolean; message: string }> {
    try {
      const response = await axios.post(`${API_BASE_URL}/stamped/stamps/register`, {
        wallet_id: walletId,
        stamp_type: stampType
      }, {
        headers: authService.getAuthHeaders()
      });
      return response.data;
    } catch (error: any) {
      return {
        success: false,
        message: error.response?.data?.message || 'Failed to stamp wallet'
      };
    }
  }

  async verifyWalletStamp(walletId: string): Promise<{ verified: boolean; stamp_type?: string; message: string }> {
    try {
      const response = await axios.get(`${API_BASE_URL}/stamped/stamps/verify/${walletId}`, {
        headers: authService.getAuthHeaders()
      });
      return response.data;
    } catch (error: any) {
      return {
        verified: false,
        message: error.response?.data?.message || 'Failed to verify wallet stamp'
      };
    }
  }
}

export const registryService = new RegistryService();
