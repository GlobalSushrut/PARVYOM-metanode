//! # vPod Performance Monitor
//! 
//! Real-time performance monitoring and optimization for vPod systems.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time::interval;

use crate::vpod::{VPodRuntime, VPodNode, RuntimeMetrics};

/// Performance monitor for vPod systems
#[derive(Debug)]
pub struct VPodPerformanceMonitor {
    /// Monitored runtimes
    runtimes: Arc<RwLock<HashMap<String, Arc<VPodRuntime>>>>,
    
    /// Monitored nodes
    nodes: Arc<RwLock<HashMap<String, Arc<VPodNode>>>>,
    
    /// Performance history
    performance_history: Arc<RwLock<Vec<PerformanceSnapshot>>>,
    
    /// Monitor configuration
    config: MonitorConfig,
    
    /// Alert thresholds
    alert_thresholds: Arc<RwLock<AlertThresholds>>,
}

/// Monitor configuration
#[derive(Debug, Clone)]
pub struct MonitorConfig {
    /// Monitoring interval
    pub monitoring_interval: Duration,
    
    /// History retention period
    pub history_retention: Duration,
    
    /// Enable real-time alerts
    pub real_time_alerts: bool,
    
    /// Performance optimization enabled
    pub auto_optimization: bool,
}

/// Performance snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSnapshot {
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Runtime metrics
    pub runtime_metrics: HashMap<String, RuntimeMetrics>,
    
    /// Node metrics
    pub node_metrics: HashMap<String, NodePerformanceMetrics>,
    
    /// System-wide metrics
    pub system_metrics: SystemMetrics,
}

/// Node performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePerformanceMetrics {
    /// CPU utilization (0.0 to 1.0)
    pub cpu_utilization: f64,
    
    /// Memory utilization (bytes)
    pub memory_utilization: u64,
    
    /// Network utilization (bytes/sec)
    pub network_utilization: u64,
    
    /// Actor efficiency
    pub actor_efficiency: f64,
    
    /// Message throughput (msgs/sec)
    pub message_throughput: f64,
    
    /// Average latency (microseconds)
    pub avg_latency_micros: f64,
}

/// System-wide performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// Total nodes
    pub total_nodes: u32,
    
    /// Total active actors
    pub total_active_actors: u64,
    
    /// System-wide throughput
    pub system_throughput: f64,
    
    /// System-wide latency
    pub system_latency: f64,
    
    /// Resource efficiency
    pub resource_efficiency: f64,
    
    /// Overall health score (0.0 to 1.0)
    pub health_score: f64,
}

/// Alert thresholds
#[derive(Debug, Clone)]
pub struct AlertThresholds {
    /// Maximum CPU utilization
    pub max_cpu_utilization: f64,
    
    /// Maximum memory utilization
    pub max_memory_utilization: u64,
    
    /// Maximum latency (microseconds)
    pub max_latency_micros: f64,
    
    /// Minimum throughput (msgs/sec)
    pub min_throughput: f64,
    
    /// Minimum efficiency score
    pub min_efficiency: f64,
}

impl VPodPerformanceMonitor {
    /// Create a new performance monitor
    pub fn new(config: MonitorConfig) -> Self {
        Self {
            runtimes: Arc::new(RwLock::new(HashMap::new())),
            nodes: Arc::new(RwLock::new(HashMap::new())),
            performance_history: Arc::new(RwLock::new(Vec::new())),
            config,
            alert_thresholds: Arc::new(RwLock::new(AlertThresholds::default())),
        }
    }
    
    /// Add runtime to monitor
    pub async fn add_runtime(&self, id: String, runtime: Arc<VPodRuntime>) {
        let mut runtimes = self.runtimes.write().await;
        runtimes.insert(id, runtime);
    }
    
    /// Add node to monitor
    pub async fn add_node(&self, id: String, node: Arc<VPodNode>) {
        let mut nodes = self.nodes.write().await;
        nodes.insert(id, node);
    }
    
    /// Start monitoring
    pub async fn start_monitoring(&self) -> Result<()> {
        let runtimes = self.runtimes.clone();
        let nodes = self.nodes.clone();
        let history = self.performance_history.clone();
        let config = self.config.clone();
        let thresholds = self.alert_thresholds.clone();
        
        tokio::spawn(async move {
            let mut interval = interval(config.monitoring_interval);
            
            loop {
                interval.tick().await;
                
                // Collect metrics
                let mut runtime_metrics = HashMap::new();
                let mut node_metrics = HashMap::new();
                
                // Collect runtime metrics
                {
                    let runtimes_guard = runtimes.read().await;
                    for (id, runtime) in runtimes_guard.iter() {
                        let metrics = runtime.get_metrics().await;
                        runtime_metrics.insert(id.clone(), metrics);
                    }
                }
                
                // Collect node metrics
                {
                    let nodes_guard = nodes.read().await;
                    for (id, node) in nodes_guard.iter() {
                        let node_perf_metrics = Self::collect_node_metrics(node).await;
                        node_metrics.insert(id.clone(), node_perf_metrics);
                    }
                }
                
                // Calculate system metrics
                let system_metrics = Self::calculate_system_metrics(&runtime_metrics, &node_metrics);
                
                // Create snapshot
                let snapshot = PerformanceSnapshot {
                    timestamp: chrono::Utc::now(),
                    runtime_metrics,
                    node_metrics,
                    system_metrics,
                };
                
                // Store snapshot
                {
                    let mut history_guard = history.write().await;
                    history_guard.push(snapshot.clone());
                    
                    // Maintain history size
                    let retention_limit = chrono::Utc::now() - 
                        chrono::Duration::from_std(config.history_retention).unwrap();
                    
                    history_guard.retain(|s| s.timestamp > retention_limit);
                }
                
                // Check alerts
                if config.real_time_alerts {
                    Self::check_alerts(&snapshot, &thresholds).await;
                }
            }
        });
        
        Ok(())
    }
    
    /// Collect node performance metrics
    async fn collect_node_metrics(node: &VPodNode) -> NodePerformanceMetrics {
        let node_metrics = node.get_metrics().await;
        
        NodePerformanceMetrics {
            cpu_utilization: node_metrics.cpu_utilization,
            memory_utilization: node_metrics.memory_utilization,
            network_utilization: node_metrics.network_utilization,
            actor_efficiency: node_metrics.actor_efficiency,
            message_throughput: node_metrics.throughput_mps,
            avg_latency_micros: node_metrics.avg_message_latency_micros,
        }
    }
    
    /// Calculate system-wide metrics
    fn calculate_system_metrics(
        runtime_metrics: &HashMap<String, RuntimeMetrics>,
        node_metrics: &HashMap<String, NodePerformanceMetrics>,
    ) -> SystemMetrics {
        let total_nodes = node_metrics.len() as u32;
        let total_active_actors: u64 = runtime_metrics.values()
            .map(|m| m.active_actors)
            .sum();
        
        let system_throughput: f64 = node_metrics.values()
            .map(|m| m.message_throughput)
            .sum();
        
        let system_latency: f64 = if !node_metrics.is_empty() {
            node_metrics.values()
                .map(|m| m.avg_latency_micros)
                .sum::<f64>() / node_metrics.len() as f64
        } else {
            0.0
        };
        
        let resource_efficiency: f64 = if !node_metrics.is_empty() {
            node_metrics.values()
                .map(|m| m.actor_efficiency)
                .sum::<f64>() / node_metrics.len() as f64
        } else {
            0.0
        };
        
        // Calculate health score based on various factors
        let health_score = Self::calculate_health_score(
            system_throughput,
            system_latency,
            resource_efficiency,
        );
        
        SystemMetrics {
            total_nodes,
            total_active_actors,
            system_throughput,
            system_latency,
            resource_efficiency,
            health_score,
        }
    }
    
    /// Calculate overall system health score
    fn calculate_health_score(
        throughput: f64,
        latency: f64,
        efficiency: f64,
    ) -> f64 {
        // Normalize metrics to 0-1 scale
        let throughput_score = (throughput / 1_000_000.0).min(1.0); // Normalize to 1M msgs/sec
        let latency_score = (1000.0 / latency.max(1.0)).min(1.0); // Better latency = higher score
        let efficiency_score = efficiency;
        
        // Weighted average
        (throughput_score * 0.4 + latency_score * 0.3 + efficiency_score * 0.3)
    }
    
    /// Check for performance alerts
    async fn check_alerts(
        snapshot: &PerformanceSnapshot,
        thresholds: &Arc<RwLock<AlertThresholds>>,
    ) {
        let thresholds_guard = thresholds.read().await;
        
        // Check system-wide alerts
        if snapshot.system_metrics.system_latency > thresholds_guard.max_latency_micros {
            eprintln!("ALERT: System latency {} exceeds threshold {}", 
                snapshot.system_metrics.system_latency, 
                thresholds_guard.max_latency_micros);
        }
        
        if snapshot.system_metrics.system_throughput < thresholds_guard.min_throughput {
            eprintln!("ALERT: System throughput {} below threshold {}", 
                snapshot.system_metrics.system_throughput, 
                thresholds_guard.min_throughput);
        }
        
        // Check node-specific alerts
        for (node_id, node_metrics) in &snapshot.node_metrics {
            if node_metrics.cpu_utilization > thresholds_guard.max_cpu_utilization {
                eprintln!("ALERT: Node {} CPU utilization {} exceeds threshold {}", 
                    node_id, node_metrics.cpu_utilization, thresholds_guard.max_cpu_utilization);
            }
            
            if node_metrics.memory_utilization > thresholds_guard.max_memory_utilization {
                eprintln!("ALERT: Node {} memory utilization {} exceeds threshold {}", 
                    node_id, node_metrics.memory_utilization, thresholds_guard.max_memory_utilization);
            }
        }
    }
    
    /// Get performance history
    pub async fn get_performance_history(&self) -> Vec<PerformanceSnapshot> {
        self.performance_history.read().await.clone()
    }
    
    /// Get latest performance snapshot
    pub async fn get_latest_snapshot(&self) -> Option<PerformanceSnapshot> {
        let history = self.performance_history.read().await;
        history.last().cloned()
    }
    
    /// Update alert thresholds
    pub async fn update_thresholds(&self, new_thresholds: AlertThresholds) {
        let mut thresholds = self.alert_thresholds.write().await;
        *thresholds = new_thresholds;
    }
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            monitoring_interval: Duration::from_millis(100), // 100ms
            history_retention: Duration::from_secs(3600), // 1 hour
            real_time_alerts: true,
            auto_optimization: false,
        }
    }
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            max_cpu_utilization: 0.8, // 80%
            max_memory_utilization: 2 * 1024 * 1024 * 1024, // 2GB (BPCI Enterprise allocation in 4GB system)
            max_latency_micros: 1000.0, // 1ms
            min_throughput: 1000.0, // 1K msgs/sec
            min_efficiency: 0.7, // 70%
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_score_calculation() {
        let score = VPodPerformanceMonitor::calculate_health_score(
            500_000.0, // 500K msgs/sec throughput
            100.0,     // 100μs latency
            0.8,       // 80% efficiency
        );
        
        assert!(score > 0.0 && score <= 1.0);
    }

    #[test]
    fn test_monitor_config_default() {
        let config = MonitorConfig::default();
        assert_eq!(config.monitoring_interval, Duration::from_millis(100));
        assert!(config.real_time_alerts);
    }

    #[test]
    fn test_alert_thresholds_default() {
        let thresholds = AlertThresholds::default();
        assert_eq!(thresholds.max_cpu_utilization, 0.8);
        assert_eq!(thresholds.max_latency_micros, 1000.0);
    }
}
