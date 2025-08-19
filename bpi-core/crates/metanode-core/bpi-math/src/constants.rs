//! Mathematical Constants for Blockchain Operations

/// Domain separators for cryptographic operations
pub const POA_DOMAIN: &[u8] = b"PROOF_OF_ACTION";
pub const POE_DOMAIN: &[u8] = b"PROOF_OF_EXECUTION";
pub const POT_DOMAIN: &[u8] = b"PROOF_OF_TRANSACT";
pub const POG_DOMAIN: &[u8] = b"PROOF_OF_GOLD";
pub const POH_DOMAIN: &[u8] = b"PROOF_OF_HISTORY";

/// Category theory constants
pub const LEDGER_CATEGORY_DOMAIN: &[u8] = b"LEDGER_CATEGORY";
pub const MORPHISM_COMPOSITION_DOMAIN: &[u8] = b"MORPHISM_COMPOSITION";

/// Knot theory constants
pub const KNOT_INVARIANT_DOMAIN: &[u8] = b"KNOT_INVARIANT";
pub const ALEXANDER_POLYNOMIAL_DOMAIN: &[u8] = b"ALEXANDER_POLYNOMIAL";

/// Receipt aggregation constants
pub const RECEIPT_AGGREGATION_DOMAIN: &[u8] = b"RECEIPT_AGGREGATION";
pub const DOCKLOCK_RECEIPT_DOMAIN: &[u8] = b"DOCKLOCK_RECEIPT";

/// Mining constants
pub const MINING_PROOF_DOMAIN: &[u8] = b"MINING_PROOF";
pub const MINING_MERKLE_DOMAIN: &[u8] = b"MINING_MERKLE";
pub const MINING_BLOCK_DOMAIN: &[u8] = b"MINING_BLOCK";

/// Receipt domain constants
pub const CLUSTER_RECEIPT_DOMAIN: &[u8] = b"CLUSTER_RECEIPT";
pub const BPI_RECEIPT_DOMAIN: &[u8] = b"BPI_RECEIPT";
pub const BPCI_RECEIPT_DOMAIN: &[u8] = b"BPCI_RECEIPT";
pub const ECONOMY_RECEIPT_DOMAIN: &[u8] = b"ECONOMY_RECEIPT";

/// Mathematical constants
pub const GOLDEN_RATIO: f64 = 1.618033988749895;
pub const EULER_CONSTANT: f64 = std::f64::consts::E;
