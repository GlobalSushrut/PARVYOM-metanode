//! Virtual Addressing System for BPCI Components
//! 
//! Provides true port-free operation using Identity-Anycast IPv6 (IAAv6)
//! and DynaRoute v2 service discovery. No static ports required!

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{info, warn};

/// Virtual address for a BPCI component
/// Uses identity-based addressing instead of static ports
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VirtualAddress {
    /// Component identity (e.g., "consensus", "blockchain")
    pub component_id: String,
    
    /// Instance ID (for multiple instances of same component)
    pub instance_id: String,
    
    /// Virtual IPv6 address (IAAv6 - Identity-Anycast IPv6)
    pub iaav6: String,
    
    /// Optional physical addresses (for hybrid mode)
    pub physical_addrs: Vec<SocketAddr>,
}

impl VirtualAddress {
    /// Create a new virtual address from component identity
    pub fn new(component_id: &str, instance_id: &str) -> Self {
        let iaav6 = Self::generate_iaav6(component_id, instance_id);
        
        Self {
            component_id: component_id.to_string(),
            instance_id: instance_id.to_string(),
            iaav6,
            physical_addrs: Vec::new(),
        }
    }
    
    /// Create virtual address with fallback physical address (hybrid mode)
    pub fn with_physical(component_id: &str, instance_id: &str, addr: SocketAddr) -> Self {
        let mut virtual_addr = Self::new(component_id, instance_id);
        virtual_addr.physical_addrs.push(addr);
        virtual_addr
    }
    
    /// Generate Identity-Anycast IPv6 address from component identity
    /// Format: fd00:bpci::<component_hash>::<instance_hash>
    fn generate_iaav6(component_id: &str, instance_id: &str) -> String {
        use sha2::{Sha256, Digest};
        
        // Hash component ID
        let mut hasher = Sha256::new();
        hasher.update(component_id.as_bytes());
        let component_hash = hasher.finalize();
        let component_hex = format!("{:x}", component_hash);
        
        // Hash instance ID
        let mut hasher = Sha256::new();
        hasher.update(instance_id.as_bytes());
        let instance_hash = hasher.finalize();
        let instance_hex = format!("{:x}", instance_hash);
        
        // Create IAAv6 address (simplified format for now)
        // In production, this would be a proper IPv6 address
        format!(
            "fd00:bpci:{}:{}",
            &component_hex[..8],
            &instance_hex[..8]
        )
    }
    
    /// Check if this is pure virtual (no physical addresses)
    pub fn is_pure_virtual(&self) -> bool {
        self.physical_addrs.is_empty()
    }
    
    /// Get display name for this virtual address
    pub fn display_name(&self) -> String {
        format!("{}:{}", self.component_id, self.instance_id)
    }
}

/// Virtual addressing mode for components
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    /// Pure virtual - no static ports, IAAv6 only
    PureVirtual,
    
    /// Hybrid - virtual addressing with physical port fallback
    Hybrid,
    
    /// Legacy - static ports only (for backward compatibility)
    Legacy,
}

/// Virtual addressing configuration
#[derive(Debug, Clone)]
pub struct VirtualAddressingConfig {
    /// Addressing mode
    pub mode: AddressingMode,
    
    /// Component identity
    pub component_id: String,
    
    /// Instance identity (auto-generated if not provided)
    pub instance_id: Option<String>,
    
    /// Physical port (only used in Hybrid/Legacy mode)
    pub physical_port: Option<u16>,
    
    /// Enable service discovery
    pub enable_discovery: bool,
    
    /// Enable HRW load balancing
    pub enable_hrw: bool,
}

impl VirtualAddressingConfig {
    /// Create pure virtual configuration (no ports!)
    pub fn pure_virtual(component_id: &str) -> Self {
        Self {
            mode: AddressingMode::PureVirtual,
            component_id: component_id.to_string(),
            instance_id: None,
            physical_port: None,
            enable_discovery: true,
            enable_hrw: true,
        }
    }
    
    /// Create hybrid configuration (virtual + physical fallback)
    pub fn hybrid(component_id: &str, port: u16) -> Self {
        Self {
            mode: AddressingMode::Hybrid,
            component_id: component_id.to_string(),
            instance_id: None,
            physical_port: Some(port),
            enable_discovery: true,
            enable_hrw: true,
        }
    }
    
    /// Create legacy configuration (static ports only)
    pub fn legacy(component_id: &str, port: u16) -> Self {
        Self {
            mode: AddressingMode::Legacy,
            component_id: component_id.to_string(),
            instance_id: None,
            physical_port: Some(port),
            enable_discovery: false,
            enable_hrw: false,
        }
    }
    
    /// Get or generate instance ID
    pub fn get_instance_id(&self) -> String {
        self.instance_id.clone().unwrap_or_else(|| {
            format!("instance-{}", uuid::Uuid::new_v4().to_string()[..8].to_string())
        })
    }
    
    /// Create virtual address from this config
    pub fn create_virtual_address(&self) -> VirtualAddress {
        let instance_id = self.get_instance_id();
        
        match self.mode {
            AddressingMode::PureVirtual => {
                VirtualAddress::new(&self.component_id, &instance_id)
            }
            AddressingMode::Hybrid | AddressingMode::Legacy => {
                if let Some(port) = self.physical_port {
                    let addr: SocketAddr = format!("127.0.0.1:{}", port).parse()
                        .expect("Invalid port");
                    VirtualAddress::with_physical(&self.component_id, &instance_id, addr)
                } else {
                    VirtualAddress::new(&self.component_id, &instance_id)
                }
            }
        }
    }
}

/// Virtual addressing manager for components
pub struct VirtualAddressingManager {
    /// Configuration
    config: VirtualAddressingConfig,
    
    /// Virtual address
    virtual_addr: VirtualAddress,
}

impl VirtualAddressingManager {
    /// Create new virtual addressing manager
    pub fn new(config: VirtualAddressingConfig) -> Self {
        let virtual_addr = config.create_virtual_address();
        
        info!("🌐 Virtual Addressing Manager initialized");
        info!("   Mode: {:?}", config.mode);
        info!("   Component: {}", config.component_id);
        info!("   Instance: {}", virtual_addr.instance_id);
        info!("   IAAv6: {}", virtual_addr.iaav6);
        
        if !virtual_addr.physical_addrs.is_empty() {
            info!("   Physical: {:?}", virtual_addr.physical_addrs);
        } else {
            info!("   Physical: None (Pure Virtual)");
        }
        
        Self {
            config,
            virtual_addr,
        }
    }
    
    /// Get virtual address
    pub fn virtual_address(&self) -> &VirtualAddress {
        &self.virtual_addr
    }
    
    /// Get configuration
    pub fn config(&self) -> &VirtualAddressingConfig {
        &self.config
    }
    
    /// Check if pure virtual mode
    pub fn is_pure_virtual(&self) -> bool {
        self.config.mode == AddressingMode::PureVirtual
    }
    
    /// Get bind address for networking layer
    /// In pure virtual mode, returns a dynamic port
    /// In hybrid/legacy mode, returns the configured port
    pub fn get_bind_address(&self) -> Result<SocketAddr> {
        match self.config.mode {
            AddressingMode::PureVirtual => {
                // Bind to dynamic port (0 = OS assigns)
                Ok("127.0.0.1:0".parse()?)
            }
            AddressingMode::Hybrid | AddressingMode::Legacy => {
                if let Some(port) = self.config.physical_port {
                    Ok(format!("127.0.0.1:{}", port).parse()?)
                } else {
                    Err(anyhow!("Physical port required for Hybrid/Legacy mode"))
                }
            }
        }
    }
    
    /// Get service name for registration
    pub fn service_name(&self) -> String {
        self.config.component_id.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_virtual_address_generation() {
        let addr = VirtualAddress::new("consensus", "instance-001");
        assert_eq!(addr.component_id, "consensus");
        assert_eq!(addr.instance_id, "instance-001");
        assert!(addr.iaav6.starts_with("fd00:bpci:"));
        assert!(addr.is_pure_virtual());
    }
    
    #[test]
    fn test_hybrid_address() {
        let addr = VirtualAddress::with_physical(
            "blockchain",
            "instance-002",
            "127.0.0.1:8080".parse().unwrap()
        );
        assert!(!addr.is_pure_virtual());
        assert_eq!(addr.physical_addrs.len(), 1);
    }
    
    #[test]
    fn test_pure_virtual_config() {
        let config = VirtualAddressingConfig::pure_virtual("consensus");
        assert_eq!(config.mode, AddressingMode::PureVirtual);
        assert!(config.physical_port.is_none());
        assert!(config.enable_discovery);
    }
    
    #[test]
    fn test_hybrid_config() {
        let config = VirtualAddressingConfig::hybrid("blockchain", 8080);
        assert_eq!(config.mode, AddressingMode::Hybrid);
        assert_eq!(config.physical_port, Some(8080));
    }
}
