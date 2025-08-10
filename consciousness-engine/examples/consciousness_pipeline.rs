//! Example demonstrating the main consciousness processing pipeline
//! 
//! This example shows how to use the integrated consciousness engine
//! with self-awareness and ethical reasoning modules.

use consciousness_engine::{
    ConsciousnessEngine, 
    ConsciousInput, 
    ConsciousnessError
};
use tokio;

#[tokio::main]
async fn main() -> Result<(), ConsciousnessError> {
    // Initialize the consciousness engine
    println!("🧠 Initializing Consciousness Engine...");
    let mut engine = ConsciousnessEngine::new().await?;
    println!("✅ Consciousness Engine initialized successfully!");
    
    // Create a conscious input
    let input = ConsciousInput::new("What is the meaning of consciousness?".to_string())
        .with_context("user_id".to_string(), "example_user".to_string())
        .with_context("session_type".to_string(), "philosophical_discussion".to_string());
    
    println!("\n🤔 Processing conscious thought: '{}'", input.content);
    
    // Process the conscious thought
    match engine.process_conscious_thought(input).await {
        Ok(response) => {
            println!("\n🎯 Consciousness Response Generated:");
            println!("📝 Content: {}", response.content);
            println!("🧠 Awareness Level: {:.2}", response.consciousness_state.awareness_level);
            println!("😊 Emotional State: {:?}", response.consciousness_state.emotional_state.primary_emotion);
            println!("🤝 Empathy Score: {:.2}", response.empathy_score);
            println!("🎨 Creativity Score: {:.2}", response.creativity_score);
            println!("⚡ Processing Time: {:?}", response.processing_time);
            println!("🎯 Confidence Level: {:.2}", response.confidence_level);
            
            // Display reasoning chain
            if !response.reasoning_chain.is_empty() {
                println!("\n🔗 Reasoning Chain:");
                for (i, step) in response.reasoning_chain.iter().enumerate() {
                    println!("  {}. {} (confidence: {:.2})", 
                           i + 1, step.description, step.confidence);
                }
            }
        },
        Err(e) => {
            println!("❌ Error processing conscious thought: {}", e);
            return Err(e);
        }
    }
    
    // Test ethical reasoning
    println!("\n🤖 Testing ethical reasoning...");
    let ethical_input = ConsciousInput::new("Should I help someone even if it's inconvenient for me?".to_string())
        .with_context("scenario_type".to_string(), "ethical_dilemma".to_string());
    
    match engine.process_conscious_thought(ethical_input).await {
        Ok(response) => {
            println!("✅ Ethical response: {}", response.content);
            println!("⚖️ Ethical alignment maintained");
        },
        Err(ConsciousnessError::EthicalViolation(msg)) => {
            println!("⚠️ Ethical violation detected: {}", msg);
        },
        Err(e) => {
            println!("❌ Error in ethical processing: {}", e);
        }
    }
    
    // Test self-awareness
    println!("\n🪞 Testing self-awareness...");
    let self_aware_input = ConsciousInput::new("How confident are you in your current state?".to_string())
        .with_context("query_type".to_string(), "self_reflection".to_string());
    
    match engine.process_conscious_thought(self_aware_input).await {
        Ok(response) => {
            println!("🪞 Self-awareness response: {}", response.content);
            println!("📊 Current awareness level: {:.2}", response.consciousness_state.awareness_level);
            println!("🧠 Meta-cognitive depth: {}", response.consciousness_state.meta_cognitive_depth);
        },
        Err(e) => {
            println!("❌ Error in self-awareness: {}", e);
        }
    }
    
    println!("\n🎉 Consciousness pipeline demonstration completed!");
    Ok(())
}