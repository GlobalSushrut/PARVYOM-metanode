/**
 * BPI OS Core Binary Downloader Component
 * Provides secure download functionality for BPI OS Core binaries with beautiful UI
 */

import React, { useState, useEffect } from 'react';
import {
  Card,
  Row,
  Col,
  Button,
  Typography,
  Space,
  Tag,
  Statistic,
  Alert,
  Modal,
  List,
  Divider,
  Tooltip,
  Progress,
  notification,
  Spin,
} from 'antd';
import {
  DownloadOutlined,
  SafetyOutlined,
  InfoCircleOutlined,
  CheckCircleOutlined,
  CodeOutlined,
  GlobalOutlined,
  ClockCircleOutlined,
  FileTextOutlined,
  SafetyCertificateOutlined,
} from '@ant-design/icons';
import { bpiDownloadService, type BpiBinaryInfo, type BpiDownloadStats } from '../../services/bpiDownloadService';

const { Title, Text, Paragraph } = Typography;

interface BpiOSDownloaderProps {
  showStats?: boolean;
  showInstructions?: boolean;
  compactMode?: boolean;
}

export const BpiOSDownloader: React.FC<BpiOSDownloaderProps> = ({
  showStats = true,
  showInstructions = true,
  compactMode = false,
}) => {
  const [binaries, setBinaries] = useState<BpiBinaryInfo[]>([]);
  const [stats, setStats] = useState<BpiDownloadStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [downloading, setDownloading] = useState<string | null>(null);
  const [instructionsVisible, setInstructionsVisible] = useState(false);
  const [selectedBinary, setSelectedBinary] = useState<BpiBinaryInfo | null>(null);

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    setLoading(true);
    try {
      const [binariesData, statsData] = await Promise.all([
        bpiDownloadService.getAvailableBinaries(),
        showStats ? bpiDownloadService.getDownloadStats() : Promise.resolve(null),
      ]);
      
      setBinaries(binariesData);
      setStats(statsData);
    } catch (error) {
      console.error('Error loading download data:', error);
      notification.error({
        message: 'Download Error',
        description: 'Failed to load download information. Please try again.',
      });
    } finally {
      setLoading(false);
    }
  };

  const handleDownload = async (binary: BpiBinaryInfo) => {
    setDownloading(binary.name);
    try {
      await bpiDownloadService.downloadBinary(binary);
      
      notification.success({
        message: 'Download Started',
        description: `${binary.name} ${binary.version} download has been initiated.`,
        duration: 4,
      });
      
      // Refresh stats after download
      if (showStats) {
        setTimeout(() => {
          bpiDownloadService.getDownloadStats().then(setStats);
        }, 1000);
      }
    } catch (error) {
      console.error('Download error:', error);
      notification.error({
        message: 'Download Failed',
        description: 'Failed to initiate download. Please try again.',
      });
    } finally {
      setDownloading(null);
    }
  };

  const showInstructionsModal = (binary: BpiBinaryInfo) => {
    setSelectedBinary(binary);
    setInstructionsVisible(true);
  };

  const getPlatformIcon = (platform: string) => {
    switch (platform.toLowerCase()) {
      case 'linux': return '🐧';
      case 'darwin': return '🍎';
      case 'windows': return '🪟';
      default: return '💻';
    }
  };

  const getPlatformColor = (platform: string) => {
    switch (platform.toLowerCase()) {
      case 'linux': return 'blue';
      case 'darwin': return 'purple';
      case 'windows': return 'green';
      default: return 'default';
    }
  };

  if (loading) {
    return (
      <Card>
        <div style={{ textAlign: 'center', padding: '40px' }}>
          <Spin size="large" />
          <div style={{ marginTop: '16px' }}>
            <Text>Loading BPI OS downloads...</Text>
          </div>
        </div>
      </Card>
    );
  }

  return (
    <div>
      {/* Header */}
      <Card>
        <Row align="middle" justify="space-between">
          <Col>
            <Space direction="vertical" size="small">
              <Title level={compactMode ? 4 : 3} style={{ margin: 0 }}>
                <DownloadOutlined /> BPI OS Core Downloads
              </Title>
              <Text type="secondary">
                Production-ready BPI Blockchain Operating System binaries
              </Text>
            </Space>
          </Col>
          <Col>
            <Space>
              <Tag color="green" icon={<CheckCircleOutlined />}>
                Production Ready
              </Tag>
              <Tag color="blue" icon={<SafetyCertificateOutlined />}>
                Verified Builds
              </Tag>
            </Space>
          </Col>
        </Row>
      </Card>

      {/* Download Statistics */}
      {showStats && stats && (
        <Card title="Download Statistics" style={{ marginTop: '16px' }}>
          <Row gutter={16}>
            <Col xs={12} sm={6}>
              <Statistic
                title="Total Downloads"
                value={stats.totalDownloads}
                prefix={<DownloadOutlined />}
              />
            </Col>
            <Col xs={12} sm={6}>
              <Statistic
                title="This Month"
                value={stats.monthlyDownloads}
                prefix={<ClockCircleOutlined />}
              />
            </Col>
            <Col xs={12} sm={6}>
              <Statistic
                title="This Week"
                value={stats.weeklyDownloads}
                prefix={<GlobalOutlined />}
              />
            </Col>
            <Col xs={12} sm={6}>
              <Statistic
                title="Today"
                value={stats.dailyDownloads}
                prefix={<CheckCircleOutlined />}
              />
            </Col>
          </Row>
        </Card>
      )}

      {/* Available Downloads */}
      <Card title="Available Downloads" style={{ marginTop: '16px' }}>
        <List
          dataSource={bpiDownloadService.getAllPlatforms()}
          renderItem={(platform) => {
            const binary = binaries.find(b => b.platform === platform.platform && b.architecture === platform.architecture);
            return (
              <List.Item
                actions={[
                  platform.available ? (
                    <Button
                      type="primary"
                      icon={<DownloadOutlined />}
                      loading={downloading === binary?.name}
                      onClick={() => binary && handleDownload(binary)}
                      size={compactMode ? 'small' : 'middle'}
                    >
                      Download
                    </Button>
                  ) : (
                    <Button
                      disabled
                      size={compactMode ? 'small' : 'middle'}
                    >
                      {platform.maturityStatus}
                    </Button>
                  ),
                  platform.available && showInstructions && binary && (
                    <Button
                      icon={<FileTextOutlined />}
                      onClick={() => showInstructionsModal(binary)}
                      size={compactMode ? 'small' : 'middle'}
                    >
                      Instructions
                    </Button>
                  ),
                ].filter(Boolean)}
              >
                <List.Item.Meta
                  avatar={
                    <div style={{ fontSize: '32px' }}>
                      {getPlatformIcon(platform.platform)}
                    </div>
                  }
                  title={
                    <Space>
                      <Text strong>BPI OS Core</Text>
                      <Tag color={getPlatformColor(platform.platform)}>
                        {platform.platform}-{platform.architecture}
                      </Tag>
                      {platform.available ? (
                        <Tag color="green">Production Ready</Tag>
                      ) : (
                        <Tag color="orange">{platform.maturityStatus}</Tag>
                      )}
                    </Space>
                  }
                  description={
                    <Space direction="vertical" size="small">
                      <Text>
                        {platform.available 
                          ? `Production-ready BPI OS Core binary for ${platform.platform} ${platform.architecture}`
                          : `BPI OS Core for ${platform.platform} ${platform.architecture} - ${platform.maturityStatus}`
                        }
                      </Text>
                      {platform.available && binary && (
                        <Space>
                          <Text type="secondary">
                            <SafetyOutlined /> Size: {bpiDownloadService.formatFileSize(binary.size)}
                          </Text>
                          <Text type="secondary">
                            <CodeOutlined /> MD5: {binary.md5.substring(0, 8)}...
                          </Text>
                          <Text type="secondary">
                            <ClockCircleOutlined /> Built: {new Date(binary.buildDate).toLocaleDateString()}
                          </Text>
                        </Space>
                      )}
                    </Space>
                  }
                />
              </List.Item>
            );
          }}
        />
      </Card>

      {/* Security Notice */}
      <Alert
        message="Security Notice"
        description={
          <div>
            <Paragraph>
              Always verify the integrity of downloaded binaries using the provided MD5 checksums.
              Only download from official sources to ensure security and authenticity.
            </Paragraph>
            <Space>
              <Text strong>Verification Command:</Text>
              <Text code>md5sum bpi-core-linux-x64</Text>
            </Space>
          </div>
        }
        type="info"
        icon={<SafetyCertificateOutlined />}
        style={{ marginTop: '16px' }}
      />

      {/* Installation Instructions Modal */}
      <Modal
        title={
          <Space>
            <FileTextOutlined />
            Installation Instructions
            {selectedBinary && (
              <Tag color={getPlatformColor(selectedBinary.platform)}>
                {selectedBinary.platform}-{selectedBinary.architecture}
              </Tag>
            )}
          </Space>
        }
        open={instructionsVisible}
        onCancel={() => setInstructionsVisible(false)}
        footer={[
          <Button key="close" onClick={() => setInstructionsVisible(false)}>
            Close
          </Button>,
        ]}
        width={800}
      >
        {selectedBinary && (
          <div>
            <Alert
              message="Quick Start Guide"
              description={`Follow these steps to install and run ${selectedBinary.name} on your system.`}
              type="info"
              style={{ marginBottom: '16px' }}
            />
            
            <Card>
              <pre style={{ 
                background: '#f6f8fa', 
                padding: '16px', 
                borderRadius: '6px',
                overflow: 'auto',
                fontSize: '13px',
                lineHeight: '1.45'
              }}>
                {bpiDownloadService.getInstallationInstructions(selectedBinary).join('\n')}
              </pre>
            </Card>

            <Divider />
            
            <Row gutter={16}>
              <Col span={12}>
                <Card size="small" title="Binary Info">
                  <Space direction="vertical" size="small">
                    <Text><strong>Version:</strong> {selectedBinary.version}</Text>
                    <Text><strong>Platform:</strong> {selectedBinary.platform}</Text>
                    <Text><strong>Architecture:</strong> {selectedBinary.architecture}</Text>
                    <Text><strong>Size:</strong> {bpiDownloadService.formatFileSize(selectedBinary.size)}</Text>
                  </Space>
                </Card>
              </Col>
              <Col span={12}>
                <Card size="small" title="Verification">
                  <Space direction="vertical" size="small">
                    <Text><strong>MD5:</strong> <Text code>{selectedBinary.md5}</Text></Text>
                    <Text><strong>Build Date:</strong> {new Date(selectedBinary.buildDate).toLocaleString()}</Text>
                  </Space>
                </Card>
              </Col>
            </Row>
          </div>
        )}
      </Modal>
    </div>
  );
};

export default BpiOSDownloader;
