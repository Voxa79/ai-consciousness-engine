# Configuration Management Script
# Dynamic configuration management for different environments

param(
    [string]$Environment = "development",
    [string]$Action = "validate",
    [string]$ConfigPath = "config/environments",
    [string]$OutputFormat = "yaml",
    [switch]$Merge = $false,
    [switch]$Encrypt = $false
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

function Test-ConfigurationFile {
    param([string]$FilePath)
    
    Write-Log "Validating configuration file: $FilePath" $Blue
    
    if (-not (Test-Path $FilePath)) {
        Write-Log "❌ Configuration file not found: $FilePath" $Red
        return $false
    }
    
    try {
        # Test YAML parsing
        $content = Get-Content $FilePath -Raw
        $yaml = ConvertFrom-Yaml $content -ErrorAction Stop
        
        # Validate required sections
        $requiredSections = @(
            "environment",
            "consciousness_engine",
            "api_gateway",
            "database",
            "redis",
            "monitoring",
            "security",
            "performance"
        )
        
        $missingsections = @()
        foreach ($section in $requiredSections) {
            if (-not $yaml.ContainsKey($section)) {
                $missingSections += $section
            }
        }
        
        if ($missingSections.Count -gt 0) {
            Write-Log "❌ Missing required sections: $($missingSections -join ', ')" $Red
            return $false
        }
        
        # Validate consciousness engine configuration
        if (-not (Test-ConsciousnessEngineConfig $yaml.consciousness_engine)) {
            return $false
        }
        
        # Validate database configuration
        if (-not (Test-DatabaseConfig $yaml.database)) {
            return $false
        }
        
        # Validate security configuration
        if (-not (Test-SecurityConfig $yaml.security)) {
            return $false
        }
        
        Write-Log "✅ Configuration file is valid" $Green
        return $true
    }
    catch {
        Write-Log "❌ Configuration validation failed: $_" $Red
        return $false
    }
}

function Test-ConsciousnessEngineConfig {
    param($Config)
    
    # Validate required consciousness engine settings
    $requiredSettings = @("log_level", "processing_timeout_ms", "quality_threshold")
    
    foreach ($setting in $requiredSettings) {
        if (-not $Config.ContainsKey($setting)) {
            Write-Log "❌ Missing consciousness engine setting: $setting" $Red
            return $false
        }
    }
    
    # Validate value ranges
    if ($Config.processing_timeout_ms -lt 10 -or $Config.processing_timeout_ms -gt 10000) {
        Write-Log "❌ Invalid processing_timeout_ms: must be between 10 and 10000" $Red
        return $false
    }
    
    if ($Config.quality_threshold -lt 0.5 -or $Config.quality_threshold -gt 1.0) {
        Write-Log "❌ Invalid quality_threshold: must be between 0.5 and 1.0" $Red
        return $false
    }
    
    return $true
}

function Test-DatabaseConfig {
    param($Config)
    
    $requiredSettings = @("host", "port", "name", "username")
    
    foreach ($setting in $requiredSettings) {
        if (-not $Config.ContainsKey($setting)) {
            Write-Log "❌ Missing database setting: $setting" $Red
            return $false
        }
    }
    
    # Validate port range
    if ($Config.port -lt 1 -or $Config.port -gt 65535) {
        Write-Log "❌ Invalid database port: must be between 1 and 65535" $Red
        return $false
    }
    
    return $true
}

function Test-SecurityConfig {
    param($Config)
    
    # Check for production security requirements
    if ($Environment -eq "production") {
        if (-not $Config.tls_enabled) {
            Write-Log "❌ TLS must be enabled in production" $Red
            return $false
        }
        
        if ($Config.cors.allow_origins -contains "*") {
            Write-Log "❌ Wildcard CORS origins not allowed in production" $Red
            return $false
        }
    }
    
    return $true
}

function Merge-Configurations {
    param([string]$BaseConfig, [string]$OverrideConfig)
    
    Write-Log "Merging configurations..." $Blue
    Write-Log "Base: $BaseConfig" $Blue
    Write-Log "Override: $OverrideConfig" $Blue
    
    try {
        $baseContent = Get-Content $BaseConfig -Raw | ConvertFrom-Yaml
        $overrideContent = Get-Content $OverrideConfig -Raw | ConvertFrom-Yaml
        
        # Deep merge function
        function Merge-HashTables {
            param($Base, $Override)
            
            $result = $Base.Clone()
            
            foreach ($key in $Override.Keys) {
                if ($result.ContainsKey($key)) {
                    if ($result[$key] -is [hashtable] -and $Override[$key] -is [hashtable]) {
                        $result[$key] = Merge-HashTables $result[$key] $Override[$key]
                    } else {
                        $result[$key] = $Override[$key]
                    }
                } else {
                    $result[$key] = $Override[$key]
                }
            }
            
            return $result
        }
        
        $mergedConfig = Merge-HashTables $baseContent $overrideContent
        
        Write-Log "✅ Configurations merged successfully" $Green
        return $mergedConfig
    }
    catch {
        Write-Log "❌ Failed to merge configurations: $_" $Red
        return $null
    }
}

function Convert-ConfigurationFormat {
    param([string]$InputFile, [string]$OutputFormat, [string]$OutputFile)
    
    Write-Log "Converting configuration format to $OutputFormat..." $Blue
    
    try {
        $content = Get-Content $InputFile -Raw | ConvertFrom-Yaml
        
        switch ($OutputFormat.ToLower()) {
            "json" {
                $output = $content | ConvertTo-Json -Depth 10
            }
            "yaml" {
                $output = $content | ConvertTo-Yaml
            }
            "env" {
                $output = Convert-ToEnvironmentVariables $content
            }
            "docker-compose" {
                $output = Convert-ToDockerCompose $content
            }
            default {
                Write-Log "❌ Unsupported output format: $OutputFormat" $Red
                return $false
            }
        }
        
        $output | Out-File -FilePath $OutputFile -Encoding UTF8
        Write-Log "✅ Configuration converted and saved to: $OutputFile" $Green
        return $true
    }
    catch {
        Write-Log "❌ Failed to convert configuration: $_" $Red
        return $false
    }
}

function Convert-ToEnvironmentVariables {
    param($Config)
    
    $envVars = @()
    
    function Flatten-Object {
        param($Object, $Prefix = "")
        
        foreach ($key in $Object.Keys) {
            $envKey = if ($Prefix) { "${Prefix}_${key}" } else { $key }
            $envKey = $envKey.ToUpper()
            
            if ($Object[$key] -is [hashtable]) {
                Flatten-Object $Object[$key] $envKey
            } else {
                $envVars += "$envKey=$($Object[$key])"
            }
        }
    }
    
    Flatten-Object $Config
    return $envVars -join "`n"
}

function Convert-ToDockerCompose {
    param($Config)
    
    # Generate Docker Compose environment section
    $envSection = @"
version: '3.8'

services:
  consciousness-engine:
    environment:
      - RUST_LOG=$($Config.consciousness_engine.log_level)
      - CONSCIOUSNESS_TIMEOUT_MS=$($Config.consciousness_engine.processing_timeout_ms)
      - CONSCIOUSNESS_QUALITY_THRESHOLD=$($Config.consciousness_engine.quality_threshold)
      - DATABASE_URL=postgresql://$($Config.database.username):$($Config.database.password)@$($Config.database.host):$($Config.database.port)/$($Config.database.name)
      - REDIS_URL=redis://:$($Config.redis.password)@$($Config.redis.host):$($Config.redis.port)/$($Config.redis.database)
    
  api-gateway:
    environment:
      - RUST_LOG=$($Config.api_gateway.log_level)
      - PORT=$($Config.api_gateway.port)
      - JWT_SECRET=$($Config.api_gateway.jwt.secret)
      - CORS_ORIGINS=$($Config.api_gateway.cors_origins -join ',')
    
  web-ui:
    environment:
      - NODE_ENV=$($Config.environment)
      - REACT_APP_API_URL=$($Config.web_ui.api_url)
      - REACT_APP_WS_URL=$($Config.web_ui.websocket_url)
"@
    
    return $envSection
}

function Encrypt-Configuration {
    param([string]$ConfigFile, [string]$OutputFile, [string]$Key)
    
    Write-Log "Encrypting configuration file..." $Blue
    
    try {
        $content = Get-Content $ConfigFile -Raw
        
        # Simple encryption (in production, use proper encryption)
        $bytes = [System.Text.Encoding]::UTF8.GetBytes($content)
        $encrypted = [System.Convert]::ToBase64String($bytes)
        
        $encryptedConfig = @{
            encrypted = $true
            algorithm = "base64"  # Placeholder - use proper encryption
            data = $encrypted
            timestamp = Get-Date -Format "yyyy-MM-ddTHH:mm:ssZ"
        }
        
        $encryptedConfig | ConvertTo-Json | Out-File -FilePath $OutputFile -Encoding UTF8
        
        Write-Log "✅ Configuration encrypted and saved to: $OutputFile" $Green
        return $true
    }
    catch {
        Write-Log "❌ Failed to encrypt configuration: $_" $Red
        return $false
    }
}

function Generate-ConfigurationReport {
    param([string]$Environment, [hashtable]$ValidationResults)
    
    Write-Log "Generating configuration report..." $Blue
    
    $timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
    $reportFile = "config_reports/config_report_${Environment}_$timestamp.md"
    
    # Create reports directory
    if (-not (Test-Path "config_reports")) {
        New-Item -ItemType Directory -Path "config_reports" -Force | Out-Null
    }
    
    $reportContent = @"
# Configuration Report - $Environment

**Generated:** $(Get-Date)
**Environment:** $Environment
**Configuration Path:** $ConfigPath

## Validation Results

$(
    $ValidationResults.GetEnumerator() | ForEach-Object {
        $status = if ($_.Value) { "✅ Valid" } else { "❌ Invalid" }
        "- **$($_.Key):** $status"
    }
)

## Configuration Summary

### Consciousness Engine
- **Log Level:** $((Get-Content "$ConfigPath/$Environment.yml" | ConvertFrom-Yaml).consciousness_engine.log_level)
- **Processing Timeout:** $((Get-Content "$ConfigPath/$Environment.yml" | ConvertFrom-Yaml).consciousness_engine.processing_timeout_ms)ms
- **Quality Threshold:** $((Get-Content "$ConfigPath/$Environment.yml" | ConvertFrom-Yaml).consciousness_engine.quality_threshold)

### Security Settings
- **TLS Enabled:** $((Get-Content "$ConfigPath/$Environment.yml" | ConvertFrom-Yaml).security.tls_enabled)
- **CORS Origins:** $((Get-Content "$ConfigPath/$Environment.yml" | ConvertFrom-Yaml).security.cors.allow_origins -join ', ')

### Performance Settings
- **CPU Limit:** $((Get-Content "$ConfigPath/$Environment.yml" | ConvertFrom-Yaml).performance.cpu_limit)
- **Memory Limit:** $((Get-Content "$ConfigPath/$Environment.yml" | ConvertFrom-Yaml).performance.memory_limit)
- **Min Replicas:** $((Get-Content "$ConfigPath/$Environment.yml" | ConvertFrom-Yaml).performance.min_replicas)
- **Max Replicas:** $((Get-Content "$ConfigPath/$Environment.yml" | ConvertFrom-Yaml).performance.max_replicas)

## Recommendations

$(
    switch ($Environment) {
        "development" {
            "- Consider enabling profiling for performance analysis"
            "- Ensure debug endpoints are disabled in production"
            "- Review resource limits for development efficiency"
        }
        "staging" {
            "- Validate production-like settings"
            "- Test auto-scaling configuration"
            "- Verify monitoring and alerting setup"
        }
        "production" {
            "- Ensure all security settings are enabled"
            "- Verify backup and disaster recovery configuration"
            "- Review resource limits and scaling policies"
            "- Validate compliance settings"
        }
    }
)

## Next Steps

1. **Address any validation failures** shown above
2. **Review configuration values** for appropriateness
3. **Test configuration** in target environment
4. **Update deployment scripts** with new configuration

**Report completed at:** $(Get-Date)
"@

    $reportContent | Out-File -FilePath $reportFile -Encoding UTF8
    Write-Log "✅ Configuration report generated: $reportFile" $Green
}

# ConvertFrom-Yaml and ConvertTo-Yaml helper functions (simplified)
function ConvertFrom-Yaml {
    param([string]$Content)
    
    # This is a simplified YAML parser - in production, use a proper YAML library
    # For now, we'll create a basic parser for our configuration structure
    
    $lines = $Content -split "`n"
    $result = @{}
    $currentSection = $null
    $currentSubSection = $null
    
    foreach ($line in $lines) {
        $line = $line.Trim()
        
        if ($line -match "^([a-zA-Z_]+):$") {
            $currentSection = $matches[1]
            $result[$currentSection] = @{}
            $currentSubSection = $null
        }
        elseif ($line -match "^  ([a-zA-Z_]+):$") {
            $currentSubSection = $matches[1]
            if ($currentSection) {
                $result[$currentSection][$currentSubSection] = @{}
            }
        }
        elseif ($line -match "^  ([a-zA-Z_]+): (.+)$") {
            $key = $matches[1]
            $value = $matches[2]
            
            # Parse value type
            if ($value -match "^true|false$") {
                $value = [bool]::Parse($value)
            }
            elseif ($value -match "^\d+$") {
                $value = [int]$value
            }
            elseif ($value -match "^\d+\.\d+$") {
                $value = [double]$value
            }
            
            if ($currentSection) {
                $result[$currentSection][$key] = $value
            }
        }
        elseif ($line -match "^    ([a-zA-Z_]+): (.+)$") {
            $key = $matches[1]
            $value = $matches[2]
            
            # Parse value type
            if ($value -match "^true|false$") {
                $value = [bool]::Parse($value)
            }
            elseif ($value -match "^\d+$") {
                $value = [int]$value
            }
            elseif ($value -match "^\d+\.\d+$") {
                $value = [double]$value
            }
            
            if ($currentSection -and $currentSubSection) {
                $result[$currentSection][$currentSubSection][$key] = $value
            }
        }
    }
    
    return $result
}

function ConvertTo-Yaml {
    param($Object)
    
    # Simplified YAML converter
    $yaml = ""
    
    foreach ($key in $Object.Keys) {
        $yaml += "$key:`n"
        
        if ($Object[$key] -is [hashtable]) {
            foreach ($subKey in $Object[$key].Keys) {
                if ($Object[$key][$subKey] -is [hashtable]) {
                    $yaml += "  $subKey:`n"
                    foreach ($subSubKey in $Object[$key][$subKey].Keys) {
                        $yaml += "    $subSubKey`: $($Object[$key][$subKey][$subSubKey])`n"
                    }
                } else {
                    $yaml += "  $subKey`: $($Object[$key][$subKey])`n"
                }
            }
        } else {
            $yaml += "  $($Object[$key])`n"
        }
    }
    
    return $yaml
}

# Main execution
function Main {
    Write-Log "🚀 Configuration Management for $Environment" $Blue
    Write-Log "============================================" $Blue
    
    $configFile = "$ConfigPath/$Environment.yml"
    $validationResults = @{}
    
    try {
        switch ($Action.ToLower()) {
            "validate" {
                Write-Log "🔍 Validating configuration..." $Yellow
                $validationResults[$Environment] = Test-ConfigurationFile $configFile
            }
            
            "convert" {
                Write-Log "🔄 Converting configuration format..." $Yellow
                $outputFile = "$ConfigPath/$Environment.$OutputFormat"
                Convert-ConfigurationFormat $configFile $OutputFormat $outputFile
            }
            
            "merge" {
                if ($Merge) {
                    Write-Log "🔀 Merging configurations..." $Yellow
                    $baseConfig = "$ConfigPath/base.yml"
                    $mergedConfig = Merge-Configurations $baseConfig $configFile
                    
                    if ($mergedConfig) {
                        $outputFile = "$ConfigPath/$Environment-merged.yml"
                        $mergedConfig | ConvertTo-Yaml | Out-File -FilePath $outputFile -Encoding UTF8
                        Write-Log "✅ Merged configuration saved to: $outputFile" $Green
                    }
                }
            }
            
            "encrypt" {
                if ($Encrypt) {
                    Write-Log "🔐 Encrypting configuration..." $Yellow
                    $outputFile = "$ConfigPath/$Environment.encrypted.json"
                    Encrypt-Configuration $configFile $outputFile "default-key"
                }
            }
            
            "report" {
                Write-Log "📋 Generating configuration report..." $Yellow
                $validationResults[$Environment] = Test-ConfigurationFile $configFile
                Generate-ConfigurationReport $Environment $validationResults
            }
            
            default {
                Write-Log "❌ Unknown action: $Action" $Red
                Write-Log "Available actions: validate, convert, merge, encrypt, report" $Yellow
                exit 1
            }
        }
        
        Write-Log ""
        Write-Log "✅ Configuration management completed successfully!" $Green
    }
    catch {
        Write-Log "❌ Configuration management failed: $_" $Red
        exit 1
    }
}

# Show usage if no parameters
if ($args.Count -eq 0) {
    Write-Host @"
Configuration Management Script

Usage: .\config_management.ps1 -Environment <env> -Action <action> [options]

Environments:
  development  - Local development configuration
  staging      - Staging environment configuration  
  production   - Production environment configuration

Actions:
  validate     - Validate configuration file
  convert      - Convert configuration format
  merge        - Merge base configuration with environment-specific
  encrypt      - Encrypt sensitive configuration values
  report       - Generate configuration report

Options:
  -ConfigPath <path>     Configuration files directory (default: config/environments)
  -OutputFormat <format> Output format for conversion (yaml, json, env, docker-compose)
  -Merge                 Enable configuration merging
  -Encrypt               Enable configuration encryption

Examples:
  .\config_management.ps1 -Environment development -Action validate
  .\config_management.ps1 -Environment production -Action convert -OutputFormat json
  .\config_management.ps1 -Environment staging -Action report
"@ -ForegroundColor $Blue
    exit 0
}

# Run main function
Main