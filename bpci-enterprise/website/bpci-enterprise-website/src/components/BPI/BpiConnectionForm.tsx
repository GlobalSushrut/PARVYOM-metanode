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
  HddOutlined,
  LoadingOutlined,
  InfoCircleOutlined,
  CloudOutlined,
  ApiOutlined,
} from '@ant-design/icons';

const { Title, Text } = Typography;
const { Option } = Select;
const { Panel } = Collapse;
const { Step } = Steps;

// Supporting type definitions
interface ProcessTypeConfig {
  type: string;
  priority: number;
  resources: {
    cpu: number;
    memory: number;
  };
}

interface ResourceAllocationConfig {
  cpu: {
    cores: number;
    frequency: number;
  };
  memory: {
    total: number;
    available: number;
  };
  storage: {
    total: number;
    available: number;
  };
}

interface SecurityContextConfig {
  level: 'basic' | 'standard' | 'enhanced' | 'maximum';
  encryption: boolean;
  authentication: boolean;
  authorization: boolean;
}

interface ScalingPolicyConfig {
  minInstances: number;
  maxInstances: number;
  targetCpuUtilization: number;
  scaleUpCooldown: number;
  scaleDownCooldown: number;
}

// BPI connection configuration type alias
type BpiConnectionConfig = BpiOSKernelConnectionConfig;

// Mock RBAC hook
const useRBAC = () => ({
  hasPermission: (permission: string) => true,
  userRole: 'admin' as const,
  hasBpiAccess: true
});

// Mock validation functions
const validateBpiEndpoint = (endpoint: string) => ({
  isValid: endpoint.startsWith('http') && endpoint.includes('bpi'),
  error: endpoint.startsWith('http') && endpoint.includes('bpi') ? null : 'Invalid BPI endpoint'
});

const validateBpiAddress = (address: string) => ({
  isValid: address.length > 10 && /^[a-zA-Z0-9]+$/.test(address),
  error: address.length > 10 && /^[a-zA-Z0-9]+$/.test(address) ? null : 'Invalid BPI address'
});

// Mock endpoint reachability check
const checkEndpointReachability = (endpoint: string) => ({
  isReachable: true,
  latency: Math.random() * 100,
  error: null
});

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
  healthCheckInterval: number;
}

export interface BpiConnectionStatus {
  status: 'disconnected' | 'connecting' | 'connected' | 'error';
  message: string;
  connectedAt?: Date;
  lastHealthCheck?: Date;
  nodeInfo?: {
    nodeId: string;
    version: string;
    chainHeight: number;
    peerCount: number;
  };
  performanceMetrics?: {
    latency: number;
    throughput: number;
    errorRate: number;
  };
}

interface BpiConnectionFormProps {
  onConnectionChange?: (status: BpiConnectionStatus) => void;
  onConfigChange?: (config: BpiConnectionConfig) => void;
  initialConfig?: Partial<BpiConnectionConfig>;
}

export const BpiConnectionForm: React.FC<BpiConnectionFormProps> = ({
  onConnectionChange,
  onConfigChange,
  initialConfig,
}) => {
  const { hasBpiAccess, hasPermission } = useRBAC();
  const [form] = Form.useForm();
  
  // State management
  const [currentStep, setCurrentStep] = useState(0);
  const [connectionStatus, setConnectionStatus] = useState<BpiConnectionStatus>({
    status: 'disconnected',
    message: 'Not connected to BPI infrastructure',
  });
  const [config, setConfig] = useState<BpiConnectionConfig>({
    kernelAddress: 'http://localhost:7777',
    kernelToken: '',
    bridgeId: 'bpi-bridge-001',
    networkType: 'testnet',
    osVersion: '1.0.0',
    kernelVersion: '1.0.0',
    enableProcessMapping: true,
    maxProcesses: 100,
    processTypes: [],
    resourceAllocation: {
      cpu: { cores: 4, frequency: 2400 },
      memory: { total: 8192, available: 6144 },
      storage: { total: 100000, available: 80000 }
    },
    securityLevel: 'standard',
    enableKernelSecurity: true,
    securityContext: {
      level: 'standard',
      encryption: true,
      authentication: true,
      authorization: true
    },
    enableServiceMapper: true,
    autoScaling: true,
    scalingPolicy: {
      minInstances: 1,
      maxInstances: 10,
      targetCpuUtilization: 70,
      scaleUpCooldown: 300,
      scaleDownCooldown: 600
    },
    enableZkProofs: true,
    enableEconomicCoordination: true,
    consensusType: 'lccd',
    maxRetries: 3,
    timeoutMs: 30000,
    heartbeatInterval: 30000,
    communicationChannel: 'socket',
    healthCheckInterval: 30000,
    ...initialConfig,
  });
  const [loading, setLoading] = useState(false);
  const [validationErrors, setValidationErrors] = useState<Record<string, string>>({});
  const [connectionProgress, setConnectionProgress] = useState(0);
  const [advancedMode, setAdvancedMode] = useState(false);

  // Check BPI access permission
  if (!hasBpiAccess) {
    return (
      <Card>
        <Alert
          message="BPI Access Required"
          description="You need BPI infrastructure access permissions to use this feature. Please contact your administrator."
          type="warning"
          showIcon
          icon={<SecurityScanOutlined />}
        />
      </Card>
    );
  }

  // Connection steps
  const connectionSteps = [
    {
      title: 'Configuration',
      description: 'Enter BPI connection details',
      icon: <SettingOutlined />,
    },
    {
      title: 'Validation',
      description: 'Validate endpoints and credentials',
      icon: <SecurityScanOutlined />,
    },
    {
      title: 'Connection',
      description: 'Establish BPI connection',
      icon: <LinkOutlined />,
    },
    {
      title: 'Verification',
      description: 'Verify connection health',
      icon: <CheckCircleOutlined />,
    },
  ];

  // Handle form submission
  const handleConnect = async (values: any) => {
    setLoading(true);
    setConnectionProgress(0);
    setCurrentStep(1);

    try {
      // Step 1: Validate configuration
      setConnectionProgress(25);
      await validateConfiguration(values);
      
      // Step 2: Establish connection
      setCurrentStep(2);
      setConnectionProgress(50);
      await establishConnection(values);
      
      // Step 3: Verify connection
      setCurrentStep(3);
      setConnectionProgress(75);
      await verifyConnection();
      
      // Step 4: Complete
      setConnectionProgress(100);
      setConnectionStatus({
        status: 'connected',
        message: 'Successfully connected to BPI infrastructure',
        connectedAt: new Date(),
      });
      
      // Notify parent component
      if (onConnectionChange) {
        onConnectionChange(connectionStatus);
      }
      
    } catch (error) {
      setConnectionStatus({
        status: 'error',
        message: error instanceof Error ? error.message : 'Connection failed',
      });
      setCurrentStep(0);
    } finally {
      setLoading(false);
    }
  };

  // Validate configuration
  const validateConfiguration = async (values: any) => {
    const errors: Record<string, string> = {};

    // Validate primary endpoint
    if (values.primaryEndpoint) {
      const endpointValidation = validateBpiEndpoint(values.primaryEndpoint);
      if (!endpointValidation.isValid) {
        errors.primaryEndpoint = endpointValidation.error || 'Invalid endpoint';
      }
    }

    // Validate BPI node address
    if (values.nodeAddress) {
      const addressValidation = validateBpiAddress(values.nodeAddress);
      if (!addressValidation.isValid) {
        errors.nodeAddress = addressValidation.error || 'Invalid BPI address';
      }
    }

    // Validate auth token
    if (!values.authToken || values.authToken.length < 32) {
      errors.authToken = 'Authentication token must be at least 32 characters';
    }

    if (Object.keys(errors).length > 0) {
      setValidationErrors(errors);
      throw new Error('Configuration validation failed');
    }

    setValidationErrors({});
  };

  // Establish connection to BPI
  const establishConnection = async (values: any) => {
    const newConfig: BpiConnectionConfig = {
      kernelAddress: values.primaryEndpoint || 'http://localhost:7777',
      kernelToken: values.authToken || '',
      bridgeId: values.bridgeId || 'bpi-bridge-001',
      networkType: values.connectionType || 'testnet',
      osVersion: '1.0.0',
      kernelVersion: '1.0.0',
      enableProcessMapping: true,
      maxProcesses: 100,
      processTypes: [],
      resourceAllocation: {
        cpu: { cores: 4, frequency: 2400 },
        memory: { total: 8192, available: 6144 },
        storage: { total: 100000, available: 80000 }
      },
      securityLevel: 'standard',
      enableKernelSecurity: true,
      securityContext: {
        level: 'standard',
        encryption: true,
        authentication: true,
        authorization: true
      },
      enableServiceMapper: true,
      autoScaling: true,
      scalingPolicy: {
        minInstances: 1,
        maxInstances: 10,
        targetCpuUtilization: 70,
        scaleUpCooldown: 300,
        scaleDownCooldown: 600
      },
      enableZkProofs: values.zkProofEnabled ?? true,
      enableEconomicCoordination: values.economicCoordination ?? true,
      consensusType: values.consensusType || 'lccd',
      maxRetries: 3,
      timeoutMs: 30000,
      heartbeatInterval: values.heartbeatInterval || 30000,
      communicationChannel: 'socket',
      healthCheckInterval: values.healthCheckInterval || 30000,
    };

    // Call backend API to establish connection
    const response = await fetch(`${process.env.REACT_APP_API_URL || 'https://api.pravyom.com'}/api/bpi/connect`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${values.authToken}`,
      },
      body: JSON.stringify(newConfig),
    });

    if (!response.ok) {
      const error = await response.json();
      throw new Error(error.message || 'Failed to establish BPI connection');
    }

    setConfig(newConfig);
    if (onConfigChange) {
      onConfigChange(newConfig);
    }
  };

  // Verify connection health
  const verifyConnection = async () => {
    const response = await fetch(`${process.env.REACT_APP_API_URL || 'https://api.pravyom.com'}/api/bpi/status`);
    
    if (!response.ok) {
      throw new Error('Failed to verify BPI connection');
    }

    const status = await response.json();
    
    if (status.status !== 'connected') {
      throw new Error('BPI connection verification failed');
    }

    // Update connection status with node info
    setConnectionStatus(prev => ({
      ...prev,
      nodeInfo: status.nodeInfo,
      performanceMetrics: status.performanceMetrics,
      lastHealthCheck: new Date(),
    }));
  };

  // Handle disconnect
  const handleDisconnect = async () => {
    setLoading(true);
    
    try {
      await fetch(`${process.env.REACT_APP_API_URL || 'https://api.pravyom.com'}/api/bpi/disconnect`, { method: 'POST' });
      
      setConnectionStatus({
        status: 'disconnected',
        message: 'Disconnected from BPI infrastructure',
      });
      setCurrentStep(0);
      setConnectionProgress(0);
      
      if (onConnectionChange) {
        onConnectionChange(connectionStatus);
      }
    } catch (error) {
      console.error('Disconnect failed:', error);
    } finally {
      setLoading(false);
    }
  };

  // Test connection
  const testConnection = async () => {
    const values = form.getFieldsValue();
    
    try {
      setLoading(true);
      await validateConfiguration(values);
      
      // Test endpoint reachability
      const endpointResult = checkEndpointReachability(values.primaryEndpoint);
      
      if (endpointResult.isReachable) {
        setConnectionStatus({
          status: 'connected',
          message: 'Test connection successful',
        });
      } else {
        setConnectionStatus({
          status: 'error',
          message: 'Endpoint is not reachable',
        });
      }
    } catch (error) {
      setConnectionStatus({
        status: 'error',
        message: error instanceof Error ? error.message : 'Test failed',
      });
    } finally {
      setLoading(false);
    }
  };

  return (
    <Card
      title={
        <Space>
          <LinkOutlined />
          <span>BPI Infrastructure Connection</span>
        </Space>
      }
      extra={
        <Space>
          <Switch
            checkedChildren="Advanced"
            unCheckedChildren="Simple"
            checked={advancedMode}
            onChange={setAdvancedMode}
          />
          {connectionStatus.status === 'connected' && (
            <Button
              danger
              onClick={handleDisconnect}
              loading={loading}
            >
              Disconnect
            </Button>
          )}
        </Space>
      }
    >
      {/* Connection Status */}
      <Alert
        message={`Connection Status: ${connectionStatus.status.toUpperCase()}`}
        description={connectionStatus.message}
        type={
          connectionStatus.status === 'connected' ? 'success' :
          connectionStatus.status === 'error' ? 'error' : 'info'
        }
        showIcon
        style={{ marginBottom: 24 }}
      />

      {/* Connection Progress */}
      {loading && (
        <div style={{ marginBottom: 24 }}>
          <Steps current={currentStep} size="small">
            {connectionSteps.map((step, index) => (
              <Step
                key={index}
                title={step.title}
                description={step.description}
                icon={loading && index === currentStep ? <LoadingOutlined /> : step.icon}
              />
            ))}
          </Steps>
          <Progress percent={connectionProgress} style={{ marginTop: 16 }} />
        </div>
      )}

      {/* Connection Form */}
      <Form
        form={form}
        layout="vertical"
        onFinish={handleConnect}
        initialValues={config}
      >
        <Row gutter={16}>
          <Col span={12}>
            <Form.Item
              label={
                <Space>
                  <span>Primary BPI Endpoint</span>
                  <Tooltip title="Main BPI node endpoint URL (e.g., https://node1.bpi.network:8545)">
                    <InfoCircleOutlined />
                  </Tooltip>
                </Space>
              }
              name="primaryEndpoint"
              rules={[
                { required: true, message: 'Primary endpoint is required' },
                { type: 'url', message: 'Please enter a valid URL' },
              ]}
              validateStatus={validationErrors.primaryEndpoint ? 'error' : ''}
              help={validationErrors.primaryEndpoint}
            >
              <Input
                placeholder="https://node1.bpi.network:8545"
                prefix={<LinkOutlined />}
              />
            </Form.Item>
          </Col>
          
          <Col span={12}>
            <Form.Item
              label="Secondary BPI Endpoint (Optional)"
              name="secondaryEndpoint"
            >
              <Input
                placeholder="https://node2.bpi.network:8545"
                prefix={<LinkOutlined />}
              />
            </Form.Item>
          </Col>
        </Row>

        <Row gutter={16}>
          <Col span={12}>
            <Form.Item
              label={
                <Space>
                  <span>BPI Node Address</span>
                  <Tooltip title="Your BPI node address (e.g., bpi:node:1234567890abcdef...)">
                    <InfoCircleOutlined />
                  </Tooltip>
                </Space>
              }
              name="nodeAddress"
              rules={[
                { required: true, message: 'BPI node address is required' },
              ]}
              validateStatus={validationErrors.nodeAddress ? 'error' : ''}
              help={validationErrors.nodeAddress}
            >
              <Input
                placeholder="bpi:node:1234567890abcdef..."
                prefix={<SecurityScanOutlined />}
              />
            </Form.Item>
          </Col>
          
          <Col span={12}>
            <Form.Item
              label="Connection Type"
              name="connectionType"
              rules={[{ required: true }]}
            >
              <Select>
                <Option value="Mainnet">Mainnet</Option>
                <Option value="Testnet">Testnet</Option>
                <Option value="Development">Development</Option>
              </Select>
            </Form.Item>
          </Col>
        </Row>

        <Form.Item
          label={
            <Space>
              <span>Authentication Token</span>
              <Tooltip title="Your BPI authentication token for secure access">
                <InfoCircleOutlined />
              </Tooltip>
            </Space>
          }
          name="authToken"
          rules={[
            { required: true, message: 'Authentication token is required' },
            { min: 32, message: 'Token must be at least 32 characters' },
          ]}
          validateStatus={validationErrors.authToken ? 'error' : ''}
          help={validationErrors.authToken}
        >
          <Input.Password
            placeholder="Enter your BPI authentication token"
            prefix={<SecurityScanOutlined />}
          />
        </Form.Item>

        {/* Advanced Configuration */}
        {advancedMode && (
          <>
            <Divider>Advanced Configuration</Divider>
            
            <Row gutter={16}>
              <Col span={8}>
                <Form.Item
                  label="Chain ID"
                  name="chainId"
                  tooltip="BPI chain identifier"
                >
                  <Input type="number" placeholder="1337" />
                </Form.Item>
              </Col>
              
              <Col span={8}>
                <Form.Item
                  label="Consensus Type"
                  name="consensusType"
                >
                  <Select>
                    <Option value="LCCD">LCCD</Option>
                    <Option value="QGC-C²">QGC-C²</Option>
                    <Option value="6D">6D Quantum</Option>
                  </Select>
                </Form.Item>
              </Col>
              
              <Col span={8}>
                <Form.Item
                  label="Health Check Interval (ms)"
                  name="healthCheckInterval"
                >
                  <Input type="number" placeholder="30000" />
                </Form.Item>
              </Col>
            </Row>

            <Row gutter={16}>
              <Col span={8}>
                <Form.Item
                  name="zkProofEnabled"
                  valuePropName="checked"
                >
                  <Space>
                    <Switch />
                    <span>Enable ZK Proofs</span>
                  </Space>
                </Form.Item>
              </Col>
              
              <Col span={8}>
                <Form.Item
                  name="economicCoordination"
                  valuePropName="checked"
                >
                  <Space>
                    <Switch />
                    <span>Economic Coordination</span>
                  </Space>
                </Form.Item>
              </Col>
              
              <Col span={8}>
                <Form.Item
                  name="autoReconnect"
                  valuePropName="checked"
                >
                  <Space>
                    <Switch />
                    <span>Auto Reconnect</span>
                  </Space>
                </Form.Item>
              </Col>
            </Row>
          </>
        )}

        {/* Action Buttons */}
        <Form.Item>
          <Space>
            <Button
              type="primary"
              htmlType="submit"
              loading={loading}
              disabled={connectionStatus.status === 'connected'}
              icon={<LinkOutlined />}
            >
              {connectionStatus.status === 'connected' ? 'Connected' : 'Connect to BPI'}
            </Button>
            
            <Button
              onClick={testConnection}
              loading={loading}
              icon={<SecurityScanOutlined />}
            >
              Test Connection
            </Button>
            
            {hasPermission('system:admin') && (
              <Button
                onClick={() => form.resetFields()}
                disabled={loading}
              >
                Reset
              </Button>
            )}
          </Space>
        </Form.Item>
      </Form>

      {/* Connection Info */}
      {connectionStatus.status === 'connected' && connectionStatus.nodeInfo && (
        <Card size="small" title="Connected Node Information" style={{ marginTop: 16 }}>
          <Row gutter={16}>
            <Col span={6}>
              <Text strong>Node ID:</Text>
              <br />
              <Text code>{connectionStatus.nodeInfo.nodeId}</Text>
            </Col>
            <Col span={6}>
              <Text strong>Version:</Text>
              <br />
              <Text>{connectionStatus.nodeInfo.version}</Text>
            </Col>
            <Col span={6}>
              <Text strong>Chain Height:</Text>
              <br />
              <Text>{connectionStatus.nodeInfo.chainHeight.toLocaleString()}</Text>
            </Col>
            <Col span={6}>
              <Text strong>Peers:</Text>
              <br />
              <Text>{connectionStatus.nodeInfo.peerCount}</Text>
            </Col>
          </Row>
          
          {connectionStatus.performanceMetrics && (
            <div style={{ marginTop: 16 }}>
              <Text strong>Performance Metrics:</Text>
              <Row gutter={16} style={{ marginTop: 8 }}>
                <Col span={8}>
                  <Text>Latency: {connectionStatus.performanceMetrics.latency}ms</Text>
                </Col>
                <Col span={8}>
                  <Text>Throughput: {connectionStatus.performanceMetrics.throughput} TPS</Text>
                </Col>
                <Col span={8}>
                  <Text>Error Rate: {(connectionStatus.performanceMetrics.errorRate * 100).toFixed(2)}%</Text>
                </Col>
              </Row>
            </div>
          )}
        </Card>
      )}
    </Card>
  );
};

export default BpiConnectionForm;
