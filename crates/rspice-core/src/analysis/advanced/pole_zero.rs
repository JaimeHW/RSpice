//! Pole-Zero Analysis (.PZ)
//!
//! Finds the poles and zeros of a circuit's transfer function.
//!
//! # Theory
//!
//! For a linear circuit, the transfer function H(s) can be expressed as:
//!
//! ```text
//! H(s) = K Â· âˆ(s - záµ¢) / âˆ(s - pâ±¼)
//! ```
//!
//! - **Poles (pâ±¼)**: Values of s where H(s) â†’ âˆž (natural frequencies)
//! - **Zeros (záµ¢)**: Values of s where H(s) = 0
//!
//! # Algorithm
//!
//! 1. Build MNA matrix as Y(s) = G + sÂ·C where G is conductance, C is capacitance
//! 2. **Poles**: Solve generalized eigenvalue problem GÂ·x = -sÂ·CÂ·x
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
    /// Î¶ = -Re(p) / |p|
    pub fn damping_factor(&self) -> Value {
        let mag = self.magnitude();
        if mag > 1e-15 { -self.re / mag } else { 0.0 }
    }

    /// Get time constant (for real pole)
    /// Ï„ = -1/Re(p)
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
    /// High-frequency gain H(âˆž) if finite
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

    /// Sort poles by magnitude
    pub fn sort_poles_by_magnitude(&mut self) {
        self.poles.sort_by(|a, b| {
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
            a_mag.total_cmp(&b_mag)
        });
    }

    /// Sort zeros by magnitude
    pub fn sort_zeros_by_magnitude(&mut self) {
        self.zeros.sort_by(|a, b| {
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
            a_mag.total_cmp(&b_mag)
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

impl PoleZeroAnalyzer {
    /// Create analyzer from G and C matrices
    ///
    /// The MNA equation is: (G + sÂ·C)Â·x = b
    /// Poles are values of s where det(G + sÂ·C) = 0
    pub fn new(g_matrix: Matrix, c_matrix: Matrix) -> Self {
        let num_nodes = g_matrix.dims().0;
        Self {
            g_matrix,
            c_matrix,
            num_nodes,
        }
    }

    /// Find poles using companion matrix method
    ///
    /// Poles are eigenvalues of -Câ»Â¹Â·G (if C is invertible)
    /// For singular C, use generalized eigenvalue: GÂ·x = -sÂ·CÂ·x
    pub fn find_poles(&self, config: &PoleZeroConfig) -> Vec<Complex> {
        let n = self.num_nodes;
        if n == 0 {
            return Vec::new();
        }
        let expected_poles = self.finite_pole_count();

        // For single-node RC circuit:
        // G + sÂ·C = 0 â†’ s = -G/C
        if n == 1 {
            let g = self.g_matrix.get(0, 0);
            let c = self.c_matrix.get(0, 0);
            if c.abs() > 1e-15 {
                return vec![Complex::real(-g / c)];
            }
            return Vec::new();
        }

        if let Some(state_space) = self.build_state_space(&vec![0.0; n], &vec![0.0; n])
            && let Some(mut poles) = self.eigenvalues_from_matrix(&state_space.a)
        {
            self.canonicalize_real_roots(&mut poles);
            poles.retain(|p| {
                p.re.is_finite()
                    && p.im.is_finite()
                    && p.magnitude() < config.max_pole_freq * 2.0 * PI
            });
            poles.sort_by(|a, b| a.magnitude().total_cmp(&b.magnitude()));
            if expected_poles > 0 && poles.len() > expected_poles {
                poles.truncate(expected_poles);
            }
            if self.has_complete_pole_set(&poles, expected_poles) {
                return poles;
            }
        }

        if let Some(mut poles) = self.generalized_eigenvalues(&self.g_matrix, &self.c_matrix) {
            self.canonicalize_real_roots(&mut poles);
            poles.retain(|p| {
                p.re.is_finite()
                    && p.im.is_finite()
                    && p.magnitude() < config.max_pole_freq * 2.0 * PI
            });
            poles.sort_by(|a, b| a.magnitude().total_cmp(&b.magnitude()));
            if expected_poles > 0 && poles.len() > expected_poles {
                poles.truncate(expected_poles);
            }
            if self.has_complete_pole_set(&poles, expected_poles) {
                return poles;
            }
        }

        if let Some(state_matrix) = self.build_descriptor_state_matrix() {
            let mut poles = self.qr_eigenvalues(&state_matrix);
            self.canonicalize_real_roots(&mut poles);
            poles.retain(|p| {
                p.re.is_finite()
                    && p.im.is_finite()
                    && p.magnitude() < config.max_pole_freq * 2.0 * PI
            });
            poles.sort_by(|a, b| a.magnitude().total_cmp(&b.magnitude()));
            if expected_poles > 0 && poles.len() > expected_poles {
                poles.truncate(expected_poles);
            }
            if self.has_complete_pole_set(&poles, expected_poles) {
                return poles;
            }
        }

        // Fallback for heavily singular descriptors where state extraction fails.
        self.eigenvalues_diagonal_fallback(config)
    }

    /// Descriptor-system reduction:
    /// Cx' + Gx = 0
    /// Split x into dynamic/algebraic variables and eliminate algebraic states.
    /// This yields x_d' = A x_d where A is used for pole extraction.
    fn build_descriptor_state_matrix(&self) -> Option<Matrix> {
        let partition = self.partition_descriptor()?;
        if !self.partition_is_regular(&partition) {
            return None;
        }

        let g_eff = self.reduced_g_matrix(&partition)?;
        let c_dd_inv_g_eff = self.solve_matrix_columns_regularized(&partition.c_dd, &g_eff)?;
        let mut a = c_dd_inv_g_eff;
        for row in &mut a.data {
            for value in row {
                *value = -*value;
            }
        }

        Some(a)
    }

    fn partition_descriptor(&self) -> Option<DescriptorPartition> {
        let n = self.num_nodes;
        let tol = 1e-15;

        let mut dynamic = Vec::new();
        for i in 0..n {
            let row_nonzero = self.c_matrix.data[i].iter().any(|v| v.abs() > tol);
            let col_nonzero = (0..n).any(|r| self.c_matrix.data[r][i].abs() > tol);
            if row_nonzero || col_nonzero {
                dynamic.push(i);
            }
        }

        if dynamic.is_empty() {
            return None;
        }

        let mut is_dynamic = vec![false; n];
        for &idx in &dynamic {
            is_dynamic[idx] = true;
        }
        let algebraic: Vec<usize> = (0..n).filter(|i| !is_dynamic[*i]).collect();

        Some(DescriptorPartition {
            c_dd: self.extract_submatrix(&self.c_matrix, &dynamic, &dynamic),
            g_dd: self.extract_submatrix(&self.g_matrix, &dynamic, &dynamic),
            g_da: self.extract_submatrix(&self.g_matrix, &dynamic, &algebraic),
            g_ad: self.extract_submatrix(&self.g_matrix, &algebraic, &dynamic),
            g_aa: self.extract_submatrix(&self.g_matrix, &algebraic, &algebraic),
            dynamic,
            algebraic,
        })
    }

    fn reduced_g_matrix(&self, partition: &DescriptorPartition) -> Option<Matrix> {
        if partition.algebraic.is_empty() {
            return Some(partition.g_dd.clone());
        }

        let g_aa_inv_g_ad =
            self.solve_matrix_columns_regularized(&partition.g_aa, &partition.g_ad)?;
        let correction = self.matrix_multiply(&partition.g_da, &g_aa_inv_g_ad);
        Some(self.matrix_subtract(&partition.g_dd, &correction))
    }

    fn partition_is_regular(&self, partition: &DescriptorPartition) -> bool {
        if !self.matrix_has_stable_inverse(&partition.c_dd) {
            return false;
        }

        if partition.algebraic.is_empty() {
            return true;
        }

        self.matrix_has_stable_inverse(&partition.g_aa)
    }

    fn extract_subvector(&self, values: &[Value], indices: &[usize]) -> Vec<Value> {
        indices.iter().map(|&idx| values[idx]).collect()
    }

    fn vector_to_column_matrix(&self, values: &[Value]) -> Matrix {
        let mut column = Matrix::zeros(values.len(), 1);
        for (row, value) in values.iter().enumerate() {
            column.data[row][0] = *value;
        }
        column
    }

    fn column_matrix_to_vector(&self, column: &Matrix) -> Vec<Value> {
        (0..column.rows).map(|row| column.data[row][0]).collect()
    }

    fn row_vector_times_matrix(&self, row: &[Value], matrix: &Matrix) -> Vec<Value> {
        assert_eq!(row.len(), matrix.rows);
        let mut out = vec![0.0; matrix.cols];
        for (r, weight) in row.iter().copied().enumerate() {
            if weight == 0.0 {
                continue;
            }
            for (c, target) in out.iter_mut().enumerate().take(matrix.cols) {
                *target += weight * matrix.data[r][c];
            }
        }
        out
    }

    fn build_state_space(
        &self,
        input_vec: &[Value],
        output_vec: &[Value],
    ) -> Option<StateSpaceModel> {
        let partition = self.partition_descriptor()?;
        if !self.partition_is_regular(&partition) {
            return None;
        }
        let g_eff = self.reduced_g_matrix(&partition)?;

        let b_d = self.extract_subvector(input_vec, &partition.dynamic);
        let l_d = self.extract_subvector(output_vec, &partition.dynamic);

        let (b_eff, c_eff, d_eff) = if partition.algebraic.is_empty() {
            (b_d, l_d, 0.0)
        } else {
            let b_a = self.extract_subvector(input_vec, &partition.algebraic);
            let l_a = self.extract_subvector(output_vec, &partition.algebraic);
            let g_aa_inv_g_ad =
                self.solve_matrix_columns_regularized(&partition.g_aa, &partition.g_ad)?;
            let g_aa_inv_ba = self.solve_matrix_columns_regularized(
                &partition.g_aa,
                &self.vector_to_column_matrix(&b_a),
            )?;

            let gda_ginv_ba = self.matrix_multiply(&partition.g_da, &g_aa_inv_ba);
            let mut b_eff = b_d;
            for (target, correction) in b_eff
                .iter_mut()
                .zip(self.column_matrix_to_vector(&gda_ginv_ba))
            {
                *target -= correction;
            }

            let l_a_ginv_gad = self.row_vector_times_matrix(&l_a, &g_aa_inv_g_ad);
            let mut c_eff = l_d;
            for (target, correction) in c_eff.iter_mut().zip(l_a_ginv_gad) {
                *target -= correction;
            }

            let d_eff = self
                .row_vector_times_matrix(&l_a, &g_aa_inv_ba)
                .into_iter()
                .next()
                .unwrap_or(0.0);

            (b_eff, c_eff, d_eff)
        };

        let mut a = self.solve_matrix_columns_regularized(&partition.c_dd, &g_eff)?;
        for row in &mut a.data {
            for value in row {
                *value = -*value;
            }
        }

        let b = self.solve_matrix_columns_regularized(
            &partition.c_dd,
            &self.vector_to_column_matrix(&b_eff),
        )?;

        Some(StateSpaceModel {
            a,
            b: self.column_matrix_to_vector(&b),
            c: c_eff,
            d: d_eff,
        })
    }

    fn extract_submatrix(&self, m: &Matrix, rows: &[usize], cols: &[usize]) -> Matrix {
        let mut out = Matrix::zeros(rows.len(), cols.len());
        for (ri, &src_r) in rows.iter().enumerate() {
            for (ci, &src_c) in cols.iter().enumerate() {
                out.data[ri][ci] = m.data[src_r][src_c];
            }
        }
        out
    }

    fn matrix_subtract(&self, a: &Matrix, b: &Matrix) -> Matrix {
        assert_eq!(a.rows, b.rows);
        assert_eq!(a.cols, b.cols);
        let mut out = Matrix::zeros(a.rows, a.cols);
        for i in 0..a.rows {
            for j in 0..a.cols {
                out.data[i][j] = a.data[i][j] - b.data[i][j];
            }
        }
        out
    }

    fn matrix_multiply(&self, a: &Matrix, b: &Matrix) -> Matrix {
        assert_eq!(a.cols, b.rows);
        let mut out = Matrix::zeros(a.rows, b.cols);
        for i in 0..a.rows {
            for j in 0..b.cols {
                let mut sum = 0.0;
                for k in 0..a.cols {
                    sum += a.data[i][k] * b.data[k][j];
                }
                out.data[i][j] = sum;
            }
        }
        out
    }

    fn solve_matrix_columns_regularized(&self, a: &Matrix, b: &Matrix) -> Option<Matrix> {
        assert_eq!(a.rows, a.cols);
        assert_eq!(a.rows, b.rows);

        let scale = a
            .data
            .iter()
            .flatten()
            .map(|value| value.abs())
            .fold(0.0_f64, f64::max)
            .max(1.0);

        // Try exact solve first, then progressively stronger diagonal regularization.
        let regularizations = [
            0.0,
            1e-18 * scale,
            1e-15 * scale,
            1e-12 * scale,
            1e-9 * scale,
            1e-6 * scale,
        ];
        for &eps in &regularizations {
            let mut a_reg = a.clone();
            if eps > 0.0 {
                for i in 0..a_reg.rows.min(a_reg.cols) {
                    a_reg.data[i][i] += eps;
                }
            }
            if let Some(x) = self.solve_matrix_columns(&a_reg, b) {
                return Some(x);
            }
        }

        None
    }

    fn solve_matrix_columns(&self, a: &Matrix, b: &Matrix) -> Option<Matrix> {
        assert_eq!(a.rows, a.cols);
        assert_eq!(a.rows, b.rows);

        let mut out = Matrix::zeros(a.rows, b.cols);
        let triangular = self.triangular_kind(a, self.relative_matrix_tolerance(a, 1e-12));
        for col in 0..b.cols {
            let rhs: Vec<Value> = (0..b.rows).map(|r| b.data[r][col]).collect();
            let x = if let Some(kind) = triangular {
                self.solve_triangular(a, &rhs, kind)?
            } else {
                self.solve_linear(a, &rhs)?
            };
            for (row, value) in x.into_iter().enumerate() {
                out.data[row][col] = value;
            }
        }
        Some(out)
    }

    fn qr_eigenvalues(&self, matrix: &Matrix) -> Vec<Complex> {
        let n = matrix.rows;
        if n == 0 {
            return Vec::new();
        }
        let scale = self.matrix_eigen_scale(matrix);
        let scaled = self.scale_matrix(matrix, 1.0 / scale);
        let tol = 1e-10;
        if let Some(diagonal_roots) = self.triangular_diagonal_eigenvalues(&scaled, tol) {
            return diagonal_roots
                .into_iter()
                .map(|root| Complex::new(root.re * scale, root.im * scale))
                .collect();
        }
        if n == 1 {
            return vec![Complex::real(scaled.data[0][0] * scale)];
        }
        if n == 2 {
            return self
                .eigenvalues_2x2(
                    scaled.data[0][0],
                    scaled.data[0][1],
                    scaled.data[1][0],
                    scaled.data[1][1],
                )
                .into_iter()
                .map(|root| Complex::new(root.re * scale, root.im * scale))
                .collect();
        }

        let max_iter = 2000;
        let mut a = scaled.data.clone();

        for _ in 0..max_iter {
            let mut converged = true;
            for i in 1..n {
                if a[i][i - 1].abs() > tol {
                    converged = false;
                    break;
                }
            }
            if converged {
                break;
            }

            // Basic shifted QR iteration.
            let shift = a[n - 1][n - 1];
            for (i, row) in a.iter_mut().enumerate().take(n) {
                row[i] -= shift;
            }

            let (q, r) = self.qr_decompose(&a);
            a = self.matrix_multiply_raw(&r, &q);

            for (i, row) in a.iter_mut().enumerate().take(n) {
                row[i] += shift;
            }
        }

        let mut eigenvalues = Vec::with_capacity(n);
        let mut i = 0;
        while i < n {
            if i == n - 1 || a[i + 1][i].abs() < tol {
                eigenvalues.push(Complex::real(a[i][i]));
                i += 1;
            } else {
                eigenvalues.extend(self.eigenvalues_2x2(
                    a[i][i],
                    a[i][i + 1],
                    a[i + 1][i],
                    a[i + 1][i + 1],
                ));
                i += 2;
            }
        }

        eigenvalues
            .into_iter()
            .map(|root| Complex::new(root.re * scale, root.im * scale))
            .collect()
    }

    fn eigenvalues_2x2(&self, a00: Value, a01: Value, a10: Value, a11: Value) -> Vec<Complex> {
        let trace = a00 + a11;
        let det = a00 * a11 - a01 * a10;
        let discriminant = trace * trace - 4.0 * det;

        if discriminant >= 0.0 {
            let sqrt_d = discriminant.sqrt();
            vec![
                Complex::real((trace + sqrt_d) / 2.0),
                Complex::real((trace - sqrt_d) / 2.0),
            ]
        } else {
            let sqrt_d = (-discriminant).sqrt() / 2.0;
            vec![
                Complex::new(trace / 2.0, sqrt_d),
                Complex::new(trace / 2.0, -sqrt_d),
            ]
        }
    }

    fn qr_decompose(&self, a: &[Vec<Value>]) -> (Vec<Vec<Value>>, Vec<Vec<Value>>) {
        let n = a.len();
        let mut q = vec![vec![0.0; n]; n];
        let mut r = vec![vec![0.0; n]; n];
        let cols: Vec<Vec<Value>> = (0..n).map(|j| (0..n).map(|i| a[i][j]).collect()).collect();

        for j in 0..n {
            let mut v = cols[j].clone();
            for i in 0..j {
                let q_col: Vec<Value> = (0..n).map(|k| q[k][i]).collect();
                let dot: Value = v.iter().zip(&q_col).map(|(x, y)| x * y).sum();
                r[i][j] = dot;
                for k in 0..n {
                    v[k] -= dot * q_col[k];
                }
            }

            let norm = v.iter().map(|x| x * x).sum::<Value>().sqrt();
            r[j][j] = norm;
            if norm > 1e-15 {
                for k in 0..n {
                    q[k][j] = v[k] / norm;
                }
            }
        }

        (q, r)
    }

    fn matrix_multiply_raw(&self, a: &[Vec<Value>], b: &[Vec<Value>]) -> Vec<Vec<Value>> {
        let n = a.len();
        let mut out = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..n {
                let mut sum = 0.0;
                for k in 0..n {
                    sum += a[i][k] * b[k][j];
                }
                out[i][j] = sum;
            }
        }
        out
    }

    /// Fallback pole estimator for highly singular systems.
    fn eigenvalues_diagonal_fallback(&self, config: &PoleZeroConfig) -> Vec<Complex> {
        let n = self.num_nodes;
        let mut poles = Vec::new();
        for i in 0..n {
            let g = self.g_matrix.get(i, i);
            let c = self.c_matrix.get(i, i);
            if c.abs() > 1e-15 && g.abs() > 1e-15 {
                let pole = -g / c;
                if pole.abs() < config.max_pole_freq * 2.0 * PI {
                    poles.push(Complex::real(pole));
                }
            }
        }
        poles.sort_by(|a, b| {
            let a_re = if a.re.is_finite() {
                a.re
            } else {
                f64::INFINITY
            };
            let b_re = if b.re.is_finite() {
                b.re
            } else {
                f64::INFINITY
            };
            a_re.total_cmp(&b_re)
        });
        poles.dedup_by(|a, b| (a.re - b.re).abs() < 1e-6);
        poles
    }

    fn build_port_vectors(&self, config: &PoleZeroConfig) -> Option<(Vec<Value>, Vec<Value>)> {
        let n = self.num_nodes;
        let mut input_vec: Vec<Value> = vec![0.0; n];
        let mut output_vec: Vec<Value> = vec![0.0; n];

        if config.input_pos >= n || config.output_pos >= n {
            return None;
        }

        input_vec[config.input_pos] += 1.0;
        if let Some(input_neg) = config.input_neg {
            if input_neg >= n {
                return None;
            }
            input_vec[input_neg] -= 1.0;
        }

        output_vec[config.output_pos] += 1.0;
        if let Some(output_neg) = config.output_neg {
            if output_neg >= n {
                return None;
            }
            output_vec[output_neg] -= 1.0;
        }

        let input_norm = input_vec.iter().map(|v| v.abs()).sum::<Value>();
        let output_norm = output_vec.iter().map(|v| v.abs()).sum::<Value>();
        if input_norm < 1e-15 || output_norm < 1e-15 {
            return None;
        }

        Some((input_vec, output_vec))
    }

    fn is_direct_voltage_port_measurement(&self, config: &PoleZeroConfig) -> bool {
        if config.input_is_current {
            return false;
        }

        (config.input_pos == config.output_pos && config.input_neg == config.output_neg)
            || (config.input_pos == config.output_neg.unwrap_or(usize::MAX)
                && config.output_pos == config.input_neg.unwrap_or(usize::MAX))
    }

    fn is_same_root(a: &Complex, b: &Complex, tol: Value) -> bool {
        let re_scale = 1.0 + a.re.abs().max(b.re.abs());
        let im_scale = 1.0 + a.im.abs().max(b.im.abs());
        (a.re - b.re).abs() <= tol * re_scale && (a.im - b.im).abs() <= tol * im_scale
    }

    fn sort_roots(&self, roots: &mut [Complex]) {
        roots.sort_by(|a, b| {
            let a_re = if a.re.is_finite() {
                a.re
            } else {
                f64::INFINITY
            };
            let b_re = if b.re.is_finite() {
                b.re
            } else {
                f64::INFINITY
            };
            let a_im = if a.im.is_finite() {
                a.im
            } else {
                f64::INFINITY
            };
            let b_im = if b.im.is_finite() {
                b.im
            } else {
                f64::INFINITY
            };
            a_re.total_cmp(&b_re).then_with(|| a_im.total_cmp(&b_im))
        });
    }

    fn round_to_significant_digits(&self, value: Value, digits: i32) -> Value {
        if !value.is_finite() || value == 0.0 {
            return value;
        }

        let exponent = value.abs().log10().floor() as i32;
        let scale = 10.0_f64.powi(digits - exponent - 1);
        (value * scale).round() / scale
    }

    fn canonicalize_real_roots(&self, roots: &mut [Complex]) {
        for root in roots {
            if !root.re.is_finite() || !root.im.is_finite() {
                continue;
            }
            if root.im.abs() <= (1.0 + root.re.abs()) * 1e-12 {
                root.im = 0.0;
            }
            if root.im == 0.0 {
                let rounded = self.round_to_significant_digits(root.re, 8);
                let tolerance = (1.0 + root.re.abs()) * 1e-6;
                if (rounded - root.re).abs() <= tolerance {
                    root.re = rounded;
                }
            }
        }
    }

    fn canonicalize_near_real_zero_pairs(&self, zeros: &mut [Complex]) {
        let snap_ratio = 1e-6;
        let real_tolerance = 1e-9;

        for idx in 0..zeros.len().saturating_sub(1) {
            let (left, right) = zeros.split_at_mut(idx + 1);
            let a = &mut left[idx];
            let b = &mut right[0];

            if !a.re.is_finite() || !a.im.is_finite() || !b.re.is_finite() || !b.im.is_finite() {
                continue;
            }
            if (a.re - b.re).abs() > (1.0 + a.re.abs().max(b.re.abs())) * real_tolerance {
                continue;
            }
            if (a.im + b.im).abs() > (1.0 + a.im.abs().max(b.im.abs())) * real_tolerance {
                continue;
            }

            let imag_scale = a.im.abs().max(b.im.abs());
            let root_scale = 1.0 + a.re.abs().max(b.re.abs());
            if imag_scale <= root_scale * snap_ratio {
                a.re = (a.re + b.re) * 0.5;
                b.re = a.re;
                a.im = 0.0;
                b.im = 0.0;
            }
        }

        self.canonicalize_real_roots(zeros);
    }

    fn finite_pole_count(&self) -> usize {
        self.matrix_rank(
            &self.c_matrix,
            self.relative_matrix_tolerance(&self.c_matrix, 1e-9),
        )
    }

    fn has_complete_pole_set(&self, poles: &[Complex], expected: usize) -> bool {
        if expected == 0 {
            return true;
        }
        poles.len() == expected
    }

    fn relative_matrix_tolerance(&self, matrix: &Matrix, relative_tolerance: Value) -> Value {
        let max_abs = matrix
            .data
            .iter()
            .flatten()
            .map(|value| value.abs())
            .fold(0.0_f64, f64::max);
        if max_abs > 0.0 {
            relative_tolerance * max_abs
        } else {
            relative_tolerance
        }
    }

    fn matrix_eigen_scale(&self, matrix: &Matrix) -> Value {
        matrix
            .data
            .iter()
            .flatten()
            .map(|value| value.abs())
            .fold(0.0_f64, f64::max)
            .max(1.0)
    }

    fn scale_matrix(&self, matrix: &Matrix, factor: Value) -> Matrix {
        let mut scaled = matrix.clone();
        for row in &mut scaled.data {
            for value in row {
                *value *= factor;
            }
        }
        scaled
    }

    fn triangular_diagonal_eigenvalues(
        &self,
        matrix: &Matrix,
        tolerance: Value,
    ) -> Option<Vec<Complex>> {
        let _ = self.triangular_kind(matrix, tolerance)?;

        Some(
            (0..matrix.rows)
                .map(|idx| Complex::real(matrix.data[idx][idx]))
                .collect(),
        )
    }

    fn triangular_kind(&self, matrix: &Matrix, tolerance: Value) -> Option<TriangularKind> {
        if matrix.rows != matrix.cols {
            return None;
        }

        let mut lower = true;
        let mut upper = true;
        for row in 0..matrix.rows {
            for col in 0..matrix.cols {
                let value = matrix.data[row][col].abs();
                if row > col && value > tolerance {
                    upper = false;
                }
                if col > row && value > tolerance {
                    lower = false;
                }
            }
        }

        if lower {
            Some(TriangularKind::Lower)
        } else if upper {
            Some(TriangularKind::Upper)
        } else {
            None
        }
    }

    fn solve_triangular(
        &self,
        a: &Matrix,
        b: &[Value],
        kind: TriangularKind,
    ) -> Option<Vec<Value>> {
        let n = a.rows;
        let pivot_tolerance = self.relative_matrix_tolerance(a, 1e-15);
        let mut x = vec![0.0; n];

        match kind {
            TriangularKind::Lower => {
                for row in 0..n {
                    let pivot = a.data[row][row];
                    if pivot.abs() <= pivot_tolerance {
                        return None;
                    }
                    let mut sum = b[row];
                    for (col, value) in x.iter().enumerate().take(row) {
                        sum -= a.data[row][col] * *value;
                    }
                    x[row] = sum / pivot;
                }
            }
            TriangularKind::Upper => {
                for row in (0..n).rev() {
                    let pivot = a.data[row][row];
                    if pivot.abs() <= pivot_tolerance {
                        return None;
                    }
                    let mut sum = b[row];
                    for (col, value) in x.iter().enumerate().skip(row + 1) {
                        sum -= a.data[row][col] * *value;
                    }
                    x[row] = sum / pivot;
                }
            }
        }

        Some(x)
    }

    fn matrix_has_stable_inverse(&self, matrix: &Matrix) -> bool {
        if matrix.rows != matrix.cols {
            return false;
        }
        if matrix.rows == 0 {
            return true;
        }
        if self
            .triangular_kind(matrix, self.relative_matrix_tolerance(matrix, 1e-12))
            .is_some()
        {
            let pivot_tolerance = self.relative_matrix_tolerance(matrix, 1e-15);
            return (0..matrix.rows).all(|idx| matrix.data[idx][idx].abs() > pivot_tolerance);
        }

        let identity = Matrix::identity(matrix.rows);
        let Some(inverse) = self.solve_matrix_columns_regularized(matrix, &identity) else {
            return false;
        };
        let product = self.matrix_multiply(matrix, &inverse);
        let mut max_residual = 0.0_f64;
        for row in 0..matrix.rows {
            for col in 0..matrix.cols {
                let expected = if row == col { 1.0 } else { 0.0 };
                max_residual = max_residual.max((product.data[row][col] - expected).abs());
            }
        }

        max_residual <= 1e-6
    }

    fn matrix_rank(&self, matrix: &Matrix, tolerance: Value) -> usize {
        let (rows, cols) = matrix.dims();
        if rows == 0 || cols == 0 {
            return 0;
        }

        let mut data = matrix.data.clone();
        let mut rank = 0usize;
        let mut pivot_row = 0usize;

        for pivot_col in 0..cols {
            if pivot_row >= rows {
                break;
            }

            let mut best_row = pivot_row;
            let mut best_value = data[pivot_row][pivot_col].abs();
            for (row_idx, row) in data.iter().enumerate().skip(pivot_row + 1) {
                let candidate = row[pivot_col].abs();
                if candidate > best_value {
                    best_value = candidate;
                    best_row = row_idx;
                }
            }

            if best_value <= tolerance {
                continue;
            }

            data.swap(pivot_row, best_row);
            let pivot = data[pivot_row][pivot_col];
            for row_idx in (pivot_row + 1)..rows {
                let factor = data[row_idx][pivot_col] / pivot;
                if factor.abs() <= tolerance {
                    continue;
                }
                for col_idx in pivot_col..cols {
                    data[row_idx][col_idx] -= factor * data[pivot_row][col_idx];
                }
            }

            rank += 1;
            pivot_row += 1;
        }

        rank
    }

    fn numerator_roots_raw(
        &self,
        input_vec: &[Value],
        output_vec: &[Value],
        config: &PoleZeroConfig,
    ) -> Vec<Complex> {
        if self.num_nodes == 0 {
            return Vec::new();
        }
        if self.num_nodes == 1 {
            return Vec::new();
        }
        if self.num_nodes == 2 {
            if let Some(root) = self.numerator_root_2x2(input_vec, output_vec) {
                return vec![root];
            }
            return Vec::new();
        }

        let n = self.num_nodes;
        let mut g_aug = Matrix::zeros(n + 1, n + 1);
        let mut c_aug = Matrix::zeros(n + 1, n + 1);

        for i in 0..n {
            for j in 0..n {
                g_aug.set(i, j, self.g_matrix.get(i, j));
                c_aug.set(i, j, self.c_matrix.get(i, j));
            }
            g_aug.set(i, n, -input_vec[i]);
            g_aug.set(n, i, output_vec[i]);
        }

        self.generalized_eigenvalues(&g_aug, &c_aug)
            .map(|mut roots| {
                roots.retain(|r| {
                    r.re.is_finite()
                        && r.im.is_finite()
                        && r.magnitude() < config.max_pole_freq * 2.0 * PI
                });
                roots
            })
            .unwrap_or_default()
    }

    fn to_faer_matrix(&self, matrix: &Matrix) -> Mat<f64> {
        let mut out = Mat::zeros(matrix.rows, matrix.cols);
        for row in 0..matrix.rows {
            for col in 0..matrix.cols {
                out[(row, col)] = matrix.data[row][col];
            }
        }
        out
    }

    fn eigenvalues_from_matrix(&self, matrix: &Matrix) -> Option<Vec<Complex>> {
        if matrix.rows == 0 || matrix.rows != matrix.cols {
            return None;
        }
        let scale = self.matrix_eigen_scale(matrix);
        let scaled = self.scale_matrix(matrix, 1.0 / scale);
        if let Some(diagonal_roots) = self.triangular_diagonal_eigenvalues(&scaled, 1e-10) {
            return Some(
                diagonal_roots
                    .into_iter()
                    .map(|root| Complex::new(root.re * scale, root.im * scale))
                    .collect(),
            );
        }
        let faer_matrix = self.to_faer_matrix(&scaled);
        let eigen =
            faer::linalg::solvers::Eigen::<f64>::new_from_real(faer_matrix.as_ref()).ok()?;
        let spectrum = eigen.S().column_vector();
        let mut eigenvalues = Vec::with_capacity(matrix.rows);
        for idx in 0..matrix.rows {
            let value = *spectrum.get(idx);
            if value.re.is_finite() && value.im.is_finite() {
                eigenvalues.push(Complex::new(value.re * scale, value.im * scale));
            }
        }
        Some(eigenvalues)
    }

    fn generalized_eigenvalues(
        &self,
        g_matrix: &Matrix,
        c_matrix: &Matrix,
    ) -> Option<Vec<Complex>> {
        let n = g_matrix.rows;
        if n == 0 || g_matrix.rows != g_matrix.cols || c_matrix.rows != c_matrix.cols {
            return None;
        }
        if g_matrix.rows != c_matrix.rows {
            return None;
        }

        let scale = self
            .matrix_eigen_scale(g_matrix)
            .max(self.matrix_eigen_scale(c_matrix));
        let g_scaled = self.scale_matrix(g_matrix, 1.0 / scale);
        let c_scaled = self.scale_matrix(c_matrix, 1.0 / scale);

        let mut a = self.to_faer_matrix(&g_scaled);
        for row in 0..n {
            for col in 0..n {
                a[(row, col)] = -a[(row, col)];
            }
        }
        let b = self.to_faer_matrix(&c_scaled);
        let gevd = GeneralizedEigen::<f64>::new_from_real(a.as_ref(), b.as_ref()).ok()?;
        let alpha = gevd.S_a().column_vector();
        let beta = gevd.S_b().column_vector();

        let mut eigenvalues = Vec::with_capacity(n);
        for idx in 0..n {
            let alpha = *alpha.get(idx);
            let beta = *beta.get(idx);
            if !alpha.re.is_finite()
                || !alpha.im.is_finite()
                || !beta.re.is_finite()
                || !beta.im.is_finite()
            {
                continue;
            }

            let beta_norm = beta.norm();
            if beta_norm <= 1e-18 {
                continue;
            }

            let lambda = alpha / beta;
            if lambda.re.is_finite() && lambda.im.is_finite() {
                eigenvalues.push(Complex::new(lambda.re, lambda.im));
            }
        }

        Some(eigenvalues)
    }

    fn zeros_from_state_space(
        &self,
        model: &StateSpaceModel,
        poles: &[Complex],
        config: &PoleZeroConfig,
    ) -> Vec<Complex> {
        let n = model.a.rows;
        if n == 0 {
            return Vec::new();
        }

        let mut g_zero = Matrix::zeros(n + 1, n + 1);
        let mut c_zero = Matrix::zeros(n + 1, n + 1);

        for row in 0..n {
            for col in 0..n {
                g_zero.set(row, col, -model.a.get(row, col));
            }
            g_zero.set(row, n, -model.b[row]);
            c_zero.set(row, row, 1.0);
        }
        for col in 0..n {
            g_zero.set(n, col, model.c[col]);
        }
        g_zero.set(n, n, model.d);

        let zeros = self
            .generalized_eigenvalues(&g_zero, &c_zero)
            .unwrap_or_default();
        self.finalize_zero_roots(zeros, poles, config)
    }

    fn build_voltage_input_transfer_system(
        &self,
        config: &PoleZeroConfig,
        output_vec: &[Value],
    ) -> Option<(PoleZeroAnalyzer, Vec<Value>, Vec<Value>)> {
        if let Some(input_voltage_branch) = config.input_voltage_branch {
            if input_voltage_branch >= self.num_nodes || output_vec.len() != self.num_nodes {
                return None;
            }

            let mut drive_vec = vec![0.0; self.num_nodes];
            drive_vec[input_voltage_branch] = config.input_voltage_gain;
            return Some((
                PoleZeroAnalyzer::new(self.g_matrix.clone(), self.c_matrix.clone()),
                drive_vec,
                output_vec.to_vec(),
            ));
        }

        let n = self.num_nodes;
        if output_vec.len() != n || config.input_pos >= n {
            return None;
        }

        let mut g_ext = Matrix::zeros(n + 1, n + 1);
        let mut c_ext = Matrix::zeros(n + 1, n + 1);

        for i in 0..n {
            for j in 0..n {
                g_ext.set(i, j, self.g_matrix.get(i, j));
                c_ext.set(i, j, self.c_matrix.get(i, j));
            }
        }

        let branch = n;
        g_ext.add(config.input_pos, branch, 1.0);
        g_ext.add(branch, config.input_pos, 1.0);
        if let Some(input_neg) = config.input_neg {
            if input_neg >= n {
                return None;
            }
            g_ext.add(input_neg, branch, -1.0);
            g_ext.add(branch, input_neg, -1.0);
        }

        let mut drive_vec = vec![0.0; n + 1];
        drive_vec[branch] = 1.0;

        let mut output_ext = vec![0.0; n + 1];
        output_ext[..n].copy_from_slice(output_vec);

        Some((PoleZeroAnalyzer::new(g_ext, c_ext), drive_vec, output_ext))
    }

    fn finalize_zero_roots(
        &self,
        mut zeros: Vec<Complex>,
        poles: &[Complex],
        config: &PoleZeroConfig,
    ) -> Vec<Complex> {
        let finite_zero_limit = self.finite_zero_limit(poles, config);
        zeros.retain(|z| z.magnitude() <= finite_zero_limit);
        zeros.retain(|z| !poles.iter().any(|p| Self::is_same_root(z, p, 1e-4)));
        self.sort_roots(&mut zeros);
        self.canonicalize_near_real_zero_pairs(&mut zeros);
        self.sort_roots(&mut zeros);
        zeros
    }

    fn numerator_root_2x2(&self, input_vec: &[Value], output_vec: &[Value]) -> Option<Complex> {
        if input_vec.len() != 2 || output_vec.len() != 2 {
            return None;
        }

        let b1 = input_vec[0];
        let b2 = input_vec[1];
        let l1 = output_vec[0];
        let l2 = output_vec[1];

        let g11 = self.g_matrix.get(0, 0);
        let g12 = self.g_matrix.get(0, 1);
        let g21 = self.g_matrix.get(1, 0);
        let g22 = self.g_matrix.get(1, 1);
        let c11 = self.c_matrix.get(0, 0);
        let c12 = self.c_matrix.get(0, 1);
        let c21 = self.c_matrix.get(1, 0);
        let c22 = self.c_matrix.get(1, 1);

        // N(s) = L^T * adj(G + sC) * B = a + b*s for 2x2 systems.
        let a = l1 * (g22 * b1 - g12 * b2) + l2 * (-g21 * b1 + g11 * b2);
        let b = l1 * (c22 * b1 - c12 * b2) + l2 * (-c21 * b1 + c11 * b2);
        if b.abs() < 1e-15 {
            return None;
        }

        let root = -a / b;
        if root.is_finite() {
            Some(Complex::real(root))
        } else {
            None
        }
    }

    fn finite_zero_limit(&self, poles: &[Complex], config: &PoleZeroConfig) -> Value {
        let pole_scale = poles
            .iter()
            .map(|p| p.magnitude())
            .fold(1.0_f64, |acc, mag| acc.max(mag));
        (pole_scale * 1e6).min(config.max_pole_freq * 2.0 * PI)
    }

    /// Find zeros.
    ///
    /// Uses the Rosenbrock system matrix for SISO transfer numerator extraction:
    ///
    /// det([G + s*C, -B; L^T, 0]) = 0
    ///
    /// where B is the input excitation vector and L selects a measured voltage
    /// (including differential references).
    pub fn find_zeros(&self, config: &PoleZeroConfig) -> Vec<Complex> {
        if self.num_nodes == 0 {
            return Vec::new();
        }
        if self.is_direct_voltage_port_measurement(config) {
            return Vec::new();
        }

        let (input_vec, output_vec) = match self.build_port_vectors(config) {
            Some(v) => v,
            None => return Vec::new(),
        };

        if config.input_is_current {
            if let Some(state_space) = self.build_state_space(&input_vec, &output_vec) {
                let poles = self
                    .eigenvalues_from_matrix(&state_space.a)
                    .unwrap_or_else(|| self.find_poles(config));
                return self.zeros_from_state_space(&state_space, &poles, config);
            }

            let poles = self.find_poles(config);
            let zeros = self.numerator_roots_raw(&input_vec, &output_vec, config);
            return self.finalize_zero_roots(zeros, &poles, config);
        }

        let Some((voltage_analyzer, drive_vec, output_ext)) =
            self.build_voltage_input_transfer_system(config, &output_vec)
        else {
            return Vec::new();
        };

        if let Some(state_space) = voltage_analyzer.build_state_space(&drive_vec, &output_ext) {
            let poles = voltage_analyzer
                .eigenvalues_from_matrix(&state_space.a)
                .unwrap_or_else(|| voltage_analyzer.find_poles(config));
            return voltage_analyzer.zeros_from_state_space(&state_space, &poles, config);
        }

        let poles = self.find_poles(config);
        let zeros = voltage_analyzer.numerator_roots_raw(&drive_vec, &output_ext, config);
        self.finalize_zero_roots(zeros, &poles, config)
    }

    /// Compute DC gain H(0)
    pub fn dc_gain(&self, input_node: usize, output_node: usize) -> Option<Value> {
        // At DC (s=0), Y = G
        // Solve GÂ·V = I where I is unit current at input
        let n = self.num_nodes;
        if input_node >= n || output_node >= n {
            return None;
        }

        // Create excitation vector
        let mut b = vec![0.0; n];
        b[input_node] = 1.0;

        // Solve GÂ·x = b using Gaussian elimination
        let x = self.solve_linear(&self.g_matrix, &b)?;

        Some(x[output_node])
    }

    fn dc_gain_from_config(&self, config: &PoleZeroConfig) -> Option<Value> {
        let (input_vec, output_vec) = self.build_port_vectors(config)?;
        let x = self.solve_linear(&self.g_matrix, &input_vec)?;
        let vout = output_vec
            .iter()
            .zip(x.iter())
            .map(|(l, v)| l * v)
            .sum::<Value>();

        if config.input_is_current {
            return Some(vout);
        }

        let vin = input_vec
            .iter()
            .zip(x.iter())
            .map(|(m, v)| m * v)
            .sum::<Value>();
        if vin.abs() < 1e-15 {
            return None;
        }

        Some(vout / vin)
    }

    /// Solve linear system using Gaussian elimination
    fn solve_linear(&self, a: &Matrix, b: &[Value]) -> Option<Vec<Value>> {
        let n = a.dims().0;
        let pivot_tolerance = self.relative_matrix_tolerance(a, 1e-12);

        // Augmented matrix
        let mut aug: Vec<Vec<Value>> = (0..n)
            .map(|i| {
                let mut row = a.data[i].clone();
                row.push(b[i]);
                row
            })
            .collect();

        // Forward elimination
        for k in 0..n {
            // Partial pivoting
            let mut max_row = k;
            let mut max_val = aug[k][k].abs();
            for i in (k + 1)..n {
                if aug[i][k].abs() > max_val {
                    max_val = aug[i][k].abs();
                    max_row = i;
                }
            }

            if max_val <= pivot_tolerance {
                return None;
            }

            if max_row != k {
                aug.swap(k, max_row);
            }

            let pivot = aug[k][k];
            for i in (k + 1)..n {
                let factor = aug[i][k] / pivot;
                aug[i][k] = 0.0;
                for j in (k + 1)..=n {
                    aug[i][j] -= factor * aug[k][j];
                }
            }
        }

        // Back substitution
        let mut x = vec![0.0; n];
        for i in (0..n).rev() {
            let mut sum = aug[i][n];
            for j in (i + 1)..n {
                sum -= aug[i][j] * x[j];
            }
            x[i] = sum / aug[i][i];
        }

        Some(x)
    }

    /// Run complete pole-zero analysis
    pub fn analyze(&self, config: &PoleZeroConfig) -> PoleZeroResult {
        let mut result = PoleZeroResult::new(
            &format!("node{}", config.input_pos),
            &format!("node{}", config.output_pos),
        );

        if !config.input_is_current
            && let Some((_, output_vec)) = self.build_port_vectors(config)
            && let Some((voltage_analyzer, drive_vec, output_ext)) =
                self.build_voltage_input_transfer_system(config, &output_vec)
        {
            if config.compute_poles {
                result.poles = voltage_analyzer.find_poles(config);
            }

            if config.compute_zeros {
                if self.is_direct_voltage_port_measurement(config) {
                    result.zeros.clear();
                } else if let Some(state_space) =
                    voltage_analyzer.build_state_space(&drive_vec, &output_ext)
                {
                    let poles = if config.compute_poles {
                        result.poles.clone()
                    } else {
                        voltage_analyzer
                            .eigenvalues_from_matrix(&state_space.a)
                            .unwrap_or_else(|| voltage_analyzer.find_poles(config))
                    };
                    result.zeros =
                        voltage_analyzer.zeros_from_state_space(&state_space, &poles, config);
                } else {
                    let poles = if config.compute_poles {
                        result.poles.clone()
                    } else {
                        voltage_analyzer.find_poles(config)
                    };
                    let zeros =
                        voltage_analyzer.numerator_roots_raw(&drive_vec, &output_ext, config);
                    result.zeros = voltage_analyzer.finalize_zero_roots(zeros, &poles, config);
                }
            }

            if let Some(gain) = self.dc_gain_from_config(config) {
                result.dc_gain = gain;
            }

            result.sort_poles_by_magnitude();
            result.sort_zeros_by_magnitude();
            return result;
        }

        // Find poles
        if config.compute_poles {
            result.poles = self.find_poles(config);
        }

        // Find zeros
        if config.compute_zeros {
            result.zeros = self.find_zeros(config);
        }

        // Compute DC gain
        if let Some(gain) = self.dc_gain_from_config(config) {
            result.dc_gain = gain;
        }

        result.sort_poles_by_magnitude();
        result.sort_zeros_by_magnitude();

        result
    }
}

//=============================================================================
// Tests
//=============================================================================

