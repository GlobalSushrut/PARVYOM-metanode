import React, { useState, useEffect } from 'react';
import { Card, Button, Typography, Row, Col, Statistic, Progress, Tag, Avatar, Divider } from 'antd';
import { 
  WalletOutlined, 
  RocketOutlined, 
  SafetyOutlined, 
  ApiOutlined,
  CheckCircleOutlined,
  UserOutlined,
  MailOutlined,
  TeamOutlined,
  TrophyOutlined,
  ThunderboltOutlined,
  ClockCircleOutlined,
  ArrowRightOutlined
} from '@ant-design/icons';
import { useNavigate } from 'react-router-dom';
import { authService } from '../services/authService';
import DashboardLayout from '../layouts/DashboardLayout';

const { Title, Text, Paragraph } = Typography;

const BasicDashboard: React.FC = () => {
  const navigate = useNavigate();
  const [loading, setLoading] = useState(true);
  const [user, setUser] = useState<any>(null);
  const [walletInfo, setWalletInfo] = useState<any>(null);

  useEffect(() => {
    loadUserData();
  }, []);

  const loadUserData = async () => {
    try {
      const currentUser = authService.getCurrentDeveloper();
      const wallet = authService.getWalletInfo();
      
      setUser(currentUser);
      setWalletInfo(wallet);
    } catch (error) {
      console.error('Failed to load user data:', error);
    } finally {
      setLoading(false);
    }
  };

  const profileCompleteness = user?.profile_complete ? 100 : 50;
  const hasWallet = authService.hasWalletActivated();

  return (
    <DashboardLayout>
      <div style={{ padding: '2rem' }}>
        <div style={{ maxWidth: '1400px', margin: '0 auto' }}>
        {/* Header */}
        <div style={{ marginBottom: '2rem' }}>
          <Title level={1} style={{ color: '#ffffff', marginBottom: '0.5rem' }}>
            Welcome back, {user?.name || 'Developer'}! 👋
          </Title>
          <Text style={{ color: '#9CA3AF', fontSize: '1.125rem' }}>
            Your BPCI development dashboard
          </Text>
        </div>

        {/* Quick Stats Row */}
        <Row gutter={[24, 24]} style={{ marginBottom: '2rem' }}>
          <Col xs={24} sm={12} lg={6}>
            <Card
              style={{
                background: 'linear-gradient(135deg, rgba(232, 180, 79, 0.2) 0%, rgba(232, 180, 79, 0.05) 100%)',
                border: '2px solid rgba(232, 180, 79, 0.3)',
                borderRadius: '12px'
              }}
            >
              <Statistic
                title={<span style={{ color: '#E8B44F', fontWeight: '600' }}>Profile Status</span>}
                value={profileCompleteness}
                suffix="%"
                prefix={<CheckCircleOutlined />}
                valueStyle={{ color: '#E8B44F', fontSize: '2rem', fontWeight: 'bold' }}
              />
              <Progress 
                percent={profileCompleteness} 
                strokeColor="#E8B44F" 
                showInfo={false}
                style={{ marginTop: '0.5rem' }}
              />
            </Card>
          </Col>

          <Col xs={24} sm={12} lg={6}>
            <Card
              style={{
                background: 'linear-gradient(135deg, rgba(16, 185, 129, 0.2) 0%, rgba(16, 185, 129, 0.05) 100%)',
                border: '2px solid rgba(16, 185, 129, 0.3)',
                borderRadius: '12px'
              }}
            >
              <Statistic
                title={<span style={{ color: '#10B981', fontWeight: '600' }}>Wallet Status</span>}
                value={hasWallet ? 'Active' : 'Not Activated'}
                prefix={<WalletOutlined />}
                valueStyle={{ color: '#10B981', fontSize: '1.5rem', fontWeight: 'bold' }}
              />
              {hasWallet && walletInfo && (
                <Text style={{ color: '#10B981', fontSize: '0.875rem', display: 'block', marginTop: '0.5rem' }}>
                  {walletInfo.wallet_type} Wallet
                </Text>
              )}
            </Card>
          </Col>


        </Row>

        {/* Main Content Row */}
        <Row gutter={[24, 24]}>
          {/* Profile Card */}
          <Col xs={24} lg={8}>
            <Card
              style={{
                background: 'rgba(10, 22, 40, 0.95)',
                border: '2px solid rgba(232, 180, 79, 0.3)',
                borderRadius: '12px',
                height: '100%'
              }}
            >
              <div style={{ textAlign: 'center', marginBottom: '1.5rem' }}>
                <Avatar 
                  size={80} 
                  icon={<UserOutlined />}
                  style={{ 
                    background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)',
                    marginBottom: '1rem'
                  }}
                />
                <Title level={3} style={{ color: '#ffffff', marginBottom: '0.5rem' }}>
                  {user?.name || 'Developer'}
                </Title>
                <Tag color="gold" style={{ fontSize: '0.875rem' }}>
                  {user?.role || 'Developer'}
                </Tag>
              </div>

              <Divider style={{ borderColor: 'rgba(232, 180, 79, 0.2)' }} />

              <div style={{ marginBottom: '1rem' }}>
                <Text style={{ color: '#9CA3AF', display: 'block', marginBottom: '0.5rem' }}>
                  <MailOutlined style={{ marginRight: '0.5rem', color: '#E8B44F' }} />
                  Email
                </Text>
                <Text style={{ color: '#ffffff', fontSize: '1rem' }}>
                  {user?.email || 'Not set'}
                </Text>
              </div>

              {user?.company && (
                <div style={{ marginBottom: '1rem' }}>
                  <Text style={{ color: '#9CA3AF', display: 'block', marginBottom: '0.5rem' }}>
                    <TeamOutlined style={{ marginRight: '0.5rem', color: '#E8B44F' }} />
                    Company
                  </Text>
                  <Text style={{ color: '#ffffff', fontSize: '1rem' }}>
                    {user.company}
                  </Text>
                </div>
              )}

              <div style={{ marginBottom: '1rem' }}>
                <Text style={{ color: '#9CA3AF', display: 'block', marginBottom: '0.5rem' }}>
                  <ClockCircleOutlined style={{ marginRight: '0.5rem', color: '#E8B44F' }} />
                  Member Since
                </Text>
                <Text style={{ color: '#ffffff', fontSize: '1rem' }}>
                  {user?.created_at ? new Date(user.created_at).toLocaleDateString() : 'Recently'}
                </Text>
              </div>

              <Button
                block
                size="large"
                style={{
                  background: 'transparent',
                  border: '2px solid #E8B44F',
                  color: '#E8B44F',
                  fontWeight: '600',
                  marginTop: '1rem'
                }}
                onClick={() => navigate('/profile')}
              >
                Edit Profile
              </Button>
            </Card>
          </Col>

          {/* Quick Actions Card */}
          <Col xs={24} lg={16}>
            <Card
              style={{
                background: 'rgba(10, 22, 40, 0.95)',
                border: '2px solid rgba(232, 180, 79, 0.3)',
                borderRadius: '12px',
                height: '100%'
              }}
            >
              <Title level={3} style={{ color: '#E8B44F', marginBottom: '1.5rem' }}>
                Quick Actions
              </Title>

              <Row gutter={[16, 16]}>
                {/* Wallet Action */}
                <Col xs={24} sm={12}>
                  <Card
                    hoverable
                    onClick={() => navigate('/wallet')}
                    style={{
                      background: 'linear-gradient(135deg, rgba(16, 185, 129, 0.1) 0%, rgba(16, 185, 129, 0.05) 100%)',
                      border: '1px solid rgba(16, 185, 129, 0.3)',
                      borderRadius: '8px',
                      cursor: 'pointer'
                    }}
                  >
                    <div style={{ display: 'flex', alignItems: 'center', gap: '1rem' }}>
                      <div style={{
                        width: '48px',
                        height: '48px',
                        borderRadius: '8px',
                        background: 'rgba(16, 185, 129, 0.2)',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center'
                      }}>
                        <WalletOutlined style={{ fontSize: '1.5rem', color: '#10B981' }} />
                      </div>
                      <div style={{ flex: 1 }}>
                        <Title level={5} style={{ color: '#ffffff', margin: 0 }}>
                          {hasWallet ? 'Manage Wallet' : 'Activate Wallet'}
                        </Title>
                        <Text style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>
                          {hasWallet ? 'View balance & transactions' : 'Get started with Mojo'}
                        </Text>
                      </div>
                      <ArrowRightOutlined style={{ color: '#10B981' }} />
                    </div>
                  </Card>
                </Col>

                {/* API Access */}
                <Col xs={24} sm={12}>
                  <Card
                    hoverable
                    onClick={() => navigate('/api-docs')}
                    style={{
                      background: 'linear-gradient(135deg, rgba(59, 130, 246, 0.1) 0%, rgba(59, 130, 246, 0.05) 100%)',
                      border: '1px solid rgba(59, 130, 246, 0.3)',
                      borderRadius: '8px',
                      cursor: 'pointer'
                    }}
                  >
                    <div style={{ display: 'flex', alignItems: 'center', gap: '1rem' }}>
                      <div style={{
                        width: '48px',
                        height: '48px',
                        borderRadius: '8px',
                        background: 'rgba(59, 130, 246, 0.2)',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center'
                      }}>
                        <ApiOutlined style={{ fontSize: '1.5rem', color: '#3B82F6' }} />
                      </div>
                      <div style={{ flex: 1 }}>
                        <Title level={5} style={{ color: '#ffffff', margin: 0 }}>
                          API Documentation
                        </Title>
                        <Text style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>
                          Explore BPCI APIs
                        </Text>
                      </div>
                      <ArrowRightOutlined style={{ color: '#3B82F6' }} />
                    </div>
                  </Card>
                </Col>

                {/* Deploy Node */}
                <Col xs={24} sm={12}>
                  <Card
                    hoverable
                    onClick={() => navigate('/installer')}
                    style={{
                      background: 'linear-gradient(135deg, rgba(124, 58, 237, 0.1) 0%, rgba(124, 58, 237, 0.05) 100%)',
                      border: '1px solid rgba(124, 58, 237, 0.3)',
                      borderRadius: '8px',
                      cursor: 'pointer'
                    }}
                  >
                    <div style={{ display: 'flex', alignItems: 'center', gap: '1rem' }}>
                      <div style={{
                        width: '48px',
                        height: '48px',
                        borderRadius: '8px',
                        background: 'rgba(124, 58, 237, 0.2)',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center'
                      }}>
                        <RocketOutlined style={{ fontSize: '1.5rem', color: '#7C3AED' }} />
                      </div>
                      <div style={{ flex: 1 }}>
                        <Title level={5} style={{ color: '#ffffff', margin: 0 }}>
                          Deploy BPI Node
                        </Title>
                        <Text style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>
                          Install BPI OS & SDK
                        </Text>
                      </div>
                      <ArrowRightOutlined style={{ color: '#7C3AED' }} />
                    </div>
                  </Card>
                </Col>

                {/* Security */}
                <Col xs={24} sm={12}>
                  <Card
                    hoverable
                    onClick={() => navigate('/security')}
                    style={{
                      background: 'linear-gradient(135deg, rgba(239, 68, 68, 0.1) 0%, rgba(239, 68, 68, 0.05) 100%)',
                      border: '1px solid rgba(239, 68, 68, 0.3)',
                      borderRadius: '8px',
                      cursor: 'pointer'
                    }}
                  >
                    <div style={{ display: 'flex', alignItems: 'center', gap: '1rem' }}>
                      <div style={{
                        width: '48px',
                        height: '48px',
                        borderRadius: '8px',
                        background: 'rgba(239, 68, 68, 0.2)',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center'
                      }}>
                        <SafetyOutlined style={{ fontSize: '1.5rem', color: '#EF4444' }} />
                      </div>
                      <div style={{ flex: 1 }}>
                        <Title level={5} style={{ color: '#ffffff', margin: 0 }}>
                          Security Settings
                        </Title>
                        <Text style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>
                          2FA & password
                        </Text>
                      </div>
                      <ArrowRightOutlined style={{ color: '#EF4444' }} />
                    </div>
                  </Card>
                </Col>
              </Row>

              {/* Getting Started Section */}
              {!hasWallet && (
                <div style={{
                  marginTop: '2rem',
                  padding: '1.5rem',
                  background: 'linear-gradient(135deg, rgba(232, 180, 79, 0.1) 0%, rgba(232, 180, 79, 0.05) 100%)',
                  border: '1px solid rgba(232, 180, 79, 0.3)',
                  borderRadius: '8px'
                }}>
                  <Title level={4} style={{ color: '#E8B44F', marginBottom: '1rem' }}>
                    🚀 Get Started with BPCI
                  </Title>
                  <Paragraph style={{ color: '#ffffff', marginBottom: '1rem' }}>
                    Activate your Mojo wallet to unlock full access to the Pravyom network:
                  </Paragraph>
                  <ul style={{ color: '#9CA3AF', marginBottom: '1rem', paddingLeft: '1.5rem' }}>
                    <li>Receive Mother Coin allocation</li>
                    <li>Mine Baby Coins with Proof-of-Existence</li>
                    <li>Access developer APIs and tools</li>
                    <li>Deploy BPI nodes and services</li>
                  </ul>
                  <Button
                    type="primary"
                    size="large"
                    icon={<WalletOutlined />}
                    onClick={() => navigate('/wallet')}
                    style={{
                      background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)',
                      border: 'none',
                      color: '#0A1628',
                      fontWeight: '600'
                    }}
                  >
                    Activate Mojo Wallet
                  </Button>
                </div>
              )}
            </Card>
          </Col>
        </Row>
        </div>
      </div>
    </DashboardLayout>
  );
};

export default BasicDashboard;
