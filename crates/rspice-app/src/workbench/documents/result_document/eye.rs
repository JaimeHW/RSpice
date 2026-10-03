//! Application coordination for eye presentation and source-bound timebase requests.

use super::{
    ResultViewer,
    frame_work::{self, DatasetWalk},
};
use crate::workbench::AppState;
use egui::Ui;
use rspice_results_ui::eye_diagram::view;

/// Apply a timebase request to the immutable source selected by the document.
pub fn inline_actions(ui: &mut Ui, state: &mut AppState) {
    let owner = state.active_specialized_viewer_cache_provenance();
    let timebase = owner.map(|owner| state.eye_timebase_for(owner));
    let requested = view::inline_actions(ui, &mut state.analysis.eye_diagram_state, timebase);
    if let Some(owner) = owner
        && let Some(timebase) = requested
    {
        state.set_eye_timebase(owner, timebase);
    }
}

fn selected_source(state: &AppState) -> &str {
    state
        .analysis
        .fft_state
        .selected_source
        .as_deref()
        .unwrap_or("the active trace")
}

pub fn unavailable_hint(state: &AppState) -> Option<String> {
    view::unavailable_hint(&state.analysis.eye_diagram_state, selected_source(state))
}

pub fn show(ui: &mut Ui, state: &mut AppState) {
    let source = state
        .analysis
        .fft_state
        .selected_source
        .as_deref()
        .unwrap_or("the active trace");
    let plot_view = state.ui.results.session.plot_view(ResultViewer::Eye, 0);
    let out = view::show(
        ui,
        &state.analysis.eye_diagram_state,
        source,
        plot_view,
        &mut state.ui.results.session.eye_texture,
        &mut state.ui.results.session.cache,
    );
    if out.density_baked {
        frame_work::note(DatasetWalk::EyeRaster);
    }
    if let Some(response) = out.plot {
        super::record_drawn_axes(&mut state.ui.results, ResultViewer::Eye, &response);
        if response.view.any() {
            state
                .ui
                .results
                .session
                .plot_view_mut(ResultViewer::Eye, 0)
                .apply(&response.view);
        }
    }
}

pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    view::right_panel(
        ui,
        &state.analysis.eye_diagram_state,
        selected_source(state),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_results_ui::eye_diagram::{EyeData, EyeTrace};

    /// Panning and zooming must not rebake. The bake walks every acquisition,
    /// and a drag is exactly when the reader can least afford it.
    #[test]
    fn moving_the_view_does_not_rebake_the_density() {
        let mut state = AppState::default();
        let mut data = EyeData::new(1.0e-9, 2);
        for trace in 0..8 {
            data.add_trace(EyeTrace::new(
                vec![0.0, 0.5, 1.0, 1.5],
                vec![0.0, 1.0, 0.0, f64::from(trace) * 0.1],
            ));
        }
        state.analysis.eye_diagram_state.load_data(data);

        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let frame = |state: &mut AppState| {
            let _ = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1_200.0, 800.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| show(ui, state));
                },
            );
        };

        frame(&mut state);
        let baseline = super::super::frame_work::WorkCounts::reset();

        // The reader zooms into the middle of the eye, then pans.
        for window in [(0.4, 1.2, 0.1, 0.8), (0.6, 1.4, 0.2, 0.9)] {
            let view = state
                .ui
                .results
                .session
                .plot_view_mut(super::super::ResultViewer::Eye, 0);
            view.x = Some((window.0, window.1));
            view.y = Some((window.2, window.3));
            frame(&mut state);
        }

        assert_eq!(
            baseline
                .since()
                .get(super::super::frame_work::DatasetWalk::EyeRaster),
            0,
            "the eye rebaked its density texture while the reader moved the view"
        );
    }
}
