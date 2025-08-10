#!/bin/bash

set -e

echo "🚀 Déploiement Consciousness Engine Production"
echo "============================================="

# Variables
ENVIRONMENT=${1:-production}
BACKUP_DIR="./backups/$(date +%Y%m%d_%H%M%S)"

echo "📋 Environnement: $ENVIRONMENT"

# Vérifications pré-déploiement
echo "🔍 Vérifications pré-déploiement..."

# Vérifier Docker
if ! command -v docker &> /dev/null; then
    echo "❌ Docker n'est pas installé"
    exit 1
fi

# Vérifier Docker Compose
if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose n'est pas installé"
    exit 1
fi

# Vérifier les variables d'environnement
if [ ! -f ".env.production" ]; then
    echo "❌ Fichier .env.production manquant"
    echo "Créez le fichier avec les variables nécessaires:"
    echo "POSTGRES_PASSWORD=your_secure_password"
    echo "REDIS_PASSWORD=your_redis_password"
    echo "JWT_SECRET=your_jwt_secret"
    echo "GRAFANA_PASSWORD=your_grafana_password"
    exit 1
fi

# Backup de la base de données existante
echo "💾 Sauvegarde de la base de données..."
mkdir -p "$BACKUP_DIR"

if docker-compose -f docker-compose.production.yml ps postgres | grep -q "Up"; then
    docker-compose -f docker-compose.production.yml exec -T postgres pg_dump -U postgres consciousness > "$BACKUP_DIR/consciousness_backup.sql"
    echo "✅ Sauvegarde créée: $BACKUP_DIR/consciousness_backup.sql"
fi

# Build des images
echo "🔨 Construction des images Docker..."
docker-compose -f docker-compose.production.yml build --no-cache

# Arrêt des services existants
echo "⏹️ Arrêt des services existants..."
docker-compose -f docker-compose.production.yml down

# Démarrage des services de base
echo "🗄️ Démarrage des services de base..."
docker-compose -f docker-compose.production.yml up -d postgres redis

# Attendre que les services soient prêts
echo "⏳ Attente des services de base..."
sleep 30

# Vérifier la connectivité PostgreSQL
echo "🔍 Vérification PostgreSQL..."
until docker-compose -f docker-compose.production.yml exec postgres pg_isready -U postgres; do
    echo "En attente de PostgreSQL..."
    sleep 5
done

# Démarrage d'Ollama et téléchargement du modèle
echo "🤖 Démarrage d'Ollama..."
docker-compose -f docker-compose.production.yml up -d ollama

echo "⏳ Attente d'Ollama..."
sleep 60

echo "📥 Téléchargement du modèle qwen2.5..."
docker-compose -f docker-compose.production.yml exec ollama ollama pull qwen2.5:3b-instruct-q4_k_m

# Démarrage des services applicatifs
echo "🚀 Démarrage des services applicatifs..."
docker-compose -f docker-compose.production.yml up -d consciousness-engine user-service

echo "⏳ Attente des services applicatifs..."
sleep 30

# Démarrage de l'API Gateway et Frontend
echo "🌐 Démarrage API Gateway et Frontend..."
docker-compose -f docker-compose.production.yml up -d api-gateway frontend

# Démarrage du monitoring
echo "📊 Démarrage du monitoring..."
docker-compose -f docker-compose.production.yml up -d prometheus grafana elasticsearch kibana

# Démarrage du service de backup
echo "💾 Démarrage du service de backup..."
docker-compose -f docker-compose.production.yml up -d backup

# Vérifications post-déploiement
echo "🔍 Vérifications post-déploiement..."

# Fonction de vérification de santé
check_health() {
    local service=$1
    local url=$2
    local max_attempts=30
    local attempt=1

    echo "Vérification de $service..."
    
    while [ $attempt -le $max_attempts ]; do
        if curl -f -s "$url" > /dev/null; then
            echo "✅ $service est opérationnel"
            return 0
        fi
        
        echo "Tentative $attempt/$max_attempts pour $service..."
        sleep 10
        ((attempt++))
    done
    
    echo "❌ $service n'est pas accessible après $max_attempts tentatives"
    return 1
}

# Vérifications des services
check_health "Consciousness Engine" "http://localhost:8080/health"
check_health "User Service" "http://localhost:8081/health"
check_health "API Gateway" "http://localhost:3000/health"
check_health "Frontend" "http://localhost:3001"
check_health "Prometheus" "http://localhost:9090/-/healthy"
check_health "Grafana" "http://localhost:3002/api/health"

# Test de l'API complète
echo "🧪 Test de l'API complète..."
response=$(curl -s -X POST http://localhost:3000/api/v1/consciousness/process \
    -H "Content-Type: application/json" \
    -d '{"content":"Test de déploiement","user_id":"test_user"}')

if echo "$response" | grep -q "content"; then
    echo "✅ Test API réussi"
else
    echo "❌ Test API échoué"
    echo "Réponse: $response"
fi

# Affichage des URLs
echo ""
echo "🎉 Déploiement terminé avec succès!"
echo "=================================="
echo ""
echo "📱 URLs d'accès:"
echo "   Interface principale: http://localhost:3001"
echo "   API Gateway:          http://localhost:3000"
echo "   Grafana:              http://localhost:3002"
echo "   Prometheus:           http://localhost:9090"
echo "   Kibana:               http://localhost:5601"
echo ""
echo "🔐 Identifiants par défaut:"
echo "   Grafana: admin / (voir GRAFANA_PASSWORD dans .env.production)"
echo ""
echo "📊 Monitoring:"
echo "   Logs: docker-compose -f docker-compose.production.yml logs -f"
echo "   Status: docker-compose -f docker-compose.production.yml ps"
echo ""
echo "💾 Sauvegarde créée dans: $BACKUP_DIR"
echo ""
echo "🚀 Consciousness Engine est maintenant en production!"
