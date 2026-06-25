//! Command Implementations
//!
//! This module contains the implementation logic for each CLI subcommand:
//! - `run` - Execute simulations
//! - `info` - Display netlist information
//! - `compile_va` - Compile Verilog-A models
//! - `check` - Validate netlists
//! - `convert` - Convert output formats
//! - `compare` - Golden file regression testing

pub mod check;
pub mod compare;
pub mod compile_va;
pub mod convert;
pub(crate) mod export_table;
pub mod info;
pub mod run;
mod run_signals;
pub(crate) mod waveform_io;

pub use check::execute as check;
pub use compile_va::execute as compile_va;
pub use convert::execute as convert;
pub use info::execute as info;
pub use run::execute as run;

/// `-` as a netlist argument selects stdin.
pub(crate) fn is_stdin(path: &std::path::Path) -> bool {
    path.as_os_str() == "-"
}

/// Read a netlist from stdin for commands that accept `-`.
pub(crate) fn read_stdin_source() -> Result<String, crate::cli::CliError> {
    use std::io::Read;
    let mut source = String::new();
    std::io::stdin().read_to_string(&mut source).map_err(|e| {
        crate::cli::CliError::InputReadError {
            path: std::path::PathBuf::from("-"),
            source: e,
        }
    })?;
    Ok(source)
}

/// Parse a netlist argument, treating `-` as stdin (includes resolve
/// against the working directory).
pub(crate) fn parse_netlist_input(
    input: &std::path::Path,
) -> Result<rspice_core::Netlist, crate::cli::CliError> {
    use rspice_core::Netlist;

    let map_err = |e: rspice_core::error::ParseError| crate::cli::CliError::ParseError {
        message: e.to_string(),
        line: None,
        suggestion: None,
    };

    if is_stdin(input) {
        let source = read_stdin_source()?;
        return Netlist::parse_with_path(&source, std::path::Path::new("stdin.sp"))
            .map_err(map_err);
    }

    if !input.exists() {
        return Err(crate::cli::CliError::InputNotFound {
            path: input.to_path_buf(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"),
        });
    }
    Netlist::parse_file(input).map_err(map_err)
}

pub(crate) fn format_netlist_diagnostic(
    diagnostic: &rspice_core::netlist::ParseDiagnostic,
) -> String {
    let location = if diagnostic.line == 0 {
        String::new()
    } else {
        format!("line {}: ", diagnostic.line)
    };
    format!("{location}{}", diagnostic.message)
}

pub(crate) fn emit_netlist_diagnostics(netlist: &rspice_core::Netlist, quiet: bool) {
    if quiet {
        return;
    }

    for diagnostic in &netlist.diagnostics {
        match diagnostic.severity {
            rspice_core::netlist::DiagnosticSeverity::Warning => {
                eprintln!("warning: {}", format_netlist_diagnostic(diagnostic));
            }
        }
    }
}
