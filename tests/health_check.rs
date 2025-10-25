//! tests/health_check.rs

use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server =
        zero2prod::startup::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_data() {
    //Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    //Act
    let response = client
        .post(format!("{}/subscriptions", &app_address))
        //.header("Content-Type", "application/x-www-form-urlencoded") //set by form()
        .form(&[("name", "le guin"), ("email", "ursula_le_guin@gmail.com")])
        .send()
        .await
        .expect("Failed to execute request");

    //Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_400_for_invalid() {
    //Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    //book had a vec macro here, doesn't change so array is fine
    let test_cases = [
        ([("name", "le guin")], "missing the email"),
        ([("email", "ursula_le_guin@gmail.com")], "missing the name"),
        ([("", "")], "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        //Act
        let response = client
            .post(format!("{}/subscriptions", &app_address))
            .form(&invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        //Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
