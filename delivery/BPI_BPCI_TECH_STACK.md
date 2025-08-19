# BPI & BPCI Technical Stack Specification

**Version:** 2.0  
**Date:** August 19, 2025  
**Status:** Final Stack Definition  

---

## Executive Summary

This document defines the complete technical stack for both BPI (bundled, air-gapped operations dashboard) and BPCI (hosted public platform) UIs. The stack emphasizes security, performance, offline capability, and enterprise-grade reliability while maintaining modern development practices.

---

# BPI UI Stack (Bundled in Installer, Runs Locally)

## Core Purpose
Operations dashboard for pipelines, DockLock, ENC, Wallet, PoE/audit in air-gapped environments.

## Framework & Core Technologies

### **Primary Framework**
- **SvelteKit** + **TypeScript**
  - Static adapter (`@sveltejs/adapter-static`)
  - Server-side rendering disabled for static generation
  - Type-safe component development
  - Minimal runtime overhead
  - Excellent tree-shaking capabilities

### **Styling & UI**
- **Tailwind CSS** (purged for minimal bundle size)
  - CSS variables for dynamic theming
  - Dark-first color palette
  - Utility-first approach
  - Custom design tokens for BPI branding
  - PostCSS with autoprefixer

### **Data Visualization**
- **uPlot** - Ultra-lightweight, high-performance charting
  - Real-time metrics visualization
  - Memory-efficient for long-running sessions
  - Canvas-based rendering
  - Minimal bundle impact (~40KB)

### **Data Management**
- **@tanstack/table-core** + **@tanstack/virtual-core**
  - Headless table logic
  - Virtual scrolling for large datasets
  - Sorting, filtering, pagination
  - Framework-agnostic core
- **Svelte stores** for state management
  - Native reactive state
  - No additional state library overhead
  - Custom stores for complex operations

### **Code Editing (Policies/CUE/YAML)**
- **CodeMirror 6** (lazy-loaded)
  - `@codemirror/lang-yaml`
  - `@codemirror/lang-json`
  - Syntax highlighting
  - Error detection
  - Auto-completion
  - Minimal loading impact

### **Real-time Communication**
- **Server-Sent Events (SSE)** / **WebSocket**
  - Connection to local BPI gateway
  - Real-time metrics streaming
  - Event-driven updates
  - Automatic reconnection logic

### **Background Processing**
- **Web Workers**
  - Merkle proof verification
  - ENC attestation verification
  - Heavy computation offloading
  - Non-blocking UI operations

### **Cryptography**
- **WebCrypto API** (native browser crypto)
- **@noble/curves** - Elliptic curve cryptography
- **@noble/hashes** - Cryptographic hash functions
- **BJWT** (BPI JSON Web Tokens) for authenticated writes

### **Local Deployment**
- **Static file serving** via embedded Rust Axum server
- **127.0.0.1** binding for security
- **Assets bundled** in installer (`/ui/*`)
- **Offline documentation** compiled to static pages

### **Testing & Quality**
- **Vitest** - Unit and integration testing
- **Playwright** - End-to-end testing
- **ESLint** + **TypeScript** strict mode
- **Size budgets** enforced in CI
- **Performance monitoring**

### **Performance Targets**
- Initial load: ≤ 600 KB gzipped
- Per-route chunks: ≤ 300 KB gzipped
- Total bundle: ≤ 10 MB
- Time to interactive: < 2 seconds
- Memory usage: < 100 MB sustained

## Optional BPI Features

### **DAG Visualization**
- Lightweight SVG renderer
- No heavy graph libraries
- Custom DAG layout algorithms
- Interactive node exploration

### **SBOM Badge Viewer**
- Lazy-loaded modal
- Software Bill of Materials display
- Security vulnerability tracking
- Compliance reporting

---

# BPCI UI Stack (Hosted Platform)

## Core Purpose
Public website, blockchain explorer, DID registry, and operator console.

## Framework & Core Technologies

### **Primary Framework**
- **SvelteKit** + **TypeScript**
  - Full SSR/SSG capabilities
  - Dynamic routing
  - API routes for backend integration
  - Optimal SEO support

### **Content Management**
- **mdsvex** (MDX for Svelte)
  - Markdown with Svelte components
  - Documentation generation
  - Blog and content management
  - Component embedding in docs

### **Styling & UI**
- **Tailwind CSS** with brand tokens
  - System fonts for performance
  - Responsive design system
  - Accessibility-first approach
  - Custom component library

### **Data Visualization**
- **uPlot** for blockchain metrics
  - Block rate visualization
  - Network latency charts
  - Validator performance graphs
  - Economic metrics tracking

### **Data Management**
- **@tanstack/table-core** + **@tanstack/virtual-core**
  - Large dataset handling
  - Real-time data streaming
  - Advanced filtering and search
- **Server-Sent Events** for live updates
  - Bid streams
  - Block notifications
  - Network events

### **Authentication & Security**
- **Public pages**: Anonymous access
- **Console**: OIDC (PKCE) sessions
  - Lightweight PKCE flow implementation
  - Session management
  - Multi-factor authentication support
- **Privileged writes**: BJWT signing
- **Strict CSP**: No inline scripts
- **COOP/COEP headers**
- **Subresource Integrity (SRI)**
- **Immutable asset hashes**

### **API Integration**
- **bpci-ledger** - Blockchain data
- **bpci-registry** - Identity and credentials
- **bpci-metrics** - Performance data
- **bpci-events** - Real-time notifications

### **Hosting & CDN**
- **Cloudflare Pages** (primary) or Nginx/Caddy
- **Cloudflare CDN** for global distribution
- **Edge caching** strategies
- **Geographic load balancing**

### **Edge Computing (Optional)**
- **Cloudflare Workers**
  - Documentation search
  - Explorer data caching
  - API request optimization
  - Geographic data routing

### **Security Features**
- **Split-origin audit badge**
  - RD (Registry Data) vs BDN (Blockchain Data Network) roots
  - Merkle root comparison
  - Transparency reporting
- **Content Security Policy**
- **HTTP Strict Transport Security**
- **Certificate pinning**

### **Testing & CI/CD**
- **Vitest** - Unit testing
- **Playwright** - E2E testing
- **Lighthouse CI** - Performance budgets
- **GitHub Actions** pipeline
- **Automated security scanning**

### **Performance Targets**
- Public first load: ≤ 600 KB gzipped
- Console routes: ≤ 300 KB gzipped
- Total bundle: ≤ 10 MB
- Core Web Vitals: All green
- Time to First Byte: < 200ms globally

## Optional BPCI Features

### **Data Export**
- CSV/JSON export capabilities
- Bulk data operations
- Scheduled exports
- API-driven exports

### **Governance Tools**
- Economics proposal builder
- Signed governance artifacts
- Voting interfaces
- Proposal tracking

### **Automation**
- Service tokens for API access
- Webhook integrations
- Automated reporting
- Monitoring integrations

---

# Shared Dependencies

## Core Packages
```json
{
  "dependencies": {
    "svelte": "^4.2.0",
    "@sveltejs/kit": "^1.27.0",
    "typescript": "^5.2.0",
    "tailwindcss": "^3.3.0",
    "autoprefixer": "^10.4.0",
    "uplot": "^1.6.0",
    "@tanstack/table-core": "^8.10.0",
    "@tanstack/virtual-core": "^3.5.0",
    "@noble/curves": "^1.2.0",
    "@noble/hashes": "^1.3.0"
  },
  "devDependencies": {
    "vitest": "^0.34.0",
    "@testing-library/svelte": "^4.0.0",
    "playwright": "^1.39.0",
    "eslint": "^8.50.0",
    "@typescript-eslint/eslint-plugin": "^6.7.0",
    "@typescript-eslint/parser": "^6.7.0"
  }
}
```

## BPI-Specific Additions
```json
{
  "dependencies": {
    "codemirror": "^6.0.0",
    "@codemirror/lang-yaml": "^6.0.0",
    "@codemirror/lang-json": "^6.0.0",
    "@sveltejs/adapter-static": "^2.0.0"
  }
}
```

## BPCI-Specific Additions
```json
{
  "dependencies": {
    "mdsvex": "^0.11.0",
    "@mdx-js/mdx": "^2.3.0",
    "oidc-client-ts": "^2.4.0"
  }
}
```

---

# Architecture Decisions

## Why This Stack Works

### **BPI Advantages**
- **Offline-first**: No external dependencies
- **Low latency**: Local serving eliminates network overhead
- **Security**: Air-gapped operation with local signing
- **Performance**: Minimal bundle size for constrained environments
- **Reliability**: No external service dependencies

### **BPCI Advantages**
- **Public trust**: Hosted platform with transparency
- **Discoverability**: SEO-optimized for search engines
- **Scalability**: CDN and edge computing support
- **Admin workflows**: Rich authentication and session management
- **Global reach**: Geographic distribution and caching

### **Shared Benefits**
- **Modern development**: TypeScript, testing, CI/CD
- **Performance**: Strict bundle size limits
- **Security**: Cryptographic verification throughout
- **Maintainability**: Shared component libraries
- **Developer experience**: Hot reloading, debugging tools

---

# Development Workflow

## Project Structure
```
bpi-ui/
├── src/
│   ├── lib/
│   │   ├── components/
│   │   ├── stores/
│   │   ├── crypto/
│   │   └── workers/
│   ├── routes/
│   ├── app.html
│   └── app.css
├── static/
├── tests/
├── docs/
└── package.json

bpci-ui/
├── src/
│   ├── lib/
│   │   ├── components/
│   │   ├── stores/
│   │   ├── auth/
│   │   └── api/
│   ├── routes/
│   │   ├── (public)/
│   │   ├── (console)/
│   │   └── api/
│   ├── content/
│   ├── app.html
│   └── app.css
├── static/
├── tests/
└── package.json
```

## Development Commands
```bash
# Development
npm run dev          # Development server
npm run build        # Production build
npm run preview      # Preview build

# Testing
npm run test         # Unit tests
npm run test:e2e     # End-to-end tests
npm run test:ci      # CI test suite

# Quality
npm run lint         # ESLint
npm run format       # Prettier
npm run typecheck    # TypeScript check
npm run analyze      # Bundle analysis

# Performance
npm run lighthouse   # Performance audit
npm run size-limit   # Bundle size check
```

## CI/CD Pipeline
```yaml
# .github/workflows/ci.yml
name: CI/CD
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v3
      - run: npm ci
      - run: npm run lint
      - run: npm run typecheck
      - run: npm run test
      - run: npm run build
      - run: npm run test:e2e
      - run: npm run lighthouse
      - run: npm run size-limit
```

---

# Security Considerations

## BPI Security
- **Local-only serving**: 127.0.0.1 binding
- **No external requests**: Completely air-gapped
- **Cryptographic signing**: BJWT for all write operations
- **Input validation**: All user inputs sanitized
- **Memory safety**: Web Workers for isolation

## BPCI Security
- **Content Security Policy**: Strict CSP headers
- **HTTPS everywhere**: TLS 1.3 minimum
- **Authentication**: OIDC with PKCE flow
- **Authorization**: Role-based access control
- **Audit logging**: All privileged operations logged
- **Split-origin verification**: Independent audit roots

## Shared Security
- **Dependency scanning**: Automated vulnerability checks
- **Code signing**: All releases cryptographically signed
- **Secure defaults**: Security-first configuration
- **Regular updates**: Dependency and security patches
- **Penetration testing**: Regular security assessments

---

# Performance Optimization

## Bundle Optimization
- **Tree shaking**: Eliminate unused code
- **Code splitting**: Route-based chunks
- **Lazy loading**: Non-critical components
- **Asset optimization**: Image and font optimization
- **Compression**: Brotli and gzip compression

## Runtime Performance
- **Virtual scrolling**: Large dataset handling
- **Web Workers**: CPU-intensive operations
- **Caching strategies**: Intelligent data caching
- **Prefetching**: Predictive resource loading
- **Memory management**: Garbage collection optimization

## Monitoring
- **Real User Monitoring**: Performance tracking
- **Core Web Vitals**: Google performance metrics
- **Bundle analysis**: Size and composition tracking
- **Error tracking**: Runtime error monitoring
- **Performance budgets**: Automated performance gates

---

# Deployment Strategy

## BPI Deployment
1. **Build static assets** with `adapter-static`
2. **Bundle in installer** under `/ui/*`
3. **Embed static server** in BPI gateway
4. **Serve on localhost** with strict CSP
5. **Offline documentation** included in bundle

## BPCI Deployment
1. **Build optimized bundle** with SSR/SSG
2. **Deploy to Cloudflare Pages** or equivalent
3. **Configure CDN** with edge caching
4. **Set up monitoring** and alerting
5. **Enable security headers** and CSP

---

# Future Enhancements

## Planned Features
- **Progressive Web App**: Offline capability for BPCI
- **WebAssembly**: Performance-critical operations
- **Service Workers**: Advanced caching strategies
- **Web Streams**: Large data processing
- **WebRTC**: Peer-to-peer communication

## Experimental Features
- **WebGPU**: GPU-accelerated cryptography
- **Web Locks**: Coordination across tabs
- **Persistent Storage**: Local data persistence
- **Background Sync**: Offline operation support
- **Push Notifications**: Real-time alerts

---

# Conclusion

This technical stack provides a robust foundation for both BPI and BPCI platforms, balancing security, performance, and developer experience. The clear separation between bundled (BPI) and hosted (BPCI) architectures enables optimal solutions for each use case while maintaining shared development practices and component libraries.

The stack emphasizes modern web standards, cryptographic security, and enterprise-grade reliability while keeping bundle sizes minimal and performance optimal. This foundation supports the complex requirements of blockchain infrastructure management while providing excellent user experiences across both air-gapped and public environments.
