import React, { useState } from 'react';
import { Form, Button, Card, Alert, Typography, Space, Select, Radio, Divider } from 'antd';
import { WalletOutlined, BankOutlined, SafetyOutlined, CrownOutlined } from '@ant-design/icons';
import type { WalletActivationRequest } from '../../services/authService';
import { authService } from '../../services/authService';

const { Title, Text } = Typography;
const { Option } = Select;

interface WalletActivationProps {
  developerId: string;
  onActivationSuccess: () => void;
  onBack: () => void;
}

const WalletActivation: React.FC<WalletActivationProps> = ({ 
  developerId, 
  onActivationSuccess, 
  onBack 
}) => {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [selectedWalletType, setSelectedWalletType] = useState<string>('Community');

  const onFinish = async (values: any) => {
    setLoading(true);
    setError(null);

    try {
      const walletData: WalletActivationRequest = {
        developer_id: developerId,
        wallet_type: values.wallet_type,
        owner_type: values.owner_type,
        network_type: values.network_type,
        stamp_type: values.stamp_type
      };

      const response = await authService.activateWallet(walletData);
      
      if (response.success) {
        onActivationSuccess();
      } else {
        setError(response.message);
      }
    } catch (error) {
      setError('Wallet activation failed. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const walletTypeOptions = [
    { value: 'Community', label: 'Community', icon: <SafetyOutlined />, description: 'Standard community wallet for general use' },
    { value: 'Investor', label: 'Investor', icon: <BankOutlined />, description: 'Investment-focused wallet with enhanced features' },
    { value: 'Owner', label: 'Owner', icon: <CrownOutlined />, description: 'Company owner wallet (requires owner type selection)' },
    { value: 'ESOP', label: 'ESOP', icon: <SafetyOutlined />, description: 'Employee Stock Ownership Plan wallet' },
    { value: 'Company', label: 'Company', icon: <BankOutlined />, description: 'Corporate operational wallet' }
  ];

  const ownerTypeOptions = [
    { value: 1, label: 'Founder', description: 'Company founders/core team (600 mother coins)' },
    { value: 2, label: 'Early Investor', description: 'Early investors (100 coins each)' },
    { value: 3, label: 'Community Leader', description: 'Community leaders (variable allocation)' },
    { value: 4, label: 'Strategic Partner', description: 'Strategic partners (negotiated allocation)' },
    { value: 5, label: 'Public Investor', description: 'Public investors (market-based allocation)' }
  ];

  return (
    <Card 
      style={{ maxWidth: 500, margin: '0 auto' }}
      title={
        <Space>
          <WalletOutlined />
          <Title level={3} style={{ margin: 0 }}>Activate BPCI Wallet</Title>
        </Space>
      }
    >
      <div style={{ marginBottom: 16, textAlign: 'center' }}>
        <Text type="secondary">
          Activate your BPCI wallet to generate address, tokens, and access BPI features.
        </Text>
      </div>

      {error && (
        <Alert
          message={error}
          type="error"
          style={{ marginBottom: 16 }}
          closable
          onClose={() => setError(null)}
        />
      )}

      <Form
        name="walletActivation"
        onFinish={onFinish}
        layout="vertical"
        size="large"
        initialValues={{
          wallet_type: 'Community',
          network_type: 'Testnet'
        }}
      >
        <Form.Item
          label="Wallet Type"
          name="wallet_type"
          rules={[{ required: true, message: 'Please select a wallet type!' }]}
        >
          <Select 
            placeholder="Select wallet type"
            onChange={setSelectedWalletType}
          >
            {walletTypeOptions.map(option => (
              <Option key={option.value} value={option.value}>
                <Space>
                  {option.icon}
                  <div>
                    <div>{option.label}</div>
                    <Text type="secondary" style={{ fontSize: '12px' }}>
                      {option.description}
                    </Text>
                  </div>
                </Space>
              </Option>
            ))}
          </Select>
        </Form.Item>

        {selectedWalletType === 'Owner' && (
          <Form.Item
            label="Owner Type"
            name="owner_type"
            rules={[{ required: true, message: 'Please select owner type!' }]}
          >
            <Select placeholder="Select owner type">
              {ownerTypeOptions.map(option => (
                <Option key={option.value} value={option.value}>
                  <div>
                    <div>Type {option.value}: {option.label}</div>
                    <Text type="secondary" style={{ fontSize: '12px' }}>
                      {option.description}
                    </Text>
                  </div>
                </Option>
              ))}
            </Select>
          </Form.Item>
        )}

        <Form.Item
          label="Network Type"
          name="network_type"
          rules={[{ required: true, message: 'Please select network type!' }]}
        >
          <Radio.Group>
            <Space direction="vertical">
              <Radio value="Testnet">
                <div>
                  <div><strong>Testnet</strong></div>
                  <Text type="secondary" style={{ fontSize: '12px' }}>
                    Free coins, no real billing, development environment
                  </Text>
                </div>
              </Radio>
              <Radio value="Mainnet">
                <div>
                  <div><strong>Mainnet</strong></div>
                  <Text type="secondary" style={{ fontSize: '12px' }}>
                    Real billing ($1/BPI default), production environment
                  </Text>
                </div>
              </Radio>
            </Space>
          </Radio.Group>
        </Form.Item>

        <Form.Item
          label="Stamp Type (Optional)"
          name="stamp_type"
        >
          <Select placeholder="Select stamp type for special access">
            <Option value="Community">Community Stamp</Option>
            <Option value="Enterprise">Enterprise Stamp</Option>
            <Option value="Government">Government Stamp (Regulatory)</Option>
            <Option value="Bank">Bank Stamp (Financial)</Option>
          </Select>
        </Form.Item>

        <Divider />

        <div style={{ marginBottom: 16 }}>
          <Alert
            message="Wallet Activation"
            description="After activation, you'll receive a unique registration ID, wallet address, and token allocation based on your selections."
            type="info"
            showIcon
          />
        </div>

        <Form.Item>
          <Space style={{ width: '100%', justifyContent: 'space-between' }}>
            <Button onClick={onBack}>
              Back to Profile
            </Button>
            <Button 
              type="primary" 
              htmlType="submit" 
              loading={loading}
              icon={<WalletOutlined />}
            >
              Activate Wallet
            </Button>
          </Space>
        </Form.Item>
      </Form>
    </Card>
  );
};

export default WalletActivation;
