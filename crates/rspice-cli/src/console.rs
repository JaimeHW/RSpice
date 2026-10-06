//! Fallible command output. A disconnected pipe is an I/O outcome, not a panic.

use crate::cli::CliError;
use std::fmt::Arguments;
use std::io::{self, Write};
use std::path::Path;

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
