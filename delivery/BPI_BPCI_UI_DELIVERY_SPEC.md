# BPI & BPCI UI Delivery Specification

**Version:** 1.0  
**Date:** August 19, 2025  
**Target:** Production-ready UI deployment for both BPI (bundled) and BPCI (hosted) platforms

---

## Executive Summary

This document specifies the delivery of two distinct UI platforms:

1. **BPI UI** - Bundled with installer, runs locally, works offline/air-gapped
2. **BPCI UI** - Hosted platform with public website/explorer and authenticated console

Both platforms share core technology but have different deployment, security, and operational requirements.

---

# Part 1: BPI UI (Bundled with Installer)

## Overview

**Goal:** Ship an operations dashboard that runs locally with the BPI stack, works offline/air-gapped, and provides complete control over the local BPI node.

## Architecture

### Packaging Strategy

**Build Process:**
- SvelteKit → static SPA using `adapter-static`
- Route-level code splitting for optimal loading
- Gzip/Brotli compression for all assets
- Asset hashing for cache busting
- Inline fonts and icons (no CDN dependencies)

**Bundle Structure:**
```
bpi/
  bin/
    bpi-gateway          # Main BPI binary
    bpi-node            # Node binary
    bpi-wallet          # Wallet binary
  ui/                   # Static UI assets
    index.html          # Entry point
    assets/
      app-[hash].js     # Main application bundle
      chunk-[hash].js   # Route-specific chunks
      style-[hash].css  # Styles
      fonts/            # Embedded fonts
      icons/            # SVG icon set
  config/
    ui.json             # UI configuration
    update.json         # Update channel configuration
  docs/                 # Offline documentation
    api/                # API documentation
    guides/             # User guides
    troubleshooting/    # Help content
```

### Local Server Integration

**Embedded Static Server:**
- Built into BPI gateway (Rust/Axum implementation)
- Default binding: `127.0.0.1:8617` (configurable)
- Serves `/ui` directory and proxies API calls to `/_bpi/*`
- Strict security headers for local files

**Security Headers:**
```http
Content-Security-Policy: default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
Strict-Transport-Security: max-age=31536000; includeSubDomains
Subresource-Integrity: [hash] for all assets
```

**Launch Integration:**
- Installer creates desktop/start menu shortcuts
- Opens `http://127.0.0.1:8617/` in default browser
- Fallback to system tray icon with "Open Dashboard" option

### Update Mechanism

**Channel Configuration (`config/update.json`):**
```json
{
  "channel": "stable",
  "manifestUrl": "https://releases.bpi.org/manifest.json",
  "checkInterval": 86400,
  "autoDownload": false,
  "verifySignatures": true
}
```

**Update Flow:**
1. UI periodically checks signed release manifest
2. Shows "Update available" notification if newer version found
3. Downloads new `/ui` bundle to `ui-<version>/` directory
4. Atomically swaps symlink after verification
5. Works offline if update is skipped

**Compatibility Checking:**
- UI calls `/api/version` to get core version
- Compares against `ui.minCore`/`ui.maxCore` in manifest
- Prevents UI/core version mismatches

### Security Model

**Network Binding:**
- UI server only binds to `127.0.0.1` by default
- `--ui-public` flag allows external access (with warnings)
- No external network dependencies after installation

**Authentication:**
- All write operations require Wallet + BJWT challenge
- Local gateway validates all requests
- No third-party authentication services

**Privacy:**
- No telemetry by default
- Analytics entirely opt-in and stored locally
- No third-party scripts or tracking

### Size Targets

**Performance Budgets:**
- First load: ≤ 600 KB gzipped
- Full bundle on disk: ≤ 10 MB
- Individual route chunks: ≤ 200 KB gzipped
- Critical path resources: ≤ 100 KB gzipped

## Feature Set

### Core Pages

**1. Overview Dashboard**
- Node status and health metrics
- Network connectivity status
- Recent activity feed
- System resource usage
- Quick actions (start/stop services)

**2. Mesh Management**
- BPCI mesh topology visualization
- Peer connections and status
- Network statistics and performance
- Mesh configuration and settings

**3. DockLock Container Management**
- Running containers list and status
- Container logs and metrics
- Deploy new containers
- Container security policies
- Resource allocation and limits

**4. ENC Cluster Operations**
- Cluster node status
- Workload scheduling and management
- Service mesh configuration
- Attestation verification (Web Worker)
- Performance monitoring

**5. Audit Trail**
- Transaction history
- Policy enforcement logs
- Security events
- Compliance reports
- Export capabilities (CSV/JSON)

**6. Settings & Configuration**
- Node configuration
- Network settings
- Security policies
- Update preferences
- Backup/restore

### Offline Documentation

**Integrated Help System:**
- Complete API documentation
- User guides and tutorials
- Troubleshooting guides
- FAQ and common issues
- Searchable content (local index)

## Technical Implementation

### Frontend Stack

**Core Framework:**
- SvelteKit with TypeScript
- Tailwind CSS (purged for size)
- Component library: Custom BPI design system

**Data Visualization:**
- uPlot for performance charts
- Custom network topology visualization
- Real-time metrics dashboards

**State Management:**
- Svelte stores for application state
- SSE connections for real-time updates
- Local storage for user preferences

**Code Editing:**
- CodeMirror for YAML/JSON editing (lazy-loaded)
- Syntax highlighting for policies and configurations
- Validation and error highlighting

### Backend Integration

**API Communication:**
- RESTful API calls to local BPI gateway
- Server-Sent Events for real-time updates
- WebSocket fallback for older browsers

**Authentication Flow:**
```javascript
// Example BJWT challenge flow
const challenge = await fetch('/api/auth/challenge');
const signature = await wallet.sign(challenge.data);
const token = await fetch('/api/auth/verify', {
  method: 'POST',
  body: JSON.stringify({ challenge: challenge.id, signature })
});
```

### Build Process

**Development:**
```bash
npm run dev          # Development server
npm run build        # Production build
npm run preview      # Preview production build
npm run test         # Run test suite
npm run lint         # Lint and format
```

**CI/CD Pipeline:**
1. Typecheck and lint
2. Run test suite
3. Build production bundle
4. Check size budgets
5. Generate integrity hashes
6. Package for installer

---

# Part 2: BPCI UI (Hosted Platform)

## Overview

**Goal:** Public trust surface (Website + Explorer + Registry) and authenticated Operators Console for BPCI network management.

## Architecture

### Hosting Strategy

**Infrastructure:**
- **Public Pages:** Cloudflare Pages or Nginx
- **API Gateway:** Cloudflare/Nginx with mTLS to internal services
- **Console:** Same app with protected routes
- **CDN:** Immutable asset hashing with edge caching

**Deployment Zones:**
```
Production:
  - bpci.org (Website + Explorer + Registry)
  - console.bpci.org (Operators Console)
  - api.bpci.org (API Gateway)

Staging:
  - staging.bpci.org
  - staging-console.bpci.org
  - staging-api.bpci.org
```

### Security Model

**Content Security Policy:**
```http
Content-Security-Policy: 
  default-src 'self';
  script-src 'self' 'wasm-unsafe-eval';
  style-src 'self' 'unsafe-inline';
  img-src 'self' data: https:;
  connect-src 'self' wss: https:;
  worker-src 'self';
  frame-ancestors 'none';
```

**Additional Headers:**
- Cross-Origin-Opener-Policy: same-origin
- Cross-Origin-Embedder-Policy: require-corp
- Strict-Transport-Security: max-age=31536000
- Subresource Integrity for all assets

**Authentication:**
- Public pages: No authentication required
- Console: OIDC (PKCE) + BJWT for write operations
- API: mTLS for service-to-service communication

### Split-Origin Audit

**Trust Verification:**
```javascript
// Example split-origin audit
const rdRoot = await fetch('https://bpci.org/api/merkle-root');
const bdnRoot = await fetch('https://bdn.example.org/api/merkle-root');

if (rdRoot.hash === bdnRoot.hash) {
  showTrustBadge('Trusted');
} else {
  showTrustBadge('Divergent', { rdRoot, bdnRoot });
}
```

**Trust Sources:**
- **RD (Registry Data):** `https://bpci.org`
- **BDN (Blockchain Data Network):** DID/hash-wallet notary endpoints
- **Comparison:** Web Worker compares Merkle roots

### CI/CD Pipeline

**GitHub Actions Workflow:**
```yaml
name: BPCI UI Deploy
on:
  push:
    tags: ['v*']
  
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
      - run: npm ci
      - run: npm run typecheck
      - run: npm run test
      - run: npm run build
      - run: npm run size-budget
      - uses: actions/upload-artifact@v4
        with:
          name: ui-build
          path: build/
  
  deploy-staging:
    needs: build
    if: contains(github.ref, 'rc')
    runs-on: ubuntu-latest
    steps:
      - run: deploy-to-staging
  
  deploy-production:
    needs: build
    if: github.event_name == 'release'
    runs-on: ubuntu-latest
    steps:
      - run: deploy-to-production
```

**Size Budgets:**
- Public routes first load: ≤ 600 KB gzipped
- Console routes: ≤ 300 KB gzipped per route
- Full app: ≤ 10 MB total (≈2-3 MB gzipped)
- Lighthouse CI performance budgets enforced

## Feature Set

### Public Pages

**1. Homepage (`/`)**
- BPCI network overview
- Key metrics and statistics
- Getting started guides
- News and announcements
- Community links

**2. Explorer (`/explorer`)**
- Block explorer with search
- Transaction details and history
- Validator information and performance
- Network statistics and charts
- Real-time activity feed

**3. Registry (`/registry`)**
- Public registry of DIDs and entities
- Search and filtering capabilities
- Entity profiles and verification status
- Trust scores and reputation metrics

**4. Registry Detail (`/registry/:did`)**
- Detailed entity information
- Verification history
- Associated transactions
- Trust network visualization
- Public key information

**5. Documentation (`/docs/*`)**
- API documentation (mdsvex/MDX)
- Integration guides
- Best practices
- SDK documentation
- Searchable with Edge Worker

**6. Downloads (`/download`)**
- Signed release manifest
- BPI installer downloads
- SDK and tool downloads
- Verification instructions
- Release notes and changelog

### Console (Authenticated)

**1. Overview (`/console/overview`)**
- Personal dashboard
- Node status and health
- Recent activities
- Quick actions and shortcuts
- System notifications

**2. Validators (`/console/validators`)**
- Validator management
- Staking operations
- Performance monitoring
- Slashing history
- Key rotation tools

**3. Blocks (`/console/blocks`)**
- Block production monitoring
- Proposal history
- Consensus participation
- Performance metrics
- Troubleshooting tools

**4. Bids (`/console/bids`)**
- Bid management
- Auction participation
- Revenue tracking
- Strategy optimization
- Market analysis

**5. Registry Management (`/console/registry`)**
- Entity registration
- DID management
- Verification requests
- Trust network management
- Compliance reporting

**6. Economics (`/console/economics`)**
- Revenue and rewards
- Cost analysis
- ROI calculations
- Market insights
- Financial reporting

### Advanced Features

**Table Virtualization:**
- TanStack Virtual for large datasets
- Infinite scrolling for transaction lists
- Efficient rendering of 10k+ rows

**Proof Verification:**
- Web Workers for cryptographic operations
- Client-side verification of Merkle proofs
- Performance target: ≤ 30ms verification time

**Real-time Updates:**
- WebSocket connections for live data
- Server-Sent Events for notifications
- Optimistic UI updates

## Technical Implementation

### Frontend Stack

**Shared Technology:**
- SvelteKit with TypeScript
- Tailwind CSS with design system
- Component library: BPCI design system

**BPCI-Specific Additions:**
- mdsvex for documentation pages
- OIDC client for authentication
- Edge Worker for search and caching

**Cryptography:**
- WebCrypto API for client-side operations
- `@noble/*` libraries for advanced crypto
- Web Workers for heavy computations

### Information Architecture

**URL Structure:**
```
Public Routes:
  /                     # Homepage
  /explorer             # Block explorer
  /explorer/block/:id   # Block details
  /explorer/tx/:hash    # Transaction details
  /registry             # Registry listing
  /registry/:did        # Entity details
  /docs/*               # Documentation
  /download             # Downloads page

Console Routes (Auth Required):
  /console/overview     # Dashboard
  /console/validators   # Validator management
  /console/blocks       # Block management
  /console/bids         # Bid management
  /console/registry     # Registry management
  /console/economics    # Financial dashboard
```

**Navigation Structure:**
- Public: Header navigation with search
- Console: Sidebar navigation with breadcrumbs
- Mobile: Responsive drawer navigation

### Authentication Integration

**OIDC Flow (PKCE):**
```javascript
// Example OIDC integration
const authConfig = {
  authority: 'https://auth.bpci.org',
  client_id: 'bpci-console',
  redirect_uri: 'https://console.bpci.org/callback',
  response_type: 'code',
  scope: 'openid profile bpci:console'
};

// PKCE flow
const codeVerifier = generateCodeVerifier();
const codeChallenge = await generateCodeChallenge(codeVerifier);
const authUrl = buildAuthUrl(authConfig, codeChallenge);
```

**BJWT Integration:**
```javascript
// Write operations require BJWT
const bjwtChallenge = await fetch('/api/auth/bjwt-challenge');
const signature = await wallet.sign(bjwtChallenge.data);
const result = await fetch('/api/validators/register', {
  method: 'POST',
  headers: {
    'Authorization': `Bearer ${oidcToken}`,
    'X-BJWT-Challenge': bjwtChallenge.id,
    'X-BJWT-Signature': signature
  },
  body: JSON.stringify(validatorData)
});
```

---

# Part 3: Shared Technology Stack

## Core Technologies

**Framework:**
- SvelteKit with TypeScript
- Static site generation capabilities
- Route-level code splitting
- Progressive enhancement

**Styling:**
- Tailwind CSS with custom design system
- Purged CSS for minimal bundle size
- Dark/light theme support
- Responsive design patterns

**Data Visualization:**
- uPlot for high-performance charts
- Custom network topology visualization
- Real-time metrics dashboards
- Interactive data exploration

**Tables and Virtualization:**
- TanStack Table for complex data tables
- TanStack Virtual for large datasets
- Sorting, filtering, and pagination
- Export capabilities (CSV/JSON)

**State Management:**
- Svelte stores for application state
- Persistent storage for user preferences
- Real-time data synchronization
- Optimistic UI updates

**Web Workers:**
- Cryptographic operations
- Heavy computations
- Background data processing
- Proof verification

**Cryptography:**
- WebCrypto API for standard operations
- `@noble/*` libraries for advanced crypto
- Client-side key management
- Signature verification

**Testing:**
- Vitest for unit testing
- Playwright for end-to-end testing
- Component testing with Testing Library
- Performance testing and budgets

## BPI-Specific Add-ons

**Code Editing:**
- CodeMirror for YAML/JSON editing
- Syntax highlighting and validation
- Policy and configuration editing
- Lazy-loaded for bundle size

**Local Server:**
- Embedded static server in gateway
- Rust/Axum or Go/Fiber implementation
- Local API proxy and routing
- Security header management

**Offline Support:**
- Complete offline documentation
- Local search indexing
- No network dependencies
- Air-gapped operation support

## BPCI-Specific Add-ons

**Documentation:**
- mdsvex (MDX) for rich documentation
- Syntax highlighting for code blocks
- Interactive examples
- Searchable content

**Authentication:**
- OIDC (PKCE) client implementation
- Token management and refresh
- Session persistence
- Logout and cleanup

**Edge Computing:**
- Cloudflare Workers for search
- Explorer data caching
- Performance optimization
- Geographic distribution

---

# Part 4: Deliverables

## 1. BPI Installer Build

**Package Contents:**
- Complete UI bundle with all assets
- Embedded static server in gateway binary
- Update mechanism with signed manifest support
- Desktop integration (shortcuts, tray icon)
- Offline documentation package

**Installation Process:**
1. Extract BPI binaries and UI assets
2. Configure local server settings
3. Create desktop shortcuts
4. Initialize configuration files
5. Start services and open dashboard

## 2. BPCI Hosted Application

**Public Platform:**
- Complete website with explorer and registry
- Responsive design for all devices
- SEO optimization and performance
- CDN integration and caching
- Signed downloads page

**Console Application:**
- Authenticated operator dashboard
- Complete validator and node management
- Financial tracking and reporting
- Registry and DID management
- Real-time monitoring and alerts

## 3. Release Manifest Format

**Signed Manifest Structure:**
```json
{
  "version": "1.2.3",
  "releaseDate": "2025-08-19T00:00:00Z",
  "signature": "ed25519:...",
  "publicKey": "ed25519:...",
  "ui": {
    "version": "1.2.3",
    "minCore": "1.2.0",
    "maxCore": "1.3.0",
    "bundle": {
      "url": "https://releases.bpi.org/ui-1.2.3.tar.gz",
      "hash": "sha256:...",
      "size": 8388608
    }
  },
  "binaries": {
    "linux-x64": {
      "url": "https://releases.bpi.org/bpi-1.2.3-linux-x64.tar.gz",
      "hash": "sha256:...",
      "size": 67108864
    }
  }
}
```

**Usage:**
- BPI updater checks and downloads new versions
- BPCI downloads page displays verified releases
- Cryptographic verification of all downloads
- Version compatibility checking

## 4. Security Hardening Checklist

**Content Security Policy:**
- No inline scripts or styles (except whitelisted)
- Restricted resource loading domains
- Worker and frame restrictions
- Upgrade insecure requests

**HTTP Security Headers:**
- HSTS with includeSubDomains
- COOP/COEP for isolation
- X-Frame-Options: DENY
- X-Content-Type-Options: nosniff

**Subresource Integrity:**
- SRI hashes for all external resources
- Automatic hash generation in build
- Fallback mechanisms for failures
- Regular hash updates

**CORS Configuration:**
- Strict origin policies
- Preflight request handling
- Credential restrictions
- API endpoint protection

**Authentication Security:**
- OIDC with PKCE flow
- Secure token storage
- Automatic token refresh
- Proper logout handling

---

# Part 5: Implementation Timeline

## Week 1: BPI Foundation

**Days 1-2: Core Infrastructure**
- Set up SvelteKit project with TypeScript
- Implement build pipeline with size budgets
- Create embedded static server in gateway
- Basic routing and navigation

**Days 3-4: Core Pages**
- Overview dashboard with metrics
- Mesh management interface
- DockLock container management
- Basic SSE integration

**Days 5-7: Polish and Testing**
- Implement BJWT authentication flow
- Add offline documentation
- Set up CI/CD pipeline
- Size budget enforcement

## Week 2: BPI Advanced Features

**Days 1-3: Advanced Pages**
- ENC cluster operations interface
- Attestation verification (Web Worker)
- Audit trail and reporting
- Settings and configuration

**Days 4-5: Update System**
- Signed manifest checking
- Update download and installation
- Version compatibility validation
- Desktop integration

**Days 6-7: Air-gapped Testing**
- Offline operation validation
- No-network hardening
- Performance optimization
- Final testing and polish

## Week 1-2: BPCI Public Platform (Parallel)

**Days 1-3: Public Pages**
- Homepage with network overview
- Block explorer with search
- Registry listing and details
- Documentation system (mdsvex)

**Days 4-7: Infrastructure**
- CI/CD pipeline setup
- Cloudflare Pages deployment
- CDN configuration
- Signed downloads page

## Week 3-4: BPCI Console

**Days 1-4: Console Core**
- OIDC authentication integration
- Console overview dashboard
- Validator management interface
- Block and bid management

**Days 5-7: Advanced Features**
- Registry management tools
- Economics and financial tracking
- Real-time monitoring
- CSV/JSON export capabilities

**Days 8: Final Polish**
- Lighthouse performance optimization
- Security header validation
- End-to-end testing
- Production deployment

---

# Part 6: Success Metrics

## Performance Targets

**BPI UI:**
- First load: ≤ 600 KB gzipped
- Full bundle: ≤ 10 MB on disk
- Local server startup: ≤ 2 seconds
- Page load time: ≤ 1 second

**BPCI UI:**
- Public pages first load: ≤ 600 KB gzipped
- Console routes: ≤ 300 KB gzipped each
- Lighthouse score: ≥ 95 for performance
- Proof verification: ≤ 30ms

## Security Validation

**Both Platforms:**
- CSP violations: 0
- Mixed content warnings: 0
- XSS vulnerabilities: 0
- Authentication bypass: 0

## User Experience

**BPI UI:**
- Offline operation: 100% functional
- Air-gapped deployment: Successful
- Update success rate: ≥ 99%
- Desktop integration: Working

**BPCI UI:**
- Mobile responsiveness: All breakpoints
- Accessibility: WCAG 2.1 AA compliance
- Browser support: Modern browsers (2 years)
- Uptime: ≥ 99.9%

---

# Conclusion

This specification provides a complete roadmap for delivering both BPI and BPCI UI platforms with distinct deployment models, security requirements, and operational characteristics. The shared technology stack ensures consistency while allowing for platform-specific optimizations and features.

The deliverables include production-ready applications, comprehensive security measures, automated CI/CD pipelines, and a unified release management system that serves both local installations and hosted deployments.
