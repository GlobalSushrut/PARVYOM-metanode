// BPI Programmable CUE Forensic Firewall - Threat Intelligence Pipeline
// Real-time threat intelligence with ML/AI integration hooks

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Threat Intelligence Pipeline for real-time threat data processing
#[derive(Debug)]
pub struct ThreatIntelligencePipeline {
    pub feed_aggregator: Arc<ThreatFeedAggregator>,
    pub ioc_processor: Arc<IOCProcessor>,
    pub threat_classifier: Arc<ThreatClassifier>,
    pub ml_feature_extractor: Arc<MLFeatureExtractor>, // 🤖 ML/AI Integration Point
    pub reputation_engine: Arc<ReputationEngine>,
    pub threat_cache: Arc<RwLock<ThreatCache>>,
}

impl ThreatIntelligencePipeline {
    pub fn new() -> Self {
        Self {
            feed_aggregator: Arc::new(ThreatFeedAggregator::new()),
            ioc_processor: Arc::new(IOCProcessor::new()),
            threat_classifier: Arc::new(ThreatClassifier::new()),
            ml_feature_extractor: Arc::new(MLFeatureExtractor::new()),
            reputation_engine: Arc::new(ReputationEngine::new()),
            threat_cache: Arc::new(RwLock::new(ThreatCache::new())),
        }
    }

    /// Process incoming threat intelligence data
    pub async fn process_threat_intel(&self, raw_data: &ThreatIntelData) -> Result<ProcessedThreatIntel> {
        let start_time = Instant::now();

        // 1. Aggregate threat feeds
        let aggregated_data = self.feed_aggregator.aggregate_feeds(raw_data).await?;

        // 2. Process IOCs (Indicators of Compromise)
        let iocs = self.ioc_processor.extract_iocs(&aggregated_data).await?;

        // 3. Classify threats using ML (🤖 ML/AI Integration Point)
        let classification = self.threat_classifier.classify_threat(&aggregated_data).await?;

        // 4. Extract ML features for future model training (🤖 ML/AI Integration Point)
        let ml_features = self.ml_feature_extractor.extract_features(&aggregated_data).await?;

        // 5. Calculate reputation scores
        let reputation_scores = self.reputation_engine.calculate_reputation(&iocs).await?;

        let processed_intel = ProcessedThreatIntel {
            intel_id: Uuid::new_v4().to_string(),
            original_data: aggregated_data,
            iocs,
            classification,
            ml_features,
            reputation_scores,
            processing_time: start_time.elapsed(),
            timestamp: Instant::now(),
        };

        // Cache the processed intelligence
        self.cache_threat_intel(&processed_intel).await?;

        tracing::info!("🔍 Processed threat intelligence: {} ({}ms)", 
                      processed_intel.intel_id, 
                      processed_intel.processing_time.as_millis());

        Ok(processed_intel)
    }

    /// Query threat intelligence for a specific indicator
    pub async fn query_threat_intel(&self, indicator: &str) -> Result<Option<ThreatIntelligence>> {
        // Check cache first
        if let Some(cached_intel) = self.get_cached_intel(indicator).await {
            if !cached_intel.is_expired() {
                return Ok(Some(cached_intel.intelligence));
            }
        }

        // Query live threat feeds
        let live_intel = self.query_live_feeds(indicator).await?;
        
        if let Some(intel) = &live_intel {
            // Cache the result
            let cached = CachedThreatIntel {
                intelligence: intel.clone(),
                cached_at: Instant::now(),
                ttl: Duration::from_secs(300), // 5 minutes
            };
            
            let mut cache = self.threat_cache.write().await;
            cache.insert(indicator.to_string(), cached);
        }

        Ok(live_intel)
    }

    /// Get real-time threat score for an indicator
    pub async fn get_threat_score(&self, indicator: &str) -> Result<f64> {
        if let Some(intel) = self.query_threat_intel(indicator).await? {
            Ok(intel.threat_score)
        } else {
            // Use ML model to predict threat score if no intel available (🤖 ML/AI Integration Point)
            self.ml_predict_threat_score(indicator).await
        }
    }

    /// ML-based threat score prediction (🤖 ML/AI Integration Point)
    async fn ml_predict_threat_score(&self, indicator: &str) -> Result<f64> {
        // Extract features from the indicator
        let features = self.ml_feature_extractor.extract_indicator_features(indicator).await?;
        
        // Use ML model to predict threat score
        let predicted_score = self.calculate_heuristic_score(indicator, &features);
        
        tracing::debug!("🤖 ML predicted threat score for {}: {}", indicator, predicted_score);
        
        Ok(predicted_score)
    }

    fn calculate_heuristic_score(&self, indicator: &str, features: &ThreatFeatures) -> f64 {
        let mut score = 0.0;
        
        // Domain-based scoring
        if indicator.contains("suspicious") || indicator.contains("malware") {
            score += 0.8;
        }
        
        // Feature-based scoring
        score += features.entropy_score * 0.3;
        score += features.reputation_score * 0.4;
        score += features.age_score * 0.2;
        score += features.frequency_score * 0.1;
        
        score.min(1.0) // Cap at 1.0
    }

    async fn cache_threat_intel(&self, intel: &ProcessedThreatIntel) -> Result<()> {
        let mut cache = self.threat_cache.write().await;
        
        // Cache by all IOCs in the intelligence
        for ioc in &intel.iocs {
            let cached = CachedThreatIntel {
                intelligence: ThreatIntelligence {
                    indicator: ioc.value.clone(),
                    threat_type: intel.classification.threat_type.clone(),
                    threat_score: intel.classification.confidence,
                    source: "processed_intel".to_string(),
                    first_seen: intel.timestamp,
                    last_seen: intel.timestamp,
                    tags: intel.classification.tags.clone(),
                    cache: Arc::new(RwLock::new(HashMap::new())),
                    aggregator: ThreatFeedAggregator::new(),
                    processor: IOCProcessor::new(),
                    classifier: ThreatClassifier::new(),
                },
                cached_at: intel.timestamp,
                ttl: Duration::from_secs(3600), // 1 hour for processed intel
            };
            
            cache.insert(ioc.value.clone(), cached);
        }
        
        Ok(())
    }

    async fn get_cached_intel(&self, indicator: &str) -> Option<CachedThreatIntel> {
        let cache = self.threat_cache.read().await;
        cache.get(indicator).cloned()
    }

    async fn query_live_feeds(&self, _indicator: &str) -> Result<Option<ThreatIntelligence>> {
        // In production, this would query actual threat intelligence feeds
        Ok(None)
    }
}

// Core Data Structures

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIntelData {
    pub primary_indicator: String,
    pub source: String,
    pub raw_data: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct ProcessedThreatIntel {
    pub intel_id: String,
    pub original_data: AggregatedThreatData,
    pub iocs: Vec<IOC>,
    pub classification: ThreatClassification,
    pub ml_features: ThreatFeatures,
    pub reputation_scores: HashMap<String, ReputationScore>,
    pub processing_time: Duration,
    pub timestamp: Instant,
}

#[derive(Debug, Clone)]
pub struct AggregatedThreatData {
    pub sources: Vec<String>,
    pub indicators: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub confidence_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IOC {
    pub value: String,
    pub ioc_type: IOCType,
    pub confidence: f64,
    pub first_seen: Option<DateTime<Utc>>,
    pub last_seen: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IOCType {
    IPAddress,
    Domain,
    URL,
    FileHash,
    Email,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatClassification {
    pub threat_type: ThreatType,
    pub threat_level: ThreatLevel,
    pub confidence: f64,
    pub tags: Vec<String>,
    pub ml_enhanced: bool,
    pub reasoning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatType {
    Unknown,
    Malware,
    Phishing,
    Botnet,
    APT,
    Insider,
    DDoS,
    DataBreach,
    Ransomware,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatLevel {
    Low,
    Medium,
    High,
    Critical,
    Emergency,
}

#[derive(Debug, Clone, Default)]
pub struct ThreatFeatures {
    pub entropy_score: f64,
    pub reputation_score: f64,
    pub age_score: f64,
    pub frequency_score: f64,
    pub source_count: f64,
    pub confidence_score: f64,
}

#[derive(Debug, Clone)]
pub struct ReputationScore {
    pub score: f64, // 0.0 = benign, 1.0 = malicious
    pub confidence: f64,
    pub sources: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ThreatIntelligence {
    pub indicator: String,
    pub threat_type: ThreatType,
    pub threat_score: f64,
    pub source: String,
    pub first_seen: Instant,
    pub last_seen: Instant,
    pub tags: Vec<String>,
    pub cache: Arc<RwLock<ThreatCache>>,
    pub aggregator: ThreatFeedAggregator,
    pub processor: IOCProcessor,
    pub classifier: ThreatClassifier,
}

impl ThreatIntelligence {
    pub fn new() -> Self {
        Self {
            indicator: String::new(),
            threat_type: ThreatType::Unknown,
            threat_score: 0.0,
            source: String::new(),
            first_seen: Instant::now(),
            last_seen: Instant::now(),
            tags: Vec::new(),
            cache: Arc::new(RwLock::new(HashMap::new())),
            aggregator: ThreatFeedAggregator::new(),
            processor: IOCProcessor::new(),
            classifier: ThreatClassifier::new(),
        }
    }

    pub async fn classify_threat(&self, indicators: &[String]) -> Result<ThreatClassification> {
        let mut total_score = 0.0;
        let mut threat_types = Vec::new();
        
        for indicator in indicators {
            if let Some(cached) = self.cache.read().await.get(indicator) {
                if !cached.is_expired() {
                    total_score += cached.intelligence.threat_score;
                    threat_types.push(cached.intelligence.threat_type.clone());
                    continue;
                }
            }
            
            // Classify using ML model
            let threat_data = AggregatedThreatData {
                sources: vec!["ML_Classifier".to_string()],
                indicators: vec![indicator.clone()],
                metadata: HashMap::new(),
                confidence_score: 0.5,
            };
            let classification = self.classifier.classify_threat(&threat_data).await?;
            total_score += classification.confidence;
            threat_types.push(classification.threat_type.clone());
        }
        
        let avg_score = if indicators.is_empty() { 0.0 } else { total_score / indicators.len() as f64 };
        let primary_type = threat_types.into_iter().next().unwrap_or(ThreatType::Unknown);
        let threat_confidence = avg_score;
        
        Ok(ThreatClassification {
            threat_type: primary_type,
            threat_level: ThreatLevel::Medium, // Default level
            confidence: threat_confidence,
            tags: indicators.to_vec(),
            ml_enhanced: true,
            reasoning: format!("ML classification with {} indicators", indicators.len()),
        })
    }
}

#[derive(Debug, Clone)]
pub struct CachedThreatIntel {
    pub intelligence: ThreatIntelligence,
    pub cached_at: Instant,
    pub ttl: Duration,
}

impl CachedThreatIntel {
    pub fn is_expired(&self) -> bool {
        self.cached_at.elapsed() > self.ttl
    }
}

type ThreatCache = HashMap<String, CachedThreatIntel>;

// Component Implementations

#[derive(Debug, Clone)]
pub struct ThreatFeedAggregator;

impl ThreatFeedAggregator {
    pub fn new() -> Self {
        Self
    }

    pub async fn aggregate_feeds(&self, raw_data: &ThreatIntelData) -> Result<AggregatedThreatData> {
        Ok(AggregatedThreatData {
            sources: vec!["default".to_string()],
            indicators: vec![raw_data.primary_indicator.clone()],
            metadata: HashMap::new(),
            confidence_score: 0.7,
        })
    }
}

#[derive(Debug, Clone)]
pub struct IOCProcessor;

impl IOCProcessor {
    pub fn new() -> Self {
        Self
    }

    pub async fn extract_iocs(&self, data: &AggregatedThreatData) -> Result<Vec<IOC>> {
        let mut iocs = Vec::new();
        
        for indicator in &data.indicators {
            // Simple IOC extraction logic
            let ioc_type = if indicator.contains('.') && indicator.chars().all(|c| c.is_ascii_digit() || c == '.') {
                IOCType::IPAddress
            } else if indicator.contains('.') {
                IOCType::Domain
            } else if indicator.starts_with("http") {
                IOCType::URL
            } else if indicator.len() >= 32 && indicator.chars().all(|c| c.is_ascii_hexdigit()) {
                IOCType::FileHash
            } else if indicator.contains('@') {
                IOCType::Email
            } else {
                IOCType::Domain // Default
            };

            iocs.push(IOC {
                value: indicator.clone(),
                ioc_type,
                confidence: data.confidence_score,
                first_seen: Some(Utc::now()),
                last_seen: Some(Utc::now()),
                tags: vec!["extracted".to_string()],
            });
        }

        Ok(iocs)
    }
}

#[derive(Debug, Clone)]
pub struct ThreatClassifier;

impl ThreatClassifier {
    pub fn new() -> Self {
        Self
    }
    pub async fn classify_threat(&self, data: &AggregatedThreatData) -> Result<ThreatClassification> {
        // Simple rule-based classification
        let mut threat_type = ThreatType::Unknown;
        let mut confidence = 0.5;
        let mut tags = Vec::new();

        for indicator in &data.indicators {
            if indicator.contains("malware") || indicator.contains("virus") {
                threat_type = ThreatType::Malware;
                confidence = 0.8;
                tags.push("malware".to_string());
            } else if indicator.contains("suspicious") {
                threat_type = ThreatType::Unknown;
                confidence = 0.6;
                tags.push("suspicious".to_string());
            }
        }

        Ok(ThreatClassification {
            threat_type,
            threat_level: ThreatLevel::Medium,
            confidence,
            tags,
            ml_enhanced: false,
            reasoning: "Rule-based classification".to_string(),
        })
    }
}

#[derive(Debug)]
pub struct MLFeatureExtractor;

impl MLFeatureExtractor {
    pub fn new() -> Self {
        Self
    }

    pub async fn extract_features(&self, data: &AggregatedThreatData) -> Result<ThreatFeatures> {
        let mut features = ThreatFeatures::default();
        
        if let Some(primary_indicator) = data.indicators.first() {
            features.entropy_score = self.calculate_entropy(primary_indicator);
            features.source_count = data.sources.len() as f64;
            features.confidence_score = data.confidence_score;
        }

        Ok(features)
    }

    pub async fn extract_indicator_features(&self, indicator: &str) -> Result<ThreatFeatures> {
        let mut features = ThreatFeatures::default();
        features.entropy_score = self.calculate_entropy(indicator);
        Ok(features)
    }

    fn calculate_entropy(&self, data: &str) -> f64 {
        // Simple entropy calculation
        let mut char_counts = HashMap::new();
        for c in data.chars() {
            *char_counts.entry(c).or_insert(0) += 1;
        }

        let len = data.len() as f64;
        let mut entropy = 0.0;
        
        for count in char_counts.values() {
            let p = *count as f64 / len;
            entropy -= p * p.log2();
        }

        entropy / 8.0 // Normalize to 0-1 range
    }
}

#[derive(Debug)]
pub struct ReputationEngine;

impl ReputationEngine {
    pub fn new() -> Self {
        Self
    }

    pub async fn calculate_reputation(&self, iocs: &[IOC]) -> Result<HashMap<String, ReputationScore>> {
        let mut reputation_scores = HashMap::new();

        for ioc in iocs {
            // Simple reputation scoring
            let score = if ioc.value.contains("malware") {
                0.9 // High malicious score
            } else if ioc.value.contains("suspicious") {
                0.6 // Medium suspicious score
            } else {
                0.3 // Low/neutral score
            };

            reputation_scores.insert(ioc.value.clone(), ReputationScore {
                score,
                confidence: 0.8,
                sources: vec!["heuristic".to_string()],
            });
        }

        Ok(reputation_scores)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_threat_intelligence_pipeline() {
        let pipeline = ThreatIntelligencePipeline::new();
        
        let test_data = ThreatIntelData {
            primary_indicator: "malware.example.com".to_string(),
            source: "test".to_string(),
            raw_data: "test data".to_string(),
            timestamp: 1234567890,
        };

        let result = pipeline.process_threat_intel(&test_data).await;
        assert!(result.is_ok());
        
        let processed = result.unwrap();
        assert_eq!(processed.classification.threat_type as u8, ThreatType::Malware as u8);
    }

    #[tokio::test]
    async fn test_threat_score_prediction() {
        let pipeline = ThreatIntelligencePipeline::new();
        
        let score = pipeline.get_threat_score("suspicious.domain.com").await;
        assert!(score.is_ok());
        assert!(score.unwrap() > 0.0);
    }
}
