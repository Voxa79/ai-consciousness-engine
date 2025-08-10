#!/usr/bin/env pwsh

# Start MVP - Consciousness Engine Platform
# This script starts all components needed for the MVP

Write-Host "🚀 Starting Consciousness Engine MVP" -ForegroundColor Cyan
Write-Host "===================================" -ForegroundColor Cyan

# Set error action preference
$ErrorActionPreference = "Stop"

# Function to check if a port is available
function Test-Port {
    param([int]$Port)
    try {
        $listener = [System.Net.Sockets.TcpListener]::new([System.Net.IPAddress]::Any, $Port)
        $listener.Start()
        $listener.Stop()
        return $true
    } catch {
        return $false
    }
}

# Function to start a service in background
function Start-Service {
    param(
        [string]$Name,
        [string]$Command,
        [string]$WorkingDirectory,
        [int]$Port,
        [string]$HealthEndpoint
    )
    
    Write-Host "🔧 Starting $Name..." -ForegroundColor Yellow
    
    if (-not (Test-Port -Port $Port)) {
        Write-Host "⚠️  Port $Port is already in use. Skipping $Name." -ForegroundColor Yellow
        return
    }
    
    try {
        $process = Start-Process -FilePath "pwsh" -ArgumentList "-Command", $Command -WorkingDirectory $WorkingDirectory -PassThru -WindowStyle Hidden
        
        # Wait a moment for the service to start
        Start-Sleep -Seconds 3
        
        # Check if service is healthy
        $maxRetries = 10
        $retries = 0
        $healthy = $false
        
        while ($retries -lt $maxRetries -and -not $healthy) {
            try {
                $response = Invoke-RestMethod -Uri $HealthEndpoint -Method Get -TimeoutSec 5
                if ($response) {
                    $healthy = $true
                    Write-Host "✅ $Name started successfully on port $Port" -ForegroundColor Green
                }
            } catch {
                $retries++
                Start-Sleep -Seconds 2
            }
        }
        
        if (-not $healthy) {
            Write-Host "❌ $Name failed to start or is not responding" -ForegroundColor Red
            if ($process -and -not $process.HasExited) {
                $process.Kill()
            }
        }
        
        return $process
    } catch {
        Write-Host "❌ Failed to start $Name : $($_.Exception.Message)" -ForegroundColor Red
        return $null
    }
}

Write-Host "`n📋 Checking prerequisites..." -ForegroundColor Yellow

# Check if required tools are installed
$tools = @("cargo", "node", "npm")
foreach ($tool in $tools) {
    try {
        & $tool --version | Out-Null
        Write-Host "✅ $tool is installed" -ForegroundColor Green
    } catch {
        Write-Host "❌ $tool is not installed or not in PATH" -ForegroundColor Red
        Write-Host "Please install $tool and try again." -ForegroundColor Yellow
        exit 1
    }
}

Write-Host "`n🏗️  Building components..." -ForegroundColor Yellow

# Build Consciousness Engine
Write-Host "🧠 Building Consciousness Engine..." -ForegroundColor Cyan
try {
    Set-Location "consciousness-engine"
    cargo build --release --bin consciousness-server
    if ($LASTEXITCODE -ne 0) {
        throw "Consciousness Engine build failed"
    }
    Set-Location ".."
    Write-Host "✅ Consciousness Engine built successfully" -ForegroundColor Green
} catch {
    Write-Host "❌ Failed to build Consciousness Engine: $($_.Exception.Message)" -ForegroundColor Red
    Set-Location ".."
    exit 1
}

# Build API Gateway
Write-Host "🌐 Building API Gateway..." -ForegroundColor Cyan
try {
    Set-Location "api-gateway"
    cargo build --release
    if ($LASTEXITCODE -ne 0) {
        throw "API Gateway build failed"
    }
    Set-Location ".."
    Write-Host "✅ API Gateway built successfully" -ForegroundColor Green
} catch {
    Write-Host "❌ Failed to build API Gateway: $($_.Exception.Message)" -ForegroundColor Red
    Set-Location ".."
    exit 1
}

# Build Web UI
Write-Host "🎨 Building Web UI..." -ForegroundColor Cyan
try {
    Set-Location "web-ui"
    npm install
    if ($LASTEXITCODE -ne 0) {
        throw "Web UI dependencies installation failed"
    }
    npm run build
    if ($LASTEXITCODE -ne 0) {
        throw "Web UI build failed"
    }
    Set-Location ".."
    Write-Host "✅ Web UI built successfully" -ForegroundColor Green
} catch {
    Write-Host "❌ Failed to build Web UI: $($_.Exception.Message)" -ForegroundColor Red
    Set-Location ".."
    exit 1
}

Write-Host "`n🚀 Starting services..." -ForegroundColor Yellow

# Start services
$processes = @()

# Start Consciousness Engine
$consciousnessProcess = Start-Service -Name "Consciousness Engine" -Command "cargo run --release --bin consciousness-server" -WorkingDirectory "consciousness-engine" -Port 8080 -HealthEndpoint "http://localhost:8080/health"
if ($consciousnessProcess) {
    $processes += $consciousnessProcess
}

# Start API Gateway
$gatewayProcess = Start-Service -Name "API Gateway" -Command "cargo run --release" -WorkingDirectory "api-gateway" -Port 3000 -HealthEndpoint "http://localhost:3000/health"
if ($gatewayProcess) {
    $processes += $gatewayProcess
}

# Start Web UI
$webProcess = Start-Service -Name "Web UI" -Command "npm start" -WorkingDirectory "web-ui" -Port 3001 -HealthEndpoint "http://localhost:3001"
if ($webProcess) {
    $processes += $webProcess
}

Write-Host "`n🎉 MVP Started Successfully!" -ForegroundColor Green
Write-Host "=========================" -ForegroundColor Green

Write-Host "`n📡 Service Endpoints:" -ForegroundColor Cyan
Write-Host "• Consciousness Engine: http://localhost:8080" -ForegroundColor White
Write-Host "  - Health: http://localhost:8080/health" -ForegroundColor Gray
Write-Host "  - Process: http://localhost:8080/consciousness/process" -ForegroundColor Gray

Write-Host "• API Gateway: http://localhost:3000" -ForegroundColor White
Write-Host "  - Health: http://localhost:3000/health" -ForegroundColor Gray
Write-Host "  - API: http://localhost:3000/api/v1/" -ForegroundColor Gray

Write-Host "• Web UI: http://localhost:3001" -ForegroundColor White
Write-Host "  - Main Interface: http://localhost:3001" -ForegroundColor Gray

Write-Host "`n🧪 Testing the MVP:" -ForegroundColor Cyan
Write-Host "1. Open http://localhost:3001 in your browser" -ForegroundColor White
Write-Host "2. Try the consciousness chat interface" -ForegroundColor White
Write-Host "3. Test self-reflection and growth opportunities" -ForegroundColor White
Write-Host "4. Monitor consciousness metrics in real-time" -ForegroundColor White

Write-Host "`n📊 API Testing Examples:" -ForegroundColor Cyan
Write-Host "# Test consciousness processing" -ForegroundColor Gray
Write-Host 'curl -X POST http://localhost:3000/api/v1/consciousness/process \' -ForegroundColor White
Write-Host '  -H "Content-Type: application/json" \' -ForegroundColor White
Write-Host '  -d ''{"content":"Hello, how are you?","user_id":"test"}''' -ForegroundColor White

Write-Host "`n# Get consciousness state" -ForegroundColor Gray
Write-Host 'curl http://localhost:3000/api/v1/consciousness/state' -ForegroundColor White

Write-Host "`n# Generate self-reflection" -ForegroundColor Gray
Write-Host 'curl -X POST http://localhost:3000/api/v1/consciousness/reflection' -ForegroundColor White

Write-Host "`n⚠️  Important Notes:" -ForegroundColor Yellow
Write-Host "• This is an MVP with basic functionality" -ForegroundColor White
Write-Host "• Some advanced features may use mock implementations" -ForegroundColor White
Write-Host "• Monitor logs for any issues or errors" -ForegroundColor White
Write-Host "• Press Ctrl+C to stop all services" -ForegroundColor White

Write-Host "`n🔧 Troubleshooting:" -ForegroundColor Yellow
Write-Host "• If services fail to start, check if ports are available" -ForegroundColor White
Write-Host "• Check individual service logs for detailed error information" -ForegroundColor White
Write-Host "• Ensure all dependencies are properly installed" -ForegroundColor White

# Wait for user input to stop services
Write-Host "`nPress any key to stop all services..." -ForegroundColor Magenta
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")

Write-Host "`n🛑 Stopping services..." -ForegroundColor Yellow

# Stop all processes
foreach ($process in $processes) {
    if ($process -and -not $process.HasExited) {
        try {
            $process.Kill()
            $process.WaitForExit(5000)
            Write-Host "✅ Service stopped" -ForegroundColor Green
        } catch {
            Write-Host "⚠️  Force killing service" -ForegroundColor Yellow
        }
    }
}

Write-Host "`n👋 MVP stopped. Thank you for testing!" -ForegroundColor Cyan