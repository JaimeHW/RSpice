//! XSPICE A-device element parsing
//!
//! Parses XSPICE code model instances with the following syntax:
//!
//! ```text
//! A<name> <port1> <port2> ... <model_name> [param=value ...]
//! ```
//!
//! Port types are distinguished by syntax:
//! - `node` - Analog node (voltage/current port)
//! - `[node]` - Digital node (12-state logic)
//! - `[n1 n2 n3]` - Digital vector (multiple digital nodes)
//! - `%v node` - Explicit single-ended voltage input/output
//! - `%v([n1 n2])` - Compact vector of single-ended voltage ports
//! - `%i vsrc` - Named voltage-source branch-current input
//! - `%vd[n+ n-]` - Differential voltage input/output
//! - `%vd([n1+ n1- n2+ n2-])` - Compact vector of differential voltage ports
//! - `%id[n+ n-]` - Differential current input/output
//! - `%vnam vsrc` - Named voltage-source branch-current input
//! - `%g[node]` or `%g node` - Single-ended conductance terminal
//! - `%gd[n+ n-]` or `%gd n+ n-` - Differential conductance terminal pair
//! - `%h[node]` or `%h node` - Single-ended hybrid terminal
//! - `%hd[n+ n-]` or `%hd n+ n-` - Differential hybrid terminal pair
//! - `null` - Unconnected port
//!
//! # Examples
//!
//! ```text
//! * Gain block (analog)
//! A1 in out gain gain=2.0
//!
//! * AND gate (digital)
//! A2 [a] [b] [y] d_and rise_delay=10n fall_delay=10n
//!
//! * ADC bridge (mixed-signal)  
//! A3 analog_in [digital_out] adc_bridge in_low=0.5 in_high=2.5
//! ```

use super::lexer::{Token, TokenKind, TokenStream, parse_spice_value, tokenize};
use super::{Element, ElementKind, ParamContext, ParseError, XspiceDigitalNode, XspicePort, expr};
use crate::Value;

//=============================================================================
// Main Parser Entry Point
//=============================================================================

/// Parse an XSPICE A-device element
///
/// Called when the parser encounters a line starting with 'A'.
/// The stream should be positioned AFTER the element name has been consumed.
pub fn parse_xspice(
    stream: &mut TokenStream,
    line_num: usize,
    name: String,
    elements: &mut Vec<Element>,
    netlist_params: &ParamContext,
    defer_simple_param_refs: bool,
) -> Result<(), ParseError> {
    // Collect all ports and potential model name in order
    // We use a strategy where all identifiers are added as ports,
    // and at the end we take the last analog port as the model name
    let mut ports = Vec::new();
    let mut params = Vec::new();
    let mut expr_params = Vec::new();
    let mut string_params = Vec::new();
    let mut string_expr_params = Vec::new();
    let mut string_vector_params = Vec::new();
    let mut string_vector_expr_params = Vec::new();
    let mut real_vector_params = Vec::new();
    let mut real_vector_expr_params = Vec::new();

    loop {
        skip_xspice_mif_token_separators(stream);

        if ports
            .iter()
            .any(|port| matches!(port, XspicePort::Analog(_)))
            && consume_xspice_params_marker(stream)
        {
            continue;
        }

        if is_xspice_null_token(stream.peek()) {
            stream.advance();
            ports.push(XspicePort::Null);
            continue;
        }

        match &stream.peek().kind {
            // End of line
            TokenKind::Newline | TokenKind::Eof => break,

            // Bracketed connection: `[node]`, `[n1 n2 n3]`, or ngspice
            // typed vectors such as `[%id(n1 p1) %id(n2 p2)]`.
            TokenKind::LBracket => {
                let bracket_ports = parse_bracketed_ports(stream, line_num)?;
                ports.extend(bracket_ports);
            }

            // Potential differential port or analog node
            TokenKind::Ident(id) => {
                let id_str = id.clone();

                // Check if next token is '=' (this is a parameter)
                if matches!(stream.peek_n(1).kind, TokenKind::Equals) {
                    // This is a parameter assignment
                    stream.advance(); // consume identifier
                    stream.advance(); // consume '='
                    match parse_param_value(
                        stream,
                        line_num,
                        &id_str,
                        netlist_params,
                        defer_simple_param_refs,
                    )? {
                        XspiceParamValue::Resolved(value) => params.push((id_str, value)),
                        XspiceParamValue::Deferred(expr) => expr_params.push((id_str, expr)),
                        XspiceParamValue::String(value) => string_params.push((id_str, value)),
                        XspiceParamValue::StringDeferred(expr) => {
                            string_expr_params.push((id_str, expr))
                        }
                        XspiceParamValue::StringVector(values) => {
                            string_vector_params.push((id_str, values))
                        }
                        XspiceParamValue::StringVectorDeferred(expr) => {
                            string_vector_expr_params.push((id_str, expr))
                        }
                        XspiceParamValue::RealVector(values) => {
                            real_vector_params.push((id_str, values))
                        }
                        XspiceParamValue::RealVectorDeferred(exprs) => {
                            real_vector_expr_params.push((id_str, exprs))
                        }
                    }
                } else if id_str.starts_with('%') {
                    // Typed analog port: %v node, %i vsrc, %vd[...], %id[...],
                    // %vnam name, %g node, %gd n+ n-, %h node, or %hd n+ n-.
                    let typed_ports = parse_typed_ports(stream, line_num)?;
                    ports.extend(typed_ports);
                } else {
                    // This is an analog node - add it as a port for now
                    // The last analog identifier will be extracted as model name
                    let node_name = parse_node_name(stream, line_num, "XSPICE port")?;
                    ports.push(XspicePort::Analog(node_name));
                }
            }

            // Number - could be a node name like "0"
            TokenKind::Number(_)
            | TokenKind::StringLit(_)
            | TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::AtSign
            | TokenKind::Other(_) => {
                let node_name = parse_node_name(stream, line_num, "XSPICE port")?;
                ports.push(XspicePort::Analog(node_name));
            }

            TokenKind::Tilde => {
                stream.advance();
                if is_xspice_null_token(stream.peek()) {
                    stream.advance();
                    ports.push(XspicePort::Null);
                } else {
                    let node_name =
                        parse_node_name(stream, line_num, "inverted XSPICE digital port")?;
                    ports.push(XspicePort::DigitalInverted(node_name));
                }
            }

            other => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "Unsupported XSPICE instance token '{}'; expected port, model name, or NAME=value",
                        other
                    ),
                });
            }
        }
    }

    // The last analog port should be the model name
    // Find and remove the last analog port to use as model
    let model = if let Some(pos) = ports
        .iter()
        .rposition(|p| matches!(p, XspicePort::Analog(_)))
    {
        if let XspicePort::Analog(name) = ports.remove(pos) {
            name
        } else {
            unreachable!()
        }
    } else {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("XSPICE element {} missing model name", name),
        });
    };

    // Validate we have at least one port
    if ports.is_empty() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("XSPICE element {} has no ports", name),
        });
    }

    // Create the element
    elements.push(Element {
        name,
        kind: ElementKind::Xspice {
            model,
            ports,
            params,
            expr_params,
            string_params,
            string_expr_params,
            string_vector_params,
            string_vector_expr_params,
            real_vector_params,
            real_vector_expr_params,
        },
        nodes: Vec::new(), // XSPICE uses ports instead of simple nodes
    });

    Ok(())
}

//=============================================================================
// Port Parsing Helpers
//=============================================================================

/// Parse a bracketed connection. Plain bracketed names preserve the existing
/// digital-vector AST shape, while a `%` type marker switches the bracket into
/// ngspice's vector-of-typed-ports mode.
fn parse_bracketed_ports(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<Vec<XspicePort>, ParseError> {
    // Consume opening bracket
    if !matches!(stream.peek().kind, TokenKind::LBracket) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "Expected '[' for digital port".to_string(),
        });
    }
    stream.advance();

    let mut nodes = Vec::new();
    let mut typed_ports: Option<Vec<XspicePort>> = None;

    // Collect nodes until closing bracket
    loop {
        skip_xspice_mif_token_separators(stream);

        match &stream.peek().kind {
            TokenKind::RBracket => {
                stream.advance(); // consume ']'
                break;
            }
            _ if is_xspice_null_token(stream.peek()) => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: "NULL connection found where not allowed in XSPICE array".to_string(),
                });
            }
            TokenKind::Ident(id) => {
                if id.starts_with('%') {
                    let ports = typed_ports
                        .get_or_insert_with(|| nodes.drain(..).map(digital_node_to_port).collect());
                    ports.extend(parse_typed_ports(stream, line_num)?);
                } else {
                    let node = XspiceDigitalNode::new(
                        parse_node_name(stream, line_num, "XSPICE digital port")?,
                        false,
                    );
                    if let Some(ports) = typed_ports.as_mut() {
                        ports.push(digital_node_to_port(node));
                    } else {
                        nodes.push(node);
                    }
                }
            }
            TokenKind::Number(_)
            | TokenKind::StringLit(_)
            | TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::AtSign
            | TokenKind::Other(_) => {
                // Allow numeric node names
                let node = XspiceDigitalNode::new(
                    parse_node_name(stream, line_num, "XSPICE digital port")?,
                    false,
                );
                if let Some(ports) = typed_ports.as_mut() {
                    ports.push(digital_node_to_port(node));
                } else {
                    nodes.push(node);
                }
            }
            TokenKind::Tilde => {
                stream.advance();
                let node = XspiceDigitalNode::new(
                    parse_node_name(stream, line_num, "inverted XSPICE digital port")?,
                    true,
                );
                if let Some(ports) = typed_ports.as_mut() {
                    ports.push(digital_node_to_port(node));
                } else {
                    nodes.push(node);
                }
            }
            TokenKind::Newline | TokenKind::Eof => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: "Unclosed bracket in digital port".to_string(),
                });
            }
            other => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "Unsupported XSPICE digital port token '{}'; expected node name or ']'",
                        other
                    ),
                });
            }
        }
    }

    if let Some(ports) = typed_ports {
        return if ports.is_empty() {
            Ok(vec![XspicePort::Null])
        } else {
            Ok(ports)
        };
    }

    Ok(vec![digital_nodes_to_port(nodes)])
}

fn digital_node_to_port(node: XspiceDigitalNode) -> XspicePort {
    if node.inverted {
        XspicePort::DigitalInverted(node.name)
    } else {
        XspicePort::Digital(node.name)
    }
}

fn digital_nodes_to_port(nodes: Vec<XspiceDigitalNode>) -> XspicePort {
    // Return appropriate port type
    match nodes.len() {
        0 => XspicePort::Null, // [] = null
        1 => {
            let node = nodes.into_iter().next().unwrap();
            if node.inverted {
                XspicePort::DigitalInverted(node.name)
            } else {
                XspicePort::Digital(node.name)
            }
        }
        _ if nodes.iter().any(|node| node.inverted) => XspicePort::DigitalVectorMixed(nodes),
        _ => XspicePort::DigitalVector(nodes.into_iter().map(|node| node.name).collect()),
    }
}

/// Parse one explicitly typed analog port, or ngspice's compact typed vector:
/// `%v node`, `%i vsrc`, `%g node`, `%h node`, `%vnam vsrc`,
/// `%v([n1 n2])`, `%vd([n1+ n1- n2+ n2-])`,
/// `%id(n+ n-)`, `%gd n+ n-`, or `%hd n+ n-`.
fn parse_typed_ports(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<Vec<XspicePort>, ParseError> {
    let prefix = parse_typed_port_prefix(stream, line_num)?;

    let prefix_lower = prefix.to_lowercase();
    let is_single_voltage = prefix_lower == "%v";
    let is_single_current = prefix_lower == "%i";
    let is_single_conductance = prefix_lower == "%g";
    let is_single_hybrid = prefix_lower == "%h";
    let is_voltage_name = prefix_lower == "%vnam";
    let is_voltage = prefix_lower == "%vd";
    let is_current = prefix_lower == "%id";
    let is_conductance = prefix_lower == "%gd";
    let is_hybrid = prefix_lower == "%hd";

    if !is_single_voltage
        && !is_single_current
        && !is_single_conductance
        && !is_single_hybrid
        && !is_voltage_name
        && !is_voltage
        && !is_current
        && !is_conductance
        && !is_hybrid
    {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("Unknown differential port type: {}", prefix),
        });
    }

    if let Some(nodes) = parse_compact_typed_vector(stream, line_num, &prefix)? {
        return nodes_to_typed_ports(
            nodes,
            line_num,
            &prefix,
            TypedPortKind {
                is_single_voltage,
                is_single_current,
                is_single_conductance,
                is_single_hybrid,
                is_voltage_name,
                is_voltage,
                is_current,
                is_conductance,
            },
        );
    }

    if consume_xspice_null_token_after_mif_separators(stream) {
        return Ok(vec![XspicePort::Null]);
    }

    if is_single_voltage {
        return parse_single_typed_node(stream, line_num, &prefix)
            .map(XspicePort::Analog)
            .map(|port| vec![port]);
    }
    if is_single_current {
        return parse_single_typed_node(stream, line_num, &prefix)
            .map(XspicePort::Current)
            .map(|port| vec![port]);
    }
    if is_single_conductance {
        return parse_single_typed_node(stream, line_num, &prefix)
            .map(XspicePort::Conductance)
            .map(|port| vec![port]);
    }
    if is_single_hybrid {
        return parse_single_typed_node(stream, line_num, &prefix)
            .map(XspicePort::Hybrid)
            .map(|port| vec![port]);
    }
    if is_voltage_name {
        return parse_single_typed_node(stream, line_num, &prefix)
            .map(XspicePort::VoltageName)
            .map(|port| vec![port]);
    }

    // Parse nodes in brackets or parentheses
    let mut nodes = Vec::new();

    let delimiter = match &stream.peek().kind {
        TokenKind::LBracket => {
            stream.advance();
            Some((TokenKind::RBracket, "]"))
        }
        TokenKind::LParen => {
            stream.advance();
            Some((TokenKind::RParen, ")"))
        }
        _ => None,
    };

    if let Some((closing, closing_name)) = delimiter {
        loop {
            skip_xspice_loose_separators(stream);

            if stream.check(&closing) {
                stream.advance();
                break;
            }

            match &stream.peek().kind {
                _ if token_can_start_node_name(stream) => {
                    nodes.push(parse_node_name(
                        stream,
                        line_num,
                        "XSPICE differential port",
                    )?);
                }
                TokenKind::Newline | TokenKind::Eof => {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!(
                            "Unclosed differential port {}; expected '{}'",
                            prefix, closing_name
                        ),
                    });
                }
                other => {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!(
                            "Unsupported XSPICE differential port token '{}'; expected node name or '{}'",
                            other, closing_name
                        ),
                    });
                }
            }
        }
    } else {
        for _ in 0..2 {
            skip_xspice_loose_separators(stream);

            match &stream.peek().kind {
                _ if token_can_start_node_name(stream) => {
                    nodes.push(parse_node_name(
                        stream,
                        line_num,
                        "XSPICE differential port",
                    )?);
                }
                other => {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!(
                            "Differential port {} requires two node names, found '{}'",
                            prefix, other
                        ),
                    });
                }
            }
        }
    }

    if nodes.len() != 2 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Differential port {} requires exactly 2 nodes, got {}",
                prefix,
                nodes.len()
            ),
        });
    }

    let pos = nodes[0].clone();
    let neg = nodes[1].clone();

    if is_voltage {
        Ok(vec![XspicePort::DifferentialVoltage { pos, neg }])
    } else if is_current {
        Ok(vec![XspicePort::DifferentialCurrent { pos, neg }])
    } else if is_conductance {
        Ok(vec![XspicePort::DifferentialConductance { pos, neg }])
    } else {
        Ok(vec![XspicePort::DifferentialHybrid { pos, neg }])
    }
}

fn parse_typed_port_prefix(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<String, ParseError> {
    match &stream.peek().kind {
        TokenKind::Ident(id) if id == "%" => {
            stream.advance();
            match &stream.peek().kind {
                TokenKind::Ident(suffix) | TokenKind::StringLit(suffix) => {
                    let prefix = format!("%{suffix}");
                    stream.advance();
                    Ok(prefix)
                }
                other => Err(ParseError::Syntax {
                    line: line_num,
                    message: format!("Invalid port type specifier '{}'", other),
                }),
            }
        }
        TokenKind::Ident(id) if id.starts_with('%') => {
            let prefix = id.clone();
            stream.advance();
            Ok(prefix)
        }
        other => Err(ParseError::Syntax {
            line: line_num,
            message: format!("Expected XSPICE typed port prefix, found '{}'", other),
        }),
    }
}

#[derive(Clone, Copy)]
struct TypedPortKind {
    is_single_voltage: bool,
    is_single_current: bool,
    is_single_conductance: bool,
    is_single_hybrid: bool,
    is_voltage_name: bool,
    is_voltage: bool,
    is_current: bool,
    is_conductance: bool,
}

fn parse_compact_typed_vector(
    stream: &mut TokenStream,
    line_num: usize,
    prefix: &str,
) -> Result<Option<Vec<String>>, ParseError> {
    if !matches!(stream.peek().kind, TokenKind::LParen)
        || !matches!(stream.peek_n(1).kind, TokenKind::LBracket)
    {
        return Ok(None);
    }

    stream.advance(); // '('
    stream.advance(); // '['

    let mut nodes = Vec::new();
    loop {
        skip_xspice_loose_separators(stream);

        match &stream.peek().kind {
            TokenKind::RBracket => {
                stream.advance();
                break;
            }
            _ if is_xspice_null_token(stream.peek()) => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message:
                        "NULL connection found where not allowed in compact XSPICE port vector"
                            .to_string(),
                });
            }
            _ if token_can_start_node_name(stream) => {
                nodes.push(parse_node_name(
                    stream,
                    line_num,
                    "compact typed XSPICE port vector",
                )?);
            }
            TokenKind::Newline | TokenKind::Eof => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!("Unclosed compact typed XSPICE port vector {}", prefix),
                });
            }
            other => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "Unsupported compact typed XSPICE port token '{}'; expected node name or ']'",
                        other
                    ),
                });
            }
        }
    }

    if nodes.is_empty() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Compact typed XSPICE port vector {} cannot be empty",
                prefix
            ),
        });
    }

    if stream.consume(&TokenKind::RParen) {
        Ok(Some(nodes))
    } else {
        Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Compact typed XSPICE port vector {} requires closing ')'",
                prefix
            ),
        })
    }
}

fn nodes_to_typed_ports(
    nodes: Vec<String>,
    line_num: usize,
    prefix: &str,
    kind: TypedPortKind,
) -> Result<Vec<XspicePort>, ParseError> {
    if kind.is_single_voltage {
        return Ok(nodes.into_iter().map(XspicePort::Analog).collect());
    }
    if kind.is_single_current {
        return Ok(nodes.into_iter().map(XspicePort::Current).collect());
    }
    if kind.is_single_conductance {
        return Ok(nodes.into_iter().map(XspicePort::Conductance).collect());
    }
    if kind.is_single_hybrid {
        return Ok(nodes.into_iter().map(XspicePort::Hybrid).collect());
    }
    if kind.is_voltage_name {
        return Ok(nodes.into_iter().map(XspicePort::VoltageName).collect());
    }

    if nodes.len() % 2 != 0 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Compact differential XSPICE port vector {} requires node pairs, got {} nodes",
                prefix,
                nodes.len()
            ),
        });
    }

    let mut ports = Vec::with_capacity(nodes.len() / 2);
    for pair in nodes.chunks_exact(2) {
        let pos = pair[0].clone();
        let neg = pair[1].clone();
        if kind.is_voltage {
            ports.push(XspicePort::DifferentialVoltage { pos, neg });
        } else if kind.is_current {
            ports.push(XspicePort::DifferentialCurrent { pos, neg });
        } else if kind.is_conductance {
            ports.push(XspicePort::DifferentialConductance { pos, neg });
        } else {
            ports.push(XspicePort::DifferentialHybrid { pos, neg });
        }
    }
    Ok(ports)
}

fn parse_single_typed_node(
    stream: &mut TokenStream,
    line_num: usize,
    prefix: &str,
) -> Result<String, ParseError> {
    skip_xspice_loose_separators(stream);

    let delimiter = match &stream.peek().kind {
        TokenKind::LBracket => {
            stream.advance();
            Some((TokenKind::RBracket, "]"))
        }
        TokenKind::LParen => {
            stream.advance();
            Some((TokenKind::RParen, ")"))
        }
        _ => None,
    };

    let node = match &stream.peek().kind {
        _ if token_can_start_node_name(stream) => {
            parse_node_name(stream, line_num, "typed XSPICE port")?
        }
        other => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    "Typed XSPICE port {} requires a node name, found '{}'",
                    prefix, other
                ),
            });
        }
    };

    if let Some((closing, closing_name)) = delimiter {
        if stream.check(&closing) {
            stream.advance();
        } else {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    "Typed XSPICE port {} requires closing '{}'",
                    prefix, closing_name
                ),
            });
        }
    }

    Ok(node)
}

fn skip_xspice_loose_separators(stream: &mut TokenStream) {
    while matches!(stream.peek().kind, TokenKind::Comma | TokenKind::Equals) {
        stream.advance();
    }
}

fn skip_xspice_mif_token_separators(stream: &mut TokenStream) {
    while matches!(
        stream.peek().kind,
        TokenKind::Comma | TokenKind::Equals | TokenKind::LParen | TokenKind::RParen
    ) {
        stream.advance();
    }
}

fn consume_xspice_params_marker(stream: &mut TokenStream) -> bool {
    let mut probe = stream.clone();

    let TokenKind::Ident(marker) = &probe.peek().kind else {
        return false;
    };
    let marker_without_colon = marker.strip_suffix(':').unwrap_or(marker);
    if !marker_without_colon.eq_ignore_ascii_case("params") {
        return false;
    }

    let marker_had_colon = marker.ends_with(':');
    probe.advance();
    if !marker_had_colon && matches!(probe.peek().kind, TokenKind::Other(':')) {
        probe.advance();
    }

    if matches!(probe.peek().kind, TokenKind::Ident(_))
        && matches!(probe.peek_n(1).kind, TokenKind::Equals)
    {
        *stream = probe;
        true
    } else {
        false
    }
}

fn consume_xspice_null_token_after_mif_separators(stream: &mut TokenStream) -> bool {
    let mut probe = stream.clone();
    skip_xspice_mif_token_separators(&mut probe);

    if is_xspice_null_token(probe.peek()) {
        probe.advance();
        *stream = probe;
        true
    } else {
        false
    }
}

fn is_xspice_null_token(token: &Token) -> bool {
    match &token.kind {
        TokenKind::Ident(value) | TokenKind::StringLit(value) => value.eq_ignore_ascii_case("null"),
        _ => false,
    }
}

fn token_can_start_node_name(stream: &TokenStream) -> bool {
    let first = stream.peek();
    if node_name_piece_from_token(first).is_none() {
        return false;
    }

    if matches!(
        first.kind,
        TokenKind::Ident(_) | TokenKind::StringLit(_) | TokenKind::Number(_) | TokenKind::Other(_)
    ) {
        return true;
    }

    let next = stream.peek_n(1);
    node_name_piece_from_token(next).is_some() && next.span.start == first.span.end
}

fn parse_node_name(
    stream: &mut TokenStream,
    line_num: usize,
    context: &str,
) -> Result<String, ParseError> {
    if !token_can_start_node_name(stream) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "{} requires a node name, found '{}'",
                context,
                stream.peek().kind
            ),
        });
    }

    let mut node = String::new();
    let mut end = None;

    loop {
        let token = stream.peek().clone();
        let quoted_token = matches!(token.kind, TokenKind::StringLit(_));
        let Some(piece) = node_name_piece_from_token(&token) else {
            break;
        };

        if let Some(previous_end) = end
            && token.span.start != previous_end
        {
            break;
        }

        node.push_str(&piece);
        end = Some(token.span.end);
        stream.advance();

        if quoted_token {
            break;
        }
    }

    Ok(node)
}

fn node_name_piece_from_token(token: &Token) -> Option<String> {
    match &token.kind {
        TokenKind::Ident(id) => Some(id.clone()),
        TokenKind::StringLit(value) => Some(value.to_uppercase()),
        TokenKind::Number(value) => Some(if token.lexeme.is_empty() {
            value.to_string()
        } else {
            token.lexeme.clone()
        }),
        TokenKind::Other('<' | '>') => None,
        TokenKind::Plus
        | TokenKind::Minus
        | TokenKind::Star
        | TokenKind::Slash
        | TokenKind::AtSign
        | TokenKind::Other(_) => Some(if token.lexeme.is_empty() {
            token.kind.to_string()
        } else {
            token.lexeme.clone()
        }),
        _ => None,
    }
}

enum XspiceParamValue {
    Resolved(Value),
    Deferred(String),
    String(String),
    StringDeferred(String),
    StringVector(Vec<String>),
    StringVectorDeferred(String),
    RealVector(Vec<Value>),
    RealVectorDeferred(Vec<String>),
}

/// Parse a scalar XSPICE instance parameter value.
fn parse_param_value(
    stream: &mut TokenStream,
    line_num: usize,
    param_name: &str,
    netlist_params: &ParamContext,
    defer_simple_param_refs: bool,
) -> Result<XspiceParamValue, ParseError> {
    let sign = match &stream.peek().kind {
        TokenKind::Plus => Some(1.0),
        TokenKind::Minus => Some(-1.0),
        _ => None,
    };
    if sign.is_some()
        && xspice_param_prefers_bare_string(param_name)
        && !param_name.eq_ignore_ascii_case("model")
    {
        return parse_bare_string_param_value(stream, line_num, param_name);
    }
    if let Some(sign) = sign {
        stream.advance();
        return parse_unsigned_param_value(
            stream,
            line_num,
            param_name,
            netlist_params,
            defer_simple_param_refs,
        )
        .map(|value| match value {
            XspiceParamValue::Resolved(value) => XspiceParamValue::Resolved(sign * value),
            XspiceParamValue::Deferred(expr) => {
                if sign < 0.0 {
                    XspiceParamValue::Deferred(format!("-({expr})"))
                } else {
                    XspiceParamValue::Deferred(expr)
                }
            }
            string_value @ (XspiceParamValue::String(_) | XspiceParamValue::StringDeferred(_)) => {
                string_value
            }
            vector_value @ (XspiceParamValue::StringVector(_)
            | XspiceParamValue::StringVectorDeferred(_)
            | XspiceParamValue::RealVector(_)
            | XspiceParamValue::RealVectorDeferred(_)) => vector_value,
        });
    }

    parse_unsigned_param_value(
        stream,
        line_num,
        param_name,
        netlist_params,
        defer_simple_param_refs,
    )
}

fn parse_unsigned_param_value(
    stream: &mut TokenStream,
    line_num: usize,
    param_name: &str,
    netlist_params: &ParamContext,
    defer_simple_param_refs: bool,
) -> Result<XspiceParamValue, ParseError> {
    match &stream.peek().kind {
        TokenKind::Number(_)
            if xspice_param_prefers_bare_string(param_name)
                && !param_name.eq_ignore_ascii_case("model") =>
        {
            parse_bare_string_param_value(stream, line_num, param_name)
        }
        TokenKind::Number(n) => {
            let v = *n;
            stream.advance();
            Ok(XspiceParamValue::Resolved(v))
        }
        TokenKind::Expression(expr_text) => {
            let expr_text = expr_text.clone();
            stream.advance();
            if defer_simple_param_refs {
                if xspice_param_prefers_string_vector(param_name) {
                    Ok(XspiceParamValue::StringVectorDeferred(expr_text))
                } else if xspice_param_prefers_string(param_name) {
                    Ok(XspiceParamValue::StringDeferred(expr_text))
                } else {
                    Ok(XspiceParamValue::Deferred(expr_text))
                }
            } else {
                match expr::eval_expression(&expr_text, netlist_params) {
                    Ok(value) => Ok(XspiceParamValue::Resolved(value)),
                    Err(_) => {
                        if let Some(value) = netlist_params.get_string(&expr_text) {
                            let parsed =
                                parse_string_backed_param_value(param_name, value, line_num)?
                                    .unwrap_or_else(|| {
                                        xspice_string_value_from_param_preference(
                                            param_name,
                                            value.to_string(),
                                        )
                                    });
                            Ok(parsed)
                        } else if xspice_param_prefers_string_vector(param_name) {
                            Ok(XspiceParamValue::StringVectorDeferred(expr_text))
                        } else if xspice_param_prefers_string(param_name) {
                            Ok(XspiceParamValue::StringDeferred(expr_text))
                        } else {
                            Ok(XspiceParamValue::Deferred(expr_text))
                        }
                    }
                }
            }
        }
        TokenKind::StringLit(value) => {
            let value = value.clone();
            stream.advance();
            let parsed = parse_string_backed_param_value(param_name, &value, line_num)?
                .unwrap_or_else(|| xspice_string_value_from_param_preference(param_name, value));
            Ok(parsed)
        }
        TokenKind::LBracket => parse_vector_param_value(
            stream,
            line_num,
            param_name,
            netlist_params,
            defer_simple_param_refs,
        ),
        TokenKind::Ident(raw) => {
            let raw = raw.clone();
            if xspice_param_prefers_bare_string(param_name) {
                if let Some(value) = netlist_params.get_string(&raw) {
                    stream.advance();
                    if defer_simple_param_refs {
                        if xspice_param_prefers_string_vector(param_name) {
                            Ok(XspiceParamValue::StringVectorDeferred(raw))
                        } else {
                            Ok(XspiceParamValue::StringDeferred(raw))
                        }
                    } else {
                        let parsed = parse_string_backed_param_value(param_name, value, line_num)?
                            .unwrap_or_else(|| {
                                xspice_string_value_from_param_preference(
                                    param_name,
                                    value.to_string(),
                                )
                            });
                        Ok(parsed)
                    }
                } else {
                    parse_bare_string_param_value(stream, line_num, param_name)
                }
            } else {
                stream.advance();
                if let Some(value) = netlist_params.get(&raw) {
                    if defer_simple_param_refs {
                        Ok(XspiceParamValue::Deferred(raw))
                    } else {
                        Ok(XspiceParamValue::Resolved(value))
                    }
                } else if let Some(value) = parse_boolean_literal(&raw) {
                    Ok(XspiceParamValue::Resolved(value))
                } else if let Ok(value) = parse_spice_value(&raw) {
                    Ok(XspiceParamValue::Resolved(value))
                } else {
                    Ok(XspiceParamValue::Deferred(raw))
                }
            }
        }
        kind if xspice_param_prefers_bare_string(param_name)
            && scalar_string_param_token_can_start(kind) =>
        {
            parse_bare_string_param_value(stream, line_num, param_name)
        }
        _ => Err(ParseError::Syntax {
            line: line_num,
            message: "Expected parameter value".to_string(),
        }),
    }
}

fn xspice_param_prefers_bare_string(param_name: &str) -> bool {
    xspice_param_prefers_string(param_name) || xspice_param_prefers_string_vector(param_name)
}

fn xspice_string_value_from_param_preference(param_name: &str, value: String) -> XspiceParamValue {
    if xspice_param_prefers_string_vector(param_name) {
        XspiceParamValue::StringVector(vec![value])
    } else {
        XspiceParamValue::String(value)
    }
}

fn parse_bare_string_param_value(
    stream: &mut TokenStream,
    line_num: usize,
    param_name: &str,
) -> Result<XspiceParamValue, ParseError> {
    let mut value = String::new();
    let mut end = None;

    loop {
        let token = stream.peek().clone();
        match token.kind {
            TokenKind::Comma
            | TokenKind::RParen
            | TokenKind::RBracket
            | TokenKind::Newline
            | TokenKind::Eof => break,
            TokenKind::StringLit(_) if value.is_empty() => break,
            _ => {}
        }

        if let Some(previous_end) = end
            && token.span.start != previous_end
        {
            break;
        }

        let Some(piece) = scalar_string_param_piece_from_token(&token) else {
            break;
        };
        if piece.is_empty() {
            break;
        }

        value.push_str(&piece);
        end = Some(token.span.end);
        stream.advance();
    }

    if value.is_empty() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Expected XSPICE string parameter value for '{}', found '{}'",
                param_name,
                stream.peek().kind
            ),
        });
    }

    Ok(xspice_string_value_from_param_preference(param_name, value))
}

fn scalar_string_param_token_can_start(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Ident(_)
            | TokenKind::Number(_)
            | TokenKind::Equals
            | TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::AtSign
            | TokenKind::Tilde
            | TokenKind::Other(_)
    )
}

fn scalar_string_param_piece_from_token(token: &Token) -> Option<String> {
    match &token.kind {
        TokenKind::Ident(_)
        | TokenKind::Number(_)
        | TokenKind::Equals
        | TokenKind::Plus
        | TokenKind::Minus
        | TokenKind::Star
        | TokenKind::Slash
        | TokenKind::AtSign
        | TokenKind::Tilde
        | TokenKind::Other(_) => Some(if token.lexeme.is_empty() {
            token.kind.to_string()
        } else {
            token.lexeme.clone()
        }),
        _ => None,
    }
}

enum XspiceVectorEntry {
    Resolved(Value),
    Deferred(String),
}

fn parse_vector_param_value(
    stream: &mut TokenStream,
    line_num: usize,
    param_name: &str,
    netlist_params: &ParamContext,
    defer_simple_param_refs: bool,
) -> Result<XspiceParamValue, ParseError> {
    if vector_param_should_parse_as_string(
        stream,
        param_name,
        netlist_params,
        defer_simple_param_refs,
    ) {
        parse_string_vector_param(stream, line_num).map(XspiceParamValue::StringVector)
    } else {
        parse_real_vector_param(stream, line_num, netlist_params, defer_simple_param_refs)
    }
}

fn vector_param_should_parse_as_string(
    stream: &TokenStream,
    param_name: &str,
    netlist_params: &ParamContext,
    defer_simple_param_refs: bool,
) -> bool {
    if xspice_param_prefers_string_vector(param_name) {
        return true;
    }

    let mut probe = stream.clone();
    if !probe.consume(&TokenKind::LBracket) {
        return false;
    }
    skip_vector_commas(&mut probe);

    match &probe.peek().kind {
        TokenKind::StringLit(_) => true,
        TokenKind::Ident(_) if defer_simple_param_refs => false,
        TokenKind::Ident(value) => {
            netlist_params.get(value).is_none()
                && parse_boolean_literal(value).is_none()
                && parse_spice_value(value).is_err()
        }
        _ => false,
    }
}

fn parse_string_backed_param_value(
    param_name: &str,
    value: &str,
    line_num: usize,
) -> Result<Option<XspiceParamValue>, ParseError> {
    if !value.trim_start().starts_with('[')
        || (xspice_param_prefers_string(param_name)
            && !xspice_param_prefers_string_vector(param_name))
    {
        return Ok(None);
    }

    let tokens = tokenize(value).map_err(|err| ParseError::Syntax {
        line: line_num,
        message: format!("Invalid XSPICE vector parameter literal: {err}"),
    })?;
    let mut stream = TokenStream::new(tokens);
    parse_vector_param_value(
        &mut stream,
        line_num,
        param_name,
        &ParamContext::new(),
        false,
    )
    .map(Some)
}

pub(crate) fn parse_xspice_string_vector_literal(
    value: &str,
    line_num: usize,
    param_name: &str,
) -> Result<Vec<String>, ParseError> {
    let tokens = tokenize(value).map_err(|err| ParseError::Syntax {
        line: line_num,
        message: format!("Invalid XSPICE string-vector parameter literal: {err}"),
    })?;
    let mut stream = TokenStream::new(tokens);
    let values = parse_string_vector_param(&mut stream, line_num)?;
    while stream.consume(&TokenKind::Newline) {}
    if !stream.is_eof() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Unexpected token '{}' after XSPICE string-vector parameter '{}'",
                stream.peek().kind,
                param_name
            ),
        });
    }
    Ok(values)
}

fn parse_real_vector_param(
    stream: &mut TokenStream,
    line_num: usize,
    netlist_params: &ParamContext,
    defer_simple_param_refs: bool,
) -> Result<XspiceParamValue, ParseError> {
    if !stream.consume(&TokenKind::LBracket) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "Expected '[' for XSPICE vector parameter".to_string(),
        });
    }

    let mut entries = Vec::new();
    loop {
        skip_vector_commas(stream);

        match &stream.peek().kind {
            TokenKind::RBracket => {
                stream.advance();
                break;
            }
            TokenKind::Newline | TokenKind::Eof => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: "Unclosed XSPICE vector parameter".to_string(),
                });
            }
            _ => entries.push(parse_real_vector_entry(
                stream,
                line_num,
                netlist_params,
                defer_simple_param_refs,
            )?),
        }
    }

    if entries
        .iter()
        .any(|entry| matches!(entry, XspiceVectorEntry::Deferred(_)))
    {
        let exprs = entries
            .into_iter()
            .map(|entry| match entry {
                XspiceVectorEntry::Resolved(value) => value.to_string(),
                XspiceVectorEntry::Deferred(expr) => expr,
            })
            .collect();
        Ok(XspiceParamValue::RealVectorDeferred(exprs))
    } else {
        let values = entries
            .into_iter()
            .map(|entry| match entry {
                XspiceVectorEntry::Resolved(value) => value,
                XspiceVectorEntry::Deferred(_) => unreachable!(),
            })
            .collect();
        Ok(XspiceParamValue::RealVector(values))
    }
}

fn parse_real_vector_entry(
    stream: &mut TokenStream,
    line_num: usize,
    netlist_params: &ParamContext,
    defer_simple_param_refs: bool,
) -> Result<XspiceVectorEntry, ParseError> {
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

    let signed_expr = |expr: String| {
        if sign < 0.0 {
            format!("-({expr})")
        } else {
            expr
        }
    };

    match &stream.peek().kind {
        TokenKind::Number(value) => {
            let value = sign * *value;
            stream.advance();
            Ok(XspiceVectorEntry::Resolved(value))
        }
        TokenKind::Expression(expr_text) => {
            let expr_text = expr_text.clone();
            stream.advance();
            if !defer_simple_param_refs
                && let Ok(value) = expr::eval_expression(&expr_text, netlist_params)
            {
                Ok(XspiceVectorEntry::Resolved(sign * value))
            } else {
                Ok(XspiceVectorEntry::Deferred(signed_expr(expr_text)))
            }
        }
        TokenKind::Ident(raw) => {
            let raw = raw.clone();
            stream.advance();
            if let Ok(value) = parse_spice_value(&raw) {
                Ok(XspiceVectorEntry::Resolved(sign * value))
            } else if let Some(value) = parse_boolean_literal(&raw) {
                Ok(XspiceVectorEntry::Resolved(sign * value))
            } else if !defer_simple_param_refs && let Some(value) = netlist_params.get(&raw) {
                Ok(XspiceVectorEntry::Resolved(sign * value))
            } else {
                Ok(XspiceVectorEntry::Deferred(signed_expr(raw)))
            }
        }
        other => Err(ParseError::Syntax {
            line: line_num,
            message: format!("Expected XSPICE vector parameter value, found '{}'", other),
        }),
    }
}

fn parse_string_vector_param(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<Vec<String>, ParseError> {
    if !stream.consume(&TokenKind::LBracket) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "Expected '[' for XSPICE string-vector parameter".to_string(),
        });
    }

    let mut values = Vec::new();
    loop {
        skip_vector_commas(stream);

        match &stream.peek().kind {
            TokenKind::RBracket => {
                stream.advance();
                return Ok(values);
            }
            TokenKind::StringLit(value) => {
                let value = value.clone();
                stream.advance();
                values.push(value);
            }
            TokenKind::Newline | TokenKind::Eof => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: "Unclosed XSPICE string-vector parameter".to_string(),
                });
            }
            _ => values.push(parse_string_vector_bare_value(stream, line_num)?),
        }
    }
}

fn parse_string_vector_bare_value(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<String, ParseError> {
    let mut value = String::new();
    let mut end = None;

    loop {
        let token = stream.peek().clone();
        match token.kind {
            TokenKind::Comma | TokenKind::RBracket | TokenKind::Newline | TokenKind::Eof => break,
            TokenKind::StringLit(_) if value.is_empty() => break,
            _ => {}
        }

        if let Some(previous_end) = end
            && token.span.start != previous_end
        {
            break;
        }

        let Some(piece) = string_vector_piece_from_token(&token) else {
            break;
        };
        value.push_str(&piece);
        end = Some(token.span.end);
        stream.advance();
    }

    if value.is_empty() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Expected XSPICE string-vector value, found '{}'",
                stream.peek().kind
            ),
        });
    }
    Ok(value)
}

fn string_vector_piece_from_token(token: &Token) -> Option<String> {
    match &token.kind {
        TokenKind::StringLit(value) => Some(value.clone()),
        TokenKind::Ident(_)
        | TokenKind::Number(_)
        | TokenKind::Equals
        | TokenKind::LParen
        | TokenKind::RParen
        | TokenKind::Plus
        | TokenKind::Minus
        | TokenKind::Star
        | TokenKind::Slash
        | TokenKind::AtSign
        | TokenKind::Tilde
        | TokenKind::Other(_) => Some(if token.lexeme.is_empty() {
            token.kind.to_string()
        } else {
            token.lexeme.clone()
        }),
        _ => None,
    }
}

fn skip_vector_commas(stream: &mut TokenStream) {
    while stream.consume(&TokenKind::Comma) {}
}

pub(crate) fn xspice_param_prefers_string_vector(name: &str) -> bool {
    matches!(
        name.trim().to_ascii_lowercase().as_str(),
        "lib_args" | "sim_args" | "process_params"
    )
}

fn xspice_param_prefers_string(name: &str) -> bool {
    let normalized = name.trim().to_ascii_lowercase();
    xspice_model_param_accepts_bare_string(&normalized)
        || normalized == "model"
        || normalized.ends_with("_path")
}

pub(crate) fn xspice_model_param_accepts_bare_string(name: &str) -> bool {
    let normalized = name.trim().to_ascii_lowercase();
    normalized == "file"
        || normalized == "input_file"
        || normalized == "state_file"
        || normalized == "process_file"
        || normalized == "simulation"
        || normalized == "table_values"
        || normalized == "family"
        || normalized.ends_with("_file")
        || normalized.ends_with("file")
        || normalized.ends_with("path")
}

pub(crate) fn xspice_param_preserves_numeric_string(name: &str) -> bool {
    matches!(name.trim().to_ascii_lowercase().as_str(), "table_values")
}

fn parse_boolean_literal(raw: &str) -> Option<Value> {
    if raw.eq_ignore_ascii_case("true") {
        Some(1.0)
    } else if raw.eq_ignore_ascii_case("false") {
        Some(0.0)
    } else {
        None
    }
}

//=============================================================================
// Tests
//=============================================================================
