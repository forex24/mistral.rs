use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

use crate::DEBUG;

static LOGGER: std::sync::OnceLock<()> = std::sync::OnceLock::new();

/// This should be called to initialize the debug flag and logging.
/// This should not be called in mistralrs-core code due to Rust usage.
pub fn initialize_logging() {
    let is_debug = std::env::var("MISTRALRS_DEBUG")
        .unwrap_or_default()
        .contains('1');
    DEBUG.store(is_debug, std::sync::atomic::Ordering::Relaxed);

    LOGGER.get_or_init(|| {
        let filter = EnvFilter::builder()
            .with_default_directive(if is_debug {
                LevelFilter::DEBUG.into()
            } else {
                LevelFilter::INFO.into()
            })
            .from_env_lossy();
        tracing_subscriber::fmt().with_env_filter(filter).init();
    });
}
