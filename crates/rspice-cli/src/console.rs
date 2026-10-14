//! Fallible command output. A disconnected pipe is an I/O outcome, not a panic.

use crate::cli::CliError;
use rspice_core::config::{HdlOutputRecord, HdlOutputSink, HdlSeverity, HdlStream};
use std::fmt::Arguments;
use std::io::{self, Write};
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

pub(crate) fn line(arguments: Arguments<'_>) -> Result<(), CliError> {
    let mut out = io::stdout().lock();
    writeln!(out, "{arguments}")
        .and_then(|()| out.flush())
        .map_err(|error| CliError::output_error(Path::new("stdout"), error))
}

/// Diagnostics are best effort: a closed stderr must not replace the primary
/// command failure with a panic, and there is no remaining error channel.
pub(crate) fn diagnostic_line(arguments: Arguments<'_>) {
    let mut err = io::stderr().lock();
    let _ = writeln!(err, "{arguments}").and_then(|()| err.flush());
}

pub(crate) fn bytes(bytes: &[u8]) -> Result<(), CliError> {
    let mut out = io::stdout().lock();
    out.write_all(bytes)
        .and_then(|()| out.flush())
        .map_err(|error| CliError::output_error(Path::new("stdout"), error))
}

pub(crate) fn json(payload: &serde_json::Value, pretty: bool) -> Result<(), CliError> {
    write_json(&mut io::stdout().lock(), payload, pretty)
}

pub(crate) fn write_json(
    out: &mut impl Write,
    payload: &serde_json::Value,
    pretty: bool,
) -> Result<(), CliError> {
    if pretty {
        serde_json::to_writer_pretty(&mut *out, payload)
    } else {
        serde_json::to_writer(&mut *out, payload)
    }
    .map_err(|error| CliError::output_json_error(Path::new("stdout"), error))?;
    writeln!(out)
        .and_then(|()| out.flush())
        .map_err(|error| CliError::output_error(Path::new("stdout"), error))
}

/// The console of the run's HDL designs (design P2 section 4.3).
///
/// Records arrive at each accepted point, in emission order, and are written
/// as they arrive: design text to stdout (to stderr when stdout carries a
/// machine payload), severity reports and finish notices to stderr. The same
/// rendering is kept as the transcript `<run>.hdl.log`.
#[derive(Debug, Default)]
pub(crate) struct HdlConsole {
    stdout_is_payload: AtomicBool,
    /// `$error` and `$fatal` reports, which fail the process (exit 3).
    failures: AtomicU32,
    warnings: AtomicU32,
    transcript: Mutex<Transcript>,
}

#[derive(Debug, Default)]
struct Transcript {
    text: String,
    truncated: bool,
}

/// The transcript's bound, `ResourceLimits::max_hdl_output_bytes`'s default.
const MAX_TRANSCRIPT_BYTES: usize = 64 << 20;

/// The process's HDL console. One CLI process runs one command, so every run
/// of a multi-run deck shares it, and the exit status sees all of them.
pub(crate) fn hdl_console() -> Arc<HdlConsole> {
    static CONSOLE: OnceLock<Arc<HdlConsole>> = OnceLock::new();
    CONSOLE.get_or_init(Default::default).clone()
}

impl HdlConsole {
    pub(crate) fn set_stdout_is_payload(&self, payload: bool) {
        self.stdout_is_payload.store(payload, Ordering::Relaxed);
    }

    /// How many `$error` and `$fatal` reports the designs made.
    pub(crate) fn failures(&self) -> u32 {
        self.failures.load(Ordering::Relaxed)
    }

    /// The transcript so far, or `None` if the designs printed nothing.
    pub(crate) fn transcript(&self) -> Option<String> {
        let transcript = self
            .transcript
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        (!transcript.text.is_empty()).then(|| {
            let mut text = transcript.text.clone();
            if transcript.truncated {
                text.push_str("[HDL transcript truncated at 64 MiB]\n");
            }
            text
        })
    }

    fn record(&self, text: &str) {
        let mut transcript = self
            .transcript
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if transcript.text.len() + text.len() > MAX_TRANSCRIPT_BYTES {
            transcript.truncated = true;
        } else {
            transcript.text.push_str(text);
        }
    }
}

impl HdlOutputSink for HdlConsole {
    fn emit(&self, record: &HdlOutputRecord) {
        let (text, to_stderr) = match &record.stream {
            HdlStream::Stdout => (
                record.text.clone(),
                self.stdout_is_payload.load(Ordering::Relaxed),
            ),
            // A design's writes to standard error stay there.
            HdlStream::Stderr => (record.text.clone(), true),
            // File writes land in their files, not the console.
            HdlStream::File { .. } => return,
            HdlStream::Diagnostic { severity } => {
                match severity {
                    HdlSeverity::Error | HdlSeverity::Fatal => {
                        self.failures.fetch_add(1, Ordering::Relaxed);
                    }
                    HdlSeverity::Warning => {
                        self.warnings.fetch_add(1, Ordering::Relaxed);
                    }
                    HdlSeverity::Info => {}
                }
                let task = format!("${}", severity.label().to_ascii_lowercase());
                (
                    format!("{task} {}: {}\n", located(record), record.text),
                    true,
                )
            }
            HdlStream::Finish { stop, level } => {
                if *level == 0 {
                    return;
                }
                let verb = if *stop {
                    "stopped by $stop"
                } else {
                    "finished by $finish"
                };
                let mut text = format!("{verb} {}\n", located(record));
                if *level >= 2 {
                    text.push_str(&format!(
                        "  {} warning(s), {} error(s) reported by the design\n",
                        self.warnings.load(Ordering::Relaxed),
                        self.failures.load(Ordering::Relaxed)
                    ));
                }
                (text, true)
            }
        };
        if to_stderr {
            let mut err = io::stderr().lock();
            let _ = err.write_all(text.as_bytes()).and_then(|()| err.flush());
        } else {
            // A closed stdout is reported by the command's own next write;
            // the sink has no error channel.
            let mut out = io::stdout().lock();
            let _ = out.write_all(text.as_bytes()).and_then(|()| out.flush());
        }
        self.record(&text);
    }
}

/// `at t (file:line, scope)`.
fn located(record: &HdlOutputRecord) -> String {
    let origin = &record.origin;
    format!(
        "at {} ({}:{}, {})",
        si_seconds(record.seconds),
        origin.file,
        origin.line,
        origin.scope
    )
}

/// A time in seconds with an SI prefix: `65 ns`, `1.5 us`, `0 s`.
fn si_seconds(seconds: f64) -> String {
    if seconds == 0.0 || !seconds.is_finite() {
        return format!("{seconds} s");
    }
    let (scale, unit) = [
        (1.0, "s"),
        (1e-3, "ms"),
        (1e-6, "us"),
        (1e-9, "ns"),
        (1e-12, "ps"),
        (1e-15, "fs"),
    ]
    .into_iter()
    .find(|(scale, _)| seconds.abs() >= *scale * (1.0 - 1e-12))
    .unwrap_or((1e-15, "fs"));
    let value = format!("{:.6}", seconds / scale);
    let value = value.trim_end_matches('0').trim_end_matches('.');
    format!("{value} {unit}")
}

#[cfg(test)]
mod tests {
    use super::si_seconds;

    #[test]
    fn hdl_times_print_with_an_si_prefix() {
        assert_eq!(si_seconds(0.0), "0 s");
        assert_eq!(si_seconds(65e-9), "65 ns");
        assert_eq!(si_seconds(1.5e-6), "1.5 us");
        assert_eq!(si_seconds(2.0), "2 s");
        assert_eq!(si_seconds(1e-18), "0.001 fs");
    }
}
