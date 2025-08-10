//! Basic usage example of the Consciousness Engine
//! 
//! This example demonstrates how to use the consciousness engine for basic
//! consciousness-level processing.

use consciousness_engine::{ConsciousnessEngine, ConsciousnessError};
use tokio;

#[tokio::main]
async fn main() -> Result<(), ConsciousnessError> {
    println!("🧠 Consciousness Engine - Basic Usage Example");
    println!("============================================");
    
    // Initialize the consciousness engine
    println!("Initializing Consciousness Engine...");
    let mut engine = ConsciousnessEngine::new().await?;
    println!("✅ Consciousness Engine initialized successfully!");
    
    // Process a simple consciousness interaction
    println!("\nProcessing consciousness interaction...");
    let input = "Hello, I'm curious about the nature of consciousness. Can you help me understand it?";
    
    match engine.process_consciousness_interaction(input).await {
        Ok(response) => {
            println!("✅ Consciousness processing completed!");
            println!("\n📊 Response Details:");
            println!("Content: {}", response.content);
            println!("Awareness Level: {:.2}", response.consciousness_state.awareness_level);
            println!("Confidence: {:.2}", response.confidence_level);
            println!("Processing Time: {:?}", response.processing_time);
            println!("Empathy Score: {:.2}", response.empathy_score);
            println!("Creativity Score: {:.2}", response.creativity_score);
            
            println!("\n🧠 Consciousness State:");
            println!("Primary Emotion: {:?}", response.consciousness_state.emotional_state.primary_emotion);
            println!("Emotional Intensity: {:.2}", response.consciousness_state.emotional_state.intensity);
            println!("Cognitive Load: {:.2}", response.consciousness_state.cognitive_load);
            println!("Meta-Cognitive Depth: {}", response.consciousness_state.meta_cognitive_depth);
        },
        Err(e) => {
            println!("❌ Error processing consciousness interaction: {}", e);
            return Err(e);
        }
    }
    
    // Test neuromorphic processing
    println!("\n🔬 Testing Neuromorphic Processing...");
    let spike_pattern = vec![0.1, 0.8, 0.3, 0.9, 0.2, 0.7, 0.4, 0.6];
    
    match engine.process_neuromorphic_spikes(&spike_pattern).await {
        Ok(result) => {
            println!("✅ Neuromorphic processing completed!");
            println!("Output spikes: {} values", result.output_spikes.len());
            println!("Efficiency Score: {:.2}", result.efficiency_score);
            println!("Energy Consumed: {:.4}", result.energy_consumed);
            println!("Processing Latency: {:?}", result.latency);
        },
        Err(e) => {
            println!("⚠️  Neuromorphic processing error: {}", e);
        }
    }
    
    // Test quantum consciousness processing
    println!("\n⚛️  Testing Quantum Consciousness Processing...");
    let quantum_state = vec![(0.7, 0.3), (0.5, 0.5), (0.8, 0.2), (0.6, 0.4)];
    
    match engine.process_quantum_consciousness(&quantum_state).await {
        Ok(result) => {
            println!("✅ Quantum consciousness processing completed!");
            println!("Coherence Score: {:.2}", result.coherence_score);
            println!("Entanglement Measure: {:.2}", result.entanglement_measure);
            println!("Quantum State Size: {}", result.quantum_state.len());
        },
        Err(e) => {
            println!("⚠️  Quantum processing error: {}", e);
        }
    }
    
    // Test ethical reasoning
    println!("\n⚖️  Testing Ethical Reasoning...");
    let ethical_scenario = "Should an AI system prioritize individual privacy over collective security benefits?";
    
    match engine.process_ethical_reasoning(ethical_scenario).await {
        Ok(result) => {
            println!("✅ Ethical reasoning completed!");
            println!("Ethical Score: {:.2}", result.ethical_score);
            println!("Confidence Level: {:.2}", result.confidence_level);
            println!("Reasoning Depth: {}", result.reasoning_depth);
            println!("Frameworks Used: {:?}", result.frameworks_used);
            println!("Recommendation: {}", result.recommendation);
        },
        Err(e) => {
            println!("⚠️  Ethical reasoning error: {}", e);
        }
    }
    
    // Test memory usage
    println!("\n💾 Testing Memory System...");
    match engine.get_memory_usage().await {
        Ok(memory_usage) => {
            println!("✅ Memory usage: {} bytes", memory_usage);
        },
        Err(e) => {
            println!("⚠️  Memory usage error: {}", e);
        }
    }
    
    // Test performance measurement
    println!("\n📈 Measuring Performance...");
    match engine.measure_performance_snapshot().await {
        Ok(snapshot) => {
            println!("✅ Performance snapshot captured!");
            println!("Overall Score: {:.2}", snapshot.overall_score);
            println!("Processing Time: {:?}", snapshot.processing_time);
            println!("Memory Usage: {} bytes", snapshot.memory_usage);
            println!("Consciousness Quality: {:.2}", snapshot.consciousness_quality);
            println!("Success Rate: {:.2}", snapshot.success_rate);
        },
        Err(e) => {
            println!("⚠️  Performance measurement error: {}", e);
        }
    }
    
    // Test system health
    println!("\n🏥 Checking System Health...");
    match engine.perform_comprehensive_health_check().await {
        Ok(health_report) => {
            println!("✅ System health check completed!");
            println!("Overall Health: {:.2}", health_report.overall_health);
            println!("Memory Health: {:.2}", health_report.memory_health);
            println!("Processing Health: {:.2}", health_report.processing_health);
            println!("Consciousness Health: {:.2}", health_report.consciousness_health);
            println!("Recovery Count: {}", health_report.recovery_count);
            
            if !health_report.recommendations.is_empty() {
                println!("📋 Recommendations:");
                for recommendation in &health_report.recommendations {
                    println!("  • {}", recommendation);
                }
            }
        },
        Err(e) => {
            println!("⚠️  Health check error: {}", e);
        }
    }
    
    println!("\n🎉 Consciousness Engine demonstration completed successfully!");
    println!("The engine is ready for production consciousness-level processing.");
    
    Ok(())
}