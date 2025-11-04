import React, { useState } from 'react';
import { Card, Typography, Input, Button, Alert, message, Space } from 'antd';
import { FileTextOutlined, WalletOutlined, PlusOutlined } from '@ant-design/icons';
import { authService } from '../services/authService';
import { useNavigate } from 'react-router-dom';

const { Title, Text } = Typography;
const { TextArea } = Input;

const BlogCreate: React.FC = () => {
  const navigate = useNavigate();
  const [title, setTitle] = useState('');
  const [content, setContent] = useState('');
  const [imageUrl, setImageUrl] = useState('');
  const [submitting, setSubmitting] = useState(false);
  
  const currentUser = authService.getCurrentDeveloper();
  const isAuthenticated = authService.isAuthenticated();
  const hasWallet = authService.hasWalletActivated();

  const handleCreatePost = async () => {
    // Check authentication
    if (!isAuthenticated || !hasWallet || !currentUser) {
      message.error('Please login with your Mojo wallet to create posts');
      navigate('/login');
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
        }),
      });

      if (response.ok) {
        message.success('Post created successfully!');
        setTitle('');
        setContent('');
        setImageUrl('');
        navigate('/blog');
      } else {
        message.error('Failed to create post');
      }
    } catch (error) {
      console.error('Error creating post:', error);
      message.error('Failed to create post');
    } finally {
      setSubmitting(false);
    }
  };

  const resetForm = () => {
    setTitle('');
    setContent('');
    setImageUrl('');
  };

  return (
    <div style={{ padding: '1.5rem', maxWidth: '800px', margin: '0 auto' }}>
      {/* Header */}
      <div style={{ marginBottom: '2rem' }}>
        <Title level={2} style={{ color: '#E8B44F', marginBottom: '0.5rem' }}>
          <FileTextOutlined /> Create New Post
        </Title>
        <Text style={{ color: '#9CA3AF' }}>
          Share your thoughts, research findings, or updates with the community
        </Text>
      </div>

      {/* Auth Status */}
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

      {/* Create Post Form */}
      <Card
        style={{
          background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
          border: '1px solid rgba(232, 180, 79, 0.2)',
          borderRadius: '12px'
        }}
      >
        <Space direction="vertical" size="large" style={{ width: '100%' }}>
          {/* Title */}
          <div>
            <label style={{ display: 'block', marginBottom: '0.5rem', fontWeight: '600', color: '#E8B44F' }}>
              Post Title *
            </label>
            <Input
              placeholder="What's your post about?"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              size="large"
              maxLength={200}
              style={{
                background: 'rgba(255, 255, 255, 0.05)',
                border: '1px solid rgba(232, 180, 79, 0.2)',
                color: '#ffffff'
              }}
            />
          </div>

          {/* Content */}
          <div>
            <label style={{ display: 'block', marginBottom: '0.5rem', fontWeight: '600', color: '#E8B44F' }}>
              Content *
            </label>
            <TextArea
              placeholder="Share your thoughts, research findings, or updates..."
              value={content}
              onChange={(e) => setContent(e.target.value)}
              rows={10}
              maxLength={5000}
              showCount
              style={{
                background: 'rgba(255, 255, 255, 0.05)',
                border: '1px solid rgba(232, 180, 79, 0.2)',
                color: '#ffffff'
              }}
            />
          </div>

          {/* Image URL */}
          <div>
            <label style={{ display: 'block', marginBottom: '0.5rem', fontWeight: '600', color: '#E8B44F' }}>
              Image URL (Optional)
            </label>
            <Input
              placeholder="https://your-cloud-storage.com/image.jpg"
              value={imageUrl}
              onChange={(e) => setImageUrl(e.target.value)}
              size="large"
              style={{
                background: 'rgba(255, 255, 255, 0.05)',
                border: '1px solid rgba(232, 180, 79, 0.2)',
                color: '#ffffff'
              }}
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
                  style={{ width: '100%', maxHeight: '300px', objectFit: 'cover', borderRadius: '8px' }}
                  onError={(e) => {
                    (e.target as HTMLImageElement).style.display = 'none';
                    message.warning('Invalid image URL');
                  }}
                />
              </div>
            )}
          </div>

          {/* Guidelines */}
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

          {/* Action Buttons */}
          <div style={{ display: 'flex', gap: '1rem', justifyContent: 'flex-end' }}>
            <Button
              size="large"
              onClick={resetForm}
              style={{
                background: 'transparent',
                border: '1px solid rgba(232, 180, 79, 0.3)',
                color: '#9CA3AF'
              }}
            >
              Clear
            </Button>
            <Button
              type="primary"
              size="large"
              icon={<PlusOutlined />}
              onClick={handleCreatePost}
              loading={submitting}
              disabled={!title || !content}
              style={{
                background: 'linear-gradient(135deg, #E8B44F 0%, #D4A044 100%)',
                border: 'none',
                fontWeight: '600'
              }}
            >
              Publish Post
            </Button>
          </div>
        </Space>
      </Card>
    </div>
  );
};

export default BlogCreate;
