# Script PowerShell pour démarrer l'interface web sans problèmes de rafraîchissement
# Auteur: Consciousness Engine Team
# Version: 1.0

param(
    [switch]$Static,
    [switch]$Dev,
    [int]$Port = 3001
)

Write-Host "🧠 Consciousness Engine - Interface Web Stable" -ForegroundColor Cyan
Write-Host "=============================================" -ForegroundColor Cyan

# Vérifier si Node.js est installé
try {
    $nodeVersion = node --version
    Write-Host "✅ Node.js détecté: $nodeVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Node.js n'est pas installé ou non accessible" -ForegroundColor Red
    Write-Host "Veuillez installer Node.js depuis https://nodejs.org/" -ForegroundColor Yellow
    exit 1
}

# Aller dans le dossier web-ui
$webUiPath = Join-Path $PSScriptRoot "web-ui"
if (-not (Test-Path $webUiPath)) {
    Write-Host "❌ Dossier web-ui non trouvé: $webUiPath" -ForegroundColor Red
    exit 1
}

Set-Location $webUiPath
Write-Host "📁 Répertoire de travail: $webUiPath" -ForegroundColor Blue

# Vérifier si les dépendances sont installées
if (-not (Test-Path "node_modules")) {
    Write-Host "📦 Installation des dépendances..." -ForegroundColor Yellow
    npm install
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Échec de l'installation des dépendances" -ForegroundColor Red
        exit 1
    }
}

if ($Static) {
    Write-Host "🔧 Mode statique - Construction et serveur HTTP simple" -ForegroundColor Yellow
    
    # Construire l'application
    Write-Host "🏗️ Construction de l'application..." -ForegroundColor Blue
    npm run build
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Échec de la construction" -ForegroundColor Red
        exit 1
    }
    
    # Démarrer le serveur statique
    Write-Host "🚀 Démarrage du serveur statique sur le port $Port..." -ForegroundColor Green
    
    # Serveur HTTP simple en PowerShell
    $listener = New-Object System.Net.HttpListener
    $listener.Prefixes.Add("http://127.0.0.1:$Port/")
    $listener.Start()
    
    Write-Host "✅ Serveur démarré: http://127.0.0.1:$Port" -ForegroundColor Green
    Write-Host "Appuyez sur Ctrl+C pour arrêter" -ForegroundColor Yellow
    
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
} else {
    Write-Host "🔧 Mode développement stable" -ForegroundColor Yellow
    
    # Variables d'environnement pour stabilité
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
    
    Write-Host "🚀 Démarrage du serveur de développement stable..." -ForegroundColor Green
    Write-Host "   Local:   http://127.0.0.1:$Port" -ForegroundColor Cyan
    Write-Host "   Accès:   http://127.0.0.1:$Port/#/login" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "⚠️  Hot Reload et Fast Refresh sont DÉSACTIVÉS pour éviter les boucles" -ForegroundColor Yellow
    Write-Host "   Rechargez manuellement la page après vos modifications" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Appuyez sur Ctrl+C pour arrêter" -ForegroundColor Yellow
    
    # Démarrer React
    npm start
}
