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

    if is_stdin(input) {
        let source = read_stdin_source()?;
        return Netlist::parse_with_path(&source, std::path::Path::new("stdin.sp"))
            .map_err(map_parse_error);
    }

    if !input.exists() {
        return Err(crate::cli::CliError::InputNotFound {
            path: input.to_path_buf(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"),
        });
    }
    Netlist::parse_file(input).map_err(map_parse_error)
}

/// Preserve typed semantic context when crossing the core/CLI boundary.
pub(crate) fn map_parse_error(error: rspice_core::error::ParseError) -> crate::cli::CliError {
    let (message, line) = match &error {
        rspice_core::netlist::ParseError::OutputSymbolValidation(validation) => {
            let mut message = String::from("undefined output symbols:");
            for unresolved in &validation.unresolved {
                use std::fmt::Write as _;
                let _ = write!(
                    message,
                    "\n  {}: {} {} '{}' via {}",
                    unresolved.origin,
                    unresolved.directive,
                    unresolved.kind,
                    unresolved.symbol,
                    unresolved.operator,
                );
            }
            (
                message,
                validation.unresolved.first().and_then(|unresolved| {
                    (unresolved.origin.line != 0).then_some(unresolved.origin.line)
                }),
            )
        }
        rspice_core::netlist::ParseError::StartupDirectiveConflict(conflict) => (
            error.to_string(),
            (conflict.conflicting.line != 0).then_some(conflict.conflicting.line),
        ),
        _ => (error.to_string(), None),
    };
    crate::cli::CliError::ParseError {
        message,
        line,
        suggestion: None,
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_output_error_text_preserves_occurrence_order_and_locations() {
        let core_error = rspice_core::netlist::ParseError::OutputSymbolValidation(Box::new(
            rspice_core::netlist::OutputSymbolValidationError {
                unresolved: vec![
                    rspice_core::netlist::UnresolvedOutputSymbol {
                        directive: rspice_core::netlist::OutputDirectiveKind::Print,
                        origin: rspice_core::netlist::NetlistSourceLocation::in_file("deck.cir", 4),
                        operator: "V".into(),
                        symbol: "missing".into(),
                        kind: rspice_core::netlist::OutputSymbolKind::Node,
                    },
                    rspice_core::netlist::UnresolvedOutputSymbol {
                        directive: rspice_core::netlist::OutputDirectiveKind::Print,
                        origin: rspice_core::netlist::NetlistSourceLocation::in_file(
                            "included.cir",
                            9,
                        ),
                        operator: "I".into(),
                        symbol: "Rbogus".into(),
                        kind: rspice_core::netlist::OutputSymbolKind::Device,
                    },
                    rspice_core::netlist::UnresolvedOutputSymbol {
                        directive: rspice_core::netlist::OutputDirectiveKind::Print,
                        origin: rspice_core::netlist::NetlistSourceLocation::in_file(
                            "included.cir",
                            9,
                        ),
                        operator: "I".into(),
                        symbol: "Rbogus".into(),
                        kind: rspice_core::netlist::OutputSymbolKind::Device,
                    },
                ],
            },
        ));

        let text = map_parse_error(core_error).to_string();
        let first = text
            .find("deck.cir:4: .PRINT node 'missing' via V")
            .unwrap();
        let repeated = "included.cir:9: .PRINT device 'Rbogus' via I";
        let second = text.find(repeated).unwrap();
        let third = text.rfind(repeated).unwrap();
        assert!(first < second && second < third, "{text}");

        let mapped = map_parse_error(rspice_core::netlist::ParseError::OutputSymbolValidation(
            Box::new(rspice_core::netlist::OutputSymbolValidationError {
                unresolved: vec![rspice_core::netlist::UnresolvedOutputSymbol {
                    directive: rspice_core::netlist::OutputDirectiveKind::Save,
                    origin: rspice_core::netlist::NetlistSourceLocation::in_memory(17),
                    operator: "V".into(),
                    symbol: "missing".into(),
                    kind: rspice_core::netlist::OutputSymbolKind::Node,
                }],
            }),
        ));
        let crate::cli::CliError::ParseError { line, .. } = mapped else {
            panic!("typed output failure must map to a CLI parse error")
        };
        assert_eq!(line, Some(17));
    }

    #[test]
    fn startup_conflict_maps_to_the_conflicting_card_location() {
        let mapped = map_parse_error(rspice_core::netlist::ParseError::StartupDirectiveConflict(
            Box::new(rspice_core::netlist::StartupDirectiveConflictError {
                first_kind: rspice_core::netlist::StartupDirectiveKind::Ic,
                first: rspice_core::netlist::NetlistSourceLocation::in_file("deck.cir", 4),
                conflicting_kind: rspice_core::netlist::StartupDirectiveKind::NodeSet,
                conflicting: rspice_core::netlist::NetlistSourceLocation::in_file("deck.cir", 9),
            }),
        ));

        match mapped {
            crate::cli::CliError::ParseError { message, line, .. } => {
                assert_eq!(line, Some(9));
                assert!(message.contains("Cannot set both .IC and .NODESET simultaneously"));
                assert!(message.contains("deck.cir:4"));
                assert!(message.contains("deck.cir:9"));
            }
            other => panic!("expected parse error, got {other:?}"),
        }
    }
}
