//! The reliability form: the ages the degradation is projected to, and which
//! wear-out mechanisms are projected.

use egui::Ui;

use crate::simulation::dialog::ReliabilityDialogState;

use super::{input_row, switch_row};

/// Render the reliability fields.
pub(super) fn fields(ui: &mut Ui, setup: &mut ReliabilityDialogState) {
    input_row(ui, "Years", &mut setup.years_csv);
    input_row(ui, "Min stress V", &mut setup.min_stress_voltage);
    switch_row(ui, "Hot carrier (HCI)", &mut setup.enable_hci);
    switch_row(ui, "Bias instability (NBTI)", &mut setup.enable_nbti);
    switch_row(ui, "Electromigration", &mut setup.enable_em);
}
