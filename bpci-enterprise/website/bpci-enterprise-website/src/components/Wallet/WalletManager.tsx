import React, { useState, useEffect } from 'react';
import { Card, Row, Col, Button, Modal, Form, Select, InputNumber, Typography, Space, Alert, Spin, Tag, Statistic } from 'antd';
import { 
  WalletOutlined, 
  PlusOutlined, 
  SafetyOutlined, 
  BankOutlined,
  CrownOutlined,
  TeamOutlined
} from '@ant-design/icons';
import { registryService, WalletInfo } from '../../services/registryService';
import { authService } from '../../services/authService';

const { Title, Text } = Typography;
const { Option } = Select;

export const WalletManager: React.FC = () => {
  const [wallets, setWallets] = useState<WalletInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const [createModalVisible, setCreateModalVisible] = useState(false);
  const [stampModalVisible, setStampModalVisible] = useState(false);
  const [selectedWallet, setSelectedWallet] = useState<string | null>(null);
  const [form] = Form.useForm();
  const [stampForm] = Form.useForm();
  const [error, setError] = useState<string | null>(null);

  const fetchWallets = async () => {
    setLoading(true);
    setError(null);

    try {
      const walletsData = await registryService.getWallets(1, 20);
      setWallets(walletsData?.wallets || []);
    } catch (err) {
      setError('Failed to load wallet data');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchWallets();
  }, []);

  const handleCreateWallet = async (values: any) => {
    try {
      const result = await registryService.createBPCIWallet({
        wallet_type: values.wallet_type,
        initial_stake: values.initial_stake
      });

      if (result.success) {
        setCreateModalVisible(false);
        form.resetFields();
        fetchWallets();
      } else {
        setError(result.message);
      }
    } catch (err) {
      setError('Failed to create wallet');
    }
  };

  const handleStampWallet = async (values: any) => {
    if (!selectedWallet) return;

    try {
      const result = await registryService.stampWallet(selectedWallet, values.stamp_type);

      if (result.success) {
        setStampModalVisible(false);
        stampForm.resetFields();
        setSelectedWallet(null);
        fetchWallets();
      } else {
        setError(result.message);
      }
    } catch (err) {
      setError('Failed to stamp wallet');
    }
  };

  const getWalletTypeIcon = (walletType: string) => {
    const icons: Record<string, React.ReactNode> = {
      'Normal': <WalletOutlined />,
      'Compliance': <SafetyOutlined />,
      'Regulated': <BankOutlined />,
      'Government': <CrownOutlined />,
      'Emergency': <SafetyOutlined />,
      'Bank': <BankOutlined />,
      'Community': <TeamOutlined />
    };
    return icons[walletType] || <WalletOutlined />;
  };

  const getWalletTypeColor = (walletType: string) => {
    const colors: Record<string, string> = {
      'Normal': 'default',
      'Compliance': 'blue',
      'Regulated': 'orange',
      'Government': 'red',
      'Emergency': 'magenta',
      'Bank': 'green',
      'Community': 'cyan'
    };
    return colors[walletType] || 'default';
  };

  const getStatusColor = (status: string) => {
    const colors: Record<string, string> = {
      'Active': 'success',
      'Suspended': 'warning',
      'Locked': 'error'
    };
    return colors[status] || 'default';
  };

  if (loading) {
    return (
      <div style={{ textAlign: 'center', padding: '50px' }}>
        <Spin size="large" />
        <div style={{ marginTop: '16px' }}>Loading wallet data...</div>
      </div>
    );
  }

  return (
    <div style={{ padding: '24px' }}>
      <div style={{ marginBottom: '24px', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <Title level={2} style={{ margin: 0 }}>
          <WalletOutlined style={{ marginRight: '12px', color: '#667eea' }} />
          BPCI Wallet Manager
        </Title>
        <Button 
          type="primary" 
          icon={<PlusOutlined />}
          onClick={() => setCreateModalVisible(true)}
          style={{
            background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
            border: 'none'
          }}
        >
          Create New Wallet
        </Button>
      </div>

      {error && (
        <Alert
          message="Error"
          description={error}
          type="error"
          showIcon
          closable
          onClose={() => setError(null)}
          style={{ marginBottom: '24px' }}
        />
      )}

      <Row gutter={[16, 16]}>
        {wallets.map((wallet) => (
          <Col xs={24} sm={12} lg={8} xl={6} key={wallet.wallet_id}>
            <Card
              hoverable
              style={{ height: '100%' }}
              actions={[
                <Button 
                  type="link" 
                  onClick={() => {
                    setSelectedWallet(wallet.wallet_id);
                    setStampModalVisible(true);
                  }}
                >
                  Stamp Wallet
                </Button>,
                <Button type="link">View Details</Button>
              ]}
            >
              <div style={{ textAlign: 'center', marginBottom: '16px' }}>
                <div style={{ fontSize: '32px', color: '#667eea', marginBottom: '8px' }}>
                  {getWalletTypeIcon(wallet.wallet_type)}
                </div>
                <Text code style={{ fontSize: '12px' }}>
                  {wallet.wallet_id.substring(0, 16)}...
                </Text>
              </div>

              <Space direction="vertical" style={{ width: '100%' }} size="small">
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                  <Text strong>Type:</Text>
                  <Tag color={getWalletTypeColor(wallet.wallet_type)}>
                    {wallet.wallet_type}
                  </Tag>
                </div>

                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                  <Text strong>Status:</Text>
                  <Tag color={getStatusColor(wallet.status)}>
                    {wallet.status}
                  </Tag>
                </div>

                {wallet.stamp_type && (
                  <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                    <Text strong>Stamp:</Text>
                    <Tag color="gold">
                      {wallet.stamp_type}
                    </Tag>
                  </div>
                )}

                <div style={{ marginTop: '16px' }}>
                  <Text strong>Balances:</Text>
                  <Row gutter={8} style={{ marginTop: '8px' }}>
                    <Col span={12}>
                      <Statistic
                        title="GEN"
                        value={wallet.balance.gen}
                        precision={2}
                        valueStyle={{ fontSize: '12px', color: '#1890ff' }}
                      />
                    </Col>
                    <Col span={12}>
                      <Statistic
                        title="NEX"
                        value={wallet.balance.nex}
                        precision={2}
                        valueStyle={{ fontSize: '12px', color: '#52c41a' }}
                      />
                    </Col>
                  </Row>
                  <Row gutter={8} style={{ marginTop: '8px' }}>
                    <Col span={12}>
                      <Statistic
                        title="FLX"
                        value={wallet.balance.flx}
                        precision={2}
                        valueStyle={{ fontSize: '12px', color: '#faad14' }}
                      />
                    </Col>
                    <Col span={12}>
                      <Statistic
                        title="AUR"
                        value={wallet.balance.aur}
                        precision={2}
                        valueStyle={{ fontSize: '12px', color: '#722ed1' }}
                      />
                    </Col>
                  </Row>
                </div>

                <div style={{ marginTop: '16px' }}>
                  <div style={{ display: 'flex', justifyContent: 'space-between' }}>
                    <Text type="secondary">Mining Sessions:</Text>
                    <Text strong>{wallet.mining_sessions}</Text>
                  </div>
                  <div style={{ display: 'flex', justifyContent: 'space-between' }}>
                    <Text type="secondary">Total Rewards:</Text>
                    <Text strong style={{ color: '#52c41a' }}>{wallet.total_rewards.toFixed(2)}</Text>
                  </div>
                </div>
              </Space>
            </Card>
          </Col>
        ))}
      </Row>

      {wallets.length === 0 && (
        <div style={{ textAlign: 'center', padding: '50px' }}>
          <WalletOutlined style={{ fontSize: '64px', color: '#d9d9d9', marginBottom: '16px' }} />
          <Title level={4} type="secondary">No Wallets Found</Title>
          <Text type="secondary">Create your first BPCI wallet to get started</Text>
          <div style={{ marginTop: '16px' }}>
            <Button 
              type="primary" 
              icon={<PlusOutlined />}
              onClick={() => setCreateModalVisible(true)}
              size="large"
            >
              Create Wallet
            </Button>
          </div>
        </div>
      )}

      {/* Create Wallet Modal */}
      <Modal
        title="Create New BPCI Wallet"
        open={createModalVisible}
        onCancel={() => setCreateModalVisible(false)}
        footer={null}
        width={500}
      >
        <Form
          form={form}
          layout="vertical"
          onFinish={handleCreateWallet}
        >
          <Form.Item
            name="wallet_type"
            label="Wallet Type"
            rules={[{ required: true, message: 'Please select wallet type' }]}
          >
            <Select placeholder="Select wallet type" size="large">
              <Option value="Normal">Normal Wallet</Option>
              <Option value="Compliance">Compliance Wallet</Option>
              <Option value="Regulated">Regulated Wallet</Option>
              <Option value="Government">Government Wallet</Option>
              <Option value="Emergency">Emergency Wallet</Option>
              <Option value="Bank">Bank Wallet</Option>
              <Option value="Community">Community Wallet</Option>
            </Select>
          </Form.Item>

          <Form.Item
            name="initial_stake"
            label="Initial Stake (Optional)"
          >
            <InputNumber
              placeholder="Enter initial stake amount"
              style={{ width: '100%' }}
              size="large"
              min={0}
              addonAfter="BPCI"
            />
          </Form.Item>

          <Form.Item>
            <Space style={{ width: '100%', justifyContent: 'flex-end' }}>
              <Button onClick={() => setCreateModalVisible(false)}>
                Cancel
              </Button>
              <Button type="primary" htmlType="submit">
                Create Wallet
              </Button>
            </Space>
          </Form.Item>
        </Form>
      </Modal>

      {/* Stamp Wallet Modal */}
      <Modal
        title="Stamp Wallet"
        open={stampModalVisible}
        onCancel={() => {
          setStampModalVisible(false);
          setSelectedWallet(null);
        }}
        footer={null}
        width={400}
      >
        <Form
          form={stampForm}
          layout="vertical"
          onFinish={handleStampWallet}
        >
          <Alert
            message="Wallet Stamping"
            description="Stamping a wallet provides special privileges and access to dedicated APIs based on the stamp type."
            type="info"
            showIcon
            style={{ marginBottom: '16px' }}
          />

          <Form.Item
            name="stamp_type"
            label="Stamp Type"
            rules={[{ required: true, message: 'Please select stamp type' }]}
          >
            <Select placeholder="Select stamp type" size="large">
              <Option value="Government">Government Stamp</Option>
              <Option value="Bank">Bank Stamp</Option>
              <Option value="Community">Community Stamp</Option>
              <Option value="Enterprise">Enterprise Stamp</Option>
            </Select>
          </Form.Item>

          <Form.Item>
            <Space style={{ width: '100%', justifyContent: 'flex-end' }}>
              <Button onClick={() => {
                setStampModalVisible(false);
                setSelectedWallet(null);
              }}>
                Cancel
              </Button>
              <Button type="primary" htmlType="submit">
                Apply Stamp
              </Button>
            </Space>
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
};
