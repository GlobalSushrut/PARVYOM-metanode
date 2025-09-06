import React, { useState, useEffect } from 'react';
import { Button } from 'antd';
import { Link } from 'react-router-dom';
import { ArrowRightOutlined, PlayCircleOutlined, 
         DatabaseOutlined } from '@ant-design/icons';

const Hero: React.FC = () => {
  // Initial project statistics (realistic testnet values)
  const [stats, setStats] = useState({
    nodes: 3,
    transactions: 0,
    uptime: 100.0,
    wallets: 1,
    volume: 0,
    validators: 1,
    loading: false
  });
  const [isMobile, setIsMobile] = useState(false);

  useEffect(() => {
    const checkMobile = () => {
      setIsMobile(window.innerWidth <= 768);
    };
    
    checkMobile();
    window.addEventListener('resize', checkMobile);
    
    return () => window.removeEventListener('resize', checkMobile);
  }, []);

  // Fetch real-time data from backend (when available)
  const fetchRealTimeData = async () => {
    try {
      const [economyResponse, registryResponse] = await Promise.all([
        fetch('http://127.0.0.1:8081/api/economy/status'),
        fetch('http://127.0.0.1:8081/api/registry/stats')
      ]);

      if (economyResponse.ok && registryResponse.ok) {
        const economyData = await economyResponse.json();
        const registryData = await registryResponse.json();

        setStats({
          nodes: registryData.total_nodes || 3,
          transactions: economyData.total_transactions || 0,
          uptime: registryData.uptime_percentage || 100.0,
          wallets: registryData.active_wallets || 1,
          volume: economyData.total_volume || 0,
          validators: registryData.validator_count || 1,
          loading: false
        });
      }
    } catch (error) {
      // Keep initial realistic values when backend is not available
      console.log('Backend not available, showing initial project status');
    }
  };

  useEffect(() => {
    // Try to fetch real data on mount
    fetchRealTimeData();
  }, []);

  return (
    <section style={{
      position: 'relative',
      minHeight: '100vh',
      background: 'linear-gradient(135deg, #0f172a 0%, #1e293b 50%, #334155 100%)',
      overflow: 'hidden',
      display: 'flex',
      alignItems: 'center'
    }}>
      {/* Animated Background Grid */}
      <div style={{
        position: 'absolute',
        top: 0,
        left: 0,
        right: 0,
        bottom: 0,
        backgroundImage: `
          linear-gradient(rgba(59, 130, 246, 0.1) 1px, transparent 1px),
          linear-gradient(90deg, rgba(59, 130, 246, 0.1) 1px, transparent 1px)
        `,
        backgroundSize: '50px 50px',
        animation: 'gridMove 20s linear infinite',
        zIndex: 1
      }} />

      {/* Floating Particles */}
      <div style={{ position: 'absolute', top: 0, left: 0, right: 0, bottom: 0, zIndex: 1 }}>
        {[...Array(20)].map((_, i) => (
          <div
            key={i}
            style={{
              position: 'absolute',
              width: '4px',
              height: '4px',
              background: 'rgba(59, 130, 246, 0.6)',
              borderRadius: '50%',
              left: `${Math.random() * 100}%`,
              top: `${Math.random() * 100}%`,
              animation: `float ${3 + Math.random() * 4}s ease-in-out infinite`,
              animationDelay: `${Math.random() * 2}s`
            }}
          />
        ))}
      </div>

      <div style={{
        position: 'relative',
        zIndex: 2,
        maxWidth: '1200px',
        margin: '0 auto',
        padding: '0 24px',
        width: '100%'
      }}>
        <div style={{
          display: 'grid',
          gridTemplateColumns: isMobile ? '1fr' : '1fr 1fr',
          gap: isMobile ? '40px' : '80px',
          alignItems: 'center',
          minHeight: '80vh'
        }}>
          {/* Left Content */}
          <div>
            <div style={{
              display: 'inline-block',
              background: 'rgba(59, 130, 246, 0.1)',
              border: '1px solid rgba(59, 130, 246, 0.3)',
              borderRadius: '24px',
              padding: '8px 16px',
              marginBottom: '24px',
              color: '#3b82f6',
              fontSize: '14px',
              fontWeight: '500'
            }}>
              🌐 Web 5 Dimensional Civilization Internet by 2030
            </div>

            <h1 style={{
              fontSize: isMobile ? '2.5rem' : '4rem',
              fontWeight: '800',
              lineHeight: '1.1',
              marginBottom: '24px',
              background: 'linear-gradient(135deg, #ffffff 0%, #3b82f6 50%, #8b5cf6 100%)',
              WebkitBackgroundClip: 'text',
              WebkitTextFillColor: 'transparent',
              backgroundClip: 'text'
            }}>
              Universal Web3
              <br />
              <span style={{ color: '#3b82f6' }}>Infrastructure</span>
              <br />
              Platform
            </h1>

            <p style={{
              fontSize: isMobile ? '1.125rem' : '1.25rem',
              color: '#94a3b8',
              lineHeight: '1.6',
              marginBottom: '32px',
              maxWidth: '500px'
            }}>
              PARVYOM BPCI provides auditable orchestration, autocratic blockchain ledger, 
              and secure internet infrastructure for anyone who needs reliable, 
              transparent, and decentralized solutions. Evolving toward the 
              Web 5 Dimensional Civilization Internet by 2030.
            </p>

            {/* Action Buttons */}
            <div style={{
              display: 'flex',
              gap: '16px',
              flexDirection: isMobile ? 'column' : 'row',
              alignItems: isMobile ? 'stretch' : 'center',
              marginBottom: '48px'
            }}>
              <Link to="/get-started">
                <Button 
                  type="primary"
                  size={isMobile ? 'middle' : 'large'}
                  icon={<ArrowRightOutlined />}
                  style={{
                    background: 'linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%)',
                    border: 'none',
                    borderRadius: '12px',
                    height: isMobile ? '44px' : '48px',
                    fontSize: isMobile ? '14px' : '16px',
                    fontWeight: '600',
                    display: 'flex',
                    alignItems: 'center',
                    gap: '8px',
                    boxShadow: '0 4px 16px rgba(59, 130, 246, 0.3), 0 0 20px rgba(59, 130, 246, 0.2)',
                    transition: 'all 0.3s ease'
                  }}
                >
                  Get Started
                </Button>
              </Link>
              <Link to="/dashboard">
                <Button 
                  size={isMobile ? 'middle' : 'large'}
                  icon={<PlayCircleOutlined />}
                  style={{
                    background: 'rgba(255, 255, 255, 0.1)',
                    border: '1px solid rgba(255, 255, 255, 0.3)',
                    borderRadius: '12px',
                    height: isMobile ? '44px' : '48px',
                    color: 'white',
                    fontSize: isMobile ? '14px' : '16px',
                    fontWeight: '600',
                    display: 'flex',
                    alignItems: 'center',
                    gap: '8px',
                    backdropFilter: 'blur(10px)',
                    boxShadow: '0 2px 8px rgba(255, 255, 255, 0.1)',
                    transition: 'all 0.3s ease'
                  }}
                >
                  View Dashboard
                </Button>
              </Link>
            </div>

            {/* Live Stats */}
            <div style={{
              display: 'grid',
              gridTemplateColumns: isMobile ? '1fr 1fr' : 'repeat(3, 1fr)',
              gap: isMobile ? '16px' : '32px',
              marginTop: '32px',
              maxWidth: '600px'
            }}>
              <div style={{ textAlign: 'center' }}>
                <div style={{ 
                  fontSize: isMobile ? '1.25rem' : '1.75rem', 
                  fontWeight: '800', 
                  color: '#3b82f6',
                  textShadow: '0 0 20px rgba(59, 130, 246, 0.5)',
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center',
                  gap: '8px'
                }}>
                  {stats.nodes}
                </div>
                <div style={{ fontSize: '0.75rem', color: '#94a3b8', fontWeight: '600' }}>Active Nodes</div>
              </div>
              <div style={{ textAlign: 'center' }}>
                <div style={{ 
                  fontSize: isMobile ? '1.25rem' : '1.75rem', 
                  fontWeight: '800', 
                  color: '#10b981',
                  textShadow: '0 0 20px rgba(16, 185, 129, 0.5)'
                }}>
                  {stats.transactions.toLocaleString()}
                </div>
                <div style={{ fontSize: '0.75rem', color: '#94a3b8', fontWeight: '600' }}>Transactions</div>
              </div>
              <div style={{ textAlign: 'center' }}>
                <div style={{ 
                  fontSize: isMobile ? '1.25rem' : '1.75rem', 
                  fontWeight: '800', 
                  color: '#f59e0b',
                  textShadow: '0 0 20px rgba(245, 158, 11, 0.5)'
                }}>
                  {stats.uptime.toFixed(1)}%
                </div>
                <div style={{ fontSize: '0.75rem', color: '#94a3b8', fontWeight: '600' }}>Uptime</div>
              </div>
              <div style={{ textAlign: 'center' }}>
                <div style={{ 
                  fontSize: isMobile ? '1.25rem' : '1.75rem', 
                  fontWeight: '800', 
                  color: '#8b5cf6',
                  textShadow: '0 0 20px rgba(139, 92, 246, 0.5)'
                }}>
                  {stats.wallets}
                </div>
                <div style={{ fontSize: '0.75rem', color: '#94a3b8', fontWeight: '600' }}>Active Wallets</div>
              </div>
              <div style={{ textAlign: 'center' }}>
                <div style={{ 
                  fontSize: isMobile ? '1.25rem' : '1.75rem', 
                  fontWeight: '800', 
                  color: '#ef4444',
                  textShadow: '0 0 20px rgba(239, 68, 68, 0.5)'
                }}>
                  {stats.volume.toLocaleString()}
                </div>
                <div style={{ fontSize: '0.75rem', color: '#94a3b8', fontWeight: '600' }}>Daily Volume</div>
              </div>
              <div style={{ textAlign: 'center' }}>
                <div style={{ 
                  fontSize: isMobile ? '1.25rem' : '1.75rem', 
                  fontWeight: '800', 
                  color: '#06b6d4',
                  textShadow: '0 0 20px rgba(6, 182, 212, 0.5)'
                }}>
                  {stats.validators}
                </div>
                <div style={{ fontSize: '0.75rem', color: '#94a3b8', fontWeight: '600' }}>Validators</div>
              </div>
            </div>
          </div>

          {/* Right Visual - Orbital Network */}
          <div style={{
            position: 'relative',
            display: isMobile ? 'none' : 'flex',
            justifyContent: 'center',
            alignItems: 'center',
            height: '500px'
          }}>
            {/* Central Node */}
            <div style={{
              position: 'absolute',
              width: '120px',
              height: '120px',
              background: 'linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%)',
              borderRadius: '50%',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              boxShadow: '0 0 40px rgba(59, 130, 246, 0.6)',
              animation: 'pulse 2s ease-in-out infinite',
              zIndex: 3
            }}>
              <DatabaseOutlined style={{ fontSize: '48px', color: 'white' }} />
            </div>

            {/* Orbital Nodes */}
            {[0, 1, 2, 3, 4, 5].map((index) => {
              const angle = (index * 60) * (Math.PI / 180);
              const radius = 180;
              const x = Math.cos(angle) * radius;
              const y = Math.sin(angle) * radius;
              
              return (
                <div
                  key={index}
                  style={{
                    position: 'absolute',
                    width: '60px',
                    height: '60px',
                    background: 'rgba(255, 255, 255, 0.1)',
                    border: '2px solid rgba(59, 130, 246, 0.5)',
                    borderRadius: '50%',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    transform: `translate(${x}px, ${y}px)`,
                    animation: `orbit 10s linear infinite`,
                    animationDelay: `${index * 0.5}s`,
                    backdropFilter: 'blur(10px)',
                    zIndex: 2
                  }}
                >
                  <div style={{
                    width: '8px',
                    height: '8px',
                    background: '#3b82f6',
                    borderRadius: '50%',
                    boxShadow: '0 0 10px rgba(59, 130, 246, 0.8)'
                  }} />
                </div>
              );
            })}

            {/* Connection Lines */}
            <svg style={{
              position: 'absolute',
              width: '100%',
              height: '100%',
              zIndex: 1
            }}>
              {[0, 1, 2, 3, 4, 5].map((index) => {
                const angle = (index * 60) * (Math.PI / 180);
                const radius = 180;
                const x = 250 + Math.cos(angle) * radius;
                const y = 250 + Math.sin(angle) * radius;
                
                return (
                  <line
                    key={index}
                    x1="250"
                    y1="250"
                    x2={x}
                    y2={y}
                    stroke="rgba(59, 130, 246, 0.3)"
                    strokeWidth="1"
                    style={{
                      animation: `fadeInOut 3s ease-in-out infinite`,
                      animationDelay: `${index * 0.5}s`
                    }}
                  />
                );
              })}
            </svg>
          </div>
        </div>
      </div>

      {/* CSS Animations */}
      <style>{`
        @keyframes gridMove {
          0% { transform: translate(0, 0); }
          100% { transform: translate(50px, 50px); }
        }
        
        @keyframes float {
          0%, 100% { transform: translateY(0px); }
          50% { transform: translateY(-20px); }
        }
        
        @keyframes pulse {
          0%, 100% { transform: scale(1); box-shadow: 0 0 40px rgba(59, 130, 246, 0.6); }
          50% { transform: scale(1.05); box-shadow: 0 0 60px rgba(59, 130, 246, 0.8); }
        }
        
        @keyframes orbit {
          0% { transform: rotate(0deg) translateX(180px) rotate(0deg); }
          100% { transform: rotate(360deg) translateX(180px) rotate(-360deg); }
        }
        
        @keyframes fadeInOut {
          0%, 100% { opacity: 0.3; }
          50% { opacity: 0.8; }
        }
        
        @keyframes spin {
          0% { transform: rotate(0deg); }
          100% { transform: rotate(360deg); }
        }
      `}</style>
    </section>
  );
};

export default Hero;
