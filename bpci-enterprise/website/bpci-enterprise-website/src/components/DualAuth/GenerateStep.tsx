import React, { useState } from 'react';
import { Form, Input, Button, Typography, Space, Alert, Card, Spin } from 'antd';
import { RocketOutlined, CopyOutlined, QrcodeOutlined } from '@ant-design/icons';
import { apiService } from '../../services/api';

const { Title, Text, Paragraph } = Typography;

interface GenerateStepProps {
  onComplete: (data: any) => void;
  onCancel: () => void;
}

const GenerateStep: React.FC<GenerateStepProps> = ({ onComplete, onCancel }) => {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [generated, setGenerated] = useState(false);
  const [connectionData, setConnectionData] = useState<any>(null);

  const handleGenerate = async (values: { name: string; description?: string }) => {
    setLoading(true);
    setError(null);

    try {
      const response = await apiService.generateBpiConnection(
        values.name,
        values.description
      );

      if (response.success && response.data) {
        setConnectionData(response.data);
        setGenerated(true);
      } else {
        setError(response.error || 'Failed to generate BPI connection');
      }
    } catch (error: any) {
      console.error('Generate error:', error);
      setError(error.message || 'Failed to generate BPI connection');
    } finally {
      setLoading(false);
    }
  };

  const handleContinue = () => {
    onComplete(connectionData);
  };

  const copyToClipboard = (text: string, label: string) => {
    navigator.clipboard.writeText(text);
    // You can add a toast notification here
  };

  if (loading) {
    return (
      <div style={{ textAlign: 'center', padding: '60px 0' }}>
        <Spin size="large" />
        <Paragraph style={{ marginTop: 16 }}>
          Generating your BPI connection...
        </Paragraph>
      </div>
    );
  }

  if (generated && connectionData) {
    return (
      <Space direction="vertical" size="large" style={{ width: '100%' }}>
        <Alert
          message="✅ BPI Connection Generated Successfully!"
          description="Your BPI address and authentication token have been created. Save these credentials securely."
          type="success"
          showIcon
        />

        <Card title="🎯 Your BPI Connection Details" style={{ background: '#f6ffed' }}>
          <Space direction="vertical" size="middle" style={{ width: '100%' }}>
            {/* Connection Name */}
            <div>
              <Text strong>Connection Name:</Text>
              <div style={{ marginTop: 4 }}>
                <Text copyable>{connectionData.name}</Text>
              </div>
            </div>

            {/* BPI Address */}
            <div>
              <Text strong>BPI Address:</Text>
              <div style={{ 
                marginTop: 4, 
                padding: '12px', 
                background: 'white', 
                borderRadius: '4px',
                border: '1px solid #d9d9d9',
                fontFamily: 'monospace',
                fontSize: '14px',
                wordBreak: 'break-all'
              }}>
                {connectionData.address}
                <Button 
                  type="link" 
                  icon={<CopyOutlined />}
                  onClick={() => copyToClipboard(connectionData.address, 'BPI Address')}
                  style={{ marginLeft: 8 }}
                >
                  Copy
                </Button>
              </div>
            </div>

            {/* Authentication Token */}
            <div>
              <Text strong>Authentication Token:</Text>
              <div style={{ 
                marginTop: 4, 
                padding: '12px', 
                background: 'white', 
                borderRadius: '4px',
                border: '1px solid #d9d9d9',
                fontFamily: 'monospace',
                fontSize: '12px',
                wordBreak: 'break-all'
              }}>
                {connectionData.token}
                <Button 
                  type="link" 
                  icon={<CopyOutlined />}
                  onClick={() => copyToClipboard(connectionData.token, 'Token')}
                  style={{ marginLeft: 8 }}
                >
                  Copy
                </Button>
              </div>
            </div>

            {/* QR Code - Placeholder for future implementation */}
            <div style={{ textAlign: 'center', marginTop: 16 }}>
              <Text type="secondary" style={{ fontSize: '12px' }}>
                💡 Tip: You can use these credentials with BPI mobile apps
              </Text>
            </div>
          </Space>
        </Card>

        <Alert
          message="⚠️ Important: Save Your Credentials"
          description={
            <ul style={{ marginBottom: 0, paddingLeft: 20 }}>
              <li>Copy and save your BPI address and token in a secure location</li>
              <li>You'll need these credentials to access advanced features</li>
              <li>Never share your authentication token with anyone</li>
              <li>The token cannot be recovered if lost</li>
            </ul>
          }
          type="warning"
          showIcon
        />

        <div style={{ display: 'flex', justifyContent: 'space-between', marginTop: 24 }}>
          <Button onClick={onCancel}>
            Cancel
          </Button>
          <Button 
            type="primary" 
            size="large"
            onClick={handleContinue}
          >
            Continue to Binding →
          </Button>
        </div>
      </Space>
    );
  }

  return (
    <Space direction="vertical" size="large" style={{ width: '100%' }}>
      <div>
        <Title level={4}>
          <RocketOutlined style={{ marginRight: 8, color: '#1890ff' }} />
          Step 1: Generate BPI Connection
        </Title>
        <Paragraph type="secondary">
          Create your BPI wallet address and authentication token. This will be your unique identifier 
          in the BPI network and will be linked to your Keycloak account.
        </Paragraph>
      </div>

      {error && (
        <Alert
          message="Generation Error"
          description={error}
          type="error"
          closable
          onClose={() => setError(null)}
          showIcon
        />
      )}

      <Card>
        <Form
          form={form}
          layout="vertical"
          onFinish={handleGenerate}
          autoComplete="off"
        >
          <Form.Item
            label="Connection Name"
            name="name"
            rules={[
              { required: true, message: 'Please enter a connection name' },
              { min: 3, message: 'Name must be at least 3 characters' },
              { max: 50, message: 'Name must be less than 50 characters' }
            ]}
            extra="A friendly name to identify this connection (e.g., 'My Main Wallet', 'Development Node')"
          >
            <Input 
              placeholder="e.g., My Main Wallet"
              size="large"
            />
          </Form.Item>

          <Form.Item
            label="Description (Optional)"
            name="description"
            extra="Additional details about this connection"
          >
            <Input.TextArea 
              placeholder="e.g., Primary wallet for development and testing"
              rows={3}
            />
          </Form.Item>

          <Form.Item style={{ marginBottom: 0 }}>
            <div style={{ display: 'flex', justifyContent: 'space-between' }}>
              <Button onClick={onCancel} size="large">
                Cancel
              </Button>
              <Button 
                type="primary" 
                htmlType="submit" 
                loading={loading}
                size="large"
                icon={<RocketOutlined />}
              >
                Generate BPI Connection
              </Button>
            </div>
          </Form.Item>
        </Form>
      </Card>

      <Alert
        message="What happens next?"
        description={
          <ul style={{ marginBottom: 0, paddingLeft: 20 }}>
            <li>A unique BPI wallet address will be generated for you</li>
            <li>An authentication token will be created for secure access</li>
            <li>These credentials will be linked to your Keycloak account</li>
            <li>You'll be able to manage wallets, deploy nodes, and access advanced features</li>
          </ul>
        }
        type="info"
        showIcon
      />
    </Space>
  );
};

export default GenerateStep;
