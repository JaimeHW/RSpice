//! SPICE netlist parser using token-based parsing
//!
//! Parses standard SPICE netlist format with extensions including:
//! - Sloppy syntax (commas, trailing parameters)
//! - PULSE/SIN/PWL/EXP source specifications with parentheses
//! - .PARAM statements with expression evaluation
//! - Subcircuit definitions and instances

use super::expr::eval_expression;
use super::lexer::{LexError, TokenKind, TokenStream, tokenize};
use super::xspice_parser;
use super::{
    AnalysisCommand, Element, ElementKind, FreqVariation, InitialCondition, ModelDef,
    MonteCarloCommand, MonteCarloDistribution, Netlist, ParamContext, ParseError,
    PoleZeroAnalysisType, PoleZeroTransferType, SensitivityAcSweep, SourceSpec, StepCommand,
    StepSweep, StepTarget, SubcircuitDef, SwitchState, VerilogAInclude,
};
use crate::Value;

//=============================================================================
// Main Parser
//=============================================================================

/// Parse a complete netlist from string
pub fn parse_netlist(input: &str) -> Result<Netlist, ParseError> {
    let lines: Vec<&str> = input.lines().collect();

    if lines.is_empty() {
        return Ok(Netlist::default());
    }

    // First line is the title
    let title = lines[0].to_string();

    let mut elements = Vec::new();
    let mut analyses = Vec::new();
    let mut models = Vec::new();
    let mut subcircuits = Vec::new();
    let mut params = ParamContext::new();
    let mut veriloga_includes = Vec::new();
    let mut measurements = Vec::new();
    let mut options = super::SimulationOptions::default();

    // State for tracking subcircuit blocks
    let mut in_subcircuit = false;
    let mut current_subckt: Option<SubcircuitDef> = None;

    let mut line_num = 1;
    let mut continuation = String::new();

    for line in lines.iter().skip(1) {
        line_num += 1;

        // Strip inline ';' comments (common SPICE syntax), then trim.
        // We intentionally keep this simple and treat ';' as comment start.
        // This matches common model-card usage where ';' appears outside quotes.
        let no_inline_comment = strip_inline_semicolon_comment(line);
        let trimmed = no_inline_comment.trim();
        if trimmed.is_empty() || trimmed.starts_with('*') {
            continue;
        }

        // Handle line continuation (+ at start of line)
        if trimmed.starts_with('+') {
            continuation.push(' ');
            continuation.push_str(&trimmed[1..]);
            continue;
        }

        // Process previous continued line if exists
        if !continuation.is_empty() {
            process_line(
                &continuation,
                line_num - 1,
                &mut elements,
                &mut analyses,
                &mut models,
                &mut subcircuits,
                &mut in_subcircuit,
                &mut current_subckt,
                &mut params,
                &mut measurements,
                &mut options,
            )?;
            continuation.clear();
        }

        // Check for .END
        if trimmed.eq_ignore_ascii_case(".end") {
            break;
        }

        // Handle .VERILOGA directive directly (before continuation handling)
        let upper_trimmed = trimmed.to_uppercase();
        if upper_trimmed.starts_with(".VERILOGA") || upper_trimmed.starts_with(".VA") {
            // Parse: .VERILOGA filename.va [MODELNAME]
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                let file_path = std::path::PathBuf::from(parts[1]);
                let model_name = parts.get(2).map(|s| s.to_string());
                veriloga_includes.push(VerilogAInclude {
                    file_path,
                    model_name,
                });
                log::debug!("Found .VERILOGA include: {:?}", parts[1]);
                continue; // Skip normal processing
            }
        }

        // Start new continuation or process line
        continuation = trimmed.to_string();
    }

    // Process final line
    if !continuation.is_empty() {
        process_line(
            &continuation,
            line_num,
            &mut elements,
            &mut analyses,
            &mut models,
            &mut subcircuits,
            &mut in_subcircuit,
            &mut current_subckt,
            &mut params,
            &mut measurements,
            &mut options,
        )?;
    }

    // Extract initial conditions from params (they were stored as IC_nodename)
    let initial_conditions: Vec<InitialCondition> = params
        .all_params()
        .iter()
        .filter(|(k, _)| k.starts_with("IC_"))
        .map(|(k, v)| InitialCondition {
            node: k.strip_prefix("IC_").unwrap().to_string(),
            voltage: *v,
        })
        .collect();

    Ok(Netlist {
        title,
        elements,
        analyses,
        models,
        subcircuits,
        params,
        initial_conditions,
        global_nodes: std::collections::HashSet::new(),
        measurements,
        options,
        veriloga_includes,
        source_text: Some(input.to_string()),
    })
}

fn strip_inline_semicolon_comment(line: &str) -> &str {
    match line.find(';') {
        Some(idx) => &line[..idx],
        None => line,
    }
}

fn process_line(
    line: &str,
    line_num: usize,
    elements: &mut Vec<Element>,
    analyses: &mut Vec<AnalysisCommand>,
    models: &mut Vec<ModelDef>,
    subcircuits: &mut Vec<SubcircuitDef>,
    in_subcircuit: &mut bool,
    current_subckt: &mut Option<SubcircuitDef>,
    params: &mut ParamContext,
    measurements: &mut Vec<crate::analysis::MeasureStatement>,
    options: &mut super::SimulationOptions,
) -> Result<(), ParseError> {
    let upper = line.to_uppercase();

    // Check for .SUBCKT start
    if upper.starts_with(".SUBCKT") {
        let subckt = parse_subckt_def(line, line_num)?;
        *in_subcircuit = true;
        *current_subckt = Some(subckt);
        return Ok(());
    }

    // Check for .ENDS
    if upper.starts_with(".ENDS") {
        if let Some(subckt) = current_subckt.take() {
            subcircuits.push(subckt);
        }
        *in_subcircuit = false;
        return Ok(());
    }

    // If inside subcircuit, add elements to subcircuit
    if *in_subcircuit {
        if let Some(subckt) = current_subckt {
            let mut subckt_elements = Vec::new();

            // Create a merged parameter context with subcircuit's default parameters
            // This allows expressions like 'gold' to reference subcircuit params
            let mut subckt_params = params.clone();
            for (name, value) in &subckt.params {
                subckt_params.set(name, *value);
            }

            // Subcircuits don't get standalone measurements parsing
            let mut dummy_measurements = Vec::new();
            parse_line(
                line,
                line_num,
                &mut subckt_elements,
                analyses,
                models,
                &mut subckt_params,
                &mut dummy_measurements,
                options,
            )?;
            subckt.elements.extend(subckt_elements);
        }
        return Ok(());
    }

    // Normal element/command parsing
    parse_line(
        line,
        line_num,
        elements,
        analyses,
        models,
        params,
        measurements,
        options,
    )
}

fn parse_line(
    line: &str,
    line_num: usize,
    elements: &mut Vec<Element>,
    analyses: &mut Vec<AnalysisCommand>,
    models: &mut Vec<ModelDef>,
    params: &mut ParamContext,
    measurements: &mut Vec<crate::analysis::MeasureStatement>,
    options: &mut super::SimulationOptions,
) -> Result<(), ParseError> {
    // Tokenize the line
    let tokens = tokenize(line).map_err(|e| lex_to_parse_error(e, line_num))?;
    let mut stream = TokenStream::new(tokens);

    // Skip leading whitespace/newlines
    stream.skip_newlines();

    if stream.is_eof() {
        return Ok(());
    }

    let first = match &stream.peek().kind {
        TokenKind::Ident(s) => s.clone(),
        _ => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: "Expected identifier at start of line".to_string(),
            });
        }
    };

    let first_char = first.chars().next().unwrap_or(' ');

    match first_char {
        '.' => parse_command(
            &mut stream,
            line_num,
            analyses,
            models,
            params,
            measurements,
            options,
        ),
        'R' => parse_resistor(&mut stream, line_num, elements, params),
        'C' => parse_capacitor(&mut stream, line_num, elements, params),
        'L' => parse_inductor(&mut stream, line_num, elements, params),
        'V' => parse_voltage_source(&mut stream, line_num, elements, params),
        'I' => parse_current_source(&mut stream, line_num, elements, params),
        'D' => parse_diode(&mut stream, line_num, elements),
        'Q' => parse_bjt(&mut stream, line_num, elements),
        'M' => parse_mosfet(&mut stream, line_num, elements),
        'J' => parse_jfet(&mut stream, line_num, elements),
        'X' => parse_subcircuit_instance(&mut stream, line_num, elements),
        'E' => parse_vcvs(&mut stream, line_num, elements, params),
        'F' => parse_cccs(&mut stream, line_num, elements, params),
        'G' => parse_vccs(&mut stream, line_num, elements, params),
        'H' => parse_ccvs(&mut stream, line_num, elements, params),
        'B' => parse_behavioral(&mut stream, line_num, elements),
        // Coupling and switches
        'K' => parse_coupling(&mut stream, line_num, elements, params),
        'S' => parse_vswitch(&mut stream, line_num, elements),
        'W' => parse_iswitch(&mut stream, line_num, elements),
        // Transmission lines
        'T' => parse_transmission_line(&mut stream, line_num, elements, params),
        'O' => parse_lossless_tline(&mut stream, line_num, elements, params),
        'Y' => parse_lossy_tline(&mut stream, line_num, elements, params),
        'P' => parse_coupled_tlines(&mut stream, line_num, elements),
        // MESFET (Z element) - treat like JFET with model
        'Z' => parse_mesfet(&mut stream, line_num, elements),
        // XSPICE code model instance
        'A' => {
            let name = expect_ident(&mut stream, line_num)?;
            xspice_parser::parse_xspice(&mut stream, line_num, name, elements)
        }
        _ => Err(ParseError::Syntax {
            line: line_num,
            message: format!("Unknown element type: {}", first_char),
        }),
    }
}

//=============================================================================
// Command Parsing
//=============================================================================

fn parse_command(
    stream: &mut TokenStream,
    line_num: usize,
    analyses: &mut Vec<AnalysisCommand>,
    models: &mut Vec<ModelDef>,
    params: &mut ParamContext,
    measurements: &mut Vec<crate::analysis::MeasureStatement>,
    options: &mut super::SimulationOptions,
) -> Result<(), ParseError> {
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
            let name = expect_ident(stream, line_num)?;
            let model_type = expect_ident(stream, line_num)?;
            let model_params = parse_model_params(stream, params)?;

            models.push(ModelDef {
                name,
                model_type,
                params: model_params,
            });
        }
        ".PARAM" => {
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
            // Parse initial conditions - stored as params for now
            parse_ic_command(stream, line_num, params)?;
        }
        ".NODESET" => {
            // Parse nodeset hints - stored as params for now
            parse_nodeset_command(stream, line_num, params)?;
        }
        ".INCLUDE" | ".INC" => {
            // Include directives are handled in a preprocessing pass
            log::debug!("Include directive found: line {}", line_num);
        }
        ".LIB" => {
            // Library directives are handled in a preprocessing pass
            log::debug!("Library directive found: line {}", line_num);
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
            if let Ok(meas) = parse_meas_command(stream, line_num, params) {
                measurements.push(meas);
            }
        }
        _ => {
            // Ignore unknown commands
            log::debug!("Ignoring unknown command: {}", cmd);
        }
    }

    Ok(())
}

fn parse_options_command(
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

fn parse_usize_option(name: &str, value: Value, line_num: usize) -> Result<usize, ParseError> {
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

/// Parse .MEAS/.MEASURE statement
/// Syntax: .MEAS TRAN name TYPE signal [FROM=x TO=y]
/// Examples:
///   .MEAS TRAN vmax MAX V(out)
///   .MEAS TRAN vavg AVG V(out) FROM=0 TO=1m
fn parse_meas_command(
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

    // Parse signal name - handle V(node) or just node
    let mut signal = expect_ident(stream, line_num)?;

    // Check if it's a function-like signal e.g. V(out)
    if stream.consume(&TokenKind::LParen) {
        let inner = match &stream.peek().kind {
            TokenKind::Ident(s) => s.clone(),
            TokenKind::Number(v) => format!("{}", v), // Allow V(1)
            _ => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: "Expected identifier or number inside signal parentheses".to_string(),
                });
            }
        };
        stream.advance();

        if !stream.consume(&TokenKind::RParen) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: "Expected closing parenthesis for signal".to_string(),
            });
        }
        signal = format!("{}({})", signal, inner);
    }

    // Parse optional FROM/TO
    let mut from: Option<crate::Value> = None;
    let mut to: Option<crate::Value> = None;

    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        if let TokenKind::Ident(s) = &stream.peek().kind {
            let key = s.to_uppercase();
            stream.advance();
            if stream.consume(&TokenKind::Equals) {
                if let Ok(val) = expect_value(stream, line_num, params) {
                    match key.as_str() {
                        "FROM" => from = Some(val),
                        "TO" => to = Some(val),
                        _ => {}
                    }
                }
            }
        } else {
            stream.advance();
        }
    }

    // Create the measurement type based on keyword
    let measure_type = match measure_type_str.as_str() {
        "AVG" => MeasureType::Avg {
            signal: signal.clone(),
            from,
            to,
        },
        "MAX" => MeasureType::Max {
            signal: signal.clone(),
            from,
            to,
        },
        "MIN" => MeasureType::Min {
            signal: signal.clone(),
            from,
            to,
        },
        "PP" => MeasureType::PeakToPeak {
            signal: signal.clone(),
            from,
            to,
        },
        "RMS" => MeasureType::Rms {
            signal: signal.clone(),
            from,
            to,
        },
        "INTEG" => MeasureType::Integ {
            signal: signal.clone(),
            from,
            to,
        },
        _ => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Unknown measurement type: {}", measure_type_str),
            });
        }
    };

    Ok(MeasureStatement {
        name,
        measure_type,
        analysis,
    })
}

fn parse_param_statement(
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
fn parse_func_statement(
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

//=============================================================================
// Element Parsing
//=============================================================================

fn parse_resistor(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let node_pos = expect_node(stream, line_num)?;
    let node_neg = expect_node(stream, line_num)?;

    // Skip optional parameter names (R=)
    skip_optional_param_name(stream, "R");

    // Try to parse as a value first. If next token is an identifier
    // that isn't a parameter, it's a model name.
    let value = match &stream.peek().kind {
        TokenKind::Number(v) => {
            let v = *v;
            stream.advance();
            v
        }
        TokenKind::Ident(s) => {
            // Check if it's a parameter reference
            if let Some(v) = params.get(s) {
                stream.advance();
                v
            } else {
                // It's a model name - skip it and remaining parameters
                // For model-based resistors, use a placeholder value
                // Real value comes from .MODEL and geometry (L, W)
                stream.advance(); // Skip model name

                // Skip any geometry/additional params (L=, W=, AC=, etc.)
                while !stream.is_eof()
                    && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof)
                {
                    skip_commas(stream);
                    // Skip parameter assignments like L=11u
                    if let TokenKind::Ident(_) = &stream.peek().kind {
                        stream.advance();
                        if stream.consume(&TokenKind::Equals) {
                            // Consume the value
                            if let TokenKind::Number(_) = &stream.peek().kind {
                                stream.advance();
                            } else if let TokenKind::Ident(_) = &stream.peek().kind {
                                stream.advance();
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                // Use a default 1k value for model-based resistors
                // The actual simulation would need to look up the model
                1000.0
            }
        }
        TokenKind::Plus | TokenKind::Minus => {
            // Handle signed values
            let sign = if matches!(stream.peek().kind, TokenKind::Minus) {
                -1.0
            } else {
                1.0
            };
            stream.advance();
            if let TokenKind::Number(v) = &stream.peek().kind {
                let v = *v * sign;
                stream.advance();
                v
            } else {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: "Expected value after sign".to_string(),
                });
            }
        }
        TokenKind::Expression(expr) => {
            // Handle expression values like {R}
            let expr = expr.clone();
            stream.advance();
            eval_expression(&expr, params).map_err(|e| ParseError::InvalidValue(e.to_string()))?
        }
        _ => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    "Expected resistor value or model, found {:?}",
                    stream.peek().kind
                ),
            });
        }
    };

    elements.push(Element {
        name,
        kind: ElementKind::Resistor { value },
        nodes: vec![node_pos, node_neg],
    });

    Ok(())
}

fn parse_capacitor(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let node_pos = expect_node(stream, line_num)?;
    let node_neg = expect_node(stream, line_num)?;

    skip_optional_param_name(stream, "C");
    let value = expect_value(stream, line_num, params)?;
    let initial_voltage = try_value_with_param(stream, params, "IC");

    elements.push(Element {
        name,
        kind: ElementKind::Capacitor {
            value,
            initial_voltage,
        },
        nodes: vec![node_pos, node_neg],
    });

    Ok(())
}

fn parse_inductor(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let node_pos = expect_node(stream, line_num)?;
    let node_neg = expect_node(stream, line_num)?;

    skip_optional_param_name(stream, "L");
    let value = expect_value(stream, line_num, params)?;
    let initial_current = try_value_with_param(stream, params, "IC");

    // Check for MODEL parameter (indicates Jiles-Atherton or other nonlinear inductor)
    let model = try_string_with_param(stream, "MODEL");

    let kind = if let Some(model_name) = model {
        ElementKind::JilesAthertonInductor {
            value,
            model: model_name,
            initial_current,
        }
    } else {
        ElementKind::Inductor {
            value,
            initial_current,
        }
    };

    elements.push(Element {
        name,
        kind,
        nodes: vec![node_pos, node_neg],
    });

    Ok(())
}

fn parse_voltage_source(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let node_pos = expect_node(stream, line_num)?;
    let node_neg = expect_node(stream, line_num)?;

    let source_spec = parse_source_spec(stream, line_num, params)?;

    elements.push(Element {
        name,
        kind: ElementKind::VoltageSource(source_spec),
        nodes: vec![node_pos, node_neg],
    });

    Ok(())
}

fn parse_current_source(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let node_pos = expect_node(stream, line_num)?;
    let node_neg = expect_node(stream, line_num)?;

    let source_spec = parse_source_spec(stream, line_num, params)?;

    elements.push(Element {
        name,
        kind: ElementKind::CurrentSource(source_spec),
        nodes: vec![node_pos, node_neg],
    });

    Ok(())
}

fn parse_diode(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let anode = expect_node(stream, line_num)?;
    let cathode = expect_node(stream, line_num)?;
    let model = expect_ident(stream, line_num)?;

    elements.push(Element {
        name,
        kind: ElementKind::Diode { model },
        nodes: vec![anode, cathode],
    });

    Ok(())
}

fn parse_bjt(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let collector = expect_node(stream, line_num)?;
    let base = expect_node(stream, line_num)?;
    let emitter = expect_node(stream, line_num)?;

    // BJT can have optional substrate node: Q1 C B E [S] model
    // We need to peek ahead to determine if next is substrate or model
    let (substrate, model) = match &stream.peek().kind {
        TokenKind::Number(_) => {
            // It's a numeric node (substrate like "0")
            let substrate = expect_node(stream, line_num)?;
            let model = expect_ident(stream, line_num)?;
            (Some(substrate), model)
        }
        TokenKind::Ident(s) => {
            // Check if there's another identifier after this one
            // by looking if what follows looks like a model name
            let first_ident = s.clone();
            stream.advance();

            // Now peek at next token
            match &stream.peek().kind {
                TokenKind::Ident(next_s) => {
                    // Two identifiers in a row - BUT need to check if second is a parameter name
                    // If the token AFTER the second ident is '=', then second is a param name
                    // and first_ident is the model name (not substrate)
                    let next_ident = next_s.clone();

                    // Peek ahead: is there an '=' after the next ident?
                    // stream.peek_n(1) would be the token after the current peek
                    if matches!(stream.peek_n(1).kind, TokenKind::Equals) {
                        // Pattern: model_name param=value
                        // first_ident is the model, don't treat next_ident as model
                        (None, first_ident)
                    } else {
                        // Pattern: substrate model_name
                        // first is substrate node, second is model
                        stream.advance();
                        (Some(first_ident), next_ident)
                    }
                }
                TokenKind::Newline | TokenKind::Eof | TokenKind::Comma => {
                    // Only one identifier: it's the model name
                    (None, first_ident)
                }
                _ => {
                    // Assume first_ident is the model, any params follow
                    (None, first_ident)
                }
            }
        }
        _ => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Expected BJT model name, found {:?}", stream.peek().kind),
            });
        }
    };

    let mut nodes = vec![collector, base, emitter];
    if let Some(sub) = substrate {
        nodes.push(sub);
    }

    elements.push(Element {
        name,
        kind: ElementKind::Bjt {
            model,
            bjt_type: super::BjtType::Npn, // Will be set from model
        },
        nodes,
    });

    Ok(())
}

fn parse_mosfet(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let drain = expect_node(stream, line_num)?;
    let gate = expect_node(stream, line_num)?;
    let source = expect_node(stream, line_num)?;
    let bulk = expect_node(stream, line_num)?;
    let model = expect_ident(stream, line_num)?;

    elements.push(Element {
        name,
        kind: ElementKind::Mosfet {
            model,
            mos_type: super::MosType::Nmos, // Will be set from model
        },
        nodes: vec![drain, gate, source, bulk],
    });

    Ok(())
}

fn parse_jfet(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let drain = expect_node(stream, line_num)?;
    let gate = expect_node(stream, line_num)?;
    let source = expect_node(stream, line_num)?;
    let model = expect_ident(stream, line_num)?;

    elements.push(Element {
        name,
        kind: ElementKind::Jfet {
            model,
            jfet_type: super::JfetType::Njf, // Will be set from model
        },
        nodes: vec![drain, gate, source],
    });

    Ok(())
}

/// Parse MESFET (Z element) - GaAs MESFET transistors
fn parse_mesfet(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let drain = expect_node(stream, line_num)?;
    let gate = expect_node(stream, line_num)?;
    let source = expect_node(stream, line_num)?;
    let model = expect_ident(stream, line_num)?;

    elements.push(Element {
        name,
        kind: ElementKind::Mesfet {
            model,
            mesfet_type: super::MesfetType::Nmf, // Will be set from model
        },
        nodes: vec![drain, gate, source],
    });

    Ok(())
}

/// Parse lossless transmission line (O element)
fn parse_lossless_tline(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let port1_pos = expect_node(stream, line_num)?;
    let port1_neg = expect_node(stream, line_num)?;
    let port2_pos = expect_node(stream, line_num)?;
    let port2_neg = expect_node(stream, line_num)?;

    let parsed = parse_tline_params(stream, line_num, params, true)?;
    if parsed.model.is_none() && parsed.z0.is_none() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "O-line transmission line requires MODEL name or Z0".to_string(),
        });
    }

    elements.push(Element {
        name,
        kind: ElementKind::TransmissionLine {
            z0: parsed.z0,
            td: parsed.td,
            freq: parsed.freq,
            nl: parsed.nl,
            model: parsed.model,
        },
        nodes: vec![port1_pos, port1_neg, port2_pos, port2_neg],
    });

    Ok(())
}

/// Parse lossy transmission line (Y element)
fn parse_lossy_tline(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let port1_pos = expect_node(stream, line_num)?;
    let port1_neg = expect_node(stream, line_num)?;
    let port2_pos = expect_node(stream, line_num)?;
    let port2_neg = expect_node(stream, line_num)?;

    let parsed = parse_tline_params(stream, line_num, params, true)?;
    if parsed.model.is_none() && parsed.z0.is_none() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "Y-line transmission line requires MODEL name or Z0".to_string(),
        });
    }

    elements.push(Element {
        name,
        kind: ElementKind::TransmissionLine {
            z0: parsed.z0,
            td: parsed.td,
            freq: parsed.freq,
            nl: parsed.nl,
            model: parsed.model,
        },
        nodes: vec![port1_pos, port1_neg, port2_pos, port2_neg],
    });

    Ok(())
}

/// Parse coupled transmission lines (P element)
fn parse_coupled_tlines(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;

    // Coupled lines have more nodes - collect them all
    let mut nodes = Vec::new();
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        skip_commas(stream);
        if let Ok(node) = expect_node(stream, line_num) {
            nodes.push(node);
        } else {
            break;
        }
    }

    // In P-line syntax the final token is typically the model name.
    let model = if let Some(last) = nodes.last() {
        if nodes.len() >= 3 && last.parse::<f64>().is_err() {
            nodes.pop()
        } else {
            None
        }
    } else {
        None
    };

    elements.push(Element {
        name,
        kind: ElementKind::TransmissionLine {
            z0: None,
            td: None,
            freq: None,
            nl: None,
            model,
        },
        nodes,
    });

    Ok(())
}

/// Parse subcircuit instance: X1 node1 node2... SUBCKTNAME [PARAM=val ...]
fn parse_subcircuit_instance(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;

    // Collect nodes until we hit a non-node identifier (the subcircuit name)
    let mut nodes = Vec::new();
    let mut subckt_name = String::new();

    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        skip_commas(stream);

        if stream.is_eof() || matches!(stream.peek().kind, TokenKind::Newline) {
            break;
        }

        // If we see an equals sign ahead, stop collecting nodes
        if matches!(stream.peek_n(1).kind, TokenKind::Equals) {
            break;
        }

        let node_or_name = expect_node(stream, line_num)?;

        // The last identifier before any parameters is the subcircuit name
        if !subckt_name.is_empty() {
            nodes.push(subckt_name);
        }
        subckt_name = node_or_name;
    }

    if subckt_name.is_empty() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "Subcircuit instance requires name and subcircuit reference".to_string(),
        });
    }

    // Parse instance parameters: PARAM=value pairs
    let mut params = Vec::new();
    let params_ctx = ParamContext::new();

    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        skip_commas(stream);

        if stream.is_eof() || matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }

        // Skip PARAMS: keyword if present
        if let TokenKind::Ident(s) = &stream.peek().kind {
            let upper = s.to_uppercase();
            if upper == "PARAMS" || upper == "PARAMS:" {
                stream.advance();
                continue;
            }
        }

        if let TokenKind::Ident(param_name) = &stream.peek().kind {
            let param_name = param_name.clone();
            stream.advance();

            if stream.consume(&TokenKind::Equals) {
                if let Some(value) = try_value(stream, &params_ctx) {
                    params.push((param_name, value));
                }
            }
        } else {
            stream.advance(); // Skip unknown token
        }
    }

    elements.push(Element {
        name,
        kind: ElementKind::Subcircuit {
            subckt_name,
            params,
        },
        nodes,
    });

    Ok(())
}

fn parse_vcvs(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let node_pos = expect_node(stream, line_num)?;
    let node_neg = expect_node(stream, line_num)?;
    let ctrl_pos = expect_node(stream, line_num)?;
    let ctrl_neg = expect_node(stream, line_num)?;
    let gain = expect_value(stream, line_num, params)?;

    elements.push(Element {
        name,
        kind: ElementKind::Vcvs {
            gain,
            control_nodes: (ctrl_pos, ctrl_neg),
        },
        nodes: vec![node_pos, node_neg],
    });

    Ok(())
}

fn parse_cccs(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let node_pos = expect_node(stream, line_num)?;
    let node_neg = expect_node(stream, line_num)?;
    let control_element = expect_ident(stream, line_num)?;
    let gain = expect_value(stream, line_num, params)?;

    elements.push(Element {
        name,
        kind: ElementKind::Cccs {
            gain,
            control_element,
        },
        nodes: vec![node_pos, node_neg],
    });

    Ok(())
}

fn parse_vccs(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let node_pos = expect_node(stream, line_num)?;
    let node_neg = expect_node(stream, line_num)?;
    let ctrl_pos = expect_node(stream, line_num)?;
    let ctrl_neg = expect_node(stream, line_num)?;
    let transconductance = expect_value(stream, line_num, params)?;

    elements.push(Element {
        name,
        kind: ElementKind::Vccs {
            transconductance,
            control_nodes: (ctrl_pos, ctrl_neg),
        },
        nodes: vec![node_pos, node_neg],
    });

    Ok(())
}

fn parse_ccvs(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let node_pos = expect_node(stream, line_num)?;
    let node_neg = expect_node(stream, line_num)?;
    let control_element = expect_ident(stream, line_num)?;
    let transresistance = expect_value(stream, line_num, params)?;

    elements.push(Element {
        name,
        kind: ElementKind::Ccvs {
            transresistance,
            control_element,
        },
        nodes: vec![node_pos, node_neg],
    });

    Ok(())
}

fn parse_behavioral(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let node_pos = expect_node(stream, line_num)?;
    let node_neg = expect_node(stream, line_num)?;

    // Look for V= or I=
    let spec = expect_ident(stream, line_num)?;

    // Consume = if present
    stream.consume(&TokenKind::Equals);

    // The rest is the expression
    let mut expr_parts = Vec::new();
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        expr_parts.push(format!("{}", stream.peek().kind));
        stream.advance();
    }
    let expression = expr_parts.join(" ");

    let kind = if spec.starts_with('V') {
        let expr_content = if spec.len() > 1 && spec.starts_with("V=") {
            format!("{}{}", &spec[2..], expression)
        } else {
            expression
        };
        ElementKind::BehavioralVoltage {
            expression: expr_content,
        }
    } else if spec.starts_with('I') {
        let expr_content = if spec.len() > 1 && spec.starts_with("I=") {
            format!("{}{}", &spec[2..], expression)
        } else {
            expression
        };
        ElementKind::BehavioralCurrent {
            expression: expr_content,
        }
    } else {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "Behavioral source must have V=expr or I=expr".to_string(),
        });
    };

    elements.push(Element {
        name,
        kind,
        nodes: vec![node_pos, node_neg],
    });

    Ok(())
}

//=============================================================================
// Source Specification Parsing
//=============================================================================

/// Parse source specification (DC, AC, PULSE, SIN, PWL, EXP)
///
/// Supports combined DC+AC syntax: "DC 0 AC 1" or "DC 5 AC 1 45"
fn parse_source_spec(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<SourceSpec, ParseError> {
    skip_commas(stream);

    // Standard SPICE behavior: missing source spec defaults to DC 0
    if stream.is_eof() || matches!(stream.peek().kind, TokenKind::Newline) {
        return Ok(SourceSpec::Dc(0.0));
    }

    // Check for keywords
    match &stream.peek().kind {
        TokenKind::Ident(s) => {
            let upper = s.to_uppercase();
            match upper.as_str() {
                "DC" => {
                    stream.advance();
                    // Allow optional = after DC (e.g., "dc = 5" or "dc 5")
                    skip_commas(stream);
                    stream.consume(&TokenKind::Equals);
                    let dc_value = expect_value(stream, line_num, params)?;

                    // Check for optional AC specification after DC
                    skip_commas(stream);
                    if let TokenKind::Ident(next) = &stream.peek().kind {
                        if next.to_uppercase() == "AC" {
                            stream.advance();
                            let ac_magnitude = try_value(stream, params).unwrap_or(1.0);
                            // SPICE AC phase is specified in degrees; store radians internally.
                            let ac_phase = try_value(stream, params).unwrap_or(0.0).to_radians();
                            return Ok(SourceSpec::DcAc {
                                dc_value,
                                ac_magnitude,
                                ac_phase,
                            });
                        }
                    }
                    return Ok(SourceSpec::Dc(dc_value));
                }
                "AC" => {
                    stream.advance();
                    // AC magnitude is optional - defaults to 1.0 if not specified
                    let magnitude = try_value(stream, params).unwrap_or(1.0);
                    // SPICE AC phase is specified in degrees; store radians internally.
                    let phase = try_value(stream, params).unwrap_or(0.0).to_radians();
                    return Ok(SourceSpec::Ac { magnitude, phase });
                }
                "PULSE" => {
                    stream.advance();
                    return parse_pulse_spec(stream, line_num, params);
                }
                "SIN" => {
                    stream.advance();
                    return parse_sin_spec(stream, line_num, params);
                }
                "PWL" => {
                    stream.advance();
                    return parse_pwl_spec(stream, line_num, params);
                }
                "EXP" => {
                    stream.advance();
                    return parse_exp_spec(stream, line_num, params);
                }
                _ => {}
            }
        }
        _ => {}
    }

    // Default: try to parse as DC value
    let value = expect_value(stream, line_num, params)?;
    Ok(SourceSpec::Dc(value))
}

fn parse_pulse_spec(
    stream: &mut TokenStream,
    _line_num: usize,
    params: &ParamContext,
) -> Result<SourceSpec, ParseError> {
    // Consume opening paren if present
    let has_paren = stream.consume(&TokenKind::LParen);

    let v1 = expect_value_default(stream, params, 0.0);
    let v2 = expect_value_default(stream, params, 1.0);
    let delay = expect_value_default(stream, params, 0.0);
    let rise = expect_value_default(stream, params, 1e-9);
    let fall = expect_value_default(stream, params, 1e-9);
    let width = expect_value_default(stream, params, 1e-6);
    let period = expect_value_default(stream, params, 2e-6);

    if has_paren {
        stream.consume(&TokenKind::RParen);
    }

    Ok(SourceSpec::Pulse {
        v1,
        v2,
        delay,
        rise,
        fall,
        width,
        period,
    })
}

fn parse_sin_spec(
    stream: &mut TokenStream,
    _line_num: usize,
    params: &ParamContext,
) -> Result<SourceSpec, ParseError> {
    let has_paren = stream.consume(&TokenKind::LParen);

    let offset = expect_value_default(stream, params, 0.0);
    let amplitude = expect_value_default(stream, params, 1.0);
    let frequency = expect_value_default(stream, params, 1e3);
    let delay = expect_value_default(stream, params, 0.0);
    let damping = expect_value_default(stream, params, 0.0);
    // SPICE SIN phase is specified in degrees; store radians internally.
    let phase = expect_value_default(stream, params, 0.0).to_radians();

    if has_paren {
        stream.consume(&TokenKind::RParen);
    }

    Ok(SourceSpec::Sin {
        offset,
        amplitude,
        frequency,
        delay,
        damping,
        phase,
    })
}

fn parse_pwl_spec(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<SourceSpec, ParseError> {
    let has_paren = stream.consume(&TokenKind::LParen);

    // PWL FILE="path" [TSCALE=..] [VSCALE=..] [TOFFSET=..] [VOFFSET=..]
    if let TokenKind::Ident(s) = &stream.peek().kind {
        if s.eq_ignore_ascii_case("FILE") {
            stream.advance();
            stream.consume(&TokenKind::Equals);

            let path = match &stream.peek().kind {
                TokenKind::StringLit(s) => {
                    let p = s.clone();
                    stream.advance();
                    p
                }
                TokenKind::Ident(s) => {
                    let p = s.clone();
                    stream.advance();
                    p
                }
                _ => {
                    return Err(ParseError::MissingParameter(format!(
                        "PWL filename at line {}",
                        line_num
                    )));
                }
            };

            let mut time_scale = 1.0;
            let mut value_scale = 1.0;
            let mut time_offset = 0.0;
            let mut value_offset = 0.0;

            while let TokenKind::Ident(key) = &stream.peek().kind {
                let key_upper = key.to_uppercase();
                stream.advance();
                stream.consume(&TokenKind::Equals);

                let value = try_value(stream, params).unwrap_or(1.0);
                match key_upper.as_str() {
                    "TSCALE" | "TIMESCALE" => time_scale = value,
                    "VSCALE" | "VALUESCALE" | "SCALE" => value_scale = value,
                    "TOFFSET" | "TIMEOFFSET" | "TD" => time_offset = value,
                    "VOFFSET" | "VALUEOFFSET" | "DC" => value_offset = value,
                    _ => break,
                }
            }

            if has_paren {
                stream.consume(&TokenKind::RParen);
            }

            return Ok(SourceSpec::PwlFile {
                path,
                time_scale,
                value_scale,
                time_offset,
                value_offset,
            });
        }
    }

    let mut points = Vec::new();
    while !stream.is_eof() {
        skip_commas(stream);

        if matches!(
            stream.peek().kind,
            TokenKind::RParen | TokenKind::Newline | TokenKind::Eof
        ) {
            break;
        }

        if let Some(time) = try_value(stream, params) {
            if let Some(value) = try_value(stream, params) {
                points.push((time, value));
            } else {
                break;
            }
        } else {
            break;
        }
    }

    if has_paren {
        stream.consume(&TokenKind::RParen);
    }

    if points.is_empty() {
        points.push((0.0, 0.0));
    }

    Ok(SourceSpec::Pwl { points })
}

fn parse_exp_spec(
    stream: &mut TokenStream,
    _line_num: usize,
    params: &ParamContext,
) -> Result<SourceSpec, ParseError> {
    let has_paren = stream.consume(&TokenKind::LParen);

    let v1 = expect_value_default(stream, params, 0.0);
    let v2 = expect_value_default(stream, params, 1.0);
    let td1 = expect_value_default(stream, params, 0.0);
    let tau1 = expect_value_default(stream, params, 1e-6);
    let td2 = expect_value_default(stream, params, 0.0);
    let tau2 = expect_value_default(stream, params, 1e-6);

    if has_paren {
        stream.consume(&TokenKind::RParen);
    }

    Ok(SourceSpec::Exp {
        v1,
        v2,
        td1,
        tau1,
        td2,
        tau2,
    })
}

//=============================================================================
// Subcircuit Parsing
//=============================================================================

/// Parse subcircuit definition: .SUBCKT name ports [PARAMS: p1=v1 p2=v2] or .SUBCKT name ports p1=v1
fn parse_subckt_def(line: &str, line_num: usize) -> Result<SubcircuitDef, ParseError> {
    let tokens = tokenize(line).map_err(|e| lex_to_parse_error(e, line_num))?;
    let mut stream = TokenStream::new(tokens);

    // Skip .SUBCKT
    stream.advance();

    let name = expect_ident(&mut stream, line_num)?;

    // Collect ports until we hit = (parameter) or PARAMS keyword or end of line
    let mut ports = Vec::new();
    let params_ctx = ParamContext::new();

    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        skip_commas(&mut stream);

        // Check if next token is followed by = (indicating parameter, not port)
        if matches!(stream.peek_n(1).kind, TokenKind::Equals) {
            break;
        }

        // Check for PARAMS: keyword
        if let TokenKind::Ident(s) = &stream.peek().kind {
            let upper = s.to_uppercase();
            if upper == "PARAMS" || upper == "PARAMS:" {
                stream.advance();
                // Consume : if separate
                if let TokenKind::Ident(s2) = &stream.peek().kind {
                    if s2 == ":" {
                        stream.advance();
                    }
                }
                break;
            }
        }

        if stream.is_eof() || matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }

        ports.push(expect_node(&mut stream, line_num)?);
    }

    // Parse default parameters: NAME=VALUE pairs
    let mut params = Vec::new();

    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        skip_commas(&mut stream);

        if stream.is_eof() || matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }

        if let TokenKind::Ident(param_name) = &stream.peek().kind {
            let param_name = param_name.clone();
            stream.advance();

            if stream.consume(&TokenKind::Equals) {
                if let Some(value) = try_value(&mut stream, &params_ctx) {
                    params.push((param_name, value));
                }
            }
        } else {
            stream.advance(); // Skip unknown token
        }
    }

    Ok(SubcircuitDef {
        name,
        ports,
        elements: Vec::new(),
        params,
        local_options: std::collections::HashMap::new(),
        library_ref: None,
        nested_subcircuits: Vec::new(),
    })
}

//=============================================================================
// Model Parameter Parsing
//=============================================================================

fn parse_model_params(
    stream: &mut TokenStream,
    params: &ParamContext,
) -> Result<Vec<(String, Value)>, ParseError> {
    let mut model_params = Vec::new();

    // Skip optional opening paren
    stream.consume(&TokenKind::LParen);

    while !stream.is_eof() {
        skip_commas(stream);

        if matches!(
            stream.peek().kind,
            TokenKind::RParen | TokenKind::Newline | TokenKind::Eof
        ) {
            break;
        }

        // Look for NAME=VALUE
        if let TokenKind::Ident(name) = &stream.peek().kind {
            let name = name.clone();
            stream.advance();

            if stream.consume(&TokenKind::Equals) {
                if let Some(value) = try_value(stream, params) {
                    model_params.push((name, value));
                }
            }
        } else {
            stream.advance(); // Skip unknown token
        }
    }

    // Skip optional closing paren
    stream.consume(&TokenKind::RParen);

    Ok(model_params)
}

//=============================================================================
// Helper Functions
//=============================================================================

fn expect_ident(stream: &mut TokenStream, line_num: usize) -> Result<String, ParseError> {
    skip_commas(stream);

    match &stream.peek().kind {
        TokenKind::Ident(s) => {
            let s = s.clone();
            stream.advance();
            Ok(s)
        }
        other => Err(ParseError::Syntax {
            line: line_num,
            message: format!("Expected identifier, found {:?}", other),
        }),
    }
}

fn expect_node(stream: &mut TokenStream, line_num: usize) -> Result<String, ParseError> {
    skip_commas(stream);

    match &stream.peek().kind {
        TokenKind::Ident(s) => {
            let s = s.clone();
            stream.advance();
            Ok(s)
        }
        TokenKind::Number(n) => {
            // Numeric node name (e.g., "0", "1")
            let s = format!("{}", *n as i64);
            stream.advance();
            Ok(s)
        }
        other => Err(ParseError::Syntax {
            line: line_num,
            message: format!("Expected node name, found {:?}", other),
        }),
    }
}

fn expect_value(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<Value, ParseError> {
    skip_commas(stream);

    // Handle optional sign prefix (+15 or -15)
    let sign = match &stream.peek().kind {
        TokenKind::Plus => {
            stream.advance();
            1.0
        }
        TokenKind::Minus => {
            stream.advance();
            -1.0
        }
        _ => 1.0,
    };

    match &stream.peek().kind {
        TokenKind::Number(v) => {
            let v = *v * sign;
            stream.advance();
            Ok(v)
        }
        TokenKind::Expression(expr) => {
            let expr = expr.clone();
            stream.advance();
            eval_expression(&expr, params)
                .map(|v| v * sign)
                .map_err(|e| ParseError::InvalidValue(e.to_string()))
        }
        TokenKind::Ident(s) => {
            // Could be a parameter reference
            if let Some(v) = params.get(s) {
                stream.advance();
                Ok(v * sign)
            } else {
                Err(ParseError::Syntax {
                    line: line_num,
                    message: format!("Expected value, found identifier '{}'", s),
                })
            }
        }
        other => Err(ParseError::Syntax {
            line: line_num,
            message: format!("Expected value, found {:?}", other),
        }),
    }
}

fn try_value(stream: &mut TokenStream, params: &ParamContext) -> Option<Value> {
    skip_commas(stream);

    match &stream.peek().kind {
        TokenKind::Number(v) => {
            let v = *v;
            stream.advance();
            Some(v)
        }
        TokenKind::Expression(expr) => {
            let expr = expr.clone();
            stream.advance();
            eval_expression(&expr, params).ok()
        }
        TokenKind::Ident(s) => {
            if let Some(v) = params.get(s) {
                stream.advance();
                Some(v)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn expect_value_default(stream: &mut TokenStream, params: &ParamContext, default: Value) -> Value {
    skip_commas(stream);
    try_value(stream, params).unwrap_or(default)
}

fn try_value_with_param(
    stream: &mut TokenStream,
    params: &ParamContext,
    param_name: &str,
) -> Option<Value> {
    skip_commas(stream);

    // Check if next token is the param name followed by =
    if let TokenKind::Ident(s) = &stream.peek().kind {
        if s.eq_ignore_ascii_case(param_name) {
            stream.advance();
            if stream.consume(&TokenKind::Equals) {
                return try_value(stream, params);
            }
        }
    }

    try_value(stream, params)
}

/// Try to consume a named string parameter (e.g., MODEL=name)
fn try_string_with_param(stream: &mut TokenStream, param_name: &str) -> Option<String> {
    skip_commas(stream);

    // Check if next token is the param name followed by =
    if let TokenKind::Ident(s) = &stream.peek().kind {
        if s.eq_ignore_ascii_case(param_name) {
            stream.advance();
            if stream.consume(&TokenKind::Equals) {
                // Get the string value (identifier)
                if let TokenKind::Ident(value) = &stream.peek().kind {
                    let value = value.clone();
                    stream.advance();
                    return Some(value);
                }
            }
        }
    }

    None
}

fn skip_optional_param_name(stream: &mut TokenStream, param_name: &str) {
    if let TokenKind::Ident(s) = &stream.peek().kind {
        if s == param_name {
            stream.advance();
            stream.consume(&TokenKind::Equals);
        }
    }
}

fn skip_commas(stream: &mut TokenStream) {
    while stream.consume(&TokenKind::Comma) {}
}

fn lex_to_parse_error(e: LexError, line_num: usize) -> ParseError {
    ParseError::Syntax {
        line: line_num,
        message: e.to_string(),
    }
}

//=============================================================================
// New Element Type Parsing
//=============================================================================

/// Parse coupling coefficient: K1 L1 L2 [L3...] coefficient
fn parse_coupling(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;

    // Collect inductor names until we hit a number (the coefficient)
    let mut inductors = Vec::new();

    loop {
        skip_commas(stream);

        if stream.is_eof() || matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }

        match &stream.peek().kind {
            TokenKind::Ident(s) if s.starts_with('L') || s.starts_with('l') => {
                inductors.push(s.clone());
                stream.advance();
            }
            TokenKind::Number(_) | TokenKind::Expression(_) => {
                break;
            }
            _ => {
                // Try as inductor name anyway
                if let TokenKind::Ident(s) = &stream.peek().kind {
                    inductors.push(s.clone());
                    stream.advance();
                } else {
                    break;
                }
            }
        }
    }

    if inductors.len() < 2 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "Coupling requires at least two inductors".to_string(),
        });
    }

    let coefficient = expect_value(stream, line_num, params)?;

    // Clamp coefficient to valid range
    let coefficient = coefficient.abs().min(1.0);

    elements.push(Element {
        name,
        kind: ElementKind::Coupling {
            inductors,
            coefficient,
        },
        nodes: vec![], // Coupling doesn't have direct node connections
    });

    Ok(())
}

/// Parse voltage-controlled switch: S1 n+ n- nc+ nc- MODEL [ON|OFF]
fn parse_vswitch(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let node_pos = expect_node(stream, line_num)?;
    let node_neg = expect_node(stream, line_num)?;
    let control_pos = expect_node(stream, line_num)?;
    let control_neg = expect_node(stream, line_num)?;
    let model = expect_ident(stream, line_num)?;

    // Optional initial state
    let initial_state = parse_switch_state(stream);

    elements.push(Element {
        name,
        kind: ElementKind::VSwitch {
            control_pos,
            control_neg,
            model,
            initial_state,
        },
        nodes: vec![node_pos, node_neg],
    });

    Ok(())
}

/// Parse current-controlled switch: W1 n+ n- Vname MODEL [ON|OFF]
fn parse_iswitch(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let node_pos = expect_node(stream, line_num)?;
    let node_neg = expect_node(stream, line_num)?;
    let control_element = expect_ident(stream, line_num)?;
    let model = expect_ident(stream, line_num)?;

    // Optional initial state
    let initial_state = parse_switch_state(stream);

    elements.push(Element {
        name,
        kind: ElementKind::ISwitch {
            control_element,
            model,
            initial_state,
        },
        nodes: vec![node_pos, node_neg],
    });

    Ok(())
}

/// Parse switch initial state (ON/OFF)
fn parse_switch_state(stream: &mut TokenStream) -> Option<SwitchState> {
    skip_commas(stream);

    if let TokenKind::Ident(s) = &stream.peek().kind {
        let upper = s.to_uppercase();
        match upper.as_str() {
            "ON" => {
                stream.advance();
                return Some(SwitchState::On);
            }
            "OFF" => {
                stream.advance();
                return Some(SwitchState::Off);
            }
            _ => {}
        }
    }
    None
}

/// Parse transmission line: T1 port1+ port1- port2+ port2- Z0=val TD=val
/// Or: T1 port1+ port1- port2+ port2- Z0=val F=freq NL=wavelengths
fn parse_transmission_line(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let port1_pos = expect_node(stream, line_num)?;
    let port1_neg = expect_node(stream, line_num)?;
    let port2_pos = expect_node(stream, line_num)?;
    let port2_neg = expect_node(stream, line_num)?;

    let parsed = parse_tline_params(stream, line_num, params, false)?;
    let z0 = parsed.z0.ok_or_else(|| ParseError::Syntax {
        line: line_num,
        message: "Transmission line requires Z0".to_string(),
    })?;

    elements.push(Element {
        name,
        kind: ElementKind::TransmissionLine {
            z0: Some(z0),
            td: parsed.td,
            freq: parsed.freq,
            nl: parsed.nl,
            model: parsed.model,
        },
        nodes: vec![port1_pos, port1_neg, port2_pos, port2_neg],
    });

    Ok(())
}

#[derive(Default)]
struct ParsedTlineParams {
    z0: Option<Value>,
    td: Option<Value>,
    freq: Option<Value>,
    nl: Option<Value>,
    model: Option<String>,
}

fn parse_tline_params(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    allow_bare_model: bool,
) -> Result<ParsedTlineParams, ParseError> {
    let mut parsed = ParsedTlineParams::default();

    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        skip_commas(stream);
        if stream.is_eof() || matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }

        if let Some(v) = try_value(stream, params) {
            // Positional values map to Z0 then TD.
            if parsed.z0.is_none() {
                parsed.z0 = Some(v);
            } else if parsed.td.is_none() {
                parsed.td = Some(v);
            }
            continue;
        }

        let TokenKind::Ident(token) = &stream.peek().kind else {
            stream.advance();
            continue;
        };

        let token = token.clone();
        let token_upper = token.to_ascii_uppercase();
        let has_equals = matches!(stream.peek_n(1).kind, TokenKind::Equals);

        // O/Y/P legacy syntax often uses a bare model token after node list.
        if allow_bare_model
            && !has_equals
            && parsed.model.is_none()
            && !matches!(
                token_upper.as_str(),
                "Z0" | "ZO" | "TD" | "F" | "FREQ" | "NL" | "MODEL"
            )
        {
            stream.advance();
            parsed.model = Some(token);
            continue;
        }

        stream.advance();
        if has_equals {
            stream.consume(&TokenKind::Equals);
        }

        match token_upper.as_str() {
            "Z0" | "ZO" => {
                parsed.z0 = Some(expect_value(stream, line_num, params)?);
            }
            "TD" => {
                parsed.td = Some(expect_value(stream, line_num, params)?);
            }
            "F" | "FREQ" => {
                parsed.freq = Some(expect_value(stream, line_num, params)?);
            }
            "NL" => {
                parsed.nl = Some(expect_value(stream, line_num, params)?);
            }
            "MODEL" => {
                parsed.model = Some(expect_ident(stream, line_num)?);
            }
            _ => {
                // Unknown key/value token; skip one optional value token if present.
                let _ = try_value(stream, params);
            }
        }
    }

    Ok(parsed)
}

//=============================================================================
// New Analysis Command Parsing
//=============================================================================

/// Parse .STEP command
/// Formats:
/// - .STEP PARAM name start stop increment
/// - .STEP PARAM name LIST v1 v2 v3...
/// - .STEP DEC PARAM name start stop points
/// - .STEP OCT PARAM name start stop points
/// - .STEP TEMP start stop increment
/// - .STEP TEMP LIST t1 t2 t3...
fn parse_step_command(
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
fn parse_temp_command(
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
fn parse_four_command(
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

/// Parse .NOISE command: .NOISE V(out[,ref]) Vsource DEC|LIN|OCT np fstart fstop
fn parse_noise_command(
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
fn parse_sens_command(
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
fn parse_pz_command(
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
fn parse_mc_command(
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
fn parse_voltage_reference(spec: &str) -> Result<(String, Option<String>), ParseError> {
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
fn parse_voltage_output_reference(
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
fn parse_nodeset_command(
    stream: &mut TokenStream,
    _line_num: usize,
    params: &mut ParamContext,
) -> Result<(), ParseError> {
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        skip_commas(stream);

        if let TokenKind::Ident(s) = &stream.peek().kind {
            let spec = s.clone();
            stream.advance();

            // Consume = if present
            stream.consume(&TokenKind::Equals);

            if let Some(v) = try_value(stream, params) {
                // Store as NODESET_nodename parameter
                let param_name = format!("NODESET_{}", spec.replace("V(", "").replace(")", ""));
                params.set(&param_name, v);
            }
        } else {
            stream.advance();
        }
    }

    Ok(())
}

/// Parse .IC command: .IC V(node1)=val V(node2)=val...
///
/// Initial conditions set the starting voltages for transient analysis.
/// Format: .IC V(node)=voltage [V(node2)=voltage2] ...
fn parse_ic_command(
    stream: &mut TokenStream,
    _line_num: usize,
    params: &mut ParamContext,
) -> Result<(), ParseError> {
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        skip_commas(stream);

        if let TokenKind::Ident(s) = &stream.peek().kind {
            let spec = s.clone();
            stream.advance();

            // Consume = if present
            stream.consume(&TokenKind::Equals);

            if let Some(v) = try_value(stream, params) {
                // Extract node name from V(node) syntax
                let node_name = spec
                    .to_uppercase()
                    .replace("V(", "")
                    .replace(")", "")
                    .trim()
                    .to_string();

                if !node_name.is_empty() {
                    // Store as IC_nodename parameter for later use by engine
                    let param_name = format!("IC_{}", node_name);
                    params.set(&param_name, v);
                }
            }
        } else {
            stream.advance();
        }
    }

    Ok(())
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::netlist::lexer::parse_spice_value;

    #[test]
    fn test_parse_value() {
        let v = parse_spice_value("1k").unwrap();
        assert!((v - 1000.0).abs() < 1e-10);

        let v = parse_spice_value("1u").unwrap();
        assert!((v - 1e-6).abs() < 1e-20);

        let v = parse_spice_value("1MEG").unwrap();
        assert!((v - 1e6).abs() < 1e-10);
    }

    #[test]
    fn test_parse_simple_netlist() {
        let netlist = r#"Simple RC Circuit
R1 1 2 1k
C1 2 0 1u
V1 1 0 DC 5
.OP
.END
"#;
        let result = parse_netlist(netlist).unwrap();
        assert_eq!(result.title, "Simple RC Circuit");
        assert_eq!(result.elements.len(), 3);
        assert_eq!(result.analyses.len(), 1);
    }

    #[test]
    fn test_parse_options_tolerances_and_method() {
        let netlist = r#"Options Test
.OPTIONS RELTOL=2e-4 VNTOL=3e-6 ABSTOL=8e-13 IABSTOL=4e-12 RESIDUAL_RELTOL=5e-4 METHOD=GEAR ITL1=120 ITL4=9
.END
"#;
        let result = parse_netlist(netlist).unwrap();

        assert_eq!(result.options.reltol, Some(2e-4));
        assert_eq!(result.options.vntol, Some(3e-6));
        assert_eq!(result.options.abstol, Some(8e-13));
        assert_eq!(result.options.iabstol, Some(4e-12));
        assert_eq!(result.options.residual_reltol, Some(5e-4));
        assert_eq!(result.options.method.as_deref(), Some("GEAR"));
        assert_eq!(result.options.itl1, Some(120));
        assert_eq!(result.options.itl4, Some(9));
    }

    #[test]
    fn test_parse_options_continuation_and_merge() {
        let netlist = r#"Options Merge Test
.OPTIONS RELTOL=1e-3
+ IABSTOL=2e-12 RESRELTOL=7e-4
.OPTIONS RELTOL=5e-4
.END
"#;
        let result = parse_netlist(netlist).unwrap();

        assert_eq!(result.options.reltol, Some(5e-4));
        assert_eq!(result.options.iabstol, Some(2e-12));
        assert_eq!(result.options.residual_reltol, Some(7e-4));
    }

    #[test]
    fn test_parse_options_unknown_flag_does_not_consume_next_option() {
        let netlist = r#"Options Unknown Flag Test
.OPTIONS NOPAGE RELTOL=4e-4
.END
"#;
        let result = parse_netlist(netlist).unwrap();
        assert_eq!(result.options.reltol, Some(4e-4));
    }

    #[test]
    fn test_parse_with_commas() {
        let netlist = r#"Comma Test
R1 1 0 1k, temp=27
.END
"#;
        let result = parse_netlist(netlist).unwrap();
        assert_eq!(result.elements.len(), 1);
        match &result.elements[0].kind {
            ElementKind::Resistor { value } => {
                assert!((value - 1000.0).abs() < 1e-10);
            }
            _ => panic!("Expected Resistor"),
        }
    }

    #[test]
    fn test_parse_pulse() {
        let netlist = r#"Pulse Test
V1 1 0 PULSE(0 5 0 1n 1n 1u 2u)
.END
"#;
        let result = parse_netlist(netlist).unwrap();
        match &result.elements[0].kind {
            ElementKind::VoltageSource(SourceSpec::Pulse {
                v1,
                v2,
                delay,
                rise,
                fall,
                width,
                period,
            }) => {
                assert!((*v1 - 0.0).abs() < 1e-10);
                assert!((*v2 - 5.0).abs() < 1e-10);
                assert!((*delay - 0.0).abs() < 1e-10);
                assert!((*rise - 1e-9).abs() < 1e-20);
                assert!((*fall - 1e-9).abs() < 1e-20);
                assert!((*width - 1e-6).abs() < 1e-15);
                assert!((*period - 2e-6).abs() < 1e-15);
            }
            _ => panic!("Expected Pulse source"),
        }
    }

    #[test]
    fn test_parse_sin() {
        let netlist = r#"Sin Test
V1 1 0 SIN(0 1 1k)
.END
"#;
        let result = parse_netlist(netlist).unwrap();
        match &result.elements[0].kind {
            ElementKind::VoltageSource(SourceSpec::Sin {
                offset,
                amplitude,
                frequency,
                ..
            }) => {
                assert!((*offset - 0.0).abs() < 1e-10);
                assert!((*amplitude - 1.0).abs() < 1e-10);
                assert!((*frequency - 1000.0).abs() < 1e-10);
            }
            _ => panic!("Expected Sin source"),
        }
    }

    #[test]
    fn test_parse_ac_phase_is_degrees_converted_to_radians() {
        let netlist = r#"AC Phase Test
V1 1 0 AC 1 90
.END
"#;
        let result = parse_netlist(netlist).unwrap();
        match &result.elements[0].kind {
            ElementKind::VoltageSource(SourceSpec::Ac { magnitude, phase }) => {
                assert!((*magnitude - 1.0).abs() < 1e-12);
                assert!((*phase - std::f64::consts::FRAC_PI_2).abs() < 1e-12);
            }
            _ => panic!("Expected AC source"),
        }
    }

    #[test]
    fn test_parse_dcac_phase_is_degrees_converted_to_radians() {
        let netlist = r#"DC AC Phase Test
I1 1 0 DC 1m AC 2 180
.END
"#;
        let result = parse_netlist(netlist).unwrap();
        match &result.elements[0].kind {
            ElementKind::CurrentSource(SourceSpec::DcAc {
                dc_value,
                ac_magnitude,
                ac_phase,
            }) => {
                assert!((*dc_value - 1e-3).abs() < 1e-15);
                assert!((*ac_magnitude - 2.0).abs() < 1e-12);
                assert!((*ac_phase - std::f64::consts::PI).abs() < 1e-12);
            }
            _ => panic!("Expected DC+AC source"),
        }
    }

    #[test]
    fn test_parse_sin_phase_is_degrees_converted_to_radians() {
        let netlist = r#"Sin Phase Test
V1 1 0 SIN(0 1 1k 0 0 90)
.END
"#;
        let result = parse_netlist(netlist).unwrap();
        match &result.elements[0].kind {
            ElementKind::VoltageSource(SourceSpec::Sin { phase, .. }) => {
                assert!((*phase - std::f64::consts::FRAC_PI_2).abs() < 1e-12);
            }
            _ => panic!("Expected Sin source"),
        }
    }

    #[test]
    fn test_parse_param() {
        let netlist = r#"Param Test
.PARAM R=1k
R1 1 0 {R}
.END
"#;
        let result = parse_netlist(netlist).unwrap();
        assert!(result.params.get("R").is_some());
        assert!((result.params.get("R").unwrap() - 1000.0).abs() < 1e-10);

        match &result.elements[0].kind {
            ElementKind::Resistor { value } => {
                assert!((value - 1000.0).abs() < 1e-10);
            }
            _ => panic!("Expected Resistor"),
        }
    }

    #[test]
    fn test_parse_param_expression() {
        let netlist = r#"Param Expression Test
.PARAM R1=1k R2=500
.PARAM RTOTAL={R1+R2}
.END
"#;
        let result = parse_netlist(netlist).unwrap();
        assert!((result.params.get("RTOTAL").unwrap() - 1500.0).abs() < 1e-10);
    }

    #[test]
    fn test_parse_diode() {
        let netlist = r#"Diode Test
D1 1 0 1N4148
.END
"#;
        let result = parse_netlist(netlist).unwrap();
        assert_eq!(result.elements.len(), 1);
        match &result.elements[0].kind {
            ElementKind::Diode { model } => {
                assert_eq!(model, "1N4148");
            }
            _ => panic!("Expected Diode element"),
        }
    }

    #[test]
    fn test_parse_bjt() {
        let netlist = r#"BJT Test
Q1 2 1 0 2N2222
.END
"#;
        let result = parse_netlist(netlist).unwrap();
        match &result.elements[0].kind {
            ElementKind::Bjt { model, .. } => {
                assert_eq!(model, "2N2222");
            }
            _ => panic!("Expected Bjt element"),
        }
    }

    #[test]
    fn test_parse_mosfet() {
        let netlist = r#"MOSFET Test
M1 3 2 1 0 NMOS
.END
"#;
        let result = parse_netlist(netlist).unwrap();
        match &result.elements[0].kind {
            ElementKind::Mosfet { model, .. } => {
                assert_eq!(model, "NMOS");
            }
            _ => panic!("Expected Mosfet element"),
        }
    }

    #[test]
    fn test_parse_subcircuit() {
        let netlist = r#"Subcircuit Test
.SUBCKT INVERTER IN OUT VDD VSS
M1 OUT IN VDD VDD PMOS
M2 OUT IN VSS VSS NMOS
.ENDS INVERTER
X1 A B VCC GND INVERTER
.END
"#;
        let result = parse_netlist(netlist).unwrap();
        assert_eq!(result.subcircuits.len(), 1);
        assert_eq!(result.subcircuits[0].name, "INVERTER");
        assert_eq!(result.subcircuits[0].ports, vec!["IN", "OUT", "VDD", "VSS"]);
        assert_eq!(result.subcircuits[0].elements.len(), 2);
        assert_eq!(result.elements.len(), 1);
    }

    #[test]
    fn test_parse_model() {
        let netlist = r#"Model Test
.MODEL NMOS NMOS (VTO=0.7 KP=110u)
.END
"#;
        let result = parse_netlist(netlist).unwrap();
        assert_eq!(result.models.len(), 1);
        assert_eq!(result.models[0].name, "NMOS");
    }

    #[test]
    fn test_parse_model_with_inline_semicolon_comments() {
        let netlist = r#"Model Inline Comment Test
.MODEL 2N2222 NPN (
+ IS=14.34E-15 ; Saturation current
+ BF=255.9 ; Forward beta
+ VAF=74.03 ; Early voltage
+)
Q1 2 1 0 2N2222
.END
"#;
        let result = parse_netlist(netlist).unwrap();
        assert_eq!(result.models.len(), 1);
        assert_eq!(result.models[0].name, "2N2222");

        let params: std::collections::HashMap<String, Value> =
            result.models[0].params.iter().cloned().collect();
        assert!((params.get("IS").unwrap() - 14.34e-15).abs() < 1e-24);
        assert!((params.get("BF").unwrap() - 255.9).abs() < 1e-12);
        assert!((params.get("VAF").unwrap() - 74.03).abs() < 1e-12);
    }

    #[test]
    fn test_parse_element_with_inline_semicolon_comment() {
        let netlist = r#"Inline Element Comment Test
V1 1 0 5 ; DC supply
R1 1 0 1k ; load resistor
.END
"#;
        let result = parse_netlist(netlist).unwrap();
        assert_eq!(result.elements.len(), 2);
        match &result.elements[1].kind {
            ElementKind::Resistor { value } => assert!((*value - 1000.0).abs() < 1e-10),
            _ => panic!("Expected resistor"),
        }
    }

    #[test]
    fn test_parse_pwl() {
        let netlist = r#"PWL Test
V1 1 0 PWL(0 0 1u 5 2u 0)
.END
"#;
        let result = parse_netlist(netlist).unwrap();
        match &result.elements[0].kind {
            ElementKind::VoltageSource(SourceSpec::Pwl { points }) => {
                assert_eq!(points.len(), 3);
                assert!((points[0].0 - 0.0).abs() < 1e-10);
                assert!((points[0].1 - 0.0).abs() < 1e-10);
                assert!((points[1].0 - 1e-6).abs() < 1e-15);
                assert!((points[1].1 - 5.0).abs() < 1e-10);
            }
            _ => panic!("Expected PWL source"),
        }
    }

    #[test]
    fn test_parse_pwl_file_source() {
        let netlist = r#"PWL FILE Test
V1 1 0 PWL FILE="stimulus.csv" TSCALE=1e-3 VSCALE=2 TOFFSET=1u VOFFSET=0.25
.END
"#;
        let result = parse_netlist(netlist).unwrap();
        match &result.elements[0].kind {
            ElementKind::VoltageSource(SourceSpec::PwlFile {
                path,
                time_scale,
                value_scale,
                time_offset,
                value_offset,
            }) => {
                assert_eq!(path, "stimulus.csv");
                assert!((*time_scale - 1e-3).abs() < 1e-15);
                assert!((*value_scale - 2.0).abs() < 1e-12);
                assert!((*time_offset - 1e-6).abs() < 1e-15);
                assert!((*value_offset - 0.25).abs() < 1e-12);
            }
            _ => panic!("Expected PWL file source"),
        }
    }

    #[test]
    fn test_parse_pwl_file_source_defaults() {
        let netlist = r#"PWL FILE Defaults Test
I1 1 0 PWL(FILE="wave.csv")
.END
"#;
        let result = parse_netlist(netlist).unwrap();
        match &result.elements[0].kind {
            ElementKind::CurrentSource(SourceSpec::PwlFile {
                path,
                time_scale,
                value_scale,
                time_offset,
                value_offset,
            }) => {
                assert_eq!(path, "wave.csv");
                assert!((*time_scale - 1.0).abs() < 1e-12);
                assert!((*value_scale - 1.0).abs() < 1e-12);
                assert!((*time_offset - 0.0).abs() < 1e-12);
                assert!((*value_offset - 0.0).abs() < 1e-12);
            }
            _ => panic!("Expected PWL file source"),
        }
    }

    // =========================================================================
    // New Element Type Tests
    // =========================================================================

    #[test]
    fn test_parse_coupling() {
        let netlist = r#"Coupling Test
L1 1 2 1m
L2 3 4 4m
K1 L1 L2 0.99
.END
"#;
        let result = parse_netlist(netlist).unwrap();
        assert_eq!(result.elements.len(), 3);

        match &result.elements[2].kind {
            ElementKind::Coupling {
                inductors,
                coefficient,
            } => {
                assert_eq!(inductors.len(), 2);
                assert_eq!(inductors[0], "L1");
                assert_eq!(inductors[1], "L2");
                assert!((*coefficient - 0.99).abs() < 1e-10);
            }
            _ => panic!("Expected Coupling element"),
        }
    }

    #[test]
    fn test_parse_vswitch() {
        let netlist = r#"VSwitch Test
S1 out 0 ctrl 0 SW1 OFF
.END
"#;
        let result = parse_netlist(netlist).unwrap();
        match &result.elements[0].kind {
            ElementKind::VSwitch {
                control_pos,
                control_neg,
                model,
                initial_state,
            } => {
                // Note: lexer uppercases identifiers
                assert!(control_pos.eq_ignore_ascii_case("ctrl"));
                assert_eq!(control_neg, "0");
                assert!(model.eq_ignore_ascii_case("SW1"));
                assert_eq!(*initial_state, Some(SwitchState::Off));
            }
            _ => panic!("Expected VSwitch element"),
        }
    }

    #[test]
    fn test_parse_iswitch() {
        let netlist = r#"ISwitch Test
W1 out 0 V1 CSW ON
.END
"#;
        let result = parse_netlist(netlist).unwrap();
        match &result.elements[0].kind {
            ElementKind::ISwitch {
                control_element,
                model,
                initial_state,
            } => {
                assert_eq!(control_element, "V1");
                assert_eq!(model, "CSW");
                assert_eq!(*initial_state, Some(SwitchState::On));
            }
            _ => panic!("Expected ISwitch element"),
        }
    }

    #[test]
    fn test_parse_transmission_line() {
        let netlist = r#"TLine Test
T1 in 0 out 0 Z0=50 TD=1n
.END
"#;
        let result = parse_netlist(netlist).unwrap();
        match &result.elements[0].kind {
            ElementKind::TransmissionLine {
                z0,
                td,
                freq,
                nl,
                model,
            } => {
                assert_eq!(*z0, Some(50.0));
                assert!(td.is_some());
                assert!((td.unwrap() - 1e-9).abs() < 1e-20);
                assert!(freq.is_none());
                assert!(nl.is_none());
                assert!(model.is_none());
            }
            _ => panic!("Expected TransmissionLine element"),
        }
        assert_eq!(result.elements[0].nodes.len(), 4);
    }

    #[test]
    fn test_parse_lossless_tline_model_reference() {
        let netlist = r#"O-Line Model Test
O1 1 0 2 0 LLINE
.END
"#;
        let result = parse_netlist(netlist).expect("O-line with model should parse");
        match &result.elements[0].kind {
            ElementKind::TransmissionLine {
                z0,
                td,
                freq,
                nl,
                model,
            } => {
                assert!(z0.is_none());
                assert!(td.is_none());
                assert!(freq.is_none());
                assert!(nl.is_none());
                assert_eq!(model.as_deref(), Some("LLINE"));
            }
            _ => panic!("Expected TransmissionLine element"),
        }
    }

    #[test]
    fn test_parse_lossy_tline_model_with_inline_overrides() {
        let netlist = r#"Y-Line Model Test
Y1 1 0 2 0 YMOD Z0=75 TD=2N
.END
"#;
        let result = parse_netlist(netlist).expect("Y-line with model and overrides should parse");
        match &result.elements[0].kind {
            ElementKind::TransmissionLine {
                z0,
                td,
                freq,
                nl,
                model,
            } => {
                assert_eq!(*z0, Some(75.0));
                assert_eq!(*td, Some(2e-9));
                assert!(freq.is_none());
                assert!(nl.is_none());
                assert_eq!(model.as_deref(), Some("YMOD"));
            }
            _ => panic!("Expected TransmissionLine element"),
        }
    }

    #[test]
    fn test_parse_lossless_tline_requires_model_or_z0() {
        let netlist = r#"Invalid O-Line
O1 1 0 2 0
.END
"#;
        let err = parse_netlist(netlist).expect_err("O-line without model/z0 should fail");
        match err {
            ParseError::Syntax { message, .. } => {
                assert!(message.contains("requires MODEL name or Z0"));
            }
            _ => panic!("Expected syntax error"),
        }
    }

    // =========================================================================
    // New Analysis Command Tests
    // =========================================================================

    #[test]
    fn test_parse_step_linear() {
        let netlist = r#"Step Test
R1 1 0 1k
.STEP PARAM RL 100 1k 100
.END
"#;
        let result = parse_netlist(netlist).unwrap();
        assert_eq!(result.analyses.len(), 1);

        match &result.analyses[0] {
            AnalysisCommand::Step(cmd) => {
                assert_eq!(cmd.target, StepTarget::Param);
                assert_eq!(cmd.name, "RL");
                match &cmd.sweep {
                    StepSweep::Linear { start, stop, step } => {
                        assert!((*start - 100.0).abs() < 1e-10);
                        assert!((*stop - 1000.0).abs() < 1e-10);
                        assert!((*step - 100.0).abs() < 1e-10);
                    }
                    _ => panic!("Expected Linear sweep"),
                }
            }
            _ => panic!("Expected Step command"),
        }
    }

    #[test]
    fn test_parse_step_list() {
        let netlist = r#"Step List Test
.STEP PARAM C1 LIST 1n 10n 100n
.END
"#;
        let result = parse_netlist(netlist).unwrap();

        match &result.analyses[0] {
            AnalysisCommand::Step(cmd) => match &cmd.sweep {
                StepSweep::List(values) => {
                    assert_eq!(values.len(), 3);
                    assert!((values[0] - 1e-9).abs() < 1e-20);
                    assert!((values[1] - 10e-9).abs() < 1e-18);
                    assert!((values[2] - 100e-9).abs() < 1e-17);
                }
                _ => panic!("Expected List sweep"),
            },
            _ => panic!("Expected Step command"),
        }
    }

    #[test]
    fn test_parse_step_device_parenthesized_param_name() {
        let netlist = r#"Step Device Param
.STEP R1(VALUE) 1k 5k 1k
.END
"#;
        let result = parse_netlist(netlist).expect(".STEP device(param) should parse");

        match &result.analyses[0] {
            AnalysisCommand::Step(cmd) => {
                assert_eq!(cmd.target, StepTarget::Device);
                assert_eq!(cmd.name, "R1");
                assert_eq!(cmd.param_name.as_deref(), Some("VALUE"));
                match &cmd.sweep {
                    StepSweep::Linear { start, stop, step } => {
                        assert!((*start - 1e3).abs() < 1e-10);
                        assert!((*stop - 5e3).abs() < 1e-10);
                        assert!((*step - 1e3).abs() < 1e-10);
                    }
                    _ => panic!("Expected Linear sweep"),
                }
            }
            _ => panic!("Expected Step command"),
        }
    }

    #[test]
    fn test_parse_step_device_named_param_name() {
        let netlist = r#"Step Device Named Param
.STEP R1 VALUE 1k 3k 1k
.END
"#;
        let result = parse_netlist(netlist).expect(".STEP device param should parse");

        match &result.analyses[0] {
            AnalysisCommand::Step(cmd) => {
                assert_eq!(cmd.target, StepTarget::Device);
                assert_eq!(cmd.name, "R1");
                assert_eq!(cmd.param_name.as_deref(), Some("VALUE"));
            }
            _ => panic!("Expected Step command"),
        }
    }

    #[test]
    fn test_parse_step_device_named_param_with_list() {
        let netlist = r#"Step Device Named Param List
.STEP R1 VALUE LIST 1k 2k 5k
.END
"#;
        let result = parse_netlist(netlist).expect(".STEP device param LIST should parse");

        match &result.analyses[0] {
            AnalysisCommand::Step(cmd) => {
                assert_eq!(cmd.target, StepTarget::Device);
                assert_eq!(cmd.name, "R1");
                assert_eq!(cmd.param_name.as_deref(), Some("VALUE"));
                match &cmd.sweep {
                    StepSweep::List(values) => assert_eq!(values, &vec![1e3, 2e3, 5e3]),
                    _ => panic!("Expected List sweep"),
                }
            }
            _ => panic!("Expected Step command"),
        }
    }

    #[test]
    fn test_parse_step_model_param_name() {
        let netlist = r#"Step Model Param
.STEP MODEL NMOS VTO -0.5 0.5 0.25
.END
"#;
        let result = parse_netlist(netlist).expect(".STEP MODEL should parse");

        match &result.analyses[0] {
            AnalysisCommand::Step(cmd) => {
                assert_eq!(cmd.target, StepTarget::Model);
                assert_eq!(cmd.name, "NMOS");
                assert_eq!(cmd.param_name.as_deref(), Some("VTO"));
                match &cmd.sweep {
                    StepSweep::Linear { start, stop, step } => {
                        assert!((*start - -0.5).abs() < 1e-12);
                        assert!((*stop - 0.5).abs() < 1e-12);
                        assert!((*step - 0.25).abs() < 1e-12);
                    }
                    _ => panic!("Expected Linear sweep"),
                }
            }
            _ => panic!("Expected Step command"),
        }
    }

    #[test]
    fn test_parse_step_temp_linear_without_name() {
        let netlist = r#"Step Temp Linear
.STEP TEMP -40 125 55
.END
"#;
        let result = parse_netlist(netlist).expect(".STEP TEMP linear should parse");

        match &result.analyses[0] {
            AnalysisCommand::Step(cmd) => {
                assert_eq!(cmd.target, StepTarget::Temp);
                assert_eq!(cmd.name, "TEMP");
                match &cmd.sweep {
                    StepSweep::Linear { start, stop, step } => {
                        assert!((*start - -40.0).abs() < 1e-10);
                        assert!((*stop - 125.0).abs() < 1e-10);
                        assert!((*step - 55.0).abs() < 1e-10);
                    }
                    _ => panic!("Expected Linear sweep"),
                }
            }
            _ => panic!("Expected Step command"),
        }
    }

    #[test]
    fn test_parse_step_temp_list_without_name() {
        let netlist = r#"Step Temp List
.STEP TEMP LIST -40 27 85 125
.END
"#;
        let result = parse_netlist(netlist).expect(".STEP TEMP LIST should parse");

        match &result.analyses[0] {
            AnalysisCommand::Step(cmd) => {
                assert_eq!(cmd.target, StepTarget::Temp);
                assert_eq!(cmd.name, "TEMP");
                match &cmd.sweep {
                    StepSweep::List(values) => {
                        assert_eq!(values, &vec![-40.0, 27.0, 85.0, 125.0]);
                    }
                    _ => panic!("Expected List sweep"),
                }
            }
            _ => panic!("Expected Step command"),
        }
    }

    #[test]
    fn test_parse_temp() {
        let netlist = r#"Temp Test
.TEMP -40 27 85 125
.END
"#;
        let result = parse_netlist(netlist).unwrap();

        match &result.analyses[0] {
            AnalysisCommand::Temp { temperatures } => {
                assert_eq!(temperatures.len(), 4);
                assert!((*temperatures.get(0).unwrap() - -40.0).abs() < 1e-10);
                assert!((*temperatures.get(1).unwrap() - 27.0).abs() < 1e-10);
                assert!((*temperatures.get(2).unwrap() - 85.0).abs() < 1e-10);
                assert!((*temperatures.get(3).unwrap() - 125.0).abs() < 1e-10);
            }
            _ => panic!("Expected Temp command"),
        }
    }

    #[test]
    fn test_parse_four() {
        // Use simpler node names without parentheses
        let netlist = r#"Fourier Test
.FOUR 1k OUT
.END
"#;
        let result = parse_netlist(netlist).unwrap();

        match &result.analyses[0] {
            AnalysisCommand::Four {
                fundamental,
                outputs,
                ..
            } => {
                assert!((*fundamental - 1000.0).abs() < 1e-10);
                assert!(!outputs.is_empty());
            }
            _ => panic!("Expected Four command"),
        }
    }

    #[test]
    fn test_parse_noise() {
        // Use simpler output specification without V() wrapper
        let netlist = r#"Noise Test
.NOISE OUT V1 DEC 10 1 1MEG
.END
"#;
        let result = parse_netlist(netlist).unwrap();

        match &result.analyses[0] {
            AnalysisCommand::Noise {
                output_node,
                input_source,
                variation,
                points,
                start_freq,
                stop_freq,
                ..
            } => {
                assert_eq!(output_node, "OUT");
                assert_eq!(input_source, "V1");
                assert_eq!(*variation, FreqVariation::Dec);
                assert_eq!(*points, 10);
                assert!((*start_freq - 1.0).abs() < 1e-10);
                assert!((*stop_freq - 1e6).abs() < 1e-3);
            }
            _ => panic!("Expected Noise command"),
        }
    }

    #[test]
    fn test_parse_mc_defaults() {
        let netlist = r#"Monte Carlo Default
.MC 128
.END
"#;
        let result = parse_netlist(netlist).unwrap();

        match &result.analyses[0] {
            AnalysisCommand::MonteCarlo(cmd) => {
                assert_eq!(cmd.runs, 128);
                assert_eq!(cmd.seed, None);
                assert_eq!(cmd.distribution, MonteCarloDistribution::Gaussian);
                assert!((cmd.relative_spread - 0.01).abs() < 1e-12);
                assert!(cmd.params.is_empty());
            }
            _ => panic!("Expected MonteCarlo command"),
        }
    }

    #[test]
    fn test_parse_mc_with_full_options() {
        let netlist = r#"Monte Carlo Full
.MC 200 SEED 77 DIST UNIFORM SPREAD 0.05 PARAMS RVAL CVAL
.END
"#;
        let result = parse_netlist(netlist).unwrap();

        match &result.analyses[0] {
            AnalysisCommand::MonteCarlo(cmd) => {
                assert_eq!(cmd.runs, 200);
                assert_eq!(cmd.seed, Some(77));
                assert_eq!(cmd.distribution, MonteCarloDistribution::Uniform);
                assert!((cmd.relative_spread - 0.05).abs() < 1e-12);
                assert_eq!(cmd.params, vec!["RVAL".to_string(), "CVAL".to_string()]);
            }
            _ => panic!("Expected MonteCarlo command"),
        }
    }

    #[test]
    fn test_parse_mc_shorthand_gaussian() {
        let netlist = r#"Monte Carlo Shorthand
.MC 64 GAUSS 0.02 PARAMS RVAL
.END
"#;
        let result = parse_netlist(netlist).unwrap();

        match &result.analyses[0] {
            AnalysisCommand::MonteCarlo(cmd) => {
                assert_eq!(cmd.runs, 64);
                assert_eq!(cmd.distribution, MonteCarloDistribution::Gaussian);
                assert!((cmd.relative_spread - 0.02).abs() < 1e-12);
                assert_eq!(cmd.params, vec!["RVAL".to_string()]);
            }
            _ => panic!("Expected MonteCarlo command"),
        }
    }

    #[test]
    fn test_parse_mc_with_worst_case_distribution() {
        let netlist = r#"Monte Carlo Worst Case
.MC 32 DIST WORSTCASE SPREAD 0.03 PARAMS RVAL
.END
"#;
        let result = parse_netlist(netlist).unwrap();

        match &result.analyses[0] {
            AnalysisCommand::MonteCarlo(cmd) => {
                assert_eq!(cmd.runs, 32);
                assert_eq!(cmd.distribution, MonteCarloDistribution::WorstCase);
                assert!((cmd.relative_spread - 0.03).abs() < 1e-12);
                assert_eq!(cmd.params, vec!["RVAL".to_string()]);
            }
            _ => panic!("Expected MonteCarlo command"),
        }
    }

    #[test]
    fn test_parse_mc_invalid_distribution() {
        let netlist = r#"Monte Carlo Invalid Dist
.MC 16 DIST BAD
.END
"#;
        let err = parse_netlist(netlist).expect_err("expected .MC distribution parse error");
        assert!(
            err.to_string()
                .contains("expected GAUSS, UNIFORM, or WORSTCASE")
        );
    }

    #[test]
    fn test_parse_mc_invalid_runs() {
        let netlist = r#"Monte Carlo Invalid Runs
.MC 0
.END
"#;
        let err = parse_netlist(netlist).expect_err("expected .MC run count parse error");
        assert!(err.to_string().contains("positive integer"));
    }

    #[test]
    fn test_parse_sens_dc() {
        let netlist = r#"Sensitivity Test
.SENS V(out)
.END
"#;
        let result = parse_netlist(netlist).unwrap();

        match &result.analyses[0] {
            AnalysisCommand::Sensitivity {
                output_node,
                reference_node,
                ac_sweep,
            } => {
                assert_eq!(output_node, "OUT");
                assert!(reference_node.is_none());
                assert!(ac_sweep.is_none());
            }
            _ => panic!("Expected Sensitivity command"),
        }
    }

    #[test]
    fn test_parse_sens_ac_with_reference() {
        let netlist = r#"Sensitivity AC Test
.SENS V(out,ref) AC DEC 10 1 1MEG
.END
"#;
        let result = parse_netlist(netlist).unwrap();

        match &result.analyses[0] {
            AnalysisCommand::Sensitivity {
                output_node,
                reference_node,
                ac_sweep,
            } => {
                assert_eq!(output_node, "OUT");
                assert_eq!(reference_node.as_deref(), Some("REF"));
                let ac = ac_sweep.expect("expected AC sweep");
                assert_eq!(ac.variation, FreqVariation::Dec);
                assert_eq!(ac.points, 10);
                assert!((ac.start_freq - 1.0).abs() < 1e-12);
                assert!((ac.stop_freq - 1e6).abs() < 1e-3);
            }
            _ => panic!("Expected Sensitivity command"),
        }
    }

    #[test]
    fn test_parse_sens_invalid_mode() {
        let netlist = r#"Sensitivity Invalid
.SENS V(out) BAD
.END
"#;
        let err = parse_netlist(netlist).expect_err("expected invalid .SENS mode");
        assert!(err.to_string().contains("expected AC or end-of-line"));
    }

    #[test]
    fn test_parse_sens_invalid_variation() {
        let netlist = r#"Sensitivity Invalid AC
.SENS V(out) AC BAD 10 1 1MEG
.END
"#;
        let err = parse_netlist(netlist).expect_err("expected invalid .SENS AC variation");
        assert!(err.to_string().contains("expected LIN, OCT, or DEC"));
    }

    #[test]
    fn test_parse_pz_voltage_pole_zero() {
        let netlist = r#"PZ Test
.PZ in 0 out 0 VOL PZ
.END
"#;
        let result = parse_netlist(netlist).unwrap();

        match &result.analyses[0] {
            AnalysisCommand::PoleZero {
                input_pos,
                input_neg,
                output_pos,
                output_neg,
                transfer_type,
                analysis_type,
            } => {
                assert_eq!(input_pos, "IN");
                assert_eq!(input_neg, "0");
                assert_eq!(output_pos, "OUT");
                assert_eq!(output_neg, "0");
                assert_eq!(*transfer_type, PoleZeroTransferType::Voltage);
                assert_eq!(*analysis_type, PoleZeroAnalysisType::PoleZero);
            }
            _ => panic!("Expected PoleZero command"),
        }
    }

    #[test]
    fn test_parse_pz_current_poles_only() {
        let netlist = r#"PZ Poles Test
.PZ 1 0 2 0 CUR POL
.END
"#;
        let result = parse_netlist(netlist).unwrap();

        match &result.analyses[0] {
            AnalysisCommand::PoleZero {
                transfer_type,
                analysis_type,
                ..
            } => {
                assert_eq!(*transfer_type, PoleZeroTransferType::Current);
                assert_eq!(*analysis_type, PoleZeroAnalysisType::PolesOnly);
            }
            _ => panic!("Expected PoleZero command"),
        }
    }

    #[test]
    fn test_parse_pz_invalid_transfer_type() {
        let netlist = r#"PZ Invalid Transfer
.PZ in 0 out 0 BAD PZ
.END
"#;
        let err = parse_netlist(netlist).expect_err("expected .PZ transfer type error");
        assert!(err.to_string().contains("VOL or CUR"));
    }

    #[test]
    fn test_parse_pz_invalid_analysis_type() {
        let netlist = r#"PZ Invalid Type
.PZ in 0 out 0 VOL BAD
.END
"#;
        let err = parse_netlist(netlist).expect_err("expected .PZ analysis type error");
        assert!(err.to_string().contains("PZ, POL, or ZER"));
    }

    #[test]
    fn test_parse_ic() {
        // Use simpler node format without parentheses
        let netlist = r#"IC Test
.IC N1=5 N2=2.5
.END
"#;
        let result = parse_netlist(netlist).unwrap();

        // IC values stored as parameters with IC_ prefix
        assert!(result.params.get("IC_N1").is_some());
        assert!((result.params.get("IC_N1").unwrap() - 5.0).abs() < 1e-10);
        assert!((result.params.get("IC_N2").unwrap() - 2.5).abs() < 1e-10);
    }

    #[test]
    fn test_parse_veriloga_directive() {
        let netlist = r#"Verilog-A Test
.VERILOGA resistor.va
.END
"#;
        let result = parse_netlist(netlist).unwrap();

        assert_eq!(result.veriloga_includes.len(), 1);
        assert_eq!(
            result.veriloga_includes[0].file_path.to_str().unwrap(),
            "resistor.va"
        );
        assert!(result.veriloga_includes[0].model_name.is_none());
    }

    #[test]
    fn test_parse_veriloga_with_model_name() {
        let netlist = r#"Verilog-A Model Override
.VERILOGA diode.va MyDiode
.END
"#;
        let result = parse_netlist(netlist).unwrap();

        assert_eq!(result.veriloga_includes.len(), 1);
        assert_eq!(
            result.veriloga_includes[0].file_path.to_str().unwrap(),
            "diode.va"
        );
        assert_eq!(
            result.veriloga_includes[0].model_name.as_deref(),
            Some("MyDiode")
        );
    }

    #[test]
    fn test_parse_va_shorthand() {
        let netlist = r#"VA Shorthand
.VA capacitor.va
.END
"#;
        let result = parse_netlist(netlist).unwrap();

        assert_eq!(result.veriloga_includes.len(), 1);
        assert_eq!(
            result.veriloga_includes[0].file_path.to_str().unwrap(),
            "capacitor.va"
        );
    }

    #[test]
    fn test_parse_multiple_veriloga() {
        let netlist = r#"Multiple VA
.VERILOGA resistor.va
.VERILOGA diode.va Diode1N4148
.VA capacitor.va
.END
"#;
        let result = parse_netlist(netlist).unwrap();

        assert_eq!(result.veriloga_includes.len(), 3);
        assert_eq!(
            result.veriloga_includes[0].file_path.to_str().unwrap(),
            "resistor.va"
        );
        assert_eq!(
            result.veriloga_includes[1].file_path.to_str().unwrap(),
            "diode.va"
        );
        assert_eq!(
            result.veriloga_includes[1].model_name.as_deref(),
            Some("Diode1N4148")
        );
        assert_eq!(
            result.veriloga_includes[2].file_path.to_str().unwrap(),
            "capacitor.va"
        );
    }

    #[test]
    fn test_parse_veriloga_with_path() {
        let netlist = r#"VA with Path
.VERILOGA models/varactor.va
.END
"#;
        let result = parse_netlist(netlist).unwrap();

        assert_eq!(result.veriloga_includes.len(), 1);
        assert_eq!(
            result.veriloga_includes[0].file_path.to_str().unwrap(),
            "models/varactor.va"
        );
    }
}
