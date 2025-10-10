import React, { useState, useEffect } from 'react';
import { Form, Input, Button, Card, Alert, Typography, Space, Statistic } from 'antd';
import { MailOutlined, SafetyOutlined, ClockCircleOutlined } from '@ant-design/icons';
import { apiService, type SendOTPRequest, type VerifyOTPRequest } from '../../services/api';

const { Title, Text } = Typography;
const { Countdown } = Statistic;

interface OTPVerificationProps {
  email: string;
  purpose: 'Registration' | 'PasswordReset';
  onVerificationSuccess: () => void;
  onBack: () => void;
}

const OTPVerification: React.FC<OTPVerificationProps> = ({ 
  email, 
  purpose, 
  onVerificationSuccess, 
  onBack 
}) => {
  const [loading, setLoading] = useState(false);
  const [resendLoading, setResendLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [canResend, setCanResend] = useState(false);
  const [resendDeadline, setResendDeadline] = useState<number>(0);

  useEffect(() => {
    // Set initial countdown for resend (60 seconds)
    const deadline = Date.now() + 60 * 1000;
    setResendDeadline(deadline);
    setCanResend(false);
  }, []);

  const onFinish = async (values: { otp_code: string }) => {
    setLoading(true);
    setError(null);

    try {
      const request: VerifyOTPRequest = {
        email,
        otp_code: values.otp_code,
        purpose,
      };

      const response = await apiService.verifyOTP(request);
      
      if (response.success) {
        setSuccess('Email verified successfully! Completing registration...');
        setTimeout(() => {
          onVerificationSuccess();
        }, 1500);
      } else {
        setError(response.error || 'Invalid OTP code. Please try again.');
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
    setSuccess(null);

    try {
      const request: SendOTPRequest = {
        email,
        purpose,
      };

      const response = await apiService.sendOTP(request);
      
      if (response.success) {
        setSuccess('New OTP code sent to your email!');
        // Reset countdown
        const deadline = Date.now() + 60 * 1000;
        setResendDeadline(deadline);
        setCanResend(false);
        setTimeout(() => setSuccess(null), 3000);
      } else {
        setError(response.error || 'Failed to resend OTP. Please try again.');
      }
    } catch (error) {
      setError('Failed to resend OTP. Please check if the BPCI server is running.');
    } finally {
      setResendLoading(false);
    }
  };

  const onCountdownFinish = () => {
    setCanResend(true);
  };

  return (
    <Card 
      style={{ maxWidth: 400, margin: '0 auto' }}
      title={
        <Space>
          <SafetyOutlined />
          <Title level={3} style={{ margin: 0 }}>Email Verification</Title>
        </Space>
      }
    >
      <div style={{ marginBottom: 24, textAlign: 'center' }}>
        <Space direction="vertical" size="small">
          <MailOutlined style={{ fontSize: 48, color: '#1890ff' }} />
          <Text type="secondary">
            We've sent a 6-digit verification code to:
          </Text>
          <Text strong>{email}</Text>
          <Text type="secondary" style={{ fontSize: '12px' }}>
            Please check your email and enter the code below
          </Text>
        </Space>
      </div>

      {error && (
        <Alert
          message={error}
          type="error"
          style={{ marginBottom: 16 }}
          closable
          onClose={() => setError(null)}
        />
      )}

      {success && (
        <Alert
          message={success}
          type="success"
          style={{ marginBottom: 16 }}
          closable
          onClose={() => setSuccess(null)}
        />
      )}

      <Form
        name="otpVerification"
        onFinish={onFinish}
        layout="vertical"
        size="large"
      >
        <Form.Item
          label="Verification Code"
          name="otp_code"
          rules={[
            { required: true, message: 'Please enter the verification code!' },
            { len: 6, message: 'Verification code must be exactly 6 digits!' },
            { pattern: /^\d{6}$/, message: 'Verification code must contain only numbers!' }
          ]}
        >
          <Input 
            placeholder="Enter 6-digit code"
            maxLength={6}
            style={{ 
              textAlign: 'center', 
              fontSize: '18px', 
              letterSpacing: '4px',
              fontFamily: 'monospace'
            }}
          />
        </Form.Item>

        <Form.Item>
          <Button 
            type="primary" 
            htmlType="submit" 
            loading={loading}
            block
            size="large"
          >
            {loading ? 'Verifying...' : 'Verify Email'}
          </Button>
        </Form.Item>
      </Form>

      <div style={{ textAlign: 'center', marginTop: 16 }}>
        <Space direction="vertical" size="small">
          <Text type="secondary">Didn't receive the code?</Text>
          
          {!canResend ? (
            <Space>
              <ClockCircleOutlined />
              <Text type="secondary">Resend available in:</Text>
              <Countdown
                value={resendDeadline}
                format="ss"
                onFinish={onCountdownFinish}
                valueStyle={{ fontSize: '14px' }}
              />
              <Text type="secondary">seconds</Text>
            </Space>
          ) : (
            <Button 
              type="link" 
              onClick={handleResendOTP}
              loading={resendLoading}
            >
              {resendLoading ? 'Sending...' : 'Resend Code'}
            </Button>
          )}
        </Space>
      </div>

      <div style={{ textAlign: 'center', marginTop: 24 }}>
        <Button type="link" onClick={onBack}>
          ← Back to Registration
        </Button>
      </div>
    </Card>
  );
};

export default OTPVerification;
