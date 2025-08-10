//! Utility functions for the Consciousness Engine
//! 
//! This module contains helper functions and utilities used throughout the consciousness engine.

use crate::error::ConsciousnessError;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

/// Generate a unique ID for consciousness sessions
pub fn generate_session_id() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_millis();
    
    format!("consciousness_{}", timestamp)
}

/// Generate a unique ID with prefix
pub fn generate_id_with_prefix(prefix: &str) -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_millis();
    
    format!("{}_{}", prefix, timestamp)
}

/// Calculate similarity between two vectors
pub fn calculate_cosine_similarity(vec1: &[f64], vec2: &[f64]) -> Result<f64, ConsciousnessError> {
    if vec1.len() != vec2.len() {
        return Err(ConsciousnessError::InvalidInput(
            "Vectors must have the same length".to_string()
        ));
    }
    
    if vec1.is_empty() {
        return Ok(0.0);
    }
    
    let dot_product: f64 = vec1.iter().zip(vec2.iter()).map(|(a, b)| a * b).sum();
    let norm1: f64 = vec1.iter().map(|x| x * x).sum::<f64>().sqrt();
    let norm2: f64 = vec2.iter().map(|x| x * x).sum::<f64>().sqrt();
    
    if norm1 == 0.0 || norm2 == 0.0 {
        return Ok(0.0);
    }
    
    Ok(dot_product / (norm1 * norm2))
}

/// Calculate Euclidean distance between two vectors
pub fn calculate_euclidean_distance(vec1: &[f64], vec2: &[f64]) -> Result<f64, ConsciousnessError> {
    if vec1.len() != vec2.len() {
        return Err(ConsciousnessError::InvalidInput(
            "Vectors must have the same length".to_string()
        ));
    }
    
    let distance: f64 = vec1.iter()
        .zip(vec2.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f64>()
        .sqrt();
    
    Ok(distance)
}

/// Normalize a vector to unit length
pub fn normalize_vector(vec: &mut [f64]) -> Result<(), ConsciousnessError> {
    let norm: f64 = vec.iter().map(|x| x * x).sum::<f64>().sqrt();
    
    if norm == 0.0 {
        return Err(ConsciousnessError::ProcessingError(
            "Cannot normalize zero vector".to_string()
        ));
    }
    
    for value in vec.iter_mut() {
        *value /= norm;
    }
    
    Ok(())
}

/// Calculate entropy of a probability distribution
pub fn calculate_entropy(probabilities: &[f64]) -> Result<f64, ConsciousnessError> {
    let sum: f64 = probabilities.iter().sum();
    
    if (sum - 1.0).abs() > 1e-6 {
        return Err(ConsciousnessError::InvalidInput(
            "Probabilities must sum to 1.0".to_string()
        ));
    }
    
    let entropy = -probabilities.iter()
        .filter(|&&p| p > 0.0)
        .map(|&p| p * p.ln())
        .sum::<f64>();
    
    Ok(entropy)
}

/// Calculate mutual information between two probability distributions
pub fn calculate_mutual_information(joint_prob: &[Vec<f64>]) -> Result<f64, ConsciousnessError> {
    if joint_prob.is_empty() || joint_prob[0].is_empty() {
        return Ok(0.0);
    }
    
    let rows = joint_prob.len();
    let cols = joint_prob[0].len();
    
    // Calculate marginal probabilities
    let mut marginal_x = vec![0.0; rows];
    let mut marginal_y = vec![0.0; cols];
    
    for i in 0..rows {
        for j in 0..cols {
            marginal_x[i] += joint_prob[i][j];
            marginal_y[j] += joint_prob[i][j];
        }
    }
    
    // Calculate mutual information
    let mut mutual_info = 0.0;
    for i in 0..rows {
        for j in 0..cols {
            let joint = joint_prob[i][j];
            if joint > 0.0 && marginal_x[i] > 0.0 && marginal_y[j] > 0.0 {
                mutual_info += joint * (joint / (marginal_x[i] * marginal_y[j])).ln();
            }
        }
    }
    
    Ok(mutual_info)
}

/// Exponential moving average
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExponentialMovingAverage {
    alpha: f64,
    current_value: Option<f64>,
}

impl ExponentialMovingAverage {
    pub fn new(alpha: f64) -> Result<Self, ConsciousnessError> {
        if !(0.0..=1.0).contains(&alpha) {
            return Err(ConsciousnessError::InvalidInput(
                "Alpha must be between 0.0 and 1.0".to_string()
            ));
        }
        
        Ok(Self {
            alpha,
            current_value: None,
        })
    }
    
    pub fn update(&mut self, new_value: f64) -> f64 {
        match self.current_value {
            None => {
                self.current_value = Some(new_value);
                new_value
            },
            Some(current) => {
                let updated = self.alpha * new_value + (1.0 - self.alpha) * current;
                self.current_value = Some(updated);
                updated
            }
        }
    }
    
    pub fn get_value(&self) -> Option<f64> {
        self.current_value
    }
    
    pub fn reset(&mut self) {
        self.current_value = None;
    }
}

/// Sliding window statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlidingWindow {
    values: std::collections::VecDeque<f64>,
    max_size: usize,
}

impl SlidingWindow {
    pub fn new(max_size: usize) -> Result<Self, ConsciousnessError> {
        if max_size == 0 {
            return Err(ConsciousnessError::InvalidInput(
                "Window size must be greater than 0".to_string()
            ));
        }
        
        Ok(Self {
            values: std::collections::VecDeque::new(),
            max_size,
        })
    }
    
    pub fn add_value(&mut self, value: f64) {
        if self.values.len() >= self.max_size {
            self.values.pop_front();
        }
        self.values.push_back(value);
    }
    
    pub fn mean(&self) -> Option<f64> {
        if self.values.is_empty() {
            None
        } else {
            Some(self.values.iter().sum::<f64>() / self.values.len() as f64)
        }
    }
    
    pub fn variance(&self) -> Option<f64> {
        if self.values.len() < 2 {
            None
        } else {
            let mean = self.mean()?;
            let variance = self.values.iter()
                .map(|x| (x - mean).powi(2))
                .sum::<f64>() / (self.values.len() - 1) as f64;
            Some(variance)
        }
    }
    
    pub fn std_dev(&self) -> Option<f64> {
        self.variance().map(|v| v.sqrt())
    }
    
    pub fn min(&self) -> Option<f64> {
        self.values.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).copied()
    }
    
    pub fn max(&self) -> Option<f64> {
        self.values.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).copied()
    }
    
    pub fn len(&self) -> usize {
        self.values.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    
    pub fn clear(&mut self) {
        self.values.clear();
    }
}

/// Rate limiter for consciousness operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimiter {
    max_requests: u32,
    window_duration: Duration,
    requests: std::collections::VecDeque<SystemTime>,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window_duration: Duration) -> Self {
        Self {
            max_requests,
            window_duration,
            requests: std::collections::VecDeque::new(),
        }
    }
    
    pub fn is_allowed(&mut self) -> bool {
        let now = SystemTime::now();
        
        // Remove old requests outside the window
        let cutoff = now - self.window_duration;
        while let Some(&front_time) = self.requests.front() {
            if front_time < cutoff {
                self.requests.pop_front();
            } else {
                break;
            }
        }
        
        // Check if we can add a new request
        if self.requests.len() < self.max_requests as usize {
            self.requests.push_back(now);
            true
        } else {
            false
        }
    }
    
    pub fn remaining_requests(&mut self) -> u32 {
        let now = SystemTime::now();
        let cutoff = now - self.window_duration;
        
        // Remove old requests
        while let Some(&front_time) = self.requests.front() {
            if front_time < cutoff {
                self.requests.pop_front();
            } else {
                break;
            }
        }
        
        self.max_requests.saturating_sub(self.requests.len() as u32)
    }
    
    pub fn reset(&mut self) {
        self.requests.clear();
    }
}

/// Performance timer for measuring execution time
#[derive(Debug)]
pub struct PerformanceTimer {
    start_time: std::time::Instant,
    checkpoints: Vec<(String, std::time::Instant)>,
}

impl PerformanceTimer {
    pub fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
            checkpoints: Vec::new(),
        }
    }
    
    pub fn checkpoint(&mut self, name: &str) {
        self.checkpoints.push((name.to_string(), std::time::Instant::now()));
    }
    
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
    
    pub fn checkpoint_duration(&self, checkpoint_name: &str) -> Option<Duration> {
        self.checkpoints.iter()
            .find(|(name, _)| name == checkpoint_name)
            .map(|(_, time)| time.duration_since(self.start_time))
    }
    
    pub fn duration_between_checkpoints(&self, start_checkpoint: &str, end_checkpoint: &str) -> Option<Duration> {
        let start_time = self.checkpoints.iter()
            .find(|(name, _)| name == start_checkpoint)
            .map(|(_, time)| *time)?;
        
        let end_time = self.checkpoints.iter()
            .find(|(name, _)| name == end_checkpoint)
            .map(|(_, time)| *time)?;
        
        Some(end_time.duration_since(start_time))
    }
    
    pub fn get_checkpoint_summary(&self) -> Vec<(String, Duration)> {
        self.checkpoints.iter()
            .map(|(name, time)| (name.clone(), time.duration_since(self.start_time)))
            .collect()
    }
}

impl Default for PerformanceTimer {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory usage tracker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryTracker {
    initial_memory: usize,
    peak_memory: usize,
    current_memory: usize,
}

impl MemoryTracker {
    pub fn new() -> Self {
        let initial = Self::get_memory_usage();
        Self {
            initial_memory: initial,
            peak_memory: initial,
            current_memory: initial,
        }
    }
    
    pub fn update(&mut self) {
        self.current_memory = Self::get_memory_usage();
        if self.current_memory > self.peak_memory {
            self.peak_memory = self.current_memory;
        }
    }
    
    pub fn memory_increase(&self) -> usize {
        self.current_memory.saturating_sub(self.initial_memory)
    }
    
    pub fn peak_memory_increase(&self) -> usize {
        self.peak_memory.saturating_sub(self.initial_memory)
    }
    
    pub fn current_memory(&self) -> usize {
        self.current_memory
    }
    
    pub fn peak_memory(&self) -> usize {
        self.peak_memory
    }
    
    fn get_memory_usage() -> usize {
        // This is a simplified implementation
        // In a real implementation, you would use platform-specific APIs
        // or libraries like `memory-stats` to get actual memory usage
        0
    }
}

impl Default for MemoryTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration validator
pub struct ConfigValidator;

impl ConfigValidator {
    pub fn validate_probability(value: f64, name: &str) -> Result<(), ConsciousnessError> {
        if !(0.0..=1.0).contains(&value) {
            return Err(ConsciousnessError::ConfigurationError(
                format!("{} must be between 0.0 and 1.0, got {}", name, value)
            ));
        }
        Ok(())
    }
    
    pub fn validate_positive(value: f64, name: &str) -> Result<(), ConsciousnessError> {
        if value <= 0.0 {
            return Err(ConsciousnessError::ConfigurationError(
                format!("{} must be positive, got {}", name, value)
            ));
        }
        Ok(())
    }
    
    pub fn validate_non_negative(value: f64, name: &str) -> Result<(), ConsciousnessError> {
        if value < 0.0 {
            return Err(ConsciousnessError::ConfigurationError(
                format!("{} must be non-negative, got {}", name, value)
            ));
        }
        Ok(())
    }
    
    pub fn validate_range(value: f64, min: f64, max: f64, name: &str) -> Result<(), ConsciousnessError> {
        if value < min || value > max {
            return Err(ConsciousnessError::ConfigurationError(
                format!("{} must be between {} and {}, got {}", name, min, max, value)
            ));
        }
        Ok(())
    }
    
    pub fn validate_non_empty_string(value: &str, name: &str) -> Result<(), ConsciousnessError> {
        if value.trim().is_empty() {
            return Err(ConsciousnessError::ConfigurationError(
                format!("{} cannot be empty", name)
            ));
        }
        Ok(())
    }
    
    pub fn validate_duration_positive(duration: Duration, name: &str) -> Result<(), ConsciousnessError> {
        if duration.is_zero() {
            return Err(ConsciousnessError::ConfigurationError(
                format!("{} must be positive", name)
            ));
        }
        Ok(())
    }
}

/// Text processing utilities
pub struct TextUtils;

impl TextUtils {
    /// Extract keywords from text
    pub fn extract_keywords(text: &str, min_length: usize) -> Vec<String> {
        text.split_whitespace()
            .map(|word| word.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect::<String>())
            .filter(|word| word.len() >= min_length)
            .filter(|word| !Self::is_stop_word(word))
            .collect()
    }
    
    /// Check if a word is a stop word
    pub fn is_stop_word(word: &str) -> bool {
        matches!(word.to_lowercase().as_str(),
            "the" | "and" | "or" | "but" | "in" | "on" | "at" | "to" | "for" | "of" | "with" | "by" |
            "is" | "are" | "was" | "were" | "be" | "been" | "being" | "have" | "has" | "had" |
            "do" | "does" | "did" | "will" | "would" | "could" | "should" | "may" | "might" |
            "this" | "that" | "these" | "those" | "a" | "an" | "i" | "you" | "he" | "she" | "it" |
            "we" | "they" | "me" | "him" | "her" | "us" | "them"
        )
    }
    
    /// Calculate text similarity using Jaccard index
    pub fn jaccard_similarity(text1: &str, text2: &str) -> f64 {
        let words1: std::collections::HashSet<String> = Self::extract_keywords(text1, 2).into_iter().collect();
        let words2: std::collections::HashSet<String> = Self::extract_keywords(text2, 2).into_iter().collect();
        
        if words1.is_empty() && words2.is_empty() {
            return 1.0;
        }
        
        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();
        
        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }
    
    /// Clean and normalize text
    pub fn normalize_text(text: &str) -> String {
        text.trim()
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() || c.is_whitespace() { c } else { ' ' })
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let vec1 = vec![1.0, 2.0, 3.0];
        let vec2 = vec![2.0, 4.0, 6.0];
        let similarity = calculate_cosine_similarity(&vec1, &vec2).unwrap();
        assert!((similarity - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_euclidean_distance() {
        let vec1 = vec![0.0, 0.0];
        let vec2 = vec![3.0, 4.0];
        let distance = calculate_euclidean_distance(&vec1, &vec2).unwrap();
        assert!((distance - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_normalize_vector() {
        let mut vec = vec![3.0, 4.0];
        normalize_vector(&mut vec).unwrap();
        let norm: f64 = vec.iter().map(|x| x * x).sum::<f64>().sqrt();
        assert!((norm - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_entropy() {
        let probs = vec![0.5, 0.5];
        let entropy = calculate_entropy(&probs).unwrap();
        assert!((entropy - 2.0_f64.ln()).abs() < 1e-10);
    }

    #[test]
    fn test_exponential_moving_average() {
        let mut ema = ExponentialMovingAverage::new(0.5).unwrap();
        assert_eq!(ema.update(10.0), 10.0);
        assert_eq!(ema.update(20.0), 15.0);
        assert_eq!(ema.update(30.0), 22.5);
    }

    #[test]
    fn test_sliding_window() {
        let mut window = SlidingWindow::new(3).unwrap();
        window.add_value(1.0);
        window.add_value(2.0);
        window.add_value(3.0);
        assert_eq!(window.mean(), Some(2.0));
        
        window.add_value(4.0);
        assert_eq!(window.mean(), Some(3.0)); // (2+3+4)/3
    }

    #[test]
    fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(2, Duration::from_secs(1));
        assert!(limiter.is_allowed());
        assert!(limiter.is_allowed());
        assert!(!limiter.is_allowed()); // Should be rate limited
    }

    #[test]
    fn test_text_utils() {
        let keywords = TextUtils::extract_keywords("The quick brown fox jumps", 3);
        assert!(keywords.contains(&"quick".to_string()));
        assert!(keywords.contains(&"brown".to_string()));
        assert!(!keywords.contains(&"the".to_string())); // Stop word
        
        let similarity = TextUtils::jaccard_similarity("hello world", "world hello");
        assert!((similarity - 1.0).abs() < 1e-10);
    }
}