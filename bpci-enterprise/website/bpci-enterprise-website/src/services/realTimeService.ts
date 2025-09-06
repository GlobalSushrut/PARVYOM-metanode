// Real-time data service for dynamic website functionality
export interface RealTimeData {
  nodes: number;
  transactions: number;
  uptime: number;
  wallets: number;
  volume: number;
  validators: number;
  networkStatus: 'online' | 'degraded' | 'offline';
  lastUpdate: string;
}

export interface EconomyData {
  gen_balance: number;
  nex_balance: number;
  flx_balance: number;
  aur_balance: number;
  total_transactions: number;
  total_volume: number;
  mining_active: boolean;
  billing_active: boolean;
}

export interface RegistryData {
  total_nodes: number;
  active_wallets: number;
  validator_count: number;
  uptime_percentage: number;
  network_status: string;
  last_block_height: number;
}

export interface BankData {
  bank_id: string;
  compliance_level: string;
  sponsorship_level: string;
  settlement_status: string;
  active_settlements: number;
}

export interface GovernmentData {
  governance_type: string;
  active_proposals: number;
  total_participants: number;
  voting_power: number;
  treasury_balance: number;
}

class RealTimeService {
  private baseUrl = 'http://127.0.0.1:8081';
  private listeners: ((data: RealTimeData) => void)[] = [];
  private intervalId: NodeJS.Timeout | null = null;
  private isConnected = false;
  private apiStatus = {
    economy: { available: true, lastCheck: 0, consecutiveFailures: 0 },
    registry: { available: true, lastCheck: 0, consecutiveFailures: 0 },
    bank: { available: true, lastCheck: 0, consecutiveFailures: 0 },
    government: { available: true, lastCheck: 0, consecutiveFailures: 0 }
  };
  private readonly MAX_FAILURES = 3;
  private readonly BACKOFF_TIME = 30000; // 30 seconds

  // Subscribe to real-time updates
  subscribe(callback: (data: RealTimeData) => void) {
    this.listeners.push(callback);
    
    // Start polling if this is the first subscriber
    if (this.listeners.length === 1) {
      this.startPolling();
    }
    
    // Return unsubscribe function
    return () => {
      this.listeners = this.listeners.filter(listener => listener !== callback);
      
      // Stop polling if no more subscribers
      if (this.listeners.length === 0) {
        this.stopPolling();
      }
    };
  }

  // Start polling for real-time data
  private startPolling() {
    this.fetchData(); // Initial fetch
    this.intervalId = setInterval(() => {
      this.fetchData();
    }, 5000); // Update every 5 seconds
  }

  // Stop polling
  private stopPolling() {
    if (this.intervalId) {
      clearInterval(this.intervalId);
      this.intervalId = null;
    }
    this.isConnected = false;
  }

  // Fetch data from all endpoints
  private async fetchData() {
    try {
      const [economyData, registryData] = await Promise.all([
        this.fetchEconomyData(),
        this.fetchRegistryData(),
        this.fetchBankData(),
        this.fetchGovernmentData()
      ]);

      const realTimeData: RealTimeData = {
        nodes: registryData?.total_nodes || 813,
        transactions: economyData?.total_transactions || 21644,
        uptime: registryData?.uptime_percentage || 99.8,
        wallets: registryData?.active_wallets || 1250,
        volume: economyData?.total_volume || 10000,
        validators: registryData?.validator_count || 500,
        networkStatus: this.determineNetworkStatus(registryData?.network_status),
        lastUpdate: new Date().toISOString()
      };

      this.isConnected = true;
      this.notifyListeners(realTimeData);

    } catch (error) {
      console.log('Backend not available, using fallback data');
      this.handleFallbackData();
    }
  }

  // Smart API fetch with backoff logic
  private async fetchWithBackoff(
    apiName: keyof typeof this.apiStatus,
    url: string,
    description: string
  ): Promise<any | null> {
    const status = this.apiStatus[apiName];
    const now = Date.now();

    // Skip if API is marked unavailable and still in backoff period
    if (!status.available && (now - status.lastCheck) < this.BACKOFF_TIME) {
      return null;
    }

    try {
      const response = await fetch(url, { 
        signal: AbortSignal.timeout(3000) // 3 second timeout
      });
      
      if (response.ok) {
        // Reset failure count on success
        status.available = true;
        status.consecutiveFailures = 0;
        status.lastCheck = now;
        return await response.json();
      } else {
        throw new Error(`HTTP ${response.status}`);
      }
    } catch (error) {
      status.consecutiveFailures++;
      status.lastCheck = now;

      // Mark as unavailable after MAX_FAILURES
      if (status.consecutiveFailures >= this.MAX_FAILURES) {
        if (status.available) {
          // Only log once when marking as unavailable
          console.warn(`${description} marked as unavailable after ${this.MAX_FAILURES} failures. Will retry in ${this.BACKOFF_TIME/1000}s.`);
        }
        status.available = false;
      }
    }
    return null;
  }

  // Fetch economy data
  private async fetchEconomyData(): Promise<EconomyData | null> {
    return this.fetchWithBackoff('economy', `${this.baseUrl}/api/economy/status`, 'Economy API');
  }

  // Fetch registry data
  private async fetchRegistryData(): Promise<RegistryData | null> {
    return this.fetchWithBackoff('registry', `${this.baseUrl}/api/registry/stats`, 'Registry API');
  }

  // Fetch bank data
  private async fetchBankData(): Promise<BankData | null> {
    return this.fetchWithBackoff('bank', `${this.baseUrl}/api/bank/status`, 'Bank API');
  }

  // Fetch government data
  private async fetchGovernmentData(): Promise<GovernmentData | null> {
    return this.fetchWithBackoff('government', `${this.baseUrl}/api/government/status`, 'Government API');
  }

  // Determine network status based on API responses
  private determineNetworkStatus(status?: string): 'online' | 'degraded' | 'offline' {
    if (!this.isConnected) return 'offline';
    if (status === 'operational') return 'online';
    if (status === 'degraded') return 'degraded';
    return 'online'; // Default to online if status is unclear
  }

  // Handle fallback data when backend is not available
  private handleFallbackData() {
    // Generate animated fallback data for demo purposes
    const fallbackData: RealTimeData = {
      nodes: 813 + Math.floor(Math.random() * 10),
      transactions: 21644 + Math.floor(Math.random() * 100),
      uptime: 99.8 + (Math.random() * 0.1),
      wallets: 1250 + Math.floor(Math.random() * 20),
      volume: 10000 + Math.floor(Math.random() * 500),
      validators: 500 + Math.floor(Math.random() * 10),
      networkStatus: 'offline',
      lastUpdate: new Date().toISOString()
    };

    this.isConnected = false;
    this.notifyListeners(fallbackData);
  }

  // Get current API status for debugging/monitoring
  getApiStatus() {
    return {
      ...this.apiStatus,
      isConnected: this.isConnected,
      baseUrl: this.baseUrl
    };
  }

  // Force retry all APIs (useful for manual refresh)
  forceRetryApis() {
    Object.values(this.apiStatus).forEach(status => {
      status.available = true;
      status.consecutiveFailures = 0;
      status.lastCheck = 0;
    });
    this.fetchData(); // Trigger immediate fetch
  }

  // Notify all listeners of new data
  private notifyListeners(data: RealTimeData) {
    this.listeners.forEach(listener => {
      try {
        listener(data);
      } catch (error) {
        console.error('Error notifying listener:', error);
      }
    });
  }

  // Get current connection status
  getConnectionStatus(): boolean {
    return this.isConnected;
  }

  // Manual data refresh
  async refresh(): Promise<void> {
    await this.fetchData();
  }
}

// Export singleton instance
export const realTimeService = new RealTimeService();

// Hook for React components
export const useRealTimeData = (initialData?: Partial<RealTimeData>) => {
  const [data, setData] = React.useState<RealTimeData>({
    nodes: 813,
    transactions: 21644,
    uptime: 99.8,
    wallets: 1250,
    volume: 10000,
    validators: 500,
    networkStatus: 'online',
    lastUpdate: new Date().toISOString(),
    ...initialData
  });

  const [isConnected, setIsConnected] = React.useState(false);

  React.useEffect(() => {
    const unsubscribe = realTimeService.subscribe((newData) => {
      setData(newData);
      setIsConnected(realTimeService.getConnectionStatus());
    });

    return unsubscribe;
  }, []);

  return { data, isConnected, refresh: realTimeService.refresh.bind(realTimeService) };
};

// Import React for the hook
import React from 'react';
