use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use ed25519_dalek::{Keypair, Signature, Signer, Verifier};
use sha2::{Sha256, Digest};
use anyhow::{Result, anyhow};
use uuid::Uuid;
use crate::wallet_identity::WalletIdentity;
use crate::identity_registry::IdentityRegistry;

/// XTMPPAY - Universal Payment Protocol
/// Enables seamless payments between any wallet types with multi-rail support

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XTMPPayment {
    /// Unique payment identifier
    pub payment_id: String,
    /// Sender wallet address
    pub from_wallet: String,
    /// Recipient wallet address
    pub to_wallet: String,
    /// Payment amount and currency
    pub amount: PaymentAmount,
    /// Payment proof and compliance
    pub payment_proof: PaymentProof,
    /// Settlement configuration
    pub settlement: SettlementConfig,
    /// Payment status
    pub status: PaymentStatus,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    /// BPI receipt hash
    pub bpi_receipt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentAmount {
    pub value: f64,
    pub currency: String,
    /// Supported settlement rails for different payment types
    pub rails: Vec<SettlementRail>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SettlementRail {
    /// Automated Clearing House (US)
    ACH,
    /// Single Euro Payments Area
    SEPA,
    /// Real-Time Payments (US)
    RTP,
    /// Interac (Canada)
    INTERAC,
    /// BPI Native Settlement
    BPI,
    /// Cryptocurrency Settlement
    Crypto,
    /// SWIFT International
    SWIFT,
    /// Custom settlement rail
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentProof {
    /// Digital signature of payment
    pub signature: Vec<u8>,
    /// Zero-knowledge proof of funds
    pub witness: Option<String>,
    /// Compliance check result
    pub compliance: ComplianceCheck,
    /// KYC/AML verification
    pub kyc_aml: Option<KYCAMLCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheck {
    pub status: ComplianceStatus,
    pub jurisdiction: String,
    pub tax_implications: Option<TaxInfo>,
    pub regulatory_flags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceStatus {
    Approved,
    Pending,
    RequiresReview,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KYCAMLCheck {
    pub kyc_level: KYCLevel,
    pub aml_risk_score: f32,
    pub sanctions_check: bool,
    pub pep_check: bool, // Politically Exposed Person
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KYCLevel {
    Basic,
    Enhanced,
    Premium,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxInfo {
    pub taxable: bool,
    pub tax_rate: f32,
    pub tax_jurisdiction: String,
    pub reporting_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementConfig {
    /// Selected settlement rail
    pub rail: SettlementRail,
    /// Estimated settlement time
    pub estimated_time: String,
    /// Transaction fees
    pub fees: f64,
    /// Fee currency
    pub fee_currency: String,
    /// Settlement priority
    pub priority: SettlementPriority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SettlementPriority {
    Standard,
    Express,
    Instant,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentStatus {
    Created,
    Pending,
    Processing,
    Settled,
    Failed,
    Cancelled,
    Disputed,
}

/// Cross-wallet payment flows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentFlow {
    pub flow_type: PaymentFlowType,
    pub participants: Vec<String>,
    pub steps: Vec<PaymentStep>,
    pub total_time: String,
    pub total_fees: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub enum PaymentFlowType {
    PravyomToBank,
    GovernmentToCitizen,
    BusinessToBusiness,
    International,
    PeerToPeer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentStep {
    pub step_number: u32,
    pub description: String,
    pub estimated_time: String,
    pub status: StepStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EscrowStatus {
    Active,
    Released,
    Disputed,
    Expired,
}

/// Payment request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub payment_id: String,
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
    pub currency: String,
    pub rail: SettlementRail,
    pub fees: f64,
    pub memo: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub status: PaymentStatus,
}

/// Payment record for completed transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRecord {
    pub payment_id: String,
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
    pub currency: String,
    pub rail: SettlementRail,
    pub fees: f64,
    pub memo: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub status: PaymentStatus,
    pub transaction_hash: Option<String>,
    pub settlement_reference: Option<String>,
}

/// Escrow account for secure payments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscrowAccount {
    pub escrow_id: String,
    pub payment_id: String,
    pub amount: f64,
    pub currency: String,
    pub created_at: DateTime<Utc>,
    pub release_conditions: Vec<String>,
    pub status: EscrowStatus,
}

/// Jurisdiction rules for compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JurisdictionRules {
    pub max_transaction_amount: f64,
    pub requires_kyc_above: f64,
    pub tax_rate: f64,
    pub reporting_threshold: f64,
    pub restricted_countries: Vec<String>,
}

pub struct XTMPPayService {
    wallet: WalletIdentity,
    identity_registry: IdentityRegistry,
    supported_rails: Vec<SettlementRail>,
    compliance_engine: ComplianceEngine,
    payment_history: HashMap<String, PaymentRecord>,
    pending_payments: HashMap<String, PaymentRequest>,
    escrow_accounts: HashMap<String, EscrowAccount>,
    exchange_rates: HashMap<String, f64>,
    settlement_nodes: Vec<String>,
    payments: HashMap<String, XTMPPayment>,
    flows: HashMap<PaymentFlowType, PaymentFlow>,
    rail_configs: HashMap<SettlementRail, RailConfig>,
}

#[derive(Debug, Clone)]
pub struct RailConfig {
    pub name: String,
    pub base_fee: f64,
    pub percentage_fee: f32,
    pub min_amount: f64,
    pub max_amount: f64,
    pub settlement_time: String,
    pub supported_currencies: Vec<String>,
    pub requires_kyc: bool,
}

pub struct ComplianceEngine {
    /// Jurisdiction rules
    jurisdiction_rules: HashMap<String, JurisdictionRules>,
    /// Sanctions lists
    sanctions_list: Vec<String>,
    /// PEP database
    pep_database: Vec<String>,
}



impl XTMPPayService {
    /// Create new XTMP Pay service with enhanced capabilities
    pub fn new(wallet: WalletIdentity, identity_registry: IdentityRegistry) -> Result<Self> {
        let mut service = Self {
            wallet,
            identity_registry,
            supported_rails: vec![
                SettlementRail::ACH,
                SettlementRail::SEPA,
                SettlementRail::RTP,
                SettlementRail::INTERAC,
                SettlementRail::BPI,
                SettlementRail::SWIFT,
            ],
            compliance_engine: ComplianceEngine::new(),
            payment_history: HashMap::new(),
            pending_payments: HashMap::new(),
            escrow_accounts: HashMap::new(),
            exchange_rates: HashMap::new(),
            settlement_nodes: Vec::new(),
            payments: HashMap::new(),
            flows: HashMap::new(),
            rail_configs: HashMap::new(),
        };

        // Initialize rail configurations
        service.initialize_rail_configs();
        
        // Initialize exchange rates
        service.update_exchange_rates()?;
        
        // Connect to settlement nodes
        service.connect_settlement_nodes()?;

        Ok(service)
    }

    /// Update real-time exchange rates
    fn update_exchange_rates(&mut self) -> Result<()> {
        // In production, this would connect to real exchange rate APIs
        self.exchange_rates.insert("USD".to_string(), 1.0);
        self.exchange_rates.insert("EUR".to_string(), 0.85);
        self.exchange_rates.insert("GBP".to_string(), 0.73);
        self.exchange_rates.insert("JPY".to_string(), 110.0);
        self.exchange_rates.insert("BTC".to_string(), 45000.0);
        self.exchange_rates.insert("ETH".to_string(), 3000.0);
        self.exchange_rates.insert("BPCI".to_string(), 2.05);
        Ok(())
    }

    /// Connect to settlement network nodes
    fn connect_settlement_nodes(&mut self) -> Result<()> {
        // In production, this would discover and connect to real settlement nodes
        self.settlement_nodes = vec![
            "settlement-node-1.bpi.network".to_string(),
            "settlement-node-2.bpi.network".to_string(),
            "settlement-node-3.bpi.network".to_string(),
        ];
        Ok(())
    }

    /// Create a new payment
    pub fn create_payment(
        &mut self,
        sender: &WalletIdentity,
        recipient_address: &str,
        amount: f64,
        currency: &str,
        preferred_rails: Vec<SettlementRail>,
    ) -> Result<String, XTMPPayError> {
        let payment_id = format!("pay_{}", Uuid::new_v4().to_string().replace("-", ""));
        
        // Run compliance checks
        self.compliance_engine.check_payment_compliance(
            &sender.wallet_address,
            recipient_address,
            amount,
            currency,
        )?;
        
        // Select optimal settlement rail
        let optimal_rail = self.select_optimal_rail(&preferred_rails, amount, currency)?;
        let rail_config = self.rail_configs.get(&optimal_rail)
            .ok_or(XTMPPayError::UnsupportedRail)?;
        
        // Calculate fees
        let fees = rail_config.base_fee + (amount * rail_config.percentage_fee as f64 / 100.0);
        
        // Create payment proof
        let payment_data = format!("{}{}{}{}", 
            payment_id, sender.wallet_address, recipient_address, amount);
        let signature = sender.sign_message(payment_data.as_bytes());
        
        let payment_proof = PaymentProof {
            signature: signature.to_bytes().to_vec(),
            witness: None, // TODO: Generate ZK proof
            compliance: ComplianceCheck {
                status: ComplianceStatus::Approved,
                jurisdiction: "US".to_string(),
                tax_implications: None,
                regulatory_flags: Vec::new(),
            },
            kyc_aml: None, // TODO: Perform KYC/AML check
        };
        
        // Create settlement config
        let settlement = SettlementConfig {
            rail: optimal_rail,
            estimated_time: rail_config.settlement_time.clone(),
            fees,
            fee_currency: currency.to_string(),
            priority: SettlementPriority::Standard,
        };
        
        let payment = XTMPPayment {
            payment_id: payment_id.clone(),
            from_wallet: sender.wallet_address.clone(),
            to_wallet: recipient_address.to_string(),
            amount: PaymentAmount {
                value: amount,
                currency: currency.to_string(),
                rails: preferred_rails,
            },
            payment_proof,
            settlement,
            status: PaymentStatus::Created,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            bpi_receipt: None,
        };
        
        self.payments.insert(payment_id.clone(), payment);
        Ok(payment_id)
    }
    
    /// Process payment through settlement rails
    pub fn process_payment(&mut self, payment_id: &str) -> Result<(), XTMPPayError> {
        let payment = self.payments.get_mut(payment_id)
            .ok_or(XTMPPayError::PaymentNotFound)?;
        
        payment.status = PaymentStatus::Processing;
        payment.updated_at = Utc::now();
        
        // Route to appropriate settlement rail
        match &payment.settlement.rail {
            SettlementRail::BPI => {
                self.process_bpi_native_payment(payment_id)?;
            },
            SettlementRail::ACH => {
                self.process_ach_payment(payment_id)?;
            },
            SettlementRail::SEPA => {
                self.process_sepa_payment(payment_id)?;
            },
            SettlementRail::INTERAC => {
                self.process_interac_payment(payment_id)?;
            },
            SettlementRail::RTP => {
                self.process_rtp_payment(payment_id)?;
            },
            SettlementRail::SWIFT => {
                self.process_swift_payment(payment_id)?;
            },
            SettlementRail::Crypto => {
                self.process_crypto_payment(payment_id)?;
            },
            SettlementRail::Custom(name) => {
                // Process custom payment logic here
            // self.process_custom_payment(payment_id, name)?;
            },
        }
        
        Ok(())
    }
    
    /// Get payment status
    pub fn get_payment(&self, payment_id: &str) -> Option<&XTMPPayment> {
        self.payments.get(payment_id)
    }
    
    /// Get payment history for wallet
    pub fn get_payment_history(&self, wallet_address: &str) -> Vec<&XTMPPayment> {
        self.payments.values()
            .filter(|payment| {
                payment.from_wallet == wallet_address || payment.to_wallet == wallet_address
            })
            .collect()
    }
    
    /// Cancel a pending payment
    pub fn cancel_payment(&mut self, payment_id: &str) -> Result<(), XTMPPayError> {
        let payment = self.payments.get_mut(payment_id)
            .ok_or(XTMPPayError::PaymentNotFound)?;
        
        match payment.status {
            PaymentStatus::Created | PaymentStatus::Pending => {
                payment.status = PaymentStatus::Cancelled;
                payment.updated_at = Utc::now();
                Ok(())
            },
            _ => Err(XTMPPayError::PaymentCannotBeCancelled),
        }
    }
    
    /// Select optimal settlement rail
    fn select_optimal_rail(
        &self,
        preferred_rails: &[SettlementRail],
        amount: f64,
        currency: &str,
    ) -> Result<SettlementRail, XTMPPayError> {
        let mut best_rail = None;
        let mut best_score = f64::MIN;
        
        for rail in preferred_rails {
            if let Some(config) = self.rail_configs.get(rail) {
                if config.supported_currencies.contains(&currency.to_string()) &&
                   amount >= config.min_amount && amount <= config.max_amount {
                    
                    // Calculate score based on fees and speed
                    let fee_cost = config.base_fee + (amount * config.percentage_fee as f64 / 100.0);
                    let speed_score = match config.settlement_time.as_str() {
                        "instant" => 100.0,
                        "2-5_minutes" => 90.0,
                        "1_hour" => 70.0,
                        "1_day" => 50.0,
                        _ => 30.0,
                    };
                    
                    let score = speed_score - (fee_cost / amount * 100.0);
                    
                    if score > best_score {
                        best_score = score;
                        best_rail = Some(rail.clone());
                    }
                }
            }
        }
        
        best_rail.ok_or(XTMPPayError::NoSuitableRail)
    }
    
    /// Initialize settlement rail configurations
    fn initialize_rail_configs(&mut self) {
        // ACH Configuration
        self.rail_configs.insert(SettlementRail::ACH, RailConfig {
            name: "ACH".to_string(),
            base_fee: 0.25,
            percentage_fee: 0.1,
            min_amount: 1.0,
            max_amount: 1000000.0,
            settlement_time: "1_day".to_string(),
            supported_currencies: vec!["USD".to_string()],
            requires_kyc: true,
        });
        
        // SEPA Configuration
        self.rail_configs.insert(SettlementRail::SEPA, RailConfig {
            name: "SEPA".to_string(),
            base_fee: 0.50,
            percentage_fee: 0.2,
            min_amount: 1.0,
            max_amount: 1000000.0,
            settlement_time: "1_day".to_string(),
            supported_currencies: vec!["EUR".to_string()],
            requires_kyc: true,
        });
        
        // INTERAC Configuration
        self.rail_configs.insert(SettlementRail::INTERAC, RailConfig {
            name: "INTERAC".to_string(),
            base_fee: 1.00,
            percentage_fee: 0.5,
            min_amount: 0.50,
            max_amount: 3000.0,
            settlement_time: "2-5_minutes".to_string(),
            supported_currencies: vec!["CAD".to_string()],
            requires_kyc: false,
        });
        
        // RTP Configuration
        self.rail_configs.insert(SettlementRail::RTP, RailConfig {
            name: "RTP".to_string(),
            base_fee: 0.10,
            percentage_fee: 0.05,
            min_amount: 0.01,
            max_amount: 100000.0,
            settlement_time: "instant".to_string(),
            supported_currencies: vec!["USD".to_string()],
            requires_kyc: false,
        });
        
        // BPI Native Configuration
        self.rail_configs.insert(SettlementRail::BPI, RailConfig {
            name: "BPI Native".to_string(),
            base_fee: 0.001,
            percentage_fee: 0.01,
            min_amount: 0.000001,
            max_amount: f64::MAX,
            settlement_time: "instant".to_string(),
            supported_currencies: vec!["USD".to_string(), "EUR".to_string(), "CAD".to_string(), "BPI".to_string()],
            requires_kyc: false,
        });
    }
    
    /// Initialize payment flows
    fn initialize_payment_flows(&mut self) {
        // Pravyom to Bank flow
        self.flows.insert(PaymentFlowType::PravyomToBank, PaymentFlow {
            flow_type: PaymentFlowType::PravyomToBank,
            participants: vec!["pravyom_wallet".to_string(), "bank_account".to_string()],
            steps: vec![
                PaymentStep {
                    step_number: 1,
                    description: "Wallet authorization".to_string(),
                    estimated_time: "30_seconds".to_string(),
                    status: StepStatus::Pending,
                },
                PaymentStep {
                    step_number: 2,
                    description: "Bank rail settlement".to_string(),
                    estimated_time: "2-5_minutes".to_string(),
                    status: StepStatus::Pending,
                },
                PaymentStep {
                    step_number: 3,
                    description: "BPI receipt generation".to_string(),
                    estimated_time: "30_seconds".to_string(),
                    status: StepStatus::Pending,
                },
            ],
            total_time: "3-6_minutes".to_string(),
            total_fees: 0.50,
        });
        
        // Add other flow types...
    }
    
    // Settlement rail processors
    fn process_bpi_native_payment(&mut self, payment_id: &str) -> Result<(), XTMPPayError> {
        println!("Processing BPI native payment: {}", payment_id);
        // TODO: Implement BPI ledger settlement
        self.complete_payment(payment_id)
    }
    
    fn process_ach_payment(&mut self, payment_id: &str) -> Result<(), XTMPPayError> {
        println!("Processing ACH payment: {}", payment_id);
        // TODO: Implement ACH settlement
        self.complete_payment(payment_id)
    }
    
    fn process_sepa_payment(&mut self, payment_id: &str) -> Result<(), XTMPPayError> {
        println!("Processing SEPA payment: {}", payment_id);
        // TODO: Implement SEPA settlement
        self.complete_payment(payment_id)
    }
    
    fn process_interac_payment(&mut self, payment_id: &str) -> Result<(), XTMPPayError> {
        println!("Processing INTERAC payment: {}", payment_id);
        // TODO: Implement INTERAC settlement
        self.complete_payment(payment_id)
    }
    
    fn process_rtp_payment(&mut self, payment_id: &str) -> Result<(), XTMPPayError> {
        println!("Processing RTP payment: {}", payment_id);
        // Simulate RTP payment processing
        Ok(())
    }
    
    fn process_swift_payment(&mut self, payment_id: &str) -> Result<(), XTMPPayError> {
        println!("Processing SWIFT payment: {}", payment_id);
        // Simulate SWIFT payment processing
        Ok(())
    }

    fn process_crypto_payment(&mut self, payment_id: &str) -> Result<(), XTMPPayError> {
        println!("Processing crypto payment: {}", payment_id);
        // Simulate cryptocurrency payment processing
        Ok(())
    }
    
    fn process_custom_payment(&mut self, payment_id: &str, rail_name: &str) -> Result<(), XTMPPayError> {
        println!("Processing custom payment via {}: {}", rail_name, payment_id);
        // TODO: Implement custom rail settlement
        self.complete_payment(payment_id)
    }
    
    fn complete_payment(&mut self, payment_id: &str) -> Result<(), XTMPPayError> {
        let payment = self.payments.get_mut(payment_id)
            .ok_or(XTMPPayError::PaymentNotFound)?;
        
        payment.status = PaymentStatus::Settled;
        payment.updated_at = Utc::now();
        payment.bpi_receipt = Some(format!("0x{}", hex::encode(&rand::random::<[u8; 32]>())));
        
        Ok(())
    }
}

impl ComplianceEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            jurisdiction_rules: HashMap::new(),
            sanctions_list: Vec::new(),
            pep_database: Vec::new(),
        };
        
        // Initialize jurisdiction rules
        engine.initialize_jurisdiction_rules();
        engine
    }
    
    fn initialize_jurisdiction_rules(&mut self) {
        // US rules
        self.jurisdiction_rules.insert("US".to_string(), JurisdictionRules {
            max_transaction_amount: 10000.0,
            requires_kyc_above: 3000.0,
            tax_rate: 0.0, // No tax on transfers
            reporting_threshold: 10000.0,
            restricted_countries: vec!["OFAC_SANCTIONED".to_string()],
        });
        
        // EU rules
        self.jurisdiction_rules.insert("EU".to_string(), JurisdictionRules {
            max_transaction_amount: 15000.0,
            requires_kyc_above: 1000.0,
            tax_rate: 0.0,
            reporting_threshold: 15000.0,
            restricted_countries: vec!["EU_SANCTIONED".to_string()],
        });
    }

    /// Check payment compliance
    pub fn check_payment_compliance(
        &self,
        sender: &str,
        recipient: &str,
        amount: f64,
        currency: &str,
    ) -> Result<()> {
        // Check amount limits
        if amount > 50000.0 {
            return Err(anyhow!("Amount exceeds maximum limit"));
        }

        // Check sanctions list
        if self.sanctions_list.contains(&sender.to_string()) || 
           self.sanctions_list.contains(&recipient.to_string()) {
            return Err(anyhow!("Transaction blocked: sanctioned party"));
        }

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum XTMPPayError {
    #[error("Payment not found")]
    PaymentNotFound,
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Unsupported settlement rail")]
    UnsupportedRail,
    #[error("No suitable settlement rail found")]
    NoSuitableRail,
    #[error("Compliance check failed")]
    ComplianceFailed,
    #[error("Payment cannot be cancelled")]
    PaymentCannotBeCancelled,
    #[error("Settlement failed")]
    SettlementFailed,
    #[error("Invalid amount")]
    InvalidAmount,
    #[error("Currency not supported")]
    CurrencyNotSupported,
    #[error("Anyhow error: {0}")]
    AnyhowError(String),
}

impl From<anyhow::Error> for XTMPPayError {
    fn from(err: anyhow::Error) -> Self {
        XTMPPayError::AnyhowError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wallet_identity::{WalletIdentity, WalletProvider, WalletCapability, VerificationLevel};
    use crate::identity_registry::IdentityRegistry;

    #[test]
    fn test_payment_creation() {
        let alice = WalletIdentity::new_with_capabilities(
            "alice",
            WalletProvider::Pravyom,
            Some("alice@gmail.com".to_string()),
            vec![WalletCapability::BasicWallet, WalletCapability::PaymentProcessing],
            VerificationLevel::Email,
        ).unwrap();
        let registry = IdentityRegistry::new();
        let mut service = XTMPPayService::new(alice.clone(), registry).unwrap();
        
        // Ensure service has supported rails
        assert!(!service.supported_rails.is_empty());
        
        let payment_id = service.create_payment(
            &alice,
            "bob@metamail.wallet",
            100.0,
            "USD",
            service.supported_rails.clone(),
        ).unwrap();
        
        assert!(!payment_id.is_empty());
        assert!(payment_id.starts_with("pay_"));
        
        let payment = service.get_payment(&payment_id).unwrap();
        assert_eq!(payment.amount.value, 100.0);
        assert_eq!(payment.amount.currency, "USD");
        assert_eq!(payment.status, PaymentStatus::Created);
    }
    
    #[test]
    fn test_rail_selection() {
        let alice = WalletIdentity::new_with_capabilities(
            "alice",
            WalletProvider::Pravyom,
            Some("alice@example.com".to_string()),
            vec![WalletCapability::BasicWallet, WalletCapability::PaymentProcessing],
            VerificationLevel::Email,
        ).unwrap();
        let registry = IdentityRegistry::new();
        let service = XTMPPayService::new(alice, registry).unwrap();
        
        let selected_rail = service.select_optimal_rail(&service.supported_rails, 100.0, "USD").unwrap();
        
        // BPI should be selected for optimal speed and low fees for USD
        // BPI has instant settlement and lowest fees (0.001 base + 0.01%)
        assert!(matches!(selected_rail, SettlementRail::BPI));
    }
    
    #[test]
    fn test_payment_processing() {
        let alice = WalletIdentity::new_with_capabilities(
            "alice",
            WalletProvider::Pravyom,
            Some("alice@example.com".to_string()),
            vec![WalletCapability::BasicWallet, WalletCapability::PaymentProcessing],
            VerificationLevel::Email,
        ).unwrap();
        let registry = IdentityRegistry::new();
        let mut service = XTMPPayService::new(alice.clone(), registry).unwrap();
        
        let alice = WalletIdentity::new(
            "alice",
            WalletProvider::Pravyom,
            None,
        ).unwrap();
        
        let payment_id = service.create_payment(
            &alice,
            "bob@metamail.wallet",
            50.0,
            "USD",
            service.supported_rails.clone(),
        ).unwrap();
        
        // service is already unwrapped from XTMPPayService::new() call above
        service.process_payment(&payment_id).unwrap();
        
        let payment = service.get_payment(&payment_id).unwrap();
        assert_eq!(payment.status, PaymentStatus::Settled);
        assert!(payment.bpi_receipt.is_some());
    }
}
