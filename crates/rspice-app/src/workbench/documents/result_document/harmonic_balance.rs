//! Coefficient-spectrum source qualification and application viewport ownership.

use crate::state::{AnalysisResult, AnalysisType, WaveformData};
use crate::ui::tokens::Tokens;
use crate::workbench::AppState;
use egui::Ui;
use rspice_results_ui::harmonic_balance::{
    self as view, HarmonicBalanceModel, HarmonicTrace, recorded_fft,
};
use std::sync::Arc;

pub(super) fn spectrum_trace_is_renderable(waveform: &WaveformData) -> bool {
    rspice_results::harmonic_spectrum::spectrum_trace_is_renderable(waveform.as_ref(), || {
        super::frame_work::note(super::frame_work::DatasetWalk::HarmonicSpectrumScan);
    })
}

pub(super) fn analysis_is_renderable(analysis: &AnalysisResult) -> bool {
    rspice_results::harmonic_spectrum::analysis_is_renderable(
        analysis.success,
        analysis.analysis_type,
        &analysis.waveforms,
        || super::frame_work::note(super::frame_work::DatasetWalk::HarmonicSpectrumScan),
    )
}

pub(super) fn active_analysis_is_renderable(state: &AppState) -> bool {
    state.simulation.active_run().is_some_and(|run| {
        run.analyses.iter().any(|analysis| {
            super::analysis_answers_structural_gate(
                state,
                run.dataset_id,
                analysis,
                super::StructuralGate::HarmonicSpectrum,
            )
        })
    })
}

fn selected_hb_analysis(state: &AppState) -> Option<&AnalysisResult> {
    let run = state.simulation.active_run()?;
    state
        .simulation
        .active_analysis_idx
        .and_then(|index| run.analyses.get(index))
        .filter(|analysis| analysis_is_renderable(analysis))
        .or_else(|| {
            run.analyses
                .iter()
                .find(|analysis| analysis_is_renderable(analysis))
        })
}

fn build_model(state: &AppState, tokens: &Tokens) -> Option<HarmonicBalanceModel> {
    let analysis = selected_hb_analysis(state)?;

    let palette = &tokens.color.traces;
    let mut traces = Vec::new();
    let mut frequency_min = f64::INFINITY;
    let mut frequency_max = f64::NEG_INFINITY;
    let mut magnitude_min = f64::INFINITY;
    let mut magnitude_max = f64::NEG_INFINITY;
    let mut smallest_positive_magnitude: Option<f64> = None;

    for waveform in analysis
        .waveforms
        .iter()
        .enumerate()
        .filter(|(_, waveform)| waveform.visible && spectrum_trace_is_renderable(waveform))
    {
        let (waveform_index, waveform) = waveform;
        for (&frequency, &magnitude) in waveform.x.iter().zip(waveform.y.iter()) {
            frequency_min = frequency_min.min(frequency);
            frequency_max = frequency_max.max(frequency);
            magnitude_min = magnitude_min.min(magnitude);
            magnitude_max = magnitude_max.max(magnitude);
            if magnitude > 0.0 {
                smallest_positive_magnitude = Some(
                    smallest_positive_magnitude.map_or(magnitude, |held: f64| held.min(magnitude)),
                );
            }
        }
        traces.push(HarmonicTrace {
            name: waveform.complex.as_ref().map_or_else(
                || waveform.name.clone(),
                |complex| complex.source_name.clone(),
            ),
            frequency: Arc::clone(&waveform.x),
            magnitude: Arc::clone(&waveform.y),
            color: palette[traces.len() % palette.len()],
            cache_key: harmonic_trace_cache_key(analysis.id, waveform_index),
        });
    }

    if traces.is_empty()
        || !frequency_min.is_finite()
        || !frequency_max.is_finite()
        || !magnitude_min.is_finite()
        || !magnitude_max.is_finite()
    {
        return None;
    }

    let retained_frequency_count = traces.first().map_or(0, |trace| trace.frequency.len());
    Some(HarmonicBalanceModel {
        label: analysis.label.clone(),
        traces,
        frequency_max,
        magnitude_min,
        magnitude_max,
        retained_frequency_count,
        fft: recorded_fft::evidence(&analysis.data).cloned(),
        qpss: match &analysis.result_payload {
            Some(crate::state::AnalysisResultPayload::Qpss { operating_point }) => {
                Some(Arc::clone(operating_point))
            }
            _ => None,
        },
        smallest_positive_magnitude,
    })
}

const fn harmonic_trace_cache_key(analysis_id: u64, waveform_index: usize) -> u64 {
    0x48B0_0000_0000_0000_u64 ^ analysis_id.rotate_left(23) ^ (waveform_index as u64).rotate_left(7)
}

fn active_incomplete_fft(state: &AppState) -> Option<String> {
    let analysis = state.simulation.active_analysis()?;
    analysis
        .success
        .then(|| recorded_fft::incomplete_history_sentence(&analysis.data))?
}

fn active_hb_failure(state: &AppState) -> Option<&str> {
    let analysis = state.simulation.active_analysis()?;
    (matches!(
        analysis.analysis_type,
        AnalysisType::HarmonicBalance | AnalysisType::Fourier | AnalysisType::Qpss
    ) && !analysis.success)
        .then(|| {
            analysis
                .error_message
                .as_deref()
                .unwrap_or("spectrum execution failed")
        })
}

fn absence(state: &AppState) -> view::SpectrumAbsence<'_> {
    if let Some(error) = active_hb_failure(state) {
        view::SpectrumAbsence::Failed(error)
    } else if let Some(sentence) = active_incomplete_fft(state) {
        view::SpectrumAbsence::IncompleteHistory(sentence)
    } else {
        view::SpectrumAbsence::Unavailable
    }
}

/// Bind the retained spectrum and apply navigation to its host viewport.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let tokens = Tokens::get(ui.ctx());
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let Some(model) = build_model(state, &tokens) else {
        view::show_absent(ui, absence(state));
        return;
    };
    let viewport = state
        .ui
        .results
        .plot_view(super::ResultViewer::HarmonicBalance, 0);
    let response = view::show(
        ui,
        &model,
        viewport,
        &quantity_policy,
        &mut state.ui.results.cache,
    );
    if response.fit {
        state
            .ui
            .results
            .reset_plot_view(super::ResultViewer::HarmonicBalance, 0);
    }
    if let Some(response) = response.plot {
        super::record_drawn_axes(
            &mut state.ui.results,
            super::ResultViewer::HarmonicBalance,
            &response,
        );
        if response.view.any() {
            state
                .ui
                .results
                .plot_view_mut(super::ResultViewer::HarmonicBalance, 0)
                .apply(&response.view);
        }
    }
}

/// Bind the inspector to the same qualified spectrum and recorded payload.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let tokens = Tokens::get(ui.ctx());
    let model = build_model(state, &tokens);
    let source = model.as_ref().ok_or_else(|| absence(state));
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    view::right_panel(ui, source, &quantity_policy);
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    fn spectrum_waveform() -> WaveformData {
        WaveformData::new(
            "|V(out)|",
            Arc::new(vec![0.0, 1.0e6, 2.0e6]),
            Arc::new(vec![0.1, 1.0, 0.2]),
            "#ffffff",
        )
        .with_complex_components(
            "V(out)",
            Arc::new(vec![0.1, 1.0, 0.2]),
            Arc::new(vec![0.0, 0.0, 0.0]),
        )
    }

    #[test]
    fn hb_viewer_accepts_only_complex_retained_coefficients() {
        let result = AnalysisResult::new(1, AnalysisType::HarmonicBalance, "HB")
            .with_waveforms(vec![spectrum_waveform()]);
        assert!(analysis_is_renderable(&result));

        let phase_only = AnalysisResult::new(1, AnalysisType::HarmonicBalance, "HB")
            .with_waveforms(vec![WaveformData::new(
                "phase(V(out))",
                Arc::new(vec![0.0, 1.0e6]),
                Arc::new(vec![0.0, 45.0]),
                "#ffffff",
            )]);
        assert!(!analysis_is_renderable(&phase_only));

        let fourier = AnalysisResult::new(2, AnalysisType::Fourier, "FOURIER")
            .with_waveforms(vec![spectrum_waveform()]);
        assert!(analysis_is_renderable(&fourier));
    }

    #[test]
    fn hb_viewer_rejects_malformed_or_wrong_family_data() {
        let malformed = WaveformData::new(
            "|V(out)|",
            Arc::new(vec![1.0e6, 0.5e6]),
            Arc::new(vec![1.0, 0.5]),
            "#ffffff",
        )
        .with_complex_components(
            "V(out)",
            Arc::new(vec![1.0, 0.5]),
            Arc::new(vec![0.0, 0.0]),
        );
        assert!(!spectrum_trace_is_renderable(&malformed));

        let wrong_family = AnalysisResult::new(1, AnalysisType::Ac, "AC")
            .with_waveforms(vec![spectrum_waveform()]);
        assert!(!analysis_is_renderable(&wrong_family));
    }

    #[test]
    fn trace_cache_identity_survives_visibility_filtering() {
        assert_ne!(
            harmonic_trace_cache_key(7, 0),
            harmonic_trace_cache_key(7, 1)
        );
        assert_ne!(
            harmonic_trace_cache_key(7, 0),
            harmonic_trace_cache_key(8, 0)
        );
    }
}
