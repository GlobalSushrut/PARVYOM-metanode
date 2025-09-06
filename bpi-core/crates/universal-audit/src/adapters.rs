//! Runtime Adapters - Integration with BPI runtime environments

use crate::{RuntimeAuditNode, AuditLevel, OperationType, ComplianceTag, ProofChain, RuntimeType, RuntimeAdapter};
use crate::runtime_node::{
    ExecutionContext, RuntimeAddress, LocationType, NetworkAddress, ProcessContext,
    CageConfig, WorkloadSpec, RequestContext, IoTClass, ExportMetadata
};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use reqwest::Client;

/// Adapter Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterConfig {
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub verbose_logging: bool,
    pub custom_headers: HashMap<String, String>,
    pub auth_token: Option<String>,
}

impl Default for AdapterConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            max_retries: 3,
            verbose_logging: false,
            custom_headers: HashMap::new(),
            auth_token: None,
        }
    }
}

/// DockLock Container Adapter
#[derive(Debug)]
pub struct DockLockAdapter {
    api_endpoint: String,
    client: Client,
    config: AdapterConfig,
}

impl DockLockAdapter {
    pub fn new(api_endpoint: String, config: AdapterConfig) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .expect("Failed to create HTTP client");
        
        Self { api_endpoint, client, config }
    }
}

#[async_trait::async_trait]
impl RuntimeAdapter for DockLockAdapter {
    fn runtime_type(&self) -> RuntimeType {
        RuntimeType::DockLock
    }
    
    async fn capture_runtime_state(&self) -> Result<Vec<RuntimeAuditNode>> {
        let url = format!("{}/api/containers", self.api_endpoint);
        
        let mut request = self.client.get(&url);
        if let Some(token) = &self.config.auth_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        
        match request.send().await {
            Ok(response) if response.status().is_success() => {
                let containers_text = response.text().await?;
                
                let audit_node = RuntimeAuditNode {
                    node_id: [0u8; 32],
                    parent_id: None,
                    children: vec![],
                    timestamp_ns: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
                    duration_ns: None,
                    sequence_number: 0,
                    execution_context: ExecutionContext::DockLock {
                        container_id: [0u8; 32],
                        workload_id: [0u8; 32],
                        cage_config: CageConfig::default(),
                    },
                    runtime_address: RuntimeAddress::default(),
                    operation_type: OperationType::ContainerExec { command: "status".to_string(), working_dir: "/".to_string() },
                    operation_data: crate::runtime_node::OperationData {
                        data: containers_text.clone().into_bytes(),
                        data_hash: [0u8; 32],
                        size_bytes: containers_text.len(),
                        compression: Some(crate::runtime_node::CompressionType::None),
                        encryption: Some(crate::runtime_node::EncryptionType::None),
                    },
                    binary_outputs: vec![],
                    proof_chain: Default::default(),
                    integrity_hash: [0u8; 32],
                    signature: vec![0u8; 64],
                    audit_level: AuditLevel::Standard,
                    compliance_tags: vec![ComplianceTag::ContainerSecurity, ComplianceTag::ProcessMonitoring],
                    export_metadata: ExportMetadata::default(),
                };
                
                Ok(vec![audit_node])
            }
            _ => {
                let error_audit = RuntimeAuditNode {
                    node_id: [1u8; 32],
                    parent_id: None,
                    children: vec![],
                    timestamp_ns: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
                    duration_ns: None,
                    sequence_number: 0,
                    execution_context: ExecutionContext::DockLock {
                        container_id: [1u8; 32],
                        workload_id: [1u8; 32],
                        cage_config: CageConfig::default(),
                    },
                    runtime_address: RuntimeAddress::default(),
                    operation_type: OperationType::ProcessExit { exit_code: -1, signal: None },
                    operation_data: crate::runtime_node::OperationData {
                        data: b"Connection failed".to_vec(),
                        data_hash: [0u8; 32],
                        size_bytes: 17,
                        compression: Some(crate::runtime_node::CompressionType::None),
                        encryption: Some(crate::runtime_node::EncryptionType::None),
                    },
                    binary_outputs: vec![],
                    proof_chain: Default::default(),
                    integrity_hash: [0u8; 32],
                    signature: vec![0u8; 64],
                    audit_level: AuditLevel::Critical,
                    compliance_tags: vec![ComplianceTag::ErrorReporting],
                    export_metadata: ExportMetadata::default(),
                };
                
                Ok(vec![error_audit])
            }
        }
    }
    
    async fn get_runtime_address(&self) -> Result<String> {
        Ok(self.api_endpoint.clone())
    }
    
    async fn is_healthy(&self) -> bool {
        let url = format!("{}/health", self.api_endpoint);
        match self.client.get(&url).send().await {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }
    
    async fn get_metadata(&self) -> Result<HashMap<String, String>> {
        let mut metadata = HashMap::new();
        metadata.insert("adapter_type".to_string(), "DockLock".to_string());
        metadata.insert("api_endpoint".to_string(), self.api_endpoint.clone());
        Ok(metadata)
    }
}

/// ENC Cluster Adapter
#[derive(Debug)]
pub struct EncClusterAdapter {
    api_endpoint: String,
    client: Client,
    config: AdapterConfig,
}

impl EncClusterAdapter {
    pub fn new(api_endpoint: String, config: AdapterConfig) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .expect("Failed to create HTTP client");
        
        Self { api_endpoint, client, config }
    }
}

#[async_trait::async_trait]
impl RuntimeAdapter for EncClusterAdapter {
    fn runtime_type(&self) -> RuntimeType {
        RuntimeType::EncCluster
    }
    
    async fn capture_runtime_state(&self) -> Result<Vec<RuntimeAuditNode>> {
        let url = format!("{}/api/cluster/status", self.api_endpoint);
        
        match self.client.get(&url).send().await {
            Ok(response) if response.status().is_success() => {
                let status_text = response.text().await?;
                
                let audit_node = RuntimeAuditNode {
                    node_id: [0u8; 32],
                    parent_id: None,
                    children: vec![],
                    timestamp_ns: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
                    duration_ns: None,
                    sequence_number: 0,
                    execution_context: ExecutionContext::EncCluster {
                        cluster_id: [0u8; 32],
                        node_id: [0u8; 32],
                        workload_spec: WorkloadSpec::default(),
                    },
                    runtime_address: RuntimeAddress::default(),
                    operation_type: OperationType::WorkloadSchedule { workload_id: [0u8; 32], node_assignment: [0u8; 32] },
                    operation_data: crate::runtime_node::OperationData {
                        data: status_text.clone().into_bytes(),
                        data_hash: [0u8; 32],
                        size_bytes: status_text.len(),
                        compression: Some(crate::runtime_node::CompressionType::None),
                        encryption: Some(crate::runtime_node::EncryptionType::None),
                    },
                    binary_outputs: vec![],
                    proof_chain: Default::default(),
                    integrity_hash: [0u8; 32],
                    signature: vec![0u8; 64],
                    audit_level: AuditLevel::Standard,
                    compliance_tags: vec![ComplianceTag::ClusterMonitoring, ComplianceTag::NetworkSecurity],
                    export_metadata: ExportMetadata::default(),
                };
                
                Ok(vec![audit_node])
            }
            _ => {
                let error_audit = RuntimeAuditNode {
                    node_id: [1u8; 32],
                    parent_id: None,
                    children: vec![],
                    timestamp_ns: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
                    duration_ns: None,
                    sequence_number: 0,
                    execution_context: ExecutionContext::EncCluster {
                        cluster_id: [1u8; 32],
                        node_id: [1u8; 32],
                        workload_spec: WorkloadSpec::default(),
                    },
                    runtime_address: RuntimeAddress::default(),
                    operation_type: OperationType::ProcessExit { exit_code: -1, signal: None },
                    operation_data: crate::runtime_node::OperationData {
                        data: b"Connection failed".to_vec(),
                        data_hash: [0u8; 32],
                        size_bytes: 17,
                        compression: Some(crate::runtime_node::CompressionType::None),
                        encryption: Some(crate::runtime_node::EncryptionType::None),
                    },
                    binary_outputs: vec![],
                    proof_chain: Default::default(),
                    integrity_hash: [0u8; 32],
                    signature: vec![0u8; 64],
                    audit_level: AuditLevel::Critical,
                    compliance_tags: vec![ComplianceTag::ErrorReporting],
                    export_metadata: ExportMetadata::default(),
                };
                
                Ok(vec![error_audit])
            }
        }
    }
    
    async fn get_runtime_address(&self) -> Result<String> {
        Ok(self.api_endpoint.clone())
    }
    
    async fn is_healthy(&self) -> bool {
        let url = format!("{}/health", self.api_endpoint);
        match self.client.get(&url).send().await {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }
    
    async fn get_metadata(&self) -> Result<HashMap<String, String>> {
        let mut metadata = HashMap::new();
        metadata.insert("adapter_type".to_string(), "EncCluster".to_string());
        metadata.insert("api_endpoint".to_string(), self.api_endpoint.clone());
        Ok(metadata)
    }
}

/// HTTP Cage Adapter
#[derive(Debug)]
pub struct HttpCageAdapter {
    api_endpoint: String,
    client: Client,
    config: AdapterConfig,
}

impl HttpCageAdapter {
    pub fn new(api_endpoint: String, config: AdapterConfig) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .expect("Failed to create HTTP client");
        
        Self { api_endpoint, client, config }
    }
}

#[async_trait::async_trait]
impl RuntimeAdapter for HttpCageAdapter {
    fn runtime_type(&self) -> RuntimeType {
        RuntimeType::HttpCage
    }
    
    async fn capture_runtime_state(&self) -> Result<Vec<RuntimeAuditNode>> {
        let url = format!("{}/api/metrics", self.api_endpoint);
        
        match self.client.get(&url).send().await {
            Ok(response) if response.status().is_success() => {
                let metrics_text = response.text().await?;
                
                let audit_node = RuntimeAuditNode {
                    node_id: [0u8; 32],
                    parent_id: None,
                    children: vec![],
                    timestamp_ns: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
                    duration_ns: None,
                    sequence_number: 0,
                    execution_context: ExecutionContext::HttpCage {
                        cage_id: [0u8; 32],
                        client_session: [0u8; 32],
                        request_context: RequestContext::default(),
                    },
                    runtime_address: RuntimeAddress::default(),
                    operation_type: OperationType::HttpRequest { method: "GET".to_string(), path: "/metrics".to_string(), body_hash: [0u8; 32] },
                    operation_data: crate::runtime_node::OperationData {
                        data: metrics_text.clone().into_bytes(),
                        data_hash: [0u8; 32],
                        size_bytes: metrics_text.len(),
                        compression: Some(crate::runtime_node::CompressionType::None),
                        encryption: Some(crate::runtime_node::EncryptionType::None),
                    },
                    binary_outputs: vec![],
                    proof_chain: Default::default(),
                    integrity_hash: [0u8; 32],
                    signature: vec![0u8; 64],
                    audit_level: AuditLevel::Standard,
                    compliance_tags: vec![ComplianceTag::HttpSecurity, ComplianceTag::NetworkMonitoring],
                    export_metadata: ExportMetadata::default(),
                };
                
                Ok(vec![audit_node])
            }
            _ => {
                let error_audit = RuntimeAuditNode {
                    node_id: [1u8; 32],
                    parent_id: None,
                    children: vec![],
                    timestamp_ns: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
                    duration_ns: None,
                    sequence_number: 0,
                    execution_context: ExecutionContext::HttpCage {
                        cage_id: [1u8; 32],
                        client_session: [1u8; 32],
                        request_context: RequestContext::default(),
                    },
                    runtime_address: RuntimeAddress::default(),
                    operation_type: OperationType::ProcessExit { exit_code: -1, signal: None },
                    operation_data: crate::runtime_node::OperationData {
                        data: b"Connection failed".to_vec(),
                        data_hash: [0u8; 32],
                        size_bytes: 17,
                        compression: Some(crate::runtime_node::CompressionType::None),
                        encryption: Some(crate::runtime_node::EncryptionType::None),
                    },
                    binary_outputs: vec![],
                    proof_chain: Default::default(),
                    integrity_hash: [0u8; 32],
                    signature: vec![0u8; 64],
                    audit_level: AuditLevel::Critical,
                    compliance_tags: vec![ComplianceTag::ErrorReporting],
                    export_metadata: ExportMetadata::default(),
                };
                
                Ok(vec![error_audit])
            }
        }
    }
    
    async fn get_runtime_address(&self) -> Result<String> {
        Ok(self.api_endpoint.clone())
    }
    
    async fn is_healthy(&self) -> bool {
        let url = format!("{}/health", self.api_endpoint);
        match self.client.get(&url).send().await {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }
    
    async fn get_metadata(&self) -> Result<HashMap<String, String>> {
        let mut metadata = HashMap::new();
        metadata.insert("adapter_type".to_string(), "HttpCage".to_string());
        metadata.insert("api_endpoint".to_string(), self.api_endpoint.clone());
        Ok(metadata)
    }
}

/// IoT Gateway Adapter
#[derive(Debug)]
pub struct IoTGatewayAdapter {
    api_endpoint: String,
    client: Client,
    config: AdapterConfig,
}

impl IoTGatewayAdapter {
    pub fn new(api_endpoint: String, config: AdapterConfig) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .expect("Failed to create HTTP client");
        
        Self { api_endpoint, client, config }
    }
}

#[async_trait::async_trait]
impl RuntimeAdapter for IoTGatewayAdapter {
    fn runtime_type(&self) -> RuntimeType {
        RuntimeType::IoTGateway
    }
    
    async fn capture_runtime_state(&self) -> Result<Vec<RuntimeAuditNode>> {
        let url = format!("{}/api/devices", self.api_endpoint);
        
        match self.client.get(&url).send().await {
            Ok(response) if response.status().is_success() => {
                let devices_text = response.text().await?;
                
                let audit_node = RuntimeAuditNode {
                    node_id: [0u8; 32],
                    parent_id: None,
                    children: vec![],
                    timestamp_ns: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
                    duration_ns: None,
                    sequence_number: 0,
                    execution_context: ExecutionContext::IoTGateway {
                        gateway_id: [0u8; 32],
                        device_id: [0u8; 32],
                        device_class: IoTClass::Sensor,
                    },
                    runtime_address: RuntimeAddress::default(),
                    operation_type: OperationType::SensorReading { sensor_type: "gateway".to_string(), value: devices_text.clone().into_bytes() },
                    operation_data: crate::runtime_node::OperationData {
                        data: devices_text.clone().into_bytes(),
                        data_hash: [0u8; 32],
                        size_bytes: devices_text.len(),
                        compression: Some(crate::runtime_node::CompressionType::None),
                        encryption: Some(crate::runtime_node::EncryptionType::None),
                    },
                    binary_outputs: vec![],
                    proof_chain: Default::default(),
                    integrity_hash: [0u8; 32],
                    signature: vec![0u8; 64],
                    audit_level: AuditLevel::Standard,
                    compliance_tags: vec![ComplianceTag::IoTSecurity, ComplianceTag::DeviceMonitoring],
                    export_metadata: ExportMetadata::default(),
                };
                
                Ok(vec![audit_node])
            }
            _ => {
                let error_audit = RuntimeAuditNode {
                    node_id: [1u8; 32],
                    parent_id: None,
                    children: vec![],
                    timestamp_ns: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
                    duration_ns: None,
                    sequence_number: 0,
                    execution_context: ExecutionContext::IoTGateway {
                        gateway_id: [1u8; 32],
                        device_id: [1u8; 32],
                        device_class: IoTClass::Actuator,
                    },
                    runtime_address: RuntimeAddress::default(),
                    operation_type: OperationType::ProcessExit { exit_code: -1, signal: None },
                    operation_data: crate::runtime_node::OperationData {
                        data: b"Connection failed".to_vec(),
                        data_hash: [0u8; 32],
                        size_bytes: 17,
                        compression: Some(crate::runtime_node::CompressionType::None),
                        encryption: Some(crate::runtime_node::EncryptionType::None),
                    },
                    binary_outputs: vec![],
                    proof_chain: Default::default(),
                    integrity_hash: [0u8; 32],
                    signature: vec![0u8; 64],
                    audit_level: AuditLevel::Critical,
                    compliance_tags: vec![ComplianceTag::ErrorReporting],
                    export_metadata: ExportMetadata::default(),
                };
                
                Ok(vec![error_audit])
            }
        }
    }
    
    async fn get_runtime_address(&self) -> Result<String> {
        Ok(self.api_endpoint.clone())
    }
    
    async fn is_healthy(&self) -> bool {
        let url = format!("{}/health", self.api_endpoint);
        match self.client.get(&url).send().await {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }
    
    async fn get_metadata(&self) -> Result<HashMap<String, String>> {
        let mut metadata = HashMap::new();
        metadata.insert("adapter_type".to_string(), "IoTGateway".to_string());
        metadata.insert("api_endpoint".to_string(), self.api_endpoint.clone());
        Ok(metadata)
    }
}

/// Adapter Factory for creating runtime adapters
pub struct AdapterFactory;

impl AdapterFactory {
    pub fn create_adapter(
        runtime_type: RuntimeType,
        api_endpoint: String,
        config: AdapterConfig,
    ) -> Box<dyn RuntimeAdapter + Send + Sync> {
        match runtime_type {
            RuntimeType::DockLock => Box::new(DockLockAdapter::new(api_endpoint, config)),
            RuntimeType::EncCluster => Box::new(EncClusterAdapter::new(api_endpoint, config)),
            RuntimeType::HttpCage => Box::new(HttpCageAdapter::new(api_endpoint, config)),
            RuntimeType::IoTGateway => Box::new(IoTGatewayAdapter::new(api_endpoint, config)),
            RuntimeType::MobileClient | RuntimeType::FrontendClient | RuntimeType::Custom(_) => {
                Box::new(HttpCageAdapter::new(api_endpoint, config))
            }
        }
    }
}
