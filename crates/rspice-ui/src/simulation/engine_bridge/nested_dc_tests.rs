//! Real DC solves through retention, projection, and authenticated project history.
//! Nearby secondary coordinates must stay distinct, and terminal traversal and
//! voltage/current identities must survive every consumer without label inference.

use super::*;
use crate::simulation::config::DcSweepConfig;

const DECK: &str = "Nested independent sources\nV1 in 0 0\nV2 out 0 0\nR1 in out 1k\n.end\n";

pub(crate) fn nested_config() -> DcSweepConfig {
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

pub(crate) fn solve(config: DcSweepConfig) -> SimulationResult {
    EngineBridge::new()
        .run(&AnalysisConfig::DcSweep(config), DECK)
        .unwrap()
}

pub(crate) fn retain(result: SimulationResult) -> crate::state::AnalysisResult {
    crate::simulation::SimulationController::new().convert_to_analysis_result_with_metadata_owned(
        result,
        crate::state::AnalysisType::DcSweep,
        "DC family",
    )
}

pub(crate) fn evidence(
    analysis: &crate::state::AnalysisResult,
) -> &std::sync::Arc<crate::state::DcSweepEvidence> {
    let Some(crate::state::AnalysisResultPayload::DcSweep { evidence }) = &analysis.result_payload
    else {
        panic!(
            "successful DC result must retain its coordinates: {:?}",
            analysis.error_message
        );
    };
    evidence
}

#[test]
fn close_secondary_values_retain_every_solved_voltage_and_current_curve() {
    let result = solve(nested_config());
    let SimulationResult::DcSweep { waveforms, .. } = result else {
        panic!("the bridge must return the requested DC family");
    };
    assert_eq!(
        waveforms.len(),
        12,
        "three secondary points each solve two node voltages and two branch currents"
    );
    let mut constants = waveforms
        .values()
        .filter(|trace| trace.y_unit == "V")
        .filter(|trace| trace.y_values.first() == trace.y_values.last())
        .map(|trace| trace.y_values[0])
        .collect::<Vec<_>>();
    constants.sort_by(f64::total_cmp);
    assert_eq!(constants.len(), 3);
    for (actual, expected) in constants.into_iter().zip([1.0e-7, 2.0e-7, 3.0e-7]) {
        assert!(
            (actual - expected).abs() < 1.0e-20,
            "{actual} != {expected}"
        );
    }
}

#[test]
fn dc_display_order_preserves_actual_terminal_coordinates_and_branch_units() {
    use crate::state::{DcSweepDirection, DcSweepFamily, DcSweepQuantity};
    for descending in [false, true] {
        for nested in [false, true] {
            let config = DcSweepConfig {
                start: if descending { 1.0 } else { 0.0 },
                stop: if descending { 0.0 } else { 1.0 },
                step: if descending { -0.3 } else { 0.3 },
                source2: nested.then(|| "V2".to_owned()),
                start2: nested.then_some(3.0e-7),
                stop2: nested.then_some(0.0),
                step2: nested.then_some(-1.1e-7),
                ..nested_config()
            };
            let result = retain(solve(config));
            assert!(result.success, "{:?}", result.error_message);
            result.validate_retained_evidence().unwrap();
            let metadata = evidence(&result);
            assert_eq!(
                metadata.direction == DcSweepDirection::Descending,
                descending
            );
            let last_member = metadata.member_count() - 1;
            let expected_secondary = if nested { 0.8e-7 } else { 0.0 };
            if let DcSweepFamily::Nested { values, .. } = &metadata.family {
                assert_eq!(values.len(), 3);
                assert!((values[2] - expected_secondary).abs() < 1e-20);
            }
            let primary = if descending { 0.1 } else { 0.9 };
            for quantity in &metadata.quantities {
                let name = metadata.trace_name(quantity, last_member);
                let trace = result.waveforms.iter().find(|w| w.name == name).unwrap();
                assert_eq!(trace.unit.as_deref(), Some(quantity.unit()));
                assert!(trace.x.windows(2).all(|pair| pair[0] < pair[1]));
                let last = metadata
                    .terminal_sample(last_member, trace.y.len())
                    .unwrap();
                assert!((trace.x[last] - primary).abs() < 1e-14);
                let expected = match quantity {
                    DcSweepQuantity::NodeVoltage(node) if node == "IN" => primary,
                    DcSweepQuantity::NodeVoltage(_) => expected_secondary,
                    DcSweepQuantity::BranchCurrent(branch) if branch == "V1" => {
                        -(primary - expected_secondary) / 1000.0
                    }
                    DcSweepQuantity::BranchCurrent(_) => (primary - expected_secondary) / 1000.0,
                };
                assert!(
                    (trace.y[last] - expected).abs() < 1e-12,
                    "{name}: {} != {expected}",
                    trace.y[last]
                );
                assert!(std::sync::Arc::ptr_eq(&result.waveforms[0].x, &trace.x));
            }
        }
    }
}

#[test]
fn single_point_and_retraced_dc_keep_member_traversal() {
    for (start, stop, step, retrace) in [
        (0.4, 0.4, 0.1, false),
        (0.0, 1.0, 0.5, true),
        (1.0, 0.0, -0.5, true),
    ] {
        let result = retain(solve(DcSweepConfig {
            source: "V1".to_owned(),
            start,
            stop,
            step,
            hysteresis: retrace,
            ..Default::default()
        }));
        assert!(result.success, "{:?}", result.error_message);
        let metadata = evidence(&result);
        for member in 0..metadata.member_count() {
            let name = metadata.trace_name(
                &crate::state::DcSweepQuantity::NodeVoltage("IN".to_owned()),
                member,
            );
            let trace = result.waveforms.iter().find(|w| w.name == name).unwrap();
            let last = metadata.terminal_sample(member, trace.y.len()).unwrap();
            let expected = if member == 0 { stop } else { start };
            assert!((trace.y[last] - expected).abs() < 1e-12);
        }
    }
}

#[test]
fn nested_dc_accepts_one_secondary_coordinate() {
    let result = retain(solve(DcSweepConfig {
        stop2: Some(1e-7),
        ..nested_config()
    }));
    assert!(result.success, "{:?}", result.error_message);
    assert_eq!(evidence(&result).member_count(), 1);
    assert_eq!(result.waveforms.len(), 4);
    result.validate_retained_evidence().unwrap();
}

#[test]
fn dc_conversion_rejects_a_curve_axis_that_disagrees_with_the_shared_axis() {
    let mut result = solve(nested_config());
    let SimulationResult::DcSweep { sweep_values, .. } = &mut result else {
        unreachable!()
    };
    sweep_values[1] = 0.25;
    let retained = retain(result);
    assert!(!retained.success);
    assert!(
        retained
            .error_message
            .unwrap()
            .contains("different primary axes")
    );
}

pub(crate) fn history(analysis: crate::state::AnalysisResult) -> crate::state::SimulationState {
    let mut run = crate::state::SimulationRun::new(1);
    run.add_analysis(analysis);
    // This standalone bridge fixture has no prepared-task authority. PVT tests
    // separately qualify real prepared receipts; do not invent one here.
    run.restore_provenance(crate::state::SimulationRunProvenance::LegacyUnattributed)
        .unwrap();
    run.mark_running().unwrap();
    run.finish_lifecycle(crate::state::SimulationRunLifecycle::Completed)
        .unwrap();
    let mut state = crate::state::SimulationState::default();
    state.runs.push(run);
    state.next_run_id = 1;
    state.active_run_idx = Some(0);
    state.active_analysis_idx = Some(0);
    state
}

fn stored(
    analysis: crate::state::AnalysisResult,
) -> crate::io::project_io::ProjectSimulationResults {
    crate::io::project_io::ProjectSimulationResults::from_state(&history(analysis))
}

#[test]
fn dc_project_round_trip_preserves_coordinates_units_selection_and_immutable_ownership() {
    use crate::io::project_io::ProjectSimulationResults;
    let result = solve(nested_config());
    let SimulationResult::DcSweep {
        evidence: Some(original),
        ..
    } = &result
    else {
        unreachable!()
    };
    let original = original.clone();
    let analysis = retain(result);
    assert!(std::sync::Arc::ptr_eq(&original, evidence(&analysis)));
    let digest = analysis.result_data_digest();
    let snapshot = stored(analysis);
    snapshot.validate().unwrap();
    assert_eq!(snapshot.schema_version, 26);
    let serialized = serde_json::to_value(&snapshot).unwrap();
    let restored: ProjectSimulationResults = serde_json::from_value(serialized.clone()).unwrap();
    let mut state = crate::state::SimulationState::default();
    restored.apply_to_state(&mut state).unwrap();
    let analysis = &state.runs[0].analyses[0];
    assert_eq!(evidence(analysis), &original);
    assert_eq!(analysis.result_data_digest(), digest);
    assert_eq!(analysis.waveforms.len(), 12);

    for (path, value) in [
        ("/source", serde_json::json!("V9")),
        ("/direction", serde_json::json!("descending")),
        ("/family/values/0", serde_json::json!(1.01e-7)),
        ("/quantities/0/name", serde_json::json!("DIFFERENT")),
        (
            "/selection",
            serde_json::json!({"kind":"saved","curves":[]}),
        ),
    ] {
        let mut corrupt = serialized.clone();
        *corrupt
            .pointer_mut(&format!("/runs/0/analyses/0/result_payload/evidence{path}"))
            .unwrap() = value;
        let corrupt: ProjectSimulationResults = serde_json::from_value(corrupt).unwrap();
        assert!(
            corrupt.validate().is_err(),
            "altered DC evidence accepted: {path}"
        );
    }
    for schema in 1..22 {
        let mut historic = snapshot.clone();
        historic.schema_version = schema;
        assert!(
            historic
                .migrate_to_current(crate::product::ProjectId::new())
                .unwrap_err()
                .contains("DC curve evidence")
        );
    }
}

#[test]
fn schema_21_dc_history_authenticates_without_inventing_traversal() {
    let mut legacy = retain(solve(nested_config()));
    legacy.result_payload = None;
    let digest = legacy.result_data_digest();
    let old_digest = legacy.legacy_v12_result_data_digest();
    let history = history(legacy);
    let old_dataset_digest = history.runs[0].legacy_v12_dataset_content_digest();
    let mut snapshot = crate::io::project_io::ProjectSimulationResults::from_state(&history);
    snapshot.schema_version = 21;
    snapshot.runs[0].analyses[0].result_data_digest =
        crate::io::project_io::PersistedField::Value(old_digest);
    snapshot.runs[0].dataset_content_digest =
        crate::io::project_io::PersistedField::Value(old_dataset_digest);
    let mut corrupt = serde_json::to_value(&snapshot).unwrap();
    corrupt["runs"][0]["analyses"][0]["waveforms"][0]["name"] = serde_json::json!("changed");
    let mut corrupt: crate::io::project_io::ProjectSimulationResults =
        serde_json::from_value(corrupt).unwrap();
    assert!(
        corrupt
            .migrate_to_current(crate::product::ProjectId::new())
            .unwrap_err()
            .contains("digest")
    );
    snapshot
        .migrate_to_current(crate::product::ProjectId::new())
        .unwrap();
    let mut restored = crate::state::SimulationState::default();
    snapshot.apply_to_state(&mut restored).unwrap();
    assert!(restored.runs[0].analyses[0].result_payload.is_none());
    assert_eq!(restored.runs[0].analyses[0].result_data_digest(), digest);
}

#[test]
fn filtered_dc_curves_and_derived_aliases_keep_exact_projection_evidence() {
    use crate::simulation::multi_run::AnalysisSpec;
    use crate::simulation::output_contract::{PreparedSavedOutput, retain_plan_saved_outputs};
    use crate::state::{
        SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy,
        SavedOutputPrecision, SavedOutputStreaming,
    };
    let result = solve(DcSweepConfig {
        source: "V1".to_owned(),
        start: 0.0,
        stop: 1.0,
        step: 0.5,
        ..Default::default()
    });
    let mut analysis = retain(result);
    let original = evidence(&analysis).clone();
    let source_samples = analysis
        .waveforms
        .iter()
        .find(|w| w.name == "V(IN)")
        .unwrap()
        .y
        .clone();
    let spec = AnalysisSpec::DcSweep {
        source_name: "V1".to_owned(),
        start: 0.0,
        stop: 1.0,
        step: 0.5,
        source2: None,
        start2: None,
        stop2: None,
        step2: None,
        hysteresis: false,
    };
    let output = SavedOutput::new(
        SavedOutputKind::RawVoltageOrCurrent,
        "Input alias",
        "V(IN)",
        SavedOutputCompatibility::AllCompatibleAnalyses,
        SavedOutputPolicy::EveryAcceptedPoint,
        SavedOutputPrecision::FullSourcePrecision,
        SavedOutputStreaming::StoreOnly,
    )
    .unwrap();
    let contract =
        PreparedSavedOutput::prepare(&output, crate::product::AnalysisInstanceId::new(), &spec)
            .unwrap()
            .unwrap();
    retain_plan_saved_outputs(&mut analysis, &[contract]);
    assert_eq!(analysis.waveforms.len(), 1);
    assert_eq!(analysis.waveforms[0].name, "Input alias");
    assert!(std::sync::Arc::ptr_eq(
        &source_samples,
        &analysis.waveforms[0].y
    ));
    assert!(
        matches!(&evidence(&analysis).selection, crate::state::DcCurveSelection::Saved(curves) if curves.is_empty())
    );
    assert!(matches!(
        original.selection,
        crate::state::DcCurveSelection::All
    ));
    analysis.validate_retained_evidence().unwrap();
    stored(analysis).validate().unwrap();
}
