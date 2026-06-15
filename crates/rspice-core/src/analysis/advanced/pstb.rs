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
        let eigenvectors = if self.config.compute_eigenvectors {
            self.compute_right_eigenvectors(monodromy, &eigenvalues)
        } else {
            vec![None; eigenvalues.len()]
        };

        // Create FloquetMultiplier objects
        let mut multipliers: Vec<FloquetMultiplier> = eigenvalues
            .iter()
            .enumerate()
            .map(|(i, &ev)| {
                let mut fm = FloquetMultiplier::new(ev, period, self.config.stability_threshold);
                fm.index = i;
                fm.eigenvector = eigenvectors.get(i).cloned().unwrap_or(None);
                fm
            })
            .collect();

        // Sort by magnitude (most unstable first)
        multipliers.sort_by(|a, b| b.magnitude().total_cmp(&a.magnitude()));

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
                .filter(|margin| margin.is_finite())
                .min_by(|a, b| a.total_cmp(b))
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

    /// Compute right eigenvectors for each eigenvalue using shifted inverse iteration.
    fn compute_right_eigenvectors(
        &self,
        matrix: &[Vec<Value>],
        eigenvalues: &[Complex64],
    ) -> Vec<Option<Vec<Complex64>>> {
        eigenvalues
            .iter()
            .map(|&lambda| self.inverse_iteration_right_eigenvector(matrix, lambda))
            .collect()
    }

    /// Shifted inverse iteration for right eigenvector associated with `lambda`.
    fn inverse_iteration_right_eigenvector(
        &self,
        matrix: &[Vec<Value>],
        lambda: Complex64,
    ) -> Option<Vec<Complex64>> {
        let n = matrix.len();
        if n == 0 {
            return None;
        }
        if n == 1 {
            return Some(vec![Complex64::new(1.0, 0.0)]);
        }

        let mut v = (0..n)
            .map(|i| Complex64::new((i + 1) as Value, 0.0))
            .collect::<Vec<_>>();
        let mut v_norm = Self::complex_l2_norm(&v);
        if !v_norm.is_finite() || v_norm <= 1e-20 {
            return None;
        }
        for x in &mut v {
            *x /= v_norm;
        }

        // Small complex shift keeps (A - λI) solvable for inverse iteration.
        let shift = self.config.eigenvalue_tolerance.max(1e-12);
        let lambda_shifted = lambda + Complex64::new(shift, shift * 0.1);

        const MAX_ITERS: usize = 24;
        for _ in 0..MAX_ITERS {
            let mut w = self.solve_shifted_complex_system(matrix, lambda_shifted, &v)?;
            let w_norm = Self::complex_l2_norm(&w);
            if !w_norm.is_finite() || w_norm <= 1e-20 {
                return None;
            }
            for x in &mut w {
                *x /= w_norm;
            }
            Self::phase_normalize(&mut w);

            let delta = w
                .iter()
                .zip(v.iter())
                .map(|(a, b)| (*a - *b).norm_sqr())
                .sum::<Value>()
                .sqrt();
            v = w;
            v_norm = Self::complex_l2_norm(&v);
            if !v_norm.is_finite() || v_norm <= 1e-20 {
                return None;
            }
            if delta <= 1e-8 {
                break;
            }
        }

        if v.iter().all(|c| c.re.is_finite() && c.im.is_finite()) {
            Some(v)
        } else {
            None
        }
    }

    /// Solve (A - λI)x = rhs for x, where A is real and λ, rhs are complex.
    fn solve_shifted_complex_system(
        &self,
        matrix: &[Vec<Value>],
        lambda: Complex64,
        rhs: &[Complex64],
    ) -> Option<Vec<Complex64>> {
        let n = matrix.len();
        if rhs.len() != n {
            return None;
        }

        let mut aug = vec![vec![Complex64::new(0.0, 0.0); n + 1]; n];
        for i in 0..n {
            for j in 0..n {
                aug[i][j] = Complex64::new(matrix[i][j], 0.0);
            }
            aug[i][i] -= lambda;
            aug[i][n] = rhs[i];
        }

        const PIVOT_EPS: Value = 1e-20;

        for col in 0..n {
            let mut pivot_row = col;
            let mut pivot_norm = aug[col][col].norm();
            for (row, row_data) in aug.iter().enumerate().skip(col + 1) {
                let candidate = row_data[col].norm();
                if candidate > pivot_norm {
                    pivot_norm = candidate;
                    pivot_row = row;
                }
            }

            if !pivot_norm.is_finite() || pivot_norm <= PIVOT_EPS {
                return None;
            }
            if pivot_row != col {
                aug.swap(pivot_row, col);
            }

            let pivot = aug[col][col];
            let pivot_entries = aug[col].clone();
            for row in (col + 1)..n {
                let factor = aug[row][col] / pivot;
                if factor.norm() <= PIVOT_EPS {
                    continue;
                }
                for k in col..=n {
                    aug[row][k] -= factor * pivot_entries[k];
                }
            }
        }

        let mut x = vec![Complex64::new(0.0, 0.0); n];
        for i_rev in 0..n {
            let i = n - 1 - i_rev;
            let mut sum = aug[i][n];
            for (k, xk) in x.iter().enumerate().skip(i + 1) {
                sum -= aug[i][k] * *xk;
            }
            let diag = aug[i][i];
            if diag.norm() <= PIVOT_EPS {
                return None;
            }
            x[i] = sum / diag;
        }

        if x.iter().all(|c| c.re.is_finite() && c.im.is_finite()) {
            Some(x)
        } else {
            None
        }
    }

    fn complex_l2_norm(values: &[Complex64]) -> Value {
        values.iter().map(|c| c.norm_sqr()).sum::<Value>().sqrt()
    }

    /// Normalize phase so the largest-magnitude component is real-positive.
    fn phase_normalize(vector: &mut [Complex64]) {
        let Some((idx, _)) = vector.iter().enumerate().max_by(|(_, a), (_, b)| {
            a.norm()
                .partial_cmp(&b.norm())
                .unwrap_or(std::cmp::Ordering::Equal)
        }) else {
            return;
        };
        let anchor = vector[idx];
        if anchor.norm() <= 1e-20 {
            return;
        }
        let rot = Complex64::from_polar(1.0, -anchor.arg());
        for x in vector {
            *x *= rot;
        }
    }

    /// QR decomposition using modified Gram-Schmidt
    fn qr_decompose(&self, a: &[Vec<Value>]) -> (Vec<Vec<Value>>, Vec<Vec<Value>>) {
        let n = a.len();
        let mut q: Vec<Vec<Value>> = vec![vec![0.0; n]; n];
        let mut r: Vec<Vec<Value>> = vec![vec![0.0; n]; n];

        // Extract columns
        let cols: Vec<Vec<Value>> = (0..n).map(|j| (0..n).map(|i| a[i][j]).collect()).collect();

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
