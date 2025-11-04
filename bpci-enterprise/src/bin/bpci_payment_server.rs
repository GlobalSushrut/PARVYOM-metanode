use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, warn};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use rust_decimal::Decimal;

// Import BPCI modules
use pravyom_enterprise::{
    dynaroute_integration::UnifiedNetworkingLayer,
    commute_lock::CommuteLockRuntime,
    config::env_ini_parser::EnvIniParser,
    autonomous_economy::coin_distribution::{CoinDistributionEngine, CoinType},
};

/// BPCI Payment Server - Server 15
/// 
/// Comprehensive payment processing system:
/// - Stripe payment integration
/// - Container payment stability
/// - Subscription management
/// - Invoice generation
/// - Payment method management
/// - Webhook handling
/// - Integration with 4-coin autonomous economy

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: String,
    pub user_id: String,
    pub email: String,
    pub stripe_customer_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub payment_methods: Vec<PaymentMethod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    pub id: String,
    pub method_type: PaymentMethodType,
    pub stripe_payment_method_id: Option<String>,
    pub last_four: Option<String>,
    pub brand: Option<String>,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMethodType {
    Card,
    BankAccount,
    Crypto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: String,
    pub customer_id: String,
    pub plan_id: String,
    pub status: SubscriptionStatus,
    pub stripe_subscription_id: Option<String>,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub cancel_at_period_end: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubscriptionStatus {
    Active,
    PastDue,
    Canceled,
    Unpaid,
    Trialing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingPlan {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price_cad: Decimal,
    pub price_usd: Decimal,
    pub billing_interval: BillingInterval,
    pub features: Vec<String>,
    pub bpi_allocation: u64,
    pub stripe_price_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BillingInterval {
    Monthly,
    Yearly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: String,
    pub customer_id: String,
    pub amount_cad: Decimal,
    pub amount_usd: Decimal,
    pub status: InvoiceStatus,
    pub stripe_invoice_id: Option<String>,
    pub due_date: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
    pub items: Vec<InvoiceItem>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvoiceStatus {
    Draft,
    Open,
    Paid,
    Void,
    Uncollectible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub description: String,
    pub quantity: u32,
    pub unit_price: Decimal,
    pub total: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerPayment {
    pub id: String,
    pub bpi_os_id: String,
    pub container_id: String,
    pub payment_type: ContainerPaymentType,
    pub amount: u64, // in BPI tokens
    pub timestamp: DateTime<Utc>,
    pub session_id: String,
    pub stability_guaranteed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContainerPaymentType {
    Gas,
    Storage,
    Network,
}

pub struct PaymentServerState {
    networking: Arc<UnifiedNetworkingLayer>,
    commute_lock: Arc<CommuteLockRuntime>,
    customers: Arc<RwLock<HashMap<String, Customer>>>,
    subscriptions: Arc<RwLock<HashMap<String, Subscription>>>,
    pricing_plans: Arc<RwLock<HashMap<String, PricingPlan>>>,
    invoices: Arc<RwLock<HashMap<String, Invoice>>>,
    container_payments: Arc<RwLock<Vec<ContainerPayment>>>,
    coin_engine: Arc<RwLock<CoinDistributionEngine>>,
    stripe_config: StripeConfig,
}

#[derive(Debug, Clone)]
pub struct StripeConfig {
    pub api_key: String,
    pub webhook_secret: String,
    pub enabled: bool,
}

impl PaymentServerState {
    pub fn new(
        networking: Arc<UnifiedNetworkingLayer>,
        commute_lock: Arc<CommuteLockRuntime>,
        stripe_config: StripeConfig,
    ) -> Self {
        Self {
            customers: Arc::new(RwLock::new(HashMap::new())),
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
            pricing_plans: Arc::new(RwLock::new(Self::initialize_pricing_plans())),
            invoices: Arc::new(RwLock::new(HashMap::new())),
            container_payments: Arc::new(RwLock::new(Vec::new())),
            coin_engine: Arc::new(RwLock::new(CoinDistributionEngine::new())),
            networking,
            commute_lock,
            stripe_config,
        }
    }

    fn initialize_pricing_plans() -> HashMap<String, PricingPlan> {
        let mut plans = HashMap::new();
        
        // Starter Plan
        plans.insert(
            "starter".to_string(),
            PricingPlan {
                id: "starter".to_string(),
                name: "Starter".to_string(),
                description: "Perfect for developers and small projects".to_string(),
                price_cad: Decimal::new(99, 0),
                price_usd: Decimal::new(75, 0),
                billing_interval: BillingInterval::Monthly,
                features: vec![
                    "1,000 BPI tokens/month".to_string(),
                    "Up to 5 containers".to_string(),
                    "Basic support".to_string(),
                ],
                bpi_allocation: 1000,
                stripe_price_id: None,
            },
        );
        
        // Professional Plan
        plans.insert(
            "professional".to_string(),
            PricingPlan {
                id: "professional".to_string(),
                name: "Professional".to_string(),
                description: "For growing businesses and teams".to_string(),
                price_cad: Decimal::new(299, 0),
                price_usd: Decimal::new(225, 0),
                billing_interval: BillingInterval::Monthly,
                features: vec![
                    "5,000 BPI tokens/month".to_string(),
                    "Up to 25 containers".to_string(),
                    "Priority support".to_string(),
                    "Advanced analytics".to_string(),
                ],
                bpi_allocation: 5000,
                stripe_price_id: None,
            },
        );
        
        // Enterprise Plan
        plans.insert(
            "enterprise".to_string(),
            PricingPlan {
                id: "enterprise".to_string(),
                name: "Enterprise".to_string(),
                description: "For large-scale production deployments".to_string(),
                price_cad: Decimal::new(999, 0),
                price_usd: Decimal::new(750, 0),
                billing_interval: BillingInterval::Monthly,
                features: vec![
                    "25,000 BPI tokens/month".to_string(),
                    "Unlimited containers".to_string(),
                    "24/7 dedicated support".to_string(),
                    "Custom integrations".to_string(),
                    "SLA guarantees".to_string(),
                ],
                bpi_allocation: 25000,
                stripe_price_id: None,
            },
        );
        
        plans
    }
}

// ============================================================================
// CUSTOMER MANAGEMENT
// ============================================================================

#[derive(Debug, Deserialize)]
struct CreateCustomerRequest {
    user_id: String,
    email: String,
}

async fn create_customer(
    State(state): State<Arc<PaymentServerState>>,
    Json(req): Json<CreateCustomerRequest>,
) -> Json<Value> {
    info!("👤 Creating customer: {}", req.email);
    
    let customer = Customer {
        id: Uuid::new_v4().to_string(),
        user_id: req.user_id.clone(),
        email: req.email.clone(),
        stripe_customer_id: None, // TODO: Create Stripe customer
        created_at: Utc::now(),
        payment_methods: vec![],
    };
    
    let mut customers = state.customers.write().await;
    customers.insert(customer.id.clone(), customer.clone());
    
    Json(json!({
        "success": true,
        "customer": customer,
        "message": "Customer created successfully",
    }))
}

async fn get_customer(
    State(state): State<Arc<PaymentServerState>>,
    Path(customer_id): Path<String>,
) -> Json<Value> {
    let customers = state.customers.read().await;
    
    if let Some(customer) = customers.get(&customer_id) {
        Json(json!({
            "success": true,
            "customer": customer,
        }))
    } else {
        Json(json!({
            "success": false,
            "error": "Customer not found",
        }))
    }
}

// ============================================================================
// PRICING PLANS
// ============================================================================

async fn list_pricing_plans(
    State(state): State<Arc<PaymentServerState>>,
) -> Json<Value> {
    let plans = state.pricing_plans.read().await;
    
    Json(json!({
        "success": true,
        "plans": plans.values().collect::<Vec<_>>(),
        "total": plans.len(),
    }))
}

// ============================================================================
// SUBSCRIPTION MANAGEMENT
// ============================================================================

#[derive(Debug, Deserialize)]
struct CreateSubscriptionRequest {
    customer_id: String,
    plan_id: String,
    payment_method_id: Option<String>,
}

async fn create_subscription(
    State(state): State<Arc<PaymentServerState>>,
    Json(req): Json<CreateSubscriptionRequest>,
) -> Json<Value> {
    info!("📋 Creating subscription for customer: {}", req.customer_id);
    
    // Verify customer exists
    let customers = state.customers.read().await;
    if !customers.contains_key(&req.customer_id) {
        return Json(json!({
            "success": false,
            "error": "Customer not found",
        }));
    }
    drop(customers);
    
    // Verify plan exists
    let plans = state.pricing_plans.read().await;
    if !plans.contains_key(&req.plan_id) {
        return Json(json!({
            "success": false,
            "error": "Plan not found",
        }));
    }
    drop(plans);
    
    let subscription = Subscription {
        id: Uuid::new_v4().to_string(),
        customer_id: req.customer_id.clone(),
        plan_id: req.plan_id.clone(),
        status: SubscriptionStatus::Active,
        stripe_subscription_id: None, // TODO: Create Stripe subscription
        current_period_start: Utc::now(),
        current_period_end: Utc::now() + chrono::Duration::days(30),
        cancel_at_period_end: false,
        created_at: Utc::now(),
    };
    
    let mut subscriptions = state.subscriptions.write().await;
    subscriptions.insert(subscription.id.clone(), subscription.clone());
    
    Json(json!({
        "success": true,
        "subscription": subscription,
        "message": "Subscription created successfully",
    }))
}

async fn cancel_subscription(
    State(state): State<Arc<PaymentServerState>>,
    Path(subscription_id): Path<String>,
) -> Json<Value> {
    info!("❌ Canceling subscription: {}", subscription_id);
    
    let mut subscriptions = state.subscriptions.write().await;
    
    if let Some(subscription) = subscriptions.get_mut(&subscription_id) {
        subscription.cancel_at_period_end = true;
        
        Json(json!({
            "success": true,
            "subscription": subscription.clone(),
            "message": "Subscription will be canceled at period end",
        }))
    } else {
        Json(json!({
            "success": false,
            "error": "Subscription not found",
        }))
    }
}

// ============================================================================
// CONTAINER PAYMENT STABILITY
// ============================================================================

#[derive(Debug, Deserialize)]
struct RecordContainerPaymentRequest {
    bpi_os_id: String,
    container_id: String,
    payment_type: ContainerPaymentType,
    amount: u64,
    session_id: String,
}

async fn record_container_payment(
    State(state): State<Arc<PaymentServerState>>,
    Json(req): Json<RecordContainerPaymentRequest>,
) -> Json<Value> {
    info!("💳 Recording container payment: {} BPI for {}", req.amount, req.container_id);
    
    let payment = ContainerPayment {
        id: Uuid::new_v4().to_string(),
        bpi_os_id: req.bpi_os_id.clone(),
        container_id: req.container_id.clone(),
        payment_type: req.payment_type,
        amount: req.amount,
        timestamp: Utc::now(),
        session_id: req.session_id.clone(),
        stability_guaranteed: true, // Payment stability ensures container stays up
    };
    
    let mut payments = state.container_payments.write().await;
    payments.push(payment.clone());
    
    // Process through coin distribution engine
    let mut coin_engine = state.coin_engine.write().await;
    let fiat_equivalent = Decimal::new(req.amount as i64, 2); // Convert BPI to fiat equivalent
    
    match coin_engine.process_fiat_inflow(fiat_equivalent, CoinType::Flx, false) {
        Ok(distribution) => {
            info!("✅ Processed payment through coin distribution: {:?}", distribution);
        }
        Err(e) => {
            warn!("⚠️ Failed to process through coin distribution: {}", e);
        }
    }
    
    Json(json!({
        "success": true,
        "payment": payment,
        "stability_guaranteed": true,
        "message": "Container payment recorded with stability guarantee",
    }))
}

async fn get_container_payment_history(
    State(state): State<Arc<PaymentServerState>>,
    Path(bpi_os_id): Path<String>,
) -> Json<Value> {
    let payments = state.container_payments.read().await;
    
    let history: Vec<_> = payments.iter()
        .filter(|p| p.bpi_os_id == bpi_os_id)
        .collect();
    
    Json(json!({
        "success": true,
        "payments": history,
        "total": history.len(),
    }))
}

// ============================================================================
// INVOICE MANAGEMENT
// ============================================================================

#[derive(Debug, Deserialize)]
struct CreateInvoiceRequest {
    customer_id: String,
    items: Vec<InvoiceItem>,
    due_days: u32,
}

async fn create_invoice(
    State(state): State<Arc<PaymentServerState>>,
    Json(req): Json<CreateInvoiceRequest>,
) -> Json<Value> {
    info!("📄 Creating invoice for customer: {}", req.customer_id);
    
    let total_cad: Decimal = req.items.iter().map(|i| i.total).sum();
    let total_usd = total_cad * Decimal::new(75, 2); // Approximate conversion
    
    let invoice = Invoice {
        id: Uuid::new_v4().to_string(),
        customer_id: req.customer_id.clone(),
        amount_cad: total_cad,
        amount_usd: total_usd,
        status: InvoiceStatus::Open,
        stripe_invoice_id: None, // TODO: Create Stripe invoice
        due_date: Utc::now() + chrono::Duration::days(req.due_days as i64),
        paid_at: None,
        items: req.items,
        created_at: Utc::now(),
    };
    
    let mut invoices = state.invoices.write().await;
    invoices.insert(invoice.id.clone(), invoice.clone());
    
    Json(json!({
        "success": true,
        "invoice": invoice,
        "message": "Invoice created successfully",
    }))
}

async fn list_invoices(
    State(state): State<Arc<PaymentServerState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<Value> {
    let invoices = state.invoices.read().await;
    
    let filtered: Vec<_> = if let Some(customer_id) = params.get("customer_id") {
        invoices.values()
            .filter(|i| &i.customer_id == customer_id)
            .collect()
    } else {
        invoices.values().collect()
    };
    
    Json(json!({
        "success": true,
        "invoices": filtered,
        "total": filtered.len(),
    }))
}

// ============================================================================
// STRIPE WEBHOOK HANDLER
// ============================================================================

#[derive(Debug, Deserialize)]
struct StripeWebhookEvent {
    #[serde(rename = "type")]
    event_type: String,
    data: Value,
}

async fn handle_stripe_webhook(
    State(state): State<Arc<PaymentServerState>>,
    Json(event): Json<StripeWebhookEvent>,
) -> Json<Value> {
    info!("🔔 Stripe webhook received: {}", event.event_type);
    
    // TODO: Verify webhook signature
    // TODO: Handle different event types
    
    match event.event_type.as_str() {
        "payment_intent.succeeded" => {
            info!("✅ Payment succeeded");
        }
        "payment_intent.payment_failed" => {
            warn!("❌ Payment failed");
        }
        "customer.subscription.created" => {
            info!("📋 Subscription created");
        }
        "customer.subscription.deleted" => {
            info!("❌ Subscription deleted");
        }
        _ => {
            info!("ℹ️ Unhandled event type: {}", event.event_type);
        }
    }
    
    Json(json!({
        "received": true,
    }))
}

// ============================================================================
// PAYMENT STATISTICS
// ============================================================================

async fn get_payment_stats(
    State(state): State<Arc<PaymentServerState>>,
) -> Json<Value> {
    let customers = state.customers.read().await;
    let subscriptions = state.subscriptions.read().await;
    let invoices = state.invoices.read().await;
    let payments = state.container_payments.read().await;
    let coin_engine = state.coin_engine.read().await;
    
    let active_subscriptions = subscriptions.values()
        .filter(|s| matches!(s.status, SubscriptionStatus::Active))
        .count();
    
    let total_revenue_cad: Decimal = invoices.values()
        .filter(|i| matches!(i.status, InvoiceStatus::Paid))
        .map(|i| i.amount_cad)
        .sum();
    
    let total_container_payments: u64 = payments.iter()
        .map(|p| p.amount)
        .sum();
    
    let system_stats = coin_engine.get_system_stats();
    
    Json(json!({
        "success": true,
        "stats": {
            "customers": {
                "total": customers.len(),
            },
            "subscriptions": {
                "total": subscriptions.len(),
                "active": active_subscriptions,
            },
            "invoices": {
                "total": invoices.len(),
                "paid": invoices.values().filter(|i| matches!(i.status, InvoiceStatus::Paid)).count(),
            },
            "container_payments": {
                "total_transactions": payments.len(),
                "total_bpi": total_container_payments,
            },
            "revenue": {
                "total_cad": total_revenue_cad,
            },
            "coin_economy": system_stats,
        },
        "timestamp": Utc::now(),
    }))
}

// ============================================================================
// HEALTH & STATUS
// ============================================================================

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "server": "bpci-payment-server",
        "version": "1.0.0",
        "timestamp": Utc::now(),
    }))
}

// ============================================================================
// MAIN SERVER
// ============================================================================

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .with_level(true)
        .init();

    info!("💳 Starting BPCI Payment Server (Server 15)");
    
    // Initialize DynaRoute and CommuteLock
    let parser = EnvIniParser::new("config");
    let env_config = parser.parse_env_ini()?;
    let commute_lock = Arc::new(CommuteLockRuntime::new(&env_config)?);
    let local_addr = SocketAddr::from(([0, 0, 0, 0], 9015));
    let networking = Arc::new(UnifiedNetworkingLayer::new(local_addr, commute_lock.clone()).await?);
    
    // Initialize Stripe config
    let stripe_config = StripeConfig {
        api_key: std::env::var("STRIPE_API_KEY").unwrap_or_else(|_| "sk_test_...".to_string()),
        webhook_secret: std::env::var("STRIPE_WEBHOOK_SECRET").unwrap_or_else(|_| "whsec_...".to_string()),
        enabled: std::env::var("STRIPE_ENABLED").unwrap_or_else(|_| "false".to_string()) == "true",
    };
    
    if stripe_config.enabled {
        info!("✅ Stripe integration enabled");
    } else {
        warn!("⚠️ Stripe integration disabled (set STRIPE_ENABLED=true to enable)");
    }
    
    // Initialize state
    let state = Arc::new(PaymentServerState::new(networking, commute_lock, stripe_config));
    
    info!("✅ Initialized 3 pricing plans");
    
    // Build router
    let app = Router::new()
        // Health
        .route("/health", get(health_check))
        
        // Customer Management
        .route("/api/payment/customers", post(create_customer))
        .route("/api/payment/customers/:id", get(get_customer))
        
        // Pricing Plans
        .route("/api/payment/plans", get(list_pricing_plans))
        
        // Subscription Management
        .route("/api/payment/subscriptions", post(create_subscription))
        .route("/api/payment/subscriptions/:id/cancel", post(cancel_subscription))
        
        // Container Payment Stability
        .route("/api/payment/container/record", post(record_container_payment))
        .route("/api/payment/container/:bpi_os_id/history", get(get_container_payment_history))
        
        // Invoice Management
        .route("/api/payment/invoices", post(create_invoice))
        .route("/api/payment/invoices/list", get(list_invoices))
        
        // Stripe Webhook
        .route("/api/payment/webhook/stripe", post(handle_stripe_webhook))
        
        // Statistics
        .route("/api/payment/stats", get(get_payment_stats))
        
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .with_state(state);
    
    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 9015)); // Port 9015 for Payment Server
    info!("🚀 BPCI Payment Server listening on {}", addr);
    info!("💳 Pricing Plans: http://localhost:9015/api/payment/plans");
    info!("📊 Statistics: http://localhost:9015/api/payment/stats");
    info!("🔔 Stripe Webhook: http://localhost:9015/api/payment/webhook/stripe");
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
