@echo off
REM Script pour démarrer l'interface web en mode statique

echo 🧠 Consciousness Engine Web UI - Mode Statique
echo ===========================================

REM Vérifier si PowerShell est disponible
where powershell >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo ❌ PowerShell n'est pas disponible. Veuillez l'installer.
    exit /b 1
)

REM Exécuter le script PowerShell
echo 🚀 Démarrage du serveur statique...
powershell -ExecutionPolicy Bypass -File "%~dp0serve-static-ui.ps1"