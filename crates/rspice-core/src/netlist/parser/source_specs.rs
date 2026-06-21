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
                if dc_value.is_none()
                    && ac_terms.is_none()
                    && transient.is_none()
                    && let Some(v) = try_value(stream, params)
                {
                    if !v.is_finite() {
                        return Err(non_finite_source_value_error(line_num, "DC", "value", v));
                    }
                    dc_value = Some(v);
                    continue;
                }
                break;
            }
        };

        match keyword.as_str() {
            "DISTOF1" | "DISTOF2" => {
                consume_distortion_source_annotation(stream, line_num, params)?;
            }
            "DC" if dc_value.is_none() => {
                stream.advance();
                // Allow optional = after DC (e.g., "dc = 5" or "dc 5")
                skip_commas(stream);
                stream.consume(&TokenKind::Equals);
                dc_value = Some(expect_finite_source_value(
                    stream, line_num, params, "DC", "value",
                )?);
            }
            "AC" if ac_terms.is_none() => {
                stream.advance();
                skip_commas(stream);
                stream.consume(&TokenKind::Equals);
                // AC magnitude is optional - defaults to 1.0 if not specified
                let ac_magnitude =
                    optional_ac_value_or_default(stream, line_num, params, "magnitude", 1.0)?;
                // SPICE AC phase is specified in degrees; store radians internally.
                let ac_phase =
                    optional_ac_value_or_default(stream, line_num, params, "phase", 0.0)?
                        .to_radians();
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

    skip_commas(stream);
    if !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Unexpected trailing token in source specification: {}",
                stream.peek().kind
            ),
        });
    }

    Ok(match (dc_value, ac_terms, transient) {
        (None, None, None) => {
            // Nothing recognized: surface the same error a bad value gives.
            SourceSpec::Dc(expect_finite_source_value(
                stream, line_num, params, "DC", "value",
            )?)
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

fn consume_distortion_source_annotation(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let TokenKind::Ident(keyword) = &stream.peek().kind else {
        return Ok(());
    };
    let source_name = keyword.to_ascii_uppercase();
    stream.advance();

    let _magnitude =
        optional_distortion_value(stream, line_num, params, &source_name, "magnitude")?;
    let _phase = optional_distortion_value(stream, line_num, params, &source_name, "phase")?;

    Ok(())
}

fn optional_distortion_value(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    source_name: &str,
    arg_name: &str,
) -> Result<Option<Value>, ParseError> {
    skip_commas(stream);
    if source_distortion_annotation_end(stream) {
        return Ok(None);
    }

    let found = stream.peek().kind.to_string();
    let value = expect_value(stream, line_num, params).map_err(|err| ParseError::Syntax {
        line: line_num,
        message: format!("{source_name} {arg_name} expected numeric value, found {found} ({err})"),
    })?;
    if !value.is_finite() {
        return Err(non_finite_source_value_error(
            line_num,
            source_name,
            arg_name,
            value,
        ));
    }
    Ok(Some(value))
}

fn source_distortion_annotation_end(stream: &TokenStream) -> bool {
    match &stream.peek().kind {
        TokenKind::Newline | TokenKind::Eof => true,
        TokenKind::Ident(keyword) => is_source_level_keyword(keyword),
        _ => false,
    }
}

fn is_source_level_keyword(keyword: &str) -> bool {
    matches!(
        keyword.to_ascii_uppercase().as_str(),
        "DC" | "AC"
            | "PULSE"
            | "SIN"
            | "SINE"
            | "PWL"
            | "EXP"
            | "SFFM"
            | "AM"
            | "TRNOISE"
            | "DISTOF1"
            | "DISTOF2"
    )
}

fn optional_ac_value_or_default(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    arg_name: &str,
    default: Value,
) -> Result<Value, ParseError> {
    skip_commas(stream);
    if ac_term_is_omitted(stream) {
        return Ok(default);
    }

    let found = stream.peek().kind.to_string();
    let value = expect_value(stream, line_num, params).map_err(|err| ParseError::Syntax {
        line: line_num,
        message: format!("AC {arg_name} expected numeric value, found {found} ({err})"),
    })?;
    if !value.is_finite() {
        return Err(non_finite_source_value_error(
            line_num, "AC", arg_name, value,
        ));
    }
    Ok(value)
}

fn ac_term_is_omitted(stream: &TokenStream) -> bool {
    match &stream.peek().kind {
        TokenKind::Newline | TokenKind::Eof => true,
        TokenKind::Ident(keyword) => is_source_level_keyword(keyword),
        _ => false,
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

    let na = source_value_or_default(stream, line_num, params, "TRNOISE", "NA", has_paren, 0.0)?;
    let nt = source_value_or_default(stream, line_num, params, "TRNOISE", "NT", has_paren, 0.0)?;
    let nalpha = source_value_or_default(
        stream, line_num, params, "TRNOISE", "NALPHA", has_paren, 0.0,
    )?;
    let namp =
        source_value_or_default(stream, line_num, params, "TRNOISE", "NAMP", has_paren, 0.0)?;
    let rts_amplitude =
        source_optional_value(stream, line_num, params, "TRNOISE", "RTSAM", has_paren)?;
    let rts_capture =
        source_optional_value(stream, line_num, params, "TRNOISE", "RTSCAPT", has_paren)?;
    let rts_emit = source_optional_value(stream, line_num, params, "TRNOISE", "RTSEMT", has_paren)?;

    close_source_args(stream, line_num, "TRNOISE", has_paren)?;

    if [rts_amplitude, rts_capture, rts_emit]
        .into_iter()
        .flatten()
        .any(|value| value != 0.0)
    {
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

    let offset = source_value_or_default(stream, line_num, params, "SFFM", "VO", has_paren, 0.0)?;
    let amplitude =
        source_value_or_default(stream, line_num, params, "SFFM", "VA", has_paren, 1.0)?;
    let carrier_freq = source_optional_value(stream, line_num, params, "SFFM", "FC", has_paren)?
        .unwrap_or(Value::NAN);
    let modulation_index =
        source_optional_value(stream, line_num, params, "SFFM", "MDI", has_paren)?
            .unwrap_or(Value::NAN);
    let signal_freq = source_optional_value(stream, line_num, params, "SFFM", "FM", has_paren)?
        .unwrap_or(Value::NAN);
    let delay = source_value_or_default(stream, line_num, params, "SFFM", "TD", has_paren, 0.0)?;
    // SFFM/AM phases stay in degrees: the runtime converts exactly like
    // ngspice's vsrcload.c so the stored spec mirrors the netlist text.
    let phase_modulation =
        source_value_or_default(stream, line_num, params, "SFFM", "PHASEM", has_paren, 0.0)?;
    let phase_carrier =
        source_value_or_default(stream, line_num, params, "SFFM", "PHASEC", has_paren, 0.0)?;

    close_source_args(stream, line_num, "SFFM", has_paren)?;

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
    line_num: usize,
    params: &ParamContext,
) -> Result<SourceSpec, ParseError> {
    let has_paren = stream.consume(&TokenKind::LParen);

    let offset = source_value_or_default(stream, line_num, params, "AM", "VO", has_paren, 0.0)?;
    let modulation_offset =
        source_value_or_default(stream, line_num, params, "AM", "VMO", has_paren, 0.0)?;
    let modulation_amplitude =
        source_value_or_default(stream, line_num, params, "AM", "VMA", has_paren, 1.0)?;
    let modulating_freq = source_optional_value(stream, line_num, params, "AM", "FM", has_paren)?
        .unwrap_or(Value::NAN);
    let carrier_freq = source_optional_value(stream, line_num, params, "AM", "FC", has_paren)?
        .unwrap_or(Value::NAN);
    let delay = source_value_or_default(stream, line_num, params, "AM", "TD", has_paren, 0.0)?;
    let phase_modulation =
        source_value_or_default(stream, line_num, params, "AM", "PHASEM", has_paren, 0.0)?;
    let phase_carrier =
        source_value_or_default(stream, line_num, params, "AM", "PHASEC", has_paren, 0.0)?;

    close_source_args(stream, line_num, "AM", has_paren)?;

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
    line_num: usize,
    params: &ParamContext,
) -> Result<SourceSpec, ParseError> {
    // Consume opening paren if present
    let has_paren = stream.consume(&TokenKind::LParen);

    let v1 = source_value_or_default(stream, line_num, params, "PULSE", "V1", has_paren, 0.0)?;
    let v2 = source_value_or_default(stream, line_num, params, "PULSE", "V2", has_paren, 1.0)?;
    let delay = source_value_or_default(stream, line_num, params, "PULSE", "TD", has_paren, 0.0)?;
    // Keep omitted timing fields as NaN sentinels so transient runtime can
    // resolve ngspice-compatible defaults from .TRAN context (tstep/tstop).
    let rise = source_optional_value(stream, line_num, params, "PULSE", "TR", has_paren)?;
    let fall = source_optional_value(stream, line_num, params, "PULSE", "TF", has_paren)?;
    let width = source_optional_value(stream, line_num, params, "PULSE", "PW", has_paren)?;
    let width_defaults_to_zero = rise.is_some() && fall.is_some() && width.is_none();
    let period = source_optional_value(stream, line_num, params, "PULSE", "PER", has_paren)?
        .unwrap_or(Value::NAN);

    close_source_args(stream, line_num, "PULSE", has_paren)?;

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
    line_num: usize,
    params: &ParamContext,
) -> Result<SourceSpec, ParseError> {
    let has_paren = stream.consume(&TokenKind::LParen);

    let offset = source_value_or_default(stream, line_num, params, "SIN", "VO", has_paren, 0.0)?;
    let amplitude = source_value_or_default(stream, line_num, params, "SIN", "VA", has_paren, 1.0)?;
    let frequency = source_optional_value(stream, line_num, params, "SIN", "FREQ", has_paren)?
        .unwrap_or(Value::NAN);
    let delay = source_value_or_default(stream, line_num, params, "SIN", "TD", has_paren, 0.0)?;
    let damping =
        source_value_or_default(stream, line_num, params, "SIN", "THETA", has_paren, 0.0)?;
    // SPICE SIN phase is specified in degrees; store radians internally.
    let phase = source_value_or_default(stream, line_num, params, "SIN", "PHASE", has_paren, 0.0)?
        .to_radians();

    close_source_args(stream, line_num, "SIN", has_paren)?;

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

        loop {
            skip_commas(stream);
            if source_args_end(stream, has_paren) {
                break;
            }

            let TokenKind::Ident(key) = &stream.peek().kind else {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!("Unsupported PWL FILE option token '{}'", stream.peek().kind),
                });
            };
            let key = key.clone();
            let key_upper = key.to_uppercase();
            stream.advance();

            let target = match key_upper.as_str() {
                "TSCALE" | "TIMESCALE" => &mut time_scale,
                "VSCALE" | "VALUESCALE" | "SCALE" => &mut value_scale,
                "TOFFSET" | "TIMEOFFSET" | "TD" => &mut time_offset,
                "VOFFSET" | "VALUEOFFSET" | "DC" => &mut value_offset,
                _ => {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!("Unsupported PWL FILE option '{key}'"),
                    });
                }
            };

            if !stream.consume(&TokenKind::Equals) {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!("PWL FILE option '{key}' requires '='"),
                });
            }

            *target =
                source_optional_value(stream, line_num, params, "PWL FILE", &key_upper, has_paren)?
                    .ok_or_else(|| ParseError::Syntax {
                        line: line_num,
                        message: format!("PWL FILE option '{key}' requires a value"),
                    })?;
        }

        close_source_args(stream, line_num, "PWL FILE", has_paren)?;
        validate_pwl_file_scaling(line_num, time_scale, value_scale, time_offset, value_offset)?;

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

        if let Some(time) =
            source_optional_value(stream, line_num, params, "PWL", "time", has_paren)?
        {
            let Some(value) =
                source_optional_value(stream, line_num, params, "PWL", "value", has_paren)?
            else {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: "PWL requires complete time/value pairs".to_string(),
                });
            };
            points.push((time, value));
        } else {
            break;
        }
    }

    close_source_args(stream, line_num, "PWL", has_paren)?;

    if points.is_empty() {
        points.push((0.0, 0.0));
    }

    Ok(SourceSpec::Pwl { points })
}

fn source_value_or_default(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    source_name: &str,
    arg_name: &str,
    has_paren: bool,
    default: Value,
) -> Result<Value, ParseError> {
    Ok(
        source_optional_value(stream, line_num, params, source_name, arg_name, has_paren)?
            .unwrap_or(default),
    )
}

fn source_optional_value(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    source_name: &str,
    arg_name: &str,
    has_paren: bool,
) -> Result<Option<Value>, ParseError> {
    skip_commas(stream);
    if source_numeric_args_end(stream, has_paren) {
        return Ok(None);
    }

    let found = stream.peek().kind.to_string();
    let value = expect_value(stream, line_num, params).map_err(|err| ParseError::Syntax {
        line: line_num,
        message: format!(
            "{} {} expected numeric value, found {} ({})",
            source_name, arg_name, found, err
        ),
    })?;
    if !value.is_finite() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("{} {} must be finite, got {}", source_name, arg_name, value),
        });
    }
    Ok(Some(value))
}

fn expect_finite_source_value(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    source_name: &str,
    arg_name: &str,
) -> Result<Value, ParseError> {
    let value = expect_value(stream, line_num, params)?;
    if !value.is_finite() {
        return Err(non_finite_source_value_error(
            line_num,
            source_name,
            arg_name,
            value,
        ));
    }
    Ok(value)
}

fn non_finite_source_value_error(
    line_num: usize,
    source_name: &str,
    arg_name: &str,
    value: Value,
) -> ParseError {
    ParseError::Syntax {
        line: line_num,
        message: format!("{source_name} {arg_name} must be finite, got {value}"),
    }
}

fn validate_pwl_file_scaling(
    line_num: usize,
    time_scale: Value,
    value_scale: Value,
    time_offset: Value,
    value_offset: Value,
) -> Result<(), ParseError> {
    if !time_scale.is_finite() || time_scale <= 0.0 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "PWL FILE TSCALE must be finite and positive".to_string(),
        });
    }
    for (name, value) in [
        ("VSCALE", value_scale),
        ("TOFFSET", time_offset),
        ("VOFFSET", value_offset),
    ] {
        if !value.is_finite() {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("PWL FILE {name} must be finite"),
            });
        }
    }
    Ok(())
}

fn close_source_args(
    stream: &mut TokenStream,
    line_num: usize,
    source_name: &str,
    has_paren: bool,
) -> Result<(), ParseError> {
    skip_commas(stream);
    if !has_paren {
        return Ok(());
    }
    if stream.consume(&TokenKind::RParen) {
        return Ok(());
    }
    Err(ParseError::Syntax {
        line: line_num,
        message: format!("{} expected ')' before {}", source_name, stream.peek().kind),
    })
}

fn source_args_end(stream: &TokenStream, has_paren: bool) -> bool {
    match &stream.peek().kind {
        TokenKind::RParen | TokenKind::Newline | TokenKind::Eof => true,
        TokenKind::Ident(keyword)
            if !has_paren
                && (keyword.eq_ignore_ascii_case("AC")
                    || keyword.eq_ignore_ascii_case("DISTOF1")
                    || keyword.eq_ignore_ascii_case("DISTOF2")) =>
        {
            true
        }
        TokenKind::Ident(keyword) if !has_paren && keyword.eq_ignore_ascii_case("DC") => {
            !matches!(stream.peek_n(1).kind, TokenKind::Equals)
        }
        _ => false,
    }
}

fn source_numeric_args_end(stream: &TokenStream, has_paren: bool) -> bool {
    match &stream.peek().kind {
        TokenKind::RParen | TokenKind::Newline | TokenKind::Eof => true,
        TokenKind::Ident(keyword)
            if !has_paren
                && (keyword.eq_ignore_ascii_case("AC")
                    || keyword.eq_ignore_ascii_case("DC")
                    || keyword.eq_ignore_ascii_case("DISTOF1")
                    || keyword.eq_ignore_ascii_case("DISTOF2")) =>
        {
            true
        }
        _ => false,
    }
}

/// Parse EXP(V1 V2 TD1 TAU1 TD2 TAU2); omitted timing parameters stay NaN
/// so the transient runtime can resolve ngspice's tstep-based defaults
/// (TD1/TAU1/TAU2 default to TSTEP, TD2 to TD1+TSTEP).
fn parse_exp_spec(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<SourceSpec, ParseError> {
    let has_paren = stream.consume(&TokenKind::LParen);

    let v1 = source_value_or_default(stream, line_num, params, "EXP", "V1", has_paren, 0.0)?;
    let v2 = source_value_or_default(stream, line_num, params, "EXP", "V2", has_paren, 1.0)?;
    let td1 = source_optional_value(stream, line_num, params, "EXP", "TD1", has_paren)?
        .unwrap_or(Value::NAN);
    let tau1 = source_optional_value(stream, line_num, params, "EXP", "TAU1", has_paren)?
        .unwrap_or(Value::NAN);
    let td2 = source_optional_value(stream, line_num, params, "EXP", "TD2", has_paren)?
        .unwrap_or(Value::NAN);
    let tau2 = source_optional_value(stream, line_num, params, "EXP", "TAU2", has_paren)?
        .unwrap_or(Value::NAN);

    close_source_args(stream, line_num, "EXP", has_paren)?;

    Ok(SourceSpec::Exp {
        v1,
        v2,
        td1,
        tau1,
        td2,
        tau2,
    })
}
