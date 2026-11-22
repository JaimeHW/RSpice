//! Application coordination for the eye: its settings, kept per result, and
//! the sheet's bar, figures and picture.

use super::{
    ResultViewer,
    frame_work::{self, DatasetWalk},
};
use crate::workbench::documents::result_document::{ResultsParts, ResultsReads};
use egui::Ui;
use rspice_results_ui::eye_diagram::{bar, view};

/// Take in the settings kept for the result shown, so its levels, error rate
/// and mask are the ones the reader left.
fn adopt_settings(state: &mut ResultsParts<'_>) {
    let Some(owner) = state.model.active_specialized_viewer_cache_provenance() else {
        return;
    };
    let settings = state.results.eye_settings_for(owner);
    state.analysis.eye_diagram_state.adopt_settings(&settings);
}

/// The sheet's controls, left in its bar: the signal, the clock, the levels,
/// the error rate and the mask. What the reader sets is kept for the result.
pub(super) fn domain_bar(ui: &mut Ui, state: &mut ResultsParts<'_>) -> bool {
    adopt_settings(state);
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
    let significant_digits = significant_digits(state.results_reads());
    let changed = bar::domain_bar(
        ui,
        &state.analysis.eye_diagram_state,
        &sources,
        significant_digits,
    );
    if let Some(settings) = changed
        && let Some(owner) = state.model.active_specialized_viewer_cache_provenance()
    {
        state.analysis.eye_diagram_state.adopt_settings(&settings);
        state.results.set_eye_settings(owner, settings);
    }
    true
}

/// What the sheet reads out right in its bar: the mask's verdict.
pub fn inline_actions(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    adopt_settings(state);
    let significant_digits = significant_digits(state.results_reads());
    bar::outputs(ui, &state.analysis.eye_diagram_state, significant_digits);
}

/// The reader's significant digits, which the eye's figures read at.
fn significant_digits(state: ResultsReads<'_>) -> usize {
    usize::from(
        state
            .presentation_policy()
            .displayed_significant_digits()
            .get(),
    )
}

/// The signal the eye folds, as the sheet names it.
fn selected_source(state: ResultsReads<'_>) -> String {
    let named = state
        .model
        .active_specialized_viewer_cache_provenance()
        .and_then(|owner| state.results.eye_fold_for(owner).source);
    named
        .or_else(|| state.analysis.fft_state.selected_source.clone())
        .unwrap_or_else(|| "the active trace".to_owned())
}

pub fn unavailable_hint(state: ResultsReads<'_>) -> Option<String> {
    view::unavailable_hint(&state.analysis.eye_diagram_state, &selected_source(state))
}

pub fn show(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    adopt_settings(state);
    let source = selected_source(state.results_reads());
    let plot_view = state.results.session.plot_view(ResultViewer::Eye, 0);
    let significant_digits = significant_digits(state.results_reads());
    let out = view::show(
        ui,
        &state.analysis.eye_diagram_state,
        &source,
        plot_view,
        &mut state.results.session.eye_texture,
        &mut state.results.session.cache,
        significant_digits,
    );
    if out.density_baked {
        frame_work::note(DatasetWalk::EyeRaster);
    }
    if let Some(response) = out.plot {
        super::record_drawn_axes(state.results, ResultViewer::Eye, &response);
        if response.view.any() {
            state
                .results
                .session
                .plot_view_mut(ResultViewer::Eye, 0)
                .apply(&response.view);
        }
    }
}

pub fn right_panel(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    adopt_settings(state);
    view::right_panel(
        ui,
        &state.analysis.eye_diagram_state,
        &selected_source(state.results_reads()),
        significant_digits(state.results_reads()),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::documents::result_document::test_host::ResultsTestHost;
    use rspice_results_ui::eye_diagram::{EyeData, EyeTrace};

    /// Panning and zooming must not rebake. The bake walks every acquisition,
    /// and a drag is exactly when the reader can least afford it.
    #[test]
    fn moving_the_view_does_not_rebake_the_density() {
        let mut state = ResultsTestHost::default();
        let mut data = EyeData::new(1.0e-9, 2);
        for trace in 0..8 {
            data.add_trace(EyeTrace::new(
                vec![0.0, 0.5, 1.0, 1.5],
                vec![0.0, 1.0, 0.0, f64::from(trace) * 0.1],
            ));
        }
        state.results.analysis.eye_diagram_state.load_data(data);

        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let frame = |state: &mut ResultsTestHost| {
            let _ = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1_200.0, 800.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default()
                        .show(ctx, |ui| show(ui, &mut state.results_parts()));
                },
            );
        };

        frame(&mut state);
        let baseline = super::super::frame_work::WorkCounts::reset();

        // The reader zooms into the middle of the eye, then pans.
        for window in [(0.4, 1.2, 0.1, 0.8), (0.6, 1.4, 0.2, 0.9)] {
            let view = state
                .results
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
