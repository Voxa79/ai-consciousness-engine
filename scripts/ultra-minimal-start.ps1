# SCRIPT ULTRA-MINIMAL - ZERO CONFLIT GARANTI
# Expert CTO Next Gen - Solution definitive

param([int]$Port = 3003)

Write-Host "ULTRA-MINIMAL CTO - ZERO CONFLIT" -ForegroundColor Green
Write-Host "================================" -ForegroundColor Green

$webUiPath = Join-Path $PSScriptRoot "web-ui"

# Arreter tous les processus Node existants
Write-Host "Arret des processus Node existants..." -ForegroundColor Yellow
Get-Process -Name "node" -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue

Set-Location $webUiPath

# Backup et remplacement
if (Test-Path "src/index.tsx") {
    if (-not (Test-Path "src/index.original.tsx")) {
        Copy-Item "src/index.tsx" "src/index.original.tsx"
        Write-Host "Backup original cree" -ForegroundColor Green
    }
}

# Activer la version ultra-minimale
Copy-Item "src/index.ultraminimal.tsx" "src/index.tsx" -Force
Write-Host "Version ultra-minimale activee" -ForegroundColor Green

# Nettoyer le cache completement
Write-Host "Nettoyage cache complet..." -ForegroundColor Yellow
if (Test-Path "node_modules/.cache") {
    Remove-Item "node_modules/.cache" -Recurse -Force -ErrorAction SilentlyContinue
}

# Configuration ultra-stable (variables d'environnement)
$env:PORT = $Port
$env:FAST_REFRESH = "false"
$env:WDS_HOT = "false"
$env:WDS_LIVE_RELOAD = "false"
$env:WDS_SOCKET_HOST = ""
$env:WDS_SOCKET_PORT = ""
$env:WDS_SOCKET_PROTOCOL = ""
$env:CHOKIDAR_USEPOLLING = "false"
$env:WATCHPACK_POLLING = "false"
$env:BROWSER = "none"
$env:GENERATE_SOURCEMAP = "false"
$env:ESLINT_NO_DEV_ERRORS = "true"
$env:TSC_COMPILE_ON_ERROR = "true"
$env:REACT_APP_FINAL_STABLE_MODE = "true"
$env:REACT_APP_DISABLE_ALL_WEBSOCKETS = "true"
$env:REACT_APP_DISABLE_PROXY = "true"

Write-Host "Configuration ultra-stable appliquee" -ForegroundColor Green
Write-Host ""
Write-Host "DEMARRAGE ULTRA-MINIMAL" -ForegroundColor Cyan
Write-Host "URL: http://localhost:$Port" -ForegroundColor Green
Write-Host "Mode: ULTRA-MINIMAL - ZERO CONFLIT" -ForegroundColor Yellow
Write-Host ""
Write-Host "CARACTERISTIQUES:" -ForegroundColor White
Write-Host "   - Aucune dependance externe problematique" -ForegroundColor Green
Write-Host "   - Aucun WebSocket actif" -ForegroundColor Green
Write-Host "   - Aucun proxy actif" -ForegroundColor Green
Write-Host "   - Aucune boucle possible" -ForegroundColor Green
Write-Host "   - Styles inline uniquement" -ForegroundColor Green
Write-Host "   - Configuration ultra-stable" -ForegroundColor Green
Write-Host ""
Write-Host "Appuyez sur Ctrl+C pour arreter" -ForegroundColor Yellow

try {
    npm start
} finally {
    # Restaurer le fichier original
    if (Test-Path "src/index.original.tsx") {
        Copy-Item "src/index.original.tsx" "src/index.tsx" -Force
        Write-Host "Fichier original restaure" -ForegroundColor Green
    }
    Set-Location $PSScriptRoot
    Write-Host "Session ultra-minimale terminee" -ForegroundColor Cyan
}
