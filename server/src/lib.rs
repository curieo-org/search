use crate::telemetry::{get_subscriber, init_subscriber};
use settings::SETTINGS;
use startup::Application;

pub mod auth;
mod err;
mod health_check;
pub mod routing;
pub mod search;
pub mod secrets;
pub mod settings;
pub mod startup;
mod telemetry;
pub mod users;

pub type Result<T> = std::result::Result<T, err::AppError>;

pub async fn run() -> Result<Application> {
    color_eyre::install()?;

    let subscriber = get_subscriber(
        "search-server".into(),
        SETTINGS.log.level.clone(),
        SETTINGS.log.format.clone(),
    );
    init_subscriber(subscriber);

    Application::build(SETTINGS.to_owned()).await
}
