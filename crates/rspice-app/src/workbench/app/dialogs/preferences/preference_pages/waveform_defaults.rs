//! Results and precision ▸ Waveform defaults: how waveform panes draw a
//! run's traces, and what a waveform view starts with.

use egui::Ui;

use crate::workbench::{ChoicePreference, TogglePreference};

use super::super::preferences_shell::{searchable_row, section_heading, segmented, value_column};
use super::{AppState, toggle_row};

/// Thin, Normal, Thick in that order, as the stored index each keeps. Normal
/// is stored as zero because zero is the shipped value.
const TRACE_WIDTHS: [(&str, usize); 3] = [("Thin", 1), ("Normal", 0), ("Thick", 2)];

pub(super) fn section(ui: &mut Ui, state: &mut AppState) {
    section_heading(ui, "Waveform defaults");
    segment_row(
        ui,
        state,
        ChoicePreference::WaveTraceWidth,
        "Trace width",
        "Stroke of each run's traces in waveform panes.",
        &TRACE_WIDTHS,
    );
    segment_row(
        ui,
        state,
        ChoicePreference::AcMagnitude,
        "AC magnitude",
        "How AC magnitude traces are drawn in waveform panes.",
        &[("dB", 0), ("Linear", 1)],
    );
    segment_row(
        ui,
        state,
        ChoicePreference::AcFrequencyAxis,
        "AC frequency axis",
        "Scale of the frequency axis on AC and noise waveform panes.",
        &[("Log", 0), ("Linear", 1)],
    );
    if segment_row(
        ui,
        state,
        ChoicePreference::AcPhase,
        "Phase",
        "How phase traces start. Result properties can still switch it.",
        &[("Wrapped", 0), ("Unwrapped", 1)],
    ) {
        apply_to_viewer(state);
    }
    let grid = state.ui.preferences.toggle(TogglePreference::WaveMinorGrid);
    toggle_row(
        ui,
        state,
        TogglePreference::WaveMinorGrid,
        "Minor grid",
        "Whether a waveform view starts with minor grid lines.",
        "Minor grid in new waveform views",
    );
    if grid != state.ui.preferences.toggle(TogglePreference::WaveMinorGrid) {
        apply_to_viewer(state);
    }
}

/// Start the open viewer from these defaults, as a new one would start.
pub(super) fn apply_to_viewer(state: &mut AppState) {
    state.ui.results.session.phase_continuous = state.ui.preferences.unwrapped_phase();
    state.start_waveform_view_from_preferences();
}

/// A segmented row over `segments`, each a label and the stored index it
/// keeps. Returns whether the reader changed it.
fn segment_row(
    ui: &mut Ui,
    state: &mut AppState,
    key: ChoicePreference,
    title: &str,
    help: &str,
    segments: &[(&str, usize)],
) -> bool {
    let labels = segments.iter().map(|(label, _)| *label).collect::<Vec<_>>();
    let mut changed = false;
    searchable_row(ui, title, help, &labels, |ui| {
        value_column(ui, |ui| {
            let stored = state.ui.preferences.choice(key);
            let mut selected = segments
                .iter()
                .position(|(_, index)| *index == stored)
                .unwrap_or_default();
            if segmented(ui, key.stable_id(), &labels, &mut selected) {
                state
                    .ui
                    .preferences
                    .set_choice(key, segments[selected].1)
                    .expect("the segments are the complete preference domain");
                changed = true;
            }
        });
    });
    changed
}
