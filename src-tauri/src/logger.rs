use std::path::PathBuf;

use crate::constants;
use crate::utils::platform::is_windows;
use tracing_appender::rolling::Rotation;
use tracing_appender::{non_blocking::WorkerGuard, rolling::RollingFileAppender};
use tracing_subscriber::{EnvFilter, fmt, prelude::*, util::SubscriberInitExt};

/// Contains guards necessary for logging and metrics collection.
#[derive(Debug)]
pub struct LogGuard {
    _guards: Vec<WorkerGuard>,
}

/// Setup logging sub-system specifying the logging configuration, service (binary) name, and a
/// list of external crates for which a more verbose logging must be enabled. All crates within the
/// current cargo workspace are automatically considered for verbose logging.
#[allow(clippy::print_stdout)] // The logger hasn't been initialized yet
pub fn setup(log_dir: &PathBuf) -> anyhow::Result<LogGuard> {
    std::fs::create_dir_all(log_dir)?;
    let mut guards = Vec::new();

    #[cfg(debug_assertions)]
    let level_filter = get_envfilter("debug", tracing::Level::DEBUG);
    #[cfg(not(debug_assertions))]
    let level_filter = get_envfilter("info", tracing::Level::INFO);

    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix(constants::APP_NAME)
        .filename_suffix("log")
        .max_log_files(7)
        .build(log_dir)?;

    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    guards.push(guard);

    let console_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_ansi(!is_windows());

    let file_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_thread_ids(true)
        .with_thread_names(true);

    tracing_subscriber::registry()
        .with(level_filter)
        .with(file_layer)
        .with(console_layer)
        .init();

    tracing::info!(
        log_dir = %log_dir.display(),
        "Logging initialized with rotating file in directory"
    );

    Ok(LogGuard { _guards: guards })
}

fn get_envfilter(filtering_directive: &str, default_log_level: tracing::Level) -> EnvFilter {
    EnvFilter::builder()
        .with_default_directive(default_log_level.into())
        .parse(filtering_directive)
        .expect("Invalid EnvFilter filtering directive")
}
