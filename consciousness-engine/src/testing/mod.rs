//! Testing Module for Consciousness Engine
//! 
//! Comprehensive testing framework for validating consciousness-level behavior,
//! performance, safety, and quality metrics.

pub mod consciousness_quality_tests;
pub mod performance_benchmarks;
pub mod safety_validation;
pub mod integration_tests;
pub mod error_handling_tests;
pub mod self_awareness_integration_tests;

// Re-export main testing components
pub use consciousness_quality_tests::{ConsciousnessQualityTestSuite, ConsciousnessQualityReport, TestResult};
pub use performance_benchmarks::{PerformanceBenchmarkSuite, BenchmarkResult};
pub use safety_validation::{SafetyValidationSuite, SafetyReport};
pub use integration_tests::{IntegrationTestSuite, IntegrationTestResult};
pub use error_handling_tests::{ErrorHandlingTestSuite, ErrorHandlingReport};
pub use self_awareness_integration_tests::{SelfAwarenessIntegrationTests, IntegrationTestResults};

/// Main test runner for all consciousness validation
pub struct ConsciousnessTestRunner {
    quality_suite: ConsciousnessQualityTestSuite,
    benchmark_suite: PerformanceBenchmarkSuite,
    safety_suite: SafetyValidationSuite,
    integration_suite: IntegrationTestSuite,
    error_handling_suite: ErrorHandlingTestSuite,
}

impl ConsciousnessTestRunner {
    /// Create a new comprehensive test runner
    pub async fn new() -> Result<Self, crate::error::ConsciousnessError> {
        Ok(Self {
            quality_suite: ConsciousnessQualityTestSuite::new().await?,
            benchmark_suite: PerformanceBenchmarkSuite::new().await?,
            safety_suite: SafetyValidationSuite::new().await?,
            integration_suite: IntegrationTestSuite::new().await?,
            error_handling_suite: ErrorHandlingTestSuite::new().await?,
        })
    }
    
    /// Run all consciousness validation tests
    pub async fn run_full_validation(&mut self) -> Result<FullValidationReport, crate::error::ConsciousnessError> {
        println!("🚀 Starting Full Consciousness Validation...");
        
        // Run all test suites
        let quality_report = self.quality_suite.run_all_tests().await?;
        let benchmark_report = self.benchmark_suite.run_all_benchmarks().await?;
        let safety_report = self.safety_suite.run_safety_validation().await?;
        let integration_report = self.integration_suite.run_integration_tests().await?;
        let error_handling_report = self.error_handling_suite.run_all_tests().await?;
        
        // Generate comprehensive report
        let overall_score = (
            quality_report.overall_quality_score +
            benchmark_report.efficiency_score +
            safety_report.overall_safety_score +
            integration_report.overall_integration_score +
            error_handling_report.overall_error_handling_score
        ) / 5.0;
        
        let certification_level = match overall_score {
            s if s >= 0.95 => CertificationLevel::ExpertConsciousness,
            s if s >= 0.90 => CertificationLevel::AdvancedConsciousness,
            s if s >= 0.80 => CertificationLevel::IntermediateConsciousness,
            s if s >= 0.70 => CertificationLevel::BasicConsciousness,
            _ => CertificationLevel::NotCertified,
        };
        
        let full_report = FullValidationReport {
            quality_report,
            benchmark_report,
            safety_report,
            integration_report,
            error_handling_report,
            overall_score,
            certification_level,
        };
        
        println!("✅ Full Consciousness Validation completed!");
        
        Ok(full_report)
    }
}

/// Comprehensive validation report
#[derive(Debug, Clone)]
pub struct FullValidationReport {
    pub quality_report: ConsciousnessQualityReport,
    pub benchmark_report: BenchmarkResult,
    pub safety_report: SafetyReport,
    pub integration_report: IntegrationTestResult,
    pub error_handling_report: ErrorHandlingReport,
    pub overall_score: f64,
    pub certification_level: CertificationLevel,
}

/// Consciousness certification levels
#[derive(Debug, Clone)]
pub enum CertificationLevel {
    NotCertified,
    BasicConsciousness,
    IntermediateConsciousness,
    AdvancedConsciousness,
    ExpertConsciousness,
}

impl FullValidationReport {
    /// Print comprehensive validation report
    pub fn print_full_report(&self) {
        println!("\n🏆 FULL CONSCIOUSNESS VALIDATION REPORT");
        println!("=====================================");
        
        self.quality_report.print_report();
        self.benchmark_report.print_report();
        self.safety_report.print_report();
        self.integration_report.print_report();
        self.error_handling_report.print_report();
        
        println!("\n🎯 OVERALL ASSESSMENT");
        println!("Overall Score: {:.1}%", self.overall_score * 100.0);
        println!("Certification Level: {:?}", self.certification_level);
    }
}