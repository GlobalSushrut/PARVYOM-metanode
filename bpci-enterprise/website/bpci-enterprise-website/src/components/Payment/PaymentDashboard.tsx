import React, { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '../ui/card';
import { Button } from '../ui/button';
import { Badge } from '../ui/badge';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '../ui/tabs';
import { Alert, AlertDescription } from '../ui/alert';
import { 
  Wallet, 
  CreditCard, 
  Send, 
  TrendingUp, 
  Shield, 
  CheckCircle,
  AlertCircle,
  Calculator,
  Clock
} from 'lucide-react';

import BpiWalletUI from './BpiWalletUI';
import StripePaymentPage from './StripePaymentPage';
import CoinTransactionUI from './CoinTransactionUI';
import { paymentService, type TransactionResult, type WalletBalance } from '../../services/paymentService';

interface PaymentDashboardProps {
  walletAddress?: string;
  networkType?: 'testnet' | 'mainnet';
  onNetworkSwitch?: (network: 'testnet' | 'mainnet') => void;
}

export const PaymentDashboard: React.FC<PaymentDashboardProps> = ({
  walletAddress = "bpi1x7k9m2n8q4r5t6u7v8w9x0y1z2a3b4c5d6e7f8",
  networkType = 'testnet',
  onNetworkSwitch
}) => {
  const [activeTab, setActiveTab] = useState('wallet');
  const [walletBalance, setWalletBalance] = useState<WalletBalance | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [notification, setNotification] = useState<{
    type: 'success' | 'error' | 'info';
    message: string;
  } | null>(null);

  // Load wallet data on component mount
  useEffect(() => {
    loadWalletData();
  }, [walletAddress, networkType]);

  const loadWalletData = async () => {
    setIsLoading(true);
    try {
      const balance = await paymentService.getWalletBalance(walletAddress);
      setWalletBalance(balance);
    } catch (error) {
      showNotification('error', 'Failed to load wallet data');
    } finally {
      setIsLoading(false);
    }
  };

  const showNotification = (type: 'success' | 'error' | 'info', message: string) => {
    setNotification({ type, message });
    setTimeout(() => setNotification(null), 5000);
  };

  const handlePaymentSuccess = async (amount: number) => {
    showNotification('success', `Successfully purchased ${amount.toLocaleString()} BPI coins!`);
    await loadWalletData(); // Refresh wallet balance
    setActiveTab('wallet'); // Switch to wallet tab
  };

  const handlePaymentError = (error: string) => {
    showNotification('error', `Payment failed: ${error}`);
  };

  const handleSendTransaction = async (transaction: any): Promise<void> => {
    try {
      const result: TransactionResult = await paymentService.sendBpiTransaction(
        walletAddress,
        transaction.recipientAddress,
        transaction.amount,
        transaction.gasType,
        transaction.memo
      );

      if (result.success) {
        showNotification('success', `Successfully sent ${transaction.amount} BPI!`);
        await loadWalletData(); // Refresh wallet balance
      } else {
        showNotification('error', result.error || 'Transaction failed');
      }
    } catch (error) {
      showNotification('error', 'Transaction failed');
    }
  };

  const handleReceiveRequest = () => {
    showNotification('info', 'Payment request feature coming soon!');
  };

  const handleBuyMoreCoins = () => {
    setActiveTab('purchase');
  };

  // Calculate 2-month deployment analysis
  const get2MonthAnalysis = () => {
    if (!walletBalance) return null;

    const currentBpi = walletBalance.total;
    const heavyUsageCost = 3250; // Heavy usage for 2 months
    const lightUsageCost = 800;   // Light usage for 2 months
    const subscriptionCost = 20;  // $10/month × 2 months after initial period

    // Testnet also charges real fees now
    const testnetMessage = networkType === 'testnet' 
      ? `Testnet charges real gas/rent fees + $10/month after 2 months`
      : 'Mainnet with full fee structure';

    if (currentBpi >= heavyUsageCost) {
      return {
        status: 'excellent',
        message: 'Sufficient for 2+ months of heavy deployment',
        color: 'green'
      };
    } else if (currentBpi >= lightUsageCost) {
      return {
        status: 'good',
        message: 'Sufficient for 2 months of light-medium deployment',
        color: 'yellow'
      };
    } else {
      return {
        status: 'insufficient',
        message: 'Insufficient for 2 months of heavy deployment',
        color: 'red'
      };
    }
  };

  const analysis = get2MonthAnalysis();

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-center">
          <Clock className="h-8 w-8 animate-spin mx-auto mb-2" />
          <p>Loading payment dashboard...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Wallet className="h-6 w-6" />
              BPI Payment Dashboard
              <Badge variant={networkType === 'testnet' ? 'secondary' : 'default'}>
                {networkType.toUpperCase()}
              </Badge>
            </div>
            {onNetworkSwitch && (
              <Button
                variant="outline"
                onClick={() => onNetworkSwitch(networkType === 'testnet' ? 'mainnet' : 'testnet')}
                size="sm"
              >
                Switch to {networkType === 'testnet' ? 'Mainnet' : 'Testnet'}
              </Button>
            )}
          </CardTitle>
          <p className="text-sm text-muted-foreground">
            Manage your BPI coins, make payments, and purchase additional tokens
          </p>
        </CardHeader>
      </Card>

      {/* Notifications */}
      {notification && (
        <Alert className={
          notification.type === 'success' ? 'border-green-500 bg-green-50' :
          notification.type === 'error' ? 'border-red-500 bg-red-50' :
          'border-blue-500 bg-blue-50'
        }>
          {notification.type === 'success' ? <CheckCircle className="h-4 w-4" /> :
           notification.type === 'error' ? <AlertCircle className="h-4 w-4" /> :
           <Shield className="h-4 w-4" />}
          <AlertDescription>{notification.message}</AlertDescription>
        </Alert>
      )}

      {/* 2-Month Deployment Analysis */}
      {analysis && walletBalance && (
        <Card className={`border-${analysis.color}-200 bg-${analysis.color}-50`}>
          <CardHeader>
            <CardTitle className="flex items-center gap-2 text-sm">
              <Calculator className="h-4 w-4" />
              2-Month Advanced Deployment Analysis
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div className="text-center">
                <div className="text-2xl font-bold">
                  {walletBalance.total.toLocaleString()} BPI
                </div>
                <div className="text-sm text-muted-foreground">Current Balance</div>
              </div>
              <div className="text-center">
                <div className={`text-2xl font-bold text-${analysis.color}-600`}>
                  {analysis.status.toUpperCase()}
                </div>
                <div className="text-sm text-muted-foreground">Status</div>
              </div>
              <div className="text-center">
                <div className="text-lg font-medium">
                  {networkType === 'testnet' ? 'FREE' : '~3250 BPI'}
                </div>
                <div className="text-sm text-muted-foreground">Heavy Usage Cost</div>
              </div>
            </div>
            <p className={`text-sm text-${analysis.color}-700 mt-3 text-center`}>
              {analysis.message}
            </p>
          </CardContent>
        </Card>
      )}

      {/* Main Tabs */}
      <Tabs value={activeTab} onValueChange={setActiveTab}>
        <TabsList className="grid w-full grid-cols-3">
          <TabsTrigger value="wallet" className="flex items-center gap-2">
            <Wallet className="h-4 w-4" />
            Wallet
          </TabsTrigger>
          <TabsTrigger value="transactions" className="flex items-center gap-2">
            <Send className="h-4 w-4" />
            Transactions
          </TabsTrigger>
          <TabsTrigger value="purchase" className="flex items-center gap-2">
            <CreditCard className="h-4 w-4" />
            Purchase
          </TabsTrigger>
        </TabsList>

        {/* Wallet Tab */}
        <TabsContent value="wallet">
          <BpiWalletUI
            walletAddress={walletAddress}
            onSendCoins={() => setActiveTab('transactions')}
            onReceiveCoins={() => setActiveTab('transactions')}
            onBuyMoreCoins={handleBuyMoreCoins}
          />
        </TabsContent>

        {/* Transactions Tab */}
        <TabsContent value="transactions">
          <CoinTransactionUI
            walletAddress={walletAddress}
            currentBalance={walletBalance?.available || 0}
            networkType={networkType}
            onSendTransaction={handleSendTransaction}
            onReceiveRequest={handleReceiveRequest}
          />
        </TabsContent>

        {/* Purchase Tab */}
        <TabsContent value="purchase">
          <StripePaymentPage
            currentBalance={walletBalance?.total || 0}
            networkType={networkType}
            onPaymentSuccess={handlePaymentSuccess}
            onPaymentError={handlePaymentError}
          />
        </TabsContent>
      </Tabs>

      {/* Quick Stats */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <Card>
          <CardContent className="p-4 text-center">
            <div className="text-2xl font-bold text-blue-600">
              {walletBalance?.total.toLocaleString() || '0'}
            </div>
            <div className="text-sm text-muted-foreground">Total BPI</div>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="p-4 text-center">
            <div className="text-2xl font-bold text-green-600">
              {walletBalance?.available.toLocaleString() || '0'}
            </div>
            <div className="text-sm text-muted-foreground">Available</div>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="p-4 text-center">
            <div className="text-2xl font-bold text-orange-600">
              {walletBalance?.reserved.toLocaleString() || '0'}
            </div>
            <div className="text-sm text-muted-foreground">Reserved</div>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="p-4 text-center">
            <div className="text-2xl font-bold text-purple-600">
        $1.00
            </div>
            <div className="text-sm text-muted-foreground">BPI Price</div>
          </CardContent>
        </Card>
      </div>

      {/* Network Information */}
      <Card>
        <CardHeader>
          <CardTitle className="text-sm">Network Information & Fee Structure</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
            <div>
              <span className="text-muted-foreground">Network:</span>
              <span className="ml-2 font-medium">{networkType.toUpperCase()}</span>
            </div>
            <div>
              <span className="text-muted-foreground">Gas Fees:</span>
              <span className="ml-2 font-medium">$0.50-$25</span>
            </div>
            <div>
              <span className="text-muted-foreground">Rent Fees:</span>
              <span className="ml-2 font-medium">$2/hour</span>
            </div>
            <div>
              <span className="text-muted-foreground">Subscription:</span>
              <span className="ml-2 font-medium">$10/month after 2 months</span>
            </div>
            <div>
              <span className="text-muted-foreground">Consultation:</span>
              <span className="ml-2 font-medium">$100/hour</span>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
};

export default PaymentDashboard;
