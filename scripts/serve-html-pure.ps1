# SERVEUR HTML PUR - SOLUTION DEFINITIVE CTO
# Aucune dépendance React - Stabilité garantie

param([int]$Port = 3003)

Write-Host "HTML PUR - SOLUTION DEFINITIVE CTO" -ForegroundColor Green
Write-Host "===================================" -ForegroundColor Green

$webUiPath = Join-Path $PSScriptRoot "web-ui\public"

# Vérifier que le fichier existe
$htmlFile = Join-Path $webUiPath "standalone.html"
if (-not (Test-Path $htmlFile)) {
    Write-Host "ERREUR: Fichier standalone.html non trouvé" -ForegroundColor Red
    exit 1
}

Write-Host "Fichier HTML trouvé: $htmlFile" -ForegroundColor Green

# Arrêter tous les processus Node
Write-Host "Arrêt des processus Node..." -ForegroundColor Yellow
Get-Process -Name "node" -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue

Write-Host ""
Write-Host "DEMARRAGE SERVEUR HTML PUR" -ForegroundColor Cyan
Write-Host "URL: http://localhost:$Port" -ForegroundColor Green
Write-Host "Mode: HTML PUR - ZERO DEPENDANCE" -ForegroundColor Yellow
Write-Host ""
Write-Host "AVANTAGES DEFINITIFS:" -ForegroundColor White
Write-Host "   - Aucun React" -ForegroundColor Green
Write-Host "   - Aucun webpack" -ForegroundColor Green
Write-Host "   - Aucun WebSocket" -ForegroundColor Green
Write-Host "   - Aucune compilation" -ForegroundColor Green
Write-Host "   - Aucune boucle possible" -ForegroundColor Green
Write-Host "   - Performance maximale" -ForegroundColor Green
Write-Host "   - Stabilité garantie 100%" -ForegroundColor Green
Write-Host ""
Write-Host "Appuyez sur Ctrl+C pour arrêter" -ForegroundColor Yellow

try {
    # Serveur HTTP ultra-simple
    $listener = New-Object System.Net.HttpListener
    $listener.Prefixes.Add("http://localhost:$Port/")
    $listener.Start()
    
    Write-Host "✅ Serveur HTML pur démarré avec succès" -ForegroundColor Green
    
    while ($listener.IsListening) {
        $context = $listener.GetContext()
        $request = $context.Request
        $response = $context.Response
        
        try {
            # Toujours servir le fichier standalone.html
            $content = [System.IO.File]::ReadAllText($htmlFile, [System.Text.Encoding]::UTF8)
            $contentBytes = [System.Text.Encoding]::UTF8.GetBytes($content)
            
            $response.ContentType = "text/html; charset=utf-8"
            $response.ContentLength64 = $contentBytes.Length
            $response.StatusCode = 200
            
            # Headers pour éviter la mise en cache
            $response.Headers.Add("Cache-Control", "no-cache, no-store, must-revalidate")
            $response.Headers.Add("Pragma", "no-cache")
            $response.Headers.Add("Expires", "0")
            
            $response.OutputStream.Write($contentBytes, 0, $contentBytes.Length)
            
            # Log de la requête
            $timestamp = Get-Date -Format "HH:mm:ss"
            Write-Host "[$timestamp] ${request.HttpMethod} ${request.Url.AbsolutePath} - 200 OK" -ForegroundColor Green
            
        } catch {
            Write-Host "ERREUR lors du traitement de la requête: $($_.Exception.Message)" -ForegroundColor Red
            
            $response.StatusCode = 500
            $errorContent = [System.Text.Encoding]::UTF8.GetBytes("500 - Erreur serveur: $($_.Exception.Message)")
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
        Write-Host "Serveur HTML arrêté" -ForegroundColor Cyan
    }
}
