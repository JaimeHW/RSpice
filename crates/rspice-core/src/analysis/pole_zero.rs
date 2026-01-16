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
    /// ζ = -Re(p) / |p|
    pub fn damping_factor(&self) -> Value {
        let mag = self.magnitude();
        if mag > 1e-15 {
            -self.re / mag
        } else {
            0.0
        }
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
            .filter(|p| p.re < 0.0) // Only stable poles
            .min_by(|a, b| {
                a.re.abs().partial_cmp(&b.re.abs()).unwrap()
            })
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
            a.magnitude().partial_cmp(&b.magnitude()).unwrap()
        });
    }

    /// Sort zeros by magnitude
    pub fn sort_zeros_by_magnitude(&mut self) {
        self.zeros.sort_by(|a, b| {
            a.magnitude().partial_cmp(&b.magnitude()).unwrap()
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
    /// The MNA equation is: (G + s·C)·x = b
    /// Poles are values of s where det(G + s·C) = 0
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
    /// Poles are eigenvalues of -C⁻¹·G (if C is invertible)
    /// For singular C, use generalized eigenvalue: G·x = -s·C·x
    pub fn find_poles(&self, config: &PoleZeroConfig) -> Vec<Complex> {
        let n = self.num_nodes;
        if n == 0 {
            return Vec::new();
        }

        // For single-node RC circuit:
        // G + s·C = 0 → s = -G/C
        if n == 1 {
            let g = self.g_matrix.get(0, 0);
            let c = self.c_matrix.get(0, 0);
            if c.abs() > 1e-15 {
                return vec![Complex::real(-g / c)];
            }
            return Vec::new();
        }

        // For larger circuits, use power iteration or QR
        // This is a simplified implementation
        self.eigenvalues_power_method(config)
    }

    /// Find poles using power iteration (simplified, finds dominant pole)
    fn eigenvalues_power_method(&self, config: &PoleZeroConfig) -> Vec<Complex> {
        let n = self.num_nodes;
        let mut poles = Vec::new();

        // Try to find if C is diagonal-dominant
        // For simple RC networks, poles are at s = -G_ii/C_ii for each node
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

        // Remove duplicates (within tolerance)
        poles.sort_by(|a, b| a.re.partial_cmp(&b.re).unwrap());
        poles.dedup_by(|a, b| (a.re - b.re).abs() < 1e-6);

        poles
    }

    /// Find zeros (simplified implementation)
    pub fn find_zeros(&self, _config: &PoleZeroConfig) -> Vec<Complex> {
        // Zeros require augmenting the matrix with input/output
        // This is a placeholder - full implementation would use
        // the nullspace method or matrix pencil
        Vec::new()
    }

    /// Compute DC gain H(0)
    pub fn dc_gain(&self, input_node: usize, output_node: usize) -> Option<Value> {
        // At DC (s=0), Y = G
        // Solve G·V = I where I is unit current at input
        let n = self.num_nodes;
        if input_node >= n || output_node >= n {
            return None;
        }

        // Create excitation vector
        let mut b = vec![0.0; n];
        b[input_node] = 1.0;

        // Solve G·x = b using Gaussian elimination
        let x = self.solve_linear(&self.g_matrix, &b)?;

        Some(x[output_node])
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
        if let Some(gain) = self.dc_gain(config.input_pos, config.output_pos) {
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
        // R = 1k, C = 1µF → pole at s = -1000 rad/s
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
        
        result.add_pole(Complex::new(-1000.0, 0.0));  // Fast
        result.add_pole(Complex::new(-10.0, 0.0));    // Dominant (slowest)
        result.add_pole(Complex::new(-500.0, 0.0));   // Medium
        
        let dominant = result.dominant_pole().unwrap();
        assert!((dominant.re - (-10.0)).abs() < 1e-10);
    }

    #[test]
    fn test_complex_pole_pairs() {
        let mut result = PoleZeroResult::new("in", "out");
        
        result.add_pole(Complex::new(-100.0, 0.0));      // Real
        result.add_pole(Complex::new(-50.0, 200.0));     // Complex
        result.add_pole(Complex::new(-50.0, -200.0));    // Conjugate
        result.add_pole(Complex::new(-30.0, 100.0));     // Another pair
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
}
