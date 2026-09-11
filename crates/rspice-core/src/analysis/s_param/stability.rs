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

        // In the source plane, |Γout|² < 1 reduces to
        // -denom_input * (|Γs - Cs|² - rs²) < 0. The load-plane
        // inequality is analogous. The center/radius are undefined for the
        // straight-line boundary at a zero denominator.
        let input_stable_inside = denom_input < 0.0;
        let output_stable_inside = denom_output < 0.0;

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

    #[test]
    fn matched_amplifier_power_gains_follow_the_analytic_solution() {
        let gain = GainAnalysis::from_s_matrix(&two_port(0.0, 0.1, 2.0, 0.0));
        assert!((gain.msg_db - 10.0 * 20.0_f64.log10()).abs() < 1e-12);
        assert!((gain.mag_db - 10.0 * 4.0_f64.log10()).abs() < 1e-12);
        assert!((gain.mason_u_db - 10.0 * (3.61_f64 / 0.64).log10()).abs() < 1e-12);
        assert!(gain.mag_valid);
    }

    #[test]
    fn unilateral_limit_remains_finite_without_subtraction_or_reverse_gain_floors() {
        let expected = 10.0 * (4.0_f64 / ((1.0 - 0.04) * (1.0 - 0.09))).log10();
        for reverse in [0.0, 1e-200, 1e-100, 1e-20] {
            let gain = GainAnalysis::from_s_matrix(&two_port(0.2, reverse, 2.0, 0.3));
            assert!(gain.mag_valid);
            assert!((gain.mag_db - expected).abs() < 1e-10, "{gain:?}");
            assert!((gain.mason_u_db - expected).abs() < 1e-10, "{gain:?}");
            if reverse > 0.0 {
                assert!(gain.msg_db.is_finite());
                assert!(gain.s12_isolation_db.is_finite());
            }
        }
    }

    #[test]
    fn undefined_gain_is_distinct_from_a_zero_gain_or_perfect_unilaterality() {
        let unstable = GainAnalysis::from_s_matrix(&two_port(1.2, 0.0, 2.0, 0.5));
        assert!(!unstable.mag_valid);
        assert!(unstable.mag_db.is_nan());
        assert!(unstable.gtu_max_db.is_nan());
        assert!(unstable.unilateral_fom.is_nan());
        assert!(unstable.mason_u_db.is_nan());
        assert!(!GainAnalysis::from_s_matrix(&SMatrix::new(1e9, 1)).mag_valid);
        let zero = GainAnalysis::from_s_matrix(&two_port(0.2, 0.0, 0.0, 0.3));
        assert_eq!(zero.mag_db, Value::NEG_INFINITY);
        assert!(zero.mag_valid);
        assert!(zero.msg_db.is_nan());
        let finite = GainAnalysis::from_s_matrix(&two_port(0.2, 0.1, 2.0, 0.3));
        assert!((finite.unilateral_fom - 0.012 / 0.8736).abs() < 1e-14);
    }

    #[test]
    fn stability_circle_side_matches_loaded_reflection_inequality() {
        for matrix in [two_port(2.0, 0.5, 0.5, 0.1), two_port(0.1, 0.5, 0.5, 2.0)] {
            let stability = StabilityAnalysis::from_s_matrix(&matrix);
            for reflection in [-0.8, 0.0, 0.2, 0.4, 0.6, 0.8] {
                let gamma = Complex64::new(reflection, 0.05);
                for (center, radius, inside, direct, feedback) in [
                    (
                        stability.input_stability_center,
                        stability.input_stability_radius,
                        stability.input_stable_inside,
                        matrix.s22(),
                        matrix.s11(),
                    ),
                    (
                        stability.output_stability_center,
                        stability.output_stability_radius,
                        stability.output_stable_inside,
                        matrix.s11(),
                        matrix.s22(),
                    ),
                ] {
                    let loaded = direct
                        + matrix.s12() * matrix.s21() * gamma / (Complex64::ONE - feedback * gamma);
                    let point_inside = (gamma - center).norm() < radius;
                    assert_eq!(loaded.norm() < 1.0, point_inside == inside);
                }
            }
        }
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

    /// Unilateral figure of merit |S11 S12 S21 S22| /
    /// ((1 - |S11|²)(1 - |S22|²)), defined for both reflections below unity.
    pub unilateral_fom: Value,
}

impl GainAnalysis {
    fn undefined() -> Self {
        Self {
            mag_db: Value::NAN,
            msg_db: Value::NAN,
            mason_u_db: Value::NAN,
            s21_gain_db: Value::NAN,
            s12_isolation_db: Value::NAN,
            mag_valid: false,
            gtu_max_db: Value::NAN,
            unilateral_fom: Value::NAN,
        }
    }

    /// Compute power gains from a finite two-port S-matrix.
    /// Undefined metrics are NaN, while an exact zero gain is -infinity dB.
    pub fn from_s_matrix(s: &SMatrix) -> Self {
        let s11 = s.s11();
        let s12 = s.s12();
        let s21 = s.s21();
        let s22 = s.s22();

        let s11_mag_sq = s11.norm().powi(2);
        let s22_mag_sq = s22.norm().powi(2);
        if s.num_ports() != 2
            || [s11, s12, s21, s22]
                .iter()
                .any(|v| !v.re.is_finite() || !v.im.is_finite())
        {
            return Self::undefined();
        }

        // Stability check
        let stability = StabilityAnalysis::from_s_matrix(s);
        let k = stability.k_factor;
        if !s11_mag_sq.is_finite() || !s22_mag_sq.is_finite() || !stability.delta_mag.is_finite() {
            return Self::undefined();
        }

        // MSG is the magnitude ratio, not the squared magnitude ratio.
        // Subtract logarithms so a tiny reverse transmission does not vanish
        // behind a fixed floor or overflow the ratio before conversion to dB.
        let forward_log = log_magnitude(s21);
        let reverse_log = log_magnitude(s12);
        let msg_db = 10.0 * (forward_log - reverse_log);
        let s21_gain_db = 20.0 * forward_log;
        let s12_isolation_db = 20.0 * reverse_log;

        // The unilateral limit is also the finite limit of MAG as S12 -> 0.
        let match_denominator = (1.0 - s11_mag_sq) * (1.0 - s22_mag_sq);
        let gtu_max_db = if s11_mag_sq < 1.0 && s22_mag_sq < 1.0 {
            s21_gain_db
                - 10.0 * ((-s11_mag_sq).ln_1p() + (-s22_mag_sq).ln_1p()) / std::f64::consts::LN_10
        } else {
            Value::NAN
        };

        // MAG = MSG / (K + sqrt(K² - 1)). The reciprocal avoids
        // catastrophic subtraction at large K; factoring out K also avoids
        // squaring it. Exact unilateral networks have infinite K.
        let mag_db = if stability.unconditionally_stable && k >= 1.0 {
            if k.is_infinite() {
                gtu_max_db
            } else {
                msg_db
                    - 10.0 * (k.log10() + (1.0 + (1.0 - (1.0 / k).powi(2)).max(0.0).sqrt()).log10())
            }
        } else {
            Value::NAN
        };

        // Multiply the usual ratio expression by |S12|² before evaluating:
        // U = |S21-S12|² / (1-|S11|²-|S22|²+|Δ|²-2Re(S21*S12*)).
        // This form includes S12=0 without an infinite intermediate ratio.
        let mason_denominator = 1.0 - s11_mag_sq - s22_mag_sq + stability.delta_mag.powi(2)
            - 2.0 * (s21 * s12.conj()).re;
        let mason_u_db = if mason_denominator > 0.0 && mason_denominator.is_finite() {
            20.0 * log_magnitude(s21 - s12) - 10.0 * mason_denominator.log10()
        } else if mason_denominator == 0.0 && s21 != s12 {
            Value::INFINITY
        } else {
            Value::NAN
        };
        let unilateral_fom = if s11_mag_sq < 1.0 && s22_mag_sq < 1.0 {
            s11.norm() * s12.norm() * s21.norm() * s22.norm() / match_denominator
        } else {
            Value::NAN
        };

        Self {
            mag_db,
            msg_db,
            mason_u_db,
            s21_gain_db,
            s12_isolation_db,
            mag_valid: stability.unconditionally_stable && !mag_db.is_nan(),
            gtu_max_db,
            unilateral_fom,
        }
    }
}

fn log_magnitude(value: Complex64) -> Value {
    let scale = value.re.abs().max(value.im.abs());
    if scale == 0.0 {
        Value::NEG_INFINITY
    } else {
        scale.log10() + (value.re / scale).hypot(value.im / scale).log10()
    }
}
