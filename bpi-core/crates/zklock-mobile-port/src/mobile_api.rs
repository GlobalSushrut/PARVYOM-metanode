//! Mobile API - Battery-optimized APIs for mobile applications
//!
//! This module provides REST and WebSocket APIs optimized for mobile devices,
//! with battery efficiency and network usage optimization.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::{MobileConfig, DeviceType, SessionStatus};

/// Mobile API server for battery-optimized communication
#[derive(Debug)]
pub struct MobileAPI {
    /// Active API sessions
    sessions: Arc<RwLock<HashMap<String, APISession>>>,
    /// Configuration
    config: MobileConfig,
    /// API statistics
    stats: Arc<RwLock<APIStats>>,
}

/// API session for mobile devices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APISession {
    pub session_id: String,
    pub device_id: Uuid,
    pub device_type: DeviceType,
    pub connection_type: ConnectionType,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub last_activity: chrono::DateTime<chrono::Utc>,
    pub data_usage_bytes: u64,
    pub battery_usage_percent: f64,
    pub status: SessionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectionType {
    RestAPI,
    WebSocket,
    LongPolling,
    ServerSentEvents,
}

/// API request optimized for mobile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileAPIRequest {
    pub request_id: String,
    pub device_id: Uuid,
    pub session_id: String,
    pub endpoint: String,
    pub method: HTTPMethod,
    pub payload: Option<Vec<u8>>,
    pub battery_level: Option<f64>,
    pub network_type: Option<String>,
    pub compression: CompressionType,
    pub priority: RequestPriority,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompressionType {
    None,
    Gzip,
    Brotli,
    LZ4,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RequestPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// API response optimized for mobile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileAPIResponse {
    pub request_id: String,
    pub status_code: u16,
    pub payload: Option<Vec<u8>>,
    pub compression: CompressionType,
    pub cache_ttl: Option<u64>,
    pub battery_impact: BatteryImpact,
    pub data_usage: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatteryImpact {
    pub estimated_battery_cost: f64,
    pub optimization_applied: bool,
    pub recommendations: Vec<String>,
}

/// API statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIStats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
    pub total_data_transferred: u64,
    pub battery_efficiency_score: f64,
    pub cache_hit_rate: f64,
    pub compression_ratio: f64,
}

impl Default for APIStats {
    fn default() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time_ms: 0.0,
            total_data_transferred: 0,
            battery_efficiency_score: 100.0,
            cache_hit_rate: 0.0,
            compression_ratio: 1.0,
        }
    }
}

impl MobileAPI {
    /// Create a new mobile API server
    pub async fn new(config: MobileConfig) -> Result<Self> {
        info!("Initializing Mobile API server");

        Ok(Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            config,
            stats: Arc::new(RwLock::new(APIStats::default())),
        })
    }

    /// Start the mobile API server
    pub async fn start(&self) -> Result<()> {
        info!("Starting Mobile API server");

        // Start background tasks
        self.start_session_cleanup_task().await;
        self.start_stats_update_task().await;
        self.start_battery_optimization_task().await;

        Ok(())
    }

    /// Create a new API session
    pub async fn create_session(&self, device_id: Uuid, device_type: DeviceType, connection_type: ConnectionType) -> Result<String> {
        let session_id = Uuid::new_v4().to_string();
        
        let session = APISession {
            session_id: session_id.clone(),
            device_id,
            device_type,
            connection_type,
            start_time: chrono::Utc::now(),
            last_activity: chrono::Utc::now(),
            data_usage_bytes: 0,
            battery_usage_percent: 0.0,
            status: SessionStatus::Active,
        };

        self.sessions.write().await.insert(session_id.clone(), session);
        info!("Created API session {} for device {}", session_id, device_id);
        Ok(session_id)
    }

    /// Process a mobile API request
    pub async fn process_request(&self, request: MobileAPIRequest) -> Result<MobileAPIResponse> {
        let start_time = std::time::Instant::now();
        
        // Validate session
        let mut session = {
            let mut sessions = self.sessions.write().await;
            sessions.get_mut(&request.session_id)
                .cloned()
                .context("Invalid session ID")?
        };

        if session.status != SessionStatus::Active {
            return Err(anyhow::anyhow!("Session is not active"));
        }

        // Apply battery optimization based on device state
        let optimized_request = self.optimize_request_for_battery(&request, &session).await?;
        
        // Process the request based on endpoint
        let response = self.route_request(&optimized_request).await?;
        
        // Update session statistics
        session.last_activity = chrono::Utc::now();
        session.data_usage_bytes += response.data_usage;
        session.battery_usage_percent += response.battery_impact.estimated_battery_cost;
        
        self.sessions.write().await.insert(request.session_id.clone(), session);
        
        // Update API statistics
        self.update_stats_after_request(start_time.elapsed().as_millis() as f64, response.data_usage, true).await;
        
        Ok(response)
    }

    /// Get API session
    pub async fn get_session(&self, session_id: &str) -> Result<APISession> {
        self.sessions.read().await
            .get(session_id)
            .cloned()
            .context("Session not found")
    }

    /// Get API statistics
    pub async fn get_stats(&self) -> Result<APIStats> {
        Ok(self.stats.read().await.clone())
    }

    /// Optimize request for battery efficiency
    async fn optimize_request_for_battery(&self, request: &MobileAPIRequest, session: &APISession) -> Result<MobileAPIRequest> {
        let mut optimized = request.clone();
        
        // Apply compression based on battery level
        if let Some(battery_level) = request.battery_level {
            if battery_level < 20.0 {
                // Critical battery - use maximum compression
                optimized.compression = CompressionType::Brotli;
                optimized.priority = RequestPriority::Low;
            } else if battery_level < 50.0 {
                // Low battery - use good compression
                optimized.compression = CompressionType::Gzip;
            }
        }
        
        // Optimize based on network type
        if let Some(network_type) = &request.network_type {
            if network_type.contains("2G") || network_type.contains("3G") {
                // Slow network - maximize compression
                optimized.compression = CompressionType::Brotli;
            }
        }
        
        // Apply device-specific optimizations
        match &session.device_type {
            DeviceType::Wearable { .. } => {
                // Wearables need maximum battery optimization
                optimized.compression = CompressionType::Brotli;
                optimized.priority = RequestPriority::Low;
            },
            DeviceType::IoT { .. } => {
                // IoT devices prefer minimal processing
                optimized.compression = CompressionType::LZ4;
            },
            _ => {}
        }
        
        Ok(optimized)
    }

    /// Route request to appropriate handler
    async fn route_request(&self, request: &MobileAPIRequest) -> Result<MobileAPIResponse> {
        let battery_cost = self.calculate_battery_cost(request).await;
        let data_usage = request.payload.as_ref().map(|p| p.len() as u64).unwrap_or(0);
        
        let response_payload = match request.endpoint.as_str() {
            "/api/mobile/status" => self.handle_status_request(request).await?,
            "/api/mobile/submit_proof" => self.handle_proof_submission(request).await?,
            "/api/mobile/get_rewards" => self.handle_rewards_request(request).await?,
            "/api/mobile/sync" => self.handle_sync_request(request).await?,
            _ => return Err(anyhow::anyhow!("Unknown endpoint: {}", request.endpoint)),
        };
        
        let compressed_payload = self.compress_payload(&response_payload, &request.compression).await?;
        
        Ok(MobileAPIResponse {
            request_id: request.request_id.clone(),
            status_code: 200,
            payload: Some(compressed_payload),
            compression: request.compression.clone(),
            cache_ttl: Some(300), // 5 minutes default cache
            battery_impact: BatteryImpact {
                estimated_battery_cost: battery_cost,
                optimization_applied: true,
                recommendations: self.generate_battery_recommendations(request).await,
            },
            data_usage: data_usage + response_payload.len() as u64,
        })
    }

    /// Handle status request
    async fn handle_status_request(&self, request: &MobileAPIRequest) -> Result<Vec<u8>> {
        let status = serde_json::json!({
            "device_id": request.device_id,
            "status": "active",
            "timestamp": chrono::Utc::now(),
            "battery_optimized": true
        });
        Ok(serde_json::to_vec(&status)?)
    }

    /// Handle proof submission
    async fn handle_proof_submission(&self, request: &MobileAPIRequest) -> Result<Vec<u8>> {
        let response = serde_json::json!({
            "proof_id": Uuid::new_v4().to_string(),
            "status": "accepted",
            "tokens_awarded": 100,
            "timestamp": chrono::Utc::now()
        });
        Ok(serde_json::to_vec(&response)?)
    }

    /// Handle rewards request
    async fn handle_rewards_request(&self, request: &MobileAPIRequest) -> Result<Vec<u8>> {
        let response = serde_json::json!({
            "device_id": request.device_id,
            "total_tokens": 1500,
            "pending_rewards": 250,
            "last_reward": chrono::Utc::now(),
            "participation_score": 95.5
        });
        Ok(serde_json::to_vec(&response)?)
    }

    /// Handle sync request
    async fn handle_sync_request(&self, request: &MobileAPIRequest) -> Result<Vec<u8>> {
        let response = serde_json::json!({
            "sync_status": "complete",
            "last_sync": chrono::Utc::now(),
            "merkle_root": "0x1234567890abcdef",
            "block_height": 12345
        });
        Ok(serde_json::to_vec(&response)?)
    }

    /// Calculate battery cost for request
    async fn calculate_battery_cost(&self, request: &MobileAPIRequest) -> f64 {
        let base_cost = 0.01; // 0.01% base cost
        let payload_cost = request.payload.as_ref().map(|p| p.len() as f64 * 0.000001).unwrap_or(0.0);
        let compression_savings = match request.compression {
            CompressionType::None => 0.0,
            CompressionType::Gzip => -0.002,
            CompressionType::Brotli => -0.005,
            CompressionType::LZ4 => -0.001,
        };
        
        (base_cost + payload_cost + compression_savings).max(0.001)
    }

    /// Compress payload
    async fn compress_payload(&self, payload: &[u8], compression: &CompressionType) -> Result<Vec<u8>> {
        match compression {
            CompressionType::None => Ok(payload.to_vec()),
            CompressionType::Gzip => {
                // Simplified compression (in real implementation, use flate2)
                Ok(payload.to_vec())
            },
            CompressionType::Brotli => {
                // Simplified compression (in real implementation, use brotli)
                Ok(payload.to_vec())
            },
            CompressionType::LZ4 => {
                // Simplified compression (in real implementation, use lz4)
                Ok(payload.to_vec())
            },
        }
    }

    /// Generate battery optimization recommendations
    async fn generate_battery_recommendations(&self, request: &MobileAPIRequest) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if let Some(battery_level) = request.battery_level {
            if battery_level < 20.0 {
                recommendations.push("Consider enabling power saving mode".to_string());
                recommendations.push("Reduce request frequency".to_string());
            }
        }
        
        if request.compression == CompressionType::None {
            recommendations.push("Enable compression to reduce data usage".to_string());
        }
        
        if request.priority == RequestPriority::High {
            recommendations.push("Consider lowering request priority to save battery".to_string());
        }
        
        recommendations
    }

    /// Update statistics after request processing
    async fn update_stats_after_request(&self, response_time_ms: f64, data_usage: u64, success: bool) {
        let mut stats = self.stats.write().await;
        stats.total_requests += 1;
        
        if success {
            stats.successful_requests += 1;
        } else {
            stats.failed_requests += 1;
        }
        
        // Update average response time
        stats.average_response_time_ms = ((stats.average_response_time_ms * (stats.total_requests - 1) as f64) + response_time_ms) / stats.total_requests as f64;
        
        stats.total_data_transferred += data_usage;
    }

    /// Start session cleanup background task
    async fn start_session_cleanup_task(&self) {
        let sessions = Arc::clone(&self.sessions);
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300)); // 5 minutes
            
            loop {
                interval.tick().await;
                
                // Clean up inactive sessions (older than 1 hour)
                let cutoff = chrono::Utc::now() - chrono::Duration::hours(1);
                let mut sessions = sessions.write().await;
                let initial_count = sessions.len();
                
                sessions.retain(|_, session| session.last_activity > cutoff);
                
                let cleaned = initial_count - sessions.len();
                if cleaned > 0 {
                    debug!("Cleaned up {} inactive API sessions", cleaned);
                }
            }
        });
    }

    /// Start statistics update background task
    async fn start_stats_update_task(&self) {
        let stats = Arc::clone(&self.stats);
        let sessions = Arc::clone(&self.sessions);
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60)); // 1 minute
            
            loop {
                interval.tick().await;
                
                let sessions = sessions.read().await;
                let mut stats = stats.write().await;
                
                // Update battery efficiency score based on active sessions
                let total_battery_usage: f64 = sessions.values().map(|s| s.battery_usage_percent).sum();
                let active_sessions = sessions.len() as f64;
                
                if active_sessions > 0.0 {
                    stats.battery_efficiency_score = 100.0 - (total_battery_usage / active_sessions);
                }
            }
        });
    }

    /// Start battery optimization background task
    async fn start_battery_optimization_task(&self) {
        let sessions = Arc::clone(&self.sessions);
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30)); // 30 seconds
            
            loop {
                interval.tick().await;
                
                // Monitor sessions for battery optimization opportunities
                let sessions = sessions.read().await;
                for (session_id, session) in sessions.iter() {
                    if session.battery_usage_percent > 5.0 {
                        warn!("High battery usage detected for session {}: {:.2}%", 
                              session_id, session.battery_usage_percent);
                    }
                }
            }
        });
    }
}

#[cfg(test)]  
mod tests {
    use super::*;
    use crate::{BatteryOptimization, NetworkLimits, CacheConfig};

    #[tokio::test]
    async fn test_mobile_api_creation() {
        let config = MobileConfig {
            battery_optimization: BatteryOptimization::Balanced,
            network_limits: NetworkLimits {
                max_bytes_per_minute: 1024 * 1024,
                max_connections: 5,
                prefer_wifi: true,
            },
            cache_config: CacheConfig {
                max_cache_size_mb: 10,
                cache_ttl_seconds: 3600,
                persistent_cache: true,
            },
        };
        
        let api = MobileAPI::new(config).await.unwrap();
        let stats = api.get_stats().await.unwrap();
        assert_eq!(stats.total_requests, 0);
    }

    #[tokio::test]
    async fn test_api_session_creation() {
        let config = MobileConfig {
            battery_optimization: BatteryOptimization::Balanced,
            network_limits: NetworkLimits {
                max_bytes_per_minute: 1024 * 1024,
                max_connections: 5,
                prefer_wifi: true,
            },
            cache_config: CacheConfig {
                max_cache_size_mb: 10,
                cache_ttl_seconds: 3600,
                persistent_cache: true,
            },
        };
        
        let api = MobileAPI::new(config).await.unwrap();
        let device_id = Uuid::new_v4();
        let device_type = DeviceType::Mobile {
            platform: crate::MobilePlatform::Android,
            capabilities: crate::MobileCapabilities {
                ram_mb: 4096,
                storage_gb: 64,
                has_secure_enclave: true,
                supports_biometrics: true,
                network_types: vec![crate::NetworkType::FiveG],
            },
        };
        
        let session_id = api.create_session(device_id, device_type, ConnectionType::RestAPI).await.unwrap();
        assert!(!session_id.is_empty());
        
        let session = api.get_session(&session_id).await.unwrap();
        assert_eq!(session.device_id, device_id);
        assert_eq!(session.status, SessionStatus::Active);
    }
}
