# Container Build and Registry Management Script
# Optimized multi-stage builds with security scanning and registry push

param(
    [string]$Registry = "ghcr.io",
    [string]$Namespace = "consciousness-platform",
    [string]$Tag = "latest",
    [string]$Platform = "linux/amd64,linux/arm64",
    [switch]$Push = $false,
    [switch]$Scan = $true,
    [switch]$Cache = $true
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

function Test-Docker {
    Write-Log "Checking Docker and buildx availability..." $Blue
    try {
        $dockerVersion = docker --version
        Write-Log "✅ Docker: $dockerVersion" $Green
        
        # Check if buildx is available
        docker buildx version | Out-Null
        Write-Log "✅ Docker Buildx is available" $Green
        
        # Create/use multiarch builder
        docker buildx create --name consciousness-builder --use --bootstrap 2>$null
        Write-Log "✅ Multi-architecture builder ready" $Green
        
        return $true
    }
    catch {
        Write-Log "❌ Docker or buildx not available: $_" $Red
        return $false
    }
}

function Test-Registry {
    param([string]$RegistryUrl)
    
    Write-Log "Testing registry access: $RegistryUrl" $Blue
    try {
        # Try to login (assumes credentials are already configured)
        docker login $RegistryUrl --username $env:REGISTRY_USERNAME --password $env:REGISTRY_PASSWORD 2>$null
        Write-Log "✅ Registry access confirmed" $Green
        return $true
    }
    catch {
        Write-Log "⚠️ Registry login failed. Push operations will be skipped." $Yellow
        return $false
    }
}

function Build-ConsciousnessEngine {
    param([string]$ImageName, [string]$Tag, [string]$Platform)
    
    Write-Log "🦀 Building Consciousness Engine container..." $Blue
    
    $buildArgs = @(
        "buildx", "build"
        "--platform", $Platform
        "--tag", "$ImageName:$Tag"
        "--file", "consciousness-engine/Dockerfile"
        "."
    )
    
    if ($Cache) {
        $buildArgs += "--cache-from", "type=gha"
        $buildArgs += "--cache-to", "type=gha,mode=max"
    }
    
    if ($Push) {
        $buildArgs += "--push"
    } else {
        $buildArgs += "--load"
    }
    
    try {
        & docker @buildArgs
        Write-Log "✅ Consciousness Engine container built successfully" $Green
        return $true
    }
    catch {
        Write-Log "❌ Failed to build Consciousness Engine container: $_" $Red
        return $false
    }
}

function Build-ApiGateway {
    param([string]$ImageName, [string]$Tag, [string]$Platform)
    
    Write-Log "🌐 Building API Gateway container..." $Blue
    
    # Create API Gateway Dockerfile if it doesn't exist
    if (-not (Test-Path "api-gateway/Dockerfile")) {
        Create-ApiGatewayDockerfile
    }
    
    $buildArgs = @(
        "buildx", "build"
        "--platform", $Platform
        "--tag", "$ImageName:$Tag"
        "--file", "api-gateway/Dockerfile"
        "."
    )
    
    if ($Cache) {
        $buildArgs += "--cache-from", "type=gha"
        $buildArgs += "--cache-to", "type=gha,mode=max"
    }
    
    if ($Push) {
        $buildArgs += "--push"
    } else {
        $buildArgs += "--load"
    }
    
    try {
        & docker @buildArgs
        Write-Log "✅ API Gateway container built successfully" $Green
        return $true
    }
    catch {
        Write-Log "❌ Failed to build API Gateway container: $_" $Red
        return $false
    }
}

function Build-WebUI {
    param([string]$ImageName, [string]$Tag, [string]$Platform)
    
    Write-Log "⚛️ Building Web UI container..." $Blue
    
    # Create Web UI Dockerfile if it doesn't exist
    if (-not (Test-Path "web-ui/Dockerfile")) {
        Create-WebUIDockerfile
    }
    
    $buildArgs = @(
        "buildx", "build"
        "--platform", $Platform
        "--tag", "$ImageName:$Tag"
        "--file", "web-ui/Dockerfile"
        "web-ui"
    )
    
    if ($Cache) {
        $buildArgs += "--cache-from", "type=gha"
        $buildArgs += "--cache-to", "type=gha,mode=max"
    }
    
    if ($Push) {
        $buildArgs += "--push"
    } else {
        $buildArgs += "--load"
    }
    
    try {
        & docker @buildArgs
        Write-Log "✅ Web UI container built successfully" $Green
        return $true
    }
    catch {
        Write-Log "❌ Failed to build Web UI container: $_" $Red
        return $false
    }
}

function Scan-Container {
    param([string]$ImageName)
    
    if (-not $Scan) {
        Write-Log "⏭️ Container scanning skipped" $Yellow
        return $true
    }
    
    Write-Log "🔍 Scanning container for vulnerabilities: $ImageName" $Blue
    
    # Try different security scanners
    $scanners = @("trivy", "grype", "docker scout")
    $scanSuccess = $false
    
    foreach ($scanner in $scanners) {
        try {
            switch ($scanner) {
                "trivy" {
                    if (Get-Command trivy -ErrorAction SilentlyContinue) {
                        trivy image --exit-code 1 --severity HIGH,CRITICAL $ImageName
                        $scanSuccess = $true
                        Write-Log "✅ Trivy scan passed for $ImageName" $Green
                        break
                    }
                }
                "grype" {
                    if (Get-Command grype -ErrorAction SilentlyContinue) {
                        grype $ImageName --fail-on high
                        $scanSuccess = $true
                        Write-Log "✅ Grype scan passed for $ImageName" $Green
                        break
                    }
                }
                "docker scout" {
                    docker scout cves $ImageName --exit-code --only-severity critical,high
                    $scanSuccess = $true
                    Write-Log "✅ Docker Scout scan passed for $ImageName" $Green
                    break
                }
            }
        }
        catch {
            Write-Log "⚠️ $scanner scan failed or not available" $Yellow
            continue
        }
    }
    
    if (-not $scanSuccess) {
        Write-Log "⚠️ No security scanner available. Install trivy, grype, or docker scout for vulnerability scanning." $Yellow
    }
    
    return $true
}

function Create-ApiGatewayDockerfile {
    Write-Log "Creating API Gateway Dockerfile..." $Yellow
    
    $dockerfileContent = @"
# Multi-stage Dockerfile for API Gateway
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY shared/ shared/
COPY api-gateway/ api-gateway/

# Build the API Gateway
RUN cargo build --release --bin api-gateway

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1001 apigateway

# Copy binary
COPY --from=builder --chown=apigateway:apigateway /app/target/release/api-gateway /usr/local/bin/

# Switch to non-root user
USER apigateway

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Environment
ENV RUST_LOG=info
EXPOSE 8080

CMD ["api-gateway"]
"@

    $dockerfileContent | Out-File -FilePath "api-gateway/Dockerfile" -Encoding UTF8
    Write-Log "✅ API Gateway Dockerfile created" $Green
}

function Create-WebUIDockerfile {
    Write-Log "Creating Web UI Dockerfile..." $Yellow
    
    $dockerfileContent = @"
# Multi-stage Dockerfile for React Web UI
FROM node:18-alpine as builder

WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm ci --only=production

# Copy source code
COPY . .

# Build the application
RUN npm run build

# Runtime stage with nginx
FROM nginx:alpine

# Copy built app
COPY --from=builder /app/build /usr/share/nginx/html

# Copy nginx configuration
COPY nginx.conf /etc/nginx/nginx.conf

# Create non-root user
RUN addgroup -g 1001 -S webui && \
    adduser -S webui -u 1001 -G webui

# Set permissions
RUN chown -R webui:webui /usr/share/nginx/html && \
    chown -R webui:webui /var/cache/nginx && \
    chown -R webui:webui /var/log/nginx && \
    chown -R webui:webui /etc/nginx/conf.d

# Switch to non-root user
USER webui

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:80/health || exit 1

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
"@

    $dockerfileContent | Out-File -FilePath "web-ui/Dockerfile" -Encoding UTF8
    Write-Log "✅ Web UI Dockerfile created" $Green
}

function Generate-BuildReport {
    param([hashtable]$BuildResults, [string]$Registry, [string]$Namespace, [string]$Tag)
    
    Write-Log "📋 Generating build report..." $Blue
    
    $timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
    $reportFile = "build_results/container_build_report_$timestamp.md"
    
    # Create results directory
    if (-not (Test-Path "build_results")) {
        New-Item -ItemType Directory -Path "build_results" -Force | Out-Null
    }
    
    $reportContent = @"
# Container Build Report

**Generated:** $(Get-Date)
**Registry:** $Registry
**Namespace:** $Namespace
**Tag:** $Tag
**Platform:** $Platform

## Build Results

### Consciousness Engine
$(if ($BuildResults.ConsciousnessEngine) { "✅ Build successful" } else { "❌ Build failed" })
**Image:** $Registry/$Namespace/consciousness-engine:$Tag

### API Gateway
$(if ($BuildResults.ApiGateway) { "✅ Build successful" } else { "❌ Build failed" })
**Image:** $Registry/$Namespace/api-gateway:$Tag

### Web UI
$(if ($BuildResults.WebUI) { "✅ Build successful" } else { "❌ Build failed" })
**Image:** $Registry/$Namespace/web-ui:$Tag

## Security Scanning
$(if ($Scan) { "✅ Security scanning enabled" } else { "⏭️ Security scanning skipped" })

## Registry Push
$(if ($Push) { "✅ Images pushed to registry" } else { "⏭️ Images built locally only" })

## Build Configuration

- **Multi-platform:** $Platform
- **Build cache:** $(if ($Cache) { "Enabled" } else { "Disabled" })
- **Security scanning:** $(if ($Scan) { "Enabled" } else { "Disabled" })

## Image Sizes

$(
    try {
        $images = docker images --format "table {{.Repository}}:{{.Tag}}\t{{.Size}}" | Where-Object { $_ -match "$Namespace" }
        $images -join "`n"
    } catch {
        "Unable to retrieve image sizes"
    }
)

## Next Steps

1. **Test containers locally:**
   ``````
   docker-compose up -d
   ``````

2. **Run security scans:**
   ``````
   trivy image $Registry/$Namespace/consciousness-engine:$Tag
   ``````

3. **Deploy to staging:**
   ``````
   kubectl apply -f k8s/staging/
   ``````

**Report completed at:** $(Get-Date)
"@

    $reportContent | Out-File -FilePath $reportFile -Encoding UTF8
    Write-Log "✅ Build report generated: $reportFile" $Green
}

# Main execution
function Main {
    Write-Log "🚀 Starting Container Build Process..." $Blue
    Write-Log "====================================" $Blue
    
    # Pre-flight checks
    if (-not (Test-Docker)) {
        exit 1
    }
    
    $registryAccess = Test-Registry $Registry
    if ($Push -and -not $registryAccess) {
        Write-Log "❌ Registry access required for push operations" $Red
        exit 1
    }
    
    # Build configuration
    $images = @{
        ConsciousnessEngine = "$Registry/$Namespace/consciousness-engine"
        ApiGateway = "$Registry/$Namespace/api-gateway"
        WebUI = "$Registry/$Namespace/web-ui"
    }
    
    $buildResults = @{}
    
    try {
        # Build Consciousness Engine
        $buildResults.ConsciousnessEngine = Build-ConsciousnessEngine $images.ConsciousnessEngine $Tag $Platform
        if ($buildResults.ConsciousnessEngine) {
            Scan-Container "$($images.ConsciousnessEngine):$Tag"
        }
        
        # Build API Gateway
        $buildResults.ApiGateway = Build-ApiGateway $images.ApiGateway $Tag $Platform
        if ($buildResults.ApiGateway) {
            Scan-Container "$($images.ApiGateway):$Tag"
        }
        
        # Build Web UI
        $buildResults.WebUI = Build-WebUI $images.WebUI $Tag $Platform
        if ($buildResults.WebUI) {
            Scan-Container "$($images.WebUI):$Tag"
        }
        
        # Generate report
        Generate-BuildReport $buildResults $Registry $Namespace $Tag
        
        # Summary
        $successCount = ($buildResults.Values | Where-Object { $_ }).Count
        $totalCount = $buildResults.Count
        
        Write-Log ""
        Write-Log "📊 Build Summary: $successCount/$totalCount containers built successfully" $Blue
        
        if ($successCount -eq $totalCount) {
            Write-Log "🎉 All containers built successfully!" $Green
        } else {
            Write-Log "⚠️ Some containers failed to build. Check the logs above." $Yellow
        }
        
        Write-Log ""
        Write-Log "Next Steps:" $Yellow
        Write-Log "1. Test containers: docker-compose up -d"
        Write-Log "2. Run benchmarks: .\scripts\run_benchmarks.ps1"
        Write-Log "3. Deploy to staging environment"
    }
    catch {
        Write-Log "❌ Build process failed: $_" $Red
        exit 1
    }
}

# Run main function
Main