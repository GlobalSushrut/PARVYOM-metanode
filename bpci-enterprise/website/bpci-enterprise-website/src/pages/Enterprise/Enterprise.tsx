import React from 'react';
import { Typography, Card, Row, Col, Button, List, Badge, Steps, Divider } from 'antd';
import { Link } from 'react-router-dom';
import { 
  RocketOutlined, 
  SecurityScanOutlined, 
  GlobalOutlined, 
  BankOutlined,
  TeamOutlined,
  AuditOutlined,
  LockOutlined,
  ApiOutlined,
  SafetyOutlined,
  CloudServerOutlined,
  CheckCircleOutlined,
  SettingOutlined
} from '@ant-design/icons';

const { Title, Paragraph } = Typography;

const Enterprise: React.FC = () => {
  const enterpriseFeatures = [
    {
      icon: <SecurityScanOutlined className="text-3xl text-blue-600" />,
      title: "Military-Grade Security",
      description: "Post-quantum cryptography with 9.8/10 security rating, ENC Lock + QLOCK protection, and infinite noise response."
    },
    {
      icon: <BankOutlined className="text-3xl text-green-600" />,
      title: "Banking Compliance",
      description: "Basel III, PCI DSS, SOX compliance with dedicated banking APIs and settlement infrastructure."
    },
    {
      icon: <GlobalOutlined className="text-3xl text-purple-600" />,
      title: "Government Ready",
      description: "FedRAMP, FISMA, NIST frameworks with jurisdiction-aware governance and regulatory compliance."
    },
    {
      icon: <ApiOutlined className="text-3xl text-orange-600" />,
      title: "Enterprise APIs",
      description: "Production-ready REST APIs, WebSocket streaming, and comprehensive SDKs for seamless integration."
    }
  ];

  const useCases = [
    {
      sector: "Banking & Finance",
      icon: <BankOutlined className="text-2xl text-blue-600" />,
      applications: [
        "Cross-border payment infrastructure",
        "Central Bank Digital Currency (CBDC) systems",
        "Trade finance and settlement networks",
        "Regulatory compliance and audit trails",
        "Anti-money laundering (AML) monitoring"
      ],
      benefits: "Reduce settlement times from days to minutes while maintaining full regulatory compliance."
    },
    {
      sector: "Government & Defense",
      icon: <SafetyOutlined className="text-2xl text-green-600" />,
      applications: [
        "Secure government communications",
        "Digital identity and citizenship systems",
        "Healthcare data management",
        "Supply chain transparency",
        "Emergency response coordination"
      ],
      benefits: "Ensure data sovereignty and citizen privacy with quantum-resistant security."
    },
    {
      sector: "Enterprise & Corporate",
      icon: <CloudServerOutlined className="text-2xl text-purple-600" />,
      applications: [
        "Supply chain management",
        "Document verification systems",
        "Inter-company settlements",
        "Audit and compliance tracking",
        "Intellectual property protection"
      ],
      benefits: "Streamline operations with transparent, immutable business processes."
    }
  ];

  const implementationSteps = [
    {
      title: "Assessment & Planning",
      description: "Analyze your current infrastructure and define integration requirements."
    },
    {
      title: "Pilot Deployment",
      description: "Deploy a limited testnet environment for evaluation and testing."
    },
    {
      title: "Integration Development",
      description: "Develop custom integrations using our enterprise APIs and SDKs."
    },
    {
      title: "Security Validation",
      description: "Comprehensive security audits and compliance verification."
    },
    {
      title: "Production Deployment",
      description: "Full-scale deployment with monitoring and support infrastructure."
    },
    {
      title: "Ongoing Support",
      description: "24/7 monitoring, maintenance, and continuous optimization."
    }
  ];

  return (
    <div className="enterprise-page">
      {/* Hero Section */}
      <section className="hero-gradient py-20">
        <div className="max-w-6xl mx-auto px-4 text-center">
          <Title level={1} className="text-white text-5xl font-bold mb-6">
            Enterprise Solutions
          </Title>
          <Paragraph className="text-blue-100 text-xl max-w-4xl mx-auto mb-8">
            Military-grade blockchain infrastructure for banks, governments, and enterprises 
            requiring the highest levels of security, compliance, and reliability.
          </Paragraph>
          <div className="flex flex-wrap justify-center gap-4">
            <Link to="/get-started">
              <Button 
                type="primary" 
                size="large" 
                icon={<RocketOutlined />}
                className="bg-white text-blue-600 hover:bg-gray-100 text-lg h-12 px-8 font-semibold border-0"
              >
                Start Enterprise Trial
              </Button>
            </Link>
            <Link to="/dashboard">
              <Button 
                size="large" 
                icon={<ApiOutlined />}
                className="text-white border-white hover:bg-white hover:text-blue-600 text-lg h-12 px-8 font-semibold"
              >
                View Live Demo
              </Button>
            </Link>
          </div>
        </div>
      </section>

      {/* Enterprise Features */}
      <section className="py-20 bg-white">
        <div className="max-w-6xl mx-auto px-4">
          <div className="text-center mb-16">
            <Title level={2} className="text-4xl font-bold mb-6">Enterprise-Grade Features</Title>
            <Paragraph className="text-xl text-gray-600">
              Built for the most demanding institutional requirements
            </Paragraph>
          </div>

          <Row gutter={[32, 32]}>
            {enterpriseFeatures.map((feature, index) => (
              <Col xs={24} md={12} key={index}>
                <Card className="h-full p-6 hover:shadow-lg transition-shadow">
                  <div className="flex items-start space-x-4">
                    {feature.icon}
                    <div>
                      <Title level={3} className="text-xl font-semibold mb-3">
                        {feature.title}
                      </Title>
                      <Paragraph className="text-gray-700">
                        {feature.description}
                      </Paragraph>
                    </div>
                  </div>
                </Card>
              </Col>
            ))}
          </Row>
        </div>
      </section>

      {/* Use Cases by Sector */}
      <section className="py-20 bg-gray-50">
        <div className="max-w-6xl mx-auto px-4">
          <div className="text-center mb-16">
            <Title level={2} className="text-4xl font-bold mb-6">Industry Solutions</Title>
            <Paragraph className="text-xl text-gray-600">
              Tailored blockchain infrastructure for specific industry needs
            </Paragraph>
          </div>

          <Row gutter={[32, 32]}>
            {useCases.map((useCase, index) => (
              <Col xs={24} lg={8} key={index}>
                <Card className="h-full">
                  <div className="text-center mb-6">
                    {useCase.icon}
                    <Title level={3} className="text-xl font-semibold mt-4 mb-4">
                      {useCase.sector}
                    </Title>
                  </div>
                  
                  <div className="mb-6">
                    <Title level={5} className="mb-3">Key Applications:</Title>
                    <List
                      size="small"
                      dataSource={useCase.applications}
                      renderItem={(item) => (
                        <List.Item>
                          <CheckCircleOutlined className="text-green-600 mr-2" />
                          {item}
                        </List.Item>
                      )}
                    />
                  </div>

                  <div className="bg-blue-50 p-4 rounded-lg">
                    <Title level={5} className="text-blue-800 mb-2">Business Impact:</Title>
                    <Paragraph className="text-blue-700 mb-0">
                      {useCase.benefits}
                    </Paragraph>
                  </div>
                </Card>
              </Col>
            ))}
          </Row>
        </div>
      </section>

      {/* Technical Specifications */}
      <section className="py-20 bg-white">
        <div className="max-w-6xl mx-auto px-4">
          <div className="text-center mb-16">
            <Title level={2} className="text-4xl font-bold mb-6">Technical Specifications</Title>
            <Paragraph className="text-xl text-gray-600">
              Enterprise-grade performance and reliability metrics
            </Paragraph>
          </div>

          <Row gutter={[32, 32]}>
            <Col xs={24} md={12}>
              <Card title="Performance Metrics" className="h-full">
                <div className="space-y-4">
                  <div className="flex justify-between items-center">
                    <span className="font-medium">Transaction Throughput:</span>
                    <Badge count="10,000+ TPS" style={{ backgroundColor: '#52c41a' }} />
                  </div>
                  <div className="flex justify-between items-center">
                    <span className="font-medium">Network Latency:</span>
                    <Badge count="<100ms" style={{ backgroundColor: '#1890ff' }} />
                  </div>
                  <div className="flex justify-between items-center">
                    <span className="font-medium">Uptime SLA:</span>
                    <Badge count="99.9%" style={{ backgroundColor: '#722ed1' }} />
                  </div>
                  <div className="flex justify-between items-center">
                    <span className="font-medium">Data Availability:</span>
                    <Badge count="99.99%" style={{ backgroundColor: '#fa8c16' }} />
                  </div>
                </div>
              </Card>
            </Col>

            <Col xs={24} md={12}>
              <Card title="Security & Compliance" className="h-full">
                <div className="space-y-4">
                  <div className="flex justify-between items-center">
                    <span className="font-medium">Security Rating:</span>
                    <Badge count="9.8/10" style={{ backgroundColor: '#52c41a' }} />
                  </div>
                  <div className="flex justify-between items-center">
                    <span className="font-medium">Encryption:</span>
                    <Badge count="Post-Quantum" style={{ backgroundColor: '#1890ff' }} />
                  </div>
                  <div className="flex justify-between items-center">
                    <span className="font-medium">Compliance:</span>
                    <Badge count="Multi-Standard" style={{ backgroundColor: '#722ed1' }} />
                  </div>
                  <div className="flex justify-between items-center">
                    <span className="font-medium">Audit Trail:</span>
                    <Badge count="Immutable" style={{ backgroundColor: '#fa8c16' }} />
                  </div>
                </div>
              </Card>
            </Col>
          </Row>
        </div>
      </section>

      {/* Implementation Process */}
      <section className="py-20 bg-gray-50">
        <div className="max-w-6xl mx-auto px-4">
          <div className="text-center mb-16">
            <Title level={2} className="text-4xl font-bold mb-6">Implementation Process</Title>
            <Paragraph className="text-xl text-gray-600">
              Structured approach to enterprise blockchain deployment
            </Paragraph>
          </div>

          <div className="max-w-4xl mx-auto">
            <Steps
              direction="vertical"
              current={-1}
              items={implementationSteps.map((step, index) => ({
                title: step.title,
                description: step.description,
                icon: index === 0 ? <AuditOutlined /> : 
                      index === 1 ? <SettingOutlined /> :
                      index === 2 ? <ApiOutlined /> :
                      index === 3 ? <LockOutlined /> :
                      index === 4 ? <CloudServerOutlined /> :
                      <TeamOutlined />
              }))}
            />
          </div>
        </div>
      </section>

      {/* Enterprise Support */}
      <section className="py-20 bg-white">
        <div className="max-w-6xl mx-auto px-4">
          <Row gutter={[48, 48]} align="middle">
            <Col xs={24} lg={12}>
              <Title level={2} className="text-4xl font-bold mb-6">Enterprise Support</Title>
              <Paragraph className="text-lg text-gray-700 mb-6">
                Dedicated support infrastructure for mission-critical deployments with 
                24/7 monitoring, maintenance, and expert consultation.
              </Paragraph>
              
              <div className="space-y-4">
                <div className="flex items-center space-x-3">
                  <CheckCircleOutlined className="text-green-600" />
                  <span>24/7 technical support and monitoring</span>
                </div>
                <div className="flex items-center space-x-3">
                  <CheckCircleOutlined className="text-green-600" />
                  <span>Dedicated customer success manager</span>
                </div>
                <div className="flex items-center space-x-3">
                  <CheckCircleOutlined className="text-green-600" />
                  <span>Custom integration development</span>
                </div>
                <div className="flex items-center space-x-3">
                  <CheckCircleOutlined className="text-green-600" />
                  <span>Priority security updates and patches</span>
                </div>
                <div className="flex items-center space-x-3">
                  <CheckCircleOutlined className="text-green-600" />
                  <span>Compliance consulting and audit support</span>
                </div>
              </div>
            </Col>

            <Col xs={24} lg={12}>
              <Card className="p-8 bg-gradient-to-r from-blue-50 to-purple-50">
                <Title level={3} className="text-center mb-6">Ready to Get Started?</Title>
                <Paragraph className="text-center text-gray-700 mb-6">
                  Contact our enterprise team to discuss your specific requirements 
                  and explore how BPCI can transform your infrastructure.
                </Paragraph>
                
                <div className="space-y-4">
                  <Link to="/get-started">
                    <Button 
                      type="primary" 
                      size="large" 
                      block
                      icon={<RocketOutlined />}
                      className="font-semibold"
                    >
                      Start Enterprise Trial
                    </Button>
                  </Link>
                  <Button 
                    size="large" 
                    block
                    icon={<TeamOutlined />}
                    className="font-semibold"
                  >
                    Schedule Consultation
                  </Button>
                </div>

                <Divider />

                <div className="text-center">
                  <Paragraph className="text-sm text-gray-600 mb-2">
                    Enterprise inquiries:
                  </Paragraph>
                  <Paragraph className="text-sm font-medium">
                    enterprise@bpci.dev
                  </Paragraph>
                </div>
              </Card>
            </Col>
          </Row>
        </div>
      </section>

      {/* Current Status */}
      <section className="py-12 bg-blue-600">
        <div className="max-w-4xl mx-auto px-4 text-center">
          <Title level={3} className="text-white mb-4">Current Development Status</Title>
          <Paragraph className="text-blue-100 text-lg mb-6">
            BPCI Enterprise is currently in active development with testnet operational. 
            Mainnet deployment scheduled for 6 months with enterprise-ready features.
          </Paragraph>
          <div className="flex flex-wrap justify-center gap-6 text-blue-100">
            <div className="text-center">
              <div className="text-2xl font-bold">95%</div>
              <div className="text-sm">Security Layer</div>
            </div>
            <div className="text-center">
              <div className="text-2xl font-bold">90%</div>
              <div className="text-sm">Core Infrastructure</div>
            </div>
            <div className="text-center">
              <div className="text-2xl font-bold">85%</div>
              <div className="text-sm">Enterprise APIs</div>
            </div>
            <div className="text-center">
              <div className="text-2xl font-bold">80%</div>
              <div className="text-sm">Compliance Framework</div>
            </div>
          </div>
        </div>
      </section>
    </div>
  );
};

export default Enterprise;
