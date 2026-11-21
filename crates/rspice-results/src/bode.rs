//! Exact frequency-response and Bode margin mathematics over retained numeric slices.
//!
//! Retained projection accepts the application's magnitude preference; run selection
//! remains with the application adapter.

use crate::measurements::finite_extremes;

pub mod retained;

/// Stability numbers derived from one magnitude/phase pair.
///
/// # Conventions
///
/// The magnitude and phase are one complex loop gain read in polar form, and
/// the margins are read off that loop gain exactly as the Nyquist sheet reads
/// its locus: [`crate::nyquist::NyquistData::from_polar`] rebuilds `L` and its
/// margins interpolate `log|L|` and the unwrapped `∠L` against log-frequency
/// through the same samples. The two sheets therefore print the same margins
/// for one loop, to the last digit, and both carry the interpolation's stated
/// error ([`crate::nyquist::MARGIN_ERROR_DECIBELS`]).
///
/// The unwrapped branch is anchored at the lowest swept frequency. A gain
/// margin is read where the locus crosses the negative real axis. Phase margin
/// is the textbook `180° + ∠L(f_ugf)`, folded into `(-180°, 180°]` by
/// [`crate::stability::phase_margin_deg`]: the turn MATLAB, ADS and Spectre
/// all report a margin in. When the fold moved the number,
/// [`AcBodeMetrics::pm_phase_deg`] carries the angle it was folded from and the
/// card says so beside the margin.
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
/// A level touched exactly at a retained sample is reported once: each
/// segment owns its leading endpoint, and the final segment additionally owns
/// its trailing one.
pub(crate) fn log_frequency_crossings(frequency: &[f64], series: &[f64], level: f64) -> Vec<f64> {
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
        let locus = crate::nyquist::NyquistData::from_polar("", frequency, gain_db, phase);
        if let Some(margin) = locus.phase_margin() {
            metrics.ugf = Some(margin.frequency);
            metrics.pm_deg = Some(margin.value);
            metrics.pm_phase_deg = Some(margin.phase_deg);
        }
        if let Some(margin) = locus.gain_margin() {
            metrics.f180 = Some(margin.frequency);
            metrics.gm_db = Some(20.0 * margin.value.log10());
        }
    }

    metrics
}

#[cfg(test)]
mod tests {
    use super::{log_frequency_crossing, metrics_from_curves};
    use crate::nyquist::{MARGIN_ERROR_DECIBELS, MARGIN_ERROR_DEGREES, NyquistData};
    use num_complex::Complex64;

    /// `L = 4 / (1 + jf/1 kHz)³` swept from 1 Hz to 10 MHz at five points a
    /// decade, the first sample `offset` decades above 1 Hz.
    fn three_pole_sweep(offset: f64) -> (Vec<f64>, Vec<Complex64>) {
        let frequency: Vec<f64> = (0..=35)
            .map(|i| 10f64.powf(offset + f64::from(i) / 5.0))
            .collect();
        let gain = frequency
            .iter()
            .map(|&f| Complex64::new(4.0, 0.0) / Complex64::new(1.0, f / 1.0e3).powi(3))
            .collect();
        (frequency, gain)
    }

    /// The Bode card and the Nyquist card read one loop's margins off one
    /// interpolant of the complex loop gain. At five points a decade both
    /// read the closed form, GM = 20·log₁₀(8/4) = 6.0206 dB and
    /// PM = 180° − 3·atan(√(4^⅔ − 1)) = 27.1416°, within the stated error,
    /// wherever the sweep's samples fall, and they read the same numbers.
    #[test]
    fn both_cards_read_the_closed_form_margins_of_a_coarse_sweep() {
        let gain_margin = 20.0 * 2f64.log10();
        let crossover = (4f64.powf(2.0 / 3.0) - 1.0).sqrt();
        let phase_margin = 180.0 - 3.0 * crossover.atan().to_degrees();
        for step in 0..8 {
            let (frequency, gain) = three_pole_sweep(f64::from(step) * 0.025);
            let gain_db: Vec<f64> = gain.iter().map(|l| 20.0 * l.norm().log10()).collect();
            let phase: Vec<f64> = gain.iter().map(|l| l.arg().to_degrees()).collect();
            let bode = metrics_from_curves(&frequency, &gain_db, Some(&phase));
            let real: Vec<f64> = gain.iter().map(|l| l.re).collect();
            let imag: Vec<f64> = gain.iter().map(|l| l.im).collect();
            let locus = NyquistData::from_arrays("L", &frequency, &real, &imag);
            let nyquist_gm = 20.0 * locus.gain_margin().expect("a gain margin").value.log10();
            let nyquist_pm = locus.phase_margin().expect("a phase margin").value;

            let bode_gm = bode.gm_db.expect("a gain margin");
            let bode_pm = bode.pm_deg.expect("a phase margin");
            assert!(
                (bode_gm - gain_margin).abs() <= MARGIN_ERROR_DECIBELS,
                "GM {bode_gm} dB against {gain_margin} dB"
            );
            assert!(
                (bode_pm - phase_margin).abs() <= MARGIN_ERROR_DEGREES,
                "PM {bode_pm}° against {phase_margin}°"
            );
            assert!(
                (bode_gm - nyquist_gm).abs() < 1.0e-9,
                "{bode_gm} {nyquist_gm}"
            );
            assert!(
                (bode_pm - nyquist_pm).abs() < 1.0e-9,
                "{bode_pm} {nyquist_pm}"
            );
            let f180 = bode.f180.expect("a phase crossover");
            assert!(
                (f180 / (3f64.sqrt() * 1.0e3) - 1.0).abs() < 1.0e-3,
                "{f180}"
            );
            let ugf = bode.ugf.expect("a unity-gain crossover");
            assert!((ugf / (crossover * 1.0e3) - 1.0).abs() < 1.0e-3, "{ugf}");
        }
    }

    #[test]
    fn log_frequency_crossing_interpolates_between_positive_frequencies() {
        let f = [1.0, 10.0, 100.0];
        let y = [20.0, 0.0, -20.0];

        assert_eq!(log_frequency_crossing(&f, &y, 0.0), Some(10.0));
    }
}
