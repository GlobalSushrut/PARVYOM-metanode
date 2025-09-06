import React, { useState } from 'react';
import { Form, Input, Button, Card, Alert, Typography, Space } from 'antd';
import { UserOutlined, LockOutlined, LoginOutlined } from '@ant-design/icons';
import { apiService, type LoginRequest } from '../../services/api';

const { Title, Text } = Typography;

interface LoginProps {
  onLoginSuccess: () => void;
  onSwitchToSignup: () => void;
}

const Login: React.FC<LoginProps> = ({ onLoginSuccess, onSwitchToSignup }) => {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const onFinish = async (values: LoginRequest) => {
    setLoading(true);
    setError(null);

    try {
      const response = await apiService.login(values);
      
      if (response.success) {
        onLoginSuccess();
      } else {
        setError(response.error || 'Login failed. Please try again.');
      }
    } catch (error) {
      setError('Login failed. Please check if the BPCI server is running.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <Card 
      style={{ maxWidth: 400, margin: '0 auto' }}
      title={
        <Space>
          <LoginOutlined />
          <Title level={3} style={{ margin: 0 }}>Developer Login</Title>
        </Space>
      }
    >
      <div style={{ marginBottom: 16, textAlign: 'center' }}>
        <Text type="secondary">
          Login to your BPCI developer account to access wallet activation and BPI features.
        </Text>
      </div>

      {error && (
        <Alert
          message={error}
          type="error"
          style={{ marginBottom: 16 }}
          closable
          onClose={() => setError(null)}
        />
      )}

      <Form
        name="developerLogin"
        onFinish={onFinish}
        layout="vertical"
        size="large"
      >
        <Form.Item
          label="Email Address"
          name="email"
          rules={[
            { required: true, message: 'Please input your email!' },
            { type: 'email', message: 'Please enter a valid email address!' }
          ]}
        >
          <Input 
            prefix={<UserOutlined />} 
            placeholder="Enter your email address"
            type="email"
          />
        </Form.Item>

        <Form.Item
          label="Password"
          name="password"
          rules={[
            { required: true, message: 'Please input your password!' },
            { min: 8, message: 'Password must be at least 8 characters!' }
          ]}
        >
          <Input.Password 
            prefix={<LockOutlined />} 
            placeholder="Enter your password"
          />
        </Form.Item>

        <Form.Item>
          <Button 
            type="primary" 
            htmlType="submit" 
            loading={loading}
            block
            icon={<LoginOutlined />}
          >
            Login to Developer Account
          </Button>
        </Form.Item>
      </Form>

      <div style={{ textAlign: 'center', marginTop: 16 }}>
        <Text>Don't have a developer account? </Text>
        <Button type="link" onClick={onSwitchToSignup}>
          Create Developer Profile
        </Button>
      </div>
    </Card>
  );
};

export default Login;
