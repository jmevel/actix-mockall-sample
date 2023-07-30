use mockall::predicate::*;
use mockall::*;

#[automock]
pub trait EmailClient: Sync + Send {
    fn send_email(
        &self,
        recipient: &str,
        subject: &str,
        text_content: &str,
    ) -> Result<(), &'static str>;
}
