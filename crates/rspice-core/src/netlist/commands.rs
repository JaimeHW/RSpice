//! Analysis command parsing for SPICE netlists
//!
//! Parses SPICE dot commands:
//! - .OP, .DC, .AC, .TRAN - Analysis types
//! - .MODEL - Device model definitions
//! - .PARAM, .FUNC - Parameters and functions
//! - .STEP, .TEMP - Parametric sweeps
//! - .FOUR, .NOISE - Advanced analysis
//! - .IC, .NODESET - Initial conditions

use super::helpers::{expect_ident, expect_value, skip_commas, try_value};
use super::lexer::{TokenKind, TokenStream};
use super::{
    AnalysisCommand, FreqVariation, ModelDef, ParamContext, ParseError, StepCommand, StepSweep,
    StepTarget,
};
use crate::Value;

/// Parse a SPICE command (lines starting with .)
pub fn parse_command(
    stream: &mut TokenStream,
    line_num: usize,
    analyses: &mut Vec<AnalysisCommand>,
    models: &mut Vec<ModelDef>,
    params: &mut ParamContext,
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

            // Optional second (outer) source: .DC V1 a b s V2 a2 b2 s2
            skip_commas(stream);
            let sweep2 = if matches!(stream.peek().kind, TokenKind::Ident(_)) {
                let source2 = expect_ident(stream, line_num)?;
                let start2 = expect_value(stream, line_num, params)?;
                let stop2 = expect_value(stream, line_num, params)?;
                let step2 = expect_value(stream, line_num, params)?;
                Some(super::DcSecondSweep {
                    source: source2,
                    start: start2,
                    stop: stop2,
                    step: step2,
                    mode: super::DcSweepMode::Linear,
                })
            } else {
                None
            };

            analyses.push(AnalysisCommand::Dc {
                source,
                start,
                stop,
                step,
                mode: super::DcSweepMode::Linear,
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
        ".TRAN" => {
            let step = expect_value(stream, line_num, params)?;
            let stop = expect_value(stream, line_num, params)?;
            let start = try_value(stream, params);
            let max_step = try_value(stream, params);

            let uic = {
                skip_commas(stream);
                if let TokenKind::Ident(word) = &stream.peek().kind {
                    if word.eq_ignore_ascii_case("UIC") {
                        stream.advance();
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            };

            analyses.push(AnalysisCommand::Tran {
                step,
                stop,
                start,
                max_step,
                uic,
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
                expr_params: Vec::new(),
                string_params: Vec::new(),
                string_vector_params: Vec::new(),
                real_vector_params: Vec::new(),
                real_vector_expr_params: Vec::new(),
                integer_vector_params: Vec::new(),
            });
        }
        ".PARAM" | ".CSPARAM" | ".GLOBAL_PARAM" => {
            parse_param_statement(stream, line_num, params)?;
        }
        ".STEP" => {
            let step_cmd = parse_step_command(stream, line_num, params)?;
            analyses.push(AnalysisCommand::Step(step_cmd));
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
        ".IC" => {
            // Parse initial conditions - stored as params for now
            parse_ic_command(stream, params)?;
        }
        ".NODESET" => {
            // Parse nodeset hints - stored as params for now
            parse_nodeset_command(stream, params)?;
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
        _ => {
            // Ignore unknown commands
            log::debug!("Ignoring unknown command: {}", cmd);
        }
    }

    Ok(())
}

/// Parse .PARAM statement: .PARAM name=value [name2=value2 ...]
pub fn parse_param_statement(
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
pub fn parse_func_statement(
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

/// Parse model parameters: (NAME=VALUE NAME=VALUE ...)
pub fn parse_model_params(
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
            } else {
                model_params.push((name, 1.0));
            }
        } else {
            stream.advance(); // Skip unknown token
        }
    }

    // Skip optional closing paren
    stream.consume(&TokenKind::RParen);

    Ok(model_params)
}

/// Parse .STEP command
/// Formats:
/// - .STEP PARAM name start stop increment
/// - .STEP PARAM name LIST v1 v2 v3...
/// - .STEP DEC PARAM name start stop points
/// - .STEP OCT PARAM name start stop points
pub fn parse_step_command(
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
            let name = expect_ident(stream, line_num)?;
            (Some(first_upper), target.to_uppercase(), name)
        }
        "PARAM" | "MODEL" | "TEMP" => {
            let name = expect_ident(stream, line_num)?;
            (None, first_upper, name)
        }
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
        param_name: None,
        sweep,
    })
}

/// Parse .TEMP command: .TEMP t1 [t2 t3...]
pub fn parse_temp_command(
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
pub fn parse_four_command(
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

/// Parse .NOISE command: .NOISE V(out[,ref]) Vsource DEC|LIN|OCT np fstart fstop [pts_per_summary]
pub fn parse_noise_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<AnalysisCommand, ParseError> {
    // Parse output specification V(node) or V(node,ref)
    let output_spec = expect_ident(stream, line_num)?;

    // Parse the output node from V(node) format
    let (output_node, reference_node) = parse_voltage_reference(&output_spec)?;

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

/// Parse .NODESET command: .NODESET V(node1)=val V(node2)=val...
pub fn parse_nodeset_command(
    stream: &mut TokenStream,
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
pub fn parse_ic_command(
    stream: &mut TokenStream,
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
