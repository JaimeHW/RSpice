//! Application coordination for FFT presentation and source authorization.

use super::{ResultViewer, frame_work};
use crate::workbench::AppState;
use egui::Ui;
use rspice_results_ui::fft::view;

pub fn show(ui: &mut Ui, state: &mut AppState) {
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let plot_view = state.ui.results.plot_view(ResultViewer::Fft, 0);
    let out = view::show(
        ui,
        &mut state.analysis.fft_state,
        &mut state.ui.results.fft_series,
        plot_view,
        &mut state.ui.results.cache,
        &quantity_policy,
    );
    if out.fit_requested {
        state.ui.results.reset_plot_view(ResultViewer::Fft, 0);
    }
    if out.rebuild_failed {
        state.clear_fft_viewer_cache_authority();
    }
    frame_work::note_samples(
        frame_work::FrameSampleRead::TraceExtremes,
        out.range_samples_read,
    );
    if let Some(response) = out.plot {
        super::record_drawn_axes(&mut state.ui.results, ResultViewer::Fft, &response);
        if response.view.any() {
            state
                .ui
                .results
                .plot_view_mut(ResultViewer::Fft, 0)
                .apply(&response.view);
        }
    }
}

pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    view::right_panel(
        ui,
        &state.analysis.fft_state,
        &mut state.ui.results.fft_series,
        &state.ui.preferences.quantity_presentation_policy(),
    );
}
