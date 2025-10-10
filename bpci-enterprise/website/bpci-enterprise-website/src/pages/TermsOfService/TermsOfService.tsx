import React from 'react';
import { Typography, Card, Space, Alert, Timeline } from 'antd';
import { 
  ExclamationCircleOutlined,
  CheckCircleOutlined,
  InfoCircleOutlined,
  WarningOutlined
} from '@ant-design/icons';
import parvyomLogo from '../../assets/images/parvyom-logo.png';
import './TermsOfService.css';

const { Title, Paragraph, Text } = Typography;

const TermsOfService: React.FC = () => {
  return (
    <div className="terms-page">
      {/* Hero Section */}
      <div className="terms-hero">
        <div className="terms-hero-content">
          <div className="terms-hero-logo">
            <img src={parvyomLogo} alt="Parvyom Logo" />
            <Title level={1} style={{ color: 'white', marginBottom: '16px' }}>
              Terms of Service
            </Title>
          </div>
          <Paragraph style={{ color: '#e2e8f0', fontSize: '1.125rem', textAlign: 'center', maxWidth: '600px' }}>
            Legal terms governing the use of PARVYOM BPI/BPCI Enterprise Infrastructure
          </Paragraph>
          <div className="terms-meta">
            <Text style={{ color: '#94a3b8' }}>Last Updated: September 30, 2024</Text>
            <Text style={{ color: '#94a3b8' }}>Version: 1.0</Text>
          </div>
        </div>
      </div>

      {/* Terms Content */}
      <div className="terms-content">
        <div className="terms-container">
          
          {/* Important Notice */}
          <Alert
            message="Experimental Technology Notice"
            description="PARVYOM is experimental blockchain infrastructure under active development. These terms apply to testing, research, and pilot program participation only."
            type="warning"
            icon={<ExclamationCircleOutlined />}
            showIcon
            className="terms-alert"
          />

          {/* Table of Contents */}
          <Card className="terms-card">
            <Title level={2}>Table of Contents</Title>
            <div className="terms-toc">
              <a href="#acceptance">1. Acceptance of Terms</a>
              <a href="#definitions">2. Definitions</a>
              <a href="#services">3. Services Description</a>
              <a href="#eligibility">4. Eligibility and Registration</a>
              <a href="#pilot-programs">5. Enterprise Pilot Programs</a>
              <a href="#intellectual-property">6. Intellectual Property</a>
              <a href="#user-responsibilities">7. User Responsibilities</a>
              <a href="#limitations">8. Limitations and Disclaimers</a>
              <a href="#privacy">9. Privacy and Data Protection</a>
              <a href="#termination">10. Termination</a>
              <a href="#governing-law">11. Governing Law</a>
              <a href="#contact">12. Contact Information</a>
            </div>
          </Card>

          {/* Terms Sections */}
          <Card className="terms-card" id="acceptance">
            <Title level={2}>1. Acceptance of Terms</Title>
            <Paragraph>
              By accessing, using, or participating in PARVYOM BPI/BPCI infrastructure, services, 
              pilot programs, or related technologies ("Services"), you agree to be bound by these 
              Terms of Service ("Terms"). If you do not agree to these Terms, you may not use our Services.
            </Paragraph>
            <Paragraph>
              These Terms constitute a legally binding agreement between you and PARVYOM regarding 
              your use of experimental blockchain infrastructure and related services.
            </Paragraph>
          </Card>

          <Card className="terms-card" id="definitions">
            <Title level={2}>2. Definitions</Title>
            <Space direction="vertical" size="middle" style={{ width: '100%' }}>
              <div>
                <Text strong>"PARVYOM"</Text> refers to the experimental blockchain infrastructure 
                project, including BPI (Blockchain Protocol Infrastructure) and BPCI (Blockchain 
                Protocol Community Interface) components.
              </div>
              <div>
                <Text strong>"Services"</Text> includes all PARVYOM infrastructure, software, 
                documentation, pilot programs, and related technologies.
              </div>
              <div>
                <Text strong>"User"</Text> refers to any individual, organization, or entity 
                accessing or using PARVYOM Services.
              </div>
              <div>
                <Text strong>"Enterprise Participant"</Text> refers to organizations participating 
                in PARVYOM enterprise pilot programs under separate agreements.
              </div>
            </Space>
          </Card>

          <Card className="terms-card" id="services">
            <Title level={2}>3. Services Description</Title>
            <Paragraph>
              PARVYOM provides experimental blockchain infrastructure including:
            </Paragraph>
            <ul className="terms-list">
              <li>BPI Core: 6D blockchain consensus and infrastructure orchestration</li>
              <li>BPCI Interface: Community and enterprise coordination layer</li>
              <li>Immutable OS: Operating system integration and security</li>
              <li>Enterprise pilot programs and testing environments</li>
              <li>Documentation, tools, and developer resources</li>
            </ul>
            <Alert
              message="Experimental Status"
              description="All Services are experimental and provided for research, testing, and pilot purposes only. Not suitable for production use without proper evaluation."
              type="info"
              showIcon
              style={{ marginTop: '16px' }}
            />
          </Card>

          <Card className="terms-card" id="eligibility">
            <Title level={2}>4. Eligibility and Registration</Title>
            <Paragraph>
              To use PARVYOM Services, you must:
            </Paragraph>
            <Timeline
              items={[
                {
                  dot: <CheckCircleOutlined className="timeline-icon-success" />,
                  children: 'Be at least 18 years old or have legal capacity to enter contracts'
                },
                {
                  dot: <CheckCircleOutlined className="timeline-icon-success" />,
                  children: 'Provide accurate and complete registration information'
                },
                {
                  dot: <CheckCircleOutlined className="timeline-icon-success" />,
                  children: 'Comply with all applicable laws and regulations'
                },
                {
                  dot: <CheckCircleOutlined className="timeline-icon-success" />,
                  children: 'Meet technical requirements for blockchain infrastructure participation'
                }
              ]}
            />
            <Paragraph>
              Enterprise participants must additionally meet specific technical, security, 
              and operational requirements as outlined in pilot program documentation.
            </Paragraph>
          </Card>

          <Card className="terms-card" id="pilot-programs">
            <Title level={2}>5. Enterprise Pilot Programs</Title>
            <Paragraph>
              Participation in PARVYOM enterprise pilot programs requires:
            </Paragraph>
            <ul className="terms-list">
              <li>Separate pilot program agreement and compliance documentation</li>
              <li>Technical infrastructure meeting specified requirements</li>
              <li>Security protocols and operational standards compliance</li>
              <li>Regular reporting and feedback participation</li>
              <li>Commitment to experimental technology evaluation and testing</li>
            </ul>
            <Alert
              message="Pilot Program Terms"
              description="Enterprise pilot programs are governed by additional terms and agreements specific to each program. Contact enterprise@parvyom.com for details."
              type="info"
              showIcon
              style={{ marginTop: '16px' }}
            />
          </Card>

          <Card className="terms-card" id="intellectual-property">
            <Title level={2}>6. Intellectual Property</Title>
            <Paragraph>
              PARVYOM technology, documentation, and materials are protected by intellectual 
              property rights. Users are granted limited rights to:
            </Paragraph>
            <ul className="terms-list">
              <li>Use PARVYOM Services for authorized testing and evaluation</li>
              <li>Access documentation for implementation and integration</li>
              <li>Participate in pilot programs under specified conditions</li>
            </ul>
            <Paragraph>
              Commercial use, redistribution, or derivative works require separate licensing 
              agreements. Open source components are governed by their respective licenses.
            </Paragraph>
          </Card>

          <Card className="terms-card" id="user-responsibilities">
            <Title level={2}>7. User Responsibilities</Title>
            <Paragraph>Users are responsible for:</Paragraph>
            <ul className="terms-list">
              <li>Maintaining security of access credentials and infrastructure</li>
              <li>Complying with all applicable laws and regulations</li>
              <li>Using Services only for authorized purposes</li>
              <li>Reporting security vulnerabilities through proper channels</li>
              <li>Maintaining appropriate backup and disaster recovery procedures</li>
              <li>Not attempting to compromise or disrupt PARVYOM infrastructure</li>
            </ul>
          </Card>

          <Card className="terms-card" id="limitations">
            <Title level={2}>8. Limitations and Disclaimers</Title>
            <Alert
              message="Important Disclaimers"
              description="PARVYOM Services are provided 'as-is' without warranties of any kind."
              type="warning"
              icon={<WarningOutlined />}
              showIcon
              style={{ marginBottom: '16px' }}
            />
            <Paragraph>
              <Text strong>Limitation of Liability:</Text> PARVYOM and its contributors shall not 
              be liable for any damages arising from the use of experimental infrastructure, 
              including but not limited to technical failures, security vulnerabilities, or data loss.
            </Paragraph>
            <Paragraph>
              <Text strong>No Warranties:</Text> Services are provided without warranties of 
              merchantability, fitness for a particular purpose, or non-infringement.
            </Paragraph>
            <Paragraph>
              <Text strong>Experimental Nature:</Text> Users acknowledge that PARVYOM is 
              experimental technology and assume all risks associated with its use.
            </Paragraph>
          </Card>

          <Card className="terms-card" id="privacy">
            <Title level={2}>9. Privacy and Data Protection</Title>
            <Paragraph>
              Your privacy and data protection are governed by our Privacy Policy, which is 
              incorporated by reference into these Terms. By using PARVYOM Services, you 
              consent to the collection, use, and processing of information as described 
              in our Privacy Policy.
            </Paragraph>
            <Paragraph>
              Enterprise participants may be subject to additional data protection requirements 
              as specified in pilot program agreements.
            </Paragraph>
          </Card>

          <Card className="terms-card" id="termination">
            <Title level={2}>10. Termination</Title>
            <Paragraph>
              These Terms remain in effect until terminated. PARVYOM may terminate or suspend 
              access to Services at any time for any reason, including violation of these Terms.
            </Paragraph>
            <Paragraph>
              Upon termination, your right to use Services ceases immediately. Provisions 
              regarding intellectual property, limitations of liability, and governing law 
              survive termination.
            </Paragraph>
          </Card>

          <Card className="terms-card" id="governing-law">
            <Title level={2}>11. Governing Law</Title>
            <Paragraph>
              These Terms are governed by applicable laws in the jurisdiction where PARVYOM 
              operations are based, without regard to conflict of law principles. Any disputes 
              shall be resolved through appropriate legal channels in the governing jurisdiction.
            </Paragraph>
          </Card>

          <Card className="terms-card" id="contact">
            <Title level={2}>12. Contact Information</Title>
            <Paragraph>
              For questions about these Terms of Service, please contact:
            </Paragraph>
            <div className="terms-contact">
              <Text strong>Legal Team:</Text> <a href="mailto:legal@parvyom.com">legal@parvyom.com</a><br />
              <Text strong>Enterprise Legal:</Text> <a href="mailto:enterprise-legal@parvyom.com">enterprise-legal@parvyom.com</a><br />
              <Text strong>General Contact:</Text> <a href="mailto:contact@parvyom.com">contact@parvyom.com</a>
            </div>
          </Card>

          {/* Agreement Confirmation */}
          <Card className="terms-agreement-card">
            <div className="terms-agreement-content">
              <InfoCircleOutlined className="terms-agreement-icon" />
              <div>
                <Title level={3}>Agreement Confirmation</Title>
                <Paragraph>
                  By using PARVYOM Services, you acknowledge that you have read, understood, 
                  and agree to be bound by these Terms of Service and our Privacy Policy.
                </Paragraph>
                <Text type="secondary">
                  Last updated: September 30, 2024 | Version 1.0
                </Text>
              </div>
            </div>
          </Card>

        </div>
      </div>
    </div>
  );
};

export default TermsOfService;
