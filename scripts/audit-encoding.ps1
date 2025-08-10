# Audit des encodages texte (lecture seule)
# Exclut les dossiers de build/dépendances
$ErrorActionPreference = 'SilentlyContinue'

$root = Split-Path -Parent $MyInvocation.MyCommand.Path
$repo = Split-Path $root -Parent
Set-Location $repo

$TextExtensions = @(
  'txt','md','mdx','json','jsonc','js','mjs','cjs','ts','tsx','jsx',
  'html','htm','css','scss','sass','less','yaml','yml','toml',
  'py','rs','go','java','cs','ps1','psm1','psd1','sh','bash','bat','cmd',
  'env','ini','cfg','conf','xml','svg'
)

$ExcludeRegex = [regex]'\\(node_modules|dist|build|\.next|\.nuxt|\.svelte-kit|\.cache|\.turbo|\.parcel-cache|\.vercel|\.netlify|\.git)(\\|$)'

function Test-IsUtf8 {
  param([byte[]]$Bytes)
  try {
    $str = [System.Text.Encoding]::UTF8.GetString($Bytes)
    if ($str.IndexOf([char]0xFFFD) -ge 0) { return $false }
    $re = [System.Text.Encoding]::UTF8.GetBytes($str)
    return ($re.Length -eq $Bytes.Length)
  } catch { return $false }
}

$results = @()
Get-ChildItem -Recurse -File | Where-Object {
  -not $ExcludeRegex.IsMatch($_.FullName) -and 
  ($TextExtensions -contains $_.Extension.TrimStart('.').ToLower())
} | ForEach-Object {
  try {
    $b = [System.IO.File]::ReadAllBytes($_.FullName)
    $isUtf16LE = ($b.Length -ge 2 -and $b[0] -eq 0xFF -and $b[1] -eq 0xFE)
    $isUtf16BE = ($b.Length -ge 2 -and $b[0] -eq 0xFE -and $b[1] -eq 0xFF)
    $isBomUtf8 = ($b.Length -ge 3 -and $b[0] -eq 0xEF -and $b[1] -eq 0xBB -and $b[2] -eq 0xBF)
    $isUtf8 = Test-IsUtf8 -Bytes $b
    if (-not $isUtf8 -or $isUtf16LE -or $isUtf16BE) {
      $results += [PSCustomObject]@{
        File = $_.FullName
        Size = $_.Length
        UTF8 = $isUtf8
        BOM_UTF8 = $isBomUtf8
        UTF16LE = $isUtf16LE
        UTF16BE = $isUtf16BE
      }
    }
  } catch {}
}

$results | Sort-Object Size -Descending | Select-Object -First 50 | Format-Table -AutoSize

if (-not $results -or $results.Count -eq 0) {
  Write-Host "\nAucun fichier suspect trouvé (tout semble être en UTF-8)." -ForegroundColor Green
} else {
  Write-Host "\nTotal fichiers suspects: $($results.Count)" -ForegroundColor Yellow
}
