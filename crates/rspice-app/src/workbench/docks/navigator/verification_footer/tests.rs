//! What the navigator is allowed to claim about a run it did not execute.
//!
//! An empty specification set is neutral, not healthy; a finite goal miss is
//! executed but not passed; a mapping without an active dataset claims no
//! execution at all. The remaining cases hold the mock's grid and spacing, and
//! the density switch between desktop and touch.

use super::super::simulate::simulate_nav_meta;
use super::super::verify::{
    verification_coverage, verification_flow_label, verification_navigator_requires_scroll,
};
use super::{
    FLOW_DETAIL_TOP, FLOW_LABEL_TOP, FLOW_ROW_HEIGHT, FLOW_STATUS_TOP, active_mc_sample_trail,
    flow_row_geometry,
};
use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
use crate::state::pdk_config::PdkConfig;
use crate::state::{
    AnalysisResult, AnalysisResultPayload, AnalysisType, SimulationRun, SimulationState,
};
use crate::state::{SimulationPage, VerificationPage};
use crate::workbench::app_state::Model;
use crate::workbench::state::VerificationSessionState;
use rspice_app_state::workbench::workspace::restore_model_library_manager;
use rspice_results::yield_analysis::{
    DistributionStats, MonteCarloSamplingMode, YieldAnalysisProvenance, YieldResult, YieldSpec,
};

fn result(trail: Vec<bool>) -> YieldResult {
    let pass_count = trail.iter().filter(|passes| **passes).count();
    YieldResult {
        spec: YieldSpec::lower("gain", 0.0, ""),
        total_runs: trail.len(),
        pass_count,
        fail_count: trail.len() - pass_count,
        yield_percent: pass_count as f64 / trail.len() as f64 * 100.0,
        stats: DistributionStats::default(),
        samples: vec![1.0; trail.len()],
        trail,
    }
}

#[test]
fn simulation_models_meta_counts_only_active_plan_bindings() {
    let mut model = Model::session_start(PdkConfig::default(), None, restore_model_library_manager);

    assert_eq!(
        simulate_nav_meta(&model, SimulationPage::Models, "unused", None, 0),
        None,
        "globally available libraries are not owned by the active simulation plan",
    );

    model
        .sim_setup
        .model_bindings
        .push(crate::state::model_library::SimulationPlanModelBinding {
            library_name: "project-process-models".to_owned(),
            source_digest: ContentDigest::from_bytes([0x6d; 32]),
            selected_corner: Some("tt".to_owned()),
        });

    assert_eq!(
        simulate_nav_meta(&model, SimulationPage::Models, "unused", None, 0),
        Some("1".to_owned()),
    );
}

#[test]
fn verification_flow_rows_match_the_mock_grid_without_text_collisions() {
    assert!(FLOW_DETAIL_TOP - (FLOW_LABEL_TOP + 11.0) >= 4.0);
    assert!(FLOW_STATUS_TOP - (FLOW_DETAIL_TOP + 11.0) >= 4.0);
    assert!(FLOW_ROW_HEIGHT - (FLOW_STATUS_TOP + 11.0) >= 7.0);
    assert_eq!(flow_row_geometry(1, true), (63.0, 43.0));
    assert_eq!(flow_row_geometry(2, true), (78.0, 58.0));
    assert_eq!(flow_row_geometry(3, true), (93.0, 73.0));
    // A row that states only its label and one line is one line shorter.
    assert_eq!(flow_row_geometry(0, true), (43.0, 23.0));
    assert_eq!(flow_row_geometry(1, false), (43.0, 43.0));
}

#[test]
fn verification_navigation_exposes_the_operational_tuning_route() {
    assert_eq!(VerificationPage::NAVIGATION.len(), 6);
    let labels = VerificationPage::NAVIGATION.map(verification_flow_label);
    assert_eq!(
        labels,
        [
            "PVT & Monte Carlo",
            "Process corners",
            "Tuning",
            "Optimization",
            "Electrical SOA",
            "Regression",
        ]
    );
}

#[test]
fn specification_mapping_does_not_claim_execution_without_an_active_dataset() {
    let mut model = Model::session_start(PdkConfig::default(), None, restore_model_library_manager);
    model.workspace.content.specs.push(crate::state::SpecEntry {
        measurement: "gain".to_owned(),
        expression: String::new(),
        min: Some(1.0),
        max: None,
        unit: "V/V".to_owned(),
        scope: crate::state::SpecPointScope::AllPoints,
    });
    model.simulation.view.active_run_idx = None;

    let coverage = verification_coverage(&model);

    assert_eq!(coverage.total, 1);
    assert_eq!(coverage.passed, 0);
    assert_eq!(coverage.status, "1 spec not measured");
}

#[test]
fn empty_specification_set_is_neutral_not_healthy() {
    let model = Model::session_start(PdkConfig::default(), None, restore_model_library_manager);
    let coverage = verification_coverage(&model);

    assert_eq!(coverage.total, 0);
    assert_eq!(coverage.status, "No specs");
}

#[test]
fn finite_goal_miss_counts_as_executed_but_not_passed() {
    let mut model = Model::session_start(PdkConfig::default(), None, restore_model_library_manager);
    model.workspace.content.specs.push(crate::state::SpecEntry {
        measurement: "gain".to_owned(),
        expression: String::new(),
        min: Some(41.0),
        max: None,
        unit: "V/V".to_owned(),
        scope: crate::state::SpecPointScope::AllPoints,
    });
    let mut measurement = rspice_core::MeasureResult::success("gain", 40.0);
    measurement.passed = false;
    measurement.error = Some("value misses GOAL".to_owned());
    let analysis = AnalysisResult::new(1, AnalysisType::Ac, "AC")
        .with_measurements(vec![measurement])
        .with_provenance(
            crate::state::AnalysisResultProvenance::new(
                AnalysisInstanceId::new(),
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([0x7b; 32]),
                Vec::new(),
            )
            .expect("test provenance is valid"),
        );
    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    model.simulation.retained.runs = vec![run].into();
    model.simulation.view.active_run_idx = Some(0);

    let coverage = verification_coverage(&model);
    // Measured, so nothing is missing; missed its goal, so it does not pass.
    assert_eq!(coverage.status, "");
    assert_eq!(coverage.passed, 0);
}

/// A run that retains an SOA verdict states it, as an instance's Checks row
/// does; "verdict unavailable" is said only of evidence that holds none.
#[test]
fn the_device_safety_flow_states_the_retained_soa_verdict() {
    let mut model = Model::session_start(PdkConfig::default(), None, restore_model_library_manager);
    let evaluation = crate::state::SoaEvaluationEvidence {
        duration: None,
        thresholds: Default::default(),
        envelope: None,
        derating: None,
        device_id: "M1".to_owned(),
        parameter: crate::state::SoaParameterEvidence::DrainSourceVoltage,
        limit_value: 3.3,
        worst_actual_value: 3.0,
        worst_time_s: 1.0e-9,
        sample_count: 2,
        unit: "V".to_owned(),
        description: "Maximum drain-source voltage".to_owned(),
        verdict: crate::state::SoaRuleVerdictEvidence::Warning,
    };
    let analysis = AnalysisResult::new(1, AnalysisType::Soa, "SOA")
        .with_result_payload(AnalysisResultPayload::Soa {
            source_history: None,
            evaluations: vec![evaluation],
            violations: Vec::new(),
        })
        .with_provenance(
            crate::state::AnalysisResultProvenance::new(
                AnalysisInstanceId::new(),
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([0x5c; 32]),
                Vec::new(),
            )
            .expect("test provenance is valid"),
        );
    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    model.simulation.retained.runs = vec![run].into();
    model.simulation.view.active_run_idx = Some(0);

    let flow = super::super::verify::verification_flow_presentation(
        &model,
        &VerificationSessionState::default(),
        VerificationPage::DeviceSafety,
    );
    assert_eq!(flow.status, "9.1% margin");
}

#[test]
fn verification_navigator_scrolls_when_flows_exceed_compact_height() {
    assert!(verification_navigator_requires_scroll(503.0));
    assert!(!verification_navigator_requires_scroll(504.0));
    assert!(verification_navigator_requires_scroll(390.0));
    assert!(!verification_navigator_requires_scroll(560.0));
    assert!(!verification_navigator_requires_scroll(700.0));
}

#[test]
fn mc_sample_trail_is_visible_only_for_its_active_provenance_dataset() {
    let source = SimulationRun::new(1);
    let source_run_id = source.run_id;
    let source_dataset_id = source.dataset_id;
    let other = SimulationRun::new(2);
    let mut simulation = SimulationState {
        retained: crate::state::RetainedSimulationState {
            runs: vec![source, other].into(),
            ..Default::default()
        },
        view: crate::state::SimulationViewState {
            active_run_idx: Some(0),
            ..Default::default()
        },
        ..SimulationState::default()
    };
    simulation.replace_yield_evidence(
        vec![result(vec![true, false, true])],
        Some(YieldAnalysisProvenance {
            source_run_id,
            source_dataset_id,
            seed: 7,
            runs_requested: 3,
            runs_completed: 3,
            sampling_mode: MonteCarloSamplingMode::PseudoRandom,
        }),
    );

    assert_eq!(active_mc_sample_trail(&simulation), 3);
    simulation.view.active_run_idx = Some(1);
    assert_eq!(active_mc_sample_trail(&simulation), 0);
    simulation.view.active_run_idx = None;
    assert_eq!(active_mc_sample_trail(&simulation), 0);
}
