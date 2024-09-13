#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();

    let client = reqwest::Client::new();
    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address"); // Launch the server as a background task
                                                                    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
}
