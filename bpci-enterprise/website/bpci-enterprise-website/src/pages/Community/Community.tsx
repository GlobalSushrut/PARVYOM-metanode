import React, { useState, useEffect } from 'react';
import { Typography, Card, Row, Col, Button, Statistic, List, Avatar, Tag, Space, Alert, Spin, Divider } from 'antd';
import { Link } from 'react-router-dom';
import { 
  TeamOutlined, 
  GithubOutlined, 
  MessageOutlined, 
  BookOutlined,
  RocketOutlined,
  GlobalOutlined,
  SafetyOutlined,
  ApiOutlined,
  UserOutlined,
  StarOutlined,
  CoffeeOutlined,
  HeartOutlined,
  MailOutlined,
  TwitterOutlined,
  DollarOutlined,
  BulbOutlined,
  WarningOutlined
} from '@ant-design/icons';
import { registryService } from '../../services/registryService';
import './Community.css';

const { Title, Paragraph, Text } = Typography;

const Community: React.FC = () => {
  const [communityStats, setCommunityStats] = useState<any>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchCommunityStats = async () => {
      try {
        const stats = await registryService.getRegistryStats();
        setCommunityStats(stats);
      } catch (error) {
        console.error('Failed to fetch community stats:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchCommunityStats();
  }, []);

  const communityResources = [
    {
      title: "GitHub Repository",
      description: "Access the complete BPCI source code, contribute to development, and report issues.",
      icon: <GithubOutlined style={{ fontSize: '24px', color: '#333' }} />,
      link: "https://github.com/bpci-enterprise",
      action: "View on GitHub"
    },
    {
      title: "Developer Documentation",
      description: "Comprehensive guides, API references, and tutorials for building on BPCI.",
      icon: <BookOutlined style={{ fontSize: '24px', color: '#1890ff' }} />,
      link: "/docs",
      action: "Read Docs"
    },
    {
      title: "Community Forum",
      description: "Connect with other developers, ask questions, and share your projects.",
      icon: <MessageOutlined style={{ fontSize: '24px', color: '#52c41a' }} />,
      link: "https://forum.bpci.dev",
      action: "Join Forum"
    },
    {
      title: "BPI Node Installer",
      description: "Install and run your own BPI community node to join the network.",
      icon: <RocketOutlined style={{ fontSize: '24px', color: '#722ed1' }} />,
      link: "/installer",
      action: "Install Node"
    }
  ];

  const contributionAreas = [
    {
      area: "🚀 Start Small & Learn",
      description: "New to blockchain? Start with documentation, bug reports, or testing. No prior experience needed - we'll guide you!",
      skills: ["Curiosity", "Basic Computer Skills", "Willingness to Learn"],
      difficulty: "Beginner",
      realTalk: "Perfect for students, career changers, or anyone wanting to break into Web3. We provide mentorship!"
    },
    {
      area: "💻 Code Contributions",
      description: "Ready to code? Help with frontend (React/TypeScript), backend (Rust), or smart contracts. All skill levels welcome.",
      skills: ["Any Programming Language", "Git Basics", "Problem Solving"],
      difficulty: "Intermediate",
      realTalk: "Don't worry if you don't know Rust yet - we have learning resources and pair programming sessions."
    },
    {
      area: "📝 Content & Community",
      description: "Create tutorials, write blog posts, manage social media, or help with community support and onboarding.",
      skills: ["Writing", "Communication", "Social Media", "Community Management"],
      difficulty: "Beginner",
      realTalk: "Great for building your portfolio while helping others. Flexible hours, work from anywhere."
    },
    {
      area: "🔧 DevOps & Infrastructure",
      description: "Help with deployment, monitoring, CI/CD, or running community nodes. Learn real-world infrastructure skills.",
      skills: ["Linux", "Docker", "Cloud Platforms", "Networking"],
      difficulty: "Advanced",
      realTalk: "High-demand skills that translate directly to well-paying jobs. We'll teach you enterprise-grade practices."
    }
  ];

  const recentUpdates = [
    {
      title: "BPCI v2.1.0 Released",
      description: "New autonomous economy features with 4-coin system (GEN/NEX/FLX/AUR)",
      date: "2024-01-15",
      type: "release"
    },
    {
      title: "Enhanced Security Module",
      description: "Military-grade security with post-quantum cryptography support",
      date: "2024-01-10",
      type: "feature"
    },
    {
      title: "Community Node Program",
      description: "New incentive program for community node operators",
      date: "2024-01-05",
      type: "program"
    }
  ];

  return (
    <div style={{ padding: '24px' }}>
      {/* Hero Section */}
      <div style={{ textAlign: 'center', marginBottom: '48px' }}>
        <Title level={1} style={{ marginBottom: '16px' }}>
          <TeamOutlined style={{ marginRight: '12px', color: '#667eea' }} />
          BPCI Community
        </Title>
        <Paragraph style={{ fontSize: '18px', maxWidth: '800px', margin: '0 auto 32px' }}>
          Join a global community of developers, researchers, and innovators building the future 
          of secure, decentralized Internet infrastructure. Contribute to open-source projects, 
          run community nodes, and help shape the next generation of Web3 technology.
        </Paragraph>
      </div>

      {/* Community Stats */}
      {loading ? (
        <div style={{ textAlign: 'center', padding: '50px' }}>
          <Spin size="large" />
          <div style={{ marginTop: '16px' }}>Loading community statistics...</div>
        </div>
      ) : communityStats ? (
        <Row gutter={[16, 16]} style={{ marginBottom: '48px' }}>
          <Col xs={24} sm={12} md={6}>
            <Card>
              <Statistic
                title="Community Nodes"
                value={communityStats.community_nodes}
                prefix={<GlobalOutlined />}
                valueStyle={{ color: '#1890ff' }}
              />
            </Card>
          </Col>
          <Col xs={24} sm={12} md={6}>
            <Card>
              <Statistic
                title="Active Developers"
                value={communityStats.total_validators + communityStats.total_miners}
                prefix={<UserOutlined />}
                valueStyle={{ color: '#52c41a' }}
              />
            </Card>
          </Col>
          <Col xs={24} sm={12} md={6}>
            <Card>
              <Statistic
                title="Network Health"
                value={communityStats.network_health}
                prefix={<SafetyOutlined />}
                valueStyle={{ 
                  color: communityStats.network_health === 'Excellent' ? '#52c41a' : '#1890ff'
                }}
              />
            </Card>
          </Col>
          <Col xs={24} sm={12} md={6}>
            <Card>
              <Statistic
                title="Total Nodes"
                value={communityStats.total_nodes}
                prefix={<ApiOutlined />}
                valueStyle={{ color: '#722ed1' }}
              />
            </Card>
          </Col>
        </Row>
      ) : (
        <Alert
          message="Community Stats Unavailable"
          description="Unable to load community statistics at this time."
          type="warning"
          style={{ marginBottom: '48px' }}
        />
      )}

      {/* Community Resources */}
      <div style={{ marginBottom: '48px' }}>
        <Title level={2} style={{ textAlign: 'center', marginBottom: '32px' }}>
          Community Resources
        </Title>
        <Row gutter={[24, 24]}>
          {communityResources.map((resource, index) => (
            <Col xs={24} sm={12} lg={6} key={index}>
              <Card
                hoverable
                style={{ height: '100%', textAlign: 'center' }}
                actions={[
                  resource.link.startsWith('http') ? (
                    <a href={resource.link} target="_blank" rel="noopener noreferrer">
                      <Button type="primary">{resource.action}</Button>
                    </a>
                  ) : (
                    <Link to={resource.link}>
                      <Button type="primary">{resource.action}</Button>
                    </Link>
                  )
                ]}
              >
                <div style={{ padding: '24px 16px' }}>
                  <div style={{ marginBottom: '16px' }}>
                    {resource.icon}
                  </div>
                  <Title level={4} style={{ marginBottom: '12px' }}>
                    {resource.title}
                  </Title>
                  <Paragraph style={{ color: '#666', fontSize: '14px' }}>
                    {resource.description}
                  </Paragraph>
                </div>
              </Card>
            </Col>
          ))}
        </Row>
      </div>

      {/* How to Contribute - Reality-Based */}
      <div style={{ marginBottom: '48px' }}>
        <Title level={2} style={{ textAlign: 'center', marginBottom: '16px' }}>
          How to Actually Contribute (No BS Guide)
        </Title>
        <Paragraph style={{ textAlign: 'center', fontSize: '16px', maxWidth: '800px', margin: '0 auto 32px', color: '#666' }}>
          Real talk: We need help, and you can gain valuable experience. Here's how to get started without the usual corporate fluff.
        </Paragraph>
        
        <Row gutter={[24, 24]}>
          {contributionAreas.map((area, index) => (
            <Col xs={24} md={12} key={index}>
              <Card style={{ height: '100%' }}>
                <div style={{ marginBottom: '16px' }}>
                  <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '8px' }}>
                    <Title level={4} style={{ margin: 0 }}>
                      {area.area}
                    </Title>
                    <Tag color={
                      area.difficulty === 'Beginner' ? 'green' :
                      area.difficulty === 'Intermediate' ? 'orange' : 'red'
                    }>
                      {area.difficulty}
                    </Tag>
                  </div>
                  <Paragraph style={{ color: '#666', marginBottom: '16px' }}>
                    {area.description}
                  </Paragraph>
                  
                  {/* Reality Check Section */}
                  <div className="reality-section" style={{ 
                    background: 'rgba(102, 126, 234, 0.1)', 
                    border: '1px solid rgba(102, 126, 234, 0.2)', 
                    borderRadius: '6px', 
                    padding: '12px', 
                    marginBottom: '16px' 
                  }}>
                    <Text strong style={{ color: '#667eea', display: 'block', marginBottom: '4px' }}>
                      <BulbOutlined /> Reality Check:
                    </Text>
                    <Text style={{ fontSize: '14px', color: '#555' }}>
                      {area.realTalk}
                    </Text>
                  </div>

                  <div>
                    <Text strong style={{ marginBottom: '8px', display: 'block' }}>What You Need:</Text>
                    <Space wrap>
                      {area.skills.map((skill, skillIndex) => (
                        <Tag key={skillIndex} color="blue">{skill}</Tag>
                      ))}
                    </Space>
                  </div>
                </div>
              </Card>
            </Col>
          ))}
        </Row>

        {/* Getting Started Steps */}
        <Card style={{ marginTop: '32px', background: 'linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%)' }}>
          <Title level={3} style={{ textAlign: 'center', marginBottom: '24px' }}>
            <RocketOutlined /> Ready to Start? Here's Your Action Plan
          </Title>
          <Row gutter={[24, 16]}>
            <Col xs={24} md={8}>
              <div style={{ textAlign: 'center', padding: '16px' }}>
                <div style={{ fontSize: '32px', marginBottom: '8px' }}>1️⃣</div>
                <Text strong>Join Our Discord</Text>
                <br />
                <Text style={{ fontSize: '14px', color: '#666' }}>
                  Introduce yourself, ask questions, find mentors
                </Text>
              </div>
            </Col>
            <Col xs={24} md={8}>
              <div style={{ textAlign: 'center', padding: '16px' }}>
                <div style={{ fontSize: '32px', marginBottom: '8px' }}>2️⃣</div>
                <Text strong>Pick Your First Task</Text>
                <br />
                <Text style={{ fontSize: '14px', color: '#666' }}>
                  Browse "good first issue" labels on GitHub
                </Text>
              </div>
            </Col>
            <Col xs={24} md={8}>
              <div style={{ textAlign: 'center', padding: '16px' }}>
                <div style={{ fontSize: '32px', marginBottom: '8px' }}>3️⃣</div>
                <Text strong>Get Support</Text>
                <br />
                <Text style={{ fontSize: '14px', color: '#666' }}>
                  We'll pair you with a mentor and provide resources
                </Text>
              </div>
            </Col>
          </Row>
        </Card>
      </div>

      {/* Recent Updates */}
      <div style={{ marginBottom: '48px' }}>
        <Title level={2} style={{ textAlign: 'center', marginBottom: '32px' }}>
          Recent Community Updates
        </Title>
        <Card>
          <List
            itemLayout="horizontal"
            dataSource={recentUpdates}
            renderItem={(item) => (
              <List.Item>
                <List.Item.Meta
                  avatar={
                    <Avatar 
                      icon={
                        item.type === 'release' ? <RocketOutlined /> :
                        item.type === 'feature' ? <StarOutlined /> :
                        <TeamOutlined />
                      }
                      style={{ 
                        backgroundColor: 
                          item.type === 'release' ? '#52c41a' :
                          item.type === 'feature' ? '#1890ff' :
                          '#722ed1'
                      }}
                    />
                  }
                  title={item.title}
                  description={item.description}
                />
                <div style={{ color: '#666', fontSize: '14px' }}>
                  {new Date(item.date).toLocaleDateString()}
                </div>
              </List.Item>
            )}
          />
        </Card>
      </div>

      {/* Business Pitch - Reality-Based */}
      <div className="pitch-section" style={{ textAlign: 'center', padding: '48px 24px', background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)', borderRadius: '12px', color: 'white', marginBottom: '48px' }}>
        <Title level={2} style={{ color: 'white', marginBottom: '16px' }}>
          <HeartOutlined /> Why BPCI Matters (The Real Story)
        </Title>
        <Paragraph style={{ fontSize: '18px', color: 'rgba(255,255,255,0.9)', marginBottom: '24px', maxWidth: '800px', margin: '0 auto 24px' }}>
          We're not another crypto project promising to "revolutionize everything." We're building practical, 
          secure infrastructure that actually works. Here's the honest pitch:
        </Paragraph>

        <Row gutter={[32, 24]} style={{ marginBottom: '32px' }}>
          <Col xs={24} md={8}>
            <div style={{ padding: '16px' }}>
              <WarningOutlined style={{ fontSize: '32px', marginBottom: '12px', color: '#ffd700' }} />
              <Title level={4} style={{ color: 'white', marginBottom: '8px' }}>The Problem</Title>
              <Text style={{ color: 'rgba(255,255,255,0.8)', fontSize: '14px' }}>
                Current blockchain infrastructure is slow, expensive, and hard to use. 
                Most projects are overhyped and underdelivered.
              </Text>
            </div>
          </Col>
          <Col xs={24} md={8}>
            <div style={{ padding: '16px' }}>
              <BulbOutlined style={{ fontSize: '32px', marginBottom: '12px', color: '#52c41a' }} />
              <Title level={4} style={{ color: 'white', marginBottom: '8px' }}>Our Solution</Title>
              <Text style={{ color: 'rgba(255,255,255,0.8)', fontSize: '14px' }}>
                Fast, secure, and actually usable blockchain infrastructure. 
                Built by developers, for developers. No marketing fluff.
              </Text>
            </div>
          </Col>
          <Col xs={24} md={8}>
            <div style={{ padding: '16px' }}>
              <RocketOutlined style={{ fontSize: '32px', marginBottom: '12px', color: '#1890ff' }} />
              <Title level={4} style={{ color: 'white', marginBottom: '8px' }}>The Reality</Title>
              <Text style={{ color: 'rgba(255,255,255,0.8)', fontSize: '14px' }}>
                We're early stage, bootstrapped, and need your help. 
                But we're building something real that you can use today.
              </Text>
            </div>
          </Col>
        </Row>

        <Divider style={{ borderColor: 'rgba(255,255,255,0.3)', margin: '32px 0' }} />

        <Title level={3} style={{ color: 'white', marginBottom: '24px' }}>
          <DollarOutlined /> Support the Project
        </Title>
        <Paragraph style={{ fontSize: '16px', color: 'rgba(255,255,255,0.9)', marginBottom: '24px' }}>
          We're bootstrapped and could use your support. Every contribution helps us build better infrastructure.
        </Paragraph>

        <div className="support-buttons" style={{ display: 'flex', gap: '16px', justifyContent: 'center', flexWrap: 'wrap', marginBottom: '32px' }}>
          <a href="https://buymeacoffee.com/bpci" target="_blank" rel="noopener noreferrer">
            <Button 
              size="large" 
              className="coffee-button"
              style={{ background: '#ff813f', border: 'none', color: 'white', fontWeight: '600' }}
            >
              <CoffeeOutlined /> Buy Me Coffee
            </Button>
          </a>
          <a href="https://github.com/sponsors/bpci-enterprise" target="_blank" rel="noopener noreferrer">
            <Button 
              size="large" 
              className="github-button"
              style={{ background: '#333', border: 'none', color: 'white', fontWeight: '600' }}
            >
              <GithubOutlined /> GitHub Sponsors
            </Button>
          </a>
          <a href="mailto:support@bpci.dev" target="_blank" rel="noopener noreferrer">
            <Button 
              size="large" 
              style={{ background: '#52c41a', border: 'none', color: 'white', fontWeight: '600' }}
            >
              <MailOutlined /> Contact Us
            </Button>
          </a>
        </div>

        <Title level={4} style={{ color: 'white', marginBottom: '16px' }}>
          Connect With Us
        </Title>
        <Space size="large">
          <a href="https://discord.gg/bpci" target="_blank" rel="noopener noreferrer">
            <Button size="large" ghost style={{ fontWeight: '600' }}>
              <MessageOutlined /> Discord
            </Button>
          </a>
          <a href="https://twitter.com/bpci_dev" target="_blank" rel="noopener noreferrer">
            <Button size="large" ghost style={{ fontWeight: '600' }}>
              <TwitterOutlined /> Twitter
            </Button>
          </a>
          <Link to="/installer">
            <Button size="large" style={{ background: 'white', color: '#667eea', border: 'none', fontWeight: '600' }}>
              <RocketOutlined /> Try It Now
            </Button>
          </Link>
        </Space>
      </div>

      {/* Honest Expectations */}
      <Card style={{ marginBottom: '48px', background: 'rgba(255, 193, 7, 0.1)', border: '1px solid rgba(255, 193, 7, 0.3)' }}>
        <Title level={3} style={{ textAlign: 'center', marginBottom: '16px', color: '#d48806' }}>
          <WarningOutlined /> Set Your Expectations Right
        </Title>
        <Row gutter={[24, 16]}>
          <Col xs={24} md={12}>
            <Title level={5} style={{ color: '#d48806', marginBottom: '8px' }}>✅ What We Promise:</Title>
            <ul style={{ color: '#666', fontSize: '14px' }}>
              <li>Honest communication about progress and challenges</li>
              <li>Real mentorship and learning opportunities</li>
              <li>Credit for your contributions</li>
              <li>A supportive, drama-free community</li>
              <li>Practical experience with cutting-edge tech</li>
            </ul>
          </Col>
          <Col xs={24} md={12}>
            <Title level={5} style={{ color: '#d48806', marginBottom: '8px' }}>❌ What We Don't Promise:</Title>
            <ul style={{ color: '#666', fontSize: '14px' }}>
              <li>Get-rich-quick schemes or token airdrops</li>
              <li>Immediate job placement (but we'll help with skills)</li>
              <li>Perfect code or zero bugs (we're human)</li>
              <li>24/7 support (we have day jobs too)</li>
              <li>Overnight success (good things take time)</li>
            </ul>
          </Col>
        </Row>
      </Card>
    </div>
  );
};

export default Community;
