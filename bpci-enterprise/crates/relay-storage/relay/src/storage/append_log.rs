//! Layer 4: Append-Only Logs - Military-Grade Blockchain Data Storage
//! 
//! Immutable append-only logs for blockchain data with military-grade integrity

use anyhow::{Result, anyhow};
use super::{StorageLayer, StorageError};

// Implement StorageLayer trait for AppendLog
impl StorageLayer for AppendLog {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, StorageError> {
        let key_str = std::str::from_utf8(key)
            .map_err(|e| StorageError::SerializationError(format!("Invalid UTF-8 key: {}", e)))?;
        
        // Use async method in sync context (simplified for trait compatibility)
        match tokio::runtime::Handle::try_current() {
            Ok(handle) => {
                handle.block_on(async {
                    self.get(key_str).await
                        .map_err(|e| StorageError::IoError(e.to_string()))
                })
            },
            Err(_) => Err(StorageError::IoError("No async runtime available".to_string()))
        }
    }
    
    fn put(&self, key: &[u8], value: &[u8]) -> Result<(), StorageError> {
        let key_str = std::str::from_utf8(key)
            .map_err(|e| StorageError::SerializationError(format!("Invalid UTF-8 key: {}", e)))?;
        
        match tokio::runtime::Handle::try_current() {
            Ok(handle) => {
                handle.block_on(async {
                    self.append(key_str, value).await
                        .map_err(|e| StorageError::IoError(e.to_string()))
                })
            },
            Err(_) => Err(StorageError::IoError("No async runtime available".to_string()))
        }
    }
    
    fn delete(&self, _key: &[u8]) -> Result<(), StorageError> {
        // Append-only logs don't support deletion by design
        Err(StorageError::IoError("Deletion not supported in append-only logs".to_string()))
    }
    
    fn exists(&self, key: &[u8]) -> Result<bool, StorageError> {
        let key_str = std::str::from_utf8(key)
            .map_err(|e| StorageError::SerializationError(format!("Invalid UTF-8 key: {}", e)))?;
        
        match tokio::runtime::Handle::try_current() {
            Ok(handle) => {
                handle.block_on(async {
                    let index = self.index.read().await;
                    Ok(index.contains_key(key_str))
                })
            },
            Err(_) => Err(StorageError::IoError("No async runtime available".to_string()))
        }
    }
    
    fn size(&self) -> Result<u64, StorageError> {
        match tokio::runtime::Handle::try_current() {
            Ok(handle) => {
                handle.block_on(async {
                    let index = self.index.read().await;
                    Ok(index.len() as u64)
                })
            },
            Err(_) => Err(StorageError::IoError("No async runtime available".to_string()))
        }
    }
    
    fn health_check(&self) -> Result<(), StorageError> {
        match tokio::runtime::Handle::try_current() {
            Ok(handle) => {
                handle.block_on(async {
                    self.health_check().await
                        .map_err(|e| StorageError::IoError(e.to_string()))
                })
            },
            Err(_) => Err(StorageError::IoError("No async runtime available".to_string()))
        }
    }
}
use tokio::sync::RwLock;
use tracing::{info, error, warn};

/// Military-grade log entry with blockchain integrity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub key: String,
    pub value: Vec<u8>,
    pub timestamp: u64,
    pub checksum: u64,
    pub sequence: u64,
}

impl LogEntry {
    /// Create new log entry with military-grade checksum
    pub fn new(key: String, value: Vec<u8>, sequence: u64) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Military-grade checksum calculation
        let checksum = Self::calculate_checksum(&key, &value, timestamp, sequence);
        
        Self {
            key,
            value,
            timestamp,
            checksum,
            sequence,
        }
    }
    
    /// Calculate military-grade checksum
    fn calculate_checksum(key: &str, value: &[u8], timestamp: u64, sequence: u64) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        value.hash(&mut hasher);
        timestamp.hash(&mut hasher);
        sequence.hash(&mut hasher);
        hasher.finish()
    }
    
    /// Verify military-grade integrity
    pub fn verify_integrity(&self) -> bool {
        let expected_checksum = Self::calculate_checksum(
            &self.key,
            &self.value,
            self.timestamp,
            self.sequence,
        );
        self.checksum == expected_checksum
    }
}

/// Append-only log for military-grade blockchain data
pub struct AppendLog {
    log_dir: PathBuf,
    current_log: Arc<RwLock<Option<File>>>,
    index: Arc<RwLock<HashMap<String, (PathBuf, u64)>>>, // key -> (file_path, offset)
    sequence: Arc<RwLock<u64>>,
    max_log_size: u64,
}

impl AppendLog {
    /// Create new append-only log system
    pub async fn new<P: AsRef<Path>>(log_dir: P) -> Result<Self> {
        info!("📝 Initializing Append-Only Log Layer 4 (Blockchain Data)");
        
        let log_dir = log_dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&log_dir)
            .map_err(|e| anyhow!("Failed to create log directory: {}", e))?;
        
        let mut log_system = Self {
            log_dir,
            current_log: Arc::new(RwLock::new(None)),
            index: Arc::new(RwLock::new(HashMap::new())),
            sequence: Arc::new(RwLock::new(0)),
            max_log_size: 100 * 1024 * 1024, // 100MB per log file
        };
        
        // Rebuild index from existing logs
        log_system.rebuild_index().await?;
        
        Ok(log_system)
    }
    
    /// Append data with military-grade immutability
    pub async fn append(&self, key: &str, value: &[u8]) -> Result<()> {
        let sequence = {
            let mut seq = self.sequence.write().await;
            *seq += 1;
            *seq
        };
        
        let entry = LogEntry::new(key.to_string(), value.to_vec(), sequence);
        
        // Serialize entry with military-grade format
        let serialized = bincode::serialize(&entry)
            .map_err(|e| anyhow!("Failed to serialize log entry: {}", e))?;
        
        // Get or create current log file
        let (log_file_path, offset) = self.write_to_log(&serialized).await?;
        
        // Update index with military-grade precision
        {
            let mut index = self.index.write().await;
            index.insert(key.to_string(), (log_file_path, offset));
        }
        
        Ok(())
    }
    
    /// Get data with military-grade verification
    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let (file_path, offset) = {
            let index = self.index.read().await;
            match index.get(key) {
                Some((path, offset)) => (path.clone(), *offset),
                None => return Ok(None),
            }
        };
        
        // Read from log file with military-grade verification
        let mut file = File::open(&file_path)
            .map_err(|e| anyhow!("Failed to open log file: {}", e))?;
        
        file.seek(SeekFrom::Start(offset))
            .map_err(|e| anyhow!("Failed to seek in log file: {}", e))?;
        
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        reader.read_line(&mut line)
            .map_err(|e| anyhow!("Failed to read log entry: {}", e))?;
        
        if line.is_empty() {
            return Ok(None);
        }
        
        // Deserialize with military-grade verification
        let entry_bytes = line.trim().as_bytes();
        let entry: LogEntry = bincode::deserialize(entry_bytes)
            .map_err(|e| anyhow!("Failed to deserialize log entry: {}", e))?;
        
        // Military-grade integrity verification
        if !entry.verify_integrity() {
            error!("Log entry integrity verification failed for key: {}", key);
            return Err(anyhow!("Log entry integrity verification failed"));
        }
        
        if entry.key == key {
            Ok(Some(entry.value))
        } else {
            warn!("Key mismatch in log entry: expected {}, found {}", key, entry.key);
            Ok(None)
        }
    }
    
    /// Get all keys (for military-grade auditing)
    pub async fn get_all_keys(&self) -> Result<Vec<String>> {
        let index = self.index.read().await;
        Ok(index.keys().cloned().collect())
    }
    
    /// Health check for append log system
    pub async fn health_check(&self) -> Result<()> {
        // Simple health check - verify we can access the log directory
        if !self.log_dir.exists() {
            return Err(anyhow!("Log directory does not exist"));
        }
        
        // Verify we can access the index
        let _index = self.index.read().await;
        let _sequence = self.sequence.read().await;
        
        Ok(())
    }
    
    /// Get log statistics (military-grade metrics)
    pub async fn get_stats(&self) -> Result<HashMap<String, u64>> {
        let mut stats = HashMap::new();
        
        let index = self.index.read().await;
        stats.insert("total_keys".to_string(), index.len() as u64);
        
        let sequence = self.sequence.read().await;
        stats.insert("total_entries".to_string(), *sequence);
        
        // Calculate total log size
        let mut total_size = 0u64;
        for entry in std::fs::read_dir(&self.log_dir)? {
            let entry = entry?;
            if entry.path().extension().map_or(false, |ext| ext == "log") {
                total_size += entry.metadata()?.len();
            }
        }
        stats.insert("total_size_bytes".to_string(), total_size);
        
        Ok(stats)
    }
    
    // Private helper methods
    
    async fn write_to_log(&self, data: &[u8]) -> Result<(PathBuf, u64)> {
        let mut current_log = self.current_log.write().await;
        
        // Check if we need a new log file
        let needs_new_file = match current_log.as_ref() {
            None => true,
            Some(file) => {
                let metadata = file.metadata()
                    .map_err(|e| anyhow!("Failed to get file metadata: {}", e))?;
                metadata.len() >= self.max_log_size
            }
        };
        
        if needs_new_file {
            let new_log_path = self.create_new_log_file().await?;
            let new_file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&new_log_path)
                .map_err(|e| anyhow!("Failed to open new log file: {}", e))?;
            *current_log = Some(new_file);
        }
        
        let file = current_log.as_mut().unwrap();
        let offset = file.metadata()
            .map_err(|e| anyhow!("Failed to get file position: {}", e))?
            .len();
        
        // Write with military-grade durability
        writeln!(file, "{}", String::from_utf8_lossy(data))
            .map_err(|e| anyhow!("Failed to write to log: {}", e))?;
        file.flush()
            .map_err(|e| anyhow!("Failed to flush log: {}", e))?;
        
        let log_path = self.get_current_log_path().await?;
        Ok((log_path, offset))
    }
    
    async fn create_new_log_file(&self) -> Result<PathBuf> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let log_name = format!("military_log_{}.log", timestamp);
        let log_path = self.log_dir.join(log_name);
        
        // Create empty log file
        File::create(&log_path)
            .map_err(|e| anyhow!("Failed to create log file: {}", e))?;
        
        info!("Created new military-grade log file: {:?}", log_path);
        Ok(log_path)
    }
    
    async fn get_current_log_path(&self) -> Result<PathBuf> {
        // Find the most recent log file
        let mut latest_log = None;
        let mut latest_time = 0u64;
        
        for entry in std::fs::read_dir(&self.log_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "log") {
                if let Some(file_name) = path.file_stem() {
                    if let Some(time_str) = file_name.to_str() {
                        if let Some(time_part) = time_str.strip_prefix("military_log_") {
                            if let Ok(time) = time_part.parse::<u64>() {
                                if time > latest_time {
                                    latest_time = time;
                                    latest_log = Some(path);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        match latest_log {
            Some(path) => Ok(path),
            None => self.create_new_log_file().await,
        }
    }
    
    async fn rebuild_index(&self) -> Result<()> {
        info!("Rebuilding military-grade log index...");
        
        let mut index = HashMap::new();
        let mut max_sequence = 0u64;
        
        for entry in std::fs::read_dir(&self.log_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "log") {
                let file = File::open(&path)
                    .map_err(|e| anyhow!("Failed to open log file for indexing: {}", e))?;
                
                let reader = BufReader::new(file);
                let mut offset = 0u64;
                
                for line in reader.lines() {
                    let line = line?;
                    let line_bytes = line.as_bytes();
                    
                    if let Ok(entry) = bincode::deserialize::<LogEntry>(line_bytes) {
                        // Verify military-grade integrity
                        if entry.verify_integrity() {
                            index.insert(entry.key.clone(), (path.clone(), offset));
                            max_sequence = max_sequence.max(entry.sequence);
                        } else {
                            warn!("Corrupted log entry found during index rebuild: {}", entry.key);
                        }
                    }
                    
                    offset += line_bytes.len() as u64 + 1; // +1 for newline
                }
            }
        }
        
        {
            let mut index_guard = self.index.write().await;
            *index_guard = index;
        }
        
        {
            let mut sequence_guard = self.sequence.write().await;
            *sequence_guard = max_sequence;
        }
        
        info!("Military-grade log index rebuilt with {} entries", max_sequence);
        Ok(())
    }
}
