use reqwest::Client;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_full_consciousness_flow() {
    let client = Client::new();
    let base_url = "http://localhost:3000/api/v1";
    
    // Test 1: Health checks
    println!("🔍 Test des health checks...");
    
    let health_response = client
        .get(&format!("{}/consciousness/state", base_url))
        .send()
        .await
        .expect("Failed to get consciousness state");
    
    assert!(health_response.status().is_success());
    
    // Test 2: User registration
    println!("👤 Test d'inscription utilisateur...");
    
    let register_payload = json!({
        "email": "test@consciousness.ai",
        "password": "secure_password_123",
        "name": "Test User"
    });
    
    let register_response = client
        .post(&format!("{}/auth/register", base_url))
        .json(&register_payload)
        .send()
        .await
        .expect("Failed to register user");
    
    assert!(register_response.status().is_success());
    
    let auth_data: serde_json::Value = register_response
        .json()
        .await
        .expect("Failed to parse registration response");
    
    let token = auth_data["token"].as_str().expect("No token in response");
    
    // Test 3: Authenticated consciousness processing
    println!("🧠 Test de traitement consciousness authentifié...");
    
    let consciousness_payload = json!({
        "content": "Bonjour, pouvez-vous m'expliquer votre fonctionnement ?",
        "user_id": auth_data["user"]["id"],
        "context": {
            "test": true,
            "environment": "integration_test"
        }
    });
    
    let consciousness_response = client
        .post(&format!("{}/consciousness/process", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&consciousness_payload)
        .send()
        .await
        .expect("Failed to process consciousness request");
    
    assert!(consciousness_response.status().is_success());
    
    let consciousness_data: serde_json::Value = consciousness_response
        .json()
        .await
        .expect("Failed to parse consciousness response");
    
    // Vérifications de la réponse
    assert!(consciousness_data["content"].is_string());
    assert!(consciousness_data["confidence"].is_number());
    assert!(consciousness_data["consciousness_level"].is_number());
    assert!(consciousness_data["ethical_score"].is_number());
    
    // Test 4: Conversation history
    println!("📚 Test de l'historique des conversations...");
    
    sleep(Duration::from_secs(1)).await; // Attendre que la conversation soit sauvegardée
    
    let history_response = client
        .get(&format!("{}/conversations/{}", base_url, auth_data["user"]["id"]))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to get conversation history");
    
    assert!(history_response.status().is_success());
    
    let history_data: serde_json::Value = history_response
        .json()
        .await
        .expect("Failed to parse history response");
    
    assert!(history_data["conversations"].is_array());
    assert!(history_data["total"].as_u64().unwrap() > 0);
    
    // Test 5: User profile
    println!("👤 Test du profil utilisateur...");
    
    let profile_response = client
        .get(&format!("{}/auth/me", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to get user profile");
    
    assert!(profile_response.status().is_success());
    
    let profile_data: serde_json::Value = profile_response
        .json()
        .await
        .expect("Failed to parse profile response");
    
    assert_eq!(profile_data["email"], "test@consciousness.ai");
    assert_eq!(profile_data["name"], "Test User");
    
    println!("✅ Tous les tests d'intégration sont passés avec succès!");
}

#[tokio::test]
async fn test_consciousness_metrics_quality() {
    let client = Client::new();
    let base_url = "http://localhost:3000/api/v1";
    
    println!("📊 Test de la qualité des métriques consciousness...");
    
    let test_messages = vec![
        "Bonjour, comment allez-vous ?",
        "Expliquez-moi l'intelligence artificielle",
        "Que pensez-vous de l'éthique en IA ?",
        "Pouvez-vous être créatif ?",
        "Comment gérez-vous les émotions ?",
    ];
    
    for (i, message) in test_messages.iter().enumerate() {
        println!("Test message {}: {}", i + 1, message);
        
        let payload = json!({
            "content": message,
            "user_id": format!("test_user_{}", i),
            "context": {
                "test_iteration": i,
                "quality_test": true
            }
        });
        
        let response = client
            .post(&format!("{}/consciousness/process", base_url))
            .json(&payload)
            .send()
            .await
            .expect("Failed to process consciousness request");
        
        assert!(response.status().is_success());
        
        let data: serde_json::Value = response
            .json()
            .await
            .expect("Failed to parse response");
        
        // Vérifications de qualité
        let confidence = data["confidence"].as_f64().unwrap();
        let consciousness_level = data["consciousness_level"].as_f64().unwrap();
        let ethical_score = data["ethical_score"].as_f64().unwrap();
        let quality_score = data["quality_score"].as_f64().unwrap();
        
        assert!(confidence >= 0.0 && confidence <= 1.0, "Confidence hors limites: {}", confidence);
        assert!(consciousness_level >= 0.0 && consciousness_level <= 1.0, "Consciousness level hors limites: {}", consciousness_level);
        assert!(ethical_score >= 0.0 && ethical_score <= 1.0, "Ethical score hors limites: {}", ethical_score);
        assert!(quality_score >= 0.0 && quality_score <= 1.0, "Quality score hors limites: {}", quality_score);
        
        // Vérifier que la réponse n'est pas vide
        let content = data["content"].as_str().unwrap();
        assert!(!content.is_empty(), "Réponse vide pour: {}", message);
        assert!(content.len() > 10, "Réponse trop courte pour: {}", message);
        
        println!("✅ Message {} - Qualité: {:.2}, Confiance: {:.2}", i + 1, quality_score, confidence);
        
        // Pause entre les requêtes
        sleep(Duration::from_millis(500)).await;
    }
    
    println!("✅ Test de qualité des métriques terminé avec succès!");
}

#[tokio::test]
async fn test_performance_and_load() {
    let client = Client::new();
    let base_url = "http://localhost:3000/api/v1";
    
    println!("⚡ Test de performance et charge...");
    
    let concurrent_requests = 10;
    let mut handles = vec![];
    
    for i in 0..concurrent_requests {
        let client_clone = client.clone();
        let base_url_clone = base_url.to_string();
        
        let handle = tokio::spawn(async move {
            let payload = json!({
                "content": format!("Test de charge numéro {}", i),
                "user_id": format!("load_test_user_{}", i),
                "context": {
                    "load_test": true,
                    "request_id": i
                }
            });
            
            let start_time = std::time::Instant::now();
            
            let response = client_clone
                .post(&format!("{}/consciousness/process", base_url_clone))
                .json(&payload)
                .send()
                .await
                .expect("Failed to process consciousness request");
            
            let duration = start_time.elapsed();
            
            assert!(response.status().is_success());
            
            let data: serde_json::Value = response
                .json()
                .await
                .expect("Failed to parse response");
            
            (i, duration, data["processing_time_ms"].as_u64().unwrap())
        });
        
        handles.push(handle);
    }
    
    let mut total_duration = Duration::from_secs(0);
    let mut total_processing_time = 0u64;
    
    for handle in handles {
        let (request_id, duration, processing_time) = handle.await.unwrap();
        total_duration += duration;
        total_processing_time += processing_time;
        
        println!("Requête {} - Durée totale: {:?}, Temps traitement: {}ms", 
                request_id, duration, processing_time);
        
        // Vérifier que les temps de réponse sont raisonnables
        assert!(duration.as_secs() < 30, "Temps de réponse trop long: {:?}", duration);
        assert!(processing_time < 30000, "Temps de traitement trop long: {}ms", processing_time);
    }
    
    let avg_duration = total_duration / concurrent_requests as u32;
    let avg_processing_time = total_processing_time / concurrent_requests as u64;
    
    println!("📊 Résultats de performance:");
    println!("   Durée moyenne totale: {:?}", avg_duration);
    println!("   Temps de traitement moyen: {}ms", avg_processing_time);
    println!("   Requêtes concurrentes: {}", concurrent_requests);
    
    // Assertions de performance
    assert!(avg_duration.as_secs() < 15, "Durée moyenne trop élevée");
    assert!(avg_processing_time < 15000, "Temps de traitement moyen trop élevé");
    
    println!("✅ Test de performance terminé avec succès!");
}
