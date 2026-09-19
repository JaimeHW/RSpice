//! The safe-operating-area form: the transient window the checks are run
//! over, default voltage limits, and scoped voltage/current rules.
//!
//! Each bound is greyed by the check that reads it, so a limit can never be
//! typed into a check that is off.

use egui::Ui;

use crate::quantity::QuantityInputKind;
use crate::simulation::dialog::SoaDialogState;

use super::{
    QuantityPresentationPolicy, UiNumberLocale, action_line, choice_row, field_note, input_row,
    input_row_enabled, quantity_input_row, sub_header, switch_row,
};

/// Render the safe-operating-area fields.
pub(super) fn fields(
    ui: &mut Ui,
    setup: &mut SoaDialogState,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    quantity_input_row(
        ui,
        "Stop time",
        &mut setup.stop_time,
        QuantityInputKind::Time,
        policy,
        locale,
    );
    quantity_input_row(
        ui,
        "Step time",
        &mut setup.step_time,
        QuantityInputKind::Time,
        policy,
        locale,
    );
    quantity_input_row(
        ui,
        "Start checks at",
        &mut setup.start_time,
        QuantityInputKind::Time,
        policy,
        locale,
    );
    quantity_input_row(
        ui,
        "Maximum step",
        &mut setup.max_step,
        QuantityInputKind::Time,
        policy,
        locale,
    );
    switch_row(
        ui,
        "Use initial conditions",
        &mut setup.use_initial_conditions,
    );
    input_row(ui, "Devices", &mut setup.devices);
    input_row(ui, "Models", &mut setup.models);
    field_note(
        ui,
        "Optional exact names separated by spaces. Empty selects all. Subcircuit devices use X1.M1 (or X1:M1); local models use CELL::NM.",
    );
    switch_row(ui, "Check Vgs", &mut setup.check_vgs_max);
    input_row_enabled(ui, "Max Vgs", &mut setup.max_vgs, setup.check_vgs_max);
    switch_row(ui, "Check Vds", &mut setup.check_vds_max);
    input_row_enabled(ui, "Max Vds", &mut setup.max_vds, setup.check_vds_max);
    switch_row(ui, "Check Vbe", &mut setup.check_vbe_max);
    input_row_enabled(ui, "Max Vbe", &mut setup.max_vbe, setup.check_vbe_max);
    switch_row(ui, "Check Vce", &mut setup.check_vce_max);
    input_row_enabled(ui, "Max Vce", &mut setup.max_vce, setup.check_vce_max);
    sub_header(ui, "Scoped terminal rules");
    field_note(
        ui,
        "Extra rules override the same default limit within their scope. Empty device and model lists select all applicable devices; overlapping extra rules are rejected.",
    );
    let mut remove = None;
    for (index, rule) in setup.rules.iter_mut().enumerate() {
        ui.push_id(("soa-rule", index), |ui| {
            sub_header(ui, &format!("Rule {}", index + 1));
            choice_row(
                ui,
                "Parameter",
                &["Vgs", "Vds", "Vgd", "Vbe", "Vce", "Vbc", "Id", "Ic"],
                &mut rule.parameter,
            );
            input_row(
                ui,
                if rule.parameter >= 6 {
                    "Maximum |I| (A)"
                } else {
                    "Maximum |V| (V)"
                },
                &mut rule.max_value,
            );
            input_row(ui, "Devices", &mut rule.devices);
            input_row(ui, "Models", &mut rule.models);
            if action_line(ui, "Remove rule") {
                remove = Some(index);
            }
        });
    }
    if let Some(index) = remove {
        setup.rules.remove(index);
    }
    if action_line(ui, "+ Add terminal rule") {
        setup.rules.push(Default::default());
    }
}
