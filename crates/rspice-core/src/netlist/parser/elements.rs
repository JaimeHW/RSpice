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

    let mut instance_params = instance_params;
    let mut nodes = vec![node_pos, node_neg];
    expand_passive_parasitics(elements, &name, &mut nodes, &mut instance_params);
    elements.push(Element {
        name,
        kind: ElementKind::Resistor {
            value: value.unwrap_or(Value::NAN),
            value_expr,
            model,
            instance_params,
        },
        nodes,
    });

    Ok(())
}

/// Shared value/model/parameter tail for capacitors and inductors.
///
/// Accepts the same instance grammar SPICE dialects use for passives:
/// an optional leading value (`1u`, `{expr}`, a parameter reference), an
/// optional bare model name, and named `PARAM=value` assignments. `IC` and
/// `MODEL` are extracted specially; every other assignment is preserved as
/// an instance parameter for build-time resolution (M/SCALE/TC1/TC2/W/L...).
struct PassiveTail {
    value: Option<Value>,
    model: Option<String>,
    ic: Option<Value>,
    instance_params: Vec<(String, Value)>,
}

/// Remove one instance parameter by name, returning its value.
fn extract_instance_param(params: &mut Vec<(String, Value)>, key: &str) -> Option<Value> {
    let idx = params
        .iter()
        .position(|(name, _)| name.eq_ignore_ascii_case(key))?;
    Some(params.remove(idx).1)
}

/// LTspice-style passive parasitics: peel explicit `Rser`/`Rpar`/`Cpar`
/// instance parameters off an R/L/C and synthesize them as real elements
/// (`Rser` inserts an internal series node `NAME#SER`). Only *explicit*
/// parameters expand — LTspice's implicit defaults are never imposed on
/// a deck that didn't ask for them.
fn expand_passive_parasitics(
    elements: &mut Vec<Element>,
    name: &str,
    nodes: &mut [String],
    instance_params: &mut Vec<(String, Value)>,
) {
    let tag = name.to_ascii_uppercase();

    if let Some(rser) =
        extract_instance_param(instance_params, "RSER").filter(|v| v.is_finite() && *v > 0.0)
    {
        let internal = format!("{tag}#SER");
        let outer_pos = std::mem::replace(&mut nodes[0], internal.clone());
        elements.push(Element {
            name: format!("R{tag}#SER"),
            kind: ElementKind::Resistor {
                value: rser,
                value_expr: None,
                model: None,
                instance_params: Vec::new(),
            },
            nodes: vec![outer_pos, internal],
        });
    }

    if let Some(rpar) =
        extract_instance_param(instance_params, "RPAR").filter(|v| v.is_finite() && *v > 0.0)
    {
        elements.push(Element {
            name: format!("R{tag}#PAR"),
            kind: ElementKind::Resistor {
                value: rpar,
                value_expr: None,
                model: None,
                instance_params: Vec::new(),
            },
            nodes: vec![nodes[0].clone(), nodes[1].clone()],
        });
    }

    if let Some(cpar) =
        extract_instance_param(instance_params, "CPAR").filter(|v| v.is_finite() && *v > 0.0)
    {
        elements.push(Element {
            name: format!("C{tag}#PAR"),
            kind: ElementKind::Capacitor {
                value: cpar,
                initial_voltage: None,
                model: None,
                instance_params: Vec::new(),
            },
            nodes: vec![nodes[0].clone(), nodes[1].clone()],
        });
    }
}

fn parse_passive_tail(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    element_label: &str,
    value_keys: &[&str],
) -> Result<PassiveTail, ParseError> {
    let mut tail = PassiveTail {
        value: None,
        model: None,
        ic: None,
        instance_params: Vec::new(),
    };

    skip_commas(stream);

    // Optional leading positional value or model name.
    if !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        match &stream.peek().kind {
            TokenKind::Number(_) | TokenKind::Expression(_) | TokenKind::Plus | TokenKind::Minus => {
                tail.value = Some(expect_value(stream, line_num, params)?);
            }
            TokenKind::Ident(s) => {
                let ident = s.clone();
                if let Some(resolved) = params.get(&ident) {
                    stream.advance();
                    tail.value = Some(resolved);
                } else if let Ok(v) = crate::netlist::lexer::parse_spice_value(&ident) {
                    stream.advance();
                    tail.value = Some(v);
                } else if !matches!(stream.peek_n(1).kind, TokenKind::Equals) {
                    tail.model = Some(ident);
                    stream.advance();
                }
            }
            _ => {}
        }
    }

    // Remaining named parameters (and a possible bare model name).
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
                    if name_upper == "MODEL" {
                        tail.model = Some(expect_ident(stream, line_num)?);
                        continue;
                    }

                    let param_value =
                        try_value(stream, params).ok_or_else(|| ParseError::Syntax {
                            line: line_num,
                            message: format!(
                                "Expected value for {} parameter '{}'",
                                element_label, raw_name
                            ),
                        })?;

                    if name_upper == "IC" {
                        tail.ic = Some(param_value);
                        continue;
                    }
                    if value_keys.iter().any(|key| name_upper == *key) {
                        tail.value = Some(param_value);
                        continue;
                    }
                    tail.instance_params.push((name_upper, param_value));
                } else if tail.model.is_none() && tail.value.is_none() {
                    tail.model = Some(raw_name);
                }
            }
            TokenKind::Number(_) => {
                tail.value = Some(expect_value(stream, line_num, params)?);
            }
            _ => {
                stream.advance();
            }
        }
    }

    Ok(tail)
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
    let tail = parse_passive_tail(stream, line_num, params, "capacitor", &["C", "VALUE", "CAP"])?;

    if tail.value.is_none() && tail.model.is_none() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "Capacitor requires either a value or a model".to_string(),
        });
    }

    let mut instance_params = tail.instance_params;
    let mut nodes = vec![node_pos, node_neg];
    expand_passive_parasitics(elements, &name, &mut nodes, &mut instance_params);
    elements.push(Element {
        name,
        kind: ElementKind::Capacitor {
            value: tail.value.unwrap_or(Value::NAN),
            initial_voltage: tail.ic,
            model: tail.model,
            instance_params,
        },
        nodes,
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
    let tail = parse_passive_tail(stream, line_num, params, "inductor", &["L", "VALUE", "IND"])?;

    if tail.value.is_none() && tail.model.is_none() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "Inductor requires either a value or a model".to_string(),
        });
    }

    // Magnetic-core (Jiles-Atherton) vs linear model-card dispatch happens at
    // circuit-build time based on the referenced model's type; the parser
    // records the reference only.
    let mut instance_params = tail.instance_params;
    let mut nodes = vec![node_pos, node_neg];
    expand_passive_parasitics(elements, &name, &mut nodes, &mut instance_params);
    elements.push(Element {
        name,
        kind: ElementKind::Inductor {
            value: tail.value.unwrap_or(Value::NAN),
            initial_current: tail.ic,
            model: tail.model,
            instance_params,
        },
        nodes,
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
    params: &ParamContext,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let anode = expect_node(stream, line_num)?;
    let cathode = expect_node(stream, line_num)?;
    let model = expect_ident(stream, line_num)?;

    // Instance tail: positional AREA, bare OFF keyword, and PARAM=value
    // assignments (AREA/M/PJ/TEMP/DTEMP/IC...), mirroring ngspice's D-line
    // grammar.
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

                if name_upper == "OFF" && !matches!(stream.peek().kind, TokenKind::Equals) {
                    instance_params.push(("OFF".to_string(), 1.0));
                    continue;
                }

                if stream.consume(&TokenKind::Equals) {
                    let value = try_value(stream, params).ok_or_else(|| ParseError::Syntax {
                        line: line_num,
                        message: format!("Expected value for diode parameter '{}'", raw_name),
                    })?;
                    instance_params.push((name_upper, value));
                }
            }
            TokenKind::Number(v) => {
                if !area_positional_seen {
                    instance_params.push(("AREA".to_string(), *v));
                    area_positional_seen = true;
                }
                stream.advance();
            }
            _ => {
                stream.advance();
            }
        }
    }

    elements.push(Element {
        name,
        kind: ElementKind::Diode {
            model,
            instance_params,
        },
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

                if name_upper == "OFF" && !matches!(stream.peek().kind, TokenKind::Equals) {
                    instance_params.push(("OFF".to_string(), 1.0));
                    continue;
                }

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

//=============================================================================
// Extended controlled-source forms (POLY / VALUE / TABLE)
//=============================================================================

/// Which extended controlled-source keyword starts at the stream cursor.
enum ControlledSourceForm {
    Poly(usize),
    Value,
    Table,
    Laplace,
    Freq,
}

/// Detect (and consume) an extended controlled-source keyword.
///
/// Handles both token shapes the lexer can produce: `POLY ( 2 )` as separate
/// tokens and `POLY(2)` glued into one identifier.
fn try_controlled_source_form(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<Option<ControlledSourceForm>, ParseError> {
    let TokenKind::Ident(raw) = &stream.peek().kind else {
        return Ok(None);
    };
    let upper = raw.to_ascii_uppercase();

    if let Some(rest) = upper.strip_prefix("POLY") {
        let rest = rest.trim();
        // Glued form: POLY(2)
        if let Some(dims) = rest
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| s.trim().parse::<usize>().ok())
        {
            stream.advance();
            return Ok(Some(ControlledSourceForm::Poly(dims)));
        }
        // Split form: POLY ( 2 )
        if rest.is_empty() && matches!(stream.peek_n(1).kind, TokenKind::LParen) {
            stream.advance(); // POLY
            stream.advance(); // (
            let dims = match &stream.peek().kind {
                TokenKind::Number(n) if *n >= 1.0 && n.fract() == 0.0 => *n as usize,
                other => {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!(
                            "POLY dimension must be a positive integer, found {:?}",
                            other
                        ),
                    });
                }
            };
            stream.advance(); // dimension
            if !stream.consume(&TokenKind::RParen) {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: "Expected ')' after POLY dimension".to_string(),
                });
            }
            return Ok(Some(ControlledSourceForm::Poly(dims)));
        }
        return Ok(None);
    }

    let form = match upper.as_str() {
        "VALUE" => Some(ControlledSourceForm::Value),
        "TABLE" => Some(ControlledSourceForm::Table),
        "LAPLACE" => Some(ControlledSourceForm::Laplace),
        "FREQ" => Some(ControlledSourceForm::Freq),
        _ => None,
    };
    if form.is_some() {
        stream.advance();
        stream.consume(&TokenKind::Equals);
    }
    Ok(form)
}

/// Collect a signed numeric list (POLY coefficients, TABLE pairs) to end of
/// line, tolerating commas and parentheses as pair decoration.
fn collect_numeric_tail(
    stream: &mut TokenStream,
    params: &ParamContext,
) -> Vec<Value> {
    let mut values = Vec::new();
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        match &stream.peek().kind {
            TokenKind::Comma | TokenKind::LParen | TokenKind::RParen => {
                stream.advance();
            }
            TokenKind::Minus => {
                stream.advance();
                if let Some(value) = try_value(stream, params) {
                    values.push(-value);
                }
            }
            TokenKind::Plus => {
                stream.advance();
            }
            _ => {
                if let Some(value) = try_value(stream, params) {
                    values.push(value);
                } else {
                    stream.advance();
                }
            }
        }
    }
    values
}

/// Reconstruct a brace expression argument (`VALUE={...}` / `TABLE {...}`),
/// accepting either a single Expression token or a bare token run.
fn collect_expression_argument(
    stream: &mut TokenStream,
    line_num: usize,
    terminator: Option<&TokenKind>,
) -> Result<String, ParseError> {
    if let TokenKind::Expression(inner) = &stream.peek().kind {
        let inner = inner.clone();
        stream.advance();
        return Ok(inner);
    }

    let mut parts = Vec::new();
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        if let Some(term) = terminator
            && std::mem::discriminant(&stream.peek().kind) == std::mem::discriminant(term)
        {
            break;
        }
        if let Some(fragment) = behavioral_expr_token_fragment(&stream.peek().kind) {
            parts.push(fragment);
        }
        stream.advance();
    }

    let expression = parts.join(" ").trim().to_string();
    if expression.is_empty() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "Controlled source requires a non-empty expression".to_string(),
        });
    }
    Ok(expression)
}

/// Build the polynomial expression for SPICE2-style `POLY(n)` sources.
///
/// Monomials follow the graded ordering every PSpice-derived deck assumes:
/// degree 0 (`p0`), then linear terms `v1..vn`, then within each higher
/// degree the exponent tuples in lexicographically descending order —
/// for `POLY(2)`: `p3*v1^2 + p4*v1*v2 + p5*v2^2`, etc.
///
/// SPICE2 special case: a single coefficient is `p1` (a pure linear gain on
/// the first controlling variable), not a constant.
fn poly_expression(vars: &[String], coeffs: &[Value]) -> String {
    fn push_monomials(
        remaining_vars: &[String],
        degree: usize,
        prefix: &mut Vec<(usize, usize)>, // (var index offset into vars, exponent)
        out: &mut Vec<Vec<(usize, usize)>>,
        base_index: usize,
    ) {
        if remaining_vars.len() == 1 {
            let mut term = prefix.clone();
            if degree > 0 {
                term.push((base_index, degree));
            }
            out.push(term);
            return;
        }
        for first_exp in (0..=degree).rev() {
            let mut term_prefix = prefix.clone();
            if first_exp > 0 {
                term_prefix.push((base_index, first_exp));
            }
            push_monomials(
                &remaining_vars[1..],
                degree - first_exp,
                &mut term_prefix,
                out,
                base_index + 1,
            );
        }
    }

    if coeffs.is_empty() {
        return "0".to_string();
    }
    if coeffs.len() == 1 {
        // SPICE2 rule: a lone coefficient is the linear gain on v1.
        return format!("({})*({})", coeffs[0], vars[0]);
    }

    // Enumerate monomials degree by degree until coefficients are exhausted.
    let mut terms: Vec<String> = Vec::new();
    let mut coeff_idx = 0usize;
    let mut degree = 0usize;
    while coeff_idx < coeffs.len() {
        let mut monomials = Vec::new();
        let mut prefix = Vec::new();
        push_monomials(vars, degree, &mut prefix, &mut monomials, 0);
        for monomial in monomials {
            if coeff_idx >= coeffs.len() {
                break;
            }
            let coeff = coeffs[coeff_idx];
            coeff_idx += 1;
            if coeff == 0.0 {
                continue;
            }
            let mut factors = vec![format!("({})", coeff)];
            for (var_idx, exponent) in &monomial {
                for _ in 0..*exponent {
                    factors.push(format!("({})", vars[*var_idx]));
                }
            }
            terms.push(factors.join("*"));
        }
        degree += 1;
        // Safety valve: coefficients beyond degree 8 in n vars would be a
        // pathological deck; stop rather than loop unbounded.
        if degree > 8 {
            break;
        }
    }

    if terms.is_empty() {
        "0".to_string()
    } else {
        terms.join(" + ")
    }
}

/// Build the clamped TABLE transfer expression.
///
/// PSpice TABLE sources clamp to the endpoint outputs outside the listed
/// range; clamping the *input* into `[x_first, x_last]` reproduces that
/// exactly with the runtime's interpolating `table()` function.
fn table_transfer_expression(
    input_expr: &str,
    pairs: &[(Value, Value)],
) -> String {
    let mut sorted = pairs.to_vec();
    sorted.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    let x_min = sorted.first().map(|(x, _)| *x).unwrap_or(0.0);
    let x_max = sorted.last().map(|(x, _)| *x).unwrap_or(0.0);

    let mut args = Vec::with_capacity(1 + sorted.len() * 2);
    args.push(format!("limit(({}), {}, {})", input_expr, x_min, x_max));
    for (x, y) in &sorted {
        args.push(format!("{}", x));
        args.push(format!("{}", y));
    }
    format!("table({})", args.join(", "))
}

fn unsupported_form_error(
    line_num: usize,
    element: &str,
    form: &str,
) -> ParseError {
    ParseError::Syntax {
        line: line_num,
        message: format!(
            "{} {} sources are not supported yet; supported extended forms are \
             POLY(n), VALUE={{expr}}, and TABLE {{expr}} = (x,y) pairs",
            element, form
        ),
    }
}

/// Shared implementation for E (VCVS) and G (VCCS) parsing, covering the
/// linear, POLY, VALUE, and TABLE forms. Extended forms lower onto the
/// behavioral-source engine, which provides Newton linearization, AC
/// small-signal handling, and hierarchical expression remapping.
fn parse_voltage_controlled_source(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
    is_voltage_output: bool,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let node_pos = expect_node(stream, line_num)?;
    let node_neg = expect_node(stream, line_num)?;

    let element_label = if is_voltage_output { "E (VCVS)" } else { "G (VCCS)" };

    let lower_behavioral = |expression: String| -> ElementKind {
        if is_voltage_output {
            ElementKind::BehavioralVoltage {
                expression,
                tc1: 0.0,
                tc2: 0.0,
            }
        } else {
            ElementKind::BehavioralCurrent {
                expression,
                tc1: 0.0,
                tc2: 0.0,
            }
        }
    };

    match try_controlled_source_form(stream, line_num)? {
        Some(ControlledSourceForm::Poly(dims)) => {
            let mut vars = Vec::with_capacity(dims);
            for _ in 0..dims {
                let cp = expect_node(stream, line_num)?;
                let cn = expect_node(stream, line_num)?;
                vars.push(format!("V({},{})", cp, cn));
            }
            let coeffs = collect_numeric_tail(stream, params);
            if coeffs.is_empty() {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "{} POLY({}) requires at least one coefficient",
                        element_label, dims
                    ),
                });
            }
            elements.push(Element {
                name,
                kind: lower_behavioral(poly_expression(&vars, &coeffs)),
                nodes: vec![node_pos, node_neg],
            });
        }
        Some(ControlledSourceForm::Value) => {
            let expression = collect_expression_argument(stream, line_num, None)?;
            elements.push(Element {
                name,
                kind: lower_behavioral(expression),
                nodes: vec![node_pos, node_neg],
            });
        }
        Some(ControlledSourceForm::Table) => {
            let input_expr =
                collect_expression_argument(stream, line_num, Some(&TokenKind::Equals))?;
            stream.consume(&TokenKind::Equals);
            let flat = collect_numeric_tail(stream, params);
            if flat.len() < 4 || flat.len() % 2 != 0 {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "{} TABLE requires at least two (x,y) pairs",
                        element_label
                    ),
                });
            }
            let pairs: Vec<(Value, Value)> =
                flat.chunks_exact(2).map(|c| (c[0], c[1])).collect();
            elements.push(Element {
                name,
                kind: lower_behavioral(table_transfer_expression(&input_expr, &pairs)),
                nodes: vec![node_pos, node_neg],
            });
        }
        Some(ControlledSourceForm::Laplace) => {
            // LAPLACE {input} = {N(s)/D(s)} — synthesized at parse time into
            // an exact state-space realization (grounded caps + behavioral
            // sources), so every analysis handles it with existing devices.
            let input_expr =
                collect_expression_argument(stream, line_num, Some(&TokenKind::Equals))?;
            stream.consume(&TokenKind::Equals);
            let rational_text = collect_expression_argument(stream, line_num, None)?;
            let synthesized = synthesize_laplace(
                &name,
                &node_pos,
                &node_neg,
                &input_expr,
                &rational_text,
                is_voltage_output,
                line_num,
            )?;
            elements.extend(synthesized);
        }
        Some(ControlledSourceForm::Freq) => {
            return Err(unsupported_form_error(line_num, element_label, "FREQ"));
        }
        None => {
            let ctrl_pos = expect_node(stream, line_num)?;
            let ctrl_neg = expect_node(stream, line_num)?;
            let gain = expect_value(stream, line_num, params)?;
            let kind = if is_voltage_output {
                ElementKind::Vcvs {
                    gain,
                    control_nodes: (ctrl_pos, ctrl_neg),
                }
            } else {
                ElementKind::Vccs {
                    transconductance: gain,
                    control_nodes: (ctrl_pos, ctrl_neg),
                }
            };
            elements.push(Element {
                name,
                kind,
                nodes: vec![node_pos, node_neg],
            });
        }
    }

    Ok(())
}

/// Shared implementation for F (CCCS) and H (CCVS) parsing, covering the
/// linear and POLY forms (controlling variables are branch currents of named
/// voltage sources).
fn parse_current_controlled_source(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
    is_voltage_output: bool,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let node_pos = expect_node(stream, line_num)?;
    let node_neg = expect_node(stream, line_num)?;

    let element_label = if is_voltage_output { "H (CCVS)" } else { "F (CCCS)" };

    match try_controlled_source_form(stream, line_num)? {
        Some(ControlledSourceForm::Poly(dims)) => {
            let mut vars = Vec::with_capacity(dims);
            for _ in 0..dims {
                let source = expect_ident(stream, line_num)?;
                vars.push(format!("I({})", source));
            }
            let coeffs = collect_numeric_tail(stream, params);
            if coeffs.is_empty() {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "{} POLY({}) requires at least one coefficient",
                        element_label, dims
                    ),
                });
            }
            let expression = poly_expression(&vars, &coeffs);
            let kind = if is_voltage_output {
                ElementKind::BehavioralVoltage {
                    expression,
                    tc1: 0.0,
                    tc2: 0.0,
                }
            } else {
                ElementKind::BehavioralCurrent {
                    expression,
                    tc1: 0.0,
                    tc2: 0.0,
                }
            };
            elements.push(Element {
                name,
                kind,
                nodes: vec![node_pos, node_neg],
            });
        }
        Some(other) => {
            let form = match other {
                ControlledSourceForm::Value => "VALUE",
                ControlledSourceForm::Table => "TABLE",
                ControlledSourceForm::Laplace => "LAPLACE",
                ControlledSourceForm::Freq => "FREQ",
                ControlledSourceForm::Poly(_) => unreachable!(),
            };
            return Err(unsupported_form_error(line_num, element_label, form));
        }
        None => {
            let control_element = expect_ident(stream, line_num)?;
            let gain = expect_value(stream, line_num, params)?;
            let kind = if is_voltage_output {
                ElementKind::Ccvs {
                    transresistance: gain,
                    control_element,
                }
            } else {
                ElementKind::Cccs {
                    gain,
                    control_element,
                }
            };
            elements.push(Element {
                name,
                kind,
                nodes: vec![node_pos, node_neg],
            });
        }
    }

    Ok(())
}

pub(super) fn parse_vcvs(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    parse_voltage_controlled_source(stream, line_num, elements, params, true)
}

pub(super) fn parse_cccs(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    parse_current_controlled_source(stream, line_num, elements, params, false)
}

pub(super) fn parse_vccs(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    parse_voltage_controlled_source(stream, line_num, elements, params, false)
}

pub(super) fn parse_ccvs(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    parse_current_controlled_source(stream, line_num, elements, params, true)
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
