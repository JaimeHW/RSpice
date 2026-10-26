//! Application coordination for FFT presentation and source authorization.

use super::{ResultViewer, frame_work};
use crate::workbench::AppState;
use egui::Ui;
use rspice_results_ui::fft::view;

pub fn show(ui: &mut Ui, state: &mut AppState) {
    let quantity_policy = state.model.preferences.quantity_presentation_policy();
    let presentation = state.result_presentation_policy().readout();
    let plot_view = state
        .results
        .results
        .session
        .plot_view(ResultViewer::Fft, 0);
    let out = view::show(
        ui,
        &mut state.results.analysis.fft_state,
        &mut state.results.results.session.fft_series,
        plot_view,
        &mut state.results.results.session.cache,
        &quantity_policy,
        presentation,
    );
    if out.fit_requested {
        state
            .results
            .results
            .session
            .reset_plot_view(ResultViewer::Fft, 0);
    }
    if out.rebuild_failed {
        state.results.analysis.clear_fft_viewer_cache_authority();
    }
    frame_work::note_samples(
        frame_work::FrameSampleRead::TraceExtremes,
        out.range_samples_read,
    );
    if let Some(response) = out.plot {
        super::record_drawn_axes(&mut state.results.results, ResultViewer::Fft, &response);
        if response.view.any() {
            state
                .results
                .results
                .session
                .plot_view_mut(ResultViewer::Fft, 0)
                .apply(&response.view);
        }
    }
}

pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let presentation = state.result_presentation_policy();
    view::right_panel(
        ui,
        &state.results.analysis.fft_state,
        &mut state.results.results.session.fft_series,
        &state.model.preferences.quantity_presentation_policy(),
        usize::from(presentation.displayed_significant_digits().get()),
    );
}
