//! The integration method and its companion-model coefficients.
//!
//! See the module documentation on [`crate::numerics::integration`] for why
//! this is a numerics primitive rather than an analysis one.

use crate::Value;

/// Numerical integration methods for transient analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "veriloga", derive(serde::Serialize, serde::Deserialize))]
pub enum IntegrationMethod {
    /// Backward Euler (first order, very stable)
    BackwardEuler,
    /// Trapezoidal rule (second order, A-stable)
    Trapezoidal,
    /// Gear order 2 (BDF2, good for stiff systems)
    Gear2,
    /// TrapGear: Hybrid method that auto-switches between Trapezoidal and Gear2
    /// Uses Trapezoidal for smooth regions (better accuracy) and
    /// switches to Gear2 at discontinuities/oscillations (better stability)
    TrapGear,
}

/// Companion model coefficients for numerical integration
///
/// Each integration method converts differential elements (C, L) into
/// equivalent conductances and current/voltage sources using these coefficients.
#[derive(Debug, Clone, Copy)]
pub struct CompanionCoefficients {
    /// Equivalent conductance coefficient: G_eq = coeff_g * C / dt
    pub coeff_g: Value,
    /// History coefficient for v_n (most recent)
    pub coeff_v_n: Value,
    /// History coefficient for v_{n-1}
    pub coeff_v_n_minus_1: Value,
    /// Whether v_{n-1} history is needed
    pub needs_two_history: bool,
    /// Scale applied to the most recent conjugate-variable derivative.
    ///
    /// This is zero for backward Euler and Gear, one for the ordinary
    /// trapezoidal rule, and `xmu / (1 - xmu)` for ngspice's damped
    /// modified-trapezoidal corrector.
    pub coeff_i_n: Value,
}

impl CompanionCoefficients {
    /// Get coefficients for Backward Euler (first order, unconditionally stable)
    ///
    /// C·dv/dt = i  →  C·(v_{n+1} - v_n)/dt = i_{n+1}
    /// Companion: G_eq = C/dt, I_eq = G_eq·v_n
    #[inline]
    pub fn backward_euler() -> Self {
        Self {
            coeff_g: 1.0,
            coeff_v_n: 1.0,
            coeff_v_n_minus_1: 0.0,
            needs_two_history: false,
            coeff_i_n: 0.0,
        }
    }

    /// Get coefficients for Trapezoidal rule (second order, A-stable)
    ///
    /// Uses average of derivatives at n and n+1:
    /// C·(v_{n+1} - v_n)/dt = 0.5·(i_{n+1} + i_n)
    /// Companion: G_eq = 2C/dt, I_eq = G_eq·v_n + i_n
    #[inline]
    pub fn trapezoidal() -> Self {
        Self {
            coeff_g: 2.0,
            coeff_v_n: 2.0,
            coeff_v_n_minus_1: 0.0,
            needs_two_history: false,
            coeff_i_n: 1.0,
        }
    }

    /// Get ngspice's modified-trapezoidal order-two coefficients.
    ///
    /// `NIcomCof` defines `ag0 = 1 / (dt * (1 - xmu))` and
    /// `ag1 = xmu / (1 - xmu)`, while `NIintegrate` forms
    /// `qdot = ag0 * (q - q_prev) - ag1 * qdot_prev`. The parser guarantees
    /// the documented interpolation domain `0 <= xmu <= 0.5`.
    ///
    /// Returning `None` for an invalid programmatic value keeps this primitive
    /// fail-closed even when a caller constructs a netlist without going
    /// through the parser's validation.
    #[inline]
    pub(crate) fn trapezoidal_with_xmu(xmu: Value) -> Option<Self> {
        if !xmu.is_finite() || !(0.0..=0.5).contains(&xmu) {
            return None;
        }
        let denominator = 1.0 - xmu;
        let gain = 1.0 / denominator;
        Some(Self {
            coeff_g: gain,
            coeff_v_n: gain,
            coeff_v_n_minus_1: 0.0,
            needs_two_history: false,
            coeff_i_n: xmu / denominator,
        })
    }

    /// Get coefficients for Gear2/BDF2 (second order, L-stable, good for stiff)
    ///
    /// Uses backward difference formula:
    /// (3·v_{n+1} - 4·v_n + v_{n-1}) / (2·dt) = f_{n+1}
    /// Companion: G_eq = 3C/(2·dt), I_eq = (4C·v_n - C·v_{n-1})/(2·dt)
    #[inline]
    pub(crate) fn gear2() -> Self {
        Self {
            coeff_g: 1.5,            // 3/2
            coeff_v_n: 2.0,          // 4/2 = 2
            coeff_v_n_minus_1: -0.5, // -1/2
            needs_two_history: true,
            coeff_i_n: 0.0,
        }
    }

    /// Get variable-step Gear2/BDF2 coefficients.
    ///
    /// For the current step `h` and the previously accepted step `h_prev`,
    /// differentiating the quadratic interpolant through the current and two
    /// preceding solution points gives
    ///
    /// `x' = (a0*x[n+1] - a1*x[n] - a2*x[n-1]) / h`,
    ///
    /// where, for `r = h / h_prev`, `a0 = (1 + 2r)/(1 + r)`,
    /// `a1 = 1 + r`, and `a2 = -r^2/(1 + r)`. The equal-step case therefore
    /// reduces exactly to the ordinary `(3/2, 2, -1/2)` BDF2 coefficients.
    #[inline]
    pub(crate) fn gear2_variable_step(dt: Value, previous_dt: Value) -> Self {
        if !dt.is_finite() || dt <= 0.0 || !previous_dt.is_finite() || previous_dt <= 0.0 {
            return Self::backward_euler();
        }

        let ratio = dt / previous_dt;
        if !ratio.is_finite() || ratio <= 0.0 {
            return Self::backward_euler();
        }
        let denominator = 1.0 + ratio;
        let coeff_g = (1.0 + 2.0 * ratio) / denominator;
        let coeff_v_n = 1.0 + ratio;
        let coeff_v_n_minus_1 = -(ratio * ratio) / denominator;
        if !coeff_g.is_finite() || !coeff_v_n.is_finite() || !coeff_v_n_minus_1.is_finite() {
            return Self::backward_euler();
        }

        Self {
            coeff_g,
            coeff_v_n,
            coeff_v_n_minus_1,
            needs_two_history: true,
            coeff_i_n: 0.0,
        }
    }

    /// Get coefficients for the specified integration method
    #[inline]
    pub(crate) fn for_method(method: IntegrationMethod) -> Self {
        match method {
            IntegrationMethod::BackwardEuler => Self::backward_euler(),
            IntegrationMethod::Trapezoidal => Self::trapezoidal(),
            IntegrationMethod::Gear2 => Self::gear2(),
            IntegrationMethod::TrapGear => Self::trapezoidal(), // Default, actual method chosen dynamically
        }
    }

    /// Get coefficients for a method using the accepted timestep history.
    #[inline]
    pub(crate) fn for_method_with_previous_step(
        method: IntegrationMethod,
        dt: Value,
        previous_dt: Value,
    ) -> Self {
        match method {
            IntegrationMethod::Gear2 => Self::gear2_variable_step(dt, previous_dt),
            _ => Self::for_method(method),
        }
    }

    /// Calculate equivalent conductance for a capacitor
    #[inline]
    pub(crate) fn capacitor_geq(&self, capacitance: Value, dt: Value) -> Value {
        self.coeff_g * capacitance / dt
    }

    /// Calculate equivalent current source for a capacitor
    /// v_n is current voltage, v_n_minus_1 is previous voltage (for Gear2)
    #[inline]
    pub(crate) fn capacitor_ieq(
        &self,
        capacitance: Value,
        dt: Value,
        v_n: Value,
        v_n_minus_1: Value,
        i_n: Value,
    ) -> Value {
        let mut ieq = self.coeff_v_n * capacitance * v_n / dt;
        if self.needs_two_history {
            ieq += self.coeff_v_n_minus_1 * capacitance * v_n_minus_1 / dt;
        }
        if self.coeff_i_n != 0.0 {
            ieq += self.coeff_i_n * i_n;
        }
        ieq
    }

    /// Evaluate capacitor current from voltage differences before timestep
    /// scaling. This avoids subtracting large absolute Norton companions.
    pub(crate) fn capacitor_current(
        &self,
        capacitance: Value,
        dt: Value,
        voltage: Value,
        voltage_prev: Value,
        voltage_prev_prev: Value,
        current_prev: Value,
    ) -> Value {
        let mut difference = self.coeff_g * (voltage - voltage_prev);
        if self.needs_two_history {
            difference += self.coeff_v_n_minus_1 * (voltage_prev - voltage_prev_prev);
        }
        let scale = capacitance / dt;
        let mut current = if capacitance == 0.0 || difference == 0.0 {
            0.0
        } else if scale.is_finite() && scale != 0.0 {
            scale * difference
        } else {
            capacitance * (difference / dt)
        };
        if self.coeff_i_n != 0.0 {
            current -= self.coeff_i_n * current_prev;
        }
        current
    }

    /// Calculate equivalent resistance for an inductor
    #[inline]
    pub(crate) fn inductor_req(&self, inductance: Value, dt: Value) -> Value {
        self.coeff_g * inductance / dt
    }

    /// Evaluate the linear flux derivative from current differences before
    /// inductance/timestep scaling. Subtracting separately rounded L*i
    /// samples can create a false voltage in a perfectly coupled null mode.
    /// This is the capacitor charge derivative with dual physical units;
    /// callers add the separate previous-voltage contribution afterward.
    #[inline]
    pub(crate) fn inductor_charge_derivative_correction(
        &self,
        inductance: Value,
        dt: Value,
        current: Value,
        current_prev: Value,
        current_prev_prev: Value,
    ) -> Value {
        let derivative = self.capacitor_current(
            inductance,
            dt,
            current,
            current_prev,
            current_prev_prev,
            0.0,
        );
        if derivative.is_finite() {
            return derivative;
        }
        // Opposite finite currents can overflow their difference even when
        // the final flux derivative fits. Keep that cold path scaled.
        use rspice_veriloga_runtime::arithmetic::ScaledValue as Scaled;
        let difference = |a, b| {
            Scaled::product_sum(
                Scaled::new(a),
                Scaled::new(1.0),
                Scaled::new(b),
                Scaled::new(-1.0),
            )
        };
        let mut derivative = difference(current, current_prev).multiply(Scaled::new(self.coeff_g));
        if self.needs_two_history {
            derivative = Scaled::product_sum(
                derivative,
                Scaled::new(1.0),
                difference(current_prev, current_prev_prev),
                Scaled::new(self.coeff_v_n_minus_1),
            );
        }
        derivative
            .multiply(Scaled::new(inductance))
            .divide(Scaled::new(dt))
            .binary64()
    }

    /// Calculate the equivalent voltage-source magnitude for an inductor.
    ///
    /// Exact dual of `Self::capacitor_ieq` (v <-> i, C <-> L): the i_n
    /// history term uses `coeff_v_n` (NOT `coeff_g` — they differ for Gear2),
    /// the i_{n-1} term applies only when the method keeps two history points,
    /// and the conjugate-variable history v_n is weighted by `coeff_i_n`
    /// (one for ordinary trapezoidal, below one for modified trapezoidal,
    /// zero for BE/Gear2).
    ///
    /// The branch row is stamped as `v(np) - v(nn) - R_eq*i_{n+1} = -V_eq`,
    /// i.e. the stamp site negates this value, yielding:
    ///   BE:   v_{n+1} = (L/dt)*(i_{n+1} - i_n)
    ///   Trap: v_{n+1} = (2L/dt)*(i_{n+1} - i_n) - v_n
    ///   BDF2: v_{n+1} = (L/dt)*(1.5*i_{n+1} - 2*i_n + 0.5*i_{n-1})
    ///
    /// The previous formulation (`coeff_g*L*i_n/dt + v_n`, stamped without
    /// negation) made the companion recursion non-contractive: on a plain RL
    /// deck the branch state alternated sign each step, the TrapGear
    /// controller read that as ringing, and the error compounded ~2x per
    /// accepted step until node voltages crossed the +-1 kV sanity clamp and
    /// the stepper death-spiraled at femtosecond dt. See the
    /// `inductor_transient` integration tests for the analytic pins.
    #[inline]
    pub fn inductor_veq(
        &self,
        inductance: Value,
        dt: Value,
        i_n: Value,
        i_n_minus_1: Value,
        v_n: Value,
    ) -> Value {
        let mut veq = self.coeff_v_n * inductance * i_n / dt;
        if self.needs_two_history {
            veq += self.coeff_v_n_minus_1 * inductance * i_n_minus_1 / dt;
        }
        if self.coeff_i_n != 0.0 {
            veq += self.coeff_i_n * v_n;
        }
        veq
    }
}

/// Decode an authored integration-method spelling.
///
/// Deck text names a method in several dialects: SPICE's `TRAP`/`GEAR`, Xyce's
/// numeric `.OPTIONS TIMEINT METHOD=7|8`, and the hybrid RSpice defaults to.
/// The table lives beside the enum rather than in a parser or in the engine
/// facade: a spelling-to-variant map is data about this enum, so every layer
/// that reads a method name reads down into it instead of sideways.
pub(crate) fn parse_integration_method(spelling: &str) -> Option<IntegrationMethod> {
    if spelling.eq_ignore_ascii_case("TRAP")
        || spelling.eq_ignore_ascii_case("TRAPEZOIDAL")
        || spelling.eq_ignore_ascii_case("TRAPEZOID")
        || spelling.eq_ignore_ascii_case("ONESTEP")
        || spelling == "7"
    {
        Some(IntegrationMethod::Trapezoidal)
    } else if spelling.eq_ignore_ascii_case("EULER")
        || spelling.eq_ignore_ascii_case("BE")
        || spelling.eq_ignore_ascii_case("BACKWARDEULER")
    {
        Some(IntegrationMethod::BackwardEuler)
    } else if spelling.eq_ignore_ascii_case("GEAR")
        || spelling.eq_ignore_ascii_case("BDF")
        || spelling.eq_ignore_ascii_case("GEAR2")
        || spelling == "8"
    {
        Some(IntegrationMethod::Gear2)
    } else if spelling.eq_ignore_ascii_case("TRAPGEAR") || spelling.eq_ignore_ascii_case("AUTO") {
        Some(IntegrationMethod::TrapGear)
    } else {
        None
    }
}

#[cfg(test)]
mod companion_coefficients_tests {
    use super::*;

    #[test]
    fn centered_capacitor_current_preserves_affine_history_and_extreme_scales() {
        for coefficients in [
            CompanionCoefficients::backward_euler(),
            CompanionCoefficients::trapezoidal(),
            CompanionCoefficients::gear2_variable_step(2.0, 1.0),
            CompanionCoefficients::trapezoidal_with_xmu(0.49).unwrap(),
        ] {
            let current = coefficients.capacitor_current(0.25, 2.0, 13.0, 7.0, 4.0, 0.75);
            assert!((current - 0.75).abs() <= 4.0 * Value::EPSILON);
        }
        for scale in [2.0_f64.powi(-900), 2.0_f64.powi(900)] {
            for voltage in [2.0_f64.powi(-600), 2.0_f64.powi(600)] {
                assert_eq!(
                    CompanionCoefficients::backward_euler()
                        .capacitor_current(scale, scale, voltage, 0.0, 0.0, 0.0),
                    voltage
                );
            }
        }
        assert_eq!(
            CompanionCoefficients::trapezoidal()
                .capacitor_current(1e200, 1e-200, 1.0, 1.0, 1.0, 0.125),
            -0.125
        );
    }

    #[test]
    fn linear_flux_derivative_retains_one_ulp_current_changes() {
        let previous = 0.5_f64;
        let current = previous.next_up();
        // The exact current increment is 2^-53 and dt is 1/8. Thus the
        // one-step derivatives are a0*L*2^-50, with no rounded flux oracle.
        for (coefficients, a0) in [
            (CompanionCoefficients::backward_euler(), 1.0),
            (CompanionCoefficients::trapezoidal(), 2.0),
            (CompanionCoefficients::gear2(), 1.5),
        ] {
            for inductance in [3.0, 9.0, -3.0, -9.0] {
                let voltage = coefficients.inductor_charge_derivative_correction(
                    inductance, 0.125, current, previous, previous,
                );
                assert_eq!(voltage, a0 * inductance * 2.0_f64.powi(-50));
            }
            let huge = 2.0_f64.powi(1023);
            assert_eq!(
                coefficients.inductor_charge_derivative_correction(
                    2.0_f64.powi(-1020),
                    1.0,
                    huge,
                    -huge,
                    -huge,
                ),
                16.0 * a0,
            );
            // Two finite current changes in a perfect transformer's null
            // flux mode must not acquire an artificial voltage.
            let delta = 2.0_f64.powi(-48);
            let self_voltage = coefficients.inductor_charge_derivative_correction(
                1.0,
                0.125,
                0.5 + 3.0 * delta,
                0.5,
                0.5,
            );
            let mutual_voltage = coefficients.inductor_charge_derivative_correction(
                3.0,
                0.125,
                0.25 - delta,
                0.25,
                0.25,
            );
            assert_eq!(self_voltage + mutual_voltage, 0.0);
        }
    }

    #[test]
    fn variable_step_gear2_reduces_to_fixed_bdf2_for_equal_steps() {
        let variable = CompanionCoefficients::gear2_variable_step(2.0, 2.0);
        let fixed = CompanionCoefficients::gear2();

        assert_eq!(variable.coeff_g, fixed.coeff_g);
        assert_eq!(variable.coeff_v_n, fixed.coeff_v_n);
        assert_eq!(variable.coeff_v_n_minus_1, fixed.coeff_v_n_minus_1);
        assert_eq!(variable.needs_two_history, fixed.needs_two_history);
        assert_eq!(variable.coeff_i_n, fixed.coeff_i_n);
    }

    #[test]
    fn variable_step_gear2_differentiates_an_affine_history_exactly() {
        let dt = 2.0;
        let previous_dt = 1.0;
        let slope = 3.0;
        let offset = 7.0;
        let inductance = 0.25;
        let i_prev_prev = offset - slope * previous_dt;
        let i_prev = offset;
        let i_curr = offset + slope * dt;
        let coefficients = CompanionCoefficients::gear2_variable_step(dt, previous_dt);

        assert!((coefficients.coeff_g - 5.0 / 3.0).abs() <= Value::EPSILON);
        assert!((coefficients.coeff_v_n - 3.0).abs() <= Value::EPSILON);
        assert!((coefficients.coeff_v_n_minus_1 + 4.0 / 3.0).abs() <= Value::EPSILON);

        let voltage = coefficients.inductor_req(inductance, dt) * i_curr
            - coefficients.inductor_veq(inductance, dt, i_prev, i_prev_prev, 0.0);
        assert!((voltage - inductance * slope).abs() <= 8.0 * Value::EPSILON);
    }

    #[test]
    fn variable_step_gear2_fails_safe_without_valid_step_history() {
        let backward_euler = CompanionCoefficients::backward_euler();

        for invalid_previous_dt in [0.0, -1.0, Value::NAN, Value::INFINITY] {
            let coefficients = CompanionCoefficients::gear2_variable_step(1.0, invalid_previous_dt);
            assert_eq!(coefficients.coeff_g, backward_euler.coeff_g);
            assert_eq!(coefficients.coeff_v_n, backward_euler.coeff_v_n);
            assert_eq!(
                coefficients.coeff_v_n_minus_1,
                backward_euler.coeff_v_n_minus_1
            );
            assert_eq!(
                coefficients.needs_two_history,
                backward_euler.needs_two_history
            );
        }
    }

    #[test]
    fn modified_trapezoidal_coefficients_match_ngspice_endpoints_and_damping() {
        let backward_euler_endpoint =
            CompanionCoefficients::trapezoidal_with_xmu(0.0).expect("XMU=0 is valid");
        assert_eq!(backward_euler_endpoint.coeff_g, 1.0);
        assert_eq!(backward_euler_endpoint.coeff_v_n, 1.0);
        assert_eq!(backward_euler_endpoint.coeff_i_n, 0.0);

        let damped = CompanionCoefficients::trapezoidal_with_xmu(0.49).expect("XMU=0.49 is valid");
        assert_eq!(damped.coeff_g, 1.0 / 0.51);
        assert_eq!(damped.coeff_v_n, 1.0 / 0.51);
        assert_eq!(damped.coeff_i_n, 0.49 / 0.51);
        assert_eq!(
            damped.capacitor_ieq(2.0, 0.25, 3.0, 0.0, 5.0),
            (1.0 / 0.51) * 2.0 * 3.0 / 0.25 + (0.49 / 0.51) * 5.0
        );
        assert_eq!(
            damped.inductor_veq(2.0, 0.25, 3.0, 0.0, 5.0),
            (1.0 / 0.51) * 2.0 * 3.0 / 0.25 + (0.49 / 0.51) * 5.0
        );

        let standard = CompanionCoefficients::trapezoidal_with_xmu(0.5).expect("XMU=0.5 is valid");
        let canonical = CompanionCoefficients::trapezoidal();
        assert_eq!(standard.coeff_g, canonical.coeff_g);
        assert_eq!(standard.coeff_v_n, canonical.coeff_v_n);
        assert_eq!(standard.coeff_i_n, canonical.coeff_i_n);
    }

    #[test]
    fn modified_trapezoidal_coefficients_reject_invalid_programmatic_values() {
        for invalid in [
            -Value::MIN_POSITIVE,
            0.500_000_000_000_000_1,
            Value::NAN,
            Value::INFINITY,
        ] {
            assert!(CompanionCoefficients::trapezoidal_with_xmu(invalid).is_none());
        }
    }

    #[test]
    fn every_accepted_method_spelling_selects_its_variant() {
        for spelling in ["TRAP", "trapezoidal", "TrapeZoid", "onestep", "7"] {
            assert_eq!(
                parse_integration_method(spelling),
                Some(IntegrationMethod::Trapezoidal),
                "{spelling}"
            );
        }
        for spelling in ["EULER", "be", "BackwardEuler"] {
            assert_eq!(
                parse_integration_method(spelling),
                Some(IntegrationMethod::BackwardEuler),
                "{spelling}"
            );
        }
        for spelling in ["GEAR", "bdf", "Gear2", "8"] {
            assert_eq!(
                parse_integration_method(spelling),
                Some(IntegrationMethod::Gear2),
                "{spelling}"
            );
        }
        for spelling in ["TRAPGEAR", "auto"] {
            assert_eq!(
                parse_integration_method(spelling),
                Some(IntegrationMethod::TrapGear),
                "{spelling}"
            );
        }
    }

    #[test]
    fn an_unknown_method_spelling_selects_nothing() {
        for spelling in ["", "simpson", "6", "9", "trap gear"] {
            assert_eq!(parse_integration_method(spelling), None, "{spelling}");
        }
    }
}
