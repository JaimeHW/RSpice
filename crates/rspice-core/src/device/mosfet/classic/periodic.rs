//! Physical classic MOS periodic equations, retaining native Meyer storage.
use super::*;

// Fixed units conversion, independent of carrier frequency (including in
// autonomous solves). Each auxiliary voltage is tau * dVbranch/dt.
const RATE_TIME_SCALE: Value = 1e-9;

#[cfg(test)]
mod tests;

impl Mosfet {
    pub(crate) fn periodic_coupling_nodes(&self) -> [NodeId; 4] {
        [
            self.node_drain,
            self.node_gate,
            self.node_source,
            self.node_bulk,
        ]
    }

    pub(crate) fn periodic_rate_names(&self) -> Option<[String; 3]> {
        (!self.uses_legacy_bsim())
            .then(|| ["gs", "gd", "gb"].map(|port| format!("{}.__meyer_rate_{port}", self.name)))
    }

    pub(crate) fn update_periodic_noise_probe(&mut self, solution: &[Value]) {
        let (vgs, vds, vbs) = self.unlimited_branch_voltages_at(solution);
        let (id, region, gm, gds, gmb, gss, ieq) = self.linearized_operating_point(vgs, vds, vbs);
        self.eval_vgs = vgs;
        self.eval_vds = vds;
        self.eval_vbs = vbs;
        self.id = id;
        self.region = region;
        self.gm = gm;
        self.gds = gds;
        self.gmb = gmb;
        self.gss = gss;
        self.id_eq = ieq;
        (self.ibs, self.gbs) = self.body_source_junction_current_and_conductance(vbs);
        (self.ibd, self.gbd) = self.body_drain_junction_current_and_conductance(vds, vbs);
        self.linearization_cache_valid = false;
    }

    fn periodic_gate_rates(
        &self,
        solution: &[Value],
        rate_nodes: Option<[NodeId; 3]>,
    ) -> Result<[Value; 3], String> {
        if self.uses_legacy_bsim() {
            if rate_nodes.is_none() {
                return Ok([0.0; 3]);
            }
        } else if let Some(nodes) = rate_nodes {
            if nodes.iter().all(|&node| node > 0 && node <= solution.len()) {
                return Ok(nodes.map(|node| solution[node - 1] / RATE_TIME_SCALE));
            }
        }
        Err(format!(
            "MOSFET '{}': invalid periodic Meyer rate coordinates",
            self.name
        ))
    }

    /// Stamps -F, -Q on the RHS and their positive derivatives, using raw
    /// physical bias without modifying the transient limiter or history.
    pub(crate) fn stamp_periodic_physical_fq(
        &self,
        solution: &[Value],
        rate_nodes: Option<[NodeId; 3]>,
        f: &mut impl MatrixStamper,
        q: &mut impl MatrixStamper,
    ) -> Result<(), String> {
        let rates = self.periodic_gate_rates(solution, rate_nodes)?;
        let (vgs, vds, vbs) = self.unlimited_branch_voltages_at(solution);
        let (id, _, gm, gds, gmb, gss, ieq) = self.linearized_operating_point(vgs, vds, vbs);
        let (gm, gds, gmb, gss, _) =
            Self::channel_stamp_terms(self.periodic_coupling_nodes(), gm, gds, gmb, gss, ieq);
        branch(
            f,
            self.node_drain,
            self.node_source,
            id,
            [
                (self.node_gate, gm),
                (self.node_drain, gds),
                (self.node_bulk, gmb),
                (self.node_source, -gss),
            ],
        );
        let (ibs, gbs) = self.body_source_junction_current_and_conductance(vbs);
        let (ibd, gbd) = self.body_drain_junction_current_and_conductance(vds, vbs);
        let (qbs, cbs) = self.body_source_junction_charge_and_capacitance_at(vbs);
        let (qbd, cbd) = self.body_drain_junction_charge_and_capacitance_at(vds, vbs);
        for ((pos, neg), current, conductance, charge, capacitance) in [
            (self.body_source_charge_nodes(), ibs, gbs, qbs, cbs),
            (self.body_drain_charge_nodes(), ibd, gbd, qbd, cbd),
        ] {
            branch(
                f,
                pos,
                neg,
                current,
                [(pos, conductance), (neg, -conductance)],
            );
            branch(
                q,
                pos,
                neg,
                charge,
                [(pos, capacitance), (neg, -capacitance)],
            );
        }
        if let Some(charge) = self.legacy_gate_charge_at(vgs, vds, vbs) {
            for (i, negative) in [self.node_source, self.node_drain, self.node_bulk]
                .into_iter()
                .enumerate()
            {
                branch(
                    q,
                    self.node_gate,
                    negative,
                    charge.charges[i],
                    self.physical_control_gradient(charge.derivatives[i]),
                );
            }
        } else {
            let nodes = rate_nodes.expect("validated Meyer coordinates");
            let (caps, gradients) = self.meyer_capacitances_with_derivatives(vgs, vds, vbs);
            let (gs, gd, gb) = self.overlap_capacitances();
            for (i, (negative, voltage, overlap)) in [
                (self.node_source, vgs, gs),
                (self.node_drain, vgs - vds, gd),
                (self.node_bulk, vgs - vbs, gb),
            ]
            .into_iter()
            .enumerate()
            {
                let rate = nodes[i];
                let derivative =
                    self.physical_control_gradient(gradients[i].map(|dc| dc * rates[i]));
                branch(f, self.node_gate, negative, caps[i] * rates[i], derivative);
                f.stamp(self.node_gate, rate, caps[i] / RATE_TIME_SCALE);
                f.stamp(negative, rate, -caps[i] / RATE_TIME_SCALE);
                branch(
                    q,
                    self.node_gate,
                    negative,
                    overlap * voltage,
                    [(self.node_gate, overlap), (negative, -overlap)],
                );
                // F + dQ/dt = z - tau*dV/dt = 0.
                f.stamp_rhs(rate, -solution[rate - 1]);
                f.stamp(rate, rate, 1.0);
                q.stamp_rhs(rate, RATE_TIME_SCALE * voltage);
                q.stamp(rate, self.node_gate, -RATE_TIME_SCALE);
                q.stamp(rate, negative, RATE_TIME_SCALE);
            }
        }
        Ok(())
    }

    fn physical_control_gradient(&self, c: [Value; 3]) -> [(NodeId, Value); 4] {
        [
            (self.node_gate, c[0]),
            (self.node_drain, c[1]),
            (self.node_bulk, c[2]),
            (self.node_source, -(c[0] + c[1] + c[2])),
        ]
    }

    /// Separate D/G/S/B lead quantities even when authored terminals coincide.
    /// Meyer displacement is in F; only integrable storage is returned in Q.
    pub(crate) fn periodic_terminal_fq(
        &self,
        solution: &[Value],
        rate_nodes: Option<[NodeId; 3]>,
    ) -> Result<([Value; 4], [Value; 4]), String> {
        let rates = self.periodic_gate_rates(solution, rate_nodes)?;
        let (vgs, vds, vbs) = self.unlimited_branch_voltages_at(solution);
        let (id, _, _, _, _, _, _) = self.linearized_operating_point(vgs, vds, vbs);
        let (ibs, _) = self.body_source_junction_current_and_conductance(vbs);
        let (ibd, _) = self.body_drain_junction_current_and_conductance(vds, vbs);
        let (qbs, _) = self.body_source_junction_charge_and_capacitance_at(vbs);
        let (qbd, _) = self.body_drain_junction_charge_and_capacitance_at(vds, vbs);
        let (gate_current, gate_charge) = if let Some(q) = self.legacy_gate_charge_at(vgs, vds, vbs)
        {
            ([0.0; 3], q.charges)
        } else {
            let (caps, _) = self.meyer_capacitances_with_derivatives(vgs, vds, vbs);
            let (gs, gd, gb) = self.overlap_capacitances();
            (
                std::array::from_fn(|i| caps[i] * rates[i]),
                [gs * vgs, gd * (vgs - vds), gb * (vgs - vbs)],
            )
        };
        Ok((
            self.report_branch_currents(
                id,
                ibs,
                ibd,
                [gate_current[0], gate_current[1], gate_current[2], 0.0, 0.0],
            ),
            self.report_branch_currents(
                0.0,
                0.0,
                0.0,
                [gate_charge[0], gate_charge[1], gate_charge[2], qbs, qbd],
            ),
        ))
    }
}

fn branch<const N: usize>(
    stamp: &mut impl MatrixStamper,
    pos: NodeId,
    neg: NodeId,
    value: Value,
    gradient: [(NodeId, Value); N],
) {
    // Include zero contributions to keep sample ordering invariant.
    stamp.stamp_rhs(pos, -value);
    stamp.stamp_rhs(neg, value);
    for (column, derivative) in gradient {
        stamp.stamp(pos, column, derivative);
        stamp.stamp(neg, column, -derivative);
    }
}
