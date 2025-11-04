import React, { useState, useEffect } from 'react';
import {
  Card,
  Row,
  Col,
  Button,
  Typography,
  Space,
  List,
  Avatar,
  Tag,
  Input,
  Select,
  Divider,
  Spin,
  Empty
} from 'antd';
import {
  MessageOutlined,
  LikeOutlined,
  EyeOutlined,
  PlusOutlined,
  SearchOutlined,
  FireOutlined,
  ClockCircleOutlined,
  CheckCircleOutlined,
  CommentOutlined
} from '@ant-design/icons';
import { useNavigate } from 'react-router-dom';

const { Title, Text, Paragraph } = Typography;
const { Search } = Input;
const { Option } = Select;

interface ForumPost {
  id: string;
  title: string;
  content: string;
  author: string;
  avatar?: string;
  category: string;
  tags: string[];
  views: number;
  likes: number;
  comments: number;
  created_at: string;
  status: 'open' | 'answered' | 'closed';
}

const CommunityForum: React.FC = () => {
  const navigate = useNavigate();
  const [loading, setLoading] = useState(true);
  const [posts, setPosts] = useState<ForumPost[]>([]);
  const [filter, setFilter] = useState('all');
  const [category, setCategory] = useState('all');

  useEffect(() => {
    loadForumPosts();
  }, [filter, category]);

  const loadForumPosts = async () => {
    setLoading(true);
    
    try {
      // Real backend integration - Component 9 (Web Interface)
      const API_BASE = process.env.REACT_APP_API_URL || 'http://146.190.74.139:8080';
      
      const response = await fetch(`${API_BASE}/api/community/posts`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json'
        }
      });
      
      if (response.ok) {
        const data = await response.json();
        
        if (data.data?.posts) {
          setPosts(data.data.posts);
        } else {
          // Fallback demo data for initial display
          setPosts(getDemoPosts());
        }
      } else {
        // Fallback to demo data
        setPosts(getDemoPosts());
      }
    } catch (error) {
      console.error('Failed to load forum posts:', error);
      setPosts(getDemoPosts());
    } finally {
      setLoading(false);
    }
  };

  const getDemoPosts = (): ForumPost[] => [
    {
      id: '1',
      title: 'How to Deploy BPI Node on Raspberry Pi',
      content: 'Step-by-step guide for deploying a BPI Immutable OS node on Raspberry Pi 4 with 8GB RAM...',
      author: 'alice@pravyom.wallet',
      category: 'Tutorials',
      tags: ['bpi-os', 'raspberry-pi', 'deployment'],
      views: 1247,
      likes: 89,
      comments: 23,
      created_at: new Date(Date.now() - 86400000 * 2).toISOString(),
      status: 'answered'
    },
    {
      id: '2',
      title: 'Understanding the 4-Coin Economy (GEN/NEX/FLX/AUR)',
      content: 'Comprehensive explanation of the autonomous economy model with mother-daughter coin distribution...',
      author: 'bob@pravyom.wallet',
      category: 'Documentation',
      tags: ['economy', 'coins', 'autonomous'],
      views: 892,
      likes: 67,
      comments: 15,
      created_at: new Date(Date.now() - 86400000 * 5).toISOString(),
      status: 'open'
    },
    {
      id: '3',
      title: 'BSO-K8 vs Docker/Kubernetes - Performance Comparison',
      content: 'Real-world benchmarks showing BSO-K8 orchestrator performance compared to traditional container systems...',
      author: 'charlie@pravyom.wallet',
      category: 'Research',
      tags: ['bso-k8', 'performance', 'benchmarks'],
      views: 2341,
      likes: 156,
      comments: 45,
      created_at: new Date(Date.now() - 86400000 * 7).toISOString(),
      status: 'answered'
    }
  ];

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'answered':
        return <CheckCircleOutlined style={{ color: '#52c41a' }} />;
      case 'closed':
        return <ClockCircleOutlined style={{ color: '#d9d9d9' }} />;
      default:
        return <MessageOutlined style={{ color: '#1890ff' }} />;
    }
  };

  const getStatusTag = (status: string) => {
    const colors: Record<string, string> = {
      open: 'blue',
      answered: 'green',
      closed: 'default'
    };
    return <Tag color={colors[status]}>{status.toUpperCase()}</Tag>;
  };

  return (
    <div style={{ padding: '24px', maxWidth: '1400px', margin: '0 auto' }}>
      {/* Header */}
      <div style={{ marginBottom: 24 }}>
        <Space align="center" style={{ width: '100%', justifyContent: 'space-between' }}>
          <Space>
            <MessageOutlined style={{ fontSize: '32px', color: '#1890ff' }} />
            <div>
              <Title level={2} style={{ margin: 0 }}>Community Forum</Title>
              <Text type="secondary">Share knowledge and discuss BPI OS</Text>
            </div>
          </Space>
          <Button 
            type="primary" 
            size="large"
            icon={<PlusOutlined />}
            onClick={() => navigate('/community/new-post')}
          >
            Create Post
          </Button>
        </Space>
      </div>

      {/* Filters */}
      <Card style={{ marginBottom: 24 }}>
        <Row gutter={16}>
          <Col xs={24} sm={12} md={8}>
            <Search
              placeholder="Search posts..."
              allowClear
              size="large"
              prefix={<SearchOutlined />}
            />
          </Col>
          <Col xs={24} sm={6} md={4}>
            <Select
              value={category}
              onChange={setCategory}
              size="large"
              style={{ width: '100%' }}
            >
              <Option value="all">All Categories</Option>
              <Option value="tutorials">Tutorials</Option>
              <Option value="documentation">Documentation</Option>
              <Option value="research">Research</Option>
              <Option value="support">Support</Option>
              <Option value="announcements">Announcements</Option>
            </Select>
          </Col>
          <Col xs={24} sm={6} md={4}>
            <Select
              value={filter}
              onChange={setFilter}
              size="large"
              style={{ width: '100%' }}
            >
              <Option value="all">All Posts</Option>
              <Option value="trending">🔥 Trending</Option>
              <Option value="recent">🕐 Recent</Option>
              <Option value="unanswered">❓ Unanswered</Option>
            </Select>
          </Col>
        </Row>
      </Card>

      {/* Posts List */}
      {loading ? (
        <div style={{ textAlign: 'center', padding: '60px 0' }}>
          <Spin size="large" />
          <Paragraph style={{ marginTop: 16 }}>Loading forum posts...</Paragraph>
        </div>
      ) : posts.length === 0 ? (
        <Card>
          <Empty
            description="No posts found"
            image={Empty.PRESENTED_IMAGE_SIMPLE}
          >
            <Button type="primary" onClick={() => navigate('/community/new-post')}>
              Create First Post
            </Button>
          </Empty>
        </Card>
      ) : (
        <List
          itemLayout="vertical"
          size="large"
          dataSource={posts}
          renderItem={(post) => (
            <Card
              hoverable
              style={{ marginBottom: 16, cursor: 'pointer' }}
              onClick={() => navigate(`/community/post/${post.id}`)}
            >
              <List.Item
                key={post.id}
                actions={[
                  <Space key="views">
                    <EyeOutlined />
                    <Text>{post.views}</Text>
                  </Space>,
                  <Space key="likes">
                    <LikeOutlined />
                    <Text>{post.likes}</Text>
                  </Space>,
                  <Space key="comments">
                    <CommentOutlined />
                    <Text>{post.comments}</Text>
                  </Space>,
                ]}
              >
                <List.Item.Meta
                  avatar={
                    <Avatar 
                      size={48}
                      style={{ backgroundColor: '#1890ff' }}
                    >
                      {post.author[0].toUpperCase()}
                    </Avatar>
                  }
                  title={
                    <Space direction="vertical" size={4} style={{ width: '100%' }}>
                      <Space>
                        {getStatusIcon(post.status)}
                        <Title level={4} style={{ margin: 0 }}>
                          {post.title}
                        </Title>
                      </Space>
                      <Space size={8}>
                        {getStatusTag(post.status)}
                        <Tag>{post.category}</Tag>
                        {post.tags.map(tag => (
                          <Tag key={tag} color="blue">{tag}</Tag>
                        ))}
                      </Space>
                    </Space>
                  }
                  description={
                    <Space direction="vertical" size={4} style={{ width: '100%' }}>
                      <Text type="secondary">
                        {post.content.substring(0, 150)}...
                      </Text>
                      <Space>
                        <Text type="secondary" style={{ fontSize: '12px' }}>
                          by {post.author}
                        </Text>
                        <Text type="secondary" style={{ fontSize: '12px' }}>
                          • {new Date(post.created_at).toLocaleDateString()}
                        </Text>
                      </Space>
                    </Space>
                  }
                />
              </List.Item>
            </Card>
          )}
        />
      )}

      {/* Help Section */}
      <Card style={{ marginTop: 24, background: '#f0f2f5', border: 'none' }}>
        <Space direction="vertical" size="small" style={{ width: '100%' }}>
          <Title level={5}>Community Guidelines</Title>
          <ul style={{ marginBottom: 0, paddingLeft: 20 }}>
            <li>Be respectful and constructive in discussions</li>
            <li>Search before posting to avoid duplicates</li>
            <li>Use appropriate tags and categories</li>
            <li>Share code examples and documentation links when helpful</li>
          </ul>
        </Space>
      </Card>
    </div>
  );
};

export default CommunityForum;
