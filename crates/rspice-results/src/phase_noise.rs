//! Exact retained phase-noise evidence, spectra and spot values.

use crate::analysis_type::AnalysisType;
use crate::family_metadata::{AnalysisResultFamilyMetadata, PeriodicNoiseOutputQuantity};
use crate::waveform::RetainedWaveform;

pub fn retained_phase_noise_carrier(
    analysis_type: AnalysisType,
    metadata: Option<&AnalysisResultFamilyMetadata>,
) -> Option<f64> {
    let metadata = metadata?;
    if metadata.validate_for(analysis_type).is_err() {
        return None;
    }
    let AnalysisResultFamilyMetadata::PeriodicNoise {
        output_quantity: PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
        carrier_frequency_hz: Some(carrier_frequency_hz),
    } = metadata
    else {
        return None;
    };
    Some(*carrier_frequency_hz)
}

/// A waveform name is phase-noise evidence only when it says so directly.
///
/// `onoise` and `inoise` deliberately do *not* qualify: the retained result
/// adapter uses those names for ordinary periodic noise too, and assigning a
/// dBc/Hz meaning to them would invent a carrier normalization.
fn is_explicit_phase_noise_name(name: &str) -> bool {
    let compact = name
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .collect::<String>()
        .to_ascii_lowercase();
    compact.contains("phasenoise") || compact == "lf"
}

pub fn phase_noise_waveform_is_renderable(
    waveform: &RetainedWaveform,
    mut note_scan: impl FnMut(),
) -> bool {
    if !is_explicit_phase_noise_name(&waveform.name)
        || waveform.x.len() != waveform.y.len()
        || waveform.x.len() < 2
    {
        return false;
    }
    note_scan();

    waveform
        .x
        .samples(..)
        .zip(waveform.y.samples(..))
        .try_fold(None, |previous, (offset, level)| {
            (offset.is_finite()
                && offset > 0.0
                && level.is_finite()
                && previous.is_none_or(|previous| offset > previous))
            .then_some(Some(offset))
        })
        .is_some()
}

/// `true` if this analysis contains a usable, explicitly-labelled retained
/// phase-noise trace.  Callers use this for viewer availability; it never
/// widens PNOISE into a phase-noise assertion by analysis type alone.
pub fn phase_noise_is_renderable<W: AsRef<RetainedWaveform>>(
    success: bool,
    analysis_type: AnalysisType,
    metadata: Option<&AnalysisResultFamilyMetadata>,
    waveforms: &[W],
    mut note_scan: impl FnMut(),
) -> bool {
    success
        && matches!(analysis_type, AnalysisType::Pnoise | AnalysisType::Qpnoise)
        && retained_phase_noise_carrier(analysis_type, metadata).is_some()
        && waveforms
            .iter()
            .any(|waveform| phase_noise_waveform_is_renderable(waveform.as_ref(), &mut note_scan))
}

/// L(f) at `offset`, in dBc/Hz: the retained sample there, or between two
/// samples the straight line joining them in decibels against log
/// frequency, which is exact on every power-law stretch of a spectrum (a
/// −20 dB/decade line reads −20 dB/decade between its samples). `None`
/// outside the swept offsets: a spot the sweep never reached is not read.
#[must_use]
pub fn spot_level_at(offsets: &[f64], levels: &[f64], offset: f64) -> Option<f64> {
    let n = offsets.len().min(levels.len());
    let (offsets, levels) = (&offsets[..n], &levels[..n]);
    if !(offset.is_finite() && offset > 0.0) || n == 0 {
        return None;
    }
    let above = offsets.partition_point(|&sample| sample < offset);
    if above == n {
        return None;
    }
    if offsets[above] == offset {
        return levels[above].is_finite().then_some(levels[above]);
    }
    if above == 0 {
        return None;
    }
    let (f0, f1) = (offsets[above - 1], offsets[above]);
    let (l0, l1) = (levels[above - 1], levels[above]);
    if !(f0 > 0.0 && f1 > f0 && l0.is_finite() && l1.is_finite()) {
        return None;
    }
    let at = (offset / f0).ln() / (f1 / f0).ln();
    Some(l0 + (l1 - l0) * at)
}

/// ∫ S(f) df over `band` of a one-sided density sampled at ascending
/// `frequency`, each stretch between samples read as the power law through
/// its ends (a straight line on log-log axes), which integrates a 1/fⁿ
/// spectrum exactly. A stretch with an end at zero is read linearly. A
/// band edge between samples takes the density the same law gives there.
/// `None` when the band is empty or reaches outside the sweep, or a sample
/// in it is not a finite nonnegative density.
#[must_use]
pub fn integrate_density(frequency: &[f64], density: &[f64], band: (f64, f64)) -> Option<f64> {
    let n = frequency.len().min(density.len());
    let (frequency, density) = (&frequency[..n], &density[..n]);
    let (low, high) = band;
    if n < 2 || !(low.is_finite() && high.is_finite() && low < high) {
        return None;
    }
    if low < frequency[0] || high > frequency[n - 1] {
        return None;
    }
    let mut total = 0.0;
    for index in 0..n - 1 {
        let (f0, f1) = (frequency[index], frequency[index + 1]);
        if f1 <= low || f0 >= high {
            continue;
        }
        let (s0, s1) = (density[index], density[index + 1]);
        if !(f1 > f0 && s0.is_finite() && s1.is_finite() && s0 >= 0.0 && s1 >= 0.0) {
            return None;
        }
        let (a, b) = (f0.max(low), f1.min(high));
        total += segment_integral(f0, s0, f1, s1, a, b);
    }
    total.is_finite().then_some(total)
}

/// ∫ₐᵇ S df on one stretch `(f0, s0)..(f1, s1)`, `f0 <= a < b <= f1`.
fn segment_integral(f0: f64, s0: f64, f1: f64, s1: f64, a: f64, b: f64) -> f64 {
    if s0 > 0.0 && s1 > 0.0 && f0 > 0.0 {
        // S(f) = s0 (f / f0)^k.
        let k = (s1 / s0).ln() / (f1 / f0).ln();
        let at = |f: f64| s0 * (f / f0).powf(k);
        if (k + 1.0).abs() < 1.0e-12 {
            at(a) * a * (b / a).ln()
        } else {
            (at(b) * b - at(a) * a) / (k + 1.0)
        }
    } else {
        let at = |f: f64| s0 + (s1 - s0) * (f - f0) / (f1 - f0);
        0.5 * (at(a) + at(b)) * (b - a)
    }
}

/// What a band of single-sideband phase noise L(f) integrates to.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntegratedPhaseNoise {
    /// The offsets integrated over, in hertz.
    pub band: (f64, f64),
    /// ∫ L(f) df over the band, as a power ratio to the carrier.
    pub single_sideband_power: f64,
}

impl IntegratedPhaseNoise {
    /// The band's L(f) as one figure, in dBc.
    #[must_use]
    pub fn integrated_dbc(self) -> f64 {
        10.0 * self.single_sideband_power.log10()
    }

    /// RMS phase error over the band, in radians: both sidebands, so
    /// √(2 ∫ L(f) df).
    #[must_use]
    pub fn phase_error_rms_rad(self) -> f64 {
        (2.0 * self.single_sideband_power).sqrt()
    }

    /// RMS jitter over the band, in seconds: the phase error over 2π f_c.
    #[must_use]
    pub fn jitter_rms_s(self, carrier_hz: f64) -> f64 {
        self.phase_error_rms_rad() / (std::f64::consts::TAU * carrier_hz)
    }
}

/// Integrate L(f) in dBc/Hz over `band`, read as a power law between
/// samples. `None` where [`integrate_density`] refuses the band.
#[must_use]
pub fn integrate_phase_noise(
    offsets: &[f64],
    levels_dbc_per_hz: &[f64],
    band: (f64, f64),
) -> Option<IntegratedPhaseNoise> {
    let density: Vec<f64> = levels_dbc_per_hz
        .iter()
        .map(|level| 10.0_f64.powf(level / 10.0))
        .collect();
    let power = integrate_density(offsets, &density, band)?;
    Some(IntegratedPhaseNoise {
        band,
        single_sideband_power: power,
    })
}

/// Period and period-to-period jitter of an edge sequence, in seconds rms.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CycleJitter {
    /// Jc: the rms of one period's deviation, x(n+1) − x(n).
    pub cycle_s: f64,
    /// Jcc: the rms of the change between adjacent periods,
    /// x(n+2) − 2x(n+1) + x(n).
    pub cycle_to_cycle_s: f64,
}

/// Jc and Jcc from the one-sided density S_x(f) of an edge's timing
/// deviation x, in s²/Hz, on a carrier of `carrier_hz`:
///
/// Jc² = ∫ S_x(f) · 4 sin²(π f / f_c) df and Jcc² = ∫ S_x(f) · 16 sin⁴(π f / f_c) df,
///
/// over 0 … f_c/2, the band an edge sequence sampled once a period holds.
/// The sweep must reach f_c/2, and it is read from its first sample, the
/// weights vanishing at zero offset. `None` when it stops short.
#[must_use]
pub fn cycle_jitter(
    frequency: &[f64],
    timing_density: &[f64],
    carrier_hz: f64,
) -> Option<CycleJitter> {
    let n = frequency.len().min(timing_density.len());
    if n < 2 || !(carrier_hz.is_finite() && carrier_hz > 0.0) {
        return None;
    }
    let nyquist = carrier_hz / 2.0;
    if frequency[n - 1] < nyquist * (1.0 - 1.0e-9) {
        return None;
    }
    let band = (frequency[0], nyquist.min(frequency[n - 1]));
    let weighted = |weight: &dyn Fn(f64) -> f64| {
        weighted_integral(&frequency[..n], &timing_density[..n], band, weight)
    };
    let phase = |f: f64| (std::f64::consts::PI * f / carrier_hz).sin();
    let cycle = weighted(&|f| 4.0 * phase(f).powi(2))?;
    let cycle_to_cycle = weighted(&|f| 16.0 * phase(f).powi(4))?;
    Some(CycleJitter {
        cycle_s: cycle.sqrt(),
        cycle_to_cycle_s: cycle_to_cycle.sqrt(),
    })
}

/// The one-sided timing density of a carrier's edges, in s²/Hz, from its
/// single-sideband phase noise L(f) in dBc/Hz: S_φ = 2 L, S_x = S_φ / (2π f_c)².
#[must_use]
pub fn timing_density_from_phase_noise(levels_dbc_per_hz: &[f64], carrier_hz: f64) -> Vec<f64> {
    let radians_per_second = std::f64::consts::TAU * carrier_hz;
    levels_dbc_per_hz
        .iter()
        .map(|level| 2.0 * 10.0_f64.powf(level / 10.0) / radians_per_second.powi(2))
        .collect()
}

/// ∫ S(f) w(f) df over `band`, S read as in [`integrate_density`] and the
/// smooth weight sampled by Simpson's rule on each stretch.
fn weighted_integral(
    frequency: &[f64],
    density: &[f64],
    band: (f64, f64),
    weight: &dyn Fn(f64) -> f64,
) -> Option<f64> {
    const PANELS: usize = 64;
    let (low, high) = band;
    if !(low < high) || low < frequency[0] || high > frequency[frequency.len() - 1] {
        return None;
    }
    let mut total = 0.0;
    for index in 0..frequency.len() - 1 {
        let (f0, f1) = (frequency[index], frequency[index + 1]);
        if f1 <= low || f0 >= high {
            continue;
        }
        let (s0, s1) = (density[index], density[index + 1]);
        if !(f1 > f0 && s0.is_finite() && s1.is_finite() && s0 >= 0.0 && s1 >= 0.0) {
            return None;
        }
        let (a, b) = (f0.max(low), f1.min(high));
        let at = |f: f64| {
            let s = if s0 > 0.0 && s1 > 0.0 && f0 > 0.0 {
                s0 * (f / f0).powf((s1 / s0).ln() / (f1 / f0).ln())
            } else {
                s0 + (s1 - s0) * (f - f0) / (f1 - f0)
            };
            s * weight(f)
        };
        let h = (b - a) / PANELS as f64;
        let mut sum = at(a) + at(b);
        for panel in 1..PANELS {
            sum += at(a + h * panel as f64) * if panel % 2 == 1 { 4.0 } else { 2.0 };
        }
        total += sum * h / 3.0;
    }
    total.is_finite().then_some(total)
}

/// A unique, successful, finite nonnegative retained phase/jitter measurement.
#[cfg(feature = "engine-evidence")]
pub fn retained_measurement<W>(
    analysis: &crate::analysis_result::AnalysisResult<W>,
    name: &str,
) -> Option<f64> {
    let mut matching = analysis
        .measurements
        .iter()
        .filter(|measurement| measurement.name == name);
    let measurement = matching.next()?;
    let value = measurement.value?;
    (matching.next().is_none()
        && measurement.passed
        && measurement.error.is_none()
        && value.is_finite()
        && value >= 0.0)
        .then_some(value)
}

/// Successful finite nonnegative device shares, in retained measurement order.
#[cfg(feature = "engine-evidence")]
pub fn retained_device_noise_shares(
    measurements: &[rspice_core::MeasureResult],
) -> impl Iterator<Item = (&str, f64)> {
    measurements.iter().filter_map(|measurement| {
        let name = measurement
            .name
            .strip_prefix("noise_share_percent(")?
            .strip_suffix(')')?;
        let value = measurement.value?;
        (measurement.passed && measurement.error.is_none() && value.is_finite() && value >= 0.0)
            .then_some((name, value))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn waveform(name: &str) -> RetainedWaveform {
        RetainedWaveform::new(name, vec![1.0, 1.0e3, 1.0e6], vec![-72.0, -103.0, -132.0])
    }

    #[test]
    fn phase_noise_rejects_an_offset_axis_that_does_not_ascend() {
        let invalid = RetainedWaveform::new(
            "phase_noise",
            vec![1.0, 0.0, 1.0e6],
            vec![-80.0, -100.0, -130.0],
        );
        assert!(!phase_noise_waveform_is_renderable(&invalid, || {}));
        assert!(phase_noise_waveform_is_renderable(
            &waveform("phase_noise"),
            || {}
        ));
    }

    fn close(actual: f64, expected: f64, relative: f64) {
        assert!(
            ((actual - expected) / expected).abs() <= relative,
            "{actual} is not within {relative} of {expected}"
        );
    }

    /// A −20 dB/decade line: L(f) = −80 dBc/Hz at 10 kHz, sampled at
    /// irregular offsets that never land on the spots read.
    fn minus_twenty_per_decade() -> (Vec<f64>, Vec<f64>) {
        let offsets = vec![1.3e3, 7.0e3, 2.9e4, 3.1e5, 2.2e6, 9.0e6];
        let levels = offsets
            .iter()
            .map(|&f: &f64| -80.0 - 20.0 * (f / 1.0e4).log10())
            .collect();
        (offsets, levels)
    }

    /// A spot between two samples reads the line through them in dB
    /// against log frequency; a sample reads itself; outside the sweep
    /// there is nothing to read.
    #[test]
    fn a_spot_between_two_samples_reads_the_interpolated_level() {
        let (offsets, levels) = minus_twenty_per_decade();
        for spot in [1.0e4_f64, 1.0e5, 1.0e6] {
            let expected = -80.0 - 20.0 * (spot / 1.0e4).log10();
            let level = spot_level_at(&offsets, &levels, spot).unwrap();
            assert!((level - expected).abs() < 1e-9, "{spot}: {level}");
        }
        assert_eq!(spot_level_at(&offsets, &levels, 2.9e4), Some(levels[2]));
        // Two samples, a spot a quarter decade above the first.
        let level = spot_level_at(&[1.0e3, 1.0e4], &[-90.0, -110.0], 10.0_f64.powf(3.25));
        assert!((level.unwrap() + 95.0).abs() < 1e-12);
        assert_eq!(spot_level_at(&offsets, &levels, 1.0e3), None);
        assert_eq!(spot_level_at(&offsets, &levels, 1.0e7), None);
    }

    /// For L(f) = L₀ (f₀/f)², ∫ L df over f₁…f₂ is L₀ f₀² (1/f₁ − 1/f₂);
    /// the phase error is √2 times its root and the jitter that over 2π f_c.
    #[test]
    fn integrated_jitter_over_a_band_equals_the_closed_form_for_a_minus_twenty_db_per_decade_line()
    {
        let (offsets, levels) = minus_twenty_per_decade();
        let (f1, f2) = (1.0e4, 1.0e6);
        let power = 1.0e-8 * 1.0e4_f64.powi(2) * (1.0 / f1 - 1.0 / f2);
        let integrated = integrate_phase_noise(&offsets, &levels, (f1, f2)).unwrap();
        close(integrated.single_sideband_power, power, 1e-12);
        close(integrated.integrated_dbc(), 10.0 * power.log10(), 1e-12);
        close(
            integrated.phase_error_rms_rad(),
            (2.0 * power).sqrt(),
            1e-12,
        );
        let carrier = 2.4e9;
        close(
            integrated.jitter_rms_s(carrier),
            (2.0 * power).sqrt() / (std::f64::consts::TAU * carrier),
            1e-12,
        );
        // By hand: 10⁻⁸ · 10⁸ · (10⁻⁴ − 10⁻⁶) = 9.9·10⁻⁵, which is
        // −40.04 dBc and 14.07 mrad.
        close(integrated.integrated_dbc(), -40.043_648_054, 1e-9);
        close(integrated.phase_error_rms_rad(), 1.407_124_728e-2, 1e-9);
        assert!(integrate_phase_noise(&offsets, &levels, (1.0e3, 1.0e6)).is_none());
        assert!(integrate_phase_noise(&offsets, &levels, (1.0e6, 1.0e4)).is_none());
    }

    /// A white edge sequence of variance σ² has the flat one-sided density
    /// 2σ²/f_c on 0…f_c/2: its period jitter is √2 σ and its
    /// period-to-period jitter √6 σ.
    #[test]
    fn cycle_jitter_of_white_edge_timing_is_root_two_and_root_six_sigma() {
        let (carrier, sigma) = (1.0e9, 1.0e-12);
        let frequency: Vec<f64> = (0..=40)
            .map(|step| f64::from(step) * carrier / 80.0)
            .collect();
        let density = vec![2.0 * sigma * sigma / carrier; frequency.len()];
        let jitter = cycle_jitter(&frequency, &density, carrier).unwrap();
        close(jitter.cycle_s, 2.0_f64.sqrt() * sigma, 1e-9);
        close(jitter.cycle_to_cycle_s, 6.0_f64.sqrt() * sigma, 1e-9);
        assert!(cycle_jitter(&frequency[..30], &density[..30], carrier).is_none());
    }
}
