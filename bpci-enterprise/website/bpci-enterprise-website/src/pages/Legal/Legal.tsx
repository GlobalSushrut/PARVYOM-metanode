import React from 'react';
import { Typography, Card, Divider, Space, Row, Col } from 'antd';
import { 
  SafetyCertificateOutlined, 
  FileProtectOutlined, 
  GlobalOutlined,
  CheckCircleOutlined,
  AuditOutlined,
  SecurityScanOutlined
} from '@ant-design/icons';
import parvyomLogo from '../../assets/images/parvyom-logo.png';
import './Legal.css';

const { Title, Paragraph, Text } = Typography;

const Legal: React.FC = () => {
  return (
    <div className="legal-page">
      {/* Hero Section */}
      <div className="legal-hero">
        <div className="legal-hero-content">
          <div className="legal-hero-logo">
            <img src={parvyomLogo} alt="Parvyom Logo" />
            <Title level={1} style={{ color: 'white', marginBottom: '16px' }}>
              Legal Information
            </Title>
          </div>
          <Paragraph style={{ color: '#e2e8f0', fontSize: '1.125rem', textAlign: 'center', maxWidth: '600px' }}>
            Comprehensive legal framework for PARVYOM BPI/BPCI Enterprise Infrastructure
          </Paragraph>
        </div>
      </div>

      {/* Legal Content */}
      <div className="legal-content">
        <div className="legal-container">
          
          {/* Legal Overview */}
          <Card className="legal-card">
            <div className="legal-card-header">
              <AuditOutlined className="legal-icon" />
              <Title level={2}>Legal Framework Overview</Title>
            </div>
            <Paragraph>
              PARVYOM operates as an experimental blockchain infrastructure project under development. 
              This legal framework governs the use of our technology, participation in pilot programs, 
              and engagement with our enterprise infrastructure solutions.
            </Paragraph>
            <Paragraph>
              <Text strong>Important Notice:</Text> PARVYOM is currently in experimental development phase. 
              All services, technologies, and infrastructure are provided for testing, research, and 
              pilot program purposes only.
            </Paragraph>
          </Card>

          {/* Legal Documents Grid */}
          <Row gutter={[24, 24]} style={{ marginTop: '32px' }}>
            <Col xs={24} md={12}>
              <Card className="legal-doc-card" hoverable>
                <div className="legal-doc-header">
                  <FileProtectOutlined className="legal-doc-icon" />
                  <Title level={3}>Terms of Service</Title>
                </div>
                <Paragraph>
                  Comprehensive terms governing the use of PARVYOM infrastructure, 
                  pilot programs, and enterprise services.
                </Paragraph>
                <a href="/terms-of-service" className="legal-doc-link">
                  View Terms of Service →
                </a>
              </Card>
            </Col>

            <Col xs={24} md={12}>
              <Card className="legal-doc-card" hoverable>
                <div className="legal-doc-header">
                  <SafetyCertificateOutlined className="legal-doc-icon" />
                  <Title level={3}>Privacy Policy</Title>
                </div>
                <Paragraph>
                  Our commitment to protecting your privacy and data security 
                  across all PARVYOM services and infrastructure.
                </Paragraph>
                <a href="/privacy-policy" className="legal-doc-link">
                  View Privacy Policy →
                </a>
              </Card>
            </Col>

            <Col xs={24} md={12}>
              <Card className="legal-doc-card" hoverable>
                <div className="legal-doc-header">
                  <SecurityScanOutlined className="legal-doc-icon" />
                  <Title level={3}>Security Policy</Title>
                </div>
                <Paragraph>
                  Security protocols, vulnerability reporting, and our approach 
                  to maintaining secure blockchain infrastructure.
                </Paragraph>
                <a href="/security-policy" className="legal-doc-link">
                  View Security Policy →
                </a>
              </Card>
            </Col>

            <Col xs={24} md={12}>
              <Card className="legal-doc-card" hoverable>
                <div className="legal-doc-header">
                  <CheckCircleOutlined className="legal-doc-icon" />
                  <Title level={3}>Compliance Framework</Title>
                </div>
                <Paragraph>
                  Regulatory compliance, enterprise standards, and our approach 
                  to meeting global blockchain infrastructure requirements.
                </Paragraph>
                <a href="/compliance" className="legal-doc-link">
                  View Compliance Framework →
                </a>
              </Card>
            </Col>
          </Row>

          <Divider style={{ margin: '48px 0' }} />

          {/* Legal Notices */}
          <Card className="legal-card">
            <div className="legal-card-header">
              <GlobalOutlined className="legal-icon" />
              <Title level={2}>Important Legal Notices</Title>
            </div>
            
            <Space direction="vertical" size="large" style={{ width: '100%' }}>
              <div>
                <Title level={4}>Experimental Technology Disclaimer</Title>
                <Paragraph>
                  PARVYOM BPI/BPCI infrastructure is experimental technology under active development. 
                  All services are provided "as-is" for research, testing, and pilot program purposes. 
                  Users participate at their own risk and should not deploy in production environments 
                  without proper evaluation and risk assessment.
                </Paragraph>
              </div>

              <div>
                <Title level={4}>Enterprise Pilot Programs</Title>
                <Paragraph>
                  Participation in PARVYOM enterprise pilot programs requires separate agreements 
                  and compliance with additional terms. Enterprise participants must meet specific 
                  technical, security, and operational requirements as outlined in pilot program documentation.
                </Paragraph>
              </div>

              <div>
                <Title level={4}>Intellectual Property</Title>
                <Paragraph>
                  PARVYOM technology, documentation, and associated materials are protected by 
                  intellectual property rights. Open source components are governed by their 
                  respective licenses. Commercial use requires appropriate licensing agreements.
                </Paragraph>
              </div>

              <div>
                <Title level={4}>Limitation of Liability</Title>
                <Paragraph>
                  PARVYOM and its contributors provide this experimental infrastructure without 
                  warranties of any kind. Users assume all risks associated with the use of 
                  PARVYOM technology, including but not limited to technical failures, security 
                  vulnerabilities, or data loss.
                </Paragraph>
              </div>

              <div>
                <Title level={4}>Governing Law</Title>
                <Paragraph>
                  These legal terms and any disputes arising from the use of PARVYOM technology 
                  shall be governed by applicable laws in the jurisdiction where PARVYOM operations 
                  are based, subject to international blockchain and technology regulations.
                </Paragraph>
              </div>
            </Space>
          </Card>

          {/* Contact Legal Team */}
          <Card className="legal-contact-card" style={{ marginTop: '32px' }}>
            <Title level={3}>Legal Inquiries</Title>
            <Paragraph>
              For legal questions, compliance inquiries, or enterprise licensing discussions, 
              please contact our legal team:
            </Paragraph>
            <div className="legal-contact-info">
              <Text strong>Email:</Text> <a href="mailto:legal@parvyom.com">legal@parvyom.com</a><br />
              <Text strong>Enterprise Legal:</Text> <a href="mailto:enterprise-legal@parvyom.com">enterprise-legal@parvyom.com</a><br />
              <Text strong>Compliance:</Text> <a href="mailto:compliance@parvyom.com">compliance@parvyom.com</a>
            </div>
          </Card>

        </div>
      </div>
    </div>
  );
};

export default Legal;
