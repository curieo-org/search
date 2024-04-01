use tokio::task::JoinHandle;
use tracing::{subscriber::set_global_default, Subscriber};
use tracing_error::ErrorLayer;
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use crate::settings::LogFmt;

pub fn get_subscriber(
    _name: String,
    env_filter: String,
    format: LogFmt,
) -> Box<dyn Subscriber + Send + Sync> {
    let env_filter = EnvFilter::new(env_filter);

    let registry = Registry::default()
        .with(ErrorLayer::default())
        .with(env_filter);

    match format {
        LogFmt::Json => {
            Box::new(registry.with(tracing_subscriber::fmt::layer().with_target(false).json()))
        }
        LogFmt::Pretty => {
            Box::new(registry.with(tracing_subscriber::fmt::layer().with_target(false).pretty()))
        }
        LogFmt::Default => Box::new(registry.with(tracing_logfmt::layer())),
    }
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}

// Spawn a blocking task that will run the provided future, with tracing context.
pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
}
