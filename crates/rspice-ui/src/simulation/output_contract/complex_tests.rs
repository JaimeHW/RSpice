//! Rectangular arithmetic and historical recipes through real AC execution.

use super::binding_tests::{ac, check_value, close, execute};
use super::fixtures::output;
use super::*;
use crate::io::project_io::{PersistedField, ProjectSimulationResults};
use crate::state::{ComplexExpressionPolicy, SimulationState};

const PHASORS: &str =
    "Opposing phasors\nV1 a 0 DC 0 AC 1 0\nV2 b 0 DC 0 AC 1 180\nR1 a 0 1k\nR2 b 0 1k\n.end\n";

#[test]
fn complex_selected_samples_project_magnitude_after_rectangular_interpolation() {
    let source = WaveformData::new("difference", vec![0.0, 1.0], vec![1.0; 2], "#fff")
        .with_complex_components("V(out)", vec![1.0, -1.0], vec![0.0; 2]);
    let selected = resample_selected_and_final(
        &source,
        TransientSelectionGrid {
            start: 0.0,
            step: 0.5,
            stop: 1.0,
        },
    )
    .unwrap();
    assert_eq!(selected.x.as_slice(), &[0.0, 0.5, 1.0]);
    assert_eq!(selected.y.as_slice(), &[1.0, 0.0, 1.0]);
    assert_eq!(selected.complex.unwrap().real.as_slice(), &[1.0, 0.0, -1.0]);
    assert_eq!(source.y.as_slice(), &[1.0, 1.0]);
}

#[test]
fn complex_live_outputs_keep_aligned_components_and_full_precision_source_knots() {
    let mut output = output(
        SavedOutputKind::DerivedExpression,
        "Complex preview",
        "complex(V(out),V(out))",
    );
    output.streaming = SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation;
    let contract = PreparedSavedOutput::prepare(&output, AnalysisInstanceId::new(), &ac())
        .unwrap()
        .unwrap();
    let x = (0..10_000)
        .map(|index| 1e-4 + index as f64 * 1e-18)
        .collect::<Vec<_>>();
    let y = (0..x.len())
        .map(|index| 1e-120 * (index as f64 / 17.0).sin())
        .collect::<Vec<_>>();
    let source =
        AnalysisResult::live_transient_partial(1, crate::state::AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![WaveformData::new("out", x.clone(), y.clone(), "#fff")]);
    let previews = materialize_live_saved_outputs(&source, &[contract]);
    assert_eq!(previews.len(), 1);
    let preview = &previews[0];
    assert!(preview.x.len() <= DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES);
    assert!(preview.x.windows(2).all(|pair| pair[0] < pair[1]));
    let complex = preview.complex.as_ref().unwrap();
    assert_eq!(preview.x.len(), complex.real.len());
    assert_eq!(preview.x.len(), complex.imag.len());
    for (index, &point) in preview.x.iter().enumerate() {
        let source_index = x
            .binary_search_by(|candidate| candidate.total_cmp(&point))
            .unwrap();
        assert_eq!(complex.real[index], y[source_index]);
        assert_eq!(complex.imag[index], y[source_index]);
    }
    assert!(calculator::waveform_value(preview, ComplexExpressionPolicy::Rectangular).is_ok());
}

#[test]
fn complex_derived_storage_estimate_waits_for_the_evaluated_shape() {
    let output = output(
        SavedOutputKind::DerivedExpression,
        "Complex output",
        "complex(V(a),V(b))",
    );
    let report = preflight_saved_output(&output, [(AnalysisInstanceId::new(), &ac())]);
    assert!(
        matches!(report.storage_estimate(), SavedOutputStorageEstimate::Indeterminate { reason } if reason.contains("real or complex"))
    );
}

fn history(policy: ComplexExpressionPolicy, deferred: bool) -> ProjectSimulationResults {
    let mut output = output(
        SavedOutputKind::DerivedExpression,
        "Difference",
        "V(a)-V(b)",
    );
    output.complex_policy = policy;
    if deferred {
        output.save_policy = SavedOutputPolicy::OnDemandFromRetainedState;
    }
    let run = execute(PHASORS, ac(), &[output]);
    assert!(
        run.analyses[0].success,
        "{:?}",
        run.analyses[0].error_message
    );
    let mut state = SimulationState::default();
    state.next_run_id = run.id;
    state.runs = vec![run].into();
    ProjectSimulationResults::from_state(&state)
}

#[test]
fn complex_saved_outputs_keep_policy_and_components_after_reload_and_deferred_evaluation() {
    for policy in ComplexExpressionPolicy::ALL {
        for deferred in [false, true] {
            let stored = history(policy, deferred);
            stored.validate().unwrap();
            let json = serde_json::to_vec(&stored).unwrap();
            let reopened: ProjectSimulationResults = serde_json::from_slice(&json).unwrap();
            let mut state = reopened.into_simulation_state().unwrap();
            let analysis = &mut state.runs[0].analyses[0];
            assert_eq!(analysis.saved_output_receipts[0].complex_policy, policy);
            if deferred {
                materialize_deferred_saved_output(analysis, 0).unwrap();
            }
            let wave = analysis
                .waveforms
                .iter()
                .find(|wave| wave.name == "Difference")
                .unwrap();
            if policy.is_legacy() {
                assert!(wave.complex.is_none());
            } else {
                let complex = wave
                    .complex
                    .as_ref()
                    .expect("rectangular evidence retained");
                for (&real, &imag) in complex.real.iter().zip(complex.imag.iter()) {
                    close(real, 2.0);
                    close(imag, 0.0);
                }
            }
            check_value(
                &state.runs[0],
                0,
                if policy.is_legacy() { 0.0 } else { 2.0 },
            );
            let restorable = ProjectSimulationResults::from_state(&state);
            restorable.validate().unwrap();
        }
    }
}

#[test]
fn complex_policy_is_bound_to_prepared_contract_and_retained_digest() {
    let mut output = output(
        SavedOutputKind::DerivedExpression,
        "Difference",
        "V(a)-V(b)",
    );
    let id = AnalysisInstanceId::new();
    let rectangular = PreparedSavedOutput::prepare(&output, id, &ac())
        .unwrap()
        .unwrap();
    output.complex_policy = ComplexExpressionPolicy::LegacyMagnitude;
    let legacy = PreparedSavedOutput::prepare(&output, id, &ac())
        .unwrap()
        .unwrap();
    assert_ne!(rectangular.digest(), legacy.digest());
    assert_eq!(rectangular.rebind_analysis(id, &ac()).unwrap(), rectangular);
    for policy in ComplexExpressionPolicy::ALL {
        let original = history(policy, true);
        let mut corrupt = original.clone();
        corrupt.runs[0].analyses[0].saved_output_receipts[0].complex_policy = if policy.is_legacy()
        {
            ComplexExpressionPolicy::Rectangular
        } else {
            ComplexExpressionPolicy::LegacyMagnitude
        };
        assert!(corrupt.validate().unwrap_err().contains("digest"));
    }
}

#[test]
fn complex_schema_24_migration_authenticates_before_preserving_legacy_arithmetic() {
    let current = history(ComplexExpressionPolicy::LegacyMagnitude, true);
    let state = current.clone().into_simulation_state().unwrap();
    let run = &state.runs[0];
    let analysis_digest = run.analyses[0].legacy_v13_result_data_digest();
    let dataset_digest = run.legacy_v13_dataset_content_digest();
    let mut historical = current;
    historical.schema_version = 24;
    historical.runs[0].analyses[0].result_data_digest = PersistedField::Value(analysis_digest);
    historical.runs[0].dataset_content_digest = PersistedField::Value(dataset_digest);
    for corrupt_dataset in [false, true] {
        let mut corrupt = historical.clone();
        let wrong = PersistedField::Value(ContentDigest::from_bytes([7; 32]));
        if corrupt_dataset {
            corrupt.runs[0].dataset_content_digest = wrong;
        } else {
            corrupt.runs[0].analyses[0].result_data_digest = wrong;
        }
        let before = corrupt.clone();
        assert!(
            corrupt
                .migrate_to_current(crate::product::ProjectId::new())
                .unwrap_err()
                .contains("digest")
        );
        assert_eq!(corrupt, before, "failed migration must be atomic");
    }
    for schema in 1..25 {
        let mut injected = historical.clone();
        injected.schema_version = schema;
        injected.runs[0].analyses[0].saved_output_receipts[0].complex_policy =
            ComplexExpressionPolicy::Rectangular;
        assert!(
            injected
                .migrate_to_current(crate::product::ProjectId::new())
                .unwrap_err()
                .contains("rectangular")
        );
    }
    historical
        .migrate_to_current(crate::product::ProjectId::new())
        .unwrap();
    assert_eq!(historical.schema_version, 26);
    let mut state = historical.into_simulation_state().unwrap();
    let analysis = &mut state.runs[0].analyses[0];
    assert_eq!(
        analysis.saved_output_receipts[0].complex_policy,
        ComplexExpressionPolicy::LegacyMagnitude
    );
    materialize_deferred_saved_output(analysis, 0).unwrap();
    check_value(&state.runs[0], 0, 0.0);
}
