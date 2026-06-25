#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_11_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq11_e124_q: f64 = s.v[3];
        let eq11_reactive_node_derivatives: [f64; 7] = [s.dn[3][0], s.dn[3][1], s.dn[3][2], s.dn[3][3], s.dn[3][4], s.dn[3][5], s.dn[3][6]];
        let eq11_reactive_branch_derivatives: [f64; 4] = [s.db[3][0], s.db[3][1], s.db[3][2], s.db[3][3]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[5]),
            &nodes,
            &eq11_reactive_node_derivatives,
            &branches,
            &eq11_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_13_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq13_e128_q: f64 = s.v[105];
        let eq13_reactive_node_derivatives: [f64; 7] = [s.dn[105][0], s.dn[105][1], s.dn[105][2], s.dn[105][3], s.dn[105][4], s.dn[105][5], s.dn[105][6]];
        let eq13_reactive_branch_derivatives: [f64; 4] = [s.db[105][0], s.db[105][1], s.db[105][2], s.db[105][3]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            &nodes,
            &eq13_reactive_node_derivatives,
            &branches,
            &eq13_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_14_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq14_e130_q: f64 = s.v[106];
        let eq14_reactive_node_derivatives: [f64; 7] = [s.dn[106][0], s.dn[106][1], s.dn[106][2], s.dn[106][3], s.dn[106][4], s.dn[106][5], s.dn[106][6]];
        let eq14_reactive_branch_derivatives: [f64; 4] = [s.db[106][0], s.db[106][1], s.db[106][2], s.db[106][3]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[1]),
            &nodes,
            &eq14_reactive_node_derivatives,
            &branches,
            &eq14_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
