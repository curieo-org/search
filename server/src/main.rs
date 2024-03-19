use color_eyre::Result;
use server::settings::SETTINGS;
use server::startup::Application;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let application = Application::build(SETTINGS.to_owned()).await?;

    application.run_until_stopped().await?;

    Ok(())
}
