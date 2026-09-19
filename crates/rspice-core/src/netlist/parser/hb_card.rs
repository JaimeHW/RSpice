//! The authored `.HB` card.
//!
//! `.HB` is a positional-then-keyword card, like `.PSS`: the large-signal tone
//! frequencies are positional and at least one is required, and every control
//! the solve reads follows as a named keyword in any order. The positional
//! loop stops at the first `IDENT =` pair, which is the one rule that keeps
//! the two forms disjoint across every card.
//!
//! Nothing is defaulted here. An unauthored keyword stays `None` on the
//! [`HbCard`] so that
//! [`HbConfig::from_hb_card`](crate::analysis::HbConfig::from_hb_card) — the
//! single place a `.HB` card becomes a configuration — applies the engine's
//! own default. A parser that filled the gaps would be a second copy of the
//! defaults, free to disagree with the first.

use super::analysis_card_scan::*;
use super::*;

const CARD: AnalysisCard = AnalysisCard::Hb;

/// Parse `.HB f1 [f2 ...] [HARMS=n[,n...]] [SOURCE<k>=<name>]
/// [OVERSAMPLE=<n>] [POINTS=<n>] [MAXMIXING=<n>] [RELTOL=<x>] [ABSTOL=<x>]
/// [MAXITER=<n>] [DAMPING=<x>] [MINDAMPING=<x>] [SOLVER=AUTO|KRYLOV]
/// [GMRESRESTART=<n>] [SOURCESTEPPING=<bool>] [EXACTJACOBIAN=<bool>]
/// [VERBOSE=<bool>]`.
pub(super) fn parse_hb_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<AnalysisCommand, ParseError> {
    let frequencies = parse_hb_frequencies(stream, line_num, params)?;

    let mut card = HbCard {
        frequencies,
        ..HbCard::default()
    };

    loop {
        skip_commas(stream);
        if at_card_end(stream) {
            break;
        }
        let Some(keyword) = take_keyword(stream) else {
            // The positional loop already refused a non-numeric token, so a
            // bare token here follows the frequencies and is a keyword the
            // author forgot to give a value.
            return Err(card_error(
                CARD,
                line_num,
                AnalysisCardIssue::TrailingToken {
                    token: stream.peek().lexeme.clone(),
                },
            ));
        };
        if let Some(index) = tone_source_index(&keyword) {
            bind_tone_source(stream, line_num, &mut card, &keyword, index)?;
            continue;
        }
        match keyword.as_str() {
            "HARMS" => {
                if !card.harmonics.is_empty() {
                    return Err(card_error(
                        CARD,
                        line_num,
                        AnalysisCardIssue::DuplicateKeyword { keyword: "HARMS" },
                    ));
                }
                card.harmonics = card_count_list(stream, line_num, params, "HARMS")?;
                if card.harmonics.len() != 1 && card.harmonics.len() != card.frequencies.len() {
                    return Err(card_error(
                        CARD,
                        line_num,
                        AnalysisCardIssue::InvalidChoice {
                            field: "HARMS",
                            value: card
                                .harmonics
                                .iter()
                                .map(usize::to_string)
                                .collect::<Vec<_>>()
                                .join(","),
                            expected: "one harmonic count, or one for each tone the card lists",
                        },
                    ));
                }
            }
            "OVERSAMPLE" => bind_once(
                &mut card.oversample,
                card_count(stream, line_num, params, CARD, "OVERSAMPLE", 2)?,
                CARD,
                line_num,
                "OVERSAMPLE",
            )?,
            // An even grid cannot carry a bilateral spectrum and a grid below
            // three cannot carry one harmonic beside DC, so both are refused
            // here rather than at the FFT plan.
            "POINTS" => bind_once(
                &mut card.collocation_points,
                card_odd_count(stream, line_num, params, "POINTS")?,
                CARD,
                line_num,
                "POINTS",
            )?,
            "MAXMIXING" => bind_once(
                &mut card.max_mixing_order,
                card_count(stream, line_num, params, CARD, "MAXMIXING", 1)?,
                CARD,
                line_num,
                "MAXMIXING",
            )?,
            "RELTOL" => bind_once(
                &mut card.reltol,
                card_number(
                    stream,
                    line_num,
                    params,
                    CARD,
                    "RELTOL",
                    "a positive tolerance",
                    |value| value > 0.0,
                )?,
                CARD,
                line_num,
                "RELTOL",
            )?,
            "ABSTOL" => bind_once(
                &mut card.abstol,
                card_number(
                    stream,
                    line_num,
                    params,
                    CARD,
                    "ABSTOL",
                    "a positive tolerance",
                    |value| value > 0.0,
                )?,
                CARD,
                line_num,
                "ABSTOL",
            )?,
            "MAXITER" => bind_once(
                &mut card.max_iterations,
                card_count(stream, line_num, params, CARD, "MAXITER", 1)?,
                CARD,
                line_num,
                "MAXITER",
            )?,
            // The Armijo search scales its first trial by this, so a factor
            // above one is not a damped step and one below a tenth asks for
            // the floor MINDAMPING states instead.
            "DAMPING" => bind_once(
                &mut card.damping,
                card_number(
                    stream,
                    line_num,
                    params,
                    CARD,
                    "DAMPING",
                    "a step scale in [0.1, 1]",
                    |value| (0.1..=1.0).contains(&value),
                )?,
                CARD,
                line_num,
                "DAMPING",
            )?,
            "MINDAMPING" => bind_once(
                &mut card.min_damping,
                card_number(
                    stream,
                    line_num,
                    params,
                    CARD,
                    "MINDAMPING",
                    "a positive step scale no greater than DAMPING",
                    |value| value > 0.0,
                )?,
                CARD,
                line_num,
                "MINDAMPING",
            )?,
            "SOLVER" => bind_once(
                &mut card.use_krylov,
                card_hb_solver(stream, line_num)?,
                CARD,
                line_num,
                "SOLVER",
            )?,
            "GMRESRESTART" => bind_once(
                &mut card.gmres_restart,
                card_count(stream, line_num, params, CARD, "GMRESRESTART", 1)?,
                CARD,
                line_num,
                "GMRESRESTART",
            )?,
            "SOURCESTEPPING" => bind_once(
                &mut card.source_stepping,
                card_bool(stream, line_num, CARD, "SOURCESTEPPING")?,
                CARD,
                line_num,
                "SOURCESTEPPING",
            )?,
            "EXACTJACOBIAN" => bind_once(
                &mut card.use_exact_jacobian,
                card_bool(stream, line_num, CARD, "EXACTJACOBIAN")?,
                CARD,
                line_num,
                "EXACTJACOBIAN",
            )?,
            "VERBOSE" => bind_once(
                &mut card.verbose,
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

    Ok(AnalysisCommand::Hb(Box::new(card)))
}

/// Read the positional tone list, stopping at the first keyword pair.
fn parse_hb_frequencies(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<Vec<Value>, ParseError> {
    let mut frequencies = Vec::new();
    loop {
        skip_commas(stream);
        if at_card_end(stream) || at_keyword(stream) {
            break;
        }
        let frequency = expect_value(stream, line_num, params)?;
        if !frequency.is_finite() || frequency <= 0.0 {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    ".HB frequencies must be positive finite numbers, found {frequency}"
                ),
            });
        }
        frequencies.push(frequency);
    }
    if frequencies.is_empty() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: ".HB requires at least one positive frequency".to_string(),
        });
    }
    Ok(frequencies)
}

/// Tone index of a `SOURCE<k>` keyword, or `None` when the keyword is not one.
///
/// `SOURCE` with no index and `SOURCE0` are not tone selectors: the card
/// numbers its tones from one, so both fall through to the unknown-keyword
/// answer that names the exact spelling back.
fn tone_source_index(keyword: &str) -> Option<usize> {
    let digits = keyword.strip_prefix("SOURCE")?;
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    digits.parse::<usize>().ok().filter(|index| *index >= 1)
}

/// Bind one `SOURCE<k>=<name>` to the tone it numbers.
fn bind_tone_source(
    stream: &mut TokenStream,
    line_num: usize,
    card: &mut HbCard,
    keyword: &str,
    index: usize,
) -> Result<(), ParseError> {
    if index > card.frequencies.len() {
        return Err(card_error(
            CARD,
            line_num,
            AnalysisCardIssue::InvalidChoice {
                field: "SOURCE<k>",
                value: keyword.to_owned(),
                expected: "a tone index the card lists a frequency for",
            },
        ));
    }
    let name = card_name(stream, line_num, CARD, "SOURCE<k>")?;
    if card.sources.is_empty() {
        card.sources = vec![None; card.frequencies.len()];
    }
    let slot = &mut card.sources[index - 1];
    if slot.is_some() {
        return Err(card_error(
            CARD,
            line_num,
            AnalysisCardIssue::InvalidChoice {
                field: "SOURCE<k>",
                value: keyword.to_owned(),
                expected: "authored at most once for each tone index",
            },
        ));
    }
    *slot = Some(name.to_ascii_uppercase());
    Ok(())
}

/// Read `n` or `n,n,...`, parenthesized or bare, each at least one.
///
/// A bare list only continues past a comma when a number follows it, so a
/// deck that separates its keywords with commas — which SPICE treats as
/// whitespace everywhere else — is read the way it was written.
fn card_count_list(
    stream: &mut TokenStream,
    line: usize,
    params: &ParamContext,
    field: &'static str,
) -> Result<Vec<usize>, ParseError> {
    let mut counts = Vec::new();
    if stream.consume(&TokenKind::LParen) {
        loop {
            counts.push(card_count(stream, line, params, CARD, field, 1)?);
            if stream.consume(&TokenKind::RParen) {
                break;
            }
            if !stream.consume(&TokenKind::Comma) {
                return Err(card_error(
                    CARD,
                    line,
                    AnalysisCardIssue::TrailingToken {
                        token: stream.peek().lexeme.clone(),
                    },
                ));
            }
        }
    } else {
        counts.push(card_count(stream, line, params, CARD, field, 1)?);
        while matches!(stream.peek().kind, TokenKind::Comma)
            && matches!(
                stream.peek_n(1).kind,
                TokenKind::Number(_) | TokenKind::Expression(_)
            )
        {
            stream.advance();
            counts.push(card_count(stream, line, params, CARD, field, 1)?);
        }
    }
    Ok(counts)
}

/// Read an odd collocation-grid size of at least three.
fn card_odd_count(
    stream: &mut TokenStream,
    line: usize,
    params: &ParamContext,
    field: &'static str,
) -> Result<usize, ParseError> {
    const EXPECTED: &str = "an odd whole number >= 3";
    let value = card_number(stream, line, params, CARD, field, EXPECTED, |value| {
        value >= 3.0 && value.fract() == 0.0 && value <= usize::MAX as Value && value % 2.0 == 1.0
    })?;
    Ok(value as usize)
}

/// Read `SOLVER=AUTO|KRYLOV` as the Krylov force flag.
fn card_hb_solver(stream: &mut TokenStream, line: usize) -> Result<bool, ParseError> {
    let spelling = card_name(stream, line, CARD, "SOLVER")?;
    match spelling.to_ascii_uppercase().as_str() {
        "AUTO" => Ok(false),
        "KRYLOV" => Ok(true),
        _ => Err(card_error(
            CARD,
            line,
            AnalysisCardIssue::InvalidChoice {
                field: "SOLVER",
                value: spelling,
                // NEWTON is deliberately absent: both spellings are Newton
                // solves and this keyword picks the inner linear solve, so a
                // card asking for "newton" is asking for something the
                // vocabulary cannot mean.
                expected: "AUTO or KRYLOV",
            },
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::netlist::{AnalysisCommand, HbCard, Netlist};

    const FIXTURE: &str = "hb card\nV1 in 0 SIN(0 1 1G)\nV2 lo 0 SIN(0 1 800MEG)\n\
                           R1 in out 1k\nR2 lo out 1k\nC1 out 0 1p\n";

    fn card(source: &str) -> HbCard {
        let netlist = Netlist::parse(&format!("{FIXTURE}{source}\n.end\n"))
            .unwrap_or_else(|error| panic!("card parses: {error}"));
        match netlist.analyses.first().expect("one analysis") {
            AnalysisCommand::Hb(card) => (**card).clone(),
            other => panic!("expected .HB, got {other:?}"),
        }
    }

    fn refusal(source: &str) -> String {
        Netlist::parse(&format!("{FIXTURE}{source}\n.end\n"))
            .expect_err("card is refused")
            .to_string()
    }

    /// The card a deck written before the keywords existed parses to: the
    /// tones, and no control authored at all.
    #[test]
    fn an_hb_card_without_keywords_parses_to_its_tones_alone() {
        let bound = card(".HB 900MEG 800MEG");
        assert_eq!(bound.frequencies, [9.0e8, 8.0e8]);
        assert_eq!(
            bound,
            HbCard {
                frequencies: vec![9.0e8, 8.0e8],
                ..HbCard::default()
            },
            "a keyword-less card authors nothing but its tones"
        );
    }

    /// Every keyword of the vocabulary, read off one card.
    #[test]
    fn an_hb_card_carries_every_keyword_it_defines() {
        let bound = card(
            ".HB 900MEG 800MEG HARMS=3,4 SOURCE1=V1 SOURCE2=v2 OVERSAMPLE=4 POINTS=101 \
             MAXMIXING=7 RELTOL=1e-8 ABSTOL=1e-14 MAXITER=42 DAMPING=0.5 MINDAMPING=0.02 \
             SOLVER=krylov GMRESRESTART=16 SOURCESTEPPING=yes EXACTJACOBIAN=no VERBOSE=1",
        );
        assert_eq!(bound.frequencies, [9.0e8, 8.0e8]);
        assert_eq!(bound.harmonics, [3, 4]);
        assert_eq!(
            bound.sources,
            vec![Some("V1".to_owned()), Some("V2".to_owned())],
            "a source name is upper-cased like every other authored instance"
        );
        assert_eq!(bound.oversample, Some(4));
        assert_eq!(bound.collocation_points, Some(101));
        assert_eq!(bound.max_mixing_order, Some(7));
        assert_eq!(bound.reltol, Some(1.0e-8));
        assert_eq!(bound.abstol, Some(1.0e-14));
        assert_eq!(bound.max_iterations, Some(42));
        assert_eq!(bound.damping, Some(0.5));
        assert_eq!(bound.min_damping, Some(0.02));
        assert_eq!(bound.use_krylov, Some(true));
        assert_eq!(bound.gmres_restart, Some(16));
        assert_eq!(bound.source_stepping, Some(true));
        assert_eq!(bound.use_exact_jacobian, Some(false));
        assert_eq!(bound.verbose, Some(true));
    }

    /// The positional tone list ends at the first `IDENT =` pair, in either
    /// order of writing, and the keywords themselves have no order.
    #[test]
    fn the_tone_list_stops_at_the_first_keyword_pair() {
        assert_eq!(card(".HB 1G HARMS=3").frequencies, [1.0e9]);
        assert_eq!(
            card(".HB 900MEG, 800MEG, HARMS=3").frequencies,
            [9.0e8, 8.0e8]
        );
        let one_order = card(".HB 1G VERBOSE=yes HARMS=5");
        let other_order = card(".HB 1G HARMS=5 VERBOSE=yes");
        assert_eq!(one_order, other_order, "keyword order cannot change a card");
    }

    /// `HARMS=` is one count for every tone or one count per tone, written
    /// bare or parenthesized; any other length is refused against the tones.
    #[test]
    fn hb_harmonic_counts_broadcast_or_pair_with_the_tones() {
        assert_eq!(card(".HB 900MEG 800MEG HARMS=3").harmonics, [3]);
        assert_eq!(card(".HB 900MEG 800MEG HARMS=3,4").harmonics, [3, 4]);
        assert_eq!(card(".HB 900MEG 800MEG HARMS=(3,4)").harmonics, [3, 4]);
        assert_eq!(card(".HB 900MEG 800MEG HARMS=(3, 4)").harmonics, [3, 4]);
        assert!(
            refusal(".HB 900MEG 800MEG HARMS=3,4,5").contains(
                "HARMS must be one harmonic count, or one for each tone the card lists, \
                 got '3,4,5'"
            ),
            "{}",
            refusal(".HB 900MEG 800MEG HARMS=3,4,5")
        );
        assert!(refusal(".HB 1G HARMS=0").contains("HARMS must be a whole number >= 1"));
    }

    /// A tone source names its own tone and only its own: the slot a card
    /// leaves unwritten stays `None`, and an index past the tone list is
    /// refused rather than silently attached to the last tone.
    #[test]
    fn a_tone_source_keyword_routes_only_its_own_tone() {
        assert_eq!(
            card(".HB 900MEG 800MEG SOURCE2=V2").sources,
            vec![None, Some("V2".to_owned())]
        );
        assert_eq!(
            card(".HB 900MEG 800MEG SOURCE1=V1").sources,
            vec![Some("V1".to_owned()), None]
        );
        assert!(
            card(".HB 900MEG 800MEG").sources.is_empty(),
            "a card that names no source carries no slots at all"
        );
        assert!(
            refusal(".HB 900MEG 800MEG SOURCE3=V1")
                .contains("SOURCE<k> must be a tone index the card lists a frequency for"),
            "{}",
            refusal(".HB 900MEG 800MEG SOURCE3=V1")
        );
        assert!(
            refusal(".HB 900MEG 800MEG SOURCE1=V1 SOURCE1=V2")
                .contains("SOURCE<k> must be authored at most once for each tone index"),
            "{}",
            refusal(".HB 900MEG 800MEG SOURCE1=V1 SOURCE1=V2")
        );
        // `SOURCE` with no index and `SOURCE0` name no tone, so they are
        // answered as the keywords the card does not define.
        assert!(refusal(".HB 1G SOURCE=V1").contains("unknown keyword 'SOURCE'"));
        assert!(refusal(".HB 1G SOURCE0=V1").contains("unknown keyword 'SOURCE0'"));
    }

    /// One case per range in the card's vocabulary. Every one of these is a
    /// value the solver would otherwise have clamped or refused later, with
    /// the deck's own number no longer visible in the message.
    #[test]
    fn an_hb_card_refuses_a_value_outside_its_range() {
        for (source, expected) in [
            (
                ".HB 1G OVERSAMPLE=1",
                "OVERSAMPLE must be a whole number >= 2",
            ),
            (
                ".HB 1G POINTS=100",
                "POINTS must be an odd whole number >= 3",
            ),
            (".HB 1G POINTS=1", "POINTS must be an odd whole number >= 3"),
            (
                ".HB 1G MAXMIXING=0",
                "MAXMIXING must be a whole number >= 1",
            ),
            (".HB 1G RELTOL=0", "RELTOL must be a positive tolerance"),
            (".HB 1G ABSTOL=0", "ABSTOL must be a positive tolerance"),
            (".HB 1G MAXITER=0", "MAXITER must be a whole number >= 1"),
            (
                ".HB 1G DAMPING=1.5",
                "DAMPING must be a step scale in [0.1, 1]",
            ),
            (
                ".HB 1G DAMPING=0.05",
                "DAMPING must be a step scale in [0.1, 1]",
            ),
            (
                ".HB 1G MINDAMPING=0",
                "MINDAMPING must be a positive step scale no greater than DAMPING",
            ),
            (
                ".HB 1G GMRESRESTART=0",
                "GMRESRESTART must be a whole number >= 1",
            ),
            (
                ".HB 1G SOLVER=NEWTON",
                "SOLVER must be AUTO or KRYLOV, got 'NEWTON'",
            ),
            (
                ".HB 1G VERBOSE=maybe",
                "VERBOSE must be TRUE, FALSE, YES, NO, 1 or 0",
            ),
        ] {
            let message = refusal(source);
            assert!(
                message.contains(expected),
                "{source}: expected {expected:?} in {message}"
            );
        }
    }

    /// An unknown keyword, a repeated keyword and a token that is neither a
    /// tone nor a keyword pair are each refused by name.
    #[test]
    fn an_hb_card_refuses_an_unknown_or_repeated_keyword() {
        assert!(refusal(".HB 1G CORNERS=4").contains("unknown keyword 'CORNERS'"));
        assert!(
            refusal(".HB 1G HARMS=3 HARMS=4").contains("keyword HARMS authored more than once")
        );
        assert!(
            refusal(".HB 1G OVERSAMPLE=4 OVERSAMPLE=8")
                .contains("keyword OVERSAMPLE authored more than once")
        );
        assert!(
            refusal(".HB 1G VERBOSE=yes VERBOSE=no").contains("VERBOSE authored more than once")
        );
        // A `.HB` line is still refused for a tone that is not a positive
        // finite number, before any keyword is read.
        assert!(refusal(".HB 0").contains("positive finite numbers"));
        assert!(refusal(".HB HARMS=3").contains("at least one positive frequency"));
    }
}
