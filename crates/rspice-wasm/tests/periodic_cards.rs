//! Authored periodic large-signal cards on the browser deck route.
//!
//! `.PSS`, `.PAC`, `.HB`, `.ENVELOPE` and `.PNOISE` all execute and publish
//! shared result documents, each named by the identity the canonical plan
//! assigned it and — for the small-signal cards — bound to the carrier they
//! linearized around.

use rspice_wasm::run_authored_deck_document_detailed;

const CIRCUIT: &str = "periodic browser deck\n\
V1 in 0 SIN(0 0.1 1G)\n\
R1 in out 1k\n\
C1 out 0 1p\n";

fn deck(cards: &str) -> String {
    format!("{CIRCUIT}{cards}.END\n")
}

#[test]
fn an_authored_pnoise_card_publishes_a_document_bound_to_its_carrier() {
    let execution = run_authored_deck_document_detailed(&deck(
        ".PSS FUND=1G HARMS=3 POINTS=32 TSTABPERIODS=2\n\
         .PNOISE DEC 3 1k 100k OUT=V(out)\n",
    ))
    .expect("an authored .PNOISE executes on the browser deck route");
    let document = execution
        .results
        .iter()
        .find(|document| document.analysis().tag() == "pnoise-001")
        .expect("the PNoise result keeps its canonical identity");
    assert_eq!(
        document.result_kind(),
        rspice_core::execution::AnalysisResultKind::PNoise
    );
    assert_eq!(
        document.parent_analysis().map(|parent| parent.tag()),
        Some("pss-001".to_owned()),
        "a periodic-noise result must name the carrier it folded noise around"
    );
    assert!(
        document.point_count() > 0,
        "a periodic-noise sweep retains its offset grid"
    );
}

#[test]
fn a_preceding_analysis_does_not_change_the_pnoise_ordinal() {
    let execution = run_authored_deck_document_detailed(&deck(
        ".OP\n.PSS FUND=1G HARMS=3 POINTS=32 TSTABPERIODS=2\n\
         .PNOISE DEC 3 1k 100k OUT=V(out)\n",
    ))
    .expect("the deck executes");
    assert!(
        execution
            .results
            .iter()
            .any(|document| document.analysis().tag() == "pnoise-001"),
        "the ordinal is per family, so a preceding .OP does not shift it"
    );
}

#[test]
fn an_authored_pss_card_publishes_a_periodic_steady_state_document() {
    let execution = run_authored_deck_document_detailed(&deck(
        ".PSS FUND=1G HARMS=3 POINTS=32 TSTABPERIODS=2\n",
    ))
    .expect("an authored .PSS executes on the browser deck route");
    let document = execution
        .results
        .iter()
        .find(|document| document.analysis().tag() == "pss-001")
        .expect("the PSS result keeps its canonical identity");
    assert_eq!(
        document.result_kind(),
        rspice_core::execution::AnalysisResultKind::Pss
    );
    assert!(
        document.point_count() > 0,
        "a periodic steady state retains its time grid"
    );
}

#[test]
fn a_deck_without_a_periodic_card_still_publishes_its_document() {
    let execution = run_authored_deck_document_detailed(&deck(".OP\n"))
        .expect("a supported deck still executes");
    assert!(
        !execution.results.is_empty(),
        "the supported control deck must publish its results"
    );
}
