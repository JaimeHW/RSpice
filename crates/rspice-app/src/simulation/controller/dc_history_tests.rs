//! DC retention, projection and authenticated project-history integration.

use crate::simulation::SimulationResult;
use rspice_simulation_contract::config::DcSweepConfig;

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
    config.validate().expect("valid DC fixture configuration");
    assert!(
        !config.hysteresis,
        "retraced traversal is qualified by runtime tests"
    );
    let source = DECK.replace(".end", &format!("{}\n.end", config.to_spice()));
    super::test_execution::run_manual_deck(&source)
}

pub(crate) fn retain(result: SimulationResult) -> crate::state::AnalysisResult {
    crate::simulation::SimulationController::new().convert_to_analysis_result_with_metadata_owned(
        result,
        rspice_results::analysis_type::AnalysisType::DcSweep,
        "DC family",
    )
}

pub(crate) fn evidence(
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

pub(crate) fn history(analysis: crate::state::AnalysisResult) -> crate::state::SimulationState {
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
    state.runs.push(run);
    state.next_run_id = 1;
    state.active_run_idx = Some(0);
    state.active_analysis_idx = Some(0);
    state
}

fn stored(
    analysis: crate::state::AnalysisResult,
) -> crate::io::project_io::ProjectSimulationResults {
    crate::io::capture_simulation_results(&history(analysis))
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
    assert_eq!(
        snapshot.schema_version,
        crate::io::project_io::ProjectSimulationResults::default().schema_version
    );
    let serialized = serde_json::to_value(&snapshot).unwrap();
    let restored: ProjectSimulationResults = serde_json::from_value(serialized.clone()).unwrap();
    let mut state = crate::state::SimulationState::default();
    crate::io::restore_simulation_results(restored, &mut state).unwrap();
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
                .migrate_to_current(rspice_app_types::product::ProjectId::new())
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
    let old_digest = legacy
        .result_data_ref()
        .digest(rspice_results::result_digest::ResultDigestEncoding::V12);
    let history = history(legacy);
    let old_dataset_digest = history.runs[0].data.dataset_content_digest_with_encoding(
        rspice_results::result_digest::ResultDigestEncoding::V12,
    );
    let mut snapshot = crate::io::capture_simulation_results(&history);
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
            .migrate_to_current(rspice_app_types::product::ProjectId::new())
            .unwrap_err()
            .contains("digest")
    );
    snapshot
        .migrate_to_current(rspice_app_types::product::ProjectId::new())
        .unwrap();
    let mut restored = crate::state::SimulationState::default();
    crate::io::restore_simulation_results(snapshot, &mut restored).unwrap();
    assert!(restored.runs[0].analyses[0].result_payload.is_none());
    assert_eq!(restored.runs[0].analyses[0].result_data_digest(), digest);
}

#[test]
fn filtered_dc_curves_and_derived_aliases_keep_exact_projection_evidence() {
    use crate::simulation::output_contract::retain_plan_saved_outputs;
    use rspice_results::saved_output::SavedOutputKind;
    use rspice_results::saved_output::SavedOutputPolicy;
    use rspice_results::saved_output::SavedOutputPrecision;
    use rspice_results::saved_output::SavedOutputStreaming;
    use rspice_simulation::output_contract::PreparedSavedOutput;
    use rspice_simulation_contract::analysis_spec::AnalysisSpec;
    use rspice_simulation_contract::saved_output::SavedOutput;
    use rspice_simulation_contract::saved_output::SavedOutputCompatibility;
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
        modes: Default::default(),
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
    let contract = PreparedSavedOutput::prepare(
        &output,
        rspice_app_types::product::AnalysisInstanceId::new(),
        &spec,
    )
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
        matches!(&evidence(&analysis).selection, rspice_results::dc_sweep::DcCurveSelection::Saved(curves) if curves.is_empty())
    );
    assert!(matches!(
        original.selection,
        rspice_results::dc_sweep::DcCurveSelection::All
    ));
    analysis.validate_retained_evidence().unwrap();
    stored(analysis).validate().unwrap();
}
