//! Intrinsic BJT branch equations, limiting, and static linearization.

use super::*;

impl Bjt {
    /// Calculate base-emitter junction capacitance
    /// Cbe = CJE / (1 - Vbe/VJE)^MJE + gm * TF
    pub fn cbe(&self, vbe: Value, gm: Value) -> Value {
        let p = self.polarity();
        let (_, factor) = self.vbic_depletion_charge_and_derivative(
            p * vbe,
            self.vje,
            self.mje,
            self.fc,
            self.aje,
        );
        let cj = self.cje * factor;
        let cd = gm * self.tf; // Diffusion capacitance
        cj + cd
    }

    /// Calculate base-collector junction capacitance
    /// Cbc = CJC / (1 - Vbc/VJC)^MJC
    pub fn cbc(&self, vbc: Value) -> Value {
        let p = self.polarity();
        let (_, factor) = self.vbic_depletion_charge_and_derivative(
            p * vbc,
            self.vjc,
            self.mjc,
            self.fc,
            self.ajc,
        );
        self.cjc * factor
    }

    /// Calculate total capacitances for transient analysis
    /// Returns (Cbe, Cbc)
    pub fn junction_capacitances(&self, vbe: Value, vbc: Value) -> (Value, Value) {
        let charges = self.legacy_transient_charge_state(vbe, vbc, 0.0);
        (charges.capbe, charges.capbc)
    }

    /// Return cached collector, base, and emitter currents at the operating point.
    pub fn operating_point_currents(&self) -> (Value, Value, Value) {
        (self.ic, self.ib, self.ie)
    }

    /// Return the net current leaving a physical node, summing any tied BJT terminals.
    pub(crate) fn node_current(&self, node: NodeId) -> Value {
        let mut current = 0.0;
        if self.node_collector == node {
            current += self.ic;
        }
        if self.node_base == node {
            current += self.ib;
        }
        if self.node_emitter == node {
            current += self.ie;
        }
        if self.node_substrate == node {
            current += self.isub;
        }
        current
    }

    /// Return the shot-noise branch currents referenced to the physical junctions.
    pub fn noise_branch_currents(&self) -> (Value, Value, Value) {
        let vp_be = self.polarity() * self.vbe;
        let vp_bc = self.polarity() * self.vbc;
        let ibe = self.diode_current_with_is(self.ibei, vp_be, self.nei)
            + self.diode_current_with_is(self.iben, vp_be, self.nen);
        let ibc = self.diode_current_with_is(self.ibci, vp_bc, self.nci)
            + self.diode_current_with_is(self.ibcn, vp_bc, self.ncn);
        (self.ic.abs(), ibe.abs(), ibc.abs())
    }

    /// Return flicker-noise coefficients, if enabled by the model card.
    pub fn flicker_noise_coefficients(&self) -> Option<(Value, Value, Value)> {
        if self.kf > 0.0 && self.kf.is_finite() {
            Some((self.kf, self.af.max(1e-12), self.ef.max(1e-12)))
        } else {
            None
        }
    }

    /// Link this device to a StaticMatrix for O(1) stamping
    pub fn link(&mut self, matrix: &StaticMatrix) {
        let c = self.node_collector;
        let b = self.node_base;
        let e = self.node_emitter;

        // Collector row
        if c > 0 {
            self.indices.cc = matrix.get_index(c - 1, c - 1);
        }
        if c > 0 && b > 0 {
            self.indices.cb = matrix.get_index(c - 1, b - 1);
        }
        if c > 0 && e > 0 {
            self.indices.ce = matrix.get_index(c - 1, e - 1);
        }
        // Base row
        if b > 0 && c > 0 {
            self.indices.bc = matrix.get_index(b - 1, c - 1);
        }
        if b > 0 {
            self.indices.bb = matrix.get_index(b - 1, b - 1);
        }
        if b > 0 && e > 0 {
            self.indices.be = matrix.get_index(b - 1, e - 1);
        }
        // Emitter row
        if e > 0 && c > 0 {
            self.indices.ec = matrix.get_index(e - 1, c - 1);
        }
        if e > 0 && b > 0 {
            self.indices.eb = matrix.get_index(e - 1, b - 1);
        }
        if e > 0 {
            self.indices.ee = matrix.get_index(e - 1, e - 1);
        }
    }

    /// Stamp using O(1) direct indexing (call after link)
    pub fn stamp_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value], voltages: &[Value]) {
        let [vc, vb, ve, vs] = self.external_terminal_voltages(voltages);
        let rows = self.small_signal_row_coefficients(vc, vb, ve, vs);
        let nodes = self.external_terminal_nodes();
        let biases = [vc, vb, ve, vs];
        let currents = [self.ic, self.ib, self.ie, self.isub];

        let stamp_entry =
            |matrix: &mut StaticMatrix, row_idx: usize, col_idx: usize, value: Value| {
                let row = nodes[row_idx];
                let col = nodes[col_idx];
                if row == 0 || col == 0 {
                    return;
                }

                match (row_idx, col_idx) {
                    (EXT_C, EXT_C) => {
                        if let Some(idx) = self.indices.cc {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    (EXT_C, EXT_B) => {
                        if let Some(idx) = self.indices.cb {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    (EXT_C, EXT_E) => {
                        if let Some(idx) = self.indices.ce {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    (EXT_B, EXT_C) => {
                        if let Some(idx) = self.indices.bc {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    (EXT_B, EXT_B) => {
                        if let Some(idx) = self.indices.bb {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    (EXT_B, EXT_E) => {
                        if let Some(idx) = self.indices.be {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    (EXT_E, EXT_C) => {
                        if let Some(idx) = self.indices.ec {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    (EXT_E, EXT_B) => {
                        if let Some(idx) = self.indices.eb {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    (EXT_E, EXT_E) => {
                        if let Some(idx) = self.indices.ee {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    _ => matrix.add(row - 1, col - 1, value),
                }
            };

        for row_idx in 0..EXTERNAL_DIM {
            let ieq = currents[row_idx]
                - (0..EXTERNAL_DIM)
                    .map(|col_idx| rows[row_idx][col_idx] * biases[col_idx])
                    .sum::<Value>();
            for col_idx in 0..EXTERNAL_DIM {
                stamp_entry(matrix, row_idx, col_idx, rows[row_idx][col_idx]);
            }
            if nodes[row_idx] > 0 {
                rhs[nodes[row_idx] - 1] -= ieq;
            }
        }
    }

    /// Get polarity multiplier (+1 for NPN, -1 for PNP)
    pub(super) fn polarity(&self) -> Value {
        match self.bjt_type {
            BjtType::Npn => 1.0,
            BjtType::Pnp => -1.0,
        }
    }

    /// Diode current: I = Is * (exp(V / (n * Vt)) - 1)
    ///
    /// SPICE-style voltage limiting:
    /// - Forward: limit to 80*n*Vt to prevent exp overflow
    /// - Reverse: for V < -5*n*Vt, use linear extrapolation (negligible current)
    pub(super) fn diode_current_with_is(&self, isat: Value, v: Value, n: Value) -> Value {
        let nvt = n * self.vt;
        let v_crit = 80.0 * nvt; // Forward limit
        let v_rev = -5.0 * nvt; // Reverse limit (around -0.13V at room temp)

        if v > v_crit {
            // Forward saturation - linear extrapolation
            let i_crit = isat * ((v_crit / nvt).exp() - 1.0);
            let g_crit = (isat / nvt) * (v_crit / nvt).exp();
            i_crit + g_crit * (v - v_crit)
        } else if v < v_rev {
            // Deep reverse bias - essentially just -Is (negligible)
            -isat
        } else {
            // Normal operating region
            isat * ((v / nvt).exp() - 1.0)
        }
    }

    #[inline]
    pub(super) fn diode_current(&self, v: Value, n: Value) -> Value {
        self.diode_current_with_is(self.is, v, n)
    }

    /// Diode conductance: g = Is / (n * Vt) * exp(V / (n * Vt))
    ///
    /// SPICE-style limiting with minimum conductance floor for numerical stability
    pub(super) fn diode_conductance_with_is(&self, isat: Value, v: Value, n: Value) -> Value {
        let nvt = n * self.vt;
        let v_crit = 80.0 * nvt;
        let v_rev = -5.0 * nvt;

        let g = if v > v_crit {
            // Forward saturation - constant high conductance
            (isat / nvt) * (v_crit / nvt).exp()
        } else if v < v_rev {
            // Deep reverse bias - minimum conductance
            1e-15
        } else {
            // Normal region
            (isat / nvt) * (v / nvt).exp()
        };

        // Apply minimum conductance floor
        g.max(1e-15)
    }

    #[inline]
    pub(super) fn diode_conductance(&self, v: Value, n: Value) -> Value {
        self.diode_conductance_with_is(self.is, v, n)
    }

    #[inline]
    pub(super) fn depletion_charge_base(
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

    pub(super) fn depletion_capacitance_factor(
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

    pub(super) fn vbic_depletion_charge_and_derivative(
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

    pub(super) fn legacy_transport_charge_state(
        &self,
        vbe_eff: Value,
        vbc_eff: Value,
    ) -> TransportChargeState {
        let ifi = self.diode_current(vbe_eff, self.nf).max(0.0);
        let iri = self
            .diode_current_with_is(self.is * self.isrr.max(0.0), vbc_eff, self.nr)
            .max(0.0);
        let gfi = self.diode_conductance(vbe_eff, self.nf);
        let gri = self.diode_conductance_with_is(self.is * self.isrr.max(0.0), vbc_eff, self.nr);

        let raw_q1_inv =
            1.0 - if self.var.is_finite() && self.var > 0.0 {
                vbe_eff / self.var
            } else {
                0.0
            } - if self.vaf.is_finite() && self.vaf > 0.0 {
                vbc_eff / self.vaf
            } else {
                0.0
            };
        let (q1_inv, dq1_inv_draw_q1_inv) = Self::smooth_positive_floor(raw_q1_inv, 1e-9);
        let q1 = 1.0 / q1_inv.max(1e-18);
        let dq1_dvbe_eff = if self.var.is_finite() && self.var > 0.0 {
            dq1_inv_draw_q1_inv / (self.var * q1_inv * q1_inv)
        } else {
            0.0
        };
        let dq1_dvbc_eff = if self.vaf.is_finite() && self.vaf > 0.0 {
            dq1_inv_draw_q1_inv / (self.vaf * q1_inv * q1_inv)
        } else {
            0.0
        };

        let inv_rolloff_f = if self.ikf > 0.0 { 1.0 / self.ikf } else { 0.0 };
        let inv_rolloff_r = if self.ikr > 0.0 { 1.0 / self.ikr } else { 0.0 };
        let (qb, dqb_dvbe_eff, dqb_dvbc_eff) = if inv_rolloff_f == 0.0 && inv_rolloff_r == 0.0 {
            (q1.max(1e-12), dq1_dvbe_eff, dq1_dvbc_eff)
        } else {
            let q2 = inv_rolloff_f * ifi + inv_rolloff_r * iri;
            let dq2_dvbe_eff = inv_rolloff_f * gfi;
            let dq2_dvbc_eff = inv_rolloff_r * gri;
            let sqrt_term = (1.0 + 4.0 * q2).sqrt().max(1e-18);
            (
                (0.5 * q1 * (1.0 + sqrt_term)).max(1e-12),
                0.5 * (1.0 + sqrt_term) * dq1_dvbe_eff + q1 * dq2_dvbe_eff / sqrt_term,
                0.5 * (1.0 + sqrt_term) * dq1_dvbc_eff + q1 * dq2_dvbc_eff / sqrt_term,
            )
        };

        let itzf = ifi / qb;
        let ditzf_dvbe_eff = gfi / qb - ifi * dqb_dvbe_eff / (qb * qb);
        let ditzf_dvbc_eff = -ifi * dqb_dvbc_eff / (qb * qb);
        let itzr = iri / qb;
        let ditzr_dvbe_eff = -iri * dqb_dvbe_eff / (qb * qb);
        let ditzr_dvbc_eff = gri / qb - iri * dqb_dvbc_eff / (qb * qb);

        TransportChargeState {
            q1,
            qb,
            ifi,
            iri,
            gfi,
            gri,
            dq1_dvbe_eff,
            dq1_dvbc_eff,
            itzf,
            itzr,
            dqb_dvbe_eff,
            dqb_dvbc_eff,
            ditzf_dvbe_eff,
            ditzf_dvbc_eff,
            ditzr_dvbe_eff,
            ditzr_dvbc_eff,
        }
    }

    pub(crate) fn legacy_transient_charge_state(
        &self,
        vbe: Value,
        vbc: Value,
        vcs: Value,
    ) -> LegacyTransientChargeState {
        let p = self.polarity();
        let vbe_eff = p * vbe;
        let vbc_eff = p * vbc;
        let vsub_eff = -p * vcs;
        let transport = self.legacy_transport_charge_state(vbe_eff, vbc_eff);

        let mut argtf = 0.0;
        let mut arg2 = 0.0;
        let mut arg3 = 0.0;
        if self.tf != 0.0 && vbe_eff > 0.0 && self.xtf != 0.0 {
            argtf = self.xtf;
            let mut ovtf = 0.0;
            if self.vtf > 0.0 {
                ovtf = 1.0 / (self.vtf * 1.44);
                argtf *= Self::limited_exp(vbc_eff * ovtf).0;
            }
            arg2 = argtf;
            if self.itf > 0.0 {
                let temp = transport.ifi / (transport.ifi + self.itf).max(1e-18);
                argtf *= temp * temp;
                arg2 = argtf * (3.0 - temp - temp);
            }
            arg3 = transport.ifi * argtf * ovtf;
        }

        let qb = transport.qb.max(1e-18);
        let qbe_diffusion_current = if self.tf != 0.0 {
            transport.ifi * (1.0 + argtf) / qb
        } else {
            0.0
        };
        let gbe_dynamic = if self.tf != 0.0 {
            (transport.gfi * (1.0 + arg2) - qbe_diffusion_current * transport.dqb_dvbe_eff) / qb
        } else {
            0.0
        };
        let geqcb_dynamic = if self.tf != 0.0 && vbe_eff > 0.0 {
            (arg3 - qbe_diffusion_current * transport.dqb_dvbc_eff) / qb
        } else {
            0.0
        };

        let (qbe_dep_norm, capbe_dep) =
            self.vbic_depletion_charge_and_derivative(vbe_eff, self.vje, self.mje, self.fc, 0.0);
        let (qbc_dep_norm, capbc_dep) =
            self.vbic_depletion_charge_and_derivative(vbc_eff, self.vjc, self.mjc, self.fc, 0.0);
        let (qsub_norm, capsub_dep) =
            self.vbic_depletion_charge_and_derivative(vsub_eff, self.ps, self.ms, 0.0, 0.0);

        LegacyTransientChargeState {
            qbe: p * (self.tf * qbe_diffusion_current + self.cje * qbe_dep_norm + self.cbeo * vbe),
            capbe: (self.tf * gbe_dynamic + self.cje * capbe_dep + self.cbeo).max(0.0),
            capbe_vbc: self.tf * geqcb_dynamic,
            qbc: p * (self.tr * transport.iri + self.cjc * qbc_dep_norm + self.cbco * vbc),
            capbc: (self.tr * transport.gri + self.cjc * capbc_dep + self.cbco).max(0.0),
            qcs: -p * (self.cjcp * qsub_norm),
            capcs: (self.cjcp * capsub_dep).max(0.0),
        }
    }

    #[inline]
    pub(crate) fn legacy_charge_branch_voltages(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> (Value, Value, Value) {
        let static_internal = if self.cache_matches_external_biases(vc, vb, ve, vs) {
            self.internal_state_vector()
        } else {
            let internal = self.dynamic_internal_state_seed(vc, vb, ve, vs);
            let mut static_internal = [0.0; INTERNAL_DIM];
            static_internal.copy_from_slice(&internal[..INTERNAL_DIM]);
            static_internal
        };
        (
            static_internal[IDX_VBI] - static_internal[IDX_VEI],
            static_internal[IDX_VBI] - static_internal[IDX_VCI],
            static_internal[IDX_VCI] - static_internal[IDX_VSI],
        )
    }

    pub(super) fn vbic_transport_charge_state(
        &self,
        vbe_eff: Value,
        vbc_eff: Value,
    ) -> TransportChargeState {
        let ifi = self.diode_current(vbe_eff, self.nf).max(0.0);
        let iri = self
            .diode_current_with_is(self.is * self.isrr.max(0.0), vbc_eff, self.nr)
            .max(0.0);
        let gfi = self.diode_conductance(vbe_eff, self.nf);
        let gri = self.diode_conductance_with_is(self.is * self.isrr.max(0.0), vbc_eff, self.nr);

        let (qdbe, dqdbe_dvbe_eff) = self
            .vbic_depletion_charge_and_derivative(vbe_eff, self.vje, self.mje, self.fc, self.aje);
        let (qdbc, dqdbc_dvbc_eff) = self
            .vbic_depletion_charge_and_derivative(vbc_eff, self.vjc, self.mjc, self.fc, self.ajc);

        let q1z =
            1.0 + if self.var.is_finite() && self.var > 0.0 {
                qdbe / self.var
            } else {
                0.0
            } + if self.vaf.is_finite() && self.vaf > 0.0 {
                qdbc / self.vaf
            } else {
                0.0
            };
        let q1_shift = q1z - 1e-4;
        let q1_sqrt = (q1_shift * q1_shift + 1e-8).sqrt();
        let q1 = 0.5 * (q1_sqrt + q1_shift) + 1e-4;
        let dq1_dq1z = 0.5 * (q1_shift / q1_sqrt + 1.0);
        let dq1_dvbe_eff = dq1_dq1z
            * if self.var.is_finite() && self.var > 0.0 {
                dqdbe_dvbe_eff / self.var
            } else {
                0.0
            };
        let dq1_dvbc_eff = dq1_dq1z
            * if self.vaf.is_finite() && self.vaf > 0.0 {
                dqdbc_dvbc_eff / self.vaf
            } else {
                0.0
            };

        let inv_rolloff_f = if self.ikf > 0.0 { 1.0 / self.ikf } else { 0.0 };
        let inv_rolloff_r = if self.ikr > 0.0 { 1.0 / self.ikr } else { 0.0 };
        let q2 = inv_rolloff_f * ifi + inv_rolloff_r * iri;
        let dq2_dvbe_eff = inv_rolloff_f * gfi;
        let dq2_dvbc_eff = inv_rolloff_r * gri;
        let nkf = self.nkf.max(1e-12);
        let (qb, dqb_dvbe_eff, dqb_dvbc_eff) = if self.qbm < 0.5 {
            let inv_nkf = 1.0 / nkf;
            let xvar3 = q1.max(1e-18).powf(inv_nkf);
            let dxvar3_dvbe_eff = if q1 > 0.0 {
                xvar3 * inv_nkf * dq1_dvbe_eff / q1.max(1e-18)
            } else {
                0.0
            };
            let dxvar3_dvbc_eff = if q1 > 0.0 {
                xvar3 * inv_nkf * dq1_dvbc_eff / q1.max(1e-18)
            } else {
                0.0
            };
            let xvar1 = (xvar3 + 4.0 * q2).max(1e-18);
            let dxvar1_dvbe_eff = dxvar3_dvbe_eff + 4.0 * dq2_dvbe_eff;
            let dxvar1_dvbc_eff = dxvar3_dvbc_eff + 4.0 * dq2_dvbc_eff;
            let xvar4 = xvar1.powf(nkf);
            let dxvar4_dvbe_eff = xvar4 * nkf * dxvar1_dvbe_eff / xvar1;
            let dxvar4_dvbc_eff = xvar4 * nkf * dxvar1_dvbc_eff / xvar1;
            (
                (0.5 * (q1 + xvar4)).max(1e-12),
                0.5 * (dq1_dvbe_eff + dxvar4_dvbe_eff),
                0.5 * (dq1_dvbc_eff + dxvar4_dvbc_eff),
            )
        } else {
            let xvar1 = (1.0 + 4.0 * q2).max(1e-18);
            let dxvar1_dvbe_eff = 4.0 * dq2_dvbe_eff;
            let dxvar1_dvbc_eff = 4.0 * dq2_dvbc_eff;
            let xvar2 = xvar1.powf(nkf);
            let dxvar2_dvbe_eff = xvar2 * nkf * dxvar1_dvbe_eff / xvar1;
            let dxvar2_dvbc_eff = xvar2 * nkf * dxvar1_dvbc_eff / xvar1;
            (
                (0.5 * q1 * (1.0 + xvar2)).max(1e-12),
                0.5 * (1.0 + xvar2) * dq1_dvbe_eff + 0.5 * q1 * dxvar2_dvbe_eff,
                0.5 * (1.0 + xvar2) * dq1_dvbc_eff + 0.5 * q1 * dxvar2_dvbc_eff,
            )
        };

        let itzf = ifi / qb;
        let ditzf_dvbe_eff = gfi / qb - ifi * dqb_dvbe_eff / (qb * qb);
        let ditzf_dvbc_eff = -ifi * dqb_dvbc_eff / (qb * qb);

        let itzr = iri / qb;
        let ditzr_dvbe_eff = -iri * dqb_dvbe_eff / (qb * qb);
        let ditzr_dvbc_eff = gri / qb - iri * dqb_dvbc_eff / (qb * qb);

        TransportChargeState {
            q1,
            qb,
            ifi,
            iri,
            gfi,
            gri,
            dq1_dvbe_eff,
            dq1_dvbc_eff,
            itzf,
            itzr,
            dqb_dvbe_eff,
            dqb_dvbc_eff,
            ditzf_dvbe_eff,
            ditzf_dvbc_eff,
            ditzr_dvbe_eff,
            ditzr_dvbc_eff,
        }
    }

    pub(super) fn transport_charge_state(
        &self,
        vbe_eff: Value,
        vbc_eff: Value,
    ) -> TransportChargeState {
        match self.charge_model {
            BjtChargeModel::LegacyGummelPoon => {
                self.legacy_transport_charge_state(vbe_eff, vbc_eff)
            }
            BjtChargeModel::Vbic => self.vbic_transport_charge_state(vbe_eff, vbc_eff),
        }
    }

    pub(super) fn base_collector_current_state(
        &self,
        transport: TransportChargeState,
        vbc_eff: Value,
    ) -> BaseCollectorCurrentState {
        let ibcj = self.diode_current_with_is(self.ibci, vbc_eff, self.nci)
            + self.diode_current_with_is(self.ibcn, vbc_eff, self.ncn);
        let dibcj_dvbc_eff = self.diode_conductance_with_is(self.ibci, vbc_eff, self.nci)
            + self.diode_conductance_with_is(self.ibcn, vbc_eff, self.ncn);

        if self.avc1 <= 0.0 {
            return BaseCollectorCurrentState {
                ibc: ibcj,
                dibc_dvbe_eff: 0.0,
                dibc_dvbc_eff: dibcj_dvbc_eff,
            };
        }

        let vl_arg = self.vjc - vbc_eff;
        let vl_sqrt = (vl_arg * vl_arg + 0.01).sqrt().max(1e-18);
        let vl = 0.5 * (vl_sqrt + vl_arg);
        let dvl_dvbc_eff = 0.5 * (-vl_arg / vl_sqrt - 1.0);

        let power = self.mjc - 1.0;
        let vl_safe = vl.max(1e-18);
        let vl_power = vl_safe.powf(power);
        let d_vl_power_dvbc_eff = power * vl_safe.powf(power - 1.0) * dvl_dvbc_eff;

        let avalanche_arg = -self.avc2.max(0.0) * vl_power;
        let (avalanche_exp, d_avalanche_exp_darg) = Self::limited_exp(avalanche_arg);
        let d_avalanche_arg_dvbc_eff = -self.avc2.max(0.0) * d_vl_power_dvbc_eff;
        let avalf = self.avc1 * vl * avalanche_exp;
        let davalf_dvbc_eff = self.avc1
            * (dvl_dvbc_eff * avalanche_exp + vl * d_avalanche_exp_darg * d_avalanche_arg_dvbc_eff);

        let transport_minus_ibcj = transport.itzf - transport.itzr - ibcj;
        let d_transport_minus_ibcj_dvbe_eff = transport.ditzf_dvbe_eff - transport.ditzr_dvbe_eff;
        let d_transport_minus_ibcj_dvbc_eff =
            transport.ditzf_dvbc_eff - transport.ditzr_dvbc_eff - dibcj_dvbc_eff;

        let igc = transport_minus_ibcj * avalf;
        let digc_dvbe_eff = d_transport_minus_ibcj_dvbe_eff * avalf;
        let digc_dvbc_eff =
            d_transport_minus_ibcj_dvbc_eff * avalf + transport_minus_ibcj * davalf_dvbc_eff;

        BaseCollectorCurrentState {
            ibc: ibcj - igc,
            dibc_dvbe_eff: -digc_dvbe_eff,
            dibc_dvbc_eff: dibcj_dvbc_eff - digc_dvbc_eff,
        }
    }

    pub(super) fn linearize_currents_with_branches(
        &self,
        vbe: Value,
        vbc: Value,
    ) -> (BjtLinearization, BjtIntrinsicBranches) {
        let p = self.polarity();
        let vbe_eff = p * vbe;
        let vbc_eff = p * vbc;
        let transport = self.transport_charge_state(vbe_eff, vbc_eff);
        let bc = self.base_collector_current_state(transport, vbc_eff);

        let ib_be = self.diode_current_with_is(self.ibei, vbe_eff, self.nei)
            + self.diode_current_with_is(self.iben, vbe_eff, self.nen);
        let dibe_dvbe = self.gbe(vbe);
        let iciei = transport.itzf - transport.itzr;
        let diciei_dvbe = transport.ditzf_dvbe_eff - transport.ditzr_dvbe_eff;
        let diciei_dvbc = transport.ditzf_dvbc_eff - transport.ditzr_dvbc_eff;
        let ibe_branch = Self::branch_from_vbe_vbc(p * ib_be, dibe_dvbe, 0.0);
        let ibc_branch = Self::branch_from_vbe_vbc(p * bc.ibc, bc.dibc_dvbe_eff, bc.dibc_dvbc_eff);
        let iciei_branch = Self::branch_from_vbe_vbc(p * iciei, diciei_dvbe, diciei_dvbc);
        let linearized = BjtLinearization {
            // The intrinsic collector terminal sees both the transport branch
            // (collector to emitter) and the opposing B-C junction branch.
            ic: p * (iciei - bc.ibc),
            ib: p * (ib_be + bc.ibc),
            dic_dvbe: diciei_dvbe - bc.dibc_dvbe_eff,
            dic_dvbc: diciei_dvbc - bc.dibc_dvbc_eff,
            dic_dvrth: 0.0,
            dib_dvbe: dibe_dvbe + bc.dibc_dvbe_eff,
            dib_dvbc: bc.dibc_dvbc_eff,
            dib_dvrth: 0.0,
            qb: transport.qb,
            dqb_dvbe: p * transport.dqb_dvbe_eff,
            dqb_dvbc: p * transport.dqb_dvbc_eff,
            dqb_dvrth: 0.0,
        };

        (
            linearized,
            BjtIntrinsicBranches {
                ibe: ibe_branch,
                ibc: ibc_branch,
                iciei: iciei_branch,
            },
        )
    }

    pub(super) fn linearize_currents(&self, vbe: Value, vbc: Value) -> BjtLinearization {
        self.linearize_currents_with_branches(vbe, vbc).0
    }

    #[inline]
    pub(super) fn collector_row_coefficients(
        &self,
        linearized: BjtLinearization,
    ) -> BjtRowCoefficients {
        [
            -linearized.dic_dvbc,
            linearized.dic_dvbe + linearized.dic_dvbc,
            -linearized.dic_dvbe,
            0.0,
        ]
    }

    #[inline]
    pub(super) fn base_row_coefficients(&self, linearized: BjtLinearization) -> BjtRowCoefficients {
        [
            -linearized.dib_dvbc,
            linearized.dib_dvbe + linearized.dib_dvbc,
            -linearized.dib_dvbe,
            0.0,
        ]
    }

    #[inline]
    pub(super) fn emitter_row_coefficients(
        &self,
        linearized: BjtLinearization,
    ) -> BjtRowCoefficients {
        let collector = self.collector_row_coefficients(linearized);
        let base = self.base_row_coefficients(linearized);
        let mut emitter = [0.0; EXTERNAL_DIM];
        for idx in 0..EXTERNAL_DIM {
            emitter[idx] = -(collector[idx] + base[idx]);
        }
        emitter
    }

    #[inline]
    pub(super) fn series_active(resistance: Value) -> bool {
        resistance.is_finite() && resistance > 0.0
    }

    #[inline]
    pub(super) fn limited_exp(arg: Value) -> (Value, Value) {
        let clamped = arg.clamp(-80.0, 80.0);
        let value = clamped.exp();
        let slope = if (arg - clamped).abs() < f64::EPSILON {
            value
        } else {
            0.0
        };
        (value, slope)
    }

    pub(super) fn intrinsic_terminal_derivatives(
        &self,
        linearized: BjtLinearization,
    ) -> (
        [Value; INTERNAL_DIM],
        [Value; INTERNAL_DIM],
        [Value; INTERNAL_DIM],
    ) {
        let mut collector = [0.0; INTERNAL_DIM];
        collector[IDX_VCI] = -linearized.dic_dvbc;
        collector[IDX_VBI] = linearized.dic_dvbe + linearized.dic_dvbc;
        collector[IDX_VEI] = -linearized.dic_dvbe;
        collector[IDX_VRTH] = linearized.dic_dvrth;

        let mut base = [0.0; INTERNAL_DIM];
        base[IDX_VCI] = -linearized.dib_dvbc;
        base[IDX_VBI] = linearized.dib_dvbe + linearized.dib_dvbc;
        base[IDX_VEI] = -linearized.dib_dvbe;
        base[IDX_VRTH] = linearized.dib_dvrth;

        let mut emitter = [0.0; INTERNAL_DIM];
        for idx in 0..INTERNAL_DIM {
            emitter[idx] = -(collector[idx] + base[idx]);
        }

        (collector, base, emitter)
    }

    pub(super) fn ircx_branch(&self, vc: Value, vcx: Value) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.rcx) {
            return branch;
        }

        let g = 1.0 / self.rcx.max(1e-12);
        branch.current = g * (vc - vcx);
        branch.d_internal[IDX_VCX] = -g;
        branch.d_external[0] = g;
        branch
    }

    pub(super) fn irbx_branch(&self, vb: Value, vbx: Value) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.rbx) {
            return branch;
        }

        let g = 1.0 / self.rbx.max(1e-12);
        branch.current = g * (vb - vbx);
        branch.d_internal[IDX_VBX] = -g;
        branch.d_external[1] = g;
        branch
    }

    pub(super) fn ire_branch(&self, ve: Value, vei: Value) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.re) {
            return branch;
        }

        let g = 1.0 / self.re.max(1e-12);
        branch.current = g * (ve - vei);
        branch.d_internal[IDX_VEI] = -g;
        branch.d_external[2] = g;
        branch
    }

    pub(super) fn irbi_branch(
        &self,
        linearized: BjtLinearization,
        vbx: Value,
        vbi: Value,
    ) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.rbi) {
            return branch;
        }

        let rb = self.rbi.max(1e-12);
        let vrbi = vbx - vbi;
        let qb = linearized.qb.max(1e-12);
        let scale = vrbi / rb;
        let dqb_dvbi = linearized.dqb_dvbe + linearized.dqb_dvbc;
        let dqb_dvci = -linearized.dqb_dvbc;
        let dqb_dvei = -linearized.dqb_dvbe;

        branch.current = scale * qb;
        branch.d_internal[IDX_VBX] = qb / rb;
        branch.d_internal[IDX_VBI] = -qb / rb + scale * dqb_dvbi;
        branch.d_internal[IDX_VCI] = scale * dqb_dvci;
        branch.d_internal[IDX_VEI] = scale * dqb_dvei;
        branch
    }

    pub(super) fn ibep_branch(&self, vbx: Value, vbp: Value) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if self.ibeip <= 0.0 && self.ibenp <= 0.0 {
            return branch;
        }

        let p = self.polarity();
        let vbep_eff = p * (vbx - vbp);
        let ibeip = self.diode_current_with_is(self.ibeip, vbep_eff, self.nci.max(1e-12));
        let ibenp = self.diode_current_with_is(self.ibenp, vbep_eff, self.ncn.max(1e-12));
        let gbep = self.diode_conductance_with_is(self.ibeip, vbep_eff, self.nci.max(1e-12))
            + self.diode_conductance_with_is(self.ibenp, vbep_eff, self.ncn.max(1e-12));

        branch.current = p * (ibeip + ibenp);
        branch.d_internal[IDX_VBX] = gbep;
        branch.d_internal[IDX_VBP] = -gbep;
        branch
    }

    pub(super) fn parasitic_transport_state(
        &self,
        vbx: Value,
        vbi: Value,
        vci: Value,
        vbp: Value,
        vsi: Value,
    ) -> ParasiticTransportState {
        let mut state = ParasiticTransportState {
            qbp: 1.0,
            d_qbp: [0.0; INTERNAL_DIM],
            ifp: 0.0,
            d_ifp: [0.0; INTERNAL_DIM],
            irp: 0.0,
            d_irp: [0.0; INTERNAL_DIM],
        };

        if self.isp <= 0.0 {
            return state;
        }

        let p = self.polarity();
        let nfp_vt = (self.nfp.max(1e-12) * self.vt.max(1e-12)).max(1e-18);
        let vbep_eff = p * (vbx - vbp);
        let vbci_eff = p * (vbi - vci);
        let vbcp_eff = p * (vsi - vbp);

        let (exp_bep, dexp_bep_darg) = Self::limited_exp(vbep_eff / nfp_vt);
        let (exp_bci, dexp_bci_darg) = Self::limited_exp(vbci_eff / nfp_vt);
        let d_ifp_d_vbep_eff = self.isp * self.wsp * dexp_bep_darg / nfp_vt;
        let d_ifp_d_vbci_eff = self.isp * (1.0 - self.wsp) * dexp_bci_darg / nfp_vt;
        state.ifp = self.isp * (self.wsp * exp_bep + (1.0 - self.wsp) * exp_bci - 1.0);
        state.d_ifp[IDX_VBX] = d_ifp_d_vbep_eff * p;
        state.d_ifp[IDX_VBP] = -d_ifp_d_vbep_eff * p;
        state.d_ifp[IDX_VBI] = d_ifp_d_vbci_eff * p;
        state.d_ifp[IDX_VCI] = -d_ifp_d_vbci_eff * p;

        let iikp = if self.ikp.is_finite() && self.ikp > 0.0 {
            1.0 / self.ikp
        } else {
            0.0
        };
        let sqrt_term = (1.0 + 4.0 * state.ifp * iikp).max(1e-18).sqrt();
        state.qbp = (0.5 * (1.0 + sqrt_term)).max(1e-12);
        if iikp > 0.0 {
            let d_qbp_d_ifp = iikp / sqrt_term;
            for idx in 0..INTERNAL_DIM {
                state.d_qbp[idx] = d_qbp_d_ifp * state.d_ifp[idx];
            }
        }

        let (exp_bcp, dexp_bcp_darg) = Self::limited_exp(vbcp_eff / nfp_vt);
        let d_irp_d_vbcp_eff = self.isp * dexp_bcp_darg / nfp_vt;
        state.irp = self.isp * (exp_bcp - 1.0);
        state.d_irp[IDX_VSI] = d_irp_d_vbcp_eff * p;
        state.d_irp[IDX_VBP] = -d_irp_d_vbcp_eff * p;

        state
    }

    pub(super) fn irbp_branch(
        &self,
        vbx: Value,
        vbi: Value,
        vcx: Value,
        vci: Value,
        vbp: Value,
        vsi: Value,
    ) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.rbp) {
            return branch;
        }

        let parasitic = self.parasitic_transport_state(vbx, vbi, vci, vbp, vsi);
        let rbp = self.rbp.max(1e-12);
        let vrbp = vbp - vcx;
        let scale = vrbp / rbp;

        branch.current = scale * parasitic.qbp;
        branch.d_internal[IDX_VCX] = -parasitic.qbp / rbp;
        branch.d_internal[IDX_VBP] = parasitic.qbp / rbp + scale * parasitic.d_qbp[IDX_VBP];
        branch.d_internal[IDX_VBX] = scale * parasitic.d_qbp[IDX_VBX];
        branch.d_internal[IDX_VBI] = scale * parasitic.d_qbp[IDX_VBI];
        branch.d_internal[IDX_VCI] = scale * parasitic.d_qbp[IDX_VCI];
        branch.d_internal[IDX_VSI] = scale * parasitic.d_qbp[IDX_VSI];
        branch
    }

    pub(super) fn ibcp_branch(&self, vbp: Value, vsi: Value) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if self.ibcip <= 0.0 && self.ibcnp <= 0.0 {
            return branch;
        }

        let p = self.polarity();
        let vbcp_eff = p * (vsi - vbp);
        let ibcip = self.diode_current_with_is(self.ibcip, vbcp_eff, self.ncip.max(1e-12));
        let ibcnp = self.diode_current_with_is(self.ibcnp, vbcp_eff, self.ncnp.max(1e-12));
        let gbcp = self.diode_conductance_with_is(self.ibcip, vbcp_eff, self.ncip.max(1e-12))
            + self.diode_conductance_with_is(self.ibcnp, vbcp_eff, self.ncnp.max(1e-12));

        branch.current = p * (ibcip + ibcnp);
        branch.d_internal[IDX_VSI] = gbcp;
        branch.d_internal[IDX_VBP] = -gbcp;
        branch
    }

    pub(super) fn iccp_branch(
        &self,
        vbx: Value,
        vbi: Value,
        vci: Value,
        vbp: Value,
        vsi: Value,
    ) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if self.isp <= 0.0 {
            return branch;
        }

        let parasitic = self.parasitic_transport_state(vbx, vbi, vci, vbp, vsi);
        let p = self.polarity();
        let inv_qbp = 1.0 / parasitic.qbp.max(1e-12);
        let delta = parasitic.ifp - parasitic.irp;

        branch.current = p * delta * inv_qbp;
        for idx in 0..INTERNAL_DIM {
            branch.d_internal[idx] = p
                * ((parasitic.d_ifp[idx] - parasitic.d_irp[idx]) * inv_qbp
                    - delta * parasitic.d_qbp[idx] * inv_qbp * inv_qbp);
        }
        branch
    }

    pub(super) fn irs_branch(&self, vs: Value, vsi: Value) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.rs) {
            return branch;
        }

        let g = 1.0 / self.rs.max(1e-12);
        branch.current = g * (vs - vsi);
        branch.d_internal[IDX_VSI] = -g;
        branch.d_external[EXT_S] = g;
        branch
    }

    pub(super) fn irci_branch(&self, vcx: Value, vci: Value, vbi: Value) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.rci) {
            return branch;
        }

        let p = self.polarity();
        let vt = self.vt.max(1e-12);
        let rci = self.rci.max(1e-12);
        let gamm = self.gamm.max(0.0);
        let ivo = if self.vo.is_finite() && self.vo > 0.0 {
            1.0 / self.vo
        } else {
            0.0
        };
        let ihrcf = if self.hrcf.is_finite() && self.hrcf > 0.0 {
            1.0 / self.hrcf
        } else {
            0.0
        };

        let vrci_eff = p * (vcx - vci);
        let vbci_eff = p * (vbi - vci);
        let vbcx_eff = p * (vbi - vcx);

        let (exp_bci, dexp_bci_darg) = Self::limited_exp(vbci_eff / vt);
        let (exp_bcx, dexp_bcx_darg) = Self::limited_exp(vbcx_eff / vt);
        let d_exp_bci_dvbci_eff = dexp_bci_darg / vt;
        let d_exp_bcx_dvbcx_eff = dexp_bcx_darg / vt;

        let kbci = (1.0 + gamm * exp_bci).sqrt().max(1e-12);
        let kbcx = (1.0 + gamm * exp_bcx).sqrt().max(1e-12);
        let d_kbci_dvbci_eff = if gamm > 0.0 {
            gamm * d_exp_bci_dvbci_eff / (2.0 * kbci)
        } else {
            0.0
        };
        let d_kbcx_dvbcx_eff = if gamm > 0.0 {
            gamm * d_exp_bcx_dvbcx_eff / (2.0 * kbcx)
        } else {
            0.0
        };

        let ratio = ((kbci + 1.0) / (kbcx + 1.0)).max(1e-18);
        let log_ratio = ratio.ln();
        let d_ratio_dkbci = 1.0 / (kbcx + 1.0);
        let d_ratio_dkbcx = -(kbci + 1.0) / (kbcx + 1.0).powi(2);
        let d_log_ratio_dkbci = d_ratio_dkbci / ratio;
        let d_log_ratio_dkbcx = d_ratio_dkbcx / ratio;

        let iohm = (vrci_eff + vt * (kbci - kbcx - log_ratio)) / rci;
        let d_iohm_dvrci_eff = 1.0 / rci;
        let d_iohm_dvbci_eff = vt * d_kbci_dvbci_eff * (1.0 - d_log_ratio_dkbci) / rci;
        let d_iohm_dvbcx_eff = vt * d_kbcx_dvbcx_eff * (-1.0 - d_log_ratio_dkbcx) / rci;

        let sqrt_vrci = (vrci_eff * vrci_eff + 0.01).sqrt();
        let denom = 1.0 + 0.5 * ivo * ihrcf * sqrt_vrci;
        let d_denom_dvrci_eff = if ivo > 0.0 && ihrcf > 0.0 {
            0.5 * ivo * ihrcf * vrci_eff / sqrt_vrci
        } else {
            0.0
        };

        let derf_scale = ivo * rci;
        let derf = if derf_scale > 0.0 {
            derf_scale * iohm / denom
        } else {
            0.0
        };
        let d_derf_dvrci_eff = if derf_scale > 0.0 {
            derf_scale * (d_iohm_dvrci_eff / denom - iohm * d_denom_dvrci_eff / denom.powi(2))
        } else {
            0.0
        };
        let d_derf_dvbci_eff = if derf_scale > 0.0 {
            derf_scale * d_iohm_dvbci_eff / denom
        } else {
            0.0
        };
        let d_derf_dvbcx_eff = if derf_scale > 0.0 {
            derf_scale * d_iohm_dvbcx_eff / denom
        } else {
            0.0
        };

        let irci_scale = (1.0 + derf * derf).sqrt();
        let inv_irci_scale = 1.0 / irci_scale;
        let common = -iohm * derf / (irci_scale * irci_scale * irci_scale);
        let d_irci_eff_dvrci_eff = d_iohm_dvrci_eff * inv_irci_scale + common * d_derf_dvrci_eff;
        let d_irci_eff_dvbci_eff = d_iohm_dvbci_eff * inv_irci_scale + common * d_derf_dvbci_eff;
        let d_irci_eff_dvbcx_eff = d_iohm_dvbcx_eff * inv_irci_scale + common * d_derf_dvbcx_eff;
        let irci_eff = iohm * inv_irci_scale;

        branch.current = p * irci_eff;
        branch.d_internal[IDX_VCX] = d_irci_eff_dvrci_eff - d_irci_eff_dvbcx_eff;
        branch.d_internal[IDX_VCI] = -(d_irci_eff_dvrci_eff + d_irci_eff_dvbci_eff);
        branch.d_internal[IDX_VBI] = d_irci_eff_dvbci_eff + d_irci_eff_dvbcx_eff;
        branch
    }

    #[inline]
    pub(super) fn thermal_derivative_step(&self, vrth: Value) -> Value {
        // Use a small relative perturbation to keep Vrth-derivative finite
        // differences accurate for strongly temperature-sensitive currents.
        ((self.requested_temperature() + vrth).abs().max(1.0) * 1e-6).clamp(1e-7, 1e-3)
    }

    #[inline]
    pub(super) fn limit_logarithmic_step(vnew: Value, vold: Value, limit: Value) -> Value {
        let limit = limit.max(1e-18);
        if !vnew.is_finite() {
            return vold;
        }
        if !vold.is_finite() {
            return vnew;
        }

        if vnew > vold + limit {
            vold + limit + ((vnew - vold) / limit).log10()
        } else if vnew < vold - limit {
            vold - limit - ((vold - vnew) / limit).log10()
        } else {
            vnew
        }
    }

    #[inline]
    pub(super) fn junction_critical_voltage(vt: Value, isat: Value) -> Value {
        let vt = vt.max(1e-18);
        let isat = isat.abs().max(1e-18);
        vt * (vt / (core::f64::consts::SQRT_2 * isat)).ln()
    }

    #[inline]
    pub(super) fn vbic_limiting_parameters(&self, previous_vrth: Value) -> (Value, Value) {
        self.with_temperature_variant(previous_vrth, |model| {
            let vt = model.vt.max(1e-18);
            let vcrit = Self::junction_critical_voltage(vt, model.is);
            (vt, vcrit)
        })
    }

    #[inline]
    pub(super) fn vbic_nonlinear_branch_voltages(
        &self,
        internal: [Value; INTERNAL_DIM],
    ) -> VbicNonlinearBranchVoltages {
        let p = self.polarity();
        VbicNonlinearBranchVoltages {
            vbei: p * (internal[IDX_VBI] - internal[IDX_VEI]),
            vbex: p * (internal[IDX_VBX] - internal[IDX_VEI]),
            vbci: p * (internal[IDX_VBI] - internal[IDX_VCI]),
            vbcx: p * (internal[IDX_VBI] - internal[IDX_VCX]),
            vbep: p * (internal[IDX_VBX] - internal[IDX_VBP]),
            vbcp: p * (internal[IDX_VSI] - internal[IDX_VBP]),
            vrth: internal[IDX_VRTH],
        }
    }

    #[inline]
    pub(super) fn legacy_limiting_parameters(&self, previous_vrth: Value) -> (Value, Value, Value) {
        self.with_temperature_variant(previous_vrth, |model| {
            let vt = model.vt.max(1e-18);
            let vcrit = Self::junction_critical_voltage(vt, model.is);
            (vt, vcrit, 50.0)
        })
    }

    #[inline]
    pub(super) fn legacy_nonlinear_branch_voltages(
        &self,
        internal: [Value; INTERNAL_DIM],
    ) -> LegacyNonlinearBranchVoltages {
        let p = self.polarity();
        LegacyNonlinearBranchVoltages {
            vbe: p * (internal[IDX_VBI] - internal[IDX_VEI]),
            vbc: p * (internal[IDX_VBI] - internal[IDX_VCI]),
            vsub: p * (internal[IDX_VSI] - internal[IDX_VCI]),
        }
    }

    pub(super) fn project_vbic_limited_branches_onto_internal_state(
        &self,
        raw: [Value; INTERNAL_DIM],
        limited: VbicNonlinearBranchVoltages,
    ) -> [Value; INTERNAL_DIM] {
        let p = self.polarity();
        let raw_nodes = [
            raw[IDX_VCX],
            raw[IDX_VCI],
            raw[IDX_VBX],
            raw[IDX_VBI],
            raw[IDX_VEI],
            raw[IDX_VBP],
            raw[IDX_VSI],
        ];
        let constraints = [
            [0.0, 0.0, 0.0, p, -p, 0.0, 0.0],
            [0.0, 0.0, p, 0.0, -p, 0.0, 0.0],
            [0.0, -p, 0.0, p, 0.0, 0.0, 0.0],
            [-p, 0.0, 0.0, p, 0.0, 0.0, 0.0],
            [0.0, 0.0, p, 0.0, 0.0, -p, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, -p, p],
        ];
        let targets = [
            limited.vbei,
            limited.vbex,
            limited.vbci,
            limited.vbcx,
            limited.vbep,
            limited.vbcp,
        ];

        let mut residual = [0.0; VBIC_LIMITED_BRANCH_DIM];
        for row in 0..VBIC_LIMITED_BRANCH_DIM {
            residual[row] = -targets[row];
            for col in 0..raw_nodes.len() {
                residual[row] += constraints[row][col] * raw_nodes[col];
            }
        }

        let mut gram = [[0.0; VBIC_LIMITED_BRANCH_DIM]; VBIC_LIMITED_BRANCH_DIM];
        for row in 0..VBIC_LIMITED_BRANCH_DIM {
            for col in 0..VBIC_LIMITED_BRANCH_DIM {
                gram[row][col] = (0..raw_nodes.len())
                    .map(|idx| constraints[row][idx] * constraints[col][idx])
                    .sum();
            }
        }

        let Some(lagrange) =
            Self::solve_small_dense_system(&gram, &residual, VBIC_LIMITED_BRANCH_DIM)
        else {
            let mut fallback = raw;
            fallback[IDX_VRTH] = limited.vrth;
            return fallback;
        };

        let mut projected = raw;
        for node_idx in 0..raw_nodes.len() {
            let correction = (0..VBIC_LIMITED_BRANCH_DIM)
                .map(|row| constraints[row][node_idx] * lagrange[row])
                .sum::<Value>();
            projected[node_idx] = raw_nodes[node_idx] - correction;
        }
        projected[IDX_VRTH] = limited.vrth;
        projected
    }

    pub(super) fn project_legacy_limited_branches_onto_internal_state(
        &self,
        raw: [Value; INTERNAL_DIM],
        limited: LegacyNonlinearBranchVoltages,
    ) -> [Value; INTERNAL_DIM] {
        let p = self.polarity();
        let raw_nodes = [
            raw[IDX_VCX],
            raw[IDX_VCI],
            raw[IDX_VBX],
            raw[IDX_VBI],
            raw[IDX_VEI],
            raw[IDX_VBP],
            raw[IDX_VSI],
        ];
        let constraints = [
            [0.0, 0.0, 0.0, p, -p, 0.0, 0.0],
            [0.0, -p, 0.0, p, 0.0, 0.0, 0.0],
            [0.0, -p, 0.0, 0.0, 0.0, 0.0, p],
        ];
        let targets = [limited.vbe, limited.vbc, limited.vsub];

        let mut residual = [0.0; LEGACY_LIMITED_BRANCH_DIM];
        for row in 0..LEGACY_LIMITED_BRANCH_DIM {
            residual[row] = -targets[row];
            for col in 0..raw_nodes.len() {
                residual[row] += constraints[row][col] * raw_nodes[col];
            }
        }

        let mut gram = [[0.0; LEGACY_LIMITED_BRANCH_DIM]; LEGACY_LIMITED_BRANCH_DIM];
        for row in 0..LEGACY_LIMITED_BRANCH_DIM {
            for col in 0..LEGACY_LIMITED_BRANCH_DIM {
                gram[row][col] = (0..raw_nodes.len())
                    .map(|idx| constraints[row][idx] * constraints[col][idx])
                    .sum();
            }
        }

        let Some(lagrange) =
            Self::solve_small_dense_system(&gram, &residual, LEGACY_LIMITED_BRANCH_DIM)
        else {
            return raw;
        };

        let mut projected = raw;
        for node_idx in 0..raw_nodes.len() {
            let correction = (0..LEGACY_LIMITED_BRANCH_DIM)
                .map(|row| constraints[row][node_idx] * lagrange[row])
                .sum::<Value>();
            projected[node_idx] = raw_nodes[node_idx] - correction;
        }
        projected
    }

    pub(super) fn limit_vbic_internal_state_to_previous(
        &self,
        raw: [Value; INTERNAL_DIM],
        previous: [Value; INTERNAL_DIM],
    ) -> [Value; INTERNAL_DIM] {
        if self.charge_model != BjtChargeModel::Vbic {
            return raw;
        }

        let raw_branches = self.vbic_nonlinear_branch_voltages(raw);
        let previous_branches = self.vbic_nonlinear_branch_voltages(previous);
        let (vt, vcrit) = self.vbic_limiting_parameters(previous[IDX_VRTH]);
        let limited_branches = VbicNonlinearBranchVoltages {
            vbei: Self::limit_junction_voltage(
                raw_branches.vbei,
                previous_branches.vbei,
                vt,
                vcrit,
            ),
            vbex: Self::limit_junction_voltage(
                raw_branches.vbex,
                previous_branches.vbex,
                vt,
                vcrit,
            ),
            vbci: Self::limit_junction_voltage(
                raw_branches.vbci,
                previous_branches.vbci,
                vt,
                vcrit,
            ),
            vbcx: Self::limit_junction_voltage(
                raw_branches.vbcx,
                previous_branches.vbcx,
                vt,
                vcrit,
            ),
            vbep: Self::limit_junction_voltage(
                raw_branches.vbep,
                previous_branches.vbep,
                vt,
                vcrit,
            ),
            vbcp: Self::limit_junction_voltage(
                raw_branches.vbcp,
                previous_branches.vbcp,
                vt,
                vcrit,
            ),
            vrth: if self.self_heating_enabled() {
                Self::limit_logarithmic_step(raw_branches.vrth, previous_branches.vrth, 100.0)
                    .max(self.minimum_thermal_rise())
            } else {
                0.0
            },
        };

        let projected =
            self.project_vbic_limited_branches_onto_internal_state(raw, limited_branches);
        if projected.iter().all(|value| value.is_finite()) {
            projected
        } else {
            raw
        }
    }

    pub(super) fn limit_legacy_internal_state_to_previous(
        &self,
        raw: [Value; INTERNAL_DIM],
        previous: [Value; INTERNAL_DIM],
    ) -> [Value; INTERNAL_DIM] {
        if self.charge_model != BjtChargeModel::LegacyGummelPoon {
            return raw;
        }

        let raw_branches = self.legacy_nonlinear_branch_voltages(raw);
        let previous_branches = self.legacy_nonlinear_branch_voltages(previous);
        let (vt, vcrit, sub_vcrit) = self.legacy_limiting_parameters(previous[IDX_VRTH]);
        let limited_branches = LegacyNonlinearBranchVoltages {
            vbe: Self::limit_junction_voltage(raw_branches.vbe, previous_branches.vbe, vt, vcrit),
            vbc: Self::limit_junction_voltage(raw_branches.vbc, previous_branches.vbc, vt, vcrit),
            vsub: Self::limit_junction_voltage(
                raw_branches.vsub,
                previous_branches.vsub,
                vt,
                sub_vcrit,
            ),
        };

        let projected =
            self.project_legacy_limited_branches_onto_internal_state(raw, limited_branches);
        if projected.iter().all(|value| value.is_finite()) {
            projected
        } else {
            raw
        }
    }

    pub(crate) fn limit_vbic_dynamic_internal_state_to_previous(
        &self,
        raw: [Value; BJT_INTERNAL_STATE_DIM],
        previous: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> [Value; BJT_INTERNAL_STATE_DIM] {
        if self.charge_model != BjtChargeModel::Vbic {
            return raw;
        }

        let mut raw_static = [0.0; INTERNAL_DIM];
        raw_static.copy_from_slice(&raw[..INTERNAL_DIM]);
        let mut previous_static = [0.0; INTERNAL_DIM];
        previous_static.copy_from_slice(&previous[..INTERNAL_DIM]);

        let mut limited = raw;
        limited[..INTERNAL_DIM].copy_from_slice(
            &self.limit_vbic_internal_state_to_previous(raw_static, previous_static),
        );
        limited
    }

    #[inline]
    pub(crate) fn predict_vbic_dynamic_internal_state_from_previous_external_bias(
        &self,
        previous_external: [Value; EXTERNAL_DIM],
        previous_dynamic: [Value; BJT_INTERNAL_STATE_DIM],
        proposed_external: [Value; EXTERNAL_DIM],
    ) -> Option<[Value; BJT_INTERNAL_STATE_DIM]> {
        if self.charge_model != BjtChargeModel::Vbic {
            return None;
        }

        let mut previous_static = [0.0; INTERNAL_DIM];
        previous_static.copy_from_slice(&previous_dynamic[..INTERNAL_DIM]);
        let predicted_static = self.predict_intrinsic_state_from_previous_external_bias(
            previous_external,
            previous_static,
            proposed_external,
        )?;

        let mut predicted_dynamic = previous_dynamic;
        predicted_dynamic[..INTERNAL_DIM].copy_from_slice(&predicted_static);
        Some(
            self.limit_vbic_dynamic_internal_state_to_previous(predicted_dynamic, previous_dynamic),
        )
    }

    #[inline]
    pub(crate) fn vbic_dynamic_internal_state_within_local_branch_envelope(
        &self,
        state: [Value; BJT_INTERNAL_STATE_DIM],
        reference: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> bool {
        let mut state_static = [0.0; INTERNAL_DIM];
        state_static.copy_from_slice(&state[..INTERNAL_DIM]);
        let mut reference_static = [0.0; INTERNAL_DIM];
        reference_static.copy_from_slice(&reference[..INTERNAL_DIM]);
        self.vbic_internal_state_within_local_branch_envelope(state_static, reference_static)
    }

    #[inline]
    pub(super) fn limit_intrinsic_state_against_previous(
        &self,
        raw: [Value; INTERNAL_DIM],
        previous: [Value; INTERNAL_DIM],
    ) -> [Value; INTERNAL_DIM] {
        let mut limited = if self.charge_model == BjtChargeModel::Vbic {
            self.limit_vbic_internal_state_to_previous(raw, previous)
        } else {
            self.limit_legacy_internal_state_to_previous(raw, previous)
        };

        if self.charge_model != BjtChargeModel::Vbic && self.self_heating_enabled() {
            limited[IDX_VRTH] =
                Self::limit_logarithmic_step(raw[IDX_VRTH], previous[IDX_VRTH], 100.0)
                    .max(1.0 - self.requested_temperature());
        }

        limited
    }

    pub(super) fn predict_intrinsic_state_from_previous_external_bias_unlimited(
        &self,
        previous_external: [Value; EXTERNAL_DIM],
        previous_internal: [Value; INTERNAL_DIM],
        proposed_external: [Value; EXTERNAL_DIM],
    ) -> Option<[Value; INTERNAL_DIM]> {
        let previous_state = self.intrinsic_state_from_internal_vector(previous_internal);
        let sensitivities = self.internal_voltage_sensitivities(
            previous_state,
            previous_external[EXT_C],
            previous_external[EXT_B],
            previous_external[EXT_E],
            previous_external[EXT_S],
        );
        let delta_external = [
            proposed_external[EXT_C] - previous_external[EXT_C],
            proposed_external[EXT_B] - previous_external[EXT_B],
            proposed_external[EXT_E] - previous_external[EXT_E],
            proposed_external[EXT_S] - previous_external[EXT_S],
        ];

        let mut predicted = previous_internal;
        for internal_idx in 0..INTERNAL_DIM {
            predicted[internal_idx] += sensitivities[internal_idx]
                .iter()
                .zip(delta_external.iter())
                .map(|(sensitivity, delta)| sensitivity * delta)
                .sum::<Value>();
        }

        predicted
            .iter()
            .all(|value| value.is_finite())
            .then_some(predicted)
    }

    pub(super) fn predict_intrinsic_state_from_previous_external_bias(
        &self,
        previous_external: [Value; EXTERNAL_DIM],
        previous_internal: [Value; INTERNAL_DIM],
        proposed_external: [Value; EXTERNAL_DIM],
    ) -> Option<[Value; INTERNAL_DIM]> {
        let predicted = self.predict_intrinsic_state_from_previous_external_bias_unlimited(
            previous_external,
            previous_internal,
            proposed_external,
        )?;
        Some(self.limit_intrinsic_state_against_previous(predicted, previous_internal))
    }

    #[inline]
    pub(super) fn vbic_internal_state_within_local_branch_envelope(
        &self,
        state: [Value; INTERNAL_DIM],
        reference: [Value; INTERNAL_DIM],
    ) -> bool {
        if self.charge_model != BjtChargeModel::Vbic {
            return true;
        }

        let state_branches = self.vbic_nonlinear_branch_voltages(state);
        let reference_branches = self.vbic_nonlinear_branch_voltages(reference);
        let (vt, vcrit) = self.vbic_limiting_parameters(reference[IDX_VRTH]);
        let expected = VbicNonlinearBranchVoltages {
            vbei: Self::limit_junction_voltage(
                state_branches.vbei,
                reference_branches.vbei,
                vt,
                vcrit,
            ),
            vbex: Self::limit_junction_voltage(
                state_branches.vbex,
                reference_branches.vbex,
                vt,
                vcrit,
            ),
            vbci: Self::limit_junction_voltage(
                state_branches.vbci,
                reference_branches.vbci,
                vt,
                vcrit,
            ),
            vbcx: Self::limit_junction_voltage(
                state_branches.vbcx,
                reference_branches.vbcx,
                vt,
                vcrit,
            ),
            vbep: Self::limit_junction_voltage(
                state_branches.vbep,
                reference_branches.vbep,
                vt,
                vcrit,
            ),
            vbcp: Self::limit_junction_voltage(
                state_branches.vbcp,
                reference_branches.vbcp,
                vt,
                vcrit,
            ),
            vrth: if self.self_heating_enabled() {
                Self::limit_logarithmic_step(state_branches.vrth, reference_branches.vrth, 100.0)
                    .max(self.minimum_thermal_rise())
            } else {
                0.0
            },
        };

        [
            (state_branches.vbei, expected.vbei),
            (state_branches.vbex, expected.vbex),
            (state_branches.vbci, expected.vbci),
            (state_branches.vbcx, expected.vbcx),
            (state_branches.vbep, expected.vbep),
            (state_branches.vbcp, expected.vbcp),
            (state_branches.vrth, expected.vrth),
        ]
        .into_iter()
        .all(|(actual, limited)| (actual - limited).abs() <= 1e-12)
    }

    #[inline]
    pub(super) fn vbic_max_local_branch_delta(
        &self,
        lhs: [Value; INTERNAL_DIM],
        rhs: [Value; INTERNAL_DIM],
    ) -> Value {
        if self.charge_model != BjtChargeModel::Vbic {
            return lhs
                .iter()
                .zip(rhs.iter())
                .map(|(lhs, rhs)| (lhs - rhs).abs())
                .fold(0.0, Value::max);
        }

        let lhs_branches = self.vbic_nonlinear_branch_voltages(lhs);
        let rhs_branches = self.vbic_nonlinear_branch_voltages(rhs);
        [
            (lhs_branches.vbei - rhs_branches.vbei).abs(),
            (lhs_branches.vbex - rhs_branches.vbex).abs(),
            (lhs_branches.vbci - rhs_branches.vbci).abs(),
            (lhs_branches.vbcx - rhs_branches.vbcx).abs(),
            (lhs_branches.vbep - rhs_branches.vbep).abs(),
            (lhs_branches.vbcp - rhs_branches.vbcp).abs(),
            (lhs_branches.vrth - rhs_branches.vrth).abs(),
        ]
        .into_iter()
        .fold(0.0, Value::max)
    }

    pub(super) fn solve_intrinsic_state_with_external_continuation(
        &self,
        previous_external: [Value; EXTERNAL_DIM],
        previous_state: [Value; INTERNAL_DIM],
        target_external: [Value; EXTERNAL_DIM],
    ) -> Option<([Value; INTERNAL_DIM], Value)> {
        let mut current_external = previous_external;
        let mut current_state = previous_state;
        let mut lambda: Value = 0.0;
        let mut step: Value = 1.0;
        let use_linear_prediction = self.charge_model == BjtChargeModel::Vbic;
        let required_residual = if self.charge_model == BjtChargeModel::LegacyGummelPoon {
            1e-6
        } else {
            Value::INFINITY
        };

        while lambda < 1.0 - 1e-15 {
            let candidate_lambda = (lambda + step).min(1.0);
            let next_external = [
                previous_external[EXT_C]
                    + (target_external[EXT_C] - previous_external[EXT_C]) * candidate_lambda,
                previous_external[EXT_B]
                    + (target_external[EXT_B] - previous_external[EXT_B]) * candidate_lambda,
                previous_external[EXT_E]
                    + (target_external[EXT_E] - previous_external[EXT_E]) * candidate_lambda,
                previous_external[EXT_S]
                    + (target_external[EXT_S] - previous_external[EXT_S]) * candidate_lambda,
            ];

            let seed = if use_linear_prediction {
                self.predict_intrinsic_state_from_previous_external_bias(
                    current_external,
                    current_state,
                    next_external,
                )
                .unwrap_or(current_state)
            } else {
                current_state
            };
            let (solved_state, solved_residual) = self.solve_intrinsic_state_from_seed(
                next_external[EXT_C],
                next_external[EXT_B],
                next_external[EXT_E],
                next_external[EXT_S],
                seed,
            );

            if solved_residual.is_finite()
                && solved_residual <= required_residual
                && (!use_linear_prediction
                    || self.vbic_max_local_branch_delta(solved_state, seed) <= 0.1)
            {
                current_external = next_external;
                current_state = solved_state;
                lambda = candidate_lambda;
                step = (step * 2.0).min(1.0 - lambda).max(1e-6);
                continue;
            }

            if step <= 1.0 / 256.0 {
                return None;
            }
            step *= 0.5;
        }

        let residual = Self::intrinsic_state_residual_norm(
            &self
                .intrinsic_state_residual_jacobian(
                    target_external[EXT_C],
                    target_external[EXT_B],
                    target_external[EXT_E],
                    target_external[EXT_S],
                    current_state,
                )
                .0,
        );
        Some((current_state, residual))
    }

    #[inline]
    pub(super) fn has_intrinsic_state_unknowns(&self) -> bool {
        Self::series_active(self.rcx)
            || Self::series_active(self.rci)
            || Self::series_active(self.rbx)
            || Self::series_active(self.rbi)
            || Self::series_active(self.re)
            || Self::series_active(self.rs)
            || Self::series_active(self.rbp)
            || self.ibeip > 0.0
            || self.ibenp > 0.0
            || self.ibcip > 0.0
            || self.ibcnp > 0.0
            || self.self_heating_enabled()
    }

    #[inline]
    pub(super) fn intrinsic_state_seed_for_external_bias(
        &self,
        external: [Value; EXTERNAL_DIM],
    ) -> [Value; INTERNAL_DIM] {
        [
            external[EXT_C],
            external[EXT_C],
            external[EXT_B],
            external[EXT_B],
            external[EXT_E],
            external[EXT_C],
            external[EXT_S],
            if self.self_heating_enabled() {
                self.minimum_thermal_rise()
            } else {
                0.0
            },
        ]
    }

    #[inline]
    pub(super) fn initial_forward_bias_anchor_external(
        &self,
        target_external: [Value; EXTERNAL_DIM],
    ) -> Option<[Value; EXTERNAL_DIM]> {
        let p = self.polarity();
        let max_forward_bias = 0.8;
        let mut anchor = target_external;
        let mut changed = false;

        let vbe = p * (anchor[EXT_B] - anchor[EXT_E]);
        if vbe.is_finite() && vbe > max_forward_bias {
            anchor[EXT_B] = anchor[EXT_E] + p * max_forward_bias;
            changed = true;
        }

        let vbc = p * (anchor[EXT_B] - anchor[EXT_C]);
        if vbc.is_finite() && vbc > max_forward_bias {
            anchor[EXT_B] = anchor[EXT_C] + p * max_forward_bias;
            changed = true;
        }

        changed.then_some(anchor)
    }

    pub(super) fn solve_intrinsic_state_from_forward_bias_anchor(
        &self,
        anchor_external: [Value; EXTERNAL_DIM],
        target_external: [Value; EXTERNAL_DIM],
    ) -> Option<([Value; INTERNAL_DIM], Value)> {
        let anchor_seed = self.intrinsic_state_seed_for_external_bias(anchor_external);
        let (anchor_state, anchor_residual_norm) = self.solve_intrinsic_state_from_seed(
            anchor_external[EXT_C],
            anchor_external[EXT_B],
            anchor_external[EXT_E],
            anchor_external[EXT_S],
            anchor_seed,
        );
        if !anchor_residual_norm.is_finite() {
            return None;
        }

        let mut best = self.solve_intrinsic_state_from_seed(
            target_external[EXT_C],
            target_external[EXT_B],
            target_external[EXT_E],
            target_external[EXT_S],
            anchor_state,
        );
        if self.charge_model == BjtChargeModel::Vbic {
            let projected_target_seed = self
                .predict_intrinsic_state_from_previous_external_bias(
                    anchor_external,
                    anchor_state,
                    target_external,
                )
                .unwrap_or(anchor_state);
            let projected = self.solve_intrinsic_state_from_seed(
                target_external[EXT_C],
                target_external[EXT_B],
                target_external[EXT_E],
                target_external[EXT_S],
                projected_target_seed,
            );
            if projected.1 + 1e-15 < best.1 {
                best = projected;
            }
        }

        if let Some(continued) = self.solve_intrinsic_state_with_external_continuation(
            anchor_external,
            anchor_state,
            target_external,
        ) && continued.1 + 1e-15 < best.1
        {
            best = continued;
        }

        best.1.is_finite().then_some(best)
    }

    #[inline]
    pub(super) fn vbic_cached_external_matches(
        &self,
        external: [Value; EXTERNAL_DIM],
        voltage_abstol: Value,
        reltol: Value,
    ) -> bool {
        let cached = [self.vc_ext, self.vb_ext, self.ve_ext, self.vs_ext];
        cached
            .iter()
            .zip(external.iter())
            .all(|(cached, external)| {
                let diff = (cached - external).abs();
                let tol = reltol * cached.abs().max(external.abs()) + voltage_abstol;
                diff <= tol
            })
    }

    #[inline]
    pub(super) fn vbic_branch_limit_scale(
        previous: Value,
        raw: Value,
        limited: Value,
    ) -> Option<Value> {
        let raw_delta = raw - previous;
        if !raw_delta.is_finite() || raw_delta.abs() <= 1e-18 {
            return None;
        }
        let limited_delta = limited - previous;
        if !limited_delta.is_finite() {
            return Some(0.0);
        }
        Some((limited_delta.abs() / raw_delta.abs()).clamp(0.0, 1.0))
    }

    pub(crate) fn vbic_external_step_limit_scale_from_state(
        &self,
        previous_external: [Value; EXTERNAL_DIM],
        previous_internal: [Value; INTERNAL_DIM],
        proposed_external: [Value; EXTERNAL_DIM],
    ) -> Option<Value> {
        if self.charge_model != BjtChargeModel::Vbic {
            return None;
        }

        let delta_external = [
            proposed_external[EXT_C] - previous_external[EXT_C],
            proposed_external[EXT_B] - previous_external[EXT_B],
            proposed_external[EXT_E] - previous_external[EXT_E],
            proposed_external[EXT_S] - previous_external[EXT_S],
        ];
        let max_delta = delta_external
            .iter()
            .map(|value| value.abs())
            .fold(0.0, Value::max);
        if !max_delta.is_finite() || max_delta <= 1e-15 {
            return None;
        }

        let Some(raw_internal) = self
            .predict_intrinsic_state_from_previous_external_bias_unlimited(
                previous_external,
                previous_internal,
                proposed_external,
            )
        else {
            return Some(0.5);
        };
        if !raw_internal.iter().all(|value| value.is_finite()) {
            return Some(0.5);
        }

        let limited_internal =
            self.limit_intrinsic_state_against_previous(raw_internal, previous_internal);
        let previous_branches = self.vbic_nonlinear_branch_voltages(previous_internal);
        let raw_branches = self.vbic_nonlinear_branch_voltages(raw_internal);
        let limited_branches = self.vbic_nonlinear_branch_voltages(limited_internal);

        let mut scale: Value = 1.0;
        let mut engaged = false;
        for branch_scale in [
            Self::vbic_branch_limit_scale(
                previous_branches.vbei,
                raw_branches.vbei,
                limited_branches.vbei,
            ),
            Self::vbic_branch_limit_scale(
                previous_branches.vbex,
                raw_branches.vbex,
                limited_branches.vbex,
            ),
            Self::vbic_branch_limit_scale(
                previous_branches.vbci,
                raw_branches.vbci,
                limited_branches.vbci,
            ),
            Self::vbic_branch_limit_scale(
                previous_branches.vbcx,
                raw_branches.vbcx,
                limited_branches.vbcx,
            ),
            Self::vbic_branch_limit_scale(
                previous_branches.vbep,
                raw_branches.vbep,
                limited_branches.vbep,
            ),
            Self::vbic_branch_limit_scale(
                previous_branches.vbcp,
                raw_branches.vbcp,
                limited_branches.vbcp,
            ),
            if self.self_heating_enabled() {
                Self::vbic_branch_limit_scale(
                    previous_branches.vrth,
                    raw_branches.vrth,
                    limited_branches.vrth,
                )
            } else {
                None
            },
        ]
        .into_iter()
        .flatten()
        {
            if branch_scale + 1e-15 < 1.0 {
                engaged = true;
            }
            scale = scale.min(branch_scale);
        }

        engaged.then_some(scale.max(0.0))
    }

    pub(crate) fn vbic_external_step_limit_scale_against_previous(
        &self,
        previous_external: [Value; EXTERNAL_DIM],
        proposed_external: [Value; EXTERNAL_DIM],
    ) -> Option<Value> {
        let previous_internal = if self.vbic_cached_external_matches(previous_external, 1e-12, 1e-9)
        {
            self.internal_state_vector()
        } else {
            let solved_previous = self.solve_intrinsic_terminal_state(
                previous_external[EXT_C],
                previous_external[EXT_B],
                previous_external[EXT_E],
                previous_external[EXT_S],
            );
            [
                solved_previous.vcx,
                solved_previous.vci,
                solved_previous.vbx,
                solved_previous.vbi,
                solved_previous.vei,
                solved_previous.vbp,
                solved_previous.vsi,
                solved_previous.vrth,
            ]
        };

        self.vbic_external_step_limit_scale_from_state(
            previous_external,
            previous_internal,
            proposed_external,
        )
    }

    pub(super) fn evaluate_state_fixed_temperature(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        vcx: Value,
        vci: Value,
        vbx: Value,
        vbi: Value,
        vei: Value,
        vbp: Value,
        vsi: Value,
    ) -> EvaluatedBjtState {
        let (linearized, intrinsic) = self.linearize_currents_with_branches(vbi - vei, vbi - vci);
        EvaluatedBjtState {
            linearized,
            ibe: intrinsic.ibe,
            ibc: intrinsic.ibc,
            iciei: intrinsic.iciei,
            ircx: self.ircx_branch(vc, vcx),
            irci: self.irci_branch(vcx, vci, vbi),
            irbx: self.irbx_branch(vb, vbx),
            irbi: self.irbi_branch(linearized, vbx, vbi),
            ire: self.ire_branch(ve, vei),
            ibep: self.ibep_branch(vbx, vbp),
            irbp: self.irbp_branch(vbx, vbi, vcx, vci, vbp, vsi),
            ibcp: self.ibcp_branch(vbp, vsi),
            iccp: self.iccp_branch(vbx, vbi, vci, vbp, vsi),
            irs: self.irs_branch(vs, vsi),
        }
    }

    pub(super) fn apply_thermal_derivative(
        base: &mut BranchLinearization,
        plus: BranchLinearization,
        minus: BranchLinearization,
        denom: Value,
    ) {
        base.d_internal[IDX_VRTH] = (plus.current - minus.current) / denom;
    }

    pub(super) fn evaluate_state(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        vcx: Value,
        vci: Value,
        vbx: Value,
        vbi: Value,
        vei: Value,
        vbp: Value,
        vsi: Value,
        vrth: Value,
    ) -> EvaluatedBjtState {
        let mut evaluated = self.with_temperature_variant(vrth, |model| {
            model
                .evaluate_state_fixed_temperature(vc, vb, ve, vs, vcx, vci, vbx, vbi, vei, vbp, vsi)
        });

        if !self.self_heating_enabled() {
            return evaluated;
        }

        let h = self.thermal_derivative_step(vrth);
        let plus = self.with_temperature_variant(vrth + h, |model| {
            model
                .evaluate_state_fixed_temperature(vc, vb, ve, vs, vcx, vci, vbx, vbi, vei, vbp, vsi)
        });
        let minus = self.with_temperature_variant(vrth - h, |model| {
            model
                .evaluate_state_fixed_temperature(vc, vb, ve, vs, vcx, vci, vbx, vbi, vei, vbp, vsi)
        });
        let denom = 2.0 * h;

        evaluated.linearized.dic_dvrth = (plus.linearized.ic - minus.linearized.ic) / denom;
        evaluated.linearized.dib_dvrth = (plus.linearized.ib - minus.linearized.ib) / denom;
        evaluated.linearized.dqb_dvrth = (plus.linearized.qb - minus.linearized.qb) / denom;
        Self::apply_thermal_derivative(&mut evaluated.ibe, plus.ibe, minus.ibe, denom);
        Self::apply_thermal_derivative(&mut evaluated.ibc, plus.ibc, minus.ibc, denom);
        Self::apply_thermal_derivative(&mut evaluated.iciei, plus.iciei, minus.iciei, denom);
        Self::apply_thermal_derivative(&mut evaluated.ircx, plus.ircx, minus.ircx, denom);
        Self::apply_thermal_derivative(&mut evaluated.irci, plus.irci, minus.irci, denom);
        Self::apply_thermal_derivative(&mut evaluated.irbx, plus.irbx, minus.irbx, denom);
        Self::apply_thermal_derivative(&mut evaluated.irbi, plus.irbi, minus.irbi, denom);
        Self::apply_thermal_derivative(&mut evaluated.ire, plus.ire, minus.ire, denom);
        Self::apply_thermal_derivative(&mut evaluated.ibep, plus.ibep, minus.ibep, denom);
        Self::apply_thermal_derivative(&mut evaluated.irbp, plus.irbp, minus.irbp, denom);
        Self::apply_thermal_derivative(&mut evaluated.ibcp, plus.ibcp, minus.ibcp, denom);
        Self::apply_thermal_derivative(&mut evaluated.iccp, plus.iccp, minus.iccp, denom);
        Self::apply_thermal_derivative(&mut evaluated.irs, plus.irs, minus.irs, denom);
        evaluated
    }

    pub(super) fn intrinsic_state_for_biases(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> IntrinsicTerminalState {
        if self.cache_matches_external_biases(vc, vb, ve, vs) {
            let thermal_model = self
                .self_heating_enabled()
                .then(|| self.temperature_variant(self.vrth));
            let model = thermal_model.as_ref().unwrap_or(self);
            IntrinsicTerminalState {
                vcx: self.vcx,
                vci: self.vci,
                vbx: self.vbx,
                vbi: self.vbi,
                vei: self.vei,
                vbp: self.vbp,
                vsi: self.vsi,
                vrth: self.vrth,
                linearized: model.linearize_currents(self.vbe, self.vbc),
            }
        } else {
            self.solve_intrinsic_terminal_state(vc, vb, ve, vs)
        }
    }

    #[inline]
    pub(super) fn intrinsic_state_residual_norm(residual: &[Value; INTERNAL_DIM]) -> Value {
        residual
            .iter()
            .fold(0.0, |max_norm, value| max_norm.max(value.abs()))
    }

    #[inline]
    pub(super) fn intrinsic_state_step_limit(iteration: usize, residual_norm: Value) -> Value {
        if residual_norm > 1e-2 {
            if iteration < 4 { 0.25 } else { 0.15 }
        } else if residual_norm > 1e-6 {
            0.1
        } else {
            0.05
        }
    }

    pub(super) fn solve_intrinsic_state_from_seed_with_thermal_scale(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        thermal_scale: Value,
        mut state: [Value; INTERNAL_DIM],
    ) -> ([Value; INTERNAL_DIM], Value) {
        let mut best_state = state;
        let mut best_residual_norm = Value::INFINITY;

        for iteration in 0..32 {
            let (residual, jacobian) = self.intrinsic_state_residual_jacobian_with_thermal_scale(
                vc,
                vb,
                ve,
                vs,
                state,
                thermal_scale,
            );
            let residual_norm = Self::intrinsic_state_residual_norm(&residual);
            if residual_norm < best_residual_norm {
                best_residual_norm = residual_norm;
                best_state = state;
            }
            if !residual_norm.is_finite() || residual_norm < 1e-14 {
                break;
            }

            let rhs = residual.map(|value| -value);
            let Some(delta) = Self::solve_small_dense_system(&jacobian, &rhs, INTERNAL_DIM) else {
                break;
            };

            let max_raw_delta = delta
                .iter()
                .fold(0.0_f64, |max_delta, value| max_delta.max(value.abs()));
            if max_raw_delta < 1e-13 {
                break;
            }

            let base_limit = Self::intrinsic_state_step_limit(iteration, residual_norm);
            let mut alpha = if max_raw_delta > base_limit {
                base_limit / max_raw_delta
            } else {
                1.0
            };
            alpha = alpha.clamp(1e-3, 1.0);

            let mut accepted = false;
            let mut candidate = state;
            let mut candidate_residual_norm = residual_norm;
            let mut best_candidate = state;
            let mut best_candidate_residual_norm = residual_norm;
            for _ in 0..12 {
                for idx in 0..INTERNAL_DIM {
                    candidate[idx] = state[idx] + alpha * delta[idx];
                }
                candidate = self.limit_intrinsic_state_against_previous(candidate, state);
                let (candidate_residual, _) = self
                    .intrinsic_state_residual_jacobian_with_thermal_scale(
                        vc,
                        vb,
                        ve,
                        vs,
                        candidate,
                        thermal_scale,
                    );
                candidate_residual_norm = Self::intrinsic_state_residual_norm(&candidate_residual);
                if candidate_residual_norm.is_finite()
                    && candidate_residual_norm < best_candidate_residual_norm
                {
                    best_candidate = candidate;
                    best_candidate_residual_norm = candidate_residual_norm;
                }
                if candidate_residual_norm.is_finite() && candidate_residual_norm < residual_norm {
                    accepted = true;
                    break;
                }
                alpha *= 0.5;
            }

            if !accepted && best_candidate_residual_norm < residual_norm {
                candidate = best_candidate;
                candidate_residual_norm = best_candidate_residual_norm;
                accepted = true;
            }

            if !accepted {
                break;
            }

            state = candidate;
            if candidate_residual_norm < best_residual_norm {
                best_residual_norm = candidate_residual_norm;
                best_state = state;
            }
            if candidate_residual_norm < 1e-14 {
                break;
            }
        }

        (best_state, best_residual_norm)
    }

    pub(super) fn solve_intrinsic_state_from_seed(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        state: [Value; INTERNAL_DIM],
    ) -> ([Value; INTERNAL_DIM], Value) {
        self.solve_intrinsic_state_from_seed_with_thermal_scale(vc, vb, ve, vs, 1.0, state)
    }

    pub(super) fn solve_intrinsic_state_with_self_heating_continuation(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        state: [Value; INTERNAL_DIM],
    ) -> ([Value; INTERNAL_DIM], Value) {
        let (direct_state, direct_residual_norm) =
            self.solve_intrinsic_state_from_seed(vc, vb, ve, vs, state);
        if !self.self_heating_enabled() {
            return (direct_state, direct_residual_norm);
        }

        let minimum_vrth = 0.0_f64.max(self.minimum_thermal_rise());
        let mut continuation_state = state;
        continuation_state[IDX_VRTH] = continuation_state[IDX_VRTH].max(minimum_vrth);
        for thermal_scale in [0.0, 0.05, 0.125, 0.25, 0.5, 0.75, 1.0] {
            if thermal_scale == 0.0 {
                continuation_state[IDX_VRTH] = minimum_vrth;
            }
            let (solved_state, _) = self.solve_intrinsic_state_from_seed_with_thermal_scale(
                vc,
                vb,
                ve,
                vs,
                thermal_scale,
                continuation_state,
            );
            continuation_state = solved_state;
        }

        let (continued_state, continued_residual_norm) =
            self.solve_intrinsic_state_from_seed(vc, vb, ve, vs, continuation_state);
        if continued_residual_norm < direct_residual_norm {
            (continued_state, continued_residual_norm)
        } else {
            (direct_state, direct_residual_norm)
        }
    }

    pub(super) fn rebalance_intrinsic_thermal_state(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        state: [Value; INTERNAL_DIM],
    ) -> [Value; INTERNAL_DIM] {
        if !self.self_heating_enabled() {
            return state;
        }

        let mut current_state = state;
        let mut best_state = state;
        let mut best_residual = Value::INFINITY;
        let minimum_vrth = self.minimum_thermal_rise();

        for _ in 0..8 {
            let (residual, jacobian) =
                self.intrinsic_state_residual_jacobian(vc, vb, ve, vs, current_state);
            let thermal_residual = residual[IDX_VRTH];
            let thermal_residual_abs = thermal_residual.abs();
            if thermal_residual_abs.is_finite() && thermal_residual_abs < best_residual {
                best_residual = thermal_residual_abs;
                best_state = current_state;
            }
            let thermal_derivative = jacobian[IDX_VRTH][IDX_VRTH];
            if !thermal_residual.is_finite()
                || !thermal_derivative.is_finite()
                || thermal_derivative.abs() < 1e-18
                || thermal_residual_abs < 1e-12
            {
                break;
            }

            let current_vrth = current_state[IDX_VRTH];
            let max_step = (current_vrth - minimum_vrth + 10.0).max(1.0) * 0.5;
            let step = (-thermal_residual / thermal_derivative).clamp(-max_step, max_step);
            if step.abs() < 1e-12 {
                break;
            }

            let mut alpha = 1.0;
            let mut accepted = false;
            let mut best_candidate = current_state;
            let mut best_candidate_residual = thermal_residual_abs;
            for _ in 0..10 {
                let raw_vrth = current_vrth + alpha * step;
                let candidate_vrth =
                    Self::limit_logarithmic_step(raw_vrth, current_vrth, 100.0).max(minimum_vrth);
                if (candidate_vrth - current_vrth).abs() < 1e-12 {
                    break;
                }

                let mut candidate = current_state;
                candidate[IDX_VRTH] = candidate_vrth;
                let candidate_residual = self
                    .intrinsic_state_residual_jacobian(vc, vb, ve, vs, candidate)
                    .0[IDX_VRTH]
                    .abs();
                if candidate_residual.is_finite() && candidate_residual < best_candidate_residual {
                    best_candidate = candidate;
                    best_candidate_residual = candidate_residual;
                }
                if candidate_residual.is_finite() && candidate_residual < thermal_residual_abs {
                    current_state = candidate;
                    accepted = true;
                    break;
                }
                alpha *= 0.5;
            }

            if accepted {
                continue;
            }
            if best_candidate_residual + 1e-15 < thermal_residual_abs {
                current_state = best_candidate;
                continue;
            }
            break;
        }

        best_state
    }

    pub(super) fn intrinsic_state_residual_jacobian_with_thermal_scale(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        state: [Value; INTERNAL_DIM],
        thermal_scale: Value,
    ) -> ([Value; INTERNAL_DIM], [[Value; INTERNAL_DIM]; INTERNAL_DIM]) {
        let has_rcx = Self::series_active(self.rcx);
        let has_rci = Self::series_active(self.rci);
        let has_rbx = Self::series_active(self.rbx);
        let has_rbi = Self::series_active(self.rbi);
        let has_re = Self::series_active(self.re);
        let has_rs = Self::series_active(self.rs);
        let has_self_heat = self.self_heating_enabled();
        let solve_vbp = Self::series_active(self.rbp)
            || self.ibeip > 0.0
            || self.ibenp > 0.0
            || self.ibcip > 0.0
            || self.ibcnp > 0.0;

        let [
            mut vcx,
            mut vci,
            mut vbx,
            mut vbi,
            mut vei,
            mut vbp,
            mut vsi,
            mut vrth,
        ] = state;
        if !has_rcx {
            vcx = vc;
        }
        if !has_rci {
            vci = vcx;
        }
        if !has_rbx {
            vbx = vb;
        }
        if !has_rbi {
            vbi = vbx;
        }
        if !has_re {
            vei = ve;
        }
        if !has_rs {
            vsi = vs;
        }
        if !solve_vbp {
            vbp = vcx;
        }
        if !has_self_heat {
            vrth = 0.0;
        }

        let eval = self.evaluate_state(vc, vb, ve, vs, vcx, vci, vbx, vbi, vei, vbp, vsi, vrth);
        let (collector_d, base_d, emitter_d) = self.intrinsic_terminal_derivatives(eval.linearized);
        let collector_internal = Self::branch_from_internal(eval.linearized.ic, collector_d);
        let base_internal = Self::branch_from_internal(eval.linearized.ib, base_d);
        let emitter_internal =
            Self::branch_from_internal(-(eval.linearized.ic + eval.linearized.ib), emitter_d);
        let thermal_sink = self.thermal_sink_branch(vrth);
        let thermal_power = Self::scale_branch(
            self.thermal_power_branch(eval, [vc, vb, ve, vs], state),
            thermal_scale,
        );

        let mut jacobian = [[0.0; INTERNAL_DIM]; INTERNAL_DIM];
        let mut residual = [0.0; INTERNAL_DIM];

        if has_rcx {
            let row = Self::sub_branches(
                Self::add_branches(eval.ircx, eval.irbp),
                if has_rci {
                    eval.irci
                } else {
                    collector_internal
                },
            );
            residual[IDX_VCX] = row.current;
            jacobian[IDX_VCX] = row.d_internal;
        } else {
            residual[IDX_VCX] = vcx - vc;
            jacobian[IDX_VCX][IDX_VCX] = 1.0;
        }

        if has_rci {
            let row = Self::sub_branches(eval.irci, collector_internal);
            residual[IDX_VCI] = row.current;
            jacobian[IDX_VCI] = row.d_internal;
        } else {
            residual[IDX_VCI] = vci - vcx;
            jacobian[IDX_VCI][IDX_VCI] = 1.0;
            jacobian[IDX_VCI][IDX_VCX] = -1.0;
        }

        if has_rbx {
            let row = Self::sub_branches(
                Self::sub_branches(
                    Self::sub_branches(eval.irbx, if has_rbi { eval.irbi } else { base_internal }),
                    eval.ibep,
                ),
                eval.iccp,
            );
            residual[IDX_VBX] = row.current;
            jacobian[IDX_VBX] = row.d_internal;
        } else {
            residual[IDX_VBX] = vbx - vb;
            jacobian[IDX_VBX][IDX_VBX] = 1.0;
        }

        if has_rbi {
            let row = Self::sub_branches(eval.irbi, base_internal);
            residual[IDX_VBI] = row.current;
            jacobian[IDX_VBI] = row.d_internal;
        } else {
            residual[IDX_VBI] = vbi - vbx;
            jacobian[IDX_VBI][IDX_VBI] = 1.0;
            jacobian[IDX_VBI][IDX_VBX] = -1.0;
        }

        if has_re {
            let row = Self::sub_branches(eval.ire, emitter_internal);
            residual[IDX_VEI] = row.current;
            jacobian[IDX_VEI] = row.d_internal;
        } else {
            residual[IDX_VEI] = vei - ve;
            jacobian[IDX_VEI][IDX_VEI] = 1.0;
        }

        if solve_vbp {
            let row = Self::sub_branches(Self::add_branches(eval.ibep, eval.ibcp), eval.irbp);
            residual[IDX_VBP] = row.current;
            jacobian[IDX_VBP] = row.d_internal;
        } else {
            residual[IDX_VBP] = vbp - vcx;
            jacobian[IDX_VBP][IDX_VBP] = 1.0;
            jacobian[IDX_VBP][IDX_VCX] = -1.0;
        }

        if has_rs {
            let row = Self::sub_branches(Self::add_branches(eval.irs, eval.iccp), eval.ibcp);
            residual[IDX_VSI] = row.current;
            jacobian[IDX_VSI] = row.d_internal;
        } else {
            residual[IDX_VSI] = vsi - vs;
            jacobian[IDX_VSI][IDX_VSI] = 1.0;
        }

        if has_self_heat {
            let row = Self::sub_branches(thermal_sink, thermal_power);
            residual[IDX_VRTH] = row.current;
            jacobian[IDX_VRTH] = row.d_internal;
        } else {
            residual[IDX_VRTH] = vrth;
            jacobian[IDX_VRTH][IDX_VRTH] = 1.0;
        }

        (residual, jacobian)
    }

    pub(super) fn intrinsic_state_residual_jacobian(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        state: [Value; INTERNAL_DIM],
    ) -> ([Value; INTERNAL_DIM], [[Value; INTERNAL_DIM]; INTERNAL_DIM]) {
        self.intrinsic_state_residual_jacobian_with_thermal_scale(vc, vb, ve, vs, state, 1.0)
    }

    pub(super) fn internal_kcl_linearization(
        &self,
        state: IntrinsicTerminalState,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> (
        EvaluatedBjtState,
        [[Value; INTERNAL_DIM]; INTERNAL_DIM],
        [[Value; EXTERNAL_DIM]; INTERNAL_DIM],
    ) {
        let eval = self.evaluate_state(
            vc, vb, ve, vs, state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp,
            state.vsi, state.vrth,
        );
        let (jacobian, external_partials, _) =
            self.internal_kcl_linearization_from_eval(state, eval, vc, vb, ve, vs);
        (eval, jacobian, external_partials)
    }

    pub(super) fn internal_kcl_linearization_from_eval(
        &self,
        state: IntrinsicTerminalState,
        eval: EvaluatedBjtState,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> (
        [[Value; INTERNAL_DIM]; INTERNAL_DIM],
        [[Value; EXTERNAL_DIM]; INTERNAL_DIM],
        [Value; INTERNAL_DIM],
    ) {
        let has_rcx = Self::series_active(self.rcx);
        let has_rci = Self::series_active(self.rci);
        let has_rbx = Self::series_active(self.rbx);
        let has_rbi = Self::series_active(self.rbi);
        let has_re = Self::series_active(self.re);
        let has_rs = Self::series_active(self.rs);
        let has_self_heat = self.self_heating_enabled();
        let solve_vbp = Self::series_active(self.rbp)
            || self.ibeip > 0.0
            || self.ibenp > 0.0
            || self.ibcip > 0.0
            || self.ibcnp > 0.0;
        let (collector_d, base_d, emitter_d) = self.intrinsic_terminal_derivatives(eval.linearized);
        let collector_internal = Self::branch_from_internal(eval.linearized.ic, collector_d);
        let base_internal = Self::branch_from_internal(eval.linearized.ib, base_d);
        let emitter_internal =
            Self::branch_from_internal(-(eval.linearized.ic + eval.linearized.ib), emitter_d);
        let thermal_sink = self.thermal_sink_branch(state.vrth);
        let thermal_power = self.thermal_power_branch(
            eval,
            [vc, vb, ve, vs],
            [
                state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi,
                state.vrth,
            ],
        );

        let mut jacobian = [[0.0; INTERNAL_DIM]; INTERNAL_DIM];
        let mut external_partials = [[0.0; EXTERNAL_DIM]; INTERNAL_DIM];
        let mut source = [0.0; INTERNAL_DIM];
        let internal = [
            state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi, state.vrth,
        ];
        let external = [vc, vb, ve, vs];
        let assign_row = |row_idx: usize,
                          row: BranchLinearization,
                          jacobian: &mut [[Value; INTERNAL_DIM]; INTERNAL_DIM],
                          external_partials: &mut [[Value; EXTERNAL_DIM]; INTERNAL_DIM],
                          source: &mut [Value; INTERNAL_DIM]| {
            jacobian[row_idx] = row.d_internal;
            external_partials[row_idx] = row.d_external;
            source[row_idx] = row
                .d_internal
                .iter()
                .zip(internal.iter())
                .map(|(d, v)| d * v)
                .sum::<Value>()
                + row
                    .d_external
                    .iter()
                    .zip(external.iter())
                    .map(|(d, v)| d * v)
                    .sum::<Value>()
                - row.current;
        };

        if has_rcx {
            let row = Self::sub_branches(
                Self::add_branches(eval.ircx, eval.irbp),
                if has_rci {
                    eval.irci
                } else {
                    collector_internal
                },
            );
            assign_row(
                IDX_VCX,
                row,
                &mut jacobian,
                &mut external_partials,
                &mut source,
            );
        } else {
            jacobian[IDX_VCX][IDX_VCX] = 1.0;
            external_partials[IDX_VCX][EXT_C] = -1.0;
        }

        if has_rci {
            let row = Self::sub_branches(eval.irci, collector_internal);
            assign_row(
                IDX_VCI,
                row,
                &mut jacobian,
                &mut external_partials,
                &mut source,
            );
        } else {
            jacobian[IDX_VCI][IDX_VCI] = 1.0;
            jacobian[IDX_VCI][IDX_VCX] = -1.0;
        }

        if has_rbx {
            let row = Self::sub_branches(
                Self::sub_branches(
                    Self::sub_branches(eval.irbx, if has_rbi { eval.irbi } else { base_internal }),
                    eval.ibep,
                ),
                eval.iccp,
            );
            assign_row(
                IDX_VBX,
                row,
                &mut jacobian,
                &mut external_partials,
                &mut source,
            );
        } else {
            jacobian[IDX_VBX][IDX_VBX] = 1.0;
            external_partials[IDX_VBX][EXT_B] = -1.0;
        }

        if has_rbi {
            let row = Self::sub_branches(eval.irbi, base_internal);
            assign_row(
                IDX_VBI,
                row,
                &mut jacobian,
                &mut external_partials,
                &mut source,
            );
        } else {
            jacobian[IDX_VBI][IDX_VBI] = 1.0;
            jacobian[IDX_VBI][IDX_VBX] = -1.0;
        }

        if has_re {
            let row = Self::sub_branches(eval.ire, emitter_internal);
            assign_row(
                IDX_VEI,
                row,
                &mut jacobian,
                &mut external_partials,
                &mut source,
            );
        } else {
            jacobian[IDX_VEI][IDX_VEI] = 1.0;
            external_partials[IDX_VEI][EXT_E] = -1.0;
        }

        if solve_vbp {
            let row = Self::sub_branches(Self::add_branches(eval.ibep, eval.ibcp), eval.irbp);
            assign_row(
                IDX_VBP,
                row,
                &mut jacobian,
                &mut external_partials,
                &mut source,
            );
        } else {
            jacobian[IDX_VBP][IDX_VBP] = 1.0;
            jacobian[IDX_VBP][IDX_VCX] = -1.0;
        }

        if has_rs {
            let row = Self::sub_branches(Self::add_branches(eval.irs, eval.iccp), eval.ibcp);
            assign_row(
                IDX_VSI,
                row,
                &mut jacobian,
                &mut external_partials,
                &mut source,
            );
        } else {
            jacobian[IDX_VSI][IDX_VSI] = 1.0;
            external_partials[IDX_VSI][EXT_S] = -1.0;
        }

        if has_self_heat {
            let row = Self::sub_branches(thermal_sink, thermal_power);
            assign_row(
                IDX_VRTH,
                row,
                &mut jacobian,
                &mut external_partials,
                &mut source,
            );
        } else {
            jacobian[IDX_VRTH][IDX_VRTH] = 1.0;
        }

        (jacobian, external_partials, source)
    }

    pub(super) fn reduced_linearization_from_state_and_eval(
        &self,
        state: IntrinsicTerminalState,
        eval: EvaluatedBjtState,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> BjtReducedLinearization {
        let (g_ii, g_ie, z_i_static) =
            self.internal_kcl_linearization_from_eval(state, eval, vc, vb, ve, vs);
        let terminal_currents = self.external_terminal_branches(eval);
        let (g_ei, g_ee, g_reduced) =
            Self::linearized_terminal_conductance_matrices(&g_ii, &g_ie, &terminal_currents);
        let internal = [
            state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi, state.vrth,
        ];
        let external = [vc, vb, ve, vs];
        let mut z_e_static = [0.0; EXTERNAL_DIM];
        for row in 0..EXTERNAL_DIM {
            z_e_static[row] = terminal_currents[row]
                .d_internal
                .iter()
                .zip(internal.iter())
                .map(|(d, v)| d * v)
                .sum::<Value>()
                + terminal_currents[row]
                    .d_external
                    .iter()
                    .zip(external.iter())
                    .map(|(d, v)| d * v)
                    .sum::<Value>()
                - terminal_currents[row].current;
        }
        let cached_dynamic_inputs = if self.uses_vbic_dynamic_charges() {
            let internal = [
                state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi,
                state.vrth, 0.0, 0.0,
            ];
            Some(if self.self_heating_enabled() {
                self.with_temperature_variant(state.vrth, |model| {
                    model.dynamic_charge_inputs(external, internal)
                })
            } else {
                self.dynamic_charge_inputs(external, internal)
            })
        } else {
            None
        };

        BjtReducedLinearization {
            internal_voltages: [
                state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi,
                state.vrth,
            ],
            external_voltages: [vc, vb, ve, vs],
            g_ii,
            g_ie,
            g_ei,
            g_ee,
            g_reduced,
            z_i_static,
            z_e_static,
            cached_dynamic_inputs,
        }
    }

    #[inline]
    pub(super) fn intrinsic_state_from_internal_vector(
        &self,
        internal: [Value; INTERNAL_DIM],
    ) -> IntrinsicTerminalState {
        let [vcx, vci, vbx, vbi, vei, vbp, vsi, vrth] = internal;
        let linearized = self
            .with_temperature_variant(vrth, |model| model.linearize_currents(vbi - vei, vbi - vci));

        IntrinsicTerminalState {
            vcx,
            vci,
            vbx,
            vbi,
            vei,
            vbp,
            vsi,
            vrth,
            linearized,
        }
    }

    pub(super) fn compute_reduced_linearization(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> BjtReducedLinearization {
        let state = self.intrinsic_state_for_biases(vc, vb, ve, vs);
        let eval = self.evaluate_state(
            vc, vb, ve, vs, state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp,
            state.vsi, state.vrth,
        );
        self.reduced_linearization_from_state_and_eval(state, eval, vc, vb, ve, vs)
    }

    pub(crate) fn reduced_linearization(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> BjtReducedLinearization {
        if self.reduced_linearization_cache_valid.get()
            && self.cache_matches_external_biases(vc, vb, ve, vs)
        {
            return self.reduced_linearization_cache.get();
        }

        let reduced = self.compute_reduced_linearization(vc, vb, ve, vs);
        if self.cache_matches_external_biases(vc, vb, ve, vs) {
            self.reduced_linearization_cache.set(reduced);
            self.reduced_linearization_cache_valid.set(true);
        }
        reduced
    }

    pub(super) fn linearized_terminal_conductance_matrices(
        g_ii: &[[Value; INTERNAL_DIM]; INTERNAL_DIM],
        g_ie: &[[Value; EXTERNAL_DIM]; INTERNAL_DIM],
        terminal_currents: &[BranchLinearization; EXTERNAL_DIM],
    ) -> (
        [[Value; INTERNAL_DIM]; EXTERNAL_DIM],
        [[Value; EXTERNAL_DIM]; EXTERNAL_DIM],
        BjtConductanceMatrix,
    ) {
        let mut g_ei = [[0.0; INTERNAL_DIM]; EXTERNAL_DIM];
        let mut g_ee = [[0.0; EXTERNAL_DIM]; EXTERNAL_DIM];
        for row in 0..EXTERNAL_DIM {
            g_ei[row] = terminal_currents[row].d_internal;
            g_ee[row] = terminal_currents[row].d_external;
        }

        let mut sensitivities = [[0.0; EXTERNAL_DIM]; INTERNAL_DIM];
        for external in 0..EXTERNAL_DIM {
            let rhs = g_ie.map(|partials| -partials[external]);
            if let Some(solution) = Self::solve_small_dense_system(g_ii, &rhs, INTERNAL_DIM) {
                for idx in 0..INTERNAL_DIM {
                    sensitivities[idx][external] = solution[idx];
                }
            }
        }

        let mut g_reduced = [[0.0; EXTERNAL_DIM]; EXTERNAL_DIM];
        for row in 0..EXTERNAL_DIM {
            for col in 0..EXTERNAL_DIM {
                let mut value = g_ee[row][col];
                for internal in 0..INTERNAL_DIM {
                    value += g_ei[row][internal] * sensitivities[internal][col];
                }
                g_reduced[row][col] = value;
            }
        }

        (g_ei, g_ee, g_reduced)
    }
}
