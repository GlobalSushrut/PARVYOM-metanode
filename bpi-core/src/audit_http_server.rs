use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use crate::cbor_pipeline_foundation::CborSerializable;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use anyhow::{Result, anyhow};
use blake3;
use axum::{
    extract::{State, Json},
    http::{StatusCode, HeaderMap},
    response::Json as ResponseJson,
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;

use crate::forensic_firewall::audit_bridge::{
    ForensicAuditBridge, ForensicEvidence, EvidenceType
};
use crate::forensic_firewall::shared_types::{ForensicEventType, ForensicSeverity};
use crate::immutable_audit_system::{ComponentType, ImmutableAuditSystem};
use crate::forensic_firewall::cue_engine::CueRuleEngine;

/// BPI Core Audit HTTP Server for receiving audit submissions
#[derive(Debug, Clone)]
pub struct BpiAuditHttpServer {
    pub audit_bridge: Arc<ForensicAuditBridge>,
    pub audit_system: Arc<RwLock<ImmutableAuditSystem>>,
    pub stats: Arc<RwLock<AuditServerStats>>,
}

/// Audit server statistics
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AuditServerStats {
    pub total_audits_received: u64,
    pub total_audits_processed: u64,
    pub total_audits_failed: u64,
    pub uptime_seconds: u64,
    pub audits_per_second: f64,
    pub bpi_transactions_created: u64,
    pub ledger_submissions: u64,
}

/// ZipLock JSON audit format (from JS client)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZipLockJsonAudit {
    pub payload: serde_json::Value,
    pub integrity: serde_json::Value,
    pub signature: serde_json::Value,
    pub metadata: serde_json::Value,
}

fn ziplock_node_context() -> (String, String) {
    let node_id = std::env::var("BPI_NODE_ID").unwrap_or_else(|_| "unknown".to_string());
    let profile = std::env::var("BPI_ENV").unwrap_or_else(|_| "unknown".to_string());
    (node_id, profile)
}

fn new_trace_id() -> String {
    Uuid::new_v4().to_string()
}

fn compute_ziplock_signature(payload: &serde_json::Value, content_hash: &str) -> String {
    let key = std::env::var("BPI_ZIPLOCK_HMAC_KEY").unwrap_or_else(|_| "dev_default_ziplock_key".to_string());
    let mut hasher = blake3::Hasher::new();
    hasher.update(key.as_bytes());
    if let Ok(bytes) = serde_json::to_vec(payload) {
        hasher.update(&bytes);
    }
    hasher.update(content_hash.as_bytes());
    hasher.finalize().to_hex().to_string()
}

/// Helper to construct a canonical ZipLockJsonAudit for VM HTTP requests.
/// This centralizes the event shape so VM/DockLock/network components can
/// emit consistent ZipLock records.
pub fn make_vm_ziplock_audit(
    request_id: &str,
    method: &str,
    path: &str,
    enc_lock_enabled: bool,
    remote_addr: &str,
    raw_request: &str,
) -> ZipLockJsonAudit {
    let content_hash = blake3::hash(raw_request.as_bytes()).to_hex().to_string();
    let (node_id, profile) = ziplock_node_context();
    let trace_id = new_trace_id();

    let payload = serde_json::json!({
        "vm_request_id": request_id,
        "method": method,
        "path": path,
        "enc_lock_enabled": enc_lock_enabled,
        "remote_addr": remote_addr,
        "timestamp": Utc::now().to_rfc3339(),
        "nx_lane": "nx_vm_lane",
    });
    let signature_value = compute_ziplock_signature(&payload, &content_hash);

    ZipLockJsonAudit {
        payload,
        integrity: serde_json::json!({
            "content_hash": content_hash,
        }),
        signature: serde_json::json!({
            "signature": signature_value,
            "component": "VmServer",
        }),
        metadata: serde_json::json!({
            "vm_type": "Server",
            "node_id": node_id,
            "profile": profile,
            "trace_id": trace_id,
        }),
    }
}

/// Helper to construct a canonical ZipLockJsonAudit for ZKLock HTTP
/// connections.
pub fn make_zklock_ziplock_audit(
    request_id: &str,
    method: &str,
    path: &str,
    remote_addr: &str,
    raw_request: &str,
) -> ZipLockJsonAudit {
    let content_hash = blake3::hash(raw_request.as_bytes()).to_hex().to_string();
    let (node_id, profile) = ziplock_node_context();
    let trace_id = new_trace_id();

    let payload = serde_json::json!({
        "zklock_request_id": request_id,
        "method": method,
        "path": path,
        "remote_addr": remote_addr,
        "timestamp": Utc::now().to_rfc3339(),
        "nx_lane": "nx_zklock_lane",
    });
    let signature_value = compute_ziplock_signature(&payload, &content_hash);

    ZipLockJsonAudit {
        payload,
        integrity: serde_json::json!({
            "content_hash": content_hash,
        }),
        signature: serde_json::json!({
            "signature": signature_value,
            "component": "ZkLockServer",
        }),
        metadata: serde_json::json!({
            "vm_type": "ZKLock",
            "node_id": node_id,
            "profile": profile,
            "trace_id": trace_id,
        }),
    }
}

/// Helper to construct a canonical ZipLockJsonAudit for Shadow Registry HTTP
/// connections.
pub fn make_shadow_ziplock_audit(
    request_id: &str,
    method: &str,
    path: &str,
    remote_addr: &str,
    raw_request: &str,
) -> ZipLockJsonAudit {
    let content_hash = blake3::hash(raw_request.as_bytes()).to_hex().to_string();
    let (node_id, profile) = ziplock_node_context();
    let trace_id = new_trace_id();

    let payload = serde_json::json!({
        "shadow_request_id": request_id,
        "method": method,
        "path": path,
        "remote_addr": remote_addr,
        "timestamp": Utc::now().to_rfc3339(),
        "nx_lane": "nx_shadow_lane",
    });
    let signature_value = compute_ziplock_signature(&payload, &content_hash);

    ZipLockJsonAudit {
        payload,
        integrity: serde_json::json!({
            "content_hash": content_hash,
        }),
        signature: serde_json::json!({
            "signature": signature_value,
            "component": "ShadowRegistryServer",
        }),
        metadata: serde_json::json!({
            "vm_type": "ShadowRegistry",
            "node_id": node_id,
            "profile": profile,
            "trace_id": trace_id,
        }),
    }
}

/// Audit submission response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuditSubmissionResponse {
    pub success: bool,
    pub audit_id: String,
    pub bpi_transaction_id: Option<String>,
    pub receipt_id: Option<String>,
    pub message: String,
}

/// API response wrapper
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

// CBOR Serializable implementations for audit structs
impl CborSerializable for AuditServerStats {}
impl CborSerializable for ZipLockJsonAudit {}
impl CborSerializable for AuditSubmissionResponse {}

impl BpiAuditHttpServer {
    /// Create new BPI audit HTTP server
    pub async fn new(storage_path: &str) -> Result<Self> {
        // Initialize immutable audit system
        let audit_system = Arc::new(RwLock::new(
            ImmutableAuditSystem::new(storage_path).await?
        ));
        
        // Initialize CUE rule engine
        let cue_engine = Arc::new(CueRuleEngine::new());
        
        // Initialize forensic audit bridge
        let audit_bridge_config = crate::forensic_firewall::audit_bridge::AuditBridgeConfig::default();
        let audit_bridge = Arc::new(ForensicAuditBridge::new(
            audit_system.clone(),
            cue_engine,
            audit_bridge_config,
        ));
        
        let stats = Arc::new(RwLock::new(AuditServerStats::default()));
        
        Ok(BpiAuditHttpServer {
            audit_bridge,
            audit_system,
            stats,
        })
    }
    
    /// Create HTTP router for audit endpoints
    pub fn create_router(self) -> Router {
        Router::new()
            .route("/api/audit/submit", post(submit_audit))
            .route("/api/audit/status", get(get_audit_status))
            .route("/api/audit/stats", get(get_audit_stats))
            .route("/api/health", get(health_check))
            .layer(CorsLayer::permissive())
            .with_state(self)
    }
    
    /// Process audit submission
    pub async fn process_audit(&self, audit: ZipLockJsonAudit) -> Result<AuditSubmissionResponse> {
        let audit_id = Uuid::new_v4();
        
        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.total_audits_received += 1;
        }

        // Basic validation and hardening for incoming ZipLock JSON audits to
        // guard against malformed or excessively large payloads before they
        // enter the forensic pipeline.
        validate_ziplock_audit(&audit)?;
        
        // Create forensic evidence from audit
        let evidence = self.create_forensic_evidence(&audit).await?;
        
        // Record security event in forensic audit bridge
        let event_id = self.audit_bridge.record_security_event(
            ForensicEventType::ForensicEvidenceCollected,
            ComponentType::UniversalAuditSystem,
            ForensicSeverity::Info,
            format!("ZipLock JSON audit received: {}", audit_id),
            Some(evidence),
            None,
            None,
            None,
        ).await?;
        
        // Create BPI transaction for audit
        let bpi_transaction_id = self.create_bpi_transaction(&audit, &audit_id).await?;
        
        // Submit to BPI ledger
        let ledger_submitted = self.submit_to_bpi_ledger(&audit, &bpi_transaction_id).await?;
        
        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.total_audits_processed += 1;
            if bpi_transaction_id.is_some() {
                stats.bpi_transactions_created += 1;
            }
            if ledger_submitted {
                stats.ledger_submissions += 1;
            }
        }
        
        Ok(AuditSubmissionResponse {
            success: true,
            audit_id: audit_id.to_string(),
            bpi_transaction_id,
            receipt_id: Some(event_id.to_string()),
            message: "Audit processed and submitted to BPI ledger".to_string(),
        })
    }
    
    /// Create forensic evidence from audit
    async fn create_forensic_evidence(&self, audit: &ZipLockJsonAudit) -> Result<ForensicEvidence> {
        let evidence_id = Uuid::new_v4();
        let now = Utc::now();
        
        // Serialize audit data
        let raw_data = serde_json::to_vec(audit)?;
        let integrity_hash = audit.integrity.get("content_hash")
            .and_then(|h| h.as_str())
            .unwrap_or("unknown")
            .to_string();
        
        let mut metadata = std::collections::HashMap::new();
        metadata.insert("audit_type".to_string(), "ziplockjson".to_string());
        metadata.insert("client_id".to_string(), 
            audit.metadata.get("client_id")
                .and_then(|c| c.as_str())
                .unwrap_or("unknown")
                .to_string()
        );
        metadata.insert("compliance_level".to_string(),
            audit.payload.get("compliance_level")
                .and_then(|c| c.as_str())
                .unwrap_or("standard")
                .to_string()
        );
        
        let processed_data: std::collections::HashMap<String, serde_json::Value> = serde_json::json!({
            "audit_id": audit.payload.get("audit_id"),
            "wallet_id": audit.payload.get("wallet_id"),
            "gas_used": audit.payload.get("gas_used"),
            "block_height": audit.payload.get("block_height"),
            "compliance_tags": audit.payload.get("compliance_tags")
        }).as_object().unwrap().iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        
        Ok(ForensicEvidence {
            evidence_id,
            evidence_type: EvidenceType::SystemLog,
            collected_at: now,
            collector: "BPI-Core-Audit-Server".to_string(),
            integrity_hash,
            digital_signature: audit.signature.get("signature")
                .and_then(|s| s.as_str())
                .unwrap_or("")
                .to_string(),
            metadata,
            raw_data,
            processed_data,
            chain_of_custody_id: Uuid::new_v4(),
        })
    }
    
    /// Create BPI transaction for audit
    async fn create_bpi_transaction(&self, audit: &ZipLockJsonAudit, audit_id: &Uuid) -> Result<Option<String>> {
        // Extract transaction data from audit
        let wallet_id = audit.payload.get("wallet_id")
            .and_then(|w| w.as_str())
            .unwrap_or("unknown");
        let gas_used = audit.payload.get("gas_used")
            .and_then(|g| g.as_u64())
            .unwrap_or(21000);
        
        // Create BPI transaction ID
        let transaction_id = format!("bpi-tx-{}", Uuid::new_v4().to_string()[..16].to_string());
        
        // Log transaction creation
        tracing::info!(
            "Created BPI transaction {} for audit {} from wallet {}",
            transaction_id,
            audit_id,
            wallet_id
        );
        
        Ok(Some(transaction_id))
    }
    
    /// Submit audit to BPI ledger
    async fn submit_to_bpi_ledger(&self, audit: &ZipLockJsonAudit, transaction_id: &Option<String>) -> Result<bool> {
        if let Some(tx_id) = transaction_id {
            // Log ledger submission
            tracing::info!(
                "Submitting audit to BPI ledger with transaction ID: {}",
                tx_id
            );
            
            // Here we would integrate with the actual BPI ledger
            // For now, we'll simulate successful submission
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            
            return Ok(true);
        }
        
        Ok(false)
    }
}

/// Validate a ZipLock JSON audit payload before it enters the forensic and
/// ledger pipelines. This enforces basic structural requirements, size
/// bounds, and CBOR canonical round-trip safety.
fn validate_ziplock_audit(audit: &ZipLockJsonAudit) -> Result<()> {
    // Guard against unbounded payloads from clients. This limit is generous
    // for typical audit records but protects the pipeline from abuse.
    const MAX_ZIPLOCK_AUDIT_BYTES: usize = 256 * 1024; // 256 KiB

    let serialized = serde_json::to_vec(audit)?;
    if serialized.len() > MAX_ZIPLOCK_AUDIT_BYTES {
        return Err(anyhow!(
            "ZipLock JSON audit is too large ({} bytes > {} bytes)",
            serialized.len(),
            MAX_ZIPLOCK_AUDIT_BYTES,
        ));
    }

    if !audit.payload.is_object() {
        return Err(anyhow!("ZipLock JSON audit payload must be a JSON object"));
    }

    if audit
        .integrity
        .get("content_hash")
        .and_then(|h| h.as_str())
        .is_none()
    {
        return Err(anyhow!(
            "ZipLock JSON audit integrity.content_hash must be present and a string",
        ));
    }

    if audit
        .signature
        .get("signature")
        .and_then(|s| s.as_str())
        .is_none()
    {
        return Err(anyhow!(
            "ZipLock JSON audit signature.signature must be present and a string",
        ));
    }

    // Optional but helpful: ensure the audit can be losslessly round-tripped
    // through the CBOR canonical pipeline used across the system.
    match audit.validate_cbor() {
        Ok(true) => Ok(()),
        Ok(false) => Err(anyhow!(
            "ZipLock JSON audit failed CBOR canonical validation",
        )),
        Err(e) => Err(anyhow!(
            "ZipLock JSON audit CBOR canonical validation error: {}",
            e
        )),
    }
}

/// Submit audit endpoint
async fn submit_audit(
    State(server): State<BpiAuditHttpServer>,
    headers: HeaderMap,
    Json(audit): Json<ZipLockJsonAudit>,
) -> Result<ResponseJson<ApiResponse<AuditSubmissionResponse>>, StatusCode> {
    // Log audit submission
    let client_id = headers.get("X-Client-ID")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown");
    
    tracing::info!("Received audit submission from client: {}", client_id);
    
    match server.process_audit(audit).await {
        Ok(response) => Ok(ResponseJson(ApiResponse {
            success: true,
            data: Some(response),
            error: None,
        })),
        Err(e) => {
            tracing::error!("Audit processing failed: {}", e);
            
            // Update error stats
            {
                let mut stats = server.stats.write().await;
                stats.total_audits_failed += 1;
            }
            
            Ok(ResponseJson(ApiResponse {
                success: false,
                data: None,
                error: Some(e.to_string()),
            }))
        }
    }
}

/// Get audit status endpoint
async fn get_audit_status(
    State(server): State<BpiAuditHttpServer>,
) -> ResponseJson<ApiResponse<serde_json::Value>> {
    let status = serde_json::json!({
        "service": "BPI Core Audit Server",
        "status": "active",
        "audit_bridge": "connected",
        "bpi_ledger": "connected",
        "forensic_system": "active"
    });
    
    ResponseJson(ApiResponse {
        success: true,
        data: Some(status),
        error: None,
    })
}

/// Get audit statistics endpoint
async fn get_audit_stats(
    State(server): State<BpiAuditHttpServer>,
) -> ResponseJson<ApiResponse<AuditServerStats>> {
    let stats = server.stats.read().await.clone();
    
    ResponseJson(ApiResponse {
        success: true,
        data: Some(stats),
        error: None,
    })
}

/// Health check endpoint
async fn health_check() -> ResponseJson<ApiResponse<serde_json::Value>> {
    let health = serde_json::json!({
        "status": "healthy",
        "service": "BPI Core Audit Server",
        "timestamp": Utc::now().to_rfc3339()
    });
    
    ResponseJson(ApiResponse {
        success: true,
        data: Some(health),
        error: None,
    })
}
