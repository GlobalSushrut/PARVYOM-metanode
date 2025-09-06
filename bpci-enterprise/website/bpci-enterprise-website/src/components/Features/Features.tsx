import React, { useState } from 'react';
import { 
  SecurityScanOutlined, 
  DatabaseOutlined, 
  ThunderboltOutlined,
  BankOutlined,
  GlobalOutlined,
  RocketOutlined
} from '@ant-design/icons';

const Features: React.FC = () => {
  const [hoveredFeature, setHoveredFeature] = useState<number | null>(null);

  const features = [
    {
      icon: <SecurityScanOutlined />,
      title: 'Post-Quantum Ready Security',
      description: 'Advanced cryptographic algorithms (Ed25519, Blake3) with upgrade path for quantum resistance, providing strong security for your blockchain infrastructure.',
      color: '#3b82f6',
      gradient: 'linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%)'
    },
    {
      icon: <DatabaseOutlined />,
      title: 'Enterprise Registry',
      description: 'Comprehensive node registry system with RBAC authentication, supporting multiple registry types including community, enterprise, and government APIs.',
      color: '#10b981',
      gradient: 'linear-gradient(135deg, #10b981 0%, #059669 100%)'
    },
    {
      icon: <ThunderboltOutlined />,
      title: 'Real-Time Monitoring',
      description: 'Live dashboard with real-time metrics, system health monitoring, and economic tracking for comprehensive infrastructure visibility.',
      color: '#f59e0b',
      gradient: 'linear-gradient(135deg, #f59e0b 0%, #d97706 100%)'
    },
    {
      icon: <BankOutlined />,
      title: 'Banking Integration (Planned)',
      description: 'Planned integration with traditional banking systems through secure APIs, enabling hybrid financial infrastructure solutions in future releases.',
      color: '#8b5cf6',
      gradient: 'linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%)'
    },
    {
      icon: <GlobalOutlined />,
      title: 'Decentralized Network',
      description: 'Distributed blockchain network with validator nodes, consensus mechanisms, and economic systems designed for decentralization.',
      color: '#06b6d4',
      gradient: 'linear-gradient(135deg, #06b6d4 0%, #0891b2 100%)'
    },
    {
      icon: <RocketOutlined />,
      title: 'Scalable Architecture',
      description: 'Built for scalability with high-performance Rust backend, efficient consensus algorithms, and optimized network protocols.',
      color: '#ef4444',
      gradient: 'linear-gradient(135deg, #ef4444 0%, #dc2626 100%)'
    }
  ];

  return (
    <section style={{
      position: 'relative',
      padding: '120px 0',
      background: 'linear-gradient(135deg, #0f172a 0%, #1e293b 100%)',
      overflow: 'hidden'
    }}>
      {/* Background Pattern */}
      <div style={{
        position: 'absolute',
        top: 0,
        left: 0,
        right: 0,
        bottom: 0,
        backgroundImage: `
          radial-gradient(circle at 25% 25%, rgba(59, 130, 246, 0.1) 0%, transparent 50%),
          radial-gradient(circle at 75% 75%, rgba(139, 92, 246, 0.1) 0%, transparent 50%)
        `,
        pointerEvents: 'none'
      }} />

      <div style={{
        position: 'relative',
        zIndex: 2,
        maxWidth: '1200px',
        margin: '0 auto',
        padding: '0 24px'
      }}>
        <div style={{
          textAlign: 'center',
          marginBottom: '80px'
        }}>
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
            🏗️ Infrastructure Features
          </div>
          <h2 style={{
            fontSize: '3rem',
            fontWeight: '800',
            color: 'white',
            marginBottom: '24px',
            background: 'linear-gradient(135deg, #ffffff 0%, #3b82f6 100%)',
            WebkitBackgroundClip: 'text',
            WebkitTextFillColor: 'transparent',
            backgroundClip: 'text'
          }}>
            Enterprise-Grade Features
          </h2>
          <p style={{
            fontSize: '1.25rem',
            color: '#94a3b8',
            lineHeight: '1.6',
            maxWidth: '600px',
            margin: '0 auto'
          }}>
            Discover the powerful capabilities that make PARVYOM BPCI the leading choice 
            for enterprise blockchain infrastructure and autonomous economic systems.
          </p>
        </div>

        <div style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fit, minmax(350px, 1fr))',
          gap: '32px'
        }}>
          {features.map((feature, index) => (
            <div 
              key={index}
              style={{
                background: 'rgba(255, 255, 255, 0.05)',
                border: '1px solid rgba(255, 255, 255, 0.1)',
                borderRadius: '16px',
                padding: '32px',
                backdropFilter: 'blur(10px)',
                transition: 'all 0.3s ease',
                cursor: 'pointer',
                transform: hoveredFeature === index ? 'translateY(-8px)' : 'translateY(0)',
                boxShadow: hoveredFeature === index 
                  ? `0 20px 40px rgba(${feature.color.replace('#', '').match(/.{2}/g)?.map(x => parseInt(x, 16)).join(', ')}, 0.3)` 
                  : '0 8px 32px rgba(0, 0, 0, 0.1)'
              }}
              onMouseEnter={() => setHoveredFeature(index)}
              onMouseLeave={() => setHoveredFeature(null)}
            >
              <div style={{
                width: '64px',
                height: '64px',
                background: feature.gradient,
                borderRadius: '16px',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                marginBottom: '24px',
                fontSize: '1.5rem',
                color: 'white',
                boxShadow: `0 8px 24px rgba(${feature.color.replace('#', '').match(/.{2}/g)?.map(x => parseInt(x, 16)).join(', ')}, 0.3)`
              }}>
                {feature.icon}
              </div>
              <h3 style={{
                color: 'white',
                fontSize: '1.25rem',
                fontWeight: '700',
                marginBottom: '16px'
              }}>
                {feature.title}
              </h3>
              <p style={{
                color: '#94a3b8',
                lineHeight: '1.6',
                fontSize: '14px'
              }}>
                {feature.description}
              </p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
};

export default Features;
