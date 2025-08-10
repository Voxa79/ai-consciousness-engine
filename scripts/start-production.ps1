# Script de démarrage production Consciousness Engine
# Expert CTO Next Gen - Version finale

param(
    [switch]$Build,
    [switch]$Clean,
    [switch]$Test,
    [string]$Environment = "production"
)

Write-Host "🧠 CONSCIOUSNESS ENGINE - DÉMARRAGE PRODUCTION" -ForegroundColor Green
Write-Host "===============================================" -ForegroundColor Green

$ErrorActionPreference = "Stop"

# Vérifications préalables
Write-Host "🔍 Vérifications préalables..." -ForegroundColor Yellow

if (-not (Get-Command docker -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Docker n'est pas installé ou accessible" -ForegroundColor Red
    exit 1
}

if (-not (Get-Command docker-compose -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Docker Compose n'est pas installé ou accessible" -ForegroundColor Red
    exit 1
}

# Vérifier le fichier d'environnement
$envFile = ".env.production"
if (-not (Test-Path $envFile)) {
    Write-Host "❌ Fichier $envFile manquant" -ForegroundColor Red
    Write-Host "Copiez .env.production.example vers .env.production et configurez les variables" -ForegroundColor Yellow
    exit 1
}

Write-Host "✅ Vérifications préalables terminées" -ForegroundColor Green

# Nettoyage si demandé
if ($Clean) {
    Write-Host "🧹 Nettoyage des conteneurs et volumes..." -ForegroundColor Yellow
    docker-compose -f docker-compose.production.yml down -v --remove-orphans
    docker system prune -f
    Write-Host "✅ Nettoyage terminé" -ForegroundColor Green
}

# Build si demandé
if ($Build) {
    Write-Host "🔨 Construction des images..." -ForegroundColor Yellow
    docker-compose -f docker-compose.production.yml build --no-cache
    Write-Host "✅ Construction terminée" -ForegroundColor Green
}

# Arrêt des services existants
Write-Host "⏹️ Arrêt des services existants..." -ForegroundColor Yellow
docker-compose -f docker-compose.production.yml down

# Création du répertoire de sauvegarde
$backupDir = "backups\$(Get-Date -Format 'yyyyMMdd_HHmmss')"
New-Item -ItemType Directory -Path $backupDir -Force | Out-Null

# Démarrage des services de base
Write-Host "🗄️ Démarrage des services de base (PostgreSQL, Redis)..." -ForegroundColor Cyan
docker-compose -f docker-compose.production.yml up -d postgres redis

Write-Host "⏳ Attente des services de base..." -ForegroundColor Yellow
Start-Sleep -Seconds 30

# Vérification PostgreSQL
Write-Host "🔍 Vérification PostgreSQL..." -ForegroundColor Yellow
$maxAttempts = 12
$attempt = 1
do {
    try {
        docker-compose -f docker-compose.production.yml exec -T postgres pg_isready -U postgres
        $pgReady = $true
        break
    } catch {
        Write-Host "Tentative $attempt/$maxAttempts - PostgreSQL pas encore prêt..." -ForegroundColor Yellow
        Start-Sleep -Seconds 5
        $attempt++
        $pgReady = $false
    }
} while ($attempt -le $maxAttempts -and -not $pgReady)

if (-not $pgReady) {
    Write-Host "❌ PostgreSQL n'est pas accessible après $maxAttempts tentatives" -ForegroundColor Red
    exit 1
}

Write-Host "✅ PostgreSQL opérationnel" -ForegroundColor Green

# Démarrage d'Ollama
Write-Host "🤖 Démarrage d'Ollama..." -ForegroundColor Cyan
docker-compose -f docker-compose.production.yml up -d ollama

Write-Host "⏳ Attente d'Ollama..." -ForegroundColor Yellow
Start-Sleep -Seconds 60

# Téléchargement du modèle
Write-Host "📥 Téléchargement du modèle qwen2.5..." -ForegroundColor Cyan
try {
    docker-compose -f docker-compose.production.yml exec ollama ollama pull qwen2.5:3b-instruct-q4_k_m
    Write-Host "✅ Modèle téléchargé avec succès" -ForegroundColor Green
} catch {
    Write-Host "⚠️ Erreur lors du téléchargement du modèle, mais on continue..." -ForegroundColor Yellow
}

# Démarrage des services applicatifs
Write-Host "🚀 Démarrage des services applicatifs..." -ForegroundColor Cyan
docker-compose -f docker-compose.production.yml up -d consciousness-engine user-service

Write-Host "⏳ Attente des services applicatifs..." -ForegroundColor Yellow
Start-Sleep -Seconds 30

# Démarrage de l'API Gateway et Frontend
Write-Host "🌐 Démarrage API Gateway et Frontend..." -ForegroundColor Cyan
docker-compose -f docker-compose.production.yml up -d api-gateway frontend

# Démarrage du monitoring
Write-Host "📊 Démarrage du monitoring..." -ForegroundColor Cyan
docker-compose -f docker-compose.production.yml up -d prometheus grafana

# Démarrage des logs
Write-Host "📝 Démarrage de la gestion des logs..." -ForegroundColor Cyan
docker-compose -f docker-compose.production.yml up -d elasticsearch kibana

# Démarrage du backup
Write-Host "💾 Démarrage du service de backup..." -ForegroundColor Cyan
docker-compose -f docker-compose.production.yml up -d backup

# Vérifications de santé
Write-Host "🔍 Vérifications de santé des services..." -ForegroundColor Yellow

function Test-ServiceHealth {
    param($ServiceName, $Url, $MaxAttempts = 15)
    
    Write-Host "Vérification de $ServiceName..." -ForegroundColor Yellow
    
    for ($i = 1; $i -le $MaxAttempts; $i++) {
        try {
            $response = Invoke-RestMethod -Uri $Url -Method Get -TimeoutSec 10
            Write-Host "✅ $ServiceName est opérationnel" -ForegroundColor Green
            return $true
        } catch {
            Write-Host "Tentative $i/$MaxAttempts pour $ServiceName..." -ForegroundColor Yellow
            Start-Sleep -Seconds 10
        }
    }
    
    Write-Host "❌ $ServiceName n'est pas accessible après $MaxAttempts tentatives" -ForegroundColor Red
    return $false
}

# Tests de santé
$services = @(
    @{Name="Consciousness Engine"; Url="http://localhost:8080/health"},
    @{Name="User Service"; Url="http://localhost:8081/health"},
    @{Name="API Gateway"; Url="http://localhost:3000/health"},
    @{Name="Frontend"; Url="http://localhost:3001"},
    @{Name="Prometheus"; Url="http://localhost:9090/-/healthy"},
    @{Name="Grafana"; Url="http://localhost:3002/api/health"}
)

$allHealthy = $true
foreach ($service in $services) {
    if (-not (Test-ServiceHealth -ServiceName $service.Name -Url $service.Url)) {
        $allHealthy = $false
    }
}

# Test de l'API complète
if ($allHealthy) {
    Write-Host "🧪 Test de l'API complète..." -ForegroundColor Yellow
    
    try {
        $testPayload = @{
            content = "Test de démarrage production"
            user_id = "production_test_user"
            context = @{
                test = $true
                environment = "production_startup"
            }
        } | ConvertTo-Json
        
        $response = Invoke-RestMethod -Uri "http://localhost:3000/api/v1/consciousness/process" `
                                    -Method Post `
                                    -ContentType "application/json" `
                                    -Body $testPayload `
                                    -TimeoutSec 30
        
        if ($response.content) {
            Write-Host "✅ Test API réussi" -ForegroundColor Green
        } else {
            Write-Host "❌ Test API échoué - Pas de contenu dans la réponse" -ForegroundColor Red
            $allHealthy = $false
        }
    } catch {
        Write-Host "❌ Test API échoué: $($_.Exception.Message)" -ForegroundColor Red
        $allHealthy = $false
    }
}

# Tests d'intégration si demandé
if ($Test -and $allHealthy) {
    Write-Host "🧪 Exécution des tests d'intégration..." -ForegroundColor Cyan
    
    if (Test-Path "tests/integration_tests.rs") {
        try {
            cargo test --test integration_tests
            Write-Host "✅ Tests d'intégration réussis" -ForegroundColor Green
        } catch {
            Write-Host "❌ Tests d'intégration échoués" -ForegroundColor Red
            $allHealthy = $false
        }
    } else {
        Write-Host "⚠️ Fichiers de tests d'intégration non trouvés" -ForegroundColor Yellow
    }
}

# Affichage du statut final
Write-Host ""
if ($allHealthy) {
    Write-Host "🎉 DÉMARRAGE PRODUCTION RÉUSSI!" -ForegroundColor Green
} else {
    Write-Host "⚠️ DÉMARRAGE AVEC ERREURS" -ForegroundColor Yellow
}

Write-Host "=================================" -ForegroundColor Green
Write-Host ""
Write-Host "📱 URLs d'accès:" -ForegroundColor White
Write-Host "   🌐 Interface principale: http://localhost:3001" -ForegroundColor Cyan
Write-Host "   🔌 API Gateway:          http://localhost:3000" -ForegroundColor Cyan
Write-Host "   📊 Grafana:              http://localhost:3002" -ForegroundColor Cyan
Write-Host "   📈 Prometheus:           http://localhost:9090" -ForegroundColor Cyan
Write-Host "   📝 Kibana:               http://localhost:5601" -ForegroundColor Cyan
Write-Host ""
Write-Host "🔧 Commandes utiles:" -ForegroundColor White
Write-Host "   Logs:   docker-compose -f docker-compose.production.yml logs -f" -ForegroundColor Yellow
Write-Host "   Status: docker-compose -f docker-compose.production.yml ps" -ForegroundColor Yellow
Write-Host "   Stop:   docker-compose -f docker-compose.production.yml down" -ForegroundColor Yellow
Write-Host ""
Write-Host "💾 Sauvegarde: $backupDir" -ForegroundColor White
Write-Host ""

if ($allHealthy) {
    Write-Host "🚀 CONSCIOUSNESS ENGINE EST MAINTENANT EN PRODUCTION!" -ForegroundColor Green
    Write-Host "Tous les services sont opérationnels et prêts à recevoir du trafic." -ForegroundColor Green
} else {
    Write-Host "⚠️ Certains services ont des problèmes. Vérifiez les logs." -ForegroundColor Yellow
    Write-Host "Utilisez: docker-compose -f docker-compose.production.yml logs" -ForegroundColor Yellow
}

Write-Host ""
