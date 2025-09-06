import React, { useState } from 'react';
import { Form, Input, Button, Card, Alert, Typography, Space } from 'antd';
import { UserOutlined, MailOutlined, LockOutlined, TeamOutlined, UserAddOutlined } from '@ant-design/icons';
import { apiService, type RegisterRequest } from '../../services/api';

const { Title, Text } = Typography;

interface SignupProps {
  onSignupSuccess: () => void;
  onSwitchToLogin: () => void;
}

const Signup: React.FC<SignupProps> = ({ onSignupSuccess, onSwitchToLogin }) => {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const onFinish = async (values: RegisterRequest) => {
    setLoading(true);
    setError(null);

    try {
      const response = await apiService.register(values);
      
      if (response.success) {
        onSignupSuccess();
      } else {
        setError(response.error || 'Registration failed. Please try again.');
      }
    } catch (error) {
      setError('Registration failed. Please check if the BPCI server is running.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <Card 
      style={{ maxWidth: 400, margin: '0 auto' }}
      title={
        <Space>
          <UserAddOutlined />
          <Title level={3} style={{ margin: 0 }}>Create Developer Profile</Title>
        </Space>
      }
    >
      <div style={{ marginBottom: 16, textAlign: 'center' }}>
        <Text type="secondary">
          Create your BPCI developer profile to access wallet activation and BPI features.
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
        name="developerSignup"
        onFinish={onFinish}
        layout="vertical"
        size="large"
      >
        <Form.Item
          label="Full Name"
          name="name"
          rules={[
            { required: true, message: 'Please input your full name!' },
            { min: 2, message: 'Name must be at least 2 characters!' }
          ]}
        >
          <Input 
            prefix={<UserOutlined />} 
            placeholder="Enter your full name"
          />
        </Form.Item>

        <Form.Item
          label="Email Address"
          name="email"
          rules={[
            { required: true, message: 'Please input your email!' },
            { type: 'email', message: 'Please enter a valid email address!' }
          ]}
        >
          <Input 
            prefix={<MailOutlined />} 
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
            placeholder="Create a secure password"
          />
        </Form.Item>

        <Form.Item
          label="Company (Optional)"
          name="company"
        >
          <Input 
            prefix={<TeamOutlined />} 
            placeholder="Enter your company name"
          />
        </Form.Item>

        <Form.Item
          label="Role (Optional)"
          name="role"
        >
          <Input 
            placeholder="e.g., Developer, CTO, Founder"
          />
        </Form.Item>

        <Form.Item>
          <Button 
            type="primary" 
            htmlType="submit" 
            loading={loading}
            block
            icon={<UserAddOutlined />}
          >
            Create Developer Profile
          </Button>
        </Form.Item>
      </Form>

      <div style={{ textAlign: 'center', marginTop: 16 }}>
        <Text>Already have a developer account? </Text>
        <Button type="link" onClick={onSwitchToLogin}>
          Login Here
        </Button>
      </div>
    </Card>
  );
};

export default Signup;
