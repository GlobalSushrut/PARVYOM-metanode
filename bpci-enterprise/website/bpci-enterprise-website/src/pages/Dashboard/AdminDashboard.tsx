import React, { useState, useEffect } from 'react';
import { 
  Card, 
  Typography, 
  Button, 
  Alert, 
  Spin, 
  Space,
  Tag,
  Tabs,
  Statistic,
  Row,
  Col,
  Divider,
  Table
} from 'antd';
import { 
  DashboardOutlined,
  ThunderboltOutlined,
  ReloadOutlined,
  DollarOutlined,
  CloudServerOutlined,
  ApiOutlined,
  MonitorOutlined,
  WalletOutlined,
  CrownOutlined,
  SafetyOutlined
} from '@ant-design/icons';
import axios from 'axios';

const { Title, Text, Paragraph } = Typography;

const ADMIN_API_URL = 'http://localhost:9014/api/admin';

interface ComprehensiveDashboard {
  timestamp: string;
  uptime: string;
  total_services: number;
  active_services: number;
  coin_economy: {
    gen_supply: number;
    nex_supply: number;
    flx_supply: number;
    aur_supply: number;
    total_usd_value: number;
    infrastructure_fund_balance: number;
  };
  payment_system: {
    total_wallets: number;
    active_subscriptions: number;
    total_revenue: number;
    testnet_users: number;
    pilot_users: number;
  };
  mojo_monitoring: {
    total_monitored_bpi_os: number;
    active_dashboards: number;
  };
  bso_orchestration: {
    total_deployments: number;
    running_services: number;
  };
}

const AdminDashboard: React.FC = () => {
  const [dashboard, setDashboard] = useState<ComprehensiveDashboard | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchDashboardData = async () => {
    setLoading(true);
    try {
      const response = await axios.get(`${ADMIN_API_URL}/dashboard`);
      if (response.data) {
        setDashboard(response.data);
        setError(null);
      }
    } catch (err: any) {
      console.error('Failed to fetch admin dashboard:', err);
      setError('Failed to connect to Admin Server (port 9014). Please ensure bpci_admin_server is running.');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchDashboardData();
    const interval = setInterval(fetchDashboardData, 10000); // Update every 10 seconds
    return () => clearInterval(interval);
  }, []);

  if (loading && !dashboard) {
    return (
      <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', minHeight: '80vh' }}>
        <Space direction="vertical" size="large" style={{ textAlign: 'center' }}>
          <Spin size="large" />
          <Text style={{ color: '#9CA3AF' }}>Loading Admin Dashboard...</Text>
        </Space>
      </div>
    );
  }

  if (error) {
    return (
      <div style={{ padding: '2rem', maxWidth: '800px', margin: '0 auto' }}>
        <Alert
          message="Admin Server Not Available"
          description={
            <div>
              <Paragraph style={{ color: '#9CA3AF' }}>{error}</Paragraph>
              <Paragraph style={{ color: '#9CA3AF', marginTop: '1rem' }}>
                To start the Admin Server:
              </Paragraph>
              <pre style={{ 
                background: 'rgba(0, 0, 0, 0.3)', 
                padding: '1rem', 
                borderRadius: '8px',
                color: '#E8B44F',
                fontFamily: 'monospace'
              }}>
                cd /home/umesh/metanode/bpci-enterprise{'\n'}
                cargo run --bin bpci_admin_server
              </pre>
            </div>
          }
          type="error"
          showIcon
          action={
            <Button onClick={fetchDashboardData} icon={<ReloadOutlined />}>
              Retry
            </Button>
          }
        />
      </div>
    );
  }

  return (
    <div style={{ padding: '1.5rem' }}>
      {/* Header */}
      <div style={{ marginBottom: '2rem', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <div>
          <Title level={2} style={{ color: '#E8B44F', marginBottom: '0.5rem' }}>
            <DashboardOutlined /> BPCI Admin Dashboard
          </Title>
          <Text style={{ color: '#9CA3AF' }}>
            Real-time monitoring of blockchain infrastructure and autonomous economy
          </Text>
        </div>
        <Button 
          icon={<ReloadOutlined />} 
          onClick={fetchDashboardData}
          loading={loading}
          style={{
            background: 'transparent',
            border: '1px solid #E8B44F',
            color: '#E8B44F'
          }}
        >
          Refresh
        </Button>
      </div>

      {/* System Status */}
      {dashboard && (
        <>
          <Row gutter={[16, 16]} style={{ marginBottom: '2rem' }}>
            <Col xs={24} sm={12} md={6}>
              <Card
                style={{
                  background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
                  border: '1px solid rgba(232, 180, 79, 0.2)',
                  borderRadius: '12px'
                }}
              >
                <Statistic
                  title={<Text style={{ color: '#9CA3AF' }}>System Uptime</Text>}
                  value={dashboard.uptime}
                  prefix={<ThunderboltOutlined style={{ color: '#10B981' }} />}
                  valueStyle={{ color: '#10B981', fontSize: '1.5rem' }}
                />
              </Card>
            </Col>

            <Col xs={24} sm={12} md={6}>
              <Card
                style={{
                  background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
                  border: '1px solid rgba(232, 180, 79, 0.2)',
                  borderRadius: '12px'
                }}
              >
                <Statistic
                  title={<Text style={{ color: '#9CA3AF' }}>Active Services</Text>}
                  value={dashboard.active_services}
                  suffix={`/ ${dashboard.total_services}`}
                  prefix={<CloudServerOutlined style={{ color: '#3B82F6' }} />}
                  valueStyle={{ color: '#3B82F6', fontSize: '1.5rem' }}
                />
              </Card>
            </Col>

            <Col xs={24} sm={12} md={6}>
              <Card
                style={{
                  background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
                  border: '1px solid rgba(232, 180, 79, 0.2)',
                  borderRadius: '12px'
                }}
              >
                <Statistic
                  title={<Text style={{ color: '#9CA3AF' }}>Total Wallets</Text>}
                  value={dashboard.payment_system.total_wallets}
                  prefix={<WalletOutlined style={{ color: '#E8B44F' }} />}
                  valueStyle={{ color: '#E8B44F', fontSize: '1.5rem' }}
                />
              </Card>
            </Col>

            <Col xs={24} sm={12} md={6}>
              <Card
                style={{
                  background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
                  border: '1px solid rgba(232, 180, 79, 0.2)',
                  borderRadius: '12px'
                }}
              >
                <Statistic
                  title={<Text style={{ color: '#9CA3AF' }}>Mojo Monitoring</Text>}
                  value={dashboard.mojo_monitoring.total_monitored_bpi_os}
                  prefix={<MonitorOutlined style={{ color: '#F59E0B' }} />}
                  valueStyle={{ color: '#F59E0B', fontSize: '1.5rem' }}
                />
              </Card>
            </Col>
          </Row>

          {/* 4-Coin Economy */}
          <Card
            title={
              <span style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>
                <DollarOutlined /> 4-Coin Autonomous Economy (GEN/NEX/FLX/AUR)
              </span>
            }
            style={{
              background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
              border: '1px solid rgba(232, 180, 79, 0.2)',
              borderRadius: '12px',
              marginBottom: '2rem'
            }}
          >
            <Row gutter={[16, 16]}>
              <Col xs={12} md={6}>
                <Statistic
                  title={<Text style={{ color: '#9CA3AF' }}>GEN Supply</Text>}
                  value={dashboard.coin_economy.gen_supply}
                  precision={2}
                  valueStyle={{ color: '#10B981' }}
                />
              </Col>
              <Col xs={12} md={6}>
                <Statistic
                  title={<Text style={{ color: '#9CA3AF' }}>NEX Supply</Text>}
                  value={dashboard.coin_economy.nex_supply}
                  precision={2}
                  valueStyle={{ color: '#3B82F6' }}
                />
              </Col>
              <Col xs={12} md={6}>
                <Statistic
                  title={<Text style={{ color: '#9CA3AF' }}>FLX Supply</Text>}
                  value={dashboard.coin_economy.flx_supply}
                  precision={2}
                  valueStyle={{ color: '#F59E0B' }}
                />
              </Col>
              <Col xs={12} md={6}>
                <Statistic
                  title={<Text style={{ color: '#9CA3AF' }}>AUR Supply</Text>}
                  value={dashboard.coin_economy.aur_supply}
                  precision={2}
                  valueStyle={{ color: '#E8B44F' }}
                />
              </Col>
            </Row>

            <Divider style={{ borderColor: 'rgba(232, 180, 79, 0.2)' }} />

            <Row gutter={[16, 16]}>
              <Col xs={24} md={12}>
                <Statistic
                  title={<Text style={{ color: '#9CA3AF' }}>Total USD Value</Text>}
                  value={dashboard.coin_economy.total_usd_value}
                  precision={2}
                  prefix="$"
                  valueStyle={{ color: '#10B981', fontSize: '1.5rem' }}
                />
              </Col>
              <Col xs={24} md={12}>
                <Statistic
                  title={<Text style={{ color: '#9CA3AF' }}>Infrastructure Fund (20%)</Text>}
                  value={dashboard.coin_economy.infrastructure_fund_balance}
                  precision={2}
                  prefix="$"
                  valueStyle={{ color: '#E8B44F', fontSize: '1.5rem' }}
                />
              </Col>
            </Row>
          </Card>

          {/* Payment System */}
          <Card
            title={
              <span style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>
                <CrownOutlined /> Payment System (BPI Bridge)
              </span>
            }
            style={{
              background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
              border: '1px solid rgba(232, 180, 79, 0.2)',
              borderRadius: '12px',
              marginBottom: '2rem'
            }}
          >
            <Row gutter={[16, 16]}>
              <Col xs={12} md={6}>
                <Statistic
                  title={<Text style={{ color: '#9CA3AF' }}>Testnet Users</Text>}
                  value={dashboard.payment_system.testnet_users}
                  valueStyle={{ color: '#3B82F6' }}
                  suffix={<Tag color="blue">10 CAD/mo</Tag>}
                />
              </Col>
              <Col xs={12} md={6}>
                <Statistic
                  title={<Text style={{ color: '#9CA3AF' }}>Pilot Users</Text>}
                  value={dashboard.payment_system.pilot_users}
                  valueStyle={{ color: '#E8B44F' }}
                  suffix={<Tag color="gold">Pilot</Tag>}
                />
              </Col>
              <Col xs={12} md={6}>
                <Statistic
                  title={<Text style={{ color: '#9CA3AF' }}>Active Subscriptions</Text>}
                  value={dashboard.payment_system.active_subscriptions}
                  valueStyle={{ color: '#10B981' }}
                />
              </Col>
              <Col xs={12} md={6}>
                <Statistic
                  title={<Text style={{ color: '#9CA3AF' }}>Total Revenue</Text>}
                  value={dashboard.payment_system.total_revenue}
                  precision={2}
                  prefix="$"
                  valueStyle={{ color: '#10B981', fontSize: '1.25rem' }}
                />
              </Col>
            </Row>
          </Card>

          {/* Infrastructure Status */}
          <Row gutter={[16, 16]}>
            <Col xs={24} md={12}>
              <Card
                title={
                  <span style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>
                    <CloudServerOutlined /> BSO-K8 Orchestration
                  </span>
                }
                style={{
                  background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
                  border: '1px solid rgba(232, 180, 79, 0.2)',
                  borderRadius: '12px'
                }}
              >
                <Space direction="vertical" size="middle" style={{ width: '100%' }}>
                  <Statistic
                    title={<Text style={{ color: '#9CA3AF' }}>Total Deployments</Text>}
                    value={dashboard.bso_orchestration.total_deployments}
                    valueStyle={{ color: '#3B82F6' }}
                  />
                  <Statistic
                    title={<Text style={{ color: '#9CA3AF' }}>Running Services</Text>}
                    value={dashboard.bso_orchestration.running_services}
                    valueStyle={{ color: '#10B981' }}
                  />
                </Space>
              </Card>
            </Col>

            <Col xs={24} md={12}>
              <Card
                title={
                  <span style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>
                    <MonitorOutlined /> Mojo Monitoring
                  </span>
                }
                style={{
                  background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
                  border: '1px solid rgba(232, 180, 79, 0.2)',
                  borderRadius: '12px'
                }}
              >
                <Space direction="vertical" size="middle" style={{ width: '100%' }}>
                  <Statistic
                    title={<Text style={{ color: '#9CA3AF' }}>Monitored BPI OS</Text>}
                    value={dashboard.mojo_monitoring.total_monitored_bpi_os}
                    valueStyle={{ color: '#F59E0B' }}
                  />
                  <Statistic
                    title={<Text style={{ color: '#9CA3AF' }}>Active Dashboards</Text>}
                    value={dashboard.mojo_monitoring.active_dashboards}
                    valueStyle={{ color: '#10B981' }}
                  />
                  <Text style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>
                    Grafana + Prometheus monitoring with token-based auth
                  </Text>
                </Space>
              </Card>
            </Col>
          </Row>

          {/* Footer Info */}
          <Alert
            message="Admin Dashboard - Owner Access Only"
            description="This dashboard provides comprehensive monitoring of all BPCI infrastructure components including the 4-coin autonomous economy, payment system, BSO-K8 orchestration, quantum systems, and Mojo monitoring."
            type="info"
            showIcon
            icon={<SafetyOutlined />}
            style={{
              marginTop: '2rem',
              background: 'rgba(59, 130, 246, 0.1)',
              border: '1px solid rgba(59, 130, 246, 0.3)'
            }}
          />
        </>
      )}
    </div>
  );
};

export default AdminDashboard;
