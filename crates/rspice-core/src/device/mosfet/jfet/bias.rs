//! Bias limiting, gate leakage, temperature, and operating-point helpers.

use super::*;

impl Jfet {
    /// Thermal voltage at given temperature
    pub(super) fn thermal_voltage(&self, temp: Value) -> Value {
        const K_BOLTZMANN: Value = 1.380649e-23;
        const Q_ELECTRON: Value = 1.602176634e-19;
        K_BOLTZMANN * temp / Q_ELECTRON
    }

    #[inline]
    pub(super) fn junction_scale(&self) -> Value {
        self.area * self.m
    }

    #[inline]
    pub(super) fn resolved_temperatures(&self, ambient: Value) -> (Value, Value, Value) {
        let mut base = if ambient.is_finite() && ambient > 0.0 {
            ambient
        } else {
            self.params.tnom.max(1.0)
        };

        if let Some(temp) = self.instance_temp.filter(|v| v.is_finite() && *v > 0.0) {
            base = temp;
        } else {
            base += self.instance_dtemp;
        }
        if !base.is_finite() || base <= 0.0 {
            base = self.params.tnom.max(1.0);
        }

        let ts = self
            .instance_ts
            .filter(|v| v.is_finite() && *v > 0.0)
            .unwrap_or(base);
        let td = self
            .instance_td
            .filter(|v| v.is_finite() && *v > 0.0)
            .unwrap_or(base);

        (base.max(1.0), ts.max(1.0), td.max(1.0))
    }

    /// ngspice-compatible gate-junction branch evaluation for Level-1 JFETs.
    ///
    /// Mirrors `jfetload.c`:
    /// - reverse branch asymptote for `v < -3*n*Vt`
    /// - explicit `gmin * v` current contribution
    /// - explicit `+ gmin` small-signal conductance floor
    #[inline]
    pub(super) fn junction_diode_terms(&self, v_ak: Value, temp: Value) -> (Value, Value) {
        const JFET_GMIN: Value = 1e-12;
        let temp_k = if temp.is_finite() && temp > 0.0 {
            temp
        } else {
            self.params.tnom.max(1.0)
        };
        let nvt = (self.params.n.max(1e-12) * self.thermal_voltage(temp_k)).max(1e-12);
        let isat = (self.params.is * self.junction_scale()).max(0.0);

        if v_ak < -3.0 * nvt {
            // ngspice `jfetload.c` reverse-bias asymptote:
            // arg = (3*vt/(v*e))^3
            let mut arg = 3.0 * nvt / (v_ak * std::f64::consts::E);
            arg = arg * arg * arg;
            let i = -isat * (1.0 + arg) + JFET_GMIN * v_ak;
            let g = isat * 3.0 * arg / v_ak + JFET_GMIN;
            if i.is_finite() && g.is_finite() {
                (i, g.max(JFET_GMIN))
            } else {
                (JFET_GMIN * v_ak, JFET_GMIN)
            }
        } else {
            // Clamp exponent for robustness outside pnjlim/fetlim regimes.
            let exp_term = (v_ak / nvt).clamp(-80.0, 80.0).exp();
            let i = isat * (exp_term - 1.0) + JFET_GMIN * v_ak;
            let g = isat * exp_term / nvt + JFET_GMIN;
            if i.is_finite() && g.is_finite() {
                (i, g.max(JFET_GMIN))
            } else {
                (JFET_GMIN * v_ak, JFET_GMIN)
            }
        }
    }

    /// Gate junction diode current for internal anode-cathode voltage.
    pub(super) fn junction_diode_current(&self, v_ak: Value, temp: Value) -> Value {
        self.junction_diode_terms(v_ak, temp).0
    }

    /// Gate junction diode small-signal conductance for internal anode-cathode voltage.
    pub(super) fn junction_diode_conductance(&self, v_ak: Value, temp: Value) -> Value {
        self.junction_diode_terms(v_ak, temp).1
    }

    #[inline]
    pub(super) fn hfet_gate_geometry_scale(&self) -> Value {
        let w = self.width.max(1e-12);
        let l = self.length.max(1e-12);
        (0.5 * w * l * self.area.max(0.0) * self.m.max(0.0)).max(0.0)
    }

    /// ngspice HFET helper (`hfetload.c:diode`) used by `leak()`.
    pub(super) fn hfet_diode_aux(u: Value) -> Value {
        const U0: Value = -2.303;
        const A: Value = 2.221;
        const B: Value = 6.804;
        const C: Value = 1.685;

        let expu = u.exp();
        let it = if u <= U0 {
            expu * (1.0 - expu)
        } else {
            let b = 0.5 * (u - U0);
            u + A * ((U0 - u) / B).exp() - (b + (b * b + 0.25 * C * C).sqrt()).ln()
        };
        let it = if it.is_finite() && it > 1e-30 {
            it
        } else {
            1e-30
        };
        let ut = it + it.ln();
        let b = u - ut;
        let c = 1.0 + it;
        it * (1.0 + b / c + 0.5 * b * b / (c * c * c))
    }

    /// ngspice HFET Schottky branch model (`hfetload.c:leak`).
    pub(super) fn hfet_leak(
        gmin: Value,
        vt: Value,
        v: Value,
        rs: Value,
        is1: Value,
        is2: Value,
        m1: Value,
        m2: Value,
    ) -> (Value, Value) {
        let vt1 = (vt * m1).max(1e-18);
        let vt2 = (vt * m2).max(1e-18);
        let rs = rs.max(0.0);
        let is1 = is1.max(0.0);
        let is2 = is2.max(0.0);
        let gmin = gmin.max(1e-30);

        if v > -10.0 * vt1 {
            let vteff = (vt1 + vt2).max(1e-18);
            let msum = (m1 + m2).max(1e-18);
            let ratio = if is2 > 0.0 { is1 / is2 } else { 0.0 };
            let iseff = if is1 > 0.0 && is2 > 0.0 && ratio.is_finite() && ratio > 0.0 {
                is2 * ratio.powf(m1 / msum)
            } else {
                0.0
            };

            let (iaprox1, iaprox2) = if rs > 0.0 {
                let rsis1 = (rs * is1).max(1e-30);
                let rsiseff = (rs * iseff).max(1e-30);
                let u1 = (v + rs * is1) / vt1 + (rsis1 / vt1).ln();
                let u2 = (v + rs * iseff) / vteff + (rsiseff / vteff).ln();
                let i1 = vt1 * Self::hfet_diode_aux(u1) / rs - is1;
                let i2 = vteff * Self::hfet_diode_aux(u2) / rs - iseff;
                (i1, i2)
            } else {
                (
                    is1 * ((v / vt1).exp() - 1.0),
                    iseff * ((v / vteff).exp() - 1.0),
                )
            };

            let iaprox = if (iaprox1 * iaprox2) != 0.0 {
                1.0 / (1.0 / iaprox1 + 1.0 / iaprox2)
            } else {
                0.5 * (iaprox1 + iaprox2)
            };

            let dvdi0 = rs + vt1 / (iaprox + is1).max(1e-30) + vt2 / (iaprox + is2).max(1e-30);
            let v0 =
                rs * iaprox + vt1 * (iaprox / is1 + 1.0).ln() + vt2 * (iaprox / is2 + 1.0).ln();
            let il = (iaprox + (v - v0) / dvdi0).max(-is1) * 0.99999;
            let gl = 1.0 / (rs + vt1 / (il + is1).max(1e-30) + vt2 / (il + is2).max(1e-30));
            let il = if il.is_finite() { il } else { -is1 };
            let gl = if gl.is_finite() { gl.max(0.0) } else { gmin };
            (il, gl)
        } else {
            let gl = gmin;
            let il = gl * v - is1;
            (il, gl)
        }
    }

    /// HFET1 gate branch current + conductance for internal gate-source/drain voltage.
    pub(super) fn hfet_gate_branch(
        &self,
        v_int: Value,
        temp: Value,
        js1: Value,
        js2: Value,
        m1: Value,
        m2: Value,
        rg: Value,
    ) -> (Value, Value) {
        let temp_k = if temp.is_finite() && temp > 0.0 {
            temp
        } else {
            self.params.tnom.max(1.0)
        };
        let vt = self.thermal_voltage(temp_k).max(1e-12);
        let scale = self.hfet_gate_geometry_scale();
        let is1 = js1.max(0.0) * scale;
        let is2 = js2.max(0.0) * scale;

        let (mut il, mut gl) = if is1 > 0.0 && is2 > 0.0 {
            Self::hfet_leak(
                self.junction_gmin,
                vt,
                v_int,
                rg.max(0.0),
                is1,
                is2,
                m1.max(1e-12),
                m2.max(1e-12),
            )
        } else {
            (0.0, 0.0)
        };

        // ngspice HFET generation-recombination branch: GGRWL * v * exp(-v*DEL/vt)
        let ggrwl =
            self.params.hfet_ggr.max(0.0) * self.gate_generation_scale.clamp(0.0, 1.0) * scale;
        if ggrwl > 0.0 {
            let arg = -v_int * self.params.hfet_del / vt;
            let arg_eff = arg.clamp(-80.0, 80.0);
            let earg = arg_eff.exp();
            il += ggrwl * v_int * earg;
            gl += ggrwl * earg * (1.0 - arg_eff);
        }

        if !il.is_finite() {
            il = 0.0;
        }
        if !gl.is_finite() {
            gl = 0.0;
        }
        (il, gl)
    }

    /// MESA gate branch approximation (`mesaload.c`): ASTAR Schottky + GGR + GMIN.
    pub(super) fn mesa_gate_branch(&self, v_int: Value, temp: Value) -> (Value, Value) {
        const K_BOLTZMANN: Value = 1.380649e-23;

        let temp_k = if temp.is_finite() && temp > 0.0 {
            temp
        } else {
            self.params.tnom.max(1.0)
        };
        let vt = self.thermal_voltage(temp_k).max(1e-12);
        let nvt = (self.params.n.max(1e-12) * vt).max(1e-12);
        let scale = self.hfet_gate_geometry_scale();
        let astar = self.params.mesa_astar.max(0.0);
        let phib = self.params.mesa_phib.max(0.0);
        let texp = (-phib / (K_BOLTZMANN * temp_k)).clamp(-80.0, 80.0).exp();
        let csat = 0.5 * astar * temp_k * temp_k * texp * 2.0 * scale;
        let ggrwl = self.params.hfet_ggr.max(0.0)
            * self.gate_generation_scale.clamp(0.0, 1.0)
            * 2.0
            * scale
            * (self.params.mesa_xchi * (temp_k - self.params.tnom)).exp();

        let expe = (v_int / nvt).clamp(-80.0, 80.0).exp();
        let arg = -v_int * self.params.hfet_del / vt;
        let arg_eff = arg.clamp(-80.0, 80.0);
        let earg = arg_eff.exp();

        let mut g = csat * expe / nvt + ggrwl * earg * (1.0 - arg_eff) + self.junction_gmin;
        let mut i = csat * (expe - 1.0) + ggrwl * v_int * earg + self.junction_gmin * v_int;
        if !i.is_finite() {
            i = 0.0;
        }
        if !g.is_finite() {
            g = self.junction_gmin;
        }
        (i, g)
    }

    #[inline]
    pub(super) fn node_voltage(voltages: &[Value], node: NodeId) -> Value {
        if node == 0 {
            0.0
        } else {
            voltages.get(node - 1).copied().unwrap_or(0.0)
        }
    }

    pub(crate) fn uses_hfet_legacy_inverse_mode(&self) -> bool {
        self.hfet_legacy_inverse_mode
            && matches!(self.params.channel_model, JfetChannelModel::Hfet1)
            && self.params.hfet_level >= 5
    }

    #[inline]
    pub(crate) fn internal_vds_limited_state(&self) -> Value {
        self.jfet_type.polarity() * self.vds
    }

    #[inline]
    pub(crate) fn internal_branch_state_voltages(&self) -> Option<(Value, Value, Value)> {
        if self.vgs.is_finite() && self.vds.is_finite() {
            let vgs = self.vgs;
            let vds = self.vds;
            let vgd = vgs - vds;
            Some((vgs, vgd, vds))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn set_hfet_legacy_inverse_active(&mut self, active: bool) {
        let active = self.uses_hfet_legacy_inverse_mode() && active;
        if self.hfet_legacy_inverse_active != active {
            self.hfet_legacy_inverse_active = active;
            self.eval_valid = false;
            self.last_raw_vgs = Value::NAN;
            self.last_raw_vgd = Value::NAN;
            self.last_raw_vgs_prev = Value::NAN;
            self.last_raw_vgd_prev = Value::NAN;
        }
    }

    #[inline]
    pub(crate) fn set_model_order(&mut self, order: usize) {
        self.model_order = order;
    }

    #[inline]
    pub(crate) fn model_order(&self) -> usize {
        self.model_order
    }

    #[inline]
    pub(crate) fn set_junction_gmin(&mut self, gmin: Value) {
        let gmin = if gmin.is_finite() && gmin > 0.0 {
            gmin
        } else {
            0.0
        };
        if self.junction_gmin != gmin {
            self.junction_gmin = gmin;
            self.eval_valid = false;
            self.last_raw_vgs = Value::NAN;
            self.last_raw_vgd = Value::NAN;
            self.last_raw_vgs_prev = Value::NAN;
            self.last_raw_vgd_prev = Value::NAN;
        }
    }

    #[inline]
    pub(crate) fn has_gate_generation_branch(&self) -> bool {
        matches!(self.params.channel_model, JfetChannelModel::Hfet1)
            && self.params.hfet_ggr.is_finite()
            && self.params.hfet_ggr > 0.0
    }

    #[inline]
    pub(crate) fn set_gate_generation_scale(&mut self, scale: Value) {
        let scale = if scale.is_finite() {
            scale.clamp(0.0, 1.0)
        } else {
            1.0
        };
        if (self.gate_generation_scale - scale).abs() > Value::EPSILON {
            self.gate_generation_scale = scale;
            self.eval_valid = false;
            self.last_raw_vgs = Value::NAN;
            self.last_raw_vgd = Value::NAN;
            self.last_raw_vgs_prev = Value::NAN;
            self.last_raw_vgd_prev = Value::NAN;
        }
    }

    #[inline]
    pub(super) fn matches_last_raw_branch_input(&self, vgs_raw: Value, vgd_raw: Value) -> bool {
        self.eval_valid
            && !self.limiter_applied
            && self.last_raw_vgs.is_finite()
            && self.last_raw_vgd.is_finite()
            && vgs_raw == self.last_raw_vgs
            && vgd_raw == self.last_raw_vgd
    }

    /// SPICE DEVfetlim voltage limiting helper.
    ///
    /// This bounds per-iteration FET gate-voltage excursions and improves
    /// convergence robustness for stiff nonlinear bias points.
    #[inline]
    pub(super) fn fetlim(vnew: Value, vold: Value, vto: Value) -> Value {
        let vtsthi = (2.0 * (vold - vto)).abs() + 2.0;
        let vtstlo = (vold - vto).abs() + 1.0;
        let vtox = vto + 3.5;
        let delv = vnew - vold;

        if vold >= vto {
            if vold >= vtox {
                if delv <= 0.0 {
                    if vnew >= vtox {
                        if -delv > vtstlo { vold - vtstlo } else { vnew }
                    } else {
                        vnew.max(vto + 2.0)
                    }
                } else if delv >= vtsthi {
                    vold + vtsthi
                } else {
                    vnew
                }
            } else if delv <= 0.0 {
                vnew.max(vto - 0.5)
            } else {
                vnew.min(vto + 4.0)
            }
        } else if delv <= 0.0 {
            if -delv > vtsthi { vold - vtsthi } else { vnew }
        } else {
            let vtemp = vto + 0.5;
            if vnew <= vtemp {
                if delv > vtstlo { vold + vtstlo } else { vnew }
            } else {
                vtemp
            }
        }
    }

    /// SPICE DEVpnjlim helper for Schottky/PN gate-junction limiting.
    ///
    /// This limits overly aggressive forward-bias updates before `fetlim`
    /// in MESA level-2..4 paths to match ngspice `mesaload.c` behavior.
    #[inline]
    pub(super) fn pnjlim(vnew: Value, vold: Value, vt: Value, vcrit: Value) -> Value {
        if !vnew.is_finite()
            || !vold.is_finite()
            || !vt.is_finite()
            || !vcrit.is_finite()
            || vt <= 0.0
        {
            return vnew;
        }

        // ngspice DEVpnjlim (devsup.c): forward limiting branch.
        if (vnew > vcrit) && ((vnew - vold).abs() > (vt + vt)) {
            if vold > 0.0 {
                let arg = (vnew - vold) / vt;
                if arg > 0.0 {
                    return vold + vt * (2.0 + (arg - 2.0).ln());
                }
                return vold - vt * (2.0 + (2.0 - arg).ln());
            }
            return vt * (vnew / vt).ln();
        }

        // ngspice DEVpnjlim negative-voltage clamp branch.
        if vnew < 0.0 {
            let arg = if vold > 0.0 {
                -vold - 1.0
            } else {
                2.0 * vold - 1.0
            };
            if vnew < arg {
                return arg;
            }
        }
        vnew
    }

    #[inline]
    pub(super) fn classic_gate_vcrit(&self, nvt: Value) -> Value {
        let isat = (self.params.is * self.junction_scale()).max(0.0);
        if isat > 0.0 && nvt > 0.0 {
            let arg = (nvt / (core::f64::consts::SQRT_2 * isat)).max(1.0);
            nvt * arg.ln()
        } else {
            1.0
        }
    }

    #[inline]
    pub(super) fn classic_limited_branch_voltages(
        &self,
        vgs_new: Value,
        vgd_new: Value,
    ) -> (Value, Value) {
        let pol = self.jfet_type.polarity();
        if !pol.is_finite() || pol.abs() < 0.5 {
            return (vgs_new, vgd_new);
        }

        if !self.vgs.is_finite() || !self.vds.is_finite() {
            // ngspice JFET MODEINITJCT seeds active devices at -1 V on both
            // internal gate junctions before regular limiting takes over.
            let seed = -1.0 / pol;
            return (seed, seed);
        }

        let temp_k = self.params.tnom.max(1.0);
        let nvt = (self.params.n.max(1e-12) * self.thermal_voltage(temp_k)).max(1e-12);
        let vcrit = self.classic_gate_vcrit(nvt);

        let vgs_old_int = pol * self.vgs;
        let vgd_old_int = pol * (self.vgs - self.vds);
        let mut vgs_int = pol * vgs_new;
        let mut vgd_int = pol * vgd_new;

        vgs_int = Self::pnjlim(vgs_int, vgs_old_int, nvt, vcrit);
        vgd_int = Self::pnjlim(vgd_int, vgd_old_int, nvt, vcrit);
        vgs_int = Self::fetlim(vgs_int, vgs_old_int, self.params.vto);
        vgd_int = Self::fetlim(vgd_int, vgd_old_int, self.params.vto);

        (vgs_int / pol, vgd_int / pol)
    }

    #[inline]
    pub(super) fn mesa_gate_csat(&self, temp_k: Value) -> Value {
        const K_BOLTZMANN: Value = 1.380649e-23;

        let scale = self.hfet_gate_geometry_scale();
        let astar = self.params.mesa_astar.max(0.0);
        let phib = self.params.mesa_phib.max(0.0);
        let texp = (-phib / (K_BOLTZMANN * temp_k)).clamp(-80.0, 80.0).exp();
        0.5 * astar * temp_k * temp_k * texp * 2.0 * scale
    }

    #[inline]
    pub(super) fn mesa_gate_vcrit(&self, temp_k: Value, nvt: Value) -> Value {
        let csat = self.mesa_gate_csat(temp_k);
        if csat > 0.0 && nvt > 0.0 {
            let arg = (nvt / (core::f64::consts::SQRT_2 * csat)).max(1.0);
            nvt * arg.ln()
        } else {
            1.0
        }
    }

    #[inline]
    pub(super) fn temperature_shape_scale(temp_k: Value, tf_k: Value) -> Value {
        if !temp_k.is_finite() || temp_k <= 0.0 || !tf_k.is_finite() || tf_k.abs() < 1e-18 {
            return 1.0;
        }
        (temp_k / tf_k.abs()).clamp(-80.0, 80.0).exp()
    }

    #[inline]
    pub(super) fn mesa_ac_lambda(&self, temp_k: Value, frequency_hz: Option<Value>) -> Value {
        let lambda_lo = self.params.lambda;
        let lambda_hi = if self.params.mesa_lambdahf.is_finite() {
            self.params.mesa_lambdahf
        } else {
            lambda_lo
        };
        let Some(frequency_hz) = frequency_hz.filter(|f| f.is_finite() && *f >= 0.0) else {
            return lambda_lo;
        };

        let transition = self.params.mesa_delfo.abs();
        if transition <= 0.0 {
            return lambda_lo;
        }

        let scale = Self::temperature_shape_scale(temp_k, self.params.mesa_tf);
        let flo = self.params.mesa_flo.max(0.0) * scale;
        let delf = transition * scale;
        lambda_lo + 0.5 * (lambda_hi - lambda_lo) * (1.0 + ((frequency_hz - flo) / delf).tanh())
    }

    #[inline]
    pub(super) fn hfet_ac_gds_scale(&self, temp_k: Value, frequency_hz: Value) -> Value {
        if !frequency_hz.is_finite() || frequency_hz < 0.0 {
            return 1.0;
        }
        if self.params.hfet_kappa == 0.0 {
            return 1.0;
        }
        let transition = self.params.hfet_delf_freq.abs();
        if transition <= 0.0 {
            return 1.0;
        }

        let scale = Self::temperature_shape_scale(temp_k, self.params.hfet_tf);
        let fgds = self.params.hfet_fgds.max(0.0) * scale;
        let delf = transition * scale;
        1.0 + 0.5 * self.params.hfet_kappa * (1.0 + ((frequency_hz - fgds) / delf).tanh())
    }

    /// Resolve branch voltages used for nonlinear stamping.
    ///
    /// Prefer the device state updated in `update()` (which may include
    /// HFET-specific limiting), and fall back to raw terminal differences if
    /// no state is available yet.
    #[inline]
    pub(super) fn state_or_raw_branch_voltages(&self, voltages: &[Value]) -> (Value, Value, Value) {
        if self.vgs.is_finite() && self.vds.is_finite() {
            let vgs = self.vgs;
            let vds = self.vds;
            let vgd = vgs - vds;
            return (vgs, vds, vgd);
        }

        let vd = Self::node_voltage(voltages, self.drain);
        let vg = Self::node_voltage(voltages, self.gate);
        let vs = Self::node_voltage(voltages, self.source);

        let vgs = vg - vs;
        let vgd = vg - vd;
        let vds = vgs - vgd;
        (vgs, vds, vgd)
    }

    /// HFET branch-voltage limiting and startup seed.
    ///
    /// Returns `(vgs, vgd)` in external terminal orientation.
    #[inline]
    pub(super) fn hfet_limited_branch_voltages(
        &self,
        vgs_new: Value,
        vgd_new: Value,
    ) -> (Value, Value) {
        let pol = self.jfet_type.polarity();
        if !pol.is_finite() || pol.abs() < 0.5 {
            return (vgs_new, vgd_new);
        }

        if !self.vgs.is_finite() || !self.vds.is_finite() {
            // Match ngspice MODEINITJCT startup for active HFET devices:
            // seed internal vgs/vgd to -1V so Newton lands on the intended
            // low-current branch before regular limiting takes over.
            let seed = -1.0 / pol;
            return (seed, seed);
        }

        let vto_int = pol * self.params.vto;
        let vgs_old_int = pol * self.vgs;
        let vgd_old_int = pol * (self.vgs - self.vds);
        let vgs_new_int = pol * vgs_new;
        let vgd_new_int = pol * vgd_new;
        let vgs_limited_int = Self::fetlim(vgs_new_int, vgs_old_int, vto_int);
        let vgd_limited_int = Self::fetlim(vgd_new_int, vgd_old_int, vto_int);
        (vgs_limited_int / pol, vgd_limited_int / pol)
    }

    #[inline]
    pub(super) fn compute_operating_terms(
        &self,
        vgs: Value,
        vds: Value,
        vgd: Value,
    ) -> (Value, Value, Value, Value, Value, Value, Value, Value) {
        let mut vds_linear = vds;
        let (mut ids, mut gm, mut gds_raw) = self.calculate(vgs, vds, self.params.tnom);
        if self.hfet_legacy_inverse_active
            && matches!(self.params.channel_model, JfetChannelModel::Hfet1)
            && vds >= 0.0
        {
            match self.params.hfet_level {
                2..=4 => {
                    let (_, temp_source, _) = self.resolved_temperatures(self.params.tnom);
                    let (ids_legacy, gm_legacy, gds_legacy) = self.calculate_mesa_level(
                        vgs,
                        vds,
                        temp_source,
                        self.params.hfet_level,
                        true,
                    );
                    ids = ids_legacy;
                    gm = gm_legacy;
                    gds_raw = gds_legacy;
                }
                5.. => {
                    let (ids_forward, gm_forward, gds_forward) =
                        self.calculate(vgs, vds.abs(), self.params.tnom);
                    ids = -ids_forward;
                    gm = gm_forward;
                    gds_raw = gds_forward;
                    vds_linear = -vds.abs();
                }
                _ => {}
            }
        }
        let (igs, igd, ggs, ggd) = self.gate_junctions(vgs, vgd, self.params.tnom);
        let gds = if gds_raw.is_finite() { gds_raw } else { 0.0 };
        (ids, gm, gds, igs, igd, ggs, ggd, vds_linear)
    }

    /// Calculate classic Shichman-Hodges drain current and conductances.
    pub(super) fn calculate_shichman_hodges(
        &self,
        vgs: Value,
        vds: Value,
    ) -> (Value, Value, Value) {
        let pol = self.jfet_type.polarity();

        // Apply polarity for P-JFET
        let vgs_int = pol * vgs;
        let vds_int = pol * vds;

        let vto = self.params.vto;
        let beta = self.params.beta * self.area * self.m;
        let lambda = self.params.lambda;

        // Effective Vgs (gate-source overdrive)
        let vgst = vgs_int - vto;

        let (ids, gm, gds) = if vgst <= 0.0 {
            // Cutoff region
            (0.0, 0.0, 0.0)
        } else if vds_int < 0.0 {
            // Reverse operation - swap drain and source
            // This handles the symmetric JFET behavior
            let vds_rev = -vds_int;
            let vgs_rev = vgs_int - vds_int;
            let vgst_rev = vgs_rev - vto;

            if vgst_rev <= 0.0 {
                (0.0, 0.0, 0.0)
            } else if vds_rev <= vgst_rev {
                // Linear (reversed)
                // Evaluate forward current from swapped terminals, then map back
                // to the original drain-source orientation.
                let ids_fwd = beta
                    * (2.0 * vgst_rev * vds_rev - vds_rev * vds_rev)
                    * (1.0 + lambda * vds_rev);
                let gm_fwd = 2.0 * beta * vds_rev * (1.0 + lambda * vds_rev);
                let gds_fwd = beta * 2.0 * (vgst_rev - vds_rev) * (1.0 + lambda * vds_rev)
                    + beta * (2.0 * vgst_rev * vds_rev - vds_rev * vds_rev) * lambda;
                (-ids_fwd, -gm_fwd, gm_fwd + gds_fwd)
            } else {
                // Saturation (reversed)
                let ids_fwd = beta * vgst_rev * vgst_rev * (1.0 + lambda * vds_rev);
                let gm_fwd = 2.0 * beta * vgst_rev * (1.0 + lambda * vds_rev);
                let gds_fwd = beta * vgst_rev * vgst_rev * lambda;
                (-ids_fwd, -gm_fwd, gm_fwd + gds_fwd)
            }
        } else if vds_int <= vgst {
            // Linear (triode) region: Vds < Vgs - Vto
            let ids = beta * (2.0 * vgst * vds_int - vds_int * vds_int) * (1.0 + lambda * vds_int);

            // gm = dIds/dVgs = 2 * beta * Vds * (1 + lambda * Vds)
            let gm = 2.0 * beta * vds_int * (1.0 + lambda * vds_int);

            // gds = dIds/dVds
            let gds = beta * 2.0 * (vgst - vds_int) * (1.0 + lambda * vds_int)
                + beta * (2.0 * vgst * vds_int - vds_int * vds_int) * lambda;

            (ids, gm, gds)
        } else {
            // Saturation region: Vds >= Vgs - Vto
            let ids = beta * vgst * vgst * (1.0 + lambda * vds_int);
            let gm = 2.0 * beta * vgst * (1.0 + lambda * vds_int);
            let gds = beta * vgst * vgst * lambda;
            (ids, gm, gds)
        };

        // Apply polarity for output current
        (pol * ids, gm, gds)
    }

    /// Calculate Berkeley SPICE level-1 MESFET drain current and conductances.
    pub(super) fn calculate_legacy_mesfet(&self, vgs: Value, vds: Value) -> (Value, Value, Value) {
        let pol = self.jfet_type.polarity();
        let vgs_int = pol * vgs;
        let vds_int = pol * vds;
        let vgd_int = vgs_int - vds_int;
        let beta = (self.params.beta * self.area * self.m).max(0.0);
        let lambda = self.params.lambda;
        let alpha = if self.params.mesa_alpha.is_finite() && self.params.mesa_alpha.abs() > 1.0e-30
        {
            self.params.mesa_alpha
        } else {
            2.0
        };
        let b = self.params.mes_b.max(0.0);
        let vto = self.params.vto;

        let (ids, gm, gds) = if vds_int >= 0.0 {
            let vgst = vgs_int - vto;
            if vgst <= 0.0 {
                (0.0, 0.0, 0.0)
            } else {
                let prod = 1.0 + lambda * vds_int;
                let betap = beta * prod;
                let denom = (1.0 + b * vgst).max(1.0e-30);
                let inv_denom = 1.0 / denom;
                let vgst2_over_denom = vgst * vgst * inv_denom;
                let gm_sat = betap * vgst * (1.0 + denom) * inv_denom * inv_denom;
                if vds_int >= 3.0 / alpha {
                    (
                        betap * vgst2_over_denom,
                        gm_sat,
                        lambda * beta * vgst2_over_denom,
                    )
                } else {
                    let afact = 1.0 - alpha * vds_int / 3.0;
                    let lfact = 1.0 - afact * afact * afact;
                    (
                        betap * vgst2_over_denom * lfact,
                        gm_sat * lfact,
                        beta * vgst2_over_denom * (alpha * afact * afact * prod + lfact * lambda),
                    )
                }
            }
        } else {
            let vgdt = vgd_int - vto;
            if vgdt <= 0.0 {
                (0.0, 0.0, 0.0)
            } else {
                let prod = 1.0 - lambda * vds_int;
                let betap = beta * prod;
                let denom = (1.0 + b * vgdt).max(1.0e-30);
                let inv_denom = 1.0 / denom;
                let vgdt2_over_denom = vgdt * vgdt * inv_denom;
                let gm_sat = -betap * vgdt * (1.0 + denom) * inv_denom * inv_denom;
                if -vds_int >= 3.0 / alpha {
                    (
                        -betap * vgdt2_over_denom,
                        gm_sat,
                        lambda * beta * vgdt2_over_denom - gm_sat,
                    )
                } else {
                    let afact = 1.0 + alpha * vds_int / 3.0;
                    let lfact = 1.0 - afact * afact * afact;
                    (
                        -betap * vgdt2_over_denom * lfact,
                        gm_sat * lfact,
                        beta * vgdt2_over_denom * (alpha * afact * afact * prod + lfact * lambda)
                            - gm_sat * lfact,
                    )
                }
            }
        };

        (
            if ids.is_finite() { pol * ids } else { 0.0 },
            if gm.is_finite() { gm } else { 0.0 },
            if gds.is_finite() { gds } else { 0.0 },
        )
    }
}
