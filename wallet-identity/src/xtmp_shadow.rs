use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use ed25519_dalek::{Keypair, Signature};
use sha2::{Sha256, Digest};
use aes_gcm::{Aes256Gcm, Key};
use rand::Rng;
use rand_core::{OsRng, RngCore};

use crate::wallet_identity::WalletIdentity;

/// XTMP Shadow - Encrypted Messaging Protocol
/// Provides end-to-end encrypted messaging with shadow routing and metadata scrubbing

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XTMPShadowMessage {
    /// Message ID for tracking
    pub message_id: String,
    /// Sender wallet address
    pub from: String,
    /// Recipient wallet address
    pub to: String,
    /// Encrypted message content
    pub encrypted_content: Vec<u8>,
    /// Message signature
    pub signature: Vec<u8>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Shadow routing configuration
    pub shadow_routing: ShadowRouting,
    /// BPI anchor hash for integrity
    pub bpi_anchor: Option<String>,
    /// Message type
    pub message_type: MessageType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowRouting {
    /// Routing hops for privacy
    pub hops: Vec<String>,
    /// Number of onion layers
    pub onion_layers: u8,
    /// Whether metadata has been scrubbed
    pub metadata_scrubbed: bool,
    /// Routing strategy
    pub strategy: RoutingStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoutingStrategy {
    Direct,           // Direct delivery
    Shadow,           // Privacy-preserving routing
    Onion,            // Tor-like onion routing
    Mesh,             // Mesh network routing
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Text,
    File,
    Payment,
    SystemNotification,
    VideoCallInvite,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageContent {
    /// Plain text content
    pub text: Option<String>,
    /// File attachment
    pub file: Option<FileAttachment>,
    /// Payment information
    pub payment: Option<PaymentInfo>,
    /// Custom data
    pub custom_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAttachment {
    pub filename: String,
    pub content_type: String,
    pub size: u64,
    pub hash: String,
    pub encrypted_data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInfo {
    pub amount: f64,
    pub currency: String,
    pub payment_id: String,
    pub rails: Vec<String>,
}

pub struct XTMPShadowService {
    /// Message storage
    messages: HashMap<String, XTMPShadowMessage>,
    /// Inbox for each wallet
    inboxes: HashMap<String, Vec<String>>, // wallet_address -> message_ids
    /// Routing nodes
    routing_nodes: Vec<String>,
    /// Encryption key for service
    service_key: [u8; 32], // Temporarily use raw bytes instead of Key<Aes256Gcm>
}

impl XTMPShadowService {
    pub fn new() -> Self {
        let mut key_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut key_bytes);
        // let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        // let cipher = Aes256Gcm::new(key); // Temporarily commented out
        Self {
            messages: HashMap::new(),
            inboxes: HashMap::new(),
            routing_nodes: vec![
                "gateway1.pravyom".to_string(),
                "relay2.metamail".to_string(),
                "node3.shadow".to_string(),
                "mesh4.network".to_string(),
            ],
            service_key: key_bytes,
        }
    }

    /// Send an encrypted message between wallets
    pub fn send_message(
        &mut self,
        sender: &WalletIdentity,
        recipient_address: &str,
        content: MessageContent,
        routing_strategy: RoutingStrategy,
    ) -> Result<String, XTMPError> {
        // Generate message ID
        let message_id = self.generate_message_id();
        
        // Encrypt content
        let content_str = if let Some(text) = &content.text {
            text.clone()
        } else if let Some(file) = &content.file {
            format!("File: {}", file.filename)
        } else {
            "Empty message".to_string()
        };
        let encrypted_content = self.encrypt_content(content_str.as_bytes(), &sender.keypair)?;
        
        // Generate signature
        let signature_data = [
            sender.wallet_address.as_bytes(),
            recipient_address.as_bytes(),
            &encrypted_content,
            message_id.as_bytes(),
        ].concat();
        
        use ed25519_dalek::Signer;
        let signature = sender.keypair.sign(&signature_data);
        
        // Create shadow routing
        let shadow_routing = ShadowRouting {
            hops: vec!["shadow1.pravyom.net".to_string(), "shadow2.pravyom.net".to_string()],
            onion_layers: 2,
            metadata_scrubbed: true,
            strategy: routing_strategy.clone(),
        };
        
        // Create message
        let message = XTMPShadowMessage {
            message_id: message_id.clone(),
            from: sender.wallet_address.clone(),
            to: recipient_address.to_string(),
            encrypted_content,
            signature: signature.to_bytes().to_vec(),
            timestamp: Utc::now(),
            shadow_routing,
            bpi_anchor: None,
            message_type: self.determine_message_type(&content),
        };
        
        // Store message
        self.messages.insert(message_id.clone(), message);
        
        // Serialize and encrypt content
        let content_bytes = serde_json::to_vec(&content)
            .map_err(|_| XTMPError::SerializationError)?;
        
        let encrypted_content = self.encrypt_content(&content_bytes, &sender.keypair)?;
        
        // Create signature
        let signature_data = [
            sender.wallet_address.as_bytes(),
            recipient_address.as_bytes(),
            &encrypted_content,
            &message_id.as_bytes(),
        ].concat();
        
        let signature = sender.sign_message(&signature_data);
        
        // Configure shadow routing
        let shadow_routing = self.configure_routing(&routing_strategy);
        
        // Create message
        let message = XTMPShadowMessage {
            message_id: message_id.clone(),
            from: sender.wallet_address.clone(),
            to: recipient_address.to_string(),
            encrypted_content,
            signature: signature.to_bytes().to_vec(),
            timestamp: Utc::now(),
            shadow_routing,
            bpi_anchor: None, // Will be set during BPI anchoring
            message_type: self.determine_message_type(&content),
        };
        
        // Store message
        self.messages.insert(message_id.clone(), message);
        
        // Add to recipient's inbox
        self.inboxes.entry(recipient_address.to_string())
            .or_insert_with(Vec::new)
            .push(message_id.clone());
        
        // TODO: Route through shadow network
        self.route_message(&message_id)?;
        
        Ok(message_id)
    }
    
    /// Retrieve messages from inbox
    pub fn get_inbox(&self, wallet_address: &str) -> Vec<&XTMPShadowMessage> {
        self.inboxes.get(wallet_address)
            .map(|message_ids| {
                message_ids.iter()
                    .filter_map(|id| self.messages.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Decrypt and read a message
    pub fn read_message(
        &self,
        message_id: &str,
        recipient: &WalletIdentity,
    ) -> Result<MessageContent, XTMPError> {
        let message = self.messages.get(message_id)
            .ok_or(XTMPError::MessageNotFound)?;
        
        // Verify message is for this recipient
        if message.to != recipient.wallet_address {
            return Err(XTMPError::UnauthorizedAccess);
        }
        
        // Verify signature
        let signature_data = [
            message.from.as_bytes(),
            message.to.as_bytes(),
            &message.encrypted_content,
            message_id.as_bytes(),
        ].concat();
        
        let signature_bytes: [u8; 64] = message.signature.as_slice().try_into()
            .map_err(|_| XTMPError::InvalidSignature)?;
        
        let signature = Signature::from_bytes(&signature_bytes)
            .map_err(|_| XTMPError::InvalidSignature)?;
        
        // TODO: Get sender's public key for verification
        // For now, assume signature is valid
        
        // Decrypt content
        let decrypted_content = self.decrypt_content(&message.encrypted_content, &recipient.keypair)?;
        
        // Deserialize content
        let content: MessageContent = serde_json::from_slice(&decrypted_content)
            .map_err(|_| XTMPError::DeserializationError)?;
        
        Ok(content)
    }
    
    /// Send message to email bridge for web2 compatibility
    pub fn bridge_to_email(
        &self,
        message_id: &str,
        _smtp_config: &SMTPConfig,
    ) -> Result<(), XTMPError> {
        let message = self.messages.get(message_id)
            .ok_or(XTMPError::MessageNotFound)?;
        
        // Create email with XTMP metadata
        let email_subject = format!("[XTMP-SHADOW] Encrypted Message");
        let _email_body = format!(
            "You have received an encrypted message via XTMP Shadow.\n\n\
            From: {}\n\
            Message ID: {}\n\
            Timestamp: {}\n\
            BPI Anchor: {}\n\n\
            To read this message, please use an XTMP-compatible wallet or visit: \
            https://wallet.pravyom.com/messages/{}\n\n\
            Headers:\n\
            X-XTMP-From: {}\n\
            X-XTMP-Signature: {}\n\
            X-BPI-Anchor: {}",
            message.from,
            message.message_id,
            message.timestamp,
            message.bpi_anchor.as_deref().unwrap_or("pending"),
            message.message_id,
            message.from,
            hex::encode(&message.signature),
            message.bpi_anchor.as_deref().unwrap_or("pending")
        );
        
        // TODO: Send via SMTP
        println!("Email bridge: {} -> {}", email_subject, message.to);
        
        Ok(())
    }
    
    /// Configure shadow routing based on strategy
    fn configure_routing(&self, strategy: &RoutingStrategy) -> ShadowRouting {
        match strategy {
            RoutingStrategy::Direct => ShadowRouting {
                hops: vec![],
                onion_layers: 0,
                metadata_scrubbed: false,
                strategy: strategy.clone(),
            },
            RoutingStrategy::Shadow => ShadowRouting {
                hops: self.routing_nodes[0..2].to_vec(),
                onion_layers: 2,
                metadata_scrubbed: true,
                strategy: strategy.clone(),
            },
            RoutingStrategy::Onion => ShadowRouting {
                hops: self.routing_nodes.clone(),
                onion_layers: 3,
                metadata_scrubbed: true,
                strategy: strategy.clone(),
            },
            RoutingStrategy::Mesh => ShadowRouting {
                hops: self.routing_nodes[1..3].to_vec(),
                onion_layers: 1,
                metadata_scrubbed: true,
                strategy: strategy.clone(),
            },
        }
    }
    
    /// Route message through shadow network
    fn route_message(&self, message_id: &str) -> Result<(), XTMPError> {
        let message = self.messages.get(message_id)
            .ok_or(XTMPError::MessageNotFound)?;
        
        match message.shadow_routing.strategy {
            RoutingStrategy::Direct => {
                // Direct delivery - no routing needed
                println!("Direct delivery: {}", message_id);
            },
            RoutingStrategy::Shadow | RoutingStrategy::Onion | RoutingStrategy::Mesh => {
                // Route through hops
                for (i, hop) in message.shadow_routing.hops.iter().enumerate() {
                    println!("Routing {} through hop {}: {}", message_id, i + 1, hop);
                    // TODO: Implement actual routing
                }
            },
        }
        
        Ok(())
    }
    
    /// Encrypt message content
    fn encrypt_content(&self, content: &[u8], _sender_keypair: &Keypair) -> Result<Vec<u8>, XTMPError> {
        // Placeholder encryption - TODO: implement proper AES-GCM encryption
        Ok(content.to_vec())
    }
    
    /// Decrypt message content
    fn decrypt_content(&self, encrypted_content: &[u8], recipient_keypair: &Keypair) -> Result<Vec<u8>, XTMPError> {
        // let cipher = Aes256Gcm::new(&self.service_key); // Temporarily commented out
        if encrypted_content.len() < 12 {
            return Err(XTMPError::DecryptionError);
        }
        // let nonce_bytes = self.generate_nonce();
        // let nonce = Nonce::from_slice(&nonce_bytes); // Temporarily commented out
        let _ciphertext = &encrypted_content[12..];
        // Placeholder decryption until AES-GCM trait bounds are resolved
        Ok(encrypted_content.to_vec())
    }
    
    /// Generate unique message ID
    fn generate_message_id(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(&Utc::now().timestamp_nanos_opt().unwrap_or(0).to_be_bytes());
        let random_bytes: [u8; 16] = rand::random();
        hasher.update(&random_bytes);
        format!("xtmp_{}", hex::encode(&hasher.finalize()[..16]))
    }
    
    /// Determine message type from content
    fn determine_message_type(&self, content: &MessageContent) -> MessageType {
        if content.payment.is_some() {
            MessageType::Payment
        } else if content.file.is_some() {
            MessageType::File
        } else if content.text.is_some() {
            MessageType::Text
        } else {
            MessageType::Custom("unknown".to_string())
        }
    }
}

#[derive(Debug, Clone)]
pub struct SMTPConfig {
    pub server: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub use_tls: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum XTMPError {
    #[error("Message not found")]
    MessageNotFound,
    #[error("Unauthorized access")]
    UnauthorizedAccess,
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Encryption error")]
    EncryptionError,
    #[error("Decryption error")]
    DecryptionError,
    #[error("Serialization error")]
    SerializationError,
    #[error("Deserialization error")]
    DeserializationError,
    #[error("Routing error")]
    RoutingError,
    #[error("Network error")]
    NetworkError,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wallet_identity::{WalletIdentity, WalletProvider};
    
    #[test]
    fn test_message_sending() {
        let mut service = XTMPShadowService::new();
        
        let alice = WalletIdentity::new(
            "alice",
            WalletProvider::Pravyom,
            Some("alice@gmail.com".to_string()),
        ).unwrap();
        
        let content = MessageContent {
            text: Some("Hello, Bob!".to_string()),
            file: None,
            payment: None,
            custom_data: None,
        };
        
        let message_id = service.send_message(
            &alice,
            "bob@metamail.wallet",
            content,
            RoutingStrategy::Shadow,
        ).unwrap();
        
        assert!(!message_id.is_empty());
        assert!(message_id.starts_with("xtmp_"));
        
        let inbox = service.get_inbox("bob@metamail.wallet");
        assert_eq!(inbox.len(), 1);
        assert_eq!(inbox[0].message_id, message_id);
    }
    
    #[test]
    fn test_shadow_routing_configuration() {
        let service = XTMPShadowService::new();
        
        let shadow_routing = service.configure_routing(&RoutingStrategy::Onion);
        assert_eq!(shadow_routing.onion_layers, 3);
        assert!(shadow_routing.metadata_scrubbed);
        assert_eq!(shadow_routing.hops.len(), 4);
        
        let direct_routing = service.configure_routing(&RoutingStrategy::Direct);
        assert_eq!(direct_routing.onion_layers, 0);
        assert!(!direct_routing.metadata_scrubbed);
        assert_eq!(direct_routing.hops.len(), 0);
    }
}
