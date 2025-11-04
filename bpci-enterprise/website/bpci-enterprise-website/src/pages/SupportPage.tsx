import React from 'react';
import { Card, Row, Col, Typography, Space, Button, Collapse, Input, Form } from 'antd';
import { QuestionCircleOutlined, MessageOutlined, BookOutlined, SendOutlined } from '@ant-design/icons';

const { Title, Text, Paragraph } = Typography;
const { Panel } = Collapse;
const { TextArea } = Input;

const SupportPage: React.FC = () => {
  const [form] = Form.useForm();

  const faqs = [
    {
      question: 'How do I deploy a BPI OS node?',
      answer: 'Use the Node Deployment Wizard to deploy a BPI OS node in 4 easy steps. Select your platform, configure settings, download the installer, and deploy using BSO-K8.'
    },
    {
      question: 'What is the 4-coin economy?',
      answer: 'The autonomous economy uses 4 coins: GEN (Genesis), NEX (Nexus), FLX (Flux), and AUR (Aurum). Each coin serves a specific purpose in the ecosystem.'
    },
    {
      question: 'How does dual-authentication work?',
      answer: 'Dual-auth combines Keycloak SSO (Level 1) with BPI wallet authentication (Level 2) for enhanced security. Complete the 3-step wizard to activate.'
    }
  ];

  return (
    <div style={{ padding: '24px', maxWidth: '1200px', margin: '0 auto' }}>
      <div style={{ marginBottom: 24 }}>
        <Space>
          <QuestionCircleOutlined style={{ fontSize: '32px', color: '#1890ff' }} />
          <div>
            <Title level={2} style={{ margin: 0 }}>Help & Support</Title>
            <Text type="secondary">Get help with BPCI Enterprise</Text>
          </div>
        </Space>
      </div>

      <Row gutter={[16, 16]}>
        <Col xs={24} lg={12}>
          <Card title={<Space><BookOutlined />Frequently Asked Questions</Space>}>
            <Collapse>
              {faqs.map((faq, index) => (
                <Panel header={faq.question} key={index}>
                  <Paragraph>{faq.answer}</Paragraph>
                </Panel>
              ))}
            </Collapse>
          </Card>
        </Col>

        <Col xs={24} lg={12}>
          <Card title={<Space><MessageOutlined />Contact Support</Space>}>
            <Form form={form} layout="vertical">
              <Form.Item label="Subject" name="subject" rules={[{ required: true }]}>
                <Input placeholder="Enter subject" />
              </Form.Item>
              <Form.Item label="Message" name="message" rules={[{ required: true }]}>
                <TextArea rows={6} placeholder="Describe your issue" />
              </Form.Item>
              <Form.Item>
                <Button type="primary" icon={<SendOutlined />} block>
                  Submit Ticket
                </Button>
              </Form.Item>
            </Form>
          </Card>
        </Col>

        <Col xs={24}>
          <Card>
            <Space direction="vertical" style={{ width: '100%' }}>
              <Title level={5}>Quick Links</Title>
              <Space wrap>
                <Button>📖 Documentation</Button>
                <Button>💬 Community Forum</Button>
                <Button>🎥 Video Tutorials</Button>
                <Button>📧 Email Support</Button>
              </Space>
            </Space>
          </Card>
        </Col>
      </Row>
    </div>
  );
};

export default SupportPage;
