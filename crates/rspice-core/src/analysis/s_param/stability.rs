use super::*;

/// Divide a signed stability numerator by a non-negative magnitude without
/// inventing a favorable result at an exact zero denominator.
fn signed_stability_ratio(numerator: Value, denominator: Value) -> Value {
    if denominator != 0.0 {
        numerator / denominator
    } else if numerator > 0.0 {
        Value::INFINITY
    } else if numerator < 0.0 {
        Value::NEG_INFINITY
    } else {
        Value::NAN
    }
}

#[derive(Debug, Clone, Default)]
pub struct StabilityAnalysis {
    /// Rollett stability factor K (unconditionally stable if K > 1 and |Δ| < 1)
    pub k_factor: Value,

    /// Determinant of S-matrix (Δ = S11*S22 - S12*S21)
    pub delta: Complex64,

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
    pub input_stability_center: Complex64,

    /// Input stability circle radius
    pub input_stability_radius: Value,

    /// Output stability circle center (Γ-plane)
    pub output_stability_center: Complex64,

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
        let delta_mag_sq = delta.norm_sqr();
        let delta_mag = delta_mag_sq.sqrt();

        // K = (1 - |S11|² - |S22|² + |Δ|²) / (2|S12*S21|)
        let s11_mag_sq = s11.norm().powi(2);
        let s22_mag_sq = s22.norm().powi(2);
        let s12s21_mag = (s12 * s21).norm();

        let k_numerator = 1.0 - s11_mag_sq - s22_mag_sq + delta_mag_sq;
        let k_factor = signed_stability_ratio(k_numerator, 2.0 * s12s21_mag);

        // μ = (1 - |S11|²) / (|S22 - Δ*S11*| + |S12*S21|)
        let s11_conj = s11.conj();
        let s22_minus_delta_s11_conj = s22 - delta * s11_conj;
        let denom_mu = s22_minus_delta_s11_conj.norm() + s12s21_mag;

        let mu_factor = signed_stability_ratio(1.0 - s11_mag_sq, denom_mu);

        // μ' = (1 - |S22|²) / (|S11 - Δ*S22*| + |S12*S21|)
        let s22_conj = s22.conj();
        let s11_minus_delta_s22_conj = s11 - delta * s22_conj;
        let denom_mu_prime = s11_minus_delta_s22_conj.norm() + s12s21_mag;

        let mu_prime = signed_stability_ratio(1.0 - s22_mag_sq, denom_mu_prime);

        // Stability circles (for potentially unstable devices)
        // Input stability circle: center Cs, radius rs
        // Cs = (S11 - Δ*S22*)* / (|S11|² - |Δ|²)
        // rs = |S12*S21| / ||S11|² - |Δ|²|
        let denom_input = s11_mag_sq - delta_mag_sq;
        let (input_center, input_radius) = if denom_input != 0.0 {
            let center_num = s11 - delta * s22.conj();
            let center = center_num.conj() / denom_input;
            let radius = s12s21_mag / denom_input.abs();
            (center, radius)
        } else {
            (
                Complex64::new(Value::NAN, Value::NAN),
                signed_stability_ratio(s12s21_mag, 0.0),
            )
        };

        // Output stability circle: center CL, radius rL
        // CL = (S22 - Δ*S11*)* / (|S22|² - |Δ|²)
        // rL = |S12*S21| / ||S22|² - |Δ|²|
        let denom_output = s22_mag_sq - delta_mag_sq;
        let (output_center, output_radius) = if denom_output != 0.0 {
            let center_num = s22 - delta * s11.conj();
            let center = center_num.conj() / denom_output;
            let radius = s12s21_mag / denom_output.abs();
            (center, radius)
        } else {
            (
                Complex64::new(Value::NAN, Value::NAN),
                signed_stability_ratio(s12s21_mag, 0.0),
            )
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn two_port(s11: Value, s12: Value, s21: Value, s22: Value) -> SMatrix {
        let mut matrix = SMatrix::new(1.0e9, 2);
        matrix.set(1, 1, Complex64::new(s11, 0.0));
        matrix.set(1, 2, Complex64::new(s12, 0.0));
        matrix.set(2, 1, Complex64::new(s21, 0.0));
        matrix.set(2, 2, Complex64::new(s22, 0.0));
        matrix
    }

    #[test]
    fn tiny_reverse_product_cannot_turn_negative_k_into_infinity() {
        let analysis = StabilityAnalysis::from_s_matrix(&two_port(1.01, 1.0e-8, 1.0e-8, 0.98));

        assert!(analysis.k_factor.is_finite());
        assert!(analysis.k_factor < 0.0, "K={}", analysis.k_factor);
        assert!(!analysis.unconditionally_stable);
        assert!(analysis.potentially_unstable);
    }

    #[test]
    fn stability_classification_is_independent_of_an_absolute_feedback_floor() {
        let analysis = StabilityAnalysis::from_s_matrix(&two_port(0.2, 1.0e-10, 1.0e-10, 0.3));

        assert!(analysis.k_factor.is_finite());
        assert!(analysis.k_factor > 1.0);
        assert!(analysis.unconditionally_stable);
    }

    #[test]
    fn exact_unilateral_limits_retain_the_numerator_sign() {
        let stable = StabilityAnalysis::from_s_matrix(&two_port(0.5, 0.0, 2.0, 0.5));
        assert_eq!(stable.k_factor, Value::INFINITY);
        assert!(stable.unconditionally_stable);

        let unstable = StabilityAnalysis::from_s_matrix(&two_port(1.01, 0.0, 2.0, 0.5));
        assert_eq!(unstable.k_factor, Value::NEG_INFINITY);
        assert!(!unstable.unconditionally_stable);

        let boundary = StabilityAnalysis::from_s_matrix(&two_port(1.0, 0.0, 2.0, 0.5));
        assert!(boundary.k_factor.is_nan());
        assert!(!boundary.unconditionally_stable);
    }
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

        let s11_mag_sq = s11.norm().powi(2);
        let s12_mag_sq = s12.norm().powi(2);
        let s21_mag_sq = s21.norm().powi(2);
        let s22_mag_sq = s22.norm().powi(2);

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
        let s21_over_s12 = wave_ratio(s21, s12);
        let ratio_minus_1 = s21_over_s12 - Complex64::ONE;
        let u = if s12_mag_sq > 1e-30 {
            let num = ratio_minus_1.norm().powi(2);
            let denom = 2.0 * k * s21_over_s12.norm() - 2.0 * s21_over_s12.re;
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
