use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

use crate::bpi_wallet_command::WalletState;
use crate::virtual_addressing_system::{
    AddressResolution, AddressSecurity, VirtualAddress, VirtualAddressType, VirtualAddressingSystem,
};
use crate::xtmp_bpci_client::{ProductionToken, ProductionWalletAddress, XTMPBpciClient};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletAddressRecord {
    pub wallet_id: String,
    pub network: String,
    pub bpci_domain: Option<String>,
    pub registry_address: Option<String>,
    pub virtual_address: Option<VirtualAddress>,
    pub last_synced: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct WalletAddressOrchestrator {
    vas: Arc<VirtualAddressingSystem>,
    bpci_endpoint: String,
}

impl WalletAddressOrchestrator {
    pub fn new(system_id: String, bpci_endpoint: String) -> Result<Self> {
        let vas = VirtualAddressingSystem::new(system_id)?;
        Ok(Self {
            vas: Arc::new(vas),
            bpci_endpoint,
        })
    }

    pub async fn load_wallet_record(&self) -> Result<WalletAddressRecord> {
        let state = WalletState::load()?;

        let virtual_address = if state.bpci_connected {
            match self.ensure_virtual_address_for_wallet(&state.wallet_id).await {
                Ok(addr) => Some(addr),
                Err(e) => {
                    warn!("failed to allocate virtual address for wallet {}: {}", state.wallet_id, e);
                    None
                }
            }
        } else {
            None
        };

        Ok(WalletAddressRecord {
            wallet_id: state.wallet_id,
            network: state.network,
            bpci_domain: state.bpci_domain,
            registry_address: state.registry_address,
            virtual_address,
            last_synced: state.last_connection,
        })
    }

    async fn ensure_virtual_address_for_wallet(&self, wallet_id: &str) -> Result<VirtualAddress> {
        {
            let mapping = self.vas.service_mapping.read().await;
            if let Some(existing) = mapping.get(wallet_id) {
                return Ok(existing.clone());
            }
        }

        self.vas
            .allocate_virtual_address(
                wallet_id.to_string(),
                format!("wallet-node-{}", wallet_id),
                VirtualAddressType::ApplicationService,
                AddressSecurity::High,
            )
            .await
    }

    pub async fn resolve_wallet_virtual_address(&self, wallet_id: &str) -> Result<AddressResolution> {
        self.vas.resolve_service(wallet_id).await
    }

    pub async fn register_wallet_with_bpci(&self, auth_token: &str) -> Result<()> {
        let mut state = WalletState::load()?;

        let bpci_domain = state
            .bpci_domain
            .clone()
            .ok_or_else(|| anyhow!("BPCI domain not set in wallet state"))?;

        let registry_address = state
            .registry_address
            .clone()
            .ok_or_else(|| anyhow!("Registry address not set in wallet state"))?;

        let wallet_address: ProductionWalletAddress = registry_address.clone();
        let token: ProductionToken = auth_token.to_string();

        info!(
            "registering wallet with BPCI via XTMP: wallet={}, endpoint={}",
            wallet_address, self.bpci_endpoint
        );

        let mut client = XTMPBpciClient::new(self.bpci_endpoint.clone()).await?;
        let response = client.register_wallet(&wallet_address, &token).await?;
        client.close().await?;

        if response.status != "success" {
            return Err(anyhow!(
                "BPCI wallet registration failed: {}",
                response.message
            ));
        }

        state.bpci_connected = true;
        state.save()?;

        info!("wallet registration completed: registration_id={}", response.registration_id);
        Ok(())
    }
}
