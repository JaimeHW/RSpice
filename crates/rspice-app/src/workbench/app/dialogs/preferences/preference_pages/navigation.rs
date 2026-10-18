//! The Navigation page: how the schematic and symbol canvases answer the
//! mouse wheel and the pan button.
//!
//! The layout editor and the waveform panes keep their own navigation, which
//! is why the page names the two canvases it reaches.

use egui::Ui;

use crate::workbench::{ChoicePreference, TogglePreference};

use super::super::preferences_shell::{
    preference_row, searchable_row, section_heading, segmented, value_column,
};
use super::{AppState, keyword_choice_row, toggle_row};

/// The zoom speeds in the order the row shows them, as the stored index each
/// one keeps. Normal is stored as zero because zero is the shipped value.
const ZOOM_SPEEDS: [(&str, usize); 3] = [("Slow", 1), ("Normal", 0), ("Fast", 2)];

pub(super) fn page(ui: &mut Ui, state: &mut AppState) {
    section_heading(ui, "Schematic and symbol canvases");
    keyword_choice_row(
        ui,
        state,
        ChoicePreference::CanvasWheel,
        "Mouse wheel",
        "What the wheel does over the canvas. Shift+wheel always scrolls sideways.",
        &["Zoom", "Scroll (Ctrl+wheel zooms)"],
        &["trackpad", "mouse", "scroll"],
    );
    toggle_row(
        ui,
        state,
        TogglePreference::InvertWheelZoom,
        "Invert wheel zoom",
        "Wheel up zooms out instead of in.",
        "Invert wheel zoom",
    );
    zoom_speed_row(ui, state);
    searchable_row(
        ui,
        "Pan with",
        "A right click that does not move is still a right click.",
        &["Middle button", "Right button", "drag", "mouse"],
        |ui| {
            value_column(ui, |ui| {
                let mut selected = state
                    .ui
                    .preferences
                    .choice(ChoicePreference::CanvasPanButton);
                if segmented(
                    ui,
                    "preferences.navigation.pan-button",
                    &["Middle button", "Right button"],
                    &mut selected,
                ) {
                    state
                        .ui
                        .preferences
                        .set_choice(ChoicePreference::CanvasPanButton, selected)
                        .expect("the pan button segments are the complete domain");
                }
            });
        },
    );
    toggle_row(
        ui,
        state,
        TogglePreference::CanvasEdgePan,
        "Pan at canvas edges while dragging",
        "The view follows a part, wire or paste carried to the edge of the canvas.",
        "Pan at canvas edges while dragging",
    );
}

/// Slow, Normal, Fast in that order, over a stored index whose zero is
/// Normal.
fn zoom_speed_row(ui: &mut Ui, state: &mut AppState) {
    preference_row(ui, "Zoom speed", "How far one wheel notch zooms.", |ui| {
        value_column(ui, |ui| {
            let stored = state
                .ui
                .preferences
                .choice(ChoicePreference::CanvasZoomSpeed);
            let mut selected = ZOOM_SPEEDS
                .iter()
                .position(|(_, index)| *index == stored)
                .unwrap_or(1);
            if segmented(
                ui,
                "preferences.navigation.zoom-speed",
                &ZOOM_SPEEDS.map(|(label, _)| label),
                &mut selected,
            ) {
                state
                    .ui
                    .preferences
                    .set_choice(ChoicePreference::CanvasZoomSpeed, ZOOM_SPEEDS[selected].1)
                    .expect("the zoom speeds are the complete domain");
            }
        });
    });
}
