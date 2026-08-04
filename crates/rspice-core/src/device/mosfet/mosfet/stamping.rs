use super::*;

impl Mosfet {
    /// Link this device to a StaticMatrix for O(1) stamping
    pub fn link(&mut self, matrix: &StaticMatrix) {
        let d = self.node_drain;
        let g = self.node_gate;
        let s = self.node_source;
        let b = self.node_bulk;

        // Drain row (4 columns)
        if d > 0 {
            self.indices.dd = matrix.get_index(d - 1, d - 1);
        }
        if d > 0 && g > 0 {
            self.indices.dg = matrix.get_index(d - 1, g - 1);
        }
        if d > 0 && s > 0 {
            self.indices.ds = matrix.get_index(d - 1, s - 1);
        }
        if d > 0 && b > 0 {
            self.indices.db = matrix.get_index(d - 1, b - 1);
        }
        // Source row (4 columns)
        if s > 0 && d > 0 {
            self.indices.sd = matrix.get_index(s - 1, d - 1);
        }
        if s > 0 && g > 0 {
            self.indices.sg = matrix.get_index(s - 1, g - 1);
        }
        if s > 0 {
            self.indices.ss = matrix.get_index(s - 1, s - 1);
        }
        if s > 0 && b > 0 {
            self.indices.sb = matrix.get_index(s - 1, b - 1);
        }
    }

    fn stamp_direct_operating_point(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        eval_vds: Value,
        eval_vbs: Value,
        gm: Value,
        gds: Value,
        gmb: Value,
        id_eq: Value,
        cache_matches: bool,
    ) {
        // Stamp matrix using direct indexing
        // Drain row
        if let Some(idx) = self.indices.dd {
            matrix.stamp_direct(idx, gds);
        }
        if let Some(idx) = self.indices.dg {
            matrix.stamp_direct(idx, gm);
        }
        if let Some(idx) = self.indices.ds {
            matrix.stamp_direct(idx, -gm - gds - gmb);
        }
        if let Some(idx) = self.indices.db {
            matrix.stamp_direct(idx, gmb);
        }
        // Source row
        if let Some(idx) = self.indices.sd {
            matrix.stamp_direct(idx, -gds);
        }
        if let Some(idx) = self.indices.sg {
            matrix.stamp_direct(idx, -gm);
        }
        if let Some(idx) = self.indices.ss {
            matrix.stamp_direct(idx, gm + gds + gmb);
        }
        if let Some(idx) = self.indices.sb {
            matrix.stamp_direct(idx, -gmb);
        }

        // Stamp RHS
        if self.node_drain > 0 {
            rhs[self.node_drain - 1] -= id_eq;
        }
        if self.node_source > 0 {
            rhs[self.node_source - 1] += id_eq;
        }

        let (bs_anode, bs_cathode, gbs, ieq_bs) =
            self.body_source_junction_linearization_cached(eval_vbs, cache_matches);
        self.stamp_body_source_linearization_direct(matrix, rhs, bs_anode, bs_cathode, gbs, ieq_bs);

        let (bd_anode, bd_cathode, gbd, ieq_bd) =
            self.body_drain_junction_linearization_cached(eval_vds, eval_vbs, cache_matches);
        self.stamp_body_drain_linearization_direct(matrix, rhs, bd_anode, bd_cathode, gbd, ieq_bd);
    }

    /// Stamp using O(1) direct indexing (call after link).
    pub fn stamp_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value], voltages: &[Value]) {
        let (vgs, vds, vbs) = self.branch_voltages(voltages);
        let (eval_vgs, eval_vds, eval_vbs) = self.limited_branch_voltages_for_eval(vgs, vds, vbs);
        let cache_matches = self.cached_linearization_matches_eval(eval_vgs, eval_vds, eval_vbs);
        let (gm, gds, gmb, id_eq) = if cache_matches {
            (self.gm, self.gds, self.gmb, self.id_eq)
        } else {
            let (_, _, gm, gds, gmb, id_eq) =
                self.linearized_operating_point(eval_vgs, eval_vds, eval_vbs);
            (gm, gds, gmb, id_eq)
        };

        self.stamp_direct_operating_point(
            matrix,
            rhs,
            eval_vds,
            eval_vbs,
            gm,
            gds,
            gmb,
            id_eq,
            cache_matches,
        );
    }

    /// Stamp the Newton linearization already cached by [`NonlinearDevice::update`].
    ///
    /// The transient classic-MOS fast path proves that device state matches
    /// its current iterate before calling this, so reloading terminal voltages,
    /// rerunning limiting, and comparing the cached bias is redundant.
    pub(crate) fn stamp_cached_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value]) {
        debug_assert!(self.has_branch_history);
        self.stamp_direct_operating_point(
            matrix,
            rhs,
            self.eval_vds,
            self.eval_vbs,
            self.gm,
            self.gds,
            self.gmb,
            self.id_eq,
            true,
        );
    }

    /// Evaluate the true, unlimited static device at `solution` and add its
    /// linearized `A*x` and RHS contributions directly to row accumulators.
    ///
    /// Residual acceptance needs physical currents, not the Newton Jacobian.
    /// Keeping this pure avoids mutating limiter history while also avoiding
    /// sparse writes that would only be multiplied back by the same candidate.
    pub(crate) fn add_physical_static_residual_row_terms_at(
        &self,
        solution: &[Value],
        constants: &ClassicMosTransientConstants,
        row_ax: &mut [Value],
        row_rhs: &mut [Value],
    ) {
        let (vgs, vds, vbs) = self.branch_voltages(solution);
        let cache_matches = self.cached_linearization_matches_eval(vgs, vds, vbs);
        let (gm, gds, gmb, id_eq) = if cache_matches {
            (self.gm, self.gds, self.gmb, self.id_eq)
        } else {
            let (_, _, gm, gds, gmb, id_eq) =
                self.linearized_transient_operating_point(vgs, vds, vbs, constants);
            (gm, gds, gmb, id_eq)
        };
        let vd = Self::terminal_voltage(solution, self.node_drain);
        let vg = Self::terminal_voltage(solution, self.node_gate);
        let vs = Self::terminal_voltage(solution, self.node_source);
        let vb = Self::terminal_voltage(solution, self.node_bulk);
        let source_diagonal = gm + gds + gmb;

        if self.node_drain > 0 {
            let row = self.node_drain - 1;
            row_ax[row] += gds * vd;
            if self.node_gate > 0 {
                row_ax[row] += gm * vg;
            }
            if self.node_source > 0 {
                row_ax[row] -= source_diagonal * vs;
            }
            if self.node_bulk > 0 {
                row_ax[row] += gmb * vb;
            }
            row_rhs[row] -= id_eq;
        }
        if self.node_source > 0 {
            let row = self.node_source - 1;
            if self.node_drain > 0 {
                row_ax[row] -= gds * vd;
            }
            if self.node_gate > 0 {
                row_ax[row] -= gm * vg;
            }
            row_ax[row] += source_diagonal * vs;
            if self.node_bulk > 0 {
                row_ax[row] -= gmb * vb;
            }
            row_rhs[row] += id_eq;
        }

        let (bs_anode, bs_cathode, gbs, ieq_bs) =
            self.body_source_junction_linearization_cached(vbs, cache_matches);
        Self::add_diode_residual_row_terms(
            solution, row_ax, row_rhs, bs_anode, bs_cathode, gbs, ieq_bs,
        );
        let (bd_anode, bd_cathode, gbd, ieq_bd) =
            self.body_drain_junction_linearization_cached(vds, vbs, cache_matches);
        Self::add_diode_residual_row_terms(
            solution, row_ax, row_rhs, bd_anode, bd_cathode, gbd, ieq_bd,
        );
    }

    #[inline]
    fn add_diode_residual_row_terms(
        solution: &[Value],
        row_ax: &mut [Value],
        row_rhs: &mut [Value],
        anode: NodeId,
        cathode: NodeId,
        conductance: Value,
        equivalent_current: Value,
    ) {
        if conductance == 0.0 && equivalent_current == 0.0 {
            return;
        }
        let va = Self::terminal_voltage(solution, anode);
        let vc = Self::terminal_voltage(solution, cathode);
        if anode > 0 {
            let row = anode - 1;
            row_ax[row] += conductance * va;
            if cathode > 0 {
                row_ax[row] -= conductance * vc;
            }
            row_rhs[row] -= equivalent_current;
        }
        if cathode > 0 {
            let row = cathode - 1;
            if anode > 0 {
                row_ax[row] -= conductance * va;
            }
            row_ax[row] += conductance * vc;
            row_rhs[row] += equivalent_current;
        }
    }

    /// Stamp the physical equations directly at a static candidate.
    ///
    /// Newton voltage limiting is iteration history, not part of the device
    /// equations. Residual and line-search probes must therefore bypass it or
    /// an otherwise valid operating point can be rejected based on the path
    /// taken to reach it.
    pub(crate) fn stamp_static_probe_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        let (vgs, vds, vbs) = self.branch_voltages(voltages);
        let cache_matches = self.cached_linearization_matches_eval(vgs, vds, vbs);
        let (gm, gds, gmb, id_eq) = if cache_matches {
            (self.gm, self.gds, self.gmb, self.id_eq)
        } else {
            let (_, _, gm, gds, gmb, id_eq) = self.linearized_operating_point(vgs, vds, vbs);
            (gm, gds, gmb, id_eq)
        };
        self.stamp_direct_operating_point(
            matrix,
            rhs,
            vds,
            vbs,
            gm,
            gds,
            gmb,
            id_eq,
            cache_matches,
        );
    }

    /// Get polarity multiplier (+1 for NMOS, -1 for PMOS)
    pub fn polarity(&self) -> Value {
        match self.mos_type {
            MosType::Nmos => 1.0,
            MosType::Pmos => -1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn full_matrix(size: usize) -> StaticMatrix {
        let triplets: Vec<_> = (0..size)
            .flat_map(|row| (0..size).map(move |col| (row, col, 0.0)))
            .collect();
        StaticMatrix::from_triplets(size, size, &triplets).expect("full test matrix")
    }

    #[test]
    fn static_probe_bypasses_classic_mos_limiter_history() {
        let initial = [0.0, 0.0, 0.0, 0.0];
        let candidate = [0.0, 0.0, 5.0, 5.0];
        let mut history_mos = Mosfet::new_pmos("mp".to_string(), 1, 2, 3, 4);
        history_mos.update(&initial);
        history_mos.update(&candidate);

        let mut limited_matrix = full_matrix(4);
        history_mos.link(&limited_matrix);
        let mut limited_rhs = vec![0.0; 4];
        history_mos.stamp_direct(&mut limited_matrix, &mut limited_rhs, &candidate);

        let mut static_matrix = full_matrix(4);
        history_mos.link(&static_matrix);
        let mut static_rhs = vec![0.0; 4];
        history_mos.stamp_static_probe_direct(&mut static_matrix, &mut static_rhs, &candidate);

        let mut fresh_mos = Mosfet::new_pmos("mp".to_string(), 1, 2, 3, 4);
        let mut expected_matrix = full_matrix(4);
        fresh_mos.link(&expected_matrix);
        let mut expected_rhs = vec![0.0; 4];
        fresh_mos.stamp_direct(&mut expected_matrix, &mut expected_rhs, &candidate);

        assert_ne!(limited_rhs, static_rhs, "test bias must exercise limiting");
        assert_eq!(static_rhs, expected_rhs);
        assert_eq!(static_matrix.values_mut(), expected_matrix.values_mut());
    }

    #[test]
    fn cached_direct_stamp_matches_verified_cached_bias_exactly() {
        let candidate = [1.2, 1.8, 0.2, -0.1];
        let mut mos = Mosfet::new_nmos("mn".to_string(), 1, 2, 3, 4);
        mos.update(&candidate);

        let mut verified_matrix = full_matrix(4);
        mos.link(&verified_matrix);
        let mut verified_rhs = vec![0.0; 4];
        mos.stamp_direct(&mut verified_matrix, &mut verified_rhs, &candidate);

        let mut cached_matrix = full_matrix(4);
        mos.link(&cached_matrix);
        let mut cached_rhs = vec![0.0; 4];
        mos.stamp_cached_direct(&mut cached_matrix, &mut cached_rhs);

        assert_eq!(cached_rhs, verified_rhs);
        assert_eq!(cached_matrix.values_mut(), verified_matrix.values_mut());
    }

    #[test]
    fn physical_residual_rows_reuse_matching_device_cache_exactly() {
        let candidate = [1.2, 1.8, 0.2, -0.1];
        let mut cached_mos = Mosfet::new_nmos("cached".to_string(), 1, 2, 3, 4);
        let constants = cached_mos.classic_transient_constants();
        cached_mos.update_with_classic_transient_constants(&candidate, &constants);
        assert!(cached_mos.cached_linearization_is_physical());

        let mut cached_ax = vec![0.0; 4];
        let mut cached_rhs = vec![0.0; 4];
        cached_mos.add_physical_static_residual_row_terms_at(
            &candidate,
            &constants,
            &mut cached_ax,
            &mut cached_rhs,
        );

        let fresh_mos = Mosfet::new_nmos("fresh".to_string(), 1, 2, 3, 4);
        let mut evaluated_ax = vec![0.0; 4];
        let mut evaluated_rhs = vec![0.0; 4];
        fresh_mos.add_physical_static_residual_row_terms_at(
            &candidate,
            &constants,
            &mut evaluated_ax,
            &mut evaluated_rhs,
        );

        assert_eq!(cached_ax, evaluated_ax);
        assert_eq!(cached_rhs, evaluated_rhs);
    }
}
