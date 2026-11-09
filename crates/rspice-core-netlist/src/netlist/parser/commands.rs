//! Dot-command parsing for analyses, options, measurements, params, and functions.

use super::analysis_card_scan::{
    at_card_end, bind_once, card_bool, card_error, card_name, take_keyword,
};
use super::scoping::ModelDefinitionDeferrals;
use crate::config::DampingStrategy;
use crate::netlist::lexer::Token;
use crate::netlist::{XspiceAutoBridgeParamName, XspiceAutoBridgeTemplate, XyceHbTimeDomainMode};
use crate::numerics::integration::{TransientErrorControl, TransientLteReference};
use crate::solver::RealSolverBackend;

use super::*;

pub(super) mod analysis;
mod assertion;
mod options;
pub(super) use options::parse_options_command;

pub(super) fn parse_command(
    stream: &mut TokenStream,
    line_num: usize,
    context: ParseCommandContext<'_>,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    let ParseCommandContext {
        parameter_direction,
        parameter_overrides,
        mut root_redefinitions,
        logical_line,
        analysis_cards,
        startup_cards,
        scopes,
        analyses,
        monte_carlo_source_cards,
        lin_analysis,
        fft_analyses,
        unknown_warned,
        models,
        params,
        initial_conditions,
        device_initial_conditions,
        node_sets,
        global_nodes,
        measurements,
        assertions,
        vector_files,
        saves,
        output_requests,
        startup_directives,
        startup_scope,
        options,
        temperature_options,
        max_analysis_points,
        diagnostics,
        spef_includes,
        origin,
        spice_dialect,
        defer_scoped_values,
        deferred_body_params,
        model_bare_ident_deferrals,
        pending_xyce_diode_model_warnings,
        pending_ako_models,
    } = context;

    let cmd = expect_ident(stream, line_num)?;
    if analysis_cards.parse(
        scopes,
        &cmd,
        stream,
        analysis::AnalysisCardContext {
            line_num,
            logical_line,
            params,
            max_analysis_points,
            origin,
            spice_dialect,
            lin_exists: lin_analysis.is_some(),
            current_noise: options.transient_noise,
        },
        analysis::AnalysisCardSink {
            analyses,
            monte_carlo_source_cards,
            lin_analysis,
            fft_analyses,
            output_requests,
            diagnostics,
            options,
        },
    )? {
        return Ok(());
    }
    if startup_cards.parse(
        scopes,
        &cmd,
        stream,
        startup_cards::StartupCardContext {
            line: line_num,
            params,
            origin,
            scope: &startup_scope,
            defer_values: defer_scoped_values,
            after_provisional_error: temperature_options.plan.has_error(),
        },
        startup_cards::StartupCardSink {
            initial_conditions,
            node_sets,
            records: startup_directives,
            device: device_initial_conditions,
        },
    )? {
        return Ok(());
    }
    let mut require_line_consumed = true;

    match cmd.as_str() {
        ".MODEL" => {
            let model = parse_model_definition(
                stream,
                line_num,
                origin,
                params,
                models,
                false,
                ModelDefinitionDeferrals {
                    bare_ident_deferrals: model_bare_ident_deferrals,
                    pending_xyce_diode_model_warnings,
                    pending_ako_models,
                    enclosing_scopes: &[],
                },
            )?;
            models.push(model);
        }
        ".RSPICE_AUTO_BRIDGE_TEMPLATE" => {
            parse_rspice_auto_bridge_template_command(stream, line_num, params, options)?;
        }
        ".RSPICE_AUTO_BRIDGE_PARAM" => {
            parse_rspice_auto_bridge_param_command(stream, line_num, options)?;
        }
        ".RSPICE_AUTO_BRIDGE_FAMILY" => {
            parse_rspice_auto_bridge_family_command(stream, line_num, params, options)?;
        }
        _ if cmd.eq_ignore_ascii_case(
            crate::netlist::spectre_statistics::SPECTRE_STATISTICS_DIRECTIVE,
        ) =>
        {
            // Internal Spectre-adapter ABI: the versioned payload is decoded
            // once, by the whole-source prescan, so declarations apply
            // independent of library statement order. Reaching this branch
            // only proves the line was lexically consumed; it is never
            // interpreted a second time here.
            stream.skip_to_eol();
        }
        ".CODEMODEL" | ".RSPICE_UNSUPPORTED_CODEMODEL" => {
            parse_xspice_codemodel_command(stream, line_num, &cmd)?;
        }
        // ngspice reads every card that starts `.param` as one: IHP's SG13G2
        // device subcircuits are written with `.PARAMS`.
        ".PARAM" | ".PARAMS" | ".PARAMETERS" | ".CSPARAM" => {
            parse_param_statement(
                stream,
                line_num,
                params,
                deferred_body_params,
                false,
                diagnostics,
                origin,
                parameter_overrides,
                root_redefinitions.as_deref_mut(),
                temperature_options.plan,
                abort,
            )?;
        }
        ".GLOBAL_PARAM" => {
            parse_param_statement(
                stream,
                line_num,
                params,
                deferred_body_params,
                true,
                diagnostics,
                origin,
                parameter_overrides,
                root_redefinitions.as_deref_mut(),
                temperature_options.plan,
                abort,
            )?;
        }
        ".INCLUDE" | ".INC" => {
            // Include directives are handled in a preprocessing pass
            log::debug!("Include directive found: line {}", line_num);
            require_line_consumed = false;
        }
        ".SPEF_INCLUDE" => {
            // SPEF parasitics back-annotate after parsing (netlist::spef);
            // the path-aware parse entry points resolve and apply them.
            if let Some(path) = super::command_parsers::quoted_path_lexeme(stream) {
                spef_includes.push(path);
            } else {
                let TokenKind::Ident(path) = &stream.peek().kind else {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: ".spef_include requires a file path".to_string(),
                    }
                    .into());
                };
                let path = path.clone();
                stream.advance();
                spef_includes.push(path);
            }
        }
        ".LIB" => {
            // Library directives are handled in a preprocessing pass
            log::debug!("Library directive found: line {}", line_num);
            require_line_consumed = false;
        }
        ".GLOBAL" => {
            parse_global_command(stream, line_num, global_nodes)?;
        }
        ".ESAVE" => {
            options.xspice_event_trace_save = Some(parse_esave_command(stream, line_num)?);
        }
        ".TOLERANCE" => {
            let tolerance =
                super::command_parsers::parse_tolerance_command(stream, line_num, params)?;
            options
                .part_tolerances
                .retain(|existing| existing.parameter != tolerance.parameter);
            options.part_tolerances.push(tolerance);
        }
        ".FUNC" => {
            // Parse user-defined function: .FUNC name(arg1, arg2, ...) = expression
            parse_func_statement(stream, line_num, params, root_redefinitions)?;
        }
        ".PREPROCESS" => parse_preprocess_command(stream, line_num, diagnostics)?,
        ".OPTIONS" | ".OPTION" | ".OPT" | ".OPTS" => {
            let parameter_direction = if defer_scoped_values {
                // Scoped option bindings have not retained instance directions.
                if let Some(capture) = parameter_direction {
                    capture.has_uncaptured_dependencies = true;
                }
                None
            } else {
                parameter_direction
            };
            parse_options_command(
                stream,
                line_num,
                params,
                options,
                max_analysis_points,
                unknown_warned,
                diagnostics,
                parameter_direction,
                false,
                Some(temperature_options),
            )?;
        }
        ".MEAS" | ".MEASURE" => {
            // Parse measurement statement: .MEAS TRAN name TYPE signal [options]
            let authored_source = remaining_command_source(stream);
            let statement = parse_meas_command(stream, line_num, params)?;
            if let Some(previous) = measurements
                .iter()
                .position(|candidate| candidate.name.eq_ignore_ascii_case(&statement.name))
            {
                let previous_name = measurements[previous].name.clone();
                measurements.remove(previous);
                output_requests.retain(|request| {
                    request.directive != OutputDirectiveKind::Measure
                        || request
                            .name
                            .as_deref()
                            .is_none_or(|name| !name.eq_ignore_ascii_case(&previous_name))
                });
                let message = format!(
                    "measure '{previous_name}' redefined as '{}'; ignoring the previous definition",
                    statement.name
                );
                log::warn!("line {line_num}: {message}");
                diagnostics.push(ParseDiagnostic::warning(
                    line_num,
                    "measure-redefined",
                    message,
                ));
            }
            output_requests.push(OutputRequest::from_measure(
                &statement,
                origin.clone(),
                &authored_source,
            ));
            measurements.push(statement);
        }
        ".ASSERT" => {
            let Some(assertions) = assertions else {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: ".ASSERT checks a deck's result; it cannot be written inside a \
                              .SUBCKT body"
                        .to_string(),
                }
                .into());
            };
            let statement = assertion::parse_assert_command(stream, line_num, params, origin)?;
            if let Some(previous) = assertions
                .iter()
                .find(|candidate| candidate.name.eq_ignore_ascii_case(&statement.name))
            {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        ".ASSERT {} repeats the name of the assertion at {}; each assertion \
                         reports under its own name",
                        statement.name, previous.origin
                    ),
                }
                .into());
            }
            assertions.push(statement);
        }
        ".VEC" => {
            let Some(vector_files) = vector_files else {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: ".VEC applies a digital vector file to the deck; it cannot be \
                              written inside a .SUBCKT body"
                        .to_string(),
                }
                .into());
            };
            vector_files.push(assertion::parse_vec_command(stream, line_num, origin)?);
        }
        ".SAVE" | ".PROBE" => {
            let directive = if cmd == ".SAVE" {
                OutputDirectiveKind::Save
            } else {
                OutputDirectiveKind::Probe
            };
            let request = OutputRequest::from_source(
                directive,
                origin.clone(),
                &remaining_command_source(stream),
                remaining_command_expressions(stream),
            );
            let _ = parse_save_command(
                stream,
                line_num,
                logical_line,
                saves,
                false,
                SaveCommandContext {
                    params,
                    diagnostics: None,
                    origin: None,
                },
            )?;
            output_requests.push(request);
        }
        ".PRINT" | ".PLOT" => {
            // .PRINT/.PLOT take an optional leading analysis type before the
            // probe list; the probes feed the same output-selection set.
            let directive = if cmd == ".PRINT" {
                OutputDirectiveKind::Print
            } else {
                OutputDirectiveKind::Plot
            };
            let parsed = parse_save_command(
                stream,
                line_num,
                logical_line,
                saves,
                true,
                SaveCommandContext {
                    params,
                    diagnostics: Some(diagnostics),
                    origin: Some(origin),
                },
            )?;
            output_requests.push(
                OutputRequest::from_ordered_operands(
                    directive,
                    origin.clone(),
                    parsed.analysis,
                    parsed.operands,
                )
                .with_print_delimiter(parsed.delimiter)
                .with_print_layout(parsed.precision, parsed.width),
            );
        }
        _ => {
            // An unrecognized dot-command means whatever it requests will not
            // happen; that must be visible, not a debug-level whisper.
            let key = cmd.to_ascii_uppercase();
            let analysis = UNRUN_ANALYSIS_CARDS.contains(&key.as_str());
            if unknown_warned.insert(key) {
                if analysis {
                    let message = format!("unsupported analysis '{cmd}': RSpice does not run it");
                    log::warn!("line {line_num}: {message}");
                    diagnostics.push(ParseDiagnostic::warning_at(
                        origin.clone(),
                        "unsupported-analysis",
                        message,
                    ));
                } else {
                    let message = format!(
                        "unsupported dot-command '{cmd}' ignored; whatever it requests \
                         (analysis, option, output) will not run"
                    );
                    log::warn!("line {line_num}: {message}");
                    diagnostics.push(ParseDiagnostic::warning(
                        line_num,
                        "unsupported-dot-command",
                        message,
                    ));
                }
            }
            require_line_consumed = false;
        }
    }

    if require_line_consumed {
        reject_unconsumed_command_tokens(stream, line_num, &cmd)?;
    }

    Ok(())
}

/// Analysis cards of other simulators that RSpice does not run: HSPICE's
/// harmonic-balance, shooting-Newton, phase-noise, loop-stability and
/// statistical-eye analyses, Xyce's sampling, PCE and MPDE analyses, and
/// matrix-exponential and transient-noise cards.
const UNRUN_ANALYSIS_CARDS: &[&str] = &[
    ".EMBEDDEDSAMPLING",
    ".HBAC",
    ".HBLSP",
    ".HBNOISE",
    ".HBOSC",
    ".HBXF",
    ".LSTB",
    ".MATEX",
    ".MPDE",
    ".PCE",
    ".PHASENOISE",
    ".PTDNOISE",
    ".SAMPLING",
    ".SN",
    ".SNAC",
    ".SNFT",
    ".SNNOISE",
    ".SNOSC",
    ".SNXF",
    ".STATEYE",
    ".TRANNOISE",
];

/// Parse `.STB DEC|LIN|OCT np fstart fstop PROBE=<v>|<v> [NYQUIST=yes|no]`.
///
/// The probe designates the 0 V source standing in the loop: `PROBE=vname`
/// (the Spectre flavour) or, for decks written before the keyword existed, a
/// single bare trailing name. Either spelling names it once.
///
/// `NYQUIST=` is on the card because it is an engine input: it sizes the
/// result and its resource accounting and decides whether the contour is
/// sampled at all. Absent, it is yes — what every surface ran before the
/// keyword existed, so no deck changes what it computes.
fn parse_stb_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<AnalysisCommand, ParseError> {
    const CARD: AnalysisCard = AnalysisCard::Stb;

    let var_str = expect_ident(stream, line_num)?;
    let variation = match var_str.as_str() {
        "LIN" => FreqVariation::Lin,
        "OCT" => FreqVariation::Oct,
        "DEC" => FreqVariation::Dec,
        _ => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Unknown frequency variation: {}", var_str),
            });
        }
    };
    let points = expect_value(stream, line_num, params)? as usize;
    let start_freq = expect_value(stream, line_num, params)?;
    let stop_freq = expect_value(stream, line_num, params)?;

    let mut probe: Option<String> = None;
    let mut compute_nyquist: Option<bool> = None;
    loop {
        skip_commas(stream);
        if at_card_end(stream) || at_small_signal_state_keyword(stream) {
            break;
        }
        if let Some(keyword) = take_keyword(stream) {
            match keyword.as_str() {
                "PROBE" => bind_once(
                    &mut probe,
                    card_name(stream, line_num, CARD, "PROBE")?.to_ascii_uppercase(),
                    CARD,
                    line_num,
                    "PROBE",
                )?,
                "NYQUIST" => bind_once(
                    &mut compute_nyquist,
                    card_bool(stream, line_num, CARD, "NYQUIST")?,
                    CARD,
                    line_num,
                    "NYQUIST",
                )?,
                _ => {
                    return Err(card_error(
                        CARD,
                        line_num,
                        AnalysisCardIssue::UnknownKeyword { keyword },
                    ));
                }
            }
            continue;
        }
        // The legacy positional form. It names the probe and nothing else, so
        // a second bare token is the same field authored twice.
        let name = card_name(stream, line_num, CARD, "PROBE")?.to_ascii_uppercase();
        bind_once(&mut probe, name, CARD, line_num, "PROBE")?;
    }

    let Some(probe) = probe else {
        return Err(card_error(
            CARD,
            line_num,
            AnalysisCardIssue::MissingField { field: "PROBE" },
        ));
    };

    Ok(AnalysisCommand::Stb {
        variation,
        points,
        start_freq,
        stop_freq,
        probe,
        compute_nyquist: compute_nyquist.unwrap_or(true),
    })
}

#[cfg(test)]
mod stb_command_tests {
    use crate::netlist::{AnalysisCommand, Netlist};

    fn stb_deck(tail: &str) -> String {
        format!(
            "STB card\n\
             E1 eo 0 ctrl 0 -2\n\
             Vprobe eo x DC 0\n\
             R1 x ctrl 1k\n\
             C1 ctrl 0 1n\n\
             .stb dec 10 1 1meg {tail}\n\
             .end\n"
        )
    }

    fn parsed(tail: &str) -> (String, bool) {
        let netlist = Netlist::parse(&stb_deck(tail)).expect(".STB card parses");
        let [
            AnalysisCommand::Stb {
                probe,
                compute_nyquist,
                ..
            },
        ] = netlist.analyses.as_slice()
        else {
            panic!("expected one .STB analysis, got {:?}", netlist.analyses);
        };
        (probe.clone(), *compute_nyquist)
    }

    #[test]
    fn stb_reads_the_nyquist_switch_beside_either_probe_spelling() {
        let probe = "VPROBE".to_owned();
        assert_eq!(parsed("probe=vprobe"), (probe.clone(), true));
        assert_eq!(parsed("vprobe"), (probe.clone(), true));
        assert_eq!(parsed("probe=vprobe nyquist=no"), (probe.clone(), false));
        assert_eq!(parsed("NYQUIST=YES probe=vprobe"), (probe.clone(), true));
        assert_eq!(parsed("nyquist=0 vprobe"), (probe, false));
    }

    #[test]
    fn an_stb_card_refuses_an_unknown_or_repeated_keyword() {
        for (tail, named) in [
            ("probe=vprobe margins=yes", "MARGINS"),
            ("probe=vprobe probe=vother", "PROBE"),
            ("probe=vprobe nyquist=yes nyquist=no", "NYQUIST"),
            ("vprobe vother", "PROBE"),
            ("probe=vprobe nyquist=maybe", "NYQUIST"),
            ("", "PROBE"),
        ] {
            let error =
                Netlist::parse(&stb_deck(tail)).expect_err("a malformed .STB card must fail");
            let message = error.to_string();
            assert!(
                message.contains(".STB") && message.to_ascii_uppercase().contains(named),
                "the refusal names the card and the field for '{tail}': {message}"
            );
        }
    }
}

fn push_xyce_inconsistent_dc_sweep_warning(
    params: &ParamContext,
    diagnostics: &mut Vec<ParseDiagnostic>,
    origin: &NetlistSourceLocation,
    source: &str,
    spec: &crate::netlist::DcSweepSpec,
) {
    if params.expression_dialect() != crate::config::ExpressionDialect::Xyce {
        return;
    }
    let label = match spec.mode {
        crate::netlist::DcSweepMode::Linear
            if (spec.stop > spec.start && spec.step < 0.0)
                || (spec.stop < spec.start && spec.step > 0.0) =>
        {
            "Linear"
        }
        crate::netlist::DcSweepMode::Decade { .. } if spec.start > spec.stop => "Decade",
        crate::netlist::DcSweepMode::Octave { .. } if spec.start > spec.stop => "Octave",
        _ => return,
    };
    let message = format!(
        "{label} DC or STEP parameters for sweep over {source} are inconsistent; only the first requested point will be evaluated"
    );
    diagnostics.push(ParseDiagnostic::warning_at(
        origin.clone(),
        "xyce-inconsistent-dc-sweep-direction",
        message,
    ));
}

fn parse_lin_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<crate::netlist::LinAnalysis, ParseError> {
    let mut sparcalc = None;
    let mut saw_assignment = false;
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        skip_commas(stream);
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }
        let key = expect_ident(stream, line_num)?;
        if !stream.consume(&TokenKind::Equals) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    ".LIN parameter '{key}' requires an explicit NAME=value assignment"
                ),
            });
        }
        let value = expect_value(stream, line_num, params)?;
        saw_assignment = true;
        if key.eq_ignore_ascii_case("SPARCALC") {
            if sparcalc.is_some() {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: ".LIN SPARCALC may be specified only once".to_string(),
                });
            }
            if !value.is_finite() || value.fract() != 0.0 || !(0.0..=1.0).contains(&value) {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        ".LIN SPARCALC requires a finite integer 0 or 1, found {value}"
                    ),
                });
            }
            sparcalc = Some(value as i32);
        } else {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    ".LIN parameter '{key}' is not in the executable SPARCALC=0 subset"
                ),
            });
        }
    }

    if !saw_assignment || sparcalc != Some(0) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: ".LIN currently executes only the explicit SPARCALC=0 ordinary-AC mode"
                .to_string(),
        });
    }
    Ok(crate::netlist::LinAnalysis::AcOnly)
}

fn remaining_command_source(stream: &TokenStream) -> String {
    let tokens = stream.remaining_line_tokens();
    let capacity = tokens.iter().map(|token| token.lexeme.len()).sum::<usize>()
        + tokens.len().saturating_sub(1);
    let mut source = String::with_capacity(capacity);
    for token in tokens {
        push_command_token(&mut source, token);
    }
    source
}

fn remaining_command_expressions(stream: &TokenStream) -> Vec<String> {
    stream
        .remaining_line_tokens()
        .iter()
        .filter_map(|token| match &token.kind {
            TokenKind::Expression(expression) => Some(expression.clone()),
            _ => None,
        })
        .collect()
}

fn push_command_token(source: &mut String, token: &Token) {
    if !source.is_empty() {
        source.push(' ');
    }
    source.push_str(&token.lexeme);
}

fn parse_preprocess_command(
    stream: &mut TokenStream,
    line_num: usize,
    _diagnostics: &mut Vec<ParseDiagnostic>,
) -> Result<(), ParseError> {
    let operation = expect_ident(stream, line_num).map_err(|_| ParseError::Syntax {
        line: line_num,
        message: ".PREPROCESS requires an operation".to_string(),
    })?;
    if operation.eq_ignore_ascii_case("REMOVEUNUSED")
        || operation.eq_ignore_ascii_case("ADDRESISTORS")
    {
        // Root-wide semantic validation and the typed selection are owned by
        // the physical-file prescan, which also sees cards after `.END` and
        // suppresses controls from included files. The ordinary command pass
        // only consumes the already validated logical card.
        stream.skip_to_eol();
        return Ok(());
    }
    if !operation.eq_ignore_ascii_case("REPLACEGROUND") {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("Unknown .PREPROCESS operation '{operation}'"),
        });
    }
    let value = expect_ident(stream, line_num).map_err(|_| ParseError::Syntax {
        line: line_num,
        message: ".PREPROCESS REPLACEGROUND requires TRUE or FALSE".to_string(),
    })?;
    match value.to_ascii_uppercase().as_str() {
        "TRUE" | "FALSE" => {}
        _ => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Unknown argument {value} in .PREPROCESS REPLACEGROUND statement"),
            });
        }
    }
    if !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        stream.skip_to_eol();
    }
    Ok(())
}

fn parse_xspice_codemodel_command(
    stream: &mut TokenStream,
    line_num: usize,
    command: &str,
) -> Result<(), ParseError> {
    let requested = collect_codemodel_library_args(stream);
    let unsupported = requested
        .iter()
        .filter(|path| !crate::codemodels::is_builtin_codemodel_library_path(path))
        .cloned()
        .collect::<Vec<_>>();

    if !requested.is_empty() && unsupported.is_empty() {
        log::debug!(
            "{command} accepted as a compatibility no-op for built-in ngspice-46 XSPICE bundle(s): {}",
            requested.join(" ")
        );
        return Ok(());
    }

    Err(unsupported_xspice_codemodel_command(
        line_num,
        command,
        &requested,
        &unsupported,
    ))
}

fn unsupported_xspice_codemodel_command(
    line_num: usize,
    command: &str,
    requested: &[String],
    unsupported: &[String],
) -> ParseError {
    let target = if requested.is_empty() {
        "no external code-model library path was provided".to_string()
    } else if unsupported.is_empty() {
        format!(
            "requested external code-model library path(s): {}",
            requested.join(" ")
        )
    } else {
        format!(
            "unsupported external code-model library path(s): {}; requested path(s): {}",
            unsupported.join(" "),
            requested.join(" ")
        )
    };
    ParseError::Syntax {
        line: line_num,
        message: format!(
            "{command} is an ngspice dynamic XSPICE code-model loader command, \
             but RSpice does not yet load arbitrary .cm/MIF libraries; {target}. \
             Standard ngspice-46 built-in bundle names ({}) are accepted as compatibility no-ops \
             because those XSPICE models are compiled into RSpice.",
            crate::codemodels::BUILTIN_CODEMODEL_LIBRARY_NAMES.join(", ")
        ),
    }
}

fn collect_codemodel_library_args(stream: &mut TokenStream) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();

    while !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        let token = stream.advance();
        if matches!(token.kind, TokenKind::Comma) {
            flush_codemodel_library_arg(&mut current, &mut args);
            continue;
        }

        let piece = match &token.kind {
            TokenKind::StringLit(value) => {
                if token.lexeme.is_empty() {
                    value.as_str()
                } else {
                    token.lexeme.as_str()
                }
            }
            _ if !token.lexeme.is_empty() => token.lexeme.as_str(),
            _ => {
                current.push_str(&token.kind.to_string());
                if codemodel_arg_complete(&current) {
                    flush_codemodel_library_arg(&mut current, &mut args);
                }
                continue;
            }
        };

        if matches!(token.kind, TokenKind::StringLit(_)) {
            flush_codemodel_library_arg(&mut current, &mut args);
            args.push(piece.to_string());
        } else {
            current.push_str(piece);
            if codemodel_arg_complete(&current) {
                flush_codemodel_library_arg(&mut current, &mut args);
            }
        }
    }

    flush_codemodel_library_arg(&mut current, &mut args);
    args
}

fn codemodel_arg_complete(arg: &str) -> bool {
    arg.trim()
        .trim_matches('"')
        .trim_matches('\'')
        .to_ascii_lowercase()
        .ends_with(".cm")
}

fn flush_codemodel_library_arg(current: &mut String, args: &mut Vec<String>) {
    let arg = current.trim();
    if !arg.is_empty() {
        args.push(arg.to_string());
    }
    current.clear();
}

fn reject_unconsumed_command_tokens(
    stream: &mut TokenStream,
    line_num: usize,
    command: &str,
) -> Result<(), ParseError> {
    if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        return Ok(());
    }

    Err(ParseError::Syntax {
        line: line_num,
        message: format!(
            "{} has unexpected trailing token {:?}",
            command,
            stream.peek().kind
        ),
    })
}

fn parse_rspice_auto_bridge_template_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    options: &mut SimulationOptions,
) -> Result<(), ParseError> {
    let key = expect_auto_bridge_template_field(stream, line_num, "key")?;
    let setup_card = expect_auto_bridge_template_field(stream, line_num, "setup card")?;
    let device_card = expect_auto_bridge_template_field(stream, line_num, "device card")?;
    let max_value = expect_value(stream, line_num, params)?;
    if !max_value.is_finite() || max_value < 0.0 || max_value.fract() != 0.0 {
        return Err(ParseError::InvalidValue(format!(
            "line {line_num}: RSpice auto-bridge template max_nodes must be a non-negative integer"
        )));
    }

    options.set_auto_bridge_template(XspiceAutoBridgeTemplate {
        key,
        setup_card,
        device_card,
        max_nodes: (max_value as usize > 0).then_some(max_value as usize),
    });
    Ok(())
}

fn parse_rspice_auto_bridge_param_command(
    stream: &mut TokenStream,
    line_num: usize,
    options: &mut SimulationOptions,
) -> Result<(), ParseError> {
    let node_type = expect_auto_bridge_template_field(stream, line_num, "node type")?;
    let param_name = expect_auto_bridge_template_field(stream, line_num, "parameter name")?;
    if node_type.trim().is_empty() || param_name.trim().is_empty() {
        return Err(ParseError::InvalidValue(format!(
            "line {line_num}: RSpice auto-bridge parameter selector requires non-empty node type and parameter name"
        )));
    }

    options.set_auto_bridge_param_name(XspiceAutoBridgeParamName {
        node_type,
        param_name,
    });
    Ok(())
}

fn parse_rspice_auto_bridge_family_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    options: &mut SimulationOptions,
) -> Result<(), ParseError> {
    let enabled = expect_value(stream, line_num, params)?;
    if enabled != 0.0 && enabled != 1.0 {
        return Err(ParseError::InvalidValue(format!(
            "line {line_num}: RSpice auto-bridge family setting must be 0 or 1"
        )));
    }
    options.auto_bridge_family = Some(enabled != 0.0);
    Ok(())
}

fn expect_auto_bridge_template_field(
    stream: &mut TokenStream,
    line_num: usize,
    label: &str,
) -> Result<String, ParseError> {
    let raw = match &stream.peek().kind {
        TokenKind::Ident(value) | TokenKind::StringLit(value) => value.clone(),
        _ => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("RSpice auto-bridge template requires encoded {label}"),
            });
        }
    };
    stream.advance();
    decode_auto_bridge_template_field(&raw).ok_or_else(|| {
        ParseError::InvalidValue(format!(
            "line {line_num}: RSpice auto-bridge template {label} is not valid HEX_ data"
        ))
    })
}

fn decode_auto_bridge_template_field(raw: &str) -> Option<String> {
    let hex = raw.strip_prefix("HEX_")?;
    if hex.len() % 2 != 0 {
        return None;
    }
    let mut bytes = Vec::with_capacity(hex.len() / 2);
    let mut index = 0usize;
    while index < hex.len() {
        let byte = u8::from_str_radix(&hex[index..index + 2], 16).ok()?;
        bytes.push(byte);
        index += 2;
    }
    String::from_utf8(bytes).ok()
}

/// Parse a `.SAVE`/`.PROBE`/`.PRINT`/`.PLOT` probe list into the netlist's
/// output-selection set.
///
/// Accepted probe forms (case-insensitive):
/// - `all`
/// - `v(node)` / `v(a,b)` — also when the lexer splits them into
///   `v ( node )` token runs
/// - `i(elem)`
/// - `n(dev:param)` - Xyce-style native device output variable
/// - `@dev[param]`
/// - bare vector names (`out` is shorthand for `v(out)`)
///
/// With `skip_analysis_type`, a leading analysis keyword (`TRAN`, `AC`, ...)
/// is consumed and returned as request metadata, matching `.PRINT TRAN
/// v(out)` usage.
pub(super) struct ParsedSaveCommand {
    analysis: Option<OutputAnalysisKind>,
    delimiter: PrintDelimiter,
    precision: Option<i32>,
    width: Option<i32>,
    operands: Vec<OutputOperand>,
}

/// What a `.SAVE` line is parsed against: the parameter scope in force, the
/// diagnostic sink, and the source location the line came from.
pub(super) struct SaveCommandContext<'a, 'b> {
    pub params: &'a ParamContext,
    pub diagnostics: Option<&'a mut Vec<ParseDiagnostic>>,
    pub origin: Option<&'b NetlistSourceLocation>,
}

fn is_output_binary_operator(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::Other('^')
    )
}

/// Consume one bracketed run, the opening token included, and return where
/// it ends.
fn consume_output_group(
    stream: &mut TokenStream,
    line_num: usize,
    open: &TokenKind,
    close: &TokenKind,
) -> Result<usize, ParseError> {
    let mut end = stream.advance().span.end;
    let mut depth = 1usize;
    while depth > 0 {
        let token = stream.peek().clone();
        if matches!(token.kind, TokenKind::Newline | TokenKind::Eof) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Unterminated '{open}' in output expression"),
            });
        }
        if &token.kind == open {
            depth += 1;
        } else if &token.kind == close {
            depth -= 1;
        }
        end = stream.advance().span.end;
    }
    Ok(end)
}

/// Consume one operand of an unbraced output expression (signs, then a
/// probe, number, `@dev[param]`, braced or parenthesized expression).
fn consume_output_primary(
    stream: &mut TokenStream,
    line_num: usize,
    after: &str,
) -> Result<usize, ParseError> {
    while matches!(stream.peek().kind, TokenKind::Plus | TokenKind::Minus) {
        stream.advance();
    }
    let token = stream.peek().clone();
    match token.kind {
        TokenKind::Ident(_) => {
            let end = stream.advance().span.end;
            if matches!(stream.peek().kind, TokenKind::LParen) {
                consume_output_group(stream, line_num, &TokenKind::LParen, &TokenKind::RParen)
            } else {
                Ok(end)
            }
        }
        TokenKind::Number(_) | TokenKind::Expression(_) => Ok(stream.advance().span.end),
        TokenKind::LParen => {
            consume_output_group(stream, line_num, &TokenKind::LParen, &TokenKind::RParen)
        }
        TokenKind::AtSign => {
            stream.advance();
            let end = match stream.peek().kind {
                TokenKind::Ident(_) => stream.advance().span.end,
                _ => token.span.end,
            };
            if matches!(stream.peek().kind, TokenKind::LBracket) {
                consume_output_group(stream, line_num, &TokenKind::LBracket, &TokenKind::RBracket)
            } else {
                Ok(end)
            }
        }
        other => Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Expected an operand after '{after}' in output expression, found '{other}'"
            ),
        }),
    }
}

/// Consume every `<operator> <operand>` that continues an output expression
/// ending at `end`; ngspice joins operands across whitespace this way.
fn consume_output_expression_tail(
    stream: &mut TokenStream,
    line_num: usize,
    mut end: usize,
) -> Result<usize, ParseError> {
    while is_output_binary_operator(&stream.peek().kind) {
        let operator = stream.advance().kind.to_string();
        end = consume_output_primary(stream, line_num, &operator)?;
    }
    Ok(end)
}

pub(super) fn parse_save_command(
    stream: &mut TokenStream,
    line_num: usize,
    logical_line: &str,
    saves: &mut super::SaveSet,
    skip_analysis_type: bool,
    context: SaveCommandContext<'_, '_>,
) -> Result<ParsedSaveCommand, ParseError> {
    let SaveCommandContext {
        params,
        mut diagnostics,
        origin,
    } = context;
    use super::SaveSignal;
    // The operand count, save count and source offset as the latest operand
    // began, so a following operator can extend it into an expression.
    let mut previous: Option<(usize, usize, usize)> = None;

    let mut first_token = true;
    let mut parsed_any = false;
    let mut delimiter = PrintDelimiter::Whitespace;
    let mut precision = None;
    let mut width = None;
    let mut analysis = None;
    let mut operands = Vec::new();

    let authored_slice = |start: usize, end: usize, fallback: &str| {
        logical_line
            .get(start..end)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or(fallback)
            .to_string()
    };

    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        // An operator after a complete operand (`V(g)/10`, `v(1) - v(2)`),
        // or a sign that opens one (`-i(vc)`), makes the operand an
        // expression that ngspice's vector parser evaluates as one column.
        // `.save` keeps what such an expression depends on, as ngspice does.
        let extends = previous
            .filter(|(operands_before, ..)| operands.len() == operands_before + 1)
            .filter(|_| is_output_binary_operator(&stream.peek().kind));
        let opens_signed = matches!(stream.peek().kind, TokenKind::Plus | TokenKind::Minus);
        if extends.is_some() || opens_signed {
            let (operands_before, saves_before, start) = match extends {
                Some(operand) => operand,
                None => (
                    operands.len(),
                    saves.signals.len(),
                    stream.peek().span.start,
                ),
            };
            operands.truncate(operands_before);
            saves.signals.truncate(saves_before);
            let end = if extends.is_some() {
                consume_output_expression_tail(stream, line_num, start)?
            } else {
                let end = consume_output_primary(stream, line_num, "a sign")?;
                consume_output_expression_tail(stream, line_num, end)?
            };
            let body = logical_line
                .get(start..end)
                .unwrap_or_default()
                .trim()
                .to_string();
            // As for an accessor expression below: the raw selector keeps the
            // directive's save set restrictive, and the request's
            // dependencies (`V(g)` in `V(g)/10`) decide what is captured.
            saves.signals.push(SaveSignal::Raw(body.clone()));
            operands.push(OutputOperand {
                authored: body.clone(),
                kind: OutputOperandKind::Expression { body },
            });
            parsed_any = true;
            first_token = false;
            previous = None;
            continue;
        }
        skip_commas(stream);
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }
        previous = Some((
            operands.len(),
            saves.signals.len(),
            stream.peek().span.start,
        ));

        match &stream.peek().kind {
            TokenKind::Ident(raw) => {
                // TokenKind is canonicalized for case-insensitive grammar,
                // while lexeme retains the authored spelling needed by
                // output columns and typed missing-signal diagnostics.
                let mut raw = if stream.peek().lexeme.is_empty() {
                    raw.clone()
                } else {
                    stream.peek().lexeme.clone()
                };
                let upper = raw.to_ascii_uppercase();

                if first_token
                    && skip_analysis_type
                    && let Some(parsed_analysis) = OutputAnalysisKind::from_keyword(&upper)
                {
                    stream.advance();
                    analysis = Some(parsed_analysis);
                    first_token = false;
                    continue;
                }
                first_token = false;

                if skip_analysis_type && matches!(stream.peek_n(1).kind, TokenKind::Equals) {
                    stream.advance();
                    stream.advance();
                    let value = stream.peek().clone();
                    if matches!(value.kind, TokenKind::Newline | TokenKind::Eof) {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: format!(".PRINT {upper}= requires a value"),
                        });
                    }
                    let value_start = value.span.start;
                    let mut value_end = value.span.end;
                    stream.advance();
                    while !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof)
                        && stream.peek().span.start == value_end
                    {
                        value_end = stream.advance().span.end;
                    }
                    let authored_value =
                        authored_slice(value_start, value_end, value.lexeme.as_str());
                    if upper == "DELIMITER" {
                        match xyce_print_delimiter_from_value(&value.kind, &authored_value) {
                            Some(parsed) => delimiter = parsed,
                            None => {
                                delimiter = PrintDelimiter::Whitespace;
                                let message =
                                    "Invalid value of DELIMITER in .PRINT statment, ignoring";
                                log::warn!("line {line_num}: {message}");
                                if let Some(diagnostics) = diagnostics.as_deref_mut() {
                                    diagnostics.push(match origin {
                                        Some(origin) => ParseDiagnostic::warning_at(
                                            origin.clone(),
                                            "xyce-invalid-print-delimiter",
                                            message,
                                        ),
                                        None => ParseDiagnostic::warning(
                                            line_num,
                                            "xyce-invalid-print-delimiter",
                                            message,
                                        ),
                                    });
                                }
                            }
                        }
                    } else if upper == "PRECISION" || upper == "WIDTH" {
                        let parsed =
                            parse_print_layout_integer(&upper, &value.kind, params, line_num)?;
                        if upper == "PRECISION" {
                            precision = Some(parsed);
                        } else {
                            width = Some(parsed);
                        }
                    }
                    continue;
                }

                if skip_analysis_type && upper == "NOINDEX" {
                    stream.advance();
                    continue;
                }

                let start = stream.peek().span.start;
                let mut end = stream.peek().span.end;
                let raw_end = end;
                stream.advance();

                // Differential-style bare node names commonly end in `+`
                // or `-` (for example `save in+ in-`). The general lexer
                // emits the sign separately because it is also an arithmetic
                // operator. Reattach only a source-contiguous suffix so
                // whitespace-separated expressions retain their old token
                // boundaries.
                if stream.peek().span.start == raw_end {
                    match stream.peek().kind {
                        TokenKind::Plus => {
                            raw.push('+');
                            end = stream.advance().span.end;
                        }
                        TokenKind::Minus => {
                            raw.push('-');
                            end = stream.advance().span.end;
                        }
                        _ => {}
                    }
                }

                if upper == "ALL" {
                    let signal = SaveSignal::All;
                    saves.signals.push(signal.clone());
                    operands.push(OutputOperand {
                        authored: authored_slice(start, end, &raw),
                        kind: OutputOperandKind::Probe(signal),
                    });
                    parsed_any = true;
                    continue;
                }

                // Function-style output operands may arrive as an identifier
                // followed by a parenthesized token run. Collect the complete
                // run here so it remains exactly one ordered column. V/I and
                // device-parameter N forms have a direct SaveSignal; the
                // remaining Xyce accessors (including node-form N and derived
                // currents such as IR) are evaluated through the expression
                // engine, which owns their typed accessor semantics.
                let is_direct_probe_prefix = upper == "V" || upper == "I" || upper == "N";
                if matches!(stream.peek().kind, TokenKind::LParen) {
                    let mut probe = raw.clone();
                    probe.push('(');
                    end = stream.advance().span.end; // consume '('
                    let mut depth = 1usize;
                    while depth > 0
                        && !stream.is_eof()
                        && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof)
                    {
                        let token = stream.peek().clone();
                        match &token.kind {
                            TokenKind::LParen => {
                                depth += 1;
                                probe.push('(');
                            }
                            TokenKind::RParen => {
                                depth -= 1;
                                if depth > 0 {
                                    probe.push(')');
                                }
                            }
                            _ => probe.push_str(&token.lexeme),
                        }
                        end = token.span.end;
                        stream.advance();
                    }
                    if depth != 0 {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: format!("Unterminated output probe '{probe}'"),
                        });
                    }
                    probe.push(')');
                    let kind = match parse_save_probe(&probe) {
                        Some(signal) if is_direct_probe_prefix => {
                            saves.signals.push(signal.clone());
                            OutputOperandKind::Probe(signal)
                        }
                        _ => {
                            // A source-authored output directive must keep the
                            // save set non-empty even when the operand is an
                            // expression/accessor. Runtime capture is refined
                            // by OutputRequest dependencies; this exact raw
                            // selector preserves the directive's restrictive
                            // (rather than implicit-ALL) storage contract.
                            saves.signals.push(SaveSignal::Raw(probe.clone()));
                            OutputOperandKind::Expression {
                                body: probe.clone(),
                            }
                        }
                    };
                    operands.push(OutputOperand {
                        authored: authored_slice(start, end, &probe),
                        kind,
                    });
                    parsed_any = true;
                    continue;
                }

                if let Some(signal) = parse_save_probe(&raw) {
                    saves.signals.push(signal.clone());
                    operands.push(OutputOperand {
                        authored: authored_slice(start, end, &raw),
                        kind: OutputOperandKind::Probe(signal),
                    });
                    parsed_any = true;
                }
            }
            TokenKind::AtSign => {
                let start = stream.peek().span.start;
                stream.advance();
                first_token = false;
                // @dev[param]: device then bracketed parameter name.
                let (device, mut end) = match &stream.peek().kind {
                    TokenKind::Ident(s) => {
                        let device = if stream.peek().lexeme.is_empty() {
                            s.clone()
                        } else {
                            stream.peek().lexeme.clone()
                        };
                        let end = stream.advance().span.end;
                        (device, end)
                    }
                    _ => {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: "Expected device name after '@' in save directive".to_string(),
                        });
                    }
                };
                let signal = if stream.consume(&TokenKind::LBracket) {
                    let param = match &stream.peek().kind {
                        TokenKind::Ident(s) => {
                            let param = if stream.peek().lexeme.is_empty() {
                                s.clone()
                            } else {
                                stream.peek().lexeme.clone()
                            };
                            stream.advance();
                            param
                        }
                        _ => {
                            return Err(ParseError::Syntax {
                                line: line_num,
                                message: format!(
                                    "Expected parameter name in '@{}[...]' save directive",
                                    device
                                ),
                            });
                        }
                    };
                    if !matches!(stream.peek().kind, TokenKind::RBracket) {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: format!(
                                "Expected closing ']' in '@{device}[{param}]' save directive"
                            ),
                        });
                    }
                    end = stream.advance().span.end;
                    SaveSignal::DeviceParam { device, param }
                } else {
                    SaveSignal::Raw(device)
                };
                saves.signals.push(signal.clone());
                let fallback = logical_line.get(start..end).unwrap_or_default();
                operands.push(OutputOperand {
                    authored: authored_slice(start, end, fallback),
                    kind: OutputOperandKind::Probe(signal),
                });
                parsed_any = true;
            }
            TokenKind::Number(n) => {
                let start = stream.peek().span.start;
                let end = stream.peek().span.end;
                // Numeric node names (e.g. `.save 2`) select v(2).
                let name = if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                };
                stream.advance();
                let signal = SaveSignal::Raw(name.clone());
                saves.signals.push(signal.clone());
                operands.push(OutputOperand {
                    authored: authored_slice(start, end, &name),
                    kind: OutputOperandKind::Probe(signal),
                });
                parsed_any = true;
                first_token = false;
            }
            TokenKind::Expression(body) | TokenKind::StringLit(body) => {
                let body = body.clone();
                let start = stream.peek().span.start;
                let end = stream.advance().span.end;
                operands.push(OutputOperand {
                    authored: authored_slice(start, end, &body),
                    kind: OutputOperandKind::Expression { body },
                });
                parsed_any = true;
                first_token = false;
            }
            TokenKind::Comma => {
                stream.advance();
            }
            unexpected => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!("Unexpected token '{unexpected}' in output directive"),
                });
            }
        }
    }

    if !parsed_any {
        // ngspice warns and ignores a bare .print/.save (several corpus
        // decks carry one); a hard error would reject the whole deck.
        log::warn!("line {line_num}: save/print directive without output signals ignored");
    }

    Ok(ParsedSaveCommand {
        analysis,
        delimiter,
        precision,
        width,
        operands,
    })
}

fn parse_print_layout_integer(
    name: &str,
    token: &TokenKind,
    params: &ParamContext,
    line_num: usize,
) -> Result<i32, ParseError> {
    let value = match token {
        TokenKind::Number(value) => *value,
        TokenKind::Expression(expression) => eval_expression(expression, params)
            .map_err(|error| ParseError::InvalidValue(error.to_string()))?,
        TokenKind::Ident(identifier) => params
            .get(identifier)
            .or_else(|| crate::netlist::lexer::parse_spice_value(identifier).ok())
            .ok_or_else(|| ParseError::Syntax {
                line: line_num,
                message: format!(".PRINT {name} expects an integer value, found {identifier:?}"),
            })?,
        _ => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(".PRINT {name} expects an integer value"),
            });
        }
    };
    let truncated = value.trunc();
    if !value.is_finite() || truncated < i32::MIN as Value || truncated > i32::MAX as Value {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                ".PRINT {name} expects a finite signed 32-bit numeric value, found {value}"
            ),
        });
    }
    // Xyce 7.10 retrieves WIDTH and PRECISION with
    // `Param::getImmutableValue<int>()`, which converts numeric values with
    // `static_cast<int>` and therefore truncates toward zero.
    Ok(truncated as i32)
}

fn xyce_print_delimiter_from_value(
    first_token: &TokenKind,
    authored_value: &str,
) -> Option<PrintDelimiter> {
    let keyword = authored_value.trim_matches('"');
    if keyword.eq_ignore_ascii_case("TAB") {
        return Some(PrintDelimiter::Tab);
    }
    if keyword.eq_ignore_ascii_case("COMMA") || keyword == "," {
        return Some(PrintDelimiter::Comma);
    }
    if keyword.eq_ignore_ascii_case("COLON") || keyword == ":" {
        return Some(PrintDelimiter::Colon);
    }
    if keyword.eq_ignore_ascii_case("SEMICOLON") || keyword == ";" {
        return Some(PrintDelimiter::Semicolon);
    }

    match first_token {
        TokenKind::StringLit(value) if !value.is_empty() => {
            Some(PrintDelimiter::Custom(value.clone()))
        }
        _ => None,
    }
}

/// Parse a single textual probe (`v(out)`, `v(a,b)`, `i(v1)`, `n(m1:id)`,
/// `@m1[id]`, or a bare vector name) into a [`super::SaveSignal`].
///
/// Public (via the netlist module) so frontends can build a
/// [`super::SaveSet`] from user-supplied probe specs — e.g. the CLI
/// `--save` flag — with netlist semantics.
pub fn parse_save_probe(raw: &str) -> Option<super::SaveSignal> {
    use super::SaveSignal;

    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    if trimmed.eq_ignore_ascii_case("all") {
        return Some(SaveSignal::All);
    }

    let wrapped = |operator: char| {
        let mut chars = trimmed.chars();
        let first = chars.next()?;
        if !first.eq_ignore_ascii_case(&operator) {
            return None;
        }
        let rest = &trimmed[first.len_utf8()..];
        rest.strip_prefix('(')
            .and_then(|value| value.strip_suffix(')'))
    };

    if let Some(inner) = wrapped('v') {
        let inner = inner.trim();
        if inner.is_empty() {
            return None;
        }
        return Some(match inner.split_once(',') {
            Some((a, b)) => SaveSignal::VoltageDiff(a.trim().to_string(), b.trim().to_string()),
            None => SaveSignal::Voltage(inner.to_string()),
        });
    }

    if let Some(inner) = wrapped('i') {
        let inner = inner.trim();
        if inner.is_empty() {
            return None;
        }
        return Some(SaveSignal::Current(inner.to_string()));
    }

    if let Some(inner) = wrapped('n') {
        if let Some((device, param)) = inner.rsplit_once(':') {
            let device = device.trim();
            let param = param.trim();
            if !device.is_empty() && !param.is_empty() {
                return Some(SaveSignal::DeviceParam {
                    device: device.to_string(),
                    param: param.to_string(),
                });
            }
        }
        return None;
    }

    if let Some(rest) = trimmed.strip_prefix('@') {
        if let Some((device, param)) = rest
            .split_once('[')
            .and_then(|(d, p)| p.strip_suffix(']').map(|p| (d, p)))
        {
            return Some(SaveSignal::DeviceParam {
                device: device.trim().to_string(),
                param: param.trim().to_string(),
            });
        }
        return Some(SaveSignal::Raw(rest.trim().to_string()));
    }

    // ngspice names a branch current `<device>#branch`.
    if let Some(device) = trimmed
        .len()
        .checked_sub("#branch".len())
        .filter(|&split| split > 0 && trimmed.is_char_boundary(split))
        .filter(|&split| trimmed[split..].eq_ignore_ascii_case("#branch"))
        .map(|split| &trimmed[..split])
    {
        return Some(SaveSignal::Current(device.to_string()));
    }

    Some(SaveSignal::Raw(trimmed.to_string()))
}

fn warn_duplicate_packaged_option(
    name: &str,
    line_num: usize,
    diagnostics: &mut Vec<ParseDiagnostic>,
) {
    let message = format!("duplicate .OPTIONS {name} ignored; using the first value");
    log::warn!("line {line_num}: {message}");
    diagnostics.push(ParseDiagnostic::warning(
        line_num,
        "duplicate-option",
        message,
    ));
}

fn ignore_unknown_option(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    has_equals: bool,
    warning_key: &str,
    unknown_warned: &mut std::collections::HashSet<String>,
    diagnostics: &mut Vec<ParseDiagnostic>,
) {
    // Unknown options may be bare flags. Consume a value only when it was
    // explicitly assigned. Silently swallowing tolerance or compatibility
    // knobs misleads users into thinking they took effect, so diagnose each
    // scoped key once.
    if unknown_warned.insert(format!(".options {warning_key}")) {
        let message = format!("unknown .options key '{warning_key}' ignored");
        log::warn!("line {line_num}: {message}");
        diagnostics.push(ParseDiagnostic::warning(
            line_num,
            "unknown-option",
            message,
        ));
    }
    if has_equals {
        skip_ignored_option_value(stream, params);
    }
}

/// Consume an ignored option's value: one value, or Xyce's comma list of them
/// (`initialvalue=0.0,0.0`, `conparam=mosfet:gainscale,mosfet:nltermscale`,
/// `outputs={v(5)},{v(2)}`), each a number, a name, a brace expression or a
/// quoted string. A comma before `key=` starts the next option instead.
fn skip_ignored_option_value(stream: &mut TokenStream, params: &ParamContext) {
    loop {
        if try_value(stream, params).is_none() {
            if !matches!(
                stream.peek().kind,
                TokenKind::Ident(_) | TokenKind::Expression(_) | TokenKind::StringLit(_)
            ) {
                return;
            }
            stream.advance();
            while matches!(stream.peek().kind, TokenKind::Other(':'))
                && matches!(stream.peek_n(1).kind, TokenKind::Ident(_))
            {
                stream.advance();
                stream.advance();
            }
        }
        if !matches!(stream.peek().kind, TokenKind::Comma)
            || matches!(stream.peek_n(1).kind, TokenKind::Ident(_))
                && matches!(stream.peek_n(2).kind, TokenKind::Equals)
        {
            return;
        }
        stream.advance();
    }
}

pub(super) fn expect_option_key(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<(String, usize), ParseError> {
    skip_commas(stream);

    let TokenKind::Ident(first) = &stream.peek().kind else {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("Expected identifier, found {:?}", stream.peek().kind),
        });
    };
    let mut key = first.clone();
    let mut end = stream.peek().span.end;
    stream.advance();

    while matches!(stream.peek().kind, TokenKind::Minus)
        && matches!(stream.peek_n(1).kind, TokenKind::Ident(_))
    {
        stream.advance();
        let TokenKind::Ident(part) = &stream.peek().kind else {
            unreachable!("peek_n verified identifier after option-key hyphen")
        };
        key.push('-');
        key.push_str(part);
        end = stream.peek().span.end;
        stream.advance();
    }

    Ok((key, end))
}

/// Whether an upper-cased identifier names one of the option packages this
/// parser scopes a `.OPTIONS` command to.
///
/// Public because a writer of `.OPTIONS` cards has to be able to check its own
/// package headers against the reader's list rather than against a second copy
/// of it: a header this returns `false` for is read as an ordinary global key,
/// so every key after it on that card lands in the wrong place.
pub fn option_package_key_is_known(key_upper: &str) -> bool {
    matches!(
        key_upper,
        "TOPOLOGY"
            | "FFT"
            | "MEASURE"
            | "DEVICE"
            | "XSPICE"
            | "TIMEINT"
            | "NONLIN"
            | "NONLIN-TRAN"
            | "NONLIN-TRANSIENT"
            | "NONLIN-HB"
            | "NONLIN-CONTINUATION"
            | "LOCA"
            | "OUTPUT"
            | "PARSER"
            | "RESTART"
            | "HBINT"
            | "LINSOL-HB"
    )
}

fn restart_interval_schedule_starts(stream: &TokenStream, params: &ParamContext) -> bool {
    // An assignment starts a named option even when its name also names a
    // parameter. It cannot be one of the schedule's positional values.
    if matches!(stream.peek().kind, TokenKind::Ident(_))
        && matches!(stream.peek_n(1).kind, TokenKind::Equals)
    {
        return false;
    }
    if let TokenKind::Ident(key) = &stream.peek().kind
        && matches!(
            key.to_ascii_uppercase().as_str(),
            "PACK"
                | "PRINT_TIMEINT_OPTIONS"
                | "PRINTTIMEINTOPTIONS"
                | "JOB"
                | "START_TIME"
                | "STARTTIME"
                | "FILE"
                | "INITIAL_INTERVAL"
                | "INITIALINTERVAL"
        )
    {
        return false;
    }

    // Classification must neither draw a random value nor erase a failed
    // authored expression. Numeric validation belongs to the schedule reader.
    let offset = usize::from(matches!(
        stream.peek().kind,
        TokenKind::Plus | TokenKind::Minus
    ));
    match &stream.peek_n(offset).kind {
        TokenKind::Number(_) | TokenKind::Expression(_) => true,
        TokenKind::Ident(name) => {
            params.get(name).is_some()
                || parse_boolean_literal(name).is_some()
                || parse_spice_value(name).is_ok()
        }
        _ => false,
    }
}

fn restart_interval_count(options: &super::SimulationOptions) -> usize {
    options
        .restart
        .as_ref()
        .map_or(0, |restart| restart.intervals.len())
}

fn parse_restart_boolean_option(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    has_equals: bool,
    option_name: &str,
) -> Result<bool, ParseError> {
    if !has_equals {
        return Ok(true);
    }

    if let TokenKind::Ident(word) = &stream.peek().kind {
        let enabled = match word.to_ascii_lowercase().as_str() {
            "true" | "yes" | "on" => Some(true),
            "false" | "no" | "off" => Some(false),
            _ => None,
        };
        if let Some(enabled) = enabled {
            stream.advance();
            return Ok(enabled);
        }
    }

    let authored = stream.peek().lexeme.clone();
    let value = expect_value(stream, line_num, params).map_err(|_| ParseError::Syntax {
        line: line_num,
        message: format!("{option_name} expects 0 or 1, found '{authored}'"),
    })?;
    if !value.is_finite() || value.fract() != 0.0 || !(0.0..=1.0).contains(&value) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("{option_name} expects 0 or 1, found {value}"),
        });
    }
    Ok(value == 1.0)
}

/// Parse one logical Xyce restart file/job name.
///
/// The lexer intentionally splits punctuation that has meaning elsewhere, so
/// an unquoted name such as `trans_test2e-08` arrives as several tokens. Only
/// source-contiguous fragments are rejoined: whitespace remains the reliable
/// boundary between this value and the next option. Quoted values are decoded
/// by the lexer and may contain whitespace.
pub(super) fn parse_restart_string_option(
    stream: &mut TokenStream,
    line_num: usize,
    option_name: &str,
) -> Result<String, ParseError> {
    let first = stream.peek().clone();
    match &first.kind {
        TokenKind::StringLit(value) => {
            if value.trim().is_empty() {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!("{option_name} requires a non-empty value"),
                });
            }
            let value = value.clone();
            stream.advance();
            reject_restart_quoted_value_suffix(stream, line_num, option_name, first.span.end)?;
            return Ok(value);
        }
        TokenKind::Expression(value)
            if first.lexeme.starts_with('\'') && first.lexeme.ends_with('\'') =>
        {
            if value.trim().is_empty() {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!("{option_name} requires a non-empty value"),
                });
            }
            let value = value.clone();
            stream.advance();
            reject_restart_quoted_value_suffix(stream, line_num, option_name, first.span.end)?;
            return Ok(value);
        }
        TokenKind::Newline | TokenKind::Eof | TokenKind::Comma | TokenKind::Equals => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("{option_name} requires a non-empty value"),
            });
        }
        _ => {}
    }

    let mut value = String::new();
    let mut previous_end = None;
    loop {
        let token = stream.peek();
        if matches!(
            token.kind,
            TokenKind::Newline
                | TokenKind::Eof
                | TokenKind::Comma
                | TokenKind::Equals
                | TokenKind::StringLit(_)
                | TokenKind::Expression(_)
        ) || previous_end.is_some_and(|end| token.span.start != end)
        {
            break;
        }
        if token.lexeme.is_empty() {
            break;
        }
        value.push_str(&token.lexeme);
        previous_end = Some(token.span.end);
        stream.advance();
    }

    if value.is_empty() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("{option_name} requires a non-empty value"),
        });
    }
    Ok(value)
}

fn reject_restart_quoted_value_suffix(
    stream: &TokenStream,
    line_num: usize,
    option_name: &str,
    quoted_end: usize,
) -> Result<(), ParseError> {
    if stream.peek().span.start == quoted_end
        && !matches!(
            stream.peek().kind,
            TokenKind::Newline | TokenKind::Eof | TokenKind::Comma
        )
    {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "{option_name} quoted value must be separated from the following option"
            ),
        });
    }
    Ok(())
}

fn parse_restart_interval_schedule(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    options: &mut super::SimulationOptions,
    max_analysis_points: usize,
) -> Result<(), ParseError> {
    let retained_points = options
        .output_time_points
        .len()
        .saturating_add(options.timeint_breakpoints.len())
        .saturating_add(output_interval_count(options))
        .saturating_add(
            options
                .restart
                .as_ref()
                .map_or(0, |restart| restart.intervals.len()),
        );
    let mut intervals = Vec::new();
    let mut previous_time = options
        .restart
        .as_ref()
        .and_then(|restart| restart.intervals.last())
        .map(|interval| interval.time);

    while !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        skip_commas(stream);
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }

        let time = expect_value(stream, line_num, params).map_err(|_| ParseError::Syntax {
            line: line_num,
            message: "RESTART interval schedule expects <time> <interval> pairs".to_string(),
        })?;
        let time = parse_non_negative_real_option("RESTART.TIME", time, line_num)?;

        skip_commas(stream);
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    "RESTART interval schedule is missing the interval paired with time {time}"
                ),
            });
        }
        let interval = expect_value(stream, line_num, params).map_err(|_| ParseError::Syntax {
            line: line_num,
            message: "RESTART interval schedule expects <time> <interval> pairs".to_string(),
        })?;
        let interval = parse_positive_real_option("RESTART.INTERVAL", interval, line_num)?;

        if previous_time.is_some_and(|previous| time <= previous) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    "RESTART.TIME values must be strictly increasing; found {time} after {}",
                    previous_time.expect("checked above")
                ),
            });
        }
        crate::resource::ResourceLimitError::ensure(
            crate::resource::ResourceKind::AnalysisPoints,
            retained_points
                .saturating_add(intervals.len())
                .saturating_add(1),
            max_analysis_points,
        )
        .map_err(ParseError::from)?;

        let time = if time == 0.0 { 0.0 } else { time };
        previous_time = Some(time);
        intervals.push(crate::netlist::XyceRestartInterval { time, interval });
    }

    options
        .restart
        .get_or_insert_default()
        .intervals
        .extend(intervals);
    Ok(())
}

fn parse_output_interval_schedule(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    options: &super::SimulationOptions,
    max_analysis_points: usize,
) -> Result<Vec<crate::netlist::XyceOutputInterval>, ParseError> {
    let retained_points = options
        .output_time_points
        .len()
        .saturating_add(options.timeint_breakpoints.len())
        .saturating_add(restart_interval_count(options));
    let mut intervals = Vec::new();
    let mut previous_time = None;
    loop {
        skip_commas(stream);
        if !restart_interval_schedule_starts(stream, params) {
            return Ok(intervals);
        }
        let time = expect_value(stream, line_num, params).map_err(|_| ParseError::Syntax {
            line: line_num,
            message: "OUTPUT interval schedule expects <time> <interval> pairs".to_string(),
        })?;
        let time = parse_non_negative_real_option("OUTPUT.TIME", time, line_num)?;
        skip_commas(stream);
        if !restart_interval_schedule_starts(stream, params) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    "OUTPUT interval schedule is missing the interval paired with time {time}"
                ),
            });
        }
        let interval = expect_value(stream, line_num, params).map_err(|_| ParseError::Syntax {
            line: line_num,
            message: "OUTPUT interval schedule expects <time> <interval> pairs".to_string(),
        })?;
        let interval = parse_positive_real_option("OUTPUT.INTERVAL", interval, line_num)?;
        if previous_time.is_some_and(|previous| time <= previous) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    "OUTPUT.TIME values must be strictly increasing; found {time} after {}",
                    previous_time.expect("checked above")
                ),
            });
        }
        crate::resource::ResourceLimitError::ensure(
            crate::resource::ResourceKind::AnalysisPoints,
            retained_points
                .saturating_add(intervals.len())
                .saturating_add(1),
            max_analysis_points,
        )
        .map_err(ParseError::from)?;
        let time = if time == 0.0 { 0.0 } else { time };
        previous_time = Some(time);
        intervals.push(crate::netlist::XyceOutputInterval { time, interval });
    }
}

fn output_interval_count(options: &super::SimulationOptions) -> usize {
    options
        .output_interval_schedule
        .as_ref()
        .map_or(0, |schedule| schedule.intervals.len())
}

fn parse_time_point_vector_option(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    option_name: &str,
    retained_points: usize,
    max_analysis_points: usize,
) -> Result<Vec<Value>, ParseError> {
    let mut values = Vec::new();
    loop {
        let value = expect_value(stream, line_num, params).map_err(|_| ParseError::Syntax {
            line: line_num,
            message: format!("{option_name} expects a non-empty comma-separated time list"),
        })?;
        crate::resource::ResourceLimitError::ensure(
            crate::resource::ResourceKind::AnalysisPoints,
            retained_points
                .saturating_add(values.len())
                .saturating_add(1),
            max_analysis_points,
        )
        .map_err(ParseError::from)?;
        if !value.is_finite() || value < 0.0 {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    "{option_name} time points must be finite and nonnegative, found {value}"
                ),
            });
        }
        values.push(if value == 0.0 { 0.0 } else { value });

        if !stream.consume(&TokenKind::Comma) {
            break;
        }
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("{option_name} has a trailing comma without a time point"),
            });
        }
    }
    Ok(values)
}

fn append_canonical_time_points(
    target: &mut Vec<Value>,
    mut values: Vec<Value>,
    other: &[Value],
    max_analysis_points: usize,
) -> Result<(), ParseError> {
    target.append(&mut values);
    target.sort_by(Value::total_cmp);
    target.dedup_by(|left, right| {
        (*left - *right).abs() <= crate::numerics::integration::XYCE_BREAKPOINT_TOLERANCE
    });
    crate::resource::ResourceLimitError::ensure(
        crate::resource::ResourceKind::AnalysisPoints,
        target.len().saturating_add(other.len()),
        max_analysis_points,
    )
    .map_err(ParseError::from)
}

fn parse_boolean_option(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    has_equals: bool,
) -> Result<bool, ParseError> {
    if !has_equals {
        return Ok(true);
    }

    if let TokenKind::Ident(word) = &stream.peek().kind {
        let enabled = match word.to_ascii_lowercase().as_str() {
            "true" | "yes" | "on" => Some(true),
            "false" | "no" | "off" => Some(false),
            _ => None,
        };
        if let Some(enabled) = enabled {
            stream.advance();
            return Ok(enabled);
        }
    }

    Ok(expect_value(stream, line_num, params)? != 0.0)
}

fn parse_fft_boolean_option(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    has_equals: bool,
    name: &str,
) -> Result<bool, ParseError> {
    if !has_equals {
        return Ok(true);
    }

    if let TokenKind::Ident(word) = &stream.peek().kind {
        let enabled = match word.to_ascii_lowercase().as_str() {
            "true" | "yes" | "on" => Some(true),
            "false" | "no" | "off" => Some(false),
            _ => None,
        };
        if let Some(enabled) = enabled {
            stream.advance();
            return Ok(enabled);
        }
    }

    let value = expect_value(stream, line_num, params)?;
    if !value.is_finite() || value != value.round() || !(0.0..=1.0).contains(&value) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("FFT.{name} must be the integer 0 or 1, found {value}"),
        });
    }
    Ok(value == 1.0)
}

fn parse_auto_bridge_option(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    has_equals: bool,
) -> Result<(bool, bool), ParseError> {
    if !has_equals {
        return Ok((true, false));
    }

    if let TokenKind::Ident(word) = &stream.peek().kind {
        let enabled = match word.to_ascii_lowercase().as_str() {
            "true" | "yes" | "on" => Some(true),
            "false" | "no" | "off" => Some(false),
            _ => None,
        };
        if let Some(enabled) = enabled {
            stream.advance();
            return Ok((enabled, false));
        }
    }

    let value = expect_value(stream, line_num, params)?;
    Ok((value != 0.0, value >= 2.0))
}

fn parse_method_option(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<String, ParseError> {
    match &stream.peek().kind {
        TokenKind::Ident(method) => {
            let method = method.to_uppercase();
            stream.advance();
            Ok(method)
        }
        TokenKind::Number(_) | TokenKind::Expression(_) => {
            let value = expect_value(stream, line_num, params)?;
            if !value.is_finite() || value.fract() != 0.0 {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        ".OPTIONS METHOD expects an integer selector or method name, found {value}"
                    ),
                });
            }
            Ok(format!("{value:.0}"))
        }
        _ => Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Expected .OPTIONS METHOD value, found {:?}",
                stream.peek().kind
            ),
        }),
    }
}

/// Parse `.OPTIONS DAMPING=<strategy>`.
///
/// Named strategies only. A damping strategy has no ordering, so a numeric
/// selector would be an index into a list the deck cannot see; an unreadable
/// name is rejected rather than defaulted, because silently running an
/// undamped Newton on a deck that asked for limiting is the failure this
/// option exists to prevent.
fn parse_damping_option(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<DampingStrategy, ParseError> {
    let name = match &stream.peek().kind {
        TokenKind::Ident(name) => {
            let name = name.clone();
            stream.advance();
            name
        }
        _ => {
            let value = expect_value(stream, line_num, params)?;
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(".OPTIONS DAMPING expects a strategy name, found {value}"),
            });
        }
    };

    match name.to_ascii_uppercase().replace('_', "").as_str() {
        "NONE" | "OFF" => Ok(DampingStrategy::None),
        "LINESEARCH" => Ok(DampingStrategy::LineSearch),
        "VOLTAGELIMITING" | "LIMIT" | "LIMITING" => Ok(DampingStrategy::VoltageLimiting),
        "BANKROSE" => Ok(DampingStrategy::BankRose),
        "COMBINED" => Ok(DampingStrategy::Combined),
        _ => Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "unsupported .OPTIONS DAMPING strategy '{name}'; expected NONE, LINESEARCH, VOLTAGELIMITING, BANKROSE, or COMBINED"
            ),
        }),
    }
}

/// Parse `.OPTIONS SOLVER=<backend>`.
///
/// The three names are the ones `RSPICE_SOLVER` already takes, so a deck and
/// an environment override read the same. `AUTO` is an explicit request for
/// measured routing, which is not the same as saying nothing: an unstated
/// backend also lets the dialect profile force a choice.
fn parse_matrix_solver_option(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<RealSolverBackend, ParseError> {
    let TokenKind::Ident(name) = &stream.peek().kind else {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                ".OPTIONS SOLVER expects a backend name, found {:?}",
                stream.peek().kind
            ),
        });
    };
    let name = name.clone();
    stream.advance();

    match name.to_ascii_uppercase().as_str() {
        "AUTO" => Ok(RealSolverBackend::Auto),
        "KLU" => Ok(RealSolverBackend::Klu),
        "FAER" => Ok(RealSolverBackend::Faer),
        _ => Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "unsupported .OPTIONS SOLVER backend '{name}'; expected AUTO, KLU, or FAER"
            ),
        }),
    }
}

fn parse_xyce_hb_preconditioner_option(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<crate::netlist::XyceHbPreconditioner, ParseError> {
    let TokenKind::Ident(name) = &stream.peek().kind else {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                ".OPTIONS LINSOL-HB PREC_TYPE expects a preconditioner name, found {:?}",
                stream.peek().kind
            ),
        });
    };
    let name = name.clone();
    stream.advance();

    match name.to_ascii_uppercase().as_str() {
        "BLOCK_JACOBI" => Ok(crate::netlist::XyceHbPreconditioner::BlockJacobi),
        _ => Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "unsupported .OPTIONS LINSOL-HB PREC_TYPE '{name}'; expected BLOCK_JACOBI"
            ),
        }),
    }
}

fn parse_nonlinear_continuation_option(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<crate::config::NonlinearContinuationMode, ParseError> {
    use crate::config::NonlinearContinuationMode as Mode;

    if let TokenKind::Ident(name) = &stream.peek().kind {
        let name = name.clone();
        let upper = name.to_ascii_uppercase();
        stream.advance();
        let mode = if upper.starts_with("SOURCESTEP2") {
            Some(Mode::SequentialSourceStep)
        } else if upper.starts_with("SOURCESTEP") {
            Some(Mode::SimultaneousSourceStep)
        } else if upper.starts_with("STAN") {
            Some(Mode::Standard)
        } else if upper.starts_with("NAT") {
            Some(Mode::Natural)
        } else if upper.starts_with("MOS") {
            Some(Mode::Mosfet)
        } else if upper.starts_with("GMIN") {
            Some(Mode::Gmin)
        } else if upper.starts_with("PSEUDO") {
            Some(Mode::PseudoTransient)
        } else {
            None
        };
        return mode.ok_or_else(|| ParseError::Syntax {
            line: line_num,
            message: format!(
                "unsupported .OPTIONS NONLIN CONTINUATION mode '{name}'; expected STANDARD, NATURAL, MOS, GMIN, PSEUDO, SOURCESTEP, or SOURCESTEP2"
            ),
        });
    }

    let value = expect_value(stream, line_num, params)?;
    if !value.is_finite() || value.fract() != 0.0 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                ".OPTIONS NONLIN CONTINUATION expects an integer selector or mode name, found {value}"
            ),
        });
    }
    let selector = value as i64;
    Mode::from_xyce_selector(selector).ok_or_else(|| ParseError::Syntax {
        line: line_num,
        message: format!(
            "unsupported .OPTIONS NONLIN CONTINUATION selector {selector}; expected 0, 1, 2, 3, 9, 34, or 35"
        ),
    })
}

/// `.ESAVE all | none | <net> ...`: ngspice's `esave` (`evtprint.c`
/// `EVTsave`). A lone `all` or `none` selects every or no event net; any other
/// list, including one that merely contains those words, names the nets to
/// record. Each command replaces the previous selection. Nets keep the deck's
/// spelling, so a refusal quotes the name the author wrote; every consumer
/// matches them without regard to case.
fn parse_esave_command(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<crate::netlist::EventSaveSelection, ParseError> {
    let mut nets = Vec::new();
    while !stream.is_eof() {
        skip_commas(stream);
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }
        let net = expect_authored_node(stream, line_num)?;
        if !net.trim().is_empty() {
            nets.push(net);
        }
    }
    match nets.as_slice() {
        [] => Err(ParseError::Syntax {
            line: line_num,
            message: ".ESAVE requires all, none, or at least one event net".to_string(),
        }),
        [word] if word.eq_ignore_ascii_case("all") => Ok(crate::netlist::EventSaveSelection::All),
        [word] if word.eq_ignore_ascii_case("none") => Ok(crate::netlist::EventSaveSelection::None),
        _ => Ok(crate::netlist::EventSaveSelection::Nets(nets)),
    }
}

pub(super) fn parse_global_command(
    stream: &mut TokenStream,
    line_num: usize,
    global_nodes: &mut std::collections::HashSet<String>,
) -> Result<(), ParseError> {
    let mut parsed_any = false;

    while !stream.is_eof() {
        skip_commas(stream);
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }

        let node = expect_node(stream, line_num)?;
        if !node.trim().is_empty() {
            global_nodes.insert(node.to_ascii_uppercase());
            parsed_any = true;
        }
    }

    if parsed_any {
        Ok(())
    } else {
        Err(ParseError::Syntax {
            line: line_num,
            message: ".GLOBAL requires at least one node name".to_string(),
        })
    }
}

/// Package-specific option sets that do not implement the global seed key
/// must not select a random stream during the early scan either.
pub(super) fn seed_option_applies_to_package(package: Option<&str>) -> bool {
    !matches!(
        package,
        Some(
            "FFT"
                | "MEASURE"
                | "HBINT"
                | "LINSOL-HB"
                | "NONLIN-HB"
                | "NONLIN-TRAN"
                | "NONLIN-TRANSIENT"
                | "TIMEINT"
                | "RESTART"
        )
    )
}

pub(super) fn expect_seed_option(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<u64, ParseError> {
    expect_u64_literal(stream, line_num, "SEED")
}

pub(super) fn parse_positive_real_option(
    name: &str,
    value: Value,
    line_num: usize,
) -> Result<Value, ParseError> {
    if !value.is_finite() || value <= 0.0 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("{} must be a positive finite number, found {}", name, value),
        });
    }
    Ok(value)
}

pub(super) fn parse_non_negative_real_option(
    name: &str,
    value: Value,
    line_num: usize,
) -> Result<Value, ParseError> {
    if !value.is_finite() || value < 0.0 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "{} must be a finite non-negative number, found {}",
                name, value
            ),
        });
    }
    Ok(value)
}

/// Validate ngspice's modified-trapezoidal interpolation domain.
///
/// Ngspice defines `xmu=0` as backward Euler and `xmu=0.5` as the ordinary
/// trapezoidal corrector (`nicomcof.c`). Values between those endpoints add
/// damping; values outside them cease to be an interpolation between the two
/// integration formulas and are rejected before coefficient construction.
pub(super) fn parse_xmu_option(value: Value, line_num: usize) -> Result<Value, ParseError> {
    if !value.is_finite() || !(0.0..=0.5).contains(&value) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("XMU must be finite and within [0, 0.5], found {value}"),
        });
    }
    Ok(value)
}

pub(super) fn parse_celsius_option(
    name: &str,
    value: Value,
    line_num: usize,
) -> Result<Value, ParseError> {
    if !value.is_finite() || value <= -273.15 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "{} must be finite and above absolute zero, found {} C",
                name, value
            ),
        });
    }
    Ok(value)
}

pub(super) fn parse_usize_option(
    name: &str,
    value: Value,
    line_num: usize,
) -> Result<usize, ParseError> {
    if !value.is_finite() || value < 0.0 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("{} must be a non-negative integer, found {}", name, value),
        });
    }

    let rounded = value.round();
    if (value - rounded).abs() > 1e-9 || rounded > usize::MAX as Value {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("{} must be a non-negative integer, found {}", name, value),
        });
    }

    Ok(rounded as usize)
}

fn parse_i64_option(name: &str, value: Value, line_num: usize) -> Result<i64, ParseError> {
    if !value.is_finite() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("{name} must be an integer, found {value}"),
        });
    }
    let rounded = value.round();
    if (value - rounded).abs() > 1e-9 || rounded < i64::MIN as Value || rounded > i64::MAX as Value
    {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("{name} must be an integer, found {value}"),
        });
    }
    Ok(rounded as i64)
}

fn parse_xyce_i32_option(name: &str, value: Value, line_num: usize) -> Result<i32, ParseError> {
    let truncated = value.trunc();
    if !value.is_finite() || truncated < i32::MIN as Value || truncated > i32::MAX as Value {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("{name} must be a finite signed 32-bit numeric value, found {value}"),
        });
    }
    // Xyce 7.10's `Param::getImmutableValue<int>()` converts real numeric
    // option values with `static_cast<int>`, truncating toward zero.
    Ok(truncated as i32)
}

fn parse_digital_delay_type_option(value: Value, line_num: usize) -> Result<i64, ParseError> {
    if !value.is_finite() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("DIGITAL_DELAY_TYPE must be an integer from 0 to 3, found {value}"),
        });
    }

    let rounded = value.round();
    if value != rounded || !(0.0..=3.0).contains(&rounded) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("DIGITAL_DELAY_TYPE must be an integer from 0 to 3, found {value}"),
        });
    }

    Ok(rounded as i64)
}

fn parse_transient_lte_reference_option(
    value: Value,
    line_num: usize,
) -> Result<TransientLteReference, ParseError> {
    if !value.is_finite() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("NEWLTE must be an integer from 0 to 3, found {value}"),
        });
    }

    let rounded = value.round();
    if value != rounded || !(0.0..=3.0).contains(&rounded) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("NEWLTE must be an integer from 0 to 3, found {value}"),
        });
    }

    TransientLteReference::from_xyce_selector(rounded as u8).ok_or_else(|| ParseError::Syntax {
        line: line_num,
        message: format!("NEWLTE must be an integer from 0 to 3, found {value}"),
    })
}

fn parse_new_breakpoint_stepping_option(value: Value, line_num: usize) -> Result<bool, ParseError> {
    if value == 0.0 {
        Ok(false)
    } else if value == 1.0 {
        Ok(true)
    } else {
        Err(ParseError::Syntax {
            line: line_num,
            message: format!("NEWBPSTEPPING must be the integer 0 or 1, found {value}"),
        })
    }
}

fn parse_binary_integer_option(
    name: &str,
    value: Value,
    line_num: usize,
) -> Result<bool, ParseError> {
    if value == 0.0 {
        Ok(false)
    } else if value == 1.0 {
        Ok(true)
    } else {
        Err(ParseError::Syntax {
            line: line_num,
            message: format!("{name} must be the integer 0 or 1, found {value}"),
        })
    }
}

fn parse_exact_xyce_nonnegative_integer_option(
    name: &str,
    value: Value,
    line_num: usize,
) -> Result<usize, ParseError> {
    if !value.is_finite() || value < 0.0 || value != value.round() || value > i32::MAX as Value {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("{name} must be a non-negative signed 32-bit integer, found {value}"),
        });
    }
    Ok(value as usize)
}

fn parse_xyce_transient_order_option(
    name: &str,
    value: Value,
    line_num: usize,
) -> Result<u8, ParseError> {
    if value == 1.0 || value == 2.0 {
        Ok(value as u8)
    } else {
        Err(ParseError::Syntax {
            line: line_num,
            message: format!("{name} must be the integer 1 or 2, found {value}"),
        })
    }
}

fn parse_transient_error_control_option(
    value: Value,
    line_num: usize,
) -> Result<TransientErrorControl, ParseError> {
    if !value.is_finite() || value != value.round() || !(0.0..=1.0).contains(&value) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("ERROPTION must be the integer 0 or 1, found {value}"),
        });
    }
    TransientErrorControl::from_xyce_selector(value as usize).ok_or_else(|| ParseError::Syntax {
        line: line_num,
        message: format!("ERROPTION must be the integer 0 or 1, found {value}"),
    })
}

fn parse_fft_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    diagnostics: &mut Vec<ParseDiagnostic>,
) -> Result<FftAnalysis, ParseError> {
    let output = match &stream.peek().kind {
        TokenKind::Expression(expression) => {
            let expression = expression.clone();
            if expression.trim().is_empty() {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: ".FFT output expression must not be empty".to_string(),
                });
            }
            stream.advance();
            FftOutput::Expression(expression)
        }
        TokenKind::AtSign => FftOutput::Probe(parse_meas_signal(stream, line_num, params)?),
        TokenKind::Ident(_) => {
            let probe = parse_meas_signal(stream, line_num, params)?;
            validate_fft_probe(&probe, line_num)?;
            FftOutput::Probe(probe)
        }
        _ => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: ".FFT requires one output probe or braced expression as its first field"
                    .to_string(),
            });
        }
    };

    let mut start = None;
    let mut stop = None;
    let mut points = FftAnalysis::DEFAULT_POINTS;
    let mut format = None;
    let mut window = FftWindow::Rectangular;
    let mut window_name = "RECT".to_string();
    let mut alpha = FftAnalysis::DEFAULT_ALPHA;
    let mut fundamental_frequency = None;
    let mut minimum_frequency = None;
    let mut maximum_frequency = None;

    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        let key = expect_ident(stream, line_num)?.to_ascii_uppercase();
        if !stream.consume(&TokenKind::Equals) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(".FFT qualifier {key} requires '=' and a value"),
            });
        }
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(".FFT qualifier {key} is missing its value"),
            });
        }

        match key.as_str() {
            "START" | "FROM" => {
                let value = expect_value(stream, line_num, params)?;
                if !value.is_finite() {
                    return Err(non_finite_fft_qualifier(line_num, &key, value));
                }
                if value < 0.0 {
                    diagnostics.push(ParseDiagnostic::warning(
                        line_num,
                        "fft-start-clamped",
                        format!(".FFT {key}={value} is negative; Xyce resets the start time to 0"),
                    ));
                    start = Some(0.0);
                } else {
                    start = Some(value);
                }
            }
            "STOP" | "TO" => {
                let value = expect_value(stream, line_num, params)?;
                if !value.is_finite() {
                    return Err(non_finite_fft_qualifier(line_num, &key, value));
                }
                stop = Some(value);
            }
            "NP" => {
                let value = expect_value(stream, line_num, params)?;
                points = normalize_fft_points(value, line_num, diagnostics)?;
            }
            "FORMAT" => {
                let value = expect_ident(stream, line_num)?.to_ascii_uppercase();
                format = Some(match value.as_str() {
                    "NORM" => FftFormat::Normalized,
                    "UNORM" => FftFormat::Unnormalized,
                    _ => {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: format!("Invalid FORMAT type {value} on .FFT line"),
                        });
                    }
                });
            }
            "WINDOW" => {
                window_name = expect_ident(stream, line_num)?.to_ascii_uppercase();
                window = parse_fft_window(&window_name, line_num)?;
            }
            "ALFA" => {
                let value = expect_value(stream, line_num, params)?;
                if !value.is_finite() {
                    return Err(non_finite_fft_qualifier(line_num, &key, value));
                }
                alpha = value.clamp(1.0, 20.0);
            }
            "FREQ" | "FMIN" | "FMAX" => {
                let value = expect_value(stream, line_num, params)?;
                if !value.is_finite() {
                    return Err(non_finite_fft_qualifier(line_num, &key, value));
                }
                match key.as_str() {
                    "FREQ" => fundamental_frequency = Some(value),
                    "FMIN" => minimum_frequency = Some(value),
                    "FMAX" => maximum_frequency = Some(value),
                    _ => unreachable!(),
                }
            }
            _ => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!("Unknown .FFT qualifier {key}"),
                });
            }
        }
    }

    Ok(FftAnalysis {
        output,
        start,
        stop,
        points,
        format,
        window,
        window_name,
        alpha,
        fundamental_frequency,
        minimum_frequency,
        maximum_frequency,
    })
}

fn validate_fft_probe(probe: &str, line_num: usize) -> Result<(), ParseError> {
    let Some((operator, arguments)) = probe.split_once('(') else {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(".FFT output '{probe}' must be a parenthesized probe"),
        });
    };
    if !arguments.ends_with(')') {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(".FFT output '{probe}' has an unterminated argument list"),
        });
    }
    let operator = operator.to_ascii_uppercase();
    let allowed = crate::netlist::is_current_output_accessor(&operator)
        || matches!(operator.as_str(), "V" | "P" | "W" | "N");
    if !allowed {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("Unsupported .FFT output operator {operator}"),
        });
    }
    let argument_count = arguments[..arguments.len() - 1].split(',').count();
    if operator == "V" && !(1..=2).contains(&argument_count) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: ".FFT voltage output requires one or two nodes".to_string(),
        });
    }
    if operator != "V" && argument_count != 1 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(".FFT {operator} output requires exactly one argument"),
        });
    }
    Ok(())
}

fn normalize_fft_points(
    value: Value,
    line_num: usize,
    diagnostics: &mut Vec<ParseDiagnostic>,
) -> Result<usize, ParseError> {
    if !value.is_finite() || value > usize::MAX as Value {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(".FFT NP must be a finite representable integer, found {value}"),
        });
    }
    let truncated = value.trunc();
    if truncated <= 0.0 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: ".FFT NP must be positive".to_string(),
        });
    }
    let raw = truncated as usize;
    let normalized = if raw < 4 {
        4
    } else if raw.is_power_of_two() {
        raw
    } else {
        let lower = 1usize << raw.ilog2();
        let upper = lower.checked_mul(2).unwrap_or(lower);
        if raw - lower >= (upper - lower) / 2 {
            upper
        } else {
            lower
        }
    };
    if normalized != raw {
        diagnostics.push(ParseDiagnostic::warning(
            line_num,
            "fft-points-normalized",
            format!(".FFT NP={value} is normalized to {normalized} samples"),
        ));
    }
    Ok(normalized)
}

fn parse_fft_window(value: &str, line_num: usize) -> Result<FftWindow, ParseError> {
    let window = match value {
        "RECT" | "RECTANGULAR" => FftWindow::Rectangular,
        "BART" | "BARTLETT" => FftWindow::Bartlett,
        "BARTLETTHANN" => FftWindow::BartlettHann,
        "HAMM" | "HAMMING" => FftWindow::Hamming,
        "HANN" | "HANNING" => FftWindow::Hann,
        "BLACK" => FftWindow::Blackman67Db,
        "BLACKMAN" => FftWindow::Blackman,
        "HARRIS" | "BLACKMANHARRIS" => FftWindow::BlackmanHarris,
        "NUTTALL" => FftWindow::Nuttall,
        "HALFCYCLESINE" => FftWindow::HalfCycleSine,
        "HALFCYCLESINE3" => FftWindow::HalfCycleSine3,
        "HALFCYCLESINE6" => FftWindow::HalfCycleSine6,
        "COSINE2" => FftWindow::Cosine2,
        "COSINE4" => FftWindow::Cosine4,
        "GAUSS" | "GAUSSIAN" => FftWindow::Gaussian,
        "KAISER" => FftWindow::Kaiser,
        _ => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Invalid WINDOW type {value} on .FFT line"),
            });
        }
    };
    Ok(window)
}

fn non_finite_fft_qualifier(line_num: usize, key: &str, value: Value) -> ParseError {
    ParseError::Syntax {
        line: line_num,
        message: format!(".FFT {key} must be finite, found {value}"),
    }
}

/// LTspice and QSPICE take a part of a complex probe in an AC measurement
/// with a function: `mag(V(out))`, `Phase(V(out))`, `db(V(out))`,
/// `re(I(R1))`, `im(I(R1))`. Each is the probe's projection accessor
/// (`VM(out)`, `VP(out)`, `VDB(out)`, `IR(R1)`, `II(R1)`), which reads the
/// complex point rather than its real part.
fn project_complex_measure_functions(stream: &mut TokenStream) {
    let base = stream.checkpoint();
    let mut index = 0;
    loop {
        let tokens = stream.remaining_line_tokens();
        if index + 4 >= tokens.len() {
            break;
        }
        let projection = match &tokens[index].kind {
            TokenKind::Ident(function) => match function.to_ascii_uppercase().as_str() {
                "MAG" => Some("M"),
                "PH" | "PHASE" => Some("P"),
                "DB" => Some("DB"),
                "RE" | "REAL" => Some("R"),
                "IM" | "IMAG" => Some("I"),
                _ => None,
            },
            _ => None,
        };
        let probe = match &tokens[index + 2].kind {
            TokenKind::Ident(probe)
                if probe.eq_ignore_ascii_case("V") || probe.eq_ignore_ascii_case("I") =>
            {
                Some(probe.to_ascii_uppercase())
            }
            _ => None,
        };
        let close = tokens[index + 4..]
            .iter()
            .position(|token| matches!(token.kind, TokenKind::LParen | TokenKind::RParen))
            .map(|offset| index + 4 + offset);
        let (Some(projection), Some(probe), Some(close)) = (projection, probe, close) else {
            index += 1;
            continue;
        };
        if !matches!(tokens[index + 1].kind, TokenKind::LParen)
            || !matches!(tokens[index + 3].kind, TokenKind::LParen)
            || !matches!(tokens[close].kind, TokenKind::RParen)
            || !matches!(
                tokens.get(close + 1).map(|token| &token.kind),
                Some(TokenKind::RParen)
            )
        {
            index += 1;
            continue;
        }
        let accessor = format!("{probe}{projection}");
        let span = crate::netlist::lexer::Span {
            end: tokens[index + 3].span.start,
            ..tokens[index].span
        };
        let token = crate::netlist::lexer::Token {
            kind: TokenKind::Ident(accessor.clone()),
            span,
            lexeme: accessor,
        };
        stream.remove_tokens(base + close + 1..base + close + 2);
        stream.remove_tokens(base + index + 1..base + index + 3);
        stream.replace_token(base + index, token);
        index += 1;
    }
}

pub(super) fn parse_meas_signal(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<String, ParseError> {
    if let Some(expression) = parse_measure_expression_operand(stream, line_num)? {
        return Ok(expression);
    }
    if params.expression_dialect() != crate::config::ExpressionDialect::Xyce
        && let Some(expression) = take_unbraced_measure_expression(stream, false)
    {
        return Ok(format!("{{{expression}}}"));
    }
    if stream.consume(&TokenKind::AtSign) {
        // `expect_ident` accepts contiguous DEV-name punctuation, including
        // brackets. Here brackets delimit the parameter and must stay tokens.
        let TokenKind::Ident(device) = stream.advance().kind.clone() else {
            return Err(ParseError::Syntax {
                line: line_num,
                message: "Expected device name after '@' in .MEAS operand".into(),
            });
        };
        if !stream.consume(&TokenKind::LBracket) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: "Expected '[' after device name in .MEAS operand".into(),
            });
        }
        let TokenKind::Ident(parameter) = stream.advance().kind.clone() else {
            return Err(ParseError::Syntax {
                line: line_num,
                message: "Expected device parameter inside .MEAS brackets".into(),
            });
        };
        if !stream.consume(&TokenKind::RBracket) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: "Expected ']' after device parameter in .MEAS operand".into(),
            });
        }
        return Ok(format!("@{device}[{parameter}]"));
    }
    if params.expression_dialect() == crate::config::ExpressionDialect::Xyce
        && let TokenKind::Ident(name) = &stream.peek().kind
        && matches!(stream.peek_n(1).kind, TokenKind::LParen)
        && is_xyce_measure_output_operator(name)
    {
        return parse_xyce_measure_raw_output_operator(stream, line_num);
    }
    let mut signal = expect_ident(stream, line_num)?;

    if stream.consume(&TokenKind::LParen) {
        let mut args = Vec::new();
        loop {
            let arg = match &stream.peek().kind {
                TokenKind::Ident(s) => s.clone(),
                TokenKind::Number(_) => stream.peek().lexeme.clone(),
                _ => {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: "Expected identifier or number inside signal parentheses"
                            .to_string(),
                    });
                }
            };
            stream.advance();
            args.push(arg);

            if stream.consume(&TokenKind::Comma) {
                continue;
            }
            // The generic dialect also accepts Xyce's established
            // `I(YDEVICE BRANCH)` branch-current spelling, but it retains the
            // generic parser's canonical identifier casing. Raw authored
            // lexemes remain an explicitly Xyce-dialect behavior above.
            if signal == "I"
                && args.len() == 1
                && matches!(&stream.peek().kind, TokenKind::Ident(part) if part == "BRANCH")
            {
                args[0].push(' ');
                args[0].push_str("BRANCH");
                stream.advance();
            }
            if stream.consume(&TokenKind::RParen) {
                break;
            }
            return Err(ParseError::Syntax {
                line: line_num,
                message: "Expected closing parenthesis for signal".to_string(),
            });
        }

        signal = format!("{}({})", signal, args.join(","));
    }

    Ok(signal)
}

fn measure_expression_operand_ahead(stream: &TokenStream) -> bool {
    matches!(
        stream.peek().kind,
        TokenKind::Expression(_) | TokenKind::StringLit(_)
    ) || matches!(&stream.peek().kind, TokenKind::Ident(name) if name.eq_ignore_ascii_case("PAR"))
        && matches!(stream.peek_n(1).kind, TokenKind::LParen)
        || matches!(stream.peek().kind, TokenKind::LParen)
            && matches!(
                stream.peek_n(1).kind,
                TokenKind::Expression(_) | TokenKind::StringLit(_)
            )
}

/// LTspice writes a `.MEAS` operand or PARAM expression without braces:
/// `MAX v(1)*2`, `FIND -i(v1)*2 AT=2`, `PARAM p * 2`. The field runs, spaces
/// included, to the next option (`NAME=`, or a bare AT, WHEN, TRIG or TARG),
/// a top-level comma or `=`, or the end of the card. A waveform operand
/// (`any_field` false) is an expression only when it has a top-level
/// operator or opens with a sign, a parenthesis or a number, so a lone signal
/// keeps its own parser and a `-` joining name fragments (`n-1`) stays part
/// of a name, as an identifier reads it. Consumes and returns the field's
/// source text; a field holding a braced, quoted or `@dev[param]` part is
/// left to the other operand parsers.
fn take_unbraced_measure_expression(stream: &mut TokenStream, any_field: bool) -> Option<String> {
    let token_text = |token: &crate::netlist::lexer::Token| match &token.kind {
        TokenKind::Ident(name) if token.lexeme.is_empty() => name.clone(),
        TokenKind::Ident(_) | TokenKind::Number(_) if !token.lexeme.is_empty() => {
            token.lexeme.clone()
        }
        kind => kind.to_string(),
    };
    let name_fragment =
        |kind: &TokenKind| matches!(kind, TokenKind::Ident(_) | TokenKind::Number(_));
    let mut text = String::new();
    let mut count = 0usize;
    let mut depth = 0usize;
    let mut operator = false;
    let mut previous_end = None;
    loop {
        let token = stream.peek_n(count);
        let spaced = previous_end.is_some_and(|end| token.span.start != end);
        match &token.kind {
            TokenKind::Newline | TokenKind::Eof => break,
            TokenKind::Expression(_)
            | TokenKind::StringLit(_)
            | TokenKind::AtSign
            | TokenKind::LBracket
            | TokenKind::RBracket => return None,
            TokenKind::Comma | TokenKind::Equals if depth == 0 => break,
            TokenKind::Ident(name)
                if depth == 0
                    && spaced
                    && (matches!(stream.peek_n(count + 1).kind, TokenKind::Equals)
                        || ["AT", "WHEN", "TRIG", "TARG"]
                            .iter()
                            .any(|keyword| name.eq_ignore_ascii_case(keyword))) =>
            {
                break;
            }
            TokenKind::LParen => depth += 1,
            TokenKind::RParen => depth = depth.checked_sub(1)?,
            TokenKind::Star | TokenKind::Slash | TokenKind::Plus | TokenKind::Other('^')
                if depth == 0 =>
            {
                operator = true;
            }
            TokenKind::Minus if depth == 0 => {
                let next = stream.peek_n(count + 1);
                let joins_name = count > 0
                    && !spaced
                    && name_fragment(&stream.peek_n(count - 1).kind)
                    && next.span.start == token.span.end
                    && name_fragment(&next.kind);
                operator |= !joins_name;
            }
            _ => {}
        }
        if spaced {
            text.push(' ');
        }
        text.push_str(&token_text(token));
        previous_end = Some(token.span.end);
        count += 1;
    }
    let opens_expression = matches!(
        stream.peek().kind,
        TokenKind::Minus | TokenKind::Plus | TokenKind::LParen | TokenKind::Number(_)
    );
    if count == 0 || depth != 0 || !(any_field || operator || opens_expression) {
        return None;
    }
    for _ in 0..count {
        stream.advance();
    }
    Some(text)
}

fn parse_measure_expression_operand(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<Option<String>, ParseError> {
    if !measure_expression_operand_ahead(stream) {
        return Ok(None);
    }
    let wrapped =
        matches!(&stream.peek().kind, TokenKind::Ident(name) if name.eq_ignore_ascii_case("PAR"));
    let parenthesized = wrapped || matches!(stream.peek().kind, TokenKind::LParen);
    if wrapped {
        stream.advance();
    }
    if parenthesized {
        stream.advance();
    }
    let expression = match &stream.peek().kind {
        TokenKind::Expression(expression) | TokenKind::StringLit(expression) => {
            let expression = expression.clone();
            stream.advance();
            expression
        }
        _ => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: "Expected quoted or braced expression in .MEAS operand".to_string(),
            });
        }
    };
    if parenthesized && !stream.consume(&TokenKind::RParen) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "Expected ')' after .MEAS expression operand".to_string(),
        });
    }
    Ok(Some(format!("{{{expression}}}")))
}

/// Parse .MEAS/.MEASURE statement
/// Syntax:
///   .MEAS TRAN name TYPE signal [FROM=x TO=y]
///   .MEAS TRAN name FIND signal AT[=]time
///   .MEAS TRAN name FIND signal WHEN ref_signal=value
///   .MEAS TRAN name TRIG signal VAL=x [RISE=n|FALL=n|CROSS=n] [TD=x]
///                     TARG signal VAL=x [RISE=n|FALL=n|CROSS=n] [TD=x]
/// Examples:
///   .MEAS TRAN vmax MAX V(out)
///   .MEAS TRAN vavg AVG V(out) FROM=0 TO=1m
///   .MEAS TRAN vout FIND V(out) AT=1u
///   .MEAS AC vimag FIND VI(out) AT 10k
///   .MEAS TRAN delay TRIG V(in) VAL=0.5 RISE=1 TARG V(out) VAL=0.5 RISE=1
pub(super) fn parse_meas_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<crate::netlist::measure::MeasureStatement, ParseError> {
    use crate::netlist::measure::{MeasureStatement, MeasureType};

    // Parse analysis type (TRAN, AC, DC)
    let parsed_analysis = expect_ident(stream, line_num)?.to_ascii_uppercase();
    let analysis = if parsed_analysis == "TR" {
        "TRAN".to_string()
    } else if parsed_analysis.ends_with("_CONT")
        && !matches!(
            parsed_analysis.as_str(),
            "TRAN_CONT" | "AC_CONT" | "DC_CONT" | "NOISE_CONT"
        )
    {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Unknown continuous .MEASURE mode '{parsed_analysis}'; expected TRAN_CONT, AC_CONT, DC_CONT, or NOISE_CONT"
            ),
        });
    } else {
        parsed_analysis
    };

    // Xyce treats the measurement name as one whitespace-delimited source
    // field.  It may therefore contain punctuation (for example
    // `CONSTANT-AT`) that the general lexer correctly emits as multiple
    // adjacent tokens.
    let name = parse_measure_name(stream, line_num)?;
    if matches!(analysis.as_str(), "AC" | "AC_CONT" | "NOISE" | "NOISE_CONT") {
        project_complex_measure_functions(stream);
    }

    // Parse measurement type keyword
    let measure_type_str = expect_ident(stream, line_num)?;
    let measure_type_key = measure_type_str.to_ascii_uppercase();
    if analysis.ends_with("_CONT")
        && !matches!(
            measure_type_key.as_str(),
            "DERIV" | "DERIVATIVE" | "FIND" | "TRIG" | "TARG" | "WHEN"
        )
    {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Illegal .MEASURE type '{measure_type_str}' for {analysis}; expected DERIV, DERIVATIVE, FIND, TRIG, TARG, or WHEN"
            ),
        });
    }
    let statement_options = scan_meas_statement_options(stream, line_num, params)?;

    // Create the measurement type based on keyword
    let measure_type = match measure_type_key.as_str() {
        "TRIG" => {
            let mut window = (None, None);
            let trig = parse_meas_delay_spec(stream, line_num, params, "TRIG", true, &mut window)?;
            let targ_keyword = expect_ident(stream, line_num)?;
            if !targ_keyword.eq_ignore_ascii_case("TARG") {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "Expected TARG after .MEAS TRIG specification, found '{}'",
                        targ_keyword
                    ),
                });
            }
            let targ = parse_meas_delay_spec(stream, line_num, params, "TARG", false, &mut window)?;
            let (from, to) = window;
            MeasureType::Delay {
                trig,
                targ,
                from,
                to,
                minval: statement_options.minval,
            }
        }
        "PARAM" | "EQN" => {
            // .MEAS <an> name PARAM='expr' — an expression over previously
            // evaluated measurement results.
            let _optional_equals = stream.consume(&TokenKind::Equals);
            let expression = match &stream.peek().kind {
                TokenKind::Expression(expr) => {
                    let expr = expr.clone();
                    stream.advance();
                    crate::netlist::measure::MeasureExpression::expression(expr)
                }
                TokenKind::StringLit(expr) => {
                    let expr = expr.clone();
                    stream.advance();
                    crate::netlist::measure::MeasureExpression::expression(expr)
                }
                TokenKind::Number(_) | TokenKind::Plus | TokenKind::Minus
                    if (measure_type_key == "EQN"
                        || params.expression_dialect()
                            == crate::config::ExpressionDialect::Xyce)
                        && measure_equation_literal_ends_at_next_token(stream) =>
                {
                    let mut literal = String::new();
                    if matches!(stream.peek().kind, TokenKind::Plus | TokenKind::Minus) {
                        literal.push_str(&stream.peek().lexeme);
                        stream.advance();
                    }
                    literal.push_str(&stream.peek().lexeme);
                    stream.advance();
                    crate::netlist::measure::MeasureExpression::expression(literal)
                }
                TokenKind::Ident(operator)
                    if (measure_type_key == "EQN"
                        || params.expression_dialect()
                            == crate::config::ExpressionDialect::Xyce)
                        && matches!(stream.peek_n(1).kind, TokenKind::LParen)
                        && is_xyce_measure_output_operator(operator) =>
                {
                    let operator = parse_xyce_measure_raw_output_operator(stream, line_num)?;
                    // Xyce extracts the first output operator as the EQN/PARAM
                    // operand.  Adjacent arithmetic fields are not folded into
                    // an implicit expression; authored arithmetic requires
                    // braces or quotes.
                    discard_xyce_measure_output_operator_arithmetic(stream);
                    if is_xyce_measure_raw_output_operator(
                        operator.split_once('(').map_or("", |(name, _)| name),
                    ) {
                        crate::netlist::measure::MeasureExpression::raw_output_operator(operator)
                    } else {
                        // RF accessors are the source-level exception: MeasureBase
                        // rebuilds them as an ExpressionOp even without braces.
                        crate::netlist::measure::MeasureExpression::expression(operator)
                    }
                }
                TokenKind::Ident(name)
                    if (measure_type_key == "EQN"
                        || params.expression_dialect()
                            == crate::config::ExpressionDialect::Xyce)
                        && measure_equation_raw_reference_ends_at_next_token(stream) =>
                {
                    let name = name.clone();
                    stream.advance();
                    crate::netlist::measure::MeasureExpression::raw_reference(name)
                }
                _ if measure_type_key == "EQN"
                    || params.expression_dialect() == crate::config::ExpressionDialect::Xyce =>
                {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: ".MEAS EQN/PARAM arithmetic expressions must be braced or quoted"
                            .to_string(),
                    });
                }
                other => {
                    let other = other.clone();
                    let Some(expression) = take_unbraced_measure_expression(stream, true) else {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: format!(
                                ".MEAS PARAM expects an expression, found {:?}",
                                other
                            ),
                        });
                    };
                    crate::netlist::measure::MeasureExpression::expression(expression)
                }
            };
            if measure_type_key == "EQN"
                || params.expression_dialect() == crate::config::ExpressionDialect::Xyce
            {
                let (from, to, td) = parse_measure_equation_options(stream, line_num, params)?;
                MeasureType::Equation {
                    expression,
                    from,
                    to,
                    td,
                }
            } else {
                // The statement-wide scan records these policies but does not
                // consume them from the original stream. PARAM has no window
                // parser to do that, so consume its common qualifiers here.
                while consume_meas_statement_qualifier(stream, line_num, params, false)?.is_some() {
                }
                MeasureType::Param { expression }
            }
        }
        "ERR" | "ERR1" | "ERR2" => {
            let measured = parse_measure_error_operand(stream, line_num, params)?;
            let comparison = parse_measure_error_operand(stream, line_num, params)?;
            let options = parse_measure_error_function_options(stream, line_num, params)?;
            MeasureType::ErrorFunction {
                measured,
                comparison,
                norm: if measure_type_key == "ERR2" {
                    crate::netlist::measure::ErrorFunctionNorm::MeanAbsolute
                } else {
                    crate::netlist::measure::ErrorFunctionNorm::RootMeanSquare
                },
                from: options.from,
                to: options.to,
                minval: options.minval,
                ymin: options.ymin,
                ymax: options.ymax,
                weight: options.weight,
            }
        }
        "ERROR" => {
            let signal = parse_meas_signal(stream, line_num, params)?;
            let options = parse_measure_file_error_options(stream, line_num, params)?;
            MeasureType::FileError {
                signal,
                file: options.file.into(),
                norm: options.norm,
                independent_column: options.independent_column,
                dependent_column: options.dependent_column,
            }
        }
        _ => {
            // Parse signal name - handle V(node), V(pos,neg), or just node
            let signal = parse_meas_signal(stream, line_num, params)?;

            match measure_type_key.as_str() {
                "AVG" => {
                    let (from, to, td) = parse_measure_range_options(stream, line_num, params)?;
                    MeasureType::Avg {
                        signal: signal.clone(),
                        from,
                        to,
                        td,
                    }
                }
                "MAX" | "MAX_AT" => {
                    let mut options =
                        parse_measure_extrema_options(stream, line_num, params, true)?;
                    if measure_type_key == "MAX_AT" {
                        options.output = crate::netlist::measure::ExtremaOutput::IndependentAxis;
                    }
                    MeasureType::Max {
                        signal: signal.clone(),
                        from: options.from,
                        to: options.to,
                        td: options.td,
                        output: options.output,
                        crossing: options.crossing,
                        rfc_level: options.rfc_level,
                    }
                }
                "MIN" | "MIN_AT" => {
                    let mut options =
                        parse_measure_extrema_options(stream, line_num, params, true)?;
                    if measure_type_key == "MIN_AT" {
                        options.output = crate::netlist::measure::ExtremaOutput::IndependentAxis;
                    }
                    MeasureType::Min {
                        signal: signal.clone(),
                        from: options.from,
                        to: options.to,
                        td: options.td,
                        output: options.output,
                        crossing: options.crossing,
                        rfc_level: options.rfc_level,
                    }
                }
                "PP" => {
                    let options = parse_measure_extrema_options(stream, line_num, params, false)?;
                    MeasureType::PeakToPeak {
                        signal: signal.clone(),
                        from: options.from,
                        to: options.to,
                        td: options.td,
                        crossing: options.crossing,
                        rfc_level: options.rfc_level,
                    }
                }
                "RMS" => {
                    let (from, to, td) = parse_measure_range_options(stream, line_num, params)?;
                    MeasureType::Rms {
                        signal: signal.clone(),
                        from,
                        to,
                        td,
                    }
                }
                "INTEG" | "INTEGRAL" => {
                    let (from, to, td) = parse_measure_range_options(stream, line_num, params)?;
                    MeasureType::Integ {
                        signal: signal.clone(),
                        from,
                        to,
                        td,
                    }
                }
                "ENOB" | "SFDR" | "SNR" | "SNDR" | "THD" => {
                    let options =
                        parse_measure_fft_options(stream, line_num, params, &measure_type_key)?;
                    MeasureType::FftMetric {
                        metric: match measure_type_key.as_str() {
                            "ENOB" => crate::netlist::measure::FftMetric::Enob,
                            "SFDR" => crate::netlist::measure::FftMetric::Sfdr,
                            "SNR" => crate::netlist::measure::FftMetric::Snr,
                            "SNDR" => crate::netlist::measure::FftMetric::Sndr,
                            _ => crate::netlist::measure::FftMetric::Thd,
                        },
                        signal: signal.clone(),
                        bin_size: options.bin_size,
                        min_freq: options.min_freq,
                        max_freq: options.max_freq,
                        harmonics: options.harmonics,
                    }
                }
                "FOUR" => {
                    let options = parse_measure_fourier_options(stream, line_num, params)?;
                    MeasureType::Fourier {
                        signal: signal.clone(),
                        at: options.at,
                        num_freq: options.num_freq,
                        grid_size: options.grid_size,
                        from: options.from,
                        to: options.to,
                        td: options.td,
                        minval: statement_options.minval,
                    }
                }
                "DUTY" | "ON_TIME" | "OFF_TIME" | "FREQ" => {
                    let options = parse_measure_on_off_options(stream, line_num, params)?;
                    MeasureType::OnOff {
                        statistic: match measure_type_key.as_str() {
                            "DUTY" => crate::netlist::measure::OnOffStatistic::Duty,
                            "ON_TIME" => crate::netlist::measure::OnOffStatistic::OnTime,
                            "OFF_TIME" => crate::netlist::measure::OnOffStatistic::OffTime,
                            _ => crate::netlist::measure::OnOffStatistic::Frequency,
                        },
                        signal: signal.clone(),
                        from: options.from,
                        to: options.to,
                        td: options.td,
                        on: options.on,
                        off: options.off,
                        minval: statement_options.minval,
                    }
                }
                "FIND" => {
                    let options = parse_point_measure_options(
                        stream,
                        line_num,
                        params,
                        "FIND",
                        measure_result_axis(&analysis),
                    )?;

                    MeasureType::Find {
                        signal: signal.clone(),
                        at: options.at,
                        when: options.when,
                        from: options.from,
                        to: options.to,
                        td: options.td,
                        minval: statement_options.minval,
                    }
                }
                "DERIV" | "DERIVATIVE" => {
                    let options = parse_point_measure_options(
                        stream,
                        line_num,
                        params,
                        "DERIV",
                        measure_result_axis(&analysis),
                    )?;

                    MeasureType::Derivative {
                        signal: signal.clone(),
                        at: options.at,
                        when: options.when,
                        from: options.from,
                        to: options.to,
                        td: options.td,
                        minval: statement_options.minval,
                    }
                }
                "WHEN" => {
                    if !stream.consume(&TokenKind::Equals) {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: "Expected '=' after left operand in .MEAS WHEN".to_string(),
                        });
                    }
                    let right = parse_measure_when_operand(stream, line_num, params)?;
                    let (from, to, td, occurrence) =
                        parse_measure_when_event_options(stream, line_num, params)?;
                    MeasureType::When {
                        condition: crate::netlist::measure::WhenCondition {
                            left: signal.clone(),
                            right,
                            occurrence,
                        },
                        from,
                        to,
                        td,
                        minval: statement_options.minval,
                    }
                }
                _ => {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!("Unknown measurement type: {}", measure_type_str),
                    });
                }
            }
        }
    };

    Ok(MeasureStatement {
        name,
        measure_type,
        analysis,
        goal: statement_options.goal,
        tolerance: statement_options.tolerance,
        default_value: statement_options.default_value,
        fail_value: statement_options.fail_value,
        print_policy: statement_options.print_policy,
    })
}

fn measure_equation_raw_reference_ends_at_next_token(stream: &TokenStream) -> bool {
    match &stream.peek_n(1).kind {
        TokenKind::Newline | TokenKind::Eof => true,
        TokenKind::Ident(name) => crate::netlist::measure::XYCE_MEASURE_QUALIFIER_KEYWORDS
            .iter()
            .chain(crate::netlist::measure::XYCE_MEASURE_TYPE_KEYWORDS)
            .any(|keyword| name.eq_ignore_ascii_case(keyword)),
        _ => false,
    }
}

fn measure_equation_literal_ends_at_next_token(stream: &TokenStream) -> bool {
    let number_offset = usize::from(matches!(
        stream.peek().kind,
        TokenKind::Plus | TokenKind::Minus
    ));
    matches!(stream.peek_n(number_offset).kind, TokenKind::Number(_))
        && match &stream.peek_n(number_offset + 1).kind {
            TokenKind::Newline | TokenKind::Eof => true,
            TokenKind::Ident(name) => crate::netlist::measure::XYCE_MEASURE_QUALIFIER_KEYWORDS
                .iter()
                .chain(crate::netlist::measure::XYCE_MEASURE_TYPE_KEYWORDS)
                .any(|keyword| name.eq_ignore_ascii_case(keyword)),
            _ => false,
        }
}

fn is_xyce_measure_output_operator(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    let first = upper.as_bytes().first().copied();
    matches!(upper.as_str(), "N" | "P" | "W")
        || crate::netlist::is_current_output_accessor(&upper)
        || (matches!(first, Some(b'V' | b'S' | b'Y' | b'Z')) && upper.len() <= 3)
        || (first == Some(b'D') && upper.len() == 3)
}

fn is_xyce_measure_raw_output_operator(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    matches!(upper.as_str(), "N" | "P" | "W" | "DNO" | "DNI")
        || crate::netlist::is_current_output_accessor(&upper)
        || matches!(upper.as_str(), "V" | "VR" | "VI" | "VM" | "VP" | "VDB")
}

fn discard_xyce_measure_output_operator_arithmetic(stream: &mut TokenStream) {
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        if matches!(&stream.peek().kind, TokenKind::Ident(name)
            if crate::netlist::measure::XYCE_MEASURE_QUALIFIER_KEYWORDS
                .iter()
                .chain(crate::netlist::measure::XYCE_MEASURE_TYPE_KEYWORDS)
                .any(|keyword| name.eq_ignore_ascii_case(keyword)))
        {
            break;
        }
        // Stop at an identifier instead of hiding a second operand or a typo.
        // Xyce's ignored tail in this grammar is the adjacent arithmetic field.
        if matches!(stream.peek().kind, TokenKind::Ident(_)) {
            break;
        }
        stream.advance();
    }
}

fn parse_xyce_measure_raw_output_operator(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<String, ParseError> {
    let name = expect_ident(stream, line_num)?;
    if !is_xyce_measure_output_operator(&name) || !stream.consume(&TokenKind::LParen) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "Expected a Xyce .MEAS output operator".to_string(),
        });
    }
    let mut arguments = Vec::new();
    loop {
        let token = stream.peek().clone();
        match token.kind {
            TokenKind::Ident(_) | TokenKind::Number(_) => {
                arguments.push(token.lexeme);
                stream.advance();
            }
            _ => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!("Invalid argument in .MEAS output operator '{name}'"),
                });
            }
        }
        if stream.consume(&TokenKind::Comma) {
            continue;
        }
        // Xyce accepts the legacy branch-current spelling I(YSOMETHING NAME)
        // and stores the two whitespace-separated tokens as one branch name.
        if crate::netlist::is_current_output_accessor(&name.to_ascii_uppercase())
            && arguments.len() == 1
            && matches!(stream.peek().kind, TokenKind::Ident(_))
        {
            let token = stream.peek().clone();
            arguments[0].push(' ');
            arguments[0].push_str(&token.lexeme);
            stream.advance();
        }
        if stream.consume(&TokenKind::RParen) {
            break;
        }
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("Expected closing parenthesis for .MEAS output operator '{name}'"),
        });
    }
    let operator = format!("{name}({})", arguments.join(","));
    validate_xyce_measure_output_operator(&operator, line_num)?;
    Ok(operator)
}

fn validate_xyce_measure_output_operator(
    operator: &str,
    line_num: usize,
) -> Result<(), ParseError> {
    let Some((name, arguments)) = operator.split_once('(').and_then(|(name, arguments)| {
        arguments
            .strip_suffix(')')
            .map(|arguments| (name, arguments))
    }) else {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("Malformed .MEAS output operator '{operator}'"),
        });
    };
    let arity = if arguments.is_empty() {
        0
    } else {
        arguments.split(',').count()
    };
    let upper = name.to_ascii_uppercase();
    let valid = match upper.as_bytes().first().copied() {
        Some(b'V') => (1..=2).contains(&arity),
        Some(b'D') => (1..=2).contains(&arity),
        Some(b'S' | b'Y' | b'Z') => arity == 2,
        Some(b'I') => arity == 1,
        _ => matches!(upper.as_str(), "N" | "P" | "W") && arity == 1,
    };
    if valid {
        Ok(())
    } else {
        Err(ParseError::Syntax {
            line: line_num,
            message: format!("Invalid argument count for .MEAS output operator '{operator}'"),
        })
    }
}

struct PointMeasureOptions {
    at: Option<Value>,
    when: Option<crate::netlist::measure::WhenCondition>,
    from: Option<Value>,
    to: Option<Value>,
    td: Option<Value>,
    occurrence_given: bool,
}

/// Read a `.MEAS` numeric field: a number, a parameter, a `{expression}`,
/// or, as ngspice also reads it, an unbraced expression written as one word
/// (`AT=t0/2`). The word ends at the first space, so a following keyword is
/// never taken into it.
fn expect_measure_value(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<Value, ParseError> {
    skip_commas(stream);
    let mut length = 1;
    let mut operator_inside = false;
    loop {
        let previous = stream.peek_n(length - 1);
        let next = stream.peek_n(length);
        if next.span.start != previous.span.end
            || next.span.line != previous.span.line
            || matches!(
                next.kind,
                TokenKind::Newline | TokenKind::Eof | TokenKind::Comma | TokenKind::Equals
            )
        {
            break;
        }
        operator_inside |= matches!(
            next.kind,
            TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash
        ) || matches!(next.kind, TokenKind::Other(ch) if param_rhs_operator_char(ch));
        length += 1;
    }
    if !operator_inside {
        return expect_value(stream, line_num, params);
    }
    let mut expression = String::new();
    for offset in 0..length {
        let token = stream.peek_n(offset);
        expression.push_str(&param_rhs_token_fragment(&token.kind, &token.lexeme));
    }
    let value = stream
        .numeric_expression(&expression, params)
        .map_err(|error| ParseError::Syntax {
            line: line_num,
            message: format!(".MEAS value '{expression}' could not be evaluated: {error}"),
        })?;
    for _ in 0..length {
        stream.advance();
    }
    Ok(value)
}

fn expect_finite_measure_td(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<Value, ParseError> {
    let value = expect_measure_value(stream, line_num, params)?;
    if !value.is_finite() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(".MEAS TD must be finite, found {value}"),
        });
    }
    Ok(value)
}

/// The independent axis an `AT` naming an earlier measurement places its
/// point on, for the analyses whose measurements evaluate after the run.
fn measure_result_axis(analysis: &str) -> Option<&'static str> {
    match analysis {
        "TRAN" => Some("TIME"),
        "AC" | "NOISE" => Some("FREQUENCY"),
        _ => None,
    }
}

/// Whether the word after `AT` names an earlier measurement rather than a
/// number or a parameter: a bare name standing alone as the operand.
fn names_measurement_result(stream: &TokenStream, params: &ParamContext) -> bool {
    let TokenKind::Ident(name) = &stream.peek().kind else {
        return false;
    };
    params.get(name).is_none()
        && crate::netlist::lexer::parse_spice_value_complete(name).is_err()
        && matches!(
            stream.peek_n(1).kind,
            TokenKind::Newline | TokenKind::Eof | TokenKind::Comma | TokenKind::Ident(_)
        )
}

fn parse_point_measure_options(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    measure_type: &str,
    result_axis: Option<&str>,
) -> Result<PointMeasureOptions, ParseError> {
    use crate::netlist::measure::{MeasureOperand, WhenCondition};

    let mut options = PointMeasureOptions {
        at: None,
        when: None,
        from: None,
        to: None,
        td: None,
        occurrence_given: false,
    };
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        if consume_meas_statement_qualifier(stream, line_num, params, true)?.is_some() {
            continue;
        }
        if matches!(stream.peek().kind, TokenKind::Comma) {
            stream.advance();
            continue;
        }
        let TokenKind::Ident(keyword) = &stream.peek().kind else {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    "Unexpected token '{}' in .MEAS {measure_type}",
                    stream.peek().kind
                ),
            });
        };
        let keyword = keyword.to_ascii_uppercase();
        match keyword.as_str() {
            "AT" | "FROM" | "TO" | "TD" => {
                stream.advance();
                let has_equals = stream.consume(&TokenKind::Equals);
                // Xyce's simple-keyword grammar permits the separator to be
                // omitted (`AT value`, `TD value`) as well as written
                // explicitly (`AT=value`, `TD=value`). FROM and TO retain
                // their established explicit-separator contract here.
                if !matches!(keyword.as_str(), "AT" | "TD") && !has_equals {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!("Expected '=' after {keyword} in .MEAS {measure_type}"),
                    });
                }
                // `AT t1` names an earlier measurement: the point is where the
                // axis reaches that measurement's result.
                if keyword == "AT"
                    && let Some(axis) = result_axis
                    && names_measurement_result(stream, params)
                {
                    if options.when.is_some() {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: format!(".MEAS {measure_type} takes AT or WHEN, not both"),
                        });
                    }
                    let name = stream.peek().lexeme.clone();
                    stream.advance();
                    options.when = Some(WhenCondition {
                        left: axis.to_string(),
                        right: MeasureOperand::Result(name),
                        occurrence: crate::netlist::measure::EventOccurrence::default(),
                    });
                    continue;
                }
                let value = if keyword == "TD" {
                    expect_finite_measure_td(stream, line_num, params)?
                } else {
                    expect_measure_value(stream, line_num, params)?
                };
                match keyword.as_str() {
                    "AT" => options.at = Some(value),
                    "FROM" => options.from = Some(value),
                    "TO" => options.to = Some(value),
                    "TD" => options.td = Some(value),
                    _ => unreachable!(),
                }
            }
            "WHEN" => {
                stream.advance();
                let left = parse_meas_signal(stream, line_num, params)?;
                if !stream.consume(&TokenKind::Equals) {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!("Expected '=' after WHEN signal in .MEAS {measure_type}"),
                    });
                }
                let right = parse_measure_when_operand(stream, line_num, params)?;
                options.when = Some(WhenCondition {
                    left,
                    right,
                    occurrence: crate::netlist::measure::EventOccurrence::default(),
                });
            }
            "RISE" | "FALL" | "CROSS" => {
                let Some(condition) = options.when.as_mut() else {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!(
                            "{keyword} occurrence must follow WHEN in .MEAS {measure_type}"
                        ),
                    });
                };
                if options.occurrence_given {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!(
                            "Only one RISE, FALL, or CROSS qualifier is allowed in .MEAS {measure_type}"
                        ),
                    });
                }
                stream.advance();
                if !stream.consume(&TokenKind::Equals) {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!("Expected '=' after {keyword} in .MEAS {measure_type}"),
                    });
                }
                condition.occurrence = crate::netlist::measure::EventOccurrence {
                    edge: match keyword.as_str() {
                        "RISE" => crate::netlist::measure::EdgeType::Rise,
                        "FALL" => crate::netlist::measure::EdgeType::Fall,
                        "CROSS" => crate::netlist::measure::EdgeType::Cross,
                        _ => unreachable!(),
                    },
                    number: parse_measure_event_occurrence(stream, line_num, params, &keyword)?,
                };
                options.occurrence_given = true;
            }
            "GOAL" | "TOL" | "DEFAULT_VAL" => break,
            _ => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!("Unexpected option '{keyword}' in .MEAS {measure_type}"),
                });
            }
        }
    }
    Ok(options)
}

fn parse_measure_when_operand(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<crate::netlist::measure::MeasureOperand, ParseError> {
    use crate::netlist::measure::MeasureOperand;

    if measure_expression_operand_ahead(stream) {
        return Ok(MeasureOperand::Waveform(parse_meas_signal(
            stream, line_num, params,
        )?));
    }
    match &stream.peek().kind {
        TokenKind::Expression(_) => Ok(MeasureOperand::Waveform(parse_meas_signal(
            stream, line_num, params,
        )?)),
        TokenKind::Ident(_) if matches!(stream.peek_n(1).kind, TokenKind::LParen) => Ok(
            MeasureOperand::Waveform(parse_meas_signal(stream, line_num, params)?),
        ),
        // A digit-leading word such as `10m` is a number with its scale
        // suffix, which the lexer keeps as a name; ngspice reads it as the
        // number (frontend/com_measure2.c:1522 measure_parse_when).
        TokenKind::Ident(name)
            if params.get(name).is_none()
                && !(name.starts_with(|c: char| c.is_ascii_digit())
                    && crate::netlist::lexer::parse_spice_value_complete(name).is_ok()) =>
        {
            Ok(MeasureOperand::Waveform(parse_meas_signal(
                stream, line_num, params,
            )?))
        }
        _ => Ok(MeasureOperand::Constant(expect_measure_value(
            stream, line_num, params,
        )?)),
    }
}

fn parse_measure_error_operand(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<String, ParseError> {
    parse_meas_signal(stream, line_num, params)
}

struct ErrorFunctionOptions {
    from: Option<Value>,
    to: Option<Value>,
    minval: Value,
    ymin: Value,
    ymax: Value,
    weight: Option<Value>,
}

fn parse_measure_error_function_options(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<ErrorFunctionOptions, ParseError> {
    let mut options = ErrorFunctionOptions {
        from: None,
        to: None,
        minval: crate::netlist::measure::XYCE_DEFAULT_MEASURE_MINVAL,
        ymin: 1.0e-15,
        ymax: 1.0e15,
        weight: None,
    };
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        if let Some(qualifier) = consume_meas_statement_qualifier(stream, line_num, params, true)? {
            if let ParsedMeasStatementQualifier::Numeric { key, value } = qualifier
                && key == "MINVAL"
            {
                options.minval = value;
            }
            continue;
        }
        let TokenKind::Ident(keyword) = &stream.peek().kind else {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Unexpected token '{}' in .MEAS ERR", stream.peek().kind),
            });
        };
        let keyword = keyword.to_ascii_uppercase();
        if matches!(keyword.as_str(), "GOAL" | "TOL" | "DEFAULT_VAL") {
            break;
        }
        if !matches!(
            keyword.as_str(),
            "FROM" | "TO" | "YMIN" | "YMAX" | "IGNOR" | "IGNORE" | "WEIGHT"
        ) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Unexpected option '{keyword}' in .MEAS ERR"),
            });
        }
        stream.advance();
        if !stream.consume(&TokenKind::Equals) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Expected '=' after {keyword} in .MEAS ERR"),
            });
        }
        let value = expect_measure_value(stream, line_num, params)?;
        if !value.is_finite() {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(".MEAS ERR {keyword} must be finite, found {value}"),
            });
        }
        match keyword.as_str() {
            "FROM" => options.from = Some(value),
            "TO" => options.to = Some(value),
            "YMIN" | "IGNOR" | "IGNORE" => {
                if value < 0.0 {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!(".MEAS ERR {keyword} must be non-negative"),
                    });
                }
                options.ymin = value;
            }
            "YMAX" => {
                if value <= 0.0 {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: ".MEAS ERR YMAX must be positive".to_string(),
                    });
                }
                options.ymax = value;
            }
            "WEIGHT" => options.weight = Some(value),
            _ => unreachable!(),
        }
    }
    Ok(options)
}

struct FileErrorOptions {
    file: String,
    norm: crate::netlist::measure::FileErrorNorm,
    independent_column: Option<isize>,
    dependent_column: usize,
}

fn parse_measure_file_error_options(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<FileErrorOptions, ParseError> {
    use crate::netlist::measure::FileErrorNorm;

    let mut file = None;
    let mut norm = FileErrorNorm::L2;
    let mut independent_column = None;
    let mut dependent_column = None;
    let mut seen = std::collections::HashSet::new();

    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        if consume_meas_statement_qualifier(stream, line_num, params, false)?.is_some() {
            continue;
        }
        let TokenKind::Ident(keyword) = &stream.peek().kind else {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Unexpected token '{}' in .MEAS ERROR", stream.peek().kind),
            });
        };
        let keyword = keyword.to_ascii_uppercase();
        if matches!(keyword.as_str(), "GOAL" | "TOL" | "DEFAULT_VAL") {
            break;
        }
        if !matches!(
            keyword.as_str(),
            "FILE" | "COMP_FUNCTION" | "INDEPVARCOL" | "DEPVARCOL"
        ) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Unexpected option '{keyword}' in .MEAS ERROR"),
            });
        }
        if !seen.insert(keyword.clone()) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Duplicate option '{keyword}' in .MEAS ERROR"),
            });
        }
        stream.advance();
        // Xyce accepts both `KEY=value` and `KEY value` spellings for these
        // measure qualifiers.
        let _optional_equals = stream.consume(&TokenKind::Equals);
        match keyword.as_str() {
            "FILE" => file = Some(parse_measure_file_path(stream, line_num)?),
            "COMP_FUNCTION" => {
                let value = match &stream.peek().kind {
                    TokenKind::Ident(_) => stream.peek().lexeme.clone(),
                    TokenKind::StringLit(value) => value.clone(),
                    other => {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: format!(
                                "Expected comparison-function name in .MEAS ERROR, found {other:?}"
                            ),
                        });
                    }
                };
                stream.advance();
                norm = if value.eq_ignore_ascii_case("INFNORM") {
                    FileErrorNorm::Infinity
                } else if value.eq_ignore_ascii_case("L1NORM") {
                    FileErrorNorm::L1
                } else {
                    // Xyce defaults both an omitted and an unrecognized
                    // comparison function to the Frobenius/L2 norm.
                    FileErrorNorm::L2
                };
            }
            "INDEPVARCOL" => {
                independent_column = Some(parse_measure_column(stream, line_num, params, true)?);
            }
            "DEPVARCOL" => {
                let value = parse_measure_column(stream, line_num, params, false)?;
                dependent_column = usize::try_from(value).ok();
            }
            _ => unreachable!(),
        }
    }

    let file = file.ok_or_else(|| {
        ParseError::MissingParameter(format!("FILE in .MEAS ERROR at line {line_num}"))
    })?;
    let dependent_column = dependent_column.ok_or_else(|| {
        ParseError::MissingParameter(format!(
            "non-negative DEPVARCOL in .MEAS ERROR at line {line_num}"
        ))
    })?;
    Ok(FileErrorOptions {
        file,
        norm,
        independent_column,
        dependent_column,
    })
}

fn parse_measure_column(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    allow_negative: bool,
) -> Result<isize, ParseError> {
    let value = expect_value(stream, line_num, params)?;
    if !value.is_finite()
        || value.fract() != 0.0
        || value < isize::MIN as Value
        || value > isize::MAX as Value
        || !allow_negative && value < 0.0
    {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                ".MEAS ERROR column index must be {}integer-valued, found {value}",
                if allow_negative {
                    "an "
                } else {
                    "a non-negative "
                }
            ),
        });
    }
    Ok(value as isize)
}

fn parse_measure_file_path(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<String, ParseError> {
    if matches!(stream.peek().kind, TokenKind::StringLit(_))
        && let Some(path) = super::command_parsers::quoted_path_lexeme(stream)
    {
        if path.is_empty() {
            return Err(ParseError::MissingParameter(format!(
                "FILE path in .MEAS ERROR at line {line_num}"
            )));
        }
        return Ok(path);
    }

    let first = stream.peek().clone();
    let source_line = first.span.line;
    let mut previous_end = first.span.start;
    let mut path = String::new();
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        if !path.is_empty()
            && (stream.peek().span.line != source_line || stream.peek().span.start != previous_end)
        {
            break;
        }
        if measure_file_option_ahead(stream) {
            break;
        }
        match stream.peek().kind {
            TokenKind::Equals | TokenKind::StringLit(_) | TokenKind::Expression(_) => break,
            _ => {
                path.push_str(&stream.peek().lexeme);
                previous_end = stream.peek().span.end;
                stream.advance();
            }
        }
    }
    if path.is_empty() {
        Err(ParseError::MissingParameter(format!(
            "FILE path in .MEAS ERROR at line {line_num}"
        )))
    } else {
        Ok(path)
    }
}

fn measure_file_option_ahead(stream: &TokenStream) -> bool {
    let TokenKind::Ident(keyword) = &stream.peek().kind else {
        return false;
    };
    matches!(
        keyword.to_ascii_uppercase().as_str(),
        "FILE"
            | "COMP_FUNCTION"
            | "INDEPVARCOL"
            | "DEPVARCOL"
            | "GOAL"
            | "TOL"
            | "DEFAULT_VAL"
            | "PRINT"
    )
}

/// The `FROM`/`TO`/`TD` window a `.MEASURE` statement may restrict itself to.
type MeasureWindow = (
    Option<crate::Value>,
    Option<crate::Value>,
    Option<crate::Value>,
);

/// A measure window plus which occurrence of the event the statement selects.
type MeasureWindowWithOccurrence = (
    Option<crate::Value>,
    Option<crate::Value>,
    Option<crate::Value>,
    crate::netlist::measure::EventOccurrence,
);

fn parse_measure_when_event_options(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<MeasureWindowWithOccurrence, ParseError> {
    let mut from = None;
    let mut to = None;
    let mut td = None;
    let mut occurrence = crate::netlist::measure::EventOccurrence::default();
    let mut occurrence_given = false;

    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        if consume_meas_statement_qualifier(stream, line_num, params, true)?.is_some() {
            continue;
        }
        let TokenKind::Ident(keyword) = &stream.peek().kind else {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Unexpected token '{}' in .MEAS WHEN", stream.peek().kind),
            });
        };
        let keyword = keyword.to_ascii_uppercase();
        if matches!(keyword.as_str(), "GOAL" | "TOL" | "DEFAULT_VAL") {
            break;
        }
        if !matches!(
            keyword.as_str(),
            "FROM" | "TO" | "TD" | "RISE" | "FALL" | "CROSS"
        ) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Unexpected option '{keyword}' in .MEAS WHEN"),
            });
        }
        stream.advance();
        let has_equals = stream.consume(&TokenKind::Equals);
        if keyword != "TD" && !has_equals {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Expected '=' after {keyword} in .MEAS WHEN"),
            });
        }
        match keyword.as_str() {
            "FROM" => from = Some(expect_measure_value(stream, line_num, params)?),
            "TO" => to = Some(expect_measure_value(stream, line_num, params)?),
            "TD" => td = Some(expect_finite_measure_td(stream, line_num, params)?),
            "RISE" | "FALL" | "CROSS" => {
                if occurrence_given {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: "Only one RISE, FALL, or CROSS qualifier is allowed in .MEAS WHEN"
                            .to_string(),
                    });
                }
                occurrence = crate::netlist::measure::EventOccurrence {
                    edge: match keyword.as_str() {
                        "RISE" => crate::netlist::measure::EdgeType::Rise,
                        "FALL" => crate::netlist::measure::EdgeType::Fall,
                        "CROSS" => crate::netlist::measure::EdgeType::Cross,
                        _ => unreachable!(),
                    },
                    number: parse_measure_event_occurrence(stream, line_num, params, &keyword)?,
                };
                occurrence_given = true;
            }
            _ => unreachable!(),
        }
    }

    Ok((from, to, td, occurrence))
}

fn parse_measure_name(stream: &mut TokenStream, line_num: usize) -> Result<String, ParseError> {
    let first = stream.peek().clone();
    if matches!(first.kind, TokenKind::Newline | TokenKind::Eof) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "Expected measurement name".to_string(),
        });
    }

    let mut name = first.lexeme;
    let mut end = first.span.end;
    stream.advance();

    while !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof)
        && stream.peek().span.line == first.span.line
        && stream.peek().span.start == end
    {
        let token = stream.peek().clone();
        name.push_str(&token.lexeme);
        end = token.span.end;
        stream.advance();
    }

    if name.is_empty() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "Expected measurement name".to_string(),
        });
    }
    Ok(name.to_ascii_uppercase())
}

/// Scan statement-wide Xyce measurement qualifiers without disturbing the
/// type-specific parser. Xyce accepts these qualifiers in any order relative
/// to measurement-specific qualifiers, and the last duplicate wins.
struct MeasStatementOptions {
    goal: Option<Value>,
    tolerance: Option<Value>,
    default_value: Option<Value>,
    fail_value: Option<Value>,
    print_policy: crate::netlist::measure::MeasurePrintPolicy,
    minval: Value,
}

fn scan_meas_statement_options(
    stream: &TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<MeasStatementOptions, ParseError> {
    let mut stream = stream.clone();
    let mut options = MeasStatementOptions {
        goal: None,
        tolerance: None,
        default_value: None,
        fail_value: None,
        print_policy: crate::netlist::measure::MeasurePrintPolicy::All,
        minval: crate::netlist::measure::XYCE_DEFAULT_MEASURE_MINVAL,
    };
    // The separator is optional for FAILVALUE, so top-level grammar context
    // distinguishes it from a node or branch named `FAILVALUE` inside a probe.
    let mut parenthesis_depth = 0usize;
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        let statement_qualifier_ahead = parenthesis_depth == 0
            && match &stream.peek().kind {
                TokenKind::Ident(key) => {
                    matches!(stream.peek_n(1).kind, TokenKind::Equals)
                        || key.eq_ignore_ascii_case("FAILVALUE")
                }
                _ => false,
            };
        if statement_qualifier_ahead
            && let Some(qualifier) =
                consume_meas_statement_qualifier(&mut stream, line_num, params, true)?
        {
            match qualifier {
                ParsedMeasStatementQualifier::Numeric { key, value } => match key.as_str() {
                    "GOAL" => options.goal = Some(value),
                    "TOL" => options.tolerance = Some(value),
                    "DEFAULT_VAL" => options.default_value = Some(value),
                    "FAILVALUE" => options.fail_value = Some(value),
                    "MINVAL" => options.minval = value,
                    _ => unreachable!(),
                },
                ParsedMeasStatementQualifier::Print(policy) => options.print_policy = policy,
            }
            continue;
        }
        match stream.peek().kind {
            TokenKind::LParen => parenthesis_depth += 1,
            TokenKind::RParen => parenthesis_depth = parenthesis_depth.saturating_sub(1),
            _ => {}
        }
        stream.advance();
    }
    Ok(options)
}

enum ParsedMeasStatementQualifier {
    Numeric { key: String, value: Value },
    Print(crate::netlist::measure::MeasurePrintPolicy),
}

fn consume_meas_statement_qualifier(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    allow_minval: bool,
) -> Result<Option<ParsedMeasStatementQualifier>, ParseError> {
    let TokenKind::Ident(key) = &stream.peek().kind else {
        return Ok(None);
    };
    let key = key.to_ascii_uppercase();
    if !matches!(
        key.as_str(),
        "GOAL" | "TOL" | "DEFAULT_VAL" | "FAILVALUE" | "PRINT" | "MINVAL"
    ) || key == "MINVAL" && !allow_minval
        || key == "TOL" && params.expression_dialect() == crate::config::ExpressionDialect::Xyce
    {
        return Ok(None);
    }
    if key == "FAILVALUE" && params.expression_dialect() != crate::config::ExpressionDialect::Xyce {
        return Err(ParseError::Syntax {
            line: line_num,
            message: ".MEAS FAILVALUE is only supported in Xyce compatibility mode".to_string(),
        });
    }
    stream.advance();
    let has_equals = stream.consume(&TokenKind::Equals);
    if key != "FAILVALUE" && !has_equals {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("Expected '=' after {key} in .MEAS"),
        });
    }
    if key == "PRINT" {
        let value = expect_ident(stream, line_num)?;
        let policy = match value.to_ascii_uppercase().as_str() {
            "ALL" => crate::netlist::measure::MeasurePrintPolicy::All,
            "STDOUT" => crate::netlist::measure::MeasurePrintPolicy::Stdout,
            "NONE" => crate::netlist::measure::MeasurePrintPolicy::None,
            // Xyce leaves the constructor's ALL default in place for an
            // unrecognized per-measure PRINT value.
            _ => crate::netlist::measure::MeasurePrintPolicy::All,
        };
        return Ok(Some(ParsedMeasStatementQualifier::Print(policy)));
    }
    let value = expect_value(stream, line_num, params)?;
    if key == "DEFAULT_VAL" && !value.is_finite() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(".MEAS DEFAULT_VAL must be finite, found {value}"),
        });
    }
    if key == "MINVAL" && (!value.is_finite() || value < 0.0) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(".MEAS MINVAL must be finite and non-negative, found {value}"),
        });
    }
    Ok(Some(ParsedMeasStatementQualifier::Numeric { key, value }))
}

pub(super) fn parse_meas_delay_spec(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    section_name: &str,
    stop_at_targ: bool,
    window: &mut (Option<Value>, Option<Value>),
) -> Result<crate::netlist::measure::TrigSpec, ParseError> {
    use crate::netlist::measure::{
        EdgeType, EventOccurrence, TrigSpec, TriggerEvent, WhenCondition,
    };

    let event = if matches!(&stream.peek().kind, TokenKind::Ident(value) if value.eq_ignore_ascii_case("AT"))
        && matches!(stream.peek_n(1).kind, TokenKind::Equals)
    {
        stream.advance();
        stream.advance();
        TriggerEvent::At(expect_measure_value(stream, line_num, params)?)
    } else {
        let left = parse_meas_signal(stream, line_num, params)?;
        let right = if stream.consume(&TokenKind::Equals) {
            parse_measure_when_operand(stream, line_num, params)?
        } else if matches!(
            stream.peek().kind,
            TokenKind::Number(_) | TokenKind::Expression(_) | TokenKind::Plus | TokenKind::Minus
        ) || measure_expression_operand_ahead(stream)
        {
            // Legacy Xyce/HSpice TRIG/TARG syntax permits the target value as
            // the next field without an intervening '=' or VAL keyword.
            parse_measure_when_operand(stream, line_num, params)?
        } else if matches!(&stream.peek().kind, TokenKind::Ident(keyword)
            if keyword.eq_ignore_ascii_case("FRAC_MAX"))
        {
            // FRAC_MAX supplies the legacy detector's dynamic target after
            // the waveform maximum is known.
            crate::netlist::measure::MeasureOperand::Constant(0.0)
        } else {
            let keyword = expect_ident(stream, line_num)?;
            if !keyword.eq_ignore_ascii_case("VAL") || !stream.consume(&TokenKind::Equals) {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "Expected '=' or VAL= after signal in .MEAS {section_name} specification"
                    ),
                });
            }
            parse_measure_when_operand(stream, line_num, params)?
        };
        TriggerEvent::When(WhenCondition {
            left,
            right,
            occurrence: EventOccurrence::default(),
        })
    };
    let mut spec = TrigSpec {
        event,
        td: None,
        frac_max: None,
        occurrence_explicit: false,
    };
    let mut occurrence_given = false;

    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        if consume_meas_statement_qualifier(stream, line_num, params, true)?.is_some() {
            continue;
        }
        match &stream.peek().kind {
            TokenKind::Comma => {
                stream.advance();
            }
            TokenKind::Ident(s) if stop_at_targ && s.eq_ignore_ascii_case("TARG") => break,
            // Verification options end the spec; the statement parser
            // consumes them.
            TokenKind::Ident(s)
                if s.eq_ignore_ascii_case("GOAL")
                    || s.eq_ignore_ascii_case("TOL")
                    || s.eq_ignore_ascii_case("DEFAULT_VAL") =>
            {
                break;
            }
            TokenKind::Ident(s) if s.eq_ignore_ascii_case("VAL") => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!("Duplicate VAL option in .MEAS {section_name} specification"),
                });
            }
            TokenKind::Ident(s) if s.eq_ignore_ascii_case("TD") => {
                stream.advance();
                let _optional_equals = stream.consume(&TokenKind::Equals);
                spec.td = Some(expect_finite_measure_td(stream, line_num, params)?);
            }
            TokenKind::Ident(s)
                if s.eq_ignore_ascii_case("FROM") || s.eq_ignore_ascii_case("TO") =>
            {
                let key = s.to_ascii_uppercase();
                stream.advance();
                let _optional_equals = stream.consume(&TokenKind::Equals);
                let value = expect_measure_value(stream, line_num, params)?;
                if !value.is_finite() {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!(".MEAS {key} must be finite, found {value}"),
                    });
                }
                if key == "FROM" {
                    window.0 = Some(value);
                } else {
                    window.1 = Some(value);
                }
            }
            TokenKind::Ident(s) if s.eq_ignore_ascii_case("FRAC_MAX") => {
                if spec.frac_max.is_some() {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!(
                            "Duplicate FRAC_MAX option in .MEAS {section_name} specification"
                        ),
                    });
                }
                stream.advance();
                let _optional_equals = stream.consume(&TokenKind::Equals);
                let value = expect_value(stream, line_num, params)?;
                if !value.is_finite() {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!(
                            "FRAC_MAX must be finite in .MEAS {section_name} specification"
                        ),
                    });
                }
                spec.frac_max = Some(value);
            }
            TokenKind::Ident(s)
                if s.eq_ignore_ascii_case("RISE")
                    || s.eq_ignore_ascii_case("FALL")
                    || s.eq_ignore_ascii_case("CROSS") =>
            {
                let TriggerEvent::When(condition) = &mut spec.event else {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!(
                            "RISE, FALL, and CROSS are invalid with AT in .MEAS {section_name} specification"
                        ),
                    });
                };
                if occurrence_given {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!(
                            "Only one RISE, FALL, or CROSS qualifier is allowed in .MEAS {section_name} specification"
                        ),
                    });
                }
                let keyword = s.to_ascii_uppercase();
                stream.advance();
                if !stream.consume(&TokenKind::Equals) {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!(
                            "Expected '=' after {} in .MEAS {} specification",
                            keyword, section_name
                        ),
                    });
                }
                condition.occurrence = EventOccurrence {
                    edge: match keyword.as_str() {
                        "RISE" => EdgeType::Rise,
                        "FALL" => EdgeType::Fall,
                        "CROSS" => EdgeType::Cross,
                        _ => unreachable!(),
                    },
                    number: parse_measure_event_occurrence(stream, line_num, params, &keyword)?,
                };
                occurrence_given = true;
                spec.occurrence_explicit = true;
            }
            _ => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "Unexpected token '{}' in .MEAS {} specification",
                        stream.peek().kind,
                        section_name
                    ),
                });
            }
        }
    }

    Ok(spec)
}

fn parse_measure_event_occurrence(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    keyword: &str,
) -> Result<isize, ParseError> {
    if let TokenKind::Ident(value) = &stream.peek().kind
        && value.eq_ignore_ascii_case("LAST")
    {
        stream.advance();
        return Ok(-1);
    }
    let value = expect_value(stream, line_num, params)?;
    let truncated = value.trunc();
    if !value.is_finite()
        || truncated < isize::MIN as crate::Value
        || truncated > isize::MAX as crate::Value
    {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Expected a finite in-range occurrence or LAST for {keyword} in .MEAS, found {value}"
            ),
        });
    }
    // Xyce stores the parsed numeric value with static_cast<int>, truncating
    // toward zero. Zero is meaningful: scalar modern selectors take the
    // first requested edge and continuous selectors emit from the start.
    Ok(truncated as isize)
}

/// `FROM`, `TO` and, in the Xyce dialect, `TD` of a range measure.
type MeasureRange = (
    Option<crate::Value>,
    Option<crate::Value>,
    Option<crate::Value>,
);

/// Xyce's `TD` delays a range or extrema measure; the other dialects keep
/// refusing it there.
fn accepts_measure_delay(params: &ParamContext, key: &str) -> bool {
    key.eq_ignore_ascii_case("TD")
        && params.expression_dialect() == crate::config::ExpressionDialect::Xyce
}

pub(super) fn parse_measure_range_options(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<MeasureRange, ParseError> {
    let mut from = None;
    let mut to = None;
    let mut td = None;

    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        if consume_meas_statement_qualifier(stream, line_num, params, false)?.is_some() {
            continue;
        }
        match &stream.peek().kind {
            TokenKind::Comma => {
                stream.advance();
            }
            TokenKind::Ident(s)
                if s.eq_ignore_ascii_case("GOAL")
                    || s.eq_ignore_ascii_case("TOL")
                    || s.eq_ignore_ascii_case("DEFAULT_VAL") =>
            {
                break;
            }
            TokenKind::Ident(s)
                if s.eq_ignore_ascii_case("FROM")
                    || s.eq_ignore_ascii_case("TO")
                    || accepts_measure_delay(params, s) =>
            {
                let key = s.to_ascii_uppercase();
                stream.advance();
                if !stream.consume(&TokenKind::Equals) {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!("Expected '=' after {} in .MEAS", key),
                    });
                }
                let value = expect_measure_value(stream, line_num, params)?;
                match key.as_str() {
                    "FROM" => from = Some(value),
                    "TO" => to = Some(value),
                    "TD" => td = Some(value),
                    _ => {}
                }
            }
            _ => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "Unexpected token '{}' after .MEAS range operand",
                        stream.peek().kind
                    ),
                });
            }
        }
    }

    Ok((from, to, td))
}

/// The bins and band of a Xyce `.MEASURE FFT` `ENOB`, `SFDR`, `SNR`, `SNDR`
/// or `THD` (N_IO_MeasureBase.C:508-524). `BINSIZ` defaults to 0, and Xyce
/// reads it and `NBHARM` as integers.
struct FftMetricOptions {
    bin_size: usize,
    min_freq: Option<crate::Value>,
    max_freq: Option<crate::Value>,
    harmonics: Option<i64>,
}

fn parse_measure_fft_options(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    kind: &str,
) -> Result<FftMetricOptions, ParseError> {
    let mut options = FftMetricOptions {
        bin_size: 0,
        min_freq: None,
        max_freq: None,
        harmonics: None,
    };
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        if consume_meas_statement_qualifier(stream, line_num, params, false)?.is_some() {
            continue;
        }
        if matches!(stream.peek().kind, TokenKind::Comma) {
            stream.advance();
            continue;
        }
        let TokenKind::Ident(key) = &stream.peek().kind else {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    "Unexpected token '{}' after .MEAS {kind} operand",
                    stream.peek().kind
                ),
            });
        };
        let key = key.to_ascii_uppercase();
        if matches!(key.as_str(), "GOAL" | "TOL" | "DEFAULT_VAL") {
            break;
        }
        if !matches!(key.as_str(), "BINSIZ" | "MINFREQ" | "MAXFREQ" | "NBHARM") {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Unexpected option '{key}' after .MEAS {kind} operand"),
            });
        }
        stream.advance();
        if !stream.consume(&TokenKind::Equals) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Expected '=' after {key} in .MEAS {kind}"),
            });
        }
        let value = expect_measure_value(stream, line_num, params)?;
        let whole = value.is_finite() && value.fract() == 0.0;
        match key.as_str() {
            "BINSIZ" if whole && value >= 0.0 => options.bin_size = value as usize,
            "BINSIZ" => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        ".MEAS {kind} BINSIZ must be a whole number of bins, got {value}"
                    ),
                });
            }
            "MINFREQ" => options.min_freq = Some(value),
            "MAXFREQ" => options.max_freq = Some(value),
            _ if whole => options.harmonics = Some(value as i64),
            _ => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(".MEAS {kind} NBHARM must be an integer, got {value}"),
                });
            }
        }
    }
    Ok(options)
}

/// The fundamental, harmonic count, grid and window of a Xyce `FOUR`
/// measure. `NUMFREQ` defaults to 10 and `GRIDSIZE` to 200
/// (N_IO_MeasureBase.C:140-141). Without `AT` the fundamental is 0, so the
/// measure fails at the end of the run, as Xyce's does; a negative `AT` is
/// an error (N_IO_MeasureFourier.C:67-70).
/// `PRECISION` only sets the digits of Xyce's printed harmonic table.
struct FourierMeasureOptions {
    at: crate::Value,
    num_freq: usize,
    grid_size: usize,
    from: Option<crate::Value>,
    to: Option<crate::Value>,
    td: Option<crate::Value>,
}

fn parse_measure_fourier_options(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<FourierMeasureOptions, ParseError> {
    let mut at = None;
    let mut options = FourierMeasureOptions {
        at: 0.0,
        num_freq: 10,
        grid_size: 200,
        from: None,
        to: None,
        td: None,
    };
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        // MINVAL is this measure's window tolerance; the statement scan has
        // already read it.
        if consume_meas_statement_qualifier(stream, line_num, params, true)?.is_some() {
            continue;
        }
        if matches!(stream.peek().kind, TokenKind::Comma) {
            stream.advance();
            continue;
        }
        let TokenKind::Ident(key) = &stream.peek().kind else {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    "Unexpected token '{}' after .MEAS FOUR operand",
                    stream.peek().kind
                ),
            });
        };
        let key = key.to_ascii_uppercase();
        if matches!(key.as_str(), "GOAL" | "TOL" | "DEFAULT_VAL") {
            break;
        }
        if !matches!(
            key.as_str(),
            "AT" | "NUMFREQ" | "GRIDSIZE" | "FROM" | "TO" | "TD" | "PRECISION"
        ) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Unexpected option '{key}' after .MEAS FOUR operand"),
            });
        }
        stream.advance();
        if !stream.consume(&TokenKind::Equals) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Expected '=' after {key} in .MEAS FOUR"),
            });
        }
        let value = expect_measure_value(stream, line_num, params)?;
        let count = |name: &str| {
            if value.is_finite() && value >= 1.0 && value.fract() == 0.0 {
                Ok(value as usize)
            } else {
                Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(".MEAS FOUR {name} must be a positive integer, got {value}"),
                })
            }
        };
        match key.as_str() {
            "AT" => at = Some(value),
            "NUMFREQ" => options.num_freq = count("NUMFREQ")?,
            "GRIDSIZE" => options.grid_size = count("GRIDSIZE")?,
            "FROM" => options.from = Some(value),
            "TO" => options.to = Some(value),
            "TD" => options.td = Some(value),
            // The digits Xyce prints the harmonic table with.
            _ => {}
        }
    }
    options.at = match at {
        Some(at) if at.is_finite() && at >= 0.0 => at,
        Some(at) => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(".MEAS FOUR AT must be a frequency of 0 or more, got {at}"),
            });
        }
        None => 0.0,
    };
    Ok(options)
}

/// The window and levels of a Xyce `DUTY`, `ON_TIME`, `OFF_TIME` or `FREQ`
/// measure. `ON` and `OFF` default to 0 (N_IO_MeasureBase.C:95-98).
struct OnOffMeasureOptions {
    from: Option<crate::Value>,
    to: Option<crate::Value>,
    td: Option<crate::Value>,
    on: crate::Value,
    off: crate::Value,
}

fn parse_measure_on_off_options(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<OnOffMeasureOptions, ParseError> {
    let mut options = OnOffMeasureOptions {
        from: None,
        to: None,
        td: None,
        on: 0.0,
        off: 0.0,
    };
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        // MINVAL is these measures' comparison tolerance; the statement scan
        // has already read it.
        if consume_meas_statement_qualifier(stream, line_num, params, true)?.is_some() {
            continue;
        }
        if matches!(stream.peek().kind, TokenKind::Comma) {
            stream.advance();
            continue;
        }
        let TokenKind::Ident(key) = &stream.peek().kind else {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    "Unexpected token '{}' after .MEAS on/off operand",
                    stream.peek().kind
                ),
            });
        };
        let key = key.to_ascii_uppercase();
        if matches!(key.as_str(), "GOAL" | "TOL" | "DEFAULT_VAL") {
            break;
        }
        if !matches!(key.as_str(), "FROM" | "TO" | "TD" | "ON" | "OFF") {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Unexpected option '{key}' after .MEAS on/off operand"),
            });
        }
        stream.advance();
        if !stream.consume(&TokenKind::Equals) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Expected '=' after {key} in .MEAS"),
            });
        }
        let value = expect_measure_value(stream, line_num, params)?;
        match key.as_str() {
            "FROM" => options.from = Some(value),
            "TO" => options.to = Some(value),
            "TD" => options.td = Some(value),
            "ON" => options.on = value,
            _ => options.off = value,
        }
    }
    Ok(options)
}

/// The window, output and Xyce RISE/FALL/CROSS window of a `MAX`, `MIN` or
/// `PP` measure. Xyce reads RISE, FALL, CROSS and RFC_LEVEL on these three
/// alone (Extrema::updateTran, N_IO_MeasureExtrema.C:109-162).
struct ExtremaMeasureOptions {
    from: Option<crate::Value>,
    to: Option<crate::Value>,
    td: Option<crate::Value>,
    output: crate::netlist::measure::ExtremaOutput,
    crossing: Option<crate::netlist::measure::EventOccurrence>,
    rfc_level: Option<crate::Value>,
}

fn parse_measure_extrema_options(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    allow_output: bool,
) -> Result<ExtremaMeasureOptions, ParseError> {
    use crate::netlist::measure::{EdgeType, EventOccurrence, ExtremaOutput};

    let mut from = None;
    let mut to = None;
    let mut td = None;
    let mut output = ExtremaOutput::Value;
    let mut crossing = None;
    let mut rfc_level = None;
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        if consume_meas_statement_qualifier(stream, line_num, params, false)?.is_some() {
            continue;
        }
        if matches!(stream.peek().kind, TokenKind::Comma) {
            stream.advance();
            continue;
        }
        let TokenKind::Ident(key) = &stream.peek().kind else {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    "Unexpected token '{}' after .MEAS extrema operand",
                    stream.peek().kind
                ),
            });
        };
        let key = key.to_ascii_uppercase();
        if matches!(key.as_str(), "GOAL" | "TOL" | "DEFAULT_VAL") {
            break;
        }
        let known = matches!(
            key.as_str(),
            "FROM" | "TO" | "RISE" | "FALL" | "CROSS" | "RFC_LEVEL"
        ) || (key == "OUTPUT" && allow_output)
            || accepts_measure_delay(params, &key);
        if !known {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Unexpected option '{key}' after .MEAS extrema operand"),
            });
        }
        stream.advance();
        let has_equals = stream.consume(&TokenKind::Equals);
        if key != "OUTPUT" && !has_equals {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Expected '=' after {key} in .MEAS"),
            });
        }
        if key == "OUTPUT" {
            let value = expect_ident(stream, line_num)?;
            if matches!(value.to_ascii_uppercase().as_str(), "TIME" | "FREQ" | "SV") {
                output = ExtremaOutput::IndependentAxis;
            }
        } else if matches!(key.as_str(), "RISE" | "FALL" | "CROSS") {
            if crossing.is_some() {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message:
                        "Only one RISE, FALL, or CROSS qualifier is allowed on an extrema .MEAS"
                            .to_string(),
                });
            }
            crossing = Some(EventOccurrence {
                edge: match key.as_str() {
                    "RISE" => EdgeType::Rise,
                    "FALL" => EdgeType::Fall,
                    _ => EdgeType::Cross,
                },
                number: parse_measure_event_occurrence(stream, line_num, params, &key)?,
            });
        } else {
            let value = expect_measure_value(stream, line_num, params)?;
            match key.as_str() {
                "FROM" => from = Some(value),
                "TD" => td = Some(value),
                "RFC_LEVEL" => rfc_level = Some(value),
                _ => to = Some(value),
            }
        }
    }
    Ok(ExtremaMeasureOptions {
        from,
        to,
        td,
        output,
        crossing,
        rfc_level,
    })
}

fn parse_measure_equation_options(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<MeasureWindow, ParseError> {
    let mut from = None;
    let mut to = None;
    let mut td = None;

    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        if consume_meas_statement_qualifier(stream, line_num, params, false)?.is_some() {
            continue;
        }
        let TokenKind::Ident(key) = &stream.peek().kind else {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    "Unexpected token {:?} after .MEAS equation expression",
                    stream.peek().kind
                ),
            });
        };
        let key = key.to_ascii_uppercase();
        if !matches!(key.as_str(), "FROM" | "TO" | "TD") {
            break;
        }
        stream.advance();
        if !stream.consume(&TokenKind::Equals) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Expected '=' after {key} in .MEAS equation"),
            });
        }
        let value = expect_measure_value(stream, line_num, params)?;
        match key.as_str() {
            "FROM" => from = Some(value),
            "TO" => to = Some(value),
            "TD" => td = Some(value),
            _ => unreachable!(),
        }
    }

    Ok((from, to, td))
}

#[allow(clippy::too_many_arguments)]
pub(super) fn parse_param_statement(
    stream: &mut TokenStream,
    line_num: usize,
    params: &mut ParamContext,
    deferred_params: Option<&mut Vec<(String, String)>>,
    retain_global_expression: bool,
    diagnostics: &mut Vec<ParseDiagnostic>,
    origin: &NetlistSourceLocation,
    overrides: &[ParameterOverride],
    root_redefinitions: Option<&mut RootRedefinitions>,
    temperature_options: &mut temperature::TemperatureOptionPlan,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    let top_level = deferred_params.is_none();
    let draws_before = params.statistical_draw_count();
    let parsed = parse_param_statement_cards(
        stream,
        line_num,
        params,
        deferred_params,
        retain_global_expression,
        diagnostics,
        origin,
        overrides,
        root_redefinitions,
        temperature_options,
        abort,
    );
    if top_level && params.statistical_draw_count() != draws_before {
        params.mark_parse_statistical_draws();
    }
    parsed
}

#[allow(clippy::too_many_arguments)]
fn parse_param_statement_cards(
    stream: &mut TokenStream,
    line_num: usize,
    params: &mut ParamContext,
    mut deferred_params: Option<&mut Vec<(String, String)>>,
    retain_global_expression: bool,
    diagnostics: &mut Vec<ParseDiagnostic>,
    origin: &NetlistSourceLocation,
    overrides: &[ParameterOverride],
    mut root_redefinitions: Option<&mut RootRedefinitions>,
    temperature_options: &mut temperature::TemperatureOptionPlan,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    if retain_global_expression && deferred_params.is_some() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: ".GLOBAL_PARAM is only valid in the top-level netlist scope".to_string(),
        }
        .into());
    }
    // Parse one or more NAME=VALUE pairs
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline) {
        // Skip commas
        while stream.consume(&TokenKind::Comma) {}

        if stream.is_eof() || matches!(stream.peek().kind, TokenKind::Newline) {
            break;
        }

        ensure_parse_not_aborted(abort)?;
        let name = expect_ident(stream, line_num)?;
        if retain_global_expression
            && matches!(
                name.to_ascii_uppercase().as_str(),
                "TIME" | "TEMP" | "TEMPER" | "VT" | "GMIN" | "FREQ" | "HERTZ"
            )
        {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(".GLOBAL_PARAM name '{}' is reserved by the simulator", name),
            }
            .into());
        }

        if matches!(stream.peek().kind, TokenKind::LParen) {
            let acceptance = params.accepts_parameter_function_definition(&name, origin);
            let error_policy = params.parameter_redefinition_diagnostic_policy()
                == ParameterRedefinitionDiagnosticPolicy::Error;
            if acceptance.authoritative && (acceptance.first_origin.is_none() || !error_policy) {
                let pinned = root_redefinitions.as_deref_mut().and_then(|redefinitions| {
                    redefinitions.define(&name, RootDefinitionKind::Function, params)
                });
                parse_param_function_definition(stream, line_num, params, name.clone())?;
                if let Some(pin) = pinned {
                    pin.bind(&name, RootDefinitionKind::Function, params);
                }
            } else {
                let mut ignored = params.clone();
                parse_param_function_definition(stream, line_num, &mut ignored, name.clone())?;
            }
            handle_parameter_redefinition(
                params,
                diagnostics,
                origin,
                name.clone(),
                ParameterDefinitionKind::ParameterFunction,
                acceptance.first_origin,
            )?;
            continue;
        }

        // Expect = sign. LTspice and QSPICE also take the value straight
        // after the name: `.param X table(phimargin, 29.4, 3.5, ...)`.
        if !stream.consume(&TokenKind::Equals) && !param_value_follows_name(stream, params) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Expected '=' after parameter name '{}'", name),
            }
            .into());
        }

        let acceptance =
            params.accepts_parameter_definition(&name, retain_global_expression, origin);
        let error_policy = params.parameter_redefinition_diagnostic_policy()
            == ParameterRedefinitionDiagnosticPolicy::Error;
        if acceptance.authoritative && (acceptance.first_origin.is_none() || !error_policy) {
            let parameter_override = overrides.iter().find(|parameter| {
                parameter.global == retain_global_expression
                    && parameter.name.eq_ignore_ascii_case(&name)
            });
            // Every root reader sees a redefined parameter's last definition.
            let kind = if retain_global_expression {
                RootDefinitionKind::GlobalParameter
            } else {
                RootDefinitionKind::Parameter
            };
            let pinned = root_redefinitions
                .as_deref_mut()
                .and_then(|redefinitions| redefinitions.define(&name, kind, params))
                .filter(|_| parameter_override.is_none());
            if let Some(error) = parse_param_assignment_value(
                stream,
                line_num,
                params,
                deferred_params.as_deref_mut(),
                retain_global_expression,
                name.clone(),
                parameter_override.map(|parameter| parameter.value),
                abort,
            )? {
                temperature_options.retain_parameter_error(error, origin);
            }
            if let Some(pin) = pinned {
                pin.bind(&name, kind, params);
            }
            params.retain_expression_origin(&name, retain_global_expression, origin);
            if parameter_override.is_some_and(|parameter| parameter.direction) {
                params.seed_parameter_direction(&name, retain_global_expression);
            }
        } else {
            // The ignored definition is still tokenized and syntax-checked,
            // but it cannot mutate the authoritative context or its deferred
            // expression list. A private deferred sink lets an unused local
            // duplicate reference an otherwise-unresolved name without
            // changing the retained first definition.
            let mut ignored_params = params.isolated_random_clone();
            let mut ignored_deferred = Vec::new();
            let ignored_deferred = (!retain_global_expression).then_some(&mut ignored_deferred);
            if let Some(error) = parse_param_assignment_value(
                stream,
                line_num,
                &mut ignored_params,
                ignored_deferred,
                retain_global_expression,
                name.clone(),
                None,
                abort,
            )? {
                temperature_options.retain_parameter_error(error, origin);
            }
        }
        handle_parameter_redefinition(
            params,
            diagnostics,
            origin,
            name,
            if retain_global_expression {
                ParameterDefinitionKind::GlobalParameter
            } else {
                ParameterDefinitionKind::Parameter
            },
            acceptance.first_origin,
        )?;
    }

    Ok(())
}

/// Whether a `.param` name is followed by its value with no `=` between,
/// which LTspice and QSPICE accept and Xyce does not.
fn param_value_follows_name(stream: &TokenStream, params: &ParamContext) -> bool {
    params.expression_dialect() != crate::config::ExpressionDialect::Xyce
        && match stream.peek().kind {
            TokenKind::Newline | TokenKind::Eof | TokenKind::Comma | TokenKind::Equals => false,
            TokenKind::Ident(_) => !matches!(stream.peek_n(1).kind, TokenKind::Equals),
            _ => true,
        }
}

fn handle_parameter_redefinition(
    params: &ParamContext,
    diagnostics: &mut Vec<ParseDiagnostic>,
    origin: &NetlistSourceLocation,
    duplicate_name: String,
    kind: ParameterDefinitionKind,
    first_origin: Option<NetlistSourceLocation>,
) -> Result<(), ParseError> {
    let Some(first_origin) = first_origin else {
        return Ok(());
    };
    let canonical_name = duplicate_name.to_ascii_uppercase();
    match params.parameter_redefinition_diagnostic_policy() {
        ParameterRedefinitionDiagnosticPolicy::Silent => Ok(()),
        ParameterRedefinitionDiagnosticPolicy::Warning => {
            let selected = match params.parameter_redefinition_policy() {
                ParameterRedefinitionPolicy::UseFirst => "first",
                ParameterRedefinitionPolicy::UseLast => "last",
            };
            diagnostics.push(ParseDiagnostic::warning_at(
                origin.clone(),
                "parameter-redefinition",
                format!("Parameter {canonical_name} defined more than once. Using {selected} one."),
            ));
            Ok(())
        }
        ParameterRedefinitionDiagnosticPolicy::Error => Err(ParseError::ParameterRedefinition(
            Box::new(ParameterRedefinitionError {
                duplicate_name,
                canonical_name,
                kind,
                first_origin,
                duplicate_origin: origin.clone(),
            }),
        )),
    }
}

#[allow(clippy::too_many_arguments)]
fn parse_param_assignment_value(
    stream: &mut TokenStream,
    line_num: usize,
    params: &mut ParamContext,
    mut deferred_params: Option<&mut Vec<(String, String)>>,
    retain_global_expression: bool,
    name: String,
    override_value: Option<Value>,
    abort: &dyn AbortSignal,
) -> Result<Option<crate::netlist::expr::ExprError>, ParseWithAbortError> {
    let mut provisional_error = None;
    let override_binding = override_value.map(|value| (name.clone(), value));
    // Get the value (could be number, expression, or string-valued vector).
    match &stream.peek().kind {
        TokenKind::StringLit(value) => {
            let value = value.clone();
            stream.advance();
            if retain_global_expression {
                params.set_global_string(&name, value);
            } else {
                params.set_string(&name, value);
            }
            clear_deferred_param_expression(deferred_params.as_deref_mut(), &name);
        }
        TokenKind::Expression(expr)
            if !param_rhs_continues(stream) && params.get_string(expr).is_some() =>
        {
            let value = params
                .get_string(expr)
                .expect("string parameter presence checked")
                .to_string();
            stream.advance();
            if retain_global_expression {
                params.set_global_string(&name, value);
            } else {
                params.set_string(&name, value);
            }
            clear_deferred_param_expression(deferred_params.as_deref_mut(), &name);
        }
        _ if param_rhs_continues(stream) => {
            let expr = collect_param_rhs_expression(stream, line_num, &name)?;
            reject_parameter_expression_circuit_probe(&name, &expr, line_num, params, abort)?;
            if !retain_runtime_param_expression(
                params,
                deferred_params.as_deref_mut(),
                retain_global_expression,
                &name,
                &expr,
                abort,
            )? {
                match eval_and_bind_param_expression(
                    params,
                    &name,
                    &expr,
                    retain_global_expression,
                    deferred_params.is_none(),
                    abort,
                ) {
                    Ok(()) => {
                        upsert_deferred_param_expression(
                            deferred_params.as_deref_mut(),
                            &name,
                            &expr,
                        );
                    }
                    Err(crate::netlist::expr::ExpressionEvaluationError::Aborted) => {
                        return Err(ParseWithAbortError::Aborted);
                    }
                    Err(crate::netlist::expr::ExpressionEvaluationError::Expression(err)) => {
                        provisional_error = defer_param_expression_or_error(
                            deferred_params.as_deref_mut(),
                            params,
                            retain_global_expression,
                            name,
                            expr,
                            err,
                        )
                        .map_err(|error| {
                            ParseError::InvalidValue(format!("line {line_num}: {error}"))
                        })?;
                    }
                }
            }
        }
        TokenKind::Expression(expr) if params.get_string(expr).is_some() => {
            let value = params
                .get_string(expr)
                .expect("string parameter presence checked")
                .to_string();
            stream.advance();
            if retain_global_expression {
                params.set_global_string(&name, value);
            } else {
                params.set_string(&name, value);
            }
            clear_deferred_param_expression(deferred_params.as_deref_mut(), &name);
        }
        TokenKind::Expression(expr) => {
            let expr = expr.clone();
            stream.advance();
            reject_parameter_expression_circuit_probe(&name, &expr, line_num, params, abort)?;
            if !retain_runtime_param_expression(
                params,
                deferred_params.as_deref_mut(),
                retain_global_expression,
                &name,
                &expr,
                abort,
            )? {
                match eval_and_bind_param_expression(
                    params,
                    &name,
                    &expr,
                    retain_global_expression,
                    deferred_params.is_none(),
                    abort,
                ) {
                    Ok(()) => {
                        upsert_deferred_param_expression(
                            deferred_params.as_deref_mut(),
                            &name,
                            &expr,
                        );
                    }
                    Err(crate::netlist::expr::ExpressionEvaluationError::Aborted) => {
                        return Err(ParseWithAbortError::Aborted);
                    }
                    Err(crate::netlist::expr::ExpressionEvaluationError::Expression(err)) => {
                        provisional_error = defer_param_expression_or_error(
                            deferred_params.as_deref_mut(),
                            params,
                            retain_global_expression,
                            name,
                            expr,
                            err,
                        )
                        .map_err(|error| {
                            ParseError::InvalidValue(format!("line {line_num}: {error}"))
                        })?;
                    }
                }
            }
        }
        TokenKind::Ident(param_name)
            if params.expression_dialect() == ExpressionDialect::Xyce
                && crate::netlist::expr::runtime_special_quantity(param_name).is_some() =>
        {
            let expr = param_name.clone();
            stream.advance();
            retain_runtime_param_expression(
                params,
                deferred_params.as_deref_mut(),
                retain_global_expression,
                &name,
                &expr,
                abort,
            )?;
        }
        TokenKind::Ident(param_name) if params.get_complex(param_name).is_some() => {
            let expr = param_name.clone();
            stream.advance();
            eval_and_bind_param_expression(
                params,
                &name,
                &expr,
                retain_global_expression,
                deferred_params.is_none(),
                abort,
            )
            .map_err(|error| parameter_evaluation_error(line_num, error))?;
            upsert_deferred_param_expression(deferred_params.as_deref_mut(), &name, &expr);
        }
        _ => {
            let deferred_expr = simple_param_value_expression(stream, params);
            let capture = if !retain_global_expression && deferred_params.is_none() {
                deferred_expr
                    .as_deref()
                    .map(|expr| params.capture_statistical_parameter_expression(expr))
                    .transpose()
                    .map_err(|error| ParseError::InvalidValue(format!("line {line_num}: {error}")))?
                    .flatten()
            } else {
                None
            };
            let mut value_stream = stream.clone();
            match expect_value(&mut value_stream, line_num, params) {
                Ok(value) => {
                    *stream = value_stream;
                    if let Some(expr) = deferred_expr {
                        if retain_global_expression {
                            params.define_global_expression(
                                &name,
                                &expr,
                                Some(crate::netlist::expr::ComplexValue::from(value)),
                            );
                        } else {
                            params.set(&name, value);
                            params.retain_statistical_parameter_capture(&name, capture);
                        }
                        upsert_deferred_param_expression(
                            deferred_params.as_deref_mut(),
                            &name,
                            &expr,
                        );
                    } else {
                        if retain_global_expression {
                            params.set_global(&name, value);
                        } else {
                            params.set(&name, value);
                        }
                        clear_deferred_param_expression(deferred_params.as_deref_mut(), &name);
                    }
                }
                Err(err) => {
                    let expr = collect_param_rhs_expression(stream, line_num, &name)?;
                    reject_parameter_expression_circuit_probe(
                        &name, &expr, line_num, params, abort,
                    )?;
                    let Err(error) = crate::netlist::expr::eval_expression_complex_with_abort(
                        &expr,
                        &params.isolated_random_clone(),
                        abort,
                    ) else {
                        return Err(err.into());
                    };
                    let error = match error {
                        crate::netlist::expr::ExpressionEvaluationError::Aborted => {
                            return Err(ParseWithAbortError::Aborted);
                        }
                        crate::netlist::expr::ExpressionEvaluationError::Expression(error) => error,
                    };
                    provisional_error = defer_param_expression_or_error(
                        deferred_params,
                        params,
                        retain_global_expression,
                        name,
                        expr,
                        error,
                    )
                    .map_err(|_| err)?;
                }
            }
        }
    }
    if let Some((name, value)) = override_binding {
        // Validate and consume the authored default, including its random
        // draws, then install the study value before any dependent assignment.
        if retain_global_expression {
            params.set_global(&name, value);
        } else {
            params.set(&name, value);
        }
    }
    Ok(provisional_error)
}

fn parameter_evaluation_error(
    line: usize,
    error: crate::netlist::expr::ExpressionEvaluationError,
) -> ParseWithAbortError {
    match error {
        crate::netlist::expr::ExpressionEvaluationError::Aborted => ParseWithAbortError::Aborted,
        crate::netlist::expr::ExpressionEvaluationError::Expression(error) => {
            ParseError::InvalidValue(format!("line {line}: {error}")).into()
        }
    }
}

fn eval_and_bind_param_expression(
    params: &mut ParamContext,
    name: &str,
    expression: &str,
    global: bool,
    root_scope: bool,
    abort: &dyn AbortSignal,
) -> Result<(), crate::netlist::expr::ExpressionEvaluationError> {
    if crate::netlist::expr::needs_forward_reference_probe(expression, params) {
        // An unresolved declaration must not retain draws from a failed
        // partial evaluation. The full expression draws once when resolved.
        crate::netlist::expr::eval_expression_complex_with_abort(
            expression,
            &params.isolated_random_clone(),
            abort,
        )?;
    }
    // Capture before evaluation so random operators replay the same draw
    // positions without consuming additional draws from the authored stream.
    let capture = if root_scope && !global {
        params.capture_statistical_parameter_expression(expression)?
    } else {
        None
    };
    let (value, direction) = params.evaluate_parameter_binding_with_abort(expression, abort)?;
    if global {
        params.define_global_expression(name, expression, Some(value));
    } else {
        params.set_complex(name, value);
        params.retain_statistical_parameter_capture(name, capture);
    }
    params.retain_parameter_direction(name, global, direction);
    Ok(())
}

// Semantic preparation failures retain the authored expression for the normal
// parameter diagnostics. Cancellation must never fall back to another evaluator.
fn prepare_parameter_expression(
    expression: &str,
    params: &ParamContext,
    abort: &dyn AbortSignal,
) -> Result<String, ParseWithAbortError> {
    use crate::netlist::expr::{
        BehavioralPreparationError, prepare_behavioral_expression_with_abort,
    };
    match prepare_behavioral_expression_with_abort(expression, params, abort) {
        Ok(prepared) => Ok(prepared),
        Err(BehavioralPreparationError::Semantic(_)) => Ok(expression.to_owned()),
        Err(BehavioralPreparationError::Aborted) => Err(ParseWithAbortError::Aborted),
    }
}

fn reject_parameter_expression_circuit_probe(
    name: &str,
    expression: &str,
    line_num: usize,
    params: &ParamContext,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    let prepared = prepare_parameter_expression(expression, params, abort)?;
    let Some(probe) = crate::netlist::expr::parameter_expression_circuit_probe(&prepared) else {
        return Ok(());
    };
    Err(ParseError::Syntax {
        line: line_num,
        message: format!(
            "{} may not be used in parameter expression ({}): {}",
            probe.kind.diagnostic_label(),
            name.to_ascii_uppercase(),
            probe.reference
        ),
    }
    .into())
}

fn upsert_deferred_param_expression(
    deferred_params: Option<&mut Vec<(String, String)>>,
    name: &str,
    expr: &str,
) {
    if let Some(deferred_params) = deferred_params {
        upsert_param_expression(deferred_params, name.to_string(), expr.to_string());
    }
}

fn clear_deferred_param_expression(
    deferred_params: Option<&mut Vec<(String, String)>>,
    name: &str,
) {
    if let Some(deferred_params) = deferred_params {
        deferred_params.retain(|(existing, _)| !existing.eq_ignore_ascii_case(name));
    }
}

fn defer_param_expression_or_error(
    deferred_params: Option<&mut Vec<(String, String)>>,
    params: &mut ParamContext,
    retain_global_expression: bool,
    name: String,
    expr: String,
    err: crate::netlist::expr::ExprError,
) -> Result<Option<crate::netlist::expr::ExprError>, crate::netlist::expr::ExprError> {
    use crate::netlist::expr::ExprError;
    let provisional_error = match err {
        ExprError::UndefinedParam(_) => None,
        // These evaluations can be invalid only at the provisional TEMP/TNOM.
        // Retain the expression without a numeric placeholder, and retain the
        // failure even if a later assignment replaces the expression.
        ExprError::DivisionByZero | ExprError::InvalidArgument(_) => Some(err),
        error => return Err(error),
    };
    if retain_global_expression {
        params.define_global_expression(&name, &expr, None);
    } else {
        params.define_parameter_expression(&name, &expr, None);
    }
    upsert_deferred_param_expression(deferred_params, &name, &expr);
    Ok(provisional_error)
}

fn retain_runtime_param_expression(
    params: &mut ParamContext,
    deferred_params: Option<&mut Vec<(String, String)>>,
    global: bool,
    name: &str,
    expression: &str,
    abort: &dyn AbortSignal,
) -> Result<bool, ParseWithAbortError> {
    if params.expression_dialect() != ExpressionDialect::Xyce {
        return Ok(false);
    }
    let prepared = prepare_parameter_expression(expression, params, abort)?;
    if !crate::netlist::expr::behavioral_expression_references_runtime_quantity(&prepared) {
        return Ok(false);
    }
    if global {
        params.define_global_expression(name, expression, None);
    } else {
        params.define_parameter_expression(name, expression, None);
    }
    upsert_deferred_param_expression(deferred_params, name, expression);
    Ok(true)
}

fn parse_param_function_definition(
    stream: &mut TokenStream,
    line_num: usize,
    params: &mut ParamContext,
    func_name: String,
) -> Result<(), ParseError> {
    let args = parse_function_argument_list(stream, line_num, &func_name)?;

    if !stream.consume(&TokenKind::Equals) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Expected '=' after .PARAM function signature '{}(...)'",
                func_name
            ),
        });
    }

    let body = match &stream.peek().kind {
        TokenKind::Expression(expr) => {
            let body = expr.clone();
            stream.advance();
            body
        }
        _ => collect_param_rhs_expression(stream, line_num, &func_name)?,
    };

    if body.is_empty() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                ".PARAM function '{}' requires an expression body",
                func_name
            ),
        });
    }

    params.define_function(&func_name, args, &body);
    log::debug!("Defined .PARAM function: {}(...) = {}", func_name, body);

    Ok(())
}

fn parse_function_argument_list(
    stream: &mut TokenStream,
    line_num: usize,
    func_name: &str,
) -> Result<Vec<String>, ParseError> {
    if !stream.consume(&TokenKind::LParen) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Function '{}' requires argument list in parentheses",
                func_name
            ),
        });
    }

    let mut args = Vec::new();
    if stream.consume(&TokenKind::RParen) {
        return Ok(args);
    }

    loop {
        args.push(expect_ident(stream, line_num)?);

        if stream.consume(&TokenKind::Comma) {
            continue;
        }
        break;
    }

    if !stream.consume(&TokenKind::RParen) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "Expected ')' after function arguments".to_string(),
        });
    }

    Ok(args)
}

fn param_rhs_continues(stream: &TokenStream) -> bool {
    let mut depth = 0usize;
    let mut offset = 0usize;
    let mut saw_token = false;

    loop {
        match &stream.peek_n(offset).kind {
            TokenKind::Newline | TokenKind::Eof => return false,
            TokenKind::Comma if depth == 0 => return false,
            TokenKind::Ident(_) if saw_token && depth == 0 => {
                return !looks_like_param_entry_at(stream, offset);
            }
            TokenKind::LParen | TokenKind::LBracket => {
                if !saw_token {
                    return true;
                }
                if depth == 0 {
                    return true;
                }
                depth += 1;
            }
            TokenKind::RParen | TokenKind::RBracket => {
                depth = depth.saturating_sub(1);
            }
            TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash
                if saw_token && depth == 0 =>
            {
                return true;
            }
            TokenKind::Equals if saw_token => return true,
            TokenKind::Other(ch) if saw_token && param_rhs_operator_char(*ch) => return true,
            TokenKind::Expression(_) if saw_token => return true,
            _ => {}
        }

        saw_token = true;
        offset += 1;
    }
}

fn simple_param_value_expression(stream: &TokenStream, params: &ParamContext) -> Option<String> {
    match &stream.peek().kind {
        TokenKind::Ident(name)
            if params.get(name).is_some() || params.get_complex(name).is_some() =>
        {
            Some(name.clone())
        }
        _ => None,
    }
}

fn collect_param_rhs_expression(
    stream: &mut TokenStream,
    line_num: usize,
    name: &str,
) -> Result<String, ParseError> {
    let mut expression = String::new();
    let mut depth = 0usize;

    loop {
        match &stream.peek().kind {
            TokenKind::Newline | TokenKind::Eof => break,
            TokenKind::Comma if depth == 0 => break,
            TokenKind::Ident(_) if !expression.is_empty() && depth == 0 => {
                if looks_like_param_entry_at(stream, 0) {
                    break;
                }
            }
            _ => {}
        }

        let token = stream.peek().clone();
        match &token.kind {
            TokenKind::LParen | TokenKind::LBracket => depth += 1,
            TokenKind::RParen | TokenKind::RBracket => depth = depth.saturating_sub(1),
            _ => {}
        }

        append_param_rhs_token(&mut expression, &token.kind, &token.lexeme);
        stream.advance();
    }

    if expression.is_empty() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("Expected value for parameter '{}'", name),
        });
    }

    Ok(expression)
}

fn looks_like_param_entry_at(stream: &TokenStream, offset: usize) -> bool {
    matches!(stream.peek_n(offset).kind, TokenKind::Ident(_))
        && (matches!(stream.peek_n(offset + 1).kind, TokenKind::Equals)
            || looks_like_param_function_entry_at(stream, offset))
}

fn looks_like_param_function_entry_at(stream: &TokenStream, offset: usize) -> bool {
    if !matches!(stream.peek_n(offset).kind, TokenKind::Ident(_))
        || !matches!(stream.peek_n(offset + 1).kind, TokenKind::LParen)
    {
        return false;
    }

    let mut depth = 0usize;
    let mut cursor = offset + 1;
    loop {
        match &stream.peek_n(cursor).kind {
            TokenKind::LParen => depth += 1,
            TokenKind::RParen => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return matches!(stream.peek_n(cursor + 1).kind, TokenKind::Equals);
                }
            }
            TokenKind::Newline | TokenKind::Eof => return false,
            _ => {}
        }
        cursor += 1;
    }
}

fn param_rhs_token_fragment(kind: &TokenKind, lexeme: &str) -> String {
    match kind {
        TokenKind::Expression(expr) => format!("({expr})"),
        _ => lexeme.to_string(),
    }
}

fn append_param_rhs_token(expression: &mut String, kind: &TokenKind, lexeme: &str) {
    let fragment = param_rhs_token_fragment(kind, lexeme);
    if param_rhs_needs_space(expression, &fragment) {
        expression.push(' ');
    }
    expression.push_str(&fragment);
}

fn param_rhs_needs_space(expression: &str, fragment: &str) -> bool {
    let Some(prev) = expression.chars().rev().find(|ch| !ch.is_whitespace()) else {
        return false;
    };
    let Some(next) = fragment.chars().find(|ch| !ch.is_whitespace()) else {
        return false;
    };

    param_rhs_atom_char(prev) && param_rhs_atom_char(next)
}

fn param_rhs_atom_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.' | '#' | '%')
}

fn param_rhs_operator_char(ch: char) -> bool {
    matches!(ch, '>' | '<' | '!' | '?' | ':' | '&' | '|' | '^')
}

/// Parse .FUNC statement: .FUNC name(arg1, arg2, ...) = expression
/// or: .FUNC name(arg1, arg2, ...) {expression}
pub(super) fn parse_func_statement(
    stream: &mut TokenStream,
    line_num: usize,
    params: &mut ParamContext,
    root_redefinitions: Option<&mut RootRedefinitions>,
) -> Result<(), ParseError> {
    let func_name = expect_function_name(stream, line_num)?;

    let args = parse_function_argument_list(stream, line_num, &func_name)?;

    // The body is `{expression}`, `= expression`, or, as ngspice also
    // reads it, the bare rest of the card: `.FUNC f(x) x*2`.
    let body = match (&stream.peek().kind, &stream.peek_n(1).kind) {
        (TokenKind::Expression(expr), TokenKind::Newline | TokenKind::Eof) => {
            let body = expr.clone();
            stream.advance();
            body
        }
        _ => {
            stream.consume(&TokenKind::Equals);
            // Keep every token as written. The body used to be rebuilt from a
            // list of known token kinds, which dropped the rest: `x^2` read
            // as `x2`.
            let mut body = String::new();
            while !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
                let token = stream.peek().clone();
                append_param_rhs_token(&mut body, &token.kind, &token.lexeme);
                stream.advance();
            }
            body
        }
    };

    if body.is_empty() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(".FUNC {} requires an expression body", func_name),
        });
    }

    // Register the function
    let pinned = root_redefinitions.and_then(|redefinitions| {
        redefinitions.define(&func_name, RootDefinitionKind::Function, params)
    });
    params.define_function(&func_name, args, &body);
    log::debug!("Defined function: {}(...) = {}", func_name, body);
    if let Some(pin) = pinned {
        pin.bind(&func_name, RootDefinitionKind::Function, params);
    }

    Ok(())
}

fn expect_function_name(stream: &mut TokenStream, line_num: usize) -> Result<String, ParseError> {
    skip_commas(stream);

    let first = stream.peek().clone();
    let mut name = match &first.kind {
        TokenKind::Ident(_) => first.lexeme.clone(),
        TokenKind::AtSign => first.lexeme.clone(),
        TokenKind::Other(ch) if is_xyce_expr_ident_start(*ch) => first.lexeme.clone(),
        other => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Expected function name, found {:?}", other),
            });
        }
    };

    let mut end = first.span.end;
    stream.advance();

    loop {
        let token = stream.peek().clone();
        if token.span.start != end {
            break;
        }

        let Some(fragment) = function_name_continuation_fragment(&token) else {
            break;
        };

        name.push_str(fragment);
        end = token.span.end;
        stream.advance();
    }

    Ok(name.to_ascii_uppercase())
}

fn function_name_continuation_fragment(token: &crate::netlist::lexer::Token) -> Option<&str> {
    if token.lexeme.is_empty()
        || !token
            .lexeme
            .chars()
            .all(is_xyce_expr_ident_continue_after_start)
    {
        return None;
    }

    match token.kind {
        TokenKind::Ident(_) | TokenKind::Number(_) | TokenKind::AtSign | TokenKind::Other(_) => {
            Some(token.lexeme.as_str())
        }
        _ => None,
    }
}

fn is_xyce_expr_ident_start(ch: char) -> bool {
    matches!(ch, '`' | '@' | '#' | '$')
}

fn is_xyce_expr_ident_continue_after_start(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '_' | ':' | '`' | '@' | '#' | '.' | '$')
}

fn parse_dc_sweep_spec(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<(String, crate::netlist::DcSweepSpec), ParseError> {
    skip_commas(stream);
    let first = expect_element_name(stream, line_num)?;
    let first_upper = first.to_ascii_uppercase();

    if is_dc_sweep_type(&first_upper) {
        let source = expect_element_name(stream, line_num)?;
        let spec = parse_dc_sweep_spec_after_type(stream, line_num, params, &first_upper)?;
        return Ok((source, spec));
    }

    skip_commas(stream);
    if let TokenKind::Ident(kind) = &stream.peek().kind {
        let kind_upper = kind.to_ascii_uppercase();
        if is_dc_sweep_type(&kind_upper) {
            stream.advance();
            let spec = parse_dc_sweep_spec_after_type(stream, line_num, params, &kind_upper)?;
            return Ok((first, spec));
        }
    }

    let spec = parse_linear_dc_sweep_spec(stream, line_num, params)?;
    Ok((first, spec))
}

fn parse_dc_sweep_spec_after_type(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    kind: &str,
) -> Result<crate::netlist::DcSweepSpec, ParseError> {
    skip_commas(stream);
    match kind {
        "LIST" => {
            let mut values = Vec::new();
            while let Some(value) = try_signed_value(stream, params) {
                values.push(value);
                skip_commas(stream);
            }
            if values.is_empty() {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: ".DC LIST requires at least one value".to_string(),
                });
            }
            Ok(crate::netlist::DcSweepSpec::list(values))
        }
        "DEC" => {
            let start = expect_value(stream, line_num, params)?;
            let stop = expect_value(stream, line_num, params)?;
            let points = expect_positive_integer_value(
                expect_value(stream, line_num, params)?,
                line_num,
                ".DC DEC points parameter",
            )?;
            Ok(crate::netlist::DcSweepSpec::decade(start, stop, points))
        }
        "OCT" => {
            let start = expect_value(stream, line_num, params)?;
            let stop = expect_value(stream, line_num, params)?;
            let points = expect_positive_integer_value(
                expect_value(stream, line_num, params)?,
                line_num,
                ".DC OCT points parameter",
            )?;
            Ok(crate::netlist::DcSweepSpec::octave(start, stop, points))
        }
        "LIN" => parse_linear_dc_sweep_spec(stream, line_num, params),
        _ => unreachable!("validated DC sweep type"),
    }
}

fn parse_linear_dc_sweep_spec(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<crate::netlist::DcSweepSpec, ParseError> {
    let start = expect_value(stream, line_num, params)?;
    let stop = expect_value(stream, line_num, params)?;
    skip_commas(stream);
    if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: ".DC linear sweep requires a step value".to_string(),
        });
    }
    let step = expect_value(stream, line_num, params)?;
    Ok(crate::netlist::DcSweepSpec::linear(start, stop, step))
}

fn is_dc_sweep_type(kind: &str) -> bool {
    matches!(kind, "LIST" | "DEC" | "OCT" | "LIN")
}

fn expect_positive_integer_value(
    value: Value,
    line_num: usize,
    label: &str,
) -> Result<usize, ParseError> {
    if value.is_finite() && value >= 1.0 && value.fract().abs() <= Value::EPSILON {
        Ok(value as usize)
    } else {
        Err(ParseError::Syntax {
            line: line_num,
            message: format!("{label} must be a positive integer"),
        })
    }
}

/// Consume LTspice's `STARTUP` keyword on a `.TRAN` card.
fn consume_startup_keyword(stream: &mut TokenStream) -> bool {
    skip_commas(stream);
    if let TokenKind::Ident(word) = &stream.peek().kind
        && word.eq_ignore_ascii_case("STARTUP")
        && !matches!(stream.peek_n(1).kind, TokenKind::Equals)
    {
        stream.advance();
        return true;
    }
    false
}

/// Consume a trailing operating-point bypass keyword on a `.TRAN` card.
pub(super) fn consume_uic_keyword(stream: &mut TokenStream) -> bool {
    skip_commas(stream);
    if let TokenKind::Ident(word) = &stream.peek().kind
        && (word.eq_ignore_ascii_case("UIC") || word.eq_ignore_ascii_case("NOOP"))
    {
        stream.advance();
        return true;
    }
    false
}

/// Read one optional positional `.TRAN` field, stopping before a `KEY=VALUE`
/// pair.
///
/// `tstart` and `tmaxstep` are plain values, so a keyword is never one of
/// them. Without this guard a deck that also defines a parameter named after
/// a keyword would have the keyword's name consumed as a positional field.
fn try_positional_transient_value(
    stream: &mut TokenStream,
    params: &ParamContext,
) -> Option<Value> {
    skip_commas(stream);
    if matches!(stream.peek().kind, TokenKind::Ident(_))
        && matches!(stream.peek_n(1).kind, TokenKind::Equals)
    {
        return None;
    }
    try_value(stream, params)
}

/// Consume a `NOISE… =` pair and return the upper-cased keyword.
///
/// Any other token is left untouched: the `.TRAN` card defines no other
/// keyword, so a non-`NOISE` pair is reported by the line-consumed check that
/// already covers every unrecognized trailing field.
fn take_transient_noise_keyword(stream: &mut TokenStream) -> Option<String> {
    skip_commas(stream);
    let TokenKind::Ident(name) = &stream.peek().kind else {
        return None;
    };
    if !matches!(stream.peek_n(1).kind, TokenKind::Equals) {
        return None;
    }
    let keyword = name.to_ascii_uppercase();
    if !keyword.starts_with("NOISE") {
        return None;
    }
    stream.advance();
    stream.advance();
    Some(keyword)
}

/// Reject a `.TRAN` noise keyword the card already bound.
fn bind_transient_noise_keyword<T>(
    slot: &mut Option<T>,
    value: T,
    line_num: usize,
    keyword: &str,
) -> Result<(), ParseError> {
    if slot.is_some() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(".TRAN carries {keyword} more than once"),
        });
    }
    *slot = Some(value);
    Ok(())
}

/// Parse the `.TRAN` card's transient-noise keyword family, in any order and
/// interleaved with `UIC`/`NOOP` and LTspice's `STARTUP`.
///
/// `NOISEFMAX=` is what turns transient device noise on; the other three
/// modify a run that has it and are refused on their own, because a deck that
/// authored only `NOISESEED=` asked for something this card cannot mean.
fn parse_transient_noise_keywords(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    uic: &mut bool,
    startup: &mut bool,
) -> Result<Option<TransientNoiseConfig>, ParseError> {
    let mut fmax = None;
    let mut fmin = None;
    let mut seed = None;
    let mut scale = None;
    loop {
        if consume_uic_keyword(stream) {
            *uic = true;
            continue;
        }
        if consume_startup_keyword(stream) {
            *startup = true;
            continue;
        }
        let Some(keyword) = take_transient_noise_keyword(stream) else {
            break;
        };
        match keyword.as_str() {
            "NOISEFMAX" => bind_transient_noise_keyword(
                &mut fmax,
                expect_value(stream, line_num, params)?,
                line_num,
                "NOISEFMAX",
            )?,
            "NOISEFMIN" => bind_transient_noise_keyword(
                &mut fmin,
                expect_value(stream, line_num, params)?,
                line_num,
                "NOISEFMIN",
            )?,
            "NOISESEED" => bind_transient_noise_keyword(
                &mut seed,
                expect_u64_literal(stream, line_num, "NOISESEED")?,
                line_num,
                "NOISESEED",
            )?,
            "NOISESCALE" => bind_transient_noise_keyword(
                &mut scale,
                expect_value(stream, line_num, params)?,
                line_num,
                "NOISESCALE",
            )?,
            unknown => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        ".TRAN does not define the keyword {unknown}; the transient-noise \
                         keywords are NOISEFMAX, NOISEFMIN, NOISESEED and NOISESCALE"
                    ),
                });
            }
        }
    }
    let Some(fmax) = fmax else {
        if fmin.is_some() || seed.is_some() || scale.is_some() {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    ".TRAN transient-noise keywords require {}= to select the noise bandwidth",
                    TransientNoiseConfig::FMAX_KEYWORD
                ),
            });
        }
        return Ok(None);
    };
    let config = TransientNoiseConfig {
        fmax,
        fmin,
        seed,
        scale: scale.unwrap_or(1.0),
    };
    config.validate().map_err(|message| ParseError::Syntax {
        line: line_num,
        message,
    })?;
    Ok(Some(config))
}

#[cfg(test)]
mod tests {
    use crate::{
        Netlist,
        netlist::{
            AnalysisCommand, DcSweepMode, ParseError, PrintDelimiter, SaveSignal,
            TransientNoiseConfig,
        },
        numerics::integration::TransientErrorControl,
    };

    /// Parse one `.TRAN` card in a minimal deck and return the deck.
    fn transient_deck(card: &str) -> Result<Netlist, ParseError> {
        Netlist::parse(&format!(
            "transient noise card\nV1 in 0 1\nR1 in 0 1k\n{card}\n.END\n"
        ))
    }

    #[test]
    fn a_tran_card_carries_every_noise_keyword() {
        let netlist = transient_deck(
            ".TRAN 1n 1u 0 2n UIC NOISEFMAX=5e8 NOISEFMIN=1k NOISESEED=97 NOISESCALE=0.5",
        )
        .expect("every transient-noise keyword parses");
        assert!(matches!(
            netlist.analyses.as_slice(),
            [AnalysisCommand::Tran {
                step,
                stop,
                start: Some(start),
                max_step: Some(max_step),
                uic: true,
            }] if step.to_bits() == 1.0e-9f64.to_bits()
                && stop.to_bits() == 1.0e-6f64.to_bits()
                && start.to_bits() == 0.0f64.to_bits()
                && max_step.to_bits() == 2.0e-9f64.to_bits()
        ));
        assert_eq!(
            netlist.options.transient_noise,
            Some(TransientNoiseConfig {
                fmax: 5.0e8,
                fmin: Some(1.0e3),
                seed: Some(97),
                scale: 0.5,
            })
        );

        // The keywords are order-free and may surround `UIC`, and the
        // positional fields remain optional in front of them.
        let reordered = transient_deck(
            ".TRAN 1n 1u NOISESCALE=0.5 NOISESEED=97 UIC NOISEFMIN=1k NOISEFMAX=5e8",
        )
        .expect("the keyword tail is order-free");
        assert_eq!(
            reordered.options.transient_noise,
            netlist.options.transient_noise
        );
        assert!(matches!(
            reordered.analyses.as_slice(),
            [AnalysisCommand::Tran {
                start: None,
                max_step: None,
                uic: true,
                ..
            }]
        ));

        // NOISEFMAX alone is the whole request; the rest take their defaults.
        let minimal = transient_deck(".TRAN 1n 1u NOISEFMAX=1e9").expect("NOISEFMAX alone parses");
        assert_eq!(
            minimal.options.transient_noise,
            Some(TransientNoiseConfig {
                fmax: 1.0e9,
                fmin: None,
                seed: None,
                scale: 1.0,
            })
        );

        // A deck that never names the family asks for a deterministic run.
        let deterministic = transient_deck(".TRAN 1n 1u").expect("a plain .TRAN parses");
        assert_eq!(deterministic.options.transient_noise, None);
    }

    #[test]
    fn an_unknown_noise_keyword_on_a_tran_card_is_refused_by_name() {
        let error = transient_deck(".TRAN 1n 1u NOISEX=1e9")
            .expect_err("an unknown NOISE keyword is an error, not a warning");
        let ParseError::Syntax { message, .. } = &error else {
            panic!("expected a syntax error, got {error:?}");
        };
        assert!(
            message.contains("NOISEX") && message.contains("NOISEFMAX"),
            "the refusal must name the keyword and the family: {message}"
        );
    }

    #[test]
    fn transient_noise_keywords_require_the_noise_bandwidth() {
        for card in [
            ".TRAN 1n 1u NOISEFMIN=1k",
            ".TRAN 1n 1u NOISESEED=7",
            ".TRAN 1n 1u NOISESCALE=2",
        ] {
            let error = transient_deck(card).expect_err("NOISEFMAX is what turns noise on");
            let ParseError::Syntax { message, .. } = &error else {
                panic!("expected a syntax error for {card}, got {error:?}");
            };
            assert!(
                message.contains("NOISEFMAX"),
                "{card} must be refused by naming NOISEFMAX: {message}"
            );
        }
    }

    #[test]
    fn transient_noise_keyword_values_are_validated_on_the_card() {
        for (card, expected) in [
            (".TRAN 1n 1u NOISEFMAX=0", "NOISEFMAX"),
            (".TRAN 1n 1u NOISEFMAX=-1e9", "NOISEFMAX"),
            (".TRAN 1n 1u NOISEFMAX=1e9 NOISEFMIN=1e9", "NOISEFMIN"),
            (".TRAN 1n 1u NOISEFMAX=1e9 NOISEFMIN=2e9", "NOISEFMIN"),
            (".TRAN 1n 1u NOISEFMAX=1e9 NOISEFMIN=0", "NOISEFMIN"),
            (".TRAN 1n 1u NOISEFMAX=1e9 NOISESCALE=-1", "NOISESCALE"),
            (".TRAN 1n 1u NOISEFMAX=1e9 NOISEFMAX=2e9", "NOISEFMAX"),
        ] {
            let error = transient_deck(card).expect_err("an inadmissible value is refused");
            let ParseError::Syntax { message, .. } = &error else {
                panic!("expected a syntax error for {card}, got {error:?}");
            };
            assert!(
                message.contains(expected),
                "{card} must be refused by naming {expected}: {message}"
            );
        }
        // A seed is an integer literal, not a real.
        let error = transient_deck(".TRAN 1n 1u NOISEFMAX=1e9 NOISESEED=1.5")
            .expect_err("a fractional seed is refused");
        let ParseError::Syntax { message, .. } = &error else {
            panic!("expected a syntax error, got {error:?}");
        };
        assert!(message.contains("NOISESEED"), "{message}");
    }

    #[test]
    fn two_tran_cards_may_not_request_different_noise_realizations() {
        let shared = Netlist::parse(
            "two matching transient noise cards\n\
             V1 in 0 1\n\
             R1 in 0 1k\n\
             .TRAN 1n 1u NOISEFMAX=1e9 NOISESEED=4\n\
             .TRAN 1n 2u NOISEFMAX=1e9 NOISESEED=4\n\
             .END\n",
        )
        .expect("two cards that agree parse");
        assert_eq!(
            shared.options.transient_noise,
            Some(TransientNoiseConfig {
                fmax: 1.0e9,
                fmin: None,
                seed: Some(4),
                scale: 1.0,
            })
        );

        let error = Netlist::parse(
            "two conflicting transient noise cards\n\
             V1 in 0 1\n\
             R1 in 0 1k\n\
             .TRAN 1n 1u NOISEFMAX=1e9\n\
             .TRAN 1n 1u NOISEFMAX=2e9\n\
             .END\n",
        )
        .expect_err("two cards that disagree are refused");
        let ParseError::Syntax { message, .. } = &error else {
            panic!("expected a syntax error, got {error:?}");
        };
        assert!(message.contains("one noise realization"), "{message}");
    }

    #[test]
    fn print_delimiters_are_typed_without_polluting_saved_signals() {
        let netlist = Netlist::parse(
            "typed print delimiters\n\
             V1 out 0 1\n\
             .PRINT DC DELIMITER=coMmA FORMAT=STD V(out)\n\
             .PRINT DC DELIMITER=TAB WIDTH=17 I(V1)\n\
             .PRINT DC DELIMITER=COLON V(out)\n\
             .PRINT DC DELIMITER=SEMICOLON V(out)\n\
             .PRINT DC DELIMITER=\"|\" V(out)\n\
             .DC V1 0 1 1\n\
             .END\n",
        )
        .expect("the complete Xyce delimiter domain parses");

        let delimiters = netlist
            .output_requests
            .iter()
            .map(|request| request.print_delimiter.clone())
            .collect::<Vec<_>>();
        assert_eq!(
            delimiters,
            vec![
                Some(PrintDelimiter::Comma),
                Some(PrintDelimiter::Tab),
                Some(PrintDelimiter::Colon),
                Some(PrintDelimiter::Semicolon),
                Some(PrintDelimiter::Custom("|".to_string())),
            ]
        );
        assert_eq!(
            netlist.saves.signals,
            vec![
                SaveSignal::Voltage("out".to_string()),
                SaveSignal::Current("V1".to_string()),
                SaveSignal::Voltage("out".to_string()),
                SaveSignal::Voltage("out".to_string()),
                SaveSignal::Voltage("out".to_string()),
            ]
        );
        assert!(netlist.diagnostics.is_empty());
    }

    #[test]
    fn xyce_phase_output_radians_is_a_typed_output_option() {
        let netlist = Netlist::parse(
            "phase in radians\n\
             V1 out 0 AC 1\n\
             .OPTIONS OUTPUT PHASE_OUTPUT_RADIANS=true\n\
             .AC DEC 1 1 10\n\
             .PRINT AC VP(out)\n\
             .END\n",
        )
        .expect("the Xyce phase option parses");
        assert_eq!(netlist.options.output_phase_radians, Some(true));
        assert!(
            netlist.diagnostics.is_empty(),
            "a known option is not reported as ignored: {:?}",
            netlist.diagnostics
        );
        let mut merged = crate::netlist::SimulationOptions::default();
        merged.merge(&netlist.options);
        assert_eq!(merged.output_phase_radians, Some(true));
    }

    #[test]
    fn xyce_print_layout_and_output_framing_are_typed_last_authored() {
        let netlist = Netlist::parse(
            "typed Xyce output layout\n\
             V1 out 0 1\n\
             .OPTIONS OUTPUT PRINTHEADER=off PRINTFOOTER=0\n\
             .OPTIONS OUTPUT PRINTHEADER=yes PRINTFOOTER=true\n\
             .PRINT TRAN WIDTH=21 PRECISION=12 V(out)\n\
             .TRAN 1n 2n\n\
             .END\n",
        )
        .expect("Xyce boolean spellings and print layout parse");

        assert_eq!(netlist.options.output_print_header, Some(true));
        assert_eq!(netlist.options.output_print_footer, Some(true));
        let [request] = netlist.output_requests.as_slice() else {
            panic!("expected one typed output request");
        };
        assert_eq!(request.print_precision, Some(12));
        assert_eq!(request.print_width, Some(21));
        let mut merged = crate::netlist::SimulationOptions {
            output_print_header: Some(false),
            output_print_footer: Some(false),
            ..Default::default()
        };
        merged.merge(&netlist.options);
        assert_eq!(merged.output_print_header, Some(true));
        assert_eq!(merged.output_print_footer, Some(true));
        assert_eq!(netlist.options.output_phase_radians, None);
        assert_eq!(
            netlist.saves.signals,
            vec![SaveSignal::Voltage("out".to_string())]
        );
        assert!(netlist.diagnostics.is_empty());
    }

    #[test]
    fn xyce_print_integer_layout_uses_signed_i32_truncation() {
        for (authored, expected) in [
            ("0", 0),
            ("1", 1),
            ("16", 16),
            ("17", 17),
            ("12.9", 12),
            ("-1.9", -1),
        ] {
            let deck = format!(
                "print integer conversion\nV1 out 0 1\n.PRINT TRAN PRECISION={authored} WIDTH={authored} V(out)\n.TRAN 1n 2n\n.END\n"
            );
            let netlist = Netlist::parse(&deck)
                .unwrap_or_else(|error| panic!("layout value {authored} must parse: {error}"));
            let [request] = netlist.output_requests.as_slice() else {
                panic!("expected one typed output request");
            };
            assert_eq!(request.print_precision, Some(expected));
            assert_eq!(request.print_width, Some(expected));
        }

        for invalid in ["2147483648", "-2147483649"] {
            let deck = format!(
                "invalid print width\nV1 out 0 1\n.PRINT TRAN WIDTH={invalid} V(out)\n.TRAN 1n 2n\n.END\n"
            );
            assert!(
                Netlist::parse(&deck).is_err(),
                "WIDTH={invalid} must fail closed"
            );
        }
    }

    #[test]
    fn xyce_inconsistent_dc_sweeps_warn_and_preserve_one_start_point() {
        let options = crate::netlist::NetlistParseOptions {
            expression_dialect: crate::config::ExpressionDialect::Xyce,
            ..Default::default()
        };
        for (statement, label) in [
            (".DC V1 100 1 1", "Linear"),
            (".DC DEC V1 100 1 4", "Decade"),
            (".DC OCT V1 100 1 4", "Octave"),
        ] {
            let deck = format!(
                "inconsistent Xyce sweep\nV1 1 0 1\nR1 1 0 1k\n{statement}\n.PRINT DC V(1)\n.END\n"
            );
            let netlist = Netlist::parse_with_options(&deck, options)
                .unwrap_or_else(|error| panic!("{statement} failed to parse: {error}"));
            let [
                AnalysisCommand::Dc {
                    source,
                    start,
                    stop,
                    step,
                    mode,
                    sweep2: None,
                    ..
                },
            ] = netlist.analyses.as_slice()
            else {
                panic!("{statement} lost its sole DC analysis")
            };
            assert_eq!(source, "V1");
            let spec = crate::netlist::DcSweepSpec::linear(*start, *stop, *step);
            let spec = crate::netlist::DcSweepSpec {
                mode: mode.clone(),
                ..spec
            };
            assert_eq!(spec.points(), vec![100.0]);
            let [diagnostic] = netlist.diagnostics.as_slice() else {
                panic!("{statement} must emit one Xyce warning")
            };
            assert_eq!(diagnostic.code, "xyce-inconsistent-dc-sweep-direction");
            assert_eq!(diagnostic.line, 4);
            assert!(
                diagnostic
                    .message
                    .starts_with(&format!("{label} DC or STEP parameters for sweep over V1"))
            );
        }

        let generic = Netlist::parse("generic sweep\nV1 1 0 1\nR1 1 0 1k\n.DC V1 100 1 1\n.END\n")
            .expect("generic SPICE also retains the authored start point");
        assert!(generic.diagnostics.is_empty());
    }

    #[test]
    fn invalid_print_delimiter_warns_once_and_falls_back_to_whitespace() {
        let netlist = Netlist::parse(
            "invalid print delimiter\n\
             V1 out 0 1\n\
             .PRINT DC deLIMitEr=Fribble V(out)\n\
             .DC V1 0 1 1\n\
             .END\n",
        )
        .expect("invalid Xyce delimiter is a warning, not a parse failure");

        let [request] = netlist.output_requests.as_slice() else {
            panic!("expected one typed output request");
        };
        assert_eq!(request.print_delimiter, Some(PrintDelimiter::Whitespace));
        assert_eq!(
            netlist.saves.signals,
            vec![SaveSignal::Voltage("out".to_string())]
        );
        let [diagnostic] = netlist.diagnostics.as_slice() else {
            panic!("expected one invalid-delimiter warning");
        };
        assert_eq!(diagnostic.code, "xyce-invalid-print-delimiter");
        assert_eq!(
            diagnostic.message,
            "Invalid value of DELIMITER in .PRINT statment, ignoring"
        );
        assert_eq!(diagnostic.line, 3);
        assert_eq!(
            diagnostic.origin.as_ref().map(|origin| origin.line),
            Some(3)
        );
    }

    #[test]
    fn print_metadata_is_not_treated_as_an_output_dependency() {
        let netlist = Netlist::parse_validated(
            "print metadata isolation\n\
             V1 out 0 1\n\
             .PRINT DC DELIMITER=\"V(MISSING)\" FILE=\"I(NOSUCH)\" V(out)\n\
             .DC V1 0 1 1\n\
             .END\n",
        )
        .expect("accessor-shaped print metadata must not enter output validation");

        let [request] = netlist.output_requests.as_slice() else {
            panic!("expected one typed output request");
        };
        assert_eq!(
            request.print_delimiter,
            Some(PrintDelimiter::Custom("V(MISSING)".to_string()))
        );
        assert_eq!(request.dependencies.len(), 1);
        assert!(!format!("{:?}", request.dependencies).contains("MISSING"));
        assert!(!format!("{:?}", request.dependencies).contains("NOSUCH"));
    }

    #[test]
    fn unquoted_punctuated_print_metadata_is_consumed_as_one_authored_field() {
        let netlist = Netlist::parse_validated(
            "punctuated print metadata\n\
             V1 b 0 1\n\
             .PRINT AC FILE=ac-sens-step-gnuplot.cir.FD.SENS.splot.prn FORMAT=SPLOT NOINDEX=TRUE FUTURE_OPTION=1 VR(b)\n\
             .AC DEC 2 1 10\n\
             .END\n",
        )
        .expect("an unquoted Xyce FILE value may contain punctuation without becoming probes");

        let [request] = netlist.output_requests.as_slice() else {
            panic!("expected one typed output request");
        };
        assert_eq!(request.operands.len(), 1);
        assert_eq!(request.operands[0], "VR(b)");
        assert_eq!(request.dependencies.len(), 1);
        assert_eq!(
            netlist.saves.signals,
            vec![SaveSignal::Raw("VR(b)".to_string())]
        );
    }

    #[test]
    fn repeated_print_delimiters_use_last_assignment_with_invalid_fallback() {
        let netlist = Netlist::parse(
            "last delimiter wins\n\
             V1 out 0 1\n\
             .PRINT DC DELIMITER=COMMA DELIMITER=BAD V(out)\n\
             .PRINT DC DELIMITER=BAD DELIMITER=TAB V(out)\n\
             .DC V1 0 1 1\n\
             .END\n",
        )
        .expect("repeated delimiters retain Xyce last-write semantics");

        assert_eq!(
            netlist
                .output_requests
                .iter()
                .map(|request| request.print_delimiter.clone())
                .collect::<Vec<_>>(),
            vec![Some(PrintDelimiter::Whitespace), Some(PrintDelimiter::Tab)]
        );
        assert_eq!(
            netlist
                .diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.code == "xyce-invalid-print-delimiter")
                .count(),
            2
        );
    }

    #[test]
    fn unterminated_custom_print_delimiter_fails_lexing() {
        let error = Netlist::parse(
            "unterminated delimiter\n\
             V1 out 0 1\n\
             .PRINT DC DELIMITER=\"| V(out)\n\
             .DC V1 0 1 1\n\
             .END\n",
        )
        .expect_err("unterminated quoted delimiter must fail closed");
        assert!(
            error.to_string().contains("Unterminated string"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn dc_list_accepts_signed_values_without_leading_zero() {
        let netlist = Netlist::parse(
            "signed dc list\n\
             V1 out 0 0\n\
             .dc V1 LIST -.05 +.5 -1.0\n\
             .end\n",
        )
        .expect("signed .DC LIST values parse");

        let [crate::netlist::AnalysisCommand::Dc { source, mode, .. }] =
            netlist.analyses.as_slice()
        else {
            panic!("expected one .DC analysis, got {:?}", netlist.analyses);
        };
        assert_eq!(source, "V1");
        assert!(matches!(
            mode,
            DcSweepMode::List(values)
                if values
                    .iter()
                    .zip([-0.05, 0.5, -1.0])
                    .all(|(actual, expected)| (actual - expected).abs() < 1e-15)
        ));
    }

    #[test]
    fn harmonic_balance_command_and_hbint_orders_parse() {
        let netlist = Netlist::parse(
            "HB parser\n\
             V1 1 0 SIN(0 1 10k)\n\
             R1 1 0 1k\n\
             .param tone=10k\n\
             .hb {tone} 20k\n\
             .options hbint numfreq=50 numfreq2=25\n\
             .end\n",
        )
        .expect("valid multi-tone HB deck parses");

        let [crate::netlist::AnalysisCommand::Hb(card)] = netlist.analyses.as_slice() else {
            panic!("expected one HB analysis")
        };
        assert_eq!(card.frequencies, [10.0e3, 20.0e3]);
        assert_eq!(netlist.options.hb_num_frequencies, vec![50, 25]);
    }

    #[test]
    fn harmonic_balance_authored_options_are_typed_without_unknown_diagnostics() {
        let netlist = Netlist::parse(
            "typed HB options\n\
             V1 1 0 SIN(0 1 100k)\n\
             R1 1 0 1k\n\
             .hb 100k\n\
             .options hbint numfreq=5 saveicdata=1 tahb=0\n\
             .options nonlin-hb maxstep=2\n\
             .options linsol-hb prec_type=block_jacobi\n\
             .end\n",
        )
        .expect("BUG340 harmonic-balance options parse");

        assert_eq!(netlist.options.hb_num_frequencies, vec![5]);
        assert_eq!(netlist.options.hb_save_ic_data, Some(true));
        assert_eq!(
            netlist.options.hb_time_domain_mode,
            Some(crate::netlist::XyceHbTimeDomainMode::Direct)
        );
        assert_eq!(netlist.options.nonlin_hb_maxstep, Some(2));
        assert_eq!(
            netlist.options.linsol_hb_preconditioner,
            Some(crate::netlist::XyceHbPreconditioner::BlockJacobi)
        );
        assert!(
            netlist
                .diagnostics
                .iter()
                .all(|diagnostic| diagnostic.code != "unknown-option"),
            "typed HB options must not be diagnosed as unknown: {:?}",
            netlist.diagnostics
        );

        let mut merged = crate::netlist::SimulationOptions {
            hb_save_ic_data: Some(false),
            ..Default::default()
        };
        merged.merge(&netlist.options);
        assert_eq!(merged.hb_save_ic_data, Some(true));
        assert_eq!(
            merged.hb_time_domain_mode,
            Some(crate::netlist::XyceHbTimeDomainMode::Direct)
        );
        assert_eq!(merged.nonlin_hb_maxstep, Some(2));
        assert_eq!(
            merged.linsol_hb_preconditioner,
            Some(crate::netlist::XyceHbPreconditioner::BlockJacobi)
        );
    }

    #[test]
    fn harmonic_balance_preconditioner_rejects_unsupported_names() {
        let error = Netlist::parse(
            "invalid HB preconditioner\n\
             V1 1 0 1\n\
             R1 1 0 1k\n\
             .hb 100k\n\
             .options linsol-hb prec_type=ilu\n\
             .end\n",
        )
        .expect_err("unsupported HB preconditioners must fail closed");

        assert!(
            error.to_string().contains("expected BLOCK_JACOBI"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn harmonic_balance_tahb_and_nonlinear_budget_reject_malformed_values() {
        for (package, option) in [
            ("HBINT", "TAHB=-1"),
            ("HBINT", "TAHB=1.5"),
            ("HBINT", "TAHB=3"),
            ("NONLIN-HB", "MAXSTEP=0"),
            ("NONLIN-HB", "MAXSTEP=-1"),
            ("NONLIN-HB", "MAXSTEP=2.5"),
        ] {
            let deck = format!(
                "invalid typed HB option\nV1 1 0 0\nR1 1 0 1k\n.hb 1k\n.options {package} {option}\n.end\n"
            );
            assert!(
                Netlist::parse(&deck).is_err(),
                "{package} {option} must fail closed"
            );
        }
    }

    #[test]
    fn harmonic_balance_typed_modes_and_budget_merge_last_authored_values() {
        let mut merged = crate::netlist::SimulationOptions {
            hb_time_domain_mode: Some(crate::netlist::XyceHbTimeDomainMode::TransientAssisted),
            nonlin_hb_maxstep: Some(17),
            ..Default::default()
        };
        merged.merge(&crate::netlist::SimulationOptions {
            hb_time_domain_mode: Some(crate::netlist::XyceHbTimeDomainMode::DcOperatingPoint),
            nonlin_hb_maxstep: Some(2),
            ..Default::default()
        });

        assert_eq!(
            merged.hb_time_domain_mode,
            Some(crate::netlist::XyceHbTimeDomainMode::DcOperatingPoint)
        );
        assert_eq!(merged.nonlin_hb_maxstep, Some(2));
    }

    #[test]
    fn harmonic_balance_rejects_missing_or_invalid_frequency_and_order() {
        for line in [".hb", ".hb 0", ".hb -1k", ".hb 1e309"] {
            let deck = format!("invalid HB\nV1 1 0 0\nR1 1 0 1k\n{line}\n.end\n");
            assert!(Netlist::parse(&deck).is_err(), "{line} must fail");
        }
        for value in ["0", "1.5", "-2"] {
            let deck = format!(
                "invalid HBINT\nV1 1 0 0\nR1 1 0 1k\n.hb 1k\n.options hbint numfreq={value}\n.end\n"
            );
            assert!(Netlist::parse(&deck).is_err(), "NUMFREQ={value} must fail");
        }
    }

    fn deck_with_options(options: &str) -> String {
        format!(
            "options test\n\
             {options}\n\
             V1 1 0 1\n\
             R1 1 0 1k\n\
             .op\n\
             .end\n"
        )
    }

    #[test]
    fn options_record_the_continuation_ladder_damping_and_solver() {
        let netlist = Netlist::parse(&deck_with_options(
            ".options gminstepping=0 sourcestepping=0 pseudotransient=1 arclength=1\n\
             + damping=combined solver=klu",
        ))
        .expect("ladder, damping and solver options parse");

        assert_eq!(netlist.options.gmin_stepping, Some(false));
        assert_eq!(netlist.options.source_stepping, Some(false));
        assert_eq!(netlist.options.pseudo_transient, Some(true));
        assert_eq!(netlist.options.arc_length, Some(true));
        assert_eq!(
            netlist.options.damping_strategy,
            Some(crate::config::DampingStrategy::Combined)
        );
        assert_eq!(
            netlist.options.matrix_solver,
            Some(crate::solver::RealSolverBackend::Klu)
        );
    }

    #[test]
    fn ladder_options_accept_the_underscored_and_bare_flag_spellings() {
        let netlist = Netlist::parse(&deck_with_options(
            ".options gmin_stepping source_stepping=off pseudo_transient=false arc_length=yes",
        ))
        .expect("underscored ladder spellings parse");

        assert_eq!(netlist.options.gmin_stepping, Some(true));
        assert_eq!(netlist.options.source_stepping, Some(false));
        assert_eq!(netlist.options.pseudo_transient, Some(false));
        assert_eq!(netlist.options.arc_length, Some(true));
    }

    #[test]
    fn timeint_mintimestep_records_the_transient_step_floor() {
        let netlist = Netlist::parse(&deck_with_options(".options timeint mintimestep=2e-18"))
            .expect("TIMEINT MINTIMESTEP parses");

        assert_eq!(netlist.options.timeint_min_timestep, Some(2.0e-18));
    }

    #[test]
    fn the_run_ceiling_and_the_integrators_ceiling_are_two_separate_keys() {
        // Both clamp the transient step, and the engine applies them one after
        // the other. A deck stating both must therefore land two values, not
        // one: whichever key were to overwrite the other would turn a pair of
        // bounds into whichever happened to be parsed last.
        let netlist = Netlist::parse(&deck_with_options(
            ".options maxtimestep=4e-9\n.options timeint delmax=7e-9",
        ))
        .expect("both step ceilings parse");

        assert_eq!(netlist.options.max_timestep, Some(4.0e-9));
        assert_eq!(netlist.options.timeint_delmax, Some(7.0e-9));

        let only_delmax = Netlist::parse(&deck_with_options(".options timeint delmax=7e-9"))
            .expect("DELMAX alone parses");
        assert_eq!(only_delmax.options.max_timestep, None);

        let only_ceiling = Netlist::parse(&deck_with_options(".options max_timestep=4e-9"))
            .expect("the underscored run ceiling parses");
        assert_eq!(only_ceiling.options.max_timestep, Some(4.0e-9));
        assert_eq!(only_ceiling.options.timeint_delmax, None);
    }

    #[test]
    fn the_run_ceiling_is_not_a_timeint_key() {
        // Naming it inside the package would put two "largest step" keys in
        // one namespace. It falls to TIMEINT's unknown-key arm instead, which
        // reports the card rather than applying it somewhere unexpected.
        let netlist = Netlist::parse(&deck_with_options(".options timeint maxtimestep=4e-9"))
            .expect("an unknown TIMEINT key is reported, not fatal");

        assert_eq!(netlist.options.max_timestep, None);
        assert_eq!(netlist.options.timeint_delmax, None);
    }

    #[test]
    fn a_timeint_scoped_tolerance_never_reaches_the_global_one() {
        // The package selector is checked before the unscoped `(_, KEY)`
        // arms, and a package whose own arms do not cover a key falls through
        // to them. TIMEINT covers RELTOL, ABSTOL and MINTIMESTEP, so a card
        // that names only those must leave every global tolerance unstated.
        let netlist = Netlist::parse(&deck_with_options(
            ".options timeint reltol=1e-7 abstol=2e-13 mintimestep=3e-18",
        ))
        .expect("TIMEINT card parses");

        assert_eq!(netlist.options.timeint_reltol, Some(1.0e-7));
        assert_eq!(netlist.options.timeint_abstol, Some(2.0e-13));
        assert_eq!(netlist.options.timeint_min_timestep, Some(3.0e-18));
        assert_eq!(netlist.options.reltol, None);
        assert_eq!(netlist.options.abstol, None);
    }

    #[test]
    fn fused_option_suffix_is_unknown_instead_of_an_implicit_negative_value() {
        let netlist = Netlist::parse(&deck_with_options(
            ".options TIMEINT RELTOL=1e-6 ABSTOL-1e-6",
        ))
        .expect("a fused malformed option is nonfatal like Xyce");

        assert_eq!(netlist.options.timeint_reltol, Some(1.0e-6));
        assert_eq!(netlist.options.timeint_abstol, None);
        assert!(netlist.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "unknown-option"
                && diagnostic.message.contains("TIMEINT.ABSTOL-1E-6")
        }));
    }

    #[test]
    fn options_reject_unreadable_damping_and_solver_names() {
        for options in [
            ".options damping=quadratic",
            ".options damping=1",
            ".options solver=gmres",
        ] {
            let err = Netlist::parse(&deck_with_options(options))
                .expect_err("an unreadable strategy or backend name must fail parsing");
            assert!(
                err.to_string().contains("Syntax error"),
                "unexpected error for {options}: {err}"
            );
        }
    }

    #[test]
    fn options_reject_invalid_solver_real_values() {
        for options in [
            ".options reltol=0",
            ".options reltol=-1e-3",
            ".options abstol=0",
            ".options vntol=-1n",
            ".options iabstol=0",
            ".options residual_reltol=-1e-3",
            ".options trtol=0",
            ".options ramptime=-1e-9",
            ".options ramptime=1e309",
            ".options chgtol=-1e-15",
            ".options pivtol=0",
            ".options gmin=-1e-12",
            ".options rshunt=0",
            ".options rshunt=-1e9",
            ".options nonlin-tran reltol=0",
            ".options nonlin-tran abstol=-1e-6",
            ".options nonlin-tran deltaxtol=0",
            ".options nonlin-tran rhstol=1e309",
        ] {
            let err = Netlist::parse(&deck_with_options(options))
                .expect_err("invalid .OPTIONS value must fail parsing");
            assert!(
                err.to_string().contains("Syntax error"),
                "unexpected error for {options}: {err}"
            );
        }
    }

    #[test]
    fn options_parse_and_validate_ngspice_xmu() {
        for (authored, expected) in [("0", 0.0), (".49", 0.49), (".5", 0.5)] {
            let netlist = Netlist::parse(&deck_with_options(&format!(".options xmu={authored}")))
                .expect("valid XMU parses");
            assert_eq!(netlist.options.xmu, Some(expected));
        }

        for invalid in ["-1e-12", ".5000000000000001", "1e309"] {
            let error = Netlist::parse(&deck_with_options(&format!(".options xmu={invalid}")))
                .expect_err("out-of-domain XMU must fail parsing");
            assert!(
                error.to_string().contains("XMU"),
                "unexpected error: {error}"
            );
        }

        let xyce_options = crate::netlist::NetlistParseOptions {
            expression_dialect: crate::config::ExpressionDialect::Xyce,
            ..Default::default()
        };
        let xyce =
            Netlist::parse_with_options(&deck_with_options(".options xmu=.49"), xyce_options)
                .expect("Xyce treats ngspice XMU as an unknown option");
        assert!(xyce.options.xmu.is_none());
        assert!(xyce.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "unknown-option" && diagnostic.message.contains("XMU")
        }));
    }

    #[test]
    fn options_reject_non_finite_or_nonphysical_temperature() {
        for options in [
            ".options temp=1e309",
            ".options temp=-273.15",
            ".options tnom=1e309",
            ".options tnom=-300",
            ".options device temp=1e309",
            ".options device temp=-273.15",
            ".options device tnom=1e309",
            ".options device tnom=-300",
        ] {
            let err = Netlist::parse(&deck_with_options(options))
                .expect_err("invalid temperature option must fail parsing");
            assert!(
                err.to_string().contains("absolute zero") || err.to_string().contains("finite"),
                "unexpected error for {options}: {err}"
            );
        }
    }

    #[test]
    fn options_device_temperature_matches_spice_unscoped_temperature() {
        let unscoped = Netlist::parse(&deck_with_options(".options temp=35"))
            .expect("unscoped SPICE temperature option parses");
        let device_scoped = Netlist::parse(&deck_with_options(".options device temp=35"))
            .expect("Xyce DEVICE-scoped temperature option parses");

        assert_eq!(unscoped.options.temp, Some(35.0));
        assert_eq!(device_scoped.options.temp, Some(35.0));
        assert_eq!(
            unscoped.options.temp.map(f64::to_bits),
            device_scoped.options.temp.map(f64::to_bits),
            "the SPICE and Xyce spellings must resolve to the same run temperature"
        );
        assert_eq!(unscoped.diagnostics, device_scoped.diagnostics);
        for built_in in ["TEMP", "TEMPER", "VT"] {
            assert_eq!(
                unscoped.params.get(built_in).map(f64::to_bits),
                device_scoped.params.get(built_in).map(f64::to_bits),
                "the {built_in} built-in must be identical for both spellings"
            );
        }
    }

    #[test]
    fn options_temperature_is_not_accepted_under_an_unrelated_package() {
        let netlist = Netlist::parse(&deck_with_options(".options topology temp=35"))
            .expect("unknown scoped temperature is diagnosed without corrupting the deck");

        assert_eq!(netlist.options.temp, None);
        assert!(netlist.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "unknown-option" && diagnostic.message.contains("TOPOLOGY.TEMP")
        }));
    }

    fn deck_with_directives(directives: &str) -> String {
        format!(
            "temp directive test\n\
             {directives}\n\
             V1 1 0 1\n\
             R1 1 0 1k\n\
             .op\n\
             .end\n"
        )
    }

    #[test]
    fn temp_directive_sets_the_run_temperature() {
        let netlist =
            Netlist::parse(&deck_with_directives(".temp 85")).expect("`.temp` deck parses");
        assert_eq!(netlist.options.temp, Some(85.0));
        assert!(matches!(
            netlist.analyses.as_slice(),
            [AnalysisCommand::Temp { .. }, AnalysisCommand::Op]
        ));
    }

    #[test]
    fn temp_directive_wins_over_options_temp_in_either_order() {
        // ngspice reads `.temp` out of the whole deck and applies it after the
        // circuit exists, so authored order does not decide this.
        for directives in [".options temp=27\n.temp 85", ".temp 85\n.options temp=27"] {
            let netlist = Netlist::parse(&deck_with_directives(directives))
                .expect("mixed temperature deck parses");
            assert_eq!(
                netlist.options.temp,
                Some(85.0),
                "`.temp` must win for {directives:?}"
            );
        }
    }

    #[test]
    fn last_temp_directive_wins() {
        let netlist = Netlist::parse(&deck_with_directives(".temp 85\n.temp 0"))
            .expect("repeated `.temp` deck parses");
        assert_eq!(netlist.options.temp, Some(0.0));
    }

    #[test]
    fn temp_directive_accepts_sub_zero_corners_and_the_equals_spelling() {
        for (directive, expected) in [(".temp -40", -40.0), (".temp=125", 125.0)] {
            let netlist = Netlist::parse(&deck_with_directives(directive))
                .expect("`.temp` corner deck parses");
            assert_eq!(
                netlist.options.temp,
                Some(expected),
                "unexpected temperature for {directive:?}"
            );
        }
    }

    #[test]
    fn multi_valued_temp_directive_stays_a_sweep_and_leaves_the_run_temperature() {
        // The list is a temperature sweep the runners expand a point at a
        // time, so folding its first point into the single-run temperature
        // would silently pin every other point to it.
        let netlist = Netlist::parse(&deck_with_directives(".options temp=40\n.temp 25 50 85"))
            .expect("temperature sweep deck parses");
        assert_eq!(netlist.options.temp, Some(40.0));
        assert!(netlist.analyses.iter().any(|analysis| matches!(
            analysis,
            AnalysisCommand::Temp { temperatures } if temperatures.as_slice() == [25.0, 50.0, 85.0]
        )));
    }

    #[test]
    fn temp_directive_rejects_nonphysical_temperatures() {
        for directive in [".temp -273.15", ".temp 1e309"] {
            let err = Netlist::parse(&deck_with_directives(directive))
                .expect_err("invalid `.temp` value must fail parsing");
            assert!(
                err.to_string().contains("absolute zero") || err.to_string().contains("finite"),
                "unexpected error for {directive:?}: {err}"
            );
        }
    }

    #[test]
    fn options_parse_ramptime() {
        let netlist = Netlist::parse(&deck_with_options(".options ramptime=10n"))
            .expect("ramptime option parses");
        assert_eq!(netlist.options.ramptime, Some(10.0e-9));
    }

    #[test]
    fn options_accept_xyce_output_initial_interval_schedule() {
        let netlist = Netlist::parse(&deck_with_options(
            ".options output initial_interval=.001ms .5ms .01ms",
        ))
        .expect("Xyce OUTPUT INITIAL_INTERVAL schedule syntax parses");
        let schedule = netlist
            .options
            .output_interval_schedule
            .expect("OUTPUT interval schedule is retained");
        assert_eq!(schedule.initial_interval.to_bits(), 1.0e-6f64.to_bits());
        assert_eq!(schedule.intervals.len(), 1);
        assert_eq!(schedule.intervals[0].time.to_bits(), 5.0e-4f64.to_bits());
        assert_eq!(
            schedule.intervals[0].interval.to_bits(),
            1.0e-5f64.to_bits()
        );

        let bug456 = Netlist::parse(&deck_with_options(
            ".options output printheader=0 initial_interval=2e-8 \
             2e-4 1e-8 printfooter=0",
        ))
        .expect("named OUTPUT fields may surround the interval schedule");
        let schedule = bug456
            .options
            .output_interval_schedule
            .expect("BUG456 output cadence is typed");
        assert_eq!(schedule.initial_interval.to_bits(), 2.0e-8f64.to_bits());
        assert_eq!(schedule.intervals.len(), 1);
        assert_eq!(schedule.intervals[0].time.to_bits(), 2.0e-4f64.to_bits());
        assert_eq!(
            schedule.intervals[0].interval.to_bits(),
            1.0e-8f64.to_bits()
        );
        assert_eq!(bug456.options.output_print_header, Some(false));
        assert_eq!(bug456.options.output_print_footer, Some(false));
    }

    #[test]
    fn output_interval_schedule_rejects_incomplete_or_unordered_pairs() {
        for option in [
            ".options output initial_interval=1u 2u",
            ".options output initial_interval=1u 3u 1u 2u 1u",
            ".options output initial_interval=1u 2u 0",
        ] {
            let error = Netlist::parse(&deck_with_options(option))
                .expect_err("malformed OUTPUT cadence must fail closed");
            assert!(
                error.to_string().contains("OUTPUT"),
                "unexpected error for {option:?}: {error}"
            );
        }

        let error = Netlist::parse_with_options(
            &deck_with_options(".options output initial_interval=1u 2u 1u 4u 1u"),
            crate::netlist::NetlistParseOptions {
                resource_limits: crate::resource::ResourceLimits::default()
                    .with(|limits| limits.max_analysis_points = 1),
                ..crate::netlist::NetlistParseOptions::default()
            },
        )
        .expect_err("OUTPUT interval transitions share the analysis-point budget");
        assert!(error.to_string().contains("analysis_points"));
    }

    #[test]
    fn options_parse_the_exact_bug_1284_restart_controls() {
        let checkpoint_writer = Netlist::parse(&deck_with_options(
            ".options restart job=trans_test initial_interval=5n",
        ))
        .expect("BUG_1284 checkpoint-writer options parse");
        let writer = checkpoint_writer
            .options
            .restart
            .as_ref()
            .expect("RESTART package is typed");
        assert_eq!(writer.job.as_deref(), Some("trans_test"));
        assert_eq!(writer.initial_interval, Some(5.0e-9));
        assert_eq!(writer.file, None);

        let resumed = Netlist::parse(&deck_with_options(".options restart file=trans_test2e-08"))
            .expect("BUG_1284 resume-file options parse");
        let restart = resumed
            .options
            .restart
            .as_ref()
            .expect("RESTART package is typed");
        assert_eq!(restart.file.as_deref(), Some("trans_test2e-08"));
        assert!(
            resumed
                .diagnostics
                .iter()
                .all(|diagnostic| diagnostic.code != "unknown-option")
        );
    }

    #[test]
    fn xyce_restart_full_metadata_and_interval_schedule_are_typed() {
        let netlist = Netlist::parse(&deck_with_options(
            ".options restart pack=0 print_timeint_options=yes job=\"Nightly Run\" \
             start_time=20n file=\"checkpoints/run 20n.chk\" initial_interval=5n \
             10n 2n 20n 4n",
        ))
        .expect("complete Xyce 7.10 RESTART metadata parses");
        let restart = netlist
            .options
            .restart
            .as_ref()
            .expect("RESTART package is typed");

        assert_eq!(restart.pack, Some(false));
        assert_eq!(restart.print_timeint_options, Some(true));
        assert_eq!(restart.job.as_deref(), Some("Nightly Run"));
        assert_eq!(restart.start_time, Some(20.0e-9));
        assert_eq!(restart.file.as_deref(), Some("checkpoints/run 20n.chk"));
        assert_eq!(restart.initial_interval, Some(5.0e-9));
        assert_eq!(
            restart.intervals,
            vec![
                crate::netlist::XyceRestartInterval {
                    time: 10.0e-9,
                    interval: 2.0e-9,
                },
                crate::netlist::XyceRestartInterval {
                    time: 20.0e-9,
                    interval: 4.0e-9,
                },
            ]
        );
    }

    #[test]
    fn xyce_restart_unquoted_names_preserve_contiguous_source_spelling() {
        let netlist = Netlist::parse(&deck_with_options(
            r#".options restart job=Mixed_Case-Job file=C:\checkpoints\Mixed_Case-Job2e-08.bin"#,
        ))
        .expect("punctuation-rich restart names parse");
        let restart = netlist.options.restart.as_ref().expect("typed RESTART");

        assert_eq!(restart.job.as_deref(), Some("Mixed_Case-Job"));
        assert_eq!(
            restart.file.as_deref(),
            Some(r"C:\checkpoints\Mixed_Case-Job2e-08.bin")
        );
    }

    #[test]
    fn restart_names_cannot_leak_into_the_temperature_prescan() {
        let netlist = Netlist::parse(&deck_with_options(
            r#".options restart file=C:\temp\runs\checkpoint2e-08"#,
        ))
        .expect("a restart path containing a TEMP component parses");
        let restart = netlist.options.restart.as_ref().expect("typed RESTART");

        assert_eq!(
            restart.file.as_deref(),
            Some(r"C:\temp\runs\checkpoint2e-08")
        );
        assert_eq!(netlist.options.temp, None);
    }

    #[test]
    fn unknown_restart_keys_do_not_escape_into_global_options() {
        let netlist = Netlist::parse(&deck_with_options(
            ".options restart reltol=1e-6 method=gear",
        ))
        .expect("unknown RESTART keys are warnings like other option packages");

        assert_eq!(netlist.options.reltol, None);
        assert_eq!(netlist.options.method, None);
        assert!(netlist.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "unknown-option" && diagnostic.message.contains("RESTART.RELTOL")
        }));
        assert!(netlist.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "unknown-option" && diagnostic.message.contains("RESTART.METHOD")
        }));
    }

    #[test]
    fn repeated_xyce_restart_cards_merge_scalars_and_append_ordered_pairs() {
        let netlist = Netlist::parse(&deck_with_options(
            ".options restart job=first pack=1 initial_interval=5n 10n 2n\n\
             .options restart job=second print_timeint_options=1 20n 4n",
        ))
        .expect("repeated restart cards merge");
        let restart = netlist.options.restart.as_ref().expect("typed RESTART");

        assert_eq!(restart.job.as_deref(), Some("second"));
        assert_eq!(restart.pack, Some(true));
        assert_eq!(restart.print_timeint_options, Some(true));
        assert_eq!(restart.initial_interval, Some(5.0e-9));
        assert_eq!(restart.intervals.len(), 2);
        assert_eq!(restart.intervals[0].time, 10.0e-9);
        assert_eq!(restart.intervals[1].time, 20.0e-9);
    }

    #[test]
    fn simulation_option_merge_overrides_a_restart_schedule_as_one_unit() {
        let mut merged = crate::netlist::SimulationOptions {
            restart: Some(crate::netlist::XyceRestartOptions {
                pack: Some(true),
                job: Some("base".to_string()),
                initial_interval: Some(1.0e-9),
                intervals: vec![crate::netlist::XyceRestartInterval {
                    time: 10.0e-9,
                    interval: 2.0e-9,
                }],
                ..crate::netlist::XyceRestartOptions::default()
            }),
            ..crate::netlist::SimulationOptions::default()
        };
        merged.merge(&crate::netlist::SimulationOptions {
            restart: Some(crate::netlist::XyceRestartOptions {
                pack: Some(false),
                file: Some("resume.chk".to_string()),
                intervals: vec![crate::netlist::XyceRestartInterval {
                    time: 20.0e-9,
                    interval: 4.0e-9,
                }],
                ..crate::netlist::XyceRestartOptions::default()
            }),
            ..crate::netlist::SimulationOptions::default()
        });
        let restart = merged.restart.as_ref().expect("merged RESTART");

        assert_eq!(restart.pack, Some(false));
        assert_eq!(restart.job.as_deref(), Some("base"));
        assert_eq!(restart.file.as_deref(), Some("resume.chk"));
        assert_eq!(restart.initial_interval, Some(1.0e-9));
        assert_eq!(restart.intervals.len(), 1);
        assert_eq!(restart.intervals[0].time, 20.0e-9);
    }

    #[test]
    fn xyce_restart_rejects_malformed_or_nonphysical_metadata() {
        for options in [
            ".options restart pack=2",
            ".options restart pack=maybe",
            ".options restart print_timeint_options=.5",
            ".options restart job=\"\"",
            ".options restart job=\"   \"",
            ".options restart file=\"resume.chk\"pack=1",
            ".options restart file=",
            ".options restart start_time=-1n",
            ".options restart start_time=1e309",
            ".options restart initial_interval=0",
            ".options restart initial_interval=1e309",
            ".options restart -1n 2n",
            ".options restart 1e309 2n",
            ".options restart 1n 0",
            ".options restart 1n 1e309",
            ".options restart 1n",
            ".options restart 2n 1n 1n 1n",
            ".options restart 1n 1n 1n 2n",
        ] {
            let error = Netlist::parse(&deck_with_options(options))
                .expect_err("invalid RESTART metadata must fail parsing");
            assert!(
                error.to_string().contains("RESTART"),
                "unexpected error for {options:?}: {error}"
            );
        }
    }

    #[test]
    fn xyce_restart_interval_pairs_obey_the_analysis_point_limit() {
        let error = Netlist::parse_with_options(
            &deck_with_options(".options restart initial_interval=1n 10n 2n 20n 4n"),
            crate::netlist::NetlistParseOptions {
                resource_limits: crate::resource::ResourceLimits::default()
                    .with(|limits| limits.max_analysis_points = 1),
                ..crate::netlist::NetlistParseOptions::default()
            },
        )
        .expect_err("restart schedules must honor the parser resource limit");
        assert!(matches!(
            error,
            ParseError::ResourceLimit(crate::resource::ResourceLimitError {
                resource: crate::resource::ResourceKind::AnalysisPoints,
                requested: 2,
                limit: 1,
            })
        ));
    }

    #[test]
    fn restart_and_transient_point_schedules_share_one_order_independent_limit() {
        for options in [
            ".options restart 10n 2n\n.options output outputtimepoints=20n",
            ".options output outputtimepoints=20n\n.options restart 10n 2n",
        ] {
            let error = Netlist::parse_with_options(
                &deck_with_options(options),
                crate::netlist::NetlistParseOptions {
                    resource_limits: crate::resource::ResourceLimits::default()
                        .with(|limits| limits.max_analysis_points = 1),
                    ..crate::netlist::NetlistParseOptions::default()
                },
            )
            .expect_err("all retained transient schedules share the point limit");
            assert!(
                matches!(
                    error,
                    ParseError::ResourceLimit(crate::resource::ResourceLimitError {
                        resource: crate::resource::ResourceKind::AnalysisPoints,
                        requested: 2,
                        limit: 1,
                    })
                ),
                "unexpected limit error for {options:?}: {error}"
            );
        }
    }

    #[test]
    fn xyce_output_and_timeint_time_point_lists_are_typed_and_canonical() {
        let netlist = Netlist::parse(&deck_with_options(
            ".options output outputtimepoints=4ms,1ms,1ms,-0\n\
             .options output outputtimepoints=3ms,2ms\n\
             .options timeint breakpoints=4ms,2ms,2ms\n\
             .options timeint breakpoints=3ms,1ms",
        ))
        .expect("Xyce transient time-point lists parse");

        assert_eq!(
            netlist.options.output_time_points,
            vec![0.0, 1.0e-3, 2.0e-3, 3.0e-3, 4.0e-3]
        );
        assert_eq!(
            netlist.options.timeint_breakpoints,
            vec![1.0e-3, 2.0e-3, 3.0e-3, 4.0e-3]
        );
        assert_eq!(
            netlist.options.output_time_points[0].to_bits(),
            0.0f64.to_bits()
        );
        assert!(
            netlist
                .diagnostics
                .iter()
                .all(|diagnostic| diagnostic.code != "unknown-option")
        );
    }

    #[test]
    fn xyce_timeint_error_control_and_companions_are_typed_first_value_wins() {
        let netlist = Netlist::parse(&deck_with_options(
            ".options timeint erroption=1 mintimestepsbp=12 nlmin=2 nlmax=9 timestepsreversal=1 minord=2 maxord=2\n\
             .options timeint erroption=7 mintimestepsbp=-20 nlmin=-4 nlmax=1.5 timestepsreversal=8 minord=0 maxord=3",
        ))
        .expect("Xyce TIMEINT iteration control parses");

        assert_eq!(
            netlist.options.timeint_error_control,
            Some(TransientErrorControl::NonlinearIterations)
        );
        assert_eq!(
            netlist.options.timeint_min_steps_between_breakpoints,
            Some(12)
        );
        assert_eq!(netlist.options.timeint_nlmin, Some(2));
        assert_eq!(netlist.options.timeint_nlmax, Some(9));
        assert_eq!(netlist.options.timeint_timesteps_reversal, Some(true));
        assert_eq!(netlist.options.timeint_min_order, Some(2));
        assert_eq!(netlist.options.timeint_max_order, Some(2));
        assert_eq!(
            netlist
                .diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.code == "duplicate-option")
                .count(),
            7
        );

        let zero_then_one = Netlist::parse(&deck_with_options(
            ".options timeint erroption=0\n.options timeint erroption=1",
        ))
        .expect("reverse duplicate order parses");
        assert_eq!(
            zero_then_one.options.timeint_error_control,
            Some(TransientErrorControl::LocalTruncation),
            "zero followed by one must retain the first packaged value"
        );
    }

    #[test]
    fn xyce_timeint_nonlinear_return_codes_take_any_nonzero_integer_first_value_wins() {
        let netlist = Netlist::parse(&deck_with_options(
            ".options timeint nlnearconv=1 nlsmallupdate=0\n\
             .options timeint nlnearconv=0 nlsmallupdate=1",
        ))
        .expect("Xyce TIMEINT nonlinear return codes parse");
        assert_eq!(netlist.options.timeint_nl_near_convergence, Some(true));
        assert_eq!(netlist.options.timeint_nl_small_update, Some(false));
        assert_eq!(
            netlist
                .diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.code == "duplicate-option")
                .count(),
            2
        );
        assert!(
            netlist
                .diagnostics
                .iter()
                .all(|diagnostic| diagnostic.code != "unknown-option"),
            "{:?}",
            netlist.diagnostics
        );

        // Param::getImmutableValue<int>() truncates, and any nonzero result
        // selects the positive code (N_NLS_Manager.C:281, :288).
        let truncated = Netlist::parse(&deck_with_options(
            ".options timeint nlnearconv=2.7 nlsmallupdate=0.5",
        ))
        .expect("real-valued return-code options parse");
        assert_eq!(truncated.options.timeint_nl_near_convergence, Some(true));
        assert_eq!(truncated.options.timeint_nl_small_update, Some(false));

        let unset = Netlist::parse(&deck_with_options(".options timeint reltol=1e-3"))
            .expect("deck without the return-code options parses");
        assert_eq!(unset.options.timeint_nl_near_convergence, None);
        assert_eq!(unset.options.timeint_nl_small_update, None);
    }

    #[test]
    fn xyce_timeint_iteration_control_rejects_invalid_integer_domains_and_ranges() {
        for options in [
            ".options timeint erroption=2",
            ".options timeint erroption=.5",
            ".options timeint mintimestepsbp=1.5",
            ".options timeint mintimestepsbp=1.0000000001",
            ".options timeint mintimestepsbp=2147483648",
            ".options timeint nlmin=-1",
            ".options timeint nlmax=2147483648",
            ".options timeint nlmax=2",
            ".options timeint timestepsreversal=2",
            ".options timeint minord=0",
            ".options timeint maxord=3",
            ".options timeint minord=2 maxord=1",
        ] {
            Netlist::parse(&deck_with_options(options))
                .expect_err("invalid Xyce TIMEINT integer option must fail closed");
        }

        let valid = Netlist::parse(&deck_with_options(".options timeint nlmin=9 nlmax=9"))
            .expect("equal iteration thresholds are valid");
        assert_eq!(valid.options.timeint_nlmin, Some(9));
        assert_eq!(valid.options.timeint_nlmax, Some(9));

        let zero = Netlist::parse(&deck_with_options(
            ".options timeint erroption=1 mintimestepsbp=0",
        ))
        .expect("explicit zero disables MINTIMESTEPSBP");
        assert_eq!(zero.options.timeint_min_steps_between_breakpoints, Some(0));

        let split = Netlist::parse(&deck_with_options(
            ".options timeint nlmin=9 minord=2\n.options timeint nlmax=9 maxord=2",
        ))
        .expect("valid aggregate split across TIMEINT cards is finalized once");
        assert_eq!(split.options.timeint_nlmin, Some(9));
        assert_eq!(split.options.timeint_nlmax, Some(9));
    }

    #[test]
    fn xyce_timeint_iteration_control_ast_merge_preserves_first_package_values() {
        let mut merged = crate::netlist::SimulationOptions {
            timeint_error_control: Some(TransientErrorControl::NonlinearIterations),
            timeint_min_steps_between_breakpoints: Some(12),
            timeint_nlmin: Some(2),
            timeint_nlmax: Some(9),
            timeint_timesteps_reversal: Some(true),
            timeint_min_order: Some(2),
            timeint_max_order: Some(2),
            ..crate::netlist::SimulationOptions::default()
        };
        merged.merge(&crate::netlist::SimulationOptions {
            timeint_error_control: Some(TransientErrorControl::LocalTruncation),
            timeint_min_steps_between_breakpoints: Some(20),
            timeint_nlmin: Some(4),
            timeint_nlmax: Some(10),
            timeint_timesteps_reversal: Some(false),
            timeint_min_order: Some(1),
            timeint_max_order: Some(1),
            ..crate::netlist::SimulationOptions::default()
        });

        assert_eq!(
            merged.timeint_error_control,
            Some(TransientErrorControl::NonlinearIterations)
        );
        assert_eq!(merged.timeint_min_steps_between_breakpoints, Some(12));
        assert_eq!(merged.timeint_nlmin, Some(2));
        assert_eq!(merged.timeint_nlmax, Some(9));
        assert_eq!(merged.timeint_timesteps_reversal, Some(true));
        assert_eq!(merged.timeint_min_order, Some(2));
        assert_eq!(merged.timeint_max_order, Some(2));
    }

    #[test]
    fn xyce_time_point_lists_reject_malformed_values_and_interval_conflicts() {
        for options in [
            ".options output outputtimepoints=",
            ".options output outputtimepoints=1ms,",
            ".options output outputtimepoints=-1ms",
            ".options timeint breakpoints=1ms,",
            ".options timeint breakpoints=-1ms",
        ] {
            Netlist::parse(&deck_with_options(options))
                .expect_err("malformed transient time-point list must fail parsing");
        }

        for options in [
            ".options output initial_interval=1ms\n.options output outputtimepoints=2ms",
            ".options output outputtimepoints=2ms\n.options output initial_interval=1ms",
        ] {
            let error = Netlist::parse(&deck_with_options(options))
                .expect_err("OUTPUT interval and point schedules are mutually exclusive");
            assert!(error.to_string().contains("INITIAL_INTERVAL"));
            assert!(error.to_string().contains("OUTPUTTIMEPOINTS"));
        }
    }

    #[test]
    fn xyce_time_point_lists_share_the_analysis_point_resource_limit() {
        let error = Netlist::parse_with_options(
            &deck_with_options(
                ".options output outputtimepoints=1ms,2ms\n\
                 .options timeint breakpoints=3ms,4ms",
            ),
            crate::netlist::NetlistParseOptions {
                resource_limits: crate::resource::ResourceLimits::default()
                    .with(|limits| limits.max_analysis_points = 3),
                ..crate::netlist::NetlistParseOptions::default()
            },
        )
        .expect_err("combined transient schedules must honor the parser resource limit");
        assert!(matches!(
            error,
            ParseError::ResourceLimit(crate::resource::ResourceLimitError {
                resource: crate::resource::ResourceKind::AnalysisPoints,
                requested: 4,
                limit: 3,
            })
        ));
    }

    #[test]
    fn options_parse_xspice_digital_delay_type() {
        let netlist = Netlist::parse(&deck_with_options(".options digital_delay_type=3"))
            .expect("digital_delay_type option parses");
        assert_eq!(netlist.options.digital_delay_type, Some(3));

        let err = Netlist::parse(&deck_with_options(".options digital_delay_type=4"))
            .expect_err("invalid digital_delay_type must fail parsing");
        assert!(
            err.to_string().contains("DIGITAL_DELAY_TYPE"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn options_parse_xyce_numeric_timeint_method() {
        let netlist = Netlist::parse(&deck_with_options(
            ".options timeint method=7 newlte=1 newbpstepping=1",
        ))
        .expect("Xyce numeric TIMEINT method selector parses");

        assert_eq!(netlist.options.method.as_deref(), Some("7"));
        assert_eq!(
            netlist.options.transient_lte_reference,
            Some(super::TransientLteReference::PointGlobal)
        );
        assert_eq!(netlist.options.transient_new_bp_stepping, Some(true));
        assert!(netlist.diagnostics.is_empty());
    }

    #[test]
    fn options_parse_and_validate_xyce_new_breakpoint_stepping() {
        for (selector, expected) in [(0, false), (1, true)] {
            let netlist = Netlist::parse(&deck_with_options(&format!(
                ".options timeint newbpstepping={selector}"
            )))
            .expect("supported Xyce NEWBPSTEPPING selector parses");
            assert_eq!(netlist.options.transient_new_bp_stepping, Some(expected));
            assert!(netlist.diagnostics.is_empty());
        }

        for invalid in ["-1", "0.0000000001", "1.0000000001", "2", "1e309"] {
            let err = Netlist::parse(&deck_with_options(&format!(
                ".options timeint newbpstepping={invalid}"
            )))
            .expect_err("invalid Xyce NEWBPSTEPPING selector must fail parsing");
            assert!(
                err.to_string().contains("NEWBPSTEPPING"),
                "unexpected error: {err}"
            );
        }
    }

    #[test]
    fn options_parse_and_validate_xyce_newlte_reference_modes() {
        for (selector, expected) in [
            (0, super::TransientLteReference::PointLocal),
            (1, super::TransientLteReference::PointGlobal),
            (2, super::TransientLteReference::SignalGlobal),
            (3, super::TransientLteReference::SignalLocal),
        ] {
            let netlist = Netlist::parse(&deck_with_options(&format!(
                ".options timeint newlte={selector}"
            )))
            .expect("supported Xyce NEWLTE selector parses");
            assert_eq!(netlist.options.transient_lte_reference, Some(expected));
            assert!(netlist.diagnostics.is_empty());
        }

        for invalid in ["-1", "1.0000000001", "1.5", "4", "1e309"] {
            let err = Netlist::parse(&deck_with_options(&format!(
                ".options timeint newlte={invalid}"
            )))
            .expect_err("invalid Xyce NEWLTE selector must fail parsing");
            assert!(
                err.to_string().contains("NEWLTE"),
                "unexpected error: {err}"
            );
        }

        Netlist::parse(&deck_with_options(".options timeint newlte=missing"))
            .expect_err("a malformed Xyce NEWLTE selector must fail parsing");
    }

    #[test]
    fn timeint_tolerances_remain_separate_from_generic_solver_tolerances() {
        let netlist = Netlist::parse(&deck_with_options(
            ".options timeint reltol=2e-6 abstol=3e-9 delmax=4e-8 useDeviceMax=0 newlte=2",
        ))
        .expect("TIMEINT tolerances parse");

        assert_eq!(netlist.options.timeint_reltol, Some(2.0e-6));
        assert_eq!(netlist.options.timeint_abstol, Some(3.0e-9));
        assert_eq!(netlist.options.timeint_delmax, Some(4.0e-8));
        assert_eq!(netlist.options.timeint_use_device_max_timestep, Some(false));
        assert_eq!(netlist.options.reltol, None);
        assert_eq!(netlist.options.abstol, None);
        assert_eq!(
            netlist.options.transient_lte_reference,
            Some(super::TransientLteReference::SignalGlobal)
        );
    }

    #[test]
    fn timeint_use_device_max_accepts_xyce_boolean_forms() {
        for (option, expected) in [
            ("USEDEVICEMAX", true),
            ("USEDEVICEMAX=1", true),
            ("USEDEVICEMAX=0", false),
            ("USEDEVICEMAX=TRUE", true),
            ("USEDEVICEMAX=FALSE", false),
        ] {
            let netlist = Netlist::parse(&deck_with_options(&format!(".options timeint {option}")))
                .expect("Xyce USEDEVICEMAX option parses");
            assert_eq!(
                netlist.options.timeint_use_device_max_timestep,
                Some(expected)
            );
        }
    }

    #[test]
    fn measure_measfail_option_preserves_xyce_output_policy() {
        for (value, expected) in [(0.0, false), (0.5, false), (1.0, true), (1.9, true)] {
            let netlist = Netlist::parse(&deck_with_options(&format!(
                ".options measure measfail={value}"
            )))
            .expect("Xyce MEASURE MEASFAIL option parses");
            assert_eq!(netlist.options.measure_fail_output, Some(expected));
            assert!(netlist.diagnostics.is_empty());
        }

        let defaulted = Netlist::parse(&deck_with_options(".options measure measfail=2"))
            .expect("Xyce defaults invalid MEASFAIL values to enabled");
        assert_eq!(defaulted.options.measure_fail_output, Some(true));
        assert!(defaulted.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "invalid-option-defaulted"
                && diagnostic.message.contains("MEASURE.MEASFAIL")
        }));

        let global_default = Netlist::parse(&deck_with_options(
            ".options measure measfail=0 default_val=-10",
        ))
        .expect("global Xyce measurement default parses");
        assert_eq!(global_default.options.measure_fail_output, Some(false));
        assert_eq!(global_default.options.measure_default_value, Some(-10.0));

        for options in [
            ".options measure measfail=1e309",
            ".options measure default_val=1e309",
        ] {
            let error = Netlist::parse(&deck_with_options(options))
                .expect_err("non-finite MEASURE option must fail parsing");
            assert!(error.to_string().contains("finite"));
        }
    }

    #[test]
    fn measure_base_qualifiers_are_order_independent_and_do_not_capture_node_names() {
        let netlist = Netlist::parse(
            "interleaved measurement qualifiers\n\
             V1 DEFAULT_VAL 0 1\n\
             .dc V1 0 1 1\n\
             .measure dc sample AVG V(DEFAULT_VAL) DEFAULT_VAL=2 FROM=0.25 DEFAULT_VAL=3 TO=0.75\n\
             .end\n",
        )
        .expect("statement-wide qualifiers parse around derived qualifiers");
        let statement = &netlist.measurements[0];
        assert_eq!(statement.default_value, Some(3.0));
        let crate::netlist::measure::MeasureType::Avg {
            signal, from, to, ..
        } = &statement.measure_type
        else {
            panic!("expected AVG measurement");
        };
        assert_eq!(signal, "V(DEFAULT_VAL)");
        assert_eq!((*from, *to), (Some(0.25), Some(0.75)));
    }

    #[test]
    fn xyce_measure_failvalue_is_typed_order_independent_and_last_wins() {
        let netlist = Netlist::parse_with_options(
            "typed Xyce FAILVALUE qualifier\n\
             .param LIMIT=3\n\
             V1 FAILVALUE 0 1\n\
             .dc V1 0 1 1\n\
             .measure dc sample AVG V(FAILVALUE) FROM=0 FaIlVaLuE +LIMIT DEFAULT_VAL=2 TO=1 FAILVALUE=-{LIMIT/2}\n\
             .measure dc zero AVG V(FAILVALUE) FAILVALUE = 0\n\
             .measure dc positive AVG V(FAILVALUE) FAILVALUE +LIMIT\n\
             .end\n",
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("Xyce FAILVALUE forms parse");

        assert_eq!(netlist.measurements[0].fail_value, Some(-1.5));
        assert_eq!(netlist.measurements[0].default_value, Some(2.0));
        let crate::netlist::measure::MeasureType::Avg {
            signal, from, to, ..
        } = &netlist.measurements[0].measure_type
        else {
            panic!("expected AVG measurement");
        };
        assert_eq!(signal, "V(FAILVALUE)");
        assert_eq!((*from, *to), (Some(0.0), Some(1.0)));
        assert_eq!(netlist.measurements[1].fail_value, Some(0.0));
        assert_eq!(netlist.measurements[2].fail_value, Some(3.0));
    }

    #[test]
    fn measure_failvalue_rejects_malformed_values_and_non_xyce_dialects() {
        let xyce_options = crate::netlist::NetlistParseOptions {
            expression_dialect: crate::config::ExpressionDialect::Xyce,
            ..Default::default()
        };
        for qualifier in [
            "FAILVALUE",
            "FAILVALUE=",
            "FAILVALUE=UNKNOWN",
            "FAILVALUE FROM=0",
        ] {
            let deck = format!(
                "malformed FAILVALUE\nV1 out 0 1\n.tran 1n 2n\n.measure tran sample AVG V(out) {qualifier}\n.end\n"
            );
            Netlist::parse_with_options(&deck, xyce_options)
                .expect_err("a missing or malformed FAILVALUE must fail parsing");
        }

        let ngspice_deck = "FAILVALUE is Xyce-specific\n\
                            V1 out 0 1\n\
                            .tran 1n 2n\n\
                            .measure tran sample AVG V(out) FAILVALUE=1\n\
                            .end\n";
        let error = Netlist::parse(ngspice_deck)
            .expect_err("ngspice compatibility must reject Xyce FAILVALUE");
        assert!(error.to_string().contains("Xyce compatibility mode"));
    }

    #[test]
    fn measure_failvalue_is_retained_for_every_continuous_mode() {
        let options = crate::netlist::NetlistParseOptions {
            expression_dialect: crate::config::ExpressionDialect::Xyce,
            ..Default::default()
        };
        for analysis in ["TRAN_CONT", "AC_CONT", "DC_CONT", "NOISE_CONT"] {
            let deck = format!(
                "continuous FAILVALUE requires per-record semantics\n\
                 V1 out 0 1\n\
                 .measure {analysis} sample FIND V(out) AT=1 FAILVALUE=2\n\
                 .end\n"
            );
            let netlist = Netlist::parse_with_options(&deck, options)
                .expect("continuous FAILVALUE has defined per-record semantics");
            assert_eq!(netlist.measurements.len(), 1, "{analysis}");
            assert_eq!(netlist.measurements[0].analysis, analysis);
            assert_eq!(netlist.measurements[0].fail_value, Some(2.0));
        }
    }

    #[test]
    fn measure_option_parsers_reject_unrepresented_trailing_operands() {
        let aggregate_types = ["AVG", "PP", "RMS", "INTEG", "MAX", "MIN"];
        let point_types = ["FIND", "DERIV"];
        let trailing_operands = [
            "2",
            "foo",
            "{1+2}",
            "PAR('p')",
            "V(extra)",
            "I(V1)",
            "MYSTERY=1",
        ];

        for measure_type in aggregate_types {
            for trailing in trailing_operands {
                let statement = format!(".measure tran sample {measure_type} V(out) {trailing}");
                let deck = format!(
                    "strict aggregate measure\nV1 out 0 1\n.tran 1n 2n\n{statement}\n.end\n"
                );
                let error = Netlist::parse(&deck)
                    .expect_err("an unrepresented aggregate operand must fail parsing");
                assert!(
                    error.to_string().contains("Unexpected"),
                    "{statement} produced an unrelated error: {error}"
                );
            }
        }

        for measure_type in point_types {
            for trailing in trailing_operands {
                let statement =
                    format!(".measure tran sample {measure_type} V(out) AT=1n {trailing}");
                let deck =
                    format!("strict point measure\nV1 out 0 1\n.tran 1n 2n\n{statement}\n.end\n");
                let error = Netlist::parse(&deck)
                    .expect_err("an unrepresented point operand must fail parsing");
                assert!(
                    error.to_string().contains("Unexpected"),
                    "{statement} produced an unrelated error: {error}"
                );
            }
        }

        for statement in [
            ".measure tran sample AVG V(out) AVG V(out)",
            ".measure tran sample MAX V(out) TARG V(extra)",
            ".measure tran sample FIND V(out) AT=1n FIND V(extra)",
        ] {
            let deck =
                format!("strict repeated type\nV1 out 0 1\n.tran 1n 2n\n{statement}\n.end\n");
            Netlist::parse(&deck).expect_err("a repeated measure type must fail parsing");
        }
    }

    #[test]
    fn measure_options_accept_only_typed_minval_owners() {
        for statement in [
            ".measure tran sample AVG V(out) MINVAL=1e-9",
            ".measure tran sample MAX V(out) MINVAL=1e-9",
            ".measure tran sample ERROR V(out) FILE=data.csv INDEPVARCOL=0 DEPVARCOL=1 MINVAL=1e-9",
            ".measure tran sample EQN {V(out)} MINVAL=1e-9",
        ] {
            let deck =
                format!("strict minval ownership\nV1 out 0 1\n.tran 1n 2n\n{statement}\n.end\n");
            Netlist::parse(&deck)
                .expect_err("MINVAL must not be consumed by a type that cannot represent it");
        }

        for statement in [
            ".measure tran sample FIND V(out) AT=1n MINVAL=1e-9",
            ".measure tran sample DERIV V(out) AT=1n MINVAL=1e-9",
            ".measure tran sample WHEN V(out)=0.5 MINVAL=1e-9",
            ".measure tran sample TRIG V(out)=0.5 MINVAL=1e-9 TARG V(out)=0.75",
            ".measure tran sample ERR V(out) V(out) MINVAL=1e-9",
        ] {
            let deck =
                format!("typed minval ownership\nV1 out 0 1\n.tran 1n 2n\n{statement}\n.end\n");
            Netlist::parse(&deck).unwrap_or_else(|error| {
                panic!("typed MINVAL owner failed to parse: {statement}: {error}")
            });
        }
    }

    /// ngspice 46 reads an unbraced `.func` body and an unbraced one-word
    /// `.meas` value: `.func f(x) x*2` gives f(3) = 6, and `AT=t0/2` with
    /// t0 = 2m measures at 1m. Both were syntax errors here, and the `=` form
    /// of `.func` dropped every token kind it did not list (`x^2` read as
    /// `x2`).
    #[test]
    fn unbraced_func_bodies_and_measure_values_read_as_ngspice_does() {
        for dialect in [
            crate::config::ExpressionDialect::Ngspice,
            crate::config::ExpressionDialect::Xyce,
        ] {
            let netlist = Netlist::parse_with_options(
                "unbraced\n.func f(x) x*2\n.func g(x) = x^2+1\n.func h(x,y) {x-y}\n\
                 .param t0=2m a={f(3)} b={g(3)} c={h(5,1)}\n\
                 V1 out 0 PWL(0 0 2m 2)\n.tran 10u 2m\n\
                 .meas tran find_at FIND V(out) AT=t0/2\n\
                 .meas tran avg_window AVG V(out) FROM=t0/4 TO=t0*0.5\n\
                 .meas tran trig_at TRIG AT=t0/4 TARG V(out) VAL=t0*500 RISE=1\n.end\n",
                crate::netlist::NetlistParseOptions {
                    expression_dialect: dialect,
                    ..Default::default()
                },
            )
            .unwrap_or_else(|error| panic!("{dialect:?}: {error}"));
            for (name, expected) in [("A", 6.0), ("B", 10.0), ("C", 4.0)] {
                let value = netlist.params.get(name).expect("parameter is defined");
                assert!(
                    (value - expected).abs() < 1e-12,
                    "{dialect:?} {name}={value}"
                );
            }
            let measure = |name: &str| {
                &netlist
                    .measurements
                    .iter()
                    .find(|measure| measure.name.eq_ignore_ascii_case(name))
                    .expect("measurement parses")
                    .measure_type
            };
            let close = |value: Option<f64>, expected: f64| {
                value.is_some_and(|value| (value - expected).abs() < 1e-15)
            };
            assert!(
                matches!(measure("find_at"), crate::netlist::measure::MeasureType::Find { at, .. } if close(*at, 1e-3)),
                "{dialect:?}: {:?}",
                measure("find_at")
            );
            assert!(
                format!("{:?}", measure("avg_window")).contains("0.0005")
                    && format!("{:?}", measure("avg_window")).contains("0.001"),
                "{dialect:?}: {:?}",
                measure("avg_window")
            );
            assert!(
                format!("{:?}", measure("trig_at")).contains("At(0.0005)"),
                "{dialect:?}: {:?}",
                measure("trig_at")
            );
        }
        // A space still ends the value, so a following keyword is never read
        // into it, and a word that is not an expression stays an error.
        assert!(
            Netlist::parse(
                ".meas word\nV1 out 0 1\n.tran 1n 2n\n.meas tran x FIND V(out) AT=1n /2\n.end\n"
            )
            .is_err()
        );
    }

    #[test]
    fn xyce_measure_keyword_boundaries_match_release_710() {
        let xyce_options = crate::netlist::NetlistParseOptions {
            expression_dialect: crate::config::ExpressionDialect::Xyce,
            ..Default::default()
        };
        let tol_deck = "Xyce does not define a TOL measure qualifier\n\
                        V1 out 0 1\n\
                        .tran 1n 2n\n\
                        .measure tran sample AVG V(out) GOAL=1 TOL=0.1\n\
                        .end\n";
        Netlist::parse_with_options(tol_deck, xyce_options)
            .expect_err("Xyce must parse TOL as an extra operand, not a represented qualifier");
        let generic = Netlist::parse(tol_deck).expect("the generic SPICE dialect retains TOL");
        assert_eq!(generic.measurements[0].tolerance, Some(0.1));

        Netlist::parse_with_options(
            "unbraced Xyce arithmetic is not an expression\n\
             V1 out 0 1\n\
             .param VAL=2\n\
             .tran 1n 2n\n\
             .measure tran sample EQN VAL+1\n\
             .end\n",
            xyce_options,
        )
        .expect_err("Xyce requires arithmetic EQN/PARAM expressions to be braced or quoted");
        let equation = Netlist::parse_with_options(
            "braced Xyce arithmetic\n\
             V1 out 0 1\n\
             .param VAL=2\n\
             .tran 1n 2n\n\
             .measure tran sample EQN {VAL+1}\n\
             .end\n",
            xyce_options,
        )
        .expect("braced arithmetic remains a Xyce ExpressionOp");
        let crate::netlist::measure::MeasureType::Equation { expression, .. } =
            &equation.measurements[0].measure_type
        else {
            panic!("expected an equation measurement");
        };
        assert_eq!(expression.text, "VAL+1");
        assert_eq!(
            expression.kind,
            crate::netlist::measure::MeasureExpressionKind::Expression
        );

        Netlist::parse_with_options(
            "type marker cannot join equation text\n\
             V1 out 0 1\n\
             .tran 1n 2n\n\
             .measure tran sample EQN V(out) AVG\n\
             .end\n",
            xyce_options,
        )
        .expect_err("a second Xyce measure type must remain outside equation text");
    }

    #[test]
    fn explicit_eqn_accepts_a_bare_measure_reference_in_generic_dialect() {
        let netlist = Netlist::parse(
            "explicit EQN selects Xyce equation grammar\n\
             V1 out 0 1\n\
             .tran 1n 2n\n\
             .measure tran DMAX MAX V(out)\n\
             .measure tran EQN1 EQN DMAX\n\
             .measure tran EQN2 EQN {DMAX}\n\
             .end\n",
        )
        .expect("an explicit EQN keyword accepts its unquoted expression");
        let crate::netlist::measure::MeasureType::Equation { expression, .. } =
            &netlist.measurements[1].measure_type
        else {
            panic!("explicit EQN must produce an equation measurement");
        };
        assert_eq!(expression.text, "DMAX");
        assert_eq!(
            expression.kind,
            crate::netlist::measure::MeasureExpressionKind::RawReference
        );
        let crate::netlist::measure::MeasureType::Equation { expression, .. } =
            &netlist.measurements[2].measure_type
        else {
            panic!("braced EQN must produce an equation measurement");
        };
        assert_eq!(expression.text, "DMAX");
        assert_eq!(
            expression.kind,
            crate::netlist::measure::MeasureExpressionKind::Expression
        );
    }

    #[test]
    fn xyce_eqn_and_param_preserve_raw_output_operator_provenance() {
        let netlist = Netlist::parse_with_options(
            "raw Xyce measure output operators\n\
             V1 1 0 AC 1\n\
             .ac lin 2 1 2\n\
             .measure ac raw_v EQN V(1,0) FROM=1 TO=2\n\
             .measure ac raw_i EQN IP(V1)\n\
             .measure ac raw_dno EQN DNO(M1,thermal)\n\
             .measure ac raw_dni EQN DNI(M1)\n\
             .measure ac raw_device EQN N(X1:M1:id)\n\
             .measure ac raw_power EQN P(R1)\n\
             .measure ac raw_watt EQN W(R1)\n\
             .measure ac raw_network EQN SDB(1,2)\n\
             .measure ac raw_param PARAM VDB(0) FROM=1\n\
             .measure ac compound EQN VDB(0)+1\n\
             .measure ac braced EQN {VDB(0)}\n\
             .measure ac quoted PARAM='VDB(0)'\n\
             .end\n",
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("Xyce raw output-operator deck parses");

        for (index, expected) in [
            (0, "V(1,0)"),
            (1, "IP(V1)"),
            (2, "DNO(M1,thermal)"),
            (3, "DNI(M1)"),
            (4, "N(X1:M1:id)"),
            (5, "P(R1)"),
            (6, "W(R1)"),
            (8, "VDB(0)"),
            // Xyce extracts the first output operator and ignores the
            // adjacent unbraced arithmetic field.
            (9, "VDB(0)"),
        ] {
            let statement = &netlist.measurements[index];
            let crate::netlist::measure::MeasureType::Equation { expression, .. } =
                &statement.measure_type
            else {
                panic!("Xyce PARAM/EQN output operator must be an equation measure");
            };
            assert_eq!(expression.text, expected);
            assert_eq!(
                expression.kind,
                crate::netlist::measure::MeasureExpressionKind::RawOutputOperator
            );
        }
        let crate::netlist::measure::MeasureType::Equation { expression, .. } =
            &netlist.measurements[7].measure_type
        else {
            panic!("RF output operator must be an equation measure");
        };
        assert_eq!(expression.text, "SDB(1,2)");
        assert_eq!(
            expression.kind,
            crate::netlist::measure::MeasureExpressionKind::Expression
        );
        let crate::netlist::measure::MeasureType::Equation { from, to, .. } =
            &netlist.measurements[0].measure_type
        else {
            unreachable!()
        };
        assert_eq!((*from, *to), (Some(1.0), Some(2.0)));
        let crate::netlist::measure::MeasureType::Equation { from, .. } =
            &netlist.measurements[8].measure_type
        else {
            unreachable!()
        };
        assert_eq!(*from, Some(1.0));

        for statement in &netlist.measurements[10..] {
            let crate::netlist::measure::MeasureType::Equation { expression, .. } =
                &statement.measure_type
            else {
                panic!("braced/quoted Xyce PARAM/EQN must be equation measures");
            };
            assert_eq!(expression.text, "VDB(0)");
            assert_eq!(
                expression.kind,
                crate::netlist::measure::MeasureExpressionKind::Expression
            );
        }
    }

    #[test]
    fn xyce_raw_output_operator_preserves_legacy_current_and_numeric_lexemes() {
        let netlist = Netlist::parse_with_options(
            "raw Xyce operator spelling\n\
             V1 1 0 1\n\
             .tran 1 2\n\
             .measure tran legacy EQN I(YPDE BRANCH)\n\
             .measure tran numeric EQN V(2e3)\n\
             .end\n",
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("legacy and numeric raw operator spellings parse");
        for (statement, expected) in netlist
            .measurements
            .iter()
            .zip(["I(YPDE BRANCH)", "V(2e3)"])
        {
            let crate::netlist::measure::MeasureType::Equation { expression, .. } =
                &statement.measure_type
            else {
                panic!("raw output operator must be an equation measure");
            };
            assert_eq!(expression.text, expected);
            assert_eq!(
                expression.kind,
                crate::netlist::measure::MeasureExpressionKind::RawOutputOperator
            );
        }
    }

    #[test]
    fn file_measure_path_is_one_authored_whitespace_field() {
        let valid = Netlist::parse(
            "file measure path\n\
             V1 out 0 1\n\
             .tran 1n 2n\n\
             .measure tran sample ERROR V(out) FILE=fixtures/data.csv COMP_FUNCTION=L2NORM INDEPVARCOL=0 DEPVARCOL=1\n\
             .end\n",
        )
        .expect("a punctuation-bearing unquoted FILE field parses");
        let crate::netlist::measure::MeasureType::FileError { file, .. } =
            &valid.measurements[0].measure_type
        else {
            panic!("expected a file-backed ERROR measurement");
        };
        assert_eq!(file.path(), "fixtures/data.csv");

        let error = Netlist::parse(
            "file measure trailing operand\n\
             V1 out 0 1\n\
             .tran 1n 2n\n\
             .measure tran sample ERROR V(out) FILE=fixtures/data.csv stray INDEPVARCOL=0 DEPVARCOL=1\n\
             .end\n",
        )
        .expect_err("a second whitespace field after FILE must not join the path");
        assert!(error.to_string().contains("Unexpected option 'STRAY'"));
    }

    #[test]
    fn continuous_measure_modes_types_and_print_policies_are_typed() {
        use crate::netlist::measure::MeasurePrintPolicy;

        let netlist = Netlist::parse(
            "continuous measures\n\
             V1 out 0 1\n\
             .tran 1n 10n\n\
             .measure tran_cont first WHEN V(out)=0.5 PRINT=stdout\n\
             .measure ac_cont second FIND VM(out) AT=1k PRINT=NONE\n\
             .measure dc_cont third DERIV V(out) AT=0.5 PRINT=ALL\n\
             .end\n",
        )
        .expect("canonical continuous modes and point-event types parse");

        assert_eq!(netlist.measurements.len(), 3);
        assert_eq!(netlist.measurements[0].analysis, "TRAN_CONT");
        assert_eq!(
            netlist.measurements[0].print_policy,
            MeasurePrintPolicy::Stdout
        );
        assert_eq!(
            netlist.measurements[1].print_policy,
            MeasurePrintPolicy::None
        );
        assert_eq!(
            netlist.measurements[2].print_policy,
            MeasurePrintPolicy::All
        );

        for line in [
            ".measure hb_cont bad WHEN V(out)=0.5",
            ".measure tran_cont bad AVG V(out)",
        ] {
            let deck = format!("invalid continuous measure\nV1 out 0 1\n{line}\n.end\n");
            assert!(Netlist::parse(&deck).is_err(), "{line} must be rejected");
        }
    }

    #[test]
    fn continuous_file_option_defaults_enabled_and_merges_without_clobbering() {
        let defaults = crate::netlist::SimulationOptions::default();
        assert!(defaults.measure_use_cont_files());

        let netlist = Netlist::parse(&deck_with_options(
            ".options measure use_cont_files=0\n.options measure measfail=0",
        ))
        .expect("separate MEASURE option cards merge");
        assert_eq!(netlist.options.measure_use_cont_files, Some(false));
        assert!(!netlist.options.measure_use_cont_files());
        assert_eq!(netlist.options.measure_fail_output, Some(false));

        let mut merged = crate::netlist::SimulationOptions::default();
        let override_options = crate::netlist::SimulationOptions {
            measure_use_cont_files: Some(false),
            ..crate::netlist::SimulationOptions::default()
        };
        merged.merge(&override_options);
        assert!(!merged.measure_use_cont_files());
    }

    #[test]
    fn device_try_to_compact_parses_boolean_forms_and_merges() {
        let enabled = Netlist::parse(&deck_with_options(".options device trytocompact=1"))
            .expect("scoped Xyce TRYTOCOMPACT parses");
        assert_eq!(enabled.options.device_try_to_compact, Some(true));

        let disabled = Netlist::parse(&deck_with_options(".options device try_to_compact=false"))
            .expect("underscored scoped TRYTOCOMPACT parses");
        assert_eq!(disabled.options.device_try_to_compact, Some(false));

        let mut merged = crate::netlist::SimulationOptions {
            device_try_to_compact: Some(true),
            ..crate::netlist::SimulationOptions::default()
        };
        merged.merge(&disabled.options);
        assert_eq!(merged.device_try_to_compact, Some(false));
    }

    #[test]
    fn xyce_voltage_limiting_and_serial_partition_options_are_typed() {
        let netlist = Netlist::parse(&deck_with_options(
            ".options device voltlim=0\n.options linsol tr_partition=0",
        ))
        .expect("Xyce DEVICE/LINSOL options parse");
        assert_eq!(netlist.options.device_voltage_limiting, Some(false));
        assert_eq!(netlist.options.linsol_tr_partition, Some(false));
        assert!(
            netlist
                .diagnostics
                .iter()
                .all(|diagnostic| diagnostic.code != "unknown-option"),
            "typed Xyce options must not be diagnosed as unknown: {:?}",
            netlist.diagnostics
        );

        let repeated = Netlist::parse(&deck_with_options(
            ".options device voltlim=0 voltlim=2\n.options linsol tr_partition=1 tr_partition=0",
        ))
        .expect("repeated Xyce boolean options parse");
        assert_eq!(repeated.options.device_voltage_limiting, Some(true));
        assert_eq!(repeated.options.linsol_tr_partition, Some(false));

        let bare = Netlist::parse(&deck_with_options(
            ".options device voltlim\n.options linsol tr_partition",
        ))
        .expect("bare Xyce boolean options mean enabled");
        assert_eq!(bare.options.device_voltage_limiting, Some(true));
        assert_eq!(bare.options.linsol_tr_partition, Some(true));

        let mut merged = crate::netlist::SimulationOptions {
            device_voltage_limiting: Some(true),
            linsol_tr_partition: Some(true),
            ..Default::default()
        };
        merged.merge(&netlist.options);
        assert_eq!(merged.device_voltage_limiting, Some(false));
        assert_eq!(merged.linsol_tr_partition, Some(false));

        let unrelated_linsol = Netlist::parse(&deck_with_options(".options linsol type=klu"))
            .expect("unimplemented LINSOL option remains a warning");
        assert_eq!(unrelated_linsol.options.linsol_tr_partition, None);
        assert_eq!(unrelated_linsol.diagnostics.len(), 2);
        assert_eq!(
            unrelated_linsol.diagnostics[0].message,
            "unknown .options key 'LINSOL' ignored"
        );
        assert_eq!(
            unrelated_linsol.diagnostics[1].message,
            "unknown .options key 'TYPE' ignored"
        );
    }

    #[test]
    fn xyce_device_debug_level_is_typed_and_merges_last_value() {
        let netlist = Netlist::parse(&deck_with_options(
            ".options device debuglevel=3 debug_level=0",
        ))
        .expect("Xyce DEVICE.DEBUGLEVEL parses");
        assert_eq!(netlist.options.device_debug_level, Some(0));
        assert!(
            netlist
                .diagnostics
                .iter()
                .all(|diagnostic| diagnostic.code != "unknown-option"),
            "typed DEVICE.DEBUGLEVEL must not be diagnosed as unknown: {:?}",
            netlist.diagnostics
        );

        let mut merged = crate::netlist::SimulationOptions {
            device_debug_level: Some(7),
            ..Default::default()
        };
        merged.merge(&netlist.options);
        assert_eq!(merged.device_debug_level, Some(0));

        let negative = Netlist::parse(&deck_with_options(".options device debuglevel=-100"))
            .expect("negative Xyce debug levels disable verbosity");
        assert_eq!(negative.options.device_debug_level, Some(-100));

        let fractional = deck_with_options(".options device debuglevel=1.5");
        assert!(
            Netlist::parse(&fractional).is_err(),
            "fractional DEVICE.DEBUGLEVEL must be rejected"
        );
    }

    #[test]
    fn xyce_device_separate_load_is_typed_and_merges_last_value() {
        let netlist = Netlist::parse(&deck_with_options(
            ".options device separateload=0 separate_load=1",
        ))
        .expect("Xyce DEVICE.SEPARATELOAD parses");
        assert_eq!(netlist.options.device_separate_load, Some(true));
        assert!(
            netlist
                .diagnostics
                .iter()
                .all(|diagnostic| diagnostic.code != "unknown-option"),
            "typed DEVICE.SEPARATELOAD must not be diagnosed as unknown: {:?}",
            netlist.diagnostics
        );

        let disabled = Netlist::parse(&deck_with_options(
            ".options device separateload=true\n.options device separateload=0",
        ))
        .expect("repeated Xyce DEVICE.SEPARATELOAD cards merge");
        assert_eq!(disabled.options.device_separate_load, Some(false));

        let bare = Netlist::parse(&deck_with_options(".options device separateload"))
            .expect("bare Xyce DEVICE.SEPARATELOAD means enabled");
        assert_eq!(bare.options.device_separate_load, Some(true));

        let mut merged = crate::netlist::SimulationOptions {
            device_separate_load: Some(true),
            ..Default::default()
        };
        merged.merge(&disabled.options);
        assert_eq!(merged.device_separate_load, Some(false));
    }

    #[test]
    fn xyce_timeint_debug_level_is_typed_and_merges_last_value() {
        let netlist = Netlist::parse(&deck_with_options(
            ".options timeint debuglevel=3 debug_level=-100",
        ))
        .expect("Xyce TIMEINT.DEBUGLEVEL parses");
        assert_eq!(netlist.options.timeint_debug_level, Some(-100));
        assert!(
            netlist
                .diagnostics
                .iter()
                .all(|diagnostic| diagnostic.code != "unknown-option"),
            "typed TIMEINT.DEBUGLEVEL must not be diagnosed as unknown: {:?}",
            netlist.diagnostics
        );

        let mut merged = crate::netlist::SimulationOptions {
            timeint_debug_level: Some(7),
            ..Default::default()
        };
        merged.merge(&netlist.options);
        assert_eq!(merged.timeint_debug_level, Some(-100));

        let fractional = Netlist::parse(&deck_with_options(".options timeint debuglevel=1.9"))
            .expect("Xyce truncates numeric DEBUGLEVEL toward zero");
        assert_eq!(fractional.options.timeint_debug_level, Some(1));

        for invalid in ["2147483648", "-2147483649"] {
            assert!(
                Netlist::parse(&deck_with_options(&format!(
                    ".options timeint debuglevel={invalid}"
                )))
                .is_err(),
                "out-of-range DEBUGLEVEL={invalid} must fail closed"
            );
        }
    }

    #[test]
    fn device_minimum_defaults_parse_and_merge() {
        let netlist = Netlist::parse(&deck_with_options(
            ".options device minres=1 mincap=1nf pnjmaxi=1u",
        ))
        .expect("scoped Xyce MINRES/MINCAP parse");
        assert_eq!(netlist.options.device_min_resistance, Some(1.0));
        assert_eq!(netlist.options.device_min_capacitance, Some(1.0e-9));
        assert_eq!(netlist.options.device_pnjmaxi, Some(1.0e-6));

        let mut merged = crate::netlist::SimulationOptions {
            device_min_resistance: Some(2.0),
            device_pnjmaxi: Some(1.0),
            ..crate::netlist::SimulationOptions::default()
        };
        merged.merge(&netlist.options);
        assert_eq!(merged.device_min_resistance, Some(1.0));
        assert_eq!(merged.device_min_capacitance, Some(1.0e-9));
        assert_eq!(merged.device_pnjmaxi, Some(1.0e-6));
    }

    #[test]
    fn duplicate_measure_names_are_global_case_insensitive_last_wins() {
        let netlist = Netlist::parse(
            "duplicate measures\n\
             V1 out 0 1\n\
             .measure tran Shared WHEN V(out)=0.1\n\
             .measure ac shared FIND VM(out) AT=1k PRINT=NONE\n\
             .end\n",
        )
        .expect("Xyce replacement semantics preserve the final definition");

        assert_eq!(netlist.measurements.len(), 1);
        assert_eq!(netlist.measurements[0].name, "SHARED");
        assert_eq!(netlist.measurements[0].analysis, "AC");
        assert!(netlist.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "measure-redefined"
                && diagnostic
                    .message
                    .contains("ignoring the previous definition")
        }));
    }

    #[test]
    fn rshunt_option_parses_as_a_resistance() {
        let netlist =
            Netlist::parse(&deck_with_options(".options rshunt=1e12")).expect("RSHUNT parses");
        assert_eq!(netlist.options.rshunt, Some(1.0e12));
        assert_eq!(
            Netlist::parse(&deck_with_options(".options rshunt=1meg"))
                .expect("suffixed RSHUNT parses")
                .options
                .rshunt,
            Some(1.0e6)
        );
    }

    #[test]
    fn timeint_package_does_not_capture_unrelated_solver_options() {
        let netlist = Netlist::parse(&deck_with_options(
            ".options timeint gmin=0 vntol=1e-9 itl1=9 trtol=2",
        ))
        .expect("unknown TIMEINT keys remain non-fatal diagnostics");

        assert_eq!(netlist.options.gmin, None);
        assert_eq!(netlist.options.vntol, None);
        assert_eq!(netlist.options.itl1, None);
        assert_eq!(netlist.options.trtol, None);
        for key in [
            "TIMEINT.GMIN",
            "TIMEINT.VNTOL",
            "TIMEINT.ITL1",
            "TIMEINT.TRTOL",
        ] {
            assert!(netlist.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "unknown-option" && diagnostic.message.contains(key)
            }));
        }
    }

    #[test]
    fn unscoped_newlte_is_diagnosed_and_does_not_change_solver_policy() {
        let netlist = Netlist::parse(&deck_with_options(".options newlte=2"))
            .expect("unknown unscoped option remains a non-fatal diagnostic");

        assert_eq!(netlist.options.transient_lte_reference, None);
        assert!(netlist.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "unknown-option" && diagnostic.message.contains("NEWLTE")
        }));
    }

    #[test]
    fn options_parse_xyce_hyphenated_solver_package() {
        let netlist = Netlist::parse(&deck_with_options(
            ".options reltol=9e-4 abstol=8e-12\n\
             .options nonlin-tran reltol=1e-3 abstol=2e-6 deltaxtol=0.25 rhstol=3e-3 maxstep=37 enforceDeviceConv=1\n\
             .options nonlin-transient reltol=4e-3\n\
             .options timeint method=gear",
        ))
        .expect("Xyce hyphenated solver option package parses");

        assert_eq!(netlist.options.reltol, Some(9.0e-4));
        assert_eq!(netlist.options.abstol, Some(8.0e-12));
        assert_eq!(netlist.options.nonlin_transient_reltol, Some(4.0e-3));
        assert_eq!(netlist.options.nonlin_transient_abstol, Some(2.0e-6));
        assert_eq!(netlist.options.nonlin_transient_deltaxtol, Some(0.25));
        assert_eq!(netlist.options.nonlin_transient_rhstol, Some(3.0e-3));
        assert_eq!(netlist.options.nonlin_transient_maxstep, Some(37));
        assert_eq!(
            netlist.options.nonlin_transient_enforce_device_convergence,
            Some(true)
        );
        assert_eq!(netlist.options.method.as_deref(), Some("GEAR"));
        assert!(netlist.diagnostics.is_empty());
    }

    #[test]
    fn transient_nonlinear_options_merge_without_clobbering_other_packages() {
        let mut merged = crate::netlist::SimulationOptions {
            nonlin_transient_reltol: Some(1.0e-3),
            timeint_reltol: Some(2.0e-3),
            ..Default::default()
        };
        merged.merge(&crate::netlist::SimulationOptions {
            nonlin_transient_rhstol: Some(3.0e-3),
            nonlin_transient_maxstep: Some(23),
            nonlin_transient_enforce_device_convergence: Some(false),
            ..Default::default()
        });

        assert_eq!(merged.nonlin_transient_reltol, Some(1.0e-3));
        assert_eq!(merged.nonlin_transient_rhstol, Some(3.0e-3));
        assert_eq!(merged.nonlin_transient_maxstep, Some(23));
        assert_eq!(
            merged.nonlin_transient_enforce_device_convergence,
            Some(false)
        );
        assert_eq!(merged.timeint_reltol, Some(2.0e-3));
    }

    #[test]
    fn tran_noop_alias_sets_uic() {
        let netlist = Netlist::parse(
            "tran noop alias\n\
             V1 1 0 1\n\
             R1 1 0 1k\n\
             .tran 1n 10n noop\n\
             .end\n",
        )
        .expect(".TRAN NOOP should parse as an operating-point bypass");

        let Some(crate::netlist::AnalysisCommand::Tran { uic, .. }) = netlist.analyses.first()
        else {
            panic!("expected transient analysis");
        };

        assert!(*uic);
    }

    #[test]
    fn xyce_tr_alias_normalizes_analysis_and_output_domains() {
        let netlist = Netlist::parse(
            "Xyce TR alias\n\
             V1 1 0 1\n\
             R1 1 0 1k\n\
             .tr 1n 10n\n\
             .print tr v(1)\n\
             .plot tr v(1)\n\
             .measure tr vmax max v(1)\n\
             .end\n",
        )
        .expect("Xyce .TR and TR-qualified outputs should parse as transient");

        assert!(matches!(
            netlist.analyses.as_slice(),
            [crate::netlist::AnalysisCommand::Tran { step, stop, .. }]
                if step.to_bits() == 1.0e-9f64.to_bits()
                    && stop.to_bits() == 10.0e-9f64.to_bits()
        ));
        assert_eq!(netlist.output_requests.len(), 3);
        assert!(
            netlist.output_requests.iter().all(|request| {
                request.analysis == Some(crate::netlist::OutputAnalysisKind::Tran)
            })
        );
        assert!(matches!(
            netlist.measurements.as_slice(),
            [measurement] if measurement.analysis == "TRAN"
        ));
    }

    #[test]
    fn ltspice_measures_read_unbraced_expressions() {
        // LTspice 26.1.1 measures each of these, spaces and all.
        use crate::netlist::measure::MeasureType;
        let netlist = Netlist::parse(
            "unbraced measures\n\
             .param p=5\n\
             v1 1 0 1\n\
             r1 1 0 {p}\n\
             .dc v1 1 2 1\n\
             .meas dc m1 max p * 2\n\
             .meas dc m2 param p*2+1\n\
             .meas dc m3 max v(1)*2 from=1 to=1.5\n\
             .meas dc m4 find -i(v1)*2 at=2\n\
             .meas dc m5 param m3-1\n\
             .meas dc m6 max v(1)\n\
             .meas dc m7 avg n-1\n\
             .end\n",
        )
        .expect("LTspice measure expressions parse");
        let operands = netlist
            .measurements
            .iter()
            .map(|statement| match &statement.measure_type {
                MeasureType::Max { signal, .. } | MeasureType::Avg { signal, .. } => signal.clone(),
                MeasureType::Find { signal, at, .. } => format!("{signal} at {at:?}"),
                MeasureType::Param { expression } => format!("param {}", expression.text),
                other => format!("{other:?}"),
            })
            .collect::<Vec<_>>();
        assert_eq!(
            operands,
            [
                "{p * 2}",
                "param p*2+1",
                "{v(1)*2}",
                "{-i(v1)*2} at Some(2.0)",
                "param m3-1",
                "V(1)",
                // A `-` joining name fragments stays part of the name.
                "N-1",
            ]
        );
        assert!(matches!(
            &netlist.measurements[2].measure_type,
            MeasureType::Max { from: Some(from), to: Some(to), .. } if *from == 1.0 && *to == 1.5
        ));
        // Xyce still requires the braces LTspice leaves out.
        let xyce = Netlist::parse_with_options(
            "unbraced measures\nv1 1 0 1\nr1 1 0 1\n.dc v1 1 2 1\n.meas dc m1 param 2*3\n.end\n",
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        );
        assert!(xyce.is_err(), "{xyce:?}");
    }

    fn parse_in(deck: &str, dialect: crate::config::SpiceDialect) -> Result<Netlist, ParseError> {
        Netlist::parse_with_options(
            deck,
            crate::netlist::NetlistParseOptions {
                spice_dialect: Some(dialect),
                ..Default::default()
            },
        )
    }

    fn transient_fields(netlist: &Netlist) -> (f64, f64, Option<f64>, Option<f64>, bool) {
        netlist
            .analyses
            .iter()
            .find_map(|analysis| match analysis {
                AnalysisCommand::Tran {
                    step,
                    stop,
                    start,
                    max_step,
                    uic,
                } => Some((*step, *stop, *start, *max_step, *uic)),
                _ => None,
            })
            .expect("a transient analysis")
    }

    /// ngspice and Xyce refuse a card without a stop time; LTspice reads the
    /// lone field as the stop time with no print step.
    #[test]
    fn a_lone_tran_field_is_the_ltspice_stop_time() {
        for dialect in [
            crate::config::SpiceDialect::Xyce,
            crate::config::SpiceDialect::Ngspice,
        ] {
            let error = parse_in("lone field\nR1 1 0 1\n.tran 1u\n.end\n", dialect)
                .expect_err("a transient analysis without a stop time must fail");
            assert!(
                matches!(
                    error,
                    ParseError::Syntax { line: 3, ref message }
                        if message == ".TRAN line has an unexpected number of fields\nUnrecognized dot line will be ignored"
                ),
                "{dialect:?}: {error:?}"
            );
        }
        // Xyce's BUG_636_SON deck is read with Xyce expressions and no
        // dialect; it must keep Xyce's refusal.
        let xyce_expressions = Netlist::parse_with_options(
            "lone field\nR1 1 0 1\n.tran 1u\n.end\n",
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        );
        assert!(
            matches!(xyce_expressions, Err(ParseError::Syntax { line: 3, .. })),
            "{xyce_expressions:?}"
        );
        for (card, expected) in [
            (".tran 1u", (0.0, 1e-6, None, None, false)),
            (".tran 1u uic", (0.0, 1e-6, None, None, true)),
            (".tran 1u 1m", (1e-6, 1e-3, None, None, false)),
        ] {
            let deck = format!("lone field\nR1 1 0 1\n{card}\n.end\n");
            let netlist = parse_in(&deck, crate::config::SpiceDialect::BestAvailable)
                .unwrap_or_else(|error| panic!("{card}: {error}"));
            assert_eq!(transient_fields(&netlist), expected, "{card}");
        }
    }

    /// Xyce gives an option a comma list (HOMOTOPY's `.options loca`,
    /// `SENSITIVITY adjointTimePoints`), a brace expression (`SAMPLES outputs`)
    /// or a quoted string (`MPDE oscout`); an ignored option takes all of it,
    /// and the options after it still apply.
    #[test]
    fn an_ignored_xyce_option_takes_its_whole_value() {
        let netlist = parse_in(
            "ignored option values\nV1 1 0 1\nR1 1 0 1k\n\
             .options loca stepper=0\n\
             + conparam=mosfet:gainscale,mosfet:nltermscale\n\
             + initialvalue=0.0,0.0 minvalue=-1.0,-1.0\n\
             .options samples numsamples=10\n+ outputs={v(1)},{v(1)+1}\n\
             .options mpde oscout=\"I(R1)\" phase=1\n\
             .options SENSITIVITY adjoint=1\n+ adjointTimePoints=0.25,0.5,0.75\n\
             .options timeint reltol=1e-4\n.op\n.end\n",
            crate::config::SpiceDialect::Xyce,
        )
        .expect("ignored Xyce option values parse");
        assert_eq!(netlist.options.timeint_reltol, Some(1e-4));
    }

    /// Xyce's `TD` delays a range or extrema measure (N_IO_MeasureBase.C:668);
    /// the other dialects keep refusing it there.
    #[test]
    fn xyce_range_and_extrema_measures_take_a_delay() {
        use crate::netlist::measure::MeasureType;
        let deck = "delay\nV1 1 0 1\nR1 1 0 1\n.tran 0 2\n\
                    .measure tran mx max v(1) td=1.5\n\
                    .measure tran av avg v(1) from=0.2 td=1.5\n.end\n";
        let xyce = Netlist::parse_with_options(
            deck,
            crate::netlist::NetlistParseOptions {
                spice_dialect: Some(crate::config::SpiceDialect::Xyce),
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("Xyce deck parses");
        assert!(matches!(
            &xyce.measurements[0].measure_type,
            MeasureType::Max { td: Some(td), from: None, .. } if *td == 1.5
        ));
        assert!(matches!(
            &xyce.measurements[1].measure_type,
            MeasureType::Avg { td: Some(td), from: Some(from), .. } if *td == 1.5 && *from == 0.2
        ));
        let error = parse_in(deck, crate::config::SpiceDialect::Ngspice)
            .expect_err("ngspice dialect refuses TD on MAX");
        assert!(error.to_string().contains("TD"), "{error}");
    }

    /// Xyce's DUTY, ON_TIME, OFF_TIME and FREQ take a window, a delay and
    /// the ON and OFF levels, which default to 0 (N_IO_MeasureBase.C:95-98).
    #[test]
    fn xyce_on_off_measures_parse_their_levels_and_window() {
        use crate::netlist::measure::{MeasureType, OnOffStatistic};
        let netlist = Netlist::parse(
            "on off\nV1 1 0 1\nR1 1 0 1\n.tran 0 2\n\
             .measure tran d duty v(1) on=0.75 off=0.25 from=0.1 to=1.9 td=0.2\n\
             .measure tran n on_time v(1)\n\
             .measure tran f off_time v(1) minval=1e-6\n\
             .measure tran q freq v(1) on=0.5 off=0.5\n.end\n",
        )
        .expect("on/off measures parse");
        let shapes = netlist
            .measurements
            .iter()
            .map(|statement| match &statement.measure_type {
                MeasureType::OnOff {
                    statistic,
                    from,
                    to,
                    td,
                    on,
                    off,
                    minval,
                    ..
                } => (*statistic, *from, *to, *td, *on, *off, *minval),
                other => panic!("expected an on/off measure, got {other:?}"),
            })
            .collect::<Vec<_>>();
        let minval = crate::netlist::measure::XYCE_DEFAULT_MEASURE_MINVAL;
        assert_eq!(
            shapes,
            [
                (
                    OnOffStatistic::Duty,
                    Some(0.1),
                    Some(1.9),
                    Some(0.2),
                    0.75,
                    0.25,
                    minval
                ),
                (OnOffStatistic::OnTime, None, None, None, 0.0, 0.0, minval),
                (OnOffStatistic::OffTime, None, None, None, 0.0, 0.0, 1e-6),
                (
                    OnOffStatistic::Frequency,
                    None,
                    None,
                    None,
                    0.5,
                    0.5,
                    minval
                ),
            ]
        );
    }

    /// Xyce's FOUR takes its fundamental `AT`, with `NUMFREQ` 10 and
    /// `GRIDSIZE` 200 by default (N_IO_MeasureBase.C:140-141); `PRECISION`
    /// only sets the digits of Xyce's harmonic table, and a negative `AT` is
    /// refused (N_IO_MeasureFourier.C:67-70).
    #[test]
    fn xyce_fourier_measures_parse_their_fundamental_and_grid() {
        use crate::netlist::measure::MeasureType;
        let deck =
            |measure: &str| format!("four\nV1 1 0 1\nR1 1 0 1\n.tran 0 2\n{measure}\n.end\n");
        let shape = |measure: &str| {
            let netlist = Netlist::parse(&deck(measure)).expect("FOUR measure parses");
            match &netlist.measurements[0].measure_type {
                MeasureType::Fourier {
                    at,
                    num_freq,
                    grid_size,
                    from,
                    to,
                    ..
                } => (*at, *num_freq, *grid_size, *from, *to),
                other => panic!("expected a FOUR measure, got {other:?}"),
            }
        };
        assert_eq!(
            shape(".measure tran a four v(1) at=1e3"),
            (1e3, 10, 200, None, None)
        );
        assert_eq!(
            shape(
                ".measure tran b four v(1) at=2k numfreq=5 gridsize=64 from=0.5 to=1.5 precision=4"
            ),
            (2e3, 5, 64, Some(0.5), Some(1.5))
        );
        assert_eq!(
            shape(".measure tran c four v(1)"),
            (0.0, 10, 200, None, None)
        );
        let error = Netlist::parse(&deck(".measure tran d four v(1) at=-1"))
            .expect_err("a negative AT is refused");
        assert!(error.to_string().contains("AT"), "{error}");
    }

    /// Xyce's `.MEASURE FFT` statistics take `BINSIZ`, `MINFREQ`, `MAXFREQ`
    /// and `NBHARM` (N_IO_MeasureBase.C:508-524), with BINSIZ 0 by default.
    #[test]
    fn xyce_fft_measures_parse_their_bins_and_band() {
        use crate::netlist::measure::{FftMetric, MeasureType};
        let netlist = Netlist::parse(
            "fft\nV1 1 0 1\nR1 1 0 1\n.tran 0 1\n.fft v(1) np=8\n\
             .measure fft e enob v(1)\n\
             .measure fft s sfdr v(1) minfreq=2 maxfreq=3 binsiz=1\n\
             .measure fft n snr v(1) maxfreq=2\n\
             .measure fft d sndr v(1)\n\
             .measure fft t thd v(1) nbharm=3\n.end\n",
        )
        .expect("FFT measures parse");
        let shapes = netlist
            .measurements
            .iter()
            .map(|statement| match &statement.measure_type {
                MeasureType::FftMetric {
                    metric,
                    bin_size,
                    min_freq,
                    max_freq,
                    harmonics,
                    ..
                } => (*metric, *bin_size, *min_freq, *max_freq, *harmonics),
                other => panic!("expected an FFT measure, got {other:?}"),
            })
            .collect::<Vec<_>>();
        assert_eq!(
            shapes,
            [
                (FftMetric::Enob, 0, None, None, None),
                (FftMetric::Sfdr, 1, Some(2.0), Some(3.0), None),
                (FftMetric::Snr, 0, None, Some(2.0), None),
                (FftMetric::Sndr, 0, None, None, None),
                (FftMetric::Thd, 0, None, None, Some(3)),
            ]
        );
    }

    /// Xyce's MAX, MIN and PP take RISE, FALL or CROSS, with LAST, and
    /// RFC_LEVEL (N_IO_MeasureBase.C:323-372); one of the three at a time,
    /// and PP has no OUTPUT.
    #[test]
    fn xyce_extrema_parse_their_rise_fall_or_cross() {
        use crate::netlist::measure::{EdgeType, EventOccurrence, MeasureType};
        let deck = |measure: &str| format!("rfc\nV1 1 0 1\nR1 1 0 1\n.tran 0 1\n{measure}\n.end\n");
        let window = |measure: &str| {
            let netlist = Netlist::parse(&deck(measure)).expect("extrema measure parses");
            match &netlist.measurements[0].measure_type {
                MeasureType::Max {
                    crossing,
                    rfc_level,
                    ..
                }
                | MeasureType::Min {
                    crossing,
                    rfc_level,
                    ..
                }
                | MeasureType::PeakToPeak {
                    crossing,
                    rfc_level,
                    ..
                } => (*crossing, *rfc_level),
                other => panic!("expected an extrema measure, got {other:?}"),
            }
        };
        let occurrence = |edge, number| Some(EventOccurrence { edge, number });
        assert_eq!(
            window(".measure tran a max v(1) fall=2"),
            (occurrence(EdgeType::Fall, 2), None)
        );
        assert_eq!(
            window(".measure tran a min v(1) rise=last"),
            (occurrence(EdgeType::Rise, -1), None)
        );
        assert_eq!(
            window(".measure tran a pp v(1) rfc_level=0.5 cross=1"),
            (occurrence(EdgeType::Cross, 1), Some(0.5))
        );
        assert_eq!(window(".measure tran a max v(1)"), (None, None));
        for refused in [
            ".measure tran a max v(1) rise=1 fall=1",
            ".measure tran a pp v(1) output=time",
        ] {
            assert!(Netlist::parse(&deck(refused)).is_err(), "{refused}");
        }
    }

    /// Xyce reads `agauss` at its mean outside a sampling analysis (the
    /// ISSUE_239 `*_test2` decks); ngspice draws it.
    #[test]
    fn xyce_reads_a_statistical_function_at_its_nominal_value() {
        let deck = "statistics\n.param p={agauss(10k,1k,1)}\nV1 1 0 1\nR1 1 0 {p}\n.op\n.end\n";
        let xyce = parse_in(deck, crate::config::SpiceDialect::Xyce).expect("Xyce deck parses");
        assert_eq!(xyce.params.get("p"), Some(1e4));
        let ngspice =
            parse_in(deck, crate::config::SpiceDialect::Ngspice).expect("ngspice deck parses");
        assert_ne!(ngspice.params.get("p"), Some(1e4));
    }

    /// Xyce spells the geometry scale `.options parser scale=`; it is the
    /// same factor as `.options scale`, with no "unknown key" warning.
    #[test]
    fn xyce_parser_scale_is_the_geometry_scale() {
        let netlist = parse_in(
            "parser scale\nV1 1 0 1\nR1 1 0 1k\n.options parser scale=1.0e-6\n.op\n.end\n",
            crate::config::SpiceDialect::Xyce,
        )
        .expect("parser scale parses");
        assert_eq!(netlist.options.scale, Some(1e-6));
        assert!(netlist.diagnostics.is_empty(), "{:?}", netlist.diagnostics);
    }

    /// `numfreq=11,7` lists one count per tone, as `numfreq=11 numfreq2=7`.
    #[test]
    fn hbint_numfreq_takes_one_count_per_tone() {
        let netlist = parse_in(
            "hb tones\nV1 1 0 1\nR1 1 0 1k\n.options hbint numfreq=11,7 tahb=1\n.hb 1e5 1.1e5\n.end\n",
            crate::config::SpiceDialect::Xyce,
        )
        .expect("a per-tone list parses");
        assert_eq!(netlist.options.hb_num_frequencies, [11, 7]);
    }

    /// ngspice prints a branch current as `<device>#branch` (the mes/subth
    /// and mesa decks print `vids#branch`): it is that device's current.
    #[test]
    fn a_branch_vector_name_is_the_device_current() {
        assert_eq!(
            super::parse_save_probe("vids#branch"),
            Some(SaveSignal::Current("vids".to_string()))
        );
        assert_eq!(
            super::parse_save_probe("X1.VD#BRANCH"),
            Some(SaveSignal::Current("X1.VD".to_string()))
        );
        assert_eq!(
            super::parse_save_probe("#branch"),
            Some(SaveSignal::Raw("#branch".to_string()))
        );
        let netlist = Netlist::parse(
            "branch\nV1 1 0 1\nvids 1 2 0\nR1 2 0 1k\n.dc v1 0 1 1\n.print dc vids#branch\n.end\n",
        )
        .expect("a branch vector prints");
        assert!(matches!(
            netlist.output_requests[0].operand_kinds.as_slice(),
            [crate::netlist::OutputOperandKind::Probe(SaveSignal::Current(device))] if device == "vids"
        ));
    }

    /// TSTEP=0 is LTspice's and Xyce's "no print step"; ngspice refuses it
    /// with its own message (transetp.c).
    #[test]
    fn a_zero_tran_step_is_refused_only_in_the_ngspice_dialect() {
        let deck = "zero step\nR1 1 0 1\n.tran 0 20u 0 0.1u\n.end\n";
        for dialect in [
            crate::config::SpiceDialect::BestAvailable,
            crate::config::SpiceDialect::Xyce,
        ] {
            let netlist = parse_in(deck, dialect).expect("TSTEP=0 parses");
            assert_eq!(
                transient_fields(&netlist),
                (0.0, 20e-6, Some(0.0), Some(0.1e-6), false),
                "{dialect:?}"
            );
        }
        let error = parse_in(deck, crate::config::SpiceDialect::Ngspice)
            .expect_err("ngspice refuses TSTEP=0");
        assert!(
            matches!(
                error,
                ParseError::Syntax { line: 3, ref message }
                    if message == "TSTEP is invalid, must be greater than zero."
            ),
            "{error:?}"
        );
    }

    #[test]
    fn dc_sweep_accepts_contiguous_punctuation_rich_source_names() {
        let netlist = Netlist::parse(
            "punctuation-rich dc source\n\
             v1` 1` 0 1\n\
             r1` 1` 0 1\n\
             .dc v1` 1 1 1\n\
             .print dc v(1`)\n\
             .end\n",
        )
        .expect("DC source names may contain Xyce DEV punctuation");
        let Some(crate::netlist::AnalysisCommand::Dc { source, .. }) = netlist.analyses.first()
        else {
            panic!("expected DC analysis");
        };
        assert_eq!(source, "V1`");
    }

    #[test]
    fn dc_sweep_punctuation_family_cards_parse_without_consuming_values() {
        for source in [
            "v1`", "v1~", "v1!", "v1@", "v1#", "v1$", "v1%", "v1^", "v1&", "v1*", "v1-", "v1_",
            "v1+", "v1[", "v1]", "v1|", "v1\\", "v1<", "v1>", "v1.", "v1/",
        ] {
            let deck = format!(
                "punctuation-rich dc source\n{source} 1 0 1\nR1 1 0 1\n.DC {source} 1 1 1\n.end\n"
            );
            Netlist::parse(&deck).unwrap_or_else(|error| {
                panic!("{source} must preserve the following DC values: {error}")
            });
        }
    }

    #[test]
    fn xyce_mode_retains_standalone_dollar_and_double_slash_fields() {
        let netlist = Netlist::parse_with_options(
            "standalone punctuation fields\n\
             V$ $ 0 1\n\
             R// // 0 1\n\
             .DC V$ 1 1 1\n\
             .PRINT DC V($) V(//)\n\
             .END\n",
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("Xyce permits standalone $ and // DEV/NODE fields");

        let voltage = netlist
            .elements
            .iter()
            .find(|element| element.name == "V$")
            .expect("standalone-dollar voltage source is retained");
        assert_eq!(voltage.nodes, ["$", "0"]);
        let resistor = netlist
            .elements
            .iter()
            .find(|element| element.name == "R//")
            .expect("double-slash resistor name is retained");
        assert_eq!(resistor.nodes, ["//", "0"]);
    }

    /// ngspice prints `.print tran @m1[Vbs], V(g)/10` (bsim3soifd
    /// RampVg2.cir) as the columns `@m1[vbs]` and `v(g)/10`, and
    /// `.print dc -i(vc)` (vbic FO.cir) as `-i(vc)`: its vector parser joins
    /// operands across operators and whitespace. Both were refused here
    /// ("Unexpected token '/'", "Unexpected token '-'").
    #[test]
    fn output_directives_take_unbraced_expressions() {
        for dialect in [
            crate::config::ExpressionDialect::Ngspice,
            crate::config::ExpressionDialect::Xyce,
        ] {
            let netlist = Netlist::parse_with_options(
                "unbraced output expressions\nV1 g 0 1\nR1 g s 1k\nR2 s 0 1k\n\
                 .print tran @r1[i], V(g)/10\n\
                 .print dc -i(v1)\n\
                 .plot tran v(g) - v(s) v(s) 2*v(s)\n\
                 .probe v(g)*-2\n\
                 .end\n",
                crate::netlist::NetlistParseOptions {
                    expression_dialect: dialect,
                    ..Default::default()
                },
            )
            .unwrap_or_else(|error| panic!("{dialect:?}: {error}"));
            let columns = |index: usize| {
                let request = &netlist.output_requests[index];
                request
                    .operands
                    .iter()
                    .zip(&request.operand_kinds)
                    .map(|(operand, kind)| {
                        (
                            operand.as_str(),
                            matches!(kind, super::OutputOperandKind::Expression { .. }),
                        )
                    })
                    .collect::<Vec<_>>()
            };
            assert_eq!(
                columns(0),
                [("@r1[i]", false), ("V(g)/10", true)],
                "{dialect:?}"
            );
            assert_eq!(columns(1), [("-i(v1)", true)], "{dialect:?}");
            assert_eq!(
                columns(2),
                [("v(g) - v(s)", true), ("v(s)", false), ("2*v(s)", true)],
                "{dialect:?}"
            );
            assert_eq!(netlist.output_requests[1].expressions, ["-i(v1)"]);
            assert_eq!(netlist.output_requests.len(), 4, "{dialect:?}");
        }
        // ngspice also takes `.save @m1[Vbs], V(g)/10` (bsim3soipd
        // RampVg2.cir) and keeps what the expression reads.
        let netlist =
            Netlist::parse("save expression\nV1 g 0 1\n.save @v1[i], V(g)/10\n.end\n").unwrap();
        let save = &netlist.output_requests[0];
        assert!(
            save.dependencies
                .iter()
                .any(|dependency| dependency.symbol.eq_ignore_ascii_case("g")),
            "{:?}",
            save.dependencies
        );
    }

    /// `X` is HSPICE's mega and RSpice's. ngspice 46 knows no `x`: on
    /// `R1 a 0 1x`, `.param p={2x}` and a subckt default `q=4x` it draws
    /// 1 A, 0.5 A and 0.25 A from 1 V sources. The ngspice dialect reads
    /// them so; the others keep mega.
    #[test]
    fn the_x_suffix_is_mega_except_in_the_ngspice_dialect() {
        let deck = "x suffix\n.param p={2x}\nV1 a 0 1\nR1 a 0 1x\nX1 c s\n\
                    .subckt s n q=4x\nR3 n 0 {q}\n.ends\n.end\n";
        for (dialect, unit) in [
            (None, 1e6),
            (Some(crate::config::SpiceDialect::BestAvailable), 1e6),
            (Some(crate::config::SpiceDialect::Ngspice), 1.0),
        ] {
            let netlist = Netlist::parse_with_options(
                deck,
                crate::netlist::NetlistParseOptions {
                    spice_dialect: dialect,
                    ..Default::default()
                },
            )
            .unwrap();
            assert_eq!(netlist.params.get("p"), Some(2.0 * unit), "{dialect:?}");
            let resistance = |elements: &[crate::netlist::Element], name: &str| {
                elements
                    .iter()
                    .find(|element| element.name.eq_ignore_ascii_case(name))
                    .and_then(|element| match &element.kind {
                        crate::netlist::ElementKind::Resistor { value, .. } => Some(*value),
                        _ => None,
                    })
                    .unwrap_or_else(|| panic!("{dialect:?}: no resistor {name}"))
            };
            assert_eq!(resistance(&netlist.elements, "R1"), unit, "{dialect:?}");
            let flattened = crate::netlist::flattener::flatten_netlist(&netlist).unwrap();
            assert_eq!(resistance(&flattened, "X1.R3"), 4.0 * unit, "{dialect:?}");
        }
    }
}
