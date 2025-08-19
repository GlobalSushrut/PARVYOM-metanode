# BPCI UI Components Specification

**Version:** 1.0  
**Date:** August 19, 2025  
**Target:** Complete component library for BPCI hosted platform (Website + Registry + Console + Login)

---

## Executive Summary

This document specifies all UI components needed for the BPCI (BPI Communication Interface) hosted platform. Based on analysis of the codebase including decentralized identity systems, verifiable credentials, authentication protocols, and blockchain infrastructure, this component library enables comprehensive public blockchain platform management with both community-facing and operator-focused interfaces.

---

# Platform Architecture Overview

## BPCI Platform Components

### **1. Public Website Components**
- Homepage and marketing pages
- Documentation and developer resources
- Community engagement interfaces
- Public blockchain explorer

### **2. Registry Components (Dual-Side)**
- **Community Registry**: Public DID/entity browsing and verification
- **BPI Registry**: Operator-controlled entity management and validation
- Cross-registry verification and trust scoring
- Credential and attestation management

### **3. Console Dashboard Components**
- Authenticated operator dashboard
- Validator and node management
- Economic tracking and rewards
- Registry administration tools

### **4. Authentication & Identity Components**
- OIDC (PKCE) authentication flow
- Multi-factor authentication
- DID-based identity verification
- Session and access management

---

# Public Website Components

## 1. Homepage & Marketing

### **HeroSection**
```typescript
interface HeroSectionProps {
  headline: string;
  subheadline: string;
  ctaButtons: Array<{
    text: string;
    href: string;
    variant: 'primary' | 'secondary' | 'outline';
  }>;
  backgroundVideo?: string;
  statistics?: Array<{
    value: string;
    label: string;
    trend?: number;
  }>;
}
```

**Features:**
- Compelling value proposition presentation
- Call-to-action button group
- Real-time network statistics
- Background video or animation
- Responsive design with mobile optimization

### **NetworkStatsOverview**
```typescript
interface NetworkStatsProps {
  stats: {
    totalNodes: number;
    activeValidators: number;
    totalTransactions: number;
    networkUptime: number;
    avgBlockTime: number;
    stakingRatio: number;
  };
  realTime: boolean;
  onStatsClick?: (statType: string) => void;
}
```

**Features:**
- Live network statistics display
- Animated counters and progress indicators
- Click-through to detailed explorer
- Historical trend indicators
- Mobile-responsive grid layout

### **FeatureShowcase**
```typescript
interface FeatureShowcaseProps {
  features: Array<{
    id: string;
    title: string;
    description: string;
    icon: string;
    benefits: string[];
    demoUrl?: string;
    learnMoreUrl?: string;
  }>;
  layout: 'grid' | 'carousel' | 'tabs';
  onFeatureSelect?: (featureId: string) => void;
}
```

**Features:**
- Feature highlighting with benefits
- Interactive demonstrations
- Multiple layout options
- Progressive disclosure of details
- Integration with documentation

## 2. Documentation Hub

### **DocumentationHub**
```typescript
interface DocumentationHubProps {
  sections: Array<{
    id: string;
    title: string;
    description: string;
    icon: string;
    articles: Array<{
      title: string;
      excerpt: string;
      readTime: number;
      difficulty: 'beginner' | 'intermediate' | 'advanced';
      url: string;
    }>;
  }>;
  searchQuery: string;
  onSearch: (query: string) => void;
  onSectionSelect: (sectionId: string) => void;
}
```

**Features:**
- Categorized documentation sections
- Full-text search functionality
- Difficulty level indicators
- Reading time estimates
- Progressive navigation

### **APIDocumentation**
```typescript
interface APIDocumentationProps {
  endpoints: Array<{
    method: 'GET' | 'POST' | 'PUT' | 'DELETE';
    path: string;
    description: string;
    parameters: Array<{
      name: string;
      type: string;
      required: boolean;
      description: string;
    }>;
    responses: Array<{
      status: number;
      description: string;
      schema: any;
    }>;
    examples: Array<{
      title: string;
      request: any;
      response: any;
    }>;
  }>;
  onTryEndpoint: (endpoint: any, params: any) => void;
}
```

**Features:**
- Interactive API explorer
- Code examples in multiple languages
- Try-it-now functionality
- Response schema visualization
- Authentication examples

---

# Registry Components (Dual-Side)

## 3. Community Registry (Public)

### **EntityBrowser**
```typescript
interface EntityBrowserProps {
  entities: Array<{
    did: string;
    name?: string;
    type: 'individual' | 'organization' | 'service' | 'device';
    verificationStatus: 'verified' | 'pending' | 'unverified' | 'revoked';
    trustScore: number;
    credentials: number;
    lastActivity: Date;
    publicProfile: {
      description?: string;
      website?: string;
      location?: string;
      avatar?: string;
    };
  }>;
  filters: {
    type?: string[];
    verificationStatus?: string[];
    trustScoreRange?: [number, number];
    location?: string;
  };
  searchQuery: string;
  onEntitySelect: (did: string) => void;
  onFilterChange: (filters: any) => void;
  onSearch: (query: string) => void;
}
```

**Features:**
- Public entity directory browsing
- Advanced filtering and search
- Trust score visualization
- Verification status indicators
- Entity profile previews

### **EntityProfileViewer**
```typescript
interface EntityProfileViewerProps {
  entity: {
    did: string;
    profile: {
      name?: string;
      description?: string;
      avatar?: string;
      website?: string;
      location?: string;
      publicKey: string;
    };
    verification: {
      status: string;
      verifiedBy: string[];
      verificationDate: Date;
      attestations: Array<{
        type: string;
        issuer: string;
        issuedDate: Date;
        expiryDate?: Date;
        status: 'valid' | 'expired' | 'revoked';
      }>;
    };
    trustMetrics: {
      score: number;
      factors: Array<{
        name: string;
        weight: number;
        value: number;
      }>;
      history: Array<{
        date: Date;
        score: number;
        event: string;
      }>;
    };
    activity: {
      transactionCount: number;
      lastActivity: Date;
      interactionCount: number;
    };
  };
  onVerificationRequest: () => void;
  onTrustVote: (vote: 'up' | 'down') => void;
}
```

**Features:**
- Comprehensive entity profiles
- Verification status display
- Trust score breakdown
- Activity history
- Community interaction tools

### **CredentialVerifier**
```typescript
interface CredentialVerifierProps {
  credential: {
    id: string;
    type: string;
    issuer: {
      did: string;
      name: string;
      trustScore: number;
    };
    subject: {
      did: string;
      claims: any;
    };
    issuanceDate: Date;
    expirationDate?: Date;
    proof: {
      type: string;
      signature: string;
      verificationMethod: string;
    };
    status: 'valid' | 'expired' | 'revoked' | 'suspended';
  };
  onVerify: () => Promise<boolean>;
  onDownload: () => void;
  onShare: () => void;
}
```

**Features:**
- Credential authenticity verification
- Cryptographic proof validation
- Issuer trust assessment
- Credential sharing tools
- Revocation status checking

## 4. BPI Registry (Operator)

### **EntityManagementConsole**
```typescript
interface EntityManagementConsoleProps {
  entities: Array<{
    did: string;
    name: string;
    type: string;
    status: 'active' | 'suspended' | 'pending_review' | 'revoked';
    registrationDate: Date;
    lastUpdate: Date;
    verificationLevel: 'basic' | 'enhanced' | 'premium';
    riskScore: number;
    flags: string[];
    operator: string;
  }>;
  pendingActions: Array<{
    id: string;
    type: 'verification' | 'suspension' | 'revocation';
    entityDid: string;
    requestedBy: string;
    reason: string;
    priority: 'low' | 'medium' | 'high' | 'critical';
    dueDate: Date;
  }>;
  onEntityAction: (entityDid: string, action: string, params?: any) => void;
  onBulkAction: (entityDids: string[], action: string) => void;
  onActionApprove: (actionId: string) => void;
  onActionReject: (actionId: string, reason: string) => void;
}
```

**Features:**
- Entity lifecycle management
- Bulk operations support
- Pending action queue
- Risk assessment tools
- Audit trail tracking

### **VerificationWorkflow**
```typescript
interface VerificationWorkflowProps {
  workflow: {
    id: string;
    entityDid: string;
    type: 'kyc' | 'kyb' | 'technical' | 'security';
    status: 'initiated' | 'in_progress' | 'review' | 'approved' | 'rejected';
    steps: Array<{
      id: string;
      name: string;
      status: 'pending' | 'completed' | 'failed' | 'skipped';
      assignee?: string;
      dueDate?: Date;
      documents: Array<{
        name: string;
        type: string;
        status: 'uploaded' | 'verified' | 'rejected';
        url: string;
      }>;
      notes: string[];
    }>;
    reviewers: Array<{
      id: string;
      name: string;
      role: string;
      decision?: 'approve' | 'reject' | 'request_info';
      comments?: string;
    }>;
  };
  onStepUpdate: (stepId: string, status: string, notes?: string) => void;
  onDocumentReview: (documentId: string, decision: string, notes: string) => void;
  onWorkflowDecision: (decision: string, comments: string) => void;
}
```

**Features:**
- Multi-step verification process
- Document review interface
- Reviewer assignment
- Decision tracking
- Compliance reporting

---

# Console Dashboard Components

## 5. Operator Dashboard

### **OperatorOverview**
```typescript
interface OperatorOverviewProps {
  overview: {
    nodeStatus: {
      total: number;
      active: number;
      syncing: number;
      offline: number;
    };
    validatorMetrics: {
      totalStake: string;
      activeValidators: number;
      rewardsEarned: string;
      slashingEvents: number;
    };
    networkHealth: {
      blockHeight: number;
      avgBlockTime: number;
      networkHashrate: string;
      peerCount: number;
    };
    registryStats: {
      totalEntities: number;
      pendingVerifications: number;
      recentRegistrations: number;
      complianceScore: number;
    };
  };
  alerts: Array<{
    id: string;
    severity: 'critical' | 'warning' | 'info';
    title: string;
    description: string;
    timestamp: Date;
    acknowledged: boolean;
  }>;
  onAlertAcknowledge: (alertId: string) => void;
  onQuickAction: (action: string) => void;
}
```

**Features:**
- Comprehensive system overview
- Real-time status monitoring
- Alert management
- Quick action buttons
- Performance metrics

### **ValidatorManagement**
```typescript
interface ValidatorManagementProps {
  validators: Array<{
    id: string;
    address: string;
    status: 'active' | 'inactive' | 'jailed' | 'unbonding';
    stake: {
      selfStake: string;
      delegatedStake: string;
      totalStake: string;
    };
    performance: {
      uptime: number;
      missedBlocks: number;
      proposedBlocks: number;
      attestationRate: number;
    };
    rewards: {
      totalEarned: string;
      lastPayout: Date;
      pendingRewards: string;
    };
    commission: {
      rate: number;
      maxRate: number;
      maxChangeRate: number;
    };
  }>;
  onValidatorAction: (validatorId: string, action: string, params?: any) => void;
  onStakeManagement: (validatorId: string, action: 'stake' | 'unstake', amount: string) => void;
  onCommissionUpdate: (validatorId: string, newRate: number) => void;
}
```

**Features:**
- Validator lifecycle management
- Staking operations
- Performance monitoring
- Reward tracking
- Commission management

---

# Authentication & Identity Components

## 6. Authentication System

### **LoginInterface**
```typescript
interface LoginInterfaceProps {
  authMethods: Array<{
    type: 'oidc' | 'did' | 'webauthn' | 'oauth2';
    name: string;
    icon: string;
    enabled: boolean;
    description: string;
  }>;
  loginState: {
    loading: boolean;
    error?: string;
    redirectUrl?: string;
  };
  onAuthMethodSelect: (method: string) => void;
  onCredentialSubmit: (credentials: any) => void;
  onForgotPassword: () => void;
  onCreateAccount: () => void;
}
```

**Features:**
- Multiple authentication methods
- OIDC (PKCE) integration
- DID-based authentication
- WebAuthn support
- Error handling and recovery

### **OIDCAuthFlow**
```typescript
interface OIDCAuthFlowProps {
  config: {
    authority: string;
    clientId: string;
    redirectUri: string;
    scope: string[];
    responseType: string;
  };
  flowState: {
    step: 'initiate' | 'authorize' | 'callback' | 'complete' | 'error';
    codeVerifier?: string;
    codeChallenge?: string;
    state?: string;
    authorizationCode?: string;
    accessToken?: string;
    idToken?: string;
    error?: string;
  };
  onFlowStart: () => void;
  onAuthorizationReceived: (code: string, state: string) => void;
  onTokenExchange: () => void;
  onFlowComplete: (tokens: any) => void;
}
```

**Features:**
- PKCE flow implementation
- State parameter validation
- Token exchange handling
- Security validation
- Error recovery

### **MultiFactorAuth**
```typescript
interface MultiFactorAuthProps {
  factors: Array<{
    type: 'totp' | 'sms' | 'email' | 'webauthn' | 'backup_codes';
    name: string;
    enabled: boolean;
    configured: boolean;
    lastUsed?: Date;
  }>;
  currentChallenge?: {
    type: string;
    challenge: string;
    expiresAt: Date;
  };
  onFactorSetup: (factorType: string) => void;
  onFactorVerify: (factorType: string, response: string) => void;
  onFactorDisable: (factorType: string) => void;
  onBackupCodesGenerate: () => void;
}
```

**Features:**
- Multiple MFA methods
- Factor setup and configuration
- Challenge-response handling
- Backup code management
- Factor recovery options

---

# Shared UI Components

## 7. Common Interface Elements

### **BlockchainExplorer**
```typescript
interface BlockchainExplorerProps {
  data: {
    blocks: Array<{
      height: number;
      hash: string;
      timestamp: Date;
      proposer: string;
      transactionCount: number;
      size: number;
    }>;
    transactions: Array<{
      hash: string;
      blockHeight: number;
      from: string;
      to: string;
      value: string;
      fee: string;
      status: string;
    }>;
    validators: Array<{
      address: string;
      moniker: string;
      votingPower: string;
      commission: number;
      status: string;
    }>;
  };
  searchQuery: string;
  onSearch: (query: string, type: 'block' | 'transaction' | 'address') => void;
  onItemSelect: (type: string, id: string) => void;
}
```

**Features:**
- Multi-type blockchain data exploration
- Advanced search and filtering
- Real-time data updates
- Detailed item views
- Export capabilities

### **TrustScoreIndicator**
```typescript
interface TrustScoreIndicatorProps {
  score: number;
  maxScore: number;
  factors: Array<{
    name: string;
    weight: number;
    value: number;
    description: string;
  }>;
  trend?: 'up' | 'down' | 'stable';
  size?: 'small' | 'medium' | 'large';
  showBreakdown?: boolean;
}
```

**Features:**
- Visual trust score representation
- Factor breakdown display
- Trend indicators
- Multiple size variants
- Tooltip explanations

### **StatusBadge**
```typescript
interface StatusBadgeProps {
  status: string;
  variant: 'success' | 'warning' | 'error' | 'info' | 'neutral';
  size?: 'small' | 'medium' | 'large';
  animated?: boolean;
  tooltip?: string;
}
```

**Features:**
- Consistent status representation
- Multiple visual variants
- Animation support
- Tooltip integration
- Accessibility compliance

---

# Implementation Guidelines

## Development Standards

### **Component Architecture**
- Modular, reusable component design
- TypeScript interfaces for all props
- Consistent naming conventions
- Proper error boundaries
- Performance optimization

### **State Management**
- React Context for global state
- React Query for server state
- Local state for component-specific data
- Optimistic updates for better UX
- Real-time data synchronization

### **Security Implementation**
- Input validation and sanitization
- XSS prevention measures
- CSRF protection
- Secure token storage
- Content Security Policy compliance

### **Testing Strategy**
- Unit tests for all components
- Integration tests for workflows
- End-to-end tests for critical paths
- Visual regression testing
- Accessibility testing

### **Performance Optimization**
- Code splitting by route and feature
- Lazy loading for non-critical components
- Virtual scrolling for large datasets
- Image optimization and lazy loading
- Bundle size monitoring

---

# Integration Points

## API Integration

### **REST Endpoints**
- `/api/auth/*` - Authentication and authorization
- `/api/registry/*` - Entity and credential management
- `/api/validators/*` - Validator operations
- `/api/explorer/*` - Blockchain data
- `/api/compliance/*` - Compliance and audit data

### **WebSocket Connections**
- Real-time network statistics
- Live transaction feeds
- Validator status updates
- Alert notifications
- Registry change events

### **External Services**
- OIDC provider integration
- DID resolver services
- Credential verification services
- Blockchain RPC endpoints
- Analytics and monitoring

---

# Deployment Strategy

## Build Process

### **Development Environment**
```bash
npm run dev          # Development server
npm run storybook    # Component library
npm run test         # Test suite
npm run lint         # Code quality
```

### **Production Build**
```bash
npm run build        # Production build
npm run analyze      # Bundle analysis
npm run lighthouse   # Performance audit
npm run deploy       # Deployment
```

## Hosting Configuration

### **Cloudflare Pages**
- Static site hosting
- Edge caching configuration
- Custom domain setup
- SSL/TLS certificates
- Performance optimization

### **CDN Integration**
- Asset optimization
- Geographic distribution
- Cache invalidation
- Security headers
- Performance monitoring

---

# Conclusion

This comprehensive BPCI component specification provides the foundation for building a world-class public blockchain platform. The components are designed to handle complex identity management, registry operations, and blockchain infrastructure while maintaining security, performance, and usability.

The dual-registry approach enables both community engagement and operator control, while the authentication system supports multiple methods including cutting-edge DID-based authentication. The modular architecture ensures maintainability and extensibility for future platform evolution.
