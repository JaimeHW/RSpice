//! Exact frequency-response and Bode margin mathematics over retained numeric slices.
//!
//! Trace selection and run lookup remain with the application adapter.

/// Stability numbers derived from one magnitude/phase pair.
///
/// # Conventions
///
/// Phase is unwrapped before any margin is measured — see
/// `unwrap_retained_phase_deg` — and the unwrapped branch is anchored at the
/// lowest swept frequency. Both crossing searches and both reads of a curve at
/// a located crossing interpolate in log-frequency, so a margin never depends
/// on which measure the reader happens to think in.
///
/// The instability phases are `-180° - 360k` for `k >= 0`: the lag-sense
/// inversions a loop transmission reaches as it rolls off. Phase margin is the
/// textbook `180° + ∠L(f_ugf)` against the first of them, folded into
/// `(-180°, 180°]` by [`crate::stability::phase_margin_deg`] — the
/// turn MATLAB, ADS and Spectre all report a margin in, and the turn the
/// Nyquist card reports in. When the fold moved the number,
/// [`AcBodeMetrics::pm_phase_deg`] carries the angle it was folded from and
/// the card says so beside the margin.
///
/// Either curve can reach its level more than once. Both margins then name the
/// crossing that *binds* — the one a perturbation reaches first, `min |GM_dB|`
/// for the gain margin and `min |PM|` for the phase margin — and both report
/// that crossing's signed value, so a margin already crossed still reads
/// negative. `ugf` and `f180` name the crossings their own margins were read
/// at. This is the one convention both stability cards are ratified to report
/// under, so the Bode and Nyquist rows can never name different crossings.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct AcBodeMetrics {
    /// Gain at the lowest swept frequency. It is the DC gain only when
    /// [`AcBodeMetrics::adc_is_dc`] is set.
    pub adc_db: Option<f64>,
    /// Whether the sweep provably starts below every pole, so `adc_db` may be
    /// presented as the DC gain rather than as `A(f_min)`.
    pub adc_is_dc: bool,
    /// Frequency of the unity-gain crossing the phase margin was measured at.
    pub ugf: Option<f64>,
    /// Binding phase margin: the smallest `|PM|` over every unity-gain
    /// crossing in the swept band, reported signed and folded into one turn.
    pub pm_deg: Option<f64>,
    /// Unwrapped `∠L` at the crossing [`AcBodeMetrics::pm_deg`] was read at.
    ///
    /// The margin is folded into `(-180°, 180°]`; this is what it was folded
    /// from. They differ exactly when the loop's phase left that turn before
    /// crossover — a wound loop, or one with net lead — and a folded margin
    /// reads healthy on a loop that is not, so the card has to say which case
    /// it is in. [`crate::stability::phase_margin_is_folded`] asks
    /// that question of this value.
    pub pm_phase_deg: Option<f64>,
    /// Frequency of the phase inversion the gain margin was measured at.
    pub f180: Option<f64>,
    /// Binding gain margin: the smallest `|GM_dB|` over every phase inversion
    /// in the swept band, reported signed.
    pub gm_db: Option<f64>,
    pub f3db: Option<f64>,
    pub gain_extremes: (f64, f64),
    /// Extremes of the *retained* phase trace, which is what the plot paints
    /// unless the reader asks for the continuous branch. Margins do not read
    /// this.
    pub phase_extremes: Option<(f64, f64)>,
}

/// The first frequency at which `series` reaches `level`.
///
/// Bandwidth edges are the first crossing by definition, and a curve with no
/// phase to weigh the later ones against has nothing better to name.
pub fn log_frequency_crossing(frequency: &[f64], series: &[f64], level: f64) -> Option<f64> {
    log_frequency_crossings(frequency, series, level)
        .into_iter()
        .next()
}

/// Every frequency in the swept band at which `series` reaches `level`, in
/// sweep order.
///
/// A level touched exactly at a retained sample is reported once, under the
/// same ownership rule the phase inversions use: each segment owns its leading
/// endpoint, and the final segment additionally owns its trailing one.
fn log_frequency_crossings(frequency: &[f64], series: &[f64], level: f64) -> Vec<f64> {
    let n = frequency.len().min(series.len());
    let mut out = Vec::new();
    for i in 1..n {
        let (f0, f1) = (frequency[i - 1], frequency[i]);
        if f0 <= 0.0 || f1 <= 0.0 {
            continue;
        }
        let (y0, y1) = (series[i - 1] - level, series[i] - level);
        if y0 == 0.0 {
            out.push(f0);
        } else if y1 == 0.0 {
            if i == n - 1 {
                out.push(f1);
            }
        } else if y0 * y1 < 0.0 {
            out.push(interpolate_log_frequency(f0, f1, y0 / (y0 - y1)));
        }
    }
    out
}

/// The frequency a fraction `t` of the way from `f0` to `f1` measured in
/// decades. Exact at both ends, so a crossing that lands on a retained sample
/// reports that sample's own frequency rather than a re-exponentiated copy.
fn interpolate_log_frequency(f0: f64, f1: f64, t: f64) -> f64 {
    if t <= 0.0 {
        return f0;
    }
    if t >= 1.0 {
        return f1;
    }
    let (l0, l1) = (f0.log10(), f1.log10());
    10f64.powf(l0 + t * (l1 - l0))
}

/// Continuous phase from a retained `(-180°, 180°]` trace.
///
/// Retained AC phase is `imag.atan2(real)`, which folds every branch into a
/// single turn: a response passing -180° does not cross it, it jumps to
/// +180°. A margin is an angle, not a position relative to that fold, so every
/// margin in this module is measured on the unwrapped series.
///
/// The branch is anchored at the lowest swept frequency, whose sample is
/// carried through exactly as retained. A response whose true phase has
/// already passed -180° before its first sample cannot be anchored from the
/// swept data alone; sweeping from below the dominant pole resolves it.
///
/// Non-finite samples pass through and are skipped when measuring jumps, so a
/// gap in the trace does not poison the running branch offset.
fn unwrap_retained_phase_deg(phase: &[f64]) -> Vec<f64> {
    let mut out = Vec::with_capacity(phase.len());
    let mut offset = 0.0_f64;
    let mut previous: Option<f64> = None;
    for &sample in phase {
        if !sample.is_finite() {
            out.push(sample);
            continue;
        }
        if let Some(previous) = previous {
            let jump = sample - previous;
            if jump.abs() > 180.0 {
                offset -= 360.0 * (jump / 360.0).round();
            }
        }
        previous = Some(sample);
        out.push(sample + offset);
    }
    out
}

/// `log10` of the frequency axis, which is the abscissa every interpolation
/// here works in. A non-positive frequency — never produced by an AC sweep —
/// maps below every real sample so it can never bracket a query, and the
/// crossing searches skip its segments outright.
fn log_frequency_axis(frequency: &[f64]) -> Vec<f64> {
    frequency
        .iter()
        .map(|&f| {
            if f > 0.0 {
                f.log10()
            } else {
                f64::NEG_INFINITY
            }
        })
        .collect()
}

/// Read `series` at `frequency` interpolating in log-frequency — the same
/// measure the crossing searches use to locate that frequency.
///
/// Sampling a decade-swept curve linearly in frequency is a large error at
/// realistic sweep densities: on a `-50°/decade` phase read at a crossing
/// three-tenths of a decade into a decade-wide interval it is nearly 13°.
fn sample_at_log_frequency(log_frequency: &[f64], series: &[f64], frequency: f64) -> f64 {
    crate::sampling::sample_at(log_frequency, series, frequency.log10())
}

/// Phases at which a loop transmission inverts, in the lag sense a rolling-off
/// response reaches them: -180°, -540°, -900°, …
fn instability_phases_between(p0: f64, p1: f64) -> impl Iterator<Item = f64> {
    let (lo, hi) = if p0 <= p1 { (p0, p1) } else { (p1, p0) };
    // `-180 - 360k` lies in `[lo, hi]` exactly when `k` does. Only `k >= 0`
    // is reachable by a response anchored inside one turn and rolling off.
    let k_first = ((-hi - 180.0) / 360.0).ceil().max(0.0);
    let k_last = ((-lo - 180.0) / 360.0).floor();
    let count = if k_last >= k_first {
        (k_last - k_first) as usize + 1
    } else {
        0
    };
    (0..count).map(move |i| -180.0 - 360.0 * (k_first + i as f64))
}

/// Every frequency in the swept band at which the unwrapped phase reaches an
/// instability phase.
///
/// A level touched exactly at a retained sample is reported once: each segment
/// owns its leading endpoint, and the final segment additionally owns its
/// trailing one.
fn phase_inversion_frequencies(frequency: &[f64], phase_unwrapped: &[f64]) -> Vec<f64> {
    let n = frequency.len().min(phase_unwrapped.len());
    let mut out = Vec::new();
    for i in 1..n {
        let (f0, f1) = (frequency[i - 1], frequency[i]);
        if f0 <= 0.0 || f1 <= 0.0 {
            continue;
        }
        let (p0, p1) = (phase_unwrapped[i - 1], phase_unwrapped[i]);
        if !p0.is_finite() || !p1.is_finite() {
            continue;
        }
        let owns_trailing_endpoint = i == n - 1;
        for level in instability_phases_between(p0, p1) {
            if level == p1 && !owns_trailing_endpoint {
                continue;
            }
            out.push(if p0 == p1 {
                f0
            } else {
                interpolate_log_frequency(f0, f1, (level - p0) / (p1 - p0))
            });
        }
    }
    out
}

/// The gain margin the reader has to act on, and the phase inversion it was
/// measured at.
///
/// A response can invert more than once — every conditionally stable loop dips
/// through -180° above 0 dB and comes back. Reporting the first inversion would
/// hide the one that matters, and reporting the deepest one over-warns by the
/// whole conditionally stable hump: a healthy loop with 2 dB in hand reads as
/// 26.7 dB past the margin. The inversion that *binds* is the one nearest unity
/// gain in log magnitude — `min |GM_dB|`, the smallest gain change that reaches
/// instability, which is the perturbation a gain error actually applies, and
/// the crossing MATLAB's `margin` names. The reported value keeps its sign, so
/// a negative gain margin still says the loop is already past that crossing.
/// Ties — two inversions the sweep cannot separate, being good to about
/// [`crate::stability::GAIN_MARGIN_TIE_DECIBELS`] — go to the
/// inversion nearest the unity-gain frequency in log-frequency, which is the
/// one the loop is working at.
///
/// The tie band is not a nicety. Two inversions can sit the same distance
/// either side of unity, one with headroom and one already past the critical
/// point; separating them on a hundredth of that distance makes the *sign* of
/// the reported margin a function of the sweep's point density. The Nyquist
/// card applies the same band, sequentially over the same crossing order, so
/// the two cards resolve such a dead heat to the same inversion — see
/// `crate::analysis::nyquist`'s agreement module.
fn binding_gain_margin(
    frequency: &[f64],
    log_frequency: &[f64],
    gain_db: &[f64],
    phase_unwrapped: &[f64],
    ugf: Option<f64>,
) -> Option<(f64, f64)> {
    let reference = ugf.filter(|f| *f > 0.0).map(f64::log10);
    let distance = |f: f64| reference.map_or(f.log10(), |reference| (f.log10() - reference).abs());
    phase_inversion_frequencies(frequency, phase_unwrapped)
        .into_iter()
        .map(|f180| (f180, -sample_at_log_frequency(log_frequency, gain_db, f180)))
        .reduce(|best, candidate| {
            let binds = if (candidate.1.abs() - best.1.abs()).abs()
                <= crate::stability::GAIN_MARGIN_TIE_DECIBELS
            {
                distance(candidate.0) < distance(best.0)
            } else {
                candidate.1.abs() < best.1.abs()
            };
            if binds { candidate } else { best }
        })
}

/// The phase margin the reader has to act on, and the unity-gain crossing it
/// was measured at.
///
/// Gain reaches 0 dB more than once whenever the response has a resonance to
/// climb back over, so the same question the gain margin answers arises here:
/// which crossing binds. It is the one with the smallest `|PM|` — the same
/// distance-to-instability rule as the gain margin, measured in phase instead
/// of in log magnitude — and the reported sign is retained. Ties go to the
/// lowest crossing. `ugf` names the crossing the margin was read at, so the
/// card's two rows always describe the same point on the curve.
///
/// The margin is folded into `(-180°, 180°]` before it is either compared or
/// reported. Folding only the reported value would let this card select a
/// crossing on one quantity and print another, which is how it and the
/// Nyquist card came to name different unity-gain frequencies on a loop whose
/// phase had wound. The third element is the unwrapped angle the fold was
/// applied to, which is what a wound loop has to be reported *with*.
fn binding_phase_margin(
    frequency: &[f64],
    log_frequency: &[f64],
    gain_db: &[f64],
    phase_unwrapped: &[f64],
) -> Option<(f64, f64, f64)> {
    log_frequency_crossings(frequency, gain_db, 0.0)
        .into_iter()
        .map(|ugf| {
            let loop_phase = sample_at_log_frequency(log_frequency, phase_unwrapped, ugf);
            (
                ugf,
                crate::stability::phase_margin_deg(loop_phase),
                loop_phase,
            )
        })
        .min_by(|a, b| a.1.abs().total_cmp(&b.1.abs()))
}

/// How flat the first swept decade must be for the gain there to be presented
/// as the DC gain. A single pole held this far out contributes under 0.004 dB
/// at the lowest swept frequency.
const DC_FLATNESS_TOLERANCE_DB: f64 = 0.1;

/// Whether the sweep provably starts below every pole.
///
/// `gain_db.first()` is the gain at `f_min` and nothing more: a sweep opened
/// at 1 kHz on a loop with a 10 Hz dominant pole would otherwise report
/// mid-rolloff gain as the DC gain, and put `f₋₃dB` 3 dB below a number that
/// was never the DC gain either. The sweep has to span its first decade for
/// the question to be answerable at all.
///
/// Spanning the decade is not the same as sampling it. A `lin` sweep — what
/// the STB, PAC, PNOISE and PXF dialogs emit — is spaced evenly in frequency,
/// so 100 points from 1 kHz to 1 MHz put a single sample in `[1 kHz, 10 kHz]`
/// and the rest above it. A window holding one sample compares that gain
/// against itself and finds it flat, which is the false DC claim the span
/// check exists to stop. Flatness needs two gains to be flat *between*.
///
/// The first decade is read in source order, which an AC sweep always emits
/// ascending. A descending axis simply fails the span check and reports `A`
/// as measured — the conservative answer, never a false claim of DC.
pub fn low_frequency_gain_is_dc(frequency: &[f64], gain_db: &[f64]) -> bool {
    let n = frequency.len().min(gain_db.len());
    if n < 2 {
        return false;
    }
    let f_min = frequency[0];
    // Both guards refuse NaN explicitly rather than leaning on the negation.
    // A dataset can carry a NaN frequency, and the conservative answer — "not
    // provably DC" — is the one this function owes its caller; a NaN that fell
    // through would put `f₋₃dB` against a gain nobody measured.
    if f_min.is_nan() || f_min <= 0.0 {
        return false;
    }
    // `f_min` is positive and not NaN, so `decade_top` cannot be NaN either.
    let decade_top = f_min * 10.0;
    let f_max = frequency[n - 1];
    if f_max.is_nan() || f_max < decade_top {
        return false;
    }
    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    let mut sampled = 0usize;
    for i in 0..n {
        if frequency[i] > decade_top {
            break;
        }
        if !gain_db[i].is_finite() {
            return false;
        }
        sampled += 1;
        lo = lo.min(gain_db[i]);
        hi = hi.max(gain_db[i]);
    }
    sampled >= 2 && hi - lo <= DC_FLATNESS_TOLERANCE_DB
}

/// Derive retained Bode metrics without selecting or copying traces.
pub fn metrics_from_curves(
    frequency: &[f64],
    gain_db: &[f64],
    phase_deg: Option<&[f64]>,
) -> AcBodeMetrics {
    let adc_db = gain_db.first().copied();
    // Without a phase trace no unity-gain crossing can be shown to bind, so the
    // first one is all that can be named.
    let ugf = log_frequency_crossing(frequency, gain_db, 0.0);
    let f3db = adc_db.and_then(|adc| log_frequency_crossing(frequency, gain_db, adc - 3.0));
    let mut metrics = AcBodeMetrics {
        adc_db,
        adc_is_dc: low_frequency_gain_is_dc(frequency, gain_db),
        ugf,
        pm_deg: None,
        pm_phase_deg: None,
        f180: None,
        gm_db: None,
        f3db,
        gain_extremes: finite_extremes(gain_db).unwrap_or((0.0, 0.0)),
        phase_extremes: phase_deg.and_then(finite_extremes),
    };

    if let Some(phase) = phase_deg {
        let log_frequency = log_frequency_axis(frequency);
        let unwrapped = unwrap_retained_phase_deg(phase);
        if let Some((ugf, pm_deg, pm_phase_deg)) =
            binding_phase_margin(frequency, &log_frequency, gain_db, &unwrapped)
        {
            metrics.ugf = Some(ugf);
            metrics.pm_deg = Some(pm_deg);
            metrics.pm_phase_deg = Some(pm_phase_deg);
        }
        if let Some((f180, gm_db)) =
            binding_gain_margin(frequency, &log_frequency, gain_db, &unwrapped, metrics.ugf)
        {
            metrics.f180 = Some(f180);
            metrics.gm_db = Some(gm_db);
        }
    }

    metrics
}

fn finite_extremes(values: &[f64]) -> Option<(f64, f64)> {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for &value in values {
        if value.is_finite() {
            lo = lo.min(value);
            hi = hi.max(value);
        }
    }
    (lo <= hi).then_some((lo, hi))
}

#[cfg(test)]
mod tests {
    use super::log_frequency_crossing;

    #[test]
    fn log_frequency_crossing_interpolates_between_positive_frequencies() {
        let f = [1.0, 10.0, 100.0];
        let y = [20.0, 0.0, -20.0];

        assert_eq!(log_frequency_crossing(&f, &y, 0.0), Some(10.0));
    }
}
