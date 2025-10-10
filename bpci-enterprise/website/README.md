# BPCI Enterprise Website
## Integrated Web Interface for BPCI System

### Overview
This website is **integrated within the BPCI enterprise system**, not a standalone application. It serves as the web interface for BPCI server coordination, admin dashboard access, and HTTPCG protocol demonstration.

### Integration Architecture
```
BPCI Enterprise System
├── BPCI Server (4 CPU)
│   ├── XTMP Server (Port 9999)
│   ├── VM Server (Port 7777)
│   ├── Admin Dashboard (Port 8888)
│   └── **Website Interface** (Port 3000) ← THIS WEBSITE
├── HTTPCG Wallet (2 CPU)
│   └── Wallet Services (Port 7778)
└── Shadow Registry Bridge
    └── Web2-HTTPCG Bridge (Port 8889)
```

### Website Features
- **BPCI System Interface**: Direct integration with BPCI server
- **Admin Authentication**: Root access to BPCI dashboard
- **HTTPCG Demonstration**: Live protocol examples
- **Enterprise Branding**: Professional BPCI presentation
- **SEO Optimized**: Top 5 Google ranking strategy
- **Real-time Status**: BPCI server health monitoring

### Deployment
The website runs **inside the BPCI enterprise system** as part of the overall infrastructure, not as an external service.
