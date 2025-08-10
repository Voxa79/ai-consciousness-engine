# Script de configuration SSH sécurisée pour GitHub - Consciousness Engine
# Solution la plus sécurisée avec authentification par clés cryptographiques

param(
    [string]$GitHubUsername = "Voxa79",
    [string]$GitHubEmail = "voxagents@pm.me",
    [string]$RepoName = "consciousness-engine",
    [string]$KeyName = "consciousness-engine-deploy"
)

Write-Host "CONFIGURATION SSH SECURISEE - CONSCIOUSNESS ENGINE" -ForegroundColor Green
Write-Host "==================================================" -ForegroundColor Green

Write-Host "Configuration :" -ForegroundColor Yellow
Write-Host "   Username: $GitHubUsername" -ForegroundColor White
Write-Host "   Email: $GitHubEmail" -ForegroundColor White
Write-Host "   Repository: $RepoName" -ForegroundColor White
Write-Host "   Nom de cle: $KeyName" -ForegroundColor White

# Étape 1 : Vérifier si SSH est disponible
Write-Host ""
Write-Host "Verification de SSH..." -ForegroundColor Cyan

try {
    $sshVersion = ssh -V 2>&1
    Write-Host "   OK SSH disponible: $sshVersion" -ForegroundColor Green
} catch {
    Write-Host "   ERREUR SSH non disponible" -ForegroundColor Red
    Write-Host "   Installez OpenSSH via Windows Features ou Git Bash" -ForegroundColor Yellow
    exit 1
}

# Étape 2 : Créer le répertoire .ssh si nécessaire
Write-Host ""
Write-Host "Preparation du repertoire SSH..." -ForegroundColor Cyan

$sshDir = "$env:USERPROFILE\.ssh"
if (-not (Test-Path $sshDir)) {
    New-Item -ItemType Directory -Path $sshDir -Force | Out-Null
    Write-Host "   OK Repertoire .ssh cree" -ForegroundColor Green
} else {
    Write-Host "   OK Repertoire .ssh existe" -ForegroundColor Green
}

# Étape 3 : Générer la paire de clés SSH
Write-Host ""
Write-Host "Generation de la paire de cles SSH..." -ForegroundColor Cyan

$keyPath = "$sshDir\$KeyName"
$publicKeyPath = "$keyPath.pub"

if (Test-Path $keyPath) {
    Write-Host "   ATTENTION Cle existante trouvee" -ForegroundColor Yellow
    $overwrite = Read-Host "Voulez-vous la remplacer ? (y/N)"
    if ($overwrite -ne "y" -and $overwrite -ne "Y") {
        Write-Host "   OK Utilisation de la cle existante" -ForegroundColor Green
    } else {
        Remove-Item $keyPath -Force -ErrorAction SilentlyContinue
        Remove-Item $publicKeyPath -Force -ErrorAction SilentlyContinue
    }
}

if (-not (Test-Path $keyPath)) {
    Write-Host "   Generation de la cle SSH (ED25519 - plus securisee)..." -ForegroundColor Yellow
    
    # Utiliser ED25519 pour une sécurité maximale
    ssh-keygen -t ed25519 -C "$GitHubEmail" -f "$keyPath" -N '""'
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "   OK Cle SSH generee avec succes" -ForegroundColor Green
    } else {
        Write-Host "   ERREUR lors de la generation de la cle" -ForegroundColor Red
        exit 1
    }
}

# Étape 4 : Configurer l'agent SSH
Write-Host ""
Write-Host "Configuration de l'agent SSH..." -ForegroundColor Cyan

# Démarrer l'agent SSH
$sshAgent = Get-Process ssh-agent -ErrorAction SilentlyContinue
if (-not $sshAgent) {
    Write-Host "   Demarrage de l'agent SSH..." -ForegroundColor Yellow
    Start-Service ssh-agent -ErrorAction SilentlyContinue
    ssh-agent | Out-Null
}

# Ajouter la clé à l'agent
ssh-add "$keyPath" 2>$null
if ($LASTEXITCODE -eq 0) {
    Write-Host "   OK Cle ajoutee a l'agent SSH" -ForegroundColor Green
} else {
    Write-Host "   ATTENTION Impossible d'ajouter la cle a l'agent" -ForegroundColor Yellow
}

# Étape 5 : Afficher la clé publique
Write-Host ""
Write-Host "CLE PUBLIQUE A AJOUTER SUR GITHUB :" -ForegroundColor Yellow
Write-Host "===================================" -ForegroundColor Yellow

if (Test-Path $publicKeyPath) {
    $publicKey = Get-Content $publicKeyPath -Raw
    Write-Host $publicKey -ForegroundColor Cyan
    
    # Copier dans le presse-papiers si possible
    try {
        $publicKey | Set-Clipboard
        Write-Host ""
        Write-Host "OK Cle publique copiee dans le presse-papiers" -ForegroundColor Green
    } catch {
        Write-Host ""
        Write-Host "ATTENTION Impossible de copier dans le presse-papiers" -ForegroundColor Yellow
    }
} else {
    Write-Host "ERREUR Cle publique non trouvee" -ForegroundColor Red
    exit 1
}

# Étape 6 : Instructions pour GitHub
Write-Host ""
Write-Host "ETAPES SUIVANTES - CONFIGURATION GITHUB :" -ForegroundColor Yellow
Write-Host "=========================================" -ForegroundColor Yellow
Write-Host ""
Write-Host "1. Aller sur GitHub SSH Settings :" -ForegroundColor White
Write-Host "   https://github.com/settings/ssh/new" -ForegroundColor Cyan
Write-Host ""
Write-Host "2. Coller la cle publique ci-dessus dans le champ 'Key'" -ForegroundColor White
Write-Host ""
Write-Host "3. Donner un titre : 'Consciousness Engine Deploy Key'" -ForegroundColor White
Write-Host ""
Write-Host "4. Cliquer 'Add SSH key'" -ForegroundColor White
Write-Host ""
Write-Host "5. Revenir ici et appuyer sur Entree pour continuer..." -ForegroundColor White

Read-Host "Appuyez sur Entree apres avoir ajoute la cle sur GitHub"

# Étape 7 : Tester la connexion SSH
Write-Host ""
Write-Host "Test de la connexion SSH vers GitHub..." -ForegroundColor Cyan

$testResult = ssh -T git@github.com 2>&1
if ($testResult -match "successfully authenticated") {
    Write-Host "   OK Connexion SSH reussie !" -ForegroundColor Green
} else {
    Write-Host "   ATTENTION Test de connexion:" -ForegroundColor Yellow
    Write-Host "   $testResult" -ForegroundColor Gray
}

# Étape 8 : Reconfigurer Git pour SSH
Write-Host ""
Write-Host "Configuration de Git pour SSH..." -ForegroundColor Cyan

# Configurer l'utilisateur Git
git config user.name $GitHubUsername
git config user.email $GitHubEmail
Write-Host "   OK Git configure avec $GitHubUsername" -ForegroundColor Green

# Changer l'URL remote vers SSH
$sshUrl = "git@github.com:$GitHubUsername/$RepoName.git"
git remote set-url origin $sshUrl
Write-Host "   OK Remote configure pour SSH: $sshUrl" -ForegroundColor Green

# Étape 9 : Tentative de push sécurisé
Write-Host ""
Write-Host "Push securise vers GitHub via SSH..." -ForegroundColor Cyan

try {
    git push -u origin main
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "   OK Push SSH reussi !" -ForegroundColor Green
        
        # Succès complet
        Write-Host ""
        Write-Host "SUCCES ! CONSCIOUSNESS ENGINE DEPLOYE SECURISE !" -ForegroundColor Green
        Write-Host "================================================" -ForegroundColor Green
        Write-Host ""
        Write-Host "Repository securise : https://github.com/$GitHubUsername/$RepoName" -ForegroundColor Cyan
        Write-Host "Authentification : SSH ED25519 (cryptographie forte)" -ForegroundColor Cyan
        Write-Host ""
        Write-Host "Prochaines etapes :" -ForegroundColor White
        Write-Host "   1. Connecter a Netlify via GitHub" -ForegroundColor Yellow
        Write-Host "   2. Deploiement automatique securise" -ForegroundColor Yellow
        Write-Host "   3. Presentation aux investisseurs" -ForegroundColor Yellow
        Write-Host ""
        Write-Host "Securite :" -ForegroundColor White
        Write-Host "   - Authentification SSH ED25519" -ForegroundColor Green
        Write-Host "   - Cles cryptographiques 256-bit" -ForegroundColor Green
        Write-Host "   - Aucun mot de passe transmis" -ForegroundColor Green
        Write-Host "   - Protection contre MITM" -ForegroundColor Green
        
        # Ouvrir GitHub
        $openGitHub = Read-Host "Ouvrir le repository securise sur GitHub ? (y/N)"
        if ($openGitHub -eq "y" -or $openGitHub -eq "Y") {
            Start-Process "https://github.com/$GitHubUsername/$RepoName"
            Write-Host "OK GitHub ouvert dans le navigateur" -ForegroundColor Green
        }
        
    } else {
        throw "Erreur lors du push"
    }
    
} catch {
    Write-Host "   ERREUR lors du push SSH" -ForegroundColor Red
    Write-Host ""
    Write-Host "DIAGNOSTIC :" -ForegroundColor Yellow
    Write-Host "============" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "1. Verifiez que la cle SSH est ajoutee sur GitHub" -ForegroundColor White
    Write-Host "2. Testez la connexion : ssh -T git@github.com" -ForegroundColor White
    Write-Host "3. Verifiez que le repository existe sur GitHub" -ForegroundColor White
    Write-Host ""
    Write-Host "Solutions :" -ForegroundColor White
    Write-Host "- Creer le repository manuellement sur GitHub" -ForegroundColor Cyan
    Write-Host "- Verifier les permissions de la cle SSH" -ForegroundColor Cyan
    Write-Host "- Relancer ce script apres correction" -ForegroundColor Cyan
}

Write-Host ""
Write-Host "Consciousness Engine - Deploiement securise pour la transcendance !" -ForegroundColor Green
