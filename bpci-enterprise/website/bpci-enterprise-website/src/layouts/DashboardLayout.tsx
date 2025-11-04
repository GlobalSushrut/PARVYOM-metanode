import React, { useState } from 'react';
import { Layout, Menu, Button, Avatar, Dropdown, Typography } from 'antd';
import {
  MenuFoldOutlined,
  MenuUnfoldOutlined,
  DashboardOutlined,
  WalletOutlined,
  FileTextOutlined,
  TeamOutlined,
  ApiOutlined,
  RocketOutlined,
  SettingOutlined,
  LogoutOutlined,
  UserOutlined,
  DatabaseOutlined,
  CloudServerOutlined,
  SafetyOutlined,
  BarChartOutlined,
  BookOutlined
} from '@ant-design/icons';
import { useNavigate, useLocation } from 'react-router-dom';
import { authService } from '../services/authService';
import type { MenuProps } from 'antd';

const { Header, Sider, Content } = Layout;
const { Text } = Typography;

interface DashboardLayoutProps {
  children: React.ReactNode;
}

const DashboardLayout: React.FC<DashboardLayoutProps> = ({ children }) => {
  // Smart default: open on desktop, collapsed on mobile
  const [collapsed, setCollapsed] = useState(false);
  const navigate = useNavigate();
  const location = useLocation();
  const user = authService.getCurrentDeveloper();

  const handleLogout = () => {
    authService.logout();
    navigate('/login');
  };

  const userMenuItems: MenuProps['items'] = [
    {
      key: 'profile',
      icon: <UserOutlined />,
      label: 'Profile Settings',
      onClick: () => navigate('/profile')
    },
    {
      key: 'security',
      icon: <SafetyOutlined />,
      label: 'Security',
      onClick: () => navigate('/security')
    },
    {
      type: 'divider'
    },
    {
      key: 'logout',
      icon: <LogoutOutlined />,
      label: 'Logout',
      onClick: handleLogout,
      danger: true
    }
  ];

  const menuItems: MenuProps['items'] = [
    {
      key: '/basic-dashboard',
      icon: <DashboardOutlined />,
      label: 'Dashboard',
      onClick: () => navigate('/basic-dashboard')
    },
    {
      key: 'wallet-group',
      icon: <WalletOutlined />,
      label: 'Wallet',
      children: [
        {
          key: '/wallet',
          label: 'My Wallet',
          onClick: () => navigate('/wallet')
        },
        {
          key: '/wallet/transactions',
          label: 'Transactions',
          onClick: () => navigate('/wallet/transactions')
        },
        {
          key: '/wallet/settings',
          label: 'Wallet Settings',
          onClick: () => navigate('/wallet/settings')
        }
      ]
    },
    {
      key: 'content-group',
      icon: <FileTextOutlined />,
      label: 'Content',
      children: [
        {
          key: '/blog',
          label: 'Blog Feed',
          onClick: () => navigate('/blog')
        },
        {
          key: '/blog/create',
          label: 'Create Post',
          onClick: () => navigate('/blog/create')
        },
        {
          key: '/community',
          label: 'Community',
          onClick: () => navigate('/community')
        }
      ]
    },
    {
      key: 'developer-group',
      icon: <ApiOutlined />,
      label: 'Developer',
      children: [
        {
          key: '/api-docs',
          label: 'API Documentation',
          onClick: () => navigate('/api-docs')
        },
        {
          key: '/mojo-dashboard',
          label: 'Mojo Dashboard',
          onClick: () => navigate('/mojo-dashboard')
        },
        {
          key: '/registry',
          label: 'Registry',
          onClick: () => navigate('/registry')
        }
      ]
    },
    {
      key: 'infrastructure-group',
      icon: <CloudServerOutlined />,
      label: 'Infrastructure',
      children: [
        {
          key: '/dashboard',
          label: 'System Dashboard',
          onClick: () => navigate('/dashboard')
        },
        {
          key: '/nodes',
          label: 'Node Management',
          onClick: () => navigate('/nodes')
        },
        {
          key: '/metrics',
          label: 'Metrics',
          onClick: () => navigate('/metrics')
        }
      ]
    },
    {
      key: 'resources-group',
      icon: <BookOutlined />,
      label: 'Resources',
      children: [
        {
          key: '/get-started',
          label: 'Get Started',
          onClick: () => navigate('/get-started')
        },
        {
          key: '/technology',
          label: 'Technology',
          onClick: () => navigate('/technology')
        },
        {
          key: '/enterprise',
          label: 'Enterprise',
          onClick: () => navigate('/enterprise')
        }
      ]
    }
  ];

  // Get current selected key from location
  const selectedKey = location.pathname;

  return (
    <Layout style={{ minHeight: '100vh', background: '#0A1628' }}>
      {/* Desktop Sidebar */}
      <Sider
        trigger={null}
        collapsible
        collapsed={collapsed}
        breakpoint="lg"
        collapsedWidth={80}
        width={260}
        style={{
          overflow: 'hidden',
          height: '100vh',
          background: '#0A1628',
          borderRight: '2px solid rgba(232, 180, 79, 0.2)',
          transition: 'all 0.2s ease-in-out'
        }}
        className="dashboard-sider"
      >
        <div style={{
          display: 'flex',
          flexDirection: 'column',
          height: '100%'
        }}>
          {/* Logo */}
          <div style={{
          height: '64px',
          display: 'flex',
          alignItems: 'center',
          justifyContent: collapsed ? 'center' : 'space-between',
          padding: collapsed ? '0' : '0 1.5rem',
          borderBottom: '1px solid rgba(232, 180, 79, 0.2)',
          flexShrink: 0
        }}>
          {!collapsed && (
            <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
              <div style={{
                width: '32px',
                height: '32px',
                borderRadius: '8px',
                background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                fontWeight: 'bold',
                color: '#0A1628'
              }}>
                P
              </div>
              <Text style={{ color: '#E8B44F', fontWeight: 'bold', fontSize: '1.125rem' }}>
                PRAVYOM
              </Text>
            </div>
          )}
          {collapsed && (
            <div style={{
              width: '32px',
              height: '32px',
              borderRadius: '8px',
              background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              fontWeight: 'bold',
              color: '#0A1628'
            }}>
              P
            </div>
          )}
        </div>

        {/* Menu - Scrollable */}
        <div style={{
          flex: 1,
          overflowY: 'auto',
          overflowX: 'hidden'
        }}>
          <Menu
            mode="inline"
            selectedKeys={[selectedKey]}
            defaultOpenKeys={['wallet-group', 'content-group', 'developer-group']}
            items={menuItems}
            style={{
              background: 'transparent',
              border: 'none',
              color: '#ffffff'
            }}
            theme="dark"
          />
        </div>

        {/* User Profile at Bottom */}
        {!collapsed && (
          <div style={{
            flexShrink: 0,
            padding: '1rem',
            borderTop: '1px solid rgba(232, 180, 79, 0.2)',
            background: 'rgba(232, 180, 79, 0.05)'
          }}>
            <Dropdown menu={{ items: userMenuItems }} placement="topRight">
              <div style={{
                display: 'flex',
                alignItems: 'center',
                gap: '0.75rem',
                cursor: 'pointer',
                padding: '0.5rem',
                borderRadius: '8px',
                transition: 'background 0.3s'
              }}
              onMouseEnter={(e) => e.currentTarget.style.background = 'rgba(232, 180, 79, 0.1)'}
              onMouseLeave={(e) => e.currentTarget.style.background = 'transparent'}
              >
                <Avatar
                  icon={<UserOutlined />}
                  style={{
                    background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)',
                    color: '#0A1628'
                  }}
                />
                <div style={{ flex: 1, overflow: 'hidden' }}>
                  <Text style={{ color: '#ffffff', fontWeight: '600', display: 'block', fontSize: '0.875rem' }} ellipsis>
                    {user?.name || 'Developer'}
                  </Text>
                  <Text style={{ color: '#9CA3AF', fontSize: '0.75rem', display: 'block' }} ellipsis>
                    {user?.email || 'user@example.com'}
                  </Text>
                </div>
              </div>
            </Dropdown>
          </div>
        )}
        </div>
      </Sider>

      {/* Main Layout */}
      <Layout>
        {/* Top Header */}
        <Header style={{
          padding: '0 1.5rem',
          background: '#0A1628',
          borderBottom: '1px solid rgba(232, 180, 79, 0.2)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between'
        }}>
          <Button
            type="text"
            icon={collapsed ? <MenuUnfoldOutlined /> : <MenuFoldOutlined />}
            onClick={() => setCollapsed(!collapsed)}
            style={{
              fontSize: '1.25rem',
              width: 48,
              height: 48,
              color: '#E8B44F'
            }}
          />

          {/* Right side - User dropdown for mobile */}
          <div className="mobile-only">
            <Dropdown menu={{ items: userMenuItems }} placement="bottomRight">
              <Avatar
                icon={<UserOutlined />}
                style={{
                  background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)',
                  color: '#0A1628',
                  cursor: 'pointer'
                }}
              />
            </Dropdown>
          </div>
        </Header>

        {/* Content */}
        <Content style={{
          margin: 0,
          minHeight: 'calc(100vh - 64px)',
          background: '#0A1628'
        }}>
          {children}
        </Content>
      </Layout>

      {/* Custom Styles */}
      <style>{`
        .dashboard-sider .ant-menu-dark {
          background: transparent !important;
        }
        
        .dashboard-sider .ant-menu-item {
          color: #9CA3AF !important;
          margin: 4px 8px !important;
          border-radius: 8px !important;
        }
        
        .dashboard-sider .ant-menu-item:hover {
          background: rgba(232, 180, 79, 0.1) !important;
          color: #E8B44F !important;
        }
        
        .dashboard-sider .ant-menu-item-selected {
          background: linear-gradient(135deg, rgba(232, 180, 79, 0.2) 0%, rgba(232, 180, 79, 0.1) 100%) !important;
          color: #E8B44F !important;
          border-right: 3px solid #E8B44F !important;
        }
        
        .dashboard-sider .ant-menu-submenu-title {
          color: #9CA3AF !important;
          margin: 4px 8px !important;
          border-radius: 8px !important;
        }
        
        .dashboard-sider .ant-menu-submenu-title:hover {
          background: rgba(232, 180, 79, 0.1) !important;
          color: #E8B44F !important;
        }
        
        .dashboard-sider .ant-menu-submenu-open > .ant-menu-submenu-title {
          color: #E8B44F !important;
        }
        
        .dashboard-sider .ant-menu-sub {
          background: rgba(0, 0, 0, 0.2) !important;
        }
        
        @media (max-width: 992px) {
          .mobile-only {
            display: block !important;
          }
        }
        
        @media (min-width: 993px) {
          .mobile-only {
            display: none !important;
          }
        }
      `}</style>
    </Layout>
  );
};

export default DashboardLayout;
