#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_81_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq81_e2216_q: f64 = s.v[525];
        let eq81_e2217: f64 = (s.v[379] * s.v[525]);
        let eq81_e2217_d_n0: f64 = ((s.dn[379][0] * s.v[525]) + (s.v[379] * s.dn[525][0]));
        let eq81_e2217_d_n1: f64 = ((s.dn[379][1] * s.v[525]) + (s.v[379] * s.dn[525][1]));
        let eq81_e2217_d_n2: f64 = ((s.dn[379][2] * s.v[525]) + (s.v[379] * s.dn[525][2]));
        let eq81_e2217_d_n3: f64 = ((s.dn[379][3] * s.v[525]) + (s.v[379] * s.dn[525][3]));
        let eq81_e2217_d_n4: f64 = ((s.dn[379][4] * s.v[525]) + (s.v[379] * s.dn[525][4]));
        let eq81_e2217_d_n5: f64 = ((s.dn[379][5] * s.v[525]) + (s.v[379] * s.dn[525][5]));
        let eq81_e2217_d_n6: f64 = ((s.dn[379][6] * s.v[525]) + (s.v[379] * s.dn[525][6]));
        let eq81_e2217_d_n7: f64 = ((s.dn[379][7] * s.v[525]) + (s.v[379] * s.dn[525][7]));
        let eq81_e2217_d_n8: f64 = ((s.dn[379][8] * s.v[525]) + (s.v[379] * s.dn[525][8]));
        let eq81_e2217_d_n9: f64 = ((s.dn[379][9] * s.v[525]) + (s.v[379] * s.dn[525][9]));
        let eq81_e2217_d_n10: f64 = ((s.dn[379][10] * s.v[525]) + (s.v[379] * s.dn[525][10]));
        let eq81_e2217_d_n11: f64 = ((s.dn[379][11] * s.v[525]) + (s.v[379] * s.dn[525][11]));
        let eq81_e2217_d_n12: f64 = ((s.dn[379][12] * s.v[525]) + (s.v[379] * s.dn[525][12]));
        let eq81_e2217_d_n13: f64 = ((s.dn[379][13] * s.v[525]) + (s.v[379] * s.dn[525][13]));
        let eq81_e2217_q: f64 = (s.v[379] * eq81_e2216_q);
        let eq81_e2217_q_d_n0: f64 = ((s.dn[379][0] * eq81_e2216_q) + (s.v[379] * s.dn[525][0]));
        let eq81_e2217_q_d_n1: f64 = ((s.dn[379][1] * eq81_e2216_q) + (s.v[379] * s.dn[525][1]));
        let eq81_e2217_q_d_n2: f64 = ((s.dn[379][2] * eq81_e2216_q) + (s.v[379] * s.dn[525][2]));
        let eq81_e2217_q_d_n3: f64 = ((s.dn[379][3] * eq81_e2216_q) + (s.v[379] * s.dn[525][3]));
        let eq81_e2217_q_d_n4: f64 = ((s.dn[379][4] * eq81_e2216_q) + (s.v[379] * s.dn[525][4]));
        let eq81_e2217_q_d_n5: f64 = ((s.dn[379][5] * eq81_e2216_q) + (s.v[379] * s.dn[525][5]));
        let eq81_e2217_q_d_n6: f64 = ((s.dn[379][6] * eq81_e2216_q) + (s.v[379] * s.dn[525][6]));
        let eq81_e2217_q_d_n7: f64 = ((s.dn[379][7] * eq81_e2216_q) + (s.v[379] * s.dn[525][7]));
        let eq81_e2217_q_d_n8: f64 = ((s.dn[379][8] * eq81_e2216_q) + (s.v[379] * s.dn[525][8]));
        let eq81_e2217_q_d_n9: f64 = ((s.dn[379][9] * eq81_e2216_q) + (s.v[379] * s.dn[525][9]));
        let eq81_e2217_q_d_n10: f64 = ((s.dn[379][10] * eq81_e2216_q) + (s.v[379] * s.dn[525][10]));
        let eq81_e2217_q_d_n11: f64 = ((s.dn[379][11] * eq81_e2216_q) + (s.v[379] * s.dn[525][11]));
        let eq81_e2217_q_d_n12: f64 = ((s.dn[379][12] * eq81_e2216_q) + (s.v[379] * s.dn[525][12]));
        let eq81_e2217_q_d_n13: f64 = ((s.dn[379][13] * eq81_e2216_q) + (s.v[379] * s.dn[525][13]));
        let eq81_reactive_node_derivatives: [f64; 14] = [eq81_e2217_q_d_n0, eq81_e2217_q_d_n1, eq81_e2217_q_d_n2, eq81_e2217_q_d_n3, eq81_e2217_q_d_n4, eq81_e2217_q_d_n5, eq81_e2217_q_d_n6, eq81_e2217_q_d_n7, eq81_e2217_q_d_n8, eq81_e2217_q_d_n9, eq81_e2217_q_d_n10, eq81_e2217_q_d_n11, eq81_e2217_q_d_n12, eq81_e2217_q_d_n13];
        let eq81_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[6]),
            &nodes,
            &eq81_reactive_node_derivatives,
            &branches,
            &eq81_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
