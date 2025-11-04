import React, { useState, useEffect } from 'react';
import { Card, Row, Col, Typography, Space, Statistic, Select, DatePicker, Spin } from 'antd';
import {
  LineChartOutlined,
  DashboardOutlined,
  ThunderboltOutlined,
  DatabaseOutlined,
  CloudServerOutlined,
  ArrowUpOutlined,
  ArrowDownOutlined
} from '@ant-design/icons';

const { Title, Text } = Typography;
const { RangePicker } = DatePicker;
const { Option } = Select;

const AdvancedMetrics: React.FC = () => {
  const [loading, setLoading] = useState(false);
  const [timeRange, setTimeRange] = useState('24h');

  return (
    <div style={{ padding: '24px', maxWidth: '1600px', margin: '0 auto' }}>
      <div style={{ marginBottom: 24 }}>
        <Space align="center" style={{ width: '100%', justifyContent: 'space-between' }}>
          <Space>
            <LineChartOutlined style={{ fontSize: '32px', color: '#1890ff' }} />
            <div>
              <Title level={2} style={{ margin: 0 }}>Advanced Metrics</Title>
              <Text type="secondary">Real-time monitoring and analytics</Text>
            </div>
          </Space>
          <Space>
            <Select value={timeRange} onChange={setTimeRange} style={{ width: 120 }}>
              <Option value="1h">Last Hour</Option>
              <Option value="24h">Last 24h</Option>
              <Option value="7d">Last 7 Days</Option>
              <Option value="30d">Last 30 Days</Option>
            </Select>
          </Space>
        </Space>
      </div>

      <Row gutter={[16, 16]}>
        <Col xs={24} sm={12} md={6}>
          <Card>
            <Statistic
              title="Total Transactions"
              value={11234}
              prefix={<ThunderboltOutlined />}
              suffix={<ArrowUpOutlined style={{ color: '#52c41a', fontSize: '14px' }} />}
              valueStyle={{ color: '#1890ff' }}
            />
            <Text type="secondary" style={{ fontSize: '12px' }}>+12.5% from last period</Text>
          </Card>
        </Col>
        <Col xs={24} sm={12} md={6}>
          <Card>
            <Statistic
              title="Network Throughput"
              value={1247}
              suffix="TPS"
              prefix={<DashboardOutlined />}
              valueStyle={{ color: '#52c41a' }}
            />
            <Text type="secondary" style={{ fontSize: '12px' }}>Target: 1,250 TPS</Text>
          </Card>
        </Col>
        <Col xs={24} sm={12} md={6}>
          <Card>
            <Statistic
              title="Active Nodes"
              value={156}
              prefix={<CloudServerOutlined />}
              valueStyle={{ color: '#722ed1' }}
            />
            <Text type="secondary" style={{ fontSize: '12px' }}>Across all clusters</Text>
          </Card>
        </Col>
        <Col xs={24} sm={12} md={6}>
          <Card>
            <Statistic
              title="Storage Used"
              value={2.4}
              suffix="TB"
              prefix={<DatabaseOutlined />}
              valueStyle={{ color: '#faad14' }}
            />
            <Text type="secondary" style={{ fontSize: '12px' }}>of 10 TB total</Text>
          </Card>
        </Col>
      </Row>

      <Card style={{ marginTop: 24 }}>
        <Title level={5}>Performance Metrics</Title>
        <Text type="secondary">Detailed charts and analytics coming soon...</Text>
      </Card>
    </div>
  );
};

export default AdvancedMetrics;
