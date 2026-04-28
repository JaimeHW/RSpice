//! Device and subcircuit element parsers.

use super::*;

pub(super) fn parse_resistor(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
    defer_simple_param_refs: bool,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let node_pos = expect_node(stream, line_num)?;
    let node_neg = expect_node(stream, line_num)?;

    // Skip optional parameter names (R=)
    skip_optional_param_name(stream, "R");

    let mut value: Option<Value> = None;
    let mut value_expr: Option<String> = None;
    let mut model: Option<String> = None;
    let mut instance_params: Vec<(String, Value)> = Vec::new();

    skip_commas(stream);

    // First token after nodes can be:
    // 1) Explicit value (numeric/expression/param ref)
    // 2) Model name
    // 3) First named parameter (e.g. R=, VALUE=, MODEL=, L=, W=...)
    if !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        match &stream.peek().kind {
            TokenKind::Number(_) => {
                value = Some(expect_value(stream, line_num, params)?);
            }
            TokenKind::Expression(_) => {
                if let Some(expr) = take_value_expression_string(stream, params) {
                    if !defer_simple_param_refs && let Some(resolved) = params.get(&expr) {
                        value = Some(resolved);
                    } else {
                        value_expr = Some(expr);
                    }
                }
            }
            TokenKind::Plus | TokenKind::Minus => {
                if matches!(stream.peek_n(1).kind, TokenKind::Expression(_)) {
                    if let Some(expr) = take_value_expression_string(stream, params) {
                        if !defer_simple_param_refs && let Some(resolved) = params.get(&expr) {
                            value = Some(resolved);
                        } else {
                            value_expr = Some(expr);
                        }
                    }
                } else {
                    value = Some(expect_value(stream, line_num, params)?);
                }
            }
            TokenKind::Ident(s) => {
                let ident = s.clone();
                if !defer_simple_param_refs && params.get(&ident).is_some() {
                    stream.advance();
                    value = params.get(&ident);
                } else if let Ok(v) = crate::netlist::lexer::parse_spice_value(&ident) {
                    stream.advance();
                    value = Some(v);
                } else if !matches!(stream.peek_n(1).kind, TokenKind::Equals) {
                    model = Some(ident);
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

                    if (name_upper == "R" || name_upper == "VALUE")
                        && let Some(expr) = take_value_expression_string(stream, params)
                    {
                        if !defer_simple_param_refs && let Some(resolved) = params.get(&expr) {
                            value = Some(resolved);
                            value_expr = None;
                        } else {
                            value_expr = Some(expr);
                            value = None;
                        }
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
                        value_expr = None;
                    }
                    instance_params.push((name_upper, param_value));
                } else if model.is_none() && value.is_none() {
                    // Bare identifier after value-less prefix: treat as model name.
                    model = Some(raw_name);
                }
            }
            TokenKind::Number(_) => {
                // Allow trailing unnamed numeric value as explicit resistance override.
                value = Some(expect_value(stream, line_num, params)?);
                value_expr = None;
            }
            TokenKind::Expression(_) | TokenKind::Plus | TokenKind::Minus => {
                if let Some(expr) = take_value_expression_string(stream, params) {
                    if !defer_simple_param_refs && let Some(resolved) = params.get(&expr) {
                        value = Some(resolved);
                        value_expr = None;
                    } else {
                        value_expr = Some(expr);
                        value = None;
                    }
                } else {
                    value = Some(expect_value(stream, line_num, params)?);
                    value_expr = None;
                }
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

    if value.is_none() && value_expr.is_none() && model.is_none() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "Resistor requires either a value or a model".to_string(),
        });
    }

    elements.push(Element {
        name,
        kind: ElementKind::Resistor {
            value: value.unwrap_or(Value::NAN),
            value_expr,
            model,
            instance_params,
        },
        nodes: vec![node_pos, node_neg],
    });

    Ok(())
}

pub(super) fn parse_capacitor(
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

pub(super) fn parse_inductor(
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

pub(super) fn parse_voltage_source(
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

pub(super) fn parse_current_source(
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

pub(super) fn parse_diode(
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

pub(super) fn parse_bjt(
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

pub(super) fn parse_mosfet(
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

pub(super) fn parse_jfet(
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
pub(super) fn parse_mesfet(
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

pub(super) fn parse_fet_instance_params(
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

                if !area_positional_seen && let Ok(parsed) = raw_name.parse::<f64>() {
                    instance_params.push(("AREA".to_string(), parsed));
                    area_positional_seen = true;
                }
            }
            _ => {
                if !area_positional_seen && let Some(value) = try_value(stream, params) {
                    instance_params.push(("AREA".to_string(), value));
                    area_positional_seen = true;
                    continue;
                }
                stream.advance();
            }
        }
    }

    instance_params
}

/// Parse lossless transmission line (O element)
pub(super) fn parse_lossless_tline(
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
pub(super) fn parse_lossy_tline(
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
pub(super) fn parse_coupled_tlines(
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
pub(super) fn parse_subcircuit_instance(
    line: &str,
    line_num: usize,
    elements: &mut Vec<Element>,
    params_ctx: &ParamContext,
) -> Result<(), ParseError> {
    let fields = split_spice_fields(line);
    if fields.len() < 2 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "Subcircuit instance requires name and subcircuit reference".to_string(),
        });
    }

    let name = fields[0].clone();
    let mut param_start = fields.len();
    for (idx, field) in fields.iter().enumerate().skip(1) {
        if field.eq_ignore_ascii_case("PARAMS")
            || field.eq_ignore_ascii_case("PARAMS:")
            || field.contains('=')
        {
            param_start = idx;
            break;
        }
    }

    if param_start < 2 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "Subcircuit instance requires name and subcircuit reference".to_string(),
        });
    }

    let subckt_name = fields[param_start - 1].clone();
    let nodes = fields[1..param_start - 1]
        .iter()
        .map(|field| field.to_ascii_uppercase())
        .collect::<Vec<_>>();

    // Parse instance parameters: PARAM=value pairs
    let mut params = Vec::new();
    for field in fields.iter().skip(param_start) {
        if field.eq_ignore_ascii_case("PARAMS") || field.eq_ignore_ascii_case("PARAMS:") {
            continue;
        }
        if let Some((param_name, raw_value)) = field.split_once('=') {
            params.push((
                param_name.to_string(),
                parse_parametric_field_value(raw_value, params_ctx),
            ));
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

pub(super) fn parse_vcvs(
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

pub(super) fn parse_cccs(
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

pub(super) fn parse_vccs(
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

pub(super) fn parse_ccvs(
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

pub(super) fn parse_behavioral(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
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
        if behavioral_trailing_assignment(stream) {
            break;
        }
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

    let mut tc1 = 0.0;
    let mut tc2 = 0.0;
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        skip_commas(stream);
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }

        let param_name = expect_ident(stream, line_num)?;
        if !stream.consume(&TokenKind::Equals) {
            continue;
        }
        let param_value = expect_value(stream, line_num, params)?;
        match param_name.as_str() {
            "TC1" => tc1 = param_value,
            "TC2" => tc2 = param_value,
            _ => {}
        }
    }

    let kind = match spec_designator.as_str() {
        "V" => ElementKind::BehavioralVoltage {
            expression,
            tc1,
            tc2,
        },
        "I" => ElementKind::BehavioralCurrent {
            expression,
            tc1,
            tc2,
        },
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

pub(super) fn behavioral_trailing_assignment(stream: &TokenStream) -> bool {
    matches!(
        &stream.peek().kind,
        TokenKind::Ident(name)
            if (name.eq_ignore_ascii_case("TC1") || name.eq_ignore_ascii_case("TC2"))
                && matches!(stream.peek_n(1).kind, TokenKind::Equals)
    )
}

pub(super) fn behavioral_expr_token_fragment(token: &TokenKind) -> Option<String> {
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

/// Parse subcircuit definition: .SUBCKT name ports [PARAMS: p1=v1 p2=v2] or .SUBCKT name ports p1=v1
pub(super) fn parse_subckt_def(line: &str, line_num: usize) -> Result<SubcircuitDef, ParseError> {
    let fields = split_spice_fields(line);
    if fields.len() < 2 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: ".SUBCKT requires a subcircuit name".to_string(),
        });
    }

    let name = fields[1].clone();
    let mut ports = Vec::new();
    let params_ctx = ParamContext::new();

    let mut idx = 2usize;
    while idx < fields.len() {
        let field = &fields[idx];
        if field.eq_ignore_ascii_case("PARAMS") || field.eq_ignore_ascii_case("PARAMS:") {
            idx += 1;
            break;
        }
        if field.contains('=') {
            break;
        }
        ports.push(field.to_ascii_uppercase());
        idx += 1;
    }

    // Parse default parameters: NAME=VALUE pairs
    let mut params = Vec::new();
    while idx < fields.len() {
        let field = &fields[idx];
        if field.eq_ignore_ascii_case("PARAMS") || field.eq_ignore_ascii_case("PARAMS:") {
            idx += 1;
            continue;
        }

        if let Some((param_name, raw_value)) = field.split_once('=') {
            let value = parse_numeric_field_value(raw_value, &params_ctx, line_num)?;
            params.push((param_name.to_string(), value));
        }

        idx += 1;
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
