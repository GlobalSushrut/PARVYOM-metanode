import React, { useState, useEffect } from 'react';
import {
  Card,
  Row,
  Col,
  Button,
  Typography,
  Space,
  Table,
  Tag,
  Statistic,
  Progress,
  Spin,
  Alert,
  Tabs,
  Badge
} from 'antd';
import {
  DashboardOutlined,
  PlayCircleOutlined,
  PauseCircleOutlined,
  StopOutlined,
  ReloadOutlined,
  SettingOutlined,
  FileTextOutlined,
  BarChartOutlined,
  CheckCircleOutlined,
  CloseCircleOutlined,
  ClockCircleOutlined
} from '@ant-design/icons';

const { Title, Text } = Typography;
const { TabPane } = Tabs;

interface DeployedNode {
  node_id: string;
  name: string;
  type: string;
  status: 'running' | 'stopped' | 'error' | 'starting';
  uptime: string;
  cpu_usage: number;
  memory_usage: number;
  network_in: string;
  network_out: string;
  vpods: number;
  endpoint: string;
}

const NodeManagementDashboard: React.FC = () => {
  const [loading, setLoading] = useState(true);
  const [nodes, setNodes] = useState<DeployedNode[]>([]);
  const [selectedNode, setSelectedNode] = useState<string | null>(null);

  useEffect(() => {
    loadDeployedNodes();
  }, []);

  const loadDeployedNodes = async () => {
    setLoading(true);
    
    try {
      // Real backend integration - Component 6 (Cluster Ledger) via Component 9 (Web)
      const API_BASE = process.env.REACT_APP_API_URL || 'http://146.190.74.139:8080';
      
      const response = await fetch(`${API_BASE}/api/nodes/deployed`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json'
        }
      });
      
      if (response.ok) {
        const data = await response.json();
        
        if (data.data?.nodes) {
          setNodes(data.data.nodes);
        } else {
          // Fallback demo data
          setNodes(getDemoNodes());
        }
      } else {
        setNodes(getDemoNodes());
      }
    } catch (error) {
      console.error('Failed to load deployed nodes:', error);
      setNodes(getDemoNodes());
    } finally {
      setLoading(false);
    }
  };

  const getDemoNodes = (): DeployedNode[] => [
    {
      node_id: 'node_abc123',
      name: 'bpi-mainnet-01',
      type: 'Full Node',
      status: 'running',
      uptime: '15d 7h 23m',
      cpu_usage: 45,
      memory_usage: 62,
      network_in: '125 MB/s',
      network_out: '89 MB/s',
      vpods: 25,
      endpoint: 'http://159.203.101.136:7777'
    },
    {
      node_id: 'node_def456',
      name: 'bpi-validator-02',
      type: 'Validator',
      status: 'running',
      uptime: '8d 12h 45m',
      cpu_usage: 32,
      memory_usage: 48,
      network_in: '78 MB/s',
      network_out: '56 MB/s',
      vpods: 15,
      endpoint: 'http://159.203.101.136:7778'
    }
  ];

  const getStatusTag = (status: string) => {
    const colors: Record<string, string> = {
      running: 'success',
      stopped: 'default',
      error: 'error',
      starting: 'processing'
    };
    const icons: Record<string, React.ReactNode> = {
      running: <CheckCircleOutlined />,
      stopped: <CloseCircleOutlined />,
      error: <CloseCircleOutlined />,
      starting: <ClockCircleOutlined />
    };
    
    return (
      <Tag color={colors[status]} icon={icons[status]}>
        {status.toUpperCase()}
      </Tag>
    );
  };

  const columns = [
    {
      title: 'Node Name',
      dataIndex: 'name',
      key: 'name',
      render: (text: string, record: DeployedNode) => (
        <Space direction="vertical" size={0}>
          <Text strong>{text}</Text>
          <Text type="secondary" style={{ fontSize: '12px' }}>{record.node_id}</Text>
        </Space>
      )
    },
    {
      title: 'Type',
      dataIndex: 'type',
      key: 'type',
      render: (type: string) => <Tag color="blue">{type}</Tag>
    },
    {
      title: 'Status',
      dataIndex: 'status',
      key: 'status',
      render: (status: string) => getStatusTag(status)
    },
    {
      title: 'Uptime',
      dataIndex: 'uptime',
      key: 'uptime'
    },
    {
      title: 'CPU',
      dataIndex: 'cpu_usage',
      key: 'cpu_usage',
      render: (usage: number) => (
        <Progress 
          percent={usage} 
          size="small" 
          status={usage > 80 ? 'exception' : 'normal'}
        />
      )
    },
    {
      title: 'Memory',
      dataIndex: 'memory_usage',
      key: 'memory_usage',
      render: (usage: number) => (
        <Progress 
          percent={usage} 
          size="small" 
          status={usage > 80 ? 'exception' : 'normal'}
        />
      )
    },
    {
      title: 'vPods',
      dataIndex: 'vpods',
      key: 'vpods',
      render: (vpods: number) => <Badge count={vpods} showZero color="#1890ff" />
    },
    {
      title: 'Actions',
      key: 'actions',
      render: (_: any, record: DeployedNode) => (
        <Space>
          {record.status === 'running' ? (
            <Button 
              size="small" 
              icon={<PauseCircleOutlined />}
              onClick={() => handleNodeAction(record.node_id, 'pause')}
            >
              Pause
            </Button>
          ) : (
            <Button 
              size="small" 
              type="primary"
              icon={<PlayCircleOutlined />}
              onClick={() => handleNodeAction(record.node_id, 'start')}
            >
              Start
            </Button>
          )}
          <Button 
            size="small" 
            icon={<ReloadOutlined />}
            onClick={() => handleNodeAction(record.node_id, 'restart')}
          >
            Restart
          </Button>
          <Button 
            size="small" 
            danger
            icon={<StopOutlined />}
            onClick={() => handleNodeAction(record.node_id, 'stop')}
          >
            Stop
          </Button>
        </Space>
      )
    }
  ];

  const handleNodeAction = async (nodeId: string, action: string) => {
    console.log(`Performing ${action} on node ${nodeId}`);
    // Real backend integration would go here
  };

  if (loading) {
    return (
      <div style={{ textAlign: 'center', padding: '100px 0' }}>
        <Spin size="large" />
        <Text style={{ display: 'block', marginTop: 16 }}>Loading deployed nodes...</Text>
      </div>
    );
  }

  return (
    <div style={{ padding: '24px', maxWidth: '1600px', margin: '0 auto' }}>
      {/* Header */}
      <div style={{ marginBottom: 24 }}>
        <Space align="center" style={{ width: '100%', justifyContent: 'space-between' }}>
          <Space>
            <DashboardOutlined style={{ fontSize: '32px', color: '#1890ff' }} />
            <div>
              <Title level={2} style={{ margin: 0 }}>Node Management</Title>
              <Text type="secondary">Manage your deployed BPI OS nodes</Text>
            </div>
          </Space>
          <Button 
            type="primary" 
            size="large"
            icon={<PlayCircleOutlined />}
            onClick={() => window.location.href = '/deploy-node'}
          >
            Deploy New Node
          </Button>
        </Space>
      </div>

      {/* Stats Overview */}
      <Row gutter={[16, 16]} style={{ marginBottom: 24 }}>
        <Col xs={24} sm={12} md={6}>
          <Card>
            <Statistic
              title="Total Nodes"
              value={nodes.length}
              prefix={<DashboardOutlined />}
              valueStyle={{ color: '#1890ff' }}
            />
          </Card>
        </Col>
        <Col xs={24} sm={12} md={6}>
          <Card>
            <Statistic
              title="Running Nodes"
              value={nodes.filter(n => n.status === 'running').length}
              prefix={<CheckCircleOutlined />}
              valueStyle={{ color: '#52c41a' }}
            />
          </Card>
        </Col>
        <Col xs={24} sm={12} md={6}>
          <Card>
            <Statistic
              title="Total vPods"
              value={nodes.reduce((sum, n) => sum + n.vpods, 0)}
              prefix={<BarChartOutlined />}
              valueStyle={{ color: '#722ed1' }}
            />
          </Card>
        </Col>
        <Col xs={24} sm={12} md={6}>
          <Card>
            <Statistic
              title="Avg CPU Usage"
              value={Math.round(nodes.reduce((sum, n) => sum + n.cpu_usage, 0) / nodes.length)}
              suffix="%"
              valueStyle={{ color: '#faad14' }}
            />
          </Card>
        </Col>
      </Row>

      {/* Nodes Table */}
      <Card
        title={
          <Space>
            <DashboardOutlined />
            <Text strong>Deployed Nodes</Text>
          </Space>
        }
        extra={
          <Button icon={<ReloadOutlined />} onClick={loadDeployedNodes}>
            Refresh
          </Button>
        }
      >
        {nodes.length === 0 ? (
          <Alert
            message="No Nodes Deployed"
            description="You haven't deployed any BPI OS nodes yet. Click the 'Deploy New Node' button to get started."
            type="info"
            showIcon
            action={
              <Button type="primary" onClick={() => window.location.href = '/deploy-node'}>
                Deploy Node
              </Button>
            }
          />
        ) : (
          <Table 
            columns={columns} 
            dataSource={nodes}
            rowKey="node_id"
            pagination={{ pageSize: 10 }}
            scroll={{ x: 1200 }}
          />
        )}
      </Card>

      {/* Quick Actions */}
      <Card style={{ marginTop: 24, background: '#f0f2f5', border: 'none' }}>
        <Space direction="vertical" size="small" style={{ width: '100%' }}>
          <Title level={5}>Quick Actions</Title>
          <Space wrap>
            <Button icon={<FileTextOutlined />}>View Logs</Button>
            <Button icon={<BarChartOutlined />}>View Metrics</Button>
            <Button icon={<SettingOutlined />}>Configure Nodes</Button>
            <Button icon={<ReloadOutlined />}>Restart All</Button>
          </Space>
        </Space>
      </Card>
    </div>
  );
};

export default NodeManagementDashboard;
