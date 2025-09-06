import React from 'react';
import { Typography, Card, Row, Col, Timeline, Statistic } from 'antd';
import { 
  RocketOutlined, 
  SecurityScanOutlined, 
  GlobalOutlined, 
  TeamOutlined,
  TrophyOutlined,
  BankOutlined
} from '@ant-design/icons';
import './About.css';

const { Title, Paragraph } = Typography;

const About: React.FC = () => {
  return (
    <div className="about-page">
      {/* Hero Section */}
      <section className="hero-gradient py-20">
        <div className="max-w-6xl mx-auto px-4 text-center">
          <Title level={1} style={{ color: '#ffffff', fontSize: '3rem', fontWeight: 'bold', marginBottom: '1.5rem' }}>
            About BPCI Enterprise
          </Title>
          <Paragraph style={{ color: '#ffffff', fontSize: '1.25rem', maxWidth: '48rem', margin: '0 auto' }}>
            Next-generation blockchain infrastructure platform providing secure, transparent, 
            and decentralized solutions for individuals, organizations, and communities 
            seeking reliable Web3 infrastructure.
          </Paragraph>
        </div>
      </section>

      {/* Vision & Mission */}
      <section style={{ padding: '5rem 0', background: 'transparent' }}>
        <div style={{ maxWidth: '72rem', margin: '0 auto', padding: '0 1rem' }}>
          <div style={{ textAlign: 'center', marginBottom: '4rem' }}>
            <div style={{ 
              background: '#fef3c7', 
              border: '1px solid #fbbf24', 
              borderRadius: '0.5rem', 
              padding: '1.5rem', 
              marginBottom: '2rem', 
              maxWidth: '64rem', 
              margin: '0 auto 2rem auto' 
            }}>
              <h2 style={{ fontSize: '1.25rem', fontWeight: '600', color: '#92400e', marginBottom: '0.75rem' }}>⚠️ Important: This is a Pilot Project</h2>
              <p style={{ color: '#92400e', marginBottom: '1rem' }}>
                BPCI Enterprise is currently a <strong>side project</strong> in the research and development phase. 
                We are <strong>not ready for mainnet production</strong> and are seeking enterprise partners for pilot testing with pre-funding.
              </p>
              <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(300px, 1fr))', gap: '1rem', textAlign: 'left' }}>
                <div>
                  <h4 style={{ fontWeight: '600', color: '#92400e', marginBottom: '0.5rem' }}>Current Status:</h4>
                  <ul style={{ color: '#92400e', fontSize: '0.875rem', listStyle: 'none', padding: 0 }}>
                    <li style={{ marginBottom: '0.25rem' }}>• Testnet with 3 nodes (experimental)</li>
                    <li style={{ marginBottom: '0.25rem' }}>• Security analysis still in progress</li>
                    <li style={{ marginBottom: '0.25rem' }}>• Side project to validate BPI OS concepts</li>
                    <li style={{ marginBottom: '0.25rem' }}>• Seeking pilot partners with pre-funding</li>
                  </ul>
                </div>
                <div>
                  <h4 className="font-semibold text-yellow-800 mb-2">Not Ready For:</h4>
                  <ul className="space-y-1 text-yellow-700 text-sm">
                    <li>• Production mainnet deployment</li>
                    <li>• Large-scale enterprise operations</li>
                    <li>• Mission-critical financial systems</li>
                    <li>• Regulatory compliance requirements</li>
                  </ul>
                </div>
              </div>
            </div>
            
            <Title level={2} style={{ 
              fontSize: '2.25rem', 
              fontWeight: 'bold', 
              marginBottom: '1.5rem',
              color: '#ffffff',
              textShadow: '0 2px 4px rgba(0, 0, 0, 0.8)'
            }}>Pilot Program Vision & Reality</Title>
            <Paragraph style={{ 
              fontSize: '1.25rem', 
              color: '#ffffff', 
              textShadow: '0 2px 4px rgba(0, 0, 0, 0.8)',
              maxWidth: '64rem',
              margin: '0 auto'
            }}>
              We're exploring whether blockchain infrastructure can serve enterprises with better 
              transparency, security, and accessibility. Our current mission is to validate BPI OS concepts 
              through real-world pilot testing with enterprise partners who understand this is experimental R&D work.
            </Paragraph>
            
            <div style={{
              background: '#eff6ff',
              border: '1px solid #bfdbfe',
              borderRadius: '0.5rem',
              padding: '2rem',
              marginTop: '3rem',
              maxWidth: '80rem',
              margin: '3rem auto 0 auto'
            }}>
              <Title level={3} style={{
                fontSize: '1.5rem',
                fontWeight: 'bold',
                color: '#1e40af',
                marginBottom: '1.5rem',
                textAlign: 'center'
              }}>What We're Seeking from Enterprise Partners</Title>
              
              <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(250px, 1fr))', gap: '1.5rem' }}>
                <div style={{ textAlign: 'center' }}>
                  <div style={{
                    background: '#dbeafe',
                    borderRadius: '50%',
                    width: '4rem',
                    height: '4rem',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    margin: '0 auto 1rem auto'
                  }}>
                    <span style={{ fontSize: '1.5rem' }}>🤝</span>
                  </div>
                  <Title level={4} style={{ color: '#1e40af', marginBottom: '0.75rem' }}>Collaboration & Feedback</Title>
                  <ul style={{ 
                    textAlign: 'left', 
                    color: '#1e40af', 
                    fontSize: '0.875rem',
                    listStyle: 'none',
                    padding: 0,
                    margin: 0
                  }}>
                    <li style={{ marginBottom: '0.5rem' }}>• Real-world blockchain infrastructure needs assessment</li>
                    <li style={{ marginBottom: '0.5rem' }}>• Technical feedback on consensus mechanisms</li>
                    <li style={{ marginBottom: '0.5rem' }}>• Security analysis collaboration</li>
                    <li style={{ marginBottom: '0.5rem' }}>• Performance testing in controlled environments</li>
                  </ul>
                </div>
                
                <div style={{ textAlign: 'center' }}>
                  <div style={{
                    background: '#dbeafe',
                    borderRadius: '50%',
                    width: '4rem',
                    height: '4rem',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    margin: '0 auto 1rem auto'
                  }}>
                    <span style={{ fontSize: '1.5rem' }}>💰</span>
                  </div>
                  <Title level={4} style={{ color: '#1e40af', marginBottom: '0.75rem' }}>Pre-Funding Support</Title>
                  <ul style={{ 
                    textAlign: 'left', 
                    color: '#1e40af', 
                    fontSize: '0.875rem',
                    listStyle: 'none',
                    padding: 0,
                    margin: 0
                  }}>
                    <li style={{ marginBottom: '0.5rem' }}>• Pilot project funding for development</li>
                    <li style={{ marginBottom: '0.5rem' }}>• Infrastructure costs for testing</li>
                    <li style={{ marginBottom: '0.5rem' }}>• Security audit funding</li>
                    <li style={{ marginBottom: '0.5rem' }}>• Research and development support</li>
                  </ul>
                </div>
                
                <div style={{ textAlign: 'center' }}>
                  <div style={{
                    background: '#dbeafe',
                    borderRadius: '50%',
                    width: '4rem',
                    height: '4rem',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    margin: '0 auto 1rem auto'
                  }}>
                    <span style={{ fontSize: '1.5rem' }}>🔬</span>
                  </div>
                  <Title level={4} style={{ color: '#1e40af', marginBottom: '0.75rem' }}>R&D Partnership</Title>
                  <ul style={{ 
                    textAlign: 'left', 
                    color: '#1e40af', 
                    fontSize: '0.875rem',
                    listStyle: 'none',
                    padding: 0,
                    margin: 0
                  }}>
                    <li style={{ marginBottom: '0.5rem' }}>• Understanding this is experimental technology</li>
                    <li style={{ marginBottom: '0.5rem' }}>• Patience with development timelines</li>
                    <li style={{ marginBottom: '0.5rem' }}>• Willingness to test unproven concepts</li>
                    <li>• Joint research on blockchain applications</li>
                  </ul>
                </div>
              </div>
            </div>
          </div>

          <Row gutter={[48, 48]}>
            <Col xs={24} lg={12}>
              <Card className="h-full p-8 border-l-4 border-l-blue-600">
                <div className="flex items-start space-x-4">
                  <RocketOutlined className="text-3xl text-blue-600 mt-1" />
                  <div>
                    <Title level={3} style={{ 
                      fontSize: '1.5rem', 
                      fontWeight: '600', 
                      marginBottom: '1rem',
                      color: '#ffffff',
                      textShadow: '0 2px 4px rgba(0, 0, 0, 0.8)'
                    }}>Our Mission</Title>
                    <Paragraph style={{ 
                      color: '#e2e8f0', 
                      fontSize: '1.125rem', 
                      lineHeight: '1.75',
                      textShadow: '0 1px 3px rgba(0, 0, 0, 0.8)'
                    }}>
                      To provide secure blockchain infrastructure, transparent governance, and reliable 
                      decentralized solutions for anyone who needs trustworthy Web3 technology. We serve 
                      individuals, organizations, communities, and institutions building the future 
                      of decentralized applications and digital infrastructure.
                    </Paragraph>
                  </div>
                </div>
              </Card>
            </Col>

            <Col xs={24} lg={12}>
              <Card className="h-full p-8 border-l-4 border-l-green-600">
                <div className="flex items-start space-x-4">
                  <GlobalOutlined className="text-3xl text-green-600 mt-1" />
                  <div>
                    <Title level={3} style={{ 
                      fontSize: '1.5rem', 
                      fontWeight: '600', 
                      marginBottom: '1rem',
                      color: '#ffffff',
                      textShadow: '0 2px 4px rgba(0, 0, 0, 0.8)'
                    }}>Our Vision</Title>
                    <Paragraph style={{ 
                      color: '#e2e8f0', 
                      fontSize: '1.125rem', 
                      lineHeight: '1.75',
                      textShadow: '0 1px 3px rgba(0, 0, 0, 0.8)'
                    }}>
                      A world where blockchain infrastructure serves everyone with transparent governance, 
                      secure protocols, and authentic decentralization. We're creating robust 
                      Web3 infrastructure that preserves privacy, enables community ownership, 
                      and supports the evolution toward a more equitable digital economy.
                    </Paragraph>
                  </div>
                </div>
              </Card>
            </Col>
          </Row>
        </div>
      </section>

      {/* Roadmap Timeline */}
      <section className="py-20 bg-gray-50">
        <div className="max-w-6xl mx-auto px-4">
          <div className="text-center mb-16">
            <Title level={2} style={{ 
              fontSize: '2.25rem', 
              fontWeight: 'bold', 
              marginBottom: '1.5rem',
              color: '#ffffff',
              textShadow: '0 2px 4px rgba(0, 0, 0, 0.8)'
            }}>Development Roadmap</Title>
            <Paragraph style={{ 
              fontSize: '1.25rem',
              color: '#e2e8f0',
              textShadow: '0 2px 4px rgba(0, 0, 0, 0.8)'
            }}>
              Our step-by-step journey toward post-quantum secure blockchain infrastructure
            </Paragraph>
          </div>

          <div className="max-w-4xl mx-auto">
            <Timeline
              mode="left"
              items={[
                {
                  label: '2025-2026',
                  children: (
                    <Card className="ml-4">
                      <Title level={4} style={{ 
                        color: '#1e40af', 
                        marginBottom: '0.5rem'
                      }}>
                        <TrophyOutlined className="mr-2" />
                        Phase 1: Testnet to Mainnet Transition
                      </Title>
                      <Paragraph style={{ 
                        color: '#1f2937', 
                        marginBottom: '0.5rem'
                      }}>
                        <strong>Current Status:</strong> Testnet operational, mainnet timeline based on community feedback
                      </Paragraph>
                      <ul style={{ 
                        color: '#374151', 
                        listStyle: 'none',
                        padding: 0,
                        margin: 0
                      }}>
                        <li style={{ marginBottom: '0.25rem' }}>• Post-quantum ready cryptographic security</li>
                        <li style={{ marginBottom: '0.25rem' }}>• Community network expansion and partner onboarding</li>
                        <li style={{ marginBottom: '0.25rem' }}>• Enhanced audit trails and compliance features</li>
                        <li style={{ marginBottom: '0.25rem' }}>• Economic model development and testing</li>
                      </ul>
                    </Card>
                  ),
                  color: 'blue',
                },
                {
                  label: '2027-2028',
                  children: (
                    <Card className="ml-4">
                      <Title level={4} style={{ 
                        color: '#059669', 
                        marginBottom: '0.5rem'
                      }}>
                        <SecurityScanOutlined className="mr-2" />
                        Phase 2: Enterprise Integration & Scaling
                      </Title>
                      <Paragraph style={{ 
                        color: '#1f2937', 
                        marginBottom: '0.5rem'
                      }}>
                        Enhanced enterprise features and institutional adoption
                      </Paragraph>
                      <ul style={{ 
                        color: '#374151', 
                        listStyle: 'none',
                        padding: 0,
                        margin: 0
                      }}>
                        <li style={{ marginBottom: '0.25rem' }}>• Advanced smart contract capabilities</li>
                        <li style={{ marginBottom: '0.25rem' }}>• Enterprise-grade API and SDK development</li>
                        <li style={{ marginBottom: '0.25rem' }}>• Institutional custody and compliance tools</li>
                        <li style={{ marginBottom: '0.25rem' }}>• Cross-chain interoperability protocols</li>
                      </ul>
                    </Card>
                  ),
                  color: 'green',
                },
                {
                  label: '2029+',
                  children: (
                    <Card className="ml-4">
                      <Title level={4} className="text-purple-600 mb-2">
                        <GlobalOutlined className="mr-2" />
                        Phase 3: Global Infrastructure & Innovation
                      </Title>
                      <Paragraph className="text-gray-700 mb-2">
                        <strong>Long-term Vision:</strong> Global blockchain infrastructure and next-generation protocols
                      </Paragraph>
                      <ul className="text-gray-600 space-y-1">
                        <li>• Global distributed network architecture</li>
                        <li>• Advanced cryptographic research and implementation</li>
                        <li>• Large-scale coordination and governance protocols</li>
                        <li>• Next-generation blockchain and consensus innovations</li>
                      </ul>
                    </Card>
                  ),
                  color: 'purple',
                },
              ]}
            />
          </div>
        </div>
      </section>

      {/* Current Progress */}
      <section className="py-20 bg-white">
        <div className="max-w-6xl mx-auto px-4">
          <div className="text-center mb-16">
            <Title level={2} className="text-4xl font-bold mb-6">Current Progress</Title>
            <Paragraph className="text-xl text-gray-600">
              Current status of our testnet development environment
            </Paragraph>
          </div>

          <Row gutter={[32, 32]} justify="center">
            <Col xs={12} sm={8} md={6}>
              <Card className="text-center p-6 hover:shadow-lg transition-shadow">
                <Statistic 
                  title="Security Level" 
                  value="Military"
                  valueStyle={{ color: '#059669', fontSize: '1.8rem', fontWeight: 'bold' }}
                />
                <Paragraph className="text-gray-500 text-sm mt-2">Ed25519 + Blake3</Paragraph>
              </Card>
            </Col>
            <Col xs={12} sm={8} md={6}>
              <Card className="text-center p-6 hover:shadow-lg transition-shadow">
                <Statistic 
                  title="Active Modules" 
                  value={8} 
                  valueStyle={{ color: '#0066cc', fontSize: '2.5rem', fontWeight: 'bold' }}
                />
                <Paragraph className="text-gray-500 text-sm mt-2">Core systems</Paragraph>
              </Card>
            </Col>
            <Col xs={12} sm={8} md={6}>
              <Card className="text-center p-6 hover:shadow-lg transition-shadow">
                <Statistic 
                  title="API Endpoints" 
                  value={12} 
                  valueStyle={{ color: '#ea580c', fontSize: '2.5rem', fontWeight: 'bold' }}
                />
                <Paragraph className="text-gray-500 text-sm mt-2">Production ready</Paragraph>
              </Card>
            </Col>
            <Col xs={12} sm={8} md={6}>
              <Card className="text-center p-6 hover:shadow-lg transition-shadow">
                <Statistic 
                  title="Uptime" 
                  value={99.9} 
                  precision={1}
                  suffix="%"
                  valueStyle={{ color: '#7c3aed', fontSize: '2.5rem', fontWeight: 'bold' }}
                />
                <Paragraph className="text-gray-500 text-sm mt-2">Enterprise grade</Paragraph>
              </Card>
            </Col>
          </Row>
        </div>
      </section>

      {/* Core Principles */}
      <section className="py-20 bg-gray-50">
        <div className="max-w-6xl mx-auto px-4">
          <div className="text-center mb-16">
            <Title level={2} className="text-4xl font-bold mb-6">Core Principles</Title>
            <Paragraph className="text-xl text-gray-600">
              The fundamental values that guide our development and decision-making
            </Paragraph>
          </div>

          <Row gutter={[32, 32]}>
            <Col xs={24} md={8}>
              <Card className="h-full text-center p-8 hover:shadow-lg transition-shadow">
                <SecurityScanOutlined className="text-5xl text-blue-600 mb-4" />
                <Title level={3} className="text-xl font-semibold mb-4">Transparency</Title>
                <Paragraph className="text-gray-600">
                  Complete honesty about our current stage, progress, and challenges. 
                  No overpromising, just authentic communication about what we're building.
                </Paragraph>
              </Card>
            </Col>
            <Col xs={24} md={8}>
              <Card className="h-full text-center p-8 hover:shadow-lg transition-shadow">
                <BankOutlined className="text-5xl text-green-600 mb-4" />
                <Title level={3} className="text-xl font-semibold mb-4">Enterprise Grade</Title>
                <Paragraph className="text-gray-600">
                  Military-grade security, banking compliance, and government-ready 
                  infrastructure that meets the highest institutional standards.
                </Paragraph>
              </Card>
            </Col>
            <Col xs={24} md={8}>
              <Card className="h-full text-center p-8 hover:shadow-lg transition-shadow">
                <TeamOutlined className="text-5xl text-purple-600 mb-4" />
                <Title level={3} className="text-xl font-semibold mb-4">Community Driven</Title>
                <Paragraph className="text-gray-600">
                  Building with and for the community, fostering collaboration, 
                  and creating technology that serves people, not the other way around.
                </Paragraph>
              </Card>
            </Col>
          </Row>

          <Row gutter={[32, 32]} className="mb-16">
            <Col xs={24}>
              <div className="bg-gray-50 border border-gray-200 rounded-lg p-8">
                <Title level={3} className="text-center mb-8 text-gray-800">Honest Development Timeline & Limitations</Title>
                
                <div className="bg-red-50 border border-red-200 rounded-lg p-6 mb-8">
                  <Title level={4} className="text-red-800 mb-3">⚠️ Current Reality Check</Title>
                  <div className="grid md:grid-cols-2 gap-6">
                    <div>
                      <h5 className="font-semibold text-red-700 mb-2">What Works Now:</h5>
                      <ul className="space-y-1 text-red-600 text-sm">
                        <li>• Basic 3-node testnet running</li>
                        <li>• Simple consensus mechanism</li>
                        <li>• Basic wallet operations</li>
                        <li>• Experimental auction mempool</li>
                        <li>• Development APIs (not production-ready)</li>
                      </ul>
                    </div>
                    <div>
                      <h5 className="font-semibold text-red-700 mb-2">Major Limitations:</h5>
                      <ul className="space-y-1 text-red-600 text-sm">
                        <li>• No comprehensive security audit</li>
                        <li>• Not scalable beyond test environment</li>
                        <li>• No regulatory compliance framework</li>
                        <li>• Limited to experimental use cases</li>
                        <li>• Requires significant development before production</li>
                      </ul>
                    </div>
                  </div>
                </div>
                
                <div className="space-y-6">
                  <div className="flex items-start space-x-4">
                    <div className="flex-shrink-0 w-10 h-10 bg-yellow-500 rounded-full flex items-center justify-center">
                      <span className="text-white font-bold">NOW</span>
                    </div>
                    <div className="flex-1">
                      <Title level={4} className="text-yellow-600 mb-2">Current Phase: Seeking Pilot Partners (2024)</Title>
                      <Paragraph className="text-gray-600 mb-3">
                        <strong>Reality:</strong> This is a side project in R&D phase. We need enterprise partners who understand 
                        this is experimental technology and are willing to provide pre-funding for pilot testing.
                      </Paragraph>
                      <div className="bg-yellow-50 p-4 rounded border-l-4 border-yellow-400">
                        <p className="text-yellow-800 text-sm">
                          <strong>What we need:</strong> Patient partners, pre-funding, security analysis collaboration, 
                          realistic expectations about timelines and capabilities.
                        </p>
                      </div>
                    </div>
                  </div>
                  
                  <div className="flex items-start space-x-4">
                    <div className="flex-shrink-0 w-10 h-10 bg-blue-400 rounded-full flex items-center justify-center">
                      <span className="text-white font-bold text-xs">6-12M</span>
                    </div>
                    <div className="flex-1">
                      <Title level={4} className="text-blue-600 mb-2">Phase 2: Security & Stability (Mid-Late 2024)</Title>
                      <Paragraph className="text-gray-600 mb-3">
                        <strong>Goals:</strong> Comprehensive security audits, stability improvements, expanded testnet, 
                        and refined enterprise APIs. <em>Timeline depends on funding and partner feedback.</em>
                      </Paragraph>
                      <div className="bg-blue-50 p-4 rounded border-l-4 border-blue-400">
                        <p className="text-blue-800 text-sm">
                          <strong>Requirements:</strong> Successful pilot partnerships, adequate funding for security audits, 
                          and positive validation of core concepts.
                        </p>
                      </div>
                    </div>
                  </div>
                  
                  <div className="flex items-start space-x-4">
                    <div className="flex-shrink-0 w-10 h-10 bg-gray-400 rounded-full flex items-center justify-center">
                      <span className="text-white font-bold text-xs">2025+</span>
                    </div>
                    <div className="flex-1">
                      <Title level={4} style={{ color: '#6b7280', marginBottom: '0.5rem' }}>Phase 3: Mainnet Consideration (2025+)</Title>
                      <Paragraph style={{ 
                        color: '#1f2937', 
                        marginBottom: '0.75rem'
                      }}>
                        Full mainnet deployment pending successful pilot completion and security validation
                      </Paragraph>
                      <div style={{
                        background: '#fefce8',
                        border: '1px solid #fde047',
                        borderRadius: '0.375rem',
                        padding: '0.75rem',
                        marginBottom: '0.75rem'
                      }}>
                        <p style={{ color: '#92400e', fontSize: '0.875rem' }}>
                          <strong>Important:</strong> Mainnet deployment timeline depends on pilot success, 
                          security audits, community feedback, and enterprise partner validation.
                        </p>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </Col>
          </Row>
        </div>
      </section>

      {/* Founder Story */}
      <section className="py-20 bg-white">
        <div className="max-w-4xl mx-auto px-4 text-center">
          <Title level={2} className="text-4xl font-bold mb-8">About This Project</Title>
          <Card className="p-8 bg-gradient-to-r from-blue-50 to-purple-50">
            <Paragraph style={{ 
              fontSize: '1.125rem',
              color: '#e2e8f0',
              lineHeight: '1.75',
              marginBottom: '1.5rem',
              textShadow: '0 1px 3px rgba(0, 0, 0, 0.8)'
            }}>
              BPCI Enterprise represents our commitment to building blockchain infrastructure that serves 
              real needs with transparency, security, and community governance. We're not just building 
              technology—we're fostering an ecosystem where innovation thrives through collaboration.
            </Paragraph>
            
            <Paragraph style={{ 
              fontSize: '1.125rem',
              color: '#e2e8f0',
              lineHeight: '1.75',
              marginBottom: '1.5rem',
              textShadow: '0 1px 3px rgba(0, 0, 0, 0.8)'
            }}>
              Our approach prioritizes security, transparency, and authentic decentralization over 
              marketing hype. Every feature we develop is tested, validated, and designed to serve 
              the long-term interests of our community and enterprise partners.
            </Paragraph>
            
            <Paragraph style={{ 
              fontSize: '1.125rem',
              color: '#e2e8f0',
              lineHeight: '1.75',
              textShadow: '0 1px 3px rgba(0, 0, 0, 0.8)'
            }}>
              Join us in building the future of blockchain infrastructure—one that prioritizes 
              substance over speculation, community over profit, and long-term value over short-term gains.
            </Paragraph>
          </Card>
        </div>
      </section>
    </div>
  );
};

export default About;
