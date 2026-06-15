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
use crate::Value;
use faer::{Mat, linalg::solvers::GeneralizedEigen};
use std::f64::consts::PI;

//=============================================================================
// Complex Number for Poles/Zeros
//=============================================================================

/// Complex number for representing poles and zeros
#[derive(Debug, Clone, Copy, Default)]
pub struct Complex {
    pub re: Value,
    pub im: Value,
}

impl Complex {
    pub const ZERO: Complex = Complex { re: 0.0, im: 0.0 };

    pub fn new(re: Value, im: Value) -> Self {
        Self { re, im }
    }

    pub fn real(re: Value) -> Self {
        Self { re, im: 0.0 }
    }

    pub fn magnitude(&self) -> Value {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    pub fn phase(&self) -> Value {
        self.im.atan2(self.re)
    }

    /// Check if this is a real pole/zero (imaginary part near zero)
    pub fn is_real(&self, tolerance: Value) -> bool {
        self.im.abs() < tolerance
    }

    /// Check if this is the conjugate of another complex number
    pub fn is_conjugate_of(&self, other: &Complex, tolerance: Value) -> bool {
        (self.re - other.re).abs() < tolerance && (self.im + other.im).abs() < tolerance
    }

    /// Get frequency in Hz (for imaginary pole/zero)
    pub fn frequency_hz(&self) -> Value {
        self.im.abs() / (2.0 * PI)
    }

    /// Get damping factor (for complex pole)
    /// ζ = -Re(p) / |p|
    pub fn damping_factor(&self) -> Value {
        let mag = self.magnitude();
        if mag > 1e-15 { -self.re / mag } else { 0.0 }
    }

    /// Get time constant (for real pole)
    /// τ = -1/Re(p)
    pub fn time_constant(&self) -> Option<Value> {
        if self.is_real(1e-10) && self.re.abs() > 1e-15 {
            Some(-1.0 / self.re)
        } else {
            None
        }
    }
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.im >= 0.0 {
            write!(f, "{:.6e}+{:.6e}j", self.re, self.im)
        } else {
            write!(f, "{:.6e}{:.6e}j", self.re, self.im)
        }
    }
}

//=============================================================================
// Pole-Zero Result
//=============================================================================

/// Result of pole-zero analysis
#[derive(Debug, Clone)]
pub struct PoleZeroResult {
    /// System poles (natural frequencies)
    pub poles: Vec<Complex>,
    /// System zeros
    pub zeros: Vec<Complex>,
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

    /// Add a pole
    pub fn add_pole(&mut self, pole: Complex) {
        self.poles.push(pole);
    }

    /// Add a zero
    pub fn add_zero(&mut self, zero: Complex) {
        self.zeros.push(zero);
    }

    /// Get real poles only
    pub fn real_poles(&self) -> Vec<&Complex> {
        self.poles.iter().filter(|p| p.is_real(1e-10)).collect()
    }

    /// Get complex conjugate pole pairs
    pub fn complex_pole_pairs(&self) -> Vec<(&Complex, &Complex)> {
        let mut pairs = Vec::new();
        let mut used = vec![false; self.poles.len()];

        for i in 0..self.poles.len() {
            if used[i] || self.poles[i].is_real(1e-10) {
                continue;
            }
            for j in (i + 1)..self.poles.len() {
                if !used[j] && self.poles[i].is_conjugate_of(&self.poles[j], 1e-10) {
                    pairs.push((&self.poles[i], &self.poles[j]));
                    used[i] = true;
                    used[j] = true;
                    break;
                }
            }
        }
        pairs
    }

    /// Get dominant pole (slowest, closest to imaginary axis)
    pub fn dominant_pole(&self) -> Option<&Complex> {
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

    fn sort_by_magnitude_canonical(values: &mut [Complex]) {
        values.sort_by(|a, b| {
            let a_mag = a.magnitude();
            let b_mag = b.magnitude();
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

    /// Matrix-vector multiply
    pub fn mul_vec(&self, v: &[Value]) -> Vec<Value> {
        assert_eq!(v.len(), self.cols);
        let mut result = vec![0.0; self.rows];
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[i] += self.data[i][j] * v[j];
            }
        }
        result
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

    /// Poles only
    pub fn poles_only(input: usize, output: usize) -> Self {
        let mut config = Self::poles_and_zeros(input, output);
        config.compute_zeros = false;
        config
    }

    /// Zeros only
    pub fn zeros_only(input: usize, output: usize) -> Self {
        let mut config = Self::poles_and_zeros(input, output);
        config.compute_poles = false;
        config
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
