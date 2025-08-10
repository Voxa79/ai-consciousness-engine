# Script de test pour valider la stabilité de l'interface web
# Auteur: Consciousness Engine Team
# Version: 1.0

param(
    [int]$Duration = 30,  # Durée du test en secondes
    [switch]$Verbose
)

Write-Host "🧪 Test de Stabilité de l'Interface Web" -ForegroundColor Cyan
Write-Host "=======================================" -ForegroundColor Cyan

$webUiPath = Join-Path $PSScriptRoot "web-ui"
if (-not (Test-Path $webUiPath)) {
    Write-Host "❌ Dossier web-ui non trouvé" -ForegroundColor Red
    exit 1
}

Set-Location $webUiPath

# Vérifier les fichiers de configuration
$configFiles = @(
    ".env.local",
    ".env.development.local",
    "src/contexts/ConsciousnessContext.tsx",
    "src/hooks/useEventSource.ts",
    "src/components/Debug/RenderDiagnostic.tsx"
)

Write-Host "🔍 Vérification des fichiers de configuration..." -ForegroundColor Blue

foreach ($file in $configFiles) {
    if (Test-Path $file) {
        Write-Host "✅ $file" -ForegroundColor Green
    } else {
        Write-Host "❌ $file manquant" -ForegroundColor Red
    }
}

# Vérifier les dépendances
Write-Host "`n📦 Vérification des dépendances..." -ForegroundColor Blue
if (-not (Test-Path "node_modules")) {
    Write-Host "⚠️  Installation des dépendances..." -ForegroundColor Yellow
    npm install --silent
}

# Vérifier la configuration .env.local
Write-Host "`n⚙️  Vérification de la configuration..." -ForegroundColor Blue
if (Test-Path ".env.local") {
    $envContent = Get-Content ".env.local" -Raw
    $requiredSettings = @(
        "FAST_REFRESH=false",
        "WDS_HOT=false",
        "WDS_LIVE_RELOAD=false"
    )
    
    foreach ($setting in $requiredSettings) {
        if ($envContent -match [regex]::Escape($setting)) {
            Write-Host "✅ $setting" -ForegroundColor Green
        } else {
            Write-Host "❌ $setting manquant" -ForegroundColor Red
        }
    }
} else {
    Write-Host "❌ Fichier .env.local manquant" -ForegroundColor Red
}

# Test de compilation
Write-Host "`n🏗️  Test de compilation..." -ForegroundColor Blue
$env:GENERATE_SOURCEMAP = "false"
$env:ESLINT_NO_DEV_ERRORS = "true"

$buildOutput = npm run build 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Compilation réussie" -ForegroundColor Green
} else {
    Write-Host "❌ Échec de compilation" -ForegroundColor Red
    if ($Verbose) {
        Write-Host $buildOutput -ForegroundColor Yellow
    }
}

# Test de démarrage du serveur de développement
Write-Host "`n🚀 Test de démarrage du serveur..." -ForegroundColor Blue

# Variables d'environnement pour stabilité
$env:FAST_REFRESH = "false"
$env:WDS_HOT = "false"
$env:WDS_LIVE_RELOAD = "false"
$env:CHOKIDAR_USEPOLLING = "true"
$env:WATCHPACK_POLLING = "true"
$env:BROWSER = "none"
$env:PORT = "3001"

# Démarrer le serveur en arrière-plan
$serverJob = Start-Job -ScriptBlock {
    param($webUiPath)
    Set-Location $webUiPath
    
    $env:FAST_REFRESH = "false"
    $env:WDS_HOT = "false"
    $env:WDS_LIVE_RELOAD = "false"
    $env:CHOKIDAR_USEPOLLING = "true"
    $env:WATCHPACK_POLLING = "true"
    $env:BROWSER = "none"
    $env:PORT = "3001"
    
    npm start 2>&1
} -ArgumentList $webUiPath

Write-Host "⏳ Attente du démarrage du serveur..." -ForegroundColor Yellow
Start-Sleep -Seconds 10

# Tester la connectivité
$testUrl = "http://127.0.0.1:3001"
$attempts = 0
$maxAttempts = 5
$serverReady = $false

while ($attempts -lt $maxAttempts -and -not $serverReady) {
    try {
        $response = Invoke-WebRequest -Uri $testUrl -TimeoutSec 5 -UseBasicParsing
        if ($response.StatusCode -eq 200) {
            $serverReady = $true
            Write-Host "✅ Serveur accessible sur $testUrl" -ForegroundColor Green
        }
    } catch {
        $attempts++
        Write-Host "⏳ Tentative $attempts/$maxAttempts..." -ForegroundColor Yellow
        Start-Sleep -Seconds 2
    }
}

if (-not $serverReady) {
    Write-Host "❌ Serveur non accessible après $maxAttempts tentatives" -ForegroundColor Red
    Stop-Job $serverJob -Force
    Remove-Job $serverJob -Force
    exit 1
}

# Test de stabilité
Write-Host "`n🔄 Test de stabilité pendant $Duration secondes..." -ForegroundColor Blue
$startTime = Get-Date
$endTime = $startTime.AddSeconds($Duration)
$requestCount = 0
$errorCount = 0

while ((Get-Date) -lt $endTime) {
    try {
        $response = Invoke-WebRequest -Uri $testUrl -TimeoutSec 3 -UseBasicParsing
        if ($response.StatusCode -eq 200) {
            $requestCount++
            if ($Verbose) {
                Write-Host "." -NoNewline -ForegroundColor Green
            }
        }
    } catch {
        $errorCount++
        if ($Verbose) {
            Write-Host "X" -NoNewline -ForegroundColor Red
        }
    }
    
    Start-Sleep -Seconds 1
}

# Arrêter le serveur
Stop-Job $serverJob -Force
Remove-Job $serverJob -Force

# Résultats
Write-Host "`n`n📊 Résultats du Test" -ForegroundColor Cyan
Write-Host "===================" -ForegroundColor Cyan
Write-Host "Durée: $Duration secondes" -ForegroundColor White
Write-Host "Requêtes réussies: $requestCount" -ForegroundColor Green
Write-Host "Erreurs: $errorCount" -ForegroundColor Red

$successRate = if ($requestCount + $errorCount -gt 0) { 
    [math]::Round(($requestCount / ($requestCount + $errorCount)) * 100, 2) 
} else { 
    0 
}

Write-Host "Taux de succès: $successRate%" -ForegroundColor $(if ($successRate -gt 90) { "Green" } elseif ($successRate -gt 70) { "Yellow" } else { "Red" })

if ($successRate -gt 90) {
    Write-Host "`n🎉 Test RÉUSSI - Interface stable!" -ForegroundColor Green
    exit 0
} elseif ($successRate -gt 70) {
    Write-Host "`n⚠️  Test PARTIEL - Quelques problèmes détectés" -ForegroundColor Yellow
    exit 1
} else {
    Write-Host "`n❌ Test ÉCHOUÉ - Interface instable" -ForegroundColor Red
    exit 2
}
