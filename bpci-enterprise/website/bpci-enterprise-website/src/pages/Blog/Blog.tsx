import React, { useState, useEffect } from 'react';
import { 
  Typography, 
  Card, 
  Button, 
  Input, 
  Form, 
  message, 
  Modal, 
  Avatar, 
  Badge,
  Tooltip,
  Spin,
  Empty,
  Select,
  Tabs,
  Tag,
  Divider
} from 'antd';
import { 
  PlusOutlined,
  HeartOutlined,
  MessageOutlined,
  UserOutlined,
  ClockCircleOutlined,
  TagOutlined,
  BugOutlined,
  FileTextOutlined,
  ExperimentOutlined,
  GlobalOutlined,
  WalletOutlined,
  CommentOutlined,
  ShareAltOutlined
} from '@ant-design/icons';
import type { BlogPost, CreatePostRequest, CreateCommentRequest, BlogFilters } from '../../types/blog';
import { blogService } from '../../services/blogService';
import { apiService } from '../../services/api';

const { Title, Paragraph, Text } = Typography;
const { TextArea } = Input;

const { Option } = Select;
const { TabPane } = Tabs;

const Blog: React.FC = () => {
  const [posts, setPosts] = useState<BlogPost[]>([]);
  const [loading, setLoading] = useState(true);
  const [createModalVisible, setCreateModalVisible] = useState(false);
  const [selectedPost, setSelectedPost] = useState<BlogPost | null>(null);
  const [postDetailVisible, setPostDetailVisible] = useState(false);
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [userWallet, setUserWallet] = useState<string>('');
  const [filters, setFilters] = useState<BlogFilters>({ sortBy: 'newest' });
  const [form] = Form.useForm();
  const [commentForm] = Form.useForm();

  // Check authentication status
  useEffect(() => {
    checkAuthStatus();
    loadPosts();
  }, []);

  const checkAuthStatus = async () => {
    try {
      const user = await apiService.getCurrentUser();
      if (user && (user as any).wallet_address) {
        setIsAuthenticated(true);
        setUserWallet((user as any).wallet_address);
      }
    } catch (error) {
      setIsAuthenticated(false);
    }
  };

  const loadPosts = async () => {
    try {
      setLoading(true);
      const fetchedPosts = await blogService.getPosts(filters);
      setPosts(fetchedPosts);
    } catch (error) {
      message.error('Failed to load posts');
    } finally {
      setLoading(false);
    }
  };

  const handleCreatePost = async (values: CreatePostRequest) => {
    if (!isAuthenticated) {
      message.error('Please authenticate with your BPCI wallet first');
      return;
    }

    try {
      const newPost = await blogService.createPost(values);
      setPosts([newPost, ...posts]);
      setCreateModalVisible(false);
      form.resetFields();
      message.success('Post created successfully!');
      
      if (values.autoPost) {
        message.info('Auto-posting to social media platforms...');
      }
    } catch (error) {
      message.error('Failed to create post');
    }
  };

  const handleLikePost = async (postId: string) => {
    if (!isAuthenticated) {
      message.error('Please authenticate with your BPCI wallet first');
      return;
    }

    try {
      await blogService.likePost(postId);
      setPosts(posts.map(post => 
        post.id === postId 
          ? { ...post, likes: post.likes + 1 }
          : post
      ));
    } catch (error) {
      message.error('Failed to like post');
    }
  };

  const handleCreateComment = async (values: CreateCommentRequest) => {
    if (!isAuthenticated) {
      message.error('Please authenticate with your BPCI wallet first');
      return;
    }

    try {
      const newComment = await blogService.createComment(values);
      if (selectedPost) {
        setSelectedPost({
          ...selectedPost,
          comments: [...selectedPost.comments, newComment]
        });
      }
      commentForm.resetFields();
      message.success('Comment added successfully!');
    } catch (error) {
      message.error('Failed to add comment');
    }
  };

  const getPostTypeIcon = (type: string) => {
    switch (type) {
      case 'bug_report': return <BugOutlined style={{ color: '#ff4d4f' }} />;
      case 'documentation': return <FileTextOutlined style={{ color: '#1890ff' }} />;
      case 'experience': return <ExperimentOutlined style={{ color: '#52c41a' }} />;
      default: return <GlobalOutlined style={{ color: '#722ed1' }} />;
    }
  };

  const getPostTypeColor = (type: string) => {
    switch (type) {
      case 'bug_report': return 'red';
      case 'documentation': return 'blue';
      case 'experience': return 'green';
      default: return 'purple';
    }
  };

  const formatTimeAgo = (dateString: string) => {
    const date = new Date(dateString);
    const now = new Date();
    const diffInHours = Math.floor((now.getTime() - date.getTime()) / (1000 * 60 * 60));
    
    if (diffInHours < 1) return 'Just now';
    if (diffInHours < 24) return `${diffInHours}h ago`;
    if (diffInHours < 168) return `${Math.floor(diffInHours / 24)}d ago`;
    return date.toLocaleDateString();
  };

  const AuthenticationPrompt = () => (
    <Card className="text-center p-8 mb-6" style={{ background: 'linear-gradient(135deg, #f0f9ff 0%, #e0e7ff 100%)' }}>
      <WalletOutlined className="text-4xl text-blue-600 mb-4" />
      <Title level={3} className="text-blue-800 mb-4">BPCI Wallet Authentication Required</Title>
      <Paragraph className="text-blue-700 mb-6">
        To post experiences, documentation, bug reports, and interact with the community, 
        you need to authenticate with your BPCI wallet.
      </Paragraph>
      <Button 
        type="primary" 
        size="large" 
        icon={<WalletOutlined />}
        onClick={() => window.location.href = '/auth'}
      >
        Connect BPCI Wallet
      </Button>
    </Card>
  );

  return (
    <div className="max-w-6xl mx-auto px-4 py-8">
      {/* Header */}
      <div className="text-center mb-8">
        <Title level={1} className="mb-4">Developer Community & Blog</Title>
        <Paragraph className="text-lg text-gray-600 mb-6">
          Share experiences, document findings, report bugs, and connect with fellow BPCI developers
        </Paragraph>
        
        {isAuthenticated ? (
          <div className="flex justify-center items-center gap-4 mb-6">
            <Badge status="success" text={`Connected: ${userWallet.slice(0, 8)}...${userWallet.slice(-6)}`} />
            <Button 
              type="primary" 
              icon={<PlusOutlined />} 
              onClick={() => setCreateModalVisible(true)}
              size="large"
            >
              Create Post
            </Button>
          </div>
        ) : (
          <AuthenticationPrompt />
        )}
      </div>

      {/* Filters and Tabs */}
      <Card className="mb-6">
        <Tabs 
          defaultActiveKey="all" 
          onChange={(key) => {
            const newFilters = { ...filters };
            if (key !== 'all') {
              newFilters.type = key as any;
            } else {
              delete newFilters.type;
            }
            setFilters(newFilters);
            loadPosts();
          }}
        >
          <TabPane tab="All Posts" key="all" />
          <TabPane tab={<span><ExperimentOutlined /> Experiences</span>} key="experience" />
          <TabPane tab={<span><FileTextOutlined /> Documentation</span>} key="documentation" />
          <TabPane tab={<span><BugOutlined /> Bug Reports</span>} key="bug_report" />
          <TabPane tab={<span><GlobalOutlined /> General</span>} key="general" />
        </Tabs>
        
        <div className="flex justify-between items-center mt-4">
          <Select
            value={filters.sortBy}
            onChange={(value) => {
              setFilters({ ...filters, sortBy: value });
              loadPosts();
            }}
            style={{ width: 200 }}
          >
            <Option value="newest">Newest First</Option>
            <Option value="oldest">Oldest First</Option>
            <Option value="most_liked">Most Liked</Option>
            <Option value="most_commented">Most Commented</Option>
          </Select>
          
          <Text type="secondary">{posts.length} posts</Text>
        </div>
      </Card>

      {/* Posts List */}
      <Spin spinning={loading}>
        {posts.length === 0 ? (
          <Empty 
            description="No posts yet. Be the first to share your experience!"
            image={Empty.PRESENTED_IMAGE_SIMPLE}
          />
        ) : (
          <div className="space-y-6">
            {posts.map((post) => (
              <Card 
                key={post.id} 
                className="hover:shadow-lg transition-shadow"
                actions={[
                  <Tooltip title="Like">
                    <Button 
                      type="text" 
                      icon={<HeartOutlined />} 
                      onClick={() => handleLikePost(post.id)}
                    >
                      {post.likes}
                    </Button>
                  </Tooltip>,
                  <Tooltip title="Comments">
                    <Button 
                      type="text" 
                      icon={<CommentOutlined />}
                      onClick={() => {
                        setSelectedPost(post);
                        setPostDetailVisible(true);
                      }}
                    >
                      {post.comments.length}
                    </Button>
                  </Tooltip>,
                  <Tooltip title="Share">
                    <Button type="text" icon={<ShareAltOutlined />} />
                  </Tooltip>,
                  <Tooltip title="Message Author">
                    <Button 
                      type="text" 
                      icon={<MessageOutlined />}
                      disabled={!isAuthenticated}
                    />
                  </Tooltip>
                ]}
              >
                <div className="flex items-start space-x-4">
                  <Avatar icon={<UserOutlined />} size="large" />
                  <div className="flex-1">
                    <div className="flex items-center space-x-2 mb-2">
                      <Text strong>{post.author.name}</Text>
                      <Text type="secondary" className="text-sm">
                        {post.author.walletAddress.slice(0, 8)}...{post.author.walletAddress.slice(-6)}
                      </Text>
                      <Tag color={getPostTypeColor(post.type)} icon={getPostTypeIcon(post.type)}>
                        {post.type.replace('_', ' ').toUpperCase()}
                      </Tag>
                      {post.autoPosted && (
                        <Tag color="blue" icon={<ShareAltOutlined />}>AUTO-POSTED</Tag>
                      )}
                      <Text type="secondary" className="text-sm">
                        <ClockCircleOutlined /> {formatTimeAgo(post.createdAt)}
                      </Text>
                    </div>
                    
                    <Title level={4} className="mb-2">{post.title}</Title>
                    <Paragraph className="mb-3">
                      {post.content.length > 300 
                        ? `${post.content.substring(0, 300)}...` 
                        : post.content
                      }
                    </Paragraph>
                    
                    {post.tags.length > 0 && (
                      <div className="flex flex-wrap gap-1">
                        {post.tags.map((tag, index) => (
                          <Tag key={index} icon={<TagOutlined />} color="default">
                            {tag}
                          </Tag>
                        ))}
                      </div>
                    )}
                  </div>
                </div>
              </Card>
            ))}
          </div>
        )}
      </Spin>

      {/* Create Post Modal */}
      <Modal
        title="Create New Post"
        open={createModalVisible}
        onCancel={() => setCreateModalVisible(false)}
        footer={null}
        width={800}
      >
        <Form
          form={form}
          layout="vertical"
          onFinish={handleCreatePost}
        >
          <Form.Item
            name="title"
            label="Title"
            rules={[{ required: true, message: 'Please enter a title' }]}
          >
            <Input placeholder="Enter post title" />
          </Form.Item>

          <Form.Item
            name="type"
            label="Post Type"
            rules={[{ required: true, message: 'Please select post type' }]}
          >
            <Select placeholder="Select post type">
              <Option value="experience">
                <ExperimentOutlined /> Experience Share
              </Option>
              <Option value="documentation">
                <FileTextOutlined /> Documentation
              </Option>
              <Option value="bug_report">
                <BugOutlined /> Bug Report
              </Option>
              <Option value="general">
                <GlobalOutlined /> General Discussion
              </Option>
            </Select>
          </Form.Item>

          <Form.Item
            name="content"
            label="Content"
            rules={[{ required: true, message: 'Please enter content' }]}
          >
            <TextArea 
              rows={8} 
              placeholder="Share your experience, document your findings, or describe the bug..."
            />
          </Form.Item>

          <Form.Item
            name="tags"
            label="Tags"
          >
            <Select
              mode="tags"
              placeholder="Add tags (press Enter to add)"
              style={{ width: '100%' }}
            />
          </Form.Item>

          <Form.Item name="isPublic" valuePropName="checked" initialValue={true}>
            <div className="flex items-center space-x-4">
              <input type="checkbox" />
              <span>Make this post public</span>
            </div>
          </Form.Item>

          <Form.Item name="autoPost" valuePropName="checked">
            <div className="flex items-center space-x-4">
              <input type="checkbox" />
              <span>Auto-post to social media platforms</span>
            </div>
          </Form.Item>

          <div className="flex justify-end space-x-2">
            <Button onClick={() => setCreateModalVisible(false)}>
              Cancel
            </Button>
            <Button type="primary" htmlType="submit">
              Create Post
            </Button>
          </div>
        </Form>
      </Modal>

      {/* Post Detail Modal with Comments */}
      <Modal
        title={selectedPost?.title}
        open={postDetailVisible}
        onCancel={() => setPostDetailVisible(false)}
        footer={null}
        width={900}
      >
        {selectedPost && (
          <div>
            <div className="mb-6">
              <div className="flex items-center space-x-2 mb-4">
                <Avatar icon={<UserOutlined />} />
                <Text strong>{selectedPost.author.name}</Text>
                <Tag color={getPostTypeColor(selectedPost.type)}>
                  {selectedPost.type.replace('_', ' ').toUpperCase()}
                </Tag>
                <Text type="secondary">{formatTimeAgo(selectedPost.createdAt)}</Text>
              </div>
              <Paragraph>{selectedPost.content}</Paragraph>
              {selectedPost.tags.length > 0 && (
                <div className="flex flex-wrap gap-1 mt-4">
                  {selectedPost.tags.map((tag, index) => (
                    <Tag key={index}>{tag}</Tag>
                  ))}
                </div>
              )}
            </div>

            <Divider />

            {/* Comments Section */}
            <div>
              <Title level={4}>Comments ({selectedPost.comments.length})</Title>
              
              {isAuthenticated && (
                <Form
                  form={commentForm}
                  onFinish={(values) => handleCreateComment({
                    ...values,
                    postId: selectedPost.id
                  })}
                  className="mb-6"
                >
                  <Form.Item
                    name="content"
                    rules={[{ required: true, message: 'Please enter your comment' }]}
                  >
                    <TextArea 
                      rows={3} 
                      placeholder="Add a comment..."
                    />
                  </Form.Item>
                  <Form.Item>
                    <Button type="primary" htmlType="submit">
                      Add Comment
                    </Button>
                  </Form.Item>
                </Form>
              )}

              <div className="space-y-4">
                {selectedPost.comments.map((comment) => (
                  <Card key={comment.id} size="small">
                    <div className="flex items-start space-x-3">
                      <Avatar icon={<UserOutlined />} size="small" />
                      <div className="flex-1">
                        <div className="flex items-center space-x-2 mb-1">
                          <Text strong className="text-sm">{comment.author.name}</Text>
                          <Text type="secondary" className="text-xs">
                            {formatTimeAgo(comment.createdAt)}
                          </Text>
                        </div>
                        <Paragraph className="text-sm mb-2">{comment.content}</Paragraph>
                        <div className="flex items-center space-x-2">
                          <Button 
                            type="text" 
                            size="small" 
                            icon={<HeartOutlined />}
                          >
                            {comment.likes}
                          </Button>
                          <Button type="text" size="small">
                            Reply
                          </Button>
                        </div>
                      </div>
                    </div>
                  </Card>
                ))}
              </div>
            </div>
          </div>
        )}
      </Modal>
    </div>
  );
};

export default Blog;
