//! What a graded sweep's point field is called, in the mode selected beside it.
//!
//! Fourteen forms on this surface — AC, loop stability, noise, S-parameter,
//! PAC, PNOISE, PXF, DISTO, HBSP, HBNOISE, PSP, QPAC, QPNOISE and QPXF — pair a
//! point count with a mode selector. The count means a different thing in each
//! mode: a density per decade, a density per octave, or the entire number of
//! points. That is not a convention this surface chose; it is what the one
//! shared grid builder does with the keyword beside it — see
//! `services::simulation_runner::helpers::generate_freq_points_with_abort`,
//! which every one of these analyses reaches.
//!
//! A form that spells the field once therefore states something true in one
//! mode and false in the other two, and the noise form did exactly that: it
//! read `Points / decade` in every mode, including Linear, where an engineer
//! who types 50 gets fifty points in total rather than fifty per decade.
//!
//! That failure is silent. Nothing refuses the run, no diagnostic names it, and
//! the result is simply a sweep of the wrong density. So the spelling lives
//! here, resolved from the mode, rather than being written out at each of the
//! forms that would each have to remember to keep it honest.

use crate::simulation::config::NoiseSweepType;

/// The sweep modes a graded frequency sweep offers, in the order the selector
/// paints them. The index into this list is what the forms store.
pub(super) const SWEEP_KINDS: &[&str] = &["dec", "oct", "lin"];

/// What the point field is called where the mode does not grade the count: an
/// explicit frequency list, or a retained mode no option list claims. The field
/// still holds a number; nothing at that point can say what it is a number of.
pub(super) const SWEEP_POINT_NEUTRAL_LABEL: &str = "Points";

/// What one sweep point *is*, in the mode selected beside it.
///
/// The index is a position in [`SWEEP_KINDS`], which every graded sweep on this
/// surface stores directly.
pub(super) const fn sweep_point_field_label(sweep_kind: usize) -> &'static str {
    match sweep_kind {
        0 => "Points / decade",
        1 => "Points / octave",
        2 => "Total points",
        _ => SWEEP_POINT_NEUTRAL_LABEL,
    }
}

/// The same resolution for the noise form, whose sweep is a typed enum rather
/// than an index into [`SWEEP_KINDS`].
///
/// `NoiseSweepType` orders its first three modes the way [`SWEEP_KINDS`] does,
/// so the spellings are derived rather than copied;
/// `noise_sweep_modes_are_graded_the_way_the_shared_sweep_is` pins that
/// agreement instead of trusting it. The explicit-list mode grades nothing —
/// the axis is the list, and the point field is disabled beside it.
pub(super) fn noise_point_field_label(sweep: NoiseSweepType) -> &'static str {
    match sweep.selection_index() {
        Some(index @ (0 | 1 | 2)) => sweep_point_field_label(index),
        _ => SWEEP_POINT_NEUTRAL_LABEL,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The point field's units are the mode's, not a fixed spelling.
    ///
    /// Every graded sweep on this surface stores a [`SWEEP_KINDS`] index, so one
    /// resolver serves AC, loop stability and the noise form alike. The failure
    /// this pins is silent: a form that reads `Points / decade` in Linear mode
    /// tells an engineer their 50 is a density when the engine will read it as
    /// the whole count, and the run is wrong by a factor of the decade span.
    #[test]
    fn the_sweep_point_label_names_what_a_point_is_in_each_mode() {
        let by_mode = SWEEP_KINDS
            .iter()
            .enumerate()
            .map(|(index, kind)| (*kind, sweep_point_field_label(index)))
            .collect::<Vec<_>>();
        assert_eq!(
            by_mode,
            [
                ("dec", "Points / decade"),
                ("oct", "Points / octave"),
                ("lin", "Total points"),
            ]
        );
        // Every mode resolves to a distinct spelling; a label shared by two
        // modes would state nothing the mode selector had not already said.
        assert_eq!(
            by_mode
                .iter()
                .map(|(_, label)| *label)
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            SWEEP_KINDS.len()
        );
        // A retained index no option claims names a count without claiming to
        // know what it counts.
        assert_eq!(
            sweep_point_field_label(SWEEP_KINDS.len()),
            SWEEP_POINT_NEUTRAL_LABEL
        );
    }

    /// [`noise_point_field_label`] derives its spellings from the shared
    /// resolver by index. That is only sound while the two orderings agree, so
    /// this is the pin: reorder either list and this fails rather than silently
    /// labelling an octave sweep as a decade one.
    #[test]
    fn noise_sweep_modes_are_graded_the_way_the_shared_sweep_is() {
        for (mode, expected) in [
            (NoiseSweepType::Decade, "Points / decade"),
            (NoiseSweepType::Octave, "Points / octave"),
            (NoiseSweepType::Linear, "Total points"),
        ] {
            let index = mode
                .selection_index()
                .expect("a graded noise mode occupies an option slot");
            assert_eq!(sweep_point_field_label(index), expected, "{mode:?}");
            assert_eq!(noise_point_field_label(mode), expected, "{mode:?}");
        }
        // The explicit list grades nothing: its axis is the list, and the point
        // field is disabled beside it.
        assert_eq!(
            noise_point_field_label(NoiseSweepType::ExplicitFrequencyList),
            SWEEP_POINT_NEUTRAL_LABEL
        );
    }
}
