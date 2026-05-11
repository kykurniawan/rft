use tracing_appender::{non_blocking::WorkerGuard, rolling::Rotation};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::common::config::Tracing;

pub fn init(config: &Tracing) -> WorkerGuard {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let file_suffix = Some(&config.filename_suffix)
        .filter(|s| !s.is_empty())
        .map(|s| s.as_str())
        .unwrap_or("application.log");

    let directory = Some(&config.directory)
        .filter(|s| !s.is_empty())
        .map(|s| s.as_str())
        .unwrap_or("log");

    let file_appender = tracing_appender::rolling::RollingFileAppender::builder()
        .filename_suffix(file_suffix)
        .rotation(Rotation::DAILY)
        .build(directory)
        .expect("failed to build file appender for tracing");

    let (non_blocking, worker_guard) = tracing_appender::non_blocking(file_appender);

    let file_layer = fmt::layer().with_ansi(false).with_writer(non_blocking);

    let stdout_layer = fmt::layer().with_ansi(false);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(stdout_layer)
        .with(file_layer)
        .init();

    worker_guard
}
