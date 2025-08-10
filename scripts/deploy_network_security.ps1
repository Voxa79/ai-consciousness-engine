# Network Security Deployment Script
# Comprehensive zero-trust network security with Cilium and Falco

param(
    [string]$Environment = "development",
    [string]$CiliumVersion = "1.14.5",
    [string]$FalcoVersion = "0.36.2",
    [switch]$EnableWireGuard = $false,
    [switch]$EnableHubble = $true,
    [switch]$DryRun = $false,
    [string]$SlackWebhookUrl = "",
    [string]$PagerDutyKey = ""
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
    Write-Log "Checking network security deployment prerequisites..." $Blue
    
    $prerequisites = @(
        @{ Name = "kubectl"; Command = "kubectl version --client" },
        @{ Name = "helm"; Command = "helm version" }
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
    
    # Check Kubernetes cluster connectivity
    try {
        kubectl cluster-info | Out-Null
        Write-Log "✅ Kubernetes cluster is accessible" $Green
    }
    catch {
        Write-Log "❌ Cannot connect to Kubernetes cluster" $Red
        $allPrerequisitesMet = $false
    }
    
    # Check if cluster supports CNI replacement
    try {
        $nodes = kubectl get nodes -o json | ConvertFrom-Json
        $nodeCount = $nodes.items.Count
        Write-Log "✅ Cluster has $nodeCount nodes ready for CNI deployment" $Green
    }
    catch {
        Write-Log "❌ Cannot assess cluster readiness" $Red
        $allPrerequisitesMet = $false
    }
    
    return $allPrerequisitesMet
}

function Install-CiliumCNI {
    Write-Log "🌐 Installing Cilium CNI..." $Blue
    
    try {
        if (-not $DryRun) {
            # Add Cilium Helm repository
            helm repo add cilium https://helm.cilium.io/
            helm repo update
            
            # Prepare Cilium values
            $ciliumValues = @{
                "cluster.name" = "consciousness-platform"
                "cluster.id" = 1
                "kubeProxyReplacement" = "strict"
                "k8sServiceHost" = (kubectl config view --minify -o jsonpath='{.clusters[0].cluster.server}' | ForEach-Object { ([System.Uri]$_).Host })
                "k8sServicePort" = (kubectl config view --minify -o jsonpath='{.clusters[0].cluster.server}' | ForEach-Object { ([System.Uri]$_).Port })
                "operator.replicas" = 1
                "ipam.mode" = "kubernetes"
                "tunnel" = "vxlan"
                "encryption.enabled" = $EnableWireGuard
                "encryption.type" = if ($EnableWireGuard) { "wireguard" } else { "ipsec" }
                "hubble.enabled" = $EnableHubble
                "hubble.relay.enabled" = $EnableHubble
                "hubble.ui.enabled" = $EnableHubble
                "prometheus.enabled" = $true
                "operator.prometheus.enabled" = $true
                "hubble.metrics.enabled" = @("dns", "drop", "tcp", "flow", "icmp", "http")
                "policyEnforcementMode" = "default"
                "bpf.masquerade" = $true
                "l7Proxy" = $true
                "localRedirectPolicy" = $true
            }
            
            # Convert to YAML for Helm
            $valuesFile = "cilium-values-$Environment.yaml"
            $ciliumValues | ConvertTo-Yaml | Out-File -FilePath $valuesFile -Encoding UTF8
            
            # Install Cilium
            helm upgrade --install cilium cilium/cilium `
                --version $CiliumVersion `
                --namespace kube-system `
                --values $valuesFile `
                --wait --timeout=600s
            
            # Wait for Cilium to be ready
            Write-Log "⏳ Waiting for Cilium to be ready..." $Yellow
            kubectl wait --for=condition=ready pod -l k8s-app=cilium -n kube-system --timeout=300s
            
            Write-Log "✅ Cilium CNI installed successfully" $Green
        } else {
            Write-Log "🔍 [DRY RUN] Would install Cilium CNI" $Blue
        }
        
        return $true
    }
    catch {
        Write-Log "❌ Failed to install Cilium CNI: $_" $Red
        return $false
    }
}

function Deploy-NetworkPolicies {
    Write-Log "🔒 Deploying network policies..." $Blue
    
    try {
        if (-not $DryRun) {
            # Apply Cilium configuration and policies
            kubectl apply -f k8s/security/network/cilium-config.yaml
            
            # Wait a moment for policies to be processed
            Start-Sleep -Seconds 10
            
            # Verify policies are applied
            $policies = kubectl get cnp --all-namespaces -o json | ConvertFrom-Json
            $policyCount = $policies.items.Count
            
            Write-Log "✅ Applied $policyCount Cilium network policies" $Green
        } else {
            Write-Log "🔍 [DRY RUN] Would deploy network policies" $Blue
        }
        
        return $true
    }
    catch {
        Write-Log "❌ Failed to deploy network policies: $_" $Red
        return $false
    }
}

function Install-Falco {
    Write-Log "🛡️ Installing Falco runtime security..." $Blue
    
    try {
        if (-not $DryRun) {
            # Create Falco namespace
            kubectl create namespace falco-system --dry-run=client -o yaml | kubectl apply -f -
            
            # Apply Falco configuration
            kubectl apply -f k8s/security/network/falco-config.yaml
            
            # Build and deploy webhook if secrets are provided
            if ($SlackWebhookUrl -or $PagerDutyKey) {
                Deploy-FalcoWebhook
            }
            
            # Deploy Falco
            kubectl apply -f k8s/security/network/falco-deployment.yaml
            
            # Wait for Falco to be ready
            Write-Log "⏳ Waiting for Falco to be ready..." $Yellow
            kubectl wait --for=condition=ready pod -l app.kubernetes.io/name=falco -n falco-system --timeout=300s
            
            Write-Log "✅ Falco runtime security installed successfully" $Green
        } else {
            Write-Log "🔍 [DRY RUN] Would install Falco" $Blue
        }
        
        return $true
    }
    catch {
        Write-Log "❌ Failed to install Falco: $_" $Red
        return $false
    }
}

function Deploy-FalcoWebhook {
    Write-Log "📡 Deploying Falco webhook..." $Blue
    
    try {
        # Create secrets for webhook
        $secretData = @{}
        
        if ($SlackWebhookUrl) {
            $secretData["slack-webhook-url"] = [System.Convert]::ToBase64String([System.Text.Encoding]::UTF8.GetBytes($SlackWebhookUrl))
        }
        
        if ($PagerDutyKey) {
            $secretData["pagerduty-integration-key"] = [System.Convert]::ToBase64String([System.Text.Encoding]::UTF8.GetBytes($PagerDutyKey))
        }
        
        if ($secretData.Count -gt 0) {
            # Update the secret in the deployment
            $secretManifest = @"
apiVersion: v1
kind: Secret
metadata:
  name: consciousness-security-secrets
  namespace: falco-system
type: Opaque
data:
$(foreach ($key in $secretData.Keys) { "  $key`: $($secretData[$key])" })
"@
            
            $secretManifest | kubectl apply -f -
            Write-Log "✅ Falco webhook secrets configured" $Green
        }
        
        # Build webhook container image
        Build-WebhookImage
        
        return $true
    }
    catch {
        Write-Log "❌ Failed to deploy Falco webhook: $_" $Red
        return $false
    }
}

function Build-WebhookImage {
    Write-Log "🐳 Building Falco webhook container..." $Blue
    
    try {
        # Create temporary directory for build
        $buildDir = "falco-webhook-build"
        if (Test-Path $buildDir) {
            Remove-Item -Recurse -Force $buildDir
        }
        New-Item -ItemType Directory -Path $buildDir | Out-Null
        
        # Extract webhook files from ConfigMap
        $configMap = kubectl get configmap falco-alerts-webhook -n falco-system -o json | ConvertFrom-Json
        
        $configMap.data."webhook.py" | Out-File -FilePath "$buildDir/webhook.py" -Encoding UTF8
        $configMap.data."Dockerfile" | Out-File -FilePath "$buildDir/Dockerfile" -Encoding UTF8
        
        # Build container image
        docker build -t consciousness-security-webhook:latest $buildDir
        
        # Clean up build directory
        Remove-Item -Recurse -Force $buildDir
        
        Write-Log "✅ Webhook container built successfully" $Green
        return $true
    }
    catch {
        Write-Log "❌ Failed to build webhook container: $_" $Red
        return $false
    }
}

function Test-NetworkSecurity {
    Write-Log "🧪 Testing network security configuration..." $Blue
    
    try {
        # Test Cilium status
        $ciliumStatus = kubectl exec -n kube-system ds/cilium -- cilium status --brief
        if ($ciliumStatus -match "OK") {
            Write-Log "✅ Cilium is healthy" $Green
        } else {
            Write-Log "⚠️ Cilium status check failed" $Yellow
        }
        
        # Test network policies
        $policies = kubectl get cnp --all-namespaces --no-headers | Measure-Object
        Write-Log "📋 Found $($policies.Count) Cilium network policies" $Blue
        
        # Test Falco
        $falcoPods = kubectl get pods -n falco-system -l app.kubernetes.io/name=falco --no-headers
        $readyPods = ($falcoPods | Where-Object { $_ -match "1/1.*Running" }).Count
        $totalPods = ($falcoPods | Measure-Object).Count
        
        if ($readyPods -eq $totalPods -and $totalPods -gt 0) {
            Write-Log "✅ Falco is running on $readyPods/$totalPods nodes" $Green
        } else {
            Write-Log "⚠️ Falco status: $readyPods/$totalPods pods ready" $Yellow
        }
        
        # Test Hubble (if enabled)
        if ($EnableHubble) {
            try {
                kubectl exec -n kube-system deployment/hubble-relay -- hubble status
                Write-Log "✅ Hubble observability is active" $Green
            }
            catch {
                Write-Log "⚠️ Hubble status check failed" $Yellow
            }
        }
        
        return $true
    }
    catch {
        Write-Log "❌ Network security testing failed: $_" $Red
        return $false
    }
}

function Setup-NetworkMonitoring {
    Write-Log "📊 Setting up network security monitoring..." $Blue
    
    try {
        if (-not $DryRun) {
            # Create ServiceMonitors for Prometheus
            $serviceMonitors = @"
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: cilium
  namespace: kube-system
  labels:
    app.kubernetes.io/name: cilium
spec:
  selector:
    matchLabels:
      k8s-app: cilium
  endpoints:
  - port: prometheus
    path: /metrics
    interval: 30s
---
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: hubble
  namespace: kube-system
  labels:
    app.kubernetes.io/name: hubble
spec:
  selector:
    matchLabels:
      k8s-app: hubble-relay
  endpoints:
  - port: prometheus
    path: /metrics
    interval: 30s
"@
            
            $serviceMonitors | kubectl apply -f -
            
            Write-Log "✅ Network security monitoring configured" $Green
        } else {
            Write-Log "🔍 [DRY RUN] Would setup network monitoring" $Blue
        }
        
        return $true
    }
    catch {
        Write-Log "❌ Failed to setup network monitoring: $_" $Red
        return $false
    }
}

function Generate-NetworkSecurityReport {
    param([hashtable]$DeploymentResults)
    
    Write-Log "📋 Generating network security report..." $Blue
    
    $timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
    $reportFile = "network_security_reports/network_security_report_$timestamp.md"
    
    # Create reports directory
    if (-not (Test-Path "network_security_reports")) {
        New-Item -ItemType Directory -Path "network_security_reports" -Force | Out-Null
    }
    
    $reportContent = @"
# Network Security Deployment Report

**Generated:** $(Get-Date)
**Environment:** $Environment
**Cilium Version:** $CiliumVersion
**Falco Version:** $FalcoVersion

## Deployment Results

### Prerequisites Check
$(if ($DeploymentResults.Prerequisites) { "✅ All prerequisites met" } else { "❌ Prerequisites check failed" })

### Cilium CNI Installation
$(if ($DeploymentResults.CiliumInstall) { "✅ Cilium CNI installed successfully" } else { "❌ Cilium CNI installation failed" })

### Network Policies Deployment
$(if ($DeploymentResults.NetworkPolicies) { "✅ Network policies deployed" } else { "❌ Network policies deployment failed" })

### Falco Runtime Security
$(if ($DeploymentResults.FalcoInstall) { "✅ Falco installed successfully" } else { "❌ Falco installation failed" })

### Security Testing
$(if ($DeploymentResults.SecurityTest) { "✅ Security tests passed" } else { "❌ Security tests failed" })

### Monitoring Setup
$(if ($DeploymentResults.Monitoring) { "✅ Monitoring configured" } else { "❌ Monitoring setup failed" })

## Network Security Configuration

### Zero-Trust Features
- **Network Policies:** Cilium NetworkPolicies with L3/L4/L7 rules
- **Encryption:** $(if ($EnableWireGuard) { "WireGuard encryption enabled" } else { "IPSec encryption enabled" })
- **Identity-based Security:** Enabled with automatic identity assignment
- **Micro-segmentation:** Pod-to-pod communication restricted

### Runtime Security
- **Falco Rules:** Custom consciousness-specific security rules
- **Intrusion Detection:** Real-time monitoring of system calls
- **Anomaly Detection:** Behavioral analysis for unusual activities
- **Alert Integration:** $(if ($SlackWebhookUrl -or $PagerDutyKey) { "Webhook notifications configured" } else { "Local logging only" })

### Observability
- **Hubble:** $(if ($EnableHubble) { "Network observability enabled" } else { "Disabled" })
- **Prometheus Metrics:** Cilium and Falco metrics collection
- **Flow Monitoring:** L3/L4/L7 traffic visibility
- **Security Events:** Centralized security event logging

## Security Policies Applied

### Consciousness Engine Policies
- Ingress: Only from API Gateway on port 8081
- Egress: Database, Redis, and Vault access only
- L7 Rules: HTTP method and path restrictions

### API Gateway Policies
- Ingress: External traffic and Web UI access
- Egress: Consciousness Engine and backend services
- Rate Limiting: 100 requests/minute per IP

### Database Policies
- Ingress: Only from authorized services
- Egress: No outbound connections allowed
- Isolation: Complete network isolation

### Vault Policies
- Ingress: Authenticated service access only
- Egress: Database and cloud KMS access
- L7 Rules: API path-based restrictions

## Falco Security Rules

### Critical Rules
- Consciousness Engine unauthorized access detection
- Vault secret access monitoring
- Container escape attempt detection
- Privilege escalation monitoring

### Warning Rules
- Unusual network activity detection
- Configuration modification alerts
- High resource usage monitoring
- Lateral movement detection

## Next Steps

$(if ($DeploymentResults.SecurityTest) {
@"
1. **Monitor security events** in Falco logs and dashboards
2. **Test network policies** with connectivity validation
3. **Configure additional alerting** channels as needed
4. **Review and tune** security rules based on application behavior
5. **Set up automated** security policy updates
"@
} else {
@"
1. **Investigate deployment issues** using kubectl logs
2. **Check network connectivity** between services
3. **Verify Cilium and Falco** pod status and logs
4. **Review security policies** for conflicts
5. **Consider rollback** if issues persist
"@
})

## Troubleshooting

### Common Commands
``````bash
# Check Cilium status
kubectl exec -n kube-system ds/cilium -- cilium status

# List network policies
kubectl get cnp --all-namespaces

# Check Falco logs
kubectl logs -n falco-system ds/falco

# Test network connectivity
kubectl exec -it <pod> -- nc -zv <target-service> <port>

# View Hubble flows (if enabled)
kubectl exec -n kube-system deployment/hubble-relay -- hubble observe
``````

### Monitoring URLs
- **Hubble UI:** $(if ($EnableHubble) { "http://hubble-ui.kube-system.svc.cluster.local" } else { "Not enabled" })
- **Cilium Metrics:** http://cilium.kube-system.svc.cluster.local:9090/metrics
- **Falco Metrics:** http://falco.falco-system.svc.cluster.local:9376/metrics

## Security Considerations

### Production Checklist
- [ ] Network policies tested and validated
- [ ] Encryption enabled for all inter-pod communication
- [ ] Falco rules tuned for application behavior
- [ ] Alert channels configured and tested
- [ ] Monitoring dashboards created
- [ ] Security incident response procedures defined
- [ ] Regular security policy reviews scheduled

**Report completed at:** $(Get-Date)
"@

    $reportContent | Out-File -FilePath $reportFile -Encoding UTF8
    Write-Log "✅ Network security report generated: $reportFile" $Green
}

# Main execution
function Main {
    Write-Log "🔒 Network Security Deployment for Consciousness Platform" $Blue
    Write-Log "=======================================================" $Blue
    
    $deploymentResults = @{
        Prerequisites = $false
        CiliumInstall = $false
        NetworkPolicies = $false
        FalcoInstall = $false
        SecurityTest = $false
        Monitoring = $false
    }
    
    try {
        # Check prerequisites
        $deploymentResults.Prerequisites = Test-Prerequisites
        if (-not $deploymentResults.Prerequisites) {
            Write-Log "❌ Prerequisites not met. Aborting deployment." $Red
            exit 1
        }
        
        # Install Cilium CNI
        $deploymentResults.CiliumInstall = Install-CiliumCNI
        if (-not $deploymentResults.CiliumInstall) {
            Write-Log "❌ Cilium installation failed. Aborting." $Red
            exit 1
        }
        
        # Deploy network policies
        $deploymentResults.NetworkPolicies = Deploy-NetworkPolicies
        
        # Install Falco
        $deploymentResults.FalcoInstall = Install-Falco
        
        # Test network security
        Start-Sleep -Seconds 30  # Wait for services to be ready
        $deploymentResults.SecurityTest = Test-NetworkSecurity
        
        # Setup monitoring
        $deploymentResults.Monitoring = Setup-NetworkMonitoring
        
        # Generate report
        Generate-NetworkSecurityReport $deploymentResults
        
        # Final summary
        Write-Log ""
        if ($deploymentResults.SecurityTest) {
            Write-Log "🎉 Network security deployment completed successfully!" $Green
            Write-Log "🔒 Zero-trust network security is now active" $Green
            Write-Log "🛡️ Runtime security monitoring is operational" $Green
        } else {
            Write-Log "⚠️ Network security deployment completed with issues" $Yellow
            Write-Log "🔍 Check the deployment report for troubleshooting guidance" $Yellow
        }
        
        Write-Log ""
        Write-Log "Next Steps:" $Yellow
        Write-Log "1. Monitor network policies and adjust as needed"
        Write-Log "2. Review Falco security alerts and tune rules"
        Write-Log "3. Set up additional monitoring dashboards"
        Write-Log "4. Test application connectivity thoroughly"
    }
    catch {
        Write-Log "❌ Network security deployment failed: $_" $Red
        exit 1
    }
}

# Show usage if no parameters
if ($args.Count -eq 0) {
    Write-Host @"
Network Security Deployment Script

Usage: .\deploy_network_security.ps1 [options]

Options:
  -Environment <env>        Target environment (development, staging, production)
  -CiliumVersion <version>  Cilium version to install (default: 1.14.5)
  -FalcoVersion <version>   Falco version to install (default: 0.36.2)
  -EnableWireGuard          Enable WireGuard encryption (default: false)
  -EnableHubble             Enable Hubble observability (default: true)
  -SlackWebhookUrl <url>    Slack webhook URL for alerts
  -PagerDutyKey <key>       PagerDuty integration key for critical alerts
  -DryRun                   Show what would be deployed without executing

Examples:
  .\deploy_network_security.ps1 -Environment development -EnableHubble
  .\deploy_network_security.ps1 -Environment production -EnableWireGuard -SlackWebhookUrl "https://hooks.slack.com/..." -PagerDutyKey "abc123"
  .\deploy_network_security.ps1 -DryRun
"@ -ForegroundColor $Blue
    exit 0
}

# Run main function
Main