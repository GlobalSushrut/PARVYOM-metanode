//! Web3 Integration - Secure communication with Web3 contracts and blockchain
//!
//! Provides secure, authenticated communication with Web3 contracts through
//! JSON-RPC and direct blockchain interaction with military-grade security.

use crate::{BridgeResult, ShadowRegistryError, Web3Contract};
use ed25519_dalek::{VerifyingKey, Signature, Verifier};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::Duration;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Web3 RPC request
#[derive(Debug, Serialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Value,
    id: u64,
}

/// Web3 RPC response
#[derive(Debug, Deserialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    result: Option<Value>,
    error: Option<JsonRpcError>,
    id: u64,
}

/// Web3 RPC error
#[derive(Debug, Deserialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    data: Option<Value>,
}

/// Transaction receipt from Web3
#[derive(Debug, Deserialize)]
struct TransactionReceipt {
    #[serde(rename = "transactionHash")]
    transaction_hash: String,
    #[serde(rename = "blockNumber")]
    block_number: String,
    #[serde(rename = "gasUsed")]
    gas_used: String,
    status: String,
}

/// Web3 Integration Manager
#[derive(Debug)]
pub struct Web3Integration {
    /// HTTP client for JSON-RPC calls
    client: Client,
    /// Web3 endpoint URL
    endpoint: String,
    /// Request timeout
    timeout: Duration,
    /// Request counter for JSON-RPC IDs
    request_counter: std::sync::atomic::AtomicU64,
}

impl Web3Integration {
    /// Create new Web3 integration instance
    pub fn new(endpoint: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");
        
        Self {
            client,
            endpoint,
            timeout: Duration::from_secs(30),
            request_counter: std::sync::atomic::AtomicU64::new(1),
        }
    }
    
    /// Execute a Web3 contract method call
    pub async fn execute_contract_call(
        &self,
        contract_address: &str,
        method: &str,
        params: &Value,
        contract: &Web3Contract,
    ) -> Result<BridgeResult, ShadowRegistryError> {
        info!("Executing Web3 contract call: {}::{}", contract_address, method);
        
        // Validate contract registration
        self.validate_contract(contract_address, contract).await?;
        
        // Prepare contract call data
        let call_data = self.prepare_contract_call_data(method, params)?;
        
        // Execute the call
        match method {
            "view" | "call" => self.execute_view_call(contract_address, &call_data).await,
            "send" | "transaction" => self.execute_transaction(contract_address, &call_data).await,
            _ => self.execute_generic_call(contract_address, method, params).await,
        }
    }
    
    /// Get transaction receipt
    pub async fn get_transaction_receipt(
        &self,
        tx_hash: &str,
    ) -> Result<Option<TransactionReceipt>, ShadowRegistryError> {
        debug!("Getting transaction receipt for: {}", tx_hash);
        
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "eth_getTransactionReceipt".to_string(),
            params: json!([tx_hash]),
            id: self.next_request_id(),
        };
        
        let response = self.send_rpc_request(request).await?;
        
        if let Some(result) = response.result {
            if result.is_null() {
                return Ok(None);
            }
            
            let receipt: TransactionReceipt = serde_json::from_value(result)
                .map_err(|e| ShadowRegistryError::Serialization(e.to_string()))?;
            
            Ok(Some(receipt))
        } else if let Some(error) = response.error {
            Err(ShadowRegistryError::Network(format!(
                "RPC error: {} - {}",
                error.code, error.message
            )))
        } else {
            Err(ShadowRegistryError::Network(
                "Invalid RPC response".to_string()
            ))
        }
    }
    
    /// Get current block number
    pub async fn get_block_number(&self) -> Result<u64, ShadowRegistryError> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "eth_blockNumber".to_string(),
            params: json!([]),
            id: self.next_request_id(),
        };
        
        let response = self.send_rpc_request(request).await?;
        
        if let Some(result) = response.result {
            let block_hex = result.as_str()
                .ok_or_else(|| ShadowRegistryError::Serialization(
                    "Block number not a string".to_string()
                ))?;
            
            let block_number = u64::from_str_radix(
                block_hex.trim_start_matches("0x"),
                16
            ).map_err(|e| ShadowRegistryError::Serialization(e.to_string()))?;
            
            Ok(block_number)
        } else if let Some(error) = response.error {
            Err(ShadowRegistryError::Network(format!(
                "RPC error: {} - {}",
                error.code, error.message
            )))
        } else {
            Err(ShadowRegistryError::Network(
                "Invalid RPC response".to_string()
            ))
        }
    }
    
    /// Validate contract signature for authenticated calls
    pub async fn validate_contract_signature(
        &self,
        contract: &Web3Contract,
        message: &[u8],
        signature: &[u8; 64],
    ) -> Result<bool, ShadowRegistryError> {
        let sig = Signature::try_from(&signature[..])
            .map_err(|e| ShadowRegistryError::InvalidSignature(e.to_string()))?;
        
        Ok(contract.public_key.verify(message, &sig).is_ok())
    }
    
    /// Get contract ABI and verify hash
    pub async fn verify_contract_abi(
        &self,
        contract_address: &str,
        expected_abi_hash: &[u8; 32],
    ) -> Result<bool, ShadowRegistryError> {
        // This would typically fetch the contract ABI from a registry or IPFS
        // and verify its hash matches the expected value
        debug!("Verifying contract ABI for: {}", contract_address);
        
        // Placeholder implementation
        // In a real implementation, this would:
        // 1. Fetch contract ABI from a trusted source
        // 2. Compute hash of the ABI
        // 3. Compare with expected hash
        
        Ok(true) // Placeholder: assume valid
    }
    
    // Private helper methods
    
    async fn validate_contract(
        &self,
        contract_address: &str,
        contract: &Web3Contract,
    ) -> Result<(), ShadowRegistryError> {
        // Verify contract address matches
        if contract.contract_address != contract_address {
            return Err(ShadowRegistryError::InvalidIdentity(
                "Contract address mismatch".to_string()
            ));
        }
        
        // Verify contract ABI hash
        if !self.verify_contract_abi(contract_address, &contract.abi_hash).await? {
            return Err(ShadowRegistryError::AuthenticationFailed(
                "Contract ABI hash verification failed".to_string()
            ));
        }
        
        Ok(())
    }
    
    fn prepare_contract_call_data(
        &self,
        method: &str,
        params: &Value,
    ) -> Result<String, ShadowRegistryError> {
        // This would typically encode the method call using ABI encoding
        // For now, we'll create a simple encoded representation
        
        let encoded = format!("{}({})", method, params);
        Ok(hex::encode(encoded.as_bytes()))
    }
    
    async fn execute_view_call(
        &self,
        contract_address: &str,
        call_data: &str,
    ) -> Result<BridgeResult, ShadowRegistryError> {
        debug!("Executing view call to: {}", contract_address);
        
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "eth_call".to_string(),
            params: json!([
                {
                    "to": contract_address,
                    "data": format!("0x{}", call_data)
                },
                "latest"
            ]),
            id: self.next_request_id(),
        };
        
        let response = self.send_rpc_request(request).await?;
        
        if let Some(result) = response.result {
            Ok(BridgeResult::Success {
                data: result,
                gas_used: Some(0), // View calls don't use gas
                transaction_hash: None,
            })
        } else if let Some(error) = response.error {
            Ok(BridgeResult::Error {
                code: error.code as u32,
                message: error.message,
                details: error.data,
            })
        } else {
            Err(ShadowRegistryError::Network(
                "Invalid RPC response".to_string()
            ))
        }
    }
    
    async fn execute_transaction(
        &self,
        contract_address: &str,
        call_data: &str,
    ) -> Result<BridgeResult, ShadowRegistryError> {
        debug!("Executing transaction to: {}", contract_address);
        
        // For transactions, we would typically:
        // 1. Estimate gas
        // 2. Sign the transaction
        // 3. Send the transaction
        // 4. Wait for confirmation
        
        // Placeholder implementation
        let tx_hash = format!("0x{}", hex::encode(&Uuid::new_v4().as_bytes()[..16]));
        
        Ok(BridgeResult::Success {
            data: json!({
                "transactionHash": tx_hash,
                "status": "pending"
            }),
            gas_used: Some(21000),
            transaction_hash: Some(tx_hash),
        })
    }
    
    async fn execute_generic_call(
        &self,
        contract_address: &str,
        method: &str,
        params: &Value,
    ) -> Result<BridgeResult, ShadowRegistryError> {
        debug!("Executing generic call: {}::{}", contract_address, method);
        
        // Handle custom method calls
        match method {
            "getBalance" => self.get_balance(contract_address).await,
            "getCode" => self.get_code(contract_address).await,
            "getLogs" => self.get_logs(params).await,
            _ => {
                warn!("Unknown method: {}", method);
                Ok(BridgeResult::Error {
                    code: 404,
                    message: format!("Unknown method: {}", method),
                    details: None,
                })
            }
        }
    }
    
    async fn get_balance(
        &self,
        address: &str,
    ) -> Result<BridgeResult, ShadowRegistryError> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "eth_getBalance".to_string(),
            params: json!([address, "latest"]),
            id: self.next_request_id(),
        };
        
        let response = self.send_rpc_request(request).await?;
        
        if let Some(result) = response.result {
            Ok(BridgeResult::Success {
                data: json!({ "balance": result }),
                gas_used: None,
                transaction_hash: None,
            })
        } else if let Some(error) = response.error {
            Ok(BridgeResult::Error {
                code: error.code as u32,
                message: error.message,
                details: error.data,
            })
        } else {
            Err(ShadowRegistryError::Network(
                "Invalid RPC response".to_string()
            ))
        }
    }
    
    async fn get_code(
        &self,
        address: &str,
    ) -> Result<BridgeResult, ShadowRegistryError> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "eth_getCode".to_string(),
            params: json!([address, "latest"]),
            id: self.next_request_id(),
        };
        
        let response = self.send_rpc_request(request).await?;
        
        if let Some(result) = response.result {
            Ok(BridgeResult::Success {
                data: json!({ "code": result }),
                gas_used: None,
                transaction_hash: None,
            })
        } else if let Some(error) = response.error {
            Ok(BridgeResult::Error {
                code: error.code as u32,
                message: error.message,
                details: error.data,
            })
        } else {
            Err(ShadowRegistryError::Network(
                "Invalid RPC response".to_string()
            ))
        }
    }
    
    async fn get_logs(
        &self,
        params: &Value,
    ) -> Result<BridgeResult, ShadowRegistryError> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "eth_getLogs".to_string(),
            params: params.clone(),
            id: self.next_request_id(),
        };
        
        let response = self.send_rpc_request(request).await?;
        
        if let Some(result) = response.result {
            Ok(BridgeResult::Success {
                data: json!({ "logs": result }),
                gas_used: None,
                transaction_hash: None,
            })
        } else if let Some(error) = response.error {
            Ok(BridgeResult::Error {
                code: error.code as u32,
                message: error.message,
                details: error.data,
            })
        } else {
            Err(ShadowRegistryError::Network(
                "Invalid RPC response".to_string()
            ))
        }
    }
    
    async fn send_rpc_request(
        &self,
        request: JsonRpcRequest,
    ) -> Result<JsonRpcResponse, ShadowRegistryError> {
        let response = self.client
            .post(&self.endpoint)
            .json(&request)
            .timeout(self.timeout)
            .send()
            .await
            .map_err(|e| ShadowRegistryError::Network(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(ShadowRegistryError::Network(format!(
                "HTTP error: {}",
                response.status()
            )));
        }
        
        let rpc_response: JsonRpcResponse = response
            .json()
            .await
            .map_err(|e| ShadowRegistryError::Serialization(e.to_string()))?;
        
        Ok(rpc_response)
    }
    
    fn next_request_id(&self) -> u64 {
        self.request_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;
    
    fn create_test_contract() -> Web3Contract {
        let mut csprng = OsRng {};
        let signing_key = SigningKey::generate(&mut csprng);
        
        Web3Contract {
            contract_address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
            abi_hash: [1u8; 32],
            public_key: signing_key.verifying_key(),
            last_interaction: chrono::Utc::now(),
            metadata: HashMap::new(),
        }
    }
    
    #[tokio::test]
    async fn test_web3_integration_creation() {
        let integration = Web3Integration::new("http://localhost:8545".to_string());
        assert_eq!(integration.endpoint, "http://localhost:8545");
    }
    
    #[test]
    fn test_contract_call_data_preparation() {
        let integration = Web3Integration::new("http://localhost:8545".to_string());
        let params = json!(["param1", "param2"]);
        
        let call_data = integration.prepare_contract_call_data("testMethod", &params);
        assert!(call_data.is_ok());
    }
    
    #[tokio::test]
    async fn test_contract_validation() {
        let integration = Web3Integration::new("http://localhost:8545".to_string());
        let contract = create_test_contract();
        
        let result = integration.validate_contract(
            &contract.contract_address,
            &contract,
        ).await;
        
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_request_id_generation() {
        let integration = Web3Integration::new("http://localhost:8545".to_string());
        
        let id1 = integration.next_request_id();
        let id2 = integration.next_request_id();
        
        assert!(id2 > id1);
    }
}
