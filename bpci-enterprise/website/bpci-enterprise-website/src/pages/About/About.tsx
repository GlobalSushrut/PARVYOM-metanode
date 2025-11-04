import React from 'react';
import { Typography, Card, Row, Col, Timeline, Statistic, Button } from 'antd';
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
            About Pravyom Research Platform
          </Title>
          <Paragraph style={{ color: '#ffffff', fontSize: '1.25rem', maxWidth: '48rem', margin: '0 auto' }}>
            Experimental research platform exploring distributed operating systems and blockchain infrastructure. 
            <strong>75% infrastructure ready</strong> with 15 services operational. 
            Needs testing, pilot partnerships, and 6-12 months to pilot-ready status.
          </Paragraph>
        </div>
      </section>

      {/* Vision & Mission */}
      <section style={{ padding: '5rem 0', background: 'transparent' }}>
        <div style={{ maxWidth: '72rem', margin: '0 auto', padding: '0 1rem' }}>
          <div style={{ textAlign: 'center', marginBottom: '4rem' }}>
            <div style={{ 
              background: 'rgba(10, 22, 40, 0.9)', /* Navy background */
              border: '2px solid rgba(232, 180, 79, 0.5)', /* Gold border */
              borderRadius: '12px', 
              padding: '2rem', 
              marginBottom: '2rem', 
              maxWidth: '64rem', 
              margin: '0 auto 2rem auto',
              backdropFilter: 'blur(10px)',
              boxShadow: '0 4px 20px rgba(0, 0, 0, 0.3)'
            }}>
              <h2 style={{ fontSize: '1.5rem', fontWeight: '700', color: '#E8B44F', marginBottom: '1rem', textAlign: 'center' }}>🔬 Current Status: 75% Infrastructure Ready</h2>
              <p style={{ color: '#ffffff', marginBottom: '1.5rem', fontSize: '1rem', lineHeight: '1.6', textAlign: 'center' }}>
                Pravyom is a <strong style={{ color: '#E8B44F' }}>single-engineer research project</strong> with 75% infrastructure complete. 
                15 backend services are operational, but the system <strong style={{ color: '#E8B44F' }}>needs testing and pilot partnerships</strong> before production readiness. 
                Timeline: <strong style={{ color: '#E8B44F' }}>6-12 months to pilot-ready, 1-2 years to mainnet</strong> with proper funding and team.
              </p>
              <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(300px, 1fr))', gap: '2rem', textAlign: 'left' }}>
                <div style={{ 
                  background: 'rgba(16, 185, 129, 0.1)', /* Emerald tint */
                  padding: '1.5rem',
                  borderRadius: '8px',
                  border: '1px solid rgba(16, 185, 129, 0.3)'
                }}>
                  <h4 style={{ fontWeight: '700', color: '#10B981', marginBottom: '1rem', fontSize: '1.125rem' }}>✅ What's Operational (75%):</h4>
                  <ul style={{ color: '#ffffff', fontSize: '0.875rem', listStyle: 'none', padding: 0, lineHeight: '1.8' }}>
                    <li style={{ marginBottom: '0.5rem' }}>✅ 15 backend services deployed</li>
                    <li style={{ marginBottom: '0.5rem' }}>✅ Keycloak authentication working</li>
                    <li style={{ marginBottom: '0.5rem' }}>✅ Real APIs (not mock data)</li>
                    <li style={{ marginBottom: '0.5rem' }}>✅ PostgreSQL, Redis, MongoDB, RabbitMQ running</li>
                    <li style={{ marginBottom: '0.5rem' }}>✅ DynaRoute v2 networking operational</li>
                  </ul>
                </div>
                <div style={{ 
                  background: 'rgba(245, 158, 11, 0.1)', /* Amber tint */
                  padding: '1.5rem',
                  borderRadius: '8px',
                  border: '1px solid rgba(245, 158, 11, 0.3)'
                }}>
                  <h4 style={{ fontWeight: '700', color: '#F59E0B', marginBottom: '1rem', fontSize: '1.125rem' }}>⚠️ What Needs Work (25%):</h4>
                  <ul style={{ color: '#ffffff', fontSize: '0.875rem', listStyle: 'none', padding: 0, lineHeight: '1.8' }}>
                    <li style={{ marginBottom: '0.5rem' }}>⚠️ Testing with real users (0% - critical)</li>
                    <li style={{ marginBottom: '0.5rem' }}>⚠️ Security audits (0% - critical)</li>
                    <li style={{ marginBottom: '0.5rem' }}>⚠️ Performance optimization (0% - important)</li>
                    <li style={{ marginBottom: '0.5rem' }}>⚠️ External validation (0% - critical)</li>
                    <li style={{ marginBottom: '0.5rem' }}>⚠️ Pilot partnerships (0% - critical)</li>
                  </ul>
                </div>
              </div>
            </div>
            
            <Title level={2} style={{ 
              fontSize: '2.25rem', 
              fontWeight: 'bold', 
              marginBottom: '1.5rem',
              color: '#1f2937'
            }}>Research Vision & Current Reality</Title>
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
              background: 'rgba(10, 22, 40, 0.8)', /* Navy with transparency */
              border: '2px solid rgba(232, 180, 79, 0.3)', /* Gold border */
              borderRadius: '12px',
              padding: '2rem',
              marginTop: '3rem',
              maxWidth: '80rem',
              margin: '3rem auto 0 auto',
              backdropFilter: 'blur(10px)'
            }}>
              <Title level={3} style={{
                fontSize: '1.5rem',
                fontWeight: 'bold',
                color: '#E8B44F', /* Gold */
                marginBottom: '1.5rem',
                textAlign: 'center'
              }}>What We're Seeking from Pilot Partners</Title>
              
              <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(250px, 1fr))', gap: '1.5rem' }}>
                <div style={{ textAlign: 'center' }}>
                  <div style={{
                    background: 'rgba(232, 180, 79, 0.2)', /* Gold background */
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
                  <Title level={4} style={{ color: '#E8B44F', marginBottom: '0.75rem' }}>Collaboration & Feedback</Title>
                  <ul style={{ 
                    textAlign: 'left', 
                    color: '#ffffff', 
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
                    background: 'rgba(232, 180, 79, 0.2)', /* Gold background */
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
                  <Title level={4} style={{ color: '#E8B44F', marginBottom: '0.75rem' }}>Pre-Funding Support</Title>
                  <ul style={{ 
                    textAlign: 'left', 
                    color: '#ffffff', 
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
                    background: 'rgba(232, 180, 79, 0.2)', /* Gold background */
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
                  <Title level={4} style={{ color: '#E8B44F', marginBottom: '0.75rem' }}>R&D Partnership</Title>
                  <ul style={{ 
                    textAlign: 'left', 
                    color: '#ffffff', 
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

      {/* Realistic Development Timeline - Redesigned */}
      <section style={{ padding: '5rem 0', background: 'transparent' }}>
        <div style={{ maxWidth: '72rem', margin: '0 auto', padding: '0 2rem' }}>
          <div style={{ textAlign: 'center', marginBottom: '4rem' }}>
            <Title level={2} style={{ fontSize: '2.5rem', fontWeight: 'bold', marginBottom: '1rem', color: '#E8B44F' }}>
              Realistic Development Timeline
            </Title>
            <Paragraph style={{ fontSize: '1.25rem', color: '#ffffff', maxWidth: '48rem', margin: '0 auto' }}>
              Honest roadmap based on current 75% infrastructure status
            </Paragraph>
          </div>

          <div style={{ display: 'grid', gap: '2rem', maxWidth: '56rem', margin: '0 auto' }}>
            {/* Phase 1: NOW */}
            <div style={{
              background: 'rgba(10, 22, 40, 0.9)',
              border: '2px solid rgba(232, 180, 79, 0.5)',
              borderRadius: '12px',
              padding: '2rem',
              backdropFilter: 'blur(10px)'
            }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '1rem', marginBottom: '1rem' }}>
                <div style={{
                  background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)',
                  color: '#0A1628',
                  fontWeight: 'bold',
                  padding: '0.5rem 1rem',
                  borderRadius: '6px',
                  fontSize: '0.875rem'
                }}>
                  NOW
                </div>
                <Title level={3} style={{ color: '#E8B44F', margin: 0, fontSize: '1.5rem' }}>
                  Current: 75% Infrastructure Ready
                </Title>
              </div>
              <Paragraph style={{ color: '#ffffff', marginBottom: '1rem', lineHeight: '1.6' }}>
                <strong style={{ color: '#E8B44F' }}>Status:</strong> 15 backend services operational, needs testing & pilot partnerships
              </Paragraph>
              <ul style={{ color: '#ffffff', fontSize: '0.875rem', listStyle: 'none', padding: 0, lineHeight: '1.8' }}>
                <li style={{ marginBottom: '0.5rem' }}>✅ Keycloak, PostgreSQL, Redis, MongoDB, RabbitMQ running</li>
                <li style={{ marginBottom: '0.5rem' }}>✅ DynaRoute v2 networking operational</li>
                <li style={{ marginBottom: '0.5rem' }}>⚠️ Needs: Testing, security audits, external validation</li>
              </ul>
            </div>

            {/* Phase 2: 6-12 Months */}
            <div style={{
              background: 'rgba(10, 22, 40, 0.8)',
              border: '2px solid rgba(16, 185, 129, 0.3)',
              borderRadius: '12px',
              padding: '2rem',
              backdropFilter: 'blur(10px)'
            }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '1rem', marginBottom: '1rem' }}>
                <div style={{
                  background: 'rgba(16, 185, 129, 0.2)',
                  color: '#10B981',
                  fontWeight: 'bold',
                  padding: '0.5rem 1rem',
                  borderRadius: '6px',
                  fontSize: '0.875rem',
                  border: '1px solid rgba(16, 185, 129, 0.5)'
                }}>
                  6-12 MONTHS
                </div>
                <Title level={3} style={{ color: '#10B981', margin: 0, fontSize: '1.5rem' }}>
                  Pilot-Ready Phase
                </Title>
              </div>
              <Paragraph style={{ color: '#ffffff', marginBottom: '1rem', lineHeight: '1.6' }}>
                <strong style={{ color: '#10B981' }}>Goal:</strong> Complete testing, security audits, and pilot partnerships
              </Paragraph>
              <ul style={{ color: '#ffffff', fontSize: '0.875rem', listStyle: 'none', padding: 0, lineHeight: '1.8' }}>
                <li style={{ marginBottom: '0.5rem' }}>🔒 Comprehensive security audits</li>
                <li style={{ marginBottom: '0.5rem' }}>🧪 Real-world pilot testing with partners</li>
                <li style={{ marginBottom: '0.5rem' }}>⚡ Performance optimization</li>
                <li style={{ marginBottom: '0.5rem' }}>📊 External validation and feedback</li>
              </ul>
            </div>

            {/* Phase 3: Traction-Based */}
            <div style={{
              background: 'rgba(10, 22, 40, 0.7)',
              border: '2px solid rgba(124, 58, 237, 0.3)',
              borderRadius: '12px',
              padding: '2rem',
              backdropFilter: 'blur(10px)'
            }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '1rem', marginBottom: '1rem' }}>
                <div style={{
                  background: 'rgba(124, 58, 237, 0.2)',
                  color: '#7C3AED',
                  fontWeight: 'bold',
                  padding: '0.5rem 1rem',
                  borderRadius: '6px',
                  fontSize: '0.875rem',
                  border: '1px solid rgba(124, 58, 237, 0.5)'
                }}>
                  TRACTION-BASED
                </div>
                <Title level={3} style={{ color: '#7C3AED', margin: 0, fontSize: '1.5rem' }}>
                  GEN Coin → Mainnet Launch
                </Title>
              </div>
              <Paragraph style={{ color: '#ffffff', marginBottom: '1rem', lineHeight: '1.6' }}>
                <strong style={{ color: '#7C3AED' }}>Depends on:</strong> Traction, testing validation, and funding
              </Paragraph>
              <ul style={{ color: '#ffffff', fontSize: '0.875rem', listStyle: 'none', padding: 0, lineHeight: '1.8' }}>
                <li style={{ marginBottom: '0.5rem' }}>📊 Market traction and user adoption</li>
                <li style={{ marginBottom: '0.5rem' }}>✅ Complete testing and validation</li>
                <li style={{ marginBottom: '0.5rem' }}>💰 Adequate funding secured</li>
                <li style={{ marginBottom: '0.5rem' }}>🪙 Trigger GEN coin distribution</li>
                <li style={{ marginBottom: '0.5rem' }}>🚀 Launch mainnet (codes ready)</li>
              </ul>
            </div>
          </div>
        </div>
      </section>

      {/* About the Developer */}
      <section style={{ padding: '5rem 0', background: 'transparent' }}>
        <div style={{ maxWidth: '64rem', margin: '0 auto', padding: '0 2rem' }}>
          <div style={{ textAlign: 'center', marginBottom: '3rem' }}>
            <Title level={2} style={{ fontSize: '2.5rem', fontWeight: 'bold', marginBottom: '1rem', color: '#E8B44F' }}>
              About the Developer
            </Title>
            <Paragraph style={{ fontSize: '1.25rem', color: '#ffffff', maxWidth: '48rem', margin: '0 auto' }}>
              Background, approach, and why this is a solo research project
            </Paragraph>
          </div>

          <div style={{
            background: 'rgba(10, 22, 40, 0.9)',
            border: '2px solid rgba(232, 180, 79, 0.3)',
            borderRadius: '12px',
            padding: '3rem',
            backdropFilter: 'blur(10px)'
          }}>
            <Paragraph style={{ color: '#ffffff', fontSize: '1.125rem', lineHeight: '1.8', marginBottom: '1.5rem' }}>
              This project is developed by a <strong style={{ color: '#E8B44F' }}>self-taught systems engineer</strong> with deep experience in distributed systems, blockchain architecture, and infrastructure design. The technical foundation comes from years of hands-on learning, experimentation, and building real systems—not from traditional academic paths.
            </Paragraph>

            <Paragraph style={{ color: '#ffffff', fontSize: '1.125rem', lineHeight: '1.8', marginBottom: '1.5rem' }}>
              <strong style={{ color: '#E8B44F' }}>Why solo?</strong> This is a deliberate choice for deep, focused research. Complex systems architecture requires sustained concentration and coherent vision. Working alone allows for rapid iteration, architectural consistency, and the freedom to explore unconventional approaches without committee consensus. It's not about isolation—it's about <strong style={{ color: '#E8B44F' }}>depth over breadth</strong> during the research phase.
            </Paragraph>

            <Paragraph style={{ color: '#ffffff', fontSize: '1.125rem', lineHeight: '1.8', marginBottom: '1.5rem' }}>
              The 75% infrastructure completion represents <strong style={{ color: '#E8B44F' }}>real, operational code</strong>—15 backend services, authentication systems, databases, and networking infrastructure. This isn't vaporware or whitepapers. The remaining 25% (testing, security audits, external validation) requires collaboration, which is why pilot partnerships are critical.
            </Paragraph>

            <Paragraph style={{ color: '#ffffff', fontSize: '1.125rem', lineHeight: '1.8', marginBottom: '0' }}>
              <strong style={{ color: '#E8B44F' }}>What this means for partners:</strong> You're working with someone who has built the system from the ground up and understands every component deeply. The trade-off is that external validation, security audits, and real-world testing are essential next steps. This is research-grade infrastructure that needs production-grade validation—and that's exactly what we're seeking partners for.
            </Paragraph>
          </div>
        </div>
      </section>

      {/* Core Principles - Redesigned */}
      <section style={{ padding: '5rem 0', background: 'transparent' }}>
        <div style={{ maxWidth: '72rem', margin: '0 auto', padding: '0 2rem' }}>
          <div style={{ textAlign: 'center', marginBottom: '4rem' }}>
            <Title level={2} style={{ fontSize: '2.5rem', fontWeight: 'bold', marginBottom: '1rem', color: '#E8B44F' }}>
              Our Principles
            </Title>
            <Paragraph style={{ fontSize: '1.25rem', color: '#ffffff', maxWidth: '48rem', margin: '0 auto' }}>
              What guides our research and development
            </Paragraph>
          </div>

          <Row gutter={[32, 32]}>
            <Col xs={24} md={8}>
              <div style={{
                background: 'rgba(10, 22, 40, 0.8)',
                border: '2px solid rgba(232, 180, 79, 0.3)',
                borderRadius: '12px',
                padding: '2rem',
                textAlign: 'center',
                height: '100%',
                backdropFilter: 'blur(10px)',
                transition: 'transform 0.2s, border-color 0.2s',
                cursor: 'pointer'
              }}
              onMouseEnter={(e) => {
                e.currentTarget.style.transform = 'translateY(-4px)';
                e.currentTarget.style.borderColor = 'rgba(232, 180, 79, 0.6)';
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.transform = 'translateY(0)';
                e.currentTarget.style.borderColor = 'rgba(232, 180, 79, 0.3)';
              }}>
                <div style={{ fontSize: '3rem', marginBottom: '1rem' }}>🔍</div>
                <Title level={3} style={{ color: '#E8B44F', marginBottom: '1rem', fontSize: '1.5rem' }}>
                  Complete Transparency
                </Title>
                <Paragraph style={{ color: '#ffffff', lineHeight: '1.6' }}>
                  Full honesty about our 75% ready status, limitations, and realistic timelines. No overselling or false claims.
                </Paragraph>
              </div>
            </Col>

            <Col xs={24} md={8}>
              <div style={{
                background: 'rgba(10, 22, 40, 0.8)',
                border: '2px solid rgba(232, 180, 79, 0.3)',
                borderRadius: '12px',
                padding: '2rem',
                textAlign: 'center',
                height: '100%',
                backdropFilter: 'blur(10px)',
                transition: 'transform 0.2s, border-color 0.2s',
                cursor: 'pointer'
              }}
              onMouseEnter={(e) => {
                e.currentTarget.style.transform = 'translateY(-4px)';
                e.currentTarget.style.borderColor = 'rgba(232, 180, 79, 0.6)';
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.transform = 'translateY(0)';
                e.currentTarget.style.borderColor = 'rgba(232, 180, 79, 0.3)';
              }}>
                <div style={{ fontSize: '3rem', marginBottom: '1rem' }}>🔬</div>
                <Title level={3} style={{ color: '#E8B44F', marginBottom: '1rem', fontSize: '1.5rem' }}>
                  Research-Driven
                </Title>
                <Paragraph style={{ color: '#ffffff', lineHeight: '1.6' }}>
                  Single-engineer experimental project exploring distributed systems. Needs testing and external validation.
                </Paragraph>
              </div>
            </Col>

            <Col xs={24} md={8}>
              <div style={{
                background: 'rgba(10, 22, 40, 0.8)',
                border: '2px solid rgba(232, 180, 79, 0.3)',
                borderRadius: '12px',
                padding: '2rem',
                textAlign: 'center',
                height: '100%',
                backdropFilter: 'blur(10px)',
                transition: 'transform 0.2s, border-color 0.2s',
                cursor: 'pointer'
              }}
              onMouseEnter={(e) => {
                e.currentTarget.style.transform = 'translateY(-4px)';
                e.currentTarget.style.borderColor = 'rgba(232, 180, 79, 0.6)';
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.transform = 'translateY(0)';
                e.currentTarget.style.borderColor = 'rgba(232, 180, 79, 0.3)';
              }}>
                <div style={{ fontSize: '3rem', marginBottom: '1rem' }}>🤝</div>
                <Title level={3} style={{ color: '#E8B44F', marginBottom: '1rem', fontSize: '1.5rem' }}>
                  Collaboration-Focused
                </Title>
                <Paragraph style={{ color: '#ffffff', lineHeight: '1.6' }}>
                  Seeking pilot partners for testing and validation. Building with feedback, not in isolation.
                </Paragraph>
              </div>
            </Col>
          </Row>
        </div>
      </section>

      {/* Call to Action */}
      <section style={{ padding: '5rem 0', background: 'transparent' }}>
        <div style={{ maxWidth: '64rem', margin: '0 auto', padding: '0 2rem', textAlign: 'center' }}>
          <Title level={2} style={{ 
            fontSize: '2.5rem', 
            fontWeight: 'bold', 
            marginBottom: '2rem',
            color: '#E8B44F'
          }}>
            Ready to Collaborate?
          </Title>
          <Paragraph style={{ 
            fontSize: '1.25rem',
            color: '#ffffff',
            lineHeight: '1.8',
            marginBottom: '2rem',
            maxWidth: '48rem',
            margin: '0 auto 2rem auto'
          }}>
            We're seeking pilot partners who understand experimental technology and are willing to collaborate on testing and validation.
          </Paragraph>
          <div style={{ display: 'flex', gap: '1rem', justifyContent: 'center', flexWrap: 'wrap' }}>
            <Button 
              type="primary" 
              size="large"
              style={{
                background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)',
                border: 'none',
                color: '#0A1628',
                fontWeight: '600',
                height: '48px',
                padding: '0 2rem',
                fontSize: '1rem'
              }}
              onClick={() => window.location.href = '/contact'}
            >
              Contact Us
            </Button>
            <Button 
              size="large"
              style={{
                background: 'transparent',
                border: '2px solid #E8B44F',
                color: '#E8B44F',
                fontWeight: '600',
                height: '48px',
                padding: '0 2rem',
                fontSize: '1rem'
              }}
              onClick={() => window.location.href = '/research'}
            >
              Explore Research
            </Button>
          </div>
        </div>
      </section>
    </div>
  );
};

export default About;
