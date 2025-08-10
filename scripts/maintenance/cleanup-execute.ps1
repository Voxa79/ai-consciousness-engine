# Executes the approved safe cleanup. USE WITH CAUTION.
param(
  [string]$RepoPath = (Resolve-Path '..').Path
)

$ErrorActionPreference = 'Stop'
$repo = (Resolve-Path $RepoPath).Path
Set-Location $repo

function Ensure-Dir($p){ if(-not (Test-Path $p)){ New-Item -ItemType Directory -Force -Path $p | Out-Null } }

# 1) Docs: move root-level Markdown except README.md into docs/
$docsDir = Join-Path $repo 'docs'
Ensure-Dir $docsDir
Get-ChildItem -Path $repo -File -Filter '*.md' | Where-Object { $_.Name -ne 'README.md' } | ForEach-Object {
  $dest = Join-Path $docsDir $_.Name
  Move-Item -Force -LiteralPath $_.FullName -Destination $dest
}

# 2) .env outside web-ui => delete
Get-ChildItem -Path $repo -Recurse -File -Include '.env*','*.env' |
  Where-Object { $_.FullName -notmatch '(?i)\\web-ui\\' -and $_.DirectoryName -notmatch '(?i)\\(node_modules|\\.git)\\' } |
  ForEach-Object { Remove-Item -Force -LiteralPath $_.FullName }

# 3) Move scripts
# 3a) Root-level scripts -> scripts/
$scriptsDir = Join-Path $repo 'scripts'
Ensure-Dir $scriptsDir
Get-ChildItem -Path $repo -File | Where-Object { $_.Extension -in '.ps1','.sh','.bat' } | ForEach-Object {
  $dest = Join-Path $scriptsDir $_.Name
  Move-Item -Force -LiteralPath $_.FullName -Destination $dest
}
# 3b) Scripts outside scripts/ -> scripts/misc/
$miscDir = Join-Path $scriptsDir 'misc'
Ensure-Dir $miscDir
Get-ChildItem -Path $repo -Recurse -File -Include '*.ps1','*.sh','*.bat' |
  Where-Object { $_.FullName -notmatch '(?i)\\scripts\\' -and $_.FullName -notmatch '(?i)\\(node_modules|\\.git)\\' } |
  ForEach-Object {
    $dest = Join-Path $miscDir $_.Name
    if(Test-Path $dest){ $dest = Join-Path $miscDir ("{0}-{1}{2}" -f $_.BaseName, (Get-Random), $_.Extension) }
    Move-Item -Force -LiteralPath $_.FullName -Destination $dest
  }

# 4) Delete binaries/archives tracked in repo (outside excluded dirs)
$excludeRe = '(?i)(^|\\\\)(node_modules|\\.git)(\\\\|$)'
Get-ChildItem -Path $repo -Recurse -File -Include '*.exe','*.dll','*.zip','*.7z','*.tar','*.tgz','*.tar.gz','*.rar','*.iso','*.bin' |
  Where-Object { $_.FullName -notmatch $excludeRe } |
  ForEach-Object { Remove-Item -Force -LiteralPath $_.FullName }

# 5) Ensure web-ui/build is ignored
$gitignore = Join-Path $repo '.gitignore'
if(Test-Path $gitignore){
  $gi = Get-Content -LiteralPath $gitignore -Encoding UTF8
  if(-not ($gi -match '^web-ui/build/?$' -or $gi -match '^/?.*web-ui/build/?$')){
    Add-Content -Encoding UTF8 -LiteralPath $gitignore -Value "`n# Ignore CRA build output`nweb-ui/build/"
  }
} else {
  Set-Content -Encoding UTF8 -LiteralPath $gitignore -Value "# Ignore CRA build output`nweb-ui/build/"
}

Write-Host 'Cleanup safe actions executed.'
