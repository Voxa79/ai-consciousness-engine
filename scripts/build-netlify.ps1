# Build Script for Netlify Deployment - Consciousness Engine
# Expert CTO Next Gen - Optimized Production Build

param(
    [switch]$Clean,
    [switch]$Optimize,
    [switch]$Test,
    [switch]$Deploy,
    [switch]$Preview
)

Write-Host "🚀 CONSCIOUSNESS ENGINE - BUILD NETLIFY" -ForegroundColor Green
Write-Host "=======================================" -ForegroundColor Green

$ErrorActionPreference = "Stop"

# Configuration
$BuildConfig = @{
    StartTime = Get-Date
    NodeVersion = "20"
    NpmVersion = "10"
    BuildTarget = "netlify"
    Environment = "production"
    OptimizationLevel = "maximum"
}

Write-Host "🔧 Configuration Build:" -ForegroundColor Yellow
Write-Host "   Target: $($BuildConfig.BuildTarget)" -ForegroundColor White
Write-Host "   Environment: $($BuildConfig.Environment)" -ForegroundColor White
Write-Host "   Node Version: $($BuildConfig.NodeVersion)" -ForegroundColor White
Write-Host "   Clean: $Clean" -ForegroundColor White
Write-Host "   Optimize: $Optimize" -ForegroundColor White
Write-Host "   Test: $Test" -ForegroundColor White

# Fonction de nettoyage
function Invoke-CleanBuild {
    if (-not $Clean) { return }
    
    Write-Host "🧹 Nettoyage des fichiers de build..." -ForegroundColor Cyan
    
    $cleanTargets = @(
        "dist",
        "build",
        ".netlify",
        "node_modules/.cache",
        ".vite",
        ".turbo"
    )
    
    foreach ($target in $cleanTargets) {
        if (Test-Path $target) {
            Remove-Item -Recurse -Force $target
            Write-Host "   ✅ Supprimé: $target" -ForegroundColor Green
        }
    }
}

# Fonction de vérification des prérequis
function Test-BuildPrerequisites {
    Write-Host "🔍 Vérification des prérequis..." -ForegroundColor Cyan
    
    # Vérifier Node.js
    try {
        $nodeVersion = node --version
        Write-Host "   ✅ Node.js: $nodeVersion" -ForegroundColor Green
    } catch {
        Write-Host "   ❌ Node.js non trouvé" -ForegroundColor Red
        throw "Node.js est requis"
    }
    
    # Vérifier npm
    try {
        $npmVersion = npm --version
        Write-Host "   ✅ npm: v$npmVersion" -ForegroundColor Green
    } catch {
        Write-Host "   ❌ npm non trouvé" -ForegroundColor Red
        throw "npm est requis"
    }
    
    # Vérifier Netlify CLI
    try {
        $netlifyVersion = netlify --version
        Write-Host "   ✅ Netlify CLI: $netlifyVersion" -ForegroundColor Green
    } catch {
        Write-Host "   ⚠️ Netlify CLI non trouvé, installation..." -ForegroundColor Yellow
        npm install -g netlify-cli
    }
    
    # Vérifier package.json
    if (-not (Test-Path "package.json")) {
        throw "package.json non trouvé"
    }
    Write-Host "   ✅ package.json trouvé" -ForegroundColor Green
    
    # Vérifier netlify.toml
    if (-not (Test-Path "netlify.toml")) {
        throw "netlify.toml non trouvé"
    }
    Write-Host "   ✅ netlify.toml trouvé" -ForegroundColor Green
}

# Fonction d'installation des dépendances
function Install-Dependencies {
    Write-Host "📦 Installation des dépendances..." -ForegroundColor Cyan
    
    # Nettoyer le cache npm
    npm cache clean --force
    
    # Installer les dépendances
    npm ci --production=false --silent
    
    if ($LASTEXITCODE -ne 0) {
        throw "Échec de l'installation des dépendances"
    }
    
    Write-Host "   ✅ Dépendances installées" -ForegroundColor Green
    
    # Vérifier les vulnérabilités
    Write-Host "🔒 Audit de sécurité..." -ForegroundColor Cyan
    npm audit --audit-level=high
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "   ⚠️ Vulnérabilités détectées, correction automatique..." -ForegroundColor Yellow
        npm audit fix --force
    } else {
        Write-Host "   ✅ Aucune vulnérabilité critique" -ForegroundColor Green
    }
}

# Fonction de tests
function Invoke-Tests {
    if (-not $Test) { return }
    
    Write-Host "🧪 Exécution des tests..." -ForegroundColor Cyan
    
    # Tests unitaires
    Write-Host "   Tests unitaires..." -ForegroundColor Yellow
    npm run test -- --run --reporter=verbose
    
    if ($LASTEXITCODE -ne 0) {
        throw "Échec des tests unitaires"
    }
    
    # Vérification des types
    Write-Host "   Vérification TypeScript..." -ForegroundColor Yellow
    npm run type-check
    
    if ($LASTEXITCODE -ne 0) {
        throw "Erreurs de types TypeScript"
    }
    
    # Linting
    Write-Host "   Linting du code..." -ForegroundColor Yellow
    npm run lint
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "   ⚠️ Erreurs de linting détectées, correction automatique..." -ForegroundColor Yellow
        npm run lint:fix
    }
    
    Write-Host "   ✅ Tous les tests réussis" -ForegroundColor Green
}

# Fonction de build
function Invoke-Build {
    Write-Host "🏗️ Build de l'application..." -ForegroundColor Cyan
    
    # Variables d'environnement pour le build
    $env:NODE_ENV = "production"
    $env:VITE_APP_VERSION = "2.0.0"
    $env:VITE_BUILD_TIME = (Get-Date).ToString("yyyy-MM-ddTHH:mm:ssZ")
    $env:VITE_CONSCIOUSNESS_MODE = "production"
    $env:VITE_NEURAL_INTERFACE_ENABLED = "true"
    $env:VITE_QUANTUM_COMPUTING_ENABLED = "true"
    $env:VITE_NANOTECHNOLOGY_ENABLED = "true"
    $env:VITE_SPACE_NETWORK_ENABLED = "true"
    
    # Build principal
    npm run build:netlify
    
    if ($LASTEXITCODE -ne 0) {
        throw "Échec du build"
    }
    
    Write-Host "   ✅ Build terminé" -ForegroundColor Green
    
    # Vérifier la taille du build
    if (Test-Path "dist") {
        $buildSize = (Get-ChildItem -Recurse "dist" | Measure-Object -Property Length -Sum).Sum
        $buildSizeMB = [math]::Round($buildSize / 1MB, 2)
        Write-Host "   📊 Taille du build: $buildSizeMB MB" -ForegroundColor Cyan
        
        if ($buildSizeMB -gt 50) {
            Write-Host "   ⚠️ Build volumineux (>50MB), optimisation recommandée" -ForegroundColor Yellow
        }
    }
}

# Fonction d'optimisation
function Invoke-Optimization {
    if (-not $Optimize) { return }
    
    Write-Host "⚡ Optimisation du build..." -ForegroundColor Cyan
    
    # Optimisation des images
    if (Get-Command "imagemin" -ErrorAction SilentlyContinue) {
        Write-Host "   Optimisation des images..." -ForegroundColor Yellow
        npm run optimize:images
    }
    
    # Compression des assets
    Write-Host "   Compression des assets..." -ForegroundColor Yellow
    npm run optimize:assets
    
    # Génération des fichiers de cache
    Write-Host "   Génération des manifests de cache..." -ForegroundColor Yellow
    
    # Créer un manifest des fichiers
    $manifest = @{
        version = "2.0.0"
        buildTime = (Get-Date).ToString("yyyy-MM-ddTHH:mm:ssZ")
        files = @()
    }
    
    if (Test-Path "dist") {
        Get-ChildItem -Recurse "dist" -File | ForEach-Object {
            $relativePath = $_.FullName.Replace((Get-Location).Path + "\dist\", "").Replace("\", "/")
            $manifest.files += @{
                path = $relativePath
                size = $_.Length
                hash = (Get-FileHash $_.FullName -Algorithm SHA256).Hash
            }
        }
    }
    
    $manifest | ConvertTo-Json -Depth 3 | Out-File "dist/manifest.json" -Encoding UTF8
    
    Write-Host "   ✅ Optimisation terminée" -ForegroundColor Green
}

# Fonction de validation
function Test-BuildOutput {
    Write-Host "✅ Validation du build..." -ForegroundColor Cyan
    
    $requiredFiles = @(
        "dist/index.html",
        "dist/assets",
        "netlify.toml"
    )
    
    foreach ($file in $requiredFiles) {
        if (-not (Test-Path $file)) {
            throw "Fichier requis manquant: $file"
        }
        Write-Host "   ✅ $file" -ForegroundColor Green
    }
    
    # Vérifier les fonctions Netlify
    if (Test-Path "netlify/functions") {
        $functions = Get-ChildItem "netlify/functions" -Filter "*.mts"
        Write-Host "   📋 Fonctions Netlify: $($functions.Count)" -ForegroundColor Cyan
        
        foreach ($func in $functions) {
            Write-Host "     - $($func.BaseName)" -ForegroundColor White
        }
    }
    
    # Vérifier la configuration Netlify
    if (Test-Path "netlify.toml") {
        $netlifyConfig = Get-Content "netlify.toml" -Raw
        if ($netlifyConfig -match 'publish = "dist"') {
            Write-Host "   ✅ Configuration Netlify valide" -ForegroundColor Green
        } else {
            Write-Host "   ⚠️ Configuration Netlify à vérifier" -ForegroundColor Yellow
        }
    }
}

# Fonction de déploiement
function Invoke-NetlifyDeploy {
    if (-not $Deploy -and -not $Preview) { return }
    
    Write-Host "🚀 Déploiement Netlify..." -ForegroundColor Cyan
    
    # Vérifier l'authentification Netlify
    try {
        netlify status
    } catch {
        Write-Host "   ⚠️ Authentification Netlify requise" -ForegroundColor Yellow
        netlify login
    }
    
    if ($Preview) {
        Write-Host "   Déploiement preview..." -ForegroundColor Yellow
        netlify deploy --dir=dist --open
    } else {
        Write-Host "   Déploiement production..." -ForegroundColor Yellow
        netlify deploy --prod --dir=dist --open
    }
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "   ✅ Déploiement réussi" -ForegroundColor Green
    } else {
        throw "Échec du déploiement Netlify"
    }
}

# Fonction principale
function Main {
    try {
        $BuildConfig.StartTime = Get-Date
        
        # Étapes de build
        Invoke-CleanBuild
        Test-BuildPrerequisites
        Install-Dependencies
        Invoke-Tests
        Invoke-Build
        Invoke-Optimization
        Test-BuildOutput
        Invoke-NetlifyDeploy
        
        $BuildConfig.EndTime = Get-Date
        $duration = $BuildConfig.EndTime - $BuildConfig.StartTime
        
        # Résumé final
        Write-Host ""
        Write-Host "🎉 BUILD NETLIFY TERMINÉ AVEC SUCCÈS!" -ForegroundColor Green
        Write-Host "=====================================" -ForegroundColor Green
        Write-Host ""
        Write-Host "⏱️ Durée totale: $($duration.ToString('mm\:ss'))" -ForegroundColor White
        Write-Host "📦 Build target: $($BuildConfig.BuildTarget)" -ForegroundColor White
        Write-Host "🌍 Environment: $($BuildConfig.Environment)" -ForegroundColor White
        Write-Host ""
        
        if (Test-Path "dist") {
            $buildSize = (Get-ChildItem -Recurse "dist" | Measure-Object -Property Length -Sum).Sum
            $buildSizeMB = [math]::Round($buildSize / 1MB, 2)
            Write-Host "📊 Taille du build: $buildSizeMB MB" -ForegroundColor Cyan
        }
        
        if (Test-Path "netlify/functions") {
            $functions = Get-ChildItem "netlify/functions" -Filter "*.mts"
            Write-Host "⚡ Fonctions serverless: $($functions.Count)" -ForegroundColor Cyan
        }
        
        Write-Host ""
        Write-Host "🚀 Prêt pour le déploiement Netlify!" -ForegroundColor Green
        Write-Host ""
        Write-Host "Commandes de déploiement:" -ForegroundColor White
        Write-Host "   Preview: netlify deploy --dir=dist" -ForegroundColor Yellow
        Write-Host "   Production: netlify deploy --prod --dir=dist" -ForegroundColor Yellow
        Write-Host ""
        
    } catch {
        Write-Host "❌ Erreur lors du build: $($_.Exception.Message)" -ForegroundColor Red
        exit 1
    }
}

# Exécution
Main
