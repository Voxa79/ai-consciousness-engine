# Script PowerShell pour redémarrer les services de la plateforme
# Auteur: Consciousness Engine Team
# Version: 1.0

param(
    [string[]]$Services = @("all"),
    [switch]$Force,
    [switch]$Verbose,
    [int]$Timeout = 30,
    [switch]$WaitForHealthy
)

Write-Host "🔄 Redémarrage des Services - Consciousness Engine" -ForegroundColor Cyan
Write-Host "=================================================" -ForegroundColor Cyan

# Configuration des services
$ServiceConfig = @{
    "consciousness-engine" = @{
        Name = "Consciousness Engine"
        Path = "consciousness-engine"
        Port = 8080
        HealthEndpoint = "/health"
        StartCommand = "cargo run --release"
        StopSignal = "SIGTERM"
    }
    "api-gateway" = @{
        Name = "API Gateway"
        Path = "api-gateway"
        Port = 3000
        HealthEndpoint = "/api/v1/health"
        StartCommand = "cargo run --release"
        StopSignal = "SIGTERM"
    }
    "agent-orchestrator" = @{
        Name = "Agent Orchestrator"
        Path = "agent-orchestrator"
        Port = 8081
        HealthEndpoint = "/health"
        StartCommand = "cargo run --release"
        StopSignal = "SIGTERM"
    }
    "web-ui" = @{
        Name = "Web UI"
        Path = "web-ui"
        Port = 3001
        HealthEndpoint = "/"
        StartCommand = "npm start"
        StopSignal = "SIGTERM"
    }
}

# Fonctions utilitaires
function Write-Log {
    param([string]$Message, [string]$Level = "INFO")
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $color = switch ($Level) {
        "ERROR" { "Red" }
        "WARN" { "Yellow" }
        "SUCCESS" { "Green" }
        default { "White" }
    }
    Write-Host "[$timestamp] [$Level] $Message" -ForegroundColor $color
}

function Test-ServiceHealth {
    param([string]$ServiceName, [hashtable]$Config)
    
    try {
        $url = "http://localhost:$($Config.Port)$($Config.HealthEndpoint)"
        $response = Invoke-WebRequest -Uri $url -TimeoutSec 5 -UseBasicParsing
        return $response.StatusCode -eq 200
    } catch {
        return $false
    }
}

function Stop-ServiceByPort {
    param([int]$Port, [string]$ServiceName)
    
    try {
        $processes = Get-NetTCPConnection -LocalPort $Port -ErrorAction SilentlyContinue | 
                    Select-Object -ExpandProperty OwningProcess | 
                    Get-Process -Id { $_ } -ErrorAction SilentlyContinue

        if ($processes) {
            foreach ($process in $processes) {
                Write-Log "Arrêt du processus $($process.Name) (PID: $($process.Id)) pour $ServiceName" "INFO"
                
                if ($Force) {
                    Stop-Process -Id $process.Id -Force
                } else {
                    $process.CloseMainWindow()
                    Start-Sleep -Seconds 2
                    if (!$process.HasExited) {
                        Stop-Process -Id $process.Id -Force
                    }
                }
            }
            return $true
        }
        return $false
    } catch {
        Write-Log "Erreur lors de l'arrêt du service $ServiceName : $_" "ERROR"
        return $false
    }
}

function Start-ServiceProcess {
    param([string]$ServiceName, [hashtable]$Config)
    
    try {
        $servicePath = Join-Path $PSScriptRoot $Config.Path
        if (-not (Test-Path $servicePath)) {
            Write-Log "Chemin du service non trouvé: $servicePath" "ERROR"
            return $false
        }

        Set-Location $servicePath
        Write-Log "Démarrage de $ServiceName dans $servicePath" "INFO"

        # Démarrer le processus en arrière-plan
        $processInfo = New-Object System.Diagnostics.ProcessStartInfo
        $processInfo.FileName = "powershell.exe"
        $processInfo.Arguments = "-Command `"$($Config.StartCommand)`""
        $processInfo.WorkingDirectory = $servicePath
        $processInfo.UseShellExecute = $false
        $processInfo.CreateNoWindow = $false

        $process = [System.Diagnostics.Process]::Start($processInfo)
        
        if ($process) {
            Write-Log "Service $ServiceName démarré (PID: $($process.Id))" "SUCCESS"
            return $true
        } else {
            Write-Log "Échec du démarrage de $ServiceName" "ERROR"
            return $false
        }
    } catch {
        Write-Log "Erreur lors du démarrage de $ServiceName : $_" "ERROR"
        return $false
    }
}

function Wait-ForServiceHealth {
    param([string]$ServiceName, [hashtable]$Config, [int]$TimeoutSeconds = 30)
    
    Write-Log "Attente de la santé de $ServiceName..." "INFO"
    $startTime = Get-Date
    
    do {
        Start-Sleep -Seconds 2
        if (Test-ServiceHealth -ServiceName $ServiceName -Config $Config) {
            $elapsed = (Get-Date) - $startTime
            Write-Log "$ServiceName est en bonne santé (${elapsed.TotalSeconds}s)" "SUCCESS"
            return $true
        }
        
        $elapsed = (Get-Date) - $startTime
        if ($elapsed.TotalSeconds -gt $TimeoutSeconds) {
            Write-Log "Timeout atteint pour $ServiceName" "WARN"
            return $false
        }
        
        Write-Host "." -NoNewline -ForegroundColor Yellow
    } while ($true)
}

function Restart-Service {
    param([string]$ServiceName, [hashtable]$Config)
    
    Write-Log "Redémarrage de $($Config.Name)..." "INFO"
    
    # 1. Arrêter le service
    Write-Log "Arrêt de $($Config.Name)..." "INFO"
    $stopped = Stop-ServiceByPort -Port $Config.Port -ServiceName $Config.Name
    
    if ($stopped) {
        Write-Log "$($Config.Name) arrêté avec succès" "SUCCESS"
    } else {
        Write-Log "Aucun processus trouvé pour $($Config.Name) sur le port $($Config.Port)" "WARN"
    }
    
    # 2. Attendre un peu
    Start-Sleep -Seconds 3
    
    # 3. Démarrer le service
    Write-Log "Démarrage de $($Config.Name)..." "INFO"
    $started = Start-ServiceProcess -ServiceName $ServiceName -Config $Config
    
    if (-not $started) {
        Write-Log "Échec du démarrage de $($Config.Name)" "ERROR"
        return $false
    }
    
    # 4. Attendre que le service soit en bonne santé si demandé
    if ($WaitForHealthy) {
        $healthy = Wait-ForServiceHealth -ServiceName $ServiceName -Config $Config -TimeoutSeconds $Timeout
        if (-not $healthy) {
            Write-Log "$($Config.Name) n'est pas en bonne santé après le redémarrage" "WARN"
        }
    }
    
    return $true
}

# Script principal
try {
    $servicesToRestart = if ($Services -contains "all") {
        $ServiceConfig.Keys
    } else {
        $Services | Where-Object { $ServiceConfig.ContainsKey($_) }
    }
    
    if ($servicesToRestart.Count -eq 0) {
        Write-Log "Aucun service valide spécifié" "ERROR"
        Write-Host "Services disponibles: $($ServiceConfig.Keys -join ', ')" -ForegroundColor Yellow
        exit 1
    }
    
    Write-Log "Services à redémarrer: $($servicesToRestart -join ', ')" "INFO"
    
    # Ordre de redémarrage (dépendances)
    $restartOrder = @()
    if ($servicesToRestart -contains "consciousness-engine") { $restartOrder += "consciousness-engine" }
    if ($servicesToRestart -contains "agent-orchestrator") { $restartOrder += "agent-orchestrator" }
    if ($servicesToRestart -contains "api-gateway") { $restartOrder += "api-gateway" }
    if ($servicesToRestart -contains "web-ui") { $restartOrder += "web-ui" }
    
    $successCount = 0
    $totalCount = $restartOrder.Count
    
    foreach ($serviceName in $restartOrder) {
        $config = $ServiceConfig[$serviceName]
        
        Write-Host "`n" + "="*50 -ForegroundColor Cyan
        Write-Log "Redémarrage de $($config.Name) ($($successCount + 1)/$totalCount)" "INFO"
        Write-Host "="*50 -ForegroundColor Cyan
        
        $success = Restart-Service -ServiceName $serviceName -Config $config
        
        if ($success) {
            $successCount++
            Write-Log "$($config.Name) redémarré avec succès" "SUCCESS"
        } else {
            Write-Log "Échec du redémarrage de $($config.Name)" "ERROR"
            
            if (-not $Force) {
                Write-Host "Continuer malgré l'erreur? (y/N): " -NoNewline -ForegroundColor Yellow
                $response = Read-Host
                if ($response -ne "y" -and $response -ne "Y") {
                    Write-Log "Redémarrage interrompu par l'utilisateur" "WARN"
                    break
                }
            }
        }
    }
    
    # Résumé
    Write-Host "`n" + "="*50 -ForegroundColor Cyan
    Write-Log "Résumé du redémarrage" "INFO"
    Write-Host "="*50 -ForegroundColor Cyan
    Write-Log "Services redémarrés avec succès: $successCount/$totalCount" $(if ($successCount -eq $totalCount) { "SUCCESS" } else { "WARN" })
    
    if ($successCount -eq $totalCount) {
        Write-Log "Tous les services ont été redémarrés avec succès!" "SUCCESS"
        exit 0
    } else {
        Write-Log "Certains services n'ont pas pu être redémarrés" "WARN"
        exit 1
    }
    
} catch {
    Write-Log "Erreur critique lors du redémarrage: $_" "ERROR"
    exit 2
} finally {
    # Retourner au répertoire original
    Set-Location $PSScriptRoot
}
