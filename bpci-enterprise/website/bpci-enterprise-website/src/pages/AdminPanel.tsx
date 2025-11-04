import React, { useState } from 'react';
import {
  Card,
  Row,
  Col,
  Typography,
  Space,
  Statistic,
  Table,
  Tag,
  Button,
  Tabs,
  Badge,
  Switch
} from 'antd';
import {
  CrownOutlined,
  UserOutlined,
  DashboardOutlined,
  SettingOutlined,
  BarChartOutlined,
  TeamOutlined,
  CheckCircleOutlined,
  CloseCircleOutlined
} from '@ant-design/icons';

const { Title, Text } = Typography;
const { TabPane } = Tabs;

interface User {
  id: string;
  email: string;
  plan: string;
  status: 'active' | 'suspended';
  nodes: number;
  balance: number;
  joined: string;
}

const AdminPanel: React.FC = () => {
  const [users] = useState<User[]>([
    {
      id: '1',
      email: 'alice@pravyom.wallet',
      plan: 'Enterprise',
      status: 'active',
      nodes: 5,
      balance: 2500,
      joined: '2025-01-15'
    },
    {
      id: '2',
      email: 'bob@pravyom.wallet',
      plan: 'Testnet',
      status: 'active',
      nodes: 2,
      balance: 500,
      joined: '2025-02-20'
    }
  ]);

  const userColumns = [
    {
      title: 'User',
      dataIndex: 'email',
      key: 'email',
      render: (email: string) => (
        <Space>
          <UserOutlined />
          <Text>{email}</Text>
        </Space>
      )
    },
    {
      title: 'Plan',
      dataIndex: 'plan',
      key: 'plan',
      render: (plan: string) => <Tag color="blue">{plan}</Tag>
    },
    {
      title: 'Status',
      dataIndex: 'status',
      key: 'status',
      render: (status: string) => (
        <Tag color={status === 'active' ? 'success' : 'error'} icon={status === 'active' ? <CheckCircleOutlined /> : <CloseCircleOutlined />}>
          {status.toUpperCase()}
        </Tag>
      )
    },
    {
      title: 'Nodes',
      dataIndex: 'nodes',
      key: 'nodes',
      render: (nodes: number) => <Badge count={nodes} showZero color="#1890ff" />
    },
    {
      title: 'Balance',
      dataIndex: 'balance',
      key: 'balance',
      render: (balance: number) => <Text>{balance} BPI</Text>
    },
    {
      title: 'Joined',
      dataIndex: 'joined',
      key: 'joined'
    },
    {
      title: 'Actions',
      key: 'actions',
      render: (_: any, record: User) => (
        <Space>
          <Button size="small">Edit</Button>
          <Button size="small" danger={record.status === 'active'}>
            {record.status === 'active' ? 'Suspend' : 'Activate'}
          </Button>
        </Space>
      )
    }
  ];

  return (
    <div style={{ padding: '24px', maxWidth: '1600px', margin: '0 auto' }}>
      <div style={{ marginBottom: 24 }}>
        <Space>
          <CrownOutlined style={{ fontSize: '32px', color: '#faad14' }} />
          <div>
            <Title level={2} style={{ margin: 0 }}>Admin Panel</Title>
            <Text type="secondary">Manage BPCI Enterprise platform</Text>
          </div>
        </Space>
      </div>

      {/* System Stats */}
      <Row gutter={[16, 16]} style={{ marginBottom: 24 }}>
        <Col xs={24} sm={12} md={6}>
          <Card>
            <Statistic
              title="Total Users"
              value={users.length}
              prefix={<TeamOutlined />}
              valueStyle={{ color: '#1890ff' }}
            />
          </Card>
        </Col>
        <Col xs={24} sm={12} md={6}>
          <Card>
            <Statistic
              title="Active Nodes"
              value={users.reduce((sum, u) => sum + u.nodes, 0)}
              prefix={<DashboardOutlined />}
              valueStyle={{ color: '#52c41a' }}
            />
          </Card>
        </Col>
        <Col xs={24} sm={12} md={6}>
          <Card>
            <Statistic
              title="Total Transactions"
              value={11234}
              prefix={<BarChartOutlined />}
              valueStyle={{ color: '#722ed1' }}
            />
          </Card>
        </Col>
        <Col xs={24} sm={12} md={6}>
          <Card>
            <Statistic
              title="System Health"
              value="99.9%"
              prefix={<CheckCircleOutlined />}
              valueStyle={{ color: '#52c41a' }}
            />
          </Card>
        </Col>
      </Row>

      {/* Admin Tabs */}
      <Card>
        <Tabs defaultActiveKey="users">
          <TabPane tab={<Space><UserOutlined />Users</Space>} key="users">
            <Table
              columns={userColumns}
              dataSource={users}
              rowKey="id"
              pagination={{ pageSize: 10 }}
            />
          </TabPane>
          
          <TabPane tab={<Space><DashboardOutlined />Nodes</Space>} key="nodes">
            <Text type="secondary">Node management interface</Text>
          </TabPane>
          
          <TabPane tab={<Space><SettingOutlined />System Settings</Space>} key="settings">
            <Space direction="vertical" style={{ width: '100%' }}>
              <Card>
                <Space style={{ width: '100%', justifyContent: 'space-between' }}>
                  <div>
                    <Text strong>Maintenance Mode</Text>
                    <br />
                    <Text type="secondary" style={{ fontSize: '12px' }}>
                      Enable maintenance mode for system updates
                    </Text>
                  </div>
                  <Switch />
                </Space>
              </Card>
              <Card>
                <Space style={{ width: '100%', justifyContent: 'space-between' }}>
                  <div>
                    <Text strong>Auto-Scaling</Text>
                    <br />
                    <Text type="secondary" style={{ fontSize: '12px' }}>
                      Automatically scale BSO-K8 resources
                    </Text>
                  </div>
                  <Switch defaultChecked />
                </Space>
              </Card>
            </Space>
          </TabPane>
          
          <TabPane tab={<Space><BarChartOutlined />Analytics</Space>} key="analytics">
            <Text type="secondary">System analytics and metrics</Text>
          </TabPane>
        </Tabs>
      </Card>
    </div>
  );
};

export default AdminPanel;
