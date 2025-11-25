//! Fibonacci-Stability Handshake Protocol Messages
//! 
//! Implements 3-way handshake with retry tokens and witness gossip.
//! 
//! # Protocol Flow
//! 
//! ```text
//! Initiator (i)                    Responder (j)
//!     |                                  |
//!     |------ HELLO₁ (ID, e_i, π) ----->|
//!     |                                  | (validate puzzle, build witness set)
//!     |<----- HELLO₂ (ID, e_j, H) ------|
//!     | (verify HMAC, check witnesses)   |
//!     |------ ACK₃ (H', token) -------->|
//!     |                                  | (issue lease with Fibonacci credits)
//!     |<===== Encrypted Channel =======>|
//! ```

use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

use super::fibonacci_stability::WitnessEndorsement;

/// Node capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeCapabilities {
    /// Maximum connections
    pub max_connections: usize,
    
    /// Supported protocols
    pub protocols: Vec<String>,
    
    /// Service types
    pub services: Vec<String>,
    
    /// Protocol version
    pub version: String,
}

impl Default for NodeCapabilities {
    fn default() -> Self {
        Self {
            max_connections: 100,
            protocols: vec!["quic".to_string(), "tcp".to_string()],
            services: vec!["consensus".to_string(), "storage".to_string()],
            version: "1.0.0".to_string(),
        }
    }
}

/// Proof of Work puzzle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofOfWork {
    /// Challenge nonce
    pub challenge: [u8; 32],
    
    /// Solution nonce
    pub solution: u64,
    
    /// Difficulty (leading zero bits)
    pub difficulty: u8,
}

impl ProofOfWork {
    /// Create a new PoW challenge
    pub fn new_challenge(difficulty: u8) -> Self {
        let mut challenge = [0u8; 32];
        use rand::RngCore;
        rand::thread_rng().fill_bytes(&mut challenge);
        
        Self {
            challenge,
            solution: 0,
            difficulty,
        }
    }
    
    /// Solve the PoW puzzle
    pub fn solve(&mut self) {
        for nonce in 0..u64::MAX {
            if self.verify(nonce) {
                self.solution = nonce;
                return;
            }
        }
    }
    
    /// Verify a PoW solution
    pub fn verify(&self, nonce: u64) -> bool {
        let mut hasher = <Sha256 as Digest>::new();
        hasher.update(b"POW_V1");
        hasher.update(&self.challenge);
        hasher.update(nonce.to_le_bytes());
        let hash = hasher.finalize();
        
        // Check leading zero bits
        let leading_zeros = hash.iter().take_while(|&&b| b == 0).count() * 8;
        leading_zeros >= self.difficulty as usize
    }
}

/// HELLO₁ message (initiator → responder)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hello1 {
    /// Initiator node ID
    pub node_id: String,
    
    /// Ephemeral X25519 public key
    pub ephemeral_key: [u8; 32],
    
    /// Timestamp
    pub timestamp: u64,
    
    /// Random nonce
    pub nonce: [u8; 32],
    
    /// Node capabilities
    pub capabilities: NodeCapabilities,
    
    /// Proof of Work
    pub proof_of_work: ProofOfWork,
}

impl Hello1 {
    /// Create a new HELLO₁ message
    pub fn new(node_id: String, ephemeral_key: [u8; 32], difficulty: u8) -> Self {
        let mut nonce = [0u8; 32];
        use rand::RngCore;
        rand::thread_rng().fill_bytes(&mut nonce);
        
        let mut pow = ProofOfWork::new_challenge(difficulty);
        pow.solve();
        
        Self {
            node_id,
            ephemeral_key,
            timestamp: current_timestamp(),
            nonce,
            capabilities: NodeCapabilities::default(),
            proof_of_work: pow,
        }
    }
}

/// Retry token (stateless)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryToken {
    /// Client address
    pub address: SocketAddr,
    
    /// Time bucket (coarse)
    pub time_bucket: u64,
    
    /// HMAC signature
    pub signature: [u8; 32],
}

impl RetryToken {
    /// Create a new retry token
    pub fn new(address: SocketAddr, secret: &[u8; 32]) -> Self {
        let time_bucket = current_timestamp() / 60; // 1-minute buckets
        
        let mut hasher = <Sha256 as Digest>::new();
        hasher.update(b"RETRY_TOKEN_V1");
        hasher.update(secret);
        hasher.update(address.to_string().as_bytes());
        hasher.update(time_bucket.to_le_bytes());
        let signature = hasher.finalize().into();
        
        Self {
            address,
            time_bucket,
            signature,
        }
    }
    
    /// Verify retry token
    pub fn verify(&self, secret: &[u8; 32]) -> bool {
        let expected = Self::new(self.address, secret);
        
        // Allow current and previous time bucket
        let current_bucket = current_timestamp() / 60;
        if self.time_bucket != current_bucket && self.time_bucket != current_bucket - 1 {
            return false;
        }
        
        self.signature == expected.signature
    }
}

/// HELLO₂ message (responder → initiator)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hello2 {
    /// Responder node ID
    pub node_id: String,
    
    /// Ephemeral X25519 public key
    pub ephemeral_key: [u8; 32],
    
    /// Timestamp
    pub timestamp: u64,
    
    /// Retry token
    pub retry_token: RetryToken,
    
    /// Witness endorsements
    pub witness_set: Vec<WitnessEndorsement>,
    
    /// HMAC of transcript
    pub hmac: [u8; 32],
}

impl Hello2 {
    /// Create a new HELLO₂ message
    pub fn new(
        node_id: String,
        ephemeral_key: [u8; 32],
        retry_token: RetryToken,
        witness_set: Vec<WitnessEndorsement>,
        shared_key: &[u8; 32],
        hello1: &Hello1,
    ) -> Self {
        let hmac = Self::compute_hmac(shared_key, hello1, &node_id, &ephemeral_key);
        
        Self {
            node_id,
            ephemeral_key,
            timestamp: current_timestamp(),
            retry_token,
            witness_set,
            hmac,
        }
    }
    
    /// Compute HMAC of transcript
    fn compute_hmac(
        key: &[u8; 32],
        hello1: &Hello1,
        responder_id: &str,
        responder_key: &[u8; 32],
    ) -> [u8; 32] {
        let mut hasher = <Sha256 as Digest>::new();
        hasher.update(b"HANDSHAKE_HMAC_V1");
        hasher.update(key);
        hasher.update(hello1.node_id.as_bytes());
        hasher.update(&hello1.ephemeral_key);
        hasher.update(hello1.timestamp.to_le_bytes());
        hasher.update(&hello1.nonce);
        hasher.update(responder_id.as_bytes());
        hasher.update(responder_key);
        hasher.finalize().into()
    }
    
    /// Verify HMAC
    pub fn verify_hmac(&self, key: &[u8; 32], hello1: &Hello1) -> bool {
        let expected = Self::compute_hmac(key, hello1, &self.node_id, &self.ephemeral_key);
        self.hmac == expected
    }
}

/// ACK₃ message (initiator → responder)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ack3 {
    /// HMAC confirmation
    pub hmac: [u8; 32],
    
    /// Echo retry token
    pub retry_token: RetryToken,
}

impl Ack3 {
    /// Create a new ACK₃ message
    pub fn new(shared_key: &[u8; 32], hello1: &Hello1, hello2: &Hello2) -> Self {
        let mut hasher = <Sha256 as Digest>::new();
        hasher.update(b"ACK3_HMAC_V1");
        hasher.update(shared_key);
        hasher.update(&hello2.hmac);
        hasher.update(hello2.timestamp.to_le_bytes());
        let hmac = hasher.finalize().into();
        
        Self {
            hmac,
            retry_token: hello2.retry_token.clone(),
        }
    }
    
    /// Verify ACK₃
    pub fn verify(&self, shared_key: &[u8; 32], hello1: &Hello1, hello2: &Hello2) -> bool {
        let expected = Self::new(shared_key, hello1, hello2);
        self.hmac == expected.hmac
    }
}

/// Handshake message envelope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HandshakeMessage {
    /// HELLO₁ from initiator
    Hello1(Hello1),
    
    /// HELLO₂ from responder
    Hello2(Hello2),
    
    /// ACK₃ from initiator
    Ack3(Ack3),
    
    /// RETRY from responder
    Retry(RetryToken),
}

/// Lease granted after successful handshake
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lease {
    /// Lease ID
    pub lease_id: String,
    
    /// Node ID
    pub node_id: String,
    
    /// Duration (Fibonacci seconds)
    pub duration: u64,
    
    /// Resource credits
    pub credits: f64,
    
    /// Issued at
    pub issued_at: u64,
    
    /// Expires at
    pub expires_at: u64,
}

impl Lease {
    /// Create a new lease
    pub fn new(node_id: String, duration_secs: u64, credits: f64) -> Self {
        let issued_at = current_timestamp();
        let expires_at = issued_at + duration_secs;
        
        let lease_id = format!("{}_{}", node_id, issued_at);
        
        Self {
            lease_id,
            node_id,
            duration: duration_secs,
            credits,
            issued_at,
            expires_at,
        }
    }
    
    /// Check if lease is expired
    pub fn is_expired(&self) -> bool {
        current_timestamp() >= self.expires_at
    }
    
    /// Remaining time
    pub fn remaining(&self) -> u64 {
        let now = current_timestamp();
        if now >= self.expires_at {
            0
        } else {
            self.expires_at - now
        }
    }
}

/// Get current Unix timestamp
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_proof_of_work() {
        let mut pow = ProofOfWork::new_challenge(8); // 8-bit difficulty
        pow.solve();
        
        assert!(pow.verify(pow.solution));
    }
    
    #[test]
    fn test_retry_token() {
        let secret = [42u8; 32];
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        
        let token = RetryToken::new(addr, &secret);
        assert!(token.verify(&secret));
        
        let wrong_secret = [99u8; 32];
        assert!(!token.verify(&wrong_secret));
    }
    
    #[test]
    fn test_hello1_creation() {
        let node_id = "node1".to_string();
        let ephemeral_key = [1u8; 32];
        
        let hello1 = Hello1::new(node_id.clone(), ephemeral_key, 8);
        
        assert_eq!(hello1.node_id, node_id);
        assert_eq!(hello1.ephemeral_key, ephemeral_key);
        assert!(hello1.proof_of_work.verify(hello1.proof_of_work.solution));
    }
    
    #[test]
    fn test_hello2_hmac() {
        let hello1 = Hello1::new("node1".to_string(), [1u8; 32], 8);
        let shared_key = [42u8; 32];
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let retry_token = RetryToken::new(addr, &[99u8; 32]);
        
        let hello2 = Hello2::new(
            "node2".to_string(),
            [2u8; 32],
            retry_token,
            vec![],
            &shared_key,
            &hello1,
        );
        
        assert!(hello2.verify_hmac(&shared_key, &hello1));
        
        let wrong_key = [99u8; 32];
        assert!(!hello2.verify_hmac(&wrong_key, &hello1));
    }
    
    #[test]
    fn test_ack3_verification() {
        let hello1 = Hello1::new("node1".to_string(), [1u8; 32], 8);
        let shared_key = [42u8; 32];
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let retry_token = RetryToken::new(addr, &[99u8; 32]);
        
        let hello2 = Hello2::new(
            "node2".to_string(),
            [2u8; 32],
            retry_token,
            vec![],
            &shared_key,
            &hello1,
        );
        
        let ack3 = Ack3::new(&shared_key, &hello1, &hello2);
        
        assert!(ack3.verify(&shared_key, &hello1, &hello2));
    }
    
    #[test]
    fn test_lease_expiration() {
        let lease = Lease::new("node1".to_string(), 10, 100.0);
        
        assert!(!lease.is_expired());
        assert!(lease.remaining() > 0);
    }
    
    #[test]
    fn test_handshake_message_serialization() {
        let hello1 = Hello1::new("node1".to_string(), [1u8; 32], 8);
        let msg = HandshakeMessage::Hello1(hello1);
        
        let json = serde_json::to_string(&msg).unwrap();
        let msg2: HandshakeMessage = serde_json::from_str(&json).unwrap();
        
        match (msg, msg2) {
            (HandshakeMessage::Hello1(h1), HandshakeMessage::Hello1(h2)) => {
                assert_eq!(h1.node_id, h2.node_id);
            }
            _ => panic!("Deserialization failed"),
        }
    }
}
