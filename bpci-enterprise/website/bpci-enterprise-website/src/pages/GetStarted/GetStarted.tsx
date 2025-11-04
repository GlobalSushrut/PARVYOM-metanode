import React, { useState } from 'react';
import { Typography, Card, Button, Row, Col, Tabs, Alert, Collapse } from 'antd';
import { 
  DownloadOutlined, 
  AppleOutlined, 
  WindowsOutlined, 
  CodeOutlined,
  SafetyOutlined,
  WalletOutlined,
  ApiOutlined,
  CloudServerOutlined,
  CheckCircleOutlined,
  InfoCircleOutlined
} from '@ant-design/icons';

const { Title, Paragraph, Text } = Typography;
const { TabPane } = Tabs;
const { Panel } = Collapse;

const GetStarted: React.FC = () => {
  const [activeTab, setActiveTab] = useState('overview');

  return (
    <div style={{ minHeight: '100vh', background: '#0A1628', padding: '2rem 0' }}>
      {/* Hero Section */}
      <div style={{ maxWidth: '1200px', margin: '0 auto', padding: '0 2rem', marginBottom: '3rem' }}>
        <div style={{ textAlign: 'center', marginBottom: '2rem' }}>
          <Title level={1} style={{ color: '#ffffff', fontSize: '3rem', marginBottom: '1rem' }}>
            Get Started with BPCI Infrastructure
          </Title>
          <Paragraph style={{ color: '#ffffff', fontSize: '1.25rem', maxWidth: '800px', margin: '0 auto' }}>
            Download BPI OS, SDK, and set up your development environment. Everything you need to build on the Pravyom network.
          </Paragraph>
        </div>

        {/* Main Tabs */}
        <Tabs 
          activeKey={activeTab} 
          onChange={setActiveTab}
          centered
          size="large"
          style={{ marginTop: '2rem' }}
        >
          <TabPane tab="📋 Overview" key="overview">
            <div style={{ maxWidth: '900px', margin: '0 auto' }}>
              {/* What is BPI OS */}
              <Card style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(232, 180, 79, 0.3)', borderRadius: '12px', marginBottom: '2rem', backdropFilter: 'blur(10px)' }}>
                <Title level={3} style={{ color: '#E8B44F', marginBottom: '1rem' }}>
                  <CloudServerOutlined /> What is BPI OS?
                </Title>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', marginBottom: '1rem' }}>
                  <strong style={{ color: '#E8B44F' }}>BPI OS (Blockchain Protocol Infrastructure Operating System)</strong> is a distributed operating system designed for running BPCI nodes and services. It provides:
                </Paragraph>
                <ul style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', paddingLeft: '2rem' }}>
                  <li><strong>DynaRoute v2:</strong> Dynamic port allocation and mesh networking</li>
                  <li><strong>vPod Orchestration:</strong> Virtual pod management for services</li>
                  <li><strong>CommuteLock:</strong> Lock-based inter-service communication</li>
                  <li><strong>15 Backend Services:</strong> Complete BPCI infrastructure stack</li>
                </ul>
              </Card>

              {/* Mojo Wallet & Auth */}
              <Card style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(232, 180, 79, 0.3)', borderRadius: '12px', marginBottom: '2rem', backdropFilter: 'blur(10px)' }}>
                <Title level={3} style={{ color: '#E8B44F', marginBottom: '1rem' }}>
                  <WalletOutlined /> Mojo Wallet & Authentication
                </Title>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', marginBottom: '1rem' }}>
                  <strong style={{ color: '#E8B44F' }}>Mojo Wallet</strong> is your identity and authentication system on the Pravyom network:
                </Paragraph>
                <ul style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', paddingLeft: '2rem', marginBottom: '1rem' }}>
                  <li><strong>Developer Authentication:</strong> Proves you're a legitimate developer</li>
                  <li><strong>Wallet Types:</strong> Community, Investor, Government, Bank, Owner, ESOP, Treasury, Company</li>
                  <li><strong>Mother Coin Allocation:</strong> Receive initial coin allocation upon activation</li>
                  <li><strong>Baby Coin Balance:</strong> Earn through Proof-of-Existence (PoE) mining</li>
                </ul>
                <Alert
                  message="Required for Development"
                  description="You must activate a Mojo wallet to access developer features, create blog posts, and interact with the network."
                  type="info"
                  showIcon
                  icon={<InfoCircleOutlined />}
                  style={{ background: 'rgba(59, 130, 246, 0.1)', border: '1px solid rgba(59, 130, 246, 0.3)' }}
                />
              </Card>

              {/* BPCI Infrastructure */}
              <Card style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(232, 180, 79, 0.3)', borderRadius: '12px', marginBottom: '2rem', backdropFilter: 'blur(10px)' }}>
                <Title level={3} style={{ color: '#E8B44F', marginBottom: '1rem' }}>
                  <ApiOutlined /> BPCI Infrastructure Components
                </Title>
                <Row gutter={[16, 16]}>
                  <Col xs={24} md={12}>
                    <div style={{ padding: '1rem', background: 'rgba(16, 185, 129, 0.1)', borderRadius: '8px', border: '1px solid rgba(16, 185, 129, 0.3)' }}>
                      <strong style={{ color: '#10B981' }}>Core Services (9)</strong>
                      <ul style={{ color: '#ffffff', fontSize: '0.875rem', paddingLeft: '1rem', marginTop: '0.5rem', marginBottom: 0 }}>
                        <li>API Gateway</li>
                        <li>Auction Mempool</li>
                        <li>Auction DB Maintainer</li>
                        <li>BPI Bridge</li>
                        <li>BSO K8 Orchestrator</li>
                        <li>Cluster Ledger</li>
                        <li>Mojo Service</li>
                        <li>Network Service</li>
                        <li>Shadow Registry</li>
                      </ul>
                    </div>
                  </Col>
                  <Col xs={24} md={12}>
                    <div style={{ padding: '1rem', background: 'rgba(124, 58, 237, 0.1)', borderRadius: '8px', border: '1px solid rgba(124, 58, 237, 0.3)' }}>
                      <strong style={{ color: '#7C3AED' }}>Critical Services (2)</strong>
                      <ul style={{ color: '#ffffff', fontSize: '0.875rem', paddingLeft: '1rem', marginTop: '0.5rem', marginBottom: 0 }}>
                        <li>Consensus (LCCD)</li>
                        <li>Blockchain (6D)</li>
                      </ul>
                      <div style={{ marginTop: '1rem', fontSize: '0.875rem', color: '#ffffff' }}>
                        <strong style={{ color: '#7C3AED' }}>Databases (4)</strong>
                        <ul style={{ paddingLeft: '1rem', marginTop: '0.5rem', marginBottom: 0 }}>
                          <li>PostgreSQL</li>
                          <li>Redis</li>
                          <li>MongoDB</li>
                          <li>RabbitMQ</li>
                        </ul>
                      </div>
                    </div>
                  </Col>
                </Row>
              </Card>
            </div>
          </TabPane>

          <TabPane tab="💻 Download BPI OS" key="bpios">
            <div style={{ maxWidth: '900px', margin: '0 auto' }}>
              <Alert
                message="Testnet Version Available"
                description="Current version: BPI OS v0.75 (75% complete). Mainnet launch depends on testing and pilot partnerships."
                type="warning"
                showIcon
                style={{ marginBottom: '2rem', background: 'rgba(245, 158, 11, 0.1)', border: '1px solid rgba(245, 158, 11, 0.3)' }}
              />

              {/* Linux Download */}
              <Card style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(232, 180, 79, 0.3)', borderRadius: '12px', marginBottom: '2rem', backdropFilter: 'blur(10px)' }}>
                <Row gutter={[24, 24]} align="middle">
                  <Col xs={24} md={4} style={{ textAlign: 'center' }}>
                    <CodeOutlined style={{ fontSize: '4rem', color: '#E8B44F' }} />
                  </Col>
                  <Col xs={24} md={14}>
                    <Title level={4} style={{ color: '#E8B44F', marginBottom: '0.5rem' }}>BPI OS for Linux</Title>
                    <Paragraph style={{ color: '#ffffff', marginBottom: '0.5rem' }}>
                      Full BPI OS installation with all 15 services
                    </Paragraph>
                    <Text style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>
                      Version: 0.75 | Size: ~2.5 GB | Requires: Ubuntu 20.04+ or Debian 11+
                    </Text>
                  </Col>
                  <Col xs={24} md={6} style={{ textAlign: 'center' }}>
                    <Button
                      type="primary"
                      size="large"
                      icon={<DownloadOutlined />}
                      style={{
                        background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)',
                        border: 'none',
                        color: '#0A1628',
                        fontWeight: '600',
                        width: '100%'
                      }}
                    >
                      Download
                    </Button>
                  </Col>
                </Row>
              </Card>

              {/* macOS Download */}
              <Card style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(232, 180, 79, 0.3)', borderRadius: '12px', marginBottom: '2rem', backdropFilter: 'blur(10px)' }}>
                <Row gutter={[24, 24]} align="middle">
                  <Col xs={24} md={4} style={{ textAlign: 'center' }}>
                    <AppleOutlined style={{ fontSize: '4rem', color: '#E8B44F' }} />
                  </Col>
                  <Col xs={24} md={14}>
                    <Title level={4} style={{ color: '#E8B44F', marginBottom: '0.5rem' }}>BPI OS for macOS</Title>
                    <Paragraph style={{ color: '#ffffff', marginBottom: '0.5rem' }}>
                      BPI OS with native macOS support
                    </Paragraph>
                    <Text style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>
                      Version: 0.75 | Size: ~2.3 GB | Requires: macOS 11+ (Intel & Apple Silicon)
                    </Text>
                  </Col>
                  <Col xs={24} md={6} style={{ textAlign: 'center' }}>
                    <Button
                      type="primary"
                      size="large"
                      icon={<DownloadOutlined />}
                      style={{
                        background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)',
                        border: 'none',
                        color: '#0A1628',
                        fontWeight: '600',
                        width: '100%'
                      }}
                    >
                      Download
                    </Button>
                  </Col>
                </Row>
              </Card>

              {/* Windows Download */}
              <Card style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(232, 180, 79, 0.3)', borderRadius: '12px', marginBottom: '2rem', backdropFilter: 'blur(10px)' }}>
                <Row gutter={[24, 24]} align="middle">
                  <Col xs={24} md={4} style={{ textAlign: 'center' }}>
                    <WindowsOutlined style={{ fontSize: '4rem', color: '#E8B44F' }} />
                  </Col>
                  <Col xs={24} md={14}>
                    <Title level={4} style={{ color: '#E8B44F', marginBottom: '0.5rem' }}>BPI OS for Windows</Title>
                    <Paragraph style={{ color: '#ffffff', marginBottom: '0.5rem' }}>
                      BPI OS with WSL2 integration
                    </Paragraph>
                    <Text style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>
                      Version: 0.75 | Size: ~2.7 GB | Requires: Windows 10/11 with WSL2
                    </Text>
                  </Col>
                  <Col xs={24} md={6} style={{ textAlign: 'center' }}>
                    <Button
                      type="primary"
                      size="large"
                      icon={<DownloadOutlined />}
                      style={{
                        background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)',
                        border: 'none',
                        color: '#0A1628',
                        fontWeight: '600',
                        width: '100%'
                      }}
                    >
                      Download
                    </Button>
                  </Col>
                </Row>
              </Card>

              {/* Installation Instructions */}
              <Card style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(16, 185, 129, 0.3)', borderRadius: '12px', backdropFilter: 'blur(10px)' }}>
                <Title level={4} style={{ color: '#10B981', marginBottom: '1rem' }}>
                  <CheckCircleOutlined /> Quick Installation
                </Title>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8' }}>
                  After downloading, run the installer:
                </Paragraph>
                <div style={{ background: '#1e293b', padding: '1rem', borderRadius: '8px', marginBottom: '1rem' }}>
                  <code style={{ color: '#10B981', fontFamily: 'monospace' }}>
                    # Linux/macOS<br />
                    chmod +x bpios-installer.sh<br />
                    ./bpios-installer.sh<br />
                    <br />
                    # Windows (PowerShell)<br />
                    .\bpios-installer.exe
                  </code>
                </div>
                <Paragraph style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>
                  The installer will set up all 15 services, databases, and configure DynaRoute v2 networking automatically.
                </Paragraph>
              </Card>
            </div>
          </TabPane>

          <TabPane tab="🛠️ SDK & Tools" key="sdk">
            <div style={{ maxWidth: '900px', margin: '0 auto' }}>
              {/* Rust SDK */}
              <Card style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(232, 180, 79, 0.3)', borderRadius: '12px', marginBottom: '2rem', backdropFilter: 'blur(10px)' }}>
                <Row gutter={[24, 24]} align="middle">
                  <Col xs={24} md={4} style={{ textAlign: 'center' }}>
                    <CodeOutlined style={{ fontSize: '4rem', color: '#E8B44F' }} />
                  </Col>
                  <Col xs={24} md={14}>
                    <Title level={4} style={{ color: '#E8B44F', marginBottom: '0.5rem' }}>BPCI Rust SDK</Title>
                    <Paragraph style={{ color: '#ffffff', marginBottom: '0.5rem' }}>
                      Complete Rust SDK for building on BPCI infrastructure
                    </Paragraph>
                    <Text style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>
                      Version: 0.75 | Size: ~150 MB | Includes: API clients, wallet integration, examples
                    </Text>
                  </Col>
                  <Col xs={24} md={6} style={{ textAlign: 'center' }}>
                    <Button
                      type="primary"
                      size="large"
                      icon={<DownloadOutlined />}
                      style={{
                        background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)',
                        border: 'none',
                        color: '#0A1628',
                        fontWeight: '600',
                        width: '100%'
                      }}
                    >
                      Download SDK
                    </Button>
                  </Col>
                </Row>
              </Card>

              {/* CLI Tools */}
              <Card style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(232, 180, 79, 0.3)', borderRadius: '12px', marginBottom: '2rem', backdropFilter: 'blur(10px)' }}>
                <Title level={4} style={{ color: '#E8B44F', marginBottom: '1rem' }}>Command Line Tools</Title>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', marginBottom: '1rem' }}>
                  Essential CLI tools for BPCI development:
                </Paragraph>
                <ul style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', paddingLeft: '2rem' }}>
                  <li><strong>bpci-cli:</strong> Main CLI for interacting with BPCI services</li>
                  <li><strong>mojo-wallet-cli:</strong> Wallet management and transactions</li>
                  <li><strong>vpod-manager:</strong> vPod orchestration and deployment</li>
                  <li><strong>dynaroute-config:</strong> DynaRoute v2 configuration tool</li>
                </ul>
                <div style={{ marginTop: '1rem' }}>
                  <Button
                    size="large"
                    icon={<DownloadOutlined />}
                    style={{
                      background: 'transparent',
                      border: '2px solid #E8B44F',
                      color: '#E8B44F',
                      fontWeight: '600'
                    }}
                  >
                    Download CLI Tools
                  </Button>
                </div>
              </Card>

              {/* SDK Usage Example */}
              <Card style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(16, 185, 129, 0.3)', borderRadius: '12px', backdropFilter: 'blur(10px)' }}>
                <Title level={4} style={{ color: '#10B981', marginBottom: '1rem' }}>Quick Start Example</Title>
                <div style={{ background: '#1e293b', padding: '1rem', borderRadius: '8px' }}>
                  <pre style={{ color: '#10B981', fontFamily: 'monospace', fontSize: '0.875rem', margin: 0, whiteSpace: 'pre-wrap' }}>
{`use bpci_sdk::{Client, MojoWallet};

// Initialize client
let client = Client::new("http://localhost:8888")?;

// Connect wallet
let wallet = MojoWallet::from_credentials(email, password)?;

// Make API call
let balance = client.get_wallet_balance(&wallet).await?;`}
                  </pre>
                </div>
              </Card>
            </div>
          </TabPane>

          <TabPane tab="📚 Documentation" key="docs">
            <div style={{ maxWidth: '900px', margin: '0 auto' }}>
              <Card style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(232, 180, 79, 0.3)', borderRadius: '12px', marginBottom: '2rem', backdropFilter: 'blur(10px)' }}>
                <Title level={3} style={{ color: '#E8B44F', marginBottom: '1.5rem' }}>Developer Resources</Title>
                
                <Collapse
                  bordered={false}
                  style={{ background: 'transparent' }}
                  expandIconPosition="end"
                >
                  <Panel
                    header={<span style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>Getting Started Guide</span>}
                    key="1"
                    style={{ background: 'rgba(232, 180, 79, 0.1)', border: '1px solid rgba(232, 180, 79, 0.3)', borderRadius: '8px', marginBottom: '1rem' }}
                  >
                    <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8' }}>
                      Complete guide to setting up your development environment, activating your Mojo wallet, and making your first API call.
                    </Paragraph>
                    <Button type="link" style={{ color: '#10B981', padding: 0 }}>
                      Read Guide →
                    </Button>
                  </Panel>

                  <Panel
                    header={<span style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>API Reference</span>}
                    key="2"
                    style={{ background: 'rgba(232, 180, 79, 0.1)', border: '1px solid rgba(232, 180, 79, 0.3)', borderRadius: '8px', marginBottom: '1rem' }}
                  >
                    <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8' }}>
                      Complete API documentation for all 15 BPCI services, including endpoints, parameters, and response formats.
                    </Paragraph>
                    <Button type="link" style={{ color: '#10B981', padding: 0 }}>
                      View API Docs →
                    </Button>
                  </Panel>

                  <Panel
                    header={<span style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>Architecture Overview</span>}
                    key="3"
                    style={{ background: 'rgba(232, 180, 79, 0.1)', border: '1px solid rgba(232, 180, 79, 0.3)', borderRadius: '8px', marginBottom: '1rem' }}
                  >
                    <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8' }}>
                      Deep dive into BPCI architecture: DynaRoute v2, vPod orchestration, LCCD consensus, 6D blockchain, and more.
                    </Paragraph>
                    <Button type="link" style={{ color: '#10B981', padding: 0 }}>
                      Read Architecture →
                    </Button>
                  </Panel>

                  <Panel
                    header={<span style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>Code Examples</span>}
                    key="4"
                    style={{ background: 'rgba(232, 180, 79, 0.1)', border: '1px solid rgba(232, 180, 79, 0.3)', borderRadius: '8px', marginBottom: '1rem' }}
                  >
                    <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8' }}>
                      Real-world code examples for common tasks: wallet integration, API calls, transaction handling, and more.
                    </Paragraph>
                    <Button type="link" style={{ color: '#10B981', padding: 0 }}>
                      Browse Examples →
                    </Button>
                  </Panel>
                </Collapse>
              </Card>

              <Card style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(16, 185, 129, 0.3)', borderRadius: '12px', backdropFilter: 'blur(10px)' }}>
                <Title level={4} style={{ color: '#10B981', marginBottom: '1rem' }}>Need Help?</Title>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', marginBottom: '1rem' }}>
                  Join our community or contact us directly:
                </Paragraph>
                <div style={{ display: 'flex', gap: '1rem', flexWrap: 'wrap' }}>
                  <Button size="large" style={{ background: 'transparent', border: '2px solid #10B981', color: '#10B981', fontWeight: '600' }}>
                    Community Forum
                  </Button>
                  <Button size="large" style={{ background: 'transparent', border: '2px solid #10B981', color: '#10B981', fontWeight: '600' }}>
                    Discord
                  </Button>
                  <Button size="large" style={{ background: 'transparent', border: '2px solid #10B981', color: '#10B981', fontWeight: '600' }}>
                    Email: umesh@pravyom.com
                  </Button>
                </div>
              </Card>
            </div>
          </TabPane>
        </Tabs>
      </div>
    </div>
  );
};

export default GetStarted;
