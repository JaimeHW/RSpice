//! Application coordination for FFT presentation and source authorization.

#[cfg(test)]
mod tests;

use super::{AnalysisPresentationKey, ResultViewer, ResultsReads, frame_work, view_edits};
use crate::results::visualization_document::DocumentEdit;
use crate::workbench::documents::result_document::ResultsParts;
use egui::Ui;
use rspice_results::fft::settings::SpectrumSettings;
use rspice_results_ui::fft::view;
use rspice_results_ui::session::PersistentPaneContext;

/// The pane of the result view that keeps the shown analysis' sheets: the
/// pane of a saved document being drawn, or the run's working view's.
pub(super) fn sheet_pane(state: ResultsReads<'_>) -> Option<PersistentPaneContext> {
    let run = state.model.simulation.active_run()?;
    let analysis = state.model.simulation.active_analysis()?;
    state
        .results
        .session
        .pane_for(AnalysisPresentationKey::new(run.dataset_id, analysis))
}

/// The settings the view keeps for the sheet: the reader's, or the sheet's
/// defaults until they change one. `None` when no view keeps the sheet.
fn kept_settings(state: ResultsReads<'_>) -> Option<(PersistentPaneContext, SpectrumSettings)> {
    let pane = sheet_pane(state)?;
    let document = state
        .model
        .workspace
        .content
        .visualization_document(pane.document_id)?;
    let settings = document
        .spectrum_settings(pane.pane_id)
        .cloned()
        .unwrap_or_default();
    Some((pane, settings))
}

/// Take in the settings the view keeps for the sheet, so a re-run, an undo or
/// another view's pane shows the spectrum as that view was set.
pub(crate) fn adopt_kept_settings(state: &mut ResultsParts<'_>) {
    // A source chosen from a trace's menu is the reader's latest word on
    // the sheet: the view keeps it rather than putting back its own.
    if let Some(source) = state.analysis.fft_state.take_unkept_choice() {
        let held = kept_settings(state.results_reads())
            .map(|(_, settings)| settings)
            .unwrap_or_else(|| state.analysis.fft_state.settings());
        keep_settings(
            state,
            SpectrumSettings {
                source: Some(source),
                ..held
            },
        );
        return;
    }
    if let Some((_, settings)) = kept_settings(state.results_reads()) {
        state.analysis.fft_state.adopt_settings(&settings);
    }
}

/// Keep `settings` the reader set on the sheet: in the view that keeps the
/// sheet, through its one writer, as a step the reader can undo. A setting
/// that fails to rebuild the spectrum revokes the viewer's source
/// authorization, as it always has.
pub(crate) fn keep_settings(state: &mut ResultsParts<'_>, settings: SpectrumSettings) {
    state.analysis.fft_state.adopt_settings(&settings);
    if state.analysis.fft_state.last_error.is_some() {
        state.analysis.clear_fft_viewer_cache_authority();
    }
    let Some((pane, held)) = kept_settings(state.results_reads()) else {
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
    let edit = DocumentEdit::SetSpectrumSettings {
        pane_id: pane.pane_id,
        settings: (settings != SpectrumSettings::default()).then_some(settings),
    };
    if let Err(error) = view_edits::apply(state, pane.document_id, revision, vec![edit]) {
        state.prompt(format!("Could not keep this change to the view: {error}"));
    }
}

/// Keep the sheet's source on what the reader plots: until the reader chooses
/// a source, the spectrum is of the first voltage plotted, and plotting
/// another first drops the spectrum so the sheet prepares that one.
pub(crate) fn follow_plotted(state: &mut ResultsParts<'_>) {
    let source = state.results_reads().first_plotted_voltage();
    state.analysis.fft_state.set_default_source(source);
}

pub fn show(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    adopt_kept_settings(state);
    follow_plotted(state);
    let quantity_policy = state.model.preferences.quantity_presentation_policy();
    let presentation = state.results_reads().presentation_policy().readout();
    let plot_view = state.results.session.plot_view(ResultViewer::Fft, 0);
    let mut pin = super::readout_pins::pinned(state.results_reads(), ResultViewer::Fft);
    let pinned = pin;
    let kind_tag = super::active_kind_tag(state.results_reads());
    let out = view::show(
        ui,
        &mut state.analysis.fft_state,
        &mut state.results.session.fft_series,
        plot_view,
        &mut state.results.session.cache,
        &quantity_policy,
        presentation,
        &mut pin,
        &kind_tag,
    );
    if pin != pinned {
        super::readout_pins::keep(state, ResultViewer::Fft, pin);
    }
    if out.fit_requested {
        state.results.session.reset_plot_view(ResultViewer::Fft, 0);
    }
    frame_work::note_samples(
        frame_work::FrameSampleRead::TraceExtremes,
        out.range_samples_read,
    );
    if let Some(response) = out.plot {
        super::record_drawn_axes(state.results, ResultViewer::Fft, &response);
        if response.view.any() {
            state
                .results
                .session
                .plot_view_mut(ResultViewer::Fft, 0)
                .apply(&response.view);
        }
    }
}

/// The transform settings in the sheet bar. A setting that fails to rebuild
/// the spectrum revokes the viewer's source authorization, as it always has.
pub(super) fn domain_bar(ui: &mut Ui, state: &mut ResultsParts<'_>) -> bool {
    adopt_kept_settings(state);
    // The signals the shown analysis saved, in the order the run wrote them.
    let sources: Vec<String> = state
        .model
        .simulation
        .active_analysis()
        .map(|analysis| {
            analysis
                .waveforms
                .iter()
                .map(|waveform| waveform.name.clone())
                .collect()
        })
        .unwrap_or_default();
    let cursors = view::RecordCursors {
        a: state.results.session.cursors.a,
        b: state.results.session.cursors.b,
    };
    if let Some(settings) = view::domain_bar(ui, &state.analysis.fft_state, &sources, cursors) {
        keep_settings(state, settings);
    }
    true
}

/// The fundamental and its THD as the card shows them, for a printed
/// spectrum; none before the spectrum has been analysed.
pub(super) fn printed_figures(state: ResultsReads<'_>) -> rspice_results_ui::table::SheetPrint {
    use rspice_results_ui::table::SheetPrint;
    let Some(analysis) = state.analysis.fft_state.analysis.as_ref() else {
        return SheetPrint::default();
    };
    SheetPrint::figures(
        state
            .model
            .simulation
            .active_analysis()
            .map(|analysis| analysis.id),
        view::card_figures(
            analysis,
            &state.model.preferences.quantity_presentation_policy(),
            usize::from(
                state
                    .presentation_policy()
                    .displayed_significant_digits()
                    .get(),
            ),
        ),
    )
}

pub fn right_panel(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    let presentation = state.results_reads().presentation_policy();
    view::right_panel(
        ui,
        &state.analysis.fft_state,
        &mut state.results.session.fft_series,
        &state.model.preferences.quantity_presentation_policy(),
        usize::from(presentation.displayed_significant_digits().get()),
    );
}
