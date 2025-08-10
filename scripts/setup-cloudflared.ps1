# scripts/setup-cloudflared.ps1
# Prérequis:
# - Un domaine géré par Cloudflare (ex: chatbotaurus.com)
# - Connexion Cloudflare dans le navigateur quand 'cloudflared tunnel login' s'ouvre
# - Ports locaux: 3000 (HTTP API), 3001 (WS /ws), 3002 (Analytics / Grafana)

param(
  [string]$ApiHost = "api.chatbotaurus.com",
  [string]$AnalyticsHost = "analytique.chatbotaurus.com",
  [string]$TunnelName = "ce-stack"
)

function Ensure-Cloudflared {
  if (Get-Command cloudflared -ErrorAction SilentlyContinue) {
    Write-Host "cloudflared déjà installé." -ForegroundColor Green
    return
  }
  Write-Host "Installation de cloudflared via winget..." -ForegroundColor Yellow
  winget install --id Cloudflare.cloudflared -e --accept-package-agreements --accept-source-agreements
  if (-not (Get-Command cloudflared -ErrorAction SilentlyContinue)) {
    Write-Error "cloudflared introuvable après installation. Installe-le manuellement: https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/downloads/"
    exit 1
  }
}

function Cf-Login {
  Write-Host "Ouverture du login Cloudflare..." -ForegroundColor Yellow
  cloudflared tunnel login
}

function Create-Tunnel {
  Write-Host "Création du tunnel '$TunnelName'..." -ForegroundColor Yellow
  cloudflared tunnel create $TunnelName
}

function Write-Config {
  $configDir = Join-Path $env:USERPROFILE ".cloudflared"
  $configPath = Join-Path $configDir "config.yml"
  $credPath = Join-Path $configDir ("{0}.json" -f $TunnelName)

  $config = @"
tunnel: $TunnelName
credentials-file: $credPath

ingress:
  # WebSocket en priorité sur le chemin /ws
  - hostname: $ApiHost
    path: /ws
    service: http://localhost:3001

  # API HTTP par défaut
  - hostname: $ApiHost
    service: http://localhost:3000

  # Analytics (ex: Grafana)
  - hostname: $AnalyticsHost
    service: http://localhost:3002

  # Catch-all
  - service: http_status:404
"@

  if (-not (Test-Path $configDir)) { New-Item -ItemType Directory -Path $configDir | Out-Null }
  $config | Out-File -FilePath $configPath -Encoding utf8 -Force
  Write-Host "config.yml écrit dans: $configPath" -ForegroundColor Green
  return $configPath
}

function Route-DNS {
  Write-Host "Création des enregistrements DNS pour $ApiHost et $AnalyticsHost ..." -ForegroundColor Yellow
  cloudflared tunnel route dns $TunnelName $ApiHost
  cloudflared tunnel route dns $TunnelName $AnalyticsHost
}

function Run-Tunnel($configPath) {
  Write-Host "Démarrage du tunnel. Laisse cette fenêtre ouverte." -ForegroundColor Green
  cloudflared tunnel --config $configPath run $TunnelName
}

# Orchestration
Ensure-Cloudflared
Cf-Login
Create-Tunnel
$configPath = Write-Config
Route-DNS
Run-Tunnel -configPath $configPath
