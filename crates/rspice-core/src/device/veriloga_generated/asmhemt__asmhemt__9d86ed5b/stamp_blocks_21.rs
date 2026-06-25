#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_155_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq155_e1969, eq155_e1969_d_n0, eq155_e1969_d_n1, eq155_e1969_d_n2, eq155_e1969_d_n3, eq155_e1969_d_n4, eq155_e1969_d_n5, eq155_e1969_d_n6, eq155_e1969_d_n7, eq155_e1969_d_n8, eq155_e1969_d_n9, eq155_e1969_d_n10, eq155_e1969_d_n11, eq155_e1969_d_n12, eq155_e1969_d_n13, eq155_e1969_d_n14, eq155_e1969_d_n15, eq155_e1969_d_n16, eq155_e1969_d_n17, eq155_e1969_d_n18, eq155_e1969_d_n19, eq155_e1969_d_n20, eq155_e1969_d_n21, eq155_e1969_d_n22, eq155_e1969_q, eq155_e1969_q_d_n0, eq155_e1969_q_d_n1, eq155_e1969_q_d_n2, eq155_e1969_q_d_n3, eq155_e1969_q_d_n4, eq155_e1969_q_d_n5, eq155_e1969_q_d_n6, eq155_e1969_q_d_n7, eq155_e1969_q_d_n8, eq155_e1969_q_d_n9, eq155_e1969_q_d_n10, eq155_e1969_q_d_n11, eq155_e1969_q_d_n12, eq155_e1969_q_d_n13, eq155_e1969_q_d_n14, eq155_e1969_q_d_n15, eq155_e1969_q_d_n16, eq155_e1969_q_d_n17, eq155_e1969_q_d_n18, eq155_e1969_q_d_n19, eq155_e1969_q_d_n20, eq155_e1969_q_d_n21, eq155_e1969_q_d_n22,) = {
    if ((!(s.v[580] != 0.0)) && (s.v[583] != 0.0)) {
        let eq155_e1965: f64 = (p.p252 * s.v[252]);
        let eq155_e1965_d_n0: f64 = (p.p252 * s.dn[252][0]);
        let eq155_e1965_d_n1: f64 = (p.p252 * s.dn[252][1]);
        let eq155_e1965_d_n2: f64 = (p.p252 * s.dn[252][2]);
        let eq155_e1965_d_n3: f64 = (p.p252 * s.dn[252][3]);
        let eq155_e1965_d_n4: f64 = (p.p252 * s.dn[252][4]);
        let eq155_e1965_d_n5: f64 = (p.p252 * s.dn[252][5]);
        let eq155_e1965_d_n6: f64 = (p.p252 * s.dn[252][6]);
        let eq155_e1965_d_n7: f64 = (p.p252 * s.dn[252][7]);
        let eq155_e1965_d_n8: f64 = (p.p252 * s.dn[252][8]);
        let eq155_e1965_d_n9: f64 = (p.p252 * s.dn[252][9]);
        let eq155_e1965_d_n10: f64 = (p.p252 * s.dn[252][10]);
        let eq155_e1965_d_n11: f64 = (p.p252 * s.dn[252][11]);
        let eq155_e1965_d_n12: f64 = (p.p252 * s.dn[252][12]);
        let eq155_e1965_d_n13: f64 = (p.p252 * s.dn[252][13]);
        let eq155_e1965_d_n14: f64 = (p.p252 * s.dn[252][14]);
        let eq155_e1965_d_n15: f64 = (p.p252 * s.dn[252][15]);
        let eq155_e1965_d_n16: f64 = (p.p252 * s.dn[252][16]);
        let eq155_e1965_d_n17: f64 = (p.p252 * s.dn[252][17]);
        let eq155_e1965_d_n18: f64 = (p.p252 * s.dn[252][18]);
        let eq155_e1965_d_n19: f64 = (p.p252 * s.dn[252][19]);
        let eq155_e1965_d_n20: f64 = (p.p252 * s.dn[252][20]);
        let eq155_e1965_d_n21: f64 = (p.p252 * s.dn[252][21]);
        let eq155_e1965_d_n22: f64 = (p.p252 * s.dn[252][22]);
        let eq155_e1966_q: f64 = eq155_e1965;
        let eq155_e1967: f64 = (p.p7 * eq155_e1965);
        let eq155_e1967_d_n0: f64 = (p.p7 * eq155_e1965_d_n0);
        let eq155_e1967_d_n1: f64 = (p.p7 * eq155_e1965_d_n1);
        let eq155_e1967_d_n2: f64 = (p.p7 * eq155_e1965_d_n2);
        let eq155_e1967_d_n3: f64 = (p.p7 * eq155_e1965_d_n3);
        let eq155_e1967_d_n4: f64 = (p.p7 * eq155_e1965_d_n4);
        let eq155_e1967_d_n5: f64 = (p.p7 * eq155_e1965_d_n5);
        let eq155_e1967_d_n6: f64 = (p.p7 * eq155_e1965_d_n6);
        let eq155_e1967_d_n7: f64 = (p.p7 * eq155_e1965_d_n7);
        let eq155_e1967_d_n8: f64 = (p.p7 * eq155_e1965_d_n8);
        let eq155_e1967_d_n9: f64 = (p.p7 * eq155_e1965_d_n9);
        let eq155_e1967_d_n10: f64 = (p.p7 * eq155_e1965_d_n10);
        let eq155_e1967_d_n11: f64 = (p.p7 * eq155_e1965_d_n11);
        let eq155_e1967_d_n12: f64 = (p.p7 * eq155_e1965_d_n12);
        let eq155_e1967_d_n13: f64 = (p.p7 * eq155_e1965_d_n13);
        let eq155_e1967_d_n14: f64 = (p.p7 * eq155_e1965_d_n14);
        let eq155_e1967_d_n15: f64 = (p.p7 * eq155_e1965_d_n15);
        let eq155_e1967_d_n16: f64 = (p.p7 * eq155_e1965_d_n16);
        let eq155_e1967_d_n17: f64 = (p.p7 * eq155_e1965_d_n17);
        let eq155_e1967_d_n18: f64 = (p.p7 * eq155_e1965_d_n18);
        let eq155_e1967_d_n19: f64 = (p.p7 * eq155_e1965_d_n19);
        let eq155_e1967_d_n20: f64 = (p.p7 * eq155_e1965_d_n20);
        let eq155_e1967_d_n21: f64 = (p.p7 * eq155_e1965_d_n21);
        let eq155_e1967_d_n22: f64 = (p.p7 * eq155_e1965_d_n22);
        let eq155_e1967_q: f64 = (p.p7 * eq155_e1966_q);
        let eq155_e1967_q_d_n0: f64 = (p.p7 * eq155_e1965_d_n0);
        let eq155_e1967_q_d_n1: f64 = (p.p7 * eq155_e1965_d_n1);
        let eq155_e1967_q_d_n2: f64 = (p.p7 * eq155_e1965_d_n2);
        let eq155_e1967_q_d_n3: f64 = (p.p7 * eq155_e1965_d_n3);
        let eq155_e1967_q_d_n4: f64 = (p.p7 * eq155_e1965_d_n4);
        let eq155_e1967_q_d_n5: f64 = (p.p7 * eq155_e1965_d_n5);
        let eq155_e1967_q_d_n6: f64 = (p.p7 * eq155_e1965_d_n6);
        let eq155_e1967_q_d_n7: f64 = (p.p7 * eq155_e1965_d_n7);
        let eq155_e1967_q_d_n8: f64 = (p.p7 * eq155_e1965_d_n8);
        let eq155_e1967_q_d_n9: f64 = (p.p7 * eq155_e1965_d_n9);
        let eq155_e1967_q_d_n10: f64 = (p.p7 * eq155_e1965_d_n10);
        let eq155_e1967_q_d_n11: f64 = (p.p7 * eq155_e1965_d_n11);
        let eq155_e1967_q_d_n12: f64 = (p.p7 * eq155_e1965_d_n12);
        let eq155_e1967_q_d_n13: f64 = (p.p7 * eq155_e1965_d_n13);
        let eq155_e1967_q_d_n14: f64 = (p.p7 * eq155_e1965_d_n14);
        let eq155_e1967_q_d_n15: f64 = (p.p7 * eq155_e1965_d_n15);
        let eq155_e1967_q_d_n16: f64 = (p.p7 * eq155_e1965_d_n16);
        let eq155_e1967_q_d_n17: f64 = (p.p7 * eq155_e1965_d_n17);
        let eq155_e1967_q_d_n18: f64 = (p.p7 * eq155_e1965_d_n18);
        let eq155_e1967_q_d_n19: f64 = (p.p7 * eq155_e1965_d_n19);
        let eq155_e1967_q_d_n20: f64 = (p.p7 * eq155_e1965_d_n20);
        let eq155_e1967_q_d_n21: f64 = (p.p7 * eq155_e1965_d_n21);
        let eq155_e1967_q_d_n22: f64 = (p.p7 * eq155_e1965_d_n22);
        (eq155_e1967, eq155_e1967_d_n0, eq155_e1967_d_n1, eq155_e1967_d_n2, eq155_e1967_d_n3, eq155_e1967_d_n4, eq155_e1967_d_n5, eq155_e1967_d_n6, eq155_e1967_d_n7, eq155_e1967_d_n8, eq155_e1967_d_n9, eq155_e1967_d_n10, eq155_e1967_d_n11, eq155_e1967_d_n12, eq155_e1967_d_n13, eq155_e1967_d_n14, eq155_e1967_d_n15, eq155_e1967_d_n16, eq155_e1967_d_n17, eq155_e1967_d_n18, eq155_e1967_d_n19, eq155_e1967_d_n20, eq155_e1967_d_n21, eq155_e1967_d_n22, eq155_e1967_q, eq155_e1967_q_d_n0, eq155_e1967_q_d_n1, eq155_e1967_q_d_n2, eq155_e1967_q_d_n3, eq155_e1967_q_d_n4, eq155_e1967_q_d_n5, eq155_e1967_q_d_n6, eq155_e1967_q_d_n7, eq155_e1967_q_d_n8, eq155_e1967_q_d_n9, eq155_e1967_q_d_n10, eq155_e1967_q_d_n11, eq155_e1967_q_d_n12, eq155_e1967_q_d_n13, eq155_e1967_q_d_n14, eq155_e1967_q_d_n15, eq155_e1967_q_d_n16, eq155_e1967_q_d_n17, eq155_e1967_q_d_n18, eq155_e1967_q_d_n19, eq155_e1967_q_d_n20, eq155_e1967_q_d_n21, eq155_e1967_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq155_reactive_node_derivatives: [f64; 23] = [eq155_e1969_q_d_n0, eq155_e1969_q_d_n1, eq155_e1969_q_d_n2, eq155_e1969_q_d_n3, eq155_e1969_q_d_n4, eq155_e1969_q_d_n5, eq155_e1969_q_d_n6, eq155_e1969_q_d_n7, eq155_e1969_q_d_n8, eq155_e1969_q_d_n9, eq155_e1969_q_d_n10, eq155_e1969_q_d_n11, eq155_e1969_q_d_n12, eq155_e1969_q_d_n13, eq155_e1969_q_d_n14, eq155_e1969_q_d_n15, eq155_e1969_q_d_n16, eq155_e1969_q_d_n17, eq155_e1969_q_d_n18, eq155_e1969_q_d_n19, eq155_e1969_q_d_n20, eq155_e1969_q_d_n21, eq155_e1969_q_d_n22];
        let eq155_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            &nodes,
            &eq155_reactive_node_derivatives,
            &branches,
            &eq155_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_156_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq156_e1978, eq156_e1978_d_n0, eq156_e1978_d_n1, eq156_e1978_d_n2, eq156_e1978_d_n3, eq156_e1978_d_n4, eq156_e1978_d_n5, eq156_e1978_d_n6, eq156_e1978_d_n7, eq156_e1978_d_n8, eq156_e1978_d_n9, eq156_e1978_d_n10, eq156_e1978_d_n11, eq156_e1978_d_n12, eq156_e1978_d_n13, eq156_e1978_d_n14, eq156_e1978_d_n15, eq156_e1978_d_n16, eq156_e1978_d_n17, eq156_e1978_d_n18, eq156_e1978_d_n19, eq156_e1978_d_n20, eq156_e1978_d_n21, eq156_e1978_d_n22, eq156_e1978_q, eq156_e1978_q_d_n0, eq156_e1978_q_d_n1, eq156_e1978_q_d_n2, eq156_e1978_q_d_n3, eq156_e1978_q_d_n4, eq156_e1978_q_d_n5, eq156_e1978_q_d_n6, eq156_e1978_q_d_n7, eq156_e1978_q_d_n8, eq156_e1978_q_d_n9, eq156_e1978_q_d_n10, eq156_e1978_q_d_n11, eq156_e1978_q_d_n12, eq156_e1978_q_d_n13, eq156_e1978_q_d_n14, eq156_e1978_q_d_n15, eq156_e1978_q_d_n16, eq156_e1978_q_d_n17, eq156_e1978_q_d_n18, eq156_e1978_q_d_n19, eq156_e1978_q_d_n20, eq156_e1978_q_d_n21, eq156_e1978_q_d_n22,) = {
    if ((s.v[585] != 0.0) && (s.v[586] != 0.0)) {
        let eq156_e1975_q: f64 = s.v[265];
        let eq156_e1976: f64 = (p.p7 * s.v[265]);
        let eq156_e1976_d_n0: f64 = (p.p7 * s.dn[265][0]);
        let eq156_e1976_d_n1: f64 = (p.p7 * s.dn[265][1]);
        let eq156_e1976_d_n2: f64 = (p.p7 * s.dn[265][2]);
        let eq156_e1976_d_n3: f64 = (p.p7 * s.dn[265][3]);
        let eq156_e1976_d_n4: f64 = (p.p7 * s.dn[265][4]);
        let eq156_e1976_d_n5: f64 = (p.p7 * s.dn[265][5]);
        let eq156_e1976_d_n6: f64 = (p.p7 * s.dn[265][6]);
        let eq156_e1976_d_n7: f64 = (p.p7 * s.dn[265][7]);
        let eq156_e1976_d_n8: f64 = (p.p7 * s.dn[265][8]);
        let eq156_e1976_d_n9: f64 = (p.p7 * s.dn[265][9]);
        let eq156_e1976_d_n10: f64 = (p.p7 * s.dn[265][10]);
        let eq156_e1976_d_n11: f64 = (p.p7 * s.dn[265][11]);
        let eq156_e1976_d_n12: f64 = (p.p7 * s.dn[265][12]);
        let eq156_e1976_d_n13: f64 = (p.p7 * s.dn[265][13]);
        let eq156_e1976_d_n14: f64 = (p.p7 * s.dn[265][14]);
        let eq156_e1976_d_n15: f64 = (p.p7 * s.dn[265][15]);
        let eq156_e1976_d_n16: f64 = (p.p7 * s.dn[265][16]);
        let eq156_e1976_d_n17: f64 = (p.p7 * s.dn[265][17]);
        let eq156_e1976_d_n18: f64 = (p.p7 * s.dn[265][18]);
        let eq156_e1976_d_n19: f64 = (p.p7 * s.dn[265][19]);
        let eq156_e1976_d_n20: f64 = (p.p7 * s.dn[265][20]);
        let eq156_e1976_d_n21: f64 = (p.p7 * s.dn[265][21]);
        let eq156_e1976_d_n22: f64 = (p.p7 * s.dn[265][22]);
        let eq156_e1976_q: f64 = (p.p7 * eq156_e1975_q);
        let eq156_e1976_q_d_n0: f64 = (p.p7 * s.dn[265][0]);
        let eq156_e1976_q_d_n1: f64 = (p.p7 * s.dn[265][1]);
        let eq156_e1976_q_d_n2: f64 = (p.p7 * s.dn[265][2]);
        let eq156_e1976_q_d_n3: f64 = (p.p7 * s.dn[265][3]);
        let eq156_e1976_q_d_n4: f64 = (p.p7 * s.dn[265][4]);
        let eq156_e1976_q_d_n5: f64 = (p.p7 * s.dn[265][5]);
        let eq156_e1976_q_d_n6: f64 = (p.p7 * s.dn[265][6]);
        let eq156_e1976_q_d_n7: f64 = (p.p7 * s.dn[265][7]);
        let eq156_e1976_q_d_n8: f64 = (p.p7 * s.dn[265][8]);
        let eq156_e1976_q_d_n9: f64 = (p.p7 * s.dn[265][9]);
        let eq156_e1976_q_d_n10: f64 = (p.p7 * s.dn[265][10]);
        let eq156_e1976_q_d_n11: f64 = (p.p7 * s.dn[265][11]);
        let eq156_e1976_q_d_n12: f64 = (p.p7 * s.dn[265][12]);
        let eq156_e1976_q_d_n13: f64 = (p.p7 * s.dn[265][13]);
        let eq156_e1976_q_d_n14: f64 = (p.p7 * s.dn[265][14]);
        let eq156_e1976_q_d_n15: f64 = (p.p7 * s.dn[265][15]);
        let eq156_e1976_q_d_n16: f64 = (p.p7 * s.dn[265][16]);
        let eq156_e1976_q_d_n17: f64 = (p.p7 * s.dn[265][17]);
        let eq156_e1976_q_d_n18: f64 = (p.p7 * s.dn[265][18]);
        let eq156_e1976_q_d_n19: f64 = (p.p7 * s.dn[265][19]);
        let eq156_e1976_q_d_n20: f64 = (p.p7 * s.dn[265][20]);
        let eq156_e1976_q_d_n21: f64 = (p.p7 * s.dn[265][21]);
        let eq156_e1976_q_d_n22: f64 = (p.p7 * s.dn[265][22]);
        (eq156_e1976, eq156_e1976_d_n0, eq156_e1976_d_n1, eq156_e1976_d_n2, eq156_e1976_d_n3, eq156_e1976_d_n4, eq156_e1976_d_n5, eq156_e1976_d_n6, eq156_e1976_d_n7, eq156_e1976_d_n8, eq156_e1976_d_n9, eq156_e1976_d_n10, eq156_e1976_d_n11, eq156_e1976_d_n12, eq156_e1976_d_n13, eq156_e1976_d_n14, eq156_e1976_d_n15, eq156_e1976_d_n16, eq156_e1976_d_n17, eq156_e1976_d_n18, eq156_e1976_d_n19, eq156_e1976_d_n20, eq156_e1976_d_n21, eq156_e1976_d_n22, eq156_e1976_q, eq156_e1976_q_d_n0, eq156_e1976_q_d_n1, eq156_e1976_q_d_n2, eq156_e1976_q_d_n3, eq156_e1976_q_d_n4, eq156_e1976_q_d_n5, eq156_e1976_q_d_n6, eq156_e1976_q_d_n7, eq156_e1976_q_d_n8, eq156_e1976_q_d_n9, eq156_e1976_q_d_n10, eq156_e1976_q_d_n11, eq156_e1976_q_d_n12, eq156_e1976_q_d_n13, eq156_e1976_q_d_n14, eq156_e1976_q_d_n15, eq156_e1976_q_d_n16, eq156_e1976_q_d_n17, eq156_e1976_q_d_n18, eq156_e1976_q_d_n19, eq156_e1976_q_d_n20, eq156_e1976_q_d_n21, eq156_e1976_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq156_reactive_node_derivatives: [f64; 23] = [eq156_e1978_q_d_n0, eq156_e1978_q_d_n1, eq156_e1978_q_d_n2, eq156_e1978_q_d_n3, eq156_e1978_q_d_n4, eq156_e1978_q_d_n5, eq156_e1978_q_d_n6, eq156_e1978_q_d_n7, eq156_e1978_q_d_n8, eq156_e1978_q_d_n9, eq156_e1978_q_d_n10, eq156_e1978_q_d_n11, eq156_e1978_q_d_n12, eq156_e1978_q_d_n13, eq156_e1978_q_d_n14, eq156_e1978_q_d_n15, eq156_e1978_q_d_n16, eq156_e1978_q_d_n17, eq156_e1978_q_d_n18, eq156_e1978_q_d_n19, eq156_e1978_q_d_n20, eq156_e1978_q_d_n21, eq156_e1978_q_d_n22];
        let eq156_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[20]),
            &nodes,
            &eq156_reactive_node_derivatives,
            &branches,
            &eq156_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_157_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq157_e1989, eq157_e1989_d_n0, eq157_e1989_d_n1, eq157_e1989_d_n2, eq157_e1989_d_n3, eq157_e1989_d_n4, eq157_e1989_d_n5, eq157_e1989_d_n6, eq157_e1989_d_n7, eq157_e1989_d_n8, eq157_e1989_d_n9, eq157_e1989_d_n10, eq157_e1989_d_n11, eq157_e1989_d_n12, eq157_e1989_d_n13, eq157_e1989_d_n14, eq157_e1989_d_n15, eq157_e1989_d_n16, eq157_e1989_d_n17, eq157_e1989_d_n18, eq157_e1989_d_n19, eq157_e1989_d_n20, eq157_e1989_d_n21, eq157_e1989_d_n22, eq157_e1989_q, eq157_e1989_q_d_n0, eq157_e1989_q_d_n1, eq157_e1989_q_d_n2, eq157_e1989_q_d_n3, eq157_e1989_q_d_n4, eq157_e1989_q_d_n5, eq157_e1989_q_d_n6, eq157_e1989_q_d_n7, eq157_e1989_q_d_n8, eq157_e1989_q_d_n9, eq157_e1989_q_d_n10, eq157_e1989_q_d_n11, eq157_e1989_q_d_n12, eq157_e1989_q_d_n13, eq157_e1989_q_d_n14, eq157_e1989_q_d_n15, eq157_e1989_q_d_n16, eq157_e1989_q_d_n17, eq157_e1989_q_d_n18, eq157_e1989_q_d_n19, eq157_e1989_q_d_n20, eq157_e1989_q_d_n21, eq157_e1989_q_d_n22,) = {
    if (((s.v[585] != 0.0) && (s.v[586] != 0.0)) && (s.v[587] != 0.0)) {
        let eq157_e1986_q: f64 = s.v[264];
        let eq157_e1987: f64 = (p.p7 * s.v[264]);
        let eq157_e1987_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq157_e1987_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq157_e1987_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq157_e1987_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq157_e1987_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq157_e1987_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq157_e1987_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq157_e1987_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq157_e1987_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq157_e1987_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq157_e1987_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq157_e1987_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq157_e1987_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq157_e1987_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq157_e1987_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq157_e1987_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq157_e1987_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq157_e1987_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq157_e1987_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq157_e1987_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq157_e1987_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq157_e1987_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq157_e1987_d_n22: f64 = (p.p7 * s.dn[264][22]);
        let eq157_e1987_q: f64 = (p.p7 * eq157_e1986_q);
        let eq157_e1987_q_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq157_e1987_q_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq157_e1987_q_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq157_e1987_q_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq157_e1987_q_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq157_e1987_q_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq157_e1987_q_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq157_e1987_q_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq157_e1987_q_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq157_e1987_q_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq157_e1987_q_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq157_e1987_q_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq157_e1987_q_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq157_e1987_q_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq157_e1987_q_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq157_e1987_q_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq157_e1987_q_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq157_e1987_q_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq157_e1987_q_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq157_e1987_q_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq157_e1987_q_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq157_e1987_q_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq157_e1987_q_d_n22: f64 = (p.p7 * s.dn[264][22]);
        (eq157_e1987, eq157_e1987_d_n0, eq157_e1987_d_n1, eq157_e1987_d_n2, eq157_e1987_d_n3, eq157_e1987_d_n4, eq157_e1987_d_n5, eq157_e1987_d_n6, eq157_e1987_d_n7, eq157_e1987_d_n8, eq157_e1987_d_n9, eq157_e1987_d_n10, eq157_e1987_d_n11, eq157_e1987_d_n12, eq157_e1987_d_n13, eq157_e1987_d_n14, eq157_e1987_d_n15, eq157_e1987_d_n16, eq157_e1987_d_n17, eq157_e1987_d_n18, eq157_e1987_d_n19, eq157_e1987_d_n20, eq157_e1987_d_n21, eq157_e1987_d_n22, eq157_e1987_q, eq157_e1987_q_d_n0, eq157_e1987_q_d_n1, eq157_e1987_q_d_n2, eq157_e1987_q_d_n3, eq157_e1987_q_d_n4, eq157_e1987_q_d_n5, eq157_e1987_q_d_n6, eq157_e1987_q_d_n7, eq157_e1987_q_d_n8, eq157_e1987_q_d_n9, eq157_e1987_q_d_n10, eq157_e1987_q_d_n11, eq157_e1987_q_d_n12, eq157_e1987_q_d_n13, eq157_e1987_q_d_n14, eq157_e1987_q_d_n15, eq157_e1987_q_d_n16, eq157_e1987_q_d_n17, eq157_e1987_q_d_n18, eq157_e1987_q_d_n19, eq157_e1987_q_d_n20, eq157_e1987_q_d_n21, eq157_e1987_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq157_reactive_node_derivatives: [f64; 23] = [eq157_e1989_q_d_n0, eq157_e1989_q_d_n1, eq157_e1989_q_d_n2, eq157_e1989_q_d_n3, eq157_e1989_q_d_n4, eq157_e1989_q_d_n5, eq157_e1989_q_d_n6, eq157_e1989_q_d_n7, eq157_e1989_q_d_n8, eq157_e1989_q_d_n9, eq157_e1989_q_d_n10, eq157_e1989_q_d_n11, eq157_e1989_q_d_n12, eq157_e1989_q_d_n13, eq157_e1989_q_d_n14, eq157_e1989_q_d_n15, eq157_e1989_q_d_n16, eq157_e1989_q_d_n17, eq157_e1989_q_d_n18, eq157_e1989_q_d_n19, eq157_e1989_q_d_n20, eq157_e1989_q_d_n21, eq157_e1989_q_d_n22];
        let eq157_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[20]),
            &nodes,
            &eq157_reactive_node_derivatives,
            &branches,
            &eq157_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_158_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq158_e2002, eq158_e2002_d_n0, eq158_e2002_d_n1, eq158_e2002_d_n2, eq158_e2002_d_n3, eq158_e2002_d_n4, eq158_e2002_d_n5, eq158_e2002_d_n6, eq158_e2002_d_n7, eq158_e2002_d_n8, eq158_e2002_d_n9, eq158_e2002_d_n10, eq158_e2002_d_n11, eq158_e2002_d_n12, eq158_e2002_d_n13, eq158_e2002_d_n14, eq158_e2002_d_n15, eq158_e2002_d_n16, eq158_e2002_d_n17, eq158_e2002_d_n18, eq158_e2002_d_n19, eq158_e2002_d_n20, eq158_e2002_d_n21, eq158_e2002_d_n22, eq158_e2002_q, eq158_e2002_q_d_n0, eq158_e2002_q_d_n1, eq158_e2002_q_d_n2, eq158_e2002_q_d_n3, eq158_e2002_q_d_n4, eq158_e2002_q_d_n5, eq158_e2002_q_d_n6, eq158_e2002_q_d_n7, eq158_e2002_q_d_n8, eq158_e2002_q_d_n9, eq158_e2002_q_d_n10, eq158_e2002_q_d_n11, eq158_e2002_q_d_n12, eq158_e2002_q_d_n13, eq158_e2002_q_d_n14, eq158_e2002_q_d_n15, eq158_e2002_q_d_n16, eq158_e2002_q_d_n17, eq158_e2002_q_d_n18, eq158_e2002_q_d_n19, eq158_e2002_q_d_n20, eq158_e2002_q_d_n21, eq158_e2002_q_d_n22,) = {
    if (((s.v[585] != 0.0) && (s.v[586] != 0.0)) && (s.v[587] != 0.0)) {
        let eq158_e1997_q: f64 = s.v[264];
        let eq158_e1998: f64 = (p.p7 * s.v[264]);
        let eq158_e1998_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq158_e1998_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq158_e1998_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq158_e1998_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq158_e1998_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq158_e1998_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq158_e1998_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq158_e1998_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq158_e1998_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq158_e1998_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq158_e1998_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq158_e1998_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq158_e1998_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq158_e1998_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq158_e1998_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq158_e1998_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq158_e1998_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq158_e1998_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq158_e1998_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq158_e1998_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq158_e1998_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq158_e1998_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq158_e1998_d_n22: f64 = (p.p7 * s.dn[264][22]);
        let eq158_e1998_q: f64 = (p.p7 * eq158_e1997_q);
        let eq158_e1998_q_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq158_e1998_q_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq158_e1998_q_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq158_e1998_q_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq158_e1998_q_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq158_e1998_q_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq158_e1998_q_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq158_e1998_q_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq158_e1998_q_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq158_e1998_q_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq158_e1998_q_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq158_e1998_q_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq158_e1998_q_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq158_e1998_q_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq158_e1998_q_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq158_e1998_q_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq158_e1998_q_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq158_e1998_q_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq158_e1998_q_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq158_e1998_q_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq158_e1998_q_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq158_e1998_q_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq158_e1998_q_d_n22: f64 = (p.p7 * s.dn[264][22]);
        let eq158_e2000: f64 = (eq158_e1998 * p.p247);
        let eq158_e2000_d_n0: f64 = (eq158_e1998_d_n0 * p.p247);
        let eq158_e2000_d_n1: f64 = (eq158_e1998_d_n1 * p.p247);
        let eq158_e2000_d_n2: f64 = (eq158_e1998_d_n2 * p.p247);
        let eq158_e2000_d_n3: f64 = (eq158_e1998_d_n3 * p.p247);
        let eq158_e2000_d_n4: f64 = (eq158_e1998_d_n4 * p.p247);
        let eq158_e2000_d_n5: f64 = (eq158_e1998_d_n5 * p.p247);
        let eq158_e2000_d_n6: f64 = (eq158_e1998_d_n6 * p.p247);
        let eq158_e2000_d_n7: f64 = (eq158_e1998_d_n7 * p.p247);
        let eq158_e2000_d_n8: f64 = (eq158_e1998_d_n8 * p.p247);
        let eq158_e2000_d_n9: f64 = (eq158_e1998_d_n9 * p.p247);
        let eq158_e2000_d_n10: f64 = (eq158_e1998_d_n10 * p.p247);
        let eq158_e2000_d_n11: f64 = (eq158_e1998_d_n11 * p.p247);
        let eq158_e2000_d_n12: f64 = (eq158_e1998_d_n12 * p.p247);
        let eq158_e2000_d_n13: f64 = (eq158_e1998_d_n13 * p.p247);
        let eq158_e2000_d_n14: f64 = (eq158_e1998_d_n14 * p.p247);
        let eq158_e2000_d_n15: f64 = (eq158_e1998_d_n15 * p.p247);
        let eq158_e2000_d_n16: f64 = (eq158_e1998_d_n16 * p.p247);
        let eq158_e2000_d_n17: f64 = (eq158_e1998_d_n17 * p.p247);
        let eq158_e2000_d_n18: f64 = (eq158_e1998_d_n18 * p.p247);
        let eq158_e2000_d_n19: f64 = (eq158_e1998_d_n19 * p.p247);
        let eq158_e2000_d_n20: f64 = (eq158_e1998_d_n20 * p.p247);
        let eq158_e2000_d_n21: f64 = (eq158_e1998_d_n21 * p.p247);
        let eq158_e2000_d_n22: f64 = (eq158_e1998_d_n22 * p.p247);
        let eq158_e2000_q: f64 = (eq158_e1998_q * p.p247);
        let eq158_e2000_q_d_n0: f64 = (eq158_e1998_q_d_n0 * p.p247);
        let eq158_e2000_q_d_n1: f64 = (eq158_e1998_q_d_n1 * p.p247);
        let eq158_e2000_q_d_n2: f64 = (eq158_e1998_q_d_n2 * p.p247);
        let eq158_e2000_q_d_n3: f64 = (eq158_e1998_q_d_n3 * p.p247);
        let eq158_e2000_q_d_n4: f64 = (eq158_e1998_q_d_n4 * p.p247);
        let eq158_e2000_q_d_n5: f64 = (eq158_e1998_q_d_n5 * p.p247);
        let eq158_e2000_q_d_n6: f64 = (eq158_e1998_q_d_n6 * p.p247);
        let eq158_e2000_q_d_n7: f64 = (eq158_e1998_q_d_n7 * p.p247);
        let eq158_e2000_q_d_n8: f64 = (eq158_e1998_q_d_n8 * p.p247);
        let eq158_e2000_q_d_n9: f64 = (eq158_e1998_q_d_n9 * p.p247);
        let eq158_e2000_q_d_n10: f64 = (eq158_e1998_q_d_n10 * p.p247);
        let eq158_e2000_q_d_n11: f64 = (eq158_e1998_q_d_n11 * p.p247);
        let eq158_e2000_q_d_n12: f64 = (eq158_e1998_q_d_n12 * p.p247);
        let eq158_e2000_q_d_n13: f64 = (eq158_e1998_q_d_n13 * p.p247);
        let eq158_e2000_q_d_n14: f64 = (eq158_e1998_q_d_n14 * p.p247);
        let eq158_e2000_q_d_n15: f64 = (eq158_e1998_q_d_n15 * p.p247);
        let eq158_e2000_q_d_n16: f64 = (eq158_e1998_q_d_n16 * p.p247);
        let eq158_e2000_q_d_n17: f64 = (eq158_e1998_q_d_n17 * p.p247);
        let eq158_e2000_q_d_n18: f64 = (eq158_e1998_q_d_n18 * p.p247);
        let eq158_e2000_q_d_n19: f64 = (eq158_e1998_q_d_n19 * p.p247);
        let eq158_e2000_q_d_n20: f64 = (eq158_e1998_q_d_n20 * p.p247);
        let eq158_e2000_q_d_n21: f64 = (eq158_e1998_q_d_n21 * p.p247);
        let eq158_e2000_q_d_n22: f64 = (eq158_e1998_q_d_n22 * p.p247);
        (eq158_e2000, eq158_e2000_d_n0, eq158_e2000_d_n1, eq158_e2000_d_n2, eq158_e2000_d_n3, eq158_e2000_d_n4, eq158_e2000_d_n5, eq158_e2000_d_n6, eq158_e2000_d_n7, eq158_e2000_d_n8, eq158_e2000_d_n9, eq158_e2000_d_n10, eq158_e2000_d_n11, eq158_e2000_d_n12, eq158_e2000_d_n13, eq158_e2000_d_n14, eq158_e2000_d_n15, eq158_e2000_d_n16, eq158_e2000_d_n17, eq158_e2000_d_n18, eq158_e2000_d_n19, eq158_e2000_d_n20, eq158_e2000_d_n21, eq158_e2000_d_n22, eq158_e2000_q, eq158_e2000_q_d_n0, eq158_e2000_q_d_n1, eq158_e2000_q_d_n2, eq158_e2000_q_d_n3, eq158_e2000_q_d_n4, eq158_e2000_q_d_n5, eq158_e2000_q_d_n6, eq158_e2000_q_d_n7, eq158_e2000_q_d_n8, eq158_e2000_q_d_n9, eq158_e2000_q_d_n10, eq158_e2000_q_d_n11, eq158_e2000_q_d_n12, eq158_e2000_q_d_n13, eq158_e2000_q_d_n14, eq158_e2000_q_d_n15, eq158_e2000_q_d_n16, eq158_e2000_q_d_n17, eq158_e2000_q_d_n18, eq158_e2000_q_d_n19, eq158_e2000_q_d_n20, eq158_e2000_q_d_n21, eq158_e2000_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq158_reactive_node_derivatives: [f64; 23] = [eq158_e2002_q_d_n0, eq158_e2002_q_d_n1, eq158_e2002_q_d_n2, eq158_e2002_q_d_n3, eq158_e2002_q_d_n4, eq158_e2002_q_d_n5, eq158_e2002_q_d_n6, eq158_e2002_q_d_n7, eq158_e2002_q_d_n8, eq158_e2002_q_d_n9, eq158_e2002_q_d_n10, eq158_e2002_q_d_n11, eq158_e2002_q_d_n12, eq158_e2002_q_d_n13, eq158_e2002_q_d_n14, eq158_e2002_q_d_n15, eq158_e2002_q_d_n16, eq158_e2002_q_d_n17, eq158_e2002_q_d_n18, eq158_e2002_q_d_n19, eq158_e2002_q_d_n20, eq158_e2002_q_d_n21, eq158_e2002_q_d_n22];
        let eq158_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[20]),
            &nodes,
            &eq158_reactive_node_derivatives,
            &branches,
            &eq158_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_159_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq159_e2014, eq159_e2014_d_n0, eq159_e2014_d_n1, eq159_e2014_d_n2, eq159_e2014_d_n3, eq159_e2014_d_n4, eq159_e2014_d_n5, eq159_e2014_d_n6, eq159_e2014_d_n7, eq159_e2014_d_n8, eq159_e2014_d_n9, eq159_e2014_d_n10, eq159_e2014_d_n11, eq159_e2014_d_n12, eq159_e2014_d_n13, eq159_e2014_d_n14, eq159_e2014_d_n15, eq159_e2014_d_n16, eq159_e2014_d_n17, eq159_e2014_d_n18, eq159_e2014_d_n19, eq159_e2014_d_n20, eq159_e2014_d_n21, eq159_e2014_d_n22, eq159_e2014_q, eq159_e2014_q_d_n0, eq159_e2014_q_d_n1, eq159_e2014_q_d_n2, eq159_e2014_q_d_n3, eq159_e2014_q_d_n4, eq159_e2014_q_d_n5, eq159_e2014_q_d_n6, eq159_e2014_q_d_n7, eq159_e2014_q_d_n8, eq159_e2014_q_d_n9, eq159_e2014_q_d_n10, eq159_e2014_q_d_n11, eq159_e2014_q_d_n12, eq159_e2014_q_d_n13, eq159_e2014_q_d_n14, eq159_e2014_q_d_n15, eq159_e2014_q_d_n16, eq159_e2014_q_d_n17, eq159_e2014_q_d_n18, eq159_e2014_q_d_n19, eq159_e2014_q_d_n20, eq159_e2014_q_d_n21, eq159_e2014_q_d_n22,) = {
    if (((s.v[585] != 0.0) && (s.v[586] != 0.0)) && (!(s.v[587] != 0.0))) {
        let eq159_e2011_q: f64 = s.v[264];
        let eq159_e2012: f64 = (p.p7 * s.v[264]);
        let eq159_e2012_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq159_e2012_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq159_e2012_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq159_e2012_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq159_e2012_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq159_e2012_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq159_e2012_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq159_e2012_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq159_e2012_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq159_e2012_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq159_e2012_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq159_e2012_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq159_e2012_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq159_e2012_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq159_e2012_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq159_e2012_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq159_e2012_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq159_e2012_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq159_e2012_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq159_e2012_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq159_e2012_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq159_e2012_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq159_e2012_d_n22: f64 = (p.p7 * s.dn[264][22]);
        let eq159_e2012_q: f64 = (p.p7 * eq159_e2011_q);
        let eq159_e2012_q_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq159_e2012_q_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq159_e2012_q_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq159_e2012_q_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq159_e2012_q_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq159_e2012_q_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq159_e2012_q_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq159_e2012_q_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq159_e2012_q_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq159_e2012_q_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq159_e2012_q_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq159_e2012_q_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq159_e2012_q_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq159_e2012_q_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq159_e2012_q_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq159_e2012_q_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq159_e2012_q_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq159_e2012_q_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq159_e2012_q_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq159_e2012_q_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq159_e2012_q_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq159_e2012_q_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq159_e2012_q_d_n22: f64 = (p.p7 * s.dn[264][22]);
        (eq159_e2012, eq159_e2012_d_n0, eq159_e2012_d_n1, eq159_e2012_d_n2, eq159_e2012_d_n3, eq159_e2012_d_n4, eq159_e2012_d_n5, eq159_e2012_d_n6, eq159_e2012_d_n7, eq159_e2012_d_n8, eq159_e2012_d_n9, eq159_e2012_d_n10, eq159_e2012_d_n11, eq159_e2012_d_n12, eq159_e2012_d_n13, eq159_e2012_d_n14, eq159_e2012_d_n15, eq159_e2012_d_n16, eq159_e2012_d_n17, eq159_e2012_d_n18, eq159_e2012_d_n19, eq159_e2012_d_n20, eq159_e2012_d_n21, eq159_e2012_d_n22, eq159_e2012_q, eq159_e2012_q_d_n0, eq159_e2012_q_d_n1, eq159_e2012_q_d_n2, eq159_e2012_q_d_n3, eq159_e2012_q_d_n4, eq159_e2012_q_d_n5, eq159_e2012_q_d_n6, eq159_e2012_q_d_n7, eq159_e2012_q_d_n8, eq159_e2012_q_d_n9, eq159_e2012_q_d_n10, eq159_e2012_q_d_n11, eq159_e2012_q_d_n12, eq159_e2012_q_d_n13, eq159_e2012_q_d_n14, eq159_e2012_q_d_n15, eq159_e2012_q_d_n16, eq159_e2012_q_d_n17, eq159_e2012_q_d_n18, eq159_e2012_q_d_n19, eq159_e2012_q_d_n20, eq159_e2012_q_d_n21, eq159_e2012_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq159_reactive_node_derivatives: [f64; 23] = [eq159_e2014_q_d_n0, eq159_e2014_q_d_n1, eq159_e2014_q_d_n2, eq159_e2014_q_d_n3, eq159_e2014_q_d_n4, eq159_e2014_q_d_n5, eq159_e2014_q_d_n6, eq159_e2014_q_d_n7, eq159_e2014_q_d_n8, eq159_e2014_q_d_n9, eq159_e2014_q_d_n10, eq159_e2014_q_d_n11, eq159_e2014_q_d_n12, eq159_e2014_q_d_n13, eq159_e2014_q_d_n14, eq159_e2014_q_d_n15, eq159_e2014_q_d_n16, eq159_e2014_q_d_n17, eq159_e2014_q_d_n18, eq159_e2014_q_d_n19, eq159_e2014_q_d_n20, eq159_e2014_q_d_n21, eq159_e2014_q_d_n22];
        let eq159_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[20]),
            &nodes,
            &eq159_reactive_node_derivatives,
            &branches,
            &eq159_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_160_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq160_e2028, eq160_e2028_d_n0, eq160_e2028_d_n1, eq160_e2028_d_n2, eq160_e2028_d_n3, eq160_e2028_d_n4, eq160_e2028_d_n5, eq160_e2028_d_n6, eq160_e2028_d_n7, eq160_e2028_d_n8, eq160_e2028_d_n9, eq160_e2028_d_n10, eq160_e2028_d_n11, eq160_e2028_d_n12, eq160_e2028_d_n13, eq160_e2028_d_n14, eq160_e2028_d_n15, eq160_e2028_d_n16, eq160_e2028_d_n17, eq160_e2028_d_n18, eq160_e2028_d_n19, eq160_e2028_d_n20, eq160_e2028_d_n21, eq160_e2028_d_n22, eq160_e2028_q, eq160_e2028_q_d_n0, eq160_e2028_q_d_n1, eq160_e2028_q_d_n2, eq160_e2028_q_d_n3, eq160_e2028_q_d_n4, eq160_e2028_q_d_n5, eq160_e2028_q_d_n6, eq160_e2028_q_d_n7, eq160_e2028_q_d_n8, eq160_e2028_q_d_n9, eq160_e2028_q_d_n10, eq160_e2028_q_d_n11, eq160_e2028_q_d_n12, eq160_e2028_q_d_n13, eq160_e2028_q_d_n14, eq160_e2028_q_d_n15, eq160_e2028_q_d_n16, eq160_e2028_q_d_n17, eq160_e2028_q_d_n18, eq160_e2028_q_d_n19, eq160_e2028_q_d_n20, eq160_e2028_q_d_n21, eq160_e2028_q_d_n22,) = {
    if (((s.v[585] != 0.0) && (s.v[586] != 0.0)) && (!(s.v[587] != 0.0))) {
        let eq160_e2023_q: f64 = s.v[264];
        let eq160_e2024: f64 = (p.p7 * s.v[264]);
        let eq160_e2024_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq160_e2024_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq160_e2024_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq160_e2024_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq160_e2024_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq160_e2024_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq160_e2024_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq160_e2024_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq160_e2024_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq160_e2024_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq160_e2024_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq160_e2024_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq160_e2024_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq160_e2024_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq160_e2024_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq160_e2024_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq160_e2024_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq160_e2024_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq160_e2024_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq160_e2024_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq160_e2024_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq160_e2024_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq160_e2024_d_n22: f64 = (p.p7 * s.dn[264][22]);
        let eq160_e2024_q: f64 = (p.p7 * eq160_e2023_q);
        let eq160_e2024_q_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq160_e2024_q_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq160_e2024_q_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq160_e2024_q_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq160_e2024_q_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq160_e2024_q_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq160_e2024_q_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq160_e2024_q_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq160_e2024_q_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq160_e2024_q_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq160_e2024_q_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq160_e2024_q_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq160_e2024_q_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq160_e2024_q_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq160_e2024_q_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq160_e2024_q_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq160_e2024_q_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq160_e2024_q_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq160_e2024_q_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq160_e2024_q_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq160_e2024_q_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq160_e2024_q_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq160_e2024_q_d_n22: f64 = (p.p7 * s.dn[264][22]);
        let eq160_e2026: f64 = (eq160_e2024 * p.p247);
        let eq160_e2026_d_n0: f64 = (eq160_e2024_d_n0 * p.p247);
        let eq160_e2026_d_n1: f64 = (eq160_e2024_d_n1 * p.p247);
        let eq160_e2026_d_n2: f64 = (eq160_e2024_d_n2 * p.p247);
        let eq160_e2026_d_n3: f64 = (eq160_e2024_d_n3 * p.p247);
        let eq160_e2026_d_n4: f64 = (eq160_e2024_d_n4 * p.p247);
        let eq160_e2026_d_n5: f64 = (eq160_e2024_d_n5 * p.p247);
        let eq160_e2026_d_n6: f64 = (eq160_e2024_d_n6 * p.p247);
        let eq160_e2026_d_n7: f64 = (eq160_e2024_d_n7 * p.p247);
        let eq160_e2026_d_n8: f64 = (eq160_e2024_d_n8 * p.p247);
        let eq160_e2026_d_n9: f64 = (eq160_e2024_d_n9 * p.p247);
        let eq160_e2026_d_n10: f64 = (eq160_e2024_d_n10 * p.p247);
        let eq160_e2026_d_n11: f64 = (eq160_e2024_d_n11 * p.p247);
        let eq160_e2026_d_n12: f64 = (eq160_e2024_d_n12 * p.p247);
        let eq160_e2026_d_n13: f64 = (eq160_e2024_d_n13 * p.p247);
        let eq160_e2026_d_n14: f64 = (eq160_e2024_d_n14 * p.p247);
        let eq160_e2026_d_n15: f64 = (eq160_e2024_d_n15 * p.p247);
        let eq160_e2026_d_n16: f64 = (eq160_e2024_d_n16 * p.p247);
        let eq160_e2026_d_n17: f64 = (eq160_e2024_d_n17 * p.p247);
        let eq160_e2026_d_n18: f64 = (eq160_e2024_d_n18 * p.p247);
        let eq160_e2026_d_n19: f64 = (eq160_e2024_d_n19 * p.p247);
        let eq160_e2026_d_n20: f64 = (eq160_e2024_d_n20 * p.p247);
        let eq160_e2026_d_n21: f64 = (eq160_e2024_d_n21 * p.p247);
        let eq160_e2026_d_n22: f64 = (eq160_e2024_d_n22 * p.p247);
        let eq160_e2026_q: f64 = (eq160_e2024_q * p.p247);
        let eq160_e2026_q_d_n0: f64 = (eq160_e2024_q_d_n0 * p.p247);
        let eq160_e2026_q_d_n1: f64 = (eq160_e2024_q_d_n1 * p.p247);
        let eq160_e2026_q_d_n2: f64 = (eq160_e2024_q_d_n2 * p.p247);
        let eq160_e2026_q_d_n3: f64 = (eq160_e2024_q_d_n3 * p.p247);
        let eq160_e2026_q_d_n4: f64 = (eq160_e2024_q_d_n4 * p.p247);
        let eq160_e2026_q_d_n5: f64 = (eq160_e2024_q_d_n5 * p.p247);
        let eq160_e2026_q_d_n6: f64 = (eq160_e2024_q_d_n6 * p.p247);
        let eq160_e2026_q_d_n7: f64 = (eq160_e2024_q_d_n7 * p.p247);
        let eq160_e2026_q_d_n8: f64 = (eq160_e2024_q_d_n8 * p.p247);
        let eq160_e2026_q_d_n9: f64 = (eq160_e2024_q_d_n9 * p.p247);
        let eq160_e2026_q_d_n10: f64 = (eq160_e2024_q_d_n10 * p.p247);
        let eq160_e2026_q_d_n11: f64 = (eq160_e2024_q_d_n11 * p.p247);
        let eq160_e2026_q_d_n12: f64 = (eq160_e2024_q_d_n12 * p.p247);
        let eq160_e2026_q_d_n13: f64 = (eq160_e2024_q_d_n13 * p.p247);
        let eq160_e2026_q_d_n14: f64 = (eq160_e2024_q_d_n14 * p.p247);
        let eq160_e2026_q_d_n15: f64 = (eq160_e2024_q_d_n15 * p.p247);
        let eq160_e2026_q_d_n16: f64 = (eq160_e2024_q_d_n16 * p.p247);
        let eq160_e2026_q_d_n17: f64 = (eq160_e2024_q_d_n17 * p.p247);
        let eq160_e2026_q_d_n18: f64 = (eq160_e2024_q_d_n18 * p.p247);
        let eq160_e2026_q_d_n19: f64 = (eq160_e2024_q_d_n19 * p.p247);
        let eq160_e2026_q_d_n20: f64 = (eq160_e2024_q_d_n20 * p.p247);
        let eq160_e2026_q_d_n21: f64 = (eq160_e2024_q_d_n21 * p.p247);
        let eq160_e2026_q_d_n22: f64 = (eq160_e2024_q_d_n22 * p.p247);
        (eq160_e2026, eq160_e2026_d_n0, eq160_e2026_d_n1, eq160_e2026_d_n2, eq160_e2026_d_n3, eq160_e2026_d_n4, eq160_e2026_d_n5, eq160_e2026_d_n6, eq160_e2026_d_n7, eq160_e2026_d_n8, eq160_e2026_d_n9, eq160_e2026_d_n10, eq160_e2026_d_n11, eq160_e2026_d_n12, eq160_e2026_d_n13, eq160_e2026_d_n14, eq160_e2026_d_n15, eq160_e2026_d_n16, eq160_e2026_d_n17, eq160_e2026_d_n18, eq160_e2026_d_n19, eq160_e2026_d_n20, eq160_e2026_d_n21, eq160_e2026_d_n22, eq160_e2026_q, eq160_e2026_q_d_n0, eq160_e2026_q_d_n1, eq160_e2026_q_d_n2, eq160_e2026_q_d_n3, eq160_e2026_q_d_n4, eq160_e2026_q_d_n5, eq160_e2026_q_d_n6, eq160_e2026_q_d_n7, eq160_e2026_q_d_n8, eq160_e2026_q_d_n9, eq160_e2026_q_d_n10, eq160_e2026_q_d_n11, eq160_e2026_q_d_n12, eq160_e2026_q_d_n13, eq160_e2026_q_d_n14, eq160_e2026_q_d_n15, eq160_e2026_q_d_n16, eq160_e2026_q_d_n17, eq160_e2026_q_d_n18, eq160_e2026_q_d_n19, eq160_e2026_q_d_n20, eq160_e2026_q_d_n21, eq160_e2026_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq160_reactive_node_derivatives: [f64; 23] = [eq160_e2028_q_d_n0, eq160_e2028_q_d_n1, eq160_e2028_q_d_n2, eq160_e2028_q_d_n3, eq160_e2028_q_d_n4, eq160_e2028_q_d_n5, eq160_e2028_q_d_n6, eq160_e2028_q_d_n7, eq160_e2028_q_d_n8, eq160_e2028_q_d_n9, eq160_e2028_q_d_n10, eq160_e2028_q_d_n11, eq160_e2028_q_d_n12, eq160_e2028_q_d_n13, eq160_e2028_q_d_n14, eq160_e2028_q_d_n15, eq160_e2028_q_d_n16, eq160_e2028_q_d_n17, eq160_e2028_q_d_n18, eq160_e2028_q_d_n19, eq160_e2028_q_d_n20, eq160_e2028_q_d_n21, eq160_e2028_q_d_n22];
        let eq160_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[20]),
            &nodes,
            &eq160_reactive_node_derivatives,
            &branches,
            &eq160_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_161_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq161_e2039, eq161_e2039_d_n0, eq161_e2039_d_n1, eq161_e2039_d_n2, eq161_e2039_d_n3, eq161_e2039_d_n4, eq161_e2039_d_n5, eq161_e2039_d_n6, eq161_e2039_d_n7, eq161_e2039_d_n8, eq161_e2039_d_n9, eq161_e2039_d_n10, eq161_e2039_d_n11, eq161_e2039_d_n12, eq161_e2039_d_n13, eq161_e2039_d_n14, eq161_e2039_d_n15, eq161_e2039_d_n16, eq161_e2039_d_n17, eq161_e2039_d_n18, eq161_e2039_d_n19, eq161_e2039_d_n20, eq161_e2039_d_n21, eq161_e2039_d_n22, eq161_e2039_q, eq161_e2039_q_d_n0, eq161_e2039_q_d_n1, eq161_e2039_q_d_n2, eq161_e2039_q_d_n3, eq161_e2039_q_d_n4, eq161_e2039_q_d_n5, eq161_e2039_q_d_n6, eq161_e2039_q_d_n7, eq161_e2039_q_d_n8, eq161_e2039_q_d_n9, eq161_e2039_q_d_n10, eq161_e2039_q_d_n11, eq161_e2039_q_d_n12, eq161_e2039_q_d_n13, eq161_e2039_q_d_n14, eq161_e2039_q_d_n15, eq161_e2039_q_d_n16, eq161_e2039_q_d_n17, eq161_e2039_q_d_n18, eq161_e2039_q_d_n19, eq161_e2039_q_d_n20, eq161_e2039_q_d_n21, eq161_e2039_q_d_n22,) = {
    if ((s.v[585] != 0.0) && (s.v[586] != 0.0)) {
        let eq161_e2035: f64 = (p.p252 * s.v[264]);
        let eq161_e2035_d_n0: f64 = (p.p252 * s.dn[264][0]);
        let eq161_e2035_d_n1: f64 = (p.p252 * s.dn[264][1]);
        let eq161_e2035_d_n2: f64 = (p.p252 * s.dn[264][2]);
        let eq161_e2035_d_n3: f64 = (p.p252 * s.dn[264][3]);
        let eq161_e2035_d_n4: f64 = (p.p252 * s.dn[264][4]);
        let eq161_e2035_d_n5: f64 = (p.p252 * s.dn[264][5]);
        let eq161_e2035_d_n6: f64 = (p.p252 * s.dn[264][6]);
        let eq161_e2035_d_n7: f64 = (p.p252 * s.dn[264][7]);
        let eq161_e2035_d_n8: f64 = (p.p252 * s.dn[264][8]);
        let eq161_e2035_d_n9: f64 = (p.p252 * s.dn[264][9]);
        let eq161_e2035_d_n10: f64 = (p.p252 * s.dn[264][10]);
        let eq161_e2035_d_n11: f64 = (p.p252 * s.dn[264][11]);
        let eq161_e2035_d_n12: f64 = (p.p252 * s.dn[264][12]);
        let eq161_e2035_d_n13: f64 = (p.p252 * s.dn[264][13]);
        let eq161_e2035_d_n14: f64 = (p.p252 * s.dn[264][14]);
        let eq161_e2035_d_n15: f64 = (p.p252 * s.dn[264][15]);
        let eq161_e2035_d_n16: f64 = (p.p252 * s.dn[264][16]);
        let eq161_e2035_d_n17: f64 = (p.p252 * s.dn[264][17]);
        let eq161_e2035_d_n18: f64 = (p.p252 * s.dn[264][18]);
        let eq161_e2035_d_n19: f64 = (p.p252 * s.dn[264][19]);
        let eq161_e2035_d_n20: f64 = (p.p252 * s.dn[264][20]);
        let eq161_e2035_d_n21: f64 = (p.p252 * s.dn[264][21]);
        let eq161_e2035_d_n22: f64 = (p.p252 * s.dn[264][22]);
        let eq161_e2036_q: f64 = eq161_e2035;
        let eq161_e2037: f64 = (p.p7 * eq161_e2035);
        let eq161_e2037_d_n0: f64 = (p.p7 * eq161_e2035_d_n0);
        let eq161_e2037_d_n1: f64 = (p.p7 * eq161_e2035_d_n1);
        let eq161_e2037_d_n2: f64 = (p.p7 * eq161_e2035_d_n2);
        let eq161_e2037_d_n3: f64 = (p.p7 * eq161_e2035_d_n3);
        let eq161_e2037_d_n4: f64 = (p.p7 * eq161_e2035_d_n4);
        let eq161_e2037_d_n5: f64 = (p.p7 * eq161_e2035_d_n5);
        let eq161_e2037_d_n6: f64 = (p.p7 * eq161_e2035_d_n6);
        let eq161_e2037_d_n7: f64 = (p.p7 * eq161_e2035_d_n7);
        let eq161_e2037_d_n8: f64 = (p.p7 * eq161_e2035_d_n8);
        let eq161_e2037_d_n9: f64 = (p.p7 * eq161_e2035_d_n9);
        let eq161_e2037_d_n10: f64 = (p.p7 * eq161_e2035_d_n10);
        let eq161_e2037_d_n11: f64 = (p.p7 * eq161_e2035_d_n11);
        let eq161_e2037_d_n12: f64 = (p.p7 * eq161_e2035_d_n12);
        let eq161_e2037_d_n13: f64 = (p.p7 * eq161_e2035_d_n13);
        let eq161_e2037_d_n14: f64 = (p.p7 * eq161_e2035_d_n14);
        let eq161_e2037_d_n15: f64 = (p.p7 * eq161_e2035_d_n15);
        let eq161_e2037_d_n16: f64 = (p.p7 * eq161_e2035_d_n16);
        let eq161_e2037_d_n17: f64 = (p.p7 * eq161_e2035_d_n17);
        let eq161_e2037_d_n18: f64 = (p.p7 * eq161_e2035_d_n18);
        let eq161_e2037_d_n19: f64 = (p.p7 * eq161_e2035_d_n19);
        let eq161_e2037_d_n20: f64 = (p.p7 * eq161_e2035_d_n20);
        let eq161_e2037_d_n21: f64 = (p.p7 * eq161_e2035_d_n21);
        let eq161_e2037_d_n22: f64 = (p.p7 * eq161_e2035_d_n22);
        let eq161_e2037_q: f64 = (p.p7 * eq161_e2036_q);
        let eq161_e2037_q_d_n0: f64 = (p.p7 * eq161_e2035_d_n0);
        let eq161_e2037_q_d_n1: f64 = (p.p7 * eq161_e2035_d_n1);
        let eq161_e2037_q_d_n2: f64 = (p.p7 * eq161_e2035_d_n2);
        let eq161_e2037_q_d_n3: f64 = (p.p7 * eq161_e2035_d_n3);
        let eq161_e2037_q_d_n4: f64 = (p.p7 * eq161_e2035_d_n4);
        let eq161_e2037_q_d_n5: f64 = (p.p7 * eq161_e2035_d_n5);
        let eq161_e2037_q_d_n6: f64 = (p.p7 * eq161_e2035_d_n6);
        let eq161_e2037_q_d_n7: f64 = (p.p7 * eq161_e2035_d_n7);
        let eq161_e2037_q_d_n8: f64 = (p.p7 * eq161_e2035_d_n8);
        let eq161_e2037_q_d_n9: f64 = (p.p7 * eq161_e2035_d_n9);
        let eq161_e2037_q_d_n10: f64 = (p.p7 * eq161_e2035_d_n10);
        let eq161_e2037_q_d_n11: f64 = (p.p7 * eq161_e2035_d_n11);
        let eq161_e2037_q_d_n12: f64 = (p.p7 * eq161_e2035_d_n12);
        let eq161_e2037_q_d_n13: f64 = (p.p7 * eq161_e2035_d_n13);
        let eq161_e2037_q_d_n14: f64 = (p.p7 * eq161_e2035_d_n14);
        let eq161_e2037_q_d_n15: f64 = (p.p7 * eq161_e2035_d_n15);
        let eq161_e2037_q_d_n16: f64 = (p.p7 * eq161_e2035_d_n16);
        let eq161_e2037_q_d_n17: f64 = (p.p7 * eq161_e2035_d_n17);
        let eq161_e2037_q_d_n18: f64 = (p.p7 * eq161_e2035_d_n18);
        let eq161_e2037_q_d_n19: f64 = (p.p7 * eq161_e2035_d_n19);
        let eq161_e2037_q_d_n20: f64 = (p.p7 * eq161_e2035_d_n20);
        let eq161_e2037_q_d_n21: f64 = (p.p7 * eq161_e2035_d_n21);
        let eq161_e2037_q_d_n22: f64 = (p.p7 * eq161_e2035_d_n22);
        (eq161_e2037, eq161_e2037_d_n0, eq161_e2037_d_n1, eq161_e2037_d_n2, eq161_e2037_d_n3, eq161_e2037_d_n4, eq161_e2037_d_n5, eq161_e2037_d_n6, eq161_e2037_d_n7, eq161_e2037_d_n8, eq161_e2037_d_n9, eq161_e2037_d_n10, eq161_e2037_d_n11, eq161_e2037_d_n12, eq161_e2037_d_n13, eq161_e2037_d_n14, eq161_e2037_d_n15, eq161_e2037_d_n16, eq161_e2037_d_n17, eq161_e2037_d_n18, eq161_e2037_d_n19, eq161_e2037_d_n20, eq161_e2037_d_n21, eq161_e2037_d_n22, eq161_e2037_q, eq161_e2037_q_d_n0, eq161_e2037_q_d_n1, eq161_e2037_q_d_n2, eq161_e2037_q_d_n3, eq161_e2037_q_d_n4, eq161_e2037_q_d_n5, eq161_e2037_q_d_n6, eq161_e2037_q_d_n7, eq161_e2037_q_d_n8, eq161_e2037_q_d_n9, eq161_e2037_q_d_n10, eq161_e2037_q_d_n11, eq161_e2037_q_d_n12, eq161_e2037_q_d_n13, eq161_e2037_q_d_n14, eq161_e2037_q_d_n15, eq161_e2037_q_d_n16, eq161_e2037_q_d_n17, eq161_e2037_q_d_n18, eq161_e2037_q_d_n19, eq161_e2037_q_d_n20, eq161_e2037_q_d_n21, eq161_e2037_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq161_reactive_node_derivatives: [f64; 23] = [eq161_e2039_q_d_n0, eq161_e2039_q_d_n1, eq161_e2039_q_d_n2, eq161_e2039_q_d_n3, eq161_e2039_q_d_n4, eq161_e2039_q_d_n5, eq161_e2039_q_d_n6, eq161_e2039_q_d_n7, eq161_e2039_q_d_n8, eq161_e2039_q_d_n9, eq161_e2039_q_d_n10, eq161_e2039_q_d_n11, eq161_e2039_q_d_n12, eq161_e2039_q_d_n13, eq161_e2039_q_d_n14, eq161_e2039_q_d_n15, eq161_e2039_q_d_n16, eq161_e2039_q_d_n17, eq161_e2039_q_d_n18, eq161_e2039_q_d_n19, eq161_e2039_q_d_n20, eq161_e2039_q_d_n21, eq161_e2039_q_d_n22];
        let eq161_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[20]),
            &nodes,
            &eq161_reactive_node_derivatives,
            &branches,
            &eq161_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_162_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq162_e2049, eq162_e2049_d_n0, eq162_e2049_d_n1, eq162_e2049_d_n2, eq162_e2049_d_n3, eq162_e2049_d_n4, eq162_e2049_d_n5, eq162_e2049_d_n6, eq162_e2049_d_n7, eq162_e2049_d_n8, eq162_e2049_d_n9, eq162_e2049_d_n10, eq162_e2049_d_n11, eq162_e2049_d_n12, eq162_e2049_d_n13, eq162_e2049_d_n14, eq162_e2049_d_n15, eq162_e2049_d_n16, eq162_e2049_d_n17, eq162_e2049_d_n18, eq162_e2049_d_n19, eq162_e2049_d_n20, eq162_e2049_d_n21, eq162_e2049_d_n22, eq162_e2049_q, eq162_e2049_q_d_n0, eq162_e2049_q_d_n1, eq162_e2049_q_d_n2, eq162_e2049_q_d_n3, eq162_e2049_q_d_n4, eq162_e2049_q_d_n5, eq162_e2049_q_d_n6, eq162_e2049_q_d_n7, eq162_e2049_q_d_n8, eq162_e2049_q_d_n9, eq162_e2049_q_d_n10, eq162_e2049_q_d_n11, eq162_e2049_q_d_n12, eq162_e2049_q_d_n13, eq162_e2049_q_d_n14, eq162_e2049_q_d_n15, eq162_e2049_q_d_n16, eq162_e2049_q_d_n17, eq162_e2049_q_d_n18, eq162_e2049_q_d_n19, eq162_e2049_q_d_n20, eq162_e2049_q_d_n21, eq162_e2049_q_d_n22,) = {
    if ((!(s.v[585] != 0.0)) && (s.v[588] != 0.0)) {
        let eq162_e2046_q: f64 = s.v[265];
        let eq162_e2047: f64 = (p.p7 * s.v[265]);
        let eq162_e2047_d_n0: f64 = (p.p7 * s.dn[265][0]);
        let eq162_e2047_d_n1: f64 = (p.p7 * s.dn[265][1]);
        let eq162_e2047_d_n2: f64 = (p.p7 * s.dn[265][2]);
        let eq162_e2047_d_n3: f64 = (p.p7 * s.dn[265][3]);
        let eq162_e2047_d_n4: f64 = (p.p7 * s.dn[265][4]);
        let eq162_e2047_d_n5: f64 = (p.p7 * s.dn[265][5]);
        let eq162_e2047_d_n6: f64 = (p.p7 * s.dn[265][6]);
        let eq162_e2047_d_n7: f64 = (p.p7 * s.dn[265][7]);
        let eq162_e2047_d_n8: f64 = (p.p7 * s.dn[265][8]);
        let eq162_e2047_d_n9: f64 = (p.p7 * s.dn[265][9]);
        let eq162_e2047_d_n10: f64 = (p.p7 * s.dn[265][10]);
        let eq162_e2047_d_n11: f64 = (p.p7 * s.dn[265][11]);
        let eq162_e2047_d_n12: f64 = (p.p7 * s.dn[265][12]);
        let eq162_e2047_d_n13: f64 = (p.p7 * s.dn[265][13]);
        let eq162_e2047_d_n14: f64 = (p.p7 * s.dn[265][14]);
        let eq162_e2047_d_n15: f64 = (p.p7 * s.dn[265][15]);
        let eq162_e2047_d_n16: f64 = (p.p7 * s.dn[265][16]);
        let eq162_e2047_d_n17: f64 = (p.p7 * s.dn[265][17]);
        let eq162_e2047_d_n18: f64 = (p.p7 * s.dn[265][18]);
        let eq162_e2047_d_n19: f64 = (p.p7 * s.dn[265][19]);
        let eq162_e2047_d_n20: f64 = (p.p7 * s.dn[265][20]);
        let eq162_e2047_d_n21: f64 = (p.p7 * s.dn[265][21]);
        let eq162_e2047_d_n22: f64 = (p.p7 * s.dn[265][22]);
        let eq162_e2047_q: f64 = (p.p7 * eq162_e2046_q);
        let eq162_e2047_q_d_n0: f64 = (p.p7 * s.dn[265][0]);
        let eq162_e2047_q_d_n1: f64 = (p.p7 * s.dn[265][1]);
        let eq162_e2047_q_d_n2: f64 = (p.p7 * s.dn[265][2]);
        let eq162_e2047_q_d_n3: f64 = (p.p7 * s.dn[265][3]);
        let eq162_e2047_q_d_n4: f64 = (p.p7 * s.dn[265][4]);
        let eq162_e2047_q_d_n5: f64 = (p.p7 * s.dn[265][5]);
        let eq162_e2047_q_d_n6: f64 = (p.p7 * s.dn[265][6]);
        let eq162_e2047_q_d_n7: f64 = (p.p7 * s.dn[265][7]);
        let eq162_e2047_q_d_n8: f64 = (p.p7 * s.dn[265][8]);
        let eq162_e2047_q_d_n9: f64 = (p.p7 * s.dn[265][9]);
        let eq162_e2047_q_d_n10: f64 = (p.p7 * s.dn[265][10]);
        let eq162_e2047_q_d_n11: f64 = (p.p7 * s.dn[265][11]);
        let eq162_e2047_q_d_n12: f64 = (p.p7 * s.dn[265][12]);
        let eq162_e2047_q_d_n13: f64 = (p.p7 * s.dn[265][13]);
        let eq162_e2047_q_d_n14: f64 = (p.p7 * s.dn[265][14]);
        let eq162_e2047_q_d_n15: f64 = (p.p7 * s.dn[265][15]);
        let eq162_e2047_q_d_n16: f64 = (p.p7 * s.dn[265][16]);
        let eq162_e2047_q_d_n17: f64 = (p.p7 * s.dn[265][17]);
        let eq162_e2047_q_d_n18: f64 = (p.p7 * s.dn[265][18]);
        let eq162_e2047_q_d_n19: f64 = (p.p7 * s.dn[265][19]);
        let eq162_e2047_q_d_n20: f64 = (p.p7 * s.dn[265][20]);
        let eq162_e2047_q_d_n21: f64 = (p.p7 * s.dn[265][21]);
        let eq162_e2047_q_d_n22: f64 = (p.p7 * s.dn[265][22]);
        (eq162_e2047, eq162_e2047_d_n0, eq162_e2047_d_n1, eq162_e2047_d_n2, eq162_e2047_d_n3, eq162_e2047_d_n4, eq162_e2047_d_n5, eq162_e2047_d_n6, eq162_e2047_d_n7, eq162_e2047_d_n8, eq162_e2047_d_n9, eq162_e2047_d_n10, eq162_e2047_d_n11, eq162_e2047_d_n12, eq162_e2047_d_n13, eq162_e2047_d_n14, eq162_e2047_d_n15, eq162_e2047_d_n16, eq162_e2047_d_n17, eq162_e2047_d_n18, eq162_e2047_d_n19, eq162_e2047_d_n20, eq162_e2047_d_n21, eq162_e2047_d_n22, eq162_e2047_q, eq162_e2047_q_d_n0, eq162_e2047_q_d_n1, eq162_e2047_q_d_n2, eq162_e2047_q_d_n3, eq162_e2047_q_d_n4, eq162_e2047_q_d_n5, eq162_e2047_q_d_n6, eq162_e2047_q_d_n7, eq162_e2047_q_d_n8, eq162_e2047_q_d_n9, eq162_e2047_q_d_n10, eq162_e2047_q_d_n11, eq162_e2047_q_d_n12, eq162_e2047_q_d_n13, eq162_e2047_q_d_n14, eq162_e2047_q_d_n15, eq162_e2047_q_d_n16, eq162_e2047_q_d_n17, eq162_e2047_q_d_n18, eq162_e2047_q_d_n19, eq162_e2047_q_d_n20, eq162_e2047_q_d_n21, eq162_e2047_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq162_reactive_node_derivatives: [f64; 23] = [eq162_e2049_q_d_n0, eq162_e2049_q_d_n1, eq162_e2049_q_d_n2, eq162_e2049_q_d_n3, eq162_e2049_q_d_n4, eq162_e2049_q_d_n5, eq162_e2049_q_d_n6, eq162_e2049_q_d_n7, eq162_e2049_q_d_n8, eq162_e2049_q_d_n9, eq162_e2049_q_d_n10, eq162_e2049_q_d_n11, eq162_e2049_q_d_n12, eq162_e2049_q_d_n13, eq162_e2049_q_d_n14, eq162_e2049_q_d_n15, eq162_e2049_q_d_n16, eq162_e2049_q_d_n17, eq162_e2049_q_d_n18, eq162_e2049_q_d_n19, eq162_e2049_q_d_n20, eq162_e2049_q_d_n21, eq162_e2049_q_d_n22];
        let eq162_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            &nodes,
            &eq162_reactive_node_derivatives,
            &branches,
            &eq162_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_163_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq163_e2061, eq163_e2061_d_n0, eq163_e2061_d_n1, eq163_e2061_d_n2, eq163_e2061_d_n3, eq163_e2061_d_n4, eq163_e2061_d_n5, eq163_e2061_d_n6, eq163_e2061_d_n7, eq163_e2061_d_n8, eq163_e2061_d_n9, eq163_e2061_d_n10, eq163_e2061_d_n11, eq163_e2061_d_n12, eq163_e2061_d_n13, eq163_e2061_d_n14, eq163_e2061_d_n15, eq163_e2061_d_n16, eq163_e2061_d_n17, eq163_e2061_d_n18, eq163_e2061_d_n19, eq163_e2061_d_n20, eq163_e2061_d_n21, eq163_e2061_d_n22, eq163_e2061_q, eq163_e2061_q_d_n0, eq163_e2061_q_d_n1, eq163_e2061_q_d_n2, eq163_e2061_q_d_n3, eq163_e2061_q_d_n4, eq163_e2061_q_d_n5, eq163_e2061_q_d_n6, eq163_e2061_q_d_n7, eq163_e2061_q_d_n8, eq163_e2061_q_d_n9, eq163_e2061_q_d_n10, eq163_e2061_q_d_n11, eq163_e2061_q_d_n12, eq163_e2061_q_d_n13, eq163_e2061_q_d_n14, eq163_e2061_q_d_n15, eq163_e2061_q_d_n16, eq163_e2061_q_d_n17, eq163_e2061_q_d_n18, eq163_e2061_q_d_n19, eq163_e2061_q_d_n20, eq163_e2061_q_d_n21, eq163_e2061_q_d_n22,) = {
    if (((!(s.v[585] != 0.0)) && (s.v[588] != 0.0)) && (s.v[589] != 0.0)) {
        let eq163_e2058_q: f64 = s.v[264];
        let eq163_e2059: f64 = (p.p7 * s.v[264]);
        let eq163_e2059_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq163_e2059_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq163_e2059_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq163_e2059_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq163_e2059_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq163_e2059_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq163_e2059_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq163_e2059_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq163_e2059_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq163_e2059_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq163_e2059_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq163_e2059_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq163_e2059_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq163_e2059_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq163_e2059_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq163_e2059_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq163_e2059_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq163_e2059_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq163_e2059_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq163_e2059_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq163_e2059_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq163_e2059_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq163_e2059_d_n22: f64 = (p.p7 * s.dn[264][22]);
        let eq163_e2059_q: f64 = (p.p7 * eq163_e2058_q);
        let eq163_e2059_q_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq163_e2059_q_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq163_e2059_q_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq163_e2059_q_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq163_e2059_q_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq163_e2059_q_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq163_e2059_q_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq163_e2059_q_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq163_e2059_q_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq163_e2059_q_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq163_e2059_q_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq163_e2059_q_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq163_e2059_q_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq163_e2059_q_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq163_e2059_q_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq163_e2059_q_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq163_e2059_q_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq163_e2059_q_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq163_e2059_q_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq163_e2059_q_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq163_e2059_q_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq163_e2059_q_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq163_e2059_q_d_n22: f64 = (p.p7 * s.dn[264][22]);
        (eq163_e2059, eq163_e2059_d_n0, eq163_e2059_d_n1, eq163_e2059_d_n2, eq163_e2059_d_n3, eq163_e2059_d_n4, eq163_e2059_d_n5, eq163_e2059_d_n6, eq163_e2059_d_n7, eq163_e2059_d_n8, eq163_e2059_d_n9, eq163_e2059_d_n10, eq163_e2059_d_n11, eq163_e2059_d_n12, eq163_e2059_d_n13, eq163_e2059_d_n14, eq163_e2059_d_n15, eq163_e2059_d_n16, eq163_e2059_d_n17, eq163_e2059_d_n18, eq163_e2059_d_n19, eq163_e2059_d_n20, eq163_e2059_d_n21, eq163_e2059_d_n22, eq163_e2059_q, eq163_e2059_q_d_n0, eq163_e2059_q_d_n1, eq163_e2059_q_d_n2, eq163_e2059_q_d_n3, eq163_e2059_q_d_n4, eq163_e2059_q_d_n5, eq163_e2059_q_d_n6, eq163_e2059_q_d_n7, eq163_e2059_q_d_n8, eq163_e2059_q_d_n9, eq163_e2059_q_d_n10, eq163_e2059_q_d_n11, eq163_e2059_q_d_n12, eq163_e2059_q_d_n13, eq163_e2059_q_d_n14, eq163_e2059_q_d_n15, eq163_e2059_q_d_n16, eq163_e2059_q_d_n17, eq163_e2059_q_d_n18, eq163_e2059_q_d_n19, eq163_e2059_q_d_n20, eq163_e2059_q_d_n21, eq163_e2059_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq163_reactive_node_derivatives: [f64; 23] = [eq163_e2061_q_d_n0, eq163_e2061_q_d_n1, eq163_e2061_q_d_n2, eq163_e2061_q_d_n3, eq163_e2061_q_d_n4, eq163_e2061_q_d_n5, eq163_e2061_q_d_n6, eq163_e2061_q_d_n7, eq163_e2061_q_d_n8, eq163_e2061_q_d_n9, eq163_e2061_q_d_n10, eq163_e2061_q_d_n11, eq163_e2061_q_d_n12, eq163_e2061_q_d_n13, eq163_e2061_q_d_n14, eq163_e2061_q_d_n15, eq163_e2061_q_d_n16, eq163_e2061_q_d_n17, eq163_e2061_q_d_n18, eq163_e2061_q_d_n19, eq163_e2061_q_d_n20, eq163_e2061_q_d_n21, eq163_e2061_q_d_n22];
        let eq163_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            &nodes,
            &eq163_reactive_node_derivatives,
            &branches,
            &eq163_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_164_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq164_e2075, eq164_e2075_d_n0, eq164_e2075_d_n1, eq164_e2075_d_n2, eq164_e2075_d_n3, eq164_e2075_d_n4, eq164_e2075_d_n5, eq164_e2075_d_n6, eq164_e2075_d_n7, eq164_e2075_d_n8, eq164_e2075_d_n9, eq164_e2075_d_n10, eq164_e2075_d_n11, eq164_e2075_d_n12, eq164_e2075_d_n13, eq164_e2075_d_n14, eq164_e2075_d_n15, eq164_e2075_d_n16, eq164_e2075_d_n17, eq164_e2075_d_n18, eq164_e2075_d_n19, eq164_e2075_d_n20, eq164_e2075_d_n21, eq164_e2075_d_n22, eq164_e2075_q, eq164_e2075_q_d_n0, eq164_e2075_q_d_n1, eq164_e2075_q_d_n2, eq164_e2075_q_d_n3, eq164_e2075_q_d_n4, eq164_e2075_q_d_n5, eq164_e2075_q_d_n6, eq164_e2075_q_d_n7, eq164_e2075_q_d_n8, eq164_e2075_q_d_n9, eq164_e2075_q_d_n10, eq164_e2075_q_d_n11, eq164_e2075_q_d_n12, eq164_e2075_q_d_n13, eq164_e2075_q_d_n14, eq164_e2075_q_d_n15, eq164_e2075_q_d_n16, eq164_e2075_q_d_n17, eq164_e2075_q_d_n18, eq164_e2075_q_d_n19, eq164_e2075_q_d_n20, eq164_e2075_q_d_n21, eq164_e2075_q_d_n22,) = {
    if (((!(s.v[585] != 0.0)) && (s.v[588] != 0.0)) && (s.v[589] != 0.0)) {
        let eq164_e2070_q: f64 = s.v[264];
        let eq164_e2071: f64 = (p.p7 * s.v[264]);
        let eq164_e2071_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq164_e2071_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq164_e2071_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq164_e2071_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq164_e2071_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq164_e2071_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq164_e2071_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq164_e2071_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq164_e2071_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq164_e2071_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq164_e2071_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq164_e2071_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq164_e2071_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq164_e2071_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq164_e2071_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq164_e2071_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq164_e2071_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq164_e2071_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq164_e2071_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq164_e2071_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq164_e2071_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq164_e2071_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq164_e2071_d_n22: f64 = (p.p7 * s.dn[264][22]);
        let eq164_e2071_q: f64 = (p.p7 * eq164_e2070_q);
        let eq164_e2071_q_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq164_e2071_q_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq164_e2071_q_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq164_e2071_q_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq164_e2071_q_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq164_e2071_q_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq164_e2071_q_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq164_e2071_q_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq164_e2071_q_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq164_e2071_q_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq164_e2071_q_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq164_e2071_q_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq164_e2071_q_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq164_e2071_q_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq164_e2071_q_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq164_e2071_q_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq164_e2071_q_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq164_e2071_q_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq164_e2071_q_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq164_e2071_q_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq164_e2071_q_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq164_e2071_q_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq164_e2071_q_d_n22: f64 = (p.p7 * s.dn[264][22]);
        let eq164_e2073: f64 = (eq164_e2071 * p.p247);
        let eq164_e2073_d_n0: f64 = (eq164_e2071_d_n0 * p.p247);
        let eq164_e2073_d_n1: f64 = (eq164_e2071_d_n1 * p.p247);
        let eq164_e2073_d_n2: f64 = (eq164_e2071_d_n2 * p.p247);
        let eq164_e2073_d_n3: f64 = (eq164_e2071_d_n3 * p.p247);
        let eq164_e2073_d_n4: f64 = (eq164_e2071_d_n4 * p.p247);
        let eq164_e2073_d_n5: f64 = (eq164_e2071_d_n5 * p.p247);
        let eq164_e2073_d_n6: f64 = (eq164_e2071_d_n6 * p.p247);
        let eq164_e2073_d_n7: f64 = (eq164_e2071_d_n7 * p.p247);
        let eq164_e2073_d_n8: f64 = (eq164_e2071_d_n8 * p.p247);
        let eq164_e2073_d_n9: f64 = (eq164_e2071_d_n9 * p.p247);
        let eq164_e2073_d_n10: f64 = (eq164_e2071_d_n10 * p.p247);
        let eq164_e2073_d_n11: f64 = (eq164_e2071_d_n11 * p.p247);
        let eq164_e2073_d_n12: f64 = (eq164_e2071_d_n12 * p.p247);
        let eq164_e2073_d_n13: f64 = (eq164_e2071_d_n13 * p.p247);
        let eq164_e2073_d_n14: f64 = (eq164_e2071_d_n14 * p.p247);
        let eq164_e2073_d_n15: f64 = (eq164_e2071_d_n15 * p.p247);
        let eq164_e2073_d_n16: f64 = (eq164_e2071_d_n16 * p.p247);
        let eq164_e2073_d_n17: f64 = (eq164_e2071_d_n17 * p.p247);
        let eq164_e2073_d_n18: f64 = (eq164_e2071_d_n18 * p.p247);
        let eq164_e2073_d_n19: f64 = (eq164_e2071_d_n19 * p.p247);
        let eq164_e2073_d_n20: f64 = (eq164_e2071_d_n20 * p.p247);
        let eq164_e2073_d_n21: f64 = (eq164_e2071_d_n21 * p.p247);
        let eq164_e2073_d_n22: f64 = (eq164_e2071_d_n22 * p.p247);
        let eq164_e2073_q: f64 = (eq164_e2071_q * p.p247);
        let eq164_e2073_q_d_n0: f64 = (eq164_e2071_q_d_n0 * p.p247);
        let eq164_e2073_q_d_n1: f64 = (eq164_e2071_q_d_n1 * p.p247);
        let eq164_e2073_q_d_n2: f64 = (eq164_e2071_q_d_n2 * p.p247);
        let eq164_e2073_q_d_n3: f64 = (eq164_e2071_q_d_n3 * p.p247);
        let eq164_e2073_q_d_n4: f64 = (eq164_e2071_q_d_n4 * p.p247);
        let eq164_e2073_q_d_n5: f64 = (eq164_e2071_q_d_n5 * p.p247);
        let eq164_e2073_q_d_n6: f64 = (eq164_e2071_q_d_n6 * p.p247);
        let eq164_e2073_q_d_n7: f64 = (eq164_e2071_q_d_n7 * p.p247);
        let eq164_e2073_q_d_n8: f64 = (eq164_e2071_q_d_n8 * p.p247);
        let eq164_e2073_q_d_n9: f64 = (eq164_e2071_q_d_n9 * p.p247);
        let eq164_e2073_q_d_n10: f64 = (eq164_e2071_q_d_n10 * p.p247);
        let eq164_e2073_q_d_n11: f64 = (eq164_e2071_q_d_n11 * p.p247);
        let eq164_e2073_q_d_n12: f64 = (eq164_e2071_q_d_n12 * p.p247);
        let eq164_e2073_q_d_n13: f64 = (eq164_e2071_q_d_n13 * p.p247);
        let eq164_e2073_q_d_n14: f64 = (eq164_e2071_q_d_n14 * p.p247);
        let eq164_e2073_q_d_n15: f64 = (eq164_e2071_q_d_n15 * p.p247);
        let eq164_e2073_q_d_n16: f64 = (eq164_e2071_q_d_n16 * p.p247);
        let eq164_e2073_q_d_n17: f64 = (eq164_e2071_q_d_n17 * p.p247);
        let eq164_e2073_q_d_n18: f64 = (eq164_e2071_q_d_n18 * p.p247);
        let eq164_e2073_q_d_n19: f64 = (eq164_e2071_q_d_n19 * p.p247);
        let eq164_e2073_q_d_n20: f64 = (eq164_e2071_q_d_n20 * p.p247);
        let eq164_e2073_q_d_n21: f64 = (eq164_e2071_q_d_n21 * p.p247);
        let eq164_e2073_q_d_n22: f64 = (eq164_e2071_q_d_n22 * p.p247);
        (eq164_e2073, eq164_e2073_d_n0, eq164_e2073_d_n1, eq164_e2073_d_n2, eq164_e2073_d_n3, eq164_e2073_d_n4, eq164_e2073_d_n5, eq164_e2073_d_n6, eq164_e2073_d_n7, eq164_e2073_d_n8, eq164_e2073_d_n9, eq164_e2073_d_n10, eq164_e2073_d_n11, eq164_e2073_d_n12, eq164_e2073_d_n13, eq164_e2073_d_n14, eq164_e2073_d_n15, eq164_e2073_d_n16, eq164_e2073_d_n17, eq164_e2073_d_n18, eq164_e2073_d_n19, eq164_e2073_d_n20, eq164_e2073_d_n21, eq164_e2073_d_n22, eq164_e2073_q, eq164_e2073_q_d_n0, eq164_e2073_q_d_n1, eq164_e2073_q_d_n2, eq164_e2073_q_d_n3, eq164_e2073_q_d_n4, eq164_e2073_q_d_n5, eq164_e2073_q_d_n6, eq164_e2073_q_d_n7, eq164_e2073_q_d_n8, eq164_e2073_q_d_n9, eq164_e2073_q_d_n10, eq164_e2073_q_d_n11, eq164_e2073_q_d_n12, eq164_e2073_q_d_n13, eq164_e2073_q_d_n14, eq164_e2073_q_d_n15, eq164_e2073_q_d_n16, eq164_e2073_q_d_n17, eq164_e2073_q_d_n18, eq164_e2073_q_d_n19, eq164_e2073_q_d_n20, eq164_e2073_q_d_n21, eq164_e2073_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq164_reactive_node_derivatives: [f64; 23] = [eq164_e2075_q_d_n0, eq164_e2075_q_d_n1, eq164_e2075_q_d_n2, eq164_e2075_q_d_n3, eq164_e2075_q_d_n4, eq164_e2075_q_d_n5, eq164_e2075_q_d_n6, eq164_e2075_q_d_n7, eq164_e2075_q_d_n8, eq164_e2075_q_d_n9, eq164_e2075_q_d_n10, eq164_e2075_q_d_n11, eq164_e2075_q_d_n12, eq164_e2075_q_d_n13, eq164_e2075_q_d_n14, eq164_e2075_q_d_n15, eq164_e2075_q_d_n16, eq164_e2075_q_d_n17, eq164_e2075_q_d_n18, eq164_e2075_q_d_n19, eq164_e2075_q_d_n20, eq164_e2075_q_d_n21, eq164_e2075_q_d_n22];
        let eq164_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            &nodes,
            &eq164_reactive_node_derivatives,
            &branches,
            &eq164_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_165_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq165_e2088, eq165_e2088_d_n0, eq165_e2088_d_n1, eq165_e2088_d_n2, eq165_e2088_d_n3, eq165_e2088_d_n4, eq165_e2088_d_n5, eq165_e2088_d_n6, eq165_e2088_d_n7, eq165_e2088_d_n8, eq165_e2088_d_n9, eq165_e2088_d_n10, eq165_e2088_d_n11, eq165_e2088_d_n12, eq165_e2088_d_n13, eq165_e2088_d_n14, eq165_e2088_d_n15, eq165_e2088_d_n16, eq165_e2088_d_n17, eq165_e2088_d_n18, eq165_e2088_d_n19, eq165_e2088_d_n20, eq165_e2088_d_n21, eq165_e2088_d_n22, eq165_e2088_q, eq165_e2088_q_d_n0, eq165_e2088_q_d_n1, eq165_e2088_q_d_n2, eq165_e2088_q_d_n3, eq165_e2088_q_d_n4, eq165_e2088_q_d_n5, eq165_e2088_q_d_n6, eq165_e2088_q_d_n7, eq165_e2088_q_d_n8, eq165_e2088_q_d_n9, eq165_e2088_q_d_n10, eq165_e2088_q_d_n11, eq165_e2088_q_d_n12, eq165_e2088_q_d_n13, eq165_e2088_q_d_n14, eq165_e2088_q_d_n15, eq165_e2088_q_d_n16, eq165_e2088_q_d_n17, eq165_e2088_q_d_n18, eq165_e2088_q_d_n19, eq165_e2088_q_d_n20, eq165_e2088_q_d_n21, eq165_e2088_q_d_n22,) = {
    if (((!(s.v[585] != 0.0)) && (s.v[588] != 0.0)) && (!(s.v[589] != 0.0))) {
        let eq165_e2085_q: f64 = s.v[264];
        let eq165_e2086: f64 = (p.p7 * s.v[264]);
        let eq165_e2086_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq165_e2086_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq165_e2086_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq165_e2086_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq165_e2086_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq165_e2086_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq165_e2086_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq165_e2086_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq165_e2086_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq165_e2086_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq165_e2086_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq165_e2086_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq165_e2086_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq165_e2086_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq165_e2086_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq165_e2086_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq165_e2086_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq165_e2086_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq165_e2086_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq165_e2086_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq165_e2086_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq165_e2086_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq165_e2086_d_n22: f64 = (p.p7 * s.dn[264][22]);
        let eq165_e2086_q: f64 = (p.p7 * eq165_e2085_q);
        let eq165_e2086_q_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq165_e2086_q_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq165_e2086_q_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq165_e2086_q_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq165_e2086_q_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq165_e2086_q_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq165_e2086_q_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq165_e2086_q_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq165_e2086_q_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq165_e2086_q_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq165_e2086_q_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq165_e2086_q_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq165_e2086_q_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq165_e2086_q_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq165_e2086_q_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq165_e2086_q_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq165_e2086_q_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq165_e2086_q_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq165_e2086_q_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq165_e2086_q_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq165_e2086_q_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq165_e2086_q_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq165_e2086_q_d_n22: f64 = (p.p7 * s.dn[264][22]);
        (eq165_e2086, eq165_e2086_d_n0, eq165_e2086_d_n1, eq165_e2086_d_n2, eq165_e2086_d_n3, eq165_e2086_d_n4, eq165_e2086_d_n5, eq165_e2086_d_n6, eq165_e2086_d_n7, eq165_e2086_d_n8, eq165_e2086_d_n9, eq165_e2086_d_n10, eq165_e2086_d_n11, eq165_e2086_d_n12, eq165_e2086_d_n13, eq165_e2086_d_n14, eq165_e2086_d_n15, eq165_e2086_d_n16, eq165_e2086_d_n17, eq165_e2086_d_n18, eq165_e2086_d_n19, eq165_e2086_d_n20, eq165_e2086_d_n21, eq165_e2086_d_n22, eq165_e2086_q, eq165_e2086_q_d_n0, eq165_e2086_q_d_n1, eq165_e2086_q_d_n2, eq165_e2086_q_d_n3, eq165_e2086_q_d_n4, eq165_e2086_q_d_n5, eq165_e2086_q_d_n6, eq165_e2086_q_d_n7, eq165_e2086_q_d_n8, eq165_e2086_q_d_n9, eq165_e2086_q_d_n10, eq165_e2086_q_d_n11, eq165_e2086_q_d_n12, eq165_e2086_q_d_n13, eq165_e2086_q_d_n14, eq165_e2086_q_d_n15, eq165_e2086_q_d_n16, eq165_e2086_q_d_n17, eq165_e2086_q_d_n18, eq165_e2086_q_d_n19, eq165_e2086_q_d_n20, eq165_e2086_q_d_n21, eq165_e2086_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq165_reactive_node_derivatives: [f64; 23] = [eq165_e2088_q_d_n0, eq165_e2088_q_d_n1, eq165_e2088_q_d_n2, eq165_e2088_q_d_n3, eq165_e2088_q_d_n4, eq165_e2088_q_d_n5, eq165_e2088_q_d_n6, eq165_e2088_q_d_n7, eq165_e2088_q_d_n8, eq165_e2088_q_d_n9, eq165_e2088_q_d_n10, eq165_e2088_q_d_n11, eq165_e2088_q_d_n12, eq165_e2088_q_d_n13, eq165_e2088_q_d_n14, eq165_e2088_q_d_n15, eq165_e2088_q_d_n16, eq165_e2088_q_d_n17, eq165_e2088_q_d_n18, eq165_e2088_q_d_n19, eq165_e2088_q_d_n20, eq165_e2088_q_d_n21, eq165_e2088_q_d_n22];
        let eq165_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            &nodes,
            &eq165_reactive_node_derivatives,
            &branches,
            &eq165_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_166_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq166_e2103, eq166_e2103_d_n0, eq166_e2103_d_n1, eq166_e2103_d_n2, eq166_e2103_d_n3, eq166_e2103_d_n4, eq166_e2103_d_n5, eq166_e2103_d_n6, eq166_e2103_d_n7, eq166_e2103_d_n8, eq166_e2103_d_n9, eq166_e2103_d_n10, eq166_e2103_d_n11, eq166_e2103_d_n12, eq166_e2103_d_n13, eq166_e2103_d_n14, eq166_e2103_d_n15, eq166_e2103_d_n16, eq166_e2103_d_n17, eq166_e2103_d_n18, eq166_e2103_d_n19, eq166_e2103_d_n20, eq166_e2103_d_n21, eq166_e2103_d_n22, eq166_e2103_q, eq166_e2103_q_d_n0, eq166_e2103_q_d_n1, eq166_e2103_q_d_n2, eq166_e2103_q_d_n3, eq166_e2103_q_d_n4, eq166_e2103_q_d_n5, eq166_e2103_q_d_n6, eq166_e2103_q_d_n7, eq166_e2103_q_d_n8, eq166_e2103_q_d_n9, eq166_e2103_q_d_n10, eq166_e2103_q_d_n11, eq166_e2103_q_d_n12, eq166_e2103_q_d_n13, eq166_e2103_q_d_n14, eq166_e2103_q_d_n15, eq166_e2103_q_d_n16, eq166_e2103_q_d_n17, eq166_e2103_q_d_n18, eq166_e2103_q_d_n19, eq166_e2103_q_d_n20, eq166_e2103_q_d_n21, eq166_e2103_q_d_n22,) = {
    if (((!(s.v[585] != 0.0)) && (s.v[588] != 0.0)) && (!(s.v[589] != 0.0))) {
        let eq166_e2098_q: f64 = s.v[264];
        let eq166_e2099: f64 = (p.p7 * s.v[264]);
        let eq166_e2099_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq166_e2099_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq166_e2099_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq166_e2099_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq166_e2099_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq166_e2099_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq166_e2099_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq166_e2099_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq166_e2099_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq166_e2099_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq166_e2099_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq166_e2099_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq166_e2099_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq166_e2099_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq166_e2099_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq166_e2099_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq166_e2099_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq166_e2099_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq166_e2099_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq166_e2099_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq166_e2099_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq166_e2099_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq166_e2099_d_n22: f64 = (p.p7 * s.dn[264][22]);
        let eq166_e2099_q: f64 = (p.p7 * eq166_e2098_q);
        let eq166_e2099_q_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq166_e2099_q_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq166_e2099_q_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq166_e2099_q_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq166_e2099_q_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq166_e2099_q_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq166_e2099_q_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq166_e2099_q_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq166_e2099_q_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq166_e2099_q_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq166_e2099_q_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq166_e2099_q_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq166_e2099_q_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq166_e2099_q_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq166_e2099_q_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq166_e2099_q_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq166_e2099_q_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq166_e2099_q_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq166_e2099_q_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq166_e2099_q_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq166_e2099_q_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq166_e2099_q_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq166_e2099_q_d_n22: f64 = (p.p7 * s.dn[264][22]);
        let eq166_e2101: f64 = (eq166_e2099 * p.p247);
        let eq166_e2101_d_n0: f64 = (eq166_e2099_d_n0 * p.p247);
        let eq166_e2101_d_n1: f64 = (eq166_e2099_d_n1 * p.p247);
        let eq166_e2101_d_n2: f64 = (eq166_e2099_d_n2 * p.p247);
        let eq166_e2101_d_n3: f64 = (eq166_e2099_d_n3 * p.p247);
        let eq166_e2101_d_n4: f64 = (eq166_e2099_d_n4 * p.p247);
        let eq166_e2101_d_n5: f64 = (eq166_e2099_d_n5 * p.p247);
        let eq166_e2101_d_n6: f64 = (eq166_e2099_d_n6 * p.p247);
        let eq166_e2101_d_n7: f64 = (eq166_e2099_d_n7 * p.p247);
        let eq166_e2101_d_n8: f64 = (eq166_e2099_d_n8 * p.p247);
        let eq166_e2101_d_n9: f64 = (eq166_e2099_d_n9 * p.p247);
        let eq166_e2101_d_n10: f64 = (eq166_e2099_d_n10 * p.p247);
        let eq166_e2101_d_n11: f64 = (eq166_e2099_d_n11 * p.p247);
        let eq166_e2101_d_n12: f64 = (eq166_e2099_d_n12 * p.p247);
        let eq166_e2101_d_n13: f64 = (eq166_e2099_d_n13 * p.p247);
        let eq166_e2101_d_n14: f64 = (eq166_e2099_d_n14 * p.p247);
        let eq166_e2101_d_n15: f64 = (eq166_e2099_d_n15 * p.p247);
        let eq166_e2101_d_n16: f64 = (eq166_e2099_d_n16 * p.p247);
        let eq166_e2101_d_n17: f64 = (eq166_e2099_d_n17 * p.p247);
        let eq166_e2101_d_n18: f64 = (eq166_e2099_d_n18 * p.p247);
        let eq166_e2101_d_n19: f64 = (eq166_e2099_d_n19 * p.p247);
        let eq166_e2101_d_n20: f64 = (eq166_e2099_d_n20 * p.p247);
        let eq166_e2101_d_n21: f64 = (eq166_e2099_d_n21 * p.p247);
        let eq166_e2101_d_n22: f64 = (eq166_e2099_d_n22 * p.p247);
        let eq166_e2101_q: f64 = (eq166_e2099_q * p.p247);
        let eq166_e2101_q_d_n0: f64 = (eq166_e2099_q_d_n0 * p.p247);
        let eq166_e2101_q_d_n1: f64 = (eq166_e2099_q_d_n1 * p.p247);
        let eq166_e2101_q_d_n2: f64 = (eq166_e2099_q_d_n2 * p.p247);
        let eq166_e2101_q_d_n3: f64 = (eq166_e2099_q_d_n3 * p.p247);
        let eq166_e2101_q_d_n4: f64 = (eq166_e2099_q_d_n4 * p.p247);
        let eq166_e2101_q_d_n5: f64 = (eq166_e2099_q_d_n5 * p.p247);
        let eq166_e2101_q_d_n6: f64 = (eq166_e2099_q_d_n6 * p.p247);
        let eq166_e2101_q_d_n7: f64 = (eq166_e2099_q_d_n7 * p.p247);
        let eq166_e2101_q_d_n8: f64 = (eq166_e2099_q_d_n8 * p.p247);
        let eq166_e2101_q_d_n9: f64 = (eq166_e2099_q_d_n9 * p.p247);
        let eq166_e2101_q_d_n10: f64 = (eq166_e2099_q_d_n10 * p.p247);
        let eq166_e2101_q_d_n11: f64 = (eq166_e2099_q_d_n11 * p.p247);
        let eq166_e2101_q_d_n12: f64 = (eq166_e2099_q_d_n12 * p.p247);
        let eq166_e2101_q_d_n13: f64 = (eq166_e2099_q_d_n13 * p.p247);
        let eq166_e2101_q_d_n14: f64 = (eq166_e2099_q_d_n14 * p.p247);
        let eq166_e2101_q_d_n15: f64 = (eq166_e2099_q_d_n15 * p.p247);
        let eq166_e2101_q_d_n16: f64 = (eq166_e2099_q_d_n16 * p.p247);
        let eq166_e2101_q_d_n17: f64 = (eq166_e2099_q_d_n17 * p.p247);
        let eq166_e2101_q_d_n18: f64 = (eq166_e2099_q_d_n18 * p.p247);
        let eq166_e2101_q_d_n19: f64 = (eq166_e2099_q_d_n19 * p.p247);
        let eq166_e2101_q_d_n20: f64 = (eq166_e2099_q_d_n20 * p.p247);
        let eq166_e2101_q_d_n21: f64 = (eq166_e2099_q_d_n21 * p.p247);
        let eq166_e2101_q_d_n22: f64 = (eq166_e2099_q_d_n22 * p.p247);
        (eq166_e2101, eq166_e2101_d_n0, eq166_e2101_d_n1, eq166_e2101_d_n2, eq166_e2101_d_n3, eq166_e2101_d_n4, eq166_e2101_d_n5, eq166_e2101_d_n6, eq166_e2101_d_n7, eq166_e2101_d_n8, eq166_e2101_d_n9, eq166_e2101_d_n10, eq166_e2101_d_n11, eq166_e2101_d_n12, eq166_e2101_d_n13, eq166_e2101_d_n14, eq166_e2101_d_n15, eq166_e2101_d_n16, eq166_e2101_d_n17, eq166_e2101_d_n18, eq166_e2101_d_n19, eq166_e2101_d_n20, eq166_e2101_d_n21, eq166_e2101_d_n22, eq166_e2101_q, eq166_e2101_q_d_n0, eq166_e2101_q_d_n1, eq166_e2101_q_d_n2, eq166_e2101_q_d_n3, eq166_e2101_q_d_n4, eq166_e2101_q_d_n5, eq166_e2101_q_d_n6, eq166_e2101_q_d_n7, eq166_e2101_q_d_n8, eq166_e2101_q_d_n9, eq166_e2101_q_d_n10, eq166_e2101_q_d_n11, eq166_e2101_q_d_n12, eq166_e2101_q_d_n13, eq166_e2101_q_d_n14, eq166_e2101_q_d_n15, eq166_e2101_q_d_n16, eq166_e2101_q_d_n17, eq166_e2101_q_d_n18, eq166_e2101_q_d_n19, eq166_e2101_q_d_n20, eq166_e2101_q_d_n21, eq166_e2101_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq166_reactive_node_derivatives: [f64; 23] = [eq166_e2103_q_d_n0, eq166_e2103_q_d_n1, eq166_e2103_q_d_n2, eq166_e2103_q_d_n3, eq166_e2103_q_d_n4, eq166_e2103_q_d_n5, eq166_e2103_q_d_n6, eq166_e2103_q_d_n7, eq166_e2103_q_d_n8, eq166_e2103_q_d_n9, eq166_e2103_q_d_n10, eq166_e2103_q_d_n11, eq166_e2103_q_d_n12, eq166_e2103_q_d_n13, eq166_e2103_q_d_n14, eq166_e2103_q_d_n15, eq166_e2103_q_d_n16, eq166_e2103_q_d_n17, eq166_e2103_q_d_n18, eq166_e2103_q_d_n19, eq166_e2103_q_d_n20, eq166_e2103_q_d_n21, eq166_e2103_q_d_n22];
        let eq166_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            &nodes,
            &eq166_reactive_node_derivatives,
            &branches,
            &eq166_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_167_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq167_e2115, eq167_e2115_d_n0, eq167_e2115_d_n1, eq167_e2115_d_n2, eq167_e2115_d_n3, eq167_e2115_d_n4, eq167_e2115_d_n5, eq167_e2115_d_n6, eq167_e2115_d_n7, eq167_e2115_d_n8, eq167_e2115_d_n9, eq167_e2115_d_n10, eq167_e2115_d_n11, eq167_e2115_d_n12, eq167_e2115_d_n13, eq167_e2115_d_n14, eq167_e2115_d_n15, eq167_e2115_d_n16, eq167_e2115_d_n17, eq167_e2115_d_n18, eq167_e2115_d_n19, eq167_e2115_d_n20, eq167_e2115_d_n21, eq167_e2115_d_n22, eq167_e2115_q, eq167_e2115_q_d_n0, eq167_e2115_q_d_n1, eq167_e2115_q_d_n2, eq167_e2115_q_d_n3, eq167_e2115_q_d_n4, eq167_e2115_q_d_n5, eq167_e2115_q_d_n6, eq167_e2115_q_d_n7, eq167_e2115_q_d_n8, eq167_e2115_q_d_n9, eq167_e2115_q_d_n10, eq167_e2115_q_d_n11, eq167_e2115_q_d_n12, eq167_e2115_q_d_n13, eq167_e2115_q_d_n14, eq167_e2115_q_d_n15, eq167_e2115_q_d_n16, eq167_e2115_q_d_n17, eq167_e2115_q_d_n18, eq167_e2115_q_d_n19, eq167_e2115_q_d_n20, eq167_e2115_q_d_n21, eq167_e2115_q_d_n22,) = {
    if ((!(s.v[585] != 0.0)) && (s.v[588] != 0.0)) {
        let eq167_e2111: f64 = (p.p252 * s.v[264]);
        let eq167_e2111_d_n0: f64 = (p.p252 * s.dn[264][0]);
        let eq167_e2111_d_n1: f64 = (p.p252 * s.dn[264][1]);
        let eq167_e2111_d_n2: f64 = (p.p252 * s.dn[264][2]);
        let eq167_e2111_d_n3: f64 = (p.p252 * s.dn[264][3]);
        let eq167_e2111_d_n4: f64 = (p.p252 * s.dn[264][4]);
        let eq167_e2111_d_n5: f64 = (p.p252 * s.dn[264][5]);
        let eq167_e2111_d_n6: f64 = (p.p252 * s.dn[264][6]);
        let eq167_e2111_d_n7: f64 = (p.p252 * s.dn[264][7]);
        let eq167_e2111_d_n8: f64 = (p.p252 * s.dn[264][8]);
        let eq167_e2111_d_n9: f64 = (p.p252 * s.dn[264][9]);
        let eq167_e2111_d_n10: f64 = (p.p252 * s.dn[264][10]);
        let eq167_e2111_d_n11: f64 = (p.p252 * s.dn[264][11]);
        let eq167_e2111_d_n12: f64 = (p.p252 * s.dn[264][12]);
        let eq167_e2111_d_n13: f64 = (p.p252 * s.dn[264][13]);
        let eq167_e2111_d_n14: f64 = (p.p252 * s.dn[264][14]);
        let eq167_e2111_d_n15: f64 = (p.p252 * s.dn[264][15]);
        let eq167_e2111_d_n16: f64 = (p.p252 * s.dn[264][16]);
        let eq167_e2111_d_n17: f64 = (p.p252 * s.dn[264][17]);
        let eq167_e2111_d_n18: f64 = (p.p252 * s.dn[264][18]);
        let eq167_e2111_d_n19: f64 = (p.p252 * s.dn[264][19]);
        let eq167_e2111_d_n20: f64 = (p.p252 * s.dn[264][20]);
        let eq167_e2111_d_n21: f64 = (p.p252 * s.dn[264][21]);
        let eq167_e2111_d_n22: f64 = (p.p252 * s.dn[264][22]);
        let eq167_e2112_q: f64 = eq167_e2111;
        let eq167_e2113: f64 = (p.p7 * eq167_e2111);
        let eq167_e2113_d_n0: f64 = (p.p7 * eq167_e2111_d_n0);
        let eq167_e2113_d_n1: f64 = (p.p7 * eq167_e2111_d_n1);
        let eq167_e2113_d_n2: f64 = (p.p7 * eq167_e2111_d_n2);
        let eq167_e2113_d_n3: f64 = (p.p7 * eq167_e2111_d_n3);
        let eq167_e2113_d_n4: f64 = (p.p7 * eq167_e2111_d_n4);
        let eq167_e2113_d_n5: f64 = (p.p7 * eq167_e2111_d_n5);
        let eq167_e2113_d_n6: f64 = (p.p7 * eq167_e2111_d_n6);
        let eq167_e2113_d_n7: f64 = (p.p7 * eq167_e2111_d_n7);
        let eq167_e2113_d_n8: f64 = (p.p7 * eq167_e2111_d_n8);
        let eq167_e2113_d_n9: f64 = (p.p7 * eq167_e2111_d_n9);
        let eq167_e2113_d_n10: f64 = (p.p7 * eq167_e2111_d_n10);
        let eq167_e2113_d_n11: f64 = (p.p7 * eq167_e2111_d_n11);
        let eq167_e2113_d_n12: f64 = (p.p7 * eq167_e2111_d_n12);
        let eq167_e2113_d_n13: f64 = (p.p7 * eq167_e2111_d_n13);
        let eq167_e2113_d_n14: f64 = (p.p7 * eq167_e2111_d_n14);
        let eq167_e2113_d_n15: f64 = (p.p7 * eq167_e2111_d_n15);
        let eq167_e2113_d_n16: f64 = (p.p7 * eq167_e2111_d_n16);
        let eq167_e2113_d_n17: f64 = (p.p7 * eq167_e2111_d_n17);
        let eq167_e2113_d_n18: f64 = (p.p7 * eq167_e2111_d_n18);
        let eq167_e2113_d_n19: f64 = (p.p7 * eq167_e2111_d_n19);
        let eq167_e2113_d_n20: f64 = (p.p7 * eq167_e2111_d_n20);
        let eq167_e2113_d_n21: f64 = (p.p7 * eq167_e2111_d_n21);
        let eq167_e2113_d_n22: f64 = (p.p7 * eq167_e2111_d_n22);
        let eq167_e2113_q: f64 = (p.p7 * eq167_e2112_q);
        let eq167_e2113_q_d_n0: f64 = (p.p7 * eq167_e2111_d_n0);
        let eq167_e2113_q_d_n1: f64 = (p.p7 * eq167_e2111_d_n1);
        let eq167_e2113_q_d_n2: f64 = (p.p7 * eq167_e2111_d_n2);
        let eq167_e2113_q_d_n3: f64 = (p.p7 * eq167_e2111_d_n3);
        let eq167_e2113_q_d_n4: f64 = (p.p7 * eq167_e2111_d_n4);
        let eq167_e2113_q_d_n5: f64 = (p.p7 * eq167_e2111_d_n5);
        let eq167_e2113_q_d_n6: f64 = (p.p7 * eq167_e2111_d_n6);
        let eq167_e2113_q_d_n7: f64 = (p.p7 * eq167_e2111_d_n7);
        let eq167_e2113_q_d_n8: f64 = (p.p7 * eq167_e2111_d_n8);
        let eq167_e2113_q_d_n9: f64 = (p.p7 * eq167_e2111_d_n9);
        let eq167_e2113_q_d_n10: f64 = (p.p7 * eq167_e2111_d_n10);
        let eq167_e2113_q_d_n11: f64 = (p.p7 * eq167_e2111_d_n11);
        let eq167_e2113_q_d_n12: f64 = (p.p7 * eq167_e2111_d_n12);
        let eq167_e2113_q_d_n13: f64 = (p.p7 * eq167_e2111_d_n13);
        let eq167_e2113_q_d_n14: f64 = (p.p7 * eq167_e2111_d_n14);
        let eq167_e2113_q_d_n15: f64 = (p.p7 * eq167_e2111_d_n15);
        let eq167_e2113_q_d_n16: f64 = (p.p7 * eq167_e2111_d_n16);
        let eq167_e2113_q_d_n17: f64 = (p.p7 * eq167_e2111_d_n17);
        let eq167_e2113_q_d_n18: f64 = (p.p7 * eq167_e2111_d_n18);
        let eq167_e2113_q_d_n19: f64 = (p.p7 * eq167_e2111_d_n19);
        let eq167_e2113_q_d_n20: f64 = (p.p7 * eq167_e2111_d_n20);
        let eq167_e2113_q_d_n21: f64 = (p.p7 * eq167_e2111_d_n21);
        let eq167_e2113_q_d_n22: f64 = (p.p7 * eq167_e2111_d_n22);
        (eq167_e2113, eq167_e2113_d_n0, eq167_e2113_d_n1, eq167_e2113_d_n2, eq167_e2113_d_n3, eq167_e2113_d_n4, eq167_e2113_d_n5, eq167_e2113_d_n6, eq167_e2113_d_n7, eq167_e2113_d_n8, eq167_e2113_d_n9, eq167_e2113_d_n10, eq167_e2113_d_n11, eq167_e2113_d_n12, eq167_e2113_d_n13, eq167_e2113_d_n14, eq167_e2113_d_n15, eq167_e2113_d_n16, eq167_e2113_d_n17, eq167_e2113_d_n18, eq167_e2113_d_n19, eq167_e2113_d_n20, eq167_e2113_d_n21, eq167_e2113_d_n22, eq167_e2113_q, eq167_e2113_q_d_n0, eq167_e2113_q_d_n1, eq167_e2113_q_d_n2, eq167_e2113_q_d_n3, eq167_e2113_q_d_n4, eq167_e2113_q_d_n5, eq167_e2113_q_d_n6, eq167_e2113_q_d_n7, eq167_e2113_q_d_n8, eq167_e2113_q_d_n9, eq167_e2113_q_d_n10, eq167_e2113_q_d_n11, eq167_e2113_q_d_n12, eq167_e2113_q_d_n13, eq167_e2113_q_d_n14, eq167_e2113_q_d_n15, eq167_e2113_q_d_n16, eq167_e2113_q_d_n17, eq167_e2113_q_d_n18, eq167_e2113_q_d_n19, eq167_e2113_q_d_n20, eq167_e2113_q_d_n21, eq167_e2113_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq167_reactive_node_derivatives: [f64; 23] = [eq167_e2115_q_d_n0, eq167_e2115_q_d_n1, eq167_e2115_q_d_n2, eq167_e2115_q_d_n3, eq167_e2115_q_d_n4, eq167_e2115_q_d_n5, eq167_e2115_q_d_n6, eq167_e2115_q_d_n7, eq167_e2115_q_d_n8, eq167_e2115_q_d_n9, eq167_e2115_q_d_n10, eq167_e2115_q_d_n11, eq167_e2115_q_d_n12, eq167_e2115_q_d_n13, eq167_e2115_q_d_n14, eq167_e2115_q_d_n15, eq167_e2115_q_d_n16, eq167_e2115_q_d_n17, eq167_e2115_q_d_n18, eq167_e2115_q_d_n19, eq167_e2115_q_d_n20, eq167_e2115_q_d_n21, eq167_e2115_q_d_n22];
        let eq167_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            &nodes,
            &eq167_reactive_node_derivatives,
            &branches,
            &eq167_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_168_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq168_e2124, eq168_e2124_d_n0, eq168_e2124_d_n1, eq168_e2124_d_n2, eq168_e2124_d_n3, eq168_e2124_d_n4, eq168_e2124_d_n5, eq168_e2124_d_n6, eq168_e2124_d_n7, eq168_e2124_d_n8, eq168_e2124_d_n9, eq168_e2124_d_n10, eq168_e2124_d_n11, eq168_e2124_d_n12, eq168_e2124_d_n13, eq168_e2124_d_n14, eq168_e2124_d_n15, eq168_e2124_d_n16, eq168_e2124_d_n17, eq168_e2124_d_n18, eq168_e2124_d_n19, eq168_e2124_d_n20, eq168_e2124_d_n21, eq168_e2124_d_n22, eq168_e2124_q, eq168_e2124_q_d_n0, eq168_e2124_q_d_n1, eq168_e2124_q_d_n2, eq168_e2124_q_d_n3, eq168_e2124_q_d_n4, eq168_e2124_q_d_n5, eq168_e2124_q_d_n6, eq168_e2124_q_d_n7, eq168_e2124_q_d_n8, eq168_e2124_q_d_n9, eq168_e2124_q_d_n10, eq168_e2124_q_d_n11, eq168_e2124_q_d_n12, eq168_e2124_q_d_n13, eq168_e2124_q_d_n14, eq168_e2124_q_d_n15, eq168_e2124_q_d_n16, eq168_e2124_q_d_n17, eq168_e2124_q_d_n18, eq168_e2124_q_d_n19, eq168_e2124_q_d_n20, eq168_e2124_q_d_n21, eq168_e2124_q_d_n22,) = {
    if ((s.v[590] != 0.0) && (s.v[591] != 0.0)) {
        let eq168_e2121_q: f64 = s.v[277];
        let eq168_e2122: f64 = (p.p7 * s.v[277]);
        let eq168_e2122_d_n0: f64 = (p.p7 * s.dn[277][0]);
        let eq168_e2122_d_n1: f64 = (p.p7 * s.dn[277][1]);
        let eq168_e2122_d_n2: f64 = (p.p7 * s.dn[277][2]);
        let eq168_e2122_d_n3: f64 = (p.p7 * s.dn[277][3]);
        let eq168_e2122_d_n4: f64 = (p.p7 * s.dn[277][4]);
        let eq168_e2122_d_n5: f64 = (p.p7 * s.dn[277][5]);
        let eq168_e2122_d_n6: f64 = (p.p7 * s.dn[277][6]);
        let eq168_e2122_d_n7: f64 = (p.p7 * s.dn[277][7]);
        let eq168_e2122_d_n8: f64 = (p.p7 * s.dn[277][8]);
        let eq168_e2122_d_n9: f64 = (p.p7 * s.dn[277][9]);
        let eq168_e2122_d_n10: f64 = (p.p7 * s.dn[277][10]);
        let eq168_e2122_d_n11: f64 = (p.p7 * s.dn[277][11]);
        let eq168_e2122_d_n12: f64 = (p.p7 * s.dn[277][12]);
        let eq168_e2122_d_n13: f64 = (p.p7 * s.dn[277][13]);
        let eq168_e2122_d_n14: f64 = (p.p7 * s.dn[277][14]);
        let eq168_e2122_d_n15: f64 = (p.p7 * s.dn[277][15]);
        let eq168_e2122_d_n16: f64 = (p.p7 * s.dn[277][16]);
        let eq168_e2122_d_n17: f64 = (p.p7 * s.dn[277][17]);
        let eq168_e2122_d_n18: f64 = (p.p7 * s.dn[277][18]);
        let eq168_e2122_d_n19: f64 = (p.p7 * s.dn[277][19]);
        let eq168_e2122_d_n20: f64 = (p.p7 * s.dn[277][20]);
        let eq168_e2122_d_n21: f64 = (p.p7 * s.dn[277][21]);
        let eq168_e2122_d_n22: f64 = (p.p7 * s.dn[277][22]);
        let eq168_e2122_q: f64 = (p.p7 * eq168_e2121_q);
        let eq168_e2122_q_d_n0: f64 = (p.p7 * s.dn[277][0]);
        let eq168_e2122_q_d_n1: f64 = (p.p7 * s.dn[277][1]);
        let eq168_e2122_q_d_n2: f64 = (p.p7 * s.dn[277][2]);
        let eq168_e2122_q_d_n3: f64 = (p.p7 * s.dn[277][3]);
        let eq168_e2122_q_d_n4: f64 = (p.p7 * s.dn[277][4]);
        let eq168_e2122_q_d_n5: f64 = (p.p7 * s.dn[277][5]);
        let eq168_e2122_q_d_n6: f64 = (p.p7 * s.dn[277][6]);
        let eq168_e2122_q_d_n7: f64 = (p.p7 * s.dn[277][7]);
        let eq168_e2122_q_d_n8: f64 = (p.p7 * s.dn[277][8]);
        let eq168_e2122_q_d_n9: f64 = (p.p7 * s.dn[277][9]);
        let eq168_e2122_q_d_n10: f64 = (p.p7 * s.dn[277][10]);
        let eq168_e2122_q_d_n11: f64 = (p.p7 * s.dn[277][11]);
        let eq168_e2122_q_d_n12: f64 = (p.p7 * s.dn[277][12]);
        let eq168_e2122_q_d_n13: f64 = (p.p7 * s.dn[277][13]);
        let eq168_e2122_q_d_n14: f64 = (p.p7 * s.dn[277][14]);
        let eq168_e2122_q_d_n15: f64 = (p.p7 * s.dn[277][15]);
        let eq168_e2122_q_d_n16: f64 = (p.p7 * s.dn[277][16]);
        let eq168_e2122_q_d_n17: f64 = (p.p7 * s.dn[277][17]);
        let eq168_e2122_q_d_n18: f64 = (p.p7 * s.dn[277][18]);
        let eq168_e2122_q_d_n19: f64 = (p.p7 * s.dn[277][19]);
        let eq168_e2122_q_d_n20: f64 = (p.p7 * s.dn[277][20]);
        let eq168_e2122_q_d_n21: f64 = (p.p7 * s.dn[277][21]);
        let eq168_e2122_q_d_n22: f64 = (p.p7 * s.dn[277][22]);
        (eq168_e2122, eq168_e2122_d_n0, eq168_e2122_d_n1, eq168_e2122_d_n2, eq168_e2122_d_n3, eq168_e2122_d_n4, eq168_e2122_d_n5, eq168_e2122_d_n6, eq168_e2122_d_n7, eq168_e2122_d_n8, eq168_e2122_d_n9, eq168_e2122_d_n10, eq168_e2122_d_n11, eq168_e2122_d_n12, eq168_e2122_d_n13, eq168_e2122_d_n14, eq168_e2122_d_n15, eq168_e2122_d_n16, eq168_e2122_d_n17, eq168_e2122_d_n18, eq168_e2122_d_n19, eq168_e2122_d_n20, eq168_e2122_d_n21, eq168_e2122_d_n22, eq168_e2122_q, eq168_e2122_q_d_n0, eq168_e2122_q_d_n1, eq168_e2122_q_d_n2, eq168_e2122_q_d_n3, eq168_e2122_q_d_n4, eq168_e2122_q_d_n5, eq168_e2122_q_d_n6, eq168_e2122_q_d_n7, eq168_e2122_q_d_n8, eq168_e2122_q_d_n9, eq168_e2122_q_d_n10, eq168_e2122_q_d_n11, eq168_e2122_q_d_n12, eq168_e2122_q_d_n13, eq168_e2122_q_d_n14, eq168_e2122_q_d_n15, eq168_e2122_q_d_n16, eq168_e2122_q_d_n17, eq168_e2122_q_d_n18, eq168_e2122_q_d_n19, eq168_e2122_q_d_n20, eq168_e2122_q_d_n21, eq168_e2122_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq168_reactive_node_derivatives: [f64; 23] = [eq168_e2124_q_d_n0, eq168_e2124_q_d_n1, eq168_e2124_q_d_n2, eq168_e2124_q_d_n3, eq168_e2124_q_d_n4, eq168_e2124_q_d_n5, eq168_e2124_q_d_n6, eq168_e2124_q_d_n7, eq168_e2124_q_d_n8, eq168_e2124_q_d_n9, eq168_e2124_q_d_n10, eq168_e2124_q_d_n11, eq168_e2124_q_d_n12, eq168_e2124_q_d_n13, eq168_e2124_q_d_n14, eq168_e2124_q_d_n15, eq168_e2124_q_d_n16, eq168_e2124_q_d_n17, eq168_e2124_q_d_n18, eq168_e2124_q_d_n19, eq168_e2124_q_d_n20, eq168_e2124_q_d_n21, eq168_e2124_q_d_n22];
        let eq168_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[17]),
            Some(nodes[16]),
            &nodes,
            &eq168_reactive_node_derivatives,
            &branches,
            &eq168_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_169_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq169_e2135, eq169_e2135_d_n0, eq169_e2135_d_n1, eq169_e2135_d_n2, eq169_e2135_d_n3, eq169_e2135_d_n4, eq169_e2135_d_n5, eq169_e2135_d_n6, eq169_e2135_d_n7, eq169_e2135_d_n8, eq169_e2135_d_n9, eq169_e2135_d_n10, eq169_e2135_d_n11, eq169_e2135_d_n12, eq169_e2135_d_n13, eq169_e2135_d_n14, eq169_e2135_d_n15, eq169_e2135_d_n16, eq169_e2135_d_n17, eq169_e2135_d_n18, eq169_e2135_d_n19, eq169_e2135_d_n20, eq169_e2135_d_n21, eq169_e2135_d_n22, eq169_e2135_q, eq169_e2135_q_d_n0, eq169_e2135_q_d_n1, eq169_e2135_q_d_n2, eq169_e2135_q_d_n3, eq169_e2135_q_d_n4, eq169_e2135_q_d_n5, eq169_e2135_q_d_n6, eq169_e2135_q_d_n7, eq169_e2135_q_d_n8, eq169_e2135_q_d_n9, eq169_e2135_q_d_n10, eq169_e2135_q_d_n11, eq169_e2135_q_d_n12, eq169_e2135_q_d_n13, eq169_e2135_q_d_n14, eq169_e2135_q_d_n15, eq169_e2135_q_d_n16, eq169_e2135_q_d_n17, eq169_e2135_q_d_n18, eq169_e2135_q_d_n19, eq169_e2135_q_d_n20, eq169_e2135_q_d_n21, eq169_e2135_q_d_n22,) = {
    if (((s.v[590] != 0.0) && (s.v[591] != 0.0)) && (s.v[592] != 0.0)) {
        let eq169_e2132_q: f64 = s.v[276];
        let eq169_e2133: f64 = (p.p7 * s.v[276]);
        let eq169_e2133_d_n0: f64 = (p.p7 * s.dn[276][0]);
        let eq169_e2133_d_n1: f64 = (p.p7 * s.dn[276][1]);
        let eq169_e2133_d_n2: f64 = (p.p7 * s.dn[276][2]);
        let eq169_e2133_d_n3: f64 = (p.p7 * s.dn[276][3]);
        let eq169_e2133_d_n4: f64 = (p.p7 * s.dn[276][4]);
        let eq169_e2133_d_n5: f64 = (p.p7 * s.dn[276][5]);
        let eq169_e2133_d_n6: f64 = (p.p7 * s.dn[276][6]);
        let eq169_e2133_d_n7: f64 = (p.p7 * s.dn[276][7]);
        let eq169_e2133_d_n8: f64 = (p.p7 * s.dn[276][8]);
        let eq169_e2133_d_n9: f64 = (p.p7 * s.dn[276][9]);
        let eq169_e2133_d_n10: f64 = (p.p7 * s.dn[276][10]);
        let eq169_e2133_d_n11: f64 = (p.p7 * s.dn[276][11]);
        let eq169_e2133_d_n12: f64 = (p.p7 * s.dn[276][12]);
        let eq169_e2133_d_n13: f64 = (p.p7 * s.dn[276][13]);
        let eq169_e2133_d_n14: f64 = (p.p7 * s.dn[276][14]);
        let eq169_e2133_d_n15: f64 = (p.p7 * s.dn[276][15]);
        let eq169_e2133_d_n16: f64 = (p.p7 * s.dn[276][16]);
        let eq169_e2133_d_n17: f64 = (p.p7 * s.dn[276][17]);
        let eq169_e2133_d_n18: f64 = (p.p7 * s.dn[276][18]);
        let eq169_e2133_d_n19: f64 = (p.p7 * s.dn[276][19]);
        let eq169_e2133_d_n20: f64 = (p.p7 * s.dn[276][20]);
        let eq169_e2133_d_n21: f64 = (p.p7 * s.dn[276][21]);
        let eq169_e2133_d_n22: f64 = (p.p7 * s.dn[276][22]);
        let eq169_e2133_q: f64 = (p.p7 * eq169_e2132_q);
        let eq169_e2133_q_d_n0: f64 = (p.p7 * s.dn[276][0]);
        let eq169_e2133_q_d_n1: f64 = (p.p7 * s.dn[276][1]);
        let eq169_e2133_q_d_n2: f64 = (p.p7 * s.dn[276][2]);
        let eq169_e2133_q_d_n3: f64 = (p.p7 * s.dn[276][3]);
        let eq169_e2133_q_d_n4: f64 = (p.p7 * s.dn[276][4]);
        let eq169_e2133_q_d_n5: f64 = (p.p7 * s.dn[276][5]);
        let eq169_e2133_q_d_n6: f64 = (p.p7 * s.dn[276][6]);
        let eq169_e2133_q_d_n7: f64 = (p.p7 * s.dn[276][7]);
        let eq169_e2133_q_d_n8: f64 = (p.p7 * s.dn[276][8]);
        let eq169_e2133_q_d_n9: f64 = (p.p7 * s.dn[276][9]);
        let eq169_e2133_q_d_n10: f64 = (p.p7 * s.dn[276][10]);
        let eq169_e2133_q_d_n11: f64 = (p.p7 * s.dn[276][11]);
        let eq169_e2133_q_d_n12: f64 = (p.p7 * s.dn[276][12]);
        let eq169_e2133_q_d_n13: f64 = (p.p7 * s.dn[276][13]);
        let eq169_e2133_q_d_n14: f64 = (p.p7 * s.dn[276][14]);
        let eq169_e2133_q_d_n15: f64 = (p.p7 * s.dn[276][15]);
        let eq169_e2133_q_d_n16: f64 = (p.p7 * s.dn[276][16]);
        let eq169_e2133_q_d_n17: f64 = (p.p7 * s.dn[276][17]);
        let eq169_e2133_q_d_n18: f64 = (p.p7 * s.dn[276][18]);
        let eq169_e2133_q_d_n19: f64 = (p.p7 * s.dn[276][19]);
        let eq169_e2133_q_d_n20: f64 = (p.p7 * s.dn[276][20]);
        let eq169_e2133_q_d_n21: f64 = (p.p7 * s.dn[276][21]);
        let eq169_e2133_q_d_n22: f64 = (p.p7 * s.dn[276][22]);
        (eq169_e2133, eq169_e2133_d_n0, eq169_e2133_d_n1, eq169_e2133_d_n2, eq169_e2133_d_n3, eq169_e2133_d_n4, eq169_e2133_d_n5, eq169_e2133_d_n6, eq169_e2133_d_n7, eq169_e2133_d_n8, eq169_e2133_d_n9, eq169_e2133_d_n10, eq169_e2133_d_n11, eq169_e2133_d_n12, eq169_e2133_d_n13, eq169_e2133_d_n14, eq169_e2133_d_n15, eq169_e2133_d_n16, eq169_e2133_d_n17, eq169_e2133_d_n18, eq169_e2133_d_n19, eq169_e2133_d_n20, eq169_e2133_d_n21, eq169_e2133_d_n22, eq169_e2133_q, eq169_e2133_q_d_n0, eq169_e2133_q_d_n1, eq169_e2133_q_d_n2, eq169_e2133_q_d_n3, eq169_e2133_q_d_n4, eq169_e2133_q_d_n5, eq169_e2133_q_d_n6, eq169_e2133_q_d_n7, eq169_e2133_q_d_n8, eq169_e2133_q_d_n9, eq169_e2133_q_d_n10, eq169_e2133_q_d_n11, eq169_e2133_q_d_n12, eq169_e2133_q_d_n13, eq169_e2133_q_d_n14, eq169_e2133_q_d_n15, eq169_e2133_q_d_n16, eq169_e2133_q_d_n17, eq169_e2133_q_d_n18, eq169_e2133_q_d_n19, eq169_e2133_q_d_n20, eq169_e2133_q_d_n21, eq169_e2133_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq169_reactive_node_derivatives: [f64; 23] = [eq169_e2135_q_d_n0, eq169_e2135_q_d_n1, eq169_e2135_q_d_n2, eq169_e2135_q_d_n3, eq169_e2135_q_d_n4, eq169_e2135_q_d_n5, eq169_e2135_q_d_n6, eq169_e2135_q_d_n7, eq169_e2135_q_d_n8, eq169_e2135_q_d_n9, eq169_e2135_q_d_n10, eq169_e2135_q_d_n11, eq169_e2135_q_d_n12, eq169_e2135_q_d_n13, eq169_e2135_q_d_n14, eq169_e2135_q_d_n15, eq169_e2135_q_d_n16, eq169_e2135_q_d_n17, eq169_e2135_q_d_n18, eq169_e2135_q_d_n19, eq169_e2135_q_d_n20, eq169_e2135_q_d_n21, eq169_e2135_q_d_n22];
        let eq169_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[16]),
            &nodes,
            &eq169_reactive_node_derivatives,
            &branches,
            &eq169_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_170_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq170_e2148, eq170_e2148_d_n0, eq170_e2148_d_n1, eq170_e2148_d_n2, eq170_e2148_d_n3, eq170_e2148_d_n4, eq170_e2148_d_n5, eq170_e2148_d_n6, eq170_e2148_d_n7, eq170_e2148_d_n8, eq170_e2148_d_n9, eq170_e2148_d_n10, eq170_e2148_d_n11, eq170_e2148_d_n12, eq170_e2148_d_n13, eq170_e2148_d_n14, eq170_e2148_d_n15, eq170_e2148_d_n16, eq170_e2148_d_n17, eq170_e2148_d_n18, eq170_e2148_d_n19, eq170_e2148_d_n20, eq170_e2148_d_n21, eq170_e2148_d_n22, eq170_e2148_q, eq170_e2148_q_d_n0, eq170_e2148_q_d_n1, eq170_e2148_q_d_n2, eq170_e2148_q_d_n3, eq170_e2148_q_d_n4, eq170_e2148_q_d_n5, eq170_e2148_q_d_n6, eq170_e2148_q_d_n7, eq170_e2148_q_d_n8, eq170_e2148_q_d_n9, eq170_e2148_q_d_n10, eq170_e2148_q_d_n11, eq170_e2148_q_d_n12, eq170_e2148_q_d_n13, eq170_e2148_q_d_n14, eq170_e2148_q_d_n15, eq170_e2148_q_d_n16, eq170_e2148_q_d_n17, eq170_e2148_q_d_n18, eq170_e2148_q_d_n19, eq170_e2148_q_d_n20, eq170_e2148_q_d_n21, eq170_e2148_q_d_n22,) = {
    if (((s.v[590] != 0.0) && (s.v[591] != 0.0)) && (s.v[592] != 0.0)) {
        let eq170_e2143_q: f64 = s.v[276];
        let eq170_e2144: f64 = (p.p7 * s.v[276]);
        let eq170_e2144_d_n0: f64 = (p.p7 * s.dn[276][0]);
        let eq170_e2144_d_n1: f64 = (p.p7 * s.dn[276][1]);
        let eq170_e2144_d_n2: f64 = (p.p7 * s.dn[276][2]);
        let eq170_e2144_d_n3: f64 = (p.p7 * s.dn[276][3]);
        let eq170_e2144_d_n4: f64 = (p.p7 * s.dn[276][4]);
        let eq170_e2144_d_n5: f64 = (p.p7 * s.dn[276][5]);
        let eq170_e2144_d_n6: f64 = (p.p7 * s.dn[276][6]);
        let eq170_e2144_d_n7: f64 = (p.p7 * s.dn[276][7]);
        let eq170_e2144_d_n8: f64 = (p.p7 * s.dn[276][8]);
        let eq170_e2144_d_n9: f64 = (p.p7 * s.dn[276][9]);
        let eq170_e2144_d_n10: f64 = (p.p7 * s.dn[276][10]);
        let eq170_e2144_d_n11: f64 = (p.p7 * s.dn[276][11]);
        let eq170_e2144_d_n12: f64 = (p.p7 * s.dn[276][12]);
        let eq170_e2144_d_n13: f64 = (p.p7 * s.dn[276][13]);
        let eq170_e2144_d_n14: f64 = (p.p7 * s.dn[276][14]);
        let eq170_e2144_d_n15: f64 = (p.p7 * s.dn[276][15]);
        let eq170_e2144_d_n16: f64 = (p.p7 * s.dn[276][16]);
        let eq170_e2144_d_n17: f64 = (p.p7 * s.dn[276][17]);
        let eq170_e2144_d_n18: f64 = (p.p7 * s.dn[276][18]);
        let eq170_e2144_d_n19: f64 = (p.p7 * s.dn[276][19]);
        let eq170_e2144_d_n20: f64 = (p.p7 * s.dn[276][20]);
        let eq170_e2144_d_n21: f64 = (p.p7 * s.dn[276][21]);
        let eq170_e2144_d_n22: f64 = (p.p7 * s.dn[276][22]);
        let eq170_e2144_q: f64 = (p.p7 * eq170_e2143_q);
        let eq170_e2144_q_d_n0: f64 = (p.p7 * s.dn[276][0]);
        let eq170_e2144_q_d_n1: f64 = (p.p7 * s.dn[276][1]);
        let eq170_e2144_q_d_n2: f64 = (p.p7 * s.dn[276][2]);
        let eq170_e2144_q_d_n3: f64 = (p.p7 * s.dn[276][3]);
        let eq170_e2144_q_d_n4: f64 = (p.p7 * s.dn[276][4]);
        let eq170_e2144_q_d_n5: f64 = (p.p7 * s.dn[276][5]);
        let eq170_e2144_q_d_n6: f64 = (p.p7 * s.dn[276][6]);
        let eq170_e2144_q_d_n7: f64 = (p.p7 * s.dn[276][7]);
        let eq170_e2144_q_d_n8: f64 = (p.p7 * s.dn[276][8]);
        let eq170_e2144_q_d_n9: f64 = (p.p7 * s.dn[276][9]);
        let eq170_e2144_q_d_n10: f64 = (p.p7 * s.dn[276][10]);
        let eq170_e2144_q_d_n11: f64 = (p.p7 * s.dn[276][11]);
        let eq170_e2144_q_d_n12: f64 = (p.p7 * s.dn[276][12]);
        let eq170_e2144_q_d_n13: f64 = (p.p7 * s.dn[276][13]);
        let eq170_e2144_q_d_n14: f64 = (p.p7 * s.dn[276][14]);
        let eq170_e2144_q_d_n15: f64 = (p.p7 * s.dn[276][15]);
        let eq170_e2144_q_d_n16: f64 = (p.p7 * s.dn[276][16]);
        let eq170_e2144_q_d_n17: f64 = (p.p7 * s.dn[276][17]);
        let eq170_e2144_q_d_n18: f64 = (p.p7 * s.dn[276][18]);
        let eq170_e2144_q_d_n19: f64 = (p.p7 * s.dn[276][19]);
        let eq170_e2144_q_d_n20: f64 = (p.p7 * s.dn[276][20]);
        let eq170_e2144_q_d_n21: f64 = (p.p7 * s.dn[276][21]);
        let eq170_e2144_q_d_n22: f64 = (p.p7 * s.dn[276][22]);
        let eq170_e2146: f64 = (eq170_e2144 * p.p248);
        let eq170_e2146_d_n0: f64 = (eq170_e2144_d_n0 * p.p248);
        let eq170_e2146_d_n1: f64 = (eq170_e2144_d_n1 * p.p248);
        let eq170_e2146_d_n2: f64 = (eq170_e2144_d_n2 * p.p248);
        let eq170_e2146_d_n3: f64 = (eq170_e2144_d_n3 * p.p248);
        let eq170_e2146_d_n4: f64 = (eq170_e2144_d_n4 * p.p248);
        let eq170_e2146_d_n5: f64 = (eq170_e2144_d_n5 * p.p248);
        let eq170_e2146_d_n6: f64 = (eq170_e2144_d_n6 * p.p248);
        let eq170_e2146_d_n7: f64 = (eq170_e2144_d_n7 * p.p248);
        let eq170_e2146_d_n8: f64 = (eq170_e2144_d_n8 * p.p248);
        let eq170_e2146_d_n9: f64 = (eq170_e2144_d_n9 * p.p248);
        let eq170_e2146_d_n10: f64 = (eq170_e2144_d_n10 * p.p248);
        let eq170_e2146_d_n11: f64 = (eq170_e2144_d_n11 * p.p248);
        let eq170_e2146_d_n12: f64 = (eq170_e2144_d_n12 * p.p248);
        let eq170_e2146_d_n13: f64 = (eq170_e2144_d_n13 * p.p248);
        let eq170_e2146_d_n14: f64 = (eq170_e2144_d_n14 * p.p248);
        let eq170_e2146_d_n15: f64 = (eq170_e2144_d_n15 * p.p248);
        let eq170_e2146_d_n16: f64 = (eq170_e2144_d_n16 * p.p248);
        let eq170_e2146_d_n17: f64 = (eq170_e2144_d_n17 * p.p248);
        let eq170_e2146_d_n18: f64 = (eq170_e2144_d_n18 * p.p248);
        let eq170_e2146_d_n19: f64 = (eq170_e2144_d_n19 * p.p248);
        let eq170_e2146_d_n20: f64 = (eq170_e2144_d_n20 * p.p248);
        let eq170_e2146_d_n21: f64 = (eq170_e2144_d_n21 * p.p248);
        let eq170_e2146_d_n22: f64 = (eq170_e2144_d_n22 * p.p248);
        let eq170_e2146_q: f64 = (eq170_e2144_q * p.p248);
        let eq170_e2146_q_d_n0: f64 = (eq170_e2144_q_d_n0 * p.p248);
        let eq170_e2146_q_d_n1: f64 = (eq170_e2144_q_d_n1 * p.p248);
        let eq170_e2146_q_d_n2: f64 = (eq170_e2144_q_d_n2 * p.p248);
        let eq170_e2146_q_d_n3: f64 = (eq170_e2144_q_d_n3 * p.p248);
        let eq170_e2146_q_d_n4: f64 = (eq170_e2144_q_d_n4 * p.p248);
        let eq170_e2146_q_d_n5: f64 = (eq170_e2144_q_d_n5 * p.p248);
        let eq170_e2146_q_d_n6: f64 = (eq170_e2144_q_d_n6 * p.p248);
        let eq170_e2146_q_d_n7: f64 = (eq170_e2144_q_d_n7 * p.p248);
        let eq170_e2146_q_d_n8: f64 = (eq170_e2144_q_d_n8 * p.p248);
        let eq170_e2146_q_d_n9: f64 = (eq170_e2144_q_d_n9 * p.p248);
        let eq170_e2146_q_d_n10: f64 = (eq170_e2144_q_d_n10 * p.p248);
        let eq170_e2146_q_d_n11: f64 = (eq170_e2144_q_d_n11 * p.p248);
        let eq170_e2146_q_d_n12: f64 = (eq170_e2144_q_d_n12 * p.p248);
        let eq170_e2146_q_d_n13: f64 = (eq170_e2144_q_d_n13 * p.p248);
        let eq170_e2146_q_d_n14: f64 = (eq170_e2144_q_d_n14 * p.p248);
        let eq170_e2146_q_d_n15: f64 = (eq170_e2144_q_d_n15 * p.p248);
        let eq170_e2146_q_d_n16: f64 = (eq170_e2144_q_d_n16 * p.p248);
        let eq170_e2146_q_d_n17: f64 = (eq170_e2144_q_d_n17 * p.p248);
        let eq170_e2146_q_d_n18: f64 = (eq170_e2144_q_d_n18 * p.p248);
        let eq170_e2146_q_d_n19: f64 = (eq170_e2144_q_d_n19 * p.p248);
        let eq170_e2146_q_d_n20: f64 = (eq170_e2144_q_d_n20 * p.p248);
        let eq170_e2146_q_d_n21: f64 = (eq170_e2144_q_d_n21 * p.p248);
        let eq170_e2146_q_d_n22: f64 = (eq170_e2144_q_d_n22 * p.p248);
        (eq170_e2146, eq170_e2146_d_n0, eq170_e2146_d_n1, eq170_e2146_d_n2, eq170_e2146_d_n3, eq170_e2146_d_n4, eq170_e2146_d_n5, eq170_e2146_d_n6, eq170_e2146_d_n7, eq170_e2146_d_n8, eq170_e2146_d_n9, eq170_e2146_d_n10, eq170_e2146_d_n11, eq170_e2146_d_n12, eq170_e2146_d_n13, eq170_e2146_d_n14, eq170_e2146_d_n15, eq170_e2146_d_n16, eq170_e2146_d_n17, eq170_e2146_d_n18, eq170_e2146_d_n19, eq170_e2146_d_n20, eq170_e2146_d_n21, eq170_e2146_d_n22, eq170_e2146_q, eq170_e2146_q_d_n0, eq170_e2146_q_d_n1, eq170_e2146_q_d_n2, eq170_e2146_q_d_n3, eq170_e2146_q_d_n4, eq170_e2146_q_d_n5, eq170_e2146_q_d_n6, eq170_e2146_q_d_n7, eq170_e2146_q_d_n8, eq170_e2146_q_d_n9, eq170_e2146_q_d_n10, eq170_e2146_q_d_n11, eq170_e2146_q_d_n12, eq170_e2146_q_d_n13, eq170_e2146_q_d_n14, eq170_e2146_q_d_n15, eq170_e2146_q_d_n16, eq170_e2146_q_d_n17, eq170_e2146_q_d_n18, eq170_e2146_q_d_n19, eq170_e2146_q_d_n20, eq170_e2146_q_d_n21, eq170_e2146_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq170_reactive_node_derivatives: [f64; 23] = [eq170_e2148_q_d_n0, eq170_e2148_q_d_n1, eq170_e2148_q_d_n2, eq170_e2148_q_d_n3, eq170_e2148_q_d_n4, eq170_e2148_q_d_n5, eq170_e2148_q_d_n6, eq170_e2148_q_d_n7, eq170_e2148_q_d_n8, eq170_e2148_q_d_n9, eq170_e2148_q_d_n10, eq170_e2148_q_d_n11, eq170_e2148_q_d_n12, eq170_e2148_q_d_n13, eq170_e2148_q_d_n14, eq170_e2148_q_d_n15, eq170_e2148_q_d_n16, eq170_e2148_q_d_n17, eq170_e2148_q_d_n18, eq170_e2148_q_d_n19, eq170_e2148_q_d_n20, eq170_e2148_q_d_n21, eq170_e2148_q_d_n22];
        let eq170_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[16]),
            &nodes,
            &eq170_reactive_node_derivatives,
            &branches,
            &eq170_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
