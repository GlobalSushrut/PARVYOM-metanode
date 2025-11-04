/**
 * PRAVYOM UNIQUE AUTHENTICATION UI
 * Custom-designed authentication interface using Pravyom brand colors and styling
 * Features: Glassmorphism design, gradient backgrounds, unique visual elements
 */

import React, { useState } from 'react';
import { message } from 'antd';
import { 
  UserOutlined, 
  LockOutlined, 
  MailOutlined, 
  SafetyOutlined, 
  EyeOutlined,
  EyeInvisibleOutlined,
  RocketOutlined
} from '@ant-design/icons';

interface PravyomAuthUIProps {
  onAuthSuccess: (user: any) => void;
}

const PravyomAuthUI: React.FC<PravyomAuthUIProps> = ({ onAuthSuccess }) => {
  const [isLogin, setIsLogin] = useState(true);
  const [loading, setLoading] = useState(false);
  const [showPassword, setShowPassword] = useState(false);
  const [formData, setFormData] = useState({
    email: '',
    password: '',
    name: '',
    confirmPassword: ''
  });

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);

    try {
      // Simulate authentication
      await new Promise(resolve => setTimeout(resolve, 1500));
      
      message.success(`Welcome to Pravyom ${isLogin ? 'back' : ''}!`);
      onAuthSuccess({
        email: formData.email,
        name: formData.name || formData.email.split('@')[0],
        isAuthenticated: true
      });
    } catch (error) {
      message.error('Authentication failed. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const handleInputChange = (field: string, value: string) => {
    setFormData(prev => ({ ...prev, [field]: value }));
  };

  return (
    <div style={{
      minHeight: '100vh',
      background: 'var(--gradient-depth)', // Navy gradient from Pravyom brand
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
      padding: '20px',
      position: 'relative',
      overflow: 'hidden'
    }}>
      {/* Animated Background Elements */}
      <div style={{
        position: 'absolute',
        top: '10%',
        left: '10%',
        width: '300px',
        height: '300px',
        background: 'var(--gradient-innovation)',
        borderRadius: '50%',
        opacity: 0.1,
        animation: 'pulse 4s ease-in-out infinite'
      }} />
      
      <div style={{
        position: 'absolute',
        bottom: '10%',
        right: '10%',
        width: '200px',
        height: '200px',
        background: 'var(--gradient-success)',
        borderRadius: '50%',
        opacity: 0.1,
        animation: 'pulse 6s ease-in-out infinite reverse'
      }} />

      {/* Main Authentication Card */}
      <div style={{
        background: 'rgba(255, 255, 255, 0.1)',
        backdropFilter: 'blur(20px)',
        borderRadius: '24px',
        border: '1px solid rgba(255, 255, 255, 0.2)',
        padding: '48px',
        width: '100%',
        maxWidth: '480px',
        boxShadow: '0 25px 50px rgba(0, 0, 0, 0.3)',
        position: 'relative',
        zIndex: 1
      }}>
        {/* Pravyom Logo and Header */}
        <div style={{ textAlign: 'center', marginBottom: '40px' }}>
          <div style={{
            width: '80px',
            height: '80px',
            background: 'var(--gradient-transformation)',
            borderRadius: '20px',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            margin: '0 auto 24px',
            boxShadow: '0 10px 30px rgba(232, 180, 79, 0.3)'
          }}>
            <RocketOutlined style={{ fontSize: '36px', color: 'var(--color-navy)' }} />
          </div>
          
          <h1 style={{
            color: '#ffffff',
            fontSize: '32px',
            fontWeight: '700',
            margin: '0 0 8px',
            textShadow: '0 2px 10px rgba(0, 0, 0, 0.3)'
          }}>
            PRAVYOM
          </h1>
          
          <p style={{
            color: 'rgba(255, 255, 255, 0.8)',
            fontSize: '16px',
            margin: 0,
            fontWeight: '400'
          }}>
            {isLogin ? 'Welcome back to the future' : 'Join the blockchain revolution'}
          </p>
        </div>

        {/* Tab Switcher */}
        <div style={{
          display: 'flex',
          background: 'rgba(255, 255, 255, 0.1)',
          borderRadius: '16px',
          padding: '6px',
          marginBottom: '32px'
        }}>
          <button
            onClick={() => setIsLogin(true)}
            style={{
              flex: 1,
              padding: '12px 24px',
              border: 'none',
              borderRadius: '12px',
              background: isLogin ? 'var(--gradient-transformation)' : 'transparent',
              color: isLogin ? 'var(--color-navy)' : '#ffffff',
              fontWeight: '600',
              fontSize: '16px',
              cursor: 'pointer',
              transition: 'all 0.3s ease',
              boxShadow: isLogin ? '0 4px 15px rgba(232, 180, 79, 0.3)' : 'none'
            }}
          >
            Sign In
          </button>
          <button
            onClick={() => setIsLogin(false)}
            style={{
              flex: 1,
              padding: '12px 24px',
              border: 'none',
              borderRadius: '12px',
              background: !isLogin ? 'var(--gradient-transformation)' : 'transparent',
              color: !isLogin ? 'var(--color-navy)' : '#ffffff',
              fontWeight: '600',
              fontSize: '16px',
              cursor: 'pointer',
              transition: 'all 0.3s ease',
              boxShadow: !isLogin ? '0 4px 15px rgba(232, 180, 79, 0.3)' : 'none'
            }}
          >
            Sign Up
          </button>
        </div>

        {/* Authentication Form */}
        <form onSubmit={handleSubmit}>
          {/* Name Field (Signup only) */}
          {!isLogin && (
            <div style={{ marginBottom: '24px' }}>
              <div style={{
                position: 'relative',
                background: 'rgba(255, 255, 255, 0.1)',
                borderRadius: '16px',
                border: '1px solid rgba(255, 255, 255, 0.2)'
              }}>
                <UserOutlined style={{
                  position: 'absolute',
                  left: '20px',
                  top: '50%',
                  transform: 'translateY(-50%)',
                  color: 'rgba(255, 255, 255, 0.7)',
                  fontSize: '18px'
                }} />
                <input
                  type="text"
                  placeholder="Full Name"
                  value={formData.name}
                  onChange={(e) => handleInputChange('name', e.target.value)}
                  required={!isLogin}
                  style={{
                    width: '100%',
                    padding: '18px 20px 18px 60px',
                    border: 'none',
                    background: 'transparent',
                    color: '#ffffff',
                    fontSize: '16px',
                    borderRadius: '16px',
                    outline: 'none'
                  }}
                />
              </div>
            </div>
          )}

          {/* Email Field */}
          <div style={{ marginBottom: '24px' }}>
            <div style={{
              position: 'relative',
              background: 'rgba(255, 255, 255, 0.1)',
              borderRadius: '16px',
              border: '1px solid rgba(255, 255, 255, 0.2)'
            }}>
              <MailOutlined style={{
                position: 'absolute',
                left: '20px',
                top: '50%',
                transform: 'translateY(-50%)',
                color: 'rgba(255, 255, 255, 0.7)',
                fontSize: '18px'
              }} />
              <input
                type="email"
                placeholder="Email Address"
                value={formData.email}
                onChange={(e) => handleInputChange('email', e.target.value)}
                required
                style={{
                  width: '100%',
                  padding: '18px 20px 18px 60px',
                  border: 'none',
                  background: 'transparent',
                  color: '#ffffff',
                  fontSize: '16px',
                  borderRadius: '16px',
                  outline: 'none'
                }}
              />
            </div>
          </div>

          {/* Password Field */}
          <div style={{ marginBottom: isLogin ? '32px' : '24px' }}>
            <div style={{
              position: 'relative',
              background: 'rgba(255, 255, 255, 0.1)',
              borderRadius: '16px',
              border: '1px solid rgba(255, 255, 255, 0.2)'
            }}>
              <LockOutlined style={{
                position: 'absolute',
                left: '20px',
                top: '50%',
                transform: 'translateY(-50%)',
                color: 'rgba(255, 255, 255, 0.7)',
                fontSize: '18px'
              }} />
              <input
                type={showPassword ? 'text' : 'password'}
                placeholder="Password"
                value={formData.password}
                onChange={(e) => handleInputChange('password', e.target.value)}
                required
                style={{
                  width: '100%',
                  padding: '18px 60px 18px 60px',
                  border: 'none',
                  background: 'transparent',
                  color: '#ffffff',
                  fontSize: '16px',
                  borderRadius: '16px',
                  outline: 'none'
                }}
              />
              <button
                type="button"
                onClick={() => setShowPassword(!showPassword)}
                style={{
                  position: 'absolute',
                  right: '20px',
                  top: '50%',
                  transform: 'translateY(-50%)',
                  border: 'none',
                  background: 'transparent',
                  color: 'rgba(255, 255, 255, 0.7)',
                  fontSize: '18px',
                  cursor: 'pointer'
                }}
              >
                {showPassword ? <EyeInvisibleOutlined /> : <EyeOutlined />}
              </button>
            </div>
          </div>

          {/* Confirm Password (Signup only) */}
          {!isLogin && (
            <div style={{ marginBottom: '32px' }}>
              <div style={{
                position: 'relative',
                background: 'rgba(255, 255, 255, 0.1)',
                borderRadius: '16px',
                border: '1px solid rgba(255, 255, 255, 0.2)'
              }}>
                <SafetyOutlined style={{
                  position: 'absolute',
                  left: '20px',
                  top: '50%',
                  transform: 'translateY(-50%)',
                  color: 'rgba(255, 255, 255, 0.7)',
                  fontSize: '18px'
                }} />
                <input
                  type="password"
                  placeholder="Confirm Password"
                  value={formData.confirmPassword}
                  onChange={(e) => handleInputChange('confirmPassword', e.target.value)}
                  required={!isLogin}
                  style={{
                    width: '100%',
                    padding: '18px 20px 18px 60px',
                    border: 'none',
                    background: 'transparent',
                    color: '#ffffff',
                    fontSize: '16px',
                    borderRadius: '16px',
                    outline: 'none'
                  }}
                />
              </div>
            </div>
          )}

          {/* Submit Button */}
          <button
            type="submit"
            disabled={loading}
            style={{
              width: '100%',
              padding: '18px',
              border: 'none',
              borderRadius: '16px',
              background: loading ? 'rgba(232, 180, 79, 0.5)' : 'var(--gradient-transformation)',
              color: 'var(--color-navy)',
              fontSize: '18px',
              fontWeight: '700',
              cursor: loading ? 'not-allowed' : 'pointer',
              transition: 'all 0.3s ease',
              boxShadow: '0 8px 25px rgba(232, 180, 79, 0.4)',
              transform: loading ? 'scale(0.98)' : 'scale(1)'
            }}
          >
            {loading ? 'Processing...' : (isLogin ? 'Sign In to Pravyom' : 'Join Pravyom')}
          </button>
        </form>

        {/* SSO Options */}
        <div style={{ marginTop: '32px' }}>
          <div style={{
            display: 'flex',
            alignItems: 'center',
            marginBottom: '24px'
          }}>
            <div style={{ flex: 1, height: '1px', background: 'rgba(255, 255, 255, 0.2)' }} />
            <span style={{ 
              color: 'rgba(255, 255, 255, 0.7)', 
              margin: '0 16px',
              fontSize: '14px'
            }}>
              Or continue with
            </span>
            <div style={{ flex: 1, height: '1px', background: 'rgba(255, 255, 255, 0.2)' }} />
          </div>

          <button
            type="button"
            style={{
              width: '100%',
              padding: '16px',
              border: '1px solid rgba(255, 255, 255, 0.2)',
              borderRadius: '16px',
              background: 'rgba(255, 255, 255, 0.1)',
              color: '#ffffff',
              fontSize: '16px',
              fontWeight: '600',
              cursor: 'pointer',
              transition: 'all 0.3s ease',
              backdropFilter: 'blur(10px)'
            }}
            onMouseEnter={(e) => {
              e.currentTarget.style.background = 'rgba(255, 255, 255, 0.2)';
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.background = 'rgba(255, 255, 255, 0.1)';
            }}
          >
            🔐 Enterprise SSO
          </button>
        </div>

        {/* Footer */}
        <div style={{ 
          textAlign: 'center', 
          marginTop: '32px',
          color: 'rgba(255, 255, 255, 0.6)',
          fontSize: '14px'
        }}>
          Secured by BPCI • Powered by Blockchain
        </div>
      </div>
    </div>
  );
};

export default PravyomAuthUI;
