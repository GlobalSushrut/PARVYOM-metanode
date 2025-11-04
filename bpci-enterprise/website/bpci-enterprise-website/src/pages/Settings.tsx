import React, { useState } from 'react';
import { Card, Typography, Form, Input, Button, Switch, message, Space, Divider, Alert, Tabs } from 'antd';
import {
  SettingOutlined,
  LockOutlined,
  BellOutlined,
  SafetyOutlined,
  KeyOutlined,
  SecurityScanOutlined,
  MailOutlined,
  MobileOutlined
} from '@ant-design/icons';
import { authService } from '../services/authService';
import axios from 'axios';

const { Title, Text } = Typography;
const { TabPane } = Tabs;

const INSTALLER_API = 'http://localhost:8080/api';

const Settings: React.FC = () => {
  const [passwordForm] = Form.useForm();
  const [twoFactorForm] = Form.useForm();
  const [loading, setLoading] = useState(false);
  const [notifications, setNotifications] = useState({
    email: true,
    push: false,
    sms: false,
    transactions: true,
    security: true,
    updates: true
  });

  const handleChangePassword = async (values: any) => {
    setLoading(true);
    try {
      // TODO: Replace with real API endpoint when backend is ready
      const response = await axios.post(`${INSTALLER_API}/auth/change-password`, {
        current_password: values.current_password,
        new_password: values.new_password
      });

      if (response.data.success) {
        message.success('Password changed successfully!');
        passwordForm.resetFields();
      }
    } catch (error: any) {
      message.error(error.response?.data?.message || 'Failed to change password. Make sure the backend is running.');
    } finally {
      setLoading(false);
    }
  };

  const handleEnable2FA = async (values: any) => {
    setLoading(true);
    try {
      // TODO: Replace with real API endpoint when backend is ready
      const response = await axios.post(`${INSTALLER_API}/auth/enable-2fa`, {
        method: values.method,
        contact: values.contact
      });

      if (response.data.success) {
        message.success('Two-factor authentication enabled!');
        twoFactorForm.resetFields();
      }
    } catch (error: any) {
      message.error(error.response?.data?.message || 'Failed to enable 2FA. Make sure the backend is running.');
    } finally {
      setLoading(false);
    }
  };

  const handleNotificationChange = (key: string, value: boolean) => {
    setNotifications(prev => ({
      ...prev,
      [key]: value
    }));
    message.success(`${key} notifications ${value ? 'enabled' : 'disabled'}`);
  };

  return (
    <div style={{ padding: '1.5rem', maxWidth: '1200px', margin: '0 auto' }}>
      {/* Header */}
      <div style={{ marginBottom: '2rem' }}>
        <Title level={2} style={{ color: '#E8B44F', marginBottom: '0.5rem' }}>
          <SettingOutlined /> Settings
        </Title>
        <Text style={{ color: '#9CA3AF' }}>
          Manage your account security and preferences
        </Text>
      </div>

      <Tabs
        defaultActiveKey="security"
        size="large"
        tabBarStyle={{
          borderBottom: '1px solid rgba(232, 180, 79, 0.2)',
          marginBottom: '2rem'
        }}
      >
        {/* Security Tab */}
        <TabPane
          tab={
            <span style={{ color: '#E8B44F' }}>
              <SafetyOutlined /> Security
            </span>
          }
          key="security"
        >
          <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '1.5rem' }}>
            {/* Change Password */}
            <Card
              title={
                <span style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>
                  <LockOutlined /> Change Password
                </span>
              }
              style={{
                background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
                border: '1px solid rgba(232, 180, 79, 0.2)',
                borderRadius: '12px'
              }}
            >
              <Form
                form={passwordForm}
                layout="vertical"
                onFinish={handleChangePassword}
              >
                <Form.Item
                  label={<span style={{ color: '#E8B44F', fontWeight: '600' }}>Current Password</span>}
                  name="current_password"
                  rules={[{ required: true, message: 'Please enter your current password' }]}
                >
                  <Input.Password
                    prefix={<LockOutlined style={{ color: '#E8B44F' }} />}
                    placeholder="Enter current password"
                    size="large"
                    style={{
                      background: 'rgba(255, 255, 255, 0.05)',
                      border: '1px solid rgba(232, 180, 79, 0.2)',
                      color: '#ffffff'
                    }}
                  />
                </Form.Item>

                <Form.Item
                  label={<span style={{ color: '#E8B44F', fontWeight: '600' }}>New Password</span>}
                  name="new_password"
                  rules={[
                    { required: true, message: 'Please enter new password' },
                    { min: 8, message: 'Password must be at least 8 characters' }
                  ]}
                >
                  <Input.Password
                    prefix={<KeyOutlined style={{ color: '#E8B44F' }} />}
                    placeholder="Enter new password"
                    size="large"
                    style={{
                      background: 'rgba(255, 255, 255, 0.05)',
                      border: '1px solid rgba(232, 180, 79, 0.2)',
                      color: '#ffffff'
                    }}
                  />
                </Form.Item>

                <Form.Item
                  label={<span style={{ color: '#E8B44F', fontWeight: '600' }}>Confirm New Password</span>}
                  name="confirm_password"
                  dependencies={['new_password']}
                  rules={[
                    { required: true, message: 'Please confirm your password' },
                    ({ getFieldValue }) => ({
                      validator(_, value) {
                        if (!value || getFieldValue('new_password') === value) {
                          return Promise.resolve();
                        }
                        return Promise.reject(new Error('Passwords do not match'));
                      },
                    }),
                  ]}
                >
                  <Input.Password
                    prefix={<KeyOutlined style={{ color: '#E8B44F' }} />}
                    placeholder="Confirm new password"
                    size="large"
                    style={{
                      background: 'rgba(255, 255, 255, 0.05)',
                      border: '1px solid rgba(232, 180, 79, 0.2)',
                      color: '#ffffff'
                    }}
                  />
                </Form.Item>

                <Form.Item>
                  <Button
                    type="primary"
                    htmlType="submit"
                    size="large"
                    loading={loading}
                    icon={<LockOutlined />}
                    style={{
                      background: 'linear-gradient(135deg, #E8B44F 0%, #D4A044 100%)',
                      border: 'none',
                      fontWeight: '600',
                      width: '100%'
                    }}
                  >
                    Change Password
                  </Button>
                </Form.Item>
              </Form>
            </Card>

            {/* Two-Factor Authentication */}
            <Card
              title={
                <span style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>
                  <SecurityScanOutlined /> Two-Factor Authentication
                </span>
              }
              style={{
                background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
                border: '1px solid rgba(232, 180, 79, 0.2)',
                borderRadius: '12px'
              }}
            >
              <Alert
                message="Enhanced Security"
                description="Add an extra layer of security to your account by enabling two-factor authentication."
                type="info"
                showIcon
                style={{
                  marginBottom: '1.5rem',
                  background: 'rgba(59, 130, 246, 0.1)',
                  border: '1px solid rgba(59, 130, 246, 0.3)'
                }}
              />

              <Form
                form={twoFactorForm}
                layout="vertical"
                onFinish={handleEnable2FA}
              >
                <Form.Item
                  label={<span style={{ color: '#E8B44F', fontWeight: '600' }}>Authentication Method</span>}
                  name="method"
                  rules={[{ required: true, message: 'Please select a method' }]}
                >
                  <Input
                    placeholder="Email or SMS"
                    size="large"
                    style={{
                      background: 'rgba(255, 255, 255, 0.05)',
                      border: '1px solid rgba(232, 180, 79, 0.2)',
                      color: '#ffffff'
                    }}
                  />
                </Form.Item>

                <Form.Item
                  label={<span style={{ color: '#E8B44F', fontWeight: '600' }}>Contact</span>}
                  name="contact"
                  rules={[{ required: true, message: 'Please enter your contact' }]}
                >
                  <Input
                    placeholder="Email or phone number"
                    size="large"
                    style={{
                      background: 'rgba(255, 255, 255, 0.05)',
                      border: '1px solid rgba(232, 180, 79, 0.2)',
                      color: '#ffffff'
                    }}
                  />
                </Form.Item>

                <Form.Item>
                  <Button
                    type="primary"
                    htmlType="submit"
                    size="large"
                    loading={loading}
                    icon={<SecurityScanOutlined />}
                    style={{
                      background: 'linear-gradient(135deg, #10B981 0%, #059669 100%)',
                      border: 'none',
                      fontWeight: '600',
                      width: '100%'
                    }}
                  >
                    Enable 2FA
                  </Button>
                </Form.Item>
              </Form>
            </Card>
          </div>
        </TabPane>

        {/* Notifications Tab */}
        <TabPane
          tab={
            <span style={{ color: '#E8B44F' }}>
              <BellOutlined /> Notifications
            </span>
          }
          key="notifications"
        >
          <Card
            style={{
              background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
              border: '1px solid rgba(232, 180, 79, 0.2)',
              borderRadius: '12px',
              maxWidth: '800px'
            }}
          >
            <Title level={4} style={{ color: '#E8B44F', marginBottom: '1.5rem' }}>
              Notification Preferences
            </Title>

            <Space direction="vertical" size="large" style={{ width: '100%' }}>
              {/* Notification Channels */}
              <div>
                <Text style={{ color: '#E8B44F', fontSize: '1rem', fontWeight: '600', display: 'block', marginBottom: '1rem' }}>
                  Notification Channels
                </Text>
                
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '1rem' }}>
                  <Space>
                    <MailOutlined style={{ color: '#E8B44F', fontSize: '1.25rem' }} />
                    <div>
                      <Text style={{ color: '#ffffff', display: 'block' }}>Email Notifications</Text>
                      <Text style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>Receive notifications via email</Text>
                    </div>
                  </Space>
                  <Switch
                    checked={notifications.email}
                    onChange={(checked) => handleNotificationChange('email', checked)}
                  />
                </div>

                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '1rem' }}>
                  <Space>
                    <BellOutlined style={{ color: '#E8B44F', fontSize: '1.25rem' }} />
                    <div>
                      <Text style={{ color: '#ffffff', display: 'block' }}>Push Notifications</Text>
                      <Text style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>Receive push notifications in browser</Text>
                    </div>
                  </Space>
                  <Switch
                    checked={notifications.push}
                    onChange={(checked) => handleNotificationChange('push', checked)}
                  />
                </div>

                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                  <Space>
                    <MobileOutlined style={{ color: '#E8B44F', fontSize: '1.25rem' }} />
                    <div>
                      <Text style={{ color: '#ffffff', display: 'block' }}>SMS Notifications</Text>
                      <Text style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>Receive notifications via SMS</Text>
                    </div>
                  </Space>
                  <Switch
                    checked={notifications.sms}
                    onChange={(checked) => handleNotificationChange('sms', checked)}
                  />
                </div>
              </div>

              <Divider style={{ borderColor: 'rgba(232, 180, 79, 0.2)' }} />

              {/* Notification Types */}
              <div>
                <Text style={{ color: '#E8B44F', fontSize: '1rem', fontWeight: '600', display: 'block', marginBottom: '1rem' }}>
                  Notification Types
                </Text>
                
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '1rem' }}>
                  <div>
                    <Text style={{ color: '#ffffff', display: 'block' }}>Transaction Alerts</Text>
                    <Text style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>Get notified about wallet transactions</Text>
                  </div>
                  <Switch
                    checked={notifications.transactions}
                    onChange={(checked) => handleNotificationChange('transactions', checked)}
                  />
                </div>

                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '1rem' }}>
                  <div>
                    <Text style={{ color: '#ffffff', display: 'block' }}>Security Alerts</Text>
                    <Text style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>Important security notifications</Text>
                  </div>
                  <Switch
                    checked={notifications.security}
                    onChange={(checked) => handleNotificationChange('security', checked)}
                  />
                </div>

                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                  <div>
                    <Text style={{ color: '#ffffff', display: 'block' }}>System Updates</Text>
                    <Text style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>Updates about new features and changes</Text>
                  </div>
                  <Switch
                    checked={notifications.updates}
                    onChange={(checked) => handleNotificationChange('updates', checked)}
                  />
                </div>
              </div>
            </Space>
          </Card>
        </TabPane>
      </Tabs>
    </div>
  );
};

export default Settings;
