use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use crate::registry::geodid::{GeoDID, GeoScope, AdminLevel};

/// GeoLedger - On-chain registries for jurisdiction mapping and geopolitical relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLedger {
    /// Jurisdiction mapping: GeoDID -> geographic and political information
    pub jurisdiction_map: HashMap<String, JurisdictionInfo>,
    /// Adjacency graph: geographic and political relationships between jurisdictions
    pub adjacency_graph: AdjacencyGraph,
    /// Alignment matrix: treaty blocks, sanctions, data adequacy groups
    pub alignment_matrix: AlignmentMatrix,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

/// Jurisdiction information for a GeoDID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JurisdictionInfo {
    /// GeoDID reference
    pub geodid: String,
    /// Geographic polygon (WKT format)
    pub polygon: Option<String>,
    /// ISO codes covered by this jurisdiction
    pub iso_codes: Vec<String>,
    /// Neighboring jurisdictions
    pub neighbors: Vec<String>,
    /// Sovereign risk score (0.0 = highest risk, 1.0 = lowest risk)
    pub sovereign_risk: f64,
    /// Rule of law index (0.0 = weakest, 1.0 = strongest)
    pub rule_of_law: f64,
    /// Corruption perception index (0.0 = most corrupt, 1.0 = least corrupt)
    pub corruption_index: f64,
    /// Political stability score (0.0 = least stable, 1.0 = most stable)
    pub political_stability: f64,
    /// Last risk assessment update
    pub risk_updated_at: DateTime<Utc>,
}

/// Adjacency graph representing relationships between jurisdictions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdjacencyGraph {
    /// Vertices: jurisdiction GeoDIDs
    pub vertices: HashSet<String>,
    /// Edges: relationships between jurisdictions
    pub edges: HashMap<String, Vec<Relationship>>,
}

/// Relationship between two jurisdictions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    /// Target jurisdiction GeoDID
    pub target: String,
    /// Type of relationship
    pub relationship_type: RelationshipType,
    /// Relationship strength (0.0 = weakest, 1.0 = strongest)
    pub strength: f64,
    /// Relationship established date
    pub established_at: DateTime<Utc>,
}

/// Types of relationships between jurisdictions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    /// Physical border sharing
    SharedBorder,
    /// Maritime boundary
    MaritimeBoundary,
    /// Data corridor (digital relationship)
    DataCorridor,
    /// Energy corridor
    EnergyCorridor,
    /// Compute corridor
    ComputeCorridor,
    /// Financial corridor
    FinancialCorridor,
    /// Treaty alliance
    TreatyAlliance,
    /// Trade agreement
    TradeAgreement,
    /// Sanctions regime
    Sanctions,
    /// Diplomatic relations
    DiplomaticRelations,
}

/// Alignment matrix for geopolitical clustering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlignmentMatrix {
    /// Alignment clusters (e.g., EU, NATO, ASEAN, etc.)
    pub clusters: HashMap<String, AlignmentCluster>,
    /// Jurisdiction to cluster mappings
    pub jurisdiction_clusters: HashMap<String, Vec<String>>,
    /// Sanctions lists
    pub sanctions: HashMap<String, SanctionsList>,
    /// Data adequacy groups
    pub data_adequacy: HashMap<String, DataAdequacyGroup>,
}

/// Alignment cluster (treaty block, alliance, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlignmentCluster {
    /// Cluster name
    pub name: String,
    /// Cluster type
    pub cluster_type: ClusterType,
    /// Member jurisdictions
    pub members: Vec<String>,
    /// Cluster establishment date
    pub established_at: DateTime<Utc>,
    /// Cluster status
    pub status: ClusterStatus,
}

/// Types of alignment clusters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClusterType {
    /// Military alliance
    MilitaryAlliance,
    /// Economic union
    EconomicUnion,
    /// Trade bloc
    TradeBloc,
    /// Regulatory framework
    RegulatoryFramework,
    /// Data sharing agreement
    DataSharingAgreement,
    /// Sanctions regime
    SanctionsRegime,
    /// Custom cluster
    Custom { description: String },
}

/// Status of alignment cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClusterStatus {
    Active,
    Suspended,
    Dissolved,
    Pending,
}

/// Sanctions list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanctionsList {
    /// Sanctions name
    pub name: String,
    /// Sanctioning authority
    pub authority: String,
    /// Sanctioned jurisdictions
    pub sanctioned_jurisdictions: Vec<String>,
    /// Sanctions type
    pub sanctions_type: SanctionsType,
    /// Effective date
    pub effective_from: DateTime<Utc>,
    /// Expiration date (if any)
    pub expires_at: Option<DateTime<Utc>>,
}

/// Types of sanctions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SanctionsType {
    Economic,
    Financial,
    Technology,
    DataTransfer,
    Diplomatic,
    Comprehensive,
}

/// Data adequacy group for cross-border data transfers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAdequacyGroup {
    /// Group name
    pub name: String,
    /// Data protection standard
    pub standard: String,
    /// Member jurisdictions with adequate data protection
    pub adequate_jurisdictions: Vec<String>,
    /// Adequacy assessment date
    pub assessed_at: DateTime<Utc>,
    /// Next review date
    pub next_review: DateTime<Utc>,
}

impl GeoLedger {
    /// Create a new GeoLedger
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            jurisdiction_map: HashMap::new(),
            adjacency_graph: AdjacencyGraph::new(),
            alignment_matrix: AlignmentMatrix::new(),
            created_at: now,
            last_updated: now,
        }
    }

    /// Register a jurisdiction in the ledger
    pub fn register_jurisdiction(&mut self, geodid: &GeoDID, risk_info: RiskAssessment) -> Result<(), String> {
        let jurisdiction_info = JurisdictionInfo {
            geodid: geodid.did.clone(),
            polygon: geodid.geo_scope.polygon.clone(),
            iso_codes: geodid.geo_scope.iso_codes.clone(),
            neighbors: Vec::new(),
            sovereign_risk: risk_info.sovereign_risk,
            rule_of_law: risk_info.rule_of_law,
            corruption_index: risk_info.corruption_index,
            political_stability: risk_info.political_stability,
            risk_updated_at: Utc::now(),
        };

        self.jurisdiction_map.insert(geodid.did.clone(), jurisdiction_info);
        self.adjacency_graph.add_vertex(geodid.did.clone());
        self.last_updated = Utc::now();
        
        Ok(())
    }

    /// Add relationship between jurisdictions
    pub fn add_relationship(&mut self, from: &str, to: &str, relationship: Relationship) -> Result<(), String> {
        if !self.jurisdiction_map.contains_key(from) {
            return Err(format!("Jurisdiction {} not found", from));
        }
        if !self.jurisdiction_map.contains_key(to) {
            return Err(format!("Jurisdiction {} not found", to));
        }

        self.adjacency_graph.add_edge(from.to_string(), relationship);
        
        // Update neighbors list
        if let Some(jurisdiction) = self.jurisdiction_map.get_mut(from) {
            if !jurisdiction.neighbors.contains(&to.to_string()) {
                jurisdiction.neighbors.push(to.to_string());
            }
        }

        self.last_updated = Utc::now();
        Ok(())
    }

    /// Get neighbors of a jurisdiction
    pub fn get_neighbors(&self, geodid: &str) -> Vec<&String> {
        self.adjacency_graph.get_neighbors(geodid)
    }

    /// Calculate stability score for a jurisdiction
    pub fn calculate_stability(&self, geodid: &str) -> f64 {
        match self.jurisdiction_map.get(geodid) {
            Some(info) => {
                // Composite stability score
                let risk_component = 1.0 - info.sovereign_risk;
                let rule_of_law_component = info.rule_of_law;
                let corruption_component = info.corruption_index;
                let stability_component = info.political_stability;
                
                // Weighted average
                (risk_component * 0.3 + rule_of_law_component * 0.25 + 
                 corruption_component * 0.25 + stability_component * 0.2).min(1.0).max(0.0)
            },
            None => 0.0, // Unknown jurisdiction has zero stability
        }
    }

    /// Check if jurisdictions are in the same alignment cluster
    pub fn are_aligned(&self, geodid1: &str, geodid2: &str) -> bool {
        self.alignment_matrix.are_jurisdictions_aligned(geodid1, geodid2)
    }

    /// Get alignment clusters for a jurisdiction
    pub fn get_alignment_clusters(&self, geodid: &str) -> Vec<&AlignmentCluster> {
        self.alignment_matrix.get_clusters_for_jurisdiction(geodid)
    }

    /// Check if data transfer is allowed between jurisdictions
    pub fn is_data_transfer_allowed(&self, from: &str, to: &str) -> bool {
        // Check sanctions
        if self.alignment_matrix.has_sanctions_between(from, to) {
            return false;
        }

        // Check data adequacy
        self.alignment_matrix.has_data_adequacy_between(from, to)
    }

    /// Update risk assessment for a jurisdiction
    pub fn update_risk_assessment(&mut self, geodid: &str, risk_info: RiskAssessment) -> Result<(), String> {
        match self.jurisdiction_map.get_mut(geodid) {
            Some(jurisdiction) => {
                jurisdiction.sovereign_risk = risk_info.sovereign_risk;
                jurisdiction.rule_of_law = risk_info.rule_of_law;
                jurisdiction.corruption_index = risk_info.corruption_index;
                jurisdiction.political_stability = risk_info.political_stability;
                jurisdiction.risk_updated_at = Utc::now();
                self.last_updated = Utc::now();
                Ok(())
            },
            None => Err(format!("Jurisdiction {} not found", geodid)),
        }
    }
}

impl AdjacencyGraph {
    /// Create a new adjacency graph
    pub fn new() -> Self {
        Self {
            vertices: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    /// Add a vertex to the graph
    pub fn add_vertex(&mut self, vertex: String) {
        self.vertices.insert(vertex.clone());
        self.edges.entry(vertex).or_insert_with(Vec::new);
    }

    /// Add an edge to the graph
    pub fn add_edge(&mut self, from: String, relationship: Relationship) {
        self.edges.entry(from).or_insert_with(Vec::new).push(relationship);
    }

    /// Get neighbors of a vertex
    pub fn get_neighbors(&self, vertex: &str) -> Vec<&String> {
        match self.edges.get(vertex) {
            Some(relationships) => relationships.iter().map(|r| &r.target).collect(),
            None => Vec::new(),
        }
    }

    /// Get relationships from a vertex
    pub fn get_relationships(&self, vertex: &str) -> Option<&Vec<Relationship>> {
        self.edges.get(vertex)
    }
}

impl AlignmentMatrix {
    /// Create a new alignment matrix
    pub fn new() -> Self {
        Self {
            clusters: HashMap::new(),
            jurisdiction_clusters: HashMap::new(),
            sanctions: HashMap::new(),
            data_adequacy: HashMap::new(),
        }
    }

    /// Add an alignment cluster
    pub fn add_cluster(&mut self, cluster: AlignmentCluster) {
        for member in &cluster.members {
            self.jurisdiction_clusters
                .entry(member.clone())
                .or_insert_with(Vec::new)
                .push(cluster.name.clone());
        }
        self.clusters.insert(cluster.name.clone(), cluster);
    }

    /// Check if two jurisdictions are aligned
    pub fn are_jurisdictions_aligned(&self, geodid1: &str, geodid2: &str) -> bool {
        if let (Some(clusters1), Some(clusters2)) = (
            self.jurisdiction_clusters.get(geodid1),
            self.jurisdiction_clusters.get(geodid2)
        ) {
            clusters1.iter().any(|c1| clusters2.contains(c1))
        } else {
            false
        }
    }

    /// Get clusters for a jurisdiction
    pub fn get_clusters_for_jurisdiction(&self, geodid: &str) -> Vec<&AlignmentCluster> {
        match self.jurisdiction_clusters.get(geodid) {
            Some(cluster_names) => cluster_names.iter()
                .filter_map(|name| self.clusters.get(name))
                .collect(),
            None => Vec::new(),
        }
    }

    /// Check if there are sanctions between jurisdictions
    pub fn has_sanctions_between(&self, from: &str, to: &str) -> bool {
        self.sanctions.values().any(|sanctions_list| {
            sanctions_list.sanctioned_jurisdictions.contains(&to.to_string()) &&
            (sanctions_list.authority == from || 
             self.are_jurisdictions_aligned(&sanctions_list.authority, from))
        })
    }

    /// Check if there is data adequacy between jurisdictions
    pub fn has_data_adequacy_between(&self, from: &str, to: &str) -> bool {
        self.data_adequacy.values().any(|adequacy_group| {
            adequacy_group.adequate_jurisdictions.contains(&from.to_string()) &&
            adequacy_group.adequate_jurisdictions.contains(&to.to_string())
        })
    }
}

/// Risk assessment information for a jurisdiction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    /// Sovereign risk score (0.0 = highest risk, 1.0 = lowest risk)
    pub sovereign_risk: f64,
    /// Rule of law index (0.0 = weakest, 1.0 = strongest)
    pub rule_of_law: f64,
    /// Corruption perception index (0.0 = most corrupt, 1.0 = least corrupt)
    pub corruption_index: f64,
    /// Political stability score (0.0 = least stable, 1.0 = most stable)
    pub political_stability: f64,
}

impl RiskAssessment {
    /// Create a new risk assessment with default values
    pub fn new() -> Self {
        Self {
            sovereign_risk: 0.5,
            rule_of_law: 0.5,
            corruption_index: 0.5,
            political_stability: 0.5,
        }
    }

    /// Create a high-risk assessment
    pub fn high_risk() -> Self {
        Self {
            sovereign_risk: 0.2,
            rule_of_law: 0.3,
            corruption_index: 0.2,
            political_stability: 0.3,
        }
    }

    /// Create a low-risk assessment
    pub fn low_risk() -> Self {
        Self {
            sovereign_risk: 0.1,  // Low risk means low sovereign risk
            rule_of_law: 0.9,     // High rule of law is good
            corruption_index: 0.9, // High corruption index (low corruption) is good
            political_stability: 0.9, // High political stability is good
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::geodid::{GeoDID, GeoScope};

    #[test]
    fn test_geoledger_creation() {
        let ledger = GeoLedger::new();
        assert!(ledger.jurisdiction_map.is_empty());
        assert!(ledger.adjacency_graph.vertices.is_empty());
    }

    #[test]
    fn test_jurisdiction_registration() {
        let mut ledger = GeoLedger::new();
        let scope = GeoScope::country("US".to_string(), "America/New_York".to_string());
        let geodid = GeoDID::new("did:geo:us:gov".to_string(), scope);
        let risk = RiskAssessment::low_risk();

        assert!(ledger.register_jurisdiction(&geodid, risk).is_ok());
        assert!(ledger.jurisdiction_map.contains_key("did:geo:us:gov"));
    }

    #[test]
    fn test_relationship_management() {
        let mut ledger = GeoLedger::new();
        
        // Register two jurisdictions
        let us_scope = GeoScope::country("US".to_string(), "America/New_York".to_string());
        let us_geodid = GeoDID::new("did:geo:us:gov".to_string(), us_scope);
        let ca_scope = GeoScope::country("CA".to_string(), "America/Toronto".to_string());
        let ca_geodid = GeoDID::new("did:geo:ca:gov".to_string(), ca_scope);
        
        ledger.register_jurisdiction(&us_geodid, RiskAssessment::low_risk()).unwrap();
        ledger.register_jurisdiction(&ca_geodid, RiskAssessment::low_risk()).unwrap();

        // Add relationship
        let relationship = Relationship {
            target: "did:geo:ca:gov".to_string(),
            relationship_type: RelationshipType::SharedBorder,
            strength: 0.9,
            established_at: Utc::now(),
        };

        assert!(ledger.add_relationship("did:geo:us:gov", "did:geo:ca:gov", relationship).is_ok());
        
        let neighbors = ledger.get_neighbors("did:geo:us:gov");
        assert_eq!(neighbors.len(), 1);
        assert_eq!(neighbors[0], "did:geo:ca:gov");
    }

    #[test]
    fn test_stability_calculation() {
        let mut ledger = GeoLedger::new();
        let scope = GeoScope::country("US".to_string(), "America/New_York".to_string());
        let geodid = GeoDID::new("did:geo:us:gov".to_string(), scope);
        let risk = RiskAssessment::low_risk();

        ledger.register_jurisdiction(&geodid, risk).unwrap();
        
        let stability = ledger.calculate_stability("did:geo:us:gov");
        assert!(stability > 0.8); // Should be high for low-risk assessment
    }
}
