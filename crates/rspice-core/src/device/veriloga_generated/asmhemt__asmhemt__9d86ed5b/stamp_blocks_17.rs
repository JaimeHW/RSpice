#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_215_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq215_e2699, eq215_e2699_d_n0, eq215_e2699_d_n1, eq215_e2699_d_n2, eq215_e2699_d_n3, eq215_e2699_d_n4, eq215_e2699_d_n5, eq215_e2699_d_n6, eq215_e2699_d_n7, eq215_e2699_d_n8, eq215_e2699_d_n9, eq215_e2699_d_n10, eq215_e2699_d_n11, eq215_e2699_d_n12, eq215_e2699_d_n13, eq215_e2699_d_n14, eq215_e2699_d_n15, eq215_e2699_d_n16, eq215_e2699_d_n17, eq215_e2699_d_n18, eq215_e2699_d_n19, eq215_e2699_d_n20, eq215_e2699_d_n21, eq215_e2699_d_n22,) = {
    if ((!(s.v[605] != 0.0)) && (s.v[608] != 0.0)) {
        let eq215_e2695: f64 = (p.p254 * s.v[312]);
        let eq215_e2695_d_n0: f64 = (p.p254 * s.dn[312][0]);
        let eq215_e2695_d_n1: f64 = (p.p254 * s.dn[312][1]);
        let eq215_e2695_d_n2: f64 = (p.p254 * s.dn[312][2]);
        let eq215_e2695_d_n3: f64 = (p.p254 * s.dn[312][3]);
        let eq215_e2695_d_n4: f64 = (p.p254 * s.dn[312][4]);
        let eq215_e2695_d_n5: f64 = (p.p254 * s.dn[312][5]);
        let eq215_e2695_d_n6: f64 = (p.p254 * s.dn[312][6]);
        let eq215_e2695_d_n7: f64 = (p.p254 * s.dn[312][7]);
        let eq215_e2695_d_n8: f64 = (p.p254 * s.dn[312][8]);
        let eq215_e2695_d_n9: f64 = (p.p254 * s.dn[312][9]);
        let eq215_e2695_d_n10: f64 = (p.p254 * s.dn[312][10]);
        let eq215_e2695_d_n11: f64 = (p.p254 * s.dn[312][11]);
        let eq215_e2695_d_n12: f64 = (p.p254 * s.dn[312][12]);
        let eq215_e2695_d_n13: f64 = (p.p254 * s.dn[312][13]);
        let eq215_e2695_d_n14: f64 = (p.p254 * s.dn[312][14]);
        let eq215_e2695_d_n15: f64 = (p.p254 * s.dn[312][15]);
        let eq215_e2695_d_n16: f64 = (p.p254 * s.dn[312][16]);
        let eq215_e2695_d_n17: f64 = (p.p254 * s.dn[312][17]);
        let eq215_e2695_d_n18: f64 = (p.p254 * s.dn[312][18]);
        let eq215_e2695_d_n19: f64 = (p.p254 * s.dn[312][19]);
        let eq215_e2695_d_n20: f64 = (p.p254 * s.dn[312][20]);
        let eq215_e2695_d_n21: f64 = (p.p254 * s.dn[312][21]);
        let eq215_e2695_d_n22: f64 = (p.p254 * s.dn[312][22]);
        let eq215_e2696: f64 = self.eval_ddt(114, eq215_e2695);
        let eq215_e2696_d_n0: f64 = self.ddt_jacobian(eq215_e2695_d_n0);
        let eq215_e2696_d_n1: f64 = self.ddt_jacobian(eq215_e2695_d_n1);
        let eq215_e2696_d_n2: f64 = self.ddt_jacobian(eq215_e2695_d_n2);
        let eq215_e2696_d_n3: f64 = self.ddt_jacobian(eq215_e2695_d_n3);
        let eq215_e2696_d_n4: f64 = self.ddt_jacobian(eq215_e2695_d_n4);
        let eq215_e2696_d_n5: f64 = self.ddt_jacobian(eq215_e2695_d_n5);
        let eq215_e2696_d_n6: f64 = self.ddt_jacobian(eq215_e2695_d_n6);
        let eq215_e2696_d_n7: f64 = self.ddt_jacobian(eq215_e2695_d_n7);
        let eq215_e2696_d_n8: f64 = self.ddt_jacobian(eq215_e2695_d_n8);
        let eq215_e2696_d_n9: f64 = self.ddt_jacobian(eq215_e2695_d_n9);
        let eq215_e2696_d_n10: f64 = self.ddt_jacobian(eq215_e2695_d_n10);
        let eq215_e2696_d_n11: f64 = self.ddt_jacobian(eq215_e2695_d_n11);
        let eq215_e2696_d_n12: f64 = self.ddt_jacobian(eq215_e2695_d_n12);
        let eq215_e2696_d_n13: f64 = self.ddt_jacobian(eq215_e2695_d_n13);
        let eq215_e2696_d_n14: f64 = self.ddt_jacobian(eq215_e2695_d_n14);
        let eq215_e2696_d_n15: f64 = self.ddt_jacobian(eq215_e2695_d_n15);
        let eq215_e2696_d_n16: f64 = self.ddt_jacobian(eq215_e2695_d_n16);
        let eq215_e2696_d_n17: f64 = self.ddt_jacobian(eq215_e2695_d_n17);
        let eq215_e2696_d_n18: f64 = self.ddt_jacobian(eq215_e2695_d_n18);
        let eq215_e2696_d_n19: f64 = self.ddt_jacobian(eq215_e2695_d_n19);
        let eq215_e2696_d_n20: f64 = self.ddt_jacobian(eq215_e2695_d_n20);
        let eq215_e2696_d_n21: f64 = self.ddt_jacobian(eq215_e2695_d_n21);
        let eq215_e2696_d_n22: f64 = self.ddt_jacobian(eq215_e2695_d_n22);
        let eq215_e2697: f64 = (p.p7 * eq215_e2696);
        let eq215_e2697_d_n0: f64 = (p.p7 * eq215_e2696_d_n0);
        let eq215_e2697_d_n1: f64 = (p.p7 * eq215_e2696_d_n1);
        let eq215_e2697_d_n2: f64 = (p.p7 * eq215_e2696_d_n2);
        let eq215_e2697_d_n3: f64 = (p.p7 * eq215_e2696_d_n3);
        let eq215_e2697_d_n4: f64 = (p.p7 * eq215_e2696_d_n4);
        let eq215_e2697_d_n5: f64 = (p.p7 * eq215_e2696_d_n5);
        let eq215_e2697_d_n6: f64 = (p.p7 * eq215_e2696_d_n6);
        let eq215_e2697_d_n7: f64 = (p.p7 * eq215_e2696_d_n7);
        let eq215_e2697_d_n8: f64 = (p.p7 * eq215_e2696_d_n8);
        let eq215_e2697_d_n9: f64 = (p.p7 * eq215_e2696_d_n9);
        let eq215_e2697_d_n10: f64 = (p.p7 * eq215_e2696_d_n10);
        let eq215_e2697_d_n11: f64 = (p.p7 * eq215_e2696_d_n11);
        let eq215_e2697_d_n12: f64 = (p.p7 * eq215_e2696_d_n12);
        let eq215_e2697_d_n13: f64 = (p.p7 * eq215_e2696_d_n13);
        let eq215_e2697_d_n14: f64 = (p.p7 * eq215_e2696_d_n14);
        let eq215_e2697_d_n15: f64 = (p.p7 * eq215_e2696_d_n15);
        let eq215_e2697_d_n16: f64 = (p.p7 * eq215_e2696_d_n16);
        let eq215_e2697_d_n17: f64 = (p.p7 * eq215_e2696_d_n17);
        let eq215_e2697_d_n18: f64 = (p.p7 * eq215_e2696_d_n18);
        let eq215_e2697_d_n19: f64 = (p.p7 * eq215_e2696_d_n19);
        let eq215_e2697_d_n20: f64 = (p.p7 * eq215_e2696_d_n20);
        let eq215_e2697_d_n21: f64 = (p.p7 * eq215_e2696_d_n21);
        let eq215_e2697_d_n22: f64 = (p.p7 * eq215_e2696_d_n22);
        (eq215_e2697, eq215_e2697_d_n0, eq215_e2697_d_n1, eq215_e2697_d_n2, eq215_e2697_d_n3, eq215_e2697_d_n4, eq215_e2697_d_n5, eq215_e2697_d_n6, eq215_e2697_d_n7, eq215_e2697_d_n8, eq215_e2697_d_n9, eq215_e2697_d_n10, eq215_e2697_d_n11, eq215_e2697_d_n12, eq215_e2697_d_n13, eq215_e2697_d_n14, eq215_e2697_d_n15, eq215_e2697_d_n16, eq215_e2697_d_n17, eq215_e2697_d_n18, eq215_e2697_d_n19, eq215_e2697_d_n20, eq215_e2697_d_n21, eq215_e2697_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq215_value: f64 = eq215_e2699;
        let eq215_node_derivatives: [f64; 23] = [eq215_e2699_d_n0, eq215_e2699_d_n1, eq215_e2699_d_n2, eq215_e2699_d_n3, eq215_e2699_d_n4, eq215_e2699_d_n5, eq215_e2699_d_n6, eq215_e2699_d_n7, eq215_e2699_d_n8, eq215_e2699_d_n9, eq215_e2699_d_n10, eq215_e2699_d_n11, eq215_e2699_d_n12, eq215_e2699_d_n13, eq215_e2699_d_n14, eq215_e2699_d_n15, eq215_e2699_d_n16, eq215_e2699_d_n17, eq215_e2699_d_n18, eq215_e2699_d_n19, eq215_e2699_d_n20, eq215_e2699_d_n21, eq215_e2699_d_n22];
        let eq215_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            self.multiplicity * (eq215_value),
            &nodes,
            &eq215_node_derivatives,
            &branches,
            &eq215_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_216_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq216_e2702: f64 = self.eval_ddt(115, s.v[195]);
        let eq216_e2702_d_n0: f64 = self.ddt_jacobian(s.dn[195][0]);
        let eq216_e2702_d_n1: f64 = self.ddt_jacobian(s.dn[195][1]);
        let eq216_e2702_d_n2: f64 = self.ddt_jacobian(s.dn[195][2]);
        let eq216_e2702_d_n3: f64 = self.ddt_jacobian(s.dn[195][3]);
        let eq216_e2702_d_n4: f64 = self.ddt_jacobian(s.dn[195][4]);
        let eq216_e2702_d_n5: f64 = self.ddt_jacobian(s.dn[195][5]);
        let eq216_e2702_d_n6: f64 = self.ddt_jacobian(s.dn[195][6]);
        let eq216_e2702_d_n7: f64 = self.ddt_jacobian(s.dn[195][7]);
        let eq216_e2702_d_n8: f64 = self.ddt_jacobian(s.dn[195][8]);
        let eq216_e2702_d_n9: f64 = self.ddt_jacobian(s.dn[195][9]);
        let eq216_e2702_d_n10: f64 = self.ddt_jacobian(s.dn[195][10]);
        let eq216_e2702_d_n11: f64 = self.ddt_jacobian(s.dn[195][11]);
        let eq216_e2702_d_n12: f64 = self.ddt_jacobian(s.dn[195][12]);
        let eq216_e2702_d_n13: f64 = self.ddt_jacobian(s.dn[195][13]);
        let eq216_e2702_d_n14: f64 = self.ddt_jacobian(s.dn[195][14]);
        let eq216_e2702_d_n15: f64 = self.ddt_jacobian(s.dn[195][15]);
        let eq216_e2702_d_n16: f64 = self.ddt_jacobian(s.dn[195][16]);
        let eq216_e2702_d_n17: f64 = self.ddt_jacobian(s.dn[195][17]);
        let eq216_e2702_d_n18: f64 = self.ddt_jacobian(s.dn[195][18]);
        let eq216_e2702_d_n19: f64 = self.ddt_jacobian(s.dn[195][19]);
        let eq216_e2702_d_n20: f64 = self.ddt_jacobian(s.dn[195][20]);
        let eq216_e2702_d_n21: f64 = self.ddt_jacobian(s.dn[195][21]);
        let eq216_e2702_d_n22: f64 = self.ddt_jacobian(s.dn[195][22]);
        let eq216_e2703: f64 = (p.p7 * eq216_e2702);
        let eq216_e2703_d_n0: f64 = (p.p7 * eq216_e2702_d_n0);
        let eq216_e2703_d_n1: f64 = (p.p7 * eq216_e2702_d_n1);
        let eq216_e2703_d_n2: f64 = (p.p7 * eq216_e2702_d_n2);
        let eq216_e2703_d_n3: f64 = (p.p7 * eq216_e2702_d_n3);
        let eq216_e2703_d_n4: f64 = (p.p7 * eq216_e2702_d_n4);
        let eq216_e2703_d_n5: f64 = (p.p7 * eq216_e2702_d_n5);
        let eq216_e2703_d_n6: f64 = (p.p7 * eq216_e2702_d_n6);
        let eq216_e2703_d_n7: f64 = (p.p7 * eq216_e2702_d_n7);
        let eq216_e2703_d_n8: f64 = (p.p7 * eq216_e2702_d_n8);
        let eq216_e2703_d_n9: f64 = (p.p7 * eq216_e2702_d_n9);
        let eq216_e2703_d_n10: f64 = (p.p7 * eq216_e2702_d_n10);
        let eq216_e2703_d_n11: f64 = (p.p7 * eq216_e2702_d_n11);
        let eq216_e2703_d_n12: f64 = (p.p7 * eq216_e2702_d_n12);
        let eq216_e2703_d_n13: f64 = (p.p7 * eq216_e2702_d_n13);
        let eq216_e2703_d_n14: f64 = (p.p7 * eq216_e2702_d_n14);
        let eq216_e2703_d_n15: f64 = (p.p7 * eq216_e2702_d_n15);
        let eq216_e2703_d_n16: f64 = (p.p7 * eq216_e2702_d_n16);
        let eq216_e2703_d_n17: f64 = (p.p7 * eq216_e2702_d_n17);
        let eq216_e2703_d_n18: f64 = (p.p7 * eq216_e2702_d_n18);
        let eq216_e2703_d_n19: f64 = (p.p7 * eq216_e2702_d_n19);
        let eq216_e2703_d_n20: f64 = (p.p7 * eq216_e2702_d_n20);
        let eq216_e2703_d_n21: f64 = (p.p7 * eq216_e2702_d_n21);
        let eq216_e2703_d_n22: f64 = (p.p7 * eq216_e2702_d_n22);
        let eq216_value: f64 = eq216_e2703;
        let eq216_node_derivatives: [f64; 23] = [eq216_e2703_d_n0, eq216_e2703_d_n1, eq216_e2703_d_n2, eq216_e2703_d_n3, eq216_e2703_d_n4, eq216_e2703_d_n5, eq216_e2703_d_n6, eq216_e2703_d_n7, eq216_e2703_d_n8, eq216_e2703_d_n9, eq216_e2703_d_n10, eq216_e2703_d_n11, eq216_e2703_d_n12, eq216_e2703_d_n13, eq216_e2703_d_n14, eq216_e2703_d_n15, eq216_e2703_d_n16, eq216_e2703_d_n17, eq216_e2703_d_n18, eq216_e2703_d_n19, eq216_e2703_d_n20, eq216_e2703_d_n21, eq216_e2703_d_n22];
        let eq216_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            self.multiplicity * (eq216_value),
            &nodes,
            &eq216_node_derivatives,
            &branches,
            &eq216_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_217_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let eq217_e2707: f64 = (p.p4 * p.p5);
        let eq217_e2709: f64 = (eq217_e2707 * p.p220);
        let eq217_e2711: f64 = (eq217_e2709 * (nv1 - nv2));
        let eq217_e2711_d_n2: f64 = (-eq217_e2709);
        let eq217_e2712: f64 = self.eval_ddt(116, eq217_e2711);
        let eq217_e2712_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq217_e2712_d_n1: f64 = self.ddt_jacobian(eq217_e2709);
        let eq217_e2712_d_n2: f64 = self.ddt_jacobian(eq217_e2711_d_n2);
        let eq217_e2712_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq217_e2712_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq217_e2712_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq217_e2712_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq217_e2712_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq217_e2712_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq217_e2712_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq217_e2712_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq217_e2712_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq217_e2712_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq217_e2712_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq217_e2712_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq217_e2712_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq217_e2712_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq217_e2712_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq217_e2712_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq217_e2712_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq217_e2712_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq217_e2712_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq217_e2712_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq217_e2713: f64 = (p.p7 * eq217_e2712);
        let eq217_e2713_d_n0: f64 = (p.p7 * eq217_e2712_d_n0);
        let eq217_e2713_d_n1: f64 = (p.p7 * eq217_e2712_d_n1);
        let eq217_e2713_d_n2: f64 = (p.p7 * eq217_e2712_d_n2);
        let eq217_e2713_d_n3: f64 = (p.p7 * eq217_e2712_d_n3);
        let eq217_e2713_d_n4: f64 = (p.p7 * eq217_e2712_d_n4);
        let eq217_e2713_d_n5: f64 = (p.p7 * eq217_e2712_d_n5);
        let eq217_e2713_d_n6: f64 = (p.p7 * eq217_e2712_d_n6);
        let eq217_e2713_d_n7: f64 = (p.p7 * eq217_e2712_d_n7);
        let eq217_e2713_d_n8: f64 = (p.p7 * eq217_e2712_d_n8);
        let eq217_e2713_d_n9: f64 = (p.p7 * eq217_e2712_d_n9);
        let eq217_e2713_d_n10: f64 = (p.p7 * eq217_e2712_d_n10);
        let eq217_e2713_d_n11: f64 = (p.p7 * eq217_e2712_d_n11);
        let eq217_e2713_d_n12: f64 = (p.p7 * eq217_e2712_d_n12);
        let eq217_e2713_d_n13: f64 = (p.p7 * eq217_e2712_d_n13);
        let eq217_e2713_d_n14: f64 = (p.p7 * eq217_e2712_d_n14);
        let eq217_e2713_d_n15: f64 = (p.p7 * eq217_e2712_d_n15);
        let eq217_e2713_d_n16: f64 = (p.p7 * eq217_e2712_d_n16);
        let eq217_e2713_d_n17: f64 = (p.p7 * eq217_e2712_d_n17);
        let eq217_e2713_d_n18: f64 = (p.p7 * eq217_e2712_d_n18);
        let eq217_e2713_d_n19: f64 = (p.p7 * eq217_e2712_d_n19);
        let eq217_e2713_d_n20: f64 = (p.p7 * eq217_e2712_d_n20);
        let eq217_e2713_d_n21: f64 = (p.p7 * eq217_e2712_d_n21);
        let eq217_e2713_d_n22: f64 = (p.p7 * eq217_e2712_d_n22);
        let eq217_value: f64 = eq217_e2713;
        let eq217_node_derivatives: [f64; 23] = [eq217_e2713_d_n0, eq217_e2713_d_n1, eq217_e2713_d_n2, eq217_e2713_d_n3, eq217_e2713_d_n4, eq217_e2713_d_n5, eq217_e2713_d_n6, eq217_e2713_d_n7, eq217_e2713_d_n8, eq217_e2713_d_n9, eq217_e2713_d_n10, eq217_e2713_d_n11, eq217_e2713_d_n12, eq217_e2713_d_n13, eq217_e2713_d_n14, eq217_e2713_d_n15, eq217_e2713_d_n16, eq217_e2713_d_n17, eq217_e2713_d_n18, eq217_e2713_d_n19, eq217_e2713_d_n20, eq217_e2713_d_n21, eq217_e2713_d_n22];
        let eq217_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            self.multiplicity * (eq217_value),
            &nodes,
            &eq217_node_derivatives,
            &branches,
            &eq217_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_218_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq218_e2716: f64 = self.eval_ddt(117, s.v[196]);
        let eq218_e2716_d_n0: f64 = self.ddt_jacobian(s.dn[196][0]);
        let eq218_e2716_d_n1: f64 = self.ddt_jacobian(s.dn[196][1]);
        let eq218_e2716_d_n2: f64 = self.ddt_jacobian(s.dn[196][2]);
        let eq218_e2716_d_n3: f64 = self.ddt_jacobian(s.dn[196][3]);
        let eq218_e2716_d_n4: f64 = self.ddt_jacobian(s.dn[196][4]);
        let eq218_e2716_d_n5: f64 = self.ddt_jacobian(s.dn[196][5]);
        let eq218_e2716_d_n6: f64 = self.ddt_jacobian(s.dn[196][6]);
        let eq218_e2716_d_n7: f64 = self.ddt_jacobian(s.dn[196][7]);
        let eq218_e2716_d_n8: f64 = self.ddt_jacobian(s.dn[196][8]);
        let eq218_e2716_d_n9: f64 = self.ddt_jacobian(s.dn[196][9]);
        let eq218_e2716_d_n10: f64 = self.ddt_jacobian(s.dn[196][10]);
        let eq218_e2716_d_n11: f64 = self.ddt_jacobian(s.dn[196][11]);
        let eq218_e2716_d_n12: f64 = self.ddt_jacobian(s.dn[196][12]);
        let eq218_e2716_d_n13: f64 = self.ddt_jacobian(s.dn[196][13]);
        let eq218_e2716_d_n14: f64 = self.ddt_jacobian(s.dn[196][14]);
        let eq218_e2716_d_n15: f64 = self.ddt_jacobian(s.dn[196][15]);
        let eq218_e2716_d_n16: f64 = self.ddt_jacobian(s.dn[196][16]);
        let eq218_e2716_d_n17: f64 = self.ddt_jacobian(s.dn[196][17]);
        let eq218_e2716_d_n18: f64 = self.ddt_jacobian(s.dn[196][18]);
        let eq218_e2716_d_n19: f64 = self.ddt_jacobian(s.dn[196][19]);
        let eq218_e2716_d_n20: f64 = self.ddt_jacobian(s.dn[196][20]);
        let eq218_e2716_d_n21: f64 = self.ddt_jacobian(s.dn[196][21]);
        let eq218_e2716_d_n22: f64 = self.ddt_jacobian(s.dn[196][22]);
        let eq218_e2717: f64 = (p.p7 * eq218_e2716);
        let eq218_e2717_d_n0: f64 = (p.p7 * eq218_e2716_d_n0);
        let eq218_e2717_d_n1: f64 = (p.p7 * eq218_e2716_d_n1);
        let eq218_e2717_d_n2: f64 = (p.p7 * eq218_e2716_d_n2);
        let eq218_e2717_d_n3: f64 = (p.p7 * eq218_e2716_d_n3);
        let eq218_e2717_d_n4: f64 = (p.p7 * eq218_e2716_d_n4);
        let eq218_e2717_d_n5: f64 = (p.p7 * eq218_e2716_d_n5);
        let eq218_e2717_d_n6: f64 = (p.p7 * eq218_e2716_d_n6);
        let eq218_e2717_d_n7: f64 = (p.p7 * eq218_e2716_d_n7);
        let eq218_e2717_d_n8: f64 = (p.p7 * eq218_e2716_d_n8);
        let eq218_e2717_d_n9: f64 = (p.p7 * eq218_e2716_d_n9);
        let eq218_e2717_d_n10: f64 = (p.p7 * eq218_e2716_d_n10);
        let eq218_e2717_d_n11: f64 = (p.p7 * eq218_e2716_d_n11);
        let eq218_e2717_d_n12: f64 = (p.p7 * eq218_e2716_d_n12);
        let eq218_e2717_d_n13: f64 = (p.p7 * eq218_e2716_d_n13);
        let eq218_e2717_d_n14: f64 = (p.p7 * eq218_e2716_d_n14);
        let eq218_e2717_d_n15: f64 = (p.p7 * eq218_e2716_d_n15);
        let eq218_e2717_d_n16: f64 = (p.p7 * eq218_e2716_d_n16);
        let eq218_e2717_d_n17: f64 = (p.p7 * eq218_e2716_d_n17);
        let eq218_e2717_d_n18: f64 = (p.p7 * eq218_e2716_d_n18);
        let eq218_e2717_d_n19: f64 = (p.p7 * eq218_e2716_d_n19);
        let eq218_e2717_d_n20: f64 = (p.p7 * eq218_e2716_d_n20);
        let eq218_e2717_d_n21: f64 = (p.p7 * eq218_e2716_d_n21);
        let eq218_e2717_d_n22: f64 = (p.p7 * eq218_e2716_d_n22);
        let eq218_value: f64 = eq218_e2717;
        let eq218_node_derivatives: [f64; 23] = [eq218_e2717_d_n0, eq218_e2717_d_n1, eq218_e2717_d_n2, eq218_e2717_d_n3, eq218_e2717_d_n4, eq218_e2717_d_n5, eq218_e2717_d_n6, eq218_e2717_d_n7, eq218_e2717_d_n8, eq218_e2717_d_n9, eq218_e2717_d_n10, eq218_e2717_d_n11, eq218_e2717_d_n12, eq218_e2717_d_n13, eq218_e2717_d_n14, eq218_e2717_d_n15, eq218_e2717_d_n16, eq218_e2717_d_n17, eq218_e2717_d_n18, eq218_e2717_d_n19, eq218_e2717_d_n20, eq218_e2717_d_n21, eq218_e2717_d_n22];
        let eq218_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[2]),
            self.multiplicity * (eq218_value),
            &nodes,
            &eq218_node_derivatives,
            &branches,
            &eq218_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_219_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq219_e2720: f64 = self.eval_ddt(118, s.v[197]);
        let eq219_e2720_d_n0: f64 = self.ddt_jacobian(s.dn[197][0]);
        let eq219_e2720_d_n1: f64 = self.ddt_jacobian(s.dn[197][1]);
        let eq219_e2720_d_n2: f64 = self.ddt_jacobian(s.dn[197][2]);
        let eq219_e2720_d_n3: f64 = self.ddt_jacobian(s.dn[197][3]);
        let eq219_e2720_d_n4: f64 = self.ddt_jacobian(s.dn[197][4]);
        let eq219_e2720_d_n5: f64 = self.ddt_jacobian(s.dn[197][5]);
        let eq219_e2720_d_n6: f64 = self.ddt_jacobian(s.dn[197][6]);
        let eq219_e2720_d_n7: f64 = self.ddt_jacobian(s.dn[197][7]);
        let eq219_e2720_d_n8: f64 = self.ddt_jacobian(s.dn[197][8]);
        let eq219_e2720_d_n9: f64 = self.ddt_jacobian(s.dn[197][9]);
        let eq219_e2720_d_n10: f64 = self.ddt_jacobian(s.dn[197][10]);
        let eq219_e2720_d_n11: f64 = self.ddt_jacobian(s.dn[197][11]);
        let eq219_e2720_d_n12: f64 = self.ddt_jacobian(s.dn[197][12]);
        let eq219_e2720_d_n13: f64 = self.ddt_jacobian(s.dn[197][13]);
        let eq219_e2720_d_n14: f64 = self.ddt_jacobian(s.dn[197][14]);
        let eq219_e2720_d_n15: f64 = self.ddt_jacobian(s.dn[197][15]);
        let eq219_e2720_d_n16: f64 = self.ddt_jacobian(s.dn[197][16]);
        let eq219_e2720_d_n17: f64 = self.ddt_jacobian(s.dn[197][17]);
        let eq219_e2720_d_n18: f64 = self.ddt_jacobian(s.dn[197][18]);
        let eq219_e2720_d_n19: f64 = self.ddt_jacobian(s.dn[197][19]);
        let eq219_e2720_d_n20: f64 = self.ddt_jacobian(s.dn[197][20]);
        let eq219_e2720_d_n21: f64 = self.ddt_jacobian(s.dn[197][21]);
        let eq219_e2720_d_n22: f64 = self.ddt_jacobian(s.dn[197][22]);
        let eq219_e2721: f64 = (p.p7 * eq219_e2720);
        let eq219_e2721_d_n0: f64 = (p.p7 * eq219_e2720_d_n0);
        let eq219_e2721_d_n1: f64 = (p.p7 * eq219_e2720_d_n1);
        let eq219_e2721_d_n2: f64 = (p.p7 * eq219_e2720_d_n2);
        let eq219_e2721_d_n3: f64 = (p.p7 * eq219_e2720_d_n3);
        let eq219_e2721_d_n4: f64 = (p.p7 * eq219_e2720_d_n4);
        let eq219_e2721_d_n5: f64 = (p.p7 * eq219_e2720_d_n5);
        let eq219_e2721_d_n6: f64 = (p.p7 * eq219_e2720_d_n6);
        let eq219_e2721_d_n7: f64 = (p.p7 * eq219_e2720_d_n7);
        let eq219_e2721_d_n8: f64 = (p.p7 * eq219_e2720_d_n8);
        let eq219_e2721_d_n9: f64 = (p.p7 * eq219_e2720_d_n9);
        let eq219_e2721_d_n10: f64 = (p.p7 * eq219_e2720_d_n10);
        let eq219_e2721_d_n11: f64 = (p.p7 * eq219_e2720_d_n11);
        let eq219_e2721_d_n12: f64 = (p.p7 * eq219_e2720_d_n12);
        let eq219_e2721_d_n13: f64 = (p.p7 * eq219_e2720_d_n13);
        let eq219_e2721_d_n14: f64 = (p.p7 * eq219_e2720_d_n14);
        let eq219_e2721_d_n15: f64 = (p.p7 * eq219_e2720_d_n15);
        let eq219_e2721_d_n16: f64 = (p.p7 * eq219_e2720_d_n16);
        let eq219_e2721_d_n17: f64 = (p.p7 * eq219_e2720_d_n17);
        let eq219_e2721_d_n18: f64 = (p.p7 * eq219_e2720_d_n18);
        let eq219_e2721_d_n19: f64 = (p.p7 * eq219_e2720_d_n19);
        let eq219_e2721_d_n20: f64 = (p.p7 * eq219_e2720_d_n20);
        let eq219_e2721_d_n21: f64 = (p.p7 * eq219_e2720_d_n21);
        let eq219_e2721_d_n22: f64 = (p.p7 * eq219_e2720_d_n22);
        let eq219_value: f64 = eq219_e2721;
        let eq219_node_derivatives: [f64; 23] = [eq219_e2721_d_n0, eq219_e2721_d_n1, eq219_e2721_d_n2, eq219_e2721_d_n3, eq219_e2721_d_n4, eq219_e2721_d_n5, eq219_e2721_d_n6, eq219_e2721_d_n7, eq219_e2721_d_n8, eq219_e2721_d_n9, eq219_e2721_d_n10, eq219_e2721_d_n11, eq219_e2721_d_n12, eq219_e2721_d_n13, eq219_e2721_d_n14, eq219_e2721_d_n15, eq219_e2721_d_n16, eq219_e2721_d_n17, eq219_e2721_d_n18, eq219_e2721_d_n19, eq219_e2721_d_n20, eq219_e2721_d_n21, eq219_e2721_d_n22];
        let eq219_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[0]),
            self.multiplicity * (eq219_value),
            &nodes,
            &eq219_node_derivatives,
            &branches,
            &eq219_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_220_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq220_e2724: f64 = self.eval_ddt(119, s.v[194]);
        let eq220_e2724_d_n0: f64 = self.ddt_jacobian(s.dn[194][0]);
        let eq220_e2724_d_n1: f64 = self.ddt_jacobian(s.dn[194][1]);
        let eq220_e2724_d_n2: f64 = self.ddt_jacobian(s.dn[194][2]);
        let eq220_e2724_d_n3: f64 = self.ddt_jacobian(s.dn[194][3]);
        let eq220_e2724_d_n4: f64 = self.ddt_jacobian(s.dn[194][4]);
        let eq220_e2724_d_n5: f64 = self.ddt_jacobian(s.dn[194][5]);
        let eq220_e2724_d_n6: f64 = self.ddt_jacobian(s.dn[194][6]);
        let eq220_e2724_d_n7: f64 = self.ddt_jacobian(s.dn[194][7]);
        let eq220_e2724_d_n8: f64 = self.ddt_jacobian(s.dn[194][8]);
        let eq220_e2724_d_n9: f64 = self.ddt_jacobian(s.dn[194][9]);
        let eq220_e2724_d_n10: f64 = self.ddt_jacobian(s.dn[194][10]);
        let eq220_e2724_d_n11: f64 = self.ddt_jacobian(s.dn[194][11]);
        let eq220_e2724_d_n12: f64 = self.ddt_jacobian(s.dn[194][12]);
        let eq220_e2724_d_n13: f64 = self.ddt_jacobian(s.dn[194][13]);
        let eq220_e2724_d_n14: f64 = self.ddt_jacobian(s.dn[194][14]);
        let eq220_e2724_d_n15: f64 = self.ddt_jacobian(s.dn[194][15]);
        let eq220_e2724_d_n16: f64 = self.ddt_jacobian(s.dn[194][16]);
        let eq220_e2724_d_n17: f64 = self.ddt_jacobian(s.dn[194][17]);
        let eq220_e2724_d_n18: f64 = self.ddt_jacobian(s.dn[194][18]);
        let eq220_e2724_d_n19: f64 = self.ddt_jacobian(s.dn[194][19]);
        let eq220_e2724_d_n20: f64 = self.ddt_jacobian(s.dn[194][20]);
        let eq220_e2724_d_n21: f64 = self.ddt_jacobian(s.dn[194][21]);
        let eq220_e2724_d_n22: f64 = self.ddt_jacobian(s.dn[194][22]);
        let eq220_e2725: f64 = (p.p7 * eq220_e2724);
        let eq220_e2725_d_n0: f64 = (p.p7 * eq220_e2724_d_n0);
        let eq220_e2725_d_n1: f64 = (p.p7 * eq220_e2724_d_n1);
        let eq220_e2725_d_n2: f64 = (p.p7 * eq220_e2724_d_n2);
        let eq220_e2725_d_n3: f64 = (p.p7 * eq220_e2724_d_n3);
        let eq220_e2725_d_n4: f64 = (p.p7 * eq220_e2724_d_n4);
        let eq220_e2725_d_n5: f64 = (p.p7 * eq220_e2724_d_n5);
        let eq220_e2725_d_n6: f64 = (p.p7 * eq220_e2724_d_n6);
        let eq220_e2725_d_n7: f64 = (p.p7 * eq220_e2724_d_n7);
        let eq220_e2725_d_n8: f64 = (p.p7 * eq220_e2724_d_n8);
        let eq220_e2725_d_n9: f64 = (p.p7 * eq220_e2724_d_n9);
        let eq220_e2725_d_n10: f64 = (p.p7 * eq220_e2724_d_n10);
        let eq220_e2725_d_n11: f64 = (p.p7 * eq220_e2724_d_n11);
        let eq220_e2725_d_n12: f64 = (p.p7 * eq220_e2724_d_n12);
        let eq220_e2725_d_n13: f64 = (p.p7 * eq220_e2724_d_n13);
        let eq220_e2725_d_n14: f64 = (p.p7 * eq220_e2724_d_n14);
        let eq220_e2725_d_n15: f64 = (p.p7 * eq220_e2724_d_n15);
        let eq220_e2725_d_n16: f64 = (p.p7 * eq220_e2724_d_n16);
        let eq220_e2725_d_n17: f64 = (p.p7 * eq220_e2724_d_n17);
        let eq220_e2725_d_n18: f64 = (p.p7 * eq220_e2724_d_n18);
        let eq220_e2725_d_n19: f64 = (p.p7 * eq220_e2724_d_n19);
        let eq220_e2725_d_n20: f64 = (p.p7 * eq220_e2724_d_n20);
        let eq220_e2725_d_n21: f64 = (p.p7 * eq220_e2724_d_n21);
        let eq220_e2725_d_n22: f64 = (p.p7 * eq220_e2724_d_n22);
        let eq220_value: f64 = eq220_e2725;
        let eq220_node_derivatives: [f64; 23] = [eq220_e2725_d_n0, eq220_e2725_d_n1, eq220_e2725_d_n2, eq220_e2725_d_n3, eq220_e2725_d_n4, eq220_e2725_d_n5, eq220_e2725_d_n6, eq220_e2725_d_n7, eq220_e2725_d_n8, eq220_e2725_d_n9, eq220_e2725_d_n10, eq220_e2725_d_n11, eq220_e2725_d_n12, eq220_e2725_d_n13, eq220_e2725_d_n14, eq220_e2725_d_n15, eq220_e2725_d_n16, eq220_e2725_d_n17, eq220_e2725_d_n18, eq220_e2725_d_n19, eq220_e2725_d_n20, eq220_e2725_d_n21, eq220_e2725_d_n22];
        let eq220_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[0]),
            self.multiplicity * (eq220_value),
            &nodes,
            &eq220_node_derivatives,
            &branches,
            &eq220_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_221_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq221_ad: A = {
    if (s.v[610] != 0.0) {
        A::sub(A::sub(A::sub(A::sub(A::mul(A::scale(s.ad_value(94), (-1.0)), s.ad_value(38)), A::mul(s.ad_value(233), s.ad_value(231))), A::mul(s.ad_value(257), s.ad_value(255))), A::mul(s.ad_value(281), s.ad_value(279))), A::mul(s.ad_value(305), s.ad_value(303)))
    } else {
        A::constant(0.0)
    }
};
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            self.multiplicity * eq221_ad.value,
            &nodes,
            &eq221_ad.dn,
            &branches,
            &eq221_ad.db,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_222_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq222_e2764, eq222_e2764_d_n4,) = {
    if (s.v[610] != 0.0) {
        let eq222_e2762: f64 = ((nv4 - 0.0) / p.p32);
        let eq222_e2762_d_n4: f64 = (1.0 / p.p32);
        (eq222_e2762, eq222_e2762_d_n4,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq222_value: f64 = eq222_e2764;
        stamper.stamp_current(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq222_value),
            &[
                GeneratedDerivative::node(nodes[4], self.multiplicity * eq222_e2764_d_n4),
            ],
        );
    }

    pub(super) fn stamp_transient_equation_223_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq223_e2771, eq223_e2771_d_n0, eq223_e2771_d_n1, eq223_e2771_d_n2, eq223_e2771_d_n3, eq223_e2771_d_n4, eq223_e2771_d_n5, eq223_e2771_d_n6, eq223_e2771_d_n7, eq223_e2771_d_n8, eq223_e2771_d_n9, eq223_e2771_d_n10, eq223_e2771_d_n11, eq223_e2771_d_n12, eq223_e2771_d_n13, eq223_e2771_d_n14, eq223_e2771_d_n15, eq223_e2771_d_n16, eq223_e2771_d_n17, eq223_e2771_d_n18, eq223_e2771_d_n19, eq223_e2771_d_n20, eq223_e2771_d_n21, eq223_e2771_d_n22,) = {
    if (s.v[610] != 0.0) {
        let eq223_e2768: f64 = ((nv4 - 0.0) * p.p33);
        let eq223_e2768_d_n4: f64 = p.p33;
        let eq223_e2769: f64 = self.eval_ddt(120, eq223_e2768);
        let eq223_e2769_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq223_e2769_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq223_e2769_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq223_e2769_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq223_e2769_d_n4: f64 = self.ddt_jacobian(eq223_e2768_d_n4);
        let eq223_e2769_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq223_e2769_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq223_e2769_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq223_e2769_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq223_e2769_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq223_e2769_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq223_e2769_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq223_e2769_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq223_e2769_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq223_e2769_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq223_e2769_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq223_e2769_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq223_e2769_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq223_e2769_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq223_e2769_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq223_e2769_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq223_e2769_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq223_e2769_d_n22: f64 = self.ddt_jacobian(0.0);
        (eq223_e2769, eq223_e2769_d_n0, eq223_e2769_d_n1, eq223_e2769_d_n2, eq223_e2769_d_n3, eq223_e2769_d_n4, eq223_e2769_d_n5, eq223_e2769_d_n6, eq223_e2769_d_n7, eq223_e2769_d_n8, eq223_e2769_d_n9, eq223_e2769_d_n10, eq223_e2769_d_n11, eq223_e2769_d_n12, eq223_e2769_d_n13, eq223_e2769_d_n14, eq223_e2769_d_n15, eq223_e2769_d_n16, eq223_e2769_d_n17, eq223_e2769_d_n18, eq223_e2769_d_n19, eq223_e2769_d_n20, eq223_e2769_d_n21, eq223_e2769_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq223_value: f64 = eq223_e2771;
        let eq223_node_derivatives: [f64; 23] = [eq223_e2771_d_n0, eq223_e2771_d_n1, eq223_e2771_d_n2, eq223_e2771_d_n3, eq223_e2771_d_n4, eq223_e2771_d_n5, eq223_e2771_d_n6, eq223_e2771_d_n7, eq223_e2771_d_n8, eq223_e2771_d_n9, eq223_e2771_d_n10, eq223_e2771_d_n11, eq223_e2771_d_n12, eq223_e2771_d_n13, eq223_e2771_d_n14, eq223_e2771_d_n15, eq223_e2771_d_n16, eq223_e2771_d_n17, eq223_e2771_d_n18, eq223_e2771_d_n19, eq223_e2771_d_n20, eq223_e2771_d_n21, eq223_e2771_d_n22];
        let eq223_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq223_value),
            &nodes,
            &eq223_node_derivatives,
            &branches,
            &eq223_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_224_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq224_e2776,) = {
    if (!(s.v[610] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq224_value: f64 = eq224_e2776;
        stamper.stamp_potential(
            branches[56],
            eq224_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_9_block_0(
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
        let (eq9_e355, eq9_e355_d_n5, eq9_e355_q, eq9_e355_q_d_n5,) = {
    if ((s.v[388] != 0.0) && (!(s.v[387] != 0.0))) {
        let eq9_e352_q: f64 = (nv5 - 0.0);
        let eq9_e353: f64 = (p.p97 * (nv5 - 0.0));
        let eq9_e353_d_n5: f64 = p.p97;
        let eq9_e353_q: f64 = (p.p97 * eq9_e352_q);
        let eq9_e353_q_d_n5: f64 = p.p97;
        (eq9_e353, eq9_e353_d_n5, eq9_e353_q, eq9_e353_q_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[5]),
            None,
            &[
                GeneratedDerivative::node(nodes[5], self.multiplicity * (eq9_e355_q_d_n5)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_17_block_0(
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
        let (eq17_e427, eq17_e427_d_n5, eq17_e427_q, eq17_e427_q_d_n5,) = {
    if ((s.v[389] != 0.0) && (!((s.v[387] != 0.0) || (s.v[388] != 0.0)))) {
        let eq17_e424_q: f64 = (nv5 - 0.0);
        let eq17_e425: f64 = (p.p110 * (nv5 - 0.0));
        let eq17_e425_d_n5: f64 = p.p110;
        let eq17_e425_q: f64 = (p.p110 * eq17_e424_q);
        let eq17_e425_q_d_n5: f64 = p.p110;
        (eq17_e425, eq17_e425_d_n5, eq17_e425_q, eq17_e425_q_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[5]),
            None,
            &[
                GeneratedDerivative::node(nodes[5], self.multiplicity * (eq17_e427_q_d_n5)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_20_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq20_e462, eq20_e462_d_n6, eq20_e462_q, eq20_e462_q_d_n6,) = {
    if ((s.v[389] != 0.0) && (!((s.v[387] != 0.0) || (s.v[388] != 0.0)))) {
        let eq20_e459_q: f64 = (nv6 - 0.0);
        let eq20_e460: f64 = (p.p111 * (nv6 - 0.0));
        let eq20_e460_d_n6: f64 = p.p111;
        let eq20_e460_q: f64 = (p.p111 * eq20_e459_q);
        let eq20_e460_q_d_n6: f64 = p.p111;
        (eq20_e460, eq20_e460_d_n6, eq20_e460_q, eq20_e460_q_d_n6,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[6]),
            None,
            &[
                GeneratedDerivative::node(nodes[6], self.multiplicity * (eq20_e462_q_d_n6)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_27_block_0(
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
        let (eq27_e539, eq27_e539_d_n0, eq27_e539_d_n1, eq27_e539_d_n2, eq27_e539_d_n3, eq27_e539_d_n4, eq27_e539_d_n5, eq27_e539_d_n6, eq27_e539_d_n7, eq27_e539_d_n8, eq27_e539_d_n9, eq27_e539_d_n10, eq27_e539_d_n11, eq27_e539_d_n12, eq27_e539_d_n13, eq27_e539_d_n14, eq27_e539_d_n15, eq27_e539_d_n16, eq27_e539_d_n17, eq27_e539_d_n18, eq27_e539_d_n19, eq27_e539_d_n20, eq27_e539_d_n21, eq27_e539_d_n22, eq27_e539_q, eq27_e539_q_d_n0, eq27_e539_q_d_n1, eq27_e539_q_d_n2, eq27_e539_q_d_n3, eq27_e539_q_d_n4, eq27_e539_q_d_n5, eq27_e539_q_d_n6, eq27_e539_q_d_n7, eq27_e539_q_d_n8, eq27_e539_q_d_n9, eq27_e539_q_d_n10, eq27_e539_q_d_n11, eq27_e539_q_d_n12, eq27_e539_q_d_n13, eq27_e539_q_d_n14, eq27_e539_q_d_n15, eq27_e539_q_d_n16, eq27_e539_q_d_n17, eq27_e539_q_d_n18, eq27_e539_q_d_n19, eq27_e539_q_d_n20, eq27_e539_q_d_n21, eq27_e539_q_d_n22,) = {
    if ((s.v[390] != 0.0) && (!(((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)))) {
        let eq27_e536_q: f64 = (nv5 - 0.0);
        let eq27_e537: f64 = (s.v[149] * (nv5 - 0.0));
        let eq27_e537_d_n0: f64 = (s.dn[149][0] * (nv5 - 0.0));
        let eq27_e537_d_n1: f64 = (s.dn[149][1] * (nv5 - 0.0));
        let eq27_e537_d_n2: f64 = (s.dn[149][2] * (nv5 - 0.0));
        let eq27_e537_d_n3: f64 = (s.dn[149][3] * (nv5 - 0.0));
        let eq27_e537_d_n4: f64 = (s.dn[149][4] * (nv5 - 0.0));
        let eq27_e537_d_n5: f64 = ((s.dn[149][5] * (nv5 - 0.0)) + s.v[149]);
        let eq27_e537_d_n6: f64 = (s.dn[149][6] * (nv5 - 0.0));
        let eq27_e537_d_n7: f64 = (s.dn[149][7] * (nv5 - 0.0));
        let eq27_e537_d_n8: f64 = (s.dn[149][8] * (nv5 - 0.0));
        let eq27_e537_d_n9: f64 = (s.dn[149][9] * (nv5 - 0.0));
        let eq27_e537_d_n10: f64 = (s.dn[149][10] * (nv5 - 0.0));
        let eq27_e537_d_n11: f64 = (s.dn[149][11] * (nv5 - 0.0));
        let eq27_e537_d_n12: f64 = (s.dn[149][12] * (nv5 - 0.0));
        let eq27_e537_d_n13: f64 = (s.dn[149][13] * (nv5 - 0.0));
        let eq27_e537_d_n14: f64 = (s.dn[149][14] * (nv5 - 0.0));
        let eq27_e537_d_n15: f64 = (s.dn[149][15] * (nv5 - 0.0));
        let eq27_e537_d_n16: f64 = (s.dn[149][16] * (nv5 - 0.0));
        let eq27_e537_d_n17: f64 = (s.dn[149][17] * (nv5 - 0.0));
        let eq27_e537_d_n18: f64 = (s.dn[149][18] * (nv5 - 0.0));
        let eq27_e537_d_n19: f64 = (s.dn[149][19] * (nv5 - 0.0));
        let eq27_e537_d_n20: f64 = (s.dn[149][20] * (nv5 - 0.0));
        let eq27_e537_d_n21: f64 = (s.dn[149][21] * (nv5 - 0.0));
        let eq27_e537_d_n22: f64 = (s.dn[149][22] * (nv5 - 0.0));
        let eq27_e537_q: f64 = (s.v[149] * eq27_e536_q);
        let eq27_e537_q_d_n0: f64 = (s.dn[149][0] * eq27_e536_q);
        let eq27_e537_q_d_n1: f64 = (s.dn[149][1] * eq27_e536_q);
        let eq27_e537_q_d_n2: f64 = (s.dn[149][2] * eq27_e536_q);
        let eq27_e537_q_d_n3: f64 = (s.dn[149][3] * eq27_e536_q);
        let eq27_e537_q_d_n4: f64 = (s.dn[149][4] * eq27_e536_q);
        let eq27_e537_q_d_n5: f64 = ((s.dn[149][5] * eq27_e536_q) + s.v[149]);
        let eq27_e537_q_d_n6: f64 = (s.dn[149][6] * eq27_e536_q);
        let eq27_e537_q_d_n7: f64 = (s.dn[149][7] * eq27_e536_q);
        let eq27_e537_q_d_n8: f64 = (s.dn[149][8] * eq27_e536_q);
        let eq27_e537_q_d_n9: f64 = (s.dn[149][9] * eq27_e536_q);
        let eq27_e537_q_d_n10: f64 = (s.dn[149][10] * eq27_e536_q);
        let eq27_e537_q_d_n11: f64 = (s.dn[149][11] * eq27_e536_q);
        let eq27_e537_q_d_n12: f64 = (s.dn[149][12] * eq27_e536_q);
        let eq27_e537_q_d_n13: f64 = (s.dn[149][13] * eq27_e536_q);
        let eq27_e537_q_d_n14: f64 = (s.dn[149][14] * eq27_e536_q);
        let eq27_e537_q_d_n15: f64 = (s.dn[149][15] * eq27_e536_q);
        let eq27_e537_q_d_n16: f64 = (s.dn[149][16] * eq27_e536_q);
        let eq27_e537_q_d_n17: f64 = (s.dn[149][17] * eq27_e536_q);
        let eq27_e537_q_d_n18: f64 = (s.dn[149][18] * eq27_e536_q);
        let eq27_e537_q_d_n19: f64 = (s.dn[149][19] * eq27_e536_q);
        let eq27_e537_q_d_n20: f64 = (s.dn[149][20] * eq27_e536_q);
        let eq27_e537_q_d_n21: f64 = (s.dn[149][21] * eq27_e536_q);
        let eq27_e537_q_d_n22: f64 = (s.dn[149][22] * eq27_e536_q);
        (eq27_e537, eq27_e537_d_n0, eq27_e537_d_n1, eq27_e537_d_n2, eq27_e537_d_n3, eq27_e537_d_n4, eq27_e537_d_n5, eq27_e537_d_n6, eq27_e537_d_n7, eq27_e537_d_n8, eq27_e537_d_n9, eq27_e537_d_n10, eq27_e537_d_n11, eq27_e537_d_n12, eq27_e537_d_n13, eq27_e537_d_n14, eq27_e537_d_n15, eq27_e537_d_n16, eq27_e537_d_n17, eq27_e537_d_n18, eq27_e537_d_n19, eq27_e537_d_n20, eq27_e537_d_n21, eq27_e537_d_n22, eq27_e537_q, eq27_e537_q_d_n0, eq27_e537_q_d_n1, eq27_e537_q_d_n2, eq27_e537_q_d_n3, eq27_e537_q_d_n4, eq27_e537_q_d_n5, eq27_e537_q_d_n6, eq27_e537_q_d_n7, eq27_e537_q_d_n8, eq27_e537_q_d_n9, eq27_e537_q_d_n10, eq27_e537_q_d_n11, eq27_e537_q_d_n12, eq27_e537_q_d_n13, eq27_e537_q_d_n14, eq27_e537_q_d_n15, eq27_e537_q_d_n16, eq27_e537_q_d_n17, eq27_e537_q_d_n18, eq27_e537_q_d_n19, eq27_e537_q_d_n20, eq27_e537_q_d_n21, eq27_e537_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_reactive_node_derivatives: [f64; 23] = [eq27_e539_q_d_n0, eq27_e539_q_d_n1, eq27_e539_q_d_n2, eq27_e539_q_d_n3, eq27_e539_q_d_n4, eq27_e539_q_d_n5, eq27_e539_q_d_n6, eq27_e539_q_d_n7, eq27_e539_q_d_n8, eq27_e539_q_d_n9, eq27_e539_q_d_n10, eq27_e539_q_d_n11, eq27_e539_q_d_n12, eq27_e539_q_d_n13, eq27_e539_q_d_n14, eq27_e539_q_d_n15, eq27_e539_q_d_n16, eq27_e539_q_d_n17, eq27_e539_q_d_n18, eq27_e539_q_d_n19, eq27_e539_q_d_n20, eq27_e539_q_d_n21, eq27_e539_q_d_n22];
        let eq27_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            &nodes,
            &eq27_reactive_node_derivatives,
            &branches,
            &eq27_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_37_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq37_e668, eq37_e668_d_n12, eq37_e668_q, eq37_e668_q_d_n12,) = {
    if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
        let eq37_e661_q: f64 = (nv12 - 0.0);
        let eq37_e662: f64 = (p.p97 * (nv12 - 0.0));
        let eq37_e662_d_n12: f64 = p.p97;
        let eq37_e662_q: f64 = (p.p97 * eq37_e661_q);
        let eq37_e662_q_d_n12: f64 = p.p97;
        let eq37_e665: f64 = (1e-12 * (nv12 - 0.0));
        let eq37_e665_d_n12: f64 = 1e-12;
        let eq37_e666: f64 = (eq37_e662 + eq37_e665);
        let eq37_e666_d_n12: f64 = (eq37_e662_d_n12 + eq37_e665_d_n12);
        let eq37_e666_q: f64 = eq37_e662_q;
        (eq37_e666, eq37_e666_d_n12, eq37_e666_q, eq37_e662_q_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[12]),
            None,
            &[
                GeneratedDerivative::node(nodes[12], self.multiplicity * (eq37_e668_q_d_n12)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_40_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq40_e716, eq40_e716_d_n14, eq40_e716_q, eq40_e716_q_d_n14,) = {
    if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
        let eq40_e709_q: f64 = (nv14 - 0.0);
        let eq40_e710: f64 = (p.p83 * (nv14 - 0.0));
        let eq40_e710_d_n14: f64 = p.p83;
        let eq40_e710_q: f64 = (p.p83 * eq40_e709_q);
        let eq40_e710_q_d_n14: f64 = p.p83;
        let eq40_e713: f64 = (1e-12 * (nv14 - 0.0));
        let eq40_e713_d_n14: f64 = 1e-12;
        let eq40_e714: f64 = (eq40_e710 + eq40_e713);
        let eq40_e714_d_n14: f64 = (eq40_e710_d_n14 + eq40_e713_d_n14);
        let eq40_e714_q: f64 = eq40_e710_q;
        (eq40_e714, eq40_e714_d_n14, eq40_e714_q, eq40_e710_q_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[14]),
            None,
            &[
                GeneratedDerivative::node(nodes[14], self.multiplicity * (eq40_e716_q_d_n14)),
            ],
        );
    }
}
