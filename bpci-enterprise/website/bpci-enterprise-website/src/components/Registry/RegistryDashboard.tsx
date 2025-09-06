import React, { useState, useEffect } from 'react';
import { Card, Row, Col, Statistic, Table, Tag, Button, Space, Typography, Alert, Spin } from 'antd';
import { 
  NodeIndexOutlined, 
  WalletOutlined, 
  SafetyOutlined, 
  CloudServerOutlined,
  ReloadOutlined,
  EyeOutlined
} from '@ant-design/icons';
import { registryService, RegistryStats, NodeInfo, WalletInfo } from '../../services/registryService';

const { Title, Text } = Typography;

export const RegistryDashboard: React.FC = () => {
  const [stats, setStats] = useState<RegistryStats | null>(null);
  const [nodes, setNodes] = useState<NodeInfo[]>([]);
  const [wallets, setWallets] = useState<WalletInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchData = async () => {
    setLoading(true);
    setError(null);

    try {
      const [statsData, nodesData, walletsData] = await Promise.all([
        registryService.getRegistryStats(),
        registryService.getNodes(1, 10),
        registryService.getWallets(1, 10)
      ]);

      setStats(statsData);
      setNodes(nodesData?.nodes || []);
      setWallets(walletsData?.wallets || []);
    } catch (err) {
      setError('Failed to load registry data');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
    // Set up polling for real-time updates
    const interval = setInterval(fetchData, 30000); // Update every 30 seconds
    return () => clearInterval(interval);
  }, []);

  const getNodeTypeColor = (nodeType: string) => {
    const colors: Record<string, string> = {
      'BpiCommunity': 'blue',
      'BpciEnterprise': 'purple',
      'Hybrid': 'orange',
      'BankApiRegistry': 'green',
      'GovernmentApiRegistry': 'red'
    };
    return colors[nodeType] || 'default';
  };

  const getStatusColor = (status: string) => {
    const colors: Record<string, string> = {
      'Active': 'success',
      'Inactive': 'default',
      'Maintenance': 'warning'
    };
    return colors[status] || 'default';
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

  const nodeColumns = [
    {
      title: 'Node ID',
      dataIndex: 'node_id',
      key: 'node_id',
      render: (text: string) => <Text code>{text.substring(0, 12)}...</Text>
    },
    {
      title: 'Type',
      dataIndex: 'node_type',
      key: 'node_type',
      render: (type: string) => <Tag color={getNodeTypeColor(type)}>{type}</Tag>
    },
    {
      title: 'Status',
      dataIndex: 'status',
      key: 'status',
      render: (status: string) => <Tag color={getStatusColor(status)}>{status}</Tag>
    },
    {
      title: 'Name',
      dataIndex: ['identity', 'name'],
      key: 'name'
    },
    {
      title: 'Trust Score',
      dataIndex: ['authority', 'trust_score'],
      key: 'trust_score',
      render: (score: number) => (
        <Statistic 
          value={score} 
          suffix="/100" 
          valueStyle={{ fontSize: '14px' }}
        />
      )
    },
    {
      title: 'Uptime',
      dataIndex: 'uptime_percentage',
      key: 'uptime',
      render: (uptime: number) => (
        <Statistic 
          value={uptime} 
          suffix="%" 
          valueStyle={{ fontSize: '14px', color: uptime > 95 ? '#52c41a' : uptime > 80 ? '#faad14' : '#ff4d4f' }}
        />
      )
    },
    {
      title: 'Actions',
      key: 'actions',
      render: (_, record: NodeInfo) => (
        <Button 
          type="link" 
          icon={<EyeOutlined />}
          onClick={() => console.log('View node details:', record.node_id)}
        >
          Details
        </Button>
      )
    }
  ];

  const walletColumns = [
    {
      title: 'Wallet ID',
      dataIndex: 'wallet_id',
      key: 'wallet_id',
      render: (text: string) => <Text code>{text.substring(0, 12)}...</Text>
    },
    {
      title: 'Type',
      dataIndex: 'wallet_type',
      key: 'wallet_type',
      render: (type: string) => <Tag color={getWalletTypeColor(type)}>{type}</Tag>
    },
    {
      title: 'Status',
      dataIndex: 'status',
      key: 'status',
      render: (status: string) => <Tag color={getStatusColor(status)}>{status}</Tag>
    },
    {
      title: 'Balance (GEN)',
      dataIndex: ['balance', 'gen'],
      key: 'gen_balance',
      render: (balance: number) => (
        <Statistic 
          value={balance} 
          precision={2}
          valueStyle={{ fontSize: '14px' }}
        />
      )
    },
    {
      title: 'Mining Sessions',
      dataIndex: 'mining_sessions',
      key: 'mining_sessions'
    },
    {
      title: 'Total Rewards',
      dataIndex: 'total_rewards',
      key: 'total_rewards',
      render: (rewards: number) => (
        <Statistic 
          value={rewards} 
          precision={2}
          valueStyle={{ fontSize: '14px', color: '#52c41a' }}
        />
      )
    }
  ];

  if (loading && !stats) {
    return (
      <div style={{ textAlign: 'center', padding: '50px' }}>
        <Spin size="large" />
        <div style={{ marginTop: '16px' }}>Loading registry data...</div>
      </div>
    );
  }

  return (
    <div style={{ padding: '24px' }}>
      <div style={{ marginBottom: '24px', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <Title level={2} style={{ margin: 0 }}>
          <NodeIndexOutlined style={{ marginRight: '12px', color: '#667eea' }} />
          BPI Registry Dashboard
        </Title>
        <Button 
          type="primary" 
          icon={<ReloadOutlined />}
          onClick={fetchData}
          loading={loading}
        >
          Refresh
        </Button>
      </div>

      {error && (
        <Alert
          message="Error Loading Registry Data"
          description={error}
          type="error"
          showIcon
          style={{ marginBottom: '24px' }}
        />
      )}

      {stats && (
        <Row gutter={[16, 16]} style={{ marginBottom: '32px' }}>
          <Col xs={24} sm={12} md={6}>
            <Card>
              <Statistic
                title="Total Nodes"
                value={stats.total_nodes}
                prefix={<CloudServerOutlined />}
                valueStyle={{ color: '#3f8600' }}
              />
            </Card>
          </Col>
          <Col xs={24} sm={12} md={6}>
            <Card>
              <Statistic
                title="Active Nodes"
                value={stats.active_nodes}
                prefix={<SafetyOutlined />}
                valueStyle={{ color: '#cf1322' }}
              />
            </Card>
          </Col>
          <Col xs={24} sm={12} md={6}>
            <Card>
              <Statistic
                title="Enterprise Nodes"
                value={stats.enterprise_nodes}
                prefix={<NodeIndexOutlined />}
                valueStyle={{ color: '#722ed1' }}
              />
            </Card>
          </Col>
          <Col xs={24} sm={12} md={6}>
            <Card>
              <Statistic
                title="Network Health"
                value={stats.network_health}
                prefix={<SafetyOutlined />}
                valueStyle={{ 
                  color: stats.network_health === 'Excellent' ? '#52c41a' :
                         stats.network_health === 'Good' ? '#1890ff' :
                         stats.network_health === 'Fair' ? '#faad14' : '#ff4d4f'
                }}
              />
            </Card>
          </Col>
        </Row>
      )}

      <Row gutter={[16, 16]} style={{ marginBottom: '32px' }}>
        <Col xs={24} sm={12} md={8}>
          <Card>
            <Statistic
              title="Community Nodes"
              value={stats?.community_nodes || 0}
              valueStyle={{ color: '#1890ff' }}
            />
          </Card>
        </Col>
        <Col xs={24} sm={12} md={8}>
          <Card>
            <Statistic
              title="Bank Nodes"
              value={stats?.bank_nodes || 0}
              valueStyle={{ color: '#52c41a' }}
            />
          </Card>
        </Col>
        <Col xs={24} sm={12} md={8}>
          <Card>
            <Statistic
              title="Government Nodes"
              value={stats?.government_nodes || 0}
              valueStyle={{ color: '#ff4d4f' }}
            />
          </Card>
        </Col>
      </Row>

      <Row gutter={[16, 16]}>
        <Col xs={24} lg={14}>
          <Card 
            title={
              <Space>
                <NodeIndexOutlined />
                <span>Active Nodes</span>
                <Tag color="blue">{nodes.length}</Tag>
              </Space>
            }
            extra={
              <Button type="link" onClick={() => console.log('View all nodes')}>
                View All
              </Button>
            }
          >
            <Table
              dataSource={nodes}
              columns={nodeColumns}
              rowKey="node_id"
              pagination={false}
              size="small"
              scroll={{ x: 800 }}
            />
          </Card>
        </Col>
        <Col xs={24} lg={10}>
          <Card 
            title={
              <Space>
                <WalletOutlined />
                <span>Active Wallets</span>
                <Tag color="green">{wallets.length}</Tag>
              </Space>
            }
            extra={
              <Button type="link" onClick={() => console.log('View all wallets')}>
                View All
              </Button>
            }
          >
            <Table
              dataSource={wallets}
              columns={walletColumns}
              rowKey="wallet_id"
              pagination={false}
              size="small"
              scroll={{ x: 600 }}
            />
          </Card>
        </Col>
      </Row>

      {stats && (
        <div style={{ marginTop: '16px', textAlign: 'center' }}>
          <Text type="secondary">
            Last updated: {new Date(stats.last_updated).toLocaleString()}
          </Text>
        </div>
      )}
    </div>
  );
};
