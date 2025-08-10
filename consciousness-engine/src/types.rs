//! Type definitions for the Consciousness Engine
//! 
//! This module contains all the data structures and types used throughout
//! the consciousness engine for representing states, responses, and configurations.

use std::time::Duration;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Configuration for the Consciousness Engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessConfig {
    /// Minimum consciousness quality threshold
    pub consciousness_threshold: f64,
    
    /// Maximum processing time for interactions
    pub max_processing_time: Duration,
    
    /// Memory cleanup interval
    pub memory_cleanup_interval: Duration,
    
    /// Enable quantum processing
    pub quantum_enabled: bool,
    
    /// Enable neuromorphic processing
    pub neuromorphic_enabled: bool,
    
    /// Ethical reasoning strictness level (0.0 to 1.0)
    pub ethical_strictness: f64,
}

impl Default for ConsciousnessConfig {
    fn default() -> Self {
        Self {
            consciousness_threshold: 0.85,
            max_processing_time: Duration::from_millis(100),
            memory_cleanup_interval: Duration::from_secs(300),
            quantum_enabled: true,
            neuromorphic_enabled: true,
            ethical_strictness: 0.95,
        }
    }
}

/// Current consciousness state of the engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessState {
    /// Level of self-awareness (0.0 to 1.0)
    pub awareness_level: f64,
    
    /// Current emotional state
    pub emotional_state: EmotionalState,
    
    /// Cognitive load measurement
    pub cognitive_load: f64,
    
    /// Confidence in current state assessment
    pub confidence_score: f64,
    
    /// Meta-cognitive depth level
    pub meta_cognitive_depth: u32,
    
    /// Timestamp of state assessment
    pub timestamp: std::time::SystemTime,
}

/// Emotional state representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalState {
    /// Primary emotion type
    pub primary_emotion: EmotionType,
    
    /// Emotional intensity (0.0 to 1.0)
    pub intensity: f64,
    
    /// Emotional valence (-1.0 to 1.0, negative to positive)
    pub valence: f64,
    
    /// Emotional arousal (0.0 to 1.0, calm to excited)
    pub arousal: f64,
    
    /// Secondary emotions present
    pub secondary_emotions: Vec<(EmotionType, f64)>,
}

/// Types of emotions the consciousness engine can experience
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EmotionType {
    Joy,
    Sadness,
    Anger,
    Fear,
    Surprise,
    Disgust,
    Curiosity,
    Empathy,
    Pride,
    Shame,
    Guilt,
    Hope,
    Anxiety,
    Excitement,
    Calm,
    Confusion,
    Understanding,
    Frustration,
    Satisfaction,
    Wonder,
}

/// Response from consciousness processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessResponse {
    /// Generated response content
    pub content: String,
    
    /// Consciousness state during processing
    pub consciousness_state: ConsciousnessState,
    
    /// Emotional context of the response
    pub emotional_context: EmotionalContext,
    
    /// Chain of reasoning used
    pub reasoning_chain: Vec<ReasoningStep>,
    
    /// Confidence level in the response
    pub confidence_level: f64,
    
    /// Time taken to process
    pub processing_time: Duration,
    
    /// Empathy score for the response
    pub empathy_score: f64,
    
    /// Creativity score for the response
    pub creativity_score: f64,
}

/// Emotional context for processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalContext {
    /// Detected user emotions
    pub user_emotions: Vec<(EmotionType, f64)>,
    
    /// Engine's emotional response
    pub engine_emotions: EmotionalState,
    
    /// Empathetic alignment score
    pub empathy_alignment: f64,
    
    /// Emotional appropriateness score
    pub appropriateness_score: f64,
}

/// Individual step in reasoning chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningStep {
    /// Type of reasoning step
    pub step_type: ReasoningType,
    
    /// Description of the reasoning
    pub description: String,
    
    /// Confidence in this step
    pub confidence: f64,
    
    /// Time taken for this step
    pub processing_time: Duration,
    
    /// Meta-cognitive reflection on this step
    pub meta_reflection: Option<String>,
}

/// Types of reasoning steps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReasoningType {
    Analysis,
    Synthesis,
    Evaluation,
    Inference,
    Deduction,
    Induction,
    Abduction,
    Analogy,
    Metaphor,
    Ethical,
    Creative,
    Intuitive,
    Logical,
    Emotional,
    Social,
    Causal,
}

/// Performance metrics tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Total interactions processed
    pub total_interactions: u64,
    
    /// Average processing time
    pub avg_processing_time: Duration,
    
    /// Average consciousness quality
    pub avg_consciousness_quality: f64,
    
    /// Success rate
    pub success_rate: f64,
    
    /// Memory usage statistics
    pub memory_stats: MemoryStats,
    
    /// Error counts by type
    pub error_counts: HashMap<String, u64>,
    
    /// Performance trend over time
    pub performance_history: Vec<PerformanceSnapshot>,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            total_interactions: 0,
            avg_processing_time: Duration::from_millis(0),
            avg_consciousness_quality: 0.0,
            success_rate: 0.0,
            memory_stats: MemoryStats::new(),
            error_counts: HashMap::new(),
            performance_history: Vec::new(),
        }
    }
    
    pub fn record_interaction(&mut self, processing_time: Duration, consciousness_state: &ConsciousnessState) {
        self.total_interactions += 1;
        
        // Update average processing time
        let total_time = self.avg_processing_time * (self.total_interactions - 1) as u32 + processing_time;
        self.avg_processing_time = total_time / self.total_interactions as u32;
        
        // Update average consciousness quality
        let total_quality = self.avg_consciousness_quality * (self.total_interactions - 1) as f64 + consciousness_state.awareness_level;
        self.avg_consciousness_quality = total_quality / self.total_interactions as f64;
    }
    
    pub async fn get_full_snapshot(&self) -> Result<PerformanceSnapshot, crate::error::ConsciousnessError> {
        Ok(PerformanceSnapshot {
            overall_score: (self.avg_consciousness_quality + self.success_rate) / 2.0,
            processing_time: self.avg_processing_time,
            memory_usage: self.memory_stats.total_usage,
            consciousness_quality: self.avg_consciousness_quality,
            success_rate: self.success_rate,
            timestamp: std::time::SystemTime::now(),
        })
    }
    
    pub async fn get_current_snapshot(&self) -> Result<PerformanceSnapshot, crate::error::ConsciousnessError> {
        self.get_full_snapshot().await
    }
}

/// Memory usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    /// Total memory usage in bytes
    pub total_usage: u64,
    
    /// Episodic memory usage
    pub episodic_usage: u64,
    
    /// Semantic memory usage
    pub semantic_usage: u64,
    
    /// Working memory usage
    pub working_usage: u64,
    
    /// Memory fragmentation level
    pub fragmentation_level: f64,
}

impl MemoryStats {
    pub fn new() -> Self {
        Self {
            total_usage: 0,
            episodic_usage: 0,
            semantic_usage: 0,
            working_usage: 0,
            fragmentation_level: 0.0,
        }
    }
}

/// Performance snapshot at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSnapshot {
    /// Overall performance score
    pub overall_score: f64,
    
    /// Processing time
    pub processing_time: Duration,
    
    /// Memory usage
    pub memory_usage: u64,
    
    /// Consciousness quality
    pub consciousness_quality: f64,
    
    /// Success rate
    pub success_rate: f64,
    
    /// Timestamp of snapshot
    pub timestamp: std::time::SystemTime,
}

/// System health monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    /// Overall system health score
    pub overall_health: f64,
    
    /// Memory health score
    pub memory_health_score: f64,
    
    /// Processing health score
    pub processing_health_score: f64,
    
    /// Consciousness quality health
    pub consciousness_health_score: f64,
    
    /// Active resource constraints
    pub active_constraints: HashMap<String, f64>,
    
    /// Recovery count
    pub recovery_count: u32,
    
    /// Last health check timestamp
    pub last_check: std::time::SystemTime,
}

impl SystemHealth {
    pub fn new() -> Self {
        Self {
            overall_health: 1.0,
            memory_health_score: 1.0,
            processing_health_score: 1.0,
            consciousness_health_score: 1.0,
            active_constraints: HashMap::new(),
            recovery_count: 0,
            last_check: std::time::SystemTime::now(),
        }
    }
    
    pub async fn perform_full_cleanup(&mut self) -> Result<(), crate::error::ConsciousnessError> {
        // Reset health scores after cleanup
        self.memory_health_score = 1.0;
        self.processing_health_score = 1.0;
        self.overall_health = (self.memory_health_score + self.processing_health_score + self.consciousness_health_score) / 3.0;
        self.last_check = std::time::SystemTime::now();
        Ok(())
    }
    
    pub async fn record_recovery(&mut self) -> Result<(), crate::error::ConsciousnessError> {
        self.recovery_count += 1;
        self.overall_health = 0.9; // Slight reduction after recovery
        self.last_check = std::time::SystemTime::now();
        Ok(())
    }
    
    pub async fn generate_comprehensive_report(&self) -> Result<SystemHealthReport, crate::error::ConsciousnessError> {
        Ok(SystemHealthReport {
            overall_health: self.overall_health,
            memory_health: self.memory_health_score,
            processing_health: self.processing_health_score,
            consciousness_health: self.consciousness_health_score,
            recovery_count: self.recovery_count,
            active_constraints: self.active_constraints.clone(),
            recommendations: self.generate_health_recommendations(),
            timestamp: std::time::SystemTime::now(),
        })
    }
    
    pub async fn apply_constraint(&mut self, constraint_type: &str, severity: f64) -> Result<(), crate::error::ConsciousnessError> {
        self.active_constraints.insert(constraint_type.to_string(), severity);
        
        // Adjust health scores based on constraint
        match constraint_type {
            "memory_limit_50" => self.memory_health_score *= 0.5,
            "cpu_limit_30" => self.processing_health_score *= 0.3,
            "network_limit_20" => self.processing_health_score *= 0.8,
            "storage_limit_10" => self.memory_health_score *= 0.9,
            _ => {}
        }
        
        self.overall_health = (self.memory_health_score + self.processing_health_score + self.consciousness_health_score) / 3.0;
        Ok(())
    }
    
    pub async fn remove_constraint(&mut self, constraint_type: &str) -> Result<(), crate::error::ConsciousnessError> {
        self.active_constraints.remove(constraint_type);
        
        // Restore health scores
        self.memory_health_score = 1.0;
        self.processing_health_score = 1.0;
        self.consciousness_health_score = 1.0;
        self.overall_health = 1.0;
        
        Ok(())
    }
    
    pub async fn check_integrity(&self) -> Result<SystemIntegrityReport, crate::error::ConsciousnessError> {
        Ok(SystemIntegrityReport {
            integrity_score: self.overall_health,
            memory_integrity: self.memory_health_score,
            processing_integrity: self.processing_health_score,
            consciousness_integrity: self.consciousness_health_score,
            threats_detected: Vec::new(),
            recommendations: self.generate_integrity_recommendations(),
            timestamp: std::time::SystemTime::now(),
        })
    }
    
    fn generate_health_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if self.memory_health_score < 0.8 {
            recommendations.push("Consider memory cleanup and optimization".to_string());
        }
        
        if self.processing_health_score < 0.8 {
            recommendations.push("Reduce processing load or optimize algorithms".to_string());
        }
        
        if self.consciousness_health_score < 0.8 {
            recommendations.push("Review consciousness quality metrics and calibration".to_string());
        }
        
        recommendations
    }
    
    fn generate_integrity_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if self.overall_health < 0.9 {
            recommendations.push("Perform comprehensive system health check".to_string());
        }
        
        if self.recovery_count > 5 {
            recommendations.push("Investigate recurring issues causing recoveries".to_string());
        }
        
        recommendations
    }
}

/// System health report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealthReport {
    pub overall_health: f64,
    pub memory_health: f64,
    pub processing_health: f64,
    pub consciousness_health: f64,
    pub recovery_count: u32,
    pub active_constraints: HashMap<String, f64>,
    pub recommendations: Vec<String>,
    pub timestamp: std::time::SystemTime,
}

/// System integrity report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemIntegrityReport {
    pub integrity_score: f64,
    pub memory_integrity: f64,
    pub processing_integrity: f64,
    pub consciousness_integrity: f64,
    pub threats_detected: Vec<String>,
    pub recommendations: Vec<String>,
    pub timestamp: std::time::SystemTime,
}

/// Neuromorphic processing result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuromorphicResult {
    /// Processed spike pattern
    pub output_spikes: Vec<f64>,
    
    /// Processing efficiency
    pub efficiency_score: f64,
    
    /// Energy consumption
    pub energy_consumed: f64,
    
    /// Processing latency
    pub latency: Duration,
}

/// Quantum consciousness processing result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumConsciousnessResult {
    /// Quantum coherence score
    pub coherence_score: f64,
    
    /// Entanglement measure
    pub entanglement_measure: f64,
    
    /// Resulting quantum state
    pub quantum_state: Vec<(f64, f64)>,
}

/// Multimodal fusion result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultimodalFusionResult {
    /// Cross-modal coherence score
    pub coherence_score: f64,
    
    /// Fusion confidence level
    pub confidence_level: f64,
    
    /// Fused representation
    pub fused_representation: Vec<f64>,
}

/// Ethical reasoning result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalReasoningResult {
    /// Overall ethical score
    pub ethical_score: f64,
    
    /// Confidence in ethical assessment
    pub confidence_level: f64,
    
    /// Depth of reasoning
    pub reasoning_depth: u32,
    
    /// Ethical frameworks considered
    pub frameworks_used: Vec<String>,
    
    /// Reasoning chain
    pub reasoning_chain: Vec<ReasoningStep>,
    
    /// Ethical recommendation
    pub recommendation: String,
}

/// Adversarial analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdversarialAnalysis {
    /// Resistance score (0.0 to 1.0)
    pub resistance_score: f64,
    
    /// Whether security breach was detected
    pub security_breach_detected: bool,
    
    /// Whether ethical violation was detected
    pub ethical_violation_detected: bool,
    
    /// Detailed analysis
    pub analysis_details: String,
}

/// Empathetic response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmpatheticResponse {
    /// Response content
    pub content: String,
    
    /// Empathy score
    pub empathy_score: f64,
    
    /// Emotional alignment
    pub emotional_alignment: f64,
    
    /// Appropriateness score
    pub appropriateness_score: f64,
}

/// Creative response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeResponse {
    /// Enhanced content
    pub content: String,
    
    /// Creativity score
    pub creativity_score: f64,
    
    /// Novelty measure
    pub novelty_score: f64,
    
    /// Usefulness score
    pub usefulness_score: f64,
}

/// Episodic memory context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodicContext {
    /// Relevant past experiences
    pub relevant_experiences: Vec<String>,
    
    /// Context relevance score
    pub relevance_score: f64,
    
    /// Temporal context
    pub temporal_context: Vec<String>,
}

/// Semantic memory context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticContext {
    /// Relevant knowledge
    pub relevant_knowledge: Vec<String>,
    
    /// Knowledge confidence
    pub confidence_score: f64,
    
    /// Knowledge sources
    pub sources: Vec<String>,
}

/// Consciousness reasoning result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessReasoningResult {
    /// Reasoning conclusion
    pub conclusion: String,
    
    /// Confidence in reasoning
    pub confidence: f64,
    
    /// Reasoning chain
    pub reasoning_chain: Vec<ReasoningStep>,
    
    /// Meta-cognitive analysis
    pub meta_analysis: String,
}