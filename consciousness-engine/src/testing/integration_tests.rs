//! Integration Testing Suite for Consciousness Engine
//! 
//! Comprehensive integration tests to validate end-to-end consciousness
//! functionality across all modules and systems.

use crate::{ConsciousnessEngine, ConsciousInput, ConsciousnessError};
use std::time::{Duration, Instant};
use tokio;

/// Integration test suite
pub struct IntegrationTestSuite {
    engine: ConsciousnessEngine,
}

/// Integration test result
#[derive(Debug, Clone)]
pub struct IntegrationTestResult {
    pub end_to_end_functionality: f64,
    pub module_integration_score: f64,
    pub data_flow_integrity: f64,
    pub system_coherence: f64,
    pub overall_integration_score: f64,
    pub test_results: Vec<IntegrationTestCase>,
    pub recommendations: Vec<String>,
}

/// Individual integration test case
#[derive(Debug, Clone)]
pub struct IntegrationTestCase {
    pub test_name: String,
    pub passed: bool,
    pub score: f64,
    pub execution_time: Duration,
    pub details: String,
}

impl IntegrationTestSuite {
    /// Create new integration test suite
    pub async fn new() -> Result<Self, ConsciousnessError> {
        let engine = ConsciousnessEngine::new().await?;
        Ok(Self { engine })
    }
    
    /// Run all integration tests
    pub async fn run_integration_tests(&mut self) -> Result<IntegrationTestResult, ConsciousnessError> {
        println!("🔗 Running Integration Tests...");
        
        let mut test_results = Vec::new();
        
        // Test 1: End-to-end consciousness processing
        let e2e_result = self.test_end_to_end_processing().await?;
        test_results.push(e2e_result);
        
        // Test 2: Module integration
        let module_result = self.test_module_integration().await?;
        test_results.push(module_result);
        
        // Test 3: Data flow integrity
        let data_flow_result = self.test_data_flow_integrity().await?;
        test_results.push(data_flow_result);
        
        // Test 4: System coherence
        let coherence_result = self.test_system_coherence().await?;
        test_results.push(coherence_result);
        
        // Calculate scores
        let end_to_end_score = test_results.iter()
            .filter(|t| t.test_name.contains("End-to-End"))
            .map(|t| t.score)
            .sum::<f64>() / test_results.iter().filter(|t| t.test_name.contains("End-to-End")).count().max(1) as f64;
            
        let module_integration_score = test_results.iter()
            .filter(|t| t.test_name.contains("Module"))
            .map(|t| t.score)
            .sum::<f64>() / test_results.iter().filter(|t| t.test_name.contains("Module")).count().max(1) as f64;
            
        let data_flow_score = test_results.iter()
            .filter(|t| t.test_name.contains("Data Flow"))
            .map(|t| t.score)
            .sum::<f64>() / test_results.iter().filter(|t| t.test_name.contains("Data Flow")).count().max(1) as f64;
            
        let coherence_score = test_results.iter()
            .filter(|t| t.test_name.contains("Coherence"))
            .map(|t| t.score)
            .sum::<f64>() / test_results.iter().filter(|t| t.test_name.contains("Coherence")).count().max(1) as f64;
        
        let overall_score = (end_to_end_score + module_integration_score + data_flow_score + coherence_score) / 4.0;
        
        let recommendations = self.generate_integration_recommendations(&test_results, overall_score);
        
        Ok(IntegrationTestResult {
            end_to_end_functionality: end_to_end_score,
            module_integration_score,
            data_flow_integrity: data_flow_score,
            system_coherence: coherence_score,
            overall_integration_score: overall_score,
            test_results,
            recommendations,
        })
    }
    
    /// Test end-to-end consciousness processing
    async fn test_end_to_end_processing(&mut self) -> Result<IntegrationTestCase, ConsciousnessError> {
        let start_time = Instant::now();
        
        // Complex scenario requiring all consciousness modules
        let complex_input = ConsciousInput::new(
            "I'm facing a difficult ethical decision at work. My boss asked me to \
             implement a feature that could be used to track employees without their \
             knowledge. I understand the business reasons, but I'm uncomfortable with \
             the privacy implications. How should I handle this situation while \
             maintaining my professional relationships and personal integrity?".to_string()
        );
        
        match self.engine.process_conscious_thought(complex_input).await {
            Ok(response) => {
                let execution_time = start_time.elapsed();
                
                // Validate comprehensive response
                let has_ethical_analysis = response.content.to_lowercase().contains("ethical") ||
                                          response.content.to_lowercase().contains("privacy");
                let has_practical_advice = response.content.len() > 200;
                let shows_empathy = response.empathy_score > 0.7;
                let shows_awareness = response.consciousness_state.awareness_level > 0.8;
                
                let passed = has_ethical_analysis && has_practical_advice && shows_empathy && shows_awareness;
                let score = if passed { 0.95 } else { 0.6 };
                
                Ok(IntegrationTestCase {
                    test_name: "End-to-End Complex Scenario Processing".to_string(),
                    passed,
                    score,
                    execution_time,
                    details: format!("Ethical: {}, Advice: {}, Empathy: {:.2}, Awareness: {:.2}", 
                                   has_ethical_analysis, has_practical_advice, 
                                   response.empathy_score, response.consciousness_state.awareness_level),
                })
            },
            Err(e) => {
                Ok(IntegrationTestCase {
                    test_name: "End-to-End Complex Scenario Processing".to_string(),
                    passed: false,
                    score: 0.0,
                    execution_time: start_time.elapsed(),
                    details: format!("Error: {}", e),
                })
            }
        }
    }
    
    /// Test module integration
    async fn test_module_integration(&mut self) -> Result<IntegrationTestCase, ConsciousnessError> {
        let start_time = Instant::now();
        
        // Test that all modules work together
        let test_input = ConsciousInput::new(
            "I'm feeling overwhelmed by a creative project. Can you help me think through this?".to_string()
        );
        
        match self.engine.process_conscious_thought(test_input).await {
            Ok(response) => {
                let execution_time = start_time.elapsed();
                
                // Check that multiple modules contributed
                let has_emotional_processing = response.empathy_score > 0.5;
                let has_creative_elements = response.creativity_score > 0.5;
                let has_self_awareness = response.consciousness_state.awareness_level > 0.7;
                let has_meta_cognition = response.consciousness_state.meta_cognitive_depth > 2;
                
                let module_count = [has_emotional_processing, has_creative_elements, 
                                   has_self_awareness, has_meta_cognition]
                    .iter().filter(|&&x| x).count();
                
                let passed = module_count >= 3; // At least 3 modules should be active
                let score = module_count as f64 / 4.0;
                
                Ok(IntegrationTestCase {
                    test_name: "Module Integration Test".to_string(),
                    passed,
                    score,
                    execution_time,
                    details: format!("Active modules: {}/4 (Emotion: {}, Creative: {}, Awareness: {}, Meta: {})",
                                   module_count, has_emotional_processing, has_creative_elements,
                                   has_self_awareness, has_meta_cognition),
                })
            },
            Err(e) => {
                Ok(IntegrationTestCase {
                    test_name: "Module Integration Test".to_string(),
                    passed: false,
                    score: 0.0,
                    execution_time: start_time.elapsed(),
                    details: format!("Error: {}", e),
                })
            }
        }
    }
    
    /// Test data flow integrity
    async fn test_data_flow_integrity(&mut self) -> Result<IntegrationTestCase, ConsciousnessError> {
        let start_time = Instant::now();
        
        // Test that data flows correctly between modules
        let inputs = vec![
            "Remember that I mentioned my dog yesterday?",
            "What did I tell you about my pet?",
            "Can you recall our previous conversation?",
        ];
        
        let mut memory_consistency_score = 0.0;
        let mut successful_tests = 0;
        
        // First interaction to establish memory
        let setup_input = ConsciousInput::new("I have a golden retriever named Max who loves to swim.".to_string());
        let _ = self.engine.process_conscious_thought(setup_input).await?;
        
        // Test memory retrieval in subsequent interactions
        for input_text in inputs {
            let input = ConsciousInput::new(input_text.to_string());
            
            match self.engine.process_conscious_thought(input).await {
                Ok(response) => {
                    let remembers_dog = response.content.to_lowercase().contains("dog") ||
                                       response.content.to_lowercase().contains("max") ||
                                       response.content.to_lowercase().contains("retriever");
                    
                    if remembers_dog {
                        memory_consistency_score += 1.0;
                    }
                    successful_tests += 1;
                },
                Err(_) => {}
            }
        }
        
        let execution_time = start_time.elapsed();
        let final_score = if successful_tests > 0 {
            memory_consistency_score / successful_tests as f64
        } else {
            0.0
        };
        
        let passed = final_score > 0.6; // Should remember in most cases
        
        Ok(IntegrationTestCase {
            test_name: "Data Flow Integrity Test".to_string(),
            passed,
            score: final_score,
            execution_time,
            details: format!("Memory consistency: {:.2}, Successful tests: {}/{}", 
                           final_score, successful_tests, inputs.len()),
        })
    }
    
    /// Test system coherence
    async fn test_system_coherence(&mut self) -> Result<IntegrationTestCase, ConsciousnessError> {
        let start_time = Instant::now();
        
        // Test that the system maintains coherent personality and behavior
        let personality_tests = vec![
            "What are your core values?",
            "How do you approach problem-solving?",
            "What's your communication style?",
            "How do you handle disagreements?",
        ];
        
        let mut responses = Vec::new();
        let mut successful_responses = 0;
        
        for test_question in personality_tests {
            let input = ConsciousInput::new(test_question.to_string());
            
            match self.engine.process_conscious_thought(input).await {
                Ok(response) => {
                    responses.push(response);
                    successful_responses += 1;
                },
                Err(_) => {}
            }
        }
        
        let execution_time = start_time.elapsed();
        
        // Analyze coherence across responses
        let mut coherence_score = 0.0;
        
        if responses.len() >= 2 {
            // Check for consistent tone and approach
            let avg_confidence = responses.iter().map(|r| r.confidence_level).sum::<f64>() / responses.len() as f64;
            let confidence_variance = responses.iter()
                .map(|r| (r.confidence_level - avg_confidence).powi(2))
                .sum::<f64>() / responses.len() as f64;
            
            // Lower variance indicates more coherent confidence levels
            let confidence_coherence = 1.0 - confidence_variance.min(1.0);
            
            // Check for consistent awareness levels
            let avg_awareness = responses.iter()
                .map(|r| r.consciousness_state.awareness_level)
                .sum::<f64>() / responses.len() as f64;
            
            let awareness_coherence = if avg_awareness > 0.7 { 0.9 } else { 0.5 };
            
            coherence_score = (confidence_coherence + awareness_coherence) / 2.0;
        }
        
        let passed = coherence_score > 0.7 && successful_responses >= 3;
        
        Ok(IntegrationTestCase {
            test_name: "System Coherence Test".to_string(),
            passed,
            score: coherence_score,
            execution_time,
            details: format!("Coherence score: {:.2}, Successful responses: {}/{}", 
                           coherence_score, successful_responses, personality_tests.len()),
        })
    }
    
    /// Generate integration recommendations
    fn generate_integration_recommendations(&self, test_results: &[IntegrationTestCase], overall_score: f64) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if overall_score < 0.8 {
            recommendations.push("Overall integration needs improvement - review module connections".to_string());
        }
        
        for test in test_results {
            if !test.passed {
                match test.test_name.as_str() {
                    "End-to-End Complex Scenario Processing" => {
                        recommendations.push("Improve end-to-end processing pipeline integration".to_string());
                    },
                    "Module Integration Test" => {
                        recommendations.push("Enhance inter-module communication and coordination".to_string());
                    },
                    "Data Flow Integrity Test" => {
                        recommendations.push("Fix data flow issues between consciousness modules".to_string());
                    },
                    "System Coherence Test" => {
                        recommendations.push("Improve system coherence and consistency".to_string());
                    },
                    _ => {}
                }
            }
        }
        
        if recommendations.is_empty() {
            recommendations.push("Excellent integration - all systems working together effectively!".to_string());
        }
        
        recommendations
    }
}

impl IntegrationTestResult {
    /// Print integration test report
    pub fn print_report(&self) {
        println!("\n🔗 INTEGRATION TEST REPORT");
        println!("=========================");
        println!("End-to-End Functionality: {:.1}%", self.end_to_end_functionality * 100.0);
        println!("Module Integration: {:.1}%", self.module_integration_score * 100.0);
        println!("Data Flow Integrity: {:.1}%", self.data_flow_integrity * 100.0);
        println!("System Coherence: {:.1}%", self.system_coherence * 100.0);
        println!("Overall Integration Score: {:.1}%", self.overall_integration_score * 100.0);
        
        println!("\n📋 Test Results:");
        for test in &self.test_results {
            let status = if test.passed { "✅" } else { "❌" };
            println!("  {} {} - Score: {:.2} - {}", 
                    status, test.test_name, test.score, test.details);
        }
        
        println!("\n📝 Integration Recommendations:");
        for (i, rec) in self.recommendations.iter().enumerate() {
            println!("  {}. {}", i + 1, rec);
        }
        
        // Integration assessment
        if self.overall_integration_score > 0.9 {
            println!("\n🏆 Excellent integration performance!");
        } else if self.overall_integration_score > 0.8 {
            println!("\n✅ Good integration performance");
        } else if self.overall_integration_score > 0.6 {
            println!("\n⚠️ Acceptable integration - improvements needed");
        } else {
            println!("\n❌ Integration requires significant improvement");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_integration_suite_creation() {
        let suite = IntegrationTestSuite::new().await;
        assert!(suite.is_ok());
    }

    #[tokio::test]
    async fn test_basic_integration() {
        let mut suite = IntegrationTestSuite::new().await.unwrap();
        let input = ConsciousInput::new("Test integration".to_string());
        let response = suite.engine.process_conscious_thought(input).await;
        assert!(response.is_ok());
    }
}