import React, { useState, useEffect } from 'react';
import {
  Card,
  Table,
  Typography,
  Space,
  Tag,
  Input,
  Select,
  DatePicker,
  Button,
  Spin,
  Tooltip
} from 'antd';
import {
  HistoryOutlined,
  SearchOutlined,
  DownloadOutlined,
  ArrowUpOutlined,
  ArrowDownOutlined,
  SwapOutlined,
  CheckCircleOutlined,
  ClockCircleOutlined,
  CloseCircleOutlined
} from '@ant-design/icons';

const { Title, Text } = Typography;
const { RangePicker } = DatePicker;
const { Option } = Select;

interface Transaction {
  tx_hash: string;
  type: 'send' | 'receive' | 'swap';
  from: string;
  to: string;
  amount: number;
  coin: string;
  status: 'confirmed' | 'pending' | 'failed';
  timestamp: string;
  block_height: number;
  gas_fee: number;
}

const TransactionHistory: React.FC = () => {
  const [loading, setLoading] = useState(true);
  const [transactions, setTransactions] = useState<Transaction[]>([]);
  const [filter, setFilter] = useState('all');

  useEffect(() => {
    loadTransactions();
  }, []);

  const loadTransactions = async () => {
    setLoading(true);
    try {
      const API_BASE = process.env.REACT_APP_API_URL || 'http://146.190.74.139:8080';
      const response = await fetch(`${API_BASE}/api/wallet/transactions`, {
        method: 'GET',
        headers: { 'Content-Type': 'application/json' }
      });
      
      if (response.ok) {
        const data = await response.json();
        setTransactions(data.data?.transactions || getDemoTransactions());
      } else {
        setTransactions(getDemoTransactions());
      }
    } catch (error) {
      setTransactions(getDemoTransactions());
    } finally {
      setLoading(false);
    }
  };

  const getDemoTransactions = (): Transaction[] => [
    {
      tx_hash: '0x7a3f...b2e9',
      type: 'receive',
      from: 'bpi:wallet:abc123',
      to: 'bpi:wallet:def456',
      amount: 500,
      coin: 'GEN',
      status: 'confirmed',
      timestamp: new Date(Date.now() - 3600000).toISOString(),
      block_height: 12345,
      gas_fee: 0.001
    },
    {
      tx_hash: '0x9c2d...f4a1',
      type: 'send',
      from: 'bpi:wallet:def456',
      to: 'bpi:wallet:ghi789',
      amount: 100,
      coin: 'NEX',
      status: 'confirmed',
      timestamp: new Date(Date.now() - 7200000).toISOString(),
      block_height: 12344,
      gas_fee: 0.0005
    }
  ];

  const columns = [
    {
      title: 'Type',
      dataIndex: 'type',
      key: 'type',
      render: (type: string) => {
        const icons = {
          send: <ArrowUpOutlined style={{ color: '#1890ff' }} />,
          receive: <ArrowDownOutlined style={{ color: '#52c41a' }} />,
          swap: <SwapOutlined style={{ color: '#722ed1' }} />
        };
        return <Space>{icons[type as keyof typeof icons]} <Text>{type}</Text></Space>;
      }
    },
    {
      title: 'Amount',
      dataIndex: 'amount',
      key: 'amount',
      render: (amount: number, record: Transaction) => (
        <Space>
          <Text strong>{amount}</Text>
          <Tag color="blue">{record.coin}</Tag>
        </Space>
      )
    },
    {
      title: 'From/To',
      key: 'address',
      render: (_: any, record: Transaction) => (
        <Tooltip title={record.type === 'send' ? record.to : record.from}>
          <Text code style={{ fontSize: '12px' }}>
            {(record.type === 'send' ? record.to : record.from).substring(0, 20)}...
          </Text>
        </Tooltip>
      )
    },
    {
      title: 'Status',
      dataIndex: 'status',
      key: 'status',
      render: (status: string) => {
        const config = {
          confirmed: { color: 'success', icon: <CheckCircleOutlined /> },
          pending: { color: 'processing', icon: <ClockCircleOutlined /> },
          failed: { color: 'error', icon: <CloseCircleOutlined /> }
        };
        const { color, icon } = config[status as keyof typeof config];
        return <Tag color={color} icon={icon}>{status.toUpperCase()}</Tag>;
      }
    },
    {
      title: 'Time',
      dataIndex: 'timestamp',
      key: 'timestamp',
      render: (timestamp: string) => new Date(timestamp).toLocaleString()
    },
    {
      title: 'Tx Hash',
      dataIndex: 'tx_hash',
      key: 'tx_hash',
      render: (hash: string) => (
        <Tooltip title={hash}>
          <Text code copyable>{hash}</Text>
        </Tooltip>
      )
    }
  ];

  return (
    <div style={{ padding: '24px', maxWidth: '1600px', margin: '0 auto' }}>
      <div style={{ marginBottom: 24 }}>
        <Space align="center" style={{ width: '100%', justifyContent: 'space-between' }}>
          <Space>
            <HistoryOutlined style={{ fontSize: '32px', color: '#1890ff' }} />
            <div>
              <Title level={2} style={{ margin: 0 }}>Transaction History</Title>
              <Text type="secondary">View all your transactions</Text>
            </div>
          </Space>
          <Button icon={<DownloadOutlined />}>Export CSV</Button>
        </Space>
      </div>

      <Card style={{ marginBottom: 24 }}>
        <Space wrap style={{ width: '100%' }}>
          <Input
            placeholder="Search by hash or address"
            prefix={<SearchOutlined />}
            style={{ width: 300 }}
          />
          <Select value={filter} onChange={setFilter} style={{ width: 150 }}>
            <Option value="all">All Types</Option>
            <Option value="send">Send</Option>
            <Option value="receive">Receive</Option>
            <Option value="swap">Swap</Option>
          </Select>
          <RangePicker />
        </Space>
      </Card>

      <Card>
        {loading ? (
          <div style={{ textAlign: 'center', padding: '60px 0' }}>
            <Spin size="large" />
          </div>
        ) : (
          <Table
            columns={columns}
            dataSource={transactions}
            rowKey="tx_hash"
            pagination={{ pageSize: 20 }}
            scroll={{ x: 1200 }}
          />
        )}
      </Card>
    </div>
  );
};

export default TransactionHistory;
