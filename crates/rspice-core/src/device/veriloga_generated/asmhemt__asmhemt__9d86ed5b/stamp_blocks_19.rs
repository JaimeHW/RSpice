#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_123_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq123_e1576, eq123_e1576_d_n0, eq123_e1576_d_n1, eq123_e1576_d_n2, eq123_e1576_d_n3, eq123_e1576_d_n4, eq123_e1576_d_n5, eq123_e1576_d_n6, eq123_e1576_d_n7, eq123_e1576_d_n8, eq123_e1576_d_n9, eq123_e1576_d_n10, eq123_e1576_d_n11, eq123_e1576_d_n12, eq123_e1576_d_n13, eq123_e1576_d_n14, eq123_e1576_d_n15, eq123_e1576_d_n16, eq123_e1576_d_n17, eq123_e1576_d_n18, eq123_e1576_d_n19, eq123_e1576_d_n20, eq123_e1576_d_n21, eq123_e1576_d_n22, eq123_e1576_q, eq123_e1576_q_d_n0, eq123_e1576_q_d_n1, eq123_e1576_q_d_n2, eq123_e1576_q_d_n3, eq123_e1576_q_d_n4, eq123_e1576_q_d_n5, eq123_e1576_q_d_n6, eq123_e1576_q_d_n7, eq123_e1576_q_d_n8, eq123_e1576_q_d_n9, eq123_e1576_q_d_n10, eq123_e1576_q_d_n11, eq123_e1576_q_d_n12, eq123_e1576_q_d_n13, eq123_e1576_q_d_n14, eq123_e1576_q_d_n15, eq123_e1576_q_d_n16, eq123_e1576_q_d_n17, eq123_e1576_q_d_n18, eq123_e1576_q_d_n19, eq123_e1576_q_d_n20, eq123_e1576_q_d_n21, eq123_e1576_q_d_n22,) = {
    if (((s.v[570] != 0.0) && (s.v[571] != 0.0)) && (!(s.v[572] != 0.0))) {
        let eq123_e1573_q: f64 = s.v[228];
        let eq123_e1574: f64 = (p.p7 * s.v[228]);
        let eq123_e1574_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq123_e1574_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq123_e1574_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq123_e1574_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq123_e1574_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq123_e1574_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq123_e1574_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq123_e1574_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq123_e1574_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq123_e1574_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq123_e1574_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq123_e1574_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq123_e1574_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq123_e1574_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq123_e1574_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq123_e1574_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq123_e1574_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq123_e1574_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq123_e1574_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq123_e1574_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq123_e1574_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq123_e1574_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq123_e1574_d_n22: f64 = (p.p7 * s.dn[228][22]);
        let eq123_e1574_q: f64 = (p.p7 * eq123_e1573_q);
        let eq123_e1574_q_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq123_e1574_q_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq123_e1574_q_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq123_e1574_q_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq123_e1574_q_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq123_e1574_q_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq123_e1574_q_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq123_e1574_q_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq123_e1574_q_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq123_e1574_q_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq123_e1574_q_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq123_e1574_q_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq123_e1574_q_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq123_e1574_q_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq123_e1574_q_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq123_e1574_q_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq123_e1574_q_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq123_e1574_q_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq123_e1574_q_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq123_e1574_q_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq123_e1574_q_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq123_e1574_q_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq123_e1574_q_d_n22: f64 = (p.p7 * s.dn[228][22]);
        (eq123_e1574, eq123_e1574_d_n0, eq123_e1574_d_n1, eq123_e1574_d_n2, eq123_e1574_d_n3, eq123_e1574_d_n4, eq123_e1574_d_n5, eq123_e1574_d_n6, eq123_e1574_d_n7, eq123_e1574_d_n8, eq123_e1574_d_n9, eq123_e1574_d_n10, eq123_e1574_d_n11, eq123_e1574_d_n12, eq123_e1574_d_n13, eq123_e1574_d_n14, eq123_e1574_d_n15, eq123_e1574_d_n16, eq123_e1574_d_n17, eq123_e1574_d_n18, eq123_e1574_d_n19, eq123_e1574_d_n20, eq123_e1574_d_n21, eq123_e1574_d_n22, eq123_e1574_q, eq123_e1574_q_d_n0, eq123_e1574_q_d_n1, eq123_e1574_q_d_n2, eq123_e1574_q_d_n3, eq123_e1574_q_d_n4, eq123_e1574_q_d_n5, eq123_e1574_q_d_n6, eq123_e1574_q_d_n7, eq123_e1574_q_d_n8, eq123_e1574_q_d_n9, eq123_e1574_q_d_n10, eq123_e1574_q_d_n11, eq123_e1574_q_d_n12, eq123_e1574_q_d_n13, eq123_e1574_q_d_n14, eq123_e1574_q_d_n15, eq123_e1574_q_d_n16, eq123_e1574_q_d_n17, eq123_e1574_q_d_n18, eq123_e1574_q_d_n19, eq123_e1574_q_d_n20, eq123_e1574_q_d_n21, eq123_e1574_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq123_reactive_node_derivatives: [f64; 23] = [eq123_e1576_q_d_n0, eq123_e1576_q_d_n1, eq123_e1576_q_d_n2, eq123_e1576_q_d_n3, eq123_e1576_q_d_n4, eq123_e1576_q_d_n5, eq123_e1576_q_d_n6, eq123_e1576_q_d_n7, eq123_e1576_q_d_n8, eq123_e1576_q_d_n9, eq123_e1576_q_d_n10, eq123_e1576_q_d_n11, eq123_e1576_q_d_n12, eq123_e1576_q_d_n13, eq123_e1576_q_d_n14, eq123_e1576_q_d_n15, eq123_e1576_q_d_n16, eq123_e1576_q_d_n17, eq123_e1576_q_d_n18, eq123_e1576_q_d_n19, eq123_e1576_q_d_n20, eq123_e1576_q_d_n21, eq123_e1576_q_d_n22];
        let eq123_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            &nodes,
            &eq123_reactive_node_derivatives,
            &branches,
            &eq123_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_124_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq124_e1590, eq124_e1590_d_n0, eq124_e1590_d_n1, eq124_e1590_d_n2, eq124_e1590_d_n3, eq124_e1590_d_n4, eq124_e1590_d_n5, eq124_e1590_d_n6, eq124_e1590_d_n7, eq124_e1590_d_n8, eq124_e1590_d_n9, eq124_e1590_d_n10, eq124_e1590_d_n11, eq124_e1590_d_n12, eq124_e1590_d_n13, eq124_e1590_d_n14, eq124_e1590_d_n15, eq124_e1590_d_n16, eq124_e1590_d_n17, eq124_e1590_d_n18, eq124_e1590_d_n19, eq124_e1590_d_n20, eq124_e1590_d_n21, eq124_e1590_d_n22, eq124_e1590_q, eq124_e1590_q_d_n0, eq124_e1590_q_d_n1, eq124_e1590_q_d_n2, eq124_e1590_q_d_n3, eq124_e1590_q_d_n4, eq124_e1590_q_d_n5, eq124_e1590_q_d_n6, eq124_e1590_q_d_n7, eq124_e1590_q_d_n8, eq124_e1590_q_d_n9, eq124_e1590_q_d_n10, eq124_e1590_q_d_n11, eq124_e1590_q_d_n12, eq124_e1590_q_d_n13, eq124_e1590_q_d_n14, eq124_e1590_q_d_n15, eq124_e1590_q_d_n16, eq124_e1590_q_d_n17, eq124_e1590_q_d_n18, eq124_e1590_q_d_n19, eq124_e1590_q_d_n20, eq124_e1590_q_d_n21, eq124_e1590_q_d_n22,) = {
    if (((s.v[570] != 0.0) && (s.v[571] != 0.0)) && (!(s.v[572] != 0.0))) {
        let eq124_e1585_q: f64 = s.v[228];
        let eq124_e1586: f64 = (p.p7 * s.v[228]);
        let eq124_e1586_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq124_e1586_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq124_e1586_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq124_e1586_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq124_e1586_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq124_e1586_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq124_e1586_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq124_e1586_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq124_e1586_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq124_e1586_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq124_e1586_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq124_e1586_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq124_e1586_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq124_e1586_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq124_e1586_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq124_e1586_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq124_e1586_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq124_e1586_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq124_e1586_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq124_e1586_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq124_e1586_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq124_e1586_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq124_e1586_d_n22: f64 = (p.p7 * s.dn[228][22]);
        let eq124_e1586_q: f64 = (p.p7 * eq124_e1585_q);
        let eq124_e1586_q_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq124_e1586_q_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq124_e1586_q_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq124_e1586_q_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq124_e1586_q_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq124_e1586_q_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq124_e1586_q_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq124_e1586_q_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq124_e1586_q_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq124_e1586_q_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq124_e1586_q_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq124_e1586_q_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq124_e1586_q_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq124_e1586_q_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq124_e1586_q_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq124_e1586_q_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq124_e1586_q_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq124_e1586_q_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq124_e1586_q_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq124_e1586_q_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq124_e1586_q_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq124_e1586_q_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq124_e1586_q_d_n22: f64 = (p.p7 * s.dn[228][22]);
        let eq124_e1588: f64 = (eq124_e1586 * p.p246);
        let eq124_e1588_d_n0: f64 = (eq124_e1586_d_n0 * p.p246);
        let eq124_e1588_d_n1: f64 = (eq124_e1586_d_n1 * p.p246);
        let eq124_e1588_d_n2: f64 = (eq124_e1586_d_n2 * p.p246);
        let eq124_e1588_d_n3: f64 = (eq124_e1586_d_n3 * p.p246);
        let eq124_e1588_d_n4: f64 = (eq124_e1586_d_n4 * p.p246);
        let eq124_e1588_d_n5: f64 = (eq124_e1586_d_n5 * p.p246);
        let eq124_e1588_d_n6: f64 = (eq124_e1586_d_n6 * p.p246);
        let eq124_e1588_d_n7: f64 = (eq124_e1586_d_n7 * p.p246);
        let eq124_e1588_d_n8: f64 = (eq124_e1586_d_n8 * p.p246);
        let eq124_e1588_d_n9: f64 = (eq124_e1586_d_n9 * p.p246);
        let eq124_e1588_d_n10: f64 = (eq124_e1586_d_n10 * p.p246);
        let eq124_e1588_d_n11: f64 = (eq124_e1586_d_n11 * p.p246);
        let eq124_e1588_d_n12: f64 = (eq124_e1586_d_n12 * p.p246);
        let eq124_e1588_d_n13: f64 = (eq124_e1586_d_n13 * p.p246);
        let eq124_e1588_d_n14: f64 = (eq124_e1586_d_n14 * p.p246);
        let eq124_e1588_d_n15: f64 = (eq124_e1586_d_n15 * p.p246);
        let eq124_e1588_d_n16: f64 = (eq124_e1586_d_n16 * p.p246);
        let eq124_e1588_d_n17: f64 = (eq124_e1586_d_n17 * p.p246);
        let eq124_e1588_d_n18: f64 = (eq124_e1586_d_n18 * p.p246);
        let eq124_e1588_d_n19: f64 = (eq124_e1586_d_n19 * p.p246);
        let eq124_e1588_d_n20: f64 = (eq124_e1586_d_n20 * p.p246);
        let eq124_e1588_d_n21: f64 = (eq124_e1586_d_n21 * p.p246);
        let eq124_e1588_d_n22: f64 = (eq124_e1586_d_n22 * p.p246);
        let eq124_e1588_q: f64 = (eq124_e1586_q * p.p246);
        let eq124_e1588_q_d_n0: f64 = (eq124_e1586_q_d_n0 * p.p246);
        let eq124_e1588_q_d_n1: f64 = (eq124_e1586_q_d_n1 * p.p246);
        let eq124_e1588_q_d_n2: f64 = (eq124_e1586_q_d_n2 * p.p246);
        let eq124_e1588_q_d_n3: f64 = (eq124_e1586_q_d_n3 * p.p246);
        let eq124_e1588_q_d_n4: f64 = (eq124_e1586_q_d_n4 * p.p246);
        let eq124_e1588_q_d_n5: f64 = (eq124_e1586_q_d_n5 * p.p246);
        let eq124_e1588_q_d_n6: f64 = (eq124_e1586_q_d_n6 * p.p246);
        let eq124_e1588_q_d_n7: f64 = (eq124_e1586_q_d_n7 * p.p246);
        let eq124_e1588_q_d_n8: f64 = (eq124_e1586_q_d_n8 * p.p246);
        let eq124_e1588_q_d_n9: f64 = (eq124_e1586_q_d_n9 * p.p246);
        let eq124_e1588_q_d_n10: f64 = (eq124_e1586_q_d_n10 * p.p246);
        let eq124_e1588_q_d_n11: f64 = (eq124_e1586_q_d_n11 * p.p246);
        let eq124_e1588_q_d_n12: f64 = (eq124_e1586_q_d_n12 * p.p246);
        let eq124_e1588_q_d_n13: f64 = (eq124_e1586_q_d_n13 * p.p246);
        let eq124_e1588_q_d_n14: f64 = (eq124_e1586_q_d_n14 * p.p246);
        let eq124_e1588_q_d_n15: f64 = (eq124_e1586_q_d_n15 * p.p246);
        let eq124_e1588_q_d_n16: f64 = (eq124_e1586_q_d_n16 * p.p246);
        let eq124_e1588_q_d_n17: f64 = (eq124_e1586_q_d_n17 * p.p246);
        let eq124_e1588_q_d_n18: f64 = (eq124_e1586_q_d_n18 * p.p246);
        let eq124_e1588_q_d_n19: f64 = (eq124_e1586_q_d_n19 * p.p246);
        let eq124_e1588_q_d_n20: f64 = (eq124_e1586_q_d_n20 * p.p246);
        let eq124_e1588_q_d_n21: f64 = (eq124_e1586_q_d_n21 * p.p246);
        let eq124_e1588_q_d_n22: f64 = (eq124_e1586_q_d_n22 * p.p246);
        (eq124_e1588, eq124_e1588_d_n0, eq124_e1588_d_n1, eq124_e1588_d_n2, eq124_e1588_d_n3, eq124_e1588_d_n4, eq124_e1588_d_n5, eq124_e1588_d_n6, eq124_e1588_d_n7, eq124_e1588_d_n8, eq124_e1588_d_n9, eq124_e1588_d_n10, eq124_e1588_d_n11, eq124_e1588_d_n12, eq124_e1588_d_n13, eq124_e1588_d_n14, eq124_e1588_d_n15, eq124_e1588_d_n16, eq124_e1588_d_n17, eq124_e1588_d_n18, eq124_e1588_d_n19, eq124_e1588_d_n20, eq124_e1588_d_n21, eq124_e1588_d_n22, eq124_e1588_q, eq124_e1588_q_d_n0, eq124_e1588_q_d_n1, eq124_e1588_q_d_n2, eq124_e1588_q_d_n3, eq124_e1588_q_d_n4, eq124_e1588_q_d_n5, eq124_e1588_q_d_n6, eq124_e1588_q_d_n7, eq124_e1588_q_d_n8, eq124_e1588_q_d_n9, eq124_e1588_q_d_n10, eq124_e1588_q_d_n11, eq124_e1588_q_d_n12, eq124_e1588_q_d_n13, eq124_e1588_q_d_n14, eq124_e1588_q_d_n15, eq124_e1588_q_d_n16, eq124_e1588_q_d_n17, eq124_e1588_q_d_n18, eq124_e1588_q_d_n19, eq124_e1588_q_d_n20, eq124_e1588_q_d_n21, eq124_e1588_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq124_reactive_node_derivatives: [f64; 23] = [eq124_e1590_q_d_n0, eq124_e1590_q_d_n1, eq124_e1590_q_d_n2, eq124_e1590_q_d_n3, eq124_e1590_q_d_n4, eq124_e1590_q_d_n5, eq124_e1590_q_d_n6, eq124_e1590_q_d_n7, eq124_e1590_q_d_n8, eq124_e1590_q_d_n9, eq124_e1590_q_d_n10, eq124_e1590_q_d_n11, eq124_e1590_q_d_n12, eq124_e1590_q_d_n13, eq124_e1590_q_d_n14, eq124_e1590_q_d_n15, eq124_e1590_q_d_n16, eq124_e1590_q_d_n17, eq124_e1590_q_d_n18, eq124_e1590_q_d_n19, eq124_e1590_q_d_n20, eq124_e1590_q_d_n21, eq124_e1590_q_d_n22];
        let eq124_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            &nodes,
            &eq124_reactive_node_derivatives,
            &branches,
            &eq124_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_125_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq125_e1601, eq125_e1601_d_n0, eq125_e1601_d_n1, eq125_e1601_d_n2, eq125_e1601_d_n3, eq125_e1601_d_n4, eq125_e1601_d_n5, eq125_e1601_d_n6, eq125_e1601_d_n7, eq125_e1601_d_n8, eq125_e1601_d_n9, eq125_e1601_d_n10, eq125_e1601_d_n11, eq125_e1601_d_n12, eq125_e1601_d_n13, eq125_e1601_d_n14, eq125_e1601_d_n15, eq125_e1601_d_n16, eq125_e1601_d_n17, eq125_e1601_d_n18, eq125_e1601_d_n19, eq125_e1601_d_n20, eq125_e1601_d_n21, eq125_e1601_d_n22, eq125_e1601_q, eq125_e1601_q_d_n0, eq125_e1601_q_d_n1, eq125_e1601_q_d_n2, eq125_e1601_q_d_n3, eq125_e1601_q_d_n4, eq125_e1601_q_d_n5, eq125_e1601_q_d_n6, eq125_e1601_q_d_n7, eq125_e1601_q_d_n8, eq125_e1601_q_d_n9, eq125_e1601_q_d_n10, eq125_e1601_q_d_n11, eq125_e1601_q_d_n12, eq125_e1601_q_d_n13, eq125_e1601_q_d_n14, eq125_e1601_q_d_n15, eq125_e1601_q_d_n16, eq125_e1601_q_d_n17, eq125_e1601_q_d_n18, eq125_e1601_q_d_n19, eq125_e1601_q_d_n20, eq125_e1601_q_d_n21, eq125_e1601_q_d_n22,) = {
    if ((s.v[570] != 0.0) && (s.v[571] != 0.0)) {
        let eq125_e1597: f64 = (p.p251 * s.v[228]);
        let eq125_e1597_d_n0: f64 = (p.p251 * s.dn[228][0]);
        let eq125_e1597_d_n1: f64 = (p.p251 * s.dn[228][1]);
        let eq125_e1597_d_n2: f64 = (p.p251 * s.dn[228][2]);
        let eq125_e1597_d_n3: f64 = (p.p251 * s.dn[228][3]);
        let eq125_e1597_d_n4: f64 = (p.p251 * s.dn[228][4]);
        let eq125_e1597_d_n5: f64 = (p.p251 * s.dn[228][5]);
        let eq125_e1597_d_n6: f64 = (p.p251 * s.dn[228][6]);
        let eq125_e1597_d_n7: f64 = (p.p251 * s.dn[228][7]);
        let eq125_e1597_d_n8: f64 = (p.p251 * s.dn[228][8]);
        let eq125_e1597_d_n9: f64 = (p.p251 * s.dn[228][9]);
        let eq125_e1597_d_n10: f64 = (p.p251 * s.dn[228][10]);
        let eq125_e1597_d_n11: f64 = (p.p251 * s.dn[228][11]);
        let eq125_e1597_d_n12: f64 = (p.p251 * s.dn[228][12]);
        let eq125_e1597_d_n13: f64 = (p.p251 * s.dn[228][13]);
        let eq125_e1597_d_n14: f64 = (p.p251 * s.dn[228][14]);
        let eq125_e1597_d_n15: f64 = (p.p251 * s.dn[228][15]);
        let eq125_e1597_d_n16: f64 = (p.p251 * s.dn[228][16]);
        let eq125_e1597_d_n17: f64 = (p.p251 * s.dn[228][17]);
        let eq125_e1597_d_n18: f64 = (p.p251 * s.dn[228][18]);
        let eq125_e1597_d_n19: f64 = (p.p251 * s.dn[228][19]);
        let eq125_e1597_d_n20: f64 = (p.p251 * s.dn[228][20]);
        let eq125_e1597_d_n21: f64 = (p.p251 * s.dn[228][21]);
        let eq125_e1597_d_n22: f64 = (p.p251 * s.dn[228][22]);
        let eq125_e1598_q: f64 = eq125_e1597;
        let eq125_e1599: f64 = (p.p7 * eq125_e1597);
        let eq125_e1599_d_n0: f64 = (p.p7 * eq125_e1597_d_n0);
        let eq125_e1599_d_n1: f64 = (p.p7 * eq125_e1597_d_n1);
        let eq125_e1599_d_n2: f64 = (p.p7 * eq125_e1597_d_n2);
        let eq125_e1599_d_n3: f64 = (p.p7 * eq125_e1597_d_n3);
        let eq125_e1599_d_n4: f64 = (p.p7 * eq125_e1597_d_n4);
        let eq125_e1599_d_n5: f64 = (p.p7 * eq125_e1597_d_n5);
        let eq125_e1599_d_n6: f64 = (p.p7 * eq125_e1597_d_n6);
        let eq125_e1599_d_n7: f64 = (p.p7 * eq125_e1597_d_n7);
        let eq125_e1599_d_n8: f64 = (p.p7 * eq125_e1597_d_n8);
        let eq125_e1599_d_n9: f64 = (p.p7 * eq125_e1597_d_n9);
        let eq125_e1599_d_n10: f64 = (p.p7 * eq125_e1597_d_n10);
        let eq125_e1599_d_n11: f64 = (p.p7 * eq125_e1597_d_n11);
        let eq125_e1599_d_n12: f64 = (p.p7 * eq125_e1597_d_n12);
        let eq125_e1599_d_n13: f64 = (p.p7 * eq125_e1597_d_n13);
        let eq125_e1599_d_n14: f64 = (p.p7 * eq125_e1597_d_n14);
        let eq125_e1599_d_n15: f64 = (p.p7 * eq125_e1597_d_n15);
        let eq125_e1599_d_n16: f64 = (p.p7 * eq125_e1597_d_n16);
        let eq125_e1599_d_n17: f64 = (p.p7 * eq125_e1597_d_n17);
        let eq125_e1599_d_n18: f64 = (p.p7 * eq125_e1597_d_n18);
        let eq125_e1599_d_n19: f64 = (p.p7 * eq125_e1597_d_n19);
        let eq125_e1599_d_n20: f64 = (p.p7 * eq125_e1597_d_n20);
        let eq125_e1599_d_n21: f64 = (p.p7 * eq125_e1597_d_n21);
        let eq125_e1599_d_n22: f64 = (p.p7 * eq125_e1597_d_n22);
        let eq125_e1599_q: f64 = (p.p7 * eq125_e1598_q);
        let eq125_e1599_q_d_n0: f64 = (p.p7 * eq125_e1597_d_n0);
        let eq125_e1599_q_d_n1: f64 = (p.p7 * eq125_e1597_d_n1);
        let eq125_e1599_q_d_n2: f64 = (p.p7 * eq125_e1597_d_n2);
        let eq125_e1599_q_d_n3: f64 = (p.p7 * eq125_e1597_d_n3);
        let eq125_e1599_q_d_n4: f64 = (p.p7 * eq125_e1597_d_n4);
        let eq125_e1599_q_d_n5: f64 = (p.p7 * eq125_e1597_d_n5);
        let eq125_e1599_q_d_n6: f64 = (p.p7 * eq125_e1597_d_n6);
        let eq125_e1599_q_d_n7: f64 = (p.p7 * eq125_e1597_d_n7);
        let eq125_e1599_q_d_n8: f64 = (p.p7 * eq125_e1597_d_n8);
        let eq125_e1599_q_d_n9: f64 = (p.p7 * eq125_e1597_d_n9);
        let eq125_e1599_q_d_n10: f64 = (p.p7 * eq125_e1597_d_n10);
        let eq125_e1599_q_d_n11: f64 = (p.p7 * eq125_e1597_d_n11);
        let eq125_e1599_q_d_n12: f64 = (p.p7 * eq125_e1597_d_n12);
        let eq125_e1599_q_d_n13: f64 = (p.p7 * eq125_e1597_d_n13);
        let eq125_e1599_q_d_n14: f64 = (p.p7 * eq125_e1597_d_n14);
        let eq125_e1599_q_d_n15: f64 = (p.p7 * eq125_e1597_d_n15);
        let eq125_e1599_q_d_n16: f64 = (p.p7 * eq125_e1597_d_n16);
        let eq125_e1599_q_d_n17: f64 = (p.p7 * eq125_e1597_d_n17);
        let eq125_e1599_q_d_n18: f64 = (p.p7 * eq125_e1597_d_n18);
        let eq125_e1599_q_d_n19: f64 = (p.p7 * eq125_e1597_d_n19);
        let eq125_e1599_q_d_n20: f64 = (p.p7 * eq125_e1597_d_n20);
        let eq125_e1599_q_d_n21: f64 = (p.p7 * eq125_e1597_d_n21);
        let eq125_e1599_q_d_n22: f64 = (p.p7 * eq125_e1597_d_n22);
        (eq125_e1599, eq125_e1599_d_n0, eq125_e1599_d_n1, eq125_e1599_d_n2, eq125_e1599_d_n3, eq125_e1599_d_n4, eq125_e1599_d_n5, eq125_e1599_d_n6, eq125_e1599_d_n7, eq125_e1599_d_n8, eq125_e1599_d_n9, eq125_e1599_d_n10, eq125_e1599_d_n11, eq125_e1599_d_n12, eq125_e1599_d_n13, eq125_e1599_d_n14, eq125_e1599_d_n15, eq125_e1599_d_n16, eq125_e1599_d_n17, eq125_e1599_d_n18, eq125_e1599_d_n19, eq125_e1599_d_n20, eq125_e1599_d_n21, eq125_e1599_d_n22, eq125_e1599_q, eq125_e1599_q_d_n0, eq125_e1599_q_d_n1, eq125_e1599_q_d_n2, eq125_e1599_q_d_n3, eq125_e1599_q_d_n4, eq125_e1599_q_d_n5, eq125_e1599_q_d_n6, eq125_e1599_q_d_n7, eq125_e1599_q_d_n8, eq125_e1599_q_d_n9, eq125_e1599_q_d_n10, eq125_e1599_q_d_n11, eq125_e1599_q_d_n12, eq125_e1599_q_d_n13, eq125_e1599_q_d_n14, eq125_e1599_q_d_n15, eq125_e1599_q_d_n16, eq125_e1599_q_d_n17, eq125_e1599_q_d_n18, eq125_e1599_q_d_n19, eq125_e1599_q_d_n20, eq125_e1599_q_d_n21, eq125_e1599_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq125_reactive_node_derivatives: [f64; 23] = [eq125_e1601_q_d_n0, eq125_e1601_q_d_n1, eq125_e1601_q_d_n2, eq125_e1601_q_d_n3, eq125_e1601_q_d_n4, eq125_e1601_q_d_n5, eq125_e1601_q_d_n6, eq125_e1601_q_d_n7, eq125_e1601_q_d_n8, eq125_e1601_q_d_n9, eq125_e1601_q_d_n10, eq125_e1601_q_d_n11, eq125_e1601_q_d_n12, eq125_e1601_q_d_n13, eq125_e1601_q_d_n14, eq125_e1601_q_d_n15, eq125_e1601_q_d_n16, eq125_e1601_q_d_n17, eq125_e1601_q_d_n18, eq125_e1601_q_d_n19, eq125_e1601_q_d_n20, eq125_e1601_q_d_n21, eq125_e1601_q_d_n22];
        let eq125_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            &nodes,
            &eq125_reactive_node_derivatives,
            &branches,
            &eq125_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_126_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq126_e1611, eq126_e1611_d_n0, eq126_e1611_d_n1, eq126_e1611_d_n2, eq126_e1611_d_n3, eq126_e1611_d_n4, eq126_e1611_d_n5, eq126_e1611_d_n6, eq126_e1611_d_n7, eq126_e1611_d_n8, eq126_e1611_d_n9, eq126_e1611_d_n10, eq126_e1611_d_n11, eq126_e1611_d_n12, eq126_e1611_d_n13, eq126_e1611_d_n14, eq126_e1611_d_n15, eq126_e1611_d_n16, eq126_e1611_d_n17, eq126_e1611_d_n18, eq126_e1611_d_n19, eq126_e1611_d_n20, eq126_e1611_d_n21, eq126_e1611_d_n22, eq126_e1611_q, eq126_e1611_q_d_n0, eq126_e1611_q_d_n1, eq126_e1611_q_d_n2, eq126_e1611_q_d_n3, eq126_e1611_q_d_n4, eq126_e1611_q_d_n5, eq126_e1611_q_d_n6, eq126_e1611_q_d_n7, eq126_e1611_q_d_n8, eq126_e1611_q_d_n9, eq126_e1611_q_d_n10, eq126_e1611_q_d_n11, eq126_e1611_q_d_n12, eq126_e1611_q_d_n13, eq126_e1611_q_d_n14, eq126_e1611_q_d_n15, eq126_e1611_q_d_n16, eq126_e1611_q_d_n17, eq126_e1611_q_d_n18, eq126_e1611_q_d_n19, eq126_e1611_q_d_n20, eq126_e1611_q_d_n21, eq126_e1611_q_d_n22,) = {
    if ((!(s.v[570] != 0.0)) && (s.v[573] != 0.0)) {
        let eq126_e1608_q: f64 = s.v[229];
        let eq126_e1609: f64 = (p.p7 * s.v[229]);
        let eq126_e1609_d_n0: f64 = (p.p7 * s.dn[229][0]);
        let eq126_e1609_d_n1: f64 = (p.p7 * s.dn[229][1]);
        let eq126_e1609_d_n2: f64 = (p.p7 * s.dn[229][2]);
        let eq126_e1609_d_n3: f64 = (p.p7 * s.dn[229][3]);
        let eq126_e1609_d_n4: f64 = (p.p7 * s.dn[229][4]);
        let eq126_e1609_d_n5: f64 = (p.p7 * s.dn[229][5]);
        let eq126_e1609_d_n6: f64 = (p.p7 * s.dn[229][6]);
        let eq126_e1609_d_n7: f64 = (p.p7 * s.dn[229][7]);
        let eq126_e1609_d_n8: f64 = (p.p7 * s.dn[229][8]);
        let eq126_e1609_d_n9: f64 = (p.p7 * s.dn[229][9]);
        let eq126_e1609_d_n10: f64 = (p.p7 * s.dn[229][10]);
        let eq126_e1609_d_n11: f64 = (p.p7 * s.dn[229][11]);
        let eq126_e1609_d_n12: f64 = (p.p7 * s.dn[229][12]);
        let eq126_e1609_d_n13: f64 = (p.p7 * s.dn[229][13]);
        let eq126_e1609_d_n14: f64 = (p.p7 * s.dn[229][14]);
        let eq126_e1609_d_n15: f64 = (p.p7 * s.dn[229][15]);
        let eq126_e1609_d_n16: f64 = (p.p7 * s.dn[229][16]);
        let eq126_e1609_d_n17: f64 = (p.p7 * s.dn[229][17]);
        let eq126_e1609_d_n18: f64 = (p.p7 * s.dn[229][18]);
        let eq126_e1609_d_n19: f64 = (p.p7 * s.dn[229][19]);
        let eq126_e1609_d_n20: f64 = (p.p7 * s.dn[229][20]);
        let eq126_e1609_d_n21: f64 = (p.p7 * s.dn[229][21]);
        let eq126_e1609_d_n22: f64 = (p.p7 * s.dn[229][22]);
        let eq126_e1609_q: f64 = (p.p7 * eq126_e1608_q);
        let eq126_e1609_q_d_n0: f64 = (p.p7 * s.dn[229][0]);
        let eq126_e1609_q_d_n1: f64 = (p.p7 * s.dn[229][1]);
        let eq126_e1609_q_d_n2: f64 = (p.p7 * s.dn[229][2]);
        let eq126_e1609_q_d_n3: f64 = (p.p7 * s.dn[229][3]);
        let eq126_e1609_q_d_n4: f64 = (p.p7 * s.dn[229][4]);
        let eq126_e1609_q_d_n5: f64 = (p.p7 * s.dn[229][5]);
        let eq126_e1609_q_d_n6: f64 = (p.p7 * s.dn[229][6]);
        let eq126_e1609_q_d_n7: f64 = (p.p7 * s.dn[229][7]);
        let eq126_e1609_q_d_n8: f64 = (p.p7 * s.dn[229][8]);
        let eq126_e1609_q_d_n9: f64 = (p.p7 * s.dn[229][9]);
        let eq126_e1609_q_d_n10: f64 = (p.p7 * s.dn[229][10]);
        let eq126_e1609_q_d_n11: f64 = (p.p7 * s.dn[229][11]);
        let eq126_e1609_q_d_n12: f64 = (p.p7 * s.dn[229][12]);
        let eq126_e1609_q_d_n13: f64 = (p.p7 * s.dn[229][13]);
        let eq126_e1609_q_d_n14: f64 = (p.p7 * s.dn[229][14]);
        let eq126_e1609_q_d_n15: f64 = (p.p7 * s.dn[229][15]);
        let eq126_e1609_q_d_n16: f64 = (p.p7 * s.dn[229][16]);
        let eq126_e1609_q_d_n17: f64 = (p.p7 * s.dn[229][17]);
        let eq126_e1609_q_d_n18: f64 = (p.p7 * s.dn[229][18]);
        let eq126_e1609_q_d_n19: f64 = (p.p7 * s.dn[229][19]);
        let eq126_e1609_q_d_n20: f64 = (p.p7 * s.dn[229][20]);
        let eq126_e1609_q_d_n21: f64 = (p.p7 * s.dn[229][21]);
        let eq126_e1609_q_d_n22: f64 = (p.p7 * s.dn[229][22]);
        (eq126_e1609, eq126_e1609_d_n0, eq126_e1609_d_n1, eq126_e1609_d_n2, eq126_e1609_d_n3, eq126_e1609_d_n4, eq126_e1609_d_n5, eq126_e1609_d_n6, eq126_e1609_d_n7, eq126_e1609_d_n8, eq126_e1609_d_n9, eq126_e1609_d_n10, eq126_e1609_d_n11, eq126_e1609_d_n12, eq126_e1609_d_n13, eq126_e1609_d_n14, eq126_e1609_d_n15, eq126_e1609_d_n16, eq126_e1609_d_n17, eq126_e1609_d_n18, eq126_e1609_d_n19, eq126_e1609_d_n20, eq126_e1609_d_n21, eq126_e1609_d_n22, eq126_e1609_q, eq126_e1609_q_d_n0, eq126_e1609_q_d_n1, eq126_e1609_q_d_n2, eq126_e1609_q_d_n3, eq126_e1609_q_d_n4, eq126_e1609_q_d_n5, eq126_e1609_q_d_n6, eq126_e1609_q_d_n7, eq126_e1609_q_d_n8, eq126_e1609_q_d_n9, eq126_e1609_q_d_n10, eq126_e1609_q_d_n11, eq126_e1609_q_d_n12, eq126_e1609_q_d_n13, eq126_e1609_q_d_n14, eq126_e1609_q_d_n15, eq126_e1609_q_d_n16, eq126_e1609_q_d_n17, eq126_e1609_q_d_n18, eq126_e1609_q_d_n19, eq126_e1609_q_d_n20, eq126_e1609_q_d_n21, eq126_e1609_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq126_reactive_node_derivatives: [f64; 23] = [eq126_e1611_q_d_n0, eq126_e1611_q_d_n1, eq126_e1611_q_d_n2, eq126_e1611_q_d_n3, eq126_e1611_q_d_n4, eq126_e1611_q_d_n5, eq126_e1611_q_d_n6, eq126_e1611_q_d_n7, eq126_e1611_q_d_n8, eq126_e1611_q_d_n9, eq126_e1611_q_d_n10, eq126_e1611_q_d_n11, eq126_e1611_q_d_n12, eq126_e1611_q_d_n13, eq126_e1611_q_d_n14, eq126_e1611_q_d_n15, eq126_e1611_q_d_n16, eq126_e1611_q_d_n17, eq126_e1611_q_d_n18, eq126_e1611_q_d_n19, eq126_e1611_q_d_n20, eq126_e1611_q_d_n21, eq126_e1611_q_d_n22];
        let eq126_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[7]),
            &nodes,
            &eq126_reactive_node_derivatives,
            &branches,
            &eq126_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_127_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq127_e1623, eq127_e1623_d_n0, eq127_e1623_d_n1, eq127_e1623_d_n2, eq127_e1623_d_n3, eq127_e1623_d_n4, eq127_e1623_d_n5, eq127_e1623_d_n6, eq127_e1623_d_n7, eq127_e1623_d_n8, eq127_e1623_d_n9, eq127_e1623_d_n10, eq127_e1623_d_n11, eq127_e1623_d_n12, eq127_e1623_d_n13, eq127_e1623_d_n14, eq127_e1623_d_n15, eq127_e1623_d_n16, eq127_e1623_d_n17, eq127_e1623_d_n18, eq127_e1623_d_n19, eq127_e1623_d_n20, eq127_e1623_d_n21, eq127_e1623_d_n22, eq127_e1623_q, eq127_e1623_q_d_n0, eq127_e1623_q_d_n1, eq127_e1623_q_d_n2, eq127_e1623_q_d_n3, eq127_e1623_q_d_n4, eq127_e1623_q_d_n5, eq127_e1623_q_d_n6, eq127_e1623_q_d_n7, eq127_e1623_q_d_n8, eq127_e1623_q_d_n9, eq127_e1623_q_d_n10, eq127_e1623_q_d_n11, eq127_e1623_q_d_n12, eq127_e1623_q_d_n13, eq127_e1623_q_d_n14, eq127_e1623_q_d_n15, eq127_e1623_q_d_n16, eq127_e1623_q_d_n17, eq127_e1623_q_d_n18, eq127_e1623_q_d_n19, eq127_e1623_q_d_n20, eq127_e1623_q_d_n21, eq127_e1623_q_d_n22,) = {
    if (((!(s.v[570] != 0.0)) && (s.v[573] != 0.0)) && (s.v[574] != 0.0)) {
        let eq127_e1620_q: f64 = s.v[228];
        let eq127_e1621: f64 = (p.p7 * s.v[228]);
        let eq127_e1621_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq127_e1621_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq127_e1621_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq127_e1621_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq127_e1621_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq127_e1621_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq127_e1621_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq127_e1621_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq127_e1621_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq127_e1621_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq127_e1621_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq127_e1621_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq127_e1621_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq127_e1621_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq127_e1621_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq127_e1621_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq127_e1621_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq127_e1621_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq127_e1621_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq127_e1621_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq127_e1621_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq127_e1621_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq127_e1621_d_n22: f64 = (p.p7 * s.dn[228][22]);
        let eq127_e1621_q: f64 = (p.p7 * eq127_e1620_q);
        let eq127_e1621_q_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq127_e1621_q_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq127_e1621_q_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq127_e1621_q_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq127_e1621_q_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq127_e1621_q_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq127_e1621_q_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq127_e1621_q_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq127_e1621_q_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq127_e1621_q_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq127_e1621_q_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq127_e1621_q_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq127_e1621_q_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq127_e1621_q_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq127_e1621_q_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq127_e1621_q_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq127_e1621_q_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq127_e1621_q_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq127_e1621_q_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq127_e1621_q_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq127_e1621_q_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq127_e1621_q_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq127_e1621_q_d_n22: f64 = (p.p7 * s.dn[228][22]);
        (eq127_e1621, eq127_e1621_d_n0, eq127_e1621_d_n1, eq127_e1621_d_n2, eq127_e1621_d_n3, eq127_e1621_d_n4, eq127_e1621_d_n5, eq127_e1621_d_n6, eq127_e1621_d_n7, eq127_e1621_d_n8, eq127_e1621_d_n9, eq127_e1621_d_n10, eq127_e1621_d_n11, eq127_e1621_d_n12, eq127_e1621_d_n13, eq127_e1621_d_n14, eq127_e1621_d_n15, eq127_e1621_d_n16, eq127_e1621_d_n17, eq127_e1621_d_n18, eq127_e1621_d_n19, eq127_e1621_d_n20, eq127_e1621_d_n21, eq127_e1621_d_n22, eq127_e1621_q, eq127_e1621_q_d_n0, eq127_e1621_q_d_n1, eq127_e1621_q_d_n2, eq127_e1621_q_d_n3, eq127_e1621_q_d_n4, eq127_e1621_q_d_n5, eq127_e1621_q_d_n6, eq127_e1621_q_d_n7, eq127_e1621_q_d_n8, eq127_e1621_q_d_n9, eq127_e1621_q_d_n10, eq127_e1621_q_d_n11, eq127_e1621_q_d_n12, eq127_e1621_q_d_n13, eq127_e1621_q_d_n14, eq127_e1621_q_d_n15, eq127_e1621_q_d_n16, eq127_e1621_q_d_n17, eq127_e1621_q_d_n18, eq127_e1621_q_d_n19, eq127_e1621_q_d_n20, eq127_e1621_q_d_n21, eq127_e1621_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq127_reactive_node_derivatives: [f64; 23] = [eq127_e1623_q_d_n0, eq127_e1623_q_d_n1, eq127_e1623_q_d_n2, eq127_e1623_q_d_n3, eq127_e1623_q_d_n4, eq127_e1623_q_d_n5, eq127_e1623_q_d_n6, eq127_e1623_q_d_n7, eq127_e1623_q_d_n8, eq127_e1623_q_d_n9, eq127_e1623_q_d_n10, eq127_e1623_q_d_n11, eq127_e1623_q_d_n12, eq127_e1623_q_d_n13, eq127_e1623_q_d_n14, eq127_e1623_q_d_n15, eq127_e1623_q_d_n16, eq127_e1623_q_d_n17, eq127_e1623_q_d_n18, eq127_e1623_q_d_n19, eq127_e1623_q_d_n20, eq127_e1623_q_d_n21, eq127_e1623_q_d_n22];
        let eq127_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            &nodes,
            &eq127_reactive_node_derivatives,
            &branches,
            &eq127_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_128_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq128_e1637, eq128_e1637_d_n0, eq128_e1637_d_n1, eq128_e1637_d_n2, eq128_e1637_d_n3, eq128_e1637_d_n4, eq128_e1637_d_n5, eq128_e1637_d_n6, eq128_e1637_d_n7, eq128_e1637_d_n8, eq128_e1637_d_n9, eq128_e1637_d_n10, eq128_e1637_d_n11, eq128_e1637_d_n12, eq128_e1637_d_n13, eq128_e1637_d_n14, eq128_e1637_d_n15, eq128_e1637_d_n16, eq128_e1637_d_n17, eq128_e1637_d_n18, eq128_e1637_d_n19, eq128_e1637_d_n20, eq128_e1637_d_n21, eq128_e1637_d_n22, eq128_e1637_q, eq128_e1637_q_d_n0, eq128_e1637_q_d_n1, eq128_e1637_q_d_n2, eq128_e1637_q_d_n3, eq128_e1637_q_d_n4, eq128_e1637_q_d_n5, eq128_e1637_q_d_n6, eq128_e1637_q_d_n7, eq128_e1637_q_d_n8, eq128_e1637_q_d_n9, eq128_e1637_q_d_n10, eq128_e1637_q_d_n11, eq128_e1637_q_d_n12, eq128_e1637_q_d_n13, eq128_e1637_q_d_n14, eq128_e1637_q_d_n15, eq128_e1637_q_d_n16, eq128_e1637_q_d_n17, eq128_e1637_q_d_n18, eq128_e1637_q_d_n19, eq128_e1637_q_d_n20, eq128_e1637_q_d_n21, eq128_e1637_q_d_n22,) = {
    if (((!(s.v[570] != 0.0)) && (s.v[573] != 0.0)) && (s.v[574] != 0.0)) {
        let eq128_e1632_q: f64 = s.v[228];
        let eq128_e1633: f64 = (p.p7 * s.v[228]);
        let eq128_e1633_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq128_e1633_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq128_e1633_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq128_e1633_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq128_e1633_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq128_e1633_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq128_e1633_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq128_e1633_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq128_e1633_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq128_e1633_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq128_e1633_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq128_e1633_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq128_e1633_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq128_e1633_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq128_e1633_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq128_e1633_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq128_e1633_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq128_e1633_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq128_e1633_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq128_e1633_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq128_e1633_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq128_e1633_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq128_e1633_d_n22: f64 = (p.p7 * s.dn[228][22]);
        let eq128_e1633_q: f64 = (p.p7 * eq128_e1632_q);
        let eq128_e1633_q_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq128_e1633_q_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq128_e1633_q_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq128_e1633_q_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq128_e1633_q_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq128_e1633_q_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq128_e1633_q_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq128_e1633_q_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq128_e1633_q_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq128_e1633_q_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq128_e1633_q_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq128_e1633_q_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq128_e1633_q_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq128_e1633_q_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq128_e1633_q_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq128_e1633_q_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq128_e1633_q_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq128_e1633_q_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq128_e1633_q_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq128_e1633_q_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq128_e1633_q_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq128_e1633_q_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq128_e1633_q_d_n22: f64 = (p.p7 * s.dn[228][22]);
        let eq128_e1635: f64 = (eq128_e1633 * p.p246);
        let eq128_e1635_d_n0: f64 = (eq128_e1633_d_n0 * p.p246);
        let eq128_e1635_d_n1: f64 = (eq128_e1633_d_n1 * p.p246);
        let eq128_e1635_d_n2: f64 = (eq128_e1633_d_n2 * p.p246);
        let eq128_e1635_d_n3: f64 = (eq128_e1633_d_n3 * p.p246);
        let eq128_e1635_d_n4: f64 = (eq128_e1633_d_n4 * p.p246);
        let eq128_e1635_d_n5: f64 = (eq128_e1633_d_n5 * p.p246);
        let eq128_e1635_d_n6: f64 = (eq128_e1633_d_n6 * p.p246);
        let eq128_e1635_d_n7: f64 = (eq128_e1633_d_n7 * p.p246);
        let eq128_e1635_d_n8: f64 = (eq128_e1633_d_n8 * p.p246);
        let eq128_e1635_d_n9: f64 = (eq128_e1633_d_n9 * p.p246);
        let eq128_e1635_d_n10: f64 = (eq128_e1633_d_n10 * p.p246);
        let eq128_e1635_d_n11: f64 = (eq128_e1633_d_n11 * p.p246);
        let eq128_e1635_d_n12: f64 = (eq128_e1633_d_n12 * p.p246);
        let eq128_e1635_d_n13: f64 = (eq128_e1633_d_n13 * p.p246);
        let eq128_e1635_d_n14: f64 = (eq128_e1633_d_n14 * p.p246);
        let eq128_e1635_d_n15: f64 = (eq128_e1633_d_n15 * p.p246);
        let eq128_e1635_d_n16: f64 = (eq128_e1633_d_n16 * p.p246);
        let eq128_e1635_d_n17: f64 = (eq128_e1633_d_n17 * p.p246);
        let eq128_e1635_d_n18: f64 = (eq128_e1633_d_n18 * p.p246);
        let eq128_e1635_d_n19: f64 = (eq128_e1633_d_n19 * p.p246);
        let eq128_e1635_d_n20: f64 = (eq128_e1633_d_n20 * p.p246);
        let eq128_e1635_d_n21: f64 = (eq128_e1633_d_n21 * p.p246);
        let eq128_e1635_d_n22: f64 = (eq128_e1633_d_n22 * p.p246);
        let eq128_e1635_q: f64 = (eq128_e1633_q * p.p246);
        let eq128_e1635_q_d_n0: f64 = (eq128_e1633_q_d_n0 * p.p246);
        let eq128_e1635_q_d_n1: f64 = (eq128_e1633_q_d_n1 * p.p246);
        let eq128_e1635_q_d_n2: f64 = (eq128_e1633_q_d_n2 * p.p246);
        let eq128_e1635_q_d_n3: f64 = (eq128_e1633_q_d_n3 * p.p246);
        let eq128_e1635_q_d_n4: f64 = (eq128_e1633_q_d_n4 * p.p246);
        let eq128_e1635_q_d_n5: f64 = (eq128_e1633_q_d_n5 * p.p246);
        let eq128_e1635_q_d_n6: f64 = (eq128_e1633_q_d_n6 * p.p246);
        let eq128_e1635_q_d_n7: f64 = (eq128_e1633_q_d_n7 * p.p246);
        let eq128_e1635_q_d_n8: f64 = (eq128_e1633_q_d_n8 * p.p246);
        let eq128_e1635_q_d_n9: f64 = (eq128_e1633_q_d_n9 * p.p246);
        let eq128_e1635_q_d_n10: f64 = (eq128_e1633_q_d_n10 * p.p246);
        let eq128_e1635_q_d_n11: f64 = (eq128_e1633_q_d_n11 * p.p246);
        let eq128_e1635_q_d_n12: f64 = (eq128_e1633_q_d_n12 * p.p246);
        let eq128_e1635_q_d_n13: f64 = (eq128_e1633_q_d_n13 * p.p246);
        let eq128_e1635_q_d_n14: f64 = (eq128_e1633_q_d_n14 * p.p246);
        let eq128_e1635_q_d_n15: f64 = (eq128_e1633_q_d_n15 * p.p246);
        let eq128_e1635_q_d_n16: f64 = (eq128_e1633_q_d_n16 * p.p246);
        let eq128_e1635_q_d_n17: f64 = (eq128_e1633_q_d_n17 * p.p246);
        let eq128_e1635_q_d_n18: f64 = (eq128_e1633_q_d_n18 * p.p246);
        let eq128_e1635_q_d_n19: f64 = (eq128_e1633_q_d_n19 * p.p246);
        let eq128_e1635_q_d_n20: f64 = (eq128_e1633_q_d_n20 * p.p246);
        let eq128_e1635_q_d_n21: f64 = (eq128_e1633_q_d_n21 * p.p246);
        let eq128_e1635_q_d_n22: f64 = (eq128_e1633_q_d_n22 * p.p246);
        (eq128_e1635, eq128_e1635_d_n0, eq128_e1635_d_n1, eq128_e1635_d_n2, eq128_e1635_d_n3, eq128_e1635_d_n4, eq128_e1635_d_n5, eq128_e1635_d_n6, eq128_e1635_d_n7, eq128_e1635_d_n8, eq128_e1635_d_n9, eq128_e1635_d_n10, eq128_e1635_d_n11, eq128_e1635_d_n12, eq128_e1635_d_n13, eq128_e1635_d_n14, eq128_e1635_d_n15, eq128_e1635_d_n16, eq128_e1635_d_n17, eq128_e1635_d_n18, eq128_e1635_d_n19, eq128_e1635_d_n20, eq128_e1635_d_n21, eq128_e1635_d_n22, eq128_e1635_q, eq128_e1635_q_d_n0, eq128_e1635_q_d_n1, eq128_e1635_q_d_n2, eq128_e1635_q_d_n3, eq128_e1635_q_d_n4, eq128_e1635_q_d_n5, eq128_e1635_q_d_n6, eq128_e1635_q_d_n7, eq128_e1635_q_d_n8, eq128_e1635_q_d_n9, eq128_e1635_q_d_n10, eq128_e1635_q_d_n11, eq128_e1635_q_d_n12, eq128_e1635_q_d_n13, eq128_e1635_q_d_n14, eq128_e1635_q_d_n15, eq128_e1635_q_d_n16, eq128_e1635_q_d_n17, eq128_e1635_q_d_n18, eq128_e1635_q_d_n19, eq128_e1635_q_d_n20, eq128_e1635_q_d_n21, eq128_e1635_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq128_reactive_node_derivatives: [f64; 23] = [eq128_e1637_q_d_n0, eq128_e1637_q_d_n1, eq128_e1637_q_d_n2, eq128_e1637_q_d_n3, eq128_e1637_q_d_n4, eq128_e1637_q_d_n5, eq128_e1637_q_d_n6, eq128_e1637_q_d_n7, eq128_e1637_q_d_n8, eq128_e1637_q_d_n9, eq128_e1637_q_d_n10, eq128_e1637_q_d_n11, eq128_e1637_q_d_n12, eq128_e1637_q_d_n13, eq128_e1637_q_d_n14, eq128_e1637_q_d_n15, eq128_e1637_q_d_n16, eq128_e1637_q_d_n17, eq128_e1637_q_d_n18, eq128_e1637_q_d_n19, eq128_e1637_q_d_n20, eq128_e1637_q_d_n21, eq128_e1637_q_d_n22];
        let eq128_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            &nodes,
            &eq128_reactive_node_derivatives,
            &branches,
            &eq128_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_129_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq129_e1650, eq129_e1650_d_n0, eq129_e1650_d_n1, eq129_e1650_d_n2, eq129_e1650_d_n3, eq129_e1650_d_n4, eq129_e1650_d_n5, eq129_e1650_d_n6, eq129_e1650_d_n7, eq129_e1650_d_n8, eq129_e1650_d_n9, eq129_e1650_d_n10, eq129_e1650_d_n11, eq129_e1650_d_n12, eq129_e1650_d_n13, eq129_e1650_d_n14, eq129_e1650_d_n15, eq129_e1650_d_n16, eq129_e1650_d_n17, eq129_e1650_d_n18, eq129_e1650_d_n19, eq129_e1650_d_n20, eq129_e1650_d_n21, eq129_e1650_d_n22, eq129_e1650_q, eq129_e1650_q_d_n0, eq129_e1650_q_d_n1, eq129_e1650_q_d_n2, eq129_e1650_q_d_n3, eq129_e1650_q_d_n4, eq129_e1650_q_d_n5, eq129_e1650_q_d_n6, eq129_e1650_q_d_n7, eq129_e1650_q_d_n8, eq129_e1650_q_d_n9, eq129_e1650_q_d_n10, eq129_e1650_q_d_n11, eq129_e1650_q_d_n12, eq129_e1650_q_d_n13, eq129_e1650_q_d_n14, eq129_e1650_q_d_n15, eq129_e1650_q_d_n16, eq129_e1650_q_d_n17, eq129_e1650_q_d_n18, eq129_e1650_q_d_n19, eq129_e1650_q_d_n20, eq129_e1650_q_d_n21, eq129_e1650_q_d_n22,) = {
    if (((!(s.v[570] != 0.0)) && (s.v[573] != 0.0)) && (!(s.v[574] != 0.0))) {
        let eq129_e1647_q: f64 = s.v[228];
        let eq129_e1648: f64 = (p.p7 * s.v[228]);
        let eq129_e1648_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq129_e1648_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq129_e1648_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq129_e1648_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq129_e1648_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq129_e1648_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq129_e1648_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq129_e1648_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq129_e1648_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq129_e1648_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq129_e1648_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq129_e1648_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq129_e1648_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq129_e1648_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq129_e1648_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq129_e1648_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq129_e1648_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq129_e1648_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq129_e1648_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq129_e1648_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq129_e1648_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq129_e1648_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq129_e1648_d_n22: f64 = (p.p7 * s.dn[228][22]);
        let eq129_e1648_q: f64 = (p.p7 * eq129_e1647_q);
        let eq129_e1648_q_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq129_e1648_q_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq129_e1648_q_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq129_e1648_q_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq129_e1648_q_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq129_e1648_q_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq129_e1648_q_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq129_e1648_q_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq129_e1648_q_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq129_e1648_q_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq129_e1648_q_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq129_e1648_q_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq129_e1648_q_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq129_e1648_q_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq129_e1648_q_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq129_e1648_q_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq129_e1648_q_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq129_e1648_q_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq129_e1648_q_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq129_e1648_q_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq129_e1648_q_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq129_e1648_q_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq129_e1648_q_d_n22: f64 = (p.p7 * s.dn[228][22]);
        (eq129_e1648, eq129_e1648_d_n0, eq129_e1648_d_n1, eq129_e1648_d_n2, eq129_e1648_d_n3, eq129_e1648_d_n4, eq129_e1648_d_n5, eq129_e1648_d_n6, eq129_e1648_d_n7, eq129_e1648_d_n8, eq129_e1648_d_n9, eq129_e1648_d_n10, eq129_e1648_d_n11, eq129_e1648_d_n12, eq129_e1648_d_n13, eq129_e1648_d_n14, eq129_e1648_d_n15, eq129_e1648_d_n16, eq129_e1648_d_n17, eq129_e1648_d_n18, eq129_e1648_d_n19, eq129_e1648_d_n20, eq129_e1648_d_n21, eq129_e1648_d_n22, eq129_e1648_q, eq129_e1648_q_d_n0, eq129_e1648_q_d_n1, eq129_e1648_q_d_n2, eq129_e1648_q_d_n3, eq129_e1648_q_d_n4, eq129_e1648_q_d_n5, eq129_e1648_q_d_n6, eq129_e1648_q_d_n7, eq129_e1648_q_d_n8, eq129_e1648_q_d_n9, eq129_e1648_q_d_n10, eq129_e1648_q_d_n11, eq129_e1648_q_d_n12, eq129_e1648_q_d_n13, eq129_e1648_q_d_n14, eq129_e1648_q_d_n15, eq129_e1648_q_d_n16, eq129_e1648_q_d_n17, eq129_e1648_q_d_n18, eq129_e1648_q_d_n19, eq129_e1648_q_d_n20, eq129_e1648_q_d_n21, eq129_e1648_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq129_reactive_node_derivatives: [f64; 23] = [eq129_e1650_q_d_n0, eq129_e1650_q_d_n1, eq129_e1650_q_d_n2, eq129_e1650_q_d_n3, eq129_e1650_q_d_n4, eq129_e1650_q_d_n5, eq129_e1650_q_d_n6, eq129_e1650_q_d_n7, eq129_e1650_q_d_n8, eq129_e1650_q_d_n9, eq129_e1650_q_d_n10, eq129_e1650_q_d_n11, eq129_e1650_q_d_n12, eq129_e1650_q_d_n13, eq129_e1650_q_d_n14, eq129_e1650_q_d_n15, eq129_e1650_q_d_n16, eq129_e1650_q_d_n17, eq129_e1650_q_d_n18, eq129_e1650_q_d_n19, eq129_e1650_q_d_n20, eq129_e1650_q_d_n21, eq129_e1650_q_d_n22];
        let eq129_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            &nodes,
            &eq129_reactive_node_derivatives,
            &branches,
            &eq129_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_130_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq130_e1665, eq130_e1665_d_n0, eq130_e1665_d_n1, eq130_e1665_d_n2, eq130_e1665_d_n3, eq130_e1665_d_n4, eq130_e1665_d_n5, eq130_e1665_d_n6, eq130_e1665_d_n7, eq130_e1665_d_n8, eq130_e1665_d_n9, eq130_e1665_d_n10, eq130_e1665_d_n11, eq130_e1665_d_n12, eq130_e1665_d_n13, eq130_e1665_d_n14, eq130_e1665_d_n15, eq130_e1665_d_n16, eq130_e1665_d_n17, eq130_e1665_d_n18, eq130_e1665_d_n19, eq130_e1665_d_n20, eq130_e1665_d_n21, eq130_e1665_d_n22, eq130_e1665_q, eq130_e1665_q_d_n0, eq130_e1665_q_d_n1, eq130_e1665_q_d_n2, eq130_e1665_q_d_n3, eq130_e1665_q_d_n4, eq130_e1665_q_d_n5, eq130_e1665_q_d_n6, eq130_e1665_q_d_n7, eq130_e1665_q_d_n8, eq130_e1665_q_d_n9, eq130_e1665_q_d_n10, eq130_e1665_q_d_n11, eq130_e1665_q_d_n12, eq130_e1665_q_d_n13, eq130_e1665_q_d_n14, eq130_e1665_q_d_n15, eq130_e1665_q_d_n16, eq130_e1665_q_d_n17, eq130_e1665_q_d_n18, eq130_e1665_q_d_n19, eq130_e1665_q_d_n20, eq130_e1665_q_d_n21, eq130_e1665_q_d_n22,) = {
    if (((!(s.v[570] != 0.0)) && (s.v[573] != 0.0)) && (!(s.v[574] != 0.0))) {
        let eq130_e1660_q: f64 = s.v[228];
        let eq130_e1661: f64 = (p.p7 * s.v[228]);
        let eq130_e1661_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq130_e1661_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq130_e1661_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq130_e1661_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq130_e1661_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq130_e1661_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq130_e1661_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq130_e1661_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq130_e1661_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq130_e1661_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq130_e1661_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq130_e1661_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq130_e1661_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq130_e1661_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq130_e1661_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq130_e1661_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq130_e1661_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq130_e1661_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq130_e1661_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq130_e1661_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq130_e1661_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq130_e1661_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq130_e1661_d_n22: f64 = (p.p7 * s.dn[228][22]);
        let eq130_e1661_q: f64 = (p.p7 * eq130_e1660_q);
        let eq130_e1661_q_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq130_e1661_q_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq130_e1661_q_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq130_e1661_q_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq130_e1661_q_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq130_e1661_q_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq130_e1661_q_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq130_e1661_q_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq130_e1661_q_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq130_e1661_q_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq130_e1661_q_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq130_e1661_q_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq130_e1661_q_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq130_e1661_q_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq130_e1661_q_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq130_e1661_q_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq130_e1661_q_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq130_e1661_q_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq130_e1661_q_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq130_e1661_q_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq130_e1661_q_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq130_e1661_q_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq130_e1661_q_d_n22: f64 = (p.p7 * s.dn[228][22]);
        let eq130_e1663: f64 = (eq130_e1661 * p.p246);
        let eq130_e1663_d_n0: f64 = (eq130_e1661_d_n0 * p.p246);
        let eq130_e1663_d_n1: f64 = (eq130_e1661_d_n1 * p.p246);
        let eq130_e1663_d_n2: f64 = (eq130_e1661_d_n2 * p.p246);
        let eq130_e1663_d_n3: f64 = (eq130_e1661_d_n3 * p.p246);
        let eq130_e1663_d_n4: f64 = (eq130_e1661_d_n4 * p.p246);
        let eq130_e1663_d_n5: f64 = (eq130_e1661_d_n5 * p.p246);
        let eq130_e1663_d_n6: f64 = (eq130_e1661_d_n6 * p.p246);
        let eq130_e1663_d_n7: f64 = (eq130_e1661_d_n7 * p.p246);
        let eq130_e1663_d_n8: f64 = (eq130_e1661_d_n8 * p.p246);
        let eq130_e1663_d_n9: f64 = (eq130_e1661_d_n9 * p.p246);
        let eq130_e1663_d_n10: f64 = (eq130_e1661_d_n10 * p.p246);
        let eq130_e1663_d_n11: f64 = (eq130_e1661_d_n11 * p.p246);
        let eq130_e1663_d_n12: f64 = (eq130_e1661_d_n12 * p.p246);
        let eq130_e1663_d_n13: f64 = (eq130_e1661_d_n13 * p.p246);
        let eq130_e1663_d_n14: f64 = (eq130_e1661_d_n14 * p.p246);
        let eq130_e1663_d_n15: f64 = (eq130_e1661_d_n15 * p.p246);
        let eq130_e1663_d_n16: f64 = (eq130_e1661_d_n16 * p.p246);
        let eq130_e1663_d_n17: f64 = (eq130_e1661_d_n17 * p.p246);
        let eq130_e1663_d_n18: f64 = (eq130_e1661_d_n18 * p.p246);
        let eq130_e1663_d_n19: f64 = (eq130_e1661_d_n19 * p.p246);
        let eq130_e1663_d_n20: f64 = (eq130_e1661_d_n20 * p.p246);
        let eq130_e1663_d_n21: f64 = (eq130_e1661_d_n21 * p.p246);
        let eq130_e1663_d_n22: f64 = (eq130_e1661_d_n22 * p.p246);
        let eq130_e1663_q: f64 = (eq130_e1661_q * p.p246);
        let eq130_e1663_q_d_n0: f64 = (eq130_e1661_q_d_n0 * p.p246);
        let eq130_e1663_q_d_n1: f64 = (eq130_e1661_q_d_n1 * p.p246);
        let eq130_e1663_q_d_n2: f64 = (eq130_e1661_q_d_n2 * p.p246);
        let eq130_e1663_q_d_n3: f64 = (eq130_e1661_q_d_n3 * p.p246);
        let eq130_e1663_q_d_n4: f64 = (eq130_e1661_q_d_n4 * p.p246);
        let eq130_e1663_q_d_n5: f64 = (eq130_e1661_q_d_n5 * p.p246);
        let eq130_e1663_q_d_n6: f64 = (eq130_e1661_q_d_n6 * p.p246);
        let eq130_e1663_q_d_n7: f64 = (eq130_e1661_q_d_n7 * p.p246);
        let eq130_e1663_q_d_n8: f64 = (eq130_e1661_q_d_n8 * p.p246);
        let eq130_e1663_q_d_n9: f64 = (eq130_e1661_q_d_n9 * p.p246);
        let eq130_e1663_q_d_n10: f64 = (eq130_e1661_q_d_n10 * p.p246);
        let eq130_e1663_q_d_n11: f64 = (eq130_e1661_q_d_n11 * p.p246);
        let eq130_e1663_q_d_n12: f64 = (eq130_e1661_q_d_n12 * p.p246);
        let eq130_e1663_q_d_n13: f64 = (eq130_e1661_q_d_n13 * p.p246);
        let eq130_e1663_q_d_n14: f64 = (eq130_e1661_q_d_n14 * p.p246);
        let eq130_e1663_q_d_n15: f64 = (eq130_e1661_q_d_n15 * p.p246);
        let eq130_e1663_q_d_n16: f64 = (eq130_e1661_q_d_n16 * p.p246);
        let eq130_e1663_q_d_n17: f64 = (eq130_e1661_q_d_n17 * p.p246);
        let eq130_e1663_q_d_n18: f64 = (eq130_e1661_q_d_n18 * p.p246);
        let eq130_e1663_q_d_n19: f64 = (eq130_e1661_q_d_n19 * p.p246);
        let eq130_e1663_q_d_n20: f64 = (eq130_e1661_q_d_n20 * p.p246);
        let eq130_e1663_q_d_n21: f64 = (eq130_e1661_q_d_n21 * p.p246);
        let eq130_e1663_q_d_n22: f64 = (eq130_e1661_q_d_n22 * p.p246);
        (eq130_e1663, eq130_e1663_d_n0, eq130_e1663_d_n1, eq130_e1663_d_n2, eq130_e1663_d_n3, eq130_e1663_d_n4, eq130_e1663_d_n5, eq130_e1663_d_n6, eq130_e1663_d_n7, eq130_e1663_d_n8, eq130_e1663_d_n9, eq130_e1663_d_n10, eq130_e1663_d_n11, eq130_e1663_d_n12, eq130_e1663_d_n13, eq130_e1663_d_n14, eq130_e1663_d_n15, eq130_e1663_d_n16, eq130_e1663_d_n17, eq130_e1663_d_n18, eq130_e1663_d_n19, eq130_e1663_d_n20, eq130_e1663_d_n21, eq130_e1663_d_n22, eq130_e1663_q, eq130_e1663_q_d_n0, eq130_e1663_q_d_n1, eq130_e1663_q_d_n2, eq130_e1663_q_d_n3, eq130_e1663_q_d_n4, eq130_e1663_q_d_n5, eq130_e1663_q_d_n6, eq130_e1663_q_d_n7, eq130_e1663_q_d_n8, eq130_e1663_q_d_n9, eq130_e1663_q_d_n10, eq130_e1663_q_d_n11, eq130_e1663_q_d_n12, eq130_e1663_q_d_n13, eq130_e1663_q_d_n14, eq130_e1663_q_d_n15, eq130_e1663_q_d_n16, eq130_e1663_q_d_n17, eq130_e1663_q_d_n18, eq130_e1663_q_d_n19, eq130_e1663_q_d_n20, eq130_e1663_q_d_n21, eq130_e1663_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq130_reactive_node_derivatives: [f64; 23] = [eq130_e1665_q_d_n0, eq130_e1665_q_d_n1, eq130_e1665_q_d_n2, eq130_e1665_q_d_n3, eq130_e1665_q_d_n4, eq130_e1665_q_d_n5, eq130_e1665_q_d_n6, eq130_e1665_q_d_n7, eq130_e1665_q_d_n8, eq130_e1665_q_d_n9, eq130_e1665_q_d_n10, eq130_e1665_q_d_n11, eq130_e1665_q_d_n12, eq130_e1665_q_d_n13, eq130_e1665_q_d_n14, eq130_e1665_q_d_n15, eq130_e1665_q_d_n16, eq130_e1665_q_d_n17, eq130_e1665_q_d_n18, eq130_e1665_q_d_n19, eq130_e1665_q_d_n20, eq130_e1665_q_d_n21, eq130_e1665_q_d_n22];
        let eq130_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            &nodes,
            &eq130_reactive_node_derivatives,
            &branches,
            &eq130_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_131_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq131_e1677, eq131_e1677_d_n0, eq131_e1677_d_n1, eq131_e1677_d_n2, eq131_e1677_d_n3, eq131_e1677_d_n4, eq131_e1677_d_n5, eq131_e1677_d_n6, eq131_e1677_d_n7, eq131_e1677_d_n8, eq131_e1677_d_n9, eq131_e1677_d_n10, eq131_e1677_d_n11, eq131_e1677_d_n12, eq131_e1677_d_n13, eq131_e1677_d_n14, eq131_e1677_d_n15, eq131_e1677_d_n16, eq131_e1677_d_n17, eq131_e1677_d_n18, eq131_e1677_d_n19, eq131_e1677_d_n20, eq131_e1677_d_n21, eq131_e1677_d_n22, eq131_e1677_q, eq131_e1677_q_d_n0, eq131_e1677_q_d_n1, eq131_e1677_q_d_n2, eq131_e1677_q_d_n3, eq131_e1677_q_d_n4, eq131_e1677_q_d_n5, eq131_e1677_q_d_n6, eq131_e1677_q_d_n7, eq131_e1677_q_d_n8, eq131_e1677_q_d_n9, eq131_e1677_q_d_n10, eq131_e1677_q_d_n11, eq131_e1677_q_d_n12, eq131_e1677_q_d_n13, eq131_e1677_q_d_n14, eq131_e1677_q_d_n15, eq131_e1677_q_d_n16, eq131_e1677_q_d_n17, eq131_e1677_q_d_n18, eq131_e1677_q_d_n19, eq131_e1677_q_d_n20, eq131_e1677_q_d_n21, eq131_e1677_q_d_n22,) = {
    if ((!(s.v[570] != 0.0)) && (s.v[573] != 0.0)) {
        let eq131_e1673: f64 = (p.p251 * s.v[228]);
        let eq131_e1673_d_n0: f64 = (p.p251 * s.dn[228][0]);
        let eq131_e1673_d_n1: f64 = (p.p251 * s.dn[228][1]);
        let eq131_e1673_d_n2: f64 = (p.p251 * s.dn[228][2]);
        let eq131_e1673_d_n3: f64 = (p.p251 * s.dn[228][3]);
        let eq131_e1673_d_n4: f64 = (p.p251 * s.dn[228][4]);
        let eq131_e1673_d_n5: f64 = (p.p251 * s.dn[228][5]);
        let eq131_e1673_d_n6: f64 = (p.p251 * s.dn[228][6]);
        let eq131_e1673_d_n7: f64 = (p.p251 * s.dn[228][7]);
        let eq131_e1673_d_n8: f64 = (p.p251 * s.dn[228][8]);
        let eq131_e1673_d_n9: f64 = (p.p251 * s.dn[228][9]);
        let eq131_e1673_d_n10: f64 = (p.p251 * s.dn[228][10]);
        let eq131_e1673_d_n11: f64 = (p.p251 * s.dn[228][11]);
        let eq131_e1673_d_n12: f64 = (p.p251 * s.dn[228][12]);
        let eq131_e1673_d_n13: f64 = (p.p251 * s.dn[228][13]);
        let eq131_e1673_d_n14: f64 = (p.p251 * s.dn[228][14]);
        let eq131_e1673_d_n15: f64 = (p.p251 * s.dn[228][15]);
        let eq131_e1673_d_n16: f64 = (p.p251 * s.dn[228][16]);
        let eq131_e1673_d_n17: f64 = (p.p251 * s.dn[228][17]);
        let eq131_e1673_d_n18: f64 = (p.p251 * s.dn[228][18]);
        let eq131_e1673_d_n19: f64 = (p.p251 * s.dn[228][19]);
        let eq131_e1673_d_n20: f64 = (p.p251 * s.dn[228][20]);
        let eq131_e1673_d_n21: f64 = (p.p251 * s.dn[228][21]);
        let eq131_e1673_d_n22: f64 = (p.p251 * s.dn[228][22]);
        let eq131_e1674_q: f64 = eq131_e1673;
        let eq131_e1675: f64 = (p.p7 * eq131_e1673);
        let eq131_e1675_d_n0: f64 = (p.p7 * eq131_e1673_d_n0);
        let eq131_e1675_d_n1: f64 = (p.p7 * eq131_e1673_d_n1);
        let eq131_e1675_d_n2: f64 = (p.p7 * eq131_e1673_d_n2);
        let eq131_e1675_d_n3: f64 = (p.p7 * eq131_e1673_d_n3);
        let eq131_e1675_d_n4: f64 = (p.p7 * eq131_e1673_d_n4);
        let eq131_e1675_d_n5: f64 = (p.p7 * eq131_e1673_d_n5);
        let eq131_e1675_d_n6: f64 = (p.p7 * eq131_e1673_d_n6);
        let eq131_e1675_d_n7: f64 = (p.p7 * eq131_e1673_d_n7);
        let eq131_e1675_d_n8: f64 = (p.p7 * eq131_e1673_d_n8);
        let eq131_e1675_d_n9: f64 = (p.p7 * eq131_e1673_d_n9);
        let eq131_e1675_d_n10: f64 = (p.p7 * eq131_e1673_d_n10);
        let eq131_e1675_d_n11: f64 = (p.p7 * eq131_e1673_d_n11);
        let eq131_e1675_d_n12: f64 = (p.p7 * eq131_e1673_d_n12);
        let eq131_e1675_d_n13: f64 = (p.p7 * eq131_e1673_d_n13);
        let eq131_e1675_d_n14: f64 = (p.p7 * eq131_e1673_d_n14);
        let eq131_e1675_d_n15: f64 = (p.p7 * eq131_e1673_d_n15);
        let eq131_e1675_d_n16: f64 = (p.p7 * eq131_e1673_d_n16);
        let eq131_e1675_d_n17: f64 = (p.p7 * eq131_e1673_d_n17);
        let eq131_e1675_d_n18: f64 = (p.p7 * eq131_e1673_d_n18);
        let eq131_e1675_d_n19: f64 = (p.p7 * eq131_e1673_d_n19);
        let eq131_e1675_d_n20: f64 = (p.p7 * eq131_e1673_d_n20);
        let eq131_e1675_d_n21: f64 = (p.p7 * eq131_e1673_d_n21);
        let eq131_e1675_d_n22: f64 = (p.p7 * eq131_e1673_d_n22);
        let eq131_e1675_q: f64 = (p.p7 * eq131_e1674_q);
        let eq131_e1675_q_d_n0: f64 = (p.p7 * eq131_e1673_d_n0);
        let eq131_e1675_q_d_n1: f64 = (p.p7 * eq131_e1673_d_n1);
        let eq131_e1675_q_d_n2: f64 = (p.p7 * eq131_e1673_d_n2);
        let eq131_e1675_q_d_n3: f64 = (p.p7 * eq131_e1673_d_n3);
        let eq131_e1675_q_d_n4: f64 = (p.p7 * eq131_e1673_d_n4);
        let eq131_e1675_q_d_n5: f64 = (p.p7 * eq131_e1673_d_n5);
        let eq131_e1675_q_d_n6: f64 = (p.p7 * eq131_e1673_d_n6);
        let eq131_e1675_q_d_n7: f64 = (p.p7 * eq131_e1673_d_n7);
        let eq131_e1675_q_d_n8: f64 = (p.p7 * eq131_e1673_d_n8);
        let eq131_e1675_q_d_n9: f64 = (p.p7 * eq131_e1673_d_n9);
        let eq131_e1675_q_d_n10: f64 = (p.p7 * eq131_e1673_d_n10);
        let eq131_e1675_q_d_n11: f64 = (p.p7 * eq131_e1673_d_n11);
        let eq131_e1675_q_d_n12: f64 = (p.p7 * eq131_e1673_d_n12);
        let eq131_e1675_q_d_n13: f64 = (p.p7 * eq131_e1673_d_n13);
        let eq131_e1675_q_d_n14: f64 = (p.p7 * eq131_e1673_d_n14);
        let eq131_e1675_q_d_n15: f64 = (p.p7 * eq131_e1673_d_n15);
        let eq131_e1675_q_d_n16: f64 = (p.p7 * eq131_e1673_d_n16);
        let eq131_e1675_q_d_n17: f64 = (p.p7 * eq131_e1673_d_n17);
        let eq131_e1675_q_d_n18: f64 = (p.p7 * eq131_e1673_d_n18);
        let eq131_e1675_q_d_n19: f64 = (p.p7 * eq131_e1673_d_n19);
        let eq131_e1675_q_d_n20: f64 = (p.p7 * eq131_e1673_d_n20);
        let eq131_e1675_q_d_n21: f64 = (p.p7 * eq131_e1673_d_n21);
        let eq131_e1675_q_d_n22: f64 = (p.p7 * eq131_e1673_d_n22);
        (eq131_e1675, eq131_e1675_d_n0, eq131_e1675_d_n1, eq131_e1675_d_n2, eq131_e1675_d_n3, eq131_e1675_d_n4, eq131_e1675_d_n5, eq131_e1675_d_n6, eq131_e1675_d_n7, eq131_e1675_d_n8, eq131_e1675_d_n9, eq131_e1675_d_n10, eq131_e1675_d_n11, eq131_e1675_d_n12, eq131_e1675_d_n13, eq131_e1675_d_n14, eq131_e1675_d_n15, eq131_e1675_d_n16, eq131_e1675_d_n17, eq131_e1675_d_n18, eq131_e1675_d_n19, eq131_e1675_d_n20, eq131_e1675_d_n21, eq131_e1675_d_n22, eq131_e1675_q, eq131_e1675_q_d_n0, eq131_e1675_q_d_n1, eq131_e1675_q_d_n2, eq131_e1675_q_d_n3, eq131_e1675_q_d_n4, eq131_e1675_q_d_n5, eq131_e1675_q_d_n6, eq131_e1675_q_d_n7, eq131_e1675_q_d_n8, eq131_e1675_q_d_n9, eq131_e1675_q_d_n10, eq131_e1675_q_d_n11, eq131_e1675_q_d_n12, eq131_e1675_q_d_n13, eq131_e1675_q_d_n14, eq131_e1675_q_d_n15, eq131_e1675_q_d_n16, eq131_e1675_q_d_n17, eq131_e1675_q_d_n18, eq131_e1675_q_d_n19, eq131_e1675_q_d_n20, eq131_e1675_q_d_n21, eq131_e1675_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq131_reactive_node_derivatives: [f64; 23] = [eq131_e1677_q_d_n0, eq131_e1677_q_d_n1, eq131_e1677_q_d_n2, eq131_e1677_q_d_n3, eq131_e1677_q_d_n4, eq131_e1677_q_d_n5, eq131_e1677_q_d_n6, eq131_e1677_q_d_n7, eq131_e1677_q_d_n8, eq131_e1677_q_d_n9, eq131_e1677_q_d_n10, eq131_e1677_q_d_n11, eq131_e1677_q_d_n12, eq131_e1677_q_d_n13, eq131_e1677_q_d_n14, eq131_e1677_q_d_n15, eq131_e1677_q_d_n16, eq131_e1677_q_d_n17, eq131_e1677_q_d_n18, eq131_e1677_q_d_n19, eq131_e1677_q_d_n20, eq131_e1677_q_d_n21, eq131_e1677_q_d_n22];
        let eq131_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            &nodes,
            &eq131_reactive_node_derivatives,
            &branches,
            &eq131_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_132_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq132_e1686, eq132_e1686_d_n0, eq132_e1686_d_n1, eq132_e1686_d_n2, eq132_e1686_d_n3, eq132_e1686_d_n4, eq132_e1686_d_n5, eq132_e1686_d_n6, eq132_e1686_d_n7, eq132_e1686_d_n8, eq132_e1686_d_n9, eq132_e1686_d_n10, eq132_e1686_d_n11, eq132_e1686_d_n12, eq132_e1686_d_n13, eq132_e1686_d_n14, eq132_e1686_d_n15, eq132_e1686_d_n16, eq132_e1686_d_n17, eq132_e1686_d_n18, eq132_e1686_d_n19, eq132_e1686_d_n20, eq132_e1686_d_n21, eq132_e1686_d_n22, eq132_e1686_q, eq132_e1686_q_d_n0, eq132_e1686_q_d_n1, eq132_e1686_q_d_n2, eq132_e1686_q_d_n3, eq132_e1686_q_d_n4, eq132_e1686_q_d_n5, eq132_e1686_q_d_n6, eq132_e1686_q_d_n7, eq132_e1686_q_d_n8, eq132_e1686_q_d_n9, eq132_e1686_q_d_n10, eq132_e1686_q_d_n11, eq132_e1686_q_d_n12, eq132_e1686_q_d_n13, eq132_e1686_q_d_n14, eq132_e1686_q_d_n15, eq132_e1686_q_d_n16, eq132_e1686_q_d_n17, eq132_e1686_q_d_n18, eq132_e1686_q_d_n19, eq132_e1686_q_d_n20, eq132_e1686_q_d_n21, eq132_e1686_q_d_n22,) = {
    if ((s.v[575] != 0.0) && (s.v[576] != 0.0)) {
        let eq132_e1683_q: f64 = s.v[241];
        let eq132_e1684: f64 = (p.p7 * s.v[241]);
        let eq132_e1684_d_n0: f64 = (p.p7 * s.dn[241][0]);
        let eq132_e1684_d_n1: f64 = (p.p7 * s.dn[241][1]);
        let eq132_e1684_d_n2: f64 = (p.p7 * s.dn[241][2]);
        let eq132_e1684_d_n3: f64 = (p.p7 * s.dn[241][3]);
        let eq132_e1684_d_n4: f64 = (p.p7 * s.dn[241][4]);
        let eq132_e1684_d_n5: f64 = (p.p7 * s.dn[241][5]);
        let eq132_e1684_d_n6: f64 = (p.p7 * s.dn[241][6]);
        let eq132_e1684_d_n7: f64 = (p.p7 * s.dn[241][7]);
        let eq132_e1684_d_n8: f64 = (p.p7 * s.dn[241][8]);
        let eq132_e1684_d_n9: f64 = (p.p7 * s.dn[241][9]);
        let eq132_e1684_d_n10: f64 = (p.p7 * s.dn[241][10]);
        let eq132_e1684_d_n11: f64 = (p.p7 * s.dn[241][11]);
        let eq132_e1684_d_n12: f64 = (p.p7 * s.dn[241][12]);
        let eq132_e1684_d_n13: f64 = (p.p7 * s.dn[241][13]);
        let eq132_e1684_d_n14: f64 = (p.p7 * s.dn[241][14]);
        let eq132_e1684_d_n15: f64 = (p.p7 * s.dn[241][15]);
        let eq132_e1684_d_n16: f64 = (p.p7 * s.dn[241][16]);
        let eq132_e1684_d_n17: f64 = (p.p7 * s.dn[241][17]);
        let eq132_e1684_d_n18: f64 = (p.p7 * s.dn[241][18]);
        let eq132_e1684_d_n19: f64 = (p.p7 * s.dn[241][19]);
        let eq132_e1684_d_n20: f64 = (p.p7 * s.dn[241][20]);
        let eq132_e1684_d_n21: f64 = (p.p7 * s.dn[241][21]);
        let eq132_e1684_d_n22: f64 = (p.p7 * s.dn[241][22]);
        let eq132_e1684_q: f64 = (p.p7 * eq132_e1683_q);
        let eq132_e1684_q_d_n0: f64 = (p.p7 * s.dn[241][0]);
        let eq132_e1684_q_d_n1: f64 = (p.p7 * s.dn[241][1]);
        let eq132_e1684_q_d_n2: f64 = (p.p7 * s.dn[241][2]);
        let eq132_e1684_q_d_n3: f64 = (p.p7 * s.dn[241][3]);
        let eq132_e1684_q_d_n4: f64 = (p.p7 * s.dn[241][4]);
        let eq132_e1684_q_d_n5: f64 = (p.p7 * s.dn[241][5]);
        let eq132_e1684_q_d_n6: f64 = (p.p7 * s.dn[241][6]);
        let eq132_e1684_q_d_n7: f64 = (p.p7 * s.dn[241][7]);
        let eq132_e1684_q_d_n8: f64 = (p.p7 * s.dn[241][8]);
        let eq132_e1684_q_d_n9: f64 = (p.p7 * s.dn[241][9]);
        let eq132_e1684_q_d_n10: f64 = (p.p7 * s.dn[241][10]);
        let eq132_e1684_q_d_n11: f64 = (p.p7 * s.dn[241][11]);
        let eq132_e1684_q_d_n12: f64 = (p.p7 * s.dn[241][12]);
        let eq132_e1684_q_d_n13: f64 = (p.p7 * s.dn[241][13]);
        let eq132_e1684_q_d_n14: f64 = (p.p7 * s.dn[241][14]);
        let eq132_e1684_q_d_n15: f64 = (p.p7 * s.dn[241][15]);
        let eq132_e1684_q_d_n16: f64 = (p.p7 * s.dn[241][16]);
        let eq132_e1684_q_d_n17: f64 = (p.p7 * s.dn[241][17]);
        let eq132_e1684_q_d_n18: f64 = (p.p7 * s.dn[241][18]);
        let eq132_e1684_q_d_n19: f64 = (p.p7 * s.dn[241][19]);
        let eq132_e1684_q_d_n20: f64 = (p.p7 * s.dn[241][20]);
        let eq132_e1684_q_d_n21: f64 = (p.p7 * s.dn[241][21]);
        let eq132_e1684_q_d_n22: f64 = (p.p7 * s.dn[241][22]);
        (eq132_e1684, eq132_e1684_d_n0, eq132_e1684_d_n1, eq132_e1684_d_n2, eq132_e1684_d_n3, eq132_e1684_d_n4, eq132_e1684_d_n5, eq132_e1684_d_n6, eq132_e1684_d_n7, eq132_e1684_d_n8, eq132_e1684_d_n9, eq132_e1684_d_n10, eq132_e1684_d_n11, eq132_e1684_d_n12, eq132_e1684_d_n13, eq132_e1684_d_n14, eq132_e1684_d_n15, eq132_e1684_d_n16, eq132_e1684_d_n17, eq132_e1684_d_n18, eq132_e1684_d_n19, eq132_e1684_d_n20, eq132_e1684_d_n21, eq132_e1684_d_n22, eq132_e1684_q, eq132_e1684_q_d_n0, eq132_e1684_q_d_n1, eq132_e1684_q_d_n2, eq132_e1684_q_d_n3, eq132_e1684_q_d_n4, eq132_e1684_q_d_n5, eq132_e1684_q_d_n6, eq132_e1684_q_d_n7, eq132_e1684_q_d_n8, eq132_e1684_q_d_n9, eq132_e1684_q_d_n10, eq132_e1684_q_d_n11, eq132_e1684_q_d_n12, eq132_e1684_q_d_n13, eq132_e1684_q_d_n14, eq132_e1684_q_d_n15, eq132_e1684_q_d_n16, eq132_e1684_q_d_n17, eq132_e1684_q_d_n18, eq132_e1684_q_d_n19, eq132_e1684_q_d_n20, eq132_e1684_q_d_n21, eq132_e1684_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq132_reactive_node_derivatives: [f64; 23] = [eq132_e1686_q_d_n0, eq132_e1686_q_d_n1, eq132_e1686_q_d_n2, eq132_e1686_q_d_n3, eq132_e1686_q_d_n4, eq132_e1686_q_d_n5, eq132_e1686_q_d_n6, eq132_e1686_q_d_n7, eq132_e1686_q_d_n8, eq132_e1686_q_d_n9, eq132_e1686_q_d_n10, eq132_e1686_q_d_n11, eq132_e1686_q_d_n12, eq132_e1686_q_d_n13, eq132_e1686_q_d_n14, eq132_e1686_q_d_n15, eq132_e1686_q_d_n16, eq132_e1686_q_d_n17, eq132_e1686_q_d_n18, eq132_e1686_q_d_n19, eq132_e1686_q_d_n20, eq132_e1686_q_d_n21, eq132_e1686_q_d_n22];
        let eq132_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[19]),
            &nodes,
            &eq132_reactive_node_derivatives,
            &branches,
            &eq132_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_133_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq133_e1697, eq133_e1697_d_n0, eq133_e1697_d_n1, eq133_e1697_d_n2, eq133_e1697_d_n3, eq133_e1697_d_n4, eq133_e1697_d_n5, eq133_e1697_d_n6, eq133_e1697_d_n7, eq133_e1697_d_n8, eq133_e1697_d_n9, eq133_e1697_d_n10, eq133_e1697_d_n11, eq133_e1697_d_n12, eq133_e1697_d_n13, eq133_e1697_d_n14, eq133_e1697_d_n15, eq133_e1697_d_n16, eq133_e1697_d_n17, eq133_e1697_d_n18, eq133_e1697_d_n19, eq133_e1697_d_n20, eq133_e1697_d_n21, eq133_e1697_d_n22, eq133_e1697_q, eq133_e1697_q_d_n0, eq133_e1697_q_d_n1, eq133_e1697_q_d_n2, eq133_e1697_q_d_n3, eq133_e1697_q_d_n4, eq133_e1697_q_d_n5, eq133_e1697_q_d_n6, eq133_e1697_q_d_n7, eq133_e1697_q_d_n8, eq133_e1697_q_d_n9, eq133_e1697_q_d_n10, eq133_e1697_q_d_n11, eq133_e1697_q_d_n12, eq133_e1697_q_d_n13, eq133_e1697_q_d_n14, eq133_e1697_q_d_n15, eq133_e1697_q_d_n16, eq133_e1697_q_d_n17, eq133_e1697_q_d_n18, eq133_e1697_q_d_n19, eq133_e1697_q_d_n20, eq133_e1697_q_d_n21, eq133_e1697_q_d_n22,) = {
    if (((s.v[575] != 0.0) && (s.v[576] != 0.0)) && (s.v[577] != 0.0)) {
        let eq133_e1694_q: f64 = s.v[240];
        let eq133_e1695: f64 = (p.p7 * s.v[240]);
        let eq133_e1695_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq133_e1695_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq133_e1695_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq133_e1695_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq133_e1695_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq133_e1695_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq133_e1695_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq133_e1695_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq133_e1695_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq133_e1695_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq133_e1695_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq133_e1695_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq133_e1695_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq133_e1695_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq133_e1695_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq133_e1695_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq133_e1695_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq133_e1695_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq133_e1695_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq133_e1695_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq133_e1695_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq133_e1695_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq133_e1695_d_n22: f64 = (p.p7 * s.dn[240][22]);
        let eq133_e1695_q: f64 = (p.p7 * eq133_e1694_q);
        let eq133_e1695_q_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq133_e1695_q_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq133_e1695_q_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq133_e1695_q_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq133_e1695_q_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq133_e1695_q_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq133_e1695_q_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq133_e1695_q_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq133_e1695_q_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq133_e1695_q_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq133_e1695_q_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq133_e1695_q_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq133_e1695_q_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq133_e1695_q_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq133_e1695_q_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq133_e1695_q_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq133_e1695_q_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq133_e1695_q_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq133_e1695_q_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq133_e1695_q_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq133_e1695_q_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq133_e1695_q_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq133_e1695_q_d_n22: f64 = (p.p7 * s.dn[240][22]);
        (eq133_e1695, eq133_e1695_d_n0, eq133_e1695_d_n1, eq133_e1695_d_n2, eq133_e1695_d_n3, eq133_e1695_d_n4, eq133_e1695_d_n5, eq133_e1695_d_n6, eq133_e1695_d_n7, eq133_e1695_d_n8, eq133_e1695_d_n9, eq133_e1695_d_n10, eq133_e1695_d_n11, eq133_e1695_d_n12, eq133_e1695_d_n13, eq133_e1695_d_n14, eq133_e1695_d_n15, eq133_e1695_d_n16, eq133_e1695_d_n17, eq133_e1695_d_n18, eq133_e1695_d_n19, eq133_e1695_d_n20, eq133_e1695_d_n21, eq133_e1695_d_n22, eq133_e1695_q, eq133_e1695_q_d_n0, eq133_e1695_q_d_n1, eq133_e1695_q_d_n2, eq133_e1695_q_d_n3, eq133_e1695_q_d_n4, eq133_e1695_q_d_n5, eq133_e1695_q_d_n6, eq133_e1695_q_d_n7, eq133_e1695_q_d_n8, eq133_e1695_q_d_n9, eq133_e1695_q_d_n10, eq133_e1695_q_d_n11, eq133_e1695_q_d_n12, eq133_e1695_q_d_n13, eq133_e1695_q_d_n14, eq133_e1695_q_d_n15, eq133_e1695_q_d_n16, eq133_e1695_q_d_n17, eq133_e1695_q_d_n18, eq133_e1695_q_d_n19, eq133_e1695_q_d_n20, eq133_e1695_q_d_n21, eq133_e1695_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq133_reactive_node_derivatives: [f64; 23] = [eq133_e1697_q_d_n0, eq133_e1697_q_d_n1, eq133_e1697_q_d_n2, eq133_e1697_q_d_n3, eq133_e1697_q_d_n4, eq133_e1697_q_d_n5, eq133_e1697_q_d_n6, eq133_e1697_q_d_n7, eq133_e1697_q_d_n8, eq133_e1697_q_d_n9, eq133_e1697_q_d_n10, eq133_e1697_q_d_n11, eq133_e1697_q_d_n12, eq133_e1697_q_d_n13, eq133_e1697_q_d_n14, eq133_e1697_q_d_n15, eq133_e1697_q_d_n16, eq133_e1697_q_d_n17, eq133_e1697_q_d_n18, eq133_e1697_q_d_n19, eq133_e1697_q_d_n20, eq133_e1697_q_d_n21, eq133_e1697_q_d_n22];
        let eq133_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[19]),
            &nodes,
            &eq133_reactive_node_derivatives,
            &branches,
            &eq133_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_134_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq134_e1710, eq134_e1710_d_n0, eq134_e1710_d_n1, eq134_e1710_d_n2, eq134_e1710_d_n3, eq134_e1710_d_n4, eq134_e1710_d_n5, eq134_e1710_d_n6, eq134_e1710_d_n7, eq134_e1710_d_n8, eq134_e1710_d_n9, eq134_e1710_d_n10, eq134_e1710_d_n11, eq134_e1710_d_n12, eq134_e1710_d_n13, eq134_e1710_d_n14, eq134_e1710_d_n15, eq134_e1710_d_n16, eq134_e1710_d_n17, eq134_e1710_d_n18, eq134_e1710_d_n19, eq134_e1710_d_n20, eq134_e1710_d_n21, eq134_e1710_d_n22, eq134_e1710_q, eq134_e1710_q_d_n0, eq134_e1710_q_d_n1, eq134_e1710_q_d_n2, eq134_e1710_q_d_n3, eq134_e1710_q_d_n4, eq134_e1710_q_d_n5, eq134_e1710_q_d_n6, eq134_e1710_q_d_n7, eq134_e1710_q_d_n8, eq134_e1710_q_d_n9, eq134_e1710_q_d_n10, eq134_e1710_q_d_n11, eq134_e1710_q_d_n12, eq134_e1710_q_d_n13, eq134_e1710_q_d_n14, eq134_e1710_q_d_n15, eq134_e1710_q_d_n16, eq134_e1710_q_d_n17, eq134_e1710_q_d_n18, eq134_e1710_q_d_n19, eq134_e1710_q_d_n20, eq134_e1710_q_d_n21, eq134_e1710_q_d_n22,) = {
    if (((s.v[575] != 0.0) && (s.v[576] != 0.0)) && (s.v[577] != 0.0)) {
        let eq134_e1705_q: f64 = s.v[240];
        let eq134_e1706: f64 = (p.p7 * s.v[240]);
        let eq134_e1706_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq134_e1706_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq134_e1706_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq134_e1706_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq134_e1706_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq134_e1706_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq134_e1706_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq134_e1706_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq134_e1706_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq134_e1706_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq134_e1706_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq134_e1706_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq134_e1706_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq134_e1706_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq134_e1706_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq134_e1706_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq134_e1706_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq134_e1706_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq134_e1706_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq134_e1706_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq134_e1706_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq134_e1706_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq134_e1706_d_n22: f64 = (p.p7 * s.dn[240][22]);
        let eq134_e1706_q: f64 = (p.p7 * eq134_e1705_q);
        let eq134_e1706_q_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq134_e1706_q_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq134_e1706_q_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq134_e1706_q_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq134_e1706_q_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq134_e1706_q_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq134_e1706_q_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq134_e1706_q_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq134_e1706_q_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq134_e1706_q_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq134_e1706_q_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq134_e1706_q_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq134_e1706_q_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq134_e1706_q_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq134_e1706_q_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq134_e1706_q_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq134_e1706_q_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq134_e1706_q_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq134_e1706_q_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq134_e1706_q_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq134_e1706_q_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq134_e1706_q_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq134_e1706_q_d_n22: f64 = (p.p7 * s.dn[240][22]);
        let eq134_e1708: f64 = (eq134_e1706 * p.p246);
        let eq134_e1708_d_n0: f64 = (eq134_e1706_d_n0 * p.p246);
        let eq134_e1708_d_n1: f64 = (eq134_e1706_d_n1 * p.p246);
        let eq134_e1708_d_n2: f64 = (eq134_e1706_d_n2 * p.p246);
        let eq134_e1708_d_n3: f64 = (eq134_e1706_d_n3 * p.p246);
        let eq134_e1708_d_n4: f64 = (eq134_e1706_d_n4 * p.p246);
        let eq134_e1708_d_n5: f64 = (eq134_e1706_d_n5 * p.p246);
        let eq134_e1708_d_n6: f64 = (eq134_e1706_d_n6 * p.p246);
        let eq134_e1708_d_n7: f64 = (eq134_e1706_d_n7 * p.p246);
        let eq134_e1708_d_n8: f64 = (eq134_e1706_d_n8 * p.p246);
        let eq134_e1708_d_n9: f64 = (eq134_e1706_d_n9 * p.p246);
        let eq134_e1708_d_n10: f64 = (eq134_e1706_d_n10 * p.p246);
        let eq134_e1708_d_n11: f64 = (eq134_e1706_d_n11 * p.p246);
        let eq134_e1708_d_n12: f64 = (eq134_e1706_d_n12 * p.p246);
        let eq134_e1708_d_n13: f64 = (eq134_e1706_d_n13 * p.p246);
        let eq134_e1708_d_n14: f64 = (eq134_e1706_d_n14 * p.p246);
        let eq134_e1708_d_n15: f64 = (eq134_e1706_d_n15 * p.p246);
        let eq134_e1708_d_n16: f64 = (eq134_e1706_d_n16 * p.p246);
        let eq134_e1708_d_n17: f64 = (eq134_e1706_d_n17 * p.p246);
        let eq134_e1708_d_n18: f64 = (eq134_e1706_d_n18 * p.p246);
        let eq134_e1708_d_n19: f64 = (eq134_e1706_d_n19 * p.p246);
        let eq134_e1708_d_n20: f64 = (eq134_e1706_d_n20 * p.p246);
        let eq134_e1708_d_n21: f64 = (eq134_e1706_d_n21 * p.p246);
        let eq134_e1708_d_n22: f64 = (eq134_e1706_d_n22 * p.p246);
        let eq134_e1708_q: f64 = (eq134_e1706_q * p.p246);
        let eq134_e1708_q_d_n0: f64 = (eq134_e1706_q_d_n0 * p.p246);
        let eq134_e1708_q_d_n1: f64 = (eq134_e1706_q_d_n1 * p.p246);
        let eq134_e1708_q_d_n2: f64 = (eq134_e1706_q_d_n2 * p.p246);
        let eq134_e1708_q_d_n3: f64 = (eq134_e1706_q_d_n3 * p.p246);
        let eq134_e1708_q_d_n4: f64 = (eq134_e1706_q_d_n4 * p.p246);
        let eq134_e1708_q_d_n5: f64 = (eq134_e1706_q_d_n5 * p.p246);
        let eq134_e1708_q_d_n6: f64 = (eq134_e1706_q_d_n6 * p.p246);
        let eq134_e1708_q_d_n7: f64 = (eq134_e1706_q_d_n7 * p.p246);
        let eq134_e1708_q_d_n8: f64 = (eq134_e1706_q_d_n8 * p.p246);
        let eq134_e1708_q_d_n9: f64 = (eq134_e1706_q_d_n9 * p.p246);
        let eq134_e1708_q_d_n10: f64 = (eq134_e1706_q_d_n10 * p.p246);
        let eq134_e1708_q_d_n11: f64 = (eq134_e1706_q_d_n11 * p.p246);
        let eq134_e1708_q_d_n12: f64 = (eq134_e1706_q_d_n12 * p.p246);
        let eq134_e1708_q_d_n13: f64 = (eq134_e1706_q_d_n13 * p.p246);
        let eq134_e1708_q_d_n14: f64 = (eq134_e1706_q_d_n14 * p.p246);
        let eq134_e1708_q_d_n15: f64 = (eq134_e1706_q_d_n15 * p.p246);
        let eq134_e1708_q_d_n16: f64 = (eq134_e1706_q_d_n16 * p.p246);
        let eq134_e1708_q_d_n17: f64 = (eq134_e1706_q_d_n17 * p.p246);
        let eq134_e1708_q_d_n18: f64 = (eq134_e1706_q_d_n18 * p.p246);
        let eq134_e1708_q_d_n19: f64 = (eq134_e1706_q_d_n19 * p.p246);
        let eq134_e1708_q_d_n20: f64 = (eq134_e1706_q_d_n20 * p.p246);
        let eq134_e1708_q_d_n21: f64 = (eq134_e1706_q_d_n21 * p.p246);
        let eq134_e1708_q_d_n22: f64 = (eq134_e1706_q_d_n22 * p.p246);
        (eq134_e1708, eq134_e1708_d_n0, eq134_e1708_d_n1, eq134_e1708_d_n2, eq134_e1708_d_n3, eq134_e1708_d_n4, eq134_e1708_d_n5, eq134_e1708_d_n6, eq134_e1708_d_n7, eq134_e1708_d_n8, eq134_e1708_d_n9, eq134_e1708_d_n10, eq134_e1708_d_n11, eq134_e1708_d_n12, eq134_e1708_d_n13, eq134_e1708_d_n14, eq134_e1708_d_n15, eq134_e1708_d_n16, eq134_e1708_d_n17, eq134_e1708_d_n18, eq134_e1708_d_n19, eq134_e1708_d_n20, eq134_e1708_d_n21, eq134_e1708_d_n22, eq134_e1708_q, eq134_e1708_q_d_n0, eq134_e1708_q_d_n1, eq134_e1708_q_d_n2, eq134_e1708_q_d_n3, eq134_e1708_q_d_n4, eq134_e1708_q_d_n5, eq134_e1708_q_d_n6, eq134_e1708_q_d_n7, eq134_e1708_q_d_n8, eq134_e1708_q_d_n9, eq134_e1708_q_d_n10, eq134_e1708_q_d_n11, eq134_e1708_q_d_n12, eq134_e1708_q_d_n13, eq134_e1708_q_d_n14, eq134_e1708_q_d_n15, eq134_e1708_q_d_n16, eq134_e1708_q_d_n17, eq134_e1708_q_d_n18, eq134_e1708_q_d_n19, eq134_e1708_q_d_n20, eq134_e1708_q_d_n21, eq134_e1708_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq134_reactive_node_derivatives: [f64; 23] = [eq134_e1710_q_d_n0, eq134_e1710_q_d_n1, eq134_e1710_q_d_n2, eq134_e1710_q_d_n3, eq134_e1710_q_d_n4, eq134_e1710_q_d_n5, eq134_e1710_q_d_n6, eq134_e1710_q_d_n7, eq134_e1710_q_d_n8, eq134_e1710_q_d_n9, eq134_e1710_q_d_n10, eq134_e1710_q_d_n11, eq134_e1710_q_d_n12, eq134_e1710_q_d_n13, eq134_e1710_q_d_n14, eq134_e1710_q_d_n15, eq134_e1710_q_d_n16, eq134_e1710_q_d_n17, eq134_e1710_q_d_n18, eq134_e1710_q_d_n19, eq134_e1710_q_d_n20, eq134_e1710_q_d_n21, eq134_e1710_q_d_n22];
        let eq134_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[19]),
            &nodes,
            &eq134_reactive_node_derivatives,
            &branches,
            &eq134_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_135_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq135_e1722, eq135_e1722_d_n0, eq135_e1722_d_n1, eq135_e1722_d_n2, eq135_e1722_d_n3, eq135_e1722_d_n4, eq135_e1722_d_n5, eq135_e1722_d_n6, eq135_e1722_d_n7, eq135_e1722_d_n8, eq135_e1722_d_n9, eq135_e1722_d_n10, eq135_e1722_d_n11, eq135_e1722_d_n12, eq135_e1722_d_n13, eq135_e1722_d_n14, eq135_e1722_d_n15, eq135_e1722_d_n16, eq135_e1722_d_n17, eq135_e1722_d_n18, eq135_e1722_d_n19, eq135_e1722_d_n20, eq135_e1722_d_n21, eq135_e1722_d_n22, eq135_e1722_q, eq135_e1722_q_d_n0, eq135_e1722_q_d_n1, eq135_e1722_q_d_n2, eq135_e1722_q_d_n3, eq135_e1722_q_d_n4, eq135_e1722_q_d_n5, eq135_e1722_q_d_n6, eq135_e1722_q_d_n7, eq135_e1722_q_d_n8, eq135_e1722_q_d_n9, eq135_e1722_q_d_n10, eq135_e1722_q_d_n11, eq135_e1722_q_d_n12, eq135_e1722_q_d_n13, eq135_e1722_q_d_n14, eq135_e1722_q_d_n15, eq135_e1722_q_d_n16, eq135_e1722_q_d_n17, eq135_e1722_q_d_n18, eq135_e1722_q_d_n19, eq135_e1722_q_d_n20, eq135_e1722_q_d_n21, eq135_e1722_q_d_n22,) = {
    if (((s.v[575] != 0.0) && (s.v[576] != 0.0)) && (!(s.v[577] != 0.0))) {
        let eq135_e1719_q: f64 = s.v[240];
        let eq135_e1720: f64 = (p.p7 * s.v[240]);
        let eq135_e1720_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq135_e1720_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq135_e1720_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq135_e1720_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq135_e1720_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq135_e1720_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq135_e1720_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq135_e1720_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq135_e1720_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq135_e1720_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq135_e1720_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq135_e1720_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq135_e1720_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq135_e1720_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq135_e1720_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq135_e1720_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq135_e1720_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq135_e1720_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq135_e1720_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq135_e1720_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq135_e1720_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq135_e1720_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq135_e1720_d_n22: f64 = (p.p7 * s.dn[240][22]);
        let eq135_e1720_q: f64 = (p.p7 * eq135_e1719_q);
        let eq135_e1720_q_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq135_e1720_q_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq135_e1720_q_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq135_e1720_q_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq135_e1720_q_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq135_e1720_q_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq135_e1720_q_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq135_e1720_q_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq135_e1720_q_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq135_e1720_q_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq135_e1720_q_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq135_e1720_q_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq135_e1720_q_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq135_e1720_q_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq135_e1720_q_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq135_e1720_q_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq135_e1720_q_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq135_e1720_q_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq135_e1720_q_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq135_e1720_q_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq135_e1720_q_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq135_e1720_q_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq135_e1720_q_d_n22: f64 = (p.p7 * s.dn[240][22]);
        (eq135_e1720, eq135_e1720_d_n0, eq135_e1720_d_n1, eq135_e1720_d_n2, eq135_e1720_d_n3, eq135_e1720_d_n4, eq135_e1720_d_n5, eq135_e1720_d_n6, eq135_e1720_d_n7, eq135_e1720_d_n8, eq135_e1720_d_n9, eq135_e1720_d_n10, eq135_e1720_d_n11, eq135_e1720_d_n12, eq135_e1720_d_n13, eq135_e1720_d_n14, eq135_e1720_d_n15, eq135_e1720_d_n16, eq135_e1720_d_n17, eq135_e1720_d_n18, eq135_e1720_d_n19, eq135_e1720_d_n20, eq135_e1720_d_n21, eq135_e1720_d_n22, eq135_e1720_q, eq135_e1720_q_d_n0, eq135_e1720_q_d_n1, eq135_e1720_q_d_n2, eq135_e1720_q_d_n3, eq135_e1720_q_d_n4, eq135_e1720_q_d_n5, eq135_e1720_q_d_n6, eq135_e1720_q_d_n7, eq135_e1720_q_d_n8, eq135_e1720_q_d_n9, eq135_e1720_q_d_n10, eq135_e1720_q_d_n11, eq135_e1720_q_d_n12, eq135_e1720_q_d_n13, eq135_e1720_q_d_n14, eq135_e1720_q_d_n15, eq135_e1720_q_d_n16, eq135_e1720_q_d_n17, eq135_e1720_q_d_n18, eq135_e1720_q_d_n19, eq135_e1720_q_d_n20, eq135_e1720_q_d_n21, eq135_e1720_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq135_reactive_node_derivatives: [f64; 23] = [eq135_e1722_q_d_n0, eq135_e1722_q_d_n1, eq135_e1722_q_d_n2, eq135_e1722_q_d_n3, eq135_e1722_q_d_n4, eq135_e1722_q_d_n5, eq135_e1722_q_d_n6, eq135_e1722_q_d_n7, eq135_e1722_q_d_n8, eq135_e1722_q_d_n9, eq135_e1722_q_d_n10, eq135_e1722_q_d_n11, eq135_e1722_q_d_n12, eq135_e1722_q_d_n13, eq135_e1722_q_d_n14, eq135_e1722_q_d_n15, eq135_e1722_q_d_n16, eq135_e1722_q_d_n17, eq135_e1722_q_d_n18, eq135_e1722_q_d_n19, eq135_e1722_q_d_n20, eq135_e1722_q_d_n21, eq135_e1722_q_d_n22];
        let eq135_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[19]),
            &nodes,
            &eq135_reactive_node_derivatives,
            &branches,
            &eq135_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_136_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq136_e1736, eq136_e1736_d_n0, eq136_e1736_d_n1, eq136_e1736_d_n2, eq136_e1736_d_n3, eq136_e1736_d_n4, eq136_e1736_d_n5, eq136_e1736_d_n6, eq136_e1736_d_n7, eq136_e1736_d_n8, eq136_e1736_d_n9, eq136_e1736_d_n10, eq136_e1736_d_n11, eq136_e1736_d_n12, eq136_e1736_d_n13, eq136_e1736_d_n14, eq136_e1736_d_n15, eq136_e1736_d_n16, eq136_e1736_d_n17, eq136_e1736_d_n18, eq136_e1736_d_n19, eq136_e1736_d_n20, eq136_e1736_d_n21, eq136_e1736_d_n22, eq136_e1736_q, eq136_e1736_q_d_n0, eq136_e1736_q_d_n1, eq136_e1736_q_d_n2, eq136_e1736_q_d_n3, eq136_e1736_q_d_n4, eq136_e1736_q_d_n5, eq136_e1736_q_d_n6, eq136_e1736_q_d_n7, eq136_e1736_q_d_n8, eq136_e1736_q_d_n9, eq136_e1736_q_d_n10, eq136_e1736_q_d_n11, eq136_e1736_q_d_n12, eq136_e1736_q_d_n13, eq136_e1736_q_d_n14, eq136_e1736_q_d_n15, eq136_e1736_q_d_n16, eq136_e1736_q_d_n17, eq136_e1736_q_d_n18, eq136_e1736_q_d_n19, eq136_e1736_q_d_n20, eq136_e1736_q_d_n21, eq136_e1736_q_d_n22,) = {
    if (((s.v[575] != 0.0) && (s.v[576] != 0.0)) && (!(s.v[577] != 0.0))) {
        let eq136_e1731_q: f64 = s.v[240];
        let eq136_e1732: f64 = (p.p7 * s.v[240]);
        let eq136_e1732_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq136_e1732_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq136_e1732_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq136_e1732_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq136_e1732_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq136_e1732_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq136_e1732_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq136_e1732_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq136_e1732_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq136_e1732_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq136_e1732_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq136_e1732_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq136_e1732_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq136_e1732_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq136_e1732_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq136_e1732_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq136_e1732_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq136_e1732_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq136_e1732_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq136_e1732_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq136_e1732_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq136_e1732_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq136_e1732_d_n22: f64 = (p.p7 * s.dn[240][22]);
        let eq136_e1732_q: f64 = (p.p7 * eq136_e1731_q);
        let eq136_e1732_q_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq136_e1732_q_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq136_e1732_q_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq136_e1732_q_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq136_e1732_q_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq136_e1732_q_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq136_e1732_q_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq136_e1732_q_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq136_e1732_q_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq136_e1732_q_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq136_e1732_q_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq136_e1732_q_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq136_e1732_q_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq136_e1732_q_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq136_e1732_q_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq136_e1732_q_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq136_e1732_q_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq136_e1732_q_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq136_e1732_q_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq136_e1732_q_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq136_e1732_q_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq136_e1732_q_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq136_e1732_q_d_n22: f64 = (p.p7 * s.dn[240][22]);
        let eq136_e1734: f64 = (eq136_e1732 * p.p246);
        let eq136_e1734_d_n0: f64 = (eq136_e1732_d_n0 * p.p246);
        let eq136_e1734_d_n1: f64 = (eq136_e1732_d_n1 * p.p246);
        let eq136_e1734_d_n2: f64 = (eq136_e1732_d_n2 * p.p246);
        let eq136_e1734_d_n3: f64 = (eq136_e1732_d_n3 * p.p246);
        let eq136_e1734_d_n4: f64 = (eq136_e1732_d_n4 * p.p246);
        let eq136_e1734_d_n5: f64 = (eq136_e1732_d_n5 * p.p246);
        let eq136_e1734_d_n6: f64 = (eq136_e1732_d_n6 * p.p246);
        let eq136_e1734_d_n7: f64 = (eq136_e1732_d_n7 * p.p246);
        let eq136_e1734_d_n8: f64 = (eq136_e1732_d_n8 * p.p246);
        let eq136_e1734_d_n9: f64 = (eq136_e1732_d_n9 * p.p246);
        let eq136_e1734_d_n10: f64 = (eq136_e1732_d_n10 * p.p246);
        let eq136_e1734_d_n11: f64 = (eq136_e1732_d_n11 * p.p246);
        let eq136_e1734_d_n12: f64 = (eq136_e1732_d_n12 * p.p246);
        let eq136_e1734_d_n13: f64 = (eq136_e1732_d_n13 * p.p246);
        let eq136_e1734_d_n14: f64 = (eq136_e1732_d_n14 * p.p246);
        let eq136_e1734_d_n15: f64 = (eq136_e1732_d_n15 * p.p246);
        let eq136_e1734_d_n16: f64 = (eq136_e1732_d_n16 * p.p246);
        let eq136_e1734_d_n17: f64 = (eq136_e1732_d_n17 * p.p246);
        let eq136_e1734_d_n18: f64 = (eq136_e1732_d_n18 * p.p246);
        let eq136_e1734_d_n19: f64 = (eq136_e1732_d_n19 * p.p246);
        let eq136_e1734_d_n20: f64 = (eq136_e1732_d_n20 * p.p246);
        let eq136_e1734_d_n21: f64 = (eq136_e1732_d_n21 * p.p246);
        let eq136_e1734_d_n22: f64 = (eq136_e1732_d_n22 * p.p246);
        let eq136_e1734_q: f64 = (eq136_e1732_q * p.p246);
        let eq136_e1734_q_d_n0: f64 = (eq136_e1732_q_d_n0 * p.p246);
        let eq136_e1734_q_d_n1: f64 = (eq136_e1732_q_d_n1 * p.p246);
        let eq136_e1734_q_d_n2: f64 = (eq136_e1732_q_d_n2 * p.p246);
        let eq136_e1734_q_d_n3: f64 = (eq136_e1732_q_d_n3 * p.p246);
        let eq136_e1734_q_d_n4: f64 = (eq136_e1732_q_d_n4 * p.p246);
        let eq136_e1734_q_d_n5: f64 = (eq136_e1732_q_d_n5 * p.p246);
        let eq136_e1734_q_d_n6: f64 = (eq136_e1732_q_d_n6 * p.p246);
        let eq136_e1734_q_d_n7: f64 = (eq136_e1732_q_d_n7 * p.p246);
        let eq136_e1734_q_d_n8: f64 = (eq136_e1732_q_d_n8 * p.p246);
        let eq136_e1734_q_d_n9: f64 = (eq136_e1732_q_d_n9 * p.p246);
        let eq136_e1734_q_d_n10: f64 = (eq136_e1732_q_d_n10 * p.p246);
        let eq136_e1734_q_d_n11: f64 = (eq136_e1732_q_d_n11 * p.p246);
        let eq136_e1734_q_d_n12: f64 = (eq136_e1732_q_d_n12 * p.p246);
        let eq136_e1734_q_d_n13: f64 = (eq136_e1732_q_d_n13 * p.p246);
        let eq136_e1734_q_d_n14: f64 = (eq136_e1732_q_d_n14 * p.p246);
        let eq136_e1734_q_d_n15: f64 = (eq136_e1732_q_d_n15 * p.p246);
        let eq136_e1734_q_d_n16: f64 = (eq136_e1732_q_d_n16 * p.p246);
        let eq136_e1734_q_d_n17: f64 = (eq136_e1732_q_d_n17 * p.p246);
        let eq136_e1734_q_d_n18: f64 = (eq136_e1732_q_d_n18 * p.p246);
        let eq136_e1734_q_d_n19: f64 = (eq136_e1732_q_d_n19 * p.p246);
        let eq136_e1734_q_d_n20: f64 = (eq136_e1732_q_d_n20 * p.p246);
        let eq136_e1734_q_d_n21: f64 = (eq136_e1732_q_d_n21 * p.p246);
        let eq136_e1734_q_d_n22: f64 = (eq136_e1732_q_d_n22 * p.p246);
        (eq136_e1734, eq136_e1734_d_n0, eq136_e1734_d_n1, eq136_e1734_d_n2, eq136_e1734_d_n3, eq136_e1734_d_n4, eq136_e1734_d_n5, eq136_e1734_d_n6, eq136_e1734_d_n7, eq136_e1734_d_n8, eq136_e1734_d_n9, eq136_e1734_d_n10, eq136_e1734_d_n11, eq136_e1734_d_n12, eq136_e1734_d_n13, eq136_e1734_d_n14, eq136_e1734_d_n15, eq136_e1734_d_n16, eq136_e1734_d_n17, eq136_e1734_d_n18, eq136_e1734_d_n19, eq136_e1734_d_n20, eq136_e1734_d_n21, eq136_e1734_d_n22, eq136_e1734_q, eq136_e1734_q_d_n0, eq136_e1734_q_d_n1, eq136_e1734_q_d_n2, eq136_e1734_q_d_n3, eq136_e1734_q_d_n4, eq136_e1734_q_d_n5, eq136_e1734_q_d_n6, eq136_e1734_q_d_n7, eq136_e1734_q_d_n8, eq136_e1734_q_d_n9, eq136_e1734_q_d_n10, eq136_e1734_q_d_n11, eq136_e1734_q_d_n12, eq136_e1734_q_d_n13, eq136_e1734_q_d_n14, eq136_e1734_q_d_n15, eq136_e1734_q_d_n16, eq136_e1734_q_d_n17, eq136_e1734_q_d_n18, eq136_e1734_q_d_n19, eq136_e1734_q_d_n20, eq136_e1734_q_d_n21, eq136_e1734_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq136_reactive_node_derivatives: [f64; 23] = [eq136_e1736_q_d_n0, eq136_e1736_q_d_n1, eq136_e1736_q_d_n2, eq136_e1736_q_d_n3, eq136_e1736_q_d_n4, eq136_e1736_q_d_n5, eq136_e1736_q_d_n6, eq136_e1736_q_d_n7, eq136_e1736_q_d_n8, eq136_e1736_q_d_n9, eq136_e1736_q_d_n10, eq136_e1736_q_d_n11, eq136_e1736_q_d_n12, eq136_e1736_q_d_n13, eq136_e1736_q_d_n14, eq136_e1736_q_d_n15, eq136_e1736_q_d_n16, eq136_e1736_q_d_n17, eq136_e1736_q_d_n18, eq136_e1736_q_d_n19, eq136_e1736_q_d_n20, eq136_e1736_q_d_n21, eq136_e1736_q_d_n22];
        let eq136_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[19]),
            &nodes,
            &eq136_reactive_node_derivatives,
            &branches,
            &eq136_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_137_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq137_e1747, eq137_e1747_d_n0, eq137_e1747_d_n1, eq137_e1747_d_n2, eq137_e1747_d_n3, eq137_e1747_d_n4, eq137_e1747_d_n5, eq137_e1747_d_n6, eq137_e1747_d_n7, eq137_e1747_d_n8, eq137_e1747_d_n9, eq137_e1747_d_n10, eq137_e1747_d_n11, eq137_e1747_d_n12, eq137_e1747_d_n13, eq137_e1747_d_n14, eq137_e1747_d_n15, eq137_e1747_d_n16, eq137_e1747_d_n17, eq137_e1747_d_n18, eq137_e1747_d_n19, eq137_e1747_d_n20, eq137_e1747_d_n21, eq137_e1747_d_n22, eq137_e1747_q, eq137_e1747_q_d_n0, eq137_e1747_q_d_n1, eq137_e1747_q_d_n2, eq137_e1747_q_d_n3, eq137_e1747_q_d_n4, eq137_e1747_q_d_n5, eq137_e1747_q_d_n6, eq137_e1747_q_d_n7, eq137_e1747_q_d_n8, eq137_e1747_q_d_n9, eq137_e1747_q_d_n10, eq137_e1747_q_d_n11, eq137_e1747_q_d_n12, eq137_e1747_q_d_n13, eq137_e1747_q_d_n14, eq137_e1747_q_d_n15, eq137_e1747_q_d_n16, eq137_e1747_q_d_n17, eq137_e1747_q_d_n18, eq137_e1747_q_d_n19, eq137_e1747_q_d_n20, eq137_e1747_q_d_n21, eq137_e1747_q_d_n22,) = {
    if ((s.v[575] != 0.0) && (s.v[576] != 0.0)) {
        let eq137_e1743: f64 = (p.p251 * s.v[240]);
        let eq137_e1743_d_n0: f64 = (p.p251 * s.dn[240][0]);
        let eq137_e1743_d_n1: f64 = (p.p251 * s.dn[240][1]);
        let eq137_e1743_d_n2: f64 = (p.p251 * s.dn[240][2]);
        let eq137_e1743_d_n3: f64 = (p.p251 * s.dn[240][3]);
        let eq137_e1743_d_n4: f64 = (p.p251 * s.dn[240][4]);
        let eq137_e1743_d_n5: f64 = (p.p251 * s.dn[240][5]);
        let eq137_e1743_d_n6: f64 = (p.p251 * s.dn[240][6]);
        let eq137_e1743_d_n7: f64 = (p.p251 * s.dn[240][7]);
        let eq137_e1743_d_n8: f64 = (p.p251 * s.dn[240][8]);
        let eq137_e1743_d_n9: f64 = (p.p251 * s.dn[240][9]);
        let eq137_e1743_d_n10: f64 = (p.p251 * s.dn[240][10]);
        let eq137_e1743_d_n11: f64 = (p.p251 * s.dn[240][11]);
        let eq137_e1743_d_n12: f64 = (p.p251 * s.dn[240][12]);
        let eq137_e1743_d_n13: f64 = (p.p251 * s.dn[240][13]);
        let eq137_e1743_d_n14: f64 = (p.p251 * s.dn[240][14]);
        let eq137_e1743_d_n15: f64 = (p.p251 * s.dn[240][15]);
        let eq137_e1743_d_n16: f64 = (p.p251 * s.dn[240][16]);
        let eq137_e1743_d_n17: f64 = (p.p251 * s.dn[240][17]);
        let eq137_e1743_d_n18: f64 = (p.p251 * s.dn[240][18]);
        let eq137_e1743_d_n19: f64 = (p.p251 * s.dn[240][19]);
        let eq137_e1743_d_n20: f64 = (p.p251 * s.dn[240][20]);
        let eq137_e1743_d_n21: f64 = (p.p251 * s.dn[240][21]);
        let eq137_e1743_d_n22: f64 = (p.p251 * s.dn[240][22]);
        let eq137_e1744_q: f64 = eq137_e1743;
        let eq137_e1745: f64 = (p.p7 * eq137_e1743);
        let eq137_e1745_d_n0: f64 = (p.p7 * eq137_e1743_d_n0);
        let eq137_e1745_d_n1: f64 = (p.p7 * eq137_e1743_d_n1);
        let eq137_e1745_d_n2: f64 = (p.p7 * eq137_e1743_d_n2);
        let eq137_e1745_d_n3: f64 = (p.p7 * eq137_e1743_d_n3);
        let eq137_e1745_d_n4: f64 = (p.p7 * eq137_e1743_d_n4);
        let eq137_e1745_d_n5: f64 = (p.p7 * eq137_e1743_d_n5);
        let eq137_e1745_d_n6: f64 = (p.p7 * eq137_e1743_d_n6);
        let eq137_e1745_d_n7: f64 = (p.p7 * eq137_e1743_d_n7);
        let eq137_e1745_d_n8: f64 = (p.p7 * eq137_e1743_d_n8);
        let eq137_e1745_d_n9: f64 = (p.p7 * eq137_e1743_d_n9);
        let eq137_e1745_d_n10: f64 = (p.p7 * eq137_e1743_d_n10);
        let eq137_e1745_d_n11: f64 = (p.p7 * eq137_e1743_d_n11);
        let eq137_e1745_d_n12: f64 = (p.p7 * eq137_e1743_d_n12);
        let eq137_e1745_d_n13: f64 = (p.p7 * eq137_e1743_d_n13);
        let eq137_e1745_d_n14: f64 = (p.p7 * eq137_e1743_d_n14);
        let eq137_e1745_d_n15: f64 = (p.p7 * eq137_e1743_d_n15);
        let eq137_e1745_d_n16: f64 = (p.p7 * eq137_e1743_d_n16);
        let eq137_e1745_d_n17: f64 = (p.p7 * eq137_e1743_d_n17);
        let eq137_e1745_d_n18: f64 = (p.p7 * eq137_e1743_d_n18);
        let eq137_e1745_d_n19: f64 = (p.p7 * eq137_e1743_d_n19);
        let eq137_e1745_d_n20: f64 = (p.p7 * eq137_e1743_d_n20);
        let eq137_e1745_d_n21: f64 = (p.p7 * eq137_e1743_d_n21);
        let eq137_e1745_d_n22: f64 = (p.p7 * eq137_e1743_d_n22);
        let eq137_e1745_q: f64 = (p.p7 * eq137_e1744_q);
        let eq137_e1745_q_d_n0: f64 = (p.p7 * eq137_e1743_d_n0);
        let eq137_e1745_q_d_n1: f64 = (p.p7 * eq137_e1743_d_n1);
        let eq137_e1745_q_d_n2: f64 = (p.p7 * eq137_e1743_d_n2);
        let eq137_e1745_q_d_n3: f64 = (p.p7 * eq137_e1743_d_n3);
        let eq137_e1745_q_d_n4: f64 = (p.p7 * eq137_e1743_d_n4);
        let eq137_e1745_q_d_n5: f64 = (p.p7 * eq137_e1743_d_n5);
        let eq137_e1745_q_d_n6: f64 = (p.p7 * eq137_e1743_d_n6);
        let eq137_e1745_q_d_n7: f64 = (p.p7 * eq137_e1743_d_n7);
        let eq137_e1745_q_d_n8: f64 = (p.p7 * eq137_e1743_d_n8);
        let eq137_e1745_q_d_n9: f64 = (p.p7 * eq137_e1743_d_n9);
        let eq137_e1745_q_d_n10: f64 = (p.p7 * eq137_e1743_d_n10);
        let eq137_e1745_q_d_n11: f64 = (p.p7 * eq137_e1743_d_n11);
        let eq137_e1745_q_d_n12: f64 = (p.p7 * eq137_e1743_d_n12);
        let eq137_e1745_q_d_n13: f64 = (p.p7 * eq137_e1743_d_n13);
        let eq137_e1745_q_d_n14: f64 = (p.p7 * eq137_e1743_d_n14);
        let eq137_e1745_q_d_n15: f64 = (p.p7 * eq137_e1743_d_n15);
        let eq137_e1745_q_d_n16: f64 = (p.p7 * eq137_e1743_d_n16);
        let eq137_e1745_q_d_n17: f64 = (p.p7 * eq137_e1743_d_n17);
        let eq137_e1745_q_d_n18: f64 = (p.p7 * eq137_e1743_d_n18);
        let eq137_e1745_q_d_n19: f64 = (p.p7 * eq137_e1743_d_n19);
        let eq137_e1745_q_d_n20: f64 = (p.p7 * eq137_e1743_d_n20);
        let eq137_e1745_q_d_n21: f64 = (p.p7 * eq137_e1743_d_n21);
        let eq137_e1745_q_d_n22: f64 = (p.p7 * eq137_e1743_d_n22);
        (eq137_e1745, eq137_e1745_d_n0, eq137_e1745_d_n1, eq137_e1745_d_n2, eq137_e1745_d_n3, eq137_e1745_d_n4, eq137_e1745_d_n5, eq137_e1745_d_n6, eq137_e1745_d_n7, eq137_e1745_d_n8, eq137_e1745_d_n9, eq137_e1745_d_n10, eq137_e1745_d_n11, eq137_e1745_d_n12, eq137_e1745_d_n13, eq137_e1745_d_n14, eq137_e1745_d_n15, eq137_e1745_d_n16, eq137_e1745_d_n17, eq137_e1745_d_n18, eq137_e1745_d_n19, eq137_e1745_d_n20, eq137_e1745_d_n21, eq137_e1745_d_n22, eq137_e1745_q, eq137_e1745_q_d_n0, eq137_e1745_q_d_n1, eq137_e1745_q_d_n2, eq137_e1745_q_d_n3, eq137_e1745_q_d_n4, eq137_e1745_q_d_n5, eq137_e1745_q_d_n6, eq137_e1745_q_d_n7, eq137_e1745_q_d_n8, eq137_e1745_q_d_n9, eq137_e1745_q_d_n10, eq137_e1745_q_d_n11, eq137_e1745_q_d_n12, eq137_e1745_q_d_n13, eq137_e1745_q_d_n14, eq137_e1745_q_d_n15, eq137_e1745_q_d_n16, eq137_e1745_q_d_n17, eq137_e1745_q_d_n18, eq137_e1745_q_d_n19, eq137_e1745_q_d_n20, eq137_e1745_q_d_n21, eq137_e1745_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq137_reactive_node_derivatives: [f64; 23] = [eq137_e1747_q_d_n0, eq137_e1747_q_d_n1, eq137_e1747_q_d_n2, eq137_e1747_q_d_n3, eq137_e1747_q_d_n4, eq137_e1747_q_d_n5, eq137_e1747_q_d_n6, eq137_e1747_q_d_n7, eq137_e1747_q_d_n8, eq137_e1747_q_d_n9, eq137_e1747_q_d_n10, eq137_e1747_q_d_n11, eq137_e1747_q_d_n12, eq137_e1747_q_d_n13, eq137_e1747_q_d_n14, eq137_e1747_q_d_n15, eq137_e1747_q_d_n16, eq137_e1747_q_d_n17, eq137_e1747_q_d_n18, eq137_e1747_q_d_n19, eq137_e1747_q_d_n20, eq137_e1747_q_d_n21, eq137_e1747_q_d_n22];
        let eq137_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[19]),
            &nodes,
            &eq137_reactive_node_derivatives,
            &branches,
            &eq137_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_138_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq138_e1757, eq138_e1757_d_n0, eq138_e1757_d_n1, eq138_e1757_d_n2, eq138_e1757_d_n3, eq138_e1757_d_n4, eq138_e1757_d_n5, eq138_e1757_d_n6, eq138_e1757_d_n7, eq138_e1757_d_n8, eq138_e1757_d_n9, eq138_e1757_d_n10, eq138_e1757_d_n11, eq138_e1757_d_n12, eq138_e1757_d_n13, eq138_e1757_d_n14, eq138_e1757_d_n15, eq138_e1757_d_n16, eq138_e1757_d_n17, eq138_e1757_d_n18, eq138_e1757_d_n19, eq138_e1757_d_n20, eq138_e1757_d_n21, eq138_e1757_d_n22, eq138_e1757_q, eq138_e1757_q_d_n0, eq138_e1757_q_d_n1, eq138_e1757_q_d_n2, eq138_e1757_q_d_n3, eq138_e1757_q_d_n4, eq138_e1757_q_d_n5, eq138_e1757_q_d_n6, eq138_e1757_q_d_n7, eq138_e1757_q_d_n8, eq138_e1757_q_d_n9, eq138_e1757_q_d_n10, eq138_e1757_q_d_n11, eq138_e1757_q_d_n12, eq138_e1757_q_d_n13, eq138_e1757_q_d_n14, eq138_e1757_q_d_n15, eq138_e1757_q_d_n16, eq138_e1757_q_d_n17, eq138_e1757_q_d_n18, eq138_e1757_q_d_n19, eq138_e1757_q_d_n20, eq138_e1757_q_d_n21, eq138_e1757_q_d_n22,) = {
    if ((!(s.v[575] != 0.0)) && (s.v[578] != 0.0)) {
        let eq138_e1754_q: f64 = s.v[241];
        let eq138_e1755: f64 = (p.p7 * s.v[241]);
        let eq138_e1755_d_n0: f64 = (p.p7 * s.dn[241][0]);
        let eq138_e1755_d_n1: f64 = (p.p7 * s.dn[241][1]);
        let eq138_e1755_d_n2: f64 = (p.p7 * s.dn[241][2]);
        let eq138_e1755_d_n3: f64 = (p.p7 * s.dn[241][3]);
        let eq138_e1755_d_n4: f64 = (p.p7 * s.dn[241][4]);
        let eq138_e1755_d_n5: f64 = (p.p7 * s.dn[241][5]);
        let eq138_e1755_d_n6: f64 = (p.p7 * s.dn[241][6]);
        let eq138_e1755_d_n7: f64 = (p.p7 * s.dn[241][7]);
        let eq138_e1755_d_n8: f64 = (p.p7 * s.dn[241][8]);
        let eq138_e1755_d_n9: f64 = (p.p7 * s.dn[241][9]);
        let eq138_e1755_d_n10: f64 = (p.p7 * s.dn[241][10]);
        let eq138_e1755_d_n11: f64 = (p.p7 * s.dn[241][11]);
        let eq138_e1755_d_n12: f64 = (p.p7 * s.dn[241][12]);
        let eq138_e1755_d_n13: f64 = (p.p7 * s.dn[241][13]);
        let eq138_e1755_d_n14: f64 = (p.p7 * s.dn[241][14]);
        let eq138_e1755_d_n15: f64 = (p.p7 * s.dn[241][15]);
        let eq138_e1755_d_n16: f64 = (p.p7 * s.dn[241][16]);
        let eq138_e1755_d_n17: f64 = (p.p7 * s.dn[241][17]);
        let eq138_e1755_d_n18: f64 = (p.p7 * s.dn[241][18]);
        let eq138_e1755_d_n19: f64 = (p.p7 * s.dn[241][19]);
        let eq138_e1755_d_n20: f64 = (p.p7 * s.dn[241][20]);
        let eq138_e1755_d_n21: f64 = (p.p7 * s.dn[241][21]);
        let eq138_e1755_d_n22: f64 = (p.p7 * s.dn[241][22]);
        let eq138_e1755_q: f64 = (p.p7 * eq138_e1754_q);
        let eq138_e1755_q_d_n0: f64 = (p.p7 * s.dn[241][0]);
        let eq138_e1755_q_d_n1: f64 = (p.p7 * s.dn[241][1]);
        let eq138_e1755_q_d_n2: f64 = (p.p7 * s.dn[241][2]);
        let eq138_e1755_q_d_n3: f64 = (p.p7 * s.dn[241][3]);
        let eq138_e1755_q_d_n4: f64 = (p.p7 * s.dn[241][4]);
        let eq138_e1755_q_d_n5: f64 = (p.p7 * s.dn[241][5]);
        let eq138_e1755_q_d_n6: f64 = (p.p7 * s.dn[241][6]);
        let eq138_e1755_q_d_n7: f64 = (p.p7 * s.dn[241][7]);
        let eq138_e1755_q_d_n8: f64 = (p.p7 * s.dn[241][8]);
        let eq138_e1755_q_d_n9: f64 = (p.p7 * s.dn[241][9]);
        let eq138_e1755_q_d_n10: f64 = (p.p7 * s.dn[241][10]);
        let eq138_e1755_q_d_n11: f64 = (p.p7 * s.dn[241][11]);
        let eq138_e1755_q_d_n12: f64 = (p.p7 * s.dn[241][12]);
        let eq138_e1755_q_d_n13: f64 = (p.p7 * s.dn[241][13]);
        let eq138_e1755_q_d_n14: f64 = (p.p7 * s.dn[241][14]);
        let eq138_e1755_q_d_n15: f64 = (p.p7 * s.dn[241][15]);
        let eq138_e1755_q_d_n16: f64 = (p.p7 * s.dn[241][16]);
        let eq138_e1755_q_d_n17: f64 = (p.p7 * s.dn[241][17]);
        let eq138_e1755_q_d_n18: f64 = (p.p7 * s.dn[241][18]);
        let eq138_e1755_q_d_n19: f64 = (p.p7 * s.dn[241][19]);
        let eq138_e1755_q_d_n20: f64 = (p.p7 * s.dn[241][20]);
        let eq138_e1755_q_d_n21: f64 = (p.p7 * s.dn[241][21]);
        let eq138_e1755_q_d_n22: f64 = (p.p7 * s.dn[241][22]);
        (eq138_e1755, eq138_e1755_d_n0, eq138_e1755_d_n1, eq138_e1755_d_n2, eq138_e1755_d_n3, eq138_e1755_d_n4, eq138_e1755_d_n5, eq138_e1755_d_n6, eq138_e1755_d_n7, eq138_e1755_d_n8, eq138_e1755_d_n9, eq138_e1755_d_n10, eq138_e1755_d_n11, eq138_e1755_d_n12, eq138_e1755_d_n13, eq138_e1755_d_n14, eq138_e1755_d_n15, eq138_e1755_d_n16, eq138_e1755_d_n17, eq138_e1755_d_n18, eq138_e1755_d_n19, eq138_e1755_d_n20, eq138_e1755_d_n21, eq138_e1755_d_n22, eq138_e1755_q, eq138_e1755_q_d_n0, eq138_e1755_q_d_n1, eq138_e1755_q_d_n2, eq138_e1755_q_d_n3, eq138_e1755_q_d_n4, eq138_e1755_q_d_n5, eq138_e1755_q_d_n6, eq138_e1755_q_d_n7, eq138_e1755_q_d_n8, eq138_e1755_q_d_n9, eq138_e1755_q_d_n10, eq138_e1755_q_d_n11, eq138_e1755_q_d_n12, eq138_e1755_q_d_n13, eq138_e1755_q_d_n14, eq138_e1755_q_d_n15, eq138_e1755_q_d_n16, eq138_e1755_q_d_n17, eq138_e1755_q_d_n18, eq138_e1755_q_d_n19, eq138_e1755_q_d_n20, eq138_e1755_q_d_n21, eq138_e1755_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq138_reactive_node_derivatives: [f64; 23] = [eq138_e1757_q_d_n0, eq138_e1757_q_d_n1, eq138_e1757_q_d_n2, eq138_e1757_q_d_n3, eq138_e1757_q_d_n4, eq138_e1757_q_d_n5, eq138_e1757_q_d_n6, eq138_e1757_q_d_n7, eq138_e1757_q_d_n8, eq138_e1757_q_d_n9, eq138_e1757_q_d_n10, eq138_e1757_q_d_n11, eq138_e1757_q_d_n12, eq138_e1757_q_d_n13, eq138_e1757_q_d_n14, eq138_e1757_q_d_n15, eq138_e1757_q_d_n16, eq138_e1757_q_d_n17, eq138_e1757_q_d_n18, eq138_e1757_q_d_n19, eq138_e1757_q_d_n20, eq138_e1757_q_d_n21, eq138_e1757_q_d_n22];
        let eq138_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            &nodes,
            &eq138_reactive_node_derivatives,
            &branches,
            &eq138_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
