# SCRIPT EMERGENCY SIMPLE - MODE CTO DIAGNOSTIC
param([int]$Port = 3002)

Write-Host "EMERGENCY MODE - CTO DIAGNOSTIC" -ForegroundColor Red
Write-Host "===============================" -ForegroundColor Red

$webUiPath = Join-Path $PSScriptRoot "web-ui"
Set-Location $webUiPath

# Backup et remplacement
if (Test-Path "src/index.tsx" -and -not (Test-Path "src/index.original.tsx")) {
    Copy-Item "src/index.tsx" "src/index.original.tsx"
    Write-Host "Backup cree" -ForegroundColor Green
}

Copy-Item "src/index.emergency.tsx" "src/index.tsx" -Force
Write-Host "Version emergency activee" -ForegroundColor Green

# Configuration ultra-stable
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
$env:REACT_APP_EMERGENCY_MODE = "true"
$env:REACT_APP_DISABLE_WEBSOCKETS = "true"
$env:PORT = $Port

# Copier la configuration emergency
Copy-Item ".env.emergency" ".env.local" -Force

Write-Host "Demarrage serveur emergency sur port $Port..." -ForegroundColor Cyan
Write-Host "URL: http://localhost:$Port" -ForegroundColor Green

try {
    npm start
} finally {
    # Restaurer
    if (Test-Path "src/index.original.tsx") {
        Copy-Item "src/index.original.tsx" "src/index.tsx" -Force
        Remove-Item "src/index.original.tsx" -Force
        Write-Host "Fichier original restaure" -ForegroundColor Green
    }
    Set-Location $PSScriptRoot
}
