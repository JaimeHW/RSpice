//! Strict native `.QPXF` parsing with signed frequency and sideband lists.
use super::analysis_card_scan::*;
use super::*;
use crate::netlist::{QpacSweep, QpxfCard, QpxfCardLattices, QpxfCardOutput, QpxfCardSources};
const CARD: AnalysisCard = AnalysisCard::Qpxf;
fn error(line: usize, message: impl Into<String>) -> ParseError {
    ParseError::Syntax {
        line,
        message: format!(".QPXF: {}", message.into()),
    }
}
fn list<T>(
    stream: &mut TokenStream,
    line: usize,
    mut item: impl FnMut(&mut TokenStream) -> Result<T, ParseError>,
) -> Result<Vec<T>, ParseError> {
    if !stream.consume(&TokenKind::LParen) {
        return Err(error(line, "lists require parentheses"));
    }
    let mut values = Vec::new();
    loop {
        values.push(item(stream)?);
        if stream.consume(&TokenKind::RParen) {
            return Ok(values);
        }
        if !stream.consume(&TokenKind::Comma) {
            return Err(error(line, "expected ',' or ')' in list"));
        }
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
fn choice(
    stream: &mut TokenStream,
    line: usize,
    field: &'static str,
    choices: &[&str],
) -> Result<String, ParseError> {
    let value = card_name(stream, line, CARD, field)?.to_ascii_uppercase();
    if !choices.contains(&value.as_str()) {
        return Err(error(
            line,
            format!("{field} must be {}", choices.join(" or ")),
        ));
    }
    Ok(value)
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
        let values = list(stream, line, |s| number(s, line, params, "LIST"))?;
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
    let mut card = QpxfCard {
        sweep,
        ..Default::default()
    };
    let mut sources = None;
    let mut lattices = None;
    let mut output = None;
    let mut output_lattice = None;
    let mut from = None;
    loop {
        skip_commas(stream);
        if at_card_end(stream) {
            break;
        }
        let keyword = take_keyword(stream).ok_or_else(|| error(line, "expected KEY=VALUE"))?;
        match keyword.as_str() {
            "SOURCE" | "SOURCES" => {
                let selected = if keyword == "SOURCE" {
                    QpxfCardSources::Named(vec![name(stream, line)?])
                } else if matches!(stream.peek().kind, TokenKind::LParen) {
                    QpxfCardSources::Named(list(stream, line, |s| name(s, line))?)
                } else {
                    choice(stream, line, "SOURCES", &["ALL"])?;
                    QpxfCardSources::AllIndependent
                };
                bind_once(&mut sources, selected, CARD, line, "SOURCES")?;
            }
            "OUT" => {
                let kind = choice(stream, line, "OUT", &["V", "I"])?;
                if !stream.consume(&TokenKind::LParen) {
                    return Err(error(line, "OUT requires V(node[,reference]) or I(branch)"));
                }
                let first = name(stream, line)?;
                let value = if kind == "V" {
                    let negative = if stream.consume(&TokenKind::Comma) {
                        name(stream, line)?
                    } else {
                        "0".into()
                    };
                    QpxfCardOutput::Voltage {
                        positive: first,
                        negative,
                    }
                } else {
                    QpxfCardOutput::BranchCurrent { branch: first }
                };
                if !stream.consume(&TokenKind::RParen) {
                    return Err(error(line, "expected ')' after output"));
                }
                bind_once(&mut output, value, CARD, line, "OUT")?;
            }
            "INLATTICES" => {
                let selected = if matches!(stream.peek().kind, TokenKind::LParen) {
                    QpxfCardLattices::Explicit(list(stream, line, |s| {
                        list(s, line, |s| {
                            card_signed(s, line, params, CARD, "INLATTICES", i32::MIN)
                        })
                    })?)
                } else {
                    choice(stream, line, "INLATTICES", &["ALL"])?;
                    QpxfCardLattices::AllRetained
                };
                bind_once(
                    &mut lattices,
                    selected,
                    CARD,
                    line,
                    "input lattice selection",
                )?;
            }
            "INLATTICE" => {
                let tuple = list(stream, line, |s| {
                    card_signed(s, line, params, CARD, "INLATTICE", i32::MIN)
                })?;
                bind_once(
                    &mut lattices,
                    QpxfCardLattices::Explicit(vec![tuple]),
                    CARD,
                    line,
                    "input lattice selection",
                )?;
            }
            "MAXORDERS" => {
                let orders = list(stream, line, |s| {
                    card_count(s, line, params, CARD, "MAXORDERS", 0)
                })?;
                bind_once(
                    &mut lattices,
                    QpxfCardLattices::MaxOrders(orders),
                    CARD,
                    line,
                    "input lattice selection",
                )?;
            }
            "OUTLATTICE" => {
                let tuple = list(stream, line, |s| {
                    card_signed(s, line, params, CARD, "OUTLATTICE", i32::MIN)
                })?;
                bind_once(&mut output_lattice, tuple, CARD, line, "OUTLATTICE")?;
            }
            "AXIS" => bind_once(
                &mut card.frequency_axis,
                choice(stream, line, "AXIS", &["OUTPUT", "OFFSET"])?,
                CARD,
                line,
                "AXIS",
            )?,
            "SOLVER" => bind_once(
                &mut card.linear_solver,
                choice(stream, line, "SOLVER", &["AUTO", "DIRECT", "KRYLOV"])?,
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
            "GROUPDELAY" => bind_once(
                &mut card.group_delay,
                card_bool(stream, line, CARD, "GROUPDELAY")?,
                CARD,
                line,
                "GROUPDELAY",
            )?,
            "GDFLOOR" => bind_once(
                &mut card.group_delay_magnitude_floor,
                card_number(
                    stream,
                    line,
                    params,
                    CARD,
                    "GDFLOOR",
                    "a nonnegative magnitude floor",
                    |v| v >= 0.0,
                )?,
                CARD,
                line,
                "GDFLOOR",
            )?,
            "FROM" => bind_once(
                &mut from,
                choice(stream, line, "FROM", &["QPSS"])?,
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
    card.input_sources = sources.unwrap_or_default();
    card.input_lattices = lattices.unwrap_or_default();
    card.output = output.ok_or_else(|| error(line, "OUT is required"))?;
    card.output_lattice = output_lattice.ok_or_else(|| error(line, "OUTLATTICE is required"))?;
    let dimensions = card.output_lattice.len();
    if dimensions < 2
        || match &card.input_lattices {
            QpxfCardLattices::AllRetained => false,
            QpxfCardLattices::Explicit(tuples) => tuples.iter().any(|t| t.len() != dimensions),
            QpxfCardLattices::MaxOrders(orders) => orders.len() != dimensions,
        }
    {
        return Err(error(
            line,
            "input selection and output tuple must have the same dimensions, at least two",
        ));
    }
    Ok(AnalysisCommand::Qpxf(Box::new(card)))
}
