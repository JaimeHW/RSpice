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
    hinted_input_row(
        ui,
        "Frequencies",
        &mut setup.frequencies,
        "Hz, comma- or space-separated",
    );
    input_row(ui, "Table", &mut setup.table_name);
    match setup.to_config() {
        Ok(_) => field_note(ui, &setup.summary()),
        // The axis is refused by the same rule the run is refused by, so the
        // cause is stated where the list is typed rather than only at Run.
        Err(error) => field_advisory(ui, &error),
    }
}
