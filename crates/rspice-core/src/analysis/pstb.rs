//! Periodic Stability (PSTB) Analysis Module
//!
//! PSTB analysis evaluates the stability of periodic steady-state solutions
//! using Floquet theory. This is critical for:
//!
//! - **Oscillator design**: Verify stable oscillation and startup conditions
//! - **PLL stability**: Check loop stability around limit cycles
//! - **Bifurcation detection**: Find parameter values where stability changes
//! - **Parametric oscillation**: Detect unintended oscillation modes
//!
//! # Theory
//!
//! For a periodic orbit x*(t) with period T, small perturbations δx(t) evolve as:
//!
//! δx(t) ≈ Φ(t,0) · δx(0)
//!
//! where Φ(t,0) is the state transition (Monodromy) matrix. The eigenvalues of
//! Φ(T,0) are called **Floquet multipliers** (λ). Stability is determined by:
//!
//! - |λ| < 1 for all λ (except one λ = 1 for autonomous systems): Stable
//! - |λ| > 1 for any λ: Unstable
//! - |λ| = 1 (beyond the trivial one): Marginally stable / bifurcation point
//!
//! # Floquet Exponents
//!
//! Floquet exponents μ relate to multipliers as: λ = exp(μ·T)
//!
//! For oscillators, one μ = 0 always (corresponding to phase shifts along orbit).
//! The other exponents determine amplitude stability.

#![allow(clippy::needless_range_loop)]
use crate::abort_signal::AbortSignal;
use crate::numerics::eigenspectrum::{OrdinarySpectrumError, qualified_real_eigenspectrum};
use crate::{SimulationError, Value};
use num_complex::Complex64;
use std::f64::consts::PI;

//=============================================================================
// PSTB Configuration
//=============================================================================

/// Configuration for Periodic Stability (PSTB) analysis
#[derive(Debug, Clone)]
pub struct PstbConfig {
    /// Compatibility field. PSTB always retains the complete spectrum because
    /// a truncated spectrum cannot prove stability.
    pub num_eigenvalues: usize,

    /// Compatibility field. Strict qualification always uses the canonical
    /// `128 * n * EPSILON` backward-error bound.
    pub eigenvalue_tolerance: Value,

    /// Compatibility field. The faer eigensolve is atomic and does not expose
    /// an iteration limit.
    pub max_iterations: usize,

    /// Compatibility field. Exponents remain populated to preserve the public
    /// result schema; callers may ignore them when this is false.
    pub compute_exponents: bool,

    /// Whether to compute eigenvectors (mode shapes)
    pub compute_eigenvectors: bool,

    /// Outer numerical band around the physical `|λ| = 1` boundary.
    /// Values must be finite and at least one. The symmetric inner bound is
    /// `max(0, 2 - stability_threshold)`; only roots below it are stable.
    pub stability_threshold: Value,

    /// Enable subharmonic detection (multipliers at roots of unity).
    pub detect_subharmonics: bool,

    /// Numerical perturbation for finite difference Monodromy computation
    pub fd_perturbation: Value,
}

impl Default for PstbConfig {
    fn default() -> Self {
        Self {
            num_eigenvalues: 0, // Compute all
            eigenvalue_tolerance: 1e-10,
            max_iterations: 1000,
            compute_exponents: true,
            compute_eigenvectors: false,
            stability_threshold: 1.0 + 1e-6, // Allow small margin for numerical error
            detect_subharmonics: true,
            fd_perturbation: 1e-8,
        }
    }
}

impl PstbConfig {
    /// Create new PSTB configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set number of eigenvalues to compute
    pub fn with_num_eigenvalues(mut self, n: usize) -> Self {
        self.num_eigenvalues = n;
        self
    }

    /// Enable/disable eigenvector computation
    pub fn with_eigenvectors(mut self, compute: bool) -> Self {
        self.compute_eigenvectors = compute;
        self
    }

    /// Set stability threshold
    pub fn with_stability_threshold(mut self, threshold: Value) -> Self {
        self.stability_threshold = threshold;
        self
    }

    /// Enable/disable subharmonic detection
    pub fn with_subharmonic_detection(mut self, detect: bool) -> Self {
        self.detect_subharmonics = detect;
        self
    }

    /// Set convergence tolerance
    pub fn with_tolerance(mut self, tol: Value) -> Self {
        self.eigenvalue_tolerance = tol;
        self
    }
}

//=============================================================================
// Floquet Multiplier
//=============================================================================

/// A Floquet multiplier with associated metadata
#[derive(Debug, Clone)]
pub struct FloquetMultiplier {
    /// Complex eigenvalue (Floquet multiplier)
    pub value: Complex64,

    /// Floquet exponent: μ = ln(λ)/T.
    ///
    /// Exact-zero multipliers are rejected because a finite-dimensional
    /// continuous-time state-transition map must be nonsingular.
    pub exponent: Complex64,

    /// Index in sorted order (0 = most unstable)
    pub index: usize,

    /// Whether this multiplier indicates instability
    pub is_unstable: bool,

    /// Whether this is a near-unity multiplier that could be the phase mode of
    /// an autonomous orbit. This flag is descriptive only: PSTB has no orbit-
    /// type evidence yet, so the mode is never exempted from classification.
    pub is_trivial: bool,

    /// Subharmonic order if |λ| ≈ 1 and λ is near a root of unity
    pub subharmonic_order: Option<usize>,

    /// Associated eigenvector (if computed)
    pub eigenvector: Option<Vec<Complex64>>,
}

impl FloquetMultiplier {
    /// Create a new Floquet multiplier
    pub fn new(value: Complex64, period: Value, stability_threshold: Value) -> Self {
        let exponent = value.ln() / period;
        let magnitude = value.norm();

        // Check if this is the trivial multiplier (λ ≈ 1)
        let is_trivial = (value - Complex64::new(1.0, 0.0)).norm() < 1e-6;

        // Check for subharmonic (λ near n-th root of unity)
        let subharmonic_order = Self::detect_subharmonic(&value);

        Self {
            value,
            exponent,
            index: 0,
            is_unstable: magnitude > stability_threshold,
            is_trivial,
            subharmonic_order,
            eigenvector: None,
        }
    }

    /// Detect if multiplier is near a root of unity (subharmonic)
    fn detect_subharmonic(value: &Complex64) -> Option<usize> {
        let mag = value.norm();
        if (mag - 1.0).abs() > 0.01 {
            return None; // Not on unit circle
        }

        let angle = value.arg().abs();

        // Check for n-th roots of unity (n = 2, 3, 4, ...)
        for n in 2..=8 {
            let expected_angle = 2.0 * PI / (n as f64);
            if (angle - expected_angle).abs() < 0.01 {
                return Some(n);
            }
        }

        None
    }

    /// Get magnitude of multiplier
    pub fn magnitude(&self) -> Value {
        self.value.norm()
    }

    /// Get phase angle in radians
    pub fn phase(&self) -> Value {
        self.value.arg()
    }

    /// Get phase angle in degrees
    pub fn phase_degrees(&self) -> Value {
        self.value.arg() * 180.0 / PI
    }

    /// Get damping factor (negative real part of exponent)
    pub fn damping(&self) -> Value {
        -self.exponent.re
    }

    /// Get natural frequency from exponent
    pub fn natural_frequency(&self) -> Value {
        self.exponent.im.abs() / (2.0 * PI)
    }

    /// Stability margin in dB (20·log10(1/|λ|)).
    pub fn stability_margin_db(&self) -> Value {
        -20.0 * self.magnitude().log10()
    }
}

//=============================================================================
// Stability Classification
//=============================================================================

/// Classification of periodic orbit stability
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StabilityType {
    /// All multipliers are strictly inside the configured inner band.
    Stable,

    /// At least one multiplier outside unit circle with positive real part
    UnstableReal,

    /// Pair of complex multipliers outside unit circle
    UnstableComplex,

    /// Multiplier at λ = -1 (period-doubling bifurcation)
    PeriodDoubling,

    /// Complex pair on unit circle (Neimark-Sacker/torus bifurcation)
    NeimarkSacker,

    /// Multiplier at λ = +1 (saddle-node bifurcation)
    SaddleNode,

    /// Marginal stability (numerical uncertainty)
    Marginal,
}

impl StabilityType {
    /// Check if this is a stable classification
    pub fn is_stable(&self) -> bool {
        matches!(self, StabilityType::Stable)
    }
}

//=============================================================================
// PSTB Result
//=============================================================================

/// Result of Periodic Stability (PSTB) analysis
#[derive(Debug, Clone)]
pub struct PstbResult {
    /// Period of the analyzed orbit
    pub period: Value,

    /// Fundamental frequency (1/period)
    pub fundamental_frequency: Value,

    /// All computed Floquet multipliers (sorted by magnitude, descending)
    pub multipliers: Vec<FloquetMultiplier>,

    /// Overall stability classification
    pub stability: StabilityType,

    /// Monodromy matrix (state transition over one period)
    pub monodromy: Vec<Vec<Value>>,

    /// Number of unstable multipliers
    pub num_unstable: usize,

    /// Minimum stability margin (for stable systems)
    pub min_stability_margin_db: Value,

    /// Maximum multiplier magnitude
    pub max_multiplier_magnitude: Value,

    /// Detected subharmonics (if any)
    pub subharmonics: Vec<usize>,

    /// Whether the analysis converged
    pub converged: bool,

    /// Number of iterations used. Zero means the atomic eigensolver did not
    /// expose an iteration count.
    pub iterations: usize,
}

impl PstbResult {
    /// Check if the periodic orbit is stable
    pub fn is_stable(&self) -> bool {
        self.stability.is_stable()
    }
}

//=============================================================================
// PSTB Analyzer
//=============================================================================

/// Periodic Stability (PSTB) Analyzer
///
/// Analyzes stability of periodic orbits using Floquet theory.
#[derive(Debug)]
pub struct PstbAnalyzer {
    /// Configuration
    config: PstbConfig,

    /// State dimension
    dimension: usize,
}

impl PstbAnalyzer {
    /// Create a new PSTB analyzer
    pub fn new(config: PstbConfig) -> Self {
        Self {
            config,
            dimension: 0,
        }
    }

    /// Analyze stability from a pre-computed Monodromy matrix, cooperatively
    /// returning [`SimulationError::Aborted`] when cancellation is requested.
    ///
    /// Cancellation is polled throughout matrix preparation, conversion,
    /// residual qualification, and result conversion, and immediately before
    /// and after the atomic eigensolve.
    pub fn analyze_monodromy_with_abort(
        &mut self,
        monodromy: &[Vec<Value>],
        period: Value,
        abort: &dyn AbortSignal,
    ) -> Result<PstbResult, SimulationError> {
        ensure_not_aborted(abort)?;
        if !period.is_finite() || period <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "PSTB requires a finite positive period, got {period}"
            )));
        }
        let fundamental_frequency = 1.0 / period;
        if !fundamental_frequency.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "PSTB period is too small to form a finite fundamental frequency: {period}"
            )));
        }
        if !self.config.stability_threshold.is_finite() || self.config.stability_threshold < 1.0 {
            return Err(SimulationError::Circuit(format!(
                "PSTB requires a finite stability threshold greater than or equal to one, got {}",
                self.config.stability_threshold
            )));
        }

        let spectrum = qualified_real_eigenspectrum(monodromy, abort)
            .map_err(map_eigenspectrum_error_for_pstb)?;
        let n = spectrum.certificate.problem_order;
        debug_assert!(
            spectrum.certificate.max_backward_error <= spectrum.certificate.qualification_tolerance
        );
        self.dimension = n;

        // Create FloquetMultiplier objects
        let mut multipliers = Vec::with_capacity(spectrum.eigenvalues.len());
        for (i, &ev) in spectrum.eigenvalues.iter().enumerate() {
            poll_periodically(abort, i)?;
            if ev.re == 0.0 && ev.im == 0.0 {
                return Err(SimulationError::Circuit(format!(
                    "PSTB Floquet multiplier {i} is exactly zero; a continuous-time state-transition map must be nonsingular"
                )));
            }
            let mut fm = FloquetMultiplier::new(ev, period, self.config.stability_threshold);
            if !fm.exponent.re.is_finite() || !fm.exponent.im.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "PSTB Floquet exponent {i} is non-finite"
                )));
            }
            fm.index = i;
            if self.config.compute_eigenvectors {
                fm.eigenvector = Some(spectrum.right_eigenvectors[i].clone());
            }
            multipliers.push(fm);
        }

        // Sort by magnitude (most unstable first)
        ensure_not_aborted(abort)?;
        multipliers.sort_by(|a, b| {
            b.magnitude()
                .total_cmp(&a.magnitude())
                .then_with(|| a.value.re.total_cmp(&b.value.re))
                .then_with(|| a.value.im.total_cmp(&b.value.im))
        });
        ensure_not_aborted(abort)?;

        // Re-index after sorting
        for (i, m) in multipliers.iter_mut().enumerate() {
            poll_periodically(abort, i)?;
            m.index = i;
        }

        // Classify stability
        let stability = self.classify_stability_with_abort(&multipliers, abort)?;

        // Count unstable and compute margins
        let mut num_unstable = 0;
        for (index, multiplier) in multipliers.iter().enumerate() {
            poll_periodically(abort, index)?;
            if multiplier.is_unstable {
                num_unstable += 1;
            }
        }
        let max_magnitude = multipliers.first().map(|m| m.magnitude()).unwrap_or(0.0);

        let min_margin_db = if num_unstable == 0 && !multipliers.is_empty() {
            // Include every mode. A near-unity mode is not exempt without
            // explicit evidence that the orbit is autonomous.
            let mut minimum = f64::INFINITY;
            for (index, multiplier) in multipliers.iter().enumerate() {
                poll_periodically(abort, index)?;
                let margin = multiplier.stability_margin_db();
                if margin.total_cmp(&minimum).is_lt() {
                    minimum = margin;
                }
            }
            minimum
        } else {
            -max_magnitude.log10() * 20.0 // Negative margin = unstable
        };

        // Detect subharmonics
        let mut subharmonics = Vec::new();
        for (index, multiplier) in multipliers.iter().enumerate() {
            poll_periodically(abort, index)?;
            if self.config.detect_subharmonics
                && let Some(order) = multiplier.subharmonic_order
            {
                subharmonics.push(order);
            }
        }

        let mut monodromy_copy = Vec::with_capacity(monodromy.len());
        let mut flat_index = 0;
        for row in monodromy {
            let mut copied_row = Vec::with_capacity(row.len());
            for &value in row {
                poll_periodically(abort, flat_index)?;
                flat_index += 1;
                copied_row.push(value);
            }
            monodromy_copy.push(copied_row);
        }
        ensure_not_aborted(abort)?;

        Ok(PstbResult {
            period,
            fundamental_frequency,
            multipliers,
            stability,
            monodromy: monodromy_copy,
            num_unstable,
            min_stability_margin_db: min_margin_db,
            max_multiplier_magnitude: max_magnitude,
            subharmonics,
            // The qualified eigenspectrum helper returns only after every
            // eigenpair has passed the canonical residual criterion.
            converged: true,
            iterations: 0,
        })
    }

    /// Classify stability based on multipliers
    fn classify_stability_with_abort(
        &self,
        multipliers: &[FloquetMultiplier],
        abort: &dyn AbortSignal,
    ) -> Result<StabilityType, SimulationError> {
        let mut unstable = Vec::new();
        for (index, multiplier) in multipliers.iter().enumerate() {
            poll_periodically(abort, index)?;
            if multiplier.is_unstable {
                unstable.push(multiplier);
            }
        }

        if unstable.is_empty() {
            let stable_inner_bound = (2.0 - self.config.stability_threshold).max(0.0);
            // Check for bifurcations at unit circle
            for (index, m) in multipliers.iter().enumerate() {
                poll_periodically(abort, index)?;
                if m.is_trivial {
                    // There is no explicit autonomous-orbit evidence in this
                    // API, so +1 cannot be silently discarded as a phase mode.
                    return Ok(StabilityType::SaddleNode);
                }
                let mag = m.magnitude();
                let imag = m.value.im.abs();

                // Near λ = -1: period doubling
                if (m.value + Complex64::new(1.0, 0.0)).norm() < 0.01 {
                    return Ok(StabilityType::PeriodDoubling);
                }

                // Near λ = +1: saddle-node. (The tighter is_trivial band was
                // handled above.)
                if (m.value - Complex64::new(1.0, 0.0)).norm() < 0.01 {
                    return Ok(StabilityType::SaddleNode);
                }

                // Complex pair near unit circle: Neimark-Sacker
                if (mag - 1.0).abs() < 0.01 && imag > 0.01 {
                    return Ok(StabilityType::NeimarkSacker);
                }

                // Everything at or above the symmetric inner band is
                // marginal unless a more specific bifurcation matched above.
                if mag >= stable_inner_bound {
                    return Ok(StabilityType::Marginal);
                }
            }

            Ok(StabilityType::Stable)
        } else {
            // There are unstable multipliers
            let dominant = &unstable[0];

            if dominant.value.im.abs() > 0.01 {
                Ok(StabilityType::UnstableComplex)
            } else {
                Ok(StabilityType::UnstableReal)
            }
        }
    }
}

fn map_eigenspectrum_error_for_pstb(error: OrdinarySpectrumError) -> SimulationError {
    match error {
        OrdinarySpectrumError::Aborted => SimulationError::Aborted,
        error => SimulationError::Circuit(format!(
            "PSTB Floquet spectrum qualification failed: {error}"
        )),
    }
}

#[inline]
fn ensure_not_aborted(abort: &dyn AbortSignal) -> Result<(), SimulationError> {
    if abort.is_aborted() {
        Err(SimulationError::Aborted)
    } else {
        Ok(())
    }
}

#[inline]
fn poll_periodically(abort: &dyn AbortSignal, index: usize) -> Result<(), SimulationError> {
    const ABORT_POLL_STRIDE: usize = 64;
    if index % ABORT_POLL_STRIDE == 0 {
        ensure_not_aborted(abort)?;
    }
    Ok(())
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::{CountingAbort, ImmediateAbort, NoAbort};

    #[test]
    fn pstb_eigenanalysis_honors_entry_abort() {
        let mut analyzer = PstbAnalyzer::new(PstbConfig::default());
        let result = analyzer.analyze_monodromy_with_abort(
            &[vec![1.0, 0.0], vec![0.0, 0.5]],
            1.0,
            &ImmediateAbort,
        );

        assert!(matches!(result, Err(SimulationError::Aborted)));
    }

    #[test]
    fn pstb_eigenanalysis_honors_abort_during_validation() {
        const DIMENSION: usize = 16;
        let mut monodromy = vec![vec![0.0; DIMENSION]; DIMENSION];
        for (row, values) in monodromy.iter_mut().enumerate() {
            values[row] = 0.9 - row as Value * 0.001;
            values[(row + 1) % DIMENSION] = 0.01;
        }

        let abort = CountingAbort::new(2);
        let mut analyzer = PstbAnalyzer::new(PstbConfig::default());
        let result = analyzer.analyze_monodromy_with_abort(&monodromy, 1e-6, &abort);

        assert!(matches!(result, Err(SimulationError::Aborted)));
        assert!(abort.count() > 2);
    }

    #[test]
    fn pstb_rejects_invalid_period_and_threshold() {
        for period in [0.0, -1.0, Value::NAN, Value::INFINITY, Value::from_bits(1)] {
            let mut analyzer = PstbAnalyzer::new(PstbConfig::default());
            assert!(
                analyzer
                    .analyze_monodromy_with_abort(&[vec![0.5]], period, &NoAbort)
                    .is_err(),
                "period {period:?} must be rejected"
            );
        }

        for threshold in [0.999, 0.0, -1.0, Value::NAN, Value::INFINITY] {
            let mut config = PstbConfig::default();
            config.stability_threshold = threshold;
            let mut analyzer = PstbAnalyzer::new(config);
            assert!(
                analyzer
                    .analyze_monodromy_with_abort(&[vec![0.5]], 1.0, &NoAbort)
                    .is_err(),
                "threshold {threshold:?} must be rejected"
            );
        }
    }

    #[test]
    fn pstb_rejects_empty_ragged_and_nonfinite_monodromy() {
        let invalid = [
            Vec::<Vec<Value>>::new(),
            vec![vec![1.0, 0.0], vec![0.0]],
            vec![vec![Value::NAN]],
        ];
        for monodromy in invalid {
            let mut analyzer = PstbAnalyzer::new(PstbConfig::default());
            assert!(
                analyzer
                    .analyze_monodromy_with_abort(&monodromy, 1.0, &NoAbort)
                    .is_err()
            );
        }
    }

    #[test]
    fn pstb_rejects_an_exact_zero_multiplier() {
        let mut analyzer = PstbAnalyzer::new(PstbConfig::default());
        let error = analyzer
            .analyze_monodromy_with_abort(&[vec![0.0]], 1.0, &NoAbort)
            .unwrap_err();

        assert!(
            matches!(error, SimulationError::Circuit(message) if message.contains("exactly zero"))
        );
    }

    #[test]
    fn near_unity_mode_is_not_exempt_without_autonomous_evidence() {
        let mut analyzer = PstbAnalyzer::new(PstbConfig::default());
        let result = analyzer
            .analyze_monodromy_with_abort(&[vec![1.0]], 1.0, &NoAbort)
            .unwrap();

        assert_eq!(result.stability, StabilityType::SaddleNode);
        assert!(!result.is_stable());
        assert_eq!(result.min_stability_margin_db, -0.0);
    }

    #[test]
    fn wide_outer_threshold_cannot_manufacture_stability() {
        let mut config = PstbConfig::default();
        config.stability_threshold = 2.0;
        let mut analyzer = PstbAnalyzer::new(config);
        let result = analyzer
            .analyze_monodromy_with_abort(&[vec![1.5]], 1.0, &NoAbort)
            .unwrap();

        assert_eq!(result.stability, StabilityType::Marginal);
        assert!(!result.is_stable());
    }

    #[test]
    fn pstb_returns_full_sorted_spectrum_with_paired_eigenvectors() {
        let matrix = vec![
            vec![0.0, -1.0, 0.0],
            vec![1.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.5],
        ];
        let mut config = PstbConfig::default().with_eigenvectors(true);
        config.num_eigenvalues = 1;
        let mut analyzer = PstbAnalyzer::new(config);
        let result = analyzer
            .analyze_monodromy_with_abort(&matrix, 1.0, &NoAbort)
            .unwrap();

        assert_eq!(result.multipliers.len(), 3);
        assert!(result.converged);
        assert!(
            result
                .multipliers
                .windows(2)
                .all(|pair| pair[0].magnitude() >= pair[1].magnitude())
        );

        for multiplier in &result.multipliers {
            let vector = multiplier.eigenvector.as_ref().unwrap();
            let mut residual_norm = 0.0_f64;
            for row in 0..matrix.len() {
                let product: Complex64 = matrix[row]
                    .iter()
                    .zip(vector)
                    .map(|(&coefficient, &component)| coefficient * component)
                    .sum();
                residual_norm =
                    residual_norm.hypot((product - multiplier.value * vector[row]).norm());
            }
            assert!(residual_norm < 1e-12, "residual={residual_norm:.3e}");
        }
    }

    #[test]
    fn subharmonic_reporting_honors_configuration() {
        let mut config = PstbConfig::default();
        config.detect_subharmonics = false;
        let mut analyzer = PstbAnalyzer::new(config);
        let result = analyzer
            .analyze_monodromy_with_abort(&[vec![-1.0]], 1.0, &NoAbort)
            .unwrap();

        assert_eq!(result.stability, StabilityType::PeriodDoubling);
        assert!(result.subharmonics.is_empty());
    }
}
