/**
 * BPI Metrics Dashboard Component
 * Comprehensive real-time monitoring of BPI infrastructure activity
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
} from 'antd';
import {
  LineChartOutlined,
  DatabaseOutlined,
  ThunderboltOutlined,
  SecurityScanOutlined,
  WalletOutlined,
  NodeIndexOutlined,
  TrophyOutlined,
  ExclamationCircleOutlined,
  CheckCircleOutlined,
  ClockCircleOutlined,
  ReloadOutlined,
  DownloadOutlined,
} from '@ant-design/icons';
import { Line, Area, Column } from '@ant-design/plots';

const { Title, Text } = Typography;
const { TabPane } = Tabs;
const { RangePicker } = DatePicker;
const { Option } = Select;

// Interfaces for BPI data structures
export interface BpiEconomicMetrics {
  totalValueLocked: number;
  activeSettlements: number;
  crossLedgerTransfers: number;
  genBalance: number;
  nexBalance: number;
  flxBalance: number;
  aurBalance: number;
  dailyVolume: number;
  weeklyVolume: number;
  monthlyVolume: number;
}

export interface BpiZkProofMetrics {
  proofsGenerated: number;
  proofsVerified: number;
  verificationRate: number;
  averageProofTime: number;
  proofTypes: Record<string, number>;
  failedProofs: number;
}

export interface BpiValidatorMetrics {
  totalValidators: number;
  activeValidators: number;
  stakingRatio: number;
  averageStake: number;
  topValidators: Array<{
    id: string;
    stake: number;
    performance: number;
    uptime: number;
  }>;
}

export interface BpiTransactionMetrics {
  totalTransactions: number;
  pendingTransactions: number;
  confirmedTransactions: number;
  failedTransactions: number;
  averageConfirmationTime: number;
  transactionFees: number;
  throughput: number;
}

export interface BpiNetworkMetrics {
  nodeCount: number;
  peerConnections: number;
  networkLatency: number;
  bandwidthUsage: number;
  syncStatus: number;
  forkCount: number;
}

export interface BpiAuctionMetrics {
  activeAuctions: number;
  completedAuctions: number;
  totalAuctionValue: number;
  averageBidAmount: number;
  participantCount: number;
  recentAuctions: Array<{
    id: string;
    type: 'government' | 'community';
    status: 'active' | 'completed' | 'cancelled';
    value: number;
    participants: number;
  }>;
}

export interface BpiHistoricalData {
  timestamp: string;
  blockHeight: number;
  transactionCount: number;
  networkLatency: number;
  validatorCount: number;
  tvl: number;
}

interface BpiMetricsDashboardProps {
  refreshInterval?: number;
  showHistoricalData?: boolean;
  defaultTimeRange?: [string, string];
}

export const BpiMetricsDashboard: React.FC<BpiMetricsDashboardProps> = ({
  refreshInterval = 30000,
  showHistoricalData = true,
  defaultTimeRange,
}) => {
  // State management
  const [loading, setLoading] = useState(false);
  const [economicMetrics, setEconomicMetrics] = useState<BpiEconomicMetrics | null>(null);
  const [zkProofMetrics, setZkProofMetrics] = useState<BpiZkProofMetrics | null>(null);
  const [validatorMetrics, setValidatorMetrics] = useState<BpiValidatorMetrics | null>(null);
  const [transactionMetrics, setTransactionMetrics] = useState<BpiTransactionMetrics | null>(null);
  const [networkMetrics, setNetworkMetrics] = useState<BpiNetworkMetrics | null>(null);
  const [auctionMetrics, setAuctionMetrics] = useState<BpiAuctionMetrics | null>(null);
  const [historicalData, setHistoricalData] = useState<BpiHistoricalData[]>([]);
  const [timeRange, setTimeRange] = useState<[string, string] | undefined>(defaultTimeRange);
  const [selectedMetric, setSelectedMetric] = useState<string>('overview');
  const [error, setError] = useState<string | null>(null);

  const intervalRef = useRef<NodeJS.Timeout | null>(null);

  // Fetch all BPI metrics
  const fetchMetrics = async () => {
    setLoading(true);
    setError(null);

    try {
      const API_BASE = process.env.REACT_APP_API_URL || 'https://api.pravyom.com';
      const [
        economicResponse,
        zkProofResponse,
        validatorResponse,
        transactionResponse,
        networkResponse,
        auctionResponse,
        historicalResponse,
      ] = await Promise.all([
        fetch(`${API_BASE}/api/bpi/metrics/economic`),
        fetch(`${API_BASE}/api/bpi/metrics/zkproof`),
        fetch(`${API_BASE}/api/bpi/metrics/validators`),
        fetch(`${API_BASE}/api/bpi/metrics/transactions`),
        fetch(`${API_BASE}/api/bpi/metrics/network`),
        fetch(`${API_BASE}/api/bpi/metrics/auctions`),
        showHistoricalData ? fetch(`${API_BASE}/api/bpi/metrics/historical${timeRange ? `?from=${timeRange[0]}&to=${timeRange[1]}` : ''}`) : Promise.resolve({ ok: true, json: () => [] }),
      ]);

      if (economicResponse.ok) {
        setEconomicMetrics(await economicResponse.json());
      }
      if (zkProofResponse.ok) {
        setZkProofMetrics(await zkProofResponse.json());
      }
      if (validatorResponse.ok) {
        setValidatorMetrics(await validatorResponse.json());
      }
      if (transactionResponse.ok) {
        setTransactionMetrics(await transactionResponse.json());
      }
      if (networkResponse.ok) {
        setNetworkMetrics(await networkResponse.json());
      }
      if (auctionResponse.ok) {
        setAuctionMetrics(await auctionResponse.json());
      }
      if (historicalResponse.ok) {
        setHistoricalData(await historicalResponse.json());
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch BPI metrics');
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

  // Economic metrics chart data
  const getEconomicChartData = () => {
    if (!economicMetrics) return [];
    
    return [
      { type: 'GEN', value: economicMetrics.genBalance },
      { type: 'NEX', value: economicMetrics.nexBalance },
      { type: 'FLX', value: economicMetrics.flxBalance },
      { type: 'AUR', value: economicMetrics.aurBalance },
    ];
  };

  // ZK Proof types chart data
  const getZkProofChartData = () => {
    if (!zkProofMetrics) return [];
    
    return Object.entries(zkProofMetrics.proofTypes).map(([type, count]) => ({
      type,
      count,
    }));
  };

  // Historical data chart config
  const getHistoricalChartConfig = () => ({
    data: historicalData,
    xField: 'timestamp',
    yField: selectedMetric === 'blockHeight' ? 'blockHeight' :
            selectedMetric === 'transactions' ? 'transactionCount' :
            selectedMetric === 'latency' ? 'networkLatency' :
            selectedMetric === 'validators' ? 'validatorCount' : 'tvl',
    smooth: true,
    color: '#1890ff',
    point: {
      size: 3,
      shape: 'circle',
    },
    tooltip: {
      showMarkers: false,
    },
    state: {
      active: {
        style: {
          shadowBlur: 4,
          stroke: '#000',
          fill: 'red',
        },
      },
    },
  });

  // Validator table columns
  const validatorColumns = [
    {
      title: 'Validator ID',
      dataIndex: 'id',
      key: 'id',
      render: (id: string) => <Text code>{id.substring(0, 12)}...</Text>,
    },
    {
      title: 'Stake',
      dataIndex: 'stake',
      key: 'stake',
      render: (stake: number) => `${stake.toLocaleString()} BPI`,
      sorter: (a: any, b: any) => a.stake - b.stake,
    },
    {
      title: 'Performance',
      dataIndex: 'performance',
      key: 'performance',
      render: (performance: number) => (
        <Progress
          percent={performance}
          size="small"
          strokeColor={performance > 90 ? '#52c41a' : performance > 70 ? '#faad14' : '#ff4d4f'}
        />
      ),
      sorter: (a: any, b: any) => a.performance - b.performance,
    },
    {
      title: 'Uptime',
      dataIndex: 'uptime',
      key: 'uptime',
      render: (uptime: number) => `${uptime.toFixed(2)}%`,
      sorter: (a: any, b: any) => a.uptime - b.uptime,
    },
  ];

  if (error) {
    return (
      <Alert
        message="BPI Metrics Error"
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
            <DatabaseOutlined /> BPI Infrastructure Metrics
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
            {/* Economic Metrics */}
            <Col span={12}>
              <Card title="Economic Metrics" loading={loading}>
                {economicMetrics && (
                  <Row gutter={16}>
                    <Col span={12}>
                      <Statistic
                        title="Total Value Locked"
                        value={economicMetrics.totalValueLocked}
                        precision={2}
                        prefix="$"
                        suffix="M"
                      />
                    </Col>
                    <Col span={12}>
                      <Statistic
                        title="Active Settlements"
                        value={economicMetrics.activeSettlements}
                        prefix={<ThunderboltOutlined />}
                      />
                    </Col>
                    <Col span={12}>
                      <Statistic
                        title="Cross-Ledger Transfers"
                        value={economicMetrics.crossLedgerTransfers}
                        prefix={<NodeIndexOutlined />}
                      />
                    </Col>
                    <Col span={12}>
                      <Statistic
                        title="Daily Volume"
                        value={economicMetrics.dailyVolume}
                        precision={2}
                        prefix="$"
                        suffix="K"
                      />
                    </Col>
                  </Row>
                )}
              </Card>
            </Col>

            {/* Network Metrics */}
            <Col span={12}>
              <Card title="Network Health" loading={loading}>
                {networkMetrics && (
                  <Row gutter={16}>
                    <Col span={12}>
                      <Statistic
                        title="Active Nodes"
                        value={networkMetrics.nodeCount}
                        prefix={<DatabaseOutlined />}
                      />
                    </Col>
                    <Col span={12}>
                      <Statistic
                        title="Peer Connections"
                        value={networkMetrics.peerConnections}
                        prefix={<NodeIndexOutlined />}
                      />
                    </Col>
                    <Col span={12}>
                      <Statistic
                        title="Network Latency"
                        value={networkMetrics.networkLatency}
                        suffix="ms"
                        valueStyle={{
                          color: networkMetrics.networkLatency < 100 ? '#3f8600' : '#cf1322',
                        }}
                      />
                    </Col>
                    <Col span={12}>
                      <Statistic
                        title="Sync Status"
                        value={networkMetrics.syncStatus}
                        suffix="%"
                        prefix={<CheckCircleOutlined />}
                      />
                    </Col>
                  </Row>
                )}
              </Card>
            </Col>

            {/* Transaction Metrics */}
            <Col span={12}>
              <Card title="Transaction Activity" loading={loading}>
                {transactionMetrics && (
                  <Row gutter={16}>
                    <Col span={12}>
                      <Statistic
                        title="Total Transactions"
                        value={transactionMetrics.totalTransactions}
                        prefix={<ThunderboltOutlined />}
                      />
                    </Col>
                    <Col span={12}>
                      <Statistic
                        title="Pending"
                        value={transactionMetrics.pendingTransactions}
                        prefix={<ClockCircleOutlined />}
                      />
                    </Col>
                    <Col span={12}>
                      <Statistic
                        title="Throughput"
                        value={transactionMetrics.throughput}
                        suffix="TPS"
                        prefix={<LineChartOutlined />}
                      />
                    </Col>
                    <Col span={12}>
                      <Statistic
                        title="Avg Confirmation"
                        value={transactionMetrics.averageConfirmationTime}
                        suffix="ms"
                      />
                    </Col>
                  </Row>
                )}
              </Card>
            </Col>

            {/* ZK Proof Metrics */}
            <Col span={12}>
              <Card title="ZK Proof System" loading={loading}>
                {zkProofMetrics && (
                  <Row gutter={16}>
                    <Col span={12}>
                      <Statistic
                        title="Proofs Generated"
                        value={zkProofMetrics.proofsGenerated}
                        prefix={<SecurityScanOutlined />}
                      />
                    </Col>
                    <Col span={12}>
                      <Statistic
                        title="Verification Rate"
                        value={zkProofMetrics.verificationRate}
                        suffix="%"
                        valueStyle={{
                          color: zkProofMetrics.verificationRate > 95 ? '#3f8600' : '#cf1322',
                        }}
                      />
                    </Col>
                    <Col span={12}>
                      <Statistic
                        title="Avg Proof Time"
                        value={zkProofMetrics.averageProofTime}
                        suffix="ms"
                      />
                    </Col>
                    <Col span={12}>
                      <Statistic
                        title="Failed Proofs"
                        value={zkProofMetrics.failedProofs}
                        prefix={<ExclamationCircleOutlined />}
                        valueStyle={{ color: '#cf1322' }}
                      />
                    </Col>
                  </Row>
                )}
              </Card>
            </Col>
          </Row>
        </TabPane>

        {/* Validators Tab */}
        <TabPane tab="Validators" key="validators">
          <Row gutter={[16, 16]}>
            <Col span={24}>
              <Card title="Validator Overview" loading={loading}>
                {validatorMetrics && (
                  <Row gutter={16}>
                    <Col span={6}>
                      <Statistic
                        title="Total Validators"
                        value={validatorMetrics.totalValidators}
                        prefix={<SecurityScanOutlined />}
                      />
                    </Col>
                    <Col span={6}>
                      <Statistic
                        title="Active Validators"
                        value={validatorMetrics.activeValidators}
                        prefix={<CheckCircleOutlined />}
                      />
                    </Col>
                    <Col span={6}>
                      <Statistic
                        title="Staking Ratio"
                        value={validatorMetrics.stakingRatio}
                        suffix="%"
                        prefix={<WalletOutlined />}
                      />
                    </Col>
                    <Col span={6}>
                      <Statistic
                        title="Average Stake"
                        value={validatorMetrics.averageStake}
                        suffix=" BPI"
                        prefix={<TrophyOutlined />}
                      />
                    </Col>
                  </Row>
                )}
              </Card>
            </Col>

            <Col span={24}>
              <Card title="Top Validators" loading={loading}>
                {validatorMetrics && (
                  <Table
                    dataSource={validatorMetrics.topValidators}
                    columns={validatorColumns}
                    rowKey="id"
                    pagination={{ pageSize: 10 }}
                    size="small"
                  />
                )}
              </Card>
            </Col>
          </Row>
        </TabPane>

        {/* Auctions Tab */}
        <TabPane tab="Auctions" key="auctions">
          <Row gutter={[16, 16]}>
            <Col span={24}>
              <Card title="Auction Activity" loading={loading}>
                {auctionMetrics && (
                  <Row gutter={16}>
                    <Col span={6}>
                      <Statistic
                        title="Active Auctions"
                        value={auctionMetrics.activeAuctions}
                        prefix={<ThunderboltOutlined />}
                      />
                    </Col>
                    <Col span={6}>
                      <Statistic
                        title="Completed Auctions"
                        value={auctionMetrics.completedAuctions}
                        prefix={<CheckCircleOutlined />}
                      />
                    </Col>
                    <Col span={6}>
                      <Statistic
                        title="Total Auction Value"
                        value={auctionMetrics.totalAuctionValue}
                        prefix="$"
                        suffix="M"
                      />
                    </Col>
                    <Col span={6}>
                      <Statistic
                        title="Participants"
                        value={auctionMetrics.participantCount}
                        prefix={<NodeIndexOutlined />}
                      />
                    </Col>
                  </Row>
                )}
              </Card>
            </Col>

            <Col span={24}>
              <Card title="Recent Auctions" loading={loading}>
                {auctionMetrics && (
                  <List
                    dataSource={auctionMetrics.recentAuctions}
                    renderItem={(auction) => (
                      <List.Item>
                        <List.Item.Meta
                          avatar={
                            <Avatar
                              icon={auction.type === 'government' ? <SecurityScanOutlined /> : <NodeIndexOutlined />}
                              style={{
                                backgroundColor: auction.type === 'government' ? '#1890ff' : '#52c41a',
                              }}
                            />
                          }
                          title={
                            <Space>
                              <Text strong>{auction.id}</Text>
                              <Tag color={auction.type === 'government' ? 'blue' : 'green'}>
                                {auction.type.toUpperCase()}
                              </Tag>
                              <Badge
                                status={
                                  auction.status === 'active' ? 'processing' :
                                  auction.status === 'completed' ? 'success' : 'error'
                                }
                                text={auction.status.toUpperCase()}
                              />
                            </Space>
                          }
                          description={
                            <Space>
                              <Text>Value: ${auction.value.toLocaleString()}</Text>
                              <Text>Participants: {auction.participants}</Text>
                            </Space>
                          }
                        />
                      </List.Item>
                    )}
                  />
                )}
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
                      <Option value="blockHeight">Block Height</Option>
                      <Option value="transactions">Transactions</Option>
                      <Option value="latency">Network Latency</Option>
                      <Option value="validators">Validators</Option>
                      <Option value="tvl">Total Value Locked</Option>
                    </Select>
                  }
                >
                  {historicalData.length > 0 && (
                    <Line {...getHistoricalChartConfig()} height={400} />
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

export default BpiMetricsDashboard;
