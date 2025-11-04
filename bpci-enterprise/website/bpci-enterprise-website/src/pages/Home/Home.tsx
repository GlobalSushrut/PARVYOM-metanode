import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Button } from 'antd';
import { 
  RocketOutlined,
  TeamOutlined
} from '@ant-design/icons';
import Features from '../../components/Features/Features';
import CommunityPopup from '../../components/CommunityPopup/CommunityPopup';
import './Home.css';

const Home: React.FC = () => {
  const navigate = useNavigate();
  const [showCommunityPopup, setShowCommunityPopup] = useState(false);

  return (
    <div className="home-page">
      {/* Hero Section */}
      <section className="hero-section">
        <div className="hero-container">
          <div className="pilot-warning">
            🔬 <strong>EXPERIMENTAL TESTNET</strong> • 75% Infrastructure Ready
          </div>
          
          <h1 className="hero-title">
            Research Platform for Future Internet
          </h1>
          
          <p className="hero-subtitle">
            Exploring distributed operating systems and blockchain infrastructure. 
            15 services operational, needs testing & pilot partnerships. 
            <strong>6-12 months to pilot-ready, 1-2 years to mainnet.</strong>
          </p>
          
          <div className="status-info">
            <strong>Current Status:</strong> 75% ready • Needs testing & pilots • Single-engineer research project
          </div>
          
          <div className="hero-stats">
            <div className="stat-card">
              <div className="stat-number emerald">75%</div>
              <div className="stat-label">Infrastructure Ready</div>
            </div>
            <div className="stat-card">
              <div className="stat-number amber">15</div>
              <div className="stat-label">Services Operational</div>
            </div>
            <div className="stat-card">
              <div className="stat-number purple">Testing</div>
              <div className="stat-label">Phase (Pilots Needed)</div>
            </div>
          </div>
          
          <div className="hero-actions">
            <Button 
              type="primary" 
              size="large" 
              onClick={() => navigate('/research')}
              className="btn-primary-hero"
              icon={<RocketOutlined />}
            >
              Explore Research
            </Button>
            <Button 
              size="large" 
              onClick={() => navigate('/about')}
              className="btn-community-hero"
              icon={<TeamOutlined />}
            >
              Join Pilot Program
            </Button>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <Features />

      {/* Community Popup */}
      {showCommunityPopup && (
        <CommunityPopup 
          visible={showCommunityPopup}
          onClose={() => setShowCommunityPopup(false)}
        />
      )}
    </div>
  );
};

export default Home;
