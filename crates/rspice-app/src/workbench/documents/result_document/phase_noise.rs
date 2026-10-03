//! Phase-noise source qualification, retained trace binding and viewport ownership.

use crate::state::{AnalysisResult, AnalysisType, SharedWaveformValues, WaveformData};
#[cfg(test)]
use crate::state::{AnalysisResultFamilyMetadata, PeriodicNoiseOutputQuantity};
use crate::workbench::AppState;
use egui::Ui;
#[cfg(test)]
use rspice_results::phase_noise::retained_measurement;
use rspice_results::phase_noise::retained_phase_noise_carrier;
use rspice_results_ui::phase_noise as view;
use std::sync::Arc;

/// Exactly one phase-noise waveform selected from the active immutable run.
struct PhaseNoiseModel {
    analysis_index: usize,
    waveform_index: usize,
    label: String,
    source: String,
    carrier_frequency_hz: f64,
    offset_hz: SharedWaveformValues,
    level_dbc_per_hz: SharedWaveformValues,
}

impl PhaseNoiseModel {
    fn trace(&self) -> view::PhaseNoiseTrace<'_> {
        view::PhaseNoiseTrace {
            label: &self.label,
            source: &self.source,
            carrier_frequency_hz: self.carrier_frequency_hz,
            offset_hz: &self.offset_hz,
            level_dbc_per_hz: &self.level_dbc_per_hz,
            cache_key: 0x504E_0000_u64
                | ((self.analysis_index as u64) << 16)
                | self.waveform_index as u64,
        }
    }
}

pub(super) fn phase_noise_waveform_is_renderable(waveform: &WaveformData) -> bool {
    rspice_results::phase_noise::phase_noise_waveform_is_renderable(waveform.as_ref(), || {
        super::frame_work::note(super::frame_work::DatasetWalk::PhaseNoiseSpectrumScan);
    })
}

pub(super) fn phase_noise_is_renderable(analysis: &AnalysisResult) -> bool {
    rspice_results::phase_noise::phase_noise_is_renderable(
        analysis.success,
        analysis.analysis_type,
        analysis.family_metadata.as_ref(),
        &analysis.waveforms,
        || super::frame_work::note(super::frame_work::DatasetWalk::PhaseNoiseSpectrumScan),
    )
}

fn selected_phase_noise_analysis_index(state: &AppState) -> Option<usize> {
    let run = state.simulation.active_run()?;
    // Through the workspace memo: the predicate walks every retained offset
    // and level, and this resolves on every frame the sheet is open.
    let renderable = |analysis: &AnalysisResult| {
        super::analysis_answers_structural_gate(
            state,
            run.dataset_id,
            analysis,
            super::StructuralGate::PhaseNoiseSpectrum,
        )
    };
    state
        .simulation
        .active_analysis_idx
        .filter(|&index| run.analyses.get(index).is_some_and(&renderable))
        .or_else(|| run.analyses.iter().position(renderable))
}

fn model_from_analysis(
    analysis_index: usize,
    analysis: &AnalysisResult,
) -> Option<PhaseNoiseModel> {
    if !phase_noise_is_renderable(analysis) {
        return None;
    }
    let (waveform_index, waveform) = analysis
        .waveforms
        .iter()
        .enumerate()
        .find(|(_, waveform)| phase_noise_waveform_is_renderable(waveform))?;
    Some(PhaseNoiseModel {
        analysis_index,
        waveform_index,
        label: analysis.label.clone(),
        source: waveform.name.clone(),
        carrier_frequency_hz: retained_phase_noise_carrier(
            analysis.analysis_type,
            analysis.family_metadata.as_ref(),
        )?,
        offset_hz: Arc::clone(&waveform.x),
        level_dbc_per_hz: Arc::clone(&waveform.y),
    })
}

fn build_model(state: &AppState) -> Option<PhaseNoiseModel> {
    let run = state.simulation.active_run()?;
    let analysis_index = selected_phase_noise_analysis_index(state)?;
    model_from_analysis(analysis_index, run.analyses.get(analysis_index)?)
}

fn active_periodic_noise_without_phase_trace(state: &AppState) -> bool {
    let Some(run) = state.simulation.active_run() else {
        return false;
    };
    let Some(index) = state.simulation.active_analysis_idx else {
        return false;
    };
    let Some(analysis) = run.analyses.get(index) else {
        return false;
    };
    matches!(
        analysis.analysis_type,
        AnalysisType::Pnoise | AnalysisType::Qpnoise
    ) && !phase_noise_is_renderable(analysis)
}

fn finite_range(values: &[f64]) -> Option<(f64, f64)> {
    super::finite_extremes(values)
}

/// Resolve the displayed source and apply requests to the phase-noise viewport.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let quantities = state.ui.preferences.quantity_presentation_policy();
    let Some(model) = build_model(state) else {
        view::show_absent(ui, active_periodic_noise_without_phase_trace(state));
        return;
    };
    let viewport = state
        .ui
        .results
        .plot_view(super::ResultViewer::PhaseNoise, 0);
    let level_range = finite_range(&model.level_dbc_per_hz);
    let response = view::show(
        ui,
        &model.trace(),
        viewport,
        level_range,
        &quantities,
        &mut state.ui.results.cache,
    );
    if response.fit {
        state
            .ui
            .results
            .reset_plot_view(super::ResultViewer::PhaseNoise, 0);
    }
    if let Some(response) = response.plot {
        super::record_drawn_axes(
            &mut state.ui.results,
            super::ResultViewer::PhaseNoise,
            &response,
        );
        if response.view.any() {
            state
                .ui
                .results
                .plot_view_mut(super::ResultViewer::PhaseNoise, 0)
                .apply(&response.view);
        }
    }
}

/// Bind inspector measurements to the selected phase-noise trace.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let quantities = state.ui.preferences.quantity_presentation_policy();
    let model = build_model(state);
    let input = model.as_ref().map(|model| view::PhaseNoiseInspector {
        trace: model.trace(),
        analysis: &state.simulation.active_run().unwrap().analyses[model.analysis_index].data,
        offset_range: finite_range(&model.offset_hz),
    });
    view::right_panel(ui, input, &quantities);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pnoise_inspector_uses_retained_jitter_and_never_a_failed_measurement() {
        let mut analysis = phase_result(1, AnalysisType::Pnoise, "PNOISE");
        analysis.measurements = vec![
            rspice_core::MeasureResult::success("phase_error_rms_rad", 0.02),
            rspice_core::MeasureResult::success("timing_jitter_rms_s", 3e-12),
        ];
        assert_eq!(
            retained_measurement(&analysis.data, "phase_error_rms_rad"),
            Some(0.02)
        );
        assert_eq!(
            retained_measurement(&analysis.data, "timing_jitter_rms_s"),
            Some(3e-12)
        );
        analysis.measurements[1].passed = false;
        assert_eq!(
            retained_measurement(&analysis.data, "timing_jitter_rms_s"),
            None
        );
        analysis
            .data
            .measurements
            .push(analysis.data.measurements[0].clone());
        assert_eq!(
            retained_measurement(&analysis.data, "phase_error_rms_rad"),
            None
        );
    }

    fn waveform(name: &str) -> WaveformData {
        WaveformData::new(
            name,
            vec![1.0, 1.0e3, 1.0e6],
            vec![-72.0, -103.0, -132.0],
            "#f5b700",
        )
    }

    fn phase_result(id: u64, analysis_type: AnalysisType, label: &str) -> AnalysisResult {
        AnalysisResult::new(id, analysis_type, label)
            .with_waveforms(vec![waveform("phase_noise")])
            .with_family_metadata(AnalysisResultFamilyMetadata::PeriodicNoise {
                output_quantity: PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
                carrier_frequency_hz: Some(2.4e9),
            })
    }

    #[test]
    fn phase_noise_model_requires_explicit_trace_evidence() {
        let ordinary = AnalysisResult::new(1, AnalysisType::Pnoise, "PNOISE")
            .with_waveforms(vec![waveform("onoise")]);
        assert!(!phase_noise_is_renderable(&ordinary));
        assert!(model_from_analysis(0, &ordinary).is_none());

        let phase = phase_result(2, AnalysisType::Pnoise, "PNOISE phase");
        let model = model_from_analysis(3, &phase).expect("explicit phase trace renders");
        assert_eq!(model.analysis_index, 3);
        assert_eq!(model.source, "phase_noise");
        assert!(phase_noise_is_renderable(&phase));
    }

    #[test]
    fn qpnoise_and_l_of_f_are_accepted_but_other_analysis_kinds_are_not() {
        let mut qpnoise = phase_result(1, AnalysisType::Qpnoise, "QPNOISE");
        qpnoise.waveforms = vec![waveform("L(f)")];
        assert!(phase_noise_is_renderable(&qpnoise));

        let mut noise = AnalysisResult::new(1, AnalysisType::Noise, "NOISE")
            .with_waveforms(vec![waveform("phase noise")]);
        noise.family_metadata = Some(AnalysisResultFamilyMetadata::PeriodicNoise {
            output_quantity: PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
            carrier_frequency_hz: Some(2.4e9),
        });
        assert!(!phase_noise_is_renderable(&noise));
    }

    #[test]
    fn failed_phase_noise_analysis_never_becomes_renderable_evidence() {
        let mut failed = phase_result(1, AnalysisType::Pnoise, "PNOISE failed");
        failed.success = false;

        assert!(!phase_noise_is_renderable(&failed));
        assert!(model_from_analysis(0, &failed).is_none());
    }

    #[test]
    fn phase_noise_requires_typed_quantity_and_retained_carrier() {
        let output_psd = AnalysisResult::new(1, AnalysisType::Pnoise, "PNOISE output")
            .with_waveforms(vec![waveform("phase_noise")])
            .with_family_metadata(AnalysisResultFamilyMetadata::PeriodicNoise {
                output_quantity: PeriodicNoiseOutputQuantity::OutputNoisePowerSpectralDensity,
                carrier_frequency_hz: Some(2.4e9),
            });
        assert!(!phase_noise_is_renderable(&output_psd));

        let mut missing_carrier = AnalysisResult::new(2, AnalysisType::Pnoise, "PNOISE phase")
            .with_waveforms(vec![waveform("phase_noise")]);
        missing_carrier.family_metadata = Some(AnalysisResultFamilyMetadata::PeriodicNoise {
            output_quantity: PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
            carrier_frequency_hz: None,
        });
        assert!(!phase_noise_is_renderable(&missing_carrier));
    }
}
