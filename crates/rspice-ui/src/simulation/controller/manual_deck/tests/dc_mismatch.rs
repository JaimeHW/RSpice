//! What a hand-written `.DCMATCH` deck plans.
//!
//! Two routes reach one DC mismatch run: the typed plan, and a deck someone
//! was handed. They have to agree, or a deck exported from the Studio and
//! reopened is a different analysis from the one that produced it, with the
//! same name on it.

use super::*;

/// A hand-written `.DCMATCH` card is read as a DC mismatch analysis.
///
/// The card used to be refused by name, which was honest while the kind
/// had no route; now that it runs, a deck someone was handed plans the
/// same analysis a typed plan does.
#[test]
fn a_manual_deck_with_a_dcmatch_card_is_read_as_dc_mismatch() {
    let specs = specs_for(
        "mismatch deck\n\
             V1 in 0 DC 1\n\
             R1 in out 10k\n\
             R2 out 0 10k\n\
             .DCMATCH OUT=V(out,in) MISMATCH=no PROCESS=yes CONTRIBUTORS=3 THRESHOLD=0.25 \
             SIGMA=6\n\
             .end\n",
    );
    let [
        AnalysisSpec::DcMismatch {
            output_expression,
            sigma_multiplier,
            contributor_limit,
            include_process,
            include_mismatch,
            normalized_contributions,
            contribution_threshold,
        },
    ] = specs.as_slice()
    else {
        panic!("a .DCMATCH card plans one DC mismatch analysis: {specs:?}");
    };
    // The parser canonicalizes the probe to upper case.
    assert_eq!(output_expression, "V(OUT,IN)");
    assert_eq!(*sigma_multiplier, 6.0);
    assert_eq!(*contributor_limit, 3);
    assert!(*include_process);
    assert!(!*include_mismatch);
    assert!(
        *normalized_contributions,
        "the card has no operand for the report basis, so the product default applies"
    );
    assert_eq!(*contribution_threshold, Some(0.25));
}

/// A bare card reads as the engine's own defaults, not as this crate's.
#[test]
fn a_bare_dcmatch_card_reads_as_the_engine_defaults() {
    let specs = specs_for(
        "bare mismatch\n\
             V1 in 0 DC 1\n\
             R1 in out 10k\n\
             R2 out 0 10k\n\
             .DCMATCH OUT=V(out)\n\
             .end\n",
    );
    let [
        AnalysisSpec::DcMismatch {
            sigma_multiplier,
            contributor_limit,
            include_process,
            include_mismatch,
            contribution_threshold,
            ..
        },
    ] = specs.as_slice()
    else {
        panic!("a bare .DCMATCH card plans one analysis: {specs:?}");
    };
    assert_eq!(*sigma_multiplier, 1.0);
    assert_eq!(
        *contributor_limit,
        rspice_core::netlist::DcMatchCard::DEFAULT_CONTRIBUTORS
    );
    assert!(*include_mismatch);
    assert!(!*include_process);
    assert_eq!(
        *contribution_threshold, None,
        "an unstated threshold is the unauthored card, not a stated zero"
    );

    // And a fresh Studio draft is that same specification, which is the
    // reason the draft's defaults are the card's.
    let state = AppState::default();
    let draft = crate::simulation::plan::AnalysisDraft::for_kind(
        crate::simulation::plan::AnalysisKind::DcMismatch,
    );
    let authored = SimulationController::new()
        .build_manifest_preview_spec(&state, &draft)
        .expect("a default DC mismatch draft builds a specification")
        .expect("DC mismatch is a manifest kind");
    let AnalysisSpec::DcMismatch {
        sigma_multiplier: draft_sigma,
        contributor_limit: draft_limit,
        include_process: draft_process,
        include_mismatch: draft_mismatch,
        contribution_threshold: draft_threshold,
        ..
    } = authored
    else {
        panic!("the draft builds a DC mismatch specification");
    };
    assert_eq!(draft_sigma, *sigma_multiplier);
    assert_eq!(draft_limit, *contributor_limit);
    assert_eq!(draft_process, *include_process);
    assert_eq!(draft_mismatch, *include_mismatch);
    assert_eq!(draft_threshold, *contribution_threshold);
}

/// The card the Studio writes, read back by the deck reader, is the
/// specification it was written from.
///
/// Canonical upper-case probes, because that is what the engine echoes
/// and therefore what a Studio-exported deck carries. A lower-case probe
/// is asserted separately rather than hidden: the parser canonicalizes
/// it, so it reads back upper-cased, and pretending otherwise would be
/// pretending the round trip is something it is not.
#[test]
fn the_studio_dcmatch_card_round_trips_through_the_manual_deck_reader() {
    for probe in ["V(OUT,IN)", "V(OUT)", "I(V1)"] {
        for limit in [0_usize, 3] {
            for threshold in [None, Some(0.25)] {
                let authored = AnalysisSpec::DcMismatch {
                    output_expression: probe.to_owned(),
                    sigma_multiplier: 6.0,
                    contributor_limit: limit,
                    include_process: true,
                    include_mismatch: true,
                    normalized_contributions: true,
                    contribution_threshold: threshold,
                };
                let card = SimulationController::build_dc_mismatch_command(&authored)
                    .expect("the plan writes its card");
                let specs = specs_for(&format!(
                    "round trip\n\
                         V1 in 0 DC 1\n\
                         R1 in out 10k\n\
                         R2 out 0 10k\n\
                         {card}\n\
                         .end\n"
                ));
                assert_eq!(
                    specs.as_slice(),
                    &[authored],
                    "the card `{card}` read back as something else"
                );
            }
        }
    }

    // A lower-case probe is the same request upper-cased, which is the
    // engine's canonicalization and not a loss.
    let specs = specs_for(
        "lower case probe\n\
             V1 in 0 DC 1\n\
             R1 in out 10k\n\
             R2 out 0 10k\n\
             .dcmatch OUT=v(out) MISMATCH=yes PROCESS=no CONTRIBUTORS=10 SIGMA=1\n\
             .end\n",
    );
    let [
        AnalysisSpec::DcMismatch {
            output_expression, ..
        },
    ] = specs.as_slice()
    else {
        panic!("one analysis: {specs:?}");
    };
    assert_eq!(output_expression, "V(OUT)");
}

/// Two cards in one deck are two analyses, each running its own.
///
/// The run executes the specification's card rather than picking one out
/// of the deck, so a deck that asks two questions gets two answers.
#[test]
fn two_dcmatch_cards_in_one_deck_queue_two_analyses() {
    let specs = specs_for(
        "two questions\n\
             V1 in 0 DC 1\n\
             R1 in out 10k\n\
             R2 out 0 10k\n\
             .DCMATCH OUT=V(out) SIGMA=1\n\
             .DCMATCH OUT=I(V1) SIGMA=6 CONTRIBUTORS=0\n\
             .end\n",
    );
    let [first, second] = specs.as_slice() else {
        panic!("two cards queue two analyses: {specs:?}");
    };
    let probe = |spec: &AnalysisSpec| match spec {
        AnalysisSpec::DcMismatch {
            output_expression,
            sigma_multiplier,
            contributor_limit,
            ..
        } => (
            output_expression.clone(),
            *sigma_multiplier,
            *contributor_limit,
        ),
        other => panic!("expected a DC mismatch specification, got {other:?}"),
    };
    assert_eq!(
        probe(first),
        (
            "V(OUT)".to_owned(),
            1.0,
            rspice_core::netlist::DcMatchCard::DEFAULT_CONTRIBUTORS
        )
    );
    assert_eq!(probe(second), ("I(V1)".to_owned(), 6.0, 0));
}
