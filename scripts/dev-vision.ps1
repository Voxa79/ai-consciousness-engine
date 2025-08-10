$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

function Write-JsonUtf8NoBom {
  param(
    [Parameter(Mandatory)][string]$Path,
    [Parameter(Mandatory)]$Object
  )
  $json = $Object | ConvertTo-Json -Compress
  $enc = New-Object System.Text.UTF8Encoding($false)
  [System.IO.File]::WriteAllText($Path, $json, $enc)
}

# Petit PNG 1x1 en data URL
$DataUrl = 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMB/axu3YQAAAAASUVORK5CYII='

# Payloads (snake_case et camelCase). Prompt ASCII pour écarter tout souci d'encodage.
$PayloadSnake = [ordered]@{ image_base64 = $DataUrl; model = 'llava:7b'; prompt = 'Describe this image.' }
$PayloadCamel = [ordered]@{ imageBase64 = $DataUrl; model = 'llava:7b'; prompt = 'Describe this image.' }

# Fichiers temporaires
$TmpSnake = Join-Path ([System.IO.Path]::GetTempPath()) ('vision_snake_' + [Guid]::NewGuid() + '.json')
$TmpCamel = Join-Path ([System.IO.Path]::GetTempPath()) ('vision_camel_' + [Guid]::NewGuid() + '.json')
Write-JsonUtf8NoBom -Path $TmpSnake -Object $PayloadSnake
Write-JsonUtf8NoBom -Path $TmpCamel -Object $PayloadCamel

$ok1 = $true
$ok2 = $true

Write-Host '=== Test mvp-server /vision/describe (snake_case) ==='
try {
  $r1 = Invoke-RestMethod -Uri 'http://localhost:4000/vision/describe' -Method Post -ContentType 'application/json' -InFile $TmpSnake -TimeoutSec 120
  $r1 | ConvertTo-Json -Depth 6 | Write-Host
} catch {
  $ok1 = $false
  $msg = $_.Exception.Message
  $code = $null
  $body = $null
  if ($_.Exception.Response) {
    try { $code = [int]$_.Exception.Response.StatusCode } catch { $code = $null }
    try {
      $reader = New-Object IO.StreamReader($_.Exception.Response.GetResponseStream())
      $body = $reader.ReadToEnd()
      $reader.Dispose()
    } catch { $body = $null }
  }
  Write-Warning ("mvp-server error: " + $msg)
  if ($code) { Write-Host ("[mvp-server] HTTP " + $code) }
  if ($body) { Write-Host '--- BODY (mvp-server) ---'; Write-Host $body }
}

Write-Host '=== Test api-gateway /api/v1/vision/describe (snake_case) ==='
try {
  $r2 = Invoke-RestMethod -Uri 'http://localhost:3000/api/v1/vision/describe' -Method Post -ContentType 'application/json' -InFile $TmpSnake -TimeoutSec 120
  $r2 | ConvertTo-Json -Depth 6 | Write-Host
} catch {
  $ok2 = $false
  $msg = $_.Exception.Message
  $code = $null
  $body = $null
  if ($_.Exception.Response) {
    try { $code = [int]$_.Exception.Response.StatusCode } catch { $code = $null }
    try {
      $reader = New-Object IO.StreamReader($_.Exception.Response.GetResponseStream())
      $body = $reader.ReadToEnd()
      $reader.Dispose()
    } catch { $body = $null }
  }
  Write-Warning ("gateway error: " + $msg)
  if ($code) { Write-Host ("[gateway]   HTTP " + $code) }
  if ($body) { Write-Host '--- BODY (gateway) ---'; Write-Host $body }
}

Write-Host '=== Test mvp-server /vision/describe (camelCase) ==='
try {
  $r1b = Invoke-RestMethod -Uri 'http://localhost:4000/vision/describe' -Method Post -ContentType 'application/json' -InFile $TmpCamel -TimeoutSec 120
  $r1b | ConvertTo-Json -Depth 6 | Write-Host
} catch {
  $ok1 = $ok1 -and $false
  $msg = $_.Exception.Message
  $code = $null
  $body = $null
  if ($_.Exception.Response) {
    try { $code = [int]$_.Exception.Response.StatusCode } catch { $code = $null }
    try {
      $reader = New-Object IO.StreamReader($_.Exception.Response.GetResponseStream())
      $body = $reader.ReadToEnd()
      $reader.Dispose()
    } catch { $body = $null }
  }
  Write-Warning ("mvp-server (camelCase) error: " + $msg)
  if ($code) { Write-Host ("[mvp-server] HTTP " + $code) }
  if ($body) { Write-Host '--- BODY (mvp-server camel) ---'; Write-Host $body }
}

Write-Host '=== Test api-gateway /api/v1/vision/describe (camelCase) ==='
try {
  $r2b = Invoke-RestMethod -Uri 'http://localhost:3000/api/v1/vision/describe' -Method Post -ContentType 'application/json' -InFile $TmpCamel -TimeoutSec 120
  $r2b | ConvertTo-Json -Depth 6 | Write-Host
} catch {
  $ok2 = $ok2 -and $false
  $msg = $_.Exception.Message
  $code = $null
  $body = $null
  if ($_.Exception.Response) {
    try { $code = [int]$_.Exception.Response.StatusCode } catch { $code = $null }
    try {
      $reader = New-Object IO.StreamReader($_.Exception.Response.GetResponseStream())
      $body = $reader.ReadToEnd()
      $reader.Dispose()
    } catch { $body = $null }
  }
  Write-Warning ("gateway (camelCase) error: " + $msg)
  if ($code) { Write-Host ("[gateway]   HTTP " + $code) }
  if ($body) { Write-Host '--- BODY (gateway camel) ---'; Write-Host $body }
}

Remove-Item $TmpSnake,$TmpCamel -Force -ErrorAction SilentlyContinue

if (-not $ok1) {
  Write-Host '--- LOGS mvp-server (10m) ---'
  try {
    docker compose -f 'docker-compose.dev.yml' logs --no-color --since=10m mvp-server
  } catch {
    Write-Warning 'Impossible de récupérer les logs mvp-server.'
  }
}
if (-not $ok2) {
  Write-Host '--- LOGS api-gateway (10m) ---'
  try {
    docker compose -f 'docker-compose.dev.yml' logs --no-color --since=10m api-gateway
  } catch {
    Write-Warning 'Impossible de récupérer les logs api-gateway.'
  }
}

if (-not ($ok1 -and $ok2)) { exit 1 } else { exit 0 }
