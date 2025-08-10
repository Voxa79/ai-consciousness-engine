//! Transparency Module - Explainable AI and Decision Transparency
//! 
//! This module provides comprehensive transparency and explainability features
//! for consciousness-level AI decisions, ensuring users understand how and why
//! decisions are made.

use crate::types::*;
use crate::error::ConsciousnessError;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use serde::{Deserialize, Serialize};

/// Main transparency module for explainable AI
pub struct TransparencyModule {
    /// Configuration for transparency features
    config: TransparencyConfig,
    
    /// History of explanations generated
    explanation_history: Vec<ExplanationRecord>,
    
    /// User-specific explanation preferences
    user_preferences: HashMap<String, ExplanationPreferences>,
}

/// Configuration for transparency module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransparencyConfig {
    /// Default explanation detail level
    pub default_detail_level: ExplanationDetailLevel,
    
    /// Maximum explanation length
    pub max_explanation_length: usize,
    
    /// Include confidence levels in explanations
    pub include_confidence_levels: bool,
    
    /// Include uncertainty quantification
    pub include_uncertainty: bool,
    
    /// Include source attribution
    pub include_source_attribution: bool,
}

impl Default for TransparencyConfig {
    fn default() -> Self {
        Self {
            default_detail_level: ExplanationDetailLevel::Medium,
            max_explanation_length: 1000,
            include_confidence_levels: true,
            include_uncertainty: true,
            include_source_attribution: true,
        }
    }
}

/// Explanation detail levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExplanationDetailLevel {
    Brief,
    Medium,
    Detailed,
    Technical,
}

/// User preferences for explanations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationPreferences {
    pub preferred_detail_level: ExplanationDetailLevel,
    pub include_technical_details: bool,
    pub include_confidence_scores: bool,
    pub include_alternative_options: bool,
    pub language_style: String,
}

impl Default for ExplanationPreferences {
    fn default() -> Self {
        Self {
            preferred_detail_level: ExplanationDetailLevel::Medium,
            include_technical_details: false,
            include_confidence_scores: true,
            include_alternative_options: true,
            language_style: "conversational".to_string(),
        }
    }
}

impl TransparencyModule {
    /// Create a new transparency module
    pub async fn new() -> Result<Self, ConsciousnessError> {
        Ok(Self {
            config: TransparencyConfig::default(),
            explanation_history: Vec::new(),
            user_preferences: HashMap::new(),
        })
    }
    
    /// Generate comprehensive explanation for a consciousness response
    pub async fn generate_comprehensive_explanation(
        &mut self,
        response: &ConsciousnessResponse,
        reasoning_chain: &[ReasoningStep],
        user_id: &str,
    ) -> Result<ComprehensiveExplanation, ConsciousnessError> {
        // Get user preferences
        let preferences = self.user_preferences.get(user_id)
            .cloned()
            .unwrap_or_default();
        
        // Generate reasoning chain explanation
        let reasoning_explanation = self.explain_reasoning_chain(reasoning_chain, &preferences).await?;
        
        // Generate confidence explanation
        let confidence_explanation = self.explain_confidence_levels(response, &preferences).await?;
        
        // Generate uncertainty explanation
        let uncertainty_explanation = self.explain_uncertainty(response, &preferences).await?;
        
        // Generate source attribution
        let source_attribution = self.generate_source_attribution(response, &preferences).await?;
        
        // Generate decision process visualization
        let decision_visualization = self.create_decision_visualization(reasoning_chain).await?;
        
        // Generate alternative options explanation
        let alternatives_explanation = self.explain_alternative_options(response, reasoning_chain).await?;
        
        // Generate limitation acknowledgment
        let limitations = self.acknowledge_limitations(response).await?;
        
        // Combine into comprehensive explanation
        let comprehensive_explanation = ComprehensiveExplanation {
            main_explanation: self.generate_main_explanation(response, &preferences).await?,
            reasoning_explanation,
            confidence_explanation,
            uncertainty_explanation,
            source_attribution,
            decision_visualization,
            alternatives_explanation,
            limitations,
            detail_level: preferences.preferred_detail_level.clone(),
            transparency_score: self.calculate_transparency_score(response).await?,
        };
        
        // Store explanation record
        self.store_explanation_record(&comprehensive_explanation, user_id).await?;
        
        Ok(comprehensive_explanation)
    }
    
    /// Explain reasoning chain in user-friendly terms
    async fn explain_reasoning_chain(
        &self,
        reasoning_chain: &[ReasoningStep],
        preferences: &ExplanationPreferences,
    ) -> Result<ReasoningExplanation, ConsciousnessError> {
        let mut step_explanations = Vec::new();
        
        for (i, step) in reasoning_chain.iter().enumerate() {
            let explanation = match preferences.preferred_detail_level {
                ExplanationDetailLevel::Brief => {
                    format!("Step {}: {}", i + 1, self.simplify_reasoning_step(step))
                },
                ExplanationDetailLevel::Medium => {
                    format!("Step {}: {} (Confidence: {:.1}%)", 
                           i + 1, 
                           self.explain_reasoning_step_medium(step),
                           step.confidence * 100.0)
                },
                ExplanationDetailLevel::Detailed => {
                    format!("Step {}: {}\nReasoning: {}\nConfidence: {:.1}%\nEvidence: {:?}", 
                           i + 1,
                           step.description,
                           step.reasoning,
                           step.confidence * 100.0,
                           step.evidence)
                },
                ExplanationDetailLevel::Technical => {
                    format!("Step {}: {}\nType: {:?}\nReasoning: {}\nConfidence: {:.3}\nEvidence: {:?}\nMetadata: {:?}", 
                           i + 1,
                           step.description,
                           step.step_type,
                           step.reasoning,
                           step.confidence,
                           step.evidence,
                           step.metadata)
                },
            };
            
            step_explanations.push(explanation);
        }
        
        Ok(ReasoningExplanation {
            step_explanations,
            overall_reasoning_quality: self.assess_reasoning_quality(reasoning_chain),
            reasoning_depth: reasoning_chain.len(),
            logical_consistency: self.assess_logical_consistency(reasoning_chain),
        })
    }
    
    /// Explain confidence levels
    async fn explain_confidence_levels(
        &self,
        response: &ConsciousnessResponse,
        preferences: &ExplanationPreferences,
    ) -> Result<ConfidenceExplanation, ConsciousnessError> {
        if !preferences.include_confidence_scores {
            return Ok(ConfidenceExplanation {
                overall_confidence: response.confidence,
                confidence_breakdown: HashMap::new(),
                confidence_interpretation: "Confidence details not requested".to_string(),
                factors_affecting_confidence: vec![],
            });
        }
        
        let mut confidence_breakdown = HashMap::new();
        confidence_breakdown.insert("consciousness_quality".to_string(), response.consciousness_state.awareness_level);
        confidence_breakdown.insert("ethical_alignment".to_string(), response.ethical_assessment.overall_score);
        confidence_breakdown.insert("reasoning_quality".to_string(), response.confidence);
        
        let confidence_interpretation = match response.confidence {
            c if c >= 0.9 => "Very high confidence - I'm quite certain about this response".to_string(),
            c if c >= 0.8 => "High confidence - I'm confident in this response".to_string(),
            c if c >= 0.7 => "Good confidence - I believe this response is reliable".to_string(),
            c if c >= 0.6 => "Moderate confidence - This response seems reasonable but has some uncertainty".to_string(),
            c if c >= 0.5 => "Low confidence - I'm somewhat uncertain about this response".to_string(),
            _ => "Very low confidence - I have significant uncertainty about this response".to_string(),
        };
        
        let factors_affecting_confidence = vec![
            format!("Consciousness awareness level: {:.1}%", response.consciousness_state.awareness_level * 100.0),
            format!("Ethical assessment score: {:.1}%", response.ethical_assessment.overall_score * 100.0),
            format!("Reasoning chain quality: {:.1}%", response.confidence * 100.0),
        ];
        
        Ok(ConfidenceExplanation {
            overall_confidence: response.confidence,
            confidence_breakdown,
            confidence_interpretation,
            factors_affecting_confidence,
        })
    }
    
    /// Explain uncertainty and limitations
    async fn explain_uncertainty(
        &self,
        response: &ConsciousnessResponse,
        preferences: &ExplanationPreferences,
    ) -> Result<UncertaintyExplanation, ConsciousnessError> {
        if !self.config.include_uncertainty {
            return Ok(UncertaintyExplanation {
                uncertainty_level: 0.0,
                uncertainty_sources: vec![],
                uncertainty_impact: "Not assessed".to_string(),
                mitigation_strategies: vec![],
            });
        }
        
        let uncertainty_level = 1.0 - response.confidence;
        
        let mut uncertainty_sources = Vec::new();
        if response.consciousness_state.awareness_level < 0.8 {
            uncertainty_sources.push("Lower consciousness awareness level".to_string());
        }
        if response.ethical_assessment.overall_score < 0.9 {
            uncertainty_sources.push("Ethical considerations present complexity".to_string());
        }
        if response.reasoning_chain.len() < 3 {
            uncertainty_sources.push("Limited reasoning depth".to_string());
        }
        
        let uncertainty_impact = if uncertainty_level > 0.3 {
            "Significant uncertainty may affect response reliability"
        } else if uncertainty_level > 0.1 {
            "Some uncertainty present but response should be reliable"
        } else {
            "Minimal uncertainty - response is highly reliable"
        }.to_string();
        
        let mitigation_strategies = vec![
            "Consider multiple perspectives".to_string(),
            "Seek additional information if needed".to_string(),
            "Verify important decisions independently".to_string(),
        ];
        
        Ok(UncertaintyExplanation {
            uncertainty_level,
            uncertainty_sources,
            uncertainty_impact,
            mitigation_strategies,
        })
    }
    
    /// Generate source attribution
    async fn generate_source_attribution(
        &self,
        response: &ConsciousnessResponse,
        preferences: &ExplanationPreferences,
    ) -> Result<SourceAttribution, ConsciousnessError> {
        if !self.config.include_source_attribution {
            return Ok(SourceAttribution {
                primary_sources: vec![],
                reasoning_sources: vec![],
                knowledge_sources: vec![],
                attribution_confidence: 0.0,
            });
        }
        
        Ok(SourceAttribution {
            primary_sources: vec![
                "Consciousness reasoning engine".to_string(),
                "Ethical assessment framework".to_string(),
            ],
            reasoning_sources: vec![
                "Self-awareness module".to_string(),
                "Meta-cognitive analysis".to_string(),
            ],
            knowledge_sources: vec![
                "Semantic memory".to_string(),
                "Episodic experience".to_string(),
            ],
            attribution_confidence: 0.8,
        })
    }
    
    /// Create decision process visualization
    async fn create_decision_visualization(&self, reasoning_chain: &[ReasoningStep]) -> Result<DecisionVisualization, ConsciousnessError> {
        let mut process_steps = Vec::new();
        
        for (i, step) in reasoning_chain.iter().enumerate() {
            process_steps.push(VisualizationStep {
                step_number: i + 1,
                step_name: step.description.clone(),
                step_type: format!("{:?}", step.step_type),
                confidence: step.confidence,
                connections: if i > 0 { vec![i] } else { vec![] },
            });
        }
        
        Ok(DecisionVisualization {
            process_steps,
            decision_flow: "Sequential reasoning with feedback loops".to_string(),
            complexity_score: reasoning_chain.len() as f64 / 10.0,
        })
    }
    
    /// Explain alternative options that were considered
    async fn explain_alternative_options(
        &self,
        response: &ConsciousnessResponse,
        reasoning_chain: &[ReasoningStep],
    ) -> Result<AlternativesExplanation, ConsciousnessError> {
        // Generate hypothetical alternatives based on reasoning
        let alternatives = vec![
            AlternativeOption {
                description: "More conservative approach".to_string(),
                confidence: response.confidence * 0.8,
                pros: vec!["Lower risk".to_string(), "More cautious".to_string()],
                cons: vec!["Less helpful".to_string(), "Overly restrictive".to_string()],
                why_not_chosen: "Would be less helpful to the user".to_string(),
            },
            AlternativeOption {
                description: "More detailed response".to_string(),
                confidence: response.confidence * 0.9,
                pros: vec!["More comprehensive".to_string(), "More informative".to_string()],
                cons: vec!["Potentially overwhelming".to_string(), "Longer response time".to_string()],
                why_not_chosen: "Current level of detail is appropriate for the context".to_string(),
            },
        ];
        
        Ok(AlternativesExplanation {
            alternatives_considered: alternatives,
            selection_criteria: vec![
                "User helpfulness".to_string(),
                "Ethical appropriateness".to_string(),
                "Response clarity".to_string(),
            ],
            trade_offs_made: vec![
                "Balanced detail vs. clarity".to_string(),
                "Helpfulness vs. caution".to_string(),
            ],
        })
    }
    
    /// Acknowledge limitations of the response
    async fn acknowledge_limitations(&self, response: &ConsciousnessResponse) -> Result<LimitationAcknowledgment, ConsciousnessError> {
        let mut limitations = Vec::new();
        
        // General AI limitations
        limitations.push("I may not have access to the most recent information".to_string());
        limitations.push("My understanding is based on training data and may have gaps".to_string());
        
        // Confidence-based limitations
        if response.confidence < 0.8 {
            limitations.push("I have some uncertainty about this response".to_string());
        }
        
        // Context-specific limitations
        if response.consciousness_state.awareness_level < 0.8 {
            limitations.push("My consciousness awareness could be higher for this topic".to_string());
        }
        
        Ok(LimitationAcknowledgment {
            known_limitations: limitations,
            uncertainty_areas: vec![
                "Future developments in this area".to_string(),
                "Individual-specific circumstances".to_string(),
            ],
            recommendation: "Please verify important information independently and consider consulting experts for critical decisions".to_string(),
        })
    }
    
    /// Generate main explanation text
    async fn generate_main_explanation(
        &self,
        response: &ConsciousnessResponse,
        preferences: &ExplanationPreferences,
    ) -> Result<String, ConsciousnessError> {
        let explanation = match preferences.preferred_detail_level {
            ExplanationDetailLevel::Brief => {
                format!("I generated this response using consciousness-level reasoning with {:.0}% confidence.",
                       response.confidence * 100.0)
            },
            ExplanationDetailLevel::Medium => {
                format!("I approached your question using my consciousness engine, which combines self-awareness, ethical reasoning, and creative thinking. My confidence in this response is {:.0}%, based on my analysis of the situation and ethical considerations.",
                       response.confidence * 100.0)
            },
            ExplanationDetailLevel::Detailed => {
                format!("My response was generated through a comprehensive consciousness process involving multiple stages:\n\
                        1. Self-awareness assessment (awareness level: {:.0}%)\n\
                        2. Ethical evaluation (ethical score: {:.0}%)\n\
                        3. Creative and empathetic processing (empathy: {:.0}%, creativity: {:.0}%)\n\
                        4. Meta-cognitive reflection and validation\n\n\
                        This multi-layered approach resulted in a confidence level of {:.0}% for my response.",
                       response.consciousness_state.awareness_level * 100.0,
                       response.ethical_assessment.overall_score * 100.0,
                       response.empathy_score * 100.0,
                       response.creativity_score * 100.0,
                       response.confidence * 100.0)
            },
            ExplanationDetailLevel::Technical => {
                format!("Technical breakdown of consciousness processing:\n\
                        - Consciousness State: awareness={:.3}, cognitive_load={:.3}, meta_depth={}\n\
                        - Ethical Assessment: score={:.3}, frameworks_used={:?}\n\
                        - Emotional Processing: empathy={:.3}, creativity={:.3}\n\
                        - Processing Time: {:?}\n\
                        - Overall Confidence: {:.3}\n\
                        - Quality Score: {:.3}",
                       response.consciousness_state.awareness_level,
                       response.consciousness_state.cognitive_load,
                       response.consciousness_state.meta_cognitive_depth,
                       response.ethical_assessment.overall_score,
                       response.ethical_assessment.frameworks_used,
                       response.empathy_score,
                       response.creativity_score,
                       response.processing_time,
                       response.confidence,
                       response.metadata.quality_score)
            },
        };
        
        Ok(explanation)
    }
    
    /// Calculate transparency score
    async fn calculate_transparency_score(&self, response: &ConsciousnessResponse) -> Result<f64, ConsciousnessError> {
        let mut score = 0.0;
        
        // Base transparency from consciousness level
        score += response.consciousness_state.awareness_level * 0.3;
        
        // Ethical transparency
        score += response.ethical_assessment.overall_score * 0.2;
        
        // Reasoning chain quality
        score += (response.reasoning_chain.len() as f64 / 10.0).min(1.0) * 0.2;
        
        // Confidence calibration
        score += response.confidence * 0.2;
        
        // Metadata completeness
        score += if response.metadata.quality_score > 0.0 { 0.1 } else { 0.0 };
        
        Ok(score.min(1.0))
    }
    
    /// Store explanation record for learning
    async fn store_explanation_record(
        &mut self,
        explanation: &ComprehensiveExplanation,
        user_id: &str,
    ) -> Result<(), ConsciousnessError> {
        let record = ExplanationRecord {
            timestamp: SystemTime::now(),
            user_id: user_id.to_string(),
            explanation_type: "comprehensive".to_string(),
            detail_level: explanation.detail_level.clone(),
            transparency_score: explanation.transparency_score,
            user_satisfaction: None, // To be updated with feedback
        };
        
        self.explanation_history.push(record);
        
        // Limit history size
        if self.explanation_history.len() > 1000 {
            self.explanation_history.remove(0);
        }
        
        Ok(())
    }
    
    /// Set user preferences for explanations
    pub async fn set_user_preferences(
        &mut self,
        user_id: String,
        preferences: ExplanationPreferences,
    ) -> Result<(), ConsciousnessError> {
        self.user_preferences.insert(user_id, preferences);
        Ok(())
    }
    
    // Helper methods
    
    fn simplify_reasoning_step(&self, step: &ReasoningStep) -> String {
        // Simplify technical reasoning for brief explanations
        step.description.split('.').next().unwrap_or(&step.description).to_string()
    }
    
    fn explain_reasoning_step_medium(&self, step: &ReasoningStep) -> String {
        format!("{} - {}", step.description, step.reasoning)
    }
    
    fn assess_reasoning_quality(&self, reasoning_chain: &[ReasoningStep]) -> f64 {
        if reasoning_chain.is_empty() {
            return 0.0;
        }
        
        let avg_confidence = reasoning_chain.iter().map(|s| s.confidence).sum::<f64>() / reasoning_chain.len() as f64;
        let depth_factor = (reasoning_chain.len() as f64 / 5.0).min(1.0);
        
        (avg_confidence + depth_factor) / 2.0
    }
    
    fn assess_logical_consistency(&self, reasoning_chain: &[ReasoningStep]) -> f64 {
        // Simple consistency check - in a real implementation, this would be more sophisticated
        if reasoning_chain.len() < 2 {
            return 1.0;
        }
        
        // Check for contradictions or inconsistencies
        let mut consistency_score = 1.0;
        
        for i in 1..reasoning_chain.len() {
            let prev_confidence = reasoning_chain[i-1].confidence;
            let curr_confidence = reasoning_chain[i].confidence;
            
            // Penalize large confidence drops without explanation
            if curr_confidence < prev_confidence - 0.3 {
                consistency_score -= 0.1;
            }
        }
        
        consistency_score.max(0.0)
    }
}

/// Comprehensive explanation structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveExplanation {
    pub main_explanation: String,
    pub reasoning_explanation: ReasoningExplanation,
    pub confidence_explanation: ConfidenceExplanation,
    pub uncertainty_explanation: UncertaintyExplanation,
    pub source_attribution: SourceAttribution,
    pub decision_visualization: DecisionVisualization,
    pub alternatives_explanation: AlternativesExplanation,
    pub limitations: LimitationAcknowledgment,
    pub detail_level: ExplanationDetailLevel,
    pub transparency_score: f64,
}

/// Reasoning explanation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningExplanation {
    pub step_explanations: Vec<String>,
    pub overall_reasoning_quality: f64,
    pub reasoning_depth: usize,
    pub logical_consistency: f64,
}

/// Confidence explanation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceExplanation {
    pub overall_confidence: f64,
    pub confidence_breakdown: HashMap<String, f64>,
    pub confidence_interpretation: String,
    pub factors_affecting_confidence: Vec<String>,
}

/// Uncertainty explanation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncertaintyExplanation {
    pub uncertainty_level: f64,
    pub uncertainty_sources: Vec<String>,
    pub uncertainty_impact: String,
    pub mitigation_strategies: Vec<String>,
}

/// Source attribution information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceAttribution {
    pub primary_sources: Vec<String>,
    pub reasoning_sources: Vec<String>,
    pub knowledge_sources: Vec<String>,
    pub attribution_confidence: f64,
}

/// Decision process visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionVisualization {
    pub process_steps: Vec<VisualizationStep>,
    pub decision_flow: String,
    pub complexity_score: f64,
}

/// Visualization step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationStep {
    pub step_number: usize,
    pub step_name: String,
    pub step_type: String,
    pub confidence: f64,
    pub connections: Vec<usize>,
}

/// Alternative options explanation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativesExplanation {
    pub alternatives_considered: Vec<AlternativeOption>,
    pub selection_criteria: Vec<String>,
    pub trade_offs_made: Vec<String>,
}

/// Alternative option details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeOption {
    pub description: String,
    pub confidence: f64,
    pub pros: Vec<String>,
    pub cons: Vec<String>,
    pub why_not_chosen: String,
}

/// Limitation acknowledgment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitationAcknowledgment {
    pub known_limitations: Vec<String>,
    pub uncertainty_areas: Vec<String>,
    pub recommendation: String,
}

/// Explanation record for learning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationRecord {
    pub timestamp: SystemTime,
    pub user_id: String,
    pub explanation_type: String,
    pub detail_level: ExplanationDetailLevel,
    pub transparency_score: f64,
    pub user_satisfaction: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_transparency_module_creation() {
        let module = TransparencyModule::new().await.unwrap();
        assert_eq!(module.config.default_detail_level, ExplanationDetailLevel::Medium);
    }
    
    #[tokio::test]
    async fn test_comprehensive_explanation_generation() {
        let mut module = TransparencyModule::new().await.unwrap();
        
        let response = ConsciousnessResponse {
            id: "test".to_string(),
            content: "Test response".to_string(),
            consciousness_state: ConsciousnessState {
                awareness_level: 0.8,
                emotional_state: EmotionalState {
                    primary_emotion: EmotionType::Calm,
                    intensity: 0.7,
                    valence: 0.0,
                    arousal: 0.5,
                    secondary_emotions: vec![],
                },
                cognitive_load: 0.3,
                confidence_score: 0.8,
                meta_cognitive_depth: 3,
                timestamp: SystemTime::now(),
            },
            confidence: 0.8,
            ethical_assessment: EthicalAssessment {
                overall_score: 0.9,
                frameworks_used: vec!["utilitarian".to_string()],
                ethical_concerns: vec![],
                confidence: 0.9,
            },
            emotional_state: EmotionalState {
                primary_emotion: EmotionType::Calm,
                intensity: 0.7,
                valence: 0.0,
                arousal: 0.5,
                secondary_emotions: vec![],
            },
            creativity_score: 0.7,
            empathy_score: 0.8,
            reasoning_chain: vec![
                ReasoningStep {
                    step_type: ReasoningStepType::Analysis,
                    description: "Analyze user input".to_string(),
                    reasoning: "Understanding user intent".to_string(),
                    confidence: 0.8,
                    evidence: vec!["User question".to_string()],
                    metadata: HashMap::new(),
                },
            ],
            reasoning_explanation: "Test reasoning".to_string(),
            processing_time: Duration::from_millis(100),
            timestamp: SystemTime::now(),
            metadata: ResponseMetadata {
                input_id: "test".to_string(),
                session_id: "test_session".to_string(),
                processing_modules: vec!["consciousness".to_string()],
                quality_score: 0.8,
            },
        };
        
        let explanation = module.generate_comprehensive_explanation(
            &response,
            &response.reasoning_chain,
            "test_user",
        ).await.unwrap();
        
        assert!(!explanation.main_explanation.is_empty());
        assert!(explanation.transparency_score > 0.0);
        assert!(!explanation.reasoning_explanation.step_explanations.is_empty());
    }
    
    #[tokio::test]
    async fn test_user_preferences() {
        let mut module = TransparencyModule::new().await.unwrap();
        
        let preferences = ExplanationPreferences {
            preferred_detail_level: ExplanationDetailLevel::Technical,
            include_technical_details: true,
            include_confidence_scores: true,
            include_alternative_options: false,
            language_style: "technical".to_string(),
        };
        
        module.set_user_preferences("test_user".to_string(), preferences).await.unwrap();
        
        assert!(module.user_preferences.contains_key("test_user"));
        assert_eq!(
            module.user_preferences.get("test_user").unwrap().preferred_detail_level,
            ExplanationDetailLevel::Technical
        );
    }
}