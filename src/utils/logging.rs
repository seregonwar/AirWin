use tracing::{Level, Subscriber};
use tracing_subscriber::{
    fmt::{self, time::UtcTime},
    EnvFilter,
};

pub fn setup_logging() {
    let format = fmt::format()
        .with_level(true)
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_timer(UtcTime::rfc_3339());

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .event_format(format)
        .with_env_filter(env_filter)
        .init();
}
