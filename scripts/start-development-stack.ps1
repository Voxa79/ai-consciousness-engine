# SCRIPT DE DÉMARRAGE STACK DÉVELOPPEMENT COMPLÈTE
# Expert CTO Next Gen - Environnement de développement avec tous les services

param(
    [switch]$Docker = $false,
    [switch]$Local = $true,
    [switch]$MockOnly = $false,
    [switch]$Clean = $false,
    [int]$UIPort = 3001,
    [int]$APIPort = 3000,
    [int]$EnginePort = 8080
)

Write-Host "🚀 DÉMARRAGE STACK DÉVELOPPEMENT CONSCIOUSNESS" -ForegroundColor Green
Write-Host "=============================================" -ForegroundColor Green

# Nettoyage si demandé
if ($Clean) {
    Write-Host "🧹 Nettoyage de l'environnement..." -ForegroundColor Yellow
    
    # Arrêter tous les processus Node
    Get-Process -Name "node" -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue
    
    # Nettoyer Docker si utilisé
    if ($Docker) {
        docker-compose -f docker-compose.dev-ui.yml down -v --remove-orphans 2>$null
        docker system prune -f 2>$null
    }
    
    Write-Host "✅ Nettoyage terminé" -ForegroundColor Green
}

# Fonction pour vérifier si un port est libre
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

# Vérifier les ports
$ports = @($UIPort, $APIPort, $EnginePort)
foreach ($port in $ports) {
    if (-not (Test-Port $port)) {
        Write-Host "⚠️ Port $port est déjà utilisé" -ForegroundColor Yellow
        $process = Get-NetTCPConnection -LocalPort $port -ErrorAction SilentlyContinue
        if ($process) {
            Write-Host "   Processus utilisant le port: PID $($process.OwningProcess)" -ForegroundColor Gray
        }
    }
}

if ($Docker) {
    Write-Host "🐳 DÉMARRAGE AVEC DOCKER" -ForegroundColor Cyan
    Write-Host "========================" -ForegroundColor Cyan
    
    # Vérifier que Docker est disponible
    try {
        docker --version | Out-Null
        docker-compose --version | Out-Null
    } catch {
        Write-Host "❌ Docker ou Docker Compose non disponible" -ForegroundColor Red
        Write-Host "   Veuillez installer Docker Desktop" -ForegroundColor Yellow
        exit 1
    }
    
    # Construire et démarrer les services
    Write-Host "🔨 Construction des images Docker..." -ForegroundColor Cyan
    docker-compose -f docker-compose.dev-ui.yml build
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Échec de la construction Docker" -ForegroundColor Red
        exit 1
    }
    
    Write-Host "🚀 Démarrage des services Docker..." -ForegroundColor Cyan
    docker-compose -f docker-compose.dev-ui.yml up -d
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Échec du démarrage Docker" -ForegroundColor Red
        exit 1
    }
    
    # Attendre que les services soient prêts
    Write-Host "⏳ Attente de la disponibilité des services..." -ForegroundColor Yellow
    
    $maxWait = 120
    $waited = 0
    $allReady = $false
    
    while ($waited -lt $maxWait -and -not $allReady) {
        Start-Sleep -Seconds 5
        $waited += 5
        
        try {
            $healthChecks = @(
                (Invoke-RestMethod -Uri "http://localhost:$EnginePort/health" -TimeoutSec 2),
                (Invoke-RestMethod -Uri "http://localhost:$APIPort/health" -TimeoutSec 2)
            )
            
            if ($healthChecks.Count -eq 2) {
                $allReady = $true
                Write-Host "✅ Tous les services sont prêts" -ForegroundColor Green
            }
        } catch {
            Write-Host "." -NoNewline -ForegroundColor Yellow
        }
    }
    
    if (-not $allReady) {
        Write-Host "⚠️ Certains services mettent du temps à démarrer" -ForegroundColor Yellow
        Write-Host "   Vérifiez les logs avec: docker-compose -f docker-compose.dev-ui.yml logs" -ForegroundColor Gray
    }
    
} else {
    Write-Host "💻 DÉMARRAGE LOCAL" -ForegroundColor Cyan
    Write-Host "==================" -ForegroundColor Cyan
    
    # Vérifier Node.js
    try {
        $nodeVersion = node --version
        Write-Host "✅ Node.js détecté: $nodeVersion" -ForegroundColor Green
    } catch {
        Write-Host "❌ Node.js non disponible" -ForegroundColor Red
        exit 1
    }
    
    # Démarrer Mock Consciousness Engine
    Write-Host "🤖 Démarrage Mock Consciousness Engine..." -ForegroundColor Cyan
    
    Set-Location "mock-services"
    
    # Installer les dépendances si nécessaire
    if (-not (Test-Path "node_modules")) {
        Write-Host "📦 Installation des dépendances mock..." -ForegroundColor Yellow
        npm init -y | Out-Null
        npm install express cors http-proxy-middleware | Out-Null
    }
    
    # Démarrer le mock engine
    $env:PORT = $EnginePort
    Start-Process -FilePath "node" -ArgumentList "mock-consciousness-api.js" -WindowStyle Hidden
    
    Start-Sleep -Seconds 3
    
    # Démarrer Mock API Gateway
    Write-Host "🌐 Démarrage Mock API Gateway..." -ForegroundColor Cyan
    $env:PORT = $APIPort
    $env:CONSCIOUSNESS_ENGINE_URL = "http://localhost:$EnginePort"
    Start-Process -FilePath "node" -ArgumentList "mock-api-gateway.js" -WindowStyle Hidden
    
    Start-Sleep -Seconds 3
    
    Set-Location ".."
    
    # Démarrer l'interface UI
    if (-not $MockOnly) {
        Write-Host "🎨 Démarrage Interface UI..." -ForegroundColor Cyan
        
        # Utiliser le script de production UI avec mock API
        $env:REACT_APP_API_URL = "http://localhost:$APIPort/api/v1"
        .\start-production-ui.ps1 -Port $UIPort -ApiUrl "http://localhost:$APIPort/api/v1"
    }
}

# Affichage des URLs d'accès
Write-Host ""
Write-Host "🎯 SERVICES DISPONIBLES" -ForegroundColor Green
Write-Host "========================" -ForegroundColor Green
Write-Host "🧠 Consciousness Engine: http://localhost:$EnginePort" -ForegroundColor Cyan
Write-Host "🌐 API Gateway:          http://localhost:$APIPort" -ForegroundColor Cyan

if (-not $MockOnly) {
    Write-Host "🎨 Interface Web UI:     http://localhost:$UIPort" -ForegroundColor Cyan
}

Write-Host ""
Write-Host "📊 ENDPOINTS DE TEST" -ForegroundColor Yellow
Write-Host "===================" -ForegroundColor Yellow
Write-Host "Health Checks:" -ForegroundColor White
Write-Host "  - GET http://localhost:$EnginePort/health" -ForegroundColor Gray
Write-Host "  - GET http://localhost:$APIPort/health" -ForegroundColor Gray
Write-Host ""
Write-Host "Consciousness API:" -ForegroundColor White
Write-Host "  - POST http://localhost:$APIPort/api/v1/consciousness/process" -ForegroundColor Gray
Write-Host "  - GET  http://localhost:$APIPort/api/v1/consciousness/state" -ForegroundColor Gray
Write-Host ""
Write-Host "Autres Services:" -ForegroundColor White
Write-Host "  - GET  http://localhost:$APIPort/api/v1/agents" -ForegroundColor Gray
Write-Host "  - GET  http://localhost:$APIPort/api/v1/governance/policies" -ForegroundColor Gray
Write-Host "  - GET  http://localhost:$APIPort/api/v1/metrics" -ForegroundColor Gray

Write-Host ""
Write-Host "🔧 COMMANDES UTILES" -ForegroundColor Yellow
Write-Host "==================" -ForegroundColor Yellow

if ($Docker) {
    Write-Host "Voir les logs:     docker-compose -f docker-compose.dev-ui.yml logs -f" -ForegroundColor Gray
    Write-Host "Arrêter:          docker-compose -f docker-compose.dev-ui.yml down" -ForegroundColor Gray
    Write-Host "Redémarrer:       docker-compose -f docker-compose.dev-ui.yml restart" -ForegroundColor Gray
} else {
    Write-Host "Arrêter services: Get-Process -Name 'node' | Stop-Process" -ForegroundColor Gray
    Write-Host "Redémarrer:       .\start-development-stack.ps1" -ForegroundColor Gray
}

Write-Host ""
Write-Host "✅ Stack de développement prête !" -ForegroundColor Green
Write-Host "Appuyez sur Ctrl+C pour arrêter" -ForegroundColor Yellow

# Attendre l'interruption utilisateur si en mode local
if (-not $Docker -and -not $MockOnly) {
    try {
        while ($true) {
            Start-Sleep -Seconds 1
        }
    } finally {
        Write-Host ""
        Write-Host "🛑 Arrêt des services..." -ForegroundColor Yellow
        Get-Process -Name "node" -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue
        Write-Host "✅ Services arrêtés" -ForegroundColor Green
    }
}
