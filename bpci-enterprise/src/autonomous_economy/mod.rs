//! Autonomous Economy Module for BPCI
//! 
//! Implements the formal mathematical model for Mother-Daughter coin distribution:
//! - GEN (Mother Coin): Governance reserve anchor
//! - NEX (Daughter Coin): PoE mining rewards  
//! - FLX (Daughter Coin): Network usage fees
//! - AUR (Bank Coin): Bank-stamped wallets only
//! 
//! Mathematical Model:
//! F = C + T where C = 0.25F (coins) and T = 0.75F (treasury)
//! 
//! Mother Coin: C_fix^M = 0.125F, C_claim^M = 0.125F
//! Daughter Coin: C_fix^D = 0.075F, C_claim^D = 0.125F, C_fix_to_M = 0.075F
//! Treasury: T_company = 0.1875F, T_owner = 0.10F, T_community = 0.20F, T_infra = 0.20F

pub mod coin_distribution;
pub mod work_proof;
pub mod bpi_integration;
pub mod settlement_coin;
pub mod bpci_economic_integration;
pub mod bpci_treasury_integration;
pub mod economic_distribution_flow;
pub mod economic_flow_demo;
pub mod bank_api_integration;
pub mod mother_coin_distribution;
pub mod internal_governance;
pub mod internal_governance_engine;

pub use coin_distribution::*;
pub use work_proof::*;
pub use bpi_integration::*;
pub use settlement_coin::*;
pub use bpci_economic_integration::*;
pub use bpci_treasury_integration::*;
pub use economic_distribution_flow::*;
pub use economic_flow_demo::*;
pub use bank_api_integration::*;
pub use mother_coin_distribution::*;
pub use internal_governance::*;
pub use internal_governance_engine::*;
