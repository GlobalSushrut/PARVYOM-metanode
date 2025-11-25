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
    { key: '/proof', label: 'Proof', path: '/proof' },
    { key: '/enterprise', label: 'Enterprise', path: '/enterprise' },
    { key: '/community', label: 'Community', path: '/community' },
    { key: '/blog', label: 'Blog', path: '/blog' },
  ];

  // Authenticated menu items (only visible when logged in)
  const authMenuItems = [
    { key: '/basic-dashboard', label: 'Dashboard', path: '/basic-dashboard' },
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
                padding: '8px 16px',
                borderRadius: '6px',
                color: '#ffffff',
                textDecoration: 'none',
                fontSize: '14px',
                fontWeight: '500',
                transition: 'all 0.2s ease',
                background: location.pathname === item.path ? 'rgba(232, 180, 79, 0.2)' : 'transparent',
                border: location.pathname === item.path ? '1px solid rgba(232, 180, 79, 0.5)' : '1px solid transparent',
                display: 'inline-block'
              }}
              onMouseEnter={(e) => {
                if (location.pathname !== item.path) {
                  e.currentTarget.style.background = 'rgba(232, 180, 79, 0.15)';
                  e.currentTarget.style.border = '1px solid rgba(232, 180, 79, 0.3)';
                  e.currentTarget.style.color = '#E8B44F';
                }
              }}
              onMouseLeave={(e) => {
                if (location.pathname !== item.path) {
                  e.currentTarget.style.background = 'transparent';
                  e.currentTarget.style.border = '1px solid transparent';
                  e.currentTarget.style.color = '#ffffff';
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
        background: 'rgba(10, 22, 40, 0.95)', /* Pravyom Navy with transparency */
        backdropFilter: 'blur(20px)',
        borderBottom: '2px solid rgba(232, 180, 79, 0.3)', /* Gold accent */
        boxShadow: '0 2px 16px rgba(0, 0, 0, 0.3)',
        height: '64px', /* Reduced height for cleaner look */
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
              width: '40px',
              height: '40px',
              background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)', /* Gold transformation gradient */
              borderRadius: '8px',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              marginRight: '12px',
              fontSize: '20px',
              fontWeight: 'bold',
              color: '#0A1628', /* Navy text on gold */
              boxShadow: '0 2px 8px rgba(232, 180, 79, 0.3)',
              border: '1px solid rgba(232, 180, 79, 0.5)'
            }}>
              P
            </div>
            <div>
              <div style={{ 
                display: 'flex',
                alignItems: 'center',
                gap: '8px',
                fontSize: '18px', 
                fontWeight: '700', 
                lineHeight: '1.2',
                color: '#FFFFFF',
                letterSpacing: '0.5px'
              }}>
                <span style={{ 
                  fontSize: '20px', 
                  color: '#E8B44F',
                  fontWeight: 'bold'
                }}>∞</span>
                PRAVYOM
              </div>
              <div style={{ 
                fontSize: '10px', 
                color: '#E8B44F', /* Gold for subtitle */
                lineHeight: '1.2',
                fontWeight: '500',
                letterSpacing: '1px'
              }}>
                RESEARCH PLATFORM
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

      {/* Background with blob pattern */}
      <div style={{
        position: 'fixed',
        top: 0,
        left: 0,
        right: 0,
        bottom: 0,
        background: `
          linear-gradient(135deg, #667eea 0%, #764ba2 100%),
          radial-gradient(circle at 20% 30%, rgba(192, 192, 192, 0.03) 0%, transparent 50%),
          radial-gradient(circle at 80% 20%, rgba(169, 169, 169, 0.03) 0%, transparent 50%),
          radial-gradient(circle at 40% 70%, rgba(211, 211, 211, 0.03) 0%, transparent 50%),
          radial-gradient(circle at 90% 60%, rgba(192, 192, 192, 0.03) 0%, transparent 50%),
          radial-gradient(circle at 10% 80%, rgba(169, 169, 169, 0.03) 0%, transparent 50%),
          radial-gradient(circle at 60% 40%, rgba(211, 211, 211, 0.03) 0%, transparent 50%),
          radial-gradient(circle at 30% 50%, rgba(192, 192, 192, 0.03) 0%, transparent 50%),
          radial-gradient(circle at 70% 80%, rgba(169, 169, 169, 0.03) 0%, transparent 50%)
        `,
        backgroundSize: '100% 100%, 800px 800px, 600px 600px, 900px 900px, 700px 700px, 1000px 1000px, 750px 750px, 850px 850px, 650px 650px',
        zIndex: 0
      }} />

      {/* Content */}
      <div style={{
        position: 'absolute',
        top: '64px',
        left: 0,
        right: 0,
        bottom: 0,
        overflow: 'auto',
        zIndex: 1
      }}>
        
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
            backgroundColor: 'rgba(10, 22, 40, 0.98)', /* Pravyom Navy */
            backdropFilter: 'blur(20px)',
            borderTop: '2px solid rgba(232, 180, 79, 0.3)', /* Gold accent */
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
                      background: 'linear-gradient(135deg, #E8B44F 0%, #FFFFFF 100%)', /* Gold gradient */
                      borderRadius: '6px',
                      display: 'flex',
                      alignItems: 'center',
                      justifyContent: 'center',
                      fontSize: '16px',
                      fontWeight: 'bold',
                      color: '#0A1628' /* Navy text on gold */
                    }}>
                      P
                    </div>
                    <span style={{ fontWeight: 'bold', fontSize: '1.125rem', color: '#ffffff' }}>Pravyom Research</span>
                  </div>
                  <p style={{ color: '#e2e8f0', fontSize: '0.875rem', lineHeight: '1.5' }}>
                    Experimental research platform exploring distributed operating systems and blockchain infrastructure. 
                    75% infrastructure ready, needs testing & pilot partnerships.
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
                    <li><Link to="/documentation" style={{ color: '#e2e8f0', textDecoration: 'none', transition: 'color 0.3s ease' }}>Documentation</Link></li>
                    <li><Link to="/research" style={{ color: '#e2e8f0', textDecoration: 'none', transition: 'color 0.3s ease' }}>Research</Link></li>
                  </ul>
                </div>

                {/* Legal & Contact */}
                <div>
                  <h3 style={{ fontWeight: '600', fontSize: '1.125rem', marginBottom: '16px', color: '#ffffff' }}>Legal & Contact</h3>
                  <ul style={{ display: 'flex', flexDirection: 'column', gap: '8px', listStyle: 'none', padding: 0, margin: 0 }}>
                    <li><Link to="/contact" style={{ color: '#e2e8f0', textDecoration: 'none', transition: 'color 0.3s ease' }}>Contact Us</Link></li>
                    <li><Link to="/privacy-policy" style={{ color: '#e2e8f0', textDecoration: 'none', transition: 'color 0.3s ease' }}>Privacy Policy</Link></li>
                    <li><Link to="/terms-of-service" style={{ color: '#e2e8f0', textDecoration: 'none', transition: 'color 0.3s ease' }}>Terms of Service</Link></li>
                    <li><Link to="/legal" style={{ color: '#e2e8f0', textDecoration: 'none', transition: 'color 0.3s ease' }}>Legal</Link></li>
                  </ul>
                </div>
              </div>

              {/* Bottom Bar */}
              <div style={{ borderTop: '1px solid rgba(232, 180, 79, 0.2)', paddingTop: '32px', paddingBottom: '16px' }}>
                <div style={{ display: 'flex', flexDirection: 'column', justifyContent: 'space-between', alignItems: 'center', gap: '16px' }}>
                  {/* Social Media Links */}
                  <div style={{ display: 'flex', gap: '1.5rem', alignItems: 'center' }}>
                    <a href="https://github.com/GlobalSushrut/PARVYOM-metanode" target="_blank" rel="noopener noreferrer" style={{ color: '#E8B44F', fontSize: '1.5rem', transition: 'color 0.3s ease' }} title="GitHub">
                      💻
                    </a>
                    <a href="https://twitter.com/pravyom" target="_blank" rel="noopener noreferrer" style={{ color: '#E8B44F', fontSize: '1.5rem', transition: 'color 0.3s ease' }} title="Twitter">
                      🐦
                    </a>
                    <a href="https://linkedin.com/company/pravyom" target="_blank" rel="noopener noreferrer" style={{ color: '#E8B44F', fontSize: '1.5rem', transition: 'color 0.3s ease' }} title="LinkedIn">
                      💼
                    </a>
                    <a href="https://discord.gg/pravyom" target="_blank" rel="noopener noreferrer" style={{ color: '#E8B44F', fontSize: '1.5rem', transition: 'color 0.3s ease' }} title="Discord">
                      💬
                    </a>
                    <a href="https://t.me/pravyom" target="_blank" rel="noopener noreferrer" style={{ color: '#E8B44F', fontSize: '1.5rem', transition: 'color 0.3s ease' }} title="Telegram">
                      ✈️
                    </a>
                    <a href="https://youtube.com/@pravyom" target="_blank" rel="noopener noreferrer" style={{ color: '#E8B44F', fontSize: '1.5rem', transition: 'color 0.3s ease' }} title="YouTube">
                      📺
                    </a>
                    <a href="mailto:umesh@pravyom.com" style={{ color: '#E8B44F', fontSize: '1.5rem', transition: 'color 0.3s ease' }} title="Email">
                      📧
                    </a>
                  </div>
                  
                  <p style={{ color: '#e2e8f0', fontSize: '0.875rem', margin: 0 }}>
                    © 2025 Pravyom Research Platform. Experimental project.
                  </p>
                  <p style={{ color: '#E8B44F', fontSize: '0.75rem', margin: 0 }}>
                    75% Infrastructure Ready • Single-Engineer Research • Needs Testing & Pilots
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
