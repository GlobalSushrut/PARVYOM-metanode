/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  env: {
    BPCI_DOMAIN: process.env.BPCI_DOMAIN || 'pravyom.com',
    HTTPCG_ENABLED: process.env.HTTPCG_ENABLED || 'true',
    DEMO_MODE: process.env.DEMO_MODE || 'true'
  },
  async headers() {
    return [
      {
        source: '/(.*)',
        headers: [
          {
            key: 'X-BPCI-Enterprise',
            value: 'Production'
          },
          {
            key: 'X-HTTPCG-Protocol',
            value: 'Enabled'
          },
          {
            key: 'X-Frame-Options',
            value: 'DENY'
          },
          {
            key: 'X-Content-Type-Options',
            value: 'nosniff'
          },
          {
            key: 'Referrer-Policy',
            value: 'strict-origin-when-cross-origin'
          }
        ]
      }
    ]
  },
  async redirects() {
    return [
      {
        source: '/dashboard',
        destination: 'https://admin.pravyom.com:8888/httpcg/dashboard',
        permanent: false
      },
      {
        source: '/wallet',
        destination: 'https://api.pravyom.com:7778',
        permanent: false
      }
    ]
  }
}

module.exports = nextConfig
