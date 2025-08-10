# Liste en lecture seule des dossiers candidats à suppression
$ErrorActionPreference = 'SilentlyContinue'
$root = Split-Path -Parent $MyInvocation.MyCommand.Path
$repo = Split-Path $root -Parent
Set-Location $repo

$Patterns = @(
  'node_modules','dist','build','out','.next','.nuxt','.svelte-kit',
  'coverage','.cache','.turbo','.parcel-cache','.vercel','.netlify'
)

Write-Host "--- DRY-RUN: dossiers candidats à suppression ---" -ForegroundColor Cyan

$all = Get-ChildItem -Recurse -Directory | Where-Object { $Patterns -contains $_.Name }
if (-not $all -or $all.Count -eq 0) {
  Write-Host "Aucun dossier candidat trouvé." -ForegroundColor Green
  return
}

$all | Select-Object FullName | Sort-Object FullName | Format-Table -AutoSize

# Petit récap par motif
Write-Host "\n--- Récapitulatif par motif ---" -ForegroundColor Yellow
foreach ($p in $Patterns) {
  $count = ($all | Where-Object { $_.Name -eq $p }).Count
  '{0,-15} {1,6}' -f $p, $count
}
