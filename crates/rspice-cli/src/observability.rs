//! Process-level logging and correlation metadata.

use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::cli::LogFormat;

static MACHINE_DIAGNOSTICS: OnceLock<bool> = OnceLock::new();

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

/// Select one stderr grammar for nonfatal and fatal process diagnostics.
pub fn set_machine_diagnostics(enabled: bool) {
    let _ = MACHINE_DIAGNOSTICS.set(enabled);
}

pub fn diagnostic(code: &str, line: Option<usize>, message: impl std::fmt::Display) {
    if MACHINE_DIAGNOSTICS.get().copied().unwrap_or(false) {
        crate::console::diagnostic_line(format_args!(
            "{}",
            envelope(
                "rspice.diagnostic",
                serde_json::json!({
                    "diagnostic": { "code": code, "line": line, "message": message.to_string() },
                })
            )
        ));
    } else {
        crate::console::diagnostic_line(format_args!("{message}"));
    }
}

/// Add version and process correlation to a command's existing JSON fields.
pub fn envelope(schema: &str, mut payload: serde_json::Value) -> serde_json::Value {
    if let Some(object) = payload.as_object_mut() {
        object.insert("schema".into(), schema.into());
        object.insert("schema_version".into(), 1.into());
        object.insert("run_id".into(), run_id().into());
        object.insert(
            "tool".into(),
            serde_json::json!({
                "name": "rspice", "version": env!("CARGO_PKG_VERSION"),
                "target": env!("RSPICE_BUILD_TARGET"), "profile": env!("RSPICE_BUILD_PROFILE"),
                "commit": env!("RSPICE_BUILD_COMMIT"),
            }),
        );
    }
    payload
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
