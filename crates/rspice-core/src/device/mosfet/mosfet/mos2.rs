use super::*;
use std::ops::{Add, Div, Mul, Neg, Sub};

const EPSSIL: Value = 11.7 * 8.854_214_871e-12;
const CHARGE: Value = 1.602_176_634e-19;

#[derive(Debug, Clone, Copy)]
pub(in crate::device::mosfet::mosfet) struct Mos2Evaluation {
    pub(in crate::device::mosfet::mosfet) id: Value,
    pub(in crate::device::mosfet::mosfet) region: MosRegion,
    pub(in crate::device::mosfet::mosfet) von: Value,
    pub(in crate::device::mosfet::mosfet) vdsat: Value,
}

#[derive(Debug, Clone, Copy)]
struct Mos2ForwardOperatingPoint {
    gm: Value,
    gds: Value,
    gmb: Value,
}

#[derive(Debug, Clone, Copy)]
struct Dual3 {
    value: Value,
    derivative: [Value; 3],
}

impl Dual3 {
    #[inline]
    fn constant(value: Value) -> Self {
        Self {
            value,
            derivative: [0.0; 3],
        }
    }

    #[inline]
    fn variable(value: Value, index: usize) -> Self {
        let mut derivative = [0.0; 3];
        derivative[index] = 1.0;
        Self { value, derivative }
    }

    #[inline]
    fn sqrt(self) -> Self {
        let value = self.value.sqrt();
        if value > 0.0 && value.is_finite() {
            self.map_unary(value, 0.5 / value)
        } else {
            Self::constant(value)
        }
    }

    #[inline]
    fn powf(self, exponent: Value) -> Self {
        let value = self.value.powf(exponent);
        if self.value > 0.0 && value.is_finite() {
            self.map_unary(value, exponent * self.value.powf(exponent - 1.0))
        } else {
            Self::constant(value)
        }
    }

    #[inline]
    fn exp(self) -> Self {
        let value = self.value.exp();
        if value.is_finite() {
            self.map_unary(value, value)
        } else {
            Self::constant(value)
        }
    }

    #[inline]
    fn max_const(self, floor: Value) -> Self {
        if self.value > floor {
            self
        } else {
            Self::constant(floor)
        }
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        if self.value >= other.value {
            self
        } else {
            other
        }
    }

    #[inline]
    fn map_unary(self, value: Value, scale: Value) -> Self {
        Self {
            value,
            derivative: [
                self.derivative[0] * scale,
                self.derivative[1] * scale,
                self.derivative[2] * scale,
            ],
        }
    }

    #[inline]
    fn sanitized_derivative(self, index: usize) -> Value {
        let value = self.derivative[index];
        if value.is_finite() { value } else { 0.0 }
    }
}

impl Add for Dual3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
            derivative: [
                self.derivative[0] + rhs.derivative[0],
                self.derivative[1] + rhs.derivative[1],
                self.derivative[2] + rhs.derivative[2],
            ],
        }
    }
}

impl Add<Value> for Dual3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Value) -> Self::Output {
        Self {
            value: self.value + rhs,
            derivative: self.derivative,
        }
    }
}

impl Add<Dual3> for Value {
    type Output = Dual3;

    #[inline]
    fn add(self, rhs: Dual3) -> Self::Output {
        rhs + self
    }
}

impl Sub for Dual3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
            derivative: [
                self.derivative[0] - rhs.derivative[0],
                self.derivative[1] - rhs.derivative[1],
                self.derivative[2] - rhs.derivative[2],
            ],
        }
    }
}

impl Sub<Value> for Dual3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Value) -> Self::Output {
        Self {
            value: self.value - rhs,
            derivative: self.derivative,
        }
    }
}

impl Sub<Dual3> for Value {
    type Output = Dual3;

    #[inline]
    fn sub(self, rhs: Dual3) -> Self::Output {
        Dual3 {
            value: self - rhs.value,
            derivative: [-rhs.derivative[0], -rhs.derivative[1], -rhs.derivative[2]],
        }
    }
}

impl Mul for Dual3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value * rhs.value,
            derivative: [
                self.derivative[0] * rhs.value + self.value * rhs.derivative[0],
                self.derivative[1] * rhs.value + self.value * rhs.derivative[1],
                self.derivative[2] * rhs.value + self.value * rhs.derivative[2],
            ],
        }
    }
}

impl Mul<Value> for Dual3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Value) -> Self::Output {
        Self {
            value: self.value * rhs,
            derivative: [
                self.derivative[0] * rhs,
                self.derivative[1] * rhs,
                self.derivative[2] * rhs,
            ],
        }
    }
}

impl Mul<Dual3> for Value {
    type Output = Dual3;

    #[inline]
    fn mul(self, rhs: Dual3) -> Self::Output {
        rhs * self
    }
}

impl Div for Dual3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        let denominator = rhs.value * rhs.value;
        Self {
            value: self.value / rhs.value,
            derivative: [
                (self.derivative[0] * rhs.value - self.value * rhs.derivative[0]) / denominator,
                (self.derivative[1] * rhs.value - self.value * rhs.derivative[1]) / denominator,
                (self.derivative[2] * rhs.value - self.value * rhs.derivative[2]) / denominator,
            ],
        }
    }
}

impl Div<Value> for Dual3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Value) -> Self::Output {
        Self {
            value: self.value / rhs,
            derivative: [
                self.derivative[0] / rhs,
                self.derivative[1] / rhs,
                self.derivative[2] / rhs,
            ],
        }
    }
}

impl Div<Dual3> for Value {
    type Output = Dual3;

    #[inline]
    fn div(self, rhs: Dual3) -> Self::Output {
        Dual3::constant(self) / rhs
    }
}

impl Neg for Dual3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            value: -self.value,
            derivative: [
                -self.derivative[0],
                -self.derivative[1],
                -self.derivative[2],
            ],
        }
    }
}

impl Mosfet {
    #[inline]
    pub(in crate::device::mosfet::mosfet) fn level2_effective_length(&self) -> Value {
        (self.l - 2.0 * self.ld).max(1.0e-12)
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn level2_model_space_onset_voltage(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> Value {
        self.level2_evaluate(vgs, vds, vbs).von
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn level2_operating_point(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, MosRegion, Value, Value, Value) {
        let eval = self.level2_evaluate(vgs, vds, vbs);
        let p = self.polarity();
        let vgs_m = p * vgs;
        let vds_m = p * vds;
        let vbs_m = p * vbs;
        let vbd_m = vbs_m - vds_m;
        let vgd_m = vgs_m - vds_m;
        let mode = if vds_m >= 0.0 { 1.0 } else { -1.0 };
        let lvds = mode * vds_m;
        let lvbs = if mode > 0.0 { vbs_m } else { vbd_m };
        let lvgs = if mode > 0.0 { vgs_m } else { vgd_m };

        let forward = self.level2_forward_operating_point(lvgs, lvds, lvbs);
        let (gm, gds, gmb) = if mode > 0.0 {
            (forward.gm, forward.gds, forward.gmb)
        } else {
            (
                -forward.gm,
                forward.gm + forward.gds + forward.gmb,
                -forward.gmb,
            )
        };
        let sanitize = |value: Value| if value.is_finite() { value } else { 0.0 };

        (
            eval.id,
            eval.region,
            sanitize(gm),
            sanitize(gds),
            sanitize(gmb),
        )
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn level2_evaluate(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> Mos2Evaluation {
        if !vgs.is_finite() || !vds.is_finite() || !vbs.is_finite() {
            return Mos2Evaluation {
                id: 0.0,
                region: MosRegion::Cutoff,
                von: self.polarity() * self.vto,
                vdsat: 0.0,
            };
        }

        let p = self.polarity();
        let vgs_m = p * vgs;
        let vds_m = p * vds;
        let vbs_m = p * vbs;
        let vbd_m = vbs_m - vds_m;
        let vgd_m = vgs_m - vds_m;
        let mode = if vds_m >= 0.0 { 1.0 } else { -1.0 };
        let lvds = mode * vds_m;
        let lvbs = if mode > 0.0 { vbs_m } else { vbd_m };
        let lvgs = if mode > 0.0 { vgs_m } else { vgd_m };

        let forward = self.level2_forward_evaluate(lvgs, lvds, lvbs);
        Mos2Evaluation {
            id: p * mode * forward.id,
            region: forward.region,
            von: forward.von,
            vdsat: forward.vdsat,
        }
    }

    fn level2_forward_evaluate(&self, lvgs: Value, lvds: Value, lvbs: Value) -> Mos2Evaluation {
        let effective_length = self.level2_effective_length();
        let effective_width = self.w.max(1.0e-18);
        let phi = self.phi.max(1.0e-12);
        let sqrt_phi = phi.sqrt();
        let phi_min_vbs = (phi - lvbs).max(1.0e-18);
        let cox = self.cox.max(0.0);
        let oxide_cap = cox * effective_length * effective_width;
        let beta = self.kp * effective_width / effective_length;
        let xd = self.level2_depletion_width_factor();
        let model_vto = if self.polarity() < 0.0 {
            -self.vto.abs()
        } else {
            self.vto
        };
        let t_vbi = model_vto - self.polarity() * self.gamma * sqrt_phi;

        let sarg = if lvbs <= 0.0 {
            phi_min_vbs.sqrt()
        } else {
            sqrt_phi / (1.0 + 0.5 * lvbs / phi)
        };
        let dsrgdb = if lvbs <= 0.0 {
            -0.5 / sarg
        } else {
            -0.5 * sarg * sarg / (phi * sqrt_phi)
        };
        let barg_input = (phi_min_vbs + lvds).max(1.0e-18);
        let barg = if (lvbs - lvds) <= 0.0 {
            barg_input.sqrt()
        } else {
            sqrt_phi / (1.0 + 0.5 * (lvbs - lvds) / phi)
        };
        let dbrgdb = if (lvbs - lvds) <= 0.0 {
            -0.5 / barg
        } else {
            -0.5 * barg * barg / (phi * sqrt_phi)
        };

        let factor = if oxide_cap > 0.0 {
            0.125 * self.mos2_narrow_factor * 2.0 * std::f64::consts::PI * EPSSIL / oxide_cap
                * effective_length
        } else {
            0.0
        };
        let eta = 1.0 + factor;
        let vbin = self.polarity() * t_vbi + factor * phi_min_vbs;

        let (gamasd, dgddvb) = self.level2_short_channel_gamma_with_body_derivative(
            sarg,
            barg,
            dsrgdb,
            dbrgdb,
            effective_length,
            xd,
        );
        let mut von = vbin + gamasd * sarg;
        let mut vdsat = 0.0;

        let fast_surface = self.mos2_fast_surface_state_density != 0.0 && oxide_cap > 0.0;
        let mut argg = 0.0;
        if fast_surface {
            if let Some((delta_von, surface_argg)) =
                self.level2_fast_surface_state_shift(gamasd, dgddvb, sarg, dsrgdb, factor, cox)
            {
                von += delta_von;
                argg = surface_argg;
            }
        } else if lvgs <= vbin {
            return Mos2Evaluation {
                id: 0.0,
                region: MosRegion::Cutoff,
                von,
                vdsat,
            };
        }

        let vgst = lvgs - von;
        let sarg3 = sarg * sarg * sarg;
        let body = barg * barg * barg - sarg3;

        let critical_gate_overdrive = if cox > 0.0 {
            self.mos2_crit_field * 100.0 * EPSSIL / cox
        } else {
            Value::INFINITY
        };
        let ufact = if vgst > critical_gate_overdrive && critical_gate_overdrive > 0.0 {
            (critical_gate_overdrive / vgst).powf(self.mos2_crit_field_exp)
        } else {
            1.0
        };

        let gammad = gamasd / eta;
        let vgsx = if fast_surface { lvgs.max(von) } else { lvgs };
        if gammad > 0.0 {
            let gammd2 = gammad * gammad;
            let argv = (vgsx - vbin) / eta + phi_min_vbs;
            if argv > 0.0 {
                let arg1 = (1.0 + 4.0 * argv / gammd2).sqrt();
                vdsat = ((vgsx - vbin) / eta + gammd2 * (1.0 - arg1) / 2.0).max(0.0);
            }
        } else {
            vdsat = ((vgsx - vbin) / eta).max(0.0);
        }

        let mut xlamda = self.lambda;
        let mut bodys = body;
        if lvds != 0.0 {
            let bsarg_input = (vdsat + phi_min_vbs).max(1.0e-18);
            let bsarg = if (lvbs - vdsat) <= 0.0 {
                bsarg_input.sqrt()
            } else {
                sqrt_phi / (1.0 + 0.5 * (lvbs - vdsat) / phi)
            };
            bodys = bsarg * bsarg * bsarg - sarg3;

            if self.mos2_max_drift_vel <= 0.0 && self.mos2_substrate_doping > 0.0 && xlamda <= 0.0 {
                let argv = (lvds - vdsat) / 4.0;
                let sargv = (1.0 + argv * argv).sqrt();
                let arg1 = (argv + sargv).max(0.0).sqrt();
                xlamda = xd * arg1 / (effective_length * lvds);
            }
        }

        let mut clfact = 1.0 - xlamda * lvds;
        if !clfact.is_finite() || clfact <= 1.0e-12 {
            clfact = 1.0e-12;
        }
        let xleff = effective_length * clfact;
        let deltal = xlamda * lvds * effective_length;
        let mut punch_through_width = xd * self.pb.max(1.0e-12).sqrt();
        if self.mos2_substrate_doping == 0.0 {
            punch_through_width = 0.25e-6;
        }
        if xleff < punch_through_width {
            let xld = effective_length - punch_through_width;
            let denom = 1.0 + (deltal - xld) / punch_through_width;
            if denom.is_finite() && denom > 1.0e-12 {
                clfact = punch_through_width / denom / effective_length;
            }
        }

        let beta1 = beta * ufact / clfact.max(1.0e-12);
        if lvds <= 1.0e-10 {
            return Mos2Evaluation {
                id: 0.0,
                region: MosRegion::Cutoff,
                von,
                vdsat,
            };
        }

        let (cdrain, region) = if fast_surface && lvgs <= von {
            if vdsat <= 0.0 {
                (0.0, MosRegion::Cutoff)
            } else {
                let vdson = vdsat.min(lvds);
                let body_for_vdson = if lvds > vdsat { bodys } else { body };
                let cdson = beta1
                    * ((von - vbin - eta * vdson / 2.0) * vdson - gamasd * body_for_vdson / 1.5);
                (cdson * (argg * (lvgs - von)).exp(), MosRegion::Cutoff)
            }
        } else if lvds <= vdsat {
            let current = beta1 * ((lvgs - vbin - eta * lvds / 2.0) * lvds - gamasd * body / 1.5);
            (current, MosRegion::Linear)
        } else {
            let current =
                beta1 * ((lvgs - vbin - eta * vdsat / 2.0) * vdsat - gamasd * bodys / 1.5);
            (current, MosRegion::Saturation)
        };

        Mos2Evaluation {
            id: if cdrain.is_finite() { cdrain } else { 0.0 },
            region,
            von,
            vdsat,
        }
    }

    fn level2_forward_operating_point(
        &self,
        lvgs: Value,
        lvds: Value,
        lvbs: Value,
    ) -> Mos2ForwardOperatingPoint {
        let lvgs = Dual3::variable(lvgs, 0);
        let lvds = Dual3::variable(lvds, 1);
        let lvbs = Dual3::variable(lvbs, 2);
        let effective_length = self.level2_effective_length();
        let effective_width = self.w.max(1.0e-18);
        let phi = self.phi.max(1.0e-12);
        let sqrt_phi = phi.sqrt();
        let phi_min_vbs = (phi - lvbs).max_const(1.0e-18);
        let cox = self.cox.max(0.0);
        let oxide_cap = cox * effective_length * effective_width;
        let beta = self.kp * effective_width / effective_length;
        let xd = self.level2_depletion_width_factor();
        let model_vto = if self.polarity() < 0.0 {
            -self.vto.abs()
        } else {
            self.vto
        };
        let t_vbi = model_vto - self.polarity() * self.gamma * sqrt_phi;

        let sarg = if lvbs.value <= 0.0 {
            phi_min_vbs.sqrt()
        } else {
            Dual3::constant(sqrt_phi) / (1.0 + 0.5 * lvbs / phi)
        };
        let dsrgdb = if lvbs.value <= 0.0 {
            -0.5 / sarg.value
        } else {
            -0.5 * sarg.value * sarg.value / (phi * sqrt_phi)
        };
        let barg_input = (phi_min_vbs + lvds).max_const(1.0e-18);
        let barg = if (lvbs - lvds).value <= 0.0 {
            barg_input.sqrt()
        } else {
            Dual3::constant(sqrt_phi) / (1.0 + 0.5 * (lvbs - lvds) / phi)
        };

        let factor = if oxide_cap > 0.0 {
            0.125 * self.mos2_narrow_factor * 2.0 * std::f64::consts::PI * EPSSIL / oxide_cap
                * effective_length
        } else {
            0.0
        };
        let eta = 1.0 + factor;
        let vbin = self.polarity() * t_vbi + factor * phi_min_vbs;

        let gamasd = self.level2_short_channel_gamma_dual(sarg, barg, effective_length, xd);
        let mut von = vbin + gamasd * sarg;
        let mut vdsat = Dual3::constant(0.0);

        let fast_surface = self.mos2_fast_surface_state_density != 0.0 && oxide_cap > 0.0;
        let mut argg = Dual3::constant(0.0);
        if fast_surface {
            if let Some((delta_von, surface_argg)) =
                self.level2_fast_surface_state_shift_dual(gamasd, sarg, dsrgdb, factor, cox)
            {
                von = von + delta_von;
                argg = surface_argg;
            }
        } else if lvgs.value <= vbin.value {
            return Mos2ForwardOperatingPoint::from_dual(0.0);
        }

        let vgst = lvgs - von;
        let sarg3 = sarg * sarg * sarg;
        let body = barg * barg * barg - sarg3;

        let critical_gate_overdrive = if cox > 0.0 {
            self.mos2_crit_field * 100.0 * EPSSIL / cox
        } else {
            Value::INFINITY
        };
        let ufact = if vgst.value > critical_gate_overdrive && critical_gate_overdrive > 0.0 {
            (critical_gate_overdrive / vgst).powf(self.mos2_crit_field_exp)
        } else {
            Dual3::constant(1.0)
        };

        let gammad = gamasd / eta;
        let vgsx = if fast_surface { lvgs.max(von) } else { lvgs };
        if gammad.value > 0.0 {
            let gammd2 = gammad * gammad;
            let argv = (vgsx - vbin) / eta + phi_min_vbs;
            if argv.value > 0.0 {
                let arg1 = (1.0 + 4.0 * argv / gammd2).sqrt();
                vdsat = ((vgsx - vbin) / eta + gammd2 * (1.0 - arg1) / 2.0).max_const(0.0);
            }
        } else {
            vdsat = ((vgsx - vbin) / eta).max_const(0.0);
        }

        let mut xlamda = Dual3::constant(self.lambda);
        let mut bodys = body;
        if lvds.value != 0.0 {
            let bsarg_input = (vdsat + phi_min_vbs).max_const(1.0e-18);
            let bsarg = if (lvbs - vdsat).value <= 0.0 {
                bsarg_input.sqrt()
            } else {
                Dual3::constant(sqrt_phi) / (1.0 + 0.5 * (lvbs - vdsat) / phi)
            };
            bodys = bsarg * bsarg * bsarg - sarg3;

            if self.mos2_max_drift_vel <= 0.0
                && self.mos2_substrate_doping > 0.0
                && xlamda.value <= 0.0
            {
                let argv = (lvds - vdsat) / 4.0;
                let sargv = (1.0 + argv * argv).sqrt();
                let arg1 = (argv + sargv).max_const(0.0).sqrt();
                xlamda = xd * arg1 / (effective_length * lvds);
            }
        }

        let mut clfact = 1.0 - xlamda * lvds;
        if !clfact.value.is_finite() || clfact.value <= 1.0e-12 {
            clfact = Dual3::constant(1.0e-12);
        }
        let xleff = effective_length * clfact;
        let deltal = xlamda * lvds * effective_length;
        let mut punch_through_width = xd * self.pb.max(1.0e-12).sqrt();
        if self.mos2_substrate_doping == 0.0 {
            punch_through_width = 0.25e-6;
        }
        if xleff.value < punch_through_width {
            let xld = effective_length - punch_through_width;
            let denom = 1.0 + (deltal - xld) / punch_through_width;
            if denom.value.is_finite() && denom.value > 1.0e-12 {
                clfact = punch_through_width / denom / effective_length;
            }
        }

        let beta1 = beta * ufact / clfact.max_const(1.0e-12);
        if lvds.value <= 1.0e-10 {
            let gds = if self.mos2_fast_surface_state_density != 0.0 && oxide_cap != 0.0 {
                if lvgs.value <= von.value {
                    beta1 * (von - vbin - gamasd * sarg)
                } else {
                    beta1 * (lvgs - vbin - gamasd * sarg)
                }
            } else if lvgs.value <= von.value {
                Dual3::constant(0.0)
            } else {
                beta1 * (lvgs - vbin - gamasd * sarg)
            };

            return Mos2ForwardOperatingPoint::channel_conductance(gds.value);
        }

        let cdrain = if fast_surface && lvgs.value <= von.value {
            if vdsat.value <= 0.0 {
                Dual3::constant(0.0)
            } else {
                let vdson = if vdsat.value <= lvds.value {
                    vdsat
                } else {
                    lvds
                };
                let body_for_vdson = if lvds.value > vdsat.value {
                    bodys
                } else {
                    body
                };
                let cdson = beta1
                    * ((von - vbin - eta * vdson / 2.0) * vdson - gamasd * body_for_vdson / 1.5);
                cdson * (argg * (lvgs - von)).exp()
            }
        } else if lvds.value <= vdsat.value {
            beta1 * ((lvgs - vbin - eta * lvds / 2.0) * lvds - gamasd * body / 1.5)
        } else {
            beta1 * ((lvgs - vbin - eta * vdsat / 2.0) * vdsat - gamasd * bodys / 1.5)
        };

        Mos2ForwardOperatingPoint::from_dual(cdrain)
    }

    #[inline]
    fn level2_short_channel_gamma_with_body_derivative(
        &self,
        sarg: Value,
        barg: Value,
        dsrgdb: Value,
        dbrgdb: Value,
        effective_length: Value,
        xd: Value,
    ) -> (Value, Value) {
        if self.gamma <= 0.0 && self.mos2_substrate_doping <= 0.0 {
            return (self.gamma, 0.0);
        }

        let mut argss = 0.0;
        let mut argsd = 0.0;
        let mut dbargs = 0.0;
        let mut dbargd = 0.0;
        if self.mos2_junction_depth > 0.0 && xd > 0.0 {
            let scale = 2.0 / self.mos2_junction_depth;
            let argxs = (1.0 + xd * sarg * scale).max(0.0);
            let argxd = (1.0 + xd * barg * scale).max(0.0);
            let args = argxs.sqrt();
            let argd = argxd.sqrt();
            let length_scale = 0.5 * self.mos2_junction_depth / effective_length;
            argss = length_scale * (args - 1.0);
            argsd = length_scale * (argd - 1.0);

            if args > 0.0 && argd > 0.0 {
                let derivative_scale = 0.5 / effective_length;
                dbargs = derivative_scale * xd * dsrgdb / args;
                dbargd = derivative_scale * xd * dbrgdb / argd;
            }
        }

        (
            self.gamma * (1.0 - argss - argsd),
            -self.gamma * (dbargs + dbargd),
        )
    }

    #[inline]
    fn level2_fast_surface_state_shift(
        &self,
        gamasd: Value,
        dgddvb: Value,
        sarg: Value,
        dsrgdb: Value,
        factor: Value,
        cox: Value,
    ) -> Option<(Value, Value)> {
        if self.mos2_fast_surface_state_density == 0.0 || cox <= 0.0 {
            return None;
        }

        let cfs = CHARGE * self.mos2_fast_surface_state_density * 1.0e4;
        let cdonco = -(gamasd * dsrgdb + dgddvb * sarg) + factor;
        let xn = 1.0 + cfs / cox + cdonco;
        let thermal_slope = VT_REFERENCE * xn;
        if thermal_slope.is_finite() && thermal_slope > 0.0 {
            Some((thermal_slope, 1.0 / thermal_slope))
        } else {
            None
        }
    }

    #[inline]
    fn level2_fast_surface_state_shift_dual(
        &self,
        gamasd: Dual3,
        sarg: Dual3,
        dsrgdb: Value,
        factor: Value,
        cox: Value,
    ) -> Option<(Dual3, Dual3)> {
        if self.mos2_fast_surface_state_density == 0.0 || cox <= 0.0 {
            return None;
        }

        let cfs = CHARGE * self.mos2_fast_surface_state_density * 1.0e4;
        let cdonco = -(gamasd.value * dsrgdb + gamasd.derivative[2] * sarg.value) + factor;
        let xn = 1.0 + cfs / cox + cdonco;
        let thermal_slope = VT_REFERENCE * xn;
        if thermal_slope.is_finite() && thermal_slope > 0.0 {
            let slope = Dual3::constant(thermal_slope);
            Some((slope, 1.0 / slope))
        } else {
            None
        }
    }

    #[inline]
    fn level2_short_channel_gamma_dual(
        &self,
        sarg: Dual3,
        barg: Dual3,
        effective_length: Value,
        xd: Value,
    ) -> Dual3 {
        if self.gamma <= 0.0 && self.mos2_substrate_doping <= 0.0 {
            return Dual3::constant(self.gamma);
        }

        let mut argss = Dual3::constant(0.0);
        let mut argsd = Dual3::constant(0.0);
        if self.mos2_junction_depth > 0.0 && xd > 0.0 {
            let xws = xd * sarg;
            let xwd = xd * barg;
            let scale = 2.0 / self.mos2_junction_depth;
            let args = (1.0 + xws * scale).max_const(0.0).sqrt();
            let argd = (1.0 + xwd * scale).max_const(0.0).sqrt();
            let length_scale = 0.5 * self.mos2_junction_depth / effective_length;
            argss = length_scale * (args - 1.0);
            argsd = length_scale * (argd - 1.0);
        }

        self.gamma * (1.0 - argss - argsd)
    }

    #[inline]
    fn level2_depletion_width_factor(&self) -> Value {
        if self.mos2_substrate_doping <= 0.0 {
            return 0.0;
        }

        let nsub_m3 = self.mos2_substrate_doping * 1.0e6;
        let denom = CHARGE * nsub_m3;
        if denom <= 0.0 {
            0.0
        } else {
            ((EPSSIL + EPSSIL) / denom).sqrt()
        }
    }
}

impl Mos2ForwardOperatingPoint {
    #[inline]
    fn from_dual(cdrain: impl Into<Dual3>) -> Self {
        let cdrain = cdrain.into();
        let (gm, gds, gmb) = if cdrain.value.is_finite() {
            (
                cdrain.sanitized_derivative(0),
                cdrain.sanitized_derivative(1),
                cdrain.sanitized_derivative(2),
            )
        } else {
            (0.0, 0.0, 0.0)
        };

        Self { gm, gds, gmb }
    }

    #[inline]
    fn channel_conductance(gds: Value) -> Self {
        Self {
            gm: 0.0,
            gds: if gds.is_finite() { gds } else { 0.0 },
            gmb: 0.0,
        }
    }
}

impl From<Value> for Dual3 {
    #[inline]
    fn from(value: Value) -> Self {
        Self::constant(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn mos2_reference_device() -> Mosfet {
        let mut params = HashMap::new();
        params.insert("LEVEL".to_string(), 2.0);
        params.insert("NSUB".to_string(), 2.2e15);
        params.insert("UO".to_string(), 575.0);
        params.insert("UCRIT".to_string(), 49.0e3);
        params.insert("UEXP".to_string(), 0.1);
        params.insert("TOX".to_string(), 0.11e-6);
        params.insert("XJ".to_string(), 2.95e-6);
        params.insert("LD".to_string(), 2.4485e-6);
        params.insert("KP".to_string(), 2.0e-5);
        params.insert("PHI".to_string(), 0.6);

        Mosfet::new_nmos("m1".to_string(), 1, 2, 3, 4)
            .with_params(&params)
            .with_geometry(30.0e-6, 12.0e-6)
    }

    #[test]
    fn level2_exact_jacobian_matches_centered_difference() {
        let mos = mos2_reference_device();
        let cases = [(1.2, 0.15, -0.02), (3.5, 1.8, -0.35), (2.0, -0.4, -0.1)];

        for (vgs, vds, vbs) in cases {
            let (_, _, gm, gds, gmb) = mos.level2_operating_point(vgs, vds, vbs);
            let finite_difference = |dvgs: Value, dvds: Value, dvbs: Value| {
                let step = 1.0e-7;
                let plus =
                    mos.level2_evaluate(vgs + dvgs * step, vds + dvds * step, vbs + dvbs * step);
                let minus =
                    mos.level2_evaluate(vgs - dvgs * step, vds - dvds * step, vbs - dvbs * step);
                (plus.id - minus.id) / (2.0 * step)
            };

            assert_relative(gm, finite_difference(1.0, 0.0, 0.0), 2.0e-5);
            assert_relative(gds, finite_difference(0.0, 1.0, 0.0), 2.0e-5);
            assert_relative(gmb, finite_difference(0.0, 0.0, 1.0), 2.0e-5);
        }
    }

    #[test]
    fn level2_zero_drain_bias_keeps_channel_conductance() {
        let mos = mos2_reference_device();
        let vgs = 2.0;
        let vbs = -0.1;
        let (_, _, gm, gds, gmb) = mos.level2_operating_point(vgs, 0.0, vbs);
        let effective_length = mos.level2_effective_length();
        let phi_min_vbs = mos.phi - vbs;
        let sarg = phi_min_vbs.sqrt();
        let factor = 0.125 * mos.mos2_narrow_factor * 2.0 * std::f64::consts::PI * EPSSIL
            / (mos.cox * effective_length * mos.w)
            * effective_length;
        let vbin = mos.vto - mos.gamma * mos.phi.sqrt() + factor * phi_min_vbs;
        let dsrgdb = -0.5 / sarg;
        let (gamasd, _) = mos.level2_short_channel_gamma_with_body_derivative(
            sarg,
            sarg,
            dsrgdb,
            dsrgdb,
            effective_length,
            mos.level2_depletion_width_factor(),
        );
        let vgst = vgs - (vbin + gamasd * sarg);
        let critical_gate_overdrive = mos.mos2_crit_field * 100.0 * EPSSIL / mos.cox;
        let ufact = if vgst > critical_gate_overdrive {
            (critical_gate_overdrive / vgst).powf(mos.mos2_crit_field_exp)
        } else {
            1.0
        };
        let expected = mos.kp * mos.w / effective_length * ufact * (vgs - vbin - gamasd * sarg);

        assert_eq!(gm, 0.0);
        assert_eq!(gmb, 0.0);
        assert!(gds > 0.0, "gds={gds:e}");
        assert_relative(gds, expected, 1.0e-12);
    }

    #[test]
    fn level2_fast_surface_state_subthreshold_branch_is_finite() {
        let mut mos = mos2_reference_device();
        mos.mos2_fast_surface_state_density = 2.0e10;
        let vgs = -0.25;
        let vds = 0.8;
        let vbs = -0.15;

        let eval = mos.level2_evaluate(vgs, vds, vbs);
        let (_, _, gm, gds, gmb) = mos.level2_operating_point(vgs, vds, vbs);

        assert!(eval.id.is_finite() && eval.id > 0.0, "id={:e}", eval.id);
        assert!(gm.is_finite() && gm > 0.0, "gm={gm:e}");
        assert!(gds.is_finite(), "gds={gds:e}");
        assert!(gmb.is_finite(), "gmb={gmb:e}");
    }

    fn assert_relative(actual: Value, expected: Value, tolerance: Value) {
        let scale = actual.abs().max(expected.abs()).max(1.0e-18);
        assert!(
            (actual - expected).abs() <= tolerance * scale,
            "actual={actual:e} expected={expected:e}"
        );
    }
}
