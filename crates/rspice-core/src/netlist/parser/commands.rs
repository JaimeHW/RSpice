//! Dot-command parsing for analyses, options, measurements, params, and functions.

use super::*;

pub(super) fn parse_command(
    stream: &mut TokenStream,
    line_num: usize,
    context: ParseCommandContext<'_>,
) -> Result<(), ParseError> {
    let ParseCommandContext {
        analyses,
        models,
        params,
        initial_conditions,
        node_sets,
        global_nodes,
        measurements,
        options,
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

            analyses.push(AnalysisCommand::Dc {
                source,
                start,
                stop,
                step,
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
        ".DISTO" => {
            let disto = parse_disto_command(stream, line_num, params)?;
            analyses.push(disto);
        }
        ".TRAN" => {
            let step = expect_value(stream, line_num, params)?;
            let stop = expect_value(stream, line_num, params)?;
            let start = try_value(stream, params);
            let max_step = try_value(stream, params);

            analyses.push(AnalysisCommand::Tran {
                step,
                stop,
                start,
                max_step,
            });
        }
        ".MODEL" => {
            models.push(parse_model_definition(stream, line_num, params)?);
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
        ".OPTIONS" | ".OPTION" | ".OPT" => {
            parse_options_command(stream, line_num, params, options)?;
        }
        ".MEAS" | ".MEASURE" => {
            // Parse measurement statement: .MEAS TRAN name TYPE signal [options]
            measurements.push(parse_meas_command(stream, line_num, params)?);
        }
        _ => {
            // Ignore unknown commands
            log::debug!("Ignoring unknown command: {}", cmd);
        }
    }

    Ok(())
}

pub(super) fn parse_options_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    options: &mut super::SimulationOptions,
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
            _ => {
                // Unknown option: allow bare flags; consume value only when explicitly assigned.
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
    })
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
