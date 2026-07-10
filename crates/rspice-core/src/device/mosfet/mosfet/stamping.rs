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

        let (bs_anode, bs_cathode, gbs, ieq_bs) = self.body_source_junction_linearization(eval_vbs);
        Self::stamp_diode_linearization_direct(matrix, rhs, bs_anode, bs_cathode, gbs, ieq_bs);

        let (bd_anode, bd_cathode, gbd, ieq_bd) =
            self.body_drain_junction_linearization(eval_vds, eval_vbs);
        Self::stamp_diode_linearization_direct(matrix, rhs, bd_anode, bd_cathode, gbd, ieq_bd);
    }

    /// Stamp using O(1) direct indexing (call after link).
    pub fn stamp_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value], voltages: &[Value]) {
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

        self.stamp_direct_operating_point(matrix, rhs, eval_vds, eval_vbs, gm, gds, gmb, id_eq);
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
        let (_, _, gm, gds, gmb, id_eq) = self.linearized_operating_point(vgs, vds, vbs);
        self.stamp_direct_operating_point(matrix, rhs, vds, vbs, gm, gds, gmb, id_eq);
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
}
