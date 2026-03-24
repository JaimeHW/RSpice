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
    /// Whether to compute poles
    pub compute_poles: bool,
    /// Whether to compute zeros
    pub compute_zeros: bool,
    /// Maximum pole magnitude to include (filter spurious)
    pub max_pole_freq: Value,
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

        if let Some(state_matrix) = self.build_descriptor_state_matrix() {
            let mut poles = self.qr_eigenvalues(&state_matrix);
            poles.retain(|p| {
                p.re.is_finite()
                    && p.im.is_finite()
                    && p.magnitude() < config.max_pole_freq * 2.0 * PI
            });
            poles.sort_by(|a, b| a.re.total_cmp(&b.re).then_with(|| a.im.total_cmp(&b.im)));
            poles.dedup_by(|a, b| (a.re - b.re).abs() < 1e-9 && (a.im - b.im).abs() < 1e-9);
            return poles;
        }

        // Fallback for heavily singular descriptors where state extraction fails.
        self.eigenvalues_diagonal_fallback(config)
    }

    /// Descriptor-system reduction:
    /// Cx' + Gx = 0
    /// Split x into dynamic/algebraic variables and eliminate algebraic states.
    /// This yields x_d' = A x_d where A is used for pole extraction.
    fn build_descriptor_state_matrix(&self) -> Option<Matrix> {
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

        let c_dd = self.extract_submatrix(&self.c_matrix, &dynamic, &dynamic);
        let g_dd = self.extract_submatrix(&self.g_matrix, &dynamic, &dynamic);

        let g_eff = if algebraic.is_empty() {
            g_dd
        } else {
            let g_da = self.extract_submatrix(&self.g_matrix, &dynamic, &algebraic);
            let g_ad = self.extract_submatrix(&self.g_matrix, &algebraic, &dynamic);
            let g_aa = self.extract_submatrix(&self.g_matrix, &algebraic, &algebraic);

            let g_aa_inv_g_ad = self.solve_matrix_columns_regularized(&g_aa, &g_ad)?;
            let correction = self.matrix_multiply(&g_da, &g_aa_inv_g_ad);
            self.matrix_subtract(&g_dd, &correction)
        };

        let c_dd_inv_g_eff = self.solve_matrix_columns_regularized(&c_dd, &g_eff)?;
        let mut a = c_dd_inv_g_eff;
        for row in &mut a.data {
            for value in row {
                *value = -*value;
            }
        }

        Some(a)
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

        // Try exact solve first, then progressively stronger diagonal regularization.
        let regularizations = [0.0, 1e-18, 1e-15, 1e-12];
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
        for col in 0..b.cols {
            let rhs: Vec<Value> = (0..b.rows).map(|r| b.data[r][col]).collect();
            let x = self.solve_linear(a, &rhs)?;
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
        if n == 1 {
            return vec![Complex::real(matrix.data[0][0])];
        }
        if n == 2 {
            return self.eigenvalues_2x2(
                matrix.data[0][0],
                matrix.data[0][1],
                matrix.data[1][0],
                matrix.data[1][1],
            );
        }

        let tol = 1e-10;
        let max_iter = 2000;
        let mut a = matrix.data.clone();

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

    fn is_same_root(a: &Complex, b: &Complex, tol: Value) -> bool {
        let re_scale = 1.0 + a.re.abs().max(b.re.abs());
        let im_scale = 1.0 + a.im.abs().max(b.im.abs());
        (a.re - b.re).abs() <= tol * re_scale && (a.im - b.im).abs() <= tol * im_scale
    }

    fn dedup_and_sort_roots(&self, roots: &mut Vec<Complex>) {
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
        roots.dedup_by(|a, b| Self::is_same_root(a, b, 1e-8));
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

        let zero_analyzer = PoleZeroAnalyzer::new(g_aug, c_aug);
        let mut zero_config = PoleZeroConfig::poles_only(0, 0);
        zero_config.max_pole_freq = config.max_pole_freq;
        let mut roots = zero_analyzer.find_poles(&zero_config);
        roots.retain(|r| r.re.is_finite() && r.im.is_finite());
        roots
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

        let (input_vec, output_vec) = match self.build_port_vectors(config) {
            Some(v) => v,
            None => return Vec::new(),
        };

        let poles = self.find_poles(config);
        let finite_zero_limit = self.finite_zero_limit(&poles, config);

        let mut zeros = self.numerator_roots_raw(&input_vec, &output_vec, config);
        zeros.retain(|z| z.magnitude() <= finite_zero_limit);

        if config.input_is_current {
            // Current input transfer: H = L*Y^{-1}B = N(s)/D(s)
            // Remove N/D cancellations from uncontrollable/unobservable modes.
            zeros.retain(|z| !poles.iter().any(|p| Self::is_same_root(z, p, 1e-4)));
        } else {
            // Voltage input transfer: H = Vout/Vin = Nout(s)/Nin(s)
            // Remove numerator/denominator cancellations from Vin polynomial roots.
            let mut vin_roots = self.numerator_roots_raw(&input_vec, &input_vec, config);
            vin_roots.retain(|z| z.magnitude() <= finite_zero_limit);
            zeros.retain(|z| !vin_roots.iter().any(|r| Self::is_same_root(z, r, 1e-4)));
        }

        self.dedup_and_sort_roots(&mut zeros);
        zeros
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

            if max_val < 1e-15 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_basics() {
        let p = Complex::new(-1000.0, 2000.0);

        assert!(!p.is_real(1e-10));
        assert!((p.magnitude() - 2236.07).abs() < 0.1);

        let real_p = Complex::real(-1000.0);
        assert!(real_p.is_real(1e-10));

        // Time constant for real pole at -1000 rad/s
        let tau = real_p.time_constant().unwrap();
        assert!((tau - 0.001).abs() < 1e-6);
    }

    #[test]
    fn test_conjugate_detection() {
        let p1 = Complex::new(-100.0, 500.0);
        let p2 = Complex::new(-100.0, -500.0);
        let p3 = Complex::new(-100.0, 501.0);

        assert!(p1.is_conjugate_of(&p2, 1e-10));
        assert!(!p1.is_conjugate_of(&p3, 1e-10));
    }

    #[test]
    fn test_simple_rc_pole() {
        // RC lowpass: pole at s = -1/RC
        // R = 1k, C = 1ÂµF â†’ pole at s = -1000 rad/s
        let r = 1000.0;
        let c = 1e-6;
        let g = 1.0 / r;

        // Single-node circuit: G and C are 1x1
        let mut g_matrix = Matrix::zeros(1, 1);
        g_matrix.set(0, 0, g);

        let mut c_matrix = Matrix::zeros(1, 1);
        c_matrix.set(0, 0, c);

        let analyzer = PoleZeroAnalyzer::new(g_matrix, c_matrix);
        let config = PoleZeroConfig::poles_only(0, 0);
        let result = analyzer.analyze(&config);

        assert_eq!(result.poles.len(), 1);

        // Pole should be at -1/(RC) = -1000 rad/s
        let expected_pole = -1.0 / (r * c);
        assert!(
            (result.poles[0].re - expected_pole).abs() < 1.0,
            "Expected pole at {}, got {}",
            expected_pole,
            result.poles[0].re
        );
        assert!(result.poles[0].is_real(1e-10));
    }

    #[test]
    fn test_stability_check() {
        let mut result = PoleZeroResult::new("in", "out");

        // All stable poles
        result.add_pole(Complex::new(-100.0, 0.0));
        result.add_pole(Complex::new(-50.0, 100.0));
        result.add_pole(Complex::new(-50.0, -100.0));

        assert!(result.is_stable());

        // Add unstable pole
        result.add_pole(Complex::new(10.0, 0.0));
        assert!(!result.is_stable());
    }

    #[test]
    fn test_dominant_pole() {
        let mut result = PoleZeroResult::new("in", "out");

        result.add_pole(Complex::new(-1000.0, 0.0)); // Fast
        result.add_pole(Complex::new(-10.0, 0.0)); // Dominant (slowest)
        result.add_pole(Complex::new(-500.0, 0.0)); // Medium

        let dominant = result.dominant_pole().unwrap();
        assert!((dominant.re - (-10.0)).abs() < 1e-10);
    }

    #[test]
    fn test_dominant_pole_ignores_non_finite_real_parts() {
        let mut result = PoleZeroResult::new("in", "out");
        result.add_pole(Complex::new(f64::NAN, 0.0));
        result.add_pole(Complex::new(-500.0, 0.0));
        result.add_pole(Complex::new(-20.0, 0.0));

        let dominant = result.dominant_pole().expect("finite dominant pole");
        assert!((dominant.re - (-20.0)).abs() < 1e-12);
    }

    #[test]
    fn test_sort_by_magnitude_demotes_non_finite_entries() {
        let mut result = PoleZeroResult::new("in", "out");
        result.add_pole(Complex::new(f64::NAN, 0.0));
        result.add_pole(Complex::new(-100.0, 0.0));
        result.add_pole(Complex::new(-10.0, 0.0));
        result.add_zero(Complex::new(f64::INFINITY, 0.0));
        result.add_zero(Complex::new(-5.0, 0.0));
        result.add_zero(Complex::new(-50.0, 0.0));

        result.sort_poles_by_magnitude();
        result.sort_zeros_by_magnitude();

        assert!((result.poles[0].re - (-10.0)).abs() < 1e-12);
        assert!((result.poles[1].re - (-100.0)).abs() < 1e-12);
        assert!(!result.poles[2].re.is_finite());

        assert!((result.zeros[0].re - (-5.0)).abs() < 1e-12);
        assert!((result.zeros[1].re - (-50.0)).abs() < 1e-12);
        assert!(!result.zeros[2].re.is_finite());
    }

    #[test]
    fn test_complex_pole_pairs() {
        let mut result = PoleZeroResult::new("in", "out");

        result.add_pole(Complex::new(-100.0, 0.0)); // Real
        result.add_pole(Complex::new(-50.0, 200.0)); // Complex
        result.add_pole(Complex::new(-50.0, -200.0)); // Conjugate
        result.add_pole(Complex::new(-30.0, 100.0)); // Another pair
        result.add_pole(Complex::new(-30.0, -100.0));

        let pairs = result.complex_pole_pairs();
        assert_eq!(pairs.len(), 2);
    }

    #[test]
    fn test_dc_gain() {
        // Voltage divider: R1=1k between input and output, R2=1k to ground
        // DC gain = R2/(R1+R2) = 0.5
        let g1 = 1.0 / 1000.0;
        let g2 = 1.0 / 1000.0;

        let mut g_matrix = Matrix::zeros(2, 2);
        g_matrix.set(0, 0, g1);
        g_matrix.set(0, 1, -g1);
        g_matrix.set(1, 0, -g1);
        g_matrix.set(1, 1, g1 + g2);

        let c_matrix = Matrix::zeros(2, 2);

        let analyzer = PoleZeroAnalyzer::new(g_matrix, c_matrix);

        // Gain from node 0 to node 1
        let gain = analyzer.dc_gain(0, 1).unwrap();

        // With 1A injected at node 0:
        // V0 = I * (R1 || (R2 + ...)) - more complex
        // Simplified: if we inject 1A at node 0, V1 depends on the network
        assert!(gain > 0.0);
    }

    #[test]
    fn test_matrix_operations() {
        let mut m = Matrix::zeros(2, 2);
        m.set(0, 0, 1.0);
        m.set(1, 1, 2.0);

        let v = vec![3.0, 4.0];
        let result = m.mul_vec(&v);

        assert!((result[0] - 3.0).abs() < 1e-10);
        assert!((result[1] - 8.0).abs() < 1e-10);
    }

    #[test]
    fn test_descriptor_pole_with_branch_state_rl() {
        // Parallel RL at one node:
        // KCL: g*v + iL = 0
        // Inductor: v - sL*iL = 0
        // Pole: s = -R/L
        let r = 1e3;
        let l = 1e-3;
        let g = 1.0 / r;

        let mut g_matrix = Matrix::zeros(2, 2);
        g_matrix.set(0, 0, g);
        g_matrix.set(0, 1, 1.0);
        g_matrix.set(1, 0, 1.0);
        g_matrix.set(1, 1, 0.0);

        let mut c_matrix = Matrix::zeros(2, 2);
        c_matrix.set(1, 1, -l);

        let analyzer = PoleZeroAnalyzer::new(g_matrix, c_matrix);
        let config = PoleZeroConfig::poles_only(0, 0);
        let result = analyzer.analyze(&config);

        let expected = -r / l;
        let closest = result
            .poles
            .iter()
            .min_by(|a, b| {
                (a.re - expected)
                    .abs()
                    .partial_cmp(&(b.re - expected).abs())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .expect("expected at least one pole");
        assert!((closest.re - expected).abs() < 1e3);
        assert!(closest.im.abs() < 1e-6);
    }

    #[test]
    fn test_qr_eigenvalues_with_complex_pair() {
        // A = -G with C = I
        // A = [ -2  10 ]
        //     [ -10 -2 ]
        // poles = -2 Â± j10
        let mut g_matrix = Matrix::zeros(2, 2);
        g_matrix.set(0, 0, 2.0);
        g_matrix.set(0, 1, -10.0);
        g_matrix.set(1, 0, 10.0);
        g_matrix.set(1, 1, 2.0);

        let c_matrix = Matrix::identity(2);

        let analyzer = PoleZeroAnalyzer::new(g_matrix, c_matrix);
        let config = PoleZeroConfig::poles_only(0, 0);
        let result = analyzer.analyze(&config);

        assert_eq!(result.poles.len(), 2);
        assert!(
            result
                .poles
                .iter()
                .all(|p| (p.re + 2.0).abs() < 1e-6 && (p.im.abs() - 10.0).abs() < 1e-5)
        );
    }

    #[test]
    fn test_two_by_two_zero_extraction() {
        // For output=input=0 in 2x2, numerator is cofactor(0,0)=a22+b22*s.
        // Choose a22=2, b22=4 => zero at s=-0.5.
        let mut g_matrix = Matrix::zeros(2, 2);
        g_matrix.set(0, 0, 1.0);
        g_matrix.set(0, 1, -1.0);
        g_matrix.set(1, 0, -1.0);
        g_matrix.set(1, 1, 2.0);

        let mut c_matrix = Matrix::zeros(2, 2);
        c_matrix.set(1, 1, 4.0);

        let analyzer = PoleZeroAnalyzer::new(g_matrix, c_matrix);
        let config = PoleZeroConfig::zeros_only(0, 0);
        let result = analyzer.analyze(&config);

        assert_eq!(result.zeros.len(), 1);
        assert!((result.zeros[0].re + 0.5).abs() < 1e-10);
        assert!(result.zeros[0].im.abs() < 1e-12);
    }

    #[test]
    fn test_general_zero_extraction_filters_cancelled_modes() {
        // Diagonal Y(s) with one decoupled dynamic mode:
        // y1=s+1, y2=s+2, y3=s+3
        // Differential in/out on nodes 0 and 1 gives:
        // H(s) = 1/(s+1) + 1/(s+2) = (2s+3)/((s+1)(s+2))
        // True transfer zero at s=-1.5.
        let mut g_matrix = Matrix::zeros(3, 3);
        g_matrix.set(0, 0, 1.0);
        g_matrix.set(1, 1, 2.0);
        g_matrix.set(2, 2, 3.0);

        let c_matrix = Matrix::identity(3);

        let analyzer = PoleZeroAnalyzer::new(g_matrix, c_matrix);
        let mut config = PoleZeroConfig::zeros_only(0, 0);
        config.input_neg = Some(1);
        config.output_neg = Some(1);

        let result = analyzer.analyze(&config);
        assert!(
            result
                .zeros
                .iter()
                .any(|z| { (z.re + 1.5).abs() < 0.1 && z.im.abs() < 1e-6 }),
            "expected zero near -1.5, got {:?}",
            result.zeros
        );
        assert!(
            !result
                .zeros
                .iter()
                .any(|z| (z.re + 3.0).abs() < 1e-3 && z.im.abs() < 1e-3)
        );
    }

    #[test]
    fn test_dc_gain_differential_current_port() {
        let mut g_matrix = Matrix::zeros(3, 3);
        g_matrix.set(0, 0, 1.0);
        g_matrix.set(1, 1, 2.0);
        g_matrix.set(2, 2, 3.0);
        let c_matrix = Matrix::identity(3);

        let analyzer = PoleZeroAnalyzer::new(g_matrix, c_matrix);
        let mut config = PoleZeroConfig::poles_only(0, 0);
        config.input_neg = Some(1);
        config.output_neg = Some(1);
        config.input_is_current = true;

        let result = analyzer.analyze(&config);
        assert!((result.dc_gain - 1.5).abs() < 1e-12);
    }

    #[test]
    fn test_dc_gain_voltage_mode_normalizes_by_input_voltage() {
        // Resistive divider: R1 between input and output, R2 from output to ground.
        // Vout/Vin = R2/(R1+R2) = 0.5 for R1=R2.
        let g1 = 1.0 / 1e3;
        let g2 = 1.0 / 1e3;
        let mut g_matrix = Matrix::zeros(2, 2);
        g_matrix.set(0, 0, g1);
        g_matrix.set(0, 1, -g1);
        g_matrix.set(1, 0, -g1);
        g_matrix.set(1, 1, g1 + g2);
        let c_matrix = Matrix::zeros(2, 2);

        let analyzer = PoleZeroAnalyzer::new(g_matrix, c_matrix);
        let mut config = PoleZeroConfig::poles_only(0, 1);
        config.input_is_current = false;

        let result = analyzer.analyze(&config);
        assert!((result.dc_gain - 0.5).abs() < 1e-12);
    }

    #[test]
    fn test_voltage_mode_highpass_zero_at_origin() {
        // High-pass network:
        // C between input/output, R from output to ground.
        // H(s)=Vout/Vin = sRC/(1+sRC), so zero at s=0.
        let r = 1e3;
        let c = 1e-9;
        let g = 1.0 / r;

        let mut g_matrix = Matrix::zeros(2, 2);
        g_matrix.set(1, 1, g);

        let mut c_matrix = Matrix::zeros(2, 2);
        c_matrix.set(0, 0, c);
        c_matrix.set(0, 1, -c);
        c_matrix.set(1, 0, -c);
        c_matrix.set(1, 1, c);

        let analyzer = PoleZeroAnalyzer::new(g_matrix, c_matrix);
        let mut config = PoleZeroConfig::zeros_only(0, 1);
        config.input_is_current = false;

        let result = analyzer.analyze(&config);
        assert!(
            result.zeros.iter().any(|z| z.magnitude() < 1e-8),
            "expected zero near origin, got {:?}",
            result.zeros
        );
    }

    #[test]
    fn test_voltage_mode_unity_transfer_has_no_zeros() {
        // If output port equals input port, H(s)=Vin/Vin=1 and there are no finite zeros.
        let mut g_matrix = Matrix::zeros(3, 3);
        g_matrix.set(0, 0, 1.0);
        g_matrix.set(1, 1, 2.0);
        g_matrix.set(2, 2, 3.0);
        let c_matrix = Matrix::identity(3);

        let analyzer = PoleZeroAnalyzer::new(g_matrix, c_matrix);
        let mut config = PoleZeroConfig::zeros_only(0, 0);
        config.input_is_current = false;
        config.input_neg = Some(1);
        config.output_neg = Some(1);

        let result = analyzer.analyze(&config);
        assert!(
            result.zeros.is_empty(),
            "expected no zeros for unity transfer, got {:?}",
            result.zeros
        );
    }
}
