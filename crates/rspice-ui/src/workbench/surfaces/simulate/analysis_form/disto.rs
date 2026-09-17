//! The distortion form: the band the Volterra run is taken over, and the
//! second tone's ratio to the first.
//!
//! The ratio is the one bound the run refuses on, and it is said where it is
//! typed: it has no meaning at or above the first tone, and an empty field is
//! the single-tone run rather than a mistake.

use egui::Ui;

use crate::quantity::QuantityInputKind;
use crate::simulation::plan::DistoDraft;

use super::{
    QuantityPresentationPolicy, SWEEP_KINDS, UiNumberLocale, choice_row, field_note, input_row,
    quantity_input_row, sweep_point_field_label,
};

/// Render the distortion fields.
pub(super) fn fields(
    ui: &mut Ui,
    setup: &mut DistoDraft,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    quantity_input_row(
        ui,
        "Start",
        &mut setup.sweep.fstart,
        QuantityInputKind::Frequency,
        policy,
        locale,
    );
    quantity_input_row(
        ui,
        "Stop",
        &mut setup.sweep.fstop,
        QuantityInputKind::Frequency,
        policy,
        locale,
    );
    input_row(
        ui,
        sweep_point_field_label(setup.sweep.sweep),
        &mut setup.sweep.points,
    );
    choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.sweep.sweep);
    input_row(ui, "f2/f1", &mut setup.f2_over_f1);
    // The one bound the Volterra run refuses on, said where it is
    // typed: the ratio has no meaning at or above the first tone, and
    // an empty field is the single-tone run rather than a mistake.
    field_note(
        ui,
        "Between 0 and 1, exclusive, or empty for a single-tone run.",
    );
}
