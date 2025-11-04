import React, { useState } from 'react';
import { Form, Input, Button, Alert, Typography, Select, Checkbox, message } from 'antd';
import { UserOutlined, MailOutlined, LockOutlined, TeamOutlined, WalletOutlined } from '@ant-design/icons';
import { authService } from '../../services/authService';

const { Title, Text } = Typography;
const { Option } = Select;

interface SignupProps {
  onSignupSuccess: () => void;
  onSwitchToLogin: () => void;
}

const Signup: React.FC<SignupProps> = ({ onSignupSuccess, onSwitchToLogin }) => {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [form] = Form.useForm();

  const onFinish = async (values: any) => {
    setLoading(true);
    setError(null);

    try {
      const response = await authService.signup({
        name: values.name,
        email: values.email,
        password: values.password,
        company: values.company,
        role: values.role,
      });

      if (response.success) {
        message.success('Account created successfully! Please check your email for verification.');
        onSignupSuccess();
      } else {
        setError(response.message || 'Signup failed. Please try again.');
      }
    } catch (error: any) {
      console.error('Signup error:', error);
      setError(error.message || 'Signup failed. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={{ 
      minHeight: '100vh', 
      background: 'linear-gradient(135deg, #0A1628 0%, #1e293b 100%)',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
      padding: '2rem'
    }}>
      <div style={{
        maxWidth: '520px',
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
            Create Your Account
          </Title>
          <Text style={{ color: '#9CA3AF', fontSize: '1rem' }}>
            Join the Pravyom network and activate your Mojo wallet
          </Text>
        </div>

        {/* Error Alert */}
        {error && (
          <Alert
            message="Signup Failed"
            description={error}
            type="error"
            showIcon
            closable
            onClose={() => setError(null)}
            style={{ marginBottom: '1.5rem' }}
          />
        )}

        {/* Signup Form */}
        <Form
          form={form}
          name="signup"
          onFinish={onFinish}
          layout="vertical"
          size="large"
          requiredMark={false}
        >
          <Form.Item
            name="name"
            label={<span style={{ color: '#E8B44F', fontWeight: '600' }}>Full Name</span>}
            rules={[{ required: true, message: 'Please enter your full name' }]}
          >
            <Input
              prefix={<UserOutlined style={{ color: '#9CA3AF' }} />}
              placeholder="John Doe"
              style={{
                background: 'rgba(255, 255, 255, 0.05)',
                border: '1px solid rgba(232, 180, 79, 0.2)',
                color: '#ffffff',
                height: '48px'
              }}
            />
          </Form.Item>

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
            rules={[
              { required: true, message: 'Please enter a password' },
              { min: 8, message: 'Password must be at least 8 characters' }
            ]}
            hasFeedback
          >
            <Input.Password
              prefix={<LockOutlined style={{ color: '#9CA3AF' }} />}
              placeholder="Minimum 8 characters"
              style={{
                background: 'rgba(255, 255, 255, 0.05)',
                border: '1px solid rgba(232, 180, 79, 0.2)',
                color: '#ffffff',
                height: '48px'
              }}
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
              style={{
                background: 'rgba(255, 255, 255, 0.05)',
                border: '1px solid rgba(232, 180, 79, 0.2)',
                color: '#ffffff',
                height: '48px'
              }}
            />
          </Form.Item>

          <Form.Item
            name="company"
            label={<span style={{ color: '#E8B44F', fontWeight: '600' }}>Company (Optional)</span>}
          >
            <Input
              prefix={<TeamOutlined style={{ color: '#9CA3AF' }} />}
              placeholder="Your company name"
              style={{
                background: 'rgba(255, 255, 255, 0.05)',
                border: '1px solid rgba(232, 180, 79, 0.2)',
                color: '#ffffff',
                height: '48px'
              }}
            />
          </Form.Item>

          <Form.Item
            name="role"
            label={<span style={{ color: '#E8B44F', fontWeight: '600' }}>Role</span>}
            rules={[{ required: true, message: 'Please select your role' }]}
          >
            <Select
              placeholder="Select your role"
              style={{ height: '48px' }}
            >
              <Option value="developer">Developer</Option>
              <Option value="researcher">Researcher</Option>
              <Option value="enterprise">Enterprise User</Option>
              <Option value="investor">Investor</Option>
              <Option value="other">Other</Option>
            </Select>
          </Form.Item>

          <Form.Item
            name="terms"
            valuePropName="checked"
            rules={[
              {
                validator: (_, value) =>
                  value ? Promise.resolve() : Promise.reject(new Error('Please accept the terms')),
              },
            ]}
          >
            <Checkbox style={{ color: '#ffffff' }}>
              <span style={{ color: '#ffffff' }}>
                I agree to the{' '}
                <a href="/terms-of-service" style={{ color: '#E8B44F' }}>Terms of Service</a>
                {' '}and{' '}
                <a href="/privacy-policy" style={{ color: '#E8B44F' }}>Privacy Policy</a>
              </span>
            </Checkbox>
          </Form.Item>

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
              Create Account
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

        {/* Login Link */}
        <div style={{ textAlign: 'center' }}>
          <Text style={{ color: '#9CA3AF' }}>
            Already have an account?{' '}
            <a 
              onClick={onSwitchToLogin}
              style={{ 
                color: '#E8B44F', 
                fontWeight: '600',
                cursor: 'pointer',
                textDecoration: 'none'
              }}
            >
              Sign In
            </a>
          </Text>
        </div>

        {/* Info Box */}
        <div style={{
          marginTop: '2rem',
          padding: '1rem',
          background: 'rgba(16, 185, 129, 0.1)',
          border: '1px solid rgba(16, 185, 129, 0.3)',
          borderRadius: '8px'
        }}>
          <Text style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6' }}>
            <strong style={{ color: '#10B981' }}>What you get:</strong> Access to developer features, Mojo wallet activation, blog posting, and full access to the Pravyom network infrastructure.
          </Text>
        </div>
      </div>
    </div>
  );
};

export default Signup;
