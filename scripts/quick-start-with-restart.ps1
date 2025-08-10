# Script de démarrage rapide avec système de redémarrage intégré
# Auteur: Consciousness Engine Team
# Version: 1.0

param(
    [switch]$SkipTests,
    [switch]$Verbose,
    [string]$Mode = "dev", # dev, static, full
    [int]$Port = 3001
)

Write-Host "🚀 Démarrage Rapide - Consciousness Engine avec Redémarrage" -ForegroundColor Cyan
Write-Host "==========================================================" -ForegroundColor Cyan

# Configuration
$webUiPath = Join-Path $PSScriptRoot "web-ui"
$startTime = Get-Date

function Write-Log {
    param([string]$Message, [string]$Level = "INFO")
    $timestamp = Get-Date -Format "HH:mm:ss"
    $color = switch ($Level) {
        "ERROR" { "Red" }
        "WARN" { "Yellow" }
        "SUCCESS" { "Green" }
        "STEP" { "Cyan" }
        default { "White" }
    }
    Write-Host "[$timestamp] [$Level] $Message" -ForegroundColor $color
}

# Étape 1: Vérifications préliminaires
Write-Log "Étape 1: Vérifications préliminaires" "STEP"

# Vérifier Node.js
try {
    $nodeVersion = node --version
    Write-Log "✅ Node.js détecté: $nodeVersion" "SUCCESS"
} catch {
    Write-Log "❌ Node.js non installé" "ERROR"
    exit 1
}

# Vérifier PowerShell
$psVersion = $PSVersionTable.PSVersion
if ($psVersion.Major -lt 5) {
    Write-Log "⚠️ PowerShell version ancienne: $psVersion" "WARN"
} else {
    Write-Log "✅ PowerShell version: $psVersion" "SUCCESS"
}

# Vérifier les dossiers
if (-not (Test-Path $webUiPath)) {
    Write-Log "❌ Dossier web-ui non trouvé: $webUiPath" "ERROR"
    exit 1
}

# Étape 2: Test du système de redémarrage (optionnel)
if (-not $SkipTests) {
    Write-Log "Étape 2: Test du système de redémarrage" "STEP"
    
    try {
        $testResult = & "$PSScriptRoot\test-restart-system.ps1" -Verbose:$Verbose
        if ($LASTEXITCODE -eq 0) {
            Write-Log "✅ Système de redémarrage opérationnel" "SUCCESS"
        } else {
            Write-Log "⚠️ Problèmes détectés dans le système de redémarrage" "WARN"
            Write-Host "Continuer malgré les problèmes? (y/N): " -NoNewline -ForegroundColor Yellow
            $response = Read-Host
            if ($response -ne "y" -and $response -ne "Y") {
                Write-Log "Démarrage annulé par l'utilisateur" "WARN"
                exit 1
            }
        }
    } catch {
        Write-Log "⚠️ Impossible de tester le système de redémarrage" "WARN"
    }
} else {
    Write-Log "Étape 2: Tests ignorés (--SkipTests)" "WARN"
}

# Étape 3: Préparation de l'environnement
Write-Log "Étape 3: Préparation de l'environnement" "STEP"

Set-Location $webUiPath

# Vérifier/installer les dépendances
if (-not (Test-Path "node_modules")) {
    Write-Log "📦 Installation des dépendances..." "INFO"
    npm install --silent
    if ($LASTEXITCODE -ne 0) {
        Write-Log "❌ Échec de l'installation des dépendances" "ERROR"
        exit 1
    }
    Write-Log "✅ Dépendances installées" "SUCCESS"
} else {
    Write-Log "✅ Dépendances déjà installées" "SUCCESS"
}

# Configuration des variables d'environnement pour stabilité
$env:FAST_REFRESH = "false"
$env:WDS_HOT = "false"
$env:WDS_LIVE_RELOAD = "false"
$env:CHOKIDAR_USEPOLLING = "true"
$env:WATCHPACK_POLLING = "true"
$env:BROWSER = "none"
$env:GENERATE_SOURCEMAP = "false"
$env:ESLINT_NO_DEV_ERRORS = "true"
$env:TSC_COMPILE_ON_ERROR = "true"
$env:PORT = $Port

Write-Log "✅ Variables d'environnement configurées" "SUCCESS"

# Étape 4: Démarrage selon le mode
Write-Log "Étape 4: Démarrage en mode $Mode" "STEP"

switch ($Mode.ToLower()) {
    "dev" {
        Write-Log "🔧 Mode développement avec redémarrage automatique" "INFO"
        Write-Log "   Local:   http://127.0.0.1:$Port" "INFO"
        Write-Log "   Accès:   http://127.0.0.1:$Port/#/login" "INFO"
        Write-Log "" "INFO"
        Write-Log "🔄 Fonctionnalités de redémarrage disponibles:" "INFO"
        Write-Log "   - Bouton de redémarrage en bas à droite" "INFO"
        Write-Log "   - Récupération automatique d'erreurs" "INFO"
        Write-Log "   - Monitoring de santé en temps réel" "INFO"
        Write-Log "   - Sauvegarde automatique d'état" "INFO"
        Write-Log "" "INFO"
        Write-Log "⚠️  Hot Reload DÉSACTIVÉ pour éviter les boucles" "WARN"
        Write-Log "   Rechargez manuellement après vos modifications" "WARN"
        Write-Log "" "INFO"
        Write-Log "Appuyez sur Ctrl+C pour arrêter" "INFO"
        
        # Démarrer React avec monitoring
        npm start
    }
    
    "static" {
        Write-Log "🏗️ Construction de l'application..." "INFO"
        npm run build
        if ($LASTEXITCODE -ne 0) {
            Write-Log "❌ Échec de la construction" "ERROR"
            exit 1
        }
        
        Write-Log "🚀 Démarrage du serveur statique sur le port $Port..." "INFO"
        Write-Log "   Local:   http://127.0.0.1:$Port" "INFO"
        Write-Log "   Accès:   http://127.0.0.1:$Port/#/login" "INFO"
        Write-Log "" "INFO"
        Write-Log "Appuyez sur Ctrl+C pour arrêter" "INFO"
        
        # Serveur HTTP simple
        $listener = New-Object System.Net.HttpListener
        $listener.Prefixes.Add("http://127.0.0.1:$Port/")
        $listener.Start()
        
        try {
            while ($listener.IsListening) {
                $context = $listener.GetContext()
                $request = $context.Request
                $response = $context.Response
                
                $path = $request.Url.AbsolutePath
                if ($path -eq "/") { $path = "/index.html" }
                
                $filePath = Join-Path "build" $path.TrimStart('/')
                
                if (Test-Path $filePath) {
                    $content = [System.IO.File]::ReadAllBytes($filePath)
                    $response.ContentLength64 = $content.Length
                    $response.OutputStream.Write($content, 0, $content.Length)
                } else {
                    # SPA fallback
                    $indexPath = Join-Path "build" "index.html"
                    if (Test-Path $indexPath) {
                        $content = [System.IO.File]::ReadAllBytes($indexPath)
                        $response.ContentLength64 = $content.Length
                        $response.OutputStream.Write($content, 0, $content.Length)
                    } else {
                        $response.StatusCode = 404
                    }
                }
                
                $response.Close()
            }
        } finally {
            $listener.Stop()
        }
    }
    
    "full" {
        Write-Log "🔄 Démarrage complet avec tous les services..." "INFO"
        
        # Démarrer les services backend
        Write-Log "Démarrage des services backend..." "INFO"
        Set-Location $PSScriptRoot
        
        try {
            & ".\restart-services.ps1" -Services @("consciousness-engine", "api-gateway") -WaitForHealthy
            if ($LASTEXITCODE -ne 0) {
                Write-Log "⚠️ Certains services n'ont pas démarré correctement" "WARN"
            }
        } catch {
            Write-Log "⚠️ Erreur lors du démarrage des services: $_" "WARN"
        }
        
        # Attendre un peu
        Start-Sleep -Seconds 3
        
        # Démarrer l'interface
        Set-Location $webUiPath
        Write-Log "Démarrage de l'interface web..." "INFO"
        Write-Log "   Local:   http://127.0.0.1:$Port" "INFO"
        Write-Log "   Accès:   http://127.0.0.1:$Port/#/login" "INFO"
        Write-Log "" "INFO"
        Write-Log "🔄 Système complet avec redémarrage automatique actif" "SUCCESS"
        Write-Log "" "INFO"
        Write-Log "Appuyez sur Ctrl+C pour arrêter" "INFO"
        
        npm start
    }
    
    default {
        Write-Log "❌ Mode non reconnu: $Mode" "ERROR"
        Write-Log "Modes disponibles: dev, static, full" "INFO"
        exit 1
    }
}

# Nettoyage à la fin
$endTime = Get-Date
$duration = $endTime - $startTime
Write-Log "Durée totale: $($duration.TotalSeconds) secondes" "INFO"
Write-Log "Arrêt du système..." "INFO"

# Retourner au répertoire original
Set-Location $PSScriptRoot
