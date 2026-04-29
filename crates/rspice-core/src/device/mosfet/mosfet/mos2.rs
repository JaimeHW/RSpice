use super::*;

const EPSSIL: Value = 11.7 * 8.854_214_871e-12;
const CHARGE: Value = 1.602_176_634e-19;

#[derive(Debug, Clone, Copy)]
pub(in crate::device::mosfet::mosfet) struct Mos2Evaluation {
    pub(in crate::device::mosfet::mosfet) id: Value,
    pub(in crate::device::mosfet::mosfet) region: MosRegion,
    pub(in crate::device::mosfet::mosfet) von: Value,
    pub(in crate::device::mosfet::mosfet) vdsat: Value,
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
        let derivative = |dvgs: Value, dvds: Value, dvbs: Value, step: Value| -> Value {
            if step <= 0.0 || !step.is_finite() {
                return 0.0;
            }
            let plus =
                self.level2_evaluate(vgs + dvgs * step, vds + dvds * step, vbs + dvbs * step);
            let minus =
                self.level2_evaluate(vgs - dvgs * step, vds - dvds * step, vbs - dvbs * step);
            let slope = (plus.id - minus.id) / (2.0 * step);
            if slope.is_finite() { slope } else { 0.0 }
        };

        let gm_step = 1.0e-6 * vgs.abs().max(1.0);
        let gds_step = 1.0e-6 * vds.abs().max(1.0);
        let gmb_step = 1.0e-6 * vbs.abs().max(1.0);
        let gm = derivative(1.0, 0.0, 0.0, gm_step);
        let gds = derivative(0.0, 1.0, 0.0, gds_step);
        let gmb = derivative(0.0, 0.0, 1.0, gmb_step);

        (eval.id, eval.region, gm, gds, gmb)
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
        let barg_input = (phi_min_vbs + lvds).max(1.0e-18);
        let barg = if (lvbs - lvds) <= 0.0 {
            barg_input.sqrt()
        } else {
            sqrt_phi / (1.0 + 0.5 * (lvbs - lvds) / phi)
        };

        let factor = if oxide_cap > 0.0 {
            0.125 * self.mos2_narrow_factor * 2.0 * std::f64::consts::PI * EPSSIL / oxide_cap
                * effective_length
        } else {
            0.0
        };
        let eta = 1.0 + factor;
        let vbin = self.polarity() * t_vbi + factor * phi_min_vbs;

        let gamasd = self.level2_short_channel_gamma(sarg, barg, effective_length, xd);
        let von = vbin + gamasd * sarg;
        let mut vdsat = 0.0;

        if self.mos2_fast_surface_state_density == 0.0 || oxide_cap == 0.0 {
            if lvgs <= vbin {
                return Mos2Evaluation {
                    id: 0.0,
                    region: MosRegion::Cutoff,
                    von,
                    vdsat,
                };
            }
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
        let vgsx = if self.mos2_fast_surface_state_density != 0.0 && oxide_cap != 0.0 {
            lvgs.max(von)
        } else {
            lvgs
        };
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

        let (cdrain, region) = if lvds <= vdsat {
            let current = beta1 * ((lvgs - vbin - eta * lvds / 2.0) * lvds - gamasd * body / 1.5);
            (current, MosRegion::Linear)
        } else {
            let current =
                beta1 * ((lvgs - vbin - eta * vdsat / 2.0) * vdsat - gamasd * bodys / 1.5);
            (current, MosRegion::Saturation)
        };

        Mos2Evaluation {
            id: if cdrain.is_finite() {
                cdrain.max(0.0)
            } else {
                0.0
            },
            region,
            von,
            vdsat,
        }
    }

    #[inline]
    fn level2_short_channel_gamma(
        &self,
        sarg: Value,
        barg: Value,
        effective_length: Value,
        xd: Value,
    ) -> Value {
        if self.gamma <= 0.0 && self.mos2_substrate_doping <= 0.0 {
            return self.gamma;
        }

        let mut argss = 0.0;
        let mut argsd = 0.0;
        if self.mos2_junction_depth > 0.0 && xd > 0.0 {
            let xws = xd * sarg;
            let xwd = xd * barg;
            let scale = 2.0 / self.mos2_junction_depth;
            let args = (1.0 + xws * scale).max(0.0).sqrt();
            let argd = (1.0 + xwd * scale).max(0.0).sqrt();
            let length_scale = 0.5 * self.mos2_junction_depth / effective_length;
            argss = length_scale * (args - 1.0);
            argsd = length_scale * (argd - 1.0);
        }

        let gamma = self.gamma * (1.0 - argss - argsd);
        gamma
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
