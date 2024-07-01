use crate::settings::LogFmt;
use opentelemetry::global;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::{runtime, trace, Resource};
use tokio::task::JoinHandle;
use tracing::{subscriber::set_global_default, Subscriber};
use tracing_error::ErrorLayer;
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub fn get_subscriber(
    name: String,
    opentelemetry_collector: &String,
    env_filter: String,
    format: LogFmt,
) -> Box<dyn Subscriber + Send + Sync> {
    let env_filter = EnvFilter::new(env_filter);

    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(opentelemetry_collector),
        )
        .with_trace_config(
            trace::config().with_resource(Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                name,
            )])),
        )
        .install_batch(runtime::Tokio)
        .unwrap();

    let registry = Registry::default()
        .with(ErrorLayer::default())
        .with(env_filter)
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .with(sentry::integrations::tracing::layer());

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
