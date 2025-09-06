import React, { useState } from 'react';
import { Layout, Menu, Button, Drawer } from 'antd';
import { Link, useLocation } from 'react-router-dom';
import { MenuOutlined, CloseOutlined } from '@ant-design/icons';

const { Header, Content, Footer } = Layout;

interface MainLayoutProps {
  children: React.ReactNode;
}

const MainLayout: React.FC<MainLayoutProps> = ({ children }) => {
  const [mobileMenuVisible, setMobileMenuVisible] = useState(false);
  const location = useLocation();

  const menuItems = [
    { key: '/', label: 'Home', path: '/' },
    { key: '/about', label: 'About', path: '/about' },
    { key: '/technology', label: 'Technology', path: '/technology' },
    { key: '/dashboard', label: 'Dashboard', path: '/dashboard' },
    { key: '/enterprise', label: 'Enterprise', path: '/enterprise' },
    { key: '/community', label: 'Community', path: '/community' },
    { key: '/blog', label: 'Blog', path: '/blog' },
  ];

  const renderMenu = (mode: 'horizontal' | 'vertical' = 'horizontal') => {
    if (mode === 'horizontal') {
      return (
        <>
          {menuItems.map(item => (
            <Link 
              key={item.key} 
              to={item.path}
              className={location.pathname === item.path ? 'active' : ''}
            >
              {item.label}
            </Link>
          ))}
        </>
      );
    }
    return (
      <Menu
        mode={mode}
        selectedKeys={[location.pathname]}
        className="mobile-menu"
        items={menuItems.map(item => ({
          key: item.key,
          label: <Link to={item.path}>{item.label}</Link>,
        }))}
      />
    );
  };

  return (
    <Layout style={{ minHeight: '100vh' }}>
      {/* Header */}
      <Header className="nav-header" style={{ height: '72px' }}>
        <div className="container" style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', height: '100%' }}>
          {/* Logo */}
          <Link to="/" style={{ display: 'flex', alignItems: 'center', gap: '12px', textDecoration: 'none' }}>
            <img 
              src="/src/assets/images/parvyom-logo.png" 
              alt="PARVYOM" 
              style={{ height: '40px', width: 'auto' }}
            />
            <div style={{ display: 'flex', flexDirection: 'column' }}>
              <span style={{ color: 'white', fontWeight: '700', fontSize: '1.5rem', lineHeight: '1.2' }}>PARVYOM</span>
              <span style={{ color: '#cbd5e1', fontSize: '0.875rem', fontWeight: '500' }}>BPCI Enterprise</span>
            </div>
          </Link>

          {/* Desktop Navigation */}
          <div className="desktop-nav">
            <div className="nav-menu">
              {renderMenu()}
            </div>
            <Link to="/get-started" style={{ marginLeft: '24px' }}>
              <button className="btn-primary">
                Get Started
              </button>
            </Link>
          </div>

          {/* Mobile Menu Button */}
          <Button
            type="text"
            icon={<MenuOutlined />}
            onClick={() => setMobileMenuVisible(true)}
            className="mobile-menu-btn"
            style={{ color: 'white' }}
          />
        </div>
      </Header>

      {/* Mobile Menu Drawer */}
      <Drawer
        title={
          <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
            <img 
              src="/src/assets/images/parvyom-logo.png" 
              alt="PARVYOM" 
              style={{ height: '24px', width: 'auto' }}
            />
            <span style={{ fontWeight: 'bold', fontSize: '1.125rem' }}>BPCI Enterprise</span>
          </div>
        }
        placement="right"
        onClose={() => setMobileMenuVisible(false)}
        open={mobileMenuVisible}
        closeIcon={<CloseOutlined />}
        width={280}
      >
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
          <div style={{ flex: 1 }}>
            {renderMenu('vertical')}
          </div>
          <div style={{ paddingTop: '16px', borderTop: '1px solid #e5e7eb' }}>
            <Link to="/get-started" onClick={() => setMobileMenuVisible(false)}>
              <Button 
                type="primary" 
                size="large" 
                block
                className="btn-primary"
                style={{ fontWeight: '600' }}
              >
                Get Started
              </Button>
            </Link>
          </div>
        </div>
      </Drawer>

      {/* Main Content */}
      <Content style={{ flex: 1, minHeight: 'calc(100vh - 72px)' }}>
        {children}
      </Content>

      {/* Footer */}
      <Footer style={{ backgroundColor: '#0f172a', color: 'white' }}>
        <div className="container">
          <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(250px, 1fr))', gap: '32px', padding: '48px 0' }}>
            {/* Company Info */}
            <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                <img 
                  src="/src/assets/images/parvyom-logo.png" 
                  alt="PARVYOM" 
                  style={{ height: '24px', width: 'auto' }}
                />
                <span style={{ fontWeight: 'bold', fontSize: '1.125rem' }}>BPCI Enterprise</span>
              </div>
              <p style={{ color: '#d1d5db', fontSize: '0.875rem', lineHeight: '1.5' }}>
                Building the future of post-observation secure Internet infrastructure 
                with enterprise-grade blockchain solutions.
              </p>
            </div>

            {/* Quick Links */}
            <div>
              <h3 style={{ fontWeight: '600', fontSize: '1.125rem', marginBottom: '16px' }}>Quick Links</h3>
              <ul style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                <li><Link to="/about" style={{ color: '#d1d5db', textDecoration: 'none' }}>About</Link></li>
                <li><Link to="/technology" style={{ color: '#d1d5db', textDecoration: 'none' }}>Technology</Link></li>
                <li><Link to="/enterprise" style={{ color: '#d1d5db', textDecoration: 'none' }}>Enterprise</Link></li>
                <li><Link to="/community" style={{ color: '#d1d5db', textDecoration: 'none' }}>Community</Link></li>
              </ul>
            </div>

            {/* Resources */}
            <div>
              <h3 style={{ fontWeight: '600', fontSize: '1.125rem', marginBottom: '16px' }}>Resources</h3>
              <ul style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                <li><Link to="/dashboard" style={{ color: '#d1d5db', textDecoration: 'none' }}>Dashboard</Link></li>
                <li><Link to="/blog" style={{ color: '#d1d5db', textDecoration: 'none' }}>Blog</Link></li>
                <li><a href="#" style={{ color: '#d1d5db', textDecoration: 'none' }}>Documentation</a></li>
                <li><a href="#" style={{ color: '#d1d5db', textDecoration: 'none' }}>Support</a></li>
              </ul>
            </div>

            {/* Contact */}
            <div>
              <h3 style={{ fontWeight: '600', fontSize: '1.125rem', marginBottom: '16px' }}>Contact</h3>
              <ul style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                <li><a href="#" style={{ color: '#d1d5db', textDecoration: 'none' }}>Contact Us</a></li>
                <li><a href="#" style={{ color: '#d1d5db', textDecoration: 'none' }}>Privacy Policy</a></li>
                <li><a href="#" style={{ color: '#d1d5db', textDecoration: 'none' }}>Terms of Service</a></li>
              </ul>
            </div>
          </div>

          {/* Bottom Bar */}
          <div style={{ borderTop: '1px solid #374151', paddingTop: '32px', paddingBottom: '16px' }}>
            <div style={{ display: 'flex', flexDirection: 'column', justifyContent: 'space-between', alignItems: 'center', gap: '8px' }}>
              <p style={{ color: '#d1d5db', fontSize: '0.875rem' }}>
                © 2024 PARVYOM Metanode. All rights reserved.
              </p>
              <p style={{ color: '#9ca3af', fontSize: '0.75rem' }}>
                Post-Observation Secure Internet • Web 3.5 Infrastructure
              </p>
            </div>
          </div>
        </div>
      </Footer>
    </Layout>
  );
};

export default MainLayout;
