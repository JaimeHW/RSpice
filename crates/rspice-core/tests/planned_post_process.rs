//! Authored `.FOUR` cards resolve through one core entry point and arrive with
//! the identity the canonical plan minted for them.

use rspice_core::abort_signal::{ImmediateAbort, NoAbort};
use rspice_core::execution::{
    AnalysisResultDocument, DeckPlan, ResultPayload, evaluate_planned_fourier,
    evaluate_planned_fourier_with_abort,
};
use rspice_core::resource::ResourceLimits;
use rspice_core::{Engine, Netlist, SimulationConfig};

const DECK: &str = "Fourier post-process\n\
     V1 in 0 SIN(0 1 1k)\n\
     R1 in out 1k\n\
     C1 out 0 100n\n\
     .tran 1u 5m\n\
     .four 1k v(out) v(in)\n\
     .end\n";

fn plan_and_run(source: &str) -> (Netlist, DeckPlan, rspice_core::engine::TransientResult) {
    let netlist = Netlist::parse(source).expect("deck parses");
    let limits = ResourceLimits::default();
    let plan = DeckPlan::from_netlist(&netlist, &limits).expect("deck plans");
    let engine = Engine::new(SimulationConfig::default());
    let result = engine
        .run_tran_with_abort(&netlist, 5.0e-3, 1.0e-5, &NoAbort)
        .expect("transient runs");
    (netlist, plan, result)
}

#[test]
fn every_authored_four_operand_arrives_with_its_planned_identity_and_unit() {
    let (netlist, plan, result) = plan_and_run(DECK);
    let parent = plan
        .analyses()
        .iter()
        .find(|analysis| analysis.id().tag() == "tran-001")
        .expect("the deck authors one transient")
        .id();

    let spectra =
        evaluate_planned_fourier(&plan, &netlist, parent, &result, ResourceLimits::default())
            .expect("the core resolver evaluates every authored .FOUR operand");

    assert_eq!(
        spectra
            .iter()
            .map(|entry| (entry.analysis.tag(), entry.output.as_str()))
            .collect::<Vec<_>>(),
        [
            ("four-001".to_string(), "V(OUT)"),
            ("four-002".to_string(), "V(IN)"),
        ]
    );
    for entry in &spectra {
        assert_eq!(entry.parent.tag(), "tran-001");
        assert_eq!(
            entry.output_unit,
            rspice_core::execution::SignalUnit::Volt,
            "a voltage probe carries volts"
        );
        assert!(
            !entry.result.harmonics.is_empty(),
            "a resolved .FOUR operand has harmonics"
        );
    }
}

#[test]
fn a_planned_fourier_result_publishes_the_shared_document() {
    let (netlist, plan, result) = plan_and_run(DECK);
    let parent = plan.analyses()[0].id();
    let spectra =
        evaluate_planned_fourier(&plan, &netlist, parent, &result, ResourceLimits::default())
            .expect("resolution succeeds");
    let first = spectra.first().expect("at least one spectrum");
    let document = AnalysisResultDocument::from_fourier(
        first.analysis,
        first.parent,
        &first.output,
        first.output_unit.clone(),
        &first.result,
    )
    .expect("the shared Fourier document accepts exactly what the runner returns")
    .build()
    .expect("document builds");
    assert_eq!(document.analysis().tag(), "four-001");
    assert_eq!(
        document.parent_analysis().map(|id| id.tag()),
        Some("tran-001".to_string())
    );
    let ResultPayload::Fourier(payload) = document.payload() else {
        panic!("a .FOUR card projects a Fourier payload");
    };
    assert_eq!(payload.output, "V(OUT)");
}

#[test]
fn a_deck_with_no_four_card_resolves_to_no_spectra() {
    let (netlist, plan, result) = plan_and_run(
        "No Fourier\n\
         V1 in 0 SIN(0 1 1k)\n\
         R1 in 0 1k\n\
         .tran 1u 5m\n\
         .end\n",
    );
    let parent = plan.analyses()[0].id();
    assert!(
        evaluate_planned_fourier(&plan, &netlist, parent, &result, ResourceLimits::default())
            .expect("resolution succeeds")
            .is_empty()
    );
}

#[test]
fn planned_fourier_resolution_honours_its_abort_source() {
    let (netlist, plan, result) = plan_and_run(DECK);
    let parent = plan.analyses()[0].id();
    let error = evaluate_planned_fourier_with_abort(
        &plan,
        &netlist,
        parent,
        &result,
        ResourceLimits::default(),
        &ImmediateAbort,
    )
    .expect_err("an aborted resolution must not publish spectra");
    assert!(
        matches!(error, rspice_core::engine::SimulationError::Aborted),
        "unexpected error: {error}"
    );
}

#[test]
fn the_compressed_container_publishes_the_same_spectra_as_the_shared_entry_point() {
    let netlist = Netlist::parse(DECK).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let result = engine
        .run_tran_with_abort(&netlist, 5.0e-3, 1.0e-5, &NoAbort)
        .expect("transient runs");
    let compressed = engine
        .compress_transient_result_with_abort(
            &netlist,
            &result,
            &rspice_core::engine::CompressionConfig::default(),
            &NoAbort,
        )
        .expect("compression succeeds");
    let direct = rspice_core::engine::evaluate_transient_fourier_results(
        &netlist,
        &result,
        ResourceLimits::default(),
        &NoAbort,
    )
    .expect("direct resolution succeeds");
    assert_eq!(
        compressed.post_results.fourier, direct,
        "the compressed container must carry the shared entry point's own spectra"
    );
}
