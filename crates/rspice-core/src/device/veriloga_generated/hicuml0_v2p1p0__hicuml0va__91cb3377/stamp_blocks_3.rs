#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_23_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq23_e213_q: f64 = s.v[210];
        let eq23_reactive_node_derivatives: [f64; 10] = [s.dn[210][0], s.dn[210][1], s.dn[210][2], s.dn[210][3], s.dn[210][4], s.dn[210][5], s.dn[210][6], s.dn[210][7], s.dn[210][8], s.dn[210][9]];
        let eq23_reactive_branch_derivatives: [f64; 4] = [s.db[210][0], s.db[210][1], s.db[210][2], s.db[210][3]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            None,
            &nodes,
            &eq23_reactive_node_derivatives,
            &branches,
            &eq23_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_25_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq25_e216_q: f64 = s.v[213];
        let eq25_reactive_node_derivatives: [f64; 10] = [s.dn[213][0], s.dn[213][1], s.dn[213][2], s.dn[213][3], s.dn[213][4], s.dn[213][5], s.dn[213][6], s.dn[213][7], s.dn[213][8], s.dn[213][9]];
        let eq25_reactive_branch_derivatives: [f64; 4] = [s.db[213][0], s.db[213][1], s.db[213][2], s.db[213][3]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            None,
            &nodes,
            &eq25_reactive_node_derivatives,
            &branches,
            &eq25_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
