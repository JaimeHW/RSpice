//! The browser deck API has no result adapter for the periodic large-signal
//! family, so an authored `.PSS`/`.PAC`/`.PNOISE`/`.ENVELOPE` card is refused
//! with a typed error and no result document is published.
//!
//! `.PAC`, `.PNOISE` and `.ENVELOPE` always follow a `.PSS` or `.HB`, and both
//! of those are themselves unmapped here, so the first refusal a browser deck
//! reaches is always the upstream card.

use rspice_wasm::run_authored_deck_document_detailed;

const CIRCUIT: &str = "periodic browser refusal\n\
V1 in 0 SIN(0 1 1G)\n\
R1 in out 1k\n\
C1 out 0 1p\n";

fn deck(cards: &str) -> String {
    format!("{CIRCUIT}{cards}.END\n")
}

fn refusal(cards: &str) -> rspice_wasm::WasmError {
    *run_authored_deck_document_detailed(&deck(cards))
        .expect_err("an unsupported authored card must be refused")
}

#[test]
fn an_authored_pss_card_is_refused_with_its_analysis_identity() {
    let error = refusal(".PSS FUND=1G\n");
    assert_eq!(error.kind, "unsupported_deck_analysis");
    assert_eq!(error.category, "unsupported_feature");
    assert!(
        error.message.contains(".PSS") && error.message.contains("pss-001"),
        "refusal must name the card and its instance: {}",
        error.message
    );
}

#[test]
fn a_dependent_periodic_card_is_refused_at_its_upstream_analysis() {
    let error = refusal(".PSS FUND=1G\n.PAC DEC 5 1k 1meg INPUT=V1 OUT=V(out)\n");
    assert_eq!(error.kind, "unsupported_deck_analysis");
    assert!(
        error.message.contains(".PSS") && error.message.contains("pss-001"),
        "the first unmapped card is reported: {}",
        error.message
    );
}

#[test]
fn a_periodic_card_is_refused_before_a_supported_analysis_publishes() {
    // The `.OP` alone would produce a document; the deck must publish none.
    let error = refusal(".OP\n.PSS FUND=1G\n");
    assert_eq!(error.kind, "unsupported_deck_analysis");
    assert!(
        error.message.contains("pss-001"),
        "the ordinal must survive a preceding supported analysis: {}",
        error.message
    );
    assert!(
        run_authored_deck_document_detailed(&deck(".OP\n.PSS FUND=1G\n")).is_err(),
        "a refused deck must never yield a result document"
    );
}

#[test]
fn a_deck_without_a_periodic_card_still_publishes_its_document() {
    let document = run_authored_deck_document_detailed(&deck(".OP\n"))
        .expect("a supported deck still executes");
    assert!(
        !document.results.is_empty(),
        "the supported control deck must publish its results"
    );
}
