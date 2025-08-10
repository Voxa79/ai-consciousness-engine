//! Emotional Processing Systems for Consciousness Engine
//! 
//! This module implements sophisticated emotional intelligence including empathy,
//! emotional regulation, and creative emotional expression.

use crate::types::*;
use crate::error::ConsciousnessError;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use serde::{Deserialize, Serialize};

/// Emotional processing engine
pub struct EmotionalEngine {
    /// Current emotional state
    current_state: EmotionalState,
    
    /// Emotional history
    emotional_history: Vec<EmotionalStateEntry>,
    
    /// Emotional regulation strategies
    regulation_strategies: HashMap<EmotionType, Vec<RegulationStrategy>>,
    
    /// Configuration
    config: EmotionalConfig,
}

/// Emotional state entry with timestamp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalStateEntry {
    /// Emotional state
    pub state: EmotionalState,
    
    /// Trigger that caused this state
    pub trigger: String,
    
    /// Timestamp
    pub timestamp: SystemTime,
    
    /// Duration of this state
    pub duration: Duration,
}

/// Emotional regulation strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegulationStrategy {
    /// Cognitive reappraisal
    CognitiveReappraisal,
    /// Emotional suppression
    Suppression,
    /// Mindfulness
    Mindfulness,
    /// Problem solving
    ProblemSolving,
    /// Social support seeking
    SocialSupport,
    /// Distraction
    Distraction,
}

/// Emotional configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalConfig {
    /// Emotional sensitivity level
    pub sensitivity: f64,
    
    /// Emotional stability factor
    pub stability: f64,
    
    /// Enable emotional regulation
    pub regulation_enabled: bool,
    
    /// Maximum emotional intensity
    pub max_intensity: f64,
}

impl Default for EmotionalConfig {
    fn default() -> Self {
        Self {
            sensitivity: 0.8,
            stability: 0.9,
            regulation_enabled: true,
            max_intensity: 1.0,
        }
    }
}

impl EmotionalEngine {
    /// Create a new emotional engine
    pub async fn new() -> Result<Self, ConsciousnessError> {
        let config = EmotionalConfig::default();
        
        // Initialize with calm state
        let initial_state = EmotionalState {
            primary_emotion: EmotionType::Calm,
            intensity: 0.5,
            valence: 0.1,
            arousal: 0.3,
            secondary_emotions: Vec::new(),
        };
        
        // Initialize regulation strategies
        let mut regulation_strategies = HashMap::new();
        regulation_strategies.insert(EmotionType::Anger, vec![
            RegulationStrategy::CognitiveReappraisal,
            RegulationStrategy::Mindfulness,
        ]);
        regulation_strategies.insert(EmotionType::Anxiety, vec![
            RegulationStrategy::ProblemSolving,
            RegulationStrategy::Mindfulness,
        ]);
        regulation_strategies.insert(EmotionType::Sadness, vec![
            RegulationStrategy::SocialSupport,
            RegulationStrategy::CognitiveReappraisal,
        ]);
        
        Ok(Self {
            current_state: initial_state,
            emotional_history: Vec::new(),
            regulation_strategies,
            config,
        })
    }
    
    /// Process emotional context from input
    pub async fn process_emotional_context(
        &mut self,
        input: &str,
        consciousness_state: &ConsciousnessState,
    ) -> Result<EmotionalContext, ConsciousnessError> {
        // Detect user emotions from input
        let user_emotions = self.detect_user_emotions(input).await?;
        
        // Generate appropriate emotional response
        let engine_emotions = self.generate_emotional_response(input, &user_emotions, consciousness_state).await?;
        
        // Calculate empathy alignment
        let empathy_alignment = self.calculate_empathy_alignment(&user_emotions, &engine_emotions).await?;
        
        // Assess appropriateness
        let appropriateness_score = self.assess_emotional_appropriateness(&engine_emotions, input).await?;
        
        // Update current state
        self.current_state = engine_emotions.clone();
        
        // Store in history
        self.emotional_history.push(EmotionalStateEntry {
            state: engine_emotions.clone(),
            trigger: input.to_string(),
            timestamp: SystemTime::now(),
            duration: Duration::from_secs(60), // Default duration
        });
        
        // Limit history size
        if self.emotional_history.len() > 1000 {
            self.emotional_history.remove(0);
        }
        
        Ok(EmotionalContext {
            user_emotions,
            engine_emotions,
            empathy_alignment,
            appropriateness_score,
        })
    }
    
    /// Reset emotional state
    pub async fn reset_emotional_state(&mut self) -> Result<(), ConsciousnessError> {
        self.current_state = EmotionalState {
            primary_emotion: EmotionType::Calm,
            intensity: 0.5,
            valence: 0.0,
            arousal: 0.3,
            secondary_emotions: Vec::new(),
        };
        Ok(())
    }
    
    // Helper methods
    
    async fn detect_user_emotions(&self, input: &str) -> Result<Vec<(EmotionType, f64)>, ConsciousnessError> {
        let mut detected_emotions = Vec::new();
        let input_lower = input.to_lowercase();
        
        // Simple keyword-based emotion detection
        let emotion_keywords = [
            (EmotionType::Joy, vec!["happy", "joy", "excited", "great", "wonderful", "amazing"]),
            (EmotionType::Sadness, vec!["sad", "depressed", "down", "unhappy", "miserable"]),
            (EmotionType::Anger, vec!["angry", "mad", "furious", "annoyed", "frustrated"]),
            (EmotionType::Fear, vec!["scared", "afraid", "worried", "anxious", "nervous"]),
            (EmotionType::Surprise, vec!["surprised", "shocked", "amazed", "unexpected"]),
            (EmotionType::Curiosity, vec!["curious", "wondering", "interested", "question"]),
        ];
        
        for (emotion, keywords) in &emotion_keywords {
            let matches = keywords.iter().filter(|keyword| input_lower.contains(*keyword)).count();
            if matches > 0 {
                let intensity = (matches as f64 / keywords.len() as f64).min(1.0);
                detected_emotions.push((*emotion, intensity));
            }
        }
        
        // If no emotions detected, assume neutral
        if detected_emotions.is_empty() {
            detected_emotions.push((EmotionType::Calm, 0.5));
        }
        
        Ok(detected_emotions)
    }
    
    async fn generate_emotional_response(
        &self,
        input: &str,
        user_emotions: &[(EmotionType, f64)],
        consciousness_state: &ConsciousnessState,
    ) -> Result<EmotionalState, ConsciousnessError> {
        // Base emotional response on user emotions and consciousness state
        let primary_user_emotion = user_emotions.iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(emotion, intensity)| (*emotion, *intensity))
            .unwrap_or((EmotionType::Calm, 0.5));
        
        // Generate appropriate response emotion
        let response_emotion = match primary_user_emotion.0 {
            EmotionType::Sadness => EmotionType::Empathy,
            EmotionType::Anger => EmotionType::Understanding,
            EmotionType::Fear => EmotionType::Calm,
            EmotionType::Joy => EmotionType::Joy,
            EmotionType::Curiosity => EmotionType::Excitement,
            _ => EmotionType::Understanding,
        };
        
        // Calculate intensity based on consciousness state and user emotion
        let base_intensity = primary_user_emotion.1 * 0.8; // Slightly lower than user
        let consciousness_factor = consciousness_state.awareness_level;
        let final_intensity = (base_intensity * consciousness_factor).min(self.config.max_intensity);
        
        // Calculate valence and arousal
        let valence = self.calculate_emotion_valence(response_emotion);
        let arousal = self.calculate_emotion_arousal(response_emotion, final_intensity);
        
        // Generate secondary emotions
        let secondary_emotions = self.generate_secondary_emotions(&response_emotion, user_emotions).await?;
        
        Ok(EmotionalState {
            primary_emotion: response_emotion,
            intensity: final_intensity,
            valence,
            arousal,
            secondary_emotions,
        })
    }
    
    async fn calculate_empathy_alignment(
        &self,
        user_emotions: &[(EmotionType, f64)],
        engine_emotions: &EmotionalState,
    ) -> Result<f64, ConsciousnessError> {
        if user_emotions.is_empty() {
            return Ok(0.5);
        }
        
        let primary_user_emotion = user_emotions[0].0;
        
        // Calculate alignment based on emotional appropriateness
        let alignment = match (primary_user_emotion, engine_emotions.primary_emotion) {
            (EmotionType::Sadness, EmotionType::Empathy) => 0.9,
            (EmotionType::Joy, EmotionType::Joy) => 0.9,
            (EmotionType::Anger, EmotionType::Understanding) => 0.8,
            (EmotionType::Fear, EmotionType::Calm) => 0.8,
            (EmotionType::Curiosity, EmotionType::Excitement) => 0.9,
            _ => 0.6,
        };
        
        Ok(alignment)
    }
    
    async fn assess_emotional_appropriateness(
        &self,
        engine_emotions: &EmotionalState,
        input: &str,
    ) -> Result<f64, ConsciousnessError> {
        // Simple appropriateness assessment
        let input_lower = input.to_lowercase();
        
        // Check for inappropriate emotional responses
        let inappropriate_patterns = [
            ("death", EmotionType::Joy),
            ("tragedy", EmotionType::Excitement),
            ("celebration", EmotionType::Sadness),
        ];
        
        for (keyword, inappropriate_emotion) in &inappropriate_patterns {
            if input_lower.contains(keyword) && engine_emotions.primary_emotion == *inappropriate_emotion {
                return Ok(0.2); // Very inappropriate
            }
        }
        
        // Generally appropriate if no red flags
        Ok(0.9)
    }
    
    fn calculate_emotion_valence(&self, emotion: EmotionType) -> f64 {
        match emotion {
            EmotionType::Joy | EmotionType::Excitement | EmotionType::Pride | EmotionType::Hope => 0.8,
            EmotionType::Satisfaction | EmotionType::Calm | EmotionType::Understanding => 0.3,
            EmotionType::Curiosity | EmotionType::Wonder => 0.1,
            EmotionType::Confusion | EmotionType::Surprise => 0.0,
            EmotionType::Anxiety | EmotionType::Frustration => -0.3,
            EmotionType::Sadness | EmotionType::Fear | EmotionType::Anger => -0.7,
            EmotionType::Shame | EmotionType::Guilt => -0.8,
            _ => 0.0,
        }
    }
    
    fn calculate_emotion_arousal(&self, emotion: EmotionType, intensity: f64) -> f64 {
        let base_arousal = match emotion {
            EmotionType::Excitement | EmotionType::Anger | EmotionType::Fear | EmotionType::Anxiety => 0.8,
            EmotionType::Joy | EmotionType::Surprise | EmotionType::Frustration => 0.6,
            EmotionType::Curiosity | EmotionType::Hope | EmotionType::Pride => 0.4,
            EmotionType::Understanding | EmotionType::Empathy => 0.3,
            EmotionType::Calm | EmotionType::Satisfaction => 0.2,
            EmotionType::Sadness | EmotionType::Shame | EmotionType::Guilt => 0.1,
            _ => 0.3,
        };
        
        base_arousal * intensity
    }
    
    async fn generate_secondary_emotions(
        &self,
        primary_emotion: &EmotionType,
        user_emotions: &[(EmotionType, f64)],
    ) -> Result<Vec<(EmotionType, f64)>, ConsciousnessError> {
        let mut secondary = Vec::new();
        
        // Add complementary emotions based on primary emotion
        match primary_emotion {
            EmotionType::Empathy => {
                secondary.push((EmotionType::Understanding, 0.7));
                secondary.push((EmotionType::Calm, 0.5));
            },
            EmotionType::Joy => {
                secondary.push((EmotionType::Excitement, 0.6));
                secondary.push((EmotionType::Satisfaction, 0.4));
            },
            EmotionType::Understanding => {
                secondary.push((EmotionType::Curiosity, 0.5));
                secondary.push((EmotionType::Calm, 0.6));
            },
            _ => {
                secondary.push((EmotionType::Understanding, 0.4));
            }
        }
        
        // Add mirrored user emotions at lower intensity
        for (user_emotion, intensity) in user_emotions {
            if *user_emotion != *primary_emotion {
                secondary.push((*user_emotion, intensity * 0.3));
            }
        }
        
        // Limit to top 3 secondary emotions
        secondary.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        secondary.truncate(3);
        
        Ok(secondary)
    }
}

/// Empathy system for emotional understanding
pub struct EmpathySystem {
    /// Empathy level
    empathy_level: f64,
    
    /// Emotional mirroring capability
    mirroring_strength: f64,
    
    /// Configuration
    config: EmpathyConfig,
}

/// Empathy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmpathyConfig {
    /// Base empathy level
    pub base_empathy: f64,
    
    /// Enable emotional mirroring
    pub mirroring_enabled: bool,
    
    /// Empathy adaptation rate
    pub adaptation_rate: f64,
}

impl Default for EmpathyConfig {
    fn default() -> Self {
        Self {
            base_empathy: 0.9,
            mirroring_enabled: true,
            adaptation_rate: 0.1,
        }
    }
}

impl EmpathySystem {
    /// Create a new empathy system
    pub async fn new() -> Result<Self, ConsciousnessError> {
        let config = EmpathyConfig::default();
        
        Ok(Self {
            empathy_level: config.base_empathy,
            mirroring_strength: 0.8,
            config,
        })
    }
    
    /// Generate empathetic response
    pub async fn generate_empathetic_response(
        &mut self,
        reasoning_result: &ConsciousnessReasoningResult,
        emotional_context: &EmotionalContext,
    ) -> Result<EmpatheticResponse, ConsciousnessError> {
        // Calculate empathy score based on emotional alignment
        let empathy_score = self.calculate_empathy_score(emotional_context).await?;
        
        // Generate empathetic content
        let empathetic_content = self.generate_empathetic_content(
            &reasoning_result.conclusion,
            emotional_context,
            empathy_score,
        ).await?;
        
        // Calculate emotional alignment
        let emotional_alignment = emotional_context.empathy_alignment;
        
        // Calculate appropriateness
        let appropriateness_score = emotional_context.appropriateness_score;
        
        Ok(EmpatheticResponse {
            content: empathetic_content,
            empathy_score,
            emotional_alignment,
            appropriateness_score,
        })
    }
    
    // Helper methods
    
    async fn calculate_empathy_score(&self, emotional_context: &EmotionalContext) -> Result<f64, ConsciousnessError> {
        let base_score = self.empathy_level;
        let alignment_bonus = emotional_context.empathy_alignment * 0.2;
        let appropriateness_bonus = emotional_context.appropriateness_score * 0.1;
        
        Ok((base_score + alignment_bonus + appropriateness_bonus).min(1.0))
    }
    
    async fn generate_empathetic_content(
        &self,
        base_content: &str,
        emotional_context: &EmotionalContext,
        empathy_score: f64,
    ) -> Result<String, ConsciousnessError> {
        let primary_user_emotion = emotional_context.user_emotions.first()
            .map(|(emotion, _)| *emotion)
            .unwrap_or(EmotionType::Calm);
        
        // Add empathetic prefix based on user emotion
        let empathetic_prefix = match primary_user_emotion {
            EmotionType::Sadness => "I understand this might be difficult for you. ",
            EmotionType::Anger => "I can sense your frustration. ",
            EmotionType::Fear => "I recognize your concerns. ",
            EmotionType::Joy => "I'm glad to hear your positive feelings. ",
            EmotionType::Anxiety => "I understand you might be feeling anxious about this. ",
            _ => "I appreciate you sharing this with me. ",
        };
        
        // Adjust empathetic tone based on empathy score
        let tone_adjustment = if empathy_score > 0.8 {
            " I'm here to support you through this."
        } else if empathy_score > 0.6 {
            " I want to help you with this."
        } else {
            ""
        };
        
        Ok(format!("{}{}{}", empathetic_prefix, base_content, tone_adjustment))
    }
}

/// Creative emotions system
pub struct CreativeEmotions {
    /// Creativity level
    creativity_level: f64,
    
    /// Emotional creativity patterns
    creative_patterns: HashMap<EmotionType, Vec<CreativePattern>>,
    
    /// Configuration
    config: CreativeConfig,
}

/// Creative pattern for emotional expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativePattern {
    /// Pattern name
    pub name: String,
    
    /// Creative enhancement factor
    pub enhancement_factor: f64,
    
    /// Applicable contexts
    pub contexts: Vec<String>,
}

/// Creative configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeConfig {
    /// Base creativity level
    pub base_creativity: f64,
    
    /// Enable emotional creativity
    pub emotional_creativity_enabled: bool,
    
    /// Novelty threshold
    pub novelty_threshold: f64,
}

impl Default for CreativeConfig {
    fn default() -> Self {
        Self {
            base_creativity: 0.8,
            emotional_creativity_enabled: true,
            novelty_threshold: 0.6,
        }
    }
}

impl CreativeEmotions {
    /// Create a new creative emotions system
    pub async fn new() -> Result<Self, ConsciousnessError> {
        let config = CreativeConfig::default();
        
        // Initialize creative patterns
        let mut creative_patterns = HashMap::new();
        creative_patterns.insert(EmotionType::Joy, vec![
            CreativePattern {
                name: "Enthusiastic Expression".to_string(),
                enhancement_factor: 1.2,
                contexts: vec!["celebration".to_string(), "achievement".to_string()],
            }
        ]);
        creative_patterns.insert(EmotionType::Curiosity, vec![
            CreativePattern {
                name: "Exploratory Thinking".to_string(),
                enhancement_factor: 1.3,
                contexts: vec!["learning".to_string(), "discovery".to_string()],
            }
        ]);
        
        Ok(Self {
            creativity_level: config.base_creativity,
            creative_patterns,
            config,
        })
    }
    
    /// Enhance response with creativity
    pub async fn enhance_with_creativity(
        &mut self,
        empathetic_response: &EmpatheticResponse,
    ) -> Result<CreativeResponse, ConsciousnessError> {
        // Calculate creativity score
        let creativity_score = self.calculate_creativity_score(empathetic_response).await?;
        
        // Generate creative enhancements
        let enhanced_content = self.apply_creative_enhancements(
            &empathetic_response.content,
            creativity_score,
        ).await?;
        
        // Calculate novelty score
        let novelty_score = self.calculate_novelty_score(&enhanced_content).await?;
        
        // Calculate usefulness score
        let usefulness_score = self.calculate_usefulness_score(&enhanced_content, empathetic_response).await?;
        
        Ok(CreativeResponse {
            content: enhanced_content,
            creativity_score,
            novelty_score,
            usefulness_score,
        })
    }
    
    // Helper methods
    
    async fn calculate_creativity_score(&self, empathetic_response: &EmpatheticResponse) -> Result<f64, ConsciousnessError> {
        let base_creativity = self.creativity_level;
        let empathy_boost = empathetic_response.empathy_score * 0.1;
        let emotional_boost = empathetic_response.emotional_alignment * 0.1;
        
        Ok((base_creativity + empathy_boost + emotional_boost).min(1.0))
    }
    
    async fn apply_creative_enhancements(&self, content: &str, creativity_score: f64) -> Result<String, ConsciousnessError> {
        if creativity_score < self.config.novelty_threshold {
            return Ok(content.to_string());
        }
        
        // Apply creative enhancements based on creativity score
        let enhanced_content = if creativity_score > 0.8 {
            // High creativity: add metaphors and creative language
            self.add_creative_language(content).await?
        } else if creativity_score > 0.6 {
            // Medium creativity: add interesting perspectives
            self.add_interesting_perspectives(content).await?
        } else {
            // Low creativity: minor enhancements
            self.add_minor_enhancements(content).await?
        };
        
        Ok(enhanced_content)
    }
    
    async fn add_creative_language(&self, content: &str) -> Result<String, ConsciousnessError> {
        // Add creative metaphors and language patterns
        let creative_phrases = [
            "Like a bridge connecting ideas",
            "As we navigate this together",
            "Think of it as a puzzle where",
            "Imagine if we could",
        ];
        
        let random_phrase = creative_phrases[content.len() % creative_phrases.len()];
        Ok(format!("{}. {}, {}", content, random_phrase, "we might discover new possibilities"))
    }
    
    async fn add_interesting_perspectives(&self, content: &str) -> Result<String, ConsciousnessError> {
        let perspective_additions = [
            " Have you considered looking at this from a different angle?",
            " What if we approached this creatively?",
            " There might be an innovative solution here.",
        ];
        
        let addition = perspective_additions[content.len() % perspective_additions.len()];
        Ok(format!("{}{}", content, addition))
    }
    
    async fn add_minor_enhancements(&self, content: &str) -> Result<String, ConsciousnessError> {
        // Minor creative touches
        Ok(format!("{} Let me know if you'd like to explore this further.", content))
    }
    
    async fn calculate_novelty_score(&self, content: &str) -> Result<f64, ConsciousnessError> {
        // Simple novelty calculation based on unique words and creative phrases
        let words: Vec<&str> = content.split_whitespace().collect();
        let unique_words: std::collections::HashSet<&str> = words.iter().cloned().collect();
        
        let novelty = unique_words.len() as f64 / words.len() as f64;
        Ok(novelty.min(1.0))
    }
    
    async fn calculate_usefulness_score(&self, content: &str, empathetic_response: &EmpatheticResponse) -> Result<f64, ConsciousnessError> {
        // Usefulness based on content length, empathy, and practical value
        let length_factor = (content.len() as f64 / 200.0).min(1.0);
        let empathy_factor = empathetic_response.empathy_score;
        let practical_keywords = ["help", "solution", "approach", "consider", "try"];
        
        let practical_count = practical_keywords.iter()
            .filter(|keyword| content.to_lowercase().contains(*keyword))
            .count();
        
        let practical_factor = (practical_count as f64 / practical_keywords.len() as f64).min(1.0);
        
        Ok((length_factor + empathy_factor + practical_factor) / 3.0)
    }
}