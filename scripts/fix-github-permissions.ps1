# Script de correction des permissions GitHub - Consciousness Engine

param(
    [string]$GitHubUsername = "Voxa79",
    [string]$GitHubEmail = "voxagents@pm.me",
    [string]$RepoName = "consciousness-engine"
)

Write-Host "CORRECTION PERMISSIONS GITHUB - CONSCIOUSNESS ENGINE" -ForegroundColor Green
Write-Host "====================================================" -ForegroundColor Green

$RepoUrl = "https://github.com/$GitHubUsername/$RepoName.git"

Write-Host "Configuration :" -ForegroundColor Yellow
Write-Host "   Username: $GitHubUsername" -ForegroundColor White
Write-Host "   Email: $GitHubEmail" -ForegroundColor White
Write-Host "   Repository: $RepoUrl" -ForegroundColor White

# Étape 1 : Reconfigurer Git avec les bons identifiants
Write-Host ""
Write-Host "Reconfiguration de Git..." -ForegroundColor Cyan

git config user.name $GitHubUsername
git config user.email $GitHubEmail

Write-Host "   OK Git reconfigure avec $GitHubUsername" -ForegroundColor Green

# Étape 2 : Vérifier la configuration
Write-Host ""
Write-Host "Verification de la configuration..." -ForegroundColor Cyan

$currentUser = git config user.name
$currentEmail = git config user.email

Write-Host "   Utilisateur: $currentUser" -ForegroundColor White
Write-Host "   Email: $currentEmail" -ForegroundColor White

# Étape 3 : Vérifier le remote
Write-Host ""
Write-Host "Verification du remote..." -ForegroundColor Cyan

$remotes = git remote -v
Write-Host "   Remotes configures:" -ForegroundColor White
$remotes | ForEach-Object { Write-Host "   $_" -ForegroundColor Gray }

# Étape 4 : Tentative de push
Write-Host ""
Write-Host "Tentative de push vers GitHub..." -ForegroundColor Cyan

try {
    git push -u origin main
    Write-Host "   OK Push reussi !" -ForegroundColor Green
    
    Write-Host ""
    Write-Host "SUCCES ! CONSCIOUSNESS ENGINE EST SUR GITHUB !" -ForegroundColor Green
    Write-Host "===============================================" -ForegroundColor Green
    Write-Host ""
    Write-Host "Repository : https://github.com/$GitHubUsername/$RepoName" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Prochaines etapes :" -ForegroundColor White
    Write-Host "   1. Connecter a Netlify via GitHub" -ForegroundColor Yellow
    Write-Host "   2. Deploiement automatique" -ForegroundColor Yellow
    Write-Host "   3. Presentation aux investisseurs" -ForegroundColor Yellow
    
} catch {
    Write-Host "   ERREUR lors du push" -ForegroundColor Red
    Write-Host ""
    Write-Host "SOLUTIONS ALTERNATIVES :" -ForegroundColor Yellow
    Write-Host "========================" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "1. AUTHENTIFICATION PAR TOKEN :" -ForegroundColor White
    Write-Host "   - Aller sur : https://github.com/settings/tokens" -ForegroundColor Cyan
    Write-Host "   - Generer un Personal Access Token" -ForegroundColor Cyan
    Write-Host "   - Utiliser le token comme mot de passe" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "2. AUTHENTIFICATION SSH :" -ForegroundColor White
    Write-Host "   - Configurer une cle SSH" -ForegroundColor Cyan
    Write-Host "   - Changer l'URL remote vers SSH" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "3. CREER LE REPOSITORY MANUELLEMENT :" -ForegroundColor White
    Write-Host "   - Aller sur : https://github.com/new" -ForegroundColor Cyan
    Write-Host "   - Creer le repository 'consciousness-engine'" -ForegroundColor Cyan
    Write-Host "   - Puis relancer ce script" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "4. UTILISER GITHUB DESKTOP :" -ForegroundColor White
    Write-Host "   - Installer GitHub Desktop" -ForegroundColor Cyan
    Write-Host "   - Se connecter avec votre compte" -ForegroundColor Cyan
    Write-Host "   - Publier le repository local" -ForegroundColor Cyan
}

Write-Host ""
Write-Host "Consciousness Engine - Pret pour la transcendance technologique !" -ForegroundColor Green
