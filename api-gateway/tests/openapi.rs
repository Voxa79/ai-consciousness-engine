// Tests d'intégration OpenAPI
// Ce test vérifie la présence de schémas clés via l'endpoint HTTP /openapi.json.
// Marqué #[ignore] car il nécessite que l'API Gateway soit déjà démarrée (ex: docker-compose.dev.yml).

#[tokio::test]
#[ignore]
async fn openapi_json_contains_core_schemas() {
    let url = std::env::var("OPENAPI_URL").unwrap_or_else(|_| "http://localhost:3000/openapi.json".to_string());
    let body = reqwest::get(&url).await.expect("GET /openapi.json").text().await.expect("read body");

    for key in [
        "ConsciousnessRequest",
        "ErrorResponse",
        "ConsciousnessState",
        "AgentSummary",
    ] {
        assert!(body.contains(key), "schema manquant dans openapi.json: {}", key);
    }
}
