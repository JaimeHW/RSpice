#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_18_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq18_e1112, eq18_e1112_d_n0, eq18_e1112_d_n1, eq18_e1112_d_n2, eq18_e1112_d_n3, eq18_e1112_d_n4, eq18_e1112_d_n5, eq18_e1112_d_n6, eq18_e1112_d_n7, eq18_e1112_d_n8, eq18_e1112_d_n9, eq18_e1112_d_n10, eq18_e1112_d_n11, eq18_e1112_d_n12, eq18_e1112_d_n13, eq18_e1112_d_n14, eq18_e1112_d_n15, eq18_e1112_d_n16, eq18_e1112_d_n17, eq18_e1112_d_b0, eq18_e1112_d_b1, eq18_e1112_d_b2, eq18_e1112_d_b3, eq18_e1112_d_b4, eq18_e1112_d_b5, eq18_e1112_d_b6, eq18_e1112_d_b7, eq18_e1112_d_b8,) = {
    if (s.v[3405] != 0.0) {
        let eq18_e1109: f64 = self.eval_ddt(5, s.v[68]);
        let eq18_e1109_d_n0: f64 = self.ddt_jacobian(s.dn[68][0]);
        let eq18_e1109_d_n1: f64 = self.ddt_jacobian(s.dn[68][1]);
        let eq18_e1109_d_n2: f64 = self.ddt_jacobian(s.dn[68][2]);
        let eq18_e1109_d_n3: f64 = self.ddt_jacobian(s.dn[68][3]);
        let eq18_e1109_d_n4: f64 = self.ddt_jacobian(s.dn[68][4]);
        let eq18_e1109_d_n5: f64 = self.ddt_jacobian(s.dn[68][5]);
        let eq18_e1109_d_n6: f64 = self.ddt_jacobian(s.dn[68][6]);
        let eq18_e1109_d_n7: f64 = self.ddt_jacobian(s.dn[68][7]);
        let eq18_e1109_d_n8: f64 = self.ddt_jacobian(s.dn[68][8]);
        let eq18_e1109_d_n9: f64 = self.ddt_jacobian(s.dn[68][9]);
        let eq18_e1109_d_n10: f64 = self.ddt_jacobian(s.dn[68][10]);
        let eq18_e1109_d_n11: f64 = self.ddt_jacobian(s.dn[68][11]);
        let eq18_e1109_d_n12: f64 = self.ddt_jacobian(s.dn[68][12]);
        let eq18_e1109_d_n13: f64 = self.ddt_jacobian(s.dn[68][13]);
        let eq18_e1109_d_n14: f64 = self.ddt_jacobian(s.dn[68][14]);
        let eq18_e1109_d_n15: f64 = self.ddt_jacobian(s.dn[68][15]);
        let eq18_e1109_d_n16: f64 = self.ddt_jacobian(s.dn[68][16]);
        let eq18_e1109_d_n17: f64 = self.ddt_jacobian(s.dn[68][17]);
        let eq18_e1109_d_b0: f64 = self.ddt_jacobian(s.db[68][0]);
        let eq18_e1109_d_b1: f64 = self.ddt_jacobian(s.db[68][1]);
        let eq18_e1109_d_b2: f64 = self.ddt_jacobian(s.db[68][2]);
        let eq18_e1109_d_b3: f64 = self.ddt_jacobian(s.db[68][3]);
        let eq18_e1109_d_b4: f64 = self.ddt_jacobian(s.db[68][4]);
        let eq18_e1109_d_b5: f64 = self.ddt_jacobian(s.db[68][5]);
        let eq18_e1109_d_b6: f64 = self.ddt_jacobian(s.db[68][6]);
        let eq18_e1109_d_b7: f64 = self.ddt_jacobian(s.db[68][7]);
        let eq18_e1109_d_b8: f64 = self.ddt_jacobian(s.db[68][8]);
        let eq18_e1110: f64 = (p.p87 * eq18_e1109);
        let eq18_e1110_d_n0: f64 = (p.p87 * eq18_e1109_d_n0);
        let eq18_e1110_d_n1: f64 = (p.p87 * eq18_e1109_d_n1);
        let eq18_e1110_d_n2: f64 = (p.p87 * eq18_e1109_d_n2);
        let eq18_e1110_d_n3: f64 = (p.p87 * eq18_e1109_d_n3);
        let eq18_e1110_d_n4: f64 = (p.p87 * eq18_e1109_d_n4);
        let eq18_e1110_d_n5: f64 = (p.p87 * eq18_e1109_d_n5);
        let eq18_e1110_d_n6: f64 = (p.p87 * eq18_e1109_d_n6);
        let eq18_e1110_d_n7: f64 = (p.p87 * eq18_e1109_d_n7);
        let eq18_e1110_d_n8: f64 = (p.p87 * eq18_e1109_d_n8);
        let eq18_e1110_d_n9: f64 = (p.p87 * eq18_e1109_d_n9);
        let eq18_e1110_d_n10: f64 = (p.p87 * eq18_e1109_d_n10);
        let eq18_e1110_d_n11: f64 = (p.p87 * eq18_e1109_d_n11);
        let eq18_e1110_d_n12: f64 = (p.p87 * eq18_e1109_d_n12);
        let eq18_e1110_d_n13: f64 = (p.p87 * eq18_e1109_d_n13);
        let eq18_e1110_d_n14: f64 = (p.p87 * eq18_e1109_d_n14);
        let eq18_e1110_d_n15: f64 = (p.p87 * eq18_e1109_d_n15);
        let eq18_e1110_d_n16: f64 = (p.p87 * eq18_e1109_d_n16);
        let eq18_e1110_d_n17: f64 = (p.p87 * eq18_e1109_d_n17);
        let eq18_e1110_d_b0: f64 = (p.p87 * eq18_e1109_d_b0);
        let eq18_e1110_d_b1: f64 = (p.p87 * eq18_e1109_d_b1);
        let eq18_e1110_d_b2: f64 = (p.p87 * eq18_e1109_d_b2);
        let eq18_e1110_d_b3: f64 = (p.p87 * eq18_e1109_d_b3);
        let eq18_e1110_d_b4: f64 = (p.p87 * eq18_e1109_d_b4);
        let eq18_e1110_d_b5: f64 = (p.p87 * eq18_e1109_d_b5);
        let eq18_e1110_d_b6: f64 = (p.p87 * eq18_e1109_d_b6);
        let eq18_e1110_d_b7: f64 = (p.p87 * eq18_e1109_d_b7);
        let eq18_e1110_d_b8: f64 = (p.p87 * eq18_e1109_d_b8);
        (eq18_e1110, eq18_e1110_d_n0, eq18_e1110_d_n1, eq18_e1110_d_n2, eq18_e1110_d_n3, eq18_e1110_d_n4, eq18_e1110_d_n5, eq18_e1110_d_n6, eq18_e1110_d_n7, eq18_e1110_d_n8, eq18_e1110_d_n9, eq18_e1110_d_n10, eq18_e1110_d_n11, eq18_e1110_d_n12, eq18_e1110_d_n13, eq18_e1110_d_n14, eq18_e1110_d_n15, eq18_e1110_d_n16, eq18_e1110_d_n17, eq18_e1110_d_b0, eq18_e1110_d_b1, eq18_e1110_d_b2, eq18_e1110_d_b3, eq18_e1110_d_b4, eq18_e1110_d_b5, eq18_e1110_d_b6, eq18_e1110_d_b7, eq18_e1110_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1112;
        let eq18_node_derivatives: [f64; 18] = [eq18_e1112_d_n0, eq18_e1112_d_n1, eq18_e1112_d_n2, eq18_e1112_d_n3, eq18_e1112_d_n4, eq18_e1112_d_n5, eq18_e1112_d_n6, eq18_e1112_d_n7, eq18_e1112_d_n8, eq18_e1112_d_n9, eq18_e1112_d_n10, eq18_e1112_d_n11, eq18_e1112_d_n12, eq18_e1112_d_n13, eq18_e1112_d_n14, eq18_e1112_d_n15, eq18_e1112_d_n16, eq18_e1112_d_n17];
        let eq18_branch_derivatives: [f64; 9] = [eq18_e1112_d_b0, eq18_e1112_d_b1, eq18_e1112_d_b2, eq18_e1112_d_b3, eq18_e1112_d_b4, eq18_e1112_d_b5, eq18_e1112_d_b6, eq18_e1112_d_b7, eq18_e1112_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            self.multiplicity * (eq18_value),
            &nodes,
            &eq18_node_derivatives,
            &branches,
            &eq18_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_19_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq19_e1119, eq19_e1119_d_n0, eq19_e1119_d_n1, eq19_e1119_d_n2, eq19_e1119_d_n3, eq19_e1119_d_n4, eq19_e1119_d_n5, eq19_e1119_d_n6, eq19_e1119_d_n7, eq19_e1119_d_n8, eq19_e1119_d_n9, eq19_e1119_d_n10, eq19_e1119_d_n11, eq19_e1119_d_n12, eq19_e1119_d_n13, eq19_e1119_d_n14, eq19_e1119_d_n15, eq19_e1119_d_n16, eq19_e1119_d_n17, eq19_e1119_d_b0, eq19_e1119_d_b1, eq19_e1119_d_b2, eq19_e1119_d_b3, eq19_e1119_d_b4, eq19_e1119_d_b5, eq19_e1119_d_b6, eq19_e1119_d_b7, eq19_e1119_d_b8,) = {
    if (s.v[3405] != 0.0) {
        let eq19_e1116: f64 = self.eval_ddt(6, s.v[67]);
        let eq19_e1116_d_n0: f64 = self.ddt_jacobian(s.dn[67][0]);
        let eq19_e1116_d_n1: f64 = self.ddt_jacobian(s.dn[67][1]);
        let eq19_e1116_d_n2: f64 = self.ddt_jacobian(s.dn[67][2]);
        let eq19_e1116_d_n3: f64 = self.ddt_jacobian(s.dn[67][3]);
        let eq19_e1116_d_n4: f64 = self.ddt_jacobian(s.dn[67][4]);
        let eq19_e1116_d_n5: f64 = self.ddt_jacobian(s.dn[67][5]);
        let eq19_e1116_d_n6: f64 = self.ddt_jacobian(s.dn[67][6]);
        let eq19_e1116_d_n7: f64 = self.ddt_jacobian(s.dn[67][7]);
        let eq19_e1116_d_n8: f64 = self.ddt_jacobian(s.dn[67][8]);
        let eq19_e1116_d_n9: f64 = self.ddt_jacobian(s.dn[67][9]);
        let eq19_e1116_d_n10: f64 = self.ddt_jacobian(s.dn[67][10]);
        let eq19_e1116_d_n11: f64 = self.ddt_jacobian(s.dn[67][11]);
        let eq19_e1116_d_n12: f64 = self.ddt_jacobian(s.dn[67][12]);
        let eq19_e1116_d_n13: f64 = self.ddt_jacobian(s.dn[67][13]);
        let eq19_e1116_d_n14: f64 = self.ddt_jacobian(s.dn[67][14]);
        let eq19_e1116_d_n15: f64 = self.ddt_jacobian(s.dn[67][15]);
        let eq19_e1116_d_n16: f64 = self.ddt_jacobian(s.dn[67][16]);
        let eq19_e1116_d_n17: f64 = self.ddt_jacobian(s.dn[67][17]);
        let eq19_e1116_d_b0: f64 = self.ddt_jacobian(s.db[67][0]);
        let eq19_e1116_d_b1: f64 = self.ddt_jacobian(s.db[67][1]);
        let eq19_e1116_d_b2: f64 = self.ddt_jacobian(s.db[67][2]);
        let eq19_e1116_d_b3: f64 = self.ddt_jacobian(s.db[67][3]);
        let eq19_e1116_d_b4: f64 = self.ddt_jacobian(s.db[67][4]);
        let eq19_e1116_d_b5: f64 = self.ddt_jacobian(s.db[67][5]);
        let eq19_e1116_d_b6: f64 = self.ddt_jacobian(s.db[67][6]);
        let eq19_e1116_d_b7: f64 = self.ddt_jacobian(s.db[67][7]);
        let eq19_e1116_d_b8: f64 = self.ddt_jacobian(s.db[67][8]);
        let eq19_e1117: f64 = (p.p87 * eq19_e1116);
        let eq19_e1117_d_n0: f64 = (p.p87 * eq19_e1116_d_n0);
        let eq19_e1117_d_n1: f64 = (p.p87 * eq19_e1116_d_n1);
        let eq19_e1117_d_n2: f64 = (p.p87 * eq19_e1116_d_n2);
        let eq19_e1117_d_n3: f64 = (p.p87 * eq19_e1116_d_n3);
        let eq19_e1117_d_n4: f64 = (p.p87 * eq19_e1116_d_n4);
        let eq19_e1117_d_n5: f64 = (p.p87 * eq19_e1116_d_n5);
        let eq19_e1117_d_n6: f64 = (p.p87 * eq19_e1116_d_n6);
        let eq19_e1117_d_n7: f64 = (p.p87 * eq19_e1116_d_n7);
        let eq19_e1117_d_n8: f64 = (p.p87 * eq19_e1116_d_n8);
        let eq19_e1117_d_n9: f64 = (p.p87 * eq19_e1116_d_n9);
        let eq19_e1117_d_n10: f64 = (p.p87 * eq19_e1116_d_n10);
        let eq19_e1117_d_n11: f64 = (p.p87 * eq19_e1116_d_n11);
        let eq19_e1117_d_n12: f64 = (p.p87 * eq19_e1116_d_n12);
        let eq19_e1117_d_n13: f64 = (p.p87 * eq19_e1116_d_n13);
        let eq19_e1117_d_n14: f64 = (p.p87 * eq19_e1116_d_n14);
        let eq19_e1117_d_n15: f64 = (p.p87 * eq19_e1116_d_n15);
        let eq19_e1117_d_n16: f64 = (p.p87 * eq19_e1116_d_n16);
        let eq19_e1117_d_n17: f64 = (p.p87 * eq19_e1116_d_n17);
        let eq19_e1117_d_b0: f64 = (p.p87 * eq19_e1116_d_b0);
        let eq19_e1117_d_b1: f64 = (p.p87 * eq19_e1116_d_b1);
        let eq19_e1117_d_b2: f64 = (p.p87 * eq19_e1116_d_b2);
        let eq19_e1117_d_b3: f64 = (p.p87 * eq19_e1116_d_b3);
        let eq19_e1117_d_b4: f64 = (p.p87 * eq19_e1116_d_b4);
        let eq19_e1117_d_b5: f64 = (p.p87 * eq19_e1116_d_b5);
        let eq19_e1117_d_b6: f64 = (p.p87 * eq19_e1116_d_b6);
        let eq19_e1117_d_b7: f64 = (p.p87 * eq19_e1116_d_b7);
        let eq19_e1117_d_b8: f64 = (p.p87 * eq19_e1116_d_b8);
        (eq19_e1117, eq19_e1117_d_n0, eq19_e1117_d_n1, eq19_e1117_d_n2, eq19_e1117_d_n3, eq19_e1117_d_n4, eq19_e1117_d_n5, eq19_e1117_d_n6, eq19_e1117_d_n7, eq19_e1117_d_n8, eq19_e1117_d_n9, eq19_e1117_d_n10, eq19_e1117_d_n11, eq19_e1117_d_n12, eq19_e1117_d_n13, eq19_e1117_d_n14, eq19_e1117_d_n15, eq19_e1117_d_n16, eq19_e1117_d_n17, eq19_e1117_d_b0, eq19_e1117_d_b1, eq19_e1117_d_b2, eq19_e1117_d_b3, eq19_e1117_d_b4, eq19_e1117_d_b5, eq19_e1117_d_b6, eq19_e1117_d_b7, eq19_e1117_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_value: f64 = eq19_e1119;
        let eq19_node_derivatives: [f64; 18] = [eq19_e1119_d_n0, eq19_e1119_d_n1, eq19_e1119_d_n2, eq19_e1119_d_n3, eq19_e1119_d_n4, eq19_e1119_d_n5, eq19_e1119_d_n6, eq19_e1119_d_n7, eq19_e1119_d_n8, eq19_e1119_d_n9, eq19_e1119_d_n10, eq19_e1119_d_n11, eq19_e1119_d_n12, eq19_e1119_d_n13, eq19_e1119_d_n14, eq19_e1119_d_n15, eq19_e1119_d_n16, eq19_e1119_d_n17];
        let eq19_branch_derivatives: [f64; 9] = [eq19_e1119_d_b0, eq19_e1119_d_b1, eq19_e1119_d_b2, eq19_e1119_d_b3, eq19_e1119_d_b4, eq19_e1119_d_b5, eq19_e1119_d_b6, eq19_e1119_d_b7, eq19_e1119_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            self.multiplicity * (eq19_value),
            &nodes,
            &eq19_node_derivatives,
            &branches,
            &eq19_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_20_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq20_e1125, eq20_e1125_d_n0, eq20_e1125_d_n1, eq20_e1125_d_n2, eq20_e1125_d_n3, eq20_e1125_d_n4, eq20_e1125_d_n5, eq20_e1125_d_n6, eq20_e1125_d_n7, eq20_e1125_d_n8, eq20_e1125_d_n9, eq20_e1125_d_n10, eq20_e1125_d_n11, eq20_e1125_d_n12, eq20_e1125_d_n13, eq20_e1125_d_n14, eq20_e1125_d_n15, eq20_e1125_d_n16, eq20_e1125_d_n17, eq20_e1125_d_b0, eq20_e1125_d_b1, eq20_e1125_d_b2, eq20_e1125_d_b3, eq20_e1125_d_b4, eq20_e1125_d_b5, eq20_e1125_d_b6, eq20_e1125_d_b7, eq20_e1125_d_b8,) = {
    if (s.v[3406] != 0.0) {
        let eq20_e1123: f64 = (p.p87 * s.v[200]);
        let eq20_e1123_d_n0: f64 = (p.p87 * s.dn[200][0]);
        let eq20_e1123_d_n1: f64 = (p.p87 * s.dn[200][1]);
        let eq20_e1123_d_n2: f64 = (p.p87 * s.dn[200][2]);
        let eq20_e1123_d_n3: f64 = (p.p87 * s.dn[200][3]);
        let eq20_e1123_d_n4: f64 = (p.p87 * s.dn[200][4]);
        let eq20_e1123_d_n5: f64 = (p.p87 * s.dn[200][5]);
        let eq20_e1123_d_n6: f64 = (p.p87 * s.dn[200][6]);
        let eq20_e1123_d_n7: f64 = (p.p87 * s.dn[200][7]);
        let eq20_e1123_d_n8: f64 = (p.p87 * s.dn[200][8]);
        let eq20_e1123_d_n9: f64 = (p.p87 * s.dn[200][9]);
        let eq20_e1123_d_n10: f64 = (p.p87 * s.dn[200][10]);
        let eq20_e1123_d_n11: f64 = (p.p87 * s.dn[200][11]);
        let eq20_e1123_d_n12: f64 = (p.p87 * s.dn[200][12]);
        let eq20_e1123_d_n13: f64 = (p.p87 * s.dn[200][13]);
        let eq20_e1123_d_n14: f64 = (p.p87 * s.dn[200][14]);
        let eq20_e1123_d_n15: f64 = (p.p87 * s.dn[200][15]);
        let eq20_e1123_d_n16: f64 = (p.p87 * s.dn[200][16]);
        let eq20_e1123_d_n17: f64 = (p.p87 * s.dn[200][17]);
        let eq20_e1123_d_b0: f64 = (p.p87 * s.db[200][0]);
        let eq20_e1123_d_b1: f64 = (p.p87 * s.db[200][1]);
        let eq20_e1123_d_b2: f64 = (p.p87 * s.db[200][2]);
        let eq20_e1123_d_b3: f64 = (p.p87 * s.db[200][3]);
        let eq20_e1123_d_b4: f64 = (p.p87 * s.db[200][4]);
        let eq20_e1123_d_b5: f64 = (p.p87 * s.db[200][5]);
        let eq20_e1123_d_b6: f64 = (p.p87 * s.db[200][6]);
        let eq20_e1123_d_b7: f64 = (p.p87 * s.db[200][7]);
        let eq20_e1123_d_b8: f64 = (p.p87 * s.db[200][8]);
        (eq20_e1123, eq20_e1123_d_n0, eq20_e1123_d_n1, eq20_e1123_d_n2, eq20_e1123_d_n3, eq20_e1123_d_n4, eq20_e1123_d_n5, eq20_e1123_d_n6, eq20_e1123_d_n7, eq20_e1123_d_n8, eq20_e1123_d_n9, eq20_e1123_d_n10, eq20_e1123_d_n11, eq20_e1123_d_n12, eq20_e1123_d_n13, eq20_e1123_d_n14, eq20_e1123_d_n15, eq20_e1123_d_n16, eq20_e1123_d_n17, eq20_e1123_d_b0, eq20_e1123_d_b1, eq20_e1123_d_b2, eq20_e1123_d_b3, eq20_e1123_d_b4, eq20_e1123_d_b5, eq20_e1123_d_b6, eq20_e1123_d_b7, eq20_e1123_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e1125;
        let eq20_node_derivatives: [f64; 18] = [eq20_e1125_d_n0, eq20_e1125_d_n1, eq20_e1125_d_n2, eq20_e1125_d_n3, eq20_e1125_d_n4, eq20_e1125_d_n5, eq20_e1125_d_n6, eq20_e1125_d_n7, eq20_e1125_d_n8, eq20_e1125_d_n9, eq20_e1125_d_n10, eq20_e1125_d_n11, eq20_e1125_d_n12, eq20_e1125_d_n13, eq20_e1125_d_n14, eq20_e1125_d_n15, eq20_e1125_d_n16, eq20_e1125_d_n17];
        let eq20_branch_derivatives: [f64; 9] = [eq20_e1125_d_b0, eq20_e1125_d_b1, eq20_e1125_d_b2, eq20_e1125_d_b3, eq20_e1125_d_b4, eq20_e1125_d_b5, eq20_e1125_d_b6, eq20_e1125_d_b7, eq20_e1125_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq20_value),
            &nodes,
            &eq20_node_derivatives,
            &branches,
            &eq20_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_21_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq21_e1131, eq21_e1131_d_n0, eq21_e1131_d_n1, eq21_e1131_d_n2, eq21_e1131_d_n3, eq21_e1131_d_n4, eq21_e1131_d_n5, eq21_e1131_d_n6, eq21_e1131_d_n7, eq21_e1131_d_n8, eq21_e1131_d_n9, eq21_e1131_d_n10, eq21_e1131_d_n11, eq21_e1131_d_n12, eq21_e1131_d_n13, eq21_e1131_d_n14, eq21_e1131_d_n15, eq21_e1131_d_n16, eq21_e1131_d_n17, eq21_e1131_d_b0, eq21_e1131_d_b1, eq21_e1131_d_b2, eq21_e1131_d_b3, eq21_e1131_d_b4, eq21_e1131_d_b5, eq21_e1131_d_b6, eq21_e1131_d_b7, eq21_e1131_d_b8,) = {
    if (s.v[3406] != 0.0) {
        let eq21_e1129: f64 = (p.p87 * s.v[201]);
        let eq21_e1129_d_n0: f64 = (p.p87 * s.dn[201][0]);
        let eq21_e1129_d_n1: f64 = (p.p87 * s.dn[201][1]);
        let eq21_e1129_d_n2: f64 = (p.p87 * s.dn[201][2]);
        let eq21_e1129_d_n3: f64 = (p.p87 * s.dn[201][3]);
        let eq21_e1129_d_n4: f64 = (p.p87 * s.dn[201][4]);
        let eq21_e1129_d_n5: f64 = (p.p87 * s.dn[201][5]);
        let eq21_e1129_d_n6: f64 = (p.p87 * s.dn[201][6]);
        let eq21_e1129_d_n7: f64 = (p.p87 * s.dn[201][7]);
        let eq21_e1129_d_n8: f64 = (p.p87 * s.dn[201][8]);
        let eq21_e1129_d_n9: f64 = (p.p87 * s.dn[201][9]);
        let eq21_e1129_d_n10: f64 = (p.p87 * s.dn[201][10]);
        let eq21_e1129_d_n11: f64 = (p.p87 * s.dn[201][11]);
        let eq21_e1129_d_n12: f64 = (p.p87 * s.dn[201][12]);
        let eq21_e1129_d_n13: f64 = (p.p87 * s.dn[201][13]);
        let eq21_e1129_d_n14: f64 = (p.p87 * s.dn[201][14]);
        let eq21_e1129_d_n15: f64 = (p.p87 * s.dn[201][15]);
        let eq21_e1129_d_n16: f64 = (p.p87 * s.dn[201][16]);
        let eq21_e1129_d_n17: f64 = (p.p87 * s.dn[201][17]);
        let eq21_e1129_d_b0: f64 = (p.p87 * s.db[201][0]);
        let eq21_e1129_d_b1: f64 = (p.p87 * s.db[201][1]);
        let eq21_e1129_d_b2: f64 = (p.p87 * s.db[201][2]);
        let eq21_e1129_d_b3: f64 = (p.p87 * s.db[201][3]);
        let eq21_e1129_d_b4: f64 = (p.p87 * s.db[201][4]);
        let eq21_e1129_d_b5: f64 = (p.p87 * s.db[201][5]);
        let eq21_e1129_d_b6: f64 = (p.p87 * s.db[201][6]);
        let eq21_e1129_d_b7: f64 = (p.p87 * s.db[201][7]);
        let eq21_e1129_d_b8: f64 = (p.p87 * s.db[201][8]);
        (eq21_e1129, eq21_e1129_d_n0, eq21_e1129_d_n1, eq21_e1129_d_n2, eq21_e1129_d_n3, eq21_e1129_d_n4, eq21_e1129_d_n5, eq21_e1129_d_n6, eq21_e1129_d_n7, eq21_e1129_d_n8, eq21_e1129_d_n9, eq21_e1129_d_n10, eq21_e1129_d_n11, eq21_e1129_d_n12, eq21_e1129_d_n13, eq21_e1129_d_n14, eq21_e1129_d_n15, eq21_e1129_d_n16, eq21_e1129_d_n17, eq21_e1129_d_b0, eq21_e1129_d_b1, eq21_e1129_d_b2, eq21_e1129_d_b3, eq21_e1129_d_b4, eq21_e1129_d_b5, eq21_e1129_d_b6, eq21_e1129_d_b7, eq21_e1129_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1131;
        let eq21_node_derivatives: [f64; 18] = [eq21_e1131_d_n0, eq21_e1131_d_n1, eq21_e1131_d_n2, eq21_e1131_d_n3, eq21_e1131_d_n4, eq21_e1131_d_n5, eq21_e1131_d_n6, eq21_e1131_d_n7, eq21_e1131_d_n8, eq21_e1131_d_n9, eq21_e1131_d_n10, eq21_e1131_d_n11, eq21_e1131_d_n12, eq21_e1131_d_n13, eq21_e1131_d_n14, eq21_e1131_d_n15, eq21_e1131_d_n16, eq21_e1131_d_n17];
        let eq21_branch_derivatives: [f64; 9] = [eq21_e1131_d_b0, eq21_e1131_d_b1, eq21_e1131_d_b2, eq21_e1131_d_b3, eq21_e1131_d_b4, eq21_e1131_d_b5, eq21_e1131_d_b6, eq21_e1131_d_b7, eq21_e1131_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[5]),
            self.multiplicity * (eq21_value),
            &nodes,
            &eq21_node_derivatives,
            &branches,
            &eq21_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_22_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq22_e1137, eq22_e1137_d_n0, eq22_e1137_d_n1, eq22_e1137_d_n2, eq22_e1137_d_n3, eq22_e1137_d_n4, eq22_e1137_d_n5, eq22_e1137_d_n6, eq22_e1137_d_n7, eq22_e1137_d_n8, eq22_e1137_d_n9, eq22_e1137_d_n10, eq22_e1137_d_n11, eq22_e1137_d_n12, eq22_e1137_d_n13, eq22_e1137_d_n14, eq22_e1137_d_n15, eq22_e1137_d_n16, eq22_e1137_d_n17, eq22_e1137_d_b0, eq22_e1137_d_b1, eq22_e1137_d_b2, eq22_e1137_d_b3, eq22_e1137_d_b4, eq22_e1137_d_b5, eq22_e1137_d_b6, eq22_e1137_d_b7, eq22_e1137_d_b8,) = {
    if (s.v[3406] != 0.0) {
        let eq22_e1135: f64 = (p.p87 * s.v[202]);
        let eq22_e1135_d_n0: f64 = (p.p87 * s.dn[202][0]);
        let eq22_e1135_d_n1: f64 = (p.p87 * s.dn[202][1]);
        let eq22_e1135_d_n2: f64 = (p.p87 * s.dn[202][2]);
        let eq22_e1135_d_n3: f64 = (p.p87 * s.dn[202][3]);
        let eq22_e1135_d_n4: f64 = (p.p87 * s.dn[202][4]);
        let eq22_e1135_d_n5: f64 = (p.p87 * s.dn[202][5]);
        let eq22_e1135_d_n6: f64 = (p.p87 * s.dn[202][6]);
        let eq22_e1135_d_n7: f64 = (p.p87 * s.dn[202][7]);
        let eq22_e1135_d_n8: f64 = (p.p87 * s.dn[202][8]);
        let eq22_e1135_d_n9: f64 = (p.p87 * s.dn[202][9]);
        let eq22_e1135_d_n10: f64 = (p.p87 * s.dn[202][10]);
        let eq22_e1135_d_n11: f64 = (p.p87 * s.dn[202][11]);
        let eq22_e1135_d_n12: f64 = (p.p87 * s.dn[202][12]);
        let eq22_e1135_d_n13: f64 = (p.p87 * s.dn[202][13]);
        let eq22_e1135_d_n14: f64 = (p.p87 * s.dn[202][14]);
        let eq22_e1135_d_n15: f64 = (p.p87 * s.dn[202][15]);
        let eq22_e1135_d_n16: f64 = (p.p87 * s.dn[202][16]);
        let eq22_e1135_d_n17: f64 = (p.p87 * s.dn[202][17]);
        let eq22_e1135_d_b0: f64 = (p.p87 * s.db[202][0]);
        let eq22_e1135_d_b1: f64 = (p.p87 * s.db[202][1]);
        let eq22_e1135_d_b2: f64 = (p.p87 * s.db[202][2]);
        let eq22_e1135_d_b3: f64 = (p.p87 * s.db[202][3]);
        let eq22_e1135_d_b4: f64 = (p.p87 * s.db[202][4]);
        let eq22_e1135_d_b5: f64 = (p.p87 * s.db[202][5]);
        let eq22_e1135_d_b6: f64 = (p.p87 * s.db[202][6]);
        let eq22_e1135_d_b7: f64 = (p.p87 * s.db[202][7]);
        let eq22_e1135_d_b8: f64 = (p.p87 * s.db[202][8]);
        (eq22_e1135, eq22_e1135_d_n0, eq22_e1135_d_n1, eq22_e1135_d_n2, eq22_e1135_d_n3, eq22_e1135_d_n4, eq22_e1135_d_n5, eq22_e1135_d_n6, eq22_e1135_d_n7, eq22_e1135_d_n8, eq22_e1135_d_n9, eq22_e1135_d_n10, eq22_e1135_d_n11, eq22_e1135_d_n12, eq22_e1135_d_n13, eq22_e1135_d_n14, eq22_e1135_d_n15, eq22_e1135_d_n16, eq22_e1135_d_n17, eq22_e1135_d_b0, eq22_e1135_d_b1, eq22_e1135_d_b2, eq22_e1135_d_b3, eq22_e1135_d_b4, eq22_e1135_d_b5, eq22_e1135_d_b6, eq22_e1135_d_b7, eq22_e1135_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq22_value: f64 = eq22_e1137;
        let eq22_node_derivatives: [f64; 18] = [eq22_e1137_d_n0, eq22_e1137_d_n1, eq22_e1137_d_n2, eq22_e1137_d_n3, eq22_e1137_d_n4, eq22_e1137_d_n5, eq22_e1137_d_n6, eq22_e1137_d_n7, eq22_e1137_d_n8, eq22_e1137_d_n9, eq22_e1137_d_n10, eq22_e1137_d_n11, eq22_e1137_d_n12, eq22_e1137_d_n13, eq22_e1137_d_n14, eq22_e1137_d_n15, eq22_e1137_d_n16, eq22_e1137_d_n17];
        let eq22_branch_derivatives: [f64; 9] = [eq22_e1137_d_b0, eq22_e1137_d_b1, eq22_e1137_d_b2, eq22_e1137_d_b3, eq22_e1137_d_b4, eq22_e1137_d_b5, eq22_e1137_d_b6, eq22_e1137_d_b7, eq22_e1137_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            self.multiplicity * (eq22_value),
            &nodes,
            &eq22_node_derivatives,
            &branches,
            &eq22_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_23_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq23_e1143, eq23_e1143_d_n0, eq23_e1143_d_n1, eq23_e1143_d_n2, eq23_e1143_d_n3, eq23_e1143_d_n4, eq23_e1143_d_n5, eq23_e1143_d_n6, eq23_e1143_d_n7, eq23_e1143_d_n8, eq23_e1143_d_n9, eq23_e1143_d_n10, eq23_e1143_d_n11, eq23_e1143_d_n12, eq23_e1143_d_n13, eq23_e1143_d_n14, eq23_e1143_d_n15, eq23_e1143_d_n16, eq23_e1143_d_n17, eq23_e1143_d_b0, eq23_e1143_d_b1, eq23_e1143_d_b2, eq23_e1143_d_b3, eq23_e1143_d_b4, eq23_e1143_d_b5, eq23_e1143_d_b6, eq23_e1143_d_b7, eq23_e1143_d_b8,) = {
    if (s.v[75] != 0.0) {
        let eq23_e1141: f64 = ((nv0 - nv5) / s.v[4]);
        let eq23_e1141_d_n0: f64 = ((s.v[4] - ((nv0 - nv5) * s.dn[4][0])) / (s.v[4] * s.v[4]));
        let eq23_e1141_d_n1: f64 = (-(((nv0 - nv5) * s.dn[4][1]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n2: f64 = (-(((nv0 - nv5) * s.dn[4][2]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n3: f64 = (-(((nv0 - nv5) * s.dn[4][3]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n4: f64 = (-(((nv0 - nv5) * s.dn[4][4]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n5: f64 = (((-s.v[4]) - ((nv0 - nv5) * s.dn[4][5])) / (s.v[4] * s.v[4]));
        let eq23_e1141_d_n6: f64 = (-(((nv0 - nv5) * s.dn[4][6]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n7: f64 = (-(((nv0 - nv5) * s.dn[4][7]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n8: f64 = (-(((nv0 - nv5) * s.dn[4][8]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n9: f64 = (-(((nv0 - nv5) * s.dn[4][9]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n10: f64 = (-(((nv0 - nv5) * s.dn[4][10]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n11: f64 = (-(((nv0 - nv5) * s.dn[4][11]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n12: f64 = (-(((nv0 - nv5) * s.dn[4][12]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n13: f64 = (-(((nv0 - nv5) * s.dn[4][13]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n14: f64 = (-(((nv0 - nv5) * s.dn[4][14]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n15: f64 = (-(((nv0 - nv5) * s.dn[4][15]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n16: f64 = (-(((nv0 - nv5) * s.dn[4][16]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n17: f64 = (-(((nv0 - nv5) * s.dn[4][17]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b0: f64 = (-(((nv0 - nv5) * s.db[4][0]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b1: f64 = (-(((nv0 - nv5) * s.db[4][1]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b2: f64 = (-(((nv0 - nv5) * s.db[4][2]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b3: f64 = (-(((nv0 - nv5) * s.db[4][3]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b4: f64 = (-(((nv0 - nv5) * s.db[4][4]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b5: f64 = (-(((nv0 - nv5) * s.db[4][5]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b6: f64 = (-(((nv0 - nv5) * s.db[4][6]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b7: f64 = (-(((nv0 - nv5) * s.db[4][7]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b8: f64 = (-(((nv0 - nv5) * s.db[4][8]) / (s.v[4] * s.v[4])));
        (eq23_e1141, eq23_e1141_d_n0, eq23_e1141_d_n1, eq23_e1141_d_n2, eq23_e1141_d_n3, eq23_e1141_d_n4, eq23_e1141_d_n5, eq23_e1141_d_n6, eq23_e1141_d_n7, eq23_e1141_d_n8, eq23_e1141_d_n9, eq23_e1141_d_n10, eq23_e1141_d_n11, eq23_e1141_d_n12, eq23_e1141_d_n13, eq23_e1141_d_n14, eq23_e1141_d_n15, eq23_e1141_d_n16, eq23_e1141_d_n17, eq23_e1141_d_b0, eq23_e1141_d_b1, eq23_e1141_d_b2, eq23_e1141_d_b3, eq23_e1141_d_b4, eq23_e1141_d_b5, eq23_e1141_d_b6, eq23_e1141_d_b7, eq23_e1141_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e1143;
        let eq23_node_derivatives: [f64; 18] = [eq23_e1143_d_n0, eq23_e1143_d_n1, eq23_e1143_d_n2, eq23_e1143_d_n3, eq23_e1143_d_n4, eq23_e1143_d_n5, eq23_e1143_d_n6, eq23_e1143_d_n7, eq23_e1143_d_n8, eq23_e1143_d_n9, eq23_e1143_d_n10, eq23_e1143_d_n11, eq23_e1143_d_n12, eq23_e1143_d_n13, eq23_e1143_d_n14, eq23_e1143_d_n15, eq23_e1143_d_n16, eq23_e1143_d_n17];
        let eq23_branch_derivatives: [f64; 9] = [eq23_e1143_d_b0, eq23_e1143_d_b1, eq23_e1143_d_b2, eq23_e1143_d_b3, eq23_e1143_d_b4, eq23_e1143_d_b5, eq23_e1143_d_b6, eq23_e1143_d_b7, eq23_e1143_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[5]),
            self.multiplicity * (eq23_value),
            &nodes,
            &eq23_node_derivatives,
            &branches,
            &eq23_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_24_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq24_e1148,) = {
    if (!(s.v[75] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq24_value: f64 = eq24_e1148;
        stamper.stamp_potential(
            branches[3],
            eq24_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_25_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq25_e1154, eq25_e1154_d_n0, eq25_e1154_d_n1, eq25_e1154_d_n2, eq25_e1154_d_n3, eq25_e1154_d_n4, eq25_e1154_d_n5, eq25_e1154_d_n6, eq25_e1154_d_n7, eq25_e1154_d_n8, eq25_e1154_d_n9, eq25_e1154_d_n10, eq25_e1154_d_n11, eq25_e1154_d_n12, eq25_e1154_d_n13, eq25_e1154_d_n14, eq25_e1154_d_n15, eq25_e1154_d_n16, eq25_e1154_d_n17, eq25_e1154_d_b0, eq25_e1154_d_b1, eq25_e1154_d_b2, eq25_e1154_d_b3, eq25_e1154_d_b4, eq25_e1154_d_b5, eq25_e1154_d_b6, eq25_e1154_d_b7, eq25_e1154_d_b8,) = {
    if (s.v[76] != 0.0) {
        let eq25_e1152: f64 = ((nv7 - nv2) / s.v[5]);
        let eq25_e1152_d_n0: f64 = (-(((nv7 - nv2) * s.dn[5][0]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n1: f64 = (-(((nv7 - nv2) * s.dn[5][1]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n2: f64 = (((-s.v[5]) - ((nv7 - nv2) * s.dn[5][2])) / (s.v[5] * s.v[5]));
        let eq25_e1152_d_n3: f64 = (-(((nv7 - nv2) * s.dn[5][3]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n4: f64 = (-(((nv7 - nv2) * s.dn[5][4]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n5: f64 = (-(((nv7 - nv2) * s.dn[5][5]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n6: f64 = (-(((nv7 - nv2) * s.dn[5][6]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n7: f64 = ((s.v[5] - ((nv7 - nv2) * s.dn[5][7])) / (s.v[5] * s.v[5]));
        let eq25_e1152_d_n8: f64 = (-(((nv7 - nv2) * s.dn[5][8]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n9: f64 = (-(((nv7 - nv2) * s.dn[5][9]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n10: f64 = (-(((nv7 - nv2) * s.dn[5][10]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n11: f64 = (-(((nv7 - nv2) * s.dn[5][11]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n12: f64 = (-(((nv7 - nv2) * s.dn[5][12]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n13: f64 = (-(((nv7 - nv2) * s.dn[5][13]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n14: f64 = (-(((nv7 - nv2) * s.dn[5][14]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n15: f64 = (-(((nv7 - nv2) * s.dn[5][15]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n16: f64 = (-(((nv7 - nv2) * s.dn[5][16]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n17: f64 = (-(((nv7 - nv2) * s.dn[5][17]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b0: f64 = (-(((nv7 - nv2) * s.db[5][0]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b1: f64 = (-(((nv7 - nv2) * s.db[5][1]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b2: f64 = (-(((nv7 - nv2) * s.db[5][2]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b3: f64 = (-(((nv7 - nv2) * s.db[5][3]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b4: f64 = (-(((nv7 - nv2) * s.db[5][4]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b5: f64 = (-(((nv7 - nv2) * s.db[5][5]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b6: f64 = (-(((nv7 - nv2) * s.db[5][6]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b7: f64 = (-(((nv7 - nv2) * s.db[5][7]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b8: f64 = (-(((nv7 - nv2) * s.db[5][8]) / (s.v[5] * s.v[5])));
        (eq25_e1152, eq25_e1152_d_n0, eq25_e1152_d_n1, eq25_e1152_d_n2, eq25_e1152_d_n3, eq25_e1152_d_n4, eq25_e1152_d_n5, eq25_e1152_d_n6, eq25_e1152_d_n7, eq25_e1152_d_n8, eq25_e1152_d_n9, eq25_e1152_d_n10, eq25_e1152_d_n11, eq25_e1152_d_n12, eq25_e1152_d_n13, eq25_e1152_d_n14, eq25_e1152_d_n15, eq25_e1152_d_n16, eq25_e1152_d_n17, eq25_e1152_d_b0, eq25_e1152_d_b1, eq25_e1152_d_b2, eq25_e1152_d_b3, eq25_e1152_d_b4, eq25_e1152_d_b5, eq25_e1152_d_b6, eq25_e1152_d_b7, eq25_e1152_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e1154;
        let eq25_node_derivatives: [f64; 18] = [eq25_e1154_d_n0, eq25_e1154_d_n1, eq25_e1154_d_n2, eq25_e1154_d_n3, eq25_e1154_d_n4, eq25_e1154_d_n5, eq25_e1154_d_n6, eq25_e1154_d_n7, eq25_e1154_d_n8, eq25_e1154_d_n9, eq25_e1154_d_n10, eq25_e1154_d_n11, eq25_e1154_d_n12, eq25_e1154_d_n13, eq25_e1154_d_n14, eq25_e1154_d_n15, eq25_e1154_d_n16, eq25_e1154_d_n17];
        let eq25_branch_derivatives: [f64; 9] = [eq25_e1154_d_b0, eq25_e1154_d_b1, eq25_e1154_d_b2, eq25_e1154_d_b3, eq25_e1154_d_b4, eq25_e1154_d_b5, eq25_e1154_d_b6, eq25_e1154_d_b7, eq25_e1154_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[2]),
            self.multiplicity * (eq25_value),
            &nodes,
            &eq25_node_derivatives,
            &branches,
            &eq25_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_26_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq26_e1159,) = {
    if (!(s.v[76] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e1159;
        stamper.stamp_potential(
            branches[4],
            eq26_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_27_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq27_e1163: f64 = (s.v[18] + s.v[753]);
        let eq27_e1163_d_n0: f64 = (s.dn[18][0] + s.dn[753][0]);
        let eq27_e1163_d_n1: f64 = (s.dn[18][1] + s.dn[753][1]);
        let eq27_e1163_d_n2: f64 = (s.dn[18][2] + s.dn[753][2]);
        let eq27_e1163_d_n3: f64 = (s.dn[18][3] + s.dn[753][3]);
        let eq27_e1163_d_n4: f64 = (s.dn[18][4] + s.dn[753][4]);
        let eq27_e1163_d_n5: f64 = (s.dn[18][5] + s.dn[753][5]);
        let eq27_e1163_d_n6: f64 = (s.dn[18][6] + s.dn[753][6]);
        let eq27_e1163_d_n7: f64 = (s.dn[18][7] + s.dn[753][7]);
        let eq27_e1163_d_n8: f64 = (s.dn[18][8] + s.dn[753][8]);
        let eq27_e1163_d_n9: f64 = (s.dn[18][9] + s.dn[753][9]);
        let eq27_e1163_d_n10: f64 = (s.dn[18][10] + s.dn[753][10]);
        let eq27_e1163_d_n11: f64 = (s.dn[18][11] + s.dn[753][11]);
        let eq27_e1163_d_n12: f64 = (s.dn[18][12] + s.dn[753][12]);
        let eq27_e1163_d_n13: f64 = (s.dn[18][13] + s.dn[753][13]);
        let eq27_e1163_d_n14: f64 = (s.dn[18][14] + s.dn[753][14]);
        let eq27_e1163_d_n15: f64 = (s.dn[18][15] + s.dn[753][15]);
        let eq27_e1163_d_n16: f64 = (s.dn[18][16] + s.dn[753][16]);
        let eq27_e1163_d_n17: f64 = (s.dn[18][17] + s.dn[753][17]);
        let eq27_e1163_d_b0: f64 = (s.db[18][0] + s.db[753][0]);
        let eq27_e1163_d_b1: f64 = (s.db[18][1] + s.db[753][1]);
        let eq27_e1163_d_b2: f64 = (s.db[18][2] + s.db[753][2]);
        let eq27_e1163_d_b3: f64 = (s.db[18][3] + s.db[753][3]);
        let eq27_e1163_d_b4: f64 = (s.db[18][4] + s.db[753][4]);
        let eq27_e1163_d_b5: f64 = (s.db[18][5] + s.db[753][5]);
        let eq27_e1163_d_b6: f64 = (s.db[18][6] + s.db[753][6]);
        let eq27_e1163_d_b7: f64 = (s.db[18][7] + s.db[753][7]);
        let eq27_e1163_d_b8: f64 = (s.db[18][8] + s.db[753][8]);
        let eq27_e1164: f64 = self.eval_ddt(7, eq27_e1163);
        let eq27_e1164_d_n0: f64 = self.ddt_jacobian(eq27_e1163_d_n0);
        let eq27_e1164_d_n1: f64 = self.ddt_jacobian(eq27_e1163_d_n1);
        let eq27_e1164_d_n2: f64 = self.ddt_jacobian(eq27_e1163_d_n2);
        let eq27_e1164_d_n3: f64 = self.ddt_jacobian(eq27_e1163_d_n3);
        let eq27_e1164_d_n4: f64 = self.ddt_jacobian(eq27_e1163_d_n4);
        let eq27_e1164_d_n5: f64 = self.ddt_jacobian(eq27_e1163_d_n5);
        let eq27_e1164_d_n6: f64 = self.ddt_jacobian(eq27_e1163_d_n6);
        let eq27_e1164_d_n7: f64 = self.ddt_jacobian(eq27_e1163_d_n7);
        let eq27_e1164_d_n8: f64 = self.ddt_jacobian(eq27_e1163_d_n8);
        let eq27_e1164_d_n9: f64 = self.ddt_jacobian(eq27_e1163_d_n9);
        let eq27_e1164_d_n10: f64 = self.ddt_jacobian(eq27_e1163_d_n10);
        let eq27_e1164_d_n11: f64 = self.ddt_jacobian(eq27_e1163_d_n11);
        let eq27_e1164_d_n12: f64 = self.ddt_jacobian(eq27_e1163_d_n12);
        let eq27_e1164_d_n13: f64 = self.ddt_jacobian(eq27_e1163_d_n13);
        let eq27_e1164_d_n14: f64 = self.ddt_jacobian(eq27_e1163_d_n14);
        let eq27_e1164_d_n15: f64 = self.ddt_jacobian(eq27_e1163_d_n15);
        let eq27_e1164_d_n16: f64 = self.ddt_jacobian(eq27_e1163_d_n16);
        let eq27_e1164_d_n17: f64 = self.ddt_jacobian(eq27_e1163_d_n17);
        let eq27_e1164_d_b0: f64 = self.ddt_jacobian(eq27_e1163_d_b0);
        let eq27_e1164_d_b1: f64 = self.ddt_jacobian(eq27_e1163_d_b1);
        let eq27_e1164_d_b2: f64 = self.ddt_jacobian(eq27_e1163_d_b2);
        let eq27_e1164_d_b3: f64 = self.ddt_jacobian(eq27_e1163_d_b3);
        let eq27_e1164_d_b4: f64 = self.ddt_jacobian(eq27_e1163_d_b4);
        let eq27_e1164_d_b5: f64 = self.ddt_jacobian(eq27_e1163_d_b5);
        let eq27_e1164_d_b6: f64 = self.ddt_jacobian(eq27_e1163_d_b6);
        let eq27_e1164_d_b7: f64 = self.ddt_jacobian(eq27_e1163_d_b7);
        let eq27_e1164_d_b8: f64 = self.ddt_jacobian(eq27_e1163_d_b8);
        let eq27_e1165: f64 = (p.p87 * eq27_e1164);
        let eq27_e1165_d_n0: f64 = (p.p87 * eq27_e1164_d_n0);
        let eq27_e1165_d_n1: f64 = (p.p87 * eq27_e1164_d_n1);
        let eq27_e1165_d_n2: f64 = (p.p87 * eq27_e1164_d_n2);
        let eq27_e1165_d_n3: f64 = (p.p87 * eq27_e1164_d_n3);
        let eq27_e1165_d_n4: f64 = (p.p87 * eq27_e1164_d_n4);
        let eq27_e1165_d_n5: f64 = (p.p87 * eq27_e1164_d_n5);
        let eq27_e1165_d_n6: f64 = (p.p87 * eq27_e1164_d_n6);
        let eq27_e1165_d_n7: f64 = (p.p87 * eq27_e1164_d_n7);
        let eq27_e1165_d_n8: f64 = (p.p87 * eq27_e1164_d_n8);
        let eq27_e1165_d_n9: f64 = (p.p87 * eq27_e1164_d_n9);
        let eq27_e1165_d_n10: f64 = (p.p87 * eq27_e1164_d_n10);
        let eq27_e1165_d_n11: f64 = (p.p87 * eq27_e1164_d_n11);
        let eq27_e1165_d_n12: f64 = (p.p87 * eq27_e1164_d_n12);
        let eq27_e1165_d_n13: f64 = (p.p87 * eq27_e1164_d_n13);
        let eq27_e1165_d_n14: f64 = (p.p87 * eq27_e1164_d_n14);
        let eq27_e1165_d_n15: f64 = (p.p87 * eq27_e1164_d_n15);
        let eq27_e1165_d_n16: f64 = (p.p87 * eq27_e1164_d_n16);
        let eq27_e1165_d_n17: f64 = (p.p87 * eq27_e1164_d_n17);
        let eq27_e1165_d_b0: f64 = (p.p87 * eq27_e1164_d_b0);
        let eq27_e1165_d_b1: f64 = (p.p87 * eq27_e1164_d_b1);
        let eq27_e1165_d_b2: f64 = (p.p87 * eq27_e1164_d_b2);
        let eq27_e1165_d_b3: f64 = (p.p87 * eq27_e1164_d_b3);
        let eq27_e1165_d_b4: f64 = (p.p87 * eq27_e1164_d_b4);
        let eq27_e1165_d_b5: f64 = (p.p87 * eq27_e1164_d_b5);
        let eq27_e1165_d_b6: f64 = (p.p87 * eq27_e1164_d_b6);
        let eq27_e1165_d_b7: f64 = (p.p87 * eq27_e1164_d_b7);
        let eq27_e1165_d_b8: f64 = (p.p87 * eq27_e1164_d_b8);
        let eq27_value: f64 = eq27_e1165;
        let eq27_node_derivatives: [f64; 18] = [eq27_e1165_d_n0, eq27_e1165_d_n1, eq27_e1165_d_n2, eq27_e1165_d_n3, eq27_e1165_d_n4, eq27_e1165_d_n5, eq27_e1165_d_n6, eq27_e1165_d_n7, eq27_e1165_d_n8, eq27_e1165_d_n9, eq27_e1165_d_n10, eq27_e1165_d_n11, eq27_e1165_d_n12, eq27_e1165_d_n13, eq27_e1165_d_n14, eq27_e1165_d_n15, eq27_e1165_d_n16, eq27_e1165_d_n17];
        let eq27_branch_derivatives: [f64; 9] = [eq27_e1165_d_b0, eq27_e1165_d_b1, eq27_e1165_d_b2, eq27_e1165_d_b3, eq27_e1165_d_b4, eq27_e1165_d_b5, eq27_e1165_d_b6, eq27_e1165_d_b7, eq27_e1165_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq27_value),
            &nodes,
            &eq27_node_derivatives,
            &branches,
            &eq27_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_28_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq28_e1169: f64 = (s.v[19] + s.v[751]);
        let eq28_e1169_d_n0: f64 = (s.dn[19][0] + s.dn[751][0]);
        let eq28_e1169_d_n1: f64 = (s.dn[19][1] + s.dn[751][1]);
        let eq28_e1169_d_n2: f64 = (s.dn[19][2] + s.dn[751][2]);
        let eq28_e1169_d_n3: f64 = (s.dn[19][3] + s.dn[751][3]);
        let eq28_e1169_d_n4: f64 = (s.dn[19][4] + s.dn[751][4]);
        let eq28_e1169_d_n5: f64 = (s.dn[19][5] + s.dn[751][5]);
        let eq28_e1169_d_n6: f64 = (s.dn[19][6] + s.dn[751][6]);
        let eq28_e1169_d_n7: f64 = (s.dn[19][7] + s.dn[751][7]);
        let eq28_e1169_d_n8: f64 = (s.dn[19][8] + s.dn[751][8]);
        let eq28_e1169_d_n9: f64 = (s.dn[19][9] + s.dn[751][9]);
        let eq28_e1169_d_n10: f64 = (s.dn[19][10] + s.dn[751][10]);
        let eq28_e1169_d_n11: f64 = (s.dn[19][11] + s.dn[751][11]);
        let eq28_e1169_d_n12: f64 = (s.dn[19][12] + s.dn[751][12]);
        let eq28_e1169_d_n13: f64 = (s.dn[19][13] + s.dn[751][13]);
        let eq28_e1169_d_n14: f64 = (s.dn[19][14] + s.dn[751][14]);
        let eq28_e1169_d_n15: f64 = (s.dn[19][15] + s.dn[751][15]);
        let eq28_e1169_d_n16: f64 = (s.dn[19][16] + s.dn[751][16]);
        let eq28_e1169_d_n17: f64 = (s.dn[19][17] + s.dn[751][17]);
        let eq28_e1169_d_b0: f64 = (s.db[19][0] + s.db[751][0]);
        let eq28_e1169_d_b1: f64 = (s.db[19][1] + s.db[751][1]);
        let eq28_e1169_d_b2: f64 = (s.db[19][2] + s.db[751][2]);
        let eq28_e1169_d_b3: f64 = (s.db[19][3] + s.db[751][3]);
        let eq28_e1169_d_b4: f64 = (s.db[19][4] + s.db[751][4]);
        let eq28_e1169_d_b5: f64 = (s.db[19][5] + s.db[751][5]);
        let eq28_e1169_d_b6: f64 = (s.db[19][6] + s.db[751][6]);
        let eq28_e1169_d_b7: f64 = (s.db[19][7] + s.db[751][7]);
        let eq28_e1169_d_b8: f64 = (s.db[19][8] + s.db[751][8]);
        let eq28_e1170: f64 = self.eval_ddt(8, eq28_e1169);
        let eq28_e1170_d_n0: f64 = self.ddt_jacobian(eq28_e1169_d_n0);
        let eq28_e1170_d_n1: f64 = self.ddt_jacobian(eq28_e1169_d_n1);
        let eq28_e1170_d_n2: f64 = self.ddt_jacobian(eq28_e1169_d_n2);
        let eq28_e1170_d_n3: f64 = self.ddt_jacobian(eq28_e1169_d_n3);
        let eq28_e1170_d_n4: f64 = self.ddt_jacobian(eq28_e1169_d_n4);
        let eq28_e1170_d_n5: f64 = self.ddt_jacobian(eq28_e1169_d_n5);
        let eq28_e1170_d_n6: f64 = self.ddt_jacobian(eq28_e1169_d_n6);
        let eq28_e1170_d_n7: f64 = self.ddt_jacobian(eq28_e1169_d_n7);
        let eq28_e1170_d_n8: f64 = self.ddt_jacobian(eq28_e1169_d_n8);
        let eq28_e1170_d_n9: f64 = self.ddt_jacobian(eq28_e1169_d_n9);
        let eq28_e1170_d_n10: f64 = self.ddt_jacobian(eq28_e1169_d_n10);
        let eq28_e1170_d_n11: f64 = self.ddt_jacobian(eq28_e1169_d_n11);
        let eq28_e1170_d_n12: f64 = self.ddt_jacobian(eq28_e1169_d_n12);
        let eq28_e1170_d_n13: f64 = self.ddt_jacobian(eq28_e1169_d_n13);
        let eq28_e1170_d_n14: f64 = self.ddt_jacobian(eq28_e1169_d_n14);
        let eq28_e1170_d_n15: f64 = self.ddt_jacobian(eq28_e1169_d_n15);
        let eq28_e1170_d_n16: f64 = self.ddt_jacobian(eq28_e1169_d_n16);
        let eq28_e1170_d_n17: f64 = self.ddt_jacobian(eq28_e1169_d_n17);
        let eq28_e1170_d_b0: f64 = self.ddt_jacobian(eq28_e1169_d_b0);
        let eq28_e1170_d_b1: f64 = self.ddt_jacobian(eq28_e1169_d_b1);
        let eq28_e1170_d_b2: f64 = self.ddt_jacobian(eq28_e1169_d_b2);
        let eq28_e1170_d_b3: f64 = self.ddt_jacobian(eq28_e1169_d_b3);
        let eq28_e1170_d_b4: f64 = self.ddt_jacobian(eq28_e1169_d_b4);
        let eq28_e1170_d_b5: f64 = self.ddt_jacobian(eq28_e1169_d_b5);
        let eq28_e1170_d_b6: f64 = self.ddt_jacobian(eq28_e1169_d_b6);
        let eq28_e1170_d_b7: f64 = self.ddt_jacobian(eq28_e1169_d_b7);
        let eq28_e1170_d_b8: f64 = self.ddt_jacobian(eq28_e1169_d_b8);
        let eq28_e1171: f64 = (p.p87 * eq28_e1170);
        let eq28_e1171_d_n0: f64 = (p.p87 * eq28_e1170_d_n0);
        let eq28_e1171_d_n1: f64 = (p.p87 * eq28_e1170_d_n1);
        let eq28_e1171_d_n2: f64 = (p.p87 * eq28_e1170_d_n2);
        let eq28_e1171_d_n3: f64 = (p.p87 * eq28_e1170_d_n3);
        let eq28_e1171_d_n4: f64 = (p.p87 * eq28_e1170_d_n4);
        let eq28_e1171_d_n5: f64 = (p.p87 * eq28_e1170_d_n5);
        let eq28_e1171_d_n6: f64 = (p.p87 * eq28_e1170_d_n6);
        let eq28_e1171_d_n7: f64 = (p.p87 * eq28_e1170_d_n7);
        let eq28_e1171_d_n8: f64 = (p.p87 * eq28_e1170_d_n8);
        let eq28_e1171_d_n9: f64 = (p.p87 * eq28_e1170_d_n9);
        let eq28_e1171_d_n10: f64 = (p.p87 * eq28_e1170_d_n10);
        let eq28_e1171_d_n11: f64 = (p.p87 * eq28_e1170_d_n11);
        let eq28_e1171_d_n12: f64 = (p.p87 * eq28_e1170_d_n12);
        let eq28_e1171_d_n13: f64 = (p.p87 * eq28_e1170_d_n13);
        let eq28_e1171_d_n14: f64 = (p.p87 * eq28_e1170_d_n14);
        let eq28_e1171_d_n15: f64 = (p.p87 * eq28_e1170_d_n15);
        let eq28_e1171_d_n16: f64 = (p.p87 * eq28_e1170_d_n16);
        let eq28_e1171_d_n17: f64 = (p.p87 * eq28_e1170_d_n17);
        let eq28_e1171_d_b0: f64 = (p.p87 * eq28_e1170_d_b0);
        let eq28_e1171_d_b1: f64 = (p.p87 * eq28_e1170_d_b1);
        let eq28_e1171_d_b2: f64 = (p.p87 * eq28_e1170_d_b2);
        let eq28_e1171_d_b3: f64 = (p.p87 * eq28_e1170_d_b3);
        let eq28_e1171_d_b4: f64 = (p.p87 * eq28_e1170_d_b4);
        let eq28_e1171_d_b5: f64 = (p.p87 * eq28_e1170_d_b5);
        let eq28_e1171_d_b6: f64 = (p.p87 * eq28_e1170_d_b6);
        let eq28_e1171_d_b7: f64 = (p.p87 * eq28_e1170_d_b7);
        let eq28_e1171_d_b8: f64 = (p.p87 * eq28_e1170_d_b8);
        let eq28_value: f64 = eq28_e1171;
        let eq28_node_derivatives: [f64; 18] = [eq28_e1171_d_n0, eq28_e1171_d_n1, eq28_e1171_d_n2, eq28_e1171_d_n3, eq28_e1171_d_n4, eq28_e1171_d_n5, eq28_e1171_d_n6, eq28_e1171_d_n7, eq28_e1171_d_n8, eq28_e1171_d_n9, eq28_e1171_d_n10, eq28_e1171_d_n11, eq28_e1171_d_n12, eq28_e1171_d_n13, eq28_e1171_d_n14, eq28_e1171_d_n15, eq28_e1171_d_n16, eq28_e1171_d_n17];
        let eq28_branch_derivatives: [f64; 9] = [eq28_e1171_d_b0, eq28_e1171_d_b1, eq28_e1171_d_b2, eq28_e1171_d_b3, eq28_e1171_d_b4, eq28_e1171_d_b5, eq28_e1171_d_b6, eq28_e1171_d_b7, eq28_e1171_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            self.multiplicity * (eq28_value),
            &nodes,
            &eq28_node_derivatives,
            &branches,
            &eq28_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_29_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq29_e1176: f64 = (s.v[753] + s.v[751]);
        let eq29_e1176_d_n0: f64 = (s.dn[753][0] + s.dn[751][0]);
        let eq29_e1176_d_n1: f64 = (s.dn[753][1] + s.dn[751][1]);
        let eq29_e1176_d_n2: f64 = (s.dn[753][2] + s.dn[751][2]);
        let eq29_e1176_d_n3: f64 = (s.dn[753][3] + s.dn[751][3]);
        let eq29_e1176_d_n4: f64 = (s.dn[753][4] + s.dn[751][4]);
        let eq29_e1176_d_n5: f64 = (s.dn[753][5] + s.dn[751][5]);
        let eq29_e1176_d_n6: f64 = (s.dn[753][6] + s.dn[751][6]);
        let eq29_e1176_d_n7: f64 = (s.dn[753][7] + s.dn[751][7]);
        let eq29_e1176_d_n8: f64 = (s.dn[753][8] + s.dn[751][8]);
        let eq29_e1176_d_n9: f64 = (s.dn[753][9] + s.dn[751][9]);
        let eq29_e1176_d_n10: f64 = (s.dn[753][10] + s.dn[751][10]);
        let eq29_e1176_d_n11: f64 = (s.dn[753][11] + s.dn[751][11]);
        let eq29_e1176_d_n12: f64 = (s.dn[753][12] + s.dn[751][12]);
        let eq29_e1176_d_n13: f64 = (s.dn[753][13] + s.dn[751][13]);
        let eq29_e1176_d_n14: f64 = (s.dn[753][14] + s.dn[751][14]);
        let eq29_e1176_d_n15: f64 = (s.dn[753][15] + s.dn[751][15]);
        let eq29_e1176_d_n16: f64 = (s.dn[753][16] + s.dn[751][16]);
        let eq29_e1176_d_n17: f64 = (s.dn[753][17] + s.dn[751][17]);
        let eq29_e1176_d_b0: f64 = (s.db[753][0] + s.db[751][0]);
        let eq29_e1176_d_b1: f64 = (s.db[753][1] + s.db[751][1]);
        let eq29_e1176_d_b2: f64 = (s.db[753][2] + s.db[751][2]);
        let eq29_e1176_d_b3: f64 = (s.db[753][3] + s.db[751][3]);
        let eq29_e1176_d_b4: f64 = (s.db[753][4] + s.db[751][4]);
        let eq29_e1176_d_b5: f64 = (s.db[753][5] + s.db[751][5]);
        let eq29_e1176_d_b6: f64 = (s.db[753][6] + s.db[751][6]);
        let eq29_e1176_d_b7: f64 = (s.db[753][7] + s.db[751][7]);
        let eq29_e1176_d_b8: f64 = (s.db[753][8] + s.db[751][8]);
        let eq29_e1178: f64 = (eq29_e1176 + s.v[752]);
        let eq29_e1178_d_n0: f64 = (eq29_e1176_d_n0 + s.dn[752][0]);
        let eq29_e1178_d_n1: f64 = (eq29_e1176_d_n1 + s.dn[752][1]);
        let eq29_e1178_d_n2: f64 = (eq29_e1176_d_n2 + s.dn[752][2]);
        let eq29_e1178_d_n3: f64 = (eq29_e1176_d_n3 + s.dn[752][3]);
        let eq29_e1178_d_n4: f64 = (eq29_e1176_d_n4 + s.dn[752][4]);
        let eq29_e1178_d_n5: f64 = (eq29_e1176_d_n5 + s.dn[752][5]);
        let eq29_e1178_d_n6: f64 = (eq29_e1176_d_n6 + s.dn[752][6]);
        let eq29_e1178_d_n7: f64 = (eq29_e1176_d_n7 + s.dn[752][7]);
        let eq29_e1178_d_n8: f64 = (eq29_e1176_d_n8 + s.dn[752][8]);
        let eq29_e1178_d_n9: f64 = (eq29_e1176_d_n9 + s.dn[752][9]);
        let eq29_e1178_d_n10: f64 = (eq29_e1176_d_n10 + s.dn[752][10]);
        let eq29_e1178_d_n11: f64 = (eq29_e1176_d_n11 + s.dn[752][11]);
        let eq29_e1178_d_n12: f64 = (eq29_e1176_d_n12 + s.dn[752][12]);
        let eq29_e1178_d_n13: f64 = (eq29_e1176_d_n13 + s.dn[752][13]);
        let eq29_e1178_d_n14: f64 = (eq29_e1176_d_n14 + s.dn[752][14]);
        let eq29_e1178_d_n15: f64 = (eq29_e1176_d_n15 + s.dn[752][15]);
        let eq29_e1178_d_n16: f64 = (eq29_e1176_d_n16 + s.dn[752][16]);
        let eq29_e1178_d_n17: f64 = (eq29_e1176_d_n17 + s.dn[752][17]);
        let eq29_e1178_d_b0: f64 = (eq29_e1176_d_b0 + s.db[752][0]);
        let eq29_e1178_d_b1: f64 = (eq29_e1176_d_b1 + s.db[752][1]);
        let eq29_e1178_d_b2: f64 = (eq29_e1176_d_b2 + s.db[752][2]);
        let eq29_e1178_d_b3: f64 = (eq29_e1176_d_b3 + s.db[752][3]);
        let eq29_e1178_d_b4: f64 = (eq29_e1176_d_b4 + s.db[752][4]);
        let eq29_e1178_d_b5: f64 = (eq29_e1176_d_b5 + s.db[752][5]);
        let eq29_e1178_d_b6: f64 = (eq29_e1176_d_b6 + s.db[752][6]);
        let eq29_e1178_d_b7: f64 = (eq29_e1176_d_b7 + s.db[752][7]);
        let eq29_e1178_d_b8: f64 = (eq29_e1176_d_b8 + s.db[752][8]);
        let eq29_e1179: f64 = (s.v[20] - eq29_e1178);
        let eq29_e1179_d_n0: f64 = (s.dn[20][0] - eq29_e1178_d_n0);
        let eq29_e1179_d_n1: f64 = (s.dn[20][1] - eq29_e1178_d_n1);
        let eq29_e1179_d_n2: f64 = (s.dn[20][2] - eq29_e1178_d_n2);
        let eq29_e1179_d_n3: f64 = (s.dn[20][3] - eq29_e1178_d_n3);
        let eq29_e1179_d_n4: f64 = (s.dn[20][4] - eq29_e1178_d_n4);
        let eq29_e1179_d_n5: f64 = (s.dn[20][5] - eq29_e1178_d_n5);
        let eq29_e1179_d_n6: f64 = (s.dn[20][6] - eq29_e1178_d_n6);
        let eq29_e1179_d_n7: f64 = (s.dn[20][7] - eq29_e1178_d_n7);
        let eq29_e1179_d_n8: f64 = (s.dn[20][8] - eq29_e1178_d_n8);
        let eq29_e1179_d_n9: f64 = (s.dn[20][9] - eq29_e1178_d_n9);
        let eq29_e1179_d_n10: f64 = (s.dn[20][10] - eq29_e1178_d_n10);
        let eq29_e1179_d_n11: f64 = (s.dn[20][11] - eq29_e1178_d_n11);
        let eq29_e1179_d_n12: f64 = (s.dn[20][12] - eq29_e1178_d_n12);
        let eq29_e1179_d_n13: f64 = (s.dn[20][13] - eq29_e1178_d_n13);
        let eq29_e1179_d_n14: f64 = (s.dn[20][14] - eq29_e1178_d_n14);
        let eq29_e1179_d_n15: f64 = (s.dn[20][15] - eq29_e1178_d_n15);
        let eq29_e1179_d_n16: f64 = (s.dn[20][16] - eq29_e1178_d_n16);
        let eq29_e1179_d_n17: f64 = (s.dn[20][17] - eq29_e1178_d_n17);
        let eq29_e1179_d_b0: f64 = (s.db[20][0] - eq29_e1178_d_b0);
        let eq29_e1179_d_b1: f64 = (s.db[20][1] - eq29_e1178_d_b1);
        let eq29_e1179_d_b2: f64 = (s.db[20][2] - eq29_e1178_d_b2);
        let eq29_e1179_d_b3: f64 = (s.db[20][3] - eq29_e1178_d_b3);
        let eq29_e1179_d_b4: f64 = (s.db[20][4] - eq29_e1178_d_b4);
        let eq29_e1179_d_b5: f64 = (s.db[20][5] - eq29_e1178_d_b5);
        let eq29_e1179_d_b6: f64 = (s.db[20][6] - eq29_e1178_d_b6);
        let eq29_e1179_d_b7: f64 = (s.db[20][7] - eq29_e1178_d_b7);
        let eq29_e1179_d_b8: f64 = (s.db[20][8] - eq29_e1178_d_b8);
        let eq29_e1180: f64 = self.eval_ddt(9, eq29_e1179);
        let eq29_e1180_d_n0: f64 = self.ddt_jacobian(eq29_e1179_d_n0);
        let eq29_e1180_d_n1: f64 = self.ddt_jacobian(eq29_e1179_d_n1);
        let eq29_e1180_d_n2: f64 = self.ddt_jacobian(eq29_e1179_d_n2);
        let eq29_e1180_d_n3: f64 = self.ddt_jacobian(eq29_e1179_d_n3);
        let eq29_e1180_d_n4: f64 = self.ddt_jacobian(eq29_e1179_d_n4);
        let eq29_e1180_d_n5: f64 = self.ddt_jacobian(eq29_e1179_d_n5);
        let eq29_e1180_d_n6: f64 = self.ddt_jacobian(eq29_e1179_d_n6);
        let eq29_e1180_d_n7: f64 = self.ddt_jacobian(eq29_e1179_d_n7);
        let eq29_e1180_d_n8: f64 = self.ddt_jacobian(eq29_e1179_d_n8);
        let eq29_e1180_d_n9: f64 = self.ddt_jacobian(eq29_e1179_d_n9);
        let eq29_e1180_d_n10: f64 = self.ddt_jacobian(eq29_e1179_d_n10);
        let eq29_e1180_d_n11: f64 = self.ddt_jacobian(eq29_e1179_d_n11);
        let eq29_e1180_d_n12: f64 = self.ddt_jacobian(eq29_e1179_d_n12);
        let eq29_e1180_d_n13: f64 = self.ddt_jacobian(eq29_e1179_d_n13);
        let eq29_e1180_d_n14: f64 = self.ddt_jacobian(eq29_e1179_d_n14);
        let eq29_e1180_d_n15: f64 = self.ddt_jacobian(eq29_e1179_d_n15);
        let eq29_e1180_d_n16: f64 = self.ddt_jacobian(eq29_e1179_d_n16);
        let eq29_e1180_d_n17: f64 = self.ddt_jacobian(eq29_e1179_d_n17);
        let eq29_e1180_d_b0: f64 = self.ddt_jacobian(eq29_e1179_d_b0);
        let eq29_e1180_d_b1: f64 = self.ddt_jacobian(eq29_e1179_d_b1);
        let eq29_e1180_d_b2: f64 = self.ddt_jacobian(eq29_e1179_d_b2);
        let eq29_e1180_d_b3: f64 = self.ddt_jacobian(eq29_e1179_d_b3);
        let eq29_e1180_d_b4: f64 = self.ddt_jacobian(eq29_e1179_d_b4);
        let eq29_e1180_d_b5: f64 = self.ddt_jacobian(eq29_e1179_d_b5);
        let eq29_e1180_d_b6: f64 = self.ddt_jacobian(eq29_e1179_d_b6);
        let eq29_e1180_d_b7: f64 = self.ddt_jacobian(eq29_e1179_d_b7);
        let eq29_e1180_d_b8: f64 = self.ddt_jacobian(eq29_e1179_d_b8);
        let eq29_e1181: f64 = (p.p87 * eq29_e1180);
        let eq29_e1181_d_n0: f64 = (p.p87 * eq29_e1180_d_n0);
        let eq29_e1181_d_n1: f64 = (p.p87 * eq29_e1180_d_n1);
        let eq29_e1181_d_n2: f64 = (p.p87 * eq29_e1180_d_n2);
        let eq29_e1181_d_n3: f64 = (p.p87 * eq29_e1180_d_n3);
        let eq29_e1181_d_n4: f64 = (p.p87 * eq29_e1180_d_n4);
        let eq29_e1181_d_n5: f64 = (p.p87 * eq29_e1180_d_n5);
        let eq29_e1181_d_n6: f64 = (p.p87 * eq29_e1180_d_n6);
        let eq29_e1181_d_n7: f64 = (p.p87 * eq29_e1180_d_n7);
        let eq29_e1181_d_n8: f64 = (p.p87 * eq29_e1180_d_n8);
        let eq29_e1181_d_n9: f64 = (p.p87 * eq29_e1180_d_n9);
        let eq29_e1181_d_n10: f64 = (p.p87 * eq29_e1180_d_n10);
        let eq29_e1181_d_n11: f64 = (p.p87 * eq29_e1180_d_n11);
        let eq29_e1181_d_n12: f64 = (p.p87 * eq29_e1180_d_n12);
        let eq29_e1181_d_n13: f64 = (p.p87 * eq29_e1180_d_n13);
        let eq29_e1181_d_n14: f64 = (p.p87 * eq29_e1180_d_n14);
        let eq29_e1181_d_n15: f64 = (p.p87 * eq29_e1180_d_n15);
        let eq29_e1181_d_n16: f64 = (p.p87 * eq29_e1180_d_n16);
        let eq29_e1181_d_n17: f64 = (p.p87 * eq29_e1180_d_n17);
        let eq29_e1181_d_b0: f64 = (p.p87 * eq29_e1180_d_b0);
        let eq29_e1181_d_b1: f64 = (p.p87 * eq29_e1180_d_b1);
        let eq29_e1181_d_b2: f64 = (p.p87 * eq29_e1180_d_b2);
        let eq29_e1181_d_b3: f64 = (p.p87 * eq29_e1180_d_b3);
        let eq29_e1181_d_b4: f64 = (p.p87 * eq29_e1180_d_b4);
        let eq29_e1181_d_b5: f64 = (p.p87 * eq29_e1180_d_b5);
        let eq29_e1181_d_b6: f64 = (p.p87 * eq29_e1180_d_b6);
        let eq29_e1181_d_b7: f64 = (p.p87 * eq29_e1180_d_b7);
        let eq29_e1181_d_b8: f64 = (p.p87 * eq29_e1180_d_b8);
        let eq29_value: f64 = eq29_e1181;
        let eq29_node_derivatives: [f64; 18] = [eq29_e1181_d_n0, eq29_e1181_d_n1, eq29_e1181_d_n2, eq29_e1181_d_n3, eq29_e1181_d_n4, eq29_e1181_d_n5, eq29_e1181_d_n6, eq29_e1181_d_n7, eq29_e1181_d_n8, eq29_e1181_d_n9, eq29_e1181_d_n10, eq29_e1181_d_n11, eq29_e1181_d_n12, eq29_e1181_d_n13, eq29_e1181_d_n14, eq29_e1181_d_n15, eq29_e1181_d_n16, eq29_e1181_d_n17];
        let eq29_branch_derivatives: [f64; 9] = [eq29_e1181_d_b0, eq29_e1181_d_b1, eq29_e1181_d_b2, eq29_e1181_d_b3, eq29_e1181_d_b4, eq29_e1181_d_b5, eq29_e1181_d_b6, eq29_e1181_d_b7, eq29_e1181_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            self.multiplicity * (eq29_value),
            &nodes,
            &eq29_node_derivatives,
            &branches,
            &eq29_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_30_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq30_e1184: f64 = self.eval_ddt(10, s.v[743]);
        let eq30_e1184_d_n0: f64 = self.ddt_jacobian(s.dn[743][0]);
        let eq30_e1184_d_n1: f64 = self.ddt_jacobian(s.dn[743][1]);
        let eq30_e1184_d_n2: f64 = self.ddt_jacobian(s.dn[743][2]);
        let eq30_e1184_d_n3: f64 = self.ddt_jacobian(s.dn[743][3]);
        let eq30_e1184_d_n4: f64 = self.ddt_jacobian(s.dn[743][4]);
        let eq30_e1184_d_n5: f64 = self.ddt_jacobian(s.dn[743][5]);
        let eq30_e1184_d_n6: f64 = self.ddt_jacobian(s.dn[743][6]);
        let eq30_e1184_d_n7: f64 = self.ddt_jacobian(s.dn[743][7]);
        let eq30_e1184_d_n8: f64 = self.ddt_jacobian(s.dn[743][8]);
        let eq30_e1184_d_n9: f64 = self.ddt_jacobian(s.dn[743][9]);
        let eq30_e1184_d_n10: f64 = self.ddt_jacobian(s.dn[743][10]);
        let eq30_e1184_d_n11: f64 = self.ddt_jacobian(s.dn[743][11]);
        let eq30_e1184_d_n12: f64 = self.ddt_jacobian(s.dn[743][12]);
        let eq30_e1184_d_n13: f64 = self.ddt_jacobian(s.dn[743][13]);
        let eq30_e1184_d_n14: f64 = self.ddt_jacobian(s.dn[743][14]);
        let eq30_e1184_d_n15: f64 = self.ddt_jacobian(s.dn[743][15]);
        let eq30_e1184_d_n16: f64 = self.ddt_jacobian(s.dn[743][16]);
        let eq30_e1184_d_n17: f64 = self.ddt_jacobian(s.dn[743][17]);
        let eq30_e1184_d_b0: f64 = self.ddt_jacobian(s.db[743][0]);
        let eq30_e1184_d_b1: f64 = self.ddt_jacobian(s.db[743][1]);
        let eq30_e1184_d_b2: f64 = self.ddt_jacobian(s.db[743][2]);
        let eq30_e1184_d_b3: f64 = self.ddt_jacobian(s.db[743][3]);
        let eq30_e1184_d_b4: f64 = self.ddt_jacobian(s.db[743][4]);
        let eq30_e1184_d_b5: f64 = self.ddt_jacobian(s.db[743][5]);
        let eq30_e1184_d_b6: f64 = self.ddt_jacobian(s.db[743][6]);
        let eq30_e1184_d_b7: f64 = self.ddt_jacobian(s.db[743][7]);
        let eq30_e1184_d_b8: f64 = self.ddt_jacobian(s.db[743][8]);
        let eq30_e1185: f64 = (p.p87 * eq30_e1184);
        let eq30_e1185_d_n0: f64 = (p.p87 * eq30_e1184_d_n0);
        let eq30_e1185_d_n1: f64 = (p.p87 * eq30_e1184_d_n1);
        let eq30_e1185_d_n2: f64 = (p.p87 * eq30_e1184_d_n2);
        let eq30_e1185_d_n3: f64 = (p.p87 * eq30_e1184_d_n3);
        let eq30_e1185_d_n4: f64 = (p.p87 * eq30_e1184_d_n4);
        let eq30_e1185_d_n5: f64 = (p.p87 * eq30_e1184_d_n5);
        let eq30_e1185_d_n6: f64 = (p.p87 * eq30_e1184_d_n6);
        let eq30_e1185_d_n7: f64 = (p.p87 * eq30_e1184_d_n7);
        let eq30_e1185_d_n8: f64 = (p.p87 * eq30_e1184_d_n8);
        let eq30_e1185_d_n9: f64 = (p.p87 * eq30_e1184_d_n9);
        let eq30_e1185_d_n10: f64 = (p.p87 * eq30_e1184_d_n10);
        let eq30_e1185_d_n11: f64 = (p.p87 * eq30_e1184_d_n11);
        let eq30_e1185_d_n12: f64 = (p.p87 * eq30_e1184_d_n12);
        let eq30_e1185_d_n13: f64 = (p.p87 * eq30_e1184_d_n13);
        let eq30_e1185_d_n14: f64 = (p.p87 * eq30_e1184_d_n14);
        let eq30_e1185_d_n15: f64 = (p.p87 * eq30_e1184_d_n15);
        let eq30_e1185_d_n16: f64 = (p.p87 * eq30_e1184_d_n16);
        let eq30_e1185_d_n17: f64 = (p.p87 * eq30_e1184_d_n17);
        let eq30_e1185_d_b0: f64 = (p.p87 * eq30_e1184_d_b0);
        let eq30_e1185_d_b1: f64 = (p.p87 * eq30_e1184_d_b1);
        let eq30_e1185_d_b2: f64 = (p.p87 * eq30_e1184_d_b2);
        let eq30_e1185_d_b3: f64 = (p.p87 * eq30_e1184_d_b3);
        let eq30_e1185_d_b4: f64 = (p.p87 * eq30_e1184_d_b4);
        let eq30_e1185_d_b5: f64 = (p.p87 * eq30_e1184_d_b5);
        let eq30_e1185_d_b6: f64 = (p.p87 * eq30_e1184_d_b6);
        let eq30_e1185_d_b7: f64 = (p.p87 * eq30_e1184_d_b7);
        let eq30_e1185_d_b8: f64 = (p.p87 * eq30_e1184_d_b8);
        let eq30_value: f64 = eq30_e1185;
        let eq30_node_derivatives: [f64; 18] = [eq30_e1185_d_n0, eq30_e1185_d_n1, eq30_e1185_d_n2, eq30_e1185_d_n3, eq30_e1185_d_n4, eq30_e1185_d_n5, eq30_e1185_d_n6, eq30_e1185_d_n7, eq30_e1185_d_n8, eq30_e1185_d_n9, eq30_e1185_d_n10, eq30_e1185_d_n11, eq30_e1185_d_n12, eq30_e1185_d_n13, eq30_e1185_d_n14, eq30_e1185_d_n15, eq30_e1185_d_n16, eq30_e1185_d_n17];
        let eq30_branch_derivatives: [f64; 9] = [eq30_e1185_d_b0, eq30_e1185_d_b1, eq30_e1185_d_b2, eq30_e1185_d_b3, eq30_e1185_d_b4, eq30_e1185_d_b5, eq30_e1185_d_b6, eq30_e1185_d_b7, eq30_e1185_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[2]),
            self.multiplicity * (eq30_value),
            &nodes,
            &eq30_node_derivatives,
            &branches,
            &eq30_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_31_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq31_e1188: f64 = self.eval_ddt(11, s.v[742]);
        let eq31_e1188_d_n0: f64 = self.ddt_jacobian(s.dn[742][0]);
        let eq31_e1188_d_n1: f64 = self.ddt_jacobian(s.dn[742][1]);
        let eq31_e1188_d_n2: f64 = self.ddt_jacobian(s.dn[742][2]);
        let eq31_e1188_d_n3: f64 = self.ddt_jacobian(s.dn[742][3]);
        let eq31_e1188_d_n4: f64 = self.ddt_jacobian(s.dn[742][4]);
        let eq31_e1188_d_n5: f64 = self.ddt_jacobian(s.dn[742][5]);
        let eq31_e1188_d_n6: f64 = self.ddt_jacobian(s.dn[742][6]);
        let eq31_e1188_d_n7: f64 = self.ddt_jacobian(s.dn[742][7]);
        let eq31_e1188_d_n8: f64 = self.ddt_jacobian(s.dn[742][8]);
        let eq31_e1188_d_n9: f64 = self.ddt_jacobian(s.dn[742][9]);
        let eq31_e1188_d_n10: f64 = self.ddt_jacobian(s.dn[742][10]);
        let eq31_e1188_d_n11: f64 = self.ddt_jacobian(s.dn[742][11]);
        let eq31_e1188_d_n12: f64 = self.ddt_jacobian(s.dn[742][12]);
        let eq31_e1188_d_n13: f64 = self.ddt_jacobian(s.dn[742][13]);
        let eq31_e1188_d_n14: f64 = self.ddt_jacobian(s.dn[742][14]);
        let eq31_e1188_d_n15: f64 = self.ddt_jacobian(s.dn[742][15]);
        let eq31_e1188_d_n16: f64 = self.ddt_jacobian(s.dn[742][16]);
        let eq31_e1188_d_n17: f64 = self.ddt_jacobian(s.dn[742][17]);
        let eq31_e1188_d_b0: f64 = self.ddt_jacobian(s.db[742][0]);
        let eq31_e1188_d_b1: f64 = self.ddt_jacobian(s.db[742][1]);
        let eq31_e1188_d_b2: f64 = self.ddt_jacobian(s.db[742][2]);
        let eq31_e1188_d_b3: f64 = self.ddt_jacobian(s.db[742][3]);
        let eq31_e1188_d_b4: f64 = self.ddt_jacobian(s.db[742][4]);
        let eq31_e1188_d_b5: f64 = self.ddt_jacobian(s.db[742][5]);
        let eq31_e1188_d_b6: f64 = self.ddt_jacobian(s.db[742][6]);
        let eq31_e1188_d_b7: f64 = self.ddt_jacobian(s.db[742][7]);
        let eq31_e1188_d_b8: f64 = self.ddt_jacobian(s.db[742][8]);
        let eq31_e1189: f64 = (p.p87 * eq31_e1188);
        let eq31_e1189_d_n0: f64 = (p.p87 * eq31_e1188_d_n0);
        let eq31_e1189_d_n1: f64 = (p.p87 * eq31_e1188_d_n1);
        let eq31_e1189_d_n2: f64 = (p.p87 * eq31_e1188_d_n2);
        let eq31_e1189_d_n3: f64 = (p.p87 * eq31_e1188_d_n3);
        let eq31_e1189_d_n4: f64 = (p.p87 * eq31_e1188_d_n4);
        let eq31_e1189_d_n5: f64 = (p.p87 * eq31_e1188_d_n5);
        let eq31_e1189_d_n6: f64 = (p.p87 * eq31_e1188_d_n6);
        let eq31_e1189_d_n7: f64 = (p.p87 * eq31_e1188_d_n7);
        let eq31_e1189_d_n8: f64 = (p.p87 * eq31_e1188_d_n8);
        let eq31_e1189_d_n9: f64 = (p.p87 * eq31_e1188_d_n9);
        let eq31_e1189_d_n10: f64 = (p.p87 * eq31_e1188_d_n10);
        let eq31_e1189_d_n11: f64 = (p.p87 * eq31_e1188_d_n11);
        let eq31_e1189_d_n12: f64 = (p.p87 * eq31_e1188_d_n12);
        let eq31_e1189_d_n13: f64 = (p.p87 * eq31_e1188_d_n13);
        let eq31_e1189_d_n14: f64 = (p.p87 * eq31_e1188_d_n14);
        let eq31_e1189_d_n15: f64 = (p.p87 * eq31_e1188_d_n15);
        let eq31_e1189_d_n16: f64 = (p.p87 * eq31_e1188_d_n16);
        let eq31_e1189_d_n17: f64 = (p.p87 * eq31_e1188_d_n17);
        let eq31_e1189_d_b0: f64 = (p.p87 * eq31_e1188_d_b0);
        let eq31_e1189_d_b1: f64 = (p.p87 * eq31_e1188_d_b1);
        let eq31_e1189_d_b2: f64 = (p.p87 * eq31_e1188_d_b2);
        let eq31_e1189_d_b3: f64 = (p.p87 * eq31_e1188_d_b3);
        let eq31_e1189_d_b4: f64 = (p.p87 * eq31_e1188_d_b4);
        let eq31_e1189_d_b5: f64 = (p.p87 * eq31_e1188_d_b5);
        let eq31_e1189_d_b6: f64 = (p.p87 * eq31_e1188_d_b6);
        let eq31_e1189_d_b7: f64 = (p.p87 * eq31_e1188_d_b7);
        let eq31_e1189_d_b8: f64 = (p.p87 * eq31_e1188_d_b8);
        let eq31_value: f64 = eq31_e1189;
        let eq31_node_derivatives: [f64; 18] = [eq31_e1189_d_n0, eq31_e1189_d_n1, eq31_e1189_d_n2, eq31_e1189_d_n3, eq31_e1189_d_n4, eq31_e1189_d_n5, eq31_e1189_d_n6, eq31_e1189_d_n7, eq31_e1189_d_n8, eq31_e1189_d_n9, eq31_e1189_d_n10, eq31_e1189_d_n11, eq31_e1189_d_n12, eq31_e1189_d_n13, eq31_e1189_d_n14, eq31_e1189_d_n15, eq31_e1189_d_n16, eq31_e1189_d_n17];
        let eq31_branch_derivatives: [f64; 9] = [eq31_e1189_d_b0, eq31_e1189_d_b1, eq31_e1189_d_b2, eq31_e1189_d_b3, eq31_e1189_d_b4, eq31_e1189_d_b5, eq31_e1189_d_b6, eq31_e1189_d_b7, eq31_e1189_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            self.multiplicity * (eq31_value),
            &nodes,
            &eq31_node_derivatives,
            &branches,
            &eq31_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_32_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq32_e1192: f64 = self.eval_ddt(12, s.v[744]);
        let eq32_e1192_d_n0: f64 = self.ddt_jacobian(s.dn[744][0]);
        let eq32_e1192_d_n1: f64 = self.ddt_jacobian(s.dn[744][1]);
        let eq32_e1192_d_n2: f64 = self.ddt_jacobian(s.dn[744][2]);
        let eq32_e1192_d_n3: f64 = self.ddt_jacobian(s.dn[744][3]);
        let eq32_e1192_d_n4: f64 = self.ddt_jacobian(s.dn[744][4]);
        let eq32_e1192_d_n5: f64 = self.ddt_jacobian(s.dn[744][5]);
        let eq32_e1192_d_n6: f64 = self.ddt_jacobian(s.dn[744][6]);
        let eq32_e1192_d_n7: f64 = self.ddt_jacobian(s.dn[744][7]);
        let eq32_e1192_d_n8: f64 = self.ddt_jacobian(s.dn[744][8]);
        let eq32_e1192_d_n9: f64 = self.ddt_jacobian(s.dn[744][9]);
        let eq32_e1192_d_n10: f64 = self.ddt_jacobian(s.dn[744][10]);
        let eq32_e1192_d_n11: f64 = self.ddt_jacobian(s.dn[744][11]);
        let eq32_e1192_d_n12: f64 = self.ddt_jacobian(s.dn[744][12]);
        let eq32_e1192_d_n13: f64 = self.ddt_jacobian(s.dn[744][13]);
        let eq32_e1192_d_n14: f64 = self.ddt_jacobian(s.dn[744][14]);
        let eq32_e1192_d_n15: f64 = self.ddt_jacobian(s.dn[744][15]);
        let eq32_e1192_d_n16: f64 = self.ddt_jacobian(s.dn[744][16]);
        let eq32_e1192_d_n17: f64 = self.ddt_jacobian(s.dn[744][17]);
        let eq32_e1192_d_b0: f64 = self.ddt_jacobian(s.db[744][0]);
        let eq32_e1192_d_b1: f64 = self.ddt_jacobian(s.db[744][1]);
        let eq32_e1192_d_b2: f64 = self.ddt_jacobian(s.db[744][2]);
        let eq32_e1192_d_b3: f64 = self.ddt_jacobian(s.db[744][3]);
        let eq32_e1192_d_b4: f64 = self.ddt_jacobian(s.db[744][4]);
        let eq32_e1192_d_b5: f64 = self.ddt_jacobian(s.db[744][5]);
        let eq32_e1192_d_b6: f64 = self.ddt_jacobian(s.db[744][6]);
        let eq32_e1192_d_b7: f64 = self.ddt_jacobian(s.db[744][7]);
        let eq32_e1192_d_b8: f64 = self.ddt_jacobian(s.db[744][8]);
        let eq32_e1193: f64 = (p.p87 * eq32_e1192);
        let eq32_e1193_d_n0: f64 = (p.p87 * eq32_e1192_d_n0);
        let eq32_e1193_d_n1: f64 = (p.p87 * eq32_e1192_d_n1);
        let eq32_e1193_d_n2: f64 = (p.p87 * eq32_e1192_d_n2);
        let eq32_e1193_d_n3: f64 = (p.p87 * eq32_e1192_d_n3);
        let eq32_e1193_d_n4: f64 = (p.p87 * eq32_e1192_d_n4);
        let eq32_e1193_d_n5: f64 = (p.p87 * eq32_e1192_d_n5);
        let eq32_e1193_d_n6: f64 = (p.p87 * eq32_e1192_d_n6);
        let eq32_e1193_d_n7: f64 = (p.p87 * eq32_e1192_d_n7);
        let eq32_e1193_d_n8: f64 = (p.p87 * eq32_e1192_d_n8);
        let eq32_e1193_d_n9: f64 = (p.p87 * eq32_e1192_d_n9);
        let eq32_e1193_d_n10: f64 = (p.p87 * eq32_e1192_d_n10);
        let eq32_e1193_d_n11: f64 = (p.p87 * eq32_e1192_d_n11);
        let eq32_e1193_d_n12: f64 = (p.p87 * eq32_e1192_d_n12);
        let eq32_e1193_d_n13: f64 = (p.p87 * eq32_e1192_d_n13);
        let eq32_e1193_d_n14: f64 = (p.p87 * eq32_e1192_d_n14);
        let eq32_e1193_d_n15: f64 = (p.p87 * eq32_e1192_d_n15);
        let eq32_e1193_d_n16: f64 = (p.p87 * eq32_e1192_d_n16);
        let eq32_e1193_d_n17: f64 = (p.p87 * eq32_e1192_d_n17);
        let eq32_e1193_d_b0: f64 = (p.p87 * eq32_e1192_d_b0);
        let eq32_e1193_d_b1: f64 = (p.p87 * eq32_e1192_d_b1);
        let eq32_e1193_d_b2: f64 = (p.p87 * eq32_e1192_d_b2);
        let eq32_e1193_d_b3: f64 = (p.p87 * eq32_e1192_d_b3);
        let eq32_e1193_d_b4: f64 = (p.p87 * eq32_e1192_d_b4);
        let eq32_e1193_d_b5: f64 = (p.p87 * eq32_e1192_d_b5);
        let eq32_e1193_d_b6: f64 = (p.p87 * eq32_e1192_d_b6);
        let eq32_e1193_d_b7: f64 = (p.p87 * eq32_e1192_d_b7);
        let eq32_e1193_d_b8: f64 = (p.p87 * eq32_e1192_d_b8);
        let eq32_value: f64 = eq32_e1193;
        let eq32_node_derivatives: [f64; 18] = [eq32_e1193_d_n0, eq32_e1193_d_n1, eq32_e1193_d_n2, eq32_e1193_d_n3, eq32_e1193_d_n4, eq32_e1193_d_n5, eq32_e1193_d_n6, eq32_e1193_d_n7, eq32_e1193_d_n8, eq32_e1193_d_n9, eq32_e1193_d_n10, eq32_e1193_d_n11, eq32_e1193_d_n12, eq32_e1193_d_n13, eq32_e1193_d_n14, eq32_e1193_d_n15, eq32_e1193_d_n16, eq32_e1193_d_n17];
        let eq32_branch_derivatives: [f64; 9] = [eq32_e1193_d_b0, eq32_e1193_d_b1, eq32_e1193_d_b2, eq32_e1193_d_b3, eq32_e1193_d_b4, eq32_e1193_d_b5, eq32_e1193_d_b6, eq32_e1193_d_b7, eq32_e1193_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            self.multiplicity * (eq32_value),
            &nodes,
            &eq32_node_derivatives,
            &branches,
            &eq32_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_33_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq33_e1195: f64 = (-p.p87);
        let eq33_e1197: f64 = self.eval_ddt(13, s.v[299]);
        let eq33_e1197_d_n0: f64 = self.ddt_jacobian(s.dn[299][0]);
        let eq33_e1197_d_n1: f64 = self.ddt_jacobian(s.dn[299][1]);
        let eq33_e1197_d_n2: f64 = self.ddt_jacobian(s.dn[299][2]);
        let eq33_e1197_d_n3: f64 = self.ddt_jacobian(s.dn[299][3]);
        let eq33_e1197_d_n4: f64 = self.ddt_jacobian(s.dn[299][4]);
        let eq33_e1197_d_n5: f64 = self.ddt_jacobian(s.dn[299][5]);
        let eq33_e1197_d_n6: f64 = self.ddt_jacobian(s.dn[299][6]);
        let eq33_e1197_d_n7: f64 = self.ddt_jacobian(s.dn[299][7]);
        let eq33_e1197_d_n8: f64 = self.ddt_jacobian(s.dn[299][8]);
        let eq33_e1197_d_n9: f64 = self.ddt_jacobian(s.dn[299][9]);
        let eq33_e1197_d_n10: f64 = self.ddt_jacobian(s.dn[299][10]);
        let eq33_e1197_d_n11: f64 = self.ddt_jacobian(s.dn[299][11]);
        let eq33_e1197_d_n12: f64 = self.ddt_jacobian(s.dn[299][12]);
        let eq33_e1197_d_n13: f64 = self.ddt_jacobian(s.dn[299][13]);
        let eq33_e1197_d_n14: f64 = self.ddt_jacobian(s.dn[299][14]);
        let eq33_e1197_d_n15: f64 = self.ddt_jacobian(s.dn[299][15]);
        let eq33_e1197_d_n16: f64 = self.ddt_jacobian(s.dn[299][16]);
        let eq33_e1197_d_n17: f64 = self.ddt_jacobian(s.dn[299][17]);
        let eq33_e1197_d_b0: f64 = self.ddt_jacobian(s.db[299][0]);
        let eq33_e1197_d_b1: f64 = self.ddt_jacobian(s.db[299][1]);
        let eq33_e1197_d_b2: f64 = self.ddt_jacobian(s.db[299][2]);
        let eq33_e1197_d_b3: f64 = self.ddt_jacobian(s.db[299][3]);
        let eq33_e1197_d_b4: f64 = self.ddt_jacobian(s.db[299][4]);
        let eq33_e1197_d_b5: f64 = self.ddt_jacobian(s.db[299][5]);
        let eq33_e1197_d_b6: f64 = self.ddt_jacobian(s.db[299][6]);
        let eq33_e1197_d_b7: f64 = self.ddt_jacobian(s.db[299][7]);
        let eq33_e1197_d_b8: f64 = self.ddt_jacobian(s.db[299][8]);
        let eq33_e1198: f64 = (eq33_e1195 * eq33_e1197);
        let eq33_e1198_d_n0: f64 = (eq33_e1195 * eq33_e1197_d_n0);
        let eq33_e1198_d_n1: f64 = (eq33_e1195 * eq33_e1197_d_n1);
        let eq33_e1198_d_n2: f64 = (eq33_e1195 * eq33_e1197_d_n2);
        let eq33_e1198_d_n3: f64 = (eq33_e1195 * eq33_e1197_d_n3);
        let eq33_e1198_d_n4: f64 = (eq33_e1195 * eq33_e1197_d_n4);
        let eq33_e1198_d_n5: f64 = (eq33_e1195 * eq33_e1197_d_n5);
        let eq33_e1198_d_n6: f64 = (eq33_e1195 * eq33_e1197_d_n6);
        let eq33_e1198_d_n7: f64 = (eq33_e1195 * eq33_e1197_d_n7);
        let eq33_e1198_d_n8: f64 = (eq33_e1195 * eq33_e1197_d_n8);
        let eq33_e1198_d_n9: f64 = (eq33_e1195 * eq33_e1197_d_n9);
        let eq33_e1198_d_n10: f64 = (eq33_e1195 * eq33_e1197_d_n10);
        let eq33_e1198_d_n11: f64 = (eq33_e1195 * eq33_e1197_d_n11);
        let eq33_e1198_d_n12: f64 = (eq33_e1195 * eq33_e1197_d_n12);
        let eq33_e1198_d_n13: f64 = (eq33_e1195 * eq33_e1197_d_n13);
        let eq33_e1198_d_n14: f64 = (eq33_e1195 * eq33_e1197_d_n14);
        let eq33_e1198_d_n15: f64 = (eq33_e1195 * eq33_e1197_d_n15);
        let eq33_e1198_d_n16: f64 = (eq33_e1195 * eq33_e1197_d_n16);
        let eq33_e1198_d_n17: f64 = (eq33_e1195 * eq33_e1197_d_n17);
        let eq33_e1198_d_b0: f64 = (eq33_e1195 * eq33_e1197_d_b0);
        let eq33_e1198_d_b1: f64 = (eq33_e1195 * eq33_e1197_d_b1);
        let eq33_e1198_d_b2: f64 = (eq33_e1195 * eq33_e1197_d_b2);
        let eq33_e1198_d_b3: f64 = (eq33_e1195 * eq33_e1197_d_b3);
        let eq33_e1198_d_b4: f64 = (eq33_e1195 * eq33_e1197_d_b4);
        let eq33_e1198_d_b5: f64 = (eq33_e1195 * eq33_e1197_d_b5);
        let eq33_e1198_d_b6: f64 = (eq33_e1195 * eq33_e1197_d_b6);
        let eq33_e1198_d_b7: f64 = (eq33_e1195 * eq33_e1197_d_b7);
        let eq33_e1198_d_b8: f64 = (eq33_e1195 * eq33_e1197_d_b8);
        let eq33_value: f64 = eq33_e1198;
        let eq33_node_derivatives: [f64; 18] = [eq33_e1198_d_n0, eq33_e1198_d_n1, eq33_e1198_d_n2, eq33_e1198_d_n3, eq33_e1198_d_n4, eq33_e1198_d_n5, eq33_e1198_d_n6, eq33_e1198_d_n7, eq33_e1198_d_n8, eq33_e1198_d_n9, eq33_e1198_d_n10, eq33_e1198_d_n11, eq33_e1198_d_n12, eq33_e1198_d_n13, eq33_e1198_d_n14, eq33_e1198_d_n15, eq33_e1198_d_n16, eq33_e1198_d_n17];
        let eq33_branch_derivatives: [f64; 9] = [eq33_e1198_d_b0, eq33_e1198_d_b1, eq33_e1198_d_b2, eq33_e1198_d_b3, eq33_e1198_d_b4, eq33_e1198_d_b5, eq33_e1198_d_b6, eq33_e1198_d_b7, eq33_e1198_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[0]),
            self.multiplicity * (eq33_value),
            &nodes,
            &eq33_node_derivatives,
            &branches,
            &eq33_branch_derivatives,
            self.multiplicity,
        );
    }
}
