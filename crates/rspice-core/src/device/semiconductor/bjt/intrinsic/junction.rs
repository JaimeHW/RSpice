//! Junction diode, depletion-charge, and numerical primitive helpers.

use super::*;

impl Bjt {
    /// Get polarity multiplier (+1 for NPN, -1 for PNP)
    pub(in crate::device::semiconductor::bjt) fn polarity(&self) -> Value {
        match self.bjt_type {
            BjtType::Npn => 1.0,
            BjtType::Pnp => -1.0,
        }
    }

    fn diode_iv_with_is(&self, isat: Value, v: Value, n: Value) -> (Value, Value) {
        let nvt = n * self.vt;
        if !isat.is_finite() || isat <= 0.0 || !v.is_finite() || !nvt.is_finite() || nvt <= 0.0 {
            return (0.0, 0.0);
        }

        let v_forward = 80.0 * nvt;
        if v > v_forward {
            let exp_forward = (v_forward / nvt).exp();
            let current_forward = isat * (exp_forward - 1.0);
            let conductance_forward = isat * exp_forward / nvt;
            return (
                current_forward + conductance_forward * (v - v_forward),
                conductance_forward,
            );
        }

        if v >= -3.0 * nvt {
            let exp_v = (v / nvt).exp();
            return (isat * (exp_v - 1.0), isat * exp_v / nvt);
        }

        let arg = 3.0 * nvt / (v * std::f64::consts::E);
        let arg3 = arg * arg * arg;
        (-isat * (1.0 + arg3), isat * 3.0 * arg3 / v)
    }

    /// Diode current using ngspice's BJT reverse-bias continuation.
    pub(in crate::device::semiconductor::bjt) fn diode_current_with_is(
        &self,
        isat: Value,
        v: Value,
        n: Value,
    ) -> Value {
        self.diode_iv_with_is(isat, v, n).0
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn diode_current(&self, v: Value, n: Value) -> Value {
        self.diode_current_with_is(self.is, v, n)
    }

    /// Diode conductance using the exact derivative of `diode_current_with_is`.
    pub(in crate::device::semiconductor::bjt) fn diode_conductance_with_is(
        &self,
        isat: Value,
        v: Value,
        n: Value,
    ) -> Value {
        self.diode_iv_with_is(isat, v, n).1
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn diode_conductance(
        &self,
        v: Value,
        n: Value,
    ) -> Value {
        self.diode_conductance_with_is(self.is, v, n)
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn depletion_charge_base(
        potential: Value,
        grading: Value,
        scaled_voltage: Value,
    ) -> Value {
        let phi = potential.max(1e-12);
        let exponent = 1.0 - grading;
        let one_minus = (1.0 - scaled_voltage / phi).max(1e-18);
        if exponent.abs() < 1e-12 {
            -phi * one_minus.ln()
        } else {
            phi * (1.0 - one_minus.powf(exponent)) / exponent
        }
    }

    pub(in crate::device::semiconductor::bjt) fn depletion_capacitance_factor(
        potential: Value,
        grading: Value,
        scaled_voltage: Value,
    ) -> Value {
        let phi = potential.max(1e-12);
        let one_minus = (1.0 - scaled_voltage / phi).max(1e-18);
        if (1.0 - grading).abs() < 1e-12 {
            1.0 / one_minus
        } else {
            one_minus.powf(-grading)
        }
    }

    pub(in crate::device::semiconductor::bjt) fn vbic_depletion_charge_and_derivative(
        &self,
        junction_voltage_eff: Value,
        potential: Value,
        grading: Value,
        forward_coeff: Value,
        smoothing: Value,
    ) -> (Value, Value) {
        let phi = potential.max(1e-12);
        let fc = forward_coeff.clamp(0.0, 0.999_999);

        if smoothing > 0.0 {
            let dv0 = -phi * fc;
            let mv0 = (dv0 * dv0 + 4.0 * smoothing * smoothing).sqrt();
            let vl0 = -0.5 * (dv0 + mv0);
            let q0 = -Self::depletion_charge_base(phi, grading, vl0);

            let dv = junction_voltage_eff + dv0;
            let mv = (dv * dv + 4.0 * smoothing * smoothing).sqrt();
            let dmv_dv = dv / mv.max(1e-18);
            let vl = 0.5 * (dv - mv) - dv0;
            let dvl_dv = 0.5 * (1.0 - dmv_dv);

            let qlo = -Self::depletion_charge_base(phi, grading, vl);
            let dqlo_dvl = Self::depletion_capacitance_factor(phi, grading, vl);
            let linear_gain = (1.0 - fc).max(1e-18).powf(-grading);
            let charge = qlo + linear_gain * (junction_voltage_eff - vl + vl0) - q0;
            let derivative = dqlo_dvl * dvl_dv + linear_gain * (1.0 - dvl_dv);
            return (charge, derivative.max(0.0));
        }

        let dv0 = -phi * fc;
        let dvh = junction_voltage_eff + dv0;
        if dvh > 0.0 {
            let one_minus_fc = (1.0 - fc).max(1e-18);
            let pwq = one_minus_fc.powf(-1.0 - grading);
            let qlo = Self::depletion_charge_base(phi, grading, phi * fc);
            let charge = qlo + dvh * (one_minus_fc + 0.5 * grading * dvh / phi) * pwq;
            let derivative = pwq * (one_minus_fc + grading * dvh / phi);
            return (charge, derivative.max(0.0));
        }

        let charge = Self::depletion_charge_base(phi, grading, junction_voltage_eff);
        let derivative = Self::depletion_capacitance_factor(phi, grading, junction_voltage_eff);
        (charge, derivative.max(0.0))
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn series_active(resistance: Value) -> bool {
        resistance.is_finite() && resistance > 0.0
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn limited_exp(arg: Value) -> (Value, Value) {
        let clamped = arg.clamp(-80.0, 80.0);
        let value = clamped.exp();
        let slope = if (arg - clamped).abs() < f64::EPSILON {
            value
        } else {
            0.0
        };
        (value, slope)
    }
}
