/**
 * BPI OS Kernel Connection Form Component
 * Handles BPI Blockchain Operating System kernel connection with advanced OS-level configuration
 */

import React, { useState, useEffect } from 'react';
import {
  Form,
  Input,
  Button,
  Card,
  Steps,
  Alert,
  Space,
  Switch,
  Select,
  InputNumber,
  Divider,
  Typography,
  Progress,
  Tag,
  Tooltip,
  Row,
  Col,
  Slider,
  Collapse,
  Badge,
  Spin,
} from 'antd';
import {
  LinkOutlined,
  SecurityScanOutlined,
  ThunderboltOutlined,
  CheckCircleOutlined,
  ExclamationCircleOutlined,
  SettingOutlined,
  DatabaseOutlined,
  NodeIndexOutlined,
  DesktopOutlined,
  CloudServerOutlined,
  GlobalOutlined,
  HddOutlined,
  LoadingOutlined,
  InfoCircleOutlined,
} from '@ant-design/icons';
import { useRBAC } from '../Security/RoleBasedAccess';

const { Title, Text } = Typography;
const { Option } = Select;
const { Panel } = Collapse;
const { Step } = Steps;

// Process type configuration
export interface ProcessTypeConfig {
  type: 'governance' | 'orchestration' | 'api' | 'background' | 'security' | 'audit';
  enabled: boolean;
  priority: 'low' | 'normal' | 'high' | 'critical';
  instances: number;
}

// Resource allocation configuration
export interface ResourceAllocationConfig {
  cpuCores: number;
  memoryGB: number;
  networkBandwidthMbps: number;
  storageGB: number;
  gpuUnits: number;
  priority: 'low' | 'normal' | 'high' | 'critical';
}

// Security context configuration
export interface SecurityContextConfig {
  permissions: string[];
  accessLevel: 'read' | 'write' | 'admin' | 'kernel';
  encryptionEnabled: boolean;
  auditEnabled: boolean;
}

// Scaling policy configuration
export interface ScalingPolicyConfig {
  minInstances: number;
  maxInstances: number;
  cpuThreshold: number;
  memoryThreshold: number;
  autoScaling: boolean;
}

// BPI OS Kernel connection configuration interface
export interface BpiOSKernelConnectionConfig {
  // Kernel Bridge Configuration
  kernelAddress: string;
  kernelToken: string;
  bridgeId: string;
  
  // Network and OS Configuration
  networkType: 'mainnet' | 'testnet' | 'development';
  osVersion: string;
  kernelVersion: string;
  
  // Process Mapping Configuration
  enableProcessMapping: boolean;
  maxProcesses: number;
  processTypes: ProcessTypeConfig[];
  
  // Resource Allocation Configuration
  resourceAllocation: ResourceAllocationConfig;
  
  // Security Context Configuration
  securityLevel: 'basic' | 'standard' | 'enhanced' | 'maximum';
  enableKernelSecurity: boolean;
  securityContext: SecurityContextConfig;
  
  // Service Mapper Configuration
  enableServiceMapper: boolean;
  autoScaling: boolean;
  scalingPolicy: ScalingPolicyConfig;
  
  // Advanced OS Features
  enableZkProofs: boolean;
  enableEconomicCoordination: boolean;
  consensusType: '6d-quantum' | 'lccd' | 'hybrid';
  
  // Connection Settings
  maxRetries: number;
  timeoutMs: number;
  heartbeatInterval: number;
  
  // Kernel Communication
  communicationChannel: 'ipc' | 'shared_memory' | 'socket';
  customEndpoints?: {
    kernelBridge?: string;
    serviceMapper?: string;
    resourceCoordinator?: string;
  };
}

// BPI OS Kernel connection status
export interface BpiOSKernelConnectionStatus {
  status: 'disconnected' | 'connecting' | 'connected' | 'authenticated' | 'active' | 'error';
  message: string;
  connectedAt?: Date;
  lastHeartbeat?: Date;
  kernelInfo?: {
    kernelId: string;
    version: string;
    osVersion: string;
    processCount: number;
    resourceUtilization: number;
  };
  performanceMetrics?: {
    kernelLatency: number;
    processThroughput: number;
    resourceEfficiency: number;
    errorRate: number;
  };
}

interface BpiOSKernelConnectionFormProps {
  onConnectionChange?: (status: BpiOSKernelConnectionStatus) => void;
  onConfigChange?: (config: BpiOSKernelConnectionConfig) => void;
  initialConfig?: Partial<BpiOSKernelConnectionConfig>;
}

export const BpiOSKernelConnectionForm: React.FC<BpiOSKernelConnectionFormProps> = ({
  onConnectionChange,
  onConfigChange,
  initialConfig,
}) => {
  const [form] = Form.useForm();
  const { hasPermission } = useRBAC();
  
  // State management
  const [currentStep, setCurrentStep] = useState(0);
  const [loading, setLoading] = useState(false);
  const [connectionStatus, setConnectionStatus] = useState<BpiOSKernelConnectionStatus>({
    status: 'disconnected',
    message: 'Not connected to BPI OS Kernel',
  });
  const [validationErrors, setValidationErrors] = useState<Record<string, string>>({});
  const [testResults, setTestResults] = useState<Record<string, boolean>>({});

  // Default configuration
  const defaultConfig: BpiOSKernelConnectionConfig = {
    kernelAddress: 'bpi-kernel://localhost:7777',
    kernelToken: '',
    bridgeId: `bridge-${Date.now()}`,
    networkType: 'development',
    osVersion: 'auto-detect',
    kernelVersion: 'auto-detect',
    enableProcessMapping: true,
    maxProcesses: 100,
    processTypes: [
      { type: 'governance', enabled: true, priority: 'high', instances: 2 },
      { type: 'orchestration', enabled: true, priority: 'high', instances: 3 },
      { type: 'api', enabled: true, priority: 'normal', instances: 5 },
      { type: 'background', enabled: true, priority: 'low', instances: 10 },
      { type: 'security', enabled: true, priority: 'critical', instances: 2 },
      { type: 'audit', enabled: true, priority: 'high', instances: 1 },
    ],
    resourceAllocation: {
      cpuCores: 4,
      memoryGB: 8,
      networkBandwidthMbps: 1000,
      storageGB: 100,
      gpuUnits: 0,
      priority: 'normal',
    },
    securityLevel: 'standard',
    enableKernelSecurity: true,
    securityContext: {
      permissions: ['read', 'write', 'execute'],
      accessLevel: 'admin',
      encryptionEnabled: true,
      auditEnabled: true,
    },
    enableServiceMapper: true,
    autoScaling: true,
    scalingPolicy: {
      minInstances: 1,
      maxInstances: 10,
      cpuThreshold: 80,
      memoryThreshold: 85,
      autoScaling: true,
    },
    enableZkProofs: true,
    enableEconomicCoordination: true,
    consensusType: '6d-quantum',
    maxRetries: 3,
    timeoutMs: 30000,
    heartbeatInterval: 5000,
    communicationChannel: 'ipc',
  };

  // Initialize form with default or initial config
  useEffect(() => {
    const config = { ...defaultConfig, ...initialConfig };
    form.setFieldsValue(config);
    onConfigChange?.(config);
  }, [initialConfig]);

  // Validate kernel address
  const validateKernelAddress = async (address: string): Promise<boolean> => {
    try {
      if (!address.startsWith('bpi-kernel://')) {
        setValidationErrors(prev => ({ ...prev, kernelAddress: 'Address must start with bpi-kernel://' }));
        return false;
      }

      // Test kernel bridge connection
      const response = await fetch(`${process.env.REACT_APP_API_URL || 'https://api.pravyom.com'}/api/bpi-os/validate-kernel`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ address }),
      });

      if (response.ok) {
        setValidationErrors(prev => ({ ...prev, kernelAddress: '' }));
        return true;
      } else {
        setValidationErrors(prev => ({ ...prev, kernelAddress: 'Kernel address is not reachable' }));
        return false;
      }
    } catch (error) {
      setValidationErrors(prev => ({ ...prev, kernelAddress: 'Failed to validate kernel address' }));
      return false;
    }
  };

  // Test kernel bridge connection
  const testKernelBridge = async (config: BpiOSKernelConnectionConfig): Promise<boolean> => {
    try {
      const response = await fetch(`${process.env.REACT_APP_API_URL || 'https://api.pravyom.com'}/api/bpi-os/test-kernel-bridge`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(config),
      });

      const result = await response.json();
      setTestResults(prev => ({ ...prev, kernelBridge: result.success }));
      return result.success;
    } catch (error) {
      setTestResults(prev => ({ ...prev, kernelBridge: false }));
      return false;
    }
  };

  // Test service mapper
  const testServiceMapper = async (config: BpiOSKernelConnectionConfig): Promise<boolean> => {
    try {
      const response = await fetch(`${process.env.REACT_APP_API_URL || 'https://api.pravyom.com'}/api/bpi-os/test-service-mapper`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(config),
      });

      const result = await response.json();
      setTestResults(prev => ({ ...prev, serviceMapper: result.success }));
      return result.success;
    } catch (error) {
      setTestResults(prev => ({ ...prev, serviceMapper: false }));
      return false;
    }
  };

  // Test resource coordinator
  const testResourceCoordinator = async (config: BpiOSKernelConnectionConfig): Promise<boolean> => {
    try {
      const response = await fetch(`${process.env.REACT_APP_API_URL || 'https://api.pravyom.com'}/api/bpi-os/test-resource-coordinator`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(config),
      });

      const result = await response.json();
      setTestResults(prev => ({ ...prev, resourceCoordinator: result.success }));
      return result.success;
    } catch (error) {
      setTestResults(prev => ({ ...prev, resourceCoordinator: false }));
      return false;
    }
  };

  // Connect to BPI OS Kernel
  const connectToKernel = async (config: BpiOSKernelConnectionConfig) => {
    setLoading(true);
    setConnectionStatus({ status: 'connecting', message: 'Connecting to BPI OS Kernel...' });

    try {
      // Step 1: Test kernel bridge
      const kernelBridgeOk = await testKernelBridge(config);
      if (!kernelBridgeOk) {
        throw new Error('Kernel bridge connection failed');
      }

      // Step 2: Test service mapper
      const serviceMapperOk = await testServiceMapper(config);
      if (!serviceMapperOk) {
        throw new Error('Service mapper connection failed');
      }

      // Step 3: Test resource coordinator
      const resourceCoordinatorOk = await testResourceCoordinator(config);
      if (!resourceCoordinatorOk) {
        throw new Error('Resource coordinator connection failed');
      }

      // Step 4: Establish full connection
      const response = await fetch(`${process.env.REACT_APP_API_URL || 'https://api.pravyom.com'}/api/bpi-os/connect-kernel`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(config),
      });

      if (response.ok) {
        const result = await response.json();
        const newStatus: BpiOSKernelConnectionStatus = {
          status: 'active',
          message: 'Successfully connected to BPI OS Kernel',
          connectedAt: new Date(),
          lastHeartbeat: new Date(),
          kernelInfo: result.kernelInfo,
          performanceMetrics: result.performanceMetrics,
        };
        setConnectionStatus(newStatus);
        onConnectionChange?.(newStatus);
      } else {
        throw new Error('Failed to establish kernel connection');
      }
    } catch (error) {
      const errorStatus: BpiOSKernelConnectionStatus = {
        status: 'error',
        message: error instanceof Error ? error.message : 'Unknown connection error',
      };
      setConnectionStatus(errorStatus);
      onConnectionChange?.(errorStatus);
    } finally {
      setLoading(false);
    }
  };

  // Handle form submission
  const handleSubmit = async (values: BpiOSKernelConnectionConfig) => {
    if (!hasPermission('bpi:connect')) {
      return;
    }

    await connectToKernel(values);
    onConfigChange?.(values);
  };

  // Handle form value changes
  const handleValuesChange = (changedValues: any, allValues: BpiOSKernelConnectionConfig) => {
    onConfigChange?.(allValues);
  };

  // Steps configuration
  const steps = [
    {
      title: 'Kernel Configuration',
      description: 'Configure BPI OS Kernel connection',
      icon: <DatabaseOutlined />,
    },
    {
      title: 'Process Mapping',
      description: 'Configure process mapping and resources',
      icon: <DesktopOutlined />,
    },
    {
      title: 'Security & Services',
      description: 'Configure security and service mapping',
      icon: <SecurityScanOutlined />,
    },
    {
      title: 'Connection Test',
      description: 'Test and establish connection',
      icon: <LinkOutlined />,
    },
  ];

  return (
    <div>
      {/* Header */}
      <Card>
        <Title level={4}>
          <DatabaseOutlined /> BPI OS Kernel Connection
        </Title>
        <Text type="secondary">
          Connect to BPI Blockchain Operating System Kernel with advanced OS-level configuration
        </Text>
        
        {/* Connection Status */}
        <div style={{ marginTop: 16 }}>
          <Badge
            status={
              connectionStatus.status === 'active' ? 'success' :
              connectionStatus.status === 'connecting' ? 'processing' :
              connectionStatus.status === 'error' ? 'error' : 'default'
            }
            text={connectionStatus.message}
          />
          {connectionStatus.kernelInfo && (
            <div style={{ marginTop: 8 }}>
              <Space>
                <Tag>Kernel: {connectionStatus.kernelInfo.kernelId}</Tag>
                <Tag>Version: {connectionStatus.kernelInfo.version}</Tag>
                <Tag>Processes: {connectionStatus.kernelInfo.processCount}</Tag>
                <Tag>Utilization: {connectionStatus.kernelInfo.resourceUtilization}%</Tag>
              </Space>
            </div>
          )}
        </div>
      </Card>

      {/* Steps */}
      <Card style={{ marginTop: 16 }}>
        <Steps current={currentStep} items={steps} />
      </Card>

      {/* Form */}
      <Form
        form={form}
        layout="vertical"
        onFinish={handleSubmit}
        onValuesChange={handleValuesChange}
        initialValues={defaultConfig}
      >
        {/* Step 1: Kernel Configuration */}
        {currentStep === 0 && (
          <Card title="Kernel Configuration" style={{ marginTop: 16 }}>
            <Row gutter={16}>
              <Col span={12}>
                <Form.Item
                  name="kernelAddress"
                  label="Kernel Address"
                  rules={[{ required: true, message: 'Please enter kernel address' }]}
                  validateStatus={validationErrors.kernelAddress ? 'error' : ''}
                  help={validationErrors.kernelAddress}
                >
                  <Input
                    prefix={<DatabaseOutlined />}
                    placeholder="bpi-kernel://localhost:7777"
                    onBlur={(e) => validateKernelAddress(e.target.value)}
                  />
                </Form.Item>
              </Col>
              <Col span={12}>
                <Form.Item
                  name="kernelToken"
                  label="Kernel Token"
                  rules={[{ required: true, message: 'Please enter kernel token' }]}
                >
                  <Input.Password
                    prefix={<SecurityScanOutlined />}
                    placeholder="Enter kernel authentication token"
                  />
                </Form.Item>
              </Col>
              <Col span={8}>
                <Form.Item name="networkType" label="Network Type">
                  <Select>
                    <Option value="mainnet">Mainnet</Option>
                    <Option value="testnet">Testnet</Option>
                    <Option value="development">Development</Option>
                  </Select>
                </Form.Item>
              </Col>
              <Col span={8}>
                <Form.Item name="communicationChannel" label="Communication Channel">
                  <Select>
                    <Option value="ipc">IPC (Inter-Process Communication)</Option>
                    <Option value="shared_memory">Shared Memory</Option>
                    <Option value="socket">Socket</Option>
                  </Select>
                </Form.Item>
              </Col>
              <Col span={8}>
                <Form.Item name="consensusType" label="Consensus Type">
                  <Select>
                    <Option value="6d-quantum">6D Quantum</Option>
                    <Option value="lccd">LCCD</Option>
                    <Option value="hybrid">Hybrid</Option>
                  </Select>
                </Form.Item>
              </Col>
            </Row>
          </Card>
        )}

        {/* Step 2: Process Mapping */}
        {currentStep === 1 && (
          <Card title="Process Mapping & Resources" style={{ marginTop: 16 }}>
            <Row gutter={16}>
              <Col span={24}>
                <Form.Item name="enableProcessMapping" valuePropName="checked">
                  <Switch checkedChildren="Process Mapping Enabled" unCheckedChildren="Process Mapping Disabled" />
                </Form.Item>
              </Col>
              <Col span={12}>
                <Form.Item name="maxProcesses" label="Max Processes">
                  <InputNumber min={1} max={1000} style={{ width: '100%' }} />
                </Form.Item>
              </Col>
              <Col span={12}>
                <Form.Item name={['resourceAllocation', 'cpuCores']} label="CPU Cores">
                  <Slider min={1} max={32} marks={{ 1: '1', 8: '8', 16: '16', 32: '32' }} />
                </Form.Item>
              </Col>
              <Col span={12}>
                <Form.Item name={['resourceAllocation', 'memoryGB']} label="Memory (GB)">
                  <Slider min={1} max={128} marks={{ 1: '1GB', 32: '32GB', 64: '64GB', 128: '128GB' }} />
                </Form.Item>
              </Col>
              <Col span={12}>
                <Form.Item name={['resourceAllocation', 'storageGB']} label="Storage (GB)">
                  <Slider min={10} max={1000} marks={{ 10: '10GB', 100: '100GB', 500: '500GB', 1000: '1TB' }} />
                </Form.Item>
              </Col>
            </Row>
          </Card>
        )}

        {/* Step 3: Security & Services */}
        {currentStep === 2 && (
          <Card title="Security & Service Configuration" style={{ marginTop: 16 }}>
            <Row gutter={16}>
              <Col span={12}>
                <Form.Item name="securityLevel" label="Security Level">
                  <Select>
                    <Option value="basic">Basic</Option>
                    <Option value="standard">Standard</Option>
                    <Option value="enhanced">Enhanced</Option>
                    <Option value="maximum">Maximum</Option>
                  </Select>
                </Form.Item>
              </Col>
              <Col span={12}>
                <Form.Item name={['securityContext', 'accessLevel']} label="Access Level">
                  <Select>
                    <Option value="read">Read Only</Option>
                    <Option value="write">Read/Write</Option>
                    <Option value="admin">Admin</Option>
                    <Option value="kernel">Kernel Level</Option>
                  </Select>
                </Form.Item>
              </Col>
              <Col span={8}>
                <Form.Item name="enableKernelSecurity" valuePropName="checked">
                  <Switch checkedChildren="Kernel Security" unCheckedChildren="Basic Security" />
                </Form.Item>
              </Col>
              <Col span={8}>
                <Form.Item name="enableServiceMapper" valuePropName="checked">
                  <Switch checkedChildren="Service Mapper" unCheckedChildren="Manual Mapping" />
                </Form.Item>
              </Col>
              <Col span={8}>
                <Form.Item name="autoScaling" valuePropName="checked">
                  <Switch checkedChildren="Auto Scaling" unCheckedChildren="Fixed Scale" />
                </Form.Item>
              </Col>
            </Row>
          </Card>
        )}

        {/* Step 4: Connection Test */}
        {currentStep === 3 && (
          <Card title="Connection Test & Validation" style={{ marginTop: 16 }}>
            <Space direction="vertical" style={{ width: '100%' }}>
              <Alert
                message="BPI OS Kernel Connection Test"
                description="Testing kernel bridge, service mapper, and resource coordinator connections"
                type="info"
                showIcon
              />
              
              <Row gutter={16}>
                <Col span={8}>
                  <Card size="small">
                    <Space>
                      {testResults.kernelBridge === undefined ? (
                        <LoadingOutlined />
                      ) : testResults.kernelBridge ? (
                        <CheckCircleOutlined style={{ color: '#52c41a' }} />
                      ) : (
                        <ExclamationCircleOutlined style={{ color: '#ff4d4f' }} />
                      )}
                      <Text>Kernel Bridge</Text>
                    </Space>
                  </Card>
                </Col>
                <Col span={8}>
                  <Card size="small">
                    <Space>
                      {testResults.serviceMapper === undefined ? (
                        <LoadingOutlined />
                      ) : testResults.serviceMapper ? (
                        <CheckCircleOutlined style={{ color: '#52c41a' }} />
                      ) : (
                        <ExclamationCircleOutlined style={{ color: '#ff4d4f' }} />
                      )}
                      <Text>Service Mapper</Text>
                    </Space>
                  </Card>
                </Col>
                <Col span={8}>
                  <Card size="small">
                    <Space>
                      {testResults.resourceCoordinator === undefined ? (
                        <LoadingOutlined />
                      ) : testResults.resourceCoordinator ? (
                        <CheckCircleOutlined style={{ color: '#52c41a' }} />
                      ) : (
                        <ExclamationCircleOutlined style={{ color: '#ff4d4f' }} />
                      )}
                      <Text>Resource Coordinator</Text>
                    </Space>
                  </Card>
                </Col>
              </Row>
            </Space>
          </Card>
        )}

        {/* Navigation */}
        <Card style={{ marginTop: 16 }}>
          <Space>
            {currentStep > 0 && (
              <Button onClick={() => setCurrentStep(currentStep - 1)}>
                Previous
              </Button>
            )}
            {currentStep < steps.length - 1 && (
              <Button type="primary" onClick={() => setCurrentStep(currentStep + 1)}>
                Next
              </Button>
            )}
            {currentStep === steps.length - 1 && (
              <Button
                type="primary"
                htmlType="submit"
                loading={loading}
                disabled={!hasPermission('bpi:connect')}
              >
                Connect to BPI OS Kernel
              </Button>
            )}
          </Space>
        </Card>
      </Form>
    </div>
  );
};

export default BpiOSKernelConnectionForm;
