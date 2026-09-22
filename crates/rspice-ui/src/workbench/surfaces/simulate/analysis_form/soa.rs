//! The safe-operating-area form: the transient window the checks are run
//! over, default voltage limits, and scoped voltage/current/power/temperature rules.
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
    sub_header(ui, "Severity thresholds");
    input_row(ui, "Warning above (% of limit)", &mut setup.warning_percent);
    input_row(
        ui,
        "Critical above (% of limit)",
        &mut setup.critical_percent,
    );
    field_note(
        ui,
        "Leave blank to disable that severity band. Warning accepts 0–100%; critical accepts 100% or higher. Values above the actual limit produce a violation when the rule's duration policy qualifies the excursion. These thresholds apply to every rule, including imported and temperature-derated limits.",
    );
    switch_row(
        ui,
        "Import model voltage ratings",
        &mut setup.import_model_voltage_ratings,
    );
    field_note(
        ui,
        "Uses authored voltage ratings from the selected native BSIM3/4 (including BSIM3 LEVEL=9), VDMOS, Gummel–Poon or VBIC BJT, or diode model. Imported ratings replace matching default checks; scoped rules can override them. Current, power and temperature ratings require separate rules. Devices without voltage ratings keep their enabled defaults; unsupported rated models report an error.",
    );
    switch_row(ui, "Check Vgs", &mut setup.check_vgs_max);
    input_row_enabled(ui, "Max Vgs", &mut setup.max_vgs, setup.check_vgs_max);
    switch_row(ui, "Check Vds", &mut setup.check_vds_max);
    input_row_enabled(ui, "Max Vds", &mut setup.max_vds, setup.check_vds_max);
    switch_row(ui, "Check Vbe", &mut setup.check_vbe_max);
    input_row_enabled(ui, "Max Vbe", &mut setup.max_vbe, setup.check_vbe_max);
    switch_row(ui, "Check Vce", &mut setup.check_vce_max);
    input_row_enabled(ui, "Max Vce", &mut setup.max_vce, setup.check_vce_max);
    sub_header(ui, "Scoped device rules");
    field_note(
        ui,
        "Extra rules override matching defaults within their scope. Positive/negative limits use the named terminal order and replace only that side of a default magnitude limit. Empty scope selects all applicable devices; repeated explicit constraints are rejected.",
    );
    field_note(
        ui,
        "Vgs = V(g) − V(s), Vds = V(d) − V(s), Vgd = V(g) − V(d); BJT voltages follow the same named-terminal order. Id/Ig/Is and Ic/Ib/Ie are positive into drain/gate/source and collector/base/emitter, including accepted transient displacement current. With intrinsic voltage off, Vbs/Vbd/Vgb use the external bulk/body contact. Bulk current always measures that external contact. Ves/Ved/Vge and back-gate current use the SOI back gate/substrate electrode; floating internal body states are not external terminals. BJT substrate rules use an explicit electrical substrate pin, never a thermal pin. Diode Vak = V(anode) − V(cathode); Ia is positive into the anode. Enter a nonnegative magnitude for directional limits; zero forbids that polarity.",
    );
    field_note(
        ui,
        "Temperature limits are entered in °C; results use absolute kelvin. The check observes the temperature used by the model, including self-heating only where the model implements it.",
    );
    field_note(
        ui,
        "Power checks use the model's positive conductive power, including series-resistor loss. Capacitor energy storage and release are excluded. The model determines which physical losses are represented.",
    );
    let mut remove = None;
    for (index, rule) in setup.rules.iter_mut().enumerate() {
        ui.push_id(("soa-rule", index), |ui| {
            sub_header(ui, &format!("Rule {}", index + 1));
            choice_row(
                ui,
                "Parameter",
                &crate::simulation::dialog::soa::SoaRuleDraft::PARAMETER_LABELS,
                &mut rule.parameter,
            );
            quantity_input_row(ui, "Duration threshold", &mut rule.minimum_duration, QuantityInputKind::Time, policy, locale);
            field_note(ui, "Blank reports every exceedance. With a threshold, unqualified excursions remain warnings when warnings are enabled. Crossings are linearly interpolated within the checked time window; each complete excursion is classified after the run.");
            ui.add_enabled_ui(!rule.minimum_duration.trim().is_empty(), |ui| {
                switch_row(ui, "Accumulate repeated excursions", &mut rule.cumulative_duration);
                if rule.cumulative_duration {
                    quantity_input_row(ui, "Recovery time constant", &mut rule.recovery_time, QuantityInputKind::Time, policy, locale);
                    field_note(ui, "Exposure starts at zero and adds time above the limit. Blank recovery retains all exposure; a time constant makes it decay exponentially between excursions. An excursion qualifies when its ending exposure reaches the threshold. Earlier excursions keep their own verdicts. This is a screening rule, not a device lifetime prediction.");
                } else {
                    field_note(ui, "Each excursion must last at least the duration threshold to qualify as a violation or critical event.");
                }
            });
            if rule.is_voltage() {
                switch_row(ui, "Observe intrinsic voltage", &mut rule.intrinsic_voltage);
                field_note(ui, "Intrinsic voltages use the model's electrical nodes behind lead resistance, including floating bodies. Polarity follows the named node order. Default checks use external pins.");
            }
            input_row(
                ui,
                if rule.is_power() {
                    "Maximum power (W)"
                } else if rule.is_temperature() {
                    "Maximum temperature (°C)"
                } else if rule.is_current() {
                    "Limit magnitude (A)"
                } else {
                    "Limit magnitude (V)"
                },
                &mut rule.max_value,
            );
            if rule.is_power() {
                switch_row(ui, "Derate power with temperature", &mut rule.derate_power);
                if rule.derate_power {
                    input_row(ui, "Reference temperature (°C)", &mut rule.derating_temperature_celsius);
                    input_row(ui, "Power reduction (W/°C)", &mut rule.derating_watts_per_kelvin);
                    field_note(ui, "Full rated power applies through the reference temperature. Above it, the allowed power decreases at this rate, down to zero. Uses the model's accepted device temperature, including supported self-heating.");
                }
            }
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
    if action_line(ui, "+ Add device rule") {
        setup.rules.push(Default::default());
    }
}
