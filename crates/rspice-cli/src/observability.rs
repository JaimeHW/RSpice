//! Process-level logging and correlation metadata.

use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::cli::{CliError, LogFormat};

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

/// Nonfatal compiler diagnostics use the same stderr grammar as other process
/// diagnostics, retaining their source ranges and severity in JSON records.
pub fn compiler_diagnostic(diagnostic: &rspice_veriloga::SourceCompileDiagnostic) {
    if MACHINE_DIAGNOSTICS.get().copied().unwrap_or(false) {
        crate::console::diagnostic_line(format_args!(
            "{}",
            envelope(
                "rspice.diagnostic",
                serde_json::json!({"diagnostic": diagnostic})
            )
        ));
    } else {
        compiler_diagnostic_text(diagnostic);
    }
}

pub fn compiler_diagnostic_text(diagnostic: &rspice_veriloga::SourceCompileDiagnostic) {
    crate::console::diagnostic_line(format_args!("{}", format_compiler_diagnostic(diagnostic)));
}

pub fn format_compiler_diagnostic(diagnostic: &rspice_veriloga::SourceCompileDiagnostic) -> String {
    let severity = match diagnostic.severity {
        rspice_veriloga::CompileDiagnosticSeverity::Error => "Error",
        rspice_veriloga::CompileDiagnosticSeverity::Warning => "Warning",
    };
    let mut location = diagnostic.path.clone().unwrap_or_default();
    if let Some(line) = diagnostic.line {
        location.push_str(&format!(":{line}"));
        if let Some(column) = diagnostic.column {
            location.push_str(&format!(":{column}"));
        }
    }
    if !location.is_empty() {
        location.push_str(": ");
    }
    format!(
        "{severity}: {location}[{}] {}",
        diagnostic.code, diagnostic.message
    )
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
pub fn init(level: Option<&str>, format: LogFormat) -> Result<(), CliError> {
    let config_error = |error: crate::cli::config::ConfigError| CliError::ConfigError {
        message: error.to_string(),
    };
    // Select the filter before parsing it: environment module directives and
    // regexes must not weaken an explicit CLI level or --verbose override.
    let filter = match level {
        Some(level) => level.to_owned(),
        None => crate::cli::config::text_env("RUST_LOG")
            .map_err(config_error)?
            .unwrap_or_else(|| "warn".into()),
    };
    // env_logger's compatibility parser prints warnings directly to stderr and
    // silently discards invalid directives. Validate with its underlying parser
    // first so malformed filters use the selected CLI diagnostic format.
    env_filter::Builder::new()
        .try_parse(&filter)
        .map_err(|error| CliError::ConfigError {
            message: format!("Invalid RUST_LOG filter {filter:?}: {error}"),
        })?;
    let mut builder = env_logger::Builder::new();
    builder.parse_filters(&filter);
    if let Some(style) = crate::cli::config::text_env("RUST_LOG_STYLE").map_err(config_error)? {
        if !matches!(style.as_str(), "auto" | "always" | "never") {
            return Err(CliError::ConfigError {
                message: format!(
                    "Invalid RUST_LOG_STYLE={style:?}: expected auto, always, or never"
                ),
            });
        }
        builder.parse_write_style(&style);
    }

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

    builder.try_init().map_err(|error| CliError::InternalError {
        message: format!("failed to initialize diagnostic logging: {error}"),
    })
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
