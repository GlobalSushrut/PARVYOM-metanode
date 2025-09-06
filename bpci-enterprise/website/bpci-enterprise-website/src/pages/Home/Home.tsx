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
            ⚠️ <strong>PILOT PROGRAM</strong> - Pre-funding Required
          </div>
          
          <h1 className="hero-title">
            BPCI Enterprise Blockchain Infrastructure
          </h1>
          
          <p className="hero-subtitle">
            Experimental testnet for enterprise partners exploring decentralized consensus, 
            auction-based transaction processing, and community-driven blockchain governance.
          </p>
          
          <div className="status-info">
            <strong>Current Status:</strong> R&D Phase - Not ready for mainnet production
          </div>
          
          <div className="hero-stats">
            <div className="stat-card">
              <div className="stat-number amber">~50</div>
              <div className="stat-label">Testnet Transactions</div>
            </div>
            <div className="stat-card">
              <div className="stat-number emerald">3</div>
              <div className="stat-label">Partner Pilots</div>
            </div>
            <div className="stat-card">
              <div className="stat-number purple">15+</div>
              <div className="stat-label">Core Components</div>
            </div>
          </div>
          
          <div className="hero-actions">
            <Button 
              type="primary" 
              size="large" 
              onClick={() => navigate('/about')}
              className="btn-primary-hero"
              icon={<RocketOutlined />}
            >
              Learn About Partnership
            </Button>
            <Button 
              size="large" 
              onClick={() => setShowCommunityPopup(true)}
              className="btn-community-hero"
              icon={<TeamOutlined />}
            >
              🗳️ Join Community & Vote
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
