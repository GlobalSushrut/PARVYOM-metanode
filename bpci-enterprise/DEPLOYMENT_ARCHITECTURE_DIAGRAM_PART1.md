# 🏗️ SUPREME DEPLOYMENT ARCHITECTURE - PART 1: OVERVIEW

**Date**: 2025-10-30  
**Complexity**: SUPREME - Most Detailed Deployment Diagram Ever Created

---

## 🎯 COMPLETE SYSTEM ARCHITECTURE

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         INTERNET / PUBLIC ACCESS                             │
│                    https://portal.pravyom.network                            │
└──────────────────────────────────┬──────────────────────────────────────────┘
                                   │
                                   │ HTTPS/TLS (Cloudflare SSL)
                                   │ Port 443
                                   ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                          NGINX REVERSE PROXY                                 │
│                         (Port 80/443 - SSL/TLS)                             │
│  ┌────────────────────────────────────────────────────────────────────┐    │
│  │  SSL Termination                                                    │    │
│  │  - Cloudflare SSL certificate                                       │    │
│  │  - TLS 1.3 encryption                                               │    │
│  │  - HSTS enabled                                                     │    │
│  │  - Security headers (CSP, X-Frame-Options, etc.)                   │    │
│  └────────────────────────────────────────────────────────────────────┘    │
│  ┌────────────────────────────────────────────────────────────────────┐    │
│  │  Routing Rules                                                      │    │
│  │  / → Frontend (React SPA)                                          │    │
│  │  /api/* → Backend API (Port 8080)                                  │    │
│  │  /ws/* → WebSocket (Port 8889)                                     │    │
│  │  /auth/* → Keycloak (Port 8180)                                    │    │
│  │  /installer → Community Installer (Port 18082)                     │    │
│  └────────────────────────────────────────────────────────────────────┘    │
└──────────────┬──────────────┬──────────────┬──────────────┬────────────────┘
               │              │              │              │
               │              │              │              │
    ┌──────────▼─────┐  ┌────▼─────┐  ┌────▼─────┐  ┌────▼─────┐
    │   Frontend     │  │   API    │  │   WS     │  │  Auth    │
    │   (Static)     │  │  Proxy   │  │  Proxy   │  │  Proxy   │
    └────────────────┘  └──────────┘  └──────────┘  └──────────┘
```

---

## 📊 LAYER 1: FRONTEND (4 COMPARTMENTS)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    REACT FRONTEND APPLICATION                                │
│                  (Served by Nginx - Static Files)                           │
│                  Location: /var/www/bpci-frontend/dist                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────┐      │
│  │  COMPARTMENT 1: PUBLIC WEBSITE (No Auth Required)               │      │
│  │  ────────────────────────────────────────────────────────────   │      │
│  │  Routes:                                                          │      │
│  │  • / (Home)                                                       │      │
│  │  • /about (About)                                                 │      │
│  │  • /technology (Technology)                                       │      │
│  │  • /enterprise (Enterprise)                                       │      │
│  │  • /community (Community)                                         │      │
│  │  • /blog (Blog)                                                   │      │
│  │  • /research (Research)                                           │      │
│  │  • /contact (Contact)                                             │      │
│  │  • /legal, /privacy-policy, /terms-of-service                   │      │
│  │                                                                    │      │
│  │  Backend: Static content only                                     │      │
│  └──────────────────────────────────────────────────────────────────┘      │
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────┐      │
│  │  COMPARTMENT 2: AUTHENTICATION (Keycloak OAuth2/OIDC)           │      │
│  │  ────────────────────────────────────────────────────────────   │      │
│  │  Routes:                                                          │      │
│  │  • /login (Login page)                                            │      │
│  │  • /register (Registration)                                       │      │
│  │  • /verify-email (Email verification)                            │      │
│  │  • /reset-password (Password reset)                              │      │
│  │                                                                    │      │
│  │  Services:                                                         │      │
│  │  • keycloakService.ts → Keycloak (Port 8180)                    │      │
│  │  • authService.ts → PostgreSQL (via Keycloak)                   │      │
│  │  • authTokenManager.ts → JWT token management                    │      │
│  │                                                                    │      │
│  │  Security:                                                         │      │
│  │  • OAuth2/OIDC with PKCE                                         │      │
│  │  • Automatic token refresh                                        │      │
│  │  • Session management (24-hour expiry)                           │      │
│  └──────────────────────────────────────────────────────────────────┘      │
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────┐      │
│  │  COMPARTMENT 3: DASHBOARD & MANAGEMENT (Auth Required)          │      │
│  │  ────────────────────────────────────────────────────────────   │      │
│  │  Routes:                                                          │      │
│  │  • /dashboard (Main dashboard)                                    │      │
│  │  • /registry (Registry dashboard)                                 │      │
│  │  • /wallet (Wallet manager)                                       │      │
│  │  • /installer (BPI installer)                                     │      │
│  │                                                                    │      │
│  │  Services:                                                         │      │
│  │  • bpciApi.ts → All 12+ backend servers                         │      │
│  │  • realTimeService.ts → WebSocket connections                    │      │
│  │  • registryService.ts → Registry operations                      │      │
│  │                                                                    │      │
│  │  Backend Connections:                                             │      │
│  │  • BPCI Blockchain (8080)                                        │      │
│  │  • Consensus Server (9001)                                        │      │
│  │  • Cluster Ledger (7000)                                         │      │
│  │  • Auction Mempool (7002)                                        │      │
│  │  • BPI-BPCI Bridge (6001)                                        │      │
│  │  • Shadow Registry (8081)                                         │      │
│  │  • XTMP Server (8889)                                            │      │
│  │  • BSO-K8 Orchestrator (9090)                                    │      │
│  └──────────────────────────────────────────────────────────────────┘      │
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────┐      │
│  │  COMPARTMENT 4: ADVANCED FEATURES (Admin Auth Required)         │      │
│  │  ────────────────────────────────────────────────────────────   │      │
│  │  Routes:                                                          │      │
│  │  • /admin (Admin panel)                                           │      │
│  │  • /node-deployment (Node deployment wizard)                     │      │
│  │  • /node-management (Node management dashboard)                  │      │
│  │  • /mojo-wallet (Mojo wallet dashboard)                         │      │
│  │  • /advanced-metrics (Advanced metrics)                          │      │
│  │  • /api-docs (API documentation)                                 │      │
│  │                                                                    │      │
│  │  Services:                                                         │      │
│  │  • paymentService.ts → Stripe + Crypto payments                 │      │
│  │  • emailService.ts → Email notifications                         │      │
│  │  • blogService.ts → Blog management                              │      │
│  │                                                                    │      │
│  │  Admin Features:                                                  │      │
│  │  • System administration                                          │      │
│  │  • User management                                                │      │
│  │  • Service monitoring                                             │      │
│  │  • Configuration management                                       │      │
│  └──────────────────────────────────────────────────────────────────┘      │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 🔐 LAYER 2: AUTHENTICATION & SESSION MANAGEMENT

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    KEYCLOAK AUTHENTICATION SERVER                            │
│                         (Port 8180 - OAuth2/OIDC)                           │
│                    Location: /opt/keycloak/bin/kc.sh                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Configuration:                                                              │
│  • Realm: bpci-enterprise                                                   │
│  • Client ID: bpci-web-client                                               │
│  • Client Secret: <generated>                                               │
│  • Redirect URIs: https://portal.pravyom.network/*                         │
│  • Admin User: admin                                                        │
│  • Admin Password: <secure>                                                 │
│                                                                              │
│  Features:                                                                   │
│  • OAuth2/OIDC authentication                                               │
│  • Single Sign-On (SSO)                                                     │
│  • User federation (PostgreSQL)                                             │
│  • Role-based access control (RBAC)                                         │
│  • Permission management                                                     │
│  • Token management (JWT)                                                   │
│  • Session management                                                        │
│                                                                              │
│  Database Connection:                                                        │
│  jdbc:postgresql://localhost:5432/keycloak                                  │
│  Username: keycloak                                                          │
│  Password: <secure>                                                          │
│                                                                              │
└──────────────────────────────┬──────────────────────────────────────────────┘
                               │
                               │ JDBC Connection
                               ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                    POSTGRESQL DATABASE SERVER                                │
│                         (Port 5432 - Primary DB)                            │
│                    Location: /opt/bpci/data/postgresql                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Databases:                                                                  │
│  • keycloak (Keycloak data)                                                 │
│  • bpci_users (User profiles)                                               │
│  • bpci_sessions (Active sessions)                                          │
│  • bpci_wallets (User wallets)                                              │
│                                                                              │
│  Tables:                                                                     │
│  ┌────────────────────────────────────────────────────────────┐            │
│  │  users                                                      │            │
│  │  • user_id (UUID, PRIMARY KEY)                             │            │
│  │  • email (VARCHAR, UNIQUE, INDEXED)                        │            │
│  │  • password_hash (VARCHAR) - bcrypt 12 rounds             │            │
│  │  • email_verified (BOOLEAN)                                │            │
│  │  • created_at (TIMESTAMP)                                  │            │
│  │  • last_login (TIMESTAMP)                                  │            │
│  │  • status (VARCHAR) - Active/Inactive/Suspended           │            │
│  │  • kyc_status (VARCHAR) - Unverified/Verified             │            │
│  │  • user_tier (VARCHAR) - Basic/Premium/Enterprise         │            │
│  └────────────────────────────────────────────────────────────┘            │
│  ┌────────────────────────────────────────────────────────────┐            │
│  │  sessions                                                   │            │
│  │  • session_id (UUID, PRIMARY KEY)                          │            │
│  │  • user_id (UUID, FOREIGN KEY → users)                     │            │
│  │  • session_token (VARCHAR, UNIQUE, INDEXED)                │            │
│  │  • created_at (TIMESTAMP)                                  │            │
│  │  • expires_at (TIMESTAMP) - 24 hours default              │            │
│  │  • ip_address (VARCHAR)                                    │            │
│  │  • user_agent (TEXT)                                       │            │
│  └────────────────────────────────────────────────────────────┘            │
│  ┌────────────────────────────────────────────────────────────┐            │
│  │  wallets                                                    │            │
│  │  • wallet_id (UUID, PRIMARY KEY)                           │            │
│  │  • user_id (UUID, FOREIGN KEY → users)                     │            │
│  │  • wallet_name (VARCHAR)                                   │            │
│  │  • public_key (TEXT) - Ed25519                             │            │
│  │  • private_key_encrypted (TEXT) - Encrypted with password │            │
│  │  • bpi_address (VARCHAR, UNIQUE, INDEXED)                 │            │
│  │  • is_activated (BOOLEAN)                                  │            │
│  │  • balance (BIGINT)                                        │            │
│  │  • created_at (TIMESTAMP)                                  │            │
│  └────────────────────────────────────────────────────────────┘            │
│                                                                              │
│  Performance:                                                                │
│  • Indexes on email, session_token, bpi_address                            │
│  • Connection pooling (max 100 connections)                                 │
│  • Query optimization                                                        │
│  • Automatic vacuum                                                          │
│                                                                              │
└──────────────────────────────┬──────────────────────────────────────────────┘
                               │
                               │ Session Caching
                               ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                        REDIS CACHE SERVER                                    │
│                         (Port 6379 - In-Memory Cache)                       │
│                    Location: /var/lib/redis                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Configuration:                                                              │
│  • Memory limit: 2GB                                                        │
│  • Eviction policy: allkeys-lru                                             │
│  • Persistence: RDB + AOF                                                   │
│  • Max connections: 10000                                                   │
│                                                                              │
│  Cached Data:                                                                │
│  • User sessions (TTL: 24 hours)                                            │
│  • JWT tokens (TTL: 1 hour)                                                 │
│  • Query results (TTL: 5 minutes)                                           │
│  • API responses (TTL: 1 minute)                                            │
│  • Wallet balances (TTL: 30 seconds)                                        │
│                                                                              │
│  Performance:                                                                │
│  • Sub-millisecond latency                                                  │
│  • 100,000+ ops/sec                                                         │
│  • Automatic expiration                                                      │
│  • Memory optimization                                                       │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

**CONTINUE TO PART 2 FOR BACKEND SERVERS...**
