#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_18_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq18_e891_q: f64 = s.v[239];
        let eq18_e892: f64 = (s.v[212] * s.v[239]);
        let eq18_e892_d_n0: f64 = ((s.dn[212][0] * s.v[239]) + (s.v[212] * s.dn[239][0]));
        let eq18_e892_d_n1: f64 = ((s.dn[212][1] * s.v[239]) + (s.v[212] * s.dn[239][1]));
        let eq18_e892_d_n2: f64 = ((s.dn[212][2] * s.v[239]) + (s.v[212] * s.dn[239][2]));
        let eq18_e892_d_n3: f64 = ((s.dn[212][3] * s.v[239]) + (s.v[212] * s.dn[239][3]));
        let eq18_e892_d_n4: f64 = ((s.dn[212][4] * s.v[239]) + (s.v[212] * s.dn[239][4]));
        let eq18_e892_d_n5: f64 = ((s.dn[212][5] * s.v[239]) + (s.v[212] * s.dn[239][5]));
        let eq18_e892_d_n6: f64 = ((s.dn[212][6] * s.v[239]) + (s.v[212] * s.dn[239][6]));
        let eq18_e892_d_n7: f64 = ((s.dn[212][7] * s.v[239]) + (s.v[212] * s.dn[239][7]));
        let eq18_e892_d_n8: f64 = ((s.dn[212][8] * s.v[239]) + (s.v[212] * s.dn[239][8]));
        let eq18_e892_q: f64 = (s.v[212] * eq18_e891_q);
        let eq18_e892_q_d_n0: f64 = ((s.dn[212][0] * eq18_e891_q) + (s.v[212] * s.dn[239][0]));
        let eq18_e892_q_d_n1: f64 = ((s.dn[212][1] * eq18_e891_q) + (s.v[212] * s.dn[239][1]));
        let eq18_e892_q_d_n2: f64 = ((s.dn[212][2] * eq18_e891_q) + (s.v[212] * s.dn[239][2]));
        let eq18_e892_q_d_n3: f64 = ((s.dn[212][3] * eq18_e891_q) + (s.v[212] * s.dn[239][3]));
        let eq18_e892_q_d_n4: f64 = ((s.dn[212][4] * eq18_e891_q) + (s.v[212] * s.dn[239][4]));
        let eq18_e892_q_d_n5: f64 = ((s.dn[212][5] * eq18_e891_q) + (s.v[212] * s.dn[239][5]));
        let eq18_e892_q_d_n6: f64 = ((s.dn[212][6] * eq18_e891_q) + (s.v[212] * s.dn[239][6]));
        let eq18_e892_q_d_n7: f64 = ((s.dn[212][7] * eq18_e891_q) + (s.v[212] * s.dn[239][7]));
        let eq18_e892_q_d_n8: f64 = ((s.dn[212][8] * eq18_e891_q) + (s.v[212] * s.dn[239][8]));
        let eq18_reactive_node_derivatives: [f64; 9] = [eq18_e892_q_d_n0, eq18_e892_q_d_n1, eq18_e892_q_d_n2, eq18_e892_q_d_n3, eq18_e892_q_d_n4, eq18_e892_q_d_n5, eq18_e892_q_d_n6, eq18_e892_q_d_n7, eq18_e892_q_d_n8];
        let eq18_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[3]),
            &nodes,
            &eq18_reactive_node_derivatives,
            &branches,
            &eq18_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_41_block_0(
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
        let (eq41_e1106, eq41_e1106_d_n0, eq41_e1106_d_n1, eq41_e1106_d_n2, eq41_e1106_d_n3, eq41_e1106_d_n4, eq41_e1106_d_n5, eq41_e1106_d_n6, eq41_e1106_d_n7, eq41_e1106_d_n8, eq41_e1106_q, eq41_e1106_q_d_n0, eq41_e1106_q_d_n1, eq41_e1106_q_d_n2, eq41_e1106_q_d_n3, eq41_e1106_q_d_n4, eq41_e1106_q_d_n5, eq41_e1106_q_d_n6, eq41_e1106_q_d_n7, eq41_e1106_q_d_n8,) = {
    if (s.v[671] != 0.0) {
        let eq41_e1103: f64 = ((nv4 - 0.0) * s.v[270]);
        let eq41_e1103_d_n0: f64 = ((nv4 - 0.0) * s.dn[270][0]);
        let eq41_e1103_d_n1: f64 = ((nv4 - 0.0) * s.dn[270][1]);
        let eq41_e1103_d_n2: f64 = ((nv4 - 0.0) * s.dn[270][2]);
        let eq41_e1103_d_n3: f64 = ((nv4 - 0.0) * s.dn[270][3]);
        let eq41_e1103_d_n4: f64 = (s.v[270] + ((nv4 - 0.0) * s.dn[270][4]));
        let eq41_e1103_d_n5: f64 = ((nv4 - 0.0) * s.dn[270][5]);
        let eq41_e1103_d_n6: f64 = ((nv4 - 0.0) * s.dn[270][6]);
        let eq41_e1103_d_n7: f64 = ((nv4 - 0.0) * s.dn[270][7]);
        let eq41_e1103_d_n8: f64 = ((nv4 - 0.0) * s.dn[270][8]);
        let eq41_e1104_q: f64 = eq41_e1103;
        (eq41_e1103, eq41_e1103_d_n0, eq41_e1103_d_n1, eq41_e1103_d_n2, eq41_e1103_d_n3, eq41_e1103_d_n4, eq41_e1103_d_n5, eq41_e1103_d_n6, eq41_e1103_d_n7, eq41_e1103_d_n8, eq41_e1104_q, eq41_e1103_d_n0, eq41_e1103_d_n1, eq41_e1103_d_n2, eq41_e1103_d_n3, eq41_e1103_d_n4, eq41_e1103_d_n5, eq41_e1103_d_n6, eq41_e1103_d_n7, eq41_e1103_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_reactive_node_derivatives: [f64; 9] = [eq41_e1106_q_d_n0, eq41_e1106_q_d_n1, eq41_e1106_q_d_n2, eq41_e1106_q_d_n3, eq41_e1106_q_d_n4, eq41_e1106_q_d_n5, eq41_e1106_q_d_n6, eq41_e1106_q_d_n7, eq41_e1106_q_d_n8];
        let eq41_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            &nodes,
            &eq41_reactive_node_derivatives,
            &branches,
            &eq41_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
