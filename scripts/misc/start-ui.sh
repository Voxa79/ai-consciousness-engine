#!/bin/bash

# Script de démarrage pour Consciousness Engine Web UI
# Démarre l'interface utilisateur avec toutes les optimisations

set -e

# Couleurs pour l'affichage
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${CYAN}🧠 Consciousness Engine Web UI${NC}"
echo -e "${CYAN}================================${NC}"

# Fonction d'affichage avec timestamp
log() {
    echo -e "${GREEN}[$(date +'%Y-%m-%d %H:%M:%S')] $1${NC}"
}

error() {
    echo -e "${RED}[$(date +'%Y-%m-%d %H:%M:%S')] ERROR: $1${NC}"
    exit 1
}

warning() {
    echo -e "${YELLOW}[$(date +'%Y-%m-%d %H:%M:%S')] WARNING: $1${NC}"
}

# Vérification des prérequis
check_prerequisites() {
    log "Checking prerequisites..."
    
    # Vérifier Node.js
    if ! command -v node &> /dev/null; then
        error "Node.js is not installed. Please install Node.js 18+ from https://nodejs.org/"
    fi
    
    # Vérifier la version de Node.js
    NODE_VERSION=$(node --version | cut -d'v' -f2 | cut -d'.' -f1)
    if [ "$NODE_VERSION" -lt 16 ]; then
        error "Node.js version 16+ is required. Current version: $(node --version)"
    fi
    
    # Vérifier npm
    if ! command -v npm &> /dev/null; then
        error "npm is not installed"
    fi
    
    log "Prerequisites check completed ✅"
}

# Installation des dépendances
install_dependencies() {
    log "Installing dependencies..."
    
    if [ ! -d "node_modules" ]; then
        log "Installing npm packages..."
        npm install
    else
        log "Checking for package updates..."
        npm ci
    fi
    
    log "Dependencies installed ✅"
}

# Configuration de l'environnement
setup_environment() {
    log "Setting up environment..."
    
    # Créer le fichier .env s'il n'existe pas
    if [ ! -f ".env" ]; then
        log "Creating .env file..."
        cat > .env << EOF
# Consciousness Engine Web UI Environment Configuration

# API Configuration
REACT_APP_API_URL=http://localhost:8000
REACT_APP_WS_URL=ws://localhost:8000/ws

# Application Configuration
REACT_APP_VERSION=1.0.0
REACT_APP_ENVIRONMENT=development

# Feature Flags
REACT_APP_ENABLE_QUANTUM=true
REACT_APP_ENABLE_NEUROMORPHIC=true
REACT_APP_ENABLE_ANALYTICS=true

# Debug Configuration
REACT_APP_DEBUG=false
REACT_APP_LOG_LEVEL=info

# Performance Configuration
GENERATE_SOURCEMAP=true
INLINE_RUNTIME_CHUNK=false
EOF
        log ".env file created ✅"
    else
        log ".env file already exists ✅"
    fi
    
    # Créer le dossier public s'il n'existe pas
    if [ ! -d "public" ]; then
        mkdir -p public
        log "Created public directory ✅"
    fi
    
    log "Environment setup completed ✅"
}

# Vérification de la santé de l'API
check_api_health() {
    log "Checking API health..."
    
    API_URL="${REACT_APP_API_URL:-http://localhost:8000}"
    
    if curl -f "$API_URL/health" &>/dev/null; then
        log "API is healthy ✅"
    else
        warning "API is not responding at $API_URL"
        warning "Make sure the Consciousness Engine API is running"
        warning "You can start it with: ./deploy.sh"
    fi
}

# Démarrage du serveur de développement
start_development_server() {
    log "Starting development server..."
    
    # Variables d'environnement pour optimisation
    export FAST_REFRESH=true
    export ESLINT_NO_DEV_ERRORS=true
    export TSC_COMPILE_ON_ERROR=true
    export GENERATE_SOURCEMAP=true
    
    log "🚀 Starting Consciousness Engine Web UI..."
    log "   Local:   http://localhost:3000"
    log "   Network: http://$(hostname -I | awk '{print $1}'):3000"
    log ""
    log "Press Ctrl+C to stop the server"
    
    # Démarrer le serveur React
    npm start
}

# Build pour production
build_production() {
    log "Building for production..."
    
    # Variables d'environnement pour production
    export NODE_ENV=production
    export GENERATE_SOURCEMAP=false
    export INLINE_RUNTIME_CHUNK=false
    
    # Build
    npm run build
    
    log "Production build completed ✅"
    log "Build files are in the 'build' directory"
}

# Tests
run_tests() {
    log "Running tests..."
    
    # Tests unitaires
    npm test -- --coverage --watchAll=false
    
    log "Tests completed ✅"
}

# Analyse du bundle
analyze_bundle() {
    log "Analyzing bundle size..."
    
    if ! command -v npx &> /dev/null; then
        error "npx is not available"
    fi
    
    # Installer webpack-bundle-analyzer si nécessaire
    if ! npm list webpack-bundle-analyzer &>/dev/null; then
        npm install --save-dev webpack-bundle-analyzer
    fi
    
    # Analyser le bundle
    npm run build
    npx webpack-bundle-analyzer build/static/js/*.js
    
    log "Bundle analysis completed ✅"
}

# Nettoyage
cleanup() {
    log "Cleaning up..."
    
    # Supprimer node_modules
    if [ -d "node_modules" ]; then
        rm -rf node_modules
        log "Removed node_modules ✅"
    fi
    
    # Supprimer build
    if [ -d "build" ]; then
        rm -rf build
        log "Removed build directory ✅"
    fi
    
    # Supprimer coverage
    if [ -d "coverage" ]; then
        rm -rf coverage
        log "Removed coverage directory ✅"
    fi
    
    log "Cleanup completed ✅"
}

# Menu principal
main() {
    case "${1:-start}" in
        "start"|"dev")
            check_prerequisites
            install_dependencies
            setup_environment
            check_api_health
            start_development_server
            ;;
        "build")
            check_prerequisites
            install_dependencies
            setup_environment
            build_production
            ;;
        "test")
            check_prerequisites
            install_dependencies
            run_tests
            ;;
        "analyze")
            check_prerequisites
            install_dependencies
            analyze_bundle
            ;;
        "clean")
            cleanup
            ;;
        "install")
            check_prerequisites
            install_dependencies
            setup_environment
            ;;
        "health")
            check_api_health
            ;;
        *)
            echo "Usage: $0 {start|build|test|analyze|clean|install|health}"
            echo ""
            echo "Commands:"
            echo "  start    - Start development server (default)"
            echo "  build    - Build for production"
            echo "  test     - Run tests"
            echo "  analyze  - Analyze bundle size"
            echo "  clean    - Clean build artifacts"
            echo "  install  - Install dependencies"
            echo "  health   - Check API health"
            exit 1
            ;;
    esac
}

# Gestion des signaux
trap 'echo -e "\n${YELLOW}Shutting down gracefully...${NC}"; exit 0' INT TERM

# Exécution du script principal
main "$@"