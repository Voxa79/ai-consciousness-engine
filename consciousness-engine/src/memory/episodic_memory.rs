//! Episodic Memory - Memories of specific experiences and events

use crate::{
    types::*,
    error::{ConsciousnessError, ConsciousnessResult},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use tracing::{debug, info};

/// Episodic memory system for storing and retrieving specific experiences
#[derive(Debug, Clone)]
pub struct EpisodicMemorySystem {
    memories: HashMap<Uuid, EpisodicMemory>,
    memory_index: MemoryIndex,
    consolidation_engine: ConsolidationEngine,
    retrieval_engine: RetrievalEngine,
    forgetting_curve: ForgettingCurve,
}

impl EpisodicMemorySystem {
    /// Create a new episodic memory system
    pub fn new() -> Self {
        info!("Initializing Episodic Memory System");
        
        Self {
            memories: HashMap::new(),
            memory_index: MemoryIndex::new(),
            consolidation_engine: ConsolidationEngine::new(),
            retrieval_engine: RetrievalEngine::new(),
            forgetting_curve: ForgettingCurve::new(),
        }
    }
    
    /// Store a new episodic memory
    pub async fn store_memory(
        &mut self,
        experience: Experience,
        emotional_context: EmotionalContext,
        consciousness_state: ConsciousnessState,
    ) -> ConsciousnessResult<Uuid> {
        let memory_id = Uuid::new_v4();
        
        debug!("Storing episodic memory: {}", memory_id);
        
        // Calculate initial significance score
        let significance_score = self.calculate_significance_score(
            &experience,
            &emotional_context,
            &consciousness_state,
        );
        
        // Extract lessons learned from the experience
        let lessons_learned = self.extract_lessons_learned(&experience, &emotional_context).await?;
        
        // Create episodic memory
        let episodic_memory = EpisodicMemory {
            memory_id,
            experience: experience.clone(),
            emotional_context: emotional_context.clone(),
            consciousness_state_at_time: consciousness_state,
            lessons_learned,
            significance_score,
            retrieval_count: 0,
            last_accessed: Utc::now(),
            memory_strength: 1.0,
            consolidation_level: ConsolidationLevel::Fresh,
            associated_memories: Vec::new(),
            tags: self.generate_memory_tags(&experience, &emotional_context),
            created_at: Utc::now(),
        };
        
        // Store memory
        self.memories.insert(memory_id, episodic_memory.clone());
        
        // Update memory index for fast retrieval
        self.memory_index.index_memory(&episodic_memory).await?;
        
        // Schedule for consolidation if significant
        if significance_score > 0.7 {
            self.consolidation_engine.schedule_consolidation(memory_id).await?;
        }
        
        info!("Episodic memory stored: {} (significance: {:.2})", memory_id, significance_score);
        
        Ok(memory_id)
    }
    
    /// Retrieve memories based on context and similarity
    pub async fn retrieve_memories(
        &mut self,
        query_context: &RetrievalContext,
        max_results: usize,
    ) -> ConsciousnessResult<Vec<EpisodicMemory>> {
        debug!("Retrieving episodic memories for context: {:?}", query_context.query_type);
        
        // Find candidate memories using index
        let candidate_ids = self.memory_index.find_candidates(query_context).await?;
        
        // Score and rank memories
        let mut scored_memories = Vec::new();
        for memory_id in candidate_ids {
            if let Some(memory) = self.memories.get_mut(&memory_id) {
                let relevance_score = self.calculate_relevance_score(memory, query_context).await?;
                
                if relevance_score > 0.3 { // Minimum relevance threshold
                    // Update retrieval statistics
                    memory.retrieval_count += 1;
                    memory.last_accessed = Utc::now();
                    
                    // Apply forgetting curve
                    memory.memory_strength = self.forgetting_curve.calculate_strength(
                        memory.memory_strength,
                        memory.last_accessed,
                        memory.created_at,
                        memory.retrieval_count,
                    );
                    
                    scored_memories.push((memory.clone(), relevance_score));
                }
            }
        }
        
        // Sort by relevance score
        scored_memories.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Return top results
        let results: Vec<EpisodicMemory> = scored_memories
            .into_iter()
            .take(max_results)
            .map(|(memory, _)| memory)
            .collect();
        
        debug!("Retrieved {} episodic memories", results.len());
        
        Ok(results)
    }
    
    /// Consolidate memories (strengthen important ones, weaken unimportant ones)
    pub async fn consolidate_memories(&mut self) -> ConsciousnessResult<ConsolidationReport> {
        debug!("Starting memory consolidation process");
        
        let mut consolidated_count = 0;
        let mut strengthened_count = 0;
        let mut weakened_count = 0;
        
        for (memory_id, memory) in self.memories.iter_mut() {
            // Check if memory needs consolidation
            if self.consolidation_engine.needs_consolidation(memory).await? {
                let consolidation_result = self.consolidation_engine
                    .consolidate_memory(memory).await?;
                
                match consolidation_result.action {
                    ConsolidationAction::Strengthen => {
                        memory.memory_strength = (memory.memory_strength * 1.2).min(2.0);
                        memory.consolidation_level = ConsolidationLevel::Consolidated;
                        strengthened_count += 1;
                    },
                    ConsolidationAction::Weaken => {
                        memory.memory_strength *= 0.8;
                        weakened_count += 1;
                    },
                    ConsolidationAction::Associate => {
                        // Find and link related memories
                        let related_memories = self.find_related_memories(memory).await?;
                        memory.associated_memories.extend(related_memories);
                    },
                    ConsolidationAction::NoAction => {},
                }
                
                consolidated_count += 1;
            }
        }
        
        // Remove very weak memories (forgetting)
        let forgotten_count = self.forget_weak_memories().await?;
        
        let report = ConsolidationReport {
            total_memories: self.memories.len(),
            consolidated_count,
            strengthened_count,
            weakened_count,
            forgotten_count,
            consolidation_time: Utc::now(),
        };
        
        info!("Memory consolidation completed: {:?}", report);
        
        Ok(report)
    }
    
    /// Calculate significance score for a new experience
    fn calculate_significance_score(
        &self,
        experience: &Experience,
        emotional_context: &EmotionalContext,
        consciousness_state: &ConsciousnessState,
    ) -> f64 {
        let mut significance = 0.5; // Base significance
        
        // Emotional intensity increases significance
        significance += emotional_context.emotional_intensity * 0.3;
        
        // High consciousness level increases significance
        significance += consciousness_state.overall_consciousness_score * 0.2;
        
        // Novel experiences are more significant
        if experience.novelty_score > 0.7 {
            significance += 0.2;
        }
        
        // Learning experiences are significant
        if !experience.lessons_learned.is_empty() {
            significance += 0.15;
        }
        
        // Errors and mistakes are significant for learning
        if experience.experience_type == "error" || experience.experience_type == "mistake" {
            significance += 0.25;
        }
        
        significance.min(1.0)
    }
    
    /// Extract lessons learned from an experience
    async fn extract_lessons_learned(
        &self,
        experience: &Experience,
        emotional_context: &EmotionalContext,
    ) -> ConsciousnessResult<Vec<Lesson>> {
        let mut lessons = Vec::new();
        
        // Extract lessons based on experience type
        match experience.experience_type.as_str() {
            "error" | "mistake" => {
                lessons.push(Lesson {
                    lesson_type: "error_correction".to_string(),
                    description: format!("Learned to avoid: {}", experience.description),
                    confidence: 0.8,
                    applicability: vec!["similar_contexts".to_string()],
                });
            },
            "success" | "achievement" => {
                lessons.push(Lesson {
                    lesson_type: "success_pattern".to_string(),
                    description: format!("Successful approach: {}", experience.description),
                    confidence: 0.7,
                    applicability: vec!["similar_goals".to_string()],
                });
            },
            "social_interaction" => {
                if emotional_context.emotional_valence > 0.5 {
                    lessons.push(Lesson {
                        lesson_type: "social_success".to_string(),
                        description: "Positive social interaction pattern identified".to_string(),
                        confidence: 0.6,
                        applicability: vec!["social_contexts".to_string()],
                    });
                } else if emotional_context.emotional_valence < -0.3 {
                    lessons.push(Lesson {
                        lesson_type: "social_caution".to_string(),
                        description: "Social interaction pattern to avoid".to_string(),
                        confidence: 0.7,
                        applicability: vec!["social_contexts".to_string()],
                    });
                }
            },
            _ => {
                // General learning from any experience
                if experience.novelty_score > 0.6 {
                    lessons.push(Lesson {
                        lesson_type: "general_learning".to_string(),
                        description: format!("New experience: {}", experience.description),
                        confidence: 0.5,
                        applicability: vec!["general".to_string()],
                    });
                }
            }
        }
        
        Ok(lessons)
    }
    
    /// Generate tags for memory indexing
    fn generate_memory_tags(
        &self,
        experience: &Experience,
        emotional_context: &EmotionalContext,
    ) -> Vec<String> {
        let mut tags = Vec::new();
        
        // Experience type tag
        tags.push(experience.experience_type.clone());
        
        // Emotional tags
        tags.push(emotional_context.primary_emotion.clone());
        
        if emotional_context.emotional_valence > 0.3 {
            tags.push("positive".to_string());
        } else if emotional_context.emotional_valence < -0.3 {
            tags.push("negative".to_string());
        }
        
        if emotional_context.emotional_intensity > 0.7 {
            tags.push("intense".to_string());
        }
        
        // Context tags
        for factor in &emotional_context.contextual_factors {
            tags.push(factor.clone());
        }
        
        // Significance tags
        if experience.novelty_score > 0.7 {
            tags.push("novel".to_string());
        }
        
        if !experience.lessons_learned.is_empty() {
            tags.push("educational".to_string());
        }
        
        tags
    }
    
    /// Calculate relevance score for memory retrieval
    async fn calculate_relevance_score(
        &self,
        memory: &EpisodicMemory,
        query_context: &RetrievalContext,
    ) -> ConsciousnessResult<f64> {
        let mut relevance = 0.0;
        
        // Temporal relevance (recent memories are more relevant)
        let time_diff = Utc::now().signed_duration_since(memory.created_at);
        let temporal_relevance = self.calculate_temporal_relevance(time_diff);
        relevance += temporal_relevance * 0.2;
        
        // Contextual similarity
        let context_similarity = self.calculate_context_similarity(
            &memory.emotional_context,
            &query_context.emotional_context,
        );
        relevance += context_similarity * 0.3;
        
        // Tag matching
        let tag_relevance = self.calculate_tag_relevance(&memory.tags, &query_context.tags);
        relevance += tag_relevance * 0.2;
        
        // Memory strength (stronger memories are more relevant)
        relevance += memory.memory_strength * 0.15;
        
        // Significance score
        relevance += memory.significance_score * 0.15;
        
        Ok(relevance.min(1.0))
    }
    
    /// Calculate temporal relevance based on time difference
    fn calculate_temporal_relevance(&self, time_diff: Duration) -> f64 {
        let days = time_diff.num_days() as f64;
        
        // Exponential decay with half-life of 30 days
        (-days / 30.0).exp()
    }
    
    /// Calculate similarity between emotional contexts
    fn calculate_context_similarity(
        &self,
        context1: &EmotionalContext,
        context2: &EmotionalContext,
    ) -> f64 {
        let mut similarity = 0.0;
        
        // Primary emotion match
        if context1.primary_emotion == context2.primary_emotion {
            similarity += 0.4;
        }
        
        // Emotional valence similarity
        let valence_diff = (context1.emotional_valence - context2.emotional_valence).abs();
        similarity += (1.0 - valence_diff) * 0.3;
        
        // Intensity similarity
        let intensity_diff = (context1.emotional_intensity - context2.emotional_intensity).abs();
        similarity += (1.0 - intensity_diff) * 0.3;
        
        similarity
    }
    
    /// Calculate tag relevance
    fn calculate_tag_relevance(&self, memory_tags: &[String], query_tags: &[String]) -> f64 {
        if query_tags.is_empty() || memory_tags.is_empty() {
            return 0.0;
        }
        
        let matching_tags = memory_tags.iter()
            .filter(|tag| query_tags.contains(tag))
            .count();
        
        matching_tags as f64 / query_tags.len() as f64
    }
    
    /// Find related memories for association
    async fn find_related_memories(&self, memory: &EpisodicMemory) -> ConsciousnessResult<Vec<Uuid>> {
        let mut related = Vec::new();
        
        for (other_id, other_memory) in &self.memories {
            if *other_id == memory.memory_id {
                continue;
            }
            
            // Check for tag overlap
            let tag_overlap = memory.tags.iter()
                .filter(|tag| other_memory.tags.contains(tag))
                .count();
            
            if tag_overlap >= 2 {
                related.push(*other_id);
            }
        }
        
        Ok(related)
    }
    
    /// Remove very weak memories (forgetting process)
    async fn forget_weak_memories(&mut self) -> ConsciousnessResult<usize> {
        let weak_threshold = 0.1;
        let mut forgotten_count = 0;
        
        let weak_memory_ids: Vec<Uuid> = self.memories
            .iter()
            .filter(|(_, memory)| memory.memory_strength < weak_threshold)
            .map(|(id, _)| *id)
            .collect();
        
        for memory_id in weak_memory_ids {
            self.memories.remove(&memory_id);
            self.memory_index.remove_memory(memory_id).await?;
            forgotten_count += 1;
        }
        
        if forgotten_count > 0 {
            debug!("Forgot {} weak memories", forgotten_count);
        }
        
        Ok(forgotten_count)
    }
    
    /// Get memory statistics
    pub fn get_memory_statistics(&self) -> MemoryStatistics {
        let total_memories = self.memories.len();
        let strong_memories = self.memories.values()
            .filter(|m| m.memory_strength > 0.8)
            .count();
        let consolidated_memories = self.memories.values()
            .filter(|m| matches!(m.consolidation_level, ConsolidationLevel::Consolidated))
            .count();
        
        let average_significance = if total_memories > 0 {
            self.memories.values()
                .map(|m| m.significance_score)
                .sum::<f64>() / total_memories as f64
        } else {
            0.0
        };
        
        MemoryStatistics {
            total_memories,
            strong_memories,
            consolidated_memories,
            average_significance,
            oldest_memory: self.memories.values()
                .map(|m| m.created_at)
                .min(),
            most_accessed_memory: self.memories.values()
                .max_by_key(|m| m.retrieval_count)
                .map(|m| m.memory_id),
        }
    }
}

/// Supporting structures and implementations
#[derive(Debug, Clone)]
struct MemoryIndex {
    tag_index: HashMap<String, Vec<Uuid>>,
    temporal_index: Vec<(DateTime<Utc>, Uuid)>,
}

impl MemoryIndex {
    fn new() -> Self {
        Self {
            tag_index: HashMap::new(),
            temporal_index: Vec::new(),
        }
    }
    
    async fn index_memory(&mut self, memory: &EpisodicMemory) -> ConsciousnessResult<()> {
        // Index by tags
        for tag in &memory.tags {
            self.tag_index
                .entry(tag.clone())
                .or_insert_with(Vec::new)
                .push(memory.memory_id);
        }
        
        // Index by time
        self.temporal_index.push((memory.created_at, memory.memory_id));
        self.temporal_index.sort_by_key(|(time, _)| *time);
        
        Ok(())
    }
    
    async fn find_candidates(&self, query_context: &RetrievalContext) -> ConsciousnessResult<Vec<Uuid>> {
        let mut candidates = std::collections::HashSet::new();
        
        // Find by tags
        for tag in &query_context.tags {
            if let Some(memory_ids) = self.tag_index.get(tag) {
                candidates.extend(memory_ids);
            }
        }
        
        // If no tag matches, return recent memories
        if candidates.is_empty() {
            candidates.extend(
                self.temporal_index
                    .iter()
                    .rev()
                    .take(20)
                    .map(|(_, id)| *id)
            );
        }
        
        Ok(candidates.into_iter().collect())
    }
    
    async fn remove_memory(&mut self, memory_id: Uuid) -> ConsciousnessResult<()> {
        // Remove from tag index
        for memory_ids in self.tag_index.values_mut() {
            memory_ids.retain(|&id| id != memory_id);
        }
        
        // Remove from temporal index
        self.temporal_index.retain(|(_, id)| *id != memory_id);
        
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct ConsolidationEngine;

impl ConsolidationEngine {
    fn new() -> Self { Self }
    
    async fn schedule_consolidation(&self, _memory_id: Uuid) -> ConsciousnessResult<()> {
        // TODO: Implement consolidation scheduling
        Ok(())
    }
    
    async fn needs_consolidation(&self, memory: &EpisodicMemory) -> ConsciousnessResult<bool> {
        // Consolidate if memory is significant and not yet consolidated
        Ok(memory.significance_score > 0.6 && 
           matches!(memory.consolidation_level, ConsolidationLevel::Fresh))
    }
    
    async fn consolidate_memory(&self, memory: &EpisodicMemory) -> ConsciousnessResult<ConsolidationResult> {
        let action = if memory.significance_score > 0.8 {
            ConsolidationAction::Strengthen
        } else if memory.significance_score < 0.3 {
            ConsolidationAction::Weaken
        } else {
            ConsolidationAction::Associate
        };
        
        Ok(ConsolidationResult { action })
    }
}

#[derive(Debug, Clone)]
struct RetrievalEngine;

impl RetrievalEngine {
    fn new() -> Self { Self }
}

#[derive(Debug, Clone)]
struct ForgettingCurve;

impl ForgettingCurve {
    fn new() -> Self { Self }
    
    fn calculate_strength(
        &self,
        current_strength: f64,
        last_accessed: DateTime<Utc>,
        created_at: DateTime<Utc>,
        retrieval_count: u32,
    ) -> f64 {
        let time_since_access = Utc::now().signed_duration_since(last_accessed);
        let days_since_access = time_since_access.num_days() as f64;
        
        // Ebbinghaus forgetting curve with spaced repetition
        let decay_rate = 0.1 / (1.0 + retrieval_count as f64 * 0.1);
        let strength_decay = (-decay_rate * days_since_access).exp();
        
        (current_strength * strength_decay).max(0.01)
    }
}

/// Data structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodicMemory {
    pub memory_id: Uuid,
    pub experience: Experience,
    pub emotional_context: EmotionalContext,
    pub consciousness_state_at_time: ConsciousnessState,
    pub lessons_learned: Vec<Lesson>,
    pub significance_score: f64,
    pub retrieval_count: u32,
    pub last_accessed: DateTime<Utc>,
    pub memory_strength: f64,
    pub consolidation_level: ConsolidationLevel,
    pub associated_memories: Vec<Uuid>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub experience_id: Uuid,
    pub experience_type: String,
    pub description: String,
    pub context: String,
    pub outcome: String,
    pub lessons_learned: Vec<String>,
    pub novelty_score: f64,
    pub impact_score: f64,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub lesson_type: String,
    pub description: String,
    pub confidence: f64,
    pub applicability: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsolidationLevel {
    Fresh,
    Consolidating,
    Consolidated,
    LongTerm,
}

#[derive(Debug, Clone)]
pub struct RetrievalContext {
    pub query_type: String,
    pub emotional_context: EmotionalContext,
    pub tags: Vec<String>,
    pub time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    pub significance_threshold: f64,
}

#[derive(Debug, Clone)]
pub struct ConsolidationReport {
    pub total_memories: usize,
    pub consolidated_count: usize,
    pub strengthened_count: usize,
    pub weakened_count: usize,
    pub forgotten_count: usize,
    pub consolidation_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ConsolidationResult {
    pub action: ConsolidationAction,
}

#[derive(Debug, Clone)]
pub enum ConsolidationAction {
    Strengthen,
    Weaken,
    Associate,
    NoAction,
}

#[derive(Debug, Clone)]
pub struct MemoryStatistics {
    pub total_memories: usize,
    pub strong_memories: usize,
    pub consolidated_memories: usize,
    pub average_significance: f64,
    pub oldest_memory: Option<DateTime<Utc>>,
    pub most_accessed_memory: Option<Uuid>,
}

impl Default for EpisodicMemorySystem {
    fn default() -> Self {
        Self::new()
    }
}