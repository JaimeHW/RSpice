//! MESA/HFET channel current equations and current-model dispatch.

use super::*;

/// GATEMOD=1 gate-branch evaluation in the internal (polarity-folded)
/// frame: Schottky gate-source current/conductance and the thermally
/// self-consistent gate-drain current with its vgs/vds sensitivities
/// (the gate-drain branch carries no vgd conductance in this model).
#[derive(Debug, Clone, Copy)]
pub(super) struct Hfet1GateEval {
    pub cgs: Value,
    pub ggs: Value,
    pub cgd: Value,
    pub gmg: Value,
    pub gmd: Value,
}

impl Jfet {
    #[inline]
    pub(super) fn exp_limited(x: Value) -> Value {
        x.clamp(-80.0, 80.0).exp()
    }

    pub(super) fn mesa_level2_ids(
        &self,
        vgs: Value,
        vds: Value,
        temp_k: Value,
        vto: Value,
        lambda: Value,
    ) -> Value {
        const Q_ELECTRON: Value = 1.602176634e-19;
        const EPSILONGAAS: Value = 12.244 * 8.85418e-12;

        let p = &self.params;
        let w = self.width.max(1e-12);
        let l = self.length.max(1e-12);
        let vt = self.thermal_voltage(temp_k).max(1e-12);
        let eta = p.eta.abs().max(1e-12);
        let etavth = (eta * vt).max(1e-12);
        let d = p.hfet_di.max(1e-12);
        let nd = p.mesa_nd.max(1e-30);
        let vpo = Q_ELECTRON * nd * d * d / (2.0 * EPSILONGAAS);
        let sigma = p.sigma0.max(0.0)
            / (1.0 + Self::exp_limited((vgs - vto - p.hfet_vsigmat) / p.hfet_vsigma.max(1e-12)));
        let vgt = (vgs - vto) + sigma * vds;

        let mu = (p.hfet_mu + p.mesa_theta * vgt).max(1e-12);
        let vl = (p.hfet_vs.max(1e-12) / mu * l).max(1e-30);
        let beta_inst = 2.0 * EPSILONGAAS * p.hfet_vs.max(1e-12) * p.mesa_zeta.max(1e-12) * w / d;
        let beta = beta_inst / (vpo + 3.0 * vl).max(1e-30);

        let u = vgt / vt - 1.0;
        let t = (p.hfet_delta.max(1e-9).powi(2) + u * u).sqrt();
        let vgte = 0.5 * vt * (2.0 + u + t);
        let b = Self::exp_limited(-vgt / etavth);
        let n0 = (EPSILONGAAS * eta * vt / (Q_ELECTRON * d)).max(1e-30);
        let sqrt1 = if vgte >= vpo {
            0.0
        } else {
            (1.0 - vgte / vpo.max(1e-30)).max(0.0).sqrt()
        };
        let q = (1.0 - sqrt1).max(1e-30);
        let denom = 1.0 / (nd * d * q).max(1e-30) + b / n0;
        let ns = 1.0 / denom.max(1e-30);
        if !ns.is_finite() || ns < 1e-38 {
            return 0.0;
        }

        let rt = p.hfet_rsi.max(0.0) + p.hfet_rdi.max(0.0);
        let gchi0 = Q_ELECTRON * w / l;
        let gchi = gchi0 * mu * ns;
        let gch = gchi / (1.0 + gchi * rt);
        if !gch.is_finite() || gch <= 0.0 {
            return 0.0;
        }

        let a = 2.0 * beta * vgte;
        let f = (1.0 + 2.0 * a * p.hfet_rsi.max(0.0)).sqrt();
        let d_term = 1.0 + a * p.hfet_rsi.max(0.0) + f;
        let e_term = 1.0 + p.mesa_tc * vgte;
        let isata = a * vgte / (d_term * e_term).max(1e-30);
        let isatb0 = Q_ELECTRON * n0 * vt * w / l;
        let isatb = isatb0 * mu * Self::exp_limited(vgt / etavth);
        let isat = if (isata + isatb).abs() > 1e-30 {
            isata * isatb / (isata + isatb)
        } else {
            0.0
        };
        let vsate = (isat / gch).abs().max(1e-30);
        let m = (p.hfet_m + p.mesa_alpha * vgte).max(1e-9);
        let g = (vds / vsate).max(0.0).powf(m);
        let h = (1.0 + g).powf(1.0 / m);
        let ids = gch * vds * (1.0 + lambda * vds) / h;
        if ids.is_finite() { ids } else { 0.0 }
    }

    pub(super) fn mesa_level2_linearization_forward(
        &self,
        vgs: Value,
        vds: Value,
        temp_k: Value,
        vto: Value,
        lambda: Value,
    ) -> MesaLevel2Linearization {
        const Q_ELECTRON: Value = 1.602176634e-19;
        const EPSILONGAAS: Value = 12.244 * 8.85418e-12;

        let p = &self.params;
        let w = self.width.max(1e-12);
        let l = self.length.max(1e-12);
        let d = p.hfet_di.max(1e-12);
        let nd = p.mesa_nd.max(1e-30);
        let vpo = Q_ELECTRON * nd * d * d / (2.0 * EPSILONGAAS);
        let vt = self.thermal_voltage(temp_k).max(1e-12);
        let eta = p.eta.abs().max(1e-12);
        let etavth = (eta * vt).max(1e-12);
        let rsi = p.hfet_rsi.max(0.0);
        let rt = rsi + p.hfet_rdi.max(0.0);
        let vsigma = p.hfet_vsigma.max(1e-12);

        let vgt0 = vgs - vto;
        let s = Self::exp_limited((vgt0 - p.hfet_vsigmat) / vsigma);
        let sigma = p.sigma0.max(0.0) / (1.0 + s);
        let vgt = vgt0 + sigma * vds;
        let mu = (p.hfet_mu + p.mesa_theta * vgt).max(1e-12);
        let vl = (p.hfet_vs.max(1e-12) / mu * l).max(1e-30);
        let beta_num = 2.0 * EPSILONGAAS * p.hfet_vs.max(1e-12) * p.mesa_zeta.max(1e-12) * w / d;
        let beta = beta_num / (vpo + 3.0 * vl).max(1e-30);

        let u = vgt / vt - 1.0;
        let t = (p.hfet_delta.max(1e-9).powi(2) + u * u).sqrt();
        let vgte = 0.5 * vt * (2.0 + u + t);
        let b = Self::exp_limited(-vgt / etavth);

        let sqrt1 = if vgte > vpo {
            0.0
        } else {
            (1.0 - vgte / vpo.max(1e-30)).max(0.0).sqrt()
        };
        let q = (1.0 - sqrt1).max(1e-30);
        let n0 = (EPSILONGAAS * eta * vt / (Q_ELECTRON * d)).max(1e-30);
        let ns = 1.0 / (1.0 / (nd * d * q).max(1e-30) + b / n0);
        if !ns.is_finite() || ns < 1e-38 {
            return MesaLevel2Linearization::zero();
        }

        let gchi0 = Q_ELECTRON * w / l;
        let gchi = gchi0 * mu * ns;
        let gch = gchi / (1.0 + gchi * rt);
        if !gch.is_finite() || gch <= 0.0 {
            return MesaLevel2Linearization::zero();
        }

        let a = 2.0 * beta * vgte;
        let f = (1.0 + 2.0 * a * rsi).sqrt();
        let d_term = 1.0 + a * rsi + f;
        let e_term = 1.0 + p.mesa_tc * vgte;
        let isata = a * vgte / (d_term * e_term).max(1e-30);
        let isatb0 = Q_ELECTRON * n0 * vt * w / l;
        let isatb = isatb0 * mu * Self::exp_limited(vgt / etavth);
        let isat_sum = (isata + isatb).max(1e-30);
        let isat = isata * isatb / isat_sum;
        let vsate = (isat / gch).abs().max(1e-30);
        let m = (p.hfet_m + p.mesa_alpha * vgte).max(1e-9);
        let g = (vds / vsate).max(0.0).powf(m);
        let h = (1.0 + g).powf(1.0 / m);

        let delidgch0 = vds / h;
        let delidgch = delidgch0 * (1.0 + lambda * vds);
        let ids = gch * delidgch;

        let delgchgchi = 1.0 / (1.0 + gchi * rt).powi(2);
        let delgchins = gchi0 * mu;
        let delnsvgt = ns * ns * (1.0 / n0) / etavth * b;
        let delnsvgte = if sqrt1 == 0.0 {
            0.0
        } else {
            0.5 * ns * ns / (vpo.max(1e-30) * nd * d * sqrt1 * q * q)
        };
        let delvgtevgt = 0.5 * (1.0 + u / t.max(1e-30));
        let delidvds0 = gch / h;
        let delidvds1 = if vds != 0.0 {
            ids * (vds / vsate).powf(m - 1.0) / (vsate * (1.0 + g))
        } else {
            0.0
        };
        let delidvds = delidvds0 * (1.0 + 2.0 * lambda * vds) - delidvds1;
        let delidvsate = ids * g / (vsate * (1.0 + g));
        let delvsateisat = 1.0 / gch;
        let isat_sq = isat_sum * isat_sum;
        let delisatisata = isatb * isatb / isat_sq;
        let v_term = 1.0 + 1.0 / f;
        let ddevgte = 2.0 * beta * rsi * v_term * e_term + d_term * p.mesa_tc;
        let denom = (d_term * d_term * e_term * e_term).max(1e-30);
        let delisatavgte = (2.0 * a * d_term * e_term - a * vgte * ddevgte) / denom;
        let delisatabeta =
            2.0 * vgte * vgte * (d_term * e_term - a * e_term * rsi * v_term) / denom;
        let delisatisatb = isata * isata / isat_sq;
        let delvsategch = -vsate / gch;
        let dvgtvgs = 1.0 - p.sigma0.max(0.0) * vds * s / (vsigma * (1.0 + s).powi(2));
        let theta_term = gchi0 * ns * p.mesa_theta;
        let dgchivgt = delgchins * (delnsvgte * delvgtevgt + delnsvgt) + theta_term;
        let dvgtevds = delvgtevgt * sigma;
        let dgchivds = delgchins * (delnsvgte * dvgtevds + delnsvgt * sigma) + theta_term * sigma;
        let beta_theta_term =
            delisatabeta * 3.0 * beta * vl * p.mesa_theta / (mu * (vpo + 3.0 * vl).max(1e-30));
        let disatavgt = delisatavgte * delvgtevgt + beta_theta_term;
        let disatavds = delisatavgte * dvgtevds + beta_theta_term * sigma;
        let disatbvgt = isatb / etavth + isatb / mu * p.mesa_theta;
        let p_term = delgchgchi * dgchivgt;
        let w_term = delgchgchi * dgchivds;
        let dvsatevgt = delvsateisat * (delisatisata * disatavgt + delisatisatb * disatbvgt)
            + delvsategch * p_term;
        let dvsatevds = delvsateisat
            * (delisatisata * disatavds + delisatisatb * disatbvgt * sigma)
            + delvsategch * w_term;

        let (gmmadd, gdsmadd) = if p.mesa_alpha != 0.0 && vds != 0.0 {
            let gmmadd = ids
                * ((1.0 + g).ln() / (m * m) - g * (vds / vsate).ln() / (m * (1.0 + g)))
                * p.mesa_alpha
                * delvgtevgt;
            (gmmadd, gmmadd * sigma)
        } else {
            (0.0, 0.0)
        };

        let gm1 = delidvsate * dvsatevgt;
        let gm = (delidgch * p_term + gm1 + gmmadd) * dvgtvgs;
        let gds0 = delidvsate * dvsatevds + delidgch * w_term + gdsmadd;
        let gds = delidvds + gds0;

        MesaLevel2Linearization {
            ids: if ids.is_finite() { ids } else { 0.0 },
            gm: if gm.is_finite() { gm } else { 0.0 },
            gds: if gds.is_finite() { gds } else { 0.0 },
            vds,
            delidgch0,
            delidvds0,
            delidvds1,
            gm0: p_term,
            gm1,
            gm2: dvgtvgs,
            gds0,
        }
    }

    pub(super) fn mesa_level2_small_signal_forward(
        &self,
        vgs: Value,
        vds: Value,
        temp_k: Value,
        vto: Value,
        lambda: Value,
    ) -> (Value, Value, Value) {
        self.mesa_level2_linearization_forward(vgs, vds, temp_k, vto, lambda)
            .dc_terms()
    }

    pub(super) fn mesa_level2_ac_conductances_forward(
        &self,
        vgs: Value,
        vds: Value,
        temp_k: Value,
        vto: Value,
        ac_lambda: Value,
    ) -> (Value, Value) {
        let dc_lambda = self.mesa_ac_lambda(temp_k, None);
        self.mesa_level2_linearization_forward(vgs, vds, temp_k, vto, dc_lambda)
            .ac_conductances(ac_lambda)
    }

    pub(super) fn mesa_level3_ids(
        &self,
        vgs: Value,
        vds: Value,
        temp_k: Value,
        vto: Value,
        lambda: Value,
    ) -> Value {
        const Q_ELECTRON: Value = 1.602176634e-19;
        const EPSILONGAAS: Value = 12.244 * 8.85418e-12;

        let p = &self.params;
        let w = self.width.max(1e-12);
        let l = self.length.max(1e-12);
        let vt = self.thermal_voltage(temp_k).max(1e-12);
        let eta = p.eta.abs().max(1e-12);
        let etavth = (eta * vt).max(1e-12);
        let du = p.mesa_du.max(1e-12);
        let th = p.mesa_th.max(1e-12);
        let ndelta = p.mesa_ndelta.max(1e-30);
        let ndu = p.mesa_ndu.max(1e-30);
        let vpou = Q_ELECTRON * ndu * du * du / (2.0 * EPSILONGAAS);
        let vpod = Q_ELECTRON * ndelta * th * (2.0 * du + th) / (2.0 * EPSILONGAAS);
        let vpo = vpou + vpod;

        let sigma = p.sigma0.max(0.0)
            / (1.0 + Self::exp_limited((vgs - vto - p.hfet_vsigmat) / p.hfet_vsigma.max(1e-12)));
        let vgt = (vgs - vto) + sigma * vds;
        let t = vgt / vt - 1.0;
        let q = (p.hfet_delta.max(1e-9).powi(2) + t * t).sqrt();
        let vgte = 0.5 * vt * (2.0 + t + q);
        let a = 2.0 * p.beta.max(1e-30) * vgte;

        let nsa = if vgt > vpod {
            if vgte > vpo {
                ndelta * th + ndu * du
            } else {
                let r = ((vpo - vgte) / vpou.max(1e-30)).max(0.0).sqrt();
                ndelta * th + ndu * du * (1.0 - r)
            }
        } else if vpod - vgte < 0.0 {
            ndelta * th * (1.0 - du / th)
        } else {
            let r = (1.0 + ndu / ndelta * (vpod - vgte) / vpou.max(1e-30))
                .max(0.0)
                .sqrt();
            ndelta * th * (1.0 - du / th * (r - 1.0))
        };

        let b = Self::exp_limited(vgt / etavth);
        let nsb0 = (EPSILONGAAS * eta * vt / (Q_ELECTRON * (du + th))).max(1e-30);
        let nsb = nsb0 * b;
        let ns = if (nsa + nsb).abs() > 1e-30 {
            nsa * nsb / (nsa + nsb)
        } else {
            0.0
        };
        if !ns.is_finite() || ns < 1e-38 {
            return 0.0;
        }

        let rt = p.hfet_rsi.max(0.0) + p.hfet_rdi.max(0.0);
        let gchi0 = Q_ELECTRON * w / l * p.hfet_mu.max(1e-12);
        let gchi = gchi0 * ns;
        let gch = gchi / (1.0 + gchi * rt);
        if !gch.is_finite() || gch <= 0.0 {
            return 0.0;
        }

        let f = (1.0 + 2.0 * a * p.hfet_rsi.max(0.0)).sqrt();
        let d_term = 1.0 + a * p.hfet_rsi.max(0.0) + f;
        let e_term = 1.0 + p.mesa_tc * vgte;
        let isata = a * vgte / (d_term * e_term).max(1e-30);
        let n0 = (EPSILONGAAS * eta * vt / (Q_ELECTRON * du)).max(1e-30);
        let isatb0 = Q_ELECTRON * n0 * vt * w / l;
        let isatb = isatb0 * b;
        let isat = if (isata + isatb).abs() > 1e-30 {
            isata * isatb / (isata + isatb)
        } else {
            0.0
        };
        let vsate = (isat / gch).abs().max(1e-30);
        let m = p.hfet_m.max(1e-9);
        let g = (vds / vsate).max(0.0).powf(m);
        let h = (1.0 + g).powf(1.0 / m);
        let ids = gch * vds * (1.0 + lambda * vds) / h;
        if ids.is_finite() { ids } else { 0.0 }
    }

    pub(super) fn mesa_level4_ids(
        &self,
        vgs: Value,
        vds: Value,
        temp_k: Value,
        vto: Value,
        lambda: Value,
    ) -> Value {
        const Q_ELECTRON: Value = 1.602176634e-19;

        let p = &self.params;
        let w = self.width.max(1e-12);
        let l = self.length.max(1e-12);
        let vt = self.thermal_voltage(temp_k).max(1e-12);
        let eta = p.eta.abs().max(1e-12);
        let etavth = (eta * vt).max(1e-12);
        let d0 = p.hfet_di.max(1e-12);
        let sigma = p.sigma0.max(0.0)
            / (1.0 + Self::exp_limited((vgs - vto - p.hfet_vsigmat) / p.hfet_vsigma.max(1e-12)));
        let vgt = (vgs - vto) + sigma * vds;
        let u = 0.5 * vgt / vt - 1.0;
        let t = (p.hfet_delta.max(1e-9).powi(2) + u * u).sqrt();
        let vgte = vt * (2.0 + u + t);
        let b = Self::exp_limited(vgt / etavth);
        let n0 = (p.hfet_epsi.max(1e-30) * eta * vt / (2.0 * Q_ELECTRON * d0)).max(1e-30);
        let nsm = 2.0 * n0 * (1.0 + 0.5 * b).ln();
        if !nsm.is_finite() || nsm < 1e-38 {
            return 0.0;
        }

        let c = (nsm / p.hfet_nmax.max(1e-30))
            .max(0.0)
            .powf(p.hfet_gamma.max(1e-9));
        let q = (1.0 + c).powf(1.0 / p.hfet_gamma.max(1e-9));
        let ns = nsm / q;
        let gchi0 = Q_ELECTRON * w * p.hfet_mu.max(1e-12) / l;
        let gchi = gchi0 * ns;
        let rt = p.hfet_rsi.max(0.0) + p.hfet_rdi.max(0.0);
        let gch = gchi / (1.0 + gchi * rt);
        if !gch.is_finite() || gch <= 0.0 {
            return 0.0;
        }

        let gchim = gchi0 * nsm;
        let vl = (p.hfet_vs.max(1e-12) / p.hfet_mu.max(1e-12) * l).max(1e-30);
        let h = (1.0 + 2.0 * gchim * p.hfet_rsi.max(0.0) + vgte * vgte / (vl * vl)).sqrt();
        let p_denom = 1.0 + gchim * p.hfet_rsi.max(0.0) + h;
        let isatm = gchim * vgte / p_denom.max(1e-30);
        let imax = (Q_ELECTRON * p.hfet_nmax.max(1e-12) * p.hfet_vs.max(1e-12) * w).max(1e-30);
        let g = (isatm / imax).max(0.0).powf(p.hfet_gamma.max(1e-9));
        let isat = isatm / (1.0 + g).powf(1.0 / p.hfet_gamma.max(1e-9));
        let vsate = (isat / gch).abs().max(1e-30);
        let d = (vds / vsate).max(0.0).powf(p.hfet_m.max(1e-9));
        let e = (1.0 + d).powf(1.0 / p.hfet_m.max(1e-9));
        let ids = gch * vds * (1.0 + lambda * vds) / e;
        if ids.is_finite() { ids } else { 0.0 }
    }

    pub(super) fn mesa_ids_external(
        &self,
        vgs: Value,
        vds: Value,
        temp: Value,
        level: i32,
        force_inverse: bool,
        frequency_hz: Option<Value>,
    ) -> Value {
        let pol = self.jfet_type.polarity();
        let temp_k = if temp.is_finite() && temp > 0.0 {
            temp
        } else {
            self.params.tnom.max(1.0)
        };

        let mut vgs_int = pol * vgs;
        let mut vds_int = pol * vds;
        let mut inverse = false;
        if vds_int < 0.0 {
            vgs_int -= vds_int;
            vds_int = -vds_int;
            inverse = true;
        }
        if force_inverse && !inverse {
            // ngspice MESA legacy inverse latch forces subsequent devices down
            // the inverse-control path (use Vgd as channel control) while
            // keeping |Vds| for the core model equation.
            vgs_int -= vds_int;
            inverse = true;
        }

        let vto = pol * self.params.vto;
        let lambda = self.mesa_ac_lambda(temp_k, frequency_hz);
        let mut ids_int = match level {
            2 => self.mesa_level2_ids(vgs_int, vds_int, temp_k, vto, lambda),
            3 => self.mesa_level3_ids(vgs_int, vds_int, temp_k, vto, lambda),
            _ => self.mesa_level4_ids(vgs_int, vds_int, temp_k, vto, lambda),
        };
        if inverse {
            ids_int = -ids_int;
        }
        pol * ids_int
    }

    pub(super) fn calculate_mesa_level(
        &self,
        vgs: Value,
        vds: Value,
        temp: Value,
        level: i32,
        force_inverse: bool,
    ) -> (Value, Value, Value) {
        if level == 2 && !force_inverse {
            let pol = self.jfet_type.polarity();
            let vgs_int = pol * vgs;
            let vds_int = pol * vds;
            if vds_int >= 0.0 {
                let temp_k = if temp.is_finite() && temp > 0.0 {
                    temp
                } else {
                    self.params.tnom.max(1.0)
                };
                let vto = pol * self.params.vto;
                let lambda = self.mesa_ac_lambda(temp_k, None);
                let (ids_int, gm, gds) =
                    self.mesa_level2_small_signal_forward(vgs_int, vds_int, temp_k, vto, lambda);
                return (pol * ids_int, gm, gds);
            }
        }

        let ids = self.mesa_ids_external(vgs, vds, temp, level, force_inverse, None);

        let dvgs = (1e-8_f64).max(1e-6 * (1.0 + vgs.abs()));
        let dvds = (1e-8_f64).max(1e-6 * (1.0 + vds.abs()));
        let gm = (self.mesa_ids_external(vgs + dvgs, vds, temp, level, force_inverse, None)
            - self.mesa_ids_external(vgs - dvgs, vds, temp, level, force_inverse, None))
            / (2.0 * dvgs);
        let gds = (self.mesa_ids_external(vgs, vds + dvds, temp, level, force_inverse, None)
            - self.mesa_ids_external(vgs, vds - dvds, temp, level, force_inverse, None))
            / (2.0 * dvds);

        (
            if ids.is_finite() { ids } else { 0.0 },
            if gm.is_finite() { gm } else { 0.0 },
            if gds.is_finite() { gds } else { 0.0 },
        )
    }

    pub(super) fn calculate_mesa_level_ac(
        &self,
        vgs: Value,
        vds: Value,
        temp: Value,
        level: i32,
        force_inverse: bool,
        frequency_hz: Value,
    ) -> (Value, Value, Value) {
        let ids = self.mesa_ids_external(vgs, vds, temp, level, force_inverse, Some(frequency_hz));

        let dvgs = (1e-8_f64).max(1e-6 * (1.0 + vgs.abs()));
        let dvds = (1e-8_f64).max(1e-6 * (1.0 + vds.abs()));
        let gm = (self.mesa_ids_external(
            vgs + dvgs,
            vds,
            temp,
            level,
            force_inverse,
            Some(frequency_hz),
        ) - self.mesa_ids_external(
            vgs - dvgs,
            vds,
            temp,
            level,
            force_inverse,
            Some(frequency_hz),
        )) / (2.0 * dvgs);
        let gds = (self.mesa_ids_external(
            vgs,
            vds + dvds,
            temp,
            level,
            force_inverse,
            Some(frequency_hz),
        ) - self.mesa_ids_external(
            vgs,
            vds - dvds,
            temp,
            level,
            force_inverse,
            Some(frequency_hz),
        )) / (2.0 * dvds);

        (
            if ids.is_finite() { ids } else { 0.0 },
            if gm.is_finite() { gm } else { 0.0 },
            if gds.is_finite() { gds } else { 0.0 },
        )
    }

    pub(super) fn mesa_level2_capacitances_mode(
        &self,
        vgs: Value,
        vgd: Value,
        temp: Value,
        force_inverse: bool,
    ) -> (Value, Value) {
        const Q_ELECTRON: Value = 1.602176634e-19;
        const EPSILONGAAS: Value = 12.244 * 8.85418e-12;

        let pol = self.jfet_type.polarity();
        let p = &self.params;
        let w = self.width.max(1e-12);
        let l = self.length.max(1e-12);
        let temp_k = if temp.is_finite() && temp > 0.0 {
            temp
        } else {
            p.tnom.max(1.0)
        };
        let vt = self.thermal_voltage(temp_k).max(1e-12);
        let eta = p.eta.abs().max(1e-12);
        let etavth = (eta * vt).max(1e-12);

        let vgs_int = pol * vgs;
        let vgd_int = pol * vgd;
        let mut vds_int = vgs_int - vgd_int;
        let mut vgsch = vgs_int;
        let mut inverse = false;
        if vds_int < 0.0 {
            inverse = true;
            vds_int = -vds_int;
            vgsch = vgd_int;
        }
        if force_inverse && !inverse {
            inverse = true;
            vgsch = vgd_int;
        }

        let vto = pol * p.vto;
        let vgt0 = vgsch - vto;
        let sigma = p.sigma0.max(0.0)
            / (1.0 + Self::exp_limited((vgt0 - p.hfet_vsigmat) / p.hfet_vsigma.max(1e-12)));
        let vgt = vgt0 + sigma * vds_int;

        let mu = (p.hfet_mu + p.mesa_theta * vgt).max(1e-12);
        let d = p.hfet_di.max(1e-12);
        let nd = p.mesa_nd.max(1e-30);
        let vpo = Q_ELECTRON * nd * d * d / (2.0 * EPSILONGAAS);
        let vl = (p.hfet_vs.max(1e-12) / mu * l).max(1e-30);
        let beta_inst = 2.0 * EPSILONGAAS * p.hfet_vs.max(1e-12) * p.mesa_zeta.max(1e-12) * w / d;
        let beta = beta_inst / (vpo + 3.0 * vl).max(1e-30);

        let u = vgt / vt - 1.0;
        let t = (p.hfet_delta.max(1e-9).powi(2) + u * u).sqrt();
        let vgte = 0.5 * vt * (2.0 + u + t);
        let b = Self::exp_limited(-vgt / etavth);
        let n0 = (EPSILONGAAS * eta * vt / (Q_ELECTRON * d)).max(1e-30);
        let sqrt1 = if vgte >= vpo {
            0.0
        } else {
            (1.0 - vgte / vpo.max(1e-30)).max(0.0).sqrt()
        };
        let q = (1.0 - sqrt1).max(1e-30);
        let denom = 1.0 / (nd * d * q).max(1e-30) + b / n0;
        let ns = 1.0 / denom.max(1e-30);
        if !ns.is_finite() || ns < 1e-38 {
            let cf = 0.5 * EPSILONGAAS * w;
            return (cf, cf);
        }

        let rt = p.hfet_rsi.max(0.0) + p.hfet_rdi.max(0.0);
        let gchi0 = Q_ELECTRON * w / l;
        let gchi = gchi0 * mu * ns;
        let gch = gchi / (1.0 + gchi * rt);
        if !gch.is_finite() || gch <= 0.0 {
            let cf = 0.5 * EPSILONGAAS * w;
            return (cf, cf);
        }

        let a = 2.0 * beta * vgte;
        let f = (1.0 + 2.0 * a * p.hfet_rsi.max(0.0)).sqrt();
        let d_term = 1.0 + a * p.hfet_rsi.max(0.0) + f;
        let e_term = 1.0 + p.mesa_tc * vgte;
        let isata = a * vgte / (d_term * e_term).max(1e-30);
        let isatb0 = Q_ELECTRON * n0 * vt * w / l;
        let isatb = isatb0 * mu * Self::exp_limited(vgt / etavth);
        let isat = if (isata + isatb).abs() > 1e-30 {
            isata * isatb / (isata + isatb)
        } else {
            0.0
        };
        let vsate = (isat / gch).abs().max(1e-30);
        let vdse = vds_int
            * (1.0 + (vds_int / vsate).max(0.0).powf(p.hfet_mc.max(1e-9)))
                .powf(-1.0 / p.hfet_mc.max(1e-9));

        let cf = 0.5 * EPSILONGAAS * w;
        let temp_sqrt = if vgt > vpo {
            0.0
        } else {
            (1.0 - vgt / vpo.max(1e-30)).max(0.0).sqrt()
        };
        let cgc = w * l * EPSILONGAAS / ((temp_sqrt + b).max(1e-30)) / d;
        let c1_denom = (2.0 * vsate - vdse).max(1e-30);
        let c1 = ((vsate - vdse) / c1_denom).powi(2);
        let mut capgs = cf + (2.0 / 3.0) * cgc * (1.0 - c1);
        let c2 = (vsate / c1_denom).powi(2);
        let mut capgd = cf + (2.0 / 3.0) * cgc * (1.0 - c2);
        if inverse {
            std::mem::swap(&mut capgs, &mut capgd);
        }
        (
            if capgs.is_finite() {
                capgs.max(1e-18)
            } else {
                cf.max(1e-18)
            },
            if capgd.is_finite() {
                capgd.max(1e-18)
            } else {
                cf.max(1e-18)
            },
        )
    }

    /// Calculate HFET1-compatible channel current and Jacobian.
    ///
    /// Equations are ported from ngspice HFET1 (`hfetload.c` / `hfettemp.c`)
    /// for DC small-signal terms (`Ids`, `gm`, `gds`).
    pub(super) fn calculate_hfet1(
        &self,
        vgs: Value,
        vds: Value,
        temp: Value,
    ) -> (Value, Value, Value) {
        let (ids, gm, gds, _) = self.calculate_hfet1_core(vgs, vds, temp);
        (ids, gm, gds)
    }

    /// HFET1 GATEMOD=1 channel + gate evaluation; see
    /// [`Jfet::calculate_hfet1_core`].
    pub(super) fn calculate_hfet1_gatemod(
        &self,
        vgs: Value,
        vds: Value,
        temp: Value,
    ) -> (Value, Value, Value, Option<Hfet1GateEval>) {
        self.calculate_hfet1_core(vgs, vds, temp)
    }

    /// HFET1 channel evaluation, optionally with the GATEMOD=1 thermal
    /// gate model (hfetload.c hfeta, `gatemod != 0` sections). The gate
    /// branch values are returned in the internal (polarity-folded)
    /// frame, like `hfet_gate_branch`; with GATEMOD=1 the A1/A2
    /// correction currents are already folded into the channel terms.
    pub(super) fn calculate_hfet1_core(
        &self,
        vgs: Value,
        vds: Value,
        temp: Value,
    ) -> (Value, Value, Value, Option<Hfet1GateEval>) {
        const Q_ELECTRON: Value = 1.602176634e-19;
        let pol = self.jfet_type.polarity();
        let p = &self.params;

        let vgs_int = pol * vgs;
        let mut vds_int = pol * vds;
        let mut inverse = false;
        if vds_int < 0.0 {
            // ngspice HFET level-5 evaluates reverse Vds by flipping channel
            // current sign while keeping the controlling gate branch on Vgs.
            vds_int = -vds_int;
            inverse = true;
        }

        let temp_k = if temp.is_finite() && temp > 0.0 {
            temp
        } else {
            p.tnom.max(1.0)
        };
        let vt = self.thermal_voltage(temp_k).max(1e-6);
        let eta = p.eta.abs().max(1e-9);
        let etavth = (eta * vt).max(1e-12);

        let mu = p.hfet_mu.max(1e-12);
        let vs = p.hfet_vs.max(1e-12);
        let l = self.length.max(1e-12);
        let w = self.width.max(1e-12);
        let di = p.hfet_di.max(1e-12);
        let deltad = p.hfet_deltad.max(0.0);
        let nmax = p.hfet_nmax.max(1e-12);
        let gamma = p.hfet_gamma.max(1e-9);
        let m = p.hfet_m.max(1e-9);
        let sigma0 = p.sigma0.max(0.0);
        let vsigma = p.hfet_vsigma.max(1e-12);
        let vsigmat = p.hfet_vsigmat;
        let rsi = p.hfet_rsi.max(0.0);
        let rdi = p.hfet_rdi.max(0.0);
        let rt = rsi + rdi;

        let vto = pol * p.vto;

        let n0 = p.hfet_epsi.max(1e-30) * eta * vt / (2.0 * Q_ELECTRON * (di + deltad).max(1e-30));
        let gchi0 = Q_ELECTRON * w * mu / l;
        let imax = (Q_ELECTRON * nmax * vs * w).max(1e-30);
        let vl = (vs / mu * l).max(1e-30);

        let vgt0 = vgs_int - vto;
        let s = ((vgt0 - vsigmat) / vsigma).clamp(-80.0, 80.0).exp();
        let sigma = sigma0 / (1.0 + s);
        let vgt = vgt0 + sigma * vds_int;
        let u = 0.5 * vgt / vt - 1.0;
        let delta = p.hfet_delta.max(1e-9);
        let t = (delta * delta + u * u).sqrt();
        let vgte = vt * (2.0 + u + t);
        let b = (vgt / etavth).clamp(-80.0, 80.0).exp();
        let nsm = 2.0 * n0 * (1.0 + 0.5 * b).ln();
        if !nsm.is_finite() || nsm < 1e-38 {
            return (0.0, 0.0, 0.0, None);
        }

        let c = (nsm / nmax).max(0.0).powf(gamma);
        let q = (1.0 + c).powf(1.0 / gamma);
        let ns = nsm / q;
        let gchi = gchi0 * ns;
        let gch = gchi / (1.0 + gchi * rt);
        if !gch.is_finite() || gch <= 0.0 {
            return (0.0, 0.0, 0.0, None);
        }

        let gchim = gchi0 * nsm;
        let h = (1.0 + 2.0 * gchim * rsi + vgte * vgte / (vl * vl)).sqrt();
        let p_denom = 1.0 + gchim * rsi + h;
        if !p_denom.is_finite() || p_denom <= 0.0 {
            return (0.0, 0.0, 0.0, None);
        }

        let isatm = gchim * vgte / p_denom;
        let g = (isatm / imax).max(0.0).powf(gamma);
        let isat = isatm / (1.0 + g).powf(1.0 / gamma);
        if !isat.is_finite() {
            return (0.0, 0.0, 0.0, None);
        }

        let vsate = (isat / gch).abs().max(1e-30);
        let d = (vds_int / vsate).max(0.0).powf(m);
        let e = (1.0 + d).powf(1.0 / m);
        let delidgch = vds_int * (1.0 + p.lambda * vds_int) / e;
        let ids_fwd = gch * delidgch;

        let delidvsate = ids_fwd * d / (vsate * (1.0 + d));
        let dmd = if vds_int <= 0.0 {
            0.0
        } else {
            (vds_int / vsate).powf(m - 1.0)
        };
        let delidvds =
            gch * (1.0 + 2.0 * p.lambda * vds_int) / e - ids_fwd * dmd / (vsate * (1.0 + d));

        let a = 1.0 + gchi * rt;
        let delgchgchi = 1.0 / (a * a);
        let delgchins = gchi0;
        let delnsnsm = ns / nsm * (1.0 - c / (1.0 + c));
        let delvgtevgt = 0.5 * (1.0 + u / t.max(1e-30));
        let delnsmvgt = n0 / etavth / (1.0 / b + 0.5);
        let delvsateisat = 1.0 / gch;
        let delisatisatm = isat / isatm.max(1e-30) * (1.0 - g / (1.0 + g));
        let delisatmvgte =
            gchim * (p_denom - vgte * vgte / (vl * vl * h.max(1e-30))) / (p_denom * p_denom);
        let delvsategch = -vsate / gch;
        let delisatmgchim =
            vgte * (p_denom - gchim * rsi * (1.0 + 1.0 / h.max(1e-30))) / (p_denom * p_denom);
        let delvgtvgs = 1.0 - vds_int * sigma0 / vsigma * s / ((1.0 + s) * (1.0 + s));
        let p_chain = delgchgchi * delgchins * delnsnsm * delnsmvgt;
        let delvsatevgt = delvsateisat
            * delisatisatm
            * (delisatmvgte * delvgtevgt + delisatmgchim * gchi0 * delnsmvgt)
            + delvsategch * p_chain;
        let g_total = delidgch * p_chain + delidvsate * delvsatevgt;
        let mut ids_fwd = ids_fwd;
        let mut gm_fwd = g_total * delvgtvgs;
        let mut gds_fwd = delidvds + g_total * sigma;

        // GATEMOD=1: thermally self-consistent gate model
        // (hfetload.c hfeta, gatemod != 0 sections), evaluated at the
        // internal forward bias like the C reference.
        let gate = if self.params.hfet_gatemod {
            let astar = p.mesa_astar.max(0.0);
            let iso = astar * w * l / 2.0;
            let phib = p.hfet_phib.max(0.0);
            let k_boltzmann = crate::constants::K_BOLTZMANN;
            let temp_dev = temp_k;
            let csat = iso * temp_dev * temp_dev * (-phib / (k_boltzmann * temp_dev)).exp();
            let gmin = self.junction_gmin.max(0.0);

            // Gate-drain branch with channel-temperature feedback.
            let ck1 = p.hfet_ck1;
            let ck2 = p.hfet_ck2;
            let cm1 = p.hfet_cm1;
            let cm2 = p.hfet_cm2;
            let mt1 = p.hfet_mt1.max(1e-9);
            let mt2 = p.hfet_mt2.max(1e-9);
            let talpha = p.hfet_talpha;
            let m2d = p.hfet_m2d.max(1e-12);
            let m2s = p.hfet_m2s.max(1e-12);
            let koverq = k_boltzmann / crate::constants::Q_ELECTRON;

            let vkneet = (ck1 * vsate + ck2).max(1e-30);
            let vmax_t = (cm1 * vsate + cm2).max(1e-30);
            let a = (vds_int / vmax_t).max(0.0).powf(mt2);
            let b_fold = (1.0 + a).powf(1.0 / mt2);
            let vdse_t = vds_int / b_fold;
            let c_knee = (vdse_t / vkneet).max(0.0).powf(mt1);
            let d_knee = (1.0 + c_knee).powf(1.0 / mt1);
            let td = temp_dev + talpha * vdse_t * vdse_t / d_knee;
            let e_th = koverq * td * m2d;
            let p_th = phib / (k_boltzmann * td);
            let f_th = (-p_th).exp();
            let q_th = (vgs_int - vdse_t) / e_th;
            let g_th = q_th.clamp(-700.0, 700.0).exp();
            let h_th = iso * td * td * f_th * g_th;
            let cgd = h_th - csat;

            let delcgdvgs = h_th / e_th;
            let delcgdtd = h_th * (p_th - q_th + 2.0) / td;
            let deltdvdse = talpha * vdse_t * (2.0 - c_knee / (1.0 + c_knee)) / d_knee;
            let deltdvkneet = (td - temp_dev) * c_knee / ((1.0 + c_knee) * vkneet);
            let delvdsevmax = vdse_t * a / ((1.0 + a) * vmax_t);
            let delvdsevds = (1.0 - a / (1.0 + a)) / b_fold;
            let chain_g = delvsatevgt * delvgtvgs;
            let dvdsevgs = delvdsevmax * cm1 * chain_g;
            let dtdvgs = deltdvdse * dvdsevgs + deltdvkneet * ck1 * chain_g;
            let gmg = delcgdvgs + delcgdtd * dtdvgs;
            let chain_d = delvsatevgt * sigma;
            let dvdsevds_t = delvdsevds + delvdsevmax * cm1 * chain_d;
            let dtdvds = deltdvdse * dvdsevds_t + deltdvkneet * ck1 * chain_d;
            let gmd = -delcgdvgs * dvdsevds_t + delcgdtd * dtdvds;

            // Gate-source Schottky branch.
            let vtn = (vt * m2s).max(1e-30);
            let (cgs_g, ggs_g) = if vgs_int <= -5.0 * vt {
                let ggs_g = -csat / vgs_int + gmin;
                (ggs_g * vgs_int, ggs_g)
            } else {
                let evgs = (vgs_int / vtn).clamp(-700.0, 700.0).exp();
                (
                    csat * (evgs - 1.0) + gmin * vgs_int,
                    csat * evgs / vtn + gmin,
                )
            };

            // A1/A2 correction currents folded into the channel.
            let a1 = p.hfet_a1;
            let a2 = p.hfet_a2;
            if a1 != 0.0 || a2 != 0.0 {
                let mv1 = p.hfet_mv1.max(1e-9);
                let cm3 = p.hfet_cm3;
                let vmax_c = (cm3 * vsate).max(1e-30);
                let a_c = (vds_int / vmax_c).max(0.0).powf(mv1);
                let b_c = (1.0 + a_c).powf(1.0 / mv1);
                let vdse_c = vds_int / b_c;
                let delvdsevmax_c = vdse_c * a_c / ((1.0 + a_c) * vmax_c);
                let delvdsevds_c = (1.0 - a_c / (1.0 + a_c)) / b_c;
                let dvdsevgs_c = delvdsevmax_c * cm3 * delvsatevgt * delvgtvgs;
                let dvdsevds_c = delvdsevds_c + delvdsevmax_c * cm3 * delvsatevgt * sigma;
                let c_corr = vgte * vdse_c;
                let d_corr = 1.0 + a2 * c_corr;
                let e_corr = vdse_c * delvgtevgt;
                let f_corr = a2 * cgd;
                ids_fwd += a1 * (d_corr * cgd - cgs_g);
                gds_fwd += a1 * (d_corr * gmd + f_corr * (vgte * dvdsevds_c + e_corr * sigma));
                gm_fwd +=
                    a1 * (d_corr * gmg + f_corr * (vgte * dvdsevgs_c + e_corr * delvgtvgs) - ggs_g);
            }

            Some(Hfet1GateEval {
                cgs: cgs_g,
                ggs: ggs_g,
                cgd,
                gmg,
                gmd,
            })
        } else {
            None
        };

        let (mut ids, mut gm, mut gds) = if inverse {
            (-ids_fwd, gm_fwd, gds_fwd)
        } else {
            (ids_fwd, gm_fwd, gds_fwd)
        };

        if !ids.is_finite() {
            ids = 0.0;
        }
        if !gm.is_finite() {
            gm = 0.0;
        }
        if !gds.is_finite() {
            gds = 0.0;
        }

        (pol * ids, gm, gds, gate)
    }

    /// Calculate drain current and conductances
    ///
    /// Returns (Ids, gm, gds) where:
    /// - Ids: drain-source current
    /// - gm: transconductance dIds/dVgs
    /// - gds: output conductance dIds/dVds
    pub fn calculate(&self, vgs: Value, vds: Value, temp: Value) -> (Value, Value, Value) {
        let (temp_common, temp_source, _) = self.resolved_temperatures(temp);
        match self.params.channel_model {
            JfetChannelModel::ShichmanHodges => self.calculate_shichman_hodges(vgs, vds),
            JfetChannelModel::LegacyMesfet => self.calculate_legacy_mesfet(vgs, vds),
            JfetChannelModel::Hfet1 => match self.params.hfet_level {
                2 => self.calculate_mesa_level(vgs, vds, temp_source, 2, false),
                3 => self.calculate_mesa_level(vgs, vds, temp_source, 3, false),
                4 => self.calculate_mesa_level(vgs, vds, temp_source, 4, false),
                _ => self.calculate_hfet1(vgs, vds, temp_common),
            },
        }
    }
}

#[cfg(test)]
mod hfet1_tests {
    use super::*;

    #[test]
    fn hfet1_linear_region_current_matches_ngspice46() {
        // Oracle: ngspice-46 .op on `nmf(level=5 gatemod=0 vto=0.1)`,
        // z1 l=1u w=20u, vgs=0.4, vds=0.05 -> vd#branch = -2.68269e-4.
        let mut jfet = Jfet::njf("z1", 1, 2, 3).enable_hfet_model();
        jfet.params.vto = 0.1;
        let (ids, gm, gds) = jfet.calculate(0.4, 0.05, 300.15);
        assert!(
            (ids - 2.68269e-4).abs() <= 2.68269e-4 * 1.0e-3,
            "ids={ids:e} expected 2.68269e-4 (gm={gm:e} gds={gds:e})"
        );
    }

    #[test]
    fn hfet1_gatemod_internals_match_the_gdb_oracle() {
        // gdb on the ngspice debug build, break hfetload.c:736 running the
        // gatemod=1 pin deck (20u/1u, vto=0.1, a1=0.1, a2=0.01): at the
        // iterate vgs=0.6, vds=0.8 the reference computes
        // csat=1.4497771298665816e-10 (= ISO*T^2*exp(-PHIB/kT), ISO=4e-7),
        // cgd=1.2943481726683374e-7, gmg=1.8446754447778568e-6,
        // gmd=2.4718887023647735e-8 (all pre-correction).
        let mut jfet = Jfet::njf("z1", 1, 2, 3).enable_hfet_model();
        jfet.params.vto = 0.1;
        jfet.params.hfet_gatemod = true;
        jfet.params.hfet_a1 = 0.1;
        jfet.params.hfet_a2 = 0.01;

        let (_, _, _, gate) = jfet.calculate_hfet1_gatemod(0.6, 0.8, 300.15);
        let gate = gate.expect("gatemod evaluation present");

        let assert_close = |actual: Value, expected: Value, what: &str| {
            assert!(
                (actual - expected).abs() <= expected.abs() * 1.0e-5,
                "{what}: actual={actual:e} expected={expected:e}"
            );
        };
        assert_close(gate.cgd, 1.2943481726683374e-7, "cgd");
        assert_close(gate.gmg, 1.8446754447778568e-6, "gmg");
        assert_close(gate.gmd, 2.4718887023647735e-8, "gmd");
    }

    #[test]
    fn hfet1_operating_terms_match_the_raw_channel_current() {
        // Same oracle point through the full stamping wrapper: the channel
        // term must agree with calculate(); the engine-level 399uA on this
        // deck has to come from somewhere downstream.
        let mut jfet = Jfet::njf("z1", 1, 2, 3).enable_hfet_model();
        jfet.params.vto = 0.1;
        let (ids, _gm, _gds, igs, igd, _ggs, _ggd, vds_linear, _gmg, _gmd) =
            jfet.compute_operating_terms(0.4, 0.05, 0.35);
        assert!(
            (ids - 2.68269e-4).abs() <= 2.68269e-4 * 1.0e-3,
            "ids={ids:e} expected 2.68269e-4 (igs={igs:e} igd={igd:e} vds_linear={vds_linear})"
        );
        assert_eq!(vds_linear, 0.05);
    }
}
