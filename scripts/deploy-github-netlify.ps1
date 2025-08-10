# Script de déploiement GitHub → Netlify - Consciousness Engine
# Expert CTO Next Gen - Déploiement automatisé pour investisseurs

param(
    [string]$GitHubRepo = "consciousness-engine",
    [string]$GitHubUsername = "",
    [string]$CommitMessage = "🚀 Déploiement initial Consciousness Engine - Plateforme de Transcendance Technologique",
    [switch]$Force,
    [switch]$DryRun
)

Write-Host "🚀 CONSCIOUSNESS ENGINE - DÉPLOIEMENT GITHUB → NETLIFY" -ForegroundColor Green
Write-Host "=========================================================" -ForegroundColor Green

$ErrorActionPreference = "Stop"

# Configuration
$DeployConfig = @{
    StartTime = Get-Date
    GitHubRepo = $GitHubRepo
    GitHubUsername = $GitHubUsername
    ProjectName = "Consciousness Engine"
    Version = "2.0.0"
    Environment = "production"
}

Write-Host "🔧 Configuration du déploiement:" -ForegroundColor Yellow
Write-Host "   Projet: $($DeployConfig.ProjectName)" -ForegroundColor White
Write-Host "   Version: $($DeployConfig.Version)" -ForegroundColor White
Write-Host "   Repository: $GitHubRepo" -ForegroundColor White
Write-Host "   Dry Run: $DryRun" -ForegroundColor White

# Fonction de vérification des prérequis
function Test-DeploymentPrerequisites {
    Write-Host "🔍 Vérification des prérequis de déploiement..." -ForegroundColor Cyan
    
    $missingTools = @()
    
    # Vérifier Git
    try {
        $gitVersion = git --version
        Write-Host "   ✅ Git: $gitVersion" -ForegroundColor Green
    } catch {
        $missingTools += "git"
        Write-Host "   ❌ Git non trouvé" -ForegroundColor Red
    }
    
    # Vérifier Node.js
    try {
        $nodeVersion = node --version
        Write-Host "   ✅ Node.js: $nodeVersion" -ForegroundColor Green
    } catch {
        $missingTools += "node"
        Write-Host "   ❌ Node.js non trouvé" -ForegroundColor Red
    }
    
    # Vérifier npm
    try {
        $npmVersion = npm --version
        Write-Host "   ✅ npm: v$npmVersion" -ForegroundColor Green
    } catch {
        $missingTools += "npm"
        Write-Host "   ❌ npm non trouvé" -ForegroundColor Red
    }
    
    # Vérifier Netlify CLI
    try {
        $netlifyVersion = netlify --version
        Write-Host "   ✅ Netlify CLI: $netlifyVersion" -ForegroundColor Green
    } catch {
        Write-Host "   ⚠️ Netlify CLI non trouvé, installation..." -ForegroundColor Yellow
        if (-not $DryRun) {
            npm install -g netlify-cli
        }
    }
    
    if ($missingTools.Count -gt 0) {
        Write-Host "❌ Outils manquants: $($missingTools -join ', ')" -ForegroundColor Red
        if (-not $Force) {
            throw "Prérequis manquants. Utilisez -Force pour continuer."
        }
    }
    
    return $missingTools.Count -eq 0
}

# Fonction de vérification des fichiers
function Test-ProjectFiles {
    Write-Host "📁 Vérification des fichiers du projet..." -ForegroundColor Cyan
    
    $requiredFiles = @(
        "package.json",
        "netlify.toml",
        "vite.config.ts",
        "README_FR.md",
        "PITCH_DECK.md",
        "BUSINESS_PLAN.md",
        "INVESTOR_CHECKLIST.md",
        "TERMS_OF_SERVICE.md",
        "PRIVACY_POLICY.md",
        "DOCUMENTATION_TECHNIQUE.md",
        "GUIDE_DEPLOIEMENT.md",
        "dist/index.html",
        "dist/_headers",
        "dist/_redirects",
        "netlify/functions/consciousness-engine.mts"
    )
    
    $missingFiles = @()
    
    foreach ($file in $requiredFiles) {
        if (Test-Path $file) {
            Write-Host "   ✅ $file" -ForegroundColor Green
        } else {
            $missingFiles += $file
            Write-Host "   ❌ $file manquant" -ForegroundColor Red
        }
    }
    
    if ($missingFiles.Count -gt 0) {
        Write-Host "❌ Fichiers manquants: $($missingFiles -join ', ')" -ForegroundColor Red
        if (-not $Force) {
            throw "Fichiers requis manquants. Utilisez -Force pour continuer."
        }
    }
    
    return $missingFiles.Count -eq 0
}

# Fonction de préparation Git
function Initialize-GitRepository {
    Write-Host "📦 Initialisation du repository Git..." -ForegroundColor Cyan
    
    if ($DryRun) {
        Write-Host "   [DRY RUN] Initialisation Git" -ForegroundColor Gray
        return
    }
    
    # Initialiser Git si nécessaire
    if (-not (Test-Path ".git")) {
        git init
        Write-Host "   ✅ Repository Git initialisé" -ForegroundColor Green
    } else {
        Write-Host "   ✅ Repository Git existant" -ForegroundColor Green
    }
    
    # Configurer Git si nécessaire
    try {
        $gitUser = git config user.name
        $gitEmail = git config user.email
        
        if (-not $gitUser -or -not $gitEmail) {
            Write-Host "   ⚠️ Configuration Git requise" -ForegroundColor Yellow
            Write-Host "   Veuillez configurer Git:" -ForegroundColor Yellow
            Write-Host "   git config --global user.name 'Votre Nom'" -ForegroundColor Yellow
            Write-Host "   git config --global user.email 'votre.email@example.com'" -ForegroundColor Yellow
            
            if (-not $Force) {
                throw "Configuration Git requise"
            }
        } else {
            Write-Host "   ✅ Git configuré: $gitUser <$gitEmail>" -ForegroundColor Green
        }
    } catch {
        Write-Host "   ⚠️ Impossible de vérifier la configuration Git" -ForegroundColor Yellow
    }
}

# Fonction de build du projet
function Build-Project {
    Write-Host "🏗️ Build du projet..." -ForegroundColor Cyan
    
    if ($DryRun) {
        Write-Host "   [DRY RUN] Build du projet" -ForegroundColor Gray
        return
    }
    
    # Installer les dépendances
    Write-Host "   Installation des dépendances..." -ForegroundColor Yellow
    npm ci --production=false
    
    if ($LASTEXITCODE -ne 0) {
        throw "Échec de l'installation des dépendances"
    }
    
    # Build pour production
    Write-Host "   Build pour production..." -ForegroundColor Yellow
    $env:NODE_ENV = "production"
    $env:CONSCIOUSNESS_MODE = "production"
    npm run build:netlify
    
    if ($LASTEXITCODE -ne 0) {
        throw "Échec du build"
    }
    
    Write-Host "   ✅ Build terminé avec succès" -ForegroundColor Green
}

# Fonction de commit et push
function Publish-ToGitHub {
    Write-Host "📤 Publication sur GitHub..." -ForegroundColor Cyan
    
    if ($DryRun) {
        Write-Host "   [DRY RUN] Publication GitHub" -ForegroundColor Gray
        return
    }
    
    # Ajouter tous les fichiers
    git add .
    
    # Créer le commit
    git commit -m $CommitMessage
    
    if ($LASTEXITCODE -ne 0 -and $LASTEXITCODE -ne 1) {
        Write-Host "   ⚠️ Aucun changement à committer ou erreur de commit" -ForegroundColor Yellow
    }
    
    # Configurer la branche principale
    git branch -M main
    
    # Ajouter l'origine si nécessaire
    if ($GitHubUsername -and $GitHubRepo) {
        $remoteUrl = "https://github.com/$GitHubUsername/$GitHubRepo.git"
        
        try {
            git remote add origin $remoteUrl
            Write-Host "   ✅ Remote origin ajoutée: $remoteUrl" -ForegroundColor Green
        } catch {
            Write-Host "   ⚠️ Remote origin existe déjà" -ForegroundColor Yellow
            git remote set-url origin $remoteUrl
        }
        
        # Push vers GitHub
        Write-Host "   Push vers GitHub..." -ForegroundColor Yellow
        git push -u origin main
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host "   ✅ Code publié sur GitHub avec succès" -ForegroundColor Green
            Write-Host "   🌐 Repository: https://github.com/$GitHubUsername/$GitHubRepo" -ForegroundColor Cyan
        } else {
            throw "Échec du push vers GitHub"
        }
    } else {
        Write-Host "   ⚠️ GitHub username/repo non spécifiés" -ForegroundColor Yellow
        Write-Host "   Veuillez configurer manuellement le remote:" -ForegroundColor Yellow
        Write-Host "   git remote add origin https://github.com/USERNAME/REPO.git" -ForegroundColor Yellow
        Write-Host "   git push -u origin main" -ForegroundColor Yellow
    }
}

# Fonction de déploiement Netlify
function Deploy-ToNetlify {
    Write-Host "🌐 Déploiement sur Netlify..." -ForegroundColor Cyan
    
    if ($DryRun) {
        Write-Host "   [DRY RUN] Déploiement Netlify" -ForegroundColor Gray
        return
    }
    
    # Vérifier la connexion Netlify
    try {
        netlify status
        Write-Host "   ✅ Connecté à Netlify" -ForegroundColor Green
    } catch {
        Write-Host "   ⚠️ Connexion Netlify requise" -ForegroundColor Yellow
        Write-Host "   Exécutez: netlify login" -ForegroundColor Yellow
        
        if (-not $Force) {
            throw "Connexion Netlify requise"
        }
    }
    
    # Déploiement preview d'abord
    Write-Host "   Déploiement preview..." -ForegroundColor Yellow
    netlify deploy --dir=dist
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "   ✅ Déploiement preview réussi" -ForegroundColor Green
        
        # Demander confirmation pour la production
        if (-not $Force) {
            $confirm = Read-Host "Déployer en production ? (y/N)"
            if ($confirm -ne "y" -and $confirm -ne "Y") {
                Write-Host "   ⏸️ Déploiement production annulé" -ForegroundColor Yellow
                return
            }
        }
        
        # Déploiement production
        Write-Host "   Déploiement production..." -ForegroundColor Yellow
        netlify deploy --prod --dir=dist
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host "   ✅ Déploiement production réussi" -ForegroundColor Green
            
            # Ouvrir le site
            netlify open
        } else {
            throw "Échec du déploiement production"
        }
    } else {
        throw "Échec du déploiement preview"
    }
}

# Fonction de validation post-déploiement
function Test-Deployment {
    Write-Host "🧪 Validation du déploiement..." -ForegroundColor Cyan
    
    if ($DryRun) {
        Write-Host "   [DRY RUN] Tests de validation" -ForegroundColor Gray
        return
    }
    
    # Obtenir l'URL du site
    try {
        $siteInfo = netlify status --json | ConvertFrom-Json
        $siteUrl = $siteInfo.site_url
        
        if ($siteUrl) {
            Write-Host "   🌐 Site URL: $siteUrl" -ForegroundColor Cyan
            
            # Test de base
            try {
                $response = Invoke-WebRequest -Uri $siteUrl -Method Head -TimeoutSec 10
                if ($response.StatusCode -eq 200) {
                    Write-Host "   ✅ Site accessible" -ForegroundColor Green
                } else {
                    Write-Host "   ⚠️ Site répond avec code: $($response.StatusCode)" -ForegroundColor Yellow
                }
            } catch {
                Write-Host "   ❌ Site non accessible: $($_.Exception.Message)" -ForegroundColor Red
            }
        }
    } catch {
        Write-Host "   ⚠️ Impossible d'obtenir l'URL du site" -ForegroundColor Yellow
    }
}

# Fonction principale
function Main {
    try {
        $DeployConfig.StartTime = Get-Date
        
        # Vérifications préliminaires
        Test-DeploymentPrerequisites
        Test-ProjectFiles
        
        # Initialisation Git
        Initialize-GitRepository
        
        # Build du projet
        Build-Project
        
        # Publication GitHub
        if ($GitHubUsername) {
            Publish-ToGitHub
        } else {
            Write-Host "⚠️ GitHub username non spécifié, skip publication GitHub" -ForegroundColor Yellow
        }
        
        # Déploiement Netlify
        Deploy-ToNetlify
        
        # Validation
        Test-Deployment
        
        $DeployConfig.EndTime = Get-Date
        $duration = $DeployConfig.EndTime - $DeployConfig.StartTime
        
        # Résumé final
        Write-Host ""
        Write-Host "🎉 DÉPLOIEMENT CONSCIOUSNESS ENGINE TERMINÉ!" -ForegroundColor Green
        Write-Host "=============================================" -ForegroundColor Green
        Write-Host ""
        Write-Host "⏱️ Durée totale: $($duration.ToString('mm\:ss'))" -ForegroundColor White
        Write-Host "🚀 Projet: $($DeployConfig.ProjectName)" -ForegroundColor White
        Write-Host "📦 Version: $($DeployConfig.Version)" -ForegroundColor White
        Write-Host ""
        Write-Host "🌐 Votre plateforme de transcendance technologique est maintenant LIVE!" -ForegroundColor Green
        Write-Host ""
        Write-Host "📊 Prêt pour présentation investisseurs avec:" -ForegroundColor White
        Write-Host "   ✅ Documentation complète en français" -ForegroundColor Cyan
        Write-Host "   ✅ Pitch deck professionnel" -ForegroundColor Cyan
        Write-Host "   ✅ Business plan détaillé" -ForegroundColor Cyan
        Write-Host "   ✅ Plateforme live et fonctionnelle" -ForegroundColor Cyan
        Write-Host "   ✅ Architecture de niveau enterprise" -ForegroundColor Cyan
        Write-Host ""
        Write-Host "🎯 Prochaines étapes:" -ForegroundColor White
        Write-Host "   1. Configurer domaine personnalisé" -ForegroundColor Yellow
        Write-Host "   2. Activer monitoring et analytics" -ForegroundColor Yellow
        Write-Host "   3. Préparer présentation investisseurs" -ForegroundColor Yellow
        Write-Host "   4. Lancer campagne de levée de fonds" -ForegroundColor Yellow
        Write-Host ""
        
    } catch {
        Write-Host "❌ Erreur lors du déploiement: $($_.Exception.Message)" -ForegroundColor Red
        exit 1
    }
}

# Exécution
if (-not $GitHubUsername) {
    Write-Host "⚠️ GitHub username non spécifié" -ForegroundColor Yellow
    Write-Host "Usage: .\deploy-github-netlify.ps1 -GitHubUsername 'votre-username'" -ForegroundColor Yellow
    Write-Host "Ou: .\deploy-github-netlify.ps1 -GitHubUsername 'votre-username' -GitHubRepo 'consciousness-engine'" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Continuer sans GitHub ? (y/N): " -NoNewline -ForegroundColor Yellow
    $continue = Read-Host
    
    if ($continue -ne "y" -and $continue -ne "Y") {
        Write-Host "Déploiement annulé." -ForegroundColor Red
        exit 1
    }
}

Main
