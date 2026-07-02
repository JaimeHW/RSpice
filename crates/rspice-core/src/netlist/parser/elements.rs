//! Device and subcircuit element parsers.

use super::*;
use crate::netlist::XspicePort;

pub(super) fn parse_resistor(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
    diagnostics: &mut Vec<ParseDiagnostic>,
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
    let mut deferred_params: Vec<(String, String)> = Vec::new();

    skip_commas(stream);

    // First token after nodes can be:
    // 1) Explicit value (numeric/expression/param ref)
    // 2) Model name
    // 3) First named parameter (e.g. R=, VALUE=, MODEL=, L=, W=...)
    if !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        match &stream.peek().kind {
            TokenKind::Number(_) => {
                value = Some(expect_value(stream, line_num, params)?);
                consume_passive_unit_word(stream, PASSIVE_RESISTOR_UNITS);
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
                if params.get(&ident).is_some() {
                    stream.advance();
                    if defer_simple_param_refs {
                        value_expr = Some(ident);
                    } else {
                        value = params.get(&ident);
                    }
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

                    match take_deferrable_value(stream, params, defer_simple_param_refs) {
                        Some(DeferrableValue::Resolved(param_value)) => {
                            if name_upper == "R" || name_upper == "VALUE" {
                                value = Some(param_value);
                                value_expr = None;
                            }
                            instance_params.push((name_upper, param_value));
                        }
                        Some(DeferrableValue::Deferred(expr)) => {
                            if name_upper == "R" || name_upper == "VALUE" {
                                value_expr = Some(expr);
                                value = None;
                            } else if matches!(name_upper.as_str(), "RSER" | "RPAR" | "CPAR") {
                                // Parasitic expansion happens at parse time, so a
                                // per-instance value cannot be honored; rejecting
                                // beats silently dropping the parasitic.
                                return Err(ParseError::Syntax {
                                    line: line_num,
                                    message: format!(
                                        "parameterized {raw_name} on a passive inside a \
                                         subcircuit is not supported; declare the parasitic \
                                         as an explicit element instead"
                                    ),
                                });
                            } else {
                                deferred_params.push((name_upper, expr));
                            }
                        }
                        None => {
                            return Err(ParseError::Syntax {
                                line: line_num,
                                message: format!(
                                    "Expected value for resistor parameter '{}'",
                                    raw_name
                                ),
                            });
                        }
                    }
                } else if model.is_none() {
                    // Bare identifier after the optional value is the resistor
                    // model name. This covers both model-only instances and
                    // SPICE-compatible value-plus-model form.
                    model = Some(raw_name);
                } else if let Ok(param_value) = crate::netlist::lexer::parse_spice_value(&raw_name)
                {
                    value = Some(param_value);
                    value_expr = None;
                    continue;
                } else {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!(
                            "Unexpected trailing token in resistor specification: {raw_name}"
                        ),
                    });
                }
            }
            TokenKind::Number(_) => {
                // Allow trailing unnamed numeric value as explicit resistance override.
                value = Some(expect_value(stream, line_num, params)?);
                value_expr = None;
                consume_passive_unit_word(stream, PASSIVE_RESISTOR_UNITS);
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
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "Unexpected trailing token in resistor specification: {}",
                        stream.peek().kind
                    ),
                });
            }
        }
    }

    if value.is_none() {
        value = instance_params
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case("R") || k.eq_ignore_ascii_case("VALUE"))
            .map(|(_, v)| *v);
    }

    let missing_value = value.is_none() && value_expr.is_none();
    if missing_value {
        if model.is_none() {
            diagnostics.push(ParseDiagnostic::warning(
                line_num,
                "xyce_resistor_missing_value",
                format!(
                    "Resistor '{name}' has no value or model field; using Xyce's default 1000 ohm resistance"
                ),
            ));
        } else {
            diagnostics.push(ParseDiagnostic::warning(
                line_num,
                "xyce_resistor_model_missing_value",
                format!(
                    "Resistor '{name}' has no explicit value; model resolution may use Xyce's default 1000 ohm resistance"
                ),
            ));
        }
        value = Some(0.0);
        if !instance_params.iter().any(|(param, _)| {
            param.eq_ignore_ascii_case(crate::netlist::XYCE_DEFAULT_RESISTOR_VALUE_MARKER)
        }) {
            instance_params.push((
                crate::netlist::XYCE_DEFAULT_RESISTOR_VALUE_MARKER.to_string(),
                1.0,
            ));
        }
    }

    let mut nodes = vec![node_pos, node_neg];
    expand_passive_parasitics(elements, &name, &mut nodes, &mut instance_params);
    elements.push(Element {
        name,
        kind: ElementKind::Resistor {
            value: value.unwrap_or(Value::NAN),
            value_expr,
            model,
            instance_params,
            deferred_params,
        },
        nodes,
    });

    Ok(())
}

const PASSIVE_RESISTOR_UNITS: &[&str] = &["OHM", "OHMS"];
const PASSIVE_CAPACITOR_UNITS: &[&str] = &["F", "FARAD", "FARADS"];
const PASSIVE_INDUCTOR_UNITS: &[&str] = &["H", "HENRY", "HENRIES"];

fn consume_passive_unit_word(stream: &mut TokenStream, allowed_units: &[&str]) {
    let TokenKind::Ident(unit) = &stream.peek().kind else {
        return;
    };
    if allowed_units
        .iter()
        .any(|allowed| unit.eq_ignore_ascii_case(allowed))
    {
        stream.advance();
    }
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
    value_expr: Option<String>,
    model: Option<String>,
    ic: Option<Value>,
    instance_params: Vec<(String, Value)>,
    deferred_params: Vec<(String, String)>,
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
                deferred_params: Vec::new(),
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
                deferred_params: Vec::new(),
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
                value_expr: None,
                initial_voltage: None,
                model: None,
                instance_params: Vec::new(),
                deferred_params: Vec::new(),
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
    unit_words: &[&str],
    defer_simple_param_refs: bool,
) -> Result<PassiveTail, ParseError> {
    let mut tail = PassiveTail {
        value: None,
        value_expr: None,
        model: None,
        ic: None,
        instance_params: Vec::new(),
        deferred_params: Vec::new(),
    };

    skip_commas(stream);

    // Optional leading positional value or model name.
    if !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        match &stream.peek().kind {
            TokenKind::Number(_) => {
                tail.value = Some(expect_value(stream, line_num, params)?);
                consume_passive_unit_word(stream, unit_words);
            }
            TokenKind::Expression(_) | TokenKind::Plus | TokenKind::Minus => {
                if defer_simple_param_refs && matches!(stream.peek().kind, TokenKind::Expression(_))
                {
                    tail.value_expr = take_value_expression_string(stream, params);
                } else {
                    tail.value = Some(expect_value(stream, line_num, params)?);
                }
            }
            TokenKind::Ident(s) => {
                let ident = s.clone();
                if params.get(&ident).is_some() {
                    stream.advance();
                    if defer_simple_param_refs {
                        tail.value_expr = Some(ident);
                    } else {
                        tail.value = params.get(&ident);
                    }
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

                    if name_upper == "IC" {
                        // Initial conditions stay parse-time values: the IC
                        // field is plain numeric in the AST.
                        tail.ic =
                            Some(try_value(stream, params).ok_or_else(|| ParseError::Syntax {
                                line: line_num,
                                message: format!(
                                    "Expected value for {} parameter '{}'",
                                    element_label, raw_name
                                ),
                            })?);
                        continue;
                    }

                    match take_deferrable_value(stream, params, defer_simple_param_refs) {
                        Some(DeferrableValue::Resolved(param_value)) => {
                            if value_keys.iter().any(|key| name_upper == *key) {
                                tail.value = Some(param_value);
                                tail.value_expr = None;
                                continue;
                            }
                            tail.instance_params.push((name_upper, param_value));
                        }
                        Some(DeferrableValue::Deferred(expr)) => {
                            if value_keys.iter().any(|key| name_upper == *key) {
                                tail.value_expr = Some(expr);
                                tail.value = None;
                                continue;
                            }
                            if matches!(name_upper.as_str(), "RSER" | "RPAR" | "CPAR") {
                                return Err(ParseError::Syntax {
                                    line: line_num,
                                    message: format!(
                                        "parameterized {raw_name} on a passive inside a \
                                         subcircuit is not supported; declare the parasitic \
                                         as an explicit element instead"
                                    ),
                                });
                            }
                            tail.deferred_params.push((name_upper, expr));
                        }
                        None => {
                            return Err(ParseError::Syntax {
                                line: line_num,
                                message: format!(
                                    "Expected value for {} parameter '{}'",
                                    element_label, raw_name
                                ),
                            });
                        }
                    }
                } else if tail.model.is_none() && tail.value.is_none() && tail.value_expr.is_none()
                {
                    tail.model = Some(raw_name);
                } else {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!(
                            "Unexpected trailing token in {element_label} specification: {raw_name}"
                        ),
                    });
                }
            }
            TokenKind::Number(_) => {
                tail.value = Some(expect_value(stream, line_num, params)?);
                consume_passive_unit_word(stream, unit_words);
            }
            _ => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "Unexpected trailing token in {element_label} specification: {}",
                        stream.peek().kind
                    ),
                });
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
    defer_simple_param_refs: bool,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let node_pos = expect_node(stream, line_num)?;
    let node_neg = expect_node(stream, line_num)?;

    skip_optional_param_name(stream, "C");
    let tail = parse_passive_tail(
        stream,
        line_num,
        params,
        "capacitor",
        &["C", "VALUE", "CAP"],
        PASSIVE_CAPACITOR_UNITS,
        defer_simple_param_refs,
    )?;

    if tail.value.is_none() && tail.value_expr.is_none() && tail.model.is_none() {
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
            value_expr: tail.value_expr,
            initial_voltage: tail.ic,
            model: tail.model,
            instance_params,
            deferred_params: tail.deferred_params,
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
    defer_simple_param_refs: bool,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let node_pos = expect_node(stream, line_num)?;
    let node_neg = expect_node(stream, line_num)?;

    skip_optional_param_name(stream, "L");
    let tail = parse_passive_tail(
        stream,
        line_num,
        params,
        "inductor",
        &["L", "VALUE", "IND"],
        PASSIVE_INDUCTOR_UNITS,
        defer_simple_param_refs,
    )?;

    if tail.value.is_none() && tail.value_expr.is_none() && tail.model.is_none() {
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
            value_expr: tail.value_expr,
            initial_current: tail.ic,
            model: tail.model,
            instance_params,
            deferred_params: tail.deferred_params,
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
    defer_source_spec: bool,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let node_pos = expect_node(stream, line_num)?;
    let node_neg = expect_node(stream, line_num)?;

    if defer_source_spec {
        let raw_spec = collect_deferred_source_spec(stream);
        elements.push(Element {
            name,
            kind: if raw_spec.trim().is_empty() {
                ElementKind::VoltageSource(SourceSpec::Dc(0.0))
            } else {
                ElementKind::VoltageSourceDeferred(raw_spec)
            },
            nodes: vec![node_pos, node_neg],
        });
        return Ok(());
    }

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
    defer_source_spec: bool,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let node_pos = expect_node(stream, line_num)?;
    let node_neg = expect_node(stream, line_num)?;

    if defer_source_spec {
        let raw_spec = collect_deferred_source_spec(stream);
        elements.push(Element {
            name,
            kind: if raw_spec.trim().is_empty() {
                ElementKind::CurrentSource(SourceSpec::Dc(0.0))
            } else {
                ElementKind::CurrentSourceDeferred(raw_spec)
            },
            nodes: vec![node_pos, node_neg],
        });
        return Ok(());
    }

    let source_spec = parse_source_spec(stream, line_num, params)?;

    elements.push(Element {
        name,
        kind: ElementKind::CurrentSource(source_spec),
        nodes: vec![node_pos, node_neg],
    });

    Ok(())
}

fn collect_deferred_source_spec(stream: &mut TokenStream) -> String {
    stream
        .collect_line()
        .into_iter()
        .filter_map(|token| (!token.lexeme.is_empty()).then_some(token.lexeme))
        .collect::<Vec<_>>()
        .join(" ")
}

pub(super) fn parse_pspice_u_device(
    line: &str,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let mut fields = split_spice_fields(line);
    join_split_pspice_u_type_dimensions(&mut fields);
    if fields.len() < 5 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "PSpice U-device requires name, type, power pins, and logic pins".to_string(),
        });
    }

    let name = fields[0].to_ascii_uppercase();

    if let Some(gate) = parse_pspice_simple_u_gate(&fields[1]) {
        return parse_pspice_simple_u_gate_instance(
            &name, &fields, gate, line_num, elements, params,
        );
    }

    let (kind, count) = parse_pspice_u_kind_and_count(&fields[1]);
    match kind.as_str() {
        "BUF3" | "BUF3A" => parse_pspice_u_tristate(
            &name,
            &fields,
            count.unwrap_or(1),
            false,
            line_num,
            elements,
            params,
        ),
        "BUFA" => parse_pspice_u_unary_gate_array(
            &name,
            &fields,
            count.unwrap_or(1),
            "d_buffer",
            line_num,
            elements,
            params,
        ),
        "DFF" => parse_pspice_u_dff(
            &name,
            &fields,
            count.unwrap_or(1),
            line_num,
            elements,
            params,
        ),
        "DLYLINE" => parse_pspice_u_dlyline(&name, &fields, line_num, elements, params),
        "DLTCH" => parse_pspice_u_dlatch(
            &name,
            &fields,
            count.unwrap_or(1),
            line_num,
            elements,
            params,
        ),
        "INVA" => parse_pspice_u_unary_gate_array(
            &name,
            &fields,
            count.unwrap_or(1),
            "d_inverter",
            line_num,
            elements,
            params,
        ),
        "INV3" | "INV3A" => parse_pspice_u_tristate(
            &name,
            &fields,
            count.unwrap_or(1),
            true,
            line_num,
            elements,
            params,
        ),
        "JKFF" => parse_pspice_u_jkff(
            &name,
            &fields,
            count.unwrap_or(1),
            line_num,
            elements,
            params,
        ),
        "LOGICEXP" => parse_pspice_u_logicexp(
            &name,
            &fields,
            pspice_u_count_pair(&fields[1]),
            line,
            line_num,
            elements,
        ),
        "ANDA" => parse_pspice_u_vector_gate_array(
            &name,
            &fields,
            pspice_u_count_pair(&fields[1]),
            "d_and",
            line_num,
            elements,
            params,
        ),
        "AND3" => parse_pspice_u_tristate_vector_gate_array(
            &name,
            &fields,
            count.map(|input_count| (input_count, 1)),
            "d_and",
            line_num,
            elements,
            params,
        ),
        "AND3A" => parse_pspice_u_tristate_vector_gate_array(
            &name,
            &fields,
            pspice_u_count_pair(&fields[1]),
            "d_and",
            line_num,
            elements,
            params,
        ),
        "AO" => parse_pspice_u_compound_gate(
            &name,
            &fields,
            pspice_u_count_pair(&fields[1]),
            "d_and",
            "d_or",
            "$D_HI",
            line_num,
            elements,
            params,
        ),
        "AOI" => parse_pspice_u_compound_gate(
            &name,
            &fields,
            pspice_u_count_pair(&fields[1]),
            "d_and",
            "d_nor",
            "$D_HI",
            line_num,
            elements,
            params,
        ),
        "NANDA" => parse_pspice_u_vector_gate_array(
            &name,
            &fields,
            pspice_u_count_pair(&fields[1]),
            "d_nand",
            line_num,
            elements,
            params,
        ),
        "NAND3" => parse_pspice_u_tristate_vector_gate_array(
            &name,
            &fields,
            count.map(|input_count| (input_count, 1)),
            "d_nand",
            line_num,
            elements,
            params,
        ),
        "NAND3A" => parse_pspice_u_tristate_vector_gate_array(
            &name,
            &fields,
            pspice_u_count_pair(&fields[1]),
            "d_nand",
            line_num,
            elements,
            params,
        ),
        "NORA" => parse_pspice_u_vector_gate_array(
            &name,
            &fields,
            pspice_u_count_pair(&fields[1]),
            "d_nor",
            line_num,
            elements,
            params,
        ),
        "NOR3" => parse_pspice_u_tristate_vector_gate_array(
            &name,
            &fields,
            count.map(|input_count| (input_count, 1)),
            "d_nor",
            line_num,
            elements,
            params,
        ),
        "NOR3A" => parse_pspice_u_tristate_vector_gate_array(
            &name,
            &fields,
            pspice_u_count_pair(&fields[1]),
            "d_nor",
            line_num,
            elements,
            params,
        ),
        "NXORA" => parse_pspice_u_vector_gate_array(
            &name,
            &fields,
            count.map(|gate_count| (2, gate_count)),
            "d_xnor",
            line_num,
            elements,
            params,
        ),
        "NXOR3" => parse_pspice_u_tristate_vector_gate_array(
            &name,
            &fields,
            Some((2, 1)),
            "d_xnor",
            line_num,
            elements,
            params,
        ),
        "NXOR3A" => parse_pspice_u_tristate_vector_gate_array(
            &name,
            &fields,
            count.map(|gate_count| (2, gate_count)),
            "d_xnor",
            line_num,
            elements,
            params,
        ),
        "ORA" => parse_pspice_u_vector_gate_array(
            &name,
            &fields,
            pspice_u_count_pair(&fields[1]),
            "d_or",
            line_num,
            elements,
            params,
        ),
        "OA" => parse_pspice_u_compound_gate(
            &name,
            &fields,
            pspice_u_count_pair(&fields[1]),
            "d_or",
            "d_and",
            "$D_LO",
            line_num,
            elements,
            params,
        ),
        "OAI" => parse_pspice_u_compound_gate(
            &name,
            &fields,
            pspice_u_count_pair(&fields[1]),
            "d_or",
            "d_nand",
            "$D_LO",
            line_num,
            elements,
            params,
        ),
        "PINDLY" => parse_pspice_u_pindly(
            &name,
            &fields,
            pspice_u_count_triple(&fields[1]),
            line,
            line_num,
            elements,
            params,
        ),
        "OR3" => parse_pspice_u_tristate_vector_gate_array(
            &name,
            &fields,
            count.map(|input_count| (input_count, 1)),
            "d_or",
            line_num,
            elements,
            params,
        ),
        "OR3A" => parse_pspice_u_tristate_vector_gate_array(
            &name,
            &fields,
            pspice_u_count_pair(&fields[1]),
            "d_or",
            line_num,
            elements,
            params,
        ),
        "PULLDN" | "PULLDOWN" => parse_pspice_u_pull(
            &name,
            &fields,
            count.unwrap_or(1),
            "d_pulldown",
            line_num,
            elements,
        ),
        "PULLUP" => parse_pspice_u_pull(
            &name,
            &fields,
            count.unwrap_or(1),
            "d_pullup",
            line_num,
            elements,
        ),
        "SRFF" => parse_pspice_u_srlatch(
            &name,
            &fields,
            count.unwrap_or(1),
            line_num,
            elements,
            params,
        ),
        "XORA" => parse_pspice_u_vector_gate_array(
            &name,
            &fields,
            count.map(|gate_count| (2, gate_count)),
            "d_xor",
            line_num,
            elements,
            params,
        ),
        "XOR3" => parse_pspice_u_tristate_vector_gate_array(
            &name,
            &fields,
            Some((2, 1)),
            "d_xor",
            line_num,
            elements,
            params,
        ),
        "XOR3A" => parse_pspice_u_tristate_vector_gate_array(
            &name,
            &fields,
            count.map(|gate_count| (2, gate_count)),
            "d_xor",
            line_num,
            elements,
            params,
        ),
        _ => Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Unsupported PSpice U-device type '{}'; supported frontend lowerings are simple gates, DFF, DLTCH, DLYLINE, JKFF, LOGICEXP, PINDLY, PULLUP, PULLDN, SRFF, BUFA, INVA, ANDA, NANDA, ORA, NORA, XORA, NXORA, BUF3, INV3, AND3, NAND3, OR3, NOR3, XOR3, NXOR3, BUF3A, INV3A, AND3A, NAND3A, OR3A, NOR3A, XOR3A, NXOR3A, AO, AOI, OA, and OAI",
                fields[1]
            ),
        }),
    }
}

fn join_split_pspice_u_type_dimensions(fields: &mut Vec<String>) {
    if fields.len() <= 2 || !fields[1].contains('(') || fields[1].contains(')') {
        return;
    }

    let mut joined = fields[1].clone();
    let mut consumed = 0usize;
    for field in fields.iter().skip(2) {
        joined.push(',');
        joined.push_str(field);
        consumed += 1;
        if field.contains(')') {
            break;
        }
    }

    if !joined.contains(')') {
        return;
    }

    fields[1] = joined;
    for _ in 0..consumed {
        fields.remove(2);
    }
}

fn parse_pspice_simple_u_gate_instance(
    name: &str,
    fields: &[String],
    gate: PspiceSimpleUGate,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let pins = &fields[4..]; // fields[2] and fields[3] are DPWR/DGND pins.
    if pins.len() < gate.input_count + 1 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice U-device '{}' type '{}' requires {} input pin(s) and one output pin",
                name, fields[1], gate.input_count
            ),
        });
    }

    let output = normalize_pspice_u_node(&pins[gate.input_count]);
    let ports = if gate.input_count == 1 {
        vec![
            XspicePort::Digital(normalize_pspice_u_node(&pins[0])),
            XspicePort::Digital(output),
        ]
    } else {
        vec![
            XspicePort::DigitalVector(
                pins[..gate.input_count]
                    .iter()
                    .map(|pin| normalize_pspice_u_node(pin))
                    .collect(),
            ),
            XspicePort::Digital(output),
        ]
    };

    let pspice_u_timing = pins
        .get(gate.input_count + 1)
        .and_then(|token| pspice_u_timing_from_token(token, fields, params, line_num));
    push_pspice_u_xspice_element_with_timing(
        elements,
        name.to_string(),
        gate.xspice_model,
        ports,
        pspice_u_timing,
    );

    Ok(())
}

fn parse_pspice_u_dff(
    name: &str,
    fields: &[String],
    count: usize,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    if count == 0 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice U-device '{}' type '{}' requires at least one DFF",
                name, fields[1]
            ),
        });
    }

    let pins = &fields[4..];
    let required = 3 + count * 3 + 1;
    if pins.len() < required {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice DFF U-device '{}' requires PREBAR, CLRBAR, CLK, {} D input(s), {} Q output(s), {} QBAR output(s), and a timing model",
                name, count, count, count
            ),
        });
    }

    let prebar = pspice_u_active_low_control_port(&pins[0], elements);
    let clrbar = pspice_u_active_low_control_port(&pins[1], elements);
    let clk = pspice_u_required_digital_port(&pins[2], "clock", fields, line_num, elements)?;
    let pspice_u_timing = pspice_u_timing_from_token(&pins[required - 1], fields, params, line_num);
    let d_offset = 3;
    let q_offset = d_offset + count;
    let qb_offset = q_offset + count;

    for index in 0..count {
        let data = pspice_u_required_digital_port(
            &pins[d_offset + index],
            "D input",
            fields,
            line_num,
            elements,
        )?;
        let q = pspice_u_nullable_output_port(&pins[q_offset + index]);
        let qb = pspice_u_nullable_output_port(&pins[qb_offset + index]);
        let instance_name = pspice_u_lowered_instance_name(name, count, index);
        let ports = vec![data, clk.clone(), prebar.clone(), clrbar.clone(), q, qb];
        push_pspice_u_xspice_element_with_timing(
            elements,
            instance_name,
            "d_dff",
            ports,
            pspice_u_timing.clone(),
        );
    }

    Ok(())
}

fn parse_pspice_u_vector_gate_array(
    name: &str,
    fields: &[String],
    shape: Option<(usize, usize)>,
    model: &str,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let Some((input_count, gate_count)) = shape else {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice vector gate array '{}' requires valid type dimensions",
                name
            ),
        });
    };
    if input_count < 2 || gate_count == 0 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice vector gate array '{}' requires at least two inputs and one gate",
                name
            ),
        });
    }

    let input_total = input_count
        .checked_mul(gate_count)
        .ok_or_else(|| ParseError::Syntax {
            line: line_num,
            message: format!("PSpice vector gate array '{}' is too large", name),
        })?;
    let required = input_total
        .checked_add(gate_count)
        .and_then(|count| count.checked_add(1))
        .ok_or_else(|| ParseError::Syntax {
            line: line_num,
            message: format!("PSpice vector gate array '{}' is too large", name),
        })?;

    let pins = &fields[4..];
    if pins.len() < required {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice vector gate array '{}' requires {} input pin(s), {} output pin(s), and a timing model",
                name, input_total, gate_count
            ),
        });
    }

    let output_offset = input_total;
    let pspice_u_timing = pspice_u_timing_from_token(&pins[required - 1], fields, params, line_num);
    for gate_index in 0..gate_count {
        let input_start = gate_index * input_count;
        let inputs = pins[input_start..input_start + input_count]
            .iter()
            .map(|pin| {
                pspice_u_required_digital_node(pin, "gate input", fields, line_num, elements)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let output = pspice_u_required_digital_port(
            &pins[output_offset + gate_index],
            "gate output",
            fields,
            line_num,
            elements,
        )?;
        let instance_name = pspice_u_lowered_instance_name(name, gate_count, gate_index);
        push_pspice_u_xspice_element_with_timing(
            elements,
            instance_name,
            model,
            vec![XspicePort::DigitalVector(inputs), output],
            pspice_u_timing.clone(),
        );
    }

    Ok(())
}

fn parse_pspice_u_unary_gate_array(
    name: &str,
    fields: &[String],
    count: usize,
    model: &str,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    if count == 0 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice U-device '{}' type '{}' requires at least one unary gate",
                name, fields[1]
            ),
        });
    }

    let pins = &fields[4..];
    let required = count * 2 + 1;
    if pins.len() < required {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice unary gate array '{}' requires {} input pin(s), {} output pin(s), and a timing model",
                name, count, count
            ),
        });
    }

    let output_offset = count;
    let pspice_u_timing = pspice_u_timing_from_token(&pins[required - 1], fields, params, line_num);
    for index in 0..count {
        let input =
            pspice_u_required_digital_port(&pins[index], "gate input", fields, line_num, elements)?;
        let output = pspice_u_required_digital_port(
            &pins[output_offset + index],
            "gate output",
            fields,
            line_num,
            elements,
        )?;
        let instance_name = pspice_u_lowered_instance_name(name, count, index);
        push_pspice_u_xspice_element_with_timing(
            elements,
            instance_name,
            model,
            vec![input, output],
            pspice_u_timing.clone(),
        );
    }

    Ok(())
}

fn parse_pspice_u_logicexp(
    name: &str,
    fields: &[String],
    shape: Option<(usize, usize)>,
    line: &str,
    line_num: usize,
    elements: &mut Vec<Element>,
) -> Result<(), ParseError> {
    let Some((input_count, output_count)) = shape else {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice LOGICEXP U-device '{}' requires valid dimensions",
                name
            ),
        });
    };
    if output_count == 0 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice LOGICEXP U-device '{}' requires at least one output",
                name
            ),
        });
    }

    let Some(logic_index) = fields
        .iter()
        .position(|field| field.eq_ignore_ascii_case("LOGIC:"))
    else {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice LOGICEXP U-device '{}' requires a LOGIC: section",
                name
            ),
        });
    };

    let pins = &fields[4..logic_index];
    let required = input_count
        .checked_add(output_count)
        .and_then(|count| count.checked_add(1))
        .ok_or_else(|| ParseError::Syntax {
            line: line_num,
            message: format!("PSpice LOGICEXP U-device '{}' is too large", name),
        })?;
    if pins.len() < required {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice LOGICEXP U-device '{}' requires {} input pin(s), {} output pin(s), and a timing model",
                name, input_count, output_count
            ),
        });
    }

    for pin in &pins[..input_count] {
        pspice_u_required_digital_node(pin, "LOGICEXP input", fields, line_num, elements)?;
    }
    for pin in &pins[input_count..input_count + output_count] {
        pspice_u_required_digital_port(pin, "LOGICEXP output", fields, line_num, elements)?;
    }

    let logic_section = pspice_u_logicexp_section(line).ok_or_else(|| ParseError::Syntax {
        line: line_num,
        message: format!(
            "PSpice LOGICEXP U-device '{}' requires a LOGIC: section",
            name
        ),
    })?;
    let statements = PspiceLogicexpParser::new(logic_section, line_num).parse_statements()?;
    if statements.is_empty() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice LOGICEXP U-device '{}' requires at least one assignment",
                name
            ),
        });
    }

    let mut lowerer = PspiceLogicexpLowerer::new(name, elements);
    for statement in statements {
        let target = normalize_pspice_u_node(&statement.output);
        lowerer.lower_to_target(&statement.expr, target);
    }

    Ok(())
}

fn pspice_u_logicexp_section(line: &str) -> Option<&str> {
    let upper = line.to_ascii_uppercase();
    let index = upper.find("LOGIC:")?;
    Some(&line[index + "LOGIC:".len()..])
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PspiceLogicexpStatement {
    output: String,
    expr: PspiceLogicexpExpr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PspiceLogicexpExpr {
    Ident(String),
    Not(Box<PspiceLogicexpExpr>),
    Binary {
        op: PspiceLogicexpOp,
        left: Box<PspiceLogicexpExpr>,
        right: Box<PspiceLogicexpExpr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PspiceLogicexpOp {
    And,
    Xor,
    Or,
}

struct PspiceLogicexpParser<'a> {
    input: &'a str,
    pos: usize,
    line_num: usize,
}

impl<'a> PspiceLogicexpParser<'a> {
    fn new(input: &'a str, line_num: usize) -> Self {
        Self {
            input,
            pos: 0,
            line_num,
        }
    }

    fn parse_statements(mut self) -> Result<Vec<PspiceLogicexpStatement>, ParseError> {
        let mut statements = Vec::new();
        loop {
            self.skip_ws();
            if self.is_eof() {
                break;
            }
            let output = self.parse_ident("LOGICEXP assignment output")?;
            self.skip_ws();
            self.expect_char('=')?;
            self.skip_ws();
            self.expect_char('{')?;
            let expr = self.parse_expr()?;
            self.skip_ws();
            self.expect_char('}')?;
            statements.push(PspiceLogicexpStatement { output, expr });
        }
        Ok(statements)
    }

    fn parse_expr(&mut self) -> Result<PspiceLogicexpExpr, ParseError> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<PspiceLogicexpExpr, ParseError> {
        let mut expr = self.parse_xor()?;
        loop {
            self.skip_ws();
            if !self.consume_char('|') {
                break;
            }
            let right = self.parse_xor()?;
            expr = PspiceLogicexpExpr::Binary {
                op: PspiceLogicexpOp::Or,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_xor(&mut self) -> Result<PspiceLogicexpExpr, ParseError> {
        let mut expr = self.parse_and()?;
        loop {
            self.skip_ws();
            if !self.consume_char('^') {
                break;
            }
            let right = self.parse_and()?;
            expr = PspiceLogicexpExpr::Binary {
                op: PspiceLogicexpOp::Xor,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<PspiceLogicexpExpr, ParseError> {
        let mut expr = self.parse_unary()?;
        loop {
            self.skip_ws();
            if !self.consume_char('&') {
                break;
            }
            let right = self.parse_unary()?;
            expr = PspiceLogicexpExpr::Binary {
                op: PspiceLogicexpOp::And,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<PspiceLogicexpExpr, ParseError> {
        self.skip_ws();
        if self.consume_char('~') {
            let expr = self.parse_unary()?;
            return Ok(PspiceLogicexpExpr::Not(Box::new(expr)));
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<PspiceLogicexpExpr, ParseError> {
        self.skip_ws();
        if self.consume_char('(') {
            let expr = self.parse_expr()?;
            self.skip_ws();
            self.expect_char(')')?;
            return Ok(expr);
        }
        Ok(PspiceLogicexpExpr::Ident(
            self.parse_ident("LOGICEXP expression input")?,
        ))
    }

    fn parse_ident(&mut self, role: &str) -> Result<String, ParseError> {
        self.skip_ws();
        let start = self.pos;
        let Some(ch) = self.peek_char() else {
            return self.error(format!("expected {role}"));
        };
        if !pspice_logicexp_ident_start(ch) {
            return self.error(format!("expected {role}"));
        }
        self.pos += ch.len_utf8();
        while let Some(ch) = self.peek_char() {
            if !pspice_logicexp_ident_continue(ch) {
                break;
            }
            self.pos += ch.len_utf8();
        }
        Ok(self.input[start..self.pos].to_string())
    }

    fn expect_char(&mut self, expected: char) -> Result<(), ParseError> {
        self.skip_ws();
        if self.consume_char(expected) {
            Ok(())
        } else {
            self.error(format!("expected '{expected}' in LOGICEXP section"))
        }
    }

    fn consume_char(&mut self, expected: char) -> bool {
        if self.peek_char() == Some(expected) {
            self.pos += expected.len_utf8();
            true
        } else {
            false
        }
    }

    fn skip_ws(&mut self) {
        while let Some(ch) = self.peek_char() {
            if !ch.is_whitespace() {
                break;
            }
            self.pos += ch.len_utf8();
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn error<T>(&self, message: String) -> Result<T, ParseError> {
        Err(ParseError::Syntax {
            line: self.line_num,
            message,
        })
    }
}

fn pspice_logicexp_ident_start(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '_' | '/' | '-' | '$')
}

fn pspice_logicexp_ident_continue(ch: char) -> bool {
    pspice_logicexp_ident_start(ch) || ch == '+'
}

struct PspiceLogicexpLowerer<'a> {
    owner: &'a str,
    elements: &'a mut Vec<Element>,
    next_gate: usize,
}

impl<'a> PspiceLogicexpLowerer<'a> {
    fn new(owner: &'a str, elements: &'a mut Vec<Element>) -> Self {
        Self {
            owner,
            elements,
            next_gate: 0,
        }
    }

    fn lower_to_target(&mut self, expr: &PspiceLogicexpExpr, target: String) -> String {
        self.lower_expr(expr, Some(target))
    }

    fn lower_expr(&mut self, expr: &PspiceLogicexpExpr, target: Option<String>) -> String {
        match expr {
            PspiceLogicexpExpr::Ident(raw) => {
                let node = normalize_pspice_u_node(raw);
                ensure_pspice_u_constant_driver(raw, self.elements);
                if let Some(target) = target {
                    if node.eq_ignore_ascii_case(&target) {
                        node
                    } else {
                        self.push_scalar_gate("d_buffer", node, target)
                    }
                } else {
                    node
                }
            }
            PspiceLogicexpExpr::Not(inner) => {
                if let PspiceLogicexpExpr::Not(double_inner) = inner.as_ref() {
                    return self.lower_expr(double_inner, target);
                }
                if let PspiceLogicexpExpr::Binary { op, .. } = inner.as_ref() {
                    return self.lower_binary(inner, *op, true, target);
                }
                let input = self.lower_expr(inner, None);
                let output = target.unwrap_or_else(|| self.next_connector("LOGIC"));
                self.push_scalar_gate("d_inverter", input, output)
            }
            PspiceLogicexpExpr::Binary { op, .. } => self.lower_binary(expr, *op, false, target),
        }
    }

    fn lower_binary(
        &mut self,
        expr: &PspiceLogicexpExpr,
        op: PspiceLogicexpOp,
        inverted: bool,
        target: Option<String>,
    ) -> String {
        let mut operands = Vec::new();
        collect_pspice_logicexp_operands(expr, op, &mut operands);
        let inputs = operands
            .into_iter()
            .map(|operand| self.lower_expr(operand, None))
            .collect::<Vec<_>>();
        let output = target.unwrap_or_else(|| self.next_connector("LOGIC"));
        let model = pspice_logicexp_gate_model(op, inverted);
        self.push_vector_gate(model, inputs, output)
    }

    fn push_scalar_gate(&mut self, model: &str, input: String, output: String) -> String {
        let name = self.next_gate_name();
        push_pspice_u_xspice_element_with_params(
            self.elements,
            name,
            model,
            vec![
                XspicePort::Digital(input),
                XspicePort::Digital(output.clone()),
            ],
            pspice_u_zero_gate_delay_params(),
            None,
        );
        output
    }

    fn push_vector_gate(&mut self, model: &str, inputs: Vec<String>, output: String) -> String {
        let name = self.next_gate_name();
        push_pspice_u_xspice_element_with_params(
            self.elements,
            name,
            model,
            vec![
                XspicePort::DigitalVector(inputs),
                XspicePort::Digital(output.clone()),
            ],
            pspice_u_zero_gate_delay_params(),
            None,
        );
        output
    }

    fn next_gate_name(&mut self) -> String {
        let name = format!("{}__LOGIC_{}", self.owner, self.next_gate);
        self.next_gate += 1;
        name
    }

    fn next_connector(&mut self, suffix: &str) -> String {
        pspice_u_internal_connector_name_with_suffix(
            &format!("{}_{}", self.owner, self.next_gate),
            suffix,
        )
    }
}

fn collect_pspice_logicexp_operands<'a>(
    expr: &'a PspiceLogicexpExpr,
    op: PspiceLogicexpOp,
    operands: &mut Vec<&'a PspiceLogicexpExpr>,
) {
    if let PspiceLogicexpExpr::Binary {
        op: expr_op,
        left,
        right,
    } = expr
        && *expr_op == op
    {
        collect_pspice_logicexp_operands(left, op, operands);
        collect_pspice_logicexp_operands(right, op, operands);
        return;
    }
    operands.push(expr);
}

fn pspice_logicexp_gate_model(op: PspiceLogicexpOp, inverted: bool) -> &'static str {
    match (op, inverted) {
        (PspiceLogicexpOp::And, false) => "d_and",
        (PspiceLogicexpOp::And, true) => "d_nand",
        (PspiceLogicexpOp::Or, false) => "d_or",
        (PspiceLogicexpOp::Or, true) => "d_nor",
        (PspiceLogicexpOp::Xor, false) => "d_xor",
        (PspiceLogicexpOp::Xor, true) => "d_xnor",
    }
}

fn parse_pspice_u_tristate_vector_gate_array(
    name: &str,
    fields: &[String],
    shape: Option<(usize, usize)>,
    primary_model: &str,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let Some((input_count, gate_count)) = shape else {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice tri-state vector U-device '{}' requires valid type dimensions",
                name
            ),
        });
    };
    if input_count < 2 || gate_count == 0 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice tri-state vector U-device '{}' requires at least two inputs and one gate",
                name
            ),
        });
    }

    let input_total = input_count
        .checked_mul(gate_count)
        .ok_or_else(|| ParseError::Syntax {
            line: line_num,
            message: format!("PSpice tri-state vector U-device '{}' is too large", name),
        })?;
    let required = input_total
        .checked_add(1)
        .and_then(|count| count.checked_add(gate_count))
        .and_then(|count| count.checked_add(1))
        .ok_or_else(|| ParseError::Syntax {
            line: line_num,
            message: format!("PSpice tri-state vector U-device '{}' is too large", name),
        })?;

    let pins = &fields[4..];
    if pins.len() < required {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice tri-state vector U-device '{}' requires {} input pin(s), one enable pin, {} output pin(s), and a timing model",
                name, input_total, gate_count
            ),
        });
    }

    let enable =
        pspice_u_required_digital_port(&pins[input_total], "enable", fields, line_num, elements)?;
    let output_offset = input_total + 1;
    let pspice_u_timing = pspice_u_timing_from_token(&pins[required - 1], fields, params, line_num);

    for gate_index in 0..gate_count {
        let input_start = gate_index * input_count;
        let inputs = pins[input_start..input_start + input_count]
            .iter()
            .map(|pin| {
                pspice_u_required_digital_node(pin, "gate input", fields, line_num, elements)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let output = pspice_u_required_digital_port(
            &pins[output_offset + gate_index],
            "tri-state output",
            fields,
            line_num,
            elements,
        )?;
        let instance_name = pspice_u_lowered_instance_name(name, gate_count, gate_index);
        push_pspice_u_tristate_vector_gate(
            elements,
            &instance_name,
            primary_model,
            inputs,
            enable.clone(),
            output,
            pspice_u_timing.clone(),
        );
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct PspicePindlyEntry {
    input: String,
    output: String,
    enable: Option<XspicePort>,
    delay: Option<Value>,
}

fn parse_pspice_u_pindly(
    name: &str,
    fields: &[String],
    shape: Option<(usize, usize, usize)>,
    line: &str,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let Some((io_count, enable_count, reference_count)) = shape else {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice PINDLY U-device '{}' requires valid dimensions",
                name
            ),
        });
    };
    if io_count == 0 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice PINDLY U-device '{}' requires at least one delayed output",
                name
            ),
        });
    }

    let section_index = pspice_u_first_behavior_section_field(fields).unwrap_or(fields.len());
    let pins = &fields[4..section_index];
    let required = io_count
        .checked_add(enable_count)
        .and_then(|count| count.checked_add(reference_count))
        .and_then(|count| count.checked_add(io_count))
        .and_then(|count| count.checked_add(1))
        .ok_or_else(|| ParseError::Syntax {
            line: line_num,
            message: format!("PSpice PINDLY U-device '{}' is too large", name),
        })?;
    if pins.len() < required {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice PINDLY U-device '{}' requires {} input pin(s), {} enable pin(s), {} reference pin(s), {} output pin(s), and an I/O model",
                name, io_count, enable_count, reference_count, io_count
            ),
        });
    }

    let input_offset = 0;
    let enable_offset = input_offset + io_count;
    let reference_offset = enable_offset + enable_count;
    let output_offset = reference_offset + reference_count;
    let mut entries = Vec::with_capacity(io_count);
    for index in 0..io_count {
        let input = pspice_u_required_digital_node(
            &pins[input_offset + index],
            "PINDLY input",
            fields,
            line_num,
            elements,
        )?;
        let output = pspice_u_required_digital_node(
            &pins[output_offset + index],
            "PINDLY output",
            fields,
            line_num,
            elements,
        )?;
        entries.push(PspicePindlyEntry {
            input,
            output,
            enable: None,
            delay: None,
        });
    }

    for pin in &pins[enable_offset..reference_offset] {
        pspice_u_required_digital_node(pin, "PINDLY enable", fields, line_num, elements)?;
    }
    for pin in &pins[reference_offset..output_offset] {
        pspice_u_required_digital_node(pin, "PINDLY reference", fields, line_num, elements)?;
    }

    let delay_mode = pspice_u_delay_mode(fields, section_index, params, line_num);
    if let Some(section) = pspice_u_behavior_section(line, "PINDLY:") {
        apply_pspice_u_pindly_delay_section(section, &mut entries, delay_mode, false);
    }
    if let Some(section) = pspice_u_behavior_section(line, "TRISTATE:") {
        apply_pspice_u_pindly_tristate_section(section, &mut entries, delay_mode);
    }

    for (index, entry) in entries.into_iter().enumerate() {
        let instance_name = pspice_u_lowered_instance_name(name, io_count, index);
        let delay = entry.delay.unwrap_or(10.0e-9);
        if let Some(enable) = entry.enable {
            push_pspice_u_xspice_element_with_params(
                elements,
                instance_name,
                "d_tristate",
                vec![
                    XspicePort::Digital(entry.input),
                    enable,
                    XspicePort::Digital(entry.output),
                ],
                pspice_u_pindly_tristate_delay_params(delay),
                None,
            );
        } else {
            push_pspice_u_xspice_element_with_params(
                elements,
                instance_name,
                "d_buffer",
                vec![
                    XspicePort::Digital(entry.input),
                    XspicePort::Digital(entry.output),
                ],
                pspice_u_pindly_buffer_delay_params(delay),
                None,
            );
        }
    }

    Ok(())
}

fn pspice_u_first_behavior_section_field(fields: &[String]) -> Option<usize> {
    fields.iter().position(|field| {
        matches!(
            field.to_ascii_uppercase().as_str(),
            "BOOLEAN:" | "FREQ:" | "GENERAL:" | "PINDLY:" | "SETUP_HOLD:" | "TRISTATE:" | "WIDTH:"
        )
    })
}

fn pspice_u_behavior_section<'a>(line: &'a str, section: &str) -> Option<&'a str> {
    const SECTION_MARKERS: [&str; 7] = [
        "BOOLEAN:",
        "FREQ:",
        "GENERAL:",
        "PINDLY:",
        "SETUP_HOLD:",
        "TRISTATE:",
        "WIDTH:",
    ];

    let upper = line.to_ascii_uppercase();
    let start = upper.find(section)?;
    let body_start = start + section.len();
    let rest = &line[body_start..];
    let upper_rest = &upper[body_start..];
    let end = SECTION_MARKERS
        .iter()
        .filter(|marker| !marker.eq_ignore_ascii_case(section))
        .filter_map(|marker| upper_rest.find(marker))
        .min()
        .unwrap_or(rest.len());
    Some(&rest[..end])
}

fn apply_pspice_u_pindly_delay_section(
    section: &str,
    entries: &mut [PspicePindlyEntry],
    delay_mode: PspiceUTimingMode,
    tristate: bool,
) {
    let mut parser = PspicePindlySectionParser::new(section);
    while let Some((outputs, block)) = parser.next_assignment() {
        let delay = pspice_u_pindly_delay_from_block(block, delay_mode).unwrap_or(10.0e-9);
        for output in outputs {
            if let Some(entry) = entries
                .iter_mut()
                .find(|entry| entry.output.eq_ignore_ascii_case(&output))
            {
                entry.delay = Some(delay);
                if !tristate {
                    entry.enable = None;
                }
            }
        }
    }
}

fn apply_pspice_u_pindly_tristate_section(
    section: &str,
    entries: &mut [PspicePindlyEntry],
    delay_mode: PspiceUTimingMode,
) {
    let (body, enable) = pspice_u_pindly_tristate_enable(section);
    let Some(enable) = enable else {
        return;
    };
    let mut parser = PspicePindlySectionParser::new(body);
    while let Some((outputs, block)) = parser.next_assignment() {
        let delay = pspice_u_pindly_delay_from_block(block, delay_mode).unwrap_or(10.0e-9);
        for output in outputs {
            if let Some(entry) = entries
                .iter_mut()
                .find(|entry| entry.output.eq_ignore_ascii_case(&output))
            {
                entry.delay = Some(delay);
                entry.enable = Some(enable.clone());
            }
        }
    }
}

fn pspice_u_delay_mode(
    fields: &[String],
    section_index: usize,
    params: &ParamContext,
    line_num: usize,
) -> PspiceUTimingMode {
    let Some(raw_value) = pspice_u_assignment_value(fields, section_index, "MNTYMXDLY") else {
        return PspiceUTimingMode::Typ;
    };
    let Ok(value) = parse_numeric_field_value(raw_value, params, line_num) else {
        return PspiceUTimingMode::Typ;
    };

    match value.round() as i64 {
        1 => PspiceUTimingMode::Min,
        2 => PspiceUTimingMode::Max,
        _ => PspiceUTimingMode::Typ,
    }
}

fn pspice_u_assignment_value<'a>(
    fields: &'a [String],
    section_index: usize,
    name: &str,
) -> Option<&'a str> {
    let limit = section_index.min(fields.len());
    let mut index = 0usize;
    while index < limit {
        let field = fields[index].trim();
        if let Some((lhs, rhs)) = field.split_once('=') {
            if lhs.trim().eq_ignore_ascii_case(name) && !rhs.trim().is_empty() {
                return Some(rhs.trim());
            }
        } else if field.eq_ignore_ascii_case(name) {
            let mut value_index = index + 1;
            if value_index < limit && fields[value_index].trim() == "=" {
                value_index += 1;
            }
            if value_index < limit {
                return Some(fields[value_index].trim());
            }
        }
        index += 1;
    }
    None
}

fn pspice_u_pindly_tristate_enable(section: &str) -> (&str, Option<XspicePort>) {
    let upper = section.to_ascii_uppercase();
    let Some(enable_start) = upper.find("ENABLE") else {
        return (section, None);
    };
    let mut parser = PspicePindlySectionParser::new(&section[enable_start..]);
    let Some(keyword) = parser.next_ident() else {
        return (section, None);
    };
    if !keyword.eq_ignore_ascii_case("ENABLE") {
        return (section, None);
    }
    let Some(polarity) = parser.next_ident() else {
        return (section, None);
    };
    if !polarity.eq_ignore_ascii_case("HI") && !polarity.eq_ignore_ascii_case("LO") {
        return (section, None);
    }
    parser.skip_ws();
    parser.consume_char('=');
    let Some(node) = parser.next_ident() else {
        return (section, None);
    };
    let port = if polarity.eq_ignore_ascii_case("LO") {
        XspicePort::DigitalInverted(normalize_pspice_u_node(&node))
    } else {
        XspicePort::Digital(normalize_pspice_u_node(&node))
    };
    (&section[enable_start + parser.pos..], Some(port))
}

struct PspicePindlySectionParser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> PspicePindlySectionParser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn next_assignment(&mut self) -> Option<(Vec<String>, &'a str)> {
        loop {
            self.skip_ws();
            if self.is_eof() {
                return None;
            }

            let mut outputs = Vec::new();
            while let Some(output) = self.next_ident() {
                if output.eq_ignore_ascii_case("ENABLE") {
                    outputs.clear();
                    break;
                }
                outputs.push(normalize_pspice_u_node(&output));
                self.skip_ws();
                self.consume_char(',');
                self.skip_ws();
                if self.peek_char() == Some('=') {
                    break;
                }
            }
            self.skip_ws();
            if outputs.is_empty() || !self.consume_char('=') {
                self.advance_one_char();
                continue;
            }
            self.skip_ws();
            if !self.consume_char('{') {
                continue;
            }
            let block_start = self.pos;
            let mut depth = 1usize;
            while let Some(ch) = self.peek_char() {
                self.pos += ch.len_utf8();
                match ch {
                    '{' => depth += 1,
                    '}' => {
                        depth = depth.saturating_sub(1);
                        if depth == 0 {
                            let end = self.pos - ch.len_utf8();
                            return Some((outputs, &self.input[block_start..end]));
                        }
                    }
                    _ => {}
                }
            }
            return None;
        }
    }

    fn next_ident(&mut self) -> Option<String> {
        self.skip_ws();
        let start = self.pos;
        let ch = self.peek_char()?;
        if !pspice_logicexp_ident_start(ch) {
            return None;
        }
        self.pos += ch.len_utf8();
        while let Some(ch) = self.peek_char() {
            if !pspice_logicexp_ident_continue(ch) {
                break;
            }
            self.pos += ch.len_utf8();
        }
        Some(self.input[start..self.pos].to_string())
    }

    fn consume_char(&mut self, expected: char) -> bool {
        if self.peek_char() == Some(expected) {
            self.pos += expected.len_utf8();
            true
        } else {
            false
        }
    }

    fn skip_ws(&mut self) {
        while let Some(ch) = self.peek_char() {
            if !ch.is_whitespace() {
                break;
            }
            self.pos += ch.len_utf8();
        }
    }

    fn advance_one_char(&mut self) {
        if let Some(ch) = self.peek_char() {
            self.pos += ch.len_utf8();
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}

fn pspice_u_pindly_delay_from_block(block: &str, delay_mode: PspiceUTimingMode) -> Option<Value> {
    let upper = block.to_ascii_uppercase();
    let mut search_start = 0usize;
    let mut selected = None;
    while let Some(relative) = upper[search_start..].find("DELAY(") {
        let args_start = search_start + relative + "DELAY(".len();
        let rest = &block[args_start..];
        let Some(args_end) = rest.find(')') else {
            break;
        };
        if let Some(delay) = pspice_u_pindly_delay_args(&rest[..args_end], delay_mode)
            && selected.is_none_or(|current| delay > current)
        {
            selected = Some(delay);
        }
        search_start = args_start + args_end + 1;
    }
    selected
}

fn pspice_u_pindly_delay_args(args: &str, delay_mode: PspiceUTimingMode) -> Option<Value> {
    let mut values = args
        .split(',')
        .take(3)
        .map(|part| pspice_u_parse_delay_literal(part.trim()))
        .collect::<Vec<_>>();
    while values.len() < 3 {
        values.push(None);
    }

    match delay_mode {
        PspiceUTimingMode::Min => {
            values[0].or_else(|| pspice_u_pindly_typ_delay(values[0], values[1], values[2]))
        }
        PspiceUTimingMode::Typ => pspice_u_pindly_typ_delay(values[0], values[1], values[2]),
        PspiceUTimingMode::Max => {
            values[2].or_else(|| pspice_u_pindly_typ_delay(values[0], values[1], values[2]))
        }
    }
}

fn pspice_u_pindly_typ_delay(
    min: Option<Value>,
    typ: Option<Value>,
    max: Option<Value>,
) -> Option<Value> {
    if let Some(typ) = typ {
        return Some(typ);
    }
    match (min, max) {
        (Some(min), Some(max)) => Some((min + max) * 0.5),
        (Some(min), None) => Some(min),
        (None, Some(max)) => Some(max),
        (None, None) => None,
    }
}

fn pspice_u_parse_delay_literal(raw: &str) -> Option<Value> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed.starts_with('-') {
        return None;
    }
    crate::netlist::lexer::parse_spice_value(trimmed).ok()
}

fn pspice_u_pindly_buffer_delay_params(delay: Value) -> Vec<(String, Value)> {
    vec![
        ("inertial_delay".to_string(), 1.0),
        ("rise_delay".to_string(), delay),
        ("fall_delay".to_string(), delay),
    ]
}

fn pspice_u_pindly_tristate_delay_params(delay: Value) -> Vec<(String, Value)> {
    vec![
        ("inertial_delay".to_string(), 1.0),
        ("delay".to_string(), delay),
    ]
}

fn parse_pspice_u_compound_gate(
    name: &str,
    fields: &[String],
    shape: Option<(usize, usize)>,
    term_model: &str,
    output_model: &str,
    ignored_constant: &str,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let Some((input_count, term_count)) = shape else {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice compound U-device '{}' requires valid type dimensions",
                name
            ),
        });
    };
    if input_count < 2 || term_count == 0 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice compound U-device '{}' requires at least two inputs per term and one term",
                name
            ),
        });
    }

    let input_total = input_count
        .checked_mul(term_count)
        .ok_or_else(|| ParseError::Syntax {
            line: line_num,
            message: format!("PSpice compound U-device '{}' is too large", name),
        })?;
    let required = input_total
        .checked_add(2)
        .ok_or_else(|| ParseError::Syntax {
            line: line_num,
            message: format!("PSpice compound U-device '{}' is too large", name),
        })?;

    let pins = &fields[4..];
    if pins.len() < required {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice compound U-device '{}' requires {} input pin(s), one output pin, and a timing model",
                name, input_total
            ),
        });
    }

    let output = pspice_u_required_digital_port(
        &pins[input_total],
        "compound output",
        fields,
        line_num,
        elements,
    )?;
    let pspice_u_timing =
        pspice_u_timing_from_token(&pins[input_total + 1], fields, params, line_num);
    let mut term_outputs = Vec::with_capacity(term_count);

    for term_index in 0..term_count {
        let input_start = term_index * input_count;
        let mut kept_inputs = Vec::new();
        for pin in &pins[input_start..input_start + input_count] {
            if pin.trim().eq_ignore_ascii_case(ignored_constant) {
                continue;
            }
            kept_inputs.push(pspice_u_required_digital_node(
                pin,
                "compound input",
                fields,
                line_num,
                elements,
            )?);
        }

        let term_output = match kept_inputs.len() {
            0 => {
                let folded = pspice_u_compound_folded_constant(term_model);
                ensure_pspice_u_constant_driver(folded, elements);
                normalize_pspice_u_node(folded)
            }
            1 => kept_inputs.remove(0),
            _ => {
                let instance_name = pspice_u_lowered_instance_name(name, term_count, term_index);
                let connector = pspice_u_internal_compound_connector_name(&instance_name);
                push_pspice_u_xspice_element_with_params(
                    elements,
                    format!("{instance_name}__GATE"),
                    term_model,
                    vec![
                        XspicePort::DigitalVector(kept_inputs),
                        XspicePort::Digital(connector.clone()),
                    ],
                    pspice_u_zero_gate_delay_params(),
                    None,
                );
                connector
            }
        };
        term_outputs.push(term_output);
    }

    let (final_model, input_port) =
        pspice_u_compound_output_model_and_port(output_model, term_outputs);
    push_pspice_u_xspice_element_with_timing(
        elements,
        name.to_string(),
        final_model,
        vec![input_port, output],
        pspice_u_timing,
    );

    Ok(())
}

fn pspice_u_compound_folded_constant(term_model: &str) -> &'static str {
    if term_model.eq_ignore_ascii_case("d_or") {
        "$D_LO"
    } else {
        "$D_HI"
    }
}

fn pspice_u_compound_output_model_and_port(
    output_model: &str,
    term_outputs: Vec<String>,
) -> (&'static str, XspicePort) {
    if term_outputs.len() == 1 {
        let model = if matches!(
            output_model.to_ascii_lowercase().as_str(),
            "d_nand" | "d_nor"
        ) {
            "d_inverter"
        } else {
            "d_buffer"
        };
        return (model, XspicePort::Digital(term_outputs[0].clone()));
    }

    let model = match output_model.to_ascii_lowercase().as_str() {
        "d_and" => "d_and",
        "d_nand" => "d_nand",
        "d_nor" => "d_nor",
        "d_or" => "d_or",
        _ => "d_or",
    };
    (model, XspicePort::DigitalVector(term_outputs))
}

fn push_pspice_u_tristate_vector_gate(
    elements: &mut Vec<Element>,
    instance_name: &str,
    primary_model: &str,
    inputs: Vec<String>,
    enable: XspicePort,
    output: XspicePort,
    pspice_u_timing: Option<PspiceUTiming>,
) {
    let connector = pspice_u_internal_connector_name(instance_name);
    let primary_name = format!("{instance_name}__GATE");
    push_pspice_u_xspice_element_with_params(
        elements,
        primary_name,
        primary_model,
        vec![
            XspicePort::DigitalVector(inputs),
            XspicePort::Digital(connector.clone()),
        ],
        pspice_u_zero_gate_delay_params(),
        None,
    );
    push_pspice_u_xspice_element_with_timing(
        elements,
        instance_name.to_string(),
        "d_tristate",
        vec![XspicePort::Digital(connector), enable, output],
        pspice_u_timing,
    );
}

fn parse_pspice_u_pull(
    name: &str,
    fields: &[String],
    count: usize,
    model: &str,
    line_num: usize,
    elements: &mut Vec<Element>,
) -> Result<(), ParseError> {
    if count == 0 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice U-device '{}' type '{}' requires at least one pull device",
                name, fields[1]
            ),
        });
    }

    let pins = &fields[4..];
    if pins.len() < count {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice pull U-device '{}' requires {} output pin(s)",
                name, count
            ),
        });
    }

    for (index, pin) in pins.iter().take(count).enumerate() {
        let output =
            pspice_u_required_digital_port(pin, "pull output", fields, line_num, elements)?;
        let instance_name = pspice_u_lowered_instance_name(name, count, index);
        push_pspice_u_xspice_element(elements, instance_name, model, vec![output]);
    }

    Ok(())
}

fn parse_pspice_u_dlyline(
    name: &str,
    fields: &[String],
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let pins = &fields[4..];
    if pins.len() < 3 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice DLYLINE U-device '{}' requires input, output, and a timing model",
                name
            ),
        });
    }

    let input =
        pspice_u_required_digital_port(&pins[0], "delay-line input", fields, line_num, elements)?;
    let output =
        pspice_u_required_digital_port(&pins[1], "delay-line output", fields, line_num, elements)?;
    let pspice_u_timing = pspice_u_timing_from_token(&pins[2], fields, params, line_num);
    push_pspice_u_xspice_element_with_timing(
        elements,
        name.to_string(),
        "d_buffer",
        vec![input, output],
        pspice_u_timing,
    );

    Ok(())
}

fn parse_pspice_u_jkff(
    name: &str,
    fields: &[String],
    count: usize,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    if count == 0 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice U-device '{}' type '{}' requires at least one JKFF",
                name, fields[1]
            ),
        });
    }

    let pins = &fields[4..];
    let required = 3 + count * 4 + 1;
    if pins.len() < required {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice JKFF U-device '{}' requires PREBAR, CLRBAR, CLKBAR, {} J input(s), {} K input(s), {} Q output(s), {} QBAR output(s), and a timing model",
                name, count, count, count, count
            ),
        });
    }

    let prebar = pspice_u_active_low_control_port(&pins[0], elements);
    let clrbar = pspice_u_active_low_control_port(&pins[1], elements);
    let clkbar =
        pspice_u_required_inverted_digital_port(&pins[2], "clock", fields, line_num, elements)?;
    let pspice_u_timing = pspice_u_timing_from_token(&pins[required - 1], fields, params, line_num);
    let j_offset = 3;
    let k_offset = j_offset + count;
    let q_offset = k_offset + count;
    let qb_offset = q_offset + count;

    for index in 0..count {
        let j = pspice_u_required_digital_port(
            &pins[j_offset + index],
            "J input",
            fields,
            line_num,
            elements,
        )?;
        let k = pspice_u_required_digital_port(
            &pins[k_offset + index],
            "K input",
            fields,
            line_num,
            elements,
        )?;
        let q = pspice_u_nullable_output_port(&pins[q_offset + index]);
        let qb = pspice_u_nullable_output_port(&pins[qb_offset + index]);
        let instance_name = pspice_u_lowered_instance_name(name, count, index);
        let ports = vec![j, k, clkbar.clone(), prebar.clone(), clrbar.clone(), q, qb];
        push_pspice_u_xspice_element_with_timing(
            elements,
            instance_name,
            "d_jkff",
            ports,
            pspice_u_timing.clone(),
        );
    }

    Ok(())
}

fn parse_pspice_u_dlatch(
    name: &str,
    fields: &[String],
    count: usize,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    if count == 0 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice U-device '{}' type '{}' requires at least one D latch",
                name, fields[1]
            ),
        });
    }

    let pins = &fields[4..];
    let required = 3 + count * 3 + 1;
    if pins.len() < required {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice DLTCH U-device '{}' requires PREBAR, CLRBAR, GATE, {} D input(s), {} Q output(s), {} QBAR output(s), and a timing model",
                name, count, count, count
            ),
        });
    }

    let prebar = pspice_u_active_low_control_port(&pins[0], elements);
    let clrbar = pspice_u_active_low_control_port(&pins[1], elements);
    let enable = pspice_u_required_digital_port(&pins[2], "enable", fields, line_num, elements)?;
    let pspice_u_timing = pspice_u_timing_from_token(&pins[required - 1], fields, params, line_num);
    let d_offset = 3;
    let q_offset = d_offset + count;
    let qb_offset = q_offset + count;

    for index in 0..count {
        let data = pspice_u_required_digital_port(
            &pins[d_offset + index],
            "D input",
            fields,
            line_num,
            elements,
        )?;
        let q = pspice_u_nullable_output_port(&pins[q_offset + index]);
        let qb = pspice_u_nullable_output_port(&pins[qb_offset + index]);
        let instance_name = pspice_u_lowered_instance_name(name, count, index);
        let ports = vec![data, enable.clone(), prebar.clone(), clrbar.clone(), q, qb];
        push_pspice_u_xspice_element_with_timing(
            elements,
            instance_name,
            "d_dlatch",
            ports,
            pspice_u_timing.clone(),
        );
    }

    Ok(())
}

fn parse_pspice_u_srlatch(
    name: &str,
    fields: &[String],
    count: usize,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    if count == 0 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice U-device '{}' type '{}' requires at least one SR latch",
                name, fields[1]
            ),
        });
    }

    let pins = &fields[4..];
    let required = 3 + count * 4 + 1;
    if pins.len() < required {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice SRFF U-device '{}' requires PREBAR, CLRBAR, GATE, {} S input(s), {} R input(s), {} Q output(s), {} QBAR output(s), and a timing model",
                name, count, count, count, count
            ),
        });
    }

    let prebar = pspice_u_active_low_control_port(&pins[0], elements);
    let clrbar = pspice_u_active_low_control_port(&pins[1], elements);
    let enable = pspice_u_required_digital_port(&pins[2], "enable", fields, line_num, elements)?;
    let pspice_u_timing = pspice_u_timing_from_token(&pins[required - 1], fields, params, line_num);
    let s_offset = 3;
    let r_offset = s_offset + count;
    let q_offset = r_offset + count;
    let qb_offset = q_offset + count;

    for index in 0..count {
        let set = pspice_u_required_digital_port(
            &pins[s_offset + index],
            "S input",
            fields,
            line_num,
            elements,
        )?;
        let reset = pspice_u_required_digital_port(
            &pins[r_offset + index],
            "R input",
            fields,
            line_num,
            elements,
        )?;
        let q = pspice_u_nullable_output_port(&pins[q_offset + index]);
        let qb = pspice_u_nullable_output_port(&pins[qb_offset + index]);
        let instance_name = pspice_u_lowered_instance_name(name, count, index);
        let ports = vec![
            set,
            reset,
            enable.clone(),
            prebar.clone(),
            clrbar.clone(),
            q,
            qb,
        ];
        push_pspice_u_xspice_element_with_timing(
            elements,
            instance_name,
            "d_srlatch",
            ports,
            pspice_u_timing.clone(),
        );
    }

    Ok(())
}

fn parse_pspice_u_tristate(
    name: &str,
    fields: &[String],
    count: usize,
    invert_input: bool,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    if count == 0 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice U-device '{}' type '{}' requires at least one tri-state buffer",
                name, fields[1]
            ),
        });
    }

    let pins = &fields[4..];
    let required = count + 1 + count + 1;
    if pins.len() < required {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice tri-state U-device '{}' requires {} input pin(s), one enable pin, {} output pin(s), and a timing model",
                name, count, count
            ),
        });
    }

    let enable =
        pspice_u_required_digital_port(&pins[count], "enable", fields, line_num, elements)?;
    let output_offset = count + 1;
    let pspice_u_timing = pspice_u_timing_from_token(&pins[required - 1], fields, params, line_num);

    for index in 0..count {
        let input = if invert_input {
            pspice_u_required_inverted_digital_port(
                &pins[index],
                "tri-state input",
                fields,
                line_num,
                elements,
            )?
        } else {
            pspice_u_required_digital_port(
                &pins[index],
                "tri-state input",
                fields,
                line_num,
                elements,
            )?
        };
        let output = pspice_u_required_digital_port(
            &pins[output_offset + index],
            "tri-state output",
            fields,
            line_num,
            elements,
        )?;
        let instance_name = pspice_u_lowered_instance_name(name, count, index);
        let ports = vec![input, enable.clone(), output];
        push_pspice_u_xspice_element_with_timing(
            elements,
            instance_name,
            "d_tristate",
            ports,
            pspice_u_timing.clone(),
        );
    }

    Ok(())
}

fn push_pspice_u_xspice_element(
    elements: &mut Vec<Element>,
    name: String,
    model: &str,
    ports: Vec<XspicePort>,
) {
    push_pspice_u_xspice_element_with_timing(elements, name, model, ports, None);
}

fn push_pspice_u_xspice_element_with_timing(
    elements: &mut Vec<Element>,
    name: String,
    model: &str,
    ports: Vec<XspicePort>,
    pspice_u_timing: Option<PspiceUTiming>,
) {
    push_pspice_u_xspice_element_with_params(
        elements,
        name,
        model,
        ports,
        Vec::new(),
        pspice_u_timing,
    );
}

fn push_pspice_u_xspice_element_with_params(
    elements: &mut Vec<Element>,
    name: String,
    model: &str,
    ports: Vec<XspicePort>,
    params: Vec<(String, Value)>,
    pspice_u_timing: Option<PspiceUTiming>,
) {
    elements.push(Element {
        name,
        kind: ElementKind::Xspice {
            model: model.to_string(),
            pspice_u_timing,
            ports,
            params,
            expr_params: Vec::new(),
            string_params: Vec::new(),
            string_expr_params: Vec::new(),
            string_vector_params: Vec::new(),
            string_vector_expr_params: Vec::new(),
            real_vector_params: Vec::new(),
            real_vector_expr_params: Vec::new(),
        },
        nodes: Vec::new(),
    });
}

fn pspice_u_zero_gate_delay_params() -> Vec<(String, Value)> {
    vec![
        ("inertial_delay".to_string(), 1.0),
        ("rise_delay".to_string(), 1.0e-12),
        ("fall_delay".to_string(), 1.0e-12),
    ]
}

fn pspice_u_internal_connector_name(instance_name: &str) -> String {
    pspice_u_internal_connector_name_with_suffix(instance_name, "TRI")
}

fn pspice_u_internal_compound_connector_name(instance_name: &str) -> String {
    pspice_u_internal_connector_name_with_suffix(instance_name, "CMP")
}

fn pspice_u_internal_connector_name_with_suffix(instance_name: &str, suffix: &str) -> String {
    let mut name = String::from("__PSPICE_");
    for ch in instance_name.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            name.push(ch.to_ascii_uppercase());
        } else {
            name.push('_');
        }
    }
    name.push('_');
    name.push_str(suffix);
    name
}

fn pspice_u_lowered_instance_name(name: &str, count: usize, index: usize) -> String {
    if count == 1 {
        name.to_string()
    } else {
        format!("{name}_{index}")
    }
}

fn pspice_u_required_digital_port(
    raw: &str,
    role: &str,
    fields: &[String],
    line_num: usize,
    elements: &mut Vec<Element>,
) -> Result<XspicePort, ParseError> {
    Ok(XspicePort::Digital(pspice_u_required_digital_node(
        raw, role, fields, line_num, elements,
    )?))
}

fn pspice_u_required_digital_node(
    raw: &str,
    role: &str,
    fields: &[String],
    line_num: usize,
    elements: &mut Vec<Element>,
) -> Result<String, ParseError> {
    if pspice_u_is_no_connect(raw) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "PSpice U-device '{}' type '{}' cannot use {} as a required {}",
                fields[0], fields[1], raw, role
            ),
        });
    }

    ensure_pspice_u_constant_driver(raw, elements);
    Ok(normalize_pspice_u_node(raw))
}

fn pspice_u_required_inverted_digital_port(
    raw: &str,
    role: &str,
    fields: &[String],
    line_num: usize,
    elements: &mut Vec<Element>,
) -> Result<XspicePort, ParseError> {
    Ok(XspicePort::DigitalInverted(pspice_u_required_digital_node(
        raw, role, fields, line_num, elements,
    )?))
}

fn pspice_u_active_low_control_port(raw: &str, elements: &mut Vec<Element>) -> XspicePort {
    if pspice_u_is_inactive_control(raw) {
        XspicePort::Null
    } else {
        ensure_pspice_u_constant_driver(raw, elements);
        XspicePort::DigitalInverted(normalize_pspice_u_node(raw))
    }
}

fn pspice_u_nullable_output_port(raw: &str) -> XspicePort {
    if pspice_u_is_no_connect(raw) {
        XspicePort::Null
    } else {
        XspicePort::Digital(normalize_pspice_u_node(raw))
    }
}

struct PspiceSimpleUGate {
    xspice_model: &'static str,
    input_count: usize,
}

fn parse_pspice_simple_u_gate(raw: &str) -> Option<PspiceSimpleUGate> {
    let (kind, count) = parse_pspice_u_kind_and_count(raw);
    let kind = kind.as_str();
    let default_count = match kind {
        "BUF" | "INV" => 1,
        "AND" | "NAND" | "OR" | "NOR" | "XOR" | "XNOR" | "NXOR" => 2,
        _ => return None,
    };
    let input_count = count.unwrap_or(default_count);
    if input_count == 0 {
        return None;
    }
    if matches!(kind, "BUF" | "INV") && input_count != 1 {
        return None;
    }

    let xspice_model = match kind {
        "AND" => "d_and",
        "NAND" => "d_nand",
        "OR" => "d_or",
        "NOR" => "d_nor",
        "XOR" => "d_xor",
        "XNOR" | "NXOR" => "d_xnor",
        "BUF" => "d_buffer",
        "INV" => "d_inverter",
        _ => return None,
    };

    Some(PspiceSimpleUGate {
        xspice_model,
        input_count,
    })
}

fn parse_pspice_u_kind_and_count(raw: &str) -> (String, Option<usize>) {
    let trimmed = raw.trim();
    if let Some((kind, tail)) = trimmed.split_once('(')
        && let Some(count) = tail.strip_suffix(')')
    {
        return (
            kind.to_ascii_uppercase(),
            count.trim().parse::<usize>().ok(),
        );
    }
    (trimmed.to_ascii_uppercase(), None)
}

fn pspice_u_count_pair(raw: &str) -> Option<(usize, usize)> {
    let (_, tail) = raw.trim().split_once('(')?;
    let counts = tail.strip_suffix(')')?;
    let mut parts = counts.split(',');
    let first = parts.next()?.trim().parse::<usize>().ok()?;
    let second = parts.next()?.trim().parse::<usize>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((first, second))
}

fn pspice_u_count_triple(raw: &str) -> Option<(usize, usize, usize)> {
    let (_, tail) = raw.trim().split_once('(')?;
    let counts = tail.strip_suffix(')')?;
    let mut parts = counts.split(',');
    let first = parts.next()?.trim().parse::<usize>().ok()?;
    let second = parts.next()?.trim().parse::<usize>().ok()?;
    let third = parts.next()?.trim().parse::<usize>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((first, second, third))
}

fn normalize_pspice_u_node(raw: &str) -> String {
    raw.trim().to_ascii_uppercase()
}

fn pspice_u_timing_model_token(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed.contains('=') {
        return None;
    }
    Some(trimmed.to_ascii_uppercase())
}

fn pspice_u_timing_from_token(
    raw: &str,
    fields: &[String],
    params: &ParamContext,
    line_num: usize,
) -> Option<PspiceUTiming> {
    let timing_model = pspice_u_timing_model_token(raw)?;
    Some(PspiceUTiming {
        timing_model,
        delay_mode: pspice_u_delay_mode(fields, fields.len(), params, line_num),
    })
}

fn pspice_u_is_no_connect(raw: &str) -> bool {
    let normalized = raw.trim().to_ascii_uppercase();
    matches!(normalized.as_str(), "$D_NC" | "NULL")
}

fn pspice_u_is_inactive_control(raw: &str) -> bool {
    let normalized = raw.trim().to_ascii_uppercase();
    matches!(normalized.as_str(), "$D_HI" | "$D_NC" | "NULL")
}

fn ensure_pspice_u_constant_driver(raw: &str, elements: &mut Vec<Element>) {
    let normalized = raw.trim().to_ascii_uppercase();
    let Some((node, model, base_name)) = (match normalized.as_str() {
        "$D_HI" => Some(("$D_HI", "d_pullup", "A__PSPICE_D_HI")),
        "$D_LO" => Some(("$D_LO", "d_pulldown", "A__PSPICE_D_LO")),
        _ => None,
    }) else {
        return;
    };

    if elements.iter().any(|element| {
        matches!(
            &element.kind,
            ElementKind::Xspice { model: existing_model, ports, .. }
                if existing_model.eq_ignore_ascii_case(model)
                    && ports == &[XspicePort::Digital(node.to_string())]
        )
    }) {
        return;
    }

    let name = unique_pspice_u_generated_name(elements, base_name);
    push_pspice_u_xspice_element(
        elements,
        name,
        model,
        vec![XspicePort::Digital(node.to_string())],
    );
}

fn unique_pspice_u_generated_name(elements: &[Element], base_name: &str) -> String {
    if !elements
        .iter()
        .any(|element| element.name.eq_ignore_ascii_case(base_name))
    {
        return base_name.to_string();
    }

    let mut suffix = 1usize;
    loop {
        let candidate = format!("{base_name}_{suffix}");
        if !elements
            .iter()
            .any(|element| element.name.eq_ignore_ascii_case(&candidate))
        {
            return candidate;
        }
        suffix += 1;
    }
}

pub(super) fn parse_diode(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
    defer_simple_param_refs: bool,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let anode = expect_node(stream, line_num)?;
    let cathode = expect_node(stream, line_num)?;
    let model = expect_ident(stream, line_num)?;

    // Instance tail: positional AREA, bare OFF keyword, and PARAM=value
    // assignments (AREA/M/PJ/TEMP/DTEMP/IC...), mirroring ngspice's D-line
    // grammar.
    let mut instance_params = Vec::new();
    let mut deferred_params = Vec::new();
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
                    match take_deferrable_value(stream, params, defer_simple_param_refs) {
                        Some(DeferrableValue::Resolved(value)) => {
                            instance_params.push((name_upper, value));
                        }
                        Some(DeferrableValue::Deferred(expr)) => {
                            deferred_params.push((name_upper, expr));
                        }
                        None => {
                            return Err(ParseError::Syntax {
                                line: line_num,
                                message: format!(
                                    "Expected value for diode parameter '{}'",
                                    raw_name
                                ),
                            });
                        }
                    }
                    continue;
                }

                if !area_positional_seen
                    && let Ok(parsed) = crate::netlist::lexer::parse_spice_value(&raw_name)
                {
                    instance_params.push(("AREA".to_string(), parsed));
                    area_positional_seen = true;
                    continue;
                }

                let message = if is_diode_assignment_name(&name_upper)
                    && token_starts_unassigned_value(&stream.peek().kind, params)
                {
                    format!("diode parameter '{}' expected '=' before value", raw_name)
                } else {
                    format!(
                        "Unsupported diode instance token '{}'; expected NAME=value, positional AREA, or OFF",
                        raw_name
                    )
                };
                return Err(ParseError::Syntax {
                    line: line_num,
                    message,
                });
            }
            TokenKind::Number(v) => {
                if !area_positional_seen {
                    instance_params.push(("AREA".to_string(), *v));
                    area_positional_seen = true;
                    stream.advance();
                    continue;
                }

                return Err(ParseError::Syntax {
                    line: line_num,
                    message: "Duplicate positional AREA for diode instance".to_string(),
                });
            }
            _ => {
                if !area_positional_seen && let Some(value) = try_value(stream, params) {
                    instance_params.push(("AREA".to_string(), value));
                    area_positional_seen = true;
                    continue;
                }

                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "Unsupported diode instance token '{}'; expected NAME=value, positional AREA, or OFF",
                        stream.peek().kind
                    ),
                });
            }
        }
    }

    elements.push(Element {
        name,
        kind: ElementKind::Diode {
            model,
            instance_params,
            deferred_params,
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
    defer_simple_param_refs: bool,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let collector = expect_node(stream, line_num)?;
    let base = expect_node(stream, line_num)?;
    let emitter = expect_node(stream, line_num)?;

    // BJT can have optional substrate node: Q1 C B E [S] model
    // We need to peek ahead to determine if next is substrate or model
    let (substrate, mut model) = match &stream.peek().kind {
        TokenKind::Number(_) => {
            // It's a numeric node (substrate like "0")
            let substrate = expect_node(stream, line_num)?;
            let model = expect_ident(stream, line_num)?;
            (Some(substrate), model)
        }
        TokenKind::LBracket => {
            stream.advance();
            let substrate = expect_node(stream, line_num)?;
            if !stream.consume(&TokenKind::RBracket) {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: "Expected closing ']' after BJT substrate node".to_string(),
                });
            }
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
                    } else if is_bjt_assignment_name(&next_upper)
                        && token_starts_unassigned_value(&stream.peek_n(1).kind, params)
                    {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: format!(
                                "BJT parameter '{}' expected '=' before value",
                                next_ident
                            ),
                        });
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

    let mut thermal = None;
    if substrate.is_some() {
        if let TokenKind::Ident(next_model) = &stream.peek().kind {
            let next_upper = next_model.to_ascii_uppercase();
            if !matches!(stream.peek_n(1).kind, TokenKind::Equals) && next_upper != "OFF" {
                thermal = Some(model);
                model = next_model.clone();
                stream.advance();
            }
        }
    }

    let mut nodes = vec![collector, base, emitter];
    if let Some(sub) = substrate {
        nodes.push(sub);
    }
    if let Some(thermal) = thermal {
        nodes.push(thermal);
    }

    let mut instance_params = Vec::new();
    let mut deferred_params = Vec::new();
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
                    match take_deferrable_value(stream, params, defer_simple_param_refs) {
                        Some(DeferrableValue::Resolved(value)) => {
                            instance_params.push((name_upper, value));
                        }
                        Some(DeferrableValue::Deferred(expr)) => {
                            deferred_params.push((name_upper, expr));
                        }
                        None => {
                            return Err(ParseError::Syntax {
                                line: line_num,
                                message: format!("Expected value for BJT parameter '{}'", raw_name),
                            });
                        }
                    }
                    continue;
                }

                if !area_positional_seen
                    && let Ok(parsed) = crate::netlist::lexer::parse_spice_value(&raw_name)
                {
                    instance_params.push(("AREA".to_string(), parsed));
                    area_positional_seen = true;
                    continue;
                }

                let message = if is_bjt_assignment_name(&name_upper)
                    && token_starts_unassigned_value(&stream.peek().kind, params)
                {
                    format!("BJT parameter '{}' expected '=' before value", raw_name)
                } else {
                    format!(
                        "Unsupported BJT instance token '{}'; expected NAME=value, positional AREA, or OFF",
                        raw_name
                    )
                };
                return Err(ParseError::Syntax {
                    line: line_num,
                    message,
                });
            }
            TokenKind::Number(v) => {
                // Optional positional area scaling.
                if !area_positional_seen {
                    instance_params.push(("AREA".to_string(), *v));
                    area_positional_seen = true;
                    stream.advance();
                    continue;
                }

                return Err(ParseError::Syntax {
                    line: line_num,
                    message: "Duplicate positional AREA for BJT instance".to_string(),
                });
            }
            _ => {
                if !area_positional_seen && let Some(value) = try_value(stream, params) {
                    instance_params.push(("AREA".to_string(), value));
                    area_positional_seen = true;
                    continue;
                }

                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "Unsupported BJT instance token '{}'; expected NAME=value, positional AREA, or OFF",
                        stream.peek().kind
                    ),
                });
            }
        }
    }

    elements.push(Element {
        name,
        kind: ElementKind::Bjt {
            model,
            bjt_type: super::BjtType::Npn, // Will be set from model
            instance_params,
            deferred_params,
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
    _diagnostics: &mut Vec<ParseDiagnostic>,
    defer_simple_param_refs: bool,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let drain = expect_node(stream, line_num)?;
    let gate = expect_node(stream, line_num)?;
    let source = expect_node(stream, line_num)?;
    let bulk_or_model = expect_node(stream, line_num)?;

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
            if let TokenKind::Ident(raw_name) = &stream.peek().kind {
                let name_upper = raw_name.to_ascii_uppercase();
                if !tail_tokens.is_empty()
                    && is_mosfet_assignment_name(&name_upper)
                    && mosfet_token_starts_unassigned_value(&stream.peek_n(1).kind, params)
                {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!(
                            "MOSFET parameter '{}' expected '=' before value",
                            raw_name
                        ),
                    });
                }
            }
            tail_tokens.push(expect_node(stream, line_num)?);
            continue;
        }
        break;
    }

    let mut tail_off_flag = false;
    if tail_tokens.len() >= 2
        && tail_tokens
            .last()
            .is_some_and(|token| token.eq_ignore_ascii_case("OFF"))
    {
        tail_tokens.pop();
        tail_off_flag = true;
    }

    let (bulk, model, compact_syntax) = if let Some(model) = tail_tokens.pop() {
        (bulk_or_model, model, false)
    } else {
        // ngspice VDMOS uses the compact three-terminal MOS form
        // `Mname D G S model`; source is the implicit body reference.
        (source.clone(), bulk_or_model, true)
    };

    let mut nodes = vec![drain, gate, source, bulk];
    nodes.extend(tail_tokens);

    let mut instance_params = Vec::new();
    let mut deferred_params = Vec::new();
    if tail_off_flag {
        instance_params.push(("OFF".to_string(), 1.0));
    }
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
                    if name_upper == "IC" {
                        parse_mosfet_ic_vector(
                            stream,
                            line_num,
                            params,
                            defer_simple_param_refs,
                            &mut instance_params,
                            &mut deferred_params,
                        )?;
                        continue;
                    }

                    match take_deferrable_value(stream, params, defer_simple_param_refs) {
                        Some(DeferrableValue::Resolved(value)) => {
                            instance_params.push((name_upper, value));
                        }
                        Some(DeferrableValue::Deferred(expr)) => {
                            deferred_params.push((name_upper, expr));
                        }
                        None => {
                            return Err(ParseError::Syntax {
                                line: line_num,
                                message: format!(
                                    "Expected value for MOSFET parameter '{}'",
                                    raw_name
                                ),
                            });
                        }
                    }
                } else {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!(
                            "Unsupported MOSFET instance token '{}'; expected NAME=value or OFF",
                            raw_name
                        ),
                    });
                }
            }
            _ => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "Unsupported MOSFET instance token '{}'; expected NAME=value or OFF",
                        stream.peek().kind
                    ),
                });
            }
        }
    }

    elements.push(Element {
        name,
        kind: ElementKind::Mosfet {
            model,
            mos_type: super::MosType::Nmos, // Will be set from model
            compact_syntax,
            instance_params,
            deferred_params,
        },
        nodes,
    });

    Ok(())
}

fn parse_mosfet_ic_vector(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    defer_simple_param_refs: bool,
    instance_params: &mut Vec<(String, Value)>,
    deferred_params: &mut Vec<(String, String)>,
) -> Result<(), ParseError> {
    for (idx, label) in ["IC_VDS", "IC_VGS", "IC_VBS"].iter().enumerate() {
        let value = take_mosfet_ic_value(stream, line_num, params, defer_simple_param_refs)?;
        match value {
            DeferrableValue::Resolved(value) => instance_params.push(((*label).to_string(), value)),
            DeferrableValue::Deferred(expr) => deferred_params.push(((*label).to_string(), expr)),
        }

        if idx == 2 || !stream.consume(&TokenKind::Comma) {
            break;
        }
    }

    Ok(())
}

fn take_mosfet_ic_value(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    defer_simple_param_refs: bool,
) -> Result<DeferrableValue, ParseError> {
    if matches!(stream.peek().kind, TokenKind::Plus | TokenKind::Minus) {
        return expect_value(stream, line_num, params).map(DeferrableValue::Resolved);
    }

    take_deferrable_value(stream, params, defer_simple_param_refs).ok_or_else(|| {
        ParseError::Syntax {
            line: line_num,
            message: format!(
                "Expected value for MOSFET IC vector, found {}",
                stream.peek().kind
            ),
        }
    })
}

fn is_mosfet_assignment_name(name_upper: &str) -> bool {
    matches!(
        name_upper,
        "L" | "W"
            | "M"
            | "NF"
            | "AD"
            | "AS"
            | "PD"
            | "PS"
            | "NRD"
            | "NRS"
            | "SA"
            | "SB"
            | "SD"
            | "SCA"
            | "SCB"
            | "SCC"
            | "SC"
            | "TEMP"
            | "DTEMP"
            | "IC"
            | "VDS"
            | "VGS"
            | "VBS"
            | "DELVTO"
            | "GEOMOD"
    )
}

fn is_diode_assignment_name(name_upper: &str) -> bool {
    matches!(
        name_upper,
        "AREA" | "M" | "MULT" | "PJ" | "OFF" | "TEMP" | "DTEMP" | "IC" | "NOISY" | "NOISE"
    )
}

fn is_bjt_assignment_name(name_upper: &str) -> bool {
    matches!(
        name_upper,
        "AREA" | "AREAB" | "AREAC" | "M" | "MULT" | "OFF" | "TEMP" | "DTEMP" | "IC"
    )
}

fn token_starts_unassigned_value(kind: &TokenKind, params: &ParamContext) -> bool {
    match kind {
        TokenKind::Number(_) | TokenKind::Expression(_) | TokenKind::Plus | TokenKind::Minus => {
            true
        }
        TokenKind::Ident(name) => {
            params.get(name).is_some() || crate::netlist::lexer::parse_spice_value(name).is_ok()
        }
        _ => false,
    }
}

fn mosfet_token_starts_unassigned_value(kind: &TokenKind, params: &ParamContext) -> bool {
    token_starts_unassigned_value(kind, params)
}

pub(super) fn parse_jfet(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
    defer_simple_param_refs: bool,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let drain = expect_node(stream, line_num)?;
    let gate = expect_node(stream, line_num)?;
    let source = expect_node(stream, line_num)?;
    let model = expect_ident(stream, line_num)?;
    let (instance_params, deferred_params) =
        parse_fet_instance_params(stream, line_num, params, defer_simple_param_refs, "JFET")?;

    elements.push(Element {
        name,
        kind: ElementKind::Jfet {
            model,
            jfet_type: super::JfetType::Njf, // Will be set from model
            instance_params,
            deferred_params,
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
    defer_simple_param_refs: bool,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let drain = expect_node(stream, line_num)?;
    let gate = expect_node(stream, line_num)?;
    let source = expect_node(stream, line_num)?;
    let model = expect_ident(stream, line_num)?;
    let (instance_params, deferred_params) =
        parse_fet_instance_params(stream, line_num, params, defer_simple_param_refs, "MESFET")?;

    elements.push(Element {
        name,
        kind: ElementKind::Mesfet {
            model,
            mesfet_type: super::MesfetType::Nmf, // Will be set from model
            instance_params,
            deferred_params,
        },
        nodes: vec![drain, gate, source],
    });

    Ok(())
}

type ParsedInstanceParams = (Vec<(String, Value)>, Vec<(String, String)>);

pub(super) fn parse_fet_instance_params(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    defer_simple_param_refs: bool,
    element_label: &str,
) -> Result<ParsedInstanceParams, ParseError> {
    let mut instance_params = Vec::new();
    let mut deferred_params = Vec::new();
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
                    match take_deferrable_value(stream, params, defer_simple_param_refs) {
                        Some(DeferrableValue::Resolved(value)) => {
                            instance_params.push((name_upper, value));
                        }
                        Some(DeferrableValue::Deferred(expr)) => {
                            deferred_params.push((name_upper, expr));
                        }
                        None => {
                            return Err(ParseError::Syntax {
                                line: line_num,
                                message: format!(
                                    "Expected value for {} parameter '{}'",
                                    element_label, raw_name
                                ),
                            });
                        }
                    }
                    continue;
                }

                if name_upper == "OFF" {
                    instance_params.push(("OFF".to_string(), 1.0));
                    continue;
                }

                if !area_positional_seen
                    && let Ok(parsed) = crate::netlist::lexer::parse_spice_value(&raw_name)
                {
                    instance_params.push(("AREA".to_string(), parsed));
                    area_positional_seen = true;
                    continue;
                }

                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "Unsupported {} instance token '{}'; expected NAME=value, positional AREA, or OFF",
                        element_label, raw_name
                    ),
                });
            }
            _ => {
                if !area_positional_seen && let Some(value) = try_value(stream, params) {
                    instance_params.push(("AREA".to_string(), value));
                    area_positional_seen = true;
                    continue;
                }

                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "Unsupported {} instance token '{}'; expected NAME=value, positional AREA, or OFF",
                        element_label,
                        stream.peek().kind
                    ),
                });
            }
        }
    }

    Ok((instance_params, deferred_params))
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
    if let Some(keyword) = xyce_ydevice_keyword(&name) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "{keyword} is an unsupported Xyce Y-device keyword with no native implementation yet; refusing to parse it as a Y-line transmission line"
            ),
        });
    }
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

fn xyce_ydevice_keyword(name: &str) -> Option<&'static str> {
    let upper = name.to_ascii_uppercase();
    match upper.as_str() {
        "YACC" => Some("YACC"),
        "YADC" => Some("YADC"),
        "YAND" => Some("YAND"),
        "YDAC" => Some("YDAC"),
        "YDELAY" => Some("YDELAY"),
        "YDFF" => Some("YDFF"),
        "YLIN" => Some("YLIN"),
        "YMEMRISTOR" => Some("YMEMRISTOR"),
        "YNAND" => Some("YNAND"),
        "YNEURON" => Some("YNEURON"),
        "YNOT" => Some("YNOT"),
        "YOR" => Some("YOR"),
        "YPDE" => Some("YPDE"),
        "YRXN" => Some("YRXN"),
        "YSYNAPSE" => Some("YSYNAPSE"),
        "YTRANSLINE" => Some("YTRANSLINE"),
        "YXOR" => Some("YXOR"),
        _ => None,
    }
}

/// Parse Xyce RF ports or coupled transmission lines (P element).
pub(super) fn parse_coupled_tlines(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    if xyce_port_tail_is_present(stream, line_num) {
        return parse_xyce_port(stream, line_num, elements, params);
    }

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

fn xyce_port_tail_is_present(stream: &TokenStream, line_num: usize) -> bool {
    let mut probe = stream.clone();
    if expect_ident(&mut probe, line_num).is_err()
        || expect_node(&mut probe, line_num).is_err()
        || expect_node(&mut probe, line_num).is_err()
    {
        return false;
    }

    while !probe.is_eof() && !matches!(probe.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        skip_commas(&mut probe);
        let TokenKind::Ident(raw) = &probe.peek().kind else {
            probe.advance();
            continue;
        };
        if is_xyce_port_assignment(raw) || is_xyce_port_source_keyword(raw) {
            return true;
        }
        probe.advance();
    }

    false
}

fn parse_xyce_port(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let node_pos = expect_node(stream, line_num)?;
    let node_neg = expect_node(stream, line_num)?;

    let mut z0 = 50.0;
    let mut source_tokens = Vec::new();
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        skip_commas(stream);
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }

        if let TokenKind::Ident(raw) = &stream.peek().kind {
            let upper = raw.to_ascii_uppercase();
            if is_xyce_port_assignment(&upper) {
                stream.advance();
                skip_commas(stream);
                stream.consume(&TokenKind::Equals);
                let value = expect_value(stream, line_num, params)?;
                if upper == "Z0" {
                    if value <= 0.0 || !value.is_finite() {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: format!(
                                "Xyce port '{}' requires a positive finite Z0 value",
                                name
                            ),
                        });
                    }
                    z0 = value;
                }
                continue;
            }
        }

        let lexeme = stream.peek().lexeme.clone();
        if !lexeme.is_empty() {
            source_tokens.push(lexeme);
        }
        stream.advance();
    }

    if source_tokens.is_empty() {
        elements.push(Element {
            name,
            kind: ElementKind::Resistor {
                value: z0,
                value_expr: None,
                model: None,
                instance_params: Vec::new(),
                deferred_params: Vec::new(),
            },
            nodes: vec![node_pos, node_neg],
        });
        return Ok(());
    }

    let source_text = source_tokens.join(" ");
    let source_spec = parse_source_spec_text(&source_text, line_num, params)?;
    let internal_node = format!("__RSPICE_{}_PORT", name.to_ascii_uppercase());
    let resistor_name = format!("__RSPICE_{}_Z0", name.to_ascii_uppercase());

    elements.push(Element {
        name,
        kind: ElementKind::VoltageSource(source_spec),
        nodes: vec![internal_node.clone(), node_neg],
    });
    elements.push(Element {
        name: resistor_name,
        kind: ElementKind::Resistor {
            value: z0,
            value_expr: None,
            model: None,
            instance_params: Vec::new(),
            deferred_params: Vec::new(),
        },
        nodes: vec![node_pos, internal_node],
    });

    Ok(())
}

fn is_xyce_port_assignment(raw: &str) -> bool {
    matches!(raw.to_ascii_uppercase().as_str(), "PORT" | "PORTNUM" | "Z0")
}

fn is_xyce_port_source_keyword(raw: &str) -> bool {
    matches!(
        raw.to_ascii_uppercase().as_str(),
        "DC" | "AC" | "PULSE" | "SIN" | "SINE" | "PWL" | "EXP" | "SFFM" | "AM" | "TRNOISE"
    )
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
    line_num: usize,
    params: &ParamContext,
    element_label: &str,
) -> Result<Vec<Value>, ParseError> {
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
                } else {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!(
                            "{} numeric tail expected value after '-', found {}",
                            element_label,
                            stream.peek().kind
                        ),
                    });
                }
            }
            TokenKind::Plus => {
                stream.advance();
                if !matches!(
                    stream.peek().kind,
                    TokenKind::Number(_) | TokenKind::Expression(_) | TokenKind::Ident(_)
                ) {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!(
                            "{} numeric tail expected value after '+', found {}",
                            element_label,
                            stream.peek().kind
                        ),
                    });
                }
            }
            _ => {
                if let Some(value) = try_value(stream, params) {
                    values.push(value);
                } else {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!(
                            "{} numeric tail contains non-numeric token '{}'",
                            element_label,
                            stream.peek().kind
                        ),
                    });
                }
            }
        }
    }
    Ok(values)
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

fn try_controlled_source_behavioral_assignment(
    stream: &mut TokenStream,
    line_num: usize,
    is_voltage_output: bool,
) -> Result<Option<String>, ParseError> {
    let TokenKind::Ident(raw) = &stream.peek().kind else {
        return Ok(None);
    };
    let (designator, inline_expr) = if let Some((lhs, rhs)) = raw.split_once('=') {
        (lhs.trim().to_ascii_uppercase(), rhs.trim().to_string())
    } else {
        if !matches!(stream.peek_n(1).kind, TokenKind::Equals) {
            return Ok(None);
        }
        (raw.trim().to_ascii_uppercase(), String::new())
    };

    let is_supported = match designator.as_str() {
        "VALUE" => true,
        "V" | "VOL" | "VOLTAGE" => is_voltage_output,
        "I" | "CUR" | "CURRENT" => !is_voltage_output,
        _ => false,
    };
    if !is_supported {
        return Ok(None);
    }

    stream.advance();
    let expression = if inline_expr.is_empty() {
        stream.consume(&TokenKind::Equals);
        collect_expression_argument(stream, line_num, None)?
    } else {
        let tail = collect_expression_argument(stream, line_num, None).unwrap_or_default();
        if tail.is_empty() {
            inline_expr
        } else {
            format!("{inline_expr} {tail}")
        }
    };
    Ok(Some(expression))
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
        prefix: &[(usize, usize)], // (var index offset into vars, exponent)
        out: &mut Vec<Vec<(usize, usize)>>,
        base_index: usize,
    ) {
        if remaining_vars.len() == 1 {
            let mut term = prefix.to_vec();
            if degree > 0 {
                term.push((base_index, degree));
            }
            out.push(term);
            return;
        }
        for first_exp in (0..=degree).rev() {
            let mut term_prefix = prefix.to_vec();
            if first_exp > 0 {
                term_prefix.push((base_index, first_exp));
            }
            push_monomials(
                &remaining_vars[1..],
                degree - first_exp,
                &term_prefix,
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
        let prefix = Vec::new();
        push_monomials(vars, degree, &prefix, &mut monomials, 0);
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
fn table_transfer_expression(input_expr: &str, pairs: &[(Value, Value)]) -> String {
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

fn unsupported_form_error(line_num: usize, element: &str, form: &str) -> ParseError {
    ParseError::Syntax {
        line: line_num,
        message: format!(
            "{} {} sources are not supported yet; supported extended forms are \
             POLY(n), VALUE={{expr}}, and TABLE {{expr}} = (x,y) pairs",
            element, form
        ),
    }
}

fn reject_unexpected_controlled_source_tail(
    stream: &mut TokenStream,
    line_num: usize,
    element: &str,
) -> Result<(), ParseError> {
    skip_commas(stream);
    if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        return Ok(());
    }

    Err(ParseError::Syntax {
        line: line_num,
        message: format!(
            "Unexpected trailing token in {element} source specification: {}",
            stream.peek().kind
        ),
    })
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

    let element_label = if is_voltage_output {
        "E (VCVS)"
    } else {
        "G (VCCS)"
    };

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

    if let Some(expression) =
        try_controlled_source_behavioral_assignment(stream, line_num, is_voltage_output)?
    {
        elements.push(Element {
            name,
            kind: lower_behavioral(expression),
            nodes: vec![node_pos, node_neg],
        });
        return Ok(());
    }

    match try_controlled_source_form(stream, line_num)? {
        Some(ControlledSourceForm::Poly(dims)) => {
            let mut vars = Vec::with_capacity(dims);
            for _ in 0..dims {
                let cp = expect_node(stream, line_num)?;
                let cn = expect_node(stream, line_num)?;
                vars.push(format!("V({},{})", cp, cn));
            }
            let coeffs = collect_numeric_tail(stream, line_num, params, element_label)?;
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
            let flat = collect_numeric_tail(stream, line_num, params, element_label)?;
            if flat.len() < 4 || !flat.len().is_multiple_of(2) {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!("{} TABLE requires at least two (x,y) pairs", element_label),
                });
            }
            let pairs: Vec<(Value, Value)> = flat.chunks_exact(2).map(|c| (c[0], c[1])).collect();
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
            reject_unexpected_controlled_source_tail(stream, line_num, element_label)?;
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

    let element_label = if is_voltage_output {
        "H (CCVS)"
    } else {
        "F (CCCS)"
    };

    match try_controlled_source_form(stream, line_num)? {
        Some(ControlledSourceForm::Poly(dims)) => {
            let mut vars = Vec::with_capacity(dims);
            for _ in 0..dims {
                let source = expect_ident(stream, line_num)?;
                vars.push(format!("I({})", source));
            }
            let coeffs = collect_numeric_tail(stream, line_num, params, element_label)?;
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
            reject_unexpected_controlled_source_tail(stream, line_num, element_label)?;
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
        TokenKind::Tilde => Some("~".to_string()),
        TokenKind::LBracket => Some("[".to_string()),
        TokenKind::RBracket => Some("]".to_string()),
        TokenKind::Other(c) => Some(c.to_string()),
        TokenKind::Newline | TokenKind::Eof => None,
    }
}

/// Parse subcircuit definition: .SUBCKT name ports [PARAMS: p1=v1 p2=v2] or .SUBCKT name ports p1=v1
pub(super) fn parse_subckt_def(
    line: &str,
    line_num: usize,
    params_ctx: &ParamContext,
) -> Result<SubcircuitDef, ParseError> {
    let fields = split_spice_fields(line);
    if fields.len() < 2 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: ".SUBCKT requires a subcircuit name".to_string(),
        });
    }

    let name = fields[1].clone();
    let mut ports = Vec::new();

    let mut idx = 2usize;
    while idx < fields.len() {
        let field = &fields[idx];
        if is_subckt_params_marker(field) {
            idx += 1;
            break;
        }
        if is_subckt_optional_marker(field) {
            idx += 1;
            skip_subckt_optional_defaults(&fields, &mut idx);
            continue;
        }
        if field.contains('=') || matches!(fields.get(idx + 1).map(String::as_str), Some("=")) {
            break;
        }
        ports.push(field.to_ascii_uppercase());
        idx += 1;
    }

    // Parse default parameters: NAME=VALUE pairs. Defaults may reference
    // earlier defaults in the same .SUBCKT declaration.
    let mut assignments = Vec::new();
    while idx < fields.len() {
        let field = &fields[idx];
        if is_subckt_params_marker(field) {
            idx += 1;
            continue;
        }
        if is_subckt_optional_marker(field) {
            idx += 1;
            skip_subckt_optional_defaults(&fields, &mut idx);
            continue;
        }

        let assignment = if let Some((param_name, raw_value)) = field.split_once('=') {
            idx += 1;
            Some((param_name.to_string(), raw_value.to_string()))
        } else if matches!(fields.get(idx + 1).map(String::as_str), Some("=")) {
            let Some(raw_value) = fields.get(idx + 2) else {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!("Expected value after subcircuit parameter '{}='", field),
                });
            };
            let assignment = Some((field.clone(), raw_value.clone()));
            idx += 3;
            assignment
        } else {
            idx += 1;
            None
        };

        if let Some((param_name, raw_value)) = assignment {
            if param_name.is_empty() {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: "Expected parameter name before '=' in .SUBCKT".to_string(),
                });
            }
            assignments.push((param_name, raw_value));
            continue;
        }
    }

    let (params, string_params) = resolve_subckt_default_params(assignments, params_ctx, line_num)?;

    Ok(SubcircuitDef {
        name,
        ports,
        elements: Vec::new(),
        initial_conditions: Vec::new(),
        node_sets: Vec::new(),
        params,
        string_params,
        body_params: Vec::new(),
        body_string_params: Vec::new(),
        body_functions: Vec::new(),
        local_options: std::collections::HashMap::new(),
        library_ref: None,
        nested_subcircuits: Vec::new(),
    })
}

fn is_subckt_params_marker(field: &str) -> bool {
    field.eq_ignore_ascii_case("PARAMS") || field.eq_ignore_ascii_case("PARAMS:")
}

fn is_subckt_optional_marker(field: &str) -> bool {
    field.eq_ignore_ascii_case("OPTIONAL") || field.eq_ignore_ascii_case("OPTIONAL:")
}

fn skip_subckt_optional_defaults(fields: &[String], idx: &mut usize) {
    while *idx < fields.len() {
        let field = &fields[*idx];
        if is_subckt_params_marker(field) {
            break;
        }

        if matches!(fields.get(*idx + 1).map(String::as_str), Some("=")) {
            *idx += 3;
        } else {
            *idx += 1;
        }
    }
}

fn resolve_subckt_default_params(
    assignments: Vec<(String, String)>,
    params_ctx: &ParamContext,
    line_num: usize,
) -> Result<(Vec<(String, Value)>, Vec<(String, String)>), ParseError> {
    let mut eval_ctx = params_ctx.clone();
    let mut params = Vec::new();
    let mut string_params = Vec::new();
    let mut pending = assignments;

    while !pending.is_empty() {
        let mut progress = false;
        let mut unresolved = Vec::new();
        let mut first_error = None;

        for (param_name, raw_value) in pending {
            if let Some(value) = parse_string_field_value(&raw_value, &eval_ctx) {
                eval_ctx.set_string(&param_name, value.clone());
                string_params.push((param_name, value));
                progress = true;
                continue;
            }

            match parse_numeric_field_value(&raw_value, &eval_ctx, line_num) {
                Ok(value) => {
                    eval_ctx.set(&param_name, value);
                    params.push((param_name, value));
                    progress = true;
                }
                Err(err) => {
                    first_error.get_or_insert(err);
                    unresolved.push((param_name, raw_value));
                }
            }
        }

        if !progress {
            return Err(first_error.unwrap_or_else(|| ParseError::Syntax {
                line: line_num,
                message: "subcircuit default parameters could not be resolved".to_string(),
            }));
        }
        pending = unresolved;
    }

    Ok((params, string_params))
}

fn parse_string_field_value(raw_value: &str, params_ctx: &ParamContext) -> Option<String> {
    if let Some(value) = strip_wrapping_double_quoted_string_literal(raw_value) {
        return Some(value.to_string());
    }
    let expr = strip_wrapping_expression_delimiters(raw_value);
    params_ctx.get_string(expr).map(ToString::to_string)
}
