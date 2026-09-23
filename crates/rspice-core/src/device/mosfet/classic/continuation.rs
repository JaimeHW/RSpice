//! Native charge topology and consistent shooting-period initialization.
use super::*;

impl Mosfet {
    /// Potential storage ports, independent of the current operating region.
    /// Meyer storage is a capacitance law; no fictitious terminal charge is
    /// substituted for it when selecting the shooting voltage coordinates.
    pub(crate) fn shooting_terminal_storage_nodes(&self) -> [Option<(NodeId, NodeId)>; 5] {
        let intrinsic = self.uses_legacy_bsim() || self.oxide_capacitance_total() > 0.0;
        let (gs, gd, gb) = self.overlap_capacitances();
        let junctions = self.body_junction_charge_mask();
        [
            (intrinsic || gs != 0.0).then_some((self.node_gate, self.node_source)),
            (intrinsic || gd != 0.0).then_some((self.node_gate, self.node_drain)),
            (intrinsic || gb != 0.0).then_some((self.node_gate, self.node_bulk)),
            (junctions & 1 != 0).then_some(self.body_source_charge_nodes()),
            (junctions & 2 != 0).then_some(self.body_drain_charge_nodes()),
        ]
    }

    /// Instantaneous C(v) for the initial charge-rate consistency equations.
    /// Legacy BSIM uses its coupled dQ/dV; classic levels retain native Meyer
    /// capacitances, including their dialect and geometry dependence.
    pub(crate) fn stamp_shooting_initial_charge(
        &self,
        solution: &[Value],
        stamper: &mut impl MatrixStamper,
        physical_probe: bool,
    ) {
        let (vgs, vds, vbs) = if physical_probe {
            self.unlimited_branch_voltages_at(solution)
        } else {
            self.eval_branch_voltages_at(solution)
        };
        if let Some(charge) = self.legacy_gate_charge_at(vgs, vds, vbs) {
            self.stamp_legacy_gate_charge(&charge, 1.0, [0.0; 3], [0.0; 3], stamper);
        } else {
            let (gs, gd, gb) = self.ac_capacitances_at(vgs, vds, vbs);
            for (negative, capacitance) in [
                (self.node_source, gs),
                (self.node_drain, gd),
                (self.node_bulk, gb),
            ] {
                stamp_capacitance(stamper, self.node_gate, negative, capacitance);
            }
        }
        let (_, cs) = self.body_source_junction_charge_and_capacitance_at(vbs);
        let (_, cd) = self.body_drain_junction_charge_and_capacitance_at(vds, vbs);
        let (sp, sn) = self.body_source_charge_nodes();
        let (dp, dn) = self.body_drain_charge_nodes();
        stamp_capacitance(stamper, sp, sn, cs);
        stamp_capacitance(stamper, dp, dn, cd);
    }

    /// A shooting state is an accepted physical bias, with no preceding
    /// Newton limiter path or pending t=0 OFF seed to replay.
    pub(crate) fn seed_accepted_periodic_bias(&mut self, solution: &[Value]) {
        self.has_branch_history = false;
        self.initial_off_seed_pending = false;
        self.initial_off_seed_evaluations = 0;
        self.update(solution);
        // The second update aligns both convergence-reference generations.
        self.update(solution);
    }
}

fn stamp_capacitance(
    stamper: &mut impl MatrixStamper,
    pos: NodeId,
    neg: NodeId,
    capacitance: Value,
) {
    if pos != neg {
        stamper.stamp(pos, pos, capacitance);
        stamper.stamp(pos, neg, -capacitance);
        stamper.stamp(neg, pos, -capacitance);
        stamper.stamp(neg, neg, capacitance);
    }
}
