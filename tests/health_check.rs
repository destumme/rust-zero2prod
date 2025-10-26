//! tests/health_check.rs

use sqlx::{Connection, PgConnection, PgPool, query};
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

impl TestApp {
    async fn cleanup(&self) {
        query!("DELETE FROM subscriptions",)
            .execute(&self.db_pool)
            .await
            .expect("Failed to cleanup");
    }
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("failed to bind to random port");
    let port = listener.local_addr().unwrap().port();

    let address = format!("http://127.0.0.1:{}", port);

    let config = get_configuration().expect("Failed to get config");
    let db_pool = PgPool::connect(&config.db.connection_string())
        .await
        .expect("failed to connect to pg");

    let server = zero2prod::startup::run(listener, db_pool.clone())
        .expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp { address, db_pool }
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_data() {
    //Arrange
    let app = spawn_app().await;
    app.cleanup().await;

    let configuration =
        get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.db.connection_string();

    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    let client = reqwest::Client::new();

    //Act
    let response = client
        .post(format!("{}/subscriptions", &app.address))
        //.header("Content-Type", "application/x-www-form-urlencoded") //set by form()
        .form(&[("name", "le guin"), ("email", "ursula_le_guin@gmail.com")])
        .send()
        .await
        .expect("Failed to execute request");

    //Assert
    assert_eq!(200, response.status().as_u16());

    let saved = query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_400_for_invalid() {
    //Arrange
    let app = spawn_app().await;
    app.cleanup().await;

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
            .post(format!("{}/subscriptions", &app.address))
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
