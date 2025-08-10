# API Gateway (Rust, Axum, Utoipa)

Passerelle unifiée pour les services Consciousness/Agents/Governance.
Documentation OpenAPI générée automatiquement avec `utoipa` et exposée via Swagger UI.

## Prérequis
- Docker Desktop (WSL2 recommandé sous Windows)
- docker-compose
- (Optionnel) Toolchain Rust pour exécuter `cargo test`

## Démarrage (développement)
Lance la stack dev (api-gateway + mvp-server) avec Swagger UI et OpenAPI :

```bash
docker compose -f ../docker-compose.dev.yml up -d --build
```

Endpoints principaux (par défaut port 3000) :
- Health: http://localhost:3000/health
- OpenAPI JSON: http://localhost:3000/openapi.json
- Swagger UI: http://localhost:3000/docs

## Variables d’environnement (extraits)
- `PORT` (ex: 3000)
- `CONSCIOUSNESS_ENGINE_URL`
- `AGENT_ORCHESTRATOR_URL`
- `AI_GOVERNANCE_URL`
- `JWT_SECRET` (exigé si `AUTH_OPTIONAL=false`)
- `AUTH_OPTIONAL` ("true"/"false", défaut: true en dev)
- `RATE_LIMIT_PER_MINUTE` (ex: 60)
- `REQUEST_TIMEOUT_SECONDS` (ex: 30)

Ces variables peuvent être définies via `docker-compose.dev.yml`.

## Modèles / DTO OpenAPI
Tous les types exposés dans la doc OpenAPI sont centralisés dans `src/models.rs` :
- `HealthResponse`, `MetricsResponse`
- `ConsciousnessRequest`, `ConsciousnessResult`, `ConsciousnessState`
- `AgentSummary`, `AgentDetail`, `AgentListResponse`, `AgentExecutionResult`
- `PolicySummary`, `PoliciesResponse`, `AuditLogsResponse`, `ComplianceStatus`
- `ErrorResponse`

Les handlers `axum` référencent ces modèles dans leurs annotations `#[utoipa::path]`.

## Tests
Test d’intégration OpenAPI (nécessite l’API lancée) :

```bash
# Par défaut cible http://localhost:3000/openapi.json
cargo test --test openapi -- --ignored

# Spécifier une URL différente
OPENAPI_URL=http://localhost:3000/openapi.json cargo test --test openapi -- --ignored
```

## Dépendances (pinnées)
- `axum = "0.7"`
- `utoipa = "4.2.3"`
- `utoipa-swagger-ui = { version = "7.1.0", features = ["axum"] }`
- `tower-http = { version = "0.5", features = ["cors", "trace", "compression-full", "timeout"] }`

## Notes de build Docker
- L’image utilise Rust 1.84+.
- Un mécanisme de cache-bust (`ARG CACHE_BUST`) est en place pour garantir l’inclusion des derniers changements.

## Roadmap
- Crate `shared` pour types communs (steering, client Ollama, helpers)
- Sécurité: schéma Bearer JWT déclaré via `Modify` (security schemes) et enforcement configurable
- Observabilité étendue (traces/metrics), CI/CD, QA
