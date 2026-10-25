//! A DC sweep solved through the real run and retained as one run's
//! history, for the tests that read retained DC results.

use crate::simulation::SimulationResult;
use rspice_simulation_contract::config::DcSweepConfig;

const DECK: &str = "Nested independent sources\nV1 in 0 0\nV2 out 0 0\nR1 in out 1k\n.end\n";

pub fn nested_config() -> DcSweepConfig {
    DcSweepConfig {
        source: "V1".to_owned(),
        start: 0.0,
        stop: 1.0,
        step: 0.5,
        source2: Some("V2".to_owned()),
        start2: Some(1.0e-7),
        stop2: Some(3.0e-7),
        step2: Some(1.0e-7),
        ..Default::default()
    }
}

pub fn solve(config: DcSweepConfig) -> SimulationResult {
    config.validate().expect("valid DC fixture configuration");
    assert!(
        !config.hysteresis,
        "retraced traversal is qualified by runtime tests"
    );
    let source = DECK.replace(".end", &format!("{}\n.end", config.to_spice()));
    super::run_manual_deck(&source)
}

pub fn retain(result: SimulationResult) -> crate::state::AnalysisResult {
    crate::simulation::SimulationController::new().convert_to_analysis_result_with_metadata_owned(
        result,
        rspice_results::analysis_type::AnalysisType::DcSweep,
        "DC family",
    )
}

pub fn evidence(
    analysis: &crate::state::AnalysisResult,
) -> &std::sync::Arc<rspice_results::dc_sweep::DcSweepEvidence> {
    let Some(rspice_results::analysis_payload::AnalysisResultPayload::DcSweep { evidence }) =
        &analysis.result_payload
    else {
        panic!(
            "successful DC result must retain its coordinates: {:?}",
            analysis.error_message
        );
    };
    evidence
}

pub fn history(analysis: crate::state::AnalysisResult) -> crate::state::SimulationState {
    let mut run = crate::state::SimulationRun::new(1);
    run.add_analysis(analysis);
    // Model legacy history without attributing a prepared receipt to it.
    // The solver fixture's authorization does not authenticate this imported history.
    run.restore_provenance(
        rspice_results::run_receipt::SimulationRunProvenance::LegacyUnattributed,
    )
    .unwrap();
    run.mark_running().unwrap();
    run.finish_lifecycle(rspice_results::run::SimulationRunLifecycle::Completed)
        .unwrap();
    let mut state = crate::state::SimulationState::default();
    state.retained.runs.push(run);
    state.retained.next_run_id = 1;
    state.view.active_run_idx = Some(0);
    state.view.active_analysis_idx = Some(0);
    state
}
