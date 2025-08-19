//! Receipt query and filtering capabilities
//!
//! This module provides advanced querying capabilities for transaction receipts,
//! including filtering by various criteria, pagination, and efficient lookups.

use std::collections::HashMap;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::{TransactionReceipt, TransactionStatus, ReceiptStore};

/// Query filter for receipts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptFilter {
    /// Filter by block height range
    pub block_height_range: Option<(u64, u64)>,
    /// Filter by specific block hash
    pub block_hash: Option<[u8; 32]>,
    /// Filter by sender address
    pub from_address: Option<[u8; 20]>,
    /// Filter by recipient address
    pub to_address: Option<[u8; 20]>,
    /// Filter by contract address
    pub contract_address: Option<[u8; 20]>,
    /// Filter by transaction status
    pub status: Option<TransactionStatus>,
    /// Filter by timestamp range
    pub timestamp_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    /// Filter by gas usage range
    pub gas_used_range: Option<(u64, u64)>,
    /// Filter by event log address
    pub event_address: Option<[u8; 20]>,
    /// Filter by event topics
    pub event_topics: Option<Vec<[u8; 32]>>,
}

/// Pagination parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    /// Number of results to return
    pub limit: u32,
    /// Number of results to skip
    pub offset: u32,
    /// Sort order (true = ascending, false = descending)
    pub ascending: bool,
}

/// Query result with pagination info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptQueryResult {
    /// Matching receipts
    pub receipts: Vec<TransactionReceipt>,
    /// Total number of matching receipts
    pub total_count: u64,
    /// Current page offset
    pub offset: u32,
    /// Number of results returned
    pub limit: u32,
    /// Whether there are more results
    pub has_more: bool,
}

/// Receipt statistics aggregation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptStats {
    /// Total number of receipts
    pub total_receipts: u64,
    /// Number of successful transactions
    pub successful_txs: u64,
    /// Number of failed transactions
    pub failed_txs: u64,
    /// Total gas used
    pub total_gas_used: u64,
    /// Total gas fees paid
    pub total_gas_fees: u64,
    /// Average gas price
    pub avg_gas_price: u64,
    /// Unique addresses involved
    pub unique_addresses: u64,
    /// Total event logs
    pub total_events: u64,
}

impl Default for ReceiptFilter {
    fn default() -> Self {
        Self {
            block_height_range: None,
            block_hash: None,
            from_address: None,
            to_address: None,
            contract_address: None,
            status: None,
            timestamp_range: None,
            gas_used_range: None,
            event_address: None,
            event_topics: None,
        }
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            limit: 100,
            offset: 0,
            ascending: false, // Default to newest first
        }
    }
}

impl ReceiptStore {
    /// Query receipts with filtering and pagination
    pub async fn query_receipts(
        &self,
        filter: &ReceiptFilter,
        pagination: &Pagination,
    ) -> Result<ReceiptQueryResult> {
        let mut conditions = Vec::new();
        let mut bind_values = Vec::new();
        
        if let Some(block_hash) = filter.block_hash {
            conditions.push("block_hash = ?".to_string());
            bind_values.push(block_hash.to_vec());
        }
        
        if let Some((start_height, end_height)) = filter.block_height_range {
            conditions.push("block_height >= ? AND block_height <= ?".to_string());
            bind_values.push((start_height as i64).to_string().into_bytes());
            bind_values.push((end_height as i64).to_string().into_bytes());
        }
        
        if let Some(from_addr) = filter.from_address {
            conditions.push("from_address = ?".to_string());
            bind_values.push(from_addr.to_vec());
        }
        
        if let Some(to_addr) = filter.to_address {
            conditions.push("to_address = ?".to_string());
            bind_values.push(to_addr.to_vec());
        }
        
        if let Some(contract_addr) = filter.contract_address {
            conditions.push("contract_address = ?".to_string());
            bind_values.push(contract_addr.to_vec());
        }
        
        if let Some(ref status) = filter.status {
            conditions.push("status = ?".to_string());
            bind_values.push(self.status_to_string(status).into_bytes());
        }
        
        if let Some((start_time, end_time)) = filter.timestamp_range {
            conditions.push("timestamp >= ? AND timestamp <= ?".to_string());
            bind_values.push(start_time.to_rfc3339().into_bytes());
            bind_values.push(end_time.to_rfc3339().into_bytes());
        }
        
        if let Some((min_gas, max_gas)) = filter.gas_used_range {
            conditions.push("gas_used >= ? AND gas_used <= ?".to_string());
            bind_values.push((min_gas as i64).to_string().into_bytes());
            bind_values.push((max_gas as i64).to_string().into_bytes());
        }
        
        // Build WHERE clause
        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };
        
        // Build ORDER BY clause
        let order_clause = if pagination.ascending {
            "ORDER BY block_height ASC, tx_index ASC"
        } else {
            "ORDER BY block_height DESC, tx_index DESC"
        };
        
        // Count total matching receipts
        let count_query = format!(
            "SELECT COUNT(*) FROM receipts {}",
            where_clause
        );
        
        let mut count_query_builder = sqlx::query_scalar(&count_query);
        
        // Bind filter parameters for count query
        for bind_value in &bind_values {
            if let Ok(s) = String::from_utf8(bind_value.clone()) {
                count_query_builder = count_query_builder.bind(s);
            } else {
                count_query_builder = count_query_builder.bind(bind_value.clone());
            }
        }
        
        let total_count: i64 = count_query_builder
            .fetch_one(&self.pool)
            .await
            .context("Failed to count receipts")?;
        
        // Query receipts with pagination
        let query = format!(
            "SELECT receipt_data FROM receipts {} {} LIMIT ? OFFSET ?",
            where_clause, order_clause
        );
        
        let mut query_builder = sqlx::query(&query);
        
        // Bind filter parameters
        for bind_value in bind_values {
            if let Ok(s) = String::from_utf8(bind_value.clone()) {
                query_builder = query_builder.bind(s);
            } else {
                query_builder = query_builder.bind(bind_value);
            }
        }
        
        // Bind pagination parameters
        query_builder = query_builder
            .bind(pagination.limit as i64)
            .bind(pagination.offset as i64);
        
        let rows = query_builder
            .fetch_all(&self.pool)
            .await
            .context("Failed to query receipts")?;
        
        let mut receipts = Vec::new();
        for row in rows {
            let receipt_data: Vec<u8> = row.get("receipt_data");
            let receipt: TransactionReceipt = bincode::deserialize(&receipt_data)
                .context("Failed to deserialize receipt")?;
            receipts.push(receipt);
        }
        
        Ok(ReceiptQueryResult {
            receipts,
            total_count: total_count as u64,
            offset: pagination.offset,
            limit: pagination.limit,
            has_more: (pagination.offset as u64 + pagination.limit as u64) < total_count as u64,
        })
    }
    
    /// Get receipts by event log criteria
    pub async fn query_receipts_by_events(
        &self,
        address: Option<[u8; 20]>,
        topics: Option<Vec<[u8; 32]>>,
        pagination: &Pagination,
    ) -> Result<ReceiptQueryResult> {
        let mut conditions = Vec::new();
        
        if address.is_some() {
            conditions.push("event_logs.address = ?".to_string());
        }
        
        // Build topic conditions
        if let Some(ref topic_list) = topics {
            for (i, _) in topic_list.iter().enumerate() {
                conditions.push(format!("JSON_EXTRACT(event_logs.topics, '$[{}]') = ?", i));
            }
        }
        
        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };
        
        let order_clause = if pagination.ascending {
            "ORDER BY receipts.block_height ASC, receipts.tx_index ASC"
        } else {
            "ORDER BY receipts.block_height DESC, receipts.tx_index DESC"
        };
        
        // Count total matching receipts
        let count_query = format!(
            "SELECT COUNT(DISTINCT receipts.tx_hash) FROM receipts 
             JOIN event_logs ON receipts.tx_hash = event_logs.tx_hash {}",
            where_clause
        );
        
        let total_count: i64 = sqlx::query_scalar(&count_query)
            .fetch_one(&self.pool)
            .await
            .context("Failed to count receipts by events")?;
        
        // Query receipts
        let query = format!(
            "SELECT DISTINCT receipts.receipt_data FROM receipts 
             JOIN event_logs ON receipts.tx_hash = event_logs.tx_hash 
             {} {} LIMIT ? OFFSET ?",
            where_clause, order_clause
        );
        
        let mut query_builder = sqlx::query(&query);
        
        if let Some(addr) = address {
            query_builder = query_builder.bind(addr.to_vec());
        }
        
        if let Some(ref topic_list) = topics {
            for topic in topic_list {
                query_builder = query_builder.bind(&topic[..]);
            }
        }
        
        query_builder = query_builder
            .bind(pagination.limit as i64)
            .bind(pagination.offset as i64);
        
        let rows = query_builder
            .fetch_all(&self.pool)
            .await
            .context("Failed to query receipts by events")?;
        
        let mut receipts = Vec::new();
        for row in rows {
            let receipt_data: Vec<u8> = row.get("receipt_data");
            let receipt: TransactionReceipt = bincode::deserialize(&receipt_data)
                .context("Failed to deserialize receipt")?;
            receipts.push(receipt);
        }
        
        Ok(ReceiptQueryResult {
            receipts,
            total_count: total_count as u64,
            offset: pagination.offset,
            limit: pagination.limit,
            has_more: (pagination.offset as u64 + pagination.limit as u64) < total_count as u64,
        })
    }
    
    /// Get aggregated statistics for receipts
    pub async fn get_receipt_stats(&self, filter: &ReceiptFilter) -> Result<ReceiptStats> {
        // This is a simplified implementation
        // In practice, you'd build dynamic queries based on the filter
        
        let total_receipts: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM receipts")
            .fetch_one(&self.pool)
            .await
            .context("Failed to count total receipts")?;
        
        let successful_txs: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM receipts WHERE status = 'success'")
            .fetch_one(&self.pool)
            .await
            .context("Failed to count successful transactions")?;
        
        let failed_txs = total_receipts - successful_txs;
        
        let total_gas_used: Option<i64> = sqlx::query_scalar("SELECT SUM(gas_used) FROM receipts")
            .fetch_one(&self.pool)
            .await
            .context("Failed to sum gas used")?;
        
        let total_gas_fees: Option<i64> = sqlx::query_scalar("SELECT SUM(gas_fee) FROM receipts")
            .fetch_one(&self.pool)
            .await
            .context("Failed to sum gas fees")?;
        
        let avg_gas_price: Option<f64> = sqlx::query_scalar("SELECT AVG(gas_price) FROM receipts")
            .fetch_one(&self.pool)
            .await
            .context("Failed to calculate average gas price")?;
        
        let unique_addresses: i64 = sqlx::query_scalar(
            "SELECT COUNT(DISTINCT from_address) + COUNT(DISTINCT to_address) FROM receipts"
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to count unique addresses")?;
        
        let total_events: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM event_logs")
            .fetch_one(&self.pool)
            .await
            .context("Failed to count total events")?;
        
        Ok(ReceiptStats {
            total_receipts: total_receipts as u64,
            successful_txs: successful_txs as u64,
            failed_txs: failed_txs as u64,
            total_gas_used: total_gas_used.unwrap_or(0) as u64,
            total_gas_fees: total_gas_fees.unwrap_or(0) as u64,
            avg_gas_price: avg_gas_price.unwrap_or(0.0) as u64,
            unique_addresses: unique_addresses as u64,
            total_events: total_events as u64,
        })
    }
    
    /// Search receipts by bloom filter (efficient log filtering)
    pub async fn search_by_bloom_filter(
        &self,
        bloom_filter: &[u8],
        pagination: &Pagination,
    ) -> Result<ReceiptQueryResult> {
        // This would implement bloom filter matching
        // For now, we'll do a simple implementation
        
        let order_clause = if pagination.ascending {
            "ORDER BY block_height ASC, tx_index ASC"
        } else {
            "ORDER BY block_height DESC, tx_index DESC"
        };
        
        let query = format!(
            "SELECT receipt_data FROM receipts {} LIMIT ? OFFSET ?",
            order_clause
        );
        
        let rows = sqlx::query(&query)
            .bind(pagination.limit as i64)
            .bind(pagination.offset as i64)
            .fetch_all(&self.pool)
            .await
            .context("Failed to search by bloom filter")?;
        
        let mut matching_receipts = Vec::new();
        for row in rows {
            let receipt_data: Vec<u8> = row.get("receipt_data");
            let receipt: TransactionReceipt = bincode::deserialize(&receipt_data)
                .context("Failed to deserialize receipt")?;
            
            // Check if receipt's bloom filter matches the search filter
            if self.bloom_filter_matches(&receipt.logs_bloom, bloom_filter) {
                matching_receipts.push(receipt);
            }
        }
        
        let total_count = matching_receipts.len() as u64;
        
        Ok(ReceiptQueryResult {
            receipts: matching_receipts,
            total_count,
            offset: pagination.offset,
            limit: pagination.limit,
            has_more: false, // Simplified for this implementation
        })
    }
    
    /// Check if two bloom filters have any matching bits
    fn bloom_filter_matches(&self, receipt_bloom: &[u8], search_bloom: &[u8]) -> bool {
        let min_len = receipt_bloom.len().min(search_bloom.len());
        for i in 0..min_len {
            if (receipt_bloom[i] & search_bloom[i]) != 0 {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use crate::{ReceiptConfig, EventLog, GasUsage};
    
    async fn create_test_store() -> Result<ReceiptStore> {
        let config = ReceiptConfig {
            database_url: "sqlite::memory:".to_string(),
            cache_size: 100,
            batch_size: 10,
            enable_bloom_indexing: true,
            finality_timeout_ms: 1000,
        };
        
        ReceiptStore::new(config).await
    }
    
    fn create_test_receipts() -> Vec<TransactionReceipt> {
        let mut receipts = Vec::new();
        
        for i in 0..5 {
            let tx_hash = [i as u8; 32];
            let block_hash = [1u8; 32];
            let from = [i as u8; 20];
            let to = Some([(i + 1) as u8; 20]);
            
            let gas = GasUsage {
                gas_limit: 21000 + i as u64 * 1000,
                gas_used: 21000 + i as u64 * 500,
                gas_price: 20_000_000_000 + i as u64 * 1_000_000_000,
                gas_fee: (21000 + i as u64 * 500) * (20_000_000_000 + i as u64 * 1_000_000_000),
            };
            
            let status = if i % 2 == 0 {
                TransactionStatus::Success
            } else {
                TransactionStatus::Failed("Test error".to_string())
            };
            
            let logs = vec![
                EventLog {
                    address: [i as u8; 20],
                    topics: vec![[i as u8; 32]],
                    data: vec![i as u8],
                    log_index: 0,
                }
            ];
            
            let receipt = TransactionReceipt::new(
                tx_hash,
                block_hash,
                100 + i as u64,
                i as u32,
                from,
                to,
                status,
                gas,
                logs,
            );
            
            receipts.push(receipt);
        }
        
        receipts
    }
    
    #[tokio::test]
    async fn test_query_receipts_with_filter() {
        let store = create_test_store().await.unwrap();
        let receipts = create_test_receipts();
        
        // Store test receipts
        for receipt in &receipts {
            store.store_receipt(receipt).await.unwrap();
        }
        
        // Test filtering by status
        let filter = ReceiptFilter {
            status: Some(TransactionStatus::Success),
            ..Default::default()
        };
        
        let pagination = Pagination::default();
        let result = store.query_receipts(&filter, &pagination).await.unwrap();
        
        // Should have 3 successful transactions (indices 0, 2, 4)
        assert_eq!(result.receipts.len(), 3);
        for receipt in &result.receipts {
            assert!(receipt.is_success());
        }
    }
    
    #[tokio::test]
    async fn test_query_receipts_with_pagination() {
        let store = create_test_store().await.unwrap();
        let receipts = create_test_receipts();
        
        // Store test receipts
        for receipt in &receipts {
            store.store_receipt(receipt).await.unwrap();
        }
        
        let filter = ReceiptFilter::default();
        let pagination = Pagination {
            limit: 2,
            offset: 0,
            ascending: false,
        };
        
        let result = store.query_receipts(&filter, &pagination).await.unwrap();
        
        assert_eq!(result.receipts.len(), 2);
        assert_eq!(result.total_count, 5);
        assert!(result.has_more);
    }
    
    #[tokio::test]
    async fn test_receipt_stats() {
        let store = create_test_store().await.unwrap();
        let receipts = create_test_receipts();
        
        // Store test receipts
        for receipt in &receipts {
            store.store_receipt(receipt).await.unwrap();
        }
        
        let filter = ReceiptFilter::default();
        let stats = store.get_receipt_stats(&filter).await.unwrap();
        
        assert_eq!(stats.total_receipts, 5);
        assert_eq!(stats.successful_txs, 3);
        assert_eq!(stats.failed_txs, 2);
        assert!(stats.total_gas_used > 0);
        assert!(stats.total_gas_fees > 0);
    }
}
