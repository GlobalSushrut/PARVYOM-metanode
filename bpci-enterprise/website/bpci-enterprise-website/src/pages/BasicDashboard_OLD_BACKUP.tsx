import React, { useState, useEffect } from 'react';
import { Card, Button, Typography, Space, Alert, Row, Col, Statistic, Empty, Spin } from 'antd';
import { 
  WalletOutlined, 
  RocketOutlined, 
  SafetyOutlined, 
  ApiOutlined,
  CheckCircleOutlined,
  ClockCircleOutlined,
  InfoCircleOutlined
} from '@ant-design/icons';
import { useNavigate } from 'react-router-dom';
import { apiService } from '../services/api';

const { Title, Text, Paragraph } = Typography;

interface BasicDashboardProps {
  onStartDualAuth: () => void;
}

const BasicDashboard: React.FC<BasicDashboardProps> = ({ onStartDualAuth }) => {
  const navigate = useNavigate();
  const [loading, setLoading] = useState(true);
  const [user, setUser] = useState<any>(null);
  const [hasBpiWallet, setHasBpiWallet] = useState(false);

  useEffect(() => {
    loadUserData();
  }, []);

  const loadUserData = async () => {
    try {
      const currentUser = await apiService.getCurrentUser();
      setUser(currentUser);

      // Check if user has BPI wallet
      const wallets = await apiService.listBpiWallets();
      setHasBpiWallet(!!(wallets.success && wallets.data && wallets.data.length > 0));
    } catch (error) {
      console.error('Failed to load user data:', error);
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return (
      <div style={{ textAlign: 'center', padding: '100px 0' }}>
        <Spin size="large" />
        <Paragraph style={{ marginTop: 16 }}>Loading your dashboard...</Paragraph>
      </div>
    );
  }

  return (
    <div style={{ padding: '24px', maxWidth: '1200px', margin: '0 auto' }}>
      {/* Welcome Header */}
      <Card style={{ marginBottom: 24, background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)', border: 'none' }}>
        <Space direction="vertical" size="small" style={{ width: '100%' }}>
          <Title level={2} style={{ color: 'white', margin: 0 }}>
            Welcome to BPCI Enterprise! 👋
          </Title>
          <Text style={{ color: 'rgba(255,255,255,0.9)', fontSize: '16px' }}>
            {user?.email || 'Developer'}
          </Text>
          <Paragraph style={{ color: 'rgba(255,255,255,0.8)', marginBottom: 0 }}>
            You're successfully authenticated with Keycloak. Complete the dual-auth setup to unlock full features.
          </Paragraph>
        </Space>
      </Card>

      {/* Authentication Status */}
      <Row gutter={[16, 16]} style={{ marginBottom: 24 }}>
        <Col xs={24} sm={12} md={8}>
          <Card>
            <Statistic
              title="Keycloak Authentication"
              value="Active"
              prefix={<CheckCircleOutlined style={{ color: '#52c41a' }} />}
              valueStyle={{ color: '#52c41a', fontSize: '18px' }}
            />
            <Text type="secondary" style={{ fontSize: '12px' }}>
              Level 1 authentication complete
            </Text>
          </Card>
        </Col>
        <Col xs={24} sm={12} md={8}>
          <Card>
            <Statistic
              title="BPI Wallet"
              value={hasBpiWallet ? "Connected" : "Not Connected"}
              prefix={hasBpiWallet ? <CheckCircleOutlined style={{ color: '#52c41a' }} /> : <ClockCircleOutlined style={{ color: '#faad14' }} />}
              valueStyle={{ color: hasBpiWallet ? '#52c41a' : '#faad14', fontSize: '18px' }}
            />
            <Text type="secondary" style={{ fontSize: '12px' }}>
              {hasBpiWallet ? 'Dual-auth active' : 'Dual-auth required'}
            </Text>
          </Card>
        </Col>
        <Col xs={24} sm={12} md={8}>
          <Card>
            <Statistic
              title="Access Level"
              value={hasBpiWallet ? "Full" : "Basic"}
              prefix={<SafetyOutlined style={{ color: hasBpiWallet ? '#52c41a' : '#1890ff' }} />}
              valueStyle={{ color: hasBpiWallet ? '#52c41a' : '#1890ff', fontSize: '18px' }}
            />
            <Text type="secondary" style={{ fontSize: '12px' }}>
              {hasBpiWallet ? 'All features unlocked' : 'Limited features'}
            </Text>
          </Card>
        </Col>
      </Row>

      {/* Dual-Auth Activation Prompt */}
      {!hasBpiWallet && (
        <Alert
          message="🔐 Activate Dual-Authentication for Full Access"
          description={
            <div>
              <Paragraph style={{ marginBottom: 8 }}>
                You're currently using <strong>Level 1 authentication</strong> (Keycloak only). 
                To unlock advanced features like wallet management, node deployment, and monitoring, 
                you need to activate <strong>dual-authentication</strong> by generating a BPI connection.
              </Paragraph>
              <Paragraph style={{ marginBottom: 16 }}>
                <strong>What you'll get:</strong>
              </Paragraph>
              <ul style={{ marginBottom: 16 }}>
                <li>🎯 BPI wallet address and authentication token</li>
                <li>🚀 Ability to deploy and manage BPI OS nodes</li>
                <li>📊 Access to advanced monitoring and analytics</li>
                <li>💼 Full wallet management capabilities</li>
                <li>🔒 Enhanced security with dual-layer authentication</li>
              </ul>
              <Button 
                type="primary" 
                size="large" 
                icon={<RocketOutlined />}
                onClick={onStartDualAuth}
                style={{ marginTop: 8 }}
              >
                Generate BPI Connection (3-Step Wizard)
              </Button>
            </div>
          }
          type="info"
          showIcon
          icon={<InfoCircleOutlined />}
          style={{ marginBottom: 24 }}
        />
      )}

      {/* Available Features */}
      <Title level={4} style={{ marginBottom: 16 }}>
        {hasBpiWallet ? '🎉 All Features Available' : '📋 Current Features (Basic Access)'}
      </Title>

      <Row gutter={[16, 16]}>
        {/* Keycloak Features (Always Available) */}
        <Col xs={24} sm={12} md={8}>
          <Card 
            hoverable
            style={{ height: '100%' }}
          >
            <Space direction="vertical" size="middle" style={{ width: '100%' }}>
              <SafetyOutlined style={{ fontSize: '32px', color: '#52c41a' }} />
              <Title level={5}>Keycloak SSO</Title>
              <Paragraph type="secondary">
                Enterprise-grade authentication with OAuth2, SAML, and OpenID Connect support.
              </Paragraph>
              <Text strong style={{ color: '#52c41a' }}>✓ Active</Text>
            </Space>
          </Card>
        </Col>

        <Col xs={24} sm={12} md={8}>
          <Card 
            hoverable
            style={{ height: '100%' }}
          >
            <Space direction="vertical" size="middle" style={{ width: '100%' }}>
              <ApiOutlined style={{ fontSize: '32px', color: '#52c41a' }} />
              <Title level={5}>API Documentation</Title>
              <Paragraph type="secondary">
                Browse API documentation and explore available endpoints.
              </Paragraph>
              <Button type="link" onClick={() => navigate('/api-docs')}>
                View Docs →
              </Button>
            </Space>
          </Card>
        </Col>

        <Col xs={24} sm={12} md={8}>
          <Card 
            hoverable
            style={{ height: '100%' }}
          >
            <Space direction="vertical" size="middle" style={{ width: '100%' }}>
              <InfoCircleOutlined style={{ fontSize: '32px', color: '#52c41a' }} />
              <Title level={5}>Community Forum</Title>
              <Paragraph type="secondary">
                Read community posts and discussions about BPI OS.
              </Paragraph>
              <Button type="link" onClick={() => navigate('/community')}>
                Browse Forum →
              </Button>
            </Space>
          </Card>
        </Col>

        {/* BPI Wallet Features (Requires Dual-Auth) */}
        <Col xs={24} sm={12} md={8}>
          <Card 
            hoverable={hasBpiWallet}
            style={{ 
              height: '100%',
              opacity: hasBpiWallet ? 1 : 0.6,
              cursor: hasBpiWallet ? 'pointer' : 'not-allowed'
            }}
          >
            <Space direction="vertical" size="middle" style={{ width: '100%' }}>
              <WalletOutlined style={{ fontSize: '32px', color: hasBpiWallet ? '#1890ff' : '#d9d9d9' }} />
              <Title level={5}>Mojo Wallet</Title>
              <Paragraph type="secondary">
                Manage your BPI wallet, view balance, and send transactions.
              </Paragraph>
              {hasBpiWallet ? (
                <Button type="primary" onClick={() => navigate('/wallet')}>
                  Open Wallet →
                </Button>
              ) : (
                <Text type="secondary">🔒 Requires dual-auth</Text>
              )}
            </Space>
          </Card>
        </Col>

        <Col xs={24} sm={12} md={8}>
          <Card 
            hoverable={hasBpiWallet}
            style={{ 
              height: '100%',
              opacity: hasBpiWallet ? 1 : 0.6,
              cursor: hasBpiWallet ? 'pointer' : 'not-allowed'
            }}
          >
            <Space direction="vertical" size="middle" style={{ width: '100%' }}>
              <RocketOutlined style={{ fontSize: '32px', color: hasBpiWallet ? '#1890ff' : '#d9d9d9' }} />
              <Title level={5}>Node Deployment</Title>
              <Paragraph type="secondary">
                Deploy and manage BPI Immutable OS nodes with BSO-K8.
              </Paragraph>
              {hasBpiWallet ? (
                <Button type="primary" onClick={() => navigate('/deploy-node')}>
                  Deploy Node →
                </Button>
              ) : (
                <Text type="secondary">🔒 Requires dual-auth</Text>
              )}
            </Space>
          </Card>
        </Col>

        <Col xs={24} sm={12} md={8}>
          <Card 
            hoverable={hasBpiWallet}
            style={{ 
              height: '100%',
              opacity: hasBpiWallet ? 1 : 0.6,
              cursor: hasBpiWallet ? 'pointer' : 'not-allowed'
            }}
          >
            <Space direction="vertical" size="middle" style={{ width: '100%' }}>
              <SafetyOutlined style={{ fontSize: '32px', color: hasBpiWallet ? '#1890ff' : '#d9d9d9' }} />
              <Title level={5}>Advanced Monitoring</Title>
              <Paragraph type="secondary">
                Real-time metrics, analytics, and node health monitoring.
              </Paragraph>
              {hasBpiWallet ? (
                <Button type="primary" onClick={() => navigate('/monitoring')}>
                  View Metrics →
                </Button>
              ) : (
                <Text type="secondary">🔒 Requires dual-auth</Text>
              )}
            </Space>
          </Card>
        </Col>
      </Row>

      {/* Help Section */}
      <Card style={{ marginTop: 24, background: '#f0f2f5', border: 'none' }}>
        <Space direction="vertical" size="small" style={{ width: '100%' }}>
          <Title level={5}>Need Help?</Title>
          <Paragraph type="secondary" style={{ marginBottom: 8 }}>
            Check out our documentation or join the community forum for support.
          </Paragraph>
          <Space>
            <Button onClick={() => navigate('/docs')}>
              📖 Documentation
            </Button>
            <Button onClick={() => navigate('/community')}>
              💬 Community Forum
            </Button>
            <Button onClick={() => navigate('/support')}>
              🆘 Get Support
            </Button>
          </Space>
        </Space>
      </Card>
    </div>
  );
};

export default BasicDashboard;
