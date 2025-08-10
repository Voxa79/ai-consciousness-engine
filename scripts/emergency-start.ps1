# SCRIPT D'URGENCE - MODE CTO DIAGNOSTIC
# Expert formateur CTO Next Gen - Résolution problèmes critiques

param(
    [int]$Port = 3002,
    [switch]$Verbose
)

Write-Host "🚨 MODE EMERGENCY - CTO DIAGNOSTIC ACTIVÉ" -ForegroundColor Red
Write-Host "=========================================" -ForegroundColor Red

$webUiPath = Join-Path $PSScriptRoot "web-ui"

function Write-Emergency {
    param([string]$Message, [string]$Level = "INFO")
    $timestamp = Get-Date -Format "HH:mm:ss"
    $color = switch ($Level) {
        "CRITICAL" { "Red" }
        "SUCCESS" { "Green" }
        "WARN" { "Yellow" }
        default { "Cyan" }
    }
    Write-Host "[$timestamp] [EMERGENCY-$Level] $Message" -ForegroundColor $color
}

# Vérifications critiques
Write-Emergency "Vérification de l'environnement..." "INFO"

if (-not (Test-Path $webUiPath)) {
    Write-Emergency "ERREUR: Dossier web-ui non trouvé" "CRITICAL"
    exit 1
}

Set-Location $webUiPath

# Backup du fichier index.tsx original
if (Test-Path "src/index.tsx" -and -not (Test-Path "src/index.original.tsx")) {
    Copy-Item "src/index.tsx" "src/index.original.tsx"
    Write-Emergency "Backup de index.tsx créé" "SUCCESS"
}

# Remplacer par la version d'urgence
Copy-Item "src/index.emergency.tsx" "src/index.tsx" -Force
Write-Emergency "Version d'urgence activée" "SUCCESS"

# Configuration ultra-stable
$env:FAST_REFRESH = "false"
$env:WDS_HOT = "false"
$env:WDS_LIVE_RELOAD = "false"
$env:CHOKIDAR_USEPOLLING = "false"  # Désactiver le polling
$env:WATCHPACK_POLLING = "false"
$env:BROWSER = "none"
$env:GENERATE_SOURCEMAP = "false"
$env:ESLINT_NO_DEV_ERRORS = "true"
$env:TSC_COMPILE_ON_ERROR = "true"
$env:PORT = $Port
$env:REACT_APP_EMERGENCY_MODE = "true"

Write-Emergency "Configuration ultra-stable appliquée" "SUCCESS"

# Nettoyer le cache
Write-Emergency "Nettoyage du cache..." "INFO"
if (Test-Path "node_modules/.cache") {
    Remove-Item "node_modules/.cache" -Recurse -Force -ErrorAction SilentlyContinue
}

Write-Emergency "Démarrage du serveur d'urgence..." "INFO"
Write-Emergency "URL: http://localhost:$Port" "SUCCESS"
Write-Emergency "Mode: EMERGENCY DIAGNOSTIC" "WARN"
Write-Emergency "" "INFO"
Write-Emergency "🔧 FONCTIONNALITÉS DISPONIBLES:" "INFO"
Write-Emergency "   - Interface ultra-minimaliste" "INFO"
Write-Emergency "   - Tests de stabilité intégrés" "INFO"
Write-Emergency "   - Diagnostic en temps réel" "INFO"
Write-Emergency "   - Aucun composant complexe" "INFO"
Write-Emergency "" "INFO"
Write-Emergency "Appuyez sur Ctrl+C pour arrêter" "WARN"

try {
    # Démarrer React en mode d'urgence
    npm start
} catch {
    Write-Emergency "ERREUR CRITIQUE: $($_.Exception.Message)" "CRITICAL"
} finally {
    # Restaurer le fichier original
    if (Test-Path "src/index.original.tsx") {
        Copy-Item "src/index.original.tsx" "src/index.tsx" -Force
        Remove-Item "src/index.original.tsx" -Force
        Write-Emergency "Fichier original restauré" "SUCCESS"
    }
    
    Set-Location $PSScriptRoot
    Write-Emergency "Mode d'urgence termine" "INFO"
}
