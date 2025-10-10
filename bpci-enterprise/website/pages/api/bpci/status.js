// BPCI Server Status API - Integrated within BPCI Enterprise System

export default async function handler(req, res) {
  if (req.method !== 'GET') {
    return res.status(405).json({ message: 'Method not allowed' });
  }

  try {
    // Check BPCI server components
    const bpciServerUrl = process.env.BPCI_SERVER_URL || 'http://localhost:8888';
    const xtmpServerUrl = process.env.XTMP_SERVER_URL || 'http://localhost:9999';
    const vmServerUrl = process.env.VM_SERVER_URL || 'http://localhost:7777';
    const walletServerUrl = process.env.WALLET_SERVER_URL || 'http://localhost:7778';

    const statusChecks = await Promise.allSettled([
      // BPCI Admin Dashboard
      fetch(`${bpciServerUrl}/health`, { timeout: 5000 }).then(r => r.ok),
      
      // XTMP Server
      fetch(`${xtmpServerUrl}/health`, { timeout: 5000 }).then(r => r.ok),
      
      // VM Server (HTTPCG)
      fetch(`${vmServerUrl}/health`, { timeout: 5000 }).then(r => r.ok),
      
      // Wallet Server
      fetch(`${walletServerUrl}/health`, { timeout: 5000 }).then(r => r.ok)
    ]);

    const [adminDashboard, xtmpServer, vmServer, walletServer] = statusChecks;

    const systemStatus = {
      status: 'online',
      timestamp: new Date().toISOString(),
      components: {
        admin_dashboard: {
          status: adminDashboard.status === 'fulfilled' && adminDashboard.value ? 'online' : 'offline',
          url: bpciServerUrl,
          port: 8888,
          description: 'BPCI Admin Dashboard'
        },
        xtmp_server: {
          status: xtmpServer.status === 'fulfilled' && xtmpServer.value ? 'online' : 'offline',
          url: xtmpServerUrl,
          port: 9999,
          description: 'XTMP Communication Server'
        },
        vm_server: {
          status: vmServer.status === 'fulfilled' && vmServer.value ? 'online' : 'offline',
          url: vmServerUrl,
          port: 7777,
          description: 'VM Server (HTTPCG Protocol)'
        },
        wallet_server: {
          status: walletServer.status === 'fulfilled' && walletServer.value ? 'online' : 'offline',
          url: walletServerUrl,
          port: 7778,
          description: 'HTTPCG Wallet Server'
        }
      },
      system_info: {
        bpci_version: '1.0.0',
        httpcg_protocol: 'active',
        post_quantum_security: 'enabled',
        military_grade_encryption: 'active',
        demo_mode: true
      }
    };

    // Determine overall system status
    const onlineComponents = Object.values(systemStatus.components)
      .filter(component => component.status === 'online').length;
    
    if (onlineComponents === 0) {
      systemStatus.status = 'offline';
      systemStatus.message = 'All BPCI components offline';
    } else if (onlineComponents < 4) {
      systemStatus.status = 'partial';
      systemStatus.message = `${onlineComponents}/4 BPCI components online`;
    } else {
      systemStatus.status = 'online';
      systemStatus.message = 'All BPCI components operational';
    }

    res.status(200).json(systemStatus);

  } catch (error) {
    console.error('BPCI status check error:', error);
    
    res.status(500).json({
      status: 'error',
      message: 'Failed to check BPCI system status',
      timestamp: new Date().toISOString(),
      error: error.message,
      components: {
        admin_dashboard: { status: 'unknown' },
        xtmp_server: { status: 'unknown' },
        vm_server: { status: 'unknown' },
        wallet_server: { status: 'unknown' }
      }
    });
  }
}
