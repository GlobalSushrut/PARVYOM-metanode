# CUE Configuration Patterns and Best Practices

## Introduction

This guide covers the essential CUE configuration patterns used throughout the Pravyom ecosystem. It provides practical examples, best practices, and advanced patterns for creating maintainable, type-safe, and composable configurations.

## Core Configuration Patterns

### 1. Schema Definition Patterns

**Basic Schema Pattern:**
```cue
package schemas

// Define reusable schemas with validation
#ServiceConfig: {
    name: string & =~"^[a-z0-9-]+$"
    version: string & =~"^v[0-9]+\\.[0-9]+\\.[0-9]+$"
    replicas: int & >=1 & <=100
    
    resources: {
        cpu: string & =~"^[0-9]+m?$"
        memory: string & =~"^[0-9]+[GMK]i?$"
        storage?: string & =~"^[0-9]+[GMK]i?$"
    }
    
    // Optional fields with defaults
    enabled?: bool | *true
    debug?: bool | *false
}
```

**Advanced Schema with Conditional Logic:**
```cue
package advanced_schemas

#SecurityConfig: {
    level: "standard" | "enterprise" | "military_grade"
    
    // Conditional requirements based on security level
    if level == "military_grade" {
        quantum_crypto: true
        determinism_cage: true
        witness_recording: true
        hardware_security: true
    }
    
    if level == "enterprise" {
        encryption_at_rest: true
        audit_logging: true
        access_control: "rbac"
    }
    
    // Common security features
    tls_encryption: bool | *true
    certificate_validation: bool | *true
}
```

### 2. Composition Patterns

**Mixin Pattern:**
```cue
package mixins

// Base configuration mixin
#BaseMixin: {
    metadata: {
        created_by: string
        created_at: string
        version: string
    }
    
    labels: [string]: string
    annotations: [string]: string
}

// Security mixin
#SecurityMixin: {
    security: {
        encryption: bool | *true
        audit: bool | *true
        access_control: string | *"rbac"
    }
}

// Monitoring mixin
#MonitoringMixin: {
    monitoring: {
        metrics: bool | *true
        logging: bool | *true
        tracing: bool | *false
        
        alerts: [...{
            name: string
            condition: string
            severity: "low" | "medium" | "high" | "critical"
        }]
    }
}

// Composed service configuration
#ServiceConfig: #BaseMixin & #SecurityMixin & #MonitoringMixin & {
    name: string
    image: string
    replicas: int
}
```

**Inheritance Pattern:**
```cue
package inheritance

// Base service definition
#BaseService: {
    apiVersion: "bpi.dev/v1"
    kind: "Service"
    
    metadata: {
        name: string
        namespace: string | *"default"
    }
    
    spec: {
        replicas: int & >=1 | *1
        
        template: {
            metadata: labels: [string]: string
            
            spec: {
                containers: [...{
                    name: string
                    image: string
                    ports?: [...{
                        containerPort: int
                        protocol: "TCP" | "UDP" | *"TCP"
                    }]
                }]
            }
        }
    }
}

// Web service inherits from base service
#WebService: #BaseService & {
    kind: "WebService"
    
    spec: {
        template: spec: {
            containers: [...{
                // Web-specific container configuration
                ports: [...{
                    containerPort: 80 | 443 | 8080 | 8443
                }]
                
                env?: [...{
                    name: string
                    value: string
                }]
                
                livenessProbe?: {
                    httpGet: {
                        path: string | *"/health"
                        port: int
                    }
                    initialDelaySeconds: int | *30
                    periodSeconds: int | *10
                }
            }]
        }
        
        // Web service specific fields
        ingress?: {
            enabled: bool | *false
            host: string
            tls: bool | *true
        }
    }
}
```

### 3. Validation Patterns

**Complex Validation Rules:**
```cue
package validation

#WalletConfig: {
    address: string & =~"^bpi1[a-z0-9]{38}$"
    type: "individual" | "enterprise" | "government" | "bank"
    
    // Validation based on wallet type
    if type == "government" {
        jurisdiction: string & len(2)  // ISO country code
        authority_level: "federal" | "state" | "local"
        compliance_frameworks: [...("GDPR" | "CCPA" | "HIPAA")]
    }
    
    if type == "bank" {
        banking_license: string & len(>0)
        regulatory_body: string
        settlement_capabilities: [...("ACH" | "WIRE" | "SEPA" | "RTP")]
    }
    
    if type == "enterprise" {
        business_registration: string
        compliance_level: "basic" | "enhanced" | "premium"
    }
    
    // Common fields
    created_at: string
    public_key: string & len(64)  // Ed25519 public key hex
    
    // Ensure wallet address matches public key (simplified check)
    address: =~"^bpi1" + string
}
```

**Cross-Field Validation:**
```cue
package cross_validation

#DeploymentConfig: {
    name: string
    environment: "development" | "staging" | "production"
    
    resources: {
        cpu: string
        memory: string
        replicas: int
    }
    
    security: {
        level: "standard" | "enterprise" | "military_grade"
        encryption: bool
    }
    
    // Cross-field validation rules
    if environment == "production" {
        // Production requires higher security
        security.level: "enterprise" | "military_grade"
        security.encryption: true
        resources.replicas: >=3
    }
    
    if security.level == "military_grade" {
        // Military grade requires specific resource minimums
        resources.cpu: =~"^([2-9][0-9]{3,}|[1-9][0-9]{4,})m$"  // >= 2000m
        resources.memory: =~"^([4-9]|[1-9][0-9]+)Gi$"  // >= 4Gi
    }
    
    // Ensure resource consistency
    if resources.replicas > 10 {
        resources.cpu: =~"^[0-9]{4,}m$"  // High replica count needs more CPU
    }
}
```

## BPI-Specific Configuration Patterns

### 1. SmartContract Configuration

**SmartContract Definition Pattern:**
```cue
package smartcontract

#SmartContract: {
    apiVersion: "smartcontract.bpi.dev/v1"
    kind: "SmartContract"
    
    metadata: {
        name: string & =~"^[a-z0-9-]+$"
        namespace: string | *"default"
        
        labels: {
            "contract.type": "payment" | "governance" | "compliance" | "utility"
            "security.level": "standard" | "enterprise" | "military_grade"
            "bpi.integration": "true"
        }
    }
    
    spec: {
        // Contract execution configuration
        execution: {
            vm_type: "cue_native" | "wasm" | "evm_compatible"
            gas_limit: int & >0 & <=10000000
            deterministic: bool | *true
        }
        
        // Payment integration
        payments?: {
            fiat_integration: bool | *false
            
            if fiat_integration {
                gateways: [...("stripe" | "paypal" | "square" | "bank_transfer")]
                currencies: [...string]
                settlement_delay: string | *"24h"
            }
            
            crypto_integration: bool | *true
            supported_tokens: [...string]
        }
        
        // Security configuration
        security: {
            access_control: "public" | "permissioned" | "private"
            audit_required: bool | *true
            
            if access_control == "permissioned" {
                allowed_wallets: [...string]  // Wallet addresses
            }
            
            if access_control == "private" {
                owner_wallet: string & =~"^bpi1[a-z0-9]{38}$"
            }
        }
        
        // BPI integration
        bpi_integration: {
            ledger_anchoring: bool | *true
            oracle_feeds: [...string]
            cross_chain: bool | *false
            
            if cross_chain {
                supported_chains: [...("ethereum" | "bitcoin" | "polkadot")]
            }
        }
    }
}
```

**Payment Contract Example:**
```cue
package payment_contract

import "smartcontract"

payment_processor: smartcontract.#SmartContract & {
    metadata: {
        name: "payment-processor"
        namespace: "finance"
        
        labels: {
            "contract.type": "payment"
            "security.level": "enterprise"
            "bpi.integration": "true"
        }
    }
    
    spec: {
        execution: {
            vm_type: "cue_native"
            gas_limit: 1000000
            deterministic: true
        }
        
        payments: {
            fiat_integration: true
            gateways: ["stripe", "paypal", "bank_transfer"]
            currencies: ["USD", "EUR", "GBP", "CAD"]
            settlement_delay: "24h"
            
            crypto_integration: true
            supported_tokens: ["BPI", "ETH", "BTC"]
        }
        
        security: {
            access_control: "permissioned"
            allowed_wallets: [
                "bpi1merchant001abc123def456789012345678",
                "bpi1processor001def789abc123456789012"
            ]
            audit_required: true
        }
        
        bpi_integration: {
            ledger_anchoring: true
            oracle_feeds: ["price_oracle", "compliance_oracle"]
            cross_chain: true
            supported_chains: ["ethereum", "bitcoin"]
        }
    }
}
```

### 2. BISO Policy Configuration

**BISO Policy Pattern:**
```cue
package biso_policy

#BisoPolicy: {
    apiVersion: "biso.bpi.dev/v1"
    kind: "BisoPolicy"
    
    metadata: {
        name: string
        version: string
        jurisdiction?: string
    }
    
    spec: {
        // Policy scope
        scope: {
            services?: [...string]
            wallets?: [...string]
            jurisdictions?: [...string]
            data_classifications?: [...string]
        }
        
        // Policy rules
        rules: [...{
            name: string
            type: "security" | "compliance" | "resource" | "audit"
            enforcement: "advisory" | "warning" | "blocking" | "escalation"
            
            conditions: [...string]
            actions: [...string]
            
            // Rule-specific configuration
            if type == "compliance" {
                frameworks: [...("GDPR" | "HIPAA" | "PCI_DSS" | "SOX")]
                violation_severity: "low" | "medium" | "high" | "critical"
            }
            
            if type == "security" {
                threat_level: "low" | "medium" | "high" | "critical"
                automatic_response: bool | *true
            }
        }]
        
        // Monitoring and alerting
        monitoring: {
            metrics: [...string]
            real_time: bool | *true
            
            alerts: [...{
                condition: string
                severity: "low" | "medium" | "high" | "critical"
                notification: [...string]  // Email addresses or webhook URLs
            }]
        }
        
        // Reporting configuration
        reporting: {
            frequency: "hourly" | "daily" | "weekly" | "monthly"
            format: "json" | "pdf" | "csv"
            recipients: [...string]
            include_recommendations: bool | *true
        }
    }
}
```

**GDPR Compliance Policy Example:**
```cue
package gdpr_policy

import "biso_policy"

gdpr_compliance: biso_policy.#BisoPolicy & {
    metadata: {
        name: "gdpr-compliance"
        version: "v1.2.0"
        jurisdiction: "EU"
    }
    
    spec: {
        scope: {
            services: ["user-service", "payment-service", "analytics-service"]
            data_classifications: ["personal_data", "sensitive_personal_data"]
            jurisdictions: ["EU", "EEA"]
        }
        
        rules: [
            {
                name: "data-encryption-required"
                type: "security"
                enforcement: "blocking"
                conditions: [
                    "data.classification in ['personal_data', 'sensitive_personal_data']",
                    "transport.encryption == false"
                ]
                actions: [
                    "reject_request",
                    "log_violation",
                    "notify_dpo"
                ]
                threat_level: "high"
                automatic_response: true
            },
            {
                name: "consent-verification"
                type: "compliance"
                enforcement: "blocking"
                conditions: [
                    "data.type == 'personal_data'",
                    "consent.verified == false"
                ]
                actions: [
                    "request_consent",
                    "log_consent_request",
                    "notify_user"
                ]
                frameworks: ["GDPR"]
                violation_severity: "high"
            },
            {
                name: "data-retention-limits"
                type: "compliance"
                enforcement: "warning"
                conditions: [
                    "data.age > policy.retention_period",
                    "data.classification == 'personal_data'"
                ]
                actions: [
                    "schedule_deletion",
                    "notify_data_controller",
                    "log_retention_violation"
                ]
                frameworks: ["GDPR"]
                violation_severity: "medium"
            }
        ]
        
        monitoring: {
            metrics: [
                "gdpr_violations_per_hour",
                "consent_rate",
                "data_deletion_requests",
                "cross_border_transfers"
            ]
            real_time: true
            
            alerts: [
                {
                    condition: "gdpr_violations_per_hour > 5"
                    severity: "critical"
                    notification: ["dpo@company.eu", "legal@company.eu"]
                },
                {
                    condition: "consent_rate < 0.95"
                    severity: "medium"
                    notification: ["privacy@company.eu"]
                }
            ]
        }
        
        reporting: {
            frequency: "daily"
            format: "pdf"
            recipients: ["dpo@company.eu", "compliance@company.eu"]
            include_recommendations: true
        }
    }
}
```

### 3. Infrastructure Configuration Patterns

**DockLock Container Configuration:**
```cue
package docklock

#DeterminismCage: {
    apiVersion: "docklock.bpi.dev/v1"
    kind: "DeterminismCage"
    
    metadata: {
        name: string
        namespace: string | *"default"
    }
    
    spec: {
        // Container specification
        image: string & =~"^[a-zA-Z0-9._/-]+:[a-zA-Z0-9._-]+$"
        command?: [...string]
        args?: [...string]
        
        // Determinism configuration
        determinism: {
            syscall_filtering: bool | *true
            witness_recording: bool | *true
            rng_seeding: bool | *true
            
            if rng_seeding {
                rng_seed?: string & len(64)  // 32 bytes hex
            }
        }
        
        // Security policies
        security: {
            seccomp_profile: "strict" | "moderate" | "permissive" | *"strict"
            
            allowed_syscalls?: [...string]
            blocked_syscalls?: [...string]
            
            // Default blocked syscalls for determinism
            blocked_syscalls: *[
                "gettimeofday", "clock_gettime", "rdtsc", 
                "getrandom", "random", "time"
            ] | [...string]
        }
        
        // Resource limits
        resources: {
            limits: {
                cpu: string & =~"^[0-9]+m?$"
                memory: string & =~"^[0-9]+[GMK]i?$"
                execution_time?: string | *"1h"
            }
            requests: {
                cpu: string & =~"^[0-9]+m?$"
                memory: string & =~"^[0-9]+[GMK]i?$"
            }
        }
        
        // Witness configuration
        witness?: {
            output_path: string | *"/var/log/docklock/witness"
            compression: bool | *true
            merkle_verification: bool | *true
        }
        
        // Network isolation
        network?: {
            isolation: bool | *true
            allowed_endpoints?: [...string]
        }
        
        // Filesystem isolation
        filesystem?: {
            isolation: bool | *true
            read_only_paths?: [...string]
            writable_paths?: [...string]
        }
    }
}
```

**ENC Cluster Configuration:**
```cue
package enc_cluster

#EncCluster: {
    apiVersion: "enc.bpi.dev/v1"
    kind: "EncCluster"
    
    metadata: {
        name: string
        region?: string
    }
    
    spec: {
        // Cluster configuration
        cluster: {
            max_nodes: int & >=1 & <=1000 | *10
            max_replicas_per_node: int & >=1 & <=100 | *10
            auto_scaling_enabled: bool | *true
        }
        
        // Load balancing
        load_balancer: {
            algorithm: "RoundRobin" | "ConsistentHashing" | "LeastConnections" | *"ConsistentHashing"
            
            health_check: {
                interval: string | *"30s"
                timeout: string | *"10s"
                retries: int & >=1 & <=10 | *3
            }
        }
        
        // Security configuration
        security: {
            level: "Standard" | "Enterprise" | "MilitaryGrade" | *"Enterprise"
            
            if level == "MilitaryGrade" {
                quantum_crypto_enabled: true
                determinism_cage_required: true
                witness_recording_required: true
            }
            
            audit_to_bpi_ledger: bool | *true
        }
        
        // Domain protocols
        domain_protocols: {
            httpcg_enabled: bool | *true
            rootzk_enabled: bool | *false
            cache_ttl: string | *"300s"
            cache_size: int | *10000
        }
        
        // BPI integration
        bpi_integration: {
            ledger_audit: bool | *true
            wallet_authentication: bool | *true
            shadow_registry: bool | *true
            
            audit_config: {
                batch_size: int & >=1 & <=1000 | *100
                batch_interval: string | *"10s"
            }
        }
    }
}
```

## Advanced Configuration Patterns

### 1. Multi-Environment Configuration

**Environment-Specific Configuration:**
```cue
package environments

// Base configuration
#BaseConfig: {
    name: string
    image: string
    
    resources: {
        cpu: string
        memory: string
    }
    
    security: {
        level: string
        encryption: bool
    }
}

// Environment-specific configurations
environments: {
    development: #BaseConfig & {
        name: "myapp-dev"
        image: "myapp:dev"
        
        resources: {
            cpu: "500m"
            memory: "1Gi"
        }
        
        security: {
            level: "standard"
            encryption: false  // Disabled for easier debugging
        }
        
        debug: true
        log_level: "debug"
    }
    
    staging: #BaseConfig & {
        name: "myapp-staging"
        image: "myapp:staging"
        
        resources: {
            cpu: "1000m"
            memory: "2Gi"
        }
        
        security: {
            level: "enterprise"
            encryption: true
        }
        
        debug: false
        log_level: "info"
        replicas: 2
    }
    
    production: #BaseConfig & {
        name: "myapp-prod"
        image: "myapp:v1.2.3"
        
        resources: {
            cpu: "2000m"
            memory: "4Gi"
        }
        
        security: {
            level: "military_grade"
            encryption: true
        }
        
        debug: false
        log_level: "warn"
        replicas: 5
        
        // Production-specific features
        monitoring: {
            metrics: true
            alerting: true
            tracing: true
        }
        
        backup: {
            enabled: true
            frequency: "daily"
            retention: "30d"
        }
    }
}
```

### 2. Dynamic Configuration Generation

**Template-Based Generation:**
```cue
package templates

// Service template
#ServiceTemplate: {
    _name: string
    _environment: string
    _replicas: int
    
    apiVersion: "apps/v1"
    kind: "Deployment"
    
    metadata: {
        name: "\(_name)-\(_environment)"
        namespace: _environment
        
        labels: {
            app: _name
            environment: _environment
            version: "v1.0.0"
        }
    }
    
    spec: {
        replicas: _replicas
        
        selector: matchLabels: {
            app: _name
            environment: _environment
        }
        
        template: {
            metadata: labels: {
                app: _name
                environment: _environment
            }
            
            spec: containers: [{
                name: _name
                image: "\(_name):\(_environment)"
                
                resources: {
                    if _environment == "production" {
                        limits: {
                            cpu: "2000m"
                            memory: "4Gi"
                        }
                        requests: {
                            cpu: "1000m"
                            memory: "2Gi"
                        }
                    }
                    
                    if _environment == "staging" {
                        limits: {
                            cpu: "1000m"
                            memory: "2Gi"
                        }
                        requests: {
                            cpu: "500m"
                            memory: "1Gi"
                        }
                    }
                    
                    if _environment == "development" {
                        limits: {
                            cpu: "500m"
                            memory: "1Gi"
                        }
                        requests: {
                            cpu: "250m"
                            memory: "512Mi"
                        }
                    }
                }
            }]
        }
    }
}

// Generate services for multiple environments
services: {
    for service in ["web-api", "auth-service", "payment-service"] {
        for env in ["development", "staging", "production"] {
            "\(service)-\(env)": #ServiceTemplate & {
                _name: service
                _environment: env
                _replicas: {
                    if env == "production" { 5 }
                    if env == "staging" { 2 }
                    if env == "development" { 1 }
                }
            }
        }
    }
}
```

### 3. Configuration Validation and Testing

**Validation Rules:**
```cue
package validation_rules

// Comprehensive validation for production deployments
#ProductionValidation: {
    config: {...}
    
    // Ensure production requirements
    config.environment: "production"
    config.replicas: >=3
    config.security.level: "enterprise" | "military_grade"
    config.security.encryption: true
    config.monitoring.enabled: true
    config.backup.enabled: true
    
    // Resource requirements
    config.resources.limits.cpu: =~"^([1-9][0-9]{3,}|[2-9][0-9]{2})m$"  // >= 200m
    config.resources.limits.memory: =~"^[1-9][0-9]*[GM]i$"  // >= 1Gi
    
    // Security requirements
    if config.security.level == "military_grade" {
        config.determinism_cage: true
        config.witness_recording: true
        config.quantum_crypto: true
    }
}

// Test configuration
test_config: #ProductionValidation & {
    config: {
        environment: "production"
        replicas: 5
        
        security: {
            level: "military_grade"
            encryption: true
        }
        
        monitoring: enabled: true
        backup: enabled: true
        
        resources: {
            limits: {
                cpu: "2000m"
                memory: "4Gi"
            }
        }
        
        determinism_cage: true
        witness_recording: true
        quantum_crypto: true
    }
}
```

## Best Practices

### 1. Schema Design Best Practices

- **Use Descriptive Names**: Choose clear, descriptive names for schemas and fields
- **Provide Defaults**: Use sensible defaults for optional fields
- **Add Validation**: Include comprehensive validation rules
- **Document Constraints**: Use comments to explain complex validation logic
- **Version Schemas**: Include version information in schema definitions

### 2. Composition Best Practices

- **Favor Composition**: Use composition over inheritance for flexibility
- **Create Reusable Mixins**: Design mixins for common functionality
- **Avoid Deep Nesting**: Keep configuration hierarchies shallow
- **Use Conditional Logic**: Leverage CUE's conditional capabilities
- **Validate Compositions**: Ensure composed configurations are valid

### 3. Performance Best Practices

- **Lazy Evaluation**: Structure configurations for lazy evaluation
- **Cache Configurations**: Cache compiled configurations for reuse
- **Minimize Dependencies**: Reduce cross-configuration dependencies
- **Optimize Validation**: Use efficient validation patterns
- **Profile Performance**: Monitor configuration compilation times

### 4. Security Best Practices

- **Validate Inputs**: Always validate external configuration inputs
- **Use Type Safety**: Leverage CUE's type system for security
- **Audit Changes**: Maintain audit trails for configuration changes
- **Encrypt Secrets**: Never store secrets in plain text configurations
- **Access Control**: Implement proper access controls for configurations

## Conclusion

CUE configuration patterns in the Pravyom ecosystem provide a powerful foundation for type-safe, composable, and maintainable infrastructure management. By following these patterns and best practices, developers can create robust configurations that scale from simple services to complex multi-environment deployments while maintaining security, compliance, and operational excellence.
