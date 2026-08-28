//! The two stability cards, read off one loop and checked against each other.
//!
//! Margins are measured twice in this application, by two implementations that
//! share no code:
//!
//! - the Bode card, [`crate::state::ac_bode_summary_for_analysis`],
//!   which unwraps the retained `(-180°, 180°]` phase trace and locates every
//!   crossing by linear interpolation in log-frequency;
//! - the Nyquist card, [`super::data::NyquistData`], which works on the complex
//!   locus and solves for each crossing by bisection on a Lagrange cubic over
//!   `(log f, log|L|, unwrapped ∠L)`.
//!
//! Both are ratified to report the same convention: the gain margin is the
//! −180° crossing with the **smallest |dB| distance to unity** — the smallest
//! gain change that reaches instability, which is the crossing MATLAB's
//! `margin` names — reported signed; the phase margin is the unity-gain
//! crossing with the smallest `|PM|`, also signed. Each card was aligned to
//! that convention separately. Nothing else pins them against *each other*,
//! and a convention holds only where two independent readings of it agree.
//!
//! Every fixture here is one closed-form loop transmission `L(j2πf)`, sampled
//! once. The samples become a magnitude/phase pair for the Bode card and a
//! real/imaginary pair for the Nyquist card, exactly as
//! `populate_ac_post_views` splits one measured complex response between the
//! two sheets. So the cards are not merely given equivalent data: they are
//! given the same numbers.
//!
//! Each margin is asserted three ways — Bode against the closed form, Nyquist
//! against the closed form, and the two cards against each other. The closed
//! form is solved on the continuous `L` over a grid two orders of magnitude
//! denser than the sweep, and shares no code with either card. Checking both
//! cards against it is what makes a *common* drift fail: two cards that agreed
//! on a wrong number would still miss the analytic one.

use std::f64::consts::TAU;

use num_complex::Complex64;

use crate::state::{AnalysisResult, AnalysisType, WaveformData, ac_bode_summary_for_analysis};

use super::data::NyquistData;

// -----------------------------------------------------------------------
// Agreement budget
// -----------------------------------------------------------------------

/// How far apart two readings of one margin may sit, in dB.
///
/// The two cards are not equally accurate, and the budget belongs to the
/// weaker one. Nyquist solves on a Lagrange cubic and lands within 4e-6 dB of
/// the closed form on every fixture here — better than its own quoted 0.002 dB.
/// The Bode card reads the same samples with a straight chord, and that error
/// dominates: 3.0e-3 dB at 40 points per decade, 5.0e-5 dB at 250. The budget
/// is the coarsest of those with room to spare, so the sweep density can be
/// varied without re-tuning it.
///
/// It is also three orders of magnitude below what this module exists to
/// catch. The conditionally stable fixture's binding and worst-case crossings
/// are 147 dB apart; a card that named the wrong one misses by 10⁵ budgets.
const GAIN_MARGIN_AGREEMENT_DB: f64 = 0.02;

/// How far apart two readings of one margin may sit, in degrees.
///
/// The same asymmetry: Nyquist lands within 8e-6° of the closed form, the
/// Bode card within 9.3e-3° at 40 points per decade. Nothing here is a
/// tolerance on the *convention* — a card binding at the wrong unity-gain
/// crossing misses by 95° on the resonant fixture.
const PHASE_MARGIN_AGREEMENT_DEG: f64 = 0.05;

/// How far apart two readings of one crossing *frequency* may sit, relative.
///
/// A crossing frequency is only ever as well determined as the curve's slope
/// there: a margin located on a shallow segment moves further for the same dB
/// of interpolation error than one located on a steep one. The worst case
/// measured here is 9.7e-5 — the phase margin of the conditionally stable loop
/// swept at 40 points per decade, read off a phase turning 100° per decade.
const CROSSING_FREQUENCY_AGREEMENT: f64 = 1.0e-3;

/// Points per decade the closed-form grid is scanned on.
///
/// It has two jobs: bracket every crossing, and keep consecutive phase samples
/// under half a turn apart so the reference branch is unambiguous. Two
/// thousand is far denser than any sweep here, so a crossing the oracle finds
/// is a crossing of `L` and not of a polygon through it.
const ORACLE_POINTS_PER_DECADE: usize = 2_000;

// -----------------------------------------------------------------------
// Fixtures
// -----------------------------------------------------------------------

/// One located crossing: where it is, and the signed margin read there.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Crossing {
    frequency: f64,
    /// dB for a gain margin, degrees for a phase margin. A phase margin is
    /// the folded value both cards report, not the raw `180° + ∠L`.
    value: f64,
    /// Unwrapped `∠L` at the crossing, where the reading carries it: what a
    /// phase margin was folded from, and which inversion a gain margin sits
    /// on. `None` where a card does not report it — the Bode card's gain
    /// margin names its crossing by frequency alone.
    phase_deg: Option<f64>,
}

/// A closed-form loop transmission, sampled once for both cards.
struct LoopFixture<L: Fn(f64) -> Complex64> {
    transmission: L,
    frequency: Vec<f64>,
    value: Vec<Complex64>,
    /// `log10 f` of the closed-form grid.
    grid: Vec<f64>,
    /// Continuous phase in degrees on that grid, anchored at its first point.
    grid_phase_deg: Vec<f64>,
}

/// `f0 .. f1` sampled `points_per_decade` times per decade, endpoints exact.
fn log_sweep(f0: f64, f1: f64, points_per_decade: usize) -> Vec<f64> {
    let (l0, l1) = (f0.log10(), f1.log10());
    let steps = ((l1 - l0) * points_per_decade as f64).round() as usize;
    (0..=steps)
        .map(|i| 10f64.powf(l0 + (l1 - l0) * i as f64 / steps as f64))
        .collect()
}

/// The phase an AC result retains: `imag.atan2(real)` in degrees, folded into
/// a single turn. This is the trace the Bode card has to see through, and the
/// angle the Nyquist card recovers from the same two components.
fn retained_phase_deg(value: Complex64) -> f64 {
    value.im.atan2(value.re).to_degrees()
}

/// `reference`'s branch of `L`'s phase at `f`.
///
/// The wrapped angle plus whichever whole turn puts it nearest `reference`.
/// Exact while the true phase stays within half a turn of the reference, which
/// the oracle grid's spacing guarantees.
fn continued_phase_deg(value: Complex64, reference: f64) -> f64 {
    let wrapped = retained_phase_deg(value);
    wrapped + 360.0 * ((reference - wrapped) / 360.0).round()
}

/// Bisect `probe` in log-frequency over a bracket it changes sign across.
fn bisect_log_frequency(lo: f64, hi: f64, probe: impl Fn(f64) -> f64) -> f64 {
    let (mut lo, mut hi) = (lo, hi);
    let at_lo = probe(lo);
    for _ in 0..120 {
        let middle = 0.5 * (lo + hi);
        if at_lo * probe(middle) <= 0.0 {
            hi = middle;
        } else {
            lo = middle;
        }
    }
    0.5 * (lo + hi)
}

/// Odd multiples of 180° strictly inside `[p0, p1]` — the phases at which a
/// locus meets the negative real axis.
fn negative_real_axis_levels(p0: f64, p1: f64) -> Vec<f64> {
    let (lo, hi) = if p0 <= p1 { (p0, p1) } else { (p1, p0) };
    // 180·(2m+1) lies in [lo, hi] exactly when m does.
    let first = ((lo / 180.0 - 1.0) / 2.0).ceil();
    let last = ((hi / 180.0 - 1.0) / 2.0).floor();
    let mut out = Vec::new();
    let mut m = first;
    while m <= last {
        out.push(180.0 * (2.0 * m + 1.0));
        m += 1.0;
    }
    out
}

impl<L: Fn(f64) -> Complex64> LoopFixture<L> {
    fn new(transmission: L, f_start: f64, f_stop: f64, points_per_decade: usize) -> Self {
        let frequency = log_sweep(f_start, f_stop, points_per_decade);
        let value: Vec<Complex64> = frequency.iter().map(|&f| transmission(f)).collect();

        let grid: Vec<f64> = log_sweep(f_start, f_stop, ORACLE_POINTS_PER_DECADE)
            .into_iter()
            .map(f64::log10)
            .collect();
        let mut grid_phase_deg = Vec::with_capacity(grid.len());
        let mut previous = retained_phase_deg(transmission(10f64.powf(grid[0])));
        grid_phase_deg.push(previous);
        for &u in &grid[1..] {
            previous = continued_phase_deg(transmission(10f64.powf(u)), previous);
            grid_phase_deg.push(previous);
        }

        Self {
            transmission,
            frequency,
            value,
            grid,
            grid_phase_deg,
        }
    }

    fn gain_db(&self, f: f64) -> f64 {
        20.0 * (self.transmission)(f).norm().log10()
    }

    /// The sweep, retained the way `results_convert` retains an AC response:
    /// linear magnitude under `|…|`, and a `(-180°, 180°]`-wrapped phase.
    fn bode_summary(&self) -> (Option<Crossing>, Option<Crossing>) {
        let magnitude: Vec<f64> = self.value.iter().map(|value| value.norm()).collect();
        let phase: Vec<f64> = self.value.iter().copied().map(retained_phase_deg).collect();
        let analysis = AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![
            WaveformData::new("|V(out)|", self.frequency.clone(), magnitude, "#fff"),
            WaveformData::new("phase(V(out))", self.frequency.clone(), phase, "#fff"),
        ]);
        let metrics = ac_bode_summary_for_analysis(&analysis, 0)
            .expect("the fixture is a frequency response")
            .metrics;
        let phase_margin = metrics.pm_deg.map(|value| Crossing {
            frequency: metrics.ugf.expect("a phase margin names its crossing"),
            value,
            phase_deg: metrics.pm_phase_deg,
        });
        let gain_margin = metrics.gm_db.map(|value| Crossing {
            frequency: metrics.f180.expect("a gain margin names its crossing"),
            value,
            phase_deg: None,
        });
        (phase_margin, gain_margin)
    }

    /// The same samples as a locus: real and imaginary parts against frequency.
    fn nyquist_margins(&self) -> (Option<Crossing>, Option<Crossing>) {
        let real: Vec<f64> = self.value.iter().map(|value| value.re).collect();
        let imag: Vec<f64> = self.value.iter().map(|value| value.im).collect();
        let curve = NyquistData::from_arrays("L(jω)", &self.frequency, &real, &imag);
        let phase_margin = curve.phase_margin().map(|margin| Crossing {
            frequency: margin.frequency,
            // The card carries a gain *ratio* for the gain margin and degrees
            // for the phase margin; only the former needs converting.
            value: margin.value,
            phase_deg: Some(margin.phase_deg),
        });
        let gain_margin = curve.gain_margin().map(|margin| Crossing {
            frequency: margin.frequency,
            value: 20.0 * margin.value.log10(),
            phase_deg: Some(margin.phase_deg),
        });
        (phase_margin, gain_margin)
    }

    /// Every unity-gain crossing of the continuous loop, with the phase margin
    /// there — folded into one turn, which is the ratified convention both
    /// cards report and select on — and the unwrapped `∠L` it was folded from.
    fn closed_form_unity_crossings(&self) -> Vec<Crossing> {
        let mut out = Vec::new();
        for index in 1..self.grid.len() {
            let (u0, u1) = (self.grid[index - 1], self.grid[index]);
            let (g0, g1) = (self.gain_db(10f64.powf(u0)), self.gain_db(10f64.powf(u1)));
            if g0 == 0.0 || g0 * g1 >= 0.0 {
                continue;
            }
            let reference = self.grid_phase_deg[index - 1];
            let u = bisect_log_frequency(u0, u1, |u| self.gain_db(10f64.powf(u)));
            let frequency = 10f64.powf(u);
            let loop_phase = continued_phase_deg((self.transmission)(frequency), reference);
            out.push(Crossing {
                frequency,
                value: crate::results::stability::phase_margin_deg(loop_phase),
                phase_deg: Some(loop_phase),
            });
        }
        out
    }

    /// Every negative-real-axis crossing, with the signed gain margin there
    /// and the inversion — `-180° - 360k` — it sits on.
    fn closed_form_axis_crossings(&self) -> Vec<Crossing> {
        let mut out = Vec::new();
        for index in 1..self.grid.len() {
            let (u0, u1) = (self.grid[index - 1], self.grid[index]);
            let (p0, p1) = (self.grid_phase_deg[index - 1], self.grid_phase_deg[index]);
            for level in negative_real_axis_levels(p0, p1) {
                let u = bisect_log_frequency(u0, u1, |u| {
                    continued_phase_deg((self.transmission)(10f64.powf(u)), p0) - level
                });
                let frequency = 10f64.powf(u);
                out.push(Crossing {
                    frequency,
                    value: -self.gain_db(frequency),
                    phase_deg: Some(level),
                });
            }
        }
        out
    }

    /// The crossing that binds: the smallest margin in magnitude.
    fn binding(crossings: &[Crossing]) -> Option<Crossing> {
        crossings
            .iter()
            .copied()
            .min_by(|a, b| a.value.abs().total_cmp(&b.value.abs()))
    }

    /// The sweep resolves the loop's phase: no two samples are half a turn
    /// apart, so both cards' unwrapping recovers the same branch the closed
    /// form is on. Without this the fixture, not the code, would be deciding
    /// what the margins are.
    #[track_caller]
    fn assert_sweep_resolves_the_phase(&self) {
        let mut previous = self.grid_phase_deg[0];
        for &f in &self.frequency[1..] {
            let continued = continued_phase_deg((self.transmission)(f), previous);
            assert!(
                (continued - previous).abs() < 180.0,
                "the sweep steps {}° at {f} Hz, so its branch is its own invention",
                continued - previous
            );
            previous = continued;
        }
    }

}

// -----------------------------------------------------------------------
// The three-way check
// -----------------------------------------------------------------------

#[track_caller]
fn assert_close(actual: f64, expected: f64, tolerance: f64, what: &str) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "{what}: {actual} vs {expected}, apart by {:e} (tolerance {tolerance})",
        (actual - expected).abs()
    );
}

#[track_caller]
fn assert_relative(actual: f64, expected: f64, tolerance: f64, what: &str) {
    let error = (actual - expected).abs() / expected.abs();
    assert!(
        error <= tolerance,
        "{what}: {actual} vs {expected}, relative error {error:e} (tolerance {tolerance})"
    );
}

/// One margin, read three ways: closed form, Bode card, Nyquist card.
///
/// Absence is a reading too. A margin the loop does not have must be absent
/// from both cards; one card inventing it, or the other losing one the loop
/// really has, fails here.
#[track_caller]
fn assert_one_margin(
    what: &str,
    closed_form: Option<Crossing>,
    bode: Option<Crossing>,
    nyquist: Option<Crossing>,
    tolerance: f64,
) {
    let (Some(closed_form), Some(bode), Some(nyquist)) = (closed_form, bode, nyquist) else {
        assert!(
            closed_form.is_none() && bode.is_none() && nyquist.is_none(),
            "{what}: the loop has {closed_form:?}, Bode reports {bode:?}, Nyquist reports \
             {nyquist:?} — a margin has to be present on both cards or on neither"
        );
        return;
    };

    // Against the closed form, so that a shared drift fails as loudly as a
    // divergence.
    assert_close(
        bode.value,
        closed_form.value,
        tolerance,
        &format!("{what}: Bode against the closed form"),
    );
    assert_close(
        nyquist.value,
        closed_form.value,
        tolerance,
        &format!("{what}: Nyquist against the closed form"),
    );
    assert_relative(
        bode.frequency,
        closed_form.frequency,
        CROSSING_FREQUENCY_AGREEMENT,
        &format!("{what} frequency: Bode against the closed form"),
    );
    assert_relative(
        nyquist.frequency,
        closed_form.frequency,
        CROSSING_FREQUENCY_AGREEMENT,
        &format!("{what} frequency: Nyquist against the closed form"),
    );

    // And against each other, which is the claim this module exists for.
    assert_close(
        bode.value,
        nyquist.value,
        tolerance,
        &format!("{what}: the two cards"),
    );
    assert_relative(
        bode.frequency,
        nyquist.frequency,
        CROSSING_FREQUENCY_AGREEMENT,
        &format!("{what} frequency: the two cards"),
    );

    // The angle behind the number. A phase margin is folded into one turn, so
    // two cards can print the same margin off different angles a turn apart —
    // which is exactly how they diverged before the fold was shared. Checking
    // the fold's *input* is what makes the agreement above mean the loops
    // were read the same way rather than reduced to the same residue.
    for (reading, against, label) in [
        (bode.phase_deg, closed_form.phase_deg, "Bode / closed form"),
        (
            nyquist.phase_deg,
            closed_form.phase_deg,
            "Nyquist / closed form",
        ),
        (bode.phase_deg, nyquist.phase_deg, "the two cards"),
    ] {
        let (Some(reading), Some(against)) = (reading, against) else {
            continue;
        };
        assert_close(
            reading,
            against,
            PHASE_MARGIN_AGREEMENT_DEG,
            &format!("{what}: ∠L at the crossing, {label}"),
        );
    }
}

#[track_caller]
fn assert_both_cards_agree(fixture: &LoopFixture<impl Fn(f64) -> Complex64>) {
    fixture.assert_sweep_resolves_the_phase();

    let (bode_phase, bode_gain) = fixture.bode_summary();
    let (nyquist_phase, nyquist_gain) = fixture.nyquist_margins();

    assert_one_margin(
        "phase margin",
        LoopFixture::<fn(f64) -> Complex64>::binding(&fixture.closed_form_unity_crossings()),
        bode_phase,
        nyquist_phase,
        PHASE_MARGIN_AGREEMENT_DEG,
    );
    assert_one_margin(
        "gain margin",
        LoopFixture::<fn(f64) -> Complex64>::binding(&fixture.closed_form_axis_crossings()),
        bode_gain,
        nyquist_gain,
        GAIN_MARGIN_AGREEMENT_DB,
    );
}

// -----------------------------------------------------------------------
// Loop transmissions
// -----------------------------------------------------------------------

/// `10^(decibels/20)` as a complex scalar.
fn from_decibels(decibels: f64) -> Complex64 {
    Complex64::new(10f64.powf(decibels / 20.0), 0.0)
}

/// `1 + jf/f_pole`, one real pole at `f_pole` Hz.
fn pole(f: f64, f_pole: f64) -> Complex64 {
    Complex64::new(1.0, f / f_pole)
}

/// `1 − (f/f0)² + j·2ζ(f/f0)`, a complex pole pair at `f0` with damping `zeta`.
fn resonance(f: f64, f0: f64, zeta: f64) -> Complex64 {
    let r = f / f0;
    Complex64::new(1.0 - r * r, 2.0 * zeta * r)
}

/// `L(s) = 10³ / ((1 + s/ω₁₀)(1 + s/ω₁₀ₖ)(1 + s/ω₁₀₀ₖ))`.
///
/// The plain case: one unity-gain crossing, one −180° crossing, both margins
/// unambiguous. Nothing here needs a convention to be resolved, which is what
/// makes it the control.
fn three_pole_rolloff() -> impl Fn(f64) -> Complex64 {
    |f| from_decibels(60.0) / (pole(f, 10.0) * pole(f, 1.0e4) * pole(f, 1.0e5))
}

/// `L(s) = 10^7.5·(1 + s/ω₁ₖ)² / ((1 + s/ω₁₀)³(1 + s/ω₁₀₀ₖ)²)`.
///
/// The case the convention was ratified for. Three poles take the phase to
/// −270°, a pair of zeros brings it back to −90°, and two more poles take it
/// down again: the phase crosses −180° three times, and the loop is stable
/// only because the gain is still above unity at the first two.
///
/// The three crossings carry gain margins tens of dB apart, so "binding" and
/// "worst case" are different crossings by a margin no interpolation error can
/// blur. Reading the deepest one calls a healthy loop catastrophically
/// unstable; that is the pre-alignment divergence, and it is what this fixture
/// would have caught.
fn conditionally_stable() -> impl Fn(f64) -> Complex64 {
    |f| {
        let zeros = pole(f, 1.0e3) * pole(f, 1.0e3);
        let poles = pole(f, 10.0) * pole(f, 10.0) * pole(f, 10.0);
        from_decibels(150.0) * zeros / (poles * pole(f, 1.0e5) * pole(f, 1.0e5))
    }
}

/// `L(s) = 10^2.45 / ((1 + s/ω₁₀₀)(1 − (s/ω₁₀₀ₖ)² + 0.2 s/ω₁₀₀ₖ))`.
///
/// A lightly damped pole pair lifts the magnitude back over unity after the
/// dominant pole has taken it below: the gain crosses 0 dB three times. The
/// binding phase margin is the *last* of the three and is negative, so a card
/// that named the first crossing would report a comfortable +86° for a loop
/// that is already 36° past instability.
fn resonant_multiple_unity_crossings() -> impl Fn(f64) -> Complex64 {
    |f| from_decibels(49.0) / (pole(f, 100.0) * resonance(f, 1.0e5, 0.1))
}

/// `L(s) = 10² · e^(−100µs·s) / (1 + s/ω₁₀₀)`.
///
/// A transport delay does not stop contributing lag at −180°: it keeps
/// winding, a full turn every `1/T`. By the time this loop's gain reaches
/// unity the phase has passed −449°, so `180° + ∠L` is −269° — outside the
/// turn a phase margin is reported in.
///
/// This is the everyday shape for it. Every loop closed over a link with
/// latency — a digitally controlled supply, a plant behind an ADC, a probe on
/// the far side of a cable — has one, and nothing about it is exotic enough
/// for a card to be allowed to disagree with the other card about it.
fn delay_dominated() -> impl Fn(f64) -> Complex64 {
    |f| {
        let delay = Complex64::new(0.0, -TAU * f * 1.0e-4).exp();
        from_decibels(40.0) * delay / pole(f, 100.0)
    }
}

/// `L(s) = 10⁴ / ((1 + s/ω₁₀)(1 + s/ω₂₀ₖ))`.
///
/// Two poles can contribute at most 180° of lag and reach it only at infinite
/// frequency, so this loop has no −180° crossing and therefore no gain margin
/// at all. Both cards have to say so. A card that reports a number here has
/// invented one, and the two agreeing on `None` is as much a claim about the
/// convention as agreeing on a value.
fn two_pole_without_a_gain_margin() -> impl Fn(f64) -> Complex64 {
    |f| from_decibels(80.0) / (pole(f, 10.0) * pole(f, 2.0e4))
}

// -----------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------

/// The loop whose phase has wound past a full turn by crossover.
///
/// Before the fold was shared this fixture split the two cards by exactly one
/// turn. The closed form puts `∠L` at −449.409° where the gain reaches unity;
/// the Bode card reported `180° + ∠L` unbounded as −269.409°, the Nyquist card
/// wrapped the same angle to +90.591°, and the Bode number is the one
/// `console.rs` publishes as the `phase_margin` measurement a `SpecEntry`
/// binds to. A `phase_margin ≥ 45°` requirement therefore failed a loop the
/// other sheet showed 90° of margin on.
///
/// Both cards now fold, both select the binding crossing on the folded value,
/// and both carry the angle they folded from so the sheet can say the number
/// is a fold. Agreement on the fold alone would not be worth much — two cards
/// can print the same residue off angles a turn apart — so `∠L` itself is
/// checked too.
#[test]
fn a_wound_loop_folds_to_the_same_margin_on_both_cards() {
    let fixture = LoopFixture::new(delay_dominated(), 1.0e2, 1.0e5, 400);

    let unity = fixture.closed_form_unity_crossings();
    assert_eq!(
        unity.len(),
        1,
        "the delay fixture must cross unity exactly once: {unity:?}"
    );
    let loop_phase = unity[0].phase_deg.expect("the oracle carries the angle");
    assert!(
        loop_phase < -360.0,
        "the fixture must actually wind: ∠L is {loop_phase}° at crossover"
    );
    assert!(
        crate::results::stability::phase_margin_is_folded(loop_phase),
        "a wound loop's margin has to be reported as folded"
    );
    // The fold is one whole turn, and it moves the reading from unstable-
    // looking to healthy-looking. That is the whole reason the sheet has to
    // say the fold happened.
    assert_close(
        unity[0].value,
        180.0 + loop_phase + 360.0,
        1.0e-9,
        "the fold is exactly one turn",
    );
    assert!(
        unity[0].value > 0.0 && 180.0 + loop_phase < -180.0,
        "the fixture must be one where the fold changes the verdict: folded {}°, unfolded {}°",
        unity[0].value,
        180.0 + loop_phase
    );

    // A delay keeps inverting, so there are many −180° crossings to bind
    // among — the gain margin has to pick the same one on both cards too.
    assert!(
        fixture.closed_form_axis_crossings().len() > 5,
        "a delay winds through −180° repeatedly"
    );

    assert_both_cards_agree(&fixture);
}

/// An ordinary loop is not reported as folded.
///
/// A note that fires on every response says nothing. The three-pole control
/// crosses unity at −132°, well inside the turn, and neither card may claim
/// its margin was folded.
#[test]
fn an_unwound_loop_is_not_reported_as_folded() {
    let fixture = LoopFixture::new(three_pole_rolloff(), 1.0, 1.0e7, 100);
    let unity = fixture.closed_form_unity_crossings();
    let loop_phase = unity[0].phase_deg.expect("the oracle carries the angle");
    assert!(
        loop_phase > -360.0 && loop_phase < 0.0,
        "the control loop must not wind: ∠L is {loop_phase}°"
    );
    assert!(!crate::results::stability::phase_margin_is_folded(
        loop_phase
    ));

    let (bode_phase, _) = fixture.bode_summary();
    let (nyquist_phase, _) = fixture.nyquist_margins();
    for (card, reading) in [("Bode", bode_phase), ("Nyquist", nyquist_phase)] {
        let phase = reading
            .expect("a phase margin")
            .phase_deg
            .expect("the card carries the angle");
        assert!(
            !crate::results::stability::phase_margin_is_folded(phase),
            "the {card} card called an ordinary margin folded (∠L = {phase}°)"
        );
    }
}

/// Two −180° crossings the sweep cannot tell apart in dB.
///
/// This is the fixture `a_tie_in_decibels_is_broken_at_the_unity_gain_frequency`
/// uses: `L = g(1 + s/10ω_p)² / (1 + s/ω_p)³`, whose inversions sit at
/// `(f/f_p)² = 8` and `35`, with `g` solved so the sub-unity one wins on
/// `|GM|` by 0.005 dB — a fifth of what the interpolation carries.
///
/// The two crossings are 8.06 dB either side of unity: one is that much
/// headroom, the other that much past the critical point. Deciding between
/// them on 0.005 dB makes the *sign* of the reported margin a function of the
/// sweep's point density, so both cards break the tie at the unity-gain
/// frequency instead. Before the tie band was shared, the Bode card compared
/// the two magnitudes strictly and named the other crossing: +8.058 dB at
/// 5915.9 Hz against the Nyquist card's −8.064 dB at 2828.4 Hz — opposite
/// verdicts, a factor of 2.1 apart in frequency, on one set of samples.
#[test]
fn a_dead_heat_between_two_inversions_binds_the_same_way_on_both_cards() {
    // g such that |20·log₁₀(g·above)| − |20·log₁₀(g·below)| = −0.005 dB.
    let f_pole = 1.0e3;
    let unit_above: f64 = 1.08 / 27.0;
    let unit_below: f64 = 1.35 / 216.0;
    let bias_db: f64 = 0.005;
    let gain = 10f64.powf((bias_db - 20.0 * (unit_above * unit_below).log10()) / 40.0);
    let fixture = LoopFixture::new(
        move |f: f64| {
            let zero = Complex64::new(1.0, f / (10.0 * f_pole));
            let pole = Complex64::new(1.0, f / f_pole);
            Complex64::new(gain, 0.0) * zero * zero / (pole * pole * pole)
        },
        1.0e2,
        1.0e6,
        200,
    );

    let crossings = fixture.closed_form_axis_crossings();
    assert_eq!(crossings.len(), 2, "the fixture inverts twice: {crossings:?}");
    let separation = (crossings[0].value.abs() - crossings[1].value.abs()).abs();
    assert!(
        separation <= crate::results::stability::GAIN_MARGIN_TIE_DECIBELS,
        "the fixture has to be a tie: {separation} dB apart in |GM|"
    );
    assert!(
        crossings[0].value * crossings[1].value < 0.0,
        "the two crossings must sit either side of unity, so naming the wrong one \
         inverts the sign: {crossings:?}"
    );

    // Which one binds is settled by the closed form, not by either card: the
    // unity-gain frequency comes from the continuous loop, and the crossing
    // nearer it in log-frequency is the one both cards must name.
    let unity = fixture.closed_form_unity_crossings();
    assert_eq!(unity.len(), 1, "one unity-gain crossing: {unity:?}");
    let from_unity = |crossing: &Crossing| {
        (crossing.frequency.log10() - unity[0].frequency.log10()).abs()
    };
    let binding = if from_unity(&crossings[0]) < from_unity(&crossings[1]) {
        crossings[0]
    } else {
        crossings[1]
    };

    let (_, bode_gain) = fixture.bode_summary();
    let (_, nyquist_gain) = fixture.nyquist_margins();
    for (card, reading) in [("Bode", bode_gain), ("Nyquist", nyquist_gain)] {
        let reading = reading.expect("the locus crosses −180° twice");
        assert_close(
            reading.value,
            binding.value,
            GAIN_MARGIN_AGREEMENT_DB,
            &format!("{card}: gain margin at the tie"),
        );
        assert_relative(
            reading.frequency,
            binding.frequency,
            CROSSING_FREQUENCY_AGREEMENT,
            &format!("{card}: the crossing named at the tie"),
        );
    }
}

#[test]
fn a_three_pole_rolloff_reads_the_same_margins_on_both_cards() {
    let fixture = LoopFixture::new(three_pole_rolloff(), 1.0, 1.0e7, 100);

    assert_eq!(
        fixture.closed_form_unity_crossings().len(),
        1,
        "the control fixture must have exactly one unity-gain crossing"
    );
    assert_eq!(
        fixture.closed_form_axis_crossings().len(),
        1,
        "the control fixture must have exactly one −180° crossing"
    );

    assert_both_cards_agree(&fixture);
}

#[test]
fn a_conditionally_stable_loop_binds_at_the_same_crossing_on_both_cards() {
    let fixture = LoopFixture::new(conditionally_stable(), 1.0, 1.0e7, 100);

    let crossings = fixture.closed_form_axis_crossings();
    assert_eq!(
        crossings.len(),
        3,
        "the fixture must invert three times: {crossings:?}"
    );
    let binding = LoopFixture::<fn(f64) -> Complex64>::binding(&crossings)
        .expect("three crossings have a binding one");
    let worst_case = crossings
        .iter()
        .copied()
        .min_by(|a, b| a.value.total_cmp(&b.value))
        .expect("three crossings have a deepest one");
    assert_ne!(
        binding.frequency, worst_case.frequency,
        "the fixture must separate the binding crossing from the worst-case one"
    );
    assert!(
        binding.value > 0.0 && worst_case.value < 0.0,
        "the loop must be stable at the binding crossing ({} dB) and already past the \
         worst-case one ({} dB) — that is what conditional stability is",
        binding.value,
        worst_case.value
    );
    assert!(
        binding.value - worst_case.value > 20.0,
        "the two conventions must be separated by more than any interpolation error: \
         {} dB apart",
        binding.value - worst_case.value
    );

    assert_both_cards_agree(&fixture);
}

#[test]
fn multiple_unity_gain_crossings_bind_at_the_same_crossing_on_both_cards() {
    let fixture = LoopFixture::new(resonant_multiple_unity_crossings(), 1.0e2, 1.0e7, 1_000);

    let crossings = fixture.closed_form_unity_crossings();
    assert_eq!(
        crossings.len(),
        3,
        "the fixture must cross unity three times: {crossings:?}"
    );
    let binding = LoopFixture::<fn(f64) -> Complex64>::binding(&crossings)
        .expect("three crossings have a binding one");
    assert_ne!(
        binding.frequency, crossings[0].frequency,
        "the fixture must separate the binding crossing from the first one"
    );
    assert!(
        binding.value < 0.0 && crossings[0].value > 0.0,
        "the binding crossing must be unstable ({}°) while the first reads comfortable \
         ({}°), so naming the wrong one inverts the verdict",
        binding.value,
        crossings[0].value
    );

    assert_both_cards_agree(&fixture);
}

#[test]
fn a_loop_with_no_gain_margin_reports_none_on_both_cards() {
    let fixture = LoopFixture::new(two_pole_without_a_gain_margin(), 1.0, 1.0e7, 100);

    assert!(
        fixture.closed_form_axis_crossings().is_empty(),
        "two poles never reach −180°"
    );
    assert_eq!(
        fixture.closed_form_unity_crossings().len(),
        1,
        "the loop still has a phase margin to agree about"
    );

    let (_, bode_gain) = fixture.bode_summary();
    let (_, nyquist_gain) = fixture.nyquist_margins();
    assert_eq!(bode_gain, None, "the Bode card invented a gain margin");
    assert_eq!(nyquist_gain, None, "the Nyquist card invented a gain margin");

    assert_both_cards_agree(&fixture);
}

/// The frequency axis is not what either card measures in.
///
/// Both locate crossings in log-frequency, so the same loop swept over a
/// different band with a different point density has to produce the same
/// margins. This is the fixture's own credentials: a margin that moved with
/// the sweep would mean the numbers above describe the sweep and not the loop.
#[test]
fn both_cards_read_the_same_loop_off_two_different_sweeps() {
    let coarse = LoopFixture::new(conditionally_stable(), 1.0, 1.0e7, 40);
    let fine = LoopFixture::new(conditionally_stable(), 0.1, 1.0e8, 250);

    for fixture in [&coarse, &fine] {
        assert_both_cards_agree(fixture);
    }

    let (coarse_phase, coarse_gain) = coarse.bode_summary();
    let (fine_phase, fine_gain) = fine.bode_summary();
    assert_close(
        coarse_phase.expect("phase margin").value,
        fine_phase.expect("phase margin").value,
        PHASE_MARGIN_AGREEMENT_DEG,
        "phase margin across sweeps",
    );
    assert_close(
        coarse_gain.expect("gain margin").value,
        fine_gain.expect("gain margin").value,
        GAIN_MARGIN_AGREEMENT_DB,
        "gain margin across sweeps",
    );
}
