//! Memory Manager - Unified interface for all memory systems

use crate::{
    types::*,
    error::{ConsciousnessError, ConsciousnessResult},
    memory::{
        episodic_memory::{EpisodicMemorySystem, EpisodicMemory, Experience},
        semantic_memory::{SemanticMemorySystem, Concept},
    },
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tracing::{debug, info, warn};

/// Unified memory manager coordinating all memory systems
#[derive(Debug, Clone)]
pub struct MemoryManager {
    episodic_memory: EpisodicMemorySystem,
    semantic_memory: SemanticMemorySystem,
    working_memory: WorkingMemory,
    memory_consolidation_scheduler: ConsolidationScheduler,
    memory_retrieval_optimizer: RetrievalOptimizer,
    cross_memory_associations: CrossMemoryAssociations,
}

impl MemoryManager {
    /// Create a new memory manager
    pub fn new() -> Self {
        info!("Initializing Memory Manager");
        
        Self {
            episodic_memory: EpisodicMemorySystem::new(),
            semantic_memory: SemanticMemorySystem::new(),
            working_memory: WorkingMemory::new(),
            memory_consolidation_scheduler: ConsolidationScheduler::new(),
            memory_retrieval_optimizer: RetrievalOptimizer::new(),
            cross_memory_associations: CrossMemoryAssociations::new(),
        }
    }
    
    /// Store a new experience in episodic memory
    pub async fn store_experience(
        &mut self,
        experience: Experience,
        emotional_context: EmotionalContext,
        consciousness_state: ConsciousnessState,
    ) -> ConsciousnessResult<Uuid> {
        debug!("Storing experience in memory manager");
        
        // Store in episodic memory
        let memory_id = self.episodic_memory.store_memory(
            experience.clone(),
            emotional_context.clone(),
            consciousness_state,
        ).await?;
        
        // Extract semantic knowledge from experience
        if experience.impact_score > 0.6 {
            self.extract_semantic_knowledge_from_experience(&experience).await?;
        }
        
        // Update working memory with recent experience
        self.working_memory.add_recent_experience(experience.clone()).await?;
        
        // Create cross-memory associations
        self.cross_memory_associations.create_associations_for_experience(
            &experience,
            memory_id,
        ).await?;
        
        info!("Experience stored with memory ID: {}", memory_id);
        
        Ok(memory_id)
    }
    
    /// Store semantic knowledge
    pub async fn store_knowledge(
        &mut self,
        concept_name: String,
        definition: String,
        properties: HashMap<String, String>,
        confidence: f64,
    ) -> ConsciousnessResult<()> {
        debug!("Storing semantic knowledge: {}", concept_name);
        
        // Store in semantic memory
        self.semantic_memory.store_concept(
            concept_name.clone(),
            definition,
            properties,
            confidence,
        ).await?;
        
        // Update working memory if concept is relevant
        if confidence > 0.7 {
            self.working_memory.add_active_concept(concept_name).await?;
        }
        
        Ok(())
    }
    
    /// Retrieve relevant memories for a given context
    pub async fn retrieve_relevant_memories(
        &mut self,
        context: &MemoryRetrievalContext,
    ) -> ConsciousnessResult<RetrievedMemories> {
        debug!("Retrieving relevant memories for context: {:?}", context.context_type);
        
        // Retrieve from episodic memory
        let episodic_memories = if context.include_episodic {
            let retrieval_context = self.convert_to_episodic_context(context);
            self.episodic_memory.retrieve_memories(&retrieval_context, context.max_episodic_results).await?
        } else {
            Vec::new()
        };
        
        // Retrieve from semantic memory
        let semantic_concepts = if context.include_semantic && !context.query.is_empty() {
            self.semantic_memory.search_concepts(&context.query, context.max_semantic_results).await?
        } else {
            Vec::new()
        };
        
        // Get working memory contents
        let working_memory_contents = self.working_memory.get_current_contents().await?;
        
        // Optimize retrieval results
        let optimized_results = self.memory_retrieval_optimizer.optimize_results(
            episodic_memories,
            semantic_concepts,
            working_memory_contents,
            context,
        ).await?;
        
        debug!("Retrieved {} episodic memories, {} semantic concepts", 
               optimized_results.episodic_memories.len(),
               optimized_results.semantic_concepts.len());
        
        Ok(optimized_results)
    }
    
    /// Learn from text input
    pub async fn learn_from_text(
        &mut self,
        text: &str,
        context: &str,
    ) -> ConsciousnessResult<LearningReport> {
        debug!("Learning from text: {} characters", text.len());
        
        // Learn semantic knowledge
        let semantic_learning = self.semantic_memory.learn_from_text(text, context).await?;
        
        // Create experience from learning
        let learning_experience = Experience {
            experience_id: Uuid::new_v4(),
            experience_type: "learning".to_string(),
            description: format!("Learned from text: {}", text.chars().take(100).collect::<String>()),
            context: context.to_string(),
            outcome: format!("Extracted {} concepts", semantic_learning.extracted_concepts.len()),
            lessons_learned: semantic_learning.extracted_concepts.iter()
                .map(|c| format!("Learned about: {}", c.name))
                .collect(),
            novelty_score: 0.7,
            impact_score: semantic_learning.confidence,
            occurred_at: Utc::now(),
        };
        
        // Store learning experience
        let emotional_context = EmotionalContext {
            primary_emotion: "curiosity".to_string(),
            emotional_intensity: 0.6,
            emotional_valence: 0.7,
            emotional_arousal: 0.5,
            contextual_factors: vec!["learning".to_string()],
        };
        
        let consciousness_state = ConsciousnessState::new(); // Would be passed from engine
        
        let experience_id = self.store_experience(
            learning_experience,
            emotional_context,
            consciousness_state,
        ).await?;
        
        let report = LearningReport {
            concepts_learned: semantic_learning.extracted_concepts.len(),
            relationships_discovered: semantic_learning.extracted_relationships.len(),
            experience_id,
            learning_confidence: semantic_learning.confidence,
            learning_time: Utc::now(),
        };
        
        info!("Learning completed: {:?}", report);
        
        Ok(report)
    }
    
    /// Consolidate all memory systems
    pub async fn consolidate_memories(&mut self) -> ConsciousnessResult<MemoryConsolidationReport> {
        info!("Starting comprehensive memory consolidation");
        
        // Consolidate episodic memories
        let episodic_report = self.episodic_memory.consolidate_memories().await?;
        
        // Consolidate semantic knowledge
        let semantic_report = self.semantic_memory.consolidate_knowledge().await?;
        
        // Update cross-memory associations
        let association_report = self.cross_memory_associations.consolidate_associations().await?;
        
        // Clear working memory of old items
        let working_memory_report = self.working_memory.cleanup_old_items().await?;
        
        let comprehensive_report = MemoryConsolidationReport {
            episodic_consolidation: episodic_report,
            semantic_consolidation: semantic_report,
            association_updates: association_report,
            working_memory_cleanup: working_memory_report,
            consolidation_time: Utc::now(),
        };
        
        info!("Memory consolidation completed: {:?}", comprehensive_report);
        
        Ok(comprehensive_report)
    }
    
    /// Get comprehensive memory statistics
    pub fn get_memory_statistics(&self) -> MemoryStatistics {
        let episodic_stats = self.episodic_memory.get_memory_statistics();
        let semantic_stats = self.semantic_memory.get_statistics();
        let working_memory_stats = self.working_memory.get_statistics();
        
        MemoryStatistics {
            episodic_memory_count: episodic_stats.total_memories,
            semantic_concepts_count: semantic_stats.total_concepts,
            working_memory_items: working_memory_stats.active_items,
            total_associations: self.cross_memory_associations.get_association_count(),
            memory_strength_average: (
                episodic_stats.average_significance + 
                semantic_stats.average_confidence
            ) / 2.0,
            oldest_memory: episodic_stats.oldest_memory,
            most_accessed_concept: semantic_stats.frequently_used_concepts,
        }
    }
    
    /// Extract semantic knowledge from experience
    async fn extract_semantic_knowledge_from_experience(
        &mut self,
        experience: &Experience,
    ) -> ConsciousnessResult<()> {
        // Simple knowledge extraction from experience description
        let words: Vec<&str> = experience.description.split_whitespace().collect();
        
        for word in words {
            if word.len() > 4 && word.chars().next().unwrap().is_uppercase() {
                // Potential concept
                let mut properties = HashMap::new();
                properties.insert("source".to_string(), "experience".to_string());
                properties.insert("experience_type".to_string(), experience.experience_type.clone());
                
                self.semantic_memory.store_concept(
                    word.to_string(),
                    format!("Concept from experience: {}", word),
                    properties,
                    0.6,
                ).await?;
            }
        }
        
        Ok(())
    }
    
    /// Convert memory retrieval context to episodic context
    fn convert_to_episodic_context(&self, context: &MemoryRetrievalContext) -> crate::memory::episodic_memory::RetrievalContext {
        crate::memory::episodic_memory::RetrievalContext {
            query_type: context.context_type.clone(),
            emotional_context: context.emotional_context.clone(),
            tags: context.tags.clone(),
            time_range: context.time_range,
            significance_threshold: context.significance_threshold,
        }
    }
}

/// Working memory for temporary, active information
#[derive(Debug, Clone)]
struct WorkingMemory {
    active_concepts: Vec<String>,
    recent_experiences: Vec<Experience>,
    current_context: HashMap<String, String>,
    attention_focus: Vec<String>,
    max_capacity: usize,
}

impl WorkingMemory {
    fn new() -> Self {
        Self {
            active_concepts: Vec::new(),
            recent_experiences: Vec::new(),
            current_context: HashMap::new(),
            attention_focus: Vec::new(),
            max_capacity: 20, // Limited capacity like human working memory
        }
    }
    
    async fn add_recent_experience(&mut self, experience: Experience) -> ConsciousnessResult<()> {
        self.recent_experiences.push(experience);
        
        // Maintain capacity limit
        if self.recent_experiences.len() > self.max_capacity {
            self.recent_experiences.remove(0);
        }
        
        Ok(())
    }
    
    async fn add_active_concept(&mut self, concept: String) -> ConsciousnessResult<()> {
        if !self.active_concepts.contains(&concept) {
            self.active_concepts.push(concept);
            
            // Maintain capacity limit
            if self.active_concepts.len() > self.max_capacity {
                self.active_concepts.remove(0);
            }
        }
        
        Ok(())
    }
    
    async fn get_current_contents(&self) -> ConsciousnessResult<WorkingMemoryContents> {
        Ok(WorkingMemoryContents {
            active_concepts: self.active_concepts.clone(),
            recent_experiences: self.recent_experiences.clone(),
            current_context: self.current_context.clone(),
            attention_focus: self.attention_focus.clone(),
        })
    }
    
    async fn cleanup_old_items(&mut self) -> ConsciousnessResult<WorkingMemoryCleanupReport> {
        let initial_concepts = self.active_concepts.len();
        let initial_experiences = self.recent_experiences.len();
        
        // Remove old experiences (keep only last 10)
        if self.recent_experiences.len() > 10 {
            self.recent_experiences.drain(0..self.recent_experiences.len() - 10);
        }
        
        // Remove less relevant concepts (keep only last 15)
        if self.active_concepts.len() > 15 {
            self.active_concepts.drain(0..self.active_concepts.len() - 15);
        }
        
        Ok(WorkingMemoryCleanupReport {
            concepts_removed: initial_concepts - self.active_concepts.len(),
            experiences_removed: initial_experiences - self.recent_experiences.len(),
        })
    }
    
    fn get_statistics(&self) -> WorkingMemoryStatistics {
        WorkingMemoryStatistics {
            active_items: self.active_concepts.len() + self.recent_experiences.len(),
            capacity_utilization: (self.active_concepts.len() + self.recent_experiences.len()) as f64 / (self.max_capacity * 2) as f64,
        }
    }
}

/// Cross-memory associations for linking different memory types
#[derive(Debug, Clone)]
struct CrossMemoryAssociations {
    associations: HashMap<String, Vec<MemoryAssociation>>,
}

impl CrossMemoryAssociations {
    fn new() -> Self {
        Self {
            associations: HashMap::new(),
        }
    }
    
    async fn create_associations_for_experience(
        &mut self,
        experience: &Experience,
        memory_id: Uuid,
    ) -> ConsciousnessResult<()> {
        // Create associations based on experience content
        let key = format!("experience:{}", memory_id);
        let mut associations = Vec::new();
        
        // Associate with concepts mentioned in experience
        let words: Vec<&str> = experience.description.split_whitespace().collect();
        for word in words {
            if word.len() > 4 {
                associations.push(MemoryAssociation {
                    association_type: "mentions".to_string(),
                    target_type: "concept".to_string(),
                    target_id: word.to_string(),
                    strength: 0.5,
                });
            }
        }
        
        self.associations.insert(key, associations);
        
        Ok(())
    }
    
    async fn consolidate_associations(&mut self) -> ConsciousnessResult<AssociationConsolidationReport> {
        let initial_count = self.associations.len();
        
        // Remove weak associations
        for associations in self.associations.values_mut() {
            associations.retain(|a| a.strength > 0.2);
        }
        
        // Remove empty association lists
        self.associations.retain(|_, associations| !associations.is_empty());
        
        let final_count = self.associations.len();
        
        Ok(AssociationConsolidationReport {
            associations_before: initial_count,
            associations_after: final_count,
            associations_removed: initial_count - final_count,
        })
    }
    
    fn get_association_count(&self) -> usize {
        self.associations.values().map(|v| v.len()).sum()
    }
}

/// Memory consolidation scheduler
#[derive(Debug, Clone)]
struct ConsolidationScheduler {
    last_consolidation: Option<DateTime<Utc>>,
    consolidation_interval: chrono::Duration,
}

impl ConsolidationScheduler {
    fn new() -> Self {
        Self {
            last_consolidation: None,
            consolidation_interval: chrono::Duration::hours(6),
        }
    }
}

/// Memory retrieval optimizer
#[derive(Debug, Clone)]
struct RetrievalOptimizer;

impl RetrievalOptimizer {
    fn new() -> Self { Self }
    
    async fn optimize_results(
        &self,
        episodic_memories: Vec<EpisodicMemory>,
        semantic_concepts: Vec<crate::memory::semantic_memory::ConceptSearchResult>,
        working_memory: WorkingMemoryContents,
        _context: &MemoryRetrievalContext,
    ) -> ConsciousnessResult<RetrievedMemories> {
        Ok(RetrievedMemories {
            episodic_memories,
            semantic_concepts: semantic_concepts.into_iter().map(|r| r.concept).collect(),
            working_memory_concepts: working_memory.active_concepts,
            recent_experiences: working_memory.recent_experiences,
            retrieval_confidence: 0.8,
        })
    }
}

/// Data structures
#[derive(Debug, Clone)]
pub struct MemoryRetrievalContext {
    pub context_type: String,
    pub query: String,
    pub emotional_context: EmotionalContext,
    pub tags: Vec<String>,
    pub time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    pub significance_threshold: f64,
    pub include_episodic: bool,
    pub include_semantic: bool,
    pub max_episodic_results: usize,
    pub max_semantic_results: usize,
}

#[derive(Debug, Clone)]
pub struct RetrievedMemories {
    pub episodic_memories: Vec<EpisodicMemory>,
    pub semantic_concepts: Vec<Concept>,
    pub working_memory_concepts: Vec<String>,
    pub recent_experiences: Vec<Experience>,
    pub retrieval_confidence: f64,
}

#[derive(Debug, Clone)]
pub struct LearningReport {
    pub concepts_learned: usize,
    pub relationships_discovered: usize,
    pub experience_id: Uuid,
    pub learning_confidence: f64,
    pub learning_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct MemoryConsolidationReport {
    pub episodic_consolidation: crate::memory::episodic_memory::ConsolidationReport,
    pub semantic_consolidation: crate::memory::semantic_memory::ConsolidationReport,
    pub association_updates: AssociationConsolidationReport,
    pub working_memory_cleanup: WorkingMemoryCleanupReport,
    pub consolidation_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct MemoryStatistics {
    pub episodic_memory_count: usize,
    pub semantic_concepts_count: usize,
    pub working_memory_items: usize,
    pub total_associations: usize,
    pub memory_strength_average: f64,
    pub oldest_memory: Option<DateTime<Utc>>,
    pub most_accessed_concept: usize,
}

#[derive(Debug, Clone)]
struct WorkingMemoryContents {
    active_concepts: Vec<String>,
    recent_experiences: Vec<Experience>,
    current_context: HashMap<String, String>,
    attention_focus: Vec<String>,
}

#[derive(Debug, Clone)]
struct WorkingMemoryStatistics {
    active_items: usize,
    capacity_utilization: f64,
}

#[derive(Debug, Clone)]
struct WorkingMemoryCleanupReport {
    concepts_removed: usize,
    experiences_removed: usize,
}

#[derive(Debug, Clone)]
struct MemoryAssociation {
    association_type: String,
    target_type: String,
    target_id: String,
    strength: f64,
}

#[derive(Debug, Clone)]
struct AssociationConsolidationReport {
    associations_before: usize,
    associations_after: usize,
    associations_removed: usize,
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_manager_creation() {
        let memory_manager = MemoryManager::new();
        let stats = memory_manager.get_memory_statistics();
        
        assert_eq!(stats.episodic_memory_count, 0);
        assert_eq!(stats.semantic_concepts_count, 0);
    }
    
    #[tokio::test]
    async fn test_experience_storage() {
        let mut memory_manager = MemoryManager::new();
        
        let experience = Experience {
            experience_id: Uuid::new_v4(),
            experience_type: "test".to_string(),
            description: "Test experience".to_string(),
            context: "testing".to_string(),
            outcome: "success".to_string(),
            lessons_learned: vec!["testing is important".to_string()],
            novelty_score: 0.8,
            impact_score: 0.7,
            occurred_at: Utc::now(),
        };
        
        let emotional_context = EmotionalContext {
            primary_emotion: "neutral".to_string(),
            emotional_intensity: 0.5,
            emotional_valence: 0.0,
            emotional_arousal: 0.3,
            contextual_factors: vec!["test".to_string()],
        };
        
        let consciousness_state = ConsciousnessState::new();
        
        let memory_id = memory_manager.store_experience(
            experience,
            emotional_context,
            consciousness_state,
        ).await;
        
        assert!(memory_id.is_ok());
        
        let stats = memory_manager.get_memory_statistics();
        assert_eq!(stats.episodic_memory_count, 1);
    }
}