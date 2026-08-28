//! PSS Analysis Results
//!
//! Data structures for storing and accessing PSS analysis results.

use crate::Value;
use crate::analysis::fourier::HarmonicComponent;
use crate::analysis::{
    FLOQUET_UNIT_CIRCLE_BAND, FloquetOrbitKind, FloquetSpectrumEvidence, FloquetStabilityVerdict,
    classify_floquet_stability, select_autonomous_phase_mode,
};
use std::f64::consts::PI;

/// Compatibility alias for the shared Floquet stability verdict.
pub type PssStabilityVerdict = FloquetStabilityVerdict;

/// Result of Periodic Steady-State analysis
///
/// Contains the converged periodic solution, harmonic content, and
/// convergence diagnostics. This serves as the periodic operating point
/// for subsequent analyses like PAC and PNoise.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "veriloga", derive(serde::Serialize, serde::Deserialize))]
pub struct PssResult {
    /// Converged fundamental period (seconds)
    pub period: Value,

    /// Fundamental frequency (Hz) = 1/period
    pub frequency: Value,

    /// Number of shooting iterations to reach convergence
    pub iterations: usize,

    /// Final residual norm ||x(T) - x(0)||
    pub residual_norm: Value,

    /// Time points within one period [0, T]
    pub time: Vec<Value>,

    /// Periodic waveforms for each node
    pub waveforms: Vec<PeriodicWaveform>,

    /// Node names (maps waveform index to node name)
    pub node_names: Vec<String>,

    /// Whether period was auto-detected (autonomous circuit)
    pub period_detected: bool,

    /// Monodromy matrix eigenvalues (Floquet multipliers)
    /// Used for stability analysis and PNoise
    pub floquet_multipliers: Vec<num_complex::Complex64>,

    /// Provenance proving whether `floquet_multipliers` is complete and
    /// strictly residual-qualified.
    #[cfg_attr(feature = "veriloga", serde(default))]
    pub floquet_evidence: FloquetSpectrumEvidence,

    /// Explicit driven/autonomous policy for interpreting a unity multiplier.
    #[cfg_attr(feature = "veriloga", serde(default))]
    pub floquet_orbit_kind: FloquetOrbitKind,

    /// Explicitly selected autonomous phase mode, when one was qualified
    /// within the unit-circle uncertainty band.
    #[cfg_attr(feature = "veriloga", serde(default))]
    pub trivial_floquet_multiplier_index: Option<usize>,
}

impl PssResult {
    /// Create a new empty PSS result
    pub fn new(period: Value, num_nodes: usize, num_points: usize) -> Self {
        Self {
            period,
            frequency: if period > 0.0 { 1.0 / period } else { 0.0 },
            iterations: 0,
            residual_norm: 0.0,
            time: Vec::with_capacity(num_points),
            waveforms: (0..num_nodes)
                .map(|_| PeriodicWaveform::new(num_points))
                .collect(),
            node_names: (1..=num_nodes).map(|i| format!("N{:03}", i)).collect(),
            period_detected: false,
            floquet_multipliers: Vec::new(),
            floquet_evidence: FloquetSpectrumEvidence::NotComputed,
            floquet_orbit_kind: FloquetOrbitKind::Driven,
            trivial_floquet_multiplier_index: None,
        }
    }

    /// Retain one Floquet spectrum and derive its explicit orbit policy.
    pub fn set_floquet_spectrum(
        &mut self,
        multipliers: Vec<num_complex::Complex64>,
        evidence: FloquetSpectrumEvidence,
        orbit_kind: FloquetOrbitKind,
    ) {
        let trivial_index = if orbit_kind == FloquetOrbitKind::Autonomous
            && matches!(&evidence, FloquetSpectrumEvidence::Qualified { .. })
        {
            select_autonomous_phase_mode(&multipliers)
        } else {
            None
        };
        self.floquet_multipliers = multipliers;
        self.floquet_evidence = evidence;
        self.floquet_orbit_kind = orbit_kind;
        self.trivial_floquet_multiplier_index = trivial_index;
        self.period_detected = orbit_kind == FloquetOrbitKind::Autonomous;
    }

    /// Whether the retained Floquet fields are structurally self-consistent.
    pub fn has_consistent_floquet_contract(&self) -> bool {
        if !self
            .floquet_evidence
            .is_consistent_with(&self.floquet_multipliers)
            || self.period_detected != (self.floquet_orbit_kind == FloquetOrbitKind::Autonomous)
        {
            return false;
        }
        match self.trivial_floquet_multiplier_index {
            None => true,
            Some(index) => {
                self.floquet_orbit_kind == FloquetOrbitKind::Autonomous
                    && matches!(
                        &self.floquet_evidence,
                        FloquetSpectrumEvidence::Qualified { .. }
                    )
                    && index < self.floquet_multipliers.len()
                    && (self.floquet_multipliers[index] - num_complex::Complex64::new(1.0, 0.0))
                        .norm()
                        <= FLOQUET_UNIT_CIRCLE_BAND
            }
        }
    }

    /// Get the periodic waveform for a specific node
    ///
    /// Node indices are 1-based (0 is ground).
    /// Returns None for ground (node 0) or invalid indices.
    pub fn waveform(&self, node: usize) -> Option<&PeriodicWaveform> {
        if node == 0 || node > self.waveforms.len() {
            None
        } else {
            self.waveforms.get(node - 1)
        }
    }

    /// Get voltage at a node and time within the period
    ///
    /// Uses linear interpolation for times between stored points.
    pub fn voltage_at(&self, node: usize, time: Value) -> Value {
        match self.waveform(node) {
            Some(wf) => wf.interpolate(&self.time, time, self.period),
            None => 0.0,
        }
    }

    /// Compute harmonics for a specific node
    ///
    /// Returns DC, fundamental, and harmonics up to max_harmonic.
    pub fn harmonics(&self, node: usize, max_harmonic: usize) -> Vec<HarmonicComponent> {
        match self.waveform(node) {
            Some(wf) => wf.compute_harmonics(&self.time, self.frequency, max_harmonic),
            None => Vec::new(),
        }
    }

    /// Get the DC component for a node
    pub fn dc(&self, node: usize) -> Value {
        match self.waveform(node) {
            Some(wf) => wf.dc(&self.time, self.period),
            None => 0.0,
        }
    }

    /// Get peak-to-peak amplitude for a node
    pub fn peak_to_peak(&self, node: usize) -> Value {
        match self.waveform(node) {
            Some(wf) => wf.peak_to_peak(),
            None => 0.0,
        }
    }

    /// Classify stability from the retained Floquet multipliers.
    ///
    /// Only a complete, finite, strictly qualified spectrum can be Stable,
    /// Unstable, or Marginal. All legacy, uncomputed, or inconsistent data is
    /// Indeterminate.
    pub fn stability_verdict(&self) -> PssStabilityVerdict {
        classify_floquet_stability(
            &self.floquet_multipliers,
            &self.floquet_evidence,
            self.floquet_orbit_kind,
            self.trivial_floquet_multiplier_index,
            FLOQUET_UNIT_CIRCLE_BAND,
        )
    }

    /// Convenience predicate; true only for a known-stable spectrum.
    pub fn is_stable(&self) -> bool {
        self.stability_verdict() == PssStabilityVerdict::Stable
    }

    /// Get number of nodes (excluding ground)
    pub fn num_nodes(&self) -> usize {
        self.waveforms.len()
    }

    /// Get number of time points per period
    pub fn num_points(&self) -> usize {
        self.time.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::FloquetSpectrumCertificate;
    use num_complex::Complex64;

    fn retain_qualified(
        result: &mut PssResult,
        roots: Vec<Complex64>,
        orbit_kind: FloquetOrbitKind,
    ) {
        let certificate = FloquetSpectrumCertificate::new(
            roots.len(),
            0.0,
            FloquetSpectrumCertificate::canonical_qualification_tolerance(roots.len()),
        )
        .unwrap();
        result.set_floquet_spectrum(
            roots,
            FloquetSpectrumEvidence::qualified(certificate).unwrap(),
            orbit_kind,
        );
    }

    #[test]
    fn empty_and_nonfinite_floquet_spectra_are_indeterminate() {
        let mut result = PssResult::new(1.0, 0, 0);
        assert_eq!(
            result.stability_verdict(),
            PssStabilityVerdict::Indeterminate
        );
        assert!(!result.is_stable());

        result.floquet_multipliers = vec![Complex64::new(Value::NAN, 0.0)];
        assert_eq!(
            result.stability_verdict(),
            PssStabilityVerdict::Indeterminate
        );
        assert!(!result.is_stable());
    }

    #[test]
    fn authenticated_driven_result_without_dynamic_modes_is_stable() {
        let mut result = PssResult::new(1.0, 0, 0);
        result.set_floquet_spectrum(
            Vec::new(),
            FloquetSpectrumEvidence::NoDynamicModes,
            FloquetOrbitKind::Driven,
        );
        assert!(result.has_consistent_floquet_contract());
        assert_eq!(result.stability_verdict(), PssStabilityVerdict::Stable);
        assert!(result.is_stable());
    }

    #[test]
    fn finite_floquet_spectra_have_stable_or_unstable_verdicts() {
        let mut result = PssResult::new(1.0, 0, 0);
        result.floquet_multipliers = vec![Complex64::new(0.5, 0.0)];
        assert_eq!(
            result.stability_verdict(),
            PssStabilityVerdict::Indeterminate
        );

        retain_qualified(
            &mut result,
            vec![Complex64::new(0.5, 0.0)],
            FloquetOrbitKind::Driven,
        );
        assert_eq!(result.stability_verdict(), PssStabilityVerdict::Stable);
        assert!(result.is_stable());

        retain_qualified(
            &mut result,
            vec![Complex64::new(1.01, 0.0)],
            FloquetOrbitKind::Driven,
        );
        assert_eq!(result.stability_verdict(), PssStabilityVerdict::Unstable);
        assert!(!result.is_stable());
    }

    #[test]
    fn unit_circle_is_marginal_unless_one_autonomous_phase_mode_is_exempted() {
        let mut result = PssResult::new(1.0, 0, 0);
        retain_qualified(
            &mut result,
            vec![Complex64::new(1.0, 0.0)],
            FloquetOrbitKind::Driven,
        );
        assert_eq!(result.stability_verdict(), PssStabilityVerdict::Marginal);
        assert!(!result.is_stable());

        retain_qualified(
            &mut result,
            vec![Complex64::new(1.0, 0.0)],
            FloquetOrbitKind::Autonomous,
        );
        assert_eq!(result.stability_verdict(), PssStabilityVerdict::Stable);

        retain_qualified(
            &mut result,
            vec![Complex64::new(1.0, 0.0), Complex64::new(1.0, 0.0)],
            FloquetOrbitKind::Autonomous,
        );
        assert_eq!(
            result.stability_verdict(),
            PssStabilityVerdict::Marginal,
            "at most one autonomous phase mode may be exempted"
        );
    }

    #[test]
    fn autonomous_phase_exemption_cannot_hide_an_outward_root() {
        let mut result = PssResult::new(1.0, 0, 0);
        retain_qualified(
            &mut result,
            vec![Complex64::new(1.0005, 0.0), Complex64::new(0.5, 0.0)],
            FloquetOrbitKind::Autonomous,
        );

        assert_eq!(
            result.stability_verdict(),
            PssStabilityVerdict::Indeterminate,
            "an autonomous result without a qualified unity phase mode is not classifiable"
        );
        assert!(!result.is_stable());
    }

    #[test]
    fn inconsistent_trivial_index_is_indeterminate() {
        let mut result = PssResult::new(1.0, 0, 0);
        retain_qualified(
            &mut result,
            vec![Complex64::new(0.5, 0.0)],
            FloquetOrbitKind::Driven,
        );
        result.trivial_floquet_multiplier_index = Some(0);
        assert!(!result.has_consistent_floquet_contract());
        assert_eq!(
            result.stability_verdict(),
            PssStabilityVerdict::Indeterminate
        );
    }
}

/// Periodic waveform for a single node
///
/// Stores one complete period of the waveform and provides
/// interpolation and harmonic analysis.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "veriloga", derive(serde::Serialize, serde::Deserialize))]
pub struct PeriodicWaveform {
    /// Voltage values at each time point
    pub values: Vec<Value>,
}

impl PeriodicWaveform {
    /// Create a new waveform with given capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
        }
    }

    /// Create from existing values
    pub fn from_values(values: Vec<Value>) -> Self {
        Self { values }
    }

    /// Get peak-to-peak amplitude
    pub fn peak_to_peak(&self) -> Value {
        if self.values.is_empty() {
            return 0.0;
        }
        let min = self.values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = self
            .values
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        max - min
    }

    /// Get DC (average) value
    pub fn dc(&self, time: &[Value], period: Value) -> Value {
        if self.values.len() < 2 || time.len() != self.values.len() {
            return if self.values.is_empty() {
                0.0
            } else {
                self.values[0]
            };
        }

        // Trapezoidal integration normalized by period
        let mut integral = 0.0;
        for i in 1..self.values.len() {
            let dt = time[i] - time[i - 1];
            integral += 0.5 * (self.values[i] + self.values[i - 1]) * dt;
        }
        integral / period
    }

    /// Interpolate value at arbitrary time within period
    ///
    /// Handles wraparound: time values outside [0, period] are mapped
    /// to the equivalent point within the period.
    pub fn interpolate(&self, time_grid: &[Value], t: Value, period: Value) -> Value {
        if self.values.is_empty() || time_grid.is_empty() {
            return 0.0;
        }

        // Wrap time to [0, period)
        let t_wrapped = ((t % period) + period) % period;

        // Binary search for bracketing indices
        let idx = time_grid.partition_point(|&x| x < t_wrapped);

        if idx == 0 {
            return self.values[0];
        }
        if idx >= time_grid.len() {
            return self.values[self.values.len() - 1];
        }

        // Linear interpolation
        let t0 = time_grid[idx - 1];
        let t1 = time_grid[idx];
        let v0 = self.values[idx - 1];
        let v1 = self.values[idx];

        if (t1 - t0).abs() < 1e-15 {
            return v0;
        }

        let alpha = (t_wrapped - t0) / (t1 - t0);
        v0 + alpha * (v1 - v0)
    }

    /// Compute harmonic components using DFT
    pub(crate) fn compute_harmonics(
        &self,
        time: &[Value],
        fundamental_freq: Value,
        max_harmonic: usize,
    ) -> Vec<HarmonicComponent> {
        if self.values.len() < 2 || time.len() != self.values.len() {
            return Vec::new();
        }

        let period = if fundamental_freq > 0.0 {
            1.0 / fundamental_freq
        } else {
            time.last().copied().unwrap_or(1.0) - time.first().copied().unwrap_or(0.0)
        };

        let t_start = time.first().copied().unwrap_or(0.0);

        let mut harmonics = Vec::with_capacity(max_harmonic + 1);

        for n in 0..=max_harmonic {
            let freq = n as f64 * fundamental_freq;

            if n == 0 {
                // DC component
                let dc = self.dc(time, period);
                harmonics.push(HarmonicComponent {
                    harmonic_number: 0,
                    frequency: 0.0,
                    magnitude: dc,
                    phase: 0.0,
                });
            } else {
                // AC components via trapezoidal integration
                let omega = 2.0 * PI * freq;

                let mut a_n = 0.0;
                let mut b_n = 0.0;

                for i in 1..time.len() {
                    let t0 = time[i - 1] - t_start;
                    let t1 = time[i] - t_start;
                    let v0 = self.values[i - 1];
                    let v1 = self.values[i];
                    let dt = t1 - t0;

                    // Trapezoidal integration for cos and sin integrals
                    a_n += 0.5 * dt * (v0 * (omega * t0).cos() + v1 * (omega * t1).cos());
                    b_n += 0.5 * dt * (v0 * (omega * t0).sin() + v1 * (omega * t1).sin());
                }

                a_n *= 2.0 / period;
                b_n *= 2.0 / period;

                let magnitude = (a_n * a_n + b_n * b_n).sqrt();
                let phase = (-b_n).atan2(a_n) * 180.0 / PI;

                harmonics.push(HarmonicComponent {
                    harmonic_number: n,
                    frequency: freq,
                    magnitude,
                    phase,
                });
            }
        }

        harmonics
    }
}
