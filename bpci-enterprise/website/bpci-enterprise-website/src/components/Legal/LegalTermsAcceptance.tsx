import React, { useState } from 'react';
import { Checkbox, Space, Typography, Modal, Button } from 'antd';
import { FileProtectOutlined, SafetyCertificateOutlined } from '@ant-design/icons';
import { Link } from 'react-router-dom';

const { Text, Paragraph } = Typography;

interface LegalTermsAcceptanceProps {
  onAcceptanceChange: (accepted: boolean) => void;
  required?: boolean;
  showModal?: boolean;
}

const LegalTermsAcceptance: React.FC<LegalTermsAcceptanceProps> = ({
  onAcceptanceChange,
  required = true,
  showModal = false
}) => {
  const [termsAccepted, setTermsAccepted] = useState(false);
  const [privacyAccepted, setPrivacyAccepted] = useState(false);
  const [modalVisible, setModalVisible] = useState(showModal);

  const handleTermsChange = (checked: boolean) => {
    setTermsAccepted(checked);
    updateAcceptance(checked, privacyAccepted);
  };

  const handlePrivacyChange = (checked: boolean) => {
    setPrivacyAccepted(checked);
    updateAcceptance(termsAccepted, checked);
  };

  const updateAcceptance = (terms: boolean, privacy: boolean) => {
    const allAccepted = terms && privacy;
    onAcceptanceChange(allAccepted);
  };

  const LegalContent = () => (
    <div style={{ padding: '16px 0' }}>
      <Space direction="vertical" size="large" style={{ width: '100%' }}>
        <div>
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px', marginBottom: '12px' }}>
            <FileProtectOutlined style={{ color: '#3b82f6', fontSize: '18px' }} />
            <Text strong style={{ color: '#ffffff' }}>Terms of Service</Text>
          </div>
          <Checkbox
            checked={termsAccepted}
            onChange={(e) => handleTermsChange(e.target.checked)}
            style={{ marginBottom: '8px' }}
          >
            <Text style={{ color: '#e2e8f0' }}>
              I have read and agree to the{' '}
              <Link 
                to="/terms-of-service" 
                target="_blank" 
                style={{ color: '#60a5fa', textDecoration: 'underline' }}
              >
                Terms of Service
              </Link>
              {required && <Text style={{ color: '#ef4444' }}> *</Text>}
            </Text>
          </Checkbox>
          <Paragraph style={{ color: '#94a3b8', fontSize: '0.875rem', margin: '4px 0 0 24px' }}>
            By accepting, you agree to use PARVYOM experimental infrastructure for testing and pilot purposes only.
          </Paragraph>
        </div>

        <div>
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px', marginBottom: '12px' }}>
            <SafetyCertificateOutlined style={{ color: '#10b981', fontSize: '18px' }} />
            <Text strong style={{ color: '#ffffff' }}>Privacy Policy</Text>
          </div>
          <Checkbox
            checked={privacyAccepted}
            onChange={(e) => handlePrivacyChange(e.target.checked)}
            style={{ marginBottom: '8px' }}
          >
            <Text style={{ color: '#e2e8f0' }}>
              I have read and agree to the{' '}
              <Link 
                to="/privacy-policy" 
                target="_blank" 
                style={{ color: '#60a5fa', textDecoration: 'underline' }}
              >
                Privacy Policy
              </Link>
              {required && <Text style={{ color: '#ef4444' }}> *</Text>}
            </Text>
          </Checkbox>
          <Paragraph style={{ color: '#94a3b8', fontSize: '0.875rem', margin: '4px 0 0 24px' }}>
            We are committed to protecting your privacy and implementing advanced security measures.
          </Paragraph>
        </div>

        <div style={{ 
          background: 'rgba(245, 158, 11, 0.1)', 
          border: '1px solid rgba(245, 158, 11, 0.3)',
          borderRadius: '8px',
          padding: '16px',
          marginTop: '16px'
        }}>
          <Text style={{ color: '#fbbf24', fontWeight: '600', fontSize: '0.875rem' }}>
            ⚠️ Experimental Technology Notice
          </Text>
          <Paragraph style={{ color: '#fde68a', fontSize: '0.875rem', margin: '8px 0 0 0' }}>
            PARVYOM is experimental blockchain infrastructure. By proceeding, you acknowledge 
            this is for testing and pilot programs only, not production use.
          </Paragraph>
        </div>
      </Space>
    </div>
  );

  if (showModal) {
    return (
      <>
        <Modal
          title="Legal Terms & Conditions"
          open={modalVisible}
          onCancel={() => setModalVisible(false)}
          footer={[
            <Button key="cancel" onClick={() => setModalVisible(false)}>
              Cancel
            </Button>,
            <Button 
              key="accept" 
              type="primary" 
              disabled={!termsAccepted || !privacyAccepted}
              onClick={() => {
                setModalVisible(false);
                onAcceptanceChange(true);
              }}
            >
              Accept All Terms
            </Button>
          ]}
          width={600}
          style={{ top: 20 }}
        >
          <LegalContent />
        </Modal>
        <Button 
          type="link" 
          onClick={() => setModalVisible(true)}
          style={{ padding: 0, height: 'auto' }}
        >
          View Legal Terms & Conditions
        </Button>
      </>
    );
  }

  return <LegalContent />;
};

export default LegalTermsAcceptance;
