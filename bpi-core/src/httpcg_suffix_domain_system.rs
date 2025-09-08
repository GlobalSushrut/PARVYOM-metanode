use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use tracing::{info, warn, error};

/// httpcg Domain Suffix System
/// Supports domain names like prav@global, prav@in, prav@gov, etc.
/// Similar to traditional TLDs but with enhanced security and routing

#[derive(Debug, Clone)]
pub struct HttpcgSuffixDomainSystem {
    /// Domain suffix registry
    pub suffix_registry: Arc<RwLock<HashMap<String, DomainSuffix>>>,
    /// Active domain mappings
    pub domain_mappings: Arc<RwLock<HashMap<String, DomainMapping>>>,
    /// Routing configuration
    pub routing_config: HttpcgRoutingConfig,
}

/// Domain suffix types (like TLDs but enhanced)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainSuffix {
    pub suffix: String,           // @global, @in, @us, @gov, etc.
    pub suffix_type: SuffixType,
    pub security_level: SecurityLevel,
    pub routing_plane: String,    // app, secure, gov, etc.
    pub authority: String,        // Who manages this suffix
    pub enabled: bool,
}

/// Types of domain suffixes
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum SuffixType {
    Global,        // @global (like .com)
    Country(String), // @in, @us, @uk (country codes)
    Government,    // @gov
    International, // @int
    Corporate,     // @corp
    Educational,   // @edu
    Military,      // @mil
    Dark,          // @dark (private networks)
}

/// Security levels for different suffix types
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum SecurityLevel {
    Public,        // Standard security
    Enhanced,      // Higher security
    Classified,    // Government/military
    Quantum,       // Quantum-safe only
}

/// Domain mapping for httpcg routing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainMapping {
    pub full_domain: String,      // prav@global
    pub base_name: String,        // prav
    pub suffix: String,           // @global
    pub httpcg_url: String,       // httpcg://app/prav.global/
    pub routing_plane: String,    // app, secure, gov, etc.
    pub security_level: SecurityLevel,
    pub active: bool,
}

/// httpcg routing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpcgRoutingConfig {
    pub default_plane: String,
    pub plane_mappings: HashMap<SuffixType, String>,
    pub security_policies: HashMap<SecurityLevel, SecurityPolicy>,
}

/// Security policy for different levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    pub requires_auth: bool,
    pub quantum_safe_only: bool,
    pub audit_required: bool,
    pub encryption_level: String,
}

impl HttpcgSuffixDomainSystem {
    /// Create new httpcg suffix domain system
    pub async fn new() -> Result<Self> {
        let mut system = Self {
            suffix_registry: Arc::new(RwLock::new(HashMap::new())),
            domain_mappings: Arc::new(RwLock::new(HashMap::new())),
            routing_config: Self::default_routing_config(),
        };

        // Initialize default suffixes
        system.initialize_default_suffixes().await?;
        
        info!("🌐 httpcg Suffix Domain System initialized");
        Ok(system)
    }

    /// Initialize default domain suffixes
    async fn initialize_default_suffixes(&self) -> Result<()> {
        let mut registry = self.suffix_registry.write().await;

        // Global suffixes (like .com)
        registry.insert("@global".to_string(), DomainSuffix {
            suffix: "@global".to_string(),
            suffix_type: SuffixType::Global,
            security_level: SecurityLevel::Public,
            routing_plane: "app".to_string(),
            authority: "Global Domain Authority".to_string(),
            enabled: true,
        });

        // Country suffixes
        let countries = vec![
            ("@in", "India"),
            ("@us", "United States"),
            ("@uk", "United Kingdom"),
            ("@de", "Germany"),
            ("@jp", "Japan"),
            ("@cn", "China"),
            ("@fr", "France"),
            ("@ca", "Canada"),
        ];

        for (suffix, country) in countries {
            registry.insert(suffix.to_string(), DomainSuffix {
                suffix: suffix.to_string(),
                suffix_type: SuffixType::Country(country.to_string()),
                security_level: SecurityLevel::Enhanced,
                routing_plane: "app".to_string(),
                authority: format!("{} Domain Authority", country),
                enabled: true,
            });
        }

        // Government suffix
        registry.insert("@gov".to_string(), DomainSuffix {
            suffix: "@gov".to_string(),
            suffix_type: SuffixType::Government,
            security_level: SecurityLevel::Classified,
            routing_plane: "gov".to_string(),
            authority: "Government Domain Authority".to_string(),
            enabled: true,
        });

        // International suffix
        registry.insert("@int".to_string(), DomainSuffix {
            suffix: "@int".to_string(),
            suffix_type: SuffixType::International,
            security_level: SecurityLevel::Enhanced,
            routing_plane: "secure".to_string(),
            authority: "International Domain Authority".to_string(),
            enabled: true,
        });

        // Corporate suffix
        registry.insert("@corp".to_string(), DomainSuffix {
            suffix: "@corp".to_string(),
            suffix_type: SuffixType::Corporate,
            security_level: SecurityLevel::Enhanced,
            routing_plane: "secure".to_string(),
            authority: "Corporate Domain Authority".to_string(),
            enabled: true,
        });

        // Educational suffix
        registry.insert("@edu".to_string(), DomainSuffix {
            suffix: "@edu".to_string(),
            suffix_type: SuffixType::Educational,
            security_level: SecurityLevel::Enhanced,
            routing_plane: "app".to_string(),
            authority: "Educational Domain Authority".to_string(),
            enabled: true,
        });

        // Military suffix
        registry.insert("@mil".to_string(), DomainSuffix {
            suffix: "@mil".to_string(),
            suffix_type: SuffixType::Military,
            security_level: SecurityLevel::Classified,
            routing_plane: "secure".to_string(),
            authority: "Military Domain Authority".to_string(),
            enabled: true,
        });

        // Dark network suffix
        registry.insert("@dark".to_string(), DomainSuffix {
            suffix: "@dark".to_string(),
            suffix_type: SuffixType::Dark,
            security_level: SecurityLevel::Quantum,
            routing_plane: "dark".to_string(),
            authority: "Dark Network Authority".to_string(),
            enabled: true,
        });

        info!("✅ Initialized {} domain suffixes", registry.len());
        Ok(())
    }

    /// Register a domain with suffix (e.g., "prav@global")
    pub async fn register_domain(&self, domain_name: &str, suffix: &str) -> Result<DomainMapping> {
        let full_domain = format!("{}@{}", domain_name, suffix.trim_start_matches('@'));
        
        // Check if suffix exists
        let registry = self.suffix_registry.read().await;
        let domain_suffix = registry.get(suffix)
            .ok_or_else(|| anyhow!("Unknown domain suffix: {}", suffix))?;

        if !domain_suffix.enabled {
            return Err(anyhow!("Domain suffix {} is disabled", suffix));
        }

        // Create httpcg URL based on suffix type
        let httpcg_url = self.create_httpcg_url(domain_name, &domain_suffix)?;

        let mapping = DomainMapping {
            full_domain: full_domain.clone(),
            base_name: domain_name.to_string(),
            suffix: suffix.to_string(),
            httpcg_url,
            routing_plane: domain_suffix.routing_plane.clone(),
            security_level: domain_suffix.security_level.clone(),
            active: true,
        };

        // Store mapping
        let mut mappings = self.domain_mappings.write().await;
        mappings.insert(full_domain.clone(), mapping.clone());

        info!("✅ Registered domain: {} → {}", full_domain, mapping.httpcg_url);
        Ok(mapping)
    }

    /// Create httpcg URL from domain and suffix
    fn create_httpcg_url(&self, domain_name: &str, suffix: &DomainSuffix) -> Result<String> {
        let httpcg_domain = match &suffix.suffix_type {
            SuffixType::Global => format!("{}.global", domain_name),
            SuffixType::Country(country_code) => {
                let code = suffix.suffix.trim_start_matches('@');
                format!("{}.{}", domain_name, code)
            },
            SuffixType::Government => format!("{}.gov", domain_name),
            SuffixType::International => format!("{}.int", domain_name),
            SuffixType::Corporate => format!("{}.corp", domain_name),
            SuffixType::Educational => format!("{}.edu", domain_name),
            SuffixType::Military => format!("{}.mil", domain_name),
            SuffixType::Dark => format!("{}.dark", domain_name),
        };

        Ok(format!("httpcg://{}/{}/", suffix.routing_plane, httpcg_domain))
    }

    /// Resolve domain to httpcg URL
    pub async fn resolve_domain(&self, full_domain: &str) -> Result<String> {
        let mappings = self.domain_mappings.read().await;
        let mapping = mappings.get(full_domain)
            .ok_or_else(|| anyhow!("Domain not found: {}", full_domain))?;

        if !mapping.active {
            return Err(anyhow!("Domain is inactive: {}", full_domain));
        }

        Ok(mapping.httpcg_url.clone())
    }

    /// List all registered domains
    pub async fn list_domains(&self) -> Vec<DomainMapping> {
        let mappings = self.domain_mappings.read().await;
        mappings.values().cloned().collect()
    }

    /// List all available suffixes
    pub async fn list_suffixes(&self) -> Vec<DomainSuffix> {
        let registry = self.suffix_registry.read().await;
        registry.values().cloned().collect()
    }

    /// Get routing plane for domain
    pub async fn get_routing_plane(&self, full_domain: &str) -> Result<String> {
        let mappings = self.domain_mappings.read().await;
        let mapping = mappings.get(full_domain)
            .ok_or_else(|| anyhow!("Domain not found: {}", full_domain))?;
        
        Ok(mapping.routing_plane.clone())
    }

    /// Check if domain requires special security
    pub async fn get_security_requirements(&self, full_domain: &str) -> Result<SecurityPolicy> {
        let mappings = self.domain_mappings.read().await;
        let mapping = mappings.get(full_domain)
            .ok_or_else(|| anyhow!("Domain not found: {}", full_domain))?;

        let policy = self.routing_config.security_policies
            .get(&mapping.security_level)
            .cloned()
            .unwrap_or_else(|| SecurityPolicy {
                requires_auth: false,
                quantum_safe_only: false,
                audit_required: false,
                encryption_level: "standard".to_string(),
            });

        Ok(policy)
    }

    /// Default routing configuration
    fn default_routing_config() -> HttpcgRoutingConfig {
        let mut plane_mappings = HashMap::new();
        plane_mappings.insert(SuffixType::Global, "app".to_string());
        plane_mappings.insert(SuffixType::Government, "gov".to_string());
        plane_mappings.insert(SuffixType::International, "secure".to_string());
        plane_mappings.insert(SuffixType::Corporate, "secure".to_string());
        plane_mappings.insert(SuffixType::Educational, "app".to_string());
        plane_mappings.insert(SuffixType::Military, "secure".to_string());
        plane_mappings.insert(SuffixType::Dark, "dark".to_string());

        let mut security_policies = HashMap::new();
        
        security_policies.insert(SecurityLevel::Public, SecurityPolicy {
            requires_auth: false,
            quantum_safe_only: false,
            audit_required: false,
            encryption_level: "standard".to_string(),
        });

        security_policies.insert(SecurityLevel::Enhanced, SecurityPolicy {
            requires_auth: true,
            quantum_safe_only: false,
            audit_required: true,
            encryption_level: "enhanced".to_string(),
        });

        security_policies.insert(SecurityLevel::Classified, SecurityPolicy {
            requires_auth: true,
            quantum_safe_only: true,
            audit_required: true,
            encryption_level: "classified".to_string(),
        });

        security_policies.insert(SecurityLevel::Quantum, SecurityPolicy {
            requires_auth: true,
            quantum_safe_only: true,
            audit_required: true,
            encryption_level: "quantum".to_string(),
        });

        HttpcgRoutingConfig {
            default_plane: "app".to_string(),
            plane_mappings,
            security_policies,
        }
    }
}

/// Wallet integration for automatic suffix addition
pub struct WalletDomainHelper;

impl WalletDomainHelper {
    /// Add appropriate suffix to domain based on context
    pub fn add_suffix(domain_name: &str, context: DomainContext) -> String {
        match context {
            DomainContext::Global => format!("{}@global", domain_name),
            DomainContext::Country(code) => format!("{}@{}", domain_name, code),
            DomainContext::Government => format!("{}@gov", domain_name),
            DomainContext::International => format!("{}@int", domain_name),
            DomainContext::Corporate => format!("{}@corp", domain_name),
            DomainContext::Educational => format!("{}@edu", domain_name),
            DomainContext::Military => format!("{}@mil", domain_name),
            DomainContext::Dark => format!("{}@dark", domain_name),
        }
    }

    /// Parse domain with suffix
    pub fn parse_domain(full_domain: &str) -> Result<(String, String)> {
        if let Some(at_pos) = full_domain.find('@') {
            let domain_name = full_domain[..at_pos].to_string();
            let suffix = full_domain[at_pos..].to_string();
            Ok((domain_name, suffix))
        } else {
            Err(anyhow!("Invalid domain format, missing @ suffix"))
        }
    }
}

/// Domain context for automatic suffix selection
#[derive(Debug, Clone)]
pub enum DomainContext {
    Global,
    Country(String),
    Government,
    International,
    Corporate,
    Educational,
    Military,
    Dark,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_domain_registration() -> Result<()> {
        let system = HttpcgSuffixDomainSystem::new().await?;

        // Test global domain
        let mapping = system.register_domain("prav", "@global").await?;
        assert_eq!(mapping.full_domain, "prav@global");
        assert_eq!(mapping.httpcg_url, "httpcg://app/prav.global/");

        // Test country domain
        let mapping = system.register_domain("prav", "@in").await?;
        assert_eq!(mapping.full_domain, "prav@in");
        assert_eq!(mapping.httpcg_url, "httpcg://app/prav.in/");

        // Test government domain
        let mapping = system.register_domain("identity", "@gov").await?;
        assert_eq!(mapping.full_domain, "identity@gov");
        assert_eq!(mapping.httpcg_url, "httpcg://gov/identity.gov/");

        Ok(())
    }

    #[tokio::test]
    async fn test_wallet_helper() -> Result<()> {
        // Test suffix addition
        assert_eq!(WalletDomainHelper::add_suffix("prav", DomainContext::Global), "prav@global");
        assert_eq!(WalletDomainHelper::add_suffix("prav", DomainContext::Country("in".to_string())), "prav@in");
        assert_eq!(WalletDomainHelper::add_suffix("secure", DomainContext::Government), "secure@gov");

        // Test domain parsing
        let (name, suffix) = WalletDomainHelper::parse_domain("prav@global")?;
        assert_eq!(name, "prav");
        assert_eq!(suffix, "@global");

        Ok(())
    }
}
