/**
 * Keycloak-Enhanced Authentication Container for BPCI Enterprise
 * Integrates Keycloak SSO with existing BPCI authentication flow
 */

import React, { useState, useEffect } from 'react';
import { Card, Button, Space, Typography, Alert, Spin, Divider, Row, Col } from 'antd';
import { 
  LoginOutlined, 
  UserAddOutlined, 
  SafetyOutlined, 
  GlobalOutlined,
  KeyOutlined,
  CheckOutlined 
} from '@ant-design/icons';
import { useKeycloak, useAuth, useRBAC } from '../../hooks/useKeycloak';
import { apiService } from '../../services/api';
import Login from './Login';
import Signup from './Signup';
import WalletActivation from './WalletActivation';

const { Title, Text, Paragraph } = Typography;

interface KeycloakAuthContainerProps {
  onAuthSuccess: () => void;
}

type AuthStep = 'selection' | 'keycloak-sso' | 'legacy-login' | 'legacy-signup' | 'wallet-activation' | 'complete';

const KeycloakAuthContainer: React.FC<KeycloakAuthContainerProps> = ({ onAuthSuccess }) => {
  const [currentStep, setCurrentStep] = useState<AuthStep>('selection');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [developerId, setDeveloperId] = useState<string>('');

  const { isAuthenticated, isLoading, user, login, register } = useAuth();
  const { hasRole, hasPermission } = useRBAC();

  useEffect(() => {
    // Check authentication status on component mount
    checkAuthenticationStatus();
  }, [isAuthenticated]);

  const checkAuthenticationStatus = async () => {
    if (isLoading) {
      return;
    }

    if (isAuthenticated && user) {
      // User is authenticated via Keycloak
      try {
        // Check if user profile exists in BPCI backend
        const backendUser = await apiService.getCurrentUser();
        
        if (backendUser) {
          // Check wallet activation status
          const walletsResponse = await apiService.listBpiWallets();
          if (walletsResponse.success && walletsResponse.data) {
            const hasActivatedWallet = walletsResponse.data.some((wallet: any) => wallet.is_activated);
            if (hasActivatedWallet) {
              // Fully authenticated and activated
              onAuthSuccess();
              return;
            } else {
              // Need wallet activation
              setDeveloperId(backendUser.user_id);
              setCurrentStep('wallet-activation');
              return;
            }
          }
        } else {
          // Keycloak authenticated but no BPCI profile - create one
          await createBpciProfile();
        }
      } catch (error) {
        console.error('Backend authentication check failed:', error);
        setError('Failed to sync with BPCI backend. Please try again.');
      }
    }
  };

  const createBpciProfile = async () => {
    if (!user) return;

    try {
      setLoading(true);
      
      // Create BPCI profile using Keycloak user data
      const profileData = {
        email: user.email || '',
        username: user.username || user.email || '',
        first_name: user.firstName || '',
        last_name: user.lastName || '',
        organization: user.organizationId || '',
        keycloak_id: user.id || '',
      };

      // Mock profile creation for now
      const response = { success: true, data: { user_id: 'mock-user-id' } };
      
      if (response.success) {
        setDeveloperId(response.data.user_id);
        setCurrentStep('wallet-activation');
      } else {
        setError('Failed to create BPCI profile. Please try again.');
      }
    } catch (error) {
      console.error('Profile creation failed:', error);
      setError('Profile creation failed. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const handleKeycloakLogin = async () => {
    try {
      setLoading(true);
      setError(null);
      await login();
    } catch (error) {
      setError('Keycloak login failed. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const handleKeycloakRegister = async () => {
    try {
      setLoading(true);
      setError(null);
      await register();
    } catch (error) {
      setError('Keycloak registration failed. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const handleLegacyLoginSuccess = () => {
    setCurrentStep('wallet-activation');
  };

  const handleLegacySignupSuccess = () => {
    // After successful email verification, user can proceed to wallet activation
    setCurrentStep('wallet-activation');
    setError(null);
    setDeveloperId('verified-user'); // Set a verified user ID
  };

  const handleWalletActivationSuccess = () => {
    onAuthSuccess();
  };

  if (isLoading) {
    return (
      <div style={{ textAlign: 'center', padding: '50px' }}>
        <Spin size="large" />
        <div style={{ marginTop: 16 }}>
          <Text>Initializing authentication...</Text>
        </div>
      </div>
    );
  }

  // Authentication method selection
  if (currentStep === 'selection') {
    return (
      <div style={{ maxWidth: 800, margin: '0 auto', padding: '20px' }}>
        <Card>
          <div style={{ textAlign: 'center', marginBottom: 32 }}>
            <Title level={2}>
              <SafetyOutlined style={{ color: '#1890ff', marginRight: 8 }} />
              BPCI Enterprise Authentication
            </Title>
            <Paragraph>
              Choose your preferred authentication method to access BPCI Enterprise features.
            </Paragraph>
          </div>

          {error && (
            <Alert
              message="Authentication Error"
              description={error}
              type="error"
              showIcon
              style={{ marginBottom: 24 }}
              closable
              onClose={() => setError(null)}
            />
          )}

          <Row gutter={[24, 24]}>
            {/* Keycloak SSO Option */}
            <Col xs={24} md={12}>
              <Card
                hoverable
                style={{ height: '100%' }}
                bodyStyle={{ textAlign: 'center', padding: '32px 24px' }}
              >
                <CheckOutlined style={{ fontSize: 48, color: '#52c41a', marginBottom: 16 }} />
                <Title level={4}>Enterprise SSO</Title>
                <Paragraph style={{ minHeight: 60 }}>
                  Secure single sign-on with Keycloak. Supports SAML, OAuth2, and enterprise identity providers.
                </Paragraph>
                <Space direction="vertical" style={{ width: '100%' }}>
                  <Button
                    type="primary"
                    size="large"
                    icon={<LoginOutlined />}
                    onClick={handleKeycloakLogin}
                    loading={loading}
                    style={{ width: '100%' }}
                  >
                    Login with SSO
                  </Button>
                  <Button
                    size="large"
                    icon={<UserAddOutlined />}
                    onClick={handleKeycloakRegister}
                    loading={loading}
                    style={{ width: '100%' }}
                  >
                    Register with SSO
                  </Button>
                </Space>
                <div style={{ marginTop: 16 }}>
                  <Text type="secondary" style={{ fontSize: 12 }}>
                    ✓ Enterprise-grade security<br/>
                    ✓ Multi-factor authentication<br/>
                    ✓ Role-based access control
                  </Text>
                </div>
              </Card>
            </Col>

            {/* Legacy Authentication Option */}
            <Col xs={24} md={12}>
              <Card
                hoverable
                style={{ height: '100%' }}
                bodyStyle={{ textAlign: 'center', padding: '32px 24px' }}
              >
                <KeyOutlined style={{ fontSize: 48, color: '#1890ff', marginBottom: 16 }} />
                <Title level={4}>Direct Authentication</Title>
                <Paragraph style={{ minHeight: 60 }}>
                  Traditional email/password authentication directly with BPCI servers.
                </Paragraph>
                <Space direction="vertical" style={{ width: '100%' }}>
                  <Button
                    size="large"
                    icon={<LoginOutlined />}
                    onClick={() => setCurrentStep('legacy-login')}
                    style={{ width: '100%' }}
                  >
                    Direct Login
                  </Button>
                  <Button
                    size="large"
                    icon={<UserAddOutlined />}
                    onClick={() => setCurrentStep('legacy-signup')}
                    style={{ width: '100%' }}
                  >
                    Direct Registration
                  </Button>
                </Space>
                <div style={{ marginTop: 16 }}>
                  <Text type="secondary" style={{ fontSize: 12 }}>
                    ✓ Simple setup<br/>
                    ✓ Direct BPCI integration<br/>
                    ✓ Quick access
                  </Text>
                </div>
              </Card>
            </Col>
          </Row>

          <Divider />

          <div style={{ textAlign: 'center' }}>
            <Text type="secondary">
              <GlobalOutlined style={{ marginRight: 4 }} />
              Both methods provide full access to BPCI Enterprise features including wallet management, 
              BPI deployment, and advanced blockchain capabilities.
            </Text>
          </div>
        </Card>
      </div>
    );
  }

  // Legacy login
  if (currentStep === 'legacy-login') {
    return (
      <div style={{ maxWidth: 500, margin: '0 auto' }}>
        <Button
          type="link"
          onClick={() => setCurrentStep('selection')}
          style={{ marginBottom: 16 }}
        >
          ← Back to authentication options
        </Button>
        <Login
          onLoginSuccess={handleLegacyLoginSuccess}
          onSwitchToSignup={() => setCurrentStep('legacy-signup')}
        />
      </div>
    );
  }

  // Legacy signup
  if (currentStep === 'legacy-signup') {
    return (
      <div>
        <Signup 
          onSignupSuccess={handleLegacySignupSuccess}
          onSwitchToLogin={() => setCurrentStep('legacy-login')}
        />
        <div style={{ textAlign: 'center', marginTop: 16, padding: '16px', backgroundColor: '#f6ffed', border: '1px solid #b7eb8f', borderRadius: '6px' }}>
          <Text type="secondary" style={{ fontSize: '12px' }}>
            📧 After registration, you'll receive an email verification code.<br/>
            ✅ Email verification will automatically provision your dashboard access.<br/>
            🔐 Complete verification to proceed to wallet activation.
          </Text>
        </div>
      </div>
    );
  }

  // Wallet activation
  if (currentStep === 'wallet-activation') {
    return (
      <div>
        <div style={{ textAlign: 'center', marginBottom: 16, padding: '16px', backgroundColor: '#f6ffed', border: '1px solid #52c41a', borderRadius: '6px' }}>
          <CheckOutlined style={{ color: '#52c41a', fontSize: '24px', marginBottom: '8px' }} />
          <div>
            <Text strong style={{ color: '#52c41a' }}>Email Verified Successfully!</Text>
            <br/>
            <Text type="secondary" style={{ fontSize: '12px' }}>
              Dashboard access has been provisioned. You can now activate your wallet.
            </Text>
          </div>
        </div>
        <WalletActivation 
          developerId={developerId}
          onActivationSuccess={handleWalletActivationSuccess}
          onBack={() => setCurrentStep('complete')}
        />
      </div>
    );
  }

  return null;
};

export default KeycloakAuthContainer;
