//! The stability-margin conventions both cards are ratified to report under.
//!
//! Margins are measured twice in this application — once off the Bode card's
//! magnitude/phase pair, once off the Nyquist card's complex locus — by two
//! implementations that share no code. That independence is worth keeping:
//! it is what makes their agreement evidence rather than tautology. What is
//! *not* worth keeping independent is the convention itself. A convention
//! stated twice is two conventions, and the two cards drifted apart on both
//! of the rules below before this module existed:
//!
//! - the phase margin's fold, where one card reported `180° + ∠L` unbounded
//!   and the other wrapped it, so a loop whose phase had wound past a full
//!   turn at crossover read 360° apart on the two sheets;
//! - the gain margin's tie band, where one card broke a dead heat at the
//!   unity-gain frequency and the other compared magnitudes strictly, so two
//!   inversions 0.005 dB apart in `|GM|` produced margins of opposite sign at
//!   frequencies a factor of two apart.
//!
//! Both cards call the functions here. Neither restates them.

/// A phase margin, folded into the turn every tool reports one in.
///
/// `PM = 180° + ∠L` at the unity-gain crossing, wrapped into `(-180°, 180°]`.
/// The wrap is the convention MATLAB's `margin`, ADS and Spectre's `stb` all
/// report under, and it is what makes the number comparable against those
/// tools and against a specification limit.
///
/// It is applied to the value *and* to the key a card selects the binding
/// crossing by, because selecting on one quantity and reporting another lets
/// two cards name different crossings on the same loop.
///
/// The fold is lossy on purpose, and the loss is why
/// [`phase_margin_is_folded`] exists: a loop whose phase has wound past a
/// full turn at crossover reads as healthy here, and only the encirclement
/// count settles it. A card reporting a folded margin has to say so.
#[must_use]
pub fn phase_margin_deg(loop_phase_deg: f64) -> f64 {
    let margin = 180.0 + loop_phase_deg;
    if !margin.is_finite() {
        return margin;
    }
    let wrapped = (margin + 180.0).rem_euclid(360.0) - 180.0;
    // `rem_euclid` maps the closed lower edge to itself; the reported turn is
    // half-open at the bottom, so -180° is the same angle as +180°.
    if wrapped == -180.0 { 180.0 } else { wrapped }
}

/// Whether [`phase_margin_deg`] had to fold, so the number it returned is not
/// the angle the loop actually reached at crossover.
///
/// True in two shapes, and a card's note has to cover both: a loop whose lag
/// has passed a full turn — every loop closed over a transport delay — and a
/// loop with net phase *lead* at crossover, where `180° + ∠L` runs past
/// +180° instead.
#[must_use]
pub fn phase_margin_is_folded(loop_phase_deg: f64) -> bool {
    let margin = 180.0 + loop_phase_deg;
    margin.is_finite() && !(margin > -180.0 && margin <= 180.0)
}

/// What a card says beside a folded phase margin.
///
/// Stated rather than hinted: the margin above it is a real number that reads
/// like a verdict, and on a wound loop it is not one. The note names the angle
/// the loop actually reached, so the reader can see how far the fold moved it,
/// and points at the measurement that does settle stability.
#[must_use]
pub fn folded_phase_margin_note(loop_phase_deg: f64) -> String {
    format!(
        "∠L is {loop_phase_deg:.0}° at the unity-gain crossing, outside the ±180° turn a phase \
         margin is reported in, so the margin above is that angle folded into one turn. A folded \
         margin cannot settle stability on its own — the encirclement count on the Nyquist sheet \
         does."
    )
}

/// How close two −180° crossings have to be in dB before a gain margin stops
/// preferring either on magnitude and breaks the tie at the unity-gain
/// frequency instead.
///
/// A hundredth of a dB is what interpolation between sweep samples is good to,
/// so anything inside it is a distinction the measurement does not carry. The
/// band is not a nicety: two inversions can sit the same distance either side
/// of unity, one with headroom and one already past the critical point, and
/// separating those on a hundredth of that distance makes the *sign* of the
/// reported margin a function of the sweep's point density.
pub const GAIN_MARGIN_TIE_DECIBELS: f64 = 1.0e-2;

#[cfg(test)]
mod tests {
    use super::*;

    /// An ordinary loop's margin passes through untouched — the fold is only
    /// ever a last resort, not a normalisation applied to every reading.
    #[test]
    fn a_margin_already_inside_one_turn_is_not_folded() {
        for loop_phase_deg in [-120.0, -179.999, -359.999, -1.0, -300.0] {
            assert_eq!(
                phase_margin_deg(loop_phase_deg),
                180.0 + loop_phase_deg,
                "∠L = {loop_phase_deg}° needed no fold"
            );
            assert!(!phase_margin_is_folded(loop_phase_deg));
        }
    }

    /// The wound case, from the closed form: a loop at −449.4° has 180−449.4
    /// = −269.4° of unfolded margin, which is +90.6° once folded.
    #[test]
    fn a_wound_loop_folds_by_exactly_one_turn() {
        let loop_phase_deg = -449.4;
        assert!(phase_margin_is_folded(loop_phase_deg));
        let folded = phase_margin_deg(loop_phase_deg);
        assert!(
            (folded - (180.0 + loop_phase_deg + 360.0)).abs() < 1.0e-12,
            "expected one turn of fold, got {folded}"
        );
        assert!(
            (folded - 90.6).abs() < 1.0e-12,
            "the closed form reads +90.6°, got {folded}"
        );
    }

    /// Net phase lead folds the other way. The reported margin is the same
    /// angle, measured the short way round.
    #[test]
    fn a_lead_dominated_crossover_folds_downward() {
        let loop_phase_deg = 61.0;
        assert!(phase_margin_is_folded(loop_phase_deg));
        assert!((phase_margin_deg(loop_phase_deg) - (241.0 - 360.0)).abs() < 1.0e-12);
    }

    /// The turn is half-open at the bottom: −180° and +180° are one angle, and
    /// it is reported as the positive one, so a margin never reads as the
    /// negative edge of a turn it is not on.
    ///
    /// The two boundaries fold differently, and both answers are the physical
    /// ones. `∠L = −360°` is a whole turn of lag: `180° + ∠L` lands on the edge
    /// the turn excludes, the reported value moves from −180° to +180°, and a
    /// loop that has wound a whole turn is exactly what the note exists for.
    /// `∠L = 0°` lands on the edge the turn includes, nothing moves, and an
    /// unwound loop is not announced as folded.
    #[test]
    fn the_reported_turn_is_half_open_at_the_bottom() {
        assert_eq!(phase_margin_deg(-360.0), 180.0);
        assert_eq!(phase_margin_deg(0.0), 180.0);
        assert!(phase_margin_is_folded(-360.0));
        assert!(!phase_margin_is_folded(0.0));
    }

    /// Whatever the fold does to the number, it never invents one.
    #[test]
    fn a_non_finite_phase_stays_non_finite_and_is_never_called_folded() {
        assert!(phase_margin_deg(f64::NAN).is_nan());
        assert!(!phase_margin_is_folded(f64::NAN));
        assert!(!phase_margin_is_folded(f64::NEG_INFINITY));
    }

    /// The note carries the angle, not just the fact of the fold, and points
    /// at what does settle the question.
    #[test]
    fn the_folded_note_names_the_angle_and_the_measurement_that_settles_it() {
        let note = folded_phase_margin_note(-449.4);
        assert!(note.contains("-449°"), "{note}");
        assert!(note.contains("encirclement count"), "{note}");
    }
}
