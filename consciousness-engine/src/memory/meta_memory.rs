//! Meta-Memory - Thinking about memories and memory processes

use crate::{
    types::*,
    error::{ConsciousnessError, ConsciousnessResult},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tracing::{debug, info};

/// Meta-memory system for reflecting on memory processes and content
#[derive(Debug, Clone)]
pub struct MetaMemorySystem {
    memory_reflections: HashMap<String, MemoryReflection>,
    memory_strategies: HashMap<String, MemoryStrategy>,
    memory_confidence: MemoryConfidenceTracker,
    reflection_history: Vec<MetaMemoryEvent>,
}

impl MetaMemorySystem {
    /// Create a new meta-memory system
    pub fn new() -> Self {
        info!("Initializing Meta-Memory System");
        
        Self {
            memory_reflections: HashMap::new(),
            memory_strategies: HashMap::new(),
            memory_confidence: MemoryConfidenceTracker::new(),
            reflection_history: Vec::new(),
        }
    }
    
    /// Reflect on a specific memory or memory process
    pub async fn reflect_on_memory(
        &mut self,
        memory_id: &str,
        memory_content: &str,
        memory_type: MemoryType,
        reflection_context: &ReflectionContext,
    ) -> ConsciousnessResult<MemoryReflection> {
        debug!("Reflecting on memory: {}", memory_id);
        
        // Analyze the memory content
        let content_analysis = self.analyze_memory_content(memory_content, &memory_type).await?;
        
        // Assess memory reliability
        let reliability_assessment = self.assess_memory_reliability(
            memory_id,
            memory_content,
            &memory_type,
            reflection_context,
        ).await?;
        
        // Evaluate memory significance
        let significance_evaluation = self.evaluate_memory_significance(
            memory_content,
            &memory_type,
            reflection_context,
        ).await?;
        
        // Generate insights about the memory
        let memory_insights = self.generate_memory_insights(
            &content_analysis,
            &reliability_assessment,
            &significance_evaluation,
        ).await?;
        
        // Create reflection
        let reflection = MemoryReflection {
            reflection_id: Uuid::new_v4(),
            memory_id: memory_id.to_string(),
            memory_type,
            content_analysis,
            reliability_assessment,
            significance_evaluation,
            insights: memory_insights,
            reflection_confidence: self.calculate_reflection_confidence(&memory_insights),
            created_at: Utc::now(),
            last_updated: Utc::now(),
        };
        
        // Store reflection
        self.memory_reflections.insert(memory_id.to_string(), reflection.clone());
        
        // Record meta-memory event
        self.record_meta_memory_event(MetaMemoryEvent {
            event_id: Uuid::new_v4(),
            event_type: MetaMemoryEventType::MemoryReflection,
            memory_id: memory_id.to_string(),
            description: format!("Reflected on memory: {}", memory_id),
            confidence_level: reflection.reflection_confidence,
            timestamp: Utc::now(),
        });
        
        info!("Memory reflection completed for: {}", memory_id);
        
        Ok(reflection)
    }
    
    /// Get memory confidence for a specific memory
    pub async fn get_memory_confidence(
        &self,
        memory_id: &str,
    ) -> ConsciousnessResult<MemoryConfidenceReport> {
        self.memory_confidence.get_confidence_report(memory_id).await
    }
    
    /// Get reflection for a specific memory
    pub async fn get_memory_reflection(
        &self,
        memory_id: &str,
    ) -> ConsciousnessResult<Option<MemoryReflection>> {
        Ok(self.memory_reflections.get(memory_id).cloned())
    }
    
    /// Get meta-memory statistics
    pub fn get_meta_memory_statistics(&self) -> MetaMemoryStatistics {
        let total_reflections = self.memory_reflections.len();
        let total_strategies = self.memory_strategies.len();
        let total_events = self.reflection_history.len();
        
        let event_type_counts = self.reflection_history.iter()
            .fold(HashMap::new(), |mut acc, event| {
                *acc.entry(event.event_type.clone()).or_insert(0) += 1;
                acc
            });
        
        let average_confidence = if total_events > 0 {
            self.reflection_history.iter()
                .map(|e| e.confidence_level)
                .sum::<f64>() / total_events as f64
        } else {
            0.0
        };
        
        MetaMemoryStatistics {
            total_reflections,
            total_strategies,
            total_events,
            event_type_counts,
            average_confidence,
            recent_activity: self.reflection_history.iter()
                .rev()
                .take(10)
                .cloned()
                .collect(),
        }
    }
    
    // Private helper methods
    
    async fn analyze_memory_content(
        &self,
        content: &str,
        memory_type: &MemoryType,
    ) -> ConsciousnessResult<MemoryContentAnalysis> {
        // Analyze content complexity
        let complexity = self.assess_content_complexity(content);
        
        // Analyze content structure
        let structure = self.analyze_content_structure(content, memory_type);
        
        // Analyze content themes
        let themes = self.extract_content_themes(content);
        
        // Analyze emotional content
        let emotional_content = self.analyze_emotional_content(content);
        
        Ok(MemoryContentAnalysis {
            complexity,
            structure,
            themes,
            emotional_content,
            content_length: content.len(),
            analysis_confidence: 0.85,
        })
    }
    
    async fn assess_memory_reliability(
        &self,
        memory_id: &str,
        content: &str,
        memory_type: &MemoryType,
        context: &ReflectionContext,
    ) -> ConsciousnessResult<MemoryReliabilityAssessment> {
        // Check for consistency with other memories
        let consistency_score = self.check_memory_consistency(memory_id, content).await?;
        
        // Assess source credibility
        let source_credibility = self.assess_source_credibility(context);
        
        // Check for potential biases
        let bias_assessment = self.assess_potential_biases(content, memory_type);
        
        // Calculate overall reliability
        let overall_reliability = (consistency_score + source_credibility + bias_assessment.reliability_impact) / 3.0;
        
        Ok(MemoryReliabilityAssessment {
            consistency_score,
            source_credibility,
            bias_assessment,
            overall_reliability,
            reliability_factors: vec![
                "consistency".to_string(),
                "source_credibility".to_string(),
                "bias_assessment".to_string(),
            ],
        })
    }
    
    async fn evaluate_memory_significance(
        &self,
        content: &str,
        memory_type: &MemoryType,
        context: &ReflectionContext,
    ) -> ConsciousnessResult<MemorySignificanceEvaluation> {
        // Assess personal relevance
        let personal_relevance = self.assess_personal_relevance(content, context);
        
        // Assess learning value
        let learning_value = self.assess_learning_value(content, memory_type);
        
        // Assess emotional significance
        let emotional_significance = self.assess_emotional_significance(content);
        
        // Assess future utility
        let future_utility = self.assess_future_utility(content, memory_type);
        
        // Calculate overall significance
        let overall_significance = (personal_relevance + learning_value + emotional_significance + future_utility) / 4.0;
        
        Ok(MemorySignificanceEvaluation {
            personal_relevance,
            learning_value,
            emotional_significance,
            future_utility,
            overall_significance,
            significance_factors: vec![
                "personal_relevance".to_string(),
                "learning_value".to_string(),
                "emotional_significance".to_string(),
                "future_utility".to_string(),
            ],
        })
    }
    
    async fn generate_memory_insights(
        &self,
        content_analysis: &MemoryContentAnalysis,
        reliability_assessment: &MemoryReliabilityAssessment,
        significance_evaluation: &MemorySignificanceEvaluation,
    ) -> ConsciousnessResult<Vec<MemoryInsight>> {
        let mut insights = Vec::new();
        
        // Content-based insights
        if content_analysis.complexity > 0.8 {
            insights.push(MemoryInsight {
                insight_type: MemoryInsightType::ContentComplexity,
                description: "This memory contains complex information that may require additional processing".to_string(),
                confidence: content_analysis.analysis_confidence,
                actionable_recommendations: vec![
                    "Consider breaking down into smaller components".to_string(),
                    "Create additional associative links".to_string(),
                ],
            });
        }
        
        // Reliability-based insights
        if reliability_assessment.overall_reliability < 0.6 {
            insights.push(MemoryInsight {
                insight_type: MemoryInsightType::ReliabilityConcern,
                description: "This memory has reliability concerns that should be noted".to_string(),
                confidence: 0.9,
                actionable_recommendations: vec![
                    "Seek additional verification".to_string(),
                    "Mark as potentially unreliable".to_string(),
                ],
            });
        }
        
        // Significance-based insights
        if significance_evaluation.overall_significance > 0.8 {
            insights.push(MemoryInsight {
                insight_type: MemoryInsightType::HighSignificance,
                description: "This memory is highly significant and should be prioritized for retention".to_string(),
                confidence: 0.85,
                actionable_recommendations: vec![
                    "Strengthen encoding through rehearsal".to_string(),
                    "Create multiple retrieval pathways".to_string(),
                ],
            });
        }
        
        Ok(insights)
    }
    
    fn record_meta_memory_event(&mut self, event: MetaMemoryEvent) {
        self.reflection_history.push(event);
        
        // Limit history size
        if self.reflection_history.len() > 1000 {
            self.reflection_history.remove(0);
        }
    }
    
    fn calculate_reflection_confidence(&self, insights: &[MemoryInsight]) -> f64 {
        if insights.is_empty() {
            return 0.5;
        }
        
        insights.iter().map(|i| i.confidence).sum::<f64>() / insights.len() as f64
    }
    
    // Placeholder implementations for helper methods
    fn assess_content_complexity(&self, _content: &str) -> f64 { 0.5 }
    fn analyze_content_structure(&self, _content: &str, _memory_type: &MemoryType) -> String { "structured".to_string() }
    fn extract_content_themes(&self, _content: &str) -> Vec<String> { vec!["general".to_string()] }
    fn analyze_emotional_content(&self, _content: &str) -> f64 { 0.5 }
    async fn check_memory_consistency(&self, _memory_id: &str, _content: &str) -> ConsciousnessResult<f64> { Ok(0.8) }
    fn assess_source_credibility(&self, _context: &ReflectionContext) -> f64 { 0.8 }
    fn assess_potential_biases(&self, _content: &str, _memory_type: &MemoryType) -> BiasAssessment { 
        BiasAssessment { reliability_impact: 0.8, detected_biases: vec![] } 
    }
    fn assess_personal_relevance(&self, _content: &str, _context: &ReflectionContext) -> f64 { 0.7 }
    fn assess_learning_value(&self, _content: &str, _memory_type: &MemoryType) -> f64 { 0.6 }
    fn assess_emotional_significance(&self, _content: &str) -> f64 { 0.5 }
    fn assess_future_utility(&self, _content: &str, _memory_type: &MemoryType) -> f64 { 0.6 }
}

#[derive(Debug, Clone)]
struct MemoryConfidenceTracker {
    confidence_records: HashMap<String, Vec<ConfidenceRecord>>,
}

impl MemoryConfidenceTracker {
    fn new() -> Self {
        Self {
            confidence_records: HashMap::new(),
        }
    }
    
    async fn get_confidence_report(&self, memory_id: &str) -> ConsciousnessResult<MemoryConfidenceReport> {
        let records = self.confidence_records.get(memory_id).cloned().unwrap_or_default();
        
        let current_confidence = records.last().map(|r| r.confidence_level).unwrap_or(0.5);
        let confidence_trend = self.calculate_confidence_trend(&records);
        let confidence_stability = self.calculate_confidence_stability(&records);
        
        Ok(MemoryConfidenceReport {
            memory_id: memory_id.to_string(),
            current_confidence,
            confidence_trend,
            confidence_stability,
            confidence_history: records,
        })
    }
    
    fn calculate_confidence_trend(&self, records: &[ConfidenceRecord]) -> f64 {
        if records.len() < 2 {
            return 0.0;
        }
        
        let first_half = &records[..records.len()/2];
        let second_half = &records[records.len()/2..];
        
        let first_avg = first_half.iter().map(|r| r.confidence_level).sum::<f64>() / first_half.len() as f64;
        let second_avg = second_half.iter().map(|r| r.confidence_level).sum::<f64>() / second_half.len() as f64;
        
        second_avg - first_avg
    }
    
    fn calculate_confidence_stability(&self, records: &[ConfidenceRecord]) -> f64 {
        if records.len() < 2 {
            return 1.0;
        }
        
        let mean = records.iter().map(|r| r.confidence_level).sum::<f64>() / records.len() as f64;
        let variance = records.iter()
            .map(|r| (r.confidence_level - mean).powi(2))
            .sum::<f64>() / records.len() as f64;
        
        1.0 - variance.sqrt()
    }
}

// Data structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryReflection {
    pub reflection_id: Uuid,
    pub memory_id: String,
    pub memory_type: MemoryType,
    pub content_analysis: MemoryContentAnalysis,
    pub reliability_assessment: MemoryReliabilityAssessment,
    pub significance_evaluation: MemorySignificanceEvaluation,
    pub insights: Vec<MemoryInsight>,
    pub reflection_confidence: f64,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryType {
    Episodic,
    Semantic,
    Procedural,
    Working,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryContentAnalysis {
    pub complexity: f64,
    pub structure: String,
    pub themes: Vec<String>,
    pub emotional_content: f64,
    pub content_length: usize,
    pub analysis_confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryReliabilityAssessment {
    pub consistency_score: f64,
    pub source_credibility: f64,
    pub bias_assessment: BiasAssessment,
    pub overall_reliability: f64,
    pub reliability_factors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiasAssessment {
    pub reliability_impact: f64,
    pub detected_biases: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySignificanceEvaluation {
    pub personal_relevance: f64,
    pub learning_value: f64,
    pub emotional_significance: f64,
    pub future_utility: f64,
    pub overall_significance: f64,
    pub significance_factors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInsight {
    pub insight_type: MemoryInsightType,
    pub description: String,
    pub confidence: f64,
    pub actionable_recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryInsightType {
    ContentComplexity,
    ReliabilityConcern,
    HighSignificance,
    LearningOpportunity,
    EmotionalRelevance,
    StrategicImportance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflectionContext {
    pub reflection_trigger: String,
    pub current_goals: Vec<String>,
    pub emotional_state: String,
    pub cognitive_load: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaMemoryEvent {
    pub event_id: Uuid,
    pub event_type: MetaMemoryEventType,
    pub memory_id: String,
    pub description: String,
    pub confidence_level: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MetaMemoryEventType {
    MemoryReflection,
    FormationMonitoring,
    RetrievalMonitoring,
    StrategyDevelopment,
    SystemEvaluation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStrategy {
    pub strategy_id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfidenceReport {
    pub memory_id: String,
    pub current_confidence: f64,
    pub confidence_trend: f64,
    pub confidence_stability: f64,
    pub confidence_history: Vec<ConfidenceRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceRecord {
    pub timestamp: DateTime<Utc>,
    pub confidence_level: f64,
    pub confidence_type: ConfidenceType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfidenceType {
    Formation,
    Retrieval,
    Consolidation,
    Reflection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaMemoryStatistics {
    pub total_reflections: usize,
    pub total_strategies: usize,
    pub total_events: usize,
    pub event_type_counts: HashMap<MetaMemoryEventType, u32>,
    pub average_confidence: f64,
    pub recent_activity: Vec<MetaMemoryEvent>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_reflection() {
        let mut meta_memory = MetaMemorySystem::new();
        
        let reflection_context = ReflectionContext {
            reflection_trigger: "user_query".to_string(),
            current_goals: vec!["learning".to_string()],
            emotional_state: "curious".to_string(),
            cognitive_load: 0.6,
        };
        
        let reflection = meta_memory.reflect_on_memory(
            "test_memory_1",
            "This is a test memory content",
            MemoryType::Episodic,
            &reflection_context,
        ).await.unwrap();
        
        assert_eq!(reflection.memory_id, "test_memory_1");
        assert!(matches!(reflection.memory_type, MemoryType::Episodic));
        assert!(!reflection.insights.is_empty());
    }

    #[tokio::test]
    async fn test_meta_memory_statistics() {
        let mut meta_memory = MetaMemorySystem::new();
        
        let reflection_context = ReflectionContext {
            reflection_trigger: "test".to_string(),
            current_goals: vec![],
            emotional_state: "neutral".to_string(),
            cognitive_load: 0.5,
        };
        
        meta_memory.reflect_on_memory(
            "memory1",
            "content1",
            MemoryType::Episodic,
            &reflection_context,
        ).await.unwrap();
        
        let stats = meta_memory.get_meta_memory_statistics();
        
        assert_eq!(stats.total_reflections, 1);
        assert_eq!(stats.total_events, 1);
        assert!(stats.average_confidence > 0.0);
    }
}