import React, { useState, useEffect } from 'react';
import { Typography, Button, Row, Col, Collapse, Modal, Input, Rate, message } from 'antd';
import { getVoteStats, submitVote } from '../../services/voteService';
import './Community.css';

const { Title, Paragraph } = Typography;
const { Panel } = Collapse;
const { TextArea } = Input;

const Community: React.FC = () => {
  const [activeSection, setActiveSection] = useState<number>(0);
  const [voteModalVisible, setVoteModalVisible] = useState(false);
  const [voteEmail, setVoteEmail] = useState('');
  const [voteComment, setVoteComment] = useState('');
  const [voteRating, setVoteRating] = useState(0);
  const [voteStats, setVoteStats] = useState({ total: 0, avgRating: 0 }); // Real data from DB
  const [loading, setLoading] = useState(false);
  
  // Calculate days since project launch (October 1, 2024)
  const projectLaunchDate = new Date('2024-10-01');
  const today = new Date();
  const daysLive = Math.floor((today.getTime() - projectLaunchDate.getTime()) / (1000 * 60 * 60 * 24));

  // Load vote statistics from database on component mount
  useEffect(() => {
    loadVoteStats();
  }, []);

  const loadVoteStats = async () => {
    try {
      const stats = await getVoteStats();
      setVoteStats(stats);
    } catch (error) {
      console.error('Failed to load vote stats:', error);
    }
  };

  const handleVoteSubmit = async () => {
    if (!voteEmail || !voteEmail.includes('@')) {
      message.error('Please enter a valid email address');
      return;
    }
    if (voteRating === 0) {
      message.error('Please select a rating');
      return;
    }

    setLoading(true);

    try {
      // Submit vote to backend API
      const result = await submitVote({
        email: voteEmail,
        comment: voteComment,
        rating: voteRating,
        timestamp: new Date().toISOString()
      });

      // Update stats with response from server
      setVoteStats(result.stats);

      message.success('Thank you for your vote! Your feedback has been recorded.');
      setVoteModalVisible(false);
      setVoteEmail('');
      setVoteComment('');
      setVoteRating(0);
    } catch (error) {
      message.error('Failed to submit vote. Please try again.');
      console.error('Vote submission error:', error);
    } finally {
      setLoading(false);
    }
  };

  const sections = [
    { id: 0, label: 'Overview', emoji: '🏠' },
    { id: 1, label: 'Who This Is For', emoji: '👥' },
    { id: 2, label: 'How to Contribute', emoji: '🤝' },
    { id: 3, label: 'Resources', emoji: '📚' },
    { id: 4, label: 'FAQ', emoji: '❓' }
  ];

  return (
    <div className="community-page">
      {/* Vote Statistics Banner */}
      <div style={{ 
        background: 'linear-gradient(135deg, rgba(232, 180, 79, 0.1) 0%, rgba(16, 185, 129, 0.1) 100%)',
        borderBottom: '2px solid rgba(232, 180, 79, 0.3)',
        padding: '1rem 0'
      }}>
        <div style={{ maxWidth: '72rem', margin: '0 auto', padding: '0 2rem', display: 'flex', justifyContent: 'center', alignItems: 'center', gap: '2rem', flexWrap: 'wrap' }}>
          <div style={{ textAlign: 'center' }}>
            <div style={{ color: '#7C3AED', fontSize: '2rem', fontWeight: 'bold' }}>{daysLive}</div>
            <div style={{ color: '#ffffff', fontSize: '0.875rem' }}>Days Live</div>
          </div>
          <div style={{ textAlign: 'center' }}>
            <div style={{ color: '#E8B44F', fontSize: '2rem', fontWeight: 'bold' }}>{voteStats.total}</div>
            <div style={{ color: '#ffffff', fontSize: '0.875rem' }}>Total Votes</div>
          </div>
          <div style={{ textAlign: 'center' }}>
            <div style={{ color: '#10B981', fontSize: '2rem', fontWeight: 'bold' }}>
              {voteStats.total === 0 ? '0.0' : voteStats.avgRating.toFixed(1)} ⭐
            </div>
            <div style={{ color: '#ffffff', fontSize: '0.875rem' }}>Average Rating</div>
          </div>
          <Button 
            type="primary"
            size="large"
            style={{
              background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)',
              border: 'none',
              color: '#0A1628',
              fontWeight: '600'
            }}
            onClick={() => setVoteModalVisible(true)}
          >
            👍 Vote Now
          </Button>
        </div>
      </div>

      {/* Vote Modal */}
      <Modal
        title={<span style={{ fontSize: '1.5rem', fontWeight: 'bold', color: '#E8B44F' }}>Vote for This Research</span>}
        open={voteModalVisible}
        onOk={handleVoteSubmit}
        onCancel={() => {
          setVoteModalVisible(false);
          setVoteEmail('');
          setVoteComment('');
          setVoteRating(0);
        }}
        okText="Submit Vote"
        cancelText="Cancel"
        width={600}
        confirmLoading={loading}
        okButtonProps={{
          style: {
            background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)',
            border: 'none',
            color: '#0A1628',
            fontWeight: '600'
          }
        }}
      >
        <div style={{ padding: '1rem 0' }}>
          <div style={{ marginBottom: '1.5rem' }}>
            <label style={{ display: 'block', marginBottom: '0.5rem', fontWeight: '600', color: '#E8B44F' }}>
              Email Address *
            </label>
            <Input
              type="email"
              placeholder="your.email@example.com"
              value={voteEmail}
              onChange={(e) => setVoteEmail(e.target.value)}
              size="large"
              required
            />
          </div>

          <div style={{ marginBottom: '1.5rem' }}>
            <label style={{ display: 'block', marginBottom: '0.5rem', fontWeight: '600', color: '#E8B44F' }}>
              Rating *
            </label>
            <Rate
              value={voteRating}
              onChange={setVoteRating}
              style={{ fontSize: '2rem' }}
            />
            <div style={{ marginTop: '0.5rem', fontSize: '0.875rem', color: '#666' }}>
              {voteRating === 0 && 'Select a rating (1-5 stars)'}
              {voteRating === 1 && '⭐ Poor'}
              {voteRating === 2 && '⭐⭐ Fair'}
              {voteRating === 3 && '⭐⭐⭐ Good'}
              {voteRating === 4 && '⭐⭐⭐⭐ Very Good'}
              {voteRating === 5 && '⭐⭐⭐⭐⭐ Excellent'}
            </div>
          </div>

          <div style={{ marginBottom: '1rem' }}>
            <label style={{ display: 'block', marginBottom: '0.5rem', fontWeight: '600', color: '#E8B44F' }}>
              Comment (Optional)
            </label>
            <TextArea
              placeholder="Share your thoughts about this research project..."
              value={voteComment}
              onChange={(e) => setVoteComment(e.target.value)}
              rows={4}
              maxLength={500}
              showCount
            />
          </div>

          <div style={{ padding: '1rem', background: 'rgba(232, 180, 79, 0.1)', borderRadius: '8px', fontSize: '0.875rem', color: '#666' }}>
            <strong>Note:</strong> Your vote and feedback will be stored in our database for review. We use this information to improve the project and understand community interest.
          </div>
        </div>
      </Modal>

      {/* Hero Section */}
      <section className="hero-gradient" style={{ padding: '8rem 0 6rem 0', textAlign: 'center' }}>
        <div style={{ maxWidth: '72rem', margin: '0 auto', padding: '0 2rem' }}>
          <Title level={1} style={{ color: '#ffffff', fontSize: '3rem', fontWeight: 'bold', marginBottom: '1.5rem' }}>
            Community & Open Collaboration
          </Title>
          <Paragraph style={{ color: '#ffffff', fontSize: '1.25rem', maxWidth: '48rem', margin: '0 auto 2rem auto', lineHeight: '1.8' }}>
            Open-source infrastructure built by a community of contributors. Join us to learn, build, and shape the future.
          </Paragraph>
          
          {/* Section Selector */}
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
              🏠 Welcome to the Community
            </Title>
            
            <div style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(232, 180, 79, 0.3)', borderRadius: '12px', padding: '3rem', marginBottom: '2rem', backdropFilter: 'blur(10px)' }}>
              <Paragraph style={{ color: '#ffffff', fontSize: '1.25rem', lineHeight: '1.8', marginBottom: '2rem' }}>
                <strong style={{ color: '#E8B44F' }}>This is an open-source project.</strong> The infrastructure (75% complete) is built collaboratively. Whether you're a developer, researcher, student, or enthusiast—there's a place for you here.
              </Paragraph>

              <Title level={3} style={{ color: '#E8B44F', marginBottom: '1rem' }}>What Makes This Community Different</Title>
              <Row gutter={[24, 24]} style={{ marginBottom: '2rem' }}>
                <Col xs={24} md={8}>
                  <div style={{ padding: '1.5rem', background: 'rgba(16, 185, 129, 0.1)', borderRadius: '8px', border: '1px solid rgba(16, 185, 129, 0.3)' }}>
                    <Title level={4} style={{ color: '#10B981', marginBottom: '0.5rem' }}>🎓 Learn by Building</Title>
                    <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', marginBottom: 0 }}>
                      Real infrastructure to learn from. Not tutorials—actual production code (Rust, distributed systems, cryptography).
                    </Paragraph>
                  </div>
                </Col>
                <Col xs={24} md={8}>
                  <div style={{ padding: '1.5rem', background: 'rgba(124, 58, 237, 0.1)', borderRadius: '8px', border: '1px solid rgba(124, 58, 237, 0.3)' }}>
                    <Title level={4} style={{ color: '#7C3AED', marginBottom: '0.5rem' }}>🤝 Collaborative</Title>
                    <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', marginBottom: 0 }}>
                      Not a corporate project. Community-driven development. Your contributions shape the platform.
                    </Paragraph>
                  </div>
                </Col>
                <Col xs={24} md={8}>
                  <div style={{ padding: '1.5rem', background: 'rgba(245, 158, 11, 0.1)', borderRadius: '8px', border: '1px solid rgba(245, 158, 11, 0.3)' }}>
                    <Title level={4} style={{ color: '#F59E0B', marginBottom: '0.5rem' }}>🚀 Early Stage</Title>
                    <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', marginBottom: 0 }}>
                      75% complete = your contribution matters. Not too early (vaporware) or too late (already built).
                    </Paragraph>
                  </div>
                </Col>
              </Row>

              <Title level={3} style={{ color: '#E8B44F', marginBottom: '1rem' }}>Current Status</Title>
              <ul style={{ color: '#ffffff', fontSize: '1.125rem', lineHeight: '1.8', paddingLeft: '2rem', marginBottom: '2rem' }}>
                <li><strong>Infrastructure:</strong> 15 backend services operational (Rust)</li>
                <li><strong>Codebase:</strong> Open-source on GitHub (contributions welcome)</li>
                <li><strong>Community:</strong> Growing (developers, researchers, students)</li>
                <li><strong>Phase:</strong> Testnet operational, seeking contributors for mainnet readiness</li>
              </ul>

              <div style={{ background: 'rgba(232, 180, 79, 0.1)', border: '1px solid rgba(232, 180, 79, 0.3)', borderRadius: '8px', padding: '1.5rem', textAlign: 'center' }}>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', marginBottom: '1.5rem' }}>
                  <strong style={{ color: '#E8B44F' }}>Select a section above to learn more:</strong> Who this is for → How to contribute → Resources → FAQ
                </Paragraph>
                
                {/* Action Buttons */}
                <div style={{ display: 'flex', gap: '1rem', justifyContent: 'center', flexWrap: 'wrap' }}>
                  <Button 
                    type="primary" 
                    size="large"
                    icon={<span style={{ marginRight: '0.5rem' }}>💻</span>}
                    style={{
                      background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)',
                      border: 'none',
                      color: '#0A1628',
                      fontWeight: '600',
                      height: '48px',
                      padding: '0 2rem',
                      fontSize: '1rem'
                    }}
                    onClick={() => window.open('https://github.com/GlobalSushrut/PARVYOM-metanode', '_blank')}
                  >
                    View on GitHub
                  </Button>
                  <Button 
                    size="large"
                    icon={<span style={{ marginRight: '0.5rem' }}>👍</span>}
                    style={{
                      background: 'transparent',
                      border: '2px solid #E8B44F',
                      color: '#E8B44F',
                      fontWeight: '600',
                      height: '48px',
                      padding: '0 2rem',
                      fontSize: '1rem'
                    }}
                    onClick={() => setVoteModalVisible(true)}
                  >
                    Vote for This Research
                  </Button>
                </div>
              </div>
            </div>
          </div>
        </section>
      )}

      {/* Section 1: Who This Is For */}
      {activeSection === 1 && (
        <section style={{ padding: '5rem 0' }}>
          <div style={{ maxWidth: '72rem', margin: '0 auto', padding: '0 2rem' }}>
            <Title level={2} style={{ fontSize: '2.5rem', fontWeight: 'bold', marginBottom: '2rem', color: '#10B981', textAlign: 'center' }}>
              👥 Who This Is For (Honest Complexity Assessment)
            </Title>

            {/* Complexity Warning */}
            <div style={{ background: 'rgba(239, 68, 68, 0.1)', border: '2px solid rgba(239, 68, 68, 0.3)', borderRadius: '12px', padding: '2rem', marginBottom: '2rem', backdropFilter: 'blur(10px)' }}>
              <Title level={3} style={{ color: '#EF4444', marginBottom: '1rem' }}>⚠️ Understanding the Complexity</Title>
              <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', marginBottom: '1rem' }}>
                <strong style={{ color: '#EF4444' }}>This is not a simple project.</strong> The infrastructure crosses multiple complex domains: distributed systems theory, post-quantum cryptography, category theory, consensus mechanisms, operating systems design, and metaphysical computing models. Take time to understand the scope before diving in.
              </Paragraph>
              <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', marginBottom: 0 }}>
                <strong style={{ color: '#E8B44F' }}>Cross-domain complexity:</strong> Mathematical (category theory, graph theory), Cryptographic (post-quantum, BLS signatures), Systems (distributed OS, consensus), Metaphysical (6D blockchain, quantum-inspired models). Choose your entry point based on your expertise and interests.
              </Paragraph>
            </div>

            <Row gutter={[24, 24]}>
              {/* OS & Infrastructure Builders */}
              <Col xs={24} md={12}>
                <div style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(16, 185, 129, 0.3)', borderRadius: '12px', padding: '2rem', height: '100%', backdropFilter: 'blur(10px)' }}>
                  <Title level={3} style={{ color: '#10B981', marginBottom: '1rem' }}>🖥️ OS & Infrastructure Builders</Title>
                  <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', marginBottom: '1rem' }}>
                    <strong style={{ color: '#E8B44F' }}>Start here if you have:</strong> Deep systems programming (Rust, C++), OS internals knowledge, distributed systems experience
                  </Paragraph>
                  <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', marginBottom: '1rem' }}>
                    <strong style={{ color: '#E8B44F' }}>Work on:</strong> BPI OS layer, DynaRoute v2 networking, vPod orchestration, CommuteLock communication, 15 backend services
                  </Paragraph>
                  <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', marginBottom: '1rem' }}>
                    <strong style={{ color: '#E8B44F' }}>Complexity:</strong> High. Requires understanding async Rust, tokio runtime, distributed OS concepts, dynamic networking
                  </Paragraph>
                  <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', marginBottom: 0 }}>
                    <strong style={{ color: '#E8B44F' }}>Good to know:</strong> This isn't Docker or Kubernetes. It's a distributed OS with dynamic port allocation, mesh networking, and cellular division. Study the architecture first.
                  </Paragraph>
                </div>
              </Col>

              {/* Consensus & Cryptography Researchers */}
              <Col xs={24} md={12}>
                <div style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(16, 185, 129, 0.3)', borderRadius: '12px', padding: '2rem', height: '100%', backdropFilter: 'blur(10px)' }}>
                  <Title level={3} style={{ color: '#10B981', marginBottom: '1rem' }}>🔬 Consensus & Crypto Researchers</Title>
                  <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', marginBottom: '1rem' }}>
                    <strong style={{ color: '#E8B44F' }}>Start here if you have:</strong> PhD-level cryptography, consensus mechanism research, formal verification background
                  </Paragraph>
                  <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', marginBottom: '1rem' }}>
                    <strong style={{ color: '#E8B44F' }}>Work on:</strong> LCCD consensus (Living Cellular Consensus Division), post-quantum cryptography integration, BLS signature aggregation, quantum entanglement proofs
                  </Paragraph>
                  <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', marginBottom: '1rem' }}>
                    <strong style={{ color: '#E8B44F' }}>Complexity:</strong> Extreme. Bio-inspired consensus that divides like cells, post-quantum crypto (Dilithium), category theory foundations
                  </Paragraph>
                  <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', marginBottom: 0 }}>
                    <strong style={{ color: '#E8B44F' }}>Good to know:</strong> LCCD is not PoW/PoS. It's cellular division-based. Quantum proofs are not blockchain hashes. Study the papers first.
                  </Paragraph>
                </div>
              </Col>

              {/* Auditors & Security Researchers */}
              <Col xs={24} md={12}>
                <div style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(16, 185, 129, 0.3)', borderRadius: '12px', padding: '2rem', height: '100%', backdropFilter: 'blur(10px)' }}>
                  <Title level={3} style={{ color: '#10B981', marginBottom: '1rem' }}>🔐 Security Auditors & Researchers</Title>
                  <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', marginBottom: '1rem' }}>
                    <strong style={{ color: '#E8B44F' }}>Start here if you have:</strong> Security audit experience, cryptographic analysis, formal verification, penetration testing
                  </Paragraph>
                  <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', marginBottom: '1rem' }}>
                    <strong style={{ color: '#E8B44F' }}>Work on:</strong> Security audits (critical need), cryptographic validation, attack surface analysis, formal verification of consensus, penetration testing
                  </Paragraph>
                  <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', marginBottom: '1rem' }}>
                    <strong style={{ color: '#E8B44F' }}>Complexity:</strong> Very High. Multiple attack surfaces: consensus, networking, cryptography, OS layer, 15 services
                  </Paragraph>
                  <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', marginBottom: 0 }}>
                    <strong style={{ color: '#E8B44F' }}>Good to know:</strong> This is experimental research code (75% complete). Needs external validation. Standard blockchain security models may not apply.
                  </Paragraph>
                </div>
              </Col>

              {/* Mathematical & Theoretical Researchers */}
              <Col xs={24} md={12}>
                <div style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(16, 185, 129, 0.3)', borderRadius: '12px', padding: '2rem', height: '100%', backdropFilter: 'blur(10px)' }}>
                  <Title level={3} style={{ color: '#10B981', marginBottom: '1rem' }}>📐 Mathematical & Theoretical Researchers</Title>
                  <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', marginBottom: '1rem' }}>
                    <strong style={{ color: '#E8B44F' }}>Start here if you have:</strong> Category theory, graph theory, topology, formal methods, mathematical modeling
                  </Paragraph>
                  <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', marginBottom: '1rem' }}>
                    <strong style={{ color: '#E8B44F' }}>Work on:</strong> 6D blockchain mathematical foundations, category theory models, 4D hash-graph geometry, quantum-inspired models, formal proofs
                  </Paragraph>
                  <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', marginBottom: '1rem' }}>
                    <strong style={{ color: '#E8B44F' }}>Complexity:</strong> Extreme. Metaphysical computing models, 6D transaction space (sender, receiver, amount, time, proof, intent), cuboidal geometry
                  </Paragraph>
                  <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', marginBottom: 0 }}>
                    <strong style={{ color: '#E8B44F' }}>Good to know:</strong> 6D blockchain is not marketing. It's actual dimensional modeling. Quantum entanglement proofs are mathematical constructs. Study the foundations.
                  </Paragraph>
                </div>
              </Col>

              {/* Documentation & Learning Contributors */}
              <Col xs={24} md={12}>
                <div style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(16, 185, 129, 0.3)', borderRadius: '12px', padding: '2rem', height: '100%', backdropFilter: 'blur(10px)' }}>
                  <Title level={3} style={{ color: '#10B981', marginBottom: '1rem' }}>📚 Documentation & Learning Contributors</Title>
                  <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', marginBottom: '1rem' }}>
                    <strong style={{ color: '#E8B44F' }}>Start here if you:</strong> Want to learn by documenting, explain complex concepts, create educational content
                  </Paragraph>
                  <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', marginBottom: '1rem' }}>
                    <strong style={{ color: '#E8B44F' }}>Work on:</strong> Architecture documentation, tutorials, concept explanations, video guides, translating complex ideas
                  </Paragraph>
                  <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', marginBottom: '1rem' }}>
                    <strong style={{ color: '#E8B44F' }}>Complexity:</strong> Medium. Requires understanding concepts deeply enough to explain them clearly
                  </Paragraph>
                  <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', marginBottom: 0 }}>
                    <strong style={{ color: '#E8B44F' }}>Good to know:</strong> This isn't "blockchain for dummies." Document the actual complexity honestly. Help others understand what they're getting into.
                  </Paragraph>
                </div>
              </Col>

              {/* Testing & Validation Contributors */}
              <Col xs={24} md={12}>
                <div style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(16, 185, 129, 0.3)', borderRadius: '12px', padding: '2rem', height: '100%', backdropFilter: 'blur(10px)' }}>
                  <Title level={3} style={{ color: '#10B981', marginBottom: '1rem' }}>🧪 Testing & Validation Contributors</Title>
                  <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', marginBottom: '1rem' }}>
                    <strong style={{ color: '#E8B44F' }}>Start here if you:</strong> Have testing experience, QA background, want to validate experimental systems
                  </Paragraph>
                  <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', marginBottom: '1rem' }}>
                    <strong style={{ color: '#E8B44F' }}>Work on:</strong> Integration testing, load testing, bug reports, edge case discovery, testnet validation
                  </Paragraph>
                  <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', marginBottom: '1rem' }}>
                    <strong style={{ color: '#E8B44F' }}>Complexity:</strong> Medium-High. Need to understand system architecture to test effectively
                  </Paragraph>
                  <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', marginBottom: 0 }}>
                    <strong style={{ color: '#E8B44F' }}>Good to know:</strong> This is experimental research code. Expect bugs, incomplete features, rough edges. Your job is to find them and report them clearly.
                  </Paragraph>
                </div>
              </Col>
            </Row>

            <div style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(232, 180, 79, 0.3)', borderRadius: '12px', padding: '2rem', marginTop: '2rem', backdropFilter: 'blur(10px)' }}>
              <Title level={3} style={{ color: '#E8B44F', marginBottom: '1rem', textAlign: 'center' }}>Still Not Sure Where You Fit?</Title>
              <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', marginBottom: '1rem', textAlign: 'center' }}>
                <strong style={{ color: '#E8B44F' }}>Don't guess.</strong> Contact <strong>umesh@pravyom.com</strong> with your background and interests. We'll help you find the right entry point based on your actual expertise, not what you think you can handle.
              </Paragraph>
              <Paragraph style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', marginBottom: 0, textAlign: 'center', fontStyle: 'italic' }}>
                Remember: This is experimental research infrastructure crossing multiple complex domains. Honesty about your expertise level helps everyone—including you.
              </Paragraph>
            </div>
          </div>
        </section>
      )}

      {/* Section 2: How to Contribute */}
      {activeSection === 2 && (
        <section style={{ padding: '5rem 0' }}>
          <div style={{ maxWidth: '72rem', margin: '0 auto', padding: '0 2rem' }}>
            <Title level={2} style={{ fontSize: '2.5rem', fontWeight: 'bold', marginBottom: '2rem', color: '#3B82F6', textAlign: 'center' }}>
              🤝 How to Contribute
            </Title>

            <div style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(59, 130, 246, 0.3)', borderRadius: '12px', padding: '3rem', marginBottom: '2rem', backdropFilter: 'blur(10px)' }}>
              <Title level={3} style={{ color: '#3B82F6', marginBottom: '1.5rem' }}>Step-by-Step Contribution Guide</Title>
              
              <div style={{ marginBottom: '2rem' }}>
                <Title level={4} style={{ color: '#E8B44F', marginBottom: '1rem' }}>1. Explore the Codebase</Title>
                <ul style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', paddingLeft: '1.5rem' }}>
                  <li><strong>GitHub:</strong> Browse the repository, read README files</li>
                  <li><strong>Documentation:</strong> Understand the architecture</li>
                  <li><strong>Issues:</strong> Check open issues tagged "good first issue"</li>
                </ul>
              </div>

              <div style={{ marginBottom: '2rem' }}>
                <Title level={4} style={{ color: '#E8B44F', marginBottom: '1rem' }}>2. Set Up Development Environment</Title>
                <ul style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', paddingLeft: '1.5rem' }}>
                  <li><strong>Install Rust:</strong> Follow setup guide in docs</li>
                  <li><strong>Clone repo:</strong> Fork and clone the repository</li>
                  <li><strong>Build locally:</strong> Compile and run tests</li>
                </ul>
              </div>

              <div style={{ marginBottom: '2rem' }}>
                <Title level={4} style={{ color: '#E8B44F', marginBottom: '1rem' }}>3. Make Your Contribution</Title>
                <ul style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', paddingLeft: '1.5rem' }}>
                  <li><strong>Pick an issue:</strong> Start with small, well-defined tasks</li>
                  <li><strong>Create branch:</strong> Work on a feature branch</li>
                  <li><strong>Write tests:</strong> Ensure code quality</li>
                  <li><strong>Submit PR:</strong> Create pull request with clear description</li>
                </ul>
              </div>

              <div style={{ marginBottom: '2rem' }}>
                <Title level={4} style={{ color: '#E8B44F', marginBottom: '1rem' }}>4. Get Feedback & Iterate</Title>
                <ul style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', paddingLeft: '1.5rem' }}>
                  <li><strong>Code review:</strong> Maintainers will review your PR</li>
                  <li><strong>Address feedback:</strong> Make requested changes</li>
                  <li><strong>Merge:</strong> Once approved, your code is merged!</li>
                </ul>
              </div>

              <div style={{ background: 'rgba(59, 130, 246, 0.1)', border: '1px solid rgba(59, 130, 246, 0.3)', borderRadius: '8px', padding: '1.5rem' }}>
                <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.6', marginBottom: 0 }}>
                  <strong style={{ color: '#3B82F6' }}>Need help?</strong> Don't hesitate to ask questions in GitHub issues or contact <strong>umesh@pravyom.com</strong>. We're here to help!
                </Paragraph>
              </div>
            </div>

            <div style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(59, 130, 246, 0.3)', borderRadius: '12px', padding: '3rem', backdropFilter: 'blur(10px)' }}>
              <Title level={3} style={{ color: '#3B82F6', marginBottom: '1.5rem' }}>Non-Code Contributions</Title>
              <Row gutter={[24, 24]}>
                <Col xs={24} md={12}>
                  <Title level={4} style={{ color: '#E8B44F', marginBottom: '1rem' }}>📚 Documentation</Title>
                  <ul style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', paddingLeft: '1.5rem', marginBottom: 0 }}>
                    <li>Write tutorials</li>
                    <li>Improve existing docs</li>
                    <li>Create video guides</li>
                    <li>Translate documentation</li>
                  </ul>
                </Col>
                <Col xs={24} md={12}>
                  <Title level={4} style={{ color: '#E8B44F', marginBottom: '1rem' }}>🐛 Testing & Bug Reports</Title>
                  <ul style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', paddingLeft: '1.5rem', marginBottom: 0 }}>
                    <li>Test new features</li>
                    <li>Report bugs</li>
                    <li>Suggest improvements</li>
                    <li>Validate fixes</li>
                  </ul>
                </Col>
                <Col xs={24} md={12}>
                  <Title level={4} style={{ color: '#E8B44F', marginBottom: '1rem' }}>🎨 Design & UX</Title>
                  <ul style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', paddingLeft: '1.5rem', marginBottom: 0 }}>
                    <li>UI/UX improvements</li>
                    <li>Logo and branding</li>
                    <li>Website design</li>
                    <li>User experience feedback</li>
                  </ul>
                </Col>
                <Col xs={24} md={12}>
                  <Title level={4} style={{ color: '#E8B44F', marginBottom: '1rem' }}>🌐 Community Building</Title>
                  <ul style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6', paddingLeft: '1.5rem', marginBottom: 0 }}>
                    <li>Answer questions</li>
                    <li>Organize events</li>
                    <li>Social media</li>
                    <li>Community moderation</li>
                  </ul>
                </Col>
              </Row>
            </div>
          </div>
        </section>
      )}

      {/* Section 3: Community Resources */}
      {activeSection === 3 && (
        <section style={{ padding: '5rem 0' }}>
          <div style={{ maxWidth: '72rem', margin: '0 auto', padding: '0 2rem' }}>
            <Title level={2} style={{ fontSize: '2.5rem', fontWeight: 'bold', marginBottom: '2rem', color: '#7C3AED', textAlign: 'center' }}>
              📚 Community Resources
            </Title>

            <Row gutter={[24, 24]}>
              <Col xs={24} md={12}>
                <div style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(124, 58, 237, 0.3)', borderRadius: '12px', padding: '2rem', height: '100%', backdropFilter: 'blur(10px)' }}>
                  <Title level={3} style={{ color: '#7C3AED', marginBottom: '1rem' }}>💻 GitHub Repository</Title>
                  <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', marginBottom: '1rem' }}>
                    Complete source code, issues, pull requests, and contribution guidelines.
                  </Paragraph>
                  <Button type="primary" size="large" style={{ background: 'linear-gradient(135deg, #7C3AED 0%, #FFFFFF 100%)', border: 'none', color: '#0A1628', fontWeight: '600' }}>
                    View on GitHub
                  </Button>
                </div>
              </Col>

              <Col xs={24} md={12}>
                <div style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(124, 58, 237, 0.3)', borderRadius: '12px', padding: '2rem', height: '100%', backdropFilter: 'blur(10px)' }}>
                  <Title level={3} style={{ color: '#7C3AED', marginBottom: '1rem' }}>📖 Documentation</Title>
                  <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', marginBottom: '1rem' }}>
                    Architecture guides, API references, tutorials, and setup instructions.
                  </Paragraph>
                  <Button type="primary" size="large" style={{ background: 'linear-gradient(135deg, #7C3AED 0%, #FFFFFF 100%)', border: 'none', color: '#0A1628', fontWeight: '600' }}>
                    Read Docs
                  </Button>
                </div>
              </Col>

              <Col xs={24} md={12}>
                <div style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(124, 58, 237, 0.3)', borderRadius: '12px', padding: '2rem', height: '100%', backdropFilter: 'blur(10px)' }}>
                  <Title level={3} style={{ color: '#7C3AED', marginBottom: '1rem' }}>💬 Community Chat</Title>
                  <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', marginBottom: '1rem' }}>
                    Join our Discord/Telegram for real-time discussions, questions, and collaboration.
                  </Paragraph>
                  <Button type="primary" size="large" style={{ background: 'linear-gradient(135deg, #7C3AED 0%, #FFFFFF 100%)', border: 'none', color: '#0A1628', fontWeight: '600' }}>
                    Join Chat
                  </Button>
                </div>
              </Col>

              <Col xs={24} md={12}>
                <div style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(124, 58, 237, 0.3)', borderRadius: '12px', padding: '2rem', height: '100%', backdropFilter: 'blur(10px)' }}>
                  <Title level={3} style={{ color: '#7C3AED', marginBottom: '1rem' }}>📧 Direct Contact</Title>
                  <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8', marginBottom: '1rem' }}>
                    Have questions? Want to contribute? Reach out directly.
                  </Paragraph>
                  <Button type="primary" size="large" style={{ background: 'linear-gradient(135deg, #7C3AED 0%, #FFFFFF 100%)', border: 'none', color: '#0A1628', fontWeight: '600' }}>
                    umesh@pravyom.com
                  </Button>
                </div>
              </Col>
            </Row>
          </div>
        </section>
      )}

      {/* Section 4: FAQ */}
      {activeSection === 4 && (
        <section style={{ padding: '5rem 0' }}>
          <div style={{ maxWidth: '72rem', margin: '0 auto', padding: '0 2rem' }}>
            <Title level={2} style={{ fontSize: '2.5rem', fontWeight: 'bold', marginBottom: '2rem', color: '#F59E0B', textAlign: 'center' }}>
              ❓ Frequently Asked Questions
            </Title>

            <div style={{ background: 'rgba(10, 22, 40, 0.9)', border: '2px solid rgba(245, 158, 11, 0.3)', borderRadius: '12px', padding: '3rem', backdropFilter: 'blur(10px)' }}>
              <Collapse 
                bordered={false} 
                style={{ background: 'transparent' }}
                expandIconPosition="end"
              >
                <Panel 
                  header={<span style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>Do I need to know Rust to contribute?</span>} 
                  key="1"
                  style={{ background: 'rgba(245, 158, 11, 0.1)', border: '1px solid rgba(245, 158, 11, 0.3)', borderRadius: '8px', marginBottom: '1rem' }}
                >
                  <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8' }}>
                    No! While the core infrastructure is written in Rust, you can contribute in many ways: documentation, testing, design, community building, content creation. If you want to learn Rust, we provide resources and mentorship.
                  </Paragraph>
                </Panel>

                <Panel 
                  header={<span style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>Is this really open-source?</span>} 
                  key="2"
                  style={{ background: 'rgba(245, 158, 11, 0.1)', border: '1px solid rgba(245, 158, 11, 0.3)', borderRadius: '8px', marginBottom: '1rem' }}
                >
                  <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8' }}>
                    Yes. The entire codebase is open-source on GitHub. Anyone can view, fork, and contribute. We follow standard open-source practices with transparent development and community governance.
                  </Paragraph>
                </Panel>

                <Panel 
                  header={<span style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>Will I get paid for contributions?</span>} 
                  key="3"
                  style={{ background: 'rgba(245, 158, 11, 0.1)', border: '1px solid rgba(245, 158, 11, 0.3)', borderRadius: '8px', marginBottom: '1rem' }}
                >
                  <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8' }}>
                    Currently, this is volunteer-based. However, significant contributors may receive equity/tokens when GEN coin launches (depends on traction and funding). Think of it as early-stage startup equity—high risk, high potential reward.
                  </Paragraph>
                </Panel>

                <Panel 
                  header={<span style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>How much time do I need to commit?</span>} 
                  key="4"
                  style={{ background: 'rgba(245, 158, 11, 0.1)', border: '1px solid rgba(245, 158, 11, 0.3)', borderRadius: '8px', marginBottom: '1rem' }}
                >
                  <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8' }}>
                    As much or as little as you want. Contributions range from one-time (fix a typo) to ongoing (core maintainer). Start small, increase commitment as you get more involved. No pressure.
                  </Paragraph>
                </Panel>

                <Panel 
                  header={<span style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>What if I'm a complete beginner?</span>} 
                  key="5"
                  style={{ background: 'rgba(245, 158, 11, 0.1)', border: '1px solid rgba(245, 158, 11, 0.3)', borderRadius: '8px', marginBottom: '1rem' }}
                >
                  <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8' }}>
                    Perfect! Start with documentation, testing, or asking questions. We have "good first issue" tags on GitHub for beginners. The community is supportive—everyone was a beginner once.
                  </Paragraph>
                </Panel>

                <Panel 
                  header={<span style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>How do I stay updated on project developments?</span>} 
                  key="6"
                  style={{ background: 'rgba(245, 158, 11, 0.1)', border: '1px solid rgba(245, 158, 11, 0.3)', borderRadius: '8px', marginBottom: '1rem' }}
                >
                  <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8' }}>
                    Follow the GitHub repository (watch for updates), join our community chat (Discord/Telegram), and subscribe to our newsletter. Major updates are announced across all channels.
                  </Paragraph>
                </Panel>

                <Panel 
                  header={<span style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>Can I use this for my own project?</span>} 
                  key="7"
                  style={{ background: 'rgba(245, 158, 11, 0.1)', border: '1px solid rgba(245, 158, 11, 0.3)', borderRadius: '8px', marginBottom: '1rem' }}
                >
                  <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8' }}>
                    Yes! It's open-source. Fork it, modify it, build on top of it. Just follow the license terms. If you build something cool, let us know—we'd love to feature it!
                  </Paragraph>
                </Panel>

                <Panel 
                  header={<span style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>What's the long-term vision for this project?</span>} 
                  key="8"
                  style={{ background: 'rgba(245, 158, 11, 0.1)', border: '1px solid rgba(245, 158, 11, 0.3)', borderRadius: '8px', marginBottom: '1rem' }}
                >
                  <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8' }}>
                    Build production-grade infrastructure for enterprise and government use cases. Current phase: testnet (75% complete). Next: pilot partnerships, testing, security audits. Final: mainnet launch with GEN coin (traction-based, not time-based).
                  </Paragraph>
                </Panel>

                <Panel 
                  header={<span style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>How can I get more involved beyond code?</span>} 
                  key="9"
                  style={{ background: 'rgba(245, 158, 11, 0.1)', border: '1px solid rgba(245, 158, 11, 0.3)', borderRadius: '8px', marginBottom: '1rem' }}
                >
                  <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8' }}>
                    Organize meetups, create educational content, help with community moderation, translate documentation, provide design feedback, or become a community advocate. Many ways to contribute beyond code!
                  </Paragraph>
                </Panel>

                <Panel 
                  header={<span style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>Who do I contact if I have more questions?</span>} 
                  key="10"
                  style={{ background: 'rgba(245, 158, 11, 0.1)', border: '1px solid rgba(245, 158, 11, 0.3)', borderRadius: '8px', marginBottom: '1rem' }}
                >
                  <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.8' }}>
                    Email <strong>umesh@pravyom.com</strong> for direct questions. For technical questions, use GitHub issues. For general discussion, join our community chat. We're responsive and happy to help!
                  </Paragraph>
                </Panel>
              </Collapse>
            </div>
          </div>
        </section>
      )}
    </div>
  );
};

export default Community;
