//! Confidence Estimation Module - Bayesian Confidence Modeling
//! 
//! This module implements advanced confidence estimation using Bayesian methods
//! to provide accurate self-assessment of certainty levels.

use crate::types::*;
use crate::error::{ConsciousnessError, ConsciousnessResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant, SystemTime};
use tracing::{debug, info, warn};

/// Confidence Estimator using Bayesian confidence modeling
#[derive(Debug, Clone)]
pub struct ConfidenceEstimator {
    /// Bayesian confidence model
    bayesian_model: BayesianConfidenceModel,
    
    /// Historical confidence data
    confidence_history: Vec<ConfidenceDataPoint>,
    
    /// Calibration data for confidence accuracy
    calibration_data: ConfidenceCalibrationData,
    
    /// Configuration
    config: ConfidenceEstimationConfig,
}

/// Bayesian confidence model for uncertainty quantification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BayesianConfidenceModel {
    /// Prior beliefs about confidence accuracy
    prior_beliefs: PriorBeliefs,
    
    /// Evidence accumulation
    evidence_accumulator: EvidenceAccumulator,
    
    /// Posterior confidence distribution
    posterior_distribution: PosteriorDistribution,
    
    /// Model parameters
    model_parameters: ModelParameters,
}

/// Prior beliefs about confidence accuracy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorBeliefs {
    /// Prior confidence in own assessments
    self_assessment_confidence: f64,
    
    /// Prior belief about accuracy
    accuracy_prior: BetaDistribution,
    
    /// Prior belief about calibration
    calibration_prior: BetaDistribution,
    
    /// Domain-specific priors
    domain_priors: HashMap<String, f64>,
}

/// Beta distribution for Bayesian modeling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaDistribution {
    pub alpha: f64,
    pub beta: f64,
}

impl BetaDistribution {
    pub fn new(alpha: f64, beta: f64) -> Self {
        Self { alpha, beta }
    }
    
    pub fn mean(&self) -> f64 {
        self.alpha / (self.alpha + self.beta)
    }
    
    pub fn variance(&self) -> f64 {
        let sum = self.alpha + self.beta;
        (self.alpha * self.beta) / (sum * sum * (sum + 1.0))
    }
    
    pub fn update(&mut self, success: bool) {
        if success {
            self.alpha += 1.0;
        } else {
            self.beta += 1.0;
        }
    }
}

/// Evidence accumulator for Bayesian updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceAccumulator {
    /// Total evidence points
    total_evidence: u64,
    
    /// Successful predictions
    successful_predictions: u64,
    
    /// Evidence by domain
    domain_evidence: HashMap<String, DomainEvidence>,
    
    /// Recent evidence (for adaptation)
    recent_evidence: Vec<EvidencePoint>,
}

/// Domain-specific evidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainEvidence {
    pub domain_name: String,
    pub total_predictions: u64,
    pub successful_predictions: u64,
    pub confidence_accuracy: f64,
    pub last_updated: SystemTime,
}

/// Individual evidence point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidencePoint {
    pub timestamp: SystemTime,
    pub predicted_confidence: f64,
    pub actual_outcome: bool,
    pub domain: String,
    pub context: String,
}

/// Posterior confidence distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PosteriorDistribution {
    /// Current confidence estimate
    point_estimate: f64,
    
    /// Confidence interval
    confidence_interval: (f64, f64),
    
    /// Uncertainty measure
    uncertainty: f64,
    
    /// Distribution parameters
    distribution_params: BetaDistribution,
}

/// Model parameters for confidence estimation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelParameters {
    /// Learning rate for Bayesian updates
    learning_rate: f64,
    
    /// Forgetting factor for recent evidence
    forgetting_factor: f64,
    
    /// Minimum evidence threshold
    min_evidence_threshold: u64,
    
    /// Confidence interval width (e.g., 0.95 for 95% CI)
    confidence_interval_width: f64,
}

/// Historical confidence data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceDataPoint {
    pub timestamp: SystemTime,
    pub task_domain: String,
    pub predicted_confidence: f64,
    pub actual_performance: f64,
    pub confidence_accuracy: f64,
    pub calibration_error: f64,
}

/// Confidence calibration data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceCalibrationData {
    /// Calibration curve data points
    calibration_curve: Vec<CalibrationPoint>,
    
    /// Overall calibration score
    overall_calibration: f64,
    
    /// Overconfidence bias measure
    overconfidence_bias: f64,
    
    /// Underconfidence bias measure
    underconfidence_bias: f64,
    
    /// Last calibration update
    last_calibration: SystemTime,
}

/// Point on the calibration curve
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationPoint {
    pub confidence_bin: f64,
    pub actual_accuracy: f64,
    pub sample_count: u64,
}

/// Configuration for confidence estimation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceEstimationConfig {
    /// Enable Bayesian updating
    pub enable_bayesian_updates: bool,
    
    /// Calibration update frequency
    pub calibration_frequency: Duration,
    
    /// Minimum samples for calibration
    pub min_calibration_samples: u64,
    
    /// Confidence threshold for high-stakes decisions
    pub high_stakes_threshold: f64,
    
    /// Enable uncertainty quantification
    pub enable_uncertainty_quantification: bool,
}

impl Default for ConfidenceEstimationConfig {
    fn default() -> Self {
        Self {
            enable_bayesian_updates: true,
            calibration_frequency: Duration::from_secs(3600), // 1 hour
            min_calibration_samples: 50,
            high_stakes_threshold: 0.9,
            enable_uncertainty_quantification: true,
        }
    }
}

impl ConfidenceEstimator {
    /// Create a new confidence estimator
    pub fn new() -> Self {
        info!("Initializing Bayesian Confidence Estimator");
        
        let prior_beliefs = PriorBeliefs {
            self_assessment_confidence: 0.8,
            accuracy_prior: BetaDistribution::new(8.0, 2.0), // Optimistic prior
            calibration_prior: BetaDistribution::new(5.0, 5.0), // Neutral prior
            domain_priors: HashMap::new(),
        };
        
        let evidence_accumulator = EvidenceAccumulator {
            total_evidence: 0,
            successful_predictions: 0,
            domain_evidence: HashMap::new(),
            recent_evidence: Vec::new(),
        };
        
        let posterior_distribution = PosteriorDistribution {
            point_estimate: prior_beliefs.accuracy_prior.mean(),
            confidence_interval: (0.6, 0.9),
            uncertainty: prior_beliefs.accuracy_prior.variance().sqrt(),
            distribution_params: prior_beliefs.accuracy_prior.clone(),
        };
        
        let model_parameters = ModelParameters {
            learning_rate: 0.1,
            forgetting_factor: 0.95,
            min_evidence_threshold: 10,
            confidence_interval_width: 0.95,
        };
        
        let bayesian_model = BayesianConfidenceModel {
            prior_beliefs,
            evidence_accumulator,
            posterior_distribution,
            model_parameters,
        };
        
        Self {
            bayesian_model,
            confidence_history: Vec::new(),
            calibration_data: ConfidenceCalibrationData {
                calibration_curve: Vec::new(),
                overall_calibration: 0.8,
                overconfidence_bias: 0.0,
                underconfidence_bias: 0.0,
                last_calibration: SystemTime::now(),
            },
            config: ConfidenceEstimationConfig::default(),
        }
    }
    
    /// Estimate confidence using Bayesian modeling
    pub async fn estimate_confidence(
        &mut self,
        task_domain: &str,
        performance_indicators: &PerformanceIndicators,
        context: &ConfidenceContext,
    ) -> ConsciousnessResult<ConfidenceEstimate> {
        let start_time = Instant::now();
        
        debug!("Estimating confidence for domain: {}", task_domain);
        
        // 1. Get domain-specific prior
        let domain_prior = self.get_domain_prior(task_domain).await?;
        
        // 2. Calculate base confidence from performance indicators
        let base_confidence = self.calculate_base_confidence(performance_indicators).await?;
        
        // 3. Apply Bayesian updating
        let bayesian_confidence = self.apply_bayesian_update(
            base_confidence,
            domain_prior,
            task_domain,
            context,
        ).await?;
        
        // 4. Quantify uncertainty
        let uncertainty = self.quantify_uncertainty(task_domain, context).await?;
        
        // 5. Apply calibration correction
        let calibrated_confidence = self.apply_calibration_correction(
            bayesian_confidence,
            task_domain,
        ).await?;
        
        // 6. Generate confidence interval
        let confidence_interval = self.generate_confidence_interval(
            calibrated_confidence,
            uncertainty,
        ).await?;
        
        let confidence_estimate = ConfidenceEstimate {
            point_estimate: calibrated_confidence,
            confidence_interval,
            uncertainty_measure: uncertainty,
            domain: task_domain.to_string(),
            evidence_strength: self.calculate_evidence_strength(task_domain).await?,
            calibration_quality: self.get_calibration_quality(task_domain).await?,
            bayesian_factors: self.get_bayesian_factors(task_domain).await?,
            timestamp: SystemTime::now(),
        };
        
        debug!("Confidence estimation completed in {:?}", start_time.elapsed());
        
        Ok(confidence_estimate)
    }
    
    /// Update confidence model with new evidence
    pub async fn update_with_evidence(
        &mut self,
        predicted_confidence: f64,
        actual_outcome: bool,
        task_domain: &str,
        context: &str,
    ) -> ConsciousnessResult<()> {
        debug!("Updating confidence model with new evidence");
        
        // Create evidence point
        let evidence_point = EvidencePoint {
            timestamp: SystemTime::now(),
            predicted_confidence,
            actual_outcome,
            domain: task_domain.to_string(),
            context: context.to_string(),
        };
        
        // Update evidence accumulator
        self.bayesian_model.evidence_accumulator.total_evidence += 1;
        if actual_outcome {
            self.bayesian_model.evidence_accumulator.successful_predictions += 1;
        }
        
        // Add to recent evidence
        self.bayesian_model.evidence_accumulator.recent_evidence.push(evidence_point.clone());
        
        // Limit recent evidence size
        if self.bayesian_model.evidence_accumulator.recent_evidence.len() > 1000 {
            self.bayesian_model.evidence_accumulator.recent_evidence.remove(0);
        }
        
        // Update domain-specific evidence
        self.update_domain_evidence(task_domain, predicted_confidence, actual_outcome).await?;
        
        // Update Bayesian model
        if self.config.enable_bayesian_updates {
            self.update_bayesian_model(predicted_confidence, actual_outcome).await?;
        }
        
        // Update calibration data
        self.update_calibration_data(predicted_confidence, actual_outcome).await?;
        
        // Store in confidence history
        let actual_performance = if actual_outcome { 1.0 } else { 0.0 };
        let confidence_accuracy = 1.0 - (predicted_confidence - actual_performance).abs();
        let calibration_error = predicted_confidence - actual_performance;
        
        let data_point = ConfidenceDataPoint {
            timestamp: SystemTime::now(),
            task_domain: task_domain.to_string(),
            predicted_confidence,
            actual_performance,
            confidence_accuracy,
            calibration_error,
        };
        
        self.confidence_history.push(data_point);
        
        // Limit history size
        if self.confidence_history.len() > 10000 {
            self.confidence_history.remove(0);
        }
        
        info!("Confidence model updated with new evidence");
        
        Ok(())
    }
    
    /// Get confidence calibration report
    pub async fn get_calibration_report(&self) -> ConsciousnessResult<CalibrationReport> {
        let overall_accuracy = if self.bayesian_model.evidence_accumulator.total_evidence > 0 {
            self.bayesian_model.evidence_accumulator.successful_predictions as f64 
                / self.bayesian_model.evidence_accumulator.total_evidence as f64
        } else {
            0.5
        };
        
        let calibration_error = self.calculate_calibration_error().await?;
        let overconfidence_measure = self.calculate_overconfidence().await?;
        let underconfidence_measure = self.calculate_underconfidence().await?;
        
        Ok(CalibrationReport {
            overall_accuracy,
            calibration_error,
            overconfidence_bias: overconfidence_measure,
            underconfidence_bias: underconfidence_measure,
            calibration_curve: self.calibration_data.calibration_curve.clone(),
            sample_count: self.bayesian_model.evidence_accumulator.total_evidence,
            last_updated: SystemTime::now(),
        })
    }
    
    // Private helper methods
    
    async fn get_domain_prior(&self, domain: &str) -> ConsciousnessResult<f64> {
        // Get domain-specific prior or use general prior
        let prior = self.bayesian_model.prior_beliefs.domain_priors
            .get(domain)
            .copied()
            .unwrap_or(self.bayesian_model.prior_beliefs.accuracy_prior.mean());
        
        Ok(prior)
    }
    
    async fn calculate_base_confidence(&self, indicators: &PerformanceIndicators) -> ConsciousnessResult<f64> {
        // Calculate base confidence from performance indicators
        let accuracy_weight = 0.4;
        let consistency_weight = 0.3;
        let complexity_weight = 0.2;
        let experience_weight = 0.1;
        
        let base_confidence = 
            indicators.accuracy * accuracy_weight +
            indicators.consistency * consistency_weight +
            (1.0 - indicators.task_complexity) * complexity_weight +
            indicators.domain_experience * experience_weight;
        
        Ok(base_confidence.max(0.0).min(1.0))
    }
    
    async fn apply_bayesian_update(
        &self,
        base_confidence: f64,
        domain_prior: f64,
        domain: &str,
        context: &ConfidenceContext,
    ) -> ConsciousnessResult<f64> {
        // Apply Bayesian updating
        let prior_weight = self.calculate_prior_weight(domain, context).await?;
        let evidence_weight = 1.0 - prior_weight;
        
        let bayesian_confidence = domain_prior * prior_weight + base_confidence * evidence_weight;
        
        Ok(bayesian_confidence.max(0.0).min(1.0))
    }
    
    async fn quantify_uncertainty(&self, domain: &str, context: &ConfidenceContext) -> ConsciousnessResult<f64> {
        // Quantify uncertainty based on evidence strength and context
        let evidence_strength = self.calculate_evidence_strength(domain).await?;
        let context_uncertainty = self.calculate_context_uncertainty(context).await?;
        
        // Higher uncertainty with less evidence or more complex context
        let uncertainty = (1.0 - evidence_strength) * 0.6 + context_uncertainty * 0.4;
        
        Ok(uncertainty.max(0.0).min(1.0))
    }
    
    async fn apply_calibration_correction(&self, confidence: f64, domain: &str) -> ConsciousnessResult<f64> {
        // Apply calibration correction based on historical performance
        let calibration_factor = self.get_calibration_factor(domain).await?;
        
        // Adjust confidence based on historical over/under-confidence
        let corrected_confidence = if calibration_factor > 1.0 {
            // Historically overconfident, reduce confidence
            confidence * (2.0 - calibration_factor)
        } else {
            // Historically underconfident, increase confidence
            confidence * calibration_factor
        };
        
        Ok(corrected_confidence.max(0.0).min(1.0))
    }
    
    async fn generate_confidence_interval(&self, point_estimate: f64, uncertainty: f64) -> ConsciousnessResult<(f64, f64)> {
        // Generate confidence interval based on uncertainty
        let interval_width = uncertainty * 2.0; // 2-sigma interval
        let lower_bound = (point_estimate - interval_width / 2.0).max(0.0);
        let upper_bound = (point_estimate + interval_width / 2.0).min(1.0);
        
        Ok((lower_bound, upper_bound))
    }
    
    async fn calculate_evidence_strength(&self, domain: &str) -> ConsciousnessResult<f64> {
        // Calculate evidence strength for domain
        let domain_evidence = self.bayesian_model.evidence_accumulator.domain_evidence
            .get(domain);
        
        let evidence_count = domain_evidence
            .map(|e| e.total_predictions)
            .unwrap_or(0);
        
        // Sigmoid function to map evidence count to strength
        let strength = 1.0 / (1.0 + (-0.1 * evidence_count as f64).exp());
        
        Ok(strength)
    }
    
    async fn get_calibration_quality(&self, _domain: &str) -> ConsciousnessResult<f64> {
        // Get calibration quality score
        Ok(self.calibration_data.overall_calibration)
    }
    
    async fn get_bayesian_factors(&self, domain: &str) -> ConsciousnessResult<BayesianFactors> {
        let prior = self.get_domain_prior(domain).await?;
        let evidence_strength = self.calculate_evidence_strength(domain).await?;
        let posterior = self.bayesian_model.posterior_distribution.point_estimate;
        
        Ok(BayesianFactors {
            prior_belief: prior,
            evidence_strength,
            posterior_estimate: posterior,
            bayes_factor: if prior > 0.0 { posterior / prior } else { 1.0 },
        })
    }
    
    async fn update_domain_evidence(&mut self, domain: &str, predicted_confidence: f64, actual_outcome: bool) -> ConsciousnessResult<()> {
        let domain_evidence = self.bayesian_model.evidence_accumulator.domain_evidence
            .entry(domain.to_string())
            .or_insert(DomainEvidence {
                domain_name: domain.to_string(),
                total_predictions: 0,
                successful_predictions: 0,
                confidence_accuracy: 0.0,
                last_updated: SystemTime::now(),
            });
        
        domain_evidence.total_predictions += 1;
        if actual_outcome {
            domain_evidence.successful_predictions += 1;
        }
        
        // Update confidence accuracy
        let actual_performance = if actual_outcome { 1.0 } else { 0.0 };
        let accuracy = 1.0 - (predicted_confidence - actual_performance).abs();
        
        domain_evidence.confidence_accuracy = 
            (domain_evidence.confidence_accuracy * (domain_evidence.total_predictions - 1) as f64 + accuracy) 
            / domain_evidence.total_predictions as f64;
        
        domain_evidence.last_updated = SystemTime::now();
        
        Ok(())
    }
    
    async fn update_bayesian_model(&mut self, predicted_confidence: f64, actual_outcome: bool) -> ConsciousnessResult<()> {
        // Update posterior distribution
        self.bayesian_model.posterior_distribution.distribution_params.update(actual_outcome);
        
        // Update point estimate
        self.bayesian_model.posterior_distribution.point_estimate = 
            self.bayesian_model.posterior_distribution.distribution_params.mean();
        
        // Update uncertainty
        self.bayesian_model.posterior_distribution.uncertainty = 
            self.bayesian_model.posterior_distribution.distribution_params.variance().sqrt();
        
        Ok(())
    }
    
    async fn update_calibration_data(&mut self, predicted_confidence: f64, actual_outcome: bool) -> ConsciousnessResult<()> {
        // Update calibration curve
        let confidence_bin = (predicted_confidence * 10.0).floor() / 10.0; // 0.1 bins
        
        // Find or create calibration point
        let calibration_point = self.calibration_data.calibration_curve
            .iter_mut()
            .find(|p| (p.confidence_bin - confidence_bin).abs() < 0.05);
        
        if let Some(point) = calibration_point {
            let old_accuracy = point.actual_accuracy * point.sample_count as f64;
            point.sample_count += 1;
            let new_outcome = if actual_outcome { 1.0 } else { 0.0 };
            point.actual_accuracy = (old_accuracy + new_outcome) / point.sample_count as f64;
        } else {
            self.calibration_data.calibration_curve.push(CalibrationPoint {
                confidence_bin,
                actual_accuracy: if actual_outcome { 1.0 } else { 0.0 },
                sample_count: 1,
            });
        }
        
        // Update overall calibration
        self.recalculate_overall_calibration().await?;
        
        Ok(())
    }
    
    async fn recalculate_overall_calibration(&mut self) -> ConsciousnessResult<()> {
        if self.calibration_data.calibration_curve.is_empty() {
            return Ok(());
        }
        
        // Calculate weighted average calibration error
        let mut total_error = 0.0;
        let mut total_samples = 0u64;
        
        for point in &self.calibration_data.calibration_curve {
            let error = (point.confidence_bin - point.actual_accuracy).abs();
            total_error += error * point.sample_count as f64;
            total_samples += point.sample_count;
        }
        
        if total_samples > 0 {
            let avg_error = total_error / total_samples as f64;
            self.calibration_data.overall_calibration = 1.0 - avg_error;
        }
        
        Ok(())
    }
    
    async fn calculate_calibration_error(&self) -> ConsciousnessResult<f64> {
        if self.confidence_history.is_empty() {
            return Ok(0.0);
        }
        
        let total_error: f64 = self.confidence_history
            .iter()
            .map(|point| point.calibration_error.abs())
            .sum();
        
        Ok(total_error / self.confidence_history.len() as f64)
    }
    
    async fn calculate_overconfidence(&self) -> ConsciousnessResult<f64> {
        if self.confidence_history.is_empty() {
            return Ok(0.0);
        }
        
        let overconfident_points: Vec<&ConfidenceDataPoint> = self.confidence_history
            .iter()
            .filter(|point| point.calibration_error > 0.0) // Predicted higher than actual
            .collect();
        
        if overconfident_points.is_empty() {
            return Ok(0.0);
        }
        
        let avg_overconfidence: f64 = overconfident_points
            .iter()
            .map(|point| point.calibration_error)
            .sum::<f64>() / overconfident_points.len() as f64;
        
        Ok(avg_overconfidence)
    }
    
    async fn calculate_underconfidence(&self) -> ConsciousnessResult<f64> {
        if self.confidence_history.is_empty() {
            return Ok(0.0);
        }
        
        let underconfident_points: Vec<&ConfidenceDataPoint> = self.confidence_history
            .iter()
            .filter(|point| point.calibration_error < 0.0) // Predicted lower than actual
            .collect();
        
        if underconfident_points.is_empty() {
            return Ok(0.0);
        }
        
        let avg_underconfidence: f64 = underconfident_points
            .iter()
            .map(|point| point.calibration_error.abs())
            .sum::<f64>() / underconfident_points.len() as f64;
        
        Ok(avg_underconfidence)
    }
    
    async fn calculate_prior_weight(&self, domain: &str, context: &ConfidenceContext) -> ConsciousnessResult<f64> {
        // Calculate weight for prior vs evidence based on domain experience and context
        let evidence_strength = self.calculate_evidence_strength(domain).await?;
        let context_familiarity = context.familiarity_score;
        
        // More weight on evidence when we have more experience
        let prior_weight = (1.0 - evidence_strength) * (1.0 - context_familiarity);
        
        Ok(prior_weight.max(0.1).min(0.9)) // Keep some balance
    }
    
    async fn calculate_context_uncertainty(&self, context: &ConfidenceContext) -> ConsciousnessResult<f64> {
        // Calculate uncertainty based on context factors
        let complexity_uncertainty = context.task_complexity;
        let novelty_uncertainty = 1.0 - context.familiarity_score;
        let ambiguity_uncertainty = context.ambiguity_level;
        
        let overall_uncertainty = (complexity_uncertainty + novelty_uncertainty + ambiguity_uncertainty) / 3.0;
        
        Ok(overall_uncertainty.max(0.0).min(1.0))
    }
    
    async fn get_calibration_factor(&self, domain: &str) -> ConsciousnessResult<f64> {
        // Get calibration factor for domain
        let domain_evidence = self.bayesian_model.evidence_accumulator.domain_evidence.get(domain);
        
        if let Some(evidence) = domain_evidence {
            if evidence.total_predictions > 10 {
                // Calculate calibration factor based on historical accuracy
                let expected_accuracy = evidence.confidence_accuracy;
                let actual_accuracy = evidence.successful_predictions as f64 / evidence.total_predictions as f64;
                
                if expected_accuracy > 0.0 {
                    return Ok(actual_accuracy / expected_accuracy);
                }
            }
        }
        
        Ok(1.0) // Neutral calibration factor
    }
}

// Supporting data structures

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceIndicators {
    pub accuracy: f64,
    pub consistency: f64,
    pub task_complexity: f64,
    pub domain_experience: f64,
    pub recent_performance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceContext {
    pub task_complexity: f64,
    pub familiarity_score: f64,
    pub ambiguity_level: f64,
    pub stakes_level: f64,
    pub time_pressure: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceEstimate {
    pub point_estimate: f64,
    pub confidence_interval: (f64, f64),
    pub uncertainty_measure: f64,
    pub domain: String,
    pub evidence_strength: f64,
    pub calibration_quality: f64,
    pub bayesian_factors: BayesianFactors,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BayesianFactors {
    pub prior_belief: f64,
    pub evidence_strength: f64,
    pub posterior_estimate: f64,
    pub bayes_factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationReport {
    pub overall_accuracy: f64,
    pub calibration_error: f64,
    pub overconfidence_bias: f64,
    pub underconfidence_bias: f64,
    pub calibration_curve: Vec<CalibrationPoint>,
    pub sample_count: u64,
    pub last_updated: SystemTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_confidence_estimator_creation() {
        let estimator = ConfidenceEstimator::new();
        assert!(estimator.bayesian_model.prior_beliefs.self_assessment_confidence > 0.0);
        assert_eq!(estimator.confidence_history.len(), 0);
    }

    #[tokio::test]
    async fn test_beta_distribution() {
        let mut beta = BetaDistribution::new(5.0, 5.0);
        assert_eq!(beta.mean(), 0.5);
        
        beta.update(true);
        assert!(beta.mean() > 0.5);
        
        beta.update(false);
        // Should be back close to 0.5
        assert!((beta.mean() - 0.5).abs() < 0.1);
    }

    #[tokio::test]
    async fn test_confidence_estimation() {
        let mut estimator = ConfidenceEstimator::new();
        
        let indicators = PerformanceIndicators {
            accuracy: 0.9,
            consistency: 0.8,
            task_complexity: 0.5,
            domain_experience: 0.7,
            recent_performance: 0.85,
        };
        
        let context = ConfidenceContext {
            task_complexity: 0.5,
            familiarity_score: 0.8,
            ambiguity_level: 0.3,
            stakes_level: 0.6,
            time_pressure: 0.4,
        };
        
        let estimate = estimator.estimate_confidence("reasoning", &indicators, &context).await.unwrap();
        
        assert!(estimate.point_estimate > 0.0);
        assert!(estimate.point_estimate <= 1.0);
        assert!(estimate.confidence_interval.0 <= estimate.point_estimate);
        assert!(estimate.confidence_interval.1 >= estimate.point_estimate);
    }

    #[tokio::test]
    async fn test_evidence_update() {
        let mut estimator = ConfidenceEstimator::new();
        
        // Add some evidence
        estimator.update_with_evidence(0.8, true, "reasoning", "test context").await.unwrap();
        estimator.update_with_evidence(0.7, false, "reasoning", "test context").await.unwrap();
        estimator.update_with_evidence(0.9, true, "reasoning", "test context").await.unwrap();
        
        assert_eq!(estimator.bayesian_model.evidence_accumulator.total_evidence, 3);
        assert_eq!(estimator.bayesian_model.evidence_accumulator.successful_predictions, 2);
        assert_eq!(estimator.confidence_history.len(), 3);
    }

    #[tokio::test]
    async fn test_calibration_report() {
        let mut estimat    }
} > 0);
ntple_cousamrt!(report.        asse.0);
curacy <= 1all_acoverert!(report.   ass0);
     0.acy >= _accurallverreport.oert!(
        ass;
        nwrap()).await.urt(n_repoibratioalt_cgemator.estiet report = 
        l   }
            p();
 t.unwra).await"texcon"test", "me, nce, outco(confidevidencewith_eator.update_      estimes
       outcomrnating Alte; //2 == 0me = i %    let outco0
         / 0.5 to 1.); /40.0/ (i as f64 + ce = 0.5 iden  let conf      
     in 0..20 {  for i
      nratioibr calfovidence Add e        //      

   );::new(ordenceEstimat = Confior