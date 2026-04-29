//! Eye Diagram Rendering
//!
//! Commercial-grade egui rendering for eye diagram visualization.
//! Supports overlay, persistence, and single-trace display modes.

mod axes;
mod chart;
mod cursors;
mod header;
mod layout;
mod mask;
mod measurements_panel;
mod style;
mod traces;

use egui::Ui;

use super::state::EyeDiagramState;
use crate::common::app::AppState;

use self::chart::{handle_measurements_splitter, render_chart_area, render_chart_core};
use self::header::render_header;
use self::layout::{
    calculate_layout_with_measurements_width, preferred_measurements_pane_width,
    resolve_measurements_pane_width,
};
use self::measurements_panel::render_measurements_panel;

/// Render the eye diagram viewer panel
pub fn render_eye_diagram_viewer(ui: &mut Ui, app_state: &mut AppState) {
    let available_rect = ui.available_rect_before_wrap();
    // Claim full available space so parent resizable panels keep user-set size
    // instead of snapping back to a content-driven natural size.
    let (_id, _rect) = ui.allocate_space(available_rect.size());
    let auto_width = preferred_measurements_pane_width(ui, &app_state.analysis.eye_diagram_state);
    app_state
        .analysis
        .eye_diagram_state
        .measurements_pane_auto_width_hint = auto_width;
    let measurements_width = resolve_measurements_pane_width(
        available_rect,
        app_state.analysis.eye_diagram_state.measurements_pane_width,
        auto_width,
    );
    if app_state
        .analysis
        .eye_diagram_state
        .measurements_pane_width
        .is_some()
    {
        app_state.analysis.eye_diagram_state.measurements_pane_width = Some(measurements_width);
    }

    let layout = calculate_layout_with_measurements_width(
        available_rect,
        app_state.analysis.eye_diagram_state.show_measurements,
        measurements_width,
    );

    let close_requested = {
        let state = &mut app_state.analysis.eye_diagram_state;
        let close_requested = render_header(ui, &layout, state);
        handle_measurements_splitter(ui, &layout, state);
        render_chart_area(ui, &layout, state);
        render_measurements_panel(ui, &layout, state);
        close_requested
    };
    if close_requested {
        app_state.close_active_viewer();
    }
}

/// Public render function for external use
pub fn render_eye_diagram(ui: &mut Ui, state: &EyeDiagramState) {
    let available_rect = ui.available_rect_before_wrap();
    let (_id, _rect) = ui.allocate_space(available_rect.size());
    let auto_width = preferred_measurements_pane_width(ui, state);
    let measurements_width =
        resolve_measurements_pane_width(available_rect, state.measurements_pane_width, auto_width);
    let layout = calculate_layout_with_measurements_width(
        available_rect,
        state.show_measurements,
        measurements_width,
    );

    let mut state_copy = state.clone();
    render_chart_core(ui, &layout, &mut state_copy);
}
