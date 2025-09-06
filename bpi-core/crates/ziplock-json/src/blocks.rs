//! ZIPLOCK-JSON block types and structures

use zerocopy::{AsBytes, FromBytes, FromZeroes};
use serde::{Deserialize, Serialize};
use crate::{ZjlResult, ZjlError};

/// Block type constants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BlockType {
    // JSON chunks (readable as JSON)
    JsonObject = 0x01,
    JsonArray = 0x02,
    JsonString = 0x03,
    JsonNumber = 0x04,
    JsonBool = 0x05,
    JsonNull = 0x06,
    JsonChunked = 0x07,  // For large arrays/strings

    // Merkle rollup roots
    SecondRoot = 0x10,
    MinuteRoot = 0x11,
    HourRoot = 0x12,
    DayRoot = 0x13,

    // Audit and indexing
    ReceiptMicro = 0x14,
    IndexBitmap = 0x15,
    CidCatalog = 0x16,

    // Forensics
    Reason = 0x20,
    Snapshot = 0x21,
    OobWitness = 0x22,
    AttackGraph = 0x23,

    // Shard management
    ShardFooter = 0x30,

    // Padding
    Pad = 0x7F,
}

impl BlockType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x01 => Some(Self::JsonObject),
            0x02 => Some(Self::JsonArray),
            0x03 => Some(Self::JsonString),
            0x04 => Some(Self::JsonNumber),
            0x05 => Some(Self::JsonBool),
            0x06 => Some(Self::JsonNull),
            0x07 => Some(Self::JsonChunked),
            0x10 => Some(Self::SecondRoot),
            0x11 => Some(Self::MinuteRoot),
            0x12 => Some(Self::HourRoot),
            0x13 => Some(Self::DayRoot),
            0x14 => Some(Self::ReceiptMicro),
            0x15 => Some(Self::IndexBitmap),
            0x16 => Some(Self::CidCatalog),
            0x20 => Some(Self::Reason),
            0x21 => Some(Self::Snapshot),
            0x22 => Some(Self::OobWitness),
            0x23 => Some(Self::AttackGraph),
            0x30 => Some(Self::ShardFooter),
            0x7F => Some(Self::Pad),
            _ => None,
        }
    }

    pub fn is_json_chunk(&self) -> bool {
        matches!(self, 
            Self::JsonObject | Self::JsonArray | Self::JsonString | 
            Self::JsonNumber | Self::JsonBool | Self::JsonNull | Self::JsonChunked
        )
    }

    pub fn is_merkle_root(&self) -> bool {
        matches!(self, 
            Self::SecondRoot | Self::MinuteRoot | Self::HourRoot | Self::DayRoot
        )
    }

    pub fn is_forensic(&self) -> bool {
        matches!(self, 
            Self::Reason | Self::Snapshot | Self::OobWitness | Self::AttackGraph
        )
    }
}

/// Block header structure
#[derive(Debug, Clone, AsBytes, FromBytes, FromZeroes)]
#[repr(C, packed)]
pub struct BlockHeader {
    /// Block type
    pub block_type: u8,
    /// JSON path ID (0 if N/A)
    pub path_id: u64,
    /// Uncompressed length
    pub uncompressed_len: u32,
    /// Compressed length
    pub compressed_len: u32,
    /// BLAKE3 hash of uncompressed payload
    pub hash: [u8; 32],
}

impl BlockHeader {
    pub fn new(block_type: BlockType, path_id: u64, uncompressed_len: u32, compressed_len: u32, hash: [u8; 32]) -> Self {
        Self {
            block_type: block_type as u8,
            path_id,
            uncompressed_len,
            compressed_len,
            hash,
        }
    }

    pub fn block_type(&self) -> Option<BlockType> {
        BlockType::from_u8(self.block_type)
    }

    pub fn size() -> usize {
        std::mem::size_of::<Self>()
    }
}

/// Encrypted block header (with AEAD)
#[derive(AsBytes, FromBytes, FromZeroes)]
#[repr(C, packed)]
pub struct EncryptedBlockHeader {
    /// Base block header
    pub base: BlockHeader,
    /// AEAD nonce (12 bytes for ChaCha20-Poly1305)
    pub nonce: [u8; 12],
    /// AEAD tag (16 bytes)
    pub tag: [u8; 16],
}

impl std::fmt::Debug for EncryptedBlockHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EncryptedBlockHeader")
            .field("base", &"<BlockHeader>")
            .field("nonce", &self.nonce)
            .field("tag", &self.tag)
            .finish()
    }
}

impl Clone for EncryptedBlockHeader {
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            nonce: self.nonce,
            tag: self.tag,
        }
    }
}

impl EncryptedBlockHeader {
    pub fn new(base: BlockHeader, nonce: [u8; 12], tag: [u8; 16]) -> Self {
        Self { base, nonce, tag }
    }

    pub fn size() -> usize {
        std::mem::size_of::<Self>()
    }
}

/// Second-level Merkle root
#[derive(Debug, Clone, Serialize, Deserialize, AsBytes, FromBytes, FromZeroes)]
#[repr(C, packed)]
pub struct SecondRoot {
    /// Timestamp (Unix seconds)
    pub ts_sec: u64,
    /// Event count in this second
    pub count: u32,
    /// Merkle root hash
    pub root: [u8; 32],
}

/// Minute-level Merkle root (anchored to BPI)
#[derive(Debug, Clone, Serialize, Deserialize, AsBytes, FromBytes, FromZeroes)]
#[repr(C)]
pub struct MinuteRoot {
    /// Timestamp (Unix minutes)
    pub ts_min: u64,
    /// Merkle root hash
    pub root: [u8; 32],
    /// BPI transaction hash
    pub bpi_tx: [u8; 32],
}

/// Hour-level Merkle root
#[derive(Debug, Clone, Serialize, Deserialize, AsBytes, FromBytes, FromZeroes)]
#[repr(C)]
pub struct HourRoot {
    /// Timestamp (Unix hours)
    pub ts_hour: u64,
    /// Merkle root hash
    pub root: [u8; 32],
}

/// Day-level Merkle root
#[derive(Debug, Clone, Serialize, Deserialize, AsBytes, FromBytes, FromZeroes)]
#[repr(C)]
pub struct DayRoot {
    /// Timestamp (Unix days)
    pub ts_day: u64,
    /// Merkle root hash
    pub root: [u8; 32],
}

/// Shard footer for chain of custody
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardFooter {
    /// Previous shard hash
    pub prev_hash: [u8; 32],
    /// Time span covered by this shard
    pub span: TimeSpan,
    /// Merkle roots span
    pub roots_span: TimeSpan,
    /// Anchor points span
    pub anchors_span: TimeSpan,
}

/// Time span structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSpan {
    /// Start timestamp
    pub start: u64,
    /// End timestamp
    pub end: u64,
}

/// Central directory entry
#[derive(Debug, Clone, AsBytes, FromBytes, FromZeroes)]
#[repr(C, packed)]
pub struct CentralDirEntry {
    /// Block offset in file
    pub offset: u64,
    /// Block type
    pub block_type: u8,
    /// Path ID
    pub path_id: u64,
    /// Block hash
    pub hash: [u8; 32],
    /// Compressed length
    pub compressed_len: u32,
    /// Uncompressed length
    pub uncompressed_len: u32,
}

impl CentralDirEntry {
    pub fn new(
        offset: u64,
        block_type: BlockType,
        path_id: u64,
        hash: [u8; 32],
        compressed_len: u32,
        uncompressed_len: u32,
    ) -> Self {
        Self {
            offset,
            block_type: block_type as u8,
            path_id,
            hash,
            compressed_len,
            uncompressed_len,
        }
    }

    pub fn block_type(&self) -> Option<BlockType> {
        BlockType::from_u8(self.block_type)
    }

    pub fn size() -> usize {
        std::mem::size_of::<Self>()
    }
}

/// JSON path table entry
#[derive(Debug, Clone)]
pub struct PathTableEntry {
    /// Path ID
    pub path_id: u64,
    /// First block offset
    pub first_offset: u64,
    /// Number of blocks with this path
    pub count: u32,
    /// Bloom filter (16 bytes)
    pub bloom16: [u8; 16],
    /// Minimum key
    pub key_min: String,
    /// Maximum key
    pub key_max: String,
}

/// Block with its payload
#[derive(Debug, Clone)]
pub struct Block {
    /// Block header
    pub header: BlockHeader,
    /// Block payload (compressed)
    pub payload: Vec<u8>,
}

impl Block {
    pub fn new(block_type: BlockType, path_id: u64, payload: Vec<u8>, hash: [u8; 32]) -> Self {
        let header = BlockHeader::new(
            block_type,
            path_id,
            payload.len() as u32, // This should be uncompressed length
            payload.len() as u32, // This should be compressed length
            hash,
        );

        Self { header, payload }
    }

    pub fn total_size(&self) -> usize {
        BlockHeader::size() + self.payload.len()
    }

    pub fn block_type(&self) -> Option<BlockType> {
        self.header.block_type()
    }
}

/// Encrypted block with AEAD
#[derive(Debug, Clone)]
pub struct EncryptedBlock {
    /// Encrypted block header
    pub header: EncryptedBlockHeader,
    /// Encrypted payload
    pub payload: Vec<u8>,
}

impl EncryptedBlock {
    pub fn new(base_header: BlockHeader, nonce: [u8; 12], tag: [u8; 16], payload: Vec<u8>) -> Self {
        let header = EncryptedBlockHeader::new(base_header, nonce, tag);
        Self { header, payload }
    }

    pub fn total_size(&self) -> usize {
        EncryptedBlockHeader::size() + self.payload.len()
    }

    pub fn block_type(&self) -> Option<BlockType> {
        self.header.base.block_type()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_type_conversion() {
        assert_eq!(BlockType::from_u8(0x01), Some(BlockType::JsonObject));
        assert_eq!(BlockType::from_u8(0x20), Some(BlockType::Reason));
        assert_eq!(BlockType::from_u8(0xFF), None);
    }

    #[test]
    fn test_block_type_categories() {
        assert!(BlockType::JsonObject.is_json_chunk());
        assert!(BlockType::MinuteRoot.is_merkle_root());
        assert!(BlockType::Reason.is_forensic());
        assert!(!BlockType::Pad.is_json_chunk());
    }

    #[test]
    fn test_block_header_size() {
        // Ensure block header is a reasonable size
        assert!(BlockHeader::size() < 64);
        assert!(EncryptedBlockHeader::size() < 128);
    }

    #[test]
    fn test_central_dir_entry() {
        let entry = CentralDirEntry::new(
            1000,
            BlockType::JsonObject,
            42,
            [0u8; 32],
            100,
            150,
        );

        let offset = entry.offset;
        let path_id = entry.path_id;
        let compressed_len = entry.compressed_len;
        let uncompressed_len = entry.uncompressed_len;
        assert_eq!(offset, 1000);
        assert_eq!(path_id, 42);
        assert_eq!(compressed_len, 100);
        assert_eq!(uncompressed_len, 150);
    }

    #[test]
    fn test_merkle_roots() {
        let second_root = SecondRoot {
            ts_sec: 1640995200, // 2022-01-01 00:00:00 UTC
            count: 42,
            root: [1u8; 32],
        };

        let minute_root = MinuteRoot {
            ts_min: 27349920, // 2022-01-01 00:00 UTC in minutes
            root: [2u8; 32],
            bpi_tx: [3u8; 32],
        };

        let count = second_root.count;
        assert_eq!(count, 42);
        assert_eq!(minute_root.bpi_tx, [3u8; 32]);
    }
}
