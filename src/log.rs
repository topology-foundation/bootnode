use std::{io, path::PathBuf};

use rolling_file::{RollingConditionBasic, RollingFileAppender};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

const LOG_FILE: &str = "boot.log";

pub fn init_tracing() {
    let file_path: PathBuf = LOG_FILE.into();
    let (file_appender, _guard) = tracing_appender::non_blocking(
        RollingFileAppender::new(file_path, RollingConditionBasic::new().max_size(200), 5)
            .expect("Failed to initialize file appender"),
    );

    let layers = vec![
        tracing_subscriber::fmt::layer()
            .with_writer(io::stdout)
            .with_filter(EnvFilter::builder().from_env_lossy())
            .boxed(),
        tracing_subscriber::fmt::layer()
            .with_target(true)
            .with_writer(file_appender)
            .with_filter(EnvFilter::builder().from_env_lossy())
            .boxed(),
    ];

    let _ = tracing_subscriber::registry().with(layers).try_init();
}
