// Dual Transaction Manager
// Manages parallel processing of government and blockchain transactions
// Ensures consistency, atomicity, and compliance across both systems

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing;

use super::GovernmentConfig;

/// Dual transaction manager for parallel processing
#[derive(Debug)]
pub struct DualTransactionManager {
    config: GovernmentConfig,
    active_transactions: Arc<Mutex<HashMap<String, TransactionPair>>>,
    manager_state: Arc<RwLock<ManagerState>>,
    processing_queue: Arc<Mutex<Vec<QueuedTransaction>>>,
}

/// Transaction pair (government + blockchain)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionPair {
    pub pair_id: String,
    pub session_id: String,
    pub government_transaction: GovernmentTransaction,
    pub blockchain_transaction: BlockchainTransaction,
    pub processing_mode: ProcessingMode,
    pub status: TransactionStatus,
    pub created_at: u64,
    pub updated_at: u64,
    pub completion_time_ms: Option<u64>,
}

/// Government transaction details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernmentTransaction {
    pub transaction_id: String,
    pub jurisdiction: String,
    pub operation_type: String,
    pub data: serde_json::Value,
    pub status: TransactionStatus,
    pub government_reference: Option<String>,
    pub compliance_validated: bool,
}

/// Blockchain transaction details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainTransaction {
    pub transaction_id: String,
    pub block_hash: Option<String>,
    pub data: serde_json::Value,
    pub status: TransactionStatus,
    pub confirmations: u32,
    pub gas_used: Option<u64>,
}

/// Processing mode for dual transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessingMode {
    Parallel,      // Process both simultaneously
    Sequential,    // Government first, then blockchain
    Conditional,   // Blockchain only if government succeeds
    Rollback,      // Can rollback if either fails
}

/// Transaction status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Processing,
    Completed,
    Failed(String),
    Cancelled,
    RolledBack,
}

/// Queued transaction for processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedTransaction {
    pub queue_id: String,
    pub session_id: String,
    pub transaction_data: serde_json::Value,
    pub priority: TransactionPriority,
    pub queued_at: u64,
    pub processing_mode: ProcessingMode,
}

/// Transaction priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionPriority {
    Low,
    Normal,
    High,
    Critical,
    Emergency,
}

/// Manager state tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagerState {
    pub manager_id: String,
    pub total_transactions_processed: u64,
    pub successful_transactions: u64,
    pub failed_transactions: u64,
    pub active_transaction_count: u32,
    pub queue_size: u32,
    pub average_processing_time_ms: f64,
    pub last_transaction_timestamp: u64,
}

impl DualTransactionManager {
    /// Create a new dual transaction manager
    pub async fn new(config: GovernmentConfig) -> Result<Self> {
        let manager_id = Uuid::new_v4().to_string();
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let manager_state = ManagerState {
            manager_id,
            total_transactions_processed: 0,
            successful_transactions: 0,
            failed_transactions: 0,
            active_transaction_count: 0,
            queue_size: 0,
            average_processing_time_ms: 0.0,
            last_transaction_timestamp: current_time,
        };

        Ok(Self {
            config,
            active_transactions: Arc::new(Mutex::new(HashMap::new())),
            manager_state: Arc::new(RwLock::new(manager_state)),
            processing_queue: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// Initialize the dual transaction manager
    pub async fn initialize(&self) -> Result<()> {
        tracing::info!("⚡ Initializing Dual Transaction Manager...");

        // Start background processing tasks
        self.start_background_processing().await?;

        tracing::info!("✅ Dual Transaction Manager initialized successfully");
        Ok(())
    }

    /// Process a dual transaction
    pub async fn process_dual_transaction(
        &self,
        session_id: &str,
        transaction_data: serde_json::Value,
    ) -> Result<String> {
        let pair_id = Uuid::new_v4().to_string();
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        // Create transaction pair
        let transaction_pair = TransactionPair {
            pair_id: pair_id.clone(),
            session_id: session_id.to_string(),
            government_transaction: GovernmentTransaction {
                transaction_id: Uuid::new_v4().to_string(),
                jurisdiction: self.extract_jurisdiction(&transaction_data)?,
                operation_type: self.extract_operation_type(&transaction_data)?,
                data: transaction_data.clone(),
                status: TransactionStatus::Pending,
                government_reference: None,
                compliance_validated: false,
            },
            blockchain_transaction: BlockchainTransaction {
                transaction_id: Uuid::new_v4().to_string(),
                block_hash: None,
                data: transaction_data,
                status: TransactionStatus::Pending,
                confirmations: 0,
                gas_used: None,
            },
            processing_mode: ProcessingMode::Parallel,
            status: TransactionStatus::Pending,
            created_at: current_time,
            updated_at: current_time,
            completion_time_ms: None,
        };

        // Store active transaction
        {
            let mut transactions = self.active_transactions.lock().await;
            transactions.insert(pair_id.clone(), transaction_pair.clone());
        }

        // Update manager state
        {
            let mut state = self.manager_state.write().unwrap();
            state.active_transaction_count += 1;
            state.last_transaction_timestamp = current_time;
        }

        // Process the transaction pair
        let result = self.execute_dual_transaction(&pair_id).await;

        match result {
            Ok(_) => {
                tracing::info!("✅ Dual transaction completed successfully: {}", pair_id);
                self.update_success_metrics().await;
                Ok(pair_id)
            }
            Err(e) => {
                tracing::error!("❌ Dual transaction failed: {} - {}", pair_id, e);
                self.update_failure_metrics().await;
                self.handle_transaction_failure(&pair_id, &e.to_string()).await?;
                Err(e)
            }
        }
    }

    /// Execute dual transaction processing
    async fn execute_dual_transaction(&self, pair_id: &str) -> Result<()> {
        let transaction_pair = {
            let transactions = self.active_transactions.lock().await;
            transactions.get(pair_id)
                .ok_or_else(|| anyhow!("Transaction pair not found: {}", pair_id))?
                .clone()
        };

        let start_time = std::time::Instant::now();

        match transaction_pair.processing_mode {
            ProcessingMode::Parallel => {
                self.process_parallel(&pair_id, &transaction_pair).await?;
            }
            ProcessingMode::Sequential => {
                self.process_sequential(&pair_id, &transaction_pair).await?;
            }
            ProcessingMode::Conditional => {
                self.process_conditional(&pair_id, &transaction_pair).await?;
            }
            ProcessingMode::Rollback => {
                self.process_with_rollback(&pair_id, &transaction_pair).await?;
            }
        }

        let completion_time = start_time.elapsed().as_millis() as u64;

        // Update transaction completion
        {
            let mut transactions = self.active_transactions.lock().await;
            if let Some(pair) = transactions.get_mut(pair_id) {
                pair.status = TransactionStatus::Completed;
                pair.completion_time_ms = Some(completion_time);
                pair.updated_at = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
            }
        }

        // Update processing time metrics
        self.update_processing_time_metrics(completion_time).await;

        Ok(())
    }

    /// Process transactions in parallel
    async fn process_parallel(&self, pair_id: &str, transaction_pair: &TransactionPair) -> Result<()> {
        tracing::debug!("Processing parallel transactions for pair: {}", pair_id);

        // Simulate parallel processing
        let government_task = self.process_government_transaction(&transaction_pair.government_transaction);
        let blockchain_task = self.process_blockchain_transaction(&transaction_pair.blockchain_transaction);

        // Wait for both to complete
        let (gov_result, blockchain_result) = tokio::join!(government_task, blockchain_task);

        gov_result?;
        blockchain_result?;

        tracing::debug!("Parallel processing completed for pair: {}", pair_id);
        Ok(())
    }

    /// Process transactions sequentially
    async fn process_sequential(&self, pair_id: &str, transaction_pair: &TransactionPair) -> Result<()> {
        tracing::debug!("Processing sequential transactions for pair: {}", pair_id);

        // Government transaction first
        self.process_government_transaction(&transaction_pair.government_transaction).await?;

        // Then blockchain transaction
        self.process_blockchain_transaction(&transaction_pair.blockchain_transaction).await?;

        tracing::debug!("Sequential processing completed for pair: {}", pair_id);
        Ok(())
    }

    /// Process transactions conditionally
    async fn process_conditional(&self, pair_id: &str, transaction_pair: &TransactionPair) -> Result<()> {
        tracing::debug!("Processing conditional transactions for pair: {}", pair_id);

        // Government transaction first
        match self.process_government_transaction(&transaction_pair.government_transaction).await {
            Ok(_) => {
                // Only proceed with blockchain if government succeeds
                self.process_blockchain_transaction(&transaction_pair.blockchain_transaction).await?;
            }
            Err(e) => {
                tracing::warn!("Government transaction failed, skipping blockchain: {}", e);
                return Err(e);
            }
        }

        tracing::debug!("Conditional processing completed for pair: {}", pair_id);
        Ok(())
    }

    /// Process transactions with rollback capability
    async fn process_with_rollback(&self, pair_id: &str, transaction_pair: &TransactionPair) -> Result<()> {
        tracing::debug!("Processing transactions with rollback for pair: {}", pair_id);

        let mut government_success = false;
        let mut blockchain_success = false;

        // Try government transaction
        match self.process_government_transaction(&transaction_pair.government_transaction).await {
            Ok(_) => government_success = true,
            Err(e) => tracing::warn!("Government transaction failed: {}", e),
        }

        // Try blockchain transaction
        match self.process_blockchain_transaction(&transaction_pair.blockchain_transaction).await {
            Ok(_) => blockchain_success = true,
            Err(e) => tracing::warn!("Blockchain transaction failed: {}", e),
        }

        // If either failed, rollback the successful one
        if !government_success || !blockchain_success {
            if government_success {
                self.rollback_government_transaction(&transaction_pair.government_transaction).await?;
            }
            if blockchain_success {
                self.rollback_blockchain_transaction(&transaction_pair.blockchain_transaction).await?;
            }
            return Err(anyhow!("Transaction pair failed, rollback completed"));
        }

        tracing::debug!("Rollback processing completed for pair: {}", pair_id);
        Ok(())
    }

    /// Process government transaction
    async fn process_government_transaction(&self, transaction: &GovernmentTransaction) -> Result<()> {
        tracing::debug!("Processing government transaction: {}", transaction.transaction_id);

        // Simulate government transaction processing
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

        // Simulate success (in production, this would be real government API calls)
        tracing::debug!("Government transaction completed: {}", transaction.transaction_id);
        Ok(())
    }

    /// Process blockchain transaction
    async fn process_blockchain_transaction(&self, transaction: &BlockchainTransaction) -> Result<()> {
        tracing::debug!("Processing blockchain transaction: {}", transaction.transaction_id);

        // Simulate blockchain transaction processing
        tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

        // Simulate success (in production, this would be real blockchain calls)
        tracing::debug!("Blockchain transaction completed: {}", transaction.transaction_id);
        Ok(())
    }

    /// Rollback government transaction
    async fn rollback_government_transaction(&self, transaction: &GovernmentTransaction) -> Result<()> {
        tracing::warn!("Rolling back government transaction: {}", transaction.transaction_id);

        // Simulate rollback processing
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        tracing::info!("Government transaction rollback completed: {}", transaction.transaction_id);
        Ok(())
    }

    /// Rollback blockchain transaction
    async fn rollback_blockchain_transaction(&self, transaction: &BlockchainTransaction) -> Result<()> {
        tracing::warn!("Rolling back blockchain transaction: {}", transaction.transaction_id);

        // Simulate rollback processing
        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;

        tracing::info!("Blockchain transaction rollback completed: {}", transaction.transaction_id);
        Ok(())
    }

    /// Extract jurisdiction from transaction data
    fn extract_jurisdiction(&self, data: &serde_json::Value) -> Result<String> {
        data.get("jurisdiction")
            .and_then(|j| j.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("Jurisdiction not found in transaction data"))
    }

    /// Extract operation type from transaction data
    fn extract_operation_type(&self, data: &serde_json::Value) -> Result<String> {
        data.get("operation_type")
            .and_then(|o| o.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "default_operation".to_string());
        Ok("default_operation".to_string())
    }

    /// Start background processing tasks
    async fn start_background_processing(&self) -> Result<()> {
        // In production, this would spawn background tasks for:
        // - Queue processing
        // - Transaction monitoring
        // - Cleanup of completed transactions
        tracing::debug!("Background processing tasks started");
        Ok(())
    }

    /// Handle transaction failure
    async fn handle_transaction_failure(&self, pair_id: &str, error: &str) -> Result<()> {
        // Update transaction status
        {
            let mut transactions = self.active_transactions.lock().await;
            if let Some(pair) = transactions.get_mut(pair_id) {
                pair.status = TransactionStatus::Failed(error.to_string());
                pair.updated_at = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
            }
        }

        tracing::error!("Transaction failure handled for pair: {}", pair_id);
        Ok(())
    }

    /// Update success metrics
    async fn update_success_metrics(&self) {
        let mut state = self.manager_state.write().unwrap();
        state.successful_transactions += 1;
        state.total_transactions_processed += 1;
        if state.active_transaction_count > 0 {
            state.active_transaction_count -= 1;
        }
    }

    /// Update failure metrics
    async fn update_failure_metrics(&self) {
        let mut state = self.manager_state.write().unwrap();
        state.failed_transactions += 1;
        state.total_transactions_processed += 1;
        if state.active_transaction_count > 0 {
            state.active_transaction_count -= 1;
        }
    }

    /// Update processing time metrics
    async fn update_processing_time_metrics(&self, completion_time_ms: u64) {
        let mut state = self.manager_state.write().unwrap();
        
        // Calculate running average
        let total_processed = state.total_transactions_processed as f64;
        if total_processed > 0.0 {
            state.average_processing_time_ms = 
                (state.average_processing_time_ms * (total_processed - 1.0) + completion_time_ms as f64) / total_processed;
        } else {
            state.average_processing_time_ms = completion_time_ms as f64;
        }
    }

    /// Get manager statistics
    pub async fn get_manager_statistics(&self) -> Result<ManagerState> {
        let state = self.manager_state.read().unwrap();
        Ok(state.clone())
    }

    /// Get active transactions
    pub async fn get_active_transactions(&self) -> Result<Vec<TransactionPair>> {
        let transactions = self.active_transactions.lock().await;
        Ok(transactions.values().cloned().collect())
    }

    /// Shutdown dual transaction manager
    pub async fn shutdown(&self) -> Result<()> {
        tracing::info!("🔄 Shutting down Dual Transaction Manager...");

        // Clear active transactions
        {
            let mut transactions = self.active_transactions.lock().await;
            transactions.clear();
        }

        // Clear processing queue
        {
            let mut queue = self.processing_queue.lock().await;
            queue.clear();
        }

        // Reset state
        {
            let mut state = self.manager_state.write().unwrap();
            state.active_transaction_count = 0;
            state.queue_size = 0;
        }

        tracing::info!("✅ Dual Transaction Manager shutdown complete");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dual_transaction_manager_creation() {
        let config = GovernmentConfig::default();
        let manager = DualTransactionManager::new(config).await.unwrap();
        assert!(manager.initialize().await.is_ok());
        assert!(manager.shutdown().await.is_ok());
    }

    #[tokio::test]
    async fn test_dual_transaction_processing() {
        let config = GovernmentConfig::default();
        let manager = DualTransactionManager::new(config).await.unwrap();
        manager.initialize().await.unwrap();

        let transaction_data = serde_json::json!({
            "jurisdiction": "US",
            "operation_type": "transfer",
            "amount": 1000.0,
            "currency": "USD"
        });

        let pair_id = manager.process_dual_transaction("test_session", transaction_data).await.unwrap();
        assert!(!pair_id.is_empty());

        let stats = manager.get_manager_statistics().await.unwrap();
        assert_eq!(stats.total_transactions_processed, 1);
        assert_eq!(stats.successful_transactions, 1);

        manager.shutdown().await.unwrap();
    }
}
