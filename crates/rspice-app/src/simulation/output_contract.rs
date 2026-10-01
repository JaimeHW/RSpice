//! Application presentation and adoption of exact runtime saved outputs.

use crate::state::{AnalysisResult, DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES, WaveformData};
use rspice_results::waveform::RetainedWaveform;
use rspice_simulation::output_contract::{PreparedSavedOutput, materialization};

#[cfg(test)]
use crate::product::{AnalysisInstanceId, ContentDigest};
#[cfg(test)]
use crate::simulation::config::NoiseSweepType;
#[cfg(test)]
use crate::simulation::multi_run::{AnalysisSpec, FrequencySweep};
#[cfg(test)]
use crate::state::{
    SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputMaterializationStatus,
    SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming,
};
#[cfg(test)]
use rspice_simulation::output_contract::{
    SavedOutputSemanticStatus, SavedOutputStorageEstimate, TransientSelectionGrid,
    compile_saved_output_contracts, preflight_saved_output,
    retained_engine_source_upper_bound_bytes,
};
#[cfg(test)]
use std::sync::Arc;

impl materialization::OutputWaveform for WaveformData {
    fn from_retained(data: RetainedWaveform) -> Self {
        Self {
            data,
            color: "#f5b700".to_owned(),
            visible: true,
            display_cache: None,
        }
    }

    fn reset_display_cache(&mut self) {
        self.display_cache = None;
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    fn rebuild_output_display_cache(&mut self) {
        self.rebuild_display_cache(DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES);
    }

    fn adopt_presentation(&mut self, source: Self) {
        self.display_cache = source.display_cache;
        self.visible = source.visible;
    }

    fn into_output_preview(self, maximum_samples: usize) -> Result<Self, String> {
        self.into_bounded_preview(maximum_samples)
    }
}

pub(in crate::simulation) fn apply_saved_output_policy(
    analysis: &mut AnalysisResult,
    policy: crate::simulation::execution::SavePolicy,
    contracts: &[PreparedSavedOutput],
) {
    materialization::apply_saved_output_policy(&mut analysis.data, policy, contracts);
}

pub(in crate::simulation) fn retain_plan_saved_outputs(
    analysis: &mut AnalysisResult,
    contracts: &[PreparedSavedOutput],
) {
    materialization::retain_plan_saved_outputs(&mut analysis.data, contracts);
}

pub(in crate::simulation) fn materialize_live_saved_outputs(
    analysis: &AnalysisResult,
    contracts: &[PreparedSavedOutput],
) -> Vec<WaveformData> {
    materialization::materialize_live_saved_outputs(
        &analysis.data,
        contracts,
        DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES,
    )
}

pub(crate) fn materialize_deferred_saved_output(
    analysis: &mut AnalysisResult,
    receipt_index: usize,
) -> Result<(), String> {
    materialization::materialize_deferred_saved_output(&mut analysis.data, receipt_index)
}

#[cfg(test)]
fn materialize_saved_outputs(analysis: &mut AnalysisResult, contracts: &[PreparedSavedOutput]) {
    materialization::materialize_saved_outputs(&mut analysis.data, contracts);
}

#[cfg(test)]
fn resolve_raw_probe(
    expression: &str,
    waveforms: &[WaveformData],
    output_name: &str,
    complex_domain: bool,
) -> Result<WaveformData, String> {
    materialization::resolve_raw_probe(expression, waveforms, output_name, complex_domain)
}

#[cfg(test)]
fn resample_selected_and_final(
    waveform: &WaveformData,
    grid: TransientSelectionGrid,
) -> Result<WaveformData, String> {
    materialization::resample_selected_and_final(waveform, grid)
}

#[cfg(test)]
mod binding_tests;
#[cfg(test)]
mod complex_tests;
#[cfg(test)]
mod dc_family_tests;
#[cfg(test)]
mod durable_binding_tests;
#[cfg(test)]
mod fixtures;
#[cfg(test)]
mod probe_tests;
#[cfg(test)]
mod quasi_periodic_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::AnalysisType;

    #[test]
    fn raw_probes_keep_voltage_and_current_namespaces_separate() {
        for voltage_name in ["V(V1)", "V1", "|V(V1)|"] {
            let voltage = WaveformData::new(voltage_name, vec![0.0, 1.0], vec![0.0, 1.0], "#fff");
            let current = WaveformData::new("I(V1)", vec![0.0, 1.0], vec![0.0, -0.001], "#fff");
            for traces in [
                vec![current.clone(), voltage.clone()],
                vec![voltage.clone(), current.clone()],
            ] {
                assert_eq!(
                    resolve_raw_probe("v(v1)", &traces, "Voltage", false)
                        .unwrap()
                        .y
                        .as_slice(),
                    &[0.0, 1.0]
                );
                assert_eq!(
                    resolve_raw_probe("i(v1)", &traces, "Current", false)
                        .unwrap()
                        .y
                        .as_slice(),
                    &[0.0, -0.001]
                );
            }
            assert!(resolve_raw_probe("I(V1)", &[voltage], "missing current", false).is_err());
            assert!(resolve_raw_probe("V(V1)", &[current], "missing voltage", false).is_err());
        }
    }

    fn output(policy: SavedOutputPolicy, precision: SavedOutputPrecision) -> SavedOutput {
        SavedOutput::new(
            SavedOutputKind::RawVoltageOrCurrent,
            "output_voltage",
            "V(out)",
            SavedOutputCompatibility::AllCompatibleAnalyses,
            policy,
            precision,
            SavedOutputStreaming::StoreOnly,
        )
        .expect("valid output")
    }

    fn transient_spec() -> AnalysisSpec {
        AnalysisSpec::Transient {
            stop_time: 1.0,
            step_time: 0.25,
            start_time: 0.0,
            max_timestep: Some(0.1),
            uic: false,
        }
    }

    #[test]
    fn every_accepted_materializes_exact_source_and_receipt() {
        let contract = PreparedSavedOutput::prepare(
            &output(
                SavedOutputPolicy::EveryAcceptedPoint,
                SavedOutputPrecision::FullSourcePrecision,
            ),
            AnalysisInstanceId::new(),
            &transient_spec(),
        )
        .expect("prepare")
        .expect("applies");
        let mut analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new(
                    "out",
                    vec![0.0, 0.1, 0.4, 1.0],
                    vec![0.0, 1.0, 4.0, 10.0],
                    "#fff",
                ),
            ]);
        materialize_saved_outputs(&mut analysis, &[contract]);
        assert_eq!(analysis.waveforms[1].name, "output_voltage");
        assert_eq!(
            analysis.waveforms[1].x.as_ref(),
            analysis.waveforms[0].x.as_ref()
        );
        assert!(matches!(
            analysis.saved_output_receipts[0].status,
            SavedOutputMaterializationStatus::Materialized {
                sample_count: 4,
                ..
            }
        ));
    }

    #[test]
    fn selected_and_final_uses_configured_grid_and_exact_final() {
        let contract = PreparedSavedOutput::prepare(
            &output(
                SavedOutputPolicy::SelectedAndFinalPoints,
                SavedOutputPrecision::FullSourcePrecision,
            ),
            AnalysisInstanceId::new(),
            &transient_spec(),
        )
        .expect("prepare")
        .expect("applies");
        let mut analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new(
                    "out",
                    vec![0.0, 0.1, 0.4, 0.8, 1.0],
                    vec![0.0, 1.0, 4.0, 8.0, 10.0],
                    "#fff",
                ),
            ]);
        materialize_saved_outputs(&mut analysis, &[contract]);
        assert_eq!(
            analysis.waveforms[1].x.as_ref(),
            &[0.0, 0.25, 0.5, 0.75, 1.0]
        );
        for (actual, expected) in analysis.waveforms[1]
            .y
            .iter()
            .zip([0.0_f64, 2.5, 5.0, 7.5, 10.0])
        {
            let tolerance = 4.0 * f64::EPSILON * expected.abs().max(1.0);
            assert!(
                (actual - expected).abs() <= tolerance,
                "interpolated {actual:.17e} differs from {expected:.17e} by more than {tolerance:.3e}"
            );
        }
    }

    #[test]
    fn on_demand_is_deferred_then_materializes_from_retained_source() {
        let contract = PreparedSavedOutput::prepare(
            &output(
                SavedOutputPolicy::OnDemandFromRetainedState,
                SavedOutputPrecision::DisplayCacheWithFullSourcePrecision,
            ),
            AnalysisInstanceId::new(),
            &transient_spec(),
        )
        .expect("prepare")
        .expect("applies");
        let mut analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("out", vec![0.0, 1.0], vec![0.0, 2.0], "#fff"),
            ]);
        materialize_saved_outputs(&mut analysis, &[contract]);
        assert_eq!(analysis.waveforms.len(), 1);
        assert_eq!(
            analysis.saved_output_receipts[0].status,
            SavedOutputMaterializationStatus::Deferred
        );
        materialize_deferred_saved_output(&mut analysis, 0).expect("deferred materializes");
        assert_eq!(analysis.waveforms.len(), 2);
        assert!(analysis.waveforms[1].display_cache.is_some());
    }

    #[test]
    fn deferred_materialization_is_atomic_when_the_output_name_collides() {
        let contract = PreparedSavedOutput::prepare(
            &output(
                SavedOutputPolicy::OnDemandFromRetainedState,
                SavedOutputPrecision::FullSourcePrecision,
            ),
            AnalysisInstanceId::new(),
            &transient_spec(),
        )
        .expect("prepare")
        .expect("applies");
        let mut analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("out", vec![0.0, 1.0], vec![0.0, 2.0], "#fff"),
                WaveformData::new("output_voltage", vec![0.0, 1.0], vec![9.0, 9.0], "#f00"),
            ]);
        materialize_saved_outputs(&mut analysis, &[contract]);
        let before_waveforms = analysis.waveforms.clone();

        let error = materialize_deferred_saved_output(&mut analysis, 0)
            .expect_err("different retained waveform must block materialization");

        assert!(error.contains("collides"));
        assert_eq!(analysis.waveforms, before_waveforms);
        assert_eq!(
            analysis.saved_output_receipts[0].status,
            SavedOutputMaterializationStatus::Deferred
        );
    }

    #[test]
    fn live_output_projection_is_bounded_and_does_not_mutate_source_precision() {
        let live_output = SavedOutput::new(
            SavedOutputKind::RawVoltageOrCurrent,
            "output_voltage",
            "V(out)",
            SavedOutputCompatibility::AllCompatibleAnalyses,
            SavedOutputPolicy::EveryAcceptedPoint,
            SavedOutputPrecision::DisplayCacheWithFullSourcePrecision,
            SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation,
        )
        .expect("valid live output");
        let contract = PreparedSavedOutput::prepare(
            &live_output,
            AnalysisInstanceId::new(),
            &transient_spec(),
        )
        .expect("prepare")
        .expect("applies");
        let x = (0..10_000).map(|index| index as f64).collect::<Vec<_>>();
        let y = x
            .iter()
            .map(|value| (value / 17.0).sin())
            .collect::<Vec<_>>();
        let source = AnalysisResult::live_transient_partial(1, AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![WaveformData::new("out", x.clone(), y, "#fff")]);

        let projected = materialize_live_saved_outputs(&source, &[contract]);

        assert_eq!(source.waveforms[0].x.len(), 10_000);
        assert_eq!(projected.len(), 1);
        assert_eq!(projected[0].name, "output_voltage");
        assert!(projected[0].x.len() <= DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES);
        assert!(projected[0].display_cache.is_some());
        assert!(source.saved_output_receipts.is_empty());
    }

    #[test]
    fn failure_only_is_suppressed_on_success_and_materializes_partial_failure_data() {
        let contract = PreparedSavedOutput::prepare(
            &output(
                SavedOutputPolicy::FailureDiagnosticsOnly,
                SavedOutputPrecision::FullSourcePrecision,
            ),
            AnalysisInstanceId::new(),
            &transient_spec(),
        )
        .expect("prepare")
        .expect("applies");
        let source = WaveformData::new("out", vec![0.0, 0.1], vec![0.0, 1.0], "#fff");
        let mut success = AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![source.clone()]);
        materialize_saved_outputs(&mut success, std::slice::from_ref(&contract));
        assert_eq!(
            success.saved_output_receipts[0].status,
            SavedOutputMaterializationStatus::SuppressedOnSuccess
        );

        let mut failed = AnalysisResult::failed(1, AnalysisType::Transient, "TRAN", "failed")
            .with_waveforms(vec![source]);
        materialize_saved_outputs(&mut failed, &[contract]);
        assert!(matches!(
            failed.saved_output_receipts[0].status,
            SavedOutputMaterializationStatus::Materialized {
                sample_count: 2,
                ..
            }
        ));
    }

    #[test]
    fn plan_retention_discards_unselected_engine_waveforms() {
        let contract = PreparedSavedOutput::prepare(
            &output(
                SavedOutputPolicy::EveryAcceptedPoint,
                SavedOutputPrecision::FullSourcePrecision,
            ),
            AnalysisInstanceId::new(),
            &transient_spec(),
        )
        .expect("prepare")
        .expect("applies");
        let mut analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("out", vec![0.0, 1.0], vec![0.0, 2.0], "#fff"),
                WaveformData::new("internal", vec![0.0, 1.0], vec![4.0, 5.0], "#aaa"),
            ]);

        retain_plan_saved_outputs(&mut analysis, &[contract]);

        assert_eq!(analysis.waveforms.len(), 1);
        assert_eq!(analysis.waveforms[0].name, "output_voltage");
        assert!(matches!(
            analysis.saved_output_receipts[0].status,
            SavedOutputMaterializationStatus::Materialized { .. }
        ));
    }

    #[test]
    fn retained_output_keeps_save_and_initial_display_intent_separate() {
        let saved = output(
            SavedOutputPolicy::EveryAcceptedPoint,
            SavedOutputPrecision::FullSourcePrecision,
        )
        .with_display_intent(crate::state::SavedOutputDisplayIntent::DataBrowserOnly);
        let contract =
            PreparedSavedOutput::prepare(&saved, AnalysisInstanceId::new(), &transient_spec())
                .expect("prepare")
                .expect("applies");
        let mut analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("out", vec![0.0, 1.0], vec![0.0, 2.0], "#fff"),
            ]);

        retain_plan_saved_outputs(&mut analysis, &[contract]);

        assert_eq!(analysis.waveforms.len(), 1);
        assert!(!analysis.waveforms[0].visible);
        assert_eq!(
            analysis.saved_output_receipts[0].display_intent,
            crate::state::SavedOutputDisplayIntent::DataBrowserOnly
        );
    }

    #[test]
    fn empty_plan_output_registry_retains_no_engine_waveforms() {
        let mut analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("out", vec![0.0, 1.0], vec![0.0, 2.0], "#fff"),
            ]);

        retain_plan_saved_outputs(&mut analysis, &[]);

        assert!(analysis.waveforms.is_empty());
        assert!(analysis.saved_output_receipts.is_empty());
    }

    #[test]
    fn on_demand_retention_keeps_source_and_reports_plan_level_source_ownership() {
        let deferred = output(
            SavedOutputPolicy::OnDemandFromRetainedState,
            SavedOutputPrecision::FullSourcePrecision,
        );
        let analysis_id = AnalysisInstanceId::new();
        let contract = PreparedSavedOutput::prepare(&deferred, analysis_id, &transient_spec())
            .expect("prepare")
            .expect("applies");
        let mut analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("out", vec![0.0, 1.0], vec![0.0, 2.0], "#fff"),
            ]);

        retain_plan_saved_outputs(&mut analysis, &[contract]);
        let report = preflight_saved_output(
            &deferred,
            [(analysis_id, &transient_spec())],
            crate::state::DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES,
        );

        assert_eq!(analysis.waveforms.len(), 1);
        assert_eq!(
            analysis.saved_output_receipts[0].status,
            SavedOutputMaterializationStatus::Deferred
        );
        assert_eq!(
            report.storage_estimate(),
            &SavedOutputStorageEstimate::ExactBytes(0)
        );
        assert_eq!(report.retained_engine_source_analysis_ids(), &[analysis_id]);
        assert_eq!(
            retained_engine_source_upper_bound_bytes(1),
            25_000_000 * std::mem::size_of::<f64>() as u64
        );
    }

    #[test]
    fn sp_port_bounds_and_multi_digit_indices_bind_to_the_retained_circuit() {
        let output = SavedOutput::new(
            SavedOutputKind::RfPortQuantity,
            "forward_gain",
            "S(10,1)",
            SavedOutputCompatibility::AllCompatibleAnalyses,
            SavedOutputPolicy::EveryAcceptedPoint,
            SavedOutputPrecision::FullSourcePrecision,
            SavedOutputStreaming::StoreOnly,
        )
        .expect("syntactically valid output");
        let spec = AnalysisSpec::SParameter {
            do_noise: false,
            start_freq: 1.0e6,
            stop_freq: 1.0e9,
            points_per_unit: 10,
            sweep: FrequencySweep::Decade,
            z0: 50.0,
            ports: vec![
                crate::simulation::multi_run::SpPort {
                    node_pos: "in".to_owned(),
                    node_neg: "0".to_owned(),
                    z0: None,
                },
                crate::simulation::multi_run::SpPort {
                    node_pos: "out".to_owned(),
                    node_neg: "0".to_owned(),
                    z0: None,
                },
            ],
        };
        let report = preflight_saved_output(
            &output,
            [(AnalysisInstanceId::new(), &spec)],
            crate::state::DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES,
        );
        assert!(matches!(
            report.semantic_status(),
            SavedOutputSemanticStatus::RuntimeBound { reason } if reason.contains("elaborated circuit")
        ));
        let contract = PreparedSavedOutput::prepare(&output, AnalysisInstanceId::new(), &spec)
            .unwrap()
            .unwrap();
        let source = WaveformData::new("S10_1", vec![1e6], vec![0.5], "#fff")
            .with_complex_components("S10_1", vec![0.5], vec![-0.1]);
        let mut analysis =
            AnalysisResult::new(1, AnalysisType::SParameter, "SP").with_waveforms(vec![source]);
        materialize_saved_outputs(&mut analysis, std::slice::from_ref(&contract));
        assert!(matches!(
            analysis.saved_output_receipts[0].status,
            SavedOutputMaterializationStatus::Materialized {
                sample_count: 1,
                ..
            }
        ));
        assert_eq!(analysis.waveforms[1].name, "forward_gain");
        assert_eq!(
            analysis.waveforms[1]
                .complex
                .as_ref()
                .unwrap()
                .imag
                .as_ref(),
            &[-0.1]
        );
        let mut missing = AnalysisResult::new(1, AnalysisType::SParameter, "SP");
        materialize_saved_outputs(&mut missing, &[contract]);
        assert!(matches!(
            missing.saved_output_receipts[0].status,
            SavedOutputMaterializationStatus::Unavailable { .. }
        ));
    }
}
