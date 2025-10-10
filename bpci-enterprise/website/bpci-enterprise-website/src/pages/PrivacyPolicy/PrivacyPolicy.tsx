import React from 'react';
import { Typography, Card, Divider, Space, Alert, Table } from 'antd';
import { 
  SafetyCertificateOutlined, 
  CheckCircleOutlined,
  EyeInvisibleOutlined,
  LockOutlined,
  DatabaseOutlined,
  UserOutlined
} from '@ant-design/icons';
import parvyomLogo from '../../assets/images/parvyom-logo.png';
import './PrivacyPolicy.css';

const { Title, Paragraph, Text } = Typography;

const PrivacyPolicy: React.FC = () => {
  const dataTypesColumns = [
    {
      title: 'Data Type',
      dataIndex: 'type',
      key: 'type',
      render: (text: string) => <Text strong style={{ color: '#60a5fa' }}>{text}</Text>
    },
    {
      title: 'Purpose',
      dataIndex: 'purpose',
      key: 'purpose',
      render: (text: string) => <Text style={{ color: '#e2e8f0' }}>{text}</Text>
    },
    {
      title: 'Retention',
      dataIndex: 'retention',
      key: 'retention',
      render: (text: string) => <Text style={{ color: '#cbd5e1' }}>{text}</Text>
    }
  ];

  const dataTypesData = [
    {
      key: '1',
      type: 'Account Information',
      purpose: 'User registration, authentication, and service provision',
      retention: 'Duration of account + 2 years'
    },
    {
      key: '2',
      type: 'Technical Data',
      purpose: 'Infrastructure monitoring, security, and performance optimization',
      retention: '12 months'
    },
    {
      key: '3',
      type: 'Usage Analytics',
      purpose: 'Service improvement and research purposes',
      retention: '24 months (anonymized)'
    },
    {
      key: '4',
      type: 'Communication Data',
      purpose: 'Support, updates, and pilot program coordination',
      retention: '3 years'
    },
    {
      key: '5',
      type: 'Blockchain Data',
      purpose: 'Network operation and consensus participation',
      retention: 'Permanent (immutable)'
    }
  ];

  return (
    <div className="privacy-page">
      {/* Hero Section */}
      <div className="privacy-hero">
        <div className="privacy-hero-content">
          <div className="privacy-hero-logo">
            <img src={parvyomLogo} alt="Parvyom Logo" />
            <Title level={1} style={{ color: 'white', marginBottom: '16px' }}>
              Privacy Policy
            </Title>
          </div>
          <Paragraph style={{ color: '#e2e8f0', fontSize: '1.125rem', textAlign: 'center', maxWidth: '600px' }}>
            Our commitment to protecting your privacy and data security in PARVYOM infrastructure
          </Paragraph>
          <div className="privacy-meta">
            <Text style={{ color: '#94a3b8' }}>Last Updated: September 30, 2024</Text>
            <Text style={{ color: '#94a3b8' }}>Version: 1.0</Text>
          </div>
        </div>
      </div>

      {/* Privacy Content */}
      <div className="privacy-content">
        <div className="privacy-container">
          
          {/* Privacy Commitment */}
          <Alert
            message="Privacy-First Approach"
            description="PARVYOM is built with privacy and security as core principles. We collect only necessary data and implement advanced protection measures."
            type="success"
            icon={<CheckCircleOutlined />}
            showIcon
            className="privacy-alert"
          />

          {/* Overview */}
          <Card className="privacy-card">
            <div className="privacy-card-header">
              <SafetyCertificateOutlined className="privacy-icon" />
              <Title level={2}>Privacy Overview</Title>
            </div>
            <Paragraph>
              This Privacy Policy explains how PARVYOM ("we," "us," or "our") collects, uses, 
              processes, and protects your personal information when you use our experimental 
              blockchain infrastructure, participate in pilot programs, or interact with our services.
            </Paragraph>
            <Paragraph>
              As an experimental blockchain infrastructure project, we are committed to transparency 
              and implementing privacy-by-design principles throughout our technology stack.
            </Paragraph>
          </Card>

          {/* Information We Collect */}
          <Card className="privacy-card">
            <div className="privacy-card-header">
              <DatabaseOutlined className="privacy-icon" />
              <Title level={2}>Information We Collect</Title>
            </div>
            
            <Title level={3}>Data Collection Principles</Title>
            <ul className="privacy-list">
              <li><Text strong>Minimal Collection:</Text> We collect only data necessary for service operation</li>
              <li><Text strong>Purpose Limitation:</Text> Data is used only for specified, legitimate purposes</li>
              <li><Text strong>Transparency:</Text> Clear disclosure of all data collection practices</li>
              <li><Text strong>User Control:</Text> Users maintain control over their personal data</li>
            </ul>

            <Title level={3} style={{ marginTop: '32px' }}>Data Types and Usage</Title>
            <div className="privacy-table-wrapper">
              <Table 
                columns={dataTypesColumns} 
                dataSource={dataTypesData} 
                pagination={false}
                className="privacy-table"
              />
            </div>
          </Card>

          {/* How We Use Information */}
          <Card className="privacy-card">
            <div className="privacy-card-header">
              <UserOutlined className="privacy-icon" />
              <Title level={2}>How We Use Your Information</Title>
            </div>
            
            <Space direction="vertical" size="large" style={{ width: '100%' }}>
              <div>
                <Title level={4}>Service Operations</Title>
                <ul className="privacy-list">
                  <li>Providing access to PARVYOM infrastructure and services</li>
                  <li>Managing user accounts and authentication</li>
                  <li>Processing pilot program applications and participation</li>
                  <li>Monitoring system performance and security</li>
                </ul>
              </div>

              <div>
                <Title level={4}>Communication</Title>
                <ul className="privacy-list">
                  <li>Sending service updates and security notifications</li>
                  <li>Providing technical support and assistance</li>
                  <li>Coordinating pilot program activities</li>
                  <li>Sharing research findings and development updates</li>
                </ul>
              </div>

              <div>
                <Title level={4}>Research and Development</Title>
                <ul className="privacy-list">
                  <li>Improving blockchain infrastructure and consensus mechanisms</li>
                  <li>Analyzing usage patterns for service optimization</li>
                  <li>Conducting security research and vulnerability assessment</li>
                  <li>Developing new features and capabilities</li>
                </ul>
              </div>
            </Space>
          </Card>

          {/* Data Protection */}
          <Card className="privacy-card">
            <div className="privacy-card-header">
              <LockOutlined className="privacy-icon" />
              <Title level={2}>Data Protection Measures</Title>
            </div>
            
            <Title level={3}>Technical Safeguards</Title>
            <ul className="privacy-list">
              <li><Text strong>Encryption:</Text> End-to-end encryption for data transmission and storage</li>
              <li><Text strong>Access Controls:</Text> Role-based access with multi-factor authentication</li>
              <li><Text strong>Network Security:</Text> Advanced firewall and intrusion detection systems</li>
              <li><Text strong>Blockchain Security:</Text> Immutable ledger with cryptographic verification</li>
              <li><Text strong>Zero-Knowledge Proofs:</Text> Privacy-preserving verification mechanisms</li>
            </ul>

            <Title level={3} style={{ marginTop: '24px' }}>Operational Safeguards</Title>
            <ul className="privacy-list">
              <li>Regular security audits and penetration testing</li>
              <li>Employee training on data protection and privacy</li>
              <li>Incident response procedures and breach notification</li>
              <li>Data minimization and automated deletion policies</li>
              <li>Third-party security assessments and certifications</li>
            </ul>
          </Card>

          {/* Data Sharing */}
          <Card className="privacy-card">
            <div className="privacy-card-header">
              <EyeInvisibleOutlined className="privacy-icon" />
              <Title level={2}>Data Sharing and Disclosure</Title>
            </div>
            
            <Alert
              message="No Data Sales"
              description="We never sell, rent, or trade your personal information to third parties for commercial purposes."
              type="info"
              showIcon
              style={{ marginBottom: '24px' }}
            />

            <Title level={3}>Limited Sharing Scenarios</Title>
            <Space direction="vertical" size="middle" style={{ width: '100%' }}>
              <div>
                <Text strong>Service Providers:</Text> Trusted partners who assist in service delivery 
                under strict confidentiality agreements and data protection requirements.
              </div>
              <div>
                <Text strong>Legal Requirements:</Text> When required by law, court order, or to 
                protect our rights and the safety of users and the public.
              </div>
              <div>
                <Text strong>Research Collaboration:</Text> Anonymized, aggregated data may be 
                shared with academic institutions for blockchain research purposes.
              </div>
              <div>
                <Text strong>Enterprise Partners:</Text> Pilot program participants may share 
                specific operational data under separate agreements.
              </div>
            </Space>
          </Card>

          {/* User Rights */}
          <Card className="privacy-card">
            <Title level={2}>Your Privacy Rights</Title>
            
            <div className="privacy-rights-grid">
              <div className="privacy-right-item">
                <Title level={4}>Access</Title>
                <Paragraph>Request access to your personal data and information about how it's processed.</Paragraph>
              </div>
              
              <div className="privacy-right-item">
                <Title level={4}>Correction</Title>
                <Paragraph>Request correction of inaccurate or incomplete personal information.</Paragraph>
              </div>
              
              <div className="privacy-right-item">
                <Title level={4}>Deletion</Title>
                <Paragraph>Request deletion of your personal data, subject to legal and operational requirements.</Paragraph>
              </div>
              
              <div className="privacy-right-item">
                <Title level={4}>Portability</Title>
                <Paragraph>Request export of your data in a structured, machine-readable format.</Paragraph>
              </div>
              
              <div className="privacy-right-item">
                <Title level={4}>Objection</Title>
                <Paragraph>Object to processing of your personal data for specific purposes.</Paragraph>
              </div>
              
              <div className="privacy-right-item">
                <Title level={4}>Restriction</Title>
                <Paragraph>Request restriction of processing under certain circumstances.</Paragraph>
              </div>
            </div>
          </Card>

          {/* Blockchain Considerations */}
          <Card className="privacy-card">
            <Title level={2}>Blockchain Privacy Considerations</Title>
            
            <Alert
              message="Immutable Data Notice"
              description="Data recorded on the blockchain becomes immutable and cannot be deleted or modified. We implement privacy-preserving techniques to minimize personal data on-chain."
              type="warning"
              showIcon
              style={{ marginBottom: '24px' }}
            />

            <Title level={3}>Privacy-Preserving Technologies</Title>
            <ul className="privacy-list">
              <li><Text strong>Zero-Knowledge Proofs:</Text> Verify transactions without revealing sensitive data</li>
              <li><Text strong>Hash Functions:</Text> Store only cryptographic hashes instead of raw data</li>
              <li><Text strong>Off-Chain Storage:</Text> Keep personal data off the blockchain when possible</li>
              <li><Text strong>Pseudonymization:</Text> Use pseudonyms instead of real identities</li>
              <li><Text strong>Selective Disclosure:</Text> Share only necessary information for verification</li>
            </ul>
          </Card>

          {/* International Transfers */}
          <Card className="privacy-card">
            <Title level={2}>International Data Transfers</Title>
            <Paragraph>
              PARVYOM operates globally, and your data may be transferred to and processed in 
              countries other than your residence. We ensure appropriate safeguards are in place 
              for international transfers, including:
            </Paragraph>
            <ul className="privacy-list">
              <li>Adequacy decisions by relevant data protection authorities</li>
              <li>Standard contractual clauses approved by regulatory bodies</li>
              <li>Binding corporate rules and certification mechanisms</li>
              <li>Explicit consent where required by applicable law</li>
            </ul>
          </Card>

          {/* Contact Information */}
          <Card className="privacy-contact-card">
            <Title level={2}>Privacy Contact Information</Title>
            <Paragraph>
              For privacy-related questions, requests, or concerns, please contact our 
              Data Protection Team:
            </Paragraph>
            <div className="privacy-contact-info">
              <Text strong>Privacy Officer:</Text> <a href="mailto:privacy@parvyom.com">privacy@parvyom.com</a><br />
              <Text strong>Data Protection:</Text> <a href="mailto:dpo@parvyom.com">dpo@parvyom.com</a><br />
              <Text strong>Security Issues:</Text> <a href="mailto:security@parvyom.com">security@parvyom.com</a><br />
              <Text strong>General Contact:</Text> <a href="mailto:contact@parvyom.com">contact@parvyom.com</a>
            </div>
            
            <Divider />
            
            <div className="privacy-update-notice">
              <Text type="secondary">
                This Privacy Policy was last updated on September 30, 2024. We will notify users 
                of any material changes through our services and website.
              </Text>
            </div>
          </Card>

        </div>
      </div>
    </div>
  );
};

export default PrivacyPolicy;
