# Suppression physique des dossiers artefacts (node_modules, builds, caches)
# ATTENTION: action destructive (sur disque). Utiliser après dry-run.

$ErrorActionPreference = 'SilentlyContinue'
$root = Split-Path -Parent $MyInvocation.MyCommand.Path
$repo = Split-Path $root -Parent
Set-Location $repo

$Patterns = @(
  'node_modules','dist','build','out','.next','.nuxt','.svelte-kit',
  'coverage','.cache','.turbo','.parcel-cache','.vercel','.netlify'
)

function Convert-Size([long]$bytes){
  if($bytes -lt 1KB){ return "$bytes B" }
  elseif($bytes -lt 1MB){ return "{0:N2} KB" -f ($bytes/1KB) }
  elseif($bytes -lt 1GB){ return "{0:N2} MB" -f ($bytes/1MB) }
  else { return "{0:N2} GB" -f ($bytes/1GB) }
}

Write-Host "--- SUPPRESSION PHYSIQUE DES DOSSIERS CIBLES ---" -ForegroundColor Cyan

$targets = Get-ChildItem -Recurse -Directory | Where-Object { $Patterns -contains $_.Name }
if (-not $targets -or $targets.Count -eq 0) {
  Write-Host "Aucun dossier cible trouvé. Rien à supprimer." -ForegroundColor Green
  exit 0
}

$globalTotal = 0
$report = @()

foreach($d in $targets){
  try {
    $size = (Get-ChildItem -LiteralPath $d.FullName -Recurse -File | Measure-Object -Property Length -Sum).Sum
    if(-not $size){ $size = 0 }
    $globalTotal += $size
    $report += [PSCustomObject]@{ Path=$d.FullName; SizeBytes=$size; SizeHuman=(Convert-Size $size) }

    Remove-Item -Recurse -Force -LiteralPath $d.FullName
  } catch {
    Write-Warning "Échec suppression: $($d.FullName) -> $($_.Exception.Message)"
  }
}

$report | Sort-Object SizeBytes -Descending | Format-Table -AutoSize

Write-Host "\nTOTAL LIBÉRÉ (approx.): $(Convert-Size $globalTotal)" -ForegroundColor Yellow
