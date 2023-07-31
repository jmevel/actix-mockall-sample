use actix_mockall_sample::{settings::get_default_settings, startup::Application};

use actix_mockall_sample::email_client::MockEmailClient;

pub struct TestApp {
    pub address: String,
}

pub async fn spawn_app(email_client: Option<MockEmailClient>) -> TestApp {
    // Randomize configuration to ensure test isolation
    let settings = {
        let mut settings = get_default_settings();
        settings.application.port = 0; // binding port 0 relies on the OS to bind a random available port
        settings
    };

    let email_client: MockEmailClient = email_client.unwrap_or(default_email_client_mock());

    let application = Application::build(settings.clone(), Some(Box::new(email_client)))
        .await
        .expect("Failed to build application");

    let address = format!("http://{}:{}", application.host(), application.port());

    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp { address: address }
}

fn default_email_client_mock() -> MockEmailClient {
    let mut email_client_mock = MockEmailClient::new();
    email_client_mock
        .expect_send_email()
        .returning(|_, _, _| Ok(()));
    email_client_mock
}
