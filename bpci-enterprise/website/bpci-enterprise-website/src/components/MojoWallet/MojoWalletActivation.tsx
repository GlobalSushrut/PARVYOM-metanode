import React, { useState } from 'react';
import { Card, Form, Input, Button, Typography, Alert, Space, Divider, message, Tag } from 'antd';
import { 
  WalletOutlined, 
  DashboardOutlined, 
  KeyOutlined,
  CheckCircleOutlined,
  RocketOutlined
} from '@ant-design/icons';
import axios from 'axios';

const { Title, Text, Paragraph } = Typography;

const MOJO_API_URL = 'http://localhost:8089/api/v1';

interface MojoWalletResponse {
  success: boolean;
  mojo_wallet_id: string;
  dashboard_url: string;
  access_token: string;
}

export const MojoWalletActivation: React.FC = () => {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);
  const [activated, setActivated] = useState(false);
  const [mojoWallet, setMojoWallet] = useState<MojoWalletResponse | null>(null);

  const handleActivation = async (values: any) => {
    setLoading(true);
    
    try {
      const response = await axios.post<MojoWalletResponse>(`${MOJO_API_URL}/wallet`, {
        bpi_wallet_address: values.bpi_wallet_address,
        node_id: values.node_id || 'default-node'
      });

      if (response.data.success) {
        setMojoWallet(response.data);
        setActivated(true);
        message.success('Mojo Wallet activated successfully!');
      }
    } catch (error: any) {
      console.error('Failed to activate Mojo wallet:', error);
      message.error(error.response?.data?.message || 'Failed to activate Mojo wallet');
    } finally {
      setLoading(false);
    }
  };

  if (activated && mojoWallet) {
    return (
      <div style={{ padding: '1.5rem', maxWidth: '800px', margin: '0 auto' }}>
        {/* Success Header */}
        <div style={{ marginBottom: '2rem', textAlign: 'center' }}>
          <CheckCircleOutlined style={{ fontSize: '4rem', color: '#10B981', marginBottom: '1rem' }} />
          <Title level={2} style={{ color: '#E8B44F', marginBottom: '0.5rem' }}>
            Mojo Wallet Activated!
          </Title>
          <Text style={{ color: '#9CA3AF' }}>
            Your BPI node monitoring is now active
          </Text>
        </div>

        {/* Wallet Details */}
        <Card
          style={{
            background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
            border: '1px solid rgba(232, 180, 79, 0.2)',
            borderRadius: '12px',
            marginBottom: '1.5rem'
          }}
        >
          <Space direction="vertical" size="large" style={{ width: '100%' }}>
            <div>
              <Text style={{ color: '#9CA3AF', display: 'block', marginBottom: '0.5rem' }}>
                Mojo Wallet ID
              </Text>
              <Text style={{ color: '#E8B44F', fontFamily: 'monospace', fontSize: '1rem' }}>
                {mojoWallet.mojo_wallet_id}
              </Text>
            </div>

            <Divider style={{ borderColor: 'rgba(232, 180, 79, 0.2)', margin: '0.5rem 0' }} />

            <div>
              <Text style={{ color: '#9CA3AF', display: 'block', marginBottom: '0.5rem' }}>
                <KeyOutlined /> Access Token
              </Text>
              <Input.Password
                value={mojoWallet.access_token}
                readOnly
                style={{
                  background: 'rgba(255, 255, 255, 0.05)',
                  border: '1px solid rgba(232, 180, 79, 0.2)',
                  color: '#ffffff',
                  fontFamily: 'monospace'
                }}
              />
              <Text style={{ color: '#F59E0B', fontSize: '0.875rem', display: 'block', marginTop: '0.5rem' }}>
                ⚠️ Save this token securely - you'll need it to access your dashboard
              </Text>
            </div>

            <Divider style={{ borderColor: 'rgba(232, 180, 79, 0.2)', margin: '0.5rem 0' }} />

            <div>
              <Text style={{ color: '#9CA3AF', display: 'block', marginBottom: '0.5rem' }}>
                <DashboardOutlined /> Grafana Dashboard
              </Text>
              <a 
                href={mojoWallet.dashboard_url} 
                target="_blank" 
                rel="noopener noreferrer"
                style={{ color: '#3B82F6', wordBreak: 'break-all' }}
              >
                {mojoWallet.dashboard_url}
              </a>
            </div>
          </Space>
        </Card>

        {/* Instructions */}
        <Alert
          message="Next Steps"
          description={
            <ol style={{ margin: '0.5rem 0', paddingLeft: '1.5rem', color: '#9CA3AF' }}>
              <li>Copy and save your access token in a secure location</li>
              <li>Click the dashboard link to view your BPI node metrics</li>
              <li>Use the access token when prompted for authentication</li>
              <li>Monitor your node's performance, uptime, and health</li>
            </ol>
          }
          type="info"
          showIcon
          style={{ 
            background: 'rgba(59, 130, 246, 0.1)', 
            border: '1px solid rgba(59, 130, 246, 0.3)',
            marginBottom: '1.5rem'
          }}
        />

        {/* Action Buttons */}
        <div style={{ display: 'flex', gap: '1rem', justifyContent: 'center' }}>
          <Button
            type="primary"
            size="large"
            icon={<DashboardOutlined />}
            href={mojoWallet.dashboard_url}
            target="_blank"
            style={{
              background: 'linear-gradient(135deg, #E8B44F 0%, #D4A044 100%)',
              border: 'none',
              fontWeight: '600'
            }}
          >
            Open Dashboard
          </Button>
          <Button
            size="large"
            onClick={() => {
              setActivated(false);
              setMojoWallet(null);
              form.resetFields();
            }}
            style={{
              background: 'transparent',
              border: '1px solid rgba(232, 180, 79, 0.3)',
              color: '#9CA3AF'
            }}
          >
            Activate Another Wallet
          </Button>
        </div>
      </div>
    );
  }

  return (
    <div style={{ padding: '1.5rem', maxWidth: '800px', margin: '0 auto' }}>
      {/* Header */}
      <div style={{ marginBottom: '2rem' }}>
        <Title level={2} style={{ color: '#E8B44F', marginBottom: '0.5rem' }}>
          <WalletOutlined /> Activate Mojo Wallet
        </Title>
        <Text style={{ color: '#9CA3AF' }}>
          Enable monitoring for your BPI node on mainnet with Grafana dashboards
        </Text>
      </div>

      {/* Info Alert */}
      <Alert
        message="What is Mojo Wallet?"
        description={
          <div style={{ color: '#9CA3AF' }}>
            <Paragraph style={{ color: '#9CA3AF', marginBottom: '0.5rem' }}>
              Mojo Wallet provides isolated monitoring for each BPI node on mainnet:
            </Paragraph>
            <ul style={{ margin: '0.5rem 0', paddingLeft: '1.5rem' }}>
              <li>📊 Dedicated Grafana dashboard for your node</li>
              <li>📈 Prometheus metrics collection</li>
              <li>🔐 Token-based authentication (no password needed)</li>
              <li>⚡ Real-time performance monitoring</li>
            </ul>
          </div>
        }
        type="info"
        showIcon
        icon={<RocketOutlined />}
        style={{ 
          background: 'rgba(59, 130, 246, 0.1)', 
          border: '1px solid rgba(59, 130, 246, 0.3)',
          marginBottom: '2rem'
        }}
      />

      {/* Activation Form */}
      <Card
        style={{
          background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
          border: '1px solid rgba(232, 180, 79, 0.2)',
          borderRadius: '12px'
        }}
      >
        <Form
          form={form}
          layout="vertical"
          onFinish={handleActivation}
        >
          <Form.Item
            label={<span style={{ color: '#E8B44F', fontWeight: '600' }}>BPI Wallet Address *</span>}
            name="bpi_wallet_address"
            rules={[{ required: true, message: 'Please enter your BPI wallet address' }]}
          >
            <Input
              placeholder="0x1234567890abcdef..."
              size="large"
              prefix={<WalletOutlined style={{ color: '#9CA3AF' }} />}
              style={{
                background: 'rgba(255, 255, 255, 0.05)',
                border: '1px solid rgba(232, 180, 79, 0.2)',
                color: '#ffffff'
              }}
            />
          </Form.Item>

          <Form.Item
            label={<span style={{ color: '#E8B44F', fontWeight: '600' }}>Node ID (Optional)</span>}
            name="node_id"
          >
            <Input
              placeholder="my-bpi-node"
              size="large"
              style={{
                background: 'rgba(255, 255, 255, 0.05)',
                border: '1px solid rgba(232, 180, 79, 0.2)',
                color: '#ffffff'
              }}
            />
          </Form.Item>

          <Form.Item style={{ marginBottom: 0 }}>
            <Button
              type="primary"
              htmlType="submit"
              size="large"
              loading={loading}
              block
              icon={<RocketOutlined />}
              style={{
                background: 'linear-gradient(135deg, #E8B44F 0%, #D4A044 100%)',
                border: 'none',
                fontWeight: '600',
                height: '48px'
              }}
            >
              Activate Mojo Wallet
            </Button>
          </Form.Item>
        </Form>
      </Card>

      {/* Requirements */}
      <Alert
        message="Requirements"
        description={
          <ul style={{ margin: '0.5rem 0', paddingLeft: '1.5rem', color: '#9CA3AF' }}>
            <li>Active BPI wallet address</li>
            <li>Running BPI node (for mainnet monitoring)</li>
            <li>Mojo server running on port 8089</li>
          </ul>
        }
        type="warning"
        showIcon
        style={{ 
          background: 'rgba(245, 158, 11, 0.1)', 
          border: '1px solid rgba(245, 158, 11, 0.3)',
          marginTop: '1.5rem'
        }}
      />
    </div>
  );
};

export default MojoWalletActivation;
