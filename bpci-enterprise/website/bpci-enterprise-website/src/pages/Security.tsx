import React, { useState, useEffect } from 'react';
import { Card, Typography, Button, Table, Tag, Space, Alert, Modal, Form, Input, message, Divider } from 'antd';
import {
  SafetyOutlined,
  LockOutlined,
  EnvironmentOutlined,
  ClockCircleOutlined,
  WarningOutlined,
  CheckCircleOutlined,
  CloseCircleOutlined,
  SecurityScanOutlined,
  KeyOutlined,
  DeleteOutlined,
  ReloadOutlined
} from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { authService } from '../services/authService';
import axios from 'axios';

const { Title, Text } = Typography;

const INSTALLER_API = 'http://localhost:8080/api';

interface LoginHistory {
  id: string;
  timestamp: string;
  ip_address: string;
  location: string;
  device: string;
  browser: string;
  status: 'success' | 'failed';
}

interface ActiveSession {
  id: string;
  device: string;
  browser: string;
  ip_address: string;
  location: string;
  last_active: string;
  is_current: boolean;
}

interface SecurityEvent {
  id: string;
  type: 'password_change' | 'login_failed' | '2fa_enabled' | 'session_terminated';
  description: string;
  timestamp: string;
  severity: 'low' | 'medium' | 'high';
}

const Security: React.FC = () => {
  const [loading, setLoading] = useState(false);
  const [loginHistory, setLoginHistory] = useState<LoginHistory[]>([]);
  const [activeSessions, setActiveSessions] = useState<ActiveSession[]>([]);
  const [securityEvents, setSecurityEvents] = useState<SecurityEvent[]>([]);
  const [terminateModalVisible, setTerminateModalVisible] = useState(false);
  const [selectedSession, setSelectedSession] = useState<string | null>(null);

  useEffect(() => {
    loadSecurityData();
  }, []);

  const loadSecurityData = async () => {
    setLoading(true);
    try {
      // Load login history
      await loadLoginHistory();
      
      // Load active sessions
      await loadActiveSessions();
      
      // Load security events
      await loadSecurityEvents();
    } catch (error) {
      console.error('Failed to load security data:', error);
    } finally {
      setLoading(false);
    }
  };

  const loadLoginHistory = async () => {
    try {
      // TODO: Replace with real API endpoint when backend is ready
      const response = await axios.get(`${INSTALLER_API}/security/login-history`);
      if (response.data.success) {
        setLoginHistory(response.data.data);
      }
    } catch (error) {
      // Mock data for development
      setLoginHistory([
        {
          id: '1',
          timestamp: new Date().toISOString(),
          ip_address: '192.168.1.100',
          location: 'San Francisco, CA',
          device: 'Desktop',
          browser: 'Chrome 120',
          status: 'success'
        },
        {
          id: '2',
          timestamp: new Date(Date.now() - 86400000).toISOString(),
          ip_address: '192.168.1.101',
          location: 'San Francisco, CA',
          device: 'Mobile',
          browser: 'Safari 17',
          status: 'success'
        }
      ]);
    }
  };

  const loadActiveSessions = async () => {
    try {
      // TODO: Replace with real API endpoint when backend is ready
      const response = await axios.get(`${INSTALLER_API}/security/sessions`);
      if (response.data.success) {
        setActiveSessions(response.data.data);
      }
    } catch (error) {
      // Mock data for development
      setActiveSessions([
        {
          id: '1',
          device: 'Desktop - Windows',
          browser: 'Chrome 120',
          ip_address: '192.168.1.100',
          location: 'San Francisco, CA',
          last_active: new Date().toISOString(),
          is_current: true
        }
      ]);
    }
  };

  const loadSecurityEvents = async () => {
    try {
      // TODO: Replace with real API endpoint when backend is ready
      const response = await axios.get(`${INSTALLER_API}/security/events`);
      if (response.data.success) {
        setSecurityEvents(response.data.data);
      }
    } catch (error) {
      // Mock data for development
      setSecurityEvents([
        {
          id: '1',
          type: 'password_change',
          description: 'Password changed successfully',
          timestamp: new Date(Date.now() - 172800000).toISOString(),
          severity: 'low'
        }
      ]);
    }
  };

  const handleTerminateSession = async (sessionId: string) => {
    setLoading(true);
    try {
      // TODO: Replace with real API endpoint when backend is ready
      const response = await axios.post(`${INSTALLER_API}/security/sessions/${sessionId}/terminate`);
      
      if (response.data.success) {
        message.success('Session terminated successfully');
        loadActiveSessions();
      }
    } catch (error: any) {
      message.error(error.response?.data?.message || 'Failed to terminate session');
    } finally {
      setLoading(false);
      setTerminateModalVisible(false);
      setSelectedSession(null);
    }
  };

  const handleTerminateAllSessions = async () => {
    Modal.confirm({
      title: 'Terminate All Sessions',
      content: 'Are you sure you want to terminate all other sessions? You will remain logged in on this device.',
      okText: 'Yes, Terminate All',
      okType: 'danger',
      cancelText: 'Cancel',
      onOk: async () => {
        setLoading(true);
        try {
          // TODO: Replace with real API endpoint when backend is ready
          const response = await axios.post(`${INSTALLER_API}/security/sessions/terminate-all`);
          
          if (response.data.success) {
            message.success('All other sessions terminated successfully');
            loadActiveSessions();
          }
        } catch (error: any) {
          message.error(error.response?.data?.message || 'Failed to terminate sessions');
        } finally {
          setLoading(false);
        }
      }
    });
  };

  const loginHistoryColumns: ColumnsType<LoginHistory> = [
    {
      title: 'Date & Time',
      dataIndex: 'timestamp',
      key: 'timestamp',
      render: (timestamp: string) => (
        <Space>
          <ClockCircleOutlined style={{ color: '#E8B44F' }} />
          <Text style={{ color: '#ffffff' }}>
            {new Date(timestamp).toLocaleString()}
          </Text>
        </Space>
      )
    },
    {
      title: 'Location',
      dataIndex: 'location',
      key: 'location',
      render: (location: string) => (
        <Space>
          <EnvironmentOutlined style={{ color: '#3B82F6' }} />
          <Text style={{ color: '#ffffff' }}>{location}</Text>
        </Space>
      )
    },
    {
      title: 'Device & Browser',
      key: 'device',
      render: (record: LoginHistory) => (
        <Text style={{ color: '#ffffff' }}>
          {record.device} - {record.browser}
        </Text>
      )
    },
    {
      title: 'IP Address',
      dataIndex: 'ip_address',
      key: 'ip_address',
      render: (ip: string) => (
        <Text style={{ color: '#9CA3AF', fontFamily: 'monospace' }}>{ip}</Text>
      )
    },
    {
      title: 'Status',
      dataIndex: 'status',
      key: 'status',
      render: (status: string) => (
        <Tag color={status === 'success' ? 'success' : 'error'} icon={status === 'success' ? <CheckCircleOutlined /> : <CloseCircleOutlined />}>
          {status.toUpperCase()}
        </Tag>
      )
    }
  ];

  const activeSessionsColumns: ColumnsType<ActiveSession> = [
    {
      title: 'Device & Browser',
      key: 'device',
      render: (record: ActiveSession) => (
        <Space>
          <Text style={{ color: '#ffffff', fontWeight: record.is_current ? 'bold' : 'normal' }}>
            {record.device} - {record.browser}
          </Text>
          {record.is_current && <Tag color="gold">Current Session</Tag>}
        </Space>
      )
    },
    {
      title: 'Location',
      dataIndex: 'location',
      key: 'location',
      render: (location: string) => (
        <Space>
          <EnvironmentOutlined style={{ color: '#3B82F6' }} />
          <Text style={{ color: '#ffffff' }}>{location}</Text>
        </Space>
      )
    },
    {
      title: 'IP Address',
      dataIndex: 'ip_address',
      key: 'ip_address',
      render: (ip: string) => (
        <Text style={{ color: '#9CA3AF', fontFamily: 'monospace' }}>{ip}</Text>
      )
    },
    {
      title: 'Last Active',
      dataIndex: 'last_active',
      key: 'last_active',
      render: (timestamp: string) => (
        <Text style={{ color: '#9CA3AF' }}>
          {new Date(timestamp).toLocaleString()}
        </Text>
      )
    },
    {
      title: 'Action',
      key: 'action',
      render: (record: ActiveSession) => (
        !record.is_current && (
          <Button
            danger
            size="small"
            icon={<DeleteOutlined />}
            onClick={() => {
              setSelectedSession(record.id);
              setTerminateModalVisible(true);
            }}
          >
            Terminate
          </Button>
        )
      )
    }
  ];

  const getSeverityColor = (severity: string) => {
    switch (severity) {
      case 'high': return 'error';
      case 'medium': return 'warning';
      case 'low': return 'success';
      default: return 'default';
    }
  };

  const getEventIcon = (type: string) => {
    switch (type) {
      case 'password_change': return <KeyOutlined />;
      case 'login_failed': return <WarningOutlined />;
      case '2fa_enabled': return <SecurityScanOutlined />;
      case 'session_terminated': return <CloseCircleOutlined />;
      default: return <SafetyOutlined />;
    }
  };

  return (
    <div style={{ padding: '1.5rem', maxWidth: '1400px', margin: '0 auto' }}>
      {/* Header */}
      <div style={{ marginBottom: '2rem', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <div>
          <Title level={2} style={{ color: '#E8B44F', marginBottom: '0.5rem' }}>
            <SafetyOutlined /> Security Center
          </Title>
          <Text style={{ color: '#9CA3AF' }}>
            Monitor your account security and manage active sessions
          </Text>
        </div>
        <Button
          icon={<ReloadOutlined />}
          onClick={loadSecurityData}
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

      {/* Security Overview */}
      <Alert
        message="Account Security Status"
        description="Your account is protected with standard security measures. Enable two-factor authentication for enhanced security."
        type="info"
        showIcon
        icon={<SafetyOutlined />}
        style={{
          marginBottom: '2rem',
          background: 'rgba(59, 130, 246, 0.1)',
          border: '1px solid rgba(59, 130, 246, 0.3)'
        }}
      />

      {/* Active Sessions */}
      <Card
        title={
          <span style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>
            <LockOutlined /> Active Sessions
          </span>
        }
        extra={
          activeSessions.length > 1 && (
            <Button
              danger
              size="small"
              icon={<DeleteOutlined />}
              onClick={handleTerminateAllSessions}
            >
              Terminate All Others
            </Button>
          )
        }
        style={{
          background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
          border: '1px solid rgba(232, 180, 79, 0.2)',
          borderRadius: '12px',
          marginBottom: '2rem'
        }}
      >
        <Table
          columns={activeSessionsColumns}
          dataSource={activeSessions}
          rowKey="id"
          pagination={false}
          loading={loading}
          style={{ background: 'transparent' }}
        />
      </Card>

      {/* Login History */}
      <Card
        title={
          <span style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>
            <ClockCircleOutlined /> Login History
          </span>
        }
        style={{
          background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
          border: '1px solid rgba(232, 180, 79, 0.2)',
          borderRadius: '12px',
          marginBottom: '2rem'
        }}
      >
        <Table
          columns={loginHistoryColumns}
          dataSource={loginHistory}
          rowKey="id"
          pagination={{ pageSize: 10 }}
          loading={loading}
          style={{ background: 'transparent' }}
        />
      </Card>

      {/* Security Events */}
      <Card
        title={
          <span style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>
            <SecurityScanOutlined /> Recent Security Events
          </span>
        }
        style={{
          background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
          border: '1px solid rgba(232, 180, 79, 0.2)',
          borderRadius: '12px'
        }}
      >
        <Space direction="vertical" size="middle" style={{ width: '100%' }}>
          {securityEvents.length > 0 ? (
            securityEvents.map(event => (
              <div
                key={event.id}
                style={{
                  padding: '1rem',
                  background: 'rgba(255, 255, 255, 0.05)',
                  borderRadius: '8px',
                  border: '1px solid rgba(232, 180, 79, 0.1)'
                }}
              >
                <Space style={{ width: '100%', justifyContent: 'space-between' }}>
                  <Space>
                    {getEventIcon(event.type)}
                    <div>
                      <Text style={{ color: '#ffffff', display: 'block' }}>
                        {event.description}
                      </Text>
                      <Text style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>
                        {new Date(event.timestamp).toLocaleString()}
                      </Text>
                    </div>
                  </Space>
                  <Tag color={getSeverityColor(event.severity)}>
                    {event.severity.toUpperCase()}
                  </Tag>
                </Space>
              </div>
            ))
          ) : (
            <Text style={{ color: '#9CA3AF' }}>No recent security events</Text>
          )}
        </Space>
      </Card>

      {/* Terminate Session Modal */}
      <Modal
        title="Terminate Session"
        open={terminateModalVisible}
        onOk={() => selectedSession && handleTerminateSession(selectedSession)}
        onCancel={() => {
          setTerminateModalVisible(false);
          setSelectedSession(null);
        }}
        okText="Terminate"
        okButtonProps={{ danger: true }}
        cancelText="Cancel"
      >
        <p>Are you sure you want to terminate this session? The user will be logged out immediately.</p>
      </Modal>
    </div>
  );
};

export default Security;
