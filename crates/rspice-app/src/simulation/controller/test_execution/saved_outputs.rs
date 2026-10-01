//! Saved-output materialization over results from checked runs.
//! These tests exercise retention policies independently of plan budget preflight.

use crate::state::{OutputSelectionMode, SavedOutput, SimulationRun};
use rspice_simulation_contract::analysis_spec::AnalysisSpec;

mod typed;

pub(crate) fn run(
    deck: &str,
    spec: AnalysisSpec,
    outputs: &[SavedOutput],
    output_selection_mode: OutputSelectionMode,
) -> SimulationRun {
    let mut run = if matches!(
        spec,
        AnalysisSpec::DcSweep {
            hysteresis: true,
            ..
        } | AnalysisSpec::Noise { .. }
    ) {
        typed::run(deck, &spec)
    } else {
        manual(deck, &spec)
    };
    let receipt = run.prepared_receipt().unwrap().clone();
    let analysis = run.analyses.last_mut().unwrap();
    let instance = analysis.provenance.as_ref().unwrap().source_instance_id();
    let mut contracts = Vec::new();
    for output in outputs {
        contracts.extend(
            rspice_simulation::output_contract::compile_saved_output_contracts(
                output,
                [(instance, &spec)],
            )
            .unwrap(),
        );
    }
    rspice_simulation::output_contract::PreparedSavedOutput::bind_deck(
        &mut contracts,
        &rspice_core::Netlist::parse(deck).unwrap(),
    )
    .unwrap();
    crate::simulation::output_contract::apply_saved_output_policy(
        analysis,
        rspice_simulation::execution::SavePolicy::PlanOwned {
            output_selection_mode,
            retained_dataset_limit: 20,
            maximum_storage_bytes: u64::MAX,
            live_streaming_enabled: true,
            retain_failure_diagnostics: true,
        },
        &contracts,
    );
    analysis.validate_retained_evidence().unwrap();
    assert_eq!(run.prepared_receipt(), Some(&receipt));
    run.validate_provenance().unwrap();
    run
}

fn manual(deck: &str, spec: &AnalysisSpec) -> SimulationRun {
    let mut state = super::AppState::default();
    state.sim_setup.reference_pvt.temperature_celsius = 27.0;
    let directive = rspice_simulation::analysis_preparation::analysis_spec_to_config(spec)
        .expect("fixture has an authored analysis configuration")
        .to_spice();
    let source =
        rspice_simulation::netlist_preparation::splice_before_terminal_end_card(deck, &directive);
    let queue = rspice_simulation::manual_deck::build_manual_deck_queue(
        state.sim_setup.reference_pvt.temperature_celsius,
        &source,
    )
    .expect("fixture directive is accepted by authored-deck preparation");
    assert_eq!(queue.len(), 1);
    assert_eq!(
        serde_json::to_value(&queue[0].spec).unwrap(),
        serde_json::to_value(&spec).unwrap(),
        "fixture authoring must preserve every typed analysis control"
    );
    state.simulation.run_intent = super::SimulationRunIntent::ManualDeck;
    state.workspace.content.netlist_source = Some(source);
    let mut controller = super::SimulationController::new();
    controller.validate_manual_deck_document(&state).unwrap();
    let run = super::run_batch(
        state,
        controller,
        rspice_results::run::SimulationRunLifecycle::Completed,
    );
    assert_eq!(run.analyses.len(), 1);
    run
}
