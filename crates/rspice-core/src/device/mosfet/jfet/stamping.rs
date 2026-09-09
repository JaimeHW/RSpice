//! Matrix linking, direct stamping, and nonlinear-device trait integration.

use super::*;

impl Jfet {
    fn regularized_gate_conductance(
        &self,
        voltage: Value,
        current: Value,
        conductance: Value,
    ) -> Value {
        if conductance.is_finite() && conductance >= self.junction_gmin {
            return conductance;
        }

        if voltage.abs() > 1.0e-12 {
            let chord = current / voltage;
            if chord.is_finite() && chord > self.junction_gmin {
                return chord;
            }
        }

        self.junction_gmin
    }

    fn cache_operating_terms_at(
        &mut self,
        vgs: Value,
        vds: Value,
        vgd: Value,
        external_vd: Value,
        external_vs: Value,
        regularize_gate_conductance: bool,
    ) {
        let (ids, gm, gds, igs, igd, mut ggs, mut ggd, vds_linear, gmg, gmd) =
            self.compute_operating_terms_with_terminals(vgs, vds, vgd, external_vd, external_vs);
        if regularize_gate_conductance
            && self.has_gate_generation_branch()
            && !(self.params.hfet_gatemod && self.params.hfet_level >= 5)
        {
            ggs = self.regularized_gate_conductance(vgs, igs, ggs);
            ggd = self.regularized_gate_conductance(vgd, igd, ggd);
        }
        self.eval_ids = ids;
        self.eval_gm = gm;
        self.eval_gds = gds;
        self.eval_igs = igs;
        self.eval_igd = igd;
        self.eval_ggs = ggs;
        self.eval_ggd = ggd;
        self.eval_gmg = gmg;
        self.eval_gmd = gmd;
        self.eval_vds_linear = vds_linear;
        self.lin_vgs = vgs;
        self.lin_vgd = vgd;
        self.lin_cg = igs + igd;
        self.lin_cd = ids - igd;
        self.eval_valid = true;
    }

    /// Re-linearize directly at the supplied candidate solution.
    ///
    /// This is used only by static residual/validation probes. Regular Newton
    /// updates must continue to use ngspice-style branch limiting, but fallback
    /// candidates are already full operating-point candidates and must be
    /// evaluated at their actual terminal voltages.
    pub(crate) fn update_static_linearization(&mut self, voltages: &[Value]) {
        let vd = Self::node_voltage(voltages, self.drain);
        let vg = Self::node_voltage(voltages, self.gate);
        let vs = Self::node_voltage(voltages, self.source);
        let (external_vd, external_vs) = self.external_terminal_voltages(voltages);
        let vgs = vg - vs;
        let vgd = vg - vd;
        let vds = vgs - vgd;

        if self.vgs.is_finite() && self.vds.is_finite() {
            self.vgs_prev = self.vgs;
            self.vds_prev = self.vds;
        }
        self.vgs = vgs;
        self.vds = vds;
        self.limiter_applied = false;
        self.last_raw_vgs_prev = self.last_raw_vgs;
        self.last_raw_vgd_prev = self.last_raw_vgd;
        self.last_raw_vgs = vgs;
        self.last_raw_vgd = vgd;
        self.cache_operating_terms_at(vgs, vds, vgd, external_vd, external_vs, false);
    }

    /// Link this device to a StaticMatrix for O(1) direct stamping.
    pub fn link(&mut self, matrix: &StaticMatrix) {
        let d = self.drain;
        let g = self.gate;
        let s = self.source;
        self.indices = JfetIndices::default();

        if d > 0 {
            self.indices.dd = matrix.get_index(d - 1, d - 1);
        }
        if d > 0 && g > 0 {
            self.indices.dg = matrix.get_index(d - 1, g - 1);
        }
        if d > 0 && s > 0 {
            self.indices.ds = matrix.get_index(d - 1, s - 1);
        }

        if g > 0 && d > 0 {
            self.indices.gd = matrix.get_index(g - 1, d - 1);
        }
        if g > 0 {
            self.indices.gg = matrix.get_index(g - 1, g - 1);
        }
        if g > 0 && s > 0 {
            self.indices.gs = matrix.get_index(g - 1, s - 1);
        }

        if s > 0 && d > 0 {
            self.indices.sd = matrix.get_index(s - 1, d - 1);
        }
        if s > 0 && g > 0 {
            self.indices.sg = matrix.get_index(s - 1, g - 1);
        }
        if s > 0 {
            self.indices.ss = matrix.get_index(s - 1, s - 1);
        }
    }

    /// Project the independent channel and gate branches before accumulation.
    /// A tied branch contributes exactly zero, even if its derivative is much
    /// larger than the remaining physical conductances.
    #[inline]
    pub(crate) fn terminal_jacobian(
        [d, g, s]: [NodeId; 3],
        gm: Value,
        gds: Value,
        ggs: Value,
        ggd: Value,
        gmg: Value,
        gmd: Value,
    ) -> [[Value; 3]; 3] {
        if d == s {
            if g == s {
                return [[0.0; 3]; 3];
            }
            let gate = ggs + ggd + gmg;
            return [[0.0; 3], [0.0, gate, -gate], [0.0, -gate, gate]];
        }
        let tied_conductance = if g == d {
            Some(gm + gds + ggs)
        } else if g == s {
            Some(gds + ggd - gmd)
        } else {
            None
        };
        if let Some(conductance) = tied_conductance {
            return [
                [conductance, 0.0, -conductance],
                [0.0; 3],
                [-conductance, 0.0, conductance],
            ];
        }
        [
            [gds + ggd - gmd, gm - ggd - gmg, -gm - gds + gmg + gmd],
            [-ggd + gmd, ggs + ggd + gmg, -ggs - gmg - gmd],
            [-gds, -gm - ggs, gm + gds + ggs],
        ]
    }

    #[inline]
    pub(crate) fn terminal_current_injections(
        [d, g, s]: [NodeId; 3],
        ids: Value,
        igs: Value,
        igd: Value,
    ) -> [Value; 3] {
        if d == s {
            if g == s {
                return [0.0; 3];
            }
            let gate = igs + igd;
            [0.0, -gate, gate]
        } else if g == d {
            let current = ids + igs;
            [-current, 0.0, current]
        } else if g == s {
            let current = ids - igd;
            [-current, 0.0, current]
        } else {
            [-ids + igd, -igs - igd, ids + igs]
        }
    }

    #[inline]
    fn linearized_terminal_stamp(&self, voltages: &[Value]) -> ([[Value; 3]; 3], [Value; 3]) {
        let (vgs, vds, vgd) = self.state_or_raw_branch_voltages(voltages);
        let (external_vd, external_vs) = self.external_terminal_voltages(voltages);

        let (ids, gm, gds, igs, igd, ggs, ggd, vds_linear, gmg, gmd) = if self.eval_valid {
            (
                self.eval_ids,
                self.eval_gm,
                self.eval_gds,
                self.eval_igs,
                self.eval_igd,
                self.eval_ggs,
                self.eval_ggd,
                self.eval_vds_linear,
                self.eval_gmg,
                self.eval_gmd,
            )
        } else {
            self.compute_operating_terms_with_terminals(vgs, vds, vgd, external_vd, external_vs)
        };
        let ids_eq = ids - gm * vgs - gds * vds_linear;
        let igs_eq = igs - ggs * vgs;
        // GATEMOD=1's gate-drain branch is controlled by vgs and vds
        // (gmg/gmd), not vgd; both are zero for the diode gate models.
        let igd_eq = igd - ggd * vgd - gmg * vgs - gmd * vds;

        let nodes = [self.drain, self.gate, self.source];
        (
            Self::terminal_jacobian(nodes, gm, gds, ggs, ggd, gmg, gmd),
            Self::terminal_current_injections(nodes, ids_eq, igs_eq, igd_eq),
        )
    }

    /// Stamp using O(1) direct indexing (call after `link`).
    pub fn stamp_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value], voltages: &[Value]) {
        let (jacobian, injections) = self.linearized_terminal_stamp(voltages);
        let indices = [
            [self.indices.dd, self.indices.dg, self.indices.ds],
            [self.indices.gd, self.indices.gg, self.indices.gs],
            [self.indices.sd, self.indices.sg, self.indices.ss],
        ];
        for (row, entries) in jacobian.iter().zip(indices) {
            for (&value, index) in row.iter().zip(entries) {
                if let Some(index) = index {
                    matrix.stamp_direct(index, value);
                }
            }
        }
        for (node, current) in [self.drain, self.gate, self.source]
            .into_iter()
            .zip(injections)
        {
            if node > 0 {
                rhs[node - 1] += current;
            }
        }
    }
}

impl NonlinearDevice for Jfet {
    fn update(&mut self, voltages: &[Value]) {
        let vd = Self::node_voltage(voltages, self.drain);
        let vg = Self::node_voltage(voltages, self.gate);
        let vs = Self::node_voltage(voltages, self.source);
        let (external_vd, external_vs) = self.external_terminal_voltages(voltages);
        let vgs_raw = vg - vs;
        let vgd_raw = vg - vd;
        if !matches!(
            self.params.channel_model,
            JfetChannelModel::XyceModifiedShockley
        ) && self.matches_last_raw_branch_input(vgs_raw, vgd_raw)
        {
            if self.vgs.is_finite() && self.vds.is_finite() {
                self.vgs_prev = self.vgs;
                self.vds_prev = self.vds;
            }
            self.last_raw_vgs_prev = self.last_raw_vgs;
            self.last_raw_vgd_prev = self.last_raw_vgd;
            return;
        }

        let vgs_prev = self.vgs;
        let vds_prev = self.vds;
        let last_raw_vgs_prev = self.last_raw_vgs;
        let last_raw_vgd_prev = self.last_raw_vgd;
        let vgd_prev = if vgs_prev.is_finite() && vds_prev.is_finite() {
            vgs_prev - vds_prev
        } else {
            Value::NAN
        };

        self.vgs_prev = vgs_prev;
        self.vds_prev = vds_prev;

        let mut vgs = vgs_raw;
        let mut vgd = vgd_raw;
        let mut limiter_applied = false;

        if matches!(self.params.channel_model, JfetChannelModel::Hfet1) {
            if matches!(self.params.hfet_level, 2..=4)
                && vgs_prev.is_finite()
                && vgd_prev.is_finite()
            {
                let (_, temp_source, temp_drain) =
                    self.resolved_temperatures(self.analysis_temperature());
                let n = self.params.n.max(1e-12);
                let vtes = (n * self.thermal_voltage(temp_source)).max(1e-12);
                let vted = (n * self.thermal_voltage(temp_drain)).max(1e-12);
                let vcrits = self.mesa_gate_vcrit(temp_source, vtes);
                let vcritd = self.mesa_gate_vcrit(temp_drain, vted);
                let vgs_limited = Self::pnjlim(vgs, vgs_prev, vtes, vcrits);
                let vgd_limited = Self::pnjlim(vgd, vgd_prev, vted, vcritd);
                limiter_applied |= (vgs_limited - vgs).abs() > 0.0;
                limiter_applied |= (vgd_limited - vgd).abs() > 0.0;
                vgs = vgs_limited;
                vgd = vgd_limited;
            }
            let (vgs_limited, vgd_limited) = self.hfet_limited_branch_voltages(vgs, vgd);
            limiter_applied |= (vgs_limited - vgs).abs() > 0.0;
            limiter_applied |= (vgd_limited - vgd).abs() > 0.0;
            vgs = vgs_limited;
            vgd = vgd_limited;
        } else if matches!(self.params.channel_model, JfetChannelModel::ParkerSkellern) {
            let (vgs_limited, vgd_limited) = self.jfet2_limited_branch_voltages(vgs, vgd);
            limiter_applied |= (vgs_limited - vgs).abs() > 0.0;
            limiter_applied |= (vgd_limited - vgd).abs() > 0.0;
            vgs = vgs_limited;
            vgd = vgd_limited;
        } else if matches!(
            self.params.channel_model,
            JfetChannelModel::XyceModifiedShockley
        ) {
            let (vgs_limited, vgd_limited) = self.xyce_jfet2_limited_branch_voltages(vgs, vgd);
            limiter_applied |= (vgs_limited - vgs).abs() > 0.0;
            limiter_applied |= (vgd_limited - vgd).abs() > 0.0;
            vgs = vgs_limited;
            vgd = vgd_limited;
        } else if matches!(self.params.channel_model, JfetChannelModel::ShichmanHodges) {
            let (vgs_limited, vgd_limited) = self.classic_limited_branch_voltages(vgs, vgd);
            limiter_applied |= (vgs_limited - vgs).abs() > 0.0;
            limiter_applied |= (vgd_limited - vgd).abs() > 0.0;
            vgs = vgs_limited;
            vgd = vgd_limited;
        } else if matches!(self.params.channel_model, JfetChannelModel::XyceSydney) {
            let (vgs_limited, vgd_limited) = self.xyce_jfet1_limited_branch_voltages(vgs, vgd);
            limiter_applied |= (vgs_limited - vgs).abs() > 0.0;
            limiter_applied |= (vgd_limited - vgd).abs() > 0.0;
            vgs = vgs_limited;
            vgd = vgd_limited;
        }

        // Independent junction limiters must still describe a realizable
        // terminal voltage. Select the more conservative common bias when
        // drain and source are tied; a tied gate junction has exact zero bias.
        if self.gate == self.source || self.gate == self.drain || self.drain == self.source {
            if self.gate == self.source {
                vgs = 0.0;
            }
            if self.gate == self.drain {
                vgd = 0.0;
            }
            if self.drain == self.source {
                let common = if (vgs - vgs_raw).abs() > (vgd - vgd_raw).abs() {
                    vgs
                } else {
                    vgd
                };
                vgs = common;
                vgd = common;
            }
            limiter_applied = vgs != vgs_raw || vgd != vgd_raw;
        }

        let mut bypassed = false;
        let can_use_static_bypass = self.params.channel_model
            != JfetChannelModel::XyceModifiedShockley
            && self.params.channel_model != JfetChannelModel::XyceSydney
            && (self.params.channel_model != JfetChannelModel::Hfet1 || self.params.hfet_level < 5);
        if can_use_static_bypass && self.eval_valid && vgs_prev.is_finite() && vgd_prev.is_finite()
        {
            const RELTOL: Value = 1e-3;
            const VOLT_TOL: Value = 1e-6;
            const ABSTOL: Value = 1e-12;

            let delvgs = vgs - vgs_prev;
            let delvgd = vgd - vgd_prev;
            let delvds = delvgs - delvgd;

            let cghat = self.lin_cg
                + self.eval_ggs * delvgs
                + self.eval_ggd * delvgd
                + self.eval_gmg * delvgs
                + self.eval_gmd * delvds;
            let cdhat = self.lin_cd + self.eval_gm * delvgs + self.eval_gds * delvds
                - self.eval_ggd * delvgd
                - self.eval_gmg * delvgs
                - self.eval_gmd * delvds;

            let vgs_ok = delvgs.abs() <= RELTOL * vgs.abs().max(vgs_prev.abs()) + VOLT_TOL;
            let vgd_ok = delvgd.abs() <= RELTOL * vgd.abs().max(vgd_prev.abs()) + VOLT_TOL;
            let cg_ok =
                (cghat - self.lin_cg).abs() <= RELTOL * cghat.abs().max(self.lin_cg.abs()) + ABSTOL;
            let cd_ok =
                (cdhat - self.lin_cd).abs() <= RELTOL * cdhat.abs().max(self.lin_cd.abs()) + ABSTOL;

            if vgs_ok && vgd_ok && cg_ok && cd_ok {
                vgs = vgs_prev;
                vgd = vgd_prev;
                bypassed = true;
            }
        }

        let vds = vgs - vgd;
        self.vgs = vgs;
        self.vds = vds;
        self.limiter_applied = limiter_applied;
        self.last_raw_vgs_prev = last_raw_vgs_prev;
        self.last_raw_vgd_prev = last_raw_vgd_prev;
        self.last_raw_vgs = vgs_raw;
        self.last_raw_vgd = vgd_raw;

        if !bypassed {
            self.cache_operating_terms_at(vgs, vds, vgd, external_vd, external_vs, true);
        }
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let (jacobian, injections) = self.linearized_terminal_stamp(voltages);
        let nodes = [self.drain, self.gate, self.source];
        for (row, &node) in nodes.iter().enumerate() {
            for (column, &other) in nodes.iter().enumerate() {
                matrix.stamp(node, other, jacobian[row][column]);
            }
            matrix.stamp_rhs(node, injections[row]);
        }
    }

    fn is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        const RELTOL: Value = 1e-3;
        let tolerance = criteria.voltage_tolerance();

        if self.limiter_applied {
            return false;
        }

        if !self.vgs.is_finite()
            || !self.vgs_prev.is_finite()
            || !self.vds.is_finite()
            || !self.vds_prev.is_finite()
        {
            return false;
        }

        let vgs_diff = (self.vgs - self.vgs_prev).abs();
        let vds_diff = (self.vds - self.vds_prev).abs();
        let vgs_tol = RELTOL * self.vgs.abs().max(self.vgs_prev.abs()) + tolerance;
        let vds_tol = RELTOL * self.vds.abs().max(self.vds_prev.abs()) + tolerance;

        if vgs_diff >= vgs_tol || vds_diff >= vds_tol {
            return false;
        }

        if matches!(self.params.channel_model, JfetChannelModel::Hfet1)
            && self.params.hfet_level >= 5
        {
            if !self.last_raw_vgs.is_finite()
                || !self.last_raw_vgs_prev.is_finite()
                || !self.last_raw_vgd.is_finite()
                || !self.last_raw_vgd_prev.is_finite()
            {
                return false;
            }

            let raw_vgs_diff = (self.last_raw_vgs - self.last_raw_vgs_prev).abs();
            let raw_vgd_diff = (self.last_raw_vgd - self.last_raw_vgd_prev).abs();
            let raw_vgs_tol =
                RELTOL * self.last_raw_vgs.abs().max(self.last_raw_vgs_prev.abs()) + tolerance;
            let raw_vgd_tol =
                RELTOL * self.last_raw_vgd.abs().max(self.last_raw_vgd_prev.abs()) + tolerance;
            return raw_vgs_diff < raw_vgs_tol && raw_vgd_diff < raw_vgd_tol;
        }

        true
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tied_terminal_stamps_retain_only_the_active_physical_branches() {
        // (D,G,S), channel/gate derivatives, branch currents, expected G and
        // injection at node 1. Huge terms belong only to tied branches.
        let cases = [
            (
                [1, 2, 1],
                [1e100, 1e100, 1.0, 2.0, 3.0, 1e100],
                [1e100, 2.0, 3.0],
                6.0,
                5.0,
            ),
            (
                [1, 1, 2],
                [1.0, 2.0, 3.0, 1e100, 1e100, 1e100],
                [3.0, 2.0, 1e100],
                6.0,
                -5.0,
            ),
            (
                [1, 2, 2],
                [1e100, 1.0, 1e100, 2.0, 1e100, 0.5],
                [3.0, 1e100, 1.0],
                2.5,
                -2.0,
            ),
            ([1, 1, 1], [1e100; 6], [1e100; 3], 0.0, 0.0),
            // Quadrature feedback may cancel when gate and drain are tied.
            (
                [1, 1, 2],
                [-1e100, 1e100, 0.0, 1e100, 1e100, 1e100],
                [3.0, -3.0, 1e100],
                0.0,
                0.0,
            ),
        ];
        for ([d, g, s], terms, currents, conductance, injection) in cases {
            let mut device = Jfet::njf("J1", d, g, s);
            [
                device.eval_gm,
                device.eval_gds,
                device.eval_ggs,
                device.eval_ggd,
                device.eval_gmg,
                device.eval_gmd,
            ] = terms;
            [device.eval_ids, device.eval_igs, device.eval_igd] = currents;
            device.eval_valid = true;
            device.vgs = 0.0;
            device.vds = 0.0;
            device.eval_vds_linear = 0.0;
            let mut matrix = StaticMatrix::from_triplets(
                2,
                2,
                &[(0, 0, 1.0), (0, 1, 0.25), (1, 0, -0.5), (1, 1, 2.0)],
            )
            .unwrap();
            device.link(&matrix);
            let mut rhs = [3.0, -4.0];
            device.stamp_direct(&mut matrix, &mut rhs, &[0.0, 0.0]);
            assert_eq!(
                matrix.values_mut(),
                &[
                    1.0 + conductance,
                    -0.5 - conductance,
                    0.25 - conductance,
                    2.0 + conductance
                ]
            );
            assert_eq!(rhs, [3.0 + injection, -4.0 - injection]);
        }
    }

    #[test]
    fn tied_gate_junctions_cannot_acquire_a_startup_voltage() {
        for nodes in [[1, 1, 1], [1, 1, 2], [1, 2, 2], [1, 2, 1]] {
            let [d, g, s] = nodes;
            let mut device = Jfet::njf("J1", d, g, s);
            for _ in 0..3 {
                device.update(&[0.25, 0.0]);
                if g == s {
                    assert_eq!(device.vgs, 0.0);
                }
                if g == d {
                    assert_eq!(device.vgs - device.vds, 0.0);
                }
                if d == s {
                    assert_eq!(device.vds, 0.0);
                }
            }
        }
    }

    #[test]
    fn off_instance_takes_the_zero_bias_startup_seed() {
        // jfetload.c splits MODEINITJCT two ways: an active instance starts at
        // `vgs = vgd = -1`, one the deck marked OFF starts at `vgs = vgd = 0`.
        // Both arms run in every compatibility mode.
        for (label, off, expected_vgs) in [("active", false, -1.0_f64), ("OFF", true, 0.0_f64)] {
            let mut jfet = Jfet::njf("j1", 1, 2, 0);
            jfet.initial_off = off;
            assert_eq!(jfet.is_initially_off(), off);
            jfet.update(&[4.0, 1.5]);
            assert_eq!(
                jfet.vgs.to_bits(),
                expected_vgs.to_bits(),
                "{label} NJF startup vgs={} expected {expected_vgs}",
                jfet.vgs
            );
            assert_eq!(jfet.vds, 0.0, "{label} NJF startup vds must be zero");
        }

        // The seed is written in internal orientation, so a P-channel instance
        // mirrors it. Zero has no sign to mirror, which is the point.
        for (label, off, expected_vgs) in [("active", false, 1.0_f64), ("OFF", true, 0.0_f64)] {
            let mut jfet = Jfet::pjf("j1", 1, 2, 0);
            jfet.initial_off = off;
            jfet.update(&[-4.0, -1.5]);
            assert_eq!(
                jfet.vgs.to_bits(),
                expected_vgs.to_bits(),
                "{label} PJF startup vgs={} expected {expected_vgs}",
                jfet.vgs
            );
        }
    }

    #[test]
    fn xyce_sydney_relinearizes_nearby_static_biases() {
        let mut jfet = Jfet::njf("j1", 1, 2, 0).enable_xyce_jfet1_model();
        jfet.params.lambda = 0.02;

        let first_bias = [15.0, -1.875];
        jfet.update(&first_bias);
        let first_ids = jfet.eval_ids;

        // This perturbation is deliberately small enough to satisfy the
        // generic SPICE JFET bypass tolerances. Xyce's native LEVEL=1 JFET
        // does not implement that bypass and must evaluate the new bias.
        let second_bias = [15.001, -1.875];
        jfet.update(&second_bias);

        let vgs = second_bias[1];
        let vds = second_bias[0];
        let vgd = vgs - vds;
        let (expected_ids, ..) =
            jfet.compute_operating_terms_with_terminals(vgs, vds, vgd, vds, 0.0);

        assert_ne!(jfet.eval_ids.to_bits(), first_ids.to_bits());
        assert_eq!(jfet.eval_ids.to_bits(), expected_ids.to_bits());
        assert_eq!(jfet.vgs.to_bits(), vgs.to_bits());
        assert!((jfet.vds - vds).abs() <= Value::EPSILON * vds.abs());
    }
}
