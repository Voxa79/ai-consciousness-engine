# Script GitHub Desktop - Solution la plus simple
# Déploiement Consciousness Engine via interface graphique

Write-Host "GITHUB DESKTOP - SOLUTION SIMPLE" -ForegroundColor Green
Write-Host "=================================" -ForegroundColor Green

Write-Host ""
Write-Host "Cette solution utilise GitHub Desktop pour un deploiement simple et fiable." -ForegroundColor White
Write-Host ""

# Étape 1 : Vérifier si GitHub Desktop est installé
Write-Host "Etape 1: Verification de GitHub Desktop..." -ForegroundColor Cyan

$githubDesktopPath = "$env:LOCALAPPDATA\GitHubDesktop\GitHubDesktop.exe"
if (Test-Path $githubDesktopPath) {
    Write-Host "   OK GitHub Desktop detecte" -ForegroundColor Green
} else {
    Write-Host "   GitHub Desktop non detecte" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "INSTALLATION DE GITHUB DESKTOP :" -ForegroundColor Yellow
    Write-Host "================================" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "1. Telecharger GitHub Desktop" -ForegroundColor White
    Write-Host "2. Installer et se connecter avec votre compte GitHub" -ForegroundColor White
    Write-Host "3. Revenir a ce script" -ForegroundColor White
    Write-Host ""
    
    $installGitHubDesktop = Read-Host "Ouvrir la page de telechargement GitHub Desktop ? (y/N)"
    if ($installGitHubDesktop -eq "y" -or $installGitHubDesktop -eq "Y") {
        Start-Process "https://desktop.github.com/"
        Write-Host "   OK Page de telechargement ouverte" -ForegroundColor Green
        Write-Host ""
        Read-Host "Appuyez sur Entree APRES avoir installe GitHub Desktop"
    }
}

# Étape 2 : Préparer le repository local
Write-Host ""
Write-Host "Etape 2: Preparation du repository local..." -ForegroundColor Cyan

# Initialiser Git si nécessaire
if (-not (Test-Path ".git")) {
    git init
    Write-Host "   OK Repository Git initialise" -ForegroundColor Green
}

# Configurer Git
git config user.name "Voxa79"
git config user.email "voxagents@pm.me"
Write-Host "   OK Git configure" -ForegroundColor Green

# Créer un commit si nécessaire
$gitStatus = git status --porcelain
if ($gitStatus) {
    git add .
    git commit -m "🌌 Consciousness Engine - Déploiement complet

✨ Plateforme de Transcendance Technologique
📊 427 fichiers • 155,737 lignes de code
🧠 IA Consciente + Quantique + Nano + Spatial
🔒 Sécurité Enterprise
💰 Prêt pour levée de fonds Série A

🚀 Prêt pour déploiement Netlify !"
    Write-Host "   OK Commit cree" -ForegroundColor Green
} else {
    Write-Host "   OK Repository deja a jour" -ForegroundColor Green
}

# Étape 3 : Instructions GitHub Desktop
Write-Host ""
Write-Host "Etape 3: Deploiement via GitHub Desktop..." -ForegroundColor Cyan
Write-Host ""
Write-Host "INSTRUCTIONS GITHUB DESKTOP :" -ForegroundColor Yellow
Write-Host "=============================" -ForegroundColor Yellow
Write-Host ""
Write-Host "1. Ouvrir GitHub Desktop" -ForegroundColor White
Write-Host ""
Write-Host "2. Cliquer 'File' > 'Add Local Repository'" -ForegroundColor White
Write-Host ""
Write-Host "3. Selectionner ce dossier :" -ForegroundColor White
Write-Host "   $PWD" -ForegroundColor Cyan
Write-Host ""
Write-Host "4. Cliquer 'Add Repository'" -ForegroundColor White
Write-Host ""
Write-Host "5. Cliquer 'Publish repository'" -ForegroundColor White
Write-Host ""
Write-Host "6. Configurer le repository :" -ForegroundColor White
Write-Host "   - Name: consciousness-engine" -ForegroundColor Cyan
Write-Host "   - Description: 🌌 Consciousness Engine - Plateforme de Transcendance Technologique" -ForegroundColor Cyan
Write-Host "   - Keep this code private: DECOCHER (public)" -ForegroundColor Cyan
Write-Host ""
Write-Host "7. Cliquer 'Publish Repository'" -ForegroundColor White
Write-Host ""
Write-Host "8. Attendre la fin du push (peut prendre 2-5 minutes)" -ForegroundColor White
Write-Host ""

# Ouvrir GitHub Desktop
$openGitHubDesktop = Read-Host "Ouvrir GitHub Desktop maintenant ? (y/N)"
if ($openGitHubDesktop -eq "y" -or $openGitHubDesktop -eq "Y") {
    if (Test-Path $githubDesktopPath) {
        Start-Process $githubDesktopPath
        Write-Host "   OK GitHub Desktop lance" -ForegroundColor Green
    } else {
        # Essayer d'autres chemins
        try {
            Start-Process "github"
            Write-Host "   OK GitHub Desktop lance" -ForegroundColor Green
        } catch {
            Write-Host "   Veuillez ouvrir GitHub Desktop manuellement" -ForegroundColor Yellow
        }
    }
}

Write-Host ""
Read-Host "Appuyez sur Entree APRES avoir publie le repository sur GitHub"

# Étape 4 : Vérification et prochaines étapes
Write-Host ""
Write-Host "VERIFICATION ET PROCHAINES ETAPES :" -ForegroundColor Green
Write-Host "====================================" -ForegroundColor Green
Write-Host ""

# Vérifier si le repository existe
Write-Host "Verification du repository GitHub..." -ForegroundColor Cyan
$repoUrl = "https://github.com/Voxa79/consciousness-engine"

try {
    $response = Invoke-WebRequest -Uri $repoUrl -Method Head -ErrorAction Stop
    Write-Host "   OK Repository accessible: $repoUrl" -ForegroundColor Green
    
    # Ouvrir le repository
    $openRepo = Read-Host "Ouvrir le repository sur GitHub ? (y/N)"
    if ($openRepo -eq "y" -or $openRepo -eq "Y") {
        Start-Process $repoUrl
        Write-Host "   OK Repository ouvert" -ForegroundColor Green
    }
    
} catch {
    Write-Host "   ATTENTION Repository non accessible ou non public" -ForegroundColor Yellow
    Write-Host "   Verifiez que le repository a ete publie correctement" -ForegroundColor Yellow
}

# Instructions Netlify
Write-Host ""
Write-Host "CONFIGURATION NETLIFY :" -ForegroundColor Yellow
Write-Host "======================" -ForegroundColor Yellow
Write-Host ""
Write-Host "Maintenant que le repository est sur GitHub :" -ForegroundColor White
Write-Host ""
Write-Host "1. Aller sur Netlify: https://app.netlify.com/start" -ForegroundColor Cyan
Write-Host ""
Write-Host "2. Cliquer 'Import from Git' > 'GitHub'" -ForegroundColor Cyan
Write-Host ""
Write-Host "3. Selectionner 'consciousness-engine'" -ForegroundColor Cyan
Write-Host ""
Write-Host "4. Configuration de build :" -ForegroundColor Cyan
Write-Host "   - Branch to deploy: main" -ForegroundColor White
Write-Host "   - Build command: npm install && npm run build" -ForegroundColor White
Write-Host "   - Publish directory: dist" -ForegroundColor White
Write-Host "   - Functions directory: netlify/functions" -ForegroundColor White
Write-Host ""
Write-Host "5. Variables d'environnement :" -ForegroundColor Cyan
Write-Host "   NODE_ENV=production" -ForegroundColor White
Write-Host "   REACT_APP_ENV=production" -ForegroundColor White
Write-Host ""
Write-Host "6. Cliquer 'Deploy site'" -ForegroundColor Cyan
Write-Host ""

# Ouvrir Netlify
$openNetlify = Read-Host "Ouvrir Netlify pour connecter le repository ? (y/N)"
if ($openNetlify -eq "y" -or $openNetlify -eq "Y") {
    Start-Process "https://app.netlify.com/start"
    Write-Host "   OK Netlify ouvert" -ForegroundColor Green
}

Write-Host ""
Write-Host "SUCCES ATTENDU :" -ForegroundColor Green
Write-Host "================" -ForegroundColor Green
Write-Host ""
Write-Host "Apres le deploiement Netlify :" -ForegroundColor White
Write-Host "- URL live: https://consciousness-engine.netlify.app" -ForegroundColor Cyan
Write-Host "- Deploiements automatiques a chaque push" -ForegroundColor Cyan
Write-Host "- HTTPS et CDN global automatiques" -ForegroundColor Cyan
Write-Host "- Pret pour presentation investisseurs !" -ForegroundColor Cyan
Write-Host ""
Write-Host "Consciousness Engine - Transcendance technologique realisee !" -ForegroundColor Green
