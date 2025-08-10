# Simple local test script for Consciousness Engine
# Tests the engine without Docker for quick development iteration

param(
    [string]$Mode = "basic",
    [int]$Iterations = 10
)

$Green = "Green"
$Red = "Red"
$Yellow = "Yellow"
$Blue = "Blue"

function Write-Log {
    param([string]$Message, [string]$Color = "White")
    $timestamp = Get-Date -Format "HH:mm:ss"
    Write-Host "[$timestamp] $Message" -ForegroundColor $Color
}

function Test-RustInstallation {
    Write-Log "Checking Rust installation..." $Blue
    try {
        $rustVersion = rustc --version
        Write-Log "✅ Rust is available: $rustVersion" $Green
        return $true
    }
    catch {
        Write-Log "❌ Rust is not installed. Please install Rust from https://rustup.rs/" $Red
        return $false
    }
}

function Build-ConsciousnessEngine {
    Write-Log "Building Consciousness Engine..." $Yellow
    
    Set-Location "consciousness-engine"
    
    try {
        # Build in release mode for performance testing
        cargo build --release --example consciousness_demo
        Write-Log "✅ Build completed successfully" $Green
        return $true
    }
    catch {
        Write-Log "❌ Build failed. Check the error messages above." $Red
        return $false
    }
    finally {
        Set-Location ".."
    }
}

function Run-BasicTest {
    Write-Log "🧠 Running basic consciousness test..." $Blue
    
    Set-Location "consciousness-engine"
    
    try {
        # Run the consciousness demo
        $output = & "target/release/examples/consciousness_demo.exe" 2>&1
        
        if ($LASTEXITCODE -eq 0) {
            Write-Log "✅ Basic consciousness test passed" $Green
            
            # Parse output for key metrics
            $lines = $output -split "`n"
            foreach ($line in $lines) {
                if ($line -match "Average Processing Time: (.+)") {
                    $avgTime = $matches[1]
                    Write-Log "⏱️ Average Processing Time: $avgTime" $Blue
                }
                elseif ($line -match "Average Consciousness Quality: (.+)%") {
                    $avgQuality = $matches[1]
                    Write-Log "🧠 Average Consciousness Quality: $avgQuality%" $Blue
                }
                elseif ($line -match "Current Memory Usage: (.+) MB") {
                    $memUsage = $matches[1]
                    Write-Log "💾 Memory Usage: $memUsage MB" $Blue
                }
                elseif ($line -match "Overall Score: (.+)%") {
                    $overallScore = $matches[1]
                    Write-Log "🏆 Overall Score: $overallScore%" $Blue
                }
            }
            
            return $true
        }
        else {
            Write-Log "❌ Basic consciousness test failed" $Red
            Write-Log "Output: $output" $Red
            return $false
        }
    }
    catch {
        Write-Log "❌ Error running consciousness test: $_" $Red
        return $false
    }
    finally {
        Set-Location ".."
    }
}

function Run-BenchmarkTest {
    Write-Log "📊 Running benchmark test..." $Blue
    
    Set-Location "consciousness-engine"
    
    try {
        # Run criterion benchmarks
        cargo bench --bench consciousness_benchmarks -- --output-format json > "../benchmark_results/local_benchmark_$(Get-Date -Format 'yyyyMMdd_HHmmss').json"
        
        if ($LASTEXITCODE -eq 0) {
            Write-Log "✅ Benchmark test completed" $Green
            return $true
        }
        else {
            Write-Log "❌ Benchmark test failed" $Red
            return $false
        }
    }
    catch {
        Write-Log "❌ Error running benchmark: $_" $Red
        return $false
    }
    finally {
        Set-Location ".."
    }
}

function Run-IterativeTest {
    param([int]$Count)
    
    Write-Log "🔄 Running iterative test ($Count iterations)..." $Blue
    
    $successCount = 0
    $totalTime = 0
    $results = @()
    
    for ($i = 1; $i -le $Count; $i++) {
        Write-Log "Running iteration $i/$Count..." $Yellow
        
        $startTime = Get-Date
        
        Set-Location "consciousness-engine"
        try {
            $output = & "target/release/examples/consciousness_demo.exe" --quick-test 2>&1
            
            if ($LASTEXITCODE -eq 0) {
                $successCount++
                $endTime = Get-Date
                $duration = ($endTime - $startTime).TotalMilliseconds
                $totalTime += $duration
                
                $results += [PSCustomObject]@{
                    Iteration = $i
                    Success = $true
                    Duration = $duration
                    Output = $output
                }
                
                Write-Log "✅ Iteration $i completed in $([math]::Round($duration, 2))ms" $Green
            }
            else {
                $results += [PSCustomObject]@{
                    Iteration = $i
                    Success = $false
                    Duration = 0
                    Output = $output
                }
                
                Write-Log "❌ Iteration $i failed" $Red
            }
        }
        catch {
            Write-Log "❌ Error in iteration $i: $_" $Red
        }
        finally {
            Set-Location ".."
        }
        
        # Small delay between iterations
        Start-Sleep -Milliseconds 100
    }
    
    # Summary
    Write-Log "" 
    Write-Log "📊 Iterative Test Summary:" $Blue
    Write-Log "Successful iterations: $successCount/$Count" $Blue
    Write-Log "Success rate: $([math]::Round($successCount / $Count * 100, 1))%" $Blue
    
    if ($successCount -gt 0) {
        $avgTime = $totalTime / $successCount
        Write-Log "Average execution time: $([math]::Round($avgTime, 2))ms" $Blue
    }
    
    # Save results
    $resultsFile = "benchmark_results/iterative_test_$(Get-Date -Format 'yyyyMMdd_HHmmss').json"
    $results | ConvertTo-Json | Out-File -FilePath $resultsFile -Encoding UTF8
    Write-Log "Results saved to: $resultsFile" $Blue
    
    return $successCount -eq $Count
}

function Generate-LocalReport {
    Write-Log "📋 Generating local test report..." $Blue
    
    $timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
    $reportFile = "benchmark_results/local_test_report_$timestamp.md"
    
    $reportContent = @"
# Local Consciousness Engine Test Report

**Generated:** $(Get-Date)
**Platform:** Windows PowerShell
**Mode:** $Mode
**Iterations:** $Iterations

## System Information

- **OS:** $((Get-WmiObject Win32_OperatingSystem).Caption)
- **CPU:** $((Get-WmiObject Win32_Processor).Name)
- **Memory:** $([math]::Round((Get-WmiObject Win32_ComputerSystem).TotalPhysicalMemory / 1GB, 2)) GB
- **Rust Version:** $(rustc --version 2>$null)

## Test Results

### Build Status
$(if (Test-Path "consciousness-engine/target/release/examples/consciousness_demo.exe") { "✅ Build successful" } else { "❌ Build failed" })

### Basic Functionality
$(if ($Mode -eq "basic" -or $Mode -eq "full") { "✅ Basic test completed" } else { "⏭️ Skipped" })

### Performance Benchmarks
$(if ($Mode -eq "benchmark" -or $Mode -eq "full") { "✅ Benchmark test completed" } else { "⏭️ Skipped" })

### Iterative Testing
$(if ($Mode -eq "iterative" -or $Mode -eq "full") { "✅ Iterative test completed" } else { "⏭️ Skipped" })

## Performance Targets

- **Processing Latency:** < 10ms (target)
- **Memory Usage:** < 50MB (target)
- **Consciousness Quality:** > 95% (target)
- **Success Rate:** > 99% (target)

## Next Steps

1. Review detailed test outputs
2. Identify performance bottlenecks
3. Implement optimizations
4. Run full Docker-based benchmarks

**Report completed at:** $(Get-Date)
"@

    $reportContent | Out-File -FilePath $reportFile -Encoding UTF8
    Write-Log "✅ Report generated: $reportFile" $Green
}

# Main execution
function Main {
    Write-Log "🚀 Starting Local Consciousness Engine Testing..." $Blue
    Write-Log "=============================================" $Blue
    
    # Create results directory
    if (-not (Test-Path "benchmark_results")) {
        New-Item -ItemType Directory -Path "benchmark_results" -Force | Out-Null
    }
    
    # Pre-flight checks
    if (-not (Test-RustInstallation)) {
        exit 1
    }
    
    # Build the project
    if (-not (Build-ConsciousnessEngine)) {
        exit 1
    }
    
    $allTestsPassed = $true
    
    # Run tests based on mode
    switch ($Mode) {
        "basic" {
            Write-Log "Running basic test mode..." $Yellow
            if (-not (Run-BasicTest)) {
                $allTestsPassed = $false
            }
        }
        "benchmark" {
            Write-Log "Running benchmark test mode..." $Yellow
            if (-not (Run-BenchmarkTest)) {
                $allTestsPassed = $false
            }
        }
        "iterative" {
            Write-Log "Running iterative test mode..." $Yellow
            if (-not (Run-IterativeTest $Iterations)) {
                $allTestsPassed = $false
            }
        }
        "full" {
            Write-Log "Running full test suite..." $Yellow
            if (-not (Run-BasicTest)) { $allTestsPassed = $false }
            if (-not (Run-BenchmarkTest)) { $allTestsPassed = $false }
            if (-not (Run-IterativeTest $Iterations)) { $allTestsPassed = $false }
        }
        default {
            Write-Log "Unknown mode: $Mode. Using basic mode." $Yellow
            if (-not (Run-BasicTest)) {
                $allTestsPassed = $false
            }
        }
    }
    
    # Generate report
    Generate-LocalReport
    
    # Final summary
    Write-Log ""
    if ($allTestsPassed) {
        Write-Log "🎉 All tests passed successfully!" $Green
    }
    else {
        Write-Log "⚠️ Some tests failed. Check the output above." $Yellow
    }
    
    Write-Log "📄 Local test report generated in benchmark_results/" $Blue
    Write-Log ""
    Write-Log "Next Steps:" $Yellow
    Write-Log "1. Review test results and performance metrics"
    Write-Log "2. If tests pass, run full Docker benchmarks: .\scripts\run_benchmarks.ps1"
    Write-Log "3. Implement any necessary optimizations"
    Write-Log "4. Iterate and improve performance"
}

# Run main function
Main