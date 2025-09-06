# CueDB Agreement Setup and First App Deployment

## Overview
CueDB is BPCI's advanced database system that provides cue-based rules for database operations, pipeline orchestration, and multicloud storage coordination. This guide covers setting up your first CueDB agreement and deploying your first application.

## Understanding CueDB Agreements

### Agreement Types
```rust
use bpci_enterprise::cuedb_agreement::{CueDbAgreementType, CueDbAgreementBuilder};

// Different agreement types for different use cases
pub enum CueDbAgreementType {
    // For individual developers
    Developer {
        max_storage_gb: u64,
        max_transactions_per_day: u64,
        allowed_operations: Vec<String>,
        data_retention_days: u32,
    },
    
    // For enterprise applications
    Enterprise {
        storage_quota: StorageQuota,
        pipeline_access: PipelineAccess,
        audit_requirements: AuditRequirements,
        compliance_level: ComplianceLevel,
    },
    
    // For financial applications
    Financial {
        compliance: FinancialCompliance,
        settlement_access: SettlementAccess,
        audit_trail: bool,
        encryption_required: bool,
    },
    
    // For research and analytics
    Research {
        sharing_policy: SharingPolicy,
        data_classification: DataClassification,
        retention_policy: RetentionPolicy,
        collaboration_enabled: bool,
    },
    
    // For multicloud deployments
    Multicloud {
        multicloud_access: MulticloudAccess,
        consensus_requirements: ConsensusRequirements,
        failover_policy: FailoverPolicy,
        replication_factor: u32,
    },
}
```

### Data Classification Levels
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum DataClassification {
    Public,        // Publicly accessible data
    Internal,      // Internal organization data
    Confidential,  // Sensitive business data
    Restricted,    // Highly sensitive data
    TopSecret,     // Maximum security data
}
```

## Creating Your First CueDB Agreement

### Step 1: Basic Developer Agreement
```rust
use bpci_enterprise::cuedb_agreement::{
    CueDbAgreementBuilder, CueDbAgreementType, DatabaseAction, 
    EnforcementLevel, PipelineAction, StorageAction
};
use chrono::{Utc, Duration};

async fn create_first_agreement() -> Result<CueDbAgreement> {
    let agreement = CueDbAgreementBuilder::new()
        .wallet_id("your-wallet-id-here")
        .agreement_type(CueDbAgreementType::Developer {
            max_storage_gb: 10,
            max_transactions_per_day: 1000,
            allowed_operations: vec![
                "read".to_string(),
                "write".to_string(),
                "query".to_string(),
                "index".to_string(),
            ],
            data_retention_days: 30,
        })
        
        // Add data volume monitoring
        .add_data_volume_rule(
            8, // Alert at 8GB (80% of limit)
            DatabaseAction::Alert,
            EnforcementLevel::Warning
        )
        
        // Add transaction rate monitoring
        .add_transaction_rate_rule(
            800, // Alert at 800 TPS (80% of daily limit)
            DatabaseAction::Throttle,
            EnforcementLevel::Strict
        )
        
        // Add automated backup
        .add_scheduled_pipeline_rule(
            "0 2 * * *".to_string(), // Daily at 2 AM
            PipelineAction::Backup,
            EnforcementLevel::Mandatory,
            ResourceLimits {
                max_cpu_cores: 2,
                max_memory_gb: 4,
                max_storage_gb: 5,
                max_network_mbps: 100,
                max_execution_time_minutes: 60,
            }
        )
        
        // Set expiration (30 days from now)
        .expires_at(Utc::now() + Duration::days(30))
        
        .build()?;
    
    println!("✅ CueDB Agreement created: {}", agreement.agreement_id);
    Ok(agreement)
}
```

### Step 2: Deploy Agreement to BPCI Network
```bash
# Save agreement to file
cat > my-first-agreement.json << EOF
{
  "wallet_id": "your-wallet-id-here",
  "agreement_type": {
    "Developer": {
      "max_storage_gb": 10,
      "max_transactions_per_day": 1000,
      "allowed_operations": ["read", "write", "query", "index"],
      "data_retention_days": 30
    }
  },
  "database_rules": [
    {
      "rule_id": "data-volume-alert",
      "trigger": {
        "DataVolumeThreshold": {
          "threshold_gb": 8
        }
      },
      "action": "Alert",
      "enforcement": "Warning"
    }
  ],
  "expires_at": "2024-10-05T00:00:00Z"
}
EOF

# Deploy agreement using BPCI CLI
bpci cuedb deploy-agreement --file=my-first-agreement.json

# Verify deployment
bpci cuedb list-agreements --wallet-id=your-wallet-id-here
```

## First Application Development

### Step 3: Create a Simple BPCI Application
```rust
// src/main.rs - Your first BPCI application
use bpci_enterprise::cuedb_agreement::CueDbAgreement;
use serde::{Serialize, Deserialize};
use tokio;

#[derive(Debug, Serialize, Deserialize)]
struct UserData {
    id: u64,
    name: String,
    email: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting My First BPCI App");
    
    // Initialize CueDB connection
    let cuedb_client = initialize_cuedb().await?;
    
    // Create sample data
    let user = UserData {
        id: 1,
        name: "Alice Smith".to_string(),
        email: "alice@example.com".to_string(),
        created_at: chrono::Utc::now(),
    };
    
    // Store data in CueDB
    let stored_id = cuedb_client.store("users", &user).await?;
    println!("✅ User stored with ID: {}", stored_id);
    
    // Query data from CueDB
    let retrieved_user: UserData = cuedb_client.get("users", stored_id).await?;
    println!("✅ Retrieved user: {:?}", retrieved_user);
    
    // List all users
    let all_users: Vec<UserData> = cuedb_client.list("users").await?;
    println!("✅ Total users: {}", all_users.len());
    
    println!("🎉 First BPCI app completed successfully!");
    Ok(())
}

async fn initialize_cuedb() -> Result<CueDbClient, Box<dyn std::error::Error>> {
    // Initialize CueDB client with your agreement
    let client = CueDbClient::new()
        .agreement_id("your-agreement-id-here")
        .wallet_id("your-wallet-id-here")
        .endpoint("https://cuedb.bpci-network.com")
        .connect()
        .await?;
    
    println!("✅ Connected to CueDB");
    Ok(client)
}
```

### Step 4: Application Configuration
```toml
# Cargo.toml
[package]
name = "my-first-bpci-app"
version = "0.1.0"
edition = "2021"

[dependencies]
bpci-enterprise = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1.0"
```

```toml
# bpci-app.toml - Application configuration
[app]
name = "my-first-bpci-app"
version = "0.1.0"
description = "My first BPCI application with CueDB"

[cuedb]
agreement_id = "your-agreement-id-here"
wallet_id = "your-wallet-id-here"
endpoint = "https://cuedb.bpci-network.com"
timeout_seconds = 30
retry_attempts = 3

[logging]
level = "info"
file = "/var/log/bpci/my-first-app.log"

[monitoring]
metrics_enabled = true
metrics_port = 9091
health_check_port = 8081
```

## Advanced CueDB Features

### Enterprise Agreement Example
```rust
async fn create_enterprise_agreement() -> Result<CueDbAgreement> {
    let agreement = CueDbAgreementBuilder::new()
        .wallet_id("enterprise-wallet-id")
        .agreement_type(CueDbAgreementType::Enterprise {
            storage_quota: StorageQuota {
                max_storage_gb: 1000,
                max_bandwidth_mbps: 1000,
                max_iops: 10000,
                storage_class: "premium".to_string(),
            },
            pipeline_access: PipelineAccess {
                max_concurrent_pipelines: 10,
                max_pipeline_duration_hours: 24,
                allowed_pipeline_types: vec![
                    "etl".to_string(),
                    "analytics".to_string(),
                    "backup".to_string(),
                    "replication".to_string(),
                ],
                resource_limits: ResourceLimits {
                    max_cpu_cores: 32,
                    max_memory_gb: 128,
                    max_storage_gb: 500,
                    max_network_mbps: 1000,
                    max_execution_time_minutes: 1440, // 24 hours
                },
            },
            audit_requirements: AuditRequirements {
                audit_enabled: true,
                audit_retention_days: 2555, // 7 years
                compliance_reporting: true,
                real_time_monitoring: true,
                alert_on_violations: true,
                audit_log_encryption: true,
                audit_log_immutability: true,
                third_party_audit_access: false,
                audit_trail_completeness: true,
                performance_audit: true,
            },
            compliance_level: ComplianceLevel::Enterprise,
        })
        
        // Add comprehensive monitoring
        .add_data_volume_rule(
            800, // Alert at 800GB (80% of limit)
            DatabaseAction::Alert,
            EnforcementLevel::Warning
        )
        
        // Add multicloud storage rule
        .add_multicloud_storage_rule(
            StorageTrigger::DataVolumeThreshold { threshold_gb: 500 },
            StorageAction::Replicate,
            EnforcementLevel::Mandatory,
            MulticloudAccess {
                enabled_providers: vec![
                    "aws".to_string(),
                    "gcp".to_string(),
                    "azure".to_string(),
                ],
                replication_strategy: "active-active".to_string(),
                consistency_level: "strong".to_string(),
                encryption_in_transit: true,
                encryption_at_rest: true,
                cross_region_replication: true,
                disaster_recovery_enabled: true,
                backup_frequency_hours: 6,
                geo_distribution: vec![
                    "us-east-1".to_string(),
                    "eu-west-1".to_string(),
                    "ap-southeast-1".to_string(),
                ],
            }
        )
        
        .expires_at(Utc::now() + Duration::days(365)) // 1 year
        .build()?;
    
    Ok(agreement)
}
```

### Financial Compliance Agreement
```rust
async fn create_financial_agreement() -> Result<CueDbAgreement> {
    let agreement = CueDbAgreementBuilder::new()
        .wallet_id("fintech-wallet-id")
        .agreement_type(CueDbAgreementType::Financial {
            compliance: FinancialCompliance {
                pci_dss_required: true,
                sox_compliance: true,
                gdpr_compliance: true,
                kyc_aml_enabled: true,
                transaction_monitoring: true,
                fraud_detection: true,
                regulatory_reporting: true,
                data_residency_requirements: vec![
                    "US".to_string(),
                    "EU".to_string(),
                ],
                encryption_standards: "FIPS-140-2".to_string(),
            },
            settlement_access: SettlementAccess {
                real_time_settlement: true,
                batch_settlement: true,
                cross_border_enabled: false, // Restricted for compliance
                settlement_currencies: vec![
                    "USD".to_string(),
                    "EUR".to_string(),
                ],
                max_settlement_amount: 1_000_000, // $1M limit
                settlement_window_hours: 24,
                reconciliation_required: true,
                settlement_audit_trail: true,
                regulatory_approval_required: true,
            },
            audit_trail: true,
            encryption_required: true,
        })
        .expires_at(Utc::now() + Duration::days(90)) // Quarterly renewal
        .build()?;
    
    Ok(agreement)
}
```

## Application Deployment and Testing

### Step 5: Build and Deploy Your Application
```bash
# Build the application
cargo build --release

# Create deployment package
mkdir -p deployment/bin deployment/config deployment/logs
cp target/release/my-first-bpci-app deployment/bin/
cp bpci-app.toml deployment/config/

# Create systemd service
cat > deployment/my-first-bpci-app.service << EOF
[Unit]
Description=My First BPCI Application
After=network.target bpci-node.service
Wants=network.target
Requires=bpci-node.service

[Service]
Type=simple
User=bpci
Group=bpci
WorkingDirectory=/opt/my-first-bpci-app
ExecStart=/opt/my-first-bpci-app/bin/my-first-bpci-app
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
SyslogIdentifier=my-first-bpci-app

# Environment
Environment=RUST_LOG=info
Environment=BPCI_CONFIG=/opt/my-first-bpci-app/config/bpci-app.toml

[Install]
WantedBy=multi-user.target
EOF

# Deploy to system
sudo cp -r deployment /opt/my-first-bpci-app
sudo cp deployment/my-first-bpci-app.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable my-first-bpci-app
sudo systemctl start my-first-bpci-app
```

### Step 6: Monitor and Test Your Application
```bash
# Check application status
sudo systemctl status my-first-bpci-app

# View application logs
sudo journalctl -u my-first-bpci-app -f

# Test application endpoints
curl http://localhost:8081/health
curl http://localhost:9091/metrics

# Test CueDB operations
bpci cuedb query --agreement-id=your-agreement-id --table=users
bpci cuedb stats --agreement-id=your-agreement-id
```

## Troubleshooting Common Issues

### Issue 1: Agreement Deployment Failed
```bash
# Check agreement validation
bpci cuedb validate-agreement --file=my-first-agreement.json

# Check wallet balance
bpci wallet balance --wallet-id=your-wallet-id-here

# Check network connectivity
bpci network test --endpoint=cuedb.bpci-network.com
```

### Issue 2: CueDB Connection Failed
```rust
// Add connection retry logic
use tokio::time::{sleep, Duration};

async fn connect_with_retry(max_retries: u32) -> Result<CueDbClient> {
    let mut retries = 0;
    
    loop {
        match CueDbClient::new()
            .agreement_id("your-agreement-id-here")
            .wallet_id("your-wallet-id-here")
            .endpoint("https://cuedb.bpci-network.com")
            .connect()
            .await
        {
            Ok(client) => return Ok(client),
            Err(e) if retries < max_retries => {
                retries += 1;
                eprintln!("Connection attempt {} failed: {}", retries, e);
                sleep(Duration::from_secs(2_u64.pow(retries))).await;
            }
            Err(e) => return Err(e),
        }
    }
}
```

### Issue 3: Performance Issues
```bash
# Monitor CueDB performance
bpci cuedb metrics --agreement-id=your-agreement-id --live

# Check resource usage
bpci cuedb resources --agreement-id=your-agreement-id

# Optimize queries
bpci cuedb explain --query="SELECT * FROM users WHERE created_at > '2024-01-01'"
```

## Next Steps

### 1. Explore Advanced Features
- **Multi-table operations**: Work with related data
- **Pipeline automation**: Set up data processing pipelines
- **Cross-chain integration**: Connect with partner chains
- **Real-time analytics**: Implement streaming analytics

### 2. Production Deployment
- **Load balancing**: Scale your application
- **Monitoring**: Set up comprehensive monitoring
- **Backup strategies**: Implement data backup
- **Security hardening**: Enhance security measures

### 3. Community Engagement
- **Share your app**: Contribute to the BPCI ecosystem
- **Get feedback**: Join developer discussions
- **Contribute**: Help improve BPCI and CueDB

---

**Previous**: [System Requirements and Setup](03-system-requirements-and-setup.md)  
**Next**: [First App Troubleshooting](05-first-app-troubleshooting.md)  
**Related**: [CueDB Advanced Features](../06-cue-and-usage/), [Application Development](../02-toolkit/)
