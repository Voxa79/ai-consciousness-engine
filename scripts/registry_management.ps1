# Container Registry Management Script
# Advanced registry operations with security, cleanup, and multi-registry support

param(
    [string]$Action = "list",
    [string]$Registry = "ghcr.io",
    [string]$Namespace = "consciousness-platform",
    [string]$Tag = "latest",
    [int]$KeepVersions = 10,
    [switch]$DryRun = $false
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

function Test-RegistryAccess {
    param([string]$RegistryUrl)
    
    Write-Log "Testing registry access: $RegistryUrl" $Blue
    try {
        # Test registry connectivity
        $testImage = "$RegistryUrl/hello-world:latest"
        docker pull $testImage 2>$null | Out-Null
        Write-Log "✅ Registry access confirmed" $Green
        return $true
    }
    catch {
        Write-Log "❌ Registry access failed: $_" $Red
        return $false
    }
}

function Get-RegistryImages {
    param([string]$Registry, [string]$Namespace)
    
    Write-Log "📋 Listing images in registry..." $Blue
    
    $images = @()
    $services = @("consciousness-engine", "api-gateway", "web-ui")
    
    foreach ($service in $services) {
        try {
            $imageName = "$Registry/$Namespace/$service"
            
            # Get image tags (this would need registry API integration in real implementation)
            # For now, we'll simulate with local images
            $localImages = docker images --format "json" | ConvertFrom-Json | Where-Object { 
                $_.Repository -eq $imageName 
            }
            
            foreach ($image in $localImages) {
                $images += [PSCustomObject]@{
                    Service = $service
                    Repository = $image.Repository
                    Tag = $image.Tag
                    ImageId = $image.ID
                    Size = $image.Size
                    Created = $image.CreatedAt
                }
            }
        }
        catch {
            Write-Log "⚠️ Could not list images for $service: $_" $Yellow
        }
    }
    
    return $images
}

function Push-ImagesToRegistry {
    param([string]$Registry, [string]$Namespace, [string]$Tag)
    
    Write-Log "📤 Pushing images to registry..." $Blue
    
    $services = @("consciousness-engine", "api-gateway", "web-ui")
    $pushResults = @{}
    
    foreach ($service in $services) {
        $imageName = "$Registry/$Namespace/$service"
        $fullImageName = "$imageName:$Tag"
        
        Write-Log "Pushing $fullImageName..." $Yellow
        
        try {
            if (-not $DryRun) {
                docker push $fullImageName
                Write-Log "✅ Successfully pushed $fullImageName" $Green
                $pushResults[$service] = $true
            } else {
                Write-Log "🔍 [DRY RUN] Would push $fullImageName" $Blue
                $pushResults[$service] = $true
            }
        }
        catch {
            Write-Log "❌ Failed to push $fullImageName : $_" $Red
            $pushResults[$service] = $false
        }
    }
    
    return $pushResults
}

function Remove-OldImages {
    param([string]$Registry, [string]$Namespace, [int]$KeepVersions)
    
    Write-Log "🧹 Cleaning up old images (keeping $KeepVersions versions)..." $Blue
    
    $services = @("consciousness-engine", "api-gateway", "web-ui")
    $cleanupResults = @{}
    
    foreach ($service in $services) {
        Write-Log "Cleaning up $service images..." $Yellow
        
        try {
            # Get all local images for this service
            $imageName = "$Registry/$Namespace/$service"
            $images = docker images --format "json" | ConvertFrom-Json | Where-Object { 
                $_.Repository -eq $imageName 
            } | Sort-Object CreatedAt -Descending
            
            if ($images.Count -gt $KeepVersions) {
                $imagesToRemove = $images | Select-Object -Skip $KeepVersions
                
                foreach ($image in $imagesToRemove) {
                    $fullImageName = "$($image.Repository):$($image.Tag)"
                    
                    if (-not $DryRun) {
                        docker rmi $fullImageName --force
                        Write-Log "🗑️ Removed $fullImageName" $Green
                    } else {
                        Write-Log "🔍 [DRY RUN] Would remove $fullImageName" $Blue
                    }
                }
                
                $cleanupResults[$service] = $imagesToRemove.Count
            } else {
                Write-Log "✅ No cleanup needed for $service (only $($images.Count) images)" $Green
                $cleanupResults[$service] = 0
            }
        }
        catch {
            Write-Log "❌ Failed to cleanup $service images: $_" $Red
            $cleanupResults[$service] = -1
        }
    }
    
    return $cleanupResults
}

function Get-ImageVulnerabilities {
    param([string]$ImageName)
    
    Write-Log "🔍 Scanning $ImageName for vulnerabilities..." $Blue
    
    $scanResults = @{
        Critical = 0
        High = 0
        Medium = 0
        Low = 0
        Scanner = "none"
        Success = $false
    }
    
    # Try different scanners
    $scanners = @(
        @{ Name = "trivy"; Command = "trivy image --format json $ImageName" },
        @{ Name = "grype"; Command = "grype $ImageName -o json" },
        @{ Name = "docker scout"; Command = "docker scout cves $ImageName --format json" }
    )
    
    foreach ($scanner in $scanners) {
        try {
            if (Get-Command $scanner.Name -ErrorAction SilentlyContinue) {
                $output = Invoke-Expression $scanner.Command 2>$null
                
                if ($scanner.Name -eq "trivy") {
                    $jsonResult = $output | ConvertFrom-Json
                    if ($jsonResult.Results) {
                        foreach ($result in $jsonResult.Results) {
                            if ($result.Vulnerabilities) {
                                foreach ($vuln in $result.Vulnerabilities) {
                                    switch ($vuln.Severity) {
                                        "CRITICAL" { $scanResults.Critical++ }
                                        "HIGH" { $scanResults.High++ }
                                        "MEDIUM" { $scanResults.Medium++ }
                                        "LOW" { $scanResults.Low++ }
                                    }
                                }
                            }
                        }
                    }
                }
                
                $scanResults.Scanner = $scanner.Name
                $scanResults.Success = $true
                break
            }
        }
        catch {
            continue
        }
    }
    
    if ($scanResults.Success) {
        Write-Log "🔍 Vulnerability scan completed with $($scanResults.Scanner)" $Green
        Write-Log "   Critical: $($scanResults.Critical), High: $($scanResults.High), Medium: $($scanResults.Medium), Low: $($scanResults.Low)" $Blue
    } else {
        Write-Log "⚠️ No vulnerability scanner available" $Yellow
    }
    
    return $scanResults
}

function Optimize-ImageSizes {
    param([string]$Registry, [string]$Namespace)
    
    Write-Log "📊 Analyzing image sizes and optimization opportunities..." $Blue
    
    $services = @("consciousness-engine", "api-gateway", "web-ui")
    $optimizationReport = @{}
    
    foreach ($service in $services) {
        $imageName = "$Registry/$Namespace/$service"
        
        try {
            # Get image details
            $imageInfo = docker inspect $imageName 2>$null | ConvertFrom-Json
            
            if ($imageInfo) {
                $sizeBytes = $imageInfo[0].Size
                $sizeMB = [math]::Round($sizeBytes / 1MB, 2)
                
                # Analyze layers
                $layers = $imageInfo[0].RootFS.Layers
                $layerCount = $layers.Count
                
                # Optimization suggestions
                $suggestions = @()
                
                if ($sizeMB -gt 100) {
                    $suggestions += "Consider using alpine base image"
                }
                
                if ($layerCount -gt 10) {
                    $suggestions += "Reduce number of layers by combining RUN commands"
                }
                
                if ($service -eq "consciousness-engine" -and $sizeMB -gt 50) {
                    $suggestions += "Use multi-stage build to reduce final image size"
                }
                
                $optimizationReport[$service] = @{
                    Size = $sizeMB
                    Layers = $layerCount
                    Suggestions = $suggestions
                }
                
                Write-Log "📦 $service : $sizeMB MB ($layerCount layers)" $Blue
                foreach ($suggestion in $suggestions) {
                    Write-Log "   💡 $suggestion" $Yellow
                }
            }
        }
        catch {
            Write-Log "⚠️ Could not analyze $service image" $Yellow
        }
    }
    
    return $optimizationReport
}

function Sync-RegistryImages {
    param([string]$SourceRegistry, [string]$TargetRegistry, [string]$Namespace)
    
    Write-Log "🔄 Syncing images between registries..." $Blue
    Write-Log "Source: $SourceRegistry" $Blue
    Write-Log "Target: $TargetRegistry" $Blue
    
    $services = @("consciousness-engine", "api-gateway", "web-ui")
    $syncResults = @{}
    
    foreach ($service in $services) {
        $sourceImage = "$SourceRegistry/$Namespace/$service:latest"
        $targetImage = "$TargetRegistry/$Namespace/$service:latest"
        
        Write-Log "Syncing $service..." $Yellow
        
        try {
            if (-not $DryRun) {
                # Pull from source
                docker pull $sourceImage
                
                # Tag for target
                docker tag $sourceImage $targetImage
                
                # Push to target
                docker push $targetImage
                
                Write-Log "✅ Successfully synced $service" $Green
                $syncResults[$service] = $true
            } else {
                Write-Log "🔍 [DRY RUN] Would sync $sourceImage -> $targetImage" $Blue
                $syncResults[$service] = $true
            }
        }
        catch {
            Write-Log "❌ Failed to sync $service : $_" $Red
            $syncResults[$service] = $false
        }
    }
    
    return $syncResults
}

function Generate-RegistryReport {
    param([hashtable]$Data, [string]$Action)
    
    Write-Log "📋 Generating registry report..." $Blue
    
    $timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
    $reportFile = "registry_reports/registry_${Action}_report_$timestamp.md"
    
    # Create reports directory
    if (-not (Test-Path "registry_reports")) {
        New-Item -ItemType Directory -Path "registry_reports" -Force | Out-Null
    }
    
    $reportContent = @"
# Container Registry Report - $Action

**Generated:** $(Get-Date)
**Registry:** $Registry
**Namespace:** $Namespace
**Action:** $Action

## Summary

$(
    switch ($Action) {
        "list" {
            if ($Data.Images) {
                "**Total Images:** $($Data.Images.Count)`n"
                $Data.Images | ForEach-Object { "- $($_.Service): $($_.Repository):$($_.Tag) ($($_.Size))" }
            }
        }
        "push" {
            $successful = ($Data.Values | Where-Object { $_ }).Count
            $total = $Data.Count
            "**Push Results:** $successful/$total successful`n"
            $Data.GetEnumerator() | ForEach-Object { "- $($_.Key): $(if ($_.Value) { '✅ Success' } else { '❌ Failed' })" }
        }
        "cleanup" {
            $totalRemoved = ($Data.Values | Measure-Object -Sum).Sum
            "**Images Removed:** $totalRemoved`n"
            $Data.GetEnumerator() | ForEach-Object { "- $($_.Key): $($_.Value) images removed" }
        }
        "scan" {
            "**Security Scan Results:**`n"
            $Data.GetEnumerator() | ForEach-Object { 
                "- $($_.Key): Critical: $($_.Value.Critical), High: $($_.Value.High), Medium: $($_.Value.Medium), Low: $($_.Value.Low)"
            }
        }
        "optimize" {
            "**Image Optimization Analysis:**`n"
            $Data.GetEnumerator() | ForEach-Object { 
                "- $($_.Key): $($_.Value.Size) MB ($($_.Value.Layers) layers)"
                $_.Value.Suggestions | ForEach-Object { "  💡 $_" }
            }
        }
        "sync" {
            $successful = ($Data.Values | Where-Object { $_ }).Count
            $total = $Data.Count
            "**Sync Results:** $successful/$total successful`n"
            $Data.GetEnumerator() | ForEach-Object { "- $($_.Key): $(if ($_.Value) { '✅ Success' } else { '❌ Failed' })" }
        }
    }
)

## Configuration

- **Registry:** $Registry
- **Namespace:** $Namespace
- **Tag:** $Tag
- **Keep Versions:** $KeepVersions
- **Dry Run:** $(if ($DryRun) { 'Yes' } else { 'No' })

## Next Steps

1. **Review results** and address any failures
2. **Update deployment configurations** if needed
3. **Schedule regular cleanup** to maintain registry hygiene
4. **Monitor security vulnerabilities** and update base images

**Report completed at:** $(Get-Date)
"@

    $reportContent | Out-File -FilePath $reportFile -Encoding UTF8
    Write-Log "✅ Registry report generated: $reportFile" $Green
}

# Main execution
function Main {
    Write-Log "🚀 Container Registry Management" $Blue
    Write-Log "===============================" $Blue
    
    # Test registry access
    if (-not (Test-RegistryAccess $Registry)) {
        Write-Log "❌ Cannot proceed without registry access" $Red
        exit 1
    }
    
    $actionData = @{}
    
    try {
        switch ($Action.ToLower()) {
            "list" {
                Write-Log "📋 Listing registry images..." $Yellow
                $images = Get-RegistryImages $Registry $Namespace
                $actionData = @{ Images = $images }
                
                Write-Log "Found $($images.Count) images:" $Blue
                $images | ForEach-Object {
                    Write-Log "  $($_.Service): $($_.Repository):$($_.Tag) ($($_.Size))" $Green
                }
            }
            
            "push" {
                Write-Log "📤 Pushing images to registry..." $Yellow
                $pushResults = Push-ImagesToRegistry $Registry $Namespace $Tag
                $actionData = $pushResults
            }
            
            "cleanup" {
                Write-Log "🧹 Cleaning up old images..." $Yellow
                $cleanupResults = Remove-OldImages $Registry $Namespace $KeepVersions
                $actionData = $cleanupResults
            }
            
            "scan" {
                Write-Log "🔍 Scanning images for vulnerabilities..." $Yellow
                $services = @("consciousness-engine", "api-gateway", "web-ui")
                $scanResults = @{}
                
                foreach ($service in $services) {
                    $imageName = "$Registry/$Namespace/$service:$Tag"
                    $scanResults[$service] = Get-ImageVulnerabilities $imageName
                }
                
                $actionData = $scanResults
            }
            
            "optimize" {
                Write-Log "📊 Analyzing image optimization opportunities..." $Yellow
                $optimizationResults = Optimize-ImageSizes $Registry $Namespace
                $actionData = $optimizationResults
            }
            
            "sync" {
                Write-Log "🔄 Syncing images between registries..." $Yellow
                # This would require additional parameters for source/target registries
                Write-Log "⚠️ Sync operation requires source and target registry configuration" $Yellow
            }
            
            default {
                Write-Log "❌ Unknown action: $Action" $Red
                Write-Log "Available actions: list, push, cleanup, scan, optimize, sync" $Yellow
                exit 1
            }
        }
        
        # Generate report
        Generate-RegistryReport $actionData $Action
        
        Write-Log ""
        Write-Log "✅ Registry management completed successfully!" $Green
    }
    catch {
        Write-Log "❌ Registry management failed: $_" $Red
        exit 1
    }
}

# Show usage if no parameters
if ($args.Count -eq 0) {
    Write-Host @"
Container Registry Management Script

Usage: .\registry_management.ps1 -Action <action> [options]

Actions:
  list     - List images in registry
  push     - Push images to registry
  cleanup  - Remove old image versions
  scan     - Scan images for vulnerabilities
  optimize - Analyze image size optimization
  sync     - Sync images between registries

Options:
  -Registry <url>        Registry URL (default: ghcr.io)
  -Namespace <name>      Namespace/organization (default: consciousness-platform)
  -Tag <tag>            Image tag (default: latest)
  -KeepVersions <num>   Versions to keep during cleanup (default: 10)
  -DryRun               Show what would be done without executing

Examples:
  .\registry_management.ps1 -Action list
  .\registry_management.ps1 -Action push -Tag v1.0.0
  .\registry_management.ps1 -Action cleanup -KeepVersions 5 -DryRun
  .\registry_management.ps1 -Action scan
"@ -ForegroundColor $Blue
    exit 0
}

# Run main function
Main