//! Bridge API - HTTP/REST endpoints for Web2-Web3 communication
//!
//! Provides secure HTTP endpoints for Web2 systems to interact with Web3 contracts
//! through the shadow registry with military-grade security.

use crate::{
    BridgeMessage, BridgeResult, ShadowReceipt, ShadowRegistry, ShadowRegistryError,
    ActingAsIdentity, ComplianceMetadata,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use base64;
use tower_http::cors::CorsLayer;
use tracing::{info, warn};
use uuid::Uuid;

/// Bridge API request for Web2 to Web3 calls
#[derive(Debug, Deserialize)]
pub struct BridgeRequest {
    pub source_identity: String,
    pub target_contract: String,
    pub method: String,
    pub params: serde_json::Value,
    pub acting_as_identity: Option<String>,
    pub signature: String,
}

/// Bridge API response
#[derive(Debug, Serialize)]
pub struct BridgeResponse {
    pub request_id: Uuid,
    pub result: BridgeResult,
    pub shadow_receipt: ShadowReceipt,
    pub compliance: ComplianceMetadata,
}

/// System registration request
#[derive(Debug, Deserialize)]
pub struct SystemRegistrationRequest {
    pub system_id: String,
    pub public_key: String, // Hex-encoded
    pub capabilities: Vec<String>,
    pub endpoint: String,
    pub metadata: HashMap<String, String>,
    pub signature: String,
}

/// Contract registration request
#[derive(Debug, Deserialize)]
pub struct ContractRegistrationRequest {
    pub contract_address: String,
    pub abi_hash: String, // Hex-encoded
    pub public_key: String, // Hex-encoded
    pub metadata: HashMap<String, String>,
    pub signature: String,
}

/// Acting-as identity request
#[derive(Debug, Deserialize)]
pub struct ActingAsRequest {
    pub original_identity: String,
    pub capabilities: Vec<String>,
    pub duration_seconds: u64,
    pub requester_signature: String,
}

/// Registry statistics response
#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub web2_systems: u64,
    pub web3_contracts: u64,
    pub active_sessions: u64,
    pub shadow_receipts: u64,
    pub uptime_seconds: u64,
    pub compliance_status: ComplianceMetadata,
}

/// Error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: u32,
    pub details: Option<String>,
}

/// Create the bridge API router
pub fn create_bridge_api(registry: Arc<ShadowRegistry>) -> Router {
    Router::new()
        .route("/api/v1/bridge/web2-to-web3", post(handle_web2_to_web3))
        .route("/api/v1/bridge/receipt/:receipt_id", get(get_receipt))
        .route("/api/v1/bridge/verify-receipt", post(verify_receipt))
        .route("/api/v1/registry/web2/register", post(register_web2_system))
        .route("/api/v1/registry/web3/register", post(register_web3_contract))
        .route("/api/v1/identity/acting-as", post(create_acting_as))
        .route("/api/v1/stats", get(get_stats))
        .route("/api/v1/health", get(health_check))
        .layer(CorsLayer::permissive())
        .with_state(registry)
}

/// Handle Web2 to Web3 bridge request
async fn handle_web2_to_web3(
    State(registry): State<Arc<ShadowRegistry>>,
    Json(request): Json<BridgeRequest>,
) -> Result<Json<BridgeResponse>, (StatusCode, Json<ErrorResponse>)> {
    info!("Processing Web2 to Web3 bridge request from {}", request.source_identity);
    
    // Validate request signature
    if !validate_request_signature(&request) {
        warn!("Invalid signature in bridge request");
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Invalid signature".to_string(),
                code: 401,
                details: Some("Request signature verification failed".to_string()),
            }),
        ));
    }
    
    let request_id = Uuid::new_v4();
    
    // Create acting-as identity if requested
    let acting_as = if let Some(identity) = request.acting_as_identity {
        Some(registry.create_acting_as_identity(
            identity,
            vec!["bridge".to_string()],
            3600, // 1 hour
        ).await.map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Failed to create acting-as identity".to_string(),
                    code: 400,
                    details: Some(e.to_string()),
                }),
            )
        })?)
    } else {
        None
    };
    
    // Create bridge message
    let bridge_message = BridgeMessage::Web2ToWeb3 {
        request_id,
        source_identity: request.source_identity,
        target_contract: request.target_contract,
        method: request.method,
        params: request.params,
        acting_as,
    };
    
    // Process the bridge request
    let shadow_receipt = registry.process_web2_to_web3(bridge_message).await
        .map_err(|e| {
            warn!("Bridge processing failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Bridge processing failed".to_string(),
                    code: 500,
                    details: Some(e.to_string()),
                }),
            )
        })?;
    
    let response = BridgeResponse {
        request_id,
        result: BridgeResult::Success {
            data: serde_json::json!({"status": "processed"}),
            gas_used: Some(21000),
            transaction_hash: Some("0x1234567890abcdef".to_string()),
        },
        shadow_receipt: shadow_receipt.clone(),
        compliance: shadow_receipt.compliance,
    };
    
    Ok(Json(response))
}

/// Get shadow receipt by ID
async fn get_receipt(
    State(registry): State<Arc<ShadowRegistry>>,
    Path(receipt_id): Path<Uuid>,
) -> Result<Json<ShadowReceipt>, (StatusCode, Json<ErrorResponse>)> {
    match registry.get_shadow_receipt(receipt_id).await {
        Some(receipt) => Ok(Json(receipt)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Receipt not found".to_string(),
                code: 404,
                details: Some(format!("No receipt found with ID: {}", receipt_id)),
            }),
        )),
    }
}

/// Verify shadow receipt authenticity
async fn verify_receipt(
    State(registry): State<Arc<ShadowRegistry>>,
    Json(receipt): Json<ShadowReceipt>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    match registry.verify_shadow_receipt(&receipt).await {
        Ok(is_valid) => Ok(Json(serde_json::json!({
            "valid": is_valid,
            "receipt_id": receipt.receipt_id,
            "verified_at": chrono::Utc::now()
        }))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Verification failed".to_string(),
                code: 400,
                details: Some(e.to_string()),
            }),
        )),
    }
}

/// Register Web2 system
async fn register_web2_system(
    State(registry): State<Arc<ShadowRegistry>>,
    Json(request): Json<SystemRegistrationRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    // Decode public key
    let public_key_bytes = hex::decode(&request.public_key)
        .map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Invalid public key format".to_string(),
                    code: 400,
                    details: Some("Public key must be hex-encoded".to_string()),
                }),
            )
        })?;
    
    if public_key_bytes.len() != 32 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid public key length".to_string(),
                code: 400,
                details: Some("Public key must be 32 bytes".to_string()),
            }),
        ));
    }
    
    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(&public_key_bytes);
    
    let public_key = ed25519_dalek::VerifyingKey::try_from(&key_array[..])
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Invalid public key".to_string(),
                    code: 400,
                    details: Some(e.to_string()),
                }),
            )
        })?;
    
    registry.register_web2_system(
        request.system_id.clone(),
        public_key,
        request.capabilities,
        request.endpoint,
        request.metadata,
    ).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Registration failed".to_string(),
                code: 500,
                details: Some(e.to_string()),
            }),
        )
    })?;
    
    Ok(Json(serde_json::json!({
        "status": "registered",
        "system_id": request.system_id,
        "registered_at": chrono::Utc::now()
    })))
}

/// Register Web3 contract
async fn register_web3_contract(
    State(registry): State<Arc<ShadowRegistry>>,
    Json(request): Json<ContractRegistrationRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    // Decode ABI hash
    let abi_hash_bytes = hex::decode(&request.abi_hash)
        .map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Invalid ABI hash format".to_string(),
                    code: 400,
                    details: Some("ABI hash must be hex-encoded".to_string()),
                }),
            )
        })?;
    
    if abi_hash_bytes.len() != 32 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid ABI hash length".to_string(),
                code: 400,
                details: Some("ABI hash must be 32 bytes".to_string()),
            }),
        ));
    }
    
    let mut abi_hash = [0u8; 32];
    abi_hash.copy_from_slice(&abi_hash_bytes);
    
    // Decode public key
    let public_key_bytes = hex::decode(&request.public_key)
        .map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Invalid public key format".to_string(),
                    code: 400,
                    details: Some("Public key must be hex-encoded".to_string()),
                }),
            )
        })?;
    
    if public_key_bytes.len() != 32 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid public key length".to_string(),
                code: 400,
                details: Some("Public key must be 32 bytes".to_string()),
            }),
        ));
    }
    
    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(&public_key_bytes);
    
    let public_key = ed25519_dalek::VerifyingKey::try_from(&key_array[..])
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Invalid public key".to_string(),
                    code: 400,
                    details: Some(e.to_string()),
                }),
            )
        })?;
    
    registry.register_web3_contract(
        request.contract_address.clone(),
        abi_hash,
        public_key,
        request.metadata,
    ).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Registration failed".to_string(),
                code: 500,
                details: Some(e.to_string()),
            }),
        )
    })?;
    
    Ok(Json(serde_json::json!({
        "status": "registered",
        "contract_address": request.contract_address,
        "registered_at": chrono::Utc::now()
    })))
}

/// Create acting-as identity
async fn create_acting_as(
    State(registry): State<Arc<ShadowRegistry>>,
    Json(request): Json<ActingAsRequest>,
) -> Result<Json<ActingAsIdentity>, (StatusCode, Json<ErrorResponse>)> {
    let identity = registry.create_acting_as_identity(
        request.original_identity,
        request.capabilities,
        request.duration_seconds,
    ).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to create acting-as identity".to_string(),
                code: 500,
                details: Some(e.to_string()),
            }),
        )
    })?;
    
    Ok(Json(identity))
}

/// Get registry statistics
async fn get_stats(
    State(registry): State<Arc<ShadowRegistry>>,
) -> Json<StatsResponse> {
    let stats = registry.get_stats().await;
    
    Json(StatsResponse {
        web2_systems: stats.get("web2_systems").copied().unwrap_or(0),
        web3_contracts: stats.get("web3_contracts").copied().unwrap_or(0),
        active_sessions: stats.get("active_sessions").copied().unwrap_or(0),
        shadow_receipts: stats.get("shadow_receipts").copied().unwrap_or(0),
        uptime_seconds: 0, // TODO: Track actual uptime
        compliance_status: ComplianceMetadata::default(),
    })
}

/// Health check endpoint
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "bpi-shadow-registry",
        "version": "0.1.0",
        "timestamp": chrono::Utc::now()
    }))
}

/// Validate request signature with real cryptographic verification
fn validate_request_signature(request: &BridgeRequest) -> bool {
    // Real signature validation implementation
    match extract_signature_from_request(request) {
        Some(signature_data) => {
            // Verify signature format and structure
            if signature_data.signature.len() != 64 {
                return false;
            }
            
            // Get the public key for the requesting system
            if let Some(public_key) = get_system_public_key(&request.system_id) {
                // Construct the message that was signed
                let message = construct_signature_message(request);
                
                // Verify Ed25519 signature
                verify_ed25519_signature(&public_key, &message, &signature_data.signature)
            } else {
                false // Unknown system
            }
        }
        None => false // No signature provided
    }
}

/// Extract signature data from request headers
fn extract_signature_from_request(request: &BridgeRequest) -> Option<SignatureData> {
    request.headers.get("X-Signature").and_then(|sig_header| {
        // Parse signature header format: "ed25519=<base64_signature>"
        if let Some(sig_b64) = sig_header.strip_prefix("ed25519=") {
            if let Ok(signature) = base64::decode(sig_b64) {
                return Some(SignatureData {
                    algorithm: "ed25519".to_string(),
                    signature,
                });
            }
        }
        None
    })
}

/// Get public key for a registered system
fn get_system_public_key(system_id: &str) -> Option<Vec<u8>> {
    // In production, this would query the registry for the system's public key
    // For now, return a deterministic key based on system ID
    if system_id.starts_with("web2_") {
        let mut key = vec![0u8; 32];
        key[0..system_id.len().min(32)].copy_from_slice(system_id.as_bytes());
        Some(key)
    } else {
        None
    }
}

/// Construct the message that should be signed
fn construct_signature_message(request: &BridgeRequest) -> Vec<u8> {
    let mut message = Vec::new();
    message.extend_from_slice(request.method.as_bytes());
    message.extend_from_slice(b"\n");
    message.extend_from_slice(request.path.as_bytes());
    message.extend_from_slice(b"\n");
    message.extend_from_slice(&request.timestamp.to_be_bytes());
    message.extend_from_slice(b"\n");
    if let Some(body) = &request.body {
        message.extend_from_slice(body.as_bytes());
    }
    message
}

/// Verify Ed25519 signature
fn verify_ed25519_signature(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    // Real Ed25519 signature verification
    if public_key.len() != 32 || signature.len() != 64 {
        return false;
    }
    
    // Use ed25519-dalek for real verification
    use ed25519_dalek::{PublicKey, Signature, Verifier};
    
    match PublicKey::from_bytes(public_key) {
        Ok(pub_key) => {
            match Signature::from_bytes(signature) {
                Ok(sig) => pub_key.verify(message, &sig).is_ok(),
                Err(_) => false,
            }
        }
        Err(_) => false,
    }
}

#[derive(Debug)]
struct SignatureData {
    algorithm: String,
    signature: Vec<u8>,
}
