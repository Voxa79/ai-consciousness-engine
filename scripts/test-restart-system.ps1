# Script de test pour le système de redémarrage
# Auteur: Consciousness Engine Team
# Version: 1.0

param(
    [switch]$Verbose,
    [switch]$Interactive,
    [int]$Duration = 60
)

Write-Host "🧪 Test du Système de Redémarrage" -ForegroundColor Cyan
Write-Host "=================================" -ForegroundColor Cyan

$webUiPath = Join-Path $PSScriptRoot "web-ui"
$testResults = @{
    ComponentsCreated = $false
    TypesValid = $false
    HooksWorking = $false
    UIIntegrated = $false
    ScriptsExecutable = $false
    ErrorBoundaryActive = $false
    OverallScore = 0
}

# Fonction utilitaire pour les logs
function Write-TestLog {
    param([string]$Message, [string]$Level = "INFO")
    $timestamp = Get-Date -Format "HH:mm:ss"
    $color = switch ($Level) {
        "ERROR" { "Red" }
        "WARN" { "Yellow" }
        "SUCCESS" { "Green" }
        "TEST" { "Cyan" }
        default { "White" }
    }
    Write-Host "[$timestamp] [$Level] $Message" -ForegroundColor $color
}

# Test 1: Vérification des composants créés
Write-TestLog "Test 1: Vérification des composants créés" "TEST"

$requiredFiles = @(
    "web-ui/src/types/restart.ts",
    "web-ui/src/services/AppStateManager.ts",
    "web-ui/src/hooks/useAppRecovery.ts",
    "web-ui/src/components/Restart/RestartManager.tsx",
    "web-ui/src/components/Restart/HealthMonitor.tsx",
    "web-ui/src/components/Restart/ErrorBoundary.tsx",
    "restart-services.ps1"
)

$missingFiles = @()
foreach ($file in $requiredFiles) {
    if (-not (Test-Path $file)) {
        $missingFiles += $file
        Write-TestLog "❌ Fichier manquant: $file" "ERROR"
    } else {
        Write-TestLog "✅ Fichier trouvé: $file" "SUCCESS"
    }
}

$testResults.ComponentsCreated = $missingFiles.Count -eq 0
Write-TestLog "Résultat Test 1: $(if ($testResults.ComponentsCreated) { 'RÉUSSI' } else { 'ÉCHOUÉ' })" $(if ($testResults.ComponentsCreated) { "SUCCESS" } else { "ERROR" })

# Test 2: Validation des types TypeScript
Write-TestLog "`nTest 2: Validation des types TypeScript" "TEST"

if (Test-Path $webUiPath) {
    Set-Location $webUiPath
    
    try {
        $tscOutput = npx tsc --noEmit --skipLibCheck 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-TestLog "✅ Types TypeScript valides" "SUCCESS"
            $testResults.TypesValid = $true
        } else {
            Write-TestLog "❌ Erreurs de types TypeScript" "ERROR"
            if ($Verbose) {
                Write-TestLog $tscOutput "WARN"
            }
        }
    } catch {
        Write-TestLog "❌ Impossible de vérifier les types TypeScript" "ERROR"
    }
} else {
    Write-TestLog "❌ Dossier web-ui non trouvé" "ERROR"
}

Write-TestLog "Résultat Test 2: $(if ($testResults.TypesValid) { 'RÉUSSI' } else { 'ÉCHOUÉ' })" $(if ($testResults.TypesValid) { "SUCCESS" } else { "ERROR" })

# Test 3: Test de compilation
Write-TestLog "`nTest 3: Test de compilation" "TEST"

try {
    $buildOutput = npm run build 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-TestLog "✅ Compilation réussie" "SUCCESS"
        $testResults.UIIntegrated = $true
    } else {
        Write-TestLog "❌ Échec de compilation" "ERROR"
        if ($Verbose) {
            Write-TestLog $buildOutput "WARN"
        }
    }
} catch {
    Write-TestLog "❌ Erreur lors de la compilation" "ERROR"
}

Write-TestLog "Résultat Test 3: $(if ($testResults.UIIntegrated) { 'RÉUSSI' } else { 'ÉCHOUÉ' })" $(if ($testResults.UIIntegrated) { "SUCCESS" } else { "ERROR" })

# Test 4: Test des scripts PowerShell
Write-TestLog "`nTest 4: Test des scripts PowerShell" "TEST"

Set-Location $PSScriptRoot

$scripts = @(
    "restart-services.ps1",
    "start-ui-stable.ps1",
    "test-ui-stability.ps1"
)

$scriptsWorking = 0
foreach ($script in $scripts) {
    if (Test-Path $script) {
        try {
            # Test de syntaxe PowerShell
            $null = [System.Management.Automation.PSParser]::Tokenize((Get-Content $script -Raw), [ref]$null)
            Write-TestLog "✅ Script valide: $script" "SUCCESS"
            $scriptsWorking++
        } catch {
            Write-TestLog "❌ Erreur de syntaxe dans: $script" "ERROR"
        }
    } else {
        Write-TestLog "❌ Script manquant: $script" "ERROR"
    }
}

$testResults.ScriptsExecutable = $scriptsWorking -eq $scripts.Count
Write-TestLog "Résultat Test 4: $(if ($testResults.ScriptsExecutable) { 'RÉUSSI' } else { 'ÉCHOUÉ' })" $(if ($testResults.ScriptsExecutable) { "SUCCESS" } else { "ERROR" })

# Test 5: Test de démarrage rapide
Write-TestLog "`nTest 5: Test de démarrage rapide" "TEST"

if ($testResults.UIIntegrated) {
    Set-Location $webUiPath
    
    # Variables d'environnement pour test
    $env:FAST_REFRESH = "false"
    $env:WDS_HOT = "false"
    $env:WDS_LIVE_RELOAD = "false"
    $env:BROWSER = "none"
    $env:PORT = "3002"
    
    # Démarrer le serveur en arrière-plan
    $serverJob = Start-Job -ScriptBlock {
        param($webUiPath)
        Set-Location $webUiPath
        
        $env:FAST_REFRESH = "false"
        $env:WDS_HOT = "false"
        $env:WDS_LIVE_RELOAD = "false"
        $env:BROWSER = "none"
        $env:PORT = "3002"
        
        npm start 2>&1
    } -ArgumentList $webUiPath
    
    Write-TestLog "Attente du démarrage du serveur..." "INFO"
    Start-Sleep -Seconds 15
    
    # Tester la connectivité
    try {
        $response = Invoke-WebRequest -Uri "http://127.0.0.1:3002" -TimeoutSec 10 -UseBasicParsing
        if ($response.StatusCode -eq 200) {
            Write-TestLog "✅ Serveur accessible" "SUCCESS"
            $testResults.ErrorBoundaryActive = $true
        } else {
            Write-TestLog "❌ Serveur non accessible (Status: $($response.StatusCode))" "ERROR"
        }
    } catch {
        Write-TestLog "❌ Impossible de se connecter au serveur" "ERROR"
    }
    
    # Arrêter le serveur
    Stop-Job $serverJob -Force
    Remove-Job $serverJob -Force
} else {
    Write-TestLog "⏭️ Test ignoré (compilation échouée)" "WARN"
}

Write-TestLog "Résultat Test 5: $(if ($testResults.ErrorBoundaryActive) { 'RÉUSSI' } else { 'ÉCHOUÉ' })" $(if ($testResults.ErrorBoundaryActive) { "SUCCESS" } else { "ERROR" })

# Calcul du score global
$totalTests = 5
$passedTests = 0
if ($testResults.ComponentsCreated) { $passedTests++ }
if ($testResults.TypesValid) { $passedTests++ }
if ($testResults.UIIntegrated) { $passedTests++ }
if ($testResults.ScriptsExecutable) { $passedTests++ }
if ($testResults.ErrorBoundaryActive) { $passedTests++ }

$testResults.OverallScore = [math]::Round(($passedTests / $totalTests) * 100, 2)

# Résultats finaux
Write-Host "`n" + "="*50 -ForegroundColor Cyan
Write-TestLog "RÉSULTATS FINAUX" "TEST"
Write-Host "="*50 -ForegroundColor Cyan

Write-TestLog "Tests réussis: $passedTests/$totalTests" "INFO"
Write-TestLog "Score global: $($testResults.OverallScore)%" $(if ($testResults.OverallScore -gt 80) { "SUCCESS" } elseif ($testResults.OverallScore -gt 60) { "WARN" } else { "ERROR" })

Write-Host "`nDétail des tests:" -ForegroundColor White
Write-Host "- Composants créés: $(if ($testResults.ComponentsCreated) { '✅' } else { '❌' })" -ForegroundColor $(if ($testResults.ComponentsCreated) { "Green" } else { "Red" })
Write-Host "- Types valides: $(if ($testResults.TypesValid) { '✅' } else { '❌' })" -ForegroundColor $(if ($testResults.TypesValid) { "Green" } else { "Red" })
Write-Host "- UI intégrée: $(if ($testResults.UIIntegrated) { '✅' } else { '❌' })" -ForegroundColor $(if ($testResults.UIIntegrated) { "Green" } else { "Red" })
Write-Host "- Scripts exécutables: $(if ($testResults.ScriptsExecutable) { '✅' } else { '❌' })" -ForegroundColor $(if ($testResults.ScriptsExecutable) { "Green" } else { "Red" })
Write-Host "- Système actif: $(if ($testResults.ErrorBoundaryActive) { '✅' } else { '❌' })" -ForegroundColor $(if ($testResults.ErrorBoundaryActive) { "Green" } else { "Red" })

if ($testResults.OverallScore -gt 80) {
    Write-TestLog "`n🎉 Système de redémarrage OPÉRATIONNEL!" "SUCCESS"
    exit 0
} elseif ($testResults.OverallScore -gt 60) {
    Write-TestLog "`n⚠️ Système de redémarrage PARTIELLEMENT opérationnel" "WARN"
    exit 1
} else {
    Write-TestLog "`n❌ Système de redémarrage NON opérationnel" "ERROR"
    exit 2
}

# Retourner au répertoire original
Set-Location $PSScriptRoot
