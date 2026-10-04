//! Saved selections and deferred calculations use the complete quasi-periodic payload.
use super::fixtures::output;
use super::*;
use crate::simulation::{execution::SavePolicy, plan::QpssDraft};
use crate::state::{AnalysisResultPayload, AnalysisType, OutputSelectionMode};

fn prepared(
    output: &SavedOutput,
    spec: &AnalysisSpec,
    id: AnalysisInstanceId,
) -> PreparedSavedOutput {
    PreparedSavedOutput::prepare(output, id, spec)
        .unwrap()
        .expect("eligible quasi-periodic output")
}
fn reload(analysis: AnalysisResult) -> AnalysisResult {
    let saved = crate::io::project_io::ProjectAnalysisResult::from(&analysis);
    let decoded: crate::io::project_io::ProjectAnalysisResult =
        serde_json::from_str(&serde_json::to_string(&saved).unwrap()).unwrap();
    assert_eq!(decoded, saved);
    let crate::io::project_io::PersistedField::Value(payload) = decoded.result_payload else {
        panic!("missing quasi-periodic payload")
    };
    assert_eq!(Some(&payload), analysis.result_payload.as_ref());
    if let AnalysisResultPayload::Qpss { operating_point } = &payload {
        operating_point
            .validate_retained_payload_with_abort(
                &rspice_core::ResourceLimits::default(),
                &rspice_core::NoAbort,
            )
            .unwrap();
        assert_eq!(operating_point.integral_names(), ["B:BMEMORY:sdt:0"]);
    }
    let state = crate::simulation::controller::dc_history_tests::history(analysis);
    let persisted = crate::io::capture_simulation_results(&state);
    let loaded: crate::io::project_io::ProjectSimulationResults =
        serde_json::from_slice(&serde_json::to_vec(&persisted).unwrap()).unwrap();
    crate::io::simulation_state_from_results(loaded)
        .unwrap()
        .retained
        .runs[0]
        .analyses[0]
        .clone()
}
fn policy(mode: OutputSelectionMode) -> SavePolicy {
    SavePolicy::PlanOwned {
        output_selection_mode: mode,
        retained_dataset_limit: 10,
        maximum_storage_bytes: u64::MAX,
        live_streaming_enabled: false,
        retain_failure_diagnostics: true,
    }
}
fn qpss() -> (AnalysisResult, AnalysisSpec) {
    let draft = QpssDraft {
        tones: "1000, 1414.213562373095".into(),
        harmonics: "1,1".into(),
        ..Default::default()
    };
    let spec = draft.to_spec().unwrap();
    let deck = "QPSS transfer\nV1 in 0 SIN(.1 .2 1k)\nR1 in out 1k\nC1 out 0 1u\nI1 0 out SIN(0 .001 1414.213562373095)\nBmemory memory 0 V=1k*sdt(v(out)-v(memory))\nRmemory memory 0 1k\n";
    let source = format!(
        "{deck}{}\n.end\n",
        spec.driven_qpss_config().unwrap().to_spice().unwrap()
    );
    let analysis = crate::simulation::results::retained_manual_fixture(&source, AnalysisType::Qpss);
    (analysis, spec)
}
#[test]
fn quasi_periodic_saved_outputs_select_physical_qpss_and_qpac_sources_without_losing_payloads() {
    let qpac = crate::simulation::results::qpac_retained_test_fixture();
    let spec = crate::simulation::plan::QuasiPeriodicAcDraft::default()
        .to_spec()
        .unwrap();
    for (original, spec) in [qpss(), (qpac, spec)] {
        let basis = original
            .result_payload
            .as_ref()
            .unwrap()
            .retained_waveform_basis()
            .unwrap()
            .unwrap();
        let expected = basis
            .iter()
            .find(|w| {
                w.complex
                    .as_ref()
                    .is_some_and(|c| c.source_name.to_ascii_lowercase().starts_with("v(out)"))
            })
            .unwrap();
        let id = AnalysisInstanceId::new();
        let raw = output(
            SavedOutputKind::RawVoltageOrCurrent,
            "Saved output",
            "V(out)",
        );
        let mut derived = output(
            SavedOutputKind::DerivedExpression,
            "Deferred magnitude",
            "mag(V(out))",
        );
        derived.save_policy = SavedOutputPolicy::OnDemandFromRetainedState;
        let contracts = [prepared(&raw, &spec, id), prepared(&derived, &spec, id)];
        for mode in [
            OutputSelectionMode::SaveAll,
            OutputSelectionMode::ExplicitOnly,
        ] {
            let mut retained = original.clone();
            apply_saved_output_policy(&mut retained, policy(mode), &contracts);
            retained.validate_retained_evidence().unwrap();
            assert_eq!(retained.result_payload, original.result_payload);
            if mode == OutputSelectionMode::ExplicitOnly {
                assert_eq!(retained.waveforms.len(), 1);
            }
            let mut retained = reload(retained);
            let saved = retained
                .waveforms
                .iter()
                .find(|w| w.name == "Saved output")
                .unwrap();
            assert_eq!(saved.x, expected.x);
            assert_eq!(saved.y, expected.y);
            assert_eq!(saved.unit.as_deref(), Some("V"));
            assert_eq!(
                saved.complex.as_ref().unwrap().real,
                expected.complex.as_ref().unwrap().real
            );
            materialize_deferred_saved_output(&mut retained, 1).unwrap();
            let deferred = retained
                .waveforms
                .iter()
                .find(|w| w.name == "Deferred magnitude")
                .unwrap();
            assert_eq!(deferred.y, expected.y);
            retained.validate_retained_evidence().unwrap();
            let mut changed = retained.clone();
            changed
                .waveforms
                .push(WaveformData::new("invented", vec![0.0], vec![1.0], "#fff"));
            assert!(changed.validate_retained_evidence().is_err());
        }
        let mut empty = original;
        apply_saved_output_policy(&mut empty, policy(OutputSelectionMode::ExplicitOnly), &[]);
        assert!(empty.waveforms.is_empty());
        reload(empty).validate_retained_evidence().unwrap();
    }
}
#[test]
fn quasi_periodic_saved_outputs_bind_qpxf_quoted_transfers_and_delay_after_reload() {
    let original = crate::simulation::results::qpxf_retained_test_fixture();
    let spec = crate::simulation::plan::QuasiPeriodicTransferDraft::default()
        .to_spec()
        .unwrap();
    let id = AnalysisInstanceId::new();
    let transfer = original
        .waveforms
        .iter()
        .find(|w| w.complex.is_some() && w.unit.as_deref() == Some("Ω"))
        .unwrap();
    let name = &transfer.complex.as_ref().unwrap().source_name;
    let mut selected = output(
        SavedOutputKind::DerivedExpression,
        "Saved transimpedance",
        &format!("{name:?}"),
    );
    selected.complex_policy = crate::state::ComplexExpressionPolicy::Rectangular;
    let delay = original
        .waveforms
        .iter()
        .find(|w| w.unit.as_deref() == Some("s"))
        .unwrap();
    let mut deferred = output(
        SavedOutputKind::DerivedExpression,
        "Saved delay",
        &format!("{:?}", delay.name),
    );
    deferred.save_policy = SavedOutputPolicy::OnDemandFromRetainedState;
    let contracts = [
        prepared(&selected, &spec, id),
        prepared(&deferred, &spec, id),
    ];
    let mut retained = original.clone();
    apply_saved_output_policy(
        &mut retained,
        policy(OutputSelectionMode::ExplicitOnly),
        &contracts,
    );
    assert_eq!(
        retained.waveforms.len(),
        1,
        "{:?}",
        retained.saved_output_receipts
    );
    let mut retained = reload(retained);
    let saved = &retained.waveforms[0];
    assert_eq!(saved.y, transfer.y);
    assert_eq!(saved.unit.as_deref(), Some("Ω"));
    assert_eq!(
        saved.complex.as_ref().unwrap().real,
        transfer.complex.as_ref().unwrap().real
    );
    materialize_deferred_saved_output(&mut retained, 1).unwrap();
    let saved = retained
        .waveforms
        .iter()
        .find(|w| w.name == "Saved delay")
        .unwrap();
    assert_eq!(saved.y, delay.y);
    assert_eq!(saved.x, delay.x);
    assert_eq!(saved.unit.as_deref(), Some("s"));
    assert_eq!(retained.result_payload, original.result_payload);
    let raw = output(
        SavedOutputKind::RawVoltageOrCurrent,
        "False voltage",
        "V(out)",
    );
    assert!(
        PreparedSavedOutput::prepare(&raw, id, &spec)
            .unwrap()
            .is_none()
    );
    // A saved expression cannot impersonate a new source, even after a reload.
    let forged = output(SavedOutputKind::DerivedExpression, "V(absent)", "1");
    let mut absent = output(SavedOutputKind::DerivedExpression, "Missing", "V(absent)");
    absent.save_policy = SavedOutputPolicy::OnDemandFromRetainedState;
    let mut retained = original.clone();
    apply_saved_output_policy(
        &mut retained,
        policy(OutputSelectionMode::ExplicitOnly),
        &[prepared(&forged, &spec, id), prepared(&absent, &spec, id)],
    );
    let mut retained = reload(retained);
    assert!(materialize_deferred_saved_output(&mut retained, 1).is_err());
    if let Some(AnalysisResultPayload::Qpxf { response }) = &mut retained.result_payload {
        Arc::make_mut(response).transfers[0].values[0].re += 1.0;
    }
    assert!(retained.validate_retained_evidence().is_err());
}

#[test]
fn qpnoise_result_saved_output_selection_and_deferred_reload_keep_complete_noise_evidence() {
    let original = crate::simulation::results::qpnoise_retained_test_fixture();
    let spec = crate::simulation::plan::QuasiPeriodicNoiseDraft::default()
        .to_spec()
        .unwrap();
    let spectrum = original
        .waveforms
        .iter()
        .find(|w| {
            w.name.starts_with("PSD(")
                && !w.name.contains("source")
                && w.unit.as_deref() == Some("A²/Hz")
        })
        .unwrap();
    let contributor = original
        .waveforms
        .iter()
        .find(|w| w.name.starts_with("PSD(") && w.name.contains("source"))
        .unwrap();
    let selected = output(
        SavedOutputKind::DerivedExpression,
        "Selected noise",
        &format!("{:?}", spectrum.name),
    );
    let mut deferred = output(
        SavedOutputKind::NoiseContributor,
        "Deferred mechanism",
        &format!("{:?}", contributor.name),
    );
    deferred.save_policy = SavedOutputPolicy::OnDemandFromRetainedState;
    let id = AnalysisInstanceId::new();
    let contracts = [
        prepared(&selected, &spec, id),
        prepared(&deferred, &spec, id),
    ];
    let mut retained = original.clone();
    apply_saved_output_policy(
        &mut retained,
        policy(OutputSelectionMode::ExplicitOnly),
        &contracts,
    );
    assert_eq!(
        retained.waveforms.len(),
        1,
        "{:?}",
        retained.saved_output_receipts
    );
    let mut retained = reload(retained);
    assert_eq!(retained.waveforms[0].y, spectrum.y);
    assert_eq!(retained.waveforms[0].unit, spectrum.unit);
    materialize_deferred_saved_output(&mut retained, 1).unwrap();
    let saved = retained
        .waveforms
        .iter()
        .find(|w| w.name == "Deferred mechanism")
        .unwrap();
    assert_eq!(saved.y, contributor.y);
    assert_eq!(saved.x, contributor.x);
    assert_eq!(retained.result_payload, original.result_payload);
    retained.validate_retained_evidence().unwrap();
}
