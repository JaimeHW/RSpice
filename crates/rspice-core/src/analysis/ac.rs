//! AC Small-Signal Analysis

use crate::Complex64;
use crate::Value;
use crate::analysis::AnalysisConfig;
use crate::netlist::FreqVariation;

/// Endpoint tolerance scale for the sweep loop; ngspice's acan.c derives
/// it from CKTreltol (default 1e-3) so the final multiplicative step
/// still lands on fstop despite accumulated rounding.
const SWEEP_RELTOL: Value = 1e-3;

/// Generate AC sweep frequencies with ngspice-46 `acan.c` semantics.
///
/// * `Dec`: spans of at least a decade warp the step ratio so that
///   `floor(decades * n)` steps land exactly on `fstop`; shorter spans
///   anchor a pure `10^(1/n)` ratio at `fstart` and end at or below
///   `fstop`.
/// * `Oct`: always anchors a `2^(1/n)` ratio at `fstart` (never warped,
///   `fstop` is generally not hit exactly).
/// * `Lin`: `points` total spread evenly over the range; ngspice's
///   freqDelta degenerates to zero for `points <= 2`, emitting a single
///   point at `fstart`.
///
/// Returns an empty vector for invalid requests (zero points, reversed
/// or non-finite range, non-positive start for log sweeps, negative
/// start for linear sweeps).
pub fn ac_sweep_frequencies(
    variation: FreqVariation,
    points: usize,
    fstart: Value,
    fstop: Value,
) -> Vec<Value> {
    if points == 0 || !fstart.is_finite() || !fstop.is_finite() || fstart > fstop {
        return Vec::new();
    }
    match variation {
        FreqVariation::Dec | FreqVariation::Oct if fstart <= 0.0 => return Vec::new(),
        FreqVariation::Lin if fstart < 0.0 => return Vec::new(),
        _ => {}
    }

    let delta = match variation {
        FreqVariation::Dec => {
            if fstop / 10.0 < fstart {
                // Less than a decade apart: pure ratio anchored at fstart.
                if fstop == fstart {
                    1.0
                } else {
                    (std::f64::consts::LN_10 / points as Value).exp()
                }
            } else {
                // Warp the ratio so floor(decades*n) steps span the range.
                let num_steps = ((fstop / fstart).log10().abs() * points as Value).floor();
                ((fstop / fstart).ln() / num_steps).exp()
            }
        }
        FreqVariation::Oct => (std::f64::consts::LN_2 / points as Value).exp(),
        FreqVariation::Lin => {
            if points > 2 {
                (fstop - fstart) / (points - 1) as Value
            } else {
                0.0
            }
        }
    };

    let freq_tol = match variation {
        FreqVariation::Lin => delta * SWEEP_RELTOL,
        _ => delta * fstop * SWEEP_RELTOL,
    };

    let mut frequencies = Vec::new();
    let mut freq = fstart;
    while freq <= fstop + freq_tol {
        frequencies.push(freq);
        match variation {
            FreqVariation::Lin => {
                if delta == 0.0 {
                    break;
                }
                freq += delta;
            }
            _ => {
                if delta == 1.0 {
                    break;
                }
                freq *= delta;
            }
        }
    }
    frequencies
}

/// AC Analysis engine
#[derive(Debug)]
pub struct AcAnalysis {
    config: AnalysisConfig,
    /// Frequency points to analyze
    frequencies: Vec<Value>,
}

impl AcAnalysis {
    pub fn new(config: AnalysisConfig) -> Self {
        Self {
            config,
            frequencies: Vec::new(),
        }
    }

    /// Get config
    pub fn config(&self) -> &AnalysisConfig {
        &self.config
    }

    /// Set up linear frequency sweep
    pub fn linear_sweep(&mut self, start: Value, stop: Value, points: usize) {
        self.frequencies = ac_sweep_frequencies(FreqVariation::Lin, points, start, stop);
    }

    /// Set up decade frequency sweep
    pub fn decade_sweep(&mut self, start: Value, stop: Value, points_per_decade: usize) {
        self.frequencies = ac_sweep_frequencies(FreqVariation::Dec, points_per_decade, start, stop);
    }

    /// Set up octave frequency sweep
    pub fn octave_sweep(&mut self, start: Value, stop: Value, points_per_octave: usize) {
        self.frequencies = ac_sweep_frequencies(FreqVariation::Oct, points_per_octave, start, stop);
    }

    /// Get frequency points
    pub fn frequencies(&self) -> &[Value] {
        &self.frequencies
    }
}

impl Default for AcAnalysis {
    fn default() -> Self {
        Self::new(AnalysisConfig::default())
    }
}

/// AC analysis result at a single frequency
#[derive(Debug, Clone)]
pub struct AcResult {
    /// Frequency
    pub frequency: Value,
    /// Stable node names aligned with `voltages`
    pub node_names: Vec<String>,
    /// Stable branch names aligned with `currents`
    pub branch_names: Vec<String>,
    /// Complex node voltages
    pub voltages: Vec<Complex64>,
    /// Complex branch currents  
    pub currents: Vec<Complex64>,
}

impl AcResult {
    /// Get voltage magnitude at a node (1-indexed, consistent with SPICE)
    pub fn voltage_magnitude(&self, node: usize) -> Value {
        if node == 0 {
            return 0.0; // Ground is always 0V
        }
        self.voltages.get(node - 1).map(|v| v.norm()).unwrap_or(0.0)
    }

    /// Get voltage phase at a node (in radians, 1-indexed)
    pub fn voltage_phase(&self, node: usize) -> Value {
        if node == 0 {
            return 0.0; // Ground phase is 0
        }
        self.voltages.get(node - 1).map(|v| v.arg()).unwrap_or(0.0)
    }

    /// Get voltage in dB at a node
    pub fn voltage_db(&self, node: usize) -> Value {
        20.0 * self.voltage_magnitude(node).log10()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decade_sweep_warps_to_span_whole_decade_ranges() {
        // ngspice-46: floor(decades*n) steps land exactly on fstop.
        let f = ac_sweep_frequencies(FreqVariation::Dec, 10, 1.0, 1.0e6);
        assert_eq!(f.len(), 61);
        assert_eq!(f[0], 1.0);
        let last = *f.last().unwrap();
        assert!((last - 1.0e6).abs() <= 1.0e6 * 1e-9, "last={last}");

        // Non-integer decade count: floor, not ceil (ngspice emits 55
        // points for 1Hz..300kHz at 10/decade, not 56).
        let f = ac_sweep_frequencies(FreqVariation::Dec, 10, 1.0, 3.0e5);
        assert_eq!(f.len(), 55);
        let last = *f.last().unwrap();
        assert!((last - 3.0e5).abs() <= 3.0e5 * 1e-9, "last={last}");
    }

    #[test]
    fn decade_sweep_anchors_at_fstart_for_sub_decade_ranges() {
        // Span under a decade keeps the pure 10^(1/n) ratio: 100..300 at
        // 10/decade gives 100*10^(k/10) up to 251.19, fstop not hit.
        let f = ac_sweep_frequencies(FreqVariation::Dec, 10, 100.0, 300.0);
        assert_eq!(f.len(), 5);
        assert_eq!(f[0], 100.0);
        let last = *f.last().unwrap();
        assert!((last - 251.188_643_150_958).abs() < 1e-9, "last={last}");
    }

    #[test]
    fn octave_sweep_never_warps() {
        // 10..50 at 2/octave: 10, 14.1, 20, 28.3, 40 — next step exceeds
        // fstop, which is not included.
        let f = ac_sweep_frequencies(FreqVariation::Oct, 2, 10.0, 50.0);
        assert_eq!(f.len(), 5);
        let last = *f.last().unwrap();
        assert!((last - 40.0).abs() < 40.0 * 1e-12, "last={last}");
    }

    #[test]
    fn linear_sweep_matches_ngspice_point_counts() {
        let f = ac_sweep_frequencies(FreqVariation::Lin, 5, 1.0e3, 2.0e3);
        assert_eq!(f.len(), 5);
        assert_eq!(f[0], 1.0e3);
        assert!((f[4] - 2.0e3).abs() < 1e-6);

        // ngspice's freqDelta degenerates to 0 for n <= 2: single point.
        let f = ac_sweep_frequencies(FreqVariation::Lin, 2, 1.0e3, 2.0e3);
        assert_eq!(f, vec![1.0e3]);
        let f = ac_sweep_frequencies(FreqVariation::Lin, 1, 1.0e3, 1.0e3);
        assert_eq!(f, vec![1.0e3]);
    }

    #[test]
    fn degenerate_ranges_are_rejected_or_single_point() {
        assert!(ac_sweep_frequencies(FreqVariation::Dec, 10, 0.0, 1.0e3).is_empty());
        assert!(ac_sweep_frequencies(FreqVariation::Dec, 10, 1.0e3, 1.0).is_empty());
        assert!(ac_sweep_frequencies(FreqVariation::Dec, 0, 1.0, 1.0e3).is_empty());
        // Equal start/stop emits exactly one point for log sweeps.
        assert_eq!(
            ac_sweep_frequencies(FreqVariation::Dec, 10, 5.0e3, 5.0e3),
            vec![5.0e3]
        );
        assert_eq!(
            ac_sweep_frequencies(FreqVariation::Oct, 10, 5.0e3, 5.0e3),
            vec![5.0e3]
        );
    }
}
