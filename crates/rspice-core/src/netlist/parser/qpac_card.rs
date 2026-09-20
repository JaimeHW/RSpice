//! `.QPAC LIN|DEC|OCT np start stop` or `.QPAC LIST=(offset,...)`.
use super::analysis_card_scan::*;
use super::*;
use crate::netlist::{QpacCard, QpacSweep};

const CARD: AnalysisCard = AnalysisCard::Qpac;

fn error(line: usize, message: impl Into<String>) -> ParseError {
    ParseError::Syntax {
        line,
        message: format!(".QPAC: {}", message.into()),
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
    let (_, authored) = expect_node_with_authored_spelling(stream, line)?;
    Ok(authored)
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
        if values.windows(2).any(|pair| pair[0] >= pair[1]) {
            return Err(error(line, "LIST offsets must be strictly increasing"));
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
                "generated sweep needs start <= stop (positive start for DEC/OCT)",
            ));
        }
        QpacSweep::Generated(PeriodicSweep {
            variation,
            points,
            start_freq,
            stop_freq,
        })
    };
    let mut card = QpacCard {
        sweep,
        ..Default::default()
    };
    let mut source = None;
    let mut output = None;
    let mut input_tuple = None;
    let mut output_tuple = None;
    let mut from = None;
    loop {
        skip_commas(stream);
        if at_card_end(stream) {
            break;
        }
        let keyword = take_keyword(stream).ok_or_else(|| error(line, "expected KEY=VALUE"))?;
        match keyword.as_str() {
            "SOURCE" => bind_once(&mut source, name(stream, line)?, CARD, line, "SOURCE")?,
            "OUT" => {
                let first = name(stream, line)?;
                let (node, reference) =
                    if first.eq_ignore_ascii_case("V") && stream.consume(&TokenKind::LParen) {
                        let node = name(stream, line)?;
                        let reference = if stream.consume(&TokenKind::Comma) {
                            name(stream, line)?
                        } else {
                            "0".into()
                        };
                        if !stream.consume(&TokenKind::RParen) {
                            return Err(error(line, "expected ')' after voltage output"));
                        }
                        (node, reference)
                    } else {
                        (first, "0".into())
                    };
                bind_once(&mut output, (node, reference), CARD, line, "OUT")?;
            }
            "INLATTICE" | "OUTLATTICE" => {
                let field = if keyword == "INLATTICE" {
                    "INLATTICE"
                } else {
                    "OUTLATTICE"
                };
                let values = list(stream, line, |s| {
                    card_signed(s, line, params, CARD, field, i32::MIN)
                })?;
                bind_once(
                    if keyword == "INLATTICE" {
                        &mut input_tuple
                    } else {
                        &mut output_tuple
                    },
                    values,
                    CARD,
                    line,
                    field,
                )?;
            }
            "MAG" => bind_once(
                &mut card.magnitude,
                card_number(
                    stream,
                    line,
                    params,
                    CARD,
                    "MAG",
                    "a positive magnitude",
                    |v| v > 0.0,
                )?,
                CARD,
                line,
                "MAG",
            )?,
            "PHASE" => bind_once(
                &mut card.phase_degrees,
                number(stream, line, params, "PHASE")?,
                CARD,
                line,
                "PHASE",
            )?,
            "SOLVER" => {
                let method = card_name(stream, line, CARD, "SOLVER")?;
                if !["AUTO", "DIRECT", "KRYLOV"]
                    .iter()
                    .any(|v| method.eq_ignore_ascii_case(v))
                {
                    return Err(error(line, "SOLVER must be AUTO, DIRECT or KRYLOV"));
                }
                bind_once(&mut card.linear_solver, method, CARD, line, "SOLVER")?;
            }
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
            "IABSTOL" | "ABSTOL" => bind_once(
                &mut card.current_absolute_tolerance,
                card_number(
                    stream,
                    line,
                    params,
                    CARD,
                    "IABSTOL",
                    "a positive tolerance",
                    |v| v > 0.0,
                )?,
                CARD,
                line,
                "IABSTOL",
            )?,
            "VABSTOL" => bind_once(
                &mut card.voltage_absolute_tolerance,
                card_number(
                    stream,
                    line,
                    params,
                    CARD,
                    "VABSTOL",
                    "a positive tolerance",
                    |v| v > 0.0,
                )?,
                CARD,
                line,
                "VABSTOL",
            )?,
            "FROM" => {
                let value = card_name(stream, line, CARD, "FROM")?;
                if !value.eq_ignore_ascii_case("QPSS") {
                    return Err(error(line, "FROM must be QPSS"));
                }
                bind_once(&mut from, value, CARD, line, "FROM")?;
            }
            _ => {
                return Err(card_error(
                    CARD,
                    line,
                    AnalysisCardIssue::UnknownKeyword { keyword },
                ));
            }
        }
    }
    card.input_source = source.ok_or_else(|| error(line, "SOURCE is required"))?;
    (card.output_node, card.output_ref) = output.ok_or_else(|| error(line, "OUT is required"))?;
    card.input_lattice = input_tuple.ok_or_else(|| error(line, "INLATTICE is required"))?;
    card.output_lattice = output_tuple.ok_or_else(|| error(line, "OUTLATTICE is required"))?;
    if card.input_lattice.len() < 2 || card.input_lattice.len() != card.output_lattice.len() {
        return Err(error(
            line,
            "input/output tuples require the same number of coordinates, at least two",
        ));
    }
    Ok(AnalysisCommand::Qpac(Box::new(card)))
}
