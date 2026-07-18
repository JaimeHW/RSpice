//! Process-level logging and correlation metadata.

use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::cli::LogFormat;

static RUN_ID: OnceLock<String> = OnceLock::new();

/// Correlation identifier shared by logs, fatal diagnostics, and run summaries.
pub fn run_id() -> &'static str {
    RUN_ID.get_or_init(|| {
        let epoch_nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        format!("{:x}-{epoch_nanos:x}", std::process::id())
    })
}

/// Initialize the process logger once using the requested production format.
pub fn init(level: &str, format: LogFormat) {
    let mut builder =
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(level));

    if format == LogFormat::Json {
        let correlation_id = run_id().to_string();
        builder.format(move |buffer, record| {
            use std::io::Write as _;

            let timestamp = buffer.timestamp_millis().to_string();
            let thread = std::thread::current();
            let payload = serde_json::json!({
                "timestamp": timestamp,
                "level": record.level().as_str(),
                "target": record.target(),
                "module": record.module_path(),
                "file": record.file(),
                "line": record.line(),
                "message": record.args().to_string(),
                "run_id": correlation_id.as_str(),
                "process_id": std::process::id(),
                "thread": thread.name(),
            });
            serde_json::to_writer(&mut *buffer, &payload).map_err(std::io::Error::other)?;
            writeln!(buffer)
        });
    }

    builder.init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_id_is_stable_and_nonempty_within_the_process() {
        let first = run_id();
        let second = run_id();
        assert!(!first.is_empty());
        assert_eq!(first, second);
    }
}
