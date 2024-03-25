use settings::SETTINGS;
use startup::Application;

mod err;
mod health_check;
pub mod routing;
pub mod settings;
pub mod startup;
pub mod users;

pub type Result<T> = std::result::Result<T, err::AppError>;

pub async fn run() -> color_eyre::Result<Application> {
    color_eyre::install()?;
    Application::build(SETTINGS.to_owned()).await
}
