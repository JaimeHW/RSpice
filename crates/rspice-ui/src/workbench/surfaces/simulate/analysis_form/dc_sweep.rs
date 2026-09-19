//! The DC sweep form: its fields, and what a retrace does to them.
//!
//! Split out because this one analysis carries two settings that exclude each
//! other and a note that one of them turns on. The exclusion is the reason: a
//! nested sweep is a family of curves over two sources, a retrace is one source
//! travelled twice, and "retrace a nested sweep" does not name an analysis —
//! there is no answer to which axis rewinds, or whether the outer source
//! rewinds between branches. Neither control is offered while the other is on,
//! so the pair can never be configured and then refused at preflight.

use egui::Ui;

use crate::workbench::app_state::DcSetup;

use super::{choice_row, clear_pending_cell, field_note, input_row, input_row_enabled, switch_row};

/// Render the DC sweep fields.
pub(super) fn fields(ui: &mut Ui, setup: &mut DcSetup) {
    input_row(ui, "Source", &mut setup.source);
    choice_row(
        ui,
        "Sweep",
        &["Linear", "List", "Decade", "Octave"],
        &mut setup.mode,
    );
    if setup.mode == 1 {
        input_row(ui, "Values", &mut setup.values);
        field_note(
            ui,
            "Values run in the authored order, including repeats. SI suffixes are accepted.",
        );
    } else {
        input_row(ui, "Start", &mut setup.start);
        input_row(ui, "Stop", &mut setup.stop);
        if setup.mode == 0 {
            input_row(ui, "Step", &mut setup.step);
        } else {
            input_row(
                ui,
                if setup.mode == 2 {
                    "Points / decade"
                } else {
                    "Points / octave"
                },
                &mut setup.points,
            );
        }
    }
    // Disabled rather than merely refused: a control that can be set and then
    // rejected at preflight teaches the reader nothing a greyed one does not.
    ui.add_enabled_ui(!setup.nested, |ui| {
        switch_row(ui, "Bidirectional", &mut setup.hysteresis);
    });
    // Beside the switch that turns it on, because it is that switch's
    // consequence and nothing else on the form carries it.
    if let Some(note) = retrace_note(setup) {
        field_note(ui, note);
    }
    ui.add_enabled_ui(!setup.hysteresis, |ui| {
        switch_row(ui, "Nested sweep", &mut setup.nested);
    });
    // Nested-sweep enablement is a complete field group. Do not pair the first
    // secondary-sweep value with the checkbox: doing so shifts every following
    // field by one column and leaves Step 2 stranded on a partial final row.
    clear_pending_cell(ui);
    input_row_enabled(ui, "Source 2", &mut setup.source2, setup.nested);
    ui.add_enabled_ui(setup.nested, |ui| {
        choice_row(
            ui,
            "Sweep 2",
            &["Linear", "List", "Decade", "Octave"],
            &mut setup.mode2,
        );
        if setup.mode2 == 1 {
            input_row(ui, "Values 2", &mut setup.values2);
        } else {
            input_row(ui, "Start 2", &mut setup.start2);
            input_row(ui, "Stop 2", &mut setup.stop2);
            if setup.mode2 == 0 {
                input_row(ui, "Step 2", &mut setup.step2);
            } else {
                input_row(
                    ui,
                    if setup.mode2 == 2 {
                        "Points / decade 2"
                    } else {
                        "Points / octave 2"
                    },
                    &mut setup.points2,
                );
            }
        }
    });
}

/// What a retrace will actually do, stated rather than implied. `None` while
/// the switch is off: a one-way sweep is what the four fields above already
/// say, and repeating them under the form is the kind's description, not this
/// configuration's consequence.
///
/// The retracing sentence is specific on purpose. A reverse branch is not a
/// second run: it continues from the state the forward branch ended in, which
/// is the only way the two branches can disagree and therefore the only reason
/// to ask for one. It also says where the answer appears — two named traces
/// over the same source values — because the direction cannot be read off the
/// axis, which both branches share.
///
/// And it says which sweep sources continue that way, because only some do.
/// `DcSweepConfig::validate` refuses a retrace over anything but an
/// independent voltage or current source: the engine dispatches `TEMP`, a
/// `.param` and a device parameter by cloning the netlist and re-solving from
/// cold at every point, so those branches are bit-identical and the retrace is
/// a second run of the same numbers under two trace names. A note that
/// promised a continued solve regardless of source promised something the
/// configuration would then refuse.
const fn retrace_note(setup: &DcSetup) -> Option<&'static str> {
    if setup.hysteresis {
        Some(
            "Sweeps the source up and then back down in one continued solve, carrying the \
             forward branch's final state into the reverse branch. Each signal is reported as \
             two traces, [forward] and [reverse], over the same source values. Only an \
             independent V or I source carries state that way, so a retrace over a temperature, \
             a parameter or a device parameter is refused rather than run as two identical \
             branches.",
        )
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The note is the only place the form says what a retrace does, so it has
    /// to name the two things a reader cannot see anywhere else: that the
    /// branches are one continued solve, and that they arrive as two traces.
    /// A one-way sweep gets no note at all — the fields are the whole story.
    #[test]
    fn the_note_states_what_a_retracing_sweep_actually_does() {
        let mut setup = DcSetup::default();
        assert_eq!(retrace_note(&setup), None);

        setup.hysteresis = true;
        let note = retrace_note(&setup).expect("a retrace has a consequence to state");
        assert!(note.contains("one continued solve"), "{note}");
        assert!(
            note.contains("final state into the reverse branch"),
            "{note}"
        );
        assert!(note.contains("[forward]"), "{note}");
        assert!(note.contains("[reverse]"), "{note}");
    }

    /// The note names the sources a retrace is available over.
    ///
    /// `DcSweepConfig::validate` refuses one over a temperature, a parameter or
    /// a device parameter, because those re-solve from cold at every point and
    /// carry no state between branches. The note promised the continued solve
    /// regardless of source, which is a promise the configuration would then
    /// refuse — and it is the note, not the refusal, that the reader has in
    /// front of them while they decide.
    #[test]
    fn the_note_says_which_sources_a_retrace_is_available_over() {
        let setup = DcSetup {
            hysteresis: true,
            ..DcSetup::default()
        };
        let note = retrace_note(&setup).expect("a retrace has a consequence to state");
        assert!(note.contains("independent V or I source"), "{note}");
        assert!(note.contains("refused"), "{note}");
    }
}
