use super::*;

pub(super) fn parse_step_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<StepCommand, ParseError> {
    skip_commas(stream);

    // Check for sweep type prefix
    let first = expect_ident(stream, line_num)?;
    let first_upper = first.to_uppercase();

    let (sweep_prefix, target_str, name) = match first_upper.as_str() {
        "DEC" | "OCT" | "LIN" => {
            let target = expect_ident(stream, line_num)?;
            let target_upper = target.to_uppercase();
            let name = if target_upper == "TEMP" {
                "TEMP".to_string()
            } else {
                expect_ident(stream, line_num)?
            };
            (Some(first_upper), target_upper, name)
        }
        "PARAM" | "MODEL" => {
            let name = expect_ident(stream, line_num)?;
            (None, first_upper, name)
        }
        "TEMP" => (None, first_upper, "TEMP".to_string()),
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
    match target {
        StepTarget::Model => {
            param_name = Some(expect_ident(stream, line_num)?);
        }
        StepTarget::Device => {
            // Optional device-parameter spec:
            // - .STEP R1(<param>) ...
            // - .STEP R1 <param> ...
            if stream.consume(&TokenKind::LParen) {
                let pname = expect_ident(stream, line_num)?;
                if !stream.consume(&TokenKind::RParen) {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: "Expected ')' after .STEP device parameter name".to_string(),
                    });
                }
                param_name = Some(pname);
            } else if let TokenKind::Ident(candidate) = &stream.peek().kind {
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
                points_per_decade: step_or_points as usize,
                start,
                stop,
            },
            Some("OCT") => StepSweep::Octave {
                points_per_octave: step_or_points as usize,
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
        if let TokenKind::Ident(s) = &stream.peek().kind {
            outputs.push(s.clone());
            stream.advance();
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

/// Parse .NOISE command: .NOISE V(out[,ref]) Vsource DEC|LIN|OCT np fstart fstop
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
        _ => FreqVariation::Dec, // Default
    };

    let points = expect_value(stream, line_num, params)? as usize;
    let start_freq = expect_value(stream, line_num, params)?;
    let stop_freq = expect_value(stream, line_num, params)?;

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

/// Parse .SENS command: .SENS V(out[,ref]) [AC DEC|LIN|OCT np fstart fstop]
pub(super) fn parse_sens_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<AnalysisCommand, ParseError> {
    let (output_node, reference_node) = parse_voltage_output_reference(stream, line_num)?;
    let mut ac_sweep = None;

    if !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        let mode = expect_ident(stream, line_num)?;
        if !mode.eq_ignore_ascii_case("AC") {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Invalid .SENS mode '{}': expected AC or end-of-line", mode),
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
        ac_sweep,
    })
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
        .trim_start_matches(|c: char| c == 'V' || c == 'v')
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
        let voltage = expect_value(stream, line_num, params)?;
        node_sets.push(NodeSet { node, voltage });
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
        let voltage = expect_value(stream, line_num, params)?;
        initial_conditions.push(InitialCondition { node, voltage });
    }

    Ok(())
}

fn parse_voltage_hint_target(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<Option<String>, ParseError> {
    skip_commas(stream);
    if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        return Ok(None);
    }

    if let TokenKind::Ident(ident) = &stream.peek().kind {
        if ident.eq_ignore_ascii_case("V") && matches!(stream.peek_n(1).kind, TokenKind::LParen) {
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
    }

    Ok(Some(expect_node(stream, line_num)?))
}
