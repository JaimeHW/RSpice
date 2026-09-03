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

    /// Calculate equivalent resistance for an inductor
    #[inline]
    pub(crate) fn inductor_req(&self, inductance: Value, dt: Value) -> Value {
        self.coeff_g * inductance / dt
    }

    /// Evaluate the charge-history part of an inductor Newton correction in
    /// DAE form.
    ///
    /// Forming `Q=L*i` at each state and differencing those charges before the
    /// timestep division mirrors production SPICE time integrators. It avoids
    /// constructing and then cancelling the much larger absolute companion
    /// terms `R_eq*i`, while keeping the arithmetic order of the underlying
    /// DAE instead of relying on an algebraically equivalent fused expression.
    #[inline]
    pub(crate) fn inductor_charge_derivative_correction(
        &self,
        inductance: Value,
        dt: Value,
        current: Value,
        current_prev: Value,
        current_prev_prev: Value,
    ) -> Value {
        let charge = inductance * current;
        let charge_prev = inductance * current_prev;
        let first_difference = charge - charge_prev;
        if self.needs_two_history {
            let charge_prev_prev = inductance * current_prev_prev;
            let previous_difference = charge_prev - charge_prev_prev;
            (self.coeff_g * first_difference + self.coeff_v_n_minus_1 * previous_difference) / dt
        } else {
            self.coeff_g * (first_difference / dt)
        }
    }

    /// Calculate the equivalent voltage-source magnitude for an inductor.
    ///
    /// Exact dual of [`Self::capacitor_ieq`] (v <-> i, C <-> L): the i_n
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

#[cfg(test)]
mod companion_coefficients_tests {
    use super::*;

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

/// Decode an authored integration-method spelling.
///
/// Deck text names a method in several dialects: SPICE's `TRAP`/`GEAR`, Xyce's
/// numeric `.OPTIONS TIMEINT METHOD=7|8`, and the hybrid RSpice defaults to.
/// The table lives beside the enum rather than in a parser or in the engine
/// facade: a spelling-to-variant map is data about this enum, so every layer
/// that reads a method name reads down into it instead of sideways.
pub fn parse_integration_method(spelling: &str) -> Option<IntegrationMethod> {
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
