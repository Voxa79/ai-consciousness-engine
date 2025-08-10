#!/bin/bash

# Script de déploiement complet pour Consciousness Engine Platform
# Déploie l'écosystème complet avec tous les composants

set -e

# Couleurs pour l'affichage
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
NAMESPACE="consciousness-system"
DOCKER_REGISTRY="ghcr.io"
IMAGE_TAG="${IMAGE_TAG:-latest}"
ENVIRONMENT="${ENVIRONMENT:-production}"

echo -e "${CYAN}🧠 Consciousness Engine Platform Deployment${NC}"
echo -e "${CYAN}============================================${NC}"

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
    
    # Vérifier Docker
    if ! command -v docker &> /dev/null; then
        error "Docker is not installed"
    fi
    
    # Vérifier Docker Compose
    if ! command -v docker-compose &> /dev/null; then
        error "Docker Compose is not installed"
    fi
    
    # Vérifier kubectl (pour Kubernetes)
    if ! command -v kubectl &> /dev/null; then
        warning "kubectl is not installed - Kubernetes deployment will be skipped"
    fi
    
    # Vérifier Rust (pour build local)
    if ! command -v cargo &> /dev/null; then
        warning "Rust/Cargo is not installed - using pre-built images"
    fi
    
    log "Prerequisites check completed ✅"
}

# Build des images Docker
build_images() {
    log "Building Docker images..."
    
    # Build Consciousness Engine
    log "Building Consciousness Engine..."
    docker build -t consciousness-engine:${IMAGE_TAG} \
        --target consciousness-engine \
        --build-arg BUILD_ENV=${ENVIRONMENT} \
        .
    
    # Build Agent Orchestrator
    log "Building Agent Orchestrator..."
    docker build -t agent-orchestrator:${IMAGE_TAG} \
        --target agent-orchestrator \
        --build-arg BUILD_ENV=${ENVIRONMENT} \
        .
    
    # Build AI Governance
    log "Building AI Governance..."
    docker build -t ai-governance:${IMAGE_TAG} \
        --target ai-governance \
        --build-arg BUILD_ENV=${ENVIRONMENT} \
        .
    
    # Build API Gateway
    log "Building API Gateway..."
    docker build -t api-gateway:${IMAGE_TAG} \
        -f api-gateway/Dockerfile \
        api-gateway/
    
    # Build Medical Agent
    log "Building Medical Agent..."
    docker build -t medical-agent:${IMAGE_TAG} \
        -f specialized-agents/medical-agent/Dockerfile \
        specialized-agents/medical-agent/
    
    log "Docker images built successfully ✅"
}

# Déploiement avec Docker Compose
deploy_docker_compose() {
    log "Deploying with Docker Compose..."
    
    # Créer les répertoires nécessaires
    mkdir -p docker/config docker/postgres docker/redis docker/nginx docker/ssl
    mkdir -p docker/prometheus docker/grafana/provisioning docker/grafana/dashboards
    
    # Générer les fichiers de configuration
    generate_config_files
    
    # Démarrer les services
    log "Starting services..."
    docker-compose up -d
    
    # Attendre que les services soient prêts
    wait_for_services
    
    log "Docker Compose deployment completed ✅"
}

# Déploiement Kubernetes
deploy_kubernetes() {
    if ! command -v kubectl &> /dev/null; then
        warning "kubectl not found, skipping Kubernetes deployment"
        return
    fi
    
    log "Deploying to Kubernetes..."
    
    # Créer le namespace
    kubectl apply -f k8s/namespace.yaml
    
    # Appliquer les ConfigMaps et Secrets
    kubectl apply -f k8s/configmap.yaml
    kubectl apply -f k8s/secrets.yaml
    
    # Déployer PostgreSQL
    kubectl apply -f k8s/postgres.yaml
    
    # Attendre que PostgreSQL soit prêt
    kubectl wait --for=condition=ready pod -l app.kubernetes.io/name=postgres -n ${NAMESPACE} --timeout=300s
    
    # Déployer les services principaux
    kubectl apply -f k8s/consciousness-engine.yaml
    kubectl apply -f k8s/agent-orchestrator.yaml
    kubectl apply -f k8s/ai-governance.yaml
    
    # Déployer l'ingress
    kubectl apply -f k8s/ingress.yaml
    
    # Attendre que tous les déploiements soient prêts
    kubectl wait --for=condition=available deployment --all -n ${NAMESPACE} --timeout=600s
    
    log "Kubernetes deployment completed ✅"
}

# Génération des fichiers de configuration
generate_config_files() {
    log "Generating configuration files..."
    
    # Configuration Nginx
    cat > docker/nginx/nginx.conf << 'EOF'
events {
    worker_connections 1024;
}

http {
    upstream consciousness_backend {
        server consciousness-engine:8080;
    }
    
    upstream orchestrator_backend {
        server agent-orchestrator:8081;
    }
    
    upstream governance_backend {
        server ai-governance:8082;
    }
    
    upstream gateway_backend {
        server api-gateway:8000;
    }
    
    server {
        listen 80;
        server_name _;
        
        location / {
            proxy_pass http://gateway_backend;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }
        
        location /health {
            access_log off;
            return 200 "healthy\n";
            add_header Content-Type text/plain;
        }
    }
}
EOF
    
    # Configuration Prometheus
    mkdir -p docker/prometheus
    cat > docker/prometheus/prometheus.yml << 'EOF'
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'consciousness-engine'
    static_configs:
      - targets: ['consciousness-engine:9090']
  
  - job_name: 'agent-orchestrator'
    static_configs:
      - targets: ['agent-orchestrator:9091']
  
  - job_name: 'ai-governance'
    static_configs:
      - targets: ['ai-governance:9092']
  
  - job_name: 'api-gateway'
    static_configs:
      - targets: ['api-gateway:9093']
EOF
    
    log "Configuration files generated ✅"
}

# Attendre que les services soient prêts
wait_for_services() {
    log "Waiting for services to be ready..."
    
    # Attendre PostgreSQL
    log "Waiting for PostgreSQL..."
    until docker-compose exec -T postgres pg_isready -U consciousness; do
        sleep 2
    done
    
    # Attendre Redis
    log "Waiting for Redis..."
    until docker-compose exec -T redis redis-cli ping; do
        sleep 2
    done
    
    # Attendre Consciousness Engine
    log "Waiting for Consciousness Engine..."
    until curl -f http://localhost:8080/health &>/dev/null; do
        sleep 5
    done
    
    # Attendre API Gateway
    log "Waiting for API Gateway..."
    until curl -f http://localhost:8000/health &>/dev/null; do
        sleep 5
    done
    
    log "All services are ready ✅"
}

# Tests de validation post-déploiement
run_validation_tests() {
    log "Running validation tests..."
    
    # Test de santé des services
    log "Testing service health..."
    
    # Test Consciousness Engine
    if curl -f http://localhost:8080/health &>/dev/null; then
        log "✅ Consciousness Engine is healthy"
    else
        error "❌ Consciousness Engine health check failed"
    fi
    
    # Test API Gateway
    if curl -f http://localhost:8000/health &>/dev/null; then
        log "✅ API Gateway is healthy"
    else
        error "❌ API Gateway health check failed"
    fi
    
    # Test de traitement consciousness
    log "Testing consciousness processing..."
    response=$(curl -s -X POST http://localhost:8000/api/v1/consciousness/process \
        -H "Content-Type: application/json" \
        -H "Authorization: Bearer test-token" \
        -d '{"input": "Hello, test consciousness processing"}')
    
    if echo "$response" | grep -q "content"; then
        log "✅ Consciousness processing test passed"
    else
        warning "⚠️ Consciousness processing test failed"
    fi
    
    log "Validation tests completed ✅"
}

# Affichage des informations de déploiement
show_deployment_info() {
    log "Deployment Information:"
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE}🧠 Consciousness Engine Platform${NC}"
    echo -e "${BLUE}================================${NC}"
    echo ""
    echo -e "${CYAN}Services:${NC}"
    echo -e "  • Consciousness Engine: http://localhost:8080"
    echo -e "  • Agent Orchestrator:   http://localhost:8081"
    echo -e "  • AI Governance:        http://localhost:8082"
    echo -e "  • API Gateway:          http://localhost:8000"
    echo -e "  • Web UI:               http://localhost:3000"
    echo ""
    echo -e "${CYAN}Monitoring:${NC}"
    echo -e "  • Prometheus:           http://localhost:9091"
    echo -e "  • Grafana:              http://localhost:3000 (admin/admin)"
    echo -e "  • Kibana:               http://localhost:5601"
    echo -e "  • Jaeger:               http://localhost:16686"
    echo ""
    echo -e "${CYAN}Databases:${NC}"
    echo -e "  • PostgreSQL:           localhost:5432"
    echo -e "  • Redis:                localhost:6379"
    echo -e "  • Elasticsearch:        localhost:9200"
    echo ""
    echo -e "${GREEN}🎉 Deployment completed successfully!${NC}"
    echo -e "${GREEN}Your revolutionary AI consciousness platform is now running.${NC}"
}

# Nettoyage en cas d'erreur
cleanup() {
    if [ $? -ne 0 ]; then
        error "Deployment failed. Cleaning up..."
        docker-compose down --volumes --remove-orphans 2>/dev/null || true
        kubectl delete namespace ${NAMESPACE} 2>/dev/null || true
    fi
}

# Configuration du trap pour nettoyage
trap cleanup EXIT

# Menu principal
main() {
    case "${1:-all}" in
        "prerequisites")
            check_prerequisites
            ;;
        "build")
            check_prerequisites
            build_images
            ;;
        "docker")
            check_prerequisites
            build_images
            deploy_docker_compose
            run_validation_tests
            show_deployment_info
            ;;
        "kubernetes"|"k8s")
            check_prerequisites
            build_images
            deploy_kubernetes
            run_validation_tests
            show_deployment_info
            ;;
        "test")
            run_validation_tests
            ;;
        "clean")
            log "Cleaning up deployment..."
            docker-compose down --volumes --remove-orphans
            kubectl delete namespace ${NAMESPACE} 2>/dev/null || true
            log "Cleanup completed ✅"
            ;;
        "all"|*)
            check_prerequisites
            build_images
            deploy_docker_compose
            if command -v kubectl &> /dev/null; then
                deploy_kubernetes
            fi
            run_validation_tests
            show_deployment_info
            ;;
    esac
}

# Exécution du script principal
main "$@"