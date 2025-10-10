import React, { useState, useEffect } from 'react';
import { 
  Card, 
  Button, 
  Switch, 
  Descriptions, 
  Alert, 
  Space, 
  Typography, 
  Divider,
  Tag,
  Statistic,
  Row,
  Col
} from 'antd';
import { 
  ExperimentOutlined, 
  DatabaseOutlined, 
  WalletOutlined, 
  UserOutlined,
  ApiOutlined,
  SettingOutlined,
  CheckCircleOutlined,
  CloseCircleOutlined
} from '@ant-design/icons';
import { 
  enableTestMode, 
  disableTestMode, 
  isTestMode, 
  testUsers, 
  testWallets, 
  testSystemStatus, 
  testEconomicStatus,
  getTestUser,
  setTestAuthToken
} from '../../data/testData';

const { Title, Text, Paragraph } = Typography;

const TestModePanel: React.FC = () => {
  const [testModeEnabled, setTestModeEnabled] = useState(false);
  const [currentUser, setCurrentUser] = useState<any>(null);

  useEffect(() => {
    setTestModeEnabled(isTestMode());
    if (isTestMode()) {
      setCurrentUser(getTestUser());
    }
  }, []);

  const handleTestModeToggle = (enabled: boolean) => {
    if (enabled) {
      enableTestMode();
      setCurrentUser(getTestUser());
    } else {
      disableTestMode();
      setCurrentUser(null);
    }
    setTestModeEnabled(enabled);
  };

  const switchUser = (role: string) => {
    const user = getTestUser(role);
    setTestAuthToken(user.auth_token);
    setCurrentUser(user);
    window.location.reload(); // Refresh to apply new user context
  };

  return (
    <div style={{ padding: '24px', maxWidth: '1200px', margin: '0 auto' }}>
      <Card>
        <div style={{ textAlign: 'center', marginBottom: '24px' }}>
          <ExperimentOutlined style={{ fontSize: '48px', color: '#1890ff', marginBottom: '16px' }} />
          <Title level={2}>Internal Infrastructure Test Mode</Title>
          <Paragraph>
            Enable test mode to access the internal infrastructure without authentication.
            This allows testing of BPI wallets, dashboard, and all system components with mock data.
          </Paragraph>
        </div>

        <div style={{ textAlign: 'center', marginBottom: '32px' }}>
          <Space size="large" align="center">
            <Text strong>Test Mode:</Text>
            <Switch
              checked={testModeEnabled}
              onChange={handleTestModeToggle}
              checkedChildren={<CheckCircleOutlined />}
              unCheckedChildren={<CloseCircleOutlined />}

            />
            <Tag color={testModeEnabled ? 'green' : 'red'}>
              {testModeEnabled ? 'ENABLED' : 'DISABLED'}
            </Tag>
          </Space>
        </div>

        {testModeEnabled && (
          <>
            <Alert
              message="Test Mode Active"
              description="You are now using mock data for all API calls. All wallet operations, dashboard data, and system status are simulated."
              type="success"
              showIcon
              style={{ marginBottom: '24px' }}
            />

            {/* Current Test User */}
            {currentUser && (
              <Card title={<><UserOutlined /> Current Test User</>} style={{ marginBottom: '24px' }}>
                <Descriptions column={2}>
                  <Descriptions.Item label="Name">{currentUser.name}</Descriptions.Item>
                  <Descriptions.Item label="Email">{currentUser.email}</Descriptions.Item>
                  <Descriptions.Item label="Role">
                    <Tag color="blue">{currentUser.role}</Tag>
                  </Descriptions.Item>
                  <Descriptions.Item label="User ID">{currentUser.id}</Descriptions.Item>
                </Descriptions>
                
                <Divider />
                
                <Text strong>Switch Test User:</Text>
                <div style={{ marginTop: '8px' }}>
                  <Space wrap>
                    <Button 
                      size="small" 
                      onClick={() => switchUser('founder')}
                      type={currentUser.role === 'founder' ? 'primary' : 'default'}
                    >
                      Founder
                    </Button>
                    <Button 
                      size="small" 
                      onClick={() => switchUser('developer')}
                      type={currentUser.role === 'developer' ? 'primary' : 'default'}
                    >
                      Developer
                    </Button>
                    <Button 
                      size="small" 
                      onClick={() => switchUser('enterprise')}
                      type={currentUser.role === 'enterprise' ? 'primary' : 'default'}
                    >
                      Enterprise
                    </Button>
                  </Space>
                </div>
              </Card>
            )}

            {/* Test Data Overview */}
            <Row gutter={[16, 16]} style={{ marginBottom: '24px' }}>
              <Col xs={24} sm={12} md={6}>
                <Card>
                  <Statistic
                    title="Test Wallets"
                    value={testWallets.length}
                    prefix={<WalletOutlined />}
                    valueStyle={{ color: '#3f8600' }}
                  />
                </Card>
              </Col>
              <Col xs={24} sm={12} md={6}>
                <Card>
                  <Statistic
                    title="Test Users"
                    value={testUsers.length}
                    prefix={<UserOutlined />}
                    valueStyle={{ color: '#1890ff' }}
                  />
                </Card>
              </Col>
              <Col xs={24} sm={12} md={6}>
                <Card>
                  <Statistic
                    title="BPCI Nodes"
                    value={testSystemStatus.data.bpci_nodes}
                    prefix={<DatabaseOutlined />}
                    valueStyle={{ color: '#722ed1' }}
                  />
                </Card>
              </Col>
              <Col xs={24} sm={12} md={6}>
                <Card>
                  <Statistic
                    title="Active Wallets"
                    value={testSystemStatus.data.active_wallets}
                    prefix={<ApiOutlined />}
                    valueStyle={{ color: '#eb2f96' }}
                  />
                </Card>
              </Col>
            </Row>

            {/* Test Wallets Preview */}
            <Card title={<><WalletOutlined /> Test BPI Wallets</>} style={{ marginBottom: '24px' }}>
              <div style={{ display: 'grid', gap: '16px' }}>
                {testWallets.map((wallet) => (
                  <Card key={wallet.id} size="small" style={{ backgroundColor: '#fafafa' }}>
                    <Row gutter={16}>
                      <Col span={12}>
                        <Text strong>{wallet.wallet_type} Wallet</Text>
                        <br />
                        <Text code style={{ fontSize: '12px' }}>{wallet.bpi_address}</Text>
                      </Col>
                      <Col span={6}>
                        <Text type="secondary">Mother Coins:</Text>
                        <br />
                        <Text strong style={{ color: '#3f8600' }}>{wallet.mother_coin_balance} MC</Text>
                      </Col>
                      <Col span={6}>
                        <Text type="secondary">Baby Coins:</Text>
                        <br />
                        <Text strong style={{ color: '#1890ff' }}>{wallet.baby_coin_balance} BC</Text>
                      </Col>
                    </Row>
                  </Card>
                ))}
              </div>
            </Card>

            {/* Economic Test Data */}
            <Card title={<><DatabaseOutlined /> Test Economic Data</>} style={{ marginBottom: '24px' }}>
              <Row gutter={[16, 16]}>
                <Col xs={24} md={8}>
                  <Card size="small" title="Mother Coins">
                    <Statistic
                      title="Total Supply"
                      value={testEconomicStatus.data.mother_coins.total_supply}
                      formatter={(value) => `${Number(value).toLocaleString()} MC`}
                    />
                    <Statistic
                      title="Circulating"
                      value={testEconomicStatus.data.mother_coins.circulating}
                      formatter={(value) => `${Number(value).toLocaleString()} MC`}
                    />
                  </Card>
                </Col>
                <Col xs={24} md={8}>
                  <Card size="small" title="Baby Coins">
                    <Statistic
                      title="Total Earned"
                      value={testEconomicStatus.data.baby_coins.total_earned}
                      formatter={(value) => `${Number(value).toLocaleString()} BC`}
                    />
                    <Statistic
                      title="Active Miners"
                      value={testEconomicStatus.data.baby_coins.active_miners}
                    />
                  </Card>
                </Col>
                <Col xs={24} md={8}>
                  <Card size="small" title="Bank APIs">
                    <Statistic
                      title="Total Transactions"
                      value={testEconomicStatus.data.bank_apis.total_transactions}
                      formatter={(value) => Number(value).toLocaleString()}
                    />
                    <Statistic
                      title="Settlement Volume"
                      value={testEconomicStatus.data.bank_apis.settlement_volume}
                      formatter={(value) => `$${Number(value).toLocaleString()}`}
                    />
                  </Card>
                </Col>
              </Row>
            </Card>

            {/* Quick Actions */}
            <Card title={<><SettingOutlined /> Quick Actions</>}>
              <Space wrap>
                <Button 
                  type="primary" 
                  onClick={() => window.location.href = '/dashboard'}
                >
                  Go to Dashboard
                </Button>
                <Button 
                  onClick={() => window.location.href = '/'}
                >
                  Go to Homepage
                </Button>
                <Button 
                  onClick={() => {
                    console.log('Test Data:', { testUsers, testWallets, testSystemStatus, testEconomicStatus });
                  }}
                >
                  Log Test Data to Console
                </Button>
                <Button 
                  danger
                  onClick={() => {
                    if (window.confirm('Are you sure you want to disable test mode?')) {
                      handleTestModeToggle(false);
                    }
                  }}
                >
                  Disable Test Mode
                </Button>
              </Space>
            </Card>
          </>
        )}

        {!testModeEnabled && (
          <Alert
            message="Test Mode Disabled"
            description="Enable test mode above to access internal infrastructure testing features with mock data."
            type="info"
            showIcon
          />
        )}
      </Card>
    </div>
  );
};

export default TestModePanel;
