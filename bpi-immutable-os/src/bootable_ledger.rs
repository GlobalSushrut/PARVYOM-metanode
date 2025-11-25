//! Bootable Ledger OS - First OS-level blockchain

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tracing::info;
use uuid::Uuid;

use blake3;
use hex;

use crate::blockchain_os_kernel::SixDConsensusEngine;
use crate::filesystem_engine::FilesystemImmutabilityEngine;
use crate::hardware_detection::HardwareProfile;
use vpods_core::id::NodeId;

/// Bootable Ledger OS - The OS IS the blockchain
#[derive(Debug)]
pub struct BootableLedgerOS {
    pub node_id: NodeId,
    pub consensus_engine: SixDConsensusEngine,
    pub filesystem_engine: FilesystemImmutabilityEngine,
    pub hardware_profile: HardwareProfile,
    pub ledger_config: BootableLedgerConfig,
    pub ledger_domain: LedgerDomain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootableLedgerConfig {
    pub ledger_id: String,
    pub network: LedgerNetworkConfig,
    pub consensus: LedgerConsensusConfig,
    pub storage: LedgerStorageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerNetworkConfig {
    pub p2p_port: u16,
    pub rpc_port: u16,
    pub bootstrap_nodes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerConsensusConfig {
    pub min_validators: usize,
    pub block_time_ms: u64,
    pub finality_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerStorageConfig {
    pub era_root: PathBuf,
    pub blockchain_data_path: PathBuf,
    pub max_storage_gb: u64,
}

#[derive(Debug, Clone)]
pub struct LedgerDomain {
    pub ledger_id: String,
    pub ledger_hash: String,
    pub era_root: PathBuf,
    pub data_root: PathBuf,
    pub objects_root: PathBuf,
    pub cgroup_path: String,
    pub net_ns_name: String,
    pub pid_ns_name: String,
    pub mnt_ns_name: String,
    pub uts_ns_name: String,
}

impl LedgerDomain {
    pub fn new(era_root: PathBuf, ledger_id: String) -> Result<Self> {
        let hash = blake3::hash(ledger_id.as_bytes());
        let full = hex::encode(hash.as_bytes());
        let short = full.chars().take(12).collect::<String>();

        let data_root = era_root
            .join("mutable")
            .join("var")
            .join("bpi")
            .join("ledger")
            .join(&short);

        let objects_root = era_root.join("store").join("objects");

        fs::create_dir_all(&data_root)?;
        fs::create_dir_all(&objects_root)?;

        let cgroup_path = format!("/sys/fs/cgroup/bpi-ledger/{}", short);
        let net_ns_name = format!("bpi6d-net-{}", short);
        let pid_ns_name = format!("bpi6d-pid-{}", short);
        let mnt_ns_name = format!("bpi6d-mnt-{}", short);
        let uts_ns_name = format!("bpi6d-uts-{}", short);

        Ok(Self {
            ledger_id,
            ledger_hash: short,
            era_root,
            data_root,
            objects_root,
            cgroup_path,
            net_ns_name,
            pid_ns_name,
            mnt_ns_name,
            uts_ns_name,
        })
    }
}

impl BootableLedgerOS {
    pub async fn new(
        hardware_profile: HardwareProfile,
        ledger_config: BootableLedgerConfig,
    ) -> Result<Self> {
        let node_id = NodeId(Uuid::new_v4());
        
        let consensus_config = crate::blockchain_os_kernel::consensus_engine::ConsensusConfig {
            min_validators: ledger_config.consensus.min_validators,
            finality_threshold: ledger_config.consensus.finality_threshold,
            block_time_ms: ledger_config.consensus.block_time_ms,
        };
        let consensus_engine = SixDConsensusEngine::new(consensus_config).await?;
        
        let filesystem_engine = FilesystemImmutabilityEngine::new().await?;

        let ledger_domain = LedgerDomain::new(
            ledger_config.storage.era_root.clone(),
            ledger_config.ledger_id.clone(),
        )?;

        Ok(Self {
            node_id,
            consensus_engine,
            filesystem_engine,
            hardware_profile,
            ledger_config,
            ledger_domain,
        })
    }

    pub async fn boot(&mut self) -> Result<()> {
        info!("🚀 Booting Ledger OS - Blockchain becoming the OS");
        
        self.consensus_engine.initialize_genesis().await?;
        info!("✅ Genesis ledger initialized");
        
        info!("🎉 Bootable Ledger OS operational");
        Ok(())
    }
}
