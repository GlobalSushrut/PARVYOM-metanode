import React, { useState, useEffect } from 'react';
import { Badge, Tooltip, Button } from 'antd';
import {
  CheckCircleOutlined,
  ExclamationCircleOutlined,
  CloseCircleOutlined,
  ReloadOutlined,
  WifiOutlined
} from '@ant-design/icons';
import { useRealTimeData } from '../../services/realTimeService';

interface NetworkStatusProps {
  showDetails?: boolean;
  size?: 'small' | 'default' | 'large';
}

const NetworkStatus: React.FC<NetworkStatusProps> = ({ 
  showDetails = false, 
  size = 'default' 
}) => {
  const { data, isConnected, refresh } = useRealTimeData();
  const [lastPing, setLastPing] = useState<number>(0);

  // Ping test for connection quality
  useEffect(() => {
    const pingTest = async () => {
      const start = Date.now();
      try {
        await fetch('http://127.0.0.1:8081/api/economy/status', { 
          method: 'HEAD',
          cache: 'no-cache'
        });
        setLastPing(Date.now() - start);
      } catch {
        setLastPing(-1); // Connection failed
      }
    };

    pingTest();
    const interval = setInterval(pingTest, 10000); // Ping every 10 seconds
    return () => clearInterval(interval);
  }, []);

  const getStatusColor = () => {
    if (!isConnected || lastPing === -1) return '#ff4d4f';
    if (data.networkStatus === 'degraded' || lastPing > 1000) return '#faad14';
    return '#52c41a';
  };

  const getStatusIcon = () => {
    if (!isConnected || lastPing === -1) return <CloseCircleOutlined />;
    if (data.networkStatus === 'degraded' || lastPing > 1000) return <ExclamationCircleOutlined />;
    return <CheckCircleOutlined />;
  };

  const getStatusText = () => {
    if (!isConnected || lastPing === -1) return 'Offline';
    if (data.networkStatus === 'degraded' || lastPing > 1000) return 'Degraded';
    return 'Online';
  };

  const getTooltipContent = () => (
    <div style={{ textAlign: 'left' }}>
      <div><strong>Network Status:</strong> {getStatusText()}</div>
      <div><strong>Backend:</strong> {isConnected ? 'Connected' : 'Disconnected'}</div>
      <div><strong>Ping:</strong> {lastPing === -1 ? 'Failed' : `${lastPing}ms`}</div>
      <div><strong>Nodes:</strong> {data.nodes}</div>
      <div><strong>Uptime:</strong> {data.uptime.toFixed(2)}%</div>
      <div><strong>Last Update:</strong> {new Date(data.lastUpdate).toLocaleTimeString()}</div>
    </div>
  );

  if (showDetails) {
    return (
      <div style={{
        display: 'flex',
        alignItems: 'center',
        gap: '12px',
        padding: '8px 16px',
        background: 'rgba(255, 255, 255, 0.05)',
        borderRadius: '8px',
        border: `1px solid ${getStatusColor()}40`,
        backdropFilter: 'blur(10px)'
      }}>
        <div style={{
          display: 'flex',
          alignItems: 'center',
          gap: '8px'
        }}>
          <WifiOutlined style={{ color: getStatusColor(), fontSize: '16px' }} />
          <span style={{ color: 'white', fontWeight: '600' }}>
            Network: {getStatusText()}
          </span>
        </div>
        
        <div style={{
          display: 'flex',
          alignItems: 'center',
          gap: '8px',
          fontSize: '12px',
          color: '#94a3b8'
        }}>
          <span>Ping: {lastPing === -1 ? 'Failed' : `${lastPing}ms`}</span>
          <span>•</span>
          <span>Nodes: {data.nodes}</span>
          <span>•</span>
          <span>Uptime: {data.uptime.toFixed(1)}%</span>
        </div>

        <Button
          type="text"
          size="small"
          icon={<ReloadOutlined />}
          onClick={refresh}
          style={{ color: '#94a3b8' }}
        />
      </div>
    );
  }

  return (
    <Tooltip title={getTooltipContent()} placement="bottom">
      <Badge
        color={getStatusColor()}
        text={
          <span style={{ 
            color: 'white', 
            fontWeight: '600',
            fontSize: size === 'small' ? '12px' : size === 'large' ? '16px' : '14px'
          }}>
            {getStatusIcon()} {getStatusText()}
          </span>
        }
        style={{ cursor: 'pointer' }}
      />
    </Tooltip>
  );
};

export default NetworkStatus;
