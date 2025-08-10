param(
  [string]$ComposeFile = "docker-compose.dev.yml",
  [string]$Model = "tinyllama",
  [int]$TimeoutSeconds = 300
)

$ErrorActionPreference = "Stop"

function Wait-HttpOk($Url, $TimeoutSec) {
  $deadline = (Get-Date).AddSeconds($TimeoutSec)
  while ((Get-Date) -lt $deadline) {
    try {
      $resp = Invoke-WebRequest -UseBasicParsing -Uri $Url -Method GET -TimeoutSec 10
      if ($resp.StatusCode -ge 200 -and $resp.StatusCode -lt 300) { return }
    } catch { }
    Start-Sleep -Seconds 5
  }
  throw "Timeout waiting for $Url"
}

Write-Host "[1/6] Démarrage du stack Docker..." -ForegroundColor Cyan
& docker compose -f $ComposeFile up -d --build

Write-Host "[2/6] Attente Ollama (11434)..." -ForegroundColor Cyan
Wait-HttpOk "http://localhost:11434/api/tags" 180

Write-Host "[3/6] Téléchargement du modèle $Model..." -ForegroundColor Cyan
& docker compose -f $ComposeFile exec -T ollama ollama pull $Model

Write-Host "[4/6] Attente API Gateway /health..." -ForegroundColor Cyan
Wait-HttpOk "http://localhost:3000/health" 180

Write-Host "[5/6] Test /api/v1/llm/models..." -ForegroundColor Cyan
$modelsResp = Invoke-WebRequest -UseBasicParsing -Uri "http://localhost:3000/api/v1/llm/models" -Method GET -TimeoutSec 30
if ($modelsResp.StatusCode -lt 200 -or $modelsResp.StatusCode -ge 300) { throw "/api/v1/llm/models status $($modelsResp.StatusCode)" }
$modelsJson = $modelsResp.Content | ConvertFrom-Json
if (-not $modelsJson.models -or $modelsJson.models.Count -eq 0) { throw "Liste des modèles vide" }
$firstModels = ($modelsJson.models | Select-Object -First 5)
$firstModelsStr = ($firstModels -join ', ')
Write-Host "Modèles disponibles: $firstModelsStr" -ForegroundColor DarkGray
if ($modelsJson.models -notcontains $Model) { Write-Host "[!] Modèle demandé '$Model' non listé, on continue (peut être en cours de pull)" -ForegroundColor Yellow }

Write-Host "[6/9] Test LLM generate..." -ForegroundColor Cyan
$generateBody = @{ prompt = "Bonjour, peux-tu te présenter en une phrase ?" } | ConvertTo-Json -Compress
$generateFile = New-TemporaryFile
[System.IO.File]::WriteAllText($generateFile, $generateBody, (New-Object System.Text.UTF8Encoding($false)))
$resp = curl.exe -sS -o resp.json -w "%{http_code}" -X POST "http://localhost:3000/api/v1/llm/generate" -H "Content-Type: application/json" --data-binary "@$generateFile"
Write-Host "Status: $resp"
if ($resp -ne "200") { Get-Content resp.json | Write-Host; throw "generate a échoué" }
if ((Get-Item resp.json).Length -le 2) { throw "Réponse generate vide" }

Write-Host "[7/9] Test LLM stream (SSE)..." -ForegroundColor Cyan
$streamBody = @{ prompt = "Dis bonjour en 3 mots" } | ConvertTo-Json -Compress
$streamFile = New-TemporaryFile
[System.IO.File]::WriteAllText($streamFile, $streamBody, (New-Object System.Text.UTF8Encoding($false)))
$cmd = "curl.exe -N -sS -X POST http://localhost:3000/api/v1/llm/stream -H 'Content-Type: application/json' --data-binary '@$streamFile'"
$job = Start-Process -FilePath powershell -ArgumentList "-NoProfile","-Command","$cmd" -RedirectStandardOutput sse.log -PassThru
Start-Sleep -Seconds 8
try { $job | Stop-Process -Force } catch { }
if (-not (Select-String -Path sse.log -Pattern '^(data:|event:)' -Encoding UTF8)) {
  Write-Host "Flux SSE capturé:" -ForegroundColor Yellow
  Get-Content sse.log | Select-Object -First 50 | Write-Host
  throw "stream n'a pas produit d'événements"
}

Write-Host "[8/9] Test /evaluate (quality only)..." -ForegroundColor Cyan
$evalQualityBody = @{ quality = @{ agentType = "assistant"; changeId = "demo-quality"; scores = @{ selfAwareness = 0.8; ethical = 0.9; meta_cognitive_depth = 0.75; empathy = 0.8 }; latencies = @{ assessment_ms = 120; meta_cognitive_ms = 60; response_ms = 220 } } } | ConvertTo-Json -Compress
$evalQualityFile = New-TemporaryFile
[System.IO.File]::WriteAllText($evalQualityFile, $evalQualityBody, (New-Object System.Text.UTF8Encoding($false)))
$resp = curl.exe -sS -o eval_quality.json -w "%{http_code}" -X POST "http://localhost:4000/evaluate" -H "Content-Type: application/json" --data-binary "@${evalQualityFile}"
Write-Host "Status: $resp"
if ($resp -ne "200") { Get-Content eval_quality.json | Write-Host; throw "/evaluate quality a échoué" }
if (-not (Select-String -Path eval_quality.json -Pattern '"quality"\s*:' -Encoding UTF8)) { throw "/evaluate quality: champ 'quality' manquant" }

Write-Host "[9/9] Test /evaluate (LLM generate + quality)..." -ForegroundColor Cyan
$evalGenBody = @{ llm = @{ prompt = "Explain RAG in one paragraph"; model = $Model }; quality = @{ scores = @{ selfAwareness = 0.75; ethical = 0.9; metaCognitiveDepth = 0.7; empathyAuthenticity = 0.8 } } } | ConvertTo-Json -Compress
$evalGenFile = New-TemporaryFile
[System.IO.File]::WriteAllText($evalGenFile, $evalGenBody, (New-Object System.Text.UTF8Encoding($false)))
$resp = curl.exe -sS -o eval_generate.json -w "%{http_code}" -X POST "http://localhost:4000/evaluate" -H "Content-Type: application/json" --data-binary "@${evalGenFile}"
Write-Host "Status: $resp"
if ($resp -ne "200") { Get-Content eval_generate.json | Write-Host; throw "/evaluate generate a échoué" }
if (-not (Select-String -Path eval_generate.json -Pattern '"llm"\s*:' -Encoding UTF8)) { throw "/evaluate generate: champ 'llm' manquant" }

Write-Host "[+] Test /evaluate (LLM chat)..." -ForegroundColor Cyan
$evalChatBody = @{ llm = @{ model = $Model; messages = @(@{ role = "system"; content = "You are a helpful assistant." }, @{ role = "user"; content = "Give me a short tip about Rust async." }) } } | ConvertTo-Json -Compress
$evalChatFile = New-TemporaryFile
[System.IO.File]::WriteAllText($evalChatFile, $evalChatBody, (New-Object System.Text.UTF8Encoding($false)))
$resp = curl.exe -sS -o eval_chat.json -w "%{http_code}" -X POST "http://localhost:4000/evaluate" -H "Content-Type: application/json" --data-binary "@${evalChatFile}"
Write-Host "Status: $resp"
if ($resp -ne "200") { Get-Content eval_chat.json | Write-Host; throw "/evaluate chat a échoué" }
if (-not (Select-String -Path eval_chat.json -Pattern '"llm"\s*:' -Encoding UTF8)) { throw "/evaluate chat: champ 'llm' manquant" }

Write-Host "[+] Test /api/v1/evaluate via Gateway (quality only)..." -ForegroundColor Cyan
$gwEvalBody = @{ quality = @{ agentType = "assistant"; changeId = "gw-quality"; scores = @{ selfAwareness = 0.8; ethical = 0.9; meta_cognitive_depth = 0.75; empathy = 0.8 } } } | ConvertTo-Json -Compress
$gwEvalFile = New-TemporaryFile
[System.IO.File]::WriteAllText($gwEvalFile, $gwEvalBody, (New-Object System.Text.UTF8Encoding($false)))
$resp = curl.exe -sS -o gw_eval_quality.json -w "%{http_code}" -X POST "http://localhost:3000/api/v1/evaluate" -H "Content-Type: application/json" --data-binary "@${gwEvalFile}"
Write-Host "Status: $resp"
if ($resp -ne "200") { Get-Content gw_eval_quality.json | Write-Host; throw "/api/v1/evaluate (gateway) quality a échoué" }
if (-not (Select-String -Path gw_eval_quality.json -Pattern '"quality"\s*:' -Encoding UTF8)) { throw "/api/v1/evaluate (gateway): champ 'quality' manquant" }

Write-Host "[+] Test Vision describe..." -ForegroundColor Cyan
$visionBody = @{ image_url = "https://picsum.photos/300"; prompt = "Décris brièvement l'image." } | ConvertTo-Json -Compress
$visionFile = New-TemporaryFile
[System.IO.File]::WriteAllText($visionFile, $visionBody, (New-Object System.Text.UTF8Encoding($false)))
$resp = curl.exe -sS -o vision.json -w "%{http_code}" -X POST "http://localhost:3000/api/v1/vision/describe" -H "Content-Type: application/json" --data-binary "@${visionFile}"
Write-Host "Status: $resp"
if ($resp -ne "200") { Get-Content vision.json | Write-Host; throw "/vision/describe a échoué" }
if (-not (Select-String -Path vision.json -Pattern '"text"\s*:' -Encoding UTF8)) { throw "/vision/describe: champ 'text' manquant" }

Write-Host "[+] Test ASR transcribe..." -ForegroundColor Cyan
$asrBody = @{ audio_url = "http://localhost:8000/sample.wav"; language = "fr" } | ConvertTo-Json -Compress
$asrFile = New-TemporaryFile
[System.IO.File]::WriteAllText($asrFile, $asrBody, (New-Object System.Text.UTF8Encoding($false)))
$resp = curl.exe -sS -o asr.json -w "%{http_code}" -X POST "http://localhost:3000/api/v1/asr/transcribe" -H "Content-Type: application/json" --data-binary "@${asrFile}"
Write-Host "Status: $resp"
if ($resp -ne "200") { Get-Content asr.json | Write-Host; throw "/asr/transcribe a échoué" }
if (-not (Select-String -Path asr.json -Pattern '"text"\s*:' -Encoding UTF8)) { throw "/asr/transcribe: champ 'text' manquant" }

Write-Host "[+] Test TTS synthesize..." -ForegroundColor Cyan
$ttsBody = @{ text = "Bonjour"; voice = "fr-FR"; format = "wav" } | ConvertTo-Json -Compress
$ttsFile = New-TemporaryFile
[System.IO.File]::WriteAllText($ttsFile, $ttsBody, (New-Object System.Text.UTF8Encoding($false)))
$resp = curl.exe -sS -o tts.json -w "%{http_code}" -X POST "http://localhost:3000/api/v1/tts/synthesize" -H "Content-Type: application/json" --data-binary "@${ttsFile}"
Write-Host "Status: $resp"
if ($resp -ne "200") { Get-Content tts.json | Write-Host; throw "/tts/synthesize a échoué" }
if (-not (Select-String -Path tts.json -Pattern '"audio_base64"\s*:' -Encoding UTF8)) { throw "/tts/synthesize: champ 'audio_base64' manquant" }

Write-Host "Tous les tests smoke sont PASS" -ForegroundColor Green
