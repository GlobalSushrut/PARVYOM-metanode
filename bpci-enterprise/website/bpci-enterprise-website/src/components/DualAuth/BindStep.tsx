import React, { useState } from 'react';
import { Button, Typography, Space, Alert, Card, Spin, Descriptions } from 'antd';
import { LinkOutlined, CheckCircleOutlined, UserOutlined } from '@ant-design/icons';
import { apiService } from '../../services/api';
import { keycloakService } from '../../services/keycloakService';

const { Title, Text, Paragraph } = Typography;

interface BindStepProps {
  generatedData: any;
  onComplete: (data: any) => void;
  onBack: () => void;
}

const BindStep: React.FC<BindStepProps> = ({ generatedData, onComplete, onBack }) => {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [bound, setBound] = useState(false);
  const [keycloakUser, setKeycloakUser] = useState<any>(null);

  React.useEffect(() => {
    loadKeycloakUser();
  }, []);

  const loadKeycloakUser = async () => {
    try {
      const authState = keycloakService.getAuthState();
      setKeycloakUser(authState.user);
    } catch (error) {
      console.error('Failed to load Keycloak user:', error);
    }
  };

  const handleBind = async () => {
    setLoading(true);
    setError(null);

    try {
      // Simulate binding process (replace with actual API call when backend is ready)
      await new Promise(resolve => setTimeout(resolve, 1500));
      
      setBound(true);
      setTimeout(() => {
        onComplete({
          keycloak_id: keycloakUser?.id,
          bpi_address: generatedData.address,
          bound_at: new Date().toISOString(),
        });
      }, 1500);
    } catch (error: any) {
      console.error('Bind error:', error);
      setError(error.message || 'Failed to bind Keycloak account');
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return (
      <div style={{ textAlign: 'center', padding: '60px 0' }}>
        <Spin size="large" />
        <Paragraph style={{ marginTop: 16 }}>
          Binding your BPI connection to Keycloak...
        </Paragraph>
      </div>
    );
  }

  if (bound) {
    return (
      <div style={{ textAlign: 'center', padding: '40px 0' }}>
        <CheckCircleOutlined style={{ fontSize: '64px', color: '#52c41a' }} />
        <Title level={3} style={{ marginTop: 16 }}>
          Successfully Bound!
        </Title>
        <Paragraph type="secondary">
          Your BPI connection is now linked to your Keycloak account.
        </Paragraph>
      </div>
    );
  }

  return (
    <Space direction="vertical" size="large" style={{ width: '100%' }}>
      <div>
        <Title level={4}>
          <LinkOutlined style={{ marginRight: 8, color: '#1890ff' }} />
          Step 2: Bind to Keycloak Account
        </Title>
        <Paragraph type="secondary">
          Link your BPI connection to your Keycloak account for seamless dual-authentication.
        </Paragraph>
      </div>

      {error && (
        <Alert
          message="Binding Error"
          description={error}
          type="error"
          closable
          onClose={() => setError(null)}
          showIcon
        />
      )}

      <Card title="🔗 Connection Details">
        <Descriptions column={1} bordered>
          <Descriptions.Item label="Keycloak Account">
            <Space>
              <UserOutlined />
              <Text strong>{keycloakUser?.email || 'Loading...'}</Text>
            </Space>
          </Descriptions.Item>
          <Descriptions.Item label="Keycloak ID">
            <Text code>{keycloakUser?.id || 'Loading...'}</Text>
          </Descriptions.Item>
          <Descriptions.Item label="BPI Address">
            <Text code style={{ fontSize: '12px' }}>{generatedData?.address}</Text>
          </Descriptions.Item>
          <Descriptions.Item label="Connection Name">
            <Text>{generatedData?.name}</Text>
          </Descriptions.Item>
        </Descriptions>
      </Card>

      <Alert
        message="What does binding do?"
        description={
          <ul style={{ marginBottom: 0, paddingLeft: 20 }}>
            <li>Creates a secure link between your Keycloak account and BPI wallet</li>
            <li>Enables dual-layer authentication for enhanced security</li>
            <li>Allows you to use both Keycloak SSO and BPI wallet for login</li>
            <li>Synchronizes your identity across both authentication systems</li>
          </ul>
        }
        type="info"
        showIcon
      />

      <Card style={{ background: '#f0f2f5', border: 'none' }}>
        <Space direction="vertical" size="small" style={{ width: '100%' }}>
          <Text strong>Ready to bind?</Text>
          <Paragraph type="secondary" style={{ marginBottom: 0 }}>
            Click the button below to link your BPI connection to your Keycloak account. 
            This process is secure and reversible.
          </Paragraph>
        </Space>
      </Card>

      <div style={{ display: 'flex', justifyContent: 'space-between', marginTop: 24 }}>
        <Button onClick={onBack} size="large">
          ← Back
        </Button>
        <Button 
          type="primary" 
          size="large"
          icon={<LinkOutlined />}
          onClick={handleBind}
          loading={loading}
        >
          Bind to Keycloak
        </Button>
      </div>
    </Space>
  );
};

export default BindStep;
