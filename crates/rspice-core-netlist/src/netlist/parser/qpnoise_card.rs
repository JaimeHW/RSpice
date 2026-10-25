//! Strict native `.QPNOISE` parsing with signed frequency and sideband lists.
use super::analysis_card_scan::{
    at_card_end, at_keyword, bind_once, card_bool, card_choice, card_count, card_error, card_list,
    card_number, card_signed, take_keyword,
};
use super::*;
use crate::netlist::{
    QpacSweep, QpnoiseCard, QpnoiseCardLattices, QpnoiseCardOutput, QpnoiseCardSources,
    QpxfCardOutput,
};
const CARD: AnalysisCard = AnalysisCard::Qpnoise;
fn error(line: usize, message: impl Into<String>) -> ParseError {
    ParseError::Syntax {
        line,
        message: format!(".QPNOISE: {}", message.into()),
    }
}
fn number(
    stream: &mut TokenStream,
    line: usize,
    params: &ParamContext,
    field: &'static str,
) -> Result<Value, ParseError> {
    card_number(stream, line, params, CARD, field, "a finite number", |_| {
        true
    })
}
fn name(stream: &mut TokenStream, line: usize) -> Result<String, ParseError> {
    if let TokenKind::StringLit(value) = &stream.peek().kind {
        let value = value.clone();
        stream.advance();
        if value.trim().is_empty() || value.contains(['\r', '\n']) {
            return Err(error(line, "name must be nonempty and occupy one line"));
        }
        return Ok(value);
    }
    Ok(expect_node_with_authored_spelling(stream, line)?.1)
}
fn tuple(
    stream: &mut TokenStream,
    line: usize,
    params: &ParamContext,
) -> Result<Vec<i32>, ParseError> {
    card_list(stream, line, CARD, |s| {
        card_signed(s, line, params, CARD, "tone tuple", i32::MIN)
    })
}
fn observation(stream: &mut TokenStream, line: usize) -> Result<QpxfCardOutput, ParseError> {
    let kind = card_choice(stream, line, CARD, "output", &["V", "I"])?;
    if !stream.consume(&TokenKind::LParen) {
        return Err(error(
            line,
            "output requires V(node[,reference]) or I(branch)",
        ));
    }
    let first = name(stream, line)?;
    let output = if kind == "V" {
        QpxfCardOutput::Voltage {
            positive: first,
            negative: if stream.consume(&TokenKind::Comma) {
                name(stream, line)?
            } else {
                "0".into()
            },
        }
    } else {
        QpxfCardOutput::BranchCurrent { branch: first }
    };
    if !stream.consume(&TokenKind::RParen) {
        return Err(error(line, "expected ')' after output"));
    }
    Ok(output)
}
pub(super) fn parse(
    stream: &mut TokenStream,
    line: usize,
    params: &ParamContext,
) -> Result<AnalysisCommand, ParseError> {
    let sweep = if at_keyword(stream) {
        if take_keyword(stream).as_deref() != Some("LIST") {
            return Err(error(line, "expected LIN, DEC, OCT or LIST=(...)"));
        }
        let values = card_list(stream, line, CARD, |s| number(s, line, params, "LIST"))?;
        if values.windows(2).any(|p| p[0] >= p[1]) {
            return Err(error(line, "LIST frequencies must be strictly increasing"));
        }
        QpacSweep::Explicit(values)
    } else {
        let variation = match expect_ident(stream, line)?.to_ascii_uppercase().as_str() {
            "LIN" => FreqVariation::Lin,
            "DEC" => FreqVariation::Dec,
            "OCT" => FreqVariation::Oct,
            _ => return Err(error(line, "expected LIN, DEC, OCT or LIST=(...)")),
        };
        let points = card_count(stream, line, params, CARD, "points", 1)?;
        let start_freq = number(stream, line, params, "start frequency")?;
        let stop_freq = number(stream, line, params, "stop frequency")?;
        if stop_freq < start_freq || (variation != FreqVariation::Lin && start_freq <= 0.0) {
            return Err(error(
                line,
                "generated sweep needs start <= stop and positive start for DEC/OCT",
            ));
        }
        QpacSweep::Generated(PeriodicSweep {
            variation,
            points,
            start_freq,
            stop_freq,
        })
    };
    let mut card = QpnoiseCard {
        sweep,
        ..Default::default()
    };
    let (
        mut output,
        mut output_tuple,
        mut outputs,
        mut lattices,
        mut minimum,
        mut maximum,
        mut sources,
        mut from,
    ) = (None, None, None, None, None, None, None, None);
    loop {
        skip_commas(stream);
        if at_card_end(stream) {
            break;
        }
        let keyword = take_keyword(stream).ok_or_else(|| error(line, "expected KEY=VALUE"))?;
        match keyword.as_str() {
            "OUT" => bind_once(&mut output, observation(stream, line)?, CARD, line, "OUT")?,
            "OUTLATTICE" => bind_once(
                &mut output_tuple,
                tuple(stream, line, params)?,
                CARD,
                line,
                "OUTLATTICE",
            )?,
            "OUTS" => {
                let selected = card_list(stream, line, CARD, |s| {
                    if !s.consume(&TokenKind::LParen) {
                        return Err(error(
                            line,
                            "each OUTS item must be (V(node[,ref]) or I(branch), (tone tuple))",
                        ));
                    }
                    let observation = observation(s, line)?;
                    if !s.consume(&TokenKind::Comma) {
                        return Err(error(line, "expected comma before output tuple"));
                    }
                    let lattice = tuple(s, line, params)?;
                    if !s.consume(&TokenKind::RParen) {
                        return Err(error(line, "expected ')' after output tuple"));
                    }
                    Ok(QpnoiseCardOutput {
                        observation,
                        lattice,
                    })
                })?;
                bind_once(&mut outputs, selected, CARD, line, "OUTS")?;
            }
            "SOURCE" => bind_once(
                &mut card.input_source,
                name(stream, line)?,
                CARD,
                line,
                "SOURCE",
            )?,
            "INLATTICE" => bind_once(
                &mut card.input_lattice,
                tuple(stream, line, params)?,
                CARD,
                line,
                "INLATTICE",
            )?,
            "NOISELATTICES" => {
                let value = if matches!(stream.peek().kind, TokenKind::LParen) {
                    QpnoiseCardLattices::Explicit(card_list(stream, line, CARD, |s| {
                        tuple(s, line, params)
                    })?)
                } else {
                    card_choice(stream, line, CARD, "NOISELATTICES", &["ALL"])?;
                    QpnoiseCardLattices::AllRetained
                };
                bind_once(&mut lattices, value, CARD, line, "noise lattice selection")?;
            }
            "MAXORDERS" => bind_once(
                &mut lattices,
                QpnoiseCardLattices::MaxOrders(card_list(stream, line, CARD, |s| {
                    card_count(s, line, params, CARD, "MAXORDERS", 0)
                })?),
                CARD,
                line,
                "noise lattice selection",
            )?,
            "MINLATTICE" => bind_once(
                &mut minimum,
                tuple(stream, line, params)?,
                CARD,
                line,
                "MINLATTICE",
            )?,
            "MAXLATTICE" => bind_once(
                &mut maximum,
                tuple(stream, line, params)?,
                CARD,
                line,
                "MAXLATTICE",
            )?,
            "NOISESOURCES" => {
                let value = if matches!(stream.peek().kind, TokenKind::LParen) {
                    QpnoiseCardSources::Only(card_list(stream, line, CARD, |s| name(s, line))?)
                } else {
                    card_choice(stream, line, CARD, "NOISESOURCES", &["ALL"])?;
                    QpnoiseCardSources::All
                };
                bind_once(&mut sources, value, CARD, line, "noise source selection")?;
            }
            "EXCLUDESOURCES" => bind_once(
                &mut sources,
                QpnoiseCardSources::Except(card_list(stream, line, CARD, |s| name(s, line))?),
                CARD,
                line,
                "noise source selection",
            )?,
            "AXIS" => bind_once(
                &mut card.frequency_axis,
                card_choice(stream, line, CARD, "AXIS", &["OUTPUT", "OFFSET"])?,
                CARD,
                line,
                "AXIS",
            )?,
            "SOLVER" => bind_once(
                &mut card.linear_solver,
                card_choice(stream, line, CARD, "SOLVER", &["AUTO", "DIRECT", "KRYLOV"])?,
                CARD,
                line,
                "SOLVER",
            )?,
            "KRYLOVRESTART" => bind_once(
                &mut card.krylov_restart,
                card_count(stream, line, params, CARD, "KRYLOVRESTART", 1)?,
                CARD,
                line,
                "KRYLOVRESTART",
            )?,
            "KRYLOVCYCLES" => bind_once(
                &mut card.krylov_cycles,
                card_count(stream, line, params, CARD, "KRYLOVCYCLES", 1)?,
                CARD,
                line,
                "KRYLOVCYCLES",
            )?,
            "LINEARTOL" | "RELTOL" => bind_once(
                &mut card.linear_tolerance,
                card_number(
                    stream,
                    line,
                    params,
                    CARD,
                    "LINEARTOL",
                    "a tolerance between zero and one",
                    |v| v > 0.0 && v < 1.0,
                )?,
                CARD,
                line,
                "LINEARTOL",
            )?,
            "INTEGRATED" => bind_once(
                &mut card.integrated_noise,
                card_bool(stream, line, CARD, "INTEGRATED")?,
                CARD,
                line,
                "INTEGRATED",
            )?,
            "INTEGRATION" => bind_once(
                &mut card.integration_method,
                card_choice(stream, line, CARD, "INTEGRATION", &["LINEAR", "LOGLOG"])?,
                CARD,
                line,
                "INTEGRATION",
            )?,
            "BAND" => {
                let values = card_list(stream, line, CARD, |s| number(s, line, params, "BAND"))?;
                let band = values
                    .try_into()
                    .map_err(|_| error(line, "BAND requires exactly two frequencies"))?;
                bind_once(&mut card.integration_band, band, CARD, line, "BAND")?;
            }
            "RANK" => bind_once(
                &mut card.contributor_ranking,
                card_bool(stream, line, CARD, "RANK")?,
                CARD,
                line,
                "RANK",
            )?,
            "NOISEFIGURE" => bind_once(
                &mut card.noise_figure,
                card_bool(stream, line, CARD, "NOISEFIGURE")?,
                CARD,
                line,
                "NOISEFIGURE",
            )?,
            "SOURCERESISTOR" => bind_once(
                &mut card.source_resistor,
                name(stream, line)?,
                CARD,
                line,
                "SOURCERESISTOR",
            )?,
            "TREF" => bind_once(
                &mut card.reference_temperature,
                card_number(
                    stream,
                    line,
                    params,
                    CARD,
                    "TREF",
                    "a positive temperature in kelvin",
                    |v| v > 0.0,
                )?,
                CARD,
                line,
                "TREF",
            )?,
            "REFLATTICES" => bind_once(
                &mut card.reference_lattices,
                card_list(stream, line, CARD, |s| tuple(s, line, params))?,
                CARD,
                line,
                "REFLATTICES",
            )?,
            "FROM" => bind_once(
                &mut from,
                card_choice(stream, line, CARD, "FROM", &["QPSS"])?,
                CARD,
                line,
                "FROM",
            )?,
            _ => {
                return Err(card_error(
                    CARD,
                    line,
                    AnalysisCardIssue::UnknownKeyword { keyword },
                ));
            }
        }
    }
    card.outputs = match (outputs, output, output_tuple) {
        (Some(values), None, None) => values,
        (None, Some(observation), Some(lattice)) => vec![QpnoiseCardOutput {
            observation,
            lattice,
        }],
        _ => {
            return Err(error(
                line,
                "use OUTS or the complete OUT and OUTLATTICE pair, without mixing them",
            ));
        }
    };
    card.noise_lattices = match (lattices, minimum, maximum) {
        (value, None, None) => value.unwrap_or_default(),
        (None, Some(minimum), Some(maximum)) => QpnoiseCardLattices::Range { minimum, maximum },
        _ => {
            return Err(error(
                line,
                "MINLATTICE and MAXLATTICE require each other and cannot be mixed with other noise lattice selections",
            ));
        }
    };
    card.noise_sources = sources.unwrap_or_default();
    Ok(AnalysisCommand::Qpnoise(Box::new(card)))
}
