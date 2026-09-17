//! The periodic loop-stability form: the probe the loop is broken at, and the
//! bounds a periodic orbit is called unstable by.

use egui::Ui;

use crate::simulation::dialog::PstbDialogState;

use super::{engineering_input_row, input_row, stb_probe, switch_row};

/// Render the periodic loop-stability fields.
pub(super) fn fields(ui: &mut Ui, setup: &mut PstbDialogState, placed_loop_probes: &[String]) {
    // The same element STB designates, chosen the same way. It was
    // free text here long after the stability form's stopped being
    // one, so a probe name that matched nothing on the drawing failed
    // in the solver instead of being refused by name.
    stb_probe::row(
        ui,
        placed_loop_probes,
        &mut setup.probe,
        &mut setup.probe_reference,
    );
    input_row(ui, "Harmonics", &mut setup.max_harmonics);
    input_row(ui, "Multipliers", &mut setup.num_multipliers);
    engineering_input_row(ui, "Unstable above", &mut setup.stability_threshold);
    engineering_input_row(ui, "Eigen tol", &mut setup.eigenvalue_tolerance);
    switch_row(ui, "Detect subharmonics", &mut setup.detect_subharmonics);
}
