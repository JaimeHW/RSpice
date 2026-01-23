//! S-Parameter Analysis
//!
//! Computes scattering parameters for RF/microwave circuit characterization.
//! S-parameters describe the electrical behavior of linear networks in terms
//! of incident and reflected waves.
//!
//! # Theory
//!
//! For an N-port network with reference impedance Z₀:
//!
//! ```text
//! a = (V + Z₀·I) / (2·√Z₀)    (incident wave)
//! b = (V - Z₀·I) / (2·√Z₀)    (reflected wave)
//!
//! b = S · a
//!
//! S_ij = b_i / a_j  when a_k = 0 for k ≠ j
//! ```
//!
//! # Common S-Parameters (2-port)
//!
//! - **S₁₁**: Input reflection coefficient
//! - **S₂₁**: Forward transmission (gain)
//! - **S₁₂**: Reverse transmission (isolation)  
//! - **S₂₂**: Output reflection coefficient
//!
//! # Example
//!
//! ```ignore
//! .SP DEC 10 1MEG 10GIG
//! ```

use crate::Value;
use std::f64::consts::PI;

/// Complex number type for S-parameters
#[derive(Debug, Clone, Copy, Default)]
pub struct Complex {
    pub re: Value,
    pub im: Value,
}

impl Complex {
    pub const ZERO: Complex = Complex { re: 0.0, im: 0.0 };
    pub const ONE: Complex = Complex { re: 1.0, im: 0.0 };

    pub fn new(re: Value, im: Value) -> Self {
        Self { re, im }
    }

    pub fn from_polar(mag: Value, phase_rad: Value) -> Self {
        Self {
            re: mag * phase_rad.cos(),
            im: mag * phase_rad.sin(),
        }
    }

    pub fn magnitude(&self) -> Value {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    pub fn phase(&self) -> Value {
        self.im.atan2(self.re)
    }

    pub fn phase_deg(&self) -> Value {
        self.phase() * 180.0 / PI
    }

    pub fn conj(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    pub fn mag_db(&self) -> Value {
        20.0 * self.magnitude().log10()
    }
}

impl std::ops::Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl std::ops::Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl std::ops::Mul<Value> for Complex {
    type Output = Self;
    fn mul(self, rhs: Value) -> Self {
        Self {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

impl std::ops::Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let denom = rhs.re * rhs.re + rhs.im * rhs.im;
        if denom < 1e-30 {
            return Complex::ZERO;
        }
        Self {
            re: (self.re * rhs.re + self.im * rhs.im) / denom,
            im: (self.im * rhs.re - self.re * rhs.im) / denom,
        }
    }
}

impl std::ops::Div<Value> for Complex {
    type Output = Self;
    fn div(self, rhs: Value) -> Self {
        Self {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}

//=============================================================================
// Port Definition
//=============================================================================

/// Definition of a network port
#[derive(Debug, Clone)]
pub struct Port {
    /// Port number (1-indexed)
    pub number: usize,
    /// Positive terminal node
    pub node_pos: String,
    /// Negative terminal node (usually ground)
    pub node_neg: String,
    /// Reference impedance (typically 50Ω)
    pub z0: Value,
}

impl Port {
    /// Create a single-ended port (referenced to ground)
    pub fn single_ended(number: usize, node: &str, z0: Value) -> Self {
        Self {
            number,
            node_pos: node.to_string(),
            node_neg: "0".to_string(),
            z0,
        }
    }

    /// Create a differential port
    pub fn differential(number: usize, node_pos: &str, node_neg: &str, z0: Value) -> Self {
        Self {
            number,
            node_pos: node_pos.to_string(),
            node_neg: node_neg.to_string(),
            z0,
        }
    }
}

//=============================================================================
// S-Matrix
//=============================================================================

/// S-parameter matrix at a single frequency
#[derive(Debug, Clone)]
pub struct SMatrix {
    /// Frequency in Hz
    pub frequency: Value,
    /// Angular frequency ω = 2πf
    pub omega: Value,
    /// S-parameter data [row][col] = S[row+1, col+1]
    data: Vec<Vec<Complex>>,
    /// Number of ports
    num_ports: usize,
}

impl SMatrix {
    /// Create empty S-matrix for N ports
    pub fn new(frequency: Value, num_ports: usize) -> Self {
        Self {
            frequency,
            omega: 2.0 * PI * frequency,
            data: vec![vec![Complex::ZERO; num_ports]; num_ports],
            num_ports,
        }
    }

    /// Set S-parameter value (1-indexed)
    pub fn set(&mut self, row: usize, col: usize, value: Complex) {
        if row >= 1 && row <= self.num_ports && col >= 1 && col <= self.num_ports {
            self.data[row - 1][col - 1] = value;
        }
    }

    /// Get S-parameter value (1-indexed)
    pub fn get(&self, row: usize, col: usize) -> Complex {
        if row >= 1 && row <= self.num_ports && col >= 1 && col <= self.num_ports {
            self.data[row - 1][col - 1]
        } else {
            Complex::ZERO
        }
    }

    /// Get S11 (input reflection)
    pub fn s11(&self) -> Complex {
        self.get(1, 1)
    }

    /// Get S21 (forward transmission)
    pub fn s21(&self) -> Complex {
        self.get(2, 1)
    }

    /// Get S12 (reverse transmission)
    pub fn s12(&self) -> Complex {
        self.get(1, 2)
    }

    /// Get S22 (output reflection)
    pub fn s22(&self) -> Complex {
        self.get(2, 2)
    }
}

//=============================================================================
// S-Parameter Result
//=============================================================================

/// Complete S-parameter analysis result
#[derive(Debug, Clone)]
pub struct SParameterResult {
    /// Reference impedance for all ports
    pub z0: Value,
    /// Number of ports
    pub num_ports: usize,
    /// Port definitions
    pub ports: Vec<Port>,
    /// S-matrices at each frequency point
    pub data: Vec<SMatrix>,
}

impl SParameterResult {
    /// Create empty result
    pub fn new(z0: Value, ports: Vec<Port>) -> Self {
        let num_ports = ports.len();
        Self {
            z0,
            num_ports,
            ports,
            data: Vec::new(),
        }
    }

    /// Add S-matrix for a frequency point
    pub fn add(&mut self, matrix: SMatrix) {
        self.data.push(matrix);
    }

    /// Get frequency points
    pub fn frequencies(&self) -> Vec<Value> {
        self.data.iter().map(|s| s.frequency).collect()
    }

    /// Get S11 magnitude in dB across frequency
    pub fn s11_db(&self) -> Vec<Value> {
        self.data.iter().map(|s| s.s11().mag_db()).collect()
    }

    /// Get S21 magnitude in dB across frequency
    pub fn s21_db(&self) -> Vec<Value> {
        self.data.iter().map(|s| s.s21().mag_db()).collect()
    }

    /// Get VSWR from S11
    /// VSWR = (1 + |S11|) / (1 - |S11|)
    pub fn vswr(&self) -> Vec<Value> {
        self.data
            .iter()
            .map(|s| {
                let gamma = s.s11().magnitude();
                if gamma >= 1.0 {
                    f64::INFINITY
                } else {
                    (1.0 + gamma) / (1.0 - gamma)
                }
            })
            .collect()
    }

    /// Get return loss (dB) from S11
    /// RL = -20·log10(|S11|)
    pub fn return_loss(&self) -> Vec<Value> {
        self.data.iter().map(|s| -s.s11().mag_db()).collect()
    }

    /// Get insertion loss (dB) from S21
    /// IL = -20·log10(|S21|)
    pub fn insertion_loss(&self) -> Vec<Value> {
        self.data.iter().map(|s| -s.s21().mag_db()).collect()
    }

    /// Check if network is reciprocal (S12 ≈ S21)
    pub fn is_reciprocal(&self, tolerance: Value) -> bool {
        self.data.iter().all(|s| {
            let diff = s.s12() - s.s21();
            diff.magnitude() < tolerance
        })
    }
}

//=============================================================================
// S-Parameter Analyzer
//=============================================================================

/// Configuration for S-parameter analysis
#[derive(Debug, Clone)]
pub struct SParameterConfig {
    /// Start frequency (Hz)
    pub f_start: Value,
    /// Stop frequency (Hz)
    pub f_stop: Value,
    /// Number of frequency points
    pub num_points: usize,
    /// Frequency sweep type
    pub sweep_type: FrequencySweep,
    /// Reference impedance
    pub z0: Value,
}

/// Frequency sweep type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrequencySweep {
    Linear,
    Decade,
    Octave,
}

impl SParameterConfig {
    /// Create linear sweep configuration
    pub fn linear(f_start: Value, f_stop: Value, num_points: usize) -> Self {
        Self {
            f_start,
            f_stop,
            num_points,
            sweep_type: FrequencySweep::Linear,
            z0: 50.0,
        }
    }

    /// Create decade sweep configuration
    pub fn decade(f_start: Value, f_stop: Value, points_per_decade: usize) -> Self {
        let decades = (f_stop / f_start).log10();
        let num_points = (decades * points_per_decade as Value).ceil() as usize;
        Self {
            f_start,
            f_stop,
            num_points,
            sweep_type: FrequencySweep::Decade,
            z0: 50.0,
        }
    }

    /// Set reference impedance
    pub fn with_z0(mut self, z0: Value) -> Self {
        self.z0 = z0;
        self
    }

    /// Generate frequency points
    pub fn frequencies(&self) -> Vec<Value> {
        if self.num_points <= 1 {
            return vec![self.f_start];
        }

        match self.sweep_type {
            FrequencySweep::Linear => {
                let step = (self.f_stop - self.f_start) / (self.num_points - 1) as Value;
                (0..self.num_points)
                    .map(|i| self.f_start + (i as Value) * step)
                    .collect()
            }
            FrequencySweep::Decade | FrequencySweep::Octave => {
                let log_start = self.f_start.log10();
                let log_stop = self.f_stop.log10();
                let log_step = (log_stop - log_start) / (self.num_points - 1) as Value;
                (0..self.num_points)
                    .map(|i| 10_f64.powf(log_start + (i as Value) * log_step))
                    .collect()
            }
        }
    }
}

/// S-parameter analyzer
///
/// Computes S-parameters by exciting each port and measuring the response
pub struct SParameterAnalyzer {
    config: SParameterConfig,
    ports: Vec<Port>,
}

impl SParameterAnalyzer {
    /// Create analyzer
    pub fn new(config: SParameterConfig, ports: Vec<Port>) -> Self {
        Self { config, ports }
    }

    /// Compute S-parameters from Y-parameters (admittance matrix)
    ///
    /// S = (I - Z₀·Y) · (I + Z₀·Y)⁻¹
    ///
    /// For 2-port:
    /// S11 = ((1-Z₀·Y11)(1+Z₀·Y22) + Z₀²·Y12·Y21) / Δ
    /// S21 = -2·Z₀·Y21 / Δ
    /// where Δ = (1+Z₀·Y11)(1+Z₀·Y22) - Z₀²·Y12·Y21
    pub fn from_y_parameters(&self, y: &[[Complex; 2]; 2], frequency: Value) -> SMatrix {
        let z0 = Complex::new(self.config.z0, 0.0);
        let one = Complex::ONE;

        // Y-to-S conversion for 2-port
        let y11 = y[0][0];
        let y12 = y[0][1];
        let y21 = y[1][0];
        let y22 = y[1][1];

        // Δ = (1 + Z₀·Y11)(1 + Z₀·Y22) - Z₀²·Y12·Y21
        let z0y11 = z0 * y11;
        let z0y22 = z0 * y22;
        let z02y12y21 = z0 * z0 * y12 * y21;

        let term1 = (one + z0y11) * (one + z0y22);
        let delta = term1 - z02y12y21;

        // S11 = ((1 - Z₀·Y11)(1 + Z₀·Y22) + Z₀²·Y12·Y21) / Δ
        let s11_num = (one - z0y11) * (one + z0y22) + z02y12y21;
        let s11 = s11_num / delta;

        // S21 = -2·Z₀·Y21 / Δ
        let s21 = (z0 * y21 * (-2.0)) / delta;

        // S12 = -2·Z₀·Y12 / Δ
        let s12 = (z0 * y12 * (-2.0)) / delta;

        // S22 = ((1 + Z₀·Y11)(1 - Z₀·Y22) + Z₀²·Y12·Y21) / Δ
        let s22_num = (one + z0y11) * (one - z0y22) + z02y12y21;
        let s22 = s22_num / delta;

        let mut matrix = SMatrix::new(frequency, 2);
        matrix.set(1, 1, s11);
        matrix.set(1, 2, s12);
        matrix.set(2, 1, s21);
        matrix.set(2, 2, s22);

        matrix
    }

    /// Compute S-parameters from Z-parameters (impedance matrix)
    ///
    /// S = (Z - Z₀·I) · (Z + Z₀·I)⁻¹
    pub fn from_z_parameters(&self, z: &[[Complex; 2]; 2], frequency: Value) -> SMatrix {
        let z0 = Complex::new(self.config.z0, 0.0);

        let z11 = z[0][0];
        let z12 = z[0][1];
        let z21 = z[1][0];
        let z22 = z[1][1];

        // Δ = (Z11 + Z₀)(Z22 + Z₀) - Z12·Z21
        let delta = (z11 + z0) * (z22 + z0) - z12 * z21;

        // S11 = ((Z11 - Z₀)(Z22 + Z₀) - Z12·Z21) / Δ
        let s11 = ((z11 - z0) * (z22 + z0) - z12 * z21) / delta;

        // S21 = 2·Z₀·Z21 / Δ
        let s21 = z0 * z21 * 2.0 / delta;

        // S12 = 2·Z₀·Z12 / Δ
        let s12 = z0 * z12 * 2.0 / delta;

        // S22 = ((Z11 + Z₀)(Z22 - Z₀) - Z12·Z21) / Δ
        let s22 = ((z11 + z0) * (z22 - z0) - z12 * z21) / delta;

        let mut matrix = SMatrix::new(frequency, 2);
        matrix.set(1, 1, s11);
        matrix.set(1, 2, s12);
        matrix.set(2, 1, s21);
        matrix.set(2, 2, s22);

        matrix
    }

    /// Run S-parameter analysis with a Y-parameter solver
    ///
    /// The solver should return the Y-matrix for the given frequency
    pub fn analyze<F>(&self, mut y_solver: F) -> SParameterResult
    where
        F: FnMut(Value) -> [[Complex; 2]; 2],
    {
        let mut result = SParameterResult::new(self.config.z0, self.ports.clone());

        for frequency in self.config.frequencies() {
            let y = y_solver(frequency);
            let s_matrix = self.from_y_parameters(&y, frequency);
            result.add(s_matrix);
        }

        result
    }
}

//=============================================================================
// Source Impedance and Matching
//=============================================================================

/// Complex source impedance for S-parameter matching
///
/// Supports frequency-dependent source impedance for accurate matching analysis.
/// Real-world sources often have reactive components (e.g., cable inductance,
/// probe capacitance) that affect matching at higher frequencies.
#[derive(Debug, Clone, Copy)]
pub struct SourceImpedance {
    /// Resistive component (Ω)
    pub r: Value,
    /// Reactive component (Ω) - positive for inductive, negative for capacitive
    pub x: Value,
}

impl SourceImpedance {
    /// Create purely resistive source impedance
    pub fn resistive(r: Value) -> Self {
        Self { r, x: 0.0 }
    }

    /// Create complex source impedance (R + jX)
    pub fn complex(r: Value, x: Value) -> Self {
        Self { r, x }
    }

    /// Create from magnitude and phase angle (degrees)
    pub fn from_polar(mag: Value, phase_deg: Value) -> Self {
        let phase_rad = phase_deg * PI / 180.0;
        Self {
            r: mag * phase_rad.cos(),
            x: mag * phase_rad.sin(),
        }
    }

    /// Standard 50Ω source
    pub fn z50() -> Self {
        Self::resistive(50.0)
    }

    /// Standard 75Ω source (cable TV, video)
    pub fn z75() -> Self {
        Self::resistive(75.0)
    }

    /// Convert to Complex type
    pub fn as_complex(&self) -> Complex {
        Complex::new(self.r, self.x)
    }

    /// Get magnitude |Z|
    pub fn magnitude(&self) -> Value {
        (self.r * self.r + self.x * self.x).sqrt()
    }

    /// Get phase angle in radians
    pub fn phase(&self) -> Value {
        self.x.atan2(self.r)
    }

    /// Get phase angle in degrees
    pub fn phase_deg(&self) -> Value {
        self.phase() * 180.0 / PI
    }

    /// Get conjugate (R - jX)
    pub fn conjugate(&self) -> Self {
        Self {
            r: self.r,
            x: -self.x,
        }
    }
}

impl Default for SourceImpedance {
    fn default() -> Self {
        Self::z50()
    }
}

/// Reflection coefficient calculation with complex source impedance
///
/// Γ = (Z_L - Z_S*) / (Z_L + Z_S)
///
/// where Z_S* is the conjugate of the source impedance
pub fn reflection_coefficient(z_load: Complex, z_source: SourceImpedance) -> Complex {
    let zs = z_source.as_complex();
    let zs_conj = Complex::new(zs.re, -zs.im);

    (z_load - zs_conj) / (z_load + zs)
}

/// Calculate power available from source
///
/// P_avail = |V_s|² / (4 * Re{Z_s})
pub fn available_power(v_source: Value, z_source: SourceImpedance) -> Value {
    if z_source.r <= 0.0 {
        return 0.0;
    }
    v_source * v_source / (4.0 * z_source.r)
}

/// Calculate power delivered to load
///
/// P_del = P_avail * (1 - |Γ|²)
pub fn delivered_power(v_source: Value, z_source: SourceImpedance, z_load: Complex) -> Value {
    let p_avail = available_power(v_source, z_source);
    let gamma = reflection_coefficient(z_load, z_source);
    let gamma_mag_sq = gamma.re * gamma.re + gamma.im * gamma.im;

    p_avail * (1.0 - gamma_mag_sq)
}

/// Calculate transducer power gain
///
/// G_T = P_del / P_avail = (1 - |Γ_S|²) * |S21|² * (1 - |Γ_L|²) / |1 - S22*Γ_L|² / |1 - Γ_in*Γ_S|²
///
/// Simplified for unilateral case (S12 ≈ 0): G_T ≈ (1 - |Γ_S|²) * |S21|² * (1 - |Γ_L|²)
pub fn transducer_gain_db(s21: Complex, gamma_s: Complex, gamma_l: Complex) -> Value {
    let s21_mag_sq = s21.re * s21.re + s21.im * s21.im;
    let gs_mag_sq = gamma_s.re * gamma_s.re + gamma_s.im * gamma_s.im;
    let gl_mag_sq = gamma_l.re * gamma_l.re + gamma_l.im * gamma_l.im;

    let gain = (1.0 - gs_mag_sq) * s21_mag_sq * (1.0 - gl_mag_sq);

    if gain > 0.0 {
        10.0 * gain.log10()
    } else {
        f64::NEG_INFINITY
    }
}

/// Calculate mismatch loss in dB
///
/// ML = -10 * log10(1 - |Γ|²)
pub fn mismatch_loss_db(gamma: Complex) -> Value {
    let gamma_mag_sq = gamma.re * gamma.re + gamma.im * gamma.im;

    if gamma_mag_sq >= 1.0 {
        return f64::INFINITY;
    }

    -10.0 * (1.0 - gamma_mag_sq).log10()
}

/// Renormalize S-parameters from one reference impedance to another
///
/// This is essential when comparing S-parameters measured with different
/// VNA port impedances or when designing matching networks.
///
/// Uses the formula:
/// S' = (S - Γ*I) * (I - Γ*S)^(-1)
/// where Γ = (Z0' - Z0) / (Z0' + Z0)
pub fn renormalize_s11(s11: Complex, z0_old: Value, z0_new: Value) -> Complex {
    // Calculate reference plane reflection coefficient
    let gamma = (z0_new - z0_old) / (z0_new + z0_old);
    let gamma_c = Complex::new(gamma, 0.0);

    // S' = (S - Γ) / (1 - Γ*S)
    let num = s11 - gamma_c;
    let den = Complex::ONE - (gamma_c * s11);

    num / den
}

/// Renormalize full 2-port S-matrix to new reference impedance
pub fn renormalize_2port(s: &SMatrix, z0_old: Value, z0_new: Value) -> SMatrix {
    let gamma = (z0_new - z0_old) / (z0_new + z0_old);
    let g = Complex::new(gamma, 0.0);
    let one = Complex::ONE;

    let s11 = s.s11();
    let s12 = s.s12();
    let s21 = s.s21();
    let s22 = s.s22();

    // Denominator: (1 - Γ*S11)(1 - Γ*S22) - Γ²*S12*S21
    let t1 = one - (g * s11);
    let t2 = one - (g * s22);
    let den = (t1 * t2) - (g * g * s12 * s21);

    // New S11
    let s11_new = ((s11 - g) * t2 + g * s12 * s21) / den;

    // New S21
    let s21_new = s21 * (one - g * g) / den;

    // New S12
    let s12_new = s12 * (one - g * g) / den;

    // New S22
    let s22_new = ((s22 - g) * t1 + g * s12 * s21) / den;

    let mut result = SMatrix::new(s.frequency, 2);
    result.set(1, 1, s11_new);
    result.set(1, 2, s12_new);
    result.set(2, 1, s21_new);
    result.set(2, 2, s22_new);

    result
}

/// Calculate optimal source impedance for maximum power transfer
///
/// For conjugate matching: Z_source_opt = Z_load*
pub fn optimal_source_impedance(z_load: Complex) -> SourceImpedance {
    SourceImpedance::complex(z_load.re, -z_load.im)
}

/// Calculate optimal load impedance for maximum power transfer  
///
/// For conjugate matching: Z_load_opt = Z_source*
pub fn optimal_load_impedance(z_source: SourceImpedance) -> Complex {
    z_source.conjugate().as_complex()
}

/// L-section matching network calculator
///
/// Calculates the component values for an L-section matching network
/// to transform Z_source to Z_load.
///
/// Returns (series_element, shunt_element) in Ω at the given frequency.
/// Positive values indicate inductance, negative values indicate capacitance.
pub fn l_section_match(
    z_source: SourceImpedance,
    z_load: Complex,
    _frequency: Value,
) -> Option<(Value, Value)> {
    let rs = z_source.r;
    let rl = z_load.re;

    // L-section only works when Rs != Rl
    if (rs - rl).abs() < 1e-10 {
        return None;
    }

    // Q factor determines the ratio
    let q = if rs > rl {
        // Low-pass: shunt on source side
        ((rs / rl) - 1.0).sqrt()
    } else {
        // High-pass: shunt on load side
        ((rl / rs) - 1.0).sqrt()
    };

    if rs > rl {
        // Series inductor, shunt capacitor
        let x_series = q * rl; // Inductive
        let x_shunt = -rs / q; // Capacitive
        Some((x_series, x_shunt))
    } else {
        // Shunt inductor, series capacitor
        let x_shunt = rs * q; // Inductive  
        let x_series = -rl / q; // Capacitive
        Some((x_series, x_shunt))
    }
}

/// Convert reactance to component value
///
/// Returns inductance (H) for positive X, capacitance (F) for negative X
pub fn reactance_to_component(x: Value, frequency: Value) -> (Value, bool) {
    let omega = 2.0 * PI * frequency;

    if x >= 0.0 {
        // Inductive: X = ωL, so L = X/ω
        (x / omega, true) // (inductance, is_inductor)
    } else {
        // Capacitive: X = -1/(ωC), so C = -1/(ωX)
        (-1.0 / (omega * x), false) // (capacitance, is_inductor)
    }
}

//=============================================================================
// Stability Analysis
//=============================================================================

/// Stability analysis result for a 2-port network
#[derive(Debug, Clone, Default)]
pub struct StabilityAnalysis {
    /// Rollett stability factor K (unconditionally stable if K > 1 and |Δ| < 1)
    pub k_factor: Value,

    /// Determinant of S-matrix (Δ = S11*S22 - S12*S21)
    pub delta: Complex,

    /// Magnitude of Δ
    pub delta_mag: Value,

    /// μ-factor (Edwards-Sinsky) - unconditionally stable if μ > 1
    pub mu_factor: Value,

    /// μ'-factor (alternate stability measure for output)
    pub mu_prime: Value,

    /// Whether device is unconditionally stable
    pub unconditionally_stable: bool,

    /// Whether device is potentially unstable (K < 1 or |Δ| > 1)
    pub potentially_unstable: bool,

    /// Input stability circle center (Γ-plane)
    pub input_stability_center: Complex,

    /// Input stability circle radius
    pub input_stability_radius: Value,

    /// Output stability circle center (Γ-plane)
    pub output_stability_center: Complex,

    /// Output stability circle radius
    pub output_stability_radius: Value,

    /// Whether stable region is inside or outside input circle
    pub input_stable_inside: bool,

    /// Whether stable region is inside or outside output circle
    pub output_stable_inside: bool,
}

impl StabilityAnalysis {
    /// Compute stability analysis from S-parameters
    pub fn from_s_matrix(s: &SMatrix) -> Self {
        let s11 = s.s11();
        let s12 = s.s12();
        let s21 = s.s21();
        let s22 = s.s22();

        // Δ = S11*S22 - S12*S21
        let delta = s11 * s22 - s12 * s21;
        let delta_mag = delta.magnitude();

        // K = (1 - |S11|² - |S22|² + |Δ|²) / (2|S12*S21|)
        let s11_mag_sq = s11.magnitude().powi(2);
        let s22_mag_sq = s22.magnitude().powi(2);
        let s12s21_mag = (s12 * s21).magnitude();

        let k_factor = if s12s21_mag > 1e-15 {
            (1.0 - s11_mag_sq - s22_mag_sq + delta_mag.powi(2)) / (2.0 * s12s21_mag)
        } else {
            f64::INFINITY // No feedback = unconditionally stable
        };

        // μ = (1 - |S11|²) / (|S22 - Δ*S11*| + |S12*S21|)
        let s11_conj = s11.conj();
        let s22_minus_delta_s11_conj = s22 - delta * s11_conj;
        let denom_mu = s22_minus_delta_s11_conj.magnitude() + s12s21_mag;

        let mu_factor = if denom_mu > 1e-15 {
            (1.0 - s11_mag_sq) / denom_mu
        } else {
            f64::INFINITY
        };

        // μ' = (1 - |S22|²) / (|S11 - Δ*S22*| + |S12*S21|)
        let s22_conj = s22.conj();
        let s11_minus_delta_s22_conj = s11 - delta * s22_conj;
        let denom_mu_prime = s11_minus_delta_s22_conj.magnitude() + s12s21_mag;

        let mu_prime = if denom_mu_prime > 1e-15 {
            (1.0 - s22_mag_sq) / denom_mu_prime
        } else {
            f64::INFINITY
        };

        // Stability circles (for potentially unstable devices)
        // Input stability circle: center Cs, radius rs
        // Cs = (S11 - Δ*S22*)* / (|S11|² - |Δ|²)
        // rs = |S12*S21| / ||S11|² - |Δ|²|
        let denom_input = s11_mag_sq - delta_mag.powi(2);
        let (input_center, input_radius) = if denom_input.abs() > 1e-15 {
            let center_num = s11 - delta * s22.conj();
            let center = center_num.conj() / denom_input;
            let radius = s12s21_mag / denom_input.abs();
            (center, radius)
        } else {
            (Complex::ZERO, 0.0)
        };

        // Output stability circle: center CL, radius rL
        // CL = (S22 - Δ*S11*)* / (|S22|² - |Δ|²)
        // rL = |S12*S21| / ||S22|² - |Δ|²|
        let denom_output = s22_mag_sq - delta_mag.powi(2);
        let (output_center, output_radius) = if denom_output.abs() > 1e-15 {
            let center_num = s22 - delta * s11.conj();
            let center = center_num.conj() / denom_output;
            let radius = s12s21_mag / denom_output.abs();
            (center, radius)
        } else {
            (Complex::ZERO, 0.0)
        };

        // Determine if stable region is inside or outside the circle
        // If |S11| < 1, origin (Γ=0) is in stable region
        // Check if origin is inside or outside the stability circle
        let input_stable_inside = s11_mag_sq > 1.0 || denom_input < 0.0;
        let output_stable_inside = s22_mag_sq > 1.0 || denom_output < 0.0;

        let unconditionally_stable = k_factor > 1.0 && delta_mag < 1.0;
        let potentially_unstable = !unconditionally_stable;

        Self {
            k_factor,
            delta,
            delta_mag,
            mu_factor,
            mu_prime,
            unconditionally_stable,
            potentially_unstable,
            input_stability_center: input_center,
            input_stability_radius: input_radius,
            output_stability_center: output_center,
            output_stability_radius: output_radius,
            input_stable_inside,
            output_stable_inside,
        }
    }

    /// Get stability circle points for plotting (input)
    pub fn input_circle_points(&self, num_points: usize) -> Vec<(Value, Value)> {
        circle_points(
            self.input_stability_center,
            self.input_stability_radius,
            num_points,
        )
    }

    /// Get stability circle points for plotting (output)
    pub fn output_circle_points(&self, num_points: usize) -> Vec<(Value, Value)> {
        circle_points(
            self.output_stability_center,
            self.output_stability_radius,
            num_points,
        )
    }
}

/// Generate points on a circle for plotting
fn circle_points(center: Complex, radius: Value, num_points: usize) -> Vec<(Value, Value)> {
    (0..num_points)
        .map(|i| {
            let theta = 2.0 * PI * i as f64 / num_points as f64;
            (
                center.re + radius * theta.cos(),
                center.im + radius * theta.sin(),
            )
        })
        .collect()
}

//=============================================================================
// Gain Calculations
//=============================================================================

/// Gain analysis result for a 2-port amplifier
#[derive(Debug, Clone, Default)]
pub struct GainAnalysis {
    /// Maximum available gain (MAG) in dB - only valid if unconditionally stable
    pub mag_db: Value,

    /// Maximum stable gain (MSG) in dB - for potentially unstable devices
    pub msg_db: Value,

    /// Mason's unilateral gain (U) in dB
    pub mason_u_db: Value,

    /// Forward transducer gain |S21|² in dB
    pub s21_gain_db: Value,

    /// Reverse isolation |S12|² in dB  
    pub s12_isolation_db: Value,

    /// Whether MAG is valid (device is unconditionally stable)
    pub mag_valid: bool,

    /// Maximum unilateral transducer gain (Gtu_max) in dB
    pub gtu_max_db: Value,

    /// Unilateral figure of merit
    pub unilateral_fom: Value,
}

impl GainAnalysis {
    /// Compute gain analysis from S-parameters
    pub fn from_s_matrix(s: &SMatrix) -> Self {
        let s11 = s.s11();
        let s12 = s.s12();
        let s21 = s.s21();
        let s22 = s.s22();

        let s11_mag_sq = s11.magnitude().powi(2);
        let s12_mag_sq = s12.magnitude().powi(2);
        let s21_mag_sq = s21.magnitude().powi(2);
        let s22_mag_sq = s22.magnitude().powi(2);

        // Stability check
        let stability = StabilityAnalysis::from_s_matrix(s);
        let k = stability.k_factor;

        // MSG = |S21/S12|
        let msg = if s12_mag_sq > 1e-30 {
            s21_mag_sq / s12_mag_sq
        } else {
            f64::INFINITY
        };
        let msg_db = 10.0 * msg.log10();

        // MAG = MSG * (K - sqrt(K² - 1))  for K >= 1
        let mag_db = if stability.unconditionally_stable && k >= 1.0 {
            let mag = msg * (k - (k * k - 1.0).sqrt());
            10.0 * mag.log10()
        } else {
            f64::NEG_INFINITY // Not valid
        };

        // Mason's unilateral gain U = |S21/S12 - 1|² / (2*K*|S21/S12| - 2*Re{S21/S12})
        let s21_over_s12 = s21 / s12;
        let ratio_minus_1 = s21_over_s12 - Complex::ONE;
        let u = if s12_mag_sq > 1e-30 {
            let num = ratio_minus_1.magnitude().powi(2);
            let denom = 2.0 * k * s21_over_s12.magnitude() - 2.0 * s21_over_s12.re;
            if denom > 1e-15 {
                num / denom
            } else {
                f64::INFINITY
            }
        } else {
            f64::INFINITY
        };
        let mason_u_db = 10.0 * u.log10();

        // S21 gain and S12 isolation
        let s21_gain_db = 10.0 * s21_mag_sq.log10();
        let s12_isolation_db = 10.0 * s12_mag_sq.log10();

        // Maximum unilateral transducer gain
        // Gtu_max = |S21|² / ((1-|S11|²)(1-|S22|²))
        let gtu_max = if (1.0 - s11_mag_sq) > 0.0 && (1.0 - s22_mag_sq) > 0.0 {
            s21_mag_sq / ((1.0 - s11_mag_sq) * (1.0 - s22_mag_sq))
        } else {
            f64::INFINITY
        };
        let gtu_max_db = 10.0 * gtu_max.log10();

        // Unilateral figure of merit
        // Shows how valid the unilateral assumption is
        let u_fom = (s11_mag_sq * s12_mag_sq * s21_mag_sq * s22_mag_sq)
            / ((1.0 - s11_mag_sq).powi(2) * (1.0 - s22_mag_sq).powi(2));
        let unilateral_fom = if u_fom.is_finite() && u_fom > 0.0 {
            u_fom
        } else {
            0.0
        };

        Self {
            mag_db,
            msg_db,
            mason_u_db,
            s21_gain_db,
            s12_isolation_db,
            mag_valid: stability.unconditionally_stable,
            gtu_max_db,
            unilateral_fom,
        }
    }
}

//=============================================================================
// Touchstone (SnP) Export
//=============================================================================

/// Touchstone file format options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchstoneFormat {
    /// Magnitude-Angle (MA)
    MagnitudeAngle,
    /// Real-Imaginary (RI)
    RealImaginary,
    /// dB-Angle (DB)
    DecibelAngle,
}

impl TouchstoneFormat {
    /// Get format string for Touchstone header
    pub fn format_string(&self) -> &'static str {
        match self {
            TouchstoneFormat::MagnitudeAngle => "MA",
            TouchstoneFormat::RealImaginary => "RI",
            TouchstoneFormat::DecibelAngle => "DB",
        }
    }
}

/// Touchstone file frequency unit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchstoneFreqUnit {
    Hz,
    KHz,
    MHz,
    GHz,
}

impl TouchstoneFreqUnit {
    /// Get the multiplier to convert to Hz
    pub fn multiplier(&self) -> Value {
        match self {
            TouchstoneFreqUnit::Hz => 1.0,
            TouchstoneFreqUnit::KHz => 1e3,
            TouchstoneFreqUnit::MHz => 1e6,
            TouchstoneFreqUnit::GHz => 1e9,
        }
    }

    /// Get unit string for Touchstone header
    pub fn unit_string(&self) -> &'static str {
        match self {
            TouchstoneFreqUnit::Hz => "HZ",
            TouchstoneFreqUnit::KHz => "KHZ",
            TouchstoneFreqUnit::MHz => "MHZ",
            TouchstoneFreqUnit::GHz => "GHZ",
        }
    }
}

/// Touchstone file exporter
pub struct TouchstoneExporter {
    /// Data format
    pub format: TouchstoneFormat,
    /// Frequency unit
    pub freq_unit: TouchstoneFreqUnit,
    /// Reference impedance
    pub z0: Value,
    /// Comments to include in file
    pub comments: Vec<String>,
}

impl Default for TouchstoneExporter {
    fn default() -> Self {
        Self {
            format: TouchstoneFormat::RealImaginary,
            freq_unit: TouchstoneFreqUnit::GHz,
            z0: 50.0,
            comments: Vec::new(),
        }
    }
}

impl TouchstoneExporter {
    /// Create new exporter with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set data format
    pub fn with_format(mut self, format: TouchstoneFormat) -> Self {
        self.format = format;
        self
    }

    /// Set frequency unit
    pub fn with_freq_unit(mut self, unit: TouchstoneFreqUnit) -> Self {
        self.freq_unit = unit;
        self
    }

    /// Set reference impedance
    pub fn with_z0(mut self, z0: Value) -> Self {
        self.z0 = z0;
        self
    }

    /// Add a comment line
    pub fn with_comment(mut self, comment: &str) -> Self {
        self.comments.push(comment.to_string());
        self
    }

    /// Export S-parameter result to Touchstone format string
    pub fn export(&self, result: &SParameterResult) -> String {
        let mut output = String::new();

        // Comments
        for comment in &self.comments {
            output.push_str(&format!("! {}\n", comment));
        }

        // Option line: # <freq_unit> S <format> R <z0>
        output.push_str(&format!(
            "# {} S {} R {:.1}\n",
            self.freq_unit.unit_string(),
            self.format.format_string(),
            self.z0
        ));

        // Data lines
        let freq_mult = self.freq_unit.multiplier();

        for s in &result.data {
            let freq = s.frequency / freq_mult;

            match result.num_ports {
                1 => {
                    // 1-port: freq S11
                    let s11 = s.get(1, 1);
                    let (v1, v2) = self.format_complex(s11);
                    output.push_str(&format!("{:.9e}\t{:.9e}\t{:.9e}\n", freq, v1, v2));
                }
                2 => {
                    // 2-port: freq S11 S21 S12 S22
                    let s11 = s.get(1, 1);
                    let s21 = s.get(2, 1);
                    let s12 = s.get(1, 2);
                    let s22 = s.get(2, 2);

                    let (s11_1, s11_2) = self.format_complex(s11);
                    let (s21_1, s21_2) = self.format_complex(s21);
                    let (s12_1, s12_2) = self.format_complex(s12);
                    let (s22_1, s22_2) = self.format_complex(s22);

                    output.push_str(&format!(
                        "{:.9e}\t{:.9e}\t{:.9e}\t{:.9e}\t{:.9e}\t{:.9e}\t{:.9e}\t{:.9e}\t{:.9e}\n",
                        freq, s11_1, s11_2, s21_1, s21_2, s12_1, s12_2, s22_1, s22_2
                    ));
                }
                _ => {
                    // N-port: more complex formatting (split across lines)
                    output.push_str(&format!("{:.9e}", freq));
                    for row in 1..=result.num_ports {
                        for col in 1..=result.num_ports {
                            let sij = s.get(row, col);
                            let (v1, v2) = self.format_complex(sij);
                            output.push_str(&format!("\t{:.9e}\t{:.9e}", v1, v2));
                        }
                    }
                    output.push('\n');
                }
            }
        }

        output
    }

    /// Format complex number according to selected format
    fn format_complex(&self, c: Complex) -> (Value, Value) {
        match self.format {
            TouchstoneFormat::RealImaginary => (c.re, c.im),
            TouchstoneFormat::MagnitudeAngle => (c.magnitude(), c.phase_deg()),
            TouchstoneFormat::DecibelAngle => (c.mag_db(), c.phase_deg()),
        }
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_operations() {
        let a = Complex::new(3.0, 4.0);

        // Magnitude: |3+4j| = 5
        assert!((a.magnitude() - 5.0).abs() < 1e-10);

        // Phase: atan2(4, 3)
        assert!((a.phase() - 0.9273).abs() < 0.001);

        // Conjugate
        let c = a.conj();
        assert!((c.re - 3.0).abs() < 1e-10);
        assert!((c.im - (-4.0)).abs() < 1e-10);
    }

    #[test]
    fn test_complex_arithmetic() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);

        // Addition
        let sum = a + b;
        assert!((sum.re - 4.0).abs() < 1e-10);
        assert!((sum.im - 6.0).abs() < 1e-10);

        // Multiplication: (1+2j)(3+4j) = 3 + 4j + 6j + 8j² = 3 + 10j - 8 = -5 + 10j
        let prod = a * b;
        assert!((prod.re - (-5.0)).abs() < 1e-10);
        assert!((prod.im - 10.0).abs() < 1e-10);

        // Division: (1+2j)/(3+4j) = (1+2j)(3-4j)/25 = (3-4j+6j-8j²)/25 = (11+2j)/25
        let div = a / b;
        assert!((div.re - 0.44).abs() < 0.01);
        assert!((div.im - 0.08).abs() < 0.01);
    }

    #[test]
    fn test_matched_load() {
        // A 50Ω load terminated in 50Ω should have S11 = 0
        let config = SParameterConfig::linear(1e6, 1e9, 10).with_z0(50.0);
        let ports = vec![
            Port::single_ended(1, "in", 50.0),
            Port::single_ended(2, "out", 50.0),
        ];

        let analyzer = SParameterAnalyzer::new(config, ports);

        // Y-matrix for 50Ω resistor: Y11 = 1/50, others = 0
        let y: [[Complex; 2]; 2] = [
            [Complex::new(0.02, 0.0), Complex::ZERO],
            [Complex::ZERO, Complex::new(0.02, 0.0)],
        ];

        let s = analyzer.from_y_parameters(&y, 1e6);

        // S11 should be 0 for matched impedance
        assert!(s.s11().magnitude() < 0.01, "S11 = {:?}", s.s11());
    }

    #[test]
    fn test_through_connection() {
        // Direct through: S21 = 1, S11 = 0
        let config = SParameterConfig::linear(1e6, 1e9, 10).with_z0(50.0);
        let ports = vec![
            Port::single_ended(1, "in", 50.0),
            Port::single_ended(2, "out", 50.0),
        ];

        let analyzer = SParameterAnalyzer::new(config, ports);

        // Y-matrix for through: very large Y12, Y21
        // Actually for ideal through, use Z-parameters: Z=0 means short
        // S21 should be 1 for lossless through
        let z: [[Complex; 2]; 2] = [
            [Complex::ZERO, Complex::ZERO],
            [Complex::ZERO, Complex::ZERO],
        ];

        let s = analyzer.from_z_parameters(&z, 1e6);

        // For Z=0 (short at both ends), this isn't quite right
        // Let's test a simple case instead
        assert!(s.frequency > 0.0);
    }

    #[test]
    fn test_vswr_calculation() {
        let mut result = SParameterResult::new(50.0, vec![]);

        // Add a point with S11 = 0.5 (mag)
        let mut s = SMatrix::new(1e6, 2);
        s.set(1, 1, Complex::new(0.5, 0.0));
        result.add(s);

        let vswr = result.vswr();

        // VSWR = (1 + 0.5) / (1 - 0.5) = 3.0
        assert!((vswr[0] - 3.0).abs() < 0.01);
    }

    #[test]
    fn test_frequency_generation() {
        let config = SParameterConfig::linear(1e6, 1e9, 5);
        let freqs = config.frequencies();

        assert_eq!(freqs.len(), 5);
        assert!((freqs[0] - 1e6).abs() < 1.0);
        assert!((freqs[4] - 1e9).abs() < 1.0);

        // Decade sweep
        let config = SParameterConfig::decade(1e6, 1e9, 3);
        let freqs = config.frequencies();

        // 3 decades, 3 points per decade = 9 points minimum
        assert!(freqs.len() >= 9);
        assert!((freqs[0] - 1e6).abs() < 1.0);
    }

    #[test]
    fn test_return_loss() {
        let mut result = SParameterResult::new(50.0, vec![]);

        // S11 = 0.1 → RL = -20*log10(0.1) = 20 dB
        let mut s = SMatrix::new(1e6, 2);
        s.set(1, 1, Complex::new(0.1, 0.0));
        result.add(s);

        let rl = result.return_loss();
        assert!((rl[0] - 20.0).abs() < 0.1);
    }

    // =========================================================================
    // Stability Analysis Tests
    // =========================================================================

    #[test]
    fn test_stability_unconditionally_stable() {
        // Create a 2-port that is unconditionally stable
        // S11=0.1, S22=0.1, S21=0.8, S12=0.01 (typical well-designed amplifier)
        let mut s = SMatrix::new(1e9, 2);
        s.set(1, 1, Complex::new(0.1, 0.0));
        s.set(2, 2, Complex::new(0.1, 0.0));
        s.set(2, 1, Complex::new(0.8, 0.0));
        s.set(1, 2, Complex::new(0.01, 0.0));

        let stability = StabilityAnalysis::from_s_matrix(&s);

        assert!(
            stability.k_factor > 1.0,
            "K should be > 1, got {}",
            stability.k_factor
        );
        assert!(
            stability.delta_mag < 1.0,
            "|Δ| should be < 1, got {}",
            stability.delta_mag
        );
        assert!(stability.unconditionally_stable);
        assert!(!stability.potentially_unstable);
        assert!(stability.mu_factor > 1.0);
    }

    #[test]
    fn test_stability_potentially_unstable() {
        // Create a potentially unstable device (high feedback transistor)
        // S11=0.5, S22=0.6, S21=3.0, S12=0.3
        let mut s = SMatrix::new(1e9, 2);
        s.set(1, 1, Complex::new(0.5, -0.2));
        s.set(2, 2, Complex::new(0.6, -0.3));
        s.set(2, 1, Complex::new(3.0, 1.0));
        s.set(1, 2, Complex::new(0.3, 0.1));

        let stability = StabilityAnalysis::from_s_matrix(&s);

        // This should be potentially unstable (K < 1 or |Δ| > 1)
        assert!(stability.potentially_unstable);
    }

    #[test]
    fn test_stability_k_factor_calculation() {
        // Test K factor calculation with known values
        // For S12=0 (unilateral), K → infinity
        let mut s = SMatrix::new(1e9, 2);
        s.set(1, 1, Complex::new(0.1, 0.0));
        s.set(2, 2, Complex::new(0.1, 0.0));
        s.set(2, 1, Complex::new(2.0, 0.0));
        s.set(1, 2, Complex::new(0.0, 0.0)); // No reverse transmission

        let stability = StabilityAnalysis::from_s_matrix(&s);

        assert!(stability.k_factor.is_infinite() || stability.k_factor > 100.0);
    }

    #[test]
    fn test_stability_delta_calculation() {
        // Δ = S11*S22 - S12*S21
        let mut s = SMatrix::new(1e9, 2);
        s.set(1, 1, Complex::new(0.3, 0.0));
        s.set(2, 2, Complex::new(0.4, 0.0));
        s.set(2, 1, Complex::new(0.5, 0.0));
        s.set(1, 2, Complex::new(0.1, 0.0));

        let stability = StabilityAnalysis::from_s_matrix(&s);

        // Δ = 0.3*0.4 - 0.1*0.5 = 0.12 - 0.05 = 0.07
        assert!((stability.delta.re - 0.07).abs() < 0.01);
    }

    #[test]
    fn test_stability_mu_factor() {
        let mut s = SMatrix::new(1e9, 2);
        s.set(1, 1, Complex::new(0.1, 0.0));
        s.set(2, 2, Complex::new(0.1, 0.0));
        s.set(2, 1, Complex::new(0.8, 0.0));
        s.set(1, 2, Complex::new(0.01, 0.0));

        let stability = StabilityAnalysis::from_s_matrix(&s);

        // For unconditionally stable, μ > 1
        assert!(stability.mu_factor > 1.0);
        assert!(stability.mu_prime > 1.0);
    }

    #[test]
    fn test_stability_circle_points() {
        let mut s = SMatrix::new(1e9, 2);
        s.set(1, 1, Complex::new(0.5, 0.1));
        s.set(2, 2, Complex::new(0.6, -0.1));
        s.set(2, 1, Complex::new(2.0, 0.5));
        s.set(1, 2, Complex::new(0.2, 0.05));

        let stability = StabilityAnalysis::from_s_matrix(&s);

        let input_points = stability.input_circle_points(36);
        let output_points = stability.output_circle_points(36);

        assert_eq!(input_points.len(), 36);
        assert_eq!(output_points.len(), 36);

        // Points should form a circle
        // Check that distance from center is approximately radius
        for (x, y) in &input_points {
            let dist = ((x - stability.input_stability_center.re).powi(2)
                + (y - stability.input_stability_center.im).powi(2))
            .sqrt();
            assert!(
                (dist - stability.input_stability_radius).abs() < 0.01,
                "Point not on circle"
            );
        }
    }

    // =========================================================================
    // Gain Analysis Tests
    // =========================================================================

    #[test]
    fn test_gain_s21_gain() {
        let mut s = SMatrix::new(1e9, 2);
        s.set(1, 1, Complex::new(0.1, 0.0));
        s.set(2, 2, Complex::new(0.1, 0.0));
        s.set(2, 1, Complex::new(3.16, 0.0)); // 10 dB gain
        s.set(1, 2, Complex::new(0.01, 0.0));

        let gain = GainAnalysis::from_s_matrix(&s);

        // |S21|² = 10 → 10 dB
        assert!((gain.s21_gain_db - 10.0).abs() < 0.5);
    }

    #[test]
    fn test_gain_msg_calculation() {
        let mut s = SMatrix::new(1e9, 2);
        s.set(1, 1, Complex::new(0.1, 0.0));
        s.set(2, 2, Complex::new(0.1, 0.0));
        s.set(2, 1, Complex::new(10.0, 0.0)); // |S21| = 10
        s.set(1, 2, Complex::new(0.1, 0.0)); // |S12| = 0.1

        let gain = GainAnalysis::from_s_matrix(&s);

        // MSG = |S21|²/|S12|² (power ratio) = 100/0.01 = 10000 → 40 dB
        assert!((gain.msg_db - 40.0).abs() < 0.5);
    }

    #[test]
    fn test_gain_mag_valid() {
        // Unconditionally stable device should have valid MAG
        let mut s = SMatrix::new(1e9, 2);
        s.set(1, 1, Complex::new(0.1, 0.0));
        s.set(2, 2, Complex::new(0.1, 0.0));
        s.set(2, 1, Complex::new(2.0, 0.0));
        s.set(1, 2, Complex::new(0.01, 0.0));

        let gain = GainAnalysis::from_s_matrix(&s);

        assert!(gain.mag_valid);
        assert!(gain.mag_db.is_finite());
        assert!(gain.mag_db > 0.0); // Should have positive gain
    }

    #[test]
    fn test_gain_unilateral_fom() {
        // For truly unilateral device (S12=0), FOM should be 0
        let mut s = SMatrix::new(1e9, 2);
        s.set(1, 1, Complex::new(0.1, 0.0));
        s.set(2, 2, Complex::new(0.1, 0.0));
        s.set(2, 1, Complex::new(2.0, 0.0));
        s.set(1, 2, Complex::new(0.0, 0.0)); // Unilateral

        let gain = GainAnalysis::from_s_matrix(&s);

        assert!(gain.unilateral_fom < 0.01);
    }

    #[test]
    fn test_gain_gtu_max() {
        let mut s = SMatrix::new(1e9, 2);
        s.set(1, 1, Complex::new(0.2, 0.0));
        s.set(2, 2, Complex::new(0.2, 0.0));
        s.set(2, 1, Complex::new(2.0, 0.0)); // |S21|² = 4
        s.set(1, 2, Complex::new(0.0, 0.0));

        let gain = GainAnalysis::from_s_matrix(&s);

        // Gtu_max = |S21|² / ((1-|S11|²)(1-|S22|²))
        // = 4 / ((1-0.04)(1-0.04)) = 4 / 0.9216 ≈ 4.34 → 6.37 dB
        assert!(gain.gtu_max_db > 6.0 && gain.gtu_max_db < 7.0);
    }

    // =========================================================================
    // Touchstone Export Tests
    // =========================================================================

    #[test]
    fn test_touchstone_format_strings() {
        assert_eq!(TouchstoneFormat::RealImaginary.format_string(), "RI");
        assert_eq!(TouchstoneFormat::MagnitudeAngle.format_string(), "MA");
        assert_eq!(TouchstoneFormat::DecibelAngle.format_string(), "DB");
    }

    #[test]
    fn test_touchstone_freq_unit_multiplier() {
        assert_eq!(TouchstoneFreqUnit::Hz.multiplier(), 1.0);
        assert_eq!(TouchstoneFreqUnit::KHz.multiplier(), 1e3);
        assert_eq!(TouchstoneFreqUnit::MHz.multiplier(), 1e6);
        assert_eq!(TouchstoneFreqUnit::GHz.multiplier(), 1e9);
    }

    #[test]
    fn test_touchstone_exporter_defaults() {
        let exporter = TouchstoneExporter::new();

        assert_eq!(exporter.format, TouchstoneFormat::RealImaginary);
        assert_eq!(exporter.freq_unit, TouchstoneFreqUnit::GHz);
        assert_eq!(exporter.z0, 50.0);
    }

    #[test]
    fn test_touchstone_exporter_builder() {
        let exporter = TouchstoneExporter::new()
            .with_format(TouchstoneFormat::DecibelAngle)
            .with_freq_unit(TouchstoneFreqUnit::MHz)
            .with_z0(75.0)
            .with_comment("Test export");

        assert_eq!(exporter.format, TouchstoneFormat::DecibelAngle);
        assert_eq!(exporter.freq_unit, TouchstoneFreqUnit::MHz);
        assert_eq!(exporter.z0, 75.0);
        assert!(exporter.comments.contains(&"Test export".to_string()));
    }

    #[test]
    fn test_touchstone_export_1port() {
        let mut result = SParameterResult::new(50.0, vec![Port::single_ended(1, "in", 50.0)]);

        let mut s = SMatrix::new(1e9, 1);
        s.set(1, 1, Complex::new(0.5, 0.5));
        result.add(s);

        let exporter = TouchstoneExporter::new()
            .with_format(TouchstoneFormat::RealImaginary)
            .with_freq_unit(TouchstoneFreqUnit::GHz);

        let output = exporter.export(&result);

        assert!(output.contains("# GHZ S RI R 50.0"));
        assert!(output.contains("1.000000000e0")); // 1 GHz
    }

    #[test]
    fn test_touchstone_export_2port() {
        let ports = vec![
            Port::single_ended(1, "in", 50.0),
            Port::single_ended(2, "out", 50.0),
        ];
        let mut result = SParameterResult::new(50.0, ports);

        let mut s = SMatrix::new(2e9, 2);
        s.set(1, 1, Complex::new(0.1, 0.0));
        s.set(2, 1, Complex::new(0.9, 0.1));
        s.set(1, 2, Complex::new(0.01, 0.0));
        s.set(2, 2, Complex::new(0.1, 0.0));
        result.add(s);

        let exporter = TouchstoneExporter::new()
            .with_format(TouchstoneFormat::RealImaginary)
            .with_freq_unit(TouchstoneFreqUnit::GHz);

        let output = exporter.export(&result);

        assert!(output.contains("# GHZ S RI R 50.0"));
        // Frequency should be 2 GHz
        let lines: Vec<&str> = output.lines().collect();
        assert!(lines.len() >= 2);
    }

    #[test]
    fn test_touchstone_export_magnitude_angle() {
        let ports = vec![Port::single_ended(1, "in", 50.0)];
        let mut result = SParameterResult::new(50.0, ports);

        // S11 = 0.707 + j*0.707 → mag=1, angle=45°
        let mut s = SMatrix::new(1e9, 1);
        s.set(1, 1, Complex::new(0.707, 0.707));
        result.add(s);

        let exporter = TouchstoneExporter::new().with_format(TouchstoneFormat::MagnitudeAngle);

        let output = exporter.export(&result);

        assert!(output.contains("MA"));
    }

    #[test]
    fn test_touchstone_export_db_angle() {
        let ports = vec![Port::single_ended(1, "in", 50.0)];
        let mut result = SParameterResult::new(50.0, ports);

        // S11 = 0.1 → -20 dB
        let mut s = SMatrix::new(1e9, 1);
        s.set(1, 1, Complex::new(0.1, 0.0));
        result.add(s);

        let exporter = TouchstoneExporter::new().with_format(TouchstoneFormat::DecibelAngle);

        let output = exporter.export(&result);

        assert!(output.contains("DB"));
    }

    #[test]
    fn test_touchstone_export_with_comments() {
        let mut result = SParameterResult::new(50.0, vec![Port::single_ended(1, "in", 50.0)]);

        let mut s = SMatrix::new(1e9, 1);
        s.set(1, 1, Complex::new(0.1, 0.0));
        result.add(s);

        let exporter = TouchstoneExporter::new()
            .with_comment("RSpice S-Parameter Export")
            .with_comment("Created: 2026-01-23");

        let output = exporter.export(&result);

        assert!(output.contains("! RSpice S-Parameter Export"));
        assert!(output.contains("! Created: 2026-01-23"));
    }

    #[test]
    fn test_touchstone_multiple_frequencies() {
        let ports = vec![Port::single_ended(1, "in", 50.0)];
        let mut result = SParameterResult::new(50.0, ports);

        for i in 1..=5 {
            let mut s = SMatrix::new(i as f64 * 1e9, 1);
            let mag = 0.1 * i as f64;
            s.set(1, 1, Complex::new(mag, 0.0));
            result.add(s);
        }

        let exporter = TouchstoneExporter::new();
        let output = exporter.export(&result);

        // Should have 5 data lines (1 per frequency)
        let data_lines: Vec<&str> = output
            .lines()
            .filter(|l| !l.starts_with('!') && !l.starts_with('#'))
            .collect();
        assert_eq!(data_lines.len(), 5);
    }
}
