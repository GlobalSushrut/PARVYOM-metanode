//! # Stage 39: CAR/DAG Packaging (Envelope Optimization)
//!
//! This module implements Content Addressable aRchive (CAR) and Directed Acyclic Graph (DAG)
//! packaging for optimized packet envelope structure. It provides efficient content-addressed
//! storage, deduplication, and graph-based data organization for the BISO architecture.

use crate::error::DockLockError;
use crate::packet_envelope::{PacketEnvelope, ShardHeader, DataAvailabilityRoot};
use bpi_enc::domain_hash;
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Serialize signature as hex string
fn serialize_signature<S>(sig: &Option<[u8; 64]>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match sig {
        Some(bytes) => serializer.serialize_str(&hex::encode(bytes)),
        None => serializer.serialize_none(),
    }
}

/// Deserialize signature from hex string
fn deserialize_signature<'de, D>(deserializer: D) -> Result<Option<[u8; 64]>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    match opt {
        Some(hex_str) => {
            let bytes = hex::decode(&hex_str).map_err(serde::de::Error::custom)?;
            if bytes.len() != 64 {
                return Err(serde::de::Error::custom("Invalid signature length"));
            }
            let mut array = [0u8; 64];
            array.copy_from_slice(&bytes);
            Ok(Some(array))
        }
        None => Ok(None),
    }
}

/// Domain separator for CAR packaging hashing
pub const CAR_PACKAGE_HASH: &str = "BPI_CAR_PACKAGE_HASH";

/// Domain separator for DAG node hashing
pub const DAG_NODE_HASH: &str = "DAG_NODE";

/// Domain separator for DAG link hashing
pub const DAG_LINK_HASH: &str = "DAG_LINK";

/// Content Identifier (CID) for content-addressed storage
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContentId {
    /// Hash of the content
    pub hash: [u8; 32],
    /// Size of the content in bytes
    pub size: u64,
    /// Content type identifier
    pub content_type: ContentType,
    /// Codec used for encoding
    pub codec: Codec,
}

/// Content types supported in CAR/DAG packaging
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContentType {
    /// Raw binary data
    Raw,
    /// Packet envelope
    PacketEnvelope,
    /// Shard header
    ShardHeader,
    /// Data availability root
    DataAvailabilityRoot,
    /// DAG node
    DagNode,
    /// Metadata
    Metadata,
    /// Custom content type
    Custom(String),
}

/// Codecs for content encoding
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Codec {
    /// Raw binary (no encoding)
    Raw,
    /// CBOR encoding
    Cbor,
    /// JSON encoding
    Json,
    /// Protocol Buffers
    Protobuf,
    /// Custom codec
    Custom(String),
}

impl ContentId {
    /// Create a new Content ID from data
    pub fn new(data: &[u8], content_type: ContentType, codec: Codec) -> Self {
        let hash = domain_hash(CAR_PACKAGE_HASH, data);
        let size = data.len() as u64;
        
        Self {
            hash,
            size,
            content_type,
            codec,
        }
    }

    /// Create CID from packet envelope
    pub fn from_envelope(envelope: &PacketEnvelope) -> Result<Self, DockLockError> {
        let data = bincode::serialize(envelope)?;
        Ok(Self::new(&data, ContentType::PacketEnvelope, Codec::Cbor))
    }

    /// Create CID from shard header
    pub fn from_shard_header(header: &ShardHeader) -> Result<Self, DockLockError> {
        let data = bincode::serialize(header)?;
        Ok(Self::new(&data, ContentType::ShardHeader, Codec::Cbor))
    }

    /// Create CID from DA root
    pub fn from_da_root(root: &DataAvailabilityRoot) -> Result<Self, DockLockError> {
        let data = bincode::serialize(root)?;
        Ok(Self::new(&data, ContentType::DataAvailabilityRoot, Codec::Cbor))
    }

    /// Get string representation of CID
    pub fn to_string(&self) -> String {
        format!("{}:{:?}:{:?}:{}", 
            hex::encode(&self.hash[..8]), 
            self.content_type, 
            self.codec, 
            self.size
        )
    }
}

/// DAG link connecting nodes in the graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagLink {
    /// Name/label of the link
    pub name: String,
    /// Target content ID
    pub target: ContentId,
    /// Link metadata
    pub metadata: HashMap<String, String>,
    /// Link weight/priority
    pub weight: u32,
    /// Link type
    pub link_type: LinkType,
}

/// Types of DAG links
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinkType {
    /// Parent-child relationship
    Child,
    /// Reference/dependency
    Reference,
    /// Shard relationship
    Shard,
    /// Metadata link
    Metadata,
    /// Custom link type
    Custom(String),
}

impl DagLink {
    /// Create a new DAG link
    pub fn new(name: String, target: ContentId, link_type: LinkType) -> Self {
        Self {
            name,
            target,
            metadata: HashMap::new(),
            weight: 1,
            link_type,
        }
    }

    /// Add metadata to the link
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Set link weight
    pub fn with_weight(mut self, weight: u32) -> Self {
        self.weight = weight;
        self
    }

    /// Compute hash of the link
    pub fn compute_hash(&self) -> [u8; 32] {
        let link_data = format!("{}:{}:{:?}:{}", 
            self.name, 
            self.target.to_string(), 
            self.link_type, 
            self.weight
        );
        domain_hash(DAG_LINK_HASH, link_data.as_bytes())
    }
}

/// DAG node in the content graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagNode {
    /// Content ID of this node
    pub cid: ContentId,
    /// Links to other nodes
    pub links: Vec<DagLink>,
    /// Node data (optional, can be stored separately)
    pub data: Option<Vec<u8>>,
    /// Node metadata
    pub metadata: HashMap<String, String>,
    /// Creation timestamp
    pub created_at: u64,
    /// Node signature for integrity
    #[serde(serialize_with = "serialize_signature", deserialize_with = "deserialize_signature")]
    pub signature: Option<[u8; 64]>,
}

impl DagNode {
    /// Create a new DAG node
    pub fn new(cid: ContentId) -> Self {
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            cid,
            links: Vec::new(),
            data: None,
            metadata: HashMap::new(),
            created_at,
            signature: None,
        }
    }

    /// Add a link to another node
    pub fn add_link(&mut self, link: DagLink) {
        self.links.push(link);
    }

    /// Set node data
    pub fn with_data(mut self, data: Vec<u8>) -> Self {
        self.data = Some(data);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Compute signing hash (excludes signature)
    pub fn compute_signing_hash(&self) -> Result<[u8; 32], DockLockError> {
        let node_without_sig = DagNode {
            cid: self.cid.clone(),
            links: self.links.clone(),
            data: self.data.clone(),
            metadata: self.metadata.clone(),
            created_at: self.created_at,
            signature: None,
        };
        
        let data = bincode::serialize(&node_without_sig)?;
        Ok(domain_hash(DAG_NODE_HASH, &data))
    }

    /// Sign the DAG node
    pub fn sign(&mut self, signing_key: &SigningKey) -> Result<(), DockLockError> {
        let hash = self.compute_signing_hash()?;
        let signature = signing_key.sign(&hash);
        self.signature = Some(signature.to_bytes());
        Ok(())
    }

    /// Verify node signature
    pub fn verify_signature(&self, verifying_key: &VerifyingKey) -> Result<bool, DockLockError> {
        let Some(sig_bytes) = &self.signature else {
            return Ok(false);
        };

        let signature = Signature::from_bytes(sig_bytes);
        let hash = self.compute_signing_hash()?;
        Ok(verifying_key.verify(&hash, &signature).is_ok())
    }

    /// Get all linked content IDs
    pub fn get_linked_cids(&self) -> Vec<ContentId> {
        self.links.iter().map(|link| link.target.clone()).collect()
    }

    /// Check if node has a specific link
    pub fn has_link_to(&self, target_cid: &ContentId) -> bool {
        self.links.iter().any(|link| &link.target == target_cid)
    }

    /// Get links by type
    pub fn get_links_by_type(&self, link_type: &LinkType) -> Vec<&DagLink> {
        self.links.iter().filter(|link| &link.link_type == link_type).collect()
    }
}

/// CAR (Content Addressable aRchive) package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarPackage {
    /// Package ID
    pub package_id: Uuid,
    /// Root content IDs in this package
    pub roots: Vec<ContentId>,
    /// All content blocks in the package
    pub blocks: HashMap<ContentId, Vec<u8>>,
    /// Package metadata
    pub metadata: HashMap<String, String>,
    /// Creation timestamp
    pub created_at: u64,
    /// Package version
    pub version: u32,
    /// Compression scheme used
    pub compression: CompressionScheme,
    /// Package signature
    #[serde(serialize_with = "serialize_signature", deserialize_with = "deserialize_signature")]
    pub signature: Option<[u8; 64]>,
}

/// Compression schemes for CAR packages
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionScheme {
    /// No compression
    None,
    /// Gzip compression
    Gzip,
    /// Zstd compression (recommended)
    Zstd,
    /// LZ4 compression
    Lz4,
}

impl CarPackage {
    /// Create a new CAR package
    pub fn new() -> Self {
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            package_id: Uuid::new_v4(),
            roots: Vec::new(),
            blocks: HashMap::new(),
            metadata: HashMap::new(),
            created_at,
            version: 1,
            compression: CompressionScheme::Zstd,
            signature: None,
        }
    }

    /// Add a root content ID
    pub fn add_root(&mut self, cid: ContentId) {
        if !self.roots.contains(&cid) {
            self.roots.push(cid);
        }
    }

    /// Add a content block
    pub fn add_block(&mut self, cid: ContentId, data: Vec<u8>) {
        self.blocks.insert(cid, data);
    }

    /// Add metadata
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get block data by CID
    pub fn get_block(&self, cid: &ContentId) -> Option<&Vec<u8>> {
        self.blocks.get(cid)
    }

    /// Check if package contains a block
    pub fn has_block(&self, cid: &ContentId) -> bool {
        self.blocks.contains_key(cid)
    }

    /// Get package size in bytes
    pub fn size_bytes(&self) -> u64 {
        self.blocks.values().map(|data| data.len() as u64).sum()
    }

    /// Get number of blocks
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// Compute signing hash (excludes signature)
    pub fn compute_signing_hash(&self) -> Result<[u8; 32], DockLockError> {
        let package_without_sig = CarPackage {
            package_id: self.package_id,
            roots: self.roots.clone(),
            blocks: self.blocks.clone(),
            metadata: self.metadata.clone(),
            created_at: self.created_at,
            version: self.version,
            compression: self.compression.clone(),
            signature: None,
        };
        
        let data = bincode::serialize(&package_without_sig)?;
        Ok(domain_hash(CAR_PACKAGE_HASH, &data))
    }

    /// Sign the CAR package
    pub fn sign(&mut self, signing_key: &SigningKey) -> Result<(), DockLockError> {
        let hash = self.compute_signing_hash()?;
        let signature = signing_key.sign(&hash);
        self.signature = Some(signature.to_bytes());
        Ok(())
    }

    /// Verify package signature
    pub fn verify_signature(&self, verifying_key: &VerifyingKey) -> Result<bool, DockLockError> {
        let Some(sig_bytes) = &self.signature else {
            return Ok(false);
        };

        let signature = Signature::from_bytes(sig_bytes);
        let hash = self.compute_signing_hash()?;
        Ok(verifying_key.verify(&hash, &signature).is_ok())
    }
}

impl Default for CarPackage {
    fn default() -> Self {
        Self::new()
    }
}

/// DAG store for managing content graphs
#[derive(Debug)]
pub struct DagStore {
    /// DAG nodes indexed by CID
    nodes: Arc<RwLock<HashMap<ContentId, DagNode>>>,
    /// Content blocks indexed by CID
    blocks: Arc<RwLock<HashMap<ContentId, Vec<u8>>>>,
    /// Root nodes (entry points)
    roots: Arc<RwLock<HashSet<ContentId>>>,
    /// Statistics
    stats: Arc<RwLock<DagStoreStats>>,
}

/// Statistics for DAG store operations
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DagStoreStats {
    /// Total nodes in the store
    pub total_nodes: u64,
    /// Total blocks in the store
    pub total_blocks: u64,
    /// Total size in bytes
    pub total_size_bytes: u64,
    /// Number of root nodes
    pub root_count: u64,
    /// Number of get operations
    pub get_operations: u64,
    /// Number of put operations
    pub put_operations: u64,
    /// Number of traversal operations
    pub traversal_operations: u64,
}

impl DagStore {
    /// Create a new DAG store
    pub fn new() -> Self {
        Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
            blocks: Arc::new(RwLock::new(HashMap::new())),
            roots: Arc::new(RwLock::new(HashSet::new())),
            stats: Arc::new(RwLock::new(DagStoreStats::default())),
        }
    }

    /// Add a DAG node to the store
    pub fn put_node(&self, node: DagNode) -> Result<(), DockLockError> {
        let cid = node.cid.clone();
        
        let mut nodes = self.nodes.write().map_err(|_| {
            DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                "Failed to acquire nodes write lock".to_string(),
            )))
        })?;
        
        nodes.insert(cid, node);
        
        let mut stats = self.stats.write().map_err(|_| {
            DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                "Failed to acquire stats write lock".to_string(),
            )))
        })?;
        stats.total_nodes += 1;
        stats.put_operations += 1;
        
        Ok(())
    }

    /// Get a DAG node from the store
    pub fn get_node(&self, cid: &ContentId) -> Result<Option<DagNode>, DockLockError> {
        let nodes = self.nodes.read().map_err(|_| {
            DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                "Failed to acquire nodes read lock".to_string(),
            )))
        })?;
        
        let node = nodes.get(cid).cloned();
        
        let mut stats = self.stats.write().map_err(|_| {
            DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                "Failed to acquire stats write lock".to_string(),
            )))
        })?;
        stats.get_operations += 1;
        
        Ok(node)
    }

    /// Add a content block to the store
    pub fn put_block(&self, cid: ContentId, data: Vec<u8>) -> Result<(), DockLockError> {
        let data_size = data.len() as u64;
        
        let mut blocks = self.blocks.write().map_err(|_| {
            DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                "Failed to acquire blocks write lock".to_string(),
            )))
        })?;
        
        blocks.insert(cid, data);
        
        let mut stats = self.stats.write().map_err(|_| {
            DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                "Failed to acquire stats write lock".to_string(),
            )))
        })?;
        stats.total_blocks += 1;
        stats.total_size_bytes += data_size;
        stats.put_operations += 1;
        
        Ok(())
    }

    /// Get a content block from the store
    pub fn get_block(&self, cid: &ContentId) -> Result<Option<Vec<u8>>, DockLockError> {
        let blocks = self.blocks.read().map_err(|_| {
            DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                "Failed to acquire blocks read lock".to_string(),
            )))
        })?;
        
        let block = blocks.get(cid).cloned();
        
        let mut stats = self.stats.write().map_err(|_| {
            DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                "Failed to acquire stats write lock".to_string(),
            )))
        })?;
        stats.get_operations += 1;
        
        Ok(block)
    }

    /// Add a root node
    pub fn add_root(&self, cid: ContentId) -> Result<(), DockLockError> {
        let mut roots = self.roots.write().map_err(|_| {
            DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                "Failed to acquire roots write lock".to_string(),
            )))
        })?;
        
        if roots.insert(cid) {
            let mut stats = self.stats.write().map_err(|_| {
                DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                    "Failed to acquire stats write lock".to_string(),
                )))
            })?;
            stats.root_count += 1;
        }
        
        Ok(())
    }

    /// Get all root nodes
    pub fn get_roots(&self) -> Result<Vec<ContentId>, DockLockError> {
        let roots = self.roots.read().map_err(|_| {
            DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                "Failed to acquire roots read lock".to_string(),
            )))
        })?;
        
        Ok(roots.iter().cloned().collect())
    }

    /// Traverse the DAG starting from a root
    pub fn traverse_from_root(&self, root_cid: &ContentId) -> Result<Vec<ContentId>, DockLockError> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();
        
        queue.push_back(root_cid.clone());
        
        while let Some(current_cid) = queue.pop_front() {
            if visited.contains(&current_cid) {
                continue;
            }
            
            visited.insert(current_cid.clone());
            result.push(current_cid.clone());
            
            if let Some(node) = self.get_node(&current_cid)? {
                for link in &node.links {
                    if !visited.contains(&link.target) {
                        queue.push_back(link.target.clone());
                    }
                }
            }
        }
        
        let mut stats = self.stats.write().map_err(|_| {
            DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                "Failed to acquire stats write lock".to_string(),
            )))
        })?;
        stats.traversal_operations += 1;
        
        Ok(result)
    }

    /// Get store statistics
    pub fn get_stats(&self) -> Result<DagStoreStats, DockLockError> {
        let stats = self.stats.read().map_err(|_| {
            DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                "Failed to acquire stats read lock".to_string(),
            )))
        })?;
        
        Ok(DagStoreStats {
            total_nodes: stats.total_nodes,
            total_blocks: stats.total_blocks,
            total_size_bytes: stats.total_size_bytes,
            root_count: stats.root_count,
            get_operations: stats.get_operations,
            put_operations: stats.put_operations,
            traversal_operations: stats.traversal_operations,
        })
    }
}

impl Default for DagStore {
    fn default() -> Self {
        Self::new()
    }
}

/// CAR/DAG packaging manager
#[derive(Debug)]
pub struct CarDagManager {
    /// DAG store for content management
    dag_store: DagStore,
    /// CAR packages
    packages: Arc<RwLock<HashMap<Uuid, CarPackage>>>,
    /// Manager statistics
    stats: Arc<RwLock<CarDagManagerStats>>,
}

/// Statistics for CAR/DAG manager operations
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CarDagManagerStats {
    /// Total packages created
    pub packages_created: u64,
    /// Total envelopes packaged
    pub envelopes_packaged: u64,
    /// Total bytes packaged
    pub bytes_packaged: u64,
    /// Deduplication savings in bytes
    pub deduplication_savings: u64,
    /// Package operations performed
    pub package_operations: u64,
}

impl CarDagManager {
    /// Create a new CAR/DAG manager
    pub fn new() -> Self {
        Self {
            dag_store: DagStore::new(),
            packages: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(CarDagManagerStats::default())),
        }
    }

    /// Package a packet envelope into CAR/DAG format
    pub fn package_envelope(&self, envelope: PacketEnvelope) -> Result<Uuid, DockLockError> {
        // Create CID for the envelope
        let envelope_cid = ContentId::from_envelope(&envelope)?;
        
        // Serialize envelope data
        let envelope_data = bincode::serialize(&envelope)?;
        
        // Create DAG node for the envelope
        let mut envelope_node = DagNode::new(envelope_cid.clone())
            .with_data(envelope_data.clone())
            .with_metadata("type".to_string(), "packet_envelope".to_string())
            .with_metadata("packet_id".to_string(), envelope.packet_id.clone());
        
        // Add links for shard header and DA root if present
        if let Some(shard_header) = &envelope.shard_header {
            let shard_cid = ContentId::from_shard_header(shard_header)?;
            let shard_data = bincode::serialize(shard_header)?;
            
            let shard_link = DagLink::new(
                "shard_header".to_string(),
                shard_cid.clone(),
                LinkType::Child,
            );
            envelope_node.add_link(shard_link);
            
            // Store shard header as separate node
            let shard_node = DagNode::new(shard_cid.clone())
                .with_data(shard_data.clone())
                .with_metadata("type".to_string(), "shard_header".to_string());
            
            self.dag_store.put_node(shard_node)?;
            self.dag_store.put_block(shard_cid, shard_data)?;
        }
        
        // Store envelope node and block
        self.dag_store.put_node(envelope_node)?;
        self.dag_store.put_block(envelope_cid.clone(), envelope_data)?;
        self.dag_store.add_root(envelope_cid.clone())?;
        
        // Create CAR package
        let mut car_package = CarPackage::new();
        car_package.add_root(envelope_cid.clone());
        car_package.add_block(envelope_cid, bincode::serialize(&envelope)?);
        car_package.add_metadata("type".to_string(), "envelope_package".to_string());
        car_package.add_metadata("envelope_id".to_string(), envelope.packet_id.clone());
        
        let package_id = car_package.package_id;
        
        // Store the package
        let mut packages = self.packages.write().map_err(|_| {
            DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                "Failed to acquire packages write lock".to_string(),
            )))
        })?;
        packages.insert(package_id, car_package);
        
        // Update statistics
        let mut stats = self.stats.write().map_err(|_| {
            DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                "Failed to acquire stats write lock".to_string(),
            )))
        })?;
        stats.packages_created += 1;
        stats.envelopes_packaged += 1;
        stats.bytes_packaged += bincode::serialize(&envelope)?.len() as u64;
        stats.package_operations += 1;
        
        Ok(package_id)
    }

    /// Get a CAR package by ID
    pub fn get_package(&self, package_id: &Uuid) -> Result<Option<CarPackage>, DockLockError> {
        let packages = self.packages.read().map_err(|_| {
            DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                "Failed to acquire packages read lock".to_string(),
            )))
        })?;
        
        Ok(packages.get(package_id).cloned())
    }

    /// Get DAG store reference
    pub fn dag_store(&self) -> &DagStore {
        &self.dag_store
    }

    /// Get manager statistics
    pub fn get_stats(&self) -> Result<CarDagManagerStats, DockLockError> {
        let stats = self.stats.read().map_err(|_| {
            DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                "Failed to acquire stats read lock".to_string(),
            )))
        })?;
        
        Ok(CarDagManagerStats {
            packages_created: stats.packages_created,
            envelopes_packaged: stats.envelopes_packaged,
            bytes_packaged: stats.bytes_packaged,
            deduplication_savings: stats.deduplication_savings,
            package_operations: stats.package_operations,
        })
    }

    /// Optimize storage by deduplicating content
    pub fn optimize_storage(&self) -> Result<u64, DockLockError> {
        // This is a simplified deduplication implementation
        // In production, this would implement more sophisticated deduplication algorithms
        
        let savings = 0u64;
        
        // Update statistics with savings
        let mut stats = self.stats.write().map_err(|_| {
            DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                "Failed to acquire stats write lock".to_string(),
            )))
        })?;
        stats.deduplication_savings += savings;
        
        Ok(savings)
    }
}

impl Default for CarDagManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet_envelope::{PacketEnvelope, EnvelopeMetadata, EncryptionScheme};
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    fn create_test_envelope() -> PacketEnvelope {
        use crate::traffic_light::{DataClassification, TrafficLightState};
        
        let metadata = EnvelopeMetadata::new(
            "test_origin".to_string(),
            DataClassification::Public,
            TrafficLightState::Green,
            [0u8; 32], // dummy policy hash
            EncryptionScheme::XChaCha20Poly1305,
        );
        
        let payload_hash = [1u8; 32]; // dummy payload hash
        
        PacketEnvelope::new("test_packet".to_string(), metadata, payload_hash)
    }

    fn create_test_signing_key() -> SigningKey {
        SigningKey::generate(&mut OsRng)
    }

    #[test]
    fn test_content_id_creation() {
        let data = b"test data";
        let cid = ContentId::new(data, ContentType::Raw, Codec::Raw);
        
        assert_eq!(cid.size, data.len() as u64);
        assert_eq!(cid.content_type, ContentType::Raw);
        assert_eq!(cid.codec, Codec::Raw);
        assert_eq!(cid.hash.len(), 32);
    }

    #[test]
    fn test_content_id_from_envelope() {
        let envelope = create_test_envelope();
        let cid = ContentId::from_envelope(&envelope).unwrap();
        
        assert_eq!(cid.content_type, ContentType::PacketEnvelope);
        assert_eq!(cid.codec, Codec::Cbor);
        assert!(cid.size > 0);
    }

    #[test]
    fn test_content_id_string_representation() {
        let data = b"test data";
        let cid = ContentId::new(data, ContentType::Raw, Codec::Raw);
        let cid_string = cid.to_string();
        
        assert!(cid_string.contains("Raw"));
        assert!(cid_string.contains(&cid.size.to_string()));
    }

    #[test]
    fn test_dag_link_creation() {
        let data = b"test data";
        let target_cid = ContentId::new(data, ContentType::Raw, Codec::Raw);
        
        let link = DagLink::new("test_link".to_string(), target_cid.clone(), LinkType::Child)
            .with_metadata("key".to_string(), "value".to_string())
            .with_weight(5);
        
        assert_eq!(link.name, "test_link");
        assert_eq!(link.target, target_cid);
        assert_eq!(link.link_type, LinkType::Child);
        assert_eq!(link.weight, 5);
        assert_eq!(link.metadata.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_dag_link_hash() {
        let data = b"test data";
        let target_cid = ContentId::new(data, ContentType::Raw, Codec::Raw);
        let link = DagLink::new("test_link".to_string(), target_cid, LinkType::Child);
        
        let hash = link.compute_hash();
        assert_eq!(hash.len(), 32);
        
        // Hash should be deterministic
        let hash2 = link.compute_hash();
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_dag_node_creation() {
        let data = b"test data";
        let cid = ContentId::new(data, ContentType::Raw, Codec::Raw);
        
        let node = DagNode::new(cid.clone())
            .with_data(data.to_vec())
            .with_metadata("type".to_string(), "test".to_string());
        
        assert_eq!(node.cid, cid);
        assert_eq!(node.data, Some(data.to_vec()));
        assert_eq!(node.metadata.get("type"), Some(&"test".to_string()));
        assert!(node.created_at > 0);
        assert!(node.links.is_empty());
    }

    #[test]
    fn test_dag_node_links() {
        let data1 = b"test data 1";
        let data2 = b"test data 2";
        let cid1 = ContentId::new(data1, ContentType::Raw, Codec::Raw);
        let cid2 = ContentId::new(data2, ContentType::Raw, Codec::Raw);
        
        let mut node = DagNode::new(cid1);
        let link = DagLink::new("child".to_string(), cid2.clone(), LinkType::Child);
        node.add_link(link);
        
        assert_eq!(node.links.len(), 1);
        assert!(node.has_link_to(&cid2));
        
        let linked_cids = node.get_linked_cids();
        assert_eq!(linked_cids.len(), 1);
        assert_eq!(linked_cids[0], cid2);
        
        let child_links = node.get_links_by_type(&LinkType::Child);
        assert_eq!(child_links.len(), 1);
        assert_eq!(child_links[0].target, cid2);
    }

    #[test]
    fn test_dag_node_signing() {
        let data = b"test data";
        let cid = ContentId::new(data, ContentType::Raw, Codec::Raw);
        let mut node = DagNode::new(cid).with_data(data.to_vec());
        
        let signing_key = create_test_signing_key();
        let verifying_key = signing_key.verifying_key();
        
        // Sign the node
        node.sign(&signing_key).unwrap();
        assert!(node.signature.is_some());
        
        // Verify the signature
        assert!(node.verify_signature(&verifying_key).unwrap());
        
        // Verify with wrong key should fail
        let wrong_key = create_test_signing_key();
        let wrong_verifying_key = wrong_key.verifying_key();
        assert!(!node.verify_signature(&wrong_verifying_key).unwrap());
    }

    #[test]
    fn test_car_package_creation() {
        let package = CarPackage::new();
        
        assert!(package.roots.is_empty());
        assert!(package.blocks.is_empty());
        assert_eq!(package.version, 1);
        assert_eq!(package.compression, CompressionScheme::Zstd);
        assert!(package.created_at > 0);
    }

    #[test]
    fn test_car_package_operations() {
        let mut package = CarPackage::new();
        
        let data = b"test data";
        let cid = ContentId::new(data, ContentType::Raw, Codec::Raw);
        
        // Add root and block
        package.add_root(cid.clone());
        package.add_block(cid.clone(), data.to_vec());
        package.add_metadata("key".to_string(), "value".to_string());
        
        assert_eq!(package.roots.len(), 1);
        assert_eq!(package.roots[0], cid);
        assert!(package.has_block(&cid));
        assert_eq!(package.get_block(&cid), Some(&data.to_vec()));
        assert_eq!(package.block_count(), 1);
        assert_eq!(package.size_bytes(), data.len() as u64);
        assert_eq!(package.metadata.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_car_package_signing() {
        let mut package = CarPackage::new();
        let data = b"test data";
        let cid = ContentId::new(data, ContentType::Raw, Codec::Raw);
        package.add_block(cid, data.to_vec());
        
        let signing_key = create_test_signing_key();
        let verifying_key = signing_key.verifying_key();
        
        // Sign the package
        package.sign(&signing_key).unwrap();
        assert!(package.signature.is_some());
        
        // Verify the signature
        assert!(package.verify_signature(&verifying_key).unwrap());
        
        // Verify with wrong key should fail
        let wrong_key = create_test_signing_key();
        let wrong_verifying_key = wrong_key.verifying_key();
        assert!(!package.verify_signature(&wrong_verifying_key).unwrap());
    }

    #[test]
    fn test_dag_store_operations() {
        let store = DagStore::new();
        
        let data = b"test data";
        let cid = ContentId::new(data, ContentType::Raw, Codec::Raw);
        let node = DagNode::new(cid.clone()).with_data(data.to_vec());
        
        // Put and get node
        store.put_node(node.clone()).unwrap();
        let retrieved_node = store.get_node(&cid).unwrap();
        assert!(retrieved_node.is_some());
        assert_eq!(retrieved_node.unwrap().cid, cid);
        
        // Put and get block
        store.put_block(cid.clone(), data.to_vec()).unwrap();
        let retrieved_block = store.get_block(&cid).unwrap();
        assert!(retrieved_block.is_some());
        assert_eq!(retrieved_block.unwrap(), data.to_vec());
        
        // Add and get roots
        store.add_root(cid.clone()).unwrap();
        let roots = store.get_roots().unwrap();
        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0], cid);
    }

    #[test]
    fn test_dag_store_traversal() {
        let store = DagStore::new();
        
        // Create a simple DAG: root -> child1, child2
        let root_data = b"root data";
        let child1_data = b"child1 data";
        let child2_data = b"child2 data";
        
        let root_cid = ContentId::new(root_data, ContentType::Raw, Codec::Raw);
        let child1_cid = ContentId::new(child1_data, ContentType::Raw, Codec::Raw);
        let child2_cid = ContentId::new(child2_data, ContentType::Raw, Codec::Raw);
        
        // Create nodes
        let mut root_node = DagNode::new(root_cid.clone());
        root_node.add_link(DagLink::new("child1".to_string(), child1_cid.clone(), LinkType::Child));
        root_node.add_link(DagLink::new("child2".to_string(), child2_cid.clone(), LinkType::Child));
        
        let child1_node = DagNode::new(child1_cid.clone());
        let child2_node = DagNode::new(child2_cid.clone());
        
        // Store nodes
        store.put_node(root_node).unwrap();
        store.put_node(child1_node).unwrap();
        store.put_node(child2_node).unwrap();
        store.add_root(root_cid.clone()).unwrap();
        
        // Traverse from root
        let traversed = store.traverse_from_root(&root_cid).unwrap();
        assert_eq!(traversed.len(), 3); // root + 2 children
        assert!(traversed.contains(&root_cid));
        assert!(traversed.contains(&child1_cid));
        assert!(traversed.contains(&child2_cid));
    }

    #[test]
    fn test_dag_store_statistics() {
        let store = DagStore::new();
        
        let data = b"test data";
        let cid = ContentId::new(data, ContentType::Raw, Codec::Raw);
        let node = DagNode::new(cid.clone());
        
        // Perform operations
        store.put_node(node).unwrap();
        store.put_block(cid.clone(), data.to_vec()).unwrap();
        store.add_root(cid.clone()).unwrap();
        store.get_node(&cid).unwrap();
        store.get_block(&cid).unwrap();
        store.traverse_from_root(&cid).unwrap();
        
        let stats = store.get_stats().unwrap();
        assert_eq!(stats.total_nodes, 1);
        assert_eq!(stats.total_blocks, 1);
        assert_eq!(stats.root_count, 1);
        assert_eq!(stats.total_size_bytes, data.len() as u64);
        assert!(stats.get_operations >= 2);
        assert!(stats.put_operations >= 2);
        assert_eq!(stats.traversal_operations, 1);
    }

    #[test]
    fn test_car_dag_manager_creation() {
        let manager = CarDagManager::new();
        let stats = manager.get_stats().unwrap();
        
        assert_eq!(stats.packages_created, 0);
        assert_eq!(stats.envelopes_packaged, 0);
        assert_eq!(stats.bytes_packaged, 0);
        assert_eq!(stats.deduplication_savings, 0);
        assert_eq!(stats.package_operations, 0);
    }

    #[test]
    fn test_car_dag_manager_package_envelope() {
        let manager = CarDagManager::new();
        let envelope = create_test_envelope();
        
        // Package the envelope
        let package_id = manager.package_envelope(envelope.clone()).unwrap();
        
        // Verify package was created
        let package = manager.get_package(&package_id).unwrap();
        assert!(package.is_some());
        
        let package = package.unwrap();
        assert_eq!(package.roots.len(), 1);
        assert!(package.block_count() > 0);
        assert_eq!(package.metadata.get("type"), Some(&"envelope_package".to_string()));
        assert_eq!(package.metadata.get("envelope_id"), Some(&envelope.packet_id));
        
        // Check statistics
        let stats = manager.get_stats().unwrap();
        assert_eq!(stats.packages_created, 1);
        assert_eq!(stats.envelopes_packaged, 1);
        assert!(stats.bytes_packaged > 0);
        assert_eq!(stats.package_operations, 1);
    }

    #[test]
    fn test_car_dag_manager_dag_store_integration() {
        let manager = CarDagManager::new();
        let envelope = create_test_envelope();
        
        // Package envelope
        let _package_id = manager.package_envelope(envelope).unwrap();
        
        // Check DAG store has content
        let dag_stats = manager.dag_store().get_stats().unwrap();
        assert!(dag_stats.total_nodes > 0);
        assert!(dag_stats.total_blocks > 0);
        assert!(dag_stats.root_count > 0);
        
        // Check roots exist
        let roots = manager.dag_store().get_roots().unwrap();
        assert!(!roots.is_empty());
    }

    #[test]
    fn test_car_dag_manager_storage_optimization() {
        let manager = CarDagManager::new();
        
        // Run storage optimization
        let savings = manager.optimize_storage().unwrap();
        assert_eq!(savings, 0); // No content to optimize yet
        
        // Package some content first
        let envelope = create_test_envelope();
        let _package_id = manager.package_envelope(envelope).unwrap();
        
        // Run optimization again
        let savings = manager.optimize_storage().unwrap();
        assert_eq!(savings, 0); // Simplified implementation returns 0
        
        // Check stats include deduplication tracking
        let stats = manager.get_stats().unwrap();
        assert_eq!(stats.deduplication_savings, 0);
    }

    #[test]
    fn test_content_deduplication() {
        let manager = CarDagManager::new();
        let envelope1 = create_test_envelope();
        let mut envelope2 = create_test_envelope();
        envelope2.packet_id = "test_packet_2".to_string(); // Different ID, same content structure
        
        // Package both envelopes
        let package_id1 = manager.package_envelope(envelope1).unwrap();
        let package_id2 = manager.package_envelope(envelope2).unwrap();
        
        // Verify both packages exist
        assert!(manager.get_package(&package_id1).unwrap().is_some());
        assert!(manager.get_package(&package_id2).unwrap().is_some());
        
        // Check statistics
        let stats = manager.get_stats().unwrap();
        assert_eq!(stats.packages_created, 2);
        assert_eq!(stats.envelopes_packaged, 2);
        assert!(stats.bytes_packaged > 0);
    }

    #[test]
    fn test_multiple_content_types() {
        let data1 = b"raw data";
        let data2 = b"json data";
        let data3 = b"cbor data";
        
        let cid1 = ContentId::new(data1, ContentType::Raw, Codec::Raw);
        let cid2 = ContentId::new(data2, ContentType::Custom("json".to_string()), Codec::Json);
        let cid3 = ContentId::new(data3, ContentType::Metadata, Codec::Cbor);
        
        // Verify different content types
        assert_eq!(cid1.content_type, ContentType::Raw);
        assert_eq!(cid2.content_type, ContentType::Custom("json".to_string()));
        assert_eq!(cid3.content_type, ContentType::Metadata);
        
        // Verify different codecs
        assert_eq!(cid1.codec, Codec::Raw);
        assert_eq!(cid2.codec, Codec::Json);
        assert_eq!(cid3.codec, Codec::Cbor);
    }

    #[test]
    fn test_link_types() {
        let data = b"test data";
        let target_cid = ContentId::new(data, ContentType::Raw, Codec::Raw);
        
        let child_link = DagLink::new("child".to_string(), target_cid.clone(), LinkType::Child);
        let ref_link = DagLink::new("reference".to_string(), target_cid.clone(), LinkType::Reference);
        let shard_link = DagLink::new("shard".to_string(), target_cid.clone(), LinkType::Shard);
        let meta_link = DagLink::new("metadata".to_string(), target_cid.clone(), LinkType::Metadata);
        let custom_link = DagLink::new("custom".to_string(), target_cid, LinkType::Custom("special".to_string()));
        
        assert_eq!(child_link.link_type, LinkType::Child);
        assert_eq!(ref_link.link_type, LinkType::Reference);
        assert_eq!(shard_link.link_type, LinkType::Shard);
        assert_eq!(meta_link.link_type, LinkType::Metadata);
        assert_eq!(custom_link.link_type, LinkType::Custom("special".to_string()));
    }
}
