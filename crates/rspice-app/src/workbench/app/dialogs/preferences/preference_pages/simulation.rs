//! The Simulation page: what a new plan starts from, how much of the machine
//! one run may use, how many runs a plan keeps, and how an imported deck is
//! read.

use egui::Ui;

use crate::ui::widgets::select;
use crate::workbench::preferences::IMPORTABLE_NETLIST_DIALECTS;
use crate::workbench::{ChoicePreference, ScalarPreference};

use super::super::preferences_shell::{
    control_width, searchable_row, section_heading, value_column,
};
use super::{AppState, choice_row, solver_threads_row};

/// The retention counts offered, smallest first. The shipped one is 20.
const RUNS_KEPT: [u32; 5] = [5, 10, 20, 50, 100];

pub(super) fn page(ui: &mut Ui, state: &mut AppState) {
    section_heading(ui, "Solver");
    choice_row(
        ui,
        state,
        ChoicePreference::DefaultSolverPreset,
        "Default solver preset",
        "Every new simulation plan starts from this preset.",
        &["Balanced", "Fast", "Accurate", "Robust"],
    );
    solver_threads_row(ui, state);

    section_heading(ui, "Runs and decks");
    runs_kept_row(ui, state);
    imported_dialect_row(ui, state);
}

/// How many runs a new plan keeps before it discards the oldest.
fn runs_kept_row(ui: &mut Ui, state: &mut AppState) {
    let labels = RUNS_KEPT.map(|count| format!("{count} runs")).to_vec();
    searchable_row(
        ui,
        "Runs kept",
        "Runs a new plan keeps before it drops the oldest. Baselines are always kept.",
        &["history", "retention", "datasets"],
        |ui| {
            value_column(ui, |ui| {
                // Shown as the count it is, so a stored count between two
                // of the offered ones is not misreported as its neighbour.
                let current = format!(
                    "{} runs",
                    state.ui.preferences.scalar(ScalarPreference::RunsKept)
                );
                if let Some(selected) = select(
                    ui,
                    "preferences.simulation.runs-kept",
                    "Runs kept",
                    &current,
                    &labels,
                    control_width(ui),
                ) {
                    state
                        .ui
                        .preferences
                        .set_scalar(ScalarPreference::RunsKept, RUNS_KEPT[selected])
                        .expect("every offered count is inside the preference domain");
                }
            });
        },
    );
}

/// The dialect the import review starts from when a deck names none.
fn imported_dialect_row(ui: &mut Ui, state: &mut AppState) {
    let labels = IMPORTABLE_NETLIST_DIALECTS
        .map(|dialect| dialect.label().to_owned())
        .to_vec();
    let mut terms = labels.iter().map(String::as_str).collect::<Vec<_>>();
    terms.extend(["ngspice", "hspice", "pspice", "spectre", "ads", "import"]);
    searchable_row(
        ui,
        "Netlist dialect for imported decks",
        "Used for a deck with no dialect markers. The import review can change it.",
        &terms,
        |ui| {
            value_column(ui, |ui| {
                let active = state
                    .ui
                    .preferences
                    .choice(ChoicePreference::ImportedNetlistDialect);
                if let Some(selected) = select(
                    ui,
                    "preferences.simulation.imported-netlist-dialect",
                    "Netlist dialect for imported decks",
                    &labels[active],
                    &labels,
                    control_width(ui),
                ) {
                    state
                        .ui
                        .preferences
                        .set_choice(ChoicePreference::ImportedNetlistDialect, selected)
                        .expect("the offered dialects are the complete preference domain");
                }
            });
        },
    );
}
