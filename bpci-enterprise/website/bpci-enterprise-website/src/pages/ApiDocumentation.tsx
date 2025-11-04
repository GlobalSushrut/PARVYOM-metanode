import React, { useState } from 'react';
import { Card, Typography, Space, Tabs, Input, Button, Tag, Collapse, Alert } from 'antd';
import { ApiOutlined, CodeOutlined, PlayCircleOutlined, CopyOutlined } from '@ant-design/icons';

const { Title, Text, Paragraph } = Typography;
const { TabPane } = Tabs;
const { Panel } = Collapse;
const { TextArea } = Input;

const ApiDocumentation: React.FC = () => {
  const [apiKey] = useState('your_api_key_here');

  const endpoints = [
    {
      method: 'GET',
      path: '/api/wallet/status',
      description: 'Get wallet status',
      params: [{ name: 'bpi_address', type: 'string', required: true }],
      response: `{
  "success": true,
  "data": {
    "status": "active",
    "network": "mainnet"
  }
}`
    },
    {
      method: 'GET',
      path: '/api/wallet/balance',
      description: 'Get 4-coin balance (GEN/NEX/FLX/AUR)',
      params: [{ name: 'bpi_address', type: 'string', required: true }],
      response: `{
  "success": true,
  "data": {
    "balance": {
      "gen": 1000,
      "nex": 500,
      "flx": 250,
      "aur": 100
    }
  }
}`
    }
  ];

  return (
    <div style={{ padding: '24px', maxWidth: '1400px', margin: '0 auto' }}>
      <div style={{ marginBottom: 24 }}>
        <Space>
          <ApiOutlined style={{ fontSize: '32px', color: '#1890ff' }} />
          <div>
            <Title level={2} style={{ margin: 0 }}>API Documentation</Title>
            <Text type="secondary">Integrate with BPCI Enterprise APIs</Text>
          </div>
        </Space>
      </div>

      <Alert
        message="API Key Required"
        description={`Your API Key: ${apiKey}`}
        type="info"
        showIcon
        style={{ marginBottom: 24 }}
        action={<Button size="small" icon={<CopyOutlined />}>Copy</Button>}
      />

      <Tabs defaultActiveKey="endpoints">
        <TabPane tab="Endpoints" key="endpoints">
          <Collapse>
            {endpoints.map((endpoint, index) => (
              <Panel
                header={
                  <Space>
                    <Tag color={endpoint.method === 'GET' ? 'blue' : 'green'}>{endpoint.method}</Tag>
                    <Text code>{endpoint.path}</Text>
                  </Space>
                }
                key={index}
              >
                <Paragraph>{endpoint.description}</Paragraph>
                <Title level={5}>Parameters:</Title>
                {endpoint.params.map((param, i) => (
                  <div key={i}>
                    <Text code>{param.name}</Text> - {param.type} {param.required && <Tag color="red">Required</Tag>}
                  </div>
                ))}
                <Title level={5} style={{ marginTop: 16 }}>Response:</Title>
                <pre style={{ background: '#f5f5f5', padding: 12, borderRadius: 4 }}>
                  {endpoint.response}
                </pre>
                <Button type="primary" icon={<PlayCircleOutlined />}>Try It</Button>
              </Panel>
            ))}
          </Collapse>
        </TabPane>
        <TabPane tab="Quick Start" key="quickstart">
          <Card>
            <Title level={4}>Getting Started</Title>
            <Paragraph>1. Get your API key from the dashboard</Paragraph>
            <Paragraph>2. Include the API key in your requests</Paragraph>
            <Paragraph>3. Make API calls to the endpoints</Paragraph>
            <pre style={{ background: '#f5f5f5', padding: 12, borderRadius: 4 }}>
{`curl -X GET \\
  -H "Authorization: Bearer YOUR_API_KEY" \\
  https://api.bpi.pravyom.com/v1/wallet/balance`}
            </pre>
          </Card>
        </TabPane>
      </Tabs>
    </div>
  );
};

export default ApiDocumentation;
