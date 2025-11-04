import React, { useState } from 'react';
import { Typography, Button, Row, Col } from 'antd';
import './Technology.css';

const { Title, Paragraph } = Typography;

const Technology: React.FC = () => {
  const [activeSection, setActiveSection] = useState<number>(1);

  return (
    <div className="technology-page">
      {/* Hero Section */}
      <section className="hero-gradient" style={{ padding: '8rem 0 6rem 0', textAlign: 'center' }}>
        <div style={{ maxWidth: '72rem', margin: '0 auto', padding: '0 2rem' }}>
          <Title level={1} style={{ color: '#ffffff', fontSize: '3rem', fontWeight: 'bold', marginBottom: '1.5rem' }}>
            Technology Overview
          </Title>
          <Paragraph style={{ color: '#ffffff', fontSize: '1.25rem', maxWidth: '48rem', margin: '0 auto 2rem auto', lineHeight: '1.8' }}>
            Progressive technical overview for different audiences—from general public to infrastructure engineers
          </Paragraph>
          
          {/* Audience Selector */}
          <div style={{ display: 'flex', gap: '0.5rem', justifyContent: 'center', flexWrap: 'wrap', marginTop: '2rem' }}>
            {[
              { id: 1, label: 'General Public', emoji: '👥' },
              { id: 2, label: 'Business & Technical', emoji: '💼' },
              { id: 3, label: 'Developers (SDK/BPI OS)', emoji: '👨‍💻' },
              { id: 4, label: 'Web3 Community', emoji: '🌐' },
              { id: 5, label: 'Infrastructure Engineers', emoji: '⚙️' }
            ].map((section) => (
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

      {/* Section 1: General Public */}
      {activeSection === 1 && (
        <section style={{ padding: '5rem 0', background: 'transparent' }}>
          <div style={{ maxWidth: '64rem', margin: '0 auto', padding: '0 2rem' }}>
            <Title level={2} style={{ fontSize: '2.5rem', fontWeight: 'bold', marginBottom: '2rem', color: '#E8B44F', textAlign: 'center' }}>
              👥 For Everyone: The Future of Digital Infrastructure
            </Title>
            
            <div style={{
              background: 'rgba(10, 22, 40, 0.9)',
              border: '2px solid rgba(232, 180, 79, 0.3)',
              borderRadius: '12px',
              padding: '3rem',
              marginBottom: '2rem',
              backdropFilter: 'blur(10px)'
            }}>
              <Paragraph style={{ color: '#ffffff', fontSize: '1.25rem', lineHeight: '1.8', marginBottom: '1.5rem' }}>
                <strong style={{ color: '#E8B44F' }}>Imagine the internet, but smarter and more secure:</strong> We're building infrastructure that can handle millions of transactions, remember everything (like a perfect memory), and protect your data using mathematics that even future quantum computers can't break.
              </Paragraph>

              <Paragraph style={{ color: '#ffffff', fontSize: '1.25rem', lineHeight: '1.8', marginBottom: '1.5rem' }}>
                <strong style={{ color: '#E8B44F' }}>What makes it feel like the future?</strong> Our system can grow infinitely (like cells dividing), communicate without fixed addresses (like a living organism), and track every transaction in 6 dimensions (not just who sent what, but when, why, how, and with what proof). It's like upgrading from a filing cabinet to a living, breathing digital ecosystem.
              </Paragraph>

              <Paragraph style={{ color: '#ffffff', fontSize: '1.25rem', lineHeight: '1.8', marginBottom: '1.5rem' }}>
                <strong style={{ color: '#E8B44F' }}>Real-world comparison:</strong> If Bitcoin is like digital gold (store of value), and Ethereum is like a world computer (smart contracts), we're building the <em>operating system</em> that entire digital governments and enterprises can run on—with audit trails so perfect that nothing can be hidden.
              </Paragraph>

              <Paragraph style={{ color: '#ffffff', fontSize: '1.25rem', lineHeight: '1.8', marginBottom: '0' }}>
                <strong style={{ color: '#E8B44F' }}>Current status:</strong> The core infrastructure (75%) is built and operational—15 backend services running, databases working, networking active. We're now in the testing phase, seeking partners to help validate the system. Timeline: 6-12 months for pilot testing, then mainnet launch based on traction.
              </Paragraph>
            </div>
          </div>
        </section>
      )}

      {/* Section 2: Business & Technical */}
      {activeSection === 2 && (
        <section style={{ padding: '5rem 0', background: 'transparent' }}>
          <div style={{ maxWidth: '72rem', margin: '0 auto', padding: '0 2rem' }}>
            <Title level={2} style={{ fontSize: '2.5rem', fontWeight: 'bold', marginBottom: '2rem', color: '#E8B44F', textAlign: 'center' }}>
              💼 For Business & Technical Decision Makers
            </Title>
            
            <Row gutter={[32, 32]}>
              <Col xs={24} md={12}>
                <div style={{
                  background: 'rgba(10, 22, 40, 0.9)',
                  border: '2px solid rgba(16, 185, 129, 0.3)',
                  borderRadius: '12px',
                  padding: '2rem',
                  height: '100%',
                  backdropFilter: 'blur(10px)'
                }}>
                  <Title level={3} style={{ color: '#10B981', marginBottom: '1rem' }}>✅ What's Operational</Title>
                  <ul style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', paddingLeft: '1.5rem' }}>
                    <li>15 backend services deployed and running</li>
                    <li>Authentication system (Keycloak)</li>
                    <li>Database infrastructure (PostgreSQL, Redis, MongoDB, RabbitMQ)</li>
                    <li>Dynamic networking (DynaRoute v2)</li>
                    <li>Real APIs (not mock data)</li>
                  </ul>
                </div>
              </Col>

              <Col xs={24} md={12}>
                <div style={{
                  background: 'rgba(10, 22, 40, 0.9)',
                  border: '2px solid rgba(245, 158, 11, 0.3)',
                  borderRadius: '12px',
                  padding: '2rem',
                  height: '100%',
                  backdropFilter: 'blur(10px)'
                }}>
                  <Title level={3} style={{ color: '#F59E0B', marginBottom: '1rem' }}>⚠️ What's Needed</Title>
                  <ul style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', paddingLeft: '1.5rem' }}>
                    <li>Real-world testing with pilot partners</li>
                    <li>External security audits</li>
                    <li>Performance optimization under load</li>
                    <li>Regulatory compliance validation</li>
                    <li>Production-grade monitoring</li>
                  </ul>
                </div>
              </Col>
            </Row>

            <div style={{
              background: 'rgba(10, 22, 40, 0.9)',
              border: '2px solid rgba(232, 180, 79, 0.3)',
              borderRadius: '12px',
              padding: '3rem',
              marginTop: '2rem',
              backdropFilter: 'blur(10px)'
            }}>
              <Title level={3} style={{ color: '#E8B44F', marginBottom: '1.5rem' }}>Business Value Proposition</Title>
              <Paragraph style={{ color: '#ffffff', fontSize: '1.125rem', lineHeight: '1.8', marginBottom: '1.5rem' }}>
                <strong>For enterprises:</strong> This is research-grade infrastructure that offers a foundation for distributed applications. The 75% completion means core systems are operational—authentication, databases, networking. The remaining 25% (testing, audits, validation) is where pilot partnerships are critical. You're not investing in vaporware; you're validating operational infrastructure.
              </Paragraph>

              <Title level={4} style={{ color: '#10B981', marginBottom: '1rem' }}>Real-World Comparisons</Title>
              <ul style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', paddingLeft: '1.5rem', marginBottom: '1.5rem' }}>
                <li><strong>vs. Hyperledger Fabric:</strong> We have similar enterprise focus, but with dynamic networking (no static ports), 6D transaction tracking, and quantum-ready cryptography</li>
                <li><strong>vs. Ethereum:</strong> Not EVM-compatible; designed for government/enterprise audit requirements with impossible-to-hide transaction trails</li>
                <li><strong>vs. Cosmos/Polkadot:</strong> Similar multi-chain vision, but with bio-inspired consensus (LCCD) and cellular division scaling instead of validator sets</li>
                <li><strong>vs. AWS/Azure:</strong> Distributed infrastructure that enterprises can run themselves, not rent from cloud providers</li>
              </ul>

              <Title level={4} style={{ color: '#10B981', marginBottom: '1rem' }}>What Makes This Different</Title>
              <Paragraph style={{ color: '#ffffff', fontSize: '1.125rem', lineHeight: '1.8' }}>
                <strong>6D Blockchain:</strong> Transactions tracked in 6 dimensions (sender, receiver, amount, time, proof, intent)—not just 3D (from, to, value). This enables government-grade audit trails where every transaction has quantum-proof evidence and cannot be hidden or manipulated.
              </Paragraph>
            </div>
          </div>
        </section>
      )}

      {/* Section 3: Developers (SDK/BPI OS) */}
      {activeSection === 3 && (
        <section style={{ padding: '5rem 0', background: 'transparent' }}>
          <div style={{ maxWidth: '72rem', margin: '0 auto', padding: '0 2rem' }}>
            <Title level={2} style={{ fontSize: '2.5rem', fontWeight: 'bold', marginBottom: '2rem', color: '#E8B44F', textAlign: 'center' }}>
              👨‍💻 For Developers: What You Could Build
            </Title>
            
            <div style={{
              background: 'rgba(10, 22, 40, 0.9)',
              border: '2px solid rgba(232, 180, 79, 0.3)',
              borderRadius: '12px',
              padding: '3rem',
              marginBottom: '2rem',
              backdropFilter: 'blur(10px)'
            }}>
              <Paragraph style={{ color: '#ffffff', fontSize: '1.25rem', lineHeight: '1.8', marginBottom: '2rem', textAlign: 'center' }}>
                Imagine having access to infrastructure where distributed computing feels like local computing, where every transaction has quantum-proof audit trails, and where your app can scale infinitely without managing servers.
              </Paragraph>

              <div style={{ marginBottom: '2rem' }}>
                <Title level={4} style={{ color: '#10B981', marginBottom: '1rem' }}>💡 What Becomes Possible</Title>
                <Row gutter={[24, 24]}>
                  <Col xs={24} md={12}>
                    <div style={{ padding: '1.5rem', background: 'rgba(16, 185, 129, 0.05)', borderRadius: '8px', border: '1px solid rgba(16, 185, 129, 0.2)' }}>
                      <div style={{ fontSize: '2rem', marginBottom: '0.5rem' }}>🏛️</div>
                      <Title level={5} style={{ color: '#10B981', marginBottom: '0.5rem' }}>Government-Grade Apps</Title>
                      <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', marginBottom: 0 }}>
                        Build applications with audit trails so perfect that nothing can be hidden. Every action tracked in 6 dimensions with quantum proofs.
                      </Paragraph>
                    </div>
                  </Col>
                  <Col xs={24} md={12}>
                    <div style={{ padding: '1.5rem', background: 'rgba(16, 185, 129, 0.05)', borderRadius: '8px', border: '1px solid rgba(16, 185, 129, 0.2)' }}>
                      <div style={{ fontSize: '2rem', marginBottom: '0.5rem' }}>🌐</div>
                      <Title level={5} style={{ color: '#10B981', marginBottom: '0.5rem' }}>Distributed Apps That Scale</Title>
                      <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', marginBottom: 0 }}>
                        Write code that runs across thousands of nodes without thinking about networking, load balancing, or infrastructure.
                      </Paragraph>
                    </div>
                  </Col>
                  <Col xs={24} md={12}>
                    <div style={{ padding: '1.5rem', background: 'rgba(16, 185, 129, 0.05)', borderRadius: '8px', border: '1px solid rgba(16, 185, 129, 0.2)' }}>
                      <div style={{ fontSize: '2rem', marginBottom: '0.5rem' }}>🔐</div>
                      <Title level={5} style={{ color: '#10B981', marginBottom: '0.5rem' }}>Future-Proof Security</Title>
                      <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', marginBottom: 0 }}>
                        Build with post-quantum cryptography today. Your app will still be secure when quantum computers arrive.
                      </Paragraph>
                    </div>
                  </Col>
                  <Col xs={24} md={12}>
                    <div style={{ padding: '1.5rem', background: 'rgba(16, 185, 129, 0.05)', borderRadius: '8px', border: '1px solid rgba(16, 185, 129, 0.2)' }}>
                      <div style={{ fontSize: '2rem', marginBottom: '0.5rem' }}>⚡</div>
                      <Title level={5} style={{ color: '#10B981', marginBottom: '0.5rem' }}>Real-Time Everything</Title>
                      <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', marginBottom: 0 }}>
                        XTMP protocol is 10-20x faster than HTTP. Build real-time applications that feel instant, even at scale.
                      </Paragraph>
                    </div>
                  </Col>
                </Row>
              </div>

              <div style={{ marginBottom: '2rem' }}>
                <Title level={4} style={{ color: '#10B981', marginBottom: '1rem' }}>🛠️ What You Get</Title>
                <div style={{
                  background: 'rgba(0, 0, 0, 0.3)',
                  border: '1px solid rgba(232, 180, 79, 0.3)',
                  borderRadius: '8px',
                  padding: '1.5rem'
                }}>
                  <Row gutter={[16, 16]}>
                    <Col xs={24} md={8}>
                      <div style={{ color: '#E8B44F', fontWeight: 'bold', marginBottom: '0.5rem' }}>BPI OS SDK</div>
                      <ul style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', paddingLeft: '1rem', margin: 0 }}>
                        <li>Rust-based</li>
                        <li>Async/await patterns</li>
                        <li>Distributed by default</li>
                      </ul>
                    </Col>
                    <Col xs={24} md={8}>
                      <div style={{ color: '#E8B44F', fontWeight: 'bold', marginBottom: '0.5rem' }}>15 REST APIs</div>
                      <ul style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', paddingLeft: '1rem', margin: 0 }}>
                        <li>Blockchain operations</li>
                        <li>Wallet management</li>
                        <li>Transaction submission</li>
                      </ul>
                    </Col>
                    <Col xs={24} md={8}>
                      <div style={{ color: '#E8B44F', fontWeight: 'bold', marginBottom: '0.5rem' }}>Auth Built-In</div>
                      <ul style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', paddingLeft: '1rem', margin: 0 }}>
                        <li>Keycloak OAuth2/OIDC</li>
                        <li>Production-ready</li>
                        <li>Enterprise SSO</li>
                      </ul>
                    </Col>
                  </Row>
                </div>
              </div>

              <div style={{ marginBottom: '2rem' }}>
                <Title level={4} style={{ color: '#10B981', marginBottom: '1rem' }}>📝 Example: What Your Code Could Look Like</Title>
                <div style={{
                  background: 'rgba(0, 0, 0, 0.3)',
                  border: '1px solid rgba(16, 185, 129, 0.3)',
                  borderRadius: '8px',
                  padding: '1rem',
                  fontFamily: 'monospace',
                  fontSize: '0.875rem',
                  color: '#10B981',
                  overflowX: 'auto'
                }}>
                  <div style={{ color: '#6B7280', marginBottom: '0.5rem' }}>// Your app: distributed by default</div>
                  <div><span style={{ color: '#7C3AED' }}>use</span> bpi_sdk::{'{'}<span style={{ color: '#E8B44F' }}>BpiClient</span>, <span style={{ color: '#E8B44F' }}>Transaction</span>{'}'};</div>
                  <div style={{ marginTop: '0.5rem' }}><span style={{ color: '#7C3AED' }}>async fn</span> <span style={{ color: '#E8B44F' }}>submit_transaction</span>() {'{'}</div>
                  <div style={{ paddingLeft: '1rem' }}><span style={{ color: '#7C3AED' }}>let</span> client = <span style={{ color: '#E8B44F' }}>BpiClient</span>::connect().<span style={{ color: '#7C3AED' }}>await</span>?;</div>
                  <div style={{ paddingLeft: '1rem' }}><span style={{ color: '#7C3AED' }}>let</span> tx = <span style={{ color: '#E8B44F' }}>Transaction</span>::new(sender, receiver, amount);</div>
                  <div style={{ paddingLeft: '1rem' }}></div>
                  <div style={{ paddingLeft: '1rem', color: '#6B7280' }}>// Automatically gets 6D tracking, quantum proofs, audit trails</div>
                  <div style={{ paddingLeft: '1rem' }}><span style={{ color: '#7C3AED' }}>let</span> result = client.submit(tx).<span style={{ color: '#7C3AED' }}>await</span>?;</div>
                  <div>{'}'}</div>
                  <div style={{ marginTop: '0.5rem', color: '#6B7280' }}>// No infrastructure management. Just code.</div>
                </div>
              </div>

              <div style={{
                background: 'rgba(124, 58, 237, 0.1)',
                border: '1px solid rgba(124, 58, 237, 0.3)',
                borderRadius: '8px',
                padding: '1.5rem'
              }}>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.6', marginBottom: '0' }}>
                  <strong style={{ color: '#7C3AED' }}>Current Status:</strong> SDK is experimental, APIs are operational. If you're the kind of developer who gets excited about building on cutting-edge infrastructure before it's mainstream, this is your chance to shape what becomes possible.
                </Paragraph>
              </div>
            </div>
          </div>
        </section>
      )}

      {/* Section 4: Web3 Community */}
      {activeSection === 4 && (
        <section style={{ padding: '5rem 0', background: 'transparent' }}>
          <div style={{ maxWidth: '72rem', margin: '0 auto', padding: '0 2rem' }}>
            <Title level={2} style={{ fontSize: '2.5rem', fontWeight: 'bold', marginBottom: '2rem', color: '#E8B44F', textAlign: 'center' }}>
              🌐 For Web3 Community: Real Technical Architecture
            </Title>
            
            <div style={{
              background: 'rgba(10, 22, 40, 0.9)',
              border: '2px solid rgba(232, 180, 79, 0.3)',
              borderRadius: '12px',
              padding: '3rem',
              marginBottom: '2rem',
              backdropFilter: 'blur(10px)'
            }}>
              <Title level={3} style={{ color: '#E8B44F', marginBottom: '1.5rem' }}>What the Buzzwords Actually Mean (In Real Code)</Title>
              
              <div style={{ marginBottom: '2rem' }}>
                <Title level={4} style={{ color: '#10B981', marginBottom: '1rem' }}>🧬 LCCD Consensus (Living Cellular Consensus Division)</Title>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', marginBottom: '1rem' }}>
                  <strong style={{ color: '#E8B44F' }}>What it actually is:</strong> Bio-inspired consensus where the network can divide like cells when it grows. Instead of fixed validator sets (PoS) or mining (PoW), consensus nodes can split into sub-clusters when load increases, then merge back when load decreases. Think of it like a living organism that grows and shrinks based on demand.
                </Paragraph>
                <div style={{
                  background: 'rgba(0, 0, 0, 0.3)',
                  border: '1px solid rgba(16, 185, 129, 0.3)',
                  borderRadius: '8px',
                  padding: '1rem',
                  marginTop: '1rem',
                  fontFamily: 'monospace',
                  fontSize: '0.875rem',
                  color: '#10B981',
                  overflowX: 'auto'
                }}>
                  <div style={{ color: '#6B7280', marginBottom: '0.5rem' }}>// Real Rust code from consensus server</div>
                  <div><span style={{ color: '#7C3AED' }}>pub struct</span> <span style={{ color: '#E8B44F' }}>CellularDivisionLogic</span> {'{'}</div>
                  <div style={{ paddingLeft: '1rem' }}>pub cluster_load: <span style={{ color: '#10B981' }}>f64</span>,</div>
                  <div style={{ paddingLeft: '1rem' }}>pub division_threshold: <span style={{ color: '#10B981' }}>f64</span>,</div>
                  <div style={{ paddingLeft: '1rem' }}>pub merge_threshold: <span style={{ color: '#10B981' }}>f64</span>,</div>
                  <div style={{ paddingLeft: '1rem' }}>pub sub_clusters: <span style={{ color: '#10B981' }}>Vec</span>{'<'}SubCluster{'>'}, <span style={{ color: '#6B7280' }}>// Cells</span></div>
                  <div>{'}'}</div>
                  <div style={{ marginTop: '0.5rem', color: '#6B7280' }}>// When load {'>'} 80%, cluster divides like a cell</div>
                </div>
              </div>

              <div style={{ marginBottom: '2rem' }}>
                <Title level={4} style={{ color: '#10B981', marginBottom: '1rem' }}>📦 6D Blockchain</Title>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', marginBottom: '1rem' }}>
                  <strong style={{ color: '#E8B44F' }}>What it actually is:</strong> Transactions stored in 6 dimensions: (1) Sender, (2) Receiver, (3) Amount, (4) Timestamp, (5) Proof-of-Execution, (6) Intent/Metadata. Traditional blockchains only track 3D (from, to, value). We add temporal, proof, and intent dimensions for government-grade audit trails.
                </Paragraph>
                <div style={{
                  background: 'rgba(0, 0, 0, 0.3)',
                  border: '1px solid rgba(16, 185, 129, 0.3)',
                  borderRadius: '8px',
                  padding: '1rem',
                  marginTop: '1rem',
                  fontFamily: 'monospace',
                  fontSize: '0.875rem',
                  color: '#10B981',
                  overflowX: 'auto'
                }}>
                  <div style={{ color: '#6B7280', marginBottom: '0.5rem' }}>// 6D Transaction Structure</div>
                  <div><span style={{ color: '#7C3AED' }}>pub struct</span> <span style={{ color: '#E8B44F' }}>SixDTransaction</span> {'{'}</div>
                  <div style={{ paddingLeft: '1rem' }}>pub sender: <span style={{ color: '#10B981' }}>Address</span>,      <span style={{ color: '#6B7280' }}>// Dimension 1</span></div>
                  <div style={{ paddingLeft: '1rem' }}>pub receiver: <span style={{ color: '#10B981' }}>Address</span>,    <span style={{ color: '#6B7280' }}>// Dimension 2</span></div>
                  <div style={{ paddingLeft: '1rem' }}>pub amount: <span style={{ color: '#10B981' }}>u64</span>,          <span style={{ color: '#6B7280' }}>// Dimension 3</span></div>
                  <div style={{ paddingLeft: '1rem' }}>pub timestamp: <span style={{ color: '#10B981' }}>DateTime</span>,  <span style={{ color: '#6B7280' }}>// Dimension 4</span></div>
                  <div style={{ paddingLeft: '1rem' }}>pub poe_proof: <span style={{ color: '#10B981' }}>Proof</span>,     <span style={{ color: '#6B7280' }}>// Dimension 5 (quantum proof)</span></div>
                  <div style={{ paddingLeft: '1rem' }}>pub intent: <span style={{ color: '#10B981' }}>Metadata</span>,    <span style={{ color: '#6B7280' }}>// Dimension 6 (audit trail)</span></div>
                  <div>{'}'}</div>
                  <div style={{ marginTop: '0.5rem', color: '#6B7280' }}>// Traditional blockchain: only 3D (sender, receiver, amount)</div>
                </div>
              </div>

              <div style={{ marginBottom: '2rem' }}>
                <Title level={4} style={{ color: '#10B981', marginBottom: '1rem' }}>🌐 DynaRoute v2 (Dynamic Routing)</Title>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', marginBottom: '1rem' }}>
                  <strong style={{ color: '#E8B44F' }}>What it actually is:</strong> Zero static ports. Services communicate using identity-based anycast addressing (IAAv6) instead of IP:PORT. Think of it like calling someone by name instead of phone number—the system figures out how to route the message dynamically.
                </Paragraph>
                <div style={{
                  background: 'rgba(0, 0, 0, 0.3)',
                  border: '1px solid rgba(16, 185, 129, 0.3)',
                  borderRadius: '8px',
                  padding: '1rem',
                  marginTop: '1rem',
                  fontFamily: 'monospace',
                  fontSize: '0.875rem',
                  color: '#10B981',
                  overflowX: 'auto'
                }}>
                  <div style={{ color: '#6B7280', marginBottom: '0.5rem' }}>// Traditional vs DynaRoute</div>
                  <div style={{ color: '#EF4444' }}>// ❌ Old way: client.connect("192.168.1.100:8080")</div>
                  <div style={{ marginTop: '0.5rem', color: '#10B981' }}>// ✅ DynaRoute: compute_iaav6("consensus", "cluster-ledger")</div>
                  <div style={{ marginTop: '0.5rem' }}><span style={{ color: '#7C3AED' }}>pub fn</span> <span style={{ color: '#E8B44F' }}>compute_iaav6</span>(service: &<span style={{ color: '#10B981' }}>str</span>) -{'>'} <span style={{ color: '#10B981' }}>Ipv6Addr</span> {'{'}</div>
                  <div style={{ paddingLeft: '1rem' }}><span style={{ color: '#7C3AED' }}>let</span> hash = blake3::hash(service);</div>
                  <div style={{ paddingLeft: '1rem' }}><span style={{ color: '#7C3AED' }}>return</span> ipv6_from_hash(hash); <span style={{ color: '#6B7280' }}>// No ports!</span></div>
                  <div>{'}'}</div>
                </div>
              </div>

              <div style={{ marginBottom: '2rem' }}>
                <Title level={4} style={{ color: '#10B981', marginBottom: '1rem' }}>🔐 Quantum-Ready Cryptography</Title>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', marginBottom: '1rem' }}>
                  <strong style={{ color: '#E8B44F' }}>What it actually is:</strong> Current cryptography (Ed25519, Blake3) with experimental post-quantum algorithms (Dilithium3/5) being integrated. "Quantum-ready" means we're preparing for quantum computers that could break current encryption.
                </Paragraph>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8' }}>
                  <strong>In code:</strong> Hybrid signature schemes, quantum entanglement proofs for transactions, and lattice-based cryptography experiments.
                </Paragraph>
              </div>

              <div style={{ marginBottom: '2rem' }}>
                <Title level={4} style={{ color: '#10B981', marginBottom: '1rem' }}>📦 vPods (Virtual Pods)</Title>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', marginBottom: '1rem' }}>
                  <strong style={{ color: '#E8B44F' }}>What it actually is:</strong> Lightweight virtual containers for services. Like Docker containers, but designed for dynamic networking and mesh communication. Each service runs in a vPod that can move, scale, and communicate without static addresses.
                </Paragraph>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8' }}>
                  <strong>In code:</strong> Rust-based vPod cluster coordinator with HRW (Highest Random Weight) allocation, resource sharing enforcement, and mesh integration.
                </Paragraph>
              </div>

              <div style={{ marginBottom: '2rem' }}>
                <Title level={4} style={{ color: '#10B981', marginBottom: '1rem' }}>📝 CBOR (Concise Binary Object Representation)</Title>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', marginBottom: '1rem' }}>
                  <strong style={{ color: '#E8B44F' }}>What it actually is:</strong> Binary data format (like JSON, but smaller and faster). We use it for government compliance pipelines where every transaction must have witness signatures, Merkle proofs, and audit metadata that cannot be hidden.
                </Paragraph>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8' }}>
                  <strong>In code:</strong> CBOR pipeline with immutable audit trails, forensic oracle integration, and VM client processing.
                </Paragraph>
              </div>

              <div style={{ marginBottom: '2rem' }}>
                <Title level={4} style={{ color: '#10B981', marginBottom: '1rem' }}>🎯 Mainnet vs Testnet</Title>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', marginBottom: '1rem' }}>
                  <strong style={{ color: '#E8B44F' }}>Testnet (current):</strong> 75% infrastructure operational with 15 backend services, real databases, and DynaRoute v2 networking. Used for pilot testing and validation.
                </Paragraph>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8' }}>
                  <strong style={{ color: '#E8B44F' }}>Mainnet (future):</strong> Triggered by traction, testing validation, and funding. Process: Achieve traction → Complete testing → Secure funding → Trigger GEN coin distribution → Launch mainnet. Code is ready; ecosystem needs to be ready.
                </Paragraph>
              </div>

              <div style={{
                background: 'rgba(124, 58, 237, 0.1)',
                border: '1px solid rgba(124, 58, 237, 0.3)',
                borderRadius: '8px',
                padding: '1.5rem'
              }}>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.6', marginBottom: '0' }}>
                  <strong style={{ color: '#7C3AED' }}>For Web3 builders:</strong> If you're looking for another EVM-compatible chain, this isn't it. If you're interested in experimental consensus mechanisms (LCCD), 6D transaction tracking, dynamic networking (DynaRoute), and enterprise-grade audit infrastructure (CBOR pipelines), let's talk.
                </Paragraph>
              </div>
            </div>
          </div>
        </section>
      )}

      {/* Section 5: Infrastructure Engineers */}
      {activeSection === 5 && (
        <section style={{ padding: '5rem 0', background: 'transparent' }}>
          <div style={{ maxWidth: '72rem', margin: '0 auto', padding: '0 2rem' }}>
            <Title level={2} style={{ fontSize: '2.5rem', fontWeight: 'bold', marginBottom: '2rem', color: '#E8B44F', textAlign: 'center' }}>
              ⚙️ For Infrastructure Engineers & Rust Developers: Deep Dive
            </Title>
            
            <div style={{
              background: 'rgba(10, 22, 40, 0.9)',
              border: '2px solid rgba(232, 180, 79, 0.3)',
              borderRadius: '12px',
              padding: '3rem',
              marginBottom: '2rem',
              backdropFilter: 'blur(10px)'
            }}>
              <Title level={3} style={{ color: '#E8B44F', marginBottom: '1.5rem' }}>Real Infrastructure Components (From Actual Code)</Title>
              
              <div style={{ marginBottom: '2rem' }}>
                <Title level={4} style={{ color: '#10B981', marginBottom: '1rem' }}>🏗️ 15 Backend Services (Operational)</Title>
                <Row gutter={[16, 16]}>
                  <Col xs={24} md={12}>
                    <ul style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.8', paddingLeft: '1.5rem', marginBottom: 0 }}>
                      <li><strong>api-gateway:</strong> REST/gRPC gateway</li>
                      <li><strong>auction-mempool:</strong> Transaction auction system</li>
                      <li><strong>blockchain:</strong> Core blockchain server</li>
                      <li><strong>consensus:</strong> LCCD consensus engine</li>
                      <li><strong>cluster-ledger:</strong> Distributed ledger</li>
                      <li><strong>network:</strong> P2P mesh networking</li>
                      <li><strong>shadow-registry:</strong> Service discovery</li>
                      <li><strong>bpi-bridge:</strong> BPI OS integration</li>
                    </ul>
                  </Col>
                  <Col xs={24} md={12}>
                    <ul style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.8', paddingLeft: '1.5rem', marginBottom: 0 }}>
                      <li><strong>bso-k8:</strong> Orchestration layer</li>
                      <li><strong>mojo:</strong> Wallet system</li>
                      <li><strong>web:</strong> Web layer (community installer)</li>
                      <li><strong>auction-db-maintainer:</strong> DB maintenance</li>
                      <li><strong>xtmp:</strong> High-speed protocol (10-20x faster than HTTP)</li>
                      <li><strong>central-orchestration:</strong> Service coordinator</li>
                      <li><strong>admin:</strong> Admin dashboard</li>
                    </ul>
                  </Col>
                </Row>
              </div>

              <div style={{ marginBottom: '2rem' }}>
                <Title level={4} style={{ color: '#10B981', marginBottom: '1rem' }}>💾 Database Infrastructure</Title>
                <ul style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', paddingLeft: '1.5rem' }}>
                  <li><strong>PostgreSQL:</strong> Primary relational DB (user accounts, transactions, audit trails)</li>
                  <li><strong>Redis:</strong> Cache layer (session management, hot data)</li>
                  <li><strong>MongoDB:</strong> Document store (4D hash-graph kernel, CBOR data)</li>
                  <li><strong>RabbitMQ:</strong> Message queue (inter-service communication)</li>
                  <li><strong>4D Hash-Graph:</strong> Custom storage with R, C, V, I coordinates (Row, Column, Value, Intent)</li>
                </ul>
              </div>

              <div style={{ marginBottom: '2rem' }}>
                <Title level={4} style={{ color: '#10B981', marginBottom: '1rem' }}>🌐 Networking Stack</Title>
                <ul style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', paddingLeft: '1.5rem', marginBottom: '1rem' }}>
                  <li><strong>DynaRoute v2:</strong> Zero static ports, identity-based anycast (IAAv6)</li>
                  <li><strong>CommuteLock:</strong> Lock-based inter-component communication</li>
                  <li><strong>P2P Mesh:</strong> 13-server mesh network for distributed communication</li>
                  <li><strong>vPods:</strong> Virtual pods with dynamic addressing and HRW allocation</li>
                  <li><strong>XTMP Protocol:</strong> High-speed communication (10-20x faster than HTTP)</li>
                </ul>
                <div style={{
                  background: 'rgba(0, 0, 0, 0.3)',
                  border: '1px solid rgba(16, 185, 129, 0.3)',
                  borderRadius: '8px',
                  padding: '1rem',
                  fontFamily: 'monospace',
                  fontSize: '0.75rem',
                  color: '#10B981',
                  overflowX: 'auto'
                }}>
                  <div style={{ color: '#6B7280', marginBottom: '0.5rem' }}>// Real CommuteLock implementation</div>
                  <div><span style={{ color: '#7C3AED' }}>pub struct</span> <span style={{ color: '#E8B44F' }}>CommuteLockRuntime</span> {'{'}</div>
                  <div style={{ paddingLeft: '1rem' }}>locks: HashMap{'<'}String, Arc{'<'}RwLock{'<'}LockState{'>'}{'>'}{'>'},</div>
                  <div style={{ paddingLeft: '1rem' }}>message_router: Arc{'<'}MessageRouter{'>'}, <span style={{ color: '#6B7280' }}>// Inter-service comm</span></div>
                  <div>{'}'}</div>
                  <div style={{ marginTop: '0.5rem' }}><span style={{ color: '#7C3AED' }}>impl</span> CommuteLockRuntime {'{'}</div>
                  <div style={{ paddingLeft: '1rem' }}><span style={{ color: '#7C3AED' }}>pub async fn</span> <span style={{ color: '#E8B44F' }}>send_message</span>(&<span style={{ color: '#7C3AED' }}>self</span>, target: &<span style={{ color: '#10B981' }}>str</span>, msg: Message) {'{'}</div>
                  <div style={{ paddingLeft: '2rem' }}><span style={{ color: '#7C3AED' }}>let</span> lock = <span style={{ color: '#7C3AED' }}>self</span>.acquire_lock(target).<span style={{ color: '#7C3AED' }}>await</span>?;</div>
                  <div style={{ paddingLeft: '2rem' }}><span style={{ color: '#7C3AED' }}>self</span>.message_router.route(msg).<span style={{ color: '#7C3AED' }}>await</span> <span style={{ color: '#6B7280' }}>// Lock-based routing</span></div>
                  <div style={{ paddingLeft: '1rem' }}>{'}'}</div>
                  <div>{'}'}</div>
                </div>
              </div>

              <div style={{ marginBottom: '2rem' }}>
                <Title level={4} style={{ color: '#10B981', marginBottom: '1rem' }}>🔐 Security & Cryptography</Title>
                <ul style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', paddingLeft: '1.5rem' }}>
                  <li><strong>Keycloak:</strong> OAuth2/OIDC authentication (production-ready)</li>
                  <li><strong>Ed25519:</strong> Current signature algorithm</li>
                  <li><strong>Blake3:</strong> Hashing (faster than SHA-256)</li>
                  <li><strong>Dilithium3/5:</strong> Experimental post-quantum signatures</li>
                  <li><strong>BLS Signatures:</strong> Signature aggregation for consensus</li>
                  <li><strong>Quantum Entanglement Proofs:</strong> Transaction validation with quantum-inspired proofs</li>
                </ul>
              </div>

              <div style={{ marginBottom: '2rem' }}>
                <Title level={4} style={{ color: '#10B981', marginBottom: '1rem' }}>🧬 Advanced Systems (Real Implementations)</Title>
                <ul style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', paddingLeft: '1.5rem', marginBottom: '1rem' }}>
                  <li><strong>LCCD Consensus:</strong> Living Cellular Consensus Division with cellular division logic</li>
                  <li><strong>6D Blockchain:</strong> Cuboidal geometry with R, C, V, I, T, P dimensions</li>
                  <li><strong>Forensic Oracle:</strong> Immutable audit system with impossible-to-hide trails</li>
                  <li><strong>Mutual Living Enforcer:</strong> Compulsory resource sharing for BPI OS nodes</li>
                  <li><strong>CBOR Pipeline:</strong> Government compliance with witness signatures and Merkle proofs</li>
                  <li><strong>ENCCluster VM:</strong> Virtual machine for smart contract execution</li>
                  <li><strong>Quantum Heartbeat:</strong> Ultra-compressed proof-of-life system (3 years in 1GB)</li>
                </ul>
                <div style={{
                  background: 'rgba(0, 0, 0, 0.3)',
                  border: '1px solid rgba(16, 185, 129, 0.3)',
                  borderRadius: '8px',
                  padding: '1rem',
                  fontFamily: 'monospace',
                  fontSize: '0.75rem',
                  color: '#10B981',
                  overflowX: 'auto'
                }}>
                  <div style={{ color: '#6B7280', marginBottom: '0.5rem' }}>// LCCD Consensus: Cellular Division Logic</div>
                  <div><span style={{ color: '#7C3AED' }}>async fn</span> <span style={{ color: '#E8B44F' }}>check_cellular_division</span>(&<span style={{ color: '#7C3AED' }}>mut self</span>) -{'>'} <span style={{ color: '#10B981' }}>Result</span>{'<'}(){'>'} {'{'}</div>
                  <div style={{ paddingLeft: '1rem' }}><span style={{ color: '#7C3AED' }}>if</span> <span style={{ color: '#7C3AED' }}>self</span>.cluster_load {'>'} <span style={{ color: '#7C3AED' }}>self</span>.division_threshold {'{'}</div>
                  <div style={{ paddingLeft: '2rem' }}><span style={{ color: '#6B7280' }}>// Load {'>'} 80%: Divide like a cell</span></div>
                  <div style={{ paddingLeft: '2rem' }}><span style={{ color: '#7C3AED' }}>let</span> (cluster_a, cluster_b) = <span style={{ color: '#7C3AED' }}>self</span>.divide_cluster().<span style={{ color: '#7C3AED' }}>await</span>?;</div>
                  <div style={{ paddingLeft: '2rem' }}><span style={{ color: '#7C3AED' }}>self</span>.sub_clusters.push(cluster_a);</div>
                  <div style={{ paddingLeft: '2rem' }}><span style={{ color: '#7C3AED' }}>self</span>.sub_clusters.push(cluster_b);</div>
                  <div style={{ paddingLeft: '1rem' }}>{'}'} <span style={{ color: '#7C3AED' }}>else if</span> <span style={{ color: '#7C3AED' }}>self</span>.cluster_load {'<'} <span style={{ color: '#7C3AED' }}>self</span>.merge_threshold {'{'}</div>
                  <div style={{ paddingLeft: '2rem' }}><span style={{ color: '#6B7280' }}>// Load {'<'} 30%: Merge clusters back</span></div>
                  <div style={{ paddingLeft: '2rem' }}><span style={{ color: '#7C3AED' }}>self</span>.merge_clusters().<span style={{ color: '#7C3AED' }}>await</span>?;</div>
                  <div style={{ paddingLeft: '1rem' }}>{'}'}</div>
                  <div>{'}'}</div>
                  <div style={{ marginTop: '0.5rem', color: '#6B7280' }}>// Bio-inspired: grows/shrinks like living organism</div>
                </div>
              </div>

              <div style={{ marginBottom: '2rem' }}>
                <Title level={4} style={{ color: '#F59E0B', marginBottom: '1rem' }}>⚠️ What Needs Work (25%)</Title>
                <ul style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', paddingLeft: '1.5rem' }}>
                  <li><strong>Load Testing:</strong> Integration tests exist, but need real-world stress testing</li>
                  <li><strong>Security Audit:</strong> No external audit yet (critical for production)</li>
                  <li><strong>Performance Profiling:</strong> Not optimized for high throughput</li>
                  <li><strong>Observability:</strong> Basic logging, needs Prometheus/Grafana/Jaeger</li>
                  <li><strong>Documentation:</strong> Code documented, but architecture docs incomplete</li>
                  <li><strong>Testnet → Mainnet:</strong> Needs pilot validation before mainnet trigger</li>
                </ul>
              </div>

              <div style={{ marginBottom: '2rem' }}>
                <Title level={4} style={{ color: '#10B981', marginBottom: '1rem' }}>🤝 Collaboration Opportunities</Title>
                <Paragraph style={{ color: '#ffffff', fontSize: '1.125rem', lineHeight: '1.8', marginBottom: '1rem' }}>
                  <strong style={{ color: '#E8B44F' }}>For Rust engineers:</strong> This is a large Rust codebase (distributed systems, async networking, cryptography). Areas for collaboration:
                </Paragraph>
                <ul style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', paddingLeft: '1.5rem' }}>
                  <li><strong>Consensus:</strong> LCCD cellular division logic, BLS aggregation</li>
                  <li><strong>Networking:</strong> DynaRoute v2, vPod allocation, mesh communication</li>
                  <li><strong>Cryptography:</strong> Post-quantum integration (Dilithium, SPHINCS+)</li>
                  <li><strong>Storage:</strong> 4D hash-graph optimization, MongoDB integration</li>
                  <li><strong>Performance:</strong> Tokio runtime optimization, async profiling</li>
                  <li><strong>Security:</strong> Audit preparation, penetration testing</li>
                  <li><strong>Observability:</strong> Metrics, tracing, distributed logging</li>
                </ul>
              </div>

              <div style={{
                background: 'rgba(239, 68, 68, 0.1)',
                border: '1px solid rgba(239, 68, 68, 0.3)',
                borderRadius: '8px',
                padding: '1.5rem'
              }}>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.6', marginBottom: '0' }}>
                  <strong style={{ color: '#EF4444' }}>Honest assessment:</strong> This is a single-engineer project with 75% infrastructure complete. The code is real and operational (15 services, 4 databases, dynamic networking), but it needs external eyes, security audits, and production validation. If you're looking for a polished, documented, enterprise-ready system, this isn't it yet. If you're interested in research-grade infrastructure with real Rust code that needs hardening, let's collaborate.
                </Paragraph>
              </div>
            </div>
          </div>
        </section>
      )}

      {/* Call to Action */}
      <section style={{ padding: '5rem 0', background: 'transparent' }}>
        <div style={{ maxWidth: '64rem', margin: '0 auto', padding: '0 2rem', textAlign: 'center' }}>
          <Title level={2} style={{ fontSize: '2.5rem', fontWeight: 'bold', marginBottom: '2rem', color: '#E8B44F' }}>
            Interested in Collaborating?
          </Title>
          <Paragraph style={{ fontSize: '1.25rem', color: '#ffffff', lineHeight: '1.8', marginBottom: '2rem', maxWidth: '48rem', margin: '0 auto 2rem auto' }}>
            Whether you're a business evaluating infrastructure, a developer exploring the SDK, or an engineer interested in the codebase—we're seeking pilot partners and collaborators.
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

export default Technology;
