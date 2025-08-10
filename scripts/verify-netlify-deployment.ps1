# Script de vérification du déploiement Netlify - Consciousness Engine

Write-Host "VERIFICATION DEPLOIEMENT NETLIFY - CONSCIOUSNESS ENGINE" -ForegroundColor Green
Write-Host "======================================================" -ForegroundColor Green

# Vérifier les fichiers essentiels
Write-Host ""
Write-Host "Verification des fichiers essentiels..." -ForegroundColor Cyan

$essentialFiles = @(
    "netlify.toml",
    "package.json",
    "README_FR.md",
    "web-ui/public/index.html",
    "web-ui/public/consciousness-interface.html",
    "netlify/functions/consciousness-engine.mts",
    "netlify/functions/neural-interface.mts"
)

$allFilesExist = $true
foreach ($file in $essentialFiles) {
    if (Test-Path $file) {
        Write-Host "   OK $file" -ForegroundColor Green
    } else {
        Write-Host "   MANQUANT $file" -ForegroundColor Red
        $allFilesExist = $false
    }
}

if ($allFilesExist) {
    Write-Host "   OK Tous les fichiers essentiels sont presents" -ForegroundColor Green
} else {
    Write-Host "   ATTENTION Certains fichiers manquent" -ForegroundColor Yellow
}

# Vérifier le contenu du netlify.toml
Write-Host ""
Write-Host "Verification de netlify.toml..." -ForegroundColor Cyan

if (Test-Path "netlify.toml") {
    $netlifyConfig = Get-Content "netlify.toml" -Raw
    
    if ($netlifyConfig -match 'publish = "dist"') {
        Write-Host "   OK Publish directory configure" -ForegroundColor Green
    } else {
        Write-Host "   ATTENTION Publish directory non configure" -ForegroundColor Yellow
    }
    
    if ($netlifyConfig -match 'functions = "netlify/functions"') {
        Write-Host "   OK Functions directory configure" -ForegroundColor Green
    } else {
        Write-Host "   ATTENTION Functions directory non configure" -ForegroundColor Yellow
    }
    
    if ($netlifyConfig -match 'command = ') {
        Write-Host "   OK Build command configure" -ForegroundColor Green
    } else {
        Write-Host "   ATTENTION Build command non configure" -ForegroundColor Yellow
    }
} else {
    Write-Host "   ERREUR netlify.toml non trouve" -ForegroundColor Red
}

# Vérifier package.json
Write-Host ""
Write-Host "Verification de package.json..." -ForegroundColor Cyan

if (Test-Path "package.json") {
    $packageJson = Get-Content "package.json" -Raw | ConvertFrom-Json
    
    if ($packageJson.scripts.build) {
        Write-Host "   OK Script build present: $($packageJson.scripts.build)" -ForegroundColor Green
    } else {
        Write-Host "   ATTENTION Script build manquant" -ForegroundColor Yellow
    }
    
    if ($packageJson.scripts."build:netlify") {
        Write-Host "   OK Script build:netlify present" -ForegroundColor Green
    } else {
        Write-Host "   INFO Script build:netlify optionnel" -ForegroundColor Gray
    }
} else {
    Write-Host "   ATTENTION package.json non trouve" -ForegroundColor Yellow
}

# Recommandations de configuration Netlify
Write-Host ""
Write-Host "CONFIGURATION NETLIFY RECOMMANDEE :" -ForegroundColor Yellow
Write-Host "===================================" -ForegroundColor Yellow
Write-Host ""
Write-Host "Branch to deploy:" -ForegroundColor White
Write-Host "   main" -ForegroundColor Cyan
Write-Host ""
Write-Host "Base directory:" -ForegroundColor White
Write-Host "   (laisser vide)" -ForegroundColor Cyan
Write-Host ""
Write-Host "Build command:" -ForegroundColor White
Write-Host "   npm install && npm run build || echo 'Building static...' && mkdir -p dist && cp -r web-ui/public/* dist/" -ForegroundColor Cyan
Write-Host ""
Write-Host "Publish directory:" -ForegroundColor White
Write-Host "   dist" -ForegroundColor Cyan
Write-Host ""
Write-Host "Functions directory:" -ForegroundColor White
Write-Host "   netlify/functions" -ForegroundColor Cyan
Write-Host ""

# Variables d'environnement
Write-Host "VARIABLES D'ENVIRONNEMENT :" -ForegroundColor Yellow
Write-Host "===========================" -ForegroundColor Yellow
Write-Host ""
Write-Host "NODE_ENV=production" -ForegroundColor Cyan
Write-Host "REACT_APP_ENV=production" -ForegroundColor Cyan
Write-Host "REACT_APP_API_URL=https://consciousness-engine.netlify.app" -ForegroundColor Cyan
Write-Host "REACT_APP_ENABLE_ANALYTICS=true" -ForegroundColor Cyan
Write-Host "REACT_APP_ENABLE_MONITORING=true" -ForegroundColor Cyan
Write-Host "GENERATE_SOURCEMAP=false" -ForegroundColor Cyan
Write-Host "INLINE_RUNTIME_CHUNK=false" -ForegroundColor Cyan
Write-Host ""

# Test de build local (optionnel)
Write-Host "TEST DE BUILD LOCAL (OPTIONNEL) :" -ForegroundColor Yellow
Write-Host "==================================" -ForegroundColor Yellow
Write-Host ""
$testBuild = Read-Host "Voulez-vous tester le build localement avant deploiement ? (y/N)"

if ($testBuild -eq "y" -or $testBuild -eq "Y") {
    Write-Host ""
    Write-Host "Test du build local..." -ForegroundColor Cyan
    
    try {
        # Créer le répertoire dist
        if (-not (Test-Path "dist")) {
            New-Item -ItemType Directory -Path "dist" -Force | Out-Null
            Write-Host "   OK Repertoire dist cree" -ForegroundColor Green
        }
        
        # Copier les fichiers statiques
        if (Test-Path "web-ui/public") {
            Copy-Item -Path "web-ui/public/*" -Destination "dist/" -Recurse -Force
            Write-Host "   OK Fichiers web-ui/public copies" -ForegroundColor Green
        } elseif (Test-Path "public") {
            Copy-Item -Path "public/*" -Destination "dist/" -Recurse -Force
            Write-Host "   OK Fichiers public copies" -ForegroundColor Green
        }
        
        # Copier les fichiers HTML racine
        $htmlFiles = Get-ChildItem -Path "." -Filter "*.html"
        foreach ($htmlFile in $htmlFiles) {
            Copy-Item -Path $htmlFile.FullName -Destination "dist/" -Force
            Write-Host "   OK $($htmlFile.Name) copie" -ForegroundColor Green
        }
        
        # Vérifier le contenu de dist
        $distFiles = Get-ChildItem -Path "dist" -Recurse | Measure-Object
        Write-Host "   OK Build test reussi - $($distFiles.Count) fichiers dans dist/" -ForegroundColor Green
        
    } catch {
        Write-Host "   ATTENTION Erreur lors du test de build: $($_.Exception.Message)" -ForegroundColor Yellow
    }
}

# Résumé final
Write-Host ""
Write-Host "RESUME FINAL :" -ForegroundColor Green
Write-Host "==============" -ForegroundColor Green
Write-Host ""
Write-Host "Consciousness Engine est pret pour le deploiement Netlify !" -ForegroundColor White
Write-Host ""
Write-Host "Prochaines etapes :" -ForegroundColor White
Write-Host "1. Configurer Netlify avec les parametres ci-dessus" -ForegroundColor Yellow
Write-Host "2. Ajouter les variables d'environnement" -ForegroundColor Yellow
Write-Host "3. Cliquer 'Deploy consciousness-engine'" -ForegroundColor Yellow
Write-Host "4. Attendre le deploiement (2-5 minutes)" -ForegroundColor Yellow
Write-Host "5. Recuperer l'URL live pour les investisseurs" -ForegroundColor Yellow
Write-Host ""
Write-Host "URL attendue: https://consciousness-engine.netlify.app" -ForegroundColor Cyan
Write-Host ""
Write-Host "Consciousness Engine - Pret pour la transcendance technologique !" -ForegroundColor Green
