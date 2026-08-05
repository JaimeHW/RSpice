use super::*;

impl Mosfet {
    #[inline]
    pub(crate) fn update_with_classic_transient_constants(
        &mut self,
        voltages: &[Value],
        constants: &ClassicMosTransientConstants,
    ) {
        self.update_impl(voltages, Some(constants));
    }

    #[inline]
    fn update_impl(
        &mut self,
        voltages: &[Value],
        constants: Option<&ClassicMosTransientConstants>,
    ) {
        self.vgs_prev = self.vgs;
        self.vds_prev = self.vds;
        self.vbs_prev = self.vbs;
        self.eval_vgs_prev = self.eval_vgs;
        self.eval_vds_prev = self.eval_vds;
        self.eval_vbs_prev = self.eval_vbs;
        self.id_prev = self.id;
        self.gm_prev = self.gm;
        self.gds_prev = self.gds;
        self.gmb_prev = self.gmb;
        if self.has_branch_history {
            self.ibs_prev = self.ibs;
            self.gbs_prev = self.gbs;
            self.ibd_prev = self.ibd;
            self.gbd_prev = self.gbd;
        } else {
            if let Some(constants) = constants {
                (self.ibs_prev, self.gbs_prev) = self
                    .body_source_junction_current_and_conductance_with_constants(
                        self.eval_vbs_prev,
                        constants,
                    );
                (self.ibd_prev, self.gbd_prev) = self
                    .body_drain_junction_current_and_conductance_with_constants(
                        self.eval_vds_prev,
                        self.eval_vbs_prev,
                        constants,
                    );
            } else {
                (self.ibs_prev, self.gbs_prev) =
                    self.body_source_junction_current_and_conductance(self.eval_vbs_prev);
                (self.ibd_prev, self.gbd_prev) = self.body_drain_junction_current_and_conductance(
                    self.eval_vds_prev,
                    self.eval_vbs_prev,
                );
            }
        }

        let (vgs, vds, vbs) = self.branch_voltages(voltages);
        let (eval_vgs, eval_vds, eval_vbs) = if let Some(constants) = constants {
            self.limited_branch_voltages_for_transient_eval(vgs, vds, vbs, constants)
        } else {
            self.limited_branch_voltages_for_eval(vgs, vds, vbs)
        };
        if constants.is_some()
            && self.linearization_cache_valid
            && self.has_branch_history
            && vgs.to_bits() == self.vgs.to_bits()
            && vds.to_bits() == self.vds.to_bits()
            && vbs.to_bits() == self.vbs.to_bits()
            && eval_vgs.to_bits() == self.eval_vgs.to_bits()
            && eval_vds.to_bits() == self.eval_vds.to_bits()
            && eval_vbs.to_bits() == self.eval_vbs.to_bits()
        {
            return;
        }
        self.vgs = vgs;
        self.vds = vds;
        self.vbs = vbs;
        self.eval_vgs = eval_vgs;
        self.eval_vds = eval_vds;
        self.eval_vbs = eval_vbs;

        let (id, region, gm, gds, gmb, id_eq) = if let Some(constants) = constants {
            self.linearized_transient_operating_point(
                self.eval_vgs,
                self.eval_vds,
                self.eval_vbs,
                constants,
            )
        } else {
            self.linearized_operating_point(self.eval_vgs, self.eval_vds, self.eval_vbs)
        };
        self.id = id;
        self.region = region;
        self.gm = gm;
        self.gds = gds;
        self.gmb = gmb;
        self.id_eq = id_eq;
        if let Some(constants) = constants {
            (self.ibs, self.gbs) = self
                .body_source_junction_current_and_conductance_with_constants(
                    self.eval_vbs,
                    constants,
                );
            (self.ibd, self.gbd) = self.body_drain_junction_current_and_conductance_with_constants(
                self.eval_vds,
                self.eval_vbs,
                constants,
            );
        } else {
            (self.ibs, self.gbs) = self.body_source_junction_current_and_conductance(self.eval_vbs);
            (self.ibd, self.gbd) =
                self.body_drain_junction_current_and_conductance(self.eval_vds, self.eval_vbs);
        }
        self.has_branch_history = true;
        self.linearization_cache_valid = true;
    }
}

impl NonlinearDevice for Mosfet {
    fn update(&mut self, voltages: &[Value]) {
        self.update_impl(voltages, None);
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let (vgs, vds, vbs) = self.branch_voltages(voltages);
        let (eval_vgs, eval_vds, eval_vbs) = self.limited_branch_voltages_for_eval(vgs, vds, vbs);
        let (gm, gds, gmb, id_eq) =
            if self.cached_linearization_matches_eval(eval_vgs, eval_vds, eval_vbs) {
                (self.gm, self.gds, self.gmb, self.id_eq)
            } else {
                let (_, _, gm, gds, gmb, id_eq) =
                    self.linearized_operating_point(eval_vgs, eval_vds, eval_vbs);
                (gm, gds, gmb, id_eq)
            };

        // Stamp the linearized model (Gate draws no DC current)
        // Drain node equation
        matrix.stamp(self.node_drain, self.node_drain, gds);
        matrix.stamp(self.node_drain, self.node_gate, gm);
        matrix.stamp(self.node_drain, self.node_source, -gm - gds - gmb);
        matrix.stamp(self.node_drain, self.node_bulk, gmb);

        // Source node equation (current exits source)
        matrix.stamp(self.node_source, self.node_drain, -gds);
        matrix.stamp(self.node_source, self.node_gate, -gm);
        matrix.stamp(self.node_source, self.node_source, gm + gds + gmb);
        matrix.stamp(self.node_source, self.node_bulk, -gmb);

        // Stamp equivalent current source
        matrix.stamp_rhs(self.node_drain, -id_eq);
        matrix.stamp_rhs(self.node_source, id_eq);

        let cache_matches = self.cached_linearization_matches_eval(eval_vgs, eval_vds, eval_vbs);
        let (bs_anode, bs_cathode, gbs, ieq_bs) =
            self.body_source_junction_linearization_cached(eval_vbs, cache_matches);
        Self::stamp_diode_linearization(matrix, bs_anode, bs_cathode, gbs, ieq_bs);

        let (bd_anode, bd_cathode, gbd, ieq_bd) =
            self.body_drain_junction_linearization_cached(eval_vds, eval_vbs, cache_matches);
        Self::stamp_diode_linearization(matrix, bd_anode, bd_cathode, gbd, ieq_bd);
    }

    fn is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        // Every voltage, channel-current, and previous-junction field below
        // participates in a strict `< tolerance` predicate. IEEE NaN/Inf
        // therefore fails closed without a separate scan. The current body
        // conductances are the only convergence-state fields not consumed by
        // those expressions, so retain their explicit validity guard.
        if !self.gbs.is_finite() || !self.gbd.is_finite() {
            return false;
        }

        let reltol = criteria.relative_tolerance();
        let voltage_tol = criteria.voltage_tolerance();
        let current_tol = criteria.current_tolerance();

        let vgs_diff = (self.vgs - self.vgs_prev).abs();
        let vds_diff = (self.vds - self.vds_prev).abs();
        let vbs_diff = (self.vbs - self.vbs_prev).abs();

        let vgs_tol = reltol * self.vgs.abs().max(self.vgs_prev.abs()) + voltage_tol;
        let vds_tol = reltol * self.vds.abs().max(self.vds_prev.abs()) + voltage_tol;
        let vbs_tol = reltol * self.vbs.abs().max(self.vbs_prev.abs()) + voltage_tol;

        if !(vgs_diff < vgs_tol && vds_diff < vds_tol && vbs_diff < vbs_tol) {
            return false;
        }

        let drain_current_hat = self.id_prev
            + self.gm_prev * (self.eval_vgs - self.eval_vgs_prev)
            + self.gds_prev * (self.eval_vds - self.eval_vds_prev)
            + self.gmb_prev * (self.eval_vbs - self.eval_vbs_prev);
        let drain_current_tol = reltol * self.id.abs().max(drain_current_hat.abs()) + current_tol;
        if (drain_current_hat - self.id)
            .abs()
            .partial_cmp(&drain_current_tol)
            != Some(std::cmp::Ordering::Less)
        {
            return false;
        }

        let body_source_delta = self.body_source_diode_voltage(self.eval_vbs)
            - self.body_source_diode_voltage(self.eval_vbs_prev);
        let body_drain_delta = self.body_drain_diode_voltage(self.eval_vds, self.eval_vbs)
            - self.body_drain_diode_voltage(self.eval_vds_prev, self.eval_vbs_prev);
        let bulk_current = self.ibs + self.ibd;
        let bulk_current_hat = self.ibs_prev
            + self.ibd_prev
            + self.gbs_prev * body_source_delta
            + self.gbd_prev * body_drain_delta;
        let bulk_current_tol =
            reltol * bulk_current.abs().max(bulk_current_hat.abs()) + current_tol;

        (bulk_current_hat - bulk_current).abs() < bulk_current_tol
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_tuple_bits_eq(
        actual: (Value, Value, Value),
        expected: (Value, Value, Value),
        context: &str,
    ) {
        assert_eq!(
            [actual.0.to_bits(), actual.1.to_bits(), actual.2.to_bits()],
            [
                expected.0.to_bits(),
                expected.1.to_bits(),
                expected.2.to_bits(),
            ],
            "{context}"
        );
    }

    #[test]
    fn classic_transient_constants_match_canonical_models_and_polarities() {
        let candidates = [
            [0.0, 0.0, 0.0, 0.0],
            [0.25, 1.4, 0.0, -0.15],
            [-0.4, 0.3, 0.1, 0.35],
            [2.8, 3.1, -0.2, 0.6],
        ];

        for level in [1, 2, 3, 6] {
            for mut canonical in [
                Mosfet::new_nmos(format!("mn{level}"), 1, 2, 3, 4).with_level(level),
                Mosfet::new_pmos(format!("mp{level}"), 1, 2, 3, 4).with_level(level),
            ] {
                canonical.l = 1.3e-6;
                canonical.w = 7.1e-6;
                canonical.ld = 0.08e-6;
                canonical.kp = 93.0e-6;
                canonical.gamma = 0.42;
                canonical.phi = 0.67;
                canonical.lambda = 0.025;
                canonical.cox = 1.7e-3;
                canonical.cgso = 0.23e-9;
                canonical.cgdo = 0.31e-9;
                canonical.cgbo = 0.11e-9;
                canonical.is_bulk = 1.3e-14;
                canonical.js_bulk = 2.1e-5;
                canonical.source_area = 1.2e-12;
                canonical.drain_area = 1.8e-12;

                let mut prepared = canonical.clone();
                let constants = prepared.classic_transient_constants();
                assert_tuple_bits_eq(
                    prepared.overlap_capacitances_with_constants(&constants),
                    canonical.overlap_capacitances(),
                    "cached overlap capacitances must match the canonical evaluator",
                );

                for candidate in candidates {
                    canonical.update(&candidate);
                    prepared.update_with_classic_transient_constants(&candidate, &constants);
                    assert_eq!(
                        prepared.nonlinear_state_snapshot(),
                        canonical.nonlinear_state_snapshot(),
                        "cached nonlinear state differs for level {level}, {:?}, candidate {candidate:?}",
                        canonical.mos_type
                    );

                    let (vgs, vds, vbs) = canonical.cached_eval_branch_voltages().unwrap();
                    assert_tuple_bits_eq(
                        prepared
                            .transient_capacitance_halves_with_constants(vgs, vds, vbs, &constants),
                        canonical.transient_capacitance_halves_at(vgs, vds, vbs),
                        "cached Meyer capacitances must match the canonical evaluator",
                    );
                }
            }
        }
    }

    #[test]
    fn exact_physical_transient_update_matches_full_evaluation_and_tracks_gmin() {
        for level in [1, 2, 3, 6] {
            for mut cached in [
                Mosfet::new_nmos(format!("mn{level}"), 1, 2, 3, 4).with_level(level),
                Mosfet::new_pmos(format!("mp{level}"), 1, 2, 3, 4).with_level(level),
            ] {
                cached.is_bulk = 2.0e-14;
                cached.source_area = 1.0e-12;
                cached.drain_area = 1.5e-12;
                let constants = cached.classic_transient_constants();
                let candidate = [0.0, 0.0, 0.0, 0.0];

                cached.update_with_classic_transient_constants(&candidate, &constants);
                assert!(cached.cached_linearization_is_physical());
                assert!(cached.linearization_cache_valid);

                let mut full = cached.clone();
                full.linearization_cache_valid = false;
                cached.update_with_classic_transient_constants(&candidate, &constants);
                full.update_with_classic_transient_constants(&candidate, &constants);
                assert_eq!(
                    cached.nonlinear_state_snapshot(),
                    full.nonlinear_state_snapshot(),
                    "exact cache reuse differs from full evaluation for level {level}, {:?}",
                    cached.mos_type
                );

                let old_gbs = cached.gbs;
                cached.set_junction_gmin(1.0e-6);
                assert!(!cached.linearization_cache_valid);
                cached.update_with_classic_transient_constants(&candidate, &constants);
                assert_ne!(cached.gbs.to_bits(), old_gbs.to_bits());
                assert!(cached.linearization_cache_valid);

                cached.set_junction_gmin(0.0);
                assert!(!cached.linearization_cache_valid);
                let mut full = cached.clone();
                full.linearization_cache_valid = false;
                cached.update_with_classic_transient_constants(&candidate, &constants);
                full.update_with_classic_transient_constants(&candidate, &constants);
                assert_eq!(
                    cached.nonlinear_state_snapshot(),
                    full.nonlinear_state_snapshot(),
                    "returning to the configured GMIN must not reuse an intermediate rescue cache"
                );
            }
        }
    }

    #[test]
    fn classic_mos_convergence_fails_closed_for_every_nonfinite_input() {
        type Corrupt = fn(&mut Mosfet, Value);
        let corruptions: [Corrupt; 25] = [
            |mos, value| mos.vgs = value,
            |mos, value| mos.vgs_prev = value,
            |mos, value| mos.vds = value,
            |mos, value| mos.vds_prev = value,
            |mos, value| mos.vbs = value,
            |mos, value| mos.vbs_prev = value,
            |mos, value| mos.eval_vgs = value,
            |mos, value| mos.eval_vgs_prev = value,
            |mos, value| mos.eval_vds = value,
            |mos, value| mos.eval_vds_prev = value,
            |mos, value| mos.eval_vbs = value,
            |mos, value| mos.eval_vbs_prev = value,
            |mos, value| mos.id = value,
            |mos, value| mos.id_prev = value,
            |mos, value| mos.gm_prev = value,
            |mos, value| mos.gds_prev = value,
            |mos, value| mos.gmb_prev = value,
            |mos, value| mos.ibs = value,
            |mos, value| mos.gbs = value,
            |mos, value| mos.ibd = value,
            |mos, value| mos.gbd = value,
            |mos, value| mos.ibs_prev = value,
            |mos, value| mos.gbs_prev = value,
            |mos, value| mos.ibd_prev = value,
            |mos, value| mos.gbd_prev = value,
        ];
        let candidate = [1.2, 1.8, 0.2, -0.1];
        let mut converged = Mosfet::new_nmos("m1".to_string(), 1, 2, 3, 4);
        converged.update(&candidate);
        converged.update(&candidate);
        assert!(converged.is_converged(NonlinearConvergenceCriteria::default()));

        for (index, corrupt) in corruptions.into_iter().enumerate() {
            for value in [Value::NAN, Value::INFINITY, Value::NEG_INFINITY] {
                let mut mos = converged.clone();
                corrupt(&mut mos, value);
                assert!(
                    !mos.is_converged(NonlinearConvergenceCriteria::default()),
                    "non-finite convergence field {index} must fail closed for {value:?}"
                );
            }
        }
    }
}
