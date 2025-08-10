$ErrorActionPreference = 'Stop'

function Invoke-JsonPost {
  param(
    [Parameter(Mandatory=$true)][string]$Url,
    [Parameter(Mandatory=$true)][hashtable]$Object
  )
  $json = $Object | ConvertTo-Json -Compress
  try {
    $resp = Invoke-WebRequest -Uri $Url -Method Post -ContentType 'application/json' -Body $json -TimeoutSec 60
    Write-Host ("HTTP " + [int]$resp.StatusCode)
    if ($resp.Content) { $resp.Content | Write-Host }
  } catch {
    $msg = $_.Exception.Message
    $code = $null
    $body = $null
    if ($_.Exception.Response) {
      try { $code = [int]$_.Exception.Response.StatusCode } catch {}
      try {
        $reader = New-Object IO.StreamReader($_.Exception.Response.GetResponseStream())
        $body = $reader.ReadToEnd()
        $reader.Dispose()
      } catch {}
    }
    Write-Warning ("Request error: " + $msg)
    if ($code) { Write-Host ("HTTP " + $code) }
    if ($body) { Write-Host '--- BODY ---'; Write-Host $body }
  }
}

# Données de test (mini PNG 1x1 en data URL)
$DataUrl = 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMB/axu3YQAAAAASUVORK5CYII='

Write-Host '--- _echo_body (snake_case) ---'
Invoke-JsonPost -Url 'http://localhost:4000/_echo_body' -Object ([ordered]@{
  image_base64 = $DataUrl
  model        = 'llava:7b'
  prompt       = 'Describe this image.'
})

Write-Host '--- _echo_body (camelCase) ---'
Invoke-JsonPost -Url 'http://localhost:4000/_echo_body' -Object ([ordered]@{
  imageBase64 = $DataUrl
  model       = 'llava:7b'
  prompt      = 'Describe this image.'
})
