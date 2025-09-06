import React, { useState, useEffect } from 'react';
import { apiService } from '../../services/api';
import Login from './Login';
import Signup from './Signup.tsx';
import WalletActivation from './WalletActivation';

interface AuthContainerProps {
  onAuthSuccess: () => void;
}

type AuthStep = 'login' | 'signup' | 'wallet-activation' | 'complete';

const AuthContainer: React.FC<AuthContainerProps> = ({ onAuthSuccess }) => {
  const [currentStep, setCurrentStep] = useState<AuthStep>('login');
  const [loading, setLoading] = useState(true);
  const [developerId, setDeveloperId] = useState<string>('');

  useEffect(() => {
    // Check if user is already authenticated and has wallet activated
    const checkAuth = async () => {
      try {
        if (apiService.isAuthenticated()) {
          const user = await apiService.getCurrentUser();
          if (user) {
            // Check if user has any activated wallets
            const walletsResponse = await apiService.listWallets();
            if (walletsResponse.success && walletsResponse.data) {
              const hasActivatedWallet = walletsResponse.data.some(wallet => wallet.is_activated);
              if (hasActivatedWallet) {
                // Fully authenticated with wallet activated
                onAuthSuccess();
                return;
              } else {
                // Profile created but wallet not activated
                setDeveloperId(user.user_id);
                setCurrentStep('wallet-activation');
              }
            }
          }
        }
      } catch (error) {
        console.error('Auth check failed:', error);
      } finally {
        setLoading(false);
      }
    };

    checkAuth();
  }, [onAuthSuccess]);

  const handleLoginSuccess = async () => {
    const user = await apiService.getCurrentUser();
    if (user) {
      const walletsResponse = await apiService.listWallets();
      if (walletsResponse.success && walletsResponse.data) {
        const hasActivatedWallet = walletsResponse.data.some(wallet => wallet.is_activated);
        if (hasActivatedWallet) {
          onAuthSuccess();
        } else {
          setDeveloperId(user.user_id);
          setCurrentStep('wallet-activation');
        }
      }
    }
  };

  const handleSignupSuccess = async () => {
    // After successful signup, get the current user and proceed to wallet activation
    const user = await apiService.getCurrentUser();
    if (user) {
      setDeveloperId(user.user_id);
      setCurrentStep('wallet-activation');
    }
  };

  const handleWalletActivationSuccess = () => {
    onAuthSuccess();
  };

  const switchToSignup = () => {
    setCurrentStep('signup');
  };

  const switchToLogin = () => {
    setCurrentStep('login');
  };

  const backToProfile = () => {
    setCurrentStep('login');
  };

  if (loading) {
    return (
      <div style={{ 
        display: 'flex', 
        justifyContent: 'center', 
        alignItems: 'center', 
        height: '100vh' 
      }}>
        <div>Loading...</div>
      </div>
    );
  }

  return (
    <div style={{ 
      minHeight: '100vh', 
      background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
      padding: '20px'
    }}>
      {currentStep === 'login' && (
        <Login 
          onLoginSuccess={handleLoginSuccess}
          onSwitchToSignup={switchToSignup}
        />
      )}
      
      {currentStep === 'signup' && (
        <Signup 
          onSignupSuccess={handleSignupSuccess}
          onSwitchToLogin={switchToLogin}
        />
      )}
      
      {currentStep === 'wallet-activation' && (
        <WalletActivation 
          developerId={developerId}
          onActivationSuccess={handleWalletActivationSuccess}
          onBack={backToProfile}
        />
      )}
    </div>
  );
};

export default AuthContainer;
