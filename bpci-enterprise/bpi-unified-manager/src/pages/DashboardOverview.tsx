import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import './DashboardOverview.css';

interface ComponentStatus {
  id: string;
  name: string;
  category: string;
  status: string;
  cpu_usage: number;
  memory_usage: number;
  health: string;
}

const DashboardOverview: React.FC = () => {
  const [components, setComponents] = useState<ComponentStatus[]>([]);
  const [loading, setLoading] = useState(false);
  const [stats, setStats] = useState({
    total: 32,
    running: 0,
    stopped: 0,
    error: 0,
    avgCpu: 0,
    avgMemory: 0
  });

  const loadComponents = async () => {
    setLoading(true);
    try {
      const statuses = await invoke<ComponentStatus[]>('get_all_component_status');
      setComponents(statuses);
      
      // Calculate stats
      const running = statuses.filter(c => c.status === 'Running').length;
      const stopped = statuses.filter(c => c.status === 'Stopped').length;
      const error = statuses.filter(c => c.status === 'Error').length;
      const avgCpu = statuses.reduce((sum, c) => sum + c.cpu_usage, 0) / statuses.length;
      const avgMemory = statuses.reduce((sum, c) => sum + c.memory_usage, 0) / statuses.length;
      
      setStats({
        total: 32,
        running,
        stopped,
        error,
        avgCpu,
        avgMemory
      });
    } catch (error) {
      console.error('Failed to load components:', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadComponents();
    const interval = setInterval(loadComponents, 5000); // Refresh every 5 seconds
    return () => clearInterval(interval);
  }, []);

  const getCategoryComponents = (category: string) => {
    return components.filter(c => c.category === category);
  };

  const categories = [
    { name: 'BPCI Core', key: 'BpciCore', color: '#1890ff' },
    { name: 'BPI OS Core', key: 'BpiOsCore', color: '#52c41a' },
    { name: 'vPod Infrastructure', key: 'VPodInfra', color: '#722ed1' },
    { name: 'Network & Security', key: 'NetworkSecurity', color: '#faad14' },
    { name: 'Economy & Governance', key: 'EconomyGovernance', color: '#eb2f96' },
    { name: 'Storage & Data', key: 'StorageData', color: '#13c2c2' }
  ];

  return (
    <div className="dashboard-overview">
      <div className="header">
        <h1>🎯 Unified Component Dashboard</h1>
        <p>Manage all 32 components from one place</p>
      </div>

      {/* Stats Overview */}
      <div className="stats-grid">
        <div className="stat-card">
          <div className="stat-icon">📊</div>
          <div className="stat-content">
            <div className="stat-label">Total Components</div>
            <div className="stat-value">{stats.total}</div>
          </div>
        </div>
        
        <div className="stat-card success">
          <div className="stat-icon">✅</div>
          <div className="stat-content">
            <div className="stat-label">Running</div>
            <div className="stat-value">{stats.running}</div>
          </div>
        </div>
        
        <div className="stat-card warning">
          <div className="stat-icon">⏸️</div>
          <div className="stat-content">
            <div className="stat-label">Stopped</div>
            <div className="stat-value">{stats.stopped}</div>
          </div>
        </div>
        
        <div className="stat-card error">
          <div className="stat-icon">❌</div>
          <div className="stat-content">
            <div className="stat-label">Error</div>
            <div className="stat-value">{stats.error}</div>
          </div>
        </div>
        
        <div className="stat-card">
          <div className="stat-icon">💻</div>
          <div className="stat-content">
            <div className="stat-label">Avg CPU</div>
            <div className="stat-value">{stats.avgCpu.toFixed(1)}%</div>
          </div>
        </div>
        
        <div className="stat-card">
          <div className="stat-icon">🧠</div>
          <div className="stat-content">
            <div className="stat-label">Avg Memory</div>
            <div className="stat-value">{stats.avgMemory.toFixed(0)} MB</div>
          </div>
        </div>
      </div>

      {/* Category Grid */}
      <div className="categories-grid">
        {categories.map(category => {
          const categoryComponents = getCategoryComponents(category.key);
          const runningCount = categoryComponents.filter(c => c.status === 'Running').length;
          
          return (
            <div key={category.key} className="category-card" style={{ borderColor: category.color }}>
              <div className="category-header">
                <h3>{category.name}</h3>
                <span className="category-count">{runningCount}/{categoryComponents.length}</span>
              </div>
              <div className="category-components">
                {categoryComponents.map(comp => (
                  <div key={comp.id} className="component-item">
                    <span className={`status-dot ${comp.status.toLowerCase()}`}></span>
                    <span className="component-name">{comp.name}</span>
                    <span className="component-cpu">{comp.cpu_usage.toFixed(0)}%</span>
                  </div>
                ))}
              </div>
            </div>
          );
        })}
      </div>

      {/* Quick Actions */}
      <div className="quick-actions">
        <button className="action-btn primary" onClick={loadComponents}>
          🔄 Refresh All
        </button>
        <button className="action-btn success">
          ▶️ Start All
        </button>
        <button className="action-btn warning">
          ⏸️ Stop All
        </button>
        <button className="action-btn">
          🔧 Configure
        </button>
      </div>
    </div>
  );
};

export default DashboardOverview;
