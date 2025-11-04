import { loadStripe, type Stripe } from '@stripe/stripe-js';

// Stripe configuration
const STRIPE_PUBLISHABLE_KEY = process.env.NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY || 'pk_test_...';
let stripePromise: Promise<Stripe | null>;

const getStripe = () => {
  if (!stripePromise) {
    stripePromise = loadStripe(STRIPE_PUBLISHABLE_KEY);
  }
  return stripePromise;
};

// Payment interfaces
export interface PaymentIntent {
  id: string;
  amount: number;
  currency: string;
  status: 'requires_payment_method' | 'requires_confirmation' | 'requires_action' | 'processing' | 'succeeded' | 'canceled';
  client_secret: string;
}

export interface BpiPurchaseRequest {
  bpiAmount: number;
  priceUsd: number;
  walletAddress: string;
  networkType: 'testnet' | 'mainnet';
  paymentMethod: 'card' | 'bank';
}

export interface TransactionResult {
  success: boolean;
  transactionId?: string;
  error?: string;
  bpiAmount?: number;
}

export interface WalletBalance {
  total: number;
  available: number;
  reserved: number;
  freeAllocation: number;
  networkType: 'testnet' | 'mainnet';
}

export interface BpiTransaction {
  id: string;
  type: 'gas_fee' | 'rent_payment' | 'transfer_in' | 'transfer_out' | 'purchase' | 'free_allocation';
  amount: number;
  timestamp: Date;
  description: string;
  status: 'completed' | 'pending' | 'failed';
  hash?: string;
  gasType?: string;
  fromAddress?: string;
  toAddress?: string;
}

class PaymentService {
  private apiBaseUrl: string;
  private stripe: Promise<Stripe | null>;

  constructor() {
    this.apiBaseUrl = process.env.REACT_APP_API_URL || 'https://api.pravyom.com';
    this.stripe = getStripe();
  }

  // Stripe Payment Methods
  async createPaymentIntent(request: BpiPurchaseRequest): Promise<PaymentIntent> {
    try {
      const response = await fetch(`${this.apiBaseUrl}/api/payments/create-intent`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${this.getAuthToken()}`
        },
        body: JSON.stringify(request)
      });

      if (!response.ok) {
        throw new Error(`Payment intent creation failed: ${response.statusText}`);
      }

      return await response.json();
    } catch (error) {
      console.error('Error creating payment intent:', error);
      throw error;
    }
  }

  async confirmPayment(paymentIntentId: string, paymentMethodId: string): Promise<TransactionResult> {
    try {
      const stripe = await this.stripe;
      if (!stripe) {
        throw new Error('Stripe not initialized');
      }

      const { error, paymentIntent } = await stripe.confirmCardPayment(paymentIntentId, {
        payment_method: paymentMethodId
      });

      if (error) {
        return {
          success: false,
          error: error.message
        };
      }

      if (paymentIntent?.status === 'succeeded') {
        // Notify backend of successful payment
        await this.notifyPaymentSuccess(paymentIntent.id);
        
        return {
          success: true,
          transactionId: paymentIntent.id
        };
      }

      return {
        success: false,
        error: 'Payment not completed'
      };
    } catch (error) {
      console.error('Error confirming payment:', error);
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Payment failed'
      };
    }
  }

  private async notifyPaymentSuccess(paymentIntentId: string): Promise<void> {
    try {
      await fetch(`${this.apiBaseUrl}/api/payments/confirm`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${this.getAuthToken()}`
        },
        body: JSON.stringify({ paymentIntentId })
      });
    } catch (error) {
      console.error('Error notifying payment success:', error);
    }
  }

  // BPI Wallet Methods
  async getWalletBalance(walletAddress: string): Promise<WalletBalance> {
    try {
      const response = await fetch(`${this.apiBaseUrl}/api/wallet/${walletAddress}/balance`, {
        headers: {
          'Authorization': `Bearer ${this.getAuthToken()}`
        }
      });

      if (!response.ok) {
        throw new Error(`Failed to fetch wallet balance: ${response.statusText}`);
      }

      return await response.json();
    } catch (error) {
      console.error('Error fetching wallet balance:', error);
      // Return mock data for development
      return {
        total: 2000,
        available: 1847,
        reserved: 153,
        freeAllocation: 2000,
        networkType: 'testnet'
      };
    }
  }

  async getTransactionHistory(walletAddress: string, limit: number = 10): Promise<BpiTransaction[]> {
    try {
      const response = await fetch(`${this.apiBaseUrl}/api/wallet/${walletAddress}/transactions?limit=${limit}`, {
        headers: {
          'Authorization': `Bearer ${this.getAuthToken()}`
        }
      });

      if (!response.ok) {
        throw new Error(`Failed to fetch transaction history: ${response.statusText}`);
      }

      return await response.json();
    } catch (error) {
      console.error('Error fetching transaction history:', error);
      // Return mock data for development
      return [
        {
          id: '1',
          type: 'free_allocation',
          amount: 2000,
          timestamp: new Date(Date.now() - 86400000),
          description: 'Testnet Free Allocation (1500 + 500 bonus)',
          status: 'completed'
        },
        {
          id: '2',
          type: 'gas_fee',
          amount: -5,
          timestamp: new Date(Date.now() - 3600000),
          description: 'Container Deployment',
          status: 'completed',
          gasType: 'ContainerDeploy'
        }
      ];
    }
  }

  async sendBpiTransaction(
    fromAddress: string,
    toAddress: string,
    amount: number,
    gasType: string = 'standard',
    memo?: string
  ): Promise<TransactionResult> {
    try {
      const response = await fetch(`${this.apiBaseUrl}/api/wallet/send`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${this.getAuthToken()}`
        },
        body: JSON.stringify({
          fromAddress,
          toAddress,
          amount,
          gasType,
          memo
        })
      });

      if (!response.ok) {
        throw new Error(`Transaction failed: ${response.statusText}`);
      }

      return await response.json();
    } catch (error) {
      console.error('Error sending BPI transaction:', error);
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Transaction failed'
      };
    }
  }

  // Gas Fee Estimation
  async estimateGasFee(transactionType: string, amount: number, priority: 'low' | 'medium' | 'high' = 'medium'): Promise<number> {
    try {
      const response = await fetch(`${this.apiBaseUrl}/api/gas/estimate`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${this.getAuthToken()}`
        },
        body: JSON.stringify({
          transactionType,
          amount,
          priority
        })
      });

      if (!response.ok) {
        throw new Error(`Gas estimation failed: ${response.statusText}`);
      }

      const result = await response.json();
      return result.estimatedGasFee;
    } catch (error) {
      console.error('Error estimating gas fee:', error);
      // Return mock estimation based on transaction type
      const baseFees: Record<string, number> = {
        'standard': 1,
        'fast': 2,
        'instant': 5,
        'ContainerDeploy': 5,
        'PoEBundle': 2,
        'Notarization': 1,
        'Validation': 1,
        'CrossChainBridge': 10,
        'GovernanceProposal': 25,
        'CommunityVoting': 0.5
      };
      
      const baseFee = baseFees[transactionType] || 1;
      const priorityMultiplier = priority === 'high' ? 1.5 : priority === 'low' ? 0.7 : 1;
      
      return baseFee * priorityMultiplier;
    }
  }

  // Rent Fee Calculation
  async calculateRentFee(hours: number, vmType: string = 'standard'): Promise<number> {
    try {
      const response = await fetch(`${this.apiBaseUrl}/api/rent/calculate`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${this.getAuthToken()}`
        },
        body: JSON.stringify({
          hours,
          vmType
        })
      });

      if (!response.ok) {
        throw new Error(`Rent calculation failed: ${response.statusText}`);
      }

      const result = await response.json();
      return result.totalRentFee;
    } catch (error) {
      console.error('Error calculating rent fee:', error);
      // Return mock calculation: $2/hour default
      return hours * 2;
    }
  }

  // Network and Deployment Analysis
  async analyzeDeploymentCost(
    deploymentType: 'light' | 'medium' | 'heavy',
    durationMonths: number
  ): Promise<{
    totalCost: number;
    breakdown: {
      gasFees: number;
      rentFees: number;
      operations: number;
    };
    recommendation: string;
  }> {
    const deploymentPatterns = {
      light: {
        containersPerMonth: 5,
        poeBundlesPerMonth: 30,
        validationsPerMonth: 50,
        vmHoursPerMonth: 360 // 12 hours/day
      },
      medium: {
        containersPerMonth: 20,
        poeBundlesPerMonth: 100,
        validationsPerMonth: 200,
        vmHoursPerMonth: 720 // 24 hours/day
      },
      heavy: {
        containersPerMonth: 50,
        poeBundlesPerMonth: 300,
        validationsPerMonth: 500,
        vmHoursPerMonth: 1440 // 48 hours/day (multiple VMs)
      }
    };

    const pattern = deploymentPatterns[deploymentType];
    const totalMonths = durationMonths;

    // Calculate costs
    const gasFees = (
      pattern.containersPerMonth * 5 + // Container deployments
      pattern.poeBundlesPerMonth * 2 + // PoE bundles
      pattern.validationsPerMonth * 1   // Validations
    ) * totalMonths;

    const rentFees = pattern.vmHoursPerMonth * 2 * totalMonths; // $2/hour
    const operations = gasFees; // Operations are included in gas fees
    const totalCost = gasFees + rentFees;

    let recommendation = '';
    if (totalCost <= 1500) {
      recommendation = '1500 BPI is sufficient for this deployment pattern';
    } else if (totalCost <= 3500) {
      recommendation = 'Recommend Professional plan (3500 BPI) for optimal coverage';
    } else {
      recommendation = 'Recommend Enterprise plan (10000 BPI) for heavy usage patterns';
    }

    return {
      totalCost,
      breakdown: {
        gasFees,
        rentFees,
        operations
      },
      recommendation
    };
  }

  // Utility Methods
  private getAuthToken(): string {
    // Get auth token from localStorage or context
    return localStorage.getItem('authToken') || '';
  }

  async validateBpiAddress(address: string): Promise<boolean> {
    return address.startsWith('bpi1') && address.length === 42;
  }

  formatBpiAmount(amount: number): string {
    return new Intl.NumberFormat('en-US', {
      minimumFractionDigits: 0,
      maximumFractionDigits: 6
    }).format(amount);
  }

  formatUsdAmount(amount: number): string {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD'
    }).format(amount);
  }
}

// Export singleton instance
export const paymentService = new PaymentService();
export default paymentService;
