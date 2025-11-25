import React, { useState } from 'react';
import { Form, Input, Button, Card, Typography, Row, Col, message, Select, Space } from 'antd';
import { 
  MailOutlined, 
  PhoneOutlined, 
  EnvironmentOutlined,
  UserOutlined,
  MessageOutlined,
  SendOutlined,
  BankOutlined,
  ExperimentOutlined,
  TeamOutlined,
  RocketOutlined
} from '@ant-design/icons';
import parvyomLogo from '../../assets/images/parvyom-logo.png';
import { sendContactEmail, type ContactFormData } from '../../services/emailService';
import './Contact.css';

const { Title, Paragraph, Text } = Typography;
const { TextArea } = Input;
const { Option } = Select;

const Contact: React.FC = () => {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (values: ContactFormData) => {
    setLoading(true);
    try {
      // Send email using the email service
      const result = await sendContactEmail(values);
      
      if (result.success) {
        message.success({
          content: 'Message sent successfully! We\'ll get back to you within 24 hours.',
          duration: 5,
          style: {
            marginTop: '20vh',
          },
        });
        form.resetFields();
      } else {
        throw new Error(result.error || 'Failed to send email');
      }
    } catch (error) {
      console.error('Error sending message:', error);
      message.error({
        content: 'Failed to send message. Please try again or contact us directly at umesh@pravyom.com',
        duration: 6,
        style: {
          marginTop: '20vh',
        },
      });
    } finally {
      setLoading(false);
    }
  };

  const contactCategories = [
    { value: 'enterprise', label: 'Enterprise Partnership', icon: <BankOutlined /> },
    { value: 'research', label: 'Research Collaboration', icon: <ExperimentOutlined /> },
    { value: 'community', label: 'Community & Development', icon: <TeamOutlined /> },
    { value: 'pilot', label: 'Pilot Program Interest', icon: <RocketOutlined /> },
    { value: 'technical', label: 'Technical Support', icon: <MessageOutlined /> },
    { value: 'general', label: 'General Inquiry', icon: <MailOutlined /> }
  ];

  return (
    <div style={{
      minHeight: '100vh',
      background: 'linear-gradient(135deg, #0f172a 0%, #1e293b 100%)',
      padding: '40px 20px',
      color: 'white'
    }}>
      <div style={{ maxWidth: '1200px', margin: '0 auto' }}>
        {/* Header Section with Logo */}
        <div style={{
          textAlign: 'center',
          marginBottom: '50px',
          background: 'rgba(255, 255, 255, 0.05)',
          backdropFilter: 'blur(10px)',
          border: '1px solid rgba(255, 255, 255, 0.1)',
          borderRadius: '20px',
          padding: '40px'
        }}>
          <div style={{
            display: 'flex',
            justifyContent: 'center',
            alignItems: 'center',
            marginBottom: '30px',
            flexDirection: 'column',
            gap: '20px'
          }}>
            <img 
              src={parvyomLogo} 
              alt="Parvyom Logo" 
              style={{ 
                height: '80px', 
                width: 'auto',
                filter: 'brightness(1.1) contrast(1.1)',
                borderRadius: '12px',
                boxShadow: '0 8px 24px rgba(59, 130, 246, 0.3)'
              }} 
            />
            <div>
              <Title level={1} style={{ 
                color: 'white', 
                margin: '0',
                fontSize: 'clamp(28px, 5vw, 42px)',
                fontWeight: '700'
              }}>
                Contact Parvyom
              </Title>
              <div style={{ 
                color: '#60a5fa', 
                fontSize: 'clamp(16px, 2.5vw, 20px)',
                fontWeight: '500',
                marginTop: '8px'
              }}>
                BPI/BPCI Enterprise Infrastructure
              </div>
            </div>
          </div>
          
          <Paragraph style={{ 
            color: '#cbd5e1', 
            fontSize: '18px', 
            maxWidth: '800px', 
            margin: '0 auto',
            lineHeight: '1.6'
          }}>
            Ready to explore the future of blockchain infrastructure? Get in touch with our team 
            for enterprise partnerships, research collaborations, or pilot program opportunities.
          </Paragraph>
        </div>

        <Row gutter={[32, 32]}>
          {/* Contact Form */}
          <Col xs={24} lg={14}>
            <Card
              style={{
                background: 'rgba(15, 23, 42, 0.8)',
                border: '1px solid rgba(59, 130, 246, 0.3)',
                borderRadius: '20px',
                boxShadow: '0 8px 32px rgba(0, 0, 0, 0.3)'
              }}
              bodyStyle={{ padding: '40px' }}
            >
              <Title level={2} style={{ color: 'white', marginBottom: '30px' }}>
                <MessageOutlined style={{ marginRight: '12px', color: '#60a5fa' }} />
                Send us a Message
              </Title>

              <Form
                form={form}
                layout="vertical"
                onFinish={handleSubmit}
                size="large"
              >
                <Row gutter={[16, 0]}>
                  <Col xs={24} sm={12}>
                    <Form.Item
                      name="name"
                      label={<span style={{ color: '#e2e8f0', fontWeight: '600' }}>Full Name</span>}
                      rules={[{ required: true, message: 'Please enter your name' }]}
                    >
                      <Input
                        prefix={<UserOutlined style={{ color: '#60a5fa' }} />}
                        placeholder="Your full name"
                        style={{
                          background: 'rgba(255, 255, 255, 0.05)',
                          border: '1px solid rgba(255, 255, 255, 0.2)',
                          borderRadius: '12px',
                          color: 'white'
                        }}
                      />
                    </Form.Item>
                  </Col>
                  <Col xs={24} sm={12}>
                    <Form.Item
                      name="email"
                      label={<span style={{ color: '#e2e8f0', fontWeight: '600' }}>Email Address</span>}
                      rules={[
                        { required: true, message: 'Please enter your email' },
                        { type: 'email', message: 'Please enter a valid email' }
                      ]}
                    >
                      <Input
                        prefix={<MailOutlined style={{ color: '#60a5fa' }} />}
                        placeholder="your.email@company.com"
                        style={{
                          background: 'rgba(255, 255, 255, 0.05)',
                          border: '1px solid rgba(255, 255, 255, 0.2)',
                          borderRadius: '12px',
                          color: 'white'
                        }}
                      />
                    </Form.Item>
                  </Col>
                </Row>

                <Row gutter={[16, 0]}>
                  <Col xs={24} sm={12}>
                    <Form.Item
                      name="company"
                      label={<span style={{ color: '#e2e8f0', fontWeight: '600' }}>Company/Organization</span>}
                    >
                      <Input
                        prefix={<BankOutlined style={{ color: '#60a5fa' }} />}
                        placeholder="Your company name (optional)"
                        style={{
                          background: 'rgba(255, 255, 255, 0.05)',
                          border: '1px solid rgba(255, 255, 255, 0.2)',
                          borderRadius: '12px',
                          color: 'white'
                        }}
                      />
                    </Form.Item>
                  </Col>
                  <Col xs={24} sm={12}>
                    <Form.Item
                      name="phone"
                      label={<span style={{ color: '#e2e8f0', fontWeight: '600' }}>Phone Number</span>}
                    >
                      <Input
                        prefix={<PhoneOutlined style={{ color: '#60a5fa' }} />}
                        placeholder="+1 (555) 123-4567 (optional)"
                        style={{
                          background: 'rgba(255, 255, 255, 0.05)',
                          border: '1px solid rgba(255, 255, 255, 0.2)',
                          borderRadius: '12px',
                          color: 'white'
                        }}
                      />
                    </Form.Item>
                  </Col>
                </Row>

                <Form.Item
                  name="category"
                  label={<span style={{ color: '#e2e8f0', fontWeight: '600' }}>Inquiry Category</span>}
                  rules={[{ required: true, message: 'Please select a category' }]}
                >
                  <Select
                    placeholder="Select inquiry type"
                    style={{
                      borderRadius: '12px'
                    }}
                    dropdownStyle={{
                      background: 'rgba(15, 23, 42, 0.95)',
                      backdropFilter: 'blur(20px)',
                      border: '1px solid rgba(59, 130, 246, 0.3)'
                    }}
                  >
                    {contactCategories.map(category => (
                      <Option key={category.value} value={category.value}>
                        <Space>
                          {category.icon}
                          {category.label}
                        </Space>
                      </Option>
                    ))}
                  </Select>
                </Form.Item>

                <Form.Item
                  name="subject"
                  label={<span style={{ color: '#e2e8f0', fontWeight: '600' }}>Subject</span>}
                  rules={[{ required: true, message: 'Please enter a subject' }]}
                >
                  <Input
                    placeholder="Brief description of your inquiry"
                    style={{
                      background: 'rgba(255, 255, 255, 0.05)',
                      border: '1px solid rgba(255, 255, 255, 0.2)',
                      borderRadius: '12px',
                      color: 'white'
                    }}
                  />
                </Form.Item>

                <Form.Item
                  name="message"
                  label={<span style={{ color: '#e2e8f0', fontWeight: '600' }}>Message</span>}
                  rules={[
                    { required: true, message: 'Please enter your message' },
                    { min: 20, message: 'Message should be at least 20 characters' }
                  ]}
                >
                  <TextArea
                    rows={6}
                    placeholder="Tell us more about your project, requirements, or how we can help you..."
                    style={{
                      background: 'rgba(255, 255, 255, 0.05)',
                      border: '1px solid rgba(255, 255, 255, 0.2)',
                      borderRadius: '12px',
                      color: 'white',
                      resize: 'vertical'
                    }}
                  />
                </Form.Item>

                <Form.Item style={{ marginBottom: 0 }}>
                  <Button
                    type="primary"
                    htmlType="submit"
                    loading={loading}
                    size="large"
                    icon={<SendOutlined />}
                    style={{
                      background: 'linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%)',
                      border: 'none',
                      borderRadius: '12px',
                      height: '50px',
                      fontWeight: '600',
                      fontSize: '16px',
                      boxShadow: '0 4px 15px rgba(59, 130, 246, 0.3)',
                      width: '100%'
                    }}
                  >
                    {loading ? 'Sending Message...' : 'Send Message'}
                  </Button>
                </Form.Item>
              </Form>
            </Card>
          </Col>

          {/* Contact Information */}
          <Col xs={24} lg={10}>
            <div style={{ display: 'flex', flexDirection: 'column', gap: '24px' }}>
              {/* Contact Details */}
              <Card
                style={{
                  background: 'rgba(15, 23, 42, 0.8)',
                  border: '1px solid rgba(16, 185, 129, 0.3)',
                  borderRadius: '20px',
                  boxShadow: '0 8px 32px rgba(0, 0, 0, 0.3)'
                }}
                bodyStyle={{ padding: '30px' }}
              >
                <Title level={3} style={{ color: 'white', marginBottom: '24px' }}>
                  <EnvironmentOutlined style={{ marginRight: '12px', color: '#10b981' }} />
                  Get in Touch
                </Title>

                <div style={{ display: 'flex', flexDirection: 'column', gap: '20px' }}>
                  <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                    <MailOutlined style={{ color: '#10b981', fontSize: '18px' }} />
                    <div>
                      <Text style={{ color: '#e2e8f0', fontSize: '14px', display: 'block' }}>Contact Email</Text>
                      <Text style={{ color: 'white', fontSize: '16px', fontWeight: '600' }}>
                        umesh@pravyom.com
                      </Text>
                    </div>
                  </div>

                  <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                    <MessageOutlined style={{ color: '#10b981', fontSize: '18px' }} />
                    <div>
                      <Text style={{ color: '#e2e8f0', fontSize: '14px', display: 'block' }}>All Inquiries</Text>
                      <Text style={{ color: 'white', fontSize: '16px', fontWeight: '600' }}>
                        Enterprise, Research, Community & Support
                      </Text>
                    </div>
                  </div>
                </div>
              </Card>

              {/* Response Time */}
              <Card
                style={{
                  background: 'rgba(15, 23, 42, 0.8)',
                  border: '1px solid rgba(139, 92, 246, 0.3)',
                  borderRadius: '20px',
                  boxShadow: '0 8px 32px rgba(0, 0, 0, 0.3)'
                }}
                bodyStyle={{ padding: '30px' }}
              >
                <Title level={4} style={{ color: 'white', marginBottom: '16px' }}>
                  📞 Response Time
                </Title>
                <Paragraph style={{ color: '#cbd5e1', margin: 0, lineHeight: '1.6' }}>
                  <strong style={{ color: '#8b5cf6' }}>Enterprise Inquiries:</strong> Within 4 hours<br />
                  <strong style={{ color: '#8b5cf6' }}>Research Partnerships:</strong> Within 24 hours<br />
                  <strong style={{ color: '#8b5cf6' }}>General Questions:</strong> Within 48 hours
                </Paragraph>
              </Card>

              {/* Status Notice */}
              <Card
                style={{
                  background: 'rgba(239, 68, 68, 0.1)',
                  border: '1px solid rgba(239, 68, 68, 0.3)',
                  borderRadius: '20px',
                  boxShadow: '0 8px 32px rgba(0, 0, 0, 0.3)'
                }}
                bodyStyle={{ padding: '30px' }}
              >
                <Title level={4} style={{ color: '#ef4444', marginBottom: '16px' }}>
                  🧪 Project Status
                </Title>
                <Paragraph style={{ color: '#cbd5e1', margin: 0, lineHeight: '1.6' }}>
                  Parvyom is currently in <strong style={{ color: '#ef4444' }}>experimental phase</strong>. 
                  We're seeking research partners and pilot participants to help evolve this technology 
                  into production-ready infrastructure.
                </Paragraph>
              </Card>
            </div>
          </Col>
        </Row>
      </div>
    </div>
  );
};

export default Contact;
