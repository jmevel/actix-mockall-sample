use actix_web::{
    dev::Server,
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use env_logger::Env;
use once_cell::sync::Lazy;
use std::{io::Error, net::TcpListener};

use crate::{
    email::email_test, email_client::EmailClient, settings::Settings, smtp_client::SmtpClient,
};

pub struct Application {
    host: String,
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(
        settings: Settings,
        email_client: Option<Box<dyn EmailClient>>,
    ) -> Result<Self, Error> {
        let email_client = email_client.unwrap_or(Box::new(SmtpClient::new(settings.email_client)));

        let address = format!(
            "{}:{}",
            settings.application.host, settings.application.port
        );
        let listener = TcpListener::bind(address).unwrap();

        Ok(Self {
            host: settings.application.host,
            port: listener.local_addr().unwrap().port(),
            server: run(listener, email_client)?,
        })
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), Error> {
        self.server.await
    }
}

// Ensure the environment logger is only initialized once using `once_cell`
// This is needed when running multiple integration tests
static LOGGING: Lazy<()> = Lazy::new(|| {
    env_logger::init_from_env(Env::default().default_filter_or("trace"));
});

pub fn run(
    listener: TcpListener,
    email_client: Box<dyn EmailClient>,
) -> Result<Server, std::io::Error> {
    Lazy::force(&LOGGING);

    let email_client = Data::new(email_client);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/{email}", web::get().to(email_test))
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
