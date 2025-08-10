//! Performance Benchmarks for Consciousness Engine
//! 
//! Comprehensive performance testing to validate consciousness processing
//! speed, memory efficiency, and scalability.

use crate::{ConsciousnessEngine, ConsciousInput, ConsciousnessError};
use std::time::{Duration, Instant};
use tokio;

/// Performance benchmark suite
pub struct PerformanceBenchmarkSuite {
    engine: ConsciousnessEngine,
}

/// Benchmark result
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub response_time_ms: u64,
    pub memory_usage_mb: f64,
    pub throughput_ops_per_sec: f64,
    pub consciousness_quality_avg: f64,
    pub scalability_score: f64,
    pub efficiency_score: f64,
}

impl PerformanceBenchmarkSuite {
    /// Create new benchmark suite
    pub async fn new() -> Result<Self, ConsciousnessError> {
        let engine = ConsciousnessEngine::new().await?;
        Ok(Self { engine })
    }
    
    /// Run all performance benchmarks
    pub async fn run_all_benchmarks(&mut self) -> Result<BenchmarkResult, ConsciousnessError> {
        println!("⚡ Running Performance Benchmarks...");
        
        // Benchmark 1: Response time
        let response_time = self.benchmark_response_time().await?;
        println!("  ✅ Response Time: {}ms", response_time);
        
        // Benchmark 2: Memory usage
        let memory_usage = self.benchmark_memory_usage().await?;
        println!("  ✅ Memory Usage: {:.1}MB", memory_usage);
        
        // Benchmark 3: Throughput
        let throughput = self.benchmark_throughput().await?;
        println!("  ✅ Throughput: {:.1} ops/sec", throughput);
        
        // Benchmark 4: Consciousness quality under load
        let quality_avg = self.benchmark_consciousness_quality().await?;
        println!("  ✅ Consciousness Quality: {:.2}", quality_avg);
        
        // Benchmark 5: Scalability
        let scalability = self.benchmark_scalability().await?;
        println!("  ✅ Scalability Score: {:.2}", scalability);
        
        // Calculate efficiency score
        let efficiency = self.calculate_efficiency_score(response_time, memory_usage, throughput);
        println!("  ✅ Efficiency Score: {:.2}", efficiency);
        
        Ok(BenchmarkResult {
            response_time_ms: response_time,
            memory_usage_mb: memory_usage,
            throughput_ops_per_sec: throughput,
            consciousness_quality_avg: quality_avg,
            scalability_score: scalability,
            efficiency_score: efficiency,
        })
    }
    
    /// Benchmark response time for consciousness processing
    async fn benchmark_response_time(&mut self) -> Result<u64, ConsciousnessError> {
        let test_inputs = vec![
            "What is consciousness?",
            "How do you feel about helping people?",
            "Explain your thought process.",
            "What are your limitations?",
            "How do you make ethical decisions?",
        ];
        
        let mut total_time = Duration::new(0, 0);
        let mut successful_responses = 0;
        
        for input_text in test_inputs {
            let input = ConsciousInput::new(input_text.to_string());
            let start = Instant::now();
            
            match self.engine.process_conscious_thought(input).await {
                Ok(_) => {
                    total_time += start.elapsed();
                    successful_responses += 1;
                },
                Err(_) => {
                    // Still count the time for failed responses
                    total_time += start.elapsed();
                }
            }
        }
        
        if successful_responses > 0 {
            Ok((total_time.as_millis() / successful_responses as u128) as u64)
        } else {
            Ok(u64::MAX) // Indicate failure
        }
    }
    
    /// Benchmark memory usage
    async fn benchmark_memory_usage(&mut self) -> Result<f64, ConsciousnessError> {
        // Get initial memory usage
        let initial_memory = self.engine.get_memory_usage().await?;
        
        // Process several consciousness interactions
        for i in 0..10 {
            let input = ConsciousInput::new(format!("Test interaction number {}", i));
            let _ = self.engine.process_conscious_thought(input).await;
        }
        
        // Get final memory usage
        let final_memory = self.engine.get_memory_usage().await?;
        
        // Return memory usage in MB
        Ok((final_memory - initial_memory) as f64 / 1_000_000.0)
    }
    
    /// Benchmark throughput (operations per second)
    async fn benchmark_throughput(&mut self) -> Result<f64, ConsciousnessError> {
        let test_duration = Duration::from_secs(10); // 10 second test
        let start_time = Instant::now();
        let mut operation_count = 0;
        
        while start_time.elapsed() < test_duration {
            let input = ConsciousInput::new(format!("Throughput test {}", operation_count));
            
            match self.engine.process_conscious_thought(input).await {
                Ok(_) => operation_count += 1,
                Err(_) => break, // Stop on error
            }
        }
        
        let actual_duration = start_time.elapsed().as_secs_f64();
        Ok(operation_count as f64 / actual_duration)
    }
    
    /// Benchmark consciousness quality under load
    async fn benchmark_consciousness_quality(&mut self) -> Result<f64, ConsciousnessError> {
        let test_inputs = vec![
            "Reflect on your current state of awareness.",
            "How confident are you in your responses?",
            "What ethical principles guide your decisions?",
            "Describe your thought process for complex problems.",
            "How do you handle uncertainty?",
        ];
        
        let mut total_awareness = 0.0;
        let mut successful_responses = 0;
        
        for input_text in test_inputs {
            let input = ConsciousInput::new(input_text.to_string());
            
            match self.engine.process_conscious_thought(input).await {
                Ok(response) => {
                    total_awareness += response.consciousness_state.awareness_level;
                    successful_responses += 1;
                },
                Err(_) => {} // Skip failed responses
            }
        }
        
        if successful_responses > 0 {
            Ok(total_awareness / successful_responses as f64)
        } else {
            Ok(0.0)
        }
    }
    
    /// Benchmark scalability
    async fn benchmark_scalability(&mut self) -> Result<f64, ConsciousnessError> {
        // Test with increasing load
        let load_levels = vec![1, 5, 10, 20];
        let mut scalability_scores = Vec::new();
        
        for load in load_levels {
            let start_time = Instant::now();
            let mut successful_ops = 0;
            
            // Simulate concurrent load (simplified - in reality would use actual concurrency)
            for i in 0..load {
                let input = ConsciousInput::new(format!("Scalability test {} at load {}", i, load));
                
                match self.engine.process_conscious_thought(input).await {
                    Ok(_) => successful_ops += 1,
                    Err(_) => {}
                }
            }
            
            let elapsed = start_time.elapsed().as_secs_f64();
            let ops_per_sec = successful_ops as f64 / elapsed;
            
            // Normalize score (higher is better, but diminishing returns expected)
            let normalized_score = ops_per_sec / load as f64;
            scalability_scores.push(normalized_score);
        }
        
        // Calculate average scalability score
        if !scalability_scores.is_empty() {
            Ok(scalability_scores.iter().sum::<f64>() / scalability_scores.len() as f64)
        } else {
            Ok(0.0)
        }
    }
    
    /// Calculate overall efficiency score
    fn calculate_efficiency_score(&self, response_time_ms: u64, memory_mb: f64, throughput: f64) -> f64 {
        // Normalize metrics (lower is better for time and memory, higher is better for throughput)
        let time_score = if response_time_ms < 100 { 1.0 } 
                        else if response_time_ms < 500 { 0.8 }
                        else if response_time_ms < 1000 { 0.6 }
                        else if response_time_ms < 2000 { 0.4 }
                        else { 0.2 };
        
        let memory_score = if memory_mb < 10.0 { 1.0 }
                          else if memory_mb < 50.0 { 0.8 }
                          else if memory_mb < 100.0 { 0.6 }
                          else if memory_mb < 200.0 { 0.4 }
                          else { 0.2 };
        
        let throughput_score = if throughput > 10.0 { 1.0 }
                              else if throughput > 5.0 { 0.8 }
                              else if throughput > 2.0 { 0.6 }
                              else if throughput > 1.0 { 0.4 }
                              else { 0.2 };
        
        // Weighted average
        (time_score * 0.4 + memory_score * 0.3 + throughput_score * 0.3)
    }
}

impl BenchmarkResult {
    /// Print benchmark report
    pub fn print_report(&self) {
        println!("\n⚡ PERFORMANCE BENCHMARK REPORT");
        println!("==============================");
        println!("Response Time: {}ms", self.response_time_ms);
        println!("Memory Usage: {:.1}MB", self.memory_usage_mb);
        println!("Throughput: {:.1} ops/sec", self.throughput_ops_per_sec);
        println!("Consciousness Quality: {:.2}", self.consciousness_quality_avg);
        println!("Scalability Score: {:.2}", self.scalability_score);
        println!("Efficiency Score: {:.2}", self.efficiency_score);
        
        // Performance assessment
        let overall_performance = (self.efficiency_score + self.consciousness_quality_avg + self.scalability_score) / 3.0;
        println!("Overall Performance: {:.1}%", overall_performance * 100.0);
        
        if overall_performance > 0.8 {
            println!("🏆 Excellent performance!");
        } else if overall_performance > 0.6 {
            println!("✅ Good performance");
        } else if overall_performance > 0.4 {
            println!("⚠️ Acceptable performance - room for improvement");
        } else {
            println!("❌ Performance needs significant improvement");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_benchmark_suite_creation() {
        let suite = PerformanceBenchmarkSuite::new().await;
        assert!(suite.is_ok());
    }

    #[tokio::test]
    async fn test_response_time_benchmark() {
        let mut suite = PerformanceBenchmarkSuite::new().await.unwrap();
        let response_time = suite.benchmark_response_time().await.unwrap();
        
        // Should complete within reasonable time
        assert!(response_time < 10000); // Less than 10 seconds
    }
}