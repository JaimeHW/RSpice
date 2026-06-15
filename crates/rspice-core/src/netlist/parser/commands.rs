//! Dot-command parsing for analyses, options, measurements, params, and functions.

use super::*;

pub(super) fn parse_command(
    stream: &mut TokenStream,
    line_num: usize,
    context: ParseCommandContext<'_>,
) -> Result<(), ParseError> {
    let ParseCommandContext {
        analyses,
        unknown_warned,
        models,
        params,
        initial_conditions,
        node_sets,
        global_nodes,
        measurements,
        saves,
        options,
        spef_includes,
    } = context;

    let cmd = expect_ident(stream, line_num)?;

    match cmd.as_str() {
        ".OP" => {
            analyses.push(AnalysisCommand::Op);
        }
        ".DC" => {
            let source = expect_ident(stream, line_num)?;
            let start = expect_value(stream, line_num, params)?;
            let stop = expect_value(stream, line_num, params)?;
            let step = expect_value(stream, line_num, params)?;

            // Optional second (outer) source: .DC V1 a b s V2 a2 b2 s2
            skip_commas(stream);
            let sweep2 = if matches!(stream.peek().kind, TokenKind::Ident(_)) {
                let source2 = expect_ident(stream, line_num)?;
                let start2 = expect_value(stream, line_num, params)?;
                let stop2 = expect_value(stream, line_num, params)?;
                let step2 = expect_value(stream, line_num, params)?;
                Some(crate::netlist::DcSecondSweep {
                    source: source2,
                    start: start2,
                    stop: stop2,
                    step: step2,
                })
            } else {
                None
            };

            analyses.push(AnalysisCommand::Dc {
                source,
                start,
                stop,
                step,
                sweep2,
            });
        }
        ".AC" => {
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

            analyses.push(AnalysisCommand::Ac {
                variation,
                points,
                start_freq,
                stop_freq,
            });
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
            let model = parse_model_definition(stream, line_num, params, models)?;
            models.push(model);
        }
        ".PARAM" | ".CSPARAM" => {
            parse_param_statement(stream, line_num, params)?;
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
            parse_ic_command(stream, line_num, params, initial_conditions)?;
        }
        ".NODESET" => {
            parse_nodeset_command(stream, line_num, params, node_sets)?;
        }
        ".INCLUDE" | ".INC" => {
            // Include directives are handled in a preprocessing pass
            log::debug!("Include directive found: line {}", line_num);
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
        ".OPTIONS" | ".OPTION" | ".OPT" => {
            parse_options_command(stream, line_num, params, options, unknown_warned)?;
        }
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
                log::warn!(
                    "line {line_num}: unsupported dot-command '{cmd}' ignored - \
                     whatever it requests (analysis, option, output) will not run"
                );
            }
        }
    }

    Ok(())
}

/// Parse a `.SAVE`/`.PROBE`/`.PRINT`/`.PLOT` probe list into the netlist's
/// output-selection set.
///
/// Accepted probe forms (case-insensitive):
/// - `all`
/// - `v(node)` / `v(a,b)` — also when the lexer splits them into
///   `v ( node )` token runs
/// - `i(elem)`
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
                let is_probe_prefix = upper == "V" || upper == "I";
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

/// Parse a single textual probe (`v(out)`, `v(a,b)`, `i(v1)`, `@m1[id]`, or a
/// bare vector name) into a [`super::SaveSignal`].
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
) -> Result<(), ParseError> {
    while !stream.is_eof() {
        skip_commas(stream);
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }

        let key = expect_ident(stream, line_num)?;
        let key_upper = key.to_uppercase();
        let has_equals = stream.consume(&TokenKind::Equals);

        match key_upper.as_str() {
            "RELTOL" => options.reltol = Some(expect_value(stream, line_num, params)?),
            "ABSTOL" => options.abstol = Some(expect_value(stream, line_num, params)?),
            "VNTOL" => options.vntol = Some(expect_value(stream, line_num, params)?),
            "IABSTOL" => options.iabstol = Some(expect_value(stream, line_num, params)?),
            "RESIDUAL_RELTOL" | "RESRELTOL" => {
                options.residual_reltol = Some(expect_value(stream, line_num, params)?)
            }
            "GMIN" => options.gmin = Some(expect_value(stream, line_num, params)?),
            "TRTOL" => options.trtol = Some(expect_value(stream, line_num, params)?),
            "CHGTOL" => options.chgtol = Some(expect_value(stream, line_num, params)?),
            "PIVTOL" => options.pivtol = Some(expect_value(stream, line_num, params)?),
            "TEMP" => options.temp = Some(expect_value(stream, line_num, params)?),
            "TNOM" => options.tnom = Some(expect_value(stream, line_num, params)?),
            "SEED" | "RNDSEED" => {
                // The parse pre-scan applies the seed before any parameter
                // evaluation; this arm validates and records it for
                // downstream drivers (e.g. per-run Monte-Carlo streams).
                if let TokenKind::Ident(word) = &stream.peek().kind {
                    if word.eq_ignore_ascii_case("random") {
                        stream.advance();
                        log::warn!(
                            "line {line_num}: `.options seed=random` is not supported; \
                             the deterministic default seed is used (set an explicit \
                             integer seed to vary the stream)"
                        );
                        continue;
                    }
                }
                let value = expect_value(stream, line_num, params)?;
                options.seed = Some(parse_seed_option(value, line_num)?);
            }
            "ITL1" => {
                let value = expect_value(stream, line_num, params)?;
                options.itl1 = Some(parse_usize_option("ITL1", value, line_num)?);
            }
            "ITL2" => {
                let value = expect_value(stream, line_num, params)?;
                options.itl2 = Some(parse_usize_option("ITL2", value, line_num)?);
            }
            "ITL4" => {
                let value = expect_value(stream, line_num, params)?;
                options.itl4 = Some(parse_usize_option("ITL4", value, line_num)?);
            }
            "ITL6" => {
                let value = expect_value(stream, line_num, params)?;
                options.itl6 = Some(parse_usize_option("ITL6", value, line_num)?);
            }
            "METHOD" => {
                let method = expect_ident(stream, line_num)?;
                options.method = Some(method.to_uppercase());
            }
            "ALLOW_SIMPLIFIED_MOS" | "ALLOWSIMPLIFIEDMOS" => {
                // Bare flag enables; an explicit value of 0 disables.
                let enabled = if has_equals {
                    expect_value(stream, line_num, params)? != 0.0
                } else {
                    true
                };
                options.allow_simplified_mos = Some(enabled);
            }
            _ => {
                // Unknown option: allow bare flags; consume value only when
                // explicitly assigned. Silently swallowing tolerance or
                // compatibility knobs misleads users into thinking they took
                // effect, so say so once per key.
                if unknown_warned.insert(format!(".options {key_upper}")) {
                    log::warn!("line {line_num}: unknown .options key '{key}' ignored");
                }
                if has_equals
                    && try_value(stream, params).is_none()
                    && matches!(stream.peek().kind, TokenKind::Ident(_))
                {
                    stream.advance();
                }
            }
        }
    }

    Ok(())
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

pub(super) fn parse_meas_signal(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<String, ParseError> {
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

    // Parse measurement name
    let name = expect_ident(stream, line_num)?;

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
        "PARAM" => {
            // .MEAS <an> name PARAM='expr' — an expression over previously
            // evaluated measurement results.
            if !stream.consume(&TokenKind::Equals) {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: "Expected '=' after PARAM in .MEAS".to_string(),
                });
            }
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
            MeasureType::Param { expression }
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
                    let (from, to) = parse_measure_range_options(stream, line_num, params)?;
                    MeasureType::Max {
                        signal: signal.clone(),
                        from,
                        to,
                    }
                }
                "MIN" => {
                    let (from, to) = parse_measure_range_options(stream, line_num, params)?;
                    MeasureType::Min {
                        signal: signal.clone(),
                        from,
                        to,
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
                "INTEG" => {
                    let (from, to) = parse_measure_range_options(stream, line_num, params)?;
                    MeasureType::Integ {
                        signal: signal.clone(),
                        from,
                        to,
                    }
                }
                "FIND" => {
                    let mut at = None;
                    let mut when_signal = None;
                    let mut when_value = None;

                    while !stream.is_eof()
                        && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof)
                    {
                        match &stream.peek().kind {
                            TokenKind::Ident(s) if s.eq_ignore_ascii_case("AT") => {
                                stream.advance();
                                if !stream.consume(&TokenKind::Equals) {
                                    return Err(ParseError::Syntax {
                                        line: line_num,
                                        message: "Expected '=' after AT in .MEAS FIND".to_string(),
                                    });
                                }
                                at = Some(expect_value(stream, line_num, params)?);
                            }
                            TokenKind::Ident(s) if s.eq_ignore_ascii_case("WHEN") => {
                                stream.advance();
                                when_signal = Some(parse_meas_signal(stream, line_num)?);
                                if !stream.consume(&TokenKind::Equals) {
                                    return Err(ParseError::Syntax {
                                        line: line_num,
                                        message: "Expected '=' after WHEN signal in .MEAS FIND"
                                            .to_string(),
                                    });
                                }
                                when_value = Some(expect_value(stream, line_num, params)?);
                            }
                            // Verification options belong to the statement,
                            // not the FIND clause.
                            TokenKind::Ident(s)
                                if s.eq_ignore_ascii_case("GOAL")
                                    || s.eq_ignore_ascii_case("TOL") =>
                            {
                                break;
                            }
                            _ => {
                                stream.advance();
                            }
                        }
                    }

                    MeasureType::Find {
                        signal: signal.clone(),
                        at,
                        when_signal,
                        when_value,
                    }
                }
                "DERIV" | "DERIVATIVE" => {
                    let mut at = None;
                    let mut when_signal = None;
                    let mut when_value = None;

                    while !stream.is_eof()
                        && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof)
                    {
                        match &stream.peek().kind {
                            TokenKind::Ident(s) if s.eq_ignore_ascii_case("AT") => {
                                stream.advance();
                                if !stream.consume(&TokenKind::Equals) {
                                    return Err(ParseError::Syntax {
                                        line: line_num,
                                        message: "Expected '=' after AT in .MEAS DERIV".to_string(),
                                    });
                                }
                                at = Some(expect_value(stream, line_num, params)?);
                            }
                            TokenKind::Ident(s) if s.eq_ignore_ascii_case("WHEN") => {
                                stream.advance();
                                when_signal = Some(parse_meas_signal(stream, line_num)?);
                                if !stream.consume(&TokenKind::Equals) {
                                    return Err(ParseError::Syntax {
                                        line: line_num,
                                        message: "Expected '=' after WHEN signal in .MEAS DERIV"
                                            .to_string(),
                                    });
                                }
                                when_value = Some(expect_value(stream, line_num, params)?);
                            }
                            TokenKind::Ident(s)
                                if s.eq_ignore_ascii_case("GOAL")
                                    || s.eq_ignore_ascii_case("TOL") =>
                            {
                                break;
                            }
                            _ => {
                                stream.advance();
                            }
                        }
                    }

                    MeasureType::Derivative {
                        signal: signal.clone(),
                        at,
                        when_signal,
                        when_value,
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
    use crate::analysis::{EdgeType, TrigSpec};

    let signal = parse_meas_signal(stream, line_num)?;
    let mut value = None;
    let mut edge = EdgeType::Rise;
    let mut number = 1usize;
    let mut td = 0.0;

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
                stream.advance();
                if !stream.consume(&TokenKind::Equals) {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!(
                            "Expected '=' after VAL in .MEAS {} specification",
                            section_name
                        ),
                    });
                }
                value = Some(expect_value(stream, line_num, params)?);
            }
            TokenKind::Ident(s) if s.eq_ignore_ascii_case("TD") => {
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
                td = expect_value(stream, line_num, params)?;
            }
            TokenKind::Ident(s)
                if s.eq_ignore_ascii_case("RISE")
                    || s.eq_ignore_ascii_case("FALL")
                    || s.eq_ignore_ascii_case("CROSS") =>
            {
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
                number = parse_measure_occurrence(stream, line_num, params, &keyword)?;
                edge = match keyword.as_str() {
                    "RISE" => EdgeType::Rise,
                    "FALL" => EdgeType::Fall,
                    "CROSS" => EdgeType::Cross,
                    _ => unreachable!(),
                };
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

    let value = value.ok_or_else(|| ParseError::Syntax {
        line: line_num,
        message: format!("Expected VAL=... in .MEAS {} specification", section_name),
    })?;

    let mut spec = TrigSpec::new(&signal, value)
        .with_edge(edge)
        .with_number(number);
    spec.td = td;
    Ok(spec)
}

pub(super) fn parse_measure_occurrence(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    keyword: &str,
) -> Result<usize, ParseError> {
    let value = expect_value(stream, line_num, params)?;
    let rounded = value.round();
    if !value.is_finite() || value < 1.0 || (value - rounded).abs() > 1e-12 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Expected positive integer occurrence for {} in .MEAS, found {}",
                keyword, value
            ),
        });
    }
    Ok(rounded as usize)
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

pub(super) fn parse_param_statement(
    stream: &mut TokenStream,
    line_num: usize,
    params: &mut ParamContext,
) -> Result<(), ParseError> {
    // Parse one or more NAME=VALUE pairs
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline) {
        // Skip commas
        while stream.consume(&TokenKind::Comma) {}

        if stream.is_eof() || matches!(stream.peek().kind, TokenKind::Newline) {
            break;
        }

        let name = expect_ident(stream, line_num)?;

        // Expect = sign
        if !stream.consume(&TokenKind::Equals) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Expected '=' after parameter name '{}'", name),
            });
        }

        // Get the value (could be number or expression)
        let value = expect_value(stream, line_num, params)?;
        params.set(&name, value);
    }

    Ok(())
}

/// Parse .FUNC statement: .FUNC name(arg1, arg2, ...) = expression
/// or: .FUNC name(arg1, arg2, ...) {expression}
pub(super) fn parse_func_statement(
    stream: &mut TokenStream,
    line_num: usize,
    params: &mut ParamContext,
) -> Result<(), ParseError> {
    // Get function name
    let func_name = expect_ident(stream, line_num)?;

    // Expect opening paren for arguments
    if !stream.consume(&TokenKind::LParen) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(".FUNC {} requires argument list in parentheses", func_name),
        });
    }

    // Parse argument names
    let mut args = Vec::new();
    if !stream.consume(&TokenKind::RParen) {
        loop {
            let arg_name = expect_ident(stream, line_num)?;
            args.push(arg_name);

            // Skip comma
            if !stream.consume(&TokenKind::Comma) {
                break;
            }
        }

        if !stream.consume(&TokenKind::RParen) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: "Expected ')' after function arguments".to_string(),
            });
        }
    }

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

/// Consume a trailing `UIC` keyword on a `.TRAN` card.
pub(super) fn consume_uic_keyword(stream: &mut TokenStream) -> bool {
    skip_commas(stream);
    if let TokenKind::Ident(word) = &stream.peek().kind
        && word.eq_ignore_ascii_case("UIC")
    {
        stream.advance();
        return true;
    }
    false
}
