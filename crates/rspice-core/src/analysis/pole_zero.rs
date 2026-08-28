//! Pole-Zero Analysis (.PZ)
//!
//! Finds the poles and zeros of a circuit's transfer function.
//!
//! # Theory
//!
//! For a linear circuit, the transfer function H(s) can be expressed as:
//!
//! ```text
//! H(s) = K · ∏(s - zᵢ) / ∏(s - pⱼ)
//! ```
//!
//! - **Poles (pⱼ)**: Values of s where H(s) → ∞ (natural frequencies)
//! - **Zeros (zᵢ)**: Values of s where H(s) = 0
//!
//! # Algorithm
//!
//! 1. Build MNA matrix as Y(s) = G + s·C where G is conductance, C is capacitance
//! 2. **Poles**: Solve generalized eigenvalue problem G·x = -s·C·x
//! 3. **Zeros**: Augment matrix with input/output and solve eigenvalue problem
//!
//! # Example
//!
//! ```ignore
//! .PZ V(out) Vin CUR PZ    ; Find poles and zeros, current input
//! ```

#![allow(clippy::needless_range_loop)]
use crate::{Complex64, Value};
use faer::{Mat, linalg::solvers::GeneralizedEigen};
use std::f64::consts::PI;
use thiserror::Error;

/// A root counts as real when its imaginary part sits inside this band. The
/// eigen solvers hand back conjugate pairs carrying a residual imaginary part,
/// so an exact zero test would report every real root as complex.
const REAL_ROOT_TOLERANCE: Value = 1e-10;

/// A pole-zero extraction failure that must not be represented as an empty
/// or estimated root set.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PoleZeroAnalysisError {
    /// The descriptor, state-space model, or port definition is malformed.
    #[error("invalid pole-zero system: {0}")]
    InvalidSystem(String),
    /// The generalized descriptor pencil contains an indeterminate 0/0
    /// eigenpair and therefore has no qualified finite/infinite spectrum.
    #[error(
        "the descriptor pencil G+sC is irregular (indeterminate generalized eigenvalue {index}, |alpha|={alpha_norm:.3e}, |beta|={beta_norm:.3e})"
    )]
    IrregularDescriptor {
        index: usize,
        alpha_norm: Value,
        beta_norm: Value,
    },
    /// The selected eigensolver could not produce a spectrum.
    #[error("{problem} eigenvalue extraction failed")]
    EigenvalueFailure { problem: &'static str },
    /// An eigensolver returned a non-finite coefficient or root.
    #[error("{problem} eigenvalue {index} is non-finite")]
    NonFiniteEigenvalue { problem: &'static str, index: usize },
    /// A complete square eigenproblem must return one eigenpair per row.
    #[error("{problem} spectrum is incomplete: expected {expected} eigenpairs, received {actual}")]
    IncompleteSpectrum {
        problem: &'static str,
        expected: usize,
        actual: usize,
    },
    /// A complete eigenspectrum was returned, but an eigenpair has too much
    /// normwise backward error to be retained even as an approximate result.
    #[error(
        "{problem} eigenvalue {index} failed numerical qualification (backward error {backward_error:.3e}, maximum {maximum:.3e})"
    )]
    NumericalQualification {
        problem: &'static str,
        index: usize,
        backward_error: Value,
        maximum: Value,
    },
    /// A configured reporting limit would otherwise silently discard roots.
    #[error(
        "{quantity} extraction found {omitted} finite root(s) at or above the configured limit {limit:.6e} rad/s"
    )]
    FrequencyLimitExceeded {
        quantity: &'static str,
        omitted: usize,
        limit: Value,
    },
    /// A transfer-function numerator could not be constructed or reduced.
    #[error("pole-zero transfer extraction failed: {0}")]
    TransferExtraction(&'static str),
}

/// Numerical evidence attached to one complete finite/infinite eigenspectrum.
///
/// `problem_order - infinite_count` is the number of finite roots represented
/// by the associated root vector. `max_backward_error` is the worst normalized
/// residual among every finite eigenpair and every finite right-eigenvector
/// representative returned for an infinite eigenvalue. Generalized infinite
/// algebraic multiplicity is accounted separately by exact homogeneous
/// `beta == 0` classification after rejecting non-finite and indeterminate
/// `0/0` pairs; defective infinite eigenvalues need not have one finite
/// eigenvector per algebraic copy.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpectrumCertificate {
    /// Order of the ordinary eigenproblem or generalized matrix pencil.
    pub problem_order: usize,
    /// Exact count of generalized eigenpairs whose homogeneous beta is zero.
    pub infinite_count: usize,
    /// Largest normwise backward error among finite roots and available
    /// finite eigenvector representatives of infinite roots.
    pub max_backward_error: Value,
    /// Strict threshold below which the spectrum is fully qualified.
    pub qualification_tolerance: Value,
}

impl SpectrumCertificate {
    /// Construct a certificate after validating its finite/infinite accounting.
    pub fn new(
        problem_order: usize,
        infinite_count: usize,
        max_backward_error: Value,
        qualification_tolerance: Value,
    ) -> Option<Self> {
        let certificate = Self {
            problem_order,
            infinite_count,
            max_backward_error,
            qualification_tolerance,
        };
        certificate.is_valid().then_some(certificate)
    }

    /// Construct exact analytic evidence for a scalar or polynomial result.
    pub fn exact(problem_order: usize, infinite_count: usize) -> Option<Self> {
        Self::new(
            problem_order,
            infinite_count,
            0.0,
            PoleZeroAnalyzer::qualification_tolerance(problem_order),
        )
    }

    /// Number of finite roots certified by the finite/infinite accounting.
    pub fn finite_count(self) -> usize {
        self.problem_order.saturating_sub(self.infinite_count)
    }

    /// Whether all certificate fields and counts are internally valid.
    pub fn is_valid(self) -> bool {
        self.infinite_count <= self.problem_order
            && self.max_backward_error.is_finite()
            && self.max_backward_error >= 0.0
            && self.max_backward_error <= PoleZeroAnalyzer::APPROXIMATE_BACKWARD_ERROR_LIMIT
            && self.qualification_tolerance.is_finite()
            && self.qualification_tolerance > 0.0
            && self.qualification_tolerance
                == PoleZeroAnalyzer::qualification_tolerance(self.problem_order)
    }

    /// Whether this spectrum meets the strict qualification threshold.
    pub fn is_strictly_qualified(self) -> bool {
        self.is_valid() && self.max_backward_error <= self.qualification_tolerance
    }
}

/// Evidence describing why a pole or zero vector may be interpreted as a
/// complete root set.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum RootSetEvidence {
    /// This root quantity was not requested by the analysis configuration.
    NotRequested,
    /// The requested calculation proved that there are no finite roots.
    QualifiedEmpty { certificate: SpectrumCertificate },
    /// Every reported root belongs to a complete, strictly qualified spectrum.
    Qualified { certificate: SpectrumCertificate },
    /// The spectrum is complete and usable, but its residual exceeds the
    /// strict qualification threshold while remaining below the hard limit.
    Approximate { certificate: SpectrumCertificate },
    /// Roots loaded from an older result that carried no numerical evidence.
    LegacyUnknown,
}

impl RootSetEvidence {
    /// Build evidence for a newly computed complete spectrum.
    pub fn from_certificate(root_count: usize, certificate: SpectrumCertificate) -> Option<Self> {
        if !certificate.is_valid() || certificate.finite_count() != root_count {
            return None;
        }
        if certificate.is_strictly_qualified() {
            if root_count == 0 {
                Some(Self::QualifiedEmpty { certificate })
            } else {
                Some(Self::Qualified { certificate })
            }
        } else {
            Some(Self::Approximate { certificate })
        }
    }

    /// Numerical certificate, when this evidence came from a new computation.
    pub fn certificate(&self) -> Option<&SpectrumCertificate> {
        match self {
            Self::QualifiedEmpty { certificate }
            | Self::Qualified { certificate }
            | Self::Approximate { certificate } => Some(certificate),
            Self::NotRequested | Self::LegacyUnknown => None,
        }
    }

    /// Whether this evidence is structurally consistent with a root vector.
    pub fn is_consistent_with(&self, roots: &[Complex64]) -> bool {
        match self {
            Self::NotRequested => roots.is_empty(),
            Self::QualifiedEmpty { certificate } => {
                roots.is_empty()
                    && certificate.is_strictly_qualified()
                    && certificate.finite_count() == 0
            }
            Self::Qualified { certificate } => {
                !roots.is_empty()
                    && certificate.is_strictly_qualified()
                    && certificate.finite_count() == roots.len()
            }
            Self::Approximate { certificate } => {
                certificate.is_valid()
                    && !certificate.is_strictly_qualified()
                    && certificate.finite_count() == roots.len()
            }
            Self::LegacyUnknown => true,
        }
    }

    /// Whether this evidence proves a complete, strictly qualified set.
    pub fn is_qualified(&self) -> bool {
        matches!(self, Self::QualifiedEmpty { .. } | Self::Qualified { .. })
    }
}

/// Three-valued stability result. Only a qualified pole set can prove stable
/// or unstable behavior; absent, approximate, and legacy roots are indeterminate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StabilityVerdict {
    Stable,
    Unstable,
    Indeterminate,
}

//=============================================================================
// Pole-Zero Result
//=============================================================================

/// Result of pole-zero analysis
#[derive(Debug, Clone)]
pub struct PoleZeroResult {
    /// System poles (natural frequencies)
    pub poles: Vec<Complex64>,
    /// System zeros
    pub zeros: Vec<Complex64>,
    /// Completeness and numerical evidence for [`Self::poles`].
    pub pole_evidence: RootSetEvidence,
    /// Completeness and numerical evidence for [`Self::zeros`].
    pub zero_evidence: RootSetEvidence,
    /// DC gain H(0), when the transfer function has a finite DC value.
    pub dc_gain: Option<Value>,
    /// High-frequency gain H(∞) if finite
    pub hf_gain: Option<Value>,
    /// Input specification
    pub input: String,
    /// Output specification  
    pub output: String,
}

impl PoleZeroResult {
    /// Create empty result
    pub fn new(input: &str, output: &str) -> Self {
        Self {
            poles: Vec::new(),
            zeros: Vec::new(),
            pole_evidence: RootSetEvidence::NotRequested,
            zero_evidence: RootSetEvidence::NotRequested,
            dc_gain: None,
            hf_gain: None,
            input: input.to_string(),
            output: output.to_string(),
        }
    }

    /// Get real poles only
    pub fn real_poles(&self) -> Vec<&Complex64> {
        self.poles
            .iter()
            .filter(|p| p.im.abs() < REAL_ROOT_TOLERANCE)
            .collect()
    }

    /// Get dominant pole (slowest, closest to imaginary axis)
    pub fn dominant_pole(&self) -> Option<&Complex64> {
        self.poles
            .iter()
            .filter(|p| p.re.is_finite() && p.re < 0.0) // Only stable finite poles
            .min_by(|a, b| a.re.abs().total_cmp(&b.re.abs()))
    }

    /// Return a three-valued stability verdict from qualified pole evidence.
    pub fn stability_verdict(&self) -> StabilityVerdict {
        if !self.pole_evidence.is_consistent_with(&self.poles) || !self.pole_evidence.is_qualified()
        {
            return StabilityVerdict::Indeterminate;
        }
        if self
            .poles
            .iter()
            .any(|pole| !pole.re.is_finite() || !pole.im.is_finite())
        {
            return StabilityVerdict::Indeterminate;
        }
        if self.poles.iter().all(|pole| pole.re < 0.0) {
            StabilityVerdict::Stable
        } else {
            StabilityVerdict::Unstable
        }
    }

    /// Check whether qualified pole evidence proves asymptotic stability.
    pub fn is_stable(&self) -> bool {
        self.stability_verdict() == StabilityVerdict::Stable
    }

    /// Whether both root vectors agree with their attached evidence.
    pub fn has_consistent_root_evidence(&self) -> bool {
        self.pole_evidence.is_consistent_with(&self.poles)
            && self.zero_evidence.is_consistent_with(&self.zeros)
    }

    fn set_poles(&mut self, spectrum: ComputedSpectrum) {
        self.poles = spectrum.finite;
        self.pole_evidence = spectrum.evidence;
        debug_assert!(self.pole_evidence.is_consistent_with(&self.poles));
    }

    fn set_zeros(&mut self, spectrum: ComputedSpectrum) {
        self.zeros = spectrum.finite;
        self.zero_evidence = spectrum.evidence;
        debug_assert!(self.zero_evidence.is_consistent_with(&self.zeros));
    }

    /// Get bandwidth (frequency of dominant pole)
    pub fn bandwidth_hz(&self) -> Option<Value> {
        self.dominant_pole().map(|p| p.re.abs() / (2.0 * PI))
    }

    /// Sort poles by magnitude. Complex-conjugate pairs compare equal in
    /// magnitude, so ties order the positive-imaginary member first — the
    /// convention ngspice uses when printing pole tables.
    pub fn sort_poles_by_magnitude(&mut self) {
        Self::sort_by_magnitude_canonical(&mut self.poles);
    }

    /// Sort zeros by magnitude with the same conjugate-pair canonical order
    /// as [`Self::sort_poles_by_magnitude`].
    pub fn sort_zeros_by_magnitude(&mut self) {
        Self::sort_by_magnitude_canonical(&mut self.zeros);
    }

    fn sort_by_magnitude_canonical(values: &mut [Complex64]) {
        values.sort_by(|a, b| {
            let a_mag = a.norm();
            let b_mag = b.norm();
            let a_mag = if a_mag.is_finite() {
                a_mag
            } else {
                f64::INFINITY
            };
            let b_mag = if b_mag.is_finite() {
                b_mag
            } else {
                f64::INFINITY
            };
            a_mag.total_cmp(&b_mag).then(b.im.total_cmp(&a.im))
        });
    }
}

//=============================================================================
// Matrix Classes for Pole-Zero
//=============================================================================

/// Real matrix for pole-zero computations
#[derive(Debug, Clone)]
pub struct Matrix {
    data: Vec<Vec<Value>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    /// Create zero matrix
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![vec![0.0; cols]; rows],
            rows,
            cols,
        }
    }

    /// Create a matrix from dense row-major data.
    pub fn from_dense(data: Vec<Vec<Value>>) -> Self {
        let rows = data.len();
        let cols = data.first().map_or(0, Vec::len);
        Self { data, rows, cols }
    }

    /// Create identity matrix
    pub fn identity(n: usize) -> Self {
        let mut m = Self::zeros(n, n);
        for i in 0..n {
            m.data[i][i] = 1.0;
        }
        m
    }

    /// Get element
    pub fn get(&self, row: usize, col: usize) -> Value {
        self.data[row][col]
    }

    /// Set element
    pub fn set(&mut self, row: usize, col: usize, value: Value) {
        self.data[row][col] = value;
    }

    /// Add to element
    pub fn add(&mut self, row: usize, col: usize, value: Value) {
        self.data[row][col] += value;
    }

    /// Get dimensions
    pub fn dims(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }
}

impl std::ops::Add for &Matrix {
    type Output = Matrix;
    fn add(self, rhs: Self) -> Matrix {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);
        let mut result = Matrix::zeros(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }
        result
    }
}

impl std::ops::Mul<Value> for &Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Value) -> Matrix {
        let mut result = Matrix::zeros(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] * rhs;
            }
        }
        result
    }
}

//=============================================================================
// Pole-Zero Analyzer
//=============================================================================

/// Configuration for pole-zero analysis
#[derive(Debug, Clone)]
pub struct PoleZeroConfig {
    /// Input node (positive)
    pub input_pos: usize,
    /// Input node (negative, ground if None)
    pub input_neg: Option<usize>,
    /// Output node (positive)
    pub output_pos: usize,
    /// Output node (negative, ground if None)
    pub output_neg: Option<usize>,
    /// Whether input is current (vs voltage)
    pub input_is_current: bool,
    /// Existing MNA branch equation that defines the driven input voltage,
    /// when the circuit already contains an ideal voltage source on the input port.
    pub input_voltage_branch: Option<usize>,
    /// Sign for the branch-driven input voltage excitation relative to the
    /// requested input port polarity.
    pub input_voltage_gain: Value,
    /// Whether to compute poles
    pub compute_poles: bool,
    /// Whether to compute zeros
    pub compute_zeros: bool,
    /// Maximum pole magnitude to include (filter spurious)
    pub max_pole_freq: Value,
}

#[derive(Debug, Clone)]
struct DescriptorPartition {
    dynamic: Vec<usize>,
    algebraic: Vec<usize>,
    c_dd: Matrix,
    g_dd: Matrix,
    g_da: Matrix,
    g_ad: Matrix,
    g_aa: Matrix,
}

#[derive(Debug, Clone)]
struct StateSpaceModel {
    a: Matrix,
    b: Vec<Value>,
    c: Vec<Value>,
    d: Value,
}

/// Complete finite/infinite accounting from a generalized Schur solve.
#[derive(Debug, Clone)]
struct ComputedSpectrum {
    finite: Vec<Complex64>,
    evidence: RootSetEvidence,
}

impl ComputedSpectrum {
    fn from_certificate(
        finite: Vec<Complex64>,
        certificate: SpectrumCertificate,
    ) -> Result<Self, PoleZeroAnalysisError> {
        let evidence =
            RootSetEvidence::from_certificate(finite.len(), certificate).ok_or_else(|| {
                PoleZeroAnalysisError::InvalidSystem(
                    "computed spectrum evidence is internally inconsistent".to_string(),
                )
            })?;
        Ok(Self { finite, evidence })
    }

    fn exact(
        finite: Vec<Complex64>,
        problem_order: usize,
        infinite_count: usize,
    ) -> Result<Self, PoleZeroAnalysisError> {
        let certificate =
            SpectrumCertificate::exact(problem_order, infinite_count).ok_or_else(|| {
                PoleZeroAnalysisError::InvalidSystem(
                    "analytic spectrum accounting is internally inconsistent".to_string(),
                )
            })?;
        Self::from_certificate(finite, certificate)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TriangularKind {
    Lower,
    Upper,
}

impl PoleZeroConfig {
    /// Create default configuration for poles and zeros
    pub fn poles_and_zeros(input: usize, output: usize) -> Self {
        Self {
            input_pos: input,
            input_neg: None,
            output_pos: output,
            output_neg: None,
            input_is_current: true,
            input_voltage_branch: None,
            input_voltage_gain: 1.0,
            compute_poles: true,
            compute_zeros: true,
            max_pole_freq: 1e15,
        }
    }
}

/// Pole-zero analyzer using eigenvalue methods
pub struct PoleZeroAnalyzer {
    /// Conductance matrix G (frequency-independent)
    g_matrix: Matrix,
    /// Capacitance matrix C (coefficient of s)
    c_matrix: Matrix,
    /// Number of nodes
    num_nodes: usize,
}

impl PoleZeroAnalyzer {
    const APPROXIMATE_BACKWARD_ERROR_LIMIT: Value = 1.0e-8;

    fn qualification_tolerance(problem_order: usize) -> Value {
        128.0 * problem_order.max(1) as Value * Value::EPSILON
    }
}

mod gain;
mod matrix_ops;
mod poles;
mod roots;
mod zeros;

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn second_order_model(c: [Value; 2], d: Value) -> StateSpaceModel {
        // det(sI-A) = s^2 + 7s + 12, and for B=[0,1],
        // H(s)'s numerator is d*s^2 + (c1+7d)*s + (c0+12d).
        StateSpaceModel {
            a: Matrix::from_dense(vec![vec![0.0, 1.0], vec![-12.0, -7.0]]),
            b: vec![0.0, 1.0],
            c: c.to_vec(),
            d,
        }
    }

    fn state_space_zeros(model: &StateSpaceModel) -> Vec<Complex64> {
        let helper = PoleZeroAnalyzer::new(Matrix::identity(2), Matrix::identity(2));
        helper
            .zeros_from_state_space(model, &PoleZeroConfig::poles_and_zeros(0, 0))
            .expect("the fabricated state-space transfer has finite zeros")
            .finite
    }

    #[test]
    fn genuine_near_real_complex_zero_pair_remains_complex() {
        // N(s)=(s+1)^2+epsilon^2. It is close to a repeated real zero, but
        // its two roots are genuinely complex and must not be snapped to -1.
        let epsilon = 2.0e-7;
        let model = second_order_model([-11.0 + epsilon * epsilon, -5.0], 1.0);

        let zeros = state_space_zeros(&model);

        assert_eq!(zeros.len(), 2, "{zeros:#?}");
        assert!(
            zeros.iter().all(|zero| zero.im.abs() >= 0.5 * epsilon),
            "genuine complex roots were collapsed onto the real axis: {zeros:#?}"
        );
        assert!(
            zeros.iter().all(|zero| (zero.re + 1.0).abs() <= 1.0e-9),
            "unexpected real parts: {zeros:#?}"
        );
        assert!(
            (zeros[0].im + zeros[1].im).abs() <= 1.0e-9,
            "zeros are not a conjugate pair: {zeros:#?}"
        );
    }

    #[test]
    fn extreme_finite_quadratic_numerator_keeps_representable_roots() {
        // This is 1e307 * (s^2 + 2s + 2), whose roots are -1 +/- j. Every
        // signed coefficient and intermediate product is finite, although the
        // sum of absolute q0 terms exceeds f64::MAX and must be scaled safely.
        let scale = 1.0e307;
        let model = second_order_model([-10.0 * scale, -5.0 * scale], scale);

        let zeros = state_space_zeros(&model);

        assert_eq!(zeros.len(), 2, "{zeros:#?}");
        for zero in &zeros {
            assert!(
                (zero.re + 1.0).abs() <= 1.0e-9,
                "unexpected real part: {zeros:#?}"
            );
            assert!(
                (zero.im.abs() - 1.0).abs() <= 1.0e-9,
                "extreme finite coefficients lost the complex roots: {zeros:#?}"
            );
        }
    }
}
