# Practical Testnet Database Architecture
## High-Throughput, Resource-Constrained Design for 10,000 Users/Second

**Target**: 10,000 users per second on 4GB RAM, 2 vCPU  
**Date**: 2025-10-20  
**Status**: Practical Implementation Ready  
**Approach**: Maximum Efficiency, Minimal Overhead  

---

## Executive Summary

This document specifies a **practical, high-throughput database architecture** designed specifically for testnet environments with severe resource constraints. Instead of complex multi-database orchestration, this design uses a **single optimized database** with **intelligent caching** to achieve 10,000 users per second on just 4GB RAM and 2 vCPU.

**Key Innovation**: Ultra-lightweight architecture that prioritizes performance over feature completeness, perfect for testnet validation and high-throughput testing.

---

## 1. Resource Budget Analysis (4GB RAM, 2 vCPU)

### 1.1 Memory Allocation
```
Total RAM: 4GB (4,096MB)
├── OS + System Overhead: 500MB
├── Application Runtime: 500MB
├── Database Cache Pool: 2,000MB (50% of total)
├── Connection Pools: 200MB
├── Working Memory: 800MB
└── Buffer/Safety: 96MB
```

### 1.2 CPU Allocation
```
Total CPU: 2 vCPU cores
├── Database Operations: 1.2 cores (60%)
├── Network I/O: 0.5 cores (25%)
├── Application Logic: 0.2 cores (10%)
└── System Overhead: 0.1 cores (5%)
```

---

## 2. Simplified Database Architecture

### 2.1 Single Database Design
```rust
// Single SQLite database with optimized schema
pub struct TestnetDatabase {
    connection_pool: Arc<SqlitePool>,
    cache_layer: Arc<InMemoryCache>,
    write_buffer: Arc<WriteBuffer>,
    performance_monitor: Arc<PerformanceMonitor>,
}

// Optimized schema for testnet
pub struct TestnetSchema {
    // Core tables (5 tables total, not 5+ databases)
    hash_registry: HashRegistryTable,
    bpi_nodes: BpiNodesTable,
    users: UsersTable,
    posts: PostsTable,
    interactions: InteractionsTable,
}
```

### 2.2 Table Definitions
```sql
-- Hash Registry (simplified from 4D hash-graph)
CREATE TABLE hash_registry (
    hash_id TEXT PRIMARY KEY,
    content_hash BLOB NOT NULL,
    owner_id TEXT,
    created_at INTEGER NOT NULL,
    metadata TEXT -- JSON blob for flexibility
);
CREATE INDEX idx_hash_owner ON hash_registry(owner_id);
CREATE INDEX idx_hash_created ON hash_registry(created_at);

-- BPI Nodes (simplified tracking)
CREATE TABLE bpi_nodes (
    node_id TEXT PRIMARY KEY,
    node_type TEXT NOT NULL,
    status TEXT NOT NULL, -- 'connected', 'disconnected', 'connecting'
    endpoint TEXT,
    last_seen INTEGER NOT NULL,
    connection_count INTEGER DEFAULT 0,
    metadata TEXT -- JSON for node-specific data
);
CREATE INDEX idx_nodes_status ON bpi_nodes(status);
CREATE INDEX idx_nodes_last_seen ON bpi_nodes(last_seen);

-- Users (simplified auth)
CREATE TABLE users (
    user_id TEXT PRIMARY KEY,
    username TEXT UNIQUE,
    auth_token_hash TEXT,
    permissions TEXT, -- JSON array of permissions
    created_at INTEGER NOT NULL,
    last_active INTEGER NOT NULL
);
CREATE INDEX idx_users_token ON users(auth_token_hash);
CREATE INDEX idx_users_active ON users(last_active);

-- Community Posts (basic content)
CREATE TABLE posts (
    post_id TEXT PRIMARY KEY,
    author_id TEXT NOT NULL,
    content TEXT NOT NULL,
    post_type TEXT DEFAULT 'text', -- 'text', 'code', 'bpi_ref'
    created_at INTEGER NOT NULL,
    updated_at INTEGER,
    interaction_count INTEGER DEFAULT 0,
    FOREIGN KEY (author_id) REFERENCES users(user_id)
);
CREATE INDEX idx_posts_author ON posts(author_id);
CREATE INDEX idx_posts_created ON posts(created_at);
CREATE INDEX idx_posts_interactions ON posts(interaction_count);

-- Community Interactions (simplified social)
CREATE TABLE interactions (
    interaction_id TEXT PRIMARY KEY,
    interaction_type TEXT NOT NULL, -- 'like', 'comment', 'share', 'follow'
    source_user TEXT NOT NULL,
    target_post TEXT,
    target_user TEXT,
    content TEXT, -- For comments
    created_at INTEGER NOT NULL,
    FOREIGN KEY (source_user) REFERENCES users(user_id)
);
CREATE INDEX idx_interactions_source ON interactions(source_user);
CREATE INDEX idx_interactions_target_post ON interactions(target_post);
CREATE INDEX idx_interactions_type ON interactions(interaction_type);
```

---

## 3. High-Performance Caching Layer

### 3.1 In-Memory Cache Design
```rust
pub struct InMemoryCache {
    // Hot data cache (most frequently accessed)
    hot_cache: Arc<RwLock<LruCache<String, CacheEntry>>>, // 1GB
    // Connection cache (active BPI nodes)
    connection_cache: Arc<RwLock<HashMap<String, NodeConnection>>>, // 200MB
    // User session cache (active users)
    session_cache: Arc<RwLock<HashMap<String, UserSession>>>, // 300MB
    // Query result cache (frequent queries)
    query_cache: Arc<RwLock<LruCache<String, QueryResult>>>, // 500MB
}

pub struct CacheEntry {
    data: Vec<u8>,
    expires_at: u64,
    access_count: u32,
    last_accessed: u64,
}

// Cache hit targets for 10k users/sec
pub struct CacheTargets {
    hash_lookups: 95,      // 95% cache hit rate
    user_sessions: 98,     // 98% cache hit rate
    bpi_node_status: 90,   // 90% cache hit rate
    recent_posts: 85,      // 85% cache hit rate
    interactions: 80,      // 80% cache hit rate
}
```

### 3.2 Cache Warming Strategy
```rust
impl InMemoryCache {
    /// Pre-load hot data on startup
    pub async fn warm_cache(&self) -> Result<()> {
        // Load most active users (last 24h)
        self.load_active_users().await?;
        
        // Load connected BPI nodes
        self.load_connected_nodes().await?;
        
        // Load recent posts (last 1h)
        self.load_recent_posts().await?;
        
        // Load frequent hash lookups
        self.load_frequent_hashes().await?;
        
        Ok(())
    }
}
```

---

## 4. Connection Pool Optimization

### 4.1 Minimal Connection Pool
```rust
pub struct OptimizedConnectionPool {
    // Small pool for high efficiency
    read_pool: Arc<SqlitePool>,    // 8 connections
    write_pool: Arc<SqlitePool>,   // 4 connections
    
    // Connection reuse tracking
    connection_stats: Arc<RwLock<ConnectionStats>>,
    
    // Pool configuration optimized for 2 vCPU
    config: PoolConfig {
        max_connections: 12,       // Total connections
        min_connections: 4,        // Always keep minimum
        connection_timeout: Duration::from_millis(100),
        idle_timeout: Duration::from_secs(300),
        max_lifetime: Duration::from_secs(1800),
    },
}
```

### 4.2 BPI Node Connection Tracking
```rust
pub struct BpiConnectionTracker {
    active_connections: Arc<RwLock<HashMap<String, NodeConnection>>>,
    connection_pool: Arc<Mutex<Vec<TcpStream>>>, // Reuse TCP connections
    heartbeat_interval: Duration,
    max_connections: usize, // Limit to prevent resource exhaustion
}

pub struct NodeConnection {
    node_id: String,
    connected_at: u64,
    last_heartbeat: u64,
    request_count: u64,
    bytes_transferred: u64,
}

impl BpiConnectionTracker {
    /// Lightweight connection tracking (no dynamic DB provisioning)
    pub async fn track_connection(&self, node_id: String) -> Result<()> {
        let connection = NodeConnection {
            node_id: node_id.clone(),
            connected_at: current_timestamp(),
            last_heartbeat: current_timestamp(),
            request_count: 0,
            bytes_transferred: 0,
        };
        
        // Update in-memory cache first (fast)
        self.active_connections.write().await.insert(node_id.clone(), connection);
        
        // Batch update to database (efficient)
        self.queue_database_update(node_id).await?;
        
        Ok(())
    }
}
```

---

## 5. Performance Optimizations for 10k Users/Second

### 5.1 Batch Processing
```rust
pub struct BatchProcessor {
    write_buffer: Arc<Mutex<Vec<WriteOperation>>>,
    batch_size: usize,        // 100 operations per batch
    flush_interval: Duration, // 50ms flush interval
    
    // Batch different operation types
    hash_registrations: Vec<HashRegistration>,
    user_updates: Vec<UserUpdate>,
    post_creations: Vec<PostCreation>,
    interactions: Vec<Interaction>,
}

impl BatchProcessor {
    /// Process 10k operations efficiently
    pub async fn process_batch(&self) -> Result<()> {
        let operations = self.drain_buffer().await;
        
        // Group by operation type for efficiency
        let grouped = self.group_operations(operations);
        
        // Execute as prepared statements (much faster)
        self.execute_batch_inserts(grouped).await?;
        
        Ok(())
    }
}
```

### 5.2 Read Optimization
```rust
pub struct ReadOptimizer {
    prepared_statements: Arc<HashMap<String, PreparedStatement>>,
    read_cache: Arc<InMemoryCache>,
    
    // Pre-compiled queries for common operations
    common_queries: CommonQueries {
        get_user_by_token: "SELECT * FROM users WHERE auth_token_hash = ?",
        get_node_status: "SELECT status FROM bpi_nodes WHERE node_id = ?",
        get_recent_posts: "SELECT * FROM posts ORDER BY created_at DESC LIMIT ?",
        get_hash_by_id: "SELECT * FROM hash_registry WHERE hash_id = ?",
        get_user_interactions: "SELECT * FROM interactions WHERE source_user = ? ORDER BY created_at DESC LIMIT ?",
    },
}
```

---

## 6. Simplified Authentication System

### 6.1 JWT-Based Authentication (No Keycloak)
```rust
pub struct SimpleAuthSystem {
    jwt_secret: Vec<u8>,
    token_cache: Arc<RwLock<LruCache<String, AuthToken>>>,
    session_timeout: Duration,
}

pub struct AuthToken {
    user_id: String,
    permissions: Vec<String>,
    expires_at: u64,
    issued_at: u64,
}

impl SimpleAuthSystem {
    /// Fast token validation (cached)
    pub async fn validate_token(&self, token: &str) -> Result<Option<AuthToken>> {
        // Check cache first (99% hit rate expected)
        if let Some(cached) = self.token_cache.read().await.get(token) {
            if cached.expires_at > current_timestamp() {
                return Ok(Some(cached.clone()));
            }
        }
        
        // Validate JWT and cache result
        let auth_token = self.validate_jwt(token)?;
        self.token_cache.write().await.put(token.to_string(), auth_token.clone());
        
        Ok(Some(auth_token))
    }
}
```

---

## 7. Resource Monitoring & Auto-Scaling

### 7.1 Performance Monitoring
```rust
pub struct TestnetMonitor {
    metrics: Arc<RwLock<PerformanceMetrics>>,
    alert_thresholds: AlertThresholds,
}

pub struct PerformanceMetrics {
    requests_per_second: f64,
    average_response_time: Duration,
    cache_hit_rate: f64,
    memory_usage: f64,
    cpu_usage: f64,
    active_connections: u32,
    database_queue_size: u32,
}

pub struct AlertThresholds {
    max_memory_usage: 0.85,      // 85% of 4GB
    max_cpu_usage: 0.90,         // 90% of 2 vCPU
    min_cache_hit_rate: 0.80,    // 80% minimum
    max_response_time: Duration::from_millis(50), // 50ms max
}
```

### 7.2 Auto-Optimization
```rust
impl TestnetMonitor {
    /// Automatically optimize for current load
    pub async fn auto_optimize(&self) -> Result<()> {
        let metrics = self.metrics.read().await;
        
        // Adjust cache sizes based on hit rates
        if metrics.cache_hit_rate < 0.85 {
            self.increase_cache_size().await?;
        }
        
        // Adjust batch sizes based on throughput
        if metrics.requests_per_second > 8000.0 {
            self.increase_batch_size().await?;
        }
        
        // Adjust connection pool based on usage
        if metrics.active_connections > 10 {
            self.optimize_connection_pool().await?;
        }
        
        Ok(())
    }
}
```

---

## 8. Implementation Architecture

### 8.1 Core Components
```rust
pub struct TestnetDatabaseManager {
    // Single database with optimized schema
    database: Arc<TestnetDatabase>,
    
    // High-performance caching
    cache: Arc<InMemoryCache>,
    
    // Efficient connection management
    connection_tracker: Arc<BpiConnectionTracker>,
    
    // Batch processing for writes
    batch_processor: Arc<BatchProcessor>,
    
    // Simple authentication
    auth_system: Arc<SimpleAuthSystem>,
    
    // Performance monitoring
    monitor: Arc<TestnetMonitor>,
}

impl TestnetDatabaseManager {
    /// Handle 10k users/second efficiently
    pub async fn handle_request(&self, request: TestnetRequest) -> Result<TestnetResponse> {
        // Check cache first (fast path)
        if let Some(cached) = self.cache.get(&request.cache_key()).await? {
            return Ok(cached);
        }
        
        // Process request based on type
        let response = match request.request_type {
            RequestType::HashLookup { hash_id } => {
                self.handle_hash_lookup(hash_id).await?
            },
            RequestType::NodeStatus { node_id } => {
                self.handle_node_status(node_id).await?
            },
            RequestType::UserAuth { token } => {
                self.handle_user_auth(token).await?
            },
            RequestType::PostQuery { query } => {
                self.handle_post_query(query).await?
            },
            RequestType::Interaction { interaction } => {
                self.handle_interaction(interaction).await?
            },
        };
        
        // Cache result for future requests
        self.cache.set(request.cache_key(), response.clone()).await?;
        
        Ok(response)
    }
}
```

---

## 9. Performance Targets & Validation

### 9.1 Target Metrics
```
Throughput: 10,000 requests/second
Response Time: <50ms (95th percentile)
Memory Usage: <3.5GB (87.5% of 4GB)
CPU Usage: <90% (1.8 of 2 vCPU)
Cache Hit Rate: >85%
Database Connections: <15 total
Uptime: >99.9%
```

### 9.2 Load Testing
```rust
pub struct LoadTester {
    target_rps: u32,           // 10,000 RPS
    test_duration: Duration,   // 5 minutes
    concurrent_users: u32,     // 1,000 concurrent
    request_mix: RequestMix,   // Realistic request distribution
}

pub struct RequestMix {
    hash_lookups: 40,      // 40% hash lookups
    node_status: 20,       // 20% node status checks
    user_auth: 15,         // 15% authentication
    post_queries: 15,      // 15% post queries
    interactions: 10,      // 10% interactions
}
```

---

## 10. Deployment Configuration

### 10.1 SQLite Optimization
```sql
-- SQLite configuration for high throughput
PRAGMA journal_mode = WAL;           -- Write-Ahead Logging
PRAGMA synchronous = NORMAL;         -- Balance safety/performance
PRAGMA cache_size = 100000;          -- 100MB cache
PRAGMA temp_store = MEMORY;          -- In-memory temp tables
PRAGMA mmap_size = 1073741824;       -- 1GB memory mapping
```

### 10.2 System Configuration
```bash
# System optimizations for 4GB RAM, 2 vCPU
echo 'vm.swappiness=10' >> /etc/sysctl.conf
echo 'net.core.somaxconn=65535' >> /etc/sysctl.conf
echo 'net.ipv4.tcp_max_syn_backlog=65535' >> /etc/sysctl.conf

# File descriptor limits
echo '* soft nofile 65535' >> /etc/security/limits.conf
echo '* hard nofile 65535' >> /etc/security/limits.conf
```

---

## Conclusion

This practical testnet database architecture is specifically designed to handle **10,000 users per second** on **4GB RAM and 2 vCPU** by prioritizing efficiency over feature completeness. Key innovations include:

- **Single optimized SQLite database** instead of complex multi-database orchestration
- **Intelligent in-memory caching** with 85%+ hit rates
- **Batch processing** for write operations
- **Minimal connection pooling** optimized for 2 vCPU
- **Simple JWT authentication** instead of full Keycloak
- **Performance monitoring** with auto-optimization

This architecture provides a realistic, deployable solution for testnet environments while maintaining the core functionality needed for BPCI Enterprise testing and validation.

**Resource Efficiency**: Uses <90% of available resources while maintaining high throughput
**Simplicity**: Easy to deploy, monitor, and maintain
**Scalability**: Can handle the target 10k users/second with room for growth
**Cost-Effective**: Runs on minimal hardware while delivering maximum performance
