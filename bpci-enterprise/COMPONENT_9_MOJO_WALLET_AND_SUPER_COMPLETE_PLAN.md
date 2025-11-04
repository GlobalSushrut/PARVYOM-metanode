# Component 9: Mojo Wallet & Mojo Super - Complete Implementation Plan

**Date**: 2025-10-26  
**Status**: Completion Plan  
**Existing**: Wallet components (incomplete)  
**Goal**: Complete Mojo Wallet + Mojo Super

---

## **🎯 Clear Architecture**

### **Two Distinct Systems**

```
┌─────────────────────────────────────────────────────────────┐
│  Mojo Wallet (Individual User)                               │
│  ─────────────────────────────────────────────────────────  │
│  Purpose: Wallet + Dashboard for each BPI address           │
│  Users: Individual BPI OS node owners                        │
│  Access: Per BPI address + token authentication             │
│  Features:                                                   │
│    - BPI Wallet (send/receive, balance)                     │
│    - Personal Dashboard (node metrics)                       │
│    - Security monitoring (for this address)                  │
│    - Transaction history                                     │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│  Mojo Super (Company/Owner Admin)                           │
│  ─────────────────────────────────────────────────────────  │
│  Purpose: Main admin panel for company/owner                │
│  Users: BPCI company owner, administrators                   │
│  Access: Admin credentials                                   │
│  Features:                                                   │
│    - View ALL Mojo wallets                                  │
│    - Network-wide monitoring                                 │
│    - Security oversight                                      │
│    - Infrastructure management                               │
│    - Analytics and reporting                                 │
└─────────────────────────────────────────────────────────────┘
```

---

## **📊 Existing Components Analysis**

### **What You Have**

```
website/bpci-enterprise-website/src/components/Wallet/
├── BpiWalletSystem.tsx (21KB)
├── AdvancedWalletSystem.tsx (23KB)
└── WalletManager.tsx (13KB)
```

**Status**: ✅ Good foundation, needs completion for Mojo Wallet/Super

---

## **🔧 Implementation Plan**

### **Part 1: Complete Mojo Wallet (Individual User)**

#### **What Mojo Wallet Needs**

**1. Wallet Interface** (Extend existing BpiWalletSystem.tsx)
- ✅ Send/receive transactions
- ✅ View balance
- ✅ Transaction history
- ⚠️ **ADD**: BPI address + token authentication
- ⚠️ **ADD**: Connect to Mojo Server backend

**2. Personal Dashboard** (NEW)
- Node metrics (CPU, memory, disk)
- Consensus participation
- Transaction throughput
- Connection status to BPCI

**3. Security Monitor** (NEW)
- Local security events
- Firewall status
- Audit logs
- Alerts

**File Structure**:
```
src/pages/mojo-wallet/
├── MojoWalletLayout.tsx          # Main layout
├── WalletDashboard.tsx            # Overview + balance
├── TransactionHistory.tsx         # Transaction list
├── NodeMetrics.tsx                # Node performance
└── SecurityMonitor.tsx            # Security events
```

---

### **Part 2: Build Mojo Super (Admin Panel)**

#### **What Mojo Super Needs**

**1. Admin Dashboard** (NEW)
- Total nodes overview
- Network health
- System-wide metrics
- Real-time charts

**2. Wallet Management** (NEW)
- List ALL Mojo wallets
- Search and filter
- View wallet details
- Manage permissions

**3. Network Monitoring** (NEW)
- BPCI Components 1-8 status
- Resource utilization
- Topology view

**4. Security Oversight** (NEW)
- Security events across all nodes
- Threat detection
- Compliance reporting

**File Structure**:
```
src/pages/mojo-super/
├── MojoSuperLayout.tsx            # Admin layout
├── AdminDashboard.tsx             # Overview
├── WalletManagement.tsx           # All wallets
├── NetworkMonitoring.tsx          # Components status
├── SecurityOversight.tsx          # Security events
└── Analytics.tsx                  # Reports
```

---

## **📋 Detailed Implementation**

### **Step 1: Mojo Wallet - Authentication**

**File**: `src/pages/mojo-wallet/MojoWalletAuth.tsx`

```typescript
import { useState } from 'react';
import { Form, Input, Button, Card } from 'antd';
import { WalletOutlined } from '@ant-design/icons';
import axios from 'axios';

export default function MojoWalletAuth({ onAuth }: { onAuth: (wallet: any) => void }) {
  const [loading, setLoading] = useState(false);

  const handleAuth = async (values: { address: string; token: string }) => {
    setLoading(true);
    try {
      // Verify BPI address + token with Mojo Server
      const response = await axios.post('http://localhost:8089/api/v1/wallet/auth', {
        bpi_wallet_address: values.address,
        access_token: values.token,
      });
      
      if (response.data.success) {
        onAuth(response.data.wallet);
      }
    } catch (error) {
      console.error('Authentication failed:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', minHeight: '100vh' }}>
      <Card title="Mojo Wallet Login" style={{ width: 400 }}>
        <Form onFinish={handleAuth} layout="vertical">
          <Form.Item 
            label="BPI Wallet Address" 
            name="address"
            rules={[{ required: true, message: 'Please enter your BPI address' }]}
          >
            <Input 
              prefix={<WalletOutlined />} 
              placeholder="bpi:wallet:abc123" 
            />
          </Form.Item>
          
          <Form.Item 
            label="Access Token" 
            name="token"
            rules={[{ required: true, message: 'Please enter your token' }]}
          >
            <Input.Password placeholder="Your access token" />
          </Form.Item>
          
          <Form.Item>
            <Button type="primary" htmlType="submit" loading={loading} block>
              Access Mojo Wallet
            </Button>
          </Form.Item>
        </Form>
      </Card>
    </div>
  );
}
```

---

### **Step 2: Mojo Wallet - Dashboard**

**File**: `src/pages/mojo-wallet/WalletDashboard.tsx`

```typescript
import { Card, Row, Col, Statistic, Progress } from 'antd';
import { WalletOutlined, ThunderboltOutlined, SafetyOutlined } from '@ant-design/icons';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';
import { useEffect, useState } from 'react';
import axios from 'axios';

export default function WalletDashboard({ walletAddress }: { walletAddress: string }) {
  const [balance, setBalance] = useState(0);
  const [metrics, setMetrics] = useState({
    transactions: 0,
    consensusParticipation: 0,
    uptime: 0,
  });

  useEffect(() => {
    // Fetch wallet data from Mojo Server
    axios.get(`http://localhost:8089/api/v1/wallet/${walletAddress}/dashboard`)
      .then(res => {
        setBalance(res.data.balance);
        setMetrics(res.data.metrics);
      });
  }, [walletAddress]);

  return (
    <div>
      <h2>Mojo Wallet Dashboard</h2>
      <p>Address: {walletAddress}</p>
      
      {/* Balance and Stats */}
      <Row gutter={16} style={{ marginBottom: 24 }}>
        <Col span={8}>
          <Card>
            <Statistic 
              title="Balance" 
              value={balance} 
              prefix={<WalletOutlined />}
              suffix="BPI"
            />
          </Card>
        </Col>
        <Col span={8}>
          <Card>
            <Statistic 
              title="Transactions" 
              value={metrics.transactions}
              prefix={<ThunderboltOutlined />}
            />
          </Card>
        </Col>
        <Col span={8}>
          <Card>
            <Statistic 
              title="Uptime" 
              value={metrics.uptime}
              suffix="%"
              prefix={<SafetyOutlined />}
            />
          </Card>
        </Col>
      </Row>

      {/* Node Performance */}
      <Card title="Node Performance" style={{ marginBottom: 16 }}>
        <Row gutter={16}>
          <Col span={8}>
            <div>CPU Usage</div>
            <Progress percent={45} />
          </Col>
          <Col span={8}>
            <div>Memory Usage</div>
            <Progress percent={60} />
          </Col>
          <Col span={8}>
            <div>Disk Usage</div>
            <Progress percent={30} />
          </Col>
        </Row>
      </Card>

      {/* Transaction History Chart */}
      <Card title="Transaction Activity">
        <ResponsiveContainer width="100%" height={300}>
          <LineChart data={[]}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="time" />
            <YAxis />
            <Tooltip />
            <Line type="monotone" dataKey="transactions" stroke="#8884d8" />
          </LineChart>
        </ResponsiveContainer>
      </Card>
    </div>
  );
}
```

---

### **Step 3: Mojo Super - Admin Dashboard**

**File**: `src/pages/mojo-super/AdminDashboard.tsx`

```typescript
import { Card, Row, Col, Statistic, Table, Tag } from 'antd';
import { UserOutlined, WalletOutlined, SafetyOutlined, ThunderboltOutlined } from '@ant-design/icons';
import { BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';
import { useEffect, useState } from 'react';
import axios from 'axios';

export default function AdminDashboard() {
  const [stats, setStats] = useState({
    totalWallets: 0,
    activeNodes: 0,
    totalTransactions: 0,
    securityAlerts: 0,
  });

  const [recentWallets, setRecentWallets] = useState([]);

  useEffect(() => {
    // Fetch admin stats from Mojo Server
    axios.get('http://localhost:8089/api/v1/admin/stats')
      .then(res => {
        setStats(res.data.stats);
        setRecentWallets(res.data.recentWallets);
      });
  }, []);

  const columns = [
    { title: 'Wallet Address', dataKey: 'address', key: 'address' },
    { title: 'Node ID', dataKey: 'nodeId', key: 'nodeId' },
    { title: 'Status', dataKey: 'status', key: 'status', render: (status: string) => (
      <Tag color={status === 'active' ? 'green' : 'red'}>{status.toUpperCase()}</Tag>
    )},
    { title: 'Balance', dataKey: 'balance', key: 'balance' },
  ];

  return (
    <div>
      <h2>Mojo Super - Admin Dashboard</h2>
      
      {/* Overview Stats */}
      <Row gutter={16} style={{ marginBottom: 24 }}>
        <Col span={6}>
          <Card>
            <Statistic 
              title="Total Mojo Wallets" 
              value={stats.totalWallets}
              prefix={<WalletOutlined />}
            />
          </Card>
        </Col>
        <Col span={6}>
          <Card>
            <Statistic 
              title="Active Nodes" 
              value={stats.activeNodes}
              prefix={<UserOutlined />}
              valueStyle={{ color: '#3f8600' }}
            />
          </Card>
        </Col>
        <Col span={6}>
          <Card>
            <Statistic 
              title="Total Transactions" 
              value={stats.totalTransactions}
              prefix={<ThunderboltOutlined />}
            />
          </Card>
        </Col>
        <Col span={6}>
          <Card>
            <Statistic 
              title="Security Alerts" 
              value={stats.securityAlerts}
              prefix={<SafetyOutlined />}
              valueStyle={{ color: '#cf1322' }}
            />
          </Card>
        </Col>
      </Row>

      {/* Network Activity Chart */}
      <Card title="Network Activity" style={{ marginBottom: 16 }}>
        <ResponsiveContainer width="100%" height={300}>
          <BarChart data={[]}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="hour" />
            <YAxis />
            <Tooltip />
            <Bar dataKey="transactions" fill="#8884d8" />
            <Bar dataKey="nodes" fill="#82ca9d" />
          </BarChart>
        </ResponsiveContainer>
      </Card>

      {/* Recent Wallets */}
      <Card title="Recently Created Mojo Wallets">
        <Table dataSource={recentWallets} columns={columns} />
      </Card>
    </div>
  );
}
```

---

### **Step 4: Mojo Super - Wallet Management**

**File**: `src/pages/mojo-super/WalletManagement.tsx`

```typescript
import { Card, Table, Input, Button, Tag, Modal } from 'antd';
import { SearchOutlined, EyeOutlined } from '@ant-design/icons';
import { useState, useEffect } from 'react';
import axios from 'axios';

export default function WalletManagement() {
  const [wallets, setWallets] = useState([]);
  const [selectedWallet, setSelectedWallet] = useState(null);
  const [modalVisible, setModalVisible] = useState(false);

  useEffect(() => {
    // Fetch all wallets from Mojo Server
    axios.get('http://localhost:8089/api/v1/admin/wallets')
      .then(res => setWallets(res.data));
  }, []);

  const columns = [
    { title: 'BPI Address', dataIndex: 'bpi_wallet_address', key: 'address' },
    { title: 'Node ID', dataIndex: 'bpi_node_id', key: 'node' },
    { title: 'Status', dataIndex: 'status', key: 'status', render: (status: string) => (
      <Tag color={status === 'active' ? 'green' : 'red'}>{status}</Tag>
    )},
    { title: 'Balance', dataIndex: 'balance', key: 'balance' },
    { title: 'Created', dataIndex: 'created_at', key: 'created', render: (date: string) =>
      new Date(date).toLocaleDateString()
    },
    { title: 'Actions', key: 'actions', render: (record: any) => (
      <Button 
        type="link" 
        icon={<EyeOutlined />}
        onClick={() => {
          setSelectedWallet(record);
          setModalVisible(true);
        }}
      >
        View Details
      </Button>
    )},
  ];

  return (
    <div>
      <h2>Wallet Management</h2>
      
      <Card>
        <Input 
          placeholder="Search by BPI address or node ID..." 
          prefix={<SearchOutlined />}
          style={{ marginBottom: 16 }}
        />
        <Table 
          dataSource={wallets} 
          columns={columns}
          pagination={{ pageSize: 10 }}
        />
      </Card>

      {/* Wallet Details Modal */}
      <Modal
        title="Wallet Details"
        open={modalVisible}
        onCancel={() => setModalVisible(false)}
        footer={null}
        width={600}
      >
        {selectedWallet && (
          <div>
            <p><strong>BPI Address:</strong> {selectedWallet.bpi_wallet_address}</p>
            <p><strong>Node ID:</strong> {selectedWallet.bpi_node_id}</p>
            <p><strong>Balance:</strong> {selectedWallet.balance} BPI</p>
            <p><strong>Status:</strong> <Tag color="green">Active</Tag></p>
            <p><strong>Created:</strong> {new Date(selectedWallet.created_at).toLocaleString()}</p>
          </div>
        )}
      </Modal>
    </div>
  );
}
```

---

## **🔗 Backend Updates (Mojo Server)**

### **New API Endpoints Needed**

```rust
// In bpci_mojo_server.rs

// Mojo Wallet endpoints
POST   /api/v1/wallet/auth                    // Authenticate with address + token
GET    /api/v1/wallet/:address/dashboard      // Get wallet dashboard data
GET    /api/v1/wallet/:address/transactions   // Get transaction history
GET    /api/v1/wallet/:address/metrics        // Get node metrics

// Mojo Super (admin) endpoints
GET    /api/v1/admin/stats                    // Get admin dashboard stats
GET    /api/v1/admin/wallets                  // Get all wallets
GET    /api/v1/admin/wallet/:address          // Get specific wallet details
GET    /api/v1/admin/network                  // Get network status
GET    /api/v1/admin/security                 // Get security events
```

---

## **📁 Complete File Structure**

```
website/bpci-enterprise-website/src/
├── components/
│   └── Wallet/                    # Existing (keep and extend)
│       ├── BpiWalletSystem.tsx
│       ├── AdvancedWalletSystem.tsx
│       └── WalletManager.tsx
│
├── pages/
│   ├── mojo-wallet/               # NEW: Mojo Wallet (individual user)
│   │   ├── MojoWalletAuth.tsx
│   │   ├── MojoWalletLayout.tsx
│   │   ├── WalletDashboard.tsx
│   │   ├── TransactionHistory.tsx
│   │   ├── NodeMetrics.tsx
│   │   └── SecurityMonitor.tsx
│   │
│   └── mojo-super/                # NEW: Mojo Super (admin)
│       ├── MojoSuperLayout.tsx
│       ├── AdminDashboard.tsx
│       ├── WalletManagement.tsx
│       ├── NetworkMonitoring.tsx
│       ├── SecurityOversight.tsx
│       └── Analytics.tsx
│
└── App.tsx                        # Update with routes
```

---

## **🚀 Implementation Timeline**

### **Week 1: Mojo Wallet**
- Day 1-2: Authentication (BPI address + token)
- Day 3-4: Wallet dashboard
- Day 5: Transaction history
- Day 6-7: Node metrics and security monitor

### **Week 2: Mojo Super**
- Day 1-2: Admin dashboard
- Day 3-4: Wallet management
- Day 5: Network monitoring
- Day 6-7: Security oversight and analytics

### **Week 3: Backend + Integration**
- Day 1-3: Mojo Server API endpoints
- Day 4-5: Integration testing
- Day 6-7: Polish and deployment

---

## **✅ Summary**

**Mojo Wallet** (Individual User):
- BPI address + token authentication
- Personal wallet + dashboard
- Node metrics and security
- Transaction history

**Mojo Super** (Company/Owner):
- Admin dashboard (all wallets overview)
- Wallet management (view all)
- Network monitoring (Components 1-8)
- Security oversight (all nodes)

**Existing Components**: ✅ Keep and extend  
**New Components**: Mojo Wallet pages + Mojo Super pages  
**Backend**: Update Mojo Server with new API endpoints

---

**Status**: ✅ **Complete Plan Ready**  
**Next Step**: Begin implementation of Mojo Wallet authentication and dashboard
