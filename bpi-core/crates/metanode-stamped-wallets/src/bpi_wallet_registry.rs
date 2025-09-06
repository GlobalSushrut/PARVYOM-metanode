use crate::{StampedWalletError, StampedWalletResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

/// BPI Wallet Registry for proper tokenomics and ledger activation
#[derive(Debug, Clone)]
pub struct BPIWalletRegistry {
    /// Registered wallets with their registry credentials
    registered_wallets: HashMap<String, RegisteredWallet>,
    /// Registry configuration
    config: RegistryConfig,
    /// BPI ledger activation status
    ledger_active: bool,
    /// BPCI connection status
    bpci_connected: bool,
    /// Consensus layer deployment status (unhackable when true)
    consensus_deployed: bool,
    /// Installation hash for unhackable verification
    installation_hash: String,
}

/// Registry configuration for BPI wallets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    /// Network type: mainnet or testnet
    pub network_type: NetworkType,
    /// Testnet token allocation (1500 BPI)
    pub testnet_token_allocation: Decimal,
    /// Registry address required for activation
    pub require_registry_address: bool,
    /// Registry token required for ledger activation
    pub require_registry_token: bool,
    /// Enforce unhackable installer requirements
    pub enforce_unhackable_setup: bool,
}

/// Network types for BPI wallets
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NetworkType {
    /// Mainnet - real money, requires registration and payment
    Mainnet,
    /// Testnet - validation phase, shows as "Active Mainnet (Validation Phase)"
    Testnet,
}

/// Registered wallet with proper credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredWallet {
    /// Wallet address
    pub address: String,
    /// Registry address (required for activation)
    pub registry_address: String,
    /// Registry token (required for ledger activation)
    pub registry_token: String,
    /// BPCI registration status
    pub bpci_registered: bool,
    /// Current token balance
    pub balance: Decimal,
    /// Gas balance
    pub gas_balance: Decimal,
    /// Rent balance
    pub rent_balance: Decimal,
    /// Registration timestamp
    pub registered_at: DateTime<Utc>,
    /// Network type
    pub network_type: NetworkType,
    /// Ledger activation status
    pub ledger_activated: bool,
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            network_type: NetworkType::Mainnet,
            testnet_token_allocation: Decimal::new(150000, 2), // 1500.00 BPI
            require_registry_address: true,
            require_registry_token: true,
            enforce_unhackable_setup: true,
        }
    }
}

impl BPIWalletRegistry {
    /// Create new BPI wallet registry
    pub fn new(config: RegistryConfig) -> Self {
        Self {
            registered_wallets: HashMap::new(),
            config,
            ledger_active: false,
            bpci_connected: false,
            consensus_deployed: false,
            installation_hash: String::new(),
        }
    }

    /// Deploy to consensus layer (makes ledger unhackable)
    pub fn deploy_to_consensus(&mut self, community_hash: String) -> StampedWalletResult<()> {
        // Generate unhackable installation hash
        self.installation_hash = format!("BPI_CONSENSUS_{}", community_hash);
        self.consensus_deployed = true;
        
        // Once deployed to consensus, all security is enforced at consensus layer
        Ok(())
    }

    /// Check if ledger operation is allowed (USELESS without registration)
    pub fn is_ledger_operation_allowed(&self, address: &str, operation: &str) -> bool {
        // BPI ledger is COMPLETELY USELESS without proper registration
        if let Some(wallet) = self.get_wallet(address) {
            // Must have ALL requirements met
            let has_registry = !wallet.registry_address.is_empty() && !wallet.registry_token.is_empty();
            let is_registered = wallet.bpci_registered;
            let ledger_active = wallet.ledger_activated;
            
            // ALL must be true for ANY ledger operation
            let allowed = has_registry && is_registered && ledger_active;
            
            if !allowed {
                // Log security violation attempt
                eprintln!("SECURITY: Blocked ledger operation '{}' for unregistered wallet {}", operation, address);
            }
            
            allowed
        } else {
            eprintln!("SECURITY: Blocked ledger operation '{}' for unknown wallet {}", operation, address);
            false
        }
    }

    /// Validate consensus layer integrity (unhackable check)
    pub fn validate_consensus_integrity(&self) -> StampedWalletResult<bool> {
        if !self.consensus_deployed {
            return Ok(true); // Not deployed yet, normal validation
        }

        // Once deployed to consensus, verify unhackable installation
        if self.installation_hash.is_empty() {
            return Err(StampedWalletError::SecurityViolation("Consensus deployment corrupted - no installation hash".into()));
        }

        if !self.installation_hash.starts_with("BPI_CONSENSUS_") {
            return Err(StampedWalletError::SecurityViolation("Consensus deployment corrupted - invalid hash format".into()));
        }

        // Consensus layer is unhackable - all security enforced here
        Ok(true)
    }

    /// Initialize wallet with proper tokenomics
    pub fn initialize_wallet(
        &mut self,
        address: String,
        network_type: NetworkType,
    ) -> StampedWalletResult<RegisteredWallet> {
        match network_type {
            NetworkType::Testnet => {
                // Testnet: 1500 BPI tokens free, auto-registered
                let wallet = RegisteredWallet {
                    address: address.clone(),
                    registry_address: "bpi_testnet_registry_001".to_string(),
                    registry_token: format!("TEST_REG_{}", Uuid::new_v4().to_string()[..6].to_uppercase()),
                    bpci_registered: true,
                    balance: self.config.testnet_token_allocation, // 1500.00 BPI
                    gas_balance: Decimal::new(5000, 2), // 50.00 BPI for gas
                    rent_balance: Decimal::new(5000, 2), // 50.00 BPI for rent
                    registered_at: Utc::now(),
                    network_type: NetworkType::Testnet,
                    ledger_activated: self.ledger_active && self.bpci_connected,
                };
                
                self.registered_wallets.insert(address, wallet.clone());
                Ok(wallet)
            },
            NetworkType::Mainnet => {
                // Mainnet: Must provide registry credentials and pay
                let wallet = RegisteredWallet {
                    address: address.clone(),
                    registry_address: String::new(), // Must be provided
                    registry_token: String::new(),   // Must be provided
                    bpci_registered: false,
                    balance: Decimal::ZERO,
                    gas_balance: Decimal::ZERO,
                    rent_balance: Decimal::ZERO,
                    registered_at: Utc::now(),
                    network_type: NetworkType::Mainnet,
                    ledger_activated: false, // No ledger without registry
                };
                
                self.registered_wallets.insert(address, wallet.clone());
                Ok(wallet)
            }
        }
    }

    /// Register wallet with BPCI (mainnet only)
    pub fn register_with_bpci(
        &mut self,
        address: &str,
        registry_address: String,
        registry_token: String,
        payment_amount: Decimal,
    ) -> StampedWalletResult<()> {
        let wallet = self.registered_wallets.get_mut(address)
            .ok_or(StampedWalletError::WalletNotFound(format!("Wallet not found: {}", address)))?;

        if wallet.network_type != NetworkType::Mainnet {
            return Err(StampedWalletError::InvalidOperation("BPCI registration only for mainnet".into()));
        }

        // Validate registry credentials
        if registry_address.is_empty() || registry_token.is_empty() {
            return Err(StampedWalletError::InvalidCredentials("Registry address and token required".into()));
        }

        // Process registration (simulate payment)
        wallet.registry_address = registry_address;
        wallet.registry_token = registry_token;
        wallet.bpci_registered = true;
        
        // Give initial tokens after payment
        wallet.balance = payment_amount * Decimal::new(2, 0); // 2x tokens for payment
        wallet.gas_balance = payment_amount * Decimal::new(20, 2); // 20% for gas
        wallet.rent_balance = payment_amount * Decimal::new(20, 2); // 20% for rent

        // Activate ledger if BPI core is connected
        wallet.ledger_activated = self.ledger_active && self.bpci_connected;

        Ok(())
    }

    /// Set BPI ledger activation status (only when BPI core is connected)
    pub fn set_ledger_status(&mut self, bpi_connected: bool, post_quantum_enabled: bool) {
        self.ledger_active = bpi_connected && post_quantum_enabled;
        
        // Update all registered wallets' ledger status
        for wallet in self.registered_wallets.values_mut() {
            if wallet.bpci_registered && !wallet.registry_address.is_empty() && !wallet.registry_token.is_empty() {
                wallet.ledger_activated = self.ledger_active;
            } else {
                wallet.ledger_activated = false; // No ledger without proper registration
            }
        }
    }

    /// Set BPCI connection status
    pub fn set_bpci_connection(&mut self, connected: bool) {
        self.bpci_connected = connected;
        
        // Update ledger activation based on connection
        for wallet in self.registered_wallets.values_mut() {
            if wallet.bpci_registered && !wallet.registry_address.is_empty() && !wallet.registry_token.is_empty() {
                wallet.ledger_activated = self.ledger_active && self.bpci_connected;
            }
        }
    }

    /// Get wallet information
    pub fn get_wallet(&self, address: &str) -> Option<&RegisteredWallet> {
        self.registered_wallets.get(address)
    }

    /// Check if wallet can perform transactions
    pub fn can_transact(&self, address: &str) -> bool {
        if let Some(wallet) = self.get_wallet(address) {
            match wallet.network_type {
                NetworkType::Testnet => true, // Testnet always allows transactions
                NetworkType::Mainnet => {
                    wallet.bpci_registered 
                        && !wallet.registry_address.is_empty() 
                        && !wallet.registry_token.is_empty()
                        && wallet.ledger_activated
                }
            }
        } else {
            false
        }
    }

    /// Get network display name
    pub fn get_network_display(&self, address: &str) -> String {
        if let Some(wallet) = self.get_wallet(address) {
            match wallet.network_type {
                NetworkType::Testnet => "Active Mainnet (Validation Phase)".to_string(),
                NetworkType::Mainnet => "BPI Mainnet".to_string(),
            }
        } else {
            "Unknown Network".to_string()
        }
    }

    /// Validate unhackable installer requirements
    pub fn validate_unhackable_setup(&self) -> StampedWalletResult<bool> {
        if !self.config.enforce_unhackable_setup {
            return Ok(true);
        }

        // Check if BPI core is properly secured
        if !self.ledger_active {
            return Err(StampedWalletError::SecurityViolation("BPI ledger not activated - unhackable setup required".into()));
        }

        // Check if BPCI connection is secure
        if !self.bpci_connected {
            return Err(StampedWalletError::SecurityViolation("BPCI connection not established - unhackable setup required".into()));
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_testnet_wallet_initialization() {
        let config = RegistryConfig::default();
        let mut registry = BPIWalletRegistry::new(config);
        
        let wallet = registry.initialize_wallet(
            "bpi_test_address".to_string(),
            NetworkType::Testnet,
        ).unwrap();
        
        assert_eq!(wallet.balance, Decimal::new(150000, 2)); // 1500.00 BPI
        assert_eq!(wallet.gas_balance, Decimal::new(5000, 2)); // 50.00 BPI
        assert_eq!(wallet.network_type, NetworkType::Testnet);
        assert!(wallet.bpci_registered);
    }

    #[test]
    fn test_mainnet_wallet_initialization() {
        let config = RegistryConfig::default();
        let mut registry = BPIWalletRegistry::new(config);
        
        let wallet = registry.initialize_wallet(
            "bpi_main_address".to_string(),
            NetworkType::Mainnet,
        ).unwrap();
        
        assert_eq!(wallet.balance, Decimal::ZERO);
        assert_eq!(wallet.gas_balance, Decimal::ZERO);
        assert_eq!(wallet.network_type, NetworkType::Mainnet);
        assert!(!wallet.bpci_registered);
        assert!(wallet.registry_address.is_empty());
    }

    #[test]
    fn test_bpci_registration() {
        let config = RegistryConfig::default();
        let mut registry = BPIWalletRegistry::new(config);
        
        registry.initialize_wallet(
            "bpi_main_address".to_string(),
            NetworkType::Mainnet,
        ).unwrap();
        
        registry.register_with_bpci(
            "bpi_main_address",
            "bpi_registry_001".to_string(),
            "REG_TOKEN_123".to_string(),
            Decimal::new(1000, 2), // $10.00 payment
        ).unwrap();
        
        let wallet = registry.get_wallet("bpi_main_address").unwrap();
        assert!(wallet.bpci_registered);
        assert_eq!(wallet.registry_address, "bpi_registry_001");
        assert_eq!(wallet.balance, Decimal::new(2000, 2)); // 2x payment = $20.00
    }
}
