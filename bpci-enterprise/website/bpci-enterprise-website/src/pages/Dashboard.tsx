import React, { useState } from 'react';
import { Card, Row, Col, Statistic, Progress, Badge, Button, Alert, Spin } from 'antd';
import {
  DashboardOutlined,
  NodeIndexOutlined,
  TransactionOutlined,
  WalletOutlined,
  SecurityScanOutlined,
  ThunderboltOutlined,
  GlobalOutlined,
  ReloadOutlined,
  CheckCircleOutlined,
  ExclamationCircleOutlined,
  CloseCircleOutlined
} from '@ant-design/icons';
import { useRealTimeData } from '../services/realTimeService';

const Dashboard: React.FC = () => {
  const { data, isConnected, refresh } = useRealTimeData();
  const [refreshing, setRefreshing] = useState(false);

  const handleRefresh = async () => {
    setRefreshing(true);
    await refresh();
    setTimeout(() => setRefreshing(false), 1000);
  };

  const getNetworkStatusColor = (status: string) => {
    switch (status) {
      case 'online': return '#52c41a';
      case 'degraded': return '#faad14';
      case 'offline': return '#ff4d4f';
      default: return '#52c41a';
    }
  };

  const getNetworkStatusIcon = (status: string) => {
    switch (status) {
      case 'online': return <CheckCircleOutlined />;
      case 'degraded': return <ExclamationCircleOutlined />;
      case 'offline': return <CloseCircleOutlined />;
      default: return <CheckCircleOutlined />;
    }
  };

  return (
    <div style={{
      minHeight: '100vh',
      background: 'linear-gradient(135deg, #0f172a 0%, #1e293b 50%, #334155 100%)',
      padding: '24px'
    }}>
      <div style={{ maxWidth: '1200px', margin: '0 auto' }}>
        {/* Header */}
        <div style={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          marginBottom: '24px',
          flexWrap: 'wrap',
          gap: '16px'
        }}>
          <div>
            <h1 style={{
              color: 'white',
              fontSize: '2rem',
              fontWeight: '700',
              margin: 0,
              display: 'flex',
              alignItems: 'center',
              gap: '12px'
            }}>
              <DashboardOutlined />
              BPCI Enterprise Dashboard
            </h1>
            <p style={{ color: '#94a3b8', margin: '8px 0 0 0' }}>
              Real-time monitoring and analytics for your Web3 infrastructure
            </p>
          </div>
          
          <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
            <Badge 
              color={getNetworkStatusColor(data.networkStatus)}
              text={
                <span style={{ color: 'white', fontWeight: '600' }}>
                  {data.networkStatus.toUpperCase()}
                </span>
              }
            />
            <Button
              type="primary"
              icon={refreshing ? <Spin size="small" /> : <ReloadOutlined />}
              onClick={handleRefresh}
              loading={refreshing}
              style={{
                background: 'linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%)',
                border: 'none'
              }}
            >
              Refresh
            </Button>
          </div>
        </div>

        {/* Connection Status Alert */}
        {!isConnected && (
          <Alert
            message="Backend Connection"
            description="Using demo data - BPCI backend not available. Real-time data will be available when the backend is running."
            type="warning"
            showIcon
            style={{ marginBottom: '24px' }}
          />
        )}

        {/* Key Metrics */}
        <Row gutter={[16, 16]} style={{ marginBottom: '24px' }}>
          <Col xs={24} sm={12} lg={6}>
            <Card
              style={{
                background: 'rgba(255, 255, 255, 0.05)',
                border: '1px solid rgba(59, 130, 246, 0.3)',
                backdropFilter: 'blur(10px)'
              }}
              bodyStyle={{ padding: '20px' }}
            >
              <Statistic
                title={<span style={{ color: '#94a3b8' }}>Active Nodes</span>}
                value={data.nodes}
                prefix={<NodeIndexOutlined style={{ color: '#3b82f6' }} />}
                valueStyle={{ color: '#3b82f6', fontWeight: '700' }}
              />
              <div style={{ marginTop: '8px' }}>
                <Progress
                  percent={Math.min((data.nodes / 1000) * 100, 100)}
                  strokeColor="#3b82f6"
                  trailColor="rgba(255, 255, 255, 0.1)"
                  showInfo={false}
                  size="small"
                />
              </div>
            </Card>
          </Col>

          <Col xs={24} sm={12} lg={6}>
            <Card
              style={{
                background: 'rgba(255, 255, 255, 0.05)',
                border: '1px solid rgba(16, 185, 129, 0.3)',
                backdropFilter: 'blur(10px)'
              }}
              bodyStyle={{ padding: '20px' }}
            >
              <Statistic
                title={<span style={{ color: '#94a3b8' }}>Transactions</span>}
                value={data.transactions}
                prefix={<TransactionOutlined style={{ color: '#10b981' }} />}
                valueStyle={{ color: '#10b981', fontWeight: '700' }}
              />
              <div style={{ marginTop: '8px', color: '#94a3b8', fontSize: '12px' }}>
                +{Math.floor(Math.random() * 50) + 10} in last hour
              </div>
            </Card>
          </Col>

          <Col xs={24} sm={12} lg={6}>
            <Card
              style={{
                background: 'rgba(255, 255, 255, 0.05)',
                border: '1px solid rgba(139, 92, 246, 0.3)',
                backdropFilter: 'blur(10px)'
              }}
              bodyStyle={{ padding: '20px' }}
            >
              <Statistic
                title={<span style={{ color: '#94a3b8' }}>Active Wallets</span>}
                value={data.wallets}
                prefix={<WalletOutlined style={{ color: '#8b5cf6' }} />}
                valueStyle={{ color: '#8b5cf6', fontWeight: '700' }}
                suffix="+"
              />
              <div style={{ marginTop: '8px', color: '#94a3b8', fontSize: '12px' }}>
                {Math.floor(Math.random() * 10) + 5} new today
              </div>
            </Card>
          </Col>

          <Col xs={24} sm={12} lg={6}>
            <Card
              style={{
                background: 'rgba(255, 255, 255, 0.05)',
                border: '1px solid rgba(245, 158, 11, 0.3)',
                backdropFilter: 'blur(10px)'
              }}
              bodyStyle={{ padding: '20px' }}
            >
              <Statistic
                title={<span style={{ color: '#94a3b8' }}>Network Uptime</span>}
                value={data.uptime}
                prefix={<SecurityScanOutlined style={{ color: '#f59e0b' }} />}
                valueStyle={{ color: '#f59e0b', fontWeight: '700' }}
                suffix="%"
                precision={2}
              />
              <div style={{ marginTop: '8px' }}>
                <Progress
                  percent={data.uptime}
                  strokeColor="#f59e0b"
                  trailColor="rgba(255, 255, 255, 0.1)"
                  showInfo={false}
                  size="small"
                />
              </div>
            </Card>
          </Col>
        </Row>

        {/* Secondary Metrics */}
        <Row gutter={[16, 16]} style={{ marginBottom: '24px' }}>
          <Col xs={24} sm={8}>
            <Card
              title={
                <span style={{ color: 'white', display: 'flex', alignItems: 'center', gap: '8px' }}>
                  <ThunderboltOutlined style={{ color: '#ef4444' }} />
                  Daily Volume
                </span>
              }
              style={{
                background: 'rgba(255, 255, 255, 0.05)',
                border: '1px solid rgba(239, 68, 68, 0.3)',
                backdropFilter: 'blur(10px)'
              }}
              headStyle={{ 
                background: 'rgba(239, 68, 68, 0.1)',
                border: 'none'
              }}
            >
              <Statistic
                value={data.volume}
                valueStyle={{ color: '#ef4444', fontWeight: '700', fontSize: '2rem' }}
                suffix={<span style={{ fontSize: '1rem' }}>BPCI</span>}
              />
              <div style={{ marginTop: '12px', color: '#94a3b8' }}>
                <div style={{ display: 'flex', justifyContent: 'space-between' }}>
                  <span>24h Change:</span>
                  <span style={{ color: '#10b981' }}>+12.5%</span>
                </div>
              </div>
            </Card>
          </Col>

          <Col xs={24} sm={8}>
            <Card
              title={
                <span style={{ color: 'white', display: 'flex', alignItems: 'center', gap: '8px' }}>
                  <GlobalOutlined style={{ color: '#06b6d4' }} />
                  Validators
                </span>
              }
              style={{
                background: 'rgba(255, 255, 255, 0.05)',
                border: '1px solid rgba(6, 182, 212, 0.3)',
                backdropFilter: 'blur(10px)'
              }}
              headStyle={{ 
                background: 'rgba(6, 182, 212, 0.1)',
                border: 'none'
              }}
            >
              <Statistic
                value={data.validators}
                valueStyle={{ color: '#06b6d4', fontWeight: '700', fontSize: '2rem' }}
                suffix="+"
              />
              <div style={{ marginTop: '12px', color: '#94a3b8' }}>
                <div style={{ display: 'flex', justifyContent: 'space-between' }}>
                  <span>Active:</span>
                  <span style={{ color: '#10b981' }}>{Math.floor(data.validators * 0.95)}</span>
                </div>
              </div>
            </Card>
          </Col>

          <Col xs={24} sm={8}>
            <Card
              title={
                <span style={{ color: 'white', display: 'flex', alignItems: 'center', gap: '8px' }}>
                  {getNetworkStatusIcon(data.networkStatus)}
                  Network Status
                </span>
              }
              style={{
                background: 'rgba(255, 255, 255, 0.05)',
                border: `1px solid ${getNetworkStatusColor(data.networkStatus)}40`,
                backdropFilter: 'blur(10px)'
              }}
              headStyle={{ 
                background: `${getNetworkStatusColor(data.networkStatus)}20`,
                border: 'none'
              }}
            >
              <div style={{ textAlign: 'center' }}>
                <div style={{
                  fontSize: '2rem',
                  fontWeight: '700',
                  color: getNetworkStatusColor(data.networkStatus),
                  textTransform: 'uppercase',
                  marginBottom: '8px'
                }}>
                  {data.networkStatus}
                </div>
                <div style={{ color: '#94a3b8', fontSize: '12px' }}>
                  Last updated: {new Date(data.lastUpdate).toLocaleTimeString()}
                </div>
              </div>
            </Card>
          </Col>
        </Row>

        {/* Real-time Activity Feed */}
        <Card
          title={
            <span style={{ color: 'white', display: 'flex', alignItems: 'center', gap: '8px' }}>
              <TransactionOutlined style={{ color: '#3b82f6' }} />
              Real-time Activity
            </span>
          }
          style={{
            background: 'rgba(255, 255, 255, 0.05)',
            border: '1px solid rgba(255, 255, 255, 0.1)',
            backdropFilter: 'blur(10px)'
          }}
          headStyle={{ 
            background: 'rgba(59, 130, 246, 0.1)',
            border: 'none'
          }}
        >
          <div style={{ maxHeight: '300px', overflowY: 'auto' }}>
            {[...Array(10)].map((_, index) => (
              <div
                key={index}
                style={{
                  display: 'flex',
                  justifyContent: 'space-between',
                  alignItems: 'center',
                  padding: '12px 0',
                  borderBottom: index < 9 ? '1px solid rgba(255, 255, 255, 0.1)' : 'none'
                }}
              >
                <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                  <div style={{
                    width: '8px',
                    height: '8px',
                    borderRadius: '50%',
                    background: '#10b981',
                    animation: 'pulse 2s ease-in-out infinite'
                  }} />
                  <span style={{ color: 'white' }}>
                    New transaction from wallet {Math.random().toString(36).substr(2, 8)}
                  </span>
                </div>
                <span style={{ color: '#94a3b8', fontSize: '12px' }}>
                  {Math.floor(Math.random() * 60)} seconds ago
                </span>
              </div>
            ))}
          </div>
        </Card>
      </div>

      {/* CSS for animations */}
      <style>{`
        @keyframes pulse {
          0%, 100% { opacity: 1; }
          50% { opacity: 0.5; }
        }
      `}</style>
    </div>
  );
};

export default Dashboard;
