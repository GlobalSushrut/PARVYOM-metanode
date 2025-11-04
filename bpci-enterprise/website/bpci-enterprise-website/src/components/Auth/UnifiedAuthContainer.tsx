/**
 * Unified Authentication Container for BPCI Enterprise
 * Integrates email/password authentication with Keycloak-Cloudflare SSO
 * Provides seamless signup and login experience
 */

import React, { useState, useEffect } from 'react';
import { Form, Input, Button, Alert, Typography, Card, Tabs, Checkbox, message, Space, Divider } from 'antd';
import { 
  UserOutlined, 
  LockOutlined, 
  MailOutlined, 
  SafetyOutlined, 
  GoogleOutlined,
  GithubOutlined,
  KeyOutlined
} from '@ant-design/icons';
import { unifiedAuthService, AuthMethods } from '../../services/unifiedAuthService';
import type { AuthMethod, UnifiedUserProfile, UnifiedAuthResponse } from '../../services/unifiedAuthService';

const { Title, Text, Link } = Typography;
const { TabPane } = Tabs;

interface UnifiedAuthContainerProps {
  onAuthSuccess: (user: UnifiedUserProfile) => void;
  defaultTab?: 'login' | 'signup';
  showSSOOptions?: boolean;
  showWelcomeMessage?: boolean;
}

interface LoginFormData {
  email: string;
  password: string;
  remember?: boolean;
}

interface SignupFormData {
  name: string;
  email: string;
  password: string;
  confirmPassword: string;
  company?: string;
  role?: string;
  agreeToTerms: boolean;
}

const UnifiedAuthContainer: React.FC<UnifiedAuthContainerProps> = ({
  onAuthSuccess,
  defaultTab = 'login',
  showSSOOptions = true,
  showWelcomeMessage = true,
}) => {
  const [activeTab, setActiveTab] = useState(defaultTab);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [loginForm] = Form.useForm();
  const [signupForm] = Form.useForm();

  // Handle unified authentication
  const handleAuth = async (method: AuthMethod, data: any) => {
    setLoading(true);
    setError(null);

    try {
      const response = await unifiedAuthService.authenticate(method, data);
      
      if (response.success && response.user) {
        message.success(`Welcome ${response.user.name || response.user.email}!`);
        onAuthSuccess(response.user);
      } else {
        setError(response.message || 'Authentication failed');
      }
    } catch (error: any) {
      console.error('Authentication error:', error);
      setError(error.message || 'Authentication failed. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    // Subscribe to authentication state changes
    const unsubscribe = unifiedAuthService.subscribe((user) => {
      if (user && user.isAuthenticated) {
        onAuthSuccess(user);
      }
    });

    // Check initial authentication state
    const user = unifiedAuthService.getCurrentUser();
    if (user && user.isAuthenticated) {
      setCurrentUser(user);
      onAuthSuccess(user);
    }

    return unsubscribe;
  }, [onAuthSuccess]);

  /**
   * Handle email/password login
   */
  const handleLogin = async (values: LoginFormData) => {
    setLoading(true);
    setError(null);
    setSuccess(null);

    try {
      const response: UnifiedAuthResponse = await unifiedAuthService.login(
        {
          email: values.email,
          password: values.password,
        },
        AuthMethods.EMAIL_PASSWORD
      );

      if (response.success && response.user) {
        setSuccess('Successfully logged in!');
        message.success('Welcome back!');
        
        // Handle additional requirements
        if (response.requiresVerification) {
          message.info('Please verify your email to continue.');
        }
        if (response.requiresWalletActivation) {
          message.info('Complete wallet activation to access all features.');
        }
        
        onAuthSuccess(response.user);
      } else {
        setError(response.message || 'Login failed');
        message.error(response.message || 'Login failed');
      }
    } catch (error) {
      const errorMessage = 'Network error. Please try again.';
      setError(errorMessage);
      message.error(errorMessage);
      console.error('Login error:', error);
    } finally {
      setLoading(false);
    }
  };

  /**
   * Handle user signup
   */
  const handleSignup = async (values: SignupFormData) => {
    if (values.password !== values.confirmPassword) {
      setError('Passwords do not match');
      return;
    }

    if (!values.agreeToTerms) {
      setError('Please agree to the terms and conditions');
      return;
    }

    setLoading(true);
    setError(null);
    setSuccess(null);

    try {
      const response: UnifiedAuthResponse = await unifiedAuthService.signup({
        name: values.name,
        email: values.email,
        password: values.password,
        company: values.company,
        role: values.role,
      });

      if (response.success) {
        setSuccess('Account created successfully!');
        message.success('Welcome to BPCI Enterprise!');
        
        if (response.user) {
          onAuthSuccess(response.user);
        } else if (response.requiresVerification) {
          message.info('Please check your email for verification instructions.');
          setActiveTab('login');
        }
      } else {
        setError(response.message || 'Signup failed');
        message.error(response.message || 'Signup failed');
      }
    } catch (error) {
      const errorMessage = 'Network error. Please try again.';
      setError(errorMessage);
      message.error(errorMessage);
      console.error('Signup error:', error);
    } finally {
      setLoading(false);
    }
  };

  /**
   * Handle SSO login
   */
  const handleSSOLogin = async () => {
    setSsoLoading(true);
    setError(null);

    try {
      const response: UnifiedAuthResponse = await unifiedAuthService.login(
        { email: '', password: '' }, // Not used for SSO
        AuthMethods.KEYCLOAK_SSO
      );

      if (response.success && response.user) {
        message.success('Successfully logged in via SSO!');
        onAuthSuccess(response.user);
      } else {
        setError('SSO login failed. Please try again.');
        message.error('SSO login failed');
      }
    } catch (error) {
      const errorMessage = 'SSO login failed. Please try again.';
      setError(errorMessage);
      message.error(errorMessage);
      console.error('SSO login error:', error);
    } finally {
      setSsoLoading(false);
    }
  };

  /**
   * Render login form
   */
  const renderLoginForm = () => (
    <Form
      form={loginForm}
      name="login"
      onFinish={handleLogin}
      layout="vertical"
      size="large"
    >
      <Form.Item
        name="email"
        label="Email Address"
        rules={[
          { required: true, message: 'Please enter your email' },
          { type: 'email', message: 'Please enter a valid email' },
        ]}
      >
        <Input
          prefix={<MailOutlined />}
          placeholder="Enter your email"
          autoComplete="email"
        />
      </Form.Item>

      <Form.Item
        name="password"
        label="Password"
        rules={[{ required: true, message: 'Please enter your password' }]}
      >
        <Input.Password
          prefix={<LockOutlined />}
          placeholder="Enter your password"
          autoComplete="current-password"
        />
      </Form.Item>

      <Form.Item>
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
          <Form.Item name="remember" valuePropName="checked" noStyle>
            <Checkbox>Remember me</Checkbox>
          </Form.Item>
          <Link href="#" onClick={() => message.info('Password reset coming soon!')}>
            Forgot password?
          </Link>
        </div>
      </Form.Item>

      <Form.Item>
        <Button
          type="primary"
          htmlType="submit"
          loading={loading}
          block
          size="large"
          style={{ height: '48px', fontSize: '16px' }}
        >
          Sign In
        </Button>
      </Form.Item>
    </Form>
  );

  /**
   * Render signup form
   */
  const renderSignupForm = () => (
    <Form
      form={signupForm}
      name="signup"
      onFinish={handleSignup}
      layout="vertical"
      size="large"
    >
      <Form.Item
        name="name"
        label="Full Name"
        rules={[{ required: true, message: 'Please enter your full name' }]}
      >
        <Input
          prefix={<UserOutlined />}
          placeholder="Enter your full name"
          autoComplete="name"
        />
      </Form.Item>

      <Form.Item
        name="email"
        label="Email Address"
        rules={[
          { required: true, message: 'Please enter your email' },
          { type: 'email', message: 'Please enter a valid email' },
        ]}
      >
        <Input
          prefix={<MailOutlined />}
          placeholder="Enter your email"
          autoComplete="email"
        />
      </Form.Item>

      <Form.Item
        name="password"
        label="Password"
        rules={[
          { required: true, message: 'Please enter a password' },
          { min: 8, message: 'Password must be at least 8 characters' },
        ]}
      >
        <Input.Password
          prefix={<LockOutlined />}
          placeholder="Create a password"
          autoComplete="new-password"
        />
      </Form.Item>

      <Form.Item
        name="confirmPassword"
        label="Confirm Password"
        rules={[
          { required: true, message: 'Please confirm your password' },
          ({ getFieldValue }) => ({
            validator(_, value) {
              if (!value || getFieldValue('password') === value) {
                return Promise.resolve();
              }
              return Promise.reject(new Error('Passwords do not match'));
            },
          }),
        ]}
      >
        <Input.Password
          prefix={<SafetyOutlined />}
          placeholder="Confirm your password"
          autoComplete="new-password"
        />
      </Form.Item>

      <Form.Item
        name="company"
        label="Company (Optional)"
      >
        <Input placeholder="Your company or organization" />
      </Form.Item>

      <Form.Item
        name="role"
        label="Role (Optional)"
      >
        <Input placeholder="Your role or title" />
      </Form.Item>

      <Form.Item
        name="agreeToTerms"
        valuePropName="checked"
        rules={[
          { required: true, message: 'Please agree to the terms and conditions' },
        ]}
      >
        <Checkbox>
          I agree to the <Link href="#" target="_blank">Terms of Service</Link> and{' '}
          <Link href="#" target="_blank">Privacy Policy</Link>
        </Checkbox>
      </Form.Item>

      <Form.Item>
        <Button
          type="primary"
          htmlType="submit"
          loading={loading}
          block
          size="large"
          style={{ height: '48px', fontSize: '16px' }}
        >
          Create Account
        </Button>
      </Form.Item>
    </Form>
  );

  /**
   * Render SSO options
   */
  const renderSSOOptions = () => (
    <div style={{ marginTop: '24px' }}>
      <Divider>
        <Text type="secondary">Or continue with</Text>
      </Divider>
      
      <Space direction="vertical" style={{ width: '100%' }} size="middle">
        <Button
          icon={<KeyOutlined />}
          onClick={handleSSOLogin}
          loading={ssoLoading}
          block
          size="large"
          style={{ height: '48px' }}
        >
          Enterprise SSO
        </Button>
        
        <Space style={{ width: '100%' }} size="middle">
          <Button
            icon={<GoogleOutlined />}
            onClick={() => message.info('Google OAuth coming soon!')}
            style={{ flex: 1, height: '40px' }}
          >
            Google
          </Button>
          <Button
            icon={<GithubOutlined />}
            onClick={() => message.info('GitHub OAuth coming soon!')}
            style={{ flex: 1, height: '40px' }}
          >
            GitHub
          </Button>
        </Space>
      </Space>
    </div>
  );

  return (
    <div style={{ 
      minHeight: '100vh',
      background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
      padding: '20px'
    }}>
      <Card
        style={{
          width: '100%',
          maxWidth: '480px',
          boxShadow: '0 8px 32px rgba(0,0,0,0.12)',
          borderRadius: '16px',
        }}
      >
        {showWelcomeMessage && (
          <div style={{ textAlign: 'center', marginBottom: '32px' }}>
            <Title level={2} style={{ marginBottom: '8px', color: '#1890ff' }}>
              🔗 BPCI Enterprise
            </Title>
            <Text type="secondary" style={{ fontSize: '16px' }}>
              Secure blockchain infrastructure platform
            </Text>
          </div>
        )}

        {error && (
          <Alert
            message={error}
            type="error"
            showIcon
            closable
            style={{ marginBottom: '24px' }}
            onClose={() => setError(null)}
          />
        )}

        {success && (
          <Alert
            message={success}
            type="success"
            showIcon
            closable
            style={{ marginBottom: '24px' }}
            onClose={() => setSuccess(null)}
          />
        )}

        <Tabs
          activeKey={activeTab}
          onChange={(key) => setActiveTab(key as 'login' | 'signup')}
          centered
          size="large"
        >
          <TabPane tab="Sign In" key="login">
            {renderLoginForm()}
            {showSSOOptions && renderSSOOptions()}
            
            <div style={{ textAlign: 'center', marginTop: '24px' }}>
              <Text type="secondary">
                Don't have an account?{' '}
                <Link onClick={() => setActiveTab('signup')}>
                  Sign up now
                </Link>
              </Text>
            </div>
          </TabPane>

          <TabPane tab="Sign Up" key="signup">
            {renderSignupForm()}
            
            <div style={{ textAlign: 'center', marginTop: '24px' }}>
              <Text type="secondary">
                Already have an account?{' '}
                <Link onClick={() => setActiveTab('login')}>
                  Sign in here
                </Link>
              </Text>
            </div>
          </TabPane>
        </Tabs>
      </Card>
    </div>
  );
};

export default UnifiedAuthContainer;
