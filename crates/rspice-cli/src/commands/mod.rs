//! Command Implementations
//!
//! This module contains the implementation logic for each CLI subcommand:
//! - `run` - Execute simulations
//! - `health` - Probe backend liveness and numerical readiness
//! - `info` - Display netlist information
//! - `compile_va` - Compile Verilog-A models
//! - `check` - Validate netlists
//! - `convert` - Convert output formats
//! - `compare` - Golden file regression testing
//!
//! (`completions` needs the whole clap command tree, so it stays in `main`.)
//!
//! The helpers below are the ingestion path every netlist-taking command
//! shares: `-` means stdin, the same resource policy bounds both stdin and
//! files, and core parse errors keep their typed context on the way out.

pub mod check;
pub mod compare;
pub mod compile_va;
pub mod convert;
pub(crate) mod export_table;
pub mod health;
pub mod info;
pub mod models;
pub(crate) mod publish;
pub mod run;
mod run_signals;
pub(crate) mod vcd_io;
pub(crate) mod waveform_io;

pub use check::execute as check;
pub use compile_va::execute as compile_va;
pub use convert::execute as convert;
pub use health::execute as health;
pub use info::execute as info;
pub use models::models;
pub use run::execute as run;

/// `-` as a netlist argument selects stdin.
pub(crate) fn is_stdin(path: &std::path::Path) -> bool {
    path.as_os_str() == "-"
}

/// Read stdin in bounded chunks under the same root-source byte policy used
/// for files, observing cooperative cancellation between reads.
pub(crate) fn read_stdin_source_with_limits_and_abort(
    resource_limits: rspice_core::ResourceLimits,
    abort: &dyn rspice_core::AbortSignal,
) -> Result<String, rspice_core::netlist::ParseWithAbortError> {
    use std::io::Read;

    const READ_CHUNK_BYTES: usize = 64 * 1024;
    let mut bytes = Vec::new();
    let mut chunk = [0_u8; READ_CHUNK_BYTES];
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    loop {
        if abort.is_aborted() {
            return Err(rspice_core::netlist::ParseWithAbortError::Aborted);
        }
        let count = stdin
            .read(&mut chunk)
            .map_err(rspice_core::netlist::ParseError::Io)?;
        if count == 0 {
            break;
        }
        let requested = bytes.len().saturating_add(count);
        if requested > resource_limits.max_netlist_bytes {
            return Err(rspice_core::netlist::ParseError::ResourceLimit(
                rspice_core::ResourceLimitError {
                    resource: rspice_core::ResourceKind::NetlistBytes,
                    requested,
                    limit: resource_limits.max_netlist_bytes,
                },
            )
            .into());
        }
        if bytes.capacity().saturating_sub(bytes.len()) < count {
            bytes.try_reserve(count).map_err(|error| {
                rspice_core::netlist::ParseError::Io(std::io::Error::other(format!(
                    "unable to grow stdin netlist buffer: {error}"
                )))
            })?;
        }
        bytes.extend_from_slice(&chunk[..count]);
    }
    if abort.is_aborted() {
        return Err(rspice_core::netlist::ParseWithAbortError::Aborted);
    }
    String::from_utf8(bytes).map_err(|error| {
        rspice_core::netlist::ParseError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            error.utf8_error(),
        ))
        .into()
    })
}

/// Parse a netlist argument, treating `-` as stdin (includes resolve
/// against the working directory).
pub(crate) fn parse_netlist_input(
    input: &std::path::Path,
    resource_limits: rspice_core::ResourceLimits,
) -> Result<rspice_core::Netlist, crate::cli::CliError> {
    use rspice_core::Netlist;
    let options = rspice_core::netlist::NetlistParseOptions {
        resource_limits,
        ..rspice_core::netlist::NetlistParseOptions::default()
    };

    if is_stdin(input) {
        let source =
            match read_stdin_source_with_limits_and_abort(resource_limits, &rspice_core::NoAbort) {
                Ok(source) => source,
                Err(rspice_core::netlist::ParseWithAbortError::Parse(error)) => {
                    return Err(map_parse_error(error));
                }
                Err(rspice_core::netlist::ParseWithAbortError::Aborted) => {
                    unreachable!("NoAbort cannot cancel stdin ingestion")
                }
            };
        return Netlist::parse_with_path_and_options(
            &source,
            std::path::Path::new("stdin.sp"),
            options,
        )
        .map_err(map_parse_error);
    }

    if !input.exists() {
        return Err(crate::cli::CliError::InputNotFound {
            path: input.to_path_buf(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"),
        });
    }
    Netlist::parse_file_with_options(input, options).map_err(map_parse_error)
}

/// Preserve typed semantic context when crossing the core/CLI boundary.
pub(crate) fn map_parse_error(error: rspice_core::error::ParseError) -> crate::cli::CliError {
    // A construct the grammar recognized and this build declines to lower is
    // a capability gap, not a malformed deck; the typed conversion keeps its
    // token, span, and exit code.
    if matches!(
        error,
        rspice_core::netlist::ParseError::UnsupportedCapability { .. }
    ) {
        return crate::cli::CliError::from(error);
    }
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
        rspice_core::netlist::ParseError::OutputExpressionValidation(validation) => (
            error.to_string(),
            (validation.origin.line != 0).then_some(validation.origin.line),
        ),
        rspice_core::netlist::ParseError::StartupDirectiveConflict(conflict) => (
            error.to_string(),
            (conflict.conflicting.line != 0).then_some(conflict.conflicting.line),
        ),
        rspice_core::netlist::ParseError::MissingDeviceModel(error) => {
            (error.to_string(), (error.line != 0).then_some(error.line))
        }
        _ => (
            error.to_string(),
            error.source_location().map(|origin| origin.line),
        ),
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

pub(crate) fn format_netlist_warning_lines(
    diagnostic: &rspice_core::netlist::ParseDiagnostic,
) -> Vec<String> {
    diagnostic
        .xyce_legacy_warning_lines()
        .map(Vec::from)
        .unwrap_or_else(|| {
            vec![format!(
                "warning: {}",
                format_netlist_diagnostic(diagnostic)
            )]
        })
}

pub(crate) fn emit_netlist_diagnostics(netlist: &rspice_core::Netlist, quiet: bool) {
    if quiet {
        return;
    }

    for diagnostic in &netlist.diagnostics {
        match diagnostic.severity {
            rspice_core::netlist::DiagnosticSeverity::Warning => {
                for line in format_netlist_warning_lines(diagnostic) {
                    eprintln!("{line}");
                }
            }
        }
    }
}

/// `text` cut to at most `max_len` characters, ending in an ellipsis when
/// anything was dropped.
///
/// Every framed summary this crate prints sizes its columns with a format
/// width, and a format width pads but never truncates: one authored name
/// longer than its column pushes that row's right border past the frame and
/// nothing else on screen moves with it. Callers that print inside a frame
/// pass the name through here first, so the column is the width it claims for
/// any input.
pub(crate) fn truncate(text: &str, max_len: usize) -> String {
    if text.chars().count() <= max_len {
        text.to_string()
    } else {
        let prefix: String = text.chars().take(max_len.saturating_sub(3)).collect();
        format!("{prefix}...")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_truncated_name_never_exceeds_its_column() {
        assert_eq!(truncate("short", 8), "short");
        assert_eq!(truncate("exactly8", 8), "exactly8");
        assert_eq!(truncate("twelve_chars", 8), "twelv...");
        assert_eq!(truncate("twelve_chars", 8).chars().count(), 8);
        // Characters, not bytes: a multi-byte name must not be cut mid-rune
        // and must still occupy the column count it was given.
        assert_eq!(
            truncate("\u{3a9}\u{3a9}\u{3a9}\u{3a9}\u{3a9}", 4),
            "\u{3a9}..."
        );
    }

    #[test]
    fn xyce_legacy_diode_warning_lines_match_the_historical_wrapper() {
        let diagnostic = rspice_core::netlist::ParseDiagnostic::warning_at(
            rspice_core::netlist::NetlistSourceLocation::in_file("dir/diode.cir", 29),
            "xyce-unknown-diode-model-parameter",
            "No model parameter BOGOPARAM found for model D1N3940 of type D, parameter ignored.",
        );
        assert_eq!(
            format_netlist_warning_lines(&diagnostic),
            [
                "Netlist warning in file diode.cir at or near line 29",
                "No model parameter BOGOPARAM found for model D1N3940 of type D, parameter ignored.",
            ]
        );
    }

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

    #[test]
    fn output_expression_error_maps_to_the_authored_card_location() {
        let mapped = map_parse_error(
            rspice_core::netlist::ParseError::OutputExpressionValidation(Box::new(
                rspice_core::netlist::OutputExpressionValidationError {
                    directive: rspice_core::netlist::OutputDirectiveKind::Print,
                    origin: rspice_core::netlist::NetlistSourceLocation::in_file("deck.cir", 9),
                    expression: "FABS(V(1))".into(),
                    issue: rspice_core::netlist::OutputExpressionIssue::UnknownFunction {
                        function: "FABS".into(),
                    },
                },
            )),
        );
        let crate::cli::CliError::ParseError { message, line, .. } = mapped else {
            panic!("typed output-expression failure must remain a CLI parse error")
        };
        assert_eq!(line, Some(9));
        assert!(message.contains("FABS"));
        assert!(message.contains("deck.cir:9"));
    }
}
