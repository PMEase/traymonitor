use std::path::PathBuf;

pub use tracing::{debug, error, event as log, info, trace, warn};
pub use tracing_attributes::instrument;

use tracing_appender::rolling::Rotation;
use tracing_appender::{non_blocking::WorkerGuard, rolling::RollingFileAppender};
use tracing_subscriber::{fmt, prelude::*, util::SubscriberInitExt, EnvFilter, Layer};

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
    let mut guards = Vec::new();

    // TODO: Setup OpenTelemetry traces and metrics

    // Setup file logging
    let file_layer = {
        // let mut path = crate::env::workspace_path();
        // path.push(&config.file.path);

        let file_appender = RollingFileAppender::builder()
            .rotation(Rotation::DAILY)
            .filename_prefix("traymonitor")
            .filename_suffix("log")
            .max_log_files(7)
            .build(log_dir)?;

        let (file_writer, guard) = tracing_appender::non_blocking(file_appender);
        guards.push(guard);

        let file_filter = get_envfilter("info", tracing::Level::WARN);
        println!("Using file logging filter: {file_filter}");
        let file_layer = fmt::layer()
            .with_timer(fmt::time::time())
            .compact()
            .with_ansi(false)
            .with_writer(file_writer)
            .with_filter(file_filter);

        Some(file_layer)
    };

    let subscriber = tracing_subscriber::registry().with(file_layer);

    // Setup console logging
    let (console_writer, guard) = tracing_appender::non_blocking(std::io::stdout());
    guards.push(guard);

    let console_filter = get_envfilter("debug", tracing::Level::INFO);
    println!("Using console logging filter: {console_filter}");
    let console_layer = fmt::layer()
        .with_timer(fmt::time::time())
        .pretty()
        .with_ansi(!cfg!(target_os = "windows"))
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_writer(console_writer)
        .with_filter(console_filter);
    subscriber.with(console_layer).init();

    // Returning the LogGuard for logs to be printed and metrics to be collected until it is
    // dropped
    Ok(LogGuard { _guards: guards })
}

fn get_envfilter(filtering_directive: &str, default_log_level: tracing::Level) -> EnvFilter {
    EnvFilter::builder()
        .with_default_directive(default_log_level.into())
        .parse(filtering_directive)
        .expect("Invalid EnvFilter filtering directive")
}
