import React from 'react';
import { Typography, Card, Button, Steps, Row, Col } from 'antd';
import { Link } from 'react-router-dom';
import { RocketOutlined, ApiOutlined, BookOutlined, TeamOutlined } from '@ant-design/icons';

const { Title, Paragraph } = Typography;

const GetStarted: React.FC = () => {
  const steps = [
    {
      title: 'Explore',
      description: 'Learn about BPCI technology and vision',
      icon: <BookOutlined />,
    },
    {
      title: 'Connect',
      description: 'Join our community and discussions',
      icon: <TeamOutlined />,
    },
    {
      title: 'Build',
      description: 'Start experimenting with our APIs',
      icon: <ApiOutlined />,
    },
    {
      title: 'Deploy',
      description: 'Launch your enterprise solutions',
      icon: <RocketOutlined />,
    },
  ];

  return (
    <div className="max-w-6xl mx-auto px-4 py-16">
      <div className="text-center mb-16">
        <Title level={1} className="mb-6">Get Started with BPCI</Title>
        <Paragraph className="text-xl text-gray-600 max-w-3xl mx-auto">
          Ready to explore the future of secure Internet infrastructure? 
          Here's how you can begin your journey with BPCI Enterprise.
        </Paragraph>
      </div>

      <Card className="p-8 mb-12">
        <Title level={2} className="text-center mb-8">Your Journey</Title>
        <Steps
          current={0}
          items={steps}
          direction="horizontal"
          className="mb-8"
        />
        <div className="text-center text-gray-600">
          <p>Follow these steps to get the most out of BPCI Enterprise</p>
        </div>
      </Card>

      <Row gutter={[32, 32]}>
        <Col xs={24} md={12}>
          <Card className="h-full p-6 hover:shadow-lg transition-shadow">
            <div className="text-center">
              <BookOutlined className="text-4xl text-blue-600 mb-4" />
              <Title level={3} className="mb-4">Learn the Basics</Title>
              <Paragraph className="text-gray-600 mb-6">
                Start by understanding our technology, vision, and current progress. 
                No technical background required.
              </Paragraph>
              <Link to="/about">
                <Button type="primary" size="large">
                  Learn More
                </Button>
              </Link>
            </div>
          </Card>
        </Col>

        <Col xs={24} md={12}>
          <Card className="h-full p-6 hover:shadow-lg transition-shadow">
            <div className="text-center">
              <ApiOutlined className="text-4xl text-green-600 mb-4" />
              <Title level={3} className="mb-4">Explore the Dashboard</Title>
              <Paragraph className="text-gray-600 mb-6">
                See real-time data from our blockchain infrastructure, 
                autonomous economy, and security systems.
              </Paragraph>
              <Link to="/dashboard">
                <Button type="primary" size="large">
                  View Dashboard
                </Button>
              </Link>
            </div>
          </Card>
        </Col>

        <Col xs={24} md={12}>
          <Card className="h-full p-6 hover:shadow-lg transition-shadow">
            <div className="text-center">
              <TeamOutlined className="text-4xl text-purple-600 mb-4" />
              <Title level={3} className="mb-4">Join the Community</Title>
              <Paragraph className="text-gray-600 mb-6">
                Connect with other developers, researchers, and innovators 
                building the future of secure Internet.
              </Paragraph>
              <Link to="/community">
                <Button type="primary" size="large">
                  Join Community
                </Button>
              </Link>
            </div>
          </Card>
        </Col>

        <Col xs={24} md={12}>
          <Card className="h-full p-6 hover:shadow-lg transition-shadow">
            <div className="text-center">
              <RocketOutlined className="text-4xl text-orange-600 mb-4" />
              <Title level={3} className="mb-4">Enterprise Solutions</Title>
              <Paragraph className="text-gray-600 mb-6">
                Discover how BPCI can power your enterprise infrastructure 
                with military-grade security and compliance.
              </Paragraph>
              <Link to="/enterprise">
                <Button type="primary" size="large">
                  Enterprise Info
                </Button>
              </Link>
            </div>
          </Card>
        </Col>
      </Row>

      <Card className="mt-12 p-8 bg-gradient-to-r from-blue-50 to-purple-50">
        <div className="text-center">
          <Title level={2} className="mb-4">Ready to Start?</Title>
          <Paragraph className="text-lg text-gray-600 mb-6">
            Every meaningful project starts with curiosity. Join us as we explore 
            what's possible in the post-observation Internet era.
          </Paragraph>
          <div className="space-x-4">
            <Link to="/technology">
              <Button size="large" className="mr-4">
                Learn Technology
              </Button>
            </Link>
            <Link to="/dashboard">
              <Button type="primary" size="large">
                View Live Data
              </Button>
            </Link>
          </div>
        </div>
      </Card>
    </div>
  );
};

export default GetStarted;
