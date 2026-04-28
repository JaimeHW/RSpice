use super::*;

impl NonlinearDevice for Mosfet {
    fn update(&mut self, voltages: &[Value]) {
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
        (self.ibs_prev, self.gbs_prev) =
            self.body_source_junction_current_and_conductance(self.eval_vbs_prev);
        (self.ibd_prev, self.gbd_prev) = self
            .body_drain_junction_current_and_conductance(self.eval_vds_prev, self.eval_vbs_prev);

        let (vgs, vds, vbs) = self.branch_voltages(voltages);
        let (eval_vgs, eval_vds, eval_vbs) = self.limited_branch_voltages_for_eval(vgs, vds, vbs);
        self.vgs = vgs;
        self.vds = vds;
        self.vbs = vbs;
        self.eval_vgs = eval_vgs;
        self.eval_vds = eval_vds;
        self.eval_vbs = eval_vbs;

        let (id, region, gm, gds, gmb, id_eq) =
            self.linearized_operating_point(self.eval_vgs, self.eval_vds, self.eval_vbs);
        self.id = id;
        self.region = region;
        self.gm = gm;
        self.gds = gds;
        self.gmb = gmb;
        self.id_eq = id_eq;
        self.has_branch_history = true;
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let (vgs, vds, vbs) = self.branch_voltages(voltages);
        let (eval_vgs, eval_vds, eval_vbs) = self.limited_branch_voltages_for_eval(vgs, vds, vbs);
        let (gm, gds, gmb, id_eq) = if self.cached_linearization_matches(vgs, vds, vbs) {
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

        let (bs_anode, bs_cathode, gbs, ieq_bs) = self.body_source_junction_linearization(eval_vbs);
        Self::stamp_diode_linearization(matrix, bs_anode, bs_cathode, gbs, ieq_bs);

        let (bd_anode, bd_cathode, gbd, ieq_bd) =
            self.body_drain_junction_linearization(eval_vds, eval_vbs);
        Self::stamp_diode_linearization(matrix, bd_anode, bd_cathode, gbd, ieq_bd);
    }

    fn is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        if !self.vgs.is_finite()
            || !self.vgs_prev.is_finite()
            || !self.vds.is_finite()
            || !self.vds_prev.is_finite()
            || !self.vbs.is_finite()
            || !self.vbs_prev.is_finite()
            || !self.eval_vgs.is_finite()
            || !self.eval_vgs_prev.is_finite()
            || !self.eval_vds.is_finite()
            || !self.eval_vds_prev.is_finite()
            || !self.eval_vbs.is_finite()
            || !self.eval_vbs_prev.is_finite()
            || !self.id.is_finite()
            || !self.id_prev.is_finite()
            || !self.gm_prev.is_finite()
            || !self.gds_prev.is_finite()
            || !self.gmb_prev.is_finite()
            || !self.ibs_prev.is_finite()
            || !self.gbs_prev.is_finite()
            || !self.ibd_prev.is_finite()
            || !self.gbd_prev.is_finite()
        {
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
        if (drain_current_hat - self.id).abs() >= drain_current_tol {
            return false;
        }

        let (ibs, _) = self.body_source_junction_current_and_conductance(self.eval_vbs);
        let (ibd, _) =
            self.body_drain_junction_current_and_conductance(self.eval_vds, self.eval_vbs);
        let body_source_delta = self.body_source_diode_voltage(self.eval_vbs)
            - self.body_source_diode_voltage(self.eval_vbs_prev);
        let body_drain_delta = self.body_drain_diode_voltage(self.eval_vds, self.eval_vbs)
            - self.body_drain_diode_voltage(self.eval_vds_prev, self.eval_vbs_prev);
        let bulk_current = ibs + ibd;
        let bulk_current_hat = self.ibs_prev
            + self.ibd_prev
            + self.gbs_prev * body_source_delta
            + self.gbd_prev * body_drain_delta;
        let bulk_current_tol =
            reltol * bulk_current.abs().max(bulk_current_hat.abs()) + current_tol;

        (bulk_current_hat - bulk_current).abs() < bulk_current_tol
    }
}
