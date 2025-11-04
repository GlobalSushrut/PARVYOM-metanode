import React, { useState } from 'react';
import { Form, Input, Button, Alert, Typography, Checkbox, message, Modal } from 'antd';
import { UserOutlined, LockOutlined, WalletOutlined, SafetyOutlined, MailOutlined } from '@ant-design/icons';
import { authService } from '../../services/authService';
import { keycloakService } from '../../services/keycloakService';

const { Title, Text } = Typography;

interface LoginProps {
  onLoginSuccess: () => void;
  onSwitchToSignup: () => void;
}

type ForgotPasswordStep = 'email' | 'otp' | 'reset' | 'success';

const Login: React.FC<LoginProps> = ({ onLoginSuccess, onSwitchToSignup }) => {
  const [loading, setLoading] = useState(false);
  const [keycloakLoading, setKeycloakLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [form] = Form.useForm();
  
  // Forgot Password Modal State
  const [forgotPasswordVisible, setForgotPasswordVisible] = useState(false);
  const [forgotPasswordStep, setForgotPasswordStep] = useState<ForgotPasswordStep>('email');
  const [forgotPasswordEmail, setForgotPasswordEmail] = useState('');
  const [forgotPasswordOtp, setForgotPasswordOtp] = useState('');
  const [forgotPasswordLoading, setForgotPasswordLoading] = useState(false);
  const [forgotPasswordForm] = Form.useForm();
  
  // Dev Mode Toggle State
  const [devBypassEnabled, setDevBypassEnabled] = useState(
    localStorage.getItem('DEV_BYPASS_AUTH') === 'true'
  );

  const toggleDevBypass = () => {
    const newValue = !devBypassEnabled;
    if (newValue) {
      localStorage.setItem('DEV_BYPASS_AUTH', 'true');
      message.success('🔓 Dev bypass enabled! Refresh to skip login.');
    } else {
      localStorage.removeItem('DEV_BYPASS_AUTH');
      message.info('🔒 Dev bypass disabled! Normal login required.');
    }
    setDevBypassEnabled(newValue);
  };

  const onFinish = async (values: { email: string; password: string; remember?: boolean }) => {
    setLoading(true);
    setError(null);

    try {
      const response = await authService.login({
        email: values.email,
        password: values.password,
      });

      if (response.success) {
        message.success('Welcome back! Login successful.');
        onLoginSuccess();
      } else {
        setError(response.message || 'Invalid email or password. Please try again.');
      }
    } catch (error: any) {
      console.error('Login error:', error);
      setError(error.message || 'Login failed. Please check your credentials and try again.');
    } finally {
      setLoading(false);
    }
  };

  const handleKeycloakSSO = async () => {
    setKeycloakLoading(true);
    setError(null);

    try {
      await keycloakService.login();
      const authState = keycloakService.getAuthState();
      
      if (authState.isAuthenticated) {
        message.success('SSO login successful!');
        onLoginSuccess();
      } else {
        setError('SSO login failed. Please try again.');
      }
    } catch (error: any) {
      console.error('SSO login error:', error);
      setError(error.message || 'SSO login failed. Please try again.');
    } finally {
      setKeycloakLoading(false);
    }
  };

  const handleForgotPasswordEmail = async (values: { email: string }) => {
    setForgotPasswordLoading(true);
    try {
      // TODO: Call API to send OTP
      const response = await fetch(`${process.env.REACT_APP_API_URL || 'https://api.pravyom.com'}/api/auth/forgot-password`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email: values.email })
      });

      if (response.ok) {
        setForgotPasswordEmail(values.email);
        setForgotPasswordStep('otp');
        message.success('OTP sent to your email!');
      } else {
        message.error('Failed to send OTP. Please try again.');
      }
    } catch (error) {
      message.error('Failed to send OTP. Please try again.');
    } finally {
      setForgotPasswordLoading(false);
    }
  };

  const handleVerifyOtp = async (values: { otp: string }) => {
    setForgotPasswordLoading(true);
    try {
      // TODO: Call API to verify OTP
      const response = await fetch(`${process.env.REACT_APP_API_URL || 'https://api.pravyom.com'}/api/auth/verify-otp`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email: forgotPasswordEmail, otp: values.otp })
      });

      if (response.ok) {
        setForgotPasswordOtp(values.otp);
        setForgotPasswordStep('reset');
        message.success('OTP verified!');
      } else {
        message.error('Invalid OTP. Please try again.');
      }
    } catch (error) {
      message.error('Failed to verify OTP. Please try again.');
    } finally {
      setForgotPasswordLoading(false);
    }
  };

  const handleResetPassword = async (values: { password: string }) => {
    setForgotPasswordLoading(true);
    try {
      // TODO: Call API to reset password
      const response = await fetch(`${process.env.REACT_APP_API_URL || 'https://api.pravyom.com'}/api/auth/reset-password`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ 
          email: forgotPasswordEmail, 
          otp: forgotPasswordOtp,
          newPassword: values.password 
        })
      });

      if (response.ok) {
        setForgotPasswordStep('success');
        message.success('Password reset successful!');
      } else {
        message.error('Failed to reset password. Please try again.');
      }
    } catch (error) {
      message.error('Failed to reset password. Please try again.');
    } finally {
      setForgotPasswordLoading(false);
    }
  };

  const closeForgotPasswordModal = () => {
    setForgotPasswordVisible(false);
    setForgotPasswordStep('email');
    setForgotPasswordEmail('');
    setForgotPasswordOtp('');
    forgotPasswordForm.resetFields();
  };

  return (
    <div style={{ 
      minHeight: '100vh', 
      background: 'linear-gradient(135deg, #0A1628 0%, #1e293b 100%)',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
      padding: '2rem',
      position: 'relative'
    }}>
      {/* Dev Mode Toggle Button - Top Right */}
      <div style={{ position: 'absolute', top: '1rem', right: '1rem' }}>
        <Button
          onClick={toggleDevBypass}
          style={{
            background: devBypassEnabled ? 'rgba(16, 185, 129, 0.2)' : 'rgba(239, 68, 68, 0.2)',
            border: devBypassEnabled ? '1px solid #10B981' : '1px solid #EF4444',
            color: devBypassEnabled ? '#10B981' : '#EF4444',
            fontWeight: '600',
            fontSize: '0.875rem'
          }}
        >
          {devBypassEnabled ? '🔓 Dev Mode: ON' : '🔒 Dev Mode: OFF'}
        </Button>
      </div>

      <div style={{
        maxWidth: '480px',
        width: '100%',
        background: 'rgba(10, 22, 40, 0.95)',
        border: '2px solid rgba(232, 180, 79, 0.3)',
        borderRadius: '16px',
        padding: '3rem',
        backdropFilter: 'blur(10px)',
        boxShadow: '0 8px 32px rgba(0, 0, 0, 0.3)'
      }}>
        {/* Header */}
        <div style={{ textAlign: 'center', marginBottom: '2rem' }}>
          <WalletOutlined style={{ fontSize: '3rem', color: '#E8B44F', marginBottom: '1rem' }} />
          <Title level={2} style={{ color: '#ffffff', marginBottom: '0.5rem' }}>
            Welcome Back
          </Title>
          <Text style={{ color: '#9CA3AF', fontSize: '1rem' }}>
            Sign in to your Mojo wallet account
          </Text>
        </div>

        {/* Dev Mode Info */}
        {devBypassEnabled && (
          <Alert
            message="🔓 Developer Mode Active"
            description="Click the button above to disable, then refresh the page to skip login and access internal UI."
            type="success"
            showIcon
            style={{ marginBottom: '1.5rem' }}
          />
        )}

        {/* Error Alert */}
        {error && (
          <Alert
            message="Login Failed"
            description={error}
            type="error"
            showIcon
            closable
            onClose={() => setError(null)}
            style={{ marginBottom: '1.5rem' }}
          />
        )}

        {/* Login Form */}
        <Form
          form={form}
          name="login"
          onFinish={onFinish}
          layout="vertical"
          size="large"
          requiredMark={false}
        >
          <Form.Item
            name="email"
            label={<span style={{ color: '#E8B44F', fontWeight: '600' }}>Email Address</span>}
            rules={[
              { required: true, message: 'Please enter your email' },
              { type: 'email', message: 'Please enter a valid email' }
            ]}
          >
            <Input
              prefix={<UserOutlined style={{ color: '#9CA3AF' }} />}
              placeholder="your.email@example.com"
              style={{
                background: 'rgba(255, 255, 255, 0.05)',
                border: '1px solid rgba(232, 180, 79, 0.2)',
                color: '#ffffff',
                height: '48px'
              }}
            />
          </Form.Item>

          <Form.Item
            name="password"
            label={<span style={{ color: '#E8B44F', fontWeight: '600' }}>Password</span>}
            rules={[{ required: true, message: 'Please enter your password' }]}
          >
            <Input.Password
              prefix={<LockOutlined style={{ color: '#9CA3AF' }} />}
              placeholder="Enter your password"
              style={{
                background: 'rgba(255, 255, 255, 0.05)',
                border: '1px solid rgba(232, 180, 79, 0.2)',
                color: '#ffffff',
                height: '48px'
              }}
            />
          </Form.Item>

          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '1.5rem' }}>
            <Form.Item name="remember" valuePropName="checked" noStyle>
              <Checkbox style={{ color: '#ffffff' }}>
                <span style={{ color: '#ffffff' }}>Remember me</span>
              </Checkbox>
            </Form.Item>
            <a 
              onClick={() => setForgotPasswordVisible(true)}
              style={{ color: '#E8B44F', textDecoration: 'none', cursor: 'pointer' }}
            >
              Forgot password?
            </a>
          </div>

          <Form.Item style={{ marginBottom: '1rem' }}>
            <Button
              type="primary"
              htmlType="submit"
              loading={loading}
              block
              style={{
                background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)',
                border: 'none',
                color: '#0A1628',
                fontWeight: '600',
                height: '48px',
                fontSize: '1rem'
              }}
            >
              Sign In
            </Button>
          </Form.Item>
        </Form>

        {/* Divider */}
        <div style={{ 
          display: 'flex', 
          alignItems: 'center', 
          margin: '1.5rem 0',
          gap: '1rem'
        }}>
          <div style={{ flex: 1, height: '1px', background: 'rgba(232, 180, 79, 0.2)' }} />
          <Text style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>OR</Text>
          <div style={{ flex: 1, height: '1px', background: 'rgba(232, 180, 79, 0.2)' }} />
        </div>

        {/* Keycloak SSO Button */}
        <Button
          icon={<SafetyOutlined />}
          loading={keycloakLoading}
          onClick={handleKeycloakSSO}
          block
          size="large"
          style={{
            background: 'transparent',
            border: '2px solid #E8B44F',
            color: '#E8B44F',
            fontWeight: '600',
            height: '48px',
            marginBottom: '1.5rem'
          }}
        >
          Sign in with Keycloak SSO
        </Button>

        {/* Sign Up Link */}
        <div style={{ textAlign: 'center' }}>
          <Text style={{ color: '#9CA3AF' }}>
            Don't have an account?{' '}
            <a 
              onClick={onSwitchToSignup}
              style={{ 
                color: '#E8B44F', 
                fontWeight: '600',
                cursor: 'pointer',
                textDecoration: 'none'
              }}
            >
              Create Account
            </a>
          </Text>
        </div>

        {/* Info Box */}
        <div style={{
          marginTop: '2rem',
          padding: '1rem',
          background: 'rgba(59, 130, 246, 0.1)',
          border: '1px solid rgba(59, 130, 246, 0.3)',
          borderRadius: '8px'
        }}>
          <Text style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6' }}>
            <strong style={{ color: '#3B82F6' }}>Mojo Wallet Required:</strong> Login to access developer features, create blog posts, and activate your wallet for the Pravyom network.
          </Text>
        </div>
      </div>

      {/* Forgot Password Modal */}
      <Modal
        title={
          <span style={{ fontSize: '1.5rem', fontWeight: 'bold', color: '#E8B44F' }}>
            {forgotPasswordStep === 'email' && 'Reset Password'}
            {forgotPasswordStep === 'otp' && 'Verify OTP'}
            {forgotPasswordStep === 'reset' && 'New Password'}
            {forgotPasswordStep === 'success' && 'Success!'}
          </span>
        }
        open={forgotPasswordVisible}
        onCancel={closeForgotPasswordModal}
        footer={null}
        width={480}
      >
        {/* Step 1: Email */}
        {forgotPasswordStep === 'email' && (
          <div>
            <Text style={{ color: '#9CA3AF', display: 'block', marginBottom: '1.5rem' }}>
              Enter your email address and we'll send you an OTP to reset your password.
            </Text>
            <Form
              form={forgotPasswordForm}
              onFinish={handleForgotPasswordEmail}
              layout="vertical"
              size="large"
            >
              <Form.Item
                name="email"
                label={<span style={{ color: '#E8B44F', fontWeight: '600' }}>Email Address</span>}
                rules={[
                  { required: true, message: 'Please enter your email' },
                  { type: 'email', message: 'Please enter a valid email' }
                ]}
              >
                <Input
                  prefix={<MailOutlined style={{ color: '#9CA3AF' }} />}
                  placeholder="your.email@example.com"
                />
              </Form.Item>
              <Form.Item>
                <Button
                  type="primary"
                  htmlType="submit"
                  loading={forgotPasswordLoading}
                  block
                  style={{
                    background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)',
                    border: 'none',
                    color: '#0A1628',
                    fontWeight: '600',
                    height: '48px'
                  }}
                >
                  Send OTP
                </Button>
              </Form.Item>
            </Form>
          </div>
        )}

        {/* Step 2: OTP Verification */}
        {forgotPasswordStep === 'otp' && (
          <div>
            <Text style={{ color: '#9CA3AF', display: 'block', marginBottom: '1.5rem' }}>
              We've sent a 6-digit OTP to <strong style={{ color: '#E8B44F' }}>{forgotPasswordEmail}</strong>
            </Text>
            <Form
              form={forgotPasswordForm}
              onFinish={handleVerifyOtp}
              layout="vertical"
              size="large"
            >
              <Form.Item
                name="otp"
                label={<span style={{ color: '#E8B44F', fontWeight: '600' }}>Enter OTP</span>}
                rules={[
                  { required: true, message: 'Please enter the OTP' },
                  { len: 6, message: 'OTP must be 6 digits' }
                ]}
              >
                <Input
                  prefix={<SafetyOutlined style={{ color: '#9CA3AF' }} />}
                  placeholder="000000"
                  maxLength={6}
                />
              </Form.Item>
              <Form.Item>
                <Button
                  type="primary"
                  htmlType="submit"
                  loading={forgotPasswordLoading}
                  block
                  style={{
                    background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)',
                    border: 'none',
                    color: '#0A1628',
                    fontWeight: '600',
                    height: '48px'
                  }}
                >
                  Verify OTP
                </Button>
              </Form.Item>
              <div style={{ textAlign: 'center' }}>
                <a 
                  onClick={() => setForgotPasswordStep('email')}
                  style={{ color: '#E8B44F', cursor: 'pointer' }}
                >
                  Resend OTP
                </a>
              </div>
            </Form>
          </div>
        )}

        {/* Step 3: Reset Password */}
        {forgotPasswordStep === 'reset' && (
          <div>
            <Text style={{ color: '#9CA3AF', display: 'block', marginBottom: '1.5rem' }}>
              Enter your new password
            </Text>
            <Form
              form={forgotPasswordForm}
              onFinish={handleResetPassword}
              layout="vertical"
              size="large"
            >
              <Form.Item
                name="password"
                label={<span style={{ color: '#E8B44F', fontWeight: '600' }}>New Password</span>}
                rules={[
                  { required: true, message: 'Please enter a password' },
                  { min: 8, message: 'Password must be at least 8 characters' }
                ]}
                hasFeedback
              >
                <Input.Password
                  prefix={<LockOutlined style={{ color: '#9CA3AF' }} />}
                  placeholder="Minimum 8 characters"
                />
              </Form.Item>
              <Form.Item
                name="confirmPassword"
                label={<span style={{ color: '#E8B44F', fontWeight: '600' }}>Confirm Password</span>}
                dependencies={['password']}
                hasFeedback
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
                  prefix={<LockOutlined style={{ color: '#9CA3AF' }} />}
                  placeholder="Re-enter your password"
                />
              </Form.Item>
              <Form.Item>
                <Button
                  type="primary"
                  htmlType="submit"
                  loading={forgotPasswordLoading}
                  block
                  style={{
                    background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)',
                    border: 'none',
                    color: '#0A1628',
                    fontWeight: '600',
                    height: '48px'
                  }}
                >
                  Reset Password
                </Button>
              </Form.Item>
            </Form>
          </div>
        )}

        {/* Step 4: Success */}
        {forgotPasswordStep === 'success' && (
          <div style={{ textAlign: 'center', padding: '2rem 0' }}>
            <div style={{ fontSize: '4rem', marginBottom: '1rem' }}>✅</div>
            <Title level={3} style={{ color: '#10B981', marginBottom: '1rem' }}>
              Password Reset Successful!
            </Title>
            <Text style={{ color: '#9CA3AF', display: 'block', marginBottom: '2rem' }}>
              Your password has been reset successfully. You can now login with your new password.
            </Text>
            <Button
              type="primary"
              onClick={closeForgotPasswordModal}
              block
              style={{
                background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)',
                border: 'none',
                color: '#0A1628',
                fontWeight: '600',
                height: '48px'
              }}
            >
              Back to Login
            </Button>
          </div>
        )}
      </Modal>
    </div>
  );
};

export default Login;
