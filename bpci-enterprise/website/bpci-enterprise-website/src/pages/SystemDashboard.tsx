import React, { useState, useEffect } from 'react';
import { Card, Typography, Row, Col, Statistic, Alert, Space, Button, Spin } from 'antd';
import {
  DashboardOutlined,
  WalletOutlined,
  UserOutlined,
  ThunderboltOutlined,
  CheckCircleOutlined,
  ClockCircleOutlined,
  ReloadOutlined
} from '@ant-design/icons';
import { useNavigate } from 'react-router-dom';
import axios from 'axios';
import { authService } from '../services/authService';

const { Title, Text } = Typography;

const INSTALLER_API = 'http://localhost:8080/api'; // Community Installer Web Server

interface SystemStatus {
  status: string;
  uptime: string;
  services_running: number;
  total_services: number;
}

interface WalletInfo {
  wallet_id: string;
  wallet_name: string;
  bpi_address: string;
  is_activated: boolean;
  balance: number;
}

const SystemDashboard: React.FC = () => {
  const navigate = useNavigate();
  const [loading, setLoading] = useState(true);
  const [user, setUser] = useState<any>(null);
  const [wallets, setWallets] = useState<WalletInfo[]>([]);
  const [systemStatus, setSystemStatus] = useState<SystemStatus | null>(null);

  useEffect(() => {
    loadDashboardData();
  }, []);

  const loadDashboardData = async () => {
    setLoading(true);
    try {
      // Get current user
      const currentUser = authService.getCurrentDeveloper();
      setUser(currentUser);

      // Load wallets
      await loadWallets();

      // Load system status
      await loadSystemStatus();
    } catch (error) {
      console.error('Failed to load dashboard data:', error);
    } finally {
      setLoading(false);
    }
  };

  const loadWallets = async () => {
    try {
      const response = await axios.get(`${INSTALLER_API}/wallet/list`);
      if (response.data.success && response.data.data) {
        setWallets(response.data.data);
      }
    } catch (error) {
      console.error('Failed to load wallets:', error);
    }
  };

  const loadSystemStatus = async () => {
    try {
      const response = await axios.get(`${INSTALLER_API}/status`);
      if (response.data.success && response.data.data) {
        setSystemStatus(response.data.data);
      }
    } catch (error) {
      console.error('Failed to load system status:', error);
    }
  };

  if (loading) {
    return (
      <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', minHeight: '80vh' }}>
        <Space direction="vertical" size="large" style={{ textAlign: 'center' }}>
          <Spin size="large" />
          <Text style={{ color: '#9CA3AF' }}>Loading Dashboard...</Text>
        </Space>
      </div>
    );
  }

  const activeWallets = wallets.filter(w => w.is_activated).length;
  const totalBalance = wallets.reduce((sum, w) => sum + (w.balance || 0), 0);

  return (
    <div style={{ padding: '1.5rem' }}>
      {/* Header */}
      <div style={{ marginBottom: '2rem', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <div>
          <Title level={2} style={{ color: '#E8B44F', marginBottom: '0.5rem' }}>
            <DashboardOutlined /> System Dashboard
          </Title>
          <Text style={{ color: '#9CA3AF' }}>
            Welcome back, {user?.name || user?.email || 'User'}!
          </Text>
        </div>
        <Button 
          icon={<ReloadOutlined />} 
          onClick={loadDashboardData}
          style={{
            background: 'transparent',
            border: '1px solid #E8B44F',
            color: '#E8B44F'
          }}
        >
          Refresh
        </Button>
      </div>

      {/* Quick Stats */}
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
              title={<Text style={{ color: '#9CA3AF' }}>Total Wallets</Text>}
              value={wallets.length}
              prefix={<WalletOutlined style={{ color: '#E8B44F' }} />}
              valueStyle={{ color: '#E8B44F' }}
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
              title={<Text style={{ color: '#9CA3AF' }}>Active Wallets</Text>}
              value={activeWallets}
              prefix={<CheckCircleOutlined style={{ color: '#10B981' }} />}
              valueStyle={{ color: '#10B981' }}
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
              title={<Text style={{ color: '#9CA3AF' }}>Total Balance</Text>}
              value={totalBalance}
              precision={2}
              suffix="BPI"
              prefix={<DashboardOutlined style={{ color: '#3B82F6' }} />}
              valueStyle={{ color: '#3B82F6' }}
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
              title={<Text style={{ color: '#9CA3AF' }}>System Status</Text>}
              value={systemStatus?.status || 'Unknown'}
              prefix={<ThunderboltOutlined style={{ color: '#10B981' }} />}
              valueStyle={{ color: '#10B981', fontSize: '1.25rem' }}
            />
          </Card>
        </Col>
      </Row>

      {/* Main Content */}
      <Row gutter={[16, 16]}>
        {/* User Profile */}
        <Col xs={24} md={12}>
          <Card
            title={
              <span style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>
                <UserOutlined /> User Profile
              </span>
            }
            style={{
              background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
              border: '1px solid rgba(232, 180, 79, 0.2)',
              borderRadius: '12px'
            }}
          >
            <Space direction="vertical" size="middle" style={{ width: '100%' }}>
              <div>
                <Text style={{ color: '#9CA3AF' }}>Email: </Text>
                <Text style={{ color: '#E8B44F' }}>{user?.email || 'N/A'}</Text>
              </div>
              <div>
                <Text style={{ color: '#9CA3AF' }}>Name: </Text>
                <Text style={{ color: '#E8B44F' }}>{user?.name || 'N/A'}</Text>
              </div>
              <div>
                <Text style={{ color: '#9CA3AF' }}>Account Type: </Text>
                <Text style={{ color: '#10B981' }}>Developer</Text>
              </div>
            </Space>
          </Card>
        </Col>

        {/* System Status */}
        <Col xs={24} md={12}>
          <Card
            title={
              <span style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>
                <ThunderboltOutlined /> System Status
              </span>
            }
            style={{
              background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
              border: '1px solid rgba(232, 180, 79, 0.2)',
              borderRadius: '12px'
            }}
          >
            {systemStatus ? (
              <Space direction="vertical" size="middle" style={{ width: '100%' }}>
                <div>
                  <Text style={{ color: '#9CA3AF' }}>Status: </Text>
                  <Text style={{ color: '#10B981', fontWeight: 'bold' }}>{systemStatus.status}</Text>
                </div>
                <div>
                  <Text style={{ color: '#9CA3AF' }}>Uptime: </Text>
                  <Text style={{ color: '#E8B44F' }}>{systemStatus.uptime}</Text>
                </div>
                <div>
                  <Text style={{ color: '#9CA3AF' }}>Services: </Text>
                  <Text style={{ color: '#3B82F6' }}>
                    {systemStatus.services_running} / {systemStatus.total_services} running
                  </Text>
                </div>
              </Space>
            ) : (
              <Alert
                message="System status unavailable"
                description="Unable to connect to the backend server. Please ensure the Community Installer Web Server is running on port 8080."
                type="warning"
                showIcon
              />
            )}
          </Card>
        </Col>

        {/* Quick Actions */}
        <Col xs={24}>
          <Card
            title={
              <span style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>
                <ClockCircleOutlined /> Quick Actions
              </span>
            }
            style={{
              background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
              border: '1px solid rgba(232, 180, 79, 0.2)',
              borderRadius: '12px'
            }}
          >
            <Space size="middle" wrap>
              <Button
                type="primary"
                icon={<WalletOutlined />}
                onClick={() => navigate('/wallet')}
                style={{
                  background: 'linear-gradient(135deg, #E8B44F 0%, #D4A044 100%)',
                  border: 'none',
                  fontWeight: '600'
                }}
              >
                View Wallets
              </Button>
              <Button
                icon={<DashboardOutlined />}
                onClick={() => navigate('/mojo-dashboard')}
                style={{
                  background: 'transparent',
                  border: '1px solid #E8B44F',
                  color: '#E8B44F'
                }}
              >
                Mojo Monitoring
              </Button>
              <Button
                icon={<DashboardOutlined />}
                onClick={() => navigate('/registry')}
                style={{
                  background: 'transparent',
                  border: '1px solid #3B82F6',
                  color: '#3B82F6'
                }}
              >
                Registry Dashboard
              </Button>
            </Space>
          </Card>
        </Col>
      </Row>
    </div>
  );
};

export default SystemDashboard;
