//! Dot-command parsing for analyses, options, measurements, params, and functions.

use crate::config::DampingStrategy;
use crate::netlist::lexer::Token;
use crate::netlist::{XspiceAutoBridgeParamName, XspiceAutoBridgeTemplate};
use crate::numerics::integration::{TransientErrorControl, TransientLteReference};
use crate::solver::RealSolverBackend;

use super::*;

pub(super) fn parse_command(
    stream: &mut TokenStream,
    line_num: usize,
    context: ParseCommandContext<'_>,
) -> Result<(), ParseError> {
    let ParseCommandContext {
        logical_line,
        analyses,
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
        saves,
        output_requests,
        startup_directives,
        startup_scope,
        options,
        max_analysis_points,
        output_initial_interval_seen,
        diagnostics,
        spef_includes,
        origin,
        defer_scoped_values,
        deferred_body_params,
        model_bare_ident_deferrals,
    } = context;

    let cmd = expect_ident(stream, line_num)?;
    let mut require_line_consumed = true;

    match cmd.as_str() {
        ".OP" => {
            analyses.push(AnalysisCommand::Op);
        }
        ".DC" => {
            let (source, spec) = parse_dc_sweep_spec(stream, line_num, params)?;

            push_xyce_inconsistent_dc_sweep_warning(params, diagnostics, origin, &source, &spec);

            // Optional second (outer) source: .DC V1 a b s V2 a2 b2 s2
            skip_commas(stream);
            let sweep2 = if matches!(stream.peek().kind, TokenKind::Ident(_)) {
                let (source2, spec2) = parse_dc_sweep_spec(stream, line_num, params)?;
                push_xyce_inconsistent_dc_sweep_warning(
                    params,
                    diagnostics,
                    origin,
                    &source2,
                    &spec2,
                );
                Some(crate::netlist::DcSecondSweep {
                    source: source2,
                    start: spec2.start,
                    stop: spec2.stop,
                    step: spec2.step,
                    mode: spec2.mode,
                })
            } else {
                None
            };

            analyses.push(AnalysisCommand::Dc {
                source,
                start: spec.start,
                stop: spec.stop,
                step: spec.step,
                mode: spec.mode,
                sweep2,
            });
        }
        ".AC" => {
            let var_str = expect_ident(stream, line_num)?;
            if var_str.eq_ignore_ascii_case("DATA") {
                if !stream.consume(&TokenKind::Equals) {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: ".AC DATA requires DATA=<table-name>".to_string(),
                    });
                }
                let table_name = expect_ident(stream, line_num)?;
                analyses.push(AnalysisCommand::AcData { table_name });
                return Ok(());
            }
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

            analyses.push(AnalysisCommand::Ac {
                variation,
                points,
                start_freq,
                stop_freq,
            });
        }
        ".LIN" => {
            parse_lin_command(stream, line_num, params, lin_analysis)?;
        }
        ".HB" => {
            let mut frequencies = Vec::new();
            while !stream.is_eof()
                && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof)
            {
                skip_commas(stream);
                if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
                    break;
                }
                let frequency = expect_value(stream, line_num, params)?;
                if !frequency.is_finite() || frequency <= 0.0 {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!(
                            ".HB frequencies must be positive finite numbers, found {frequency}"
                        ),
                    });
                }
                frequencies.push(frequency);
            }
            if frequencies.is_empty() {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: ".HB requires at least one positive frequency".to_string(),
                });
            }
            analyses.push(AnalysisCommand::Hb { frequencies });
        }
        ".SP" => {
            let sp = parse_sp_command(stream, line_num, params)?;
            analyses.push(sp);
        }
        ".STB" => {
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

            // The probe designates the 0 V source standing in the loop:
            // PROBE=vname (the Spectre flavor) or a bare trailing name.
            let mut probe = expect_ident(stream, line_num).map_err(|_| ParseError::Syntax {
                line: line_num,
                message: ".STB requires a probe: name a 0 V voltage source placed in \
                          the loop, e.g. .STB DEC 10 1 100MEG PROBE=VPRB"
                    .to_string(),
            })?;
            if probe.eq_ignore_ascii_case("probe") && stream.consume(&TokenKind::Equals) {
                probe = expect_ident(stream, line_num)?;
            }

            analyses.push(AnalysisCommand::Stb {
                variation,
                points,
                start_freq,
                stop_freq,
                probe,
            });
        }
        ".DISTO" => {
            let disto = parse_disto_command(stream, line_num, params)?;
            analyses.push(disto);
        }
        ".TRAN" | ".TR" => {
            let step = expect_value(stream, line_num, params)?;
            if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: ".TRAN line has an unexpected number of fields\nUnrecognized dot line will be ignored"
                        .to_string(),
                });
            }
            let stop = expect_value(stream, line_num, params)?;
            let start = try_value(stream, params);
            let max_step = try_value(stream, params);
            let uic = consume_uic_keyword(stream);

            analyses.push(AnalysisCommand::Tran {
                step,
                stop,
                start,
                max_step,
                uic,
            });
        }
        ".MODEL" => {
            let model = parse_model_definition(
                stream,
                line_num,
                params,
                models,
                false,
                model_bare_ident_deferrals,
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
        ".CODEMODEL" | ".RSPICE_UNSUPPORTED_CODEMODEL" => {
            parse_xspice_codemodel_command(stream, line_num, &cmd)?;
        }
        // `.PARAMS` is the plural spelling ngspice accepts for `.PARAM`; IHP's
        // SG13G2 device subcircuits are written with it.
        ".PARAM" | ".PARAMS" | ".CSPARAM" => {
            parse_param_statement(
                stream,
                line_num,
                params,
                deferred_body_params,
                false,
                diagnostics,
                origin,
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
            )?;
        }
        ".STEP" => {
            let step_cmd = parse_step_command(stream, line_num, params)?;
            analyses.push(AnalysisCommand::Step(step_cmd));
        }
        ".MC" => {
            let mc_cmd = parse_mc_command(stream, line_num, params)?;
            analyses.push(AnalysisCommand::MonteCarlo(mc_cmd));
        }
        ".TEMP" => {
            let temperatures = parse_temp_command(stream, line_num, params)?;
            analyses.push(AnalysisCommand::Temp { temperatures });
        }
        ".FOUR" | ".FOURIER" => {
            let authored_source = remaining_command_source(stream);
            let (fundamental, num_harmonics, outputs) =
                parse_four_command(stream, line_num, params)?;
            output_requests.push(OutputRequest::from_four(
                outputs.as_slice(),
                origin.clone(),
                &authored_source,
            ));
            analyses.push(AnalysisCommand::Four {
                fundamental,
                outputs,
                num_harmonics,
            });
        }
        ".FFT" => {
            fft_analyses.push(parse_fft_command(stream, line_num, params, diagnostics)?);
        }
        ".NOISE" => {
            let noise = parse_noise_command(stream, line_num, params)?;
            analyses.push(noise);
        }
        ".SENS" => {
            let sens = parse_sens_command(stream, line_num, params)?;
            analyses.push(sens);
        }
        ".PZ" => {
            let pz = parse_pz_command(stream, line_num)?;
            analyses.push(pz);
        }
        ".IC" => {
            let first_entry = initial_conditions.len();
            let authored_nodes = parse_ic_command(
                stream,
                line_num,
                params,
                initial_conditions,
                defer_scoped_values,
            )?;
            startup_directives.push(startup_directive_record(
                StartupDirectiveKind::Ic,
                origin,
                startup_scope.clone(),
                initial_conditions[first_entry..]
                    .iter()
                    .zip(authored_nodes.iter())
                    .map(|(entry, authored_node)| {
                        (
                            authored_node.as_str(),
                            entry.node.as_str(),
                            entry.voltage,
                            entry.voltage_expr.as_deref(),
                        )
                    }),
            ));
        }
        ".INITCOND" => {
            parse_device_initial_condition_command(
                stream,
                line_num,
                params,
                origin,
                device_initial_conditions,
            )?;
        }
        ".NODESET" => {
            let first_entry = node_sets.len();
            let authored_nodes =
                parse_nodeset_command(stream, line_num, params, node_sets, defer_scoped_values)?;
            startup_directives.push(startup_directive_record(
                StartupDirectiveKind::NodeSet,
                origin,
                startup_scope,
                node_sets[first_entry..]
                    .iter()
                    .zip(authored_nodes.iter())
                    .map(|(entry, authored_node)| {
                        (
                            authored_node.as_str(),
                            entry.node.as_str(),
                            entry.voltage,
                            entry.voltage_expr.as_deref(),
                        )
                    }),
            ));
        }
        ".INCLUDE" | ".INC" => {
            // Include directives are handled in a preprocessing pass
            log::debug!("Include directive found: line {}", line_num);
            require_line_consumed = false;
        }
        ".SPEF_INCLUDE" => {
            // SPEF parasitics back-annotate after parsing (netlist::spef);
            // the path-aware parse entry points resolve and apply them.
            let path = match &stream.peek().kind {
                TokenKind::StringLit(s) => s.clone(),
                TokenKind::Ident(s) => s.clone(),
                _ => {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: ".spef_include requires a file path".to_string(),
                    });
                }
            };
            stream.advance();
            spef_includes.push(path);
        }
        ".LIB" => {
            // Library directives are handled in a preprocessing pass
            log::debug!("Library directive found: line {}", line_num);
            require_line_consumed = false;
        }
        ".GLOBAL" => {
            parse_global_command(stream, line_num, global_nodes)?;
        }
        ".FUNC" => {
            // Parse user-defined function: .FUNC name(arg1, arg2, ...) = expression
            parse_func_statement(stream, line_num, params)?;
        }
        ".TF" => {
            analyses.push(parse_tf_command(stream, line_num)?);
        }
        ".PREPROCESS" => parse_preprocess_command(stream, line_num, diagnostics)?,
        ".OPTIONS" | ".OPTION" | ".OPT" => parse_options_command(
            stream,
            line_num,
            params,
            options,
            max_analysis_points,
            output_initial_interval_seen,
            unknown_warned,
            diagnostics,
        )?,
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
                params,
                None,
                None,
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
                params,
                Some(diagnostics),
                Some(origin),
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
            if unknown_warned.insert(key) {
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
            require_line_consumed = false;
        }
    }

    if require_line_consumed {
        reject_unconsumed_command_tokens(stream, line_num, &cmd)?;
    }

    Ok(())
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
    lin_analysis: &mut Option<crate::netlist::LinAnalysis>,
) -> Result<(), ParseError> {
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
    if lin_analysis
        .replace(crate::netlist::LinAnalysis::AcOnly)
        .is_some()
    {
        return Err(ParseError::Syntax {
            line: line_num,
            message: ".LIN may appear only once in a netlist".to_string(),
        });
    }
    Ok(())
}

fn startup_directive_record<'a>(
    kind: StartupDirectiveKind,
    origin: &NetlistSourceLocation,
    scope: StartupDirectiveScope,
    entries: impl IntoIterator<Item = (&'a str, &'a str, Value, Option<&'a str>)>,
) -> StartupDirectiveRecord {
    let entries = entries
        .into_iter()
        .map(
            |(authored_node, execution_node, voltage, voltage_expr)| StartupDirectiveEntry {
                authored_node: authored_node.to_string(),
                execution_node: execution_node.to_string(),
                canonical_node: execution_node.replace(':', ".").to_ascii_uppercase(),
                qualified_nodes: Vec::new(),
                disposition: StartupDirectiveDisposition::Applied,
                voltage,
                voltage_expr: voltage_expr.map(ToString::to_string),
            },
        )
        .collect::<Vec<_>>();
    let disposition = if entries.is_empty() {
        StartupDirectiveDisposition::Ignored(StartupDiagnosticCode::EmptyDirective)
    } else {
        StartupDirectiveDisposition::Applied
    };
    StartupDirectiveRecord {
        kind,
        origin: origin.clone(),
        scope,
        entries,
        disposition,
    }
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

pub(super) fn parse_save_command(
    stream: &mut TokenStream,
    line_num: usize,
    logical_line: &str,
    saves: &mut super::SaveSet,
    skip_analysis_type: bool,
    params: &ParamContext,
    mut diagnostics: Option<&mut Vec<ParseDiagnostic>>,
    origin: Option<&NetlistSourceLocation>,
) -> Result<ParsedSaveCommand, ParseError> {
    use super::SaveSignal;

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
        skip_commas(stream);
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }

        match &stream.peek().kind {
            TokenKind::Ident(raw) => {
                let mut raw = raw.clone();
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
                        let device = s.clone();
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
                            let param = s.clone();
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
    let lower = trimmed.to_ascii_lowercase();

    if lower == "all" {
        return Some(SaveSignal::All);
    }

    if let Some(inner) = lower.strip_prefix("v(").and_then(|s| s.strip_suffix(')')) {
        let inner = inner.trim();
        if inner.is_empty() {
            return None;
        }
        return Some(match inner.split_once(',') {
            Some((a, b)) => SaveSignal::VoltageDiff(a.trim().to_string(), b.trim().to_string()),
            None => SaveSignal::Voltage(inner.to_string()),
        });
    }

    if let Some(inner) = lower.strip_prefix("i(").and_then(|s| s.strip_suffix(')')) {
        let inner = inner.trim();
        if inner.is_empty() {
            return None;
        }
        return Some(SaveSignal::Current(inner.to_string()));
    }

    if let Some(inner) = lower.strip_prefix("n(").and_then(|s| s.strip_suffix(')')) {
        if let Some((device, param)) = inner.split_once(':') {
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

    if let Some(rest) = lower.strip_prefix('@') {
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

    Some(SaveSignal::Raw(lower))
}

pub(super) fn parse_options_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    options: &mut super::SimulationOptions,
    max_analysis_points: usize,
    output_initial_interval_seen: &mut bool,
    unknown_warned: &mut std::collections::HashSet<String>,
    diagnostics: &mut Vec<ParseDiagnostic>,
) -> Result<(), ParseError> {
    let mut option_package: Option<String> = None;

    while !stream.is_eof() {
        skip_commas(stream);
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }

        // Xyce's RESTART package uses a deliberately irregular tail grammar:
        // after the ordinary NAME=VALUE fields, bare values occur in
        // `<time> <interval>` pairs. Detect that tail before asking for an
        // option key, since its first token is normally numeric.
        if option_package.as_deref() == Some("RESTART")
            && restart_interval_schedule_starts(stream, params)
        {
            parse_restart_interval_schedule(
                stream,
                line_num,
                params,
                options,
                max_analysis_points,
            )?;
            break;
        }

        let (key, key_end) = expect_option_key(stream, line_num)?;
        let key_upper = key.to_uppercase();
        let has_equals = stream.consume(&TokenKind::Equals);

        // A value accepted without `=` must still be separated from its key.
        // Xyce treats a fused spelling such as `ABSTOL-1e-6` as one unknown
        // option token; it does not reinterpret the adjacent minus sign as an
        // assigned negative tolerance. Preserve that lexical boundary so a
        // malformed key is diagnosed and ignored instead of becoming a fatal
        // value error for an otherwise valid deck.
        if !has_equals
            && stream.peek().span.start == key_end
            && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof)
        {
            let fused_key = format!("{key_upper}{}", stream.peek().lexeme.to_uppercase());
            stream.advance();
            let warning_key = option_package
                .as_deref()
                .map_or(fused_key.clone(), |package| {
                    format!("{package}.{fused_key}")
                });
            ignore_unknown_option(
                stream,
                line_num,
                params,
                false,
                &warning_key,
                unknown_warned,
                diagnostics,
            );
            continue;
        }

        let is_supported_linsol_package = key_upper == "LINSOL"
            && matches!(&stream.peek().kind, TokenKind::Ident(next)
                if next.eq_ignore_ascii_case("TR_PARTITION")
                    || next.eq_ignore_ascii_case("TRPARTITION"));
        if !has_equals && (option_package_key_is_known(&key_upper) || is_supported_linsol_package) {
            if key_upper == "RESTART" {
                options.restart.get_or_insert_default();
            }
            option_package = Some(key_upper);
            continue;
        }

        let scoped_key = option_package
            .as_deref()
            .map(|package| format!("{package}.{key_upper}"));

        match (option_package.as_deref(), key_upper.as_str()) {
            (Some("MEASURE"), "MEASFAIL") => {
                let value = expect_value(stream, line_num, params)?;
                if !value.is_finite() {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!("MEASURE.MEASFAIL must be finite, found {value}"),
                    });
                }
                let integer_value = value.trunc();
                options.measure_fail_output = Some(match integer_value {
                    0.0 => false,
                    1.0 => true,
                    _ => {
                        let message = format!(
                            "MEASURE.MEASFAIL expects 0 or 1; defaulting invalid value {value} to 1"
                        );
                        log::warn!("line {line_num}: {message}");
                        diagnostics.push(ParseDiagnostic::warning(
                            line_num,
                            "invalid-option-defaulted",
                            message,
                        ));
                        true
                    }
                });
            }
            (Some("MEASURE"), "DEFAULT_VAL") => {
                let value = expect_value(stream, line_num, params)?;
                if !value.is_finite() {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!("MEASURE.DEFAULT_VAL must be finite, found {value}"),
                    });
                }
                options.measure_default_value = Some(value);
            }
            (Some("MEASURE"), "USE_CONT_FILES") => {
                let value = expect_value(stream, line_num, params)?;
                if !value.is_finite() {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!("MEASURE.USE_CONT_FILES must be finite, found {value}"),
                    });
                }
                options.measure_use_cont_files = Some(value.trunc() != 0.0);
            }
            (Some("MEASURE"), "USE_LTTM") => {
                let value = expect_value(stream, line_num, params)?;
                if !value.is_finite() {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!("MEASURE.USE_LTTM must be finite, found {value}"),
                    });
                }
                options.measure_use_lttm = Some(match value.trunc() {
                    0.0 => false,
                    1.0 => true,
                    _ => {
                        let message = format!(
                            "MEASURE.USE_LTTM expects 0 or 1; defaulting invalid value {value} to 1"
                        );
                        log::warn!("line {line_num}: {message}");
                        diagnostics.push(ParseDiagnostic::warning(
                            line_num,
                            "invalid-option-defaulted",
                            message,
                        ));
                        true
                    }
                });
            }
            (Some("MEASURE"), _) => {
                let warning_key = scoped_key.as_deref().unwrap_or(&key_upper);
                ignore_unknown_option(
                    stream,
                    line_num,
                    params,
                    has_equals,
                    warning_key,
                    unknown_warned,
                    diagnostics,
                );
            }
            (Some("NONLIN"), "CONTINUATION") | (Some("NONLIN-CONTINUATION"), "CONTINUATION") => {
                options.nonlinear_continuation = Some(parse_nonlinear_continuation_option(
                    stream, line_num, params,
                )?);
            }
            (Some("HBINT"), key) if key.starts_with("NUMFREQ") => {
                let value = expect_value(stream, line_num, params)?;
                let count = parse_usize_option("HBINT.NUMFREQ", value, line_num)?;
                if count == 0 {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: "HBINT.NUMFREQ must be a positive integer, found 0".to_string(),
                    });
                }
                options.hb_num_frequencies.push(count);
            }
            (Some("HBINT"), _) => {
                let warning_key = scoped_key.as_deref().unwrap_or(&key_upper);
                ignore_unknown_option(
                    stream,
                    line_num,
                    params,
                    has_equals,
                    warning_key,
                    unknown_warned,
                    diagnostics,
                );
            }
            (Some("TOPOLOGY"), "SUPERNODE") => {
                options.topology_supernode =
                    Some(parse_boolean_option(stream, line_num, params, has_equals)?);
            }
            (Some("DEVICE"), "ZERORESISTANCETOL" | "ZERO_RESISTANCE_TOL") => {
                let value = expect_value(stream, line_num, params)?;
                options.device_zero_resistance_tol = Some(parse_non_negative_real_option(
                    "DEVICE.ZERORESISTANCETOL",
                    value,
                    line_num,
                )?);
            }
            (Some("DEVICE"), "MINRES" | "MIN_RES") => {
                let value = expect_value(stream, line_num, params)?;
                options.device_min_resistance = Some(parse_non_negative_real_option(
                    "DEVICE.MINRES",
                    value,
                    line_num,
                )?);
            }
            (Some("DEVICE"), "MINCAP" | "MIN_CAP") => {
                let value = expect_value(stream, line_num, params)?;
                options.device_min_capacitance = Some(parse_non_negative_real_option(
                    "DEVICE.MINCAP",
                    value,
                    line_num,
                )?);
            }
            (Some("DEVICE"), "B3SOIGMINSCALING" | "B3SOI_GMIN_SCALING") => {
                options.b3soi_gmin_scaling =
                    Some(parse_boolean_option(stream, line_num, params, has_equals)?);
            }
            (Some("DEVICE"), "VOLTLIM" | "VOLT_LIM") => {
                options.device_voltage_limiting =
                    Some(parse_boolean_option(stream, line_num, params, has_equals)?);
            }
            (Some("DEVICE"), "DEBUGLEVEL" | "DEBUG_LEVEL") => {
                let value = expect_value(stream, line_num, params)?;
                options.device_debug_level =
                    Some(parse_i64_option("DEVICE.DEBUGLEVEL", value, line_num)?);
            }
            (Some("TIMEINT"), "DEBUGLEVEL" | "DEBUG_LEVEL") => {
                let value = expect_value(stream, line_num, params)?;
                options.timeint_debug_level = Some(parse_xyce_i32_option(
                    "TIMEINT.DEBUGLEVEL",
                    value,
                    line_num,
                )?);
            }
            (Some("DEVICE"), "TRYTOCOMPACT" | "TRY_TO_COMPACT") => {
                options.device_try_to_compact =
                    Some(parse_boolean_option(stream, line_num, params, has_equals)?);
            }
            (None, "TOPOLOGY_SUPERNODE" | "TOPOLOGYSUPERNODE") => {
                options.topology_supernode =
                    Some(parse_boolean_option(stream, line_num, params, has_equals)?);
            }
            (
                None,
                "DEVICE_ZERORESISTANCETOL"
                | "DEVICEZERORESISTANCETOL"
                | "ZERORESISTANCETOL"
                | "ZERO_RESISTANCE_TOL",
            ) => {
                let value = expect_value(stream, line_num, params)?;
                options.device_zero_resistance_tol = Some(parse_non_negative_real_option(
                    "ZERORESISTANCETOL",
                    value,
                    line_num,
                )?);
            }
            (None, "MINRES" | "MIN_RES" | "DEVICE_MINRES" | "DEVICEMINRES") => {
                let value = expect_value(stream, line_num, params)?;
                options.device_min_resistance =
                    Some(parse_non_negative_real_option("MINRES", value, line_num)?);
            }
            (None, "MINCAP" | "MIN_CAP" | "DEVICE_MINCAP" | "DEVICE_MIN_CAP") => {
                let value = expect_value(stream, line_num, params)?;
                options.device_min_capacitance =
                    Some(parse_non_negative_real_option("MINCAP", value, line_num)?);
            }
            (None, "B3SOIGMINSCALING" | "B3SOI_GMIN_SCALING" | "DEVICE_B3SOIGMINSCALING") => {
                options.b3soi_gmin_scaling =
                    Some(parse_boolean_option(stream, line_num, params, has_equals)?);
            }
            (None, "TRYTOCOMPACT" | "TRY_TO_COMPACT" | "DEVICE_TRYTOCOMPACT") => {
                options.device_try_to_compact =
                    Some(parse_boolean_option(stream, line_num, params, has_equals)?);
            }
            (Some("LINSOL"), "TR_PARTITION" | "TRPARTITION") => {
                options.linsol_tr_partition =
                    Some(parse_boolean_option(stream, line_num, params, has_equals)?);
            }
            (Some("XSPICE"), "AUTO_BRIDGE" | "AUTOBRIDGE")
            | (None, "AUTO_BRIDGE" | "AUTOBRIDGE" | "XSPICE_AUTO_BRIDGE") => {
                let (enabled, show_generated) =
                    parse_auto_bridge_option(stream, line_num, params, has_equals)?;
                options.auto_bridge = Some(enabled);
                options.auto_bridge_show_generated = Some(show_generated);
            }
            (Some("NONLIN-TRAN"), "RELTOL") | (Some("NONLIN-TRANSIENT"), "RELTOL") => {
                let value = expect_value(stream, line_num, params)?;
                options.nonlin_transient_reltol = Some(parse_positive_real_option(
                    "NONLIN-TRAN.RELTOL",
                    value,
                    line_num,
                )?);
            }
            (Some("NONLIN-TRAN"), "ABSTOL") | (Some("NONLIN-TRANSIENT"), "ABSTOL") => {
                let value = expect_value(stream, line_num, params)?;
                options.nonlin_transient_abstol = Some(parse_positive_real_option(
                    "NONLIN-TRAN.ABSTOL",
                    value,
                    line_num,
                )?);
            }
            (Some("NONLIN-TRAN"), "DELTAXTOL") | (Some("NONLIN-TRANSIENT"), "DELTAXTOL") => {
                let value = expect_value(stream, line_num, params)?;
                options.nonlin_transient_deltaxtol = Some(parse_positive_real_option(
                    "NONLIN-TRAN.DELTAXTOL",
                    value,
                    line_num,
                )?);
            }
            (Some("NONLIN-TRAN"), "RHSTOL") | (Some("NONLIN-TRANSIENT"), "RHSTOL") => {
                let value = expect_value(stream, line_num, params)?;
                options.nonlin_transient_rhstol = Some(parse_positive_real_option(
                    "NONLIN-TRAN.RHSTOL",
                    value,
                    line_num,
                )?);
            }
            (Some("NONLIN-TRAN"), "MAXSTEP") | (Some("NONLIN-TRANSIENT"), "MAXSTEP") => {
                let value = expect_value(stream, line_num, params)?;
                let value = parse_usize_option("NONLIN-TRAN.MAXSTEP", value, line_num)?;
                if value == 0 {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: "NONLIN-TRAN.MAXSTEP must be at least 1".to_string(),
                    });
                }
                options.nonlin_transient_maxstep = Some(value);
            }
            (Some("NONLIN-TRAN"), "ENFORCEDEVICECONV" | "ENFORCE_DEVICE_CONV")
            | (Some("NONLIN-TRANSIENT"), "ENFORCEDEVICECONV" | "ENFORCE_DEVICE_CONV") => {
                options.nonlin_transient_enforce_device_convergence =
                    Some(parse_boolean_option(stream, line_num, params, has_equals)?);
            }
            (Some("NONLIN-TRAN"), "NOX") | (Some("NONLIN-TRANSIENT"), "NOX") => {
                options.nonlin_transient_nox =
                    Some(parse_boolean_option(stream, line_num, params, has_equals)?);
            }
            (Some("NONLIN-TRAN"), _) | (Some("NONLIN-TRANSIENT"), _) => {
                let warning_key = scoped_key.as_deref().unwrap_or(&key_upper);
                ignore_unknown_option(
                    stream,
                    line_num,
                    params,
                    has_equals,
                    warning_key,
                    unknown_warned,
                    diagnostics,
                );
            }
            (Some("TIMEINT"), "RELTOL") => {
                let value = expect_value(stream, line_num, params)?;
                options.timeint_reltol = Some(parse_positive_real_option(
                    "TIMEINT.RELTOL",
                    value,
                    line_num,
                )?);
            }
            (Some("TIMEINT"), "ABSTOL") => {
                let value = expect_value(stream, line_num, params)?;
                options.timeint_abstol = Some(parse_positive_real_option(
                    "TIMEINT.ABSTOL",
                    value,
                    line_num,
                )?);
            }
            (Some("TIMEINT"), "DELMAX") => {
                let value = expect_value(stream, line_num, params)?;
                options.timeint_delmax = Some(parse_positive_real_option(
                    "TIMEINT.DELMAX",
                    value,
                    line_num,
                )?);
            }
            (Some("TIMEINT"), "MINTIMESTEP" | "MIN_TIMESTEP") => {
                let value = expect_value(stream, line_num, params)?;
                options.timeint_min_timestep = Some(parse_positive_real_option(
                    "TIMEINT.MINTIMESTEP",
                    value,
                    line_num,
                )?);
            }
            (Some("TIMEINT"), "ERROPTION") => {
                let value = expect_value(stream, line_num, params)?;
                if options.timeint_error_control.is_none() {
                    options.timeint_error_control =
                        Some(parse_transient_error_control_option(value, line_num)?);
                } else {
                    warn_duplicate_packaged_option("TIMEINT.ERROPTION", line_num, diagnostics);
                }
            }
            (Some("TIMEINT"), "MINTIMESTEPSBP") => {
                let value = expect_value(stream, line_num, params)?;
                if options.timeint_min_steps_between_breakpoints.is_none() {
                    options.timeint_min_steps_between_breakpoints =
                        Some(parse_exact_xyce_nonnegative_integer_option(
                            "TIMEINT.MINTIMESTEPSBP",
                            value,
                            line_num,
                        )?);
                } else {
                    warn_duplicate_packaged_option("TIMEINT.MINTIMESTEPSBP", line_num, diagnostics);
                }
            }
            (Some("TIMEINT"), "NLMIN") => {
                let value = expect_value(stream, line_num, params)?;
                if options.timeint_nlmin.is_none() {
                    options.timeint_nlmin = Some(parse_exact_xyce_nonnegative_integer_option(
                        "TIMEINT.NLMIN",
                        value,
                        line_num,
                    )?);
                } else {
                    warn_duplicate_packaged_option("TIMEINT.NLMIN", line_num, diagnostics);
                }
            }
            (Some("TIMEINT"), "NLMAX") => {
                let value = expect_value(stream, line_num, params)?;
                if options.timeint_nlmax.is_none() {
                    options.timeint_nlmax = Some(parse_exact_xyce_nonnegative_integer_option(
                        "TIMEINT.NLMAX",
                        value,
                        line_num,
                    )?);
                } else {
                    warn_duplicate_packaged_option("TIMEINT.NLMAX", line_num, diagnostics);
                }
            }
            (Some("TIMEINT"), "TIMESTEPSREVERSAL") => {
                let value = expect_value(stream, line_num, params)?;
                if options.timeint_timesteps_reversal.is_none() {
                    options.timeint_timesteps_reversal = Some(parse_binary_integer_option(
                        "TIMESTEPSREVERSAL",
                        value,
                        line_num,
                    )?);
                } else {
                    warn_duplicate_packaged_option(
                        "TIMEINT.TIMESTEPSREVERSAL",
                        line_num,
                        diagnostics,
                    );
                }
            }
            (Some("TIMEINT"), "MINORD") => {
                let value = expect_value(stream, line_num, params)?;
                if options.timeint_min_order.is_none() {
                    options.timeint_min_order = Some(parse_xyce_transient_order_option(
                        "TIMEINT.MINORD",
                        value,
                        line_num,
                    )?);
                } else {
                    warn_duplicate_packaged_option("TIMEINT.MINORD", line_num, diagnostics);
                }
            }
            (Some("TIMEINT"), "MAXORD") => {
                let value = expect_value(stream, line_num, params)?;
                if options.timeint_max_order.is_none() {
                    options.timeint_max_order = Some(parse_xyce_transient_order_option(
                        "TIMEINT.MAXORD",
                        value,
                        line_num,
                    )?);
                } else {
                    warn_duplicate_packaged_option("TIMEINT.MAXORD", line_num, diagnostics);
                }
            }
            (Some("TIMEINT"), "BREAKPOINTS") => {
                let values = parse_time_point_vector_option(
                    stream,
                    line_num,
                    params,
                    "TIMEINT.BREAKPOINTS",
                    options
                        .timeint_breakpoints
                        .len()
                        .saturating_add(options.output_time_points.len())
                        .saturating_add(restart_interval_count(options)),
                    max_analysis_points,
                )?;
                append_canonical_time_points(
                    &mut options.timeint_breakpoints,
                    values,
                    &options.output_time_points,
                    max_analysis_points,
                )?;
            }
            // The run's step ceiling is deliberately outside `TIMEINT`: that
            // package's own ceiling is `DELMAX`, and a second key inside it
            // meaning the same thing would make a deck line ambiguous. Being
            // unscoped also means a misplaced `TIMEINT MAXTIMESTEP` is
            // reported by the package's unknown-key arm instead of silently
            // taking effect.
            (None, "MAXTIMESTEP" | "MAX_TIMESTEP") => {
                let value = expect_value(stream, line_num, params)?;
                options.max_timestep =
                    Some(parse_positive_real_option("MAXTIMESTEP", value, line_num)?);
            }
            (Some("TIMEINT"), "USEDEVICEMAX" | "USE_DEVICE_MAX") => {
                options.timeint_use_device_max_timestep =
                    Some(parse_boolean_option(stream, line_num, params, has_equals)?);
            }
            (Some("TIMEINT"), "NEWLTE") => {
                let value = expect_value(stream, line_num, params)?;
                options.transient_lte_reference =
                    Some(parse_transient_lte_reference_option(value, line_num)?);
            }
            (Some("TIMEINT"), "NEWBPSTEPPING") => {
                let value = expect_value(stream, line_num, params)?;
                options.transient_new_bp_stepping =
                    Some(parse_new_breakpoint_stepping_option(value, line_num)?);
            }
            (Some("TIMEINT"), "METHOD") => {
                options.method = Some(parse_method_option(stream, line_num, params)?);
            }
            (Some("TIMEINT"), _) => {
                let warning_key = scoped_key.as_deref().unwrap_or(&key_upper);
                ignore_unknown_option(
                    stream,
                    line_num,
                    params,
                    has_equals,
                    warning_key,
                    unknown_warned,
                    diagnostics,
                );
            }
            (Some("RESTART"), "PACK") => {
                let value = parse_restart_boolean_option(
                    stream,
                    line_num,
                    params,
                    has_equals,
                    "RESTART.PACK",
                )?;
                options.restart.get_or_insert_default().pack = Some(value);
            }
            (Some("RESTART"), "PRINT_TIMEINT_OPTIONS" | "PRINTTIMEINTOPTIONS") => {
                let value = parse_restart_boolean_option(
                    stream,
                    line_num,
                    params,
                    has_equals,
                    "RESTART.PRINT_TIMEINT_OPTIONS",
                )?;
                options
                    .restart
                    .get_or_insert_default()
                    .print_timeint_options = Some(value);
            }
            (Some("RESTART"), "JOB") => {
                let value = parse_restart_string_option(stream, line_num, "RESTART.JOB")?;
                options.restart.get_or_insert_default().job = Some(value);
            }
            (Some("RESTART"), "START_TIME" | "STARTTIME") => {
                let value = expect_value(stream, line_num, params)?;
                let value = parse_non_negative_real_option("RESTART.START_TIME", value, line_num)?;
                options.restart.get_or_insert_default().start_time = Some(value);
            }
            (Some("RESTART"), "FILE") => {
                let value = parse_restart_string_option(stream, line_num, "RESTART.FILE")?;
                options.restart.get_or_insert_default().file = Some(value);
            }
            (Some("RESTART"), "INITIAL_INTERVAL" | "INITIALINTERVAL") => {
                let value = expect_value(stream, line_num, params)?;
                let value =
                    parse_positive_real_option("RESTART.INITIAL_INTERVAL", value, line_num)?;
                options.restart.get_or_insert_default().initial_interval = Some(value);
            }
            (Some("RESTART"), _) => {
                let warning_key = scoped_key.as_deref().unwrap_or(&key_upper);
                ignore_unknown_option(
                    stream,
                    line_num,
                    params,
                    has_equals,
                    warning_key,
                    unknown_warned,
                    diagnostics,
                );
            }
            // Spelled unscoped, as ngspice spells `bypass`. A `BYPASS` package
            // would have to enumerate every key it accepts before an
            // unenumerated one leaked back out to the global namespace, and
            // three keys do not need a namespace to stay apart.
            (_, "BYPASS") => {
                options.bypass = Some(parse_boolean_option(stream, line_num, params, has_equals)?);
            }
            (_, "BYPASSRELTOL" | "BYPASS_RELTOL") => {
                let value = expect_value(stream, line_num, params)?;
                options.bypass_reltol = Some(parse_non_negative_real_option(
                    "BYPASSRELTOL",
                    value,
                    line_num,
                )?);
            }
            (_, "BYPASSABSTOL" | "BYPASS_ABSTOL") => {
                let value = expect_value(stream, line_num, params)?;
                options.bypass_abstol = Some(parse_non_negative_real_option(
                    "BYPASSABSTOL",
                    value,
                    line_num,
                )?);
            }
            (_, "RELTOL") => {
                let value = expect_value(stream, line_num, params)?;
                options.reltol = Some(parse_positive_real_option("RELTOL", value, line_num)?);
            }
            (_, "ABSTOL") => {
                let value = expect_value(stream, line_num, params)?;
                options.abstol = Some(parse_positive_real_option("ABSTOL", value, line_num)?);
            }
            (_, "VNTOL") => {
                let value = expect_value(stream, line_num, params)?;
                options.vntol = Some(parse_positive_real_option("VNTOL", value, line_num)?);
            }
            (_, "IABSTOL") => {
                let value = expect_value(stream, line_num, params)?;
                options.iabstol = Some(parse_positive_real_option("IABSTOL", value, line_num)?);
            }
            (_, "RESIDUAL_RELTOL" | "RESRELTOL") => {
                let value = expect_value(stream, line_num, params)?;
                options.residual_reltol = Some(parse_positive_real_option(
                    "RESIDUAL_RELTOL",
                    value,
                    line_num,
                )?);
            }
            (_, "GMIN") => {
                let value = expect_value(stream, line_num, params)?;
                options.gmin = Some(parse_non_negative_real_option("GMIN", value, line_num)?);
            }
            (_, "RSHUNT") => {
                let value = expect_value(stream, line_num, params)?;
                options.rshunt = Some(parse_positive_real_option("RSHUNT", value, line_num)?);
            }
            (_, "CSHUNT") => {
                let value = expect_value(stream, line_num, params)?;
                options.cshunt = Some(parse_positive_real_option("CSHUNT", value, line_num)?);
            }
            (None, "XMU")
                if params.expression_dialect() != crate::config::ExpressionDialect::Xyce =>
            {
                let value = expect_value(stream, line_num, params)?;
                options.xmu = Some(parse_xmu_option(value, line_num)?);
            }
            (_, "TRTOL") => {
                let value = expect_value(stream, line_num, params)?;
                options.trtol = Some(parse_positive_real_option("TRTOL", value, line_num)?);
            }
            (_, "RAMPTIME") => {
                let value = expect_value(stream, line_num, params)?;
                options.ramptime =
                    Some(parse_non_negative_real_option("RAMPTIME", value, line_num)?);
            }
            (Some("XSPICE"), "DIGITAL_DELAY_TYPE" | "DIGITALDELAYTYPE" | "DIGITAL_DELAY")
            | (None, "DIGITAL_DELAY_TYPE" | "DIGITALDELAYTYPE" | "XSPICE_DIGITAL_DELAY_TYPE") => {
                let value = expect_value(stream, line_num, params)?;
                options.digital_delay_type =
                    Some(parse_digital_delay_type_option(value, line_num)?);
            }
            (Some("XSPICE"), "ESAVE" | "EVENT_SAVE" | "EVENTSAVE")
            | (None, "XSPICE_ESAVE" | "XSPICE_EVENT_SAVE" | "XSPICE_EVENTSAVE") => {
                options.xspice_event_trace_save =
                    Some(parse_boolean_option(stream, line_num, params, has_equals)?);
            }
            (_, "CHGTOL") => {
                let value = expect_value(stream, line_num, params)?;
                options.chgtol = Some(parse_positive_real_option("CHGTOL", value, line_num)?);
            }
            (_, "PIVTOL") => {
                let value = expect_value(stream, line_num, params)?;
                options.pivtol = Some(parse_positive_real_option("PIVTOL", value, line_num)?);
            }
            (_, "PIVREL") => {
                let value = expect_value(stream, line_num, params)?;
                options.pivrel = Some(parse_positive_real_option("PIVREL", value, line_num)?);
            }
            (None | Some("DEVICE"), "TEMP") => {
                let value = expect_value(stream, line_num, params)?;
                options.temp = Some(parse_celsius_option("TEMP", value, line_num)?);
            }
            (None | Some("DEVICE"), "TNOM") => {
                let value = expect_value(stream, line_num, params)?;
                options.tnom = Some(parse_celsius_option("TNOM", value, line_num)?);
            }
            (_, "SCALE") => {
                // Element geometry scale factor. ngspice exposes it as the
                // `scale` shell variable and reads it in device setup; the
                // geometric LEVEL=3 diode is the first RSpice device to derive
                // dimensions from it.
                let value = expect_value(stream, line_num, params)?;
                options.scale = Some(parse_positive_real_option("SCALE", value, line_num)?);
            }
            (_, "SEED" | "RNDSEED") => {
                // The parse pre-scan applies the seed before any parameter
                // evaluation; this arm validates and records it for
                // downstream drivers (e.g. per-run Monte-Carlo streams).
                if let TokenKind::Ident(word) = &stream.peek().kind
                    && word.eq_ignore_ascii_case("random")
                {
                    stream.advance();
                    log::warn!(
                        "line {line_num}: `.options seed=random` is not supported; \
                         the deterministic default seed is used (set an explicit \
                         integer seed to vary the stream)"
                    );
                    continue;
                }
                let value = expect_value(stream, line_num, params)?;
                options.seed = Some(parse_seed_option(value, line_num)?);
            }
            (_, "ITL1") => {
                let value = expect_value(stream, line_num, params)?;
                options.itl1 = Some(parse_usize_option("ITL1", value, line_num)?);
            }
            (_, "ITL2") => {
                let value = expect_value(stream, line_num, params)?;
                options.itl2 = Some(parse_usize_option("ITL2", value, line_num)?);
            }
            (_, "ITL4") => {
                let value = expect_value(stream, line_num, params)?;
                options.itl4 = Some(parse_usize_option("ITL4", value, line_num)?);
            }
            (_, "ITL6") => {
                let value = expect_value(stream, line_num, params)?;
                options.itl6 = Some(parse_usize_option("ITL6", value, line_num)?);
            }
            (_, "METHOD") => {
                options.method = Some(parse_method_option(stream, line_num, params)?);
            }
            // The continuation ladder's rungs are switched one at a time
            // rather than through `NONLIN CONTINUATION`, which selects a
            // single algorithm to run in place of the ladder.
            (_, "GMINSTEPPING" | "GMIN_STEPPING") => {
                options.gmin_stepping =
                    Some(parse_boolean_option(stream, line_num, params, has_equals)?);
            }
            (_, "SOURCESTEPPING" | "SOURCE_STEPPING" | "SRCSTEPPING") => {
                options.source_stepping =
                    Some(parse_boolean_option(stream, line_num, params, has_equals)?);
            }
            (_, "PSEUDOTRANSIENT" | "PSEUDO_TRANSIENT") => {
                options.pseudo_transient =
                    Some(parse_boolean_option(stream, line_num, params, has_equals)?);
            }
            (_, "ARCLENGTH" | "ARC_LENGTH") => {
                options.arc_length =
                    Some(parse_boolean_option(stream, line_num, params, has_equals)?);
            }
            (_, "DAMPING") => {
                options.damping_strategy = Some(parse_damping_option(stream, line_num, params)?);
            }
            (_, "SOLVER") => {
                options.matrix_solver = Some(parse_matrix_solver_option(stream, line_num)?);
            }
            (Some("OUTPUT"), "INITIAL_INTERVAL" | "INITIALINTERVAL") => {
                if !options.output_time_points.is_empty() {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: "Cannot specify both .OPTIONS OUTPUT INITIAL_INTERVAL and OUTPUTTIMEPOINTS".to_string(),
                    });
                }
                let value = expect_value(stream, line_num, params)?;
                let _ = parse_positive_real_option("OUTPUT.INITIAL_INTERVAL", value, line_num)?;
                consume_output_initial_interval_schedule(stream, line_num, params)?;
                *output_initial_interval_seen = true;
            }
            (Some("OUTPUT"), "OUTPUTTIMEPOINTS") => {
                if *output_initial_interval_seen {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: "Cannot specify both .OPTIONS OUTPUT INITIAL_INTERVAL and OUTPUTTIMEPOINTS".to_string(),
                    });
                }
                let values = parse_time_point_vector_option(
                    stream,
                    line_num,
                    params,
                    "OUTPUT.OUTPUTTIMEPOINTS",
                    options
                        .output_time_points
                        .len()
                        .saturating_add(options.timeint_breakpoints.len())
                        .saturating_add(restart_interval_count(options)),
                    max_analysis_points,
                )?;
                append_canonical_time_points(
                    &mut options.output_time_points,
                    values,
                    &options.timeint_breakpoints,
                    max_analysis_points,
                )?;
            }
            (Some("OUTPUT"), "SNAPSHOTS") => {
                options.output_snapshots =
                    Some(parse_boolean_option(stream, line_num, params, has_equals)?);
            }
            (Some("OUTPUT"), "PRINTHEADER") => {
                options.output_print_header =
                    Some(parse_boolean_option(stream, line_num, params, has_equals)?);
            }
            (Some("OUTPUT"), "PRINTFOOTER") => {
                options.output_print_footer =
                    Some(parse_boolean_option(stream, line_num, params, has_equals)?);
            }
            (_, "INTERP" | "NOACCT") => {
                // Ngspice compatibility flags. INTERP affects rawfile storage
                // density and NOACCT suppresses accounting output; neither
                // changes the solved circuit state in RSpice today.
                if has_equals {
                    let _ = expect_value(stream, line_num, params)?;
                }
            }
            (_, "ALLOW_SIMPLIFIED_MOS" | "ALLOWSIMPLIFIEDMOS") => {
                // Bare flag enables; an explicit value of 0 disables.
                let enabled = if has_equals {
                    expect_value(stream, line_num, params)? != 0.0
                } else {
                    true
                };
                options.allow_simplified_mos = Some(enabled);
            }
            _ => {
                let warning_key = scoped_key.as_deref().unwrap_or(&key_upper);
                ignore_unknown_option(
                    stream,
                    line_num,
                    params,
                    has_equals,
                    warning_key,
                    unknown_warned,
                    diagnostics,
                );
            }
        }
    }

    Ok(())
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
    if has_equals
        && try_value(stream, params).is_none()
        && matches!(stream.peek().kind, TokenKind::Ident(_))
    {
        stream.advance();
    }
}

fn expect_option_key(
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

pub(super) fn option_package_key_is_known(key_upper: &str) -> bool {
    matches!(
        key_upper,
        "TOPOLOGY"
            | "MEASURE"
            | "DEVICE"
            | "XSPICE"
            | "TIMEINT"
            | "NONLIN"
            | "NONLIN-TRAN"
            | "NONLIN-TRANSIENT"
            | "NONLIN-CONTINUATION"
            | "LOCA"
            | "OUTPUT"
            | "RESTART"
            | "HBINT"
    )
}

fn restart_interval_schedule_starts(stream: &TokenStream, params: &ParamContext) -> bool {
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

    // Digit-leading SPICE values such as `10n` are intentionally lexed as
    // identifiers because the same spelling is legal as a node or model name.
    // Probe with the value parser instead of relying on the token variant.
    let mut probe = stream.clone();
    try_value(&mut probe, params).is_some()
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

fn consume_output_initial_interval_schedule(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<(), ParseError> {
    loop {
        skip_commas(stream);
        if !matches!(
            stream.peek().kind,
            TokenKind::Number(_) | TokenKind::Expression(_)
        ) {
            return Ok(());
        }
        let value = expect_value(stream, line_num, params)?;
        let _ = parse_positive_real_option("OUTPUT.INITIAL_INTERVAL", value, line_num)?;
    }
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

pub(super) fn parse_seed_option(value: Value, line_num: usize) -> Result<u64, ParseError> {
    let rounded = value.round();
    if !value.is_finite()
        || value < 0.0
        || (value - rounded).abs() > 1e-9
        || rounded > u64::MAX as Value
    {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("SEED must be a non-negative integer, found {}", value),
        });
    }
    Ok(rounded as u64)
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

pub(super) fn parse_meas_signal(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<String, ParseError> {
    if let Some(expression) = parse_measure_expression_operand(stream, line_num)? {
        return Ok(expression);
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
    let (goal, tolerance, default_value, print_policy, minval) =
        scan_meas_statement_options(stream, line_num, params)?;

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
                minval,
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
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!(
                            ".MEAS PARAM expects a quoted or braced expression, found {:?}",
                            other
                        ),
                    });
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
                file: options.file,
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
                    let (from, to) = parse_measure_range_options(stream, line_num, params)?;
                    MeasureType::Avg {
                        signal: signal.clone(),
                        from,
                        to,
                    }
                }
                "MAX" | "MAX_AT" => {
                    let (from, to, mut output) =
                        parse_measure_extrema_options(stream, line_num, params)?;
                    if measure_type_key == "MAX_AT" {
                        output = crate::netlist::measure::ExtremaOutput::IndependentAxis;
                    }
                    MeasureType::Max {
                        signal: signal.clone(),
                        from,
                        to,
                        output,
                    }
                }
                "MIN" | "MIN_AT" => {
                    let (from, to, mut output) =
                        parse_measure_extrema_options(stream, line_num, params)?;
                    if measure_type_key == "MIN_AT" {
                        output = crate::netlist::measure::ExtremaOutput::IndependentAxis;
                    }
                    MeasureType::Min {
                        signal: signal.clone(),
                        from,
                        to,
                        output,
                    }
                }
                "PP" => {
                    let (from, to) = parse_measure_range_options(stream, line_num, params)?;
                    MeasureType::PeakToPeak {
                        signal: signal.clone(),
                        from,
                        to,
                    }
                }
                "RMS" => {
                    let (from, to) = parse_measure_range_options(stream, line_num, params)?;
                    MeasureType::Rms {
                        signal: signal.clone(),
                        from,
                        to,
                    }
                }
                "INTEG" | "INTEGRAL" => {
                    let (from, to) = parse_measure_range_options(stream, line_num, params)?;
                    MeasureType::Integ {
                        signal: signal.clone(),
                        from,
                        to,
                    }
                }
                "FIND" => {
                    let options = parse_point_measure_options(stream, line_num, params, "FIND")?;

                    MeasureType::Find {
                        signal: signal.clone(),
                        at: options.at,
                        when: options.when,
                        from: options.from,
                        to: options.to,
                        td: options.td,
                        minval,
                    }
                }
                "DERIV" | "DERIVATIVE" => {
                    let options = parse_point_measure_options(stream, line_num, params, "DERIV")?;

                    MeasureType::Derivative {
                        signal: signal.clone(),
                        at: options.at,
                        when: options.when,
                        from: options.from,
                        to: options.to,
                        td: options.td,
                        minval,
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
                        minval,
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
        goal,
        tolerance,
        default_value,
        print_policy,
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

fn expect_finite_measure_td(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<Value, ParseError> {
    let value = expect_value(stream, line_num, params)?;
    if !value.is_finite() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(".MEAS TD must be finite, found {value}"),
        });
    }
    Ok(value)
}

fn parse_point_measure_options(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    measure_type: &str,
) -> Result<PointMeasureOptions, ParseError> {
    use crate::netlist::measure::WhenCondition;

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
                let value = if keyword == "TD" {
                    expect_finite_measure_td(stream, line_num, params)?
                } else {
                    expect_value(stream, line_num, params)?
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
        TokenKind::Ident(name) if params.get(name).is_none() => Ok(MeasureOperand::Waveform(
            parse_meas_signal(stream, line_num, params)?,
        )),
        _ => Ok(MeasureOperand::Constant(expect_value(
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
        let value = expect_value(stream, line_num, params)?;
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
    if let TokenKind::StringLit(path) = &stream.peek().kind {
        let path = path.clone();
        stream.advance();
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

fn parse_measure_when_event_options(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<
    (
        Option<crate::Value>,
        Option<crate::Value>,
        Option<crate::Value>,
        crate::netlist::measure::EventOccurrence,
    ),
    ParseError,
> {
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
            "FROM" => from = Some(expect_value(stream, line_num, params)?),
            "TO" => to = Some(expect_value(stream, line_num, params)?),
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
fn scan_meas_statement_options(
    stream: &TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<
    (
        Option<Value>,
        Option<Value>,
        Option<Value>,
        crate::netlist::measure::MeasurePrintPolicy,
        Value,
    ),
    ParseError,
> {
    let mut stream = stream.clone();
    let mut goal = None;
    let mut tolerance = None;
    let mut default_value = None;
    let mut print_policy = crate::netlist::measure::MeasurePrintPolicy::All;
    let mut minval = crate::netlist::measure::XYCE_DEFAULT_MEASURE_MINVAL;
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        let base_assignment_ahead = matches!(stream.peek().kind, TokenKind::Ident(_))
            && matches!(stream.peek_n(1).kind, TokenKind::Equals);
        if base_assignment_ahead {
            if let Some(qualifier) =
                consume_meas_statement_qualifier(&mut stream, line_num, params, true)?
            {
                match qualifier {
                    ParsedMeasStatementQualifier::Numeric { key, value } => match key.as_str() {
                        "GOAL" => goal = Some(value),
                        "TOL" => tolerance = Some(value),
                        "DEFAULT_VAL" => default_value = Some(value),
                        "MINVAL" => minval = value,
                        _ => unreachable!(),
                    },
                    ParsedMeasStatementQualifier::Print(policy) => print_policy = policy,
                }
                continue;
            }
        }
        stream.advance();
    }
    Ok((goal, tolerance, default_value, print_policy, minval))
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
        "GOAL" | "TOL" | "DEFAULT_VAL" | "PRINT" | "MINVAL"
    ) || key == "MINVAL" && !allow_minval
        || key == "TOL" && params.expression_dialect() == crate::config::ExpressionDialect::Xyce
    {
        return Ok(None);
    }
    stream.advance();
    if !stream.consume(&TokenKind::Equals) {
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
        TriggerEvent::At(expect_value(stream, line_num, params)?)
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
                let value = expect_value(stream, line_num, params)?;
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

pub(super) fn parse_measure_range_options(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<(Option<crate::Value>, Option<crate::Value>), ParseError> {
    let mut from = None;
    let mut to = None;

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
                if s.eq_ignore_ascii_case("FROM") || s.eq_ignore_ascii_case("TO") =>
            {
                let key = s.to_ascii_uppercase();
                stream.advance();
                if !stream.consume(&TokenKind::Equals) {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!("Expected '=' after {} in .MEAS", key),
                    });
                }
                let value = expect_value(stream, line_num, params)?;
                match key.as_str() {
                    "FROM" => from = Some(value),
                    "TO" => to = Some(value),
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

    Ok((from, to))
}

fn parse_measure_extrema_options(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<
    (
        Option<crate::Value>,
        Option<crate::Value>,
        crate::netlist::measure::ExtremaOutput,
    ),
    ParseError,
> {
    use crate::netlist::measure::ExtremaOutput;

    let mut from = None;
    let mut to = None;
    let mut output = ExtremaOutput::Value;
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
        if !matches!(key.as_str(), "FROM" | "TO" | "OUTPUT") {
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
        } else {
            let value = expect_value(stream, line_num, params)?;
            if key == "FROM" {
                from = Some(value);
            } else {
                to = Some(value);
            }
        }
    }
    Ok((from, to, output))
}

fn parse_measure_equation_options(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<
    (
        Option<crate::Value>,
        Option<crate::Value>,
        Option<crate::Value>,
    ),
    ParseError,
> {
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
        let value = expect_value(stream, line_num, params)?;
        match key.as_str() {
            "FROM" => from = Some(value),
            "TO" => to = Some(value),
            "TD" => td = Some(value),
            _ => unreachable!(),
        }
    }

    Ok((from, to, td))
}

pub(super) fn parse_param_statement(
    stream: &mut TokenStream,
    line_num: usize,
    params: &mut ParamContext,
    mut deferred_params: Option<&mut Vec<(String, String)>>,
    retain_global_expression: bool,
    diagnostics: &mut Vec<ParseDiagnostic>,
    origin: &NetlistSourceLocation,
) -> Result<(), ParseError> {
    if retain_global_expression && deferred_params.is_some() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: ".GLOBAL_PARAM is only valid in the top-level netlist scope".to_string(),
        });
    }
    // Parse one or more NAME=VALUE pairs
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline) {
        // Skip commas
        while stream.consume(&TokenKind::Comma) {}

        if stream.is_eof() || matches!(stream.peek().kind, TokenKind::Newline) {
            break;
        }

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
            });
        }

        if matches!(stream.peek().kind, TokenKind::LParen) {
            let acceptance = params.accepts_parameter_function_definition(&name, origin);
            let error_policy = params.parameter_redefinition_diagnostic_policy()
                == ParameterRedefinitionDiagnosticPolicy::Error;
            if acceptance.authoritative && (acceptance.first_origin.is_none() || !error_policy) {
                parse_param_function_definition(stream, line_num, params, name.clone())?;
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

        // Expect = sign
        if !stream.consume(&TokenKind::Equals) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Expected '=' after parameter name '{}'", name),
            });
        }

        let acceptance =
            params.accepts_parameter_definition(&name, retain_global_expression, origin);
        let error_policy = params.parameter_redefinition_diagnostic_policy()
            == ParameterRedefinitionDiagnosticPolicy::Error;
        if acceptance.authoritative && (acceptance.first_origin.is_none() || !error_policy) {
            parse_param_assignment_value(
                stream,
                line_num,
                params,
                deferred_params.as_deref_mut(),
                retain_global_expression,
                name.clone(),
            )?;
        } else {
            // The ignored definition is still tokenized and syntax-checked,
            // but it cannot mutate the authoritative context or its deferred
            // expression list. A private deferred sink lets an unused local
            // duplicate reference an otherwise-unresolved name without
            // changing the retained first definition.
            let mut ignored_params = params.isolated_random_clone();
            let mut ignored_deferred = Vec::new();
            let ignored_deferred = (!retain_global_expression).then_some(&mut ignored_deferred);
            parse_param_assignment_value(
                stream,
                line_num,
                &mut ignored_params,
                ignored_deferred,
                retain_global_expression,
                name.clone(),
            )?;
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

fn parse_param_assignment_value(
    stream: &mut TokenStream,
    line_num: usize,
    params: &mut ParamContext,
    mut deferred_params: Option<&mut Vec<(String, String)>>,
    retain_global_expression: bool,
    name: String,
) -> Result<(), ParseError> {
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
            reject_parameter_expression_circuit_probe(&name, &expr, line_num, params)?;
            if !retain_runtime_param_expression(
                params,
                deferred_params.as_deref_mut(),
                retain_global_expression,
                &name,
                &expr,
            ) {
                match eval_expression_complex(&expr, params) {
                    Ok(value) => {
                        if retain_global_expression {
                            params.define_global_expression(&name, &expr, Some(value));
                        } else {
                            params.set_complex(&name, value);
                        }
                        upsert_deferred_param_expression(
                            deferred_params.as_deref_mut(),
                            &name,
                            &expr,
                        );
                    }
                    Err(err) => {
                        let err = ParseError::InvalidValue(format!("line {}: {}", line_num, err));
                        defer_param_expression_or_error(
                            deferred_params.as_deref_mut(),
                            params,
                            retain_global_expression,
                            name,
                            expr,
                            err,
                        )?;
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
            reject_parameter_expression_circuit_probe(&name, &expr, line_num, params)?;
            if !retain_runtime_param_expression(
                params,
                deferred_params.as_deref_mut(),
                retain_global_expression,
                &name,
                &expr,
            ) {
                match eval_expression_complex(&expr, params) {
                    Ok(value) => {
                        if retain_global_expression {
                            params.define_global_expression(&name, &expr, Some(value));
                        } else {
                            params.set_complex(&name, value);
                        }
                        upsert_deferred_param_expression(
                            deferred_params.as_deref_mut(),
                            &name,
                            &expr,
                        );
                    }
                    Err(err) => {
                        let err = ParseError::InvalidValue(format!("line {}: {}", line_num, err));
                        defer_param_expression_or_error(
                            deferred_params.as_deref_mut(),
                            params,
                            retain_global_expression,
                            name,
                            expr,
                            err,
                        )?;
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
            );
        }
        TokenKind::Ident(param_name) if params.get_complex(param_name).is_some() => {
            let value = params
                .get_complex(param_name)
                .expect("parameter presence checked");
            let expr = param_name.clone();
            stream.advance();
            if retain_global_expression {
                params.define_global_expression(&name, &expr, Some(value));
            } else {
                params.set_complex(&name, value);
            }
            upsert_deferred_param_expression(deferred_params.as_deref_mut(), &name, &expr);
        }
        _ => {
            let deferred_expr = simple_param_value_expression(stream, params);
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
                    reject_parameter_expression_circuit_probe(&name, &expr, line_num, params)?;
                    defer_param_expression_or_error(
                        deferred_params,
                        params,
                        retain_global_expression,
                        name,
                        expr,
                        err,
                    )?;
                }
            }
        }
    }
    Ok(())
}

fn reject_parameter_expression_circuit_probe(
    name: &str,
    expression: &str,
    line_num: usize,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let prepared = crate::netlist::expr::prepare_behavioral_expression(expression, params)
        .unwrap_or_else(|_| expression.to_string());
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
    })
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
    err: ParseError,
) -> Result<(), ParseError> {
    if parameter_error_can_defer(&err) {
        if retain_global_expression {
            params.define_global_expression(&name, &expr, None);
        } else if params.expression_dialect() == ExpressionDialect::Xyce {
            params.define_parameter_expression(&name, &expr, None);
        }
        if let Some(deferred_params) = deferred_params {
            upsert_param_expression(deferred_params, name, expr);
            return Ok(());
        }
        if retain_global_expression || params.expression_dialect() == ExpressionDialect::Xyce {
            return Ok(());
        }
    }
    Err(err)
}

fn retain_runtime_param_expression(
    params: &mut ParamContext,
    deferred_params: Option<&mut Vec<(String, String)>>,
    global: bool,
    name: &str,
    expression: &str,
) -> bool {
    if params.expression_dialect() != ExpressionDialect::Xyce {
        return false;
    }
    let prepared = crate::netlist::expr::prepare_behavioral_expression(expression, params)
        .unwrap_or_else(|_| expression.to_string());
    if !crate::netlist::expr::behavioral_expression_references_runtime_quantity(&prepared) {
        return false;
    }
    if global {
        params.define_global_expression(name, expression, None);
    } else {
        params.define_parameter_expression(name, expression, None);
    }
    upsert_deferred_param_expression(deferred_params, name, expression);
    true
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
) -> Result<(), ParseError> {
    let func_name = expect_function_name(stream, line_num)?;

    let args = parse_function_argument_list(stream, line_num, &func_name)?;

    // Check for = sign or Expression (which is {expression})
    let body: String;

    if stream.consume(&TokenKind::Equals) {
        // Standard syntax: .FUNC name(args) = expression
        // Collect the rest of the line as the expression body
        let mut body_parts = Vec::new();
        while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof)
        {
            match &stream.peek().kind {
                TokenKind::Ident(s) => body_parts.push(s.clone()),
                TokenKind::Number(n) => body_parts.push(format!("{}", n)),
                TokenKind::Expression(e) => body_parts.push(e.clone()),
                TokenKind::LParen => body_parts.push("(".to_string()),
                TokenKind::RParen => body_parts.push(")".to_string()),
                TokenKind::Comma => body_parts.push(",".to_string()),
                TokenKind::Plus => body_parts.push("+".to_string()),
                TokenKind::Minus => body_parts.push("-".to_string()),
                TokenKind::Star => body_parts.push("*".to_string()),
                TokenKind::Slash => body_parts.push("/".to_string()),
                TokenKind::Equals => body_parts.push("=".to_string()),
                _ => {}
            }
            stream.advance();
        }
        body = body_parts.join("");
    } else if let TokenKind::Expression(expr) = &stream.peek().kind {
        // Function definition: .FUNC name(args) {expression}
        body = expr.clone();
        stream.advance();
    } else {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Expected '=' or '{{expression}}' after .FUNC {}(...)",
                func_name
            ),
        });
    }

    if body.is_empty() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(".FUNC {} requires an expression body", func_name),
        });
    }

    // Register the function
    params.define_function(&func_name, args, &body);
    log::debug!("Defined function: {}(...) = {}", func_name, body);

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

#[cfg(test)]
mod tests {
    use crate::{
        Netlist,
        netlist::{AnalysisCommand, DcSweepMode, ParseError, PrintDelimiter, SaveSignal},
        numerics::integration::TransientErrorControl,
    };

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
                SaveSignal::Current("v1".to_string()),
                SaveSignal::Voltage("out".to_string()),
                SaveSignal::Voltage("out".to_string()),
                SaveSignal::Voltage("out".to_string()),
            ]
        );
        assert!(netlist.diagnostics.is_empty());
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
                },
            ] = netlist.analyses.as_slice()
            else {
                panic!("{statement} lost its sole DC analysis")
            };
            assert_eq!(source, "V1");
            let spec = crate::netlist::DcSweepSpec {
                start: *start,
                stop: *stop,
                step: *step,
                mode: mode.clone(),
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

        let [crate::netlist::AnalysisCommand::Hb { frequencies }] = netlist.analyses.as_slice()
        else {
            panic!("expected one HB analysis")
        };
        assert_eq!(frequencies, &[10.0e3, 20.0e3]);
        assert_eq!(netlist.options.hb_num_frequencies, vec![50, 25]);
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
        Netlist::parse(&deck_with_options(
            ".options output initial_interval=.001ms .5ms .01ms",
        ))
        .expect("Xyce OUTPUT INITIAL_INTERVAL schedule syntax parses");
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
                resource_limits: crate::resource::ResourceLimits {
                    max_analysis_points: 1,
                    ..crate::resource::ResourceLimits::default()
                },
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
                    resource_limits: crate::resource::ResourceLimits {
                        max_analysis_points: 1,
                        ..crate::resource::ResourceLimits::default()
                    },
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
                resource_limits: crate::resource::ResourceLimits {
                    max_analysis_points: 3,
                    ..crate::resource::ResourceLimits::default()
                },
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
        let crate::netlist::measure::MeasureType::Avg { signal, from, to } =
            &statement.measure_type
        else {
            panic!("expected AVG measurement");
        };
        assert_eq!(signal, "V(DEFAULT_VAL)");
        assert_eq!((*from, *to), (Some(0.25), Some(0.75)));
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
        assert_eq!(file, "fixtures/data.csv");

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
        let netlist = Netlist::parse(&deck_with_options(".options device minres=1 mincap=1nf"))
            .expect("scoped Xyce MINRES/MINCAP parse");
        assert_eq!(netlist.options.device_min_resistance, Some(1.0));
        assert_eq!(netlist.options.device_min_capacitance, Some(1.0e-9));

        let mut merged = crate::netlist::SimulationOptions {
            device_min_resistance: Some(2.0),
            ..crate::netlist::SimulationOptions::default()
        };
        merged.merge(&netlist.options);
        assert_eq!(merged.device_min_resistance, Some(1.0));
        assert_eq!(merged.device_min_capacitance, Some(1.0e-9));
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
    fn incomplete_tran_uses_the_xyce_ordered_diagnostics() {
        let error = Netlist::parse(
            "incomplete transient analysis\n\
             R1 1 0 1\n\
             .tran 1u\n\
             .end\n",
        )
        .expect_err("a transient analysis without a stop time must fail");
        assert!(matches!(
            error,
            ParseError::Syntax { line: 3, ref message }
                if message == ".TRAN line has an unexpected number of fields\nUnrecognized dot line will be ignored"
        ));

        Netlist::parse(
            "complete transient analysis\n\
             R1 1 0 1\n\
             .tran 1u 1m\n\
             .end\n",
        )
        .expect("supplying the stop time repairs the analysis");
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
}
