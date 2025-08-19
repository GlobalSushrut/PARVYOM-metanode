// Shared Protocol Definitions
// Core protocol definitions shared between BPI Core and BPCI Enterprise

//! # Protocols
//! 
//! Core protocol definitions providing consistent communication
//! and data structures across both community and enterprise products.

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
    #[error("Invalid block: {0}")]
    InvalidBlock(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Protocol version mismatch: expected {expected}, got {actual}")]
    VersionMismatch { expected: u32, actual: u32 },
    #[error("Invalid signature")]
    InvalidSignature,
}

/// Protocol version
pub const PROTOCOL_VERSION: u32 = 1;

/// Transaction types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    Transfer,
    ContractCall,
    ContractDeploy,
    Stake,
    Unstake,
    Vote,
    Proposal,
}

/// Transaction structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub transaction_type: TransactionType,
    pub from: String,
    pub to: Option<String>,
    pub amount: Decimal,
    pub fee: Decimal,
    pub data: Vec<u8>,
    pub nonce: u64,
    pub timestamp: DateTime<Utc>,
    pub signature: Vec<u8>,
}

impl Transaction {
    pub fn new(
        transaction_type: TransactionType,
        from: String,
        to: Option<String>,
        amount: Decimal,
        fee: Decimal,
        data: Vec<u8>,
        nonce: u64,
    ) -> Self {
        Transaction {
            id: Uuid::new_v4(),
            transaction_type,
            from,
            to,
            amount,
            fee,
            data,
            nonce,
            timestamp: Utc::now(),
            signature: Vec::new(),
        }
    }

    pub fn hash(&self) -> String {
        // Simple hash implementation - in production would use proper cryptographic hash
        format!("{:?}", self)
    }

    pub fn sign(&mut self, signature: Vec<u8>) {
        self.signature = signature;
    }

    pub fn verify_signature(&self) -> bool {
        // Simplified signature verification
        !self.signature.is_empty()
    }
}

/// Block header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub version: u32,
    pub previous_hash: String,
    pub merkle_root: String,
    pub timestamp: DateTime<Utc>,
    pub height: u64,
    pub nonce: u64,
    pub difficulty: u32,
}

/// Block structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub hash: String,
}

impl Block {
    pub fn new(
        previous_hash: String,
        transactions: Vec<Transaction>,
        height: u64,
        difficulty: u32,
    ) -> Self {
        let header = BlockHeader {
            version: PROTOCOL_VERSION,
            previous_hash,
            merkle_root: Self::calculate_merkle_root(&transactions),
            timestamp: Utc::now(),
            height,
            nonce: 0,
            difficulty,
        };

        let hash = Self::calculate_hash(&header);

        Block {
            header,
            transactions,
            hash,
        }
    }

    fn calculate_merkle_root(transactions: &[Transaction]) -> String {
        // Simplified merkle root calculation
        if transactions.is_empty() {
            return "0".repeat(64);
        }
        
        let tx_hashes: Vec<String> = transactions.iter().map(|tx| tx.hash()).collect();
        tx_hashes.join("")
    }

    fn calculate_hash(header: &BlockHeader) -> String {
        // Simplified hash calculation
        format!("{:?}", header)
    }

    pub fn validate(&self) -> Result<(), ProtocolError> {
        // Validate protocol version
        if self.header.version != PROTOCOL_VERSION {
            return Err(ProtocolError::VersionMismatch {
                expected: PROTOCOL_VERSION,
                actual: self.header.version,
            });
        }

        // Validate transactions
        for tx in &self.transactions {
            if !tx.verify_signature() {
                return Err(ProtocolError::InvalidSignature);
            }
        }

        Ok(())
    }
}

/// Consensus message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusMessage {
    Propose {
        block: Block,
        proposer: String,
        round: u64,
    },
    Vote {
        block_hash: String,
        voter: String,
        round: u64,
        vote_type: VoteType,
    },
    Commit {
        block_hash: String,
        round: u64,
        signatures: Vec<Vec<u8>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoteType {
    Prevote,
    Precommit,
}

/// Network protocol message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolMessage {
    pub version: u32,
    pub message_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub sender: String,
    pub payload: MessagePayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessagePayload {
    Transaction(Transaction),
    Block(Block),
    Consensus(ConsensusMessage),
    Ping,
    Pong,
    PeerDiscovery {
        peers: Vec<String>,
    },
}

impl ProtocolMessage {
    pub fn new(sender: String, payload: MessagePayload) -> Self {
        ProtocolMessage {
            version: PROTOCOL_VERSION,
            message_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            sender,
            payload,
        }
    }

    pub fn validate(&self) -> Result<(), ProtocolError> {
        if self.version != PROTOCOL_VERSION {
            return Err(ProtocolError::VersionMismatch {
                expected: PROTOCOL_VERSION,
                actual: self.version,
            });
        }

        match &self.payload {
            MessagePayload::Block(block) => block.validate()?,
            MessagePayload::Transaction(tx) => {
                if !tx.verify_signature() {
                    return Err(ProtocolError::InvalidSignature);
                }
            }
            _ => {} // Other message types don't need validation
        }

        Ok(())
    }
}

/// Account state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountState {
    pub address: String,
    pub balance: Decimal,
    pub nonce: u64,
    pub code: Option<Vec<u8>>,
    pub storage: std::collections::HashMap<String, Vec<u8>>,
}

impl AccountState {
    pub fn new(address: String) -> Self {
        AccountState {
            address,
            balance: Decimal::ZERO,
            nonce: 0,
            code: None,
            storage: std::collections::HashMap::new(),
        }
    }

    pub fn transfer(&mut self, amount: Decimal) -> Result<(), ProtocolError> {
        if self.balance < amount {
            return Err(ProtocolError::InvalidTransaction(
                "Insufficient balance".to_string()
            ));
        }
        self.balance -= amount;
        Ok(())
    }

    pub fn receive(&mut self, amount: Decimal) {
        self.balance += amount;
    }

    pub fn increment_nonce(&mut self) {
        self.nonce += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        let tx = Transaction::new(
            TransactionType::Transfer,
            "alice".to_string(),
            Some("bob".to_string()),
            Decimal::from(100),
            Decimal::from(1),
            vec![],
            1,
        );

        assert_eq!(tx.transaction_type, TransactionType::Transfer);
        assert_eq!(tx.from, "alice");
        assert_eq!(tx.to, Some("bob".to_string()));
        assert_eq!(tx.amount, Decimal::from(100));
        assert_eq!(tx.nonce, 1);
    }

    #[test]
    fn test_block_creation() {
        let transactions = vec![
            Transaction::new(
                TransactionType::Transfer,
                "alice".to_string(),
                Some("bob".to_string()),
                Decimal::from(50),
                Decimal::from(1),
                vec![],
                1,
            ),
        ];

        let block = Block::new(
            "previous_hash".to_string(),
            transactions,
            1,
            1000,
        );

        assert_eq!(block.header.height, 1);
        assert_eq!(block.header.difficulty, 1000);
        assert_eq!(block.transactions.len(), 1);
    }

    #[test]
    fn test_protocol_message() {
        let tx = Transaction::new(
            TransactionType::Transfer,
            "alice".to_string(),
            Some("bob".to_string()),
            Decimal::from(100),
            Decimal::from(1),
            vec![],
            1,
        );

        let message = ProtocolMessage::new(
            "node1".to_string(),
            MessagePayload::Transaction(tx),
        );

        assert_eq!(message.version, PROTOCOL_VERSION);
        assert_eq!(message.sender, "node1");
    }

    #[test]
    fn test_account_state() {
        let mut account = AccountState::new("alice".to_string());
        
        account.receive(Decimal::from(100));
        assert_eq!(account.balance, Decimal::from(100));
        
        account.transfer(Decimal::from(50)).unwrap();
        assert_eq!(account.balance, Decimal::from(50));
        
        let result = account.transfer(Decimal::from(100));
        assert!(result.is_err());
    }
}
