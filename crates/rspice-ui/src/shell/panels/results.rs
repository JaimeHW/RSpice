//! Results-context side panels: signal browser over the whole run (left),
//! and the active viewer's instrument readout (right — delegated to the
//! results workspace).

use egui::Ui;

use crate::common::AppState;
use crate::shell::results;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{TreeRow, section_header};

// ---------------------------------------------------------------------------
// Left panel — signals per analysis
// ---------------------------------------------------------------------------

/// Render the results context's left panel.
pub fn left(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let run_label = state
        .simulation
        .active_run()
        .map_or_else(|| "Signals".to_owned(), |run| format!("Signals — run #{}", run.id));
    section_header(ui, &run_label, None);

    let Some(run_idx) = state.simulation.active_run_idx else {
        ui.add_space(4.0);
        ui.vertical_centered(|ui| {
            ui.label(
                egui::RichText::new("Run a simulation to browse signals")
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(c.text_faint),
            );
        });
        return;
    };

    // Snapshot the tree (group label/meta + signal rows) so toggles can
    // mutate the run while we render.
    struct SignalRow {
        waveform_index: usize,
        name: String,
        color: egui::Color32,
        visible: bool,
    }
    struct AnalysisGroup {
        analysis_index: usize,
        label: String,
        meta: String,
        active: bool,
        signals: Vec<SignalRow>,
    }
    let Some(run) = state.simulation.runs.get(run_idx) else {
        return;
    };
    let active_analysis = state.simulation.active_analysis_idx;
    let groups: Vec<AnalysisGroup> = run
        .analyses
        .iter()
        .enumerate()
        .map(|(analysis_index, analysis)| {
            let points = analysis
                .waveforms
                .first()
                .map_or(0, |waveform| waveform.x.len());
            AnalysisGroup {
                analysis_index,
                label: analysis.analysis_type.short_label().to_lowercase(),
                meta: format!("{points} pts"),
                active: active_analysis == Some(analysis_index),
                signals: analysis
                    .waveforms
                    .iter()
                    .enumerate()
                    .map(|(waveform_index, waveform)| SignalRow {
                        waveform_index,
                        name: waveform.name.clone(),
                        color: results::waveform_color(waveform, waveform_index, &t),
                        visible: waveform.visible,
                    })
                    .collect(),
            }
        })
        .collect();

    let mut select_analysis: Option<usize> = None;
    let mut toggle: Option<(usize, usize)> = None;

    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        for group in &groups {
            let row = TreeRow::new(&group.label)
                .twist(true)
                .meta(&group.meta)
                .mono()
                .selected(group.active)
                .show(ui);
            if row.response.clicked() {
                select_analysis = Some(group.analysis_index);
            }
            for signal in &group.signals {
                let mut visible = signal.visible;
                let row = TreeRow::new(&signal.name)
                    .checkbox(&mut visible)
                    .chip_dot(signal.color)
                    .indent(1)
                    .mono()
                    .show(ui);
                if row.checkbox_changed {
                    toggle = Some((group.analysis_index, signal.waveform_index));
                }
            }
        }
    });

    if let Some(idx) = select_analysis {
        state.simulation.select_analysis(idx);
    }
    if let Some((analysis_index, waveform_index)) = toggle {
        results::toggle_visibility(state, analysis_index, waveform_index);
    }

    // Derived signals come from the waveform calculator.
    if let Some(action) = section_header(ui, "Expressions", Some("+ Add")) {
        if action.clicked() {
            state.dialogs.waveform_calculator_dialog = true;
        }
    }
}

// ---------------------------------------------------------------------------
// Right panel — per-viewer instrument readout
// ---------------------------------------------------------------------------

/// Render the results context's right panel.
pub fn right(ui: &mut Ui, state: &mut AppState) {
    results::right_panel(ui, state);
}
