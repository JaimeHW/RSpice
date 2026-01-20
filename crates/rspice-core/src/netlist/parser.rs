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
    AnalysisCommand, Element, ElementKind, FreqVariation, InitialCondition, ModelDef, Netlist,
    ParamContext, ParseError, SourceSpec, StepCommand, StepSweep, StepTarget, SubcircuitDef,
    SwitchState, VerilogAInclude,
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

    // State for tracking subcircuit blocks
    let mut in_subcircuit = false;
    let mut current_subckt: Option<SubcircuitDef> = None;

    let mut line_num = 1;
    let mut continuation = String::new();

    for line in lines.iter().skip(1) {
        line_num += 1;

        // Skip empty lines and comments
        let trimmed = line.trim();
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
        measurements: Vec::new(),
        options: super::SimulationOptions::default(),
        veriloga_includes,
    })
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

            parse_line(
                line,
                line_num,
                &mut subckt_elements,
                analyses,
                models,
                &mut subckt_params,
            )?;
            subckt.elements.extend(subckt_elements);
        }
        return Ok(());
    }

    // Normal element/command parsing
    parse_line(line, line_num, elements, analyses, models, params)
}

fn parse_line(
    line: &str,
    line_num: usize,
    elements: &mut Vec<Element>,
    analyses: &mut Vec<AnalysisCommand>,
    models: &mut Vec<ModelDef>,
    params: &mut ParamContext,
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
        '.' => parse_command(&mut stream, line_num, analyses, models, params),
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
        'O' => parse_lossless_tline(&mut stream, line_num, elements),
        'Y' => parse_lossy_tline(&mut stream, line_num, elements),
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
            // .OPTIONS are parsed but stored in Netlist, not here
            // Just consume the line - parsing happens at higher level
            log::debug!(".OPTIONS found at line {}", line_num);
        }
        _ => {
            // Ignore unknown commands
            log::debug!("Ignoring unknown command: {}", cmd);
        }
    }

    Ok(())
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
        // LTspice-style: .FUNC name(args) {expression}
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
                TokenKind::Ident(_) => {
                    // Two identifiers in a row: first is substrate node, second is model
                    let model = expect_ident(stream, line_num)?;
                    (Some(first_ident), model)
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
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let port1_pos = expect_node(stream, line_num)?;
    let port1_neg = expect_node(stream, line_num)?;
    let port2_pos = expect_node(stream, line_num)?;
    let port2_neg = expect_node(stream, line_num)?;

    // For now, skip to end of line (unsupported, but parsed)
    stream.skip_to_eol();

    elements.push(Element {
        name,
        kind: ElementKind::TransmissionLine {
            z0: 50.0, // Default
            td: Some(1e-9),
            freq: None,
            nl: None,
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
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let port1_pos = expect_node(stream, line_num)?;
    let port1_neg = expect_node(stream, line_num)?;
    let port2_pos = expect_node(stream, line_num)?;
    let port2_neg = expect_node(stream, line_num)?;

    // For now, skip to end of line (unsupported, but parsed)
    stream.skip_to_eol();

    elements.push(Element {
        name,
        kind: ElementKind::TransmissionLine {
            z0: 50.0,
            td: Some(1e-9),
            freq: None,
            nl: None,
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

    // Skip rest of line
    stream.skip_to_eol();

    elements.push(Element {
        name,
        kind: ElementKind::TransmissionLine {
            z0: 50.0,
            td: Some(1e-9),
            freq: None,
            nl: None,
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
                    let value = expect_value(stream, line_num, params)?;
                    return Ok(SourceSpec::Dc(value));
                }
                "AC" => {
                    stream.advance();
                    // AC magnitude is optional - defaults to 1.0 if not specified
                    let magnitude = try_value(stream, params).unwrap_or(1.0);
                    let phase = try_value(stream, params).unwrap_or(0.0);
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
    let phase = expect_value_default(stream, params, 0.0);

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
    _line_num: usize,
    params: &ParamContext,
) -> Result<SourceSpec, ParseError> {
    let has_paren = stream.consume(&TokenKind::LParen);

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

    // Parse parameters (Z0, TD, F, NL)
    let mut z0: Option<Value> = None;
    let mut td: Option<Value> = None;
    let mut freq: Option<Value> = None;
    let mut nl: Option<Value> = None;

    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        skip_commas(stream);

        if let TokenKind::Ident(param) = &stream.peek().kind {
            let param_upper = param.to_uppercase();
            stream.advance();

            // Consume = if present
            stream.consume(&TokenKind::Equals);

            match param_upper.as_str() {
                "Z0" | "ZO" => {
                    z0 = try_value(stream, params);
                }
                "TD" => {
                    td = try_value(stream, params);
                }
                "F" | "FREQ" => {
                    freq = try_value(stream, params);
                }
                "NL" => {
                    nl = try_value(stream, params);
                }
                _ => {
                    // Skip unknown parameter
                    try_value(stream, params);
                }
            }
        } else if let Some(v) = try_value(stream, params) {
            // Positional Z0
            if z0.is_none() {
                z0 = Some(v);
            } else if td.is_none() {
                td = Some(v);
            }
        } else {
            stream.advance(); // Skip unknown token
        }
    }

    let z0 = z0.ok_or_else(|| ParseError::Syntax {
        line: line_num,
        message: "Transmission line requires Z0".to_string(),
    })?;

    elements.push(Element {
        name,
        kind: ElementKind::TransmissionLine { z0, td, freq, nl },
        nodes: vec![port1_pos, port1_neg, port2_pos, port2_neg],
    });

    Ok(())
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
            let name = expect_ident(stream, line_num)?;
            (Some(first_upper), target.to_uppercase(), name)
        }
        "PARAM" | "MODEL" | "TEMP" => {
            let name = expect_ident(stream, line_num)?;
            (None, first_upper, name)
        }
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
    // Parse output specification - could be V(node), V(node,ref), or just node
    let first = expect_ident(stream, line_num)?;

    let (output_node, reference_node) =
        if first.to_uppercase() == "V" && matches!(stream.peek().kind, TokenKind::LParen) {
            // Consume LP, then parse V(node) or V(node,ref)
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
            (node, reference)
        } else if first.to_uppercase().starts_with("V(") {
            // Already parsed as V(node) string
            parse_voltage_reference(&first)?
        } else {
            // Simple node name
            (first, None)
        };

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
            ElementKind::TransmissionLine { z0, td, freq, nl } => {
                assert!((*z0 - 50.0).abs() < 1e-10);
                assert!(td.is_some());
                assert!((td.unwrap() - 1e-9).abs() < 1e-20);
                assert!(freq.is_none());
                assert!(nl.is_none());
            }
            _ => panic!("Expected TransmissionLine element"),
        }
        assert_eq!(result.elements[0].nodes.len(), 4);
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
