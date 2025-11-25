//! CBOR Serialization for ZipLock JSON
//! 
//! Implements CBOR (Concise Binary Object Representation) serialization for BPCI pipeline integration
//! Features: Compact binary encoding, XTMP protocol compatibility, efficient audit bundle transmission

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use ciborium::{ser, de};
use std::io::{Read, Write, Cursor};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// CBOR serialization options for ZipLock JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CborOptions {
    /// Use canonical CBOR encoding
    pub canonical: bool,
    /// Maximum nesting depth
    pub max_depth: u32,
    /// Enable compression
    pub compress: bool,
    /// Include metadata headers
    pub include_metadata: bool,
    /// XTMP protocol version
    pub xtmp_version: String,
}

impl Default for CborOptions {
    fn default() -> Self {
        Self {
            canonical: true,
            max_depth: 64,
            compress: true,
            include_metadata: true,
            xtmp_version: "1.0".to_string(),
        }
    }
}

/// CBOR-encoded audit bundle for BPCI pipeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CborAuditBundle {
    /// Bundle metadata
    pub metadata: CborBundleMetadata,
    /// Serialized audit data
    pub audit_data: Vec<u8>,
    /// CBOR encoding info
    pub encoding_info: CborEncodingInfo,
    /// Integrity verification
    pub integrity: CborIntegrityData,
}

/// Metadata for CBOR audit bundles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CborBundleMetadata {
    /// Bundle ID
    pub bundle_id: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Original size before CBOR encoding
    pub original_size: u64,
    /// CBOR encoded size
    pub encoded_size: u64,
    /// Compression ratio
    pub compression_ratio: f64,
    /// XTMP protocol compatibility
    pub xtmp_compatible: bool,
    /// BPCI pipeline version
    pub bpci_version: String,
}

/// CBOR encoding information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CborEncodingInfo {
    /// CBOR specification version
    pub cbor_version: String,
    /// Encoding options used
    pub options: CborOptions,
    /// Canonical form used
    pub canonical_form: bool,
    /// Encoding timestamp
    pub encoded_at: DateTime<Utc>,
    /// Encoder version
    pub encoder_version: String,
}

/// Integrity data for CBOR bundles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CborIntegrityData {
    /// CBOR data hash
    pub cbor_hash: String,
    /// Original data hash
    pub original_hash: String,
    /// Merkle root of encoded data
    pub merkle_root: String,
    /// Digital signature
    pub signature: Option<Vec<u8>>,
    /// Verification timestamp
    pub verified_at: Option<DateTime<Utc>>,
}

/// CBOR serializer for ZipLock JSON audit data
pub struct CborSerializer {
    /// Serialization options
    options: CborOptions,
    /// Encoding statistics
    stats: CborStats,
}

/// CBOR encoding statistics
#[derive(Debug, Clone, Default)]
pub struct CborStats {
    /// Total bundles encoded
    pub bundles_encoded: u64,
    /// Total bytes encoded
    pub total_bytes_encoded: u64,
    /// Total compression savings
    pub compression_savings: u64,
    /// Average encoding time (ms)
    pub avg_encoding_time_ms: f64,
    /// XTMP compatibility rate
    pub xtmp_compatibility_rate: f64,
}

impl CborSerializer {
    /// Create new CBOR serializer
    pub fn new(options: CborOptions) -> Self {
        Self {
            options,
            stats: CborStats::default(),
        }
    }

    /// Serialize audit data to CBOR format
    pub fn serialize_audit_bundle<T>(&mut self, data: &T) -> Result<CborAuditBundle>
    where
        T: Serialize,
    {
        let start_time = std::time::Instant::now();
        
        // Serialize to JSON first for size comparison
        let json_data = serde_json::to_vec(data)
            .map_err(|e| anyhow!("JSON serialization failed: {}", e))?;
        let original_size = json_data.len() as u64;

        // Serialize to CBOR
        let mut cbor_data = Vec::new();
        ser::into_writer(data, &mut cbor_data)
            .map_err(|e| anyhow!("CBOR serialization failed: {}", e))?;

        // Apply compression if enabled
        let final_data = if self.options.compress {
            self.compress_cbor_data(&cbor_data)?
        } else {
            cbor_data
        };

        let encoded_size = final_data.len() as u64;
        let compression_ratio = if original_size > 0 {
            encoded_size as f64 / original_size as f64
        } else {
            1.0
        };

        // Generate integrity data
        let integrity = self.generate_integrity_data(&final_data, &json_data)?;

        // Create bundle metadata
        let metadata = CborBundleMetadata {
            bundle_id: format!("cbor_{}", Utc::now().timestamp_nanos_opt().unwrap_or(0)),
            created_at: Utc::now(),
            original_size,
            encoded_size,
            compression_ratio,
            xtmp_compatible: self.verify_xtmp_compatibility(&final_data)?,
            bpci_version: "2.0".to_string(),
        };

        // Create encoding info
        let encoding_info = CborEncodingInfo {
            cbor_version: "RFC 8949".to_string(),
            options: self.options.clone(),
            canonical_form: self.options.canonical,
            encoded_at: Utc::now(),
            encoder_version: "ziplock-json-1.0".to_string(),
        };

        // Update statistics
        let encoding_time = start_time.elapsed().as_millis() as f64;
        self.update_stats(original_size, encoded_size, encoding_time, metadata.xtmp_compatible);

        Ok(CborAuditBundle {
            metadata,
            audit_data: final_data,
            encoding_info,
            integrity,
        })
    }

    /// Deserialize CBOR audit bundle
    pub fn deserialize_audit_bundle<T>(&self, bundle: &CborAuditBundle) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        // Verify integrity first
        self.verify_bundle_integrity(bundle)?;

        // Decompress if needed
        let cbor_data = if self.options.compress {
            self.decompress_cbor_data(&bundle.audit_data)?
        } else {
            bundle.audit_data.clone()
        };

        // Deserialize from CBOR
        let mut cursor = Cursor::new(cbor_data);
        de::from_reader(&mut cursor)
            .map_err(|e| anyhow!("CBOR deserialization failed: {}", e))
    }

    /// Compress CBOR data using zstd
    fn compress_cbor_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        zstd::encode_all(data, 6)
            .map_err(|e| anyhow!("CBOR compression failed: {}", e))
    }

    /// Decompress CBOR data
    fn decompress_cbor_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        zstd::decode_all(data)
            .map_err(|e| anyhow!("CBOR decompression failed: {}", e))
    }

    /// Generate integrity data for CBOR bundle
    fn generate_integrity_data(&self, cbor_data: &[u8], original_data: &[u8]) -> Result<CborIntegrityData> {
        use blake3::Hasher;

        let mut hasher = Hasher::new();
        hasher.update(cbor_data);
        let cbor_hash = hex::encode(hasher.finalize().as_bytes());

        let mut hasher = Hasher::new();
        hasher.update(original_data);
        let original_hash = hex::encode(hasher.finalize().as_bytes());

        // Generate Merkle root (simplified)
        let mut hasher = Hasher::new();
        hasher.update(&cbor_hash.as_bytes());
        hasher.update(&original_hash.as_bytes());
        let merkle_root = hex::encode(hasher.finalize().as_bytes());

        Ok(CborIntegrityData {
            cbor_hash,
            original_hash,
            merkle_root,
            signature: None, // Would be added by quantum-safe crypto module
            verified_at: None,
        })
    }

    /// Verify XTMP protocol compatibility
    fn verify_xtmp_compatibility(&self, data: &[u8]) -> Result<bool> {
        // Check CBOR structure for XTMP compatibility
        // XTMP requires specific CBOR tag structures
        
        if data.is_empty() {
            return Ok(false);
        }

        // Check for valid CBOR header
        let first_byte = data[0];
        let major_type = (first_byte >> 5) & 0x07;
        
        // XTMP expects map or array at root level
        match major_type {
            4 => Ok(true), // Array
            5 => Ok(true), // Map
            _ => Ok(false),
        }
    }

    /// Verify bundle integrity
    fn verify_bundle_integrity(&self, bundle: &CborAuditBundle) -> Result<()> {
        use blake3::Hasher;

        // Verify CBOR data hash
        let mut hasher = Hasher::new();
        hasher.update(&bundle.audit_data);
        let computed_hash = hex::encode(hasher.finalize().as_bytes());

        if computed_hash != bundle.integrity.cbor_hash {
            return Err(anyhow!("CBOR data integrity verification failed"));
        }

        // Verify Merkle root
        let mut hasher = Hasher::new();
        hasher.update(&bundle.integrity.cbor_hash.as_bytes());
        hasher.update(&bundle.integrity.original_hash.as_bytes());
        let computed_merkle = hex::encode(hasher.finalize().as_bytes());

        if computed_merkle != bundle.integrity.merkle_root {
            return Err(anyhow!("Merkle root verification failed"));
        }

        Ok(())
    }

    /// Update encoding statistics
    fn update_stats(&mut self, original_size: u64, encoded_size: u64, encoding_time: f64, xtmp_compatible: bool) {
        self.stats.bundles_encoded += 1;
        self.stats.total_bytes_encoded += encoded_size;
        
        if original_size > encoded_size {
            self.stats.compression_savings += original_size - encoded_size;
        }

        // Update average encoding time
        let total_time = self.stats.avg_encoding_time_ms * (self.stats.bundles_encoded - 1) as f64 + encoding_time;
        self.stats.avg_encoding_time_ms = total_time / self.stats.bundles_encoded as f64;

        // Update XTMP compatibility rate
        let compatible_count = if xtmp_compatible { 1.0 } else { 0.0 };
        let total_compatible = self.stats.xtmp_compatibility_rate * (self.stats.bundles_encoded - 1) as f64 + compatible_count;
        self.stats.xtmp_compatibility_rate = total_compatible / self.stats.bundles_encoded as f64;
    }

    /// Get encoding statistics
    pub fn get_stats(&self) -> &CborStats {
        &self.stats
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.stats = CborStats::default();
    }
}

/// CBOR-based XTMP protocol handler
pub struct XtmpCborHandler {
    /// CBOR serializer
    serializer: CborSerializer,
    /// XTMP protocol version
    protocol_version: String,
    /// Active sessions
    sessions: HashMap<String, XtmpSession>,
}

/// XTMP session data
#[derive(Debug, Clone)]
pub struct XtmpSession {
    /// Session ID
    pub session_id: String,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last activity
    pub last_activity: DateTime<Utc>,
    /// Bundles processed
    pub bundles_processed: u64,
    /// Total bytes transferred
    pub bytes_transferred: u64,
}

impl XtmpCborHandler {
    /// Create new XTMP CBOR handler
    pub fn new(cbor_options: CborOptions) -> Self {
        Self {
            serializer: CborSerializer::new(cbor_options),
            protocol_version: "XTMP/1.0".to_string(),
            sessions: HashMap::new(),
        }
    }

    /// Process audit bundle for XTMP transmission
    pub fn process_for_xtmp<T>(&mut self, session_id: &str, data: &T) -> Result<Vec<u8>>
    where
        T: Serialize,
    {
        // Create or update session
        self.update_session(session_id);

        // Serialize to CBOR
        let bundle = self.serializer.serialize_audit_bundle(data)?;

        // Verify XTMP compatibility
        if !bundle.metadata.xtmp_compatible {
            return Err(anyhow!("Bundle is not XTMP compatible"));
        }

        // Create XTMP frame
        let xtmp_frame = self.create_xtmp_frame(session_id, &bundle)?;

        // Update session stats
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.bundles_processed += 1;
            session.bytes_transferred += xtmp_frame.len() as u64;
            session.last_activity = Utc::now();
        }

        Ok(xtmp_frame)
    }

    /// Create XTMP protocol frame
    fn create_xtmp_frame(&self, session_id: &str, bundle: &CborAuditBundle) -> Result<Vec<u8>> {
        #[derive(Serialize)]
        struct XtmpFrame {
            protocol: String,
            session_id: String,
            timestamp: DateTime<Utc>,
            bundle_id: String,
            data: Vec<u8>,
        }

        let frame = XtmpFrame {
            protocol: self.protocol_version.clone(),
            session_id: session_id.to_string(),
            timestamp: Utc::now(),
            bundle_id: bundle.metadata.bundle_id.clone(),
            data: bundle.audit_data.clone(),
        };

        let mut frame_data = Vec::new();
        ser::into_writer(&frame, &mut frame_data)
            .map_err(|e| anyhow!("XTMP frame serialization failed: {}", e))?;

        Ok(frame_data)
    }

    /// Update session information
    fn update_session(&mut self, session_id: &str) {
        let session = self.sessions.entry(session_id.to_string()).or_insert_with(|| {
            XtmpSession {
                session_id: session_id.to_string(),
                created_at: Utc::now(),
                last_activity: Utc::now(),
                bundles_processed: 0,
                bytes_transferred: 0,
            }
        });
        session.last_activity = Utc::now();
    }

    /// Get session statistics
    pub fn get_session_stats(&self, session_id: &str) -> Option<&XtmpSession> {
        self.sessions.get(session_id)
    }

    /// Clean up old sessions
    pub fn cleanup_old_sessions(&mut self, max_age_hours: i64) {
        let cutoff = Utc::now() - chrono::Duration::hours(max_age_hours);
        self.sessions.retain(|_, session| session.last_activity > cutoff);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_cbor_serialization() {
        let mut serializer = CborSerializer::new(CborOptions::default());
        
        let test_data = json!({
            "audit_id": "test_123",
            "timestamp": "2025-11-11T17:00:00Z",
            "events": [
                {"type": "login", "user": "alice"},
                {"type": "file_access", "file": "/secure/data.txt"}
            ]
        });

        let bundle = serializer.serialize_audit_bundle(&test_data).unwrap();
        
        assert!(!bundle.audit_data.is_empty());
        assert!(bundle.metadata.xtmp_compatible);
        assert!(bundle.metadata.compression_ratio <= 1.0);
        
        // Test deserialization
        let deserialized: serde_json::Value = serializer.deserialize_audit_bundle(&bundle).unwrap();
        assert_eq!(deserialized["audit_id"], "test_123");
    }

    #[test]
    fn test_xtmp_compatibility() {
        let serializer = CborSerializer::new(CborOptions::default());
        
        // Test with valid CBOR array
        let array_data = vec![0x84, 0x01, 0x02, 0x03, 0x04]; // CBOR array [1,2,3,4]
        assert!(serializer.verify_xtmp_compatibility(&array_data).unwrap());
        
        // Test with valid CBOR map
        let map_data = vec![0xa2, 0x61, 0x61, 0x01, 0x61, 0x62, 0x02]; // CBOR map {"a":1,"b":2}
        assert!(serializer.verify_xtmp_compatibility(&map_data).unwrap());
        
        // Test with invalid data
        let invalid_data = vec![0x00]; // CBOR unsigned integer
        assert!(!serializer.verify_xtmp_compatibility(&invalid_data).unwrap());
    }

    #[test]
    fn test_xtmp_handler() {
        let mut handler = XtmpCborHandler::new(CborOptions::default());
        
        let test_data = json!({"test": "data"});
        let frame = handler.process_for_xtmp("session_1", &test_data).unwrap();
        
        assert!(!frame.is_empty());
        
        let session_stats = handler.get_session_stats("session_1").unwrap();
        assert_eq!(session_stats.bundles_processed, 1);
        assert!(session_stats.bytes_transferred > 0);
    }

    #[test]
    fn test_compression() {
        let mut serializer = CborSerializer::new(CborOptions {
            compress: true,
            ..Default::default()
        });
        
        // Create large test data
        let large_data = json!({
            "repeated_data": vec!["same_string"; 1000],
            "numbers": (0..1000).collect::<Vec<i32>>()
        });

        let bundle = serializer.serialize_audit_bundle(&large_data).unwrap();
        
        // Should achieve good compression on repetitive data
        assert!(bundle.metadata.compression_ratio < 0.5);
    }
}
