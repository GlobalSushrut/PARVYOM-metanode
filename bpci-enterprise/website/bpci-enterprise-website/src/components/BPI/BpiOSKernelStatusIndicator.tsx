/**
 * BPI OS Kernel Status Indicator Component
 * Real-time status monitoring for BPI Blockchain Operating System kernel
 */

import React, { useState, useEffect, useRef } from 'react';
import {
  Card,
  Badge,
  Space,
  Typography,
  Statistic,
  Progress,
  Tag,
  Tooltip,
  Button,
  Row,
  Col,
  Alert,
  Spin,
  List,
  Avatar,
  Divider,
} from 'antd';
import {
  CheckCircleOutlined,
  ExclamationCircleOutlined,
  ClockCircleOutlined,
  ReloadOutlined,
  DatabaseOutlined,
  ThunderboltOutlined,
  SecurityScanOutlined,
  NodeIndexOutlined,
  CloudServerOutlined,
  GlobalOutlined,
  HddOutlined,
  ApiOutlined,
  MonitorOutlined,
  LoadingOutlined,
  CloseCircleOutlined,
  WarningOutlined,
} from '@ant-design/icons';

const { Text, Title } = Typography;

// BPI OS Kernel status interface
export interface BpiOSKernelStatus {
  // Connection Status
  connectionStatus: 'disconnected' | 'connecting' | 'connected' | 'authenticated' | 'active' | 'error';
  connectionMessage: string;
  connectedAt?: Date;
  lastHeartbeat?: Date;
  
  // Kernel Information
  kernelInfo?: {
    kernelId: string;
    version: string;
    osVersion: string;
    bridgeId: string;
    uptime: number;
  };
  
  // Process Status
  processStatus?: {
    totalProcesses: number;
    activeProcesses: number;
    failedProcesses: number;
    processTypes: Record<string, number>;
  };
  
  // Resource Utilization
  resourceUtilization?: {
    cpuUsage: number;
    memoryUsage: number;
    networkUsage: number;
    storageUsage: number;
    gpuUsage: number;
  };
  
  // Service Mapper Status
  serviceMapperStatus?: {
    activeMappings: number;
    failedMappings: number;
    mappingEfficiency: number;
    autoScalingActive: boolean;
  };
  
  // Performance Metrics
  performanceMetrics?: {
    kernelLatency: number;
    processThroughput: number;
    resourceEfficiency: number;
    errorRate: number;
    healthScore: number;
  };
  
  // Security Status
  securityStatus?: {
    securityLevel: 'basic' | 'standard' | 'enhanced' | 'maximum';
    encryptionActive: boolean;
    auditActive: boolean;
    threatLevel: 'low' | 'medium' | 'high' | 'critical';
  };
}

interface BpiOSKernelStatusIndicatorProps {
  refreshInterval?: number;
  showDetailedMetrics?: boolean;
  onStatusChange?: (status: BpiOSKernelStatus) => void;
}

export const BpiOSKernelStatusIndicator: React.FC<BpiOSKernelStatusIndicatorProps> = ({
  refreshInterval = 5000,
  showDetailedMetrics = true,
  onStatusChange,
}) => {
  // State management
  const [status, setStatus] = useState<BpiOSKernelStatus>({
    connectionStatus: 'disconnected',
    connectionMessage: 'Not connected to BPI OS Kernel',
  });
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const intervalRef = useRef<NodeJS.Timeout | null>(null);

  // Fetch kernel status
  const fetchKernelStatus = async () => {
    setLoading(true);
    setError(null);

    try {
      const response = await fetch(`${process.env.REACT_APP_API_URL || 'https://api.pravyom.com'}/api/bpi-os/kernel-status`);
      
      if (response.ok) {
        const kernelStatus: BpiOSKernelStatus = await response.json();
        setStatus(kernelStatus);
        onStatusChange?.(kernelStatus);
      } else {
        throw new Error('Failed to fetch kernel status');
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Unknown error';
      setError(errorMessage);
      setStatus(prev => ({
        ...prev,
        connectionStatus: 'error',
        connectionMessage: errorMessage,
      }));
    } finally {
      setLoading(false);
    }
  };

  // Auto-refresh setup
  useEffect(() => {
    fetchKernelStatus();

    if (refreshInterval > 0) {
      intervalRef.current = setInterval(fetchKernelStatus, refreshInterval);
    }

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, [refreshInterval]);

  // Get status badge properties
  const getStatusBadge = () => {
    switch (status.connectionStatus) {
      case 'active':
        return { status: 'success' as const, text: 'Active', icon: <CheckCircleOutlined /> };
      case 'connected':
        return { status: 'processing' as const, text: 'Connected', icon: <CheckCircleOutlined /> };
      case 'authenticated':
        return { status: 'processing' as const, text: 'Authenticated', icon: <SecurityScanOutlined /> };
      case 'connecting':
        return { status: 'processing' as const, text: 'Connecting', icon: <LoadingOutlined /> };
      case 'error':
        return { status: 'error' as const, text: 'Error', icon: <CloseCircleOutlined /> };
      default:
        return { status: 'default' as const, text: 'Disconnected', icon: <ExclamationCircleOutlined /> };
    }
  };

  // Get health score color
  const getHealthScoreColor = (score: number) => {
    if (score >= 90) return '#52c41a';
    if (score >= 70) return '#faad14';
    if (score >= 50) return '#fa8c16';
    return '#ff4d4f';
  };

  // Get threat level color
  const getThreatLevelColor = (level: string) => {
    switch (level) {
      case 'low': return '#52c41a';
      case 'medium': return '#faad14';
      case 'high': return '#fa8c16';
      case 'critical': return '#ff4d4f';
      default: return '#d9d9d9';
    }
  };

  const statusBadge = getStatusBadge();

  if (error && !status.kernelInfo) {
    return (
      <Alert
        message="BPI OS Kernel Status Error"
        description={error}
        type="error"
        showIcon
        action={
          <Button size="small" onClick={fetchKernelStatus}>
            Retry
          </Button>
        }
      />
    );
  }

  return (
    <div>
      {/* Main Status Card */}
      <Card>
        <Row justify="space-between" align="middle">
          <Col>
            <Space>
              <Badge
                status={statusBadge.status}
                text={
                  <Space>
                    {statusBadge.icon}
                    <Text strong>BPI OS Kernel: {statusBadge.text}</Text>
                  </Space>
                }
              />
              {loading && <Spin size="small" />}
            </Space>
          </Col>
          <Col>
            <Space>
              <Button
                icon={<ReloadOutlined />}
                onClick={fetchKernelStatus}
                loading={loading}
                size="small"
              >
                Refresh
              </Button>
            </Space>
          </Col>
        </Row>

        <div style={{ marginTop: 16 }}>
          <Text type="secondary">{status.connectionMessage}</Text>
        </div>

        {/* Kernel Information */}
        {status.kernelInfo && (
          <div style={{ marginTop: 16 }}>
            <Space wrap>
              <Tag icon={<DatabaseOutlined />}>
                Kernel: {status.kernelInfo.kernelId.substring(0, 8)}...
              </Tag>
              <Tag icon={<ApiOutlined />}>
                Version: {status.kernelInfo.version}
              </Tag>
              <Tag icon={<CloudServerOutlined />}>
                OS: {status.kernelInfo.osVersion}
              </Tag>
              <Tag icon={<ClockCircleOutlined />}>
                Uptime: {Math.floor(status.kernelInfo.uptime / 3600)}h
              </Tag>
            </Space>
          </div>
        )}

        {/* Performance Health Score */}
        {status.performanceMetrics && (
          <div style={{ marginTop: 16 }}>
            <Row gutter={16}>
              <Col span={12}>
                <Statistic
                  title="Health Score"
                  value={status.performanceMetrics.healthScore}
                  suffix="%"
                  valueStyle={{ color: getHealthScoreColor(status.performanceMetrics.healthScore) }}
                  prefix={<MonitorOutlined />}
                />
              </Col>
              <Col span={12}>
                <Statistic
                  title="Kernel Latency"
                  value={status.performanceMetrics.kernelLatency}
                  suffix="ms"
                  prefix={<ThunderboltOutlined />}
                />
              </Col>
            </Row>
          </div>
        )}
      </Card>

      {/* Detailed Metrics */}
      {showDetailedMetrics && status.connectionStatus === 'active' && (
        <Row gutter={[16, 16]} style={{ marginTop: 16 }}>
          {/* Process Status */}
          {status.processStatus && (
            <Col span={12}>
              <Card title="Process Status" size="small">
                <Row gutter={16}>
                  <Col span={8}>
                    <Statistic
                      title="Total"
                      value={status.processStatus.totalProcesses}
                      prefix={<NodeIndexOutlined />}
                    />
                  </Col>
                  <Col span={8}>
                    <Statistic
                      title="Active"
                      value={status.processStatus.activeProcesses}
                      valueStyle={{ color: '#52c41a' }}
                      prefix={<CheckCircleOutlined />}
                    />
                  </Col>
                  <Col span={8}>
                    <Statistic
                      title="Failed"
                      value={status.processStatus.failedProcesses}
                      valueStyle={{ color: '#ff4d4f' }}
                      prefix={<CloseCircleOutlined />}
                    />
                  </Col>
                </Row>
                
                <Divider />
                
                <List
                  size="small"
                  dataSource={Object.entries(status.processStatus.processTypes)}
                  renderItem={([type, count]) => (
                    <List.Item>
                      <Space>
                        <Avatar size="small" icon={<NodeIndexOutlined />} />
                        <Text>{type}</Text>
                        <Tag>{count}</Tag>
                      </Space>
                    </List.Item>
                  )}
                />
              </Card>
            </Col>
          )}

          {/* Resource Utilization */}
          {status.resourceUtilization && (
            <Col span={12}>
              <Card title="Resource Utilization" size="small">
                <Space direction="vertical" style={{ width: '100%' }}>
                  <div>
                    <Text>CPU Usage</Text>
                    <Progress
                      percent={status.resourceUtilization.cpuUsage}
                      strokeColor={status.resourceUtilization.cpuUsage > 80 ? '#ff4d4f' : '#52c41a'}
                      size="small"
                    />
                  </div>
                  <div>
                    <Text>Memory Usage</Text>
                    <Progress
                      percent={status.resourceUtilization.memoryUsage}
                      strokeColor={status.resourceUtilization.memoryUsage > 85 ? '#ff4d4f' : '#52c41a'}
                      size="small"
                    />
                  </div>
                  <div>
                    <Text>Network Usage</Text>
                    <Progress
                      percent={status.resourceUtilization.networkUsage}
                      strokeColor={status.resourceUtilization.networkUsage > 90 ? '#ff4d4f' : '#52c41a'}
                      size="small"
                    />
                  </div>
                  <div>
                    <Text>Storage Usage</Text>
                    <Progress
                      percent={status.resourceUtilization.storageUsage}
                      strokeColor={status.resourceUtilization.storageUsage > 95 ? '#ff4d4f' : '#52c41a'}
                      size="small"
                    />
                  </div>
                </Space>
              </Card>
            </Col>
          )}

          {/* Service Mapper Status */}
          {status.serviceMapperStatus && (
            <Col span={12}>
              <Card title="Service Mapper" size="small">
                <Row gutter={16}>
                  <Col span={12}>
                    <Statistic
                      title="Active Mappings"
                      value={status.serviceMapperStatus.activeMappings}
                      prefix={<ApiOutlined />}
                    />
                  </Col>
                  <Col span={12}>
                    <Statistic
                      title="Efficiency"
                      value={status.serviceMapperStatus.mappingEfficiency}
                      suffix="%"
                      valueStyle={{
                        color: status.serviceMapperStatus.mappingEfficiency > 90 ? '#52c41a' : '#faad14'
                      }}
                    />
                  </Col>
                </Row>
                
                <div style={{ marginTop: 16 }}>
                  <Space>
                    <Tag color={status.serviceMapperStatus.autoScalingActive ? 'green' : 'default'}>
                      Auto Scaling: {status.serviceMapperStatus.autoScalingActive ? 'ON' : 'OFF'}
                    </Tag>
                    {status.serviceMapperStatus.failedMappings > 0 && (
                      <Tag color="red">
                        Failed: {status.serviceMapperStatus.failedMappings}
                      </Tag>
                    )}
                  </Space>
                </div>
              </Card>
            </Col>
          )}

          {/* Security Status */}
          {status.securityStatus && (
            <Col span={12}>
              <Card title="Security Status" size="small">
                <Space direction="vertical" style={{ width: '100%' }}>
                  <div>
                    <Space>
                      <Text>Security Level:</Text>
                      <Tag color="blue">{status.securityStatus.securityLevel.toUpperCase()}</Tag>
                    </Space>
                  </div>
                  <div>
                    <Space>
                      <Text>Threat Level:</Text>
                      <Tag color={getThreatLevelColor(status.securityStatus.threatLevel)}>
                        {status.securityStatus.threatLevel.toUpperCase()}
                      </Tag>
                    </Space>
                  </div>
                  <div>
                    <Space wrap>
                      <Tag
                        icon={<SecurityScanOutlined />}
                        color={status.securityStatus.encryptionActive ? 'green' : 'red'}
                      >
                        Encryption: {status.securityStatus.encryptionActive ? 'ON' : 'OFF'}
                      </Tag>
                      <Tag
                        icon={<MonitorOutlined />}
                        color={status.securityStatus.auditActive ? 'green' : 'red'}
                      >
                        Audit: {status.securityStatus.auditActive ? 'ON' : 'OFF'}
                      </Tag>
                    </Space>
                  </div>
                </Space>
              </Card>
            </Col>
          )}
        </Row>
      )}

      {/* Connection Timestamps */}
      {(status.connectedAt || status.lastHeartbeat) && (
        <Card size="small" style={{ marginTop: 16 }}>
          <Row gutter={16}>
            {status.connectedAt && (
              <Col span={12}>
                <Text type="secondary">
                  Connected: {status.connectedAt.toLocaleString()}
                </Text>
              </Col>
            )}
            {status.lastHeartbeat && (
              <Col span={12}>
                <Text type="secondary">
                  Last Heartbeat: {status.lastHeartbeat.toLocaleString()}
                </Text>
              </Col>
            )}
          </Row>
        </Card>
      )}
    </div>
  );
};

export default BpiOSKernelStatusIndicator;
