# Generates a dry-run report of cleanup candidates. No deletions performed.
param(
  [string]$RepoPath = (Resolve-Path '..').Path,
  [string]$ReportRelPath = 'scripts/reports/cleanup-candidates.txt'
)

$ErrorActionPreference = 'Stop'
$repo = (Resolve-Path $RepoPath).Path
$reportPath = Join-Path $repo $ReportRelPath
$reportDir = Split-Path $reportPath -Parent
New-Item -ItemType Directory -Force -Path $reportDir | Out-Null

function Add-Header($text){
  Add-Content -Encoding UTF8 -Path $reportPath -Value ("### " + $text)
}
function Add-Blank(){ Add-Content -Encoding UTF8 -Path $reportPath -Value '' }
function Add-List($items){
  if ($items -and $items.Count -gt 0){ $items | Sort-Object | Add-Content -Encoding UTF8 -Path $reportPath }
  else { Add-Content -Encoding UTF8 -Path $reportPath -Value '(none)' }
}

Set-Content -Encoding UTF8 -Path $reportPath -Value ("Cleanup candidates (dry-run) - " + (Get-Date -Format 'yyyy-MM-dd HH:mm:ss'))
Add-Content -Encoding UTF8 -Path $reportPath -Value ("Repo: " + $repo)
Add-Blank

$excludeRe = '(?i)(^|\\\\)(node_modules|\.git|\.cache|dist|build|out)(\\\\|$)'

# 1) Root-level Markdown except README.md
$rootMd = Get-ChildItem -Path $repo -File -Filter '*.md' | Where-Object { $_.Name -ne 'README.md' } | ForEach-Object { $_.FullName.Substring($repo.Length).TrimStart('\\') }
Add-Header 'Root-level Markdown except README.md'
Add-List $rootMd
Add-Blank

# 2) Duplicate/Extra README files across repo
$readmes = Get-ChildItem -Path $repo -Recurse -File -Include 'README*.md' |
  Where-Object { $_.FullName -notmatch $excludeRe -and $_.FullName -ne (Join-Path $repo 'README.md') } |
  ForEach-Object { $_.FullName.Substring($repo.Length).TrimStart('\\') }
Add-Header 'Duplicate/Extra README files across repo'
Add-List $readmes
Add-Blank

# 3) .env files outside web-ui/
$envs = Get-ChildItem -Path $repo -Recurse -File -Include '.env*','*.env' |
  Where-Object { $_.FullName -notmatch $excludeRe -and $_.FullName -notmatch '(?i)\\\\web-ui\\\\' } |
  ForEach-Object { $_.FullName.Substring($repo.Length).TrimStart('\\') }
Add-Header '.env files outside web-ui/'
Add-List $envs
Add-Blank

# 4) Root-level scripts to move into scripts/
$rootScripts = Get-ChildItem -Path $repo -File | Where-Object { $_.Extension -in '.ps1','.sh','.bat' } |
  ForEach-Object { $_.FullName.Substring($repo.Length).TrimStart('\\') }
Add-Header 'Root-level scripts to move into scripts/'
Add-List $rootScripts
Add-Blank

# 5) Scripts outside scripts/ directory
$scriptsOutside = Get-ChildItem -Path $repo -Recurse -File -Include '*.ps1','*.sh','*.bat' |
  Where-Object { $_.FullName -notmatch $excludeRe -and $_.FullName -notmatch '(?i)\\\\scripts\\\\' } |
  ForEach-Object { $_.FullName.Substring($repo.Length).TrimStart('\\') }
Add-Header 'Scripts outside scripts/ directory'
Add-List $scriptsOutside
Add-Blank

# 6) Binary/Archive files
$binaries = Get-ChildItem -Path $repo -Recurse -File -Include '*.exe','*.dll','*.zip','*.7z','*.tar','*.tgz','*.tar.gz','*.rar','*.iso','*.bin' |
  Where-Object { $_.FullName -notmatch $excludeRe } |
  ForEach-Object { $_.FullName.Substring($repo.Length).TrimStart('\\') }
Add-Header 'Binary/Archive files'
Add-List $binaries
Add-Blank

# 7) Build/Cache directories present
$dirs = Get-ChildItem -Path $repo -Recurse -Directory |
  Where-Object { $_.Name -in 'dist','build','out','.cache' -and $_.FullName -notmatch $excludeRe } |
  ForEach-Object { $_.FullName.Substring($repo.Length).TrimStart('\\') }
Add-Header 'Build/Cache directories present'
Add-List $dirs
Add-Blank

# 8) Other large files (> 5 MB)
$large = Get-ChildItem -Path $repo -Recurse -File |
  Where-Object { $_.Length -gt 5MB -and $_.FullName -notmatch $excludeRe } |
  ForEach-Object { ('{0}  ({1:N1} MB)' -f $_.FullName.Substring($repo.Length).TrimStart('\\'), ($_.Length/1MB)) }
Add-Header 'Other large files (> 5 MB)'
Add-List $large
Add-Blank

Write-Host ("Dry-run report written to: " + $reportPath)
