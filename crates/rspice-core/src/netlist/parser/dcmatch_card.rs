//! The authored `.DCMATCH` card.
//!
//! `.DCMATCH` is a pure keyword card: every field is named, so a deck that
//! omits one gets the documented default rather than a positional reading
//! that depends on field order. Unknown keywords are refused by name — a
//! dropped statistical control would silently answer a different question
//! from the one the deck asked.

use super::analysis_card_scan::*;
use super::*;

const CARD: AnalysisCard = AnalysisCard::DcMatch;

/// Parse `.DCMATCH OUT=V(node[,ref])|I(element) [MISMATCH=yes|no]
/// [PROCESS=yes|no] [CONTRIBUTORS=<n>] [THRESHOLD=<share>] [SIGMA=<k>]
/// [MOMENT_RELTOL=<tolerance>] [MOMENT_MAX_POINTS=<n>]`.
pub(super) fn parse_dcmatch_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<AnalysisCommand, ParseError> {
    let mut output = None;
    let mut mismatch = None;
    let mut process = None;
    let mut contributors = None;
    let mut threshold = None;
    let mut sigma = None;
    let mut moment_tolerance = None;
    let mut moment_points = None;

    loop {
        skip_commas(stream);
        if at_card_end(stream) {
            break;
        }
        let Some(keyword) = take_keyword(stream) else {
            // A bare token before `OUT=` is a deck written in the positional
            // style of `.TF`, so it is answered by naming the field it meant
            // rather than by calling the probe a stray token.
            return Err(card_error(
                CARD,
                line_num,
                if output.is_none() {
                    AnalysisCardIssue::MissingField { field: "OUT" }
                } else {
                    AnalysisCardIssue::TrailingToken {
                        token: stream.peek().lexeme.clone(),
                    }
                },
            ));
        };
        match keyword.as_str() {
            "MOMENT_RELTOL" => bind_once(
                &mut moment_tolerance,
                card_number(
                    stream,
                    line_num,
                    params,
                    CARD,
                    "MOMENT_RELTOL",
                    "a relative tolerance in (0, 0.1]",
                    |value| value > 0.0 && value <= 0.1,
                )?,
                CARD,
                line_num,
                "MOMENT_RELTOL",
            )?,
            "MOMENT_MAX_POINTS" => bind_once(
                &mut moment_points,
                card_count(stream, line_num, params, CARD, "MOMENT_MAX_POINTS", 1024)?,
                CARD,
                line_num,
                "MOMENT_MAX_POINTS",
            )?,
            "OUT" => bind_once(
                &mut output,
                card_dcmatch_probe(stream, line_num)?,
                CARD,
                line_num,
                "OUT",
            )?,
            "MISMATCH" => bind_once(
                &mut mismatch,
                card_bool(stream, line_num, CARD, "MISMATCH")?,
                CARD,
                line_num,
                "MISMATCH",
            )?,
            "PROCESS" => bind_once(
                &mut process,
                card_bool(stream, line_num, CARD, "PROCESS")?,
                CARD,
                line_num,
                "PROCESS",
            )?,
            // Zero is the authored spelling for "every contributor", so the
            // count has no minimum of its own.
            "CONTRIBUTORS" => bind_once(
                &mut contributors,
                card_count(stream, line_num, params, CARD, "CONTRIBUTORS", 0)?,
                CARD,
                line_num,
                "CONTRIBUTORS",
            )?,
            "THRESHOLD" => bind_once(
                &mut threshold,
                card_number(
                    stream,
                    line_num,
                    params,
                    CARD,
                    "THRESHOLD",
                    "a variance share in [0, 1]",
                    |value| (0.0..=1.0).contains(&value),
                )?,
                CARD,
                line_num,
                "THRESHOLD",
            )?,
            "SIGMA" => bind_once(
                &mut sigma,
                card_number(
                    stream,
                    line_num,
                    params,
                    CARD,
                    "SIGMA",
                    "a positive multiple of sigma",
                    |value| value > 0.0,
                )?,
                CARD,
                line_num,
                "SIGMA",
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

    let Some((output_node, reference_node, output_is_current)) = output else {
        return Err(card_error(
            CARD,
            line_num,
            AnalysisCardIssue::MissingField { field: "OUT" },
        ));
    };
    let mismatch = mismatch.unwrap_or(true);
    let process = process.unwrap_or(false);
    // Both scopes off leaves nothing to vary, so the card describes no study.
    // Refusing here keeps the analysis layer's own "nothing to vary" refusal
    // for the deck whose statistics block declares neither scope.
    if !mismatch && !process {
        return Err(card_error(
            CARD,
            line_num,
            AnalysisCardIssue::InvalidChoice {
                field: "PROCESS",
                value: "no".to_owned(),
                expected: "yes when MISMATCH=no, because a card with neither scope enabled has \
                           nothing to vary",
            },
        ));
    }
    Ok(AnalysisCommand::DcMatch(Box::new(DcMatchCard {
        moments: crate::netlist::StatisticalMomentOptions {
            relative_tolerance: moment_tolerance
                .unwrap_or(crate::netlist::StatisticalMomentOptions::default().relative_tolerance),
            max_points: moment_points
                .unwrap_or(crate::netlist::StatisticalMomentOptions::default().max_points),
        },
        output_node,
        reference_node,
        output_is_current,
        mismatch,
        process,
        contributor_limit: contributors.unwrap_or(DcMatchCard::DEFAULT_CONTRIBUTORS),
        threshold: threshold.unwrap_or(0.0),
        sigma_multiplier: sigma.unwrap_or(1.0),
    })))
}

/// Read `OUT=V(node[,ref])`, `OUT=I(element)` or a bare node name.
fn card_dcmatch_probe(
    stream: &mut TokenStream,
    line: usize,
) -> Result<(String, Option<String>, bool), ParseError> {
    if let Some(element) = take_current_probe(stream, line)? {
        return Ok((element, None, true));
    }
    let (node, reference) = card_output_probe(stream, line, CARD)?;
    Ok((node, reference, false))
}

/// Consume `I(element)` when the stream is positioned on one.
///
/// The lexer may hand the accessor back merged with its operand, so both
/// spellings are recognized before the voltage probe is tried; anything else
/// leaves the stream untouched.
fn take_current_probe(stream: &mut TokenStream, line: usize) -> Result<Option<String>, ParseError> {
    let TokenKind::Ident(spelling) = &stream.peek().kind else {
        return Ok(None);
    };
    let upper = spelling.to_ascii_uppercase();
    if upper == "I" {
        if !matches!(stream.peek_n(1).kind, TokenKind::LParen) {
            return Ok(None);
        }
        stream.advance();
        stream.advance();
        let element = card_name(stream, line, CARD, "OUT")?;
        if !stream.consume(&TokenKind::RParen) {
            return Err(card_error(
                CARD,
                line,
                AnalysisCardIssue::InvalidChoice {
                    field: "OUT",
                    value: stream.peek().lexeme.clone(),
                    expected: "a closing ')' for I(element)",
                },
            ));
        }
        return Ok(Some(element.to_ascii_uppercase()));
    }
    let Some(inner) = upper
        .strip_prefix("I(")
        .and_then(|rest| rest.strip_suffix(')'))
        .filter(|inner| !inner.is_empty())
    else {
        return Ok(None);
    };
    let element = inner.to_owned();
    stream.advance();
    Ok(Some(element))
}

#[cfg(test)]
mod tests {
    use crate::netlist::{AnalysisCommand, DcMatchCard, Netlist};

    fn card(source: &str) -> DcMatchCard {
        let netlist = Netlist::parse(&format!(
            "dcmatch card\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n{source}\n.end\n"
        ))
        .expect("card parses");
        match netlist.analyses.first().expect("one analysis") {
            AnalysisCommand::DcMatch(card) => (**card).clone(),
            other => panic!("expected .DCMATCH, got {other:?}"),
        }
    }

    fn refusal(source: &str) -> String {
        Netlist::parse(&format!(
            "dcmatch card\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n{source}\n.end\n"
        ))
        .expect_err("card is refused")
        .to_string()
    }

    #[test]
    fn a_dcmatch_card_carries_every_keyword() {
        let bound = card(
            ".DCMATCH OUT=V(out,in) MISMATCH=no PROCESS=yes CONTRIBUTORS=3 \
             THRESHOLD=0.25 SIGMA=6",
        );
        assert_eq!(bound.output_node, "OUT");
        assert_eq!(bound.reference_node.as_deref(), Some("IN"));
        assert!(!bound.output_is_current);
        assert!(!bound.mismatch);
        assert!(bound.process);
        assert_eq!(bound.contributor_limit, 3);
        assert_eq!(bound.threshold, 0.25);
        assert_eq!(bound.sigma_multiplier, 6.0);
    }

    #[test]
    fn a_dcmatch_card_defaults_every_optional_keyword() {
        let bound = card(".DCMATCH OUT=V(out)");
        assert_eq!(bound, DcMatchCard::voltage_probe("OUT"));
        assert!(bound.mismatch);
        assert!(!bound.process);
        assert_eq!(bound.contributor_limit, 10);
        assert_eq!(bound.threshold, 0.0);
        assert_eq!(bound.sigma_multiplier, 1.0);
    }

    #[test]
    fn a_dcmatch_card_reads_a_branch_current_probe() {
        for spelling in [".DCMATCH OUT=I(V1)", ".DCMATCH OUT=I( V1 )"] {
            let bound = card(spelling);
            assert!(bound.output_is_current, "{spelling}");
            assert_eq!(bound.output_node, "V1", "{spelling}");
            assert!(bound.reference_node.is_none(), "{spelling}");
        }
    }

    #[test]
    fn a_dcmatch_card_refuses_a_key_it_does_not_define() {
        assert!(
            refusal(".DCMATCH OUT=V(out) CORNERS=4").contains("unknown keyword 'CORNERS'"),
            "{}",
            refusal(".DCMATCH OUT=V(out) CORNERS=4")
        );
    }

    #[test]
    fn a_dcmatch_card_refuses_a_repeated_or_missing_field() {
        assert!(
            refusal(".DCMATCH OUT=V(out) SIGMA=3 SIGMA=6")
                .contains("SIGMA authored more than once")
        );
        assert!(refusal(".DCMATCH MISMATCH=yes").contains("missing required field OUT"));
        // Every field is named, so the positional spelling `.TF` accepts is
        // answered by naming the field that was meant.
        assert!(refusal(".DCMATCH V(out)").contains("missing required field OUT"));
        assert!(
            refusal(".DCMATCH OUT=V(out) V(in)").contains("unexpected trailing token"),
            "{}",
            refusal(".DCMATCH OUT=V(out) V(in)")
        );
    }

    #[test]
    fn a_dcmatch_card_refuses_a_field_outside_its_range() {
        assert!(refusal(".DCMATCH OUT=V(out) SIGMA=0").contains("SIGMA must be a positive"));
        assert!(
            refusal(".DCMATCH OUT=V(out) THRESHOLD=1.5")
                .contains("THRESHOLD must be a variance share in [0, 1]")
        );
    }

    #[test]
    fn a_dcmatch_card_with_neither_scope_has_nothing_to_vary() {
        let message = refusal(".DCMATCH OUT=V(out) MISMATCH=no PROCESS=no");
        assert!(
            message.contains("a card with neither scope enabled has nothing to vary"),
            "{message}"
        );
    }
}
