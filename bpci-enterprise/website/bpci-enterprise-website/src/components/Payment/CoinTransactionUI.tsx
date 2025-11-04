import React, { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '../ui/card';
import { Button } from '../ui/button';
import { Input } from '../ui/input';
import { Label } from '../ui/label';
import { Badge } from '../ui/badge';
import { Alert, AlertDescription } from '../ui/alert';
import { Textarea } from '../ui/textarea';
import { 
  Send, 
  Download, 
  QrCode, 
  Copy, 
  CheckCircle, 
  AlertCircle,
  Zap,
  Clock,
  Calculator,
  ArrowRight,
  Wallet,
  Shield
} from 'lucide-react';

interface TransactionForm {
  recipientAddress: string;
  amount: number;
  gasType: string;
  memo: string;
  priority: 'low' | 'medium' | 'high';
}

interface GasFeeEstimate {
  type: string;
  baseFee: number;
  priorityFee: number;
  totalFee: number;
  estimatedTime: string;
}

interface CoinTransactionUIProps {
  walletAddress?: string;
  currentBalance?: number;
  networkType?: 'testnet' | 'mainnet';
  onSendTransaction?: (transaction: TransactionForm) => Promise<void>;
  onReceiveRequest?: () => void;
}

export const CoinTransactionUI: React.FC<CoinTransactionUIProps> = ({
  walletAddress = "bpi1x7k9m2n8q4r5t6u7v8w9x0y1z2a3b4c5d6e7f8",
  currentBalance = 2000,
  networkType = 'testnet',
  onSendTransaction,
  onReceiveRequest
}) => {
  const [activeTab, setActiveTab] = useState<'send' | 'receive'>('send');
  const [transactionForm, setTransactionForm] = useState<TransactionForm>({
    recipientAddress: '',
    amount: 0,
    gasType: 'standard',
    memo: '',
    priority: 'medium'
  });
  const [gasFeeEstimate, setGasFeeEstimate] = useState<GasFeeEstimate | null>(null);
  const [isProcessing, setIsProcessing] = useState(false);
  const [showQR, setShowQR] = useState(false);
  const [copied, setCopied] = useState(false);

  // Gas fee rates based on network type
  const gasFeeRates = {
    testnet: {
      standard: { base: 0, priority: 0, time: 'Instant' },
      fast: { base: 0, priority: 0, time: 'Instant' },
      instant: { base: 0, priority: 0, time: 'Instant' }
    },
    mainnet: {
      standard: { base: 1, priority: 0.5, time: '2-5 minutes' },
      fast: { base: 2, priority: 1, time: '30-60 seconds' },
      instant: { base: 5, priority: 2, time: '10-30 seconds' }
    }
  };

  // Calculate gas fee estimate
  useEffect(() => {
    if (transactionForm.amount > 0) {
      const rates = gasFeeRates[networkType][transactionForm.gasType as keyof typeof gasFeeRates.mainnet];
      const priorityMultiplier = transactionForm.priority === 'high' ? 1.5 : 
                                transactionForm.priority === 'low' ? 0.7 : 1;
      
      setGasFeeEstimate({
        type: transactionForm.gasType,
        baseFee: rates.base,
        priorityFee: rates.priority * priorityMultiplier,
        totalFee: (rates.base + rates.priority * priorityMultiplier),
        estimatedTime: rates.time
      });
    }
  }, [transactionForm.amount, transactionForm.gasType, transactionForm.priority, networkType]);

  const handleSendTransaction = async () => {
    if (!transactionForm.recipientAddress || transactionForm.amount <= 0) {
      return;
    }

    setIsProcessing(true);
    try {
      await onSendTransaction?.(transactionForm);
      // Reset form on success
      setTransactionForm({
        recipientAddress: '',
        amount: 0,
        gasType: 'standard',
        memo: '',
        priority: 'medium'
      });
    } catch (error) {
      console.error('Transaction failed:', error);
    } finally {
      setIsProcessing(false);
    }
  };

  const copyToClipboard = async (text: string) => {
    await navigator.clipboard.writeText(text);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const validateAddress = (address: string) => {
    return address.startsWith('bpi1') && address.length === 42;
  };

  const isFormValid = () => {
    return validateAddress(transactionForm.recipientAddress) && 
           transactionForm.amount > 0 && 
           transactionForm.amount <= currentBalance;
  };

  return (
    <div className="space-y-6">
      {/* Tab Navigation */}
      <Card>
        <CardHeader>
          <div className="flex space-x-1 bg-gray-100 p-1 rounded-lg">
            <Button
              variant={activeTab === 'send' ? 'default' : 'ghost'}
              onClick={() => setActiveTab('send')}
              className="flex-1"
            >
              <Send className="h-4 w-4 mr-2" />
              Send BPI
            </Button>
            <Button
              variant={activeTab === 'receive' ? 'default' : 'ghost'}
              onClick={() => setActiveTab('receive')}
              className="flex-1"
            >
              <Download className="h-4 w-4 mr-2" />
              Receive BPI
            </Button>
          </div>
        </CardHeader>
      </Card>

      {/* Send Tab */}
      {activeTab === 'send' && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Send className="h-5 w-5" />
              Send BPI Coins
              <Badge variant={networkType === 'testnet' ? 'secondary' : 'default'}>
                {networkType.toUpperCase()}
              </Badge>
            </CardTitle>
            <p className="text-sm text-muted-foreground">
              Available Balance: <span className="font-medium">{currentBalance.toLocaleString()} BPI</span>
            </p>
          </CardHeader>
          <CardContent className="space-y-4">
            {/* Recipient Address */}
            <div>
              <Label htmlFor="recipient">Recipient Address</Label>
              <Input
                id="recipient"
                placeholder="bpi1x7k9m2n8q4r5t6u7v8w9x0y1z2a3b4c5d6e7f8"
                value={transactionForm.recipientAddress}
                onChange={(e) => setTransactionForm({
                  ...transactionForm,
                  recipientAddress: e.target.value
                })}
                className={!validateAddress(transactionForm.recipientAddress) && transactionForm.recipientAddress ? 'border-red-500' : ''}
              />
              {transactionForm.recipientAddress && !validateAddress(transactionForm.recipientAddress) && (
                <p className="text-xs text-red-500 mt-1">Invalid BPI address format</p>
              )}
            </div>

            {/* Amount */}
            <div>
              <Label htmlFor="amount">Amount (BPI)</Label>
              <Input
                id="amount"
                type="number"
                min="0"
                max={currentBalance}
                placeholder="0"
                value={transactionForm.amount || ''}
                onChange={(e) => setTransactionForm({
                  ...transactionForm,
                  amount: Number(e.target.value)
                })}
              />
              <div className="flex justify-between text-xs text-muted-foreground mt-1">
                <span>Min: 0.1 BPI</span>
                <span>Max: {currentBalance.toLocaleString()} BPI</span>
              </div>
            </div>

            {/* Gas Type Selection */}
            <div>
              <Label>Transaction Speed</Label>
              <div className="grid grid-cols-3 gap-2 mt-2">
                {Object.entries(gasFeeRates[networkType]).map(([type, rates]) => (
                  <Button
                    key={type}
                    variant={transactionForm.gasType === type ? 'default' : 'outline'}
                    onClick={() => setTransactionForm({
                      ...transactionForm,
                      gasType: type
                    })}
                    className="flex flex-col h-auto p-3"
                  >
                    <div className="font-medium capitalize">{type}</div>
                    <div className="text-xs text-muted-foreground">
                      {networkType === 'testnet' ? 'Free' : `${rates.base + rates.priority} BPI`}
                    </div>
                    <div className="text-xs">{rates.time}</div>
                  </Button>
                ))}
              </div>
            </div>

            {/* Priority */}
            <div>
              <Label>Priority</Label>
              <div className="flex gap-2 mt-2">
                {(['low', 'medium', 'high'] as const).map((priority) => (
                  <Button
                    key={priority}
                    variant={transactionForm.priority === priority ? 'default' : 'outline'}
                    onClick={() => setTransactionForm({
                      ...transactionForm,
                      priority
                    })}
                    size="sm"
                  >
                    {priority.charAt(0).toUpperCase() + priority.slice(1)}
                  </Button>
                ))}
              </div>
            </div>

            {/* Memo */}
            <div>
              <Label htmlFor="memo">Memo (Optional)</Label>
              <Textarea
                id="memo"
                placeholder="Add a note for this transaction..."
                value={transactionForm.memo}
                onChange={(e) => setTransactionForm({
                  ...transactionForm,
                  memo: e.target.value
                })}
                rows={2}
              />
            </div>

            {/* Gas Fee Estimate */}
            {gasFeeEstimate && (
              <Card className="bg-blue-50 border-blue-200">
                <CardContent className="p-4">
                  <div className="flex items-center gap-2 mb-2">
                    <Calculator className="h-4 w-4 text-blue-600" />
                    <span className="font-medium text-blue-800">Transaction Summary</span>
                  </div>
                  <div className="space-y-2 text-sm">
                    <div className="flex justify-between">
                      <span>Amount:</span>
                      <span className="font-medium">{transactionForm.amount.toLocaleString()} BPI</span>
                    </div>
                    <div className="flex justify-between">
                      <span>Gas Fee:</span>
                      <span className="font-medium">
                        {networkType === 'testnet' ? 'FREE' : `${gasFeeEstimate.totalFee} BPI`}
                      </span>
                    </div>
                    <div className="flex justify-between">
                      <span>Estimated Time:</span>
                      <span className="font-medium">{gasFeeEstimate.estimatedTime}</span>
                    </div>
                    <div className="border-t pt-2 flex justify-between font-medium">
                      <span>Total Cost:</span>
                      <span>
                        {networkType === 'testnet' 
                          ? `${transactionForm.amount.toLocaleString()} BPI` 
                          : `${(transactionForm.amount + gasFeeEstimate.totalFee).toLocaleString()} BPI`
                        }
                      </span>
                    </div>
                  </div>
                </CardContent>
              </Card>
            )}

            {/* Testnet Notice */}
            {networkType === 'testnet' && (
              <Alert>
                <Shield className="h-4 w-4" />
                <AlertDescription>
                  Testnet transactions are free and refundable. No real BPI coins will be transferred.
                </AlertDescription>
              </Alert>
            )}

            {/* Send Button */}
            <Button
              onClick={handleSendTransaction}
              disabled={!isFormValid() || isProcessing}
              className="w-full"
              size="lg"
            >
              {isProcessing ? (
                <>
                  <Clock className="h-4 w-4 mr-2 animate-spin" />
                  Processing Transaction...
                </>
              ) : (
                <>
                  <Send className="h-4 w-4 mr-2" />
                  Send {transactionForm.amount.toLocaleString()} BPI
                </>
              )}
            </Button>
          </CardContent>
        </Card>
      )}

      {/* Receive Tab */}
      {activeTab === 'receive' && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Download className="h-5 w-5" />
              Receive BPI Coins
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            {/* Wallet Address */}
            <div>
              <Label>Your BPI Address</Label>
              <div className="flex gap-2 mt-2">
                <Input
                  value={walletAddress}
                  readOnly
                  className="font-mono text-sm"
                />
                <Button
                  variant="outline"
                  onClick={() => copyToClipboard(walletAddress)}
                  className="shrink-0"
                >
                  {copied ? <CheckCircle className="h-4 w-4" /> : <Copy className="h-4 w-4" />}
                </Button>
              </div>
              {copied && (
                <p className="text-xs text-green-600 mt-1">Address copied to clipboard!</p>
              )}
            </div>

            {/* QR Code */}
            <div className="text-center">
              <Button
                variant="outline"
                onClick={() => setShowQR(!showQR)}
                className="mb-4"
              >
                <QrCode className="h-4 w-4 mr-2" />
                {showQR ? 'Hide' : 'Show'} QR Code
              </Button>
              
              {showQR && (
                <div className="bg-white p-4 border rounded-lg inline-block">
                  <div className="w-48 h-48 bg-gray-200 flex items-center justify-center rounded">
                    <QrCode className="h-16 w-16 text-gray-400" />
                    <span className="ml-2 text-gray-500">QR Code</span>
                  </div>
                  <p className="text-xs text-muted-foreground mt-2">
                    Scan to send BPI to this address
                  </p>
                </div>
              )}
            </div>

            {/* Instructions */}
            <Alert>
              <Wallet className="h-4 w-4" />
              <AlertDescription>
                Share your BPI address with others to receive payments. 
                {networkType === 'testnet' && ' Testnet transactions are free and for testing only.'}
              </AlertDescription>
            </Alert>

            {/* Request Payment */}
            <Card className="bg-gradient-to-r from-green-50 to-blue-50">
              <CardContent className="p-4">
                <h4 className="font-medium mb-2">Request Payment</h4>
                <p className="text-sm text-muted-foreground mb-3">
                  Generate a payment request link to share with others
                </p>
                <Button onClick={onReceiveRequest} variant="outline" className="w-full">
                  <ArrowRight className="h-4 w-4 mr-2" />
                  Create Payment Request
                </Button>
              </CardContent>
            </Card>
          </CardContent>
        </Card>
      )}
    </div>
  );
};

export default CoinTransactionUI;
