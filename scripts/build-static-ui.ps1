#!/usr/bin/env pwsh

# Script pour construire l'interface web en mode production
# Ce script résout les problèmes de rechargement en construisant une version statique
# sans aucune initialisation réseau au démarrage

# Couleurs pour l'affichage
$ESC = [char]27
$CYAN = "$ESC[36m"
$GREEN = "$ESC[32m"
$YELLOW = "$ESC[33m"
$RED = "$ESC[31m"
$NC = "$ESC[0m" # No Color

Write-Host "${CYAN}🧠 Consciousness Engine Web UI - Construction Statique${NC}"
Write-Host "${CYAN}=================================================${NC}"

# Vérifier si Node.js est installé
try {
    $nodeVersion = node --version
    Write-Host "${GREEN}✅ Node.js détecté: $nodeVersion${NC}"
} catch {
    Write-Host "${RED}❌ Node.js n'est pas installé ou n'est pas dans le PATH.${NC}"
    Write-Host "${YELLOW}💡 Veuillez installer Node.js depuis https://nodejs.org/${NC}"
    exit 1
}

# Vérifier si le dossier web-ui existe
if (-not (Test-Path -Path "./web-ui")) {
    Write-Host "${RED}❌ Le dossier web-ui n'existe pas.${NC}"
    exit 1
}

# Se déplacer dans le dossier web-ui
Push-Location -Path "./web-ui"

try {
    # Vérifier si package.json existe
    if (-not (Test-Path -Path "./package.json")) {
        Write-Host "${RED}❌ Le fichier package.json n'existe pas dans le dossier web-ui.${NC}"
        exit 1
    }

    # Installer les dépendances si node_modules n'existe pas
    if (-not (Test-Path -Path "./node_modules")) {
        Write-Host "${YELLOW}📦 Installation des dépendances...${NC}"
        npm install
        if ($LASTEXITCODE -ne 0) {
            Write-Host "${RED}❌ Échec de l'installation des dépendances.${NC}"
            exit 1
        }
        Write-Host "${GREEN}✅ Dépendances installées avec succès${NC}"
    } else {
        Write-Host "${GREEN}✅ Dépendances déjà installées${NC}"
    }

    # Construire l'application
    Write-Host "${YELLOW}🔨 Construction de l'application en mode production...${NC}"
    
    # Définir les variables d'environnement pour la construction
    $env:GENERATE_SOURCEMAP = "false"
    $env:REACT_APP_API_BASE = "http://localhost:3000"
    
    # Exécuter la commande de construction
    npm run build
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "${RED}❌ Échec de la construction de l'application.${NC}"
        exit 1
    }
    
    Write-Host "${GREEN}✅ Application construite avec succès${NC}"
    
    # Vérifier si le dossier build existe
    if (Test-Path -Path "./build") {
        Write-Host "${GREEN}✅ Le dossier build a été créé avec succès${NC}"
        Write-Host "${YELLOW}💡 Pour servir l'application, exécutez .\serve-static-ui.ps1${NC}"
    } else {
        Write-Host "${RED}❌ Le dossier build n'a pas été créé.${NC}"
        exit 1
    }
} finally {
    # Revenir au dossier précédent
    Pop-Location
}