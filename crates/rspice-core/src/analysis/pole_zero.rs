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

/// A root counts as real when its imaginary part sits inside this band. The
/// eigen solvers hand back conjugate pairs carrying a residual imaginary part,
/// so an exact zero test would report every real root as complex.
const REAL_ROOT_TOLERANCE: Value = 1e-10;

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
    /// DC gain H(0)
    pub dc_gain: Value,
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
            dc_gain: 1.0,
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

    /// Check if system is stable (all poles have negative real parts)
    pub fn is_stable(&self) -> bool {
        self.poles.iter().all(|p| p.re < 1e-10)
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

mod gain;
mod matrix_ops;
mod poles;
mod roots;
mod zeros;

//=============================================================================
// Tests
//=============================================================================
