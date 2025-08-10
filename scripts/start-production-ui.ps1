# SCRIPT DE DÉMARRAGE PRODUCTION UI
# Expert CTO Next Gen - Interface React stable avec API backend

param(
    [int]$Port = 3001,
    [string]$ApiUrl = "http://localhost:8080",
    [switch]$MockApi = $false
)

Write-Host "🚀 DÉMARRAGE INTERFACE PRODUCTION" -ForegroundColor Green
Write-Host "=================================" -ForegroundColor Green

$webUiPath = Join-Path $PSScriptRoot "web-ui"

# Vérifier que Node.js est installé
try {
    $nodeVersion = node --version
    Write-Host "✅ Node.js détecté: $nodeVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Node.js n'est pas installé ou non accessible" -ForegroundColor Red
    Write-Host "   Veuillez installer Node.js depuis https://nodejs.org/" -ForegroundColor Yellow
    exit 1
}

# Arrêter les processus Node existants
Write-Host "🛑 Arrêt des processus Node existants..." -ForegroundColor Yellow
Get-Process -Name "node" -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue

Set-Location $webUiPath

# Backup et activation de la version production
if (Test-Path "src/index.tsx") {
    if (-not (Test-Path "src/index.backup-prod.tsx")) {
        Copy-Item "src/index.tsx" "src/index.backup-prod.tsx"
        Write-Host "✅ Backup original créé" -ForegroundColor Green
    }
}

# Créer le point d'entrée production
$productionIndex = @"
import React from 'react';
import ReactDOM from 'react-dom/client';
import ProductionApp from './App.production';

console.log('🚀 PRODUCTION UI - Consciousness Engine Interface');

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);

// Rendu en mode production avec gestion d'erreurs
try {
  root.render(<ProductionApp />);
  console.log('✅ Interface production initialisée avec succès');
} catch (error) {
  console.error('❌ Erreur lors de l\'initialisation:', error);
  
  // Fallback vers interface simple
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
      <h1>🧠 Consciousness Engine</h1>
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

// Diagnostic global
(window as any).__productionMode = true;
(window as any).__apiUrl = '${ApiUrl}';
(window as any).__diagnosticInfo = {
  version: 'Production UI',
  timestamp: new Date().toISOString(),
  port: window.location.port,
  status: 'PRODUCTION'
};
"@

$productionIndex | Out-File -FilePath "src/index.tsx" -Encoding UTF8

Write-Host "✅ Version production activée" -ForegroundColor Green

# Configuration des variables d'environnement
$env:PORT = $Port
$env:REACT_APP_API_URL = $ApiUrl
$env:REACT_APP_PRODUCTION_MODE = "true"
$env:GENERATE_SOURCEMAP = "false"
$env:ESLINT_NO_DEV_ERRORS = "true"
$env:TSC_COMPILE_ON_ERROR = "true"

# Configuration pour stabilité
$env:FAST_REFRESH = "true"
$env:WDS_HOT = "true"
$env:WDS_LIVE_RELOAD = "false"
$env:CHOKIDAR_USEPOLLING = "false"
$env:WATCHPACK_POLLING = "false"
$env:BROWSER = "none"

Write-Host "✅ Configuration production appliquée" -ForegroundColor Green

# Vérifier les dépendances
if (-not (Test-Path "node_modules")) {
    Write-Host "📦 Installation des dépendances..." -ForegroundColor Cyan
    npm install
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Échec de l'installation des dépendances" -ForegroundColor Red
        exit 1
    }
}

# Mock API si demandé
if ($MockApi) {
    Write-Host "🔧 Démarrage Mock API sur port 8080..." -ForegroundColor Cyan
    
    $mockApiScript = @"
const express = require('express');
const cors = require('cors');
const app = express();

app.use(cors());
app.use(express.json());

// Health check
app.get('/health', (req, res) => {
  res.json({
    status: 'healthy',
    timestamp: new Date().toISOString(),
    version: '1.0.0-mock'
  });
});

// Consciousness processing
app.post('/consciousness/process', (req, res) => {
  setTimeout(() => {
    res.json({
      id: 'mock-' + Date.now(),
      content: 'This is a mock response from the Consciousness Engine. Your input: "' + req.body.content + '"',
      confidence: 0.95,
      consciousness_level: 0.88,
      emotional_state: {
        primary_emotion: 'curious',
        intensity: 0.7,
        valence: 0.8,
        arousal: 0.6
      },
      ethical_score: 0.92,
      creativity_score: 0.85,
      empathy_score: 0.90,
      processing_time_ms: Math.floor(Math.random() * 100) + 50,
      reasoning_summary: 'Mock reasoning: Analyzed input and generated appropriate response.',
      quality_score: 0.93,
      timestamp: new Date().toISOString()
    });
  }, Math.floor(Math.random() * 500) + 200);
});

// Consciousness state
app.get('/consciousness/state', (req, res) => {
  res.json({
    awareness_level: 0.87,
    emotional_state: 'balanced',
    cognitive_load: 0.45,
    last_interaction: new Date().toISOString(),
    session_count: 42,
    uptime_seconds: 3600
  });
});

const port = 8080;
app.listen(port, () => {
  console.log('🤖 Mock Consciousness API running on port ' + port);
});
"@

    $mockApiScript | Out-File -FilePath "mock-api.js" -Encoding UTF8
    
    # Démarrer le mock API en arrière-plan
    Start-Process -FilePath "node" -ArgumentList "mock-api.js" -WindowStyle Hidden
    Start-Sleep -Seconds 2
    Write-Host "✅ Mock API démarré" -ForegroundColor Green
}

Write-Host ""
Write-Host "🚀 DÉMARRAGE INTERFACE PRODUCTION" -ForegroundColor Cyan
Write-Host "URL: http://localhost:$Port" -ForegroundColor Green
Write-Host "API: $ApiUrl" -ForegroundColor Green
Write-Host "Mode: PRODUCTION UI" -ForegroundColor Yellow
Write-Host ""
Write-Host "FONCTIONNALITÉS:" -ForegroundColor White
Write-Host "   - Interface React stable" -ForegroundColor Green
Write-Host "   - Connexion API robuste" -ForegroundColor Green
Write-Host "   - Gestion d'erreurs avancée" -ForegroundColor Green
Write-Host "   - Retry automatique" -ForegroundColor Green
Write-Host "   - Métriques en temps réel" -ForegroundColor Green
Write-Host "   - Fallback gracieux" -ForegroundColor Green
Write-Host ""
Write-Host "Appuyez sur Ctrl+C pour arrêter" -ForegroundColor Yellow

try {
    npm start
} catch {
    Write-Host "❌ Erreur lors du démarrage" -ForegroundColor Red
} finally {
    # Restaurer le fichier original
    if (Test-Path "src/index.backup-prod.tsx") {
        Copy-Item "src/index.backup-prod.tsx" "src/index.tsx" -Force
        Write-Host "✅ Fichier original restauré" -ForegroundColor Green
    }
    
    # Arrêter le mock API si utilisé
    if ($MockApi) {
        Get-Process -Name "node" -ErrorAction SilentlyContinue | Where-Object { $_.CommandLine -like "*mock-api.js*" } | Stop-Process -Force -ErrorAction SilentlyContinue
        Write-Host "🛑 Mock API arrêté" -ForegroundColor Yellow
    }
    
    Set-Location $PSScriptRoot
    Write-Host "Session production terminée" -ForegroundColor Cyan
}
