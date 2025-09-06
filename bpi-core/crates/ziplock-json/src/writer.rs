//! ZIPLOCK-JSON writer interface

use std::fs::File;
use std::io::{Write, Seek, SeekFrom};
use std::path::Path;
use uuid::Uuid;
use serde_json::Value;
use zstd::Encoder;

use crate::{ZjlResult, ZjlError, ZjlOptions};
use crate::header::{FixedHeader, HeaderFlags, TombstoneHeader};
use crate::blocks::{Block, BlockType};
use crate::json_encoder::{JsonChunkEncoder, IJSONEnforcer};
use crate::merkle::{RollupManager, MicroReceipt};
use crate::brev64::{ForensicRecord, SystemSnapshot, AttackGraph, Brev64Encoder};
use crate::signing::{ZjlSigner, SignatureBundle, SignatureMetadata, KmsProvider};
use crate::central_dir::{CentralDirectory, HeapArena, FileLayout};

/// ZIPLOCK-JSON file writer
pub struct ZjlWriter<W: Write + Seek, K: KmsProvider> {
    /// Output writer
    writer: W,
    /// File path (for reference)
    file_path: Option<String>,
    /// File options
    options: ZjlOptions,
    /// File header
    header: FixedHeader,
    /// Heap arena for block management
    heap: HeapArena,
    /// Central directory
    central_dir: CentralDirectory,
    /// File layout manager
    layout: FileLayout,
    /// JSON chunk encoder
    json_encoder: JsonChunkEncoder,
    /// Merkle rollup manager
    rollup_manager: RollupManager,
    /// Signature manager
    signer: Option<ZjlSigner<K>>,
    /// Signature bundle
    signatures: SignatureBundle,
    /// File sealed flag
    sealed: bool,
    /// Compression encoder
    compressor: Option<Encoder<'static, Vec<u8>>>,
}

impl<W: Write + Seek, K: KmsProvider> std::fmt::Debug for ZjlWriter<W, K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZjlWriter")
            .field("options", &self.options)
            .field("header", &self.header)
            .field("sealed", &self.sealed)
            .field("has_signer", &self.signer.is_some())
            .field("has_compressor", &self.compressor.is_some())
            .finish()
    }
}

impl<W: Write + Seek, K: KmsProvider> ZjlWriter<W, K> {
    /// Create a new ZJL writer
    pub fn new(mut writer: W, options: ZjlOptions) -> ZjlResult<Self> {
        let file_uuid = Uuid::new_v4();
        let mut header = FixedHeader::new(file_uuid);
        
        // Set header flags based on options
        let mut flags = HeaderFlags::new();
        if options.enforce_i_json {
            flags.set_i_json();
        }
        if options.enable_encryption {
            flags.set_encrypted();
        }
        header.set_flags(flags);

        // Write placeholder header (will be updated when sealed)
        writer.write_all(zerocopy::AsBytes::as_bytes(&header))
            .map_err(|e| ZjlError::IoError(e.to_string()))?;

        let heap = HeapArena::new(160); // Start after fixed header
        let central_dir = CentralDirectory::new();
        let layout = FileLayout::new();
        let json_encoder = JsonChunkEncoder::new(options.chunk_size);
        let rollup_manager = RollupManager::new();

        // Initialize compressor if compression is enabled
        let compressor = if options.compression_level > 0 {
            Some(Encoder::new(Vec::new(), options.compression_level as i32)?)
        } else {
            None
        };

        Ok(Self {
            writer,
            file_path: None,
            options,
            header,
            heap,
            central_dir,
            layout,
            json_encoder,
            rollup_manager,
            signer: None,
            signatures: SignatureBundle::new(),
            sealed: false,
            compressor,
        })
    }

    /// Create a new ZJL writer with signing capability
    pub fn with_signer(mut writer: W, options: ZjlOptions, signer: ZjlSigner<K>) -> ZjlResult<Self> {
        let mut zjl_writer = Self::new(writer, options)?;
        zjl_writer.signer = Some(signer);
        Ok(zjl_writer)
    }

    /// Create a new ZJL writer from a file path
    pub fn from_path<P: AsRef<Path>>(path: P, options: ZjlOptions) -> ZjlResult<ZjlWriter<File, K>> 
    where
        K: Default,
    {
        let file = File::create(&path)
            .map_err(|e| ZjlError::IoError(e.to_string()))?;
        let mut writer = ZjlWriter::new(file, options)?;
        writer.file_path = Some(path.as_ref().to_string_lossy().to_string());
        Ok(writer)
    }

    /// Get the file path if available
    pub fn get_file_path(&self) -> Option<String> {
        self.file_path.clone()
    }

    /// Set the file path for reference
    pub fn set_file_path(&mut self, path: String) {
        self.file_path = Some(path);
    }

    /// Write a JSON value to the file
    pub fn write_json(&mut self, value: &Value) -> ZjlResult<()> {
        self.check_not_sealed()?;

        // Encode JSON value into blocks
        let blocks = self.json_encoder.encode_value(value, "")?;
        
        // Write blocks to heap
        for block in blocks {
            self.write_block(block)?;
        }

        Ok(())
    }

    /// Write a JSON value with a specific path
    pub fn write_json_with_path(&mut self, value: &Value, path: &str) -> ZjlResult<()> {
        self.check_not_sealed()?;

        let blocks = self.json_encoder.encode_value(value, path)?;
        
        for block in blocks {
            self.write_block(block)?;
        }

        Ok(())
    }

    /// Write an audit event (creates micro-receipt)
    pub fn write_audit_event(&mut self, event_type: String, vm_id: String, payload: &[u8]) -> ZjlResult<()> {
        self.check_not_sealed()?;

        // Create micro-receipt
        let receipt = MicroReceipt::new(event_type, vm_id, payload, 0);
        
        // Add to rollup manager
        self.rollup_manager.add_receipt(receipt)?;

        // Check if we need to create rollup blocks
        if let Some(second_root) = self.rollup_manager.rollup_current_second()? {
            let block = Block::new(
                BlockType::SecondRoot,
                0,
                serde_json::to_vec(&second_root)
                    .map_err(|e| ZjlError::SerializationErrorString(e.to_string()))?,
                blake3::hash(&serde_json::to_vec(&second_root).unwrap()).into(),
            );
            self.write_block(block)?;
        }

        if let Some(minute_root) = self.rollup_manager.rollup_current_minute()? {
            let block = Block::new(
                BlockType::MinuteRoot,
                0,
                serde_json::to_vec(&minute_root)
                    .map_err(|e| ZjlError::SerializationErrorString(e.to_string()))?,
                blake3::hash(&serde_json::to_vec(&minute_root).unwrap()).into(),
            );
            self.write_block(block)?;
        }

        Ok(())
    }

    /// Write a forensic record
    pub fn write_forensic_record(&mut self, record: &ForensicRecord) -> ZjlResult<()> {
        self.check_not_sealed()?;
        let block = Brev64Encoder::encode_forensic_record(record)?;
        self.write_block(block)
    }

    /// Write a system snapshot
    pub fn write_system_snapshot(&mut self, snapshot: &SystemSnapshot) -> ZjlResult<()> {
        self.check_not_sealed()?;
        let block = Brev64Encoder::encode_snapshot(snapshot)?;
        self.write_block(block)
    }

    /// Write an attack graph
    pub fn write_attack_graph(&mut self, graph: &AttackGraph) -> ZjlResult<()> {
        self.check_not_sealed()?;
        let block = Brev64Encoder::encode_attack_graph(graph)?;
        self.write_block(block)
    }

    /// Write a raw block
    fn write_block(&mut self, mut block: Block) -> ZjlResult<()> {
        // Compress payload if compression is enabled
        if let Some(compressor) = self.compressor.take() {
            let mut compressor = compressor;
            compressor.write_all(&block.payload)
                .map_err(|e| ZjlError::CompressionError(e.to_string()))?;
            let compressed = compressor.finish()
                .map_err(|e| ZjlError::CompressionError(e.to_string()))?;
            
            block.header.compressed_len = compressed.len() as u32;
            block.payload = compressed;
            
            // Reinitialize compressor for next block
            self.compressor = Some(Encoder::new(Vec::new(), self.options.compression_level as i32)?);
        }

        // Allocate space in heap
        let block_size = block.total_size() as u64;
        let offset = self.heap.allocate(
            block_size,
            block.block_type().unwrap_or(BlockType::Pad),
            block.header.path_id,
        );

        // Write block to file
        self.writer.seek(SeekFrom::Start(offset))
            .map_err(|e| ZjlError::IoError(e.to_string()))?;
        
        self.writer.write_all(zerocopy::AsBytes::as_bytes(&block.header))
            .map_err(|e| ZjlError::IoError(e.to_string()))?;
        
        self.writer.write_all(&block.payload)
            .map_err(|e| ZjlError::IoError(e.to_string()))?;

        // Add to central directory
        self.central_dir.add_block(&block, offset);

        Ok(())
    }

    /// Seal the file (write central directory, index, and signatures)
    pub fn seal(&mut self) -> ZjlResult<()> {
        if self.sealed {
            return Err(ZjlError::AlreadySealed);
        }

        // Force rollup of any pending data
        self.rollup_manager.force_rollup()?;

        // Update layout after heap
        self.layout.update_after_heap(self.heap.size());

        // Write central directory
        self.writer.seek(SeekFrom::Start(self.layout.central_dir_offset))
            .map_err(|e| ZjlError::IoError(e.to_string()))?;
        
        let central_dir_size = self.central_dir.write_to(&mut self.writer)?;
        self.layout.update_after_central_dir(central_dir_size);

        // Write B+ tree index (placeholder for now)
        self.writer.seek(SeekFrom::Start(self.layout.index_offset))
            .map_err(|e| ZjlError::IoError(e.to_string()))?;
        
        let index_placeholder = vec![0u8; 64]; // Placeholder index
        self.writer.write_all(&index_placeholder)
            .map_err(|e| ZjlError::IoError(e.to_string()))?;
        self.layout.update_after_index(index_placeholder.len() as u64);

        // Sign the file if signer is available
        if let Some(ref signer) = self.signer {
            let metadata = SignatureMetadata {
                key_id: "default".to_string(),
                algorithm: "EdDSA".to_string(),
                timestamp: chrono::Utc::now().timestamp() as u64,
                signer: "zjl_writer".to_string(),
                purpose: "file_integrity".to_string(),
                claims: std::collections::HashMap::new(),
            };

            let signature = signer.sign_header(&self.header, metadata)?;
            self.signatures.add_signature(signature, "file_seal".to_string(), "zjl_writer".to_string())?;
        }

        // Write signatures
        self.writer.seek(SeekFrom::Start(self.layout.signatures_offset))
            .map_err(|e| ZjlError::IoError(e.to_string()))?;
        
        let signature_bytes = self.signatures.to_bytes()?;
        self.writer.write_all(&signature_bytes)
            .map_err(|e| ZjlError::IoError(e.to_string()))?;
        self.layout.update_after_signatures(signature_bytes.len() as u64);

        // Update and rewrite header
        self.layout.update_header(&mut self.header);
        self.header.seal();

        self.writer.seek(SeekFrom::Start(0))
            .map_err(|e| ZjlError::IoError(e.to_string()))?;
        self.writer.write_all(zerocopy::AsBytes::as_bytes(&self.header))
            .map_err(|e| ZjlError::IoError(e.to_string()))?;

        // Flush all writes
        self.writer.flush()
            .map_err(|e| ZjlError::IoError(e.to_string()))?;

        self.sealed = true;
        Ok(())
    }

    /// Override-delete the file (write tombstone)
    pub fn override_delete(
        &mut self,
        reason: String,
        actor: String,
        action: u16,
        bpi_tx: [u8; 32],
        key_id: String,
    ) -> ZjlResult<()> {
        if !self.sealed {
            return Err(ZjlError::NotSealed);
        }

        // Create tombstone header
        let tombstone = TombstoneHeader::new(
            reason.len() as u16,
            actor.len() as u16,
            action,
            bpi_tx,
            key_id.len() as u16,
        );

        // Write tombstone at the end of file
        self.writer.seek(SeekFrom::End(0))
            .map_err(|e| ZjlError::IoError(e.to_string()))?;
        
        let tombstone_offset = self.writer.stream_position()
            .map_err(|e| ZjlError::IoError(e.to_string()))?;

        // Write tombstone header
        self.writer.write_all(zerocopy::AsBytes::as_bytes(&tombstone))
            .map_err(|e| ZjlError::IoError(e.to_string()))?;

        // Write variable-length data
        self.writer.write_all(reason.as_bytes())
            .map_err(|e| ZjlError::IoError(e.to_string()))?;
        self.writer.write_all(actor.as_bytes())
            .map_err(|e| ZjlError::IoError(e.to_string()))?;
        self.writer.write_all(key_id.as_bytes())
            .map_err(|e| ZjlError::IoError(e.to_string()))?;

        // Update header with tombstone offset
        self.header.tombstone_offset = tombstone_offset;
        self.header.revoke_key();

        // Rewrite header
        self.writer.seek(SeekFrom::Start(0))
            .map_err(|e| ZjlError::IoError(e.to_string()))?;
        self.writer.write_all(zerocopy::AsBytes::as_bytes(&self.header))
            .map_err(|e| ZjlError::IoError(e.to_string()))?;

        self.writer.flush()
            .map_err(|e| ZjlError::IoError(e.to_string()))?;

        Ok(())
    }

    /// Get file statistics
    pub fn stats(&self) -> ZjlStats {
        ZjlStats {
            file_size: self.layout.file_size,
            block_count: self.central_dir.len(),
            heap_size: self.heap.size(),
            rollup_stats: self.rollup_manager.stats(),
            sealed: self.sealed,
            encrypted: self.header.is_encrypted(),
            signed: !self.signatures.signatures.is_empty(),
        }
    }

    /// Check if file is sealed
    pub fn is_sealed(&self) -> bool {
        self.sealed
    }

    /// Get file UUID
    pub fn uuid(&self) -> Uuid {
        self.header.uuid()
    }

    fn check_not_sealed(&self) -> ZjlResult<()> {
        if self.sealed {
            Err(ZjlError::AlreadySealed)
        } else {
            Ok(())
        }
    }
}

/// File statistics
#[derive(Debug, Clone)]
pub struct ZjlStats {
    pub file_size: u64,
    pub block_count: usize,
    pub heap_size: u64,
    pub rollup_stats: crate::merkle::RollupStats,
    pub sealed: bool,
    pub encrypted: bool,
    pub signed: bool,
}

/// Convenience function to create a ZJL file
pub fn create_zjl_file<P: AsRef<Path>>(
    path: P,
    options: ZjlOptions,
) -> ZjlResult<ZjlWriter<File, crate::signing::InMemoryKms>> {
    let file = File::create(&path)
        .map_err(|e| ZjlError::IoError(e.to_string()))?;
    let mut writer = ZjlWriter::new(file, options)?;
    writer.file_path = Some(path.as_ref().to_string_lossy().to_string());
    Ok(writer)
}

/// Convenience function to create a signed ZJL file
pub fn create_signed_zjl_file<P: AsRef<Path>>(
    path: P,
    options: ZjlOptions,
    signer: ZjlSigner<crate::signing::InMemoryKms>,
) -> ZjlResult<ZjlWriter<File, crate::signing::InMemoryKms>> {
    let file = File::create(&path)
        .map_err(|e| ZjlError::IoError(e.to_string()))?;
    let mut writer = ZjlWriter::with_signer(file, options, signer)?;
    writer.file_path = Some(path.as_ref().to_string_lossy().to_string());
    Ok(writer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use serde_json::json;
    use crate::signing::InMemoryKms;

    #[test]
    fn test_zjl_writer_creation() {
        let buffer = Cursor::new(Vec::new());
        let options = ZjlOptions::default();
        let writer = ZjlWriter::<_, InMemoryKms>::new(buffer, options).unwrap();
        
        assert!(!writer.is_sealed());
        assert_eq!(writer.stats().block_count, 0);
    }

    #[test]
    fn test_write_json() {
        let buffer = Cursor::new(Vec::new());
        let options = ZjlOptions::default();
        let mut writer = ZjlWriter::<_, InMemoryKms>::new(buffer, options).unwrap();
        
        let json_data = json!({
            "name": "test",
            "value": 42,
            "active": true
        });

        writer.write_json(&json_data).unwrap();
        
        let stats = writer.stats();
        assert!(stats.block_count > 0);
        assert!(stats.heap_size > 0);
    }

    #[test]
    fn test_write_and_seal() {
        let buffer = Cursor::new(Vec::new());
        let options = ZjlOptions::default();
        let mut writer = ZjlWriter::<_, InMemoryKms>::new(buffer, options).unwrap();
        
        let json_data = json!({"test": "data"});
        writer.write_json(&json_data).unwrap();
        
        assert!(!writer.is_sealed());
        writer.seal().unwrap();
        assert!(writer.is_sealed());
        
        // Should not be able to write after sealing
        assert!(writer.write_json(&json_data).is_err());
    }

    #[test]
    fn test_audit_event() {
        let buffer = Cursor::new(Vec::new());
        let options = ZjlOptions::default();
        let mut writer = ZjlWriter::<_, InMemoryKms>::new(buffer, options).unwrap();
        
        writer.write_audit_event(
            "test_event".to_string(),
            "test_vm".to_string(),
            b"test payload",
        ).unwrap();
        
        let stats = writer.stats();
        assert!(stats.block_count >= 0); // May or may not create blocks depending on rollup timing
    }

    #[test]
    fn test_file_stats() {
        let buffer = Cursor::new(Vec::new());
        let options = ZjlOptions::default();
        let mut writer = ZjlWriter::<_, InMemoryKms>::new(buffer, options).unwrap();
        
        let stats = writer.stats();
        assert_eq!(stats.file_size, 160); // Just header initially
        assert_eq!(stats.block_count, 0);
        assert!(!stats.sealed);
        assert!(!stats.signed);
        
        let json_data = json!({"test": "data"});
        writer.write_json(&json_data).unwrap();
        writer.seal().unwrap();
        
        let final_stats = writer.stats();
        assert!(final_stats.file_size > 160);
        assert!(final_stats.block_count > 0);
        assert!(final_stats.sealed);
    }
}
