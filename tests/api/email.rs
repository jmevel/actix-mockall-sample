use actix_mockall_sample::email_client::MockEmailClient;

use crate::helpers::spawn_app;

#[tokio::test]
async fn email_should_send_an_email() {
    // Arrange
    let mut email_client_mock = MockEmailClient::new();
    email_client_mock
        .expect_send_email()
        .times(1)
        .returning(|_, _, _| Ok(()));

    let app = spawn_app(Some(email_client_mock)).await;
    let client = reqwest::Client::new();

    // Act
    let _ = client
        .get(format!("{}/email", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    // The mock will panic automatically if 'send_email' wasn't called or called more than once
}

#[tokio::test]
async fn email_should_respond_with_success() {
    // Arrange
    // We don't need the mock here so we let 'spawn_app' create a default one for us
    let app = spawn_app(None).await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/email", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(String::from("Email Sent"), response.text().await.unwrap());
}
