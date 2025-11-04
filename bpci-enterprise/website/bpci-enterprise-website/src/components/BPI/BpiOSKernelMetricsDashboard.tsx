/**
 * BPI OS Kernel Metrics Dashboard Component
 * Comprehensive real-time monitoring of BPI Blockchain Operating System kernel activity
 */

import React, { useState, useEffect, useRef } from 'react';
import {
  Card,
  Row,
  Col,
  Statistic,
  Progress,
  Table,
  Tag,
  Space,
  Typography,
  Button,
  Select,
  DatePicker,
  Alert,
  Tabs,
  List,
  Avatar,
  Badge,
  Tooltip,
  Spin,
  Divider,
} from 'antd';
import {
  LineChartOutlined,
  DatabaseOutlined,
  ThunderboltOutlined,
  SecurityScanOutlined,
  NodeIndexOutlined,
  CheckCircleOutlined,
  ClockCircleOutlined,
  ReloadOutlined,
  DownloadOutlined,
  ApiOutlined,
  MonitorOutlined,
  CloudServerOutlined,
  GlobalOutlined,
  HddOutlined,
  WarningOutlined,
  ExclamationCircleOutlined,
} from '@ant-design/icons';

const { Title, Text } = Typography;
const { TabPane } = Tabs;
const { RangePicker } = DatePicker;
const { Option } = Select;

// Interfaces for BPI OS Kernel data structures
export interface BpiOSKernelMetrics {
  // Kernel Core Metrics
  kernelHealth: number;
  kernelLatency: number;
  kernelThroughput: number;
  kernelUptime: number;
  
  // Process Management Metrics
  totalProcesses: number;
  activeProcesses: number;
  failedProcesses: number;
  processCreationRate: number;
  processTypes: Record<string, number>;
  
  // Resource Utilization Metrics
  cpuUtilization: number;
  memoryUtilization: number;
  networkUtilization: number;
  storageUtilization: number;
  gpuUtilization: number;
  
  // Service Mapper Metrics
  activeMappings: number;
  failedMappings: number;
  mappingEfficiency: number;
  autoScalingEvents: number;
  
  // Security Metrics
  securityLevel: 'basic' | 'standard' | 'enhanced' | 'maximum';
  threatLevel: 'low' | 'medium' | 'high' | 'critical';
  encryptionActive: boolean;
  auditEvents: number;
  securityIncidents: number;
  
  // Performance Metrics
  responseTime: number;
  errorRate: number;
  successRate: number;
  resourceEfficiency: number;
}

export interface BpiOSProcessMetrics {
  processId: string;
  processType: 'governance' | 'orchestration' | 'api' | 'background' | 'security' | 'audit';
  status: 'running' | 'stopped' | 'failed' | 'starting';
  cpuUsage: number;
  memoryUsage: number;
  uptime: number;
  priority: 'low' | 'normal' | 'high' | 'critical';
  instances: number;
}

export interface BpiOSResourceMetrics {
  resourceType: 'cpu' | 'memory' | 'network' | 'storage' | 'gpu';
  totalCapacity: number;
  usedCapacity: number;
  availableCapacity: number;
  utilizationPercentage: number;
  peakUsage: number;
  averageUsage: number;
}

export interface BpiOSServiceMapping {
  mappingId: string;
  serviceType: string;
  processId: string;
  status: 'active' | 'inactive' | 'failed' | 'scaling';
  resourceAllocation: {
    cpu: number;
    memory: number;
    network: number;
    storage: number;
  };
  performance: {
    latency: number;
    throughput: number;
    errorRate: number;
  };
  createdAt: string;
  lastUpdated: string;
}

export interface BpiOSHistoricalData {
  timestamp: string;
  kernelHealth: number;
  processCount: number;
  cpuUtilization: number;
  memoryUtilization: number;
  networkLatency: number;
  activeMappings: number;
  securityIncidents: number;
}

interface BpiOSKernelMetricsDashboardProps {
  refreshInterval?: number;
  showHistoricalData?: boolean;
  defaultTimeRange?: [string, string];
}

export const BpiOSKernelMetricsDashboard: React.FC<BpiOSKernelMetricsDashboardProps> = ({
  refreshInterval = 30000,
  showHistoricalData = true,
  defaultTimeRange,
}) => {
  // State management
  const [loading, setLoading] = useState(false);
  const [kernelMetrics, setKernelMetrics] = useState<BpiOSKernelMetrics | null>(null);
  const [processMetrics, setProcessMetrics] = useState<BpiOSProcessMetrics[]>([]);
  const [resourceMetrics, setResourceMetrics] = useState<BpiOSResourceMetrics[]>([]);
  const [serviceMappings, setServiceMappings] = useState<BpiOSServiceMapping[]>([]);
  const [historicalData, setHistoricalData] = useState<BpiOSHistoricalData[]>([]);
  const [timeRange, setTimeRange] = useState<[string, string] | undefined>(defaultTimeRange);
  const [selectedMetric, setSelectedMetric] = useState<string>('overview');
  const [error, setError] = useState<string | null>(null);

  const intervalRef = useRef<NodeJS.Timeout | null>(null);

  // Fetch all BPI OS Kernel metrics
  const fetchMetrics = async () => {
    setLoading(true);
    setError(null);

    try {
      const API_BASE = process.env.REACT_APP_API_URL || 'https://api.pravyom.com';
      const [
        kernelResponse,
        processResponse,
        resourceResponse,
        mappingResponse,
        historicalResponse,
      ] = await Promise.all([
        fetch(`${API_BASE}/api/bpi-os/kernel-metrics`),
        fetch(`${API_BASE}/api/bpi-os/process-metrics`),
        fetch(`${API_BASE}/api/bpi-os/resource-metrics`),
        fetch(`${API_BASE}/api/bpi-os/service-mappings`),
        showHistoricalData ? fetch(`${API_BASE}/api/bpi-os/historical-metrics${timeRange ? `?from=${timeRange[0]}&to=${timeRange[1]}` : ''}`) : Promise.resolve({ ok: true, json: () => [] }),
      ]);

      if (kernelResponse.ok) {
        setKernelMetrics(await kernelResponse.json());
      }
      if (processResponse.ok) {
        setProcessMetrics(await processResponse.json());
      }
      if (resourceResponse.ok) {
        setResourceMetrics(await resourceResponse.json());
      }
      if (mappingResponse.ok) {
        setServiceMappings(await mappingResponse.json());
      }
      if (historicalResponse.ok) {
        setHistoricalData(await historicalResponse.json());
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch BPI OS Kernel metrics');
    } finally {
      setLoading(false);
    }
  };

  // Auto-refresh setup
  useEffect(() => {
    fetchMetrics();

    if (refreshInterval > 0) {
      intervalRef.current = setInterval(fetchMetrics, refreshInterval);
    }

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, [refreshInterval, timeRange]);

  // Process metrics chart data
  const getProcessChartData = () => {
    if (!kernelMetrics) return [];
    
    return Object.entries(kernelMetrics.processTypes).map(([type, count]) => ({
      type,
      count,
    }));
  };

  // Resource utilization chart data
  const getResourceChartData = () => {
    if (!kernelMetrics) return [];
    
    return [
      { resource: 'CPU', utilization: kernelMetrics.cpuUtilization },
      { resource: 'Memory', utilization: kernelMetrics.memoryUtilization },
      { resource: 'Network', utilization: kernelMetrics.networkUtilization },
      { resource: 'Storage', utilization: kernelMetrics.storageUtilization },
      { resource: 'GPU', utilization: kernelMetrics.gpuUtilization },
    ];
  };

  // Historical data chart config
  const getHistoricalChartConfig = () => ({
    data: historicalData,
    xField: 'timestamp',
    yField: selectedMetric === 'kernelHealth' ? 'kernelHealth' :
            selectedMetric === 'processes' ? 'processCount' :
            selectedMetric === 'cpu' ? 'cpuUtilization' :
            selectedMetric === 'memory' ? 'memoryUtilization' :
            selectedMetric === 'mappings' ? 'activeMappings' : 'kernelHealth',
    smooth: true,
    color: '#1890ff',
    point: {
      size: 3,
      shape: 'circle',
    },
    tooltip: {
      showMarkers: false,
    },
  });

  // Process table columns
  const processColumns = [
    {
      title: 'Process ID',
      dataIndex: 'processId',
      key: 'processId',
      render: (id: string) => <Text code>{id.substring(0, 12)}...</Text>,
    },
    {
      title: 'Type',
      dataIndex: 'processType',
      key: 'processType',
      render: (type: string) => (
        <Tag color={
          type === 'governance' ? 'blue' :
          type === 'orchestration' ? 'green' :
          type === 'api' ? 'orange' :
          type === 'security' ? 'red' :
          type === 'audit' ? 'purple' : 'default'
        }>
          {type.toUpperCase()}
        </Tag>
      ),
    },
    {
      title: 'Status',
      dataIndex: 'status',
      key: 'status',
      render: (status: string) => (
        <Badge
          status={
            status === 'running' ? 'success' :
            status === 'starting' ? 'processing' :
            status === 'failed' ? 'error' : 'default'
          }
          text={status.toUpperCase()}
        />
      ),
    },
    {
      title: 'CPU Usage',
      dataIndex: 'cpuUsage',
      key: 'cpuUsage',
      render: (usage: number) => (
        <Progress
          percent={usage}
          size="small"
          strokeColor={usage > 80 ? '#ff4d4f' : usage > 60 ? '#faad14' : '#52c41a'}
        />
      ),
      sorter: (a: any, b: any) => a.cpuUsage - b.cpuUsage,
    },
    {
      title: 'Memory Usage',
      dataIndex: 'memoryUsage',
      key: 'memoryUsage',
      render: (usage: number) => (
        <Progress
          percent={usage}
          size="small"
          strokeColor={usage > 85 ? '#ff4d4f' : usage > 70 ? '#faad14' : '#52c41a'}
        />
      ),
      sorter: (a: any, b: any) => a.memoryUsage - b.memoryUsage,
    },
    {
      title: 'Priority',
      dataIndex: 'priority',
      key: 'priority',
      render: (priority: string) => (
        <Tag color={
          priority === 'critical' ? 'red' :
          priority === 'high' ? 'orange' :
          priority === 'normal' ? 'blue' : 'default'
        }>
          {priority.toUpperCase()}
        </Tag>
      ),
    },
    {
      title: 'Instances',
      dataIndex: 'instances',
      key: 'instances',
      sorter: (a: any, b: any) => a.instances - b.instances,
    },
  ];

  // Service mapping table columns
  const mappingColumns = [
    {
      title: 'Mapping ID',
      dataIndex: 'mappingId',
      key: 'mappingId',
      render: (id: string) => <Text code>{id.substring(0, 12)}...</Text>,
    },
    {
      title: 'Service Type',
      dataIndex: 'serviceType',
      key: 'serviceType',
    },
    {
      title: 'Status',
      dataIndex: 'status',
      key: 'status',
      render: (status: string) => (
        <Badge
          status={
            status === 'active' ? 'success' :
            status === 'scaling' ? 'processing' :
            status === 'failed' ? 'error' : 'default'
          }
          text={status.toUpperCase()}
        />
      ),
    },
    {
      title: 'Latency',
      dataIndex: ['performance', 'latency'],
      key: 'latency',
      render: (latency: number) => `${latency}ms`,
      sorter: (a: any, b: any) => a.performance.latency - b.performance.latency,
    },
    {
      title: 'Throughput',
      dataIndex: ['performance', 'throughput'],
      key: 'throughput',
      render: (throughput: number) => `${throughput} ops/s`,
      sorter: (a: any, b: any) => a.performance.throughput - b.performance.throughput,
    },
    {
      title: 'Error Rate',
      dataIndex: ['performance', 'errorRate'],
      key: 'errorRate',
      render: (errorRate: number) => (
        <Text style={{ color: errorRate > 5 ? '#ff4d4f' : '#52c41a' }}>
          {errorRate.toFixed(2)}%
        </Text>
      ),
      sorter: (a: any, b: any) => a.performance.errorRate - b.performance.errorRate,
    },
  ];

  // Get security level color
  const getSecurityLevelColor = (level: string) => {
    switch (level) {
      case 'maximum': return '#52c41a';
      case 'enhanced': return '#1890ff';
      case 'standard': return '#faad14';
      case 'basic': return '#ff4d4f';
      default: return '#d9d9d9';
    }
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

  if (error) {
    return (
      <Alert
        message="BPI OS Kernel Metrics Error"
        description={error}
        type="error"
        showIcon
        action={
          <Button size="small" onClick={fetchMetrics}>
            Retry
          </Button>
        }
      />
    );
  }

  return (
    <div>
      {/* Header */}
      <Row justify="space-between" align="middle" style={{ marginBottom: 24 }}>
        <Col>
          <Title level={3}>
            <DatabaseOutlined /> BPI OS Kernel Metrics Dashboard
          </Title>
        </Col>
        <Col>
          <Space>
            {showHistoricalData && (
              <RangePicker
                onChange={(dates) => {
                  if (dates) {
                    setTimeRange([dates[0]!.toISOString(), dates[1]!.toISOString()]);
                  } else {
                    setTimeRange(undefined);
                  }
                }}
              />
            )}
            <Button
              icon={<ReloadOutlined />}
              onClick={fetchMetrics}
              loading={loading}
            >
              Refresh
            </Button>
            <Button icon={<DownloadOutlined />}>
              Export
            </Button>
          </Space>
        </Col>
      </Row>

      <Tabs activeKey={selectedMetric} onChange={setSelectedMetric}>
        {/* Overview Tab */}
        <TabPane tab="Overview" key="overview">
          <Row gutter={[16, 16]}>
            {/* Kernel Health */}
            <Col span={8}>
              <Card title="Kernel Health" loading={loading}>
                {kernelMetrics && (
                  <Row gutter={16}>
                    <Col span={24}>
                      <Statistic
                        title="Health Score"
                        value={kernelMetrics.kernelHealth}
                        suffix="%"
                        valueStyle={{
                          color: kernelMetrics.kernelHealth > 90 ? '#52c41a' :
                                 kernelMetrics.kernelHealth > 70 ? '#faad14' : '#ff4d4f'
                        }}
                        prefix={<MonitorOutlined />}
                      />
                      <Progress
                        percent={kernelMetrics.kernelHealth}
                        strokeColor={
                          kernelMetrics.kernelHealth > 90 ? '#52c41a' :
                          kernelMetrics.kernelHealth > 70 ? '#faad14' : '#ff4d4f'
                        }
                        style={{ marginTop: 8 }}
                      />
                    </Col>
                    <Col span={12}>
                      <Statistic
                        title="Latency"
                        value={kernelMetrics.kernelLatency}
                        suffix="ms"
                        prefix={<ThunderboltOutlined />}
                      />
                    </Col>
                    <Col span={12}>
                      <Statistic
                        title="Throughput"
                        value={kernelMetrics.kernelThroughput}
                        suffix="ops/s"
                        prefix={<LineChartOutlined />}
                      />
                    </Col>
                  </Row>
                )}
              </Card>
            </Col>

            {/* Process Overview */}
            <Col span={8}>
              <Card title="Process Overview" loading={loading}>
                {kernelMetrics && (
                  <Row gutter={16}>
                    <Col span={8}>
                      <Statistic
                        title="Total"
                        value={kernelMetrics.totalProcesses}
                        prefix={<NodeIndexOutlined />}
                      />
                    </Col>
                    <Col span={8}>
                      <Statistic
                        title="Active"
                        value={kernelMetrics.activeProcesses}
                        valueStyle={{ color: '#52c41a' }}
                        prefix={<CheckCircleOutlined />}
                      />
                    </Col>
                    <Col span={8}>
                      <Statistic
                        title="Failed"
                        value={kernelMetrics.failedProcesses}
                        valueStyle={{ color: '#ff4d4f' }}
                        prefix={<ExclamationCircleOutlined />}
                      />
                    </Col>
                    <Col span={24}>
                      <Divider />
                      <div style={{ height: 150, display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
                        <Text type="secondary">Process Distribution Chart</Text>
                      </div>
                    </Col>
                  </Row>
                )}
              </Card>
            </Col>

            {/* Security Status */}
            <Col span={8}>
              <Card title="Security Status" loading={loading}>
                {kernelMetrics && (
                  <Space direction="vertical" style={{ width: '100%' }}>
                    <div>
                      <Space>
                        <Text>Security Level:</Text>
                        <Tag color={getSecurityLevelColor(kernelMetrics.securityLevel)}>
                          {kernelMetrics.securityLevel.toUpperCase()}
                        </Tag>
                      </Space>
                    </div>
                    <div>
                      <Space>
                        <Text>Threat Level:</Text>
                        <Tag color={getThreatLevelColor(kernelMetrics.threatLevel)}>
                          {kernelMetrics.threatLevel.toUpperCase()}
                        </Tag>
                      </Space>
                    </div>
                    <div>
                      <Row gutter={16}>
                        <Col span={12}>
                          <Statistic
                            title="Audit Events"
                            value={kernelMetrics.auditEvents}
                            prefix={<MonitorOutlined />}
                          />
                        </Col>
                        <Col span={12}>
                          <Statistic
                            title="Incidents"
                            value={kernelMetrics.securityIncidents}
                            valueStyle={{ color: kernelMetrics.securityIncidents > 0 ? '#ff4d4f' : '#52c41a' }}
                            prefix={<WarningOutlined />}
                          />
                        </Col>
                      </Row>
                    </div>
                    <div>
                      <Tag
                        icon={<SecurityScanOutlined />}
                        color={kernelMetrics.encryptionActive ? 'green' : 'red'}
                      >
                        Encryption: {kernelMetrics.encryptionActive ? 'ACTIVE' : 'INACTIVE'}
                      </Tag>
                    </div>
                  </Space>
                )}
              </Card>
            </Col>

            {/* Resource Utilization Chart */}
            <Col span={24}>
              <Card title="Resource Utilization" loading={loading}>
                {kernelMetrics && (
                  <div style={{ height: 300, display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
                    <Text type="secondary">Resource Utilization Chart</Text>
                  </div>
                )}
              </Card>
            </Col>
          </Row>
        </TabPane>

        {/* Processes Tab */}
        <TabPane tab="Processes" key="processes">
          <Row gutter={[16, 16]}>
            <Col span={24}>
              <Card title="Process Management" loading={loading}>
                {kernelMetrics && (
                  <Row gutter={16} style={{ marginBottom: 16 }}>
                    <Col span={6}>
                      <Statistic
                        title="Creation Rate"
                        value={kernelMetrics.processCreationRate}
                        suffix="/min"
                        prefix={<ThunderboltOutlined />}
                      />
                    </Col>
                    <Col span={6}>
                      <Statistic
                        title="Success Rate"
                        value={kernelMetrics.successRate}
                        suffix="%"
                        valueStyle={{ color: '#52c41a' }}
                      />
                    </Col>
                    <Col span={6}>
                      <Statistic
                        title="Error Rate"
                        value={kernelMetrics.errorRate}
                        suffix="%"
                        valueStyle={{ color: '#ff4d4f' }}
                      />
                    </Col>
                    <Col span={6}>
                      <Statistic
                        title="Response Time"
                        value={kernelMetrics.responseTime}
                        suffix="ms"
                      />
                    </Col>
                  </Row>
                )}
                
                <Table
                  dataSource={processMetrics}
                  columns={processColumns}
                  rowKey="processId"
                  pagination={{ pageSize: 10 }}
                  size="small"
                />
              </Card>
            </Col>
          </Row>
        </TabPane>

        {/* Service Mappings Tab */}
        <TabPane tab="Service Mappings" key="mappings">
          <Row gutter={[16, 16]}>
            <Col span={24}>
              <Card title="Service Mapper Status" loading={loading}>
                {kernelMetrics && (
                  <Row gutter={16} style={{ marginBottom: 16 }}>
                    <Col span={6}>
                      <Statistic
                        title="Active Mappings"
                        value={kernelMetrics.activeMappings}
                        prefix={<ApiOutlined />}
                      />
                    </Col>
                    <Col span={6}>
                      <Statistic
                        title="Failed Mappings"
                        value={kernelMetrics.failedMappings}
                        valueStyle={{ color: '#ff4d4f' }}
                        prefix={<ExclamationCircleOutlined />}
                      />
                    </Col>
                    <Col span={6}>
                      <Statistic
                        title="Mapping Efficiency"
                        value={kernelMetrics.mappingEfficiency}
                        suffix="%"
                        valueStyle={{
                          color: kernelMetrics.mappingEfficiency > 90 ? '#52c41a' : '#faad14'
                        }}
                      />
                    </Col>
                    <Col span={6}>
                      <Statistic
                        title="Auto Scaling Events"
                        value={kernelMetrics.autoScalingEvents}
                        prefix={<CloudServerOutlined />}
                      />
                    </Col>
                  </Row>
                )}
                
                <Table
                  dataSource={serviceMappings}
                  columns={mappingColumns}
                  rowKey="mappingId"
                  pagination={{ pageSize: 10 }}
                  size="small"
                />
              </Card>
            </Col>
          </Row>
        </TabPane>

        {/* Historical Data Tab */}
        {showHistoricalData && (
          <TabPane tab="Historical" key="historical">
            <Row gutter={[16, 16]}>
              <Col span={24}>
                <Card
                  title="Historical Metrics"
                  loading={loading}
                  extra={
                    <Select
                      value={selectedMetric}
                      onChange={setSelectedMetric}
                      style={{ width: 150 }}
                    >
                      <Option value="kernelHealth">Kernel Health</Option>
                      <Option value="processes">Process Count</Option>
                      <Option value="cpu">CPU Utilization</Option>
                      <Option value="memory">Memory Utilization</Option>
                      <Option value="mappings">Active Mappings</Option>
                    </Select>
                  }
                >
                  {historicalData.length > 0 && (
                    <div style={{ height: 400, display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
                      <Text type="secondary">Historical Data Chart</Text>
                    </div>
                  )}
                </Card>
              </Col>
            </Row>
          </TabPane>
        )}
      </Tabs>
    </div>
  );
};

export default BpiOSKernelMetricsDashboard;
