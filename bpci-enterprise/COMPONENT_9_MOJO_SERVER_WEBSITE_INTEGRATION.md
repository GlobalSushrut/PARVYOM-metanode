  # Component 9: Mojo Server - Website Integration Plan

**Date**: 2025-10-26  
**Approach**: Extend existing Vite + React + TypeScript website  
**Status**: Planning Complete - Ready for Implementation

---

## **✅ Existing Website Analysis**

### **Current Tech Stack** (Perfect for Mojo Server!)

```json
{
  "framework": "Vite + React 19 + TypeScript",
  "ui": "Ant Design + TailwindCSS",
  "charts": "Recharts + @ant-design/plots",
  "routing": "React Router DOM",
  "state": "Zustand",
  "http": "Axios",
  "icons": "Ant Design Icons + Lucide React"
}
```

**Status**: ✅ **Perfect stack - No additional dependencies needed!**

---

## **🎯 Integration Strategy**

### **Add Mojo Server Admin Panel as New Section**

```
Existing Website Structure:
├── Home
├── About
├── Technology
├── Enterprise
├── Get Started
└── Research

NEW: Add Mojo Server Admin Panel:
└── /admin (or /mojo)
    ├── Dashboard (overview)
    ├── Network Monitoring
    ├── Security Alerts
    ├── Wallet Management
    ├── Infrastructure
    └── Settings
```

---

## **📋 Implementation Plan**

### **Phase 1: Add Admin Routes**

**File**: `website/bpci-enterprise-website/src/App.tsx`

```typescript
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import MojoAdminLayout from './pages/admin/MojoAdminLayout';
import MojoDashboard from './pages/admin/Dashboard';
import MojoNetworkMonitoring from './pages/admin/NetworkMonitoring';
import MojoSecurity from './pages/admin/Security';
import MojoWallets from './pages/admin/Wallets';

function App() {
  return (
    <BrowserRouter>
      <Routes>
        {/* Existing routes */}
        <Route path="/" element={<Home />} />
        <Route path="/about" element={<About />} />
        
        {/* NEW: Mojo Server Admin Panel */}
        <Route path="/admin" element={<MojoAdminLayout />}>
          <Route index element={<MojoDashboard />} />
          <Route path="network" element={<MojoNetworkMonitoring />} />
          <Route path="security" element={<MojoSecurity />} />
          <Route path="wallets" element={<MojoWallets />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}
```

---

### **Phase 2: Create Admin Layout**

**File**: `website/bpci-enterprise-website/src/pages/admin/MojoAdminLayout.tsx`

```typescript
import { Layout, Menu } from 'antd';
import { DashboardOutlined, SecurityScanOutlined, WalletOutlined, ClusterOutlined } from '@ant-design/icons';
import { Outlet, useNavigate } from 'react-router-dom';

const { Header, Sider, Content } = Layout;

export default function MojoAdminLayout() {
  const navigate = useNavigate();

  const menuItems = [
    { key: '/admin', icon: <DashboardOutlined />, label: 'Dashboard' },
    { key: '/admin/network', icon: <ClusterOutlined />, label: 'Network' },
    { key: '/admin/security', icon: <SecurityScanOutlined />, label: 'Security' },
    { key: '/admin/wallets', icon: <WalletOutlined />, label: 'Wallets' },
  ];

  return (
    <Layout style={{ minHeight: '100vh' }}>
      <Header style={{ background: '#001529', color: 'white', padding: '0 24px' }}>
        <h1 style={{ color: 'white', margin: 0 }}>Mojo Server - Admin Panel</h1>
      </Header>
      <Layout>
        <Sider width={200} style={{ background: '#fff' }}>
          <Menu
            mode="inline"
            defaultSelectedKeys={['/admin']}
            items={menuItems}
            onClick={({ key }) => navigate(key)}
          />
        </Sider>
        <Content style={{ padding: '24px', background: '#f0f2f5' }}>
          <Outlet />
        </Content>
      </Layout>
    </Layout>
  );
}
```

---

### **Phase 3: Dashboard Page (Overview)**

**File**: `website/bpci-enterprise-website/src/pages/admin/Dashboard.tsx`

```typescript
import { Card, Row, Col, Statistic } from 'antd';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';
import { useEffect, useState } from 'react';
import axios from 'axios';

export default function MojoDashboard() {
  const [stats, setStats] = useState({
    totalNodes: 0,
    activeNodes: 0,
    totalTransactions: 0,
    securityAlerts: 0,
  });

  useEffect(() => {
    // Fetch stats from Mojo Server
    axios.get('http://localhost:8089/api/v1/stats')
      .then(res => setStats(res.data))
      .catch(err => console.error(err));
  }, []);

  return (
    <div>
      <h2>Mojo Server Dashboard</h2>
      
      {/* Stats Cards */}
      <Row gutter={16} style={{ marginBottom: 24 }}>
        <Col span={6}>
          <Card>
            <Statistic title="Total Nodes" value={stats.totalNodes} />
          </Card>
        </Col>
        <Col span={6}>
          <Card>
            <Statistic title="Active Nodes" value={stats.activeNodes} valueStyle={{ color: '#3f8600' }} />
          </Card>
        </Col>
        <Col span={6}>
          <Card>
            <Statistic title="Transactions" value={stats.totalTransactions} />
          </Card>
        </Col>
        <Col span={6}>
          <Card>
            <Statistic title="Security Alerts" value={stats.securityAlerts} valueStyle={{ color: '#cf1322' }} />
          </Card>
        </Col>
      </Row>

      {/* Real-time Chart */}
      <Card title="Network Activity">
        <ResponsiveContainer width="100%" height={300}>
          <LineChart data={[]}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="time" />
            <YAxis />
            <Tooltip />
            <Line type="monotone" dataKey="transactions" stroke="#8884d8" />
            <Line type="monotone" dataKey="nodes" stroke="#82ca9d" />
          </LineChart>
        </ResponsiveContainer>
      </Card>
    </div>
  );
}
```

---

### **Phase 4: Network Monitoring Page**

**File**: `website/bpci-enterprise-website/src/pages/admin/NetworkMonitoring.tsx`

```typescript
import { Card, Table, Tag, Progress } from 'antd';
import { useEffect, useState } from 'react';
import axios from 'axios';

export default function MojoNetworkMonitoring() {
  const [components, setComponents] = useState([]);

  useEffect(() => {
    // Fetch component status from Mojo Server
    axios.get('http://localhost:8089/api/v1/components')
      .then(res => setComponents(res.data))
      .catch(err => console.error(err));
  }, []);

  const columns = [
    { title: 'Component', dataIndex: 'name', key: 'name' },
    { title: 'Status', dataIndex: 'status', key: 'status', render: (status: string) => (
      <Tag color={status === 'healthy' ? 'green' : 'red'}>{status.toUpperCase()}</Tag>
    )},
    { title: 'CPU', dataIndex: 'cpu', key: 'cpu', render: (cpu: number) => (
      <Progress percent={cpu} size="small" />
    )},
    { title: 'Memory', dataIndex: 'memory', key: 'memory', render: (mem: number) => (
      <Progress percent={mem} size="small" />
    )},
    { title: 'Uptime', dataIndex: 'uptime', key: 'uptime' },
  ];

  return (
    <div>
      <h2>Network Monitoring</h2>
      <Card title="BPCI Components Status">
        <Table dataSource={components} columns={columns} />
      </Card>
    </div>
  );
}
```

---

### **Phase 5: Security Monitoring Page**

**File**: `website/bpci-enterprise-website/src/pages/admin/Security.tsx`

```typescript
import { Card, Table, Tag, Badge } from 'antd';
import { WarningOutlined, CheckCircleOutlined } from '@ant-design/icons';

export default function MojoSecurity() {
  const securityEvents = [
    { id: 1, type: 'Intrusion Attempt', severity: 'high', node: 'node-123', time: '2 min ago' },
    { id: 2, type: 'Unusual Activity', severity: 'medium', node: 'node-456', time: '5 min ago' },
    { id: 3, type: 'Login Success', severity: 'low', node: 'node-789', time: '10 min ago' },
  ];

  const columns = [
    { title: 'Event', dataIndex: 'type', key: 'type' },
    { title: 'Severity', dataIndex: 'severity', key: 'severity', render: (severity: string) => {
      const color = severity === 'high' ? 'red' : severity === 'medium' ? 'orange' : 'green';
      return <Tag color={color}>{severity.toUpperCase()}</Tag>;
    }},
    { title: 'Node', dataIndex: 'node', key: 'node' },
    { title: 'Time', dataIndex: 'time', key: 'time' },
  ];

  return (
    <div>
      <h2>Security Monitoring</h2>
      
      <Card title="Security Overview" style={{ marginBottom: 16 }}>
        <div style={{ display: 'flex', gap: 24 }}>
          <div>
            <Badge status="success" />
            <span>Active Nodes: 1,234</span>
          </div>
          <div>
            <Badge status="error" />
            <span>Security Alerts: 3</span>
          </div>
          <div>
            <Badge status="warning" />
            <span>Warnings: 12</span>
          </div>
        </div>
      </Card>

      <Card title="Recent Security Events">
        <Table dataSource={securityEvents} columns={columns} />
      </Card>
    </div>
  );
}
```

---

### **Phase 6: Wallet Management Page**

**File**: `website/bpci-enterprise-website/src/pages/admin/Wallets.tsx`

```typescript
import { Card, Table, Button, Input } from 'antd';
import { SearchOutlined } from '@ant-design/icons';
import { useState, useEffect } from 'react';
import axios from 'axios';

export default function MojoWallets() {
  const [wallets, setWallets] = useState([]);

  useEffect(() => {
    // Fetch wallets from Mojo Server
    axios.get('http://localhost:8089/api/v1/wallet')
      .then(res => setWallets(res.data))
      .catch(err => console.error(err));
  }, []);

  const columns = [
    { title: 'Wallet Address', dataIndex: 'bpi_wallet_address', key: 'address' },
    { title: 'Node ID', dataIndex: 'bpi_node_id', key: 'node' },
    { title: 'Created', dataIndex: 'created_at', key: 'created', render: (date: string) => 
      new Date(date).toLocaleString()
    },
    { title: 'Actions', key: 'actions', render: () => (
      <Button type="link">View Dashboard</Button>
    )},
  ];

  return (
    <div>
      <h2>Wallet Management</h2>
      
      <Card>
        <Input 
          placeholder="Search wallets..." 
          prefix={<SearchOutlined />}
          style={{ marginBottom: 16 }}
        />
        <Table dataSource={wallets} columns={columns} />
      </Card>
    </div>
  );
}
```

---

## **🔧 Backend Integration**

### **Update Mojo Server for CORS**

**File**: `src/bin/bpci_mojo_server.rs`

```rust
use tower_http::cors::{CorsLayer, Any};

// In main()
let app = Router::new()
    .route("/health", get(health_check))
    .route("/api/v1/wallet", post(create_mojo_wallet))
    .route("/api/v1/wallet", get(list_wallets))
    .route("/api/v1/stats", get(get_stats))  // NEW
    .route("/api/v1/components", get(get_components))  // NEW
    .layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    )
    .with_state(state);
```

---

## **📁 File Structure**

```
website/bpci-enterprise-website/
├── src/
│   ├── pages/
│   │   ├── admin/              # NEW: Mojo Server Admin
│   │   │   ├── MojoAdminLayout.tsx
│   │   │   ├── Dashboard.tsx
│   │   │   ├── NetworkMonitoring.tsx
│   │   │   ├── Security.tsx
│   │   │   └── Wallets.tsx
│   │   ├── Home.tsx            # Existing
│   │   ├── About.tsx           # Existing
│   │   └── ...
│   ├── App.tsx                 # Update with admin routes
│   └── ...
└── package.json                # No changes needed!
```

---

## **🚀 Implementation Steps**

### **Step 1: Create Admin Pages**
```bash
cd website/bpci-enterprise-website
mkdir -p src/pages/admin
```

### **Step 2: Add Routes to App.tsx**
- Import admin components
- Add `/admin` routes

### **Step 3: Update Mojo Server**
- Add CORS support
- Add new API endpoints (stats, components)

### **Step 4: Test Integration**
```bash
# Terminal 1: Start Mojo Server
cargo run --bin bpci_mojo_server --release

# Terminal 2: Start Website
cd website/bpci-enterprise-website
npm run dev

# Open: http://localhost:5173/admin
```

---

## **✅ Advantages of This Approach**

1. ✅ **Reuse existing website** - No new project needed
2. ✅ **All dependencies already installed** - Recharts, Ant Design, etc.
3. ✅ **Consistent UI** - Same design system as main website
4. ✅ **Easy deployment** - Single website deployment
5. ✅ **TypeScript** - Type-safe development
6. ✅ **Fast development** - Vite HMR for instant updates

---

## **🎯 Summary**

**Approach**: Extend existing Vite + React + TypeScript website  
**New Section**: `/admin` routes for Mojo Server admin panel  
**Dependencies**: ✅ All already installed (Recharts, Ant Design, Zustand)  
**Backend**: Update Mojo Server with CORS and new API endpoints  
**Timeline**: 1-2 weeks for full implementation

---

**Status**: ✅ **Plan Complete - Ready to Implement**  
**Next Step**: Create admin pages in existing website structure
