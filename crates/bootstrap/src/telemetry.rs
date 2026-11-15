use crate::config::Config;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

pub fn init(conf: &Config) -> Option<WorkerGuard> {
    let filter = EnvFilter::try_new(&conf.log_filter).unwrap_or_else(|_| EnvFilter::new("info"));

    let console = fmt::layer().with_target(true);

    let (file_layer, guard) = match &conf.log_dir {
        Some(dir) => {
            let appender = tracing_appender::rolling::daily(dir, "myapp.log");
            let (writer, guard) = tracing_appender::non_blocking(appender);
            let layer = fmt::layer().json().with_ansi(false).with_writer(writer);
            (Some(layer), Some(guard))
        }
        None => (None, None),
    };

    tracing_subscriber::registry()
        .with(filter)
        .with(console)
        .with(file_layer) // Option<Layer> otomatis jadi no-op bila None
        .init();

    guard
}
