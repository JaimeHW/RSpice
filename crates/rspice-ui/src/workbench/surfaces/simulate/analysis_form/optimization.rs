//! The optimization form: the variables the search moves, the objective it
//! moves them towards, and when it stops.
//!
//! The target row belongs to the target goal alone, so it is greyed under the
//! two goals that have no target to state.

use egui::Ui;

use crate::simulation::dialog::OptimizationDialogState;

use super::{choice_row, input_row, input_row_enabled};

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
}
