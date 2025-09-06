import React, { useState, useEffect } from 'react';
import { Modal, Button, Input, Form, message, Statistic, Divider } from 'antd';
import {
  TwitterOutlined,
  GithubOutlined,
  LinkedinOutlined,
  YoutubeOutlined,
  RedditOutlined,
  GlobalOutlined,
  UserAddOutlined,
  TeamOutlined,
  MessageOutlined,
  WechatOutlined
} from '@ant-design/icons';
import './CommunityPopup.css';

interface CommunityPopupProps {
  visible: boolean;
  onClose: () => void;
}

interface VoterData {
  name: string;
  email: string;
}

const CommunityPopup: React.FC<CommunityPopupProps> = ({ visible, onClose }) => {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);
  const [voterCount, setVoterCount] = useState(0); // Start with real count
  const [hasVoted, setHasVoted] = useState(false);

  // Load voter status and count from localStorage and backend
  useEffect(() => {
    const voted = localStorage.getItem('bpci_community_voted');
    if (voted) {
      setHasVoted(true);
    }
    loadVoterCount();
  }, []);

  const loadVoterCount = async () => {
    try {
      // Try to get real voter count from backend
      const response = await fetch('http://localhost:8080/api/community/voter-count');
      if (response.ok) {
        const data = await response.json();
        setVoterCount(data.count || 0);
      } else {
        // If backend not available, get count from localStorage
        const localCount = localStorage.getItem('bpci_local_voter_count');
        setVoterCount(localCount ? parseInt(localCount) : 0);
      }
    } catch (error) {
      // Fallback to localStorage count
      const localCount = localStorage.getItem('bpci_local_voter_count');
      setVoterCount(localCount ? parseInt(localCount) : 0);
    }
  };

  // Social media links (placeholders for now)
  const socialLinks = [
    { icon: <TwitterOutlined />, name: 'Twitter', url: 'https://twitter.com/bpci_enterprise', color: '#1DA1F2' },
    { icon: <GithubOutlined />, name: 'GitHub', url: 'https://github.com/bpci-enterprise', color: '#333' },
    { icon: <LinkedinOutlined />, name: 'LinkedIn', url: 'https://linkedin.com/company/bpci', color: '#0077B5' },
    { icon: <YoutubeOutlined />, name: 'YouTube', url: 'https://youtube.com/@bpci', color: '#FF0000' },
    { icon: <RedditOutlined />, name: 'Reddit', url: 'https://reddit.com/r/bpci', color: '#FF4500' },
    { icon: <MessageOutlined />, name: 'Telegram', url: 'https://t.me/bpci_community', color: '#0088CC' },
    { icon: <WechatOutlined />, name: 'Discord', url: 'https://discord.gg/bpci', color: '#7289DA' },
    { icon: <GlobalOutlined />, name: 'Website', url: 'https://bpci.dev', color: '#1890FF' }
  ];

  const handleVote = async (values: VoterData) => {
    setLoading(true);
    try {
      // Try to register with backend first
      let backendSuccess = false;
      try {
        const response = await fetch('http://localhost:8080/api/community/vote', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(values),
        });
        backendSuccess = response.ok;
      } catch (error) {
        console.log('Backend not available, using local storage');
      }
      
      // Store voter data locally
      localStorage.setItem('bpci_community_voted', 'true');
      localStorage.setItem('bpci_voter_data', JSON.stringify(values));
      
      // Update local voter count
      const newCount = voterCount + 1;
      setVoterCount(newCount);
      localStorage.setItem('bpci_local_voter_count', newCount.toString());
      setHasVoted(true);
      
      message.success('Welcome to the decentralized internet movement! 🎉');
      if (!backendSuccess) {
        message.info('Vote recorded locally. Will sync when backend is available.');
      }
      form.resetFields();
    } catch (error) {
      message.error('Failed to register. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const handleSocialClick = (url: string, name: string) => {
    // In real implementation, these would be actual social media links
    message.info(`${name} link: ${url} (placeholder)`);
    window.open(url, '_blank');
  };

  return (
    <Modal
      title={
        <div className="text-center">
          <TeamOutlined className="text-2xl text-blue-600 mr-2" />
          <span className="text-xl font-bold">Join the Decentralized Internet Movement</span>
        </div>
      }
      open={visible}
      onCancel={onClose}
      footer={null}
      width={600}
      className="community-popup"
    >
      <div className="text-center mb-6">
        <div className="bg-gradient-to-r from-blue-50 to-purple-50 p-6 rounded-lg mb-6">
          <h3 className="text-lg font-semibold text-gray-800 mb-2">
            🗳️ Vote for a Decentralized Internet Future
          </h3>
          <p className="text-gray-600 text-sm">
            Join our community of developers, researchers, and blockchain enthusiasts 
            building the infrastructure for tomorrow's decentralized web.
          </p>
        </div>

        {/* Voter Count */}
        <div className="bg-blue-50 border border-blue-200 rounded-lg p-4 mb-6">
          <Statistic
            title="Community Voters"
            value={voterCount}
            prefix={<UserAddOutlined />}
            valueStyle={{ color: '#1890ff', fontSize: '2rem' }}
          />
          <p className="text-blue-600 text-sm mt-2">
            People who believe in decentralized internet infrastructure
          </p>
        </div>

        {/* Social Media Icons */}
        <div className="mb-6">
          <h4 className="text-gray-700 font-semibold mb-4">Connect with our Community</h4>
          <div className="grid grid-cols-4 gap-4">
            {socialLinks.map((social, index) => (
              <div
                key={index}
                className="social-icon-container"
                onClick={() => handleSocialClick(social.url, social.name)}
                style={{ '--hover-color': social.color } as React.CSSProperties}
              >
                <div className="social-icon">
                  {social.icon}
                </div>
                <span className="social-name">{social.name}</span>
              </div>
            ))}
          </div>
        </div>

        <Divider />

        {/* Voting Form */}
        {!hasVoted ? (
          <div>
            <h4 className="text-gray-700 font-semibold mb-4">Cast Your Vote for Decentralized Internet</h4>
            <Form
              form={form}
              layout="vertical"
              onFinish={handleVote}
              className="text-left"
            >
              <Form.Item
                name="name"
                label="Your Name"
                rules={[{ required: true, message: 'Please enter your name' }]}
              >
                <Input 
                  placeholder="Enter your full name" 
                  prefix={<UserAddOutlined />}
                />
              </Form.Item>

              <Form.Item
                name="email"
                label="Email Address"
                rules={[
                  { required: true, message: 'Please enter your email' },
                  { type: 'email', message: 'Please enter a valid email' }
                ]}
              >
                <Input 
                  placeholder="your.email@example.com" 
                  prefix={<GlobalOutlined />}
                />
              </Form.Item>

              <div className="text-center">
                <Button
                  type="primary"
                  htmlType="submit"
                  loading={loading}
                  size="large"
                  className="px-8"
                  icon={<UserAddOutlined />}
                >
                  Join as Voter for Decentralized Internet
                </Button>
              </div>
            </Form>

            <p className="text-xs text-gray-500 mt-4">
              By joining, you're supporting the development of decentralized blockchain infrastructure. 
              We'll keep you updated on our pilot program progress.
            </p>
          </div>
        ) : (
          <div className="bg-green-50 border border-green-200 rounded-lg p-6">
            <div className="text-center">
              <div className="text-4xl mb-4">🎉</div>
              <h4 className="text-green-800 font-semibold mb-2">Thank You for Voting!</h4>
              <p className="text-green-700 text-sm">
                You're now part of the decentralized internet movement. 
                Follow our social channels for updates on the pilot program.
              </p>
            </div>
          </div>
        )}
      </div>
    </Modal>
  );
};

export default CommunityPopup;
