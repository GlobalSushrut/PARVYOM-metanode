import React, { useState } from 'react';
import { Card, Typography, Button, Form, Input, Alert, Space, Divider, message } from 'antd';
import {
  WalletOutlined,
  DashboardOutlined,
  KeyOutlined,
  CheckCircleOutlined,
  ThunderboltOutlined
} from '@ant-design/icons';
import axios from 'axios';

const { Title, Text } = Typography;

const MOJO_API_URL = 'http://localhost:8089/api/v1';

interface MojoWalletResponse {
  success: boolean;
  mojo_wallet_id: string;
  dashboard_url: string;
  access_token: string;
}

const MojoDashboard: React.FC = () => {
  const [loading, setLoading] = useState(false);
  const [form] = Form.useForm();
  const [mojoWallet, setMojoWallet] = useState<MojoWalletResponse | null>(null);

  // Create Mojo Wallet - Single API call (Real Backend)
  const handleCreateMojoWallet = async (values: any) => {
    setLoading(true);
    try {
      const response = await axios.post(`${MOJO_API_URL}/wallet`, {
        bpi_wallet_address: values.bpi_wallet_address,
        node_id: values.node_id || `node-${Date.now()}`
      });

      if (response.data && response.data.success) {
        setMojoWallet(response.data);
        message.success('Mojo Wallet created! Monitoring is now active.');
      }
    } catch (error: any) {
      message.error(error.response?.data?.message || 'Failed to create Mojo wallet. Make sure the Mojo server is running on port 8089.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={{ padding: '1.5rem', maxWidth: '800px', margin: '0 auto' }}>
      {/* Header */}
      <div style={{ marginBottom: '2rem' }}>
        <Title level={2} style={{ color: '#E8B44F', marginBottom: '0.5rem' }}>
          <ThunderboltOutlined /> Mojo Wallet - BPI Monitoring
        </Title>
        <Text style={{ color: '#9CA3AF' }}>
          Connect your BPI wallet to activate Grafana monitoring and Prometheus metrics
        </Text>
      </div>

      {!mojoWallet ? (
        // Form: Create Mojo Wallet
        <Card
          style={{
            background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
            border: '1px solid rgba(232, 180, 79, 0.2)',
            borderRadius: '12px'
          }}
        >
          <Title level={4} style={{ color: '#E8B44F', marginBottom: '1.5rem' }}>
            <WalletOutlined /> Create Mojo Wallet
          </Title>

          <Alert
            message="What is Mojo Wallet?"
            description="Mojo Wallet connects your BPI wallet address to monitoring services. It generates a Grafana dashboard and Prometheus metrics endpoint with token-based authentication."
            type="info"
            showIcon
            style={{ marginBottom: '1.5rem', background: 'rgba(59, 130, 246, 0.1)', border: '1px solid rgba(59, 130, 246, 0.3)' }}
          />

          <Form form={form} layout="vertical" onFinish={handleCreateMojoWallet}>
            <Form.Item
              label={<span style={{ color: '#E8B44F', fontWeight: '600' }}>BPI Wallet Address</span>}
              name="bpi_wallet_address"
              rules={[{ required: true, message: 'Please enter your BPI wallet address' }]}
            >
              <Input
                placeholder="0x1234...abcd"
                size="large"
                style={{
                  background: 'rgba(255, 255, 255, 0.05)',
                  border: '1px solid rgba(232, 180, 79, 0.2)',
                  color: '#ffffff',
                  fontFamily: 'monospace'
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

            <Form.Item>
              <Button
                type="primary"
                htmlType="submit"
                size="large"
                loading={loading}
                block
                icon={<ThunderboltOutlined />}
                style={{
                  background: 'linear-gradient(135deg, #E8B44F 0%, #D4A044 100%)',
                  border: 'none',
                  fontWeight: '600',
                  height: '48px'
                }}
              >
                Create Mojo Wallet
              </Button>
            </Form.Item>
          </Form>
        </Card>
      ) : (
        // Success: Show Mojo Wallet Details
        <Card
          style={{
            background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
            border: '1px solid rgba(232, 180, 79, 0.2)',
            borderRadius: '12px'
          }}
        >
          <div style={{ textAlign: 'center', marginBottom: '2rem' }}>
            <CheckCircleOutlined style={{ fontSize: '4rem', color: '#10B981', marginBottom: '1rem' }} />
            <Title level={3} style={{ color: '#E8B44F' }}>
              Mojo Wallet Created!
            </Title>
          </div>

          <Alert
            message="Monitoring is Now Active"
            description="Your BPI wallet is now connected to monitoring services. Use the access token to authenticate with Grafana."
            type="success"
            showIcon
            style={{ marginBottom: '1.5rem' }}
          />

          <Space direction="vertical" size="large" style={{ width: '100%' }}>
            <div>
              <Text style={{ color: '#9CA3AF', display: 'block', marginBottom: '0.5rem' }}>
                Mojo Wallet ID
              </Text>
              <Text style={{ color: '#E8B44F', fontFamily: 'monospace', fontSize: '1rem' }}>
                {mojoWallet.mojo_wallet_id}
              </Text>
            </div>

            <Divider style={{ borderColor: 'rgba(232, 180, 79, 0.2)' }} />

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
              <Text style={{ color: '#9CA3AF', fontSize: '0.875rem', marginTop: '0.5rem', display: 'block' }}>
                Use this token to authenticate with Grafana (NO password needed)
              </Text>
            </div>

            <Divider style={{ borderColor: 'rgba(232, 180, 79, 0.2)' }} />

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
              <Text style={{ color: '#9CA3AF', fontSize: '0.875rem', marginTop: '0.5rem', display: 'block' }}>
                Dashboard URL includes authentication token
              </Text>
            </div>

            <Divider style={{ borderColor: 'rgba(232, 180, 79, 0.2)' }} />

            <Alert
              message="What's Next?"
              description={
                <ul style={{ margin: 0, paddingLeft: '1.5rem' }}>
                  <li>Click the dashboard link above to access Grafana</li>
                  <li>The token is embedded in the URL for automatic authentication</li>
                  <li>Prometheus metrics are automatically collected</li>
                  <li>Monitor your BPI node performance in real-time</li>
                </ul>
              }
              type="info"
              showIcon
            />

            <Button
              type="primary"
              size="large"
              block
              icon={<DashboardOutlined />}
              href={mojoWallet.dashboard_url}
              target="_blank"
              style={{
                background: 'linear-gradient(135deg, #E8B44F 0%, #D4A044 100%)',
                border: 'none',
                fontWeight: '600',
                height: '48px'
              }}
            >
              Open Monitoring Dashboard
            </Button>
          </Space>
        </Card>
      )}
    </div>
  );
};

export default MojoDashboard;
