//! Junction diode, depletion-charge, and numerical primitive helpers.

use super::*;

impl Bjt {
    /// Current and its exact voltage derivative using VBIC 1.3 expLinA.
    /// Older families keep their existing junction law.
    pub(in crate::device::semiconductor::bjt) fn vbic_diode_iv(
        &self,
        isat: Value,
        v: Value,
        n: Value,
        limit: Value,
    ) -> (Value, Value) {
        if !self.vbic_13 {
            return self.diode_iv_with_is(isat, v, n);
        }
        if isat <= 0.0 {
            return (0.0, 0.0);
        }
        let nvt = n * self.vt;
        let (current, conductance) = Self::vbic_scaled_exp_lina(isat, v, nvt, limit);
        if v < limit && (v / nvt).abs() < 0.5 {
            (isat * (v / nvt).exp_m1(), conductance)
        } else {
            (current - isat, conductance)
        }
    }

    pub(in crate::device::semiconductor::bjt) fn vbic_scaled_exp_lina(
        isat: Value,
        v: Value,
        nvt: Value,
        limit: Value,
    ) -> (Value, Value) {
        let arg = v.min(limit) / nvt;
        // Combine the scale in log space only when exp(arg) would overflow
        // before multiplication by a small saturation current.
        let exponential_current = if arg > 700.0 {
            (isat.ln() + arg).exp()
        } else {
            isat * arg.exp()
        };
        let conductance = exponential_current / nvt;
        if v < limit {
            (exponential_current, conductance)
        } else {
            (exponential_current + conductance * (v - limit), conductance)
        }
    }

    /// Get polarity multiplier (+1 for NPN, -1 for PNP)
    pub(in crate::device::semiconductor::bjt) fn polarity(&self) -> Value {
        match self.bjt_type {
            BjtType::Npn => 1.0,
            BjtType::Pnp => -1.0,
        }
    }

    pub(in crate::device::semiconductor::bjt) fn diode_iv_with_is(
        &self,
        isat: Value,
        v: Value,
        n: Value,
    ) -> (Value, Value) {
        let nvt = n * self.vt;
        if !isat.is_finite() || isat <= 0.0 || !v.is_finite() || !nvt.is_finite() || nvt <= 0.0 {
            return (0.0, 0.0);
        }

        if self.charge_model == BjtChargeModel::LegacyGummelPoon && v >= -3.0 * nvt {
            let argument = v / nvt;
            // Low-temperature GP junctions can exceed 80 thermal voltages
            // at ordinary currents. A numerical cap changes their physics;
            // combine the saturation current with exp before range checks.
            let exponential = crate::numerics::scaled_exp_product(&[isat], &[], argument);
            let current = if argument.abs() < 0.5 {
                isat * argument.exp_m1()
            } else {
                exponential - isat
            };
            let conductance = if exponential.is_normal() {
                exponential / nvt
            } else {
                crate::numerics::scaled_exp_product(&[isat], &[nvt], argument)
            };
            return (current, conductance);
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

        // VBIC retains the exponential under reverse bias; the cubic
        // continuation below belongs only to the Gummel-Poon model.
        if self.charge_model == BjtChargeModel::Vbic || v >= -3.0 * nvt {
            let exp_v = (v / nvt).exp();
            return (isat * (exp_v - 1.0), isat * exp_v / nvt);
        }

        let arg = 3.0 * nvt / (v * std::f64::consts::E);
        let arg3 = arg * arg * arg;
        (-isat * (1.0 + arg3), isat * 3.0 * arg3 / v)
    }

    /// Diode current with the selected model's reverse-bias law.
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

    /// The VBIC 1.3 physical floor is already applied before M scaling;
    /// a second numerical floor here would corrupt large parallel instances.
    #[inline]
    pub(in crate::device::semiconductor::bjt) fn guarded_series_resistance(
        &self,
        resistance: Value,
    ) -> Value {
        if self.vbic_13 {
            resistance
        } else {
            resistance.max(1e-12)
        }
    }

    /// High-injection power and derivative with respect to its argument.
    /// VBIC 1.3 specifies a 1e-8 floor in both qb and qbp; the derivative
    /// of the floored contribution is zero. Retain the older model's
    /// numerical guard without applying the 1.3 floor to that family.
    pub(in crate::device::semiconductor::bjt) fn vbic_high_injection_power(
        &self,
        argument: Value,
        exponent: Value,
    ) -> (Value, Value) {
        let floor = if self.vbic_13 { 1e-8 } else { 1e-18 };
        if argument > floor {
            let value = argument.powf(exponent);
            (value, exponent * value / argument)
        } else {
            (floor.powf(exponent), 0.0)
        }
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn vbic_general_exp(
        &self,
        arg: Value,
    ) -> (Value, Value) {
        if !self.vbic_13 {
            return Self::limited_exp(arg);
        }
        let limit = self.vbic_maxexp.ln();
        if arg < limit {
            let value = arg.exp();
            (value, value)
        } else {
            (self.vbic_maxexp * (1.0 + arg - limit), self.vbic_maxexp)
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn vbic13_pnjmaxi_preserves_zero_and_subnormal_saturation_currents() {
        for isat in [0.0, 1e-40, 1e-320] {
            let bjt = Bjt::new_npn("q".into(), 1, 2, 0).with_params(&HashMap::from([
                ("LEVEL".into(), 11.0),
                ("IS".into(), isat),
                ("PNJMAXI".into(), 1e-6),
            ]));
            let voltage = bjt.vbic_junction_limits.ifi + bjt.vt;
            let state = bjt.vbic_transport_charge_state(voltage, 0.0);
            if isat == 0.0 {
                assert_eq!((state.ifi, state.gfi), (0.0, 0.0));
                continue;
            }
            // One thermal voltage past the 1uA transition, the continued
            // current is 2uA, even when exp(V/VT) alone would overflow.
            assert!((state.ifi - 2e-6).abs() < 1e-17, "IS={isat}: {}", state.ifi);
            assert!((state.gfi * bjt.vt - 1e-6).abs() < 1e-17);
            let h = 1e-6;
            let fd = (bjt.vbic_transport_charge_state(voltage + h, 0.0).ifi
                - bjt.vbic_transport_charge_state(voltage - h, 0.0).ifi)
                / (2.0 * h);
            assert!((fd - state.gfi).abs() < 1e-8 * state.gfi);
        }
    }
}
