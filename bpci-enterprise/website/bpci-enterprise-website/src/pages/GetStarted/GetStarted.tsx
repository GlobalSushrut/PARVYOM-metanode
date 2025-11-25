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
import { BpiOSDownloader } from '../../components/BPI/BpiOSDownloader';

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
              {/* Quick Start */}
              <Card style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(16, 185, 129, 0.3)', borderRadius: '12px', marginBottom: '2rem', backdropFilter: 'blur(10px)' }}>
                <Title level={3} style={{ color: '#10B981', marginBottom: '1rem' }}>
                  <CheckCircleOutlined /> Quick Start (3 Commands)
                </Title>
                <div style={{ background: 'rgba(0, 0, 0, 0.4)', padding: '1rem', borderRadius: '8px', marginBottom: '1rem' }}>
                  <pre style={{ color: '#10B981', margin: 0, fontSize: '0.9rem' }}>
{`# 1. Install BPI OS (Docker-like)
curl -fsSL https://get.bpi.dev | sh

# 2. Connect to BPCI via Cloudflare
bpi init --connect https://connect.pravyom.com

# 3. Start BPI OS (auto-connects)
bpi start`}
                  </pre>
                </div>
                <Alert
                  message="Production Ready Infrastructure"
                  description="All endpoints are validated and operational. Millions-scale onboarding architecture deployed with Cloudflare Workers."
                  type="success"
                  showIcon
                  style={{ background: 'rgba(16, 185, 129, 0.1)', border: '1px solid rgba(16, 185, 129, 0.3)' }}
                />
              </Card>

              {/* Production Infrastructure */}
              <Card style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(232, 180, 79, 0.3)', borderRadius: '12px', marginBottom: '2rem', backdropFilter: 'blur(10px)' }}>
                <Title level={3} style={{ color: '#E8B44F', marginBottom: '1rem' }}>
                  <CloudServerOutlined /> Production Infrastructure (Validated)
                </Title>
                <Row gutter={[16, 16]}>
                  <Col xs={24} md={12}>
                    <div style={{ padding: '1rem', background: 'rgba(16, 185, 129, 0.1)', borderRadius: '8px', border: '1px solid rgba(16, 185, 129, 0.3)' }}>
                      <strong style={{ color: '#10B981' }}>Cloudflare Endpoints ✓</strong>
                      <ul style={{ color: '#ffffff', fontSize: '0.875rem', paddingLeft: '1rem', marginTop: '0.5rem', marginBottom: 0 }}>
                        <li><code>connect.pravyom.com</code> - Node Connection</li>
                        <li><code>resolver.pravyom.com</code> - Address Resolution</li>
                        <li><code>api.pravyom.com</code> - API Gateway</li>
                        <li><code>explorer.pravyom.com</code> - Blockchain Explorer</li>
                      </ul>
                    </div>
                  </Col>
                  <Col xs={24} md={12}>
                    <div style={{ padding: '1rem', background: 'rgba(59, 130, 246, 0.1)', borderRadius: '8px', border: '1px solid rgba(59, 130, 246, 0.3)' }}>
                      <strong style={{ color: '#3B82F6' }}>BPCI Infrastructure ✓</strong>
                      <ul style={{ color: '#ffffff', fontSize: '0.875rem', paddingLeft: '1rem', marginTop: '0.5rem', marginBottom: 0 }}>
                        <li>BPI Bridge (Port 6001)</li>
                        <li>Cluster Ledger (Port 6002)</li>
                        <li>Blockchain Server (Port 7002)</li>
                        <li>XTMP Protocol (Port 7778)</li>
                      </ul>
                    </div>
                  </Col>
                </Row>
              </Card>

              {/* What is BPI OS */}
              <Card style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(232, 180, 79, 0.3)', borderRadius: '12px', marginBottom: '2rem', backdropFilter: 'blur(10px)' }}>
                <Title level={3} style={{ color: '#E8B44F', marginBottom: '1rem' }}>
                  <CloudServerOutlined /> What is BPI OS?
                </Title>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', marginBottom: '1rem' }}>
                  <strong style={{ color: '#E8B44F' }}>BPI OS (Blockchain Protocol Infrastructure Operating System)</strong> is a distributed operating system that connects to BPCI infrastructure via advanced Cloudflare configuration. It provides:
                </Paragraph>
                <ul style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', paddingLeft: '2rem' }}>
                  <li><strong>Docker-like Interface:</strong> Simple CLI commands (bpi start, bpi stop, bpi status)</li>
                  <li><strong>Cloudflare Integration:</strong> Millions-scale onboarding via connect.pravyom.com</li>
                  <li><strong>Auto-Connection:</strong> Automatic BPCI network registration and connection</li>
                  <li><strong>Production Ready:</strong> 1M+ address pool, quantum-resistant security</li>
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

          <TabPane tab="🌐 Cloudflare Config" key="cloudflare">
            <div style={{ maxWidth: '900px', margin: '0 auto' }}>
              {/* Connection Architecture */}
              <Card style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(16, 185, 129, 0.3)', borderRadius: '12px', marginBottom: '2rem', backdropFilter: 'blur(10px)' }}>
                <Title level={3} style={{ color: '#10B981', marginBottom: '1rem' }}>
                  <CloudServerOutlined /> Advanced Cloudflare Configuration
                </Title>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', marginBottom: '1rem' }}>
                  BPI OS uses advanced Cloudflare Workers for millions-scale onboarding and seamless BPCI network connection:
                </Paragraph>
                
                <Row gutter={[16, 16]}>
                  <Col xs={24} md={12}>
                    <div style={{ padding: '1.5rem', background: 'rgba(16, 185, 129, 0.1)', borderRadius: '8px', border: '1px solid rgba(16, 185, 129, 0.3)', marginBottom: '1rem' }}>
                      <Title level={4} style={{ color: '#10B981', marginBottom: '1rem' }}>Node Connection Handler</Title>
                      <div style={{ background: 'rgba(0, 0, 0, 0.4)', padding: '1rem', borderRadius: '6px', marginBottom: '1rem' }}>
                        <code style={{ color: '#10B981', fontSize: '0.875rem' }}>connect.pravyom.com</code>
                      </div>
                      <ul style={{ color: '#ffffff', fontSize: '0.875rem', paddingLeft: '1rem', marginBottom: 0 }}>
                        <li>BPI node registration & authentication</li>
                        <li>BPCI endpoint provisioning</li>
                        <li>Session management & tokens</li>
                        <li>Health monitoring</li>
                      </ul>
                    </div>
                  </Col>
                  <Col xs={24} md={12}>
                    <div style={{ padding: '1.5rem', background: 'rgba(59, 130, 246, 0.1)', borderRadius: '8px', border: '1px solid rgba(59, 130, 246, 0.3)', marginBottom: '1rem' }}>
                      <Title level={4} style={{ color: '#3B82F6', marginBottom: '1rem' }}>Address Resolver</Title>
                      <div style={{ background: 'rgba(0, 0, 0, 0.4)', padding: '1rem', borderRadius: '6px', marginBottom: '1rem' }}>
                        <code style={{ color: '#3B82F6', fontSize: '0.875rem' }}>resolver.pravyom.com</code>
                      </div>
                      <ul style={{ color: '#ffffff', fontSize: '0.875rem', paddingLeft: '1rem', marginBottom: 0 }}>
                        <li>Complex address resolution</li>
                        <li>1M+ address pool management</li>
                        <li>Database allocation system</li>
                        <li>Batch processing support</li>
                      </ul>
                    </div>
                  </Col>
                </Row>
              </Card>

              {/* Production Endpoints */}
              <Card style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(232, 180, 79, 0.3)', borderRadius: '12px', marginBottom: '2rem', backdropFilter: 'blur(10px)' }}>
                <Title level={3} style={{ color: '#E8B44F', marginBottom: '1rem' }}>
                  <ApiOutlined /> Production Endpoints (Validated ✓)
                </Title>
                
                <div style={{ background: 'rgba(0, 0, 0, 0.4)', padding: '1.5rem', borderRadius: '8px', marginBottom: '1rem' }}>
                  <pre style={{ color: '#E8B44F', margin: 0, fontSize: '0.875rem', lineHeight: '1.6' }}>
{`# Test Connection Endpoints
curl https://connect.pravyom.com/health
# Response: "BPI Node Connection Handler OK"

curl https://resolver.pravyom.com/health  
# Response: "Address Resolver OK"

curl https://api.pravyom.com/health
# Response: API Gateway status

curl https://explorer.pravyom.com
# Response: Blockchain explorer dashboard`}
                  </pre>
                </div>

                <Alert
                  message="All Endpoints Operational"
                  description="Production infrastructure validated with real connection tests. Ready for millions-scale onboarding."
                  type="success"
                  showIcon
                  style={{ background: 'rgba(16, 185, 129, 0.1)', border: '1px solid rgba(16, 185, 129, 0.3)' }}
                />
              </Card>

              {/* Pricing Tiers */}
              <Card style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(168, 85, 247, 0.3)', borderRadius: '12px', marginBottom: '2rem', backdropFilter: 'blur(10px)' }}>
                <Title level={3} style={{ color: '#A855F7', marginBottom: '1rem' }}>
                  <WalletOutlined /> BPI OS Pricing Tiers
                </Title>
                
                <Row gutter={[16, 16]}>
                  <Col xs={24} md={8}>
                    <div style={{ padding: '1.5rem', background: 'rgba(16, 185, 129, 0.1)', borderRadius: '8px', border: '1px solid rgba(16, 185, 129, 0.3)', textAlign: 'center' }}>
                      <Title level={4} style={{ color: '#10B981', marginBottom: '0.5rem' }}>Testnet</Title>
                      <div style={{ fontSize: '2rem', fontWeight: 'bold', color: '#10B981', marginBottom: '0.5rem' }}>$10 CAD</div>
                      <div style={{ color: '#9CA3AF', fontSize: '0.875rem', marginBottom: '1rem' }}>per month</div>
                      <ul style={{ color: '#ffffff', fontSize: '0.875rem', textAlign: 'left', paddingLeft: '1rem' }}>
                        <li>1000 BPI tokens</li>
                        <li>Basic VM Server</li>
                        <li>Local Consensus</li>
                        <li>BPCI Connection</li>
                      </ul>
                    </div>
                  </Col>
                  <Col xs={24} md={8}>
                    <div style={{ padding: '1.5rem', background: 'rgba(59, 130, 246, 0.1)', borderRadius: '8px', border: '1px solid rgba(59, 130, 246, 0.3)', textAlign: 'center' }}>
                      <Title level={4} style={{ color: '#3B82F6', marginBottom: '0.5rem' }}>Developer</Title>
                      <div style={{ fontSize: '2rem', fontWeight: 'bold', color: '#3B82F6', marginBottom: '0.5rem' }}>$25 CAD</div>
                      <div style={{ color: '#9CA3AF', fontSize: '0.875rem', marginBottom: '1rem' }}>per month</div>
                      <ul style={{ color: '#ffffff', fontSize: '0.875rem', textAlign: 'left', paddingLeft: '1rem' }}>
                        <li>2500 + 500 excess tokens</li>
                        <li>Full VM Cluster</li>
                        <li>Development Tools</li>
                        <li>API Access</li>
                      </ul>
                    </div>
                  </Col>
                  <Col xs={24} md={8}>
                    <div style={{ padding: '1.5rem', background: 'rgba(168, 85, 247, 0.1)', borderRadius: '8px', border: '1px solid rgba(168, 85, 247, 0.3)', textAlign: 'center' }}>
                      <Title level={4} style={{ color: '#A855F7', marginBottom: '0.5rem' }}>Enterprise</Title>
                      <div style={{ fontSize: '2rem', fontWeight: 'bold', color: '#A855F7', marginBottom: '0.5rem' }}>$50 CAD</div>
                      <div style={{ color: '#9CA3AF', fontSize: '0.875rem', marginBottom: '1rem' }}>per month</div>
                      <ul style={{ color: '#ffffff', fontSize: '0.875rem', textAlign: 'left', paddingLeft: '1rem' }}>
                        <li>5000 + 2000 excess tokens</li>
                        <li>Enterprise Consensus</li>
                        <li>Audit System</li>
                        <li>24/7 Support</li>
                      </ul>
                    </div>
                  </Col>
                </Row>
              </Card>

              {/* Connection Flow */}
              <Card style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(16, 185, 129, 0.3)', borderRadius: '12px', backdropFilter: 'blur(10px)' }}>
                <Title level={3} style={{ color: '#10B981', marginBottom: '1rem' }}>
                  <CheckCircleOutlined /> BPI→BPCI Connection Flow
                </Title>
                
                <div style={{ background: 'rgba(0, 0, 0, 0.4)', padding: '1.5rem', borderRadius: '8px', marginBottom: '1rem' }}>
                  <pre style={{ color: '#10B981', margin: 0, fontSize: '0.875rem', lineHeight: '1.8' }}>
{`1. BPI OS Registration
   ↓ POST /register → connect.pravyom.com
   ↓ Returns: node_id, connection_token, bpci_endpoints

2. Address Resolution  
   ↓ GET /resolve/{address} → resolver.pravyom.com
   ↓ Returns: resolved_endpoints, connection_info

3. BPCI Connection
   ↓ POST /connect → connect.pravyom.com
   ↓ Returns: session_id, active_endpoints

4. Transaction Processing
   ↓ POST /transaction/process → BPI Bridge (6001)
   ↓ XTMP Protocol → Auction → Blockchain (7002)`}
                  </pre>
                </div>

                <Paragraph style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>
                  All connections are secured with quantum-resistant encryption and managed through Cloudflare's global network for optimal performance and reliability.
                </Paragraph>
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
                  <pre style={{ color: '#10B981', fontFamily: 'monospace', margin: 0, fontSize: '0.875rem' }}>
{`# One-command install (Docker-like)
curl -fsSL https://get.bpi.dev | sh

# Or manual download and install
wget https://pravyom.com/downloads/bpi-os-linux.tar.gz
tar -xzf bpi-os-linux.tar.gz && ./install.sh

# Connect to BPCI via Cloudflare
bpi init --connect https://connect.pravyom.com
bpi start`}
                  </pre>
                </div>
                <Paragraph style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>
                  The installer automatically connects to BPCI infrastructure via advanced Cloudflare configuration with millions-scale onboarding support.
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

          <TabPane tab="💾 Downloads" key="downloads">
            <div style={{ maxWidth: '1000px', margin: '0 auto' }}>
              {/* BPI OS Downloads Header */}
              <Card style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(232, 180, 79, 0.3)', borderRadius: '12px', marginBottom: '2rem', backdropFilter: 'blur(10px)', textAlign: 'center' }}>
                <Title level={2} style={{ color: '#E8B44F', marginBottom: '1rem' }}>
                  <DownloadOutlined /> BPI OS Installation
                </Title>
                <Paragraph style={{ color: '#ffffff', fontSize: '1.125rem', maxWidth: '700px', margin: '0 auto 1.5rem' }}>
                  Install BPI OS using standard Linux package management or download the binary directly. Professional APT repository available for easy installation and updates.
                </Paragraph>
                <Alert
                  message="Professional APT Repository Available"
                  description="Install BPI OS using 'sudo apt install bpi-os' for automatic updates and system integration."
                  type="success"
                  showIcon
                  style={{ background: 'rgba(16, 185, 129, 0.1)', border: '1px solid rgba(16, 185, 129, 0.3)', marginBottom: '1rem' }}
                />
              </Card>

              {/* APT Installation Method (Recommended) */}
              <Card style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(16, 185, 129, 0.3)', borderRadius: '12px', marginBottom: '2rem', backdropFilter: 'blur(10px)' }}>
                <Title level={3} style={{ color: '#10B981', marginBottom: '1rem' }}>
                  <CheckCircleOutlined /> Method 1: APT Installation (Recommended)
                </Title>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', marginBottom: '1.5rem' }}>
                  Install BPI OS using the official PRAVYOM APT repository for automatic updates and system integration:
                </Paragraph>
                
                <div style={{ background: 'rgba(0, 0, 0, 0.3)', padding: '1.5rem', borderRadius: '8px', marginBottom: '1.5rem' }}>
                  <Text style={{ color: '#E8B44F', fontSize: '0.875rem', fontWeight: '600', display: 'block', marginBottom: '1rem' }}>
                    Step 1: Add PRAVYOM APT Repository
                  </Text>
                  <code style={{ 
                    background: 'rgba(232, 180, 79, 0.1)', 
                    color: '#E8B44F', 
                    padding: '0.5rem 1rem', 
                    borderRadius: '4px', 
                    display: 'block', 
                    fontFamily: 'monospace',
                    fontSize: '0.875rem',
                    marginBottom: '1rem',
                    wordBreak: 'break-all'
                  }}>
                    echo "deb [trusted=yes] https://pravyom.com/apt stable main" | sudo tee /etc/apt/sources.list.d/pravyom.list
                  </code>
                  
                  <Text style={{ color: '#E8B44F', fontSize: '0.875rem', fontWeight: '600', display: 'block', marginBottom: '1rem' }}>
                    Step 2: Update Package List
                  </Text>
                  <code style={{ 
                    background: 'rgba(232, 180, 79, 0.1)', 
                    color: '#E8B44F', 
                    padding: '0.5rem 1rem', 
                    borderRadius: '4px', 
                    display: 'block', 
                    fontFamily: 'monospace',
                    fontSize: '0.875rem',
                    marginBottom: '1rem'
                  }}>
                    sudo apt update
                  </code>
                  
                  <Text style={{ color: '#E8B44F', fontSize: '0.875rem', fontWeight: '600', display: 'block', marginBottom: '1rem' }}>
                    Step 3: Install BPI OS
                  </Text>
                  <code style={{ 
                    background: 'rgba(232, 180, 79, 0.1)', 
                    color: '#E8B44F', 
                    padding: '0.5rem 1rem', 
                    borderRadius: '4px', 
                    display: 'block', 
                    fontFamily: 'monospace',
                    fontSize: '0.875rem'
                  }}>
                    sudo apt install bpi-os
                  </code>
                </div>

                <Alert
                  message="Automatic Service Setup"
                  description="APT installation automatically configures systemd service, creates user accounts, and sets up proper permissions."
                  type="info"
                  showIcon
                  style={{ background: 'rgba(59, 130, 246, 0.1)', border: '1px solid rgba(59, 130, 246, 0.3)' }}
                />
              </Card>

              {/* Direct BPI OS Download Button */}
              <Card style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(232, 180, 79, 0.3)', borderRadius: '12px', marginBottom: '2rem', backdropFilter: 'blur(10px)' }}>
                <Title level={3} style={{ color: '#E8B44F', marginBottom: '1rem' }}>
                  <DownloadOutlined /> Method 2: Direct Binary Download
                </Title>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', marginBottom: '1.5rem' }}>
                  Download the BPI OS binary directly for manual installation:
                </Paragraph>
                <Row gutter={[24, 24]} align="middle">
                  <Col xs={24} md={4} style={{ textAlign: 'center' }}>
                    <CodeOutlined style={{ fontSize: '4rem', color: '#E8B44F' }} />
                  </Col>
                  <Col xs={24} md={14}>
                    <Title level={4} style={{ color: '#E8B44F', marginBottom: '0.5rem' }}>BPI OS Core for Linux x64</Title>
                    <Paragraph style={{ color: '#ffffff', marginBottom: '0.5rem' }}>
                      Production-ready blockchain operating system binary
                    </Paragraph>
                    <Text style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>
                      Version: 1.0.0-production | Size: 29.2 MB | MD5: 5c00fd2667ec65d056db771d82b626f1
                    </Text>
                  </Col>
                  <Col xs={24} md={6} style={{ textAlign: 'center' }}>
                    <Button
                      type="primary"
                      size="large"
                      icon={<DownloadOutlined />}
                      href="https://pravyom.com/downloads/bpi-os/bpi-core-linux-x64"
                      target="_blank"
                      download="bpi-core-linux-x64"
                      style={{
                        background: 'linear-gradient(135deg, #10B981 0%, #059669 100%)',
                        border: 'none',
                        color: '#FFFFFF',
                        fontWeight: '600',
                        width: '100%',
                        height: '50px',
                        fontSize: '16px'
                      }}
                    >
                      Download Now
                    </Button>
                  </Col>
                </Row>
              </Card>

              {/* BPI OS Downloader Component (Fallback) */}
              <BpiOSDownloader 
                showStats={false}
                showInstructions={false}
                compactMode={true}
              />

              {/* BPI OS to BPCI Connection Instructions */}
              <Card style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(139, 92, 246, 0.3)', borderRadius: '12px', marginTop: '2rem', backdropFilter: 'blur(10px)' }}>
                <Title level={3} style={{ color: '#8B5CF6', marginBottom: '1rem' }}>
                  <ApiOutlined /> Method 3: Connect to BPCI Network
                </Title>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', marginBottom: '1.5rem' }}>
                  After installing BPI OS, connect to the PRAVYOM BPCI infrastructure for full blockchain functionality:
                </Paragraph>

                <div style={{ background: 'rgba(0, 0, 0, 0.3)', padding: '1.5rem', borderRadius: '8px', marginBottom: '1.5rem' }}>
                  <Text style={{ color: '#8B5CF6', fontSize: '0.875rem', fontWeight: '600', display: 'block', marginBottom: '1rem' }}>
                    Step 1: Install BPI OS (Docker-like)
                  </Text>
                  <code style={{ 
                    background: 'rgba(139, 92, 246, 0.1)', 
                    color: '#8B5CF6', 
                    padding: '0.5rem 1rem', 
                    borderRadius: '4px', 
                    display: 'block', 
                    fontFamily: 'monospace',
                    fontSize: '0.875rem',
                    marginBottom: '1rem'
                  }}>
                    curl -fsSL https://get.bpi.dev | sh
                  </code>
                  
                  <Text style={{ color: '#8B5CF6', fontSize: '0.875rem', fontWeight: '600', display: 'block', marginBottom: '1rem' }}>
                    Step 2: Connect to BPCI via Cloudflare
                  </Text>
                  <code style={{ 
                    background: 'rgba(139, 92, 246, 0.1)', 
                    color: '#8B5CF6', 
                    padding: '0.5rem 1rem', 
                    borderRadius: '4px', 
                    display: 'block', 
                    fontFamily: 'monospace',
                    fontSize: '0.875rem',
                    marginBottom: '1rem'
                  }}>
                    bpi init --connect https://connect.pravyom.com
                  </code>
                  
                  <Text style={{ color: '#8B5CF6', fontSize: '0.875rem', fontWeight: '600', display: 'block', marginBottom: '1rem' }}>
                    Step 3: Start BPI OS (Auto-connects to BPCI)
                  </Text>
                  <code style={{ 
                    background: 'rgba(139, 92, 246, 0.1)', 
                    color: '#8B5CF6', 
                    padding: '0.5rem 1rem', 
                    borderRadius: '4px', 
                    display: 'block', 
                    fontFamily: 'monospace',
                    fontSize: '0.875rem',
                    marginBottom: '1rem'
                  }}>
                    bpi start
                  </code>

                  <Text style={{ color: '#8B5CF6', fontSize: '0.875rem', fontWeight: '600', display: 'block', marginBottom: '1rem' }}>
                    Step 4: Start BPI OS Service
                  </Text>
                  <code style={{ 
                    background: 'rgba(139, 92, 246, 0.1)', 
                    color: '#8B5CF6', 
                    padding: '0.5rem 1rem', 
                    borderRadius: '4px', 
                    display: 'block', 
                    fontFamily: 'monospace',
                    fontSize: '0.875rem'
                  }}>
                    sudo systemctl start bpi-os && sudo systemctl enable bpi-os
                  </code>
                </div>

                <Alert
                  message="Network Configuration"
                  description="BPI OS automatically configures LCCD/QCE2 consensus, XTMP protocol, DynaRoutes service mesh, and 6D blockchain architecture when connecting to PRAVYOM infrastructure."
                  type="info"
                  showIcon
                  style={{ background: 'rgba(139, 92, 246, 0.1)', border: '1px solid rgba(139, 92, 246, 0.3)', marginBottom: '1rem' }}
                />

                <div style={{ background: 'rgba(16, 185, 129, 0.1)', padding: '1rem', borderRadius: '8px', border: '1px solid rgba(16, 185, 129, 0.3)' }}>
                  <Text style={{ color: '#10B981', fontSize: '0.875rem', fontWeight: '600', display: 'block', marginBottom: '0.5rem' }}>
                    ✅ Connection Features Enabled:
                  </Text>
                  <ul style={{ color: '#ffffff', fontSize: '0.875rem', paddingLeft: '1rem', margin: 0 }}>
                    <li>6D Multi-Dimensional Blockchain Architecture</li>
                    <li>XTMP Protocol for Auction-Based Transactions</li>
                    <li>Complex Addressing: your-wallet@pravyom.bpi</li>
                    <li>DynaRoutes Service Mesh Communication</li>
                    <li>Quantum-Resistant Security (ZipLock Encryption)</li>
                    <li>Web2-Like Performance in Web3.5 Environment</li>
                  </ul>
                </div>
              </Card>

              {/* Quick Start Instructions */}
              <Card style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(59, 130, 246, 0.3)', borderRadius: '12px', marginTop: '2rem', backdropFilter: 'blur(10px)' }}>
                <Title level={4} style={{ color: '#3B82F6', marginBottom: '1rem' }}>
                  <CodeOutlined /> Verification & Status Check
                </Title>
                <div style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8' }}>
                  <Paragraph style={{ color: '#ffffff', marginBottom: '1rem' }}>
                    Verify your BPI OS installation and network connection:
                  </Paragraph>
                  <ol style={{ paddingLeft: '1.5rem', color: '#ffffff' }}>
                    <li style={{ marginBottom: '0.5rem' }}>
                      <strong style={{ color: '#E8B44F' }}>Check Service Status:</strong> <code style={{ background: 'rgba(232, 180, 79, 0.2)', padding: '2px 6px', borderRadius: '4px' }}>sudo systemctl status bpi-os</code>
                    </li>
                    <li style={{ marginBottom: '0.5rem' }}>
                      <strong style={{ color: '#E8B44F' }}>View Connection Logs:</strong> <code style={{ background: 'rgba(232, 180, 79, 0.2)', padding: '2px 6px', borderRadius: '4px' }}>sudo journalctl -u bpi-os -f</code>
                    </li>
                    <li style={{ marginBottom: '0.5rem' }}>
                      <strong style={{ color: '#E8B44F' }}>Test Network Connection:</strong> <code style={{ background: 'rgba(232, 180, 79, 0.2)', padding: '2px 6px', borderRadius: '4px' }}>bpi-os status --network</code>
                    </li>
                    <li style={{ marginBottom: '0.5rem' }}>
                      <strong style={{ color: '#E8B44F' }}>Access Web Interface:</strong> <code style={{ background: 'rgba(232, 180, 79, 0.2)', padding: '2px 6px', borderRadius: '4px' }}>http://localhost:8080</code>
                    </li>
                  </ol>
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
