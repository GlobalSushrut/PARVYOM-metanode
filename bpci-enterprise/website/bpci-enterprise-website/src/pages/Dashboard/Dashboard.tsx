import React, { useState, useEffect } from 'react';
import { 
  Card, 
  Typography, 
  Button, 
  Alert, 
  Spin, 
  Space,
  Tag,
  Tabs,
  Statistic
} from 'antd';
import { 
  BankOutlined, 
  SecurityScanOutlined, 
  GlobalOutlined,
  DatabaseOutlined,
  ReloadOutlined,
  ThunderboltOutlined
} from '@ant-design/icons';
import './Dashboard.css';

const { Title, Paragraph } = Typography;
const { TabPane } = Tabs;

// Real backend interfaces matching Rust structs
interface BpciSystemStatus {
  status: string;
  message: string;
  data: {
    node_count: number;
    validator_count: number;
    total_stake: number;
    last_finalized_block: number;
    blockchain_height: number;
    network: string;
    registry_healthy: boolean;
    last_updated: string;
  };
}

interface EconomicStatus {
  status: string;
  message: string;
  data: {
    autonomous_economy: {
      gen_balance: number;
      nex_balance: number;
      total_value_locked: number;
      active_settlements: number;
    };
    bank_apis: {
      active_banks: number;
      total_settlements: number;
      settlement_volume: number;
    };
  };
}

const Dashboard: React.FC = () => {
  const [systemStatus, setSystemStatus] = useState<BpciSystemStatus | null>(null);
  const [economicStatus, setEconomicStatus] = useState<EconomicStatus | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Real API calls to Rust backend
  const fetchSystemStatus = async () => {
    setLoading(true);
    try {
      // Real API call to BPCI registry stats
      const registryResponse = await fetch('http://localhost:8081/api/registry/stats');
      const registryData = await registryResponse.json();
      setSystemStatus(registryData);
      
      // Real API call to economic status
      const economicResponse = await fetch('http://localhost:8081/api/economy/status');
      const economicData = await economicResponse.json();
      setEconomicStatus(economicData);
      
      setError(null);
    } catch (err) {
      console.error('Failed to fetch real backend data:', err);
      setError('Failed to connect to BPCI backend. Please ensure the Rust server is running on port 8081.');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchSystemStatus();
    // Set up polling for real-time updates
    const interval = setInterval(fetchSystemStatus, 30000); // Update every 30 seconds
    return () => clearInterval(interval);
  }, []);

  if (loading) {
    return (
      <div className="flex justify-center items-center min-h-screen">
        <Space direction="vertical" size="large" className="text-center">
          <Spin size="large" />
          <Title level={3}>Connecting to BPCI Backend...</Title>
          <Paragraph>Loading real-time data from Rust server</Paragraph>
        </Space>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-50 to-blue-50">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Header */}
        <div className="mb-8">
          <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between">
            <div className="mb-4 sm:mb-0">
              <Title level={2} className="mb-2 text-slate-800">
                <DatabaseOutlined className="mr-3 text-blue-600" />
                BPCI Enterprise Dashboard
              </Title>
              <Paragraph className="text-slate-600 mb-0 text-lg">
                Real-time monitoring of blockchain infrastructure and autonomous economy
              </Paragraph>
            </div>
            <Button 
              type="primary" 
              size="large"
              icon={<ReloadOutlined />} 
              onClick={fetchSystemStatus}
              loading={loading}
              className="shadow-lg"
            >
              Refresh Data
            </Button>
          </div>
        </div>

        {error && (
          <Alert
            message="Backend Connection Error"
            description={error}
            type="error"
            showIcon
            className="mb-8 shadow-lg"
            action={
              <Button size="small" danger onClick={fetchSystemStatus}>
                Retry Connection
              </Button>
            }
          />
        )}

        {systemStatus && (
          <>
            {/* System Overview Cards */}
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
              <Card className="shadow-lg hover:shadow-xl transition-all duration-300 border-0">
                <div className="text-center">
                  <div className="mb-4">
                    <SecurityScanOutlined className="text-4xl text-green-500" />
                  </div>
                  <Statistic
                    title="Registry Health"
                    value={systemStatus.data.registry_healthy ? "Healthy" : "Issues"}
                    valueStyle={{ 
                      color: systemStatus.data.registry_healthy ? '#10b981' : '#ef4444',
                      fontSize: '1.5rem',
                      fontWeight: 'bold'
                    }}
                  />
                </div>
              </Card>
              
              <Card className="shadow-lg hover:shadow-xl transition-all duration-300 border-0">
                <div className="text-center">
                  <div className="mb-4">
                    <DatabaseOutlined className="text-4xl text-blue-500" />
                  </div>
                  <Statistic
                    title="Active Nodes"
                    value={systemStatus.data.node_count}
                    valueStyle={{ color: '#3b82f6', fontSize: '1.5rem', fontWeight: 'bold' }}
                  />
                </div>
              </Card>
              
              <Card className="shadow-lg hover:shadow-xl transition-all duration-300 border-0">
                <div className="text-center">
                  <div className="mb-4">
                    <ThunderboltOutlined className="text-4xl text-purple-500" />
                  </div>
                  <Statistic
                    title="Validators"
                    value={systemStatus.data.validator_count}
                    valueStyle={{ color: '#8b5cf6', fontSize: '1.5rem', fontWeight: 'bold' }}
                  />
                </div>
              </Card>
              
              <Card className="shadow-lg hover:shadow-xl transition-all duration-300 border-0">
                <div className="text-center">
                  <div className="mb-4">
                    <GlobalOutlined className="text-4xl text-orange-500" />
                  </div>
                  <Statistic
                    title="Block Height"
                    value={systemStatus.data.blockchain_height}
                    valueStyle={{ color: '#f97316', fontSize: '1.5rem', fontWeight: 'bold' }}
                  />
                </div>
              </Card>
            </div>

            {/* Real-time Data Tabs */}
            <Card className="shadow-lg border-0">
              <Tabs defaultActiveKey="1" size="large">
                <TabPane 
                  tab={
                    <span className="flex items-center">
                      <DatabaseOutlined className="mr-2" />
                      System Status
                    </span>
                  } 
                  key="1"
                >
                  <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
                    <div>
                      <Title level={4} className="mb-4 text-slate-700">
                        <GlobalOutlined className="mr-2" />
                        Network Information
                      </Title>
                      <div className="space-y-4">
                        <div className="flex justify-between items-center p-4 bg-slate-50 rounded-lg">
                          <span className="font-medium text-slate-600">Network:</span>
                          <Tag color="blue" className="text-sm">{systemStatus.data.network}</Tag>
                        </div>
                        <div className="flex justify-between items-center p-4 bg-slate-50 rounded-lg">
                          <span className="font-medium text-slate-600">Block Height:</span>
                          <span className="font-bold text-lg text-slate-800">{systemStatus.data.blockchain_height.toLocaleString()}</span>
                        </div>
                        <div className="flex justify-between items-center p-4 bg-slate-50 rounded-lg">
                          <span className="font-medium text-slate-600">Last Finalized:</span>
                          <span className="font-bold text-lg text-slate-800">{systemStatus.data.last_finalized_block.toLocaleString()}</span>
                        </div>
                        <div className="flex justify-between items-center p-4 bg-slate-50 rounded-lg">
                          <span className="font-medium text-slate-600">Total Stake:</span>
                          <span className="font-bold text-lg text-green-600">{systemStatus.data.total_stake.toLocaleString()} BPI</span>
                        </div>
                      </div>
                    </div>
                    
                    <div>
                      <Title level={4} className="mb-4 text-slate-700">
                        <SecurityScanOutlined className="mr-2" />
                        Registry Status
                      </Title>
                      <div className="space-y-4">
                        <Alert
                          message="Registry Health"
                          description={systemStatus.data.registry_healthy ? "All registry services are operational" : "Registry issues detected"}
                          type={systemStatus.data.registry_healthy ? "success" : "error"}
                          showIcon
                          className="mb-4"
                        />
                        <div className="p-4 bg-slate-50 rounded-lg">
                          <Paragraph className="mb-2 text-slate-600">
                            <strong>Last Updated:</strong> {new Date(systemStatus.data.last_updated).toLocaleString()}
                          </Paragraph>
                          <Paragraph className="mb-0 text-slate-600">
                            <strong>Data Source:</strong> Real BPCI Registry (Port 8081)
                          </Paragraph>
                        </div>
                      </div>
                    </div>
                  </div>
                </TabPane>

                <TabPane 
                  tab={
                    <span className="flex items-center">
                      <BankOutlined className="mr-2" />
                      Economic Data
                    </span>
                  } 
                  key="2"
                >
                  {economicStatus ? (
                    <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
                      <div>
                        <Title level={4} className="mb-4 text-slate-700">
                          <ThunderboltOutlined className="mr-2" />
                          Autonomous Economy
                        </Title>
                        <div className="space-y-4">
                          <div className="flex justify-between items-center p-4 bg-green-50 rounded-lg">
                            <div>
                              <div className="font-semibold text-green-800">GEN Balance</div>
                              <div className="text-sm text-green-600">Genesis tokens</div>
                            </div>
                            <div className="text-right">
                              <div className="text-2xl font-bold text-green-800">{economicStatus.data.autonomous_economy.gen_balance.toLocaleString()}</div>
                            </div>
                          </div>
                          <div className="flex justify-between items-center p-4 bg-blue-50 rounded-lg">
                            <div>
                              <div className="font-semibold text-blue-800">NEX Balance</div>
                              <div className="text-sm text-blue-600">Network tokens</div>
                            </div>
                            <div className="text-right">
                              <div className="text-2xl font-bold text-blue-800">{economicStatus.data.autonomous_economy.nex_balance.toLocaleString()}</div>
                            </div>
                          </div>
                          <div className="flex justify-between items-center p-4 bg-purple-50 rounded-lg">
                            <div>
                              <div className="font-semibold text-purple-800">Total Value Locked</div>
                              <div className="text-sm text-purple-600">Across all pools</div>
                            </div>
                            <div className="text-right">
                              <div className="text-2xl font-bold text-purple-800">${economicStatus.data.autonomous_economy.total_value_locked.toLocaleString()}</div>
                            </div>
                          </div>
                        </div>
                      </div>
                      
                      <div>
                        <Title level={4} className="mb-4 text-slate-700">
                          <BankOutlined className="mr-2" />
                          Banking Integration
                        </Title>
                        <div className="space-y-4">
                          <div className="flex justify-between items-center p-4 bg-slate-50 rounded-lg">
                            <span className="font-medium text-slate-600">Active Banks:</span>
                            <span className="font-bold text-lg text-slate-800">{economicStatus.data.bank_apis.active_banks}</span>
                          </div>
                          <div className="flex justify-between items-center p-4 bg-slate-50 rounded-lg">
                            <span className="font-medium text-slate-600">Total Settlements:</span>
                            <span className="font-bold text-lg text-slate-800">{economicStatus.data.bank_apis.total_settlements.toLocaleString()}</span>
                          </div>
                          <div className="flex justify-between items-center p-4 bg-slate-50 rounded-lg">
                            <span className="font-medium text-slate-600">Settlement Volume:</span>
                            <span className="font-bold text-lg text-green-600">${economicStatus.data.bank_apis.settlement_volume.toLocaleString()}</span>
                          </div>
                        </div>
                      </div>
                    </div>
                  ) : (
                    <Alert
                      message="Economic Data Unavailable"
                      description="Unable to fetch economic data from the backend. Please check the connection."
                      type="warning"
                      showIcon
                    />
                  )}
                </TabPane>
              </Tabs>
            </Card>
          </>
        )}
      </div>
    </div>
  );
};

export default Dashboard;
