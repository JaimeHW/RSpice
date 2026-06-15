//! Axis scales and tick generation.

use super::format::tick_label;

/// X-axis scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XScale {
    /// Linear data axis (time, sweep value, frequency bins).
    #[default]
    Linear,
    /// Logarithmic frequency axis; ticks fall on decades.
    Log10,
}

impl XScale {
    /// Map a data value to 0‥1 within `[min, max]`.
    pub fn normalize(self, x: f64, min: f64, max: f64) -> f64 {
        match self {
            XScale::Linear => (x - min) / (max - min),
            XScale::Log10 => {
                let (lx, lmin, lmax) = (x.max(f64::MIN_POSITIVE).log10(), min.log10(), max.log10());
                (lx - lmin) / (lmax - lmin)
            }
        }
    }

    /// Inverse of [`XScale::normalize`].
    pub fn denormalize(self, t: f64, min: f64, max: f64) -> f64 {
        match self {
            XScale::Linear => min + t * (max - min),
            XScale::Log10 => {
                let (lmin, lmax) = (min.log10(), max.log10());
                10f64.powf(lmin + t * (lmax - lmin))
            }
        }
    }
}

/// "Nice" linear tick positions covering `[min, max]` with roughly `target`
/// steps, on a 1–2–5 ladder.
pub fn linear_ticks(min: f64, max: f64, target: usize) -> Vec<(f64, String)> {
    let span = max - min;
    if !(span.is_finite() && span > 0.0) {
        return Vec::new();
    }
    let raw_step = span / target.max(2) as f64;
    let magnitude = 10f64.powf(raw_step.log10().floor());
    let residual = raw_step / magnitude;
    let step = magnitude
        * if residual <= 1.5 {
            1.0
        } else if residual <= 3.5 {
            2.0
        } else if residual <= 7.5 {
            5.0
        } else {
            10.0
        };
    let first = (min / step).ceil() as i64;
    let last = (max / step).floor() as i64;
    (first..=last)
        .map(|i| {
            let v = i as f64 * step;
            // Snap float noise (e.g. 0.30000000000000004) to the step grid.
            let v = (v / step).round() * step;
            (v, tick_label(v))
        })
        .collect()
}

/// Decade tick positions for a log axis covering `[min, max]` (Hz-style:
/// 1, 10, 100, 1k, …). Sub-decade minor ticks are intentionally omitted —
/// the design keeps plot grids quiet.
pub fn decade_ticks(min: f64, max: f64) -> Vec<(f64, String)> {
    if !(min > 0.0 && max > min) {
        return Vec::new();
    }
    let first = min.log10().ceil() as i32;
    let last = max.log10().floor() as i32;
    (first..=last)
        .map(|d| {
            let v = 10f64.powi(d);
            (v, tick_label(v))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_ticks_are_nice() {
        let ticks = linear_ticks(0.0, 10e-6, 6);
        assert!(ticks.len() >= 4 && ticks.len() <= 8);
        assert_eq!(ticks[0].1, "0");
        assert!(ticks.iter().any(|t| t.1 == "4µ"));
    }

    #[test]
    fn decade_ticks_span() {
        let ticks = decade_ticks(1.0, 1e9);
        assert_eq!(ticks.len(), 10);
        assert_eq!(ticks[3].1, "1k");
        assert_eq!(ticks[9].1, "1G");
    }

    #[test]
    fn log_normalize_roundtrip() {
        let x = 12_345.0;
        let t = XScale::Log10.normalize(x, 1.0, 1e9);
        let back = XScale::Log10.denormalize(t, 1.0, 1e9);
        assert!((back / x - 1.0).abs() < 1e-9);
    }
}
