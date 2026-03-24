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
    MonteCarloCommand, MonteCarloDistribution, Netlist, NodeSet, ParamContext, ParseError,
    PoleZeroAnalysisType, PoleZeroTransferType, SensitivityAcSweep, SourceSpec, StepCommand,
    StepSweep, StepTarget, SubcircuitDef, SwitchState, VerilogAInclude,
};
use crate::Value;

mod command_parsers;
use command_parsers::*;
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
    let mut initial_conditions = Vec::new();
    let mut node_sets = Vec::new();
    let mut global_nodes = std::collections::HashSet::new();
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

        // Strip inline ';' and '$' comments (common SPICE syntax), then trim.
        // We intentionally keep this simple and treat these markers as comment
        // starts only when they appear outside quoted strings.
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
                &mut initial_conditions,
                &mut node_sets,
                &mut global_nodes,
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
        if let Some(include) = parse_veriloga_directive(trimmed) {
            log::debug!("Found .VERILOGA include: {:?}", include.file_path);
            veriloga_includes.push(include);
            continue; // Skip normal processing
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
            &mut initial_conditions,
            &mut node_sets,
            &mut global_nodes,
            &mut measurements,
            &mut options,
        )?;
    }

    Ok(Netlist {
        title,
        elements,
        analyses,
        models,
        subcircuits,
        params,
        initial_conditions,
        node_sets,
        global_nodes,
        measurements,
        options,
        veriloga_includes,
        source_text: Some(input.to_string()),
    })
}

fn strip_inline_semicolon_comment(line: &str) -> &str {
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut escaped = false;

    for (idx, ch) in line.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }

        match ch {
            '\\' if in_single_quote || in_double_quote => {
                escaped = true;
            }
            '\'' if !in_double_quote => {
                in_single_quote = !in_single_quote;
            }
            '"' if !in_single_quote => {
                in_double_quote = !in_double_quote;
            }
            ';' | '$' if !in_single_quote && !in_double_quote => {
                return &line[..idx];
            }
            _ => {}
        }
    }
    line
}

fn parse_veriloga_directive(line: &str) -> Option<VerilogAInclude> {
    let mut parts = line.trim().splitn(2, char::is_whitespace);
    let command = parts.next()?;
    if !command.eq_ignore_ascii_case(".veriloga") && !command.eq_ignore_ascii_case(".va") {
        return None;
    }

    let remainder = parts.next()?.trim();
    if remainder.is_empty() {
        return None;
    }

    let (raw_path, rest) = consume_quoted_or_token(remainder)?;
    let path = raw_path.trim();
    if path.is_empty() {
        return None;
    }

    let model_name = rest
        .split_whitespace()
        .next()
        .map(|s| s.trim_matches(|c| c == '"' || c == '\'').to_string())
        .filter(|s| !s.is_empty());

    Some(VerilogAInclude {
        file_path: std::path::PathBuf::from(path),
        model_name,
    })
}

fn consume_quoted_or_token(input: &str) -> Option<(String, &str)> {
    let trimmed = input.trim_start();
    let first = trimmed.chars().next()?;

    if first == '"' || first == '\'' {
        let quote = first;
        let mut escaped = false;
        let mut value = String::new();
        for (idx, ch) in trimmed.char_indices().skip(1) {
            if escaped {
                value.push(ch);
                escaped = false;
                continue;
            }
            if ch == '\\' {
                escaped = true;
                continue;
            }
            if ch == quote {
                let rest = trimmed[idx + ch.len_utf8()..].trim_start();
                return Some((value, rest));
            }
            value.push(ch);
        }
        // Unclosed quote: consume remaining text as path body.
        return Some((value, ""));
    }

    let end = trimmed.find(char::is_whitespace).unwrap_or(trimmed.len());
    let token = trimmed[..end].to_string();
    let rest = trimmed[end..].trim_start();
    Some((token, rest))
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
    initial_conditions: &mut Vec<InitialCondition>,
    node_sets: &mut Vec<NodeSet>,
    global_nodes: &mut std::collections::HashSet<String>,
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
                initial_conditions,
                node_sets,
                global_nodes,
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
        initial_conditions,
        node_sets,
        global_nodes,
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
    initial_conditions: &mut Vec<InitialCondition>,
    node_sets: &mut Vec<NodeSet>,
    global_nodes: &mut std::collections::HashSet<String>,
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
            initial_conditions,
            node_sets,
            global_nodes,
            measurements,
            options,
        ),
        'R' => parse_resistor(&mut stream, line_num, elements, params),
        'C' => parse_capacitor(&mut stream, line_num, elements, params),
        'L' => parse_inductor(&mut stream, line_num, elements, params),
        'V' => parse_voltage_source(&mut stream, line_num, elements, params),
        'I' => parse_current_source(&mut stream, line_num, elements, params),
        'D' => parse_diode(&mut stream, line_num, elements),
        'Q' => parse_bjt(&mut stream, line_num, elements, params),
        'M' => parse_mosfet(&mut stream, line_num, elements, params),
        'J' => parse_jfet(&mut stream, line_num, elements, params),
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
        'Z' => parse_mesfet(&mut stream, line_num, elements, params),
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
    initial_conditions: &mut Vec<InitialCondition>,
    node_sets: &mut Vec<NodeSet>,
    global_nodes: &mut std::collections::HashSet<String>,
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
            let name = expect_ident(stream, line_num)?;
            let model_type = expect_ident(stream, line_num)?;
            let model_params = parse_model_params(stream, params)?;

            models.push(ModelDef {
                name,
                model_type,
                params: model_params.numeric,
                string_params: model_params.string,
            });
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

fn parse_global_command(
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

    let mut value: Option<Value> = None;
    let mut model: Option<String> = None;
    let mut instance_params: Vec<(String, Value)> = Vec::new();

    skip_commas(stream);

    // First token after nodes can be:
    // 1) Explicit value (numeric/expression/param ref)
    // 2) Model name
    // 3) First named parameter (e.g. R=, VALUE=, MODEL=, L=, W=...)
    if !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        match &stream.peek().kind {
            TokenKind::Number(_)
            | TokenKind::Expression(_)
            | TokenKind::Plus
            | TokenKind::Minus => {
                value = Some(expect_value(stream, line_num, params)?);
            }
            TokenKind::Ident(s) => {
                if let Some(v) = params.get(s) {
                    stream.advance();
                    value = Some(v);
                } else if let Ok(v) = crate::netlist::lexer::parse_spice_value(s) {
                    stream.advance();
                    value = Some(v);
                } else if !matches!(stream.peek_n(1).kind, TokenKind::Equals) {
                    model = Some(s.clone());
                    stream.advance();
                }
            }
            _ => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "Expected resistor value, model name, or parameter assignment, found {:?}",
                        stream.peek().kind
                    ),
                });
            }
        }
    }

    // Parse remaining instance parameters.
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        skip_commas(stream);
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }

        match &stream.peek().kind {
            TokenKind::Ident(name) => {
                let raw_name = name.clone();
                let name_upper = raw_name.to_ascii_uppercase();
                stream.advance();

                if stream.consume(&TokenKind::Equals) {
                    if name_upper == "MODEL" {
                        let model_name = expect_ident(stream, line_num)?;
                        model = Some(model_name);
                        continue;
                    }

                    let param_value =
                        try_value(stream, params).ok_or_else(|| ParseError::Syntax {
                            line: line_num,
                            message: format!(
                                "Expected value for resistor parameter '{}'",
                                raw_name
                            ),
                        })?;

                    if name_upper == "R" || name_upper == "VALUE" {
                        value = Some(param_value);
                    }
                    instance_params.push((name_upper, param_value));
                } else if model.is_none() && value.is_none() {
                    // Bare identifier after value-less prefix: treat as model name.
                    model = Some(raw_name);
                }
            }
            TokenKind::Number(_)
            | TokenKind::Expression(_)
            | TokenKind::Plus
            | TokenKind::Minus => {
                // Allow trailing unnamed numeric value as explicit resistance override.
                value = Some(expect_value(stream, line_num, params)?);
            }
            _ => {
                stream.advance();
            }
        }
    }

    if value.is_none() {
        value = instance_params
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case("R") || k.eq_ignore_ascii_case("VALUE"))
            .map(|(_, v)| *v);
    }

    if value.is_none() && model.is_none() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "Resistor requires either a value or a model".to_string(),
        });
    }

    elements.push(Element {
        name,
        kind: ElementKind::Resistor {
            value: value.unwrap_or(Value::NAN),
            model,
            instance_params,
        },
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
    params: &ParamContext,
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
                    let next_upper = next_ident.to_ascii_uppercase();

                    // Peek ahead: is there an '=' after the next ident?
                    // stream.peek_n(1) would be the token after the current peek
                    if matches!(stream.peek_n(1).kind, TokenKind::Equals)
                        // OFF is an optional BJT instance keyword, not a model name.
                        || next_upper == "OFF"
                    {
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

    let mut instance_params = Vec::new();
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        skip_commas(stream);
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }

        match &stream.peek().kind {
            TokenKind::Ident(raw_name) => {
                let raw_name = raw_name.clone();
                let name_upper = raw_name.to_ascii_uppercase();
                stream.advance();

                if stream.consume(&TokenKind::Equals) {
                    let value = try_value(stream, params).ok_or_else(|| ParseError::Syntax {
                        line: line_num,
                        message: format!("Expected value for BJT parameter '{}'", raw_name),
                    })?;
                    instance_params.push((name_upper, value));
                }
            }
            TokenKind::Number(v) => {
                // Optional positional area scaling.
                instance_params.push(("AREA".to_string(), *v));
                stream.advance();
            }
            _ => {
                stream.advance();
            }
        }
    }

    elements.push(Element {
        name,
        kind: ElementKind::Bjt {
            model,
            bjt_type: super::BjtType::Npn, // Will be set from model
            instance_params,
        },
        nodes,
    });

    Ok(())
}

fn parse_mosfet(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let drain = expect_node(stream, line_num)?;
    let gate = expect_node(stream, line_num)?;
    let source = expect_node(stream, line_num)?;
    let bulk = expect_node(stream, line_num)?;

    // SPICE MOS syntax variants:
    // - 4-node bulk MOS: Mname D G S B model ...
    // - BSIMSOI special form: Mname D G S E [P] [B] [T] model ...
    // Collect all bare tail tokens until explicit instance parameters begin.
    // The final bare token is always the model name; preceding tokens are
    // optional SOI nodes that must be preserved by the parser.
    let mut tail_tokens = Vec::new();
    loop {
        skip_commas(stream);
        if stream.is_eof() || matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }
        if matches!(&stream.peek().kind, TokenKind::Ident(_))
            && !matches!(stream.peek_n(1).kind, TokenKind::Equals)
        {
            tail_tokens.push(expect_node(stream, line_num)?);
            continue;
        }
        break;
    }

    let model = tail_tokens.pop().ok_or_else(|| ParseError::Syntax {
        line: line_num,
        message: "Expected MOSFET model name".to_string(),
    })?;

    let mut nodes = vec![drain, gate, source, bulk];
    nodes.extend(tail_tokens);

    let mut instance_params = Vec::new();
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        skip_commas(stream);
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }

        match &stream.peek().kind {
            TokenKind::Ident(raw_name) => {
                let raw_name = raw_name.clone();
                let name_upper = raw_name.to_ascii_uppercase();
                stream.advance();

                if stream.consume(&TokenKind::Equals) {
                    let value = try_value(stream, params).ok_or_else(|| ParseError::Syntax {
                        line: line_num,
                        message: format!("Expected value for MOSFET parameter '{}'", raw_name),
                    })?;
                    instance_params.push((name_upper, value));
                }
            }
            _ => {
                // Ignore unsupported MOS instance tokens for now.
                stream.advance();
            }
        }
    }

    elements.push(Element {
        name,
        kind: ElementKind::Mosfet {
            model,
            mos_type: super::MosType::Nmos, // Will be set from model
            instance_params,
        },
        nodes,
    });

    Ok(())
}

fn parse_jfet(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let drain = expect_node(stream, line_num)?;
    let gate = expect_node(stream, line_num)?;
    let source = expect_node(stream, line_num)?;
    let model = expect_ident(stream, line_num)?;
    let instance_params = parse_fet_instance_params(stream, line_num, params);

    elements.push(Element {
        name,
        kind: ElementKind::Jfet {
            model,
            jfet_type: super::JfetType::Njf, // Will be set from model
            instance_params,
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
    params: &ParamContext,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let drain = expect_node(stream, line_num)?;
    let gate = expect_node(stream, line_num)?;
    let source = expect_node(stream, line_num)?;
    let model = expect_ident(stream, line_num)?;
    let instance_params = parse_fet_instance_params(stream, line_num, params);

    elements.push(Element {
        name,
        kind: ElementKind::Mesfet {
            model,
            mesfet_type: super::MesfetType::Nmf, // Will be set from model
            instance_params,
        },
        nodes: vec![drain, gate, source],
    });

    Ok(())
}

fn parse_fet_instance_params(
    stream: &mut TokenStream,
    _line_num: usize,
    params: &ParamContext,
) -> Vec<(String, Value)> {
    let mut instance_params = Vec::new();
    let mut area_positional_seen = false;

    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        skip_commas(stream);
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }

        match &stream.peek().kind {
            TokenKind::Ident(raw_name) => {
                let raw_name = raw_name.clone();
                let name_upper = raw_name.to_ascii_uppercase();
                stream.advance();

                if stream.consume(&TokenKind::Equals) {
                    if let Some(value) = try_value(stream, params) {
                        instance_params.push((name_upper, value));
                    }
                    continue;
                }

                if name_upper == "OFF" {
                    continue;
                }

                if !area_positional_seen {
                    if let Ok(parsed) = raw_name.parse::<f64>() {
                        instance_params.push(("AREA".to_string(), parsed));
                        area_positional_seen = true;
                    }
                }
            }
            _ => {
                if !area_positional_seen {
                    if let Some(value) = try_value(stream, params) {
                        instance_params.push(("AREA".to_string(), value));
                        area_positional_seen = true;
                        continue;
                    }
                }
                stream.advance();
            }
        }
    }

    instance_params
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

    // Look for V= or I= form.
    let spec_token = expect_ident(stream, line_num)?;
    let (spec_designator, inline_expr) = if let Some((lhs, rhs)) = spec_token.split_once('=') {
        (lhs.trim().to_ascii_uppercase(), rhs.trim())
    } else {
        (spec_token.trim().to_ascii_uppercase(), "")
    };

    // Consume = if present as a separate token.
    stream.consume(&TokenKind::Equals);

    // Collect the expression text with token-aware reconstruction.
    // Important: TokenKind::Expression already stores the inner content and must
    // not be wrapped back into braces.
    let mut expr_parts = Vec::new();
    if !inline_expr.is_empty() {
        expr_parts.push(inline_expr.to_string());
    }
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        if let Some(fragment) = behavioral_expr_token_fragment(&stream.peek().kind) {
            expr_parts.push(fragment);
        }
        stream.advance();
    }
    let expression = expr_parts.join(" ").trim().to_string();
    if expression.is_empty() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "Behavioral source requires a non-empty expression".to_string(),
        });
    }

    let kind = match spec_designator.as_str() {
        "V" => ElementKind::BehavioralVoltage { expression },
        "I" => ElementKind::BehavioralCurrent { expression },
        _ => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: "Behavioral source must have V=expr or I=expr".to_string(),
            });
        }
    };

    elements.push(Element {
        name,
        kind,
        nodes: vec![node_pos, node_neg],
    });

    Ok(())
}

fn behavioral_expr_token_fragment(token: &TokenKind) -> Option<String> {
    match token {
        TokenKind::Ident(s) => Some(s.clone()),
        TokenKind::Number(n) => Some(format!("{}", n)),
        TokenKind::StringLit(s) => Some(s.clone()),
        TokenKind::Expression(expr) => Some(expr.clone()),
        TokenKind::Equals => Some("=".to_string()),
        TokenKind::Comma => Some(",".to_string()),
        TokenKind::LParen => Some("(".to_string()),
        TokenKind::RParen => Some(")".to_string()),
        TokenKind::Plus => Some("+".to_string()),
        TokenKind::Minus => Some("-".to_string()),
        TokenKind::Star => Some("*".to_string()),
        TokenKind::Slash => Some("/".to_string()),
        TokenKind::AtSign => Some("@".to_string()),
        TokenKind::LBracket => Some("[".to_string()),
        TokenKind::RBracket => Some("]".to_string()),
        TokenKind::Newline | TokenKind::Eof => None,
    }
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

                    let mut ac_terms: Option<(Value, Value)> = None;

                    // Optional AC specification after DC
                    skip_commas(stream);
                    if let TokenKind::Ident(next) = &stream.peek().kind {
                        if next.to_uppercase() == "AC" {
                            stream.advance();
                            let ac_magnitude = try_value(stream, params).unwrap_or(1.0);
                            // SPICE AC phase is specified in degrees; store radians internally.
                            let ac_phase = try_value(stream, params).unwrap_or(0.0).to_radians();
                            ac_terms = Some((ac_magnitude, ac_phase));
                        }
                    }

                    let transient = parse_transient_source_spec_keyword(stream, line_num, params)?;
                    return Ok(match (ac_terms, transient) {
                        (Some((ac_magnitude, ac_phase)), Some(transient)) => {
                            SourceSpec::DcAcTransient {
                                dc_value,
                                ac_magnitude,
                                ac_phase,
                                transient: Box::new(transient),
                            }
                        }
                        (Some((ac_magnitude, ac_phase)), None) => SourceSpec::DcAc {
                            dc_value,
                            ac_magnitude,
                            ac_phase,
                        },
                        (None, Some(transient)) => SourceSpec::DcTransient {
                            dc_value,
                            transient: Box::new(transient),
                        },
                        (None, None) => SourceSpec::Dc(dc_value),
                    });
                }
                "AC" => {
                    stream.advance();
                    // AC magnitude is optional - defaults to 1.0 if not specified
                    let ac_magnitude = try_value(stream, params).unwrap_or(1.0);
                    // SPICE AC phase is specified in degrees; store radians internally.
                    let ac_phase = try_value(stream, params).unwrap_or(0.0).to_radians();

                    // Support ngspice ordering like:
                    //   AC 1 DC 0 SIN(...)
                    // by accepting optional DC and transient terms after AC.
                    skip_commas(stream);
                    let mut dc_value = 0.0;
                    let mut has_dc_term = false;
                    if let TokenKind::Ident(next) = &stream.peek().kind {
                        if next.to_uppercase() == "DC" {
                            stream.advance();
                            skip_commas(stream);
                            stream.consume(&TokenKind::Equals);
                            dc_value = expect_value(stream, line_num, params)?;
                            has_dc_term = true;
                        }
                    }

                    let transient = parse_transient_source_spec_keyword(stream, line_num, params)?;
                    return Ok(match transient {
                        Some(transient) => SourceSpec::DcAcTransient {
                            dc_value,
                            ac_magnitude,
                            ac_phase,
                            transient: Box::new(transient),
                        },
                        None if has_dc_term => SourceSpec::DcAc {
                            dc_value,
                            ac_magnitude,
                            ac_phase,
                        },
                        None => SourceSpec::Ac {
                            magnitude: ac_magnitude,
                            phase: ac_phase,
                        },
                    });
                }
                "PULSE" => {
                    stream.advance();
                    return parse_pulse_spec(stream, line_num, params);
                }
                "SIN" | "SINE" => {
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
    if let Some(transient) = parse_transient_source_spec_keyword(stream, line_num, params)? {
        Ok(SourceSpec::DcTransient {
            dc_value: value,
            transient: Box::new(transient),
        })
    } else {
        Ok(SourceSpec::Dc(value))
    }
}

fn parse_transient_source_spec_keyword(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<Option<SourceSpec>, ParseError> {
    skip_commas(stream);
    let TokenKind::Ident(keyword) = &stream.peek().kind else {
        return Ok(None);
    };

    match keyword.to_uppercase().as_str() {
        "PULSE" => {
            stream.advance();
            parse_pulse_spec(stream, line_num, params).map(Some)
        }
        "SIN" => {
            stream.advance();
            parse_sin_spec(stream, line_num, params).map(Some)
        }
        "SINE" => {
            stream.advance();
            parse_sin_spec(stream, line_num, params).map(Some)
        }
        "PWL" => {
            stream.advance();
            parse_pwl_spec(stream, line_num, params).map(Some)
        }
        "EXP" => {
            stream.advance();
            parse_exp_spec(stream, line_num, params).map(Some)
        }
        _ => Ok(None),
    }
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
    // Keep omitted timing fields as NaN sentinels so transient runtime can
    // resolve ngspice-compatible defaults from .TRAN context (tstep/tstop).
    let rise = try_value(stream, params).unwrap_or(Value::NAN);
    let fall = try_value(stream, params).unwrap_or(Value::NAN);
    let width = try_value(stream, params).unwrap_or(Value::NAN);
    let period = try_value(stream, params).unwrap_or(Value::NAN);

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

struct ParsedModelParams {
    numeric: Vec<(String, Value)>,
    string: Vec<(String, String)>,
}

fn parse_model_params(
    stream: &mut TokenStream,
    params: &ParamContext,
) -> Result<ParsedModelParams, ParseError> {
    let mut numeric_params = Vec::new();
    let mut string_params = Vec::new();

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
                match &stream.peek().kind {
                    TokenKind::StringLit(value) => {
                        let value = value.clone();
                        stream.advance();
                        string_params.push((name, value));
                    }
                    _ => {
                        if let Some(value) = try_value(stream, params) {
                            numeric_params.push((name, value));
                        }
                    }
                }
            }
        } else {
            stream.advance(); // Skip unknown token
        }
    }

    // Skip optional closing paren
    stream.consume(&TokenKind::RParen);

    Ok(ParsedModelParams {
        numeric: numeric_params,
        string: string_params,
    })
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
            } else if let Ok(v) = crate::netlist::lexer::parse_spice_value(s) {
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
            } else if let Ok(v) = crate::netlist::lexer::parse_spice_value(s) {
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
// Tests
//=============================================================================

#[cfg(test)]
mod tests;
