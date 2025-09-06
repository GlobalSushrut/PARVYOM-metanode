import { useState } from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { ConfigProvider } from 'antd';
import MainLayout from './layouts/MainLayout';
import Home from './pages/Home/Home';
import About from './pages/About/About';
import Technology from './pages/Technology/Technology';
import Dashboard from './pages/Dashboard/Dashboard';
import Enterprise from './pages/Enterprise/Enterprise';
import Community from './pages/Community/Community';
import Blog from './pages/Blog/Blog';
import GetStarted from './pages/GetStarted/GetStarted';
import AuthContainer from './components/Auth/AuthContainer';
import { RegistryDashboard } from './components/Registry/RegistryDashboard';
import { WalletManager } from './components/Wallet/WalletManager';
import { BPIInstaller } from './components/Installer/BPIInstaller';

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
  const [isAuthenticated, setIsAuthenticated] = useState(false);

  const handleAuthSuccess = () => {
    setIsAuthenticated(true);
  };

  return (
    <ConfigProvider theme={theme}>
      <Router>
        <MainLayout isAuthenticated={isAuthenticated} onAuthSuccess={handleAuthSuccess}>
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/about" element={<About />} />
            <Route path="/technology" element={<Technology />} />
            <Route path="/login" element={<AuthContainer onAuthSuccess={handleAuthSuccess} />} />
            <Route path="/dashboard" element={isAuthenticated ? <Dashboard /> : <AuthContainer onAuthSuccess={handleAuthSuccess} />} />
            <Route path="/registry" element={isAuthenticated ? <RegistryDashboard /> : <AuthContainer onAuthSuccess={handleAuthSuccess} />} />
            <Route path="/wallet" element={isAuthenticated ? <WalletManager /> : <AuthContainer onAuthSuccess={handleAuthSuccess} />} />
            <Route path="/installer" element={isAuthenticated ? <BPIInstaller /> : <AuthContainer onAuthSuccess={handleAuthSuccess} />} />
            <Route path="/enterprise" element={<Enterprise />} />
            <Route path="/community" element={<Community />} />
            <Route path="/blog" element={<Blog />} />
            <Route path="/get-started" element={<GetStarted />} />
          </Routes>
        </MainLayout>
      </Router>
    </ConfigProvider>
  );
}

export default App;
