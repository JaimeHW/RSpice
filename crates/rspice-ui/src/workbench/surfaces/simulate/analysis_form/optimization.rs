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
    choice_row, clear_pending_cell, engineering_input_row, engineering_input_row_enabled,
    field_note, hinted_input_row, input_row, input_row_enabled, sub_header,
};

/// Render the optimization fields.
pub(super) fn fields(
    ui: &mut Ui,
    setup: &mut OptimizationDialogState,
    bases: &[(crate::product::AnalysisInstanceId, String)],
) {
    super::study_base_row(
        ui,
        &mut setup.base_analysis,
        bases,
        "Operating point expression",
    );
    let configured = setup.base_analysis.is_some();
    if configured {
        super::switch_row(ui, "Weighted objectives", &mut setup.weighted_objectives);
    }
    let weighted = configured && setup.weighted_objectives;
    if weighted {
        weighted_objectives(ui, setup);
    }
    super::hinted_input_row_enabled(
        ui,
        "Measurement",
        &mut setup.objective_measurement,
        "gain, scalar:V(out), or last:signal",
        configured && !weighted,
    );
    if configured {
        field_note(
            ui,
            "Each candidate runs the selected analysis at this study's Run Set point. Use a .MEAS name for a measured scalar, scalar:name for a native scalar, or last:signal for the final sample (AC real component). For Fourier/FFT use bin:1:magnitude (or real, imag, phase in degrees), with :signal appended for multiple Fourier outputs; scalar:THD(%) and scalar:DC select Fourier scalars. The configured transient producer is rerun for every candidate.",
        );
    }
    input_row(ui, "Variables", &mut setup.variables_text)
        .on_hover_text("One per line or comma, spelled name:min:max[:initial].");
    variable_domains(ui, setup);
    if configured {
        constraints(ui, setup);
    }
    input_row_enabled(
        ui,
        "Expression",
        &mut setup.objective_expression,
        !configured,
    );
    if !configured {
        field_note(
            ui,
            "Optional scalar expression, e.g. -V(supply)*I(VSUP) for supplied power. Empty uses the node voltage below.",
        );
    }
    let node_objective = !configured && setup.objective_expression.trim().is_empty();
    input_row_enabled(ui, "Objective", &mut setup.objective_node, node_objective);
    input_row_enabled(ui, "Obj ref", &mut setup.objective_ref, node_objective);
    super::choice_row_enabled(
        ui,
        "Goal",
        &["min", "max", "target"],
        &mut setup.goal_mode,
        !weighted,
    );
    input_row_enabled(
        ui,
        "Target",
        &mut setup.target_value,
        !weighted && setup.goal_mode == 2,
    );
    choice_row(
        ui,
        "Method",
        &["gradient", "pattern", "anneal"],
        &mut setup.algorithm,
    );
    input_row(ui, "Max iters", &mut setup.max_iterations);
    input_row(ui, "Tolerance", &mut setup.cost_tolerance);

    sub_header(ui, "Step sizes");
    engineering_input_row_enabled(
        ui,
        "Gradient probe",
        &mut setup.fd_step,
        setup.algorithm == 0,
    );
    engineering_input_row_enabled(
        ui,
        "Gradient stop",
        &mut setup.var_tolerance,
        setup.algorithm == 0,
    );
    engineering_input_row(ui, "First step", &mut setup.initial_step);
    hinted_input_row(
        ui,
        "Smallest step",
        &mut setup.min_step,
        "at most the first step",
    );
    clear_pending_cell(ui);
    sub_header(ui, "Annealing");
    engineering_input_row_enabled(
        ui,
        "Initial temperature",
        &mut setup.sa_initial_temp,
        setup.algorithm == 2,
    );
    engineering_input_row_enabled(
        ui,
        "Cooling factor",
        &mut setup.sa_cooling_rate,
        setup.algorithm == 2,
    );
    input_row_enabled(
        ui,
        "Random seed",
        &mut setup.random_seed,
        setup.algorithm == 2,
    )
    .on_hover_text("Repeatable random sequence. Seeds 0 and 1 select the same sequence.");
    clear_pending_cell(ui);
}

fn weighted_objectives(ui: &mut Ui, setup: &mut OptimizationDialogState) {
    field_note(
        ui,
        "All objectives use the selected base analysis. Cost is the sum of weighted terms: minimize value/scale, maximize -value/scale, or target ((value-target)/scale)². Scale is in the measurement's units; weight sets relative importance. Tolerance applies to the combined cost.",
    );
    let mut remove = None;
    for (index, term) in setup.objective_terms.iter_mut().enumerate() {
        ui.push_id(("optimization-objective", index), |ui| {
            sub_header(ui, &format!("Objective {}", index + 1));
            input_row(ui, "Measurement", &mut term.measurement);
            choice_row(ui, "Goal", &["min", "max", "target"], &mut term.goal);
            input_row_enabled(ui, "Target", &mut term.target, term.goal == 2);
            engineering_input_row(ui, "Scale", &mut term.scale);
            engineering_input_row(ui, "Weight", &mut term.weight);
            if super::action_line(ui, "Remove objective") {
                remove = Some(index);
            }
        });
    }
    if let Some(index) = remove {
        setup.objective_terms.remove(index);
    }
    if super::action_line(ui, "+ Add objective") {
        setup.objective_terms.push(Default::default());
    }
}

fn variable_domains(ui: &mut Ui, setup: &mut OptimizationDialogState) {
    sub_header(ui, "Variable domains");
    field_note(
        ui,
        "Variables use continuous linear ranges by default. Add an override for a logarithmic range, a uniform grid anchored at the minimum, or an increasing list of allowed values. The initial value must be allowed. Grid/list variables require pattern search or annealing. All reported values remain in physical units.",
    );
    let mut remove = None;
    for (index, row) in setup.variable_domains.iter_mut().enumerate() {
        ui.push_id(("optimization-domain", index), |ui| {
            input_row(ui, "Variable", &mut row.name);
            choice_row(
                ui,
                "Domain",
                &["linear", "log", "grid", "list"],
                &mut row.mode,
            );
            input_row_enabled(ui, "Grid step", &mut row.step, row.mode == 2);
            input_row_enabled(ui, "Allowed values", &mut row.values, row.mode == 3);
            if super::action_line(ui, "Remove variable domain") {
                remove = Some(index);
            }
        });
    }
    if let Some(index) = remove {
        setup.variable_domains.remove(index);
    }
    if super::action_line(ui, "+ Add variable domain") {
        setup.variable_domains.push(Default::default());
    }
}

fn constraints(ui: &mut Ui, setup: &mut OptimizationDialogState) {
    sub_header(ui, "Measurement constraints");
    field_note(
        ui,
        "Limits use the selected base analysis. Leave one bound blank for a one-sided limit; set equal bounds for equality. Tolerance is an absolute allowance in measurement units. Scale normalizes violations while searching for feasibility. A feasible design always outranks an infeasible design, regardless of objective cost.",
    );
    let mut remove = None;
    for (index, row) in setup.constraints.iter_mut().enumerate() {
        ui.push_id(("optimization-constraint", index), |ui| {
            input_row(ui, "Measurement", &mut row.measurement);
            input_row(ui, "Lower limit", &mut row.lower);
            input_row(ui, "Upper limit", &mut row.upper);
            input_row(ui, "Limit tolerance", &mut row.tolerance);
            input_row(ui, "Violation scale", &mut row.scale);
            if super::action_line(ui, "Remove constraint") {
                remove = Some(index);
            }
        });
    }
    if let Some(index) = remove {
        setup.constraints.remove(index);
    }
    if super::action_line(ui, "+ Add constraint") {
        setup.constraints.push(Default::default());
    }
}
