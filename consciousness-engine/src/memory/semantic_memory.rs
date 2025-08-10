//! Semantic Memory - General knowledge and concepts

use crate::{
    types::*,
    error::{ConsciousnessError, ConsciousnessResult},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tracing::{debug, info};

/// Semantic memory system for storing and organizing general knowledge
#[derive(Debug, Clone)]
pub struct SemanticMemorySystem {
    concepts: HashMap<String, Concept>,
    relationships: HashMap<String, Vec<Relationship>>,
    knowledge_graph: KnowledgeGraph,
    concept_embeddings: HashMap<String, Vec<f64>>,
    learning_engine: SemanticLearningEngine,
}

impl SemanticMemorySystem {
    /// Create a new semantic memory system
    pub fn new() -> Self {
        info!("Initializing Semantic Memory System");
        
        Self {
            concepts: HashMap::new(),
            relationships: HashMap::new(),
            knowledge_graph: KnowledgeGraph::new(),
            concept_embeddings: HashMap::new(),
            learning_engine: SemanticLearningEngine::new(),
        }
    }
    
    /// Store or update a concept in semantic memory
    pub async fn store_concept(
        &mut self,
        concept_name: String,
        definition: String,
        properties: HashMap<String, String>,
        confidence: f64,
    ) -> ConsciousnessResult<()> {
        debug!("Storing concept: {}", concept_name);
        
        // Check if concept already exists
        if let Some(existing_concept) = self.concepts.get_mut(&concept_name) {
            // Update existing concept
            existing_concept.update_definition(definition, confidence);
            existing_concept.add_properties(properties);
            existing_concept.last_updated = Utc::now();
        } else {
            // Create new concept
            let concept = Concept {
                name: concept_name.clone(),
                definition,
                properties,
                confidence,
                usage_count: 0,
                last_accessed: Utc::now(),
                last_updated: Utc::now(),
                created_at: Utc::now(),
                related_concepts: Vec::new(),
                examples: Vec::new(),
                categories: Vec::new(),
            };
            
            self.concepts.insert(concept_name.clone(), concept);
        }
        
        // Generate embedding for the concept
        let embedding = self.generate_concept_embedding(&concept_name).await?;
        self.concept_embeddings.insert(concept_name.clone(), embedding);
        
        // Update knowledge graph
        self.knowledge_graph.add_concept(&concept_name).await?;
        
        // Find and create relationships with existing concepts
        self.discover_relationships(&concept_name).await?;
        
        info!("Concept stored: {}", concept_name);
        
        Ok(())
    }
    
    /// Retrieve a concept by name
    pub async fn get_concept(&mut self, concept_name: &str) -> ConsciousnessResult<Option<Concept>> {
        if let Some(concept) = self.concepts.get_mut(concept_name) {
            // Update access statistics
            concept.usage_count += 1;
            concept.last_accessed = Utc::now();
            
            debug!("Retrieved concept: {} (usage: {})", concept_name, concept.usage_count);
            
            Ok(Some(concept.clone()))
        } else {
            Ok(None)
        }
    }
    
    /// Search for concepts based on query
    pub async fn search_concepts(
        &self,
        query: &str,
        max_results: usize,
    ) -> ConsciousnessResult<Vec<ConceptSearchResult>> {
        debug!("Searching concepts for query: {}", query);
        
        let mut results = Vec::new();
        
        // Generate query embedding
        let query_embedding = self.generate_query_embedding(query).await?;
        
        // Calculate similarity with all concepts
        for (concept_name, concept) in &self.concepts {
            if let Some(concept_embedding) = self.concept_embeddings.get(concept_name) {
                let similarity = self.calculate_cosine_similarity(&query_embedding, concept_embedding);
                
                if similarity > 0.3 { // Minimum similarity threshold
                    results.push(ConceptSearchResult {
                        concept: concept.clone(),
                        similarity_score: similarity,
                        match_type: self.determine_match_type(query, concept),
                    });
                }
            }
        }
        
        // Sort by similarity score
        results.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap());
        
        // Return top results
        results.truncate(max_results);
        
        debug!("Found {} concept matches", results.len());
        
        Ok(results)
    }
    
    /// Add a relationship between two concepts
    pub async fn add_relationship(
        &mut self,
        concept1: String,
        concept2: String,
        relationship_type: RelationshipType,
        strength: f64,
    ) -> ConsciousnessResult<()> {
        debug!("Adding relationship: {} {:?} {}", concept1, relationship_type, concept2);
        
        let relationship = Relationship {
            from_concept: concept1.clone(),
            to_concept: concept2.clone(),
            relationship_type: relationship_type.clone(),
            strength,
            confidence: 0.8,
            created_at: Utc::now(),
            usage_count: 0,
        };
        
        // Add relationship in both directions
        self.relationships
            .entry(concept1.clone())
            .or_insert_with(Vec::new)
            .push(relationship.clone());
        
        // Add reverse relationship if appropriate
        if matches!(relationship_type, RelationshipType::Similar | RelationshipType::Related) {
            let reverse_relationship = Relationship {
                from_concept: concept2.clone(),
                to_concept: concept1.clone(),
                relationship_type,
                strength,
                confidence: 0.8,
                created_at: Utc::now(),
                usage_count: 0,
            };
            
            self.relationships
                .entry(concept2.clone())
                .or_insert_with(Vec::new)
                .push(reverse_relationship);
        }
        
        // Update knowledge graph
        self.knowledge_graph.add_relationship(&concept1, &concept2, relationship_type).await?;
        
        // Update concept relationships
        if let Some(concept) = self.concepts.get_mut(&concept1) {
            if !concept.related_concepts.contains(&concept2) {
                concept.related_concepts.push(concept2.clone());
            }
        }
        
        if let Some(concept) = self.concepts.get_mut(&concept2) {
            if !concept.related_concepts.contains(&concept1) {
                concept.related_concepts.push(concept1);
            }
        }
        
        Ok(())
    }
    
    /// Get related concepts for a given concept
    pub async fn get_related_concepts(
        &self,
        concept_name: &str,
        max_results: usize,
    ) -> ConsciousnessResult<Vec<RelatedConcept>> {
        debug!("Getting related concepts for: {}", concept_name);
        
        let mut related = Vec::new();
        
        if let Some(relationships) = self.relationships.get(concept_name) {
            for relationship in relationships {
                if let Some(related_concept) = self.concepts.get(&relationship.to_concept) {
                    related.push(RelatedConcept {
                        concept: related_concept.clone(),
                        relationship_type: relationship.relationship_type.clone(),
                        strength: relationship.strength,
                        confidence: relationship.confidence,
                    });
                }
            }
        }
        
        // Sort by relationship strength
        related.sort_by(|a, b| b.strength.partial_cmp(&a.strength).unwrap());
        
        // Return top results
        related.truncate(max_results);
        
        debug!("Found {} related concepts", related.len());
        
        Ok(related)
    }
    
    /// Learn new knowledge from text or experience
    pub async fn learn_from_text(
        &mut self,
        text: &str,
        context: &str,
    ) -> ConsciousnessResult<LearningResult> {
        debug!("Learning from text: {} characters", text.len());
        
        let learning_result = self.learning_engine.extract_knowledge(text, context).await?;
        
        // Store extracted concepts
        for concept_info in &learning_result.extracted_concepts {
            self.store_concept(
                concept_info.name.clone(),
                concept_info.definition.clone(),
                concept_info.properties.clone(),
                concept_info.confidence,
            ).await?;
        }
        
        // Store extracted relationships
        for relationship_info in &learning_result.extracted_relationships {
            self.add_relationship(
                relationship_info.from_concept.clone(),
                relationship_info.to_concept.clone(),
                relationship_info.relationship_type.clone(),
                relationship_info.strength,
            ).await?;
        }
        
        info!("Learned {} concepts and {} relationships from text", 
              learning_result.extracted_concepts.len(),
              learning_result.extracted_relationships.len());
        
        Ok(learning_result)
    }
    
    /// Consolidate semantic knowledge (strengthen frequently used concepts)
    pub async fn consolidate_knowledge(&mut self) -> ConsciousnessResult<ConsolidationReport> {
        debug!("Starting semantic knowledge consolidation");
        
        let mut strengthened_concepts = 0;
        let mut weakened_concepts = 0;
        let mut removed_concepts = 0;
        
        // Strengthen frequently used concepts
        for concept in self.concepts.values_mut() {
            if concept.usage_count > 10 {
                concept.confidence = (concept.confidence * 1.1).min(1.0);
                strengthened_concepts += 1;
            } else if concept.usage_count == 0 && concept.confidence < 0.3 {
                // Mark for removal
                concept.confidence *= 0.5;
                weakened_concepts += 1;
            }
        }
        
        // Remove very weak concepts
        let weak_concepts: Vec<String> = self.concepts
            .iter()
            .filter(|(_, concept)| concept.confidence < 0.1)
            .map(|(name, _)| name.clone())
            .collect();
        
        for concept_name in weak_concepts {
            self.remove_concept(&concept_name).await?;
            removed_concepts += 1;
        }
        
        let report = ConsolidationReport {
            total_concepts: self.concepts.len(),
            strengthened_concepts,
            weakened_concepts,
            removed_concepts,
            consolidation_time: Utc::now(),
        };
        
        info!("Semantic knowledge consolidation completed: {:?}", report);
        
        Ok(report)
    }
    
    /// Generate embedding for a concept
    async fn generate_concept_embedding(&self, concept_name: &str) -> ConsciousnessResult<Vec<f64>> {
        // Simple embedding generation (in practice, would use a proper embedding model)
        let mut embedding = vec![0.0; 128];
        
        // Hash-based embedding for demonstration
        let hash = self.simple_hash(concept_name);
        for i in 0..128 {
            embedding[i] = ((hash + i) % 1000) as f64 / 1000.0;
        }
        
        Ok(embedding)
    }
    
    /// Generate embedding for a query
    async fn generate_query_embedding(&self, query: &str) -> ConsciousnessResult<Vec<f64>> {
        // Simple query embedding (in practice, would use a proper embedding model)
        let mut embedding = vec![0.0; 128];
        
        let hash = self.simple_hash(query);
        for i in 0..128 {
            embedding[i] = ((hash + i * 2) % 1000) as f64 / 1000.0;
        }
        
        Ok(embedding)
    }
    
    /// Calculate cosine similarity between two embeddings
    fn calculate_cosine_similarity(&self, embedding1: &[f64], embedding2: &[f64]) -> f64 {
        if embedding1.len() != embedding2.len() {
            return 0.0;
        }
        
        let dot_product: f64 = embedding1.iter()
            .zip(embedding2.iter())
            .map(|(a, b)| a * b)
            .sum();
        
        let norm1: f64 = embedding1.iter().map(|x| x * x).sum::<f64>().sqrt();
        let norm2: f64 = embedding2.iter().map(|x| x * x).sum::<f64>().sqrt();
        
        if norm1 == 0.0 || norm2 == 0.0 {
            0.0
        } else {
            dot_product / (norm1 * norm2)
        }
    }
    
    /// Simple hash function for demonstration
    fn simple_hash(&self, text: &str) -> usize {
        text.chars().map(|c| c as usize).sum()
    }
    
    /// Determine match type for search results
    fn determine_match_type(&self, query: &str, concept: &Concept) -> MatchType {
        let query_lower = query.to_lowercase();
        let concept_name_lower = concept.name.to_lowercase();
        
        if concept_name_lower == query_lower {
            MatchType::Exact
        } else if concept_name_lower.contains(&query_lower) || query_lower.contains(&concept_name_lower) {
            MatchType::Partial
        } else if concept.definition.to_lowercase().contains(&query_lower) {
            MatchType::Definition
        } else {
            MatchType::Semantic
        }
    }
    
    /// Discover relationships between concepts
    async fn discover_relationships(&mut self, concept_name: &str) -> ConsciousnessResult<()> {
        // Simple relationship discovery based on concept similarity
        if let Some(concept_embedding) = self.concept_embeddings.get(concept_name).cloned() {
            for (other_concept_name, other_embedding) in &self.concept_embeddings {
                if other_concept_name == concept_name {
                    continue;
                }
                
                let similarity = self.calculate_cosine_similarity(&concept_embedding, other_embedding);
                
                if similarity > 0.7 {
                    self.add_relationship(
                        concept_name.to_string(),
                        other_concept_name.clone(),
                        RelationshipType::Similar,
                        similarity,
                    ).await?;
                } else if similarity > 0.5 {
                    self.add_relationship(
                        concept_name.to_string(),
                        other_concept_name.clone(),
                        RelationshipType::Related,
                        similarity,
                    ).await?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Remove a concept from semantic memory
    async fn remove_concept(&mut self, concept_name: &str) -> ConsciousnessResult<()> {
        // Remove concept
        self.concepts.remove(concept_name);
        
        // Remove embedding
        self.concept_embeddings.remove(concept_name);
        
        // Remove relationships
        self.relationships.remove(concept_name);
        
        // Remove from other concepts' relationships
        for relationships in self.relationships.values_mut() {
            relationships.retain(|r| r.to_concept != concept_name);
        }
        
        // Update knowledge graph
        self.knowledge_graph.remove_concept(concept_name).await?;
        
        Ok(())
    }
    
    /// Get semantic memory statistics
    pub fn get_statistics(&self) -> SemanticMemoryStatistics {
        let total_concepts = self.concepts.len();
        let total_relationships = self.relationships.values()
            .map(|rels| rels.len())
            .sum();
        
        let high_confidence_concepts = self.concepts.values()
            .filter(|c| c.confidence > 0.8)
            .count();
        
        let frequently_used_concepts = self.concepts.values()
            .filter(|c| c.usage_count > 5)
            .count();
        
        SemanticMemoryStatistics {
            total_concepts,
            total_relationships,
            high_confidence_concepts,
            frequently_used_concepts,
            average_confidence: if total_concepts > 0 {
                self.concepts.values().map(|c| c.confidence).sum::<f64>() / total_concepts as f64
            } else {
                0.0
            },
        }
    }
}

/// Supporting structures
#[derive(Debug, Clone)]
struct KnowledgeGraph {
    nodes: HashMap<String, GraphNode>,
    edges: Vec<GraphEdge>,
}

impl KnowledgeGraph {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }
    
    async fn add_concept(&mut self, concept_name: &str) -> ConsciousnessResult<()> {
        self.nodes.insert(concept_name.to_string(), GraphNode {
            name: concept_name.to_string(),
            node_type: "concept".to_string(),
            properties: HashMap::new(),
        });
        Ok(())
    }
    
    async fn add_relationship(
        &mut self,
        from_concept: &str,
        to_concept: &str,
        relationship_type: RelationshipType,
    ) -> ConsciousnessResult<()> {
        self.edges.push(GraphEdge {
            from: from_concept.to_string(),
            to: to_concept.to_string(),
            edge_type: format!("{:?}", relationship_type),
            weight: 1.0,
        });
        Ok(())
    }
    
    async fn remove_concept(&mut self, concept_name: &str) -> ConsciousnessResult<()> {
        self.nodes.remove(concept_name);
        self.edges.retain(|edge| edge.from != concept_name && edge.to != concept_name);
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct SemanticLearningEngine;

impl SemanticLearningEngine {
    fn new() -> Self { Self }
    
    async fn extract_knowledge(&self, text: &str, _context: &str) -> ConsciousnessResult<LearningResult> {
        // Simple knowledge extraction (in practice, would use NLP models)
        let mut extracted_concepts = Vec::new();
        let mut extracted_relationships = Vec::new();
        
        // Extract potential concepts (capitalized words, technical terms)
        let words: Vec<&str> = text.split_whitespace().collect();
        for word in words {
            if word.len() > 3 && word.chars().next().unwrap().is_uppercase() {
                extracted_concepts.push(ExtractedConcept {
                    name: word.to_string(),
                    definition: format!("Concept extracted from text: {}", word),
                    properties: HashMap::new(),
                    confidence: 0.6,
                });
            }
        }
        
        Ok(LearningResult {
            extracted_concepts,
            extracted_relationships,
            confidence: 0.7,
            learning_time: Utc::now(),
        })
    }
}

/// Data structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concept {
    pub name: String,
    pub definition: String,
    pub properties: HashMap<String, String>,
    pub confidence: f64,
    pub usage_count: u32,
    pub last_accessed: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub related_concepts: Vec<String>,
    pub examples: Vec<String>,
    pub categories: Vec<String>,
}

impl Concept {
    fn update_definition(&mut self, new_definition: String, confidence: f64) {
        if confidence > self.confidence {
            self.definition = new_definition;
            self.confidence = confidence;
        }
    }
    
    fn add_properties(&mut self, new_properties: HashMap<String, String>) {
        self.properties.extend(new_properties);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub from_concept: String,
    pub to_concept: String,
    pub relationship_type: RelationshipType,
    pub strength: f64,
    pub confidence: f64,
    pub created_at: DateTime<Utc>,
    pub usage_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    IsA,        // Hierarchical relationship
    PartOf,     // Compositional relationship
    Similar,    // Similarity relationship
    Opposite,   // Antonym relationship
    Related,    // General relatedness
    Causes,     // Causal relationship
    UsedFor,    // Functional relationship
}

#[derive(Debug, Clone)]
pub struct ConceptSearchResult {
    pub concept: Concept,
    pub similarity_score: f64,
    pub match_type: MatchType,
}

#[derive(Debug, Clone)]
pub enum MatchType {
    Exact,
    Partial,
    Definition,
    Semantic,
}

#[derive(Debug, Clone)]
pub struct RelatedConcept {
    pub concept: Concept,
    pub relationship_type: RelationshipType,
    pub strength: f64,
    pub confidence: f64,
}

#[derive(Debug, Clone)]
pub struct LearningResult {
    pub extracted_concepts: Vec<ExtractedConcept>,
    pub extracted_relationships: Vec<ExtractedRelationship>,
    pub confidence: f64,
    pub learning_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ExtractedConcept {
    pub name: String,
    pub definition: String,
    pub properties: HashMap<String, String>,
    pub confidence: f64,
}

#[derive(Debug, Clone)]
pub struct ExtractedRelationship {
    pub from_concept: String,
    pub to_concept: String,
    pub relationship_type: RelationshipType,
    pub strength: f64,
}

#[derive(Debug, Clone)]
pub struct ConsolidationReport {
    pub total_concepts: usize,
    pub strengthened_concepts: usize,
    pub weakened_concepts: usize,
    pub removed_concepts: usize,
    pub consolidation_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct SemanticMemoryStatistics {
    pub total_concepts: usize,
    pub total_relationships: usize,
    pub high_confidence_concepts: usize,
    pub frequently_used_concepts: usize,
    pub average_confidence: f64,
}

#[derive(Debug, Clone)]
struct GraphNode {
    name: String,
    node_type: String,
    properties: HashMap<String, String>,
}

#[derive(Debug, Clone)]
struct GraphEdge {
    from: String,
    to: String,
    edge_type: String,
    weight: f64,
}

impl Default for SemanticMemorySystem {
    fn default() -> Self {
        Self::new()
    }
}