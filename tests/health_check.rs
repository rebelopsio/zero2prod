//! tests/health_check.rs

#[actix_web::test]
async fn health_check_warks() {
    spawn_app().await.expect("Failed to spawn our app.");
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

async fn spawn_app() -> Result<(), std::io::Error> {
    todo!()
}
