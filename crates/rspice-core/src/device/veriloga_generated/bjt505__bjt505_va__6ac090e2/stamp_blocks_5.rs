#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_32_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv11 = ctx.node_voltage(nodes[11]);
        let eq32_e394_q: f64 = (nv11 - 0.0);
        let eq32_e395: f64 = (s.v[330] * (nv11 - 0.0));
        let eq32_e395_d_n0: f64 = (s.dn[330][0] * (nv11 - 0.0));
        let eq32_e395_d_n1: f64 = (s.dn[330][1] * (nv11 - 0.0));
        let eq32_e395_d_n2: f64 = (s.dn[330][2] * (nv11 - 0.0));
        let eq32_e395_d_n3: f64 = (s.dn[330][3] * (nv11 - 0.0));
        let eq32_e395_d_n4: f64 = (s.dn[330][4] * (nv11 - 0.0));
        let eq32_e395_d_n5: f64 = (s.dn[330][5] * (nv11 - 0.0));
        let eq32_e395_d_n6: f64 = (s.dn[330][6] * (nv11 - 0.0));
        let eq32_e395_d_n7: f64 = (s.dn[330][7] * (nv11 - 0.0));
        let eq32_e395_d_n8: f64 = (s.dn[330][8] * (nv11 - 0.0));
        let eq32_e395_d_n9: f64 = (s.dn[330][9] * (nv11 - 0.0));
        let eq32_e395_d_n10: f64 = (s.dn[330][10] * (nv11 - 0.0));
        let eq32_e395_d_n11: f64 = ((s.dn[330][11] * (nv11 - 0.0)) + s.v[330]);
        let eq32_e395_q: f64 = (s.v[330] * eq32_e394_q);
        let eq32_e395_q_d_n0: f64 = (s.dn[330][0] * eq32_e394_q);
        let eq32_e395_q_d_n1: f64 = (s.dn[330][1] * eq32_e394_q);
        let eq32_e395_q_d_n2: f64 = (s.dn[330][2] * eq32_e394_q);
        let eq32_e395_q_d_n3: f64 = (s.dn[330][3] * eq32_e394_q);
        let eq32_e395_q_d_n4: f64 = (s.dn[330][4] * eq32_e394_q);
        let eq32_e395_q_d_n5: f64 = (s.dn[330][5] * eq32_e394_q);
        let eq32_e395_q_d_n6: f64 = (s.dn[330][6] * eq32_e394_q);
        let eq32_e395_q_d_n7: f64 = (s.dn[330][7] * eq32_e394_q);
        let eq32_e395_q_d_n8: f64 = (s.dn[330][8] * eq32_e394_q);
        let eq32_e395_q_d_n9: f64 = (s.dn[330][9] * eq32_e394_q);
        let eq32_e395_q_d_n10: f64 = (s.dn[330][10] * eq32_e394_q);
        let eq32_e395_q_d_n11: f64 = ((s.dn[330][11] * eq32_e394_q) + s.v[330]);
        let eq32_reactive_node_derivatives: [f64; 12] = [eq32_e395_q_d_n0, eq32_e395_q_d_n1, eq32_e395_q_d_n2, eq32_e395_q_d_n3, eq32_e395_q_d_n4, eq32_e395_q_d_n5, eq32_e395_q_d_n6, eq32_e395_q_d_n7, eq32_e395_q_d_n8, eq32_e395_q_d_n9, eq32_e395_q_d_n10, eq32_e395_q_d_n11];
        let eq32_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            &nodes,
            &eq32_reactive_node_derivatives,
            &branches,
            &eq32_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
