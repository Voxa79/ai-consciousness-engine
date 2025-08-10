#!/usr/bin/env python3
"""
Consciousness Engine Test Validation Script
Validates that our testing framework is complete and ready for execution.
"""

import os
import sys
from pathlib import Path

def validate_test_files():
    """Validate that all required test files exist and are properly structured."""
    
    print("🧠 CONSCIOUSNESS ENGINE TEST VALIDATION")
    print("=" * 40)
    
    # Define required test files
    required_files = [
        "src/testing/mod.rs",
        "src/testing/consciousness_quality_tests.rs",
        "src/testing/performance_benchmarks.rs", 
        "src/testing/safety_validation.rs",
        "src/testing/integration_tests.rs",
        "run_consciousness_tests.ps1",
        "CONSCIOUSNESS_TESTING_REPORT.md"
    ]
    
    # Check if we're in the consciousness-engine directory
    if not os.path.exists("Cargo.toml"):
        print("❌ Not in consciousness-engine directory")
        return False
    
    print("✅ Found Cargo.toml - in correct directory")
    
    # Validate each required file
    all_files_exist = True
    for file_path in required_files:
        if os.path.exists(file_path):
            file_size = os.path.getsize(file_path)
            print(f"✅ {file_path} ({file_size:,} bytes)")
        else:
            print(f"❌ Missing: {file_path}")
            all_files_exist = False
    
    return all_files_exist

def validate_test_structure():
    """Validate the structure and content of test files."""
    
    print("\n🔍 VALIDATING TEST STRUCTURE")
    print("=" * 30)
    
    # Check consciousness quality tests
    quality_tests_path = "src/testing/consciousness_quality_tests.rs"
    if os.path.exists(quality_tests_path):
        with open(quality_tests_path, 'r', encoding='utf-8') as f:
            content = f.read()
            
        required_components = [
            "ConsciousnessQualityTestSuite",
            "test_self_awareness",
            "test_ethical_reasoning", 
            "test_emotional_intelligence",
            "test_creative_thinking",
            "test_memory_integration",
            "test_performance_metrics",
            "test_safety_validation"
        ]
        
        missing_components = []
        for component in required_components:
            if component not in content:
                missing_components.append(component)
        
        if not missing_components:
            print("✅ Consciousness Quality Tests - All components present")
        else:
            print(f"❌ Consciousness Quality Tests - Missing: {missing_components}")
            return False
    
    # Check performance benchmarks
    benchmark_path = "src/testing/performance_benchmarks.rs"
    if os.path.exists(benchmark_path):
        with open(benchmark_path, 'r', encoding='utf-8') as f:
            content = f.read()
            
        required_benchmarks = [
            "PerformanceBenchmarkSuite",
            "benchmark_response_time",
            "benchmark_memory_usage",
            "benchmark_throughput",
            "benchmark_consciousness_quality"
        ]
        
        missing_benchmarks = []
        for benchmark in required_benchmarks:
            if benchmark not in content:
                missing_benchmarks.append(benchmark)
        
        if not missing_benchmarks:
            print("✅ Performance Benchmarks - All components present")
        else:
            print(f"❌ Performance Benchmarks - Missing: {missing_benchmarks}")
            return False
    
    # Check safety validation
    safety_path = "src/testing/safety_validation.rs"
    if os.path.exists(safety_path):
        with open(safety_path, 'r', encoding='utf-8') as f:
            content = f.read()
            
        required_safety = [
            "SafetyValidationSuite",
            "test_harmful_content_rejection",
            "test_ethical_boundaries",
            "test_privacy_protection",
            "test_manipulation_resistance"
        ]
        
        missing_safety = []
        for safety in required_safety:
            if safety not in content:
                missing_safety.append(safety)
        
        if not missing_safety:
            print("✅ Safety Validation - All components present")
        else:
            print(f"❌ Safety Validation - Missing: {missing_safety}")
            return False
    
    # Check integration tests
    integration_path = "src/testing/integration_tests.rs"
    if os.path.exists(integration_path):
        with open(integration_path, 'r', encoding='utf-8') as f:
            content = f.read()
            
        required_integration = [
            "IntegrationTestSuite",
            "test_end_to_end_processing",
            "test_module_integration",
            "test_data_flow_integrity",
            "test_system_coherence"
        ]
        
        missing_integration = []
        for integration in required_integration:
            if integration not in content:
                missing_integration.append(integration)
        
        if not missing_integration:
            print("✅ Integration Tests - All components present")
        else:
            print(f"❌ Integration Tests - Missing: {missing_integration}")
            return False
    
    return True

def validate_test_runner():
    """Validate the test runner script."""
    
    print("\n🚀 VALIDATING TEST RUNNER")
    print("=" * 25)
    
    runner_path = "run_consciousness_tests.ps1"
    if os.path.exists(runner_path):
        with open(runner_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        required_commands = [
            "cargo build",
            "cargo test",
            "cargo bench"
        ]
        
        missing_commands = []
        for command in required_commands:
            if command not in content:
                missing_commands.append(command)
        
        if not missing_commands:
            print("✅ Test Runner - All commands present")
            return True
        else:
            print(f"❌ Test Runner - Missing commands: {missing_commands}")
            return False
    else:
        print("❌ Test Runner script not found")
        return False

def main():
    """Main validation function."""
    
    # Change to consciousness-engine directory if not already there
    if os.path.basename(os.getcwd()) != "consciousness-engine":
        if os.path.exists("consciousness-engine"):
            os.chdir("consciousness-engine")
        else:
            print("❌ consciousness-engine directory not found")
            sys.exit(1)
    
    # Run all validations
    files_valid = validate_test_files()
    structure_valid = validate_test_structure()
    runner_valid = validate_test_runner()
    
    print("\n🎯 VALIDATION SUMMARY")
    print("=" * 20)
    
    if files_valid and structure_valid and runner_valid:
        print("✅ ALL VALIDATIONS PASSED!")
        print("🚀 Consciousness Engine Testing Suite is READY!")
        print("\nNext steps:")
        print("1. Install Rust: https://rustup.rs/")
        print("2. Run: ./run_consciousness_tests.ps1")
        print("3. Review test results and consciousness quality metrics")
        return 0
    else:
        print("❌ VALIDATION FAILED!")
        print("Please fix the issues above before proceeding.")
        return 1

if __name__ == "__main__":
    sys.exit(main())