import React, { useState } from 'react';
import {
  Card,
  Row,
  Col,
  Button,
  Typography,
  Space,
  Form,
  Input,
  Switch,
  Divider,
  Alert,
  List,
  Tag,
  Modal
} from 'antd';
import {
  SettingOutlined,
  SafetyOutlined,
  KeyOutlined,
  LockOutlined,
  BellOutlined,
  DeleteOutlined,
  ExportOutlined,
  EyeInvisibleOutlined,
  EyeOutlined
} from '@ant-design/icons';

const { Title, Text, Paragraph } = Typography;

const WalletSettings: React.FC = () => {
  const [form] = Form.useForm();
  const [twoFactorEnabled, setTwoFactorEnabled] = useState(false);
  const [showPrivateKey, setShowPrivateKey] = useState(false);

  const handleSaveSettings = (values: any) => {
    console.log('Saving settings:', values);
  };

  const handleExportWallet = () => {
    Modal.confirm({
      title: 'Export Wallet',
      content: 'Are you sure you want to export your wallet? Keep the exported file secure.',
      onOk: () => {
        console.log('Exporting wallet...');
      }
    });
  };

  return (
    <div style={{ padding: '24px', maxWidth: '1200px', margin: '0 auto' }}>
      <div style={{ marginBottom: 24 }}>
        <Space>
          <SettingOutlined style={{ fontSize: '32px', color: '#1890ff' }} />
          <div>
            <Title level={2} style={{ margin: 0 }}>Wallet Settings</Title>
            <Text type="secondary">Manage your wallet security and preferences</Text>
          </div>
        </Space>
      </div>

      <Row gutter={[16, 16]}>
        {/* Security Settings */}
        <Col xs={24} lg={12}>
          <Card title={<Space><SafetyOutlined />Security Settings</Space>}>
            <Form form={form} layout="vertical" onFinish={handleSaveSettings}>
              <Form.Item label="Two-Factor Authentication">
                <Space style={{ width: '100%', justifyContent: 'space-between' }}>
                  <Text>Enable 2FA for additional security</Text>
                  <Switch checked={twoFactorEnabled} onChange={setTwoFactorEnabled} />
                </Space>
              </Form.Item>

              <Form.Item label="Auto-Lock Timeout" name="autoLockTimeout">
                <Input suffix="minutes" defaultValue="30" />
              </Form.Item>

              <Form.Item label="Change Password" name="newPassword">
                <Input.Password placeholder="Enter new password" />
              </Form.Item>

              <Form.Item>
                <Button type="primary" htmlType="submit" block>
                  Save Security Settings
                </Button>
              </Form.Item>
            </Form>
          </Card>
        </Col>

        {/* Wallet Information */}
        <Col xs={24} lg={12}>
          <Card title={<Space><KeyOutlined />Wallet Information</Space>}>
            <Space direction="vertical" style={{ width: '100%' }} size="middle">
              <div>
                <Text strong>BPI Address:</Text>
                <Paragraph copyable code style={{ marginTop: 8 }}>
                  0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb
                </Paragraph>
              </div>

              <div>
                <Text strong>Private Key:</Text>
                <Space style={{ marginTop: 8, width: '100%', justifyContent: 'space-between' }}>
                  <Text code>
                    {showPrivateKey ? '0x1234...5678' : '••••••••••••••••'}
                  </Text>
                  <Button
                    size="small"
                    icon={showPrivateKey ? <EyeInvisibleOutlined /> : <EyeOutlined />}
                    onClick={() => setShowPrivateKey(!showPrivateKey)}
                  >
                    {showPrivateKey ? 'Hide' : 'Show'}
                  </Button>
                </Space>
              </div>

              <Alert
                message="Keep your private key secure"
                description="Never share your private key with anyone. Store it in a safe place."
                type="warning"
                showIcon
              />

              <Button icon={<ExportOutlined />} onClick={handleExportWallet} block>
                Export Wallet
              </Button>
            </Space>
          </Card>
        </Col>

        {/* Notifications */}
        <Col xs={24}>
          <Card title={<Space><BellOutlined />Notification Preferences</Space>}>
            <List>
              <List.Item>
                <Space style={{ width: '100%', justifyContent: 'space-between' }}>
                  <div>
                    <Text strong>Transaction Notifications</Text>
                    <br />
                    <Text type="secondary" style={{ fontSize: '12px' }}>
                      Receive notifications for incoming/outgoing transactions
                    </Text>
                  </div>
                  <Switch defaultChecked />
                </Space>
              </List.Item>
              <List.Item>
                <Space style={{ width: '100%', justifyContent: 'space-between' }}>
                  <div>
                    <Text strong>Node Status Alerts</Text>
                    <br />
                    <Text type="secondary" style={{ fontSize: '12px' }}>
                      Get alerted when your nodes go offline
                    </Text>
                  </div>
                  <Switch defaultChecked />
                </Space>
              </List.Item>
              <List.Item>
                <Space style={{ width: '100%', justifyContent: 'space-between' }}>
                  <div>
                    <Text strong>Security Alerts</Text>
                    <br />
                    <Text type="secondary" style={{ fontSize: '12px' }}>
                      Receive alerts for suspicious activity
                    </Text>
                  </div>
                  <Switch defaultChecked />
                </Space>
              </List.Item>
            </List>
          </Card>
        </Col>

        {/* Danger Zone */}
        <Col xs={24}>
          <Card title={<Space><DeleteOutlined />Danger Zone</Space>} style={{ borderColor: '#ff4d4f' }}>
            <Alert
              message="Irreversible Actions"
              description="These actions cannot be undone. Proceed with caution."
              type="error"
              showIcon
              style={{ marginBottom: 16 }}
            />
            <Space direction="vertical" style={{ width: '100%' }}>
              <Button danger block>
                Delete Wallet
              </Button>
              <Button danger block>
                Revoke All Sessions
              </Button>
            </Space>
          </Card>
        </Col>
      </Row>
    </div>
  );
};

export default WalletSettings;
