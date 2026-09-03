use super::*;
use std::ops::{Add, Div, Mul, Neg, Sub};

const EPSSIL: Value = 11.7 * 8.854_214_871e-12;

/// The bias-dependent terms the level-2 fast surface-state shift reads: the
/// body-effect argument, the two branch voltages it is evaluated at, and the
/// two body-charge derivatives that go with them.
#[derive(Clone, Copy)]
struct Level2SurfaceShiftBias {
    barg: Value,
    lvbs: Value,
    lvds: Value,
    dsrgdb: Value,
    dbrgdb: Value,
}

/// The geometry and oxide terms of the same shift: the fast-surface-state
/// factor, the oxide capacitance, the effective channel length and the
/// depletion-width coefficient.
#[derive(Clone, Copy)]
struct Level2SurfaceShiftGeometry {
    factor: Value,
    cox: Value,
    effective_length: Value,
    xd: Value,
}

// MOS2 is a Berkeley/Xyce legacy model.  Xyce 7.10 intentionally retains
// the historical device constants from N_DEV_Const.h rather than the modern
// SI values used by the rest of the native device library.  Keep that
// compatibility local to the canonical MOS2 equations.
const XYCE_CHARGE: Value = 1.602_191_8e-19;
const XYCE_BOLTZMANN: Value = 1.380_622_6e-23;
const XYCE_K_OVER_Q: Value = XYCE_BOLTZMANN / XYCE_CHARGE;
const PHYSICAL_K_OVER_Q: Value = crate::constants::K_BOLTZMANN / crate::constants::Q_ELECTRON;

#[derive(Debug, Clone, Copy)]
pub(in crate::device::mosfet::classic) struct Mos2Evaluation {
    pub(in crate::device::mosfet::classic) id: Value,
    pub(in crate::device::mosfet::classic) region: MosRegion,
    pub(in crate::device::mosfet::classic) von: Value,
    pub(in crate::device::mosfet::classic) vdsat: Value,
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
    fn level2_xyce_thermal_voltage(&self) -> Value {
        self.vt * (XYCE_K_OVER_Q / PHYSICAL_K_OVER_Q)
    }

    #[inline]
    pub(in crate::device::mosfet::classic) fn level2_effective_length(&self) -> Value {
        (self.l - 2.0 * self.ld).max(1.0e-12)
    }

    #[inline]
    pub(in crate::device::mosfet::classic) fn level2_model_space_onset_voltage(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> Value {
        self.level2_evaluate(vgs, vds, vbs).von
    }

    #[inline]
    pub(in crate::device::mosfet::classic) fn level2_operating_point(
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
    pub(in crate::device::mosfet::classic) fn level2_evaluate(
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

    /// Saturation voltage under velocity saturation, mos2load.c's "baum's
    /// theory of scattering velocity saturation": the resolvent cubic of
    /// the quartic in `x = sqrt(vdsat + phi - vbs)`, then the smallest
    /// positive root that satisfies the quartic to 1e-6. `None` keeps the
    /// classic vdsat (no valid root), exactly like ngspice's `jknt == 0`.
    #[allow(clippy::too_many_arguments)]
    fn level2_vmax_vdsat(
        vgsx: Value,
        vbin: Value,
        eta: Value,
        gammad: Value,
        phi_min_vbs: Value,
        sarg3: Value,
        xv: Value,
    ) -> Option<Value> {
        const SIG1: [Value; 4] = [1.0, -1.0, 1.0, -1.0];
        const SIG2: [Value; 4] = [1.0, 1.0, -1.0, -1.0];

        let v1 = (vgsx - vbin) / eta + phi_min_vbs;
        let v2 = phi_min_vbs;
        let a1 = gammad / 0.75;
        let b1 = -2.0 * (v1 + xv);
        let c1 = -2.0 * gammad * xv;
        let d1 = 2.0 * v1 * (v2 + xv) - v2 * v2 - 4.0 / 3.0 * gammad * sarg3;
        let a = -b1;
        let b = a1 * c1 - 4.0 * d1;
        let c = -d1 * (a1 * a1 - 4.0 * b1) - c1 * c1;
        let r = -a * a / 3.0 + b;
        let s = 2.0 * a * a * a / 27.0 - a * b / 3.0 + c;
        let r3 = r * r * r;
        let s2 = s * s;
        let p = s2 / 4.0 + r3 / 27.0;
        let p0 = p.abs();
        let p2 = p0.sqrt();
        let y3 = if p < 0.0 {
            let ro = (s2 / 4.0 + p0).sqrt();
            let ro = (ro.ln() / 3.0).exp();
            let fi = (-2.0 * p2 / s).atan();
            2.0 * ro * (fi / 3.0).cos() - a / 3.0
        } else {
            // ngspice takes |.|^(1/3) for both branch terms.
            let p3 = ((-s / 2.0 + p2).abs().ln() / 3.0).exp();
            let p4 = ((-s / 2.0 - p2).abs().ln() / 3.0).exp();
            p3 + p4 - a / 3.0
        };

        let a3 = (a1 * a1 / 4.0 - b1 + y3).sqrt();
        let b3 = (y3 * y3 / 4.0 - d1).sqrt();
        let mut xvalid: Option<Value> = None;
        for i in 0..4 {
            let a4 = a1 / 2.0 + SIG1[i] * a3;
            let b4 = y3 / 2.0 + SIG2[i] * b3;
            let delta4 = a4 * a4 / 4.0 - b4;
            if !delta4.is_finite() || delta4 < 0.0 {
                continue;
            }
            let tmp = delta4.sqrt();
            for root in [-a4 / 2.0 + tmp, -a4 / 2.0 - tmp] {
                if !root.is_finite() || root <= 0.0 {
                    continue;
                }
                let poly = root * root * root * root
                    + a1 * root * root * root
                    + b1 * root * root
                    + c1 * root
                    + d1;
                if !poly.is_finite() || poly.abs() > 1.0e-6 {
                    continue;
                }
                match xvalid {
                    Some(v) if root >= v => {}
                    _ => xvalid = Some(root),
                }
            }
        }

        xvalid
            .map(|x| x * x - phi_min_vbs)
            .filter(|v| v.is_finite())
    }

    /// ngspice's analytic vdsat sensitivities under velocity saturation
    /// (mos2load.c dfunds/dfundg/dfundb), evaluated at the quartic vdsat.
    /// Returns `(dsdvgs, dsdvbs)`.
    #[allow(clippy::too_many_arguments)]
    fn level2_vmax_vdsat_derivatives(
        &self,
        vdsat: Value,
        vgsx: Value,
        vbin: Value,
        eta: Value,
        gammad: Value,
        lvbs: Value,
        phi: Value,
        sqrt_phi: Value,
        phi_min_vbs: Value,
        sarg: Value,
        sarg3: Value,
        dsrgdb: Value,
        dgdvbs: Value,
        factor: Value,
        ueff: Value,
    ) -> (Value, Value) {
        let bsarg_input = (vdsat + phi_min_vbs).max(1.0e-18);
        let (bsarg, dbsrdb) = if (lvbs - vdsat) <= 0.0 {
            let bsarg = bsarg_input.sqrt();
            (bsarg, -0.5 / bsarg)
        } else {
            let bsarg = sqrt_phi / (1.0 + 0.5 * (lvbs - vdsat) / phi);
            (bsarg, -0.5 * bsarg * bsarg / (phi * sqrt_phi))
        };
        let bodys = bsarg * bsarg * bsarg - sarg3;
        let gdbdvs = 2.0 * gammad * (bsarg * bsarg * dbsrdb - sarg * sarg * dsrgdb);

        let argv = (vgsx - vbin) / eta - vdsat;
        let vqchan = argv - gammad * bsarg;
        let dqdsat = -1.0 + gammad * dbsrdb;
        let vl = self.mos2_max_drift_vel * self.level2_effective_length();
        let dfunds = vl * dqdsat - ueff * vqchan;
        let dfundg = (vl - ueff * vdsat) / eta;
        let dfundb =
            -vl * (1.0 + dqdsat - factor / eta) + ueff * (gdbdvs - dgdvbs * bodys / 1.5) / eta;
        if dfunds == 0.0 || !dfunds.is_finite() {
            return (0.0, 0.0);
        }
        (-dfundg / dfunds, -dfundb / dfunds)
    }

    fn level2_forward_evaluate(&self, lvgs: Value, lvds: Value, lvbs: Value) -> Mos2Evaluation {
        let effective_length = self.level2_effective_length();
        let effective_width = self.w.max(1.0e-18);
        let phi = self.phi.max(1.0e-12);
        let sqrt_phi = phi.sqrt();
        // Xyce retains the signed `tPhi-lvbs` value in the MOS2 equations.
        // The positive-body branch evaluates `sarg` with its rational form,
        // so clamping this term to zero changes the VMAX root and the
        // subthreshold current for forward-biased body junctions.
        let phi_min_vbs = phi - lvbs;
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

        // Xyce uses the normalized body coefficient while solving the
        // Grove--Frohman VDSAT equation.  The channel-length-shortening
        // branch below switches back to the physical short-channel gamma;
        // keep the two quantities distinct so the VMAX path cannot
        // accidentally reuse the normalized value.
        let gammad_vdsat = gamasd / eta;
        let vgsx = if fast_surface { lvgs.max(von) } else { lvgs };
        if gammad_vdsat > 0.0 {
            let gammd2 = gammad_vdsat * gammad_vdsat;
            let argv = (vgsx - vbin) / eta + phi_min_vbs;
            if argv > 0.0 {
                let arg1 = (1.0 + 4.0 * argv / gammd2).sqrt();
                vdsat = ((vgsx - vbin) / eta + gammd2 * (1.0 - arg1) / 2.0).max(0.0);
            }
        } else {
            vdsat = ((vgsx - vbin) / eta).max(0.0);
        }
        if self.mos2_max_drift_vel > 0.0 {
            // VMAX given: velocity saturation lowers vdsat (Baum quartic).
            // ngspice reads the card mobility here, not the
            // temperature-scaled one (mos2load.c uses MOS2surfaceMobility).
            let ueff = self.u0_card * 1.0e-4 * ufact;
            if ueff > 0.0 && effective_length > 0.0 {
                let xv = self.mos2_max_drift_vel * effective_length / ueff;
                if let Some(sat) =
                    Self::level2_vmax_vdsat(vgsx, vbin, eta, gammad_vdsat, phi_min_vbs, sarg3, xv)
                {
                    vdsat = sat;
                }
            }
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

            if self.mos2_substrate_doping > 0.0 && xlamda <= 0.0 {
                if self.mos2_max_drift_vel <= 0.0 {
                    let argv = (lvds - vdsat) / 4.0;
                    let sargv = (1.0 + argv * argv).sqrt();
                    let arg1 = (argv + sargv).max(0.0).sqrt();
                    xlamda = xd * arg1 / (effective_length * lvds);
                } else {
                    // Xyce/ngspice's VMAX path models the carrier-density
                    // dependent channel shortening with NEFF.  Omitting
                    // this branch leaves short-channel MOS2 devices with
                    // the wrong effective beta even when their explicit
                    // LAMBDA is zero.
                    let channel_charge = self.mos2_channel_charge.max(1.0e-18);
                    let xdv = xd / channel_charge.sqrt();
                    let ueff = self.u0_card * 1.0e-4 * ufact;
                    if xdv.is_finite() && ueff.is_finite() && ueff > 0.0 {
                        let xlv = self.mos2_max_drift_vel * xdv / (2.0 * ueff);
                        let argv = (lvds - vdsat).max(0.0);
                        let xls = (xlv * xlv + argv).sqrt();
                        if xls.is_finite() && xls > 0.0 {
                            xlamda = xdv / (effective_length * lvds) * (xls - xlv);
                        }
                    }
                }
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
        // Keep the signed Xyce `tPhi-lvbs` term; only the branch-selected
        // square-root arguments below are guarded where a root is required.
        let phi_min_vbs = Dual3::constant(phi) - lvbs;
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
        let dbrgdb = if (lvbs - lvds).value <= 0.0 {
            -0.5 / barg.value
        } else {
            -0.5 * barg.value * barg.value / (phi * sqrt_phi)
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
            if let Some((delta_von, surface_argg)) = self.level2_fast_surface_state_shift_dual(
                gamasd,
                sarg,
                Level2SurfaceShiftBias {
                    barg: barg.value,
                    lvbs: lvbs.value,
                    lvds: lvds.value,
                    dsrgdb,
                    dbrgdb,
                },
                Level2SurfaceShiftGeometry {
                    factor,
                    cox,
                    effective_length,
                    xd,
                },
            ) {
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

        // Keep the VDSAT-normalized gamma separate from the physical gamma
        // used by Xyce's VMAX channel-shortening sensitivities below.
        let gammad_vdsat = gamasd / eta;
        let vgsx = if fast_surface { lvgs.max(von) } else { lvgs };
        if gammad_vdsat.value > 0.0 {
            let gammd2 = gammad_vdsat * gammad_vdsat;
            let argv = (vgsx - vbin) / eta + phi_min_vbs;
            if argv.value > 0.0 {
                let arg1 = (1.0 + 4.0 * argv / gammd2).sqrt();
                vdsat = ((vgsx - vbin) / eta + gammd2 * (1.0 - arg1) / 2.0).max_const(0.0);
            }
        } else {
            vdsat = ((vgsx - vbin) / eta).max_const(0.0);
        }
        // Xyce deliberately treats the fast-surface-state `vgsx` clamp as a
        // value clamp only.  Its analytic VDSAT sensitivities still use the
        // canonical Grove--Frohman derivatives below, even when `vgsx` is
        // pinned to `Von` in subthreshold operation.  The ordinary Dual3
        // derivative follows the clamp and therefore cannot be used for this
        // branch.
        let mut vmax_vdsat_derivatives = (vdsat.derivative[0], vdsat.derivative[2]);
        if fast_surface {
            vmax_vdsat_derivatives = if gammad_vdsat.value > 0.0 {
                let argv = (vgsx.value - vbin.value) / eta + phi_min_vbs.value;
                if argv > 0.0 {
                    let arg1 =
                        (1.0 + 4.0 * argv / (gammad_vdsat.value * gammad_vdsat.value)).sqrt();
                    let dsdvgs = (1.0 - 1.0 / arg1) / eta;
                    let dsdvbs = (gammad_vdsat.value * (1.0 - arg1)
                        + 2.0 * argv / (gammad_vdsat.value * arg1))
                        / eta
                        * gamasd.derivative[2]
                        + 1.0 / arg1
                        + factor * dsdvgs;
                    (dsdvgs, dsdvbs)
                } else {
                    (0.0, 0.0)
                }
            } else {
                (1.0, 0.0)
            };
        }
        if self.mos2_max_drift_vel > 0.0 {
            // VMAX given: velocity saturation lowers vdsat (Baum quartic).
            // The root is found at value level; ngspice's analytic
            // dfunds/dfundg/dfundb sensitivities seed the dual so all
            // downstream derivatives chain exactly like mos2load.c.
            // Card mobility, like the scalar path (never temperature-scaled).
            let ueff = self.u0_card * 1.0e-4 * ufact.value;
            if ueff > 0.0 && effective_length > 0.0 {
                let xv = self.mos2_max_drift_vel * effective_length / ueff;
                if let Some(sat) = Self::level2_vmax_vdsat(
                    vgsx.value,
                    vbin.value,
                    eta,
                    gammad_vdsat.value,
                    phi_min_vbs.value,
                    sarg3.value,
                    xv,
                ) {
                    vdsat = Dual3 {
                        value: sat,
                        derivative: vdsat.derivative,
                    };
                }
                // Xyce evaluates the analytic VMAX sensitivities even when
                // its quartic root search keeps the ordinary Grove--Frohman
                // VDSAT.  The derivative block therefore uses the final
                // scalar VDSAT, whether or not the optional root was found.
                let (dsdvgs, dsdvbs) = self.level2_vmax_vdsat_derivatives(
                    vdsat.value,
                    vgsx.value,
                    vbin.value,
                    eta,
                    // Xyce resets gammad=gamasd before evaluating the
                    // VMAX channel-length derivative block.  The
                    // normalized value is only used by the VDSAT root.
                    gamasd.value,
                    lvbs.value,
                    phi,
                    sqrt_phi,
                    phi_min_vbs.value,
                    sarg.value,
                    sarg3.value,
                    dsrgdb,
                    gamasd.derivative[2],
                    factor,
                    ueff,
                );
                vmax_vdsat_derivatives = (dsdvgs, dsdvbs);
                vdsat.derivative = [dsdvgs, 0.0, dsdvbs];
            }
        }

        let mut xlamda = Dual3::constant(self.lambda);
        let mut channel_length_derivatives = None;
        let mut bodys = body;
        if lvds.value != 0.0 {
            let bsarg_input = (vdsat + phi_min_vbs).max_const(1.0e-18);
            let bsarg = if (lvbs - vdsat).value <= 0.0 {
                bsarg_input.sqrt()
            } else {
                Dual3::constant(sqrt_phi) / (1.0 + 0.5 * (lvbs - vdsat) / phi)
            };
            bodys = bsarg * bsarg * bsarg - sarg3;

            if self.mos2_substrate_doping > 0.0 && xlamda.value <= 0.0 {
                if self.mos2_max_drift_vel <= 0.0 {
                    let argv = (lvds - vdsat) / 4.0;
                    let sargv = (1.0 + argv * argv).sqrt();
                    let arg1 = (argv + sargv).max_const(0.0).sqrt();
                    xlamda = xd * arg1 / (effective_length * lvds);
                } else {
                    let channel_charge = self.mos2_channel_charge.max(1.0e-18);
                    let xdv = xd / channel_charge.sqrt();
                    let ueff = ufact.value * (self.u0_card * 1.0e-4);
                    if ueff.is_finite() && ueff > 0.0 {
                        let xlv = self.mos2_max_drift_vel * xdv / (2.0 * ueff);
                        let argv = (lvds - vdsat).max_const(0.0);
                        let xls = (xlv * xlv + argv).sqrt();
                        if xls.value.is_finite() && xls.value > 0.0 {
                            xlamda = Dual3::constant(
                                xdv / (effective_length * lvds.value) * (xls.value - xlv),
                            );
                            // Xyce's MOS2 Jacobian treats the mobility
                            // factor xlv as frozen in this channel-shortening
                            // derivative block.  Its dld* sensitivities are
                            // the derivatives of clfact, not the exact dual
                            // derivative of the scalar xlamda expression.
                            let dldsat = xdv / (2.0 * xls.value) / effective_length;
                            channel_length_derivatives = Some([
                                dldsat * vmax_vdsat_derivatives.0,
                                -dldsat,
                                dldsat * vmax_vdsat_derivatives.1,
                            ]);
                        }
                    }
                }
            }
        }

        let mut clfact = if let Some(derivative) = channel_length_derivatives {
            Dual3 {
                value: 1.0 - xlamda.value * lvds.value,
                derivative,
            }
        } else {
            1.0 - xlamda * lvds
        };
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
                if let Some(derivative) = &mut channel_length_derivatives {
                    let dfact =
                        xleff.value * xleff.value / (punch_through_width * punch_through_width);
                    derivative.iter_mut().for_each(|value| *value *= dfact);
                    clfact.derivative = *derivative;
                }
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

        if fast_surface && lvgs.value <= von.value {
            return self.level2_fast_surface_state_operating_point(
                lvgs.value,
                lvds.value,
                lvbs.value,
                von.value,
                vbin.value,
                vdsat.value,
                argg.value,
                eta,
                factor,
                beta1.value,
                clfact.value,
                xlamda.value,
                gamasd.value,
                sarg.value,
                barg.value,
                body.value,
                dsrgdb,
                dbrgdb,
                gamasd.derivative[2],
                gamasd.derivative[1],
                channel_length_derivatives,
                xd,
                effective_length,
                vmax_vdsat_derivatives,
            );
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

    #[allow(clippy::too_many_arguments)]
    fn level2_fast_surface_state_operating_point(
        &self,
        lvgs: Value,
        lvds: Value,
        lvbs: Value,
        von: Value,
        vbin: Value,
        vdsat: Value,
        argg: Value,
        eta: Value,
        factor: Value,
        beta1: Value,
        clfact: Value,
        xlamda: Value,
        gamasd: Value,
        sarg: Value,
        barg: Value,
        body: Value,
        dsrgdb: Value,
        dbrgdb: Value,
        dgdvbs: Value,
        dgdvds: Value,
        channel_length_derivatives: Option<[Value; 3]>,
        xd: Value,
        effective_length: Value,
        vdsat_derivatives: (Value, Value),
    ) -> Mos2ForwardOperatingPoint {
        if vdsat <= 0.0 {
            return Mos2ForwardOperatingPoint::from_dual(0.0);
        }

        let phi = self.phi.max(1.0e-12);
        let sqrt_phi = phi.sqrt();
        let phi_min_vbs = phi - lvbs;
        let d2sdb2 = if lvbs <= 0.0 {
            0.5 * dsrgdb / phi_min_vbs
        } else {
            -dsrgdb * sarg / (phi * sqrt_phi)
        };
        let d2bdb2 = if lvds - lvbs >= 0.0 {
            0.5 * dbrgdb / (phi_min_vbs + lvds).max(1.0e-18)
        } else {
            -dbrgdb * barg / (phi * sqrt_phi)
        };
        let dgddb2 = if self.mos2_junction_depth > 0.0 && xd > 0.0 {
            let junction_scale = 2.0 / self.mos2_junction_depth;
            let argxs = (1.0 + xd * sarg * junction_scale).max(1.0e-18);
            let argxd = (1.0 + xd * barg * junction_scale).max(1.0e-18);
            let args = argxs.sqrt();
            let argd = argxd.sqrt();
            let dasdb2 = -xd * (d2sdb2 + dsrgdb * dsrgdb * xd / (self.mos2_junction_depth * argxs))
                / (effective_length * args);
            let daddb2 = -xd * (d2bdb2 + dbrgdb * dbrgdb * xd / (self.mos2_junction_depth * argxd))
                / (effective_length * argd);
            -0.5 * self.gamma * (dasdb2 + daddb2)
        } else {
            0.0
        };
        let dxndvb = 2.0 * dgdvbs * dsrgdb + gamasd * d2sdb2 + dgddb2 * sarg;
        let dxndvd = dgdvds * dsrgdb;
        let vt = self.level2_xyce_thermal_voltage();
        let dodvbs = -factor + dgdvbs * sarg + gamasd * dsrgdb + vt * dxndvb;
        let dodvds = dgdvds * sarg + vt * dxndvd;

        // `channel_length_derivatives` stores Xyce's signed sensitivities:
        // they are the negatives of the corresponding derivatives of
        // `clfact`, as used by the beta derivative terms below.
        let [_, dldvds, dldvbs] = channel_length_derivatives.unwrap_or_else(|| {
            if lvds != 0.0 {
                [0.0, -xlamda, 0.0]
            } else {
                [0.0; 3]
            }
        });

        let (vdson, body_for_vdson, barg_for_vdson, gdbdv) = if lvds > vdsat {
            let bsarg_input = (vdsat + phi_min_vbs).max(1.0e-18);
            let bsarg = if lvbs - vdsat <= 0.0 {
                bsarg_input.sqrt()
            } else {
                sqrt_phi / (1.0 + 0.5 * (lvbs - vdsat) / phi)
            };
            let dbsrdb = if lvbs - vdsat <= 0.0 {
                -0.5 / bsarg
            } else {
                -0.5 * bsarg * bsarg / (phi * sqrt_phi)
            };
            let bodys = bsarg * bsarg * bsarg - sarg * sarg * sarg;
            let gdbdvs = 2.0 * gamasd * (bsarg * bsarg * dbsrdb - sarg * sarg * dsrgdb);
            (vdsat, bodys, bsarg, gdbdvs)
        } else {
            (
                lvds,
                body,
                barg,
                2.0 * gamasd * (barg * barg * dbrgdb - sarg * sarg * dsrgdb),
            )
        };
        let cdson =
            beta1 * ((von - vbin - eta * vdson * 0.5) * vdson - gamasd * body_for_vdson / 1.5);
        let didvds = beta1 * (von - vbin - eta * vdson - gamasd * barg_for_vdson);
        let mut gdson = -cdson * dldvds / clfact - beta1 * dgdvds * body_for_vdson / 1.5;
        if lvds < vdsat {
            gdson += didvds;
        }
        let mut gbson = -cdson * dldvbs / clfact
            + beta1 * (dodvbs * vdson + factor * vdson - dgdvbs * body_for_vdson / 1.5 - gdbdv);
        let (dsdvgs, dsdvbs) = vdsat_derivatives;
        if lvds > vdsat {
            gbson += didvds * dsdvbs;
        }
        let expg = (argg * (lvgs - von)).exp();
        let cdrain = cdson * expg;
        let gmw = cdrain * argg;
        let mut gm = gmw;
        if lvds > vdsat {
            gm += didvds * dsdvgs * expg;
        }
        let xn = if vt != 0.0 && argg != 0.0 {
            1.0 / (vt * argg)
        } else {
            1.0
        };
        let tmp = gmw * (lvgs - von) / xn;
        let gds = gdson * expg - gm * dodvds - tmp * dxndvd;
        let gmbs = gbson * expg - gm * dodvbs - tmp * dxndvb;
        Mos2ForwardOperatingPoint { gm, gds, gmb: gmbs }
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

        let cfs = XYCE_CHARGE * self.mos2_fast_surface_state_density * 1.0e4;
        let cdonco = -(gamasd * dsrgdb + dgddvb * sarg) + factor;
        let xn = 1.0 + cfs / cox + cdonco;
        let thermal_slope = self.level2_xyce_thermal_voltage() * xn;
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
        bias: Level2SurfaceShiftBias,
        geometry: Level2SurfaceShiftGeometry,
    ) -> Option<(Dual3, Dual3)> {
        let Level2SurfaceShiftBias {
            barg,
            lvbs,
            lvds,
            dsrgdb,
            dbrgdb,
        } = bias;
        let Level2SurfaceShiftGeometry {
            factor,
            cox,
            effective_length,
            xd,
        } = geometry;
        if self.mos2_fast_surface_state_density == 0.0 || cox <= 0.0 {
            return None;
        }

        let cfs = XYCE_CHARGE * self.mos2_fast_surface_state_density * 1.0e4;
        let cdonco = -(gamasd.value * dsrgdb + gamasd.derivative[2] * sarg.value) + factor;
        let xn = 1.0 + cfs / cox + cdonco;
        let thermal_slope = self.level2_xyce_thermal_voltage() * xn;
        if thermal_slope.is_finite() && thermal_slope > 0.0 {
            // Xyce's MOS2 load evaluates the fast-surface-state threshold
            // shift as part of the Newton Jacobian.  The scalar value above
            // is not sufficient: `xn` depends on the body and drain biases
            // through the short-channel gamma and depletion-width terms.
            // Preserve those canonical first derivatives so transient
            // Newton steps see the same threshold sensitivity as Xyce.
            let phi = self.phi.max(1.0e-12);
            let sqrt_phi = phi.sqrt();
            let phi_min_vbs = phi - lvbs;
            let d2sdb2 = if lvbs <= 0.0 {
                0.5 * dsrgdb / phi_min_vbs
            } else {
                -dsrgdb * sarg.value / (phi * sqrt_phi)
            };
            let d2bdb2 = if lvds - lvbs >= 0.0 {
                0.5 * dbrgdb / (phi_min_vbs + lvds).max(1.0e-18)
            } else {
                -dbrgdb * barg / (phi * sqrt_phi)
            };
            let dgddb2 = if self.mos2_junction_depth > 0.0 && xd > 0.0 {
                let junction_scale = 2.0 / self.mos2_junction_depth;
                let argxs = (1.0 + xd * sarg.value * junction_scale).max(1.0e-18);
                let argxd = (1.0 + xd * barg * junction_scale).max(1.0e-18);
                let args = argxs.sqrt();
                let argd = argxd.sqrt();
                let dasdb2 = -xd
                    * (d2sdb2 + dsrgdb * dsrgdb * xd / (self.mos2_junction_depth * argxs))
                    / (effective_length * args);
                let daddb2 = -xd
                    * (d2bdb2 + dbrgdb * dbrgdb * xd / (self.mos2_junction_depth * argxd))
                    / (effective_length * argd);
                -0.5 * self.gamma * (dasdb2 + daddb2)
            } else {
                0.0
            };
            let dgdvbs = gamasd.derivative[2];
            let dgdvds = gamasd.derivative[1];
            let dxndvb = 2.0 * dgdvbs * dsrgdb + gamasd.value * d2sdb2 + dgddb2 * sarg.value;
            let dxndvd = dgdvds * dsrgdb;
            let slope = Dual3 {
                value: thermal_slope,
                derivative: [
                    0.0,
                    self.level2_xyce_thermal_voltage() * dxndvd,
                    self.level2_xyce_thermal_voltage() * dxndvb,
                ],
            };
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
        // The Rust model's temperature-normalized substrate geometry is
        // derived from the SI parameter set used by construction.  Keep that
        // geometric depletion width on the physical charge constant; the
        // legacy Xyce charge constant remains localized to the explicitly
        // Xyce-defined surface-state charge path above.
        let denom = crate::constants::Q_ELECTRON * nsub_m3;
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

    fn mos2_vmax_device() -> Mosfet {
        let mut params = HashMap::new();
        params.insert("LEVEL".to_string(), 2.0);
        params.insert("VTO".to_string(), 0.7);
        params.insert("KP".to_string(), 110.0e-6);
        params.insert("GAMMA".to_string(), 0.4);
        params.insert("PHI".to_string(), 0.65);
        params.insert("LAMBDA".to_string(), 0.02);
        params.insert("NSUB".to_string(), 1.0e16);
        params.insert("TOX".to_string(), 50.0e-9);
        params.insert("UO".to_string(), 600.0);
        params.insert("VMAX".to_string(), 1.0e5);
        params.insert("XJ".to_string(), 0.5e-6);

        Mosfet::new_nmos("m1".to_string(), 1, 2, 3, 4)
            .with_params(&params)
            .with_geometry(10.0e-6, 1.0e-6)
    }

    #[test]
    fn level2_vmax_drain_current_matches_ngspice46() {
        // ngspice-46 oracle (.op, vd#branch): 3.88779e-4 A at 27C for
        // vgs=1.5, vds=2. The velocity-saturated vdsat (Baum quartic)
        // is what pulls the classic value down by ~5%.
        let mos = mos2_vmax_device();
        let eval = mos.level2_evaluate(1.5, 2.0, 0.0);
        assert_relative(eval.id, 3.88779e-4, 1.0e-5);
    }

    #[test]
    fn level2_vmax_small_signal_matches_ngspice46() {
        // In saturation the conductances follow ngspice's CONVENTION, not
        // the true derivative: mos2load.c carries no dvdsat/dvds (the
        // vdsat dependence through the short-channel gamma is deliberately
        // neglected), so a finite-difference check cannot apply here.
        // Oracle: ngspice-46 .op, print @m1[gm] @m1[gds] @m1[gmbs].
        let mos = mos2_vmax_device();
        let (_, _, gm, gds, gmb) = mos.level2_operating_point(1.5, 2.0, 0.0);
        assert_relative(gm, 8.231405e-4, 1.0e-6);
        assert_relative(gds, 1.639541e-5, 1.0e-6);
        assert_relative(gmb, 7.948465e-5, 1.0e-6);
    }

    #[test]
    fn level2_vmax_linear_region_jacobian_matches_centered_difference() {
        // Linear-region current never reads vdsat, so the exact-derivative
        // check stays valid there even with VMAX given.
        let mos = mos2_vmax_device();
        let cases = [(1.2, 0.2, -0.2), (2.5, 0.5, -0.5)];

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

            assert_relative(gm, finite_difference(1.0, 0.0, 0.0), 2.0e-4);
            assert_relative(gds, finite_difference(0.0, 1.0, 0.0), 2.0e-4);
            assert_relative(gmb, finite_difference(0.0, 0.0, 1.0), 2.0e-4);
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
