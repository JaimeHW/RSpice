//! Source specification parsing for SPICE netlists
//!
//! Parses source waveform specifications:
//! - DC: Simple DC value
//! - AC: Magnitude and phase for AC analysis
//! - PULSE: Pulse waveform with timing parameters
//! - SIN: Sinusoidal with damping
//! - PWL: Piecewise linear time-value pairs  
//! - EXP: Exponential rise/fall

use super::helpers::{expect_value, expect_value_default, skip_commas, try_value};
use super::lexer::{TokenKind, TokenStream};
use super::{ParamContext, ParseError, SourceSpec};
use crate::Value;

/// Parse source specification (DC, AC, PULSE, SIN, PWL, EXP)
///
/// Supports combined DC+AC syntax: "DC 0 AC 1" or "DC 5 AC 1 45"
pub fn parse_source_spec(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<SourceSpec, ParseError> {
    skip_commas(stream);

    if stream.is_eof() || matches!(stream.peek().kind, TokenKind::Newline) {
        return Err(ParseError::MissingParameter(
            "source specification".to_string(),
        ));
    }

    // Check for keywords
    match &stream.peek().kind {
        TokenKind::Ident(s) => {
            let upper = s.to_uppercase();
            match upper.as_str() {
                "DC" => {
                    stream.advance();
                    let dc_value = expect_value(stream, line_num, params)?;

                    // Check for optional AC specification after DC
                    skip_commas(stream);
                    if let TokenKind::Ident(next) = &stream.peek().kind {
                        if next.to_uppercase() == "AC" {
                            stream.advance();
                            let ac_magnitude = expect_value(stream, line_num, params)?;
                            // SPICE AC phase is specified in degrees; store radians internally.
                            let ac_phase = try_value(stream, params).unwrap_or(0.0).to_radians();
                            return Ok(SourceSpec::DcAc {
                                dc_value,
                                ac_magnitude,
                                ac_phase,
                            });
                        }
                    }
                    return Ok(SourceSpec::Dc(dc_value));
                }
                "AC" => {
                    stream.advance();
                    let magnitude = expect_value(stream, line_num, params)?;
                    // SPICE AC phase is specified in degrees; store radians internally.
                    let phase = try_value(stream, params).unwrap_or(0.0).to_radians();
                    return Ok(SourceSpec::Ac { magnitude, phase });
                }
                "PULSE" => {
                    stream.advance();
                    return parse_pulse_spec(stream, params);
                }
                "SIN" => {
                    stream.advance();
                    return parse_sin_spec(stream, params);
                }
                "PWL" => {
                    stream.advance();
                    return parse_pwl_spec(stream, params);
                }
                "EXP" => {
                    stream.advance();
                    return parse_exp_spec(stream, params);
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

/// Parse PULSE specification: PULSE(V1 V2 TD TR TF PW PER)
fn parse_pulse_spec(
    stream: &mut TokenStream,
    params: &ParamContext,
) -> Result<SourceSpec, ParseError> {
    // Consume opening paren if present
    let has_paren = stream.consume(&TokenKind::LParen);

    let v1 = expect_value_default(stream, params, 0.0);
    let v2 = expect_value_default(stream, params, 1.0);
    let delay = expect_value_default(stream, params, 0.0);
    // Preserve omitted timing arguments as NaN sentinels so runtime can apply
    // context-aware defaults from transient analysis settings.
    let rise = try_value(stream, params);
    let fall = try_value(stream, params);
    let width = try_value(stream, params);
    let width_defaults_to_zero = rise.is_some() && fall.is_some() && width.is_none();
    let period = try_value(stream, params).unwrap_or(Value::NAN);

    if has_paren {
        stream.consume(&TokenKind::RParen);
    }

    Ok(SourceSpec::Pulse {
        v1,
        v2,
        delay,
        rise: rise.unwrap_or(Value::NAN),
        fall: fall.unwrap_or(Value::NAN),
        width: width.unwrap_or(Value::NAN),
        period,
        width_defaults_to_zero,
    })
}

/// Parse SIN specification: SIN(VO VA FREQ TD THETA PHASE)
fn parse_sin_spec(
    stream: &mut TokenStream,
    params: &ParamContext,
) -> Result<SourceSpec, ParseError> {
    let has_paren = stream.consume(&TokenKind::LParen);

    let offset = expect_value_default(stream, params, 0.0);
    let amplitude = expect_value_default(stream, params, 1.0);
    let frequency = try_value(stream, params).unwrap_or(Value::NAN);
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

/// Parse PWL specification: PWL(T1 V1 T2 V2 ...) or PWL FILE="filename" [TSCALE=x] [VSCALE=x]
fn parse_pwl_spec(
    stream: &mut TokenStream,
    params: &ParamContext,
) -> Result<SourceSpec, ParseError> {
    let has_paren = stream.consume(&TokenKind::LParen);

    // Check for FILE= syntax
    if let TokenKind::Ident(s) = &stream.peek().kind {
        if s.to_uppercase() == "FILE" {
            stream.advance();
            stream.consume(&TokenKind::Equals);

            // Get filename (may be quoted or unquoted)
            let path = match &stream.peek().kind {
                TokenKind::String(s) => {
                    let p = s.clone();
                    stream.advance();
                    p
                }
                TokenKind::Ident(s) => {
                    let p = s.clone();
                    stream.advance();
                    p
                }
                _ => return Err(ParseError::MissingParameter("PWL filename".to_string())),
            };

            // Parse optional scaling parameters
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

    // Standard inline PWL points
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

/// Parse EXP specification: EXP(V1 V2 TD1 TAU1 TD2 TAU2)
fn parse_exp_spec(
    stream: &mut TokenStream,
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
