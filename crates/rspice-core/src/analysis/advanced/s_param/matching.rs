use super::*;

//=============================================================================
// Source Impedance and Matching
//=============================================================================

/// Complex source impedance for S-parameter matching
///
/// Supports frequency-dependent source impedance for accurate matching analysis.
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
