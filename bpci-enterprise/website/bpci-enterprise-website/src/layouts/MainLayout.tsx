import React, { useState, useEffect } from 'react';
import { Layout, Menu, Button, Drawer } from 'antd';
import { Link, useLocation } from 'react-router-dom';
import { MenuOutlined, CloseOutlined } from '@ant-design/icons';


const { Header } = Layout;

interface MainLayoutProps {
  children: React.ReactNode;
  isAuthenticated?: boolean;
  onAuthSuccess?: () => void;
}

const MainLayout: React.FC<MainLayoutProps> = ({ children, isAuthenticated = false }) => {
  const [mobileMenuVisible, setMobileMenuVisible] = useState(false);
  const [isMobile, setIsMobile] = useState(false);
  const location = useLocation();

  useEffect(() => {
    const checkMobile = () => {
      setIsMobile(window.innerWidth <= 768);
    };
    
    checkMobile();
    window.addEventListener('resize', checkMobile);
    
    return () => window.removeEventListener('resize', checkMobile);
  }, []);

  // Public menu items (always visible)
  const publicMenuItems = [
    { key: '/', label: 'Home', path: '/' },
    { key: '/about', label: 'About', path: '/about' },
    { key: '/technology', label: 'Technology', path: '/technology' },
    { key: '/enterprise', label: 'Enterprise', path: '/enterprise' },
    { key: '/community', label: 'Community', path: '/community' },
    { key: '/blog', label: 'Blog', path: '/blog' },
  ];

  // Authenticated menu items (only visible when logged in)
  const authMenuItems = [
    { key: '/dashboard', label: 'Dashboard', path: '/dashboard' },
    { key: '/registry', label: 'BPI Registry', path: '/registry' },
    { key: '/wallet', label: 'Wallet Manager', path: '/wallet' },
    { key: '/installer', label: 'BPI Installer', path: '/installer' },
  ];

  const menuItems = isAuthenticated ? [...publicMenuItems, ...authMenuItems] : publicMenuItems;

  const handleLogout = () => {
    // Clear authentication state
    localStorage.removeItem('authToken');
    localStorage.removeItem('userSession');
    // Redirect to home page
    window.location.href = '/';
  };

  const renderMenu = (mode: 'horizontal' | 'vertical' = 'horizontal') => {
    if (mode === 'horizontal') {
      return (
        <>
          {menuItems.map(item => (
            <Link 
              key={item.key} 
              to={item.path}
              style={{
                padding: '10px 18px',
                borderRadius: '8px',
                color: '#ffffff',
                textDecoration: 'none',
                fontSize: '15px',
                fontWeight: '700',
                transition: 'all 0.3s ease',
                background: location.pathname === item.path ? 'rgba(34, 197, 94, 0.25)' : 'rgba(255, 255, 255, 0.15)',
                border: location.pathname === item.path ? '2px solid rgba(34, 197, 94, 0.6)' : '2px solid rgba(255, 255, 255, 0.3)',
                textShadow: '0 3px 6px rgba(0, 0, 0, 0.8)',
                display: 'inline-block',
                backdropFilter: 'blur(10px)',
                boxShadow: location.pathname === item.path ? '0 4px 12px rgba(34, 197, 94, 0.3)' : '0 2px 8px rgba(0, 0, 0, 0.2)'
              }}
              onMouseEnter={(e) => {
                if (location.pathname !== item.path) {
                  e.currentTarget.style.background = 'rgba(34, 197, 94, 0.2)';
                  e.currentTarget.style.border = '2px solid rgba(34, 197, 94, 0.5)';
                  e.currentTarget.style.color = '#ffffff';
                  e.currentTarget.style.transform = 'translateY(-1px)';
                  e.currentTarget.style.boxShadow = '0 4px 12px rgba(34, 197, 94, 0.3)';
                }
              }}
              onMouseLeave={(e) => {
                if (location.pathname !== item.path) {
                  e.currentTarget.style.background = 'rgba(255, 255, 255, 0.15)';
                  e.currentTarget.style.border = '2px solid rgba(255, 255, 255, 0.3)';
                  e.currentTarget.style.color = '#ffffff';
                  e.currentTarget.style.transform = 'translateY(0)';
                  e.currentTarget.style.boxShadow = '0 2px 8px rgba(0, 0, 0, 0.2)';
                }
              }}
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
    <Layout style={{ 
      minHeight: '100vh', 
      background: 'transparent',
      margin: 0,
      padding: 0,
      width: '100%'
    }}>
      {/* Fixed Header */}
      <Header style={{
        position: 'fixed',
        top: 0,
        left: 0,
        right: 0,
        zIndex: 1000,
        background: 'linear-gradient(135deg, rgba(15, 23, 42, 0.95) 0%, rgba(30, 41, 59, 0.95) 100%)',
        backdropFilter: 'blur(20px)',
        borderBottom: '3px solid rgba(59, 130, 246, 0.5)',
        boxShadow: '0 4px 32px rgba(0, 0, 0, 0.6), 0 2px 12px rgba(59, 130, 246, 0.3)',
        height: '80px',
        padding: 0,
        margin: 0,
        width: '100%'
      }}>
        <div style={{ 
          maxWidth: '1200px',
          margin: '0 auto',
          display: 'flex', 
          alignItems: 'center', 
          justifyContent: 'space-between', 
          height: '100%',
          padding: '0 24px'
        }}>
          {/* Logo */}
          <Link to="/" style={{ 
            display: 'flex', 
            alignItems: 'center', 
            textDecoration: 'none',
            color: 'white'
          }}>
            <div style={{
              width: '48px',
              height: '48px',
              background: 'linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%)',
              borderRadius: '12px',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              marginRight: '16px',
              fontSize: '24px',
              fontWeight: 'bold',
              color: 'white',
              boxShadow: '0 4px 12px rgba(59, 130, 246, 0.4)',
              border: '2px solid rgba(255, 255, 255, 0.1)'
            }}>
              P
            </div>
            <div>
              <div style={{ 
                fontSize: '20px', 
                fontWeight: 'bold', 
                lineHeight: '1.2',
                color: '#ffffff',
                textShadow: '0 2px 4px rgba(0, 0, 0, 0.3)'
              }}>
                PARVYOM
              </div>
              <div style={{ 
                fontSize: '12px', 
                color: '#e2e8f0',
                lineHeight: '1.2',
                textShadow: '0 1px 2px rgba(0, 0, 0, 0.2)'
              }}>
                BPCI ENTERPRISE
              </div>
            </div>
          </Link>

          {/* Desktop Navigation */}
          {!isMobile && (
            <nav style={{ 
              display: 'flex', 
              alignItems: 'center', 
              gap: '32px',
              flex: 1,
              justifyContent: 'center'
            }}>
              <div style={{
                display: 'flex',
                gap: '24px',
                alignItems: 'center'
              }}>
                {renderMenu()}
              </div>
            </nav>
          )}

          {/* Right Side - Status and Auth */}
          <div style={{ display: 'flex', alignItems: 'center', gap: '16px' }}>
            <div style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              padding: '8px 14px',
              borderRadius: '20px',
              background: isAuthenticated ? 'rgba(34, 197, 94, 0.15)' : 'rgba(239, 68, 68, 0.15)',
              border: `2px solid ${isAuthenticated ? 'rgba(34, 197, 94, 0.4)' : 'rgba(239, 68, 68, 0.4)'}`,
              backdropFilter: 'blur(10px)'
            }}>
              <div style={{
                width: '10px',
                height: '10px',
                borderRadius: '50%',
                background: isAuthenticated ? '#22c55e' : '#ef4444',
                boxShadow: `0 0 12px ${isAuthenticated ? 'rgba(34, 197, 94, 0.8)' : 'rgba(239, 68, 68, 0.8)'}`,
                animation: isAuthenticated ? 'pulse 2s infinite' : 'none'
              }} />
              <span style={{
                color: '#ffffff',
                fontSize: '13px',
                fontWeight: '700',
                textShadow: '0 2px 4px rgba(0, 0, 0, 0.8)',
                letterSpacing: '0.5px'
              }}>
                {isAuthenticated ? 'BPCI Online' : 'Offline'}
              </span>
            </div>
            <div style={{ display: 'flex', gap: '12px' }}>
              {!isAuthenticated ? (
                <>
                  <Link to="/login">
                    <button style={{
                      background: 'rgba(255, 255, 255, 0.15)',
                      border: '2px solid rgba(255, 255, 255, 0.3)',
                      borderRadius: '8px',
                      padding: '10px 18px',
                      color: '#ffffff',
                      fontSize: '14px',
                      fontWeight: '600',
                      cursor: 'pointer',
                      transition: 'all 0.3s ease',
                      backdropFilter: 'blur(10px)',
                      textShadow: '0 1px 2px rgba(0, 0, 0, 0.3)'
                    }}>
                      Login
                    </button>
                  </Link>
                  <Link to="/get-started">
                    <button style={{
                      background: 'linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%)',
                      border: '2px solid rgba(59, 130, 246, 0.5)',
                      borderRadius: '8px',
                      padding: '10px 18px',
                      color: '#ffffff',
                      fontSize: '14px',
                      fontWeight: '600',
                      cursor: 'pointer',
                      transition: 'all 0.3s ease',
                      boxShadow: '0 4px 16px rgba(59, 130, 246, 0.4)',
                      textShadow: '0 1px 2px rgba(0, 0, 0, 0.3)'
                    }}>
                      Get Started
                    </button>
                  </Link>
                    </>
                  ) : (
                    <div style={{ display: 'flex', gap: '12px' }}>
                      <Link to="/dashboard">
                        <button style={{
                          background: 'linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%)',
                          border: '2px solid rgba(59, 130, 246, 0.5)',
                          borderRadius: '8px',
                          padding: '10px 18px',
                          color: '#ffffff',
                          fontSize: '14px',
                          fontWeight: '600',
                          cursor: 'pointer',
                          transition: 'all 0.3s ease',
                          boxShadow: '0 4px 12px rgba(59, 130, 246, 0.4)',
                          textShadow: '0 1px 2px rgba(0, 0, 0, 0.3)'
                        }}>
                          Dashboard
                        </button>
                      </Link>
                      <button 
                        onClick={handleLogout}
                        style={{
                          background: 'rgba(239, 68, 68, 0.15)',
                          border: '2px solid rgba(239, 68, 68, 0.4)',
                          borderRadius: '8px',
                          padding: '10px 18px',
                          color: '#ffffff',
                          fontSize: '14px',
                          fontWeight: '600',
                          cursor: 'pointer',
                          transition: 'all 0.3s ease',
                          textShadow: '0 1px 2px rgba(0, 0, 0, 0.3)'
                        }}
                      >
                        Logout
                      </button>
                    </div>
                  )}
            </div>
          </div>

          {/* Mobile Menu Button */}
          {isMobile && (
            <Button
              type="text"
              icon={mobileMenuVisible ? <CloseOutlined /> : <MenuOutlined />}
              onClick={() => setMobileMenuVisible(!mobileMenuVisible)}
              style={{ 
                color: 'white', 
                fontSize: '18px',
                background: 'rgba(255, 255, 255, 0.1)',
                border: '1px solid rgba(255, 255, 255, 0.2)',
                borderRadius: '8px',
                width: '40px',
                height: '40px'
              }}
            />
          )}
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

      {/* Content */}
      <div style={{
        position: 'absolute',
        top: '80px',
        left: 0,
        right: 0,
        bottom: 0,
        background: 'linear-gradient(135deg, #0f172a 0%, #1e293b 30%, #334155 70%, #475569 100%)',
        overflow: 'auto',
        zIndex: 1
      }}>
        {/* Beautiful overlay for depth */}
        <div style={{
          position: 'absolute',
          top: 0,
          left: 0,
          right: 0,
          bottom: 0,
          background: 'radial-gradient(ellipse at top, rgba(59, 130, 246, 0.1) 0%, transparent 50%), radial-gradient(ellipse at bottom, rgba(124, 58, 237, 0.1) 0%, transparent 50%)',
          pointerEvents: 'none',
          zIndex: 0
        }} />
        
        <div style={{ 
          position: 'relative',
          zIndex: 1,
          minHeight: 'calc(100vh - 80px)',
          display: 'flex',
          flexDirection: 'column'
        }}>
          <div style={{ flex: 1 }}>
            {children}
          </div>
          
          {/* Footer */}
          <footer style={{ 
            backgroundColor: 'rgba(15, 23, 42, 0.95)',
            backdropFilter: 'blur(20px)',
            borderTop: '2px solid rgba(59, 130, 246, 0.3)',
            color: 'white',
            marginTop: 'auto'
          }}>
            <div style={{ maxWidth: '1200px', margin: '0 auto', padding: '0 24px' }}>
              <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(250px, 1fr))', gap: '32px', padding: '48px 0' }}>
                {/* Company Info */}
                <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
                  <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                    <div style={{
                      width: '32px',
                      height: '32px',
                      background: 'linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%)',
                      borderRadius: '8px',
                      display: 'flex',
                      alignItems: 'center',
                      justifyContent: 'center',
                      fontSize: '16px',
                      fontWeight: 'bold',
                      color: 'white'
                    }}>
                      P
                    </div>
                    <span style={{ fontWeight: 'bold', fontSize: '1.125rem', color: '#ffffff' }}>BPCI Enterprise</span>
                  </div>
                  <p style={{ color: '#e2e8f0', fontSize: '0.875rem', lineHeight: '1.5' }}>
                    Building the future of post-observation secure Internet infrastructure 
                    with enterprise-grade blockchain solutions.
                  </p>
                </div>

                {/* Quick Links */}
                <div>
                  <h3 style={{ fontWeight: '600', fontSize: '1.125rem', marginBottom: '16px', color: '#ffffff' }}>Quick Links</h3>
                  <ul style={{ display: 'flex', flexDirection: 'column', gap: '8px', listStyle: 'none', padding: 0, margin: 0 }}>
                    <li><Link to="/about" style={{ color: '#e2e8f0', textDecoration: 'none', transition: 'color 0.3s ease' }}>About</Link></li>
                    <li><Link to="/technology" style={{ color: '#e2e8f0', textDecoration: 'none', transition: 'color 0.3s ease' }}>Technology</Link></li>
                    <li><Link to="/enterprise" style={{ color: '#e2e8f0', textDecoration: 'none', transition: 'color 0.3s ease' }}>Enterprise</Link></li>
                    <li><Link to="/community" style={{ color: '#e2e8f0', textDecoration: 'none', transition: 'color 0.3s ease' }}>Community</Link></li>
                  </ul>
                </div>

                {/* Resources */}
                <div>
                  <h3 style={{ fontWeight: '600', fontSize: '1.125rem', marginBottom: '16px', color: '#ffffff' }}>Resources</h3>
                  <ul style={{ display: 'flex', flexDirection: 'column', gap: '8px', listStyle: 'none', padding: 0, margin: 0 }}>
                    <li><Link to="/dashboard" style={{ color: '#e2e8f0', textDecoration: 'none', transition: 'color 0.3s ease' }}>Dashboard</Link></li>
                    <li><Link to="/blog" style={{ color: '#e2e8f0', textDecoration: 'none', transition: 'color 0.3s ease' }}>Blog</Link></li>
                    <li><a href="#" style={{ color: '#e2e8f0', textDecoration: 'none', transition: 'color 0.3s ease' }}>Documentation</a></li>
                    <li><a href="#" style={{ color: '#e2e8f0', textDecoration: 'none', transition: 'color 0.3s ease' }}>Support</a></li>
                  </ul>
                </div>

                {/* Contact */}
                <div>
                  <h3 style={{ fontWeight: '600', fontSize: '1.125rem', marginBottom: '16px', color: '#ffffff' }}>Contact</h3>
                  <ul style={{ display: 'flex', flexDirection: 'column', gap: '8px', listStyle: 'none', padding: 0, margin: 0 }}>
                    <li><a href="#" style={{ color: '#e2e8f0', textDecoration: 'none', transition: 'color 0.3s ease' }}>Contact Us</a></li>
                    <li><a href="#" style={{ color: '#e2e8f0', textDecoration: 'none', transition: 'color 0.3s ease' }}>Privacy Policy</a></li>
                    <li><a href="#" style={{ color: '#e2e8f0', textDecoration: 'none', transition: 'color 0.3s ease' }}>Terms of Service</a></li>
                  </ul>
                </div>
              </div>

              {/* Bottom Bar */}
              <div style={{ borderTop: '1px solid rgba(59, 130, 246, 0.2)', paddingTop: '32px', paddingBottom: '16px' }}>
                <div style={{ display: 'flex', flexDirection: 'column', justifyContent: 'space-between', alignItems: 'center', gap: '8px' }}>
                  <p style={{ color: '#e2e8f0', fontSize: '0.875rem', margin: 0 }}>
                    © 2024 PARVYOM Metanode. All rights reserved.
                  </p>
                  <p style={{ color: '#94a3b8', fontSize: '0.75rem', margin: 0 }}>
                    Post-Observation Secure Internet • Web 3.5 Infrastructure
                  </p>
                </div>
              </div>
            </div>
          </footer>
        </div>
      </div>
    </Layout>
  );
};

export default MainLayout;
