//! Native AC-only NQS with explicit relaxation coordinates.

use super::*;
use crate::device::mosfet::ac_nqs::AcNqsResponse;

struct Derivatives<'a, S>(&'a mut S);

impl<S: MatrixStamper> MatrixStamper for Derivatives<'_, S> {
    fn stamp(&mut self, row: usize, col: usize, value: Value) {
        self.0.stamp(row, col, value);
    }
    fn stamp_rhs(&mut self, _: usize, _: Value) {}
}

impl Bsim4v8Device {
    /// Complete ACNQSMOD=1 G+sC response at the raw bias. Three distinct
    /// auxiliary nodes outside the device topology must be registered by the
    /// caller. Gate/body resistance networks, leakage, NF, M and all overlap
    /// and junction charge remain in their native topology.
    pub fn stamp_ac_nqs_response(
        &self,
        solution: &[Value],
        auxiliary: [NodeId; 3],
        f: &mut impl MatrixStamper,
        q: &mut impl MatrixStamper,
    ) -> Result<(), String> {
        if !self.uses_ac_nqs() {
            return Err(format!(
                "BSIM4 '{}': AC NQS response requires ACNQSMOD=1",
                self.name
            ));
        }
        let physical = [
            self.node_drain_external,
            self.node_drain,
            self.node_gate_external,
            self.node_gate_mid,
            self.node_gate,
            self.node_source_external,
            self.node_source,
            self.node_bulk_external,
            self.node_bulk,
            self.node_drain_body,
            self.node_source_body,
            self.node_charge_deficit,
        ];
        if auxiliary.iter().enumerate().any(|(index, node)| {
            *node == 0 || physical.contains(node) || auxiliary[..index].contains(node)
        }) {
            return Err(format!(
                "BSIM4 '{}': invalid AC NQS auxiliary coordinates",
                self.name
            ));
        }
        let bias = self.raw_branch_voltages(solution);
        let junction_bias = self.raw_junction_bias(solution);
        let gate_mid_vgs = (self.core.model.rgate_mod == 3).then(|| {
            self.core.mtype
                * (Self::node_voltage(solution, self.node_gate_mid)
                    - Self::node_voltage(solution, self.node_source))
        });
        let op = self.core.eval_with_junction_and_gate_mid_bias(
            bias,
            junction_bias,
            gate_mid_vgs,
            self.gmin,
            true,
        )?;
        let charge = op.charge.as_ref().expect("charge-enabled evaluation");
        self.stamp_op(&op, bias, junction_bias, solution, &mut Derivatives(f));
        self.stamp_ac_charge_matrix(charge, op.mode, 1.0, q);
        self.stamp_trnqs_ac_charge_node_anchor_delta(|row, col, value| {
            f.stamp(row, col, value.re);
        });
        AcNqsResponse {
            model: "BSIM4",
            name: &self.name,
            terminals: [
                self.node_drain,
                self.node_gate,
                self.node_source,
                self.node_bulk,
            ],
            multiplier: self.multiplier,
            mode: op.mode,
            current: [op.gm, op.gmbs, op.gds],
            charge: [
                [charge.cggb, charge.cgdb, charge.cgsb],
                [charge.cbgb, charge.cbdb, charge.cbsb],
                [charge.cdgb, charge.cddb, charge.cdsb],
            ],
            relaxation_time: charge.taunet,
        }
        .stamp(auxiliary, f, q)
    }
}
