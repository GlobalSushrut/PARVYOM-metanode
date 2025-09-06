use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Geolocation-bound Decentralized Identifier (GeoDID)
/// Provides jurisdiction-aware identity with geopolitical binding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoDID {
    /// Base DID identifier
    pub did: String,
    /// Geographic scope definition
    pub geo_scope: GeoScope,
    /// Validity period
    pub valid_from: DateTime<Utc>,
    pub valid_to: Option<DateTime<Utc>>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last verification timestamp
    pub last_verified: DateTime<Utc>,
}

/// Geographic scope definition for GeoDID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoScope {
    /// ISO 3166-2 country and subdivision codes
    pub iso_codes: Vec<String>,
    /// Geohash precision codes for geographic areas
    pub geohash: Vec<String>,
    /// Geographic polygon definition (WKT format)
    pub polygon: Option<String>,
    /// Timezone identifier
    pub timezone: String,
    /// Administrative level (country, state, city, etc.)
    pub admin_level: AdminLevel,
}

/// Administrative levels for geographic scope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdminLevel {
    /// Global scope (no geographic restrictions)
    Global,
    /// Continental scope (e.g., Europe, Asia)
    Continental { continent: String },
    /// Country level
    Country { country_code: String },
    /// State/Province level
    State { country_code: String, state_code: String },
    /// City/Municipality level
    City { country_code: String, state_code: String, city_code: String },
    /// Custom administrative boundary
    Custom { boundary_name: String, parent_scope: Box<AdminLevel> },
}

impl GeoDID {
    /// Create a new GeoDID
    pub fn new(did: String, geo_scope: GeoScope) -> Self {
        let now = Utc::now();
        Self {
            did,
            geo_scope,
            valid_from: now,
            valid_to: None,
            created_at: now,
            last_verified: now,
        }
    }

    /// Check if GeoDID is currently valid
    pub fn is_valid(&self) -> bool {
        let now = Utc::now();
        now >= self.valid_from && self.valid_to.map_or(true, |end| now <= end)
    }

    /// Check if GeoDID has jurisdiction over a specific geographic area
    pub fn has_jurisdiction(&self, target_scope: &GeoScope) -> bool {
        // Check ISO code overlap
        let iso_overlap = self.geo_scope.iso_codes.iter()
            .any(|code| target_scope.iso_codes.contains(code));
        
        // Check geohash overlap (prefix matching for hierarchical geohash)
        let geohash_overlap = self.geo_scope.geohash.iter()
            .any(|hash| target_scope.geohash.iter()
                .any(|target_hash| target_hash.starts_with(hash) || hash.starts_with(target_hash)));
        
        // Check administrative level compatibility
        let admin_compatible = self.is_admin_level_compatible(&target_scope.admin_level);
        
        iso_overlap || geohash_overlap || admin_compatible
    }

    /// Check if administrative levels are compatible
    fn is_admin_level_compatible(&self, target_level: &AdminLevel) -> bool {
        match (&self.geo_scope.admin_level, target_level) {
            (AdminLevel::Global, _) => true,
            (AdminLevel::Continental { continent: c1 }, AdminLevel::Continental { continent: c2 }) => c1 == c2,
            (AdminLevel::Continental { continent: c1 }, AdminLevel::Country { country_code }) => {
                // Would need a continent-to-country mapping here
                // For now, simplified check
                true // TODO: Implement proper continent-country mapping
            },
            (AdminLevel::Country { country_code: c1 }, AdminLevel::Country { country_code: c2 }) => c1 == c2,
            (AdminLevel::Country { country_code: c1 }, AdminLevel::State { country_code: c2, .. }) => c1 == c2,
            (AdminLevel::State { country_code: c1, state_code: s1 }, AdminLevel::State { country_code: c2, state_code: s2 }) => {
                c1 == c2 && s1 == s2
            },
            (AdminLevel::State { country_code: c1, state_code: s1 }, AdminLevel::City { country_code: c2, state_code: s2, .. }) => {
                c1 == c2 && s1 == s2
            },
            _ => false,
        }
    }

    /// Update verification timestamp
    pub fn update_verification(&mut self) {
        self.last_verified = Utc::now();
    }

    /// Set expiration date
    pub fn set_expiration(&mut self, expiry: DateTime<Utc>) {
        self.valid_to = Some(expiry);
    }
}

impl GeoScope {
    /// Create a global scope (no geographic restrictions)
    pub fn global() -> Self {
        Self {
            iso_codes: vec![],
            geohash: vec![],
            polygon: None,
            timezone: "UTC".to_string(),
            admin_level: AdminLevel::Global,
        }
    }

    /// Create a country-level scope
    pub fn country(country_code: String, timezone: String) -> Self {
        Self {
            iso_codes: vec![country_code.clone()],
            geohash: vec![],
            polygon: None,
            timezone,
            admin_level: AdminLevel::Country { country_code },
        }
    }

    /// Create a state-level scope
    pub fn state(country_code: String, state_code: String, timezone: String) -> Self {
        let iso_code = format!("{}-{}", country_code, state_code);
        Self {
            iso_codes: vec![iso_code],
            geohash: vec![],
            polygon: None,
            timezone,
            admin_level: AdminLevel::State { country_code, state_code },
        }
    }

    /// Add geohash precision area
    pub fn with_geohash(mut self, geohash: Vec<String>) -> Self {
        self.geohash = geohash;
        self
    }

    /// Add polygon boundary
    pub fn with_polygon(mut self, polygon: String) -> Self {
        self.polygon = Some(polygon);
        self
    }

    /// Calculate area overlap with another scope (simplified)
    pub fn area_overlap(&self, other: &GeoScope) -> f64 {
        // Simplified overlap calculation
        // In a real implementation, this would use proper geographic calculations
        
        // Check ISO code overlap
        let iso_overlap = self.iso_codes.iter()
            .filter(|code| other.iso_codes.contains(code))
            .count() as f64 / self.iso_codes.len().max(1) as f64;
        
        // Check geohash overlap (simplified)
        let geohash_overlap = if !self.geohash.is_empty() && !other.geohash.is_empty() {
            let overlapping_hashes = self.geohash.iter()
                .filter(|hash| other.geohash.iter()
                    .any(|other_hash| other_hash.starts_with(*hash) || hash.starts_with(other_hash)))
                .count() as f64;
            overlapping_hashes / self.geohash.len().max(1) as f64
        } else {
            0.0
        };
        
        // Return maximum overlap found
        iso_overlap.max(geohash_overlap)
    }
}

/// GeoDID registry for managing geolocation-bound identities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoDIDRegistry {
    /// Registered GeoDIDs
    pub geodids: HashMap<String, GeoDID>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

impl GeoDIDRegistry {
    /// Create a new GeoDID registry
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            geodids: HashMap::new(),
            created_at: now,
            last_updated: now,
        }
    }

    /// Register a new GeoDID
    pub fn register_geodid(&mut self, geodid: GeoDID) -> Result<(), String> {
        if self.geodids.contains_key(&geodid.did) {
            return Err(format!("GeoDID {} already registered", geodid.did));
        }

        if !geodid.is_valid() {
            return Err(format!("GeoDID {} is not valid", geodid.did));
        }

        self.geodids.insert(geodid.did.clone(), geodid);
        self.last_updated = Utc::now();
        Ok(())
    }

    /// Get GeoDID by DID
    pub fn get_geodid(&self, did: &str) -> Option<&GeoDID> {
        self.geodids.get(did)
    }

    /// Find GeoDIDs with jurisdiction over a geographic scope
    pub fn find_jurisdictional_geodids(&self, scope: &GeoScope) -> Vec<&GeoDID> {
        self.geodids.values()
            .filter(|geodid| geodid.is_valid() && geodid.has_jurisdiction(scope))
            .collect()
    }

    /// Update GeoDID verification
    pub fn update_verification(&mut self, did: &str) -> Result<(), String> {
        match self.geodids.get_mut(did) {
            Some(geodid) => {
                geodid.update_verification();
                self.last_updated = Utc::now();
                Ok(())
            },
            None => Err(format!("GeoDID {} not found", did)),
        }
    }

    /// Remove expired GeoDIDs
    pub fn cleanup_expired(&mut self) -> usize {
        let initial_count = self.geodids.len();
        self.geodids.retain(|_, geodid| geodid.is_valid());
        let removed_count = initial_count - self.geodids.len();
        
        if removed_count > 0 {
            self.last_updated = Utc::now();
        }
        
        removed_count
    }

    /// Get registry statistics
    pub fn get_stats(&self) -> GeoDIDStats {
        let total_geodids = self.geodids.len();
        let valid_geodids = self.geodids.values().filter(|g| g.is_valid()).count();
        let expired_geodids = total_geodids - valid_geodids;

        // Count by administrative level
        let mut admin_level_counts = HashMap::new();
        for geodid in self.geodids.values() {
            let level_name = match &geodid.geo_scope.admin_level {
                AdminLevel::Global => "Global".to_string(),
                AdminLevel::Continental { continent } => format!("Continental({})", continent),
                AdminLevel::Country { country_code } => format!("Country({})", country_code),
                AdminLevel::State { country_code, state_code } => format!("State({}-{})", country_code, state_code),
                AdminLevel::City { country_code, state_code, city_code } => format!("City({}-{}-{})", country_code, state_code, city_code),
                AdminLevel::Custom { boundary_name, .. } => format!("Custom({})", boundary_name),
            };
            *admin_level_counts.entry(level_name).or_insert(0) += 1;
        }

        GeoDIDStats {
            total_geodids,
            valid_geodids,
            expired_geodids,
            admin_level_counts,
        }
    }
}

/// Statistics for GeoDID registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoDIDStats {
    pub total_geodids: usize,
    pub valid_geodids: usize,
    pub expired_geodids: usize,
    pub admin_level_counts: HashMap<String, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geodid_creation() {
        let scope = GeoScope::country("US".to_string(), "America/New_York".to_string());
        let geodid = GeoDID::new("did:geo:us:example".to_string(), scope);
        
        assert!(geodid.is_valid());
        assert_eq!(geodid.did, "did:geo:us:example");
    }

    #[test]
    fn test_jurisdiction_check() {
        let us_scope = GeoScope::country("US".to_string(), "America/New_York".to_string());
        let us_geodid = GeoDID::new("did:geo:us:gov".to_string(), us_scope);
        
        let ny_scope = GeoScope::state("US".to_string(), "NY".to_string(), "America/New_York".to_string());
        
        assert!(us_geodid.has_jurisdiction(&ny_scope));
    }

    #[test]
    fn test_registry_operations() {
        let mut registry = GeoDIDRegistry::new();
        
        let scope = GeoScope::country("CA".to_string(), "America/Toronto".to_string());
        let geodid = GeoDID::new("did:geo:ca:gov".to_string(), scope);
        
        assert!(registry.register_geodid(geodid).is_ok());
        assert!(registry.get_geodid("did:geo:ca:gov").is_some());
        
        let stats = registry.get_stats();
        assert_eq!(stats.total_geodids, 1);
        assert_eq!(stats.valid_geodids, 1);
    }
}
