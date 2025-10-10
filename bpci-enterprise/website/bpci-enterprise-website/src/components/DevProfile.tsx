import React, { useState, useEffect } from 'react';
import { Card, Button, Badge, Tabs, Table, Space, Typography, Spin, Modal, Form, Input, Select, notification } from 'antd';
import { 
  ApiOutlined, 
  WalletOutlined, 
  CloudServerOutlined, 
  SecurityScanOutlined,
  PlusOutlined,
  PlayCircleOutlined,
  StopOutlined,
  ReloadOutlined
} from '@ant-design/icons';
import { bpciApi } from '../services/bpciApi';
import type { UserProfile, DevEnvironment, DevWallet, TestNetwork } from '../services/bpciApi';

const { Title, Text, Paragraph } = Typography;
const { TabPane } = Tabs;

interface DevProfileProps {
  userProfile?: UserProfile;
}

const DevProfile: React.FC<DevProfileProps> = ({ userProfile }) => {
  const [loading, setLoading] = useState(false);
  const [profile, setProfile] = useState<UserProfile | null>(userProfile || null);
  const [, setDevEnvironment] = useState<DevEnvironment | null>(null);
  const [devWallets, setDevWallets] = useState<DevWallet[]>([]);
  const [testNetworks, setTestNetworks] = useState<TestNetwork[]>([]);
  const [bpiCoreStatus, setBpiCoreStatus] = useState<{ connected: boolean; vm_server_status: string } | null>(null);
  const [httpcgStatus, setHttpcgStatus] = useState<{ enabled: boolean; qlock_active: boolean } | null>(null);
  const [shadowRegistryStatus, setShadowRegistryStatus] = useState<{ connected: boolean; entries: number } | null>(null);
  
  // Modal states
  const [walletModalVisible, setWalletModalVisible] = useState(false);
  const [networkModalVisible, setNetworkModalVisible] = useState(false);
  const [walletForm] = Form.useForm();
  const [networkForm] = Form.useForm();

  useEffect(() => {
    loadDevProfile();
  }, []);

  const loadDevProfile = async () => {
    setLoading(true);
    try {
      // Load dev profile and environment
      const [profileRes, envRes, walletsRes, networksRes] = await Promise.all([
        bpciApi.getDevProfile(),
        bpciApi.getDevEnvironment(),
        bpciApi.getDevWallets(),
        bpciApi.getTestNetworks()
      ]);

      if (profileRes.success) setProfile(profileRes.data || null);
      if (envRes.success) setDevEnvironment(envRes.data || null);
      if (walletsRes.success) setDevWallets(walletsRes.data || []);
      if (networksRes.success) setTestNetworks(networksRes.data || []);

      // Load status information
      await loadStatusInfo();
    } catch (error) {
      console.error('Failed to load dev profile:', error);
      notification.error({
        message: 'Failed to Load Profile',
        description: 'Could not load developer profile information.'
      });
    } finally {
      setLoading(false);
    }
  };

  const loadStatusInfo = async () => {
    try {
      const [bpiRes, httpcgRes, shadowRes] = await Promise.all([
        bpciApi.getBpiCoreStatus(),
        bpciApi.getHttpcgStatus(),
        bpciApi.getShadowRegistryStatus()
      ]);

      if (bpiRes.success) setBpiCoreStatus(bpiRes.data || null);
      if (httpcgRes.success) setHttpcgStatus(httpcgRes.data || null);
      if (shadowRes.success) setShadowRegistryStatus(shadowRes.data || null);
    } catch (error) {
      console.error('Failed to load status info:', error);
    }
  };

  const handleConnectBpiCore = async () => {
    try {
      const response = await bpciApi.connectBpiCore();
      if (response.success) {
        notification.success({
          message: 'BPI Core Connected',
          description: 'Successfully connected to BPI Core VM server.'
        });
        await loadStatusInfo();
      } else {
        throw new Error(response.error || 'Failed to connect');
      }
    } catch (error) {
      notification.error({
        message: 'Connection Failed',
        description: 'Failed to connect to BPI Core VM server.'
      });
    }
  };

  const handleEnableHttpcg = async () => {
    try {
      const response = await bpciApi.enableHttpcg();
      if (response.success) {
        notification.success({
          message: 'HTTPCG Enabled',
          description: 'HTTPCG protocol has been enabled with QLOCK integration.'
        });
        await loadStatusInfo();
      } else {
        throw new Error(response.error || 'Failed to enable');
      }
    } catch (error) {
      notification.error({
        message: 'Enable Failed',
        description: 'Failed to enable HTTPCG protocol.'
      });
    }
  };

  const handleCreateWallet = async (values: any) => {
    try {
      const response = await bpciApi.createDevWallet(values.wallet_type, values.network);
      if (response.success) {
        notification.success({
          message: 'Wallet Created',
          description: `Successfully created ${values.wallet_type} wallet for ${values.network}.`
        });
        setWalletModalVisible(false);
        walletForm.resetFields();
        await loadDevProfile();
      } else {
        throw new Error(response.error || 'Failed to create wallet');
      }
    } catch (error) {
      notification.error({
        message: 'Wallet Creation Failed',
        description: 'Failed to create development wallet.'
      });
    }
  };

  const handleCreateNetwork = async (values: any) => {
    try {
      const response = await bpciApi.createTestNetwork(values.name, values.consensus_type);
      if (response.success) {
        notification.success({
          message: 'Network Created',
          description: `Successfully created test network: ${values.name}.`
        });
        setNetworkModalVisible(false);
        networkForm.resetFields();
        await loadDevProfile();
      } else {
        throw new Error(response.error || 'Failed to create network');
      }
    } catch (error) {
      notification.error({
        message: 'Network Creation Failed',
        description: 'Failed to create test network.'
      });
    }
  };

  const handleStartNetwork = async (networkId: string) => {
    try {
      const response = await bpciApi.startTestNetwork(networkId);
      if (response.success) {
        notification.success({
          message: 'Network Started',
          description: 'Test network has been started successfully.'
        });
        await loadDevProfile();
      }
    } catch (error) {
      notification.error({
        message: 'Start Failed',
        description: 'Failed to start test network.'
      });
    }
  };

  const handleStopNetwork = async (networkId: string) => {
    try {
      const response = await bpciApi.stopTestNetwork(networkId);
      if (response.success) {
        notification.success({
          message: 'Network Stopped',
          description: 'Test network has been stopped successfully.'
        });
        await loadDevProfile();
      }
    } catch (error) {
      notification.error({
        message: 'Stop Failed',
        description: 'Failed to stop test network.'
      });
    }
  };

  const walletColumns = [
    {
      title: 'Address',
      dataIndex: 'address',
      key: 'address',
      render: (address: string) => (
        <Text code copyable={{ text: address }}>
          {address.slice(0, 10)}...{address.slice(-8)}
        </Text>
      )
    },
    {
      title: 'Type',
      dataIndex: 'wallet_type',
      key: 'wallet_type',
      render: (type: string) => <Badge color="blue" text={type.toUpperCase()} />
    },
    {
      title: 'Network',
      dataIndex: 'network',
      key: 'network'
    },
    {
      title: 'Balance',
      dataIndex: 'balance',
      key: 'balance',
      render: (balance: string) => <Text strong>{balance} ETH</Text>
    }
  ];

  const networkColumns = [
    {
      title: 'Network ID',
      dataIndex: 'network_id',
      key: 'network_id',
      render: (id: string) => <Text code>{id}</Text>
    },
    {
      title: 'Name',
      dataIndex: 'name',
      key: 'name'
    },
    {
      title: 'Status',
      dataIndex: 'status',
      key: 'status',
      render: (status: string) => (
        <Badge 
          status={status === 'active' ? 'success' : 'default'} 
          text={status.toUpperCase()} 
        />
      )
    },
    {
      title: 'Nodes',
      dataIndex: 'node_count',
      key: 'node_count'
    },
    {
      title: 'Consensus',
      dataIndex: 'consensus_type',
      key: 'consensus_type'
    },
    {
      title: 'Actions',
      key: 'actions',
      render: (_: any, record: TestNetwork) => (
        <Space>
          {record.status === 'inactive' ? (
            <Button 
              type="primary" 
              size="small" 
              icon={<PlayCircleOutlined />}
              onClick={() => handleStartNetwork(record.network_id)}
            >
              Start
            </Button>
          ) : (
            <Button 
              danger 
              size="small" 
              icon={<StopOutlined />}
              onClick={() => handleStopNetwork(record.network_id)}
            >
              Stop
            </Button>
          )}
        </Space>
      )
    }
  ];

  if (loading) {
    return (
      <div style={{ textAlign: 'center', padding: '50px' }}>
        <Spin size="large" />
        <div style={{ marginTop: 16 }}>Loading Developer Profile...</div>
      </div>
    );
  }

  return (
    <div style={{ padding: '24px' }}>
      <div style={{ marginBottom: '24px' }}>
        <Title level={2}>Developer Profile</Title>
        <Paragraph>
          Manage your BPI/BPCI development environment, wallets, and test networks.
        </Paragraph>
      </div>

      {/* Status Overview */}
      <Card title="System Status" style={{ marginBottom: '24px' }}>
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(250px, 1fr))', gap: '16px' }}>
          <Card size="small">
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
              <div>
                <Text strong>BPI Core VM</Text>
                <div>
                  <Badge 
                    status={bpiCoreStatus?.connected ? 'success' : 'error'} 
                    text={bpiCoreStatus?.vm_server_status || 'Disconnected'} 
                  />
                </div>
              </div>
              <div>
                <ApiOutlined style={{ fontSize: '24px', color: bpiCoreStatus?.connected ? '#52c41a' : '#ff4d4f' }} />
                {!bpiCoreStatus?.connected && (
                  <Button 
                    type="link" 
                    size="small" 
                    onClick={handleConnectBpiCore}
                  >
                    Connect
                  </Button>
                )}
              </div>
            </div>
          </Card>

          <Card size="small">
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
              <div>
                <Text strong>HTTPCG Protocol</Text>
                <div>
                  <Badge 
                    status={httpcgStatus?.enabled ? 'success' : 'default'} 
                    text={httpcgStatus?.enabled ? 'Enabled' : 'Disabled'} 
                  />
                  {httpcgStatus?.qlock_active && (
                    <Badge status="processing" text="QLOCK Active" style={{ marginLeft: 8 }} />
                  )}
                </div>
              </div>
              <div>
                <SecurityScanOutlined style={{ fontSize: '24px', color: httpcgStatus?.enabled ? '#52c41a' : '#d9d9d9' }} />
                {!httpcgStatus?.enabled && (
                  <Button 
                    type="link" 
                    size="small" 
                    onClick={handleEnableHttpcg}
                  >
                    Enable
                  </Button>
                )}
              </div>
            </div>
          </Card>

          <Card size="small">
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
              <div>
                <Text strong>Shadow Registry</Text>
                <div>
                  <Badge 
                    status={shadowRegistryStatus?.connected ? 'success' : 'error'} 
                    text={shadowRegistryStatus?.connected ? 'Connected' : 'Disconnected'} 
                  />
                  {shadowRegistryStatus?.entries && (
                    <Text type="secondary" style={{ marginLeft: 8 }}>
                      {shadowRegistryStatus.entries} entries
                    </Text>
                  )}
                </div>
              </div>
              <CloudServerOutlined style={{ fontSize: '24px', color: shadowRegistryStatus?.connected ? '#52c41a' : '#ff4d4f' }} />
            </div>
          </Card>
        </div>
      </Card>

      {/* Main Content Tabs */}
      <Tabs defaultActiveKey="wallets">
        <TabPane tab={<span><WalletOutlined />Dev Wallets</span>} key="wallets">
          <Card 
            title="Development Wallets" 
            extra={
              <Button 
                type="primary" 
                icon={<PlusOutlined />}
                onClick={() => setWalletModalVisible(true)}
              >
                Create Wallet
              </Button>
            }
          >
            <Table 
              columns={walletColumns} 
              dataSource={devWallets} 
              rowKey="address"
              pagination={false}
            />
          </Card>
        </TabPane>

        <TabPane tab={<span><CloudServerOutlined />Test Networks</span>} key="networks">
          <Card 
            title="Test Networks" 
            extra={
              <Space>
                <Button 
                  icon={<ReloadOutlined />}
                  onClick={loadDevProfile}
                >
                  Refresh
                </Button>
                <Button 
                  type="primary" 
                  icon={<PlusOutlined />}
                  onClick={() => setNetworkModalVisible(true)}
                >
                  Create Network
                </Button>
              </Space>
            }
          >
            <Table 
              columns={networkColumns} 
              dataSource={testNetworks} 
              rowKey="network_id"
              pagination={false}
            />
          </Card>
        </TabPane>

        <TabPane tab={<span><ApiOutlined />Profile Settings</span>} key="profile">
          <Card title="Developer Profile Settings">
            {profile && (
              <div>
                <div style={{ marginBottom: '16px' }}>
                  <Text strong>Profile Type: </Text>
                  <Badge color="blue" text={profile.profile_type.toUpperCase()} />
                </div>
                <div style={{ marginBottom: '16px' }}>
                  <Text strong>BPI Core Access: </Text>
                  <Badge 
                    status={profile.bpi_core_access ? 'success' : 'error'} 
                    text={profile.bpi_core_access ? 'Enabled' : 'Disabled'} 
                  />
                </div>
                <div style={{ marginBottom: '16px' }}>
                  <Text strong>Username: </Text>
                  <Text code>{profile.username}</Text>
                </div>
                <div style={{ marginBottom: '16px' }}>
                  <Text strong>Email: </Text>
                  <Text>{profile.email}</Text>
                </div>
              </div>
            )}
          </Card>
        </TabPane>
      </Tabs>

      {/* Create Wallet Modal */}
      <Modal
        title="Create Development Wallet"
        visible={walletModalVisible}
        onCancel={() => setWalletModalVisible(false)}
        footer={null}
      >
        <Form form={walletForm} onFinish={handleCreateWallet} layout="vertical">
          <Form.Item
            name="wallet_type"
            label="Wallet Type"
            rules={[{ required: true, message: 'Please select wallet type' }]}
          >
            <Select placeholder="Select wallet type">
              <Select.Option value="test">Test Wallet</Select.Option>
              <Select.Option value="dev">Development Wallet</Select.Option>
              <Select.Option value="staging">Staging Wallet</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item
            name="network"
            label="Network"
            rules={[{ required: true, message: 'Please enter network name' }]}
          >
            <Input placeholder="e.g., ethereum, bpi-testnet" />
          </Form.Item>
          <Form.Item>
            <Space>
              <Button type="primary" htmlType="submit">
                Create Wallet
              </Button>
              <Button onClick={() => setWalletModalVisible(false)}>
                Cancel
              </Button>
            </Space>
          </Form.Item>
        </Form>
      </Modal>

      {/* Create Network Modal */}
      <Modal
        title="Create Test Network"
        visible={networkModalVisible}
        onCancel={() => setNetworkModalVisible(false)}
        footer={null}
      >
        <Form form={networkForm} onFinish={handleCreateNetwork} layout="vertical">
          <Form.Item
            name="name"
            label="Network Name"
            rules={[{ required: true, message: 'Please enter network name' }]}
          >
            <Input placeholder="e.g., My Test Network" />
          </Form.Item>
          <Form.Item
            name="consensus_type"
            label="Consensus Type"
            rules={[{ required: true, message: 'Please select consensus type' }]}
          >
            <Select placeholder="Select consensus type">
              <Select.Option value="proof-of-stake">Proof of Stake</Select.Option>
              <Select.Option value="proof-of-work">Proof of Work</Select.Option>
              <Select.Option value="bpi-consensus">BPI Consensus</Select.Option>
              <Select.Option value="delegated-pos">Delegated PoS</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item>
            <Space>
              <Button type="primary" htmlType="submit">
                Create Network
              </Button>
              <Button onClick={() => setNetworkModalVisible(false)}>
                Cancel
              </Button>
            </Space>
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
};

export default DevProfile;
