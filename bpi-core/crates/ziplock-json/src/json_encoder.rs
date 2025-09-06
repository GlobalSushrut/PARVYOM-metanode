//! JSON encoding with I-JSON enforcement for ZIPLOCK-JSON

use serde_json::{Value, Map};
use std::collections::BTreeMap;
use crate::{ZjlResult, ZjlError};
use crate::blocks::{BlockType, Block, BlockHeader};
use blake3::Hasher;

/// I-JSON enforcer that validates and normalizes JSON
pub struct IJSONEnforcer;

impl IJSONEnforcer {
    /// Enforce I-JSON compliance on a JSON value
    pub fn enforce(value: &Value) -> ZjlResult<Value> {
        match value {
            Value::Object(obj) => {
                // Sort keys for deterministic output
                let mut sorted_map = BTreeMap::new();
                for (key, val) in obj {
                    // Validate key is valid UTF-8 (already guaranteed by serde_json)
                    // Recursively enforce I-JSON on value
                    let enforced_val = Self::enforce(val)?;
                    sorted_map.insert(key.clone(), enforced_val);
                }
                
                // Convert back to serde_json::Map maintaining order
                let mut result_map = Map::new();
                for (key, val) in sorted_map {
                    result_map.insert(key, val);
                }
                Ok(Value::Object(result_map))
            }
            Value::Array(arr) => {
                let mut enforced_arr = Vec::new();
                for item in arr {
                    enforced_arr.push(Self::enforce(item)?);
                }
                Ok(Value::Array(enforced_arr))
            }
            Value::String(s) => {
                // Ensure valid UTF-8 (already guaranteed by serde_json)
                // Check for control characters that might not be I-JSON compliant
                if s.chars().any(|c| c.is_control() && c != '\t' && c != '\n' && c != '\r') {
                    return Err(ZjlError::InvalidJson("String contains invalid control characters".to_string()));
                }
                Ok(value.clone())
            }
            Value::Number(n) => {
                // I-JSON requires finite numbers
                if let Some(f) = n.as_f64() {
                    if !f.is_finite() {
                        return Err(ZjlError::InvalidJson("Number must be finite".to_string()));
                    }
                }
                Ok(value.clone())
            }
            Value::Bool(_) | Value::Null => Ok(value.clone()),
        }
    }

    /// Serialize value to canonical I-JSON bytes
    pub fn to_canonical_bytes(value: &Value) -> ZjlResult<Vec<u8>> {
        let enforced = Self::enforce(value)?;
        // Use compact serialization for deterministic output
        serde_json::to_vec(&enforced)
            .map_err(|e| ZjlError::InvalidJson(e.to_string()))
    }

    /// Validate that JSON string is I-JSON compliant
    pub fn validate_json_str(json_str: &str) -> ZjlResult<Value> {
        let value: Value = serde_json::from_str(json_str)
            .map_err(|e| ZjlError::InvalidJson(e.to_string()))?;
        Self::enforce(&value)
    }
}

/// JSON chunk encoder that splits large JSON into blocks
pub struct JsonChunkEncoder {
    /// Maximum chunk size in bytes
    max_chunk_size: usize,
    /// Path ID counter
    next_path_id: u64,
}

impl JsonChunkEncoder {
    pub fn new(max_chunk_size: usize) -> Self {
        Self {
            max_chunk_size,
            next_path_id: 1,
        }
    }

    /// Encode a JSON value into blocks
    pub fn encode_value(&mut self, value: &Value, path_prefix: &str) -> ZjlResult<Vec<Block>> {
        let enforced_value = IJSONEnforcer::enforce(value)?;
        self.encode_value_recursive(&enforced_value, path_prefix, 0)
    }

    fn encode_value_recursive(&mut self, value: &Value, path: &str, depth: usize) -> ZjlResult<Vec<Block>> {
        let mut blocks = Vec::new();

        match value {
            Value::Object(obj) => {
                // For small objects, encode as single block
                let serialized = IJSONEnforcer::to_canonical_bytes(value)?;
                if serialized.len() <= self.max_chunk_size {
                    let block = self.create_json_block(BlockType::JsonObject, serialized)?;
                    blocks.push(block);
                } else {
                    // Split large objects into separate blocks per key
                    for (key, val) in obj {
                        let key_path = if path.is_empty() {
                            key.clone()
                        } else {
                            format!("{}.{}", path, key)
                        };
                        let mut sub_blocks = self.encode_value_recursive(val, &key_path, depth + 1)?;
                        blocks.append(&mut sub_blocks);
                    }
                }
            }
            Value::Array(arr) => {
                let serialized = IJSONEnforcer::to_canonical_bytes(value)?;
                if serialized.len() <= self.max_chunk_size {
                    let block = self.create_json_block(BlockType::JsonArray, serialized)?;
                    blocks.push(block);
                } else {
                    // Split large arrays into chunks
                    let chunk_size = self.calculate_array_chunk_size(arr.len());
                    for (i, chunk) in arr.chunks(chunk_size).enumerate() {
                        let chunk_value = Value::Array(chunk.to_vec());
                        let chunk_serialized = IJSONEnforcer::to_canonical_bytes(&chunk_value)?;
                        let mut block = self.create_json_block(BlockType::JsonChunked, chunk_serialized)?;
                        // Use same path ID for all chunks of the same array
                        if i == 0 {
                            // First chunk gets new path ID
                        } else {
                            // Subsequent chunks reuse the same path ID
                            block.header.path_id = blocks.last().unwrap().header.path_id;
                        }
                        blocks.push(block);
                    }
                }
            }
            Value::String(s) => {
                let serialized = IJSONEnforcer::to_canonical_bytes(value)?;
                if serialized.len() <= self.max_chunk_size {
                    let block = self.create_json_block(BlockType::JsonString, serialized)?;
                    blocks.push(block);
                } else {
                    // Split large strings into chunks
                    let chunks = self.split_string_safely(s, self.max_chunk_size - 10); // Leave room for JSON quotes
                    for chunk in chunks {
                        let chunk_value = Value::String(chunk);
                        let chunk_serialized = IJSONEnforcer::to_canonical_bytes(&chunk_value)?;
                        let block = self.create_json_block(BlockType::JsonChunked, chunk_serialized)?;
                        blocks.push(block);
                    }
                }
            }
            Value::Number(_) => {
                let serialized = IJSONEnforcer::to_canonical_bytes(value)?;
                let block = self.create_json_block(BlockType::JsonNumber, serialized)?;
                blocks.push(block);
            }
            Value::Bool(_) => {
                let serialized = IJSONEnforcer::to_canonical_bytes(value)?;
                let block = self.create_json_block(BlockType::JsonBool, serialized)?;
                blocks.push(block);
            }
            Value::Null => {
                let serialized = IJSONEnforcer::to_canonical_bytes(value)?;
                let block = self.create_json_block(BlockType::JsonNull, serialized)?;
                blocks.push(block);
            }
        }

        Ok(blocks)
    }

    fn create_json_block(&mut self, block_type: BlockType, payload: Vec<u8>) -> ZjlResult<Block> {
        let path_id = self.next_path_id;
        self.next_path_id += 1;

        // Calculate BLAKE3 hash
        let mut hasher = Hasher::new();
        hasher.update(&payload);
        let hash = hasher.finalize();

        Ok(Block::new(block_type, path_id, payload, *hash.as_bytes()))
    }

    fn calculate_array_chunk_size(&self, array_len: usize) -> usize {
        // Aim for chunks that will be close to max_chunk_size when serialized
        // This is a heuristic - in practice you'd want more sophisticated estimation
        std::cmp::max(1, array_len / 10)
    }

    fn split_string_safely(&self, s: &str, max_bytes: usize) -> Vec<String> {
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();
        let mut current_bytes = 0;

        for ch in s.chars() {
            let ch_bytes = ch.len_utf8();
            if current_bytes + ch_bytes > max_bytes && !current_chunk.is_empty() {
                chunks.push(current_chunk);
                current_chunk = String::new();
                current_bytes = 0;
            }
            current_chunk.push(ch);
            current_bytes += ch_bytes;
        }

        if !current_chunk.is_empty() {
            chunks.push(current_chunk);
        }

        if chunks.is_empty() {
            chunks.push(String::new());
        }

        chunks
    }
}

/// JSON path utilities
pub struct JsonPath;

impl JsonPath {
    /// Generate a path ID from a JSON path string
    pub fn path_to_id(path: &str) -> u64 {
        let mut hasher = Hasher::new();
        hasher.update(path.as_bytes());
        let hash = hasher.finalize();
        u64::from_le_bytes(hash.as_bytes()[0..8].try_into().unwrap())
    }

    /// Normalize a JSON path for consistent ID generation
    pub fn normalize_path(path: &str) -> String {
        // Remove array indices for grouping related array elements
        path.replace(|c: char| c.is_ascii_digit() || c == '[' || c == ']', "")
            .replace("..", ".")
            .trim_matches('.')
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_i_json_enforcement() {
        let value = json!({
            "z_key": "last",
            "a_key": "first",
            "number": 42.5,
            "array": [1, 2, 3]
        });

        let enforced = IJSONEnforcer::enforce(&value).unwrap();
        let serialized = IJSONEnforcer::to_canonical_bytes(&enforced).unwrap();
        let serialized_str = String::from_utf8(serialized).unwrap();

        // Keys should be sorted
        assert!(serialized_str.find("a_key").unwrap() < serialized_str.find("z_key").unwrap());
    }

    #[test]
    fn test_invalid_json_rejection() {
        let value = json!({
            "invalid_number": f64::INFINITY
        });

        assert!(IJSONEnforcer::enforce(&value).is_err());
    }

    #[test]
    fn test_json_chunk_encoder() {
        let mut encoder = JsonChunkEncoder::new(100); // Small chunk size for testing
        
        let value = json!({
            "small": "value",
            "number": 42
        });

        let blocks = encoder.encode_value(&value, "").unwrap();
        assert!(!blocks.is_empty());
        assert!(blocks[0].block_type() == Some(BlockType::JsonObject));
    }

    #[test]
    fn test_large_array_chunking() {
        let mut encoder = JsonChunkEncoder::new(50); // Very small for testing
        
        let large_array: Vec<Value> = (0..20).map(|i| json!(i)).collect();
        let value = Value::Array(large_array);

        let blocks = encoder.encode_value(&value, "test_array").unwrap();
        assert!(blocks.len() > 1); // Should be split into multiple blocks
    }

    #[test]
    fn test_string_splitting() {
        let encoder = JsonChunkEncoder::new(100);
        let long_string = "a".repeat(200);
        let chunks = encoder.split_string_safely(&long_string, 50);
        
        assert!(chunks.len() > 1);
        assert!(chunks.iter().all(|chunk| chunk.len() <= 50));
        assert_eq!(chunks.join(""), long_string);
    }

    #[test]
    fn test_path_id_generation() {
        let id1 = JsonPath::path_to_id("user.name");
        let id2 = JsonPath::path_to_id("user.email");
        let id3 = JsonPath::path_to_id("user.name"); // Same as id1

        assert_ne!(id1, id2);
        assert_eq!(id1, id3);
    }

    #[test]
    fn test_path_normalization() {
        assert_eq!(JsonPath::normalize_path("users[0].name"), "users.name");
        assert_eq!(JsonPath::normalize_path("data[123].items[456]"), "data.items");
        assert_eq!(JsonPath::normalize_path("simple"), "simple");
    }
}
