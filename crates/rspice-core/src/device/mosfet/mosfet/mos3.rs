use super::{MosRegion, Mosfet};
use crate::Value;

const EPSSIL: Value = 11.7 * 8.854_214_871e-12;
const CHARGE: Value = 1.602_176_634e-19;

#[derive(Clone, Copy, Debug)]
pub(super) struct Mos3State {
    pub ids: f64,
    pub gm: f64,
    pub gds: f64,
    pub gmb: f64,
    pub von: f64,
    pub vdsat: f64,
    pub region: MosRegion,
}

#[derive(Clone, Copy, Debug)]
struct Mos3ClmParams {
    fdrain: Value,
    onvdsc: Value,
    onfg: Value,
    dfgdvg: Value,
    dfgdvd: Value,
    dfgdvb: Value,
    dfddvg: Value,
    dfddvd: Value,
    dfddvb: Value,
    lvds: Value,
    vdsat: Value,
    dvsdvg: Value,
    dvsdvd: Value,
    dvsdvb: Value,
    effective_length: Value,
}

impl Mosfet {
    #[inline]
    pub(super) fn mos3_effective_length(&self) -> Value {
        (self.l - 2.0 * self.ld + self.mos3_length_adjust).max(1.0e-12)
    }

    #[inline]
    pub(super) fn mos3_effective_width(&self) -> Value {
        (self.w - 2.0 * self.mos3_width_narrow + self.mos3_width_adjust).max(1.0e-12)
    }

    pub(in crate::device::mosfet::mosfet) fn calculate_id_mos3(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, MosRegion) {
        let state = self.mos3_state(vgs, vds, vbs);
        (self.polarity() * state.ids, state.region)
    }

    pub(in crate::device::mosfet::mosfet) fn mos3_state(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> Mos3State {
        self.mos3_bias_scalar(vgs, vds, vbs)
    }

    pub(in crate::device::mosfet::mosfet) fn mos3_terminal_small_signal(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, Value, Value) {
        let state = self.mos3_state(vgs, vds, vbs);
        let p = self.polarity();
        let vds_m = p * vds;
        let (gm, gds, gmb) = if vds_m >= 0.0 {
            (state.gm, state.gds, state.gmb)
        } else {
            (-state.gm, state.gm + state.gds + state.gmb, -state.gmb)
        };
        let sanitize = |value: Value| if value.is_finite() { value } else { 0.0 };
        (sanitize(gm), sanitize(gds), sanitize(gmb))
    }

    fn mos3_bias_scalar(&self, vgs: Value, vds: Value, vbs: Value) -> Mos3State {
        if !vgs.is_finite() || !vds.is_finite() || !vbs.is_finite() {
            return Mos3State {
                von: self.polarity() * self.vto,
                ..Mos3State::default()
            };
        }

        let p = self.polarity();
        let vgs_m = p * vgs;
        let vds_m = p * vds;
        let vbs_m = p * vbs;
        let vbd_m = vbs_m - vds_m;
        let vgd_m = vgs_m - vds_m;

        let mode = if vds_m >= 0.0 { 1.0 } else { -1.0 };
        let lvds = (mode * vds_m).max(0.0);
        let lvbs = if mode > 0.0 { vbs_m } else { vbd_m };
        let lvgs = if mode > 0.0 { vgs_m } else { vgd_m };

        let forward = self.mos3_forward_bias_scalar(lvgs, lvds, lvbs);
        Mos3State {
            ids: mode * forward.ids,
            von: p * forward.von,
            vdsat: p * forward.vdsat,
            ..forward
        }
    }

    fn mos3_forward_bias_scalar(&self, lvgs: Value, lvds: Value, lvbs: Value) -> Mos3State {
        let effective_length = self.mos3_effective_length();
        let effective_width = self.mos3_effective_width();
        let one_over_l = 1.0 / effective_length;
        let cox = self.cox.max(0.0);
        let oxide_cap = cox * effective_length * effective_width;
        let beta = self.kp * effective_width * one_over_l;
        let phi = self.phi.max(1.0e-12);
        let sqrt_phi = phi.sqrt();
        let t_vbi = self.vto - self.polarity() * self.gamma * sqrt_phi;
        let eta = if cox > 0.0 {
            self.mos3_eta * 8.15e-22 / (cox * effective_length.powi(3))
        } else {
            0.0
        };

        let (phibs, sqphbs, dsqdvb) = if lvbs <= 0.0 {
            let phibs = (phi - lvbs).max(1.0e-18);
            let sqphbs = phibs.sqrt();
            (phibs, sqphbs, -0.5 / sqphbs)
        } else {
            let sqphs3 = phi * sqrt_phi;
            let sqphbs = sqrt_phi / (1.0 + lvbs / (phi + phi)).max(1.0e-12);
            let phibs = sqphbs * sqphbs;
            (phibs, sqphbs, -phibs / (sqphs3 + sqphs3).max(1.0e-18))
        };

        let (fshort, dfsdvb) = self.mos3_short_channel_factor(sqphbs, dsqdvb, effective_length);
        let gammas = self.gamma * fshort;
        let fbodys = 0.5 * gammas / (sqphbs + sqphbs).max(1.0e-18);
        let fbody = fbodys + self.mos3_narrow_factor / effective_width;
        let onfbdy = 1.0 / (1.0 + fbody).max(1.0e-12);
        let dfbdvb = if fshort.abs() > 1.0e-18 {
            -fbodys * dsqdvb / sqphbs + fbodys * dfsdvb / fshort
        } else {
            -fbodys * dsqdvb / sqphbs
        };
        let qbonco = gammas * sqphbs + self.mos3_narrow_factor * phibs / effective_width;
        let dqbdvb = gammas * dsqdvb + self.gamma * dfsdvb * sqphbs
            - self.mos3_narrow_factor / effective_width;
        let vbix = self.polarity() * t_vbi - eta * lvds;
        let vth = vbix + qbonco;
        let dvtdvd = -eta;
        let dvtdvb = dqbdvb;
        let mut von = vth;

        let mut xn = 0.0;
        let mut dxndvb = 0.0;
        let mut dvodvd = 0.0;
        let mut dvodvb = 0.0;
        let fast_surface = self.mos3_fast_surface_state_density != 0.0 && oxide_cap > 0.0;
        if fast_surface {
            let csonco = CHARGE
                * self.mos3_fast_surface_state_density
                * 1.0e4
                * effective_length
                * effective_width
                / oxide_cap;
            let cdonco = qbonco / (phibs + phibs).max(1.0e-18);
            xn = 1.0 + csonco + cdonco;
            von = vth + self.vt * xn;
            dxndvb = dqbdvb / (phibs + phibs).max(1.0e-18)
                - qbonco * dsqdvb / (phibs * sqphbs).max(1.0e-18);
            dvodvd = dvtdvd;
            dvodvb = dvtdvb + self.vt * dxndvb;
        } else if lvgs <= von {
            return Mos3State {
                von,
                vdsat: 0.0,
                ..Mos3State::default()
            };
        }

        let vgsx = lvgs.max(von);
        let overdrive = vgsx - vth;
        let onfg = (1.0 + self.mos3_theta * overdrive).max(1.0e-12);
        let fgate = 1.0 / onfg;
        let us = self.u0.max(0.0) * 1.0e-4 * fgate;
        let dfgdvg = -self.mos3_theta * fgate * fgate;
        let dfgdvd = -dfgdvg * dvtdvd;
        let dfgdvb = -dfgdvg * dvtdvb;

        let mut vdsc = 0.0;
        let mut onvdsc = 0.0;
        let mut vdsat = overdrive * onfbdy;
        let mut dvsdvg = onfbdy;
        let mut dvsdvd = -dvsdvg * dvtdvd;
        let mut dvsdvb = -dvsdvg * dvtdvb - vdsat * dfbdvb * onfbdy;
        if self.mos3_max_drift_velocity > 0.0 && us > 0.0 {
            vdsc = effective_length * self.mos3_max_drift_velocity / us;
            onvdsc = 1.0 / vdsc;
            let arga = overdrive * onfbdy;
            let argb = (arga * arga + vdsc * vdsc).sqrt();
            vdsat = arga + vdsc - argb;
            let dvsdga = (1.0 - arga / argb) * onfbdy;
            dvsdvg = dvsdga - (1.0 - vdsc / argb) * vdsc * dfgdvg * onfg;
            dvsdvd = -dvsdvg * dvtdvd;
            dvsdvb = -dvsdvg * dvtdvb - arga * dvsdga * dfbdvb;
        }
        if !vdsat.is_finite() || vdsat < 0.0 {
            vdsat = 0.0;
        }

        let vdsx = lvds.min(vdsat);
        if vdsx.abs() <= 0.0 {
            let mut gds0 = beta * fgate * overdrive.max(0.0);
            if fast_surface && lvgs < von && xn > 0.0 {
                let exponent = ((lvgs - von) / (self.vt * xn)).clamp(-80.0, 80.0);
                gds0 *= exponent.exp();
            }
            return Mos3State {
                gds: if gds0.is_finite() { gds0 } else { 0.0 },
                von,
                vdsat,
                region: MosRegion::Linear,
                ..Mos3State::default()
            };
        }

        let cdo = overdrive - 0.5 * (1.0 + fbody) * vdsx;
        let dcodvb = -dvtdvb - 0.5 * dfbdvb * vdsx;
        let cdnorm = cdo * vdsx;
        let cd1 = beta * cdnorm;
        let mut cdrain = beta * fgate * cdnorm;
        let mut gm = beta * fgate * vdsx + dfgdvg * cd1;
        let gds_seed = if lvds > vdsat {
            -dvtdvd * vdsx
        } else {
            lvgs - vth - (1.0 + fbody + dvtdvd) * vdsx
        };
        let mut gds = beta * fgate * gds_seed + dfgdvd * cd1;
        let mut gmb = beta * fgate * dcodvb * vdsx + dfgdvb * cd1;

        let mut fdrain = 1.0;
        let mut dfddvg = 0.0;
        let mut dfddvd = 0.0;
        let mut dfddvb = 0.0;
        if self.mos3_max_drift_velocity > 0.0 && vdsc > 0.0 {
            fdrain = 1.0 / (1.0 + vdsx * onvdsc).max(1.0e-12);
            let fd2 = fdrain * fdrain;
            let arga = fd2 * vdsx * onvdsc * onfg;
            dfddvg = -dfgdvg * arga;
            dfddvd = if lvds > vdsat {
                -dfgdvd * arga
            } else {
                -dfgdvd * arga - fd2 * onvdsc
            };
            dfddvb = -dfgdvb * arga;
            gm = fdrain * gm + dfddvg * cdrain;
            gds = fdrain * gds + dfddvd * cdrain;
            gmb = fdrain * gmb + dfddvb * cdrain;
            cdrain *= fdrain;
        }

        let (clm_cdrain, clm_gm, clm_gds, clm_gmb, clm_gds0) = self
            .mos3_apply_channel_length_modulation(
                cdrain,
                gm,
                gds,
                gmb,
                Mos3ClmParams {
                    fdrain,
                    onvdsc,
                    onfg,
                    dfgdvg,
                    dfgdvd,
                    dfgdvb,
                    dfddvg,
                    dfddvd,
                    dfddvb,
                    lvds,
                    vdsat,
                    dvsdvg,
                    dvsdvd,
                    dvsdvb,
                    effective_length,
                },
            );
        cdrain = clm_cdrain;
        gm = clm_gm;
        gds = clm_gds;
        gmb = clm_gmb;

        if fast_surface && lvgs < von && xn > 0.0 {
            let exponent = ((lvgs - von) / (self.vt * xn)).clamp(-80.0, 80.0);
            let wfact = exponent.exp();
            cdrain *= wfact;
            let gms = gm * wfact;
            let gmw = cdrain / (self.vt * xn);
            gm = gmw;
            if lvds > vdsat {
                gm += clm_gds0 * dvsdvg * wfact;
            }
            gds = gds * wfact + (gms - gmw) * dvodvd;
            gmb =
                gmb * wfact + (gms - gmw) * dvodvb - gmw * (lvgs - von) / xn.max(1.0e-18) * dxndvb;
        }

        let ids = if cdrain.is_finite() { cdrain } else { 0.0 };
        let sanitize = |value: Value| if value.is_finite() { value } else { 0.0 };
        let region = if fast_surface && lvgs < von {
            MosRegion::Cutoff
        } else if lvds <= vdsat {
            MosRegion::Linear
        } else {
            MosRegion::Saturation
        };

        Mos3State {
            ids,
            gm: sanitize(gm),
            gds: sanitize(gds),
            gmb: sanitize(gmb),
            von,
            vdsat,
            region,
        }
    }

    fn mos3_apply_channel_length_modulation(
        &self,
        mut cdrain: Value,
        mut gm: Value,
        mut gds: Value,
        mut gmb: Value,
        params: Mos3ClmParams,
    ) -> (Value, Value, Value, Value, Value) {
        let alpha = self.mos3_alpha();
        if alpha <= 0.0 || self.mos3_kappa <= 0.0 || params.vdsat <= 0.0 {
            return (cdrain, gm, gds, gmb, 0.0);
        }

        let (mut delxl, mut dldvd, mut ddldvg, mut ddldvd, mut ddldvb) =
            if params.lvds <= params.vdsat {
                if self.mos3_max_drift_velocity > 0.0 {
                    return (cdrain, gm, gds, gmb, 0.0);
                }
                let ratio = params.lvds / params.vdsat;
                let mut delxl = (self.mos3_kappa * alpha * params.vdsat / 8.0).sqrt();
                let dldvd = 4.0 * delxl * ratio * ratio * ratio / params.vdsat;
                delxl *= ratio.powi(4);
                (delxl, dldvd, 0.0, -dldvd, 0.0)
            } else if self.mos3_max_drift_velocity > 0.0 {
                let one_minus_fdrain = 1.0 - params.fdrain;
                if params.onvdsc <= 0.0 || one_minus_fdrain == 0.0 || cdrain == 0.0 {
                    return (cdrain, gm, gds, gmb, 0.0);
                }
                let cdsat = cdrain;
                let gdsat = (cdsat * one_minus_fdrain * params.onvdsc).max(1.0e-12);
                let gdoncd = gdsat / cdsat;
                let gdonfd = gdsat / one_minus_fdrain;
                let gdonfg = gdsat * params.onfg;
                let dgdvg = gdoncd * gm - gdonfd * params.dfddvg + gdonfg * params.dfgdvg;
                let dgdvd = gdoncd * gds - gdonfd * params.dfddvd + gdonfg * params.dfgdvd;
                let dgdvb = gdoncd * gmb - gdonfd * params.dfddvb + gdonfg * params.dfgdvb;
                let emax = self.mos3_kappa * cdsat / params.effective_length / gdsat;
                let emoncd = emax / cdsat;
                let emongd = emax / gdsat;
                let demdvg = emoncd * gm - emongd * dgdvg;
                let demdvd = emoncd * gds - emongd * dgdvd;
                let demdvb = emoncd * gmb - emongd * dgdvb;
                let arga = 0.5 * emax * alpha;
                let argc = self.mos3_kappa * alpha;
                let argb = (arga * arga + argc * (params.lvds - params.vdsat)).sqrt();
                let delxl = argb - arga;
                let (dldvd, dldem) = if argb != 0.0 {
                    (argc / (argb + argb), 0.5 * (arga / argb - 1.0) * alpha)
                } else {
                    (0.0, 0.0)
                };
                (
                    delxl,
                    dldvd,
                    dldem * demdvg,
                    dldem * demdvd - dldvd,
                    dldem * demdvb,
                )
            } else {
                let denom = params.lvds - params.vdsat + params.vdsat / 8.0;
                let delxl = (self.mos3_kappa * alpha * denom).sqrt();
                let dldvd = 0.5 * delxl / denom;
                (delxl, dldvd, 0.0, -dldvd, 0.0)
            };

        if !delxl.is_finite() {
            return (cdrain, gm, gds, gmb, 0.0);
        }
        if delxl > 0.5 * params.effective_length {
            delxl = params.effective_length
                - params.effective_length * params.effective_length / (4.0 * delxl);
            let scale = 4.0 * (params.effective_length - delxl) * (params.effective_length - delxl)
                / (params.effective_length * params.effective_length);
            ddldvg *= scale;
            ddldvd *= scale;
            ddldvb *= scale;
            dldvd *= scale;
        }

        let dlonxl = delxl / params.effective_length;
        if dlonxl >= 1.0 {
            return (cdrain, gm, gds, gmb, 0.0);
        }
        let xlfact = 1.0 / (1.0 - dlonxl);
        cdrain *= xlfact;
        let diddl = cdrain / (params.effective_length - delxl);
        gm = gm * xlfact + diddl * ddldvg;
        gmb = gmb * xlfact + diddl * ddldvb;
        let gds0 = diddl * ddldvd;
        gm += gds0 * params.dvsdvg;
        gmb += gds0 * params.dvsdvb;
        gds = gds * xlfact + diddl * dldvd + gds0 * params.dvsdvd;

        (cdrain, gm, gds, gmb, gds0)
    }

    fn mos3_short_channel_factor(
        &self,
        sqphbs: Value,
        dsqdvb: Value,
        effective_length: Value,
    ) -> (Value, Value) {
        const COEFF0: Value = 0.063_135_3;
        const COEFF1: Value = 0.801_329_2;
        const COEFF2: Value = -0.011_107_77;

        let xd = self.mos3_coeff_depletion_width();
        if self.mos3_junction_depth <= 0.0 || xd <= 0.0 {
            return (1.0, 0.0);
        }

        let wps = xd * sqphbs;
        let one_over_xj = 1.0 / self.mos3_junction_depth;
        let xjonxl = self.mos3_junction_depth / effective_length;
        let djonxj = self.ld * one_over_xj;
        let wponxj = wps * one_over_xj;
        let wconxj = COEFF0 + COEFF1 * wponxj + COEFF2 * wponxj * wponxj;
        let arga = wconxj + djonxj;
        let argc = wponxj / (1.0 + wponxj).max(1.0e-12);
        let argb = (1.0 - argc * argc).max(0.0).sqrt();
        let fshort = 1.0 - xjonxl * (arga * argb - djonxj);

        let dwpdvb = xd * dsqdvb;
        let dadvb = (COEFF1 + COEFF2 * (wponxj + wponxj)) * dwpdvb * one_over_xj;
        let dbdvb = if argb > 0.0 && wps.abs() > 1.0e-18 {
            -argc * argc * (1.0 - argc) * dwpdvb / (argb * wps)
        } else {
            0.0
        };
        let dfsdvb = -xjonxl * (dadvb * argb + arga * dbdvb);

        (fshort, dfsdvb)
    }

    #[inline]
    fn mos3_alpha(&self) -> Value {
        if self.mos2_substrate_doping <= 0.0 {
            0.0
        } else {
            let nsub_m3 = self.mos2_substrate_doping * 1.0e6;
            let denom = CHARGE * nsub_m3;
            if denom > 0.0 {
                (EPSSIL + EPSSIL) / denom
            } else {
                0.0
            }
        }
    }

    #[inline]
    fn mos3_coeff_depletion_width(&self) -> Value {
        self.mos3_alpha().max(0.0).sqrt()
    }
}

impl Default for Mos3State {
    fn default() -> Self {
        Self {
            ids: 0.0,
            gm: 0.0,
            gds: 0.0,
            gmb: 0.0,
            von: 0.0,
            vdsat: 0.0,
            region: MosRegion::Cutoff,
        }
    }
}
