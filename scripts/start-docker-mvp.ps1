#!/usr/bin/env pwsh

# Start Consciousness Engine MVP with Docker
Write-Host "🐳 Starting Consciousness Engine MVP with Docker" -ForegroundColor Cyan
Write-Host "=================================================" -ForegroundColor Cyan

# Check if Docker is running
try {
    docker version | Out-Null
    Write-Host "✅ Docker is running" -ForegroundColor Green
} catch {
    Write-Host "❌ Docker is not running. Please start Docker Desktop first." -ForegroundColor Red
    Write-Host "💡 Start Docker Desktop and try again." -ForegroundColor Yellow
    exit 1
}

# Check if docker-compose is available
try {
    docker-compose version | Out-Null
    Write-Host "✅ Docker Compose is available" -ForegroundColor Green
} catch {
    Write-Host "❌ Docker Compose is not available" -ForegroundColor Red
    exit 1
}

Write-Host "`n🏗️  Building and starting services..." -ForegroundColor Yellow
Write-Host "This may take a few minutes on first run..." -ForegroundColor Gray

try {
    # Build and start services
    docker-compose -f docker-compose.mvp.yml up --build -d
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "`n🎉 Services started successfully!" -ForegroundColor Green
        Write-Host "=================================" -ForegroundColor Green
        
        Write-Host "`n📡 Service URLs:" -ForegroundColor Cyan
        Write-Host "• Web Interface: http://localhost:3001" -ForegroundColor White
        Write-Host "• API Gateway: http://localhost:3002" -ForegroundColor White
        Write-Host "• Consciousness Engine: http://localhost:8080" -ForegroundColor White
        
        Write-Host "`n🔍 Health Checks:" -ForegroundColor Cyan
        Write-Host "• API Gateway Health: http://localhost:3002/health" -ForegroundColor Gray
        Write-Host "• Consciousness Health: http://localhost:8080/health" -ForegroundColor Gray
        
        Write-Host "`n📊 Monitoring:" -ForegroundColor Cyan
        Write-Host "docker-compose -f docker-compose.mvp.yml logs -f" -ForegroundColor Gray
        
        Write-Host "`n🧪 Testing:" -ForegroundColor Cyan
        Write-Host "1. Open http://localhost:3001 in your browser" -ForegroundColor White
        Write-Host "2. Try the consciousness chat interface" -ForegroundColor White
        Write-Host "3. Test API endpoints with curl or Postman" -ForegroundColor White
        
        Write-Host "`n🛑 To stop services:" -ForegroundColor Yellow
        Write-Host "docker-compose -f docker-compose.mvp.yml down" -ForegroundColor Gray
        
        # Wait for services to be healthy
        Write-Host "`n⏳ Waiting for services to be ready..." -ForegroundColor Yellow
        
        $maxWait = 120 # 2 minutes
        $waited = 0
        $allHealthy = $false
        
        while ($waited -lt $maxWait -and -not $allHealthy) {
            Start-Sleep -Seconds 5
            $waited += 5
            
            try {
                # Check service health
                $gatewayHealth = Invoke-RestMethod -Uri "http://localhost:3002/health" -TimeoutSec 5 -ErrorAction SilentlyContinue
                $engineHealth = Invoke-RestMethod -Uri "http://localhost:8080/health" -TimeoutSec 5 -ErrorAction SilentlyContinue
                $webHealth = Invoke-WebRequest -Uri "http://localhost:3001" -TimeoutSec 5 -ErrorAction SilentlyContinue
                
                if ($gatewayHealth -and $engineHealth -and $webHealth) {
                    $allHealthy = $true
                    Write-Host "✅ All services are healthy and ready!" -ForegroundColor Green
                    break
                }
            } catch {
                Write-Host "⏳ Services still starting... ($waited/$maxWait seconds)" -ForegroundColor Yellow
            }
        }
        
        if (-not $allHealthy) {
            Write-Host "⚠️  Services may still be starting. Check logs if needed:" -ForegroundColor Yellow
            Write-Host "docker-compose -f docker-compose.mvp.yml logs" -ForegroundColor Gray
        }
        
        Write-Host "`n🚀 MVP is ready! Open http://localhost:3001 to start testing!" -ForegroundColor Magenta
        
    } else {
        Write-Host "`n❌ Failed to start services" -ForegroundColor Red
        Write-Host "Check the logs for more details:" -ForegroundColor Yellow
        Write-Host "docker-compose -f docker-compose.mvp.yml logs" -ForegroundColor Gray
    }
    
} catch {
    Write-Host "`n❌ Error starting services: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host "💡 Try running: docker-compose -f docker-compose.mvp.yml logs" -ForegroundColor Yellow
}

Write-Host "`nPress any key to view logs (Ctrl+C to exit logs)..." -ForegroundColor Gray
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")

# Show logs
docker-compose -f docker-compose.mvp.yml logs -f