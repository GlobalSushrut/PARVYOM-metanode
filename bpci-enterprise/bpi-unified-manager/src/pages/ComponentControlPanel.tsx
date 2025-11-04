import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

interface ComponentStatus {
  id: string;
  name: string;
  category: string;
  status: string;
  port?: number;
  endpoint?: string;
  cpu_usage: number;
  memory_usage: number;
  health: string;
  uptime?: number;
}

const ComponentControlPanel: React.FC = () => {
  const [components, setComponents] = useState<ComponentStatus[]>([]);
  const [loading, setLoading] = useState(false);
  const [filter, setFilter] = useState('all');
  const [searchTerm, setSearchTerm] = useState('');

  const loadComponents = async () => {
    setLoading(true);
    try {
      const statuses = await invoke<ComponentStatus[]>('get_all_component_status');
      setComponents(statuses);
    } catch (error) {
      console.error('Failed to load components:', error);
    } finally {
      setLoading(false);
    }
  };

  const startComponent = async (id: string) => {
    try {
      await invoke('start_component', { componentId: id });
      await loadComponents();
    } catch (error) {
      console.error('Failed to start component:', error);
    }
  };

  const stopComponent = async (id: string) => {
    try {
      await invoke('stop_component', { componentId: id });
      await loadComponents();
    } catch (error) {
      console.error('Failed to stop component:', error);
    }
  };

  const restartComponent = async (id: string) => {
    try {
      await invoke('restart_component', { componentId: id });
      await loadComponents();
    } catch (error) {
      console.error('Failed to restart component:', error);
    }
  };

  useEffect(() => {
    loadComponents();
  }, []);

  const filteredComponents = components.filter(comp => {
    const matchesFilter = filter === 'all' || comp.status.toLowerCase() === filter;
    const matchesSearch = comp.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
                         comp.id.toLowerCase().includes(searchTerm.toLowerCase());
    return matchesFilter && matchesSearch;
  });

  return (
    <div className="component-control-panel">
      <h1>🎛️ Component Control Panel</h1>
      
      <div className="controls">
        <input
          type="text"
          placeholder="Search components..."
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          className="search-input"
        />
        
        <select value={filter} onChange={(e) => setFilter(e.target.value)} className="filter-select">
          <option value="all">All Status</option>
          <option value="running">Running</option>
          <option value="stopped">Stopped</option>
          <option value="error">Error</option>
        </select>
        
        <button onClick={loadComponents} className="refresh-btn">🔄 Refresh</button>
      </div>

      <div className="components-table">
        <table>
          <thead>
            <tr>
              <th>ID</th>
              <th>Name</th>
              <th>Category</th>
              <th>Status</th>
              <th>Port</th>
              <th>CPU</th>
              <th>Memory</th>
              <th>Health</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {filteredComponents.map(comp => (
              <tr key={comp.id}>
                <td>{comp.id}</td>
                <td>{comp.name}</td>
                <td>{comp.category}</td>
                <td>
                  <span className={`status-badge ${comp.status.toLowerCase()}`}>
                    {comp.status}
                  </span>
                </td>
                <td>{comp.port || 'N/A'}</td>
                <td>{comp.cpu_usage.toFixed(1)}%</td>
                <td>{comp.memory_usage} MB</td>
                <td>
                  <span className={`health-badge ${comp.health.toLowerCase()}`}>
                    {comp.health}
                  </span>
                </td>
                <td className="actions">
                  {comp.status === 'Running' ? (
                    <>
                      <button onClick={() => stopComponent(comp.id)} className="btn-stop">⏸️</button>
                      <button onClick={() => restartComponent(comp.id)} className="btn-restart">🔄</button>
                    </>
                  ) : (
                    <button onClick={() => startComponent(comp.id)} className="btn-start">▶️</button>
                  )}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default ComponentControlPanel;
