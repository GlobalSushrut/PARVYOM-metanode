import React, { useState, useEffect } from 'react';
import { Card, Row, Col, Statistic, Button, Typography, Space, Tabs, Alert, message, Tag } from 'antd';
import {
  WalletOutlined,
  DollarOutlined,
  SwapOutlined,
  HistoryOutlined,
  SendOutlined,
  DownloadOutlined,
  SafetyOutlined,
  ReloadOutlined,
  CheckCircleOutlined,
  BlockOutlined
} from '@ant-design/icons';
import { WalletManager } from '../components/Wallet/WalletManager';
import axios from 'axios';

const { Title, Text } = Typography;

const API_BASE_URL = 'http://127.0.0.1:8081/api';

// Simple wallet status from backend (testnet BPI only)
interface WalletStatus {
  wallet_id: string;
  name: string;
  type: string;
  address: string;
  status: string;
  balance: string; // Format: "100.0 BPCI"
  last_activity: string;
  transaction_count: number;
  verification_level: string;
  blockchain_connected: boolean;
  current_block: number;
  network: string;
}

const Wallet: React.FC = () => {
  const [walletStatus, setWalletStatus] = useState<WalletStatus | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchWalletData();
  }, []);

  const fetchWalletData = async () => {
    try {
      setLoading(true);
      
      // Fetch real wallet status from backend (simple BPI balance)
      const response = await axios.get(`${API_BASE_URL}/wallet/status?wallet_id=default`);
      if (response.data.status === 'ok' && response.data.data) {
        setWalletStatus(response.data.data);
      }
    } catch (error) {
      console.error('Failed to fetch wallet data:', error);
      message.error('Failed to load wallet data from blockchain');
    } finally {
      setLoading(false);
    }
  };

  // Parse BPI balance from string like "100.0 BPCI"
  const getBpiBalance = (): number => {
    if (!walletStatus?.balance) return 0;
    const match = walletStatus.balance.match(/(\d+\.?\d*)/);
    return match ? parseFloat(match[1]) : 0;
  };

  return (
    <div style={{ padding: '1.5rem' }}>
      {/* Header */}
      <div style={{ marginBottom: '2rem' }}>
        <Title level={2} style={{ color: '#E8B44F', marginBottom: '0.5rem' }}>
          <WalletOutlined /> My Wallet
        </Title>
        <Text style={{ color: '#9CA3AF' }}>
          Manage your BPCI wallets, view balances, and perform transactions
        </Text>
      </div>

      {/* Quick Stats */}
      <Row gutter={[16, 16]} style={{ marginBottom: '2rem' }}>
        <Col xs={24} sm={12} lg={8}>
          <Card
            style={{
              background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
              border: '1px solid rgba(232, 180, 79, 0.2)',
              borderRadius: '12px'
            }}
          >
            <Statistic
              title={<Text style={{ color: '#9CA3AF' }}>BPI Balance (Testnet)</Text>}
              value={getBpiBalance()}
              precision={1}
              prefix={<WalletOutlined style={{ color: '#E8B44F' }} />}
              suffix="BPI"
              valueStyle={{ color: '#E8B44F', fontWeight: 'bold' }}
              loading={loading}
            />
          </Card>
        </Col>

        <Col xs={24} sm={12} lg={8}>
          <Card
            style={{
              background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
              border: '1px solid rgba(232, 180, 79, 0.2)',
              borderRadius: '12px'
            }}
          >
            <Statistic
              title={<Text style={{ color: '#9CA3AF' }}>Transactions</Text>}
              value={walletStatus?.transaction_count || 0}
              prefix={<SwapOutlined style={{ color: '#3B82F6' }} />}
              valueStyle={{ color: '#3B82F6', fontWeight: 'bold' }}
              loading={loading}
            />
          </Card>
        </Col>

        <Col xs={24} sm={12} lg={8}>
          <Card
            style={{
              background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
              border: '1px solid rgba(232, 180, 79, 0.2)',
              borderRadius: '12px'
            }}
          >
            <Statistic
              title={<Text style={{ color: '#9CA3AF' }}>Current Block</Text>}
              value={walletStatus?.current_block || 0}
              prefix={<BlockOutlined style={{ color: '#10B981' }} />}
              valueStyle={{ color: '#10B981', fontWeight: 'bold' }}
              loading={loading}
            />
          </Card>
        </Col>
      </Row>

      {/* Wallet Info Card */}
      {walletStatus && (
        <Card
          style={{
            background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
            border: '1px solid rgba(232, 180, 79, 0.2)',
            borderRadius: '12px',
            marginBottom: '2rem'
          }}
        >
          <Row gutter={[16, 16]}>
            <Col xs={24} md={12}>
              <Space direction="vertical" size="small">
                <Text style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>Wallet Address</Text>
                <Text style={{ color: '#E8B44F', fontFamily: 'monospace', fontSize: '0.875rem' }}>
                  {walletStatus.address}
                </Text>
              </Space>
            </Col>
            <Col xs={24} md={6}>
              <Space direction="vertical" size="small">
                <Text style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>Status</Text>
                <Tag color={walletStatus.blockchain_connected ? 'success' : 'error'} icon={<CheckCircleOutlined />}>
                  {walletStatus.status}
                </Tag>
              </Space>
            </Col>
            <Col xs={24} md={6}>
              <Space direction="vertical" size="small">
                <Text style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>Network</Text>
                <Tag color="blue">{walletStatus.network}</Tag>
              </Space>
            </Col>
          </Row>
        </Card>
      )}

      {/* Quick Actions */}
      <Card
        style={{
          background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
          border: '1px solid rgba(232, 180, 79, 0.2)',
          borderRadius: '12px',
          marginBottom: '2rem'
        }}
      >
        <Space size="middle" wrap>
          <Button
            type="primary"
            icon={<SendOutlined />}
            size="large"
            style={{
              background: 'linear-gradient(135deg, #E8B44F 0%, #D4A044 100%)',
              border: 'none',
              borderRadius: '8px',
              fontWeight: '600'
            }}
          >
            Send
          </Button>
          <Button
            icon={<DownloadOutlined />}
            size="large"
            style={{
              background: 'transparent',
              border: '1px solid #E8B44F',
              color: '#E8B44F',
              borderRadius: '8px',
              fontWeight: '600'
            }}
          >
            Receive
          </Button>
          <Button
            icon={<SwapOutlined />}
            size="large"
            style={{
              background: 'transparent',
              border: '1px solid #E8B44F',
              color: '#E8B44F',
              borderRadius: '8px',
              fontWeight: '600'
            }}
          >
            Swap
          </Button>
          <Button
            icon={<HistoryOutlined />}
            size="large"
            style={{
              background: 'transparent',
              border: '1px solid #E8B44F',
              color: '#E8B44F',
              borderRadius: '8px',
              fontWeight: '600'
            }}
          >
            History
          </Button>
        </Space>
      </Card>

      {/* Alert for Mojo Wallet Activation */}
      <Alert
        message="Activate Your Mojo Wallet"
        description="Unlock full access to the Pravyom network by activating your Mojo wallet. Receive Mother Coin allocation and mine Baby Coins with Proof-of-Existence."
        type="info"
        showIcon
        icon={<WalletOutlined />}
        action={
          <Button
            size="small"
            type="primary"
            style={{
              background: 'linear-gradient(135deg, #E8B44F 0%, #D4A044 100%)',
              border: 'none'
            }}
          >
            Activate Now
          </Button>
        }
        style={{
          background: 'rgba(232, 180, 79, 0.1)',
          border: '1px solid rgba(232, 180, 79, 0.3)',
          borderRadius: '8px',
          marginBottom: '2rem'
        }}
      />

      {/* Wallet Manager Tabs */}
      <Card
        style={{
          background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
          border: '1px solid rgba(232, 180, 79, 0.2)',
          borderRadius: '12px'
        }}
      >
        <Tabs
          defaultActiveKey="1"
          style={{ color: '#ffffff' }}
          items={[
            {
              key: '1',
              label: (
                <span style={{ color: '#E8B44F', fontWeight: '600' }}>
                  <WalletOutlined /> My Wallets
                </span>
              ),
              children: <WalletManager />
            },
            {
              key: '2',
              label: (
                <span style={{ color: '#9CA3AF', fontWeight: '600' }}>
                  <HistoryOutlined /> Transaction History
                </span>
              ),
              children: (
                <div style={{ padding: '2rem', textAlign: 'center' }}>
                  <Text style={{ color: '#9CA3AF' }}>
                    Transaction history will be displayed here
                  </Text>
                </div>
              )
            },
            {
              key: '3',
              label: (
                <span style={{ color: '#9CA3AF', fontWeight: '600' }}>
                  <SafetyOutlined /> Security
                </span>
              ),
              children: (
                <div style={{ padding: '2rem', textAlign: 'center' }}>
                  <Text style={{ color: '#9CA3AF' }}>
                    Security settings will be displayed here
                  </Text>
                </div>
              )
            }
          ]}
        />
      </Card>
    </div>
  );
};

export default Wallet;
