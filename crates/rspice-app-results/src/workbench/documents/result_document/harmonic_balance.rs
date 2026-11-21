//! Coefficient-spectrum source qualification and application viewport ownership.

use super::{ResultsParts, ResultsReads, view_edits};
use crate::results::visualization_document::DocumentEdit;
use crate::state::{AnalysisResult, AnalysisType, RetainedTrace};
use crate::ui::tokens::Tokens;
use egui::Ui;
use rspice_results::harmonic_spectrum::lines::CoefficientLine;
use rspice_results::harmonic_spectrum::settings::CoefficientSettings;
use rspice_results_ui::harmonic_balance::{
    self as view, HarmonicBalanceModel, HarmonicTrace, recorded_fft,
};
use rspice_results_ui::session::PersistentPaneContext;
use std::sync::Arc;

/// The settings the view that keeps the sheet holds for it: the reader's,
/// or the sheet's defaults until they change one. No pane when no view
/// keeps the sheet.
fn kept_settings(state: ResultsReads<'_>) -> (Option<PersistentPaneContext>, CoefficientSettings) {
    let Some(pane) = super::fft::sheet_pane(state) else {
        return (None, CoefficientSettings::default());
    };
    let settings = state
        .model
        .workspace
        .content
        .visualization_document(pane.document_id)
        .and_then(|document| document.coefficient_settings(pane.pane_id).cloned())
        .unwrap_or_default();
    (Some(pane), settings)
}

/// Keep `settings` the reader set on the sheet in the view that keeps it,
/// through its one writer, as a step the reader can undo.
fn keep_settings(state: &mut ResultsParts<'_>, settings: CoefficientSettings) {
    let (Some(pane), held) = kept_settings(state.results_reads()) else {
        return;
    };
    if held == settings {
        return;
    }
    let Some(revision) = state
        .model
        .workspace
        .content
        .visualization_document(pane.document_id)
        .map(|document| document.revision())
    else {
        return;
    };
    let edit = DocumentEdit::SetCoefficientSettings {
        pane_id: pane.pane_id,
        settings: (settings != CoefficientSettings::default()).then_some(settings),
    };
    if let Err(error) = view_edits::apply(state, pane.document_id, revision, vec![edit]) {
        state.prompt(format!("Could not keep this change to the view: {error}"));
    }
}

/// The level, amplitude and strips in the sheet bar. A recorded `.FFT` is
/// read on its own decades and owns no controls.
pub(super) fn domain_bar(ui: &mut Ui, state: &mut ResultsParts<'_>) -> bool {
    let tokens = Tokens::get(ui.ctx());
    let Some(model) = build_model(state.results_reads(), &tokens) else {
        return false;
    };
    if model.fft.is_some() {
        return false;
    }
    let (_, held) = kept_settings(state.results_reads());
    if let Some(settings) = view::domain_bar(ui, &held) {
        keep_settings(state, settings);
    }
    true
}

/// A trace's complex lines, read whole like its magnitudes: a coefficient
/// spectrum holds tens of lines, and the sheet draws every one.
fn coefficient_lines(waveform: &RetainedTrace) -> Option<Arc<[CoefficientLine]>> {
    let complex = waveform.complex.as_ref()?;
    let frequency = waveform.x.fetch_whole().ready()?;
    let real = complex.real.fetch_whole().ready()?;
    let imag = complex.imag.fetch_whole().ready()?;
    (frequency.len() == real.len() && real.len() == imag.len()).then(|| {
        frequency
            .iter()
            .zip(real.iter())
            .zip(imag.iter())
            .map(|((&frequency, &re), &im)| CoefficientLine { frequency, re, im })
            .collect()
    })
}

pub(super) fn spectrum_trace_is_renderable(waveform: &RetainedTrace) -> bool {
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

pub(super) fn active_analysis_is_renderable(state: ResultsReads<'_>) -> bool {
    state.model.simulation.active_run().is_some_and(|run| {
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

fn selected_hb_analysis(state: ResultsReads<'_>) -> Option<&AnalysisResult> {
    let run = state.model.simulation.active_run()?;
    state
        .model
        .simulation
        .view
        .active_analysis_idx
        .and_then(|index| run.analyses.get(index))
        .filter(|analysis| analysis_is_renderable(analysis))
        .or_else(|| {
            run.analyses
                .iter()
                .find(|analysis| analysis_is_renderable(analysis))
        })
}

fn build_model(state: ResultsReads<'_>, tokens: &Tokens) -> Option<HarmonicBalanceModel> {
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
        // The spectrum's lines are listed whole: a stored spectrum is read off the frame and
        // drawn once it has arrived.
        let (Some(frequencies), Some(magnitudes)) = (
            waveform.x.fetch_whole().ready(),
            waveform.y.fetch_whole().ready(),
        ) else {
            continue;
        };
        for (&frequency, &magnitude) in frequencies.iter().zip(magnitudes.iter()) {
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
            lines: coefficient_lines(waveform).unwrap_or_else(|| Arc::from([])),
            frequency: frequencies,
            magnitude: magnitudes,
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

fn active_incomplete_fft(state: ResultsReads<'_>) -> Option<String> {
    let analysis = state.model.simulation.active_analysis()?;
    analysis
        .success
        .then(|| recorded_fft::incomplete_history_sentence(&analysis.data))?
}

fn active_hb_failure(state: ResultsReads<'_>) -> Option<&str> {
    let analysis = state.model.simulation.active_analysis()?;
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

fn absence(state: ResultsReads<'_>) -> view::SpectrumAbsence<'_> {
    if let Some(error) = active_hb_failure(state) {
        view::SpectrumAbsence::Failed(error)
    } else if let Some(sentence) = active_incomplete_fft(state) {
        view::SpectrumAbsence::IncompleteHistory(sentence)
    } else {
        view::SpectrumAbsence::Unavailable
    }
}

/// Bind the retained spectrum and apply navigation to its host viewport.
pub fn show(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    let tokens = Tokens::get(ui.ctx());
    let quantity_policy = state.model.preferences.quantity_presentation_policy();
    let presentation = state.results_reads().presentation_policy().readout();
    let Some(model) = build_model(state.results_reads(), &tokens) else {
        view::show_absent(ui, absence(state.results_reads()));
        return;
    };
    let (_, settings) = kept_settings(state.results_reads());
    let sheet = super::ResultViewer::HarmonicBalance;
    let mut pin = super::readout_pins::pinned(state.results_reads(), sheet);
    let pinned = pin;
    let views = (0..view::strip_count(&model, &settings))
        .map(|index| {
            state
                .results
                .session
                .plot_view(super::ResultViewer::HarmonicBalance, index)
        })
        .collect::<Vec<_>>();
    let response = view::show(
        ui,
        &model,
        &settings,
        &views,
        &quantity_policy,
        presentation,
        &mut state.results.session.cache,
        &mut pin,
    );
    if pin != pinned {
        super::readout_pins::keep(state, sheet, pin);
    }
    if response.fit {
        for (index, _) in &response.plots {
            state
                .results
                .session
                .reset_plot_view(super::ResultViewer::HarmonicBalance, *index);
        }
    }
    for (index, plot) in &response.plots {
        if *index == 0 {
            super::record_drawn_axes(state.results, super::ResultViewer::HarmonicBalance, plot);
        }
        if plot.view.any() {
            state
                .results
                .session
                .plot_view_mut(super::ResultViewer::HarmonicBalance, *index)
                .apply(&plot.view);
        }
    }
}

/// Bind the inspector to the same qualified spectrum and recorded payload.
pub fn right_panel(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    let tokens = Tokens::get(ui.ctx());
    let model = build_model(state.results_reads(), &tokens);
    let source = model.as_ref().ok_or_else(|| absence(state.results_reads()));
    let quantity_policy = state.model.preferences.quantity_presentation_policy();
    let presentation = state.results_reads().presentation_policy();
    let digits = usize::from(presentation.displayed_significant_digits().get());
    let (_, settings) = kept_settings(state.results_reads());
    view::right_panel(ui, source, &settings, &quantity_policy, digits);
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    fn spectrum_waveform() -> RetainedTrace {
        RetainedTrace::new(
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
            .with_waveforms(vec![RetainedTrace::new(
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
        let malformed = RetainedTrace::new(
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

    /// The lines the sheet reads keep the phase the engine solved.
    #[test]
    fn the_sheet_reads_each_line_with_its_phase() {
        let lines = coefficient_lines(&spectrum_waveform().with_complex_components(
            "V(out)",
            Arc::new(vec![0.1, 0.0, 0.2]),
            Arc::new(vec![0.0, 1.0, 0.0]),
        ))
        .expect("complex lines");
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[1].frequency, 1.0e6);
        assert_eq!(lines[1].phase_degrees(), 90.0);
        assert_eq!(lines[1].magnitude(), 1.0);
    }

    /// The level the reader sets is kept by the run's working view, through
    /// its one writer, and Undo takes it back.
    #[test]
    fn the_level_the_reader_sets_is_kept_by_the_view_and_undone() {
        use crate::product::ObjectRevision;
        use crate::state::AnalysisResultProvenance;
        use crate::workbench::documents::result_document::test_fixtures::{
            completed_prepared_run, digest,
        };
        use crate::workbench::documents::result_document::test_host::ResultsTestHost;
        use crate::workbench::documents::result_document::view_edits::{self, Step};
        use rspice_results::harmonic_spectrum::settings::CoefficientScale;

        let plan = crate::product::SimulationPlanId::new();
        let analysis = crate::product::AnalysisInstanceId::new();
        let mut run = completed_prepared_run(plan, ObjectRevision::INITIAL, digest(0x60));
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::HarmonicBalance, "HB")
                .with_waveforms(vec![spectrum_waveform()])
                .with_provenance(
                    AnalysisResultProvenance::new(
                        analysis,
                        ObjectRevision::INITIAL,
                        digest(0x61),
                        Vec::new(),
                    )
                    .expect("analysis provenance"),
                ),
        );
        let mut state = ResultsTestHost::default();
        state.model.simulation.retained.runs = vec![run].into();
        assert!(state.model.simulation.select_run(0));
        state.quick_frame();
        let settings = CoefficientSettings {
            scale: CoefficientScale::Dbm,
            reference_ohms: 75.0,
            phase: true,
            ..CoefficientSettings::default()
        };
        keep_settings(&mut state.results_parts(), settings.clone());
        assert_eq!(kept_settings(state.results_reads()).1, settings);

        let document = state.working_view().expect("the run's view").id();
        view_edits::step(&mut state.results_parts(), document, Step::Undo).expect("an undo");
        assert_eq!(
            kept_settings(state.results_reads()).1,
            CoefficientSettings::default()
        );
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
