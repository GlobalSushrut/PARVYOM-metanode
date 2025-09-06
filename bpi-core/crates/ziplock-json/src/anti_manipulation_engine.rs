use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};

/// Manipulation detection patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ManipulationPattern {
    SybilAttack,
    CoordinatedVoting,
    StakeConcentration,
    GeographicClustering,
    TemporalManipulation,
    EclipseAttack,
    LongRangeAttack,
    NothingAtStake,
    Collusion,
    FrontRunning,
}

/// Manipulation detection alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManipulationAlert {
    pub alert_id: String,
    pub pattern: ManipulationPattern,
    pub severity: AlertSeverity,
    pub confidence: f64,
    pub description: String,
    pub affected_entities: Vec<String>,
    pub detected_at: DateTime<Utc>,
    pub risk_score: f64,
    pub recommended_actions: Vec<String>,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
    Emergency,
}

/// Node behavior tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeBehavior {
    pub node_id: String,
    pub stake_amount: u64,
    pub voting_history: VecDeque<VoteRecord>,
    pub geographic_location: Option<(f64, f64)>,
    pub join_time: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub reputation_score: f64,
    pub anomaly_score: f64,
}

/// Vote record for tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteRecord {
    pub proposal_id: String,
    pub vote: VoteChoice,
    pub timestamp: DateTime<Utc>,
    pub stake_weight: u64,
}

/// Vote choices
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VoteChoice {
    Yes,
    No,
    Abstain,
}

/// Manipulation detection thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionThresholds {
    pub sybil_similarity_threshold: f64,
    pub coordination_correlation_threshold: f64,
    pub stake_concentration_threshold: f64,
    pub geographic_clustering_threshold: f64,
    pub temporal_window_minutes: i64,
    pub min_confidence_threshold: f64,
    pub max_stake_per_entity: f64,
    pub min_node_age_hours: i64,
}

impl Default for DetectionThresholds {
    fn default() -> Self {
        Self {
            sybil_similarity_threshold: 0.8,
            coordination_correlation_threshold: 0.9,
            stake_concentration_threshold: 0.33, // 33% of total stake
            geographic_clustering_threshold: 0.7,
            temporal_window_minutes: 60,
            min_confidence_threshold: 0.7,
            max_stake_per_entity: 0.1, // 10% max stake per entity
            min_node_age_hours: 24,
        }
    }
}

/// Anti-Manipulation Engine
pub struct AntiManipulationEngine {
    node_behaviors: Arc<RwLock<HashMap<String, NodeBehavior>>>,
    detection_thresholds: DetectionThresholds,
    alerts: Arc<RwLock<Vec<ManipulationAlert>>>,
    monitoring_enabled: Arc<RwLock<bool>>,
    total_stake: Arc<RwLock<u64>>,
    suspicious_entities: Arc<RwLock<HashSet<String>>>,
    whitelisted_entities: Arc<RwLock<HashSet<String>>>,
}

impl AntiManipulationEngine {
    /// Create new anti-manipulation engine
    pub fn new(thresholds: Option<DetectionThresholds>) -> Self {
        Self {
            node_behaviors: Arc::new(RwLock::new(HashMap::new())),
            detection_thresholds: thresholds.unwrap_or_default(),
            alerts: Arc::new(RwLock::new(Vec::new())),
            monitoring_enabled: Arc::new(RwLock::new(true)),
            total_stake: Arc::new(RwLock::new(0)),
            suspicious_entities: Arc::new(RwLock::new(HashSet::new())),
            whitelisted_entities: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    /// Register node behavior
    pub async fn register_node(&self, node_id: String, stake_amount: u64, geographic_location: Option<(f64, f64)>) -> Result<()> {
        let mut behaviors = self.node_behaviors.write().await;
        let mut total_stake = self.total_stake.write().await;

        let behavior = NodeBehavior {
            node_id: node_id.clone(),
            stake_amount,
            voting_history: VecDeque::new(),
            geographic_location,
            join_time: Utc::now(),
            last_activity: Utc::now(),
            reputation_score: 0.5, // Start with neutral reputation
            anomaly_score: 0.0,
        };

        behaviors.insert(node_id.clone(), behavior);
        *total_stake += stake_amount;

        info!("Registered node for manipulation monitoring: {}", node_id);
        
        // Check for immediate manipulation patterns
        self.detect_manipulation_patterns().await?;

        Ok(())
    }

    /// Record vote for manipulation detection
    pub async fn record_vote(&self, node_id: String, proposal_id: String, vote: VoteChoice, stake_weight: u64) -> Result<()> {
        let mut behaviors = self.node_behaviors.write().await;
        
        if let Some(behavior) = behaviors.get_mut(&node_id) {
            let vote_record = VoteRecord {
                proposal_id,
                vote,
                timestamp: Utc::now(),
                stake_weight,
            };

            behavior.voting_history.push_back(vote_record);
            behavior.last_activity = Utc::now();

            // Keep only last 1000 votes
            if behavior.voting_history.len() > 1000 {
                behavior.voting_history.pop_front();
            }
        }

        Ok(())
    }

    /// Detect manipulation patterns
    pub async fn detect_manipulation_patterns(&self) -> Result<Vec<ManipulationAlert>> {
        let monitoring_enabled = *self.monitoring_enabled.read().await;
        if !monitoring_enabled {
            return Ok(Vec::new());
        }

        let mut new_alerts = Vec::new();

        // Detect various manipulation patterns
        new_alerts.extend(self.detect_sybil_attacks().await?);
        new_alerts.extend(self.detect_stake_concentration().await?);
        new_alerts.extend(self.detect_geographic_clustering().await?);
        new_alerts.extend(self.detect_temporal_manipulation().await?);

        // Store alerts
        let mut alerts = self.alerts.write().await;
        alerts.extend(new_alerts.clone());

        // Keep only last 10000 alerts
        if alerts.len() > 10000 {
            let drain_count = alerts.len() - 10000;
            alerts.drain(0..drain_count);
        }

        Ok(new_alerts)
    }

    /// Detect Sybil attacks
    async fn detect_sybil_attacks(&self) -> Result<Vec<ManipulationAlert>> {
        let behaviors = self.node_behaviors.read().await;
        let mut alerts = Vec::new();

        let nodes: Vec<_> = behaviors.values().collect();
        
        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let similarity = self.calculate_node_similarity(nodes[i], nodes[j])?;
                
                if similarity > self.detection_thresholds.sybil_similarity_threshold {
                    let confidence = similarity;
                    let risk_score = similarity * 0.8 + 0.2;

                    alerts.push(ManipulationAlert {
                        alert_id: Uuid::new_v4().to_string(),
                        pattern: ManipulationPattern::SybilAttack,
                        severity: if confidence > 0.9 { AlertSeverity::Critical } else { AlertSeverity::High },
                        confidence,
                        description: format!(
                            "Potential Sybil attack detected between nodes {} and {} (similarity: {:.3})",
                            nodes[i].node_id, nodes[j].node_id, similarity
                        ),
                        affected_entities: vec![nodes[i].node_id.clone(), nodes[j].node_id.clone()],
                        detected_at: Utc::now(),
                        risk_score,
                        recommended_actions: vec![
                            "Investigate node ownership".to_string(),
                            "Require additional identity verification".to_string(),
                            "Monitor voting patterns closely".to_string(),
                        ],
                    });
                }
            }
        }

        Ok(alerts)
    }

    /// Calculate similarity between two nodes
    fn calculate_node_similarity(&self, node1: &NodeBehavior, node2: &NodeBehavior) -> Result<f64> {
        let mut similarity_factors = Vec::new();

        // Geographic similarity
        if let (Some(loc1), Some(loc2)) = (node1.geographic_location, node2.geographic_location) {
            let distance = ((loc1.0 - loc2.0).powi(2) + (loc1.1 - loc2.1).powi(2)).sqrt();
            let geo_similarity = if distance < 0.1 { 1.0 } else { 1.0 / (1.0 + distance) };
            similarity_factors.push(geo_similarity);
        }

        // Temporal similarity (join time)
        let time_diff = (node1.join_time - node2.join_time).num_seconds().abs() as f64;
        let temporal_similarity = if time_diff < 3600.0 { 1.0 } else { 1.0 / (1.0 + time_diff / 3600.0) };
        similarity_factors.push(temporal_similarity);

        // Stake similarity
        let stake_ratio = if node1.stake_amount > node2.stake_amount {
            node2.stake_amount as f64 / node1.stake_amount as f64
        } else {
            node1.stake_amount as f64 / node2.stake_amount as f64
        };
        similarity_factors.push(stake_ratio);

        // Voting pattern similarity
        let voting_similarity = self.calculate_voting_similarity(&node1.voting_history, &node2.voting_history);
        similarity_factors.push(voting_similarity);

        // Calculate weighted average
        if similarity_factors.is_empty() {
            Ok(0.0)
        } else {
            Ok(similarity_factors.iter().sum::<f64>() / similarity_factors.len() as f64)
        }
    }

    /// Calculate voting pattern similarity
    fn calculate_voting_similarity(&self, votes1: &VecDeque<VoteRecord>, votes2: &VecDeque<VoteRecord>) -> f64 {
        if votes1.is_empty() || votes2.is_empty() {
            return 0.0;
        }

        let mut common_proposals = 0;
        let mut matching_votes = 0;

        for vote1 in votes1.iter() {
            for vote2 in votes2.iter() {
                if vote1.proposal_id == vote2.proposal_id {
                    common_proposals += 1;
                    if vote1.vote == vote2.vote {
                        matching_votes += 1;
                    }
                    break;
                }
            }
        }

        if common_proposals == 0 {
            0.0
        } else {
            matching_votes as f64 / common_proposals as f64
        }
    }

    /// Detect stake concentration
    async fn detect_stake_concentration(&self) -> Result<Vec<ManipulationAlert>> {
        let behaviors = self.node_behaviors.read().await;
        let total_stake = *self.total_stake.read().await;
        let mut alerts = Vec::new();

        if total_stake == 0 {
            return Ok(alerts);
        }

        // Check individual stake concentration
        for behavior in behaviors.values() {
            let stake_ratio = behavior.stake_amount as f64 / total_stake as f64;
            
            if stake_ratio > self.detection_thresholds.stake_concentration_threshold {
                let confidence = (stake_ratio - self.detection_thresholds.stake_concentration_threshold) / 
                               (1.0 - self.detection_thresholds.stake_concentration_threshold);
                let risk_score = stake_ratio;

                alerts.push(ManipulationAlert {
                    alert_id: Uuid::new_v4().to_string(),
                    pattern: ManipulationPattern::StakeConcentration,
                    severity: if stake_ratio > 0.5 { AlertSeverity::Emergency } else { AlertSeverity::Critical },
                    confidence,
                    description: format!(
                        "Excessive stake concentration detected: node {} holds {:.1}% of total stake",
                        behavior.node_id, stake_ratio * 100.0
                    ),
                    affected_entities: vec![behavior.node_id.clone()],
                    detected_at: Utc::now(),
                    risk_score,
                    recommended_actions: vec![
                        "Implement stake caps".to_string(),
                        "Encourage stake distribution".to_string(),
                        "Monitor voting power concentration".to_string(),
                    ],
                });
            }
        }

        Ok(alerts)
    }

    /// Detect geographic clustering
    async fn detect_geographic_clustering(&self) -> Result<Vec<ManipulationAlert>> {
        let behaviors = self.node_behaviors.read().await;
        let mut alerts = Vec::new();

        let nodes_with_location: Vec<_> = behaviors.values()
            .filter(|b| b.geographic_location.is_some())
            .collect();

        if nodes_with_location.len() < 3 {
            return Ok(alerts);
        }

        // Calculate clustering coefficient
        let mut total_distance = 0.0;
        let mut pair_count = 0;

        for i in 0..nodes_with_location.len() {
            for j in (i + 1)..nodes_with_location.len() {
                if let (Some(loc1), Some(loc2)) = (
                    nodes_with_location[i].geographic_location,
                    nodes_with_location[j].geographic_location
                ) {
                    let distance = ((loc1.0 - loc2.0).powi(2) + (loc1.1 - loc2.1).powi(2)).sqrt();
                    total_distance += distance;
                    pair_count += 1;
                }
            }
        }

        if pair_count > 0 {
            let avg_distance = total_distance / pair_count as f64;
            let clustering_score = 1.0 / (1.0 + avg_distance);

            if clustering_score > self.detection_thresholds.geographic_clustering_threshold {
                let confidence = (clustering_score - self.detection_thresholds.geographic_clustering_threshold) /
                               (1.0 - self.detection_thresholds.geographic_clustering_threshold);
                let risk_score = clustering_score * 0.7 + 0.3;

                alerts.push(ManipulationAlert {
                    alert_id: Uuid::new_v4().to_string(),
                    pattern: ManipulationPattern::GeographicClustering,
                    severity: AlertSeverity::Medium,
                    confidence,
                    description: format!(
                        "Geographic clustering detected: average distance {:.2} degrees (clustering score: {:.3})",
                        avg_distance, clustering_score
                    ),
                    affected_entities: nodes_with_location.iter().map(|n| n.node_id.clone()).collect(),
                    detected_at: Utc::now(),
                    risk_score,
                    recommended_actions: vec![
                        "Encourage geographic distribution".to_string(),
                        "Implement location-based incentives".to_string(),
                        "Monitor for coordinated behavior".to_string(),
                    ],
                });
            }
        }

        Ok(alerts)
    }

    /// Detect temporal manipulation
    async fn detect_temporal_manipulation(&self) -> Result<Vec<ManipulationAlert>> {
        let behaviors = self.node_behaviors.read().await;
        let mut alerts = Vec::new();

        let min_age = Duration::hours(self.detection_thresholds.min_node_age_hours);
        let now = Utc::now();

        // Check for nodes that joined recently and immediately started high-stake activities
        for behavior in behaviors.values() {
            let age = now - behavior.join_time;

            if age < min_age && behavior.stake_amount > 0 {
                let confidence = 1.0 - (age.num_seconds() as f64 / min_age.num_seconds() as f64);
                let risk_score = confidence * 0.6 + 0.4;

                alerts.push(ManipulationAlert {
                    alert_id: Uuid::new_v4().to_string(),
                    pattern: ManipulationPattern::TemporalManipulation,
                    severity: AlertSeverity::Medium,
                    confidence,
                    description: format!(
                        "New node with immediate high stake detected: {} (age: {} hours, stake: {})",
                        behavior.node_id, age.num_hours(), behavior.stake_amount
                    ),
                    affected_entities: vec![behavior.node_id.clone()],
                    detected_at: Utc::now(),
                    risk_score,
                    recommended_actions: vec![
                        "Implement stake ramping periods".to_string(),
                        "Require identity verification for new high-stake nodes".to_string(),
                        "Monitor new node behavior closely".to_string(),
                    ],
                });
            }
        }

        Ok(alerts)
    }

    /// Add entity to whitelist
    pub async fn whitelist_entity(&self, entity_id: String) -> Result<()> {
        let mut whitelisted = self.whitelisted_entities.write().await;
        whitelisted.insert(entity_id.clone());
        info!("Added entity to whitelist: {}", entity_id);
        Ok(())
    }

    /// Add entity to suspicious list
    pub async fn flag_suspicious_entity(&self, entity_id: String) -> Result<()> {
        let mut suspicious = self.suspicious_entities.write().await;
        suspicious.insert(entity_id.clone());
        warn!("Flagged entity as suspicious: {}", entity_id);
        Ok(())
    }

    /// Get recent alerts
    pub async fn get_recent_alerts(&self, limit: usize) -> Result<Vec<ManipulationAlert>> {
        let alerts = self.alerts.read().await;
        let start_idx = if alerts.len() > limit {
            alerts.len() - limit
        } else {
            0
        };
        Ok(alerts[start_idx..].to_vec())
    }

    /// Enable/disable monitoring
    pub async fn set_monitoring_enabled(&self, enabled: bool) -> Result<()> {
        let mut monitoring = self.monitoring_enabled.write().await;
        *monitoring = enabled;
        info!("Anti-manipulation monitoring {}", if enabled { "enabled" } else { "disabled" });
        Ok(())
    }

    /// Get node behavior information
    pub async fn get_node_behavior(&self, node_id: &str) -> Result<Option<NodeBehavior>> {
        let behaviors = self.node_behaviors.read().await;
        Ok(behaviors.get(node_id).cloned())
    }

    /// Remove node from tracking
    pub async fn remove_node(&self, node_id: &str) -> Result<()> {
        let mut behaviors = self.node_behaviors.write().await;
        let mut total_stake = self.total_stake.write().await;
        
        if let Some(behavior) = behaviors.remove(node_id) {
            *total_stake -= behavior.stake_amount;
            info!("Removed node from manipulation monitoring: {}", node_id);
        }
        Ok(())
    }

    /// Get manipulation statistics
    pub async fn get_statistics(&self) -> Result<ManipulationStatistics> {
        let behaviors = self.node_behaviors.read().await;
        let alerts = self.alerts.read().await;
        let total_stake = *self.total_stake.read().await;

        let total_nodes = behaviors.len();
        let total_alerts = alerts.len();
        let critical_alerts = alerts.iter().filter(|a| a.severity >= AlertSeverity::Critical).count();
        
        let avg_reputation = if total_nodes > 0 {
            behaviors.values().map(|b| b.reputation_score).sum::<f64>() / total_nodes as f64
        } else {
            0.0
        };

        Ok(ManipulationStatistics {
            total_nodes,
            total_stake,
            total_alerts,
            critical_alerts,
            average_reputation: avg_reputation,
            monitoring_enabled: *self.monitoring_enabled.read().await,
        })
    }
}

/// Manipulation monitoring statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManipulationStatistics {
    pub total_nodes: usize,
    pub total_stake: u64,
    pub total_alerts: usize,
    pub critical_alerts: usize,
    pub average_reputation: f64,
    pub monitoring_enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_anti_manipulation_engine() {
        let engine = AntiManipulationEngine::new(None);
        
        // Test node registration
        assert!(engine.register_node(
            "node1".to_string(),
            1000,
            Some((40.7128, -74.0060))
        ).await.is_ok());
        
        // Test vote recording
        assert!(engine.record_vote(
            "node1".to_string(),
            "proposal1".to_string(),
            VoteChoice::Yes,
            1000
        ).await.is_ok());
        
        // Test statistics
        let stats = engine.get_statistics().await.unwrap();
        assert_eq!(stats.total_nodes, 1);
        assert_eq!(stats.total_stake, 1000);
    }

    #[tokio::test]
    async fn test_stake_concentration_detection() {
        let mut thresholds = DetectionThresholds::default();
        thresholds.stake_concentration_threshold = 0.5; // 50% threshold
        
        let engine = AntiManipulationEngine::new(Some(thresholds));
        
        // Register node with high stake concentration
        assert!(engine.register_node(
            "node1".to_string(),
            600, // 60% of 1000 total stake
            None
        ).await.is_ok());
        
        // This should trigger stake concentration alert
        let alerts = engine.detect_manipulation_patterns().await.unwrap();
        assert!(!alerts.is_empty());
        assert!(alerts.iter().any(|a| matches!(a.pattern, ManipulationPattern::StakeConcentration)));
    }

    #[test]
    fn test_voting_similarity() {
        let engine = AntiManipulationEngine::new(None);
        
        let mut votes1 = VecDeque::new();
        let mut votes2 = VecDeque::new();
        
        // Add identical votes
        votes1.push_back(VoteRecord {
            proposal_id: "prop1".to_string(),
            vote: VoteChoice::Yes,
            timestamp: Utc::now(),
            stake_weight: 100,
        });
        
        votes2.push_back(VoteRecord {
            proposal_id: "prop1".to_string(),
            vote: VoteChoice::Yes,
            timestamp: Utc::now(),
            stake_weight: 200,
        });
        
        let similarity = engine.calculate_voting_similarity(&votes1, &votes2);
        assert_eq!(similarity, 1.0); // Perfect match
    }
}
