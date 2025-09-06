import React from 'react';
import { Card, Row, Col, Tabs, Typography, Badge, Collapse, Tag, Progress } from 'antd';
import {
  SecurityScanOutlined,
  DatabaseOutlined,
  ThunderboltOutlined,
  SafetyOutlined,
  GlobalOutlined,
  LockOutlined,
  ApiOutlined,
  CodeOutlined,
  BankOutlined,
  SettingOutlined,
  FireOutlined,
  RocketOutlined,
  NodeIndexOutlined,
  ClusterOutlined,
  MonitorOutlined,
  ControlOutlined
} from '@ant-design/icons';
import './Technology.css';
const { Title, Paragraph, Text } = Typography;
const { TabPane } = Tabs;
const { Panel } = Collapse;

const Technology: React.FC = () => {
  return (
    <div className="technology-page">
      {/* Hero Section */}
      <section className="hero-gradient py-20">
        <div className="max-w-6xl mx-auto px-4 text-center">
          <Title level={1} className="text-white text-5xl font-bold mb-6">
            Technology Stack
          </Title>
          <Paragraph className="text-blue-100 text-xl max-w-4xl mx-auto">
            Advanced blockchain infrastructure powered by post-quantum ready cryptography, 
            military-grade algorithms, and enterprise-oriented architecture for next-generation Web3 applications.
          </Paragraph>
        </div>
      </section>

      {/* Core Architecture */}
      <section className="py-20 bg-white">
        <div className="max-w-6xl mx-auto px-4">
          <div className="text-center mb-16">
            <div className="bg-orange-50 border border-orange-200 rounded-lg p-6 mb-8 max-w-4xl mx-auto">
              <h2 className="text-xl font-semibold text-orange-800 mb-3">🔬 Experimental Technology Stack</h2>
              <p className="text-orange-700 mb-4">
                The components described below are <strong>experimental and in development</strong>. 
                This is R&D work for pilot testing, not production-ready enterprise infrastructure.
              </p>
              <div className="grid md:grid-cols-2 gap-4 text-left">
                <div>
                  <h4 className="font-semibold text-orange-800 mb-2">Current Status:</h4>
                  <ul className="space-y-1 text-orange-700 text-sm">
                    <li>• Testnet implementation only</li>
                    <li>• Security analysis in progress</li>
                    <li>• Performance not optimized</li>
                    <li>• Limited to experimental use cases</li>
                  </ul>
                </div>
                <div>
                  <h4 className="font-semibold text-orange-800 mb-2">Seeking Partners For:</h4>
                  <ul className="space-y-1 text-orange-700 text-sm">
                    <li>• Concept validation and testing</li>
                    <li>• Security analysis collaboration</li>
                    <li>• Real-world feedback on architecture</li>
                    <li>• Pre-funding for development</li>
                  </ul>
                </div>
              </div>
            </div>
            
            <Title level={2} className="text-4xl font-bold text-gray-900 mb-6">
              Experimental Technology Stack (Pilot Phase)
            </Title>
            <Paragraph className="text-xl text-gray-600 max-w-3xl mx-auto">
              Exploring blockchain infrastructure concepts with post-quantum ready security, 
              experimental consensus mechanisms, and testnet-scale components. 
              <strong>Not ready for production deployment.</strong>
            </Paragraph>
          </div>

          <Row gutter={[32, 32]}>
            <Col xs={24} lg={12}>
              <Card className="h-full p-8 hover:shadow-lg transition-shadow">
                <div className="flex items-start space-x-4">
                  <LockOutlined className="text-4xl text-blue-600 mt-1" />
                  <div>
                    <Title level={3} className="text-2xl font-semibold mb-4">ENC Lock + QLOCK</Title>
                    <Paragraph className="text-gray-700 mb-4">
                      Post-quantum ready security system with domain-separated hashing 
                      and robust error handling on sync failure.
                    </Paragraph>
                    <div className="space-y-2">
                      <Tag color="blue">Blake3 Hashing</Tag>
                      <Tag color="green">Ed25519 Signatures</Tag>
                      <Tag color="purple">Distance Bounding (50m ToF)</Tag>
                      <Tag color="orange">QLOCK Sync Gates</Tag>
                    </div>
                    <div className="mt-4">
                      <Paragraph className="text-sm text-gray-600 mb-2">Security Rating</Paragraph>
                      <Progress percent={98} status="active" strokeColor="#059669" />
                    </div>
                  </div>
                </div>
              </Card>
            </Col>

            <Col xs={24} lg={12}>
              <Card className="h-full p-8 hover:shadow-lg transition-shadow">
                <div className="flex items-start space-x-4">
                  <DatabaseOutlined className="text-4xl text-green-600 mt-1" />
                  <div>
                    <Title level={3} className="text-2xl font-semibold mb-4">Autonomous Economy</Title>
                    <Paragraph className="text-gray-700 mb-4">
                      4-coin economic system with mathematical distribution models 
                      and real-time blockchain integration.
                    </Paragraph>
                    <div className="space-y-2">
                      <Tag color="gold">GEN (Genesis)</Tag>
                      <Tag color="cyan">NEX (Network)</Tag>
                      <Tag color="magenta">FLX (Flex)</Tag>
                      <Tag color="volcano">AUR (Aurum)</Tag>
                    </div>
                    <div className="mt-4">
                      <Paragraph className="text-sm text-gray-600 mb-2">Treasury Split</Paragraph>
                      <Progress percent={75} status="active" strokeColor="#0066cc" />
                      <Paragraph className="text-xs text-gray-500">25% Coin Economy / 75% Infrastructure</Paragraph>
                    </div>
                  </div>
                </div>
              </Card>
            </Col>
          </Row>
        </div>
      </section>

      {/* Technical Specifications */}
      <section className="py-20 bg-gray-50">
        <div className="max-w-6xl mx-auto px-4">
          <div className="text-center mb-16">
            <Title level={2} className="text-4xl font-bold mb-6">Technical Specifications</Title>
            <Paragraph className="text-xl text-gray-600">
              Detailed breakdown of our infrastructure components and capabilities
            </Paragraph>
          </div>

          <Tabs defaultActiveKey="security" size="large" centered>
            <TabPane 
              tab={
                <span>
                  <SecurityScanOutlined />
                  Security Layer
                </span>
              } 
              key="security"
            >
              <div className="max-w-4xl mx-auto">
                <Row gutter={[32, 32]}>
                  <Col xs={24} md={12}>
                    <Card className="h-full">
                      <Title level={4} className="text-blue-600 mb-4">
                        <LockOutlined className="mr-2" />
                        Cryptographic Foundation
                      </Title>
                      <ul className="space-y-3 text-gray-700">
                        <li><strong>Ed25519:</strong> Elliptic curve signatures with 128-bit security</li>
                        <li><strong>Blake3:</strong> Cryptographic hash function with domain separation</li>
                        <li><strong>Sync Gates:</strong> Mathematical validation protocols (in development)</li>
                        <li><strong>Network Validation:</strong> Time-based authentication protocols</li>
                        <li><strong>Error Handling:</strong> Robust response on synchronization failure</li>
                      </ul>
                    </Card>
                  </Col>
                  <Col xs={24} md={12}>
                    <Card className="h-full">
                      <Title level={4} className="text-green-600 mb-4">
                        <SafetyOutlined className="mr-2" />
                        Security Features
                      </Title>
                      <ul className="space-y-3 text-gray-700">
                        <li><strong>Post-Quantum Ready:</strong> Upgrade path for quantum resistance</li>
                        <li><strong>Military-Grade Algorithms:</strong> Ed25519 and Blake3 cryptography</li>
                        <li><strong>Zero-Knowledge:</strong> Privacy-preserving proofs</li>
                        <li><strong>Audit Trails:</strong> Immutable compliance records</li>
                        <li><strong>Multi-Layer:</strong> Defense in depth architecture</li>
                      </ul>
                    </Card>
                  </Col>
                </Row>
              </div>
            </TabPane>

            <TabPane 
              tab={
                <span>
                  <ClusterOutlined />
                  Infrastructure
                </span>
              } 
              key="infrastructure"
            >
              <div className="max-w-4xl mx-auto">
                <Row gutter={[32, 32]}>
                  <Col xs={24} md={12}>
                    <Card className="h-full">
                      <Title level={4} className="text-purple-600 mb-4">
                        <DatabaseOutlined className="mr-2" />
                        Node Architecture
                      </Title>
                      <ul className="space-y-3 text-gray-700">
                        <li><strong>ENC Cluster:</strong> Connected gateway + mempool coordination</li>
                        <li><strong>BPI Oracle:</strong> Cross-system communication bridge</li>
                        <li><strong>Shadow Registry:</strong> Web2-to-Web3 safe communication</li>
                        <li><strong>Pipeline API:</strong> BISO traffic light integration</li>
                        <li><strong>Storage Nodes:</strong> Distributed data management</li>
                      </ul>
                    </Card>
                  </Col>
                  <Col xs={24} md={12}>
                    <Card className="h-full">
                      <Title level={4} className="text-orange-600 mb-4">
                        <ThunderboltOutlined className="mr-2" />
                        Performance Metrics
                      </Title>
                      <ul className="space-y-3 text-gray-700">
                        <li><strong>Uptime:</strong> Testnet 100% availability (3 nodes)</li>
                        <li><strong>Latency:</strong> Optimized for sub-second response times</li>
                        <li><strong>Throughput:</strong> Designed for high-performance scaling</li>
                        <li><strong>Scalability:</strong> Horizontal node scaling architecture</li>
                        <li><strong>Consensus:</strong> Byzantine fault tolerant design</li>
                      </ul>
                    </Card>
                  </Col>
                </Row>
              </div>
            </TabPane>

            <TabPane 
              tab={
                <span>
                  <ApiOutlined />
                  APIs & Integration
                </span>
              } 
              key="apis"
            >
              <div className="max-w-4xl mx-auto">
                <Row gutter={[32, 32]}>
                  <Col xs={24} md={12}>
                    <Card className="h-full">
                      <Title level={4} className="text-blue-600 mb-4">
                        <BankOutlined className="mr-2" />
                        Enterprise APIs
                      </Title>
                      <ul className="space-y-3 text-gray-700">
                        <li><strong>Banking API:</strong> Settlement, compliance, audit (Port 8081)</li>
                        <li><strong>Government API:</strong> Regulatory, classification, jurisdiction</li>
                        <li><strong>Community API:</strong> Node management, governance</li>
                        <li><strong>Wallet API:</strong> Stamped wallet operations</li>
                        <li><strong>Monitoring API:</strong> Real-time system metrics</li>
                      </ul>
                    </Card>
                  </Col>
                  <Col xs={24} md={12}>
                    <Card className="h-full">
                      <Title level={4} className="text-green-600 mb-4">
                        <CodeOutlined className="mr-2" />
                        Developer Tools
                      </Title>
                      <ul className="space-y-3 text-gray-700">
                        <li><strong>REST APIs:</strong> RESTful endpoints with OpenAPI specs</li>
                        <li><strong>WebSocket:</strong> Real-time data streaming</li>
                        <li><strong>CLI Tools:</strong> Command-line interface for operations</li>
                        <li><strong>SDKs:</strong> Multiple language support</li>
                        <li><strong>Documentation:</strong> Comprehensive API documentation</li>
                      </ul>
                    </Card>
                  </Col>
                </Row>
              </div>
            </TabPane>

            <TabPane 
              tab={
                <span>
                  <GlobalOutlined />
                  Compliance
                </span>
              } 
              key="compliance"
            >
              <div className="max-w-4xl mx-auto">
                <Row gutter={[32, 32]}>
                  <Col xs={24} md={12}>
                    <Card className="h-full">
                      <Title level={4} className="text-red-600 mb-4">
                        <SafetyOutlined className="mr-2" />
                        Regulatory Compliance
                      </Title>
                      <ul className="space-y-3 text-gray-700">
                        <li><strong>Banking:</strong> Basel III, PCI DSS, SOX compliance</li>
                        <li><strong>Government:</strong> FedRAMP, FISMA, NIST frameworks</li>
                        <li><strong>Privacy:</strong> GDPR, CCPA, data sovereignty</li>
                        <li><strong>Security:</strong> ISO 27001, SOC 2 Type II</li>
                        <li><strong>Audit:</strong> Immutable audit trails and reporting</li>
                      </ul>
                    </Card>
                  </Col>
                  <Col xs={24} md={12}>
                    <Card className="h-full">
                      <Title level={4} className="text-purple-600 mb-4">
                        <LockOutlined className="mr-2" />
                        Data Protection
                      </Title>
                      <ul className="space-y-3 text-gray-700">
                        <li><strong>Encryption:</strong> End-to-end encryption at rest and in transit</li>
                        <li><strong>Access Control:</strong> Role-based permissions and MFA</li>
                        <li><strong>Data Residency:</strong> Geographic data localization</li>
                        <li><strong>Backup & Recovery:</strong> Automated disaster recovery</li>
                        <li><strong>Monitoring:</strong> 24/7 security monitoring and alerts</li>
                      </ul>
                    </Card>
                  </Col>
                </Row>
              </div>
            </TabPane>

            <TabPane 
              tab={
                <span>
                  <ClusterOutlined />
                  BPI OS Components
                </span>
              } 
              key="bpi-os"
            >
              <div className="max-w-6xl mx-auto">
                <div className="text-center mb-12">
                  <Title level={3} className="text-2xl font-bold mb-4">BPI Operating System Architecture (Experimental)</Title>
                  <Paragraph className="text-lg text-gray-600">
                    Experimental infrastructure stack with 16+ core components being developed and tested. 
                    <strong>These are concepts in R&D phase, not production systems.</strong>
                  </Paragraph>
                  
                  <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-4 mt-6 max-w-4xl mx-auto">
                    <p className="text-yellow-800 text-sm">
                      <strong>Reality Check:</strong> The components below represent our development goals and experimental implementations. 
                      Most are in early testing phases and require significant work before any production consideration.
                    </p>
                  </div>
                </div>

                <Row gutter={[24, 24]}>
                  <Col xs={24} md={12} lg={8}>
                    <Card className="h-full border-l-4 border-l-blue-500" style={{ background: 'linear-gradient(135deg, #f8fafc 0%, #e2e8f0 100%)' }}>
                      <div className="flex items-center mb-4">
                        <DatabaseOutlined className="text-2xl text-blue-600 mr-3" />
                        <Title level={4} className="mb-0 text-blue-700">Core Infrastructure</Title>
                      </div>
                      <ul className="space-y-2 text-gray-700">
                        <li><Badge status="processing" /> <strong>BPI Ledger:</strong> Distributed blockchain ledger</li>
                        <li><Badge status="processing" /> <strong>Registry System:</strong> Node and wallet registration</li>
                        <li><Badge status="processing" /> <strong>Economic Engine:</strong> 4-coin economy (GEN/NEX/FLX/AUR)</li>
                        <li><Badge status="processing" /> <strong>Consensus Layer:</strong> Byzantine fault tolerant</li>
                        <li><Badge status="processing" /> <strong>Storage Layer:</strong> Distributed data management</li>
                      </ul>
                    </Card>
                  </Col>

                  <Col xs={24} md={12} lg={8}>
                    <Card className="h-full border-l-4 border-l-green-500" style={{ background: 'linear-gradient(135deg, #f0fdf4 0%, #dcfce7 100%)' }}>
                      <div className="flex items-center mb-4">
                        <SecurityScanOutlined className="text-2xl text-green-600 mr-3" />
                        <Title level={4} className="mb-0 text-green-700">Security & Crypto</Title>
                      </div>
                      <ul className="space-y-2 text-gray-700">
                        <li><Badge status="success" /> <strong>ENC Lock:</strong> Post-quantum ready encryption</li>
                        <li><Badge status="success" /> <strong>QLOCK Gates:</strong> Synchronization protocols</li>
                        <li><Badge status="success" /> <strong>Ed25519:</strong> Elliptic curve signatures</li>
                        <li><Badge status="success" /> <strong>Blake3:</strong> Domain-separated hashing</li>
                        <li><Badge status="success" /> <strong>ZK Proofs:</strong> Privacy-preserving validation</li>
                      </ul>
                    </Card>
                  </Col>

                  <Col xs={24} md={12} lg={8}>
                    <Card className="h-full border-l-4 border-l-purple-500" style={{ background: 'linear-gradient(135deg, #faf5ff 0%, #e9d5ff 100%)' }}>
                      <div className="flex items-center mb-4">
                        <ApiOutlined className="text-2xl text-purple-600 mr-3" />
                        <Title level={4} className="mb-0 text-purple-700">Network & APIs</Title>
                      </div>
                      <ul className="space-y-2 text-gray-700">
                        <li><Badge status="default" /> <strong>HTTP/CG Gateway:</strong> Communication gateway</li>
                        <li><Badge status="default" /> <strong>API Router:</strong> Request routing and load balancing</li>
                        <li><Badge status="default" /> <strong>WebSocket Server:</strong> Real-time communication</li>
                        <li><Badge status="default" /> <strong>P2P Network:</strong> Peer-to-peer connectivity</li>
                        <li><Badge status="default" /> <strong>Oracle Bridge:</strong> External data integration</li>
                      </ul>
                    </Card>
                  </Col>

                  <Col xs={24} md={12} lg={8}>
                    <Card className="h-full border-l-4 border-l-orange-500" style={{ background: 'linear-gradient(135deg, #fffbeb 0%, #fed7aa 100%)' }}>
                      <div className="flex items-center mb-4">
                        <FireOutlined className="text-2xl text-orange-600 mr-3" />
                        <Title level={4} className="mb-0 text-orange-700">Infrastructure Services</Title>
                      </div>
                      <ul className="space-y-2 text-gray-700">
                        <li><Badge status="warning" /> <strong>CueDB:</strong> Advanced database infrastructure</li>
                        <li><Badge status="warning" /> <strong>Firewall:</strong> Network security and filtering</li>
                        <li><Badge status="warning" /> <strong>Load Balancer:</strong> Traffic distribution</li>
                        <li><Badge status="warning" /> <strong>Cache Layer:</strong> Performance optimization</li>
                        <li><Badge status="warning" /> <strong>Monitoring:</strong> System health tracking</li>
                      </ul>
                    </Card>
                  </Col>

                  <Col xs={24} md={12} lg={8}>
                    <Card className="h-full border-l-4 border-l-red-500" style={{ background: 'linear-gradient(135deg, #fef2f2 0%, #fecaca 100%)' }}>
                      <div className="flex items-center mb-4">
                        <ControlOutlined className="text-2xl text-red-600 mr-3" />
                        <Title level={4} className="mb-0 text-red-700">Orchestration</Title>
                      </div>
                      <ul className="space-y-2 text-gray-700">
                        <li><Badge status="error" /> <strong>Security Orchestration:</strong> Automated security management</li>
                        <li><Badge status="error" /> <strong>Node Orchestrator:</strong> Distributed node management</li>
                        <li><Badge status="error" /> <strong>Service Mesh:</strong> Microservice communication</li>
                        <li><Badge status="error" /> <strong>Config Manager:</strong> Dynamic configuration</li>
                        <li><Badge status="error" /> <strong>Health Monitor:</strong> System diagnostics</li>
                      </ul>
                    </Card>
                  </Col>

                  <Col xs={24} md={12} lg={8}>
                    <Card className="h-full border-l-4 border-l-cyan-500" style={{ background: 'linear-gradient(135deg, #ecfeff 0%, #a5f3fc 100%)' }}>
                      <div className="flex items-center mb-4">
                        <MonitorOutlined className="text-2xl text-cyan-600 mr-3" />
                        <Title level={4} className="mb-0 text-cyan-700">Management & Tools</Title>
                      </div>
                      <ul className="space-y-2 text-gray-700">
                        <li><Badge status="processing" /> <strong>Admin Dashboard:</strong> System administration</li>
                        <li><Badge status="processing" /> <strong>CLI Tools:</strong> Command-line interface</li>
                        <li><Badge status="processing" /> <strong>Backup System:</strong> Data backup and recovery</li>
                        <li><Badge status="processing" /> <strong>Log Aggregator:</strong> Centralized logging</li>
                        <li><Badge status="processing" /> <strong>Metrics Collector:</strong> Performance analytics</li>
                      </ul>
                    </Card>
                  </Col>
                </Row>
              </div>
            </TabPane>

            <TabPane 
              tab={
                <span>
                  <RocketOutlined />
                  BPCI Innovations
                </span>
              } 
              key="bpci-innovations"
            >
              <div className="max-w-6xl mx-auto">
                <div className="text-center mb-12">
                  <Title level={3} className="text-2xl font-bold mb-4">BPCI Blockchain Innovations (Experimental)</Title>
                  <Paragraph className="text-lg text-gray-600">
                    Experimental consensus mechanisms, Merkle tree concepts, and auction-based mempool prototypes. 
                    <strong>These are research implementations being tested in controlled environments.</strong>
                  </Paragraph>
                  
                  <div className="bg-red-50 border border-red-200 rounded-lg p-4 mt-6 max-w-4xl mx-auto">
                    <p className="text-red-800 text-sm">
                      <strong>Important:</strong> The innovations described below are experimental prototypes. 
                      They have not undergone comprehensive security audits and are not suitable for production use. 
                      We're seeking partners to help validate these concepts.
                    </p>
                  </div>
                </div>

                <Collapse 
                  defaultActiveKey={['consensus']} 
                  size="large"
                  style={{ background: 'transparent' }}
                >
                  <Panel 
                    header={
                      <div className="flex items-center">
                        <NodeIndexOutlined className="text-xl text-blue-600 mr-3" />
                        <Text strong className="text-lg">Advanced Consensus Mechanism</Text>
                      </div>
                    } 
                    key="consensus"
                    style={{ marginBottom: '16px', borderRadius: '8px' }}
                  >
                    <Row gutter={[24, 24]}>
                      <Col xs={24} md={12}>
                        <Card className="h-full">
                          <Title level={5} className="text-blue-600 mb-3">Triple Consensus Coordinator</Title>
                          <ul className="space-y-2 text-gray-700">
                            <li>• <strong>Validator Selection:</strong> Stake-weighted random selection</li>
                            <li>• <strong>Block Proposal:</strong> Round-robin with fallback mechanisms</li>
                            <li>• <strong>Finality:</strong> 2/3+ validator agreement required</li>
                            <li>• <strong>Fork Resolution:</strong> Longest valid chain rule</li>
                            <li>• <strong>Slashing:</strong> Penalty for malicious behavior</li>
                          </ul>
                        </Card>
                      </Col>
                      <Col xs={24} md={12}>
                        <Card className="h-full">
                          <Title level={5} className="text-green-600 mb-3">Byzantine Fault Tolerance</Title>
                          <ul className="space-y-2 text-gray-700">
                            <li>• <strong>Safety:</strong> No conflicting blocks finalized</li>
                            <li>• <strong>Liveness:</strong> Progress guaranteed with 2/3+ honest nodes</li>
                            <li>• <strong>Accountability:</strong> Provable evidence of violations</li>
                            <li>• <strong>Recovery:</strong> Automatic network healing</li>
                            <li>• <strong>Threshold:</strong> Tolerates up to 1/3 malicious nodes</li>
                          </ul>
                        </Card>
                      </Col>
                    </Row>
                  </Panel>

                  <Panel 
                    header={
                      <div className="flex items-center">
                        <DatabaseOutlined className="text-xl text-green-600 mr-3" />
                        <Text strong className="text-lg">Optimized Merkle Tree Implementation</Text>
                      </div>
                    } 
                    key="merkle"
                    style={{ marginBottom: '16px', borderRadius: '8px' }}
                  >
                    <Row gutter={[24, 24]}>
                      <Col xs={24} md={12}>
                        <Card className="h-full">
                          <Title level={5} className="text-green-600 mb-3">Tree Structure Optimizations</Title>
                          <ul className="space-y-2 text-gray-700">
                            <li>• <strong>Blake3 Hashing:</strong> Fast, secure hash function</li>
                            <li>• <strong>Balanced Trees:</strong> Optimal depth for verification</li>
                            <li>• <strong>Incremental Updates:</strong> Efficient tree modifications</li>
                            <li>• <strong>Proof Compression:</strong> Minimal proof sizes</li>
                            <li>• <strong>Parallel Processing:</strong> Multi-threaded tree operations</li>
                          </ul>
                        </Card>
                      </Col>
                      <Col xs={24} md={12}>
                        <Card className="h-full">
                          <Title level={5} className="text-purple-600 mb-3">Verification & Proofs</Title>
                          <ul className="space-y-2 text-gray-700">
                            <li>• <strong>Inclusion Proofs:</strong> Verify transaction membership</li>
                            <li>• <strong>Non-inclusion Proofs:</strong> Prove absence of data</li>
                            <li>• <strong>Range Proofs:</strong> Validate value ranges</li>
                            <li>• <strong>Batch Verification:</strong> Multiple proofs at once</li>
                            <li>• <strong>Zero-Knowledge:</strong> Privacy-preserving proofs</li>
                          </ul>
                        </Card>
                      </Col>
                    </Row>
                  </Panel>

                  <Panel 
                    header={
                      <div className="flex items-center">
                        <ThunderboltOutlined className="text-xl text-orange-600 mr-3" />
                        <Text strong className="text-lg">Auction-Based Mempool Architecture</Text>
                      </div>
                    } 
                    key="mempool"
                    style={{ marginBottom: '16px', borderRadius: '8px' }}
                  >
                    <Row gutter={[24, 24]}>
                      <Col xs={24} md={12}>
                        <Card className="h-full">
                          <Title level={5} className="text-orange-600 mb-3">Auction Mechanism</Title>
                          <ul className="space-y-2 text-gray-700">
                            <li>• <strong>Bid-Based Ordering:</strong> Effective bid rate calculation</li>
                            <li>• <strong>Gas Price Discovery:</strong> Market-driven pricing</li>
                            <li>• <strong>Revenue Sharing:</strong> 25% to community partners</li>
                            <li>• <strong>Auction Windows:</strong> Time-bounded auction periods</li>
                            <li>• <strong>Fair Ordering:</strong> Prevents front-running attacks</li>
                          </ul>
                        </Card>
                      </Col>
                      <Col xs={24} md={12}>
                        <Card className="h-full">
                          <Title level={5} className="text-red-600 mb-3">Performance Features</Title>
                          <ul className="space-y-2 text-gray-700">
                            <li>• <strong>Bucketed Mempool:</strong> Organized by gas price tiers</li>
                            <li>• <strong>Parallel Processing:</strong> Concurrent transaction validation</li>
                            <li>• <strong>Gas Estimation:</strong> Accurate gas cost prediction</li>
                            <li>• <strong>Transaction Replacement:</strong> RBF (Replace-By-Fee) support</li>
                            <li>• <strong>Multi-Chain Support:</strong> Cross-chain transaction handling</li>
                          </ul>
                        </Card>
                      </Col>
                    </Row>
                  </Panel>

                  <Panel 
                    header={
                      <div className="flex items-center">
                        <SettingOutlined className="text-xl text-purple-600 mr-3" />
                        <Text strong className="text-lg">Infrastructure Innovations</Text>
                      </div>
                    } 
                    key="infrastructure"
                    style={{ borderRadius: '8px' }}
                  >
                    <Row gutter={[24, 24]}>
                      <Col xs={24} md={8}>
                        <Card className="h-full text-center">
                          <DatabaseOutlined className="text-3xl text-blue-600 mb-3" />
                          <Title level={5} className="text-blue-600 mb-3">CueDB Infrastructure</Title>
                          <ul className="space-y-1 text-gray-700 text-left">
                            <li>• Advanced query optimization</li>
                            <li>• Distributed data storage</li>
                            <li>• Real-time analytics</li>
                            <li>• ACID compliance</li>
                          </ul>
                        </Card>
                      </Col>
                      <Col xs={24} md={8}>
                        <Card className="h-full text-center">
                          <SafetyOutlined className="text-3xl text-green-600 mb-3" />
                          <Title level={5} className="text-green-600 mb-3">Security Orchestration</Title>
                          <ul className="space-y-1 text-gray-700 text-left">
                            <li>• Automated threat detection</li>
                            <li>• Dynamic firewall rules</li>
                            <li>• Intrusion prevention</li>
                            <li>• Security policy enforcement</li>
                          </ul>
                        </Card>
                      </Col>
                      <Col xs={24} md={8}>
                        <Card className="h-full text-center">
                          <GlobalOutlined className="text-3xl text-purple-600 mb-3" />
                          <Title level={5} className="text-purple-600 mb-3">HTTP/CG Gateway</Title>
                          <ul className="space-y-1 text-gray-700 text-left">
                            <li>• Protocol translation</li>
                            <li>• Load balancing</li>
                            <li>• Rate limiting</li>
                            <li>• API versioning</li>
                          </ul>
                        </Card>
                      </Col>
                    </Row>
                  </Panel>
                </Collapse>
              </div>
            </TabPane>
          </Tabs>
        </div>
      </section>

      {/* Implementation Status */}
      <section className="py-20 bg-white">
        <div className="max-w-6xl mx-auto px-4">
          <div className="text-center mb-16">
            <Title level={2} className="text-4xl font-bold mb-6">Implementation Status</Title>
            <Paragraph className="text-xl text-gray-600">
              Current development progress across all technology components
            </Paragraph>
          </div>

          <Row gutter={[32, 32]}>
            <Col xs={24} md={6}>
              <Card className="text-center p-6">
                <SecurityScanOutlined className="text-4xl text-blue-600 mb-4" />
                <Title level={4} className="mb-2">Security Layer</Title>
                <Progress 
                  type="circle" 
                  percent={95} 
                  strokeColor="#0066cc"
                  size={80}
                />
                <Paragraph className="text-gray-600 mt-2">Production Ready</Paragraph>
              </Card>
            </Col>
            <Col xs={24} md={6}>
              <Card className="text-center p-6">
                <DatabaseOutlined className="text-4xl text-green-600 mb-4" />
                <Title level={4} className="mb-2">Blockchain Core</Title>
                <Progress 
                  type="circle" 
                  percent={90} 
                  strokeColor="#059669"
                  size={80}
                />
                <Paragraph className="text-gray-600 mt-2">Active Testnet</Paragraph>
              </Card>
            </Col>
            <Col xs={24} md={6}>
              <Card className="text-center p-6">
                <ApiOutlined className="text-4xl text-purple-600 mb-4" />
                <Title level={4} className="mb-2">Enterprise APIs</Title>
                <Progress 
                  type="circle" 
                  percent={85} 
                  strokeColor="#7c3aed"
                  size={80}
                />
                <Paragraph className="text-gray-600 mt-2">Beta Testing</Paragraph>
              </Card>
            </Col>
            <Col xs={24} md={6}>
              <Card className="text-center p-6">
                <GlobalOutlined className="text-4xl text-orange-600 mb-4" />
                <Title level={4} className="mb-2">Compliance</Title>
                <Progress 
                  type="circle" 
                  percent={80} 
                  strokeColor="#ea580c"
                  size={80}
                />
                <Paragraph className="text-gray-600 mt-2">In Development</Paragraph>
              </Card>
            </Col>
          </Row>
        </div>
      </section>

      {/* Technical Documentation */}
      <section className="py-20 bg-gray-50">
        <div className="max-w-4xl mx-auto px-4 text-center">
          <Title level={2} className="text-4xl font-bold mb-8">Technical Documentation</Title>
          <Card className="p-8">
            <Paragraph className="text-lg text-gray-700 mb-6">
              Comprehensive technical documentation, API references, and implementation 
              guides are available for developers and enterprise integrators.
            </Paragraph>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mt-8">
              <Card className="text-center p-4 hover:shadow-md transition-shadow">
                <CodeOutlined className="text-2xl text-blue-600 mb-2" />
                <Title level={5}>API Reference</Title>
                <Paragraph className="text-sm text-gray-600">Complete API documentation</Paragraph>
              </Card>
              <Card className="text-center p-4 hover:shadow-md transition-shadow">
                <DatabaseOutlined className="text-2xl text-green-600 mb-2" />
                <Title level={5}>Architecture Guide</Title>
                <Paragraph className="text-sm text-gray-600">System architecture details</Paragraph>
              </Card>
              <Card className="text-center p-4 hover:shadow-md transition-shadow">
                <SafetyOutlined className="text-2xl text-purple-600 mb-2" />
                <Title level={5}>Security Specs</Title>
                <Paragraph className="text-sm text-gray-600">Security implementation specs</Paragraph>
              </Card>
            </div>
          </Card>
        </div>
      </section>
    </div>
  );
};

export default Technology;
