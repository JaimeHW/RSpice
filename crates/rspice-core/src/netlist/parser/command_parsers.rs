use super::*;

pub(super) fn parse_device_initial_condition_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    origin: &NetlistSourceLocation,
    directive: &mut Option<DeviceInitialConditionDirective>,
) -> Result<(), ParseError> {
    if let Some(first) = directive {
        return Err(ParseError::DeviceInitialCondition(Box::new(
            DeviceInitialConditionError::DuplicateDirective {
                first: first.origin.clone(),
                duplicate: origin.clone(),
            },
        )));
    }

    if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        return Err(ParseError::DeviceInitialCondition(Box::new(
            DeviceInitialConditionError::MissingInformation {
                origin: origin.clone(),
            },
        )));
    }

    if matches!(&stream.peek().kind, TokenKind::Ident(value) if value.eq_ignore_ascii_case("FILE"))
    {
        stream.advance();
        let requested_path = match &stream.peek().kind {
            TokenKind::StringLit(path) => {
                let path = path.clone();
                stream.advance();
                path
            }
            _ => take_authored_initcond_token(stream).ok_or_else(|| {
                ParseError::DeviceInitialCondition(Box::new(
                    DeviceInitialConditionError::MalformedDirective {
                        origin: origin.clone(),
                        detail: "FILE requires one path".to_string(),
                    },
                ))
            })?,
        };
        if requested_path.trim().is_empty()
            || !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof)
        {
            return Err(ParseError::DeviceInitialCondition(Box::new(
                DeviceInitialConditionError::MalformedDirective {
                    origin: origin.clone(),
                    detail: "FILE requires exactly one non-empty path".to_string(),
                },
            )));
        }
        *directive = Some(DeviceInitialConditionDirective {
            origin: origin.clone(),
            source: DeviceInitialConditionSource::File {
                requested_path,
                resolved_path: None,
                content_identity: None,
            },
            entries: Vec::new(),
        });
        return Ok(());
    }

    let entries = parse_device_initial_condition_entries(stream, line_num, params, origin)?;
    *directive = Some(DeviceInitialConditionDirective {
        origin: origin.clone(),
        source: DeviceInitialConditionSource::Inline,
        entries,
    });
    Ok(())
}

fn take_authored_initcond_token(stream: &mut TokenStream) -> Option<String> {
    let first = stream.peek().clone();
    if matches!(first.kind, TokenKind::Newline | TokenKind::Eof) {
        return None;
    }
    let mut path = first.lexeme;
    let mut end = first.span.end;
    stream.advance();
    while stream.peek().span.start == end
        && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof)
    {
        let token = stream.peek().clone();
        path.push_str(&token.lexeme);
        end = token.span.end;
        stream.advance();
    }
    Some(path)
}

pub(super) fn parse_device_initial_condition_entries(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    origin: &NetlistSourceLocation,
) -> Result<Vec<DeviceInitialConditionEntry>, ParseError> {
    let mut entries = Vec::new();

    while !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        let device = take_authored_initcond_token(stream).ok_or_else(|| {
            ParseError::DeviceInitialCondition(Box::new(
                DeviceInitialConditionError::MalformedDirective {
                    origin: origin.clone(),
                    detail: "expected a fully qualified device name".to_string(),
                },
            ))
        })?;
        let keyword = expect_ident(stream, line_num).map_err(|_| {
            ParseError::DeviceInitialCondition(Box::new(
                DeviceInitialConditionError::MalformedDirective {
                    origin: origin.clone(),
                    detail: format!("device '{device}' must be followed by IC=<value>"),
                },
            ))
        })?;
        if !keyword.eq_ignore_ascii_case("IC") || !stream.consume(&TokenKind::Equals) {
            return Err(ParseError::DeviceInitialCondition(Box::new(
                DeviceInitialConditionError::MalformedDirective {
                    origin: origin.clone(),
                    detail: format!("device '{device}' must be followed by IC=<value>"),
                },
            )));
        }

        let mut values = Vec::new();
        loop {
            let value = expect_value(stream, line_num, params).map_err(|error| {
                ParseError::DeviceInitialCondition(Box::new(
                    DeviceInitialConditionError::MalformedDirective {
                        origin: origin.clone(),
                        detail: format!("device '{device}' has an invalid IC value: {error}"),
                    },
                ))
            })?;
            if !value.is_finite() {
                return Err(ParseError::DeviceInitialCondition(Box::new(
                    DeviceInitialConditionError::NonFiniteValue {
                        origin: origin.clone(),
                        device: device.clone(),
                        value_index: values.len() + 1,
                        value,
                    },
                )));
            }
            values.push(value);
            if !stream.consume(&TokenKind::Comma) {
                break;
            }
        }

        entries.push(DeviceInitialConditionEntry {
            device,
            values,
            origin: origin.clone(),
        });
    }

    if entries.is_empty() {
        return Err(ParseError::DeviceInitialCondition(Box::new(
            DeviceInitialConditionError::MissingInformation {
                origin: origin.clone(),
            },
        )));
    }
    Ok(entries)
}

pub(super) fn parse_step_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<StepCommand, ParseError> {
    skip_commas(stream);

    // Check for sweep type prefix
    let first = expect_ident(stream, line_num)?;
    let first_upper = first.to_uppercase();
    if first_upper == "DATA" {
        if !stream.consume(&TokenKind::Equals) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: ".STEP DATA requires DATA=<table-name>".to_string(),
            });
        }
        let table_name = expect_ident(stream, line_num)?;
        return Ok(StepCommand {
            target: StepTarget::Param,
            name: table_name.clone(),
            param_name: None,
            sweep: StepSweep::Data { table_name },
        });
    }

    let (sweep_prefix, target_str, mut name) = match first_upper.as_str() {
        "DEC" | "OCT" | "LIN" => {
            let target = expect_ident(stream, line_num)?;
            let target_upper = target.to_uppercase();
            match target_upper.as_str() {
                "PARAM" | "MODEL" => {
                    let name = expect_ident(stream, line_num)?;
                    (Some(first_upper), target_upper, name)
                }
                "TEMP" => (Some(first_upper), target_upper, "TEMP".to_string()),
                _ if params.get(&target).is_some() => {
                    (Some(first_upper), "PARAM".to_string(), target)
                }
                _ => (Some(first_upper), "DEVICE".to_string(), target),
            }
        }
        "PARAM" | "MODEL" => {
            let name = expect_ident(stream, line_num)?;
            (None, first_upper, name)
        }
        "TEMP" => (None, first_upper, "TEMP".to_string()),
        _ if params.get(&first).is_some() => (None, "PARAM".to_string(), first),
        _ => {
            // Assume device parameter: .STEP R1(value) or .STEP R1 start stop step
            (None, "DEVICE".to_string(), first)
        }
    };

    let target = match target_str.as_str() {
        "PARAM" => StepTarget::Param,
        "MODEL" => StepTarget::Model,
        "TEMP" => StepTarget::Temp,
        _ => StepTarget::Device,
    };

    let mut param_name: Option<String> = None;
    if target == StepTarget::Device
        && let Some((device_name, device_param)) = name
            .rsplit_once(':')
            .map(|(device_name, device_param)| (device_name.to_string(), device_param.to_string()))
    {
        if device_name.is_empty()
            || device_param.is_empty()
            || device_name.split(':').any(str::is_empty)
        {
            return Err(ParseError::Syntax {
                line: line_num,
                message:
                    "Malformed .STEP device parameter target; expected device[:child...]:param"
                        .to_string(),
            });
        }
        name = device_name.to_string();
        param_name = Some(device_param.to_string());
    }

    match target {
        StepTarget::Model => {
            param_name = Some(expect_ident(stream, line_num)?);
        }
        StepTarget::Device => {
            // Optional device-parameter spec:
            // - .STEP R1(<param>) ...
            // - .STEP R1 <param> ...
            if param_name.is_none() && stream.consume(&TokenKind::LParen) {
                let pname = expect_ident(stream, line_num)?;
                if !stream.consume(&TokenKind::RParen) {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: "Expected ')' after .STEP device parameter name".to_string(),
                    });
                }
                param_name = Some(pname);
            } else if param_name.is_none()
                && let TokenKind::Ident(candidate) = &stream.peek().kind
            {
                let candidate_upper = candidate.to_ascii_uppercase();
                let reserved = ["LIST", "LIN", "DEC", "OCT", "PARAM", "MODEL", "TEMP"];
                let next_is_value_like = matches!(
                    stream.peek_n(1).kind,
                    TokenKind::Number(_)
                        | TokenKind::Expression(_)
                        | TokenKind::Plus
                        | TokenKind::Minus
                ) || matches!(
                    &stream.peek_n(1).kind,
                    TokenKind::Ident(next) if next.eq_ignore_ascii_case("LIST")
                );
                if !reserved.contains(&candidate_upper.as_str()) && next_is_value_like {
                    param_name = Some(candidate.clone());
                    stream.advance();
                }
            }
        }
        _ => {}
    }

    // Check for LIST keyword
    skip_commas(stream);
    let is_list = if let TokenKind::Ident(s) = &stream.peek().kind {
        if s.eq_ignore_ascii_case("LIST") {
            stream.advance();
            true
        } else {
            false
        }
    } else {
        false
    };

    let sweep = if is_list {
        // Parse list of values
        let mut values = Vec::new();
        while let Some(v) = try_value(stream, params) {
            values.push(v);
        }
        if values.is_empty() {
            return Err(ParseError::Syntax {
                line: line_num,
                message: "LIST requires at least one value".to_string(),
            });
        }
        StepSweep::List(values)
    } else {
        // Parse start stop increment/points
        let start = expect_value(stream, line_num, params)?;
        let stop = expect_value(stream, line_num, params)?;
        let step_or_points = expect_value(stream, line_num, params)?;

        match sweep_prefix.as_deref() {
            Some("DEC") => StepSweep::Decade {
                points_per_decade: parse_step_points_per_interval(step_or_points, "DEC", line_num)?,
                start,
                stop,
            },
            Some("OCT") => StepSweep::Octave {
                points_per_octave: parse_step_points_per_interval(step_or_points, "OCT", line_num)?,
                start,
                stop,
            },
            _ => StepSweep::Linear {
                start,
                stop,
                step: step_or_points,
            },
        }
    };

    Ok(StepCommand {
        target,
        name,
        param_name,
        sweep,
    })
}

fn parse_step_points_per_interval(
    value: Value,
    sweep_type: &str,
    line_num: usize,
) -> Result<usize, ParseError> {
    // A usize contains values below 2^BITS. Expressing this exclusive bound in
    // floating point avoids the rounding of `usize::MAX as f64` on 64-bit hosts.
    let usize_upper_bound = 2.0_f64.powi(usize::BITS as i32);
    if !value.is_finite() || value < 1.0 || value.fract() != 0.0 || value >= usize_upper_bound {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                ".STEP {sweep_type} points per interval must be a positive integer representable as usize, found {value}"
            ),
        });
    }

    Ok(value as usize)
}

/// Parse .TEMP command: .TEMP t1 [t2 t3...]
pub(super) fn parse_temp_command(
    stream: &mut TokenStream,
    params: &ParamContext,
) -> Result<Vec<Value>, ParseError> {
    let mut temperatures = Vec::new();

    while let Some(v) = try_value(stream, params) {
        temperatures.push(v);
    }

    if temperatures.is_empty() {
        temperatures.push(27.0); // Default room temperature
    }

    Ok(temperatures)
}

/// Parse .FOUR command: .FOUR freq output1 [output2...]
pub(super) fn parse_four_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<(Value, Vec<String>), ParseError> {
    let fundamental = expect_value(stream, line_num, params)?;

    let mut outputs = Vec::new();
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        skip_commas(stream);
        if matches!(stream.peek().kind, TokenKind::Ident(_)) {
            // Probe specs like V(out) span several tokens; reuse the .MEAS
            // signal parser so the node is not silently dropped.
            outputs.push(super::commands::parse_meas_signal(stream, line_num)?);
        } else {
            break;
        }
    }

    if outputs.is_empty() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: ".FOUR requires at least one output".to_string(),
        });
    }

    Ok((fundamental, outputs))
}

/// Parse .SP command: .SP DEC|LIN|OCT np fstart fstop [donoise]
pub(super) fn parse_sp_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<AnalysisCommand, ParseError> {
    let var_str = expect_ident(stream, line_num)?;
    let variation = match var_str.to_uppercase().as_str() {
        "LIN" => FreqVariation::Lin,
        "OCT" => FreqVariation::Oct,
        "DEC" => FreqVariation::Dec,
        _ => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Unknown .SP frequency variation: {}", var_str),
            });
        }
    };

    let points = expect_value(stream, line_num, params)? as usize;
    let start_freq = expect_value(stream, line_num, params)?;
    let stop_freq = expect_value(stream, line_num, params)?;
    let do_noise = if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        false
    } else {
        let raw = expect_value(stream, line_num, params)?;
        raw != 0.0
    };

    Ok(AnalysisCommand::Sp {
        variation,
        points,
        start_freq,
        stop_freq,
        do_noise,
    })
}

/// Parse .DISTO command: .DISTO DEC|LIN|OCT np fstart fstop [f2overf1]
pub(super) fn parse_disto_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<AnalysisCommand, ParseError> {
    let var_str = expect_ident(stream, line_num)?;
    let variation = match var_str.to_uppercase().as_str() {
        "LIN" => FreqVariation::Lin,
        "OCT" => FreqVariation::Oct,
        "DEC" => FreqVariation::Dec,
        _ => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    "Invalid .DISTO frequency variation '{}': expected LIN, OCT, or DEC",
                    var_str
                ),
            });
        }
    };

    let points = expect_value(stream, line_num, params)? as usize;
    let start_freq = expect_value(stream, line_num, params)?;
    let stop_freq = expect_value(stream, line_num, params)?;
    let f2_over_f1 = if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        None
    } else {
        Some(expect_value(stream, line_num, params)?)
    };

    Ok(AnalysisCommand::Disto {
        variation,
        points,
        start_freq,
        stop_freq,
        f2_over_f1,
    })
}

/// Parse .NOISE command: .NOISE V(out[,ref]) Vsource DEC|LIN|OCT np fstart fstop [pts_per_summary]
pub(super) fn parse_noise_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<AnalysisCommand, ParseError> {
    let (output_node, reference_node) = parse_voltage_output_reference(stream, line_num)?;

    // Input source
    let input_source = expect_ident(stream, line_num)?;

    // Frequency sweep type
    let var_str = expect_ident(stream, line_num)?;
    let variation = match var_str.to_uppercase().as_str() {
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
    let _summary_interval = try_value(stream, params);

    Ok(AnalysisCommand::Noise {
        output_node,
        reference_node,
        input_source,
        variation,
        points,
        start_freq,
        stop_freq,
    })
}

/// Parse .TF command: .TF V(out[,ref]) insrc  |  .TF I(element) insrc
pub(super) fn parse_tf_command(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<AnalysisCommand, ParseError> {
    let is_current_probe = matches!(&stream.peek().kind, TokenKind::Ident(s) if {
        let upper = s.to_uppercase();
        upper == "I" || upper.starts_with("I(")
    });

    let (output_node, reference_node, output_is_current) = if is_current_probe {
        let ident = expect_ident(stream, line_num)?;
        let element = if ident.len() > 1 {
            // Merged token form `I(ELEM)`.
            parse_inline_current_probe(&ident, line_num)?
        } else {
            if !stream.consume(&TokenKind::LParen) {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: "Expected '(' after I in .TF current probe".to_string(),
                });
            }
            let element = expect_ident(stream, line_num)?;
            if !stream.consume(&TokenKind::RParen) {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: "Expected ')' in I(element) specification".to_string(),
                });
            }
            element
        };
        (element.to_uppercase(), None, true)
    } else {
        let (node, reference) = parse_voltage_output_reference(stream, line_num)?;
        (node, reference, false)
    };

    let input_source = expect_ident(stream, line_num)?.to_uppercase();

    Ok(AnalysisCommand::Tf {
        output_node,
        reference_node,
        output_is_current,
        input_source,
    })
}

/// Extract the element name from a merged `I(ELEM)` token.
fn parse_inline_current_probe(token: &str, line_num: usize) -> Result<String, ParseError> {
    let inner = token[1..]
        .strip_prefix('(')
        .and_then(|s| s.strip_suffix(')'))
        .filter(|s| !s.is_empty())
        .ok_or_else(|| ParseError::Syntax {
            line: line_num,
            message: format!("Invalid current probe `{token}` in .TF (expected I(element))"),
        })?;
    Ok(inner.to_uppercase())
}

/// Parse .SENS command:
/// .SENS V(out[,ref])|I(vsource) [devspec ...] [AC DEC|LIN|OCT np fstart fstop]
pub(super) fn parse_sens_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<AnalysisCommand, ParseError> {
    let is_current_probe = matches!(&stream.peek().kind, TokenKind::Ident(s) if {
        let upper = s.to_uppercase();
        upper == "I" || upper.starts_with("I(")
    });
    let (output_node, reference_node, output_is_current) = if is_current_probe {
        let ident = expect_ident(stream, line_num)?;
        let element = if ident.len() > 1 {
            parse_inline_current_probe(&ident, line_num)?
        } else {
            if !stream.consume(&TokenKind::LParen) {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: "Expected '(' after I in .SENS current probe".to_string(),
                });
            }
            let element = expect_ident(stream, line_num)?;
            if !stream.consume(&TokenKind::RParen) {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: "Expected ')' in I(element) specification".to_string(),
                });
            }
            element
        };
        (element.to_uppercase(), None, true)
    } else {
        let (node, reference) = parse_voltage_output_reference(stream, line_num)?;
        (node, reference, false)
    };

    let mut filters = Vec::new();
    while !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        let is_mode = matches!(
            &stream.peek().kind,
            TokenKind::Ident(mode)
                if mode.eq_ignore_ascii_case("AC") || mode.eq_ignore_ascii_case("DC")
        );
        if is_mode {
            break;
        }
        filters.push(consume_sensitivity_filter(stream));
    }

    let mut ac_sweep = None;

    if !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        let mode = expect_ident(stream, line_num)?;
        if mode.eq_ignore_ascii_case("DC") {
            if !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: ".SENS DC does not accept a frequency sweep".to_string(),
                });
            }
            return Ok(AnalysisCommand::Sensitivity {
                output_node,
                reference_node,
                output_is_current,
                filters,
                ac_sweep,
            });
        }
        if !mode.eq_ignore_ascii_case("AC") {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    "Invalid .SENS mode '{}': expected AC, DC, a device filter, or end-of-line",
                    mode
                ),
            });
        }

        let var_str = expect_ident(stream, line_num)?;
        let variation = match var_str.to_uppercase().as_str() {
            "LIN" => FreqVariation::Lin,
            "OCT" => FreqVariation::Oct,
            "DEC" => FreqVariation::Dec,
            _ => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "Invalid .SENS AC sweep variation '{}': expected LIN, OCT, or DEC",
                        var_str
                    ),
                });
            }
        };

        let points = expect_value(stream, line_num, params)? as usize;
        let start_freq = expect_value(stream, line_num, params)?;
        let stop_freq = expect_value(stream, line_num, params)?;

        ac_sweep = Some(SensitivityAcSweep {
            variation,
            points,
            start_freq,
            stop_freq,
        });
    }

    Ok(AnalysisCommand::Sensitivity {
        output_node,
        reference_node,
        output_is_current,
        filters,
        ac_sweep,
    })
}

/// Consume one whitespace-delimited `.SENS` device specification. Wildcards
/// are separate lexer tokens, so source spans are used to join adjacent pieces
/// (`M*`, `MOD:*`, `R?_TC1`) without merging the next whitespace-separated
/// filter.
fn consume_sensitivity_filter(stream: &mut TokenStream) -> String {
    let first = stream.advance().clone();
    let line = first.span.line;
    let mut end = first.span.end;
    let mut filter = first.lexeme;
    while !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof)
        && stream.peek().span.line == line
        && stream.peek().span.start == end
    {
        let token = stream.advance().clone();
        end = token.span.end;
        filter.push_str(&token.lexeme);
    }
    filter.to_ascii_uppercase()
}

/// Parse .PZ command: .PZ in+ in- out+ out- VOL|CUR PZ|POL|ZER
pub(super) fn parse_pz_command(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<AnalysisCommand, ParseError> {
    let input_pos = expect_node(stream, line_num)?;
    let input_neg = expect_node(stream, line_num)?;
    let output_pos = expect_node(stream, line_num)?;
    let output_neg = expect_node(stream, line_num)?;

    let transfer_type = expect_ident(stream, line_num)?;
    let transfer_type = match transfer_type.to_uppercase().as_str() {
        "VOL" => PoleZeroTransferType::Voltage,
        "CUR" => PoleZeroTransferType::Current,
        _ => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    "Invalid .PZ transfer type '{}': expected VOL or CUR",
                    transfer_type
                ),
            });
        }
    };

    let analysis_type = expect_ident(stream, line_num)?;
    let analysis_type = match analysis_type.to_uppercase().as_str() {
        "PZ" => PoleZeroAnalysisType::PoleZero,
        "POL" => PoleZeroAnalysisType::PolesOnly,
        "ZER" => PoleZeroAnalysisType::ZerosOnly,
        _ => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    "Invalid .PZ analysis type '{}': expected PZ, POL, or ZER",
                    analysis_type
                ),
            });
        }
    };

    Ok(AnalysisCommand::PoleZero {
        input_pos,
        input_neg,
        output_pos,
        output_neg,
        transfer_type,
        analysis_type,
    })
}

/// Parse .MC command:
/// .MC runs [SEED n] [DIST GAUSS|UNIFORM|WORSTCASE] [SPREAD rel] [PARAMS p1 p2 ...]
///
/// Supported shorthand:
/// .MC runs GAUSS sigma
/// .MC runs UNIFORM tol
/// .MC runs WORSTCASE tol
pub(super) fn parse_mc_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<MonteCarloCommand, ParseError> {
    let runs_raw = expect_value(stream, line_num, params)?;
    if !runs_raw.is_finite() || runs_raw < 1.0 || (runs_raw.fract().abs() > 1e-12) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Invalid .MC run count '{}': expected positive integer",
                runs_raw
            ),
        });
    }
    let mut command = MonteCarloCommand::new(runs_raw as usize);

    let parse_distribution = |s: &str| -> Option<MonteCarloDistribution> {
        match s.to_ascii_uppercase().as_str() {
            "GAUSS" | "GAUSSIAN" | "NORMAL" => Some(MonteCarloDistribution::Gaussian),
            "UNIFORM" | "UNIF" => Some(MonteCarloDistribution::Uniform),
            "WORST" | "WORSTCASE" | "WC" => Some(MonteCarloDistribution::WorstCase),
            _ => None,
        }
    };

    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        let keyword = expect_ident(stream, line_num)?;
        match keyword.as_str() {
            "SEED" => {
                let seed_raw = expect_value(stream, line_num, params)?;
                if !seed_raw.is_finite() || seed_raw < 0.0 || (seed_raw.fract().abs() > 1e-12) {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!(
                            "Invalid .MC seed '{}': expected non-negative integer",
                            seed_raw
                        ),
                    });
                }
                command.seed = Some(seed_raw as u64);
            }
            "DIST" | "DISTRIBUTION" => {
                let dist = expect_ident(stream, line_num)?;
                command.distribution =
                    parse_distribution(&dist).ok_or_else(|| ParseError::Syntax {
                        line: line_num,
                        message: format!(
                            "Invalid .MC distribution '{}': expected GAUSS, UNIFORM, or WORSTCASE",
                            dist
                        ),
                    })?;

                if let Some(spread) = try_value(stream, params) {
                    command.relative_spread = spread;
                }
            }
            "GAUSS" | "GAUSSIAN" | "NORMAL" => {
                command.distribution = MonteCarloDistribution::Gaussian;
                if let Some(spread) = try_value(stream, params) {
                    command.relative_spread = spread;
                }
            }
            "UNIFORM" | "UNIF" => {
                command.distribution = MonteCarloDistribution::Uniform;
                if let Some(spread) = try_value(stream, params) {
                    command.relative_spread = spread;
                }
            }
            "WORST" | "WORSTCASE" | "WC" => {
                command.distribution = MonteCarloDistribution::WorstCase;
                if let Some(spread) = try_value(stream, params) {
                    command.relative_spread = spread;
                }
            }
            "SPREAD" | "SIGMA" | "TOL" | "TOLERANCE" => {
                command.relative_spread = expect_value(stream, line_num, params)?;
            }
            "PARAMS" | "PARAMETERS" => {
                while !stream.is_eof()
                    && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof)
                {
                    skip_commas(stream);
                    match &stream.peek().kind {
                        TokenKind::Ident(name) => {
                            let name = name.clone();
                            stream.advance();
                            if !command.params.iter().any(|p| p == &name) {
                                command.params.push(name);
                            }
                        }
                        other => {
                            return Err(ParseError::Syntax {
                                line: line_num,
                                message: format!(
                                    "Invalid .MC parameter list token {:?}: expected identifier",
                                    other
                                ),
                            });
                        }
                    }
                }

                if command.params.is_empty() {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: "Invalid .MC PARAMS list: expected at least one parameter name"
                            .to_string(),
                    });
                }
            }
            _ => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "Invalid .MC keyword '{}': expected SEED, DIST, SPREAD, or PARAMS",
                        keyword
                    ),
                });
            }
        }
    }

    if !command.relative_spread.is_finite() || command.relative_spread < 0.0 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Invalid .MC spread '{}': expected non-negative finite value",
                command.relative_spread
            ),
        });
    }

    Ok(command)
}

/// Parse voltage reference like V(out) or V(out,0)
pub(super) fn parse_voltage_reference(spec: &str) -> Result<(String, Option<String>), ParseError> {
    let spec_upper = spec.to_uppercase();

    if !spec_upper.starts_with("V(") {
        return Ok((spec.to_string(), None));
    }

    // Remove V( prefix and ) suffix
    let inner = spec
        .trim_start_matches(['V', 'v'])
        .trim_start_matches('(')
        .trim_end_matches(')');

    let parts: Vec<&str> = inner.split(',').collect();

    let node = parts[0].trim().to_string();
    let reference = if parts.len() > 1 {
        Some(parts[1].trim().to_string())
    } else {
        None
    };

    Ok((node, reference))
}

/// Parse voltage output specification from stream:
/// - `V(node)`
/// - `V(node,ref)`
/// - bare `node`
pub(super) fn parse_voltage_output_reference(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<(String, Option<String>), ParseError> {
    let first = expect_ident(stream, line_num)?;

    if first.to_uppercase() == "V" && matches!(stream.peek().kind, TokenKind::LParen) {
        stream.advance(); // (
        let node = expect_node(stream, line_num)?;
        let reference = if stream.consume(&TokenKind::Comma) {
            Some(expect_node(stream, line_num)?)
        } else {
            None
        };
        if !stream.consume(&TokenKind::RParen) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: "Expected ')' in V(node) specification".to_string(),
            });
        }
        return Ok((node, reference));
    }

    if first.to_uppercase().starts_with("V(") {
        return parse_voltage_reference(&first);
    }

    Ok((first, None))
}

/// Parse .NODESET command: .NODESET V(node1)=val V(node2)=val...
pub(super) fn parse_nodeset_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    node_sets: &mut Vec<NodeSet>,
    defer_values: bool,
) -> Result<(), ParseError> {
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        skip_commas(stream);
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }

        let Some(node) = parse_voltage_hint_target(stream, line_num)? else {
            break;
        };

        stream.consume(&TokenKind::Equals);
        let (voltage, voltage_expr) =
            parse_voltage_hint_value(stream, line_num, params, defer_values)?;
        node_sets.push(NodeSet {
            node,
            voltage,
            voltage_expr,
        });
    }

    Ok(())
}

/// Parse .IC command: .IC V(node1)=val V(node2)=val...
///
/// Initial conditions set the starting voltages for transient analysis.
/// Format: .IC V(node)=voltage [V(node2)=voltage2] ...
pub(super) fn parse_ic_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    initial_conditions: &mut Vec<InitialCondition>,
    defer_values: bool,
) -> Result<(), ParseError> {
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        skip_commas(stream);
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }

        let Some(node) = parse_voltage_hint_target(stream, line_num)? else {
            break;
        };

        stream.consume(&TokenKind::Equals);
        let (voltage, voltage_expr) =
            parse_voltage_hint_value(stream, line_num, params, defer_values)?;
        initial_conditions.push(InitialCondition {
            node,
            voltage,
            voltage_expr,
        });
    }

    Ok(())
}

fn parse_voltage_hint_value(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    defer_values: bool,
) -> Result<(Value, Option<String>), ParseError> {
    if defer_values {
        // A subcircuit-scoped .IC/.NODESET expression must be evaluated in
        // each instance's effective parameter scope. Evaluating a formal
        // parameter here would freeze its definition-time default and discard
        // an X-line override before flattening. Preserve every scoped value
        // expression, while still validating it and retaining the definition
        // scope's value for parser diagnostics and introspection.
        let mut expression_stream = stream.clone();
        let expression = collect_voltage_hint_expression(&mut expression_stream, line_num)?;
        let mut value_stream = stream.clone();
        let voltage = match expect_value(&mut value_stream, line_num, params) {
            Ok(value) => value,
            Err(err) if parameter_error_can_defer(&err) => Value::NAN,
            Err(err) => return Err(err),
        };
        *stream = expression_stream;
        return Ok((voltage, Some(expression)));
    }

    let mut value_stream = stream.clone();
    match expect_value(&mut value_stream, line_num, params) {
        Ok(value) => {
            *stream = value_stream;
            Ok((value, None))
        }
        Err(err) => Err(err),
    }
}

fn collect_voltage_hint_expression(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<String, ParseError> {
    let mut expression = String::new();
    let mut saw_token = false;

    loop {
        if matches!(
            stream.peek().kind,
            TokenKind::Newline | TokenKind::Eof | TokenKind::Comma
        ) {
            break;
        }
        if saw_token && looks_like_voltage_hint_target(stream) {
            break;
        }

        let token = stream.peek().clone();
        let fragment = match &token.kind {
            TokenKind::Expression(expr) => expr.clone(),
            TokenKind::Ident(_)
            | TokenKind::Number(_)
            | TokenKind::Equals
            | TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::LParen
            | TokenKind::RParen
            | TokenKind::AtSign
            | TokenKind::Tilde
            | TokenKind::Other(_) => token.lexeme.clone(),
            other => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!("Expected voltage expression, found {}", other),
                });
            }
        };
        if !expression.is_empty()
            && expression
                .chars()
                .last()
                .is_some_and(|ch| ch.is_ascii_alphanumeric() || ch == '_')
            && fragment
                .chars()
                .next()
                .is_some_and(|ch| ch.is_ascii_alphanumeric() || ch == '_')
        {
            expression.push(' ');
        }
        expression.push_str(&fragment);
        saw_token = true;
        stream.advance();
    }

    if expression.is_empty() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "Expected voltage expression".to_string(),
        });
    }

    Ok(expression)
}

fn looks_like_voltage_hint_target(stream: &TokenStream) -> bool {
    matches!(&stream.peek().kind, TokenKind::Ident(ident) if ident.eq_ignore_ascii_case("V"))
        && matches!(stream.peek_n(1).kind, TokenKind::LParen)
}

fn parse_voltage_hint_target(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<Option<String>, ParseError> {
    skip_commas(stream);
    if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        return Ok(None);
    }

    if let TokenKind::Ident(ident) = &stream.peek().kind
        && ident.eq_ignore_ascii_case("V")
        && matches!(stream.peek_n(1).kind, TokenKind::LParen)
    {
        stream.advance(); // V
        stream.advance(); // (

        let node = expect_node(stream, line_num)?;
        if stream.consume(&TokenKind::Comma) {
            // Optional reference node (e.g. V(out,0)); currently ignored.
            let _ = expect_node(stream, line_num)?;
        }

        if !stream.consume(&TokenKind::RParen) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: "Expected ')' in voltage target specification".to_string(),
            });
        }
        return Ok(Some(node));
    }

    Ok(Some(expect_node(stream, line_num)?))
}
