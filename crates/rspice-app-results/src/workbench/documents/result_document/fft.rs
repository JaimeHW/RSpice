//! Application coordination for FFT presentation and source authorization.

#[cfg(test)]
mod tests;

use super::{ResultViewer, frame_work};
use crate::workbench::documents::result_document::ResultsParts;
use egui::Ui;
use rspice_results_ui::fft::view;

/// Keep the sheet's source on what the reader plots: until the reader chooses
/// a source, the spectrum is of the first voltage plotted, and plotting
/// another first drops the spectrum so the sheet prepares that one.
pub(crate) fn follow_plotted(state: &mut ResultsParts<'_>) {
    let source = state.results_reads().first_plotted_voltage();
    state.analysis.fft_state.set_default_source(source);
}

pub fn show(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    follow_plotted(state);
    let quantity_policy = state.model.preferences.quantity_presentation_policy();
    let presentation = state.results_reads().presentation_policy().readout();
    let plot_view = state.results.session.plot_view(ResultViewer::Fft, 0);
    let mut pin = super::readout_pins::pinned(state.results_reads(), ResultViewer::Fft);
    let pinned = pin;
    let out = view::show(
        ui,
        &mut state.analysis.fft_state,
        &mut state.results.session.fft_series,
        plot_view,
        &mut state.results.session.cache,
        &quantity_policy,
        presentation,
        &mut pin,
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
    if view::domain_bar(ui, &mut state.analysis.fft_state) {
        state.analysis.clear_fft_viewer_cache_authority();
    }
    true
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
