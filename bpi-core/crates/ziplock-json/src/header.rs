//! ZIPLOCK-JSON file header structures and constants

use zerocopy::{AsBytes, FromBytes, FromZeroes};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{ZJL_MAGIC, ZJL_VERSION, ZjlResult, ZjlError};

/// Fixed header size (aligned to 0xA0 = 160 bytes)
pub const FIXED_HEADER_SIZE: usize = 160;

/// Header flags
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HeaderFlags(pub u16);

impl HeaderFlags {
    pub const SEALED: u16 = 1 << 0;
    pub const ENCRYPTED: u16 = 1 << 1;
    pub const I_JSON: u16 = 1 << 2;
    pub const KEY_REVOKED: u16 = 1 << 3;
    pub const HAS_BREV: u16 = 1 << 4;
    pub const HAS_ROLLUPS: u16 = 1 << 5;

    pub fn new() -> Self {
        Self(0)
    }

    pub fn is_sealed(&self) -> bool {
        self.0 & Self::SEALED != 0
    }

    pub fn is_encrypted(&self) -> bool {
        self.0 & Self::ENCRYPTED != 0
    }

    pub fn enforce_i_json(&self) -> bool {
        self.0 & Self::I_JSON != 0
    }

    pub fn is_key_revoked(&self) -> bool {
        self.0 & Self::KEY_REVOKED != 0
    }

    pub fn has_brev(&self) -> bool {
        self.0 & Self::HAS_BREV != 0
    }

    pub fn has_rollups(&self) -> bool {
        self.0 & Self::HAS_ROLLUPS != 0
    }

    pub fn set_sealed(&mut self) {
        self.0 |= Self::SEALED;
    }

    pub fn set_encrypted(&mut self) {
        self.0 |= Self::ENCRYPTED;
    }

    pub fn set_i_json(&mut self) {
        self.0 |= Self::I_JSON;
    }

    pub fn set_key_revoked(&mut self) {
        self.0 |= Self::KEY_REVOKED;
    }

    pub fn set_brev(&mut self) {
        self.0 |= Self::HAS_BREV;
    }

    pub fn set_rollups(&mut self) {
        self.0 |= Self::HAS_ROLLUPS;
    }
}

/// Algorithm identifiers
#[derive(Debug, Clone, Copy, AsBytes, FromBytes, FromZeroes)]
#[repr(C)]
pub struct AlgoIds {
    pub compression: u8,    // ZSTD=1
    pub aead: u8,          // CHACHA20POLY1305=1
    pub hash: u8,          // BLAKE3=1
    pub signature: u8,     // ED25519=1, DILITHIUM=2
}

impl Default for AlgoIds {
    fn default() -> Self {
        Self {
            compression: 1, // ZSTD
            aead: 1,       // ChaCha20-Poly1305
            hash: 1,       // BLAKE3
            signature: 1,  // Ed25519
        }
    }
}

/// ZJL fixed header (160 bytes, little-endian)
#[derive(Debug, Clone, AsBytes, FromBytes, FromZeroes)]
#[repr(C, packed)]
pub struct FixedHeader {
    /// Magic signature "ZJLK"
    pub magic: [u8; 4],
    /// Format version
    pub version: u16,
    /// Header flags
    pub flags: u16,
    /// Algorithm identifiers
    pub algo_ids: AlgoIds,
    /// File UUID
    pub file_uuid: [u8; 16],
    /// Creation timestamp (Unix seconds)
    pub created_unix_sec: u64,
    /// Offset to B+ tree root
    pub root_index_offset: u64,
    /// Offset to central directory
    pub central_dir_offset: u64,
    /// Offset to signatures
    pub signatures_offset: u64,
    /// Offset to tombstone (0 if none)
    pub tombstone_offset: u64,
    /// Reserved space (92 bytes)
    pub reserved: [u8; 92],
}

impl FixedHeader {
    /// Create a new header with default values
    pub fn new(file_uuid: Uuid) -> Self {
        let mut header = Self {
            magic: ZJL_MAGIC,
            version: ZJL_VERSION,
            flags: HeaderFlags::new().0,
            algo_ids: AlgoIds::default(),
            file_uuid: *file_uuid.as_bytes(),
            created_unix_sec: chrono::Utc::now().timestamp() as u64,
            root_index_offset: 0,
            central_dir_offset: 0,
            signatures_offset: 0,
            tombstone_offset: 0,
            reserved: [0; 92],
        };

        // Ensure we're exactly 160 bytes
        assert_eq!(std::mem::size_of::<FixedHeader>(), FIXED_HEADER_SIZE);
        header
    }

    /// Validate header magic and version
    pub fn validate(&self) -> ZjlResult<()> {
        if self.magic != ZJL_MAGIC {
            return Err(ZjlError::InvalidMagic);
        }

        if self.version != ZJL_VERSION {
            return Err(ZjlError::UnsupportedVersion(self.version));
        }

        Ok(())
    }

    /// Get header flags
    pub fn flags(&self) -> HeaderFlags {
        HeaderFlags(self.flags)
    }

    /// Set header flags
    pub fn set_flags(&mut self, flags: HeaderFlags) {
        self.flags = flags.0;
    }

    /// Get file UUID
    pub fn uuid(&self) -> Uuid {
        Uuid::from_bytes(self.file_uuid)
    }

    /// Get creation timestamp
    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_timestamp(self.created_unix_sec as i64, 0)
            .unwrap_or_else(chrono::Utc::now)
    }

    /// Check if file is sealed
    pub fn is_sealed(&self) -> bool {
        self.flags().is_sealed()
    }

    /// Check if file is encrypted
    pub fn is_encrypted(&self) -> bool {
        self.flags().is_encrypted()
    }

    /// Check if key is revoked
    pub fn is_key_revoked(&self) -> bool {
        self.flags().is_key_revoked()
    }

    /// Mark file as sealed
    pub fn seal(&mut self) {
        let mut flags = self.flags();
        flags.set_sealed();
        self.set_flags(flags);
    }

    /// Mark key as revoked
    pub fn revoke_key(&mut self) {
        let mut flags = self.flags();
        flags.set_key_revoked();
        self.set_flags(flags);
    }


}

/// Tombstone header for override-delete
#[derive(Debug, Clone, AsBytes, FromBytes, FromZeroes)]
#[repr(C, packed)]
pub struct TombstoneHeader {
    /// Magic "ZJTB"
    pub magic: [u8; 4],
    /// Version
    pub version: u16,
    /// Reason length
    pub reason_len: u16,
    /// Actor DID length
    pub actor_len: u16,
    /// Action code
    pub action: u16,
    /// Timestamp
    pub timestamp: u64,
    /// BPI ledger transaction hash
    pub bpi_log_tx: [u8; 32],
    /// Key ID length
    pub key_id_len: u16,
}

impl TombstoneHeader {
    pub const MAGIC: [u8; 4] = *b"ZJTB";

    pub fn new(reason_len: u16, actor_len: u16, action: u16, bpi_tx: [u8; 32], key_id_len: u16) -> Self {
        Self {
            magic: Self::MAGIC,
            version: 1,
            reason_len,
            actor_len,
            action,
            timestamp: chrono::Utc::now().timestamp() as u64,
            bpi_log_tx: bpi_tx,
            key_id_len,
        }
    }

    pub fn validate(&self) -> ZjlResult<()> {
        if self.magic != Self::MAGIC {
            return Err(ZjlError::InvalidMagic);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_header_size() {
        assert_eq!(std::mem::size_of::<FixedHeader>(), FIXED_HEADER_SIZE);
    }

    #[test]
    fn test_header_flags() {
        let mut flags = HeaderFlags::new();
        assert!(!flags.is_sealed());
        assert!(!flags.is_encrypted());

        flags.set_sealed();
        flags.set_encrypted();
        assert!(flags.is_sealed());
        assert!(flags.is_encrypted());
    }

    #[test]
    fn test_fixed_header_creation() {
        let uuid = Uuid::new_v4();
        let header = FixedHeader::new(uuid);

        assert_eq!(header.magic, ZJL_MAGIC);
        let version = header.version;
        assert_eq!(version, ZJL_VERSION);
        assert_eq!(header.uuid(), uuid);
        assert!(header.validate().is_ok());
    }

    #[test]
    fn test_tombstone_header() {
        let bpi_tx = [0u8; 32];
        let tombstone = TombstoneHeader::new(100, 50, 1, bpi_tx, 20);

        assert_eq!(tombstone.magic, TombstoneHeader::MAGIC);
        let reason_len = tombstone.reason_len;
        let actor_len = tombstone.actor_len;
        assert_eq!(reason_len, 100);
        assert_eq!(actor_len, 50);
        assert!(tombstone.validate().is_ok());
    }
}
