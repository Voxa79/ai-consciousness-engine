# Container Deployment Script
# Automated deployment with health checks, rollback, and monitoring

param(
    [string]$Environment = "development",
    [string]$Registry = "ghcr.io",
    [string]$Namespace = "consciousness-platform",
    [string]$Tag = "latest",
    [string]$DeploymentStrategy = "rolling",
    [switch]$DryRun = $false,
    [switch]$SkipHealthCheck = $false,
    [int]$HealthCheckTimeout = 300
)

$Green = "Green"
$Red = "Red"
$Yellow = "Yellow"
$Blue = "Blue"

function Write-Log {
    param([string]$Message, [string]$Color = "White")
    $timestamp = Get-Date -Format "HH:mm:ss"
    Write-Host "[$timestamp] $Message" -ForegroundColor $Color
}

function Test-Prerequisites {
    Write-Log "Checking deployment prerequisites..." $Blue
    
    $prerequisites = @(
        @{ Name = "Docker"; Command = "docker --version" },
        @{ Name = "Docker Compose"; Command = "docker-compose --version" },
        @{ Name = "kubectl"; Command = "kubectl version --client" }
    )
    
    $allPrerequisitesMet = $true
    
    foreach ($prereq in $prerequisites) {
        try {
            Invoke-Expression $prereq.Command | Out-Null
            Write-Log "✅ $($prereq.Name) is available" $Green
        }
        catch {
            Write-Log "❌ $($prereq.Name) is not available" $Red
            $allPrerequisitesMet = $false
        }
    }
    
    return $allPrerequisitesMet
}

function Get-DeploymentConfiguration {
    param([string]$Environment)
    
    Write-Log "Loading deployment configuration for $Environment..." $Blue
    
    $configFile = "config/environments/$Environment.yml"
    
    if (-not (Test-Path $configFile)) {
        Write-Log "❌ Configuration file not found: $configFile" $Red
        return $null
    }
    
    try {
        # Load and parse configuration
        $config = Get-Content $configFile -Raw | ConvertFrom-Yaml
        
        Write-Log "✅ Configuration loaded successfully" $Green
        return $config
    }
    catch {
        Write-Log "❌ Failed to load configuration: $_" $Red
        return $null
    }
}

function Deploy-WithDockerCompose {
    param([hashtable]$Config, [string]$Environment)
    
    Write-Log "🚀 Deploying with Docker Compose..." $Blue
    
    try {
        # Generate environment-specific docker-compose file
        $composeFile = Generate-DockerComposeFile $Config $Environment
        
        if (-not $DryRun) {
            # Pull latest images
            Write-Log "📥 Pulling latest images..." $Yellow
            docker-compose -f $composeFile pull
            
            # Deploy services
            Write-Log "🚀 Starting services..." $Yellow
            docker-compose -f $composeFile up -d
            
            Write-Log "✅ Docker Compose deployment completed" $Green
        } else {
            Write-Log "🔍 [DRY RUN] Would deploy with Docker Compose using: $composeFile" $Blue
        }
        
        return $true
    }
    catch {
        Write-Log "❌ Docker Compose deployment failed: $_" $Red
        return $false
    }
}

function Deploy-WithKubernetes {
    param([hashtable]$Config, [string]$Environment)
    
    Write-Log "☸️ Deploying with Kubernetes..." $Blue
    
    try {
        # Generate Kubernetes manifests
        $manifestsDir = Generate-KubernetesManifests $Config $Environment
        
        if (-not $DryRun) {
            # Apply manifests
            Write-Log "📋 Applying Kubernetes manifests..." $Yellow
            kubectl apply -f $manifestsDir
            
            # Wait for rollout
            Write-Log "⏳ Waiting for rollout to complete..." $Yellow
            kubectl rollout status deployment/consciousness-engine -n $Environment --timeout=300s
            kubectl rollout status deployment/api-gateway -n $Environment --timeout=300s
            
            Write-Log "✅ Kubernetes deployment completed" $Green
        } else {
            Write-Log "🔍 [DRY RUN] Would deploy Kubernetes manifests from: $manifestsDir" $Blue
        }
        
        return $true
    }
    catch {
        Write-Log "❌ Kubernetes deployment failed: $_" $Red
        return $false
    }
}

function Generate-DockerComposeFile {
    param([hashtable]$Config, [string]$Environment)
    
    Write-Log "📝 Generating Docker Compose file for $Environment..." $Yellow
    
    $composeContent = @"
version: '3.8'

services:
  consciousness-engine:
    image: $Registry/$Namespace/consciousness-engine:$Tag
    container_name: consciousness-engine-$Environment
    environment:
      - RUST_LOG=$($Config.consciousness_engine.log_level)
      - CONSCIOUSNESS_TIMEOUT_MS=$($Config.consciousness_engine.processing_timeout_ms)
      - CONSCIOUSNESS_QUALITY_THRESHOLD=$($Config.consciousness_engine.quality_threshold)
      - DATABASE_URL=postgresql://$($Config.database.username):$($Config.database.password)@postgres:$($Config.database.port)/$($Config.database.name)
      - REDIS_URL=redis://:$($Config.redis.password)@redis:$($Config.redis.port)/$($Config.redis.database)
    ports:
      - "8081:8081"
    depends_on:
      - postgres
      - redis
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8081/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 30s
    deploy:
      resources:
        limits:
          memory: $($Config.performance.memory_limit)
          cpus: '$($Config.performance.cpu_limit)'
        reservations:
          memory: $($Config.performance.memory_request)
          cpus: '$($Config.performance.cpu_request)'
    networks:
      - consciousness-network

  api-gateway:
    image: $Registry/$Namespace/api-gateway:$Tag
    container_name: api-gateway-$Environment
    environment:
      - RUST_LOG=$($Config.api_gateway.log_level)
      - PORT=$($Config.api_gateway.port)
      - JWT_SECRET=$($Config.api_gateway.jwt.secret)
      - CONSCIOUSNESS_ENGINE_URL=http://consciousness-engine:8081
      - DATABASE_URL=postgresql://$($Config.database.username):$($Config.database.password)@postgres:$($Config.database.port)/$($Config.database.name)
      - REDIS_URL=redis://:$($Config.redis.password)@redis:$($Config.redis.port)/$($Config.redis.database)
    ports:
      - "$($Config.api_gateway.port):$($Config.api_gateway.port)"
    depends_on:
      - consciousness-engine
      - postgres
      - redis
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:$($Config.api_gateway.port)/health"]
      interval: 30s
      timeout: 10s
      retries: 3
    networks:
      - consciousness-network

  web-ui:
    image: $Registry/$Namespace/web-ui:$Tag
    container_name: web-ui-$Environment
    environment:
      - NODE_ENV=$Environment
      - REACT_APP_API_URL=$($Config.web_ui.api_url)
      - REACT_APP_WS_URL=$($Config.web_ui.websocket_url)
    ports:
      - "3000:80"
    depends_on:
      - api-gateway
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:80/health"]
      interval: 30s
      timeout: 10s
      retries: 3
    networks:
      - consciousness-network

  postgres:
    image: postgres:15-alpine
    container_name: postgres-$Environment
    environment:
      - POSTGRES_DB=$($Config.database.name)
      - POSTGRES_USER=$($Config.database.username)
      - POSTGRES_PASSWORD=$($Config.database.password)
    volumes:
      - postgres_data_$Environment`:/var/lib/postgresql/data
    ports:
      - "$($Config.database.port):5432"
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U $($Config.database.username)"]
      interval: 10s
      timeout: 5s
      retries: 5
    networks:
      - consciousness-network

  redis:
    image: redis:7-alpine
    container_name: redis-$Environment
    command: redis-server --appendonly yes --requirepass $($Config.redis.password)
    volumes:
      - redis_data_$Environment`:/data
    ports:
      - "$($Config.redis.port):6379"
    healthcheck:
      test: ["CMD", "redis-cli", "--raw", "incr", "ping"]
      interval: 10s
      timeout: 3s
      retries: 5
    networks:
      - consciousness-network

$(if ($Config.monitoring.enabled) {
@"
  prometheus:
    image: prom/prometheus:latest
    container_name: prometheus-$Environment
    ports:
      - "$($Config.monitoring.prometheus.port):9090"
    volumes:
      - ./monitoring/prometheus.yml:/etc/prometheus/prometheus.yml:ro
      - prometheus_data_$Environment`:/prometheus
    networks:
      - consciousness-network

  grafana:
    image: grafana/grafana:latest
    container_name: grafana-$Environment
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=$($Config.monitoring.grafana.admin_password)
    ports:
      - "$($Config.monitoring.grafana.port):3000"
    volumes:
      - grafana_data_$Environment`:/var/lib/grafana
    networks:
      - consciousness-network
"@
})

volumes:
  postgres_data_$Environment`:
  redis_data_$Environment`:
$(if ($Config.monitoring.enabled) {
@"
  prometheus_data_$Environment`:
  grafana_data_$Environment`:
"@
})

networks:
  consciousness-network:
    driver: bridge
"@

    $composeFile = "docker-compose-$Environment.yml"
    $composeContent | Out-File -FilePath $composeFile -Encoding UTF8
    
    Write-Log "✅ Docker Compose file generated: $composeFile" $Green
    return $composeFile
}

function Generate-KubernetesManifests {
    param([hashtable]$Config, [string]$Environment)
    
    Write-Log "📝 Generating Kubernetes manifests for $Environment..." $Yellow
    
    $manifestsDir = "k8s/$Environment"
    
    if (-not (Test-Path $manifestsDir)) {
        New-Item -ItemType Directory -Path $manifestsDir -Force | Out-Null
    }
    
    # Generate namespace
    $namespaceManifest = @"
apiVersion: v1
kind: Namespace
metadata:
  name: $Environment
  labels:
    environment: $Environment
    app: consciousness-platform
"@
    
    $namespaceManifest | Out-File -FilePath "$manifestsDir/namespace.yaml" -Encoding UTF8
    
    # Generate consciousness-engine deployment
    $consciousnessDeployment = @"
apiVersion: apps/v1
kind: Deployment
metadata:
  name: consciousness-engine
  namespace: $Environment
  labels:
    app: consciousness-engine
    environment: $Environment
spec:
  replicas: $($Config.performance.min_replicas)
  selector:
    matchLabels:
      app: consciousness-engine
  template:
    metadata:
      labels:
        app: consciousness-engine
        environment: $Environment
    spec:
      containers:
      - name: consciousness-engine
        image: $Registry/$Namespace/consciousness-engine:$Tag
        ports:
        - containerPort: 8081
        env:
        - name: RUST_LOG
          value: "$($Config.consciousness_engine.log_level)"
        - name: CONSCIOUSNESS_TIMEOUT_MS
          value: "$($Config.consciousness_engine.processing_timeout_ms)"
        - name: CONSCIOUSNESS_QUALITY_THRESHOLD
          value: "$($Config.consciousness_engine.quality_threshold)"
        resources:
          requests:
            memory: "$($Config.performance.memory_request)"
            cpu: "$($Config.performance.cpu_request)"
          limits:
            memory: "$($Config.performance.memory_limit)"
            cpu: "$($Config.performance.cpu_limit)"
        livenessProbe:
          httpGet:
            path: /health
            port: 8081
          initialDelaySeconds: $($Config.performance.health_check.initial_delay_seconds)
          periodSeconds: $($Config.performance.health_check.period_seconds)
          timeoutSeconds: $($Config.performance.health_check.timeout_seconds)
          failureThreshold: $($Config.performance.health_check.failure_threshold)
        readinessProbe:
          httpGet:
            path: /health
            port: 8081
          initialDelaySeconds: 10
          periodSeconds: 5
          timeoutSeconds: 2
          failureThreshold: 3
---
apiVersion: v1
kind: Service
metadata:
  name: consciousness-engine
  namespace: $Environment
spec:
  selector:
    app: consciousness-engine
  ports:
  - port: 8081
    targetPort: 8081
  type: ClusterIP
"@
    
    $consciousnessDeployment | Out-File -FilePath "$manifestsDir/consciousness-engine.yaml" -Encoding UTF8
    
    # Generate API Gateway deployment
    $apiGatewayDeployment = @"
apiVersion: apps/v1
kind: Deployment
metadata:
  name: api-gateway
  namespace: $Environment
  labels:
    app: api-gateway
    environment: $Environment
spec:
  replicas: $($Config.performance.min_replicas)
  selector:
    matchLabels:
      app: api-gateway
  template:
    metadata:
      labels:
        app: api-gateway
        environment: $Environment
    spec:
      containers:
      - name: api-gateway
        image: $Registry/$Namespace/api-gateway:$Tag
        ports:
        - containerPort: $($Config.api_gateway.port)
        env:
        - name: RUST_LOG
          value: "$($Config.api_gateway.log_level)"
        - name: PORT
          value: "$($Config.api_gateway.port)"
        - name: CONSCIOUSNESS_ENGINE_URL
          value: "http://consciousness-engine:8081"
        resources:
          requests:
            memory: "256Mi"
            cpu: "0.25"
          limits:
            memory: "512Mi"
            cpu: "0.5"
        livenessProbe:
          httpGet:
            path: /health
            port: $($Config.api_gateway.port)
          initialDelaySeconds: 30
          periodSeconds: 30
          timeoutSeconds: 5
          failureThreshold: 3
        readinessProbe:
          httpGet:
            path: /health
            port: $($Config.api_gateway.port)
          initialDelaySeconds: 10
          periodSeconds: 5
          timeoutSeconds: 2
          failureThreshold: 3
---
apiVersion: v1
kind: Service
metadata:
  name: api-gateway
  namespace: $Environment
spec:
  selector:
    app: api-gateway
  ports:
  - port: $($Config.api_gateway.port)
    targetPort: $($Config.api_gateway.port)
  type: LoadBalancer
"@
    
    $apiGatewayDeployment | Out-File -FilePath "$manifestsDir/api-gateway.yaml" -Encoding UTF8
    
    # Generate HPA if auto-scaling is enabled
    if ($Config.performance.max_replicas -gt $Config.performance.min_replicas) {
        $hpaManifest = @"
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: consciousness-engine-hpa
  namespace: $Environment
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: consciousness-engine
  minReplicas: $($Config.performance.min_replicas)
  maxReplicas: $($Config.performance.max_replicas)
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: $($Config.performance.target_cpu_utilization)
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: $($Config.performance.target_memory_utilization)
---
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: api-gateway-hpa
  namespace: $Environment
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: api-gateway
  minReplicas: $($Config.performance.min_replicas)
  maxReplicas: $($Config.performance.max_replicas)
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: $($Config.performance.target_cpu_utilization)
"@
        
        $hpaManifest | Out-File -FilePath "$manifestsDir/hpa.yaml" -Encoding UTF8
    }
    
    Write-Log "✅ Kubernetes manifests generated in: $manifestsDir" $Green
    return $manifestsDir
}

function Test-DeploymentHealth {
    param([hashtable]$Config, [string]$Environment, [string]$DeploymentType)
    
    if ($SkipHealthCheck) {
        Write-Log "⏭️ Health check skipped" $Yellow
        return $true
    }
    
    Write-Log "🏥 Performing health checks..." $Blue
    
    $healthChecksPassed = $true
    $startTime = Get-Date
    
    # Define health check endpoints
    $healthChecks = @(
        @{ Name = "Consciousness Engine"; Url = "http://localhost:8081/health" },
        @{ Name = "API Gateway"; Url = "http://localhost:$($Config.api_gateway.port)/health" },
        @{ Name = "Web UI"; Url = "http://localhost:3000/health" }
    )
    
    while (((Get-Date) - $startTime).TotalSeconds -lt $HealthCheckTimeout) {
        $allHealthy = $true
        
        foreach ($check in $healthChecks) {
            try {
                $response = Invoke-WebRequest -Uri $check.Url -TimeoutSec 5 -UseBasicParsing
                if ($response.StatusCode -eq 200) {
                    Write-Log "✅ $($check.Name) is healthy" $Green
                } else {
                    Write-Log "⚠️ $($check.Name) returned status $($response.StatusCode)" $Yellow
                    $allHealthy = $false
                }
            }
            catch {
                Write-Log "❌ $($check.Name) health check failed: $_" $Red
                $allHealthy = $false
            }
        }
        
        if ($allHealthy) {
            Write-Log "🎉 All health checks passed!" $Green
            return $true
        }
        
        Write-Log "⏳ Waiting for services to become healthy..." $Yellow
        Start-Sleep -Seconds 10
    }
    
    Write-Log "❌ Health checks timed out after $HealthCheckTimeout seconds" $Red
    return $false
}

function Rollback-Deployment {
    param([string]$Environment, [string]$DeploymentType)
    
    Write-Log "🔄 Rolling back deployment..." $Yellow
    
    try {
        switch ($DeploymentType) {
            "docker-compose" {
                $composeFile = "docker-compose-$Environment.yml"
                docker-compose -f $composeFile down
                Write-Log "✅ Docker Compose rollback completed" $Green
            }
            "kubernetes" {
                kubectl rollout undo deployment/consciousness-engine -n $Environment
                kubectl rollout undo deployment/api-gateway -n $Environment
                Write-Log "✅ Kubernetes rollback completed" $Green
            }
        }
        
        return $true
    }
    catch {
        Write-Log "❌ Rollback failed: $_" $Red
        return $false
    }
}

function Generate-DeploymentReport {
    param([hashtable]$DeploymentResults, [string]$Environment)
    
    Write-Log "📋 Generating deployment report..." $Blue
    
    $timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
    $reportFile = "deployment_reports/deployment_report_${Environment}_$timestamp.md"
    
    # Create reports directory
    if (-not (Test-Path "deployment_reports")) {
        New-Item -ItemType Directory -Path "deployment_reports" -Force | Out-Null
    }
    
    $reportContent = @"
# Deployment Report - $Environment

**Generated:** $(Get-Date)
**Environment:** $Environment
**Registry:** $Registry
**Namespace:** $Namespace
**Tag:** $Tag
**Strategy:** $DeploymentStrategy

## Deployment Results

### Prerequisites Check
$(if ($DeploymentResults.Prerequisites) { "✅ All prerequisites met" } else { "❌ Prerequisites check failed" })

### Configuration Loading
$(if ($DeploymentResults.Configuration) { "✅ Configuration loaded successfully" } else { "❌ Configuration loading failed" })

### Container Deployment
$(if ($DeploymentResults.Deployment) { "✅ Deployment completed successfully" } else { "❌ Deployment failed" })

### Health Checks
$(if ($DeploymentResults.HealthCheck) { "✅ All health checks passed" } else { "❌ Health checks failed" })

## Deployment Configuration

- **CPU Limit:** $($DeploymentResults.Config.performance.cpu_limit)
- **Memory Limit:** $($DeploymentResults.Config.performance.memory_limit)
- **Min Replicas:** $($DeploymentResults.Config.performance.min_replicas)
- **Max Replicas:** $($DeploymentResults.Config.performance.max_replicas)

## Services Deployed

1. **Consciousness Engine**
   - Image: $Registry/$Namespace/consciousness-engine:$Tag
   - Port: 8081
   - Health: $(if ($DeploymentResults.HealthCheck) { "✅ Healthy" } else { "❌ Unhealthy" })

2. **API Gateway**
   - Image: $Registry/$Namespace/api-gateway:$Tag
   - Port: $($DeploymentResults.Config.api_gateway.port)
   - Health: $(if ($DeploymentResults.HealthCheck) { "✅ Healthy" } else { "❌ Unhealthy" })

3. **Web UI**
   - Image: $Registry/$Namespace/web-ui:$Tag
   - Port: 3000
   - Health: $(if ($DeploymentResults.HealthCheck) { "✅ Healthy" } else { "❌ Unhealthy" })

## Next Steps

$(if ($DeploymentResults.HealthCheck) {
@"
1. **Monitor deployment** using Grafana dashboards
2. **Run smoke tests** to validate functionality
3. **Update DNS/Load Balancer** if needed
4. **Notify stakeholders** of successful deployment
"@
} else {
@"
1. **Investigate health check failures**
2. **Review service logs** for error details
3. **Consider rollback** if issues persist
4. **Update deployment configuration** as needed
"@
})

## Troubleshooting

### Service Logs
``````bash
# View consciousness engine logs
docker logs consciousness-engine-$Environment

# View API gateway logs
docker logs api-gateway-$Environment

# View web UI logs
docker logs web-ui-$Environment
``````

### Health Check URLs
- Consciousness Engine: http://localhost:8081/health
- API Gateway: http://localhost:$($DeploymentResults.Config.api_gateway.port)/health
- Web UI: http://localhost:3000/health

**Report completed at:** $(Get-Date)
"@

    $reportContent | Out-File -FilePath $reportFile -Encoding UTF8
    Write-Log "✅ Deployment report generated: $reportFile" $Green
}

# Main execution
function Main {
    Write-Log "🚀 Container Deployment for $Environment" $Blue
    Write-Log "=======================================" $Blue
    
    $deploymentResults = @{
        Prerequisites = $false
        Configuration = $false
        Deployment = $false
        HealthCheck = $false
        Config = $null
    }
    
    try {
        # Check prerequisites
        $deploymentResults.Prerequisites = Test-Prerequisites
        if (-not $deploymentResults.Prerequisites) {
            Write-Log "❌ Prerequisites not met. Aborting deployment." $Red
            exit 1
        }
        
        # Load configuration
        $config = Get-DeploymentConfiguration $Environment
        if (-not $config) {
            Write-Log "❌ Failed to load configuration. Aborting deployment." $Red
            exit 1
        }
        $deploymentResults.Configuration = $true
        $deploymentResults.Config = $config
        
        # Determine deployment method
        $deploymentType = if (Get-Command kubectl -ErrorAction SilentlyContinue) { "kubernetes" } else { "docker-compose" }
        
        Write-Log "📋 Deployment method: $deploymentType" $Blue
        Write-Log "🏷️ Image tag: $Tag" $Blue
        Write-Log "🌍 Environment: $Environment" $Blue
        
        # Deploy based on method
        switch ($deploymentType) {
            "kubernetes" {
                $deploymentResults.Deployment = Deploy-WithKubernetes $config $Environment
            }
            "docker-compose" {
                $deploymentResults.Deployment = Deploy-WithDockerCompose $config $Environment
            }
        }
        
        if (-not $deploymentResults.Deployment) {
            Write-Log "❌ Deployment failed. Check logs above." $Red
            exit 1
        }
        
        # Health checks
        $deploymentResults.HealthCheck = Test-DeploymentHealth $config $Environment $deploymentType
        
        if (-not $deploymentResults.HealthCheck) {
            Write-Log "❌ Health checks failed. Consider rollback." $Red
            
            # Ask for rollback
            $rollback = Read-Host "Do you want to rollback? (y/N)"
            if ($rollback -eq "y" -or $rollback -eq "Y") {
                Rollback-Deployment $Environment $deploymentType
            }
        }
        
        # Generate report
        Generate-DeploymentReport $deploymentResults $Environment
        
        # Final summary
        Write-Log ""
        if ($deploymentResults.HealthCheck) {
            Write-Log "🎉 Deployment completed successfully!" $Green
            Write-Log "🌐 Services are healthy and ready to serve traffic" $Green
        } else {
            Write-Log "⚠️ Deployment completed with issues" $Yellow
            Write-Log "🔍 Check the deployment report for details" $Yellow
        }
    }
    catch {
        Write-Log "❌ Deployment failed: $_" $Red
        exit 1
    }
}

# Show usage if no parameters
if ($args.Count -eq 0) {
    Write-Host @"
Container Deployment Script

Usage: .\deploy_containers.ps1 -Environment <env> [options]

Environments:
  development  - Local development deployment
  staging      - Staging environment deployment
  production   - Production environment deployment

Options:
  -Registry <url>              Container registry URL (default: ghcr.io)
  -Namespace <name>            Registry namespace (default: consciousness-platform)
  -Tag <tag>                   Image tag to deploy (default: latest)
  -DeploymentStrategy <type>   Deployment strategy (rolling, blue-green, canary)
  -DryRun                      Show what would be deployed without executing
  -SkipHealthCheck             Skip post-deployment health checks
  -HealthCheckTimeout <sec>    Health check timeout in seconds (default: 300)

Examples:
  .\deploy_containers.ps1 -Environment development
  .\deploy_containers.ps1 -Environment staging -Tag v1.0.0
  .\deploy_containers.ps1 -Environment production -DryRun
"@ -ForegroundColor $Blue
    exit 0
}

# Run main function
Main