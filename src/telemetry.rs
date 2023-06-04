use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

// Чтобы не писать 2 раза impl Subscriber + Send + Sync
pub trait Sub: Subscriber + Send + Sync {}
impl<T: Subscriber + Send + Sync> Sub for T {}

pub fn get_subscriber<Sink>(name: String, env_filter: String, sink: Sink) -> impl Sub
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber<S: Sub>(sub: S) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(sub).expect("Failed to set subscriber");
}
