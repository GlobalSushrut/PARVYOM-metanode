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
  LoadingOutlined,
  CloseCircleOutlined,
  ApiOutlined,
} from '@ant-design/icons';

const { Text, Title } = Typography;

export interface BpiConnectionMetrics {
  latency: number;
  throughput: number;
  errorRate: number;
  uptime: number;
  lastSync: Date;
  blockHeight: number;
  peerCount: number;
  validatorCount: number;
}

export interface BpiNodeInfo {
  nodeId: string;
  version: string;
  network: 'mainnet' | 'testnet' | 'development';
  consensusType: string;
  zkProofEnabled: boolean;
  economicCoordination: boolean;
}

export interface BpiStatusData {
  status: 'connected' | 'connecting' | 'disconnected' | 'error' | 'syncing';
  message: string;
  connectedAt?: Date;
  lastHealthCheck?: Date;
  nodeInfo?: BpiNodeInfo;
  metrics?: BpiConnectionMetrics;
  healthScore: number;
}

interface BpiStatusIndicatorProps {
  status?: BpiStatusData;
  showDetails?: boolean;
  onRefresh?: () => void;
  onReconnect?: () => void;
  refreshInterval?: number;
}

export const BpiStatusIndicator: React.FC<BpiStatusIndicatorProps> = ({
  status,
  showDetails = true,
  onRefresh,
  onReconnect,
  refreshInterval = 30000,
}) => {
  const [currentStatus, setCurrentStatus] = useState<BpiStatusData>(
    status || {
      status: 'disconnected',
      message: 'Not connected to BPI infrastructure',
      healthScore: 0,
    }
  );
  const [isRefreshing, setIsRefreshing] = useState(false);

  // Auto-refresh status
  useEffect(() => {
    if (refreshInterval > 0 && currentStatus.status === 'connected') {
      const interval = setInterval(async () => {
        await refreshStatus();
      }, refreshInterval);

      return () => clearInterval(interval);
    }
  }, [refreshInterval, currentStatus.status]);

  // Update status when prop changes
  useEffect(() => {
    if (status) {
      setCurrentStatus(status);
    }
  }, [status]);

  const refreshStatus = async () => {
    if (isRefreshing) return;

    setIsRefreshing(true);
    try {
      const response = await fetch(`${process.env.REACT_APP_API_URL || 'https://api.pravyom.com'}/api/bpi/status`);
      if (response.ok) {
        const newStatus = await response.json();
        setCurrentStatus(newStatus);
      }
    } catch (error) {
      console.error('Failed to refresh BPI status:', error);
    } finally {
      setIsRefreshing(false);
    }

    if (onRefresh) {
      onRefresh();
    }
  };

  const getStatusColor = (status: string): string => {
    switch (status) {
      case 'connected':
        return '#52c41a';
      case 'connecting':
      case 'syncing':
        return '#1890ff';
      case 'error':
        return '#ff4d4f';
      case 'disconnected':
      default:
        return '#d9d9d9';
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'connected':
        return <CheckCircleOutlined style={{ color: '#52c41a' }} />;
      case 'connecting':
      case 'syncing':
        return <LoadingOutlined style={{ color: '#1890ff' }} />;
      case 'error':
        return <CloseCircleOutlined style={{ color: '#ff4d4f' }} />;
      case 'disconnected':
      default:
        return <ExclamationCircleOutlined style={{ color: '#d9d9d9' }} />;
    }
  };

  const getHealthScoreColor = (score: number): string => {
    if (score >= 80) return '#52c41a';
    if (score >= 60) return '#faad14';
    if (score >= 40) return '#fa8c16';
    return '#ff4d4f';
  };

  const formatUptime = (uptime: number): string => {
    const hours = Math.floor(uptime / 3600);
    const minutes = Math.floor((uptime % 3600) / 60);
    return `${hours}h ${minutes}m`;
  };

  const formatLastSync = (date: Date): string => {
    const now = new Date();
    const diff = now.getTime() - date.getTime();
    const seconds = Math.floor(diff / 1000);
    
    if (seconds < 60) return `${seconds}s ago`;
    if (seconds < 3600) return `${Math.floor(seconds / 60)}m ago`;
    return `${Math.floor(seconds / 3600)}h ago`;
  };

  return (
    <Card
      size="small"
      title={
        <Space>
          <ApiOutlined />
          <span>BPI Infrastructure Status</span>
        </Space>
      }
      extra={
        <Space>
          <Button
            size="small"
            icon={<ReloadOutlined />}
            loading={isRefreshing}
            onClick={refreshStatus}
          >
            Refresh
          </Button>
          {currentStatus.status === 'error' && onReconnect && (
            <Button
              size="small"
              type="primary"
              onClick={onReconnect}
            >
              Reconnect
            </Button>
          )}
        </Space>
      }
    >
      {/* Main Status Display */}
      <Row gutter={16} align="middle">
        <Col span={6}>
          <Space direction="vertical" size="small" style={{ width: '100%' }}>
            <Badge
              status={currentStatus.status === 'connected' ? 'success' : 
                     currentStatus.status === 'error' ? 'error' : 'processing'}
              text={
                <Text strong style={{ textTransform: 'capitalize' }}>
                  {currentStatus.status}
                </Text>
              }
            />
            <Text type="secondary" style={{ fontSize: '12px' }}>
              {currentStatus.message}
            </Text>
          </Space>
        </Col>

        <Col span={6}>
          <Statistic
            title="Health Score"
            value={currentStatus.healthScore}
            suffix="%"
            valueStyle={{ 
              color: getHealthScoreColor(currentStatus.healthScore),
              fontSize: '18px'
            }}
          />
          <Progress
            percent={currentStatus.healthScore}
            strokeColor={getHealthScoreColor(currentStatus.healthScore)}
            size="small"
            showInfo={false}
          />
        </Col>

        {currentStatus.connectedAt && (
          <Col span={6}>
            <Statistic
              title="Connected Since"
              value={formatUptime((Date.now() - currentStatus.connectedAt.getTime()) / 1000)}
              prefix={<CheckCircleOutlined style={{ color: '#52c41a' }} />}
              valueStyle={{ fontSize: '14px' }}
            />
          </Col>
        )}

        {currentStatus.lastHealthCheck && (
          <Col span={6}>
            <Statistic
              title="Last Check"
              value={formatLastSync(currentStatus.lastHealthCheck)}
              prefix={<ThunderboltOutlined />}
              valueStyle={{ fontSize: '14px' }}
            />
          </Col>
        )}
      </Row>

      {/* Detailed Information */}
      {showDetails && currentStatus.status === 'connected' && (
        <>
          {/* Node Information */}
          {currentStatus.nodeInfo && (
            <Card size="small" title="Node Information" style={{ marginTop: 16 }}>
              <Row gutter={16}>
                <Col span={8}>
                  <Space direction="vertical" size="small">
                    <Text strong>Node ID</Text>
                    <Text code style={{ fontSize: '12px' }}>
                      {currentStatus.nodeInfo.nodeId.substring(0, 16)}...
                    </Text>
                  </Space>
                </Col>
                <Col span={4}>
                  <Space direction="vertical" size="small">
                    <Text strong>Version</Text>
                    <Tag color="blue">{currentStatus.nodeInfo.version}</Tag>
                  </Space>
                </Col>
                <Col span={4}>
                  <Space direction="vertical" size="small">
                    <Text strong>Network</Text>
                    <Tag color={
                      currentStatus.nodeInfo.network === 'mainnet' ? 'green' :
                      currentStatus.nodeInfo.network === 'testnet' ? 'orange' : 'purple'
                    }>
                      {currentStatus.nodeInfo.network.toUpperCase()}
                    </Tag>
                  </Space>
                </Col>
                <Col span={4}>
                  <Space direction="vertical" size="small">
                    <Text strong>Consensus</Text>
                    <Tag color="cyan">{currentStatus.nodeInfo.consensusType}</Tag>
                  </Space>
                </Col>
                <Col span={4}>
                  <Space direction="vertical" size="small">
                    <Text strong>Features</Text>
                    <Space>
                      {currentStatus.nodeInfo.zkProofEnabled && (
                        <Tooltip title="ZK Proofs Enabled">
                          <Avatar size="small" icon={<SecurityScanOutlined />} style={{ backgroundColor: '#52c41a' }} />
                        </Tooltip>
                      )}
                      {currentStatus.nodeInfo.economicCoordination && (
                        <Tooltip title="Economic Coordination">
                          <Avatar size="small" icon={<DatabaseOutlined />} style={{ backgroundColor: '#1890ff' }} />
                        </Tooltip>
                      )}
                    </Space>
                  </Space>
                </Col>
              </Row>
            </Card>
          )}

          {/* Performance Metrics */}
          {currentStatus.metrics && (
            <Card size="small" title="Performance Metrics" style={{ marginTop: 16 }}>
              <Row gutter={16}>
                <Col span={6}>
                  <Statistic
                    title="Latency"
                    value={currentStatus.metrics.latency}
                    suffix="ms"
                    valueStyle={{ 
                      color: currentStatus.metrics.latency < 100 ? '#52c41a' : 
                             currentStatus.metrics.latency < 500 ? '#faad14' : '#ff4d4f'
                    }}
                  />
                </Col>
                <Col span={6}>
                  <Statistic
                    title="Throughput"
                    value={currentStatus.metrics.throughput}
                    suffix="TPS"
                    valueStyle={{ color: '#1890ff' }}
                  />
                </Col>
                <Col span={6}>
                  <Statistic
                    title="Error Rate"
                    value={(currentStatus.metrics.errorRate * 100).toFixed(2)}
                    suffix="%"
                    valueStyle={{ 
                      color: currentStatus.metrics.errorRate < 0.01 ? '#52c41a' : 
                             currentStatus.metrics.errorRate < 0.05 ? '#faad14' : '#ff4d4f'
                    }}
                  />
                </Col>
                <Col span={6}>
                  <Statistic
                    title="Uptime"
                    value={formatUptime(currentStatus.metrics.uptime)}
                    valueStyle={{ color: '#52c41a' }}
                  />
                </Col>
              </Row>

              <Row gutter={16} style={{ marginTop: 16 }}>
                <Col span={8}>
                  <Statistic
                    title="Block Height"
                    value={currentStatus.metrics.blockHeight.toLocaleString()}
                    prefix={<DatabaseOutlined />}
                  />
                </Col>
                <Col span={8}>
                  <Statistic
                    title="Peer Count"
                    value={currentStatus.metrics.peerCount}
                    prefix={<ApiOutlined />}
                  />
                </Col>
                <Col span={8}>
                  <Statistic
                    title="Validators"
                    value={currentStatus.metrics.validatorCount}
                    prefix={<SecurityScanOutlined />}
                  />
                </Col>
              </Row>

              {currentStatus.metrics.lastSync && (
                <div style={{ marginTop: 16 }}>
                  <Text type="secondary">
                    Last sync: {formatLastSync(currentStatus.metrics.lastSync)}
                  </Text>
                </div>
              )}
            </Card>
          )}
        </>
      )}

      {/* Error Details */}
      {currentStatus.status === 'error' && (
        <Card size="small" title="Error Details" style={{ marginTop: 16 }}>
          <Text type="danger">{currentStatus.message}</Text>
          {onReconnect && (
            <div style={{ marginTop: 8 }}>
              <Button type="primary" danger onClick={onReconnect}>
                Attempt Reconnection
              </Button>
            </div>
          )}
        </Card>
      )}
    </Card>
  );
};

export default BpiStatusIndicator;
