#!/bin/bash

set -e

echo "🚀 Déploiement Kubernetes - Consciousness Engine Production"
echo "=========================================================="

# Variables
NAMESPACE="consciousness-engine"
ENVIRONMENT=${1:-production}
KUBECTL_CONTEXT=${2:-""}

echo "📋 Environnement: $ENVIRONMENT"
echo "📋 Namespace: $NAMESPACE"

# Vérifications pré-déploiement
echo "🔍 Vérifications pré-déploiement..."

# Vérifier kubectl
if ! command -v kubectl &> /dev/null; then
    echo "❌ kubectl n'est pas installé"
    exit 1
fi

# Vérifier helm
if ! command -v helm &> /dev/null; then
    echo "❌ helm n'est pas installé"
    exit 1
fi

# Vérifier le contexte kubectl
if [ -n "$KUBECTL_CONTEXT" ]; then
    echo "🔧 Configuration du contexte kubectl: $KUBECTL_CONTEXT"
    kubectl config use-context "$KUBECTL_CONTEXT"
fi

# Vérifier la connexion au cluster
echo "🔍 Vérification de la connexion au cluster..."
if ! kubectl cluster-info &> /dev/null; then
    echo "❌ Impossible de se connecter au cluster Kubernetes"
    exit 1
fi

echo "✅ Connexion au cluster confirmée"

# Créer le namespace et les ressources de base
echo "📦 Création du namespace et des ressources de base..."
kubectl apply -f k8s/production/namespace.yaml

# Attendre que le namespace soit créé
kubectl wait --for=condition=Active namespace/$NAMESPACE --timeout=60s

# Installer cert-manager si nécessaire
echo "🔐 Installation de cert-manager..."
if ! kubectl get namespace cert-manager &> /dev/null; then
    helm repo add jetstack https://charts.jetstack.io
    helm repo update
    helm install cert-manager jetstack/cert-manager \
        --namespace cert-manager \
        --create-namespace \
        --version v1.13.0 \
        --set installCRDs=true
    
    echo "⏳ Attente de cert-manager..."
    kubectl wait --for=condition=Available deployment/cert-manager -n cert-manager --timeout=300s
fi

# Installer nginx-ingress si nécessaire
echo "🌐 Installation de nginx-ingress..."
if ! kubectl get namespace ingress-nginx &> /dev/null; then
    helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx
    helm repo update
    helm install ingress-nginx ingress-nginx/ingress-nginx \
        --namespace ingress-nginx \
        --create-namespace \
        --set controller.service.type=LoadBalancer
    
    echo "⏳ Attente de nginx-ingress..."
    kubectl wait --for=condition=Available deployment/ingress-nginx-controller -n ingress-nginx --timeout=300s
fi

# Déployer les services de données
echo "🗄️ Déploiement des services de données..."
kubectl apply -f k8s/production/postgres.yaml
kubectl apply -f k8s/production/redis.yaml

echo "⏳ Attente des services de données..."
kubectl wait --for=condition=Available deployment/postgres -n $NAMESPACE --timeout=300s
kubectl wait --for=condition=Available deployment/redis -n $NAMESPACE --timeout=300s

# Déployer Ollama
echo "🤖 Déploiement d'Ollama..."
kubectl apply -f k8s/production/ollama.yaml

echo "⏳ Attente d'Ollama..."
kubectl wait --for=condition=Available deployment/ollama -n $NAMESPACE --timeout=300s

# Télécharger les modèles Ollama
echo "📥 Téléchargement des modèles Ollama..."
OLLAMA_POD=$(kubectl get pods -n $NAMESPACE -l app=ollama -o jsonpath='{.items[0].metadata.name}')
kubectl exec -n $NAMESPACE $OLLAMA_POD -- ollama pull qwen2.5:3b-instruct-q4_k_m

# Déployer les services applicatifs
echo "🚀 Déploiement des services applicatifs..."
kubectl apply -f k8s/production/consciousness-engine.yaml
kubectl apply -f k8s/production/user-service.yaml
kubectl apply -f k8s/production/api-gateway.yaml

echo "⏳ Attente des services applicatifs..."
kubectl wait --for=condition=Available deployment/consciousness-engine -n $NAMESPACE --timeout=300s
kubectl wait --for=condition=Available deployment/user-service -n $NAMESPACE --timeout=300s
kubectl wait --for=condition=Available deployment/api-gateway -n $NAMESPACE --timeout=300s

# Déployer le frontend
echo "🌐 Déploiement du frontend..."
kubectl apply -f k8s/production/frontend.yaml

echo "⏳ Attente du frontend..."
kubectl wait --for=condition=Available deployment/frontend -n $NAMESPACE --timeout=300s

# Déployer le monitoring
echo "📊 Déploiement du monitoring..."
kubectl apply -f k8s/production/monitoring.yaml
kubectl apply -f k8s/production/grafana.yaml

echo "⏳ Attente du monitoring..."
kubectl wait --for=condition=Available deployment/prometheus -n $NAMESPACE --timeout=300s
kubectl wait --for=condition=Available deployment/grafana -n $NAMESPACE --timeout=300s

# Déployer l'ingress
echo "🌐 Déploiement de l'ingress..."
kubectl apply -f k8s/production/ingress.yaml

# Vérifications post-déploiement
echo "🔍 Vérifications post-déploiement..."

# Fonction de vérification de santé
check_service_health() {
    local service=$1
    local namespace=$2
    local max_attempts=30
    local attempt=1

    echo "Vérification de $service..."
    
    while [ $attempt -le $max_attempts ]; do
        if kubectl get pods -n $namespace -l app=$service | grep -q "Running"; then
            echo "✅ $service est opérationnel"
            return 0
        fi
        
        echo "Tentative $attempt/$max_attempts pour $service..."
        sleep 10
        ((attempt++))
    done
    
    echo "❌ $service n'est pas opérationnel après $max_attempts tentatives"
    return 1
}

# Vérifications des services
services=("postgres" "redis" "ollama" "consciousness-engine" "user-service" "api-gateway" "frontend" "prometheus" "grafana")

all_healthy=true
for service in "${services[@]}"; do
    if ! check_service_health "$service" "$NAMESPACE"; then
        all_healthy=false
    fi
done

# Affichage des informations de déploiement
echo ""
if [ "$all_healthy" = true ]; then
    echo "🎉 Déploiement terminé avec succès!"
else
    echo "⚠️ Déploiement terminé avec des avertissements"
fi

echo "=================================="
echo ""
echo "📱 Informations de déploiement:"
echo ""

# Obtenir l'IP du load balancer
EXTERNAL_IP=$(kubectl get service nginx-ingress-controller -n ingress-nginx -o jsonpath='{.status.loadBalancer.ingress[0].ip}' 2>/dev/null || echo "En attente...")
if [ "$EXTERNAL_IP" = "" ]; then
    EXTERNAL_IP=$(kubectl get service nginx-ingress-controller -n ingress-nginx -o jsonpath='{.status.loadBalancer.ingress[0].hostname}' 2>/dev/null || echo "En attente...")
fi

echo "🌐 Load Balancer IP/Hostname: $EXTERNAL_IP"
echo ""
echo "📱 URLs d'accès (après configuration DNS):"
echo "   Interface principale: https://consciousness.yourdomain.com"
echo "   API Gateway:          https://api.consciousness.yourdomain.com"
echo "   Monitoring:           https://monitoring.consciousness.yourdomain.com"
echo ""
echo "🔧 Commandes utiles:"
echo "   Logs: kubectl logs -f deployment/consciousness-engine -n $NAMESPACE"
echo "   Status: kubectl get pods -n $NAMESPACE"
echo "   Services: kubectl get services -n $NAMESPACE"
echo "   Ingress: kubectl get ingress -n $NAMESPACE"
echo ""
echo "📊 Monitoring:"
echo "   kubectl port-forward service/grafana 3000:3000 -n $NAMESPACE"
echo "   kubectl port-forward service/prometheus 9090:9090 -n $NAMESPACE"
echo ""

if [ "$all_healthy" = true ]; then
    echo "🚀 CONSCIOUSNESS ENGINE EST MAINTENANT DÉPLOYÉ EN PRODUCTION KUBERNETES!"
    echo "Tous les services sont opérationnels et prêts à recevoir du trafic."
else
    echo "⚠️ Certains services ont des problèmes. Vérifiez les logs."
    echo "Utilisez: kubectl logs -f deployment/SERVICE_NAME -n $NAMESPACE"
fi

echo ""
