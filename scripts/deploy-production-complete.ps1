# Script de déploiement production complet - Consciousness Engine
# Expert CTO Next Gen - Version finale enterprise

param(
    [ValidateSet("local", "kubernetes", "aws")]
    [string]$Target = "local",
    
    [switch]$Build,
    [switch]$Clean,
    [switch]$Test,
    [switch]$Monitor,
    [string]$Environment = "production"
)

Write-Host "🧠 CONSCIOUSNESS ENGINE - DÉPLOIEMENT PRODUCTION COMPLET" -ForegroundColor Green
Write-Host "==========================================================" -ForegroundColor Green

$ErrorActionPreference = "Stop"

Write-Host "📋 Configuration:" -ForegroundColor Yellow
Write-Host "   Target: $Target" -ForegroundColor White
Write-Host "   Environment: $Environment" -ForegroundColor White
Write-Host "   Build: $Build" -ForegroundColor White
Write-Host "   Clean: $Clean" -ForegroundColor White
Write-Host "   Test: $Test" -ForegroundColor White
Write-Host "   Monitor: $Monitor" -ForegroundColor White

# Fonction de vérification des prérequis
function Test-Prerequisites {
    param([string]$Target)
    
    Write-Host "🔍 Vérification des prérequis pour $Target..." -ForegroundColor Yellow
    
    # Prérequis communs
    $commonTools = @("docker", "git")
    
    # Prérequis spécifiques par target
    $targetTools = switch ($Target) {
        "local" { @("docker-compose") }
        "kubernetes" { @("kubectl", "helm") }
        "aws" { @("kubectl", "helm", "terraform", "aws") }
    }
    
    $allTools = $commonTools + $targetTools
    
    foreach ($tool in $allTools) {
        if (-not (Get-Command $tool -ErrorAction SilentlyContinue)) {
            Write-Host "❌ $tool n'est pas installé ou accessible" -ForegroundColor Red
            return $false
        }
        Write-Host "✅ $tool disponible" -ForegroundColor Green
    }
    
    return $true
}

# Fonction de déploiement local
function Deploy-Local {
    Write-Host "🏠 Déploiement local avec Docker Compose..." -ForegroundColor Cyan
    
    if ($Clean) {
        Write-Host "🧹 Nettoyage des conteneurs existants..." -ForegroundColor Yellow
        docker-compose -f docker-compose.monitoring.yml down -v --remove-orphans
        docker system prune -f
    }
    
    if ($Build) {
        Write-Host "🔨 Construction des images..." -ForegroundColor Yellow
        docker-compose -f docker-compose.monitoring.yml build --no-cache
    }
    
    Write-Host "🚀 Démarrage des services..." -ForegroundColor Yellow
    docker-compose -f docker-compose.monitoring.yml up -d
    
    # Attendre que les services soient prêts
    Write-Host "⏳ Attente des services..." -ForegroundColor Yellow
    Start-Sleep -Seconds 60
    
    # Télécharger le modèle Ollama
    Write-Host "📥 Téléchargement du modèle Ollama..." -ForegroundColor Yellow
    docker-compose -f docker-compose.monitoring.yml exec ollama ollama pull qwen2.5:3b-instruct-q4_k_m
    
    return @{
        "Interface" = "http://localhost:3003"
        "API Gateway" = "http://localhost:3000"
        "Grafana" = "http://localhost:3002"
        "Prometheus" = "http://localhost:9090"
        "Kibana" = "http://localhost:5601"
    }
}

# Fonction de déploiement Kubernetes
function Deploy-Kubernetes {
    Write-Host "☸️ Déploiement Kubernetes..." -ForegroundColor Cyan
    
    # Vérifier la connexion au cluster
    try {
        kubectl cluster-info | Out-Null
        Write-Host "✅ Connexion au cluster Kubernetes confirmée" -ForegroundColor Green
    } catch {
        Write-Host "❌ Impossible de se connecter au cluster Kubernetes" -ForegroundColor Red
        return $null
    }
    
    # Exécuter le script de déploiement Kubernetes
    if (Test-Path "scripts/deploy-kubernetes.sh") {
        bash scripts/deploy-kubernetes.sh $Environment
    } else {
        Write-Host "❌ Script de déploiement Kubernetes non trouvé" -ForegroundColor Red
        return $null
    }
    
    # Obtenir l'IP du load balancer
    $externalIP = kubectl get service nginx-ingress-controller -n ingress-nginx -o jsonpath='{.status.loadBalancer.ingress[0].ip}' 2>$null
    if (-not $externalIP) {
        $externalIP = kubectl get service nginx-ingress-controller -n ingress-nginx -o jsonpath='{.status.loadBalancer.ingress[0].hostname}' 2>$null
    }
    
    return @{
        "Load Balancer IP" = $externalIP
        "Interface" = "https://consciousness.yourdomain.com"
        "API Gateway" = "https://api.consciousness.yourdomain.com"
        "Monitoring" = "https://monitoring.consciousness.yourdomain.com"
    }
}

# Fonction de déploiement AWS
function Deploy-AWS {
    Write-Host "☁️ Déploiement AWS avec Terraform..." -ForegroundColor Cyan
    
    # Vérifier les credentials AWS
    try {
        aws sts get-caller-identity | Out-Null
        Write-Host "✅ Credentials AWS configurés" -ForegroundColor Green
    } catch {
        Write-Host "❌ Credentials AWS non configurés" -ForegroundColor Red
        return $null
    }
    
    # Initialiser Terraform
    Write-Host "🔧 Initialisation de Terraform..." -ForegroundColor Yellow
    Set-Location terraform
    terraform init -backend-config="backend-config/prod.hcl"
    
    # Planifier le déploiement
    Write-Host "📋 Planification Terraform..." -ForegroundColor Yellow
    terraform plan -var-file="environments/prod/terraform.tfvars" -out=tfplan
    
    # Appliquer le déploiement
    Write-Host "🚀 Application Terraform..." -ForegroundColor Yellow
    terraform apply tfplan
    
    # Obtenir les outputs
    $clusterName = terraform output -raw cluster_name
    $region = terraform output -raw region
    
    # Configurer kubectl
    Write-Host "⚙️ Configuration kubectl..." -ForegroundColor Yellow
    aws eks update-kubeconfig --region $region --name $clusterName
    
    Set-Location ..
    
    # Déployer sur le cluster EKS
    return Deploy-Kubernetes
}

# Fonction de tests
function Run-Tests {
    Write-Host "🧪 Exécution des tests..." -ForegroundColor Cyan
    
    # Tests Rust
    if (Test-Path "rust-services") {
        Write-Host "🦀 Tests Rust..." -ForegroundColor Yellow
        Set-Location rust-services
        cargo test --release
        Set-Location ..
    }
    
    # Tests TypeScript
    if (Test-Path "web-ui/package.json") {
        Write-Host "📜 Tests TypeScript..." -ForegroundColor Yellow
        Set-Location web-ui
        npm test -- --coverage --watchAll=false
        Set-Location ..
    }
    
    # Tests d'intégration
    if (Test-Path "tests/integration_tests.rs") {
        Write-Host "🔗 Tests d'intégration..." -ForegroundColor Yellow
        cargo test --test integration_tests
    }
}

# Fonction de monitoring
function Setup-Monitoring {
    param([hashtable]$Endpoints)
    
    Write-Host "📊 Configuration du monitoring..." -ForegroundColor Cyan
    
    foreach ($service in $Endpoints.Keys) {
        $url = $Endpoints[$service]
        Write-Host "🔍 Vérification de $service : $url" -ForegroundColor Yellow
        
        try {
            $response = Invoke-RestMethod -Uri "$url/health" -Method Get -TimeoutSec 10 -ErrorAction SilentlyContinue
            Write-Host "✅ $service opérationnel" -ForegroundColor Green
        } catch {
            try {
                $response = Invoke-RestMethod -Uri $url -Method Get -TimeoutSec 10 -ErrorAction SilentlyContinue
                Write-Host "✅ $service accessible" -ForegroundColor Green
            } catch {
                Write-Host "⚠️ $service non accessible" -ForegroundColor Yellow
            }
        }
    }
}

# Fonction principale
function Main {
    # Vérifier les prérequis
    if (-not (Test-Prerequisites -Target $Target)) {
        Write-Host "❌ Prérequis manquants pour $Target" -ForegroundColor Red
        exit 1
    }
    
    # Exécuter les tests si demandé
    if ($Test) {
        Run-Tests
    }
    
    # Déployer selon le target
    $endpoints = switch ($Target) {
        "local" { Deploy-Local }
        "kubernetes" { Deploy-Kubernetes }
        "aws" { Deploy-AWS }
    }
    
    if (-not $endpoints) {
        Write-Host "❌ Échec du déploiement" -ForegroundColor Red
        exit 1
    }
    
    # Configurer le monitoring si demandé
    if ($Monitor) {
        Setup-Monitoring -Endpoints $endpoints
    }
    
    # Affichage final
    Write-Host ""
    Write-Host "🎉 DÉPLOIEMENT TERMINÉ AVEC SUCCÈS!" -ForegroundColor Green
    Write-Host "====================================" -ForegroundColor Green
    Write-Host ""
    Write-Host "📱 Endpoints disponibles:" -ForegroundColor White
    
    foreach ($service in $endpoints.Keys) {
        $url = $endpoints[$service]
        Write-Host "   $service : $url" -ForegroundColor Cyan
    }
    
    Write-Host ""
    Write-Host "🔧 Commandes utiles:" -ForegroundColor White
    
    switch ($Target) {
        "local" {
            Write-Host "   Logs: docker-compose -f docker-compose.monitoring.yml logs -f" -ForegroundColor Yellow
            Write-Host "   Stop: docker-compose -f docker-compose.monitoring.yml down" -ForegroundColor Yellow
        }
        "kubernetes" {
            Write-Host "   Logs: kubectl logs -f deployment/consciousness-engine -n consciousness-engine" -ForegroundColor Yellow
            Write-Host "   Status: kubectl get pods -n consciousness-engine" -ForegroundColor Yellow
        }
        "aws" {
            Write-Host "   Cluster: aws eks describe-cluster --name consciousness-engine" -ForegroundColor Yellow
            Write-Host "   Nodes: kubectl get nodes" -ForegroundColor Yellow
        }
    }
    
    Write-Host ""
    Write-Host "🚀 CONSCIOUSNESS ENGINE EST MAINTENANT EN PRODUCTION!" -ForegroundColor Green
    Write-Host "Plateforme prête pour des millions d'utilisateurs." -ForegroundColor Green
    Write-Host ""
}

# Exécution
try {
    Main
} catch {
    Write-Host "❌ Erreur lors du déploiement: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}
