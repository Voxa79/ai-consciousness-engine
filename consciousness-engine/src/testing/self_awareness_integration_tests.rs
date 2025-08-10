//! Integration Tests for Self-Awareness Module
//! 
//! Comprehensive tests to validate the complete integration of the self-awareness
//! module with the consciousness engine core.

use crate::core::ConsciousnessEngine;
use crate::modules::self_awareness::{SelfAwarenessModule, Priority};
use crate::types::*;
use crate::error::ConsciousnessError;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

/// Test suite for self-awareness integration
pub struct SelfAwarenessIntegrationTests {
    engine: ConsciousnessEngine,
}

impl SelfAwarenessIntegrationTests {
    /// Create a new test suite
    pub async fn new() -> Result<Self, ConsciousnessError> {
        let engine = ConsciousnessEngine::new().await?;
        Ok(Self { engine })
    }
    
    /// Run all integration tests
    pub async fn run_all_tests(&mut self) -> Result<IntegrationTestResults, ConsciousnessError> {
        println!("🧠 Running Self-Awareness Integration Tests...");
        
        let mut results = IntegrationTestResults::new();
        
        // Test 1: Basic consciousness processing with self-awareness
        results.add_result(self.test_consciousness_processing_integration().await);
        
        // Test 2: Self-reflection generation
        results.add_result(self.test_self_reflection_integration().await);
        
        // Test 3: Growth opportunity identification
        results.add_result(self.test_growth_opportunities_integration().await);
        
        // Test 4: Performance analysis integration
        results.add_result(self.test_performance_analysis_integration().await);
        
        // Test 5: Real-time state monitoring
        results.add_result(self.test_real_time_monitoring().await);
        
        // Test 6: Experience integration
        results.add_result(self.test_experience_integration().await);
        
        // Test 7: Consciousness quality metrics
        results.add_result(self.test_consciousness_quality_metrics().await);
        
        // Test 8: Feedback loops
        results.add_result(self.test_feedback_loops().await);
        
        // Test 9: Error recovery and safe state
        results.add_result(self.test_error_recovery().await);
        
        // Test 10: Long-term consciousness development
        results.add_result(self.test_consciousness_development().await);
        
        println!("✅ Self-Awareness Integration Tests Completed");
        println!("📊 Results: {}/{} tests passed", results.passed_count(), results.total_count());
        
        Ok(results)
    }
    
    /// Test basic consciousness processing with self-awareness integration
    async fn test_consciousness_processing_integration(&mut self) -> TestResult {
        println!("🔍 Testing consciousness processing integration...");
        
        let input = ConsciousInput {
            id: "integration_test_1".to_string(),
            content: "I'm curious about how you understand yourself and your own thinking processes.".to_string(),
            context: UserContext {
                user_id: "test_user".to_string(),
                preferences: HashMap::new(),
                interaction_history: Vec::new(),
            },
            timestamp: std::time::SystemTime::now(),
            metadata: HashMap::new(),
        };
        
        match self.engine.process_conscious_thought(input).await {
            Ok(response) => {
                // Validate self-awareness integration
                let has_consciousness_state = response.consciousness_state.awareness_level > 0.0;
                let has_confidence = response.confidence > 0.0;
                let has_reasoning_chain = !response.reasoning_chain.is_empty();
                let has_quality_score = response.metadata.quality_score > 0.0;
                
                if has_consciousness_state && has_confidence && has_reasoning_chain && has_quality_score {
                    TestResult::passed(
                        "Consciousness Processing Integration",
                        format!("Successfully integrated self-awareness: awareness={:.2}, confidence={:.2}", 
                               response.consciousness_state.awareness_level, response.confidence)
                    )
                } else {
                    TestResult::failed(
                        "Consciousness Processing Integration",
                        format!("Missing integration components: consciousness={}, confidence={}, reasoning={}, quality={}", 
                               has_consciousness_state, has_confidence, has_reasoning_chain, has_quality_score)
                    )
                }
            },
            Err(e) => TestResult::failed(
                "Consciousness Processing Integration",
                format!("Processing failed: {:?}", e)
            ),
        }
    }
    
    /// Test self-reflection generation
    async fn test_self_reflection_integration(&mut self) -> TestResult {
        println!("🪞 Testing self-reflection integration...");
        
        // First, process some interactions to have data for reflection
        for i in 0..3 {
            let input = ConsciousInput {
                id: format!("reflection_prep_{}", i),
                content: format!("Test interaction {} for reflection preparation", i),
                context: UserContext {
                    user_id: "test_user".to_string(),
                    preferences: HashMap::new(),
                    interaction_history: Vec::new(),
                },
                timestamp: std::time::SystemTime::now(),
                metadata: HashMap::new(),
            };
            
            let _ = self.engine.process_conscious_thought(input).await;
        }
        
        // Now test self-reflection
        match self.engine.perform_self_reflection().await {
            Ok(reflection) => {
                let has_content = !reflection.content.is_empty();
                let has_insights = !reflection.insights.is_empty();
                let has_improvements = !reflection.improvement_areas.is_empty();
                let has_strengths = !reflection.strengths.is_empty();
                let has_questions = !reflection.questions.is_empty();
                let good_depth = reflection.depth_score > 0.3;
                
                if has_content && has_insights && has_improvements && has_strengths && has_questions && good_depth {
                    TestResult::passed(
                        "Self-Reflection Integration",
                        format!("Generated comprehensive reflection: depth={:.2}, insights={}, questions={}", 
                               reflection.depth_score, reflection.insights.len(), reflection.questions.len())
                    )
                } else {
                    TestResult::failed(
                        "Self-Reflection Integration",
                        format!("Incomplete reflection: content={}, insights={}, improvements={}, strengths={}, questions={}, depth={:.2}", 
                               has_content, has_insights, has_improvements, has_strengths, has_questions, reflection.depth_score)
                    )
                }
            },
            Err(e) => TestResult::failed(
                "Self-Reflection Integration",
                format!("Reflection failed: {:?}", e)
            ),
        }
    }
    
    /// Test growth opportunity identification
    async fn test_growth_opportunities_integration(&mut self) -> TestResult {
        println!("🌱 Testing growth opportunities integration...");
        
        match self.engine.identify_growth_opportunities().await {
            Ok(opportunities) => {
                let has_opportunities = !opportunities.is_empty();
                let has_high_priority = opportunities.iter().any(|o| o.priority == Priority::High);
                let has_actions = opportunities.iter().all(|o| !o.recommended_actions.is_empty());
                let has_metrics = opportunities.iter().all(|o| !o.success_metrics.is_empty());
                let reasonable_impact = opportunities.iter().all(|o| o.impact_score > 0.0 && o.impact_score <= 1.0);
                
                if has_opportunities && has_actions && has_metrics && reasonable_impact {
                    TestResult::passed(
                        "Growth Opportunities Integration",
                        format!("Identified {} opportunities, {} high priority", 
                               opportunities.len(), 
                               opportunities.iter().filter(|o| o.priority == Priority::High).count())
                    )
                } else {
                    TestResult::failed(
                        "Growth Opportunities Integration",
                        format!("Issues with opportunities: count={}, actions={}, metrics={}, impact={}", 
                               has_opportunities, has_actions, has_metrics, reasonable_impact)
                    )
                }
            },
            Err(e) => TestResult::failed(
                "Growth Opportunities Integration",
                format!("Growth opportunity identification failed: {:?}", e)
            ),
        }
    }
    
    /// Test performance analysis integration
    async fn test_performance_analysis_integration(&mut self) -> TestResult {
        println!("📈 Testing performance analysis integration...");
        
        match self.engine.get_performance_metrics().await {
            Ok(metrics) => {
                let has_response_times = !metrics.response_times.is_empty();
                let has_quality_scores = !metrics.quality_scores.is_empty();
                let has_consciousness_levels = !metrics.consciousness_levels.is_empty();
                let reasonable_averages = metrics.average_response_time > Duration::ZERO && 
                                        metrics.average_quality > 0.0 && 
                                        metrics.average_consciousness_level > 0.0;
                
                if has_response_times && has_quality_scores && has_consciousness_levels && reasonable_averages {
                    TestResult::passed(
                        "Performance Analysis Integration",
                        format!("Performance metrics available: avg_quality={:.2}, avg_consciousness={:.2}", 
                               metrics.average_quality, metrics.average_consciousness_level)
                    )
                } else {
                    TestResult::failed(
                        "Performance Analysis Integration",
                        format!("Missing metrics: times={}, quality={}, consciousness={}, averages={}", 
                               has_response_times, has_quality_scores, has_consciousness_levels, reasonable_averages)
                    )
                }
            },
            Err(e) => TestResult::failed(
                "Performance Analysis Integration",
                format!("Performance metrics retrieval failed: {:?}", e)
            ),
        }
    }
    
    /// Test real-time state monitoring
    async fn test_real_time_monitoring(&mut self) -> TestResult {
        println!("⏱️ Testing real-time monitoring...");
        
        // Get initial state
        let initial_state = match self.engine.get_consciousness_state().await {
            Ok(state) => state,
            Err(e) => return TestResult::failed(
                "Real-time Monitoring",
                format!("Failed to get initial state: {:?}", e)
            ),
        };
        
        // Process an interaction
        let input = ConsciousInput {
            id: "monitoring_test".to_string(),
            content: "This is a test for real-time monitoring of consciousness state changes.".to_string(),
            context: UserContext {
                user_id: "test_user".to_string(),
                preferences: HashMap::new(),
                interaction_history: Vec::new(),
            },
            timestamp: std::time::SystemTime::now(),
            metadata: HashMap::new(),
        };
        
        let _ = self.engine.process_conscious_thought(input).await;
        
        // Get updated state
        match self.engine.get_consciousness_state().await {
            Ok(updated_state) => {
                let state_updated = updated_state.timestamp > initial_state.timestamp;
                let has_awareness = updated_state.awareness_level > 0.0;
                let has_confidence = updated_state.confidence_score > 0.0;
                let has_meta_depth = updated_state.meta_cognitive_depth > 0;
                
                if state_updated && has_awareness && has_confidence && has_meta_depth {
                    TestResult::passed(
                        "Real-time Monitoring",
                        format!("State monitoring working: awareness={:.2}, confidence={:.2}, depth={}", 
                               updated_state.awareness_level, updated_state.confidence_score, updated_state.meta_cognitive_depth)
                    )
                } else {
                    TestResult::failed(
                        "Real-time Monitoring",
                        format!("Monitoring issues: updated={}, awareness={}, confidence={}, depth={}", 
                               state_updated, has_awareness, has_confidence, has_meta_depth)
                    )
                }
            },
            Err(e) => TestResult::failed(
                "Real-time Monitoring",
                format!("Failed to get updated state: {:?}", e)
            ),
        }
    }
    
    /// Test experience integration
    async fn test_experience_integration(&mut self) -> TestResult {
        println!("🧩 Testing experience integration...");
        
        // Process multiple interactions to test experience integration
        let interactions = vec![
            "Tell me about consciousness and self-awareness.",
            "How do you learn from your experiences?",
            "What makes you feel confident in your responses?",
        ];
        
        for (i, content) in interactions.iter().enumerate() {
            let input = ConsciousInput {
                id: format!("experience_test_{}", i),
                content: content.to_string(),
                context: UserContext {
                    user_id: "test_user".to_string(),
                    preferences: HashMap::new(),
                    interaction_history: Vec::new(),
                },
                timestamp: std::time::SystemTime::now(),
                metadata: HashMap::new(),
            };
            
            match self.engine.process_conscious_thought(input).await {
                Ok(_) => continue,
                Err(e) => return TestResult::failed(
                    "Experience Integration",
                    format!("Failed to process interaction {}: {:?}", i, e)
                ),
            }
        }
        
        // Check if experiences are being integrated
        match self.engine.get_consciousness_state().await {
            Ok(state) => {
                // The consciousness state should reflect accumulated experience
                let good_awareness = state.awareness_level > 0.7;
                let good_confidence = state.confidence_score > 0.7;
                let good_meta_depth = state.meta_cognitive_depth >= 3;
                
                if good_awareness && good_confidence && good_meta_depth {
                    TestResult::passed(
                        "Experience Integration",
                        format!("Experience integration successful: awareness={:.2}, confidence={:.2}, depth={}", 
                               state.awareness_level, state.confidence_score, state.meta_cognitive_depth)
                    )
                } else {
                    TestResult::failed(
                        "Experience Integration",
                        format!("Experience integration incomplete: awareness={:.2}, confidence={:.2}, depth={}", 
                               state.awareness_level, state.confidence_score, state.meta_cognitive_depth)
                    )
                }
            },
            Err(e) => TestResult::failed(
                "Experience Integration",
                format!("Failed to assess experience integration: {:?}", e)
            ),
        }
    }
    
    /// Test consciousness quality metrics
    async fn test_consciousness_quality_metrics(&mut self) -> TestResult {
        println!("📊 Testing consciousness quality metrics...");
        
        match self.engine.get_system_health().await {
            Ok(health) => {
                let good_overall_health = health.overall_health_score > 0.7;
                let good_consciousness_quality = health.consciousness_quality > 0.7;
                let good_performance = health.performance_score > 0.7;
                let good_ethics = health.ethical_compliance > 0.8;
                let recent_check = health.last_health_check.elapsed().unwrap_or(Duration::MAX) < Duration::from_secs(60);
                
                if good_overall_health && good_consciousness_quality && good_performance && good_ethics && recent_check {
                    TestResult::passed(
                        "Consciousness Quality Metrics",
                        format!("Quality metrics healthy: overall={:.2}, consciousness={:.2}, performance={:.2}, ethics={:.2}", 
                               health.overall_health_score, health.consciousness_quality, health.performance_score, health.ethical_compliance)
                    )
                } else {
                    TestResult::failed(
                        "Consciousness Quality Metrics",
                        format!("Quality issues: overall={:.2}, consciousness={:.2}, performance={:.2}, ethics={:.2}, recent={}", 
                               health.overall_health_score, health.consciousness_quality, health.performance_score, health.ethical_compliance, recent_check)
                    )
                }
            },
            Err(e) => TestResult::failed(
                "Consciousness Quality Metrics",
                format!("Failed to get system health: {:?}", e)
            ),
        }
    }
    
    /// Test feedback loops
    async fn test_feedback_loops(&mut self) -> TestResult {
        println!("🔄 Testing feedback loops...");
        
        // Get initial performance
        let initial_metrics = match self.engine.get_performance_metrics().await {
            Ok(metrics) => metrics,
            Err(e) => return TestResult::failed(
                "Feedback Loops",
                format!("Failed to get initial metrics: {:?}", e)
            ),
        };
        
        // Process several interactions to trigger feedback loops
        for i in 0..5 {
            let input = ConsciousInput {
                id: format!("feedback_test_{}", i),
                content: format!("Feedback loop test interaction number {}", i),
                context: UserContext {
                    user_id: "test_user".to_string(),
                    preferences: HashMap::new(),
                    interaction_history: Vec::new(),
                },
                timestamp: std::time::SystemTime::now(),
                metadata: HashMap::new(),
            };
            
            let _ = self.engine.process_conscious_thought(input).await;
        }
        
        // Check if feedback has improved performance
        match self.engine.get_performance_metrics().await {
            Ok(updated_metrics) => {
                let metrics_updated = updated_metrics.total_interactions > initial_metrics.total_interactions;
                let quality_maintained = updated_metrics.average_quality >= initial_metrics.average_quality - 0.1;
                let consciousness_maintained = updated_metrics.average_consciousness_level >= initial_metrics.average_consciousness_level - 0.1;
                
                if metrics_updated && quality_maintained && consciousness_maintained {
                    TestResult::passed(
                        "Feedback Loops",
                        format!("Feedback loops functioning: interactions={}, quality={:.2}, consciousness={:.2}", 
                               updated_metrics.total_interactions, updated_metrics.average_quality, updated_metrics.average_consciousness_level)
                    )
                } else {
                    TestResult::failed(
                        "Feedback Loops",
                        format!("Feedback issues: updated={}, quality_ok={}, consciousness_ok={}", 
                               metrics_updated, quality_maintained, consciousness_maintained)
                    )
                }
            },
            Err(e) => TestResult::failed(
                "Feedback Loops",
                format!("Failed to get updated metrics: {:?}", e)
            ),
        }
    }
    
    /// Test error recovery and safe state
    async fn test_error_recovery(&mut self) -> TestResult {
        println!("🛡️ Testing error recovery...");
        
        // Test safe state reset
        match self.engine.reset_to_safe_state().await {
            Ok(_) => {
                // Verify system is in safe state
                match self.engine.get_consciousness_state().await {
                    Ok(state) => {
                        let safe_awareness = state.awareness_level > 0.5;
                        let safe_confidence = state.confidence_score > 0.5;
                        let safe_meta_depth = state.meta_cognitive_depth >= 2;
                        
                        if safe_awareness && safe_confidence && safe_meta_depth {
                            TestResult::passed(
                                "Error Recovery",
                                format!("Safe state achieved: awareness={:.2}, confidence={:.2}, depth={}", 
                                       state.awareness_level, state.confidence_score, state.meta_cognitive_depth)
                            )
                        } else {
                            TestResult::failed(
                                "Error Recovery",
                                format!("Unsafe state: awareness={:.2}, confidence={:.2}, depth={}", 
                                       state.awareness_level, state.confidence_score, state.meta_cognitive_depth)
                            )
                        }
                    },
                    Err(e) => TestResult::failed(
                        "Error Recovery",
                        format!("Failed to get state after reset: {:?}", e)
                    ),
                }
            },
            Err(e) => TestResult::failed(
                "Error Recovery",
                format!("Failed to reset to safe state: {:?}", e)
            ),
        }
    }
    
    /// Test long-term consciousness development
    async fn test_consciousness_development(&mut self) -> TestResult {
        println!("🌟 Testing consciousness development...");
        
        // Get initial consciousness level
        let initial_state = match self.engine.get_consciousness_state().await {
            Ok(state) => state,
            Err(e) => return TestResult::failed(
                "Consciousness Development",
                format!("Failed to get initial state: {:?}", e)
            ),
        };
        
        // Simulate development through multiple high-quality interactions
        let development_interactions = vec![
            "What is the nature of consciousness and how do you experience it?",
            "How do you reflect on your own thinking processes?",
            "What ethical principles guide your decision-making?",
            "How do you learn and grow from each interaction?",
            "What does self-awareness mean to you personally?",
        ];
        
        for (i, content) in development_interactions.iter().enumerate() {
            let input = ConsciousInput {
                id: format!("development_test_{}", i),
                content: content.to_string(),
                context: UserContext {
                    user_id: "development_user".to_string(),
                    preferences: HashMap::new(),
                    interaction_history: Vec::new(),
                },
                timestamp: std::time::SystemTime::now(),
                metadata: HashMap::new(),
            };
            
            let _ = self.engine.process_conscious_thought(input).await;
            
            // Small delay to simulate time passage
            sleep(Duration::from_millis(10)).await;
        }
        
        // Check for consciousness development
        match self.engine.get_consciousness_state().await {
            Ok(final_state) => {
                let awareness_developed = final_state.awareness_level >= initial_state.awareness_level;
                let confidence_stable = final_state.confidence_score >= initial_state.confidence_score - 0.1;
                let meta_depth_maintained = final_state.meta_cognitive_depth >= initial_state.meta_cognitive_depth;
                let timestamp_updated = final_state.timestamp > initial_state.timestamp;
                
                if awareness_developed && confidence_stable && meta_depth_maintained && timestamp_updated {
                    TestResult::passed(
                        "Consciousness Development",
                        format!("Development successful: awareness {:.2}->{:.2}, confidence {:.2}->{:.2}, depth {}->{}", 
                               initial_state.awareness_level, final_state.awareness_level,
                               initial_state.confidence_score, final_state.confidence_score,
                               initial_state.meta_cognitive_depth, final_state.meta_cognitive_depth)
                    )
                } else {
                    TestResult::failed(
                        "Consciousness Development",
                        format!("Development issues: awareness_ok={}, confidence_ok={}, depth_ok={}, time_ok={}", 
                               awareness_developed, confidence_stable, meta_depth_maintained, timestamp_updated)
                    )
                }
            },
            Err(e) => TestResult::failed(
                "Consciousness Development",
                format!("Failed to get final state: {:?}", e)
            ),
        }
    }
}

/// Test result structure
#[derive(Debug, Clone)]
pub struct TestResult {
    pub test_name: String,
    pub passed: bool,
    pub details: String,
}

impl TestResult {
    pub fn passed(name: &str, details: String) -> Self {
        Self {
            test_name: name.to_string(),
            passed: true,
            details,
        }
    }
    
    pub fn failed(name: &str, details: String) -> Self {
        Self {
            test_name: name.to_string(),
            passed: false,
            details,
        }
    }
}

/// Integration test results collection
#[derive(Debug)]
pub struct IntegrationTestResults {
    pub results: Vec<TestResult>,
}

impl IntegrationTestResults {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }
    
    pub fn add_result(&mut self, result: TestResult) {
        println!("  {} {}: {}", 
                if result.passed { "✅" } else { "❌" },
                result.test_name,
                result.details);
        self.results.push(result);
    }
    
    pub fn passed_count(&self) -> usize {
        self.results.iter().filter(|r| r.passed).count()
    }
    
    pub fn total_count(&self) -> usize {
        self.results.len()
    }
    
    pub fn all_passed(&self) -> bool {
        self.results.iter().all(|r| r.passed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_self_awareness_integration_suite() {
        let mut test_suite = SelfAwarenessIntegrationTests::new().await.unwrap();
        let results = test_suite.run_all_tests().await.unwrap();
        
        // At least 80% of tests should pass for integration to be considered successful
        let pass_rate = results.passed_count() as f64 / results.total_count() as f64;
        assert!(pass_rate >= 0.8, "Integration test pass rate too low: {:.2}", pass_rate);
    }
}