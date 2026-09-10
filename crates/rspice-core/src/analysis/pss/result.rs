//! PSS Analysis Results
//!
//! Data structures for storing and accessing PSS analysis results.

use crate::Value;
use crate::abort_signal::NoAbort;
use crate::analysis::fourier::{FourierQuadrature, HarmonicComponent};
use crate::analysis::{
    FLOQUET_UNIT_CIRCLE_BAND, FloquetOrbitKind, FloquetSpectrumEvidence, FloquetStabilityVerdict,
    classify_floquet_stability, select_autonomous_phase_mode,
};

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

    /// Canonical MNA branch names, in the same order as `branch_waveforms`.
    #[cfg_attr(feature = "veriloga", serde(default))]
    pub branch_names: Vec<String>,

    /// Accepted branch-current samples on `time`, preserving solver polarity.
    #[cfg_attr(feature = "veriloga", serde(default))]
    pub branch_waveforms: Vec<PeriodicWaveform>,

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
            branch_names: Vec::new(),
            branch_waveforms: Vec::new(),
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
    /// Invalid waveform evidence, nonrepresentable coefficients, or an
    /// unallocatable harmonic count yield an empty spectrum.
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

    #[test]
    fn periodic_interpolation_preserves_phase_across_time_scales() {
        let waveform = PeriodicWaveform::from_values(vec![0.0, 1.0, 0.0, -1.0, 0.0]);
        for period in [1e-300, 1e-18, 1e-9, 1.0, 1e300] {
            let time = [0.0, period * 0.25, period * 0.5, period * 0.75, period];
            for (phase, expected) in [
                (0.125, 0.5),
                (0.375, 0.5),
                (0.625, -0.5),
                (0.875, -0.5),
                (-0.125, -0.5),
                (1.125, 0.5),
            ] {
                let actual = waveform.interpolate(&time, phase * period, period);
                assert!(
                    (actual - expected).abs() < 2e-15,
                    "{period:e}, {phase}: {actual}"
                );
            }
            for (&t, &value) in time.iter().zip(&waveform.values) {
                assert_eq!(waveform.interpolate(&time, t, period), value);
            }
        }
    }

    #[test]
    fn periodic_interpolation_keeps_representable_phases_at_the_seam() {
        let before_end = 1.0_f64.next_down();
        let time = [0.0, 1e-20, 0.5, before_end, 1.0];
        let waveform = PeriodicWaveform::from_values(vec![0.0, 1.0, 0.0, 1.0, 0.0]);
        assert_eq!(waveform.interpolate(&time, 1e-20, 1.0), 1.0);
        assert_eq!(waveform.interpolate(&time, before_end, 1.0), 1.0);
        assert_eq!(waveform.interpolate(&time, before_end - 1.0, 1.0), 1.0);
        assert_eq!(waveform.interpolate(&time, 1.0, 1.0), 0.0);
    }

    #[test]
    fn periodic_interpolation_avoids_overflow_between_finite_samples() {
        let waveform = PeriodicWaveform::from_values(vec![Value::MAX, -Value::MAX, Value::MAX]);
        let time = [0.0, 0.5, 1.0];
        assert_eq!(waveform.interpolate(&time, 0.25, 1.0), 0.0);
        assert_eq!(waveform.interpolate(&time, 0.5, 1.0), -Value::MAX);
        assert_eq!(waveform.interpolate(&time, 0.75, 1.0), 0.0);
        let quarter = waveform.interpolate(&time, 0.125, 1.0);
        assert!((quarter / Value::MAX - 0.5).abs() <= Value::EPSILON);
    }

    #[test]
    fn periodic_interpolation_reports_invalid_inputs_without_panicking() {
        let waveform = PeriodicWaveform::from_values(vec![0.0, 1.0]);
        for (time, query, period) in [
            (vec![], 0.75, 1.0),
            (vec![0.0, 0.5, 1.0], 0.75, 1.0),
            (vec![0.0, 1.0], Value::NAN, 1.0),
            (vec![0.0, 1.0], Value::INFINITY, 1.0),
            (vec![0.0, 1.0], 0.5, 0.0),
            (vec![0.0, 1.0], 0.5, -1.0),
            (vec![0.0, 1.0], 0.5, Value::INFINITY),
        ] {
            assert!(waveform.interpolate(&time, query, period).is_nan());
        }
    }

    #[test]
    fn periodic_dc_normalizes_before_multiplying_physical_scales() {
        for period in [1e-300, 1.0, 1e300] {
            for amplitude in [1e-300, 1.0, Value::MAX] {
                let waveform = PeriodicWaveform::from_values(vec![amplitude; 3]);
                let dc = waveform.dc(&[0.0, period * 0.5, period], period);
                assert!(
                    (dc / amplitude - 1.0).abs() < 2e-15,
                    "{period:e}, {amplitude:e}: {dc}"
                );
            }
        }
    }

    #[test]
    fn periodic_dc_keeps_finite_area_when_a_fractional_interval_underflows() {
        let waveform = PeriodicWaveform::from_values(vec![0.0, 1e300, 0.0, 0.0]);
        let dc = waveform.dc(&[0.0, 1e-300, 2e-300, 1e300], 1e300);
        assert!((dc / 1e-300 - 1.0).abs() < 2e-15, "{dc:e}");
    }

    #[test]
    fn periodic_harmonics_preserve_small_and_large_finite_coefficients() {
        for frequency in [1e-300, 1.0, 1e300, 1e308] {
            let period = 1.0 / frequency;
            let time: Vec<_> = (0..=128)
                .map(|index| (index as Value / 128.0) * period)
                .collect();
            for amplitude in [1e-300, 1.0, 1e300] {
                let waveform = PeriodicWaveform::from_values(
                    (0..=128)
                        .map(|index| {
                            amplitude
                                * (0.25 + (std::f64::consts::TAU * index as Value / 128.0).sin())
                        })
                        .collect(),
                );
                let harmonics = waveform.compute_harmonics(&time, frequency, 1);
                assert_eq!(harmonics.len(), 2, "{frequency:e}, {amplitude:e}");
                assert!(
                    (harmonics[0].magnitude / amplitude - 0.25).abs() < 2e-14,
                    "DC: {frequency:e}, {amplitude:e}: {:?}",
                    harmonics[0]
                );
                assert!(
                    (harmonics[1].magnitude / amplitude - 1.0).abs() < 2e-14,
                    "AC: {frequency:e}, {amplitude:e}: {:?}",
                    harmonics[1]
                );
                assert!(
                    (harmonics[1].phase + 90.0).abs() < 2e-12,
                    "phase: {frequency:e}, {amplitude:e}: {:?}",
                    harmonics[1]
                );
            }
        }
    }

    #[test]
    fn periodic_spectrum_rejects_invalid_evidence_and_unrepresentable_capacity() {
        let waveform = PeriodicWaveform::from_values(vec![0.0, 1.0, 0.0]);
        let time = [0.0, 0.5, 1.0];
        assert!(
            waveform
                .compute_harmonics(&time, 1.0, usize::MAX)
                .is_empty()
        );
        assert!(waveform.compute_harmonics(&time, 0.0, 1).is_empty());
        assert!(waveform.compute_harmonics(&[0.0, 1.0], 1.0, 1).is_empty());
        assert!(waveform.dc(&[0.0, 1.0], 1.0).is_nan());
        assert!(waveform.dc(&time, 0.0).is_nan());
    }

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

    /// Get DC (average) value. Empty data yields zero and a single sample
    /// yields its value. Invalid time/value evidence yields NaN.
    pub fn dc(&self, time: &[Value], period: Value) -> Value {
        if self.values.len() < 2 && time.len() == self.values.len() {
            return if self.values.is_empty() {
                0.0
            } else {
                self.values[0]
            };
        }

        FourierQuadrature::new(time, &self.values, period, &NoAbort)
            .and_then(|quadrature| quadrature.component(0.0, 0, &NoAbort))
            .map_or(Value::NAN, |(dc, _)| dc)
    }

    /// Interpolate value at arbitrary time within period
    ///
    /// Handles wraparound: time values outside [0, period] are mapped
    /// to the equivalent point within the period.
    /// The time grid must be finite and strictly increasing. Empty data
    /// yields zero; mismatched lengths or invalid query/period values yield NaN.
    pub fn interpolate(&self, time_grid: &[Value], t: Value, period: Value) -> Value {
        if self.values.len() != time_grid.len() {
            return Value::NAN;
        }
        if self.values.is_empty() {
            return 0.0;
        }
        if !t.is_finite() || !period.is_finite() || period <= 0.0 {
            return Value::NAN;
        }

        // Adding a period to an already positive remainder can erase an
        // early phase or round the last instant before the seam to zero.
        let t_wrapped = t.rem_euclid(period);

        // Binary search for bracketing indices
        let idx = time_grid.partition_point(|&x| x < t_wrapped);

        if idx == 0 {
            return self.values[0];
        }
        if idx >= time_grid.len() {
            return self.values[self.values.len() - 1];
        }
        if time_grid[idx] == t_wrapped {
            return self.values[idx];
        }

        // Linear interpolation
        let t0 = time_grid[idx - 1];
        let t1 = time_grid[idx];
        let v0 = self.values[idx - 1];
        let v1 = self.values[idx];

        let interval = t1 - t0;
        if !interval.is_finite() || interval <= 0.0 || !v0.is_finite() || !v1.is_finite() {
            return Value::NAN;
        }

        let alpha = (t_wrapped - t0) / interval;
        // Opposite finite endpoints can have an infinite difference even
        // though every interpolated value lies within their finite range.
        if v0.is_sign_positive() != v1.is_sign_positive() {
            (1.0 - alpha) * v0 + alpha * v1
        } else {
            v0 + alpha * (v1 - v0)
        }
    }

    /// Compute harmonic components by normalized trapezoidal quadrature.
    /// Invalid evidence, frequency, or allocation yields an empty spectrum.
    pub(crate) fn compute_harmonics(
        &self,
        time: &[Value],
        fundamental_freq: Value,
        max_harmonic: usize,
    ) -> Vec<HarmonicComponent> {
        if !fundamental_freq.is_finite() || fundamental_freq <= 0.0 {
            return Vec::new();
        }
        let Ok(quadrature) =
            FourierQuadrature::new(time, &self.values, 1.0 / fundamental_freq, &NoAbort)
        else {
            return Vec::new();
        };
        let Some(count) = max_harmonic.checked_add(1) else {
            return Vec::new();
        };
        let mut harmonics = Vec::new();
        if harmonics.try_reserve_exact(count).is_err() {
            return harmonics;
        }
        for n in 0..=max_harmonic {
            let freq = n as f64 * fundamental_freq;
            let Ok((magnitude, phase)) = quadrature.component(freq, n, &NoAbort) else {
                return Vec::new();
            };
            harmonics.push(HarmonicComponent {
                harmonic_number: n,
                frequency: freq,
                magnitude,
                phase,
            });
        }

        harmonics
    }
}
