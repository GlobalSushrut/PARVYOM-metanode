# 🏢 CENTRAL BPCI SERVER - COMPREHENSIVE DESIGN DOCUMENT

**Version**: 1.0  
**Date**: 2025-10-30  
**Status**: Design Phase - Production Ready Architecture

---

## 🎯 EXECUTIVE SUMMARY

The Central BPCI Server is the **owner-operated control center** for the entire BPCI Enterprise infrastructure, providing:
- **Payment Processing** with Stripe integration
- **Admin Control Panel** for service management
- **Free Credit System** for testnet (5000 BPI tokens)
- **Container Payment Stability** for VM/vPod billing
- **Multi-tenant Management** for enterprise customers
- **Real-time Monitoring** and analytics

---

## 📊 EXISTING INFRASTRUCTURE ANALYSIS

### **✅ Already Implemented (Found in Codebase):**

1. **Token Pricing System** (`bpci_bpi_bridge.rs`)
   - TokenPricingPlan with CAD/USD pricing
   - Monthly allocations and billing cycles
   - Free allocation periods
   - Pilot excess tokens
   - Hourly rate calculations

2. **User Account Management**
   - BridgeUserAccount with balance tracking
   - Account types: Testnet, Pilot, Enterprise, Developer
   - Rent session tracking
   - Monthly usage monitoring

3. **Payment Processing** (`bpi_integration.rs`)
   - Gas fee collection
   - Rent payment processing
   - Billing cycle management
   - Payment interval tracking (60 minutes)

4. **Economic Integration** (`bpci_economic_integration.rs`)
   - 4-coin autonomous economy (GEN/NEX/FLX/AUR)
   - Treasury system with 25%/75% split
   - Fiat inflow processing
   - Distribution flow management

5. **Address Pool Manager**
   - Millions of BPI connections
   - Connection pooling
   - Auto-discovery

---

## 🏗️ NEW COMPONENTS TO BUILD

### **1. CENTRAL ADMIN SERVER (Component 14)**

**Purpose**: Owner-operated control center for entire BPCI infrastructure

**Core Features**:
```rust
pub struct CentralAdminServer {
    // Admin authentication
    admin_auth: Arc<AdminAuthSystem>,
    
    // Payment processing
    stripe_integration: Arc<StripePaymentProcessor>,
    payment_stability: Arc<ContainerPaymentStability>,
    
    // Free credit system
    credit_manager: Arc<FreeCreditManager>,
    
    // Multi-tenant management
    tenant_manager: Arc<TenantManager>,
    
    // Real-time monitoring
    monitoring_dashboard: Arc<MonitoringDashboard>,
    
    // Service control
    service_controller: Arc<ServiceController>,
}
```

---

### **2. STRIPE PAYMENT INTEGRATION**

**Architecture**: Dual-layer payment system

#### **Layer 1: Stripe Payment Processor**
```rust
pub struct StripePaymentProcessor {
    stripe_client: stripe::Client,
    webhook_secret: String,
    
    // Payment methods
    pub async fn create_customer(&self, email: String) -> Result<Customer>,
    pub async fn create_subscription(&self, plan: PricingPlan) -> Result<Subscription>,
    pub async fn process_payment(&self, amount: Decimal) -> Result<PaymentIntent>,
    pub async fn handle_webhook(&self, event: WebhookEvent) -> Result<()>,
    
    // Refunds and disputes
    pub async fn process_refund(&self, payment_id: String) -> Result<Refund>,
    pub async fn handle_dispute(&self, dispute: Dispute) -> Result<()>,
}
```

#### **Layer 2: Container Payment Stability**
```rust
pub struct ContainerPaymentStability {
    // Pre-authorization for containers
    preauth_pool: Arc<RwLock<HashMap<String, PreAuthorization>>>,
    
    // Payment buffer (prevents container shutdown during payment processing)
    payment_buffer_seconds: u64, // 300 seconds (5 minutes)
    
    // Grace period for failed payments
    grace_period_hours: u64, // 24 hours
    
    // Auto-retry mechanism
    retry_config: PaymentRetryConfig,
    
    pub async fn preauthorize_container(&self, container_id: String, duration_hours: u64) -> Result<PreAuth>,
    pub async fn charge_container_usage(&self, container_id: String) -> Result<Payment>,
    pub async fn handle_payment_failure(&self, container_id: String) -> Result<GracePeriodExtension>,
}
```

**Payment Stability Features**:
- **Pre-authorization**: Reserve funds before container starts
- **Payment Buffer**: 5-minute grace period during payment processing
- **Auto-retry**: 3 attempts with exponential backoff
- **Grace Period**: 24 hours for failed payments
- **Soft Shutdown**: Graceful container termination with data backup

---

### **3. FREE CREDIT SYSTEM (TESTNET)**

**Admin-Controlled Free Credits**:

```rust
pub struct FreeCreditManager {
    // Credit pools
    testnet_pool: Arc<RwLock<CreditPool>>,
    
    // Admin controls
    pub async fn grant_free_credits(
        &self,
        user_id: String,
        amount: u64, // e.g., 5000 BPI tokens
        reason: String,
    ) -> Result<CreditGrant>,
    
    pub async fn revoke_credits(
        &self,
        user_id: String,
        amount: u64,
    ) -> Result<CreditRevocation>,
    
    pub async fn set_testnet_default(
        &self,
        default_amount: u64, // e.g., 5000 BPI
    ) -> Result<()>,
    
    // Automatic grants
    pub async fn auto_grant_on_signup(
        &self,
        user_id: String,
    ) -> Result<CreditGrant>,
}

pub struct CreditPool {
    total_allocated: u64,
    total_used: u64,
    total_remaining: u64,
    grants: HashMap<String, Vec<CreditGrant>>,
}

pub struct CreditGrant {
    grant_id: String,
    user_id: String,
    amount: u64,
    granted_at: DateTime<Utc>,
    expires_at: Option<DateTime<Utc>>,
    granted_by: String, // Admin ID
    reason: String,
    status: CreditStatus, // Active, Expired, Revoked
}
```

**Testnet Credit Rules**:
- **Free Tier**: 1000 BPI tokens on signup (free for dev and test BPI OS)
- **After Free Tier**: Paid plans (see existing charge plans in codebase)
- **Admin Grant**: Admin can give up to 5000 BPI free to any BPI wallet address via admin server
- **Mainnet Features**: Roundtable, autonomous economy, and higher orchestration/security will be setup in mainnet (not testnet)
- **Expiration**: Optional (e.g., 30 days for testnet)
- **Usage Tracking**: Full audit trail
- **Conversion**: Cannot convert to real money

**Important Notes**:
- Testnet is NOT completely free - only first 1000 BPI is free
- After 1000 BPI used, users must use existing paid charge plans
- Admin can override and grant up to 5000 BPI free using admin server
- Advanced features (roundtable, autonomous economy, higher security) are mainnet-only

---

### **4. ADMIN CONTROL PANEL**

**Multi-Level Admin System**:

```rust
pub enum AdminRole {
    SuperAdmin,     // Full control (owner)
    FinanceAdmin,   // Payment and billing
    SupportAdmin,   // User support and credits
    TechnicalAdmin, // Service management
    ViewOnly,       // Read-only access
}

pub struct AdminAuthSystem {
    // Multi-factor authentication
    mfa_required: bool,
    
    // Role-based access control
    pub async fn authenticate_admin(
        &self,
        username: String,
        password: String,
        mfa_token: Option<String>,
    ) -> Result<AdminSession>,
    
    pub async fn check_permission(
        &self,
        admin_id: String,
        action: AdminAction,
    ) -> Result<bool>,
    
    // Audit logging
    pub async fn log_admin_action(
        &self,
        admin_id: String,
        action: AdminAction,
        details: serde_json::Value,
    ) -> Result<()>,
}
```

**Admin Dashboard Features**:

1. **Service Management**
   - Start/stop all 13 BPCI services
   - View real-time status
   - Configure DynaRoute settings
   - Monitor Quantum Heartbeat

2. **Payment Management**
   - View all transactions
   - Process refunds
   - Handle disputes
   - Configure pricing plans
   - Stripe webhook management

3. **User Management**
   - Grant/revoke free credits
   - Upgrade/downgrade accounts
   - View usage statistics
   - Suspend/unsuspend accounts

4. **Analytics Dashboard**
   - Revenue metrics
   - User growth
   - Service usage
   - Payment success rate
   - Container utilization

5. **System Health**
   - All 13 services status
   - DynaRoute mesh health
   - Quantum Heartbeat monitoring
   - Resource usage (CPU/Memory)
   - Alert management

---

### **5. PRICING PLANS**

**Testnet Plans** (with free credits):

```rust
pub fn create_testnet_plans() -> Vec<PricingPlan> {
    vec![
        PricingPlan {
            name: "Testnet Free".to_string(),
            monthly_cost_cad: 0.0,
            monthly_cost_usd: 0.0,
            free_credits: 5000, // 5000 BPI tokens
            included_hours: 100, // 100 hours VM time
            overage_rate_per_hour: 0.0, // Free overage in testnet
            features: vec![
                "Full BPCI access",
                "DynaRoute mesh networking",
                "Quantum Heartbeat monitoring",
                "Community support",
            ],
        },
        PricingPlan {
            name: "Testnet Developer".to_string(),
            monthly_cost_cad: 10.0,
            monthly_cost_usd: 7.50,
            free_credits: 10000, // 10000 BPI tokens
            included_hours: 500,
            overage_rate_per_hour: 0.01,
            features: vec![
                "Everything in Free",
                "Priority support",
                "Extended VM time",
                "API access",
            ],
        },
    ]
}
```

**Production Plans** (Stripe integration):

```rust
pub fn create_production_plans() -> Vec<PricingPlan> {
    vec![
        PricingPlan {
            name: "Starter".to_string(),
            monthly_cost_cad: 99.0,
            monthly_cost_usd: 75.0,
            stripe_price_id: "price_starter_monthly",
            included_hours: 1000,
            overage_rate_per_hour: 0.10,
        },
        PricingPlan {
            name: "Professional".to_string(),
            monthly_cost_cad: 299.0,
            monthly_cost_usd: 225.0,
            stripe_price_id: "price_pro_monthly",
            included_hours: 5000,
            overage_rate_per_hour: 0.06,
        },
        PricingPlan {
            name: "Enterprise".to_string(),
            monthly_cost_cad: 999.0,
            monthly_cost_usd: 750.0,
            stripe_price_id: "price_enterprise_monthly",
            included_hours: 20000,
            overage_rate_per_hour: 0.05,
            features: vec![
                "Dedicated support",
                "Custom SLA",
                "White-label option",
            ],
        },
    ]
}
```

---

### **6. CONTAINER PAYMENT SYSTEM**

**Payment Stability Architecture**:

```rust
pub struct ContainerBillingEngine {
    // Real-time usage tracking
    usage_tracker: Arc<UsageTracker>,
    
    // Payment processing
    payment_processor: Arc<StripePaymentProcessor>,
    
    // Stability features
    pub async fn start_container_with_preauth(
        &self,
        user_id: String,
        container_spec: ContainerSpec,
        estimated_duration_hours: u64,
    ) -> Result<ContainerSession> {
        // 1. Calculate estimated cost
        let estimated_cost = self.calculate_cost(container_spec, estimated_duration_hours);
        
        // 2. Pre-authorize payment
        let preauth = self.payment_processor
            .preauthorize(user_id.clone(), estimated_cost * 1.2) // 20% buffer
            .await?;
        
        // 3. Start container
        let container = self.start_container(container_spec).await?;
        
        // 4. Track usage in real-time
        self.usage_tracker.start_tracking(container.id.clone(), preauth).await?;
        
        Ok(ContainerSession {
            container_id: container.id,
            preauth_id: preauth.id,
            started_at: Utc::now(),
            estimated_cost,
        })
    },
    
    pub async fn handle_payment_failure(
        &self,
        container_id: String,
    ) -> Result<GracePeriodResponse> {
        // 1. Enter grace period (24 hours)
        let grace_period = self.create_grace_period(container_id.clone()).await?;
        
        // 2. Notify user
        self.notify_user_payment_failure(container_id.clone()).await?;
        
        // 3. Schedule retry
        self.schedule_payment_retry(container_id.clone(), 3).await?;
        
        // 4. If all retries fail, graceful shutdown
        if grace_period.retries_exhausted {
            self.graceful_container_shutdown(container_id).await?;
        }
        
        Ok(grace_period)
    },
}
```

**Payment Retry Strategy**:
- **Retry 1**: After 1 hour
- **Retry 2**: After 6 hours
- **Retry 3**: After 12 hours
- **Final**: Graceful shutdown after 24 hours

---

### **7. BPI OS LOGBOOK / AUDIT SYSTEM**

**Purpose**: Comprehensive audit book to track all gas, rent, blocks, and transactions from all BPI OS instances connecting to the network.

#### **Logbook Architecture**:

```rust
pub struct BpiOsLogbook {
    // Real-time tracking
    gas_tracker: Arc<GasUsageTracker>,
    rent_tracker: Arc<RentPaymentTracker>,
    block_tracker: Arc<BlockActivityTracker>,
    transaction_tracker: Arc<TransactionAuditTracker>,
    
    // Aggregation
    network_aggregator: Arc<NetworkAggregator>,
    
    // Storage
    audit_storage: Arc<AuditStorage>,
}

/// Gas Usage Tracking for All BPI OS Instances
pub struct GasUsageTracker {
    pub async fn record_gas_usage(
        &self,
        bpi_os_id: String,
        transaction_id: String,
        gas_amount: u64,
        gas_price: Decimal,
        timestamp: DateTime<Utc>,
    ) -> Result<GasRecord>,
    
    pub async fn get_gas_usage_by_bpi_os(
        &self,
        bpi_os_id: String,
        time_range: TimeRange,
    ) -> Result<Vec<GasRecord>>,
    
    pub async fn get_total_network_gas(
        &self,
        time_range: TimeRange,
    ) -> Result<NetworkGasMetrics>,
}

/// Rent Payment Tracking for All BPI OS Instances
pub struct RentPaymentTracker {
    pub async fn record_rent_payment(
        &self,
        bpi_os_id: String,
        session_id: String,
        amount: Decimal,
        duration_hours: u64,
        timestamp: DateTime<Utc>,
    ) -> Result<RentRecord>,
    
    pub async fn get_rent_history_by_bpi_os(
        &self,
        bpi_os_id: String,
        time_range: TimeRange,
    ) -> Result<Vec<RentRecord>>,
    
    pub async fn get_active_rent_sessions(
        &self,
    ) -> Result<Vec<ActiveRentSession>>,
}

/// Block Activity Tracking for All BPI OS Instances
pub struct BlockActivityTracker {
    pub async fn record_block_activity(
        &self,
        bpi_os_id: String,
        block_number: u64,
        block_hash: String,
        transaction_count: u64,
        timestamp: DateTime<Utc>,
    ) -> Result<BlockRecord>,
    
    pub async fn get_blocks_by_bpi_os(
        &self,
        bpi_os_id: String,
        time_range: TimeRange,
    ) -> Result<Vec<BlockRecord>>,
    
    pub async fn get_network_block_stats(
        &self,
        time_range: TimeRange,
    ) -> Result<NetworkBlockStats>,
}

/// Transaction Audit Tracking for All BPI OS Instances
pub struct TransactionAuditTracker {
    pub async fn record_transaction(
        &self,
        bpi_os_id: String,
        tx_hash: String,
        tx_type: TransactionType,
        from_address: String,
        to_address: String,
        amount: Decimal,
        gas_used: u64,
        status: TransactionStatus,
        timestamp: DateTime<Utc>,
    ) -> Result<TransactionRecord>,
    
    pub async fn get_transactions_by_bpi_os(
        &self,
        bpi_os_id: String,
        time_range: TimeRange,
    ) -> Result<Vec<TransactionRecord>>,
    
    pub async fn get_network_transaction_stats(
        &self,
        time_range: TimeRange,
    ) -> Result<NetworkTransactionStats>,
}
```

#### **Logbook Data Models**:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasRecord {
    pub id: String,
    pub bpi_os_id: String,
    pub bpi_os_name: String,
    pub transaction_id: String,
    pub gas_amount: u64,
    pub gas_price: Decimal,
    pub total_cost: Decimal,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RentRecord {
    pub id: String,
    pub bpi_os_id: String,
    pub bpi_os_name: String,
    pub session_id: String,
    pub amount: Decimal,
    pub duration_hours: u64,
    pub hourly_rate: Decimal,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub status: RentStatus, // Active, Completed, Failed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRecord {
    pub id: String,
    pub bpi_os_id: String,
    pub bpi_os_name: String,
    pub block_number: u64,
    pub block_hash: String,
    pub transaction_count: u64,
    pub gas_used: u64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRecord {
    pub id: String,
    pub bpi_os_id: String,
    pub bpi_os_name: String,
    pub tx_hash: String,
    pub tx_type: TransactionType, // Transfer, Contract, Deployment
    pub from_address: String,
    pub to_address: String,
    pub amount: Decimal,
    pub gas_used: u64,
    pub gas_price: Decimal,
    pub status: TransactionStatus, // Pending, Confirmed, Failed
    pub block_number: Option<u64>,
    pub timestamp: DateTime<Utc>,
}
```

#### **Network Aggregation**:

```rust
pub struct NetworkAggregator {
    pub async fn get_network_summary(
        &self,
        time_range: TimeRange,
    ) -> Result<NetworkSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkSummary {
    // BPI OS Instances
    pub total_bpi_os_instances: u64,
    pub active_bpi_os_instances: u64,
    
    // Gas Metrics
    pub total_gas_used: u64,
    pub total_gas_cost: Decimal,
    pub average_gas_price: Decimal,
    
    // Rent Metrics
    pub total_rent_collected: Decimal,
    pub active_rent_sessions: u64,
    pub total_runtime_hours: u64,
    
    // Block Metrics
    pub total_blocks: u64,
    pub average_block_time: f64,
    pub total_transactions: u64,
    
    // Transaction Metrics
    pub transactions_per_second: f64,
    pub success_rate: f64,
    pub failed_transactions: u64,
    
    // Time Range
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}
```

#### **Logbook API Endpoints**:

```
# BPI OS Logbook (Admin & User Access)

# Gas Tracking
GET    /api/logbook/gas/{bpi_os_id}
GET    /api/logbook/gas/network/summary
GET    /api/logbook/gas/network/stats

# Rent Tracking
GET    /api/logbook/rent/{bpi_os_id}
GET    /api/logbook/rent/active
GET    /api/logbook/rent/network/summary

# Block Tracking
GET    /api/logbook/blocks/{bpi_os_id}
GET    /api/logbook/blocks/network/stats
GET    /api/logbook/blocks/{block_number}

# Transaction Tracking
GET    /api/logbook/transactions/{bpi_os_id}
GET    /api/logbook/transactions/{tx_hash}
GET    /api/logbook/transactions/network/stats

# Network Summary
GET    /api/logbook/network/summary
GET    /api/logbook/network/realtime

# Admin-Only Endpoints
GET    /admin/logbook/all-bpi-os
GET    /admin/logbook/suspicious-activity
GET    /admin/logbook/export
```

#### **Logbook Database Schema**:

```sql
-- Gas Usage Tracking
CREATE TABLE gas_usage_log (
    id UUID PRIMARY KEY,
    bpi_os_id VARCHAR(255) NOT NULL,
    bpi_os_name VARCHAR(255),
    transaction_id VARCHAR(255) NOT NULL,
    gas_amount BIGINT NOT NULL,
    gas_price DECIMAL(20, 8) NOT NULL,
    total_cost DECIMAL(20, 8) NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    INDEX idx_bpi_os_id (bpi_os_id),
    INDEX idx_timestamp (timestamp)
);

-- Rent Payment Tracking
CREATE TABLE rent_payment_log (
    id UUID PRIMARY KEY,
    bpi_os_id VARCHAR(255) NOT NULL,
    bpi_os_name VARCHAR(255),
    session_id VARCHAR(255) NOT NULL,
    amount DECIMAL(20, 8) NOT NULL,
    duration_hours BIGINT NOT NULL,
    hourly_rate DECIMAL(20, 8) NOT NULL,
    started_at TIMESTAMP NOT NULL,
    ended_at TIMESTAMP,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    INDEX idx_bpi_os_id (bpi_os_id),
    INDEX idx_session_id (session_id),
    INDEX idx_status (status)
);

-- Block Activity Tracking
CREATE TABLE block_activity_log (
    id UUID PRIMARY KEY,
    bpi_os_id VARCHAR(255) NOT NULL,
    bpi_os_name VARCHAR(255),
    block_number BIGINT NOT NULL,
    block_hash VARCHAR(255) NOT NULL,
    transaction_count BIGINT NOT NULL,
    gas_used BIGINT NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    INDEX idx_bpi_os_id (bpi_os_id),
    INDEX idx_block_number (block_number),
    INDEX idx_timestamp (timestamp)
);

-- Transaction Audit Tracking
CREATE TABLE transaction_audit_log (
    id UUID PRIMARY KEY,
    bpi_os_id VARCHAR(255) NOT NULL,
    bpi_os_name VARCHAR(255),
    tx_hash VARCHAR(255) UNIQUE NOT NULL,
    tx_type VARCHAR(50) NOT NULL,
    from_address VARCHAR(255) NOT NULL,
    to_address VARCHAR(255) NOT NULL,
    amount DECIMAL(20, 8) NOT NULL,
    gas_used BIGINT NOT NULL,
    gas_price DECIMAL(20, 8) NOT NULL,
    status VARCHAR(50) NOT NULL,
    block_number BIGINT,
    timestamp TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    INDEX idx_bpi_os_id (bpi_os_id),
    INDEX idx_tx_hash (tx_hash),
    INDEX idx_from_address (from_address),
    INDEX idx_to_address (to_address),
    INDEX idx_timestamp (timestamp)
);

-- BPI OS Registry (for tracking all instances)
CREATE TABLE bpi_os_registry (
    id UUID PRIMARY KEY,
    bpi_os_id VARCHAR(255) UNIQUE NOT NULL,
    bpi_os_name VARCHAR(255) NOT NULL,
    owner_user_id UUID REFERENCES users(id),
    status VARCHAR(50) NOT NULL, -- Active, Inactive, Suspended
    first_seen TIMESTAMP NOT NULL,
    last_seen TIMESTAMP NOT NULL,
    total_gas_used BIGINT DEFAULT 0,
    total_rent_paid DECIMAL(20, 8) DEFAULT 0,
    total_blocks BIGINT DEFAULT 0,
    total_transactions BIGINT DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW()
);
```

#### **Real-Time Logbook Dashboard**:

**Features**:
1. **Live Activity Feed**: Real-time stream of all network activity
2. **BPI OS List**: All connected BPI OS instances with status
3. **Gas Usage Chart**: Real-time gas consumption graph
4. **Rent Sessions**: Active and completed rent sessions
5. **Block Explorer**: Browse blocks and transactions
6. **Network Health**: Overall network statistics
7. **Suspicious Activity Alerts**: Unusual patterns detection

**Admin Dashboard Views**:
- All BPI OS instances (with search/filter)
- Gas usage by BPI OS (sortable)
- Rent payment history (with export)
- Block activity timeline
- Transaction explorer
- Network analytics
- Audit trail export (CSV/JSON)

#### **Logbook Integration with Existing Systems**:

```rust
// Integration with BPI Bridge
impl BpiBridge {
    async fn on_transaction(&self, tx: Transaction) {
        // Record in logbook
        self.logbook.record_transaction(
            tx.bpi_os_id,
            tx.hash,
            tx.tx_type,
            tx.from,
            tx.to,
            tx.amount,
            tx.gas_used,
            tx.status,
            Utc::now(),
        ).await?;
    }
}

// Integration with Blockchain Server
impl BlockchainServer {
    async fn on_new_block(&self, block: Block) {
        // Record in logbook
        self.logbook.record_block_activity(
            block.bpi_os_id,
            block.number,
            block.hash,
            block.transaction_count,
            Utc::now(),
        ).await?;
    }
}

// Integration with Payment System
impl PaymentSystem {
    async fn on_rent_payment(&self, payment: RentPayment) {
        // Record in logbook
        self.logbook.record_rent_payment(
            payment.bpi_os_id,
            payment.session_id,
            payment.amount,
            payment.duration_hours,
            Utc::now(),
        ).await?;
    }
}
```

---

### **8. ADMIN PORTAL - DIRECT INFRASTRUCTURE ACCESS**

**Purpose**: Special admin portal with unlimited BPI transactions for enterprise projects and admin testing.

#### **Admin Portal Architecture**:

```rust
pub struct AdminPortal {
    // Direct infrastructure access
    infra_controller: Arc<InfrastructureController>,
    
    // Unlimited BPI system
    unlimited_bpi_manager: Arc<UnlimitedBpiManager>,
    
    // Project approval system
    enterprise_project_manager: Arc<EnterpriseProjectManager>,
    
    // Admin testing environment
    admin_testing_env: Arc<AdminTestingEnvironment>,
}

/// Unlimited BPI Manager for Admin-Approved Projects
pub struct UnlimitedBpiManager {
    // Approved projects with unlimited BPI
    approved_projects: Arc<RwLock<HashMap<String, UnlimitedBpiProject>>>,
    
    // Admin testing projects
    admin_test_projects: Arc<RwLock<HashMap<String, AdminTestProject>>>,
    
    pub async fn approve_enterprise_project(
        &self,
        admin_id: String,
        project_id: String,
        project_details: ProjectDetails,
    ) -> Result<UnlimitedBpiProject>,
    
    pub async fn create_admin_test_project(
        &self,
        admin_id: String,
        project_name: String,
    ) -> Result<AdminTestProject>,
    
    pub async fn check_unlimited_access(
        &self,
        wallet_address: String,
    ) -> Result<bool>,
    
    pub async fn track_unlimited_usage(
        &self,
        project_id: String,
        operation: Operation,
    ) -> Result<UsageRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlimitedBpiProject {
    pub project_id: String,
    pub project_name: String,
    pub project_type: ProjectType, // Enterprise, AdminTest
    pub wallet_addresses: Vec<String>,
    pub approved_by: String, // Admin ID
    pub approved_at: DateTime<Utc>,
    pub status: ProjectStatus, // Active, Suspended, Completed
    pub unlimited_bpi: bool, // Always true
    pub usage_tracking: UsageTracking,
    pub restrictions: Option<ProjectRestrictions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectType {
    EnterpriseApproved,  // Admin-approved enterprise project
    AdminTesting,        // Admin testing during development
    SpecialGrant,        // Special case unlimited access
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRestrictions {
    pub max_concurrent_containers: Option<u32>,
    pub max_daily_operations: Option<u64>,
    pub allowed_operations: Option<Vec<String>>,
    pub expiration_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageTracking {
    pub total_gas_used: u64,
    pub total_rent_hours: u64,
    pub total_operations: u64,
    pub total_cost_equivalent: Decimal, // Track cost even though unlimited
    pub last_activity: DateTime<Utc>,
}
```

#### **Direct Infrastructure Access**:

```rust
pub struct InfrastructureController {
    // Direct access to all 13 BPCI services
    pub async fn direct_service_access(
        &self,
        admin_id: String,
        service_name: String,
        action: ServiceAction,
    ) -> Result<ServiceResponse>,
    
    // Bypass normal billing for unlimited projects
    pub async fn execute_unlimited_operation(
        &self,
        project_id: String,
        operation: Operation,
    ) -> Result<OperationResult> {
        // 1. Verify unlimited access
        if !self.verify_unlimited_access(project_id).await? {
            return Err("Project does not have unlimited access");
        }
        
        // 2. Execute operation without billing
        let result = self.execute_operation_no_billing(operation).await?;
        
        // 3. Track usage for monitoring (but don't charge)
        self.track_unlimited_usage(project_id, operation, result.clone()).await?;
        
        Ok(result)
    },
    
    // Direct container deployment (no billing)
    pub async fn deploy_container_unlimited(
        &self,
        project_id: String,
        container_spec: ContainerSpec,
    ) -> Result<Container>,
    
    // Direct VM access (no rent charges)
    pub async fn start_vm_unlimited(
        &self,
        project_id: String,
        vm_spec: VmSpec,
    ) -> Result<VmInstance>,
}
```

#### **Enterprise Project Approval System**:

```rust
pub struct EnterpriseProjectManager {
    pub async fn submit_enterprise_project(
        &self,
        user_id: String,
        project_proposal: ProjectProposal,
    ) -> Result<ProjectSubmission>,
    
    pub async fn review_project(
        &self,
        admin_id: String,
        project_id: String,
        decision: ApprovalDecision,
    ) -> Result<ProjectApproval>,
    
    pub async fn approve_unlimited_access(
        &self,
        admin_id: String,
        project_id: String,
        restrictions: Option<ProjectRestrictions>,
    ) -> Result<UnlimitedBpiProject> {
        // 1. Verify admin permissions
        self.verify_admin_permission(admin_id.clone(), AdminAction::ApproveUnlimitedBpi)?;
        
        // 2. Create unlimited BPI project
        let project = UnlimitedBpiProject {
            project_id: project_id.clone(),
            project_type: ProjectType::EnterpriseApproved,
            approved_by: admin_id,
            approved_at: Utc::now(),
            unlimited_bpi: true,
            restrictions,
            ..Default::default()
        };
        
        // 3. Register project
        self.register_unlimited_project(project.clone()).await?;
        
        // 4. Notify project owner
        self.notify_project_approval(project_id).await?;
        
        Ok(project)
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectProposal {
    pub project_name: String,
    pub description: String,
    pub use_case: String,
    pub estimated_duration: String,
    pub team_size: u32,
    pub wallet_addresses: Vec<String>,
    pub justification: String, // Why unlimited BPI is needed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApprovalDecision {
    Approved,
    ApprovedWithRestrictions(ProjectRestrictions),
    Rejected(String), // Rejection reason
    NeedsMoreInfo(String),
}
```

#### **Admin Testing Environment**:

```rust
pub struct AdminTestingEnvironment {
    // Create isolated testing environment
    pub async fn create_test_environment(
        &self,
        admin_id: String,
        env_name: String,
    ) -> Result<TestEnvironment>,
    
    // Unlimited BPI for admin testing
    pub async fn enable_unlimited_testing(
        &self,
        admin_id: String,
        test_env_id: String,
    ) -> Result<AdminTestProject> {
        let project = AdminTestProject {
            project_id: test_env_id.clone(),
            project_name: format!("Admin Test: {}", env_name),
            project_type: ProjectType::AdminTesting,
            admin_id: admin_id.clone(),
            created_at: Utc::now(),
            unlimited_bpi: true,
            auto_cleanup: true, // Auto-cleanup after testing
            ..Default::default()
        };
        
        self.register_test_project(project.clone()).await?;
        
        Ok(project)
    },
    
    // Cleanup test environment
    pub async fn cleanup_test_environment(
        &self,
        test_env_id: String,
    ) -> Result<CleanupReport>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminTestProject {
    pub project_id: String,
    pub project_name: String,
    pub project_type: ProjectType,
    pub admin_id: String,
    pub created_at: DateTime<Utc>,
    pub unlimited_bpi: bool,
    pub auto_cleanup: bool,
    pub usage_tracking: UsageTracking,
}
```

#### **Admin Portal API Endpoints**:

```
# Admin Portal (SuperAdmin only)

# Unlimited BPI Management
POST   /admin/portal/unlimited/approve-enterprise
POST   /admin/portal/unlimited/create-test-project
GET    /admin/portal/unlimited/projects
GET    /admin/portal/unlimited/projects/{project_id}
POST   /admin/portal/unlimited/projects/{project_id}/suspend
POST   /admin/portal/unlimited/projects/{project_id}/resume
DELETE /admin/portal/unlimited/projects/{project_id}

# Enterprise Project Approval
POST   /admin/portal/enterprise/submit
GET    /admin/portal/enterprise/pending
POST   /admin/portal/enterprise/{project_id}/review
POST   /admin/portal/enterprise/{project_id}/approve
POST   /admin/portal/enterprise/{project_id}/reject

# Direct Infrastructure Access
POST   /admin/portal/infra/service/{service_name}/action
POST   /admin/portal/infra/container/deploy-unlimited
POST   /admin/portal/infra/vm/start-unlimited
GET    /admin/portal/infra/status

# Admin Testing
POST   /admin/portal/testing/create-environment
POST   /admin/portal/testing/{env_id}/enable-unlimited
DELETE /admin/portal/testing/{env_id}/cleanup
GET    /admin/portal/testing/environments

# Usage Tracking (for unlimited projects)
GET    /admin/portal/usage/{project_id}
GET    /admin/portal/usage/{project_id}/export
GET    /admin/portal/usage/network/summary
```

#### **Admin Portal Dashboard Features**:

**1. Unlimited BPI Projects Management:**
- List all unlimited BPI projects
- Approve/reject enterprise requests
- Create admin test projects
- Monitor usage (even though unlimited)
- Suspend/resume projects
- Set optional restrictions

**2. Direct Infrastructure Control:**
- One-click service management
- Direct container deployment
- Direct VM access
- Bypass all billing checks
- Real-time service monitoring

**3. Enterprise Project Approval Workflow:**
- Review project proposals
- Approve with/without restrictions
- Set expiration dates
- Assign wallet addresses
- Track project lifecycle

**4. Admin Testing Tools:**
- Create isolated test environments
- Unlimited BPI for testing
- Auto-cleanup after testing
- Test data generation
- Performance testing tools

**5. Usage Analytics (Monitoring):**
- Track unlimited project usage
- Cost equivalent calculations
- Resource utilization
- Performance metrics
- Abuse detection

#### **Security & Audit**:

```rust
pub struct AdminPortalSecurity {
    // Multi-factor authentication required
    pub async fn verify_admin_mfa(
        &self,
        admin_id: String,
        mfa_token: String,
    ) -> Result<bool>,
    
    // Audit all unlimited BPI operations
    pub async fn audit_unlimited_operation(
        &self,
        project_id: String,
        operation: Operation,
        admin_id: String,
    ) -> Result<AuditRecord> {
        let record = AuditRecord {
            timestamp: Utc::now(),
            project_id,
            operation_type: operation.operation_type,
            admin_id,
            cost_equivalent: operation.calculate_cost(),
            unlimited_access: true,
            ..Default::default()
        };
        
        self.store_audit_record(record.clone()).await?;
        
        Ok(record)
    },
    
    // Detect abuse of unlimited access
    pub async fn detect_abuse(
        &self,
        project_id: String,
    ) -> Result<AbuseReport>,
}
```

#### **Database Schema for Admin Portal**:

```sql
-- Unlimited BPI Projects
CREATE TABLE unlimited_bpi_projects (
    id UUID PRIMARY KEY,
    project_id VARCHAR(255) UNIQUE NOT NULL,
    project_name VARCHAR(255) NOT NULL,
    project_type VARCHAR(50) NOT NULL, -- 'enterprise', 'admin_test', 'special_grant'
    approved_by UUID REFERENCES admins(id),
    approved_at TIMESTAMP NOT NULL,
    status VARCHAR(50) NOT NULL, -- 'active', 'suspended', 'completed'
    unlimited_bpi BOOLEAN DEFAULT true,
    max_concurrent_containers INT,
    max_daily_operations BIGINT,
    expiration_date TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Project Wallet Addresses
CREATE TABLE project_wallet_addresses (
    id UUID PRIMARY KEY,
    project_id UUID REFERENCES unlimited_bpi_projects(id),
    wallet_address VARCHAR(255) NOT NULL,
    added_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(project_id, wallet_address)
);

-- Enterprise Project Proposals
CREATE TABLE enterprise_project_proposals (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    project_name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    use_case TEXT NOT NULL,
    justification TEXT NOT NULL,
    estimated_duration VARCHAR(100),
    team_size INT,
    status VARCHAR(50) NOT NULL, -- 'pending', 'approved', 'rejected'
    reviewed_by UUID REFERENCES admins(id),
    reviewed_at TIMESTAMP,
    decision TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Unlimited Usage Tracking
CREATE TABLE unlimited_usage_tracking (
    id UUID PRIMARY KEY,
    project_id UUID REFERENCES unlimited_bpi_projects(id),
    operation_type VARCHAR(100) NOT NULL,
    cost_equivalent DECIMAL(20, 8) NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    details JSONB,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Admin Portal Audit Log
CREATE TABLE admin_portal_audit (
    id UUID PRIMARY KEY,
    admin_id UUID REFERENCES admins(id),
    action VARCHAR(255) NOT NULL,
    project_id UUID,
    details JSONB,
    ip_address INET,
    created_at TIMESTAMP DEFAULT NOW()
);
```

#### **Admin Portal UI Components**:

**1. Dashboard:**
- Active unlimited projects count
- Total cost equivalent (monitoring)
- Pending enterprise approvals
- Active admin test environments
- Recent unlimited operations

**2. Project Management:**
- Approve/reject enterprise requests
- Create admin test projects
- Monitor project usage
- Set/modify restrictions
- Suspend/resume projects

**3. Direct Infrastructure Access:**
- Service control panel
- Container deployment interface
- VM management console
- Real-time logs viewer
- Performance metrics

**4. Usage Analytics:**
- Cost equivalent tracking
- Resource utilization charts
- Operation breakdown
- Abuse detection alerts
- Export reports

---

### **9. API ENDPOINTS**

**Admin API** (Protected by admin auth):

```
POST   /admin/auth/login
POST   /admin/auth/logout
GET    /admin/auth/verify

# Service Management
GET    /admin/services/status
POST   /admin/services/{service_id}/start
POST   /admin/services/{service_id}/stop
POST   /admin/services/{service_id}/restart

# Credit Management
POST   /admin/credits/grant
POST   /admin/credits/revoke
GET    /admin/credits/pool/status
POST   /admin/credits/pool/configure

# User Management
GET    /admin/users
GET    /admin/users/{user_id}
POST   /admin/users/{user_id}/upgrade
POST   /admin/users/{user_id}/suspend
POST   /admin/users/{user_id}/unsuspend

# Payment Management
GET    /admin/payments
GET    /admin/payments/{payment_id}
POST   /admin/payments/{payment_id}/refund
GET    /admin/payments/disputes
POST   /admin/payments/disputes/{dispute_id}/resolve

# Analytics
GET    /admin/analytics/revenue
GET    /admin/analytics/users
GET    /admin/analytics/usage
GET    /admin/analytics/health
```

**User API** (Public/authenticated):

```
# Authentication
POST   /api/auth/register
POST   /api/auth/login
POST   /api/auth/logout

# Account Management
GET    /api/account
GET    /api/account/balance
GET    /api/account/usage
GET    /api/account/credits

# Payment
POST   /api/payment/setup
POST   /api/payment/method/add
GET    /api/payment/methods
POST   /api/payment/subscribe
POST   /api/payment/cancel

# Container Management
POST   /api/containers/start
GET    /api/containers/{id}/status
POST   /api/containers/{id}/stop
GET    /api/containers/{id}/usage
GET    /api/containers/{id}/cost
```

**Stripe Webhook**:

```
POST   /webhooks/stripe
```

---

### **8. DATABASE SCHEMA**

**Admin Tables**:

```sql
CREATE TABLE admins (
    id UUID PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL,
    mfa_enabled BOOLEAN DEFAULT false,
    mfa_secret VARCHAR(255),
    created_at TIMESTAMP DEFAULT NOW(),
    last_login TIMESTAMP
);

CREATE TABLE admin_audit_log (
    id UUID PRIMARY KEY,
    admin_id UUID REFERENCES admins(id),
    action VARCHAR(255) NOT NULL,
    details JSONB,
    ip_address INET,
    created_at TIMESTAMP DEFAULT NOW()
);
```

**Credit Tables**:

```sql
CREATE TABLE credit_pools (
    id UUID PRIMARY KEY,
    pool_type VARCHAR(50) NOT NULL, -- 'testnet', 'promotional'
    total_allocated BIGINT NOT NULL,
    total_used BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE credit_grants (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    pool_id UUID REFERENCES credit_pools(id),
    amount BIGINT NOT NULL,
    granted_by UUID REFERENCES admins(id),
    reason TEXT,
    status VARCHAR(50) NOT NULL, -- 'active', 'expired', 'revoked'
    expires_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW()
);
```

**Payment Tables**:

```sql
CREATE TABLE stripe_customers (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    stripe_customer_id VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE subscriptions (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    stripe_subscription_id VARCHAR(255) UNIQUE NOT NULL,
    plan_name VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL,
    current_period_start TIMESTAMP,
    current_period_end TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE payments (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    stripe_payment_intent_id VARCHAR(255) UNIQUE,
    amount DECIMAL(10, 2) NOT NULL,
    currency VARCHAR(3) NOT NULL,
    status VARCHAR(50) NOT NULL,
    container_id UUID,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE payment_retries (
    id UUID PRIMARY KEY,
    payment_id UUID REFERENCES payments(id),
    attempt_number INT NOT NULL,
    status VARCHAR(50) NOT NULL,
    error_message TEXT,
    next_retry_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW()
);
```

**Container Billing Tables**:

```sql
CREATE TABLE container_sessions (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    container_id VARCHAR(255) NOT NULL,
    preauth_id VARCHAR(255),
    started_at TIMESTAMP NOT NULL,
    stopped_at TIMESTAMP,
    estimated_cost DECIMAL(10, 2),
    actual_cost DECIMAL(10, 2),
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE container_usage (
    id UUID PRIMARY KEY,
    session_id UUID REFERENCES container_sessions(id),
    timestamp TIMESTAMP NOT NULL,
    cpu_usage DECIMAL(5, 2),
    memory_usage BIGINT,
    network_in BIGINT,
    network_out BIGINT,
    cost_increment DECIMAL(10, 2)
);
```

---

### **9. SECURITY FEATURES**

1. **Admin Authentication**
   - Multi-factor authentication (TOTP)
   - IP whitelisting
   - Session timeout (30 minutes)
   - Audit logging of all actions

2. **Payment Security**
   - PCI DSS compliance via Stripe
   - No credit card storage
   - Webhook signature verification
   - Fraud detection

3. **API Security**
   - Rate limiting (100 req/min per user)
   - JWT authentication
   - CORS configuration
   - HTTPS only

4. **Data Protection**
   - Encryption at rest
   - Encryption in transit (TLS 1.3)
   - Regular backups
   - GDPR compliance

---

### **10. MONITORING & ALERTS**

**Real-time Monitoring**:

```rust
pub struct MonitoringDashboard {
    // Service health
    pub async fn get_all_services_status() -> ServiceHealthReport,
    
    // Payment health
    pub async fn get_payment_success_rate() -> PaymentMetrics,
    
    // User metrics
    pub async fn get_active_users() -> UserMetrics,
    
    // Container metrics
    pub async fn get_container_utilization() -> ContainerMetrics,
    
    // Quantum Heartbeat
    pub async fn get_heartbeat_status() -> HeartbeatStatus,
}
```

**Alert System**:
- Payment failure rate > 5%
- Service down > 5 minutes
- Quantum Heartbeat missed
- High resource usage (>80%)
- Suspicious admin activity

---

## 🚀 IMPLEMENTATION PHASES

### **Phase 1: Core Admin Server (Week 1-2)**
- Admin authentication system
- Service management API
- Basic monitoring dashboard

### **Phase 2: Payment Integration (Week 3-4)**
- Stripe integration
- Payment processing
- Webhook handling
- Refund system

### **Phase 3: Free Credit System (Week 5)**
- Credit pool management
- Admin credit controls
- Automatic testnet grants
- Usage tracking

### **Phase 4: Container Payment Stability (Week 6-7)**
- Pre-authorization system
- Payment retry logic
- Grace period handling
- Graceful shutdown

### **Phase 5: Admin Dashboard UI (Week 8-9)**
- React/Vue dashboard
- Real-time updates
- Analytics visualizations
- User management interface

### **Phase 6: Production Deployment (Week 10)**
- Security hardening
- Load testing
- Documentation
- Training

---

## 📊 SUCCESS METRICS

- **Payment Success Rate**: >95%
- **Container Uptime**: >99.9%
- **Payment Processing Time**: <2 seconds
- **Admin Response Time**: <500ms
- **Free Credit Utilization**: >60%
- **User Satisfaction**: >4.5/5

---

## 🎯 CONCLUSION

This Central BPCI Server design provides:
- ✅ Complete owner control
- ✅ Stripe payment integration
- ✅ Admin-controlled free credits (5000 BPI testnet)
- ✅ Container payment stability
- ✅ Multi-tenant management
- ✅ Production-ready architecture

**Next Steps**: Review design → Approve → Begin Phase 1 implementation

---

**Document Status**: Ready for Review  
**Estimated Implementation**: 10 weeks  
**Team Size**: 2-3 developers  
**Budget**: TBD based on Stripe fees and infrastructure costs
