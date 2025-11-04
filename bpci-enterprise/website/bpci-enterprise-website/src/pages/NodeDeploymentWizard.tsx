import React, { useState } from 'react';
import {
  Card,
  Steps,
  Button,
  Typography,
  Space,
  Form,
  Input,
  Select,
  Radio,
  Checkbox,
  Alert,
  Progress,
  Result,
  Divider,
  Tag
} from 'antd';
import {
  RocketOutlined,
  SettingOutlined,
  CloudServerOutlined,
  CheckCircleOutlined,
  DownloadOutlined,
  PlayCircleOutlined
} from '@ant-design/icons';

const { Title, Text, Paragraph } = Typography;
const { Option } = Select;

type DeploymentStep = 'select' | 'configure' | 'download' | 'deploy' | 'complete';

const NodeDeploymentWizard: React.FC = () => {
  const [currentStep, setCurrentStep] = useState<DeploymentStep>('select');
  const [form] = Form.useForm();
  const [deploymentConfig, setDeploymentConfig] = useState<any>({});
  const [deploymentProgress, setDeploymentProgress] = useState(0);
  const [deploying, setDeploying] = useState(false);

  const steps = [
    { title: 'Select', icon: <CloudServerOutlined /> },
    { title: 'Configure', icon: <SettingOutlined /> },
    { title: 'Download', icon: <DownloadOutlined /> },
    { title: 'Deploy', icon: <PlayCircleOutlined /> },
    { title: 'Complete', icon: <CheckCircleOutlined /> }
  ];

  const handleSelectComplete = (values: any) => {
    setDeploymentConfig({ ...deploymentConfig, ...values });
    setCurrentStep('configure');
  };

  const handleConfigureComplete = (values: any) => {
    setDeploymentConfig({ ...deploymentConfig, ...values });
    setCurrentStep('download');
  };

  const handleDownloadComplete = () => {
    setCurrentStep('deploy');
  };

  const handleDeploy = async () => {
    setDeploying(true);
    setDeploymentProgress(0);

    // Simulate deployment progress
    const interval = setInterval(() => {
      setDeploymentProgress(prev => {
        if (prev >= 100) {
          clearInterval(interval);
          setDeploying(false);
          setCurrentStep('complete');
          return 100;
        }
        return prev + 10;
      });
    }, 500);
  };

  const renderSelectStep = () => (
    <Card>
      <Title level={4}>
        <CloudServerOutlined style={{ marginRight: 8, color: '#1890ff' }} />
        Step 1: Select Deployment Type
      </Title>
      <Paragraph type="secondary">
        Choose how you want to deploy your BPI Immutable OS node
      </Paragraph>

      <Form form={form} layout="vertical" onFinish={handleSelectComplete}>
        <Form.Item
          label="Deployment Platform"
          name="platform"
          rules={[{ required: true, message: 'Please select a platform' }]}
        >
          <Radio.Group size="large">
            <Space direction="vertical" style={{ width: '100%' }}>
              <Radio value="raspberry-pi">
                <Space direction="vertical" size={0}>
                  <Text strong>Raspberry Pi</Text>
                  <Text type="secondary" style={{ fontSize: '12px' }}>
                    Deploy on Raspberry Pi 4 (8GB RAM recommended)
                  </Text>
                </Space>
              </Radio>
              <Radio value="cloud">
                <Space direction="vertical" size={0}>
                  <Text strong>Cloud Server</Text>
                  <Text type="secondary" style={{ fontSize: '12px' }}>
                    Deploy on DigitalOcean, AWS, or other cloud providers
                  </Text>
                </Space>
              </Radio>
              <Radio value="local">
                <Space direction="vertical" size={0}>
                  <Text strong>Local Machine</Text>
                  <Text type="secondary" style={{ fontSize: '12px' }}>
                    Deploy on your local Linux machine
                  </Text>
                </Space>
              </Radio>
            </Space>
          </Radio.Group>
        </Form.Item>

        <Form.Item
          label="Node Type"
          name="nodeType"
          rules={[{ required: true, message: 'Please select node type' }]}
        >
          <Select placeholder="Select node type" size="large">
            <Option value="full">Full Node (Validator + Miner)</Option>
            <Option value="validator">Validator Node Only</Option>
            <Option value="miner">Miner Node Only</Option>
            <Option value="light">Light Node (Observer)</Option>
          </Select>
        </Form.Item>

        <Form.Item>
          <Button type="primary" htmlType="submit" size="large" block>
            Continue to Configuration →
          </Button>
        </Form.Item>
      </Form>
    </Card>
  );

  const renderConfigureStep = () => (
    <Card>
      <Title level={4}>
        <SettingOutlined style={{ marginRight: 8, color: '#1890ff' }} />
        Step 2: Configure Node Settings
      </Title>
      <Paragraph type="secondary">
        Configure your BPI OS node parameters
      </Paragraph>

      <Form form={form} layout="vertical" onFinish={handleConfigureComplete}>
        <Form.Item
          label="Node Name"
          name="nodeName"
          rules={[{ required: true, message: 'Please enter node name' }]}
        >
          <Input placeholder="e.g., my-bpi-node" size="large" />
        </Form.Item>

        <Form.Item
          label="Network"
          name="network"
          rules={[{ required: true, message: 'Please select network' }]}
          initialValue="mainnet"
        >
          <Radio.Group size="large">
            <Radio value="mainnet">Mainnet</Radio>
            <Radio value="testnet">Testnet</Radio>
            <Radio value="devnet">Devnet</Radio>
          </Radio.Group>
        </Form.Item>

        <Form.Item
          label="Resource Allocation"
          name="resources"
        >
          <Space direction="vertical" style={{ width: '100%' }}>
            <div>
              <Text>Memory (MB):</Text>
              <Input type="number" defaultValue={2048} size="large" />
            </div>
            <div>
              <Text>CPU Cores:</Text>
              <Input type="number" defaultValue={2} size="large" />
            </div>
            <div>
              <Text>Disk Space (GB):</Text>
              <Input type="number" defaultValue={50} size="large" />
            </div>
          </Space>
        </Form.Item>

        <Form.Item
          name="enableMonitoring"
          valuePropName="checked"
          initialValue={true}
        >
          <Checkbox>Enable monitoring and metrics</Checkbox>
        </Form.Item>

        <Form.Item
          name="enableAutoUpdate"
          valuePropName="checked"
          initialValue={false}
        >
          <Checkbox>Enable automatic updates</Checkbox>
        </Form.Item>

        <Form.Item>
          <Space style={{ width: '100%', justifyContent: 'space-between' }}>
            <Button onClick={() => setCurrentStep('select')} size="large">
              ← Back
            </Button>
            <Button type="primary" htmlType="submit" size="large">
              Continue to Download →
            </Button>
          </Space>
        </Form.Item>
      </Form>
    </Card>
  );

  const renderDownloadStep = () => (
    <Card>
      <Title level={4}>
        <DownloadOutlined style={{ marginRight: 8, color: '#1890ff' }} />
        Step 3: Download BPI OS Installer
      </Title>
      <Paragraph type="secondary">
        Download the BPI Immutable OS installer for your platform
      </Paragraph>

      <Alert
        message="Configuration Summary"
        description={
          <div>
            <p><strong>Platform:</strong> {deploymentConfig.platform}</p>
            <p><strong>Node Type:</strong> {deploymentConfig.nodeType}</p>
            <p><strong>Node Name:</strong> {deploymentConfig.nodeName}</p>
            <p><strong>Network:</strong> {deploymentConfig.network}</p>
          </div>
        }
        type="info"
        showIcon
        style={{ marginBottom: 24 }}
      />

      <Card style={{ background: '#f6ffed', marginBottom: 24 }}>
        <Space direction="vertical" style={{ width: '100%' }}>
          <Title level={5}>📦 BPI Immutable OS Installer</Title>
          <Text>Version: 1.0.0 (Latest)</Text>
          <Text>Size: 2.5 GB</Text>
          <Text>SHA256: a3f2b9c8...</Text>
          <Divider />
          <Button
            type="primary"
            size="large"
            icon={<DownloadOutlined />}
            block
          >
            Download Installer (2.5 GB)
          </Button>
        </Space>
      </Card>

      <Alert
        message="Installation Instructions"
        description={
          <ol>
            <li>Download the installer package</li>
            <li>Verify the SHA256 checksum</li>
            <li>Extract the package to your target system</li>
            <li>Run the installation script</li>
            <li>Follow the on-screen prompts</li>
          </ol>
        }
        type="warning"
        showIcon
      />

      <div style={{ marginTop: 24 }}>
        <Space style={{ width: '100%', justifyContent: 'space-between' }}>
          <Button onClick={() => setCurrentStep('configure')} size="large">
            ← Back
          </Button>
          <Button type="primary" onClick={handleDownloadComplete} size="large">
            Continue to Deployment →
          </Button>
        </Space>
      </div>
    </Card>
  );

  const renderDeployStep = () => (
    <Card>
      <Title level={4}>
        <PlayCircleOutlined style={{ marginRight: 8, color: '#1890ff' }} />
        Step 4: Deploy Node
      </Title>
      <Paragraph type="secondary">
        Deploy your BPI OS node using BSO-K8 orchestrator
      </Paragraph>

      {!deploying && deploymentProgress === 0 && (
        <>
          <Alert
            message="Ready to Deploy"
            description="Click the button below to start the deployment process. This will deploy your BPI OS node using the BSO-K8 orchestrator (no Docker, no Kubernetes)."
            type="success"
            showIcon
            style={{ marginBottom: 24 }}
          />

          <Button
            type="primary"
            size="large"
            icon={<RocketOutlined />}
            onClick={handleDeploy}
            block
          >
            Start Deployment
          </Button>
        </>
      )}

      {(deploying || deploymentProgress > 0) && (
        <Space direction="vertical" style={{ width: '100%' }}>
          <Progress percent={deploymentProgress} status={deploying ? 'active' : 'success'} />
          <Text type="secondary">
            {deploymentProgress < 20 && 'Initializing BSO-K8 orchestrator...'}
            {deploymentProgress >= 20 && deploymentProgress < 40 && 'Creating vPod clusters...'}
            {deploymentProgress >= 40 && deploymentProgress < 60 && 'Deploying BPI OS services...'}
            {deploymentProgress >= 60 && deploymentProgress < 80 && 'Configuring network and storage...'}
            {deploymentProgress >= 80 && deploymentProgress < 100 && 'Starting node services...'}
            {deploymentProgress === 100 && 'Deployment complete!'}
          </Text>
        </Space>
      )}
    </Card>
  );

  const renderCompleteStep = () => (
    <Result
      status="success"
      title="Node Deployed Successfully!"
      subTitle={`Your BPI OS node "${deploymentConfig.nodeName}" is now running on ${deploymentConfig.network}.`}
      extra={[
        <Button type="primary" size="large" key="dashboard" onClick={() => window.location.href = '/node-management'}>
          Go to Node Dashboard
        </Button>,
        <Button size="large" key="deploy-another" onClick={() => {
          setCurrentStep('select');
          setDeploymentProgress(0);
          form.resetFields();
        }}>
          Deploy Another Node
        </Button>
      ]}
    >
      <Card style={{ marginTop: 24 }}>
        <Space direction="vertical" style={{ width: '100%' }}>
          <Title level={5}>Node Information</Title>
          <p><strong>Node ID:</strong> node_{Math.random().toString(36).substring(7)}</p>
          <p><strong>Status:</strong> <Tag color="green">Running</Tag></p>
          <p><strong>Endpoint:</strong> http://your-node-ip:7777</p>
          <p><strong>Network:</strong> {deploymentConfig.network}</p>
        </Space>
      </Card>
    </Result>
  );

  const getCurrentStepIndex = () => {
    const stepMap: Record<DeploymentStep, number> = {
      'select': 0,
      'configure': 1,
      'download': 2,
      'deploy': 3,
      'complete': 4
    };
    return stepMap[currentStep];
  };

  return (
    <div style={{ padding: '24px', maxWidth: '900px', margin: '0 auto' }}>
      <div style={{ marginBottom: 24, textAlign: 'center' }}>
        <Title level={2}>
          <RocketOutlined style={{ marginRight: 8, color: '#1890ff' }} />
          Deploy BPI OS Node
        </Title>
        <Text type="secondary">
          Deploy a BPI Immutable OS node in 4 easy steps
        </Text>
      </div>

      <Steps current={getCurrentStepIndex()} items={steps} style={{ marginBottom: 32 }} />

      {currentStep === 'select' && renderSelectStep()}
      {currentStep === 'configure' && renderConfigureStep()}
      {currentStep === 'download' && renderDownloadStep()}
      {currentStep === 'deploy' && renderDeployStep()}
      {currentStep === 'complete' && renderCompleteStep()}
    </div>
  );
};

export default NodeDeploymentWizard;
