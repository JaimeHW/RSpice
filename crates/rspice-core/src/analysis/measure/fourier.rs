//! Xyce's `FOUR` measure: the Fourier series of a window's last period.
//!
//! Xyce does not integrate the accepted steps. It fits a natural cubic
//! spline through the steps of the last period, samples it on an even grid
//! and takes a discrete Fourier series of the grid (N_IO_MeasureFourier.C:
//! getLastPeriod_ 159-195, interpolateData_ 244-308, calculateFT_ 317-354).
//! The measure's value is the total harmonic distortion of that series.

use crate::Value;

/// The total harmonic distortion, in percent, of the last period `1/at` of
/// `time`/`values` (the window's accepted steps), as Xyce's `FOUR` measure
/// reports it: harmonics 2 to `num_freq - 1` over the fundamental. `None`
/// when `at` is not positive, or the window holds less than one period or
/// too few steps for a spline, where Xyce leaves the measure at its default
/// value.
pub(super) fn xyce_total_harmonic_distortion(
    time: &[Value],
    values: &[Value],
    at: Value,
    num_freq: usize,
    grid_size: usize,
) -> Option<Value> {
    // Without a fundamental there is no period to read.
    if at <= 0.0 {
        return None;
    }
    let end_time = *time.last()?;
    let period = 1.0 / at;
    // getLastPeriod_: the period that ends at the last step opens on the last
    // step at or before its start.
    let mut last_period_start = (at * end_time - 1.0) / at;
    let start = if last_period_start.abs() < Value::EPSILON {
        last_period_start = 0.0;
        0
    } else if last_period_start > 0.0 {
        time.iter().rposition(|time| *time <= last_period_start)?
    } else {
        return None;
    };
    let time = &time[start..];
    let values = &values[start..];
    if num_freq < 2 || grid_size == 0 {
        return None;
    }

    // interpolateData_ and calculateFT_.
    let step = period / grid_size as Value;
    let grid = crate::analysis::fourier::natural_spline_grid(
        time,
        values,
        last_period_start,
        step,
        grid_size,
    )?;
    let mut sine = vec![0.0; num_freq];
    let mut cosine = vec![0.0; num_freq];
    for (i, value) in grid.into_iter().enumerate() {
        for (harmonic, (sine, cosine)) in sine.iter_mut().zip(cosine.iter_mut()).enumerate() {
            let angle =
                harmonic as Value * 2.0 * std::f64::consts::PI * i as Value / grid_size as Value;
            *sine += value * angle.sin();
            *cosine += value * angle.cos();
        }
    }
    let magnitude = |harmonic: usize| {
        let sine = sine[harmonic] * 2.0 / grid_size as Value;
        let cosine = cosine[harmonic] * 2.0 / grid_size as Value;
        sine.hypot(cosine)
    };
    let fundamental = magnitude(1);
    let distortion = (2..num_freq)
        .map(|harmonic| (magnitude(harmonic) / fundamental).powi(2))
        .sum::<Value>();
    Some(100.0 * distortion.sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A fundamental with a tenth-amplitude third harmonic has 10 % THD, and
    /// the series reads only the last period: the half-amplitude fifth
    /// harmonic of the first millisecond does not move it.
    #[test]
    fn the_last_period_gives_the_harmonic_ratio() {
        let at = 1.0e3;
        let steps = 2500;
        let time = (0..=steps)
            .map(|i| 2.5e-3 * i as Value / steps as Value)
            .collect::<Vec<_>>();
        let omega = 2.0 * std::f64::consts::PI * at;
        let values = time
            .iter()
            .map(|t| {
                let harmonic = if *t < 1.0e-3 {
                    0.5 * (5.0 * omega * t).sin()
                } else {
                    0.1 * (3.0 * omega * t).sin()
                };
                (omega * t).sin() + harmonic
            })
            .collect::<Vec<_>>();
        let thd = xyce_total_harmonic_distortion(&time, &values, at, 10, 200)
            .expect("two periods hold a last period");
        assert!((thd - 10.0).abs() < 1.0e-4, "{thd}");
        assert_eq!(
            xyce_total_harmonic_distortion(&time[..900], &values[..900], at, 10, 200),
            None,
            "less than one period"
        );
    }
}
