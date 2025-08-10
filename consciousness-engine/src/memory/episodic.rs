//! Episodic Memory System - Revolutionary Experience Storage
//! 
//! This module implements a sophisticated episodic memory system that stores and retrieves
//! experiences with temporal context, emotional associations, and consciousness states.

use crate::types::*;
use crate::error::ConsciousnessError;
use std::collections::{HashMap, BTreeMap};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Episodic memory entry representing a stored experience
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodicMemoryEntry {
    /// Unique identifier for the memory
    pub id: Uuid,
    
    /// Input that triggered this experience
    pub input: String,
    
    /// Response generated for this experience
    pub response: String,
    
    /// Consciousness state during the experience
    pub consciousness_state: ConsciousnessState,
    
    /// Emotional context of the experience
    pub emotional_context: EmotionalContext,
    
    /// Temporal context
    pub temporal_context: TemporalContext,
    
    /// Associated concepts and entities
    pub associated_concepts: Vec<String>,
    
    /// Memory strength (0.0 to 1.0)
    pub memory_strength: f64,
    
    /// Access count (how often this memory has been retrieved)
    pub access_count: u32,
    
    /// Last access time
    pub last_accessed: SystemTime,
    
    /// Creation timestamp
    pub created_at: SystemTime,
    
    /// Memory tags for categorization
    pub tags: Vec<String>,
    
    /// Importance score
    pub importance_score: f64,
    
    /// Associated sensory data
    pub sensory_data: Option<SensoryData>,
}

/// Temporal context for episodic memories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalContext {
    /// Absolute timestamp
    pub timestamp: SystemTime,
    
    /// Relative time context (e.g., "morning", "after lunch")
    pub relative_time: Option<String>,
    
    /// Duration of the experience
    pub duration: Duration,
    
    /// Sequence number in conversation
    pub sequence_number: u32,
    
    /// Related temporal events
    pub related_events: Vec<String>,
}

/// Sensory data associated with memories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensoryData {
    /// Visual elements
    pub visual: Option<Vec<f64>>,
    
    /// Auditory elements
    pub auditory: Option<Vec<f64>>,
    
    /// Textual patterns
    pub textual_patterns: Vec<String>,
    
    /// Emotional sensory imprints
    pub emotional_imprints: HashMap<EmotionType, f64>,
}

/// Memory consolidation state
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ConsolidationState {
    /// Fresh memory, not yet consolidated
    Fresh,
    /// Partially consolidated
    Consolidating,
    /// Fully consolidated long-term memory
    Consolidated,
    /// Memory marked for potential forgetting
    Fading,
}

/// Memory retrieval result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRetrievalResult {
    /// Retrieved memories
    pub memories: Vec<EpisodicMemoryEntry>,
    
    /// Relevance scores for each memory
    pub relevance_scores: Vec<f64>,
    
    /// Total memories searched
    pub total_searched: usize,
    
    /// Retrieval time
    pub retrieval_time: Duration,
    
    /// Confidence in retrieval quality
    pub confidence: f64,
}

/// Episodic Memory System
pub struct EpisodicMemory {
    /// Main memory storage indexed by ID
    memories: Arc<RwLock<HashMap<Uuid, EpisodicMemoryEntry>>>,
    
    /// Temporal index for time-based retrieval
    temporal_index: Arc<RwLock<BTreeMap<u64, Vec<Uuid>>>>,
    
    /// Concept index for semantic retrieval
    concept_index: Arc<RwLock<HashMap<String, Vec<Uuid>>>>,
    
    /// Emotional index for emotion-based retrieval
    emotional_index: Arc<RwLock<HashMap<EmotionType, Vec<Uuid>>>>,
    
    /// Memory consolidation manager
    consolidation_manager: Arc<RwLock<ConsolidationManager>>,
    
    /// Memory statistics
    statistics: Arc<RwLock<MemoryStatistics>>,
    
    /// Configuration
    config: EpisodicMemoryConfig,
}

/// Memory consolidation manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationManager {
    /// Memories pending consolidation
    pub pending_consolidation: Vec<Uuid>,
    
    /// Consolidation schedule
    pub consolidation_schedule: BTreeMap<u64, Vec<Uuid>>,
    
    /// Last consolidation run
    pub last_consolidation: SystemTime,
    
    /// Consolidation statistics
    pub consolidation_stats: ConsolidationStats,
}

/// Consolidation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationStats {
    pub total_consolidated: u64,
    pub total_forgotten: u64,
    pub average_consolidation_time: Duration,
    pub consolidation_success_rate: f64,
}

/// Memory statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStatistics {
    /// Total memories stored
    pub total_memories: u64,
    
    /// Memory by consolidation state
    pub by_consolidation_state: HashMap<ConsolidationState, u64>,
    
    /// Average memory strength
    pub average_memory_strength: f64,
    
    /// Total memory size in bytes
    pub total_size_bytes: u64,
    
    /// Retrieval statistics
    pub retrieval_stats: RetrievalStatistics,
    
    /// Forgetting curve data
    pub forgetting_curve: Vec<(Duration, f64)>,
}

/// Retrieval statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievalStatistics {
    pub total_retrievals: u64,
    pub average_retrieval_time: Duration,
    pub average_relevance_score: f64,
    pub cache_hit_rate: f64,
}

/// Configuration for episodic memory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodicMemoryConfig {
    /// Maximum number of memories to store
    pub max_memories: usize,
    
    /// Memory strength decay rate
    pub decay_rate: f64,
    
    /// Consolidation interval
    pub consolidation_interval: Duration,
    
    /// Minimum importance threshold for storage
    pub min_importance_threshold: f64,
    
    /// Enable emotional weighting
    pub emotional_weighting_enabled: bool,
    
    /// Enable temporal clustering
    pub temporal_clustering_enabled: bool,
    
    /// Forgetting threshold
    pub forgetting_threshold: f64,
}

impl Default for EpisodicMemoryConfig {
    fn default() -> Self {
        Self {
            max_memories: 10000,
            decay_rate: 0.01,
            consolidation_interval: Duration::from_secs(300), // 5 minutes
            min_importance_threshold: 0.3,
            emotional_weighting_enabled: true,
            temporal_clustering_enabled: true,
            forgetting_threshold: 0.1,
        }
    }
}

impl EpisodicMemory {
    /// Create a new episodic memory system
    pub async fn new() -> Result<Self, ConsciousnessError> {
        let config = EpisodicMemoryConfig::default();
        
        Ok(Self {
            memories: Arc::new(RwLock::new(HashMap::new())),
            temporal_index: Arc::new(RwLock::new(BTreeMap::new())),
            concept_index: Arc::new(RwLock::new(HashMap::new())),
            emotional_index: Arc::new(RwLock::new(HashMap::new())),
            consolidation_manager: Arc::new(RwLock::new(ConsolidationManager {
                pending_consolidation: Vec::new(),
                consolidation_schedule: BTreeMap::new(),
                last_consolidation: SystemTime::now(),
                consolidation_stats: ConsolidationStats {
                    total_consolidated: 0,
                    total_forgotten: 0,
                    average_consolidation_time: Duration::from_millis(0),
                    consolidation_success_rate: 1.0,
                },
            })),
            statistics: Arc::new(RwLock::new(MemoryStatistics {
                total_memories: 0,
                by_consolidation_state: HashMap::new(),
                average_memory_strength: 0.0,
                total_size_bytes: 0,
                retrieval_stats: RetrievalStatistics {
                    total_retrievals: 0,
                    average_retrieval_time: Duration::from_millis(0),
                    average_relevance_score: 0.0,
                    cache_hit_rate: 0.0,
                },
                forgetting_curve: Vec::new(),
            })),
            config,
        })
    }
    
    /// Store a new experience in episodic memory
    pub async fn store_experience(
        &mut self,
        input: &str,
        response: &ConsciousnessResponse,
        consciousness_state: &ConsciousnessState,
    ) -> Result<Uuid, ConsciousnessError> {
        let memory_id = Uuid::new_v4();
        let now = SystemTime::now();
        
        // Calculate importance score
        let importance_score = self.calculate_importance_score(
            input,
            response,
            consciousness_state,
        ).await?;
        
        // Skip storage if below importance threshold
        if importance_score < self.config.min_importance_threshold {
            return Ok(memory_id); // Return ID but don't actually store
        }
        
        // Extract concepts from input and response
        let associated_concepts = self.extract_concepts(input, &response.content).await?;
        
        // Create temporal context
        let temporal_context = TemporalContext {
            timestamp: now,
            relative_time: self.determine_relative_time(now).await?,
            duration: response.processing_time,
            sequence_number: self.get_next_sequence_number().await?,
            related_events: Vec::new(),
        };
        
        // Create memory entry
        let memory_entry = EpisodicMemoryEntry {
            id: memory_id,
            input: input.to_string(),
            response: response.content.clone(),
            consciousness_state: consciousness_state.clone(),
            emotional_context: response.emotional_context.clone(),
            temporal_context,
            associated_concepts: associated_concepts.clone(),
            memory_strength: 1.0, // Fresh memory starts at full strength
            access_count: 0,
            last_accessed: now,
            created_at: now,
            tags: self.generate_memory_tags(input, response).await?,
            importance_score,
            sensory_data: self.extract_sensory_data(input, response).await?,
        };
        
        // Store in main memory
        {
            let mut memories = self.memories.write().await;
            memories.insert(memory_id, memory_entry.clone());
        }
        
        // Update indexes
        self.update_temporal_index(memory_id, now).await?;
        self.update_concept_index(memory_id, &associated_concepts).await?;
        self.update_emotional_index(memory_id, &response.emotional_context).await?;
        
        // Schedule for consolidation
        {
            let mut consolidation = self.consolidation_manager.write().await;
            consolidation.pending_consolidation.push(memory_id);
        }
        
        // Update statistics
        self.update_storage_statistics().await?;
        
        // Trigger cleanup if needed
        if self.should_trigger_cleanup().await? {
            self.cleanup_old_memories().await?;
        }
        
        Ok(memory_id)
    }
    
    /// Retrieve relevant experiences based on input
    pub async fn retrieve_relevant_experiences(&self, input: &str) -> Result<EpisodicContext, ConsciousnessError> {
        let start_time = std::time::Instant::now();
        
        // Extract concepts from input for semantic search
        let input_concepts = self.extract_concepts_from_text(input).await?;
        
        // Perform multi-modal retrieval
        let semantic_matches = self.retrieve_by_concepts(&input_concepts).await?;
        let temporal_matches = self.retrieve_by_temporal_proximity().await?;
        let emotional_matches = self.retrieve_by_emotional_similarity(input).await?;
        
        // Combine and rank results
        let combined_results = self.combine_retrieval_results(
            semantic_matches,
            temporal_matches,
            emotional_matches,
        ).await?;
        
        // Select top relevant experiences
        let top_experiences = self.select_top_experiences(combined_results, 10).await?;
        
        // Calculate relevance scores
        let relevance_scores = self.calculate_relevance_scores(&top_experiences, input).await?;
        
        // Update access statistics
        self.update_access_statistics(&top_experiences).await?;
        
        let retrieval_time = start_time.elapsed();
        
        // Update retrieval statistics
        {
            let mut stats = self.statistics.write().await;
            stats.retrieval_stats.total_retrievals += 1;
            let total = stats.retrieval_stats.total_retrievals;
            stats.retrieval_stats.average_retrieval_time = 
                (stats.retrieval_stats.average_retrieval_time * (total - 1) + retrieval_time) / total;
        }
        
        Ok(EpisodicContext {
            relevant_experiences: top_experiences.iter().map(|e| e.response.clone()).collect(),
            relevance_score: relevance_scores.iter().sum::<f64>() / relevance_scores.len() as f64,
            temporal_context: top_experiences.iter()
                .map(|e| format!("Experience from {}", self.format_timestamp(e.created_at)))
                .collect(),
        })
    }
    
    /// Store large data for stress testing
    pub async fn store_large_data(&mut self, key: &str, data: &str) -> Result<(), ConsciousnessError> {
        // Create a large memory entry for testing
        let memory_id = Uuid::new_v4();
        let now = SystemTime::now();
        
        let large_memory = EpisodicMemoryEntry {
            id: memory_id,
            input: key.to_string(),
            response: data.to_string(),
            consciousness_state: ConsciousnessState {
                awareness_level: 0.8,
                emotional_state: EmotionalState {
                    primary_emotion: EmotionType::Calm,
                    intensity: 0.5,
                    valence: 0.0,
                    arousal: 0.3,
                    secondary_emotions: Vec::new(),
                },
                cognitive_load: 0.3,
                confidence_score: 0.8,
                meta_cognitive_depth: 2,
                timestamp: now,
            },
            emotional_context: EmotionalContext {
                user_emotions: Vec::new(),
                engine_emotions: EmotionalState {
                    primary_emotion: EmotionType::Calm,
                    intensity: 0.5,
                    valence: 0.0,
                    arousal: 0.3,
                    secondary_emotions: Vec::new(),
                },
                empathy_alignment: 0.8,
                appropriateness_score: 0.9,
            },
            temporal_context: TemporalContext {
                timestamp: now,
                relative_time: None,
                duration: Duration::from_millis(100),
                sequence_number: 0,
                related_events: Vec::new(),
            },
            associated_concepts: vec!["large_data".to_string(), "stress_test".to_string()],
            memory_strength: 1.0,
            access_count: 0,
            last_accessed: now,
            created_at: now,
            tags: vec!["large".to_string(), "test".to_string()],
            importance_score: 0.5,
            sensory_data: None,
        };
        
        let mut memories = self.memories.write().await;
        memories.insert(memory_id, large_memory);
        
        Ok(())
    }
    
    /// Get memory size for monitoring
    pub async fn get_memory_size(&self) -> Result<u64, ConsciousnessError> {
        let stats = self.statistics.read().await;
        Ok(stats.total_size_bytes)
    }
    
    /// Cleanup old memories
    pub async fn cleanup_old_memories(&mut self) -> Result<(), ConsciousnessError> {
        let mut memories_to_remove = Vec::new();
        
        // Identify memories to remove based on strength and age
        {
            let memories = self.memories.read().await;
            let now = SystemTime::now();
            
            for (id, memory) in memories.iter() {
                let age = now.duration_since(memory.created_at)
                    .unwrap_or(Duration::from_secs(0));
                
                // Apply forgetting curve
                let decayed_strength = memory.memory_strength * 
                    (-self.config.decay_rate * age.as_secs_f64()).exp();
                
                if decayed_strength < self.config.forgetting_threshold {
                    memories_to_remove.push(*id);
                }
            }
        }
        
        // Remove identified memories
        {
            let mut memories = self.memories.write().await;
            for id in &memories_to_remove {
                memories.remove(id);
            }
        }
        
        // Update indexes
        self.cleanup_indexes(&memories_to_remove).await?;
        
        // Update statistics
        {
            let mut stats = self.statistics.write().await;
            stats.total_memories = stats.total_memories.saturating_sub(memories_to_remove.len() as u64);
        }
        
        Ok(())
    }
    
    // Helper methods
    
    async fn calculate_importance_score(
        &self,
        input: &str,
        response: &ConsciousnessResponse,
        consciousness_state: &ConsciousnessState,
    ) -> Result<f64, ConsciousnessError> {
        let mut importance = 0.0;
        
        // Factor in consciousness quality
        importance += consciousness_state.awareness_level * 0.3;
        
        // Factor in emotional intensity
        importance += response.emotional_context.engine_emotions.intensity * 0.2;
        
        // Factor in response confidence
        importance += response.confidence_level * 0.2;
        
        // Factor in creativity and empathy scores
        importance += response.creativity_score * 0.15;
        importance += response.empathy_score * 0.15;
        
        Ok(importance.min(1.0))
    }
    
    async fn extract_concepts(&self, input: &str, response: &str) -> Result<Vec<String>, ConsciousnessError> {
        let mut concepts = Vec::new();
        
        // Simple keyword extraction (in production, use NLP)
        let text = format!("{} {}", input, response);
        let words: Vec<&str> = text.split_whitespace().collect();
        
        for word in words {
            let clean_word = word.to_lowercase()
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<String>();
            
            if clean_word.len() > 3 && !self.is_stop_word(&clean_word) {
                concepts.push(clean_word);
            }
        }
        
        // Remove duplicates
        concepts.sort();
        concepts.dedup();
        
        Ok(concepts)
    }
    
    async fn extract_concepts_from_text(&self, text: &str) -> Result<Vec<String>, ConsciousnessError> {
        self.extract_concepts(text, "").await
    }
    
    fn is_stop_word(&self, word: &str) -> bool {
        matches!(word, "the" | "and" | "or" | "but" | "in" | "on" | "at" | "to" | "for" | "of" | "with" | "by")
    }
    
    async fn determine_relative_time(&self, _timestamp: SystemTime) -> Result<Option<String>, ConsciousnessError> {
        // Simple relative time determination
        Ok(Some("recent".to_string()))
    }
    
    async fn get_next_sequence_number(&self) -> Result<u32, ConsciousnessError> {
        let stats = self.statistics.read().await;
        Ok(stats.total_memories as u32 + 1)
    }
    
    async fn generate_memory_tags(&self, input: &str, response: &ConsciousnessResponse) -> Result<Vec<String>, ConsciousnessError> {
        let mut tags = Vec::new();
        
        // Add emotion-based tags
        tags.push(format!("emotion_{:?}", response.emotional_context.engine_emotions.primary_emotion).to_lowercase());
        
        // Add confidence-based tags
        if response.confidence_level > 0.8 {
            tags.push("high_confidence".to_string());
        } else if response.confidence_level < 0.5 {
            tags.push("low_confidence".to_string());
        }
        
        // Add length-based tags
        if input.len() > 100 {
            tags.push("long_input".to_string());
        }
        
        if response.content.len() > 200 {
            tags.push("long_response".to_string());
        }
        
        Ok(tags)
    }
    
    async fn extract_sensory_data(&self, input: &str, response: &ConsciousnessResponse) -> Result<Option<SensoryData>, ConsciousnessError> {
        // Extract textual patterns
        let textual_patterns = vec![
            format!("input_length_{}", input.len()),
            format!("response_length_{}", response.content.len()),
        ];
        
        // Create emotional imprints
        let mut emotional_imprints = HashMap::new();
        emotional_imprints.insert(
            response.emotional_context.engine_emotions.primary_emotion,
            response.emotional_context.engine_emotions.intensity,
        );
        
        Ok(Some(SensoryData {
            visual: None,
            auditory: None,
            textual_patterns,
            emotional_imprints,
        }))
    }
    
    async fn update_temporal_index(&self, memory_id: Uuid, timestamp: SystemTime) -> Result<(), ConsciousnessError> {
        let timestamp_key = timestamp.duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        
        let mut index = self.temporal_index.write().await;
        index.entry(timestamp_key).or_insert_with(Vec::new).push(memory_id);
        
        Ok(())
    }
    
    async fn update_concept_index(&self, memory_id: Uuid, concepts: &[String]) -> Result<(), ConsciousnessError> {
        let mut index = self.concept_index.write().await;
        
        for concept in concepts {
            index.entry(concept.clone()).or_insert_with(Vec::new).push(memory_id);
        }
        
        Ok(())
    }
    
    async fn update_emotional_index(&self, memory_id: Uuid, emotional_context: &EmotionalContext) -> Result<(), ConsciousnessError> {
        let mut index = self.emotional_index.write().await;
        
        let primary_emotion = emotional_context.engine_emotions.primary_emotion;
        index.entry(primary_emotion).or_insert_with(Vec::new).push(memory_id);
        
        Ok(())
    }
    
    async fn update_storage_statistics(&self) -> Result<(), ConsciousnessError> {
        let mut stats = self.statistics.write().await;
        stats.total_memories += 1;
        
        // Update size estimate (rough calculation)
        stats.total_size_bytes += 1024; // Approximate size per memory
        
        Ok(())
    }
    
    async fn should_trigger_cleanup(&self) -> Result<bool, ConsciousnessError> {
        let stats = self.statistics.read().await;
        Ok(stats.total_memories > self.config.max_memories as u64)
    }
    
    async fn retrieve_by_concepts(&self, concepts: &[String]) -> Result<Vec<Uuid>, ConsciousnessError> {
        let index = self.concept_index.read().await;
        let mut results = Vec::new();
        
        for concept in concepts {
            if let Some(memory_ids) = index.get(concept) {
                results.extend(memory_ids.iter().cloned());
            }
        }
        
        // Remove duplicates
        results.sort();
        results.dedup();
        
        Ok(results)
    }
    
    async fn retrieve_by_temporal_proximity(&self) -> Result<Vec<Uuid>, ConsciousnessError> {
        let index = self.temporal_index.read().await;
        let now = SystemTime::now().duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        
        let mut results = Vec::new();
        
        // Get recent memories (last hour)
        let one_hour_ago = now.saturating_sub(3600);
        for (&timestamp, memory_ids) in index.range(one_hour_ago..) {
            results.extend(memory_ids.iter().cloned());
        }
        
        Ok(results)
    }
    
    async fn retrieve_by_emotional_similarity(&self, _input: &str) -> Result<Vec<Uuid>, ConsciousnessError> {
        // For now, return empty - would implement emotion detection from input
        Ok(Vec::new())
    }
    
    async fn combine_retrieval_results(
        &self,
        semantic: Vec<Uuid>,
        temporal: Vec<Uuid>,
        emotional: Vec<Uuid>,
    ) -> Result<Vec<Uuid>, ConsciousnessError> {
        let mut combined = Vec::new();
        combined.extend(semantic);
        combined.extend(temporal);
        combined.extend(emotional);
        
        // Remove duplicates while preserving order
        let mut seen = std::collections::HashSet::new();
        combined.retain(|id| seen.insert(*id));
        
        Ok(combined)
    }
    
    async fn select_top_experiences(&self, memory_ids: Vec<Uuid>, limit: usize) -> Result<Vec<EpisodicMemoryEntry>, ConsciousnessError> {
        let memories = self.memories.read().await;
        let mut experiences = Vec::new();
        
        for id in memory_ids.iter().take(limit) {
            if let Some(memory) = memories.get(id) {
                experiences.push(memory.clone());
            }
        }
        
        // Sort by importance and recency
        experiences.sort_by(|a, b| {
            let score_a = a.importance_score + (1.0 / (a.created_at.elapsed().unwrap_or(Duration::from_secs(1)).as_secs_f64() + 1.0));
            let score_b = b.importance_score + (1.0 / (b.created_at.elapsed().unwrap_or(Duration::from_secs(1)).as_secs_f64() + 1.0));
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        Ok(experiences)
    }
    
    async fn calculate_relevance_scores(&self, experiences: &[EpisodicMemoryEntry], input: &str) -> Result<Vec<f64>, ConsciousnessError> {
        let input_concepts = self.extract_concepts_from_text(input).await?;
        let mut scores = Vec::new();
        
        for experience in experiences {
            let mut relevance = 0.0;
            
            // Concept overlap
            let common_concepts = experience.associated_concepts.iter()
                .filter(|concept| input_concepts.contains(concept))
                .count();
            
            if !experience.associated_concepts.is_empty() {
                relevance += (common_concepts as f64 / experience.associated_concepts.len() as f64) * 0.6;
            }
            
            // Recency bonus
            let age = experience.created_at.elapsed().unwrap_or(Duration::from_secs(0));
            let recency_score = 1.0 / (1.0 + age.as_secs_f64() / 3600.0); // Decay over hours
            relevance += recency_score * 0.2;
            
            // Importance score
            relevance += experience.importance_score * 0.2;
            
            scores.push(relevance.min(1.0));
        }
        
        Ok(scores)
    }
    
    async fn update_access_statistics(&self, experiences: &[EpisodicMemoryEntry]) -> Result<(), ConsciousnessError> {
        let mut memories = self.memories.write().await;
        let now = SystemTime::now();
        
        for experience in experiences {
            if let Some(memory) = memories.get_mut(&experience.id) {
                memory.access_count += 1;
                memory.last_accessed = now;
                
                // Strengthen memory based on access
                memory.memory_strength = (memory.memory_strength + 0.1).min(1.0);
            }
        }
        
        Ok(())
    }
    
    fn format_timestamp(&self, timestamp: SystemTime) -> String {
        match timestamp.duration_since(UNIX_EPOCH) {
            Ok(duration) => format!("{} seconds ago", duration.as_secs()),
            Err(_) => "unknown time".to_string(),
        }
    }
    
    async fn cleanup_indexes(&self, removed_ids: &[Uuid]) -> Result<(), ConsciousnessError> {
        // Clean temporal index
        {
            let mut temporal_index = self.temporal_index.write().await;
            for (_, memory_ids) in temporal_index.iter_mut() {
                memory_ids.retain(|id| !removed_ids.contains(id));
            }
        }
        
        // Clean concept index
        {
            let mut concept_index = self.concept_index.write().await;
            for (_, memory_ids) in concept_index.iter_mut() {
                memory_ids.retain(|id| !removed_ids.contains(id));
            }
        }
        
        // Clean emotional index
        {
            let mut emotional_index = self.emotional_index.write().await;
            for (_, memory_ids) in emotional_index.iter_mut() {
                memory_ids.retain(|id| !removed_ids.contains(id));
            }
        }
        
        Ok(())
    }
}