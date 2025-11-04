/**
 * Email Verification Component for BPCI Enterprise
 * Handles OTP verification after user registration
 * Redesigned with Navy + Gold brand
 */

import React, { useState, useEffect } from 'react';
import { Form, Input, Button, Alert, Typography, message } from 'antd';
import { MailOutlined, SafetyOutlined, CheckCircleOutlined, ReloadOutlined } from '@ant-design/icons';
import { apiService } from '../../services/api';

const { Title, Text } = Typography;

interface EmailVerificationProps {
  email: string;
  onVerificationSuccess: () => void;
  onBackToSignup: () => void;
}

const EmailVerification: React.FC<EmailVerificationProps> = ({ 
  email, 
  onVerificationSuccess, 
  onBackToSignup 
}) => {
  const [loading, setLoading] = useState(false);
  const [resendLoading, setResendLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [canResend, setCanResend] = useState(false);
  const [resendCountdown, setResendCountdown] = useState(60);
  const [form] = Form.useForm();

  useEffect(() => {
    // Start countdown for resend button
    const timer = setInterval(() => {
      setResendCountdown((prev) => {
        if (prev <= 1) {
          setCanResend(true);
          clearInterval(timer);
          return 0;
        }
        return prev - 1;
      });
    }, 1000);

    return () => clearInterval(timer);
  }, []);

  const onFinish = async (values: { otp_code: string }) => {
    setLoading(true);
    setError(null);

    try {
      const response = await apiService.completeEmailVerification(email, values.otp_code);
      
      if (response.success) {
        message.success('Email verified successfully! Redirecting to dashboard...');
        setTimeout(() => {
          onVerificationSuccess();
        }, 1500);
      } else {
        setError(response.error || 'Invalid verification code. Please try again.');
      }
    } catch (error) {
      setError('Verification failed. Please check if the BPCI server is running.');
    } finally {
      setLoading(false);
    }
  };

  const handleResendOTP = async () => {
    setResendLoading(true);
    setError(null);

    try {
      const response = await apiService.initiateEmailVerification(email);
      
      if (response.success) {
        message.success('Verification code sent! Please check your email.');
        setCanResend(false);
        setResendCountdown(60);
        
        // Restart countdown
        const timer = setInterval(() => {
          setResendCountdown((prev) => {
            if (prev <= 1) {
              setCanResend(true);
              clearInterval(timer);
              return 0;
            }
            return prev - 1;
          });
        }, 1000);
      } else {
        setError(response.error || 'Failed to resend verification code.');
      }
    } catch (error) {
      setError('Failed to resend verification code. Please try again.');
    } finally {
      setResendLoading(false);
    }
  };

  return (
    <div style={{ 
      minHeight: '100vh', 
      background: 'linear-gradient(135deg, #0A1628 0%, #1e293b 100%)',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
      padding: '2rem'
    }}>
      <div style={{
        maxWidth: '520px',
        width: '100%',
        background: 'rgba(10, 22, 40, 0.95)',
        border: '2px solid rgba(232, 180, 79, 0.3)',
        borderRadius: '16px',
        padding: '3rem',
        backdropFilter: 'blur(10px)',
        boxShadow: '0 8px 32px rgba(0, 0, 0, 0.3)'
      }}>
        {/* Header */}
        <div style={{ textAlign: 'center', marginBottom: '2rem' }}>
          <MailOutlined style={{ fontSize: '3rem', color: '#E8B44F', marginBottom: '1rem' }} />
          <Title level={2} style={{ color: '#ffffff', marginBottom: '0.5rem' }}>
            Verify Your Email
          </Title>
          <Text style={{ color: '#9CA3AF', fontSize: '1rem', display: 'block', marginBottom: '1rem' }}>
            We've sent a 6-digit verification code to:
          </Text>
          <div style={{
            padding: '0.75rem 1rem',
            background: 'rgba(232, 180, 79, 0.1)',
            border: '1px solid rgba(232, 180, 79, 0.3)',
            borderRadius: '8px',
            marginBottom: '1rem'
          }}>
            <Text style={{ color: '#E8B44F', fontSize: '1.125rem', fontWeight: '600' }}>
              {email}
            </Text>
          </div>
          <Text style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>
            Please enter the code to complete your registration
          </Text>
        </div>

        {/* Error Alert */}
        {error && (
          <Alert
            message="Verification Failed"
            description={error}
            type="error"
            showIcon
            closable
            onClose={() => setError(null)}
            style={{ marginBottom: '1.5rem' }}
          />
        )}

        {/* OTP Form */}
        <Form
          form={form}
          name="emailVerification"
          onFinish={onFinish}
          layout="vertical"
          size="large"
        >
          <Form.Item
            name="otp_code"
            label={<span style={{ color: '#E8B44F', fontWeight: '600' }}>Verification Code</span>}
            rules={[
              { required: true, message: 'Please enter the verification code' },
              { len: 6, message: 'Code must be 6 digits' },
              { pattern: /^\d{6}$/, message: 'Code must contain only numbers' }
            ]}
          >
            <Input 
              prefix={<SafetyOutlined style={{ color: '#9CA3AF' }} />} 
              placeholder="000000"
              maxLength={6}
              style={{
                background: 'rgba(255, 255, 255, 0.05)',
                border: '1px solid rgba(232, 180, 79, 0.2)',
                color: '#ffffff',
                height: '56px',
                textAlign: 'center',
                fontSize: '1.5rem',
                letterSpacing: '0.5rem',
                fontWeight: '600'
              }}
            />
          </Form.Item>

          <Form.Item style={{ marginBottom: '1rem' }}>
            <Button
              type="primary"
              htmlType="submit"
              loading={loading}
              block
              icon={<CheckCircleOutlined />}
              style={{
                background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)',
                border: 'none',
                color: '#0A1628',
                fontWeight: '600',
                height: '48px',
                fontSize: '1rem'
              }}
            >
              {loading ? 'Verifying...' : 'Verify Email'}
            </Button>
          </Form.Item>
        </Form>

        {/* Resend Section */}
        <div style={{ textAlign: 'center', marginBottom: '1.5rem' }}>
          <Text style={{ color: '#9CA3AF', display: 'block', marginBottom: '0.5rem' }}>
            Didn't receive the code?
          </Text>
          <Button 
            type="link"
            onClick={handleResendOTP}
            loading={resendLoading}
            disabled={!canResend}
            icon={<ReloadOutlined />}
            style={{
              color: canResend ? '#E8B44F' : '#6B7280',
              fontWeight: '600',
              fontSize: '1rem'
            }}
          >
            {canResend ? 'Resend Code' : `Resend in ${resendCountdown}s`}
          </Button>
        </div>

        {/* Divider */}
        <div style={{ 
          display: 'flex', 
          alignItems: 'center', 
          margin: '1.5rem 0',
          gap: '1rem'
        }}>
          <div style={{ flex: 1, height: '1px', background: 'rgba(232, 180, 79, 0.2)' }} />
          <Text style={{ color: '#9CA3AF', fontSize: '0.875rem' }}>OR</Text>
          <div style={{ flex: 1, height: '1px', background: 'rgba(232, 180, 79, 0.2)' }} />
        </div>

        {/* Back to Signup */}
        <div style={{ textAlign: 'center' }}>
          <Text style={{ color: '#9CA3AF' }}>
            Need to change your email?{' '}
            <a 
              onClick={onBackToSignup}
              style={{ 
                color: '#E8B44F', 
                fontWeight: '600',
                cursor: 'pointer',
                textDecoration: 'none'
              }}
            >
              Back to Registration
            </a>
          </Text>
        </div>

        {/* Info Box */}
        <div style={{
          marginTop: '2rem',
          padding: '1rem',
          background: 'rgba(59, 130, 246, 0.1)',
          border: '1px solid rgba(59, 130, 246, 0.3)',
          borderRadius: '8px'
        }}>
          <Text style={{ color: '#ffffff', fontSize: '0.875rem', lineHeight: '1.6' }}>
            <strong style={{ color: '#3B82F6' }}>Security Note:</strong> The verification code expires in 10 minutes. If you don't receive it, check your spam folder or request a new code.
          </Text>
        </div>
      </div>
    </div>
  );
};

export default EmailVerification;
