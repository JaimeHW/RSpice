//! ngspice JFET2 Parker-Skellern short-channel model helpers.
//!
//! This module is a Rust port of the DC/charge portions of ngspice-46
//! `jfet2temp.c` and `psmodel.c`, adapted to RSpice's `Jfet` container and
//! external terminal sign convention.

use super::*;

const NG_CHARGE: Value = 1.602_176_620_8e-19;
const NG_BOLTZMANN: Value = 1.380_648_52e-23;
const NG_K_OVER_Q: Value = NG_BOLTZMANN / NG_CHARGE;
const NG_REFTEMP: Value = 300.15;
const FX: Value = -10.0;
const MX: Value = 40.0;
const EMX: Value = 2.353_852_668_370_199_8e17;

#[derive(Debug, Clone, Copy)]
pub(super) struct Jfet2OperatingTerms {
    pub ids: Value,
    pub gm: Value,
    pub gds: Value,
    pub igs: Value,
    pub igd: Value,
    pub ggs: Value,
    pub ggd: Value,
    pub power: Value,
}

#[derive(Debug, Clone, Copy)]
struct Jfet2TempTerms {
    temp: Value,
    t_sat_cur: Value,
    t_gate_pot: Value,
    t_cgs: Value,
    t_cgd: Value,
    cor_dep_cap: Value,
    xiwoo: Value,
    d3: Value,
    alpha: Value,
    za: Value,
}

#[derive(Debug, Clone, Copy)]
struct Jfet2RawTerms {
    idrain: Value,
    igs: Value,
    igd: Value,
    ggs: Value,
    ggd: Value,
    gm: Value,
    gds: Value,
    power: Value,
}

impl Jfet {
    #[inline]
    fn jfet2_hfgam(&self) -> Value {
        if self.params.jfet2_hfgam.is_finite() {
            self.params.jfet2_hfgam
        } else {
            self.params.jfet2_lfgam
        }
    }

    #[inline]
    fn jfet2_memory_weight(tau: Value, dt: Value) -> Value {
        if !dt.is_finite() || dt <= 0.0 {
            return 0.0;
        }
        let tau = tau.max(0.0);
        let denom = tau + 0.25 * dt.max(0.0);
        if denom > 0.0 {
            let ratio = tau / denom;
            let ratio2 = ratio * ratio;
            ratio2 * ratio2
        } else {
            0.0
        }
    }

    fn jfet2_temperature_terms(&self, ambient: Value) -> Jfet2TempTerms {
        let (temp, _, _) = self.resolved_temperatures(ambient);
        let temp = temp.max(1.0);
        let tnom = self.params.tnom.max(1.0);
        let vtnom = NG_K_OVER_Q * tnom;
        let fact1 = tnom / NG_REFTEMP;
        let kt1 = NG_BOLTZMANN * tnom;
        let egfet1 = 1.16 - (7.02e-4 * tnom * tnom) / (tnom + 1108.0);
        let arg1 = -egfet1 / (kt1 + kt1) + 1.115_087_7 / (NG_BOLTZMANN * (NG_REFTEMP + NG_REFTEMP));
        let pbfact1 = -2.0 * vtnom * (1.5 * fact1.ln() + NG_CHARGE * arg1);
        let pbo = (self.params.pb - pbfact1) / fact1;
        let gmaold = (self.params.pb - pbo) / pbo;
        let cjfact = 1.0 / (1.0 + 0.5 * (4.0e-4 * (tnom - NG_REFTEMP) - gmaold));

        let vt = temp * NG_K_OVER_Q;
        let fact2 = temp / NG_REFTEMP;
        let ratio1 = temp / tnom - 1.0;
        let t_sat_cur = self.params.is * (ratio1 * 1.11 / vt.max(1.0e-30)).exp();
        let mut t_cgs = self.params.cgs * cjfact;
        let mut t_cgd = self.params.cgd * cjfact;
        let kt = NG_BOLTZMANN * temp;
        let egfet = 1.16 - (7.02e-4 * temp * temp) / (temp + 1108.0);
        let arg = -egfet / (kt + kt) + 1.115_087_7 / (NG_BOLTZMANN * (NG_REFTEMP + NG_REFTEMP));
        let pbfact = -2.0 * vt * (1.5 * fact2.ln() + NG_CHARGE * arg);
        let t_gate_pot = fact2 * pbo + pbfact;
        let gmanew = (t_gate_pot - pbo) / pbo;
        let cjfact1 = 1.0 + 0.5 * (4.0e-4 * (temp - NG_REFTEMP) - gmanew);
        t_cgs *= cjfact1;
        t_cgd *= cjfact1;

        let cor_dep_cap = self.params.fc * t_gate_pot;
        let woo = (t_gate_pot - self.params.vto).max(1.0e-30);
        let xi = self.params.jfet2_xi.max(1.0e-30);
        let p = self.params.jfet2_p.max(1.0e-30);
        let q = self.params.jfet2_q.max(1.0e-30);
        let xiwoo = xi * woo;
        let z = self.params.jfet2_z.max(0.0);
        let za = (1.0 + z).sqrt() / 2.0;
        let alpha = xiwoo * xiwoo / ((xi + 1.0) * (xi + 1.0)) / 4.0;
        let d3 = p / q / woo.powf(p - q);

        Jfet2TempTerms {
            temp,
            t_sat_cur,
            t_gate_pot,
            t_cgs,
            t_cgd,
            cor_dep_cap,
            xiwoo,
            d3,
            alpha,
            za,
        }
    }

    fn jfet2_forward_junction(&self, voltage: Value, nvt: Value, isat: Value) -> (Value, Value) {
        let arg = voltage / nvt.max(1.0e-30);
        if arg > FX {
            if arg < MX {
                let zz = isat * arg.exp();
                (
                    zz - isat + self.junction_gmin * voltage,
                    zz / nvt + self.junction_gmin,
                )
            } else {
                let zz = isat * EMX;
                (
                    zz * (arg - MX + 1.0) - isat + self.junction_gmin * voltage,
                    zz / nvt + self.junction_gmin,
                )
            }
        } else {
            (-isat + self.junction_gmin * voltage, self.junction_gmin)
        }
    }

    fn jfet2_add_breakdown(
        &self,
        mut current: Value,
        mut conductance: Value,
        voltage: Value,
        ibd: Value,
    ) -> (Value, Value) {
        let vbd = self.params.jfet2_vbd.max(1.0e-30);
        let arg = -voltage / vbd;
        if arg > FX {
            if arg < MX {
                let zz = ibd * arg.exp();
                conductance += zz / vbd;
                current -= zz - ibd;
            } else {
                let zz = ibd * EMX;
                conductance += zz / vbd;
                current -= zz * (arg - MX + 1.0) - ibd;
            }
        } else {
            current += ibd;
        }
        (current, conductance)
    }

    fn jfet2_subthreshold_vgt(&self, vgst: Value, vdst: Value) -> Option<(Value, Value)> {
        let vst = self.params.jfet2_vst * (1.0 + self.params.jfet2_mvst * vdst);
        if vst == 0.0 {
            return (vgst > 0.0).then_some((vgst, 1.0));
        }

        if vgst <= FX * vst {
            return None;
        }

        if vgst > MX * vst {
            let subfac = EMX + 1.0;
            let vgt = (EMX / subfac) * (vgst - MX * vst) + MX * vst;
            Some((vgt, 1.0 - 1.0 / subfac))
        } else {
            let exp_term = (vgst / vst).exp();
            let subfac = 1.0 + exp_term;
            Some((vst * subfac.ln(), 1.0 - 1.0 / subfac))
        }
    }

    fn jfet2_ps_ids_raw_with_trap(
        &self,
        temp_terms: Jfet2TempTerms,
        vgs: Value,
        vgd: Value,
        vgstrap: Value,
        vgdtrap: Value,
        trap_h: Value,
        prev_power: Value,
        power_h: Value,
    ) -> Jfet2RawTerms {
        let area = self.area.max(0.0);
        let nvt = (temp_terms.temp * NG_K_OVER_Q * self.params.n.max(1.0e-30)).max(1.0e-30);
        let isat = temp_terms.t_sat_cur.max(0.0) * area;
        let (igs0, ggs0) = self.jfet2_forward_junction(vgs, nvt, isat);
        let (igd0, ggd0) = self.jfet2_forward_junction(vgd, nvt, isat);
        let ibd = self.params.jfet2_ibd.max(0.0) * area;
        let (igs, ggs) = self.jfet2_add_breakdown(igs0, ggs0, vgs, ibd);
        let (igd, ggd) = self.jfet2_add_breakdown(igd0, ggd0, vgd, ibd);

        let vdst = vgs - vgd;
        let mut idrain = 0.0;
        let mut gm = 0.0;
        let mut gds = 0.0;

        let dvgs = vgstrap - vgs;
        let dvgd = vgdtrap - vgd;
        let lfgam = self.params.jfet2_lfgam;
        let lfg1 = self.params.jfet2_lfg1;
        let lfg2 = self.params.jfet2_lfg2;
        let hfeta = self.params.jfet2_hfeta;
        let hfe2 = self.params.jfet2_hfe2;
        let hfgam = self.jfet2_hfgam();
        let hfe1 = self.params.jfet2_hfe1;
        let hfg1 = self.params.jfet2_hfg1;
        let hfg2 = self.params.jfet2_hfg2;

        let mut vgst = self.params.vto;
        vgst = vgs - vgst;
        vgst -= (lfgam - lfg1 * vgstrap + lfg2 * vgdtrap) * vgdtrap;
        let eta = hfeta - hfe1 * vgdtrap + hfe2 * vgstrap;
        let gam = hfgam - hfg1 * vgstrap + hfg2 * vgdtrap;
        vgst += eta * dvgs;
        vgst += gam * dvgd;

        if let Some((vgt, sub_arg)) = self.jfet2_subthreshold_vgt(vgst, vdst) {
            let q = self.params.jfet2_q.max(1.0e-30);
            let p_minus_q = self.params.jfet2_p - q;
            let dvpd_dvdst = temp_terms.d3 * vgt.powf(p_minus_q);
            let vdp = vdst * dvpd_dvdst;
            let vsat_fac = vgt / (self.params.jfet2_mxi * vgt + temp_terms.xiwoo).max(1.0e-30);
            let vsat = vgt / (1.0 + vsat_fac);
            let aa = temp_terms.za * vdp + vsat / 2.0;
            let a_aa = aa - vsat;
            let knee = vsat * vsat * self.params.jfet2_z / 4.0;
            let rpt = (aa * aa + knee).sqrt();
            let a_rpt = (a_aa * a_aa + knee).sqrt();
            let vdt = rpt - a_rpt;
            let dvdt_dvdp = temp_terms.za * (aa / rpt.max(1.0e-30) - a_aa / a_rpt.max(1.0e-30));
            let dvdt_dvgt = (vdt - vdp * dvdt_dvdp)
                * (1.0 + self.params.jfet2_mxi * vsat_fac * vsat_fac)
                / (1.0 + vsat_fac)
                / vgt.max(1.0e-30);

            let drain_power = (vgt - vdt).max(0.0).powf(q - 1.0);
            let gate_power = vgt.powf(q - 1.0);
            idrain = vdt * drain_power + vgt * (gate_power - drain_power);
            gds = drain_power * q;
            gm = (gate_power - drain_power) * q;
            gm += gds * dvdt_dvgt;
            gds *= dvdt_dvdp;
            gm += gds * p_minus_q * vdp / vgt.max(1.0e-30);
            gds *= dvpd_dvdst;

            let vst = self.params.jfet2_vst * (1.0 + self.params.jfet2_mvst * vdst);
            if vst != 0.0 {
                gds += gm * self.params.jfet2_vst * self.params.jfet2_mvst * (vgt - vgst * sub_arg)
                    / vst;
            }
            gm *= sub_arg;
        }

        let feedback = trap_h * gam
            + (1.0 - trap_h)
                * (hfe1 * dvgs - hfg2 * dvgd + 2.0 * lfg2 * vgdtrap - lfg1 * vgstrap + lfgam);
        gds += gm * feedback;
        gm *= 1.0 - trap_h * eta + (1.0 - trap_h) * (hfe2 * dvgs - hfg1 * dvgd + lfg1 * vgdtrap)
            - feedback;

        let beta = self.params.beta.max(0.0) * area;
        let beta_scale = beta * (1.0 + self.params.lambda * vdst);
        gm *= beta_scale;
        gds = beta * self.params.lambda * idrain + gds * beta_scale;
        idrain *= beta_scale;

        let delta = if area > 0.0 {
            self.params.jfet2_delta / area
        } else {
            0.0
        };
        let p_average = power_h * prev_power + (1.0 - power_h) * vdst * idrain;
        let pfac = 1.0 + p_average * delta;
        if pfac.abs() > 1.0e-30 {
            idrain /= pfac;
            let scale = (power_h * delta * prev_power + 1.0) / (pfac * pfac);
            gm *= scale;
            gds = gds * scale - (1.0 - power_h) * delta * idrain * idrain;
        }

        Jfet2RawTerms {
            idrain: finite_or_zero(idrain),
            igs: finite_or_zero(igs),
            igd: finite_or_zero(igd),
            ggs: finite_or_zero(ggs),
            ggd: finite_or_zero(ggd),
            gm: finite_or_zero(gm),
            gds: finite_or_zero(gds),
            power: finite_or_zero(p_average),
        }
    }

    fn jfet2_ps_ids_raw(
        &self,
        temp_terms: Jfet2TempTerms,
        vgs: Value,
        vgd: Value,
    ) -> Jfet2RawTerms {
        self.jfet2_ps_ids_raw_with_trap(temp_terms, vgs, vgd, vgs, vgd, 0.0, 0.0, 0.0)
    }

    pub(super) fn jfet2_operating_terms(
        &self,
        vgs: Value,
        vds: Value,
        vgd: Value,
        temp: Value,
    ) -> Jfet2OperatingTerms {
        let pol = self.jfet_type.polarity();
        let temp_terms = self.jfet2_temperature_terms(temp);
        let vgs_int = pol * vgs;
        let vgd_int = pol * vgd;
        let vds_int = pol * vds;

        let (ids_int, igs_int, igd_int, ggs, ggd, gm, gds, power) = if vds_int < 0.0 {
            let raw = self.jfet2_ps_ids_raw(temp_terms, vgd_int, vgs_int);
            (
                -raw.idrain,
                raw.igd,
                raw.igs,
                raw.ggd,
                raw.ggs,
                -raw.gm,
                raw.gds + raw.gm,
                raw.power,
            )
        } else {
            let raw = self.jfet2_ps_ids_raw(temp_terms, vgs_int, vgd_int);
            (
                raw.idrain, raw.igs, raw.igd, raw.ggs, raw.ggd, raw.gm, raw.gds, raw.power,
            )
        };

        let m = self.m.max(0.0);
        Jfet2OperatingTerms {
            ids: finite_or_zero(pol * ids_int * m),
            gm: finite_or_zero(gm * m),
            gds: finite_or_zero(gds * m),
            igs: finite_or_zero(pol * igs_int * m),
            igd: finite_or_zero(pol * igd_int * m),
            ggs: finite_or_zero(ggs * m),
            ggd: finite_or_zero(ggd * m),
            power: finite_or_zero(power),
        }
    }

    pub(super) fn jfet2_operating_terms_with_trap(
        &self,
        vgs: Value,
        vds: Value,
        vgd: Value,
        temp: Value,
        vgstrap: Value,
        vgdtrap: Value,
        trap_h: Value,
        prev_power: Value,
        power_h: Value,
    ) -> Jfet2OperatingTerms {
        let pol = self.jfet_type.polarity();
        let temp_terms = self.jfet2_temperature_terms(temp);
        let vgs_int = pol * vgs;
        let vgd_int = pol * vgd;
        let vds_int = pol * vds;
        let vgstrap_int = pol * vgstrap;
        let vgdtrap_int = pol * vgdtrap;

        let (ids_int, igs_int, igd_int, ggs, ggd, gm, gds, power) = if vds_int < 0.0 {
            let raw = self.jfet2_ps_ids_raw_with_trap(
                temp_terms,
                vgd_int,
                vgs_int,
                vgdtrap_int,
                vgstrap_int,
                trap_h,
                prev_power,
                power_h,
            );
            (
                -raw.idrain,
                raw.igd,
                raw.igs,
                raw.ggd,
                raw.ggs,
                -raw.gm,
                raw.gds + raw.gm,
                raw.power,
            )
        } else {
            let raw = self.jfet2_ps_ids_raw_with_trap(
                temp_terms,
                vgs_int,
                vgd_int,
                vgstrap_int,
                vgdtrap_int,
                trap_h,
                prev_power,
                power_h,
            );
            (
                raw.idrain, raw.igs, raw.igd, raw.ggs, raw.ggd, raw.gm, raw.gds, raw.power,
            )
        };

        let m = self.m.max(0.0);
        Jfet2OperatingTerms {
            ids: finite_or_zero(pol * ids_int * m),
            gm: finite_or_zero(gm * m),
            gds: finite_or_zero(gds * m),
            igs: finite_or_zero(pol * igs_int * m),
            igd: finite_or_zero(pol * igd_int * m),
            ggs: finite_or_zero(ggs * m),
            ggd: finite_or_zero(ggd * m),
            power: finite_or_zero(power),
        }
    }

    pub(crate) fn jfet2_next_transient_memory(
        &self,
        vgs: Value,
        vgd: Value,
        prev_vgstrap: Value,
        prev_vgdtrap: Value,
        prev_power: Value,
        dt: Value,
    ) -> (Value, Value, Value) {
        if !matches!(self.params.channel_model, JfetChannelModel::ParkerSkellern) {
            return (vgs, vgd, 0.0);
        }

        let trap_h = Self::jfet2_memory_weight(self.params.jfet2_taug, dt);
        let power_h = Self::jfet2_memory_weight(self.params.jfet2_taud, dt);
        let vgstrap = finite_or_zero(trap_h * prev_vgstrap + (1.0 - trap_h) * vgs);
        let vgdtrap = finite_or_zero(trap_h * prev_vgdtrap + (1.0 - trap_h) * vgd);
        let terms = self.jfet2_operating_terms_with_trap(
            vgs,
            vgs - vgd,
            vgd,
            self.analysis_temperature(),
            vgstrap,
            vgdtrap,
            trap_h,
            prev_power,
            power_h,
        );
        (vgstrap, vgdtrap, terms.power)
    }

    pub(crate) fn refresh_jfet2_transient_operating_terms(
        &mut self,
        voltages: &[Value],
        prev_vgstrap: Value,
        prev_vgdtrap: Value,
        prev_power: Value,
        dt: Value,
    ) {
        if !matches!(self.params.channel_model, JfetChannelModel::ParkerSkellern) {
            return;
        }

        let (vgs, vds, vgd) = self.state_or_raw_branch_voltages(voltages);
        let trap_h = Self::jfet2_memory_weight(self.params.jfet2_taug, dt);
        let power_h = Self::jfet2_memory_weight(self.params.jfet2_taud, dt);
        let vgstrap = finite_or_zero(trap_h * prev_vgstrap + (1.0 - trap_h) * vgs);
        let vgdtrap = finite_or_zero(trap_h * prev_vgdtrap + (1.0 - trap_h) * vgd);
        let terms = self.jfet2_operating_terms_with_trap(
            vgs,
            vds,
            vgd,
            self.analysis_temperature(),
            vgstrap,
            vgdtrap,
            trap_h,
            prev_power,
            power_h,
        );

        self.eval_ids = terms.ids;
        self.eval_gm = terms.gm;
        self.eval_gds = terms.gds;
        self.eval_igs = terms.igs;
        self.eval_igd = terms.igd;
        self.eval_ggs = terms.ggs;
        self.eval_ggd = terms.ggd;
        self.eval_gmg = 0.0;
        self.eval_gmd = 0.0;
        self.eval_vds_linear = vds;
        self.lin_vgs = vgs;
        self.lin_vgd = vgd;
        self.lin_cg = terms.igs + terms.igd;
        self.lin_cd = terms.ids - terms.igd;
        self.eval_valid = true;
    }

    pub(super) fn jfet2_limited_branch_voltages(
        &self,
        vgs_new: Value,
        vgd_new: Value,
    ) -> (Value, Value) {
        let pol = self.jfet_type.polarity();
        if !pol.is_finite() || pol.abs() < 0.5 {
            return (vgs_new, vgd_new);
        }

        if !self.vgs.is_finite() || !self.vds.is_finite() {
            let seed = -1.0 / pol;
            return (seed, seed);
        }

        let temp_terms = self.jfet2_temperature_terms(self.analysis_temperature());
        let vt = (temp_terms.temp * NG_K_OVER_Q).max(1.0e-12);
        let isat = temp_terms.t_sat_cur.max(0.0);
        let vcrit = if isat > 0.0 {
            vt * (vt / (core::f64::consts::SQRT_2 * isat)).max(1.0).ln()
        } else {
            1.0
        };

        let vgs_old_int = pol * self.vgs;
        let vgd_old_int = pol * (self.vgs - self.vds);
        let mut vgs_int = pol * vgs_new;
        let mut vgd_int = pol * vgd_new;

        vgs_int = Self::pnjlim(vgs_int, vgs_old_int, vt, vcrit);
        vgd_int = Self::pnjlim(vgd_int, vgd_old_int, vt, vcrit);
        vgs_int = Self::fetlim(vgs_int, vgs_old_int, self.params.vto);
        vgd_int = Self::fetlim(vgd_int, vgd_old_int, self.params.vto);

        (vgs_int / pol, vgd_int / pol)
    }

    pub(super) fn xyce_jfet2_limited_branch_voltages(
        &self,
        vgs_new: Value,
        vgd_new: Value,
    ) -> (Value, Value) {
        let pol = self.jfet_type.polarity();
        if !pol.is_finite() || pol.abs() < 0.5 {
            return (vgs_new, vgd_new);
        }

        if !self.vgs.is_finite() || !self.vds.is_finite() {
            return (0.0, 0.0);
        }

        let temp_terms = self.jfet2_temperature_terms(self.analysis_temperature());
        let vt = (temp_terms.temp * NG_K_OVER_Q).max(1.0e-12);
        let isat = temp_terms.t_sat_cur.max(0.0) * self.area.max(0.0);
        let vcrit = if isat > 0.0 {
            vt * (vt / (core::f64::consts::SQRT_2 * isat)).max(1.0).ln()
        } else {
            1.0
        };

        let vgs_old_int = pol * self.vgs;
        let vgd_old_int = pol * (self.vgs - self.vds);
        let mut vgs_int = pol * vgs_new;
        let mut vgd_int = pol * vgd_new;

        vgs_int = Self::pnjlim(vgs_int, vgs_old_int, vt, vcrit);
        vgd_int = Self::pnjlim(vgd_int, vgd_old_int, vt, vcrit);
        vgs_int = Self::fetlim(vgs_int, vgs_old_int, self.params.vto);
        vgd_int = Self::fetlim(vgd_int, vgd_old_int, self.params.vto);

        (vgs_int / pol, vgd_int / pol)
    }

    pub(super) fn jfet2_gate_junctions(
        &self,
        vgs: Value,
        vgd: Value,
        temp: Value,
    ) -> (Value, Value, Value, Value) {
        let terms = self.jfet2_operating_terms(vgs, vgs - vgd, vgd, temp);
        (terms.igs, terms.igd, terms.ggs, terms.ggd)
    }

    fn xyce_jfet2_gate_branch(&self, voltage: Value, vt: Value, csat: Value) -> (Value, Value) {
        let vt = vt.max(1.0e-30);
        let vtf = 5.0 * vt;
        if voltage <= -vtf {
            let g = if voltage.abs() > 1.0e-30 {
                -csat / voltage + self.junction_gmin
            } else {
                self.junction_gmin
            };
            (g * voltage, g)
        } else {
            let ev = (voltage / vt).exp();
            (
                csat * (ev - 1.0) + self.junction_gmin * voltage,
                csat * ev / vt + self.junction_gmin,
            )
        }
    }

    fn xyce_jfet2_channel_raw(
        &self,
        vgs: Value,
        vds: Value,
        external_vd: Option<Value>,
        external_vs: Option<Value>,
    ) -> (Value, Value, Value) {
        let beta = self.params.beta.max(0.0) * self.area.max(0.0);
        let lambda = self.params.lambda;
        let theta = self.params.jfet2_theta;
        let delta = self.params.jfet2_delta;
        let vto = self.params.vto;
        let pb = self.params.pb;
        let a = pb - vto;
        let two_a_over_three = 2.0 * a / 3.0;
        let safe_a = if a.abs() > 1.0e-30 { a } else { 1.0 };

        if vds >= 0.0 {
            let vgst = vgs - vto;
            if vgst <= 0.0 {
                return (0.0, 0.0, 0.0);
            }

            let b = if a.abs() > 1.0e-30 {
                (a - vgst) / safe_a
            } else {
                0.0
            };
            let b12 = if b >= 0.0 { b.sqrt() } else { 0.0 };
            let denom = (1.0 + theta * vgst).max(1.0e-30);
            let betap = beta * (1.0 + lambda * vds) / denom;
            let vdsat = (vgst + delta).max(1.0e-30);

            if external_vd.unwrap_or(vds) <= vdsat {
                let c = if a.abs() > 1.0e-30 {
                    (vds - delta * vds / vdsat + a - vgst) / safe_a
                } else {
                    0.0
                };
                let c12 = if c >= 0.0 { c.sqrt() } else { 0.0 };
                let d = c * c12 - b * b12;
                let cdrain = betap * (vds - delta * vds / vdsat - two_a_over_three * d);
                let gm = -cdrain * theta / denom
                    + betap * (c12 - b12)
                    + betap * delta * (1.0 - c12) * vds / (vdsat * vdsat);
                let gds = if (1.0 + lambda * vds).abs() > 1.0e-30 {
                    lambda * cdrain / (1.0 + lambda * vds)
                } else {
                    0.0
                } + betap * (1.0 - (1.0 - c12) * delta / vdsat - c12);
                (cdrain, gm, gds)
            } else {
                let cdrain = betap * (vgst - two_a_over_three * (1.0 - b * b12));
                let gm = betap * (1.0 - b12) - theta * cdrain / denom;
                let gds = if (1.0 + lambda * vds).abs() > 1.0e-30 {
                    lambda * cdrain / (1.0 + lambda * vds)
                } else {
                    0.0
                };
                (cdrain, gm, gds)
            }
        } else {
            let vgd = vgs - vds;
            let vgdt = vgd - vto;
            if vgdt <= 0.0 {
                return (0.0, 0.0, 0.0);
            }

            let b = if a.abs() > 1.0e-30 {
                (a - vgdt) / safe_a
            } else {
                0.0
            };
            let b12 = if b >= 0.0 { b.sqrt() } else { 0.0 };
            let denom = (1.0 + theta * vgdt).max(1.0e-30);
            let betap = beta * (1.0 - lambda * vds) / denom;
            let vdsat = (vgdt + delta).max(1.0e-30);

            if external_vs.unwrap_or(-vds) <= vdsat {
                let c = if a.abs() > 1.0e-30 {
                    (-vds + delta * vds / vdsat + a - vgdt) / safe_a
                } else {
                    0.0
                };
                let c12 = if c >= 0.0 { c.sqrt() } else { 0.0 };
                let d = c * c12 - b * b12;
                let cdrain = -betap * (-vds + delta * vds / vdsat - two_a_over_three * d);
                let gm = -betap * (c12 - b12) - theta * cdrain / denom
                    + betap * (1.0 - c12) * delta * vds / (vdsat * vdsat);
                let gds = betap * (1.0 - (1.0 - c12) * delta / vdsat - c12)
                    - if (1.0 - lambda * vds).abs() > 1.0e-30 {
                        lambda * cdrain / (1.0 - lambda * vds)
                    } else {
                        0.0
                    };
                (cdrain, gm, gds)
            } else {
                let cdrain = -betap * (vgdt - two_a_over_three * (1.0 - b * b12));
                let gm = -betap * (1.0 - b12) - theta * cdrain / denom;
                let gds = if (1.0 - lambda * vds).abs() > 1.0e-30 {
                    lambda * cdrain / (1.0 - lambda * vds)
                } else {
                    0.0
                };
                (cdrain, gm, gds)
            }
        }
    }

    pub(super) fn xyce_jfet2_operating_terms(
        &self,
        vgs: Value,
        vds: Value,
        vgd: Value,
        temp: Value,
    ) -> Jfet2OperatingTerms {
        self.xyce_jfet2_operating_terms_with_optional_terminals(vgs, vds, vgd, temp, None, None)
    }

    pub(super) fn xyce_jfet2_operating_terms_with_terminals(
        &self,
        vgs: Value,
        vds: Value,
        vgd: Value,
        temp: Value,
        external_vd: Value,
        external_vs: Value,
    ) -> Jfet2OperatingTerms {
        self.xyce_jfet2_operating_terms_with_optional_terminals(
            vgs,
            vds,
            vgd,
            temp,
            Some(external_vd),
            Some(external_vs),
        )
    }

    fn xyce_jfet2_operating_terms_with_optional_terminals(
        &self,
        vgs: Value,
        vds: Value,
        vgd: Value,
        temp: Value,
        external_vd: Option<Value>,
        external_vs: Option<Value>,
    ) -> Jfet2OperatingTerms {
        let pol = self.jfet_type.polarity();
        let temp_terms = self.jfet2_temperature_terms(temp);
        let vt = (temp_terms.temp * NG_K_OVER_Q).max(1.0e-30);
        let csat = temp_terms.t_sat_cur.max(0.0) * self.area.max(0.0);

        let vgs_int = pol * vgs;
        let vgd_int = pol * vgd;
        let vds_int = pol * vds;
        let external_vd_int = external_vd.map(|v| pol * v);
        let external_vs_int = external_vs.map(|v| pol * v);
        let (igs_int, ggs) = self.xyce_jfet2_gate_branch(vgs_int, vt, csat);
        let (igd_int, ggd) = self.xyce_jfet2_gate_branch(vgd_int, vt, csat);
        let (ids_int, gm, gds) =
            self.xyce_jfet2_channel_raw(vgs_int, vds_int, external_vd_int, external_vs_int);

        let m = self.m.max(0.0);
        Jfet2OperatingTerms {
            ids: finite_or_zero(pol * ids_int * m),
            gm: finite_or_zero(gm * m),
            gds: finite_or_zero(gds * m),
            igs: finite_or_zero(pol * igs_int * m),
            igd: finite_or_zero(pol * igd_int * m),
            ggs: finite_or_zero(ggs * m),
            ggd: finite_or_zero(ggd * m),
            power: finite_or_zero(vds_int * ids_int),
        }
    }

    pub(super) fn xyce_jfet2_gate_junctions(
        &self,
        vgs: Value,
        vgd: Value,
        temp: Value,
    ) -> (Value, Value, Value, Value) {
        let terms = self.xyce_jfet2_operating_terms(vgs, vgs - vgd, vgd, temp);
        (terms.igs, terms.igd, terms.ggs, terms.ggd)
    }

    fn jfet2_qgg(
        vgs: Value,
        vgd: Value,
        gamma: Value,
        pb: Value,
        alpha: Value,
        vto: Value,
        vmax: Value,
        xc: Value,
        cgso: Value,
        cgdo: Value,
    ) -> (Value, Value, Value) {
        let vds = vgs - vgd;
        let d1_xc = 1.0 - xc;
        let vert = (vds * vds + alpha).sqrt().max(1.0e-30);
        let veff = 0.5 * (vgs + vgd + vert) + gamma * vds;
        let vnr = d1_xc * (veff - vto);
        let vnrt = (vnr * vnr + 0.04).sqrt();
        let vnew = veff + 0.5 * (vnrt - vnr);

        let (qrt, ext, cgso_eff, cpm) = if vnew < vmax {
            let qrt = (1.0 - vnew / pb).sqrt().max(1.0e-30);
            (
                qrt,
                0.0,
                0.5 * cgso / qrt * (1.0 + xc + d1_xc * vnr / vnrt),
                vds / vert,
            )
        } else {
            let qrt = (1.0 - vmax / pb).sqrt().max(1.0e-30);
            let vx = 0.5 * (vnew - vmax);
            let par = 1.0 + vx / (pb - vmax).max(1.0e-30);
            (
                qrt,
                vx * (1.0 + par) / qrt,
                0.5 * cgso / qrt * (1.0 + xc + d1_xc * vnr / vnrt) * par,
                vds / vert,
            )
        };
        let cplus = 0.5 * (1.0 + cpm);
        let cminus = cplus - cpm;
        let cgs = cgso_eff * (cplus + gamma) + cgdo * (cminus + gamma);
        let cgd = cgso_eff * (cminus - gamma) + cgdo * (cplus - gamma);
        let qgg = cgso * ((pb + pb) * (1.0 - qrt) + ext) + cgdo * (veff - vert);
        (
            finite_or_zero(qgg),
            finite_or_zero(cgs),
            finite_or_zero(cgd),
        )
    }

    pub(super) fn jfet2_capacitances(&self, vgs: Value, vgd: Value, temp: Value) -> (Value, Value) {
        let pol = self.jfet_type.polarity();
        let temp_terms = self.jfet2_temperature_terms(temp);
        let scale = self.area.max(0.0) * self.m.max(0.0);
        let (_, cgs, cgd) = Self::jfet2_qgg(
            pol * vgs,
            pol * vgd,
            self.params.jfet2_acgam,
            temp_terms.t_gate_pot,
            temp_terms.alpha,
            self.params.vto,
            temp_terms.cor_dep_cap,
            self.params.jfet2_xc,
            temp_terms.t_cgs * scale,
            temp_terms.t_cgd * scale,
        );
        (cgs, cgd)
    }

    fn xyce_jfet2_junction_charge(
        voltage: Value,
        zero_bias_cap: Value,
        gate_potential: Value,
        forward_coeff: Value,
    ) -> (Value, Value) {
        if !zero_bias_cap.is_finite() || zero_bias_cap <= 0.0 {
            return (0.0, 0.0);
        }
        let pb = gate_potential.max(1.0e-30);
        let fc = forward_coeff.clamp(0.0, 0.95);
        let one_minus_fc = (1.0 - fc).max(1.0e-30);
        let dep_cap = fc * pb;
        if voltage < dep_cap {
            let sqrt_arg = (1.0 - voltage / pb).max(1.0e-30).sqrt();
            let q = 2.0 * pb * zero_bias_cap * (1.0 - sqrt_arg);
            let cap = zero_bias_cap / sqrt_arg;
            return (finite_or_zero(q), finite_or_zero(cap));
        }

        let f1 = 2.0 * pb * (1.0 - one_minus_fc.sqrt());
        let f2 = one_minus_fc.powf(1.5);
        let f3 = 1.0 - 1.5 * fc;
        let q = zero_bias_cap * f1
            + (zero_bias_cap / f2)
                * (f3 * (voltage - dep_cap) + (voltage * voltage - dep_cap * dep_cap) / (4.0 * pb));
        let cap = (zero_bias_cap / f2) * (f3 + voltage / (2.0 * pb));
        (finite_or_zero(q), finite_or_zero(cap))
    }

    pub(crate) fn xyce_jfet2_charge_state(
        &self,
        vgs: Value,
        vgd: Value,
        temp: Value,
    ) -> Jfet2ChargeState {
        let pol = self.jfet_type.polarity();
        let temp_terms = self.jfet2_temperature_terms(temp);
        let scale = self.area.max(0.0) * self.m.max(0.0);
        let vgs_int = pol * vgs;
        let vgd_int = pol * vgd;
        let (qgs_int, cgs) = Self::xyce_jfet2_junction_charge(
            vgs_int,
            temp_terms.t_cgs * scale,
            temp_terms.t_gate_pot,
            self.params.fc,
        );
        let (qgd_int, cgd) = Self::xyce_jfet2_junction_charge(
            vgd_int,
            temp_terms.t_cgd * scale,
            temp_terms.t_gate_pot,
            self.params.fc,
        );

        Jfet2ChargeState {
            qgs: finite_or_zero(pol * qgs_int),
            qgd: finite_or_zero(pol * qgd_int),
            cgs,
            cgd,
        }
    }

    pub(crate) fn analytic_gate_charge_state(
        &self,
        vgs: Value,
        vgd: Value,
        temp: Value,
        previous: Option<(Value, Value, Value, Value)>,
    ) -> Option<Jfet2ChargeState> {
        match self.params.channel_model {
            JfetChannelModel::ParkerSkellern => {
                Some(self.jfet2_charge_state(vgs, vgd, temp, previous))
            }
            JfetChannelModel::XyceModifiedShockley => {
                Some(self.xyce_jfet2_charge_state(vgs, vgd, temp))
            }
            _ => None,
        }
    }

    pub(crate) fn jfet2_charge_state(
        &self,
        vgs: Value,
        vgd: Value,
        temp: Value,
        previous: Option<(Value, Value, Value, Value)>,
    ) -> Jfet2ChargeState {
        let pol = self.jfet_type.polarity();
        let temp_terms = self.jfet2_temperature_terms(temp);
        let scale = self.area.max(0.0) * self.m.max(0.0);
        let qgg = |vgs_int: Value, vgd_int: Value| {
            Self::jfet2_qgg(
                vgs_int,
                vgd_int,
                self.params.jfet2_acgam,
                temp_terms.t_gate_pot,
                temp_terms.alpha,
                self.params.vto,
                temp_terms.cor_dep_cap,
                self.params.jfet2_xc,
                temp_terms.t_cgs * scale,
                temp_terms.t_cgd * scale,
            )
        };

        let vgs_int = pol * vgs;
        let vgd_int = pol * vgd;
        let (qgga, cgsna, cgdna) = qgg(vgs_int, vgd_int);
        let (qgs_int, qgd_int, cgs, cgd) =
            if let Some((vgs_prev, vgd_prev, qgs_prev, qgd_prev)) = previous {
                let vgs_prev_int = pol * vgs_prev;
                let vgd_prev_int = pol * vgd_prev;
                let qgs_prev_int = pol * qgs_prev;
                let qgd_prev_int = pol * qgd_prev;
                let (qggb, _, cgdnb) = qgg(vgs_prev_int, vgd_int);
                let (qggc, cgsnc, _) = qgg(vgs_int, vgd_prev_int);
                let (qggd, _, _) = qgg(vgs_prev_int, vgd_prev_int);
                (
                    qgs_prev_int + 0.5 * (qgga - qggb + qggc - qggd),
                    qgd_prev_int + 0.5 * (qgga - qggc + qggb - qggd),
                    0.5 * (cgsna + cgsnc),
                    0.5 * (cgdna + cgdnb),
                )
            } else {
                (qgga, qgga, cgsna, cgdna)
            };

        Jfet2ChargeState {
            qgs: finite_or_zero(pol * qgs_int),
            qgd: finite_or_zero(pol * qgd_int),
            cgs,
            cgd,
        }
    }

    pub(super) fn jfet2_drain_source_capacitance(&self) -> Value {
        finite_or_zero(self.params.jfet2_capds.max(0.0) * self.area.max(0.0) * self.m.max(0.0))
    }

    pub(super) fn jfet2_ac_feedback_terms(
        &self,
        vgs: Value,
        vds: Value,
        ids: Value,
        frequency_hz: Value,
        gm: Value,
        gds: Value,
    ) -> (Value, Value, Value, Value) {
        let omega = 2.0 * std::f64::consts::PI * frequency_hz.max(0.0);
        if omega == 0.0 {
            return (gm, 0.0, gds, 0.0);
        }

        let pol = self.jfet_type.polarity();
        let vgs_int = pol * vgs;
        let vds_int = pol * vds;
        let vgd_int = vgs_int - vds_int;
        let lf_g2_vgd = self.params.jfet2_lfg2 * vgd_int;
        let hf_g2_vgd = self.params.jfet2_hfg2 * vgd_int;
        let hfgam = self.jfet2_hfgam() - self.params.jfet2_hfg1 * vgs_int + hf_g2_vgd;
        let eta = self.params.jfet2_hfeta - self.params.jfet2_hfe1 * vgd_int
            + self.params.jfet2_hfe2 * vgs_int;
        let lfga =
            self.params.jfet2_lfgam - self.params.jfet2_lfg1 * vgs_int + lf_g2_vgd + lf_g2_vgd;
        let denom = 1.0 - lfga + self.params.jfet2_lfg1 * vgd_int;
        if denom.abs() <= 1.0e-30 {
            return (gm, 0.0, gds, 0.0);
        }
        let gmo = gm / denom;
        let wtg = self.params.jfet2_taug * omega;
        let wtgdet = 1.0 + wtg * wtg;
        let gwtgdet = gmo / wtgdet;
        let gdsi = (hfgam - lfga) * gwtgdet;
        let gdsr = (hfgam - lfga) * gmo - gdsi;
        let gmi = (eta + self.params.jfet2_lfg1 * vgd_int) * gwtgdet + gdsi;
        let xgds = wtg * gdsi;
        let gds_real = gds + gdsr;
        let xgm = -wtg * gmi;
        let gm_real = gmi + gmo * (1.0 - eta - hfgam);

        let area = self.area.max(1.0e-30);
        let delta = self.params.jfet2_delta / area;
        let wtd = self.params.jfet2_taud * omega;
        let wtddet = 1.0 + wtd * wtd;
        let ids_int = pol * ids;
        let fac = delta * ids_int;
        let thermal_denom = 1.0 - fac * vds_int;
        if !thermal_denom.is_finite() || thermal_denom.abs() <= 1.0e-30 {
            return (gm, 0.0, gds, 0.0);
        }
        let del = 1.0 / thermal_denom;
        let dd = (del - 1.0) / wtddet;
        let dr = del - dd;
        let di = wtd * dd;
        let cdsqr = fac * ids_int * del * wtd / wtddet;

        let gm_ac = dr * gm_real - di * xgm;
        let xgm_ac = di * gm_real + dr * xgm;
        let gds_ac = dr * gds_real - di * xgds + cdsqr * wtd;
        let xgds_ac = di * gds_real + dr * xgds + cdsqr;
        (
            finite_or_zero(gm_ac),
            finite_or_zero(xgm_ac),
            finite_or_zero(gds_ac),
            finite_or_zero(xgds_ac),
        )
    }
}

#[inline]
fn finite_or_zero(value: Value) -> Value {
    if value.is_finite() { value } else { 0.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ngspice46_common_source_jfet2() -> Jfet {
        let mut jfet = Jfet::njf("j1", 1, 2, 0).enable_jfet2_model();
        jfet.params.beta = 1.0e-3;
        jfet.params.vto = -2.0;
        jfet.params.lambda = 0.02;
        jfet.params.pb = 1.0;
        jfet.params.is = 1.0e-14;
        jfet.params.n = 1.0;
        jfet.params.jfet2_p = 2.0;
        jfet.params.jfet2_q = 2.0;
        jfet.params.jfet2_xi = 1000.0;
        jfet.params.jfet2_z = 1.0;
        jfet.params.jfet2_vst = 0.1;
        jfet.params.jfet2_mvst = 0.05;
        jfet.params.jfet2_mxi = 0.0;
        jfet.params.jfet2_lfgam = 0.01;
        jfet.params.jfet2_lfg1 = 0.002;
        jfet.params.jfet2_lfg2 = 0.001;
        jfet.params.jfet2_ibd = 1.0e-12;
        jfet.params.jfet2_vbd = 10.0;
        jfet.params.jfet2_acgam = 0.05;
        jfet.params.jfet2_hfgam = 0.02;
        jfet.params.jfet2_hfg1 = 0.001;
        jfet.params.jfet2_hfg2 = 0.0005;
        jfet.params.jfet2_hfeta = 0.01;
        jfet.params.jfet2_hfe1 = 0.001;
        jfet.params.jfet2_hfe2 = 0.0007;
        jfet.params.jfet2_taug = 1.0e-9;
        jfet.params.jfet2_taud = 2.0e-9;
        jfet.params.jfet2_delta = 0.01;
        jfet
    }

    #[test]
    fn parker_skellern_operating_terms_match_ngspice46_common_source_oracle() {
        let jfet = ngspice46_common_source_jfet2();
        let vds = 0.788_235_214_303_974_9;
        let terms = jfet.jfet2_operating_terms(-0.25, vds, -0.25 - vds, 300.15);

        assert!(
            (terms.ids - 2.105_882_339_113_719e-3).abs() < 2.0e-12,
            "ids={:.16e}",
            terms.ids
        );
        assert!(
            (terms.gm - 1.597_147_998_591_449e-3).abs() < 1.0e-12,
            "gm={:.16e}",
            terms.gm
        );
        assert!(
            (terms.gds - 1.794_436_366_968_775e-3).abs() < 1.0e-12,
            "gds={:.16e}",
            terms.gds
        );
    }

    #[test]
    fn xyce_jfet2_charge_state_matches_closed_form_depletion_charge() {
        let mut jfet = Jfet::njf("jtest", 1, 2, 3).enable_xyce_jfet2_model();
        jfet.params.pb = 1.0;
        jfet.params.fc = 0.5;
        jfet.params.cgs = 1.0e-6;
        jfet.params.cgd = 2.0e-6;
        jfet.params.m = 0.2;

        let charge = jfet.xyce_jfet2_charge_state(0.75, -0.25, 300.15);
        assert!((charge.qgs - 9.835_340_020_443_378e-7).abs() < 1.0e-18);
        assert!((charge.cgs - 1.767_766_952_966_368_8e-6).abs() < 1.0e-18);
        assert!((charge.qgd - -4.721_359_549_995_796e-7).abs() < 1.0e-18);
        assert!((charge.cgd - 1.788_854_381_999_831_5e-6).abs() < 1.0e-18);
    }
}
