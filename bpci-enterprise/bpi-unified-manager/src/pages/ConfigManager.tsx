import React, { useState } from 'react';

const ConfigManager: React.FC = () => {
  const [config, setConfig] = useState<string>('');
  const [selectedFile, setSelectedFile] = useState<string>('env.ini');

  const configFiles = [
    'env.ini',
    'component_1.toml',
    'component_2.toml',
    'bso-k8.toml',
    'cluster-ledger.toml'
  ];

  return (
    <div className="config-manager">
      <h1>⚙️ Configuration Manager</h1>
      
      <div className="config-controls">
        <select value={selectedFile} onChange={(e) => setSelectedFile(e.target.value)}>
          {configFiles.map(file => (
            <option key={file} value={file}>{file}</option>
          ))}
        </select>
        
        <button>💾 Save</button>
        <button>🔄 Reload</button>
        <button>✅ Validate</button>
      </div>

      <div className="config-editor">
        <textarea
          value={config}
          onChange={(e) => setConfig(e.target.value)}
          placeholder="Configuration content will be loaded here..."
          rows={30}
        />
      </div>

      <div className="config-help">
        <h3>Configuration Help</h3>
        <p>Edit configuration files for all 32 components</p>
        <ul>
          <li>env.ini - Global environment configuration</li>
          <li>*.toml - Component-specific configuration</li>
          <li>Validate before saving to prevent errors</li>
        </ul>
      </div>
    </div>
  );
};

export default ConfigManager;
