import React, { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '../ui/card';
import { Button } from '../ui/button';
import { Input } from '../ui/input';
import { Label } from '../ui/label';
import { Badge } from '../ui/badge';
import { Separator } from '../ui/separator';
import { Alert, AlertDescription } from '../ui/alert';
import { 
  CreditCard, 
  Shield, 
  Zap, 
  Clock, 
  Calculator,
  CheckCircle,
  AlertCircle,
  Coins,
  TrendingUp,
  Lock
} from 'lucide-react';

interface PaymentPlan {
  id: string;
  name: string;
  bpi: number;
  usd: number;
  description: string;
  features: string[];
  recommended?: boolean;
  estimatedUsage: string;
  isConsultation?: boolean;
}

interface StripePaymentPageProps {
  currentBalance?: number;
  networkType?: 'testnet' | 'mainnet';
  onPaymentSuccess?: (amount: number) => void;
  onPaymentError?: (error: string) => void;
}

export const StripePaymentPage: React.FC<StripePaymentPageProps> = ({
  currentBalance = 2000,
  networkType = 'testnet',
  onPaymentSuccess,
  onPaymentError
}) => {
  const [selectedPlan, setSelectedPlan] = useState<string>('starter');
  const [customAmount, setCustomAmount] = useState<number>(0);
  const [isProcessing, setIsProcessing] = useState(false);
  const [paymentMethod, setPaymentMethod] = useState<'card' | 'bank'>('card');
  const [showCalculator, setShowCalculator] = useState(false);

  // Payment plans for BPI coins
  const paymentPlans: PaymentPlan[] = [
    {
      id: 'testnet-monthly',
      name: 'Testnet Monthly',
      bpi: 1000,
      usd: 1000,
      description: 'Monthly BPI for testnet after free period',
      features: [
        '1000 BPI sufficient for 1 month',
        'Gas and rent fees included',
        'Perfect for continued testing',
        'After 2-month free period'
      ],
      estimatedUsage: '1 month testnet usage',
      recommended: true
    },
    {
      id: 'pilot',
      name: 'Pilot Program',
      bpi: 5000,
      usd: 5000,
      description: 'Monthly pilot program with infrastructure investment',
      features: [
        '5000 BPI per month',
        'Infrastructure investment required',
        'Production-grade support',
        'Advanced deployment capabilities',
        'Priority technical support'
      ],
      estimatedUsage: '1 month pilot program'
    },
    {
      id: 'consultation',
      name: 'Monthly Consultation',
      bpi: 0,
      usd: 100,
      description: 'Monthly expert support and guidance',
      features: [
        'Monthly expert consultation',
        'Architecture guidance',
        'Implementation advice', 
        'Best practices review',
        'Ongoing technical support'
      ],
      estimatedUsage: 'Monthly consultation service',
      isConsultation: true
    }
  ];

  // Usage calculator data
  const operationCosts = {
    containerDeploy: 5,
    poeBundle: 2,
    notarization: 1,
    validation: 1,
    crossChainBridge: 10,
    governanceProposal: 25,
    communityVoting: 0.5,
    vmRentPerHour: 2
  };

  const [calculatorInputs, setCalculatorInputs] = useState({
    containers: 10,
    poeBundles: 60,
    validations: 100,
    vmHours: 720 // 1 month
  });

  const calculateTotalCost = () => {
    const { containers, poeBundles, validations, vmHours } = calculatorInputs;
    return (
      containers * operationCosts.containerDeploy +
      poeBundles * operationCosts.poeBundle +
      validations * operationCosts.validation +
      vmHours * operationCosts.vmRentPerHour
    );
  };

  const handlePurchase = async (bpi: number, usd: number) => {
    setIsProcessing(true);
    
    try {
      // Simulate Stripe payment processing
      await new Promise(resolve => setTimeout(resolve, 2000));
      
      onPaymentSuccess?.(bpi);
    } catch (error) {
      onPaymentError?.('Payment failed. Please try again.');
    } finally {
      setIsProcessing(false);
    }
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <CreditCard className="h-5 w-5" />
            Purchase BPI Coins
            <Badge variant={networkType === 'testnet' ? 'secondary' : 'default'}>
              {networkType.toUpperCase()}
            </Badge>
          </CardTitle>
          <p className="text-sm text-muted-foreground">
            Current Balance: <span className="font-medium">{currentBalance.toLocaleString()} BPI</span>
          </p>
        </CardHeader>
      </Card>

      {/* Testnet Warning */}
      {networkType === 'testnet' && (
        <Alert>
          <AlertCircle className="h-4 w-4" />
          <AlertDescription>
            <strong>Testnet Pricing:</strong> New users get 2000 BPI free for the first 2 months. After that, you need 1000 BPI per month to run BPI Core in testnet. 1000 BPI is sufficient for 1 month of testnet operations including gas and rent fees.
          </AlertDescription>
        </Alert>
      )}

      {/* Payment Plans */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        {paymentPlans.map((plan) => (
          <Card 
            key={plan.id} 
            className={`cursor-pointer transition-all ${
              selectedPlan === plan.id ? 'ring-2 ring-blue-500' : ''
            } ${plan.recommended ? 'border-blue-500' : ''}`}
            onClick={() => setSelectedPlan(plan.id)}
          >
            <CardHeader>
              <div className="flex items-center justify-between">
                <CardTitle className="text-lg">{plan.name}</CardTitle>
                {plan.recommended && (
                  <Badge variant="secondary" className="mb-2">
                    {plan.name}
                  </Badge>
                )}
              </div>
              <div className="text-3xl font-bold">
                {plan.isConsultation ? '$100/month' : `${plan.bpi} BPI`}
              </div>
              <div className="text-lg text-muted-foreground">
                {plan.isConsultation ? 'Monthly Service' : `$${plan.usd}`}
              </div>
            </CardHeader>
            <CardContent>
              <p className="text-sm text-muted-foreground mb-3">
                {plan.description}
              </p>
              <div className="space-y-2">
                {plan.features.map((feature, index) => (
                  <div key={index} className="flex items-center gap-2 text-sm">
                    <CheckCircle className="h-3 w-3 text-green-500" />
                    {feature}
                  </div>
                ))}
              </div>
              <div className="mt-3 p-2 bg-blue-50 rounded text-xs text-blue-700">
                {plan.estimatedUsage}
              </div>
              <Button 
                className="w-full mt-4" 
                onClick={() => plan.isConsultation ? window.open('mailto:support@bpci.com?subject=Consultation Request', '_blank') : handlePurchase(plan.bpi, plan.usd)}
                disabled={isProcessing}
              >
                {plan.isConsultation ? 'Subscribe to Consultation' : `Purchase ${plan.name}`}
              </Button>
            </CardContent>
          </Card>
        ))}
      </div>

      {/* Custom Amount */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Calculator className="h-5 w-5" />
            Custom Amount
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <Label htmlFor="customAmount">BPI Amount (minimum 100)</Label>
              <Input
                id="customAmount"
                type="number"
                min="100"
                value={customAmount}
                onChange={(e: React.ChangeEvent<HTMLInputElement>) => setCustomAmount(Number(e.target.value))}
                placeholder="Enter BPI amount"
              />
            </div>
            <div>
              <Label>Total Cost</Label>
              <div className="text-2xl font-bold text-green-600">
                ${customAmount.toLocaleString()}
              </div>
            </div>
          </div>
          <Button 
            onClick={() => handlePurchase(customAmount, customAmount)}
            disabled={isProcessing || customAmount < 100}
            className="w-full"
          >
            {isProcessing ? 'Processing...' : `Purchase ${customAmount} BPI ($${customAmount})`}
          </Button>
        </CardContent>
      </Card>

      {/* Usage Calculator */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Calculator className="h-5 w-5" />
            Usage Calculator
            <Button 
              variant="ghost" 
              size="sm"
              onClick={() => setShowCalculator(!showCalculator)}
            >
              {showCalculator ? 'Hide' : 'Show'}
            </Button>
          </CardTitle>
        </CardHeader>
        {showCalculator && (
          <CardContent>
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
              <div>
                <Label>Container Deployments</Label>
                <Input
                  type="number"
                  value={calculatorInputs.containers}
                  onChange={(e) => setCalculatorInputs({
                    ...calculatorInputs,
                    containers: Number(e.target.value)
                  })}
                />
                <p className="text-xs text-muted-foreground mt-1">
                  {operationCosts.containerDeploy} BPI each
                </p>
              </div>
              <div>
                <Label>PoE Bundles</Label>
                <Input
                  type="number"
                  value={calculatorInputs.poeBundles}
                  onChange={(e) => setCalculatorInputs({
                    ...calculatorInputs,
                    poeBundles: Number(e.target.value)
                  })}
                />
                <p className="text-xs text-muted-foreground mt-1">
                  {operationCosts.poeBundle} BPI each
                </p>
              </div>
              <div>
                <Label>Validations</Label>
                <Input
                  type="number"
                  value={calculatorInputs.validations}
                  onChange={(e) => setCalculatorInputs({
                    ...calculatorInputs,
                    validations: Number(e.target.value)
                  })}
                />
                <p className="text-xs text-muted-foreground mt-1">
                  {operationCosts.validation} BPI each
                </p>
              </div>
              <div>
                <Label>VM Hours</Label>
                <Input
                  type="number"
                  value={calculatorInputs.vmHours}
                  onChange={(e) => setCalculatorInputs({
                    ...calculatorInputs,
                    vmHours: Number(e.target.value)
                  })}
                />
                <p className="text-xs text-muted-foreground mt-1">
                  {operationCosts.vmRentPerHour} BPI/hour
                </p>
              </div>
            </div>
            
            <Separator className="my-4" />
            
            <div className="bg-gradient-to-r from-blue-50 to-purple-50 p-4 rounded-lg">
              <div className="text-center">
                <div className="text-2xl font-bold text-blue-600">
                  {calculateTotalCost().toLocaleString()} BPI
                </div>
                <div className="text-lg text-muted-foreground">
                  ${calculateTotalCost().toLocaleString()} USD
                </div>
                <p className="text-sm text-muted-foreground mt-2">
                  Estimated cost for your usage pattern
                </p>
              </div>
            </div>
          </CardContent>
        )}
      </Card>

      {/* Payment Methods */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Lock className="h-5 w-5" />
            Payment Methods
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="flex items-center gap-3 p-3 border rounded-lg">
              <CreditCard className="h-5 w-5 text-blue-500" />
              <div>
                <div className="font-medium">Credit/Debit Card</div>
                <div className="text-sm text-muted-foreground">
                  Visa, Mastercard, American Express
                </div>
              </div>
            </div>
            <div className="flex items-center gap-3 p-3 border rounded-lg">
              <Shield className="h-5 w-5 text-green-500" />
              <div>
                <div className="font-medium">Bank Transfer</div>
                <div className="text-sm text-muted-foreground">
                  ACH, Wire Transfer (3-5 business days)
                </div>
              </div>
            </div>
          </div>
          
          <div className="mt-4 p-3 bg-green-50 border border-green-200 rounded-lg">
            <div className="flex items-center gap-2">
              <Shield className="h-4 w-4 text-green-600" />
              <span className="text-sm font-medium text-green-800">
                Secure Payment Processing by Stripe
              </span>
            </div>
            <p className="text-xs text-green-600 mt-1">
              Your payment information is encrypted and secure. We never store your card details.
            </p>
          </div>
        </CardContent>
      </Card>

      {/* 2-Month Analysis */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <TrendingUp className="h-5 w-5" />
            2-Month Deployment Analysis
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div className="p-4 bg-red-50 border border-red-200 rounded-lg">
              <div className="text-lg font-bold text-red-600">1500 BPI</div>
              <div className="text-sm text-red-700">❌ Not Sufficient</div>
              <p className="text-xs text-red-600 mt-1">
                For heavy mainnet usage (needs ~3250 BPI)
              </p>
            </div>
            <div className="p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
              <div className="text-lg font-bold text-yellow-600">1500 BPI</div>
              <div className="text-sm text-yellow-700">⚠️ Light Usage Only</div>
              <p className="text-xs text-yellow-600 mt-1">
                Sufficient for basic operations and testing
              </p>
            </div>
            <div className="p-4 bg-green-50 border border-green-200 rounded-lg">
              <div className="text-lg font-bold text-green-600">3500+ BPI</div>
              <div className="text-sm text-green-700">✅ Recommended</div>
              <p className="text-xs text-green-600 mt-1">
                Full 2-month heavy deployment coverage
              </p>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
};

export default StripePaymentPage;
