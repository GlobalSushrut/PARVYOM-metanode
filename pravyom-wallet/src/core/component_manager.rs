use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

use super::{WalletConfig, BpiIntegration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentInfo {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub category: ComponentCategory,
    pub status: ComponentStatus,
    pub port: Option<u16>,
    pub health_check_url: Option<String>,
    pub dependencies: Vec<String>,
    pub auto_restart: bool,
    pub last_status_check: Option<chrono::DateTime<chrono::Utc>>,
    pub uptime: Option<std::time::Duration>,
    pub metrics: ComponentMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComponentCategory {
    Core,
    Security,
    Storage,
    Network,
    Consensus,
    VM,
    API,
    Monitoring,
    Integration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComponentStatus {
    Running,
    Stopped,
    Starting,
    Stopping,
    Error(String),
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub requests_per_second: f64,
    pub error_rate: f64,
    pub response_time_ms: f64,
}

impl Default for ComponentMetrics {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0.0,
            requests_per_second: 0.0,
            error_rate: 0.0,
            response_time_ms: 0.0,
        }
    }
}

pub struct ComponentManager {
    config: WalletConfig,
    bpi_integration: Arc<BpiIntegration>,
    components: RwLock<HashMap<String, ComponentInfo>>,
}

impl ComponentManager {
    pub async fn new(config: &WalletConfig, bpi_integration: Arc<BpiIntegration>) -> Result<Self> {
        let mut components = HashMap::new();
        
        // Initialize all 28+ BPI Core components
        let component_definitions = Self::get_component_definitions();
        
        for (name, info) in component_definitions {
            components.insert(name, info);
        }
        
        Ok(Self {
            config: config.clone(),
            bpi_integration,
            components: RwLock::new(components),
        })
    }
    
    fn get_component_definitions() -> HashMap<String, ComponentInfo> {
        let mut components = HashMap::new();
        
        // Core Infrastructure Components
        components.insert("blockchain_core".to_string(), ComponentInfo {
            name: "blockchain_core".to_string(),
            display_name: "Blockchain Core".to_string(),
            description: "Core blockchain ledger and consensus engine".to_string(),
            category: ComponentCategory::Core,
            status: ComponentStatus::Stopped,
            port: Some(9001),
            health_check_url: Some("/api/blockchain/status".to_string()),
            dependencies: vec![],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        components.insert("oracle_node".to_string(), ComponentInfo {
            name: "oracle_node".to_string(),
            display_name: "Oracle Node".to_string(),
            description: "External data integration and price feeds".to_string(),
            category: ComponentCategory::Core,
            status: ComponentStatus::Stopped,
            port: Some(9002),
            health_check_url: Some("/api/oracle/status".to_string()),
            dependencies: vec!["blockchain_core".to_string()],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        components.insert("registry_service".to_string(), ComponentInfo {
            name: "registry_service".to_string(),
            display_name: "Registry Service".to_string(),
            description: "Service discovery and node registration".to_string(),
            category: ComponentCategory::Core,
            status: ComponentStatus::Stopped,
            port: Some(9003),
            health_check_url: Some("/api/registry/status".to_string()),
            dependencies: vec!["blockchain_core".to_string()],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        components.insert("storage_service".to_string(), ComponentInfo {
            name: "storage_service".to_string(),
            display_name: "Distributed Storage".to_string(),
            description: "Distributed data storage and retrieval".to_string(),
            category: ComponentCategory::Storage,
            status: ComponentStatus::Stopped,
            port: Some(9005),
            health_check_url: Some("/api/storage/status".to_string()),
            dependencies: vec!["blockchain_core".to_string()],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        components.insert("proof_verification".to_string(), ComponentInfo {
            name: "proof_verification".to_string(),
            display_name: "Proof Verification".to_string(),
            description: "Cryptographic proof validation service".to_string(),
            category: ComponentCategory::Security,
            status: ComponentStatus::Stopped,
            port: Some(9006),
            health_check_url: Some("/api/proofs/status".to_string()),
            dependencies: vec!["blockchain_core".to_string()],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        components.insert("economic_coordination".to_string(), ComponentInfo {
            name: "economic_coordination".to_string(),
            display_name: "Economic Coordination".to_string(),
            description: "Token economics and incentive management".to_string(),
            category: ComponentCategory::Core,
            status: ComponentStatus::Stopped,
            port: Some(9007),
            health_check_url: Some("/api/economics/status".to_string()),
            dependencies: vec!["blockchain_core".to_string()],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        // Security Framework Components
        components.insert("forensic_firewall".to_string(), ComponentInfo {
            name: "forensic_firewall".to_string(),
            display_name: "Forensic Firewall".to_string(),
            description: "Advanced security monitoring and threat detection".to_string(),
            category: ComponentCategory::Security,
            status: ComponentStatus::Stopped,
            port: None,
            health_check_url: Some("/api/security/firewall/status".to_string()),
            dependencies: vec!["blockchain_core".to_string()],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        components.insert("quantum_crypto".to_string(), ComponentInfo {
            name: "quantum_crypto".to_string(),
            display_name: "Quantum Cryptography".to_string(),
            description: "Post-quantum cryptographic operations".to_string(),
            category: ComponentCategory::Security,
            status: ComponentStatus::Stopped,
            port: None,
            health_check_url: Some("/api/security/quantum/status".to_string()),
            dependencies: vec![],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        components.insert("immutable_audit".to_string(), ComponentInfo {
            name: "immutable_audit".to_string(),
            display_name: "Immutable Audit System".to_string(),
            description: "Cryptographically secured audit trails".to_string(),
            category: ComponentCategory::Security,
            status: ComponentStatus::Stopped,
            port: None,
            health_check_url: Some("/api/audit/status".to_string()),
            dependencies: vec!["blockchain_core".to_string()],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        // Network and Communication Components
        components.insert("xtmp_protocol".to_string(), ComponentInfo {
            name: "xtmp_protocol".to_string(),
            display_name: "XTMP Protocol".to_string(),
            description: "High-performance messaging protocol (10-20x faster than HTTP)".to_string(),
            category: ComponentCategory::Network,
            status: ComponentStatus::Stopped,
            port: None,
            health_check_url: Some("/api/xtmp/status".to_string()),
            dependencies: vec![],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        components.insert("http_cage".to_string(), ComponentInfo {
            name: "http_cage".to_string(),
            display_name: "HTTP Cage".to_string(),
            description: "Security-enhanced HTTP processing and protocol transformation".to_string(),
            category: ComponentCategory::Network,
            status: ComponentStatus::Stopped,
            port: None,
            health_check_url: Some("/api/httpcage/status".to_string()),
            dependencies: vec!["forensic_firewall".to_string()],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        components.insert("shadow_registry".to_string(), ComponentInfo {
            name: "shadow_registry".to_string(),
            display_name: "Shadow Registry".to_string(),
            description: "Web2-Web3 bridge with privacy-preserving operations".to_string(),
            category: ComponentCategory::Integration,
            status: ComponentStatus::Stopped,
            port: None,
            health_check_url: Some("/api/shadow/status".to_string()),
            dependencies: vec!["blockchain_core".to_string()],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        // VM and Execution Components
        components.insert("bpi_action_vm".to_string(), ComponentInfo {
            name: "bpi_action_vm".to_string(),
            display_name: "BPI Action VM".to_string(),
            description: "Virtual machine for BPI smart contract execution".to_string(),
            category: ComponentCategory::VM,
            status: ComponentStatus::Stopped,
            port: None,
            health_check_url: Some("/api/vm/action/status".to_string()),
            dependencies: vec!["blockchain_core".to_string()],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        components.insert("court_node".to_string(), ComponentInfo {
            name: "court_node".to_string(),
            display_name: "Court Node".to_string(),
            description: "Governance and dispute resolution system".to_string(),
            category: ComponentCategory::VM,
            status: ComponentStatus::Stopped,
            port: None,
            health_check_url: Some("/api/court/status".to_string()),
            dependencies: vec!["blockchain_core".to_string()],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        components.insert("orchestration_vm".to_string(), ComponentInfo {
            name: "orchestration_vm".to_string(),
            display_name: "Orchestration VM".to_string(),
            description: "Infrastructure deployment and management VM".to_string(),
            category: ComponentCategory::VM,
            status: ComponentStatus::Stopped,
            port: None,
            health_check_url: Some("/api/vm/orchestration/status".to_string()),
            dependencies: vec!["blockchain_core".to_string()],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        // Storage and Database Components
        components.insert("distributed_storage".to_string(), ComponentInfo {
            name: "distributed_storage".to_string(),
            display_name: "Distributed Storage Engine".to_string(),
            description: "High-performance distributed data storage".to_string(),
            category: ComponentCategory::Storage,
            status: ComponentStatus::Stopped,
            port: None,
            health_check_url: Some("/api/storage/distributed/status".to_string()),
            dependencies: vec!["blockchain_core".to_string()],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        components.insert("enhanced_cdn".to_string(), ComponentInfo {
            name: "enhanced_cdn".to_string(),
            display_name: "Enhanced CDN Storage".to_string(),
            description: "Content delivery network with advanced caching".to_string(),
            category: ComponentCategory::Storage,
            status: ComponentStatus::Stopped,
            port: None,
            health_check_url: Some("/api/cdn/status".to_string()),
            dependencies: vec!["distributed_storage".to_string()],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        // Consensus and Coordination Components
        components.insert("node_coordinator".to_string(), ComponentInfo {
            name: "node_coordinator".to_string(),
            display_name: "Node Coordinator".to_string(),
            description: "Multi-node coordination and consensus management".to_string(),
            category: ComponentCategory::Consensus,
            status: ComponentStatus::Stopped,
            port: None,
            health_check_url: Some("/api/coordinator/status".to_string()),
            dependencies: vec!["blockchain_core".to_string()],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        components.insert("bpci_xtmp_server".to_string(), ComponentInfo {
            name: "bpci_xtmp_server".to_string(),
            display_name: "BPCI XTMP Server".to_string(),
            description: "High-performance BPI ↔ BPCI communication server".to_string(),
            category: ComponentCategory::Network,
            status: ComponentStatus::Stopped,
            port: None,
            health_check_url: Some("/api/bpci/xtmp/status".to_string()),
            dependencies: vec!["xtmp_protocol".to_string()],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        // Integration and API Components
        components.insert("cue_orchestration".to_string(), ComponentInfo {
            name: "cue_orchestration".to_string(),
            display_name: "CUE Orchestration Engine".to_string(),
            description: "Configuration and deployment orchestration using CUE".to_string(),
            category: ComponentCategory::Integration,
            status: ComponentStatus::Stopped,
            port: None,
            health_check_url: Some("/api/cue/status".to_string()),
            dependencies: vec!["blockchain_core".to_string()],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        components.insert("biso_agreement".to_string(), ComponentInfo {
            name: "biso_agreement".to_string(),
            display_name: "BISO Agreement System".to_string(),
            description: "Business Integration Service Orchestration agreements".to_string(),
            category: ComponentCategory::Integration,
            status: ComponentStatus::Stopped,
            port: None,
            health_check_url: Some("/api/biso/status".to_string()),
            dependencies: vec!["blockchain_core".to_string()],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        // Client and Communication Components
        components.insert("httpcg_client".to_string(), ComponentInfo {
            name: "httpcg_client".to_string(),
            display_name: "HTTPCG Client".to_string(),
            description: "HTTP Cage protocol client for secure communication".to_string(),
            category: ComponentCategory::Network,
            status: ComponentStatus::Stopped,
            port: None,
            health_check_url: Some("/api/httpcg/client/status".to_string()),
            dependencies: vec!["http_cage".to_string()],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        components.insert("qlock_client".to_string(), ComponentInfo {
            name: "qlock_client".to_string(),
            display_name: "QLOCK Client".to_string(),
            description: "Quantum lock client for advanced security".to_string(),
            category: ComponentCategory::Security,
            status: ComponentStatus::Stopped,
            port: None,
            health_check_url: Some("/api/qlock/client/status".to_string()),
            dependencies: vec!["quantum_crypto".to_string()],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        components.insert("tlsls_client".to_string(), ComponentInfo {
            name: "tlsls_client".to_string(),
            display_name: "TLSLS Client".to_string(),
            description: "Transport Layer Security Lock System client".to_string(),
            category: ComponentCategory::Security,
            status: ComponentStatus::Stopped,
            port: None,
            health_check_url: Some("/api/tlsls/client/status".to_string()),
            dependencies: vec!["quantum_crypto".to_string()],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        // Additional Advanced Components
        components.insert("control_fedrate_network".to_string(), ComponentInfo {
            name: "control_fedrate_network".to_string(),
            display_name: "Control FedRate Network".to_string(),
            description: "Federal rate control and economic policy management".to_string(),
            category: ComponentCategory::Core,
            status: ComponentStatus::Stopped,
            port: None,
            health_check_url: Some("/api/fedrate/status".to_string()),
            dependencies: vec!["economic_coordination".to_string()],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        components.insert("universal_audit_vm".to_string(), ComponentInfo {
            name: "universal_audit_vm".to_string(),
            display_name: "Universal Audit VM".to_string(),
            description: "Comprehensive audit and compliance virtual machine".to_string(),
            category: ComponentCategory::VM,
            status: ComponentStatus::Stopped,
            port: None,
            health_check_url: Some("/api/vm/audit/status".to_string()),
            dependencies: vec!["immutable_audit".to_string()],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        components.insert("vm_server".to_string(), ComponentInfo {
            name: "vm_server".to_string(),
            display_name: "VM Server".to_string(),
            description: "Central virtual machine server for all VM operations".to_string(),
            category: ComponentCategory::VM,
            status: ComponentStatus::Stopped,
            port: None,
            health_check_url: Some("/api/vm/server/status".to_string()),
            dependencies: vec!["blockchain_core".to_string()],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        components.insert("stamped_bpi_communication".to_string(), ComponentInfo {
            name: "stamped_bpi_communication".to_string(),
            display_name: "Stamped BPI Communication".to_string(),
            description: "Authenticated and stamped BPI communication layer".to_string(),
            category: ComponentCategory::Network,
            status: ComponentStatus::Stopped,
            port: None,
            health_check_url: Some("/api/stamped/communication/status".to_string()),
            dependencies: vec!["blockchain_core".to_string()],
            auto_restart: true,
            last_status_check: None,
            uptime: None,
            metrics: ComponentMetrics::default(),
        });
        
        components
    }
    
    pub async fn get_all_components(&self) -> HashMap<String, ComponentInfo> {
        self.components.read().await.clone()
    }
    
    pub async fn get_component(&self, name: &str) -> Option<ComponentInfo> {
        self.components.read().await.get(name).cloned()
    }
    
    pub async fn start_component(&self, name: &str) -> Result<()> {
        info!("🚀 Starting component: {}", name);
        
        // Update component status to starting
        {
            let mut components = self.components.write().await;
            if let Some(component) = components.get_mut(name) {
                component.status = ComponentStatus::Starting;
                component.last_status_check = Some(chrono::Utc::now());
            }
        }
        
        // Start component through BPI integration
        self.bpi_integration.start_component(name).await?;
        
        // Update component status to running
        {
            let mut components = self.components.write().await;
            if let Some(component) = components.get_mut(name) {
                component.status = ComponentStatus::Running;
                component.last_status_check = Some(chrono::Utc::now());
            }
        }
        
        Ok(())
    }
    
    pub async fn stop_component(&self, name: &str) -> Result<()> {
        info!("⏹️ Stopping component: {}", name);
        
        // Update component status to stopping
        {
            let mut components = self.components.write().await;
            if let Some(component) = components.get_mut(name) {
                component.status = ComponentStatus::Stopping;
                component.last_status_check = Some(chrono::Utc::now());
            }
        }
        
        // Stop component through BPI integration
        self.bpi_integration.stop_component(name).await?;
        
        // Update component status to stopped
        {
            let mut components = self.components.write().await;
            if let Some(component) = components.get_mut(name) {
                component.status = ComponentStatus::Stopped;
                component.last_status_check = Some(chrono::Utc::now());
            }
        }
        
        Ok(())
    }
    
    pub async fn refresh_component_status(&self, name: &str) -> Result<()> {
        if let Ok(status_data) = self.bpi_integration.get_component_status(name).await {
            let mut components = self.components.write().await;
            if let Some(component) = components.get_mut(name) {
                // Update status based on response
                component.status = if status_data.get("running").and_then(|v| v.as_bool()).unwrap_or(false) {
                    ComponentStatus::Running
                } else {
                    ComponentStatus::Stopped
                };
                
                component.last_status_check = Some(chrono::Utc::now());
                
                // Update metrics if available
                if let Some(metrics) = status_data.get("metrics") {
                    if let Some(cpu) = metrics.get("cpu_usage").and_then(|v| v.as_f64()) {
                        component.metrics.cpu_usage = cpu;
                    }
                    if let Some(memory) = metrics.get("memory_usage").and_then(|v| v.as_f64()) {
                        component.metrics.memory_usage = memory;
                    }
                    if let Some(rps) = metrics.get("requests_per_second").and_then(|v| v.as_f64()) {
                        component.metrics.requests_per_second = rps;
                    }
                }
            }
        }
        
        Ok(())
    }
    
    pub async fn refresh_all_components(&self) -> Result<()> {
        let component_names: Vec<String> = {
            self.components.read().await.keys().cloned().collect()
        };
        
        for name in component_names {
            if let Err(e) = self.refresh_component_status(&name).await {
                warn!("Failed to refresh status for component {}: {}", name, e);
            }
        }
        
        Ok(())
    }
    
    pub async fn get_components_by_category(&self, _category: ComponentCategory) -> Vec<ComponentInfo> {
        self.components
            .read()
            .await
            .values()
            .filter(|c| matches!(&c.category, _category))
            .cloned()
            .collect()
    }
    
    pub async fn get_running_components(&self) -> Vec<ComponentInfo> {
        self.components
            .read()
            .await
            .values()
            .filter(|c| matches!(c.status, ComponentStatus::Running))
            .cloned()
            .collect()
    }
}
