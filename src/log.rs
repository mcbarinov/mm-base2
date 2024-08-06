use tracing::Level;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter, Layer};

pub fn init_tracing(env_filter: &str, data_dir: &str) {
    let (non_blocking, _guard) = tracing_appender::non_blocking(tracing_appender::rolling::never(data_dir, "app.log"));
    let file_layer = fmt::Layer::new().with_writer(non_blocking.with_max_level(Level::WARN)).json();
    let console_layer = fmt::layer().with_filter(EnvFilter::from(env_filter));
    tracing_subscriber::registry().with(file_layer).with(console_layer).init();
}
