use actix_mockall_sample::{settings::get_default_settings, startup::Application};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let application = Application::build(get_default_settings(), None).await?;
    application.run_until_stopped().await?;
    Ok(())
}
