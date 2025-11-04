import React, { useState, useEffect } from 'react';
import { Card, Typography, Form, Input, Button, Avatar, Upload, message, Space, Divider, Tag } from 'antd';
import {
  UserOutlined,
  MailOutlined,
  PhoneOutlined,
  EnvironmentOutlined,
  CameraOutlined,
  SaveOutlined,
  CheckCircleOutlined
} from '@ant-design/icons';
import type { UploadProps } from 'antd';
import { authService } from '../services/authService';
import axios from 'axios';

const { Title, Text } = Typography;

const INSTALLER_API = 'http://localhost:8080/api';

interface UserProfile {
  email: string;
  name: string;
  phone?: string;
  location?: string;
  bio?: string;
  avatar?: string;
  role: string;
  created_at: string;
}

const Profile: React.FC = () => {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);
  const [profile, setProfile] = useState<UserProfile | null>(null);
  const [avatarUrl, setAvatarUrl] = useState<string>('');

  useEffect(() => {
    loadProfile();
  }, []);

  const loadProfile = async () => {
    try {
      const currentUser = authService.getCurrentDeveloper();
      
      // Set initial profile data
      const profileData: UserProfile = {
        email: currentUser?.email || '',
        name: currentUser?.name || '',
        phone: (currentUser as any)?.phone || '',
        location: (currentUser as any)?.location || '',
        bio: (currentUser as any)?.bio || '',
        avatar: (currentUser as any)?.avatar || '',
        role: 'Developer',
        created_at: currentUser?.created_at || new Date().toISOString()
      };

      setProfile(profileData);
      setAvatarUrl(profileData.avatar || '');
      
      // Set form values
      form.setFieldsValue({
        name: profileData.name,
        email: profileData.email,
        phone: profileData.phone,
        location: profileData.location,
        bio: profileData.bio
      });
    } catch (error) {
      console.error('Failed to load profile:', error);
      message.error('Failed to load profile');
    }
  };

  const handleUpdateProfile = async (values: any) => {
    setLoading(true);
    try {
      // TODO: Replace with real API endpoint when backend is ready
      const response = await axios.put(`${INSTALLER_API}/profile`, {
        name: values.name,
        phone: values.phone,
        location: values.location,
        bio: values.bio
      });

      if (response.data.success) {
        message.success('Profile updated successfully!');
        
        // Update local storage
        const currentUser = authService.getCurrentDeveloper();
        const updatedUser = {
          ...currentUser,
          ...values
        };
        localStorage.setItem('developer', JSON.stringify(updatedUser));
        
        loadProfile();
      }
    } catch (error: any) {
      message.error(error.response?.data?.message || 'Failed to update profile. Make sure the backend is running.');
    } finally {
      setLoading(false);
    }
  };

  const uploadProps: UploadProps = {
    name: 'avatar',
    action: `${INSTALLER_API}/profile/avatar`,
    headers: {
      authorization: 'Bearer ' + localStorage.getItem('authToken'),
    },
    beforeUpload: (file) => {
      const isImage = file.type.startsWith('image/');
      if (!isImage) {
        message.error('You can only upload image files!');
      }
      const isLt2M = file.size / 1024 / 1024 < 2;
      if (!isLt2M) {
        message.error('Image must be smaller than 2MB!');
      }
      return isImage && isLt2M;
    },
    onChange: (info) => {
      if (info.file.status === 'done') {
        message.success('Avatar uploaded successfully');
        setAvatarUrl(info.file.response.url);
      } else if (info.file.status === 'error') {
        message.error('Avatar upload failed');
      }
    },
  };

  const getMemberSince = () => {
    if (!profile?.created_at) return 'Recently';
    const date = new Date(profile.created_at);
    return date.toLocaleDateString('en-US', { year: 'numeric', month: 'long' });
  };

  return (
    <div style={{ padding: '1.5rem', maxWidth: '1200px', margin: '0 auto' }}>
      {/* Header */}
      <div style={{ marginBottom: '2rem' }}>
        <Title level={2} style={{ color: '#E8B44F', marginBottom: '0.5rem' }}>
          <UserOutlined /> My Profile
        </Title>
        <Text style={{ color: '#9CA3AF' }}>
          Manage your personal information and preferences
        </Text>
      </div>

      <div style={{ display: 'grid', gridTemplateColumns: '300px 1fr', gap: '1.5rem' }}>
        {/* Left Column - Avatar & Info */}
        <div>
          <Card
            style={{
              background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
              border: '1px solid rgba(232, 180, 79, 0.2)',
              borderRadius: '12px',
              textAlign: 'center'
            }}
          >
            <Space direction="vertical" size="large" style={{ width: '100%' }}>
              <div style={{ position: 'relative', display: 'inline-block' }}>
                <Avatar
                  size={120}
                  src={avatarUrl}
                  icon={<UserOutlined />}
                  style={{
                    background: 'linear-gradient(135deg, #E8B44F 0%, #D4A044 100%)',
                    border: '4px solid rgba(232, 180, 79, 0.3)'
                  }}
                />
                <Upload {...uploadProps} showUploadList={false}>
                  <Button
                    shape="circle"
                    icon={<CameraOutlined />}
                    style={{
                      position: 'absolute',
                      bottom: 0,
                      right: 0,
                      background: '#E8B44F',
                      border: 'none',
                      color: '#0A1628'
                    }}
                  />
                </Upload>
              </div>

              <div>
                <Title level={4} style={{ color: '#E8B44F', marginBottom: '0.25rem' }}>
                  {profile?.name || 'Developer'}
                </Title>
                <Text style={{ color: '#9CA3AF', display: 'block', marginBottom: '0.5rem' }}>
                  {profile?.email}
                </Text>
                <Tag color="gold" style={{ marginTop: '0.5rem' }}>
                  {profile?.role}
                </Tag>
              </div>

              <Divider style={{ borderColor: 'rgba(232, 180, 79, 0.2)', margin: '0.5rem 0' }} />

              <div style={{ textAlign: 'left', width: '100%' }}>
                <Space direction="vertical" size="small" style={{ width: '100%' }}>
                  <div>
                    <Text style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>Member Since</Text>
                    <br />
                    <Text style={{ color: '#E8B44F', fontWeight: '600' }}>
                      {getMemberSince()}
                    </Text>
                  </div>
                  <div>
                    <Text style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>Account Status</Text>
                    <br />
                    <Tag color="success" icon={<CheckCircleOutlined />}>
                      Active
                    </Tag>
                  </div>
                </Space>
              </div>
            </Space>
          </Card>
        </div>

        {/* Right Column - Profile Form */}
        <Card
          style={{
            background: 'linear-gradient(135deg, #1a2332 0%, #0f1419 100%)',
            border: '1px solid rgba(232, 180, 79, 0.2)',
            borderRadius: '12px'
          }}
        >
          <Title level={4} style={{ color: '#E8B44F', marginBottom: '1.5rem' }}>
            Profile Information
          </Title>

          <Form
            form={form}
            layout="vertical"
            onFinish={handleUpdateProfile}
          >
            <Form.Item
              label={<span style={{ color: '#E8B44F', fontWeight: '600' }}>Full Name</span>}
              name="name"
              rules={[{ required: true, message: 'Please enter your name' }]}
            >
              <Input
                prefix={<UserOutlined style={{ color: '#E8B44F' }} />}
                placeholder="Enter your full name"
                size="large"
                style={{
                  background: 'rgba(255, 255, 255, 0.05)',
                  border: '1px solid rgba(232, 180, 79, 0.2)',
                  color: '#ffffff'
                }}
              />
            </Form.Item>

            <Form.Item
              label={<span style={{ color: '#E8B44F', fontWeight: '600' }}>Email</span>}
              name="email"
            >
              <Input
                prefix={<MailOutlined style={{ color: '#E8B44F' }} />}
                disabled
                size="large"
                style={{
                  background: 'rgba(255, 255, 255, 0.03)',
                  border: '1px solid rgba(232, 180, 79, 0.1)',
                  color: '#9CA3AF'
                }}
              />
            </Form.Item>

            <Form.Item
              label={<span style={{ color: '#E8B44F', fontWeight: '600' }}>Phone Number</span>}
              name="phone"
            >
              <Input
                prefix={<PhoneOutlined style={{ color: '#E8B44F' }} />}
                placeholder="Enter your phone number"
                size="large"
                style={{
                  background: 'rgba(255, 255, 255, 0.05)',
                  border: '1px solid rgba(232, 180, 79, 0.2)',
                  color: '#ffffff'
                }}
              />
            </Form.Item>

            <Form.Item
              label={<span style={{ color: '#E8B44F', fontWeight: '600' }}>Location</span>}
              name="location"
            >
              <Input
                prefix={<EnvironmentOutlined style={{ color: '#E8B44F' }} />}
                placeholder="City, Country"
                size="large"
                style={{
                  background: 'rgba(255, 255, 255, 0.05)',
                  border: '1px solid rgba(232, 180, 79, 0.2)',
                  color: '#ffffff'
                }}
              />
            </Form.Item>

            <Form.Item
              label={<span style={{ color: '#E8B44F', fontWeight: '600' }}>Bio</span>}
              name="bio"
            >
              <Input.TextArea
                placeholder="Tell us about yourself"
                rows={4}
                style={{
                  background: 'rgba(255, 255, 255, 0.05)',
                  border: '1px solid rgba(232, 180, 79, 0.2)',
                  color: '#ffffff'
                }}
              />
            </Form.Item>

            <Form.Item>
              <Button
                type="primary"
                htmlType="submit"
                size="large"
                loading={loading}
                icon={<SaveOutlined />}
                style={{
                  background: 'linear-gradient(135deg, #E8B44F 0%, #D4A044 100%)',
                  border: 'none',
                  fontWeight: '600',
                  width: '200px'
                }}
              >
                Save Changes
              </Button>
            </Form.Item>
          </Form>
        </Card>
      </div>
    </div>
  );
};

export default Profile;
