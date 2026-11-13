//! The AC frequency-table form: the frequencies themselves, and the table the
//! card reads them from.
//!
//! There is no band and no point density here — the list *is* the axis, so the
//! consequence a reader cannot see in the controls is how many points the run
//! will solve and where the axis begins and ends. That is said beneath the
//! list, and it is the same summary the plan manager shows, because it is
//! computed by the draft rather than re-derived here.

use egui::Ui;

use crate::simulation::plan::AcDataDraft;

use super::{field_advisory, field_note, hinted_input_row, input_row};

/// Render the AC frequency-table fields.
pub(super) fn fields(ui: &mut Ui, setup: &mut AcDataDraft) {
    let mut source = usize::from(setup.from_netlist);
    if super::choice_row(
        ui,
        "Data source",
        &["Authored here", "Netlist table"],
        &mut source,
    ) {
        setup.from_netlist = source == 1;
    }
    input_row(ui, "Table", &mut setup.table_name);
    if setup.from_netlist {
        field_note(
            ui,
            "Use the named .DATA table from the circuit or an included file, including its frequencies and parameter values.",
        );
    } else {
        hinted_input_row(
            ui,
            "Frequencies",
            &mut setup.frequencies,
            "Hz in solve order; zero and repeats allowed",
        );
        let mut remove = None;
        for (index, column) in setup.parameter_columns.iter_mut().enumerate() {
            ui.push_id(("ac-data-column", index), |ui| {
                super::sub_header(ui, &format!("Parameter column {}", index + 1));
                hinted_input_row(ui, "Parameter", &mut column.name, "e.g. load or R1:R");
                hinted_input_row(
                    ui,
                    "Values",
                    &mut column.values,
                    "one per frequency, in the same order",
                );
                if super::action_line(ui, "Remove column") {
                    remove = Some(index);
                }
            });
        }
        if let Some(index) = remove {
            setup.parameter_columns.remove(index);
        }
        if super::action_line(ui, "+ Add parameter column") {
            setup.parameter_columns.push(Default::default());
        }
        field_note(
            ui,
            "Each row applies all parameter values together. Separate frequencies and values with commas, semicolons or spaces; engineering suffixes are accepted.",
        );
    }
    match setup.to_config() {
        Ok(_) => field_note(ui, &setup.summary()),
        // The axis is refused by the same rule the run is refused by, so the
        // cause is stated where the list is typed rather than only at Run.
        Err(error) => field_advisory(ui, &error),
    }
}
