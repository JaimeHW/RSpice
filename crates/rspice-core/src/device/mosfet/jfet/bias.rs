//! Bias limiting, gate leakage, temperature, and operating-point helpers.

use super::*;

impl Jfet {
    /// Thermal voltage at given temperature
    pub(super) fn thermal_voltage(&self, temp: Value) -> Value {
        use crate::constants::K_BOLTZMANN;
        use crate::constants::Q_ELECTRON;
        K_BOLTZMANN * temp / Q_ELECTRON
    }

    #[inline]
    pub(super) fn junction_scale(&self) -> Value {
        self.area * self.m
    }

    #[inline]
    pub(crate) fn set_analysis_temperature(&mut self, temp: Value) {
        if temp.is_finite() && temp > 0.0 {
            self.analysis_temp = temp;
            self.eval_valid = false;
        }
    }

    #[inline]
    pub(crate) fn analysis_temperature(&self) -> Value {
        if self.analysis_temp.is_finite() && self.analysis_temp > 0.0 {
            self.analysis_temp
        } else if self.params.tnom.is_finite() && self.params.tnom > 0.0 {
            self.params.tnom
        } else {
            crate::constants::TEMP_REFERENCE
        }
    }

    #[inline]
    pub(super) fn resolved_temperatures(&self, ambient: Value) -> (Value, Value, Value) {
        let mut base = if ambient.is_finite() && ambient > 0.0 {
            ambient
        } else if self.params.tnom.is_finite() && self.params.tnom > 0.0 {
            self.params.tnom
        } else {
            crate::constants::TEMP_REFERENCE
        };

        if let Some(temp) = self.instance_temp.filter(|v| v.is_finite() && *v > 0.0) {
            base = temp;
        } else {
            base += self.instance_dtemp;
        }
        if !base.is_finite() || base <= 0.0 {
            base = if self.params.tnom.is_finite() && self.params.tnom > 0.0 {
                self.params.tnom
            } else {
                crate::constants::TEMP_REFERENCE
            };
        }

        let ts = self
            .instance_ts
            .filter(|v| v.is_finite() && *v > 0.0)
            .unwrap_or(base);
        let td = self
            .instance_td
            .filter(|v| v.is_finite() && *v > 0.0)
            .unwrap_or(base);

        (base, ts, td)
    }

    /// Derive Xyce's native LEVEL=1 JFET temperature state for an ambient
    /// analysis temperature, including instance `TEMP`/`DTEMP` resolution.
    #[inline]
    pub(super) fn xyce_jfet1_temperature_terms(&self, ambient: Value) -> XyceJfet1TemperatureTerms {
        let (temperature, _, _) = self.resolved_temperatures(ambient);
        self.xyce_jfet1_temperature_terms_at(temperature)
    }

    /// Mirror Xyce 7.10 `N_DEV_JFET.C::Instance::updateTemperature()` at an
    /// already-resolved device temperature.
    pub(super) fn xyce_jfet1_temperature_terms_at(
        &self,
        temperature: Value,
    ) -> XyceJfet1TemperatureTerms {
        use crate::constants::{TEMP_REFERENCE, XYCE_K_BOLTZMANN, XYCE_Q_ELECTRON};

        const SILICON_BANDGAP_REFERENCE: Value = 1.115_087_7;
        const SILICON_BANDGAP_ZERO_KELVIN: Value = 1.16;
        const SILICON_BANDGAP_TEMPERATURE_COEFFICIENT: Value = 7.02e-4;
        const SILICON_BANDGAP_TEMPERATURE_OFFSET: Value = 1108.0;
        const JUNCTION_POTENTIAL_TEMPERATURE_COEFFICIENT: Value = 4.0e-4;

        let tnom = if self.params.tnom.is_finite() && self.params.tnom > 0.0 {
            self.params.tnom
        } else {
            TEMP_REFERENCE
        };
        let temperature = if temperature.is_finite() && temperature > 0.0 {
            temperature
        } else {
            tnom
        };
        let model_junction_potential = if self.params.pb.is_finite() && self.params.pb > 0.0 {
            self.params.pb
        } else {
            Value::MIN_POSITIVE
        };
        let k_over_q = XYCE_K_BOLTZMANN / XYCE_Q_ELECTRON;

        let nominal_thermal_voltage = tnom * k_over_q;
        let nominal_reference_ratio = tnom / TEMP_REFERENCE;
        let nominal_thermal_energy = XYCE_K_BOLTZMANN * tnom;
        let nominal_bandgap = SILICON_BANDGAP_ZERO_KELVIN
            - SILICON_BANDGAP_TEMPERATURE_COEFFICIENT * tnom * tnom
                / (tnom + SILICON_BANDGAP_TEMPERATURE_OFFSET);
        let nominal_bandgap_argument = -nominal_bandgap / (2.0 * nominal_thermal_energy)
            + SILICON_BANDGAP_REFERENCE / (2.0 * XYCE_K_BOLTZMANN * TEMP_REFERENCE);
        let nominal_potential_correction = -2.0
            * nominal_thermal_voltage
            * (1.5 * nominal_reference_ratio.ln() + XYCE_Q_ELECTRON * nominal_bandgap_argument);
        let zero_kelvin_junction_potential =
            (model_junction_potential - nominal_potential_correction) / nominal_reference_ratio;
        let safe_zero_kelvin_junction_potential =
            if zero_kelvin_junction_potential.abs() > Value::MIN_POSITIVE {
                zero_kelvin_junction_potential
            } else {
                Value::MIN_POSITIVE.copysign(zero_kelvin_junction_potential)
            };
        let nominal_gamma = (model_junction_potential - zero_kelvin_junction_potential)
            / safe_zero_kelvin_junction_potential;
        let nominal_capacitance_denominator = 1.0
            + 0.5
                * (JUNCTION_POTENTIAL_TEMPERATURE_COEFFICIENT * (tnom - TEMP_REFERENCE)
                    - nominal_gamma);
        let nominal_capacitance_factor = if nominal_capacitance_denominator.is_finite()
            && nominal_capacitance_denominator.abs() > Value::MIN_POSITIVE
        {
            nominal_capacitance_denominator.recip()
        } else {
            1.0
        };

        let thermal_voltage = temperature * k_over_q;
        let temperature_reference_ratio = temperature / TEMP_REFERENCE;
        let temperature_ratio_delta = temperature / tnom - 1.0;
        let saturation_scale = (temperature_ratio_delta * 1.11 / thermal_voltage).exp();
        let geometry_scale = self.junction_scale().max(0.0);
        let model_saturation_current = self.params.is.max(0.0);
        let saturation_current = model_saturation_current * saturation_scale * geometry_scale;

        let thermal_energy = XYCE_K_BOLTZMANN * temperature;
        let bandgap = SILICON_BANDGAP_ZERO_KELVIN
            - SILICON_BANDGAP_TEMPERATURE_COEFFICIENT * temperature * temperature
                / (temperature + SILICON_BANDGAP_TEMPERATURE_OFFSET);
        let bandgap_argument = -bandgap / (2.0 * thermal_energy)
            + SILICON_BANDGAP_REFERENCE / (2.0 * XYCE_K_BOLTZMANN * TEMP_REFERENCE);
        let potential_correction = -2.0
            * thermal_voltage
            * (1.5 * temperature_reference_ratio.ln() + XYCE_Q_ELECTRON * bandgap_argument);
        let raw_junction_potential =
            temperature_reference_ratio * zero_kelvin_junction_potential + potential_correction;
        let junction_potential =
            if raw_junction_potential.is_finite() && raw_junction_potential > Value::MIN_POSITIVE {
                raw_junction_potential
            } else {
                Value::MIN_POSITIVE
            };
        let temperature_gamma = (junction_potential - zero_kelvin_junction_potential)
            / safe_zero_kelvin_junction_potential;
        let temperature_capacitance_factor = 1.0
            + 0.5
                * (JUNCTION_POTENTIAL_TEMPERATURE_COEFFICIENT * (temperature - TEMP_REFERENCE)
                    - temperature_gamma);
        let capacitance_factor = nominal_capacitance_factor * temperature_capacitance_factor;
        let gate_source_capacitance =
            (self.params.cgs * capacitance_factor * geometry_scale).max(0.0);
        let gate_drain_capacitance =
            (self.params.cgd * capacitance_factor * geometry_scale).max(0.0);

        // Xyce caps FC at 0.95 before deriving its fixed, square-root
        // depletion-capacitance continuation coefficients.
        let forward_bias_coefficient = self.params.fc.min(0.95);
        let log_one_minus_fc = (1.0 - forward_bias_coefficient).ln();
        let depletion_denominator = (1.5 * log_one_minus_fc).exp();
        let depletion_linear_factor = 1.0 - 1.5 * forward_bias_coefficient;
        let depletion_transition_voltage = forward_bias_coefficient * junction_potential;
        let depletion_charge_at_transition =
            2.0 * junction_potential * (1.0 - (0.5 * log_one_minus_fc).exp());
        let critical_voltage = if saturation_current.is_finite() && saturation_current > 0.0 {
            thermal_voltage
                * (thermal_voltage / (core::f64::consts::SQRT_2 * saturation_current)).ln()
        } else {
            Value::INFINITY
        };

        XyceJfet1TemperatureTerms {
            thermal_voltage,
            saturation_current,
            junction_potential,
            gate_source_capacitance,
            gate_drain_capacitance,
            depletion_transition_voltage,
            depletion_charge_at_transition,
            depletion_denominator,
            depletion_linear_factor,
            critical_voltage,
        }
    }

    /// Xyce LEVEL=1 gate-junction current and conductance at an
    /// already-resolved device temperature.
    #[inline]
    pub(super) fn xyce_jfet1_junction_terms(
        &self,
        voltage: Value,
        terms: XyceJfet1TemperatureTerms,
    ) -> (Value, Value) {
        let gmin = if self.junction_gmin.is_finite() {
            self.junction_gmin.max(0.0)
        } else {
            0.0
        };
        if voltage <= -5.0 * terms.thermal_voltage {
            let conductance = -terms.saturation_current / voltage + gmin;
            (conductance * voltage, conductance)
        } else {
            let exponential = (voltage / terms.thermal_voltage).exp();
            let conductance = terms.saturation_current * exponential / terms.thermal_voltage + gmin;
            let current = terms.saturation_current * (exponential - 1.0) + gmin * voltage;
            (current, conductance)
        }
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

    /// HFET2 gate current branch (`hfet2load.c`): JS Schottky plus GGR term.
    pub(super) fn hfet2_gate_branch(&self, v_int: Value, temp: Value) -> (Value, Value) {
        let temp_k = if temp.is_finite() && temp > 0.0 {
            temp
        } else {
            self.params.tnom.max(1.0)
        };
        let vt = self.thermal_voltage(temp_k).max(1e-12);
        let scale = self.hfet_gate_geometry_scale();
        let js_lw = self.params.hfet_js.max(0.0) * scale;
        let ggr_lw =
            self.params.hfet_ggr.max(0.0) * self.gate_generation_scale.clamp(0.0, 1.0) * scale;
        let vtn = (self.params.n.max(1e-12) * vt).max(1e-12);
        let exp_gate = Self::exp_limited(v_int / vtn);
        let arg = -v_int * self.params.hfet_del / vt;
        let arg_eff = arg.clamp(-80.0, 80.0);
        let earg = arg_eff.exp();

        let current = js_lw * (exp_gate - 1.0) + ggr_lw * v_int * earg;
        let conductance = js_lw * exp_gate / vtn + ggr_lw * earg * (1.0 - arg_eff);
        (
            if current.is_finite() { current } else { 0.0 },
            if conductance.is_finite() {
                conductance.max(0.0)
            } else {
                0.0
            },
        )
    }

    /// MESA gate branch approximation (`mesaload.c`): ASTAR Schottky + GGR + GMIN.
    pub(super) fn mesa_gate_branch(&self, v_int: Value, temp: Value) -> (Value, Value) {
        use crate::constants::K_BOLTZMANN;

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

    /// True when the deck marked this instance `OFF`.
    pub fn is_initially_off(&self) -> bool {
        self.initial_off
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
    /// Cached operating-point values from the last accepted Newton
    /// solution: `(vgs, vds, ids, gm, gds, igs, igd)`.
    pub fn op_values(&self) -> (Value, Value, Value, Value, Value, Value, Value) {
        (
            self.vgs,
            self.vds,
            self.eval_ids,
            self.eval_gm,
            self.eval_gds,
            self.eval_igs,
            self.eval_igd,
        )
    }

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
        let gate_model_active =
            matches!(self.params.hfet_level, 2..=4) || self.params.hfet_level >= 5;
        matches!(self.params.channel_model, JfetChannelModel::Hfet1)
            && gate_model_active
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

    /// Xyce's legacy PN-junction limiter (`DeviceSupport::pnjlim`).
    ///
    /// Xyce's native JFET deliberately uses the original SPICE limiter, not
    /// the later ngspice variant with reverse-voltage limiting.
    #[inline]
    fn xyce_pnjlim(vnew: Value, vold: Value, vt: Value, vcrit: Value) -> Value {
        if !vnew.is_finite()
            || !vold.is_finite()
            || !vt.is_finite()
            || !vcrit.is_finite()
            || vt <= 0.0
        {
            return vnew;
        }

        if (vnew > vcrit) && ((vnew - vold).abs() > 2.0 * vt) {
            if vold > 0.0 {
                let argument = 1.0 + (vnew - vold) / vt;
                if argument > 0.0 {
                    vold + vt * argument.ln()
                } else {
                    vcrit
                }
            } else {
                vt * (vnew / vt).ln()
            }
        } else {
            vnew
        }
    }

    /// Xyce's native FET limiter (`DeviceSupport::fetlim`).
    #[inline]
    fn xyce_fetlim(vnew: Value, vold: Value, vto: Value) -> Value {
        if !vnew.is_finite() || !vold.is_finite() || !vto.is_finite() {
            return vnew;
        }

        let high_step = (2.0 * (vold - vto)).abs() + 2.0;
        let low_step = high_step / 2.0 + 2.0;
        let high_gate_voltage = vto + 3.5;
        let delta = vnew - vold;

        if vold >= vto {
            if vold >= high_gate_voltage {
                if delta <= 0.0 {
                    if vnew >= high_gate_voltage {
                        if -delta > low_step {
                            vold - low_step
                        } else {
                            vnew
                        }
                    } else {
                        vnew.max(vto + 2.0)
                    }
                } else if delta >= high_step {
                    vold + high_step
                } else {
                    vnew
                }
            } else if delta <= 0.0 {
                vnew.max(vto - 0.5)
            } else {
                vnew.min(vto + 4.0)
            }
        } else if delta <= 0.0 {
            if -delta > high_step {
                vold - high_step
            } else {
                vnew
            }
        } else {
            let transition_voltage = vto + 0.5;
            if vnew <= transition_voltage {
                if delta > low_step {
                    vold + low_step
                } else {
                    vnew
                }
            } else {
                transition_voltage
            }
        }
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

    /// The MODEINITJCT startup bias both gate junctions take before any Newton
    /// iterate exists, in external terminal orientation.
    ///
    /// Every FET-family loader in ngspice (jfetload.c, jfet2load.c,
    /// hfetload.c, mesload.c, mesaload.c) writes the same pair of arms: an
    /// active instance starts at `vgs = vgd = -1`, and one the deck marked
    /// `OFF` starts at `vgs = vgd = 0`. Neither arm is gated on a
    /// compatibility mode, so the keyword applies under every dialect.
    #[inline]
    pub(super) fn ngspice_startup_branch_seed(&self, pol: Value) -> Value {
        if self.initial_off { 0.0 } else { -1.0 / pol }
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
            let seed = self.ngspice_startup_branch_seed(pol);
            return (seed, seed);
        }

        let temp_k = self.analysis_temperature().max(1.0);
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

    /// Apply the exact Xyce LEVEL=1 JFET `pnjlim`/`fetlim` sequence.
    #[inline]
    pub(super) fn xyce_jfet1_limited_branch_voltages(
        &self,
        vgs_new: Value,
        vgd_new: Value,
    ) -> (Value, Value) {
        let polarity = self.jfet_type.polarity();
        if !polarity.is_finite() || polarity.abs() < 0.5 {
            return (vgs_new, vgd_new);
        }

        if !self.vgs.is_finite() || !self.vds.is_finite() {
            // Xyce treats the current branch voltages as their own history in
            // a no-history DC operating-point evaluation.
            return (vgs_new, vgd_new);
        }

        let terms = self.xyce_jfet1_temperature_terms(self.analysis_temperature());
        let vgs_old_internal = polarity * self.vgs;
        let vgd_old_internal = polarity * (self.vgs - self.vds);
        let mut vgs_internal = polarity * vgs_new;
        let mut vgd_internal = polarity * vgd_new;

        vgs_internal = Self::xyce_pnjlim(
            vgs_internal,
            vgs_old_internal,
            terms.thermal_voltage,
            terms.critical_voltage,
        );
        vgd_internal = Self::xyce_pnjlim(
            vgd_internal,
            vgd_old_internal,
            terms.thermal_voltage,
            terms.critical_voltage,
        );
        vgs_internal = Self::xyce_fetlim(vgs_internal, vgs_old_internal, self.params.vto);
        vgd_internal = Self::xyce_fetlim(vgd_internal, vgd_old_internal, self.params.vto);

        (vgs_internal / polarity, vgd_internal / polarity)
    }

    #[inline]
    pub(super) fn mesa_gate_csat(&self, temp_k: Value) -> Value {
        use crate::constants::K_BOLTZMANN;

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

    #[inline]
    pub(super) fn external_terminal_voltages(&self, voltages: &[Value]) -> (Value, Value) {
        (
            Self::node_voltage(voltages, self.external_drain),
            Self::node_voltage(voltages, self.external_source),
        )
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
            let seed = self.ngspice_startup_branch_seed(pol);
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
    #[allow(clippy::type_complexity)]
    pub(super) fn compute_operating_terms(
        &self,
        vgs: Value,
        vds: Value,
        vgd: Value,
    ) -> (
        Value,
        Value,
        Value,
        Value,
        Value,
        Value,
        Value,
        Value,
        Value,
        Value,
    ) {
        self.compute_operating_terms_with_terminals(vgs, vds, vgd, vds, 0.0)
    }

    #[inline]
    #[allow(clippy::type_complexity)]
    pub(super) fn compute_operating_terms_with_terminals(
        &self,
        vgs: Value,
        vds: Value,
        vgd: Value,
        external_vd: Value,
        external_vs: Value,
    ) -> (
        Value,
        Value,
        Value,
        Value,
        Value,
        Value,
        Value,
        Value,
        Value,
        Value,
    ) {
        if matches!(self.params.channel_model, JfetChannelModel::ParkerSkellern) {
            let terms = self.jfet2_operating_terms(vgs, vds, vgd, self.analysis_temperature());
            return (
                terms.ids, terms.gm, terms.gds, terms.igs, terms.igd, terms.ggs, terms.ggd, vds,
                0.0, 0.0,
            );
        }
        if matches!(
            self.params.channel_model,
            JfetChannelModel::XyceModifiedShockley
        ) {
            let terms = self.xyce_jfet2_operating_terms_with_terminals(
                vgs,
                vds,
                vgd,
                self.analysis_temperature(),
                external_vd,
                external_vs,
            );
            return (
                terms.ids, terms.gm, terms.gds, terms.igs, terms.igd, terms.ggs, terms.ggd, vds,
                0.0, 0.0,
            );
        }

        // GATEMOD=1 evaluates channel and gate together: the gate-drain
        // branch has no vgd conductance; its sensitivities are gmg/gmd
        // and the A1/A2 corrections are already in the channel terms.
        if matches!(self.params.channel_model, JfetChannelModel::Hfet1)
            && self.params.hfet_gatemod
            && self.params.hfet_level >= 5
        {
            let (temp_common, _, _) = self.resolved_temperatures(self.analysis_temperature());
            let (ids, gm, gds, gate) = self.calculate_hfet1_gatemod(vgs, vds, temp_common);
            let pol = self.jfet_type.polarity();
            let (igs, igd, ggs, gmg, gmd) = match gate {
                Some(g) => (pol * g.cgs, pol * g.cgd, g.ggs, g.gmg, g.gmd),
                None => (0.0, 0.0, 0.0, 0.0, 0.0),
            };
            let gds = if gds.is_finite() { gds } else { 0.0 };
            return (ids, gm, gds, igs, igd, ggs, 0.0, vds, gmg, gmd);
        }

        let mut vds_linear = vds;
        let analysis_temp = self.analysis_temperature();
        let (mut ids, mut gm, mut gds_raw) = self.calculate(vgs, vds, analysis_temp);
        if self.hfet_legacy_inverse_active
            && matches!(self.params.channel_model, JfetChannelModel::Hfet1)
            && vds >= 0.0
        {
            match self.params.hfet_level {
                2..=4 => {
                    let (_, temp_source, _) = self.resolved_temperatures(analysis_temp);
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
                        self.calculate(vgs, vds.abs(), analysis_temp);
                    ids = -ids_forward;
                    gm = gm_forward;
                    gds_raw = gds_forward;
                    vds_linear = -vds.abs();
                }
                _ => {}
            }
        }
        let (igs, igd, ggs, ggd) = self.gate_junctions(vgs, vgd, analysis_temp);
        let gds = if gds_raw.is_finite() { gds_raw } else { 0.0 };
        (ids, gm, gds, igs, igd, ggs, ggd, vds_linear, 0.0, 0.0)
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

    /// Calculate Xyce level-1 Sydney University JFET drain current.
    pub(super) fn calculate_xyce_sydney_level1(
        &self,
        vgs: Value,
        vds: Value,
        temperature: Value,
    ) -> (Value, Value, Value) {
        let pol = self.jfet_type.polarity();
        let vgs_int = pol * vgs;
        let vds_int = pol * vds;
        let vgd_int = vgs_int - vds_int;

        let beta = (self.params.beta * self.area * self.m).max(0.0);
        let vto = self.params.vto;
        let lambda = self.params.lambda;
        let pb = self
            .xyce_jfet1_temperature_terms_at(temperature)
            .junction_potential;
        let b = self.params.mes_b;
        let bfac_numerator = 1.0 - b;
        let bfac_base = if bfac_numerator == 0.0 {
            0.0
        } else {
            bfac_numerator / (pb - vto)
        };

        let (cdrain, gm, gds) = if vds_int >= 0.0 {
            let vgst = vgs_int - vto;
            if vgst <= 0.0 {
                (0.0, 0.0, 0.0)
            } else {
                let betap = beta * (1.0 + lambda * vds_int);
                if vgst >= vds_int {
                    let apart = 2.0 * b + 3.0 * bfac_base * (vgst - vds_int);
                    let cpart = vds_int * (vds_int * (bfac_base * vds_int - b) + vgst * apart);
                    let cdrain = betap * cpart;
                    let gm = betap * vds_int * (apart + 3.0 * bfac_base * vgst);
                    let gds = betap * (vgst - vds_int) * apart + lambda * beta * cpart;
                    (cdrain, gm, gds)
                } else {
                    let bfac = vgst * bfac_base;
                    let cpart = vgst * vgst * (b + bfac);
                    let cdrain = betap * cpart;
                    let gm = betap * vgst * (2.0 * b + 3.0 * bfac);
                    let gds = lambda * beta * cpart;
                    (cdrain, gm, gds)
                }
            }
        } else {
            let vgdt = vgd_int - vto;
            if vgdt <= 0.0 {
                (0.0, 0.0, 0.0)
            } else {
                let betap = beta * (1.0 - lambda * vds_int);
                if vgdt + vds_int >= 0.0 {
                    let apart = 2.0 * b + 3.0 * bfac_base * (vgdt + vds_int);
                    let cpart = vds_int * (-vds_int * (-bfac_base * vds_int - b) + vgdt * apart);
                    let cdrain = betap * cpart;
                    let gm = betap * vds_int * (apart + 3.0 * bfac_base * vgdt);
                    let gds = betap * (vgdt + vds_int) * apart - lambda * beta * cpart - gm;
                    (cdrain, gm, gds)
                } else {
                    let bfac = vgdt * bfac_base;
                    let cpart = vgdt * vgdt * (b + bfac);
                    let cdrain = -betap * cpart;
                    let gm = -betap * vgdt * (2.0 * b + 3.0 * bfac);
                    let gds = lambda * beta * cpart - gm;
                    (cdrain, gm, gds)
                }
            }
        };

        (pol * cdrain, gm, gds)
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

#[cfg(test)]
mod xyce_jfet1_tests {
    use super::*;
    use std::collections::HashMap;

    fn assert_close(actual: Value, expected: Value, relative_tolerance: Value, label: &str) {
        let tolerance = (relative_tolerance * expected.abs()).max(1.0e-30);
        assert!(
            (actual - expected).abs() <= tolerance,
            "{label}: actual={actual:.17e}, expected={expected:.17e}, tolerance={tolerance:.3e}"
        );
    }

    fn temperature_oracle_device() -> Jfet {
        let mut device = Jfet::njf("j_oracle", 1, 2, 3).enable_xyce_jfet1_model();
        device.params.cgs = 2.0e-12;
        device.params.cgd = 1.0e-12;
        device
    }

    fn sa2109_njf() -> Jfet {
        let mut device = Jfet::njf("j_sa2109", 1, 2, 3).enable_xyce_jfet1_model();
        device.params.beta = 2.690e-5;
        device.params.vto = -3.795;
        device.params.pb = 1.07;
        device.params.lambda = 0.0181;
        device.params.mes_b = 0.605;
        device.params.is = 1.393e-10;
        device
    }

    fn sa2108_pjf() -> Jfet {
        let mut device = Jfet::pjf("j_sa2108", 1, 2, 3).enable_xyce_jfet1_model();
        device.params.beta = 2.78e-4;
        device.params.vto = -2.10;
        device.params.pb = 0.265;
        device.params.lambda = 0.0055;
        device.params.mes_b = 0.590;
        device.params.is = 1.393e-10;
        device
    }

    #[test]
    fn xyce_jfet1_temperature_state_matches_xyce_710_equations() {
        // Independent anchors evaluated from Xyce 7.10
        // N_DEV_JFET.C::Instance::updateTemperature() using its rounded
        // physical constants. The device uses default PB/IS/TNOM and explicit
        // CGS/CGD so every temperature-dependent quantity is exercised.
        let device = temperature_oracle_device();
        let anchors = [
            (
                288.15,
                1.674_189_265_294_86e-15,
                1.010_804_127_910_272,
                1.984_395_871_304_202_2e-12,
                0.024_830_135_954_384_484,
                0.744_436_690_384_469_3,
            ),
            (
                298.15,
                7.498_476_693_804_42e-15,
                1.001_815_672_447_988_3,
                1.997_384_327_420_142e-12,
                0.025_691_844_646_190_298,
                0.732_626_618_590_806_6,
            ),
            (
                308.15,
                3.047_042_089_511_081_5e-14,
                0.992_678_497_745_189_1,
                2.010_521_502_786_015_2e-12,
                0.026_553_553_337_996_11,
                0.720_845_454_069_546_5,
            ),
        ];

        for (temperature, isat, pb, cgs, vt, vcrit) in anchors {
            let terms = device.xyce_jfet1_temperature_terms_at(temperature);
            assert_close(
                terms.saturation_current,
                isat,
                5.0e-13,
                "saturation current",
            );
            assert_close(terms.junction_potential, pb, 5.0e-14, "junction potential");
            assert_close(
                terms.gate_source_capacitance,
                cgs,
                5.0e-14,
                "gate-source capacitance",
            );
            assert_close(
                terms.gate_drain_capacitance,
                0.5 * cgs,
                5.0e-14,
                "gate-drain capacitance",
            );
            assert_close(terms.thermal_voltage, vt, 5.0e-15, "thermal voltage");
            assert_close(terms.critical_voltage, vcrit, 5.0e-14, "critical voltage");
        }

        let nominal = device.xyce_jfet1_temperature_terms_at(300.15);
        assert_close(nominal.saturation_current, 1.0e-14, 5.0e-15, "nominal IS");
        assert_close(nominal.junction_potential, 1.0, 5.0e-15, "nominal PB");
        assert_close(
            nominal.gate_source_capacitance,
            2.0e-12,
            5.0e-15,
            "nominal CGS",
        );
    }

    #[test]
    fn xyce_jfet1_temp_overrides_dtemp_and_accepts_subzero_celsius() {
        let dtemp_device =
            temperature_oracle_device().with_instance_params(&[("DTEMP".to_string(), 10.0)]);
        let temp_device = temperature_oracle_device()
            .with_instance_params(&[("TEMP".to_string(), 35.0), ("DTEMP".to_string(), -100.0)]);
        let dtemp_terms = dtemp_device.xyce_jfet1_temperature_terms(298.15);
        let temp_terms = temp_device.xyce_jfet1_temperature_terms(298.15);
        assert_eq!(
            dtemp_terms.thermal_voltage.to_bits(),
            temp_terms.thermal_voltage.to_bits(),
            "TEMP=35 C and ambient 25 C + DTEMP=10 C must resolve identically"
        );
        assert_eq!(
            dtemp_terms.junction_potential.to_bits(),
            temp_terms.junction_potential.to_bits()
        );

        let cold_device =
            temperature_oracle_device().with_instance_params(&[("TEMP".to_string(), -40.0)]);
        let cold_terms = cold_device.xyce_jfet1_temperature_terms(298.15);
        let expected_vt =
            233.15 * crate::constants::XYCE_K_BOLTZMANN / crate::constants::XYCE_Q_ELECTRON;
        assert_close(
            cold_terms.thermal_voltage,
            expected_vt,
            5.0e-15,
            "subzero TEMP",
        );
    }

    #[test]
    fn xyce_jfet1_channel_matches_both_polarity_temperature_oracles() {
        let n_device = sa2109_njf();
        let p_device = sa2108_pjf();
        let anchors = [
            (
                288.15,
                1.078_005_527_210_621_7,
                (
                    4.495_534_263_100_225e-4,
                    2.768_481_661_003_685e-4,
                    6.399_462_851_916_167e-6,
                ),
                0.305_189_435_256_598_7,
                (
                    -1.258_081_033_372_559_4e-3,
                    1.424_400_154_103_656e-3,
                    6.392_097_629_144_643e-6,
                ),
            ),
            (
                298.15,
                1.071_349_238_998_046_6,
                (
                    4.497_606_946_751_582e-4,
                    2.770_120_146_103_572_3e-4,
                    6.402_413_349_288_527e-6,
                ),
                0.271_713_223_672_376_1,
                (
                    -1.264_786_658_693_273e-3,
                    1.433_979_618_847_532_8e-3,
                    6.426_167_780_889_608e-6,
                ),
            ),
            (
                308.15,
                1.064_544_231_544_955_8,
                (
                    4.499_731_809_453_122e-4,
                    2.771_799_879_464_473_7e-4,
                    6.405_438_124_349_313e-6,
                ),
                0.238_088_292_847_637_83,
                (
                    -1.271_715_374_670_672e-3,
                    1.443_877_784_529_531_5e-3,
                    6.461_371_418_650_065e-6,
                ),
            ),
        ];

        for (temperature, n_pb, n_channel, p_pb, p_channel) in anchors {
            let n_terms = n_device.xyce_jfet1_temperature_terms_at(temperature);
            let p_terms = p_device.xyce_jfet1_temperature_terms_at(temperature);
            assert_close(n_terms.junction_potential, n_pb, 5.0e-13, "NJF tPB");
            assert_close(p_terms.junction_potential, p_pb, 5.0e-13, "PJF tPB");

            let n_actual = n_device.calculate(0.0, 15.0, temperature);
            let p_actual = p_device.calculate(0.0, -15.0, temperature);
            for (actual, expected, label) in [
                (n_actual.0, n_channel.0, "NJF ids"),
                (n_actual.1, n_channel.1, "NJF gm"),
                (n_actual.2, n_channel.2, "NJF gds"),
                (p_actual.0, p_channel.0, "PJF ids"),
                (p_actual.1, p_channel.1, "PJF gm"),
                (p_actual.2, p_channel.2, "PJF gds"),
            ] {
                assert_close(actual, expected, 5.0e-13, label);
            }
        }
    }

    #[test]
    fn xyce_jfet1_nonzero_junction_caps_follow_temperature_and_polarity() {
        let mut n_device = sa2109_njf();
        n_device.params.cgs = 1.0e-12;
        n_device.params.cgd = 2.0e-12;
        let mut p_device = sa2108_pjf();
        p_device.params.cgs = 1.0e-12;
        p_device.params.cgd = 2.0e-12;

        for (temperature, n_expected, p_expected) in [
            (
                288.15,
                (1.101_251_941_465_028_2e-12, 3.491_646_732_334_209_5e-12),
                (1.124_165_424_260_502e-12, 4.720_686_945_621_017e-12),
            ),
            (
                308.15,
                (1.114_260_128_732_948_8e-12, 3.554_451_269_688_122e-12),
                (1.381_853_055_268_949_4e-12, 6.489_070_954_119_502e-12),
            ),
        ] {
            let n_charge = n_device.xyce_jfet1_charge_state(0.2, 0.8, temperature);
            let p_charge = p_device.xyce_jfet1_charge_state(-0.1, -0.4, temperature);
            assert_close(n_charge.cgs, n_expected.0, 5.0e-13, "NJF Cgs");
            assert_close(n_charge.cgd, n_expected.1, 5.0e-13, "NJF Cgd");
            assert_close(p_charge.cgs, p_expected.0, 5.0e-13, "PJF Cgs");
            assert_close(p_charge.cgd, p_expected.1, 5.0e-13, "PJF Cgd");
            assert!(n_charge.qgs.is_finite() && n_charge.qgd.is_finite());
            assert!(p_charge.qgs.is_finite() && p_charge.qgd.is_finite());
        }
    }

    #[test]
    fn xyce_jfet1_gate_branch_uses_temperature_scaled_is_and_xfive_vt_tail() {
        let mut device = temperature_oracle_device();
        device.params.n = 9.0;
        device.junction_gmin = 2.5e-12;
        let terms = device.xyce_jfet1_temperature_terms_at(298.15);

        let reverse_voltage = -0.2;
        assert!(reverse_voltage <= -5.0 * terms.thermal_voltage);
        let (reverse_current, reverse_conductance) =
            device.xyce_jfet1_junction_terms(reverse_voltage, terms);
        let expected_reverse_conductance =
            -terms.saturation_current / reverse_voltage + device.junction_gmin;
        assert_close(
            reverse_conductance,
            expected_reverse_conductance,
            5.0e-15,
            "reverse conductance",
        );
        assert_close(
            reverse_current,
            expected_reverse_conductance * reverse_voltage,
            5.0e-15,
            "reverse current",
        );

        let exponential_voltage = -0.1;
        assert!(exponential_voltage > -5.0 * terms.thermal_voltage);
        let (current, conductance) = device.xyce_jfet1_junction_terms(exponential_voltage, terms);
        let exponential = (exponential_voltage / terms.thermal_voltage).exp();
        assert_close(
            current,
            terms.saturation_current * (exponential - 1.0)
                + device.junction_gmin * exponential_voltage,
            5.0e-15,
            "exponential current",
        );
        assert_close(
            conductance,
            terms.saturation_current * exponential / terms.thermal_voltage + device.junction_gmin,
            5.0e-15,
            "exponential conductance",
        );
    }

    #[test]
    fn xyce_jfet_model_b_is_not_a_beta_alias() {
        let mut parameters = HashMap::new();
        parameters.insert("B".to_string(), 0.605);
        let device = Jfet::njf("j_b", 1, 2, 3)
            .enable_xyce_jfet1_model()
            .with_model_params(&parameters);
        assert_eq!(device.params.beta.to_bits(), 1.0e-4_f64.to_bits());
        assert_eq!(device.params.mes_b.to_bits(), 0.605_f64.to_bits());

        parameters.insert("BETA".to_string(), 2.69e-5);
        parameters.insert("TNOM".to_string(), -40.0);
        let explicit_beta = Jfet::njf("j_beta", 1, 2, 3)
            .enable_xyce_jfet1_model()
            .with_model_params(&parameters);
        assert_eq!(explicit_beta.params.beta.to_bits(), 2.69e-5_f64.to_bits());
        assert_eq!(explicit_beta.params.mes_b.to_bits(), 0.605_f64.to_bits());
        assert_eq!(
            explicit_beta.params.tnom.to_bits(),
            (-40.0_f64 + 273.15).to_bits()
        );
    }

    #[test]
    fn xyce_jfet_limiters_match_legacy_device_support_equations() {
        let vt = 0.025;
        let vcrit = 0.6;
        let limited = Jfet::xyce_pnjlim(1.2, 0.2, vt, vcrit);
        let expected = 0.2 + vt * (1.0_f64 + (1.2 - 0.2) / vt).ln();
        assert_close(limited, expected, 5.0e-15, "Xyce pnjlim");
        assert_eq!(
            Jfet::xyce_pnjlim(-10.0, -0.2, vt, vcrit).to_bits(),
            (-10.0_f64).to_bits(),
            "Xyce's legacy pnjlim has no negative-voltage clamp"
        );

        assert_eq!(
            Jfet::xyce_fetlim(-10.0, 5.0, -2.0).to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(
            Jfet::xyce_fetlim(30.0, 5.0, -2.0).to_bits(),
            21.0_f64.to_bits()
        );
    }
}
