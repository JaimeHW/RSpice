//! The optimization form: the variables the search moves, the objective it
//! moves them towards, and when it stops.
//!
//! The target row belongs to the target goal alone, so it is greyed under the
//! two goals that have no target to state.
//!
//! The three step sizes sit under their own sub-header because they are one
//! subject — how far the search reaches, and how small a reach it gives up at —
//! and because they share a rule the labels cannot carry on their own: the
//! smallest step may not exceed the first one. The run refused an optimization
//! whose steps did not satisfy that long before the form had fields for them;
//! what was missing was any way to author the values the refusal was about.

use egui::Ui;

use crate::simulation::dialog::OptimizationDialogState;

use super::{
    choice_row, clear_pending_cell, engineering_input_row, hinted_input_row, input_row,
    input_row_enabled, sub_header,
};

/// Render the optimization fields.
pub(super) fn fields(ui: &mut Ui, setup: &mut OptimizationDialogState) {
    input_row(ui, "Variables", &mut setup.variables_text)
        .on_hover_text("One per line or comma, spelled name:min:max[:initial].");
    input_row(ui, "Objective", &mut setup.objective_node);
    input_row(ui, "Obj ref", &mut setup.objective_ref);
    choice_row(ui, "Goal", &["min", "max", "target"], &mut setup.goal_mode);
    input_row_enabled(ui, "Target", &mut setup.target_value, setup.goal_mode == 2);
    choice_row(
        ui,
        "Method",
        &["gradient", "pattern", "anneal"],
        &mut setup.algorithm,
    );
    input_row(ui, "Max iters", &mut setup.max_iterations);
    input_row(ui, "Tolerance", &mut setup.cost_tolerance);

    sub_header(ui, "Step sizes");
    engineering_input_row(ui, "Gradient probe", &mut setup.fd_step);
    engineering_input_row(ui, "First step", &mut setup.initial_step);
    hinted_input_row(
        ui,
        "Smallest step",
        &mut setup.min_step,
        "at most the first step",
    );
    // Three rows leave a half-filled grid row behind, and the next form's
    // first field would otherwise be dealt into it.
    clear_pending_cell(ui);
}
