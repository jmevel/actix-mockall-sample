pub fn get_default_settings() -> Settings {
    Settings {
        application: ApplicationSettings {
            host: String::from("localhost"),
            port: 8000,
        },
        email_client: SmtpSettings {
            smtp_server: SmtpServerSettings {
                host: String::from("localhost"),
                port: 25,
                timeout: 20,
                credentials: CredentialsSettings {
                    username: String::from("username"),
                    password: String::from("password"),
                },
            },
            email: EmailSettings {
                from: String::from("Bob <bob@domain.com>"),
                reply_to: String::from("Bob <bob@domain.com>"),
            },
        },
    }
}

#[derive(Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub email_client: SmtpSettings,
}

#[derive(Clone)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Clone)]
pub struct SmtpSettings {
    pub smtp_server: SmtpServerSettings,
    pub email: EmailSettings,
}

#[derive(Clone)]
pub struct SmtpServerSettings {
    pub host: String,
    pub port: u16,
    pub timeout: u64,
    pub credentials: CredentialsSettings,
}

#[derive(Clone)]
pub struct CredentialsSettings {
    pub username: String,
    pub password: String,
}

#[derive(Clone)]
pub struct EmailSettings {
    pub from: String,
    pub reply_to: String,
}
