use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletMetrics {
    pub system: SystemMetrics,
    pub wallet: WalletStats,
    pub bpi_core: BpiCoreMetrics,
    pub performance: PerformanceMetrics,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_rx: u64,
    pub network_tx: u64,
    pub uptime_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletStats {
    pub total_transactions: u64,
    pub successful_transactions: u64,
    pub failed_transactions: u64,
    pub total_volume: f64,
    pub average_transaction_time: f64,
    pub last_transaction: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpiCoreMetrics {
    pub connected_components: u32,
    pub total_components: u32,
    pub healthy_components: u32,
    pub average_response_time: f64,
    pub total_requests: u64,
    pub failed_requests: u64,
    pub connection_uptime: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub api_response_time_ms: f64,
    pub ui_load_time_ms: f64,
    pub websocket_latency_ms: f64,
    pub component_start_time_ms: f64,
    pub memory_efficiency: f64,
}

impl WalletMetrics {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            system: SystemMetrics::collect().await?,
            wallet: WalletStats::default(),
            bpi_core: BpiCoreMetrics::default(),
            performance: PerformanceMetrics::default(),
            last_updated: chrono::Utc::now(),
        })
    }
    
    pub async fn update(&mut self) -> Result<()> {
        self.system = SystemMetrics::collect().await?;
        self.last_updated = chrono::Utc::now();
        Ok(())
    }
    
    pub fn get_health_score(&self) -> f64 {
        let mut score = 100.0;
        
        // Deduct points for high resource usage
        if self.system.cpu_usage > 80.0 {
            score -= 20.0;
        } else if self.system.cpu_usage > 60.0 {
            score -= 10.0;
        }
        
        if self.system.memory_usage > 90.0 {
            score -= 30.0;
        } else if self.system.memory_usage > 75.0 {
            score -= 15.0;
        }
        
        // Deduct points for component issues
        let component_health = if self.bpi_core.total_components > 0 {
            (self.bpi_core.healthy_components as f64 / self.bpi_core.total_components as f64) * 100.0
        } else {
            100.0
        };
        
        if component_health < 80.0 {
            score -= 25.0;
        } else if component_health < 95.0 {
            score -= 10.0;
        }
        
        // Deduct points for high error rates
        let error_rate = if self.bpi_core.total_requests > 0 {
            (self.bpi_core.failed_requests as f64 / self.bpi_core.total_requests as f64) * 100.0
        } else {
            0.0
        };
        
        if error_rate > 5.0 {
            score -= 20.0;
        } else if error_rate > 1.0 {
            score -= 10.0;
        }
        
        (score as f64).max(0.0).min(100.0)
    }
}

impl SystemMetrics {
    pub async fn collect() -> Result<Self> {
        // In a real implementation, this would collect actual system metrics
        // For now, we'll return mock data that looks realistic
        
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        Ok(Self {
            cpu_usage: rng.gen_range(10.0..50.0),
            memory_usage: rng.gen_range(30.0..70.0),
            disk_usage: rng.gen_range(20.0..60.0),
            network_rx: rng.gen_range(1000..10000),
            network_tx: rng.gen_range(500..5000),
            uptime_seconds: rng.gen_range(3600..86400),
        })
    }
}

impl Default for WalletStats {
    fn default() -> Self {
        Self {
            total_transactions: 0,
            successful_transactions: 0,
            failed_transactions: 0,
            total_volume: 0.0,
            average_transaction_time: 0.0,
            last_transaction: None,
        }
    }
}

impl Default for BpiCoreMetrics {
    fn default() -> Self {
        Self {
            connected_components: 0,
            total_components: 28,
            healthy_components: 0,
            average_response_time: 0.0,
            total_requests: 0,
            failed_requests: 0,
            connection_uptime: 0.0,
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            api_response_time_ms: 0.0,
            ui_load_time_ms: 0.0,
            websocket_latency_ms: 0.0,
            component_start_time_ms: 0.0,
            memory_efficiency: 100.0,
        }
    }
}
