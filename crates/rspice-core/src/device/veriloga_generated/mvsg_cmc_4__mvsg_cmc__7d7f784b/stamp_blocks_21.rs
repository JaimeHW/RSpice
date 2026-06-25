#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_177_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq177_e1891_q: f64 = s.v[219];
        let eq177_reactive_node_derivatives: [f64; 30] = [s.dn[219][0], s.dn[219][1], s.dn[219][2], s.dn[219][3], s.dn[219][4], s.dn[219][5], s.dn[219][6], s.dn[219][7], s.dn[219][8], s.dn[219][9], s.dn[219][10], s.dn[219][11], s.dn[219][12], s.dn[219][13], s.dn[219][14], s.dn[219][15], s.dn[219][16], s.dn[219][17], s.dn[219][18], s.dn[219][19], s.dn[219][20], s.dn[219][21], s.dn[219][22], s.dn[219][23], s.dn[219][24], s.dn[219][25], s.dn[219][26], s.dn[219][27], s.dn[219][28], s.dn[219][29]];
        let eq177_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[3]),
            &nodes,
            &eq177_reactive_node_derivatives,
            &branches,
            &eq177_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_194_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq194_e2167, eq194_e2167_d_n4, eq194_e2167_q, eq194_e2167_q_d_n4,) = {
    if (s.v[2700] != 0.0) {
        let eq194_e2164: f64 = (p.p321 * (nv4 - 0.0));
        let eq194_e2164_d_n4: f64 = p.p321;
        let eq194_e2165_q: f64 = eq194_e2164;
        (eq194_e2164, eq194_e2164_d_n4, eq194_e2165_q, eq194_e2164_d_n4,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[4]),
            None,
            &[
                GeneratedDerivative::node(nodes[4], self.multiplicity * (eq194_e2167_q_d_n4)),
            ],
        );
    }
}
