import React, { useState, useEffect } from 'react';
import { Card, Button, Modal, Spin, Typography, Tag, Tooltip } from 'antd';
import { 
  ZoomInOutlined, 
  ZoomOutOutlined,
  FullscreenOutlined,
  DownloadOutlined,
  ExperimentOutlined,
  ThunderboltOutlined,
  RocketOutlined,
  GlobalOutlined,
  AntDesignOutlined,
  RadarChartOutlined
} from '@ant-design/icons';
import parvyomLogo from '../../assets/images/parvyom-logo.png';

const { Title, Paragraph } = Typography;

interface RevolutionaryLayer {
  id: string;
  name: string;
  description: string;
  revolutionaryAspects: string[];
  position: { y: number };
  color: string;
  height: number;
}

interface RevolutionaryComponent {
  id: string;
  name: string;
  description: string;
  revolutionaryFeatures: string[];
  technicalDetails: string;
  mathematicalFoundation?: string;
  quantumAspects?: string;
  biologicalInspiration?: string;
  temporalMechanics?: string;
  codebaseLocation: string;
  position: { x: number; y: number };
  connections: string[];
  status: 'revolutionary' | 'transcendent' | 'quantum-biological' | 'consciousness-aware';
  complexity: 'unprecedented' | 'transcendent' | 'revolutionary' | 'advanced';
}

const Diagram: React.FC = () => {
  const [selectedLayer, setSelectedLayer] = useState<RevolutionaryLayer | null>(null);
  const [layerModalVisible, setLayerModalVisible] = useState(false);
  const [zoomLevel, setZoomLevel] = useState(1);
  const [loading, setLoading] = useState(true);

  // The Real Infrastructure Layers of BPI/BPCI Architecture (based on comprehensive code analysis)
  const revolutionaryLayers: RevolutionaryLayer[] = [
    {
      id: 'bpi-service-orchestrator',
      name: 'BPI Service Orchestrator - Master Infrastructure Control',
      description: 'Revolutionary master coordinator that controls complete BPI infrastructure through one-click deployment',
      revolutionaryAspects: [
        'One-click infrastructure deployment with automatic wallet connection',
        'Dynamic NX authorization and service health monitoring',
        'Controls BPI Core Node, VM Server, Audit Pipeline, BPCI Bridge',
        'Real-time infrastructure health and performance monitoring',
        'Enterprise-grade orchestration with government compliance'
      ],
      position: { y: 50 },
      color: '#8b5cf6',
      height: 140
    },
    {
      id: 'nxos-drx-integration',
      name: 'NXOS DRX Integration - Immutable OS Control',
      description: 'Revolutionary Immutable OS that controls complete BPI infrastructure at kernel level',
      revolutionaryAspects: [
        'Sophisticated filesystem architecture with immutable storage guarantees',
        'vPod networking with trust-weighted routing algorithms',
        'QLock session steering with cryptographic forward verification',
        'Real service deployment: VM Server (7777), HTTP Cage (8888), Shadow Registry (8080), ZKLock Mobile (8081)',
        'OS-level blockchain integration with proof-of-forward verification'
      ],
      position: { y: 220 },
      color: '#ef4444',
      height: 140
    },
    {
      id: 'bpi-core-6d-blockchain',
      name: 'BPI Core 6D Blockchain & Own Consensus - 937+ Components',
      description: 'Revolutionary BPI 6D blockchain with its own unique consensus mechanism and 937+ sophisticated components',
      revolutionaryAspects: [
        'BPI 6D blockchain architecture with multi-dimensional block structure',
        'BPI own consensus mechanism (not IBFT) with advanced validator management',
        'Sophisticated block proposal pipeline with 6D validation and consensus integration',
        'Enterprise-grade security: Ed25519, BLS signatures, Merkle trees, tamper-evident audit trails',
        'Government compliance integration with authority levels and regulatory support'
      ],
      position: { y: 390 },
      color: '#10b981',
      height: 140
    },
    {
      id: 'bpci-lccd-consensus-mesh',
      name: 'BPCI LCCD Consensus & Company-Hosted XTMP Mesh',
      description: 'Revolutionary BPCI with its own LCCD consensus, community interface bridge, and company-hosted XTMP servers',
      revolutionaryAspects: [
        'BPCI LCCD consensus mechanism (separate from BPI) for community coordination',
        'Government & Community auction systems with LCCD consensus integration',
        'Company-hosted XTMP servers: REST/WebSocket/gRPC multi-protocol support',
        'HERMES-Lite Web-4 mesh with κ-aware routing and living nodes',
        'API gateway for external blockchain integration with LCCD consensus validation'
      ],
      position: { y: 560 },
      color: '#3b82f6',
      height: 140
    },
    {
      id: 'wallet-registry-registration',
      name: 'Wallet Registry & Comprehensive Registration System',
      description: 'Revolutionary authentication and registration system with government compliance and enterprise integration',
      revolutionaryAspects: [
        'Cryptographically stamped wallets with multi-signature support and HSM integration',
        'Multi-level government authority systems with compliance verification and regulatory integration',
        'Real billing & charging systems with service usage tracking and token-based billing',
        'Multi-type registration: nodes, validators, services, enterprise accounts',
        'Token address generation and activation with multi-token support and cross-chain integration'
      ],
      position: { y: 730 },
      color: '#f59e0b',
      height: 140
    },
    {
      id: 'enterprise-deployment-architecture',
      name: 'Enterprise Deployment & Real Service Architecture',
      description: 'Revolutionary enterprise-grade deployment with real services and professional infrastructure management',
      revolutionaryAspects: [
        'Real service deployment on specific ports: VM Server (7777), HTTP Cage (8888), Shadow Registry (8080), ZKLock Mobile (8081)',
        'Enterprise account management with professional service orchestration and monitoring',
        'Government compliance integration with authority levels, audit trails, and regulatory support',
        'Company-level deployment requiring organizational resources and network approval',
        'Next-generation internet infrastructure combining OS integration, consensus, networking, and security'
      ],
      position: { y: 900 },
      color: '#ec4899',
      height: 140
    }
  ];

  const handleLayerClick = (layer: RevolutionaryLayer) => {
    setSelectedLayer(layer);
    setLayerModalVisible(true);
  };

  useEffect(() => {
    const timer = setTimeout(() => {
      setLoading(false);
    }, 1500);
    return () => clearTimeout(timer);
  }, []);

  if (loading) {
    return (
      <div style={{ 
        minHeight: '100vh', 
        display: 'flex', 
        alignItems: 'center', 
        justifyContent: 'center',
        background: 'linear-gradient(135deg, #0f172a 0%, #1e293b 100%)'
      }}>
        <div style={{ textAlign: 'center', color: 'white' }}>
          <Spin size="large" />
          <div style={{ marginTop: '20px', fontSize: '18px' }}>
            Loading Pravyom Revolutionary Architecture...
          </div>
          <div style={{ marginTop: '10px', fontSize: '14px', opacity: 0.7 }}>
            The Most Sophisticated System Ever Created by Humanity
          </div>
        </div>
      </div>
    );
  }

  return (
    <div style={{ 
      minHeight: '100vh',
      background: 'linear-gradient(135deg, #0f172a 0%, #1e293b 100%)',
      padding: '20px',
      paddingTop: '100px',
      color: 'white',
      overflow: 'hidden'
    }}>
      <div style={{ 
        maxWidth: '1200px', 
        margin: '0 auto',
        width: '100%',
        boxSizing: 'border-box'
      }}>
        {/* Professional Header with Logo */}
        <div style={{ 
          background: 'rgba(255, 255, 255, 0.05)', 
          backdropFilter: 'blur(10px)',
          border: '1px solid rgba(255, 255, 255, 0.1)',
          borderRadius: '20px',
          padding: '30px',
          marginBottom: '40px',
          textAlign: 'center'
        }}>
          {/* Logo and Brand */}
          <div style={{ 
            display: 'flex', 
            alignItems: 'center', 
            justifyContent: 'center', 
            marginBottom: '20px',
            flexWrap: 'wrap',
            gap: '20px'
          }}>
            <img 
              src={parvyomLogo} 
              alt="Parvyom Logo" 
              style={{ 
                height: '60px', 
                width: 'auto',
                filter: 'brightness(1.1) contrast(1.1)'
              }} 
            />
            <div style={{ textAlign: 'left' }}>
              <Title level={1} style={{ 
                color: 'white', 
                margin: '0', 
                fontSize: 'clamp(24px, 4vw, 36px)',
                fontWeight: '700'
              }}>
                BPI/BPCI Infrastructure
              </Title>
              <div style={{ 
                color: '#60a5fa', 
                fontSize: 'clamp(14px, 2vw, 18px)',
                fontWeight: '500',
                marginTop: '4px'
              }}>
                Next-Generation Internet Architecture
              </div>
            </div>
          </div>

          {/* Key Features */}
          <div style={{ 
            display: 'grid', 
            gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))',
            gap: '15px',
            marginBottom: '20px'
          }}>
            <div style={{ 
              background: 'rgba(59, 130, 246, 0.1)', 
              border: '1px solid rgba(59, 130, 246, 0.3)',
              borderRadius: '10px',
              padding: '12px',
              textAlign: 'center'
            }}>
              <div style={{ color: '#60a5fa', fontSize: '14px', fontWeight: '600' }}>
                BPI 6D Blockchain
              </div>
              <div style={{ color: '#e2e8f0', fontSize: '12px' }}>
                Own Consensus
              </div>
            </div>
            <div style={{ 
              background: 'rgba(16, 185, 129, 0.1)', 
              border: '1px solid rgba(16, 185, 129, 0.3)',
              borderRadius: '10px',
              padding: '12px',
              textAlign: 'center'
            }}>
              <div style={{ color: '#10b981', fontSize: '14px', fontWeight: '600' }}>
                BPCI LCCD Consensus
              </div>
              <div style={{ color: '#e2e8f0', fontSize: '12px' }}>
                Community Bridge
              </div>
            </div>
            <div style={{ 
              background: 'rgba(139, 92, 246, 0.1)', 
              border: '1px solid rgba(139, 92, 246, 0.3)',
              borderRadius: '10px',
              padding: '12px',
              textAlign: 'center'
            }}>
              <div style={{ color: '#8b5cf6', fontSize: '14px', fontWeight: '600' }}>
                HERMES-Lite Web-4
              </div>
              <div style={{ color: '#e2e8f0', fontSize: '12px' }}>
                κ-aware Mesh
              </div>
            </div>
          </div>

          {/* Description */}
          <Paragraph style={{ 
            color: '#cbd5e1', 
            fontSize: 'clamp(14px, 2vw, 16px)', 
            maxWidth: '800px', 
            margin: '0 auto',
            lineHeight: '1.6'
          }}>
            Interactive architecture diagram showcasing the revolutionary BPI/BPCI infrastructure with 
            OS-level blockchain integration, enterprise-grade orchestration, and advanced mesh networking.
          </Paragraph>
        </div>

        {/* Revolutionary Features Banner */}
        <Card style={{ 
          background: 'rgba(239, 68, 68, 0.1)', 
          border: '2px solid #ef4444',
          marginBottom: '30px',
          backdropFilter: 'blur(10px)'
        }}>
          <div style={{ textAlign: 'center' }}>
            <Title level={3} style={{ color: '#ef4444', marginBottom: '20px' }}>
              🚀 Revolutionary Innovations 🚀
            </Title>
            <div style={{ display: 'flex', flexWrap: 'wrap', gap: '12px', justifyContent: 'center' }}>
              <Tag color="#ef4444" style={{ fontSize: '14px', padding: '8px 16px' }}>
                <ThunderboltOutlined /> LCCD Consciousness Intelligence
              </Tag>
              <Tag color="#8b5cf6" style={{ fontSize: '14px', padding: '8px 16px' }}>
                <AntDesignOutlined /> Mathematical Transcendence (Gödel)
              </Tag>
              <Tag color="#10b981" style={{ fontSize: '14px', padding: '8px 16px' }}>
                <ExperimentOutlined /> Quantum-Biological Integration
              </Tag>
              <Tag color="#3b82f6" style={{ fontSize: '14px', padding: '8px 16px' }}>
                <RadarChartOutlined /> Temporal Protection System
              </Tag>
              <Tag color="#f59e0b" style={{ fontSize: '14px', padding: '8px 16px' }}>
                <RocketOutlined /> Container Escape Engine
              </Tag>
              <Tag color="#06b6d4" style={{ fontSize: '14px', padding: '8px 16px' }}>
                <GlobalOutlined /> HERMES-Lite Web-4 Mesh
              </Tag>
            </div>
          </div>
        </Card>

        {/* Controls */}
        <div style={{ 
          display: 'flex', 
          justifyContent: 'center', 
          gap: '12px', 
          marginBottom: '30px',
          flexWrap: 'wrap'
        }}>
          <Button 
            icon={<ZoomInOutlined />} 
            onClick={() => setZoomLevel(prev => Math.min(prev + 0.2, 2))}
            style={{ background: 'rgba(59, 130, 246, 0.2)', border: '1px solid #3b82f6', color: 'white' }}
          >
            Zoom In
          </Button>
          <Button 
            icon={<ZoomOutOutlined />} 
            onClick={() => setZoomLevel(prev => Math.max(prev - 0.2, 0.5))}
            style={{ background: 'rgba(59, 130, 246, 0.2)', border: '1px solid #3b82f6', color: 'white' }}
          >
            Zoom Out
          </Button>
          <Button 
            icon={<FullscreenOutlined />} 
            onClick={() => setZoomLevel(1)}
            style={{ background: 'rgba(59, 130, 246, 0.2)', border: '1px solid #3b82f6', color: 'white' }}
          >
            Reset View
          </Button>
          <Button 
            icon={<DownloadOutlined />} 
            style={{ background: 'rgba(34, 197, 94, 0.2)', border: '1px solid #22c55e', color: 'white' }}
          >
            Export Diagram
          </Button>
        </div>

        {/* The Six Revolutionary Infrastructure Layers Visualization */}
        <Card style={{ 
          background: 'rgba(30, 41, 59, 0.5)', 
          border: '1px solid rgba(59, 130, 246, 0.3)',
          minHeight: '1200px',
          position: 'relative',
          overflow: 'auto'
        }}>
          <div 
            style={{ 
              transform: `scale(${zoomLevel})`,
              transformOrigin: 'center top',
              transition: 'transform 0.3s ease',
              position: 'relative',
              width: '100%',
              minHeight: '1100px'
            }}
          >
            {/* Revolutionary Layers */}
            {revolutionaryLayers.map((layer, index) => (
              <Tooltip 
                key={layer.id}
                title={`Click to explore ${layer.name}`}
                placement="right"
              >
                <div
                  onClick={() => handleLayerClick(layer)}
                  style={{
                    position: 'absolute',
                    left: '50px',
                    top: `${layer.position.y}px`,
                    right: '50px',
                    height: `${layer.height}px`,
                    background: `linear-gradient(135deg, ${layer.color}22 0%, ${layer.color}44 100%)`,
                    border: `3px solid ${layer.color}`,
                    borderRadius: '16px',
                    cursor: 'pointer',
                    transition: 'all 0.3s ease',
                    backdropFilter: 'blur(10px)',
                    boxShadow: `0 8px 32px ${layer.color}33`,
                    display: 'flex',
                    flexDirection: 'column',
                    justifyContent: 'center',
                    alignItems: 'center',
                    padding: '20px'
                  }}
                  onMouseEnter={(e) => {
                    e.currentTarget.style.transform = 'scale(1.02)';
                    e.currentTarget.style.boxShadow = `0 12px 48px ${layer.color}66`;
                  }}
                  onMouseLeave={(e) => {
                    e.currentTarget.style.transform = 'scale(1)';
                    e.currentTarget.style.boxShadow = `0 8px 32px ${layer.color}33`;
                  }}
                >
                  <div style={{ 
                    fontSize: '24px', 
                    fontWeight: 'bold', 
                    color: 'white',
                    textAlign: 'center',
                    marginBottom: '12px',
                    textShadow: '0 2px 4px rgba(0,0,0,0.5)'
                  }}>
                    Layer {index + 1}: {layer.name}
                  </div>
                  <div style={{ 
                    fontSize: '16px', 
                    color: '#e2e8f0',
                    textAlign: 'center',
                    marginBottom: '16px',
                    lineHeight: '1.4'
                  }}>
                    {layer.description}
                  </div>
                  <div style={{ display: 'flex', flexWrap: 'wrap', gap: '8px', justifyContent: 'center' }}>
                    {layer.revolutionaryAspects.slice(0, 3).map((aspect, i) => (
                      <Tag 
                        key={i}
                        color={layer.color}
                        style={{ fontSize: '12px', margin: '2px' }}
                      >
                        {aspect.length > 30 ? aspect.substring(0, 30) + '...' : aspect}
                      </Tag>
                    ))}
                    {layer.revolutionaryAspects.length > 3 && (
                      <Tag color={layer.color} style={{ fontSize: '12px', margin: '2px' }}>
                        +{layer.revolutionaryAspects.length - 3} more
                      </Tag>
                    )}
                  </div>
                </div>
              </Tooltip>
            ))}

            {/* Connection Lines between layers */}
            <svg 
              style={{ 
                position: 'absolute', 
                top: 0, 
                left: 0, 
                width: '100%', 
                height: '100%',
                pointerEvents: 'none',
                zIndex: 1
              }}
            >
              {revolutionaryLayers.slice(0, -1).map((layer, index) => {
                const nextLayer = revolutionaryLayers[index + 1];
                const startY = layer.position.y + layer.height;
                const endY = nextLayer.position.y;
                const centerX = '50%';
                
                return (
                  <line
                    key={`connection-${layer.id}-${nextLayer.id}`}
                    x1={centerX}
                    y1={startY}
                    x2={centerX}
                    y2={endY}
                    stroke="rgba(59, 130, 246, 0.6)"
                    strokeWidth="4"
                    strokeDasharray="10,5"
                    style={{
                      filter: 'drop-shadow(0 0 8px rgba(59, 130, 246, 0.4))'
                    }}
                  />
                );
              })}
            </svg>
          </div>
        </Card>

        {/* Layer Detail Modal */}
        <Modal
          title={
            <div style={{ color: 'white', fontSize: '24px' }}>
              🌟 {selectedLayer?.name} 🌟
            </div>
          }
          open={layerModalVisible}
          onCancel={() => setLayerModalVisible(false)}
          footer={[
            <Button 
              key="close" 
              onClick={() => setLayerModalVisible(false)}
              style={{ background: 'rgba(59, 130, 246, 0.2)', border: '1px solid #3b82f6', color: 'white' }}
            >
              Close
            </Button>
          ]}
          width={800}
          style={{
            background: 'rgba(15, 23, 42, 0.95)',
          }}
          bodyStyle={{
            background: 'linear-gradient(135deg, #0f172a 0%, #1e293b 100%)',
            color: 'white',
            padding: '30px'
          }}
        >
          {selectedLayer && (
            <div>
              <div style={{ 
                background: `linear-gradient(135deg, ${selectedLayer.color}22 0%, ${selectedLayer.color}44 100%)`,
                border: `2px solid ${selectedLayer.color}`,
                borderRadius: '12px',
                padding: '20px',
                marginBottom: '24px'
              }}>
                <Title level={3} style={{ color: 'white', marginBottom: '16px' }}>
                  Revolutionary Description
                </Title>
                <Paragraph style={{ color: '#e2e8f0', fontSize: '16px', lineHeight: '1.6' }}>
                  {selectedLayer.description}
                </Paragraph>
              </div>

              <Title level={3} style={{ color: 'white', marginBottom: '16px' }}>
                🚀 Revolutionary Aspects
              </Title>
              <div style={{ marginBottom: '24px' }}>
                {selectedLayer.revolutionaryAspects.map((aspect, index) => (
                  <div 
                    key={index}
                    style={{
                      background: 'rgba(30, 41, 59, 0.5)',
                      border: '1px solid rgba(59, 130, 246, 0.3)',
                      borderRadius: '8px',
                      padding: '16px',
                      marginBottom: '12px',
                      borderLeft: `4px solid ${selectedLayer.color}`
                    }}
                  >
                    <div style={{ 
                      color: 'white', 
                      fontSize: '16px', 
                      fontWeight: 'bold',
                      marginBottom: '8px'
                    }}>
                      {aspect.split(' - ')[0]}
                    </div>
                    {aspect.includes(' - ') && (
                      <div style={{ color: '#e2e8f0', fontSize: '14px' }}>
                        {aspect.split(' - ')[1]}
                      </div>
                    )}
                  </div>
                ))}
              </div>

              <div style={{
                background: 'rgba(239, 68, 68, 0.1)',
                border: '2px solid #ef4444',
                borderRadius: '12px',
                padding: '20px',
                textAlign: 'center'
              }}>
                <Title level={4} style={{ color: '#ef4444', marginBottom: '12px' }}>
                  🔬 Technical Implementation Status
                </Title>
                <Paragraph style={{ color: '#e2e8f0', fontSize: '14px', marginBottom: '16px' }}>
                  This layer represents cutting-edge research and development in quantum-biological blockchain systems.
                  Implementation requires deep understanding of consciousness intelligence, mathematical transcendence, and cellular computing.
                </Paragraph>
                <Tag color="#ef4444" style={{ fontSize: '12px', padding: '6px 12px' }}>
                  Revolutionary Technology
                </Tag>
                <Tag color="#8b5cf6" style={{ fontSize: '12px', padding: '6px 12px' }}>
                  Transcendent Architecture
                </Tag>
                <Tag color="#10b981" style={{ fontSize: '12px', padding: '6px 12px' }}>
                  Production Implementation
                </Tag>
              </div>
            </div>
          )}
        </Modal>

        {/* Educational Footer */}
        <div style={{ 
          marginTop: '60px', 
          textAlign: 'center',
          background: 'rgba(30, 41, 59, 0.3)',
          borderRadius: '16px',
          padding: '40px',
          border: '1px solid rgba(59, 130, 246, 0.2)'
        }}>
          <Title level={3} style={{ color: 'white', marginBottom: '20px' }}>
            🎓 Understanding the Revolutionary Architecture
          </Title>
          <Paragraph style={{ color: '#e2e8f0', fontSize: '16px', maxWidth: '800px', margin: '0 auto 20px' }}>
            This diagram represents the most sophisticated technological system ever created by humanity. 
            Each layer integrates revolutionary concepts that transcend current technological limitations, 
            combining consciousness intelligence, quantum mechanics, biological inspiration, and mathematical transcendence.
          </Paragraph>
          <div style={{ display: 'flex', flexWrap: 'wrap', gap: '12px', justifyContent: 'center' }}>
            <Tag color="#ef4444" style={{ fontSize: '14px', padding: '8px 16px' }}>
              🧠 Consciousness Intelligence
            </Tag>
            <Tag color="#8b5cf6" style={{ fontSize: '14px', padding: '8px 16px' }}>
              🔬 Quantum-Biological Integration
            </Tag>
            <Tag color="#10b981" style={{ fontSize: '14px', padding: '8px 16px' }}>
              ⚡ Mathematical Transcendence
            </Tag>
            <Tag color="#3b82f6" style={{ fontSize: '14px', padding: '8px 16px' }}>
              🚀 Temporal Protection
            </Tag>
            <Tag color="#f59e0b" style={{ fontSize: '14px', padding: '8px 16px' }}>
              🌱 Cellular Division Scaling
            </Tag>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Diagram;
