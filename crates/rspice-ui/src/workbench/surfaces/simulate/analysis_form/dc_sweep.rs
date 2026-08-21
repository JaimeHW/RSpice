//! The DC sweep form: its fields, and the sentence describing what it will do.
//!
//! Split out because this one analysis carries two settings that exclude each
//! other and a note that changes with them. The exclusion is the reason: a
//! nested sweep is a family of curves over two sources, a retrace is one source
//! travelled twice, and "retrace a nested sweep" does not name an analysis —
//! there is no answer to which axis rewinds, or whether the outer source
//! rewinds between branches. Neither control is offered while the other is on,
//! so the pair can never be configured and then refused at preflight.

use egui::Ui;

use crate::workbench::app_state::DcSetup;

use super::{check_row, clear_pending_cell, input_row, input_row_enabled};

/// Render the DC sweep fields; returns the note describing the configured run.
pub(super) fn fields(ui: &mut Ui, setup: &mut DcSetup) -> &'static str {
    input_row(ui, "Source", &mut setup.source);
    input_row(ui, "Start", &mut setup.start);
    input_row(ui, "Stop", &mut setup.stop);
    input_row(ui, "Step", &mut setup.step);
    // Disabled rather than merely refused: a control that can be set and then
    // rejected at preflight teaches the reader nothing a greyed one does not.
    ui.add_enabled_ui(!setup.nested, |ui| {
        check_row(ui, "Bidirectional", &mut setup.hysteresis);
    });
    ui.add_enabled_ui(!setup.hysteresis, |ui| {
        check_row(ui, "Nested sweep", &mut setup.nested);
    });
    // Nested-sweep enablement is a complete field group. Do not pair the first
    // secondary-sweep value with the checkbox: doing so shifts every following
    // field by one column and leaves Step 2 stranded on a partial final row.
    clear_pending_cell(ui);
    input_row_enabled(ui, "Source 2", &mut setup.source2, setup.nested);
    input_row_enabled(ui, "Start 2", &mut setup.start2, setup.nested);
    input_row_enabled(ui, "Stop 2", &mut setup.stop2, setup.nested);
    input_row_enabled(ui, "Step 2", &mut setup.step2, setup.nested);
    note(setup)
}

/// What this configuration will actually do, stated rather than implied.
///
/// The retracing sentence is specific on purpose. A reverse branch is not a
/// second run: it continues from the state the forward branch ended in, which
/// is the only way the two branches can disagree and therefore the only reason
/// to ask for one. It also says where the answer appears — two named traces
/// over the same source values — because the direction cannot be read off the
/// axis, which both branches share.
pub(super) const fn note(setup: &DcSetup) -> &'static str {
    if setup.hysteresis {
        "Sweeps the source up and then back down in one continued solve, carrying the forward \
         branch's final state into the reverse branch. Each signal is reported as two traces, \
         [forward] and [reverse], over the same source values."
    } else {
        "Sweeps a source over the operating range."
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The note is the only place the form says what a retrace does, so it has
    /// to name the two things a reader cannot see anywhere else: that the
    /// branches are one continued solve, and that they arrive as two traces.
    #[test]
    fn the_note_states_what_a_retracing_sweep_actually_does() {
        let mut setup = DcSetup::default();
        assert_eq!(note(&setup), "Sweeps a source over the operating range.");

        setup.hysteresis = true;
        let note = note(&setup);
        assert!(note.contains("one continued solve"), "{note}");
        assert!(
            note.contains("final state into the reverse branch"),
            "{note}"
        );
        assert!(note.contains("[forward]"), "{note}");
        assert!(note.contains("[reverse]"), "{note}");
    }
}
