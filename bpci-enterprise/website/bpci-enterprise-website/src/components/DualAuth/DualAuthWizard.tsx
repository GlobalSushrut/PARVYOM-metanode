import React, { useState } from 'react';
import { Steps, Card, Button, Typography, Space, message } from 'antd';
import { 
  RocketOutlined, 
  LinkOutlined, 
  SafetyOutlined,
  CheckCircleOutlined 
} from '@ant-design/icons';
import GenerateStep from './GenerateStep';
import BindStep from './BindStep';
import HardenStep from './HardenStep';

const { Title, Text } = Typography;

interface DualAuthWizardProps {
  onComplete: () => void;
  onCancel: () => void;
}

const DualAuthWizard: React.FC<DualAuthWizardProps> = ({ onComplete, onCancel }) => {
  const [currentStep, setCurrentStep] = useState(0);
  const [generatedData, setGeneratedData] = useState<any>(null);
  const [bindingData, setBindingData] = useState<any>(null);

  const steps = [
    {
      title: 'Generate',
      icon: <RocketOutlined />,
      description: 'Create BPI Connection',
    },
    {
      title: 'Bind',
      icon: <LinkOutlined />,
      description: 'Link to Keycloak',
    },
    {
      title: 'Harden',
      icon: <SafetyOutlined />,
      description: 'Security Verification',
    },
  ];

  const handleGenerateComplete = (data: any) => {
    setGeneratedData(data);
    setCurrentStep(1);
    message.success('BPI connection generated successfully!');
  };

  const handleBindComplete = (data: any) => {
    setBindingData(data);
    setCurrentStep(2);
    message.success('Keycloak binding successful!');
  };

  const handleHardenComplete = () => {
    message.success('Dual-authentication activated! 🎉');
    onComplete();
  };

  const handleBack = () => {
    if (currentStep > 0) {
      setCurrentStep(currentStep - 1);
    }
  };

  return (
    <div style={{ padding: '24px', maxWidth: '900px', margin: '0 auto' }}>
      <Card>
        <Space direction="vertical" size="large" style={{ width: '100%' }}>
          {/* Header */}
          <div style={{ textAlign: 'center' }}>
            <Title level={2}>
              <CheckCircleOutlined style={{ color: '#52c41a', marginRight: 8 }} />
              Activate Dual-Authentication
            </Title>
            <Text type="secondary" style={{ fontSize: '16px' }}>
              Complete this 3-step wizard to unlock full BPCI features
            </Text>
          </div>

          {/* Progress Steps */}
          <Steps 
            current={currentStep} 
            items={steps}
            style={{ marginTop: 24, marginBottom: 32 }}
          />

          {/* Step Content */}
          <div style={{ minHeight: '400px' }}>
            {currentStep === 0 && (
              <GenerateStep 
                onComplete={handleGenerateComplete}
                onCancel={onCancel}
              />
            )}
            
            {currentStep === 1 && (
              <BindStep 
                generatedData={generatedData}
                onComplete={handleBindComplete}
                onBack={handleBack}
              />
            )}
            
            {currentStep === 2 && (
              <HardenStep 
                generatedData={generatedData}
                bindingData={bindingData}
                onComplete={handleHardenComplete}
                onBack={handleBack}
              />
            )}
          </div>
        </Space>
      </Card>
    </div>
  );
};

export default DualAuthWizard;
