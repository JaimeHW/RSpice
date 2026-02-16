//! Element parsing functions for SPICE netlists
//!
//! This module contains parsing functions for circuit elements:
//! - Passive: Resistors, Capacitors, Inductors
//! - Sources: Voltage and Current sources
//! - Semiconductors: Diodes, BJTs, MOSFETs
//! - Controlled sources: VCVS, CCCS, VCCS, CCVS
//! - Advanced: Behavioral, Coupling, Switches, Transmission Lines

use super::helpers::{
    expect_ident, expect_node, expect_value, skip_commas, skip_optional_param_name, try_value,
    try_value_with_param,
};
use super::lexer::{TokenKind, TokenStream};
use super::sources::parse_source_spec;
use super::{BjtType, Element, ElementKind, MosType, ParamContext, ParseError, SwitchState};

//=============================================================================
// Basic Element Parsing
//=============================================================================

/// Parse resistor: R1 n+ n- value
pub fn parse_resistor(
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

    let value = expect_value(stream, line_num, params)?;

    elements.push(Element {
        name,
        kind: ElementKind::Resistor { value },
        nodes: vec![node_pos, node_neg],
    });

    Ok(())
}

/// Parse capacitor: C1 n+ n- value [IC=voltage]
pub fn parse_capacitor(
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

/// Parse inductor: L1 n+ n- value [IC=current]
pub fn parse_inductor(
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

    elements.push(Element {
        name,
        kind: ElementKind::Inductor {
            value,
            initial_current,
        },
        nodes: vec![node_pos, node_neg],
    });

    Ok(())
}

//=============================================================================
// Source Parsing
//=============================================================================

/// Parse voltage source: V1 n+ n- [DC|AC|PULSE|SIN|PWL|EXP] spec
pub fn parse_voltage_source(
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

/// Parse current source: I1 n+ n- [DC|AC|PULSE|SIN|PWL|EXP] spec
pub fn parse_current_source(
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

//=============================================================================
// Semiconductor Device Parsing
//=============================================================================

/// Parse diode: D1 anode cathode MODEL
pub fn parse_diode(
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

/// Parse BJT: Q1 collector base emitter MODEL
pub fn parse_bjt(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let collector = expect_node(stream, line_num)?;
    let base = expect_node(stream, line_num)?;
    let emitter = expect_node(stream, line_num)?;

    // Optional substrate node can appear before model: Q C B E [S] model.
    // "OFF" is an instance keyword and must not be interpreted as model.
    let first = expect_node(stream, line_num)?;
    let first_is_numeric = first.parse::<i64>().is_ok();
    let (maybe_substrate, model) = if first_is_numeric
        || (matches!(&stream.peek().kind, TokenKind::Ident(next))
            && !matches!(stream.peek_n(1).kind, TokenKind::Equals)
            && !next.eq_ignore_ascii_case("OFF"))
    {
        let model = expect_ident(stream, line_num)?;
        (Some(first), model)
    } else {
        (None, first)
    };

    let mut nodes = vec![collector, base, emitter];
    if let Some(sub) = maybe_substrate {
        nodes.push(sub);
    }

    elements.push(Element {
        name,
        kind: ElementKind::Bjt {
            model,
            bjt_type: BjtType::Npn, // Will be set from model
        },
        nodes,
    });

    Ok(())
}

/// Parse MOSFET: M1 drain gate source bulk MODEL [W=w L=l]
pub fn parse_mosfet(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
) -> Result<(), ParseError> {
    let name = expect_ident(stream, line_num)?;
    let drain = expect_node(stream, line_num)?;
    let gate = expect_node(stream, line_num)?;
    let source = expect_node(stream, line_num)?;
    let bulk = expect_node(stream, line_num)?;

    let first_after_bulk = expect_node(stream, line_num)?;
    let (extra_node, model) = if matches!(&stream.peek().kind, TokenKind::Ident(_))
        && !matches!(stream.peek_n(1).kind, TokenKind::Equals)
    {
        let model = expect_ident(stream, line_num)?;
        (Some(first_after_bulk), model)
    } else {
        (None, first_after_bulk)
    };

    let mut nodes = vec![drain, gate, source, bulk];
    if let Some(extra) = extra_node {
        nodes.push(extra);
    }

    elements.push(Element {
        name,
        kind: ElementKind::Mosfet {
            model,
            mos_type: MosType::Nmos, // Will be set from model
            instance_params: Vec::new(),
        },
        nodes,
    });

    Ok(())
}

//=============================================================================
// Controlled Source Parsing
//=============================================================================

/// Parse VCVS: E1 n+ n- nc+ nc- gain
pub fn parse_vcvs(
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

/// Parse CCCS: F1 n+ n- Vcontrol gain
pub fn parse_cccs(
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

/// Parse VCCS: G1 n+ n- nc+ nc- transconductance
pub fn parse_vccs(
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

/// Parse CCVS: H1 n+ n- Vcontrol transresistance
pub fn parse_ccvs(
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

//=============================================================================
// Behavioral Source Parsing
//=============================================================================

/// Parse behavioral source: B1 n+ n- V=expr or I=expr
pub fn parse_behavioral(
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

    // Collect expression text with token-aware reconstruction.
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
// Subcircuit Instance Parsing
//=============================================================================

/// Parse subcircuit instance: X1 node1 node2... SUBCKTNAME [PARAM=val ...]
pub fn parse_subcircuit_instance(
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

//=============================================================================
// Advanced Element Parsing
//=============================================================================

/// Parse coupling coefficient: K1 L1 L2 [L3...] coefficient
pub fn parse_coupling(
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
pub fn parse_vswitch(
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
pub fn parse_iswitch(
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
pub fn parse_transmission_line(
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
    let mut z0: Option<crate::Value> = None;
    let mut td: Option<crate::Value> = None;
    let mut freq: Option<crate::Value> = None;
    let mut nl: Option<crate::Value> = None;

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
