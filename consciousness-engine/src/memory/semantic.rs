//! Semantic Memory System - Knowledge Representation and Retrieval
//! 
//! This module implements a sophisticated semantic memory system for storing and
//! retrieving factual knowledge, concepts, and their relationships.

use crate::types::*;
use crate::error::ConsciousnessError;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Semantic knowledge entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticKnowledgeEntry {
    /// Unique identifier
    pub id: Uuid,
    
    /// Concept or fact
    pub concept: String,
    
    /// Knowledge content
    pub content: String,
    
    /// Knowledge type
    pub knowledge_type: KnowledgeType,
    
    /// Confidence in this knowledge
    pub confidence: f64,
    
    /// Source of knowledge
    pub source: KnowledgeSource,
    
    /// Related concepts
    pub related_concepts: Vec<String>,
    
    /// Knowledge strength (how well established)
    pub strength: f64,
    
    /// Last updated
    pub last_updated: SystemTime,
    
    /// Creation time
    pub created_at: SystemTime,
    
    /// Access frequency
    pub access_frequency: u32,
    
    /// Verification status
    pub verification_status: VerificationStatus,
    
    /// Associated metadata
    pub metadata: HashMap<String, String>,
}

/// Types of knowledge
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum KnowledgeType {
    /// Factual information
    Factual,
    /// Procedural knowledge (how-to)
    Procedural,
    /// Conceptual understanding
    Conceptual,
    /// Relational knowledge
    Relational,
    /// Experiential knowledge
    Experiential,
    /// Theoretical knowledge
    Theoretical,
    /// Practical knowledge
    Practical,
}

/// Source of knowledge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KnowledgeSource {
    /// Learned from user interaction
    UserInteraction,
    /// Derived from reasoning
    Reasoning,
    /// Inferred from patterns
    Inference,
    /// External knowledge base
    External(String),
    /// Self-generated insight
    SelfGenerated,
    /// Consolidated from multiple sources
    Consolidated(Vec<KnowledgeSource>),
}

/// Verification status of knowledge
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VerificationStatus {
    /// Unverified knowledge
    Unverified,
    /// Partially verified
    PartiallyVerified,
    /// Fully verified
    Verified,
    /// Contradicted by other knowledge
    Contradicted,
    /// Uncertain or disputed
    Uncertain,
}

/// Knowledge graph node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeNode {
    /// Node identifier
    pub id: Uuid,
    
    /// Concept name
    pub concept: String,
    
    /// Node properties
    pub properties: HashMap<String, String>,
    
    /// Connected nodes
    pub connections: Vec<KnowledgeConnection>,
    
    /// Node importance score
    pub importance: f64,
    
    /// Centrality in knowledge graph
    pub centrality: f64,
}

/// Connection between knowledge nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeConnection {
    /// Target node ID
    pub target_id: Uuid,
    
    /// Relationship type
    pub relationship: RelationshipType,
    
    /// Connection strength
    pub strength: f64,
    
    /// Bidirectional connection
    pub bidirectional: bool,
}

/// Types of relationships between concepts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RelationshipType {
    /// Is-a relationship (inheritance)
    IsA,
    /// Part-of relationship
    PartOf,
    /// Causes relationship
    Causes,
    /// Similar to relationship
    SimilarTo,
    /// Opposite of relationship
    OppositeOf,
    /// Used for relationship
    UsedFor,
    /// Located at relationship
    LocatedAt,
    /// Temporal relationship
    Temporal(TemporalRelation),
    /// Custom relationship
    Custom(String),
}

/// Temporal relationships
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TemporalRelation {
    Before,
    After,
    During,
    Simultaneous,
}

/// Semantic Memory System
pub struct SemanticMemory {
    /// Knowledge entries storage
    knowledge_base: Arc<RwLock<HashMap<Uuid, SemanticKnowledgeEntry>>>,
    
    /// Concept index for fast lookup
    concept_index: Arc<RwLock<HashMap<String, Vec<Uuid>>>>,
    
    /// Knowledge graph
    knowledge_graph: Arc<RwLock<HashMap<Uuid, KnowledgeNode>>>,
    
    /// Relationship index
    relationship_index: Arc<RwLock<HashMap<RelationshipType, Vec<(Uuid, Uuid)>>>>,
    
    /// Knowledge statistics
    statistics: Arc<RwLock<SemanticMemoryStatistics>>,
    
    /// Configuration
    config: SemanticMemoryConfig,
}

/// Semantic memory statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticMemoryStatistics {
    /// Total knowledge entries
    pub total_entries: u64,
    
    /// Knowledge by type
    pub by_type: HashMap<KnowledgeType, u64>,
    
    /// Knowledge by verification status
    pub by_verification: HashMap<VerificationStatus, u64>,
    
    /// Average confidence
    pub average_confidence: f64,
    
    /// Graph statistics
    pub graph_stats: GraphStatistics,
    
    /// Memory size in bytes
    pub memory_size_bytes: u64,
}

/// Knowledge graph statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphStatistics {
    /// Total nodes
    pub total_nodes: u64,
    
    /// Total connections
    pub total_connections: u64,
    
    /// Average connections per node
    pub average_connections: f64,
    
    /// Graph density
    pub density: f64,
    
    /// Clustering coefficient
    pub clustering_coefficient: f64,
}

/// Configuration for semantic memory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticMemoryConfig {
    /// Maximum knowledge entries
    pub max_entries: usize,
    
    /// Minimum confidence threshold
    pub min_confidence_threshold: f64,
    
    /// Knowledge decay rate
    pub decay_rate: f64,
    
    /// Enable knowledge graph
    pub knowledge_graph_enabled: bool,
    
    /// Auto-verification enabled
    pub auto_verification_enabled: bool,
    
    /// Relationship inference enabled
    pub relationship_inference_enabled: bool,
}

impl Default for SemanticMemoryConfig {
    fn default() -> Self {
        Self {
            max_entries: 50000,
            min_confidence_threshold: 0.5,
            decay_rate: 0.001,
            knowledge_graph_enabled: true,
            auto_verification_enabled: true,
            relationship_inference_enabled: true,
        }
    }
}

impl SemanticMemory {
    /// Create a new semantic memory system
    pub async fn new() -> Result<Self, ConsciousnessError> {
        let config = SemanticMemoryConfig::default();
        
        Ok(Self {
            knowledge_base: Arc::new(RwLock::new(HashMap::new())),
            concept_index: Arc::new(RwLock::new(HashMap::new())),
            knowledge_graph: Arc::new(RwLock::new(HashMap::new())),
            relationship_index: Arc::new(RwLock::new(HashMap::new())),
            statistics: Arc::new(RwLock::new(SemanticMemoryStatistics {
                total_entries: 0,
                by_type: HashMap::new(),
                by_verification: HashMap::new(),
                average_confidence: 0.0,
                graph_stats: GraphStatistics {
                    total_nodes: 0,
                    total_connections: 0,
                    average_connections: 0.0,
                    density: 0.0,
                    clustering_coefficient: 0.0,
                },
                memory_size_bytes: 0,
            })),
            config,
        })
    }
    
    /// Store new knowledge
    pub async fn store_knowledge(
        &mut self,
        concept: &str,
        content: &str,
        knowledge_type: KnowledgeType,
        source: KnowledgeSource,
        confidence: f64,
    ) -> Result<Uuid, ConsciousnessError> {
        // Check confidence threshold
        if confidence < self.config.min_confidence_threshold {
            return Err(ConsciousnessError::InvalidInput(
                "Knowledge confidence below threshold".to_string()
            ));
        }
        
        let knowledge_id = Uuid::new_v4();
        let now = SystemTime::now();
        
        // Extract related concepts
        let related_concepts = self.extract_related_concepts(content).await?;
        
        // Create knowledge entry
        let knowledge_entry = SemanticKnowledgeEntry {
            id: knowledge_id,
            concept: concept.to_string(),
            content: content.to_string(),
            knowledge_type: knowledge_type.clone(),
            confidence,
            source,
            related_concepts: related_concepts.clone(),
            strength: confidence, // Initial strength equals confidence
            last_updated: now,
            created_at: now,
            access_frequency: 0,
            verification_status: VerificationStatus::Unverified,
            metadata: HashMap::new(),
        };
        
        // Store in knowledge base
        {
            let mut kb = self.knowledge_base.write().await;
            kb.insert(knowledge_id, knowledge_entry);
        }
        
        // Update concept index
        self.update_concept_index(knowledge_id, concept).await?;
        
        // Update knowledge graph if enabled
        if self.config.knowledge_graph_enabled {
            self.update_knowledge_graph(knowledge_id, concept, &related_concepts).await?;
        }
        
        // Auto-verify if enabled
        if self.config.auto_verification_enabled {
            self.auto_verify_knowledge(knowledge_id).await?;
        }
        
        // Update statistics
        self.update_storage_statistics(&knowledge_type).await?;
        
        Ok(knowledge_id)
    }
    
    /// Retrieve relevant knowledge
    pub async fn retrieve_relevant_knowledge(&self, query: &str) -> Result<SemanticContext, ConsciousnessError> {
        let query_concepts = self.extract_concepts_from_query(query).await?;
        
        // Retrieve by concept matching
        let concept_matches = self.retrieve_by_concepts(&query_concepts).await?;
        
        // Retrieve by content similarity
        let content_matches = self.retrieve_by_content_similarity(query).await?;
        
        // Retrieve by graph traversal if enabled
        let graph_matches = if self.config.knowledge_graph_enabled {
            self.retrieve_by_graph_traversal(&query_concepts).await?
        } else {
            Vec::new()
        };
        
        // Combine and rank results
        let combined_results = self.combine_knowledge_results(
            concept_matches,
            content_matches,
            graph_matches,
        ).await?;
        
        // Select top knowledge entries
        let top_knowledge = self.select_top_knowledge(combined_results, 20).await?;
        
        // Calculate confidence score
        let confidence_score = if top_knowledge.is_empty() {
            0.0
        } else {
            top_knowledge.iter().map(|k| k.confidence).sum::<f64>() / top_knowledge.len() as f64
        };
        
        // Update access statistics
        self.update_access_statistics(&top_knowledge).await?;
        
        Ok(SemanticContext {
            relevant_knowledge: top_knowledge.iter().map(|k| k.content.clone()).collect(),
            confidence_score,
            sources: top_knowledge.iter().map(|k| format!("{:?}", k.source)).collect(),
        })
    }
    
    /// Get memory size
    pub async fn get_memory_size(&self) -> Result<u64, ConsciousnessError> {
        let stats = self.statistics.read().await;
        Ok(stats.memory_size_bytes)
    }
    
    /// Optimize storage
    pub async fn optimize_storage(&mut self) -> Result<(), ConsciousnessError> {
        // Remove low-confidence knowledge
        let mut to_remove = Vec::new();
        
        {
            let kb = self.knowledge_base.read().await;
            for (id, entry) in kb.iter() {
                // Apply decay based on age and access frequency
                let age = entry.created_at.elapsed().unwrap_or(Duration::from_secs(0));
                let decay_factor = (-self.config.decay_rate * age.as_secs_f64()).exp();
                let adjusted_strength = entry.strength * decay_factor * (1.0 + entry.access_frequency as f64 / 100.0);
                
                if adjusted_strength < self.config.min_confidence_threshold {
                    to_remove.push(*id);
                }
            }
        }
        
        // Remove identified entries
        {
            let mut kb = self.knowledge_base.write().await;
            for id in &to_remove {
                kb.remove(id);
            }
        }
        
        // Update indexes
        self.cleanup_indexes(&to_remove).await?;
        
        // Update statistics
        {
            let mut stats = self.statistics.write().await;
            stats.total_entries = stats.total_entries.saturating_sub(to_remove.len() as u64);
        }
        
        Ok(())
    }
    
    /// Infer new relationships
    pub async fn infer_relationships(&mut self) -> Result<u32, ConsciousnessError> {
        if !self.config.relationship_inference_enabled {
            return Ok(0);
        }
        
        let mut new_relationships = 0;
        let kb = self.knowledge_base.read().await;
        let mut graph = self.knowledge_graph.write().await;
        
        // Simple relationship inference based on concept co-occurrence
        for (id1, entry1) in kb.iter() {
            for (id2, entry2) in kb.iter() {
                if id1 == id2 {
                    continue;
                }
                
                // Check for common concepts
                let common_concepts: HashSet<_> = entry1.related_concepts.iter()
                    .filter(|c| entry2.related_concepts.contains(c))
                    .collect();
                
                if common_concepts.len() >= 2 {
                    // Infer similarity relationship
                    if let (Some(node1), Some(node2)) = (graph.get_mut(id1), graph.get(id2)) {
                        let connection_exists = node1.connections.iter()
                            .any(|c| c.target_id == *id2);
                        
                        if !connection_exists {
                            let strength = common_concepts.len() as f64 / 
                                (entry1.related_concepts.len().max(entry2.related_concepts.len()) as f64);
                            
                            node1.connections.push(KnowledgeConnection {
                                target_id: *id2,
                                relationship: RelationshipType::SimilarTo,
                                strength,
                                bidirectional: true,
                            });
                            
                            new_relationships += 1;
                        }
                    }
                }
            }
        }
        
        Ok(new_relationships)
    }
    
    // Helper methods
    
    async fn extract_related_concepts(&self, content: &str) -> Result<Vec<String>, ConsciousnessError> {
        // Simple concept extraction (in production, use NLP)
        let words: Vec<&str> = content.split_whitespace().collect();
        let mut concepts = Vec::new();
        
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
        concepts.truncate(10); // Limit to top 10 concepts
        
        Ok(concepts)
    }
    
    async fn extract_concepts_from_query(&self, query: &str) -> Result<Vec<String>, ConsciousnessError> {
        self.extract_related_concepts(query).await
    }
    
    fn is_stop_word(&self, word: &str) -> bool {
        matches!(word, 
            "the" | "and" | "or" | "but" | "in" | "on" | "at" | "to" | "for" | "of" | "with" | "by" |
            "is" | "are" | "was" | "were" | "be" | "been" | "being" | "have" | "has" | "had" |
            "do" | "does" | "did" | "will" | "would" | "could" | "should" | "may" | "might" |
            "this" | "that" | "these" | "those" | "a" | "an"
        )
    }
    
    async fn update_concept_index(&self, knowledge_id: Uuid, concept: &str) -> Result<(), ConsciousnessError> {
        let mut index = self.concept_index.write().await;
        index.entry(concept.to_lowercase()).or_insert_with(Vec::new).push(knowledge_id);
        Ok(())
    }
    
    async fn update_knowledge_graph(&self, knowledge_id: Uuid, concept: &str, related_concepts: &[String]) -> Result<(), ConsciousnessError> {
        let mut graph = self.knowledge_graph.write().await;
        
        // Create or update node
        let node = graph.entry(knowledge_id).or_insert_with(|| KnowledgeNode {
            id: knowledge_id,
            concept: concept.to_string(),
            properties: HashMap::new(),
            connections: Vec::new(),
            importance: 1.0,
            centrality: 0.0,
        });
        
        // Add related concepts as properties
        for (i, related) in related_concepts.iter().enumerate() {
            node.properties.insert(format!("related_{}", i), related.clone());
        }
        
        Ok(())
    }
    
    async fn auto_verify_knowledge(&self, knowledge_id: Uuid) -> Result<(), ConsciousnessError> {
        // Simple auto-verification based on consistency with existing knowledge
        let mut kb = self.knowledge_base.write().await;
        
        if let Some(entry) = kb.get_mut(&knowledge_id) {
            // Check for contradictions with existing knowledge
            let has_contradictions = false; // Placeholder for contradiction detection
            
            entry.verification_status = if has_contradictions {
                VerificationStatus::Contradicted
            } else if entry.confidence > 0.8 {
                VerificationStatus::Verified
            } else {
                VerificationStatus::PartiallyVerified
            };
        }
        
        Ok(())
    }
    
    async fn update_storage_statistics(&self, knowledge_type: &KnowledgeType) -> Result<(), ConsciousnessError> {
        let mut stats = self.statistics.write().await;
        stats.total_entries += 1;
        *stats.by_type.entry(knowledge_type.clone()).or_insert(0) += 1;
        stats.memory_size_bytes += 1024; // Rough estimate
        Ok(())
    }
    
    async fn retrieve_by_concepts(&self, concepts: &[String]) -> Result<Vec<Uuid>, ConsciousnessError> {
        let index = self.concept_index.read().await;
        let mut results = Vec::new();
        
        for concept in concepts {
            if let Some(knowledge_ids) = index.get(&concept.to_lowercase()) {
                results.extend(knowledge_ids.iter().cloned());
            }
        }
        
        // Remove duplicates
        results.sort();
        results.dedup();
        
        Ok(results)
    }
    
    async fn retrieve_by_content_similarity(&self, query: &str) -> Result<Vec<Uuid>, ConsciousnessError> {
        let kb = self.knowledge_base.read().await;
        let mut results = Vec::new();
        
        let query_lower = query.to_lowercase();
        
        for (id, entry) in kb.iter() {
            let content_lower = entry.content.to_lowercase();
            
            // Simple similarity based on common words
            let query_words: HashSet<&str> = query_lower.split_whitespace().collect();
            let content_words: HashSet<&str> = content_lower.split_whitespace().collect();
            
            let common_words = query_words.intersection(&content_words).count();
            let total_words = query_words.union(&content_words).count();
            
            if total_words > 0 {
                let similarity = common_words as f64 / total_words as f64;
                if similarity > 0.1 { // Minimum similarity threshold
                    results.push(*id);
                }
            }
        }
        
        Ok(results)
    }
    
    async fn retrieve_by_graph_traversal(&self, concepts: &[String]) -> Result<Vec<Uuid>, ConsciousnessError> {
        let graph = self.knowledge_graph.read().await;
        let mut results = HashSet::new();
        
        // Find nodes matching concepts
        for (id, node) in graph.iter() {
            if concepts.iter().any(|c| node.concept.to_lowercase().contains(&c.to_lowercase())) {
                results.insert(*id);
                
                // Add connected nodes
                for connection in &node.connections {
                    if connection.strength > 0.5 {
                        results.insert(connection.target_id);
                    }
                }
            }
        }
        
        Ok(results.into_iter().collect())
    }
    
    async fn combine_knowledge_results(
        &self,
        concept_matches: Vec<Uuid>,
        content_matches: Vec<Uuid>,
        graph_matches: Vec<Uuid>,
    ) -> Result<Vec<Uuid>, ConsciousnessError> {
        let mut combined = Vec::new();
        combined.extend(concept_matches);
        combined.extend(content_matches);
        combined.extend(graph_matches);
        
        // Remove duplicates while preserving order
        let mut seen = HashSet::new();
        combined.retain(|id| seen.insert(*id));
        
        Ok(combined)
    }
    
    async fn select_top_knowledge(&self, knowledge_ids: Vec<Uuid>, limit: usize) -> Result<Vec<SemanticKnowledgeEntry>, ConsciousnessError> {
        let kb = self.knowledge_base.read().await;
        let mut knowledge_entries = Vec::new();
        
        for id in knowledge_ids.iter().take(limit * 2) { // Get more than needed for sorting
            if let Some(entry) = kb.get(id) {
                knowledge_entries.push(entry.clone());
            }
        }
        
        // Sort by relevance score (confidence + strength + access frequency)
        knowledge_entries.sort_by(|a, b| {
            let score_a = a.confidence + a.strength + (a.access_frequency as f64 / 100.0);
            let score_b = b.confidence + b.strength + (b.access_frequency as f64 / 100.0);
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        knowledge_entries.truncate(limit);
        Ok(knowledge_entries)
    }
    
    async fn update_access_statistics(&self, knowledge_entries: &[SemanticKnowledgeEntry]) -> Result<(), ConsciousnessError> {
        let mut kb = self.knowledge_base.write().await;
        
        for entry in knowledge_entries {
            if let Some(stored_entry) = kb.get_mut(&entry.id) {
                stored_entry.access_frequency += 1;
                stored_entry.last_updated = SystemTime::now();
                
                // Strengthen knowledge based on access
                stored_entry.strength = (stored_entry.strength + 0.05).min(1.0);
            }
        }
        
        Ok(())
    }
    
    async fn cleanup_indexes(&self, removed_ids: &[Uuid]) -> Result<(), ConsciousnessError> {
        // Clean concept index
        {
            let mut concept_index = self.concept_index.write().await;
            for (_, knowledge_ids) in concept_index.iter_mut() {
                knowledge_ids.retain(|id| !removed_ids.contains(id));
            }
        }
        
        // Clean knowledge graph
        {
            let mut graph = self.knowledge_graph.write().await;
            for id in removed_ids {
                graph.remove(id);
            }
            
            // Remove connections to deleted nodes
            for (_, node) in graph.iter_mut() {
                node.connections.retain(|conn| !removed_ids.contains(&conn.target_id));
            }
        }
        
        Ok(())
    }
}