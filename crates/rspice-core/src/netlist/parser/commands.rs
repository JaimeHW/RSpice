//! Dot-command parsing for analyses, options, measurements, params, and functions.

use crate::netlist::{TransientLteReference, XspiceAutoBridgeParamName, XspiceAutoBridgeTemplate};

use super::*;

pub(super) fn parse_command(
    stream: &mut TokenStream,
    line_num: usize,
    context: ParseCommandContext<'_>,
) -> Result<(), ParseError> {
    let ParseCommandContext {
        analyses,
        fft_analyses,
        unknown_warned,
        models,
        params,
        initial_conditions,
        node_sets,
        global_nodes,
        measurements,
        saves,
        options,
        diagnostics,
        spef_includes,
        defer_scoped_values,
        deferred_body_params,
    } = context;

    let cmd = expect_ident(stream, line_num)?;
    let mut require_line_consumed = true;

    match cmd.as_str() {
        ".OP" => {
            analyses.push(AnalysisCommand::Op);
        }
        ".DC" => {
            let (source, spec) = parse_dc_sweep_spec(stream, line_num, params)?;

            // Optional second (outer) source: .DC V1 a b s V2 a2 b2 s2
            skip_commas(stream);
            let sweep2 = if matches!(stream.peek().kind, TokenKind::Ident(_)) {
                let (source2, spec2) = parse_dc_sweep_spec(stream, line_num, params)?;
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
        ".TRAN" => {
            let step = expect_value(stream, line_num, params)?;
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
            let model = parse_model_definition(stream, line_num, params, models, false)?;
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
        ".PARAM" | ".CSPARAM" => {
            parse_param_statement(stream, line_num, params, deferred_body_params, false)?;
        }
        ".GLOBAL_PARAM" => {
            parse_param_statement(stream, line_num, params, deferred_body_params, true)?;
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
            let temperatures = parse_temp_command(stream, params)?;
            analyses.push(AnalysisCommand::Temp { temperatures });
        }
        ".FOUR" | ".FOURIER" => {
            let (fundamental, outputs) = parse_four_command(stream, line_num, params)?;
            analyses.push(AnalysisCommand::Four {
                fundamental,
                outputs,
                num_harmonics: 9, // Default
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
            parse_ic_command(
                stream,
                line_num,
                params,
                initial_conditions,
                defer_scoped_values,
            )?;
        }
        ".NODESET" => {
            parse_nodeset_command(stream, line_num, params, node_sets, defer_scoped_values)?;
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
        ".OPTIONS" | ".OPTION" | ".OPT" => parse_options_command(
            stream,
            line_num,
            params,
            options,
            unknown_warned,
            diagnostics,
        )?,
        ".MEAS" | ".MEASURE" => {
            // Parse measurement statement: .MEAS TRAN name TYPE signal [options]
            measurements.push(parse_meas_command(stream, line_num, params)?);
        }
        ".SAVE" | ".PROBE" => {
            parse_save_command(stream, line_num, saves, false)?;
        }
        ".PRINT" | ".PLOT" => {
            // .PRINT/.PLOT take an optional leading analysis type before the
            // probe list; the probes feed the same output-selection set.
            parse_save_command(stream, line_num, saves, true)?;
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

fn parse_xspice_codemodel_command(
    stream: &mut TokenStream,
    line_num: usize,
    command: &str,
) -> Result<(), ParseError> {
    let requested = collect_codemodel_library_args(stream);
    let unsupported = requested
        .iter()
        .filter(|path| !crate::xspice::CodeModelRegistry::is_builtin_codemodel_library_path(path))
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
            crate::xspice::CodeModelRegistry::builtin_codemodel_library_names().join(", ")
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
/// is consumed and ignored, matching `.PRINT TRAN v(out)` usage.
pub(super) fn parse_save_command(
    stream: &mut TokenStream,
    line_num: usize,
    saves: &mut super::SaveSet,
    skip_analysis_type: bool,
) -> Result<(), ParseError> {
    use super::SaveSignal;

    let mut first_token = true;
    let mut parsed_any = false;

    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        skip_commas(stream);
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }

        match &stream.peek().kind {
            TokenKind::Ident(raw) => {
                let raw = raw.clone();
                stream.advance();
                let upper = raw.to_ascii_uppercase();

                if first_token
                    && skip_analysis_type
                    && matches!(
                        upper.as_str(),
                        "TRAN" | "AC" | "DC" | "NOISE" | "DISTO" | "OP" | "TF" | "SP" | "PSS"
                    )
                {
                    first_token = false;
                    continue;
                }
                first_token = false;

                if upper == "ALL" {
                    saves.signals.push(SaveSignal::All);
                    parsed_any = true;
                    continue;
                }

                // `v(...)` / `i(...)` may arrive either as one identifier or
                // as an identifier followed by a parenthesized token run.
                let is_probe_prefix = upper == "V" || upper == "I" || upper == "N";
                if is_probe_prefix && matches!(stream.peek().kind, TokenKind::LParen) {
                    let mut probe = raw.clone();
                    probe.push('(');
                    stream.advance(); // consume '('
                    let mut depth = 1usize;
                    while depth > 0
                        && !stream.is_eof()
                        && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof)
                    {
                        match &stream.peek().kind {
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
                            TokenKind::Ident(s) => probe.push_str(s),
                            TokenKind::Number(n) => probe.push_str(&format!("{}", n)),
                            TokenKind::Comma => probe.push(','),
                            // Hierarchy wildcard: `v(x1.*)` must keep its star.
                            TokenKind::Star => probe.push('*'),
                            _ => {}
                        }
                        stream.advance();
                    }
                    probe.push(')');
                    if let Some(signal) = parse_save_probe(&probe) {
                        saves.signals.push(signal);
                        parsed_any = true;
                    }
                    continue;
                }

                if let Some(signal) = parse_save_probe(&raw) {
                    saves.signals.push(signal);
                    parsed_any = true;
                }
            }
            TokenKind::AtSign => {
                stream.advance();
                first_token = false;
                // @dev[param]: device then bracketed parameter name.
                let device = match &stream.peek().kind {
                    TokenKind::Ident(s) => {
                        let device = s.clone();
                        stream.advance();
                        device
                    }
                    _ => {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: "Expected device name after '@' in save directive".to_string(),
                        });
                    }
                };
                if stream.consume(&TokenKind::LBracket) {
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
                    stream.consume(&TokenKind::RBracket);
                    saves
                        .signals
                        .push(SaveSignal::DeviceParam { device, param });
                } else {
                    saves.signals.push(SaveSignal::Raw(device));
                }
                parsed_any = true;
            }
            TokenKind::Number(_) => {
                // Numeric node names (e.g. `.save 2`) select v(2).
                if let TokenKind::Number(n) = &stream.peek().kind {
                    let name = if n.fract() == 0.0 {
                        format!("{}", *n as i64)
                    } else {
                        format!("{}", n)
                    };
                    saves.signals.push(SaveSignal::Raw(name));
                    parsed_any = true;
                }
                stream.advance();
                first_token = false;
            }
            _ => {
                stream.advance();
            }
        }
    }

    if !parsed_any {
        // ngspice warns and ignores a bare .print/.save (several corpus
        // decks carry one); a hard error would reject the whole deck.
        log::warn!("line {line_num}: save/print directive without output signals ignored");
    }

    Ok(())
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
    unknown_warned: &mut std::collections::HashSet<String>,
    diagnostics: &mut Vec<ParseDiagnostic>,
) -> Result<(), ParseError> {
    let mut option_package: Option<String> = None;

    while !stream.is_eof() {
        skip_commas(stream);
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }

        let key = expect_option_key(stream, line_num)?;
        let key_upper = key.to_uppercase();
        let has_equals = stream.consume(&TokenKind::Equals);

        if !has_equals && option_package_key_is_known(&key_upper) {
            option_package = Some(key_upper);
            continue;
        }

        let scoped_key = option_package
            .as_deref()
            .map(|package| format!("{package}.{key_upper}"));

        match (option_package.as_deref(), key_upper.as_str()) {
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
            (Some("DEVICE"), "B3SOIGMINSCALING" | "B3SOI_GMIN_SCALING") => {
                options.b3soi_gmin_scaling =
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
            (None, "B3SOIGMINSCALING" | "B3SOI_GMIN_SCALING" | "DEVICE_B3SOIGMINSCALING") => {
                options.b3soi_gmin_scaling =
                    Some(parse_boolean_option(stream, line_num, params, has_equals)?);
            }
            (Some("XSPICE"), "AUTO_BRIDGE" | "AUTOBRIDGE")
            | (None, "AUTO_BRIDGE" | "AUTOBRIDGE" | "XSPICE_AUTO_BRIDGE") => {
                let (enabled, show_generated) =
                    parse_auto_bridge_option(stream, line_num, params, has_equals)?;
                options.auto_bridge = Some(enabled);
                options.auto_bridge_show_generated = Some(show_generated);
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
            (_, "TEMP") => {
                let value = expect_value(stream, line_num, params)?;
                options.temp = Some(parse_celsius_option("TEMP", value, line_num)?);
            }
            (_, "TNOM") => {
                let value = expect_value(stream, line_num, params)?;
                options.tnom = Some(parse_celsius_option("TNOM", value, line_num)?);
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
            (Some("OUTPUT"), "INITIAL_INTERVAL" | "INITIALINTERVAL") => {
                let value = expect_value(stream, line_num, params)?;
                let _ = parse_positive_real_option("OUTPUT.INITIAL_INTERVAL", value, line_num)?;
                consume_output_initial_interval_schedule(stream, line_num, params)?;
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

fn expect_option_key(stream: &mut TokenStream, line_num: usize) -> Result<String, ParseError> {
    skip_commas(stream);

    let TokenKind::Ident(first) = &stream.peek().kind else {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("Expected identifier, found {:?}", stream.peek().kind),
        });
    };
    let mut key = first.clone();
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
        stream.advance();
    }

    Ok(key)
}

fn option_package_key_is_known(key_upper: &str) -> bool {
    matches!(
        key_upper,
        "TOPOLOGY"
            | "DEVICE"
            | "XSPICE"
            | "TIMEINT"
            | "NONLIN"
            | "NONLIN-TRAN"
            | "NONLIN-TRANSIENT"
            | "NONLIN-CONTINUATION"
            | "LOCA"
            | "OUTPUT"
            | "HBINT"
    )
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

fn parse_nonlinear_continuation_option(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<crate::netlist::NonlinearContinuationMode, ParseError> {
    use crate::netlist::NonlinearContinuationMode as Mode;

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
            let probe = parse_meas_signal(stream, line_num)?;
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
    let allowed = operator.starts_with('I') || matches!(operator.as_str(), "V" | "P" | "W" | "N");
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
) -> Result<String, ParseError> {
    if let TokenKind::Expression(expression) = &stream.peek().kind {
        let expression = format!("{{{expression}}}");
        stream.advance();
        return Ok(expression);
    }
    let mut signal = expect_ident(stream, line_num)?;

    if stream.consume(&TokenKind::LParen) {
        let mut args = Vec::new();
        loop {
            let arg = match &stream.peek().kind {
                TokenKind::Ident(s) => s.clone(),
                TokenKind::Number(v) => format!("{}", v),
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

/// Parse .MEAS/.MEASURE statement
/// Syntax:
///   .MEAS TRAN name TYPE signal [FROM=x TO=y]
///   .MEAS TRAN name FIND signal AT=time
///   .MEAS TRAN name FIND signal WHEN ref_signal=value
///   .MEAS TRAN name TRIG signal VAL=x [RISE=n|FALL=n|CROSS=n] [TD=x]
///                     TARG signal VAL=x [RISE=n|FALL=n|CROSS=n] [TD=x]
/// Examples:
///   .MEAS TRAN vmax MAX V(out)
///   .MEAS TRAN vavg AVG V(out) FROM=0 TO=1m
///   .MEAS TRAN vout FIND V(out) AT=1u
///   .MEAS TRAN delay TRIG V(in) VAL=0.5 RISE=1 TARG V(out) VAL=0.5 RISE=1
pub(super) fn parse_meas_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<crate::analysis::MeasureStatement, ParseError> {
    use crate::analysis::{MeasureStatement, MeasureType};

    // Parse analysis type (TRAN, AC, DC)
    let analysis = expect_ident(stream, line_num)?;

    // Xyce treats the measurement name as one whitespace-delimited source
    // field.  It may therefore contain punctuation (for example
    // `CONSTANT-AT`) that the general lexer correctly emits as multiple
    // adjacent tokens.
    let name = parse_measure_name(stream, line_num)?;

    // Parse measurement type keyword
    let measure_type_str = expect_ident(stream, line_num)?;
    let measure_type_key = measure_type_str.to_ascii_uppercase();

    // Create the measurement type based on keyword
    let measure_type = match measure_type_key.as_str() {
        "TRIG" => {
            let trig = parse_meas_delay_spec(stream, line_num, params, "TRIG", true)?;
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
            let targ = parse_meas_delay_spec(stream, line_num, params, "TARG", false)?;
            MeasureType::Delay { trig, targ }
        }
        "PARAM" | "EQN" => {
            // .MEAS <an> name PARAM='expr' — an expression over previously
            // evaluated measurement results.
            let _optional_equals = stream.consume(&TokenKind::Equals);
            let expression = match &stream.peek().kind {
                TokenKind::Expression(expr) => {
                    let expr = expr.clone();
                    stream.advance();
                    expr
                }
                TokenKind::StringLit(expr) => {
                    let expr = expr.clone();
                    stream.advance();
                    expr
                }
                _ if params.expression_dialect() == crate::netlist::ExpressionDialect::Xyce => {
                    collect_measure_equation_expression(stream, line_num)?
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
                || params.expression_dialect() == crate::netlist::ExpressionDialect::Xyce
            {
                let (from, to, td, default_value) =
                    parse_measure_equation_options(stream, line_num, params)?;
                MeasureType::Equation {
                    expression,
                    from,
                    to,
                    td,
                    default_value,
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
                    crate::analysis::ErrorFunctionNorm::MeanAbsolute
                } else {
                    crate::analysis::ErrorFunctionNorm::RootMeanSquare
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
            let signal = parse_meas_signal(stream, line_num)?;
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
            let signal = parse_meas_signal(stream, line_num)?;

            match measure_type_key.as_str() {
                "AVG" => {
                    let (from, to) = parse_measure_range_options(stream, line_num, params)?;
                    MeasureType::Avg {
                        signal: signal.clone(),
                        from,
                        to,
                    }
                }
                "MAX" => {
                    let (from, to, output) =
                        parse_measure_extrema_options(stream, line_num, params)?;
                    MeasureType::Max {
                        signal: signal.clone(),
                        from,
                        to,
                        output,
                    }
                }
                "MIN" => {
                    let (from, to, output) =
                        parse_measure_extrema_options(stream, line_num, params)?;
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
                    let (from, to, occurrence) =
                        parse_measure_when_event_options(stream, line_num, params)?;
                    MeasureType::When {
                        condition: crate::analysis::WhenCondition {
                            left: signal.clone(),
                            right,
                            occurrence,
                        },
                        from,
                        to,
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

    let (goal, tolerance) = parse_meas_goal_options(stream, line_num, params)?;

    Ok(MeasureStatement {
        name,
        measure_type,
        analysis,
        goal,
        tolerance,
    })
}

struct PointMeasureOptions {
    at: Option<Value>,
    when: Option<crate::analysis::WhenCondition>,
    from: Option<Value>,
    to: Option<Value>,
    occurrence_given: bool,
}

fn parse_point_measure_options(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    measure_type: &str,
) -> Result<PointMeasureOptions, ParseError> {
    use crate::analysis::WhenCondition;

    let mut options = PointMeasureOptions {
        at: None,
        when: None,
        from: None,
        to: None,
        occurrence_given: false,
    };
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        let TokenKind::Ident(keyword) = &stream.peek().kind else {
            stream.advance();
            continue;
        };
        let keyword = keyword.to_ascii_uppercase();
        match keyword.as_str() {
            "AT" | "FROM" | "TO" => {
                stream.advance();
                if !stream.consume(&TokenKind::Equals) {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!("Expected '=' after {keyword} in .MEAS {measure_type}"),
                    });
                }
                let value = expect_value(stream, line_num, params)?;
                match keyword.as_str() {
                    "AT" => options.at = Some(value),
                    "FROM" => options.from = Some(value),
                    "TO" => options.to = Some(value),
                    _ => unreachable!(),
                }
            }
            "WHEN" => {
                stream.advance();
                let left = parse_meas_signal(stream, line_num)?;
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
                    occurrence: crate::analysis::EventOccurrence::default(),
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
                condition.occurrence = crate::analysis::EventOccurrence {
                    edge: match keyword.as_str() {
                        "RISE" => crate::analysis::EdgeType::Rise,
                        "FALL" => crate::analysis::EdgeType::Fall,
                        "CROSS" => crate::analysis::EdgeType::Cross,
                        _ => unreachable!(),
                    },
                    number: parse_measure_event_occurrence(stream, line_num, params, &keyword)?,
                };
                options.occurrence_given = true;
            }
            "GOAL" | "TOL" => break,
            // Preserve the legacy parser's tolerance of analysis-specific
            // qualifiers that are modeled by other measurement families.
            // Their tokens remain outside this DC point-event contract.
            _ => {
                stream.advance();
            }
        }
    }
    Ok(options)
}

fn parse_measure_when_operand(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<crate::analysis::MeasureOperand, ParseError> {
    use crate::analysis::MeasureOperand;

    match &stream.peek().kind {
        TokenKind::Expression(_) => Ok(MeasureOperand::Waveform(parse_meas_signal(
            stream, line_num,
        )?)),
        TokenKind::Ident(_) if matches!(stream.peek_n(1).kind, TokenKind::LParen) => Ok(
            MeasureOperand::Waveform(parse_meas_signal(stream, line_num)?),
        ),
        TokenKind::Ident(name) if params.get(name).is_none() => Ok(MeasureOperand::Waveform(
            parse_meas_signal(stream, line_num)?,
        )),
        _ => Ok(MeasureOperand::Constant(expect_value(
            stream, line_num, params,
        )?)),
    }
}

fn parse_measure_error_operand(
    stream: &mut TokenStream,
    line_num: usize,
    _params: &ParamContext,
) -> Result<String, ParseError> {
    if matches!(&stream.peek().kind, TokenKind::Ident(name) if name.eq_ignore_ascii_case("PAR"))
        && matches!(stream.peek_n(1).kind, TokenKind::LParen)
    {
        stream.advance();
        stream.advance();
        let expression = match &stream.peek().kind {
            TokenKind::StringLit(expression) | TokenKind::Expression(expression) => {
                let expression = expression.clone();
                stream.advance();
                expression
            }
            _ => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: "Expected quoted or braced expression inside PAR(...) in .MEAS ERR"
                        .to_string(),
                });
            }
        };
        if !stream.consume(&TokenKind::RParen) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: "Expected ')' after PAR expression in .MEAS ERR".to_string(),
            });
        }
        return Ok(format!("{{{expression}}}"));
    }
    parse_meas_signal(stream, line_num)
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
        minval: 1.0e-12,
        ymin: 1.0e-15,
        ymax: 1.0e15,
        weight: None,
    };
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        let TokenKind::Ident(keyword) = &stream.peek().kind else {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Unexpected token '{}' in .MEAS ERR", stream.peek().kind),
            });
        };
        let keyword = keyword.to_ascii_uppercase();
        if matches!(keyword.as_str(), "GOAL" | "TOL") {
            break;
        }
        if !matches!(
            keyword.as_str(),
            "FROM" | "TO" | "MINVAL" | "YMIN" | "YMAX" | "IGNOR" | "IGNORE" | "WEIGHT"
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
            "MINVAL" => options.minval = value,
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
    norm: crate::analysis::FileErrorNorm,
    independent_column: Option<isize>,
    dependent_column: usize,
}

fn parse_measure_file_error_options(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<FileErrorOptions, ParseError> {
    use crate::analysis::FileErrorNorm;

    let mut file = None;
    let mut norm = FileErrorNorm::L2;
    let mut independent_column = None;
    let mut dependent_column = None;
    let mut seen = std::collections::HashSet::new();

    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        let TokenKind::Ident(keyword) = &stream.peek().kind else {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Unexpected token '{}' in .MEAS ERROR", stream.peek().kind),
            });
        };
        let keyword = keyword.to_ascii_uppercase();
        if matches!(keyword.as_str(), "GOAL" | "TOL") {
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

    let mut path = String::new();
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        if measure_file_option_assignment_ahead(stream) {
            break;
        }
        match stream.peek().kind {
            TokenKind::Equals | TokenKind::StringLit(_) | TokenKind::Expression(_) => break,
            _ => {
                path.push_str(&stream.peek().lexeme);
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

fn measure_file_option_assignment_ahead(stream: &TokenStream) -> bool {
    let TokenKind::Ident(keyword) = &stream.peek().kind else {
        return false;
    };
    matches!(stream.peek_n(1).kind, TokenKind::Equals)
        && matches!(
            keyword.to_ascii_uppercase().as_str(),
            "FILE" | "COMP_FUNCTION" | "INDEPVARCOL" | "DEPVARCOL" | "GOAL" | "TOL"
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
        crate::analysis::EventOccurrence,
    ),
    ParseError,
> {
    let mut from = None;
    let mut to = None;
    let mut occurrence = crate::analysis::EventOccurrence::default();
    let mut occurrence_given = false;

    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        let TokenKind::Ident(keyword) = &stream.peek().kind else {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Unexpected token '{}' in .MEAS WHEN", stream.peek().kind),
            });
        };
        let keyword = keyword.to_ascii_uppercase();
        if matches!(keyword.as_str(), "GOAL" | "TOL") {
            break;
        }
        if !matches!(keyword.as_str(), "FROM" | "TO" | "RISE" | "FALL" | "CROSS") {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Unexpected option '{keyword}' in .MEAS WHEN"),
            });
        }
        stream.advance();
        if !stream.consume(&TokenKind::Equals) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Expected '=' after {keyword} in .MEAS WHEN"),
            });
        }
        match keyword.as_str() {
            "FROM" => from = Some(expect_value(stream, line_num, params)?),
            "TO" => to = Some(expect_value(stream, line_num, params)?),
            "RISE" | "FALL" | "CROSS" => {
                if occurrence_given {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: "Only one RISE, FALL, or CROSS qualifier is allowed in .MEAS WHEN"
                            .to_string(),
                    });
                }
                occurrence = crate::analysis::EventOccurrence {
                    edge: match keyword.as_str() {
                        "RISE" => crate::analysis::EdgeType::Rise,
                        "FALL" => crate::analysis::EdgeType::Fall,
                        "CROSS" => crate::analysis::EdgeType::Cross,
                        _ => unreachable!(),
                    },
                    number: parse_measure_event_occurrence(stream, line_num, params, &keyword)?,
                };
                occurrence_given = true;
            }
            _ => unreachable!(),
        }
    }

    Ok((from, to, occurrence))
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

fn collect_measure_equation_expression(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<String, ParseError> {
    let mut expression = String::new();
    let mut depth = 0usize;

    loop {
        let token = stream.peek().clone();
        match &token.kind {
            TokenKind::Newline | TokenKind::Eof => break,
            TokenKind::Ident(name)
                if depth == 0
                    && matches!(stream.peek_n(1).kind, TokenKind::Equals)
                    && matches!(
                        name.to_ascii_uppercase().as_str(),
                        "FROM" | "TO" | "TD" | "DEFAULT"
                    ) =>
            {
                break;
            }
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
            message: ".MEAS EQN expects an expression".to_string(),
        });
    }
    Ok(expression)
}

/// Scan the remainder of a .MEAS line for `GOAL=value` / `TOL=value`
/// verification options, skipping tokens it does not recognize.
fn parse_meas_goal_options(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<(Option<Value>, Option<Value>), ParseError> {
    let mut goal = None;
    let mut tolerance = None;
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        match &stream.peek().kind {
            TokenKind::Ident(s) if s.eq_ignore_ascii_case("GOAL") => {
                stream.advance();
                if !stream.consume(&TokenKind::Equals) {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: "Expected '=' after GOAL in .MEAS".to_string(),
                    });
                }
                goal = Some(expect_value(stream, line_num, params)?);
            }
            TokenKind::Ident(s) if s.eq_ignore_ascii_case("TOL") => {
                stream.advance();
                if !stream.consume(&TokenKind::Equals) {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: "Expected '=' after TOL in .MEAS".to_string(),
                    });
                }
                tolerance = Some(expect_value(stream, line_num, params)?);
            }
            _ => {
                stream.advance();
            }
        }
    }
    Ok((goal, tolerance))
}

pub(super) fn parse_meas_delay_spec(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    section_name: &str,
    stop_at_targ: bool,
) -> Result<crate::analysis::TrigSpec, ParseError> {
    use crate::analysis::{EdgeType, EventOccurrence, TrigSpec, TriggerEvent, WhenCondition};

    let event = if matches!(&stream.peek().kind, TokenKind::Ident(value) if value.eq_ignore_ascii_case("AT"))
        && matches!(stream.peek_n(1).kind, TokenKind::Equals)
    {
        stream.advance();
        stream.advance();
        TriggerEvent::At(expect_value(stream, line_num, params)?)
    } else {
        let left = parse_meas_signal(stream, line_num)?;
        let right = if stream.consume(&TokenKind::Equals) {
            parse_measure_when_operand(stream, line_num, params)?
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
    let mut spec = TrigSpec { event, td: None };
    let mut occurrence_given = false;
    let mut td_given = false;

    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        match &stream.peek().kind {
            TokenKind::Comma => {
                stream.advance();
            }
            TokenKind::Ident(s) if stop_at_targ && s.eq_ignore_ascii_case("TARG") => break,
            // Verification options end the spec; the statement parser
            // consumes them.
            TokenKind::Ident(s)
                if s.eq_ignore_ascii_case("GOAL") || s.eq_ignore_ascii_case("TOL") =>
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
                if td_given {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!(
                            "Duplicate TD option in .MEAS {section_name} specification"
                        ),
                    });
                }
                stream.advance();
                if !stream.consume(&TokenKind::Equals) {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!(
                            "Expected '=' after TD in .MEAS {} specification",
                            section_name
                        ),
                    });
                }
                spec.td = Some(expect_value(stream, line_num, params)?);
                td_given = true;
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
    let rounded = value.round();
    if !value.is_finite()
        || value == 0.0
        || (value - rounded).abs() > 1e-12
        || rounded < isize::MIN as crate::Value
        || rounded > isize::MAX as crate::Value
    {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Expected a non-zero integer occurrence or LAST for {keyword} in .MEAS, found {value}"
            ),
        });
    }
    Ok(rounded as isize)
}

pub(super) fn parse_measure_range_options(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<(Option<crate::Value>, Option<crate::Value>), ParseError> {
    let mut from = None;
    let mut to = None;

    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        match &stream.peek().kind {
            TokenKind::Ident(s)
                if s.eq_ignore_ascii_case("GOAL") || s.eq_ignore_ascii_case("TOL") =>
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
                stream.advance();
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
        crate::analysis::ExtremaOutput,
    ),
    ParseError,
> {
    use crate::analysis::ExtremaOutput;

    let mut from = None;
    let mut to = None;
    let mut output = ExtremaOutput::Value;
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        let TokenKind::Ident(key) = &stream.peek().kind else {
            stream.advance();
            continue;
        };
        let key = key.to_ascii_uppercase();
        if matches!(key.as_str(), "GOAL" | "TOL") {
            break;
        }
        stream.advance();
        if !matches!(key.as_str(), "FROM" | "TO" | "OUTPUT") {
            continue;
        }
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
        Option<crate::Value>,
    ),
    ParseError,
> {
    let mut from = None;
    let mut to = None;
    let mut td = None;
    let mut default_value = None;

    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
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
        if !matches!(key.as_str(), "FROM" | "TO" | "TD" | "DEFAULT_VAL") {
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
            "DEFAULT_VAL" => default_value = Some(value),
            _ => unreachable!(),
        }
    }

    Ok((from, to, td, default_value))
}

pub(super) fn parse_param_statement(
    stream: &mut TokenStream,
    line_num: usize,
    params: &mut ParamContext,
    mut deferred_params: Option<&mut Vec<(String, String)>>,
    retain_global_expression: bool,
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
                "TIME" | "TEMP" | "TEMPER" | "VT" | "GMIN" | "FREQ" | "FREQUENCY"
            )
        {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(".GLOBAL_PARAM name '{}' is reserved by the simulator", name),
            });
        }

        if matches!(stream.peek().kind, TokenKind::LParen) {
            parse_param_function_definition(stream, line_num, params, name)?;
            continue;
        }

        // Expect = sign
        if !stream.consume(&TokenKind::Equals) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Expected '=' after parameter name '{}'", name),
            });
        }

        // Get the value (could be number, expression, or string-valued vector).
        match &stream.peek().kind {
            TokenKind::StringLit(value) => {
                let value = value.clone();
                stream.advance();
                params.set_string(&name, value);
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
                params.set_string(&name, value);
                clear_deferred_param_expression(deferred_params.as_deref_mut(), &name);
            }
            _ if param_rhs_continues(stream) => {
                let expr = collect_param_rhs_expression(stream, line_num, &name)?;
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
            TokenKind::Expression(expr) if params.get_string(expr).is_some() => {
                let value = params
                    .get_string(expr)
                    .expect("string parameter presence checked")
                    .to_string();
                stream.advance();
                params.set_string(&name, value);
                clear_deferred_param_expression(deferred_params.as_deref_mut(), &name);
            }
            TokenKind::Expression(expr) => {
                let expr = expr.clone();
                stream.advance();
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
                                    Some(crate::netlist::expr::ComplexValue::real(value)),
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
                            params.set(&name, value);
                            clear_deferred_param_expression(deferred_params.as_deref_mut(), &name);
                        }
                    }
                    Err(err) => {
                        let expr = collect_param_rhs_expression(stream, line_num, &name)?;
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
    }

    Ok(())
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
    if let Some(deferred_params) = deferred_params
        && parameter_error_can_defer(&err)
    {
        upsert_param_expression(deferred_params, name, expr);
        return Ok(());
    }
    if retain_global_expression && parameter_error_can_defer(&err) {
        params.define_global_expression(&name, expr, None);
        return Ok(());
    }
    Err(err)
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
    let first = expect_ident(stream, line_num)?;
    let first_upper = first.to_ascii_uppercase();

    if is_dc_sweep_type(&first_upper) {
        let source = expect_ident(stream, line_num)?;
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
            while let Some(value) = try_value(stream, params) {
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
    let step = if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        if dc_bounds_are_same_point(start, stop) {
            1.0
        } else {
            return Err(ParseError::Syntax {
                line: line_num,
                message: ".DC linear sweep requires a step value unless start and stop are equal"
                    .to_string(),
            });
        }
    } else {
        expect_value(stream, line_num, params)?
    };
    Ok(crate::netlist::DcSweepSpec::linear(start, stop, step))
}

fn dc_bounds_are_same_point(start: Value, stop: Value) -> bool {
    if !start.is_finite() || !stop.is_finite() {
        return false;
    }
    let scale = start.abs().max(stop.abs()).max(1.0);
    (start - stop).abs() <= Value::EPSILON * scale
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
    use crate::Netlist;

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
    fn options_reject_non_finite_or_nonphysical_temperature() {
        for options in [
            ".options temp=1e309",
            ".options temp=-273.15",
            ".options tnom=1e309",
            ".options tnom=-300",
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
            ".options timeint reltol=2e-6 abstol=3e-9 newlte=2",
        ))
        .expect("TIMEINT tolerances parse");

        assert_eq!(netlist.options.timeint_reltol, Some(2.0e-6));
        assert_eq!(netlist.options.timeint_abstol, Some(3.0e-9));
        assert_eq!(netlist.options.reltol, None);
        assert_eq!(netlist.options.abstol, None);
        assert_eq!(
            netlist.options.transient_lte_reference,
            Some(super::TransientLteReference::SignalGlobal)
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
            ".options nonlin-tran reltol=1e-3\n\
             .options timeint method=gear",
        ))
        .expect("Xyce hyphenated solver option package parses");

        assert_eq!(netlist.options.reltol, Some(1.0e-3));
        assert_eq!(netlist.options.method.as_deref(), Some("GEAR"));
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
}
