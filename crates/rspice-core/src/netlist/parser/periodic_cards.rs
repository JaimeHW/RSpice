//! Authored cards for the periodic large-signal analysis family:
//! `.PSS`, `.PAC`, `.PNOISE` and `.ENVELOPE`.
//!
//! Every card is fully validated here, so the analysis layer converts the AST
//! rather than re-deriving what the deck asked for. A field another simulator
//! accepts but RSpice cannot honour is refused with a source-located
//! [`AnalysisCardError`] rather than parsed and dropped.

use super::*;

//=============================================================================
// Shared card scanning
//=============================================================================

/// Build the typed card failure for a card, line and issue.
fn card_error(card: AnalysisCard, line: usize, issue: AnalysisCardIssue) -> ParseError {
    ParseError::AnalysisCard(Box::new(AnalysisCardError::new(card, line, issue)))
}

/// Whether the stream is positioned at the end of the card's logical line.
fn at_card_end(stream: &TokenStream) -> bool {
    matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof)
}

/// Whether the next tokens spell `IDENT =`.
///
/// This single rule keeps the positional and keyword forms disjoint: a
/// positional field is never followed by `=`, and a keyword always is.
fn at_keyword(stream: &TokenStream) -> bool {
    matches!(stream.peek().kind, TokenKind::Ident(_))
        && matches!(stream.peek_n(1).kind, TokenKind::Equals)
}

/// Consume `KEYWORD =` and return the upper-cased keyword, or leave the
/// stream untouched when it is not positioned on a keyword pair.
fn take_keyword(stream: &mut TokenStream) -> Option<String> {
    if !at_keyword(stream) {
        return None;
    }
    let TokenKind::Ident(name) = &stream.peek().kind else {
        return None;
    };
    let keyword = name.to_ascii_uppercase();
    stream.advance();
    stream.advance();
    Some(keyword)
}

/// Reject a keyword the card has already bound.
fn bind_once<T>(
    slot: &mut Option<T>,
    value: T,
    card: AnalysisCard,
    line: usize,
    keyword: &'static str,
) -> Result<(), ParseError> {
    if slot.is_some() {
        return Err(card_error(
            card,
            line,
            AnalysisCardIssue::DuplicateKeyword { keyword },
        ));
    }
    *slot = Some(value);
    Ok(())
}

/// Read a numeric field and check it against a predicate.
fn card_number(
    stream: &mut TokenStream,
    line: usize,
    params: &ParamContext,
    card: AnalysisCard,
    field: &'static str,
    expected: &'static str,
    admissible: impl Fn(Value) -> bool,
) -> Result<Value, ParseError> {
    let value = expect_value(stream, line, params)
        .map_err(|_| card_error(card, line, AnalysisCardIssue::MissingField { field }))?;
    if !value.is_finite() || !admissible(value) {
        return Err(card_error(
            card,
            line,
            AnalysisCardIssue::InvalidNumber {
                field,
                value,
                expected,
            },
        ));
    }
    Ok(value)
}

/// Read a non-negative integer-valued field.
fn card_count(
    stream: &mut TokenStream,
    line: usize,
    params: &ParamContext,
    card: AnalysisCard,
    field: &'static str,
    minimum: usize,
) -> Result<usize, ParseError> {
    let expected: &'static str = match minimum {
        1 => "a whole number >= 1",
        16 => "a whole number >= 16",
        _ => "a whole number",
    };
    let value = card_number(stream, line, params, card, field, expected, |value| {
        value >= 0.0 && value.fract() == 0.0 && value <= usize::MAX as Value
    })?;
    let count = value as usize;
    if count < minimum {
        return Err(card_error(
            card,
            line,
            AnalysisCardIssue::InvalidNumber {
                field,
                value,
                expected,
            },
        ));
    }
    Ok(count)
}

/// Read a signed integer-valued field.
fn card_signed(
    stream: &mut TokenStream,
    line: usize,
    params: &ParamContext,
    card: AnalysisCard,
    field: &'static str,
    minimum: i32,
) -> Result<i32, ParseError> {
    let expected: &'static str = if minimum >= 1 {
        "a whole number >= 1"
    } else {
        "a whole number"
    };
    let value = card_number(stream, line, params, card, field, expected, |value| {
        value.fract() == 0.0 && value >= f64::from(i32::MIN) && value <= f64::from(i32::MAX)
    })?;
    let signed = value as i32;
    if signed < minimum {
        return Err(card_error(
            card,
            line,
            AnalysisCardIssue::InvalidNumber {
                field,
                value,
                expected,
            },
        ));
    }
    Ok(signed)
}

/// Read a boolean-valued field written as `TRUE|FALSE|YES|NO|1|0`.
fn card_bool(
    stream: &mut TokenStream,
    line: usize,
    card: AnalysisCard,
    field: &'static str,
) -> Result<bool, ParseError> {
    let token = stream.peek().clone();
    let spelling = match &token.kind {
        TokenKind::Ident(name) => name.clone(),
        TokenKind::Number(value) if *value == 0.0 || *value == 1.0 => {
            stream.advance();
            return Ok(*value == 1.0);
        }
        _ => token.lexeme.clone(),
    };
    let decoded = match spelling.to_ascii_uppercase().as_str() {
        "TRUE" | "YES" | "ON" => Some(true),
        "FALSE" | "NO" | "OFF" => Some(false),
        _ => None,
    };
    match decoded {
        Some(value) => {
            stream.advance();
            Ok(value)
        }
        None => Err(card_error(
            card,
            line,
            AnalysisCardIssue::InvalidChoice {
                field,
                value: spelling,
                expected: "TRUE, FALSE, YES, NO, 1 or 0",
            },
        )),
    }
}

/// Read a bare name field (node, source or integration method).
fn card_name(
    stream: &mut TokenStream,
    line: usize,
    card: AnalysisCard,
    field: &'static str,
) -> Result<String, ParseError> {
    let name = expect_node(stream, line)
        .map_err(|_| card_error(card, line, AnalysisCardIssue::MissingField { field }))?;
    if name.trim().is_empty() {
        return Err(card_error(
            card,
            line,
            AnalysisCardIssue::InvalidName { field, value: name },
        ));
    }
    Ok(name)
}

/// Read the shared `DEC|LIN|OCT np fstart fstop` sweep spec.
fn card_sweep(
    stream: &mut TokenStream,
    line: usize,
    params: &ParamContext,
    card: AnalysisCard,
) -> Result<PeriodicSweep, ParseError> {
    let spelling = expect_ident(stream, line).map_err(|_| {
        card_error(
            card,
            line,
            AnalysisCardIssue::MissingField { field: "variation" },
        )
    })?;
    let variation = match spelling.to_ascii_uppercase().as_str() {
        "LIN" => FreqVariation::Lin,
        "OCT" => FreqVariation::Oct,
        "DEC" => FreqVariation::Dec,
        _ => {
            return Err(card_error(
                card,
                line,
                AnalysisCardIssue::InvalidChoice {
                    field: "variation",
                    value: spelling,
                    expected: "DEC, LIN or OCT",
                },
            ));
        }
    };
    let points = card_count(stream, line, params, card, "np", 1)?;
    let start_freq = card_number(
        stream,
        line,
        params,
        card,
        "fstart",
        "a positive frequency in Hz",
        |value| value > 0.0,
    )?;
    let stop_freq = card_number(
        stream,
        line,
        params,
        card,
        "fstop",
        "a positive frequency in Hz",
        |value| value > 0.0,
    )?;
    if stop_freq < start_freq {
        return Err(card_error(
            card,
            line,
            AnalysisCardIssue::InvalidNumber {
                field: "fstop",
                value: stop_freq,
                expected: "a frequency at or above fstart",
            },
        ));
    }
    Ok(PeriodicSweep {
        variation,
        points,
        start_freq,
        stop_freq,
    })
}

/// Read `FROM=PSS|HB`.
fn card_source_selector(
    stream: &mut TokenStream,
    line: usize,
    card: AnalysisCard,
) -> Result<PeriodicSourceSelector, ParseError> {
    let spelling = card_name(stream, line, card, "FROM")?;
    match spelling.to_ascii_uppercase().as_str() {
        "PSS" => Ok(PeriodicSourceSelector::Pss),
        "HB" => Ok(PeriodicSourceSelector::Hb),
        _ => Err(card_error(
            card,
            line,
            AnalysisCardIssue::InvalidChoice {
                field: "FROM",
                value: spelling,
                expected: "PSS or HB",
            },
        )),
    }
}

/// Read an output probe written `V(node[,ref])` or as a bare node name.
fn card_output_probe(
    stream: &mut TokenStream,
    line: usize,
    card: AnalysisCard,
) -> Result<(String, Option<String>), ParseError> {
    parse_voltage_output_reference(stream, line).map_err(|_| {
        card_error(
            card,
            line,
            AnalysisCardIssue::InvalidChoice {
                field: "OUT",
                value: stream.peek().lexeme.clone(),
                expected: "V(node), V(node,ref) or a bare node name",
            },
        )
    })
}

//=============================================================================
// .PSS
//=============================================================================

/// Parse `.PSS`.
///
/// Two disjoint forms are accepted:
///
/// * ngspice's positional oscillator form
///   `.PSS gfreq tstab oscnode psspoints harms sciter [KEY=VALUE ...]`
/// * a pure keyword form `.PSS KEY=VALUE ...`
///
/// ngspice's trailing `steadycoeff` and `uic` fields are refused: RSpice's
/// shooting solver has neither an ngspice steady-coefficient threshold nor a
/// user-initial-condition startup mode, and accepting them would silently
/// change what the deck asked for.
pub(super) fn parse_pss_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<AnalysisCommand, ParseError> {
    const CARD: AnalysisCard = AnalysisCard::Pss;

    let positional = !at_card_end(stream) && !at_keyword(stream);
    let mut positional_fields: Vec<&'static str> = Vec::new();
    let mut card = if positional {
        let gfreq = card_number(
            stream,
            line_num,
            params,
            CARD,
            "gfreq",
            "a positive frequency in Hz",
            |value| value > 0.0,
        )?;
        let tstab = card_number(
            stream,
            line_num,
            params,
            CARD,
            "tstab",
            "a non-negative time in seconds",
            |value| value >= 0.0,
        )?;
        let oscnode = card_name(stream, line_num, CARD, "oscnode")?;
        let points = card_count(stream, line_num, params, CARD, "psspoints", 16)?;
        let harmonics = card_count(stream, line_num, params, CARD, "harms", 1)?;
        let iterations = card_count(stream, line_num, params, CARD, "sciter", 1)?;

        // The positional form is ngspice's autonomous oscillator card: it
        // names the node the period is detected on, so period detection is on.
        let mut card = PssCard::autonomous();
        card.fundamental_freq = gfreq;
        card.period_guess = 1.0 / gfreq;
        card.tstab = tstab;
        card.points_per_period = points;
        card.num_harmonics = harmonics;
        card.max_iterations = iterations;
        card.oscillator_node = Some(oscnode);
        positional_fields.extend([
            "FUND",
            "PERIODGUESS",
            "TSTAB",
            "OSCNODE",
            "POINTS",
            "HARMS",
            "MAXITER",
            "AUTONOMOUS",
        ]);
        card
    } else {
        // Replaced below once the keywords say whether the card is driven.
        PssCard::driven(0.0)
    };

    let mut fund = None;
    let mut period_guess = None;
    let mut autonomous = None;
    let mut oscillator_node = None;
    let mut harmonics = None;
    let mut tstab = None;
    let mut tstab_periods = None;
    let mut max_iterations = None;
    let mut tolerance = None;
    let mut abstol = None;
    let mut damping = None;
    let mut max_period_change = None;
    let mut points = None;
    let mut method = None;
    let mut verbose = None;

    loop {
        skip_commas(stream);
        if at_card_end(stream) {
            break;
        }
        let Some(keyword) = take_keyword(stream) else {
            return Err(unhonourable_pss_tail(stream, line_num, positional));
        };
        if let Some(field) = positional_fields
            .iter()
            .find(|field| **field == keyword.as_str())
        {
            return Err(card_error(
                CARD,
                line_num,
                AnalysisCardIssue::ConflictingFields {
                    first: "the positional field list",
                    second: field,
                },
            ));
        }
        match keyword.as_str() {
            "FUND" => bind_once(
                &mut fund,
                card_number(
                    stream,
                    line_num,
                    params,
                    CARD,
                    "FUND",
                    "a positive frequency in Hz",
                    |value| value > 0.0,
                )?,
                CARD,
                line_num,
                "FUND",
            )?,
            "PERIODGUESS" => bind_once(
                &mut period_guess,
                card_number(
                    stream,
                    line_num,
                    params,
                    CARD,
                    "PERIODGUESS",
                    "a positive period in seconds",
                    |value| value > 0.0,
                )?,
                CARD,
                line_num,
                "PERIODGUESS",
            )?,
            "AUTONOMOUS" => bind_once(
                &mut autonomous,
                card_bool(stream, line_num, CARD, "AUTONOMOUS")?,
                CARD,
                line_num,
                "AUTONOMOUS",
            )?,
            "OSCNODE" => bind_once(
                &mut oscillator_node,
                card_name(stream, line_num, CARD, "OSCNODE")?,
                CARD,
                line_num,
                "OSCNODE",
            )?,
            "HARMS" => bind_once(
                &mut harmonics,
                card_count(stream, line_num, params, CARD, "HARMS", 1)?,
                CARD,
                line_num,
                "HARMS",
            )?,
            "TSTAB" => bind_once(
                &mut tstab,
                card_number(
                    stream,
                    line_num,
                    params,
                    CARD,
                    "TSTAB",
                    "a non-negative time in seconds",
                    |value| value >= 0.0,
                )?,
                CARD,
                line_num,
                "TSTAB",
            )?,
            "TSTABPERIODS" => bind_once(
                &mut tstab_periods,
                card_count(stream, line_num, params, CARD, "TSTABPERIODS", 1)?,
                CARD,
                line_num,
                "TSTABPERIODS",
            )?,
            "MAXITER" => bind_once(
                &mut max_iterations,
                card_count(stream, line_num, params, CARD, "MAXITER", 1)?,
                CARD,
                line_num,
                "MAXITER",
            )?,
            "TOL" => bind_once(
                &mut tolerance,
                card_number(
                    stream,
                    line_num,
                    params,
                    CARD,
                    "TOL",
                    "a positive relative tolerance",
                    |value| value > 0.0,
                )?,
                CARD,
                line_num,
                "TOL",
            )?,
            "ABSTOL" => bind_once(
                &mut abstol,
                card_number(
                    stream,
                    line_num,
                    params,
                    CARD,
                    "ABSTOL",
                    "a positive absolute tolerance",
                    |value| value > 0.0,
                )?,
                CARD,
                line_num,
                "ABSTOL",
            )?,
            "DAMPING" => bind_once(
                &mut damping,
                card_number(
                    stream,
                    line_num,
                    params,
                    CARD,
                    "DAMPING",
                    "a Newton damping factor in [0.1, 1.0]",
                    |value| (0.1..=1.0).contains(&value),
                )?,
                CARD,
                line_num,
                "DAMPING",
            )?,
            "MAXPERIODCHANGE" => bind_once(
                &mut max_period_change,
                card_number(
                    stream,
                    line_num,
                    params,
                    CARD,
                    "MAXPERIODCHANGE",
                    "a positive relative period bound",
                    |value| value > 0.0,
                )?,
                CARD,
                line_num,
                "MAXPERIODCHANGE",
            )?,
            "POINTS" => bind_once(
                &mut points,
                card_count(stream, line_num, params, CARD, "POINTS", 16)?,
                CARD,
                line_num,
                "POINTS",
            )?,
            "METHOD" => {
                let spelling = card_name(stream, line_num, CARD, "METHOD")?;
                let decoded = crate::numerics::integration::parse_integration_method(&spelling)
                    .ok_or_else(|| {
                        card_error(
                            CARD,
                            line_num,
                            AnalysisCardIssue::InvalidChoice {
                                field: "METHOD",
                                value: spelling.clone(),
                                expected: "TRAP, GEAR, EULER or TRAPGEAR",
                            },
                        )
                    })?;
                bind_once(&mut method, decoded, CARD, line_num, "METHOD")?
            }
            "VERBOSE" => bind_once(
                &mut verbose,
                card_bool(stream, line_num, CARD, "VERBOSE")?,
                CARD,
                line_num,
                "VERBOSE",
            )?,
            _ => {
                return Err(card_error(
                    CARD,
                    line_num,
                    AnalysisCardIssue::UnknownKeyword { keyword },
                ));
            }
        }
    }

    if fund.is_some() && period_guess.is_some() {
        return Err(card_error(
            CARD,
            line_num,
            AnalysisCardIssue::ConflictingFields {
                first: "FUND",
                second: "PERIODGUESS",
            },
        ));
    }

    if !positional {
        let is_autonomous = autonomous.unwrap_or(false) || oscillator_node.is_some();
        if autonomous == Some(false) && oscillator_node.is_some() {
            return Err(card_error(
                CARD,
                line_num,
                AnalysisCardIssue::ConflictingFields {
                    first: "OSCNODE",
                    second: "AUTONOMOUS=FALSE",
                },
            ));
        }
        card = if is_autonomous {
            let mut card = PssCard::autonomous();
            if let Some(period) = period_guess {
                card.period_guess = period;
                card.fundamental_freq = 1.0 / period;
            } else if let Some(frequency) = fund {
                card.period_guess = 1.0 / frequency;
                card.fundamental_freq = frequency;
            }
            card
        } else {
            if period_guess.is_some() {
                return Err(card_error(
                    CARD,
                    line_num,
                    AnalysisCardIssue::ConflictingFields {
                        first: "PERIODGUESS",
                        second: "the driven form; author AUTONOMOUS=TRUE",
                    },
                ));
            }
            let Some(frequency) = fund else {
                return Err(card_error(
                    CARD,
                    line_num,
                    AnalysisCardIssue::MissingField { field: "FUND" },
                ));
            };
            PssCard::driven(frequency)
        };
        card.auto_period = is_autonomous;
        card.oscillator_node = oscillator_node;
    }

    if let Some(value) = harmonics {
        card.num_harmonics = value;
    }
    if let Some(value) = tstab {
        card.tstab = value;
    }
    if let Some(value) = tstab_periods {
        card.tstab_periods = value;
    }
    if let Some(value) = max_iterations {
        card.max_iterations = value;
    }
    if let Some(value) = tolerance {
        card.tolerance = value;
    }
    if let Some(value) = abstol {
        card.abstol = value;
    }
    if let Some(value) = damping {
        card.damping_factor = value;
    }
    if let Some(value) = max_period_change {
        card.max_period_change = value;
    }
    if let Some(value) = points {
        card.points_per_period = value;
    }
    if let Some(value) = method {
        card.integration_method = Some(value);
    }
    if let Some(value) = verbose {
        card.verbose = value;
    }

    // Both remaining rules span two fields, so they cannot be checked while
    // reading one. Every other constraint the shooting configuration states is
    // per-field and was enforced as each field was read.
    if card
        .num_harmonics
        .checked_mul(2)
        .is_none_or(|samples| samples > card.points_per_period)
    {
        return Err(card_error(
            CARD,
            line_num,
            AnalysisCardIssue::InvalidNumber {
                field: "POINTS",
                value: card.points_per_period as Value,
                expected: "at least twice HARMS, so the retained harmonics do not alias",
            },
        ));
    }
    if !card.effective_tstab().is_finite() {
        return Err(card_error(
            CARD,
            line_num,
            AnalysisCardIssue::InvalidNumber {
                field: "TSTABPERIODS",
                value: card.tstab_periods as Value,
                expected: "small enough that TSTABPERIODS periods is a finite time",
            },
        ));
    }

    Ok(AnalysisCommand::Pss(Box::new(card)))
}

/// Diagnose the ngspice `.pss` tail fields RSpice cannot honour.
fn unhonourable_pss_tail(stream: &TokenStream, line_num: usize, positional: bool) -> ParseError {
    match &stream.peek().kind {
        // Only the positional form has a seventh field, so only there is a
        // stray number ngspice's steady coefficient rather than a typo.
        TokenKind::Number(_) if positional => card_error(
            AnalysisCard::Pss,
            line_num,
            AnalysisCardIssue::UnhonourableField {
                field: "steadycoeff",
                detail: "the shooting solver converges on a relative periodicity norm, \
                         not an ngspice per-node steady coefficient; author TOL= and ABSTOL= instead",
            },
        ),
        TokenKind::Ident(name) if name.eq_ignore_ascii_case("uic") => card_error(
            AnalysisCard::Pss,
            line_num,
            AnalysisCardIssue::UnhonourableField {
                field: "uic",
                detail: "the shooting solver always starts its stabilization run from the \
                         operating point; author TSTAB= or TSTABPERIODS= instead",
            },
        ),
        _ => card_error(
            AnalysisCard::Pss,
            line_num,
            AnalysisCardIssue::TrailingToken {
                token: stream.peek().lexeme.clone(),
            },
        ),
    }
}

//=============================================================================
// .PAC
//=============================================================================

/// Parse `.PAC DEC|LIN|OCT np fstart fstop KEY=VALUE ...`.
pub(super) fn parse_pac_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<AnalysisCommand, ParseError> {
    const CARD: AnalysisCard = AnalysisCard::Pac;

    let sweep = card_sweep(stream, line_num, params, CARD)?;

    let mut input_source = None;
    let mut output = None;
    let mut max_sideband = None;
    let mut sideband_min = None;
    let mut sideband_max = None;
    let mut reltol = None;
    let mut abstol = None;
    let mut source = None;

    loop {
        skip_commas(stream);
        if at_card_end(stream) {
            break;
        }
        let Some(keyword) = take_keyword(stream) else {
            return Err(card_error(
                CARD,
                line_num,
                AnalysisCardIssue::TrailingToken {
                    token: stream.peek().lexeme.clone(),
                },
            ));
        };
        match keyword.as_str() {
            "INPUT" => bind_once(
                &mut input_source,
                card_name(stream, line_num, CARD, "INPUT")?,
                CARD,
                line_num,
                "INPUT",
            )?,
            "OUT" => bind_once(
                &mut output,
                card_output_probe(stream, line_num, CARD)?,
                CARD,
                line_num,
                "OUT",
            )?,
            "MAXSIDEBAND" => bind_once(
                &mut max_sideband,
                card_signed(stream, line_num, params, CARD, "MAXSIDEBAND", 0)?,
                CARD,
                line_num,
                "MAXSIDEBAND",
            )?,
            "SIDEBANDMIN" => bind_once(
                &mut sideband_min,
                card_signed(stream, line_num, params, CARD, "SIDEBANDMIN", i32::MIN)?,
                CARD,
                line_num,
                "SIDEBANDMIN",
            )?,
            "SIDEBANDMAX" => bind_once(
                &mut sideband_max,
                card_signed(stream, line_num, params, CARD, "SIDEBANDMAX", i32::MIN)?,
                CARD,
                line_num,
                "SIDEBANDMAX",
            )?,
            "RELTOL" => bind_once(
                &mut reltol,
                card_number(
                    stream,
                    line_num,
                    params,
                    CARD,
                    "RELTOL",
                    "a positive relative tolerance",
                    |value| value > 0.0,
                )?,
                CARD,
                line_num,
                "RELTOL",
            )?,
            "ABSTOL" => bind_once(
                &mut abstol,
                card_number(
                    stream,
                    line_num,
                    params,
                    CARD,
                    "ABSTOL",
                    "a positive absolute tolerance",
                    |value| value > 0.0,
                )?,
                CARD,
                line_num,
                "ABSTOL",
            )?,
            "FROM" => bind_once(
                &mut source,
                card_source_selector(stream, line_num, CARD)?,
                CARD,
                line_num,
                "FROM",
            )?,
            _ => {
                return Err(card_error(
                    CARD,
                    line_num,
                    AnalysisCardIssue::UnknownKeyword { keyword },
                ));
            }
        }
    }

    if max_sideband.is_some() && (sideband_min.is_some() || sideband_max.is_some()) {
        return Err(card_error(
            CARD,
            line_num,
            AnalysisCardIssue::ConflictingFields {
                first: "MAXSIDEBAND",
                second: "SIDEBANDMIN/SIDEBANDMAX",
            },
        ));
    }
    let Some(input_source) = input_source else {
        return Err(card_error(
            CARD,
            line_num,
            AnalysisCardIssue::MissingField { field: "INPUT" },
        ));
    };
    let Some((output_node, output_ref)) = output else {
        return Err(card_error(
            CARD,
            line_num,
            AnalysisCardIssue::MissingField { field: "OUT" },
        ));
    };

    let (sideband_min, sideband_max) = match max_sideband {
        Some(bound) => (-bound, bound),
        None => (
            sideband_min.unwrap_or(PacCard::DEFAULT_SIDEBAND_MIN),
            sideband_max.unwrap_or(PacCard::DEFAULT_SIDEBAND_MAX),
        ),
    };
    if sideband_min > sideband_max {
        return Err(card_error(
            CARD,
            line_num,
            AnalysisCardIssue::ConflictingFields {
                first: "SIDEBANDMIN",
                second: "SIDEBANDMAX",
            },
        ));
    }

    Ok(AnalysisCommand::Pac(Box::new(PacCard {
        sweep,
        input_source: input_source.to_ascii_uppercase(),
        output_node: output_node.to_ascii_uppercase(),
        output_ref: output_ref.map(|node| node.to_ascii_uppercase()),
        sideband_min,
        sideband_max,
        reltol: reltol.unwrap_or(PacCard::DEFAULT_RELTOL),
        abstol: abstol.unwrap_or(PacCard::DEFAULT_ABSTOL),
        source: source.unwrap_or_default(),
    })))
}

//=============================================================================
// .PNOISE
//=============================================================================

/// Default folded sideband bound, matching the direct periodic-noise API.
const PNOISE_DEFAULT_MAX_SIDEBAND: i32 = 6;

/// Parse `.PNOISE DEC|LIN|OCT np fstart fstop KEY=VALUE ...`.
///
/// The sweep is the offset-frequency grid; the folded sideband bound and the
/// output probe follow the driven periodic-noise entry point.
pub(super) fn parse_pnoise_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<AnalysisCommand, ParseError> {
    const CARD: AnalysisCard = AnalysisCard::Pnoise;

    let sweep = card_sweep(stream, line_num, params, CARD)?;

    let mut output = None;
    let mut input_source = None;
    let mut max_sideband = None;
    let mut source = None;

    loop {
        skip_commas(stream);
        if at_card_end(stream) {
            break;
        }
        let Some(keyword) = take_keyword(stream) else {
            return Err(card_error(
                CARD,
                line_num,
                AnalysisCardIssue::TrailingToken {
                    token: stream.peek().lexeme.clone(),
                },
            ));
        };
        match keyword.as_str() {
            "OUT" => bind_once(
                &mut output,
                card_output_probe(stream, line_num, CARD)?,
                CARD,
                line_num,
                "OUT",
            )?,
            "INPUT" => bind_once(
                &mut input_source,
                card_name(stream, line_num, CARD, "INPUT")?,
                CARD,
                line_num,
                "INPUT",
            )?,
            "MAXSIDEBAND" => bind_once(
                &mut max_sideband,
                card_signed(stream, line_num, params, CARD, "MAXSIDEBAND", 1)?,
                CARD,
                line_num,
                "MAXSIDEBAND",
            )?,
            "FROM" => bind_once(
                &mut source,
                card_source_selector(stream, line_num, CARD)?,
                CARD,
                line_num,
                "FROM",
            )?,
            _ => {
                return Err(card_error(
                    CARD,
                    line_num,
                    AnalysisCardIssue::UnknownKeyword { keyword },
                ));
            }
        }
    }

    let Some((output_node, reference_node)) = output else {
        return Err(card_error(
            CARD,
            line_num,
            AnalysisCardIssue::MissingField { field: "OUT" },
        ));
    };

    Ok(AnalysisCommand::Pnoise(Box::new(PnoiseCard {
        sweep,
        output_node: output_node.to_ascii_uppercase(),
        reference_node: reference_node.map(|node| node.to_ascii_uppercase()),
        input_source: input_source.map(|name| name.to_ascii_uppercase()),
        max_sideband: max_sideband.unwrap_or(PNOISE_DEFAULT_MAX_SIDEBAND),
        source: source.unwrap_or_default(),
    })))
}

//=============================================================================
// .ENVELOPE
//=============================================================================

/// Divisor the continuation entry points use when no maximum step is authored.
const ENVELOPE_DEFAULT_STEP_DIVISOR: Value = 50.0;

/// Parse `.ENVELOPE TSTOP=<t> [MAXSTEP=<t>] [FREEZE=(<src>[,<src>...])]`.
pub(super) fn parse_envelope_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<AnalysisCommand, ParseError> {
    const CARD: AnalysisCard = AnalysisCard::Envelope;

    let mut duration = None;
    let mut max_step = None;
    let mut frozen_sources = None;

    loop {
        skip_commas(stream);
        if at_card_end(stream) {
            break;
        }
        let Some(keyword) = take_keyword(stream) else {
            return Err(card_error(
                CARD,
                line_num,
                AnalysisCardIssue::TrailingToken {
                    token: stream.peek().lexeme.clone(),
                },
            ));
        };
        match keyword.as_str() {
            "TSTOP" => bind_once(
                &mut duration,
                card_number(
                    stream,
                    line_num,
                    params,
                    CARD,
                    "TSTOP",
                    "a positive continuation length in seconds",
                    |value| value > 0.0,
                )?,
                CARD,
                line_num,
                "TSTOP",
            )?,
            "MAXSTEP" => bind_once(
                &mut max_step,
                card_number(
                    stream,
                    line_num,
                    params,
                    CARD,
                    "MAXSTEP",
                    "a positive timestep in seconds",
                    |value| value > 0.0,
                )?,
                CARD,
                line_num,
                "MAXSTEP",
            )?,
            "FREEZE" => bind_once(
                &mut frozen_sources,
                card_source_list(stream, line_num, CARD)?,
                CARD,
                line_num,
                "FREEZE",
            )?,
            _ => {
                return Err(card_error(
                    CARD,
                    line_num,
                    AnalysisCardIssue::UnknownKeyword { keyword },
                ));
            }
        }
    }

    let Some(duration) = duration else {
        return Err(card_error(
            CARD,
            line_num,
            AnalysisCardIssue::MissingField { field: "TSTOP" },
        ));
    };
    let max_step = max_step.unwrap_or(duration / ENVELOPE_DEFAULT_STEP_DIVISOR);
    if !max_step.is_finite() || max_step <= 0.0 {
        return Err(card_error(
            CARD,
            line_num,
            AnalysisCardIssue::InvalidNumber {
                field: "MAXSTEP",
                value: max_step,
                expected: "a positive timestep in seconds",
            },
        ));
    }

    Ok(AnalysisCommand::Envelope(Box::new(EnvelopeCard {
        duration,
        max_step,
        frozen_sources: frozen_sources.unwrap_or_default(),
    })))
}

/// Read `(<name>[,<name>...])` or a single bare `<name>`.
fn card_source_list(
    stream: &mut TokenStream,
    line: usize,
    card: AnalysisCard,
) -> Result<Vec<String>, ParseError> {
    let mut names = Vec::new();
    if !stream.consume(&TokenKind::LParen) {
        names.push(card_name(stream, line, card, "FREEZE")?);
    } else {
        loop {
            names.push(card_name(stream, line, card, "FREEZE")?);
            if stream.consume(&TokenKind::RParen) {
                break;
            }
            if !stream.consume(&TokenKind::Comma) {
                return Err(card_error(
                    card,
                    line,
                    AnalysisCardIssue::TrailingToken {
                        token: stream.peek().lexeme.clone(),
                    },
                ));
            }
        }
    }
    let mut seen = std::collections::BTreeSet::new();
    for name in &names {
        if !seen.insert(name.to_ascii_lowercase()) {
            return Err(card_error(
                card,
                line,
                AnalysisCardIssue::InvalidName {
                    field: "FREEZE",
                    value: name.clone(),
                },
            ));
        }
    }
    Ok(names)
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use crate::netlist::{
        AnalysisCard, AnalysisCardIssue, AnalysisCommand, EnvelopeCard, FreqVariation, Netlist,
        PacCard, ParseError, PeriodicSourceSelector, PnoiseCard, PssCard,
    };

    const CIRCUIT: &str = "periodic card parser\n\
                           V1 in 0 SIN(0 1 1G)\n\
                           R1 in out 1k\n\
                           C1 out 0 1p\n";

    fn deck(card: &str) -> String {
        format!("{CIRCUIT}{card}\n.END\n")
    }

    fn parse_one(card: &str) -> AnalysisCommand {
        let netlist = Netlist::parse(&deck(card)).expect("card parses");
        let mut analyses = netlist.analyses;
        assert_eq!(analyses.len(), 1, "expected one analysis: {analyses:?}");
        analyses.remove(0)
    }

    fn card_failure(card: &str) -> (AnalysisCard, usize, AnalysisCardIssue) {
        let error = Netlist::parse(&deck(card)).expect_err("malformed card must be refused");
        match error {
            ParseError::AnalysisCard(error) => (error.card, error.line, error.issue.clone()),
            other => panic!("expected a typed analysis-card error, got {other:?}"),
        }
    }

    fn pss(card: &str) -> Box<PssCard> {
        match parse_one(card) {
            AnalysisCommand::Pss(card) => card,
            other => panic!("expected .PSS, got {other:?}"),
        }
    }

    fn pac(card: &str) -> Box<PacCard> {
        let netlist = Netlist::parse(&deck(card)).expect("card parses");
        match netlist.analyses.into_iter().next_back() {
            Some(AnalysisCommand::Pac(card)) => card,
            other => panic!("expected .PAC, got {other:?}"),
        }
    }

    fn pnoise(card: &str) -> Box<PnoiseCard> {
        let netlist = Netlist::parse(&deck(card)).expect("card parses");
        match netlist.analyses.into_iter().next_back() {
            Some(AnalysisCommand::Pnoise(card)) => card,
            other => panic!("expected .PNOISE, got {other:?}"),
        }
    }

    fn envelope(card: &str) -> Box<EnvelopeCard> {
        let netlist = Netlist::parse(&deck(card)).expect("card parses");
        match netlist.analyses.into_iter().next_back() {
            Some(AnalysisCommand::Envelope(card)) => card,
            other => panic!("expected .ENVELOPE, got {other:?}"),
        }
    }

    //-------------------------------------------------------------------------
    // .PSS
    //-------------------------------------------------------------------------

    #[test]
    fn pss_positional_form_binds_every_ngspice_field() {
        // ngspice's own oscillator card shape, minus the fields RSpice refuses.
        let card = pss(".pss 3.1e6 500e-6 out 256 10 50");
        assert_eq!(card.fundamental_freq, 3.1e6);
        assert_eq!(card.period_guess, 1.0 / 3.1e6);
        assert_eq!(card.tstab, 500e-6);
        assert_eq!(card.oscillator_node.as_deref(), Some("OUT"));
        assert_eq!(card.points_per_period, 256);
        assert_eq!(card.num_harmonics, 10);
        assert_eq!(card.max_iterations, 50);
        assert!(card.auto_period, "the positional form is autonomous");
        assert!(card.is_autonomous());
        assert_eq!(
            card.tstab_periods,
            PssCard::DEFAULT_AUTONOMOUS_TSTAB_PERIODS
        );
    }

    #[test]
    fn pss_positional_form_accepts_rspice_only_keywords_after_the_fields() {
        let card = pss(".PSS 1e6 0 osc 512 8 40 TOL=1e-9 ABSTOL=1e-14 DAMPING=0.5 METHOD=GEAR");
        assert_eq!(card.tolerance, 1e-9);
        assert_eq!(card.abstol, 1e-14);
        assert_eq!(card.damping_factor, 0.5);
        assert_eq!(
            card.integration_method,
            Some(crate::numerics::integration::IntegrationMethod::Gear2)
        );
    }

    #[test]
    fn pss_keyword_form_defaults_every_optional_field() {
        assert_eq!(*pss(".PSS FUND=1G"), PssCard::driven(1.0e9));
    }

    #[test]
    fn pss_keyword_form_binds_every_configuration_field() {
        let card = pss(
            ".pss fund=2.5g harms=15 tstab=3n tstabperiods=7 maxiter=250 tol=1e-8 \
             abstol=1e-15 damping=0.75 maxperiodchange=0.25 points=1024 method=trap verbose=true",
        );
        assert_eq!(card.fundamental_freq, 2.5e9);
        assert_eq!(card.num_harmonics, 15);
        assert!(
            (card.tstab - 3e-9).abs() <= 1e-24,
            "tstab was {}",
            card.tstab
        );
        assert_eq!(card.tstab_periods, 7);
        assert_eq!(card.max_iterations, 250);
        assert_eq!(card.tolerance, 1e-8);
        assert_eq!(card.abstol, 1e-15);
        assert_eq!(card.damping_factor, 0.75);
        assert_eq!(card.max_period_change, 0.25);
        assert_eq!(card.points_per_period, 1024);
        assert_eq!(
            card.integration_method,
            Some(crate::numerics::integration::IntegrationMethod::Trapezoidal)
        );
        assert!(card.verbose);
        assert!(!card.auto_period);
    }

    #[test]
    fn pss_autonomous_keyword_form_seeds_the_period_from_its_guess() {
        let card = pss(".PSS AUTONOMOUS=TRUE PERIODGUESS=1n OSCNODE=OUT HARMS=12");
        assert!(card.auto_period);
        assert_eq!(card.period_guess, 1e-9);
        assert_eq!(card.fundamental_freq, 1.0 / 1e-9);
        assert_eq!(card.oscillator_node.as_deref(), Some("OUT"));
        assert_eq!(card.num_harmonics, 12);
        assert_eq!(
            card.tstab_periods,
            PssCard::DEFAULT_AUTONOMOUS_TSTAB_PERIODS
        );
    }

    #[test]
    fn pss_oscnode_alone_selects_period_detection() {
        assert!(pss(".PSS OSCNODE=out FUND=1G").auto_period);
    }

    #[test]
    fn pss_continuation_lines_join_into_one_card() {
        let source = format!("{CIRCUIT}.PSS FUND=1G\n+ HARMS=11\n+ TOL=1e-7\n.END\n");
        let netlist = Netlist::parse(&source).expect("continued .PSS parses");
        let [AnalysisCommand::Pss(card)] = netlist.analyses.as_slice() else {
            panic!("expected one .PSS, got {:?}", netlist.analyses);
        };
        assert_eq!(card.num_harmonics, 11);
        assert_eq!(card.tolerance, 1e-7);
    }

    #[test]
    fn pss_refuses_ngspice_fields_rspice_cannot_honour() {
        let (card, line, issue) = card_failure(".pss 3.1e6 500e-6 out 256 10 50 5e-3");
        assert_eq!(card, AnalysisCard::Pss);
        assert_eq!(line, 5);
        assert!(
            matches!(
                issue,
                AnalysisCardIssue::UnhonourableField {
                    field: "steadycoeff",
                    ..
                }
            ),
            "unexpected issue: {issue:?}"
        );

        let (_, _, issue) = card_failure(".pss 3.1e6 500e-6 out 256 10 50 uic");
        assert!(
            matches!(
                issue,
                AnalysisCardIssue::UnhonourableField { field: "uic", .. }
            ),
            "unexpected issue: {issue:?}"
        );
    }

    #[test]
    fn pss_refuses_missing_incomplete_and_duplicated_fields() {
        assert!(matches!(
            card_failure(".PSS").2,
            AnalysisCardIssue::MissingField { field: "FUND" }
        ));
        assert!(matches!(
            card_failure(".PSS HARMS=9").2,
            AnalysisCardIssue::MissingField { field: "FUND" }
        ));
        assert!(matches!(
            card_failure(".PSS 1e6 0 osc 512").2,
            AnalysisCardIssue::MissingField { field: "harms" }
        ));
        assert!(matches!(
            card_failure(".PSS FUND=1G HARMS=9 HARMS=11").2,
            AnalysisCardIssue::DuplicateKeyword { keyword: "HARMS" }
        ));
    }

    #[test]
    fn pss_refuses_unknown_keywords_and_trailing_tokens() {
        assert!(matches!(
            card_failure(".PSS FUND=1G ERRPRESET=conservative").2,
            AnalysisCardIssue::UnknownKeyword { ref keyword } if keyword == "ERRPRESET"
        ));
        assert!(matches!(
            card_failure(".PSS FUND=1G junk").2,
            AnalysisCardIssue::TrailingToken { ref token } if token == "junk"
        ));
        // A keyword without its value is a stray token, not a silent default.
        assert!(matches!(
            card_failure(".PSS FUND=1G HARMS").2,
            AnalysisCardIssue::TrailingToken { ref token } if token.eq_ignore_ascii_case("HARMS")
        ));
        // The keyword form has no seventh positional field, so a stray number
        // there is a typo rather than ngspice's steady coefficient.
        assert!(matches!(
            card_failure(".PSS FUND=1G 5e-3").2,
            AnalysisCardIssue::TrailingToken { .. }
        ));
        // `uic` is an ngspice spelling in either form.
        assert!(matches!(
            card_failure(".PSS FUND=1G uic").2,
            AnalysisCardIssue::UnhonourableField { field: "uic", .. }
        ));
    }

    #[test]
    fn pss_refuses_non_finite_and_out_of_range_numbers() {
        for (card, field) in [
            (".PSS FUND=-1G", "FUND"),
            (".PSS FUND=0", "FUND"),
            (".PSS FUND=1G TSTAB=-1n", "TSTAB"),
            (".PSS FUND=1G TOL=0", "TOL"),
            (".PSS FUND=1G ABSTOL=-1e-12", "ABSTOL"),
            (".PSS FUND=1G DAMPING=2", "DAMPING"),
            (".PSS FUND=1G HARMS=0", "HARMS"),
            (".PSS FUND=1G POINTS=8", "POINTS"),
            (".PSS FUND=1G MAXITER=0", "MAXITER"),
            (".PSS 1e6 0 osc 512 8 0", "sciter"),
        ] {
            let issue = card_failure(card).2;
            assert!(
                matches!(issue, AnalysisCardIssue::InvalidNumber { field: bad, .. } if bad == field),
                "expected {field} to be refused on '{card}', got {issue:?}"
            );
        }
    }

    #[test]
    fn pss_refuses_forms_that_would_bind_one_quantity_twice() {
        assert!(matches!(
            card_failure(".PSS FUND=1G PERIODGUESS=1n").2,
            AnalysisCardIssue::ConflictingFields {
                first: "FUND",
                second: "PERIODGUESS"
            }
        ));
        assert!(matches!(
            card_failure(".PSS OSCNODE=out AUTONOMOUS=FALSE").2,
            AnalysisCardIssue::ConflictingFields {
                first: "OSCNODE",
                ..
            }
        ));
        assert!(matches!(
            card_failure(".PSS 1e6 0 osc 512 8 40 HARMS=9").2,
            AnalysisCardIssue::ConflictingFields {
                second: "HARMS",
                ..
            }
        ));
        assert!(matches!(
            card_failure(".PSS PERIODGUESS=1n").2,
            AnalysisCardIssue::ConflictingFields {
                first: "PERIODGUESS",
                ..
            }
        ));
    }

    #[test]
    fn pss_refuses_a_harmonic_bandwidth_its_sampling_would_alias() {
        // Nine harmonics need at least eighteen samples per period, whether
        // the samples are authored or defaulted.
        assert!(matches!(
            card_failure(".PSS FUND=1G HARMS=9 POINTS=16").2,
            AnalysisCardIssue::InvalidNumber {
                field: "POINTS",
                ..
            }
        ));
        assert!(matches!(
            card_failure(".PSS FUND=1G HARMS=200").2,
            AnalysisCardIssue::InvalidNumber {
                field: "POINTS",
                ..
            }
        ));
        assert!(matches!(
            card_failure(".pss 1e6 0 osc 16 9 40").2,
            AnalysisCardIssue::InvalidNumber {
                field: "POINTS",
                ..
            }
        ));
    }

    #[test]
    fn pss_refuses_a_stabilization_window_that_is_not_a_finite_time() {
        assert!(matches!(
            card_failure(".PSS AUTONOMOUS=TRUE PERIODGUESS=1e300 TSTABPERIODS=1000000000000").2,
            AnalysisCardIssue::InvalidNumber {
                field: "TSTABPERIODS",
                ..
            }
        ));
    }

    #[test]
    fn pss_refuses_an_unknown_integration_method() {
        assert!(matches!(
            card_failure(".PSS FUND=1G METHOD=simpson").2,
            AnalysisCardIssue::InvalidChoice {
                field: "METHOD",
                ..
            }
        ));
    }

    //-------------------------------------------------------------------------
    // .PAC
    //-------------------------------------------------------------------------

    #[test]
    fn pac_defaults_every_optional_field() {
        let card = pac(".HB 1G\n.PAC DEC 10 1k 1G INPUT=VRF OUT=V(out)");
        assert_eq!(card.sweep.variation, FreqVariation::Dec);
        assert_eq!(card.sweep.points, 10);
        assert_eq!(card.sweep.start_freq, 1.0e3);
        assert_eq!(card.sweep.stop_freq, 1.0e9);
        assert_eq!(card.sideband_min, PacCard::DEFAULT_SIDEBAND_MIN);
        assert_eq!(card.sideband_max, PacCard::DEFAULT_SIDEBAND_MAX);
        assert_eq!(card.reltol, PacCard::DEFAULT_RELTOL);
        assert_eq!(card.abstol, PacCard::DEFAULT_ABSTOL);
        assert_eq!(card.input_source, "VRF");
        assert_eq!(card.output_node, "OUT");
        assert_eq!(card.output_ref, None);
        assert_eq!(card.source, PeriodicSourceSelector::Preceding);
    }

    #[test]
    fn pac_binds_every_authorable_field() {
        let card = pac(
            ".hb 1G\n.pac lin 21 1meg 5meg input=vrf out=v(out,ref) sidebandmin=-3 \
             sidebandmax=7 reltol=1e-5 abstol=1e-15 from=hb",
        );
        assert_eq!(card.sweep.variation, FreqVariation::Lin);
        assert_eq!(card.sweep.points, 21);
        assert_eq!(card.sweep.start_freq, 1.0e6);
        assert_eq!(card.sweep.stop_freq, 5.0e6);
        assert_eq!(card.sideband_min, -3);
        assert_eq!(card.sideband_max, 7);
        assert_eq!(card.reltol, 1e-5);
        assert_eq!(card.abstol, 1e-15);
        assert_eq!(card.output_node, "OUT");
        assert_eq!(card.output_ref.as_deref(), Some("REF"));
        assert_eq!(card.source, PeriodicSourceSelector::Hb);
    }

    #[test]
    fn pac_maxsideband_sets_a_symmetric_range() {
        let card = pac(".HB 1G\n.PAC OCT 4 1k 1meg INPUT=VRF OUT=out MAXSIDEBAND=3");
        assert_eq!(card.sideband_min, -3);
        assert_eq!(card.sideband_max, 3);
        assert_eq!(card.sweep.variation, FreqVariation::Oct);
    }

    #[test]
    fn pac_refuses_missing_required_and_conflicting_sideband_forms() {
        assert!(matches!(
            card_failure(".HB 1G\n.PAC DEC 10 1k 1G OUT=V(out)").2,
            AnalysisCardIssue::MissingField { field: "INPUT" }
        ));
        assert!(matches!(
            card_failure(".HB 1G\n.PAC DEC 10 1k 1G INPUT=VRF").2,
            AnalysisCardIssue::MissingField { field: "OUT" }
        ));
        assert!(matches!(
            card_failure(
                ".HB 1G\n.PAC DEC 10 1k 1G INPUT=VRF OUT=out MAXSIDEBAND=2 SIDEBANDMIN=-1"
            )
            .2,
            AnalysisCardIssue::ConflictingFields {
                first: "MAXSIDEBAND",
                ..
            }
        ));
        assert!(matches!(
            card_failure(".HB 1G\n.PAC DEC 10 1k 1G INPUT=VRF OUT=out SIDEBANDMIN=4 SIDEBANDMAX=1")
                .2,
            AnalysisCardIssue::ConflictingFields {
                first: "SIDEBANDMIN",
                second: "SIDEBANDMAX"
            }
        ));
    }

    #[test]
    fn pac_refuses_malformed_sweeps_keywords_and_selectors() {
        assert!(matches!(
            card_failure(".HB 1G\n.PAC LOG 10 1k 1G INPUT=VRF OUT=out").2,
            AnalysisCardIssue::InvalidChoice {
                field: "variation",
                ..
            }
        ));
        assert!(matches!(
            card_failure(".HB 1G\n.PAC DEC 10 0 1G INPUT=VRF OUT=out").2,
            AnalysisCardIssue::InvalidNumber {
                field: "fstart",
                ..
            }
        ));
        assert!(matches!(
            card_failure(".HB 1G\n.PAC DEC 10 1G 1k INPUT=VRF OUT=out").2,
            AnalysisCardIssue::InvalidNumber { field: "fstop", .. }
        ));
        assert!(matches!(
            card_failure(".HB 1G\n.PAC DEC 10 1k 1G INPUT=VRF INPUT=VLO OUT=out").2,
            AnalysisCardIssue::DuplicateKeyword { keyword: "INPUT" }
        ));
        assert!(matches!(
            card_failure(".HB 1G\n.PAC DEC 10 1k 1G INPUT=VRF OUT=out SIDEBANDS=3").2,
            AnalysisCardIssue::UnknownKeyword { ref keyword } if keyword == "SIDEBANDS"
        ));
        assert!(matches!(
            card_failure(".HB 1G\n.PAC DEC 10 1k 1G INPUT=VRF OUT=out trailing").2,
            AnalysisCardIssue::TrailingToken { .. }
        ));
        assert!(matches!(
            card_failure(".HB 1G\n.PAC DEC 10 1k 1G INPUT=VRF OUT=out FROM=TRAN").2,
            AnalysisCardIssue::InvalidChoice { field: "FROM", .. }
        ));
    }

    //-------------------------------------------------------------------------
    // .PNOISE
    //-------------------------------------------------------------------------

    #[test]
    fn pnoise_defaults_every_optional_field() {
        let card = pnoise(".HB 1G\n.PNOISE DEC 10 1 1meg OUT=V(out)");
        assert_eq!(card.sweep.variation, FreqVariation::Dec);
        assert_eq!(card.sweep.points, 10);
        assert_eq!(card.sweep.start_freq, 1.0);
        assert_eq!(card.sweep.stop_freq, 1.0e6);
        assert_eq!(card.output_node, "OUT");
        assert_eq!(card.reference_node, None);
        assert_eq!(card.input_source, None);
        assert_eq!(card.max_sideband, 6);
        assert_eq!(card.source, PeriodicSourceSelector::Preceding);
    }

    #[test]
    fn pnoise_binds_every_authorable_field() {
        let card = pnoise(
            ".pss fund=1g\n.pnoise lin 32 10 1k out=v(out,ref) input=vrf maxsideband=9 from=pss",
        );
        assert_eq!(card.sweep.variation, FreqVariation::Lin);
        assert_eq!(card.sweep.points, 32);
        assert_eq!(card.output_node, "OUT");
        assert_eq!(card.reference_node.as_deref(), Some("REF"));
        assert_eq!(card.input_source.as_deref(), Some("VRF"));
        assert_eq!(card.max_sideband, 9);
        assert_eq!(card.source, PeriodicSourceSelector::Pss);
    }

    #[test]
    fn pnoise_refuses_missing_output_and_malformed_keywords() {
        assert!(matches!(
            card_failure(".HB 1G\n.PNOISE DEC 10 1 1meg").2,
            AnalysisCardIssue::MissingField { field: "OUT" }
        ));
        assert!(matches!(
            card_failure(".HB 1G\n.PNOISE DEC 10 1 1meg OUT=out MAXSIDEBAND=0").2,
            AnalysisCardIssue::InvalidNumber {
                field: "MAXSIDEBAND",
                ..
            }
        ));
        assert!(matches!(
            card_failure(".HB 1G\n.PNOISE DEC 10 1 1meg OUT=out OUT=other").2,
            AnalysisCardIssue::DuplicateKeyword { keyword: "OUT" }
        ));
        assert!(matches!(
            card_failure(".HB 1G\n.PNOISE DEC 10 1 1meg OUT=out NOISETYPE=pm").2,
            AnalysisCardIssue::UnknownKeyword { ref keyword } if keyword == "NOISETYPE"
        ));
    }

    //-------------------------------------------------------------------------
    // .ENVELOPE
    //-------------------------------------------------------------------------

    #[test]
    fn envelope_defaults_its_maximum_step_to_a_fiftieth_of_the_window() {
        let card = envelope(".HB 1G\n.ENVELOPE TSTOP=1u");
        assert_eq!(card.duration, 1e-6);
        assert_eq!(card.max_step, 1e-6 / 50.0);
        assert!(card.frozen_sources.is_empty());
    }

    #[test]
    fn envelope_binds_a_frozen_source_list() {
        let card = envelope(".HB 1G\n.envelope tstop=2u maxstep=5n freeze=(VMOD,VBIAS)");
        assert_eq!(card.duration, 2e-6);
        assert_eq!(card.max_step, 5e-9);
        assert_eq!(
            card.frozen_sources,
            vec!["VMOD".to_string(), "VBIAS".to_string()]
        );
    }

    #[test]
    fn envelope_accepts_a_single_unparenthesized_frozen_source() {
        assert_eq!(
            envelope(".HB 1G\n.ENVELOPE TSTOP=1u FREEZE=VMOD").frozen_sources,
            vec!["VMOD".to_string()]
        );
    }

    #[test]
    fn envelope_refuses_missing_duration_bad_steps_and_repeated_sources() {
        assert!(matches!(
            card_failure(".HB 1G\n.ENVELOPE MAXSTEP=1n").2,
            AnalysisCardIssue::MissingField { field: "TSTOP" }
        ));
        assert!(matches!(
            card_failure(".HB 1G\n.ENVELOPE TSTOP=0").2,
            AnalysisCardIssue::InvalidNumber { field: "TSTOP", .. }
        ));
        assert!(matches!(
            card_failure(".HB 1G\n.ENVELOPE TSTOP=1u MAXSTEP=-1n").2,
            AnalysisCardIssue::InvalidNumber {
                field: "MAXSTEP",
                ..
            }
        ));
        assert!(matches!(
            card_failure(".HB 1G\n.ENVELOPE TSTOP=1u FREEZE=(VMOD,vmod)").2,
            AnalysisCardIssue::InvalidName {
                field: "FREEZE",
                ..
            }
        ));
        assert!(matches!(
            card_failure(".HB 1G\n.ENVELOPE TSTOP=1u TSTOP=2u").2,
            AnalysisCardIssue::DuplicateKeyword { keyword: "TSTOP" }
        ));
        assert!(matches!(
            card_failure(".HB 1G\n.ENVELOPE TSTOP=1u SOURCES=VMOD").2,
            AnalysisCardIssue::UnknownKeyword { ref keyword } if keyword == "SOURCES"
        ));
    }

    #[test]
    fn card_failures_report_their_authored_line() {
        let source = format!("{CIRCUIT}.OP\n.PSS FUND=1G HARMS=0\n.END\n");
        let error = Netlist::parse(&source).expect_err("invalid .PSS must be refused");
        let ParseError::AnalysisCard(error) = error else {
            panic!("expected a typed analysis-card error");
        };
        assert_eq!(error.card, AnalysisCard::Pss);
        assert_eq!(error.line, 6);
    }
}
