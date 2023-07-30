use crate::{email_client::EmailClient, settings::SmtpSettings};

#[derive(Clone)]
pub struct SmtpClient {
    _smtp_settings: SmtpSettings,
}

impl SmtpClient {
    pub fn new(smtp_settings: SmtpSettings) -> SmtpClient {
        SmtpClient {
            _smtp_settings: smtp_settings,
        }
    }
}

impl EmailClient for SmtpClient {
    fn send_email(
        &self,
        _recipient: &str,
        _subject: &str,
        _text_content: &str,
    ) -> Result<(), &'static str> {
        // Not implemented
        Ok(())
    }
}
