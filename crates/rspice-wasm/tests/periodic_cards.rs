//! Authored periodic large-signal cards on the browser deck route.
//!
//! `.PSS`, `.PAC`, `.HB` and `.ENVELOPE` execute and publish shared result
//! documents. `.PNOISE` is refused by name because `rspice-core` exposes no
//! runner that returns the `PnoiseResult` the shared document is built from,
//! and a refusal must never be softened into a different family's result.

use rspice_wasm::run_authored_deck_document_detailed;

const CIRCUIT: &str = "periodic browser deck\n\
V1 in 0 SIN(0 0.1 1G)\n\
R1 in out 1k\n\
C1 out 0 1p\n";

fn deck(cards: &str) -> String {
    format!("{CIRCUIT}{cards}.END\n")
}

fn refusal(cards: &str) -> rspice_wasm::WasmError {
    *run_authored_deck_document_detailed(&deck(cards))
        .expect_err("an unroutable authored card must be refused")
}

#[test]
fn an_authored_pnoise_card_is_refused_with_its_analysis_identity() {
    let error = refusal(".PSS FUND=1G\n.PNOISE DEC 3 1k 100k OUT=V(out)\n");
    assert_eq!(error.kind, "unsupported_deck_analysis");
    assert_eq!(error.category, "unsupported_feature");
    assert!(
        error.message.contains(".PNOISE") && error.message.contains("pnoise-001"),
        "the refusal names the card and its canonical instance: {}",
        error.message
    );
    assert!(
        error.message.contains("PnoiseResult"),
        "the refusal names the missing core API: {}",
        error.message
    );
}

#[test]
fn an_unroutable_card_is_refused_before_a_supported_analysis_publishes() {
    // The `.OP` alone would produce a document; the deck must publish none.
    let error = refusal(".OP\n.PSS FUND=1G\n.PNOISE DEC 3 1k 100k OUT=V(out)\n");
    assert_eq!(error.kind, "unsupported_deck_analysis");
    assert!(
        error.message.contains("pnoise-001"),
        "the ordinal survives preceding supported analyses: {}",
        error.message
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
