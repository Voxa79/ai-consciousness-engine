# Script simple pour demarrer l'interface React
# Expert CTO Next Gen - Version sans problemes d'encodage

param(
    [int]$Port = 3001,
    [string]$ApiUrl = "http://localhost:3000/api/v1"
)

Write-Host "DEMARRAGE INTERFACE REACT PRODUCTION" -ForegroundColor Green
Write-Host "====================================" -ForegroundColor Green

$webUiPath = Join-Path $PSScriptRoot "web-ui"

# Verifier Node.js
try {
    $nodeVersion = node --version
    Write-Host "Node.js detecte: $nodeVersion" -ForegroundColor Green
} catch {
    Write-Host "Node.js non disponible" -ForegroundColor Red
    exit 1
}

Set-Location $webUiPath

# Backup et activation version production
if (Test-Path "src/index.tsx") {
    if (-not (Test-Path "src/index.backup-simple.tsx")) {
        Copy-Item "src/index.tsx" "src/index.backup-simple.tsx"
        Write-Host "Backup original cree" -ForegroundColor Green
    }
}

# Creer le point d'entree production
$productionIndex = @"
import React from 'react';
import ReactDOM from 'react-dom/client';
import ProductionApp from './App.production';

console.log('PRODUCTION UI - Consciousness Engine Interface');

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);

try {
  root.render(<ProductionApp />);
  console.log('Interface production initialisee avec succes');
} catch (error) {
  console.error('Erreur lors de l initialisation:', error);
  
  root.render(
    <div style={{
      padding: '20px',
      textAlign: 'center',
      fontFamily: 'Arial, sans-serif',
      backgroundColor: '#0A0A0A',
      color: '#FFFFFF',
      minHeight: '100vh',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
      flexDirection: 'column'
    }}>
      <h1>Consciousness Engine</h1>
      <p>Interface en cours de chargement...</p>
      <p style={{color: '#FF6B35'}}>Erreur: {error?.toString()}</p>
      <button 
        onClick={() => window.location.reload()}
        style={{
          padding: '10px 20px',
          backgroundColor: '#00E5FF',
          color: '#000',
          border: 'none',
          borderRadius: '4px',
          cursor: 'pointer',
          marginTop: '20px'
        }}
      >
        Recharger
      </button>
    </div>
  );
}

(window as any).__productionMode = true;
(window as any).__apiUrl = '$ApiUrl';
"@

$productionIndex | Out-File -FilePath "src/index.tsx" -Encoding UTF8

Write-Host "Version production activee" -ForegroundColor Green

# Configuration variables d'environnement
$env:PORT = $Port
$env:REACT_APP_API_URL = $ApiUrl
$env:REACT_APP_PRODUCTION_MODE = "true"
$env:GENERATE_SOURCEMAP = "false"
$env:ESLINT_NO_DEV_ERRORS = "true"
$env:TSC_COMPILE_ON_ERROR = "true"
$env:FAST_REFRESH = "true"
$env:WDS_HOT = "true"
$env:WDS_LIVE_RELOAD = "false"
$env:CHOKIDAR_USEPOLLING = "false"
$env:BROWSER = "none"

Write-Host "Configuration production appliquee" -ForegroundColor Green

# Verifier dependances
if (-not (Test-Path "node_modules")) {
    Write-Host "Installation des dependances..." -ForegroundColor Cyan
    npm install
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Echec installation dependances" -ForegroundColor Red
        exit 1
    }
}

Write-Host ""
Write-Host "DEMARRAGE INTERFACE PRODUCTION" -ForegroundColor Cyan
Write-Host "URL: http://localhost:$Port" -ForegroundColor Green
Write-Host "API: $ApiUrl" -ForegroundColor Green
Write-Host "Mode: PRODUCTION UI" -ForegroundColor Yellow
Write-Host ""
Write-Host "FONCTIONNALITES:" -ForegroundColor White
Write-Host "   - Interface React stable" -ForegroundColor Green
Write-Host "   - Connexion API robuste" -ForegroundColor Green
Write-Host "   - Gestion d'erreurs avancee" -ForegroundColor Green
Write-Host "   - Retry automatique" -ForegroundColor Green
Write-Host "   - Metriques en temps reel" -ForegroundColor Green
Write-Host ""
Write-Host "Appuyez sur Ctrl+C pour arreter" -ForegroundColor Yellow

try {
    npm start
} catch {
    Write-Host "Erreur lors du demarrage" -ForegroundColor Red
} finally {
    # Restaurer fichier original
    if (Test-Path "src/index.backup-simple.tsx") {
        Copy-Item "src/index.backup-simple.tsx" "src/index.tsx" -Force
        Write-Host "Fichier original restaure" -ForegroundColor Green
    }
    
    Set-Location $PSScriptRoot
    Write-Host "Session production terminee" -ForegroundColor Cyan
}
