# Commandes standardisées pour le dev local

# Utilisation :
#  - just dev           # lance l'environnement de dev via docker-compose.dev.yml
#  - just dev-gw        # lance api-gateway (Rust) en local (nécessite cargo)
#  - just dev-ui        # lance le serveur Node.js (.kiro/mvp-server)
#  - just build-gw      # build docker de api-gateway
#  - just up            # docker compose up (dev)
#  - just down          # docker compose down (dev)
#  - just docs          # affiche l'URL de Swagger UI de l'api-gateway

set shell := ["powershell", "-NoLogo", "-NoProfile", "-Command"]

# Lancement complet via Docker (recommandé si cargo n'est pas installé)
dev:
    docker compose -f docker-compose.dev.yml up --build

# Monter uniquement les conteneurs (sans rebuild)
up:
    docker compose -f docker-compose.dev.yml up

down:
    docker compose -f docker-compose.dev.yml down

# API Gateway - build image
build-gw:
    docker build -t api-gateway:dev -f api-gateway/Dockerfile api-gateway

# API Gateway - run local (WSL2/Linux recommandé)
# Nécessite rustup/cargo et le toolchain 1.84 (rust-toolchain.toml pris en compte)
dev-gw:
    cd api-gateway; cargo run

# Node server (.kiro/mvp-server) - dev local avec npm/pnpm
# Nécessite Node installé en local (idéalement via nvm)
dev-ui:
    cd .kiro/mvp-server; npm install; npm run dev

# Affiche l'URL de Swagger UI
docs:
    echo "Swagger UI: http://localhost:3000/docs"
