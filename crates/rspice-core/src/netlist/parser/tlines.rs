//! Coupling, switch, and transmission-line parsers.

use super::*;

fn reject_unexpected_tail(
    stream: &mut TokenStream,
    line_num: usize,
    element_label: &str,
) -> Result<(), ParseError> {
    skip_commas(stream);
    if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        return Ok(());
    }

    Err(ParseError::Syntax {
        line: line_num,
        message: format!(
            "Unexpected trailing token in {element_label} specification: {}",
            stream.peek().kind
        ),
    })
}

/// Parse coupling coefficient: K1 L1 L2 [L3...] coefficient
pub(super) fn parse_coupling(
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
    if !coefficient.is_finite() || !(0.0..=1.0).contains(&coefficient) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Invalid coupling coefficient {coefficient}: expected a finite value in [0, 1]"
            ),
        });
    }
    reject_unexpected_tail(stream, line_num, "coupling")?;

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
pub(super) fn parse_vswitch(
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
    reject_unexpected_tail(stream, line_num, "voltage-controlled switch")?;

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
pub(super) fn parse_iswitch(
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
    reject_unexpected_tail(stream, line_num, "current-controlled switch")?;

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
pub(super) fn parse_switch_state(stream: &mut TokenStream) -> Option<SwitchState> {
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
pub(super) fn parse_transmission_line(
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
pub(super) struct ParsedTlineParams {
    pub(super) z0: Option<Value>,
    pub(super) td: Option<Value>,
    pub(super) freq: Option<Value>,
    pub(super) nl: Option<Value>,
    pub(super) model: Option<String>,
}

pub(super) fn parse_tline_params(
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
            } else {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: "Transmission line has too many positional numeric parameters"
                        .to_string(),
                });
            }
            continue;
        }

        let TokenKind::Ident(token) = &stream.peek().kind else {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    "Unsupported transmission-line parameter token '{}'",
                    stream.peek().kind
                ),
            });
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
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!("Unsupported transmission-line parameter '{}'", token),
                });
            }
        }
    }

    Ok(parsed)
}
