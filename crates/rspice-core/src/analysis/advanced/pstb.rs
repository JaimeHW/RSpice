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

use crate::Value;
use num_complex::Complex64;
use std::f64::consts::PI;

//=============================================================================
// PSTB Configuration
//=============================================================================

/// Configuration for Periodic Stability (PSTB) analysis
#[derive(Debug, Clone)]
pub struct PstbConfig {
    /// Number of eigenvalues to compute (0 = all)
    pub num_eigenvalues: usize,

    /// Tolerance for eigenvalue computation
    pub eigenvalue_tolerance: Value,

    /// Maximum iterations for eigenvalue solver
    pub max_iterations: usize,

    /// Whether to compute Floquet exponents (log of multipliers)
    pub compute_exponents: bool,

    /// Whether to compute eigenvectors (mode shapes)
    pub compute_eigenvectors: bool,

    /// Stability margin threshold (|λ| below this is considered stable)
    pub stability_threshold: Value,

    /// Enable subharmonic detection (multipliers at roots of unity)
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

    /// Floquet exponent: μ = ln(λ)/T
    pub exponent: Complex64,

    /// Index in sorted order (0 = most unstable)
    pub index: usize,

    /// Whether this multiplier indicates instability
    pub is_unstable: bool,

    /// Whether this is the trivial multiplier (λ ≈ 1 for autonomous systems)
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
            is_unstable: magnitude > stability_threshold && !is_trivial,
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

    /// Stability margin in dB (20·log10(1/|λ|))
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
    /// All non-trivial multipliers inside unit circle
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

    /// Check if this indicates a bifurcation
    pub fn is_bifurcation(&self) -> bool {
        matches!(
            self,
            StabilityType::PeriodDoubling
                | StabilityType::NeimarkSacker
                | StabilityType::SaddleNode
        )
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

    /// Number of iterations used
    pub iterations: usize,
}

impl PstbResult {
    /// Get the most unstable (or least stable) multiplier
    pub fn dominant_multiplier(&self) -> Option<&FloquetMultiplier> {
        self.multipliers.first()
    }

    /// Get all unstable multipliers
    pub fn unstable_multipliers(&self) -> Vec<&FloquetMultiplier> {
        self.multipliers.iter().filter(|m| m.is_unstable).collect()
    }

    /// Get the trivial multiplier (should be ≈ 1)
    pub fn trivial_multiplier(&self) -> Option<&FloquetMultiplier> {
        self.multipliers.iter().find(|m| m.is_trivial)
    }

    /// Check if the periodic orbit is stable
    pub fn is_stable(&self) -> bool {
        self.stability.is_stable()
    }

    /// Get stability margin in dB (positive = stable)
    pub fn stability_margin(&self) -> Value {
        self.min_stability_margin_db
    }

    /// Get natural frequencies of all oscillatory modes
    pub fn mode_frequencies(&self) -> Vec<Value> {
        self.multipliers
            .iter()
            .filter(|m| !m.is_trivial && m.exponent.im.abs() > 1e-10)
            .map(|m| m.natural_frequency())
            .collect()
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

    /// Analyze stability from a pre-computed Monodromy matrix
    pub fn analyze_monodromy(&mut self, monodromy: &[Vec<Value>], period: Value) -> PstbResult {
        let n = monodromy.len();
        self.dimension = n;

        // Compute eigenvalues using QR iteration or power method
        let eigenvalues = self.compute_eigenvalues(monodromy);

        // Create FloquetMultiplier objects
        let mut multipliers: Vec<FloquetMultiplier> = eigenvalues
            .iter()
            .enumerate()
            .map(|(i, &ev)| {
                let mut fm = FloquetMultiplier::new(ev, period, self.config.stability_threshold);
                fm.index = i;
                fm
            })
            .collect();

        // Sort by magnitude (most unstable first)
        multipliers.sort_by(|a, b| {
            b.magnitude()
                .partial_cmp(&a.magnitude())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Re-index after sorting
        for (i, m) in multipliers.iter_mut().enumerate() {
            m.index = i;
        }

        // Classify stability
        let stability = self.classify_stability(&multipliers);

        // Count unstable and compute margins
        let num_unstable = multipliers.iter().filter(|m| m.is_unstable).count();
        let max_magnitude = multipliers.first().map(|m| m.magnitude()).unwrap_or(0.0);

        let min_margin_db = if num_unstable == 0 && !multipliers.is_empty() {
            // Find the largest non-trivial magnitude
            multipliers
                .iter()
                .filter(|m| !m.is_trivial)
                .map(|m| m.stability_margin_db())
                .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .unwrap_or(f64::INFINITY)
        } else {
            -max_magnitude.log10() * 20.0 // Negative margin = unstable
        };

        // Detect subharmonics
        let subharmonics: Vec<usize> = multipliers
            .iter()
            .filter_map(|m| m.subharmonic_order)
            .collect();

        PstbResult {
            period,
            fundamental_frequency: 1.0 / period,
            multipliers,
            stability,
            monodromy: monodromy.to_vec(),
            num_unstable,
            min_stability_margin_db: min_margin_db,
            max_multiplier_magnitude: max_magnitude,
            subharmonics,
            converged: true,
            iterations: 0,
        }
    }

    /// Compute eigenvalues of the Monodromy matrix
    fn compute_eigenvalues(&self, matrix: &[Vec<Value>]) -> Vec<Complex64> {
        let n = matrix.len();
        if n == 0 {
            return Vec::new();
        }

        // For 2x2, use analytical formula
        if n == 2 {
            return self.eigenvalues_2x2(matrix);
        }

        // For larger matrices, use QR iteration
        self.qr_eigenvalues(matrix)
    }

    /// Analytical eigenvalues for 2x2 matrix
    fn eigenvalues_2x2(&self, m: &[Vec<Value>]) -> Vec<Complex64> {
        let trace = m[0][0] + m[1][1];
        let det = m[0][0] * m[1][1] - m[0][1] * m[1][0];
        let discriminant = trace * trace - 4.0 * det;

        if discriminant >= 0.0 {
            let sqrt_d = discriminant.sqrt();
            vec![
                Complex64::new((trace + sqrt_d) / 2.0, 0.0),
                Complex64::new((trace - sqrt_d) / 2.0, 0.0),
            ]
        } else {
            let sqrt_d = (-discriminant).sqrt();
            vec![
                Complex64::new(trace / 2.0, sqrt_d / 2.0),
                Complex64::new(trace / 2.0, -sqrt_d / 2.0),
            ]
        }
    }

    /// QR iteration for eigenvalues of larger matrices
    fn qr_eigenvalues(&self, matrix: &[Vec<Value>]) -> Vec<Complex64> {
        let n = matrix.len();
        let mut eigenvalues = Vec::with_capacity(n);

        // Convert to working matrix (Hessenberg form would be more efficient)
        let mut a: Vec<Vec<Value>> = matrix.to_vec();

        // Simple QR iteration (for production, use LAPACK)
        for _iter in 0..self.config.max_iterations {
            // QR decomposition using Gram-Schmidt
            let (q, r) = self.qr_decompose(&a);

            // A_new = R * Q
            a = self.matrix_multiply(&r, &q);

            // Check for convergence (subdiagonal elements small)
            let mut converged = true;
            for i in 1..n {
                if a[i][i - 1].abs() > self.config.eigenvalue_tolerance {
                    converged = false;
                    break;
                }
            }

            if converged {
                break;
            }
        }

        // Extract eigenvalues from diagonal (and 2x2 blocks for complex pairs)
        let mut i = 0;
        while i < n {
            if i == n - 1 || a[i + 1][i].abs() < self.config.eigenvalue_tolerance {
                // Real eigenvalue on diagonal
                eigenvalues.push(Complex64::new(a[i][i], 0.0));
                i += 1;
            } else {
                // 2x2 block - complex conjugate pair
                let sub = vec![
                    vec![a[i][i], a[i][i + 1]],
                    vec![a[i + 1][i], a[i + 1][i + 1]],
                ];
                let pair = self.eigenvalues_2x2(&sub);
                eigenvalues.extend(pair);
                i += 2;
            }
        }

        eigenvalues
    }

    /// QR decomposition using modified Gram-Schmidt
    fn qr_decompose(&self, a: &[Vec<Value>]) -> (Vec<Vec<Value>>, Vec<Vec<Value>>) {
        let n = a.len();
        let mut q: Vec<Vec<Value>> = vec![vec![0.0; n]; n];
        let mut r: Vec<Vec<Value>> = vec![vec![0.0; n]; n];

        // Extract columns
        let mut cols: Vec<Vec<Value>> = (0..n).map(|j| (0..n).map(|i| a[i][j]).collect()).collect();

        for j in 0..n {
            let mut v = cols[j].clone();

            // Orthogonalize against previous columns
            for i in 0..j {
                let q_col: Vec<Value> = (0..n).map(|k| q[k][i]).collect();
                let dot: Value = v.iter().zip(q_col.iter()).map(|(a, b)| a * b).sum();
                r[i][j] = dot;
                for k in 0..n {
                    v[k] -= dot * q_col[k];
                }
            }

            // Normalize
            let norm: Value = v.iter().map(|x| x * x).sum::<Value>().sqrt();
            r[j][j] = norm;

            if norm > 1e-15 {
                for k in 0..n {
                    q[k][j] = v[k] / norm;
                }
            }
        }

        (q, r)
    }

    /// Matrix multiplication
    fn matrix_multiply(&self, a: &[Vec<Value>], b: &[Vec<Value>]) -> Vec<Vec<Value>> {
        let n = a.len();
        let mut c = vec![vec![0.0; n]; n];

        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    c[i][j] += a[i][k] * b[k][j];
                }
            }
        }

        c
    }

    /// Classify stability based on multipliers
    fn classify_stability(&self, multipliers: &[FloquetMultiplier]) -> StabilityType {
        let unstable: Vec<&FloquetMultiplier> =
            multipliers.iter().filter(|m| m.is_unstable).collect();

        if unstable.is_empty() {
            // Check for bifurcations at unit circle
            for m in multipliers.iter().filter(|m| !m.is_trivial) {
                let mag = m.magnitude();
                let real = m.value.re;
                let imag = m.value.im.abs();

                // Near λ = -1: period doubling
                if (m.value + Complex64::new(1.0, 0.0)).norm() < 0.01 {
                    return StabilityType::PeriodDoubling;
                }

                // Near λ = +1 (non-trivial): saddle-node
                if (m.value - Complex64::new(1.0, 0.0)).norm() < 0.01 && !m.is_trivial {
                    return StabilityType::SaddleNode;
                }

                // Complex pair near unit circle: Neimark-Sacker
                if (mag - 1.0).abs() < 0.01 && imag > 0.01 {
                    return StabilityType::NeimarkSacker;
                }

                // Check for marginal stability
                if (mag - self.config.stability_threshold).abs() < 0.01 {
                    return StabilityType::Marginal;
                }
            }

            StabilityType::Stable
        } else {
            // There are unstable multipliers
            let dominant = &unstable[0];

            if dominant.value.im.abs() > 0.01 {
                StabilityType::UnstableComplex
            } else {
                StabilityType::UnstableReal
            }
        }
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Configuration Tests
    // =========================================================================

    #[test]
    fn test_pstb_config_default() {
        let config = PstbConfig::default();
        assert_eq!(config.num_eigenvalues, 0);
        assert!(config.compute_exponents);
        assert!(!config.compute_eigenvectors);
        assert!(config.detect_subharmonics);
    }

    #[test]
    fn test_pstb_config_builder() {
        let config = PstbConfig::new()
            .with_num_eigenvalues(4)
            .with_eigenvectors(true)
            .with_stability_threshold(1.001)
            .with_tolerance(1e-12);

        assert_eq!(config.num_eigenvalues, 4);
        assert!(config.compute_eigenvectors);
        assert!((config.stability_threshold - 1.001).abs() < 1e-10);
        assert!((config.eigenvalue_tolerance - 1e-12).abs() < 1e-20);
    }

    // =========================================================================
    // Floquet Multiplier Tests
    // =========================================================================

    #[test]
    fn test_floquet_multiplier_stable() {
        let value = Complex64::new(0.5, 0.0);
        let period = 1e-9;
        let fm = FloquetMultiplier::new(value, period, 1.0 + 1e-6);

        assert!(!fm.is_unstable);
        assert!(!fm.is_trivial);
        assert_eq!(fm.magnitude(), 0.5);
        assert!(fm.stability_margin_db() > 0.0); // Positive margin = stable
    }

    #[test]
    fn test_floquet_multiplier_unstable() {
        let value = Complex64::new(1.5, 0.0);
        let period = 1e-9;
        let fm = FloquetMultiplier::new(value, period, 1.0 + 1e-6);

        assert!(fm.is_unstable);
        assert!(!fm.is_trivial);
        assert!(fm.stability_margin_db() < 0.0); // Negative margin = unstable
    }

    #[test]
    fn test_floquet_multiplier_trivial() {
        let value = Complex64::new(1.0, 0.0);
        let period = 1e-9;
        let fm = FloquetMultiplier::new(value, period, 1.0 + 1e-6);

        assert!(fm.is_trivial);
        assert!(!fm.is_unstable); // Trivial multiplier is not counted as unstable
    }

    #[test]
    fn test_floquet_multiplier_complex() {
        // Complex pair on unit circle (marginally stable oscillation)
        let angle = PI / 4.0; // 45 degrees
        let value = Complex64::from_polar(1.0, angle);
        let period = 1.0;
        let fm = FloquetMultiplier::new(value, period, 1.0 + 1e-6);

        assert!((fm.magnitude() - 1.0).abs() < 1e-10);
        assert!((fm.phase() - angle).abs() < 1e-10);
    }

    #[test]
    fn test_floquet_multiplier_damping() {
        // Decaying oscillation: λ = 0.8 * exp(i*π/4)
        let mag = 0.8;
        let angle = PI / 4.0;
        let value = Complex64::from_polar(mag, angle);
        let period = 1e-3; // 1 ms period
        let fm = FloquetMultiplier::new(value, period, 1.0 + 1e-6);

        // Damping should be positive (decaying)
        assert!(fm.damping() > 0.0);

        // Natural frequency
        assert!(fm.natural_frequency() > 0.0);
    }

    #[test]
    fn test_floquet_subharmonic_detection() {
        // Period-2 subharmonic: λ = -1 (second root of unity)
        let value = Complex64::new(-1.0, 0.0);
        let fm = FloquetMultiplier::new(value, 1.0, 1.01);
        assert_eq!(fm.subharmonic_order, Some(2));

        // Period-3 subharmonic
        let angle = 2.0 * PI / 3.0;
        let value3 = Complex64::from_polar(1.0, angle);
        let fm3 = FloquetMultiplier::new(value3, 1.0, 1.01);
        assert_eq!(fm3.subharmonic_order, Some(3));

        // Not a subharmonic
        let value_other = Complex64::new(0.7, 0.3);
        let fm_other = FloquetMultiplier::new(value_other, 1.0, 1.01);
        assert_eq!(fm_other.subharmonic_order, None);
    }

    // =========================================================================
    // Stability Classification Tests
    // =========================================================================

    #[test]
    fn test_stability_type_is_stable() {
        assert!(StabilityType::Stable.is_stable());
        assert!(!StabilityType::UnstableReal.is_stable());
        assert!(!StabilityType::UnstableComplex.is_stable());
        assert!(!StabilityType::PeriodDoubling.is_stable());
    }

    #[test]
    fn test_stability_type_is_bifurcation() {
        assert!(!StabilityType::Stable.is_bifurcation());
        assert!(!StabilityType::UnstableReal.is_bifurcation());
        assert!(StabilityType::PeriodDoubling.is_bifurcation());
        assert!(StabilityType::NeimarkSacker.is_bifurcation());
        assert!(StabilityType::SaddleNode.is_bifurcation());
    }

    // =========================================================================
    // Analyzer Tests - 2x2 Systems
    // =========================================================================

    #[test]
    fn test_analyzer_stable_node() {
        let config = PstbConfig::default();
        let mut analyzer = PstbAnalyzer::new(config);

        // Stable node: both eigenvalues inside unit circle
        // λ₁ = 0.8, λ₂ = 0.5
        let monodromy = vec![vec![0.8, 0.0], vec![0.0, 0.5]];

        let result = analyzer.analyze_monodromy(&monodromy, 1e-9);

        assert!(result.is_stable());
        assert_eq!(result.stability, StabilityType::Stable);
        assert_eq!(result.num_unstable, 0);
        assert!(result.min_stability_margin_db > 0.0);
    }

    #[test]
    fn test_analyzer_unstable_node() {
        let config = PstbConfig::default();
        let mut analyzer = PstbAnalyzer::new(config);

        // Unstable: one eigenvalue outside unit circle
        // λ₁ = 1.2, λ₂ = 0.5
        let monodromy = vec![vec![1.2, 0.0], vec![0.0, 0.5]];

        let result = analyzer.analyze_monodromy(&monodromy, 1e-9);

        assert!(!result.is_stable());
        assert_eq!(result.stability, StabilityType::UnstableReal);
        assert!(result.num_unstable >= 1);
        assert!(result.max_multiplier_magnitude > 1.0);
    }

    #[test]
    fn test_analyzer_stable_focus() {
        let config = PstbConfig::default();
        let mut analyzer = PstbAnalyzer::new(config);

        // Stable focus: complex eigenvalues with |λ| < 1
        // Rotation by 30 degrees, decay by 0.9
        let mag = 0.9;
        let angle = PI / 6.0;
        let cos_a = angle.cos() * mag;
        let sin_a = angle.sin() * mag;
        let monodromy = vec![vec![cos_a, -sin_a], vec![sin_a, cos_a]];

        let result = analyzer.analyze_monodromy(&monodromy, 1e-9);

        assert!(result.is_stable());
        assert_eq!(result.multipliers.len(), 2);

        // Both should have magnitude ≈ 0.9
        for m in &result.multipliers {
            assert!((m.magnitude() - 0.9).abs() < 0.01);
        }
    }

    #[test]
    fn test_analyzer_period_doubling() {
        let config = PstbConfig::default();
        let mut analyzer = PstbAnalyzer::new(config);

        // Period doubling: λ = -1 (at bifurcation)
        let monodromy = vec![vec![-1.0, 0.0], vec![0.0, 0.5]];

        let result = analyzer.analyze_monodromy(&monodromy, 1e-9);

        assert_eq!(result.stability, StabilityType::PeriodDoubling);
        assert!(result.stability.is_bifurcation());
    }

    #[test]
    fn test_analyzer_with_trivial_multiplier() {
        let config = PstbConfig::default();
        let mut analyzer = PstbAnalyzer::new(config);

        // Autonomous system: one λ = 1 (trivial), one λ = 0.7 (stable)
        let monodromy = vec![vec![1.0, 0.0], vec![0.0, 0.7]];

        let result = analyzer.analyze_monodromy(&monodromy, 1e-9);

        assert!(result.is_stable());
        assert!(result.trivial_multiplier().is_some());
        assert!(result.trivial_multiplier().unwrap().is_trivial);
    }

    // =========================================================================
    // Analyzer Tests - Larger Systems
    // =========================================================================

    #[test]
    fn test_analyzer_3x3_stable() {
        let config = PstbConfig::default();
        let mut analyzer = PstbAnalyzer::new(config);

        // Diagonal matrix with all eigenvalues inside unit circle
        let monodromy = vec![
            vec![0.8, 0.0, 0.0],
            vec![0.0, 0.6, 0.0],
            vec![0.0, 0.0, 0.4],
        ];

        let result = analyzer.analyze_monodromy(&monodromy, 1e-9);

        assert!(result.is_stable());
        assert_eq!(result.multipliers.len(), 3);
    }

    #[test]
    fn test_analyzer_3x3_unstable() {
        let config = PstbConfig::default();
        let mut analyzer = PstbAnalyzer::new(config);

        // One eigenvalue outside
        let monodromy = vec![
            vec![1.5, 0.0, 0.0],
            vec![0.0, 0.6, 0.0],
            vec![0.0, 0.0, 0.4],
        ];

        let result = analyzer.analyze_monodromy(&monodromy, 1e-9);

        assert!(!result.is_stable());
        assert!(result.num_unstable >= 1);
    }

    // =========================================================================
    // Result Access Tests
    // =========================================================================

    #[test]
    fn test_result_dominant_multiplier() {
        let config = PstbConfig::default();
        let mut analyzer = PstbAnalyzer::new(config);

        let monodromy = vec![vec![0.9, 0.0], vec![0.0, 0.5]];
        let result = analyzer.analyze_monodromy(&monodromy, 1e-9);

        let dominant = result.dominant_multiplier().unwrap();
        assert!((dominant.magnitude() - 0.9).abs() < 0.01);
    }

    #[test]
    fn test_result_mode_frequencies() {
        let config = PstbConfig::default();
        let mut analyzer = PstbAnalyzer::new(config);

        // Complex pair creates oscillatory mode
        let angle = PI / 4.0;
        let mag = 0.9;
        let monodromy = vec![
            vec![mag * angle.cos(), -mag * angle.sin()],
            vec![mag * angle.sin(), mag * angle.cos()],
        ];

        let period = 1e-6; // 1 μs period = 1 MHz fundamental
        let result = analyzer.analyze_monodromy(&monodromy, period);

        let freqs = result.mode_frequencies();
        assert!(!freqs.is_empty());
    }

    #[test]
    fn test_result_stability_margin() {
        let config = PstbConfig::default();
        let mut analyzer = PstbAnalyzer::new(config);

        // Stable with margin
        let monodromy = vec![vec![0.5, 0.0], vec![0.0, 0.8]];
        let result = analyzer.analyze_monodromy(&monodromy, 1e-9);

        // Margin should be positive for stable system
        assert!(result.stability_margin() > 0.0);

        // Unstable
        let monodromy2 = vec![vec![1.5, 0.0], vec![0.0, 0.8]];
        let result2 = analyzer.analyze_monodromy(&monodromy2, 1e-9);

        // Margin should be negative for unstable system
        assert!(result2.stability_margin() < 0.0);
    }

    #[test]
    fn test_result_subharmonic_detection() {
        let config = PstbConfig::default();
        let mut analyzer = PstbAnalyzer::new(config);

        // Period-3 subharmonic
        let angle = 2.0 * PI / 3.0;
        let monodromy = vec![
            vec![angle.cos(), -angle.sin()],
            vec![angle.sin(), angle.cos()],
        ];

        let result = analyzer.analyze_monodromy(&monodromy, 1e-9);

        assert!(!result.subharmonics.is_empty());
        assert!(result.subharmonics.contains(&3));
    }

    // =========================================================================
    // Edge Cases
    // =========================================================================

    #[test]
    fn test_empty_monodromy() {
        let config = PstbConfig::default();
        let mut analyzer = PstbAnalyzer::new(config);

        let monodromy: Vec<Vec<Value>> = vec![];
        let result = analyzer.analyze_monodromy(&monodromy, 1e-9);

        assert!(result.multipliers.is_empty());
        assert!(result.is_stable()); // Empty is vacuously stable
    }

    #[test]
    fn test_1x1_monodromy() {
        let config = PstbConfig::default();
        let mut analyzer = PstbAnalyzer::new(config);

        // Single state variable
        let monodromy = vec![vec![0.7]];
        let result = analyzer.analyze_monodromy(&monodromy, 1e-9);

        assert_eq!(result.multipliers.len(), 1);
        assert!((result.multipliers[0].magnitude() - 0.7).abs() < 0.01);
        assert!(result.is_stable());
    }

    #[test]
    fn test_marginally_stable() {
        let config = PstbConfig::new().with_stability_threshold(1.0001);
        let mut analyzer = PstbAnalyzer::new(config);

        // Exactly on unit circle
        let monodromy = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let result = analyzer.analyze_monodromy(&monodromy, 1e-9);

        // Both are trivial
        for m in &result.multipliers {
            assert!(m.is_trivial);
        }
    }

    #[test]
    fn test_oscillator_stability() {
        let config = PstbConfig::default();
        let mut analyzer = PstbAnalyzer::new(config);

        // Typical oscillator: one λ = 1 (phase), one λ = 0.9 (amplitude)
        let monodromy = vec![vec![1.0, 0.0], vec![0.0, 0.9]];
        let result = analyzer.analyze_monodromy(&monodromy, 1e-9);

        assert!(result.is_stable());

        // The trivial multiplier
        let trivial = result.trivial_multiplier().unwrap();
        assert!(trivial.is_trivial);

        // The amplitude mode
        let amp_modes: Vec<_> = result
            .multipliers
            .iter()
            .filter(|m| !m.is_trivial)
            .collect();
        assert_eq!(amp_modes.len(), 1);
        assert!((amp_modes[0].magnitude() - 0.9).abs() < 0.01);
    }

    // =========================================================================
    // QR Decomposition Tests
    // =========================================================================

    #[test]
    fn test_qr_decomposition_identity() {
        let config = PstbConfig::default();
        let analyzer = PstbAnalyzer::new(config);

        let identity = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let (q, r) = analyzer.qr_decompose(&identity);

        // Q should be identity
        assert!((q[0][0] - 1.0).abs() < 1e-10);
        assert!((q[1][1] - 1.0).abs() < 1e-10);
        assert!(q[0][1].abs() < 1e-10);
        assert!(q[1][0].abs() < 1e-10);

        // R should be identity
        assert!((r[0][0] - 1.0).abs() < 1e-10);
        assert!((r[1][1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_matrix_multiply() {
        let config = PstbConfig::default();
        let analyzer = PstbAnalyzer::new(config);

        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];

        let c = analyzer.matrix_multiply(&a, &b);

        // [[1,2],[3,4]] * [[5,6],[7,8]] = [[19,22],[43,50]]
        assert!((c[0][0] - 19.0).abs() < 1e-10);
        assert!((c[0][1] - 22.0).abs() < 1e-10);
        assert!((c[1][0] - 43.0).abs() < 1e-10);
        assert!((c[1][1] - 50.0).abs() < 1e-10);
    }
}
