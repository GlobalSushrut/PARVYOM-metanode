import React, { useState, useEffect } from 'react';
import { Card, Tabs, Typography, Table, Tag, Space, Button, Input, Alert, Statistic, Row, Col } from 'antd';
import {
  DatabaseOutlined,
  SwapOutlined,
  BlockOutlined,
  GlobalOutlined,
  SafetyOutlined,
  WalletOutlined,
  SearchOutlined,
  ReloadOutlined,
  DashboardOutlined
} from '@ant-design/icons';
import axios from 'axios';

const { Title, Text } = Typography;

const REGISTRY_API = 'http://localhost:8081/api/registry';
const MOJO_API = 'http://localhost:8089/api/v1';
const LEDGER_API = 'http://localhost:8082/api/ledger';
const AUCTION_API = 'http://localhost:8083/api/auction';
const BLOCKCHAIN_API = 'http://localhost:8084/api/blockchain';
const NETWORK_API = 'http://localhost:8085/api/network';
const CONSENSUS_API = 'http://localhost:8086/api/consensus';

interface Transaction {
  tx_hash: string;
  from: string;
  to: string;
  amount: number;
  timestamp: string;
  status: string;
  block_height: number;
}

interface Auction {
  auction_id: string;
  chain_id: number;
  bid_amount: number;
  bidder: string;
  status: string;
  timestamp: string;
}

interface Block {
  block_height: number;
  block_hash: string;
  timestamp: string;
  tx_count: number;
  validator: string;
}

interface Peer {
  node_id: string;
  address: string;
  status: string;
  last_seen: string;
}

interface MojoWallet {
  mojo_wallet_id: string;
  bpi_wallet_address: string;
  grafana_dashboard_url: string;
  grafana_token: string;
  prometheus_job: string;
  created_at: string;
}

const RegistryDashboard: React.FC = () => {
  const [loading, setLoading] = useState(false);
  const [registryStats, setRegistryStats] = useState<any>(null);
  const [mojoWallets, setMojoWallets] = useState<MojoWallet[]>([]);

  useEffect(() => {
    fetchRegistryData();
    fetchMojoWallets();
  }, []);

  const fetchRegistryData = async () => {
    setLoading(true);
    try {
      const response = await axios.get(`${REGISTRY_API}/stats`);
      if (response.data.status === 'ok') {
        setRegistryStats(response.data.data);
      }
    } catch (error) {
      console.error('Failed to fetch registry data:', error);
    } finally {
      setLoading(false);
    }
  };

  const fetchMojoWallets = async () => {
    try {
      const response = await axios.get(`${MOJO_API}/wallets`);
      if (response.data) {
        setMojoWallets(response.data);
      }
    } catch (error) {
      console.error('Failed to fetch Mojo wallets:', error);
    }
  };

  // Transactions Tab
  const TransactionsTab = () => {
    const [transactions] = useState<Transaction[]>([
      // Mock data - replace with real API call
      {
        tx_hash: '0x1234...5678',
        from: '0xabcd...efgh',
        to: '0x9876...5432',
        amount: 100.5,
        timestamp: new Date().toISOString(),
        status: 'confirmed',
        block_height: 12345
      }
    ]);

    const columns = [
      {
        title: 'TX Hash',
        dataIndex: 'tx_hash',
        key: 'tx_hash',
        render: (hash: string) => <Text style={{ fontFamily: 'monospace', color: '#E8B44F' }}>{hash}</Text>
      },
      {
        title: 'From',
        dataIndex: 'from',
        key: 'from',
        render: (addr: string) => <Text style={{ fontFamily: 'monospace' }}>{addr}</Text>
      },
      {
        title: 'To',
        dataIndex: 'to',
        key: 'to',
        render: (addr: string) => <Text style={{ fontFamily: 'monospace' }}>{addr}</Text>
      },
      {
        title: 'Amount',
        dataIndex: 'amount',
        key: 'amount',
        render: (amount: number) => <Text>{amount} BPI</Text>
      },
      {
        title: 'Status',
        dataIndex: 'status',
        key: 'status',
        render: (status: string) => (
          <Tag color={status === 'confirmed' ? 'success' : 'processing'}>{status}</Tag>
        )
      },
      {
        title: 'Block',
        dataIndex: 'block_height',
        key: 'block_height'
      }
    ];

    return (
      <div>
        <Space style={{ marginBottom: '1rem' }}>
          <Input
            placeholder="Search by TX hash or address"
            prefix={<SearchOutlined />}
            style={{ width: 300 }}
          />
          <Button icon={<ReloadOutlined />}>Refresh</Button>
        </Space>
        <Table
          columns={columns}
          dataSource={transactions}
          rowKey="tx_hash"
          pagination={{ pageSize: 10 }}
        />
      </div>
    );
  };

  // Auctions Tab
  const AuctionsTab = () => {
    return (
      <Alert
        message="Auction Mempool"
        description="Real-time auction data from BPCI auction mempool will be displayed here. Connect to auction mempool backend."
        type="info"
        showIcon
      />
    );
  };

  // Blocks Tab
  const BlocksTab = () => {
    return (
      <Alert
        message="Block Explorer"
        description="Blockchain blocks, validators, and consensus data will be displayed here. Connect to blockchain backend."
        type="info"
        showIcon
      />
    );
  };

  // P2P Mesh Tab
  const P2PMeshTab = () => {
    return (
      <Alert
        message="P2P Network Mesh"
        description="Network topology, connected peers, and mesh health will be displayed here. Connect to P2P networking backend."
        type="info"
        showIcon
      />
    );
  };

  // Security Tab
  const SecurityTab = () => {
    return (
      <Alert
        message="BPI Security & Consensus"
        description="Validator set, consensus status, and security metrics will be displayed here. Connect to consensus backend."
        type="info"
        showIcon
      />
    );
  };

  // Mojo Wallet Tab
  const MojoWalletTab = () => {
    const columns = [
      {
        title: 'BPI Wallet Address',
        dataIndex: 'bpi_wallet_address',
        key: 'bpi_wallet_address',
        render: (addr: string) => <Text style={{ fontFamily: 'monospace', color: '#E8B44F' }}>{addr}</Text>
      },
      {
        title: 'Grafana Dashboard',
        dataIndex: 'grafana_dashboard_url',
        key: 'grafana_dashboard_url',
        render: (url: string) => (
          <a href={url} target="_blank" rel="noopener noreferrer" style={{ color: '#3B82F6' }}>
            Open Dashboard
          </a>
        )
      },
      {
        title: 'Prometheus Job',
        dataIndex: 'prometheus_job',
        key: 'prometheus_job'
      },
      {
        title: 'Created',
        dataIndex: 'created_at',
        key: 'created_at',
        render: (date: string) => new Date(date).toLocaleString()
      },
      {
        title: 'Actions',
        key: 'actions',
        render: (record: MojoWallet) => (
          <Button size="small" icon={<DashboardOutlined />}>
            View Details
          </Button>
        )
      }
    ];

    return (
      <div>
        <Alert
          message="Mojo Wallet - Address-Based Server Details"
          description="Each BPI wallet address is mapped to server connection details (Grafana, Prometheus, access tokens). This manages the server-side infrastructure for each wallet."
          type="info"
          showIcon
          style={{ marginBottom: '1rem' }}
        />
        <Table
          columns={columns}
          dataSource={mojoWallets}
          rowKey="mojo_wallet_id"
          pagination={{ pageSize: 10 }}
          loading={loading}
        />
      </div>
    );
  };

  return (
    <div style={{ padding: '1.5rem' }}>
      {/* Header */}
      <div style={{ marginBottom: '2rem' }}>
        <Title level={2} style={{ color: '#E8B44F', marginBottom: '0.5rem' }}>
          <DatabaseOutlined /> BPI Registry & Ledger Dashboard
        </Title>
        <Text style={{ color: '#9CA3AF' }}>
          Comprehensive blockchain monitoring: Transactions, Auctions, Blocks, P2P Mesh, Security & Wallet Management
        </Text>
      </div>

      {/* Stats Overview */}
      {registryStats && (
        <Row gutter={[16, 16]} style={{ marginBottom: '2rem' }}>
          <Col xs={24} sm={12} md={6}>
            <Card style={{ background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)', border: '1px solid rgba(232, 180, 79, 0.2)' }}>
              <Statistic
                title={<Text style={{ color: '#9CA3AF' }}>Total Nodes</Text>}
                value={registryStats.total_nodes}
                valueStyle={{ color: '#E8B44F' }}
              />
            </Card>
          </Col>
          <Col xs={24} sm={12} md={6}>
            <Card style={{ background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)', border: '1px solid rgba(232, 180, 79, 0.2)' }}>
              <Statistic
                title={<Text style={{ color: '#9CA3AF' }}>Total Wallets</Text>}
                value={registryStats.total_wallets}
                valueStyle={{ color: '#10B981' }}
              />
            </Card>
          </Col>
          <Col xs={24} sm={12} md={6}>
            <Card style={{ background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)', border: '1px solid rgba(232, 180, 79, 0.2)' }}>
              <Statistic
                title={<Text style={{ color: '#9CA3AF' }}>Validators</Text>}
                value={registryStats.total_validators}
                valueStyle={{ color: '#3B82F6' }}
              />
            </Card>
          </Col>
          <Col xs={24} sm={12} md={6}>
            <Card style={{ background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)', border: '1px solid rgba(232, 180, 79, 0.2)' }}>
              <Statistic
                title={<Text style={{ color: '#9CA3AF' }}>Block Height</Text>}
                value={registryStats.blockchain_height}
                valueStyle={{ color: '#F59E0B' }}
              />
            </Card>
          </Col>
        </Row>
      )}

      {/* Tabbed Interface */}
      <Card
        style={{
          background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
          border: '1px solid rgba(232, 180, 79, 0.2)',
          borderRadius: '12px'
        }}
      >
        <Tabs
          defaultActiveKey="1"
          items={[
            {
              key: '1',
              label: (
                <span style={{ color: '#E8B44F' }}>
                  <SwapOutlined /> Transactions
                </span>
              ),
              children: <TransactionsTab />
            },
            {
              key: '2',
              label: (
                <span style={{ color: '#9CA3AF' }}>
                  <DatabaseOutlined /> Auctions
                </span>
              ),
              children: <AuctionsTab />
            },
            {
              key: '3',
              label: (
                <span style={{ color: '#9CA3AF' }}>
                  <BlockOutlined /> Blocks
                </span>
              ),
              children: <BlocksTab />
            },
            {
              key: '4',
              label: (
                <span style={{ color: '#9CA3AF' }}>
                  <GlobalOutlined /> P2P Mesh
                </span>
              ),
              children: <P2PMeshTab />
            },
            {
              key: '5',
              label: (
                <span style={{ color: '#9CA3AF' }}>
                  <SafetyOutlined /> Security
                </span>
              ),
              children: <SecurityTab />
            },
            {
              key: '6',
              label: (
                <span style={{ color: '#9CA3AF' }}>
                  <WalletOutlined /> Mojo Wallet
                </span>
              ),
              children: <MojoWalletTab />
            }
          ]}
        />
      </Card>
    </div>
  );
};

export default RegistryDashboard;
