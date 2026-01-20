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
//! - `%vd[n+ n-]` - Differential voltage input/output
//! - `%id[n+ n-]` - Differential current input/output
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

use super::lexer::{TokenKind, TokenStream};
use super::{Element, ElementKind, ParseError, XspicePort};
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
) -> Result<(), ParseError> {
    // Collect all ports and potential model name in order
    // We use a strategy where all identifiers are added as ports,
    // and at the end we take the last analog port as the model name
    let mut ports = Vec::new();
    let mut params = Vec::new();

    loop {
        match &stream.peek().kind {
            // End of line
            TokenKind::Newline | TokenKind::Eof => break,

            // Digital port in brackets: [node] or [n1 n2 n3]
            TokenKind::LBracket => {
                let port = parse_bracketed_port(stream, line_num)?;
                ports.push(port);
            }

            // Potential differential port or analog node
            TokenKind::Ident(id) => {
                let id_str = id.clone();

                // Check if next token is '=' (this is a parameter)
                if matches!(stream.peek_n(1).kind, TokenKind::Equals) {
                    // This is a parameter assignment
                    stream.advance(); // consume identifier
                    stream.advance(); // consume '='
                    let value = parse_param_value(stream, line_num)?;
                    params.push((id_str, value));
                } else if id_str.starts_with('%') {
                    // Differential port: %vd[...] or %id[...]
                    let port = parse_differential_port(stream, line_num)?;
                    ports.push(port);
                } else if id_str.eq_ignore_ascii_case("null") {
                    // Null connection
                    stream.advance();
                    ports.push(XspicePort::Null);
                } else {
                    // This is an analog node - add it as a port for now
                    // The last analog identifier will be extracted as model name
                    stream.advance();
                    ports.push(XspicePort::Analog(id_str));
                }
            }

            // Number - could be a node name like "0"
            TokenKind::Number(n) => {
                let node_name = format!("{}", *n as i64);
                stream.advance();
                ports.push(XspicePort::Analog(node_name));
            }

            // Skip unexpected tokens
            _ => {
                stream.advance();
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
        },
        nodes: Vec::new(), // XSPICE uses ports instead of simple nodes
    });

    Ok(())
}

//=============================================================================
// Port Parsing Helpers
//=============================================================================

/// Parse a bracketed digital port: `[node]` or `[n1 n2 n3]`
fn parse_bracketed_port(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<XspicePort, ParseError> {
    // Consume opening bracket
    if !matches!(stream.peek().kind, TokenKind::LBracket) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "Expected '[' for digital port".to_string(),
        });
    }
    stream.advance();

    let mut nodes = Vec::new();

    // Collect nodes until closing bracket
    loop {
        match &stream.peek().kind {
            TokenKind::RBracket => {
                stream.advance(); // consume ']'
                break;
            }
            TokenKind::Ident(id) => {
                nodes.push(id.clone());
                stream.advance();
            }
            TokenKind::Number(n) => {
                // Allow numeric node names
                nodes.push(format!("{}", *n as i64));
                stream.advance();
            }
            TokenKind::Newline | TokenKind::Eof => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: "Unclosed bracket in digital port".to_string(),
                });
            }
            _ => {
                stream.advance(); // skip unexpected tokens
            }
        }
    }

    // Return appropriate port type
    match nodes.len() {
        0 => Ok(XspicePort::Null), // [] = null
        1 => Ok(XspicePort::Digital(nodes.into_iter().next().unwrap())),
        _ => Ok(XspicePort::DigitalVector(nodes)),
    }
}

/// Parse a differential port: `%vd[n+ n-]` or `%id(n+ n-)`
fn parse_differential_port(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<XspicePort, ParseError> {
    let prefix = match &stream.peek().kind {
        TokenKind::Ident(id) => {
            let p = id.clone();
            stream.advance();
            p
        }
        _ => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: "Expected differential port prefix".to_string(),
            });
        }
    };

    let prefix_lower = prefix.to_lowercase();
    let is_voltage = prefix_lower.starts_with("%vd");
    let is_current = prefix_lower.starts_with("%id");

    if !is_voltage && !is_current {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("Unknown differential port type: {}", prefix),
        });
    }

    // Parse nodes in brackets or parentheses
    let mut nodes = Vec::new();

    match &stream.peek().kind {
        TokenKind::LBracket => {
            stream.advance();
            loop {
                match &stream.peek().kind {
                    TokenKind::RBracket => {
                        stream.advance();
                        break;
                    }
                    TokenKind::Ident(id) => {
                        nodes.push(id.clone());
                        stream.advance();
                    }
                    _ => break,
                }
            }
        }
        TokenKind::LParen => {
            stream.advance();
            loop {
                match &stream.peek().kind {
                    TokenKind::RParen => {
                        stream.advance();
                        break;
                    }
                    TokenKind::Ident(id) => {
                        nodes.push(id.clone());
                        stream.advance();
                    }
                    _ => break,
                }
            }
        }
        _ => {}
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
        Ok(XspicePort::DifferentialVoltage { pos, neg })
    } else {
        Ok(XspicePort::DifferentialCurrent { pos, neg })
    }
}

/// Parse a parameter value (number)
fn parse_param_value(stream: &mut TokenStream, line_num: usize) -> Result<Value, ParseError> {
    match &stream.peek().kind {
        TokenKind::Number(n) => {
            let v = *n;
            stream.advance();
            Ok(v)
        }
        _ => Err(ParseError::Syntax {
            line: line_num,
            message: "Expected parameter value".to_string(),
        }),
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::super::lexer::tokenize;
    use super::*;

    fn parse_line(input: &str) -> Result<Vec<Element>, ParseError> {
        let tokens = tokenize(input).map_err(|e| ParseError::Syntax {
            line: 1,
            message: e.to_string(),
        })?;
        let mut stream = TokenStream::new(tokens);
        let mut elements = Vec::new();

        // Extract element name (first identifier)
        let name = match &stream.peek().kind {
            TokenKind::Ident(id) => {
                let n = id.clone();
                stream.advance();
                n
            }
            _ => {
                return Err(ParseError::Syntax {
                    line: 1,
                    message: "Expected element name".to_string(),
                });
            }
        };

        parse_xspice(&mut stream, 1, name, &mut elements)?;
        Ok(elements)
    }

    #[test]
    fn test_parse_simple_analog() {
        let elements = parse_line("A1 in out gain").unwrap();
        assert_eq!(elements.len(), 1);

        if let ElementKind::Xspice {
            model,
            ports,
            params,
        } = &elements[0].kind
        {
            assert_eq!(model, "GAIN");
            assert_eq!(ports.len(), 2);
            assert!(matches!(&ports[0], XspicePort::Analog(n) if n == "IN"));
            assert!(matches!(&ports[1], XspicePort::Analog(n) if n == "OUT"));
            assert!(params.is_empty());
        } else {
            panic!("Expected Xspice element");
        }
    }

    #[test]
    fn test_parse_digital_gate() {
        let elements = parse_line("A2 [a] [b] [y] d_and").unwrap();
        assert_eq!(elements.len(), 1);

        if let ElementKind::Xspice { model, ports, .. } = &elements[0].kind {
            assert_eq!(model, "D_AND");
            assert_eq!(ports.len(), 3);
            assert!(ports[0].is_digital());
            assert!(ports[1].is_digital());
            assert!(ports[2].is_digital());
        } else {
            panic!("Expected Xspice element");
        }
    }

    #[test]
    fn test_parse_digital_vector() {
        let elements = parse_line("A3 [a b c d] [y] d_and").unwrap();

        if let ElementKind::Xspice { ports, .. } = &elements[0].kind {
            assert_eq!(ports.len(), 2);
            if let XspicePort::DigitalVector(nodes) = &ports[0] {
                assert_eq!(nodes.len(), 4);
                assert_eq!(nodes, &["A", "B", "C", "D"]);
            } else {
                panic!("Expected digital vector");
            }
        } else {
            panic!("Expected Xspice element");
        }
    }

    #[test]
    fn test_parse_with_params() {
        let elements = parse_line("A1 in out gain gain=2.0").unwrap();

        if let ElementKind::Xspice { params, .. } = &elements[0].kind {
            assert_eq!(params.len(), 1);
            assert_eq!(params[0].0, "GAIN");
            assert!((params[0].1 - 2.0).abs() < 1e-10);
        } else {
            panic!("Expected Xspice element");
        }
    }

    #[test]
    fn test_parse_null_port() {
        let elements = parse_line("A1 null out d_source").unwrap();

        if let ElementKind::Xspice { ports, .. } = &elements[0].kind {
            assert!(ports[0].is_null());
        } else {
            panic!("Expected Xspice element");
        }
    }

    #[test]
    fn test_parse_empty_bracket() {
        let elements = parse_line("A1 [] out d_source").unwrap();

        if let ElementKind::Xspice { ports, .. } = &elements[0].kind {
            assert!(ports[0].is_null());
        } else {
            panic!("Expected Xspice element");
        }
    }

    #[test]
    fn test_xspice_port_methods() {
        assert!(XspicePort::Analog("n1".into()).is_analog());
        assert!(XspicePort::Digital("n1".into()).is_digital());
        assert!(XspicePort::Null.is_null());

        let diff = XspicePort::DifferentialVoltage {
            pos: "p".into(),
            neg: "n".into(),
        };
        assert!(diff.is_analog());

        let nodes = diff.node_names();
        assert_eq!(nodes, vec!["p", "n"]);
    }

    #[test]
    fn test_mixed_signal_bridge() {
        let elements = parse_line("A1 analog_in [digital_out] adc_bridge").unwrap();

        if let ElementKind::Xspice { model, ports, .. } = &elements[0].kind {
            assert_eq!(model, "ADC_BRIDGE");
            assert_eq!(ports.len(), 2);
            assert!(ports[0].is_analog());
            assert!(ports[1].is_digital());
        } else {
            panic!("Expected Xspice element");
        }
    }
}
