import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

const LogViewer: React.FC = () => {
  const [logs, setLogs] = useState<string[]>([]);
  const [selectedComponent, setSelectedComponent] = useState<string>('component_1');
  const [lineCount, setLineCount] = useState<number>(100);
  const [autoRefresh, setAutoRefresh] = useState<boolean>(true);

  const loadLogs = async () => {
    try {
      const logData = await invoke<string[]>('get_component_logs', {
        componentId: selectedComponent,
        lines: lineCount
      });
      setLogs(logData);
    } catch (error) {
      console.error('Failed to load logs:', error);
    }
  };

  useEffect(() => {
    loadLogs();
    if (autoRefresh) {
      const interval = setInterval(loadLogs, 3000);
      return () => clearInterval(interval);
    }
  }, [selectedComponent, lineCount, autoRefresh]);

  return (
    <div className="log-viewer">
      <h1>📝 Log Viewer</h1>
      
      <div className="log-controls">
        <select value={selectedComponent} onChange={(e) => setSelectedComponent(e.target.value)}>
          {Array.from({ length: 32 }, (_, i) => (
            <option key={i} value={`component_${i + 1}`}>Component {i + 1}</option>
          ))}
        </select>
        
        <select value={lineCount} onChange={(e) => setLineCount(Number(e.target.value))}>
          <option value={50}>50 lines</option>
          <option value={100}>100 lines</option>
          <option value={500}>500 lines</option>
          <option value={1000}>1000 lines</option>
        </select>
        
        <label>
          <input
            type="checkbox"
            checked={autoRefresh}
            onChange={(e) => setAutoRefresh(e.target.checked)}
          />
          Auto-refresh
        </label>
        
        <button onClick={loadLogs}>🔄 Refresh</button>
      </div>

      <div className="log-container">
        {logs.map((log, index) => (
          <div key={index} className="log-line">
            {log}
          </div>
        ))}
      </div>
    </div>
  );
};

export default LogViewer;
