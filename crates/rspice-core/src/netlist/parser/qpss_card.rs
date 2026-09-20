//! `.QPSS f1 f2 [...] HARMS=(...) [KEY=VALUE ...]`.
//! Parsing retains authored values; the engine owns defaults and joint validation.
use super::analysis_card_scan::*;
use super::*;
use crate::netlist::QpssCard;

const CARD: AnalysisCard = AnalysisCard::Qpss;

pub(super) fn parse(
    stream: &mut TokenStream,
    line: usize,
    params: &ParamContext,
) -> Result<AnalysisCommand, ParseError> {
    let mut card = QpssCard::default();
    loop {
        skip_commas(stream);
        if at_card_end(stream) || at_keyword(stream) {
            break;
        }
        card.frequencies.push(card_number(
            stream,
            line,
            params,
            CARD,
            "frequency",
            "a positive frequency",
            |v| v > 0.0,
        )?);
    }
    if card.frequencies.len() < 2 {
        return Err(card_error(
            CARD,
            line,
            AnalysisCardIssue::MissingField {
                field: "at least two tone frequencies",
            },
        ));
    }
    loop {
        skip_commas(stream);
        if at_card_end(stream) {
            break;
        }
        let keyword = take_keyword(stream).ok_or_else(|| {
            card_error(
                CARD,
                line,
                AnalysisCardIssue::TrailingToken {
                    token: stream.peek().lexeme.clone(),
                },
            )
        })?;
        match keyword.as_str() {
            "HARMS" => {
                if !card.harmonics.is_empty() {
                    return Err(card_error(
                        CARD,
                        line,
                        AnalysisCardIssue::DuplicateKeyword { keyword: "HARMS" },
                    ));
                }
                card.harmonics = card_count_list(stream, line, params, CARD, "HARMS")?;
            }
            "OVERSAMPLE" => bind_once(
                &mut card.oversample,
                card_count_list(stream, line, params, CARD, "OVERSAMPLE")?,
                CARD,
                line,
                "OVERSAMPLE",
            )?,
            "POINTS" => bind_once(
                &mut card.points,
                card_count_list(stream, line, params, CARD, "POINTS")?,
                CARD,
                line,
                "POINTS",
            )?,
            "MAXMIXING" => bind_once(
                &mut card.max_mixing_order,
                card_count(stream, line, params, CARD, "MAXMIXING", 1)?,
                CARD,
                line,
                "MAXMIXING",
            )?,
            "MAXITER" => bind_once(
                &mut card.max_iterations,
                card_count(stream, line, params, CARD, "MAXITER", 1)?,
                CARD,
                line,
                "MAXITER",
            )?,
            "MAXBACKTRACKS" => bind_once(
                &mut card.max_backtracks,
                card_count(stream, line, params, CARD, "MAXBACKTRACKS", 0)?,
                CARD,
                line,
                "MAXBACKTRACKS",
            )?,
            "RELTOL" => bind_once(
                &mut card.relative_tolerance,
                positive(stream, line, params, "RELTOL")?,
                CARD,
                line,
                "RELTOL",
            )?,
            "ABSTOL" | "IABSTOL" => bind_once(
                &mut card.current_absolute_tolerance,
                positive(stream, line, params, "IABSTOL")?,
                CARD,
                line,
                "IABSTOL",
            )?,
            "VABSTOL" => bind_once(
                &mut card.voltage_absolute_tolerance,
                positive(stream, line, params, "VABSTOL")?,
                CARD,
                line,
                "VABSTOL",
            )?,
            "INIT" => {
                let value = card_name(stream, line, CARD, "INIT")?;
                let dc = match value.to_ascii_uppercase().as_str() {
                    "ZERO" => false,
                    "DC" => true,
                    _ => {
                        return Err(card_error(
                            CARD,
                            line,
                            AnalysisCardIssue::InvalidChoice {
                                field: "INIT",
                                value,
                                expected: "ZERO or DC",
                            },
                        ));
                    }
                };
                bind_once(&mut card.dc_initial_state, dc, CARD, line, "INIT")?;
            }
            _ => {
                let index = keyword
                    .strip_prefix("SOURCE")
                    .filter(|v| !v.is_empty() && v.bytes().all(|c| c.is_ascii_digit()))
                    .and_then(|v| v.parse::<usize>().ok())
                    .filter(|n| *n > 0 && *n <= card.frequencies.len());
                let Some(index) = index else {
                    return Err(card_error(
                        CARD,
                        line,
                        AnalysisCardIssue::UnknownKeyword { keyword },
                    ));
                };
                // Keep the authored spelling so configuration identity survives a
                // writer/parser round trip; source lookup is case-insensitive.
                let source = stream.peek().lexeme.clone();
                let parsed = card_name(stream, line, CARD, "SOURCE<k>")?;
                if !parsed.eq_ignore_ascii_case(&source) {
                    return Err(card_error(
                        CARD,
                        line,
                        AnalysisCardIssue::InvalidName {
                            field: "SOURCE<k>",
                            value: parsed,
                        },
                    ));
                }
                if card
                    .sources
                    .iter()
                    .any(|(name, tone)| name.eq_ignore_ascii_case(&source) && *tone == index - 1)
                {
                    return Err(card_error(
                        CARD,
                        line,
                        AnalysisCardIssue::InvalidChoice {
                            field: "SOURCE<k>",
                            value: source,
                            expected: "a source/tone pair authored once",
                        },
                    ));
                }
                card.sources.push((source, index - 1));
            }
        }
    }
    if card.oversample.is_some() && card.points.is_some() {
        return Err(card_error(
            CARD,
            line,
            AnalysisCardIssue::InvalidChoice {
                field: "sampling",
                value: "OVERSAMPLE and POINTS".into(),
                expected: "one sampling mode",
            },
        ));
    }
    Ok(AnalysisCommand::Qpss(Box::new(card)))
}

fn positive(
    stream: &mut TokenStream,
    line: usize,
    params: &ParamContext,
    field: &'static str,
) -> Result<Value, ParseError> {
    card_number(
        stream,
        line,
        params,
        CARD,
        field,
        "a positive tolerance",
        |v| v > 0.0,
    )
}
