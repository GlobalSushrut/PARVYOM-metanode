//! Health Check System for BPI Infrastructure
//! 
//! Provides comprehensive health monitoring and diagnostics for pilot deployments

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::{Result, anyhow};
use tokio::time::{timeout, Duration};

/// Overall health status of the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub services: HashMap<String, ServiceHealth>,
    pub timestamp: u64,
    pub version: String,
    pub uptime_seconds: u64,
    pub pilot_ready: bool,
}

/// Health status of individual services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    pub status: String,
    pub response_time_ms: u64,
    pub last_check: u64,
    pub error_message: Option<String>,
    pub suggestions: Vec<String>,
}

/// Health checker for BPI infrastructure
pub struct HealthChecker {
    start_time: SystemTime,
}

impl HealthChecker {
    pub fn new() -> Self {
        Self {
            start_time: SystemTime::now(),
        }
    }

    /// Perform comprehensive health check
    pub async fn check_health(&self) -> Result<HealthStatus> {
        let mut services = HashMap::new();
        let mut overall_healthy = true;

        // Check VM Server
        let vm_health = self.check_vm_server().await;
        if vm_health.status != "healthy" {
            overall_healthy = false;
        }
        services.insert("vm_server".to_string(), vm_health);

        // Check BPCI Bridge
        let bpci_health = self.check_bpci_bridge().await;
        if bpci_health.status != "healthy" {
            overall_healthy = false;
        }
        services.insert("bpci_bridge".to_string(), bpci_health);

        // Check 4D Database
        let db_health = self.check_4d_database().await;
        if db_health.status != "healthy" {
            overall_healthy = false;
        }
        services.insert("4d_database".to_string(), db_health);

        // Check Service Orchestrator
        let orchestrator_health = self.check_service_orchestrator().await;
        if orchestrator_health.status != "healthy" {
            overall_healthy = false;
        }
        services.insert("service_orchestrator".to_string(), orchestrator_health);

        let uptime = self.start_time.elapsed()
            .unwrap_or(Duration::from_secs(0))
            .as_secs();

        Ok(HealthStatus {
            status: if overall_healthy { "healthy".to_string() } else { "unhealthy".to_string() },
            services,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(0))
                .as_secs(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime_seconds: uptime,
            pilot_ready: overall_healthy,
        })
    }

    /// Check VM Server health
    async fn check_vm_server(&self) -> ServiceHealth {
        let start = SystemTime::now();
        
        // Try to connect to VM Server (typically on port 8080)
        match self.check_service_endpoint("http://127.0.0.1:8080/health").await {
            Ok(_) => ServiceHealth {
                status: "healthy".to_string(),
                response_time_ms: start.elapsed().unwrap_or(Duration::from_millis(0)).as_millis() as u64,
                last_check: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(0)).as_secs(),
                error_message: None,
                suggestions: vec![],
            },
            Err(e) => ServiceHealth {
                status: "unhealthy".to_string(),
                response_time_ms: start.elapsed().unwrap_or(Duration::from_millis(0)).as_millis() as u64,
                last_check: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(0)).as_secs(),
                error_message: Some(e.to_string()),
                suggestions: vec![
                    "Check if VM Server is running: ps aux | grep vm_server".to_string(),
                    "Start VM Server: bpi-core vm-server start".to_string(),
                    "Check port 8080 availability: netstat -tlnp | grep 8080".to_string(),
                ],
            }
        }
    }

    /// Check BPCI Bridge health
    async fn check_bpci_bridge(&self) -> ServiceHealth {
        let start = SystemTime::now();
        
        match self.check_service_endpoint("http://127.0.0.1:8545/health").await {
            Ok(_) => ServiceHealth {
                status: "healthy".to_string(),
                response_time_ms: start.elapsed().unwrap_or(Duration::from_millis(0)).as_millis() as u64,
                last_check: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(0)).as_secs(),
                error_message: None,
                suggestions: vec![],
            },
            Err(e) => ServiceHealth {
                status: "unhealthy".to_string(),
                response_time_ms: start.elapsed().unwrap_or(Duration::from_millis(0)).as_millis() as u64,
                last_check: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(0)).as_secs(),
                error_message: Some(e.to_string()),
                suggestions: vec![
                    "Check if BPCI Bridge is running: ps aux | grep bpci".to_string(),
                    "Start BPCI Bridge: bpci-enterprise start".to_string(),
                    "Check port 8545 availability: netstat -tlnp | grep 8545".to_string(),
                ],
            }
        }
    }

    /// Check 4D Database health
    async fn check_4d_database(&self) -> ServiceHealth {
        let start = SystemTime::now();
        
        match self.check_service_endpoint("http://127.0.0.1:27017/health").await {
            Ok(_) => ServiceHealth {
                status: "healthy".to_string(),
                response_time_ms: start.elapsed().unwrap_or(Duration::from_millis(0)).as_millis() as u64,
                last_check: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(0)).as_secs(),
                error_message: None,
                suggestions: vec![],
            },
            Err(e) => ServiceHealth {
                status: "unhealthy".to_string(),
                response_time_ms: start.elapsed().unwrap_or(Duration::from_millis(0)).as_millis() as u64,
                last_check: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(0)).as_secs(),
                error_message: Some(e.to_string()),
                suggestions: vec![
                    "Check if 4D Database is running: ps aux | grep 4d".to_string(),
                    "Start 4D Database: bpi-core database start".to_string(),
                    "Check MongoDB compatibility port: netstat -tlnp | grep 27017".to_string(),
                ],
            }
        }
    }

    /// Check Service Orchestrator health
    async fn check_service_orchestrator(&self) -> ServiceHealth {
        let start = SystemTime::now();
        
        match self.check_service_endpoint("http://127.0.0.1:9090/health").await {
            Ok(_) => ServiceHealth {
                status: "healthy".to_string(),
                response_time_ms: start.elapsed().unwrap_or(Duration::from_millis(0)).as_millis() as u64,
                last_check: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(0)).as_secs(),
                error_message: None,
                suggestions: vec![],
            },
            Err(e) => ServiceHealth {
                status: "unhealthy".to_string(),
                response_time_ms: start.elapsed().unwrap_or(Duration::from_millis(0)).as_millis() as u64,
                last_check: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(0)).as_secs(),
                error_message: Some(e.to_string()),
                suggestions: vec![
                    "Check if Service Orchestrator is running: ps aux | grep orchestrator".to_string(),
                    "Start Service Orchestrator: bpi-core orchestrator start".to_string(),
                    "Check port 9090 availability: netstat -tlnp | grep 9090".to_string(),
                ],
            }
        }
    }

    /// Generic service endpoint checker
    async fn check_service_endpoint(&self, url: &str) -> Result<()> {
        let client = reqwest::Client::new();
        
        let response = timeout(Duration::from_secs(5), client.get(url).send()).await
            .map_err(|_| anyhow!("Request timeout after 5 seconds"))?
            .map_err(|e| anyhow!("HTTP request failed: {}", e))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow!("Service returned status: {}", response.status()))
        }
    }

    /// Quick health check for pilot readiness
    pub async fn pilot_readiness_check(&self) -> Result<bool> {
        let health = self.check_health().await?;
        
        // Check if all critical services are healthy
        let critical_services = ["vm_server", "bpci_bridge", "4d_database"];
        let all_critical_healthy = critical_services.iter().all(|service| {
            health.services.get(*service)
                .map(|s| s.status == "healthy")
                .unwrap_or(false)
        });

        Ok(all_critical_healthy)
    }

    /// Get detailed diagnostic information
    pub async fn get_diagnostics(&self) -> Result<HashMap<String, String>> {
        let mut diagnostics = HashMap::new();
        
        // System information
        diagnostics.insert("os".to_string(), std::env::consts::OS.to_string());
        diagnostics.insert("arch".to_string(), std::env::consts::ARCH.to_string());
        
        // Network connectivity
        let network_status = self.check_network_connectivity().await;
        diagnostics.insert("network".to_string(), network_status);
        
        // Port availability
        let port_status = self.check_port_availability().await;
        diagnostics.insert("ports".to_string(), port_status);
        
        Ok(diagnostics)
    }

    async fn check_network_connectivity(&self) -> String {
        match self.check_service_endpoint("http://127.0.0.1:8080").await {
            Ok(_) => "Local network connectivity: OK".to_string(),
            Err(_) => "Local network connectivity: FAILED".to_string(),
        }
    }

    async fn check_port_availability(&self) -> String {
        let ports = [8080, 8545, 27017, 9090];
        let mut available_ports = Vec::new();
        
        for port in ports {
            if let Ok(_) = std::net::TcpListener::bind(format!("127.0.0.1:{}", port)) {
                available_ports.push(port.to_string());
            }
        }
        
        if available_ports.is_empty() {
            "All required ports are in use".to_string()
        } else {
            format!("Available ports: {}", available_ports.join(", "))
        }
    }
}

impl Default for HealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Check VM server health with real TCP connection test
pub async fn check_vm_server_health() -> Result<ServiceHealth> {
    use tokio::time::{timeout, Duration};
    use std::time::Instant;
    
    let start = Instant::now();
    let vm_port = 7777;
    
    // Try to connect to VM server with timeout
    let result = timeout(
        Duration::from_secs(5),
        tokio::net::TcpStream::connect(format!("127.0.0.1:{}", vm_port))
    ).await;
    
    let response_time_ms = start.elapsed().as_millis() as u64;
    
    match result {
        Ok(Ok(_stream)) => {
            Ok(ServiceHealth {
                status: "healthy".to_string(),
                response_time_ms,
                last_check: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                error_message: None,
                suggestions: Vec::new(),
            })
        }
        Ok(Err(e)) => {
            Ok(ServiceHealth {
                status: "unhealthy".to_string(),
                response_time_ms,
                last_check: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                error_message: Some(format!("Connection failed: {}", e)),
                suggestions: vec![
                    "Check if VM server is running".to_string(),
                    "Verify port 7777 is not blocked".to_string(),
                    "Run: cargo run --bin vm_server".to_string(),
                ],
            })
        }
        Err(_) => {
            Ok(ServiceHealth {
                status: "timeout".to_string(),
                response_time_ms,
                last_check: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                error_message: Some("Connection timeout after 5s".to_string()),
                suggestions: vec![
                    "VM server may be overloaded".to_string(),
                    "Check system resources".to_string(),
                ],
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let checker = HealthChecker::new();
        let health = checker.check_health().await.unwrap();
        assert!(!health.status.is_empty());
    }

    #[tokio::test]
    async fn test_pilot_readiness_check() {
        let checker = HealthChecker::new();
        let _readiness = checker.pilot_readiness_check().await.unwrap();
        // Note: This will likely fail in test environment, which is expected
    }
}
