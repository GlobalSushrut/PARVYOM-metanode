/**
 * BPI OS Downloads Page
 * Dedicated page for downloading BPI OS Core binaries
 * Linux downloads available, other platforms show "Coming as per maturity"
 */

import React from 'react';
import {
  Layout,
  Typography,
  Space,
  Card,
  Row,
  Col,
  Alert,
  Divider,
  Timeline,
  Tag,
} from 'antd';
import {
  DownloadOutlined,
  RocketOutlined,
  SafetyCertificateOutlined,
  InfoCircleOutlined,
  CheckCircleOutlined,
  ClockCircleOutlined,
} from '@ant-design/icons';
import BpiOSDownloader from '../components/BPI/BpiOSDownloader';

const { Content } = Layout;
const { Title, Paragraph, Text } = Typography;

export const DownloadsPage: React.FC = () => {
  return (
    <Layout>
      <Content style={{ padding: '24px', maxWidth: '1200px', margin: '0 auto' }}>
        {/* Page Header */}
        <div style={{ textAlign: 'center', marginBottom: '32px' }}>
          <Space direction="vertical" size="large">
            <div>
              <Title level={1} style={{ margin: 0 }}>
                <DownloadOutlined /> BPI OS Downloads
              </Title>
              <Paragraph style={{ fontSize: '18px', color: '#666', marginTop: '8px' }}>
                Get the latest BPI Blockchain Operating System binaries for your platform
              </Paragraph>
            </div>
            
            <Space size="large">
              <Tag color="green" icon={<CheckCircleOutlined />} style={{ padding: '4px 12px', fontSize: '14px' }}>
                Production Ready
              </Tag>
              <Tag color="blue" icon={<SafetyCertificateOutlined />} style={{ padding: '4px 12px', fontSize: '14px' }}>
                Cryptographically Verified
              </Tag>
              <Tag color="purple" icon={<RocketOutlined />} style={{ padding: '4px 12px', fontSize: '14px' }}>
                Enterprise Grade
              </Tag>
            </Space>
          </Space>
        </div>

        {/* Platform Availability Notice */}
        <Alert
          message="Platform Availability"
          description={
            <div>
              <Paragraph>
                <strong>Linux x64:</strong> Production-ready builds available for immediate download.
              </Paragraph>
              <Paragraph>
                <strong>macOS & Windows:</strong> Builds are coming as per platform maturity and community demand.
                Join our community to stay updated on release schedules.
              </Paragraph>
            </div>
          }
          type="info"
          icon={<InfoCircleOutlined />}
          style={{ marginBottom: '24px' }}
          showIcon
        />

        {/* Main Downloads Component */}
        <BpiOSDownloader 
          showStats={true}
          showInstructions={true}
          compactMode={false}
        />

        {/* Additional Information */}
        <Row gutter={[24, 24]} style={{ marginTop: '32px' }}>
          <Col xs={24} lg={12}>
            <Card title="System Requirements" size="small">
              <Timeline
                items={[
                  {
                    color: 'green',
                    children: (
                      <div>
                        <Text strong>Operating System</Text>
                        <br />
                        <Text type="secondary">Linux (Ubuntu 20.04+, CentOS 8+, or equivalent)</Text>
                      </div>
                    ),
                  },
                  {
                    color: 'blue',
                    children: (
                      <div>
                        <Text strong>Architecture</Text>
                        <br />
                        <Text type="secondary">x86_64 (64-bit)</Text>
                      </div>
                    ),
                  },
                  {
                    color: 'purple',
                    children: (
                      <div>
                        <Text strong>Memory</Text>
                        <br />
                        <Text type="secondary">Minimum 4GB RAM (8GB+ recommended)</Text>
                      </div>
                    ),
                  },
                  {
                    color: 'orange',
                    children: (
                      <div>
                        <Text strong>Storage</Text>
                        <br />
                        <Text type="secondary">Minimum 10GB free space (SSD recommended)</Text>
                      </div>
                    ),
                  },
                ]}
              />
            </Card>
          </Col>

          <Col xs={24} lg={12}>
            <Card title="What's Included" size="small">
              <Timeline
                items={[
                  {
                    color: 'green',
                    children: (
                      <div>
                        <Text strong>BPI OS Core Runtime</Text>
                        <br />
                        <Text type="secondary">Complete blockchain operating system</Text>
                      </div>
                    ),
                  },
                  {
                    color: 'blue',
                    children: (
                      <div>
                        <Text strong>BPCI Integration</Text>
                        <br />
                        <Text type="secondary">Built-in BPCI network connectivity</Text>
                      </div>
                    ),
                  },
                  {
                    color: 'purple',
                    children: (
                      <div>
                        <Text strong>Wallet & Mining</Text>
                        <br />
                        <Text type="secondary">Integrated wallet and PoE mining capabilities</Text>
                      </div>
                    ),
                  },
                  {
                    color: 'orange',
                    children: (
                      <div>
                        <Text strong>Developer Tools</Text>
                        <br />
                        <Text type="secondary">CLI tools and development utilities</Text>
                      </div>
                    ),
                  },
                ]}
              />
            </Card>
          </Col>
        </Row>

        <Divider />

        {/* Platform Roadmap */}
        <Card title="Platform Roadmap" style={{ marginTop: '24px' }}>
          <Timeline
            mode="left"
            items={[
              {
                color: 'green',
                dot: <CheckCircleOutlined style={{ fontSize: '16px' }} />,
                children: (
                  <div>
                    <Text strong>Linux x64 - Production Ready</Text>
                    <br />
                    <Text type="secondary">Available now with full feature support</Text>
                  </div>
                ),
              },
              {
                color: 'blue',
                dot: <ClockCircleOutlined style={{ fontSize: '16px' }} />,
                children: (
                  <div>
                    <Text strong>macOS (Intel & Apple Silicon) - Coming Soon</Text>
                    <br />
                    <Text type="secondary">Release planned as per platform maturity</Text>
                  </div>
                ),
              },
              {
                color: 'orange',
                dot: <ClockCircleOutlined style={{ fontSize: '16px' }} />,
                children: (
                  <div>
                    <Text strong>Windows x64 - Coming Soon</Text>
                    <br />
                    <Text type="secondary">Release planned as per platform maturity</Text>
                  </div>
                ),
              },
              {
                color: 'purple',
                dot: <ClockCircleOutlined style={{ fontSize: '16px' }} />,
                children: (
                  <div>
                    <Text strong>Linux ARM64 - Future Release</Text>
                    <br />
                    <Text type="secondary">Planned for ARM-based servers and devices</Text>
                  </div>
                ),
              },
            ]}
          />
        </Card>

        {/* Support Information */}
        <Alert
          message="Need Help?"
          description={
            <div>
              <Paragraph>
                For installation support, documentation, and community discussions:
              </Paragraph>
              <Space direction="vertical">
                <Text>📚 <strong>Documentation:</strong> <a href="https://pravyom.com/docs" target="_blank" rel="noopener noreferrer">pravyom.com/docs</a></Text>
                <Text>💬 <strong>Community:</strong> <a href="https://pravyom.com/community" target="_blank" rel="noopener noreferrer">pravyom.com/community</a></Text>
                <Text>🐛 <strong>Issues:</strong> <a href="https://github.com/GlobalSushrut/PARVYOM-metanode/issues" target="_blank" rel="noopener noreferrer">GitHub Issues</a></Text>
              </Space>
            </div>
          }
          type="success"
          style={{ marginTop: '24px' }}
          showIcon
        />
      </Content>
    </Layout>
  );
};

export default DownloadsPage;
