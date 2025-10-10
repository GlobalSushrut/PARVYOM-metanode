import bcrypt from 'bcryptjs';
import jwt from 'jsonwebtoken';

// Demo credentials for BPCI enterprise system
const BPCI_ADMIN_USERS = {
  'root': {
    password: '$2a$10$92IXUNpkjO0rOQ5byMi.Ye4oKoEa3Ro9llC/.og/at2.uheWG/igi', // password: admin
    role: 'admin',
    permissions: ['bpci_admin', 'httpcg_access', 'system_control'],
    name: 'BPCI Root Administrator'
  },
  'admin': {
    password: '$2a$10$92IXUNpkjO0rOQ5byMi.Ye4oKoEa3Ro9llC/.og/at2.uheWG/igi', // password: admin
    role: 'admin',
    permissions: ['bpci_admin', 'httpcg_access'],
    name: 'BPCI Administrator'
  }
};

export default async function handler(req, res) {
  if (req.method !== 'POST') {
    return res.status(405).json({ message: 'Method not allowed' });
  }

  const { username, password } = req.body;

  // Validate input
  if (!username || !password) {
    return res.status(400).json({ message: 'Username and password required' });
  }

  // Check BPCI server status first
  try {
    const bpciHealthCheck = await fetch('http://localhost:8888/health');
    if (!bpciHealthCheck.ok) {
      return res.status(503).json({ 
        message: 'BPCI server unavailable',
        bpci_status: 'offline'
      });
    }
  } catch (error) {
    console.warn('BPCI server health check failed:', error.message);
    // Continue with authentication even if health check fails (for demo)
  }

  // Validate credentials
  const user = BPCI_ADMIN_USERS[username];
  if (!user || !bcrypt.compareSync(password, user.password)) {
    return res.status(401).json({ message: 'Invalid credentials' });
  }

  // Generate JWT token with BPCI-specific claims
  const token = jwt.sign(
    { 
      username, 
      role: user.role,
      permissions: user.permissions,
      name: user.name,
      bpci_access: true,
      iat: Math.floor(Date.now() / 1000),
      exp: Math.floor(Date.now() / 1000) + (24 * 60 * 60) // 24 hours
    },
    process.env.JWT_SECRET || 'bpci-enterprise-secret-key',
    { 
      issuer: 'bpci-enterprise',
      audience: 'bpci-admin-dashboard'
    }
  );

  // Generate HTTPCG redirect URL to BPCI admin dashboard
  const bpciServerUrl = process.env.BPCI_SERVER_URL || 'http://localhost:8888';
  const httpcg_redirect = `${bpciServerUrl}/httpcg/dashboard?token=${token}&user=${username}`;

  // Log successful authentication
  console.log(`✅ BPCI Admin Login: ${username} (${user.name}) at ${new Date().toISOString()}`);

  res.status(200).json({
    success: true,
    token,
    httpcg_redirect,
    user: {
      username,
      name: user.name,
      role: user.role,
      permissions: user.permissions
    },
    bpci_access: true,
    message: 'Authentication successful - redirecting to HTTPCG dashboard'
  });
}
