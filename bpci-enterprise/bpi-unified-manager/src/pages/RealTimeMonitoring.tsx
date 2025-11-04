import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

interface ComponentMetrics {
  component_id: string;
  cpu_usage: number;
  memory_usage: number;
  network_in: number;
  network_out: number;
}

const RealTimeMonitoring: React.FC = () => {
  const [metrics, setMetrics] = useState<ComponentMetrics[]>([]);
  const [selectedComponent, setSelectedComponent] = useState<string>('component_1');

  const loadMetrics = async () => {
    try {
      const data = await invoke<ComponentMetrics>('get_component_metrics', {
        componentId: selectedComponent
      });
      setMetrics(prev => [...prev.slice(-50), data]);
    } catch (error) {
      console.error('Failed to load metrics:', error);
    }
  };

  useEffect(() => {
    const interval = setInterval(loadMetrics, 1000);
    return () => clearInterval(interval);
  }, [selectedComponent]);

  return (
    <div className="real-time-monitoring">
      <h1>📊 Real-Time Monitoring</h1>
      
      <select value={selectedComponent} onChange={(e) => setSelectedComponent(e.target.value)}>
        {Array.from({ length: 32 }, (_, i) => (
          <option key={i} value={`component_${i + 1}`}>Component {i + 1}</option>
        ))}
      </select>

      <div className="metrics-grid">
        <div className="metric-card">
          <h3>CPU Usage</h3>
          <div className="metric-value">
            {metrics[metrics.length - 1]?.cpu_usage.toFixed(1) || 0}%
          </div>
        </div>
        
        <div className="metric-card">
          <h3>Memory Usage</h3>
          <div className="metric-value">
            {metrics[metrics.length - 1]?.memory_usage || 0} MB
          </div>
        </div>
        
        <div className="metric-card">
          <h3>Network In</h3>
          <div className="metric-value">
            {(metrics[metrics.length - 1]?.network_in / 1024).toFixed(2) || 0} KB/s
          </div>
        </div>
        
        <div className="metric-card">
          <h3>Network Out</h3>
          <div className="metric-value">
            {(metrics[metrics.length - 1]?.network_out / 1024).toFixed(2) || 0} KB/s
          </div>
        </div>
      </div>

      <div className="chart-container">
        <h3>Performance History</h3>
        <p>Live charts would be rendered here using a charting library</p>
      </div>
    </div>
  );
};

export default RealTimeMonitoring;
