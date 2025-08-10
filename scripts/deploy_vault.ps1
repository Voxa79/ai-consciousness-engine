# HashiCorp Vault Deployment Script
# Automated deployment and configuration of Vault for Consciousness Platform

param(
    [string]$Environment = "development",
    [string]$VaultVersion = "1.15.2",
    [string]$Namespace = "vault",
    [switch]$InitializeVault = $false,
    [switch]$GenerateCerts = $false,
    [switch]$DryRun = $false,
    [int]$Replicas = 3
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
    Write-Log "Checking Vault deployment prerequisites..." $Blue
    
    $prerequisites = @(
        @{ Name = "kubectl"; Command = "kubectl version --client" },
        @{ Name = "openssl"; Command = "openssl version" },
        @{ Name = "jq"; Command = "jq --version" }
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
    
    return $allPrerequisitesMet
}

function Generate-VaultCertificates {
    Write-Log "Generating TLS certificates for Vault..." $Blue
    
    $certDir = "vault-certs"
    if (-not (Test-Path $certDir)) {
        New-Item -ItemType Directory -Path $certDir -Force | Out-Null
    }
    
    try {
        # Generate CA private key
        openssl genrsa -out "$certDir/ca.key" 4096
        
        # Generate CA certificate
        openssl req -new -x509 -days 365 -key "$certDir/ca.key" -out "$certDir/ca.crt" -subj "/C=US/ST=CA/L=San Francisco/O=Consciousness Platform/OU=Security/CN=Vault CA"
        
        # Generate Vault private key
        openssl genrsa -out "$certDir/vault.key" 4096
        
        # Create certificate signing request
        openssl req -new -key "$certDir/vault.key" -out "$certDir/vault.csr" -subj "/C=US/ST=CA/L=San Francisco/O=Consciousness Platform/OU=Security/CN=vault"
        
        # Create certificate extensions file
        $extFile = @"
authorityKeyIdentifier=keyid,issuer
basicConstraints=CA:FALSE
keyUsage = digitalSignature, nonRepudiation, keyEncipherment, dataEncipherment
subjectAltName = @alt_names

[alt_names]
DNS.1 = vault
DNS.2 = vault.vault
DNS.3 = vault.vault.svc
DNS.4 = vault.vault.svc.cluster.local
DNS.5 = vault.consciousness.ai
DNS.6 = localhost
IP.1 = 127.0.0.1
"@
        
        $extFile | Out-File -FilePath "$certDir/vault.ext" -Encoding ASCII
        
        # Generate Vault certificate
        openssl x509 -req -in "$certDir/vault.csr" -CA "$certDir/ca.crt" -CAkey "$certDir/ca.key" -CAcreateserial -out "$certDir/vault.crt" -days 365 -extensions v3_req -extfile "$certDir/vault.ext"
        
        Write-Log "✅ TLS certificates generated successfully" $Green
        return $true
    }
    catch {
        Write-Log "❌ Failed to generate certificates: $_" $Red
        return $false
    }
}

function Create-VaultSecrets {
    Write-Log "Creating Vault Kubernetes secrets..." $Blue
    
    try {
        if (-not $DryRun) {
            # Create namespace if it doesn't exist
            kubectl create namespace $Namespace --dry-run=client -o yaml | kubectl apply -f -
            
            # Create TLS secret
            if (Test-Path "vault-certs/vault.crt") {
                kubectl create secret tls vault-tls `
                    --cert=vault-certs/vault.crt `
                    --key=vault-certs/vault.key `
                    --namespace=$Namespace `
                    --dry-run=client -o yaml | kubectl apply -f -
                
                # Create CA secret
                kubectl create secret generic vault-ca `
                    --from-file=ca.crt=vault-certs/ca.crt `
                    --namespace=$Namespace `
                    --dry-run=client -o yaml | kubectl apply -f -
            }
            
            Write-Log "✅ Vault secrets created successfully" $Green
        } else {
            Write-Log "🔍 [DRY RUN] Would create Vault secrets" $Blue
        }
        
        return $true
    }
    catch {
        Write-Log "❌ Failed to create Vault secrets: $_" $Red
        return $false
    }
}

function Deploy-VaultManifests {
    Write-Log "Deploying Vault manifests..." $Blue
    
    try {
        if (-not $DryRun) {
            # Apply Vault configuration
            kubectl apply -f k8s/security/vault/vault-config.yaml
            
            # Apply Vault deployment
            kubectl apply -f k8s/security/vault/vault-deployment.yaml
            
            # Wait for Vault pods to be ready
            Write-Log "⏳ Waiting for Vault pods to be ready..." $Yellow
            kubectl wait --for=condition=ready pod -l app=vault -n $Namespace --timeout=300s
            
            Write-Log "✅ Vault deployment completed successfully" $Green
        } else {
            Write-Log "🔍 [DRY RUN] Would deploy Vault manifests" $Blue
        }
        
        return $true
    }
    catch {
        Write-Log "❌ Failed to deploy Vault manifests: $_" $Red
        return $false
    }
}

function Initialize-Vault {
    if (-not $InitializeVault) {
        Write-Log "⏭️ Vault initialization skipped" $Yellow
        return $true
    }
    
    Write-Log "Initializing Vault..." $Blue
    
    try {
        if (-not $DryRun) {
            # Run Vault initialization job
            kubectl apply -f k8s/security/vault/init-job.yaml
            
            # Wait for initialization job to complete
            Write-Log "⏳ Waiting for Vault initialization to complete..." $Yellow
            kubectl wait --for=condition=complete job/vault-init -n $Namespace --timeout=600s
            
            # Check if initialization was successful
            $jobStatus = kubectl get job vault-init -n $Namespace -o jsonpath='{.status.conditions[0].type}'
            
            if ($jobStatus -eq "Complete") {
                Write-Log "✅ Vault initialization completed successfully" $Green
                
                # Display initialization information
                Write-Log "📋 Vault initialization details:" $Blue
                kubectl logs job/vault-init -n $Namespace | Select-String "Vault.*completed"
                
                return $true
            } else {
                Write-Log "❌ Vault initialization failed" $Red
                kubectl logs job/vault-init -n $Namespace
                return $false
            }
        } else {
            Write-Log "🔍 [DRY RUN] Would initialize Vault" $Blue
            return $true
        }
    }
    catch {
        Write-Log "❌ Failed to initialize Vault: $_" $Red
        return $false
    }
}

function Test-VaultHealth {
    Write-Log "Testing Vault health..." $Blue
    
    try {
        # Port forward to Vault service
        $portForwardJob = Start-Job -ScriptBlock {
            kubectl port-forward svc/vault 8200:8200 -n $using:Namespace
        }
        
        Start-Sleep -Seconds 5
        
        # Test Vault health endpoint
        $healthResponse = Invoke-RestMethod -Uri "https://localhost:8200/v1/sys/health" -SkipCertificateCheck -TimeoutSec 10
        
        if ($healthResponse.initialized -and -not $healthResponse.sealed) {
            Write-Log "✅ Vault is healthy and unsealed" $Green
            Write-Log "   Version: $($healthResponse.version)" $Blue
            Write-Log "   Cluster: $($healthResponse.cluster_name)" $Blue
            $healthStatus = $true
        } elseif ($healthResponse.initialized -and $healthResponse.sealed) {
            Write-Log "⚠️ Vault is initialized but sealed" $Yellow
            $healthStatus = $false
        } else {
            Write-Log "❌ Vault is not initialized" $Red
            $healthStatus = $false
        }
        
        # Stop port forward
        Stop-Job $portForwardJob -Force
        Remove-Job $portForwardJob -Force
        
        return $healthStatus
    }
    catch {
        Write-Log "❌ Failed to test Vault health: $_" $Red
        return $false
    }
}

function Configure-VaultIntegration {
    Write-Log "Configuring Vault integration for Consciousness Platform..." $Blue
    
    try {
        if (-not $DryRun) {
            # Create service accounts for Consciousness services
            $serviceAccounts = @(
                "consciousness-engine",
                "api-gateway",
                "web-ui"
            )
            
            foreach ($sa in $serviceAccounts) {
                kubectl create serviceaccount $sa --namespace=default --dry-run=client -o yaml | kubectl apply -f -
                
                # Annotate service account for Vault integration
                kubectl annotate serviceaccount $sa `
                    vault.hashicorp.com/agent-inject="true" `
                    vault.hashicorp.com/role="consciousness-service" `
                    --namespace=default --overwrite
            }
            
            Write-Log "✅ Vault integration configured successfully" $Green
        } else {
            Write-Log "🔍 [DRY RUN] Would configure Vault integration" $Blue
        }
        
        return $true
    }
    catch {
        Write-Log "❌ Failed to configure Vault integration: $_" $Red
        return $false
    }
}

function Setup-VaultMonitoring {
    Write-Log "Setting up Vault monitoring..." $Blue
    
    try {
        if (-not $DryRun) {
            # Create ServiceMonitor for Prometheus
            $serviceMonitor = @"
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: vault
  namespace: $Namespace
  labels:
    app: vault
spec:
  selector:
    matchLabels:
      app: vault
  endpoints:
  - port: http
    path: /v1/sys/metrics
    params:
      format: ['prometheus']
    scheme: https
    tlsConfig:
      insecureSkipVerify: true
    interval: 30s
    scrapeTimeout: 10s
"@
            
            $serviceMonitor | kubectl apply -f -
            
            Write-Log "✅ Vault monitoring configured successfully" $Green
        } else {
            Write-Log "🔍 [DRY RUN] Would setup Vault monitoring" $Blue
        }
        
        return $true
    }
    catch {
        Write-Log "❌ Failed to setup Vault monitoring: $_" $Red
        return $false
    }
}

function Generate-VaultReport {
    param([hashtable]$DeploymentResults)
    
    Write-Log "📋 Generating Vault deployment report..." $Blue
    
    $timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
    $reportFile = "vault_reports/vault_deployment_report_$timestamp.md"
    
    # Create reports directory
    if (-not (Test-Path "vault_reports")) {
        New-Item -ItemType Directory -Path "vault_reports" -Force | Out-Null
    }
    
    $reportContent = @"
# Vault Deployment Report

**Generated:** $(Get-Date)
**Environment:** $Environment
**Vault Version:** $VaultVersion
**Namespace:** $Namespace
**Replicas:** $Replicas

## Deployment Results

### Prerequisites Check
$(if ($DeploymentResults.Prerequisites) { "✅ All prerequisites met" } else { "❌ Prerequisites check failed" })

### Certificate Generation
$(if ($DeploymentResults.Certificates) { "✅ TLS certificates generated" } else { "⏭️ Certificate generation skipped" })

### Vault Deployment
$(if ($DeploymentResults.Deployment) { "✅ Vault deployed successfully" } else { "❌ Vault deployment failed" })

### Vault Initialization
$(if ($DeploymentResults.Initialization) { "✅ Vault initialized successfully" } else { "⏭️ Vault initialization skipped or failed" })

### Health Check
$(if ($DeploymentResults.HealthCheck) { "✅ Vault is healthy" } else { "❌ Vault health check failed" })

### Integration Setup
$(if ($DeploymentResults.Integration) { "✅ Consciousness Platform integration configured" } else { "❌ Integration setup failed" })

### Monitoring Setup
$(if ($DeploymentResults.Monitoring) { "✅ Monitoring configured" } else { "❌ Monitoring setup failed" })

## Vault Configuration

### High Availability
- **Replicas:** $Replicas
- **Storage Backend:** Integrated Storage (Raft)
- **Auto-unsealing:** $(if ($Environment -eq "production") { "Enabled (Cloud KMS)" } else { "Disabled (Development)" })

### Security Features
- **TLS Encryption:** Enabled
- **Audit Logging:** Enabled
- **RBAC:** Configured
- **Network Policies:** Applied

### Secrets Engines
- **KV v2:** Enabled at /consciousness
- **Database:** Enabled for dynamic credentials
- **PKI:** Enabled for internal certificates
- **Kubernetes Auth:** Configured

## Access Information

### Vault UI
- **URL:** https://vault.$Namespace.svc.cluster.local:8200
- **External URL:** https://vault.consciousness.ai (if ingress configured)

### Service Endpoints
- **API:** vault.$Namespace.svc.cluster.local:8200
- **Cluster:** vault.$Namespace.svc.cluster.local:8201

### Authentication
- **Root Token:** Stored in vault-init secret
- **Unseal Keys:** Stored in vault-init secret (5 keys, threshold 3)
- **Service Auth:** Kubernetes auth method

## Next Steps

$(if ($DeploymentResults.HealthCheck) {
@"
1. **Configure application secrets** in Vault
2. **Set up backup procedures** for Vault data
3. **Configure monitoring alerts** for Vault health
4. **Test secret injection** in Consciousness services
5. **Set up key rotation policies**
"@
} else {
@"
1. **Investigate deployment issues** using kubectl logs
2. **Check Vault pod status** and events
3. **Verify network connectivity** to Vault service
4. **Review Vault configuration** for errors
5. **Consider rollback** if issues persist
"@
})

## Troubleshooting

### Common Commands
``````bash
# Check Vault pod status
kubectl get pods -n $Namespace -l app=vault

# View Vault logs
kubectl logs -n $Namespace -l app=vault

# Port forward to Vault UI
kubectl port-forward svc/vault 8200:8200 -n $Namespace

# Check Vault status
vault status

# List Vault policies
vault policy list

# Check Kubernetes auth
vault auth list
``````

### Health Check URLs
- **Health:** https://vault:8200/v1/sys/health
- **Metrics:** https://vault:8200/v1/sys/metrics
- **UI:** https://vault:8200/ui

## Security Considerations

### Production Checklist
- [ ] Auto-unsealing configured with cloud KMS
- [ ] Backup procedures implemented
- [ ] Monitoring and alerting configured
- [ ] Network policies applied
- [ ] Audit logging enabled
- [ ] Key rotation policies defined
- [ ] Disaster recovery tested

**Report completed at:** $(Get-Date)
"@

    $reportContent | Out-File -FilePath $reportFile -Encoding UTF8
    Write-Log "✅ Vault deployment report generated: $reportFile" $Green
}

# Main execution
function Main {
    Write-Log "🔒 HashiCorp Vault Deployment for Consciousness Platform" $Blue
    Write-Log "=====================================================" $Blue
    
    $deploymentResults = @{
        Prerequisites = $false
        Certificates = $false
        Deployment = $false
        Initialization = $false
        HealthCheck = $false
        Integration = $false
        Monitoring = $false
    }
    
    try {
        # Check prerequisites
        $deploymentResults.Prerequisites = Test-Prerequisites
        if (-not $deploymentResults.Prerequisites) {
            Write-Log "❌ Prerequisites not met. Aborting deployment." $Red
            exit 1
        }
        
        # Generate certificates if requested
        if ($GenerateCerts) {
            $deploymentResults.Certificates = Generate-VaultCertificates
        }
        
        # Create Kubernetes secrets
        Create-VaultSecrets
        
        # Deploy Vault
        $deploymentResults.Deployment = Deploy-VaultManifests
        if (-not $deploymentResults.Deployment) {
            Write-Log "❌ Vault deployment failed. Aborting." $Red
            exit 1
        }
        
        # Initialize Vault
        $deploymentResults.Initialization = Initialize-Vault
        
        # Test Vault health
        Start-Sleep -Seconds 30  # Wait for Vault to be ready
        $deploymentResults.HealthCheck = Test-VaultHealth
        
        # Configure integration
        $deploymentResults.Integration = Configure-VaultIntegration
        
        # Setup monitoring
        $deploymentResults.Monitoring = Setup-VaultMonitoring
        
        # Generate report
        Generate-VaultReport $deploymentResults
        
        # Final summary
        Write-Log ""
        if ($deploymentResults.HealthCheck) {
            Write-Log "🎉 Vault deployment completed successfully!" $Green
            Write-Log "🔒 Vault is ready for Consciousness Platform integration" $Green
            Write-Log "📋 Check the deployment report for detailed information" $Blue
        } else {
            Write-Log "⚠️ Vault deployment completed with issues" $Yellow
            Write-Log "🔍 Check the deployment report and logs for troubleshooting" $Yellow
        }
        
        Write-Log ""
        Write-Log "Next Steps:" $Yellow
        Write-Log "1. Configure application secrets in Vault"
        Write-Log "2. Test secret injection in Consciousness services"
        Write-Log "3. Set up backup and monitoring procedures"
        Write-Log "4. Configure auto-unsealing for production"
    }
    catch {
        Write-Log "❌ Vault deployment failed: $_" $Red
        exit 1
    }
}

# Show usage if no parameters
if ($args.Count -eq 0) {
    Write-Host @"
HashiCorp Vault Deployment Script

Usage: .\deploy_vault.ps1 [options]

Options:
  -Environment <env>        Target environment (development, staging, production)
  -VaultVersion <version>   Vault version to deploy (default: 1.15.2)
  -Namespace <namespace>    Kubernetes namespace (default: vault)
  -Replicas <count>         Number of Vault replicas (default: 3)
  -InitializeVault          Initialize Vault after deployment
  -GenerateCerts            Generate TLS certificates for Vault
  -DryRun                   Show what would be deployed without executing

Examples:
  .\deploy_vault.ps1 -Environment development -GenerateCerts -InitializeVault
  .\deploy_vault.ps1 -Environment production -Replicas 5 -DryRun
  .\deploy_vault.ps1 -InitializeVault
"@ -ForegroundColor $Blue
    exit 0
}

# Run main function
Main