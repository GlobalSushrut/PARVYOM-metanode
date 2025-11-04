import React, { useState, useEffect } from 'react';
import { Typography, Card, Button, Input, Modal, Avatar, message, Empty, Spin, Alert } from 'antd';
import { HeartOutlined, HeartFilled, MessageOutlined, UserOutlined, PlusOutlined, WalletOutlined, LockOutlined } from '@ant-design/icons';
import { authService } from '../../services/authService';
import { useNavigate } from 'react-router-dom';
import './Blog.css';

const { Title, Paragraph, Text } = Typography;
const { TextArea } = Input;

interface BlogPost {
  id: number;
  title: string;
  content: string;
  author_name: string;
  author_email: string;
  wallet_address?: string;
  image_url?: string;
  tags?: string[];
  likes: number;
  created_at: string;
}

const Blog: React.FC = () => {
  const [posts, setPosts] = useState<BlogPost[]>([]);
  const [loading, setLoading] = useState(true);
  const [createModalVisible, setCreateModalVisible] = useState(false);
  const [likedPosts, setLikedPosts] = useState<Set<number>>(new Set());
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [hasWallet, setHasWallet] = useState(false);
  const [currentUser, setCurrentUser] = useState<any>(null);
  const navigate = useNavigate();
  
  // Form state
  const [title, setTitle] = useState('');
  const [content, setContent] = useState('');
  const [imageUrl, setImageUrl] = useState('');
  const [submitting, setSubmitting] = useState(false);

  useEffect(() => {
    checkAuth();
    loadPosts();
  }, []);

  const checkAuth = () => {
    const authenticated = authService.isAuthenticated();
    const walletActivated = authService.hasWalletActivated();
    const user = authService.getCurrentDeveloper();
    
    setIsAuthenticated(authenticated);
    setHasWallet(walletActivated);
    setCurrentUser(user);
  };

  const loadPosts = async () => {
    try {
      setLoading(true);
      const response = await fetch('/api/blog/posts');
      if (response.ok) {
        const data = await response.json();
        setPosts(data);
      }
    } catch (error) {
      console.error('Failed to load posts:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleCreatePost = async () => {
    // Check authentication
    if (!isAuthenticated || !hasWallet || !currentUser) {
      message.error('Please login with your Mojo wallet to create posts');
      navigate('/dashboard');
      return;
    }

    if (!title || !content) {
      message.error('Please fill in title and content');
      return;
    }

    setSubmitting(true);

    try {
      const walletInfo = authService.getWalletInfo();
      
      const response = await fetch('/api/blog/posts', {
        method: 'POST',
        headers: { 
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${authService.getSessionToken()}`
        },
        body: JSON.stringify({
          title,
          content,
          author_name: currentUser.name,
          author_email: currentUser.email,
          wallet_address: walletInfo?.wallet_address || '',
          image_url: imageUrl || null,
          tags: []
        })
      });

      if (response.ok) {
        const newPost = await response.json();
        setPosts([newPost, ...posts]);
        message.success('Post created successfully!');
        setCreateModalVisible(false);
        resetForm();
      } else {
        message.error('Failed to create post');
      }
    } catch (error) {
      message.error('Failed to create post');
    } finally {
      setSubmitting(false);
    }
  };

  const handleLike = async (postId: number) => {
    if (likedPosts.has(postId)) {
      message.info('You already liked this post');
      return;
    }

    try {
      const response = await fetch('/api/blog/like', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ postId })
      });

      if (response.ok) {
        const { likes } = await response.json();
        setPosts(posts.map(p => p.id === postId ? { ...p, likes } : p));
        setLikedPosts(new Set([...likedPosts, postId]));
      }
    } catch (error) {
      message.error('Failed to like post');
    }
  };

  const resetForm = () => {
    setTitle('');
    setContent('');
    setImageUrl('');
  };

  const handleNewPostClick = () => {
    if (!isAuthenticated || !hasWallet) {
      message.warning('Please login with your Mojo wallet to create posts');
      navigate('/dashboard');
      return;
    }
    setCreateModalVisible(true);
  };

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMins = Math.floor(diffMs / 60000);
    const diffHours = Math.floor(diffMs / 3600000);
    const diffDays = Math.floor(diffMs / 86400000);

    if (diffMins < 1) return 'Just now';
    if (diffMins < 60) return `${diffMins}m ago`;
    if (diffHours < 24) return `${diffHours}h ago`;
    if (diffDays < 7) return `${diffDays}d ago`;
    return date.toLocaleDateString();
  };

  return (
    <div style={{ minHeight: '100vh', background: '#0A1628', padding: '2rem 0' }}>
      {/* Header */}
      <div style={{ maxWidth: '680px', margin: '0 auto', padding: '0 1rem', marginBottom: '2rem' }}>
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '2rem' }}>
          <Title level={2} style={{ color: '#ffffff', margin: 0 }}>Community Blog</Title>
          <Button
            type="primary"
            icon={isAuthenticated && hasWallet ? <PlusOutlined /> : <LockOutlined />}
            size="large"
            onClick={handleNewPostClick}
            style={{
              background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)',
              border: 'none',
              color: '#0A1628',
              fontWeight: '600'
            }}
          >
            {isAuthenticated && hasWallet ? 'New Post' : 'Login to Post'}
          </Button>
        </div>
        <Paragraph style={{ color: '#ffffff', fontSize: '1rem', margin: 0 }}>
          Share updates, research findings, and connect with the community
        </Paragraph>
      </div>

      {/* Posts Feed */}
      <div style={{ maxWidth: '680px', margin: '0 auto', padding: '0 1rem' }}>
        {loading ? (
          <div style={{ textAlign: 'center', padding: '3rem' }}>
            <Spin size="large" />
          </div>
        ) : posts.length === 0 ? (
          <Empty
            description={<span style={{ color: '#ffffff' }}>No posts yet. Be the first to share!</span>}
            style={{ padding: '3rem' }}
          />
        ) : (
          posts.map((post) => (
            <Card
              key={post.id}
              style={{
                background: 'rgba(10, 22, 40, 0.9)',
                border: '1px solid rgba(232, 180, 79, 0.2)',
                borderRadius: '12px',
                marginBottom: '1.5rem',
                backdropFilter: 'blur(10px)'
              }}
              bodyStyle={{ padding: '1.5rem' }}
            >
              {/* Author Header */}
              <div style={{ display: 'flex', alignItems: 'center', marginBottom: '1rem' }}>
                <Avatar
                  size={48}
                  icon={<UserOutlined />}
                  style={{ background: '#E8B44F', marginRight: '1rem' }}
                />
                <div style={{ flex: 1 }}>
                  <div style={{ color: '#ffffff', fontWeight: '600', fontSize: '1rem' }}>
                    {post.author_name}
                  </div>
                  <div style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>
                    {formatDate(post.created_at)}
                  </div>
                </div>
              </div>

              {/* Post Title */}
              <Title level={4} style={{ color: '#E8B44F', marginBottom: '0.75rem', marginTop: 0 }}>
                {post.title}
              </Title>

              {/* Post Content */}
              <Paragraph style={{ color: '#ffffff', fontSize: '1rem', lineHeight: '1.6', marginBottom: '1rem', whiteSpace: 'pre-wrap' }}>
                {post.content}
              </Paragraph>

              {/* Post Image (if exists) */}
              {post.image_url && (
                <div style={{ marginBottom: '1rem', borderRadius: '8px', overflow: 'hidden' }}>
                  <img
                    src={post.image_url}
                    alt={post.title}
                    style={{ width: '100%', display: 'block' }}
                    onError={(e) => { (e.target as HTMLImageElement).style.display = 'none'; }}
                  />
                </div>
              )}

              {/* Actions */}
              <div style={{ display: 'flex', gap: '1.5rem', paddingTop: '1rem', borderTop: '1px solid rgba(232, 180, 79, 0.1)' }}>
                <Button
                  type="text"
                  icon={likedPosts.has(post.id) ? <HeartFilled style={{ color: '#EF4444' }} /> : <HeartOutlined />}
                  onClick={() => handleLike(post.id)}
                  style={{ color: '#ffffff', display: 'flex', alignItems: 'center', gap: '0.5rem' }}
                >
                  {post.likes} {post.likes === 1 ? 'Like' : 'Likes'}
                </Button>
                <Button
                  type="text"
                  icon={<MessageOutlined />}
                  style={{ color: '#ffffff', display: 'flex', alignItems: 'center', gap: '0.5rem' }}
                >
                  Comment
                </Button>
              </div>
            </Card>
          ))
        )}
      </div>

      {/* Create Post Modal */}
      <Modal
        title={<span style={{ fontSize: '1.5rem', fontWeight: 'bold', color: '#E8B44F' }}>Create New Post</span>}
        open={createModalVisible}
        onOk={handleCreatePost}
        onCancel={() => {
          setCreateModalVisible(false);
          resetForm();
        }}
        okText="Publish"
        cancelText="Cancel"
        width={600}
        confirmLoading={submitting}
        okButtonProps={{
          style: {
            background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)',
            border: 'none',
            color: '#0A1628',
            fontWeight: '600'
          }
        }}
      >
        <div style={{ padding: '1rem 0' }}>
          {/* Auth Status Display */}
          {currentUser && (
            <Alert
              message={
                <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
                  <WalletOutlined style={{ color: '#10B981' }} />
                  <span>Posting as: <strong>{currentUser.name}</strong> ({currentUser.email})</span>
                </div>
              }
              type="success"
              style={{ marginBottom: '1.5rem' }}
              showIcon={false}
            />
          )}

          <div style={{ marginBottom: '1rem' }}>
            <label style={{ display: 'block', marginBottom: '0.5rem', fontWeight: '600', color: '#E8B44F' }}>
              Post Title *
            </label>
            <Input
              placeholder="What's your post about?"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              size="large"
              maxLength={200}
            />
          </div>

          <div style={{ marginBottom: '1rem' }}>
            <label style={{ display: 'block', marginBottom: '0.5rem', fontWeight: '600', color: '#E8B44F' }}>
              Content *
            </label>
            <TextArea
              placeholder="Share your thoughts, research findings, or updates..."
              value={content}
              onChange={(e) => setContent(e.target.value)}
              rows={6}
              maxLength={5000}
              showCount
            />
          </div>

          <div style={{ marginBottom: '1rem' }}>
            <label style={{ display: 'block', marginBottom: '0.5rem', fontWeight: '600', color: '#E8B44F' }}>
              Image URL (Optional)
            </label>
            <Input
              placeholder="https://your-cloud-storage.com/image.jpg"
              value={imageUrl}
              onChange={(e) => setImageUrl(e.target.value)}
              size="large"
            />
            <div style={{ marginTop: '0.5rem', fontSize: '0.875rem', color: '#9CA3AF' }}>
              💡 Upload your image to Google Drive, Imgur, or any cloud storage and paste the direct link here
            </div>
            {imageUrl && (
              <div style={{ marginTop: '1rem', padding: '0.5rem', background: 'rgba(232, 180, 79, 0.1)', borderRadius: '8px' }}>
                <Text style={{ color: '#9CA3AF', fontSize: '0.75rem', display: 'block', marginBottom: '0.5rem' }}>
                  Image Preview:
                </Text>
                <img
                  src={imageUrl}
                  alt="Preview"
                  style={{ width: '100%', maxHeight: '200px', objectFit: 'cover', borderRadius: '8px' }}
                  onError={(e) => {
                    (e.target as HTMLImageElement).style.display = 'none';
                    message.warning('Invalid image URL');
                  }}
                />
              </div>
            )}
          </div>
          
          <Alert
            message="Post Guidelines"
            description={
              <ul style={{ margin: '0.5rem 0', paddingLeft: '1.5rem', color: '#9CA3AF' }}>
                <li>Be respectful and constructive</li>
                <li>Share valuable insights about BPI/BPCI</li>
                <li>Use clear and concise language</li>
                <li>Add relevant images to enhance your post</li>
              </ul>
            }
            type="info"
            showIcon
            style={{ background: 'rgba(59, 130, 246, 0.1)', border: '1px solid rgba(59, 130, 246, 0.3)' }}
          />
        </div>
      </Modal>
    </div>
  );
};

export default Blog;
