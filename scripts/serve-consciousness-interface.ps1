# Serveur pour Interface Consciousness HTML Pure
# Expert CTO Next Gen - Solution definitive sans React

param([int]$Port = 3001)

Write-Host "INTERFACE CONSCIOUSNESS HTML PURE" -ForegroundColor Green
Write-Host "=================================" -ForegroundColor Green

$htmlFile = Join-Path $PSScriptRoot "web-ui\public\consciousness-interface.html"

if (-not (Test-Path $htmlFile)) {
    Write-Host "ERREUR: Fichier consciousness-interface.html non trouve" -ForegroundColor Red
    exit 1
}

Write-Host "Fichier HTML trouve: $htmlFile" -ForegroundColor Green

# Arreter les processus Node existants sur ce port
Write-Host "Arret des processus sur port $Port..." -ForegroundColor Yellow
$processes = Get-NetTCPConnection -LocalPort $Port -ErrorAction SilentlyContinue
foreach ($proc in $processes) {
    try {
        Stop-Process -Id $proc.OwningProcess -Force -ErrorAction SilentlyContinue
    } catch {}
}

Write-Host ""
Write-Host "DEMARRAGE SERVEUR INTERFACE CONSCIOUSNESS" -ForegroundColor Cyan
Write-Host "URL: http://localhost:$Port" -ForegroundColor Green
Write-Host "Mode: HTML PUR - ZERO REACT" -ForegroundColor Yellow
Write-Host ""
Write-Host "FONCTIONNALITES:" -ForegroundColor White
Write-Host "   - Interface HTML pure" -ForegroundColor Green
Write-Host "   - Connexion API temps reel" -ForegroundColor Green
Write-Host "   - Chat consciousness" -ForegroundColor Green
Write-Host "   - Metriques avancees" -ForegroundColor Green
Write-Host "   - Gestion d'erreurs robuste" -ForegroundColor Green
Write-Host "   - Aucune boucle possible" -ForegroundColor Green
Write-Host ""
Write-Host "Appuyez sur Ctrl+C pour arreter" -ForegroundColor Yellow

try {
    # Serveur HTTP ultra-simple
    $listener = New-Object System.Net.HttpListener
    $listener.Prefixes.Add("http://localhost:$Port/")
    $listener.Start()
    
    Write-Host "✅ Serveur Interface demarré avec succes" -ForegroundColor Green
    
    while ($listener.IsListening) {
        $context = $listener.GetContext()
        $request = $context.Request
        $response = $context.Response
        
        try {
            # Servir l'interface consciousness pour toutes les routes
            $content = [System.IO.File]::ReadAllText($htmlFile, [System.Text.Encoding]::UTF8)
            $contentBytes = [System.Text.Encoding]::UTF8.GetBytes($content)
            
            $response.ContentType = "text/html; charset=utf-8"
            $response.ContentLength64 = $contentBytes.Length
            $response.StatusCode = 200
            
            # Headers CORS et cache
            $response.Headers.Add("Access-Control-Allow-Origin", "*")
            $response.Headers.Add("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
            $response.Headers.Add("Access-Control-Allow-Headers", "Content-Type")
            $response.Headers.Add("Cache-Control", "no-cache, no-store, must-revalidate")
            $response.Headers.Add("Pragma", "no-cache")
            $response.Headers.Add("Expires", "0")
            
            $response.OutputStream.Write($contentBytes, 0, $contentBytes.Length)
            
            # Log de la requete
            $timestamp = Get-Date -Format "HH:mm:ss"
            Write-Host "[$timestamp] ${request.HttpMethod} ${request.Url.AbsolutePath} - 200 OK" -ForegroundColor Green
            
        } catch {
            Write-Host "ERREUR lors du traitement: $($_.Exception.Message)" -ForegroundColor Red
            
            $response.StatusCode = 500
            $errorContent = [System.Text.Encoding]::UTF8.GetBytes("500 - Erreur serveur")
            $response.ContentLength64 = $errorContent.Length
            $response.OutputStream.Write($errorContent, 0, $errorContent.Length)
        }
        
        $response.Close()
    }
} catch {
    Write-Host "ERREUR CRITIQUE: $($_.Exception.Message)" -ForegroundColor Red
} finally {
    if ($listener) {
        $listener.Stop()
        Write-Host "Serveur Interface arrete" -ForegroundColor Cyan
    }
}
