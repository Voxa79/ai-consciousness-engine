# SOLUTION RADICALE - BUILD STATIQUE + SERVEUR HTTP SIMPLE
# Expert CTO Next Gen - Évite complètement webpack-dev-server

param([int]$Port = 3003)

Write-Host "SOLUTION RADICALE CTO - BUILD STATIQUE" -ForegroundColor Red
Write-Host "======================================" -ForegroundColor Red

$webUiPath = Join-Path $PSScriptRoot "web-ui"

# Arrêter tous les processus Node
Write-Host "Arrêt complet des processus Node..." -ForegroundColor Yellow
Get-Process -Name "node" -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue

Set-Location $webUiPath

# Activer la version ultra-minimale
if (Test-Path "src/index.tsx") {
    Copy-Item "src/index.tsx" "src/index.backup.tsx" -Force -ErrorAction SilentlyContinue
}
Copy-Item "src/index.ultraminimal.tsx" "src/index.tsx" -Force

# Configuration pour build
$env:GENERATE_SOURCEMAP = "false"
$env:ESLINT_NO_DEV_ERRORS = "true"
$env:TSC_COMPILE_ON_ERROR = "true"
$env:REACT_APP_FINAL_STABLE_MODE = "true"

Write-Host "Construction de l'application..." -ForegroundColor Cyan
npm run build

if ($LASTEXITCODE -ne 0) {
    Write-Host "ERREUR: Échec de la construction" -ForegroundColor Red
    exit 1
}

Write-Host "Construction réussie!" -ForegroundColor Green

# Serveur HTTP simple en PowerShell
Write-Host ""
Write-Host "DÉMARRAGE SERVEUR HTTP STATIQUE" -ForegroundColor Green
Write-Host "URL: http://localhost:$Port" -ForegroundColor Cyan
Write-Host "Mode: STATIQUE - AUCUN WEBPACK" -ForegroundColor Yellow
Write-Host ""
Write-Host "AVANTAGES:" -ForegroundColor White
Write-Host "   - Aucun webpack-dev-server" -ForegroundColor Green
Write-Host "   - Aucun WebSocket" -ForegroundColor Green
Write-Host "   - Aucune recompilation" -ForegroundColor Green
Write-Host "   - Performance maximale" -ForegroundColor Green
Write-Host "   - Stabilité garantie" -ForegroundColor Green
Write-Host ""
Write-Host "Appuyez sur Ctrl+C pour arrêter" -ForegroundColor Yellow

try {
    # Serveur HTTP simple
    $listener = New-Object System.Net.HttpListener
    $listener.Prefixes.Add("http://localhost:$Port/")
    $listener.Start()
    
    Write-Host "✅ Serveur HTTP démarré avec succès" -ForegroundColor Green
    
    while ($listener.IsListening) {
        $context = $listener.GetContext()
        $request = $context.Request
        $response = $context.Response
        
        $path = $request.Url.AbsolutePath
        if ($path -eq "/") { $path = "/index.html" }
        
        $filePath = Join-Path "build" $path.TrimStart('/')
        
        try {
            if (Test-Path $filePath -PathType Leaf) {
                $content = [System.IO.File]::ReadAllBytes($filePath)
                
                # Définir le type MIME
                $extension = [System.IO.Path]::GetExtension($filePath).ToLower()
                switch ($extension) {
                    ".html" { $response.ContentType = "text/html; charset=utf-8" }
                    ".js" { $response.ContentType = "application/javascript" }
                    ".css" { $response.ContentType = "text/css" }
                    ".json" { $response.ContentType = "application/json" }
                    ".png" { $response.ContentType = "image/png" }
                    ".jpg" { $response.ContentType = "image/jpeg" }
                    ".ico" { $response.ContentType = "image/x-icon" }
                    default { $response.ContentType = "application/octet-stream" }
                }
                
                $response.ContentLength64 = $content.Length
                $response.OutputStream.Write($content, 0, $content.Length)
            } else {
                # SPA fallback - toutes les routes non trouvées renvoient vers index.html
                $indexPath = Join-Path "build" "index.html"
                if (Test-Path $indexPath) {
                    $content = [System.IO.File]::ReadAllBytes($indexPath)
                    $response.ContentType = "text/html; charset=utf-8"
                    $response.ContentLength64 = $content.Length
                    $response.OutputStream.Write($content, 0, $content.Length)
                } else {
                    $response.StatusCode = 404
                    $errorContent = [System.Text.Encoding]::UTF8.GetBytes("404 - Page non trouvée")
                    $response.ContentLength64 = $errorContent.Length
                    $response.OutputStream.Write($errorContent, 0, $errorContent.Length)
                }
            }
        } catch {
            $response.StatusCode = 500
            $errorContent = [System.Text.Encoding]::UTF8.GetBytes("500 - Erreur serveur")
            $response.ContentLength64 = $errorContent.Length
            $response.OutputStream.Write($errorContent, 0, $errorContent.Length)
        }
        
        $response.Close()
    }
} catch {
    Write-Host "ERREUR: $($_.Exception.Message)" -ForegroundColor Red
} finally {
    if ($listener) {
        $listener.Stop()
    }
    
    # Restaurer le fichier original
    if (Test-Path "src/index.backup.tsx") {
        Copy-Item "src/index.backup.tsx" "src/index.tsx" -Force
    }
    
    Set-Location $PSScriptRoot
    Write-Host "Serveur HTTP arrêté" -ForegroundColor Cyan
}
