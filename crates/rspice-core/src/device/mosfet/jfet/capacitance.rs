//! JFET charge, capacitance, noise, and small-signal AC helpers.

use super::*;

impl Jfet {
    pub(super) fn mesa_level3_capacitances_mode(
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
        let t = vgt / vt - 1.0;
        let q_term = (p.hfet_delta.max(1e-9).powi(2) + t * t).sqrt();
        let vgte = 0.5 * vt * (2.0 + t + q_term);
        let a = 2.0 * p.beta.max(1e-30) * vgte;

        let du = p.mesa_du.max(1e-12);
        let th = p.mesa_th.max(1e-12);
        let ndelta = p.mesa_ndelta.max(1e-30);
        let ndu = p.mesa_ndu.max(1e-30);
        let vpou = Q_ELECTRON * ndu * du * du / (2.0 * EPSILONGAAS);
        let vpod = Q_ELECTRON * ndelta * th * (2.0 * du + th) / (2.0 * EPSILONGAAS);
        let vpo = vpou + vpod;

        let (nsa, ca) = if vgt > vpod {
            if vgte > vpo {
                (ndelta * th + ndu * du, EPSILONGAAS / du)
            } else {
                let r = ((vpo - vgte) / vpou.max(1e-30)).max(0.0).sqrt().max(1e-30);
                (ndelta * th + ndu * du * (1.0 - r), EPSILONGAAS / (du * r))
            }
        } else if vpod - vgte < 0.0 {
            (ndelta * th * (1.0 - du / th), EPSILONGAAS / du)
        } else {
            let r = (1.0 + ndu / ndelta * (vpod - vgte) / vpou.max(1e-30))
                .max(0.0)
                .sqrt()
                .max(1e-30);
            (
                ndelta * th * (1.0 - du / th * (r - 1.0)),
                EPSILONGAAS / (du * r),
            )
        };

        let b = Self::exp_limited(vgt / etavth);
        let cb = EPSILONGAAS / (du + th).max(1e-30) * b;
        let nsb0 = (EPSILONGAAS * eta * vt / (Q_ELECTRON * (du + th).max(1e-30))).max(1e-30);
        let nsb = nsb0 * b;
        let ns = if (nsa + nsb).abs() > 1e-30 {
            nsa * nsb / (nsa + nsb)
        } else {
            0.0
        };

        let cf = 0.5 * EPSILONGAAS * w;
        if !ns.is_finite()
            || ns < 1e-38
            || !ca.is_finite()
            || !cb.is_finite()
            || ca <= 0.0
            || cb <= 0.0
        {
            return (cf.max(1e-18), cf.max(1e-18));
        }

        let rt = p.hfet_rsi.max(0.0) + p.hfet_rdi.max(0.0);
        let gchi0 = Q_ELECTRON * w * p.hfet_mu.max(1e-12) / l;
        let gchi = gchi0 * ns;
        let gch = gchi / (1.0 + gchi * rt);
        if !gch.is_finite() || gch <= 0.0 {
            return (cf.max(1e-18), cf.max(1e-18));
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
        let vdse = vds_int
            * (1.0 + (vds_int / vsate).max(0.0).powf(p.hfet_mc.max(1e-9)))
                .powf(-1.0 / p.hfet_mc.max(1e-9));
        let cgc = w * l * ca * cb / (ca + cb).max(1e-30);

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

    pub(super) fn mesa_level4_capacitances_mode(
        &self,
        vgs: Value,
        vgd: Value,
        temp: Value,
        force_inverse: bool,
    ) -> (Value, Value) {
        const Q_ELECTRON: Value = 1.602176634e-19;

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
        let u = 0.5 * vgt / vt - 1.0;
        let t = (p.hfet_delta.max(1e-9).powi(2) + u * u).sqrt();
        let vgte = vt * (2.0 + u + t);
        let b = Self::exp_limited(vgt / etavth);

        let epsi = p.hfet_epsi.max(1e-30);
        let d0 = p.hfet_di.max(1e-12);
        let n0 = (epsi * eta * vt / (2.0 * Q_ELECTRON * d0)).max(1e-30);
        let nsm = 2.0 * n0 * (1.0 + 0.5 * b).ln();
        let cf = 0.5 * epsi * w;
        if !nsm.is_finite() || nsm < 1e-38 {
            return (cf.max(1e-18), cf.max(1e-18));
        }

        let gamma = p.hfet_gamma.max(1e-9);
        let comp = (nsm / p.hfet_nmax.max(1e-30)).max(0.0).powf(gamma);
        let ns = nsm / (1.0 + comp).powf(1.0 / gamma);
        let mu = p.hfet_mu.max(1e-12);
        let gchi0 = Q_ELECTRON * w * mu / l;
        let gchi = gchi0 * ns;
        let rt = p.hfet_rsi.max(0.0) + p.hfet_rdi.max(0.0);
        let gch = gchi / (1.0 + gchi * rt);
        if !gch.is_finite() || gch <= 0.0 {
            return (cf.max(1e-18), cf.max(1e-18));
        }

        let gchim = gchi0 * nsm;
        let vl = (p.hfet_vs.max(1e-12) / mu * l).max(1e-30);
        let h = (1.0 + 2.0 * gchim * p.hfet_rsi.max(0.0) + vgte * vgte / (vl * vl)).sqrt();
        let p_denom = 1.0 + gchim * p.hfet_rsi.max(0.0) + h;
        let isatm = gchim * vgte / p_denom.max(1e-30);
        let imax = (Q_ELECTRON * p.hfet_nmax.max(1e-12) * p.hfet_vs.max(1e-12) * w).max(1e-30);
        let g = (isatm / imax).max(0.0).powf(gamma);
        let isat = isatm / (1.0 + g).powf(1.0 / gamma);
        let vsate = (isat / gch).abs().max(1e-30);
        let vdse = vds_int
            * (1.0 + (vds_int / vsate).max(0.0).powf(p.hfet_mc.max(1e-9)))
                .powf(-1.0 / p.hfet_mc.max(1e-9));

        let cas = p.mesa_cas.max(1e-12);
        let cbs = p.mesa_cbs.max(1e-12);
        let cgcm_denom = d0 / (cas * epsi).max(1e-30)
            + etavth / (cbs * Q_ELECTRON * n0).max(1e-30) * Self::exp_limited(-vgt / etavth);
        let cgcm = 1.0 / cgcm_denom.max(1e-30);
        let cgc = w * l * cgcm / (1.0 + comp).powf(1.0 + 1.0 / gamma);

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

    /// Calculate gate junction current (reverse-biased diodes)
    ///
    /// Returns (Igs, Igd) - gate-source and gate-drain junction currents
    pub fn gate_current(&self, vgs: Value, vgd: Value, temp: Value) -> (Value, Value) {
        let (igs, igd, _, _) = self.gate_junctions(vgs, vgd, temp);
        (igs, igd)
    }

    /// Return the flicker-noise coefficients `(KF, AF, EF)`.
    ///
    /// jfetnoi.c applies KF directly — `m·KF·|cd|^AF / f` — with no
    /// geometry normalization; the caller folds the multiplicity.
    pub fn flicker_noise_coefficients(&self) -> Option<(Value, Value, Value)> {
        if self.params.kf <= 0.0 || !self.params.kf.is_finite() {
            return None;
        }

        Some((
            self.params.kf,
            self.params.af.max(1e-12),
            self.params.ef.max(1e-12),
        ))
    }

    /// Calculate gate junction currents and conductances.
    ///
    /// Returned currents are defined in external terminal orientation:
    /// - `igs`: current from gate to source
    /// - `igd`: current from gate to drain
    pub(super) fn gate_junctions(
        &self,
        vgs: Value,
        vgd: Value,
        temp: Value,
    ) -> (Value, Value, Value, Value) {
        let (temp_common, temp_source, temp_drain) = self.resolved_temperatures(temp);
        let pol = self.jfet_type.polarity();
        let vgs_int = pol * vgs;
        let vgd_int = pol * vgd;

        if matches!(self.params.channel_model, JfetChannelModel::Hfet1) {
            // GATEMOD=1 gate currents are produced by the channel
            // evaluation (calculate_hfet1_core) and never reach this
            // diode/leakage path; see compute_operating_terms.
            let (igs_int, ggs, igd_int, ggd) =
                if self.params.hfet_level >= 2 && self.params.hfet_level < 5 {
                    let (igs_int, ggs) = self.mesa_gate_branch(vgs_int, temp_source);
                    let (igd_int, ggd) = self.mesa_gate_branch(vgd_int, temp_drain);
                    (igs_int, ggs, igd_int, ggd)
                } else {
                    let (igs_int, ggs) = self.hfet_gate_branch(
                        vgs_int,
                        temp_source,
                        self.params.hfet_js1s,
                        self.params.hfet_js2s,
                        self.params.hfet_m1s,
                        self.params.hfet_m2s,
                        self.params.hfet_rgs,
                    );
                    let (igd_int, ggd) = self.hfet_gate_branch(
                        vgd_int,
                        temp_drain,
                        self.params.hfet_js1d,
                        self.params.hfet_js2d,
                        self.params.hfet_m1d,
                        self.params.hfet_m2d,
                        self.params.hfet_rgd,
                    );
                    (igs_int, ggs, igd_int, ggd)
                };
            return (pol * igs_int, pol * igd_int, ggs, ggd);
        }

        let igs = pol * self.junction_diode_current(vgs_int, temp_common);
        let igd = pol * self.junction_diode_current(vgd_int, temp_common);
        let ggs = self.junction_diode_conductance(vgs_int, temp_common);
        let ggd = self.junction_diode_conductance(vgd_int, temp_common);
        (igs, igd, ggs, ggd)
    }

    pub(super) fn hfet1_capacitances(&self, vgs: Value, vgd: Value, temp: Value) -> (Value, Value) {
        const Q_ELECTRON: Value = 1.602176634e-19;

        let pol = self.jfet_type.polarity();
        let p = &self.params;
        let w = self.width.max(1e-12);
        let l = self.length.max(1e-12);
        let epsi = p.hfet_epsi.max(1e-30);
        let cf = 0.5 * epsi * w;

        let vgs_int = pol * vgs;
        let vgd_int = pol * vgd;
        let vgs_eff = vgs_int;
        let mut vds_int = vgs_int - vgd_int;
        let mut inverse = false;
        if vds_int < 0.0 {
            // Match ngspice HFET1 load path: evaluate with |Vds| while keeping
            // channel control on Vgs, then swap terminal caps in inverse mode.
            vds_int = -vds_int;
            inverse = true;
        }

        let temp_k = if temp.is_finite() && temp > 0.0 {
            temp
        } else {
            p.tnom.max(1.0)
        };
        let vt = self.thermal_voltage(temp_k).max(1e-12);
        let eta = p.eta.abs().max(1e-9);
        let etavth = (eta * vt).max(1e-12);

        let mu = p.hfet_mu.max(1e-12);
        let vs = p.hfet_vs.max(1e-12);
        let di = p.hfet_di.max(1e-12);
        let deltad = p.hfet_deltad.max(0.0);
        let nmax = p.hfet_nmax.max(1e-12);
        let gamma = p.hfet_gamma.max(1e-9);
        let sigma0 = p.sigma0.max(0.0);
        let vsigma = p.hfet_vsigma.max(1e-12);
        let vsigmat = p.hfet_vsigmat;
        let rsi = p.hfet_rsi.max(0.0);
        let rdi = p.hfet_rdi.max(0.0);
        let rt = rsi + rdi;

        let vto = pol * p.vto;
        let n0 = epsi * eta * vt / (2.0 * Q_ELECTRON * (di + deltad).max(1e-30));
        let gchi0 = Q_ELECTRON * w * mu / l;
        let imax = (Q_ELECTRON * nmax * vs * w).max(1e-30);
        let vl = (vs / mu * l).max(1e-30);

        let vgt0 = vgs_eff - vto;
        let s = ((vgt0 - vsigmat) / vsigma).clamp(-80.0, 80.0).exp();
        let sigma = sigma0 / (1.0 + s);
        let vgt = vgt0 + sigma * vds_int;
        let u = 0.5 * vgt / vt - 1.0;
        let t = (p.hfet_delta.max(1e-9).powi(2) + u * u).sqrt();
        let vgte = vt * (2.0 + u + t);
        let b = (vgt / etavth).clamp(-80.0, 80.0).exp();
        let nsm = 2.0 * n0 * (1.0 + 0.5 * b).ln();
        if !nsm.is_finite() || nsm < 1.0e-38 {
            return (cf.max(1e-18), cf.max(1e-18));
        }

        let c = (nsm / nmax).max(0.0).powf(gamma);
        let q = (1.0 + c).powf(1.0 / gamma);
        let ns = nsm / q;
        let gchi = gchi0 * ns;
        let gch = gchi / (1.0 + gchi * rt);
        if !gch.is_finite() || gch <= 0.0 {
            return (cf.max(1e-18), cf.max(1e-18));
        }

        let gchim = gchi0 * nsm;
        let h = (1.0 + 2.0 * gchim * rsi + vgte * vgte / (vl * vl)).sqrt();
        let p_denom = 1.0 + gchim * rsi + h;
        if !p_denom.is_finite() || p_denom <= 0.0 {
            return (cf.max(1e-18), cf.max(1e-18));
        }

        let isatm = gchim * vgte / p_denom;
        let g = (isatm / imax).max(0.0).powf(gamma);
        let isat = isatm / (1.0 + g).powf(1.0 / gamma);
        if !isat.is_finite() {
            return (cf.max(1e-18), cf.max(1e-18));
        }
        let vsate = (isat / gch).abs().max(1e-30);

        let delnsnsm = ns / nsm * (1.0 - c / (1.0 + c));
        let delnsmvgt = n0 / etavth / (1.0 / b + 0.5);
        let delvgtvgs = 1.0 - vds_int * sigma0 / vsigma * s / ((1.0 + s) * (1.0 + s));

        let eta1 = p.hfet_eta1.max(1e-9);
        let d1 = p.hfet_d1.max(1e-12);
        let temp_eta1 = (eta1 * vt).max(1e-18);
        let vt1 = if p.hfet_vt1.is_finite() {
            p.hfet_vt1
        } else {
            vto + Q_ELECTRON * nmax * di / epsi
        };
        let cg1 = 1.0
            / (d1 / epsi + temp_eta1 * Self::exp_limited(-(vgs_eff - vt1) / temp_eta1)).max(1e-30);
        let mut cgc = w * l * (Q_ELECTRON * delnsnsm * delnsmvgt * delvgtvgs + cg1);
        if !cgc.is_finite() || cgc < 0.0 {
            cgc = 0.0;
        }

        let mc = p.hfet_mc.max(1e-9);
        let vdse = vds_int * (1.0 + (vds_int / vsate).max(0.0).powf(mc)).powf(-1.0 / mc);
        let c1_denom = (2.0 * vsate - vdse).max(1e-30);
        let a_gs = ((vsate - vdse) / c1_denom).powi(2);
        let pcap = p.hfet_p + (1.0 - p.hfet_p) * Self::exp_limited(-vds_int / vsate);
        let mut capgs = cf + (4.0 / 3.0) * cgc * (1.0 - a_gs) / (1.0 + pcap);

        let a_gd = (vsate / c1_denom).powi(2);
        let mut capgd = cf + (4.0 / 3.0) * pcap * cgc * (1.0 - a_gd) / (1.0 + pcap);

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

    /// Transient capacitances for the active JFET/MESFET model.
    ///
    /// Inputs are external branch voltages (`vgs = Vg - Vs`, `vgd = Vg - Vd`).
    pub fn transient_capacitances(&self, vgs: Value, vgd: Value, temp: Value) -> (Value, Value) {
        let (temp_common, temp_source, _) = self.resolved_temperatures(temp);
        let (mut cgs, mut cgd) = match self.params.channel_model {
            JfetChannelModel::ShichmanHodges | JfetChannelModel::LegacyMesfet => {
                let pol = self.jfet_type.polarity();
                self.capacitances(pol * vgs, pol * vgd)
            }
            JfetChannelModel::Hfet1 => match self.params.hfet_level {
                2..=4 => {
                    let pol = self.jfet_type.polarity();
                    let vds_int = pol * (vgs - vgd);
                    let local_inverse = vds_int < 0.0;
                    let force_inverse = self.hfet_legacy_inverse_active && !local_inverse;
                    match self.params.hfet_level {
                        2 => {
                            self.mesa_level2_capacitances_mode(vgs, vgd, temp_source, force_inverse)
                        }
                        3 => {
                            self.mesa_level3_capacitances_mode(vgs, vgd, temp_source, force_inverse)
                        }
                        _ => {
                            self.mesa_level4_capacitances_mode(vgs, vgd, temp_source, force_inverse)
                        }
                    }
                }
                _ => self.hfet1_capacitances(vgs, vgd, temp_common),
            },
        };
        if self.hfet_legacy_inverse_active
            && matches!(self.params.channel_model, JfetChannelModel::Hfet1)
            && self.params.hfet_level >= 5
        {
            // ngspice HFET keeps a legacy inverse latch across instances and applies
            // one effective cap swap when that latch is active. Local inverse mode
            // (vds_int < 0) is already handled inside hfet*_capacitances(), so only
            // swap here for forward-oriented instances to avoid a double swap.
            let pol = self.jfet_type.polarity();
            let vds_int = pol * (vgs - vgd);
            let local_inverse = vds_int < 0.0;
            if !local_inverse {
                std::mem::swap(&mut cgs, &mut cgd);
            }
        }
        (cgs, cgd)
    }

    pub(crate) fn ac_capacitances(
        &self,
        vgs: Value,
        vgd: Value,
        temp: Value,
    ) -> (Value, Value, Value) {
        let cds = if matches!(self.params.channel_model, JfetChannelModel::Hfet1)
            && self.params.hfet_level >= 5
        {
            self.params.hfet_cds.max(0.0)
        } else {
            0.0
        };

        match self.params.channel_model {
            JfetChannelModel::ShichmanHodges | JfetChannelModel::LegacyMesfet => {
                let pol = self.jfet_type.polarity();
                let (cgs, cgd) = self.capacitances(pol * vgs, pol * vgd);
                (cgs, cgd, cds)
            }
            JfetChannelModel::Hfet1 => {
                let (cgs, cgd) = self.transient_capacitances(vgs, vgd, temp);
                (cgs, cgd, cds)
            }
        }
    }

    pub(crate) fn transient_drain_source_capacitance(&self) -> Value {
        if matches!(self.params.channel_model, JfetChannelModel::Hfet1)
            && self.params.hfet_level >= 5
        {
            self.params.hfet_cds.max(0.0)
        } else {
            0.0
        }
    }

    pub(super) fn ac_real_terms_at_frequency(
        &self,
        vgs: Value,
        vds: Value,
        vgd: Value,
        frequency_hz: Value,
    ) -> (Value, Value, Value, Value) {
        let (temp_common, temp_source, _) = self.resolved_temperatures(self.params.tnom);
        // GATEMOD=1's gmg/gmd are deliberately not part of the AC stamp:
        // ngspice's hfetacl.c omits them too (only ggd/ggs/gm/gds appear),
        // so dropping them here is reference-exact.
        let (_, gm_base, gds_base, _, _, ggs, ggd, _, _gmg, _gmd) =
            self.compute_operating_terms(vgs, vds, vgd);

        let (gm, gds) = match self.params.channel_model {
            JfetChannelModel::ShichmanHodges | JfetChannelModel::LegacyMesfet => {
                (gm_base, gds_base)
            }
            JfetChannelModel::Hfet1 => match self.params.hfet_level {
                2..=4 => {
                    let force_inverse = self.hfet_legacy_inverse_active && vds >= 0.0;
                    if self.params.hfet_level == 2 && !force_inverse {
                        let pol = self.jfet_type.polarity();
                        let vto = pol * self.params.vto;
                        let vgs_int = pol * vgs;
                        let vds_int = pol * vds;
                        if vds_int >= 0.0 {
                            let lambda = self.mesa_ac_lambda(temp_source, Some(frequency_hz));
                            let (gm, gds) = self.mesa_level2_ac_conductances_forward(
                                vgs_int,
                                vds_int,
                                temp_source,
                                vto,
                                lambda,
                            );
                            (gm, gds)
                        } else {
                            let (_, gm, gds) = self.calculate_mesa_level_ac(
                                vgs,
                                vds,
                                temp_source,
                                self.params.hfet_level,
                                force_inverse,
                                frequency_hz,
                            );
                            (gm, gds)
                        }
                    } else {
                        let (_, gm, gds) = self.calculate_mesa_level_ac(
                            vgs,
                            vds,
                            temp_source,
                            self.params.hfet_level,
                            force_inverse,
                            frequency_hz,
                        );
                        (gm, gds)
                    }
                }
                5.. => (
                    gm_base,
                    gds_base * self.hfet_ac_gds_scale(temp_common, frequency_hz),
                ),
                _ => (gm_base, gds_base),
            },
        };

        (gm, gds, ggs, ggd)
    }

    pub(crate) fn stamp_small_signal_ac(
        &self,
        voltages: &[Value],
        frequency_hz: Value,
        matrix: &mut impl MatrixStamper,
    ) {
        let (vgs, vds, vgd) = self.state_or_raw_branch_voltages(voltages);
        let (gm, gds, ggs, ggd) = self.ac_real_terms_at_frequency(vgs, vds, vgd, frequency_hz);

        matrix.stamp(self.drain, self.drain, gds + ggd);
        matrix.stamp(self.drain, self.gate, gm - ggd);
        matrix.stamp(self.drain, self.source, -gm - gds);

        matrix.stamp(self.gate, self.drain, -ggd);
        matrix.stamp(self.gate, self.gate, ggs + ggd);
        matrix.stamp(self.gate, self.source, -ggs);

        matrix.stamp(self.source, self.drain, -gds);
        matrix.stamp(self.source, self.gate, -gm - ggs);
        matrix.stamp(self.source, self.source, gm + gds + ggs);
    }

    /// Calculate junction capacitances
    ///
    /// Returns (Cgs, Cgd) - gate-source and gate-drain capacitances
    pub fn capacitances(&self, vgs: Value, vgd: Value) -> (Value, Value) {
        let scale = self.junction_scale();
        let cgs0 = self.params.cgs * scale;
        let cgd0 = self.params.cgd * scale;
        let pb = self.params.pb;
        let m = self.params.m;
        let fc = self.params.fc;

        // Depletion capacitance model
        let cgs = if vgs <= fc * pb {
            cgs0 / (1.0 - vgs / pb).powf(m)
        } else {
            // Forward bias region - use linear extrapolation
            let f1 = (1.0 - fc).powf(1.0 + m);
            let f2 = 1.0 + m * fc;
            cgs0 / f1 * (f2 + m * vgs / pb)
        };

        let cgd = if vgd <= fc * pb {
            cgd0 / (1.0 - vgd / pb).powf(m)
        } else {
            let f1 = (1.0 - fc).powf(1.0 + m);
            let f2 = 1.0 + m * fc;
            cgd0 / f1 * (f2 + m * vgd / pb)
        };

        (cgs.max(cgs0 * 0.01), cgd.max(cgd0 * 0.01))
    }

    /// Get IDSS (drain current at Vgs=0 in saturation)
    pub fn idss(&self) -> Value {
        self.params.beta * self.params.vto * self.params.vto * self.area * self.m
    }
}
