# ⚖️ Load Balancing & Reliability

**BPI Mesh Gateway Agent** - High-availability load balancing, health monitoring, and reliability features for production HttpCG Gateway deployments.

---

## 🎯 **Overview**

The BPI Mesh Gateway Agent provides enterprise-grade load balancing, health monitoring, and reliability features for HttpCG Gateway deployments. Based on the real implementation at `/bpi-core/crates/metanode-core/gateway/src/bin/gateway.rs`, it delivers production-ready reliability with circuit breaker patterns, health checks, and intelligent request routing.

### **Key Features:**
- **Load Balancing Strategies** - Round-robin, least-connections, weighted distribution
- **Health Check Monitoring** - Continuous endpoint health validation
- **Circuit Breaker Pattern** - Automatic failure detection and recovery
- **Connection Pooling** - Efficient connection management and reuse
- **Metrics Collection** - Prometheus-compatible monitoring
- **Sidecar Mode** - Service mesh integration capability

---

## 🏗️ **Gateway Agent Architecture**

```
┌─────────────────────────────────────────────────────────────────┐
│                    BPI MESH GATEWAY AGENT                      │
├─────────────────────────────────────────────────────────────────┤
│  Request Ingress                                                │
│  ├── HTTP/HTTPS Request Handler                                │
│  ├── httpcg:// Protocol Parser                                │
│  ├── Connection Acceptance                                     │
│  └── Request Validation                                        │
├─────────────────────────────────────────────────────────────────┤
│  Load Balancing Layer                                           │
│  ├── Round-Robin Distribution                                  │
│  ├── Least-Connections Algorithm                               │
│  ├── Weighted Distribution                                     │
│  └── Health-Aware Routing                                     │
├─────────────────────────────────────────────────────────────────┤
│  Reliability Layer                                              │
│  ├── Circuit Breaker Pattern                                  │
│  ├── Health Check Monitoring                                  │
│  ├── Retry Logic with Backoff                                 │
│  └── Failover Management                                       │
├─────────────────────────────────────────────────────────────────┤
│  Connection Management                                           │
│  ├── Connection Pool Manager                                  │
│  ├── Keep-Alive Management                                    │
│  ├── Timeout Handling                                         │
│  └── Resource Cleanup                                         │
├─────────────────────────────────────────────────────────────────┤
│  Monitoring & Observability                                     │
│  ├── Prometheus Metrics Export                                │
│  ├── Health Check Endpoints                                   │
│  ├── Request/Response Logging                                 │
│  └── Performance Analytics                                    │
└─────────────────────────────────────────────────────────────────┘
```

---

## ⚖️ **Load Balancing Strategies**

### **Round-Robin Distribution**

```rust
#[derive(Debug, Clone)]
pub enum LoadBalancingStrategy {
    RoundRobin,
    LeastConnections,
    WeightedRoundRobin,
    IpHash,
    Random,
}

pub struct RoundRobinBalancer {
    endpoints: Vec<String>,
    current_index: Arc<AtomicUsize>,
}

impl RoundRobinBalancer {
    pub fn new(endpoints: Vec<String>) -> Self {
        Self {
            endpoints,
            current_index: Arc::new(AtomicUsize::new(0)),
        }
    }
    
    pub fn next_endpoint(&self) -> Option<&String> {
        if self.endpoints.is_empty() {
            return None;
        }
        
        let index = self.current_index.fetch_add(1, Ordering::Relaxed) % self.endpoints.len();
        self.endpoints.get(index)
    }
}
```

### **Least-Connections Algorithm**

```rust
pub struct LeastConnectionsBalancer {
    endpoints: Arc<RwLock<Vec<EndpointState>>>,
}

#[derive(Debug, Clone)]
struct EndpointState {
    url: String,
    active_connections: Arc<AtomicUsize>,
    is_healthy: Arc<AtomicBool>,
    last_health_check: Arc<RwLock<Instant>>,
}

impl LeastConnectionsBalancer {
    pub async fn next_endpoint(&self) -> Option<String> {
        let endpoints = self.endpoints.read().await;
        
        endpoints
            .iter()
            .filter(|ep| ep.is_healthy.load(Ordering::Relaxed))
            .min_by_key(|ep| ep.active_connections.load(Ordering::Relaxed))
            .map(|ep| ep.url.clone())
    }
    
    pub async fn increment_connections(&self, endpoint: &str) {
        let endpoints = self.endpoints.read().await;
        if let Some(ep) = endpoints.iter().find(|e| e.url == endpoint) {
            ep.active_connections.fetch_add(1, Ordering::Relaxed);
        }
    }
    
    pub async fn decrement_connections(&self, endpoint: &str) {
        let endpoints = self.endpoints.read().await;
        if let Some(ep) = endpoints.iter().find(|e| e.url == endpoint) {
            ep.active_connections.fetch_sub(1, Ordering::Relaxed);
        }
    }
}
```

### **Weighted Distribution**

```rust
pub struct WeightedBalancer {
    endpoints: Vec<WeightedEndpoint>,
    total_weight: u32,
}

#[derive(Debug, Clone)]
struct WeightedEndpoint {
    url: String,
    weight: u32,
    current_weight: Arc<AtomicU32>,
}

impl WeightedBalancer {
    pub fn next_endpoint(&self) -> Option<&String> {
        let mut best_endpoint: Option<&WeightedEndpoint> = None;
        let mut best_weight = 0i32;
        
        for endpoint in &self.endpoints {
            let current = endpoint.current_weight.load(Ordering::Relaxed) as i32;
            let new_weight = current + endpoint.weight as i32;
            endpoint.current_weight.store(new_weight as u32, Ordering::Relaxed);
            
            if new_weight > best_weight {
                best_weight = new_weight;
                best_endpoint = Some(endpoint);
            }
        }
        
        if let Some(best) = best_endpoint {
            let current = best.current_weight.load(Ordering::Relaxed) as i32;
            best.current_weight.store((current - self.total_weight as i32) as u32, Ordering::Relaxed);
            Some(&best.url)
        } else {
            None
        }
    }
}
```

---

## 🔄 **Health Check Monitoring**

### **Health Check Implementation**

```rust
pub struct HealthChecker {
    endpoints: Arc<RwLock<Vec<EndpointHealth>>>,
    check_interval: Duration,
    timeout: Duration,
    unhealthy_threshold: u32,
    healthy_threshold: u32,
}

#[derive(Debug, Clone)]
struct EndpointHealth {
    url: String,
    is_healthy: Arc<AtomicBool>,
    consecutive_failures: Arc<AtomicU32>,
    consecutive_successes: Arc<AtomicU32>,
    last_check: Arc<RwLock<Instant>>,
    response_time: Arc<RwLock<Duration>>,
}

impl HealthChecker {
    pub async fn start_monitoring(&self) -> Result<()> {
        let mut interval = tokio::time::interval(self.check_interval);
        
        loop {
            interval.tick().await;
            self.check_all_endpoints().await;
        }
    }
    
    async fn check_all_endpoints(&self) {
        let endpoints = self.endpoints.read().await;
        let mut tasks = Vec::new();
        
        for endpoint in endpoints.iter() {
            let endpoint_clone = endpoint.clone();
            let timeout = self.timeout;
            let unhealthy_threshold = self.unhealthy_threshold;
            let healthy_threshold = self.healthy_threshold;
            
            let task = tokio::spawn(async move {
                Self::check_endpoint_health(
                    endpoint_clone, 
                    timeout, 
                    unhealthy_threshold, 
                    healthy_threshold
                ).await;
            });
            
            tasks.push(task);
        }
        
        // Wait for all health checks to complete
        for task in tasks {
            let _ = task.await;
        }
    }
    
    async fn check_endpoint_health(
        endpoint: EndpointHealth,
        timeout: Duration,
        unhealthy_threshold: u32,
        healthy_threshold: u32,
    ) {
        let start_time = Instant::now();
        let health_url = format!("{}/health", endpoint.url);
        
        let result = tokio::time::timeout(timeout, async {
            reqwest::Client::new()
                .get(&health_url)
                .send()
                .await
        }).await;
        
        let response_time = start_time.elapsed();
        *endpoint.response_time.write().await = response_time;
        *endpoint.last_check.write().await = Instant::now();
        
        match result {
            Ok(Ok(response)) if response.status().is_success() => {
                // Health check succeeded
                endpoint.consecutive_failures.store(0, Ordering::Relaxed);
                let successes = endpoint.consecutive_successes.fetch_add(1, Ordering::Relaxed) + 1;
                
                if successes >= healthy_threshold {
                    endpoint.is_healthy.store(true, Ordering::Relaxed);
                    tracing::info!("Endpoint {} is healthy (response time: {:?})", 
                        endpoint.url, response_time);
                }
            },
            _ => {
                // Health check failed
                endpoint.consecutive_successes.store(0, Ordering::Relaxed);
                let failures = endpoint.consecutive_failures.fetch_add(1, Ordering::Relaxed) + 1;
                
                if failures >= unhealthy_threshold {
                    endpoint.is_healthy.store(false, Ordering::Relaxed);
                    tracing::warn!("Endpoint {} is unhealthy (failures: {})", 
                        endpoint.url, failures);
                }
            }
        }
    }
}
```

---

## 🛡️ **Circuit Breaker Pattern**

### **Circuit Breaker Implementation**

```rust
pub struct CircuitBreaker {
    failure_count: Arc<AtomicUsize>,
    last_failure_time: Arc<RwLock<Option<Instant>>>,
    failure_threshold: usize,
    recovery_timeout: Duration,
    state: Arc<RwLock<CircuitState>>,
    half_open_max_calls: usize,
    half_open_calls: Arc<AtomicUsize>,
}

#[derive(Debug, Clone, PartialEq)]
enum CircuitState {
    Closed,    // Normal operation
    Open,      // Failing, reject requests
    HalfOpen,  // Testing recovery
}

impl CircuitBreaker {
    pub fn new(failure_threshold: usize, recovery_timeout: Duration) -> Self {
        Self {
            failure_count: Arc::new(AtomicUsize::new(0)),
            last_failure_time: Arc::new(RwLock::new(None)),
            failure_threshold,
            recovery_timeout,
            state: Arc::new(RwLock::new(CircuitState::Closed)),
            half_open_max_calls: 3,
            half_open_calls: Arc::new(AtomicUsize::new(0)),
        }
    }
    
    pub async fn call<F, T, E>(&self, f: F) -> Result<T, CircuitBreakerError<E>>
    where
        F: Future<Output = Result<T, E>>,
        E: std::fmt::Debug,
    {
        let state = *self.state.read().await;
        
        match state {
            CircuitState::Open => {
                // Check if we should transition to half-open
                if let Some(last_failure) = *self.last_failure_time.read().await {
                    if last_failure.elapsed() > self.recovery_timeout {
                        *self.state.write().await = CircuitState::HalfOpen;
                        self.half_open_calls.store(0, Ordering::Relaxed);
                    } else {
                        return Err(CircuitBreakerError::CircuitOpen);
                    }
                }
            },
            CircuitState::HalfOpen => {
                // Limit calls in half-open state
                let calls = self.half_open_calls.fetch_add(1, Ordering::Relaxed);
                if calls >= self.half_open_max_calls {
                    return Err(CircuitBreakerError::CircuitOpen);
                }
            },
            CircuitState::Closed => {
                // Normal operation
            }
        }
        
        // Execute the function
        match f.await {
            Ok(result) => {
                self.on_success().await;
                Ok(result)
            },
            Err(e) => {
                self.on_failure().await;
                Err(CircuitBreakerError::CallFailed(e))
            }
        }
    }
    
    async fn on_success(&self) {
        let state = *self.state.read().await;
        
        match state {
            CircuitState::HalfOpen => {
                // Successful call in half-open state - close the circuit
                *self.state.write().await = CircuitState::Closed;
                self.failure_count.store(0, Ordering::Relaxed);
                tracing::info!("Circuit breaker closed - service recovered");
            },
            CircuitState::Closed => {
                // Reset failure count on success
                self.failure_count.store(0, Ordering::Relaxed);
            },
            _ => {}
        }
    }
    
    async fn on_failure(&self) {
        let failures = self.failure_count.fetch_add(1, Ordering::Relaxed) + 1;
        *self.last_failure_time.write().await = Some(Instant::now());
        
        let state = *self.state.read().await;
        
        match state {
            CircuitState::Closed => {
                if failures >= self.failure_threshold {
                    *self.state.write().await = CircuitState::Open;
                    tracing::warn!("Circuit breaker opened - too many failures ({})", failures);
                }
            },
            CircuitState::HalfOpen => {
                // Failure in half-open state - reopen the circuit
                *self.state.write().await = CircuitState::Open;
                tracing::warn!("Circuit breaker reopened - recovery failed");
            },
            _ => {}
        }
    }
}

#[derive(Debug)]
pub enum CircuitBreakerError<E> {
    CircuitOpen,
    CallFailed(E),
}
```

---

## 🔗 **Connection Management**

### **Connection Pool Implementation**

```rust
pub struct ConnectionPool {
    pools: Arc<RwLock<HashMap<String, Pool<PooledConnection>>>>,
    max_connections_per_host: usize,
    connection_timeout: Duration,
    idle_timeout: Duration,
    max_idle_time: Duration,
}

#[derive(Debug)]
struct PooledConnection {
    connection: reqwest::Client,
    created_at: Instant,
    last_used: Arc<RwLock<Instant>>,
    is_valid: Arc<AtomicBool>,
}

impl ConnectionPool {
    pub async fn get_connection(&self, host: &str) -> Result<PooledConnection> {
        let pools = self.pools.read().await;
        
        if let Some(pool) = pools.get(host) {
            // Try to get an existing connection
            if let Some(conn) = pool.try_get() {
                if self.is_connection_valid(&conn).await {
                    *conn.last_used.write().await = Instant::now();
                    return Ok(conn);
                }
            }
        }
        
        // Create new connection if none available
        drop(pools);
        self.create_connection(host).await
    }
    
    async fn create_connection(&self, host: &str) -> Result<PooledConnection> {
        let client = reqwest::Client::builder()
            .timeout(self.connection_timeout)
            .pool_idle_timeout(Some(self.idle_timeout))
            .pool_max_idle_per_host(self.max_connections_per_host)
            .build()
            .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))?;
        
        let connection = PooledConnection {
            connection: client,
            created_at: Instant::now(),
            last_used: Arc::new(RwLock::new(Instant::now())),
            is_valid: Arc::new(AtomicBool::new(true)),
        };
        
        // Add to pool
        let mut pools = self.pools.write().await;
        let pool = pools.entry(host.to_string()).or_insert_with(|| {
            Pool::new(self.max_connections_per_host)
        });
        
        pool.put(connection.clone());
        Ok(connection)
    }
    
    async fn is_connection_valid(&self, conn: &PooledConnection) -> bool {
        // Check if connection is still valid
        if !conn.is_valid.load(Ordering::Relaxed) {
            return false;
        }
        
        // Check age
        if conn.created_at.elapsed() > self.max_idle_time {
            conn.is_valid.store(false, Ordering::Relaxed);
            return false;
        }
        
        // Check last used time
        let last_used = *conn.last_used.read().await;
        if last_used.elapsed() > self.idle_timeout {
            conn.is_valid.store(false, Ordering::Relaxed);
            return false;
        }
        
        true
    }
    
    pub async fn cleanup_expired_connections(&self) {
        let mut pools = self.pools.write().await;
        
        for (host, pool) in pools.iter_mut() {
            pool.retain(|conn| self.is_connection_valid(conn));
            tracing::debug!("Cleaned up expired connections for host: {}", host);
        }
    }
}
```

---

## 📊 **Monitoring & Metrics**

### **Gateway Metrics Collection**

```rust
pub struct GatewayMetrics {
    // Request metrics
    request_total: Counter,
    request_duration: Histogram,
    request_size: Histogram,
    response_size: Histogram,
    
    // Load balancing metrics
    backend_requests: Counter,
    backend_failures: Counter,
    backend_response_time: Histogram,
    
    // Health check metrics
    health_check_total: Counter,
    health_check_failures: Counter,
    healthy_backends: Gauge,
    
    // Circuit breaker metrics
    circuit_breaker_state: Gauge,
    circuit_breaker_transitions: Counter,
    
    // Connection pool metrics
    active_connections: Gauge,
    connection_pool_size: Gauge,
    connection_creation_total: Counter,
}

impl GatewayMetrics {
    pub fn record_request(&self, 
        method: &str, 
        backend: &str, 
        status: u16, 
        duration: Duration,
        request_size: usize,
        response_size: usize,
    ) {
        self.request_total
            .with_label_values(&[method, backend, &status.to_string()])
            .inc();
        
        self.request_duration
            .with_label_values(&[method, backend])
            .observe(duration.as_secs_f64());
        
        self.request_size
            .with_label_values(&[method, backend])
            .observe(request_size as f64);
        
        self.response_size
            .with_label_values(&[method, backend])
            .observe(response_size as f64);
        
        if status >= 400 {
            self.backend_failures
                .with_label_values(&[backend, &status.to_string()])
                .inc();
        }
    }
    
    pub fn update_health_metrics(&self, healthy_count: usize, total_count: usize) {
        self.healthy_backends.set(healthy_count as f64);
        
        // Calculate health ratio
        let health_ratio = if total_count > 0 {
            healthy_count as f64 / total_count as f64
        } else {
            0.0
        };
        
        tracing::info!("Backend health: {}/{} ({:.1}%)", 
            healthy_count, total_count, health_ratio * 100.0);
    }
    
    pub fn record_circuit_breaker_state(&self, backend: &str, state: &str) {
        let state_value = match state {
            "closed" => 0.0,
            "half_open" => 0.5,
            "open" => 1.0,
            _ => -1.0,
        };
        
        self.circuit_breaker_state
            .with_label_values(&[backend])
            .set(state_value);
    }
}
```

---

## 🚀 **DockLock Deployment Configuration**

### **High-Availability Gateway Deployment**

```yaml
# httpcg-gateway-ha.yml
apiVersion: docklock.bpi.network/v1
kind: ClusterDeployment
metadata:
  name: httpcg-gateway-ha
  namespace: bpci-infrastructure
spec:
  cluster:
    replicas: 3
    distribution: "zone-aware"
    loadBalancing:
      algorithm: "least-connections"
      healthCheck:
        path: "/health"
        interval: "10s"
        timeout: "5s"
        unhealthyThreshold: 3
        healthyThreshold: 2
  
  template:
    spec:
      app:
        name: httpcg-gateway
        binary: "/usr/local/bin/gateway"
        args: [
          "--gateway-id", "gateway-${POD_NAME}",
          "--listen-addr", "0.0.0.0:8080",
          "--relay-endpoints", "http://backend-1:8001,http://backend-2:8002,http://backend-3:8003",
          "--health-check-interval-ms", "5000",
          "--max-connections", "1000",
          "--request-timeout-ms", "30000",
          "--retry-attempts", "3",
          "--circuit-breaker-threshold", "5",
          "--load-balancing", "least-connections",
          "--metrics-enabled"
        ]
      
      resources:
        memory: "2Gi"
        cpu: "1000m"
        disk: "5Gi"
      
      health:
        livenessProbe:
          httpGet:
            path: "/health"
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: "/ready"
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
      
      monitoring:
        metrics:
          enabled: true
          port: 9090
          path: "/metrics"
```

---

*The BPI Mesh Gateway Agent provides enterprise-grade load balancing and reliability features essential for production HttpCG Gateway deployments with high availability and fault tolerance.*
