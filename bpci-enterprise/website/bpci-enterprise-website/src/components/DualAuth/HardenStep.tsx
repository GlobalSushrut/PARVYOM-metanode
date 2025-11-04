import React, { useState } from 'react';
import { Button, Typography, Space, Alert, Card, Checkbox, Spin, Result } from 'antd';
import { SafetyOutlined, CheckCircleOutlined, LockOutlined } from '@ant-design/icons';
import { apiService } from '../../services/api';

const { Title, Text, Paragraph } = Typography;

interface HardenStepProps {
  generatedData: any;
  bindingData: any;
  onComplete: () => void;
  onBack: () => void;
}

const HardenStep: React.FC<HardenStepProps> = ({ 
  generatedData, 
  bindingData, 
  onComplete, 
  onBack 
}) => {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [completed, setCompleted] = useState(false);
  const [securityChecks, setSecurityChecks] = useState({
    backupSaved: false,
    termsAccepted: false,
    securityUnderstood: false,
  });

  const allChecksComplete = Object.values(securityChecks).every(check => check);

  const handleComplete = async () => {
    if (!allChecksComplete) {
      setError('Please complete all security checks before proceeding');
      return;
    }

    setLoading(true);
    setError(null);

    try {
      // Simulate activation process (replace with actual API call when backend is ready)
      await new Promise(resolve => setTimeout(resolve, 2000));
      
      setCompleted(true);
      setTimeout(() => {
        onComplete();
      }, 2000);
    } catch (error: any) {
      console.error('Activation error:', error);
      setError(error.message || 'Failed to activate dual-authentication');
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return (
      <div style={{ textAlign: 'center', padding: '60px 0' }}>
        <Spin size="large" />
        <Paragraph style={{ marginTop: 16 }}>
          Activating dual-authentication...
        </Paragraph>
      </div>
    );
  }

  if (completed) {
    return (
      <Result
        status="success"
        icon={<CheckCircleOutlined style={{ color: '#52c41a' }} />}
        title="Dual-Authentication Activated!"
        subTitle="Your account is now secured with dual-layer authentication. You can now access all BPCI features."
        extra={[
          <Button type="primary" size="large" onClick={onComplete} key="dashboard">
            Go to Dashboard
          </Button>
        ]}
      />
    );
  }

  return (
    <Space direction="vertical" size="large" style={{ width: '100%' }}>
      <div>
        <Title level={4}>
          <SafetyOutlined style={{ marginRight: 8, color: '#1890ff' }} />
          Step 3: Security Verification & Hardening
        </Title>
        <Paragraph type="secondary">
          Complete the final security checks to activate dual-authentication.
        </Paragraph>
      </div>

      {error && (
        <Alert
          message="Activation Error"
          description={error}
          type="error"
          closable
          onClose={() => setError(null)}
          showIcon
        />
      )}

      <Card title="🔒 Security Verification">
        <Space direction="vertical" size="middle" style={{ width: '100%' }}>
          <Alert
            message="Important Security Information"
            description="Please review and confirm the following security measures to protect your account."
            type="warning"
            showIcon
          />

          <div style={{ padding: '16px 0' }}>
            <Space direction="vertical" size="large" style={{ width: '100%' }}>
              <Checkbox
                checked={securityChecks.backupSaved}
                onChange={(e) => setSecurityChecks({
                  ...securityChecks,
                  backupSaved: e.target.checked
                })}
              >
                <Space direction="vertical" size={0}>
                  <Text strong>I have securely saved my BPI credentials</Text>
                  <Text type="secondary" style={{ fontSize: '12px' }}>
                    I have copied and stored my BPI address and authentication token in a secure location. 
                    I understand these cannot be recovered if lost.
                  </Text>
                </Space>
              </Checkbox>

              <Checkbox
                checked={securityChecks.termsAccepted}
                onChange={(e) => setSecurityChecks({
                  ...securityChecks,
                  termsAccepted: e.target.checked
                })}
              >
                <Space direction="vertical" size={0}>
                  <Text strong>I accept the terms and conditions</Text>
                  <Text type="secondary" style={{ fontSize: '12px' }}>
                    I have read and agree to the{' '}
                    <a href="/terms" target="_blank">Terms of Service</a> and{' '}
                    <a href="/privacy" target="_blank">Privacy Policy</a> for dual-authentication.
                  </Text>
                </Space>
              </Checkbox>

              <Checkbox
                checked={securityChecks.securityUnderstood}
                onChange={(e) => setSecurityChecks({
                  ...securityChecks,
                  securityUnderstood: e.target.checked
                })}
              >
                <Space direction="vertical" size={0}>
                  <Text strong>I understand the security implications</Text>
                  <Text type="secondary" style={{ fontSize: '12px' }}>
                    I understand that dual-authentication provides enhanced security and that I should 
                    never share my authentication token with anyone.
                  </Text>
                </Space>
              </Checkbox>
            </Space>
          </div>
        </Space>
      </Card>

      <Card title="✅ Activation Summary" style={{ background: '#f6ffed', borderColor: '#b7eb8f' }}>
        <Space direction="vertical" size="small" style={{ width: '100%' }}>
          <div style={{ display: 'flex', justifyContent: 'space-between' }}>
            <Text>Keycloak Account:</Text>
            <Text strong>{bindingData?.keycloak_id?.substring(0, 20)}...</Text>
          </div>
          <div style={{ display: 'flex', justifyContent: 'space-between' }}>
            <Text>BPI Address:</Text>
            <Text strong code style={{ fontSize: '11px' }}>
              {generatedData?.address?.substring(0, 30)}...
            </Text>
          </div>
          <div style={{ display: 'flex', justifyContent: 'space-between' }}>
            <Text>Connection Name:</Text>
            <Text strong>{generatedData?.name}</Text>
          </div>
          <div style={{ display: 'flex', justifyContent: 'space-between' }}>
            <Text>Status:</Text>
            <Text strong style={{ color: '#52c41a' }}>
              {allChecksComplete ? 'Ready to Activate' : 'Pending Security Checks'}
            </Text>
          </div>
        </Space>
      </Card>

      <Alert
        message="What happens after activation?"
        description={
          <ul style={{ marginBottom: 0, paddingLeft: 20 }}>
            <li>You'll have access to the full Mojo Wallet dashboard</li>
            <li>You can deploy and manage BPI OS nodes</li>
            <li>Advanced monitoring and analytics will be unlocked</li>
            <li>You can manage multiple wallets and connections</li>
            <li>Full API access with your authentication token</li>
          </ul>
        }
        type="info"
        showIcon
      />

      <div style={{ display: 'flex', justifyContent: 'space-between', marginTop: 24 }}>
        <Button onClick={onBack} size="large">
          ← Back
        </Button>
        <Button 
          type="primary" 
          size="large"
          icon={<LockOutlined />}
          onClick={handleComplete}
          loading={loading}
          disabled={!allChecksComplete}
        >
          Complete Activation
        </Button>
      </div>
    </Space>
  );
};

export default HardenStep;
