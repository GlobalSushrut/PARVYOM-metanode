//! # Address Sync Agent
//! 
//! Subscribes to BlakePage Merkle root, programs BPF maps + SRv6 policies.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use anyhow::Result;
use tracing::{info, warn};

use crate::{
    iaav6::{IAAv6Address, compute_iaav6, parse_base_prefix},
    hrw::{RendezvousHasher, VPodWeight},
    srv6::SRv6PolicyManager,
    DynaRouteConfig,
};

/// Address Sync Agent - main control plane component
pub struct AddressSyncAgent {
    /// Configuration
    config: DynaRouteConfig,
    
    /// Current Merkle root
    merkle_root: Arc<RwLock<[u8; 32]>>,
    
    /// IAAv6 prefix registry: service_id → IAAv6
    iaav6_registry: Arc<RwLock<HashMap<String, IAAv6Address>>>,
    
    /// HRW rings per service: service_id → RendezvousHasher
    hrw_rings: Arc<RwLock<HashMap<String, RendezvousHasher>>>,
    
    /// SRv6 policy manager
    srv6_manager: Arc<SRv6PolicyManager>,
    
    /// Current epoch
    current_epoch: Arc<RwLock<u64>>,
}

impl AddressSyncAgent {
    /// Create new address sync agent
    pub fn new(config: DynaRouteConfig) -> Self {
        Self {
            config,
            merkle_root: Arc::new(RwLock::new([0u8; 32])),
            iaav6_registry: Arc::new(RwLock::new(HashMap::new())),
            hrw_rings: Arc::new(RwLock::new(HashMap::new())),
            srv6_manager: Arc::new(SRv6PolicyManager::new()),
            current_epoch: Arc::new(RwLock::new(Self::get_current_epoch())),
        }
    }
    
    /// Start sync loop
    pub async fn start(&self) -> Result<()> {
        info!("🚀 Starting DynaRoute Address Sync Agent");
        
        // Start epoch rotation task
        let epoch_handle = self.start_epoch_rotation();
        
        // Start merkle sync task
        let sync_handle = self.start_merkle_sync();
        
        // Wait for both tasks
        tokio::try_join!(epoch_handle, sync_handle)?;
        
        Ok(())
    }
    
    /// Start epoch rotation task
    async fn start_epoch_rotation(&self) -> Result<()> {
        let current_epoch = Arc::clone(&self.current_epoch);
        let rotation_interval = self.config.epoch_rotation_seconds;
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(rotation_interval));
            
            loop {
                interval.tick().await;
                let new_epoch = Self::get_current_epoch();
                *current_epoch.write().await = new_epoch;
                info!("🔄 Epoch rotated to: {}", new_epoch);
            }
        });
        
        Ok(())
    }
    
    /// Start Merkle sync task
    async fn start_merkle_sync(&self) -> Result<()> {
        let merkle_root = Arc::clone(&self.merkle_root);
        let _iaav6_registry = Arc::clone(&self.iaav6_registry);
        let _hrw_rings = Arc::clone(&self.hrw_rings);
        let _srv6_manager = Arc::clone(&self.srv6_manager);
        let config = self.config.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(
                Duration::from_millis(config.merkle_sync_interval_ms)
            );
            
            loop {
                interval.tick().await;
                
                // TODO: Fetch new Merkle root from BlakePage
                // For now, simulate with dummy data
                let new_root = Self::fetch_merkle_root(&config.blakepage_url).await;
                
                match new_root {
                    Ok(root) => {
                        let current_root = *merkle_root.read().await;
                        
                        if root != current_root {
                            info!("🔄 Merkle root changed, syncing...");
                            
                            // Fetch journal (diff)
                            // Build new IAAv6 prefixes, HRW rings, SRv6 seglists
                            // Atomic update
                            
                            *merkle_root.write().await = root;
                            info!("✅ Sync complete");
                        }
                    }
                    Err(e) => {
                        warn!("⚠️ Failed to fetch Merkle root: {}", e);
                    }
                }
            }
        });
        
        Ok(())
    }
    
    /// Register vPod in HRW ring
    pub async fn add_vpod_to_ring(
        &self,
        service_id: &str,
        vpod_id: String,
        weight: VPodWeight,
    ) -> Result<()> {
        let mut rings = self.hrw_rings.write().await;
        
        let ring = rings.entry(service_id.to_string())
            .or_insert_with(RendezvousHasher::new);
        
        ring.add_vpod(vpod_id.clone(), weight);
        
        info!("✅ Added vPod {} to service {} HRW ring", vpod_id, service_id);
        
        Ok(())
    }
    
    /// Remove vPod from HRW ring
    pub async fn remove_vpod_from_ring(
        &self,
        service_id: &str,
        vpod_id: &str,
    ) -> Result<()> {
        let mut rings = self.hrw_rings.write().await;
        
        if let Some(ring) = rings.get_mut(service_id) {
            ring.remove_vpod(vpod_id);
            info!("✅ Removed vPod {} from service {} HRW ring", vpod_id, service_id);
        }
        
        Ok(())
    }
    
    /// Compute IAAv6 for service
    pub async fn compute_service_iaav6(&self, service_id: &str, holder: &str) -> Result<IAAv6Address> {
        let epoch = *self.current_epoch.read().await;
        let base_prefix = parse_base_prefix(&self.config.iaav6_base_prefix)?;
        
        let iaav6 = compute_iaav6(
            holder,
            service_id,
            epoch,
            &self.config.realm,
            base_prefix,
        );
        
        Ok(iaav6)
    }
    
    /// Select vPod for flow
    pub async fn select_vpod(
        &self,
        service_id: &str,
        holder: &str,
    ) -> Result<Option<String>> {
        let rings = self.hrw_rings.read().await;
        let epoch = *self.current_epoch.read().await;
        
        if let Some(ring) = rings.get(service_id) {
            Ok(ring.select_vpod(holder, service_id, epoch))
        } else {
            Ok(None)
        }
    }
    
    /// Get current epoch (Unix timestamp)
    fn get_current_epoch() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
    
    /// Fetch Merkle root from BlakePage
    async fn fetch_merkle_root(_url: &str) -> Result<[u8; 32]> {
        // TODO: Implement actual BlakePage client
        // For now, return dummy root
        Ok([0u8; 32])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agent_creation() {
        let config = DynaRouteConfig::default();
        let agent = AddressSyncAgent::new(config);
        
        let epoch = *agent.current_epoch.read().await;
        assert!(epoch > 0);
    }
    
    #[tokio::test]
    async fn test_add_vpod_to_ring() {
        let config = DynaRouteConfig::default();
        let agent = AddressSyncAgent::new(config);
        
        agent.add_vpod_to_ring(
            "test-service",
            "vpod1".to_string(),
            VPodWeight::default(),
        ).await.unwrap();
        
        let selected = agent.select_vpod("test-service", "holder1").await.unwrap();
        assert_eq!(selected, Some("vpod1".to_string()));
    }
    
    #[tokio::test]
    async fn test_compute_iaav6() {
        let config = DynaRouteConfig::default();
        let agent = AddressSyncAgent::new(config);
        
        let iaav6 = agent.compute_service_iaav6("test-service", "test-holder").await.unwrap();
        assert_ne!(iaav6.to_string(), "::".to_string());
    }
}
