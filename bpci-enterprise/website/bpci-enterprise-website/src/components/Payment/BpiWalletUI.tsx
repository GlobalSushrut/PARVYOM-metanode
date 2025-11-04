import React, { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '../ui/card';
import { Button } from '../ui/button';
import { Badge } from '../ui/badge';
import { Separator } from '../ui/separator';
import { 
  Wallet, 
  Send, 
  Download, 
  History, 
  Coins, 
  TrendingUp, 
  TrendingDown,
  Clock,
  Shield,
  Zap,
  RefreshCw
} from 'lucide-react';

interface BpiTransaction {
  id: string;
  type: 'gas_fee' | 'rent_payment' | 'transfer_in' | 'transfer_out' | 'free_allocation';
  amount: number;
  timestamp: Date;
  description: string;
  status: 'completed' | 'pending' | 'failed';
  gasType?: string;
}

interface WalletBalance {
  total: number;
  available: number;
  reserved: number;
  freeAllocation: number;
  networkType: 'testnet' | 'mainnet';
}

interface BpiWalletUIProps {
  walletAddress?: string;
  onSendCoins?: () => void;
  onReceiveCoins?: () => void;
  onBuyMoreCoins?: () => void;
}

export const BpiWalletUI: React.FC<BpiWalletUIProps> = ({
  walletAddress = "bpi1x7k9m2n8q4r5t6u7v8w9x0y1z2a3b4c5d6e7f8",
  onSendCoins,
  onReceiveCoins,
  onBuyMoreCoins
}) => {
  const [balance, setBalance] = useState<WalletBalance>({
    total: 2000,
    available: 1847,
    reserved: 153,
    freeAllocation: 2000,
    networkType: 'testnet'
  });

  const [transactions, setTransactions] = useState<BpiTransaction[]>([
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
    },
    {
      id: '3',
      type: 'gas_fee',
      amount: -2,
      timestamp: new Date(Date.now() - 1800000),
      description: 'PoE Bundle Submission',
      status: 'completed',
      gasType: 'PoEBundle'
    },
    {
      id: '4',
      type: 'rent_payment',
      amount: -146,
      timestamp: new Date(Date.now() - 900000),
      description: 'VM Rent (73 hours @ 2 BPI/hour)',
      status: 'completed'
    }
  ]);

  const [isRefreshing, setIsRefreshing] = useState(false);

  const refreshBalance = async () => {
    setIsRefreshing(true);
    // Simulate API call
    await new Promise(resolve => setTimeout(resolve, 1000));
    setIsRefreshing(false);
  };

  const getTransactionIcon = (type: string) => {
    switch (type) {
      case 'gas_fee': return <Zap className="h-4 w-4 text-yellow-500" />;
      case 'rent_payment': return <Clock className="h-4 w-4 text-blue-500" />;
      case 'transfer_in': return <TrendingUp className="h-4 w-4 text-green-500" />;
      case 'transfer_out': return <TrendingDown className="h-4 w-4 text-red-500" />;
      case 'free_allocation': return <Coins className="h-4 w-4 text-purple-500" />;
      default: return <History className="h-4 w-4 text-gray-500" />;
    }
  };

  const getStatusBadge = (status: string) => {
    switch (status) {
      case 'completed': return <Badge variant="default" className="bg-green-100 text-green-800">Completed</Badge>;
      case 'pending': return <Badge variant="default" className="bg-yellow-100 text-yellow-800">Pending</Badge>;
      case 'failed': return <Badge variant="destructive">Failed</Badge>;
      default: return <Badge variant="secondary">Unknown</Badge>;
    }
  };

  return (
    <div className="space-y-6">
      {/* Wallet Header */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Wallet className="h-5 w-5" />
            BPI Wallet
            <Badge variant={balance.networkType === 'testnet' ? 'secondary' : 'default'}>
              {balance.networkType.toUpperCase()}
            </Badge>
          </CardTitle>
          <p className="text-sm text-muted-foreground font-mono">
            {walletAddress}
          </p>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            {/* Total Balance */}
            <div className="text-center p-4 bg-gradient-to-r from-blue-50 to-purple-50 rounded-lg">
              <div className="text-2xl font-bold text-blue-600">
                {balance.total.toLocaleString()} BPI
              </div>
              <div className="text-sm text-muted-foreground">Total Balance</div>
            </div>

            {/* Available Balance */}
            <div className="text-center p-4 bg-gradient-to-r from-green-50 to-blue-50 rounded-lg">
              <div className="text-2xl font-bold text-green-600">
                {balance.available.toLocaleString()} BPI
              </div>
              <div className="text-sm text-muted-foreground">Available</div>
            </div>

            {/* Reserved Balance */}
            <div className="text-center p-4 bg-gradient-to-r from-yellow-50 to-orange-50 rounded-lg">
              <div className="text-2xl font-bold text-orange-600">
                {balance.reserved.toLocaleString()} BPI
              </div>
              <div className="text-sm text-muted-foreground">Reserved</div>
            </div>
          </div>

          {/* Free Allocation Info (Testnet Only) */}
          {balance.networkType === 'testnet' && (
            <div className="mt-4 p-3 bg-purple-50 border border-purple-200 rounded-lg">
              <div className="flex items-center gap-2">
                <Shield className="h-4 w-4 text-purple-600" />
                <span className="text-sm font-medium text-purple-800">
                  Testnet Free Allocation: {balance.freeAllocation.toLocaleString()} BPI
                </span>
              </div>
              <p className="text-xs text-purple-600 mt-1">
                All operations are free and refundable during testnet
              </p>
            </div>
          )}

          {/* Action Buttons */}
          <div className="flex gap-2 mt-4">
            <Button onClick={onSendCoins} className="flex-1">
              <Send className="h-4 w-4 mr-2" />
              Send
            </Button>
            <Button onClick={onReceiveCoins} variant="outline" className="flex-1">
              <Download className="h-4 w-4 mr-2" />
              Receive
            </Button>
            <Button onClick={onBuyMoreCoins} variant="outline" className="flex-1">
              <Coins className="h-4 w-4 mr-2" />
              Buy More
            </Button>
            <Button 
              onClick={refreshBalance} 
              variant="ghost" 
              size="icon"
              disabled={isRefreshing}
            >
              <RefreshCw className={`h-4 w-4 ${isRefreshing ? 'animate-spin' : ''}`} />
            </Button>
          </div>
        </CardContent>
      </Card>

      {/* Transaction History */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <History className="h-5 w-5" />
            Recent Transactions
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-3">
            {transactions.map((tx, index) => (
              <div key={tx.id}>
                <div className="flex items-center justify-between p-3 hover:bg-gray-50 rounded-lg">
                  <div className="flex items-center gap-3">
                    {getTransactionIcon(tx.type)}
                    <div>
                      <div className="font-medium text-sm">{tx.description}</div>
                      <div className="text-xs text-muted-foreground">
                        {tx.timestamp.toLocaleString()}
                      </div>
                    </div>
                  </div>
                  <div className="text-right">
                    <div className={`font-medium ${tx.amount > 0 ? 'text-green-600' : 'text-red-600'}`}>
                      {tx.amount > 0 ? '+' : ''}{tx.amount.toLocaleString()} BPI
                    </div>
                    {getStatusBadge(tx.status)}
                  </div>
                </div>
                {index < transactions.length - 1 && <Separator />}
              </div>
            ))}
          </div>

          {/* View All Transactions */}
          <Button variant="outline" className="w-full mt-4">
            View All Transactions
          </Button>
        </CardContent>
      </Card>

      {/* Network Status */}
      <Card>
        <CardHeader>
          <CardTitle className="text-sm">Network Information</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-2 gap-4 text-sm">
            <div>
              <span className="text-muted-foreground">Network:</span>
              <span className="ml-2 font-medium">{balance.networkType.toUpperCase()}</span>
            </div>
            <div>
              <span className="text-muted-foreground">Gas Fees:</span>
              <span className="ml-2 font-medium">
                {balance.networkType === 'testnet' ? 'FREE' : 'ENABLED'}
              </span>
            </div>
            <div>
              <span className="text-muted-foreground">Rent Fees:</span>
              <span className="ml-2 font-medium">
                {balance.networkType === 'testnet' ? 'FREE' : '$2/hour'}
              </span>
            </div>
            <div>
              <span className="text-muted-foreground">Refundable:</span>
              <span className="ml-2 font-medium">
                {balance.networkType === 'testnet' ? 'YES' : 'NO'}
              </span>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
};

export default BpiWalletUI;
