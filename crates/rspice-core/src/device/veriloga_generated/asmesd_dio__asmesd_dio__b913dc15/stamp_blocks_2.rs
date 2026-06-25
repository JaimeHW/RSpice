#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_12_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq12_e156, eq12_e156_d_n5, eq12_e156_q, eq12_e156_q_d_n5,) = {
    if ((!(s.v[70] != 0.0)) && (s.v[71] != 0.0)) {
        let eq12_e153: f64 = (p.p36 * (nv5 - 0.0));
        let eq12_e153_d_n5: f64 = p.p36;
        let eq12_e154_q: f64 = eq12_e153;
        (eq12_e153, eq12_e153_d_n5, eq12_e154_q, eq12_e153_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[5]),
            None,
            &[
                GeneratedDerivative::node(nodes[5], self.multiplicity * (eq12_e156_q_d_n5)),
            ],
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
        let eq25_e269: f64 = (s.v[9] * s.v[33]);
        let eq25_e269_d_n0: f64 = (s.v[9] * s.dn[33][0]);
        let eq25_e269_d_n1: f64 = (s.v[9] * s.dn[33][1]);
        let eq25_e269_d_n2: f64 = (s.v[9] * s.dn[33][2]);
        let eq25_e269_d_n3: f64 = (s.v[9] * s.dn[33][3]);
        let eq25_e269_d_n4: f64 = (s.v[9] * s.dn[33][4]);
        let eq25_e269_d_n5: f64 = (s.v[9] * s.dn[33][5]);
        let eq25_e269_d_n6: f64 = (s.v[9] * s.dn[33][6]);
        let eq25_e269_d_b0: f64 = (s.v[9] * s.db[33][0]);
        let eq25_e269_d_b1: f64 = (s.v[9] * s.db[33][1]);
        let eq25_e269_d_b2: f64 = (s.v[9] * s.db[33][2]);
        let eq25_e269_d_b3: f64 = (s.v[9] * s.db[33][3]);
        let eq25_e269_d_b4: f64 = (s.v[9] * s.db[33][4]);
        let eq25_e269_d_b5: f64 = (s.v[9] * s.db[33][5]);
        let eq25_e269_d_b6: f64 = (s.v[9] * s.db[33][6]);
        let eq25_e271: f64 = (eq25_e269 * s.v[3]);
        let eq25_e271_d_n0: f64 = (eq25_e269_d_n0 * s.v[3]);
        let eq25_e271_d_n1: f64 = (eq25_e269_d_n1 * s.v[3]);
        let eq25_e271_d_n2: f64 = (eq25_e269_d_n2 * s.v[3]);
        let eq25_e271_d_n3: f64 = (eq25_e269_d_n3 * s.v[3]);
        let eq25_e271_d_n4: f64 = (eq25_e269_d_n4 * s.v[3]);
        let eq25_e271_d_n5: f64 = (eq25_e269_d_n5 * s.v[3]);
        let eq25_e271_d_n6: f64 = (eq25_e269_d_n6 * s.v[3]);
        let eq25_e271_d_b0: f64 = (eq25_e269_d_b0 * s.v[3]);
        let eq25_e271_d_b1: f64 = (eq25_e269_d_b1 * s.v[3]);
        let eq25_e271_d_b2: f64 = (eq25_e269_d_b2 * s.v[3]);
        let eq25_e271_d_b3: f64 = (eq25_e269_d_b3 * s.v[3]);
        let eq25_e271_d_b4: f64 = (eq25_e269_d_b4 * s.v[3]);
        let eq25_e271_d_b5: f64 = (eq25_e269_d_b5 * s.v[3]);
        let eq25_e271_d_b6: f64 = (eq25_e269_d_b6 * s.v[3]);
        let eq25_e272_q: f64 = eq25_e271;
        let eq25_reactive_node_derivatives: [f64; 7] = [eq25_e271_d_n0, eq25_e271_d_n1, eq25_e271_d_n2, eq25_e271_d_n3, eq25_e271_d_n4, eq25_e271_d_n5, eq25_e271_d_n6];
        let eq25_reactive_branch_derivatives: [f64; 7] = [eq25_e271_d_b0, eq25_e271_d_b1, eq25_e271_d_b2, eq25_e271_d_b3, eq25_e271_d_b4, eq25_e271_d_b5, eq25_e271_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[4]),
            &nodes,
            &eq25_reactive_node_derivatives,
            &branches,
            &eq25_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_26_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq26_e275: f64 = (s.v[9] * s.v[32]);
        let eq26_e275_d_n0: f64 = (s.v[9] * s.dn[32][0]);
        let eq26_e275_d_n1: f64 = (s.v[9] * s.dn[32][1]);
        let eq26_e275_d_n2: f64 = (s.v[9] * s.dn[32][2]);
        let eq26_e275_d_n3: f64 = (s.v[9] * s.dn[32][3]);
        let eq26_e275_d_n4: f64 = (s.v[9] * s.dn[32][4]);
        let eq26_e275_d_n5: f64 = (s.v[9] * s.dn[32][5]);
        let eq26_e275_d_n6: f64 = (s.v[9] * s.dn[32][6]);
        let eq26_e275_d_b0: f64 = (s.v[9] * s.db[32][0]);
        let eq26_e275_d_b1: f64 = (s.v[9] * s.db[32][1]);
        let eq26_e275_d_b2: f64 = (s.v[9] * s.db[32][2]);
        let eq26_e275_d_b3: f64 = (s.v[9] * s.db[32][3]);
        let eq26_e275_d_b4: f64 = (s.v[9] * s.db[32][4]);
        let eq26_e275_d_b5: f64 = (s.v[9] * s.db[32][5]);
        let eq26_e275_d_b6: f64 = (s.v[9] * s.db[32][6]);
        let eq26_e277: f64 = (eq26_e275 * s.v[3]);
        let eq26_e277_d_n0: f64 = (eq26_e275_d_n0 * s.v[3]);
        let eq26_e277_d_n1: f64 = (eq26_e275_d_n1 * s.v[3]);
        let eq26_e277_d_n2: f64 = (eq26_e275_d_n2 * s.v[3]);
        let eq26_e277_d_n3: f64 = (eq26_e275_d_n3 * s.v[3]);
        let eq26_e277_d_n4: f64 = (eq26_e275_d_n4 * s.v[3]);
        let eq26_e277_d_n5: f64 = (eq26_e275_d_n5 * s.v[3]);
        let eq26_e277_d_n6: f64 = (eq26_e275_d_n6 * s.v[3]);
        let eq26_e277_d_b0: f64 = (eq26_e275_d_b0 * s.v[3]);
        let eq26_e277_d_b1: f64 = (eq26_e275_d_b1 * s.v[3]);
        let eq26_e277_d_b2: f64 = (eq26_e275_d_b2 * s.v[3]);
        let eq26_e277_d_b3: f64 = (eq26_e275_d_b3 * s.v[3]);
        let eq26_e277_d_b4: f64 = (eq26_e275_d_b4 * s.v[3]);
        let eq26_e277_d_b5: f64 = (eq26_e275_d_b5 * s.v[3]);
        let eq26_e277_d_b6: f64 = (eq26_e275_d_b6 * s.v[3]);
        let eq26_e278_q: f64 = eq26_e277;
        let eq26_reactive_node_derivatives: [f64; 7] = [eq26_e277_d_n0, eq26_e277_d_n1, eq26_e277_d_n2, eq26_e277_d_n3, eq26_e277_d_n4, eq26_e277_d_n5, eq26_e277_d_n6];
        let eq26_reactive_branch_derivatives: [f64; 7] = [eq26_e277_d_b0, eq26_e277_d_b1, eq26_e277_d_b2, eq26_e277_d_b3, eq26_e277_d_b4, eq26_e277_d_b5, eq26_e277_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[4]),
            &nodes,
            &eq26_reactive_node_derivatives,
            &branches,
            &eq26_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
