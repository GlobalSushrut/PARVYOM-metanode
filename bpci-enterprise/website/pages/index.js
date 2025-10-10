import { useState, useEffect } from 'react';
import { useRouter } from 'next/router';
import Head from 'next/head';
import axios from 'axios';
import { motion } from 'framer-motion';
import { 
  ShieldCheckIcon, 
  CpuChipIcon, 
  GlobeAltIcon, 
  LockClosedIcon,
  ServerIcon,
  CommandLineIcon
} from '@heroicons/react/24/outline';

export default function BPCIHomepage() {
  const [credentials, setCredentials] = useState({ username: '', password: '' });
  const [loading, setLoading] = useState(false);
  const [bpciStatus, setBpciStatus] = useState(null);
  const router = useRouter();

  // Check BPCI server status on load
  useEffect(() => {
    checkBPCIStatus();
  }, []);

  const checkBPCIStatus = async () => {
    try {
      const response = await axios.get('/api/bpci/status');
      setBpciStatus(response.data);
    } catch (error) {
      setBpciStatus({ status: 'offline', message: 'BPCI server not responding' });
    }
  };

  const handleLogin = async (e) => {
    e.preventDefault();
    setLoading(true);
    
    try {
      const response = await axios.post('/api/auth/login', credentials);
      if (response.data.success) {
        // Redirect to HTTPCG dashboard within BPCI system
        window.location.href = response.data.httpcg_redirect;
      }
    } catch (error) {
      alert('Login failed: ' + error.response?.data?.message || 'Authentication error');
    }
    
    setLoading(false);
  };

  return (
    <>
      <Head>
        {/* Primary SEO Tags */}
        <title>BPCI - Enterprise Blockchain Operating System | Post-Quantum Web3 Platform</title>
        <meta name="description" content="BPCI is the world's first enterprise blockchain operating system with HTTPCG protocol, post-quantum cryptography, and military-grade security. Deploy BPI OS for next-generation Web3 infrastructure." />
        <meta name="keywords" content="BPCI, BPI OS, HTTPCG protocol, enterprise blockchain, post-quantum cryptography, Web3 operating system, military-grade security, blockchain infrastructure" />
        
        {/* Open Graph Tags */}
        <meta property="og:title" content="BPCI - Enterprise Blockchain Operating System | Post-Quantum Web3 Platform" />
        <meta property="og:description" content="Revolutionary enterprise blockchain OS with HTTPCG protocol, post-quantum security, and military-grade encryption. The future of Web3 infrastructure." />
        <meta property="og:type" content="website" />
        <meta property="og:url" content="https://pravyom.com" />
        <meta property="og:image" content="https://pravyom.com/images/bpci-og-image.jpg" />
        
        {/* Twitter Cards */}
        <meta name="twitter:card" content="summary_large_image" />
        <meta name="twitter:title" content="BPCI - Enterprise Blockchain Operating System" />
        <meta name="twitter:description" content="Revolutionary enterprise blockchain OS with HTTPCG protocol and post-quantum security." />
        <meta name="twitter:image" content="https://pravyom.com/images/bpci-twitter-card.jpg" />
        
        {/* Technical SEO */}
        <meta name="robots" content="index, follow, max-image-preview:large, max-snippet:-1, max-video-preview:-1" />
        <meta name="googlebot" content="index, follow" />
        <link rel="canonical" href="https://pravyom.com" />
        
        {/* Schema.org Structured Data */}
        <script
          type="application/ld+json"
          dangerouslySetInnerHTML={{
            __html: JSON.stringify({
              "@context": "https://schema.org",
              "@type": "SoftwareApplication",
              "name": "BPCI",
              "description": "Enterprise blockchain operating system with HTTPCG protocol and post-quantum cryptography",
              "applicationCategory": "Operating System",
              "operatingSystem": "Linux, Enterprise",
              "offers": {
                "@type": "Offer",
                "price": "0",
                "priceCurrency": "USD"
              },
              "creator": {
                "@type": "Organization",
                "name": "Pravyom",
                "url": "https://pravyom.com"
              },
              "featureList": [
                "HTTPCG Protocol",
                "Post-Quantum Cryptography", 
                "Military-Grade Security",
                "Enterprise Blockchain Infrastructure",
                "BPI Operating System",
                "XTMP Communication Protocol"
              ]
            })
          }}
        />
      </Head>

      <div className="min-h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900">
        {/* Header */}
        <header className="relative z-10 px-6 py-4">
          <nav className="flex items-center justify-between">
            <div className="flex items-center space-x-2">
              <ServerIcon className="h-8 w-8 text-blue-400" />
              <span className="text-2xl font-bold text-white">BPCI</span>
              <span className="text-sm text-gray-400">Enterprise</span>
            </div>
            <div className="flex items-center space-x-4">
              <div className={`flex items-center space-x-2 px-3 py-1 rounded-full text-xs ${
                bpciStatus?.status === 'online' 
                  ? 'bg-green-900 text-green-300' 
                  : 'bg-red-900 text-red-300'
              }`}>
                <div className={`w-2 h-2 rounded-full ${
                  bpciStatus?.status === 'online' ? 'bg-green-400' : 'bg-red-400'
                }`}></div>
                <span>BPCI Server {bpciStatus?.status || 'Unknown'}</span>
              </div>
            </div>
          </nav>
        </header>

        {/* Main Content */}
        <main className="relative z-10 px-6 py-12">
          <div className="max-w-7xl mx-auto">
            <div className="grid lg:grid-cols-2 gap-12 items-center">
              
              {/* Left Side - Hero Content */}
              <motion.div
                initial={{ opacity: 0, x: -50 }}
                animate={{ opacity: 1, x: 0 }}
                transition={{ duration: 0.8 }}
                className="space-y-8"
              >
                <div className="space-y-4">
                  <h1 className="text-5xl lg:text-6xl font-bold text-white leading-tight">
                    BPCI
                    <span className="block text-transparent bg-clip-text bg-gradient-to-r from-blue-400 to-purple-400">
                      Enterprise
                    </span>
                    <span className="block text-3xl lg:text-4xl text-gray-300">
                      Blockchain Operating System
                    </span>
                  </h1>
                  
                  <p className="text-xl text-gray-300 leading-relaxed">
                    Revolutionary <strong className="text-white">next-generation internet infrastructure</strong> with 
                    <strong className="text-blue-400">OS-level blockchain integration</strong>, 
                    <strong className="text-purple-400">IBFT consensus (937+ components)</strong>, 
                    <strong className="text-green-400">HERMES-Lite Web-4 mesh networking</strong>, and 
                    <strong className="text-yellow-400">enterprise-grade orchestration</strong>.
                  </p>
                </div>

                {/* Feature Highlights */}
                <div className="grid grid-cols-2 gap-4">
                  <div className="flex items-center space-x-3 p-4 bg-white/5 rounded-lg backdrop-blur-sm">
                    <ServerIcon className="h-6 w-6 text-blue-400" />
                    <div>
                      <div className="text-white font-semibold">BPI Service Orchestrator</div>
                      <div className="text-gray-400 text-sm">One-click infrastructure</div>
                    </div>
                  </div>
                  
                  <div className="flex items-center space-x-3 p-4 bg-white/5 rounded-lg backdrop-blur-sm">
                    <CpuChipIcon className="h-6 w-6 text-purple-400" />
                    <div>
                      <div className="text-white font-semibold">NXOS DRX Integration</div>
                      <div className="text-gray-400 text-sm">Immutable OS control</div>
                    </div>
                  </div>
                  
                  <div className="flex items-center space-x-3 p-4 bg-white/5 rounded-lg backdrop-blur-sm">
                    <ShieldCheckIcon className="h-6 w-6 text-green-400" />
                    <div>
                      <div className="text-white font-semibold">IBFT Consensus</div>
                      <div className="text-gray-400 text-sm">937+ sophisticated components</div>
                    </div>
                  </div>
                  
                  <div className="flex items-center space-x-3 p-4 bg-white/5 rounded-lg backdrop-blur-sm">
                    <GlobeAltIcon className="h-6 w-6 text-yellow-400" />
                    <div>
                      <div className="text-white font-semibold">HERMES-Lite Web-4</div>
                      <div className="text-gray-400 text-sm">κ-aware mesh networking</div>
                    </div>
                  </div>
                </div>

                {/* Real Architecture Components */}
                <div className="space-y-4">
                  <h3 className="text-lg font-semibold text-white">Revolutionary Architecture:</h3>
                  <div className="space-y-2 text-sm text-gray-300">
                    <div className="flex items-center space-x-2">
                      <div className="w-2 h-2 bg-blue-400 rounded-full"></div>
                      <span><strong className="text-blue-400">BPI Service Orchestrator:</strong> Master infrastructure controller with one-click deployment</span>
                    </div>
                    <div className="flex items-center space-x-2">
                      <div className="w-2 h-2 bg-purple-400 rounded-full"></div>
                      <span><strong className="text-purple-400">NXOS DRX Integration:</strong> Immutable OS controls complete infrastructure</span>
                    </div>
                    <div className="flex items-center space-x-2">
                      <div className="w-2 h-2 bg-green-400 rounded-full"></div>
                      <span><strong className="text-green-400">IBFT Consensus:</strong> 937+ sophisticated components with Byzantine fault tolerance</span>
                    </div>
                    <div className="flex items-center space-x-2">
                      <div className="w-2 h-2 bg-yellow-400 rounded-full"></div>
                      <span><strong className="text-yellow-400">HERMES-Lite Web-4:</strong> κ-aware mesh networking with living nodes</span>
                    </div>
                    <div className="flex items-center space-x-2">
                      <div className="w-2 h-2 bg-red-400 rounded-full"></div>
                      <span><strong className="text-red-400">Company-Hosted XTMP:</strong> Multi-protocol servers (REST/WS/gRPC)</span>
                    </div>
                  </div>
                </div>

                {/* Enterprise Deployment */}
                <div className="space-y-3">
                  <h3 className="text-lg font-semibold text-white">Enterprise Deployment:</h3>
                  <div className="flex flex-wrap gap-2">
                    {[
                      'Government Compliance',
                      'Authority Levels', 
                      'Real Billing Systems',
                      'Wallet Registry',
                      'Service Orchestration'
                    ].map((useCase) => (
                      <span key={useCase} className="px-3 py-1 bg-blue-900/30 text-blue-300 rounded-full text-sm">
                        {useCase}
                      </span>
                    ))}
                  </div>
                </div>
              </motion.div>

              {/* Right Side - Login Form */}
              <motion.div
                initial={{ opacity: 0, x: 50 }}
                animate={{ opacity: 1, x: 0 }}
                transition={{ duration: 0.8, delay: 0.2 }}
                className="lg:max-w-md mx-auto w-full"
              >
                <div className="bg-white/10 backdrop-blur-lg p-8 rounded-2xl shadow-2xl border border-white/20">
                  <div className="text-center mb-8">
                    <CommandLineIcon className="h-12 w-12 text-blue-400 mx-auto mb-4" />
                    <h2 className="text-2xl font-bold text-white">Access BPCI System</h2>
                    <p className="text-gray-300 mt-2">Enterprise Admin Dashboard</p>
                  </div>
                  
                  <form onSubmit={handleLogin} className="space-y-6">
                    <div>
                      <label className="block text-sm font-medium text-gray-300 mb-2">
                        Username
                      </label>
                      <input
                        type="text"
                        value={credentials.username}
                        onChange={(e) => setCredentials({...credentials, username: e.target.value})}
                        className="w-full px-4 py-3 bg-white/10 border border-white/20 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent backdrop-blur-sm"
                        placeholder="Enter username"
                        required
                      />
                    </div>
                    
                    <div>
                      <label className="block text-sm font-medium text-gray-300 mb-2">
                        Password
                      </label>
                      <input
                        type="password"
                        value={credentials.password}
                        onChange={(e) => setCredentials({...credentials, password: e.target.value})}
                        className="w-full px-4 py-3 bg-white/10 border border-white/20 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent backdrop-blur-sm"
                        placeholder="Enter password"
                        required
                      />
                    </div>
                    
                    <button
                      type="submit"
                      disabled={loading}
                      className="w-full bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700 text-white font-semibold py-3 px-6 rounded-lg transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed shadow-lg"
                    >
                      {loading ? (
                        <div className="flex items-center justify-center space-x-2">
                          <div className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
                          <span>Authenticating...</span>
                        </div>
                      ) : (
                        'Access HTTPCG Dashboard'
                      )}
                    </button>
                  </form>
                  
                  <div className="mt-6 text-center">
                    <div className="text-sm text-gray-400 bg-white/5 rounded-lg p-3">
                      <p className="font-semibold text-gray-300 mb-1">Demo Credentials:</p>
                      <p><strong className="text-blue-400">root</strong> / <strong className="text-purple-400">admin</strong></p>
                    </div>
                  </div>
                  
                  <div className="mt-6 text-center">
                    <div className="text-xs text-gray-400 space-y-1">
                      <p className="flex items-center justify-center space-x-2">
                        <GlobeAltIcon className="h-4 w-4 text-blue-400" />
                        <span>HTTPCG Protocol Active</span>
                      </p>
                      <p className="flex items-center justify-center space-x-2">
                        <ShieldCheckIcon className="h-4 w-4 text-purple-400" />
                        <span>Post-Quantum Security</span>
                      </p>
                      <p className="flex items-center justify-center space-x-2">
                        <LockClosedIcon className="h-4 w-4 text-green-400" />
                        <span>Military-Grade Encryption</span>
                      </p>
                    </div>
                  </div>
                </div>
              </motion.div>
            </div>
          </div>
        </main>

        {/* Background Effects */}
        <div className="absolute inset-0 overflow-hidden">
          <div className="absolute -top-40 -right-40 w-80 h-80 bg-purple-500 rounded-full mix-blend-multiply filter blur-xl opacity-20 animate-pulse"></div>
          <div className="absolute -bottom-40 -left-40 w-80 h-80 bg-blue-500 rounded-full mix-blend-multiply filter blur-xl opacity-20 animate-pulse"></div>
        </div>
      </div>
    </>
  );
}
