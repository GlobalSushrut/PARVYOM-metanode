import React, { useState, useEffect } from 'react';
import { Card, Table, Tag, Typography, Space, Button, Input, Select, DatePicker, Row, Col } from 'antd';
import {
  HistoryOutlined,
  SearchOutlined,
  DownloadOutlined,
  FilterOutlined,
  ArrowUpOutlined,
  ArrowDownOutlined,
  SwapOutlined
} from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import axios from 'axios';

const { Title, Text } = Typography;
const { RangePicker } = DatePicker;

const API_BASE_URL = 'http://127.0.0.1:8081/api';

interface Transaction {
  key: string;
  tx_hash: string;
  type: 'send' | 'receive' | 'stake' | 'unstake';
  amount: number;
  from: string;
  to: string;
  status: 'confirmed' | 'pending' | 'failed';
  timestamp: string;
  block: number;
  fee: number;
}

const Transactions: React.FC = () => {
  const [transactions, setTransactions] = useState<Transaction[]>([]);
  const [loading, setLoading] = useState(false);
  const [searchText, setSearchText] = useState('');
  const [filterType, setFilterType] = useState<string>('all');

  useEffect(() => {
    fetchTransactions();
  }, []);

  const fetchTransactions = async () => {
    setLoading(true);
    try {
      // For now, generate mock transactions based on blockchain data
      // In production, this would call a real backend endpoint
      const response = await axios.get(`${API_BASE_URL}/wallet/status?wallet_id=default`);
      
      if (response.data.status === 'ok' && response.data.data) {
        const walletData = response.data.data;
        const txCount = walletData.transaction_count || 5;
        
        // Generate sample transactions
        const mockTxs: Transaction[] = Array.from({ length: Math.min(txCount, 20) }, (_, i) => ({
          key: `tx-${i}`,
          tx_hash: `0x${Math.random().toString(16).substring(2, 18)}...${Math.random().toString(16).substring(2, 6)}`,
          type: ['send', 'receive', 'stake', 'unstake'][Math.floor(Math.random() * 4)] as any,
          amount: parseFloat((Math.random() * 100).toFixed(2)),
          from: i % 2 === 0 ? walletData.address : `0x${Math.random().toString(16).substring(2, 18)}`,
          to: i % 2 === 1 ? walletData.address : `0x${Math.random().toString(16).substring(2, 18)}`,
          status: ['confirmed', 'pending'][Math.floor(Math.random() * 2)] as any,
          timestamp: new Date(Date.now() - i * 3600000).toISOString(),
          block: walletData.current_block - i,
          fee: parseFloat((Math.random() * 0.01).toFixed(4))
        }));
        
        setTransactions(mockTxs);
      }
    } catch (error) {
      console.error('Failed to fetch transactions:', error);
    } finally {
      setLoading(false);
    }
  };

  const getTypeIcon = (type: string) => {
    switch (type) {
      case 'send':
        return <ArrowUpOutlined style={{ color: '#EF4444' }} />;
      case 'receive':
        return <ArrowDownOutlined style={{ color: '#10B981' }} />;
      case 'stake':
      case 'unstake':
        return <SwapOutlined style={{ color: '#3B82F6' }} />;
      default:
        return null;
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'confirmed':
        return 'success';
      case 'pending':
        return 'processing';
      case 'failed':
        return 'error';
      default:
        return 'default';
    }
  };

  const columns: ColumnsType<Transaction> = [
    {
      title: 'Type',
      dataIndex: 'type',
      key: 'type',
      width: 100,
      render: (type: string) => (
        <Space>
          {getTypeIcon(type)}
          <Text style={{ color: '#ffffff', textTransform: 'capitalize' }}>{type}</Text>
        </Space>
      ),
    },
    {
      title: 'Transaction Hash',
      dataIndex: 'tx_hash',
      key: 'tx_hash',
      render: (hash: string) => (
        <Text style={{ color: '#E8B44F', fontFamily: 'monospace', fontSize: '0.875rem' }}>
          {hash}
        </Text>
      ),
    },
    {
      title: 'Amount',
      dataIndex: 'amount',
      key: 'amount',
      width: 150,
      render: (amount: number, record: Transaction) => (
        <Text style={{ 
          color: record.type === 'receive' ? '#10B981' : '#ffffff',
          fontWeight: 'bold'
        }}>
          {record.type === 'receive' ? '+' : '-'}{amount.toFixed(2)} BPI
        </Text>
      ),
    },
    {
      title: 'Status',
      dataIndex: 'status',
      key: 'status',
      width: 120,
      render: (status: string) => (
        <Tag color={getStatusColor(status)} style={{ textTransform: 'capitalize' }}>
          {status}
        </Tag>
      ),
    },
    {
      title: 'Block',
      dataIndex: 'block',
      key: 'block',
      width: 100,
      render: (block: number) => (
        <Text style={{ color: '#9CA3AF' }}>#{block}</Text>
      ),
    },
    {
      title: 'Time',
      dataIndex: 'timestamp',
      key: 'timestamp',
      width: 180,
      render: (timestamp: string) => (
        <Text style={{ color: '#9CA3AF' }}>
          {new Date(timestamp).toLocaleString()}
        </Text>
      ),
    },
    {
      title: 'Fee',
      dataIndex: 'fee',
      key: 'fee',
      width: 100,
      render: (fee: number) => (
        <Text style={{ color: '#9CA3AF' }}>{fee.toFixed(4)} BPI</Text>
      ),
    },
  ];

  const filteredTransactions = transactions.filter(tx => {
    const matchesSearch = searchText === '' || 
      tx.tx_hash.toLowerCase().includes(searchText.toLowerCase()) ||
      tx.from.toLowerCase().includes(searchText.toLowerCase()) ||
      tx.to.toLowerCase().includes(searchText.toLowerCase());
    
    const matchesType = filterType === 'all' || tx.type === filterType;
    
    return matchesSearch && matchesType;
  });

  return (
    <div style={{ padding: '1.5rem' }}>
      {/* Header */}
      <div style={{ marginBottom: '2rem' }}>
        <Title level={2} style={{ color: '#E8B44F', marginBottom: '0.5rem' }}>
          <HistoryOutlined /> Transaction History
        </Title>
        <Text style={{ color: '#9CA3AF' }}>
          View all your BPI transactions on the blockchain
        </Text>
      </div>

      {/* Filters */}
      <Card
        style={{
          background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
          border: '1px solid rgba(232, 180, 79, 0.2)',
          borderRadius: '12px',
          marginBottom: '1.5rem'
        }}
      >
        <Row gutter={[16, 16]}>
          <Col xs={24} sm={12} md={8}>
            <Input
              placeholder="Search by hash or address..."
              prefix={<SearchOutlined style={{ color: '#9CA3AF' }} />}
              value={searchText}
              onChange={(e) => setSearchText(e.target.value)}
              style={{
                background: 'rgba(255, 255, 255, 0.05)',
                border: '1px solid rgba(232, 180, 79, 0.2)',
                color: '#ffffff'
              }}
            />
          </Col>
          <Col xs={24} sm={12} md={6}>
            <Select
              value={filterType}
              onChange={setFilterType}
              style={{ width: '100%' }}
              options={[
                { label: 'All Types', value: 'all' },
                { label: 'Send', value: 'send' },
                { label: 'Receive', value: 'receive' },
                { label: 'Stake', value: 'stake' },
                { label: 'Unstake', value: 'unstake' },
              ]}
            />
          </Col>
          <Col xs={24} sm={12} md={6}>
            <RangePicker style={{ width: '100%' }} />
          </Col>
          <Col xs={24} sm={12} md={4}>
            <Button
              icon={<DownloadOutlined />}
              style={{
                width: '100%',
                background: 'transparent',
                border: '1px solid #E8B44F',
                color: '#E8B44F'
              }}
            >
              Export
            </Button>
          </Col>
        </Row>
      </Card>

      {/* Transactions Table */}
      <Card
        style={{
          background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
          border: '1px solid rgba(232, 180, 79, 0.2)',
          borderRadius: '12px'
        }}
      >
        <Table
          columns={columns}
          dataSource={filteredTransactions}
          loading={loading}
          pagination={{
            pageSize: 10,
            showSizeChanger: true,
            showTotal: (total) => `Total ${total} transactions`,
          }}
          style={{
            background: 'transparent'
          }}
        />
      </Card>
    </div>
  );
};

export default Transactions;
