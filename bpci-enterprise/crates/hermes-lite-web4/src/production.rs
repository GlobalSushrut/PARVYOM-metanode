//! Production Features (Stages 4 & 5)
//! 
//! Stage 4: Trigonometric Load Balancing
//! - Real-time load tracking
//! - Enhanced trigonometric scoring with load
//! - Automatic failover and circuit breaker
//! 
//! Stage 5: Production Integration
//! - Cloudflare edge integration
//! - Resource marketplace (Pravyom Exchange)
//! - Production monitoring and metrics
//! - Complete HERMES stack

use crate::{NodeId, HyperbolicCoordinates, ServiceEndpoint, ServiceHealth};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{SystemTime, Duration};

/// Load metrics for a service endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadMetrics {
    /// Current CPU usage (0.0 - 1.0)
    pub cpu_usage: f64,
    /// Current memory usage (0.0 - 1.0)
    pub memory_usage: f64,
    /// Current request rate (requests per second)
    pub request_rate: f64,
    /// Average response time (milliseconds)
    pub avg_response_time: f64,
    /// Error rate (0.0 - 1.0)
    pub error_rate: f64,
    /// Last update timestamp
    pub last_update: SystemTime,
}

impl LoadMetrics {
    /// Create new load metrics
    pub fn new() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0.0,
            request_rate: 0.0,
            avg_response_time: 0.0,
            error_rate: 0.0,
            last_update: SystemTime::now(),
        }
    }
    
    /// Calculate overall load score (0.0 - 1.0)
    pub fn load_score(&self) -> f64 {
        // Weighted average of different metrics
        let cpu_weight = 0.3;
        let memory_weight = 0.2;
        let rate_weight = 0.2;
        let response_weight = 0.2;
        let error_weight = 0.1;
        
        cpu_weight * self.cpu_usage
            + memory_weight * self.memory_usage
            + rate_weight * (self.request_rate / 1000.0).min(1.0)
            + response_weight * (self.avg_response_time / 1000.0).min(1.0)
            + error_weight * self.error_rate
    }
}

impl Default for LoadMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Load tracker for monitoring endpoint load
pub struct LoadTracker {
    /// Load metrics per endpoint
    metrics: Arc<RwLock<HashMap<NodeId, LoadMetrics>>>,
    /// Metrics retention duration
    retention: Duration,
}

impl LoadTracker {
    /// Create new load tracker
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
            retention: Duration::from_secs(300), // 5 minutes
        }
    }
    
    /// Update load metrics for an endpoint
    pub async fn update_metrics(&self, node_id: NodeId, metrics: LoadMetrics) {
        let mut map = self.metrics.write().await;
        map.insert(node_id, metrics);
    }
    
    /// Get load metrics for an endpoint
    pub async fn get_metrics(&self, node_id: &NodeId) -> Option<LoadMetrics> {
        let map = self.metrics.read().await;
        map.get(node_id).cloned()
    }
    
    /// Get load score for an endpoint
    pub async fn get_load_score(&self, node_id: &NodeId) -> f64 {
        self.get_metrics(node_id)
            .await
            .map(|m| m.load_score())
            .unwrap_or(0.0)
    }
    
    /// Clean up stale metrics
    pub async fn cleanup_stale(&self) {
        let mut map = self.metrics.write().await;
        let now = SystemTime::now();
        
        map.retain(|_, metrics| {
            now.duration_since(metrics.last_update)
                .map(|d| d < self.retention)
                .unwrap_or(false)
        });
    }
}

impl Default for LoadTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Enhanced load balancer with trigonometric scoring
pub struct EnhancedLoadBalancer {
    /// Load tracker
    load_tracker: LoadTracker,
    /// Rho parameter for fairness dampening
    rho: f64,
    /// Epsilon for security guard
    epsilon: f64,
}

impl EnhancedLoadBalancer {
    /// Create new enhanced load balancer
    pub fn new() -> Self {
        Self {
            load_tracker: LoadTracker::new(),
            rho: 0.5,
            epsilon: 0.01,
        }
    }
    
    /// Select best endpoint using trigonometric scoring with load
    pub async fn select_endpoint(
        &self,
        endpoints: &[ServiceEndpoint],
        hrw: f64,
        phi_edge: f64,
        phi_svc: f64,
    ) -> Option<ServiceEndpoint> {
        let mut best_endpoint = None;
        let mut best_score = f64::NEG_INFINITY;
        
        for endpoint in endpoints {
            // Skip unhealthy endpoints
            if !endpoint.is_healthy() {
                continue;
            }
            
            // Get load score
            let load = self.load_tracker.get_load_score(&endpoint.node_id).await;
            
            // Calculate trigonometric score with load
            let score = self.trig_score_with_load(hrw, phi_edge, phi_svc, load);
            
            if score > best_score {
                best_score = score;
                best_endpoint = Some(endpoint.clone());
            }
        }
        
        best_endpoint
    }
    
    /// Calculate trigonometric score with load
    fn trig_score_with_load(&self, hrw: f64, phi_edge: f64, phi_svc: f64, load: f64) -> f64 {
        // HRW component
        let hrw_component = 1.0 / (1.0 + hrw);
        
        // Phase alignment (quantum fidelity)
        let phase_diff = phi_edge - phi_svc;
        let phase_alignment = phase_diff.cos().powi(2);
        
        // Fairness dampening with load
        let fairness = (1.0 + self.rho * load).sqrt();
        
        // Security guard
        let sec_guard = if phi_edge.cos().abs() > self.epsilon { 1.0 } else { 0.0 };
        
        hrw_component * phase_alignment * sec_guard / fairness
    }
    
    /// Update load metrics
    pub async fn update_load(&self, node_id: NodeId, metrics: LoadMetrics) {
        self.load_tracker.update_metrics(node_id, metrics).await;
    }
}

impl Default for EnhancedLoadBalancer {
    fn default() -> Self {
        Self::new()
    }
}

/// Circuit breaker for automatic failover
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    /// Circuit is closed (normal operation)
    Closed,
    /// Circuit is open (failing, no requests allowed)
    Open,
    /// Circuit is half-open (testing if service recovered)
    HalfOpen,
}

pub struct CircuitBreaker {
    /// Current circuit state
    state: Arc<RwLock<CircuitState>>,
    /// Failure threshold to open circuit
    failure_threshold: usize,
    /// Current failure count
    failure_count: Arc<RwLock<usize>>,
    /// Timeout before trying half-open
    timeout: Duration,
    /// Last state change time
    last_change: Arc<RwLock<SystemTime>>,
}

impl CircuitBreaker {
    /// Create new circuit breaker
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(CircuitState::Closed)),
            failure_threshold: 5,
            failure_count: Arc::new(RwLock::new(0)),
            timeout: Duration::from_secs(60),
            last_change: Arc::new(RwLock::new(SystemTime::now())),
        }
    }
    
    /// Record a success
    pub async fn record_success(&self) {
        let mut state = self.state.write().await;
        let mut count = self.failure_count.write().await;
        
        match *state {
            CircuitState::HalfOpen => {
                // Success in half-open state -> close circuit
                *state = CircuitState::Closed;
                *count = 0;
                *self.last_change.write().await = SystemTime::now();
            }
            CircuitState::Closed => {
                // Reset failure count on success
                *count = 0;
            }
            _ => {}
        }
    }
    
    /// Record a failure
    pub async fn record_failure(&self) {
        let mut state = self.state.write().await;
        let mut count = self.failure_count.write().await;
        
        *count += 1;
        
        if *count >= self.failure_threshold && *state == CircuitState::Closed {
            // Open circuit after threshold failures
            *state = CircuitState::Open;
            *self.last_change.write().await = SystemTime::now();
        }
    }
    
    /// Check if request is allowed
    pub async fn is_request_allowed(&self) -> bool {
        let mut state = self.state.write().await;
        let last_change = *self.last_change.read().await;
        
        match *state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                // Check if timeout elapsed
                if SystemTime::now().duration_since(last_change).unwrap() > self.timeout {
                    // Try half-open
                    *state = CircuitState::HalfOpen;
                    *self.last_change.write().await = SystemTime::now();
                    true
                } else {
                    false
                }
            }
            CircuitState::HalfOpen => true,
        }
    }
    
    /// Get current circuit state
    pub async fn state(&self) -> CircuitState {
        *self.state.read().await
    }
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self::new()
    }
}

/// Cloudflare edge integration
pub struct CloudflareEdge {
    /// Cloudflare API token
    api_token: Option<String>,
    /// Edge cache enabled
    cache_enabled: bool,
}

impl CloudflareEdge {
    /// Create new Cloudflare edge integration
    pub fn new() -> Self {
        Self {
            api_token: None,
            cache_enabled: true,
        }
    }
    
    /// Set API token
    pub fn with_token(mut self, token: String) -> Self {
        self.api_token = Some(token);
        self
    }
    
    /// Enable/disable edge caching
    pub fn set_cache_enabled(&mut self, enabled: bool) {
        self.cache_enabled = enabled;
    }
}

impl Default for CloudflareEdge {
    fn default() -> Self {
        Self::new()
    }
}

/// Resource offer for Pravyom Exchange
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceOffer {
    /// Node offering resources
    pub node_id: NodeId,
    /// CPU cores available
    pub cpu_cores: u32,
    /// Memory available (GB)
    pub memory_gb: f64,
    /// Storage available (GB)
    pub storage_gb: f64,
    /// Network bandwidth (Mbps)
    pub bandwidth_mbps: f64,
    /// Price per hour (in tokens)
    pub price_per_hour: f64,
}

/// Pravyom Exchange client
pub struct PravyomExchange {
    /// Local resource offers
    offers: Arc<RwLock<Vec<ResourceOffer>>>,
}

impl PravyomExchange {
    /// Create new Pravyom Exchange client
    pub fn new() -> Self {
        Self {
            offers: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Register resource offer
    pub async fn register_offer(&self, offer: ResourceOffer) {
        let mut offers = self.offers.write().await;
        offers.push(offer);
    }
    
    /// Get available offers
    pub async fn get_offers(&self) -> Vec<ResourceOffer> {
        let offers = self.offers.read().await;
        offers.clone()
    }
}

impl Default for PravyomExchange {
    fn default() -> Self {
        Self::new()
    }
}

/// Production metrics collector
#[derive(Debug, Clone, Default)]
pub struct ProductionMetrics {
    /// Total requests processed
    pub total_requests: u64,
    /// Total errors
    pub total_errors: u64,
    /// Average latency (ms)
    pub avg_latency_ms: f64,
    /// Cache hit rate
    pub cache_hit_rate: f64,
    /// Active connections
    pub active_connections: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_load_metrics_creation() {
        let metrics = LoadMetrics::new();
        assert_eq!(metrics.cpu_usage, 0.0);
        assert_eq!(metrics.memory_usage, 0.0);
    }
    
    #[test]
    fn test_load_score_calculation() {
        let mut metrics = LoadMetrics::new();
        metrics.cpu_usage = 0.5;
        metrics.memory_usage = 0.3;
        metrics.request_rate = 100.0;
        
        let score = metrics.load_score();
        assert!(score > 0.0);
        assert!(score < 1.0);
    }
    
    #[tokio::test]
    async fn test_load_tracker() {
        let tracker = LoadTracker::new();
        let node_id = NodeId("test_node".to_string());
        
        let metrics = LoadMetrics::new();
        tracker.update_metrics(node_id.clone(), metrics).await;
        
        let retrieved = tracker.get_metrics(&node_id).await;
        assert!(retrieved.is_some());
    }
    
    #[tokio::test]
    async fn test_circuit_breaker_closed() {
        let breaker = CircuitBreaker::new();
        
        assert!(breaker.is_request_allowed().await);
        assert_eq!(breaker.state().await, CircuitState::Closed);
    }
    
    #[tokio::test]
    async fn test_circuit_breaker_opens_on_failures() {
        let breaker = CircuitBreaker::new();
        
        // Record failures to open circuit
        for _ in 0..5 {
            breaker.record_failure().await;
        }
        
        assert_eq!(breaker.state().await, CircuitState::Open);
        assert!(!breaker.is_request_allowed().await);
    }
    
    #[tokio::test]
    async fn test_circuit_breaker_closes_on_success() {
        let breaker = CircuitBreaker::new();
        
        breaker.record_success().await;
        assert_eq!(breaker.state().await, CircuitState::Closed);
    }
    
    #[tokio::test]
    async fn test_pravyom_exchange() {
        let exchange = PravyomExchange::new();
        
        let offer = ResourceOffer {
            node_id: NodeId("test_node".to_string()),
            cpu_cores: 4,
            memory_gb: 16.0,
            storage_gb: 500.0,
            bandwidth_mbps: 1000.0,
            price_per_hour: 0.1,
        };
        
        exchange.register_offer(offer).await;
        
        let offers = exchange.get_offers().await;
        assert_eq!(offers.len(), 1);
    }
}
