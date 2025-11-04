import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

interface ComponentStatus {
  id: string;
  name: string;
  status: string;
  health: string;
  uptime?: number;
  error_message?: string;
}

const HealthDashboard: React.FC = () => {
  const [components, setComponents] = useState<ComponentStatus[]>([]);
  const [healthStats, setHealthStats] = useState({
    healthy: 0,
    degraded: 0,
    unhealthy: 0,
    unknown: 0
  });

  const loadHealth = async () => {
    try {
      const statuses = await invoke<ComponentStatus[]>('get_all_component_status');
      setComponents(statuses);
      
      const healthy = statuses.filter(c => c.health === 'Healthy').length;
      const degraded = statuses.filter(c => c.health === 'Degraded').length;
      const unhealthy = statuses.filter(c => c.health === 'Unhealthy').length;
      const unknown = statuses.filter(c => c.health === 'Unknown').length;
      
      setHealthStats({ healthy, degraded, unhealthy, unknown });
    } catch (error) {
      console.error('Failed to load health:', error);
    }
  };

  useEffect(() => {
    loadHealth();
    const interval = setInterval(loadHealth, 5000);
    return () => clearInterval(interval);
  }, []);

  const getHealthIcon = (health: string) => {
    switch (health) {
      case 'Healthy': return '✅';
      case 'Degraded': return '⚠️';
      case 'Unhealthy': return '❌';
      default: return '❓';
    }
  };

  return (
    <div className="health-dashboard">
      <h1>🏥 Health Dashboard</h1>
      
      <div className="health-stats">
        <div className="health-card healthy">
          <div className="health-icon">✅</div>
          <div className="health-label">Healthy</div>
          <div className="health-count">{healthStats.healthy}</div>
        </div>
        
        <div className="health-card degraded">
          <div className="health-icon">⚠️</div>
          <div className="health-label">Degraded</div>
          <div className="health-count">{healthStats.degraded}</div>
        </div>
        
        <div className="health-card unhealthy">
          <div className="health-icon">❌</div>
          <div className="health-label">Unhealthy</div>
          <div className="health-count">{healthStats.unhealthy}</div>
        </div>
        
        <div className="health-card unknown">
          <div className="health-icon">❓</div>
          <div className="health-label">Unknown</div>
          <div className="health-count">{healthStats.unknown}</div>
        </div>
      </div>

      <div className="health-list">
        <h2>Component Health Status</h2>
        {components.map(comp => (
          <div key={comp.id} className={`health-item ${comp.health.toLowerCase()}`}>
            <span className="health-icon">{getHealthIcon(comp.health)}</span>
            <span className="component-name">{comp.name}</span>
            <span className="component-id">{comp.id}</span>
            <span className="health-status">{comp.health}</span>
            {comp.uptime && (
              <span className="uptime">Uptime: {Math.floor(comp.uptime / 3600)}h</span>
            )}
            {comp.error_message && (
              <span className="error-msg">{comp.error_message}</span>
            )}
          </div>
        ))}
      </div>
    </div>
  );
};

export default HealthDashboard;
