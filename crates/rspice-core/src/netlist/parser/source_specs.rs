//! Source specification parsing for independent, voltage, and current sources.

use super::*;
//=============================================================================
// Source Specification Parsing
//=============================================================================

/// Parse source specification (DC, AC, PULSE, SIN, PWL, EXP, SFFM, AM,
/// TRNOISE)
///
/// Like ngspice, the DC level, AC small-signal terms, and the transient
/// function may appear in any order on the card, including AC after the
/// transient function ("DC 1 SIN(...) AC 1").
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

    let mut dc_value: Option<Value> = None;
    let mut ac_terms: Option<(Value, Value)> = None;
    let mut transient: Option<SourceSpec> = None;

    loop {
        skip_commas(stream);
        let keyword = match &stream.peek().kind {
            TokenKind::Ident(s) => s.to_uppercase(),
            _ => {
                // A bare leading value is the DC level.
                if dc_value.is_none() && ac_terms.is_none() && transient.is_none() {
                    if let Some(v) = try_value(stream, params) {
                        dc_value = Some(v);
                        continue;
                    }
                }
                break;
            }
        };

        match keyword.as_str() {
            "DC" if dc_value.is_none() => {
                stream.advance();
                // Allow optional = after DC (e.g., "dc = 5" or "dc 5")
                skip_commas(stream);
                stream.consume(&TokenKind::Equals);
                dc_value = Some(expect_value(stream, line_num, params)?);
            }
            "AC" if ac_terms.is_none() => {
                stream.advance();
                // AC magnitude is optional - defaults to 1.0 if not specified
                let ac_magnitude = try_value(stream, params).unwrap_or(1.0);
                // SPICE AC phase is specified in degrees; store radians internally.
                let ac_phase = try_value(stream, params).unwrap_or(0.0).to_radians();
                ac_terms = Some((ac_magnitude, ac_phase));
            }
            _ if transient.is_none() => {
                match parse_transient_source_spec_keyword(stream, line_num, params)? {
                    Some(spec) => transient = Some(spec),
                    None => break,
                }
            }
            _ => break,
        }
    }

    Ok(match (dc_value, ac_terms, transient) {
        (None, None, None) => {
            // Nothing recognized: surface the same error a bad value gives.
            SourceSpec::Dc(expect_value(stream, line_num, params)?)
        }
        (Some(dc_value), None, None) => SourceSpec::Dc(dc_value),
        (None, Some((magnitude, phase)), None) => SourceSpec::Ac { magnitude, phase },
        (Some(dc_value), Some((ac_magnitude, ac_phase)), None) => SourceSpec::DcAc {
            dc_value,
            ac_magnitude,
            ac_phase,
        },
        (None, None, Some(transient)) => transient,
        (Some(dc_value), None, Some(transient)) => SourceSpec::DcTransient {
            dc_value,
            transient: Box::new(transient),
        },
        (dc_value, Some((ac_magnitude, ac_phase)), Some(transient)) => SourceSpec::DcAcTransient {
            dc_value: dc_value.unwrap_or(0.0),
            ac_magnitude,
            ac_phase,
            transient: Box::new(transient),
        },
    })
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
        "TRNOISE" => {
            stream.advance();
            parse_trnoise_spec(stream, line_num, params).map(Some)
        }
        _ => Ok(None),
    }
}

/// Parse TRNOISE(NA NT NALPHA NAMP [RTSAM RTSCAPT RTSEMT]).
///
/// White (`NA`/`NT`) and 1/f (`NALPHA`/`NAMP`) terms are supported; the RTS
/// (random telegraph) tail is recognized but rejected with a clear
/// diagnostic rather than silently ignored.
fn parse_trnoise_spec(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<SourceSpec, ParseError> {
    let has_paren = stream.consume(&TokenKind::LParen);

    let na = expect_value_default(stream, params, 0.0);
    let nt = expect_value_default(stream, params, 0.0);
    let nalpha = expect_value_default(stream, params, 0.0);
    let namp = expect_value_default(stream, params, 0.0);
    let rts = try_value(stream, params);

    if has_paren {
        stream.consume(&TokenKind::RParen);
    }

    if rts.is_some_and(|v| v != 0.0) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "TRNOISE RTS (random telegraph) parameters are not supported yet; \
                      set RTSAM=0 or omit the tail"
                .to_string(),
        });
    }

    if (na != 0.0 || namp != 0.0) && !(nt.is_finite() && nt > 0.0) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "TRNOISE requires a positive sample interval NT".to_string(),
        });
    }
    if namp != 0.0 && !(nalpha > 0.0 && nalpha < 2.0) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "TRNOISE NALPHA must satisfy 0 < NALPHA < 2 when NAMP is set".to_string(),
        });
    }

    Ok(SourceSpec::TrNoise {
        na,
        nt,
        nalpha,
        namp,
    })
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

/// Parse EXP(V1 V2 TD1 TAU1 TD2 TAU2); omitted timing parameters stay NaN
/// so the transient runtime can resolve ngspice's tstep-based defaults
/// (TD1/TAU1/TAU2 default to TSTEP, TD2 to TD1+TSTEP).
fn parse_exp_spec(
    stream: &mut TokenStream,
    _line_num: usize,
    params: &ParamContext,
) -> Result<SourceSpec, ParseError> {
    let has_paren = stream.consume(&TokenKind::LParen);

    let v1 = expect_value_default(stream, params, 0.0);
    let v2 = expect_value_default(stream, params, 1.0);
    let td1 = try_value(stream, params).unwrap_or(Value::NAN);
    let tau1 = try_value(stream, params).unwrap_or(Value::NAN);
    let td2 = try_value(stream, params).unwrap_or(Value::NAN);
    let tau2 = try_value(stream, params).unwrap_or(Value::NAN);

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
