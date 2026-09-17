//! The DC operating-point form: the temperature the point is solved at, the
//! startup policy that reaches it, and what the solved point reports.
//!
//! Two of its rows exclude each other. An initial guess and a node
//! initialization that cannot both be honoured is not a configuration the
//! engine has, so each row greys the combinations the other rules out rather
//! than letting the pair be set and then refused at preflight.

use egui::Ui;

use crate::simulation::dialog::OpDialogState;
use crate::ui::tokens::Tokens;

use super::{
    OpContextAvailability, choice_row, choice_row_with_disabled, field_cell, full_width_field,
    mono_input, select, uses_two_column_fields,
};

pub(super) const OP_FIELD_LABELS: [&str; 8] = [
    "Temperature",
    "Initial guess",
    "Node initialization",
    "Homotopy strategy",
    "Annotate schematic",
    "Device detail",
    "Save device OP",
    "Accuracy preset",
];

pub(super) const OP_TEMPERATURE_CHOICES: [&str; 4] = [
    "PVT run set",
    "Nominal temperature \u{00b7} 27 \u{00b0}C",
    "Explicit temperature\u{2026}",
    "Inherit active run-set axis",
];

pub(super) const OP_INITIAL_GUESS_CHOICES: [&str; 4] = [
    "Automatic",
    "Previous converged solution",
    "User node voltages",
    "Zero state",
];

pub(super) const OP_NODE_INITIALIZATION_CHOICES: [&str; 4] = [
    "Use IC / nodeset",
    "Ignore IC and nodeset",
    "Force .ic values",
    "Validate initialization only",
];

pub(super) const OP_HOMOTOPY_CHOICES: [&str; 5] = [
    "Adaptive",
    "Source stepping",
    "Gmin stepping",
    "Pseudo-transient",
    "None",
];

pub(super) const OP_ANNOTATION_CHOICES: [&str; 4] = [
    "Voltages + currents",
    "Voltages only",
    "Voltages + device OP",
    "None",
];

pub(super) const OP_DEVICE_DETAIL_CHOICES: [&str; 4] = [
    "Selected + violations",
    "All devices",
    "Violations only",
    "None",
];

pub(super) const OP_SAVE_DEVICE_CHOICES: [&str; 3] = ["Enabled", "Disabled", "Final point only"];

pub(super) const OP_ACCURACY_CHOICES: [&str; 4] = ["Fast", "Balanced", "Accurate", "Robust"];

pub(super) const OP_STARTUP_CONFLICT: &str =
    "This initial-guess and node-initialization combination is not executable";

pub(super) fn op_initial_guess_disabled(
    node_initialization_idx: usize,
    previous_state_available: bool,
) -> Vec<(usize, &'static str)> {
    let mut disabled = Vec::new();
    for initial_guess_idx in 0..OP_INITIAL_GUESS_CHOICES.len() {
        if !op_startup_indices_compatible(initial_guess_idx, node_initialization_idx) {
            disabled.push((initial_guess_idx, OP_STARTUP_CONFLICT));
        } else if initial_guess_idx == 1 && !previous_state_available {
            disabled.push((
                initial_guess_idx,
                "Run and retain a source-compatible OP state before selecting this policy",
            ));
        }
    }
    disabled
}

pub(super) fn op_node_initialization_disabled(
    initial_guess_idx: usize,
) -> Vec<(usize, &'static str)> {
    (0..OP_NODE_INITIALIZATION_CHOICES.len())
        .filter(|node_idx| !op_startup_indices_compatible(initial_guess_idx, *node_idx))
        .map(|node_idx| (node_idx, OP_STARTUP_CONFLICT))
        .collect()
}

pub(super) const fn op_startup_indices_compatible(
    initial_guess_idx: usize,
    node_initialization_idx: usize,
) -> bool {
    match initial_guess_idx {
        0 => true,
        1 | 3 => matches!(node_initialization_idx, 1 | 3),
        2 => matches!(node_initialization_idx, 0 | 2),
        _ => false,
    }
}

/// The temperature row: the mode that decides where the value comes from,
/// and the explicit value the one mode that states its own is edited in.
pub(super) fn op_temperature_row(
    ui: &mut Ui,
    setup: &mut crate::simulation::dialog::OpDialogState,
) {
    if !uses_two_column_fields(ui) {
        let control_height = Tokens::get(ui.ctx()).metrics.ctl_h * 2.0 + 6.0;
        full_width_field(
            ui,
            OP_FIELD_LABELS[0],
            Some("Celsius"),
            control_height,
            |ui| {
                let current = OP_TEMPERATURE_CHOICES
                    .get(setup.temperature_mode_idx)
                    .copied()
                    .unwrap_or("Schema unavailable");
                if let Some(index) = select(
                    ui,
                    "op-temperature-mode-stacked",
                    OP_FIELD_LABELS[0],
                    current,
                    &OP_TEMPERATURE_CHOICES.map(|value| value.to_owned()),
                    ui.available_width(),
                ) {
                    setup.temperature_mode_idx = index;
                    if index == 1 {
                        setup.temperature = "27".to_owned();
                    }
                }
                ui.add_enabled_ui(setup.temperature_mode_idx == 2, |ui| {
                    mono_input(
                        ui,
                        OP_FIELD_LABELS[0],
                        &mut setup.temperature,
                        ui.available_width(),
                    )
                });
            },
        );
        return;
    }
    field_cell(ui, OP_FIELD_LABELS[0], Some("Celsius"), |ui| {
        let input_width = 92.0;
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            let select_width = (ui.available_width() - input_width - 6.0).max(1.0);
            let current = OP_TEMPERATURE_CHOICES
                .get(setup.temperature_mode_idx)
                .copied()
                .unwrap_or("Schema unavailable");
            if let Some(index) = select(
                ui,
                "op-temperature-mode",
                OP_FIELD_LABELS[0],
                current,
                &OP_TEMPERATURE_CHOICES.map(|value| value.to_owned()),
                select_width,
            ) {
                setup.temperature_mode_idx = index;
                if index == 1 {
                    setup.temperature = "27".to_owned();
                }
            }
            ui.add_enabled_ui(setup.temperature_mode_idx == 2, |ui| {
                mono_input(ui, OP_FIELD_LABELS[0], &mut setup.temperature, input_width)
            });
        });
    });
}

/// Render the operating-point fields.
pub(super) fn fields(ui: &mut Ui, setup: &mut OpDialogState, op_context: OpContextAvailability) {
    setup.ensure_initialized();
    op_temperature_row(ui, setup);
    let initial_guess_disabled =
        op_initial_guess_disabled(setup.node_initialization_idx, op_context.previous_state);
    choice_row_with_disabled(
        ui,
        OP_FIELD_LABELS[1],
        &OP_INITIAL_GUESS_CHOICES,
        &mut setup.initial_guess_idx,
        &initial_guess_disabled,
    );
    let node_initialization_disabled = op_node_initialization_disabled(setup.initial_guess_idx);
    choice_row_with_disabled(
        ui,
        OP_FIELD_LABELS[2],
        &OP_NODE_INITIALIZATION_CHOICES,
        &mut setup.node_initialization_idx,
        &node_initialization_disabled,
    );
    choice_row(
        ui,
        OP_FIELD_LABELS[3],
        &OP_HOMOTOPY_CHOICES,
        &mut setup.homotopy_idx,
    );
    choice_row(
        ui,
        OP_FIELD_LABELS[4],
        &OP_ANNOTATION_CHOICES,
        &mut setup.annotation_idx,
    );
    choice_row_with_disabled(
        ui,
        OP_FIELD_LABELS[5],
        &OP_DEVICE_DETAIL_CHOICES,
        &mut setup.device_detail_idx,
        if op_context.soa_violations {
            &[]
        } else {
            &[(
                2,
                "Run SOA checks with warning or violation evidence before selecting this policy",
            )]
        },
    );
    choice_row(
        ui,
        OP_FIELD_LABELS[6],
        &OP_SAVE_DEVICE_CHOICES,
        &mut setup.save_device_op_idx,
    );
    choice_row(
        ui,
        OP_FIELD_LABELS[7],
        &OP_ACCURACY_CHOICES,
        &mut setup.accuracy_idx,
    );
}
