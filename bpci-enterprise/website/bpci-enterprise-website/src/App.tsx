import { useState } from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { ConfigProvider } from 'antd';
import MainLayout from './layouts/MainLayout';
import Home from './pages/Home/Home';
import About from './pages/About/About';
import Technology from './pages/Technology/Technology';
import AdminDashboard from './pages/Dashboard/AdminDashboard';
import Enterprise from './pages/Enterprise/Enterprise';
import Community from './pages/Community/Community';
import Blog from './pages/Blog/Blog';
import GetStarted from './pages/GetStarted/GetStarted';
import Contact from './pages/Contact/Contact';
import PrivacyPolicy from './pages/PrivacyPolicy/PrivacyPolicy';
import TermsOfService from './pages/TermsOfService/TermsOfService';
import Legal from './pages/Legal/Legal';
import Research from './pages/Research/Research';
import UnifiedAuthContainer from './components/Auth/UnifiedAuthContainer';
import PravyomAuthUI from './components/Auth/PravyomAuthUI';
import { useUnifiedAuth } from './hooks/useUnifiedAuth';
import RegistryDashboard from './pages/RegistryDashboard';
import { WalletManager } from './components/Wallet/WalletManager';
import MojoDashboard from './pages/MojoDashboard';
import TestOTP from './pages/TestOTP';
import BasicDashboard from './pages/BasicDashboard';
import SystemDashboard from './pages/SystemDashboard';
import Wallet from './pages/Wallet';
import Transactions from './pages/Transactions';
import WalletSettings from './pages/WalletSettings';
import BlogCreate from './pages/BlogCreate';
import Profile from './pages/Profile';
import Settings from './pages/Settings';
import Security from './pages/Security';
import ProofPage from './pages/ProofPage';
import DocumentationManager from './pages/DocumentationManager';

// BPCI Enterprise Theme Configuration
const theme = {
  token: {
    colorPrimary: '#667eea',
    colorSuccess: '#059669',
    colorWarning: '#ea580c',
    colorError: '#dc2626',
    fontFamily: 'Inter, -apple-system, BlinkMacSystemFont, sans-serif',
    borderRadius: 8,
  },
  components: {
    Button: {
      borderRadius: 8,
      fontWeight: 500,
    },
    Card: {
      borderRadius: 12,
    },
  },
};

function App() {
  const { isAuthenticated, user } = useUnifiedAuth();
  const [legacyAuth, setLegacyAuth] = useState(false);

  const handleAuthSuccess = (user?: any) => {
    setLegacyAuth(true);
  };

  // Use unified auth state or fallback to legacy auth
  const authState = isAuthenticated || legacyAuth;

  return (
    <ConfigProvider theme={theme}>
      <Router>
        <MainLayout isAuthenticated={authState} onAuthSuccess={handleAuthSuccess}>
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/about" element={<About />} />
            <Route path="/technology" element={<Technology />} />
            <Route path="/login" element={<PravyomAuthUI onAuthSuccess={handleAuthSuccess} />} />
            <Route path="/auth" element={<PravyomAuthUI onAuthSuccess={handleAuthSuccess} />} />
            <Route path="/basic-dashboard" element={authState ? <BasicDashboard /> : <PravyomAuthUI onAuthSuccess={handleAuthSuccess} />} />
            <Route path="/dashboard" element={isAuthenticated ? <SystemDashboard /> : <PravyomAuthUI onAuthSuccess={handleAuthSuccess} />} />
            <Route path="/registry" element={isAuthenticated ? <RegistryDashboard /> : <PravyomAuthUI onAuthSuccess={handleAuthSuccess} />} />
            <Route path="/wallet" element={isAuthenticated ? <Wallet /> : <PravyomAuthUI onAuthSuccess={handleAuthSuccess} />} />
            <Route path="/wallet/transactions" element={isAuthenticated ? <Transactions /> : <PravyomAuthUI onAuthSuccess={handleAuthSuccess} />} />
            <Route path="/wallet/settings" element={isAuthenticated ? <WalletSettings /> : <PravyomAuthUI onAuthSuccess={handleAuthSuccess} />} />
            <Route path="/profile" element={isAuthenticated ? <Profile /> : <PravyomAuthUI onAuthSuccess={handleAuthSuccess} />} />
            <Route path="/settings" element={isAuthenticated ? <Settings /> : <PravyomAuthUI onAuthSuccess={handleAuthSuccess} />} />
            <Route path="/security" element={isAuthenticated ? <Security /> : <PravyomAuthUI onAuthSuccess={handleAuthSuccess} />} />
            <Route path="/mojo-dashboard" element={isAuthenticated ? <MojoDashboard /> : <PravyomAuthUI onAuthSuccess={handleAuthSuccess} />} />
            <Route path="/enterprise" element={<Enterprise />} />
            <Route path="/community" element={<Community />} />
            <Route path="/blog" element={<Blog />} />
            <Route path="/blog/create" element={isAuthenticated ? <BlogCreate /> : <PravyomAuthUI onAuthSuccess={handleAuthSuccess} />} />
            <Route path="/get-started" element={<GetStarted />} />
            <Route path="/contact" element={<Contact />} />
            <Route path="/privacy-policy" element={<PrivacyPolicy />} />
            <Route path="/terms-of-service" element={<TermsOfService />} />
            <Route path="/legal" element={<Legal />} />
            <Route path="/research" element={<Research />} />
            <Route path="/proof" element={<ProofPage />} />
            <Route path="/documentation" element={isAuthenticated ? <DocumentationManager /> : <PravyomAuthUI onAuthSuccess={handleAuthSuccess} />} />
            <Route path="/test-otp" element={<TestOTP />} />
          </Routes>
        </MainLayout>
      </Router>
    </ConfigProvider>
  );
}

export default App;
