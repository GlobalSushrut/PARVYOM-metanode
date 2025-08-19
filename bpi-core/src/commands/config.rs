use anyhow::Result;
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tracing::{info, warn, error};

use crate::commands::ConfigCommands;

pub async fn handle(cmd: ConfigCommands, dry_run: bool) -> Result<()> {
    match cmd {
        ConfigCommands::Show => show_config().await,
        ConfigCommands::Set { key, value } => set_config(&key, &value, dry_run).await,
        ConfigCommands::Get { key } => get_config(&key).await,
        ConfigCommands::Reset => reset_config(dry_run).await,
        ConfigCommands::Validate => validate_config().await,
        ConfigCommands::Export { path } => export_config(&path, dry_run).await,
        ConfigCommands::Import { path } => import_config(&path, dry_run).await,
        ConfigCommands::Generate => generate_config(dry_run).await,
    }
}

async fn show_config() -> Result<()> {
    let config = load_config().await?;
    
    println!("Current Configuration:");
    println!("{}", serde_json::to_string_pretty(&config)?);
    
    Ok(())
}

async fn set_config(key: &str, value: &str, dry_run: bool) -> Result<()> {
    if dry_run {
        println!("DRY RUN: Would set {} = {}", key, value);
        return Ok(());
    }
    
    let mut config = load_config().await?;
    
    // Parse the key path (e.g., "network.p2p.port")
    let keys: Vec<&str> = key.split('.').collect();
    set_nested_value(&mut config, &keys, value)?;
    
    save_config(&config).await?;
    println!("✅ Configuration updated: {} = {}", key, value);
    
    Ok(())
}

async fn get_config(key: &str) -> Result<()> {
    let config = load_config().await?;
    
    // Parse the key path
    let keys: Vec<&str> = key.split('.').collect();
    let value = get_nested_value(&config, &keys)?;
    
    println!("{}: {}", key, value);
    
    Ok(())
}

async fn reset_config(dry_run: bool) -> Result<()> {
    if dry_run {
        println!("DRY RUN: Would reset configuration to defaults");
        return Ok(());
    }
    
    let default_config = generate_default_config();
    save_config(&default_config).await?;
    
    println!("✅ Configuration reset to defaults");
    
    Ok(())
}

async fn validate_config() -> Result<()> {
    let config = load_config().await?;
    
    // Validate required fields
    let required_fields = [
        "network.p2p.port",
        "network.rpc.port",
        "consensus.algorithm",
        "storage.data_dir",
    ];
    
    let mut errors = Vec::new();
    
    for field in &required_fields {
        let keys: Vec<&str> = field.split('.').collect();
        if get_nested_value(&config, &keys).is_err() {
            errors.push(format!("Missing required field: {}", field));
        }
    }
    
    // Validate port ranges
    if let Ok(p2p_port) = get_nested_value(&config, &["network", "p2p", "port"]) {
        if let Ok(port) = p2p_port.parse::<u16>() {
            if port < 1024 {
                errors.push("P2P port should be >= 1024".to_string());
            }
        } else {
            errors.push("P2P port must be a valid number".to_string());
        }
    }
    
    if errors.is_empty() {
        println!("✅ Configuration is valid");
    } else {
        println!("❌ Configuration validation failed:");
        for error in errors {
            println!("  - {}", error);
        }
        return Err(anyhow::anyhow!("Configuration validation failed"));
    }
    
    Ok(())
}

async fn export_config(path: &str, dry_run: bool) -> Result<()> {
    if dry_run {
        println!("DRY RUN: Would export configuration to {}", path);
        return Ok(());
    }
    
    let config = load_config().await?;
    let config_str = serde_json::to_string_pretty(&config)?;
    
    fs::write(path, config_str)?;
    println!("✅ Configuration exported to {}", path);
    
    Ok(())
}

async fn import_config(path: &str, dry_run: bool) -> Result<()> {
    if dry_run {
        println!("DRY RUN: Would import configuration from {}", path);
        return Ok(());
    }
    
    if !Path::new(path).exists() {
        return Err(anyhow::anyhow!("Configuration file not found: {}", path));
    }
    
    let config_str = fs::read_to_string(path)?;
    let config: serde_json::Value = serde_json::from_str(&config_str)?;
    
    // Validate imported config
    validate_imported_config(&config)?;
    
    save_config(&config).await?;
    println!("✅ Configuration imported from {}", path);
    
    Ok(())
}

async fn generate_config(dry_run: bool) -> Result<()> {
    if dry_run {
        println!("DRY RUN: Would generate sample configuration");
        return Ok(());
    }
    
    let sample_config = generate_sample_config();
    
    println!("Sample Configuration:");
    println!("{}", serde_json::to_string_pretty(&sample_config)?);
    
    Ok(())
}

// Helper functions

async fn load_config() -> Result<serde_json::Value> {
    let config_path = get_config_path();
    
    if Path::new(&config_path).exists() {
        let config_str = fs::read_to_string(&config_path)?;
        let config: serde_json::Value = serde_json::from_str(&config_str)?;
        Ok(config)
    } else {
        Ok(generate_default_config())
    }
}

async fn save_config(config: &serde_json::Value) -> Result<()> {
    let config_path = get_config_path();
    let config_str = serde_json::to_string_pretty(config)?;
    
    // Ensure config directory exists
    if let Some(parent) = Path::new(&config_path).parent() {
        fs::create_dir_all(parent)?;
    }
    
    fs::write(&config_path, config_str)?;
    Ok(())
}

fn get_config_path() -> String {
    std::env::var("METANODE_CONFIG")
        .unwrap_or_else(|_| "/etc/metanode/config.json".to_string())
}

fn set_nested_value(config: &mut serde_json::Value, keys: &[&str], value: &str) -> Result<()> {
    if keys.is_empty() {
        return Err(anyhow::anyhow!("Empty key path"));
    }
    
    let mut current = config;
    
    // Navigate to the parent of the target key
    for &key in &keys[..keys.len() - 1] {
        if !current.is_object() {
            *current = json!({});
        }
        
        if !current.as_object().unwrap().contains_key(key) {
            current.as_object_mut().unwrap().insert(key.to_string(), json!({}));
        }
        
        current = current.get_mut(key).unwrap();
    }
    
    // Set the final value
    let final_key = keys[keys.len() - 1];
    
    if !current.is_object() {
        *current = json!({});
    }
    
    // Try to parse value as different types
    let parsed_value = if let Ok(bool_val) = value.parse::<bool>() {
        json!(bool_val)
    } else if let Ok(int_val) = value.parse::<i64>() {
        json!(int_val)
    } else if let Ok(float_val) = value.parse::<f64>() {
        json!(float_val)
    } else {
        json!(value)
    };
    
    current.as_object_mut().unwrap().insert(final_key.to_string(), parsed_value);
    
    Ok(())
}

fn get_nested_value(config: &serde_json::Value, keys: &[&str]) -> Result<String> {
    let mut current = config;
    
    for &key in keys {
        current = current.get(key)
            .ok_or_else(|| anyhow::anyhow!("Key not found: {}", key))?;
    }
    
    Ok(match current {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        _ => current.to_string(),
    })
}

fn generate_default_config() -> serde_json::Value {
    json!({
        "network": {
            "p2p": {
                "port": 30303,
                "max_peers": 50,
                "discovery": true
            },
            "rpc": {
                "port": 8545,
                "enabled": true,
                "cors": ["*"]
            },
            "api": {
                "port": 8080,
                "enabled": true
            }
        },
        "consensus": {
            "algorithm": "ibft",
            "block_time": 5,
            "validator_set_size": 21
        },
        "storage": {
            "data_dir": "/var/lib/metanode",
            "cache_size": "1GB",
            "sync_mode": "fast"
        },
        "security": {
            "quantum_resistant": true,
            "ai_security": true,
            "zero_knowledge": true
        },
        "logging": {
            "level": "info",
            "file": "/var/log/metanode/metanode.log",
            "max_size": "100MB",
            "max_files": 10
        }
    })
}

fn generate_sample_config() -> serde_json::Value {
    json!({
        "network": {
            "p2p": {
                "port": 30303,
                "max_peers": 50,
                "discovery": true,
                "bootstrap_nodes": [
                    "/ip4/127.0.0.1/tcp/30303/p2p/QmBootstrapNode1",
                    "/ip4/127.0.0.1/tcp/30304/p2p/QmBootstrapNode2"
                ]
            },
            "rpc": {
                "port": 8545,
                "enabled": true,
                "cors": ["http://localhost:3000"],
                "rate_limit": 1000
            },
            "api": {
                "port": 8080,
                "enabled": true,
                "auth_required": true
            }
        },
        "consensus": {
            "algorithm": "ibft",
            "block_time": 5,
            "validator_set_size": 21,
            "finality_threshold": 67
        },
        "storage": {
            "data_dir": "/var/lib/metanode",
            "cache_size": "2GB",
            "sync_mode": "full",
            "pruning": {
                "enabled": true,
                "keep_blocks": 1000
            }
        },
        "security": {
            "quantum_resistant": true,
            "ai_security": {
                "enabled": true,
                "anomaly_detection": true,
                "threat_response": "auto"
            },
            "zero_knowledge": {
                "enabled": true,
                "privacy_level": "high"
            },
            "biso": {
                "enabled": true,
                "policy_enforcement": "strict"
            }
        },
        "enterprise": {
            "enabled": false,
            "license_key": "",
            "features": {
                "advanced_monitoring": false,
                "enterprise_support": false,
                "sla_guarantees": false
            }
        },
        "docklock": {
            "enabled": true,
            "deterministic_execution": true,
            "witness_recording": true,
            "policy_engine": "strict"
        },
        "enc": {
            "enabled": false,
            "cluster_mode": "standalone",
            "orchestration": "kubernetes"
        },
        "banking": {
            "enabled": false,
            "settlement": {
                "cross_chain": true,
                "instant_finality": true
            },
            "compliance": {
                "kyc_required": true,
                "aml_checks": true
            }
        },
        "governance": {
            "voting_power": "stake_weighted",
            "proposal_threshold": "1%",
            "voting_period": "7d"
        },
        "economics": {
            "base_fee": "0.001",
            "fee_adjustment": "dynamic",
            "inflation_rate": "2%"
        },
        "monitoring": {
            "metrics": {
                "enabled": true,
                "port": 9090,
                "path": "/metrics"
            },
            "alerts": {
                "enabled": true,
                "webhook_url": ""
            }
        },
        "logging": {
            "level": "info",
            "format": "json",
            "file": "/var/log/metanode/metanode.log",
            "max_size": "100MB",
            "max_files": 10,
            "components": {
                "consensus": "debug",
                "networking": "info",
                "storage": "warn"
            }
        }
    })
}

fn validate_imported_config(config: &serde_json::Value) -> Result<()> {
    // Basic validation of imported configuration
    if !config.is_object() {
        return Err(anyhow::anyhow!("Configuration must be a JSON object"));
    }
    
    // Check for required top-level sections
    let required_sections = ["network", "consensus", "storage"];
    
    for section in &required_sections {
        if !config.get(section).is_some() {
            return Err(anyhow::anyhow!("Missing required section: {}", section));
        }
    }
    
    Ok(())
}
