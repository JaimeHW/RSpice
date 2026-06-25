#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_219_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq219_e2720_q: f64 = s.v[197];
        let eq219_e2721: f64 = (p.p7 * s.v[197]);
        let eq219_e2721_d_n0: f64 = (p.p7 * s.dn[197][0]);
        let eq219_e2721_d_n1: f64 = (p.p7 * s.dn[197][1]);
        let eq219_e2721_d_n2: f64 = (p.p7 * s.dn[197][2]);
        let eq219_e2721_d_n3: f64 = (p.p7 * s.dn[197][3]);
        let eq219_e2721_d_n4: f64 = (p.p7 * s.dn[197][4]);
        let eq219_e2721_d_n5: f64 = (p.p7 * s.dn[197][5]);
        let eq219_e2721_d_n6: f64 = (p.p7 * s.dn[197][6]);
        let eq219_e2721_d_n7: f64 = (p.p7 * s.dn[197][7]);
        let eq219_e2721_d_n8: f64 = (p.p7 * s.dn[197][8]);
        let eq219_e2721_d_n9: f64 = (p.p7 * s.dn[197][9]);
        let eq219_e2721_d_n10: f64 = (p.p7 * s.dn[197][10]);
        let eq219_e2721_d_n11: f64 = (p.p7 * s.dn[197][11]);
        let eq219_e2721_d_n12: f64 = (p.p7 * s.dn[197][12]);
        let eq219_e2721_d_n13: f64 = (p.p7 * s.dn[197][13]);
        let eq219_e2721_d_n14: f64 = (p.p7 * s.dn[197][14]);
        let eq219_e2721_d_n15: f64 = (p.p7 * s.dn[197][15]);
        let eq219_e2721_d_n16: f64 = (p.p7 * s.dn[197][16]);
        let eq219_e2721_d_n17: f64 = (p.p7 * s.dn[197][17]);
        let eq219_e2721_d_n18: f64 = (p.p7 * s.dn[197][18]);
        let eq219_e2721_d_n19: f64 = (p.p7 * s.dn[197][19]);
        let eq219_e2721_d_n20: f64 = (p.p7 * s.dn[197][20]);
        let eq219_e2721_d_n21: f64 = (p.p7 * s.dn[197][21]);
        let eq219_e2721_d_n22: f64 = (p.p7 * s.dn[197][22]);
        let eq219_e2721_q: f64 = (p.p7 * eq219_e2720_q);
        let eq219_e2721_q_d_n0: f64 = (p.p7 * s.dn[197][0]);
        let eq219_e2721_q_d_n1: f64 = (p.p7 * s.dn[197][1]);
        let eq219_e2721_q_d_n2: f64 = (p.p7 * s.dn[197][2]);
        let eq219_e2721_q_d_n3: f64 = (p.p7 * s.dn[197][3]);
        let eq219_e2721_q_d_n4: f64 = (p.p7 * s.dn[197][4]);
        let eq219_e2721_q_d_n5: f64 = (p.p7 * s.dn[197][5]);
        let eq219_e2721_q_d_n6: f64 = (p.p7 * s.dn[197][6]);
        let eq219_e2721_q_d_n7: f64 = (p.p7 * s.dn[197][7]);
        let eq219_e2721_q_d_n8: f64 = (p.p7 * s.dn[197][8]);
        let eq219_e2721_q_d_n9: f64 = (p.p7 * s.dn[197][9]);
        let eq219_e2721_q_d_n10: f64 = (p.p7 * s.dn[197][10]);
        let eq219_e2721_q_d_n11: f64 = (p.p7 * s.dn[197][11]);
        let eq219_e2721_q_d_n12: f64 = (p.p7 * s.dn[197][12]);
        let eq219_e2721_q_d_n13: f64 = (p.p7 * s.dn[197][13]);
        let eq219_e2721_q_d_n14: f64 = (p.p7 * s.dn[197][14]);
        let eq219_e2721_q_d_n15: f64 = (p.p7 * s.dn[197][15]);
        let eq219_e2721_q_d_n16: f64 = (p.p7 * s.dn[197][16]);
        let eq219_e2721_q_d_n17: f64 = (p.p7 * s.dn[197][17]);
        let eq219_e2721_q_d_n18: f64 = (p.p7 * s.dn[197][18]);
        let eq219_e2721_q_d_n19: f64 = (p.p7 * s.dn[197][19]);
        let eq219_e2721_q_d_n20: f64 = (p.p7 * s.dn[197][20]);
        let eq219_e2721_q_d_n21: f64 = (p.p7 * s.dn[197][21]);
        let eq219_e2721_q_d_n22: f64 = (p.p7 * s.dn[197][22]);
        let eq219_reactive_node_derivatives: [f64; 23] = [eq219_e2721_q_d_n0, eq219_e2721_q_d_n1, eq219_e2721_q_d_n2, eq219_e2721_q_d_n3, eq219_e2721_q_d_n4, eq219_e2721_q_d_n5, eq219_e2721_q_d_n6, eq219_e2721_q_d_n7, eq219_e2721_q_d_n8, eq219_e2721_q_d_n9, eq219_e2721_q_d_n10, eq219_e2721_q_d_n11, eq219_e2721_q_d_n12, eq219_e2721_q_d_n13, eq219_e2721_q_d_n14, eq219_e2721_q_d_n15, eq219_e2721_q_d_n16, eq219_e2721_q_d_n17, eq219_e2721_q_d_n18, eq219_e2721_q_d_n19, eq219_e2721_q_d_n20, eq219_e2721_q_d_n21, eq219_e2721_q_d_n22];
        let eq219_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[0]),
            &nodes,
            &eq219_reactive_node_derivatives,
            &branches,
            &eq219_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_220_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq220_e2724_q: f64 = s.v[194];
        let eq220_e2725: f64 = (p.p7 * s.v[194]);
        let eq220_e2725_d_n0: f64 = (p.p7 * s.dn[194][0]);
        let eq220_e2725_d_n1: f64 = (p.p7 * s.dn[194][1]);
        let eq220_e2725_d_n2: f64 = (p.p7 * s.dn[194][2]);
        let eq220_e2725_d_n3: f64 = (p.p7 * s.dn[194][3]);
        let eq220_e2725_d_n4: f64 = (p.p7 * s.dn[194][4]);
        let eq220_e2725_d_n5: f64 = (p.p7 * s.dn[194][5]);
        let eq220_e2725_d_n6: f64 = (p.p7 * s.dn[194][6]);
        let eq220_e2725_d_n7: f64 = (p.p7 * s.dn[194][7]);
        let eq220_e2725_d_n8: f64 = (p.p7 * s.dn[194][8]);
        let eq220_e2725_d_n9: f64 = (p.p7 * s.dn[194][9]);
        let eq220_e2725_d_n10: f64 = (p.p7 * s.dn[194][10]);
        let eq220_e2725_d_n11: f64 = (p.p7 * s.dn[194][11]);
        let eq220_e2725_d_n12: f64 = (p.p7 * s.dn[194][12]);
        let eq220_e2725_d_n13: f64 = (p.p7 * s.dn[194][13]);
        let eq220_e2725_d_n14: f64 = (p.p7 * s.dn[194][14]);
        let eq220_e2725_d_n15: f64 = (p.p7 * s.dn[194][15]);
        let eq220_e2725_d_n16: f64 = (p.p7 * s.dn[194][16]);
        let eq220_e2725_d_n17: f64 = (p.p7 * s.dn[194][17]);
        let eq220_e2725_d_n18: f64 = (p.p7 * s.dn[194][18]);
        let eq220_e2725_d_n19: f64 = (p.p7 * s.dn[194][19]);
        let eq220_e2725_d_n20: f64 = (p.p7 * s.dn[194][20]);
        let eq220_e2725_d_n21: f64 = (p.p7 * s.dn[194][21]);
        let eq220_e2725_d_n22: f64 = (p.p7 * s.dn[194][22]);
        let eq220_e2725_q: f64 = (p.p7 * eq220_e2724_q);
        let eq220_e2725_q_d_n0: f64 = (p.p7 * s.dn[194][0]);
        let eq220_e2725_q_d_n1: f64 = (p.p7 * s.dn[194][1]);
        let eq220_e2725_q_d_n2: f64 = (p.p7 * s.dn[194][2]);
        let eq220_e2725_q_d_n3: f64 = (p.p7 * s.dn[194][3]);
        let eq220_e2725_q_d_n4: f64 = (p.p7 * s.dn[194][4]);
        let eq220_e2725_q_d_n5: f64 = (p.p7 * s.dn[194][5]);
        let eq220_e2725_q_d_n6: f64 = (p.p7 * s.dn[194][6]);
        let eq220_e2725_q_d_n7: f64 = (p.p7 * s.dn[194][7]);
        let eq220_e2725_q_d_n8: f64 = (p.p7 * s.dn[194][8]);
        let eq220_e2725_q_d_n9: f64 = (p.p7 * s.dn[194][9]);
        let eq220_e2725_q_d_n10: f64 = (p.p7 * s.dn[194][10]);
        let eq220_e2725_q_d_n11: f64 = (p.p7 * s.dn[194][11]);
        let eq220_e2725_q_d_n12: f64 = (p.p7 * s.dn[194][12]);
        let eq220_e2725_q_d_n13: f64 = (p.p7 * s.dn[194][13]);
        let eq220_e2725_q_d_n14: f64 = (p.p7 * s.dn[194][14]);
        let eq220_e2725_q_d_n15: f64 = (p.p7 * s.dn[194][15]);
        let eq220_e2725_q_d_n16: f64 = (p.p7 * s.dn[194][16]);
        let eq220_e2725_q_d_n17: f64 = (p.p7 * s.dn[194][17]);
        let eq220_e2725_q_d_n18: f64 = (p.p7 * s.dn[194][18]);
        let eq220_e2725_q_d_n19: f64 = (p.p7 * s.dn[194][19]);
        let eq220_e2725_q_d_n20: f64 = (p.p7 * s.dn[194][20]);
        let eq220_e2725_q_d_n21: f64 = (p.p7 * s.dn[194][21]);
        let eq220_e2725_q_d_n22: f64 = (p.p7 * s.dn[194][22]);
        let eq220_reactive_node_derivatives: [f64; 23] = [eq220_e2725_q_d_n0, eq220_e2725_q_d_n1, eq220_e2725_q_d_n2, eq220_e2725_q_d_n3, eq220_e2725_q_d_n4, eq220_e2725_q_d_n5, eq220_e2725_q_d_n6, eq220_e2725_q_d_n7, eq220_e2725_q_d_n8, eq220_e2725_q_d_n9, eq220_e2725_q_d_n10, eq220_e2725_q_d_n11, eq220_e2725_q_d_n12, eq220_e2725_q_d_n13, eq220_e2725_q_d_n14, eq220_e2725_q_d_n15, eq220_e2725_q_d_n16, eq220_e2725_q_d_n17, eq220_e2725_q_d_n18, eq220_e2725_q_d_n19, eq220_e2725_q_d_n20, eq220_e2725_q_d_n21, eq220_e2725_q_d_n22];
        let eq220_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[0]),
            &nodes,
            &eq220_reactive_node_derivatives,
            &branches,
            &eq220_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_223_block_0(
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
        let (eq223_e2771, eq223_e2771_d_n4, eq223_e2771_q, eq223_e2771_q_d_n4,) = {
    if (s.v[610] != 0.0) {
        let eq223_e2768: f64 = ((nv4 - 0.0) * p.p33);
        let eq223_e2768_d_n4: f64 = p.p33;
        let eq223_e2769_q: f64 = eq223_e2768;
        (eq223_e2768, eq223_e2768_d_n4, eq223_e2769_q, eq223_e2768_d_n4,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[4]),
            None,
            &[
                GeneratedDerivative::node(nodes[4], self.multiplicity * (eq223_e2771_q_d_n4)),
            ],
        );
    }
}
