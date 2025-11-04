import React, { useState } from 'react';
import { Typography, Button, Row, Col } from 'antd';
import './Enterprise.css';

const { Title, Paragraph } = Typography;

// Reusable 5-Level Component
const FiveLevelSection = ({ title, color, what, whyNow, whatYouGet, required, howToStart }: any) => (
  <div style={{
    background: 'rgba(10, 22, 40, 0.9)',
    border: `2px solid ${color}33`,
    borderRadius: '12px',
    padding: '3rem',
    marginBottom: '2rem',
    backdropFilter: 'blur(10px)'
  }}>
    <Title level={3} style={{ color, marginBottom: '1.5rem' }}>{title}</Title>
    
    <div style={{ marginBottom: '2rem' }}>
      <Title level={4} style={{ color: '#E8B44F', marginBottom: '1rem' }}>1. What</Title>
      <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8' }}>{what}</Paragraph>
    </div>

    <div style={{ marginBottom: '2rem' }}>
      <Title level={4} style={{ color: '#E8B44F', marginBottom: '1rem' }}>2. Why Now</Title>
      <div style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8' }}>{whyNow}</div>
    </div>

    <div style={{ marginBottom: '2rem' }}>
      <Title level={4} style={{ color: '#E8B44F', marginBottom: '1rem' }}>3. What You Get</Title>
      <div style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8' }}>{whatYouGet}</div>
    </div>

    <div style={{ marginBottom: '2rem' }}>
      <Title level={4} style={{ color: '#E8B44F', marginBottom: '1rem' }}>4. What's Required</Title>
      <div style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8' }}>{required}</div>
    </div>

    <div>
      <Title level={4} style={{ color: '#E8B44F', marginBottom: '1rem' }}>5. How to Start</Title>
      {howToStart}
    </div>
  </div>
);

const Enterprise: React.FC = () => {
  const [activeSection, setActiveSection] = useState<number>(0);

  const sections = [
    { id: 0, label: 'Overview', emoji: '📋' },
    { id: 1, label: 'Contributors', emoji: '👨‍💻' },
    { id: 2, label: 'Pilot Partners', emoji: '🧪' },
    { id: 3, label: 'Institutional', emoji: '🏛️' },
    { id: 4, label: 'Pre-Fundable', emoji: '💰' },
    { id: 5, label: 'IP Investors', emoji: '🔬' },
    { id: 6, label: 'Web3 Vision', emoji: '🌐' }
  ];

  return (
    <div className="enterprise-page">
      {/* Hero Section */}
      <section className="hero-gradient" style={{ padding: '8rem 0 6rem 0', textAlign: 'center' }}>
        <div style={{ maxWidth: '72rem', margin: '0 auto', padding: '0 2rem' }}>
          <Title level={1} style={{ color: '#ffffff', fontSize: '3rem', fontWeight: 'bold', marginBottom: '1.5rem' }}>
            Contribution & Partnership Opportunities
          </Title>
          <Paragraph style={{ color: '#ffffff', fontSize: '1.25rem', maxWidth: '48rem', margin: '0 auto 2rem auto', lineHeight: '1.8' }}>
            75% operational infrastructure seeking validation partners. Clear value propositions for each contribution type.
          </Paragraph>
          
          {/* 7 Section Selector */}
          <div style={{ display: 'flex', gap: '0.5rem', justifyContent: 'center', flexWrap: 'wrap', marginTop: '2rem' }}>
            {sections.map((section) => (
              <Button
                key={section.id}
                size="large"
                style={{
                  background: activeSection === section.id ? 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)' : 'rgba(255, 255, 255, 0.1)',
                  border: activeSection === section.id ? 'none' : '1px solid rgba(255, 255, 255, 0.3)',
                  color: activeSection === section.id ? '#0A1628' : '#ffffff',
                  fontWeight: '600',
                  padding: '0 1.5rem',
                  height: '48px'
                }}
                onClick={() => setActiveSection(section.id)}
              >
                {section.emoji} {section.label}
              </Button>
            ))}
          </div>
        </div>
      </section>

      {/* Section 0: Overview */}
      {activeSection === 0 && (
        <section style={{ padding: '5rem 0' }}>
          <div style={{ maxWidth: '72rem', margin: '0 auto', padding: '0 2rem' }}>
            <Title level={2} style={{ fontSize: '2.5rem', fontWeight: 'bold', marginBottom: '2rem', color: '#E8B44F', textAlign: 'center' }}>
              📋 Overview: Why This Page Exists
            </Title>
            
            <div style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(232, 180, 79, 0.3)', borderRadius: '12px', padding: '3rem', backdropFilter: 'blur(10px)' }}>
              <Paragraph style={{ color: '#ffffff', fontSize: '1.125rem', lineHeight: '1.8', marginBottom: '2rem' }}>
                <strong style={{ color: '#E8B44F' }}>75% operational infrastructure:</strong> 15 services, 4 databases, dynamic networking. Not vaporware—operational code on testnet. Not selling a product—seeking validation partners.
              </Paragraph>

              <Title level={3} style={{ color: '#E8B44F', marginBottom: '1rem' }}>The Wave (Why Now)</Title>
              <Row gutter={[24, 24]} style={{ marginBottom: '2rem' }}>
                <Col xs={24} md={8}>
                  <div style={{ padding: '1.5rem', background: 'rgba(16, 185, 129, 0.1)', borderRadius: '8px', border: '1px solid rgba(16, 185, 129, 0.3)' }}>
                    <Title level={4} style={{ color: '#10B981', marginBottom: '0.5rem' }}>75% Complete</Title>
                    <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', marginBottom: 0 }}>
                      Not 0% (vaporware) and not 100% (too late). Early contributors shape the platform.
                    </Paragraph>
                  </div>
                </Col>
                <Col xs={24} md={8}>
                  <div style={{ padding: '1.5rem', background: 'rgba(124, 58, 237, 0.1)', borderRadius: '8px', border: '1px solid rgba(124, 58, 237, 0.3)' }}>
                    <Title level={4} style={{ color: '#7C3AED', marginBottom: '0.5rem' }}>Testnet → Mainnet</Title>
                    <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', marginBottom: 0 }}>
                      Testnet operational. Mainnet depends on traction. Early contributors trigger GEN coin.
                    </Paragraph>
                  </div>
                </Col>
                <Col xs={24} md={8}>
                  <div style={{ padding: '1.5rem', background: 'rgba(245, 158, 11, 0.1)', borderRadius: '8px', border: '1px solid rgba(245, 158, 11, 0.3)' }}>
                    <Title level={4} style={{ color: '#F59E0B', marginBottom: '0.5rem' }}>Shape vs Use</Title>
                    <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', marginBottom: 0 }}>
                      Early contributors shape. Late adopters use what's built. Your choice.
                    </Paragraph>
                  </div>
                </Col>
              </Row>

              <div style={{ background: 'rgba(232, 180, 79, 0.1)', border: '1px solid rgba(232, 180, 79, 0.3)', borderRadius: '8px', padding: '1.5rem', textAlign: 'center' }}>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', marginBottom: 0 }}>
                  <strong style={{ color: '#E8B44F' }}>Select your category above.</strong> Each section: What → Why Now → What You Get → Required → How to Start
                </Paragraph>
              </div>
            </div>
          </div>
        </section>
      )}

      {/* Section 1: Contributors */}
      {activeSection === 1 && (
        <section style={{ padding: '5rem 0' }}>
          <div style={{ maxWidth: '72rem', margin: '0 auto', padding: '0 2rem' }}>
            <Title level={2} style={{ fontSize: '2.5rem', fontWeight: 'bold', marginBottom: '2rem', color: '#10B981', textAlign: 'center' }}>
              👨‍💻 Contributors: Code & Research
            </Title>

            <FiveLevelSection
              title="💻 Code Contributors"
              color="#10B981"
              what="Contribute to Rust codebase: consensus (LCCD), networking (DynaRoute), cryptography, storage (4D hash-graph), or 15 backend services."
              whyNow={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li><strong>75% complete:</strong> See what's real (not starting from zero)</li>
                  <li><strong>25% remaining:</strong> Your contribution matters (not just bug fixes)</li>
                  <li><strong>Early contributors:</strong> Recognition, influence, potential equity/tokens</li>
                </ul>
              }
              whatYouGet={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Deep technical learning (LCCD, DynaRoute, 6D blockchain, quantum crypto)</li>
                  <li>Contributor recognition + architecture influence</li>
                  <li>Potential equity/tokens (significant contributions)</li>
                </ul>
              }
              required={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Rust proficiency (async/await, tokio, distributed systems)</li>
                  <li>Time commitment (varies by contribution)</li>
                  <li>Production-grade code + tests + docs</li>
                </ul>
              }
              howToStart={
                <div style={{ display: 'flex', gap: '1rem', flexWrap: 'wrap' }}>
                  <Button type="primary" size="large" style={{ background: 'linear-gradient(135deg, #10B981 0%, #FFFFFF 100%)', border: 'none', color: '#0A1628', fontWeight: '600' }}>
                    GitHub
                  </Button>
                  <Button size="large" style={{ background: 'transparent', border: '2px solid #10B981', color: '#10B981', fontWeight: '600' }}>
                    umesh@pravyom.com
                  </Button>
                </div>
              }
            />

            <FiveLevelSection
              title="🔬 Research Contributors"
              color="#10B981"
              what="Research on consensus, quantum cryptography, distributed systems. Access operational testnet for validation."
              whyNow={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Experimental infrastructure = real research opportunities</li>
                  <li>Operational testnet to test theories</li>
                  <li>Academic papers + co-authorship opportunities</li>
                </ul>
              }
              whatYouGet={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Full testnet access</li>
                  <li>Co-authorship on papers</li>
                  <li>Research grants (when available)</li>
                </ul>
              }
              required={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Academic background (CS, cryptography, distributed systems)</li>
                  <li>Research proposal + methodology</li>
                  <li>Collaboration commitment</li>
                </ul>
              }
              howToStart={
                <Button type="primary" size="large" style={{ background: 'linear-gradient(135deg, #10B981 0%, #FFFFFF 100%)', border: 'none', color: '#0A1628', fontWeight: '600' }}>
                  umesh@pravyom.com
                </Button>
              }
            />
          </div>
        </section>
      )}

      {/* Section 2: Pilot Partners */}
      {activeSection === 2 && (
        <section style={{ padding: '5rem 0' }}>
          <div style={{ maxWidth: '72rem', margin: '0 auto', padding: '0 2rem' }}>
            <Title level={2} style={{ fontSize: '2.5rem', fontWeight: 'bold', marginBottom: '2rem', color: '#3B82F6', textAlign: 'center' }}>
              🧪 Pilot Partners: Testing & Validation
            </Title>

            <FiveLevelSection
              title="⚙️ Technical Pilots"
              color="#3B82F6"
              what="Test infrastructure with real workloads. Provide feedback. Validate performance, security, reliability."
              whyNow={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Infrastructure operational (not vaporware)</li>
                  <li>Early pilots get preferential mainnet terms</li>
                  <li>Influence feature roadmap</li>
                </ul>
              }
              whatYouGet={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Free testnet access + dedicated support</li>
                  <li>Influence roadmap</li>
                  <li>Early mainnet access (discounted)</li>
                </ul>
              }
              required={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Real use case + technical team</li>
                  <li>Monthly feedback reports</li>
                  <li>3-6 month pilot period</li>
                </ul>
              }
              howToStart={
                <Button type="primary" size="large" style={{ background: 'linear-gradient(135deg, #3B82F6 0%, #FFFFFF 100%)', border: 'none', color: '#0A1628', fontWeight: '600' }}>
                  umesh@pravyom.com
                </Button>
              }
            />

            <FiveLevelSection
              title="💼 Business Pilots"
              color="#3B82F6"
              what="Test business models. Validate market fit. Explore revenue opportunities."
              whyNow={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Infrastructure ready for business testing</li>
                  <li>Shape pricing, features, partnerships</li>
                  <li>First-mover advantage</li>
                </ul>
              }
              whatYouGet={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Free testnet + business development support</li>
                  <li>Revenue sharing opportunities</li>
                  <li>Early mainnet access</li>
                </ul>
              }
              required={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Business plan + revenue strategy</li>
                  <li>Market validation with real users</li>
                  <li>6-12 month pilot</li>
                </ul>
              }
              howToStart={
                <Button type="primary" size="large" style={{ background: 'linear-gradient(135deg, #3B82F6 0%, #FFFFFF 100%)', border: 'none', color: '#0A1628', fontWeight: '600' }}>
                  umesh@pravyom.com
                </Button>
              }
            />
          </div>
        </section>
      )}

      {/* Section 3: Institutional */}
      {activeSection === 3 && (
        <section style={{ padding: '5rem 0' }}>
          <div style={{ maxWidth: '72rem', margin: '0 auto', padding: '0 2rem' }}>
            <Title level={2} style={{ fontSize: '2.5rem', fontWeight: 'bold', marginBottom: '2rem', color: '#E8B44F', textAlign: 'center' }}>
              🏛️ Institutional Pilots
            </Title>

            <FiveLevelSection
              title="🏢 Enterprise Pilots"
              color="#E8B44F"
              what="Deploy for enterprise use cases: supply chain, audit, compliance. 6D blockchain = government-grade audit trails."
              whyNow={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Post-quantum ready = future-proof</li>
                  <li>Early enterprises get custom features</li>
                  <li>Impossible-to-hide audit trails</li>
                </ul>
              }
              whatYouGet={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Dedicated support + custom features</li>
                  <li>White-label options</li>
                  <li>Enterprise SLA (mainnet)</li>
                </ul>
              }
              required={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Enterprise use case + budget</li>
                  <li>12+ month commitment</li>
                </ul>
              }
              howToStart={
                <Button type="primary" size="large" style={{ background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)', border: 'none', color: '#0A1628', fontWeight: '600' }}>
                  umesh@pravyom.com
                </Button>
              }
            />

            <FiveLevelSection
              title="🏛️ Government Pilots"
              color="#E8B44F"
              what="Deploy for government: digital identity, land registry, voting, compliance. Impossible-to-hide audit trails (CBOR pipeline)."
              whyNow={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Quantum-ready security</li>
                  <li>Early governments shape compliance features</li>
                  <li>Sovereign deployment options</li>
                </ul>
              }
              whatYouGet={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Government-grade security</li>
                  <li>Compliance support + regulatory framework</li>
                  <li>Sovereign deployment</li>
                </ul>
              }
              required={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Government mandate + regulatory approval</li>
                  <li>Multi-year commitment</li>
                </ul>
              }
              howToStart={
                <Button type="primary" size="large" style={{ background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)', border: 'none', color: '#0A1628', fontWeight: '600' }}>
                  umesh@pravyom.com
                </Button>
              }
            />
          </div>
        </section>
      )}

      {/* Section 4: Pre-Fundable */}
      {activeSection === 4 && (
        <section style={{ padding: '5rem 0' }}>
          <div style={{ maxWidth: '72rem', margin: '0 auto', padding: '0 2rem' }}>
            <Title level={2} style={{ fontSize: '2.5rem', fontWeight: 'bold', marginBottom: '2rem', color: '#7C3AED', textAlign: 'center' }}>
              💰 Pre-Fundable Pilots
            </Title>

            <FiveLevelSection
              title="👼 Angel/Seed Investors"
              color="#7C3AED"
              what="Invest in pilot phase. Get equity/tokens before formal rounds. Pre-seed valuation."
              whyNow={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>75% complete = de-risked</li>
                  <li>Pre-mainnet = early valuation</li>
                  <li>Help trigger GEN coin launch</li>
                </ul>
              }
              whatYouGet={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Equity/token allocation (pre-seed valuation)</li>
                  <li>Board observer seat (significant investment)</li>
                  <li>Influence tokenomics</li>
                </ul>
              }
              required={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Investment: $50K - $500K</li>
                  <li>Strategic value (not just capital)</li>
                  <li>3-5 year vision</li>
                </ul>
              }
              howToStart={
                <Button type="primary" size="large" style={{ background: 'linear-gradient(135deg, #7C3AED 0%, #FFFFFF 100%)', border: 'none', color: '#0A1628', fontWeight: '600' }}>
                  umesh@pravyom.com
                </Button>
              }
            />

            <FiveLevelSection
              title="🤝 Strategic Partners"
              color="#7C3AED"
              what="Provide strategic value (distribution, partnerships, expertise) + capital."
              whyNow={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Infrastructure ready for partnerships</li>
                  <li>Shape go-to-market</li>
                  <li>Mainnet depends on ecosystem</li>
                </ul>
              }
              whatYouGet={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Equity/token allocation</li>
                  <li>Partnership agreement + co-marketing</li>
                  <li>Revenue sharing</li>
                </ul>
              }
              required={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Strategic value proposition</li>
                  <li>Investment: $100K - $1M</li>
                  <li>Active partnership</li>
                </ul>
              }
              howToStart={
                <Button type="primary" size="large" style={{ background: 'linear-gradient(135deg, #7C3AED 0%, #FFFFFF 100%)', border: 'none', color: '#0A1628', fontWeight: '600' }}>
                  umesh@pravyom.com
                </Button>
              }
            />
          </div>
        </section>
      )}

      {/* Section 5: IP Investors */}
      {activeSection === 5 && (
        <section style={{ padding: '5rem 0' }}>
          <div style={{ maxWidth: '72rem', margin: '0 auto', padding: '0 2rem' }}>
            <Title level={2} style={{ fontSize: '2.5rem', fontWeight: 'bold', marginBottom: '2rem', color: '#F59E0B', textAlign: 'center' }}>
              🔬 IP Investors
            </Title>

            <FiveLevelSection
              title="💡 Technology IP Investors"
              color="#F59E0B"
              what="Invest in technology IP: LCCD consensus, DynaRoute, 6D blockchain, quantum systems. Patent-pending."
              whyNow={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Novel technologies (not derivatives)</li>
                  <li>Patent-pending systems</li>
                  <li>Early IP investors get licensing rights</li>
                </ul>
              }
              whatYouGet={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>IP licensing rights</li>
                  <li>Technology transfer agreements</li>
                  <li>Royalty sharing</li>
                </ul>
              }
              required={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>IP investment: $500K - $5M</li>
                  <li>Technology expertise</li>
                  <li>Long-term IP strategy</li>
                </ul>
              }
              howToStart={
                <Button type="primary" size="large" style={{ background: 'linear-gradient(135deg, #F59E0B 0%, #FFFFFF 100%)', border: 'none', color: '#0A1628', fontWeight: '600' }}>
                  umesh@pravyom.com
                </Button>
              }
            />

            <FiveLevelSection
              title="📜 Patent/Licensing Partners"
              color="#F59E0B"
              what="Partner on patent filings, licensing, technology commercialization."
              whyNow={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Novel systems ready for patenting</li>
                  <li>Shape IP strategy</li>
                  <li>Licensing opportunities (multiple industries)</li>
                </ul>
              }
              whatYouGet={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Co-ownership of patents</li>
                  <li>Licensing revenue share</li>
                  <li>Commercialization rights</li>
                </ul>
              }
              required={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Patent expertise</li>
                  <li>Investment in patent filings</li>
                  <li>Commercialization strategy</li>
                </ul>
              }
              howToStart={
                <Button type="primary" size="large" style={{ background: 'linear-gradient(135deg, #F59E0B 0%, #FFFFFF 100%)', border: 'none', color: '#0A1628', fontWeight: '600' }}>
                  umesh@pravyom.com
                </Button>
              }
            />
          </div>
        </section>
      )}

      {/* Section 6: Web3 Vision */}
      {activeSection === 6 && (
        <section style={{ padding: '5rem 0' }}>
          <div style={{ maxWidth: '72rem', margin: '0 auto', padding: '0 2rem' }}>
            <Title level={2} style={{ fontSize: '2.5rem', fontWeight: 'bold', marginBottom: '2rem', color: '#06B6D4', textAlign: 'center' }}>
              🌐 Web3 Decentralized Vision
            </Title>

            <FiveLevelSection
              title="🛠️ Decentralized Ecosystem Builders"
              color="#06B6D4"
              what="Build decentralized applications, tools, infrastructure on top of platform."
              whyNow={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Infrastructure 75% ready</li>
                  <li>Early builders shape ecosystem</li>
                  <li>GEN coin launch rewards early builders</li>
                </ul>
              }
              whatYouGet={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Free testnet access</li>
                  <li>Developer grants (when available)</li>
                  <li>GEN coin allocation (significant contributions)</li>
                  <li>Ecosystem fund access</li>
                </ul>
              }
              required={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Decentralized vision</li>
                  <li>Technical capability</li>
                  <li>Community commitment</li>
                </ul>
              }
              howToStart={
                <Button type="primary" size="large" style={{ background: 'linear-gradient(135deg, #06B6D4 0%, #FFFFFF 100%)', border: 'none', color: '#0A1628', fontWeight: '600' }}>
                  umesh@pravyom.com
                </Button>
              }
            />

            <FiveLevelSection
              title="🗳️ Community Governance Contributors"
              color="#06B6D4"
              what="Participate in governance. Help shape decentralized future."
              whyNow={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Governance model being designed</li>
                  <li>Early contributors shape governance</li>
                  <li>Decentralization roadmap depends on community</li>
                </ul>
              }
              whatYouGet={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Governance tokens (when launched)</li>
                  <li>Voting rights</li>
                  <li>Community recognition</li>
                  <li>Influence platform direction</li>
                </ul>
              }
              required={
                <ul style={{ paddingLeft: '1.5rem', margin: 0 }}>
                  <li>Long-term commitment</li>
                  <li>Governance expertise</li>
                  <li>Community building skills</li>
                </ul>
              }
              howToStart={
                <Button type="primary" size="large" style={{ background: 'linear-gradient(135deg, #06B6D4 0%, #FFFFFF 100%)', border: 'none', color: '#0A1628', fontWeight: '600' }}>
                  umesh@pravyom.com
                </Button>
              }
            />
          </div>
        </section>
      )}
    </div>
  );
};

export default Enterprise;
