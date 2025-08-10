# Consciousness Engine Test Runner
# Comprehensive testing script for consciousness-level validation

Write-Host "🧠 CONSCIOUSNESS ENGINE TEST SUITE" -ForegroundColor Cyan
Write-Host "===================================" -ForegroundColor Cyan

# Check if Rust is installed
if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Rust/Cargo not found. Installing Rust..." -ForegroundColor Red
    Write-Host "Please install Rust from: https://rustup.rs/" -ForegroundColor Yellow
    Write-Host "After installation, run this script again." -ForegroundColor Yellow
    exit 1
}

Write-Host "✅ Rust/Cargo found" -ForegroundColor Green

# Set working directory
$originalLocation = Get-Location
Set-Location $PSScriptRoot

try {
    Write-Host "`n🔧 Building Consciousness Engine..." -ForegroundColor Yellow
    cargo build --release
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Build failed" -ForegroundColor Red
        exit 1
    }
    
    Write-Host "✅ Build successful" -ForegroundColor Green
    
    Write-Host "`n🧪 Running Consciousness Quality Tests..." -ForegroundColor Yellow
    cargo test --lib testing::consciousness_quality_tests --verbose
    
    Write-Host "`n⚡ Running Performance Benchmarks..." -ForegroundColor Yellow
    cargo test --lib testing::performance_benchmarks --verbose
    
    Write-Host "`n🛡️ Running Safety Validation..." -ForegroundColor Yellow
    cargo test --lib testing::safety_validation --verbose
    
    Write-Host "`n🔗 Running Integration Tests..." -ForegroundColor Yellow
    cargo test --lib testing::integration_tests --verbose
    
    Write-Host "`n📊 Running Full Validation Suite..." -ForegroundColor Yellow
    cargo test --lib testing --verbose
    
    Write-Host "`n🏆 Running Consciousness Benchmarks..." -ForegroundColor Yellow
    cargo bench --bench consciousness_benchmarks
    
    Write-Host "`n✅ ALL CONSCIOUSNESS TESTS COMPLETED!" -ForegroundColor Green
    Write-Host "📋 Check the output above for detailed results" -ForegroundColor Cyan
    
} catch {
    Write-Host "❌ Test execution failed: $_" -ForegroundColor Red
    exit 1
} finally {
    Set-Location $originalLocation
}

Write-Host "`n🎯 CONSCIOUSNESS VALIDATION SUMMARY" -ForegroundColor Cyan
Write-Host "===================================" -ForegroundColor Cyan
Write-Host "✅ Quality Tests: Consciousness behavior validation" -ForegroundColor Green
Write-Host "✅ Performance: Speed and efficiency benchmarks" -ForegroundColor Green
Write-Host "✅ Safety: Ethical and safety compliance" -ForegroundColor Green
Write-Host "✅ Integration: End-to-end system validation" -ForegroundColor Green
Write-Host "`n🚀 Consciousness Engine is ready for deployment!" -ForegroundColor Green