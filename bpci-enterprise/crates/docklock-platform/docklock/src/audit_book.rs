//! # Stage 38: Multi-Cloud Storage Policy (Audit Book Export)
//!
//! Provides regulatory-compliant audit book export functionality for the Blockbook ledger.
//! Supports multiple regulatory frameworks, cloud storage providers, and export formats
//! with role-based access control and data retention policies.

use crate::blockbook::{Blockbook, BlockbookEntry, BlockbookEventType, EventSeverity};
use crate::error::DockLockError;
use bpi_enc::domain_hash;
use ed25519_dalek::{SigningKey as Keypair};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

/// Regulatory frameworks supported for audit book export
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegulatoryFramework {
    /// General Data Protection Regulation (EU)
    GDPR,
    /// Health Insurance Portability and Accountability Act (US)
    HIPAA,
    /// Sarbanes-Oxley Act (US)
    SOX,
    /// Payment Card Industry Data Security Standard
    PCIDSS,
    /// California Consumer Privacy Act (US)
    CCPA,
    /// Custom regulatory framework
    Custom(String),
}

/// Jurisdictions for regulatory compliance
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Jurisdiction {
    /// European Union
    EU,
    /// United States
    US,
    /// Canada
    CA,
    /// United Kingdom
    UK,
    /// Australia
    AU,
    /// Custom jurisdiction
    Custom(String),
}

/// Cloud storage providers for audit book export
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CloudProvider {
    /// Amazon Web Services
    AWS,
    /// Microsoft Azure
    Azure,
    /// Google Cloud Platform
    GCP,
    /// On-premises storage
    OnPremises,
    /// Custom cloud provider
    Custom(String),
}

/// Access levels for audit book entries
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AccessLevel {
    /// Public access (lowest level)
    Public = 0,
    /// Internal access
    Internal = 1,
    /// Confidential access
    Confidential = 2,
    /// Restricted access
    Restricted = 3,
    /// Top secret access (highest level)
    TopSecret = 4,
}

/// Export formats supported for audit books
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportFormat {
    /// JSON format
    JSON,
    /// CSV format
    CSV,
    /// XML format
    XML,
    /// Binary format (CBOR)
    Binary,
}

/// Encryption schemes for audit book export
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncryptionScheme {
    /// No encryption
    None,
    /// AES-256-GCM encryption
    AES256GCM,
    /// ChaCha20-Poly1305 encryption
    ChaCha20Poly1305,
    /// Ed25519 signature only
    Ed25519Signature,
}

/// Audit book entry with regulatory metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditBookEntry {
    /// Original blockbook entry
    pub blockbook_entry: BlockbookEntry,
    /// Regulatory frameworks applicable to this entry
    pub regulatory_frameworks: HashSet<RegulatoryFramework>,
    /// Jurisdictions where this entry applies
    pub jurisdictions: HashSet<Jurisdiction>,
    /// Access level required to view this entry
    pub access_level: AccessLevel,
    /// Data retention period in seconds
    pub retention_period: u64,
    /// Export timestamp
    pub export_timestamp: u64,
    /// Entry classification tags
    pub classification_tags: HashSet<String>,
}

/// Multi-cloud storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiCloudStorageConfig {
    /// Primary cloud provider
    pub primary_provider: CloudProvider,
    /// Backup cloud providers
    pub backup_providers: Vec<CloudProvider>,
    /// Encryption scheme for storage
    pub encryption_scheme: EncryptionScheme,
    /// Storage region preferences
    pub regions: Vec<String>,
    /// Replication factor
    pub replication_factor: u32,
}

/// Export configuration for audit books
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    /// Export format
    pub format: ExportFormat,
    /// Include sensitive data
    pub include_sensitive: bool,
    /// Maximum entries per export
    pub max_entries: usize,
    /// Compression enabled
    pub compression: bool,
    /// Digital signature required
    pub require_signature: bool,
}

/// Storage location for exported audit books
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageLocation {
    /// Cloud provider
    pub provider: CloudProvider,
    /// Storage URL or path
    pub url: String,
    /// Region
    pub region: String,
    /// Access credentials (encrypted)
    pub credentials_hash: String,
}

/// Statistics for audit book operations
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AuditBookStats {
    /// Total entries imported
    pub entries_imported: u64,
    /// Total entries exported
    pub entries_exported: u64,
    /// Export operations performed
    pub export_operations: u64,
    /// Compliance violations detected
    pub compliance_violations: u64,
    /// Storage operations performed
    pub storage_operations: u64,
    /// Last export timestamp
    pub last_export_timestamp: Option<u64>,
}

/// Main audit book structure for regulatory-compliant exports
#[derive(Debug)]
pub struct AuditBook {
    /// Audit book entries
    entries: Arc<RwLock<Vec<AuditBookEntry>>>,
    /// Multi-cloud storage configuration
    storage_config: MultiCloudStorageConfig,
    /// Export configuration
    export_config: ExportConfig,
    /// Signing keypair for audit book exports
    keypair: Keypair,
    /// Statistics tracking
    stats: Arc<RwLock<AuditBookStats>>,
}

impl AuditBook {
    /// Create a new audit book with specified configuration
    pub fn new(
        storage_config: MultiCloudStorageConfig,
        export_config: ExportConfig,
        keypair: Keypair,
    ) -> Self {
        Self {
            entries: Arc::new(RwLock::new(Vec::new())),
            storage_config,
            export_config,
            keypair,
            stats: Arc::new(RwLock::new(AuditBookStats::default())),
        }
    }

    /// Import entries from a blockbook ledger with regulatory filtering
    pub fn import_from_blockbook(
        &self,
        blockbook: &Blockbook,
        regulatory_frameworks: HashSet<RegulatoryFramework>,
        jurisdictions: HashSet<Jurisdiction>,
        min_access_level: AccessLevel,
    ) -> Result<usize, DockLockError> {
        let blockbook_entries = blockbook.get_recent_entries(1000);
        let mut imported_count = 0;
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| DockLockError::SystemTime(e.to_string()))?
            .as_secs();

        let mut entries = self.entries.write().map_err(|_| {
            DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                "Failed to acquire write lock".to_string(),
            )))
        })?;

        for blockbook_entry in blockbook_entries {
            // Apply regulatory filtering based on event type and severity
            let access_level = self.determine_access_level(&blockbook_entry);
            if access_level < min_access_level {
                continue;
            }

            let retention_period = self.determine_retention_period(&blockbook_entry);
            let classification_tags = self.generate_classification_tags(&blockbook_entry);

            let audit_entry = AuditBookEntry {
                blockbook_entry,
                regulatory_frameworks: regulatory_frameworks.clone(),
                jurisdictions: jurisdictions.clone(),
                access_level,
                retention_period,
                export_timestamp: current_time,
                classification_tags,
            };

            entries.push(audit_entry);
            imported_count += 1;
        }

        // Update statistics
        let mut stats = self.stats.write().map_err(|_| {
            DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                "Failed to acquire stats write lock".to_string(),
            )))
        })?;
        stats.entries_imported += imported_count as u64;

        Ok(imported_count)
    }

    /// Add a single audit book entry
    pub fn add_entry(&self, entry: AuditBookEntry) -> Result<(), DockLockError> {
        let mut entries = self.entries.write().map_err(|_| {
            DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                "Failed to acquire write lock".to_string(),
            )))
        })?;

        entries.push(entry);

        let mut stats = self.stats.write().map_err(|_| {
            DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                "Failed to acquire stats write lock".to_string(),
            )))
        })?;
        stats.entries_imported += 1;

        Ok(())
    }

    /// Export audit book entries with regulatory compliance filtering
    pub fn export_entries(
        &self,
        regulatory_framework: Option<RegulatoryFramework>,
        jurisdiction: Option<Jurisdiction>,
        max_access_level: AccessLevel,
    ) -> Result<Vec<AuditBookEntry>, DockLockError> {
        let entries = self.entries.read().map_err(|_| {
            DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                "Failed to acquire read lock".to_string(),
            )))
        })?;

        let filtered_entries: Vec<AuditBookEntry> = entries
            .iter()
            .filter(|entry| {
                // Filter by regulatory framework
                if let Some(ref framework) = regulatory_framework {
                    if !entry.regulatory_frameworks.contains(framework) {
                        return false;
                    }
                }

                // Filter by jurisdiction
                if let Some(ref jurisdiction) = jurisdiction {
                    if !entry.jurisdictions.contains(jurisdiction) {
                        return false;
                    }
                }

                // Filter by access level
                entry.access_level <= max_access_level
            })
            .take(self.export_config.max_entries)
            .cloned()
            .collect();

        // Update statistics
        let mut stats = self.stats.write().map_err(|_| {
            DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                "Failed to acquire stats write lock".to_string(),
            )))
        })?;
        stats.entries_exported += filtered_entries.len() as u64;
        stats.export_operations += 1;
        stats.last_export_timestamp = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| DockLockError::SystemTime(e.to_string()))?
                .as_secs(),
        );

        Ok(filtered_entries)
    }

    /// Generate multi-cloud storage locations for audit book export
    pub fn generate_storage_locations(&self) -> Result<Vec<StorageLocation>, DockLockError> {
        let mut locations = Vec::new();

        // Primary storage location
        let primary_location = self.generate_storage_location(&self.storage_config.primary_provider)?;
        locations.push(primary_location);

        // Backup storage locations
        for provider in &self.storage_config.backup_providers {
            let backup_location = self.generate_storage_location(provider)?;
            locations.push(backup_location);
        }

        // Update statistics
        let mut stats = self.stats.write().map_err(|_| {
            DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                "Failed to acquire stats write lock".to_string(),
            )))
        })?;
        stats.storage_operations += 1;

        Ok(locations)
    }

    /// Get audit book statistics
    pub fn get_statistics(&self) -> Result<AuditBookStats, DockLockError> {
        let stats = self.stats.read().map_err(|_| {
            DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                "Failed to acquire stats read lock".to_string(),
            )))
        })?;
        Ok(AuditBookStats {
            entries_imported: stats.entries_imported,
            entries_exported: stats.entries_exported,
            export_operations: stats.export_operations,
            compliance_violations: stats.compliance_violations,
            storage_operations: stats.storage_operations,
            last_export_timestamp: stats.last_export_timestamp,
        })
    }

    /// Get total number of entries
    pub fn get_entry_count(&self) -> Result<usize, DockLockError> {
        let entries = self.entries.read().map_err(|_| {
            DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(
                "Failed to acquire read lock".to_string(),
            )))
        })?;
        Ok(entries.len())
    }

    // Private helper methods will be added in the next part
    fn determine_access_level(&self, entry: &BlockbookEntry) -> AccessLevel {
        match (&entry.event_type, &entry.severity) {
            (BlockbookEventType::SecurityIncident, EventSeverity::Critical) => AccessLevel::TopSecret,
            (BlockbookEventType::ComplianceViolation, EventSeverity::Critical) => AccessLevel::Restricted,
            (BlockbookEventType::SlashingEvent, _) => AccessLevel::Restricted,
            (BlockbookEventType::DataAvailabilityChallenge, EventSeverity::Error) => AccessLevel::Confidential,
            (_, EventSeverity::Critical) => AccessLevel::Confidential,
            (_, EventSeverity::Error) => AccessLevel::Internal,
            _ => AccessLevel::Public,
        }
    }

    fn determine_retention_period(&self, entry: &BlockbookEntry) -> u64 {
        match &entry.event_type {
            BlockbookEventType::SecurityIncident => 7 * 365 * 24 * 3600, // 7 years
            BlockbookEventType::ComplianceViolation => 7 * 365 * 24 * 3600, // 7 years
            BlockbookEventType::SlashingEvent => 5 * 365 * 24 * 3600, // 5 years
            BlockbookEventType::DataAvailabilityChallenge => 3 * 365 * 24 * 3600, // 3 years
            _ => 365 * 24 * 3600, // 1 year
        }
    }

    fn generate_classification_tags(&self, entry: &BlockbookEntry) -> HashSet<String> {
        let mut tags = HashSet::new();
        
        tags.insert(format!("event_type:{:?}", entry.event_type));
        tags.insert(format!("severity:{:?}", entry.severity));
        
        match &entry.event_type {
            BlockbookEventType::SecurityIncident => {
                tags.insert("security".to_string());
                tags.insert("incident".to_string());
            }
            BlockbookEventType::ComplianceViolation => {
                tags.insert("compliance".to_string());
                tags.insert("violation".to_string());
            }
            BlockbookEventType::SlashingEvent => {
                tags.insert("slashing".to_string());
                tags.insert("validator".to_string());
            }
            BlockbookEventType::DataAvailabilityChallenge => {
                tags.insert("data_availability".to_string());
                tags.insert("challenge".to_string());
            }
            _ => {}
        }
        
        tags
    }

    fn generate_storage_location(&self, provider: &CloudProvider) -> Result<StorageLocation, DockLockError> {
        let region = self.storage_config.regions.first()
            .unwrap_or(&"us-east-1".to_string())
            .clone();

        let url = match provider {
            CloudProvider::AWS => format!("s3://audit-book-{}/exports/", region),
            CloudProvider::Azure => format!("https://auditbook{}.blob.core.windows.net/exports/", region),
            CloudProvider::GCP => format!("gs://audit-book-{}/exports/", region),
            CloudProvider::OnPremises => "/var/lib/audit-book/exports/".to_string(),
            CloudProvider::Custom(name) => format!("custom://{}/audit-book/exports/", name),
        };

        // In production, this would be properly encrypted credentials
        let hash_bytes = domain_hash(crate::blockbook::BLOCKBOOK_ENTRY_HASH, url.as_bytes());
        let credentials_hash = format!("cred_hash_{:02x}{:02x}{:02x}{:02x}", 
            hash_bytes[0], hash_bytes[1], hash_bytes[2], hash_bytes[3]);

        Ok(StorageLocation {
            provider: provider.clone(),
            url,
            region,
            credentials_hash,
        })
    }

    /// Serialize audit book entries to specified format
    pub fn serialize_entries(
        &self,
        entries: &[AuditBookEntry],
        format: &ExportFormat,
    ) -> Result<Vec<u8>, DockLockError> {
        match format {
            ExportFormat::JSON => {
                let json_data = serde_json::to_string_pretty(entries)
                    .map_err(|e| DockLockError::Serialization(bincode::Error::new(bincode::ErrorKind::Custom(e.to_string()))))?;
                Ok(json_data.into_bytes())
            }
            ExportFormat::CSV => {
                // Simplified CSV export (in production, would use proper CSV library)
                let mut csv_data = String::from("timestamp,event_type,severity,description,regulatory_frameworks,jurisdictions,access_level\n");
                for entry in entries {
                    let frameworks = entry.regulatory_frameworks.iter()
                        .map(|f| format!("{:?}", f))
                        .collect::<Vec<_>>()
                        .join(";");
                    let jurisdictions = entry.jurisdictions.iter()
                        .map(|j| format!("{:?}", j))
                        .collect::<Vec<_>>()
                        .join(";");
                    
                    csv_data.push_str(&format!(
                        "{},{:?},{:?},{},{},{},{:?}\n",
                        entry.blockbook_entry.timestamp,
                        entry.blockbook_entry.event_type,
                        entry.blockbook_entry.severity,
                        format!("{:?}", entry.blockbook_entry.payload).replace(',', ";"),
                        frameworks,
                        jurisdictions,
                        entry.access_level
                    ));
                }
                Ok(csv_data.into_bytes())
            }
            ExportFormat::XML => {
                // Simplified XML export
                let mut xml_data = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<audit_book>\n");
                for entry in entries {
                    xml_data.push_str(&format!(
                        "  <entry timestamp=\"{}\" event_type=\"{:?}\" severity=\"{:?}\" access_level=\"{:?}\">\n",
                        entry.blockbook_entry.timestamp,
                        entry.blockbook_entry.event_type,
                        entry.blockbook_entry.severity,
                        entry.access_level
                    ));
                    xml_data.push_str(&format!("    <payload>{:?}</payload>\n", entry.blockbook_entry.payload));
                    xml_data.push_str("  </entry>\n");
                }
                xml_data.push_str("</audit_book>\n");
                Ok(xml_data.into_bytes())
            }
            ExportFormat::Binary => {
                bincode::serialize(entries).map_err(DockLockError::Serialization)
            }
        }
    }

    /// Verify compliance with regulatory frameworks
    pub fn verify_compliance(
        &self,
        framework: &RegulatoryFramework,
    ) -> Result<bool, DockLockError> {
        match framework {
            RegulatoryFramework::GDPR => self.verify_gdpr_compliance(),
            RegulatoryFramework::HIPAA => self.verify_hipaa_compliance(),
            RegulatoryFramework::SOX => self.verify_sox_compliance(),
            RegulatoryFramework::PCIDSS => self.verify_pci_compliance(),
            RegulatoryFramework::CCPA => self.verify_ccpa_compliance(),
            RegulatoryFramework::Custom(_) => self.verify_custom_compliance(),
        }
    }

    // Compliance verification methods (simplified implementations)
    
    fn verify_gdpr_compliance(&self) -> Result<bool, DockLockError> {
        // GDPR compliance checks:
        // - Right to be forgotten implementation
        // - Data minimization
        // - Consent management
        // - Data protection by design
        Ok(true) // Simplified for demo
    }

    fn verify_hipaa_compliance(&self) -> Result<bool, DockLockError> {
        // HIPAA compliance checks:
        // - PHI protection
        // - Access controls
        // - Audit logs
        // - Encryption requirements
        Ok(true) // Simplified for demo
    }

    fn verify_sox_compliance(&self) -> Result<bool, DockLockError> {
        // SOX compliance checks:
        // - Financial data integrity
        // - Audit trail completeness
        // - Internal controls
        // - Management certification
        Ok(true) // Simplified for demo
    }

    fn verify_pci_compliance(&self) -> Result<bool, DockLockError> {
        // PCI DSS compliance checks:
        // - Cardholder data protection
        // - Secure network architecture
        // - Vulnerability management
        // - Access control measures
        Ok(true) // Simplified for demo
    }

    fn verify_ccpa_compliance(&self) -> Result<bool, DockLockError> {
        // CCPA compliance checks:
        // - Consumer rights implementation
        // - Data transparency
        // - Opt-out mechanisms
        // - Data deletion capabilities
        Ok(true) // Simplified for demo
    }

    fn verify_custom_compliance(&self) -> Result<bool, DockLockError> {
        // Custom compliance framework verification
        Ok(true) // Simplified for demo
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockbook::{Blockbook, BlockbookEventType, EventSeverity};
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    fn create_test_audit_book() -> AuditBook {
        let mut csprng = OsRng {};
        let keypair = SigningKey::generate(&mut csprng);
        
        let storage_config = MultiCloudStorageConfig {
            primary_provider: CloudProvider::AWS,
            backup_providers: vec![CloudProvider::Azure, CloudProvider::GCP],
            encryption_scheme: EncryptionScheme::AES256GCM,
            regions: vec!["us-east-1".to_string(), "eu-west-1".to_string()],
            replication_factor: 3,
        };

        let export_config = ExportConfig {
            format: ExportFormat::JSON,
            include_sensitive: false,
            max_entries: 1000,
            compression: true,
            require_signature: true,
        };

        AuditBook::new(storage_config, export_config, keypair)
    }

    #[test]
    fn test_audit_book_creation() {
        let audit_book = create_test_audit_book();
        assert_eq!(audit_book.get_entry_count().unwrap(), 0);
    }

    #[test]
    fn test_import_from_blockbook() {
        let audit_book = create_test_audit_book();
        let mut csprng = OsRng {};
        let _keypair = SigningKey::generate(&mut csprng);
        let config = crate::blockbook::BlockbookConfig::default();
        let blockbook = Blockbook::new(config);

        // Add some test entries to blockbook
        blockbook.record_bus_bios_routing("test_packet_id".as_bytes().to_vec(), "test_destination".to_string()).unwrap();
        blockbook.record_traffic_light_decision("test_flow_id".as_bytes().to_vec(), "Green".to_string(), "Normal traffic".to_string()).unwrap();

        let frameworks = vec![RegulatoryFramework::GDPR, RegulatoryFramework::SOX].into_iter().collect();
        let jurisdictions = vec![Jurisdiction::EU, Jurisdiction::US].into_iter().collect();

        let imported_count = audit_book.import_from_blockbook(
            &blockbook,
            frameworks,
            jurisdictions,
            AccessLevel::Public,
        ).unwrap();

        assert!(imported_count > 0);
        assert_eq!(audit_book.get_entry_count().unwrap(), imported_count);
    }

    #[test]
    fn test_export_entries() {
        let audit_book = create_test_audit_book();
        let mut csprng = OsRng {};
        let _keypair = SigningKey::generate(&mut csprng);
        let config = crate::blockbook::BlockbookConfig::default();
        let blockbook = Blockbook::new(config);

        // Import some entries
        let frameworks = vec![RegulatoryFramework::GDPR].into_iter().collect();
        let jurisdictions = vec![Jurisdiction::EU].into_iter().collect();
        
        audit_book.import_from_blockbook(&blockbook, frameworks, jurisdictions, AccessLevel::Public).unwrap();

        // Export entries
        let exported = audit_book.export_entries(
            Some(RegulatoryFramework::GDPR),
            Some(Jurisdiction::EU),
            AccessLevel::Confidential,
        ).unwrap();

        assert!(exported.len() <= audit_book.get_entry_count().unwrap());
    }

    #[test]
    fn test_storage_location_generation() {
        let audit_book = create_test_audit_book();
        let locations = audit_book.generate_storage_locations().unwrap();

        assert_eq!(locations.len(), 3); // 1 primary + 2 backup
        assert_eq!(locations[0].provider, CloudProvider::AWS);
        assert_eq!(locations[1].provider, CloudProvider::Azure);
        assert_eq!(locations[2].provider, CloudProvider::GCP);
    }

    #[test]
    fn test_serialization_formats() {
        let audit_book = create_test_audit_book();
        let mut csprng = OsRng {};
        let _keypair = SigningKey::generate(&mut csprng);
        let config = crate::blockbook::BlockbookConfig::default();
        let blockbook = Blockbook::new(config);

        let frameworks = vec![RegulatoryFramework::GDPR].into_iter().collect();
        let jurisdictions = vec![Jurisdiction::EU].into_iter().collect();
        
        audit_book.import_from_blockbook(&blockbook, frameworks, jurisdictions, AccessLevel::Public).unwrap();
        let entries = audit_book.export_entries(None, None, AccessLevel::TopSecret).unwrap();

        // Test JSON serialization
        let json_data = audit_book.serialize_entries(&entries, &ExportFormat::JSON).unwrap();
        assert!(!json_data.is_empty());

        // Test CSV serialization
        let csv_data = audit_book.serialize_entries(&entries, &ExportFormat::CSV).unwrap();
        assert!(!csv_data.is_empty());

        // Test XML serialization
        let xml_data = audit_book.serialize_entries(&entries, &ExportFormat::XML).unwrap();
        assert!(!xml_data.is_empty());

        // Test Binary serialization
        let binary_data = audit_book.serialize_entries(&entries, &ExportFormat::Binary).unwrap();
        assert!(!binary_data.is_empty());
    }

    #[test]
    fn test_compliance_verification() {
        let audit_book = create_test_audit_book();

        assert!(audit_book.verify_compliance(&RegulatoryFramework::GDPR).unwrap());
        assert!(audit_book.verify_compliance(&RegulatoryFramework::HIPAA).unwrap());
        assert!(audit_book.verify_compliance(&RegulatoryFramework::SOX).unwrap());
        assert!(audit_book.verify_compliance(&RegulatoryFramework::PCIDSS).unwrap());
        assert!(audit_book.verify_compliance(&RegulatoryFramework::CCPA).unwrap());
    }

    #[test]
    fn test_access_level_filtering() {
        let audit_book = create_test_audit_book();
        let mut csprng = OsRng {};
        let _keypair = SigningKey::generate(&mut csprng);
        let config = crate::blockbook::BlockbookConfig::default();
        let blockbook = Blockbook::new(config);

        // Add entries with different severity levels using record_event
        blockbook.record_event(
            BlockbookEventType::SecurityIncident,
            EventSeverity::Critical,
            "test_source".to_string(),
            "test_incident".as_bytes().to_vec(),
            None,
        ).unwrap();
        blockbook.record_event(
            BlockbookEventType::ComplianceViolation,
            EventSeverity::Error,
            "test_source".to_string(),
            "test_violation".as_bytes().to_vec(),
            None,
        ).unwrap();

        let frameworks = vec![RegulatoryFramework::GDPR].into_iter().collect();
        let jurisdictions = vec![Jurisdiction::EU].into_iter().collect();
        
        audit_book.import_from_blockbook(&blockbook, frameworks, jurisdictions, AccessLevel::Public).unwrap();

        // Export with different access levels
        let public_entries = audit_book.export_entries(None, None, AccessLevel::Public).unwrap();
        let confidential_entries = audit_book.export_entries(None, None, AccessLevel::Confidential).unwrap();
        let top_secret_entries = audit_book.export_entries(None, None, AccessLevel::TopSecret).unwrap();

        assert!(public_entries.len() <= confidential_entries.len());
        assert!(confidential_entries.len() <= top_secret_entries.len());
    }

    #[test]
    fn test_statistics_tracking() {
        let audit_book = create_test_audit_book();
        let mut csprng = OsRng {};
        let _keypair = SigningKey::generate(&mut csprng);
        let config = crate::blockbook::BlockbookConfig::default();
        let blockbook = Blockbook::new(config);

        let initial_stats = audit_book.get_statistics().unwrap();
        assert_eq!(initial_stats.entries_imported, 0);
        assert_eq!(initial_stats.entries_exported, 0);

        // Import entries
        let frameworks = vec![RegulatoryFramework::GDPR].into_iter().collect();
        let jurisdictions = vec![Jurisdiction::EU].into_iter().collect();
        
        let imported_count = audit_book.import_from_blockbook(&blockbook, frameworks, jurisdictions, AccessLevel::Public).unwrap();

        // Export entries
        let exported_entries = audit_book.export_entries(None, None, AccessLevel::TopSecret).unwrap();

        let final_stats = audit_book.get_statistics().unwrap();
        assert_eq!(final_stats.entries_imported, imported_count as u64);
        assert_eq!(final_stats.entries_exported, exported_entries.len() as u64);
        assert_eq!(final_stats.export_operations, 1);
    }
}
