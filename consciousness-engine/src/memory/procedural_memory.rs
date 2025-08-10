//! Procedural Memory - Skills, abilities, and learned procedures

use crate::{
    types::*,
    error::{ConsciousnessError, ConsciousnessResult},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tracing::{debug, info};

/// Procedural memory system for storing and executing skills and procedures
#[derive(Debug, Clone)]
pub struct ProceduralMemorySystem {
    skills: HashMap<String, Skill>,
    procedures: HashMap<String, Procedure>,
    skill_categories: HashMap<String, Vec<String>>,
    skill_dependencies: HashMap<String, Vec<String>>,
    learning_engine: SkillLearningEngine,
    execution_engine: SkillExecutionEngine,
    skill_evolution: SkillEvolutionTracker,
}

impl ProceduralMemorySystem {
    /// Create a new procedural memory system
    pub fn new() -> Self {
        info!("Initializing Procedural Memory System");
        
        Self {
            skills: HashMap::new(),
            procedures: HashMap::new(),
            skill_categories: HashMap::new(),
            skill_dependencies: HashMap::new(),
            learning_engine: SkillLearningEngine::new(),
            execution_engine: SkillExecutionEngine::new(),
            skill_evolution: SkillEvolutionTracker::new(),
        }
    }
    
    /// Store or update a skill in procedural memory
    pub async fn store_skill(
        &mut self,
        skill_name: String,
        description: String,
        proficiency_level: ProficiencyLevel,
        categories: Vec<String>,
        execution_steps: Vec<ExecutionStep>,
    ) -> ConsciousnessResult<()> {
        debug!("Storing skill: {}", skill_name);
        
        // Check if skill already exists
        if let Some(existing_skill) = self.skills.get_mut(&skill_name) {
            // Update existing skill
            existing_skill.description = description;
            existing_skill.proficiency_level = proficiency_level;
            existing_skill.execution_steps = execution_steps;
            existing_skill.last_updated = Utc::now();
            
            // Track skill evolution
            self.skill_evolution.track_skill_update(
                &skill_name,
                existing_skill.proficiency_level,
                proficiency_level,
            ).await?;
        } else {
            // Create new skill
            let skill = Skill {
                skill_id: Uuid::new_v4(),
                name: skill_name.clone(),
                description,
                proficiency_level,
                execution_steps,
                usage_count: 0,
                success_rate: 1.0,
                learning_curve: Vec::new(),
                last_used: None,
                last_updated: Utc::now(),
                created_at: Utc::now(),
            };
            
            self.skills.insert(skill_name.clone(), skill);
        }
        
        // Update skill categories
        for category in categories {
            self.skill_categories
                .entry(category)
                .or_insert_with(Vec::new)
                .push(skill_name.clone());
        }
        
        info!("Skill stored: {}", skill_name);
        
        Ok(())
    }
    
    /// Store a procedure (sequence of skills)
    pub async fn store_procedure(
        &mut self,
        procedure_name: String,
        description: String,
        skill_sequence: Vec<String>,
        success_criteria: SuccessCriteria,
    ) -> ConsciousnessResult<()> {
        debug!("Storing procedure: {}", procedure_name);
        
        // Validate that all skills in sequence exist
        for skill_name in &skill_sequence {
            if !self.skills.contains_key(skill_name) {
                return Err(ConsciousnessError::ConfigurationError { 
                    message: format!("Procedure references non-existent skill: {}", skill_name) 
                });
            }
        }
        
        // Create or update procedure
        let procedure = Procedure {
            procedure_id: Uuid::new_v4(),
            name: procedure_name.clone(),
            description,
            skill_sequence,
            success_criteria,
            execution_count: 0,
            success_rate: 1.0,
            average_execution_time: None,
            last_executed: None,
            created_at: Utc::now(),
            last_updated: Utc::now(),
        };
        
        self.procedures.insert(procedure_name.clone(), procedure);
        
        info!("Procedure stored: {}", procedure_name);
        
        Ok(())
    }
    
    /// Execute a skill
    pub async fn execute_skill(
        &mut self,
        skill_name: &str,
        context: &ExecutionContext,
    ) -> ConsciousnessResult<SkillExecutionResult> {
        debug!("Executing skill: {}", skill_name);
        
        // Get skill
        let skill = match self.skills.get_mut(skill_name) {
            Some(skill) => skill,
            None => return Err(ConsciousnessError::ConfigurationError { 
                message: format!("Skill not found: {}", skill_name) 
            }),
        };
        
        // Update usage statistics
        skill.usage_count += 1;
        skill.last_used = Some(Utc::now());
        
        // Execute skill
        let start_time = std::time::Instant::now();
        let execution_result = self.execution_engine.execute_skill(skill, context).await?;
        let execution_time = start_time.elapsed();
        
        // Update skill success rate
        if execution_result.success {
            skill.success_rate = (skill.success_rate * (skill.usage_count - 1) as f64 + 1.0) / skill.usage_count as f64;
        } else {
            skill.success_rate = (skill.success_rate * (skill.usage_count - 1) as f64) / skill.usage_count as f64;
        }
        
        // Track in learning curve
        skill.learning_curve.push(LearningPoint {
            timestamp: Utc::now(),
            proficiency: skill.proficiency_level.clone(),
            success: execution_result.success,
            execution_time: execution_time.as_millis() as u64,
        });
        
        // Limit learning curve size
        if skill.learning_curve.len() > 100 {
            skill.learning_curve.remove(0);
        }
        
        info!("Skill execution completed: {} (success: {})", skill_name, execution_result.success);
        
        Ok(execution_result)
    }
    
    /// Execute a procedure (sequence of skills)
    pub async fn execute_procedure(
        &mut self,
        procedure_name: &str,
        context: &ExecutionContext,
    ) -> ConsciousnessResult<ProcedureExecutionResult> {
        debug!("Executing procedure: {}", procedure_name);
        
        // Get procedure
        let procedure = match self.procedures.get_mut(procedure_name) {
            Some(procedure) => procedure,
            None => return Err(ConsciousnessError::ConfigurationError { 
                message: format!("Procedure not found: {}", procedure_name) 
            }),
        };
        
        // Update execution count
        procedure.execution_count += 1;
        procedure.last_executed = Some(Utc::now());
        
        // Execute each skill in sequence
        let start_time = std::time::Instant::now();
        let mut step_results = Vec::new();
        let mut success = true;
        
        for (step_index, skill_name) in procedure.skill_sequence.iter().enumerate() {
            debug!("Executing procedure step {}: {}", step_index, skill_name);
            
            // Execute skill
            let step_result = self.execute_skill(skill_name, context).await?;
            
            // If any step fails, the procedure fails
            if !step_result.success {
                success = false;
                step_results.push(ProcedureStepResult {
                    step_index,
                    skill_name: skill_name.clone(),
                    success: false,
                    output: step_result.output,
                    error: step_result.error,
                });
                
                // Stop execution if procedure requires all steps to succeed
                if matches!(procedure.success_criteria, SuccessCriteria::AllStepsSucceed) {
                    break;
                }
            } else {
                step_results.push(ProcedureStepResult {
                    step_index,
                    skill_name: skill_name.clone(),
                    success: true,
                    output: step_result.output,
                    error: None,
                });
            }
        }
        
        let execution_time = start_time.elapsed();
        
        // Update procedure success rate
        if success {
            procedure.success_rate = (procedure.success_rate * (procedure.execution_count - 1) as f64 + 1.0) 
                / procedure.execution_count as f64;
        } else {
            procedure.success_rate = (procedure.success_rate * (procedure.execution_count - 1) as f64) 
                / procedure.execution_count as f64;
        }
        
        // Update average execution time
        let execution_time_ms = execution_time.as_millis() as u64;
        procedure.average_execution_time = Some(match procedure.average_execution_time {
            Some(avg) => (avg * (procedure.execution_count - 1) as u64 + execution_time_ms) 
                / procedure.execution_count as u64,
            None => execution_time_ms,
        });
        
        let result = ProcedureExecutionResult {
            procedure_name: procedure_name.to_string(),
            success,
            step_results,
            execution_time_ms,
        };
        
        info!("Procedure execution completed: {} (success: {})", procedure_name, success);
        
        Ok(result)
    }
    
    /// Learn a new skill or improve existing skill
    pub async fn learn_skill(
        &mut self,
        skill_name: String,
        learning_context: &LearningContext,
    ) -> ConsciousnessResult<LearningResult> {
        debug!("Learning skill: {}", skill_name);
        
        let learning_result = self.learning_engine.learn_skill(
            &skill_name,
            learning_context,
            self.skills.get(&skill_name),
        ).await?;
        
        // Store or update skill
        self.store_skill(
            skill_name.clone(),
            learning_result.description.clone(),
            learning_result.proficiency_level.clone(),
            learning_result.categories.clone(),
            learning_result.execution_steps.clone(),
        ).await?;
        
        info!("Skill learning completed: {} (proficiency: {:?})", 
              skill_name, learning_result.proficiency_level);
        
        Ok(learning_result)
    }
    
    /// Get skills by category
    pub async fn get_skills_by_category(
        &self,
        category: &str,
    ) -> ConsciousnessResult<Vec<Skill>> {
        let mut category_skills = Vec::new();
        
        if let Some(skill_names) = self.skill_categories.get(category) {
            for skill_name in skill_names {
                if let Some(skill) = self.skills.get(skill_name) {
                    category_skills.push(skill.clone());
                }
            }
        }
        
        Ok(category_skills)
    }
    
    /// Get skill by name
    pub async fn get_skill(
        &self,
        skill_name: &str,
    ) -> ConsciousnessResult<Option<Skill>> {
        Ok(self.skills.get(skill_name).cloned())
    }
    
    /// Get procedure by name
    pub async fn get_procedure(
        &self,
        procedure_name: &str,
    ) -> ConsciousnessResult<Option<Procedure>> {
        Ok(self.procedures.get(procedure_name).cloned())
    }
    
    /// Establish dependency between skills
    pub async fn add_skill_dependency(
        &mut self,
        skill_name: &str,
        dependency_skill: &str,
    ) -> ConsciousnessResult<()> {
        // Validate both skills exist
        if !self.skills.contains_key(skill_name) {
            return Err(ConsciousnessError::ConfigurationError { 
                message: format!("Skill not found: {}", skill_name) 
            });
        }
        
        if !self.skills.contains_key(dependency_skill) {
            return Err(ConsciousnessError::ConfigurationError { 
                message: format!("Dependency skill not found: {}", dependency_skill) 
            });
        }
        
        // Add dependency
        self.skill_dependencies
            .entry(skill_name.to_string())
            .or_insert_with(Vec::new)
            .push(dependency_skill.to_string());
        
        Ok(())
    }
    
    /// Get skill dependencies
    pub async fn get_skill_dependencies(
        &self,
        skill_name: &str,
    ) -> ConsciousnessResult<Vec<String>> {
        Ok(self.skill_dependencies
            .get(skill_name)
            .cloned()
            .unwrap_or_default())
    }
    
    /// Consolidate procedural memory (improve frequently used skills)
    pub async fn consolidate_procedural_memory(&mut self) -> ConsciousnessResult<ConsolidationReport> {
        debug!("Starting procedural memory consolidation");
        
        let mut improved_skills = 0;
        let mut degraded_skills = 0;
        
        // Improve frequently used skills with high success rate
        for skill in self.skills.values_mut() {
            if skill.usage_count > 10 && skill.success_rate > 0.8 
                && !matches!(skill.proficiency_level, ProficiencyLevel::Expert) {
                
                // Improve proficiency level
                skill.proficiency_level = match skill.proficiency_level {
                    ProficiencyLevel::Novice => ProficiencyLevel::Intermediate,
                    ProficiencyLevel::Intermediate => ProficiencyLevel::Advanced,
                    ProficiencyLevel::Advanced => ProficiencyLevel::Expert,
                    ProficiencyLevel::Expert => ProficiencyLevel::Expert,
                };
                
                improved_skills += 1;
            } else if skill.usage_count > 5 && skill.success_rate < 0.3 
                && !matches!(skill.proficiency_level, ProficiencyLevel::Novice) {
                
                // Degrade proficiency level due to poor performance
                skill.proficiency_level = match skill.proficiency_level {
                    ProficiencyLevel::Expert => ProficiencyLevel::Advanced,
                    ProficiencyLevel::Advanced => ProficiencyLevel::Intermediate,
                    ProficiencyLevel::Intermediate => ProficiencyLevel::Novice,
                    ProficiencyLevel::Novice => ProficiencyLevel::Novice,
                };
                
                degraded_skills += 1;
            }
        }
        
        let report = ConsolidationReport {
            total_skills: self.skills.len(),
            improved_skills,
            degraded_skills,
            consolidation_time: Utc::now(),
        };
        
        info!("Procedural memory consolidation completed: {:?}", report);
        
        Ok(report)
    }
    
    /// Get procedural memory statistics
    pub fn get_statistics(&self) -> ProceduralMemoryStatistics {
        let total_skills = self.skills.len();
        let total_procedures = self.procedures.len();
        
        let skill_proficiency_counts = self.skills.values()
            .fold(HashMap::new(), |mut acc, skill| {
                *acc.entry(skill.proficiency_level.clone()).or_insert(0) += 1;
                acc
            });
        
        let frequently_used_skills = self.skills.values()
            .filter(|s| s.usage_count > 5)
            .count();
        
        let high_success_rate_skills = self.skills.values()
            .filter(|s| s.success_rate > 0.8 && s.usage_count > 0)
            .count();
        
        ProceduralMemoryStatistics {
            total_skills,
            total_procedures,
            skill_proficiency_counts,
            frequently_used_skills,
            high_success_rate_skills,
            average_success_rate: if total_skills > 0 {
                self.skills.values().map(|s| s.success_rate).sum::<f64>() / total_skills as f64
            } else {
                0.0
            },
        }
    }
}

/// Supporting structures
#[derive(Debug, Clone)]
struct SkillLearningEngine;

impl SkillLearningEngine {
    fn new() -> Self { Self }
    
    async fn learn_skill(
        &self,
        skill_name: &str,
        learning_context: &LearningContext,
        existing_skill: Option<&Skill>,
    ) -> ConsciousnessResult<LearningResult> {
        // Determine starting proficiency level
        let starting_proficiency = existing_skill
            .map(|s| s.proficiency_level.clone())
            .unwrap_or(ProficiencyLevel::Novice);
        
        // Calculate learning effectiveness
        let learning_effectiveness = self.calculate_learning_effectiveness(
            learning_context,
            &starting_proficiency,
        );
        
        // Determine new proficiency level based on learning effectiveness
        let new_proficiency = if learning_effectiveness > 0.8 {
            // Significant improvement
            match starting_proficiency {
                ProficiencyLevel::Novice => ProficiencyLevel::Intermediate,
                ProficiencyLevel::Intermediate => ProficiencyLevel::Advanced,
                ProficiencyLevel::Advanced => ProficiencyLevel::Expert,
                ProficiencyLevel::Expert => ProficiencyLevel::Expert,
            }
        } else if learning_effectiveness > 0.4 {
            // Moderate improvement within same level
            starting_proficiency
        } else {
            // Minimal learning, stay at current level
            starting_proficiency
        };
        
        // Generate execution steps based on learning context
        let execution_steps = self.generate_execution_steps(
            skill_name,
            learning_context,
            &new_proficiency,
        );
        
        Ok(LearningResult {
            skill_name: skill_name.to_string(),
            description: format!("Skill: {}", skill_name),
            proficiency_level: new_proficiency,
            categories: learning_context.categories.clone(),
            execution_steps,
            learning_effectiveness,
            learning_time: Utc::now(),
        })
    }
    
    fn calculate_learning_effectiveness(
        &self,
        context: &LearningContext,
        current_proficiency: &ProficiencyLevel,
    ) -> f64 {
        let mut effectiveness = 0.5; // Base effectiveness
        
        // Learning method effectiveness
        match context.learning_method {
            LearningMethod::Observation => effectiveness += 0.1,
            LearningMethod::Instruction => effectiveness += 0.3,
            LearningMethod::Practice => effectiveness += 0.5,
            LearningMethod::Feedback => effectiveness += 0.4,
        }
        
        // Adjust based on current proficiency (harder to improve at higher levels)
        match current_proficiency {
            ProficiencyLevel::Novice => effectiveness *= 1.0,
            ProficiencyLevel::Intermediate => effectiveness *= 0.8,
            ProficiencyLevel::Advanced => effectiveness *= 0.6,
            ProficiencyLevel::Expert => effectiveness *= 0.3,
        }
        
        // Adjust based on learning duration
        if context.learning_duration_minutes > 60 {
            effectiveness += 0.2;
        } else if context.learning_duration_minutes < 10 {
            effectiveness -= 0.2;
        }
        
        effectiveness.max(0.0).min(1.0)
    }
    
    fn generate_execution_steps(
        &self,
        _skill_name: &str,
        _context: &LearningContext,
        proficiency: &ProficiencyLevel,
    ) -> Vec<ExecutionStep> {
        // Generate appropriate steps based on proficiency
        let step_count = match proficiency {
            ProficiencyLevel::Novice => 2,
            ProficiencyLevel::Intermediate => 3,
            ProficiencyLevel::Advanced => 4,
            ProficiencyLevel::Expert => 5,
        };
        
        (0..step_count).map(|i| ExecutionStep {
            step_index: i,
            description: format!("Step {}", i + 1),
            expected_outcome: format!("Outcome {}", i + 1),
            error_handling: Some(format!("Error handling for step {}", i + 1)),
        }).collect()
    }
}

#[derive(Debug, Clone)]
struct SkillExecutionEngine;

impl SkillExecutionEngine {
    fn new() -> Self { Self }
    
    async fn execute_skill(
        &self,
        skill: &Skill,
        context: &ExecutionContext,
    ) -> ConsciousnessResult<SkillExecutionResult> {
        // Simulate skill execution based on proficiency and context
        let success_probability = self.calculate_success_probability(skill, context);
        
        // Determine success based on probability
        let random_value = rand::random::<f64>();
        let success = random_value <= success_probability;
        
        if success {
            Ok(SkillExecutionResult {
                skill_name: skill.name.clone(),
                success: true,
                output: Some(format!("Successfully executed skill: {}", skill.name)),
                error: None,
                execution_details: Some(format!("Executed with proficiency: {:?}", skill.proficiency_level)),
            })
        } else {
            Ok(SkillExecutionResult {
                skill_name: skill.name.clone(),
                success: false,
                output: None,
                error: Some(format!("Failed to execute skill: {}", skill.name)),
                execution_details: Some(format!("Failed with proficiency: {:?}", skill.proficiency_level)),
            })
        }
    }
    
    fn calculate_success_probability(
        &self,
        skill: &Skill,
        context: &ExecutionContext,
    ) -> f64 {
        let mut probability = match skill.proficiency_level {
            ProficiencyLevel::Novice => 0.5,
            ProficiencyLevel::Intermediate => 0.7,
            ProficiencyLevel::Advanced => 0.85,
            ProficiencyLevel::Expert => 0.95,
        };
        
        // Adjust based on context complexity
        match context.complexity {
            ComplexityLevel::Simple => probability += 0.1,
            ComplexityLevel::Moderate => {},
            ComplexityLevel::Complex => probability -= 0.1,
            ComplexityLevel::VeryComplex => probability -= 0.2,
            ComplexityLevel::ExtremelyComplex => probability -= 0.3,
        }
        
        // Adjust based on previous success rate
        if skill.usage_count > 0 {
            probability = (probability + skill.success_rate) / 2.0;
        }
        
        probability.max(0.1).min(0.99)
    }
}

#[derive(Debug, Clone)]
struct SkillEvolutionTracker {
    evolution_history: HashMap<String, Vec<SkillEvolutionEvent>>,
}

impl SkillEvolutionTracker {
    fn new() -> Self {
        Self {
            evolution_history: HashMap::new(),
        }
    }
    
    async fn track_skill_update(
        &mut self,
        skill_name: &str,
        old_proficiency: ProficiencyLevel,
        new_proficiency: ProficiencyLevel,
    ) -> ConsciousnessResult<()> {
        if old_proficiency != new_proficiency {
            let event = SkillEvolutionEvent {
                timestamp: Utc::now(),
                old_proficiency,
                new_proficiency,
                event_type: if new_proficiency > old_proficiency {
                    "improvement".to_string()
                } else {
                    "degradation".to_string()
                },
            };
            
            self.evolution_history
                .entry(skill_name.to_string())
                .or_insert_with(Vec::new)
                .push(event);
        }
        
        Ok(())
    }
}

/// Data structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub skill_id: Uuid,
    pub name: String,
    pub description: String,
    pub proficiency_level: ProficiencyLevel,
    pub execution_steps: Vec<ExecutionStep>,
    pub usage_count: u32,
    pub success_rate: f64,
    pub learning_curve: Vec<LearningPoint>,
    pub last_used: Option<DateTime<Utc>>,
    pub last_updated: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProficiencyLevel {
    Novice,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStep {
    pub step_index: u32,
    pub description: String,
    pub expected_outcome: String,
    pub error_handling: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPoint {
    pub timestamp: DateTime<Utc>,
    pub proficiency: ProficiencyLevel,
    pub success: bool,
    pub execution_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Procedure {
    pub procedure_id: Uuid,
    pub name: String,
    pub description: String,
    pub skill_sequence: Vec<String>,
    pub success_criteria: SuccessCriteria,
    pub execution_count: u32,
    pub success_rate: f64,
    pub average_execution_time: Option<u64>,
    pub last_executed: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuccessCriteria {
    AllStepsSucceed,
    MajorityStepsSucceed,
    FinalStepSucceeds,
    CustomCriteria(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningContext {
    pub learning_method: LearningMethod,
    pub learning_duration_minutes: u32,
    pub categories: Vec<String>,
    pub instructor: Option<String>,
    pub resources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LearningMethod {
    Observation,
    Instruction,
    Practice,
    Feedback,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContext {
    pub complexity: ComplexityLevel,
    pub time_pressure: TimePressure,
    pub resources_available: Vec<String>,
    pub environment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplexityLevel {
    Simple,
    Moderate,
    Complex,
    VeryComplex,
    ExtremelyComplex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimePressure {
    None,
    Low,
    Moderate,
    High,
    Extreme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillExecutionResult {
    pub skill_name: String,
    pub success: bool,
    pub output: Option<String>,
    pub error: Option<String>,
    pub execution_details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureExecutionResult {
    pub procedure_name: String,
    pub success: bool,
    pub step_results: Vec<ProcedureStepResult>,
    pub execution_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureStepResult {
    pub step_index: usize,
    pub skill_name: String,
    pub success: bool,
    pub output: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningResult {
    pub skill_name: String,
    pub description: String,
    pub proficiency_level: ProficiencyLevel,
    pub categories: Vec<String>,
    pub execution_steps: Vec<ExecutionStep>,
    pub learning_effectiveness: f64,
    pub learning_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationReport {
    pub total_skills: usize,
    pub improved_skills: u32,
    pub degraded_skills: u32,
    pub consolidation_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralMemoryStatistics {
    pub total_skills: usize,
    pub total_procedures: usize,
    pub skill_proficiency_counts: HashMap<ProficiencyLevel, u32>,
    pub frequently_used_skills: usize,
    pub high_success_rate_skills: usize,
    pub average_success_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SkillEvolutionEvent {
    timestamp: DateTime<Utc>,
    old_proficiency: ProficiencyLevel,
    new_proficiency: ProficiencyLevel,
    event_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_skill_storage_and_retrieval() {
        let mut memory = ProceduralMemorySystem::new();
        
        let skill_name = "test_skill".to_string();
        let description = "A test skill".to_string();
        let proficiency = ProficiencyLevel::Intermediate;
        let categories = vec!["testing".to_string()];
        let steps = vec![ExecutionStep {
            step_index: 0,
            description: "Test step".to_string(),
            expected_outcome: "Success".to_string(),
            error_handling: None,
        }];
        
        // Store skill
        memory.store_skill(
            skill_name.clone(),
            description.clone(),
            proficiency.clone(),
            categories.clone(),
            steps.clone(),
        ).await.unwrap();
        
        // Retrieve skill
        let retrieved_skill = memory.get_skill(&skill_name).await.unwrap();
        assert!(retrieved_skill.is_some());
        
        let skill = retrieved_skill.unwrap();
        assert_eq!(skill.name, skill_name);
        assert_eq!(skill.description, description);
        assert_eq!(skill.proficiency_level, proficiency);
    }

    #[tokio::test]
    async fn test_skill_learning() {
        let mut memory = ProceduralMemorySystem::new();
        
        let learning_context = LearningContext {
            learning_method: LearningMethod::Practice,
            learning_duration_minutes: 60,
            categories: vec!["learning".to_string()],
            instructor: Some("AI Trainer".to_string()),
            resources: vec!["Manual".to_string()],
        };
        
        let result = memory.learn_skill(
            "new_skill".to_string(),
            &learning_context,
        ).await.unwrap();
        
        assert_eq!(result.skill_name, "new_skill");
        assert!(!result.categories.is_empty());
        assert!(result.learning_effectiveness > 0.0);
    }

    #[tokio::test]
    async fn test_procedure_execution() {
        let mut memory = ProceduralMemorySystem::new();
        
        // First, store some skills
        memory.store_skill(
            "skill1".to_string(),
            "First skill".to_string(),
            ProficiencyLevel::Advanced,
            vec!["test".to_string()],
            vec![],
        ).await.unwrap();
        
        memory.store_skill(
            "skill2".to_string(),
            "Second skill".to_string(),
            ProficiencyLevel::Expert,
            vec!["test".to_string()],
            vec![],
        ).await.unwrap();
        
        // Store procedure
        memory.store_procedure(
            "test_procedure".to_string(),
            "A test procedure".to_string(),
            vec!["skill1".to_string(), "skill2".to_string()],
            SuccessCriteria::AllStepsSucceed,
        ).await.unwrap();
        
        // Execute procedure
        let context = ExecutionContext {
            complexity: ComplexityLevel::Moderate,
            time_pressure: TimePressure::Low,
            resources_available: vec!["computer".to_string()],
            environment: "test_environment".to_string(),
        };
        
        let result = memory.execute_procedure("test_procedure", &context).await.unwrap();
        assert_eq!(result.procedure_name, "test_procedure");
        assert_eq!(result.step_results.len(), 2);
    }
}