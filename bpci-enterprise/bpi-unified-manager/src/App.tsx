import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import './App.css';

interface ComponentStatus {
  id: string;
  name: string;
  status: string;
  cpu_usage: number;
  memory_usage: number;
}

function App() {
  const [components, setComponents] = useState<ComponentStatus[]>([]);
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState('');

  const loadComponents = async () => {
    setLoading(true);
    try {
      const statuses = await invoke<ComponentStatus[]>('get_all_component_status');
      setComponents(statuses);
      setMessage('Components loaded successfully!');
    } catch (error) {
      setMessage(`Error: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const startAll = async () => {
    setLoading(true);
    try {
      const result = await invoke<string>('start_all_components');
      setMessage(result);
      await loadComponents();
    } catch (error) {
      setMessage(`Error: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const stopAll = async () => {
    setLoading(true);
    try {
      const result = await invoke<string>('stop_all_components');
      setMessage(result);
      await loadComponents();
    } catch (error) {
      setMessage(`Error: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadComponents();
  }, []);

  return (
    <div className="container">
      <h1>BPI Unified Manager</h1>
      <h2>Manage All 32 Components</h2>
      
      <div className="controls">
        <button onClick={startAll} disabled={loading}>
          Start All Components
        </button>
        <button onClick={stopAll} disabled={loading}>
          Stop All Components
        </button>
        <button onClick={loadComponents} disabled={loading}>
          Refresh Status
        </button>
      </div>

      {message && (
        <div className="message">
          {message}
        </div>
      )}

      <div className="components">
        <h3>Components ({components.length}/32)</h3>
        {loading ? (
          <p>Loading...</p>
        ) : (
          <table>
            <thead>
              <tr>
                <th>ID</th>
                <th>Name</th>
                <th>Status</th>
                <th>CPU</th>
                <th>Memory</th>
              </tr>
            </thead>
            <tbody>
              {components.map((comp) => (
                <tr key={comp.id}>
                  <td>{comp.id}</td>
                  <td>{comp.name}</td>
                  <td>
                    <span className={`status-${comp.status.toLowerCase()}`}>
                      {comp.status}
                    </span>
                  </td>
                  <td>{comp.cpu_usage.toFixed(1)}%</td>
                  <td>{comp.memory_usage} MB</td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>
    </div>
  );
}

export default App;
