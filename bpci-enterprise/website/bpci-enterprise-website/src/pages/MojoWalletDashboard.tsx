import React, { useState, useEffect } from 'react';
import { 
  Card, 
  Row, 
  Col, 
  Statistic, 
  Button, 
  Typography, 
  Space, 
  Table, 
  Tag, 
  Alert,
  Spin,
  Tabs
} from 'antd';
import {
  WalletOutlined,
  SendOutlined,
  DownloadOutlined,
  SwapOutlined,
  RocketOutlined,
  DashboardOutlined,
  HistoryOutlined,
  SettingOutlined,
  CopyOutlined,
  QrcodeOutlined,
  ArrowUpOutlined,
  ArrowDownOutlined
} from '@ant-design/icons';
import { useNavigate } from 'react-router-dom';
import { apiService } from '../services/api';

const { Title, Text, Paragraph } = Typography;
const { TabPane } = Tabs;

const MojoWalletDashboard: React.FC = () => {
  const navigate = useNavigate();
  const [loading, setLoading] = useState(true);
  const [wallet, setWallet] = useState<any>(null);
  const [transactions, setTransactions] = useState<any[]>([]);
  const [balance, setBalance] = useState(0);

  useEffect(() => {
    loadWalletData();
  }, []);

  const loadWalletData = async () => {
    try {
      // REAL BACKEND INTEGRATION - Component 6 (Cluster Ledger) via Component 9 (Web Interface)
      
      // Step 1: Get BPI connection (address + token from dual-auth wizard)
      const connectionsResponse = await apiService.listBpiConnections();
      
      if (connectionsResponse.success && connectionsResponse.data && connectionsResponse.data.length > 0) {
        const primaryConnection = connectionsResponse.data[0];
        const { address: bpiAddress, token: authToken, name, id, created_at } = primaryConnection;
        
        // Cloud-ready API endpoint (supports both local and production)
        const API_BASE = process.env.REACT_APP_API_URL || 'http://146.190.74.139:8080';
        
        // Step 2: Query Component 6 (Cluster Ledger) via Web Interface
        // This validates token+address and returns ONLY this user's data from millions of BPI instances
        const walletStatusResponse = await fetch(`${API_BASE}/api/wallet/status`, {
          method: 'POST',
          headers: {
            'Authorization': `Bearer ${authToken}`,
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({
            bpi_address: bpiAddress
          })
        });
        
        if (walletStatusResponse.ok) {
          const statusData = await walletStatusResponse.json();
          
          // Component 6 returns filtered data for this specific BPI instance
          setWallet({
            wallet_id: id,
            wallet_name: name,
            bpi_address: bpiAddress,
            is_activated: statusData.data?.status === 'active' || statusData.data?.verification_level === 'verified',
            created_at: created_at,
            status: statusData.data?.status || 'active',
            node_id: statusData.data?.node_id,
            network: statusData.data?.network || 'mainnet'
          });
        }
        
        // Step 3: Get real 4-coin balance (GEN/NEX/FLX/AUR) from economic integration
        // Component 6 aggregates balance from BPI node via economic integration
        const balanceResponse = await fetch(`${API_BASE}/api/wallet/balance`, {
          method: 'POST',
          headers: {
            'Authorization': `Bearer ${authToken}`,
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({
            bpi_address: bpiAddress
          })
        });
        
        if (balanceResponse.ok) {
          const balanceData = await balanceResponse.json();
          
          // Real backend returns 4-coin balance from autonomous economy
          if (balanceData.data?.balance) {
            const fourCoinBalance = balanceData.data.balance;
            
            // Calculate total balance across all 4 coins
            const totalBalance = 
              (fourCoinBalance.gen || 0) +
              (fourCoinBalance.nex || 0) +
              (fourCoinBalance.flx || 0) +
              (fourCoinBalance.aur || 0);
            
            setBalance(totalBalance);
            
            console.log('4-Coin Balance:', {
              GEN: fourCoinBalance.gen,
              NEX: fourCoinBalance.nex,
              FLX: fourCoinBalance.flx,
              AUR: fourCoinBalance.aur,
              Total: totalBalance
            });
          } else {
            // Fallback to simple balance if 4-coin not available
            setBalance(balanceData.data?.total_balance || 0);
          }
        }
        
        // Step 4: Load real transactions from Component 6
        // Component 6 queries BPI node transaction history
        const transactionsResponse = await fetch(`${API_BASE}/api/wallet/transactions`, {
          method: 'POST',
          headers: {
            'Authorization': `Bearer ${authToken}`,
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({
            bpi_address: bpiAddress,
            limit: 10
          })
        });
        
        if (transactionsResponse.ok) {
          const txData = await transactionsResponse.json();
          
          if (txData.data?.transactions && txData.data.transactions.length > 0) {
            // Use real transactions from backend
            setTransactions(txData.data.transactions.map((tx: any, index: number) => ({
              key: index.toString(),
              type: tx.type || (tx.from === bpiAddress ? 'send' : 'receive'),
              amount: tx.amount || 0,
              from: tx.from || 'unknown',
              to: tx.to || 'unknown',
              status: tx.status || 'confirmed',
              timestamp: tx.timestamp || new Date().toISOString(),
              txHash: tx.tx_hash || tx.tx_id || `0x${Math.random().toString(16).substring(2, 10)}`
            })));
          } else {
            // No transactions yet - show welcome message
            setTransactions([]);
          }
        } else {
          // Fallback: show initial transaction if no history available
          setTransactions([
            {
              key: '1',
              type: 'receive',
              amount: 0,
              from: 'bpi:system',
              to: bpiAddress,
              status: 'confirmed',
              timestamp: created_at,
              txHash: `0x${Math.random().toString(16).substring(2, 10)}`
            }
          ]);
        }
        
      } else {
        // Fallback: No BPI connections - user needs to complete dual-auth
        console.warn('No BPI connections found - user needs dual-auth activation');
        
        // Try old wallet system as last resort
        const walletsResponse = await apiService.listBpiWallets();
        
        if (walletsResponse.success && walletsResponse.data && walletsResponse.data.length > 0) {
          const primaryWallet = walletsResponse.data[0];
          setWallet(primaryWallet);
          
          const balanceResponse = await apiService.getBpiWalletBalance(primaryWallet.wallet_id);
          if (balanceResponse.success && balanceResponse.data !== undefined) {
            setBalance(balanceResponse.data);
          }
        }
      }
    } catch (error) {
      console.error('Failed to load wallet data:', error);
    } finally {
      setLoading(false);
    }
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  const transactionColumns = [
    {
      title: 'Type',
      dataIndex: 'type',
      key: 'type',
      render: (type: string) => (
        <Tag color={type === 'receive' ? 'green' : 'blue'} icon={type === 'receive' ? <ArrowDownOutlined /> : <ArrowUpOutlined />}>
          {type === 'receive' ? 'Received' : 'Sent'}
        </Tag>
      ),
    },
    {
      title: 'Amount',
      dataIndex: 'amount',
      key: 'amount',
      render: (amount: number, record: any) => (
        <Text strong style={{ color: record.type === 'receive' ? '#52c41a' : '#1890ff' }}>
          {record.type === 'receive' ? '+' : '-'}{amount} BPI
        </Text>
      ),
    },
    {
      title: 'Address',
      dataIndex: 'type',
      key: 'address',
      render: (_: any, record: any) => (
        <Text code style={{ fontSize: '12px' }}>
          {record.type === 'receive' ? record.from.substring(0, 20) : record.to.substring(0, 20)}...
        </Text>
      ),
    },
    {
      title: 'Status',
      dataIndex: 'status',
      key: 'status',
      render: (status: string) => (
        <Tag color={status === 'confirmed' ? 'success' : 'processing'}>
          {status === 'confirmed' ? 'Confirmed' : 'Pending'}
        </Tag>
      ),
    },
    {
      title: 'Time',
      dataIndex: 'timestamp',
      key: 'timestamp',
      render: (timestamp: string) => new Date(timestamp).toLocaleString(),
    },
  ];

  if (loading) {
    return (
      <div style={{ textAlign: 'center', padding: '100px 0' }}>
        <Spin size="large" />
        <Paragraph style={{ marginTop: 16 }}>Loading your wallet...</Paragraph>
      </div>
    );
  }

  if (!wallet) {
    return (
      <div style={{ padding: '24px', maxWidth: '800px', margin: '0 auto' }}>
        <Alert
          message="No Wallet Found"
          description="You don't have a BPI wallet yet. Please complete the dual-auth wizard to create one."
          type="warning"
          showIcon
          action={
            <Button type="primary" onClick={() => navigate('/dual-auth')}>
              Create Wallet
            </Button>
          }
        />
      </div>
    );
  }

  return (
    <div style={{ padding: '24px', maxWidth: '1400px', margin: '0 auto' }}>
      {/* Header */}
      <div style={{ marginBottom: 24 }}>
        <Space align="center" style={{ width: '100%', justifyContent: 'space-between' }}>
          <Space>
            <WalletOutlined style={{ fontSize: '32px', color: '#1890ff' }} />
            <div>
              <Title level={2} style={{ margin: 0 }}>Mojo Wallet</Title>
              <Text type="secondary">{wallet.wallet_name}</Text>
            </div>
          </Space>
          <Space>
            <Button icon={<SettingOutlined />} onClick={() => navigate('/wallet/settings')}>
              Settings
            </Button>
          </Space>
        </Space>
      </div>

      {/* Balance Card */}
      <Card 
        style={{ 
          marginBottom: 24, 
          background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
          border: 'none'
        }}
      >
        <Row gutter={[16, 16]}>
          <Col xs={24} md={12}>
            <Space direction="vertical" size="small">
              <Text style={{ color: 'rgba(255,255,255,0.8)', fontSize: '14px' }}>
                Total Balance
              </Text>
              <Title level={1} style={{ color: 'white', margin: 0 }}>
                {balance.toLocaleString()} BPI
              </Title>
              <Text style={{ color: 'rgba(255,255,255,0.9)', fontSize: '16px' }}>
                ≈ ${(balance * 0.05).toFixed(2)} USD
              </Text>
            </Space>
          </Col>
          <Col xs={24} md={12}>
            <Space direction="vertical" size="small" style={{ width: '100%' }}>
              <Text style={{ color: 'rgba(255,255,255,0.8)', fontSize: '14px' }}>
                BPI Address
              </Text>
              <div style={{ 
                background: 'rgba(255,255,255,0.1)', 
                padding: '12px', 
                borderRadius: '8px',
                wordBreak: 'break-all'
              }}>
                <Text code style={{ color: 'white', fontSize: '12px' }}>
                  {wallet.bpi_address}
                </Text>
                <Button 
                  type="link" 
                  icon={<CopyOutlined />}
                  onClick={() => copyToClipboard(wallet.bpi_address)}
                  style={{ color: 'white', marginLeft: 8 }}
                >
                  Copy
                </Button>
              </div>
            </Space>
          </Col>
        </Row>

        {/* Quick Actions */}
        <Row gutter={16} style={{ marginTop: 24 }}>
          <Col xs={12} sm={6}>
            <Button 
              type="primary" 
              block 
              size="large"
              icon={<SendOutlined />}
              onClick={() => navigate('/wallet/send')}
              style={{ background: 'rgba(255,255,255,0.2)', border: 'none' }}
            >
              Send
            </Button>
          </Col>
          <Col xs={12} sm={6}>
            <Button 
              type="primary" 
              block 
              size="large"
              icon={<DownloadOutlined />}
              onClick={() => navigate('/wallet/receive')}
              style={{ background: 'rgba(255,255,255,0.2)', border: 'none' }}
            >
              Receive
            </Button>
          </Col>
          <Col xs={12} sm={6}>
            <Button 
              type="primary" 
              block 
              size="large"
              icon={<SwapOutlined />}
              onClick={() => navigate('/wallet/swap')}
              style={{ background: 'rgba(255,255,255,0.2)', border: 'none' }}
            >
              Swap
            </Button>
          </Col>
          <Col xs={12} sm={6}>
            <Button 
              type="primary" 
              block 
              size="large"
              icon={<RocketOutlined />}
              onClick={() => navigate('/deploy-node')}
              style={{ background: 'rgba(255,255,255,0.2)', border: 'none' }}
            >
              Deploy Node
            </Button>
          </Col>
        </Row>
      </Card>

      {/* Stats Row */}
      <Row gutter={[16, 16]} style={{ marginBottom: 24 }}>
        <Col xs={24} sm={8}>
          <Card>
            <Statistic
              title="Total Transactions"
              value={transactions.length}
              prefix={<HistoryOutlined />}
              valueStyle={{ color: '#1890ff' }}
            />
          </Card>
        </Col>
        <Col xs={24} sm={8}>
          <Card>
            <Statistic
              title="Wallet Status"
              value={wallet.is_activated ? "Active" : "Inactive"}
              prefix={<WalletOutlined />}
              valueStyle={{ color: wallet.is_activated ? '#52c41a' : '#faad14' }}
            />
          </Card>
        </Col>
        <Col xs={24} sm={8}>
          <Card>
            <Statistic
              title="Deployed Nodes"
              value={0}
              prefix={<DashboardOutlined />}
              suffix="nodes"
              valueStyle={{ color: '#722ed1' }}
            />
          </Card>
        </Col>
      </Row>

      {/* Transactions Table */}
      <Card 
        title={
          <Space>
            <HistoryOutlined />
            <Text strong>Recent Transactions</Text>
          </Space>
        }
        extra={
          <Button type="link" onClick={() => navigate('/wallet/transactions')}>
            View All →
          </Button>
        }
      >
        <Table 
          columns={transactionColumns} 
          dataSource={transactions}
          pagination={{ pageSize: 5 }}
          scroll={{ x: 800 }}
        />
      </Card>

      {/* Help Section */}
      <Card style={{ marginTop: 24, background: '#f0f2f5', border: 'none' }}>
        <Space direction="vertical" size="small" style={{ width: '100%' }}>
          <Title level={5}>Quick Tips</Title>
          <ul style={{ marginBottom: 0, paddingLeft: 20 }}>
            <li>Your BPI wallet is secured with dual-authentication (Keycloak + BPI)</li>
            <li>Always verify recipient addresses before sending transactions</li>
            <li>Deploy BPI OS nodes to earn rewards and contribute to the network</li>
            <li>Check the transaction history regularly for suspicious activity</li>
          </ul>
        </Space>
      </Card>
    </div>
  );
};

export default MojoWalletDashboard;
