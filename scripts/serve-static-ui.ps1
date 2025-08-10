# Script pour servir l'interface web en mode statique sur le port 5000
# Ce script résout les problèmes de rechargement en servant les fichiers statiques
# sans aucune initialisation réseau au démarrage

# Couleurs pour l'affichage
$ESC = [char]27
$CYAN = "$ESC[36m"
$GREEN = "$ESC[32m"
$YELLOW = "$ESC[33m"
$RED = "$ESC[31m"
$NC = "$ESC[0m" # No Color

Write-Host "${CYAN}Consciousness Engine Web UI - Mode Statique${NC}"
Write-Host "${CYAN}===========================================${NC}"

# Vérifier si le dossier build existe
if (-not (Test-Path -Path "./web-ui/build")) {
    Write-Host "${RED}[ERREUR] Le dossier build n'existe pas. Veuillez d'abord construire l'application.${NC}"
    Write-Host "${YELLOW}[CONSEIL] Exécutez 'cd web-ui && npm run build' pour construire l'application.${NC}"
    exit 1
}

# Vérifier si Python est installé
$pythonInstalled = $false
try {
    $pythonVersion = python --version 2>&1
    if ($pythonVersion -match "Python") {
        $pythonInstalled = $true
        Write-Host "${GREEN}[OK] Python est installé: $pythonVersion${NC}"
    }
} catch {
    # Python n'est pas installé ou n'est pas dans le PATH
}

if (-not $pythonInstalled) {
    Write-Host "${RED}[ERREUR] Python n'est pas installé ou n'est pas dans le PATH.${NC}"
    Write-Host "${YELLOW}[CONSEIL] Veuillez installer Python depuis https://www.python.org/downloads/${NC}"
    exit 1
}

# Créer un script Python temporaire pour servir les fichiers statiques
$pythonScript = @"
# -*- coding: utf-8 -*-
import http.server
import socketserver
import os
import sys

PORT = 5000
DIRECTORY = os.path.join(os.getcwd(), 'web-ui', 'build')

class Handler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=DIRECTORY, **kwargs)
    
    def end_headers(self):
        # Enable CORS
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'X-Requested-With, Content-Type, Accept')
        super().end_headers()
    
    def do_GET(self):
        # For SPA routing, serve index.html for paths that don't exist
        if not os.path.exists(os.path.join(DIRECTORY, self.path.lstrip('/'))):
            if not self.path.startswith('/static/'):
                self.path = '/index.html'
        return super().do_GET()

try:
    with socketserver.TCPServer(("", PORT), Handler) as httpd:
        print(f"Serving at http://localhost:{PORT}")
        httpd.serve_forever()
except KeyboardInterrupt:
    print("\nServer stopped.")
    sys.exit(0)
except OSError as e:
    if e.errno == 10048:  # Port already in use
        print(f"Error: Port {PORT} is already in use. Please close the application using this port.")
    else:
        print(f"Error: {e}")
    sys.exit(1)
"@

$tempPythonFile = "$env:TEMP\static_server.py"
Set-Content -Path $tempPythonFile -Value $pythonScript -Encoding UTF8

# Vérifier si le port 5000 est disponible
$portInUse = $false
try {
    $listener = New-Object System.Net.Sockets.TcpListener([System.Net.IPAddress]::Any, 5000)
    $listener.Start()
    $listener.Stop()
    Write-Host "${GREEN}[OK] Le port 5000 est disponible${NC}"
} catch {
    $portInUse = $true
    Write-Host "${RED}[ERREUR] Le port 5000 est déjà utilisé. Veuillez libérer ce port ou modifier le script.${NC}"
    exit 1
}

# Démarrer le serveur Python
Write-Host "${YELLOW}[INFO] Démarrage du serveur HTTP sur le port 5000...${NC}"
$webUIPath = Join-Path (Get-Location) "web-ui/build"
Write-Host "${GREEN}[OK] Serveur démarré - Interface accessible sur ${CYAN}http://localhost:5000/#/login${NC}"
Write-Host "${YELLOW}[INFO] Servant les fichiers depuis: ${webUIPath}${NC}"
Write-Host "${YELLOW}[INFO] Appuyez sur Ctrl+C pour arrêter le serveur${NC}"

# Exécuter le script Python
python $tempPythonFile

# Nettoyer le fichier temporaire à la sortie
if (Test-Path $tempPythonFile) {
    Remove-Item $tempPythonFile
}