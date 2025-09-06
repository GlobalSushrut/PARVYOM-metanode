import React, { useState, useEffect } from 'react';
import { Card, Form, Input, Select, Button, Steps, Typography, Alert, Progress, Space, List, Tag } from 'antd';
import { 
  DownloadOutlined, 
  GithubOutlined, 
  CloudServerOutlined, 
  CheckCircleOutlined,
  LoadingOutlined,
  ExclamationCircleOutlined
} from '@ant-design/icons';
import { registryService } from '../../services/registryService';
import type { InstallerStatus } from '../../services/registryService';

const { Title, Text, Paragraph } = Typography;
const { Option } = Select;
const { Step } = Steps;

export const BPIInstaller: React.FC = () => {
  const [form] = Form.useForm();
  const [currentStep, setCurrentStep] = useState(0);
  const [loading, setLoading] = useState(false);
  const [installerId, setInstallerId] = useState<string | null>(null);
  const [installerStatus, setInstallerStatus] = useState<InstallerStatus | null>(null);
  const [logs, setLogs] = useState<string[]>([]);
  const [error, setError] = useState<string | null>(null);

  const pollInstallerStatus = async (id: string) => {
    try {
      const status = await registryService.getInstallerStatus(id);
      const currentLogs = await registryService.getInstallerLogs(id);
      
      if (status) {
        setInstallerStatus(status);
        
        if (status.status === 'Completed') {
          setCurrentStep(3);
          setLoading(false);
        } else if (status.status === 'Failed') {
          setError('Installation failed. Check logs for details.');
          setLoading(false);
        }
      }
      
      if (currentLogs) {
        setLogs(currentLogs);
      }
    } catch (err) {
      console.error('Failed to poll installer status:', err);
    }
  };

  useEffect(() => {
    if (installerId && installerStatus?.status === 'Installing') {
      const interval = setInterval(() => {
        pollInstallerStatus(installerId);
      }, 2000);
      
      return () => clearInterval(interval);
    }
  }, [installerId, installerStatus?.status]);

  const handleInstallation = async (values: any) => {
    setLoading(true);
    setError(null);
    setCurrentStep(1);

    try {
      const result = await registryService.installBPINode({
        node_type: values.node_type,
        installation_path: values.installation_path,
        github_repo: values.github_repo,
        community_config: values.community_config ? JSON.parse(values.community_config) : undefined
      });

      if (result.success && result.installer_id) {
        setInstallerId(result.installer_id);
        setCurrentStep(2);
        // Start polling for status
        pollInstallerStatus(result.installer_id);
      } else {
        setError(result.message);
        setLoading(false);
        setCurrentStep(0);
      }
    } catch (err) {
      setError('Failed to start installation');
      setLoading(false);
      setCurrentStep(0);
    }
  };

  const getStepStatus = (step: number) => {
    if (currentStep > step) return 'finish';
    if (currentStep === step) {
      if (loading) return 'process';
      if (error) return 'error';
      return 'process';
    }
    return 'wait';
  };

  const getStepIcon = (step: number) => {
    const status = getStepStatus(step);
    if (status === 'process' && loading) return <LoadingOutlined />;
    if (status === 'error') return <ExclamationCircleOutlined />;
    if (status === 'finish') return <CheckCircleOutlined />;
    return undefined;
  };

  const renderConfigurationStep = () => (
    <Card title="BPI Node Configuration" style={{ marginTop: '24px' }}>
      <Form
        form={form}
        layout="vertical"
        onFinish={handleInstallation}
        size="large"
      >
        <Form.Item
          name="node_type"
          label="Node Type"
          rules={[{ required: true, message: 'Please select a node type' }]}
        >
          <Select placeholder="Select the type of BPI node to install">
            <Option value="BpiCommunity">
              <Space>
                <CloudServerOutlined />
                BPI Community Node
              </Space>
            </Option>
            <Option value="BpciEnterprise">
              <Space>
                <CloudServerOutlined />
                BPCI Enterprise Node
              </Space>
            </Option>
            <Option value="Hybrid">
              <Space>
                <CloudServerOutlined />
                Hybrid Node
              </Space>
            </Option>
            <Option value="BankApiRegistry">
              <Space>
                <CloudServerOutlined />
                Bank API Registry Node
              </Space>
            </Option>
            <Option value="GovernmentApiRegistry">
              <Space>
                <CloudServerOutlined />
                Government API Registry Node
              </Space>
            </Option>
          </Select>
        </Form.Item>

        <Form.Item
          name="installation_path"
          label="Installation Path"
          rules={[{ required: true, message: 'Please specify installation path' }]}
        >
          <Input 
            placeholder="/opt/bpi-node" 
            addonBefore="Path:"
          />
        </Form.Item>

        <Form.Item
          name="github_repo"
          label="GitHub Repository (Optional)"
        >
          <Input 
            placeholder="https://github.com/user/bpi-node-config" 
            addonBefore={<GithubOutlined />}
          />
        </Form.Item>

        <Form.Item
          name="community_config"
          label="Community Configuration (JSON, Optional)"
        >
          <Input.TextArea 
            placeholder='{"community_name": "My Community", "initial_validators": 3}'
            rows={4}
          />
        </Form.Item>

        <Form.Item>
          <Button
            type="primary"
            htmlType="submit"
            size="large"
            block
            icon={<DownloadOutlined />}
            style={{
              background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
              border: 'none',
              height: '48px'
            }}
          >
            Start BPI Node Installation
          </Button>
        </Form.Item>
      </Form>
    </Card>
  );

  const renderInstallationProgress = () => (
    <Card title="Installation Progress" style={{ marginTop: '24px' }}>
      {installerStatus && (
        <div style={{ marginBottom: '24px' }}>
          <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '8px' }}>
            <Text strong>Status: </Text>
            <Tag color={
              installerStatus.status === 'Installing' ? 'processing' :
              installerStatus.status === 'Completed' ? 'success' :
              installerStatus.status === 'Failed' ? 'error' : 'default'
            }>
              {installerStatus.status}
            </Tag>
          </div>
          
          <Progress 
            percent={installerStatus.progress} 
            status={
              installerStatus.status === 'Failed' ? 'exception' :
              installerStatus.status === 'Completed' ? 'success' : 'active'
            }
            strokeColor={{
              '0%': '#667eea',
              '100%': '#764ba2',
            }}
          />
          
          <div style={{ marginTop: '16px' }}>
            <Text type="secondary">
              Estimated completion: {new Date(installerStatus.estimated_completion).toLocaleString()}
            </Text>
          </div>
        </div>
      )}

      {logs.length > 0 && (
        <div>
          <Title level={5}>Installation Logs:</Title>
          <div style={{ 
            background: '#001529', 
            color: '#fff', 
            padding: '16px', 
            borderRadius: '6px',
            fontFamily: 'monospace',
            fontSize: '12px',
            maxHeight: '300px',
            overflowY: 'auto'
          }}>
            {logs.map((log, index) => (
              <div key={index} style={{ marginBottom: '4px' }}>
                {log}
              </div>
            ))}
          </div>
        </div>
      )}
    </Card>
  );

  const renderCompletionStep = () => (
    <Card title="Installation Complete" style={{ marginTop: '24px' }}>
      <div style={{ textAlign: 'center', padding: '32px' }}>
        <CheckCircleOutlined style={{ fontSize: '64px', color: '#52c41a', marginBottom: '16px' }} />
        <Title level={3} style={{ color: '#52c41a' }}>
          BPI Node Successfully Installed!
        </Title>
        <Paragraph>
          Your BPI node has been successfully installed and configured. 
          The node is now ready to join the BPCI network.
        </Paragraph>
        
        {installerStatus && (
          <div style={{ marginTop: '24px' }}>
            <List
              size="small"
              bordered
              dataSource={[
                { label: 'Installer ID', value: installerStatus.installer_id },
                { label: 'Node Type', value: installerStatus.node_type },
                { label: 'Installation Time', value: new Date(installerStatus.created_at).toLocaleString() },
                { label: 'Status', value: installerStatus.status }
              ]}
              renderItem={(item) => (
                <List.Item>
                  <div style={{ display: 'flex', justifyContent: 'space-between', width: '100%' }}>
                    <Text strong>{item.label}:</Text>
                    <Text>{item.value}</Text>
                  </div>
                </List.Item>
              )}
            />
          </div>
        )}

        <div style={{ marginTop: '24px' }}>
          <Space>
            <Button 
              type="primary" 
              onClick={() => {
                setCurrentStep(0);
                setInstallerId(null);
                setInstallerStatus(null);
                setLogs([]);
                form.resetFields();
              }}
            >
              Install Another Node
            </Button>
            <Button onClick={() => console.log('View node in registry')}>
              View in Registry
            </Button>
          </Space>
        </div>
      </div>
    </Card>
  );

  return (
    <div style={{ padding: '24px', maxWidth: '800px', margin: '0 auto' }}>
      <div style={{ textAlign: 'center', marginBottom: '32px' }}>
        <Title level={2}>
          <DownloadOutlined style={{ marginRight: '12px', color: '#667eea' }} />
          BPI Node Installer
        </Title>
        <Paragraph>
          Install and configure BPI nodes for community, enterprise, or specialized use cases.
          Connect to GitHub repositories for automated deployment and configuration.
        </Paragraph>
      </div>

      {error && (
        <Alert
          message="Installation Error"
          description={error}
          type="error"
          showIcon
          closable
          onClose={() => setError(null)}
          style={{ marginBottom: '24px' }}
        />
      )}

      <Steps current={currentStep} style={{ marginBottom: '32px' }}>
        <Step 
          title="Configuration" 
          description="Configure node settings"
          icon={getStepIcon(0)}
        />
        <Step 
          title="Download" 
          description="Download and prepare"
          icon={getStepIcon(1)}
        />
        <Step 
          title="Installation" 
          description="Install and configure"
          icon={getStepIcon(2)}
        />
        <Step 
          title="Complete" 
          description="Ready to use"
          icon={getStepIcon(3)}
        />
      </Steps>

      {currentStep === 0 && renderConfigurationStep()}
      {(currentStep === 1 || currentStep === 2) && renderInstallationProgress()}
      {currentStep === 3 && renderCompletionStep()}
    </div>
  );
};
