//! Source specification parsing for independent, voltage, and current sources.

use super::*;
//=============================================================================
// Source Specification Parsing
//=============================================================================

/// Parse source specification (DC, AC, PULSE, SIN, PWL, EXP)
///
/// Supports combined DC+AC syntax: "DC 0 AC 1" or "DC 5 AC 1 45"
pub(super) fn parse_source_spec(
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
    if let TokenKind::Ident(s) = &stream.peek().kind {
        let upper = s.to_uppercase();
        match upper.as_str() {
            "DC" => {
                stream.advance();
                // Allow optional = after DC (e.g., "dc = 5" or "dc 5")
                skip_commas(stream);
                stream.consume(&TokenKind::Equals);
                let dc_value = expect_value(stream, line_num, params)?;

                let mut ac_terms: Option<(Value, Value)> = None;

                // Optional AC specification after DC
                skip_commas(stream);
                if let TokenKind::Ident(next) = &stream.peek().kind
                    && next.to_uppercase() == "AC"
                {
                    stream.advance();
                    let ac_magnitude = try_value(stream, params).unwrap_or(1.0);
                    // SPICE AC phase is specified in degrees; store radians internally.
                    let ac_phase = try_value(stream, params).unwrap_or(0.0).to_radians();
                    ac_terms = Some((ac_magnitude, ac_phase));
                }

                let transient = parse_transient_source_spec_keyword(stream, line_num, params)?;
                return Ok(match (ac_terms, transient) {
                    (Some((ac_magnitude, ac_phase)), Some(transient)) => {
                        SourceSpec::DcAcTransient {
                            dc_value,
                            ac_magnitude,
                            ac_phase,
                            transient: Box::new(transient),
                        }
                    }
                    (Some((ac_magnitude, ac_phase)), None) => SourceSpec::DcAc {
                        dc_value,
                        ac_magnitude,
                        ac_phase,
                    },
                    (None, Some(transient)) => SourceSpec::DcTransient {
                        dc_value,
                        transient: Box::new(transient),
                    },
                    (None, None) => SourceSpec::Dc(dc_value),
                });
            }
            "AC" => {
                stream.advance();
                // AC magnitude is optional - defaults to 1.0 if not specified
                let ac_magnitude = try_value(stream, params).unwrap_or(1.0);
                // SPICE AC phase is specified in degrees; store radians internally.
                let ac_phase = try_value(stream, params).unwrap_or(0.0).to_radians();

                // Support ngspice ordering like:
                //   AC 1 DC 0 SIN(...)
                // by accepting optional DC and transient terms after AC.
                skip_commas(stream);
                let mut dc_value = 0.0;
                let mut has_dc_term = false;
                if let TokenKind::Ident(next) = &stream.peek().kind
                    && next.to_uppercase() == "DC"
                {
                    stream.advance();
                    skip_commas(stream);
                    stream.consume(&TokenKind::Equals);
                    dc_value = expect_value(stream, line_num, params)?;
                    has_dc_term = true;
                }

                let transient = parse_transient_source_spec_keyword(stream, line_num, params)?;
                return Ok(match transient {
                    Some(transient) => SourceSpec::DcAcTransient {
                        dc_value,
                        ac_magnitude,
                        ac_phase,
                        transient: Box::new(transient),
                    },
                    None if has_dc_term => SourceSpec::DcAc {
                        dc_value,
                        ac_magnitude,
                        ac_phase,
                    },
                    None => SourceSpec::Ac {
                        magnitude: ac_magnitude,
                        phase: ac_phase,
                    },
                });
            }
            "PULSE" => {
                stream.advance();
                return parse_pulse_spec(stream, line_num, params);
            }
            "SIN" | "SINE" => {
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
            "SFFM" => {
                stream.advance();
                return parse_sffm_spec(stream, line_num, params);
            }
            "AM" => {
                stream.advance();
                return parse_am_spec(stream, line_num, params);
            }
            _ => {}
        }
    }

    // Default: try to parse as DC value
    let value = expect_value(stream, line_num, params)?;
    if let Some(transient) = parse_transient_source_spec_keyword(stream, line_num, params)? {
        Ok(SourceSpec::DcTransient {
            dc_value: value,
            transient: Box::new(transient),
        })
    } else {
        Ok(SourceSpec::Dc(value))
    }
}

fn parse_transient_source_spec_keyword(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<Option<SourceSpec>, ParseError> {
    skip_commas(stream);
    let TokenKind::Ident(keyword) = &stream.peek().kind else {
        return Ok(None);
    };

    match keyword.to_uppercase().as_str() {
        "PULSE" => {
            stream.advance();
            parse_pulse_spec(stream, line_num, params).map(Some)
        }
        "SIN" => {
            stream.advance();
            parse_sin_spec(stream, line_num, params).map(Some)
        }
        "SINE" => {
            stream.advance();
            parse_sin_spec(stream, line_num, params).map(Some)
        }
        "PWL" => {
            stream.advance();
            parse_pwl_spec(stream, line_num, params).map(Some)
        }
        "EXP" => {
            stream.advance();
            parse_exp_spec(stream, line_num, params).map(Some)
        }
        "SFFM" => {
            stream.advance();
            parse_sffm_spec(stream, line_num, params).map(Some)
        }
        "AM" => {
            stream.advance();
            parse_am_spec(stream, line_num, params).map(Some)
        }
        _ => Ok(None),
    }
}

/// Parse SFFM(VO VA FC MDI FM TD PHASEM PHASEC); omitted frequencies stay
/// NaN so the transient runtime can resolve ngspice's tstop-based defaults.
fn parse_sffm_spec(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<SourceSpec, ParseError> {
    let has_paren = stream.consume(&TokenKind::LParen);

    let offset = expect_value_default(stream, params, 0.0);
    let amplitude = expect_value_default(stream, params, 1.0);
    let carrier_freq = try_value(stream, params).unwrap_or(Value::NAN);
    let modulation_index = try_value(stream, params).unwrap_or(Value::NAN);
    let signal_freq = try_value(stream, params).unwrap_or(Value::NAN);
    let delay = expect_value_default(stream, params, 0.0);
    // SFFM/AM phases stay in degrees: the runtime converts exactly like
    // ngspice's vsrcload.c so the stored spec mirrors the netlist text.
    let phase_modulation = expect_value_default(stream, params, 0.0);
    let phase_carrier = expect_value_default(stream, params, 0.0);

    if has_paren {
        stream.consume(&TokenKind::RParen);
    }

    if carrier_freq.is_finite()
        && signal_freq.is_finite()
        && signal_freq > 0.0
        && modulation_index.is_finite()
        && modulation_index > carrier_freq / signal_freq
    {
        log::warn!(
            "line {line_num}: SFFM modulation index {modulation_index} exceeds FC/FM = {}; \
             it will be limited during simulation (ngspice behavior)",
            carrier_freq / signal_freq
        );
    }

    Ok(SourceSpec::Sffm {
        offset,
        amplitude,
        carrier_freq,
        modulation_index,
        signal_freq,
        delay,
        phase_modulation,
        phase_carrier,
    })
}

/// Parse AM(VO VMO VMA FM FC TD PHASEM PHASEC); omitted frequencies stay
/// NaN so the transient runtime can resolve ngspice's tstop-based defaults.
fn parse_am_spec(
    stream: &mut TokenStream,
    _line_num: usize,
    params: &ParamContext,
) -> Result<SourceSpec, ParseError> {
    let has_paren = stream.consume(&TokenKind::LParen);

    let offset = expect_value_default(stream, params, 0.0);
    let modulation_offset = expect_value_default(stream, params, 0.0);
    let modulation_amplitude = expect_value_default(stream, params, 1.0);
    let modulating_freq = try_value(stream, params).unwrap_or(Value::NAN);
    let carrier_freq = try_value(stream, params).unwrap_or(Value::NAN);
    let delay = expect_value_default(stream, params, 0.0);
    let phase_modulation = expect_value_default(stream, params, 0.0);
    let phase_carrier = expect_value_default(stream, params, 0.0);

    if has_paren {
        stream.consume(&TokenKind::RParen);
    }

    Ok(SourceSpec::Am {
        offset,
        modulation_offset,
        modulation_amplitude,
        modulating_freq,
        carrier_freq,
        delay,
        phase_modulation,
        phase_carrier,
    })
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
    // Keep omitted timing fields as NaN sentinels so transient runtime can
    // resolve ngspice-compatible defaults from .TRAN context (tstep/tstop).
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

fn parse_sin_spec(
    stream: &mut TokenStream,
    _line_num: usize,
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

fn parse_pwl_spec(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<SourceSpec, ParseError> {
    let has_paren = stream.consume(&TokenKind::LParen);

    // PWL FILE="path" [TSCALE=..] [VSCALE=..] [TOFFSET=..] [VOFFSET=..]
    if let TokenKind::Ident(s) = &stream.peek().kind
        && s.eq_ignore_ascii_case("FILE")
    {
        stream.advance();
        stream.consume(&TokenKind::Equals);

        let path = match &stream.peek().kind {
            TokenKind::StringLit(s) => {
                let p = s.clone();
                stream.advance();
                p
            }
            TokenKind::Ident(s) => {
                let p = s.clone();
                stream.advance();
                p
            }
            _ => {
                return Err(ParseError::MissingParameter(format!(
                    "PWL filename at line {}",
                    line_num
                )));
            }
        };

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
