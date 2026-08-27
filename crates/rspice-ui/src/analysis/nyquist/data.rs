//! The loop-gain locus, and the stability numbers read off it.
//!
//! A retained locus is the *measured branch*: L(jω) over the swept positive
//! frequencies. The Nyquist criterion is stated about the whole contour, so
//! every count here first builds the closed contour — the sweep, its conjugate
//! mirror at negative frequencies, and the chord that closes the two ends —
//! and winds that. Counting the measured branch alone drops the mirror's
//! contribution and never closes, which is not a winding number at all.
//!
//! Three conventions the rest of the viewer depends on:
//!
//! - Encirclements are **clockwise-positive**, the sign the criterion
//!   Z = N + P is written in.
//! - A count is only reported when the sweep's **ends are settled**. Neither
//!   end of the closed contour is measured — both are chords — so a sweep
//!   that still has gain at its top, or is still off the real axis at its
//!   bottom, is wound around the counter's own invention rather than around
//!   the loop.
//! - Margins are read from **interpolated** crossings, never from the nearest
//!   sample. A margin is a ratio, so no absolute "close enough" tolerance can
//!   be right at more than one locus scale.

use std::f64::consts::{PI, TAU};

// =============================================================================
// Nyquist Point
// =============================================================================

/// Single point on Nyquist plot
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NyquistPoint {
    /// Frequency in Hz
    pub frequency: f64,
    /// Real part of loop gain
    pub real: f64,
    /// Imaginary part of loop gain
    pub imag: f64,
}

impl NyquistPoint {
    /// Create new point
    pub fn new(frequency: f64, real: f64, imag: f64) -> Self {
        Self {
            frequency,
            real,
            imag,
        }
    }

    /// Magnitude
    pub fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }

    /// Distance from critical point (-1, 0)
    pub fn distance_from_critical(&self) -> f64 {
        ((self.real + 1.0).powi(2) + self.imag.powi(2)).sqrt()
    }

    fn is_finite(&self) -> bool {
        self.real.is_finite() && self.imag.is_finite()
    }
}

// =============================================================================
// Stability quantities
// =============================================================================

/// The winding of the closed loop-gain contour about −1 + j0.
///
/// Clockwise-positive. A locus that cannot be wound is reported as such
/// instead of being rounded to the nearest integer: a contour that meets the
/// critical point, or one whose turns do not add up to a whole number, has no
/// encirclement count, and saying "0" for it would be a fabricated verdict.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EncirclementCount {
    /// N, clockwise-positive.
    Counted(i32),
    /// The contour passes through −1 + j0. The winding is undefined there,
    /// and the closed loop has a root on the imaginary axis.
    TouchesCriticalPoint,
    /// The winding is not one the contour supports — either the sweep does
    /// not settle at its ends, so the chords that close it are fabricated, or
    /// the closed contour did not wind a whole number of turns. The turns it
    /// did make are carried so the reader can see what was rejected.
    Unresolved { turns: f64 },
    /// The locus carries a non-finite sample.
    NotFinite,
}

impl EncirclementCount {
    /// N, when the contour actually has one.
    #[must_use]
    pub const fn counted(self) -> Option<i32> {
        match self {
            Self::Counted(n) => Some(n),
            _ => None,
        }
    }
}

/// The Nyquist criterion, Z = N + P.
///
/// `N` clockwise encirclements of −1 + j0 by the closed loop-gain contour plus
/// `P` open-loop right-half-plane poles give `Z`, the number of closed-loop
/// right-half-plane poles. The closed loop is stable exactly when Z = 0.
#[must_use]
pub const fn closed_loop_rhp_poles(encirclements: i32, open_loop_rhp_poles: u32) -> i32 {
    encirclements + open_loop_rhp_poles as i32
}

/// One margin, and the frequency its crossing was interpolated to.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NyquistMargin {
    /// Gain ratio for a gain margin, degrees for a phase margin.
    pub value: f64,
    /// The interpolated crossing frequency, in Hz.
    pub frequency: f64,
}

/// |L(jω_max)| below which the chord back to the conjugate mirror stands in
/// for the arc at infinity.
///
/// A tenth is well inside the unit circle, so the locus the sweep did not
/// reach can no longer come near −1 + j0 and the chord cannot enclose it.
const SETTLED_HIGH_FREQUENCY_MAGNITUDE: f64 = 0.1;

/// |Im L(jω_min)| / |L(jω_min)| below which the gap between the mirror and the
/// sweep is a chord *along* the real axis rather than one across it.
///
/// A hundredth is about half a degree off the axis — a single pole a hundred
/// times above the start of the sweep, which is the point past which the DC
/// closure stops depending on where exactly the sweep began.
const SETTLED_LOW_FREQUENCY_AXIS_RATIO: f64 = 1.0e-2;

/// How close two −180° crossings have to be in dB before the gain margin
/// stops preferring either on magnitude and breaks the tie at the unity-gain
/// frequency instead.
///
/// A hundredth of a dB is what the interpolation between sweep samples is
/// good to, so anything inside it is a distinction the measurement does not
/// carry — and picking on it would make the reported crossing a function of
/// the sweep's point density.
const GAIN_MARGIN_TIE_DECIBELS: f64 = 1.0e-2;

// =============================================================================
// Nyquist Data
// =============================================================================

/// Complete Nyquist plot data
#[derive(Debug, Clone, Default)]
pub struct NyquistData {
    /// Name/label
    pub name: String,
    /// Data points (sorted by frequency)
    pub points: Vec<NyquistPoint>,
}

impl NyquistData {
    /// Create from frequency, real, imag arrays
    pub fn from_arrays(name: &str, freq: &[f64], real: &[f64], imag: &[f64]) -> Self {
        let n = freq.len().min(real.len()).min(imag.len());
        let points: Vec<NyquistPoint> = (0..n)
            .map(|i| NyquistPoint::new(freq[i], real[i], imag[i]))
            .collect();

        Self {
            name: name.to_string(),
            points,
        }
    }

    /// Number of points
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Is empty
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Minimum distance from critical point (-1, 0)
    pub fn min_distance_from_critical(&self) -> Option<f64> {
        self.points
            .iter()
            .map(|p| p.distance_from_critical())
            .fold(None, |acc, d| Some(acc.map_or(d, |a: f64| a.min(d))))
    }

    /// Whether the sweep's ends settle the closure the criterion needs.
    ///
    /// Neither end of the closed contour is measured. The chord from ω_max
    /// back to its conjugate mirror stands in for the arc at infinity, and the
    /// gap at ω_min stands in for the contour's crossing of the real axis at
    /// DC — including, for a loop with a pole at the origin, the indentation
    /// around it, which closes through +∞ on the *right* of the critical point
    /// where a straight chord would run down the left of it. Both stand-ins
    /// are only faithful when the sweep has settled into them: |L| well below
    /// unity at the top, so the unmeasured tail can no longer reach −1 + j0 at
    /// all, and the locus back on the real axis at the bottom, so the gap is
    /// a chord *along* the axis rather than one across it.
    ///
    /// The low-frequency test is on the *direction* of L, not its size: a
    /// settled DC gain of either sign, however large, is on the real axis and
    /// closes correctly.
    fn ends_settle_the_closure(&self) -> bool {
        let (Some(start), Some(end)) = (self.points.first(), self.points.last()) else {
            return false;
        };
        end.magnitude() < SETTLED_HIGH_FREQUENCY_MAGNITUDE
            && start.imag.abs() <= SETTLED_LOW_FREQUENCY_AXIS_RATIO * start.magnitude()
    }

    /// The closed Nyquist contour, as the criterion needs it.
    ///
    /// Vertices run from ω = −ω_max up to ω = +ω_max: the conjugate mirror of
    /// the sweep in reverse, then the sweep itself. The list is a cycle — the
    /// chord from the last vertex back to the first closes the contour at the
    /// high-frequency end, which for a strictly proper loop is the sampled
    /// stand-in for the arc at infinity.
    fn closed_contour(&self) -> Vec<(f64, f64)> {
        let mut contour = Vec::with_capacity(self.points.len() * 2);
        contour.extend(self.points.iter().rev().map(|p| (p.real, -p.imag)));
        contour.extend(self.points.iter().map(|p| (p.real, p.imag)));
        contour
    }

    /// Encirclements of −1 + j0 by the closed contour, clockwise-positive.
    ///
    /// Reported only for a sweep whose ends settle the closure — see
    /// [`Self::ends_settle_the_closure`]. One that does not is `Unresolved`
    /// rather than counted, because the number would be the chords' and not
    /// the loop's.
    pub fn count_encirclements(&self) -> EncirclementCount {
        if self.points.iter().any(|point| !point.is_finite()) {
            return EncirclementCount::NotFinite;
        }
        let contour = self.closed_contour();
        if contour.len() < 3 {
            // Fewer than two swept points: the mirror doubles back on itself,
            // enclosing nothing. That is a true zero, not a missing answer.
            return EncirclementCount::Counted(0);
        }

        // A contour that meets the critical point has no winding number, and
        // the tolerance is relative because the locus carries the loop's own
        // scale. This is degeneracy, not a small margin: `min |1 + L|` is
        // what reports a near miss.
        let scale = self
            .points
            .iter()
            .fold(1.0f64, |acc, point| acc.max(point.magnitude()));
        let touch_tolerance = 1e-12 * scale;
        for index in 0..contour.len() {
            let next = contour[(index + 1) % contour.len()];
            if critical_point_distance_to_segment(contour[index], next) <= touch_tolerance {
                return EncirclementCount::TouchesCriticalPoint;
            }
        }

        let turns = clockwise_turns_about_critical_point(&contour);

        // The two chords that close the contour are the counter's own, not the
        // sweep's, so they only carry a winding once the sweep has settled
        // into them. A whole number of turns is no evidence either way here —
        // a closed polygon always winds a whole number of them.
        if !self.ends_settle_the_closure() {
            return EncirclementCount::Unresolved { turns };
        }

        let whole = turns.round();
        if !turns.is_finite() || (turns - whole).abs() > 1e-6 {
            // A closed polygon winds a whole number of turns, so a fraction
            // left over means the contour is not the closed one the criterion
            // needs — the shape of the defect this counter replaced.
            return EncirclementCount::Unresolved { turns };
        }
        EncirclementCount::Counted(whole as i32)
    }

    /// Gain margin at the negative-real-axis crossing nearest unity in dB.
    ///
    /// The crossing is interpolated, not picked from the nearest sample: the
    /// phase is interpolated to ±180° in log-frequency and the magnitude read
    /// there in log-magnitude, which is exact for the straight segments of a
    /// Bode response and close to it everywhere else. The value is the gain
    /// ratio 1/|L| at the crossing, so 1 is the critical point and a panel
    /// reads it out as a signed 20·log₁₀.
    ///
    /// A loop that crosses −180° more than once is limited by one of those
    /// crossings, and the one that binds is the one closest to unity **in
    /// dB** — least |20·log₁₀|L|| — not the one closest to −1 along the axis.
    /// A gain margin is the factor the loop gain may be multiplied by before
    /// the locus reaches −1, so it lives in ratios: linear distance makes
    /// |L| = 0.45 (+6.94 dB of headroom left) look tighter than |L| = 2
    /// (−6.02 dB, already past the critical point), which inverts the
    /// reading. Ties — two crossings the interpolation cannot separate, being
    /// good to about a hundredth of a dB — go to whichever sits nearer the
    /// unity-gain frequency in log-frequency, the crossing the loop is
    /// actually working at.
    ///
    /// This is the rule the Bode card's gain margin is read by too; the two
    /// have to name the same crossing for the same loop.
    pub fn gain_margin(&self) -> Option<NyquistMargin> {
        let branch = self.measured_branch()?;
        let unity_gain = branch.binding_unity_crossing().map(|(at, _)| at);
        let mut best: Option<(f64, f64, NyquistMargin)> = None;
        for index in 0..branch.len() - 1 {
            let Some(crossing) = branch.phase_crossover(index) else {
                continue;
            };
            let magnitude = 10f64.powf(branch.interpolate(&branch.log_magnitude, index, crossing));
            if !(magnitude.is_finite() && magnitude > 0.0) {
                continue;
            }
            let decibels = (20.0 * magnitude.log10()).abs();
            let from_unity_gain = unity_gain.map_or(0.0, |unity| (crossing - unity).abs());
            let binds = best.is_none_or(|(closest, closest_from_unity_gain, _)| {
                if (decibels - closest).abs() <= GAIN_MARGIN_TIE_DECIBELS {
                    from_unity_gain < closest_from_unity_gain
                } else {
                    decibels < closest
                }
            });
            if binds {
                best = Some((
                    decibels,
                    from_unity_gain,
                    NyquistMargin {
                        value: 1.0 / magnitude,
                        frequency: 10f64.powf(crossing),
                    },
                ));
            }
        }
        best.map(|(_, _, margin)| margin)
    }

    /// Phase margin at the unity-magnitude crossing, in degrees.
    ///
    /// The crossing is interpolated in log-magnitude against log-frequency,
    /// and the phase read there from the *unwrapped* phase, so a loop past
    /// −180° reports a negative margin instead of a wrapped positive one. The
    /// angle is measured from the negative real axis: PM = 180° + ∠L.
    ///
    /// A loop that crosses unity more than once reports the crossing that
    /// binds — the smallest margin in magnitude.
    pub fn phase_margin(&self) -> Option<NyquistMargin> {
        let branch = self.measured_branch()?;
        let (crossing, value) = branch.binding_unity_crossing()?;
        Some(NyquistMargin {
            value,
            frequency: 10f64.powf(crossing),
        })
    }

    /// The measured branch, prepared for interpolation.
    fn measured_branch(&self) -> Option<MeasuredBranch> {
        MeasuredBranch::new(&self.points)
    }
}

// =============================================================================
// Geometry
// =============================================================================

/// Total turn of a closed contour about −1 + j0, clockwise-positive.
///
/// Each step contributes the angle it subtends at the critical point, taken on
/// the principal branch — which is exact for a contour sampled finely enough
/// that no step subtends half a turn. Screen angles grow counter-clockwise, so
/// the sum is negated to reach the criterion's sign.
fn clockwise_turns_about_critical_point(contour: &[(f64, f64)]) -> f64 {
    let mut total = 0.0;
    for index in 0..contour.len() {
        let (x0, y0) = contour[index];
        let (x1, y1) = contour[(index + 1) % contour.len()];
        let mut delta = y1.atan2(x1 + 1.0) - y0.atan2(x0 + 1.0);
        while delta > PI {
            delta -= TAU;
        }
        while delta < -PI {
            delta += TAU;
        }
        total += delta;
    }
    -total / TAU
}

/// Distance from −1 + j0 to the segment `a`–`b`.
fn critical_point_distance_to_segment(a: (f64, f64), b: (f64, f64)) -> f64 {
    let (px, py) = (-1.0 - a.0, -a.1);
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let length_squared = dx * dx + dy * dy;
    let t = if length_squared > 0.0 {
        ((px * dx + py * dy) / length_squared).clamp(0.0, 1.0)
    } else {
        0.0
    };
    ((px - t * dx).powi(2) + (py - t * dy).powi(2)).sqrt()
}

/// Wrap an angle in degrees into (−180, 180].
fn wrap_degrees(degrees: f64) -> f64 {
    let mut wrapped = degrees % 360.0;
    if wrapped > 180.0 {
        wrapped -= 360.0;
    }
    if wrapped <= -180.0 {
        wrapped += 360.0;
    }
    wrapped
}

/// The swept branch in the coordinates margins are interpolated in.
///
/// Log-frequency, log-magnitude and unwrapped phase: a Bode response is
/// piecewise straight in exactly those, so linear interpolation between two
/// samples of a decade sweep lands within a hundredth of a dB of the closed
/// form. Unwrapping matters twice over — it keeps a crossing from being
/// interpolated across a ±180° discontinuity, and it is what lets a loop past
/// −180° report a negative phase margin.
struct MeasuredBranch {
    log_frequency: Vec<f64>,
    log_magnitude: Vec<f64>,
    phase: Vec<f64>,
    real: Vec<f64>,
    imag: Vec<f64>,
}

impl MeasuredBranch {
    fn new(points: &[NyquistPoint]) -> Option<Self> {
        let usable: Vec<&NyquistPoint> = points
            .iter()
            .filter(|point| point.is_finite() && point.frequency > 0.0)
            .collect();
        if usable.len() < 2 {
            return None;
        }

        let mut branch = Self {
            log_frequency: Vec::with_capacity(usable.len()),
            log_magnitude: Vec::with_capacity(usable.len()),
            phase: Vec::with_capacity(usable.len()),
            real: Vec::with_capacity(usable.len()),
            imag: Vec::with_capacity(usable.len()),
        };
        let mut previous_raw = 0.0;
        let mut unwrapped = 0.0;
        for (index, point) in usable.iter().enumerate() {
            let raw = point.imag.atan2(point.real);
            if index == 0 {
                unwrapped = raw;
            } else {
                let mut step = raw - previous_raw;
                while step > PI {
                    step -= TAU;
                }
                while step < -PI {
                    step += TAU;
                }
                unwrapped += step;
            }
            previous_raw = raw;
            branch.log_frequency.push(point.frequency.log10());
            branch.log_magnitude.push(point.magnitude().log10());
            branch.phase.push(unwrapped);
            branch.real.push(point.real);
            branch.imag.push(point.imag);
        }
        Some(branch)
    }

    fn len(&self) -> usize {
        self.log_frequency.len()
    }

    /// The log-frequency where step `index` crosses the negative real axis.
    ///
    /// The crossing is solved on the phase rather than on the imaginary part:
    /// the phase is the quantity that is smooth in log-frequency, and solving
    /// it puts the crossing on the exact odd multiple of π the step spans.
    /// Only the negative real axis counts — an imaginary part changing sign
    /// while the locus is in the right half-plane is a 0° crossing, which is
    /// no phase crossover at all.
    fn phase_crossover(&self, index: usize) -> Option<f64> {
        let (imag0, imag1) = (self.imag[index], self.imag[index + 1]);
        if imag0 != 0.0 && imag0 * imag1 >= 0.0 {
            return None;
        }
        if self.real[index] >= 0.0 || self.real[index + 1] >= 0.0 {
            return None;
        }
        let (phase0, phase1) = (self.phase[index], self.phase[index + 1]);
        let half_turns = ((phase0 + phase1) * 0.5 / PI).round();
        if half_turns as i64 % 2 == 0 {
            return None;
        }
        self.solve(&self.phase, index, half_turns * PI)
    }

    /// The binding unity crossing: its log-frequency, and the phase margin
    /// there in degrees.
    ///
    /// The margin is measured from the negative real axis, PM = 180° + ∠L, on
    /// the *unwrapped* phase, so a loop past −180° reports a negative margin
    /// instead of a wrapped positive one. A loop that crosses unity more than
    /// once binds at the smallest margin in magnitude — and that crossing's
    /// frequency is the unity-gain frequency the gain margin's tie-break is
    /// measured against.
    fn binding_unity_crossing(&self) -> Option<(f64, f64)> {
        let mut best: Option<(f64, f64)> = None;
        for index in 0..self.len() - 1 {
            let Some(crossing) = self.unity_crossover(index) else {
                continue;
            };
            let phase = self.interpolate(&self.phase, index, crossing);
            if !phase.is_finite() {
                continue;
            }
            let margin = wrap_degrees(180.0 + phase.to_degrees());
            if best.is_none_or(|(_, current)| margin.abs() < current.abs()) {
                best = Some((crossing, margin));
            }
        }
        best
    }

    /// The log-frequency where step `index` passes through |L| = 1.
    fn unity_crossover(&self, index: usize) -> Option<f64> {
        let (lm0, lm1) = (self.log_magnitude[index], self.log_magnitude[index + 1]);
        if !lm0.is_finite() || !lm1.is_finite() {
            return None;
        }
        if lm0 != 0.0 && lm0 * lm1 >= 0.0 {
            return None;
        }
        self.solve(&self.log_magnitude, index, 0.0)
    }

    /// Solve `values(x) = target` inside step `index`, on the interpolant.
    ///
    /// Bisection rather than a formula because the interpolant is a cubic:
    /// the root is bracketed by the step's own samples, which the interpolant
    /// reproduces exactly, so bisection converges on it to the last bit and
    /// cannot wander outside the step.
    fn solve(&self, values: &[f64], index: usize, target: f64) -> Option<f64> {
        let (mut lo, mut hi) = (self.log_frequency[index], self.log_frequency[index + 1]);
        if !(lo < hi) {
            return None;
        }
        let at = |x: f64| self.interpolate(values, index, x) - target;
        let (low_value, high_value) = (at(lo), at(hi));
        if low_value == 0.0 {
            return Some(lo);
        }
        if high_value == 0.0 {
            return Some(hi);
        }
        if low_value * high_value > 0.0 {
            return None;
        }
        for _ in 0..80 {
            let middle = 0.5 * (lo + hi);
            if at(lo) * at(middle) <= 0.0 {
                hi = middle;
            } else {
                lo = middle;
            }
        }
        Some(0.5 * (lo + hi))
    }

    /// `values` interpolated against log-frequency at `x`.
    ///
    /// Lagrange through up to four samples around step `index`. A decade
    /// sweep is coarse — ten points per decade is normal — and a straight
    /// chord across one of its steps costs a tenth of a dB at the crossing,
    /// which is the whole margin budget. A cubic through the neighbours
    /// costs nothing and lands three orders of magnitude closer.
    fn interpolate(&self, values: &[f64], index: usize, x: f64) -> f64 {
        let (start, end) = self.stencil(index);
        let (nodes, samples) = (&self.log_frequency[start..end], &values[start..end]);
        let mut total = 0.0;
        for (i, (&xi, &yi)) in nodes.iter().zip(samples).enumerate() {
            let mut term = yi;
            for (j, &xj) in nodes.iter().enumerate() {
                if i != j {
                    term *= (x - xj) / (xi - xj);
                }
            }
            total += term;
        }
        total
    }

    /// Up to four samples around step `index`, always containing the step.
    fn stencil(&self, index: usize) -> (usize, usize) {
        let start = index.saturating_sub(1);
        let end = (start + 4).min(self.len());
        (end.saturating_sub(4).min(index), end)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex64;

    /// A log-spaced sweep of a closed-form loop gain, retained the way an STB
    /// run retains its contour: positive frequencies only, ascending.
    fn swept_loop(
        f_start: f64,
        f_stop: f64,
        points_per_decade: usize,
        loop_gain: impl Fn(Complex64) -> Complex64,
    ) -> NyquistData {
        let decades = (f_stop / f_start).log10();
        let count = (decades * points_per_decade as f64).round() as usize + 1;
        let mut frequency = Vec::with_capacity(count);
        let mut real = Vec::with_capacity(count);
        let mut imag = Vec::with_capacity(count);
        for index in 0..count {
            let f = f_start * 10f64.powf(decades * index as f64 / (count - 1) as f64);
            let value = loop_gain(Complex64::new(0.0, std::f64::consts::TAU * f));
            frequency.push(f);
            real.push(value.re);
            imag.push(value.im);
        }
        NyquistData::from_arrays("L(jw)", &frequency, &real, &imag)
    }

    /// L(s) = k / ((1 + s/w1)(1 + s/w2)) — two left-half-plane poles.
    fn two_pole(k: f64, f1: f64, f2: f64) -> impl Fn(Complex64) -> Complex64 {
        let (w1, w2) = (std::f64::consts::TAU * f1, std::f64::consts::TAU * f2);
        move |s| k / ((1.0 + s / w1) * (1.0 + s / w2))
    }

    /// L(s) = k / (1 + s/w)³ — the textbook conditionally stable loop.
    fn three_pole(k: f64, f_pole: f64) -> impl Fn(Complex64) -> Complex64 {
        let w = std::f64::consts::TAU * f_pole;
        move |s| k / (1.0 + s / w).powi(3)
    }

    /// L(s) = k / (s/w − 1) — one open-loop right-half-plane pole, P = 1.
    fn right_half_plane_pole(k: f64, f_pole: f64) -> impl Fn(Complex64) -> Complex64 {
        let w = std::f64::consts::TAU * f_pole;
        move |s| k / (s / w - 1.0)
    }

    /// L(s) = K / (s(1 + s/w)) — a type-1 loop, with K/w = `dc_slope`.
    ///
    /// The pole at the origin is what the sweep cannot measure: Re L(jω→0)
    /// tends to −K/w while Im L runs off to −∞, so the locus leaves the plot
    /// down the line Re = −K/w instead of returning to the real axis.
    fn integrator_with_pole(dc_slope: f64, f_pole: f64) -> impl Fn(Complex64) -> Complex64 {
        let w = std::f64::consts::TAU * f_pole;
        let k = dc_slope * w;
        move |s| k / (s * (1.0 + s / w))
    }

    /// L(s) = g(1 + s/10w)² / (1 + s/w)³ — the conditionally stable shape,
    /// with two −180° crossings and a phase that recovers above it in between.
    fn conditionally_stable(gain: f64, f_pole: f64) -> impl Fn(Complex64) -> Complex64 {
        let w = std::f64::consts::TAU * f_pole;
        move |s| gain * (1.0 + s / (10.0 * w)).powi(2) / (1.0 + s / w).powi(3)
    }

    /// Both −180° crossings of [`conditionally_stable`], from the closed form:
    /// the frequency in u = f/f_pole, and the magnitude there at unit gain.
    ///
    /// The phase 2·atan(u/10) − 3·atan(u) reaches −180° at u² = 8 and u² = 35
    /// exactly, where |L|/g is 1.08/27 and 1.35/216. The bisection below is
    /// that closed form solved rather than its two roots asserted, so the
    /// fixture stays honest if the shape is ever retuned.
    fn conditional_crossings() -> [(f64, f64); 2] {
        let phase = |u: f64| 2.0 * (u / 10.0).atan() - 3.0 * u.atan() + PI;
        let magnitude = |u: f64| (1.0 + (u / 10.0).powi(2)) / (1.0 + u * u).powf(1.5);
        let solve = |mut lo: f64, mut hi: f64| {
            for _ in 0..200 {
                let mid = (lo * hi).sqrt();
                if phase(lo) * phase(mid) <= 0.0 {
                    hi = mid;
                } else {
                    lo = mid;
                }
            }
            (lo * hi).sqrt()
        };
        [solve(1.0, 4.0), solve(4.0, 20.0)].map(|u| (u, magnitude(u)))
    }

    // -- encirclements ----------------------------------------------------

    /// A two-pole loop never reaches −180°, so its locus cannot enclose the
    /// critical point: N = 0, and the closed loop is stable for every gain
    /// (the closed-form denominator keeps both roots in the left half-plane).
    #[test]
    fn a_two_pole_loop_encircles_nothing() {
        let curve = swept_loop(1.0, 1.0e7, 20, two_pole(10.0, 1.0e3, 1.0e5));

        assert_eq!(curve.count_encirclements(), EncirclementCount::Counted(0));
        assert_eq!(closed_loop_rhp_poles(0, 0), 0);
    }

    /// L = 16/(1 + s/w)³. Routh on (s/w + 1)³ + k puts two roots in the right
    /// half-plane for k > 8, so the criterion Z = N + P with P = 0 demands
    /// exactly two clockwise encirclements.
    #[test]
    fn a_three_pole_loop_above_its_routh_limit_encircles_twice_clockwise() {
        let curve = swept_loop(1.0, 1.0e6, 20, three_pole(16.0, 1.0e3));

        assert_eq!(curve.count_encirclements(), EncirclementCount::Counted(2));
        // Two clockwise turns with no open-loop right-half-plane pole is a
        // closed loop with two of them — exactly what Routh reports.
        assert_eq!(closed_loop_rhp_poles(2, 0), 2);
    }

    /// The same loop below the Routh limit closes stable: N = 0.
    #[test]
    fn a_three_pole_loop_below_its_routh_limit_encircles_nothing() {
        let curve = swept_loop(1.0, 1.0e6, 20, three_pole(4.0, 1.0e3));

        assert_eq!(curve.count_encirclements(), EncirclementCount::Counted(0));
        assert_eq!(closed_loop_rhp_poles(0, 0), 0);
    }

    /// L = k/(s/w − 1) has one open-loop right-half-plane pole. The closed
    /// loop's root is at s = w(1 − k), so k > 1 is stable — which the
    /// criterion can only report as N = −P = −1, one *counter*-clockwise
    /// encirclement. A clockwise/counter-clockwise sign inversion turns this
    /// stable loop into an unstable one.
    #[test]
    fn a_stabilized_right_half_plane_pole_encircles_once_counter_clockwise() {
        let curve = swept_loop(1.0e-2, 1.0e6, 20, right_half_plane_pole(2.0, 1.0e3));

        assert_eq!(curve.count_encirclements(), EncirclementCount::Counted(-1));
        assert_eq!(closed_loop_rhp_poles(-1, 1), 0);
    }

    /// The same loop with too little gain leaves its right-half-plane pole
    /// uncorrected: no encirclement, so Z = N + P = 1 and the closed loop
    /// keeps the open-loop pole it never moved.
    #[test]
    fn an_uncorrected_right_half_plane_pole_encircles_nothing() {
        let curve = swept_loop(1.0e-2, 1.0e6, 20, right_half_plane_pole(0.5, 1.0e3));

        assert_eq!(curve.count_encirclements(), EncirclementCount::Counted(0));
        assert_eq!(closed_loop_rhp_poles(0, 1), 1);
    }

    /// A locus running straight through −1 + j0 has no winding number, and a
    /// counter that rounds one out of it reports a verdict the contour does
    /// not support. Here the loop gain is exactly −1 at one swept frequency:
    /// the closed loop has a root on the imaginary axis.
    #[test]
    fn a_locus_through_the_critical_point_is_reported_rather_than_rounded() {
        let curve = NyquistData::from_arrays(
            "L(jw)",
            &[1.0, 2.0, 3.0, 4.0],
            &[0.5, -0.4, -1.0, -0.2],
            &[0.8, 0.5, 0.0, -0.3],
        );

        assert_eq!(
            curve.count_encirclements(),
            EncirclementCount::TouchesCriticalPoint
        );
        assert_eq!(curve.count_encirclements().counted(), None);
    }

    /// A locus that grazes the critical point without meeting it still has a
    /// winding number; the near miss is reported by `min |1 + L|`, which is
    /// the quantity that measures it. The sweep is carried down to the real
    /// axis and out past the unit circle so the closure guard has the settled
    /// ends it needs — the near miss is what is on trial here, not the
    /// closure.
    #[test]
    fn a_near_miss_keeps_its_winding_and_shows_up_as_a_small_distance() {
        let curve = NyquistData::from_arrays(
            "L(jw)",
            &[1.0, 2.0, 3.0, 4.0, 5.0],
            &[1.5, 0.5, -0.4, -1.0, -0.02],
            &[0.0, 0.8, 0.5, 1.0e-4, -0.01],
        );

        assert!(matches!(
            curve.count_encirclements(),
            EncirclementCount::Counted(_)
        ));
        let distance = curve
            .min_distance_from_critical()
            .expect("a populated locus has a nearest approach");
        assert!(distance < 2.0e-4, "nearest approach {distance}");
    }

    /// A non-finite sample is reported, never silently wound around.
    #[test]
    fn a_non_finite_sample_has_no_encirclement_count() {
        let curve = NyquistData::from_arrays(
            "L(jw)",
            &[1.0, 2.0, 3.0],
            &[0.5, f64::NAN, -1.5],
            &[0.8, 0.5, 0.2],
        );

        assert_eq!(curve.count_encirclements(), EncirclementCount::NotFinite);
    }

    /// The contour the criterion is applied to is the closed one: the sweep,
    /// its conjugate mirror, and the chord between the ends. Winding the
    /// measured branch alone is what produced a wrong count for every locus
    /// with an odd symmetric contribution.
    #[test]
    fn the_counted_contour_is_the_closed_mirrored_one() {
        let curve = swept_loop(1.0, 1.0e6, 5, three_pole(16.0, 1.0e3));
        let contour = curve.closed_contour();

        assert_eq!(contour.len(), curve.len() * 2);
        for (mirrored, swept) in contour.iter().zip(curve.points.iter().rev()) {
            assert_eq!(mirrored.0, swept.real);
            assert_eq!(mirrored.1, -swept.imag);
        }

        // And the mirror is load-bearing: winding the measured branch on its
        // own does not give the criterion's count.
        let branch: Vec<(f64, f64)> = curve.points.iter().map(|p| (p.real, p.imag)).collect();
        let branch_turns = clockwise_turns_about_critical_point(&branch);
        let closed_turns = clockwise_turns_about_critical_point(&contour);
        assert!((closed_turns - 2.0).abs() < 1.0e-9, "{closed_turns}");
        assert!(
            (branch_turns - 2.0).abs() > 0.5,
            "the measured branch alone gave {branch_turns} turns"
        );
    }

    // -- closure trust ----------------------------------------------------

    /// A type-1 loop's low-frequency closure is a fabrication, and the count
    /// drawn from it is one too.
    ///
    /// L = K/(s(1 + s/ω_p)) closes as s² + ω_p·s + K·ω_p = 0 — every
    /// coefficient positive, so both roots sit in the left half-plane for
    /// every gain. With no open-loop right-half-plane pole either, the truth
    /// is P = 0 and Z = 0, so N = 0. But Re L(jω→0) → −K/ω_p = −10, and the
    /// chord across the gap between the mirror and the sweep therefore
    /// crosses the real axis at −10, *left* of the critical point — where the
    /// contour the criterion is stated about closes through +∞ on the right
    /// of it, around the indentation at the pole at the origin. The chord's
    /// winding is not the loop's, and the instrument has to say so rather
    /// than print a stable loop as unstable.
    #[test]
    fn a_type_one_loop_leaves_its_low_frequency_closure_uncertified() {
        let curve = swept_loop(1.0, 1.0e6, 20, integrator_with_pole(10.0, 1.0e3));

        let start = curve.points.first().expect("the sweep has points");
        assert!(
            start.real < -1.0,
            "the fabricated chord has to cross left of −1: Re = {}",
            start.real
        );
        assert!(
            matches!(
                curve.count_encirclements(),
                EncirclementCount::Unresolved { .. }
            ),
            "{:?}",
            curve.count_encirclements()
        );
    }

    /// A sweep that stops while the loop still has gain has no closure at its
    /// top end either. The k = 16 three-pole loop that winds twice over a full
    /// sweep is truncated at 500 Hz, where |L| is still about 11: the arc at
    /// infinity has not been reached, so the chord back to the conjugate
    /// mirror encloses whatever the truncation happened to leave — here
    /// nothing, which would read as an unconditionally stable loop.
    #[test]
    fn a_sweep_truncated_above_unity_gain_leaves_its_winding_uncertified() {
        let truncated = swept_loop(1.0, 500.0, 20, three_pole(16.0, 1.0e3));

        let end = truncated.points.last().expect("the sweep has points");
        assert!(end.magnitude() > 1.0, "|L(w_max)| = {}", end.magnitude());
        assert!(
            matches!(
                truncated.count_encirclements(),
                EncirclementCount::Unresolved { .. }
            ),
            "{:?}",
            truncated.count_encirclements()
        );

        // And the count the truncation would have reported is not the truth
        // the same loop reports once the sweep reaches the arc at infinity.
        assert_eq!(
            swept_loop(1.0, 1.0e6, 20, three_pole(16.0, 1.0e3)).count_encirclements(),
            EncirclementCount::Counted(2)
        );
    }

    /// The closure guard must not swallow the loci that do settle. Both
    /// oracles here end far below unity and start on the real axis, so their
    /// chords are the ones the criterion's contour actually has — and their
    /// counts, one of each sign, have to survive untouched.
    #[test]
    fn settled_sweeps_keep_their_counts_under_the_closure_guard() {
        let unstable = swept_loop(1.0, 1.0e6, 20, three_pole(16.0, 1.0e3));
        let stabilized = swept_loop(1.0e-2, 1.0e6, 20, right_half_plane_pole(2.0, 1.0e3));

        for curve in [&unstable, &stabilized] {
            let start = curve.points.first().expect("the sweep has points");
            let end = curve.points.last().expect("the sweep has points");
            assert!(end.magnitude() < 0.1, "|L(w_max)| = {}", end.magnitude());
            assert!(
                start.imag.abs() < 1.0e-2 * start.magnitude(),
                "|Im/L| at w_min = {}",
                start.imag.abs() / start.magnitude()
            );
        }

        assert_eq!(
            unstable.count_encirclements(),
            EncirclementCount::Counted(2)
        );
        assert_eq!(
            stabilized.count_encirclements(),
            EncirclementCount::Counted(-1)
        );
    }

    // -- margins ----------------------------------------------------------

    /// GM is read at the negative-real-axis crossing nearest −1. For
    /// L = k/(1 + s/w)³ the phase reaches −180° at ω = √3·w, where |L| = k/8,
    /// so GM = 8/k exactly — here −6.02 dB, an unstable loop.
    #[test]
    fn gain_margin_matches_the_closed_form_phase_crossover() {
        let k = 16.0;
        let curve = swept_loop(1.0, 1.0e6, 10, three_pole(k, 1.0e3));

        let gain_margin = curve.gain_margin().expect("the locus crosses −180°");
        let closed_form_db = 20.0 * (8.0 / k).log10();
        let measured_db = 20.0 * gain_margin.value.log10();
        assert!(
            (measured_db - closed_form_db).abs() < 0.01,
            "gain margin {measured_db:.4} dB vs closed form {closed_form_db:.4} dB"
        );
        // The crossing frequency is the closed form too: ω = √3·w.
        let closed_form_frequency = 3f64.sqrt() * 1.0e3;
        assert!(
            (gain_margin.frequency / closed_form_frequency - 1.0).abs() < 0.01,
            "crossing at {} Hz vs closed form {closed_form_frequency} Hz",
            gain_margin.frequency
        );
    }

    /// PM is read at the unity-magnitude crossing. |L| = 1 at
    /// ω = w·√(k^(2/3) − 1), where the phase is −3·atan(ω/w) — past −180°
    /// here, so the margin is negative and must not wrap to a positive one.
    #[test]
    fn phase_margin_matches_the_closed_form_unity_crossover() {
        let k = 16.0;
        let curve = swept_loop(1.0, 1.0e6, 10, three_pole(k, 1.0e3));

        let phase_margin = curve.phase_margin().expect("the locus crosses |L| = 1");
        let u = (k.powf(2.0 / 3.0) - 1.0).sqrt();
        let closed_form = 180.0 - 3.0 * u.atan().to_degrees();
        assert!(closed_form < 0.0, "the oracle must be an unstable loop");
        assert!(
            (phase_margin.value - closed_form).abs() < 0.05,
            "phase margin {:.4}° vs closed form {closed_form:.4}°",
            phase_margin.value
        );
        let closed_form_frequency = u * 1.0e3;
        assert!(
            (phase_margin.frequency / closed_form_frequency - 1.0).abs() < 0.01,
            "crossing at {} Hz vs closed form {closed_form_frequency} Hz",
            phase_margin.frequency
        );
    }

    /// A two-pole loop's phase approaches −180° only as ω → ∞, so it has no
    /// phase crossover at all and no gain margin to report.
    #[test]
    fn a_two_pole_loop_reports_no_gain_margin() {
        let curve = swept_loop(1.0, 1.0e7, 20, two_pole(10.0, 1.0e3, 1.0e5));

        assert_eq!(curve.gain_margin(), None);
    }

    /// And its phase margin is the closed form at the unity crossing.
    #[test]
    fn a_two_pole_loop_phase_margin_matches_the_closed_form() {
        let (k, f1, f2) = (10.0, 1.0e3, 1.0e5);
        let curve = swept_loop(1.0, 1.0e7, 10, two_pole(k, f1, f2));

        // Unity crossing of the closed form, bisected to machine precision.
        let magnitude = |f: f64| {
            let (u1, u2) = (f / f1, f / f2);
            k / ((1.0 + u1 * u1) * (1.0 + u2 * u2)).sqrt()
        };
        let (mut lo, mut hi) = (1.0f64, 1.0e7f64);
        for _ in 0..200 {
            let mid = (lo * hi).sqrt();
            if magnitude(mid) > 1.0 {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        let crossover = (lo * hi).sqrt();
        let closed_form =
            180.0 - (crossover / f1).atan().to_degrees() - (crossover / f2).atan().to_degrees();

        let phase_margin = curve.phase_margin().expect("the locus crosses |L| = 1");
        assert!(
            (phase_margin.value - closed_form).abs() < 0.05,
            "phase margin {:.4}° vs closed form {closed_form:.4}°",
            phase_margin.value
        );
    }

    /// The margins must not depend on how large the retained numbers are. A
    /// loop measured in a unit where the locus runs to 10⁵ has exactly the
    /// same margins as the same loop at unit scale, because a margin is a
    /// ratio. An absolute "close enough to the axis" tolerance does not.
    #[test]
    fn margins_are_free_of_the_locus_scale() {
        let k = 1.0e5;
        let curve = swept_loop(1.0, 1.0e7, 10, three_pole(k, 1.0e3));

        let gain_margin = curve.gain_margin().expect("the locus crosses −180°");
        let closed_form_db = 20.0 * (8.0 / k).log10();
        let measured_db = 20.0 * gain_margin.value.log10();
        assert!(
            (measured_db - closed_form_db).abs() < 0.01,
            "gain margin {measured_db:.4} dB vs closed form {closed_form_db:.4} dB"
        );

        // The same loop at unit scale reports the same margin in dB.
        let unit_scale = swept_loop(1.0, 1.0e7, 10, three_pole(16.0, 1.0e3));
        let unit_margin = unit_scale
            .gain_margin()
            .expect("the locus crosses −180°")
            .value;
        assert!(
            (20.0 * unit_margin.log10() - 20.0 * (8.0f64 / 16.0).log10()).abs() < 0.01,
            "unit-scale gain margin {unit_margin}"
        );
    }

    /// The binding −180° crossing is the one nearest unity **in dB**, which is
    /// not the one nearest −1 along the axis.
    ///
    /// The conditionally stable shape crosses −180° twice, at u² = 8 and
    /// u² = 35, where the unit-gain magnitude is 1.08/27 and 1.35/216. A gain
    /// of 50 puts those at |L| = 2 and |L| = 0.3125: the sub-unity crossing is
    /// nearer −1 by linear distance (0.6875 against 1.0), the super-unity one
    /// is nearer unity in dB (6.02 dB against 10.10 dB). A gain margin is a
    /// ratio, so dB is the metric that means anything — and here the two
    /// disagree by 16 dB and by the sign of the answer.
    #[test]
    fn gain_margin_binds_at_the_crossing_nearest_unity_in_decibels() {
        let (gain, f_pole) = (50.0, 1.0e3);
        let curve = swept_loop(1.0e2, 1.0e6, 40, conditionally_stable(gain, f_pole));

        let [(u_above, unit_above), (_, unit_below)] = conditional_crossings();
        let (above, below) = (gain * unit_above, gain * unit_below);
        assert!(above > 1.0 && below < 1.0, "{above} and {below}");
        assert!(
            (below - 1.0).abs() < (above - 1.0).abs(),
            "the linear metric has to prefer the sub-unity crossing: {above}, {below}"
        );
        assert!(
            above.log10().abs() < below.log10().abs(),
            "the dB metric has to prefer the super-unity crossing: {above}, {below}"
        );

        let gain_margin = curve.gain_margin().expect("the locus crosses −180° twice");
        let measured_db = 20.0 * gain_margin.value.log10();
        let closed_form_db = -20.0 * above.log10();
        assert!(
            (measured_db - closed_form_db).abs() < 0.05,
            "gain margin {measured_db:.4} dB vs closed form {closed_form_db:.4} dB \
             (the sub-unity crossing reads {:.4} dB)",
            -20.0 * below.log10()
        );
        let closed_form_frequency = u_above * f_pole;
        assert!(
            (gain_margin.frequency / closed_form_frequency - 1.0).abs() < 0.01,
            "crossing at {} Hz vs closed form {closed_form_frequency} Hz",
            gain_margin.frequency
        );
    }

    /// Two crossings the sweep cannot separate in dB are told apart by which
    /// one sits nearer the unity-gain frequency, not by sweep order.
    ///
    /// The gain here is solved for from the closed form so that the sub-unity
    /// crossing wins on magnitude by 0.005 dB — inside the hundredth of a dB
    /// the interpolation is good to, so the two are tied as far as this
    /// instrument can see. The super-unity crossing is the one nearer the UGF
    /// in log-frequency, so it is the one that binds.
    #[test]
    fn a_tie_in_decibels_is_broken_at_the_unity_gain_frequency() {
        let f_pole = 1.0e3;
        let [(u_above, unit_above), (u_below, unit_below)] = conditional_crossings();
        // |20·log₁₀(g·above)| − |20·log₁₀(g·below)| = 40·log₁₀ g + 20·log₁₀(above·below).
        let bias_db = 0.005;
        let gain = 10f64.powf((bias_db - 20.0 * (unit_above * unit_below).log10()) / 40.0);
        let (above, below) = (gain * unit_above, gain * unit_below);
        assert!(
            ((20.0 * above.log10()) - (-20.0 * below.log10()) - bias_db).abs() < 1.0e-9,
            "the fixture has to be a tie the sub-unity crossing wins: {above}, {below}"
        );

        let curve = swept_loop(1.0e2, 1.0e6, 60, conditionally_stable(gain, f_pole));
        let unity = curve
            .phase_margin()
            .expect("the locus crosses |L| = 1")
            .frequency;
        let from_unity_gain = |u: f64| ((u * f_pole).log10() - unity.log10()).abs();
        assert!(
            from_unity_gain(u_above) < from_unity_gain(u_below),
            "the super-unity crossing has to be the one nearer the UGF: {} vs {}",
            from_unity_gain(u_above),
            from_unity_gain(u_below)
        );

        let gain_margin = curve.gain_margin().expect("the locus crosses −180° twice");
        let measured_db = 20.0 * gain_margin.value.log10();
        let closed_form_db = -20.0 * above.log10();
        assert!(
            (measured_db - closed_form_db).abs() < 0.05,
            "gain margin {measured_db:.4} dB vs closed form {closed_form_db:.4} dB \
             (the sub-unity crossing reads {:.4} dB)",
            -20.0 * below.log10()
        );
    }
}
