#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_139_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq139_e1769, eq139_e1769_d_n0, eq139_e1769_d_n1, eq139_e1769_d_n2, eq139_e1769_d_n3, eq139_e1769_d_n4, eq139_e1769_d_n5, eq139_e1769_d_n6, eq139_e1769_d_n7, eq139_e1769_d_n8, eq139_e1769_d_n9, eq139_e1769_d_n10, eq139_e1769_d_n11, eq139_e1769_d_n12, eq139_e1769_d_n13, eq139_e1769_d_n14, eq139_e1769_d_n15, eq139_e1769_d_n16, eq139_e1769_d_n17, eq139_e1769_d_n18, eq139_e1769_d_n19, eq139_e1769_d_n20, eq139_e1769_d_n21, eq139_e1769_d_n22, eq139_e1769_q, eq139_e1769_q_d_n0, eq139_e1769_q_d_n1, eq139_e1769_q_d_n2, eq139_e1769_q_d_n3, eq139_e1769_q_d_n4, eq139_e1769_q_d_n5, eq139_e1769_q_d_n6, eq139_e1769_q_d_n7, eq139_e1769_q_d_n8, eq139_e1769_q_d_n9, eq139_e1769_q_d_n10, eq139_e1769_q_d_n11, eq139_e1769_q_d_n12, eq139_e1769_q_d_n13, eq139_e1769_q_d_n14, eq139_e1769_q_d_n15, eq139_e1769_q_d_n16, eq139_e1769_q_d_n17, eq139_e1769_q_d_n18, eq139_e1769_q_d_n19, eq139_e1769_q_d_n20, eq139_e1769_q_d_n21, eq139_e1769_q_d_n22,) = {
    if (((!(s.v[575] != 0.0)) && (s.v[578] != 0.0)) && (s.v[579] != 0.0)) {
        let eq139_e1766_q: f64 = s.v[240];
        let eq139_e1767: f64 = (p.p7 * s.v[240]);
        let eq139_e1767_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq139_e1767_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq139_e1767_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq139_e1767_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq139_e1767_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq139_e1767_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq139_e1767_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq139_e1767_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq139_e1767_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq139_e1767_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq139_e1767_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq139_e1767_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq139_e1767_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq139_e1767_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq139_e1767_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq139_e1767_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq139_e1767_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq139_e1767_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq139_e1767_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq139_e1767_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq139_e1767_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq139_e1767_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq139_e1767_d_n22: f64 = (p.p7 * s.dn[240][22]);
        let eq139_e1767_q: f64 = (p.p7 * eq139_e1766_q);
        let eq139_e1767_q_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq139_e1767_q_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq139_e1767_q_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq139_e1767_q_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq139_e1767_q_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq139_e1767_q_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq139_e1767_q_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq139_e1767_q_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq139_e1767_q_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq139_e1767_q_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq139_e1767_q_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq139_e1767_q_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq139_e1767_q_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq139_e1767_q_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq139_e1767_q_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq139_e1767_q_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq139_e1767_q_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq139_e1767_q_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq139_e1767_q_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq139_e1767_q_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq139_e1767_q_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq139_e1767_q_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq139_e1767_q_d_n22: f64 = (p.p7 * s.dn[240][22]);
        (eq139_e1767, eq139_e1767_d_n0, eq139_e1767_d_n1, eq139_e1767_d_n2, eq139_e1767_d_n3, eq139_e1767_d_n4, eq139_e1767_d_n5, eq139_e1767_d_n6, eq139_e1767_d_n7, eq139_e1767_d_n8, eq139_e1767_d_n9, eq139_e1767_d_n10, eq139_e1767_d_n11, eq139_e1767_d_n12, eq139_e1767_d_n13, eq139_e1767_d_n14, eq139_e1767_d_n15, eq139_e1767_d_n16, eq139_e1767_d_n17, eq139_e1767_d_n18, eq139_e1767_d_n19, eq139_e1767_d_n20, eq139_e1767_d_n21, eq139_e1767_d_n22, eq139_e1767_q, eq139_e1767_q_d_n0, eq139_e1767_q_d_n1, eq139_e1767_q_d_n2, eq139_e1767_q_d_n3, eq139_e1767_q_d_n4, eq139_e1767_q_d_n5, eq139_e1767_q_d_n6, eq139_e1767_q_d_n7, eq139_e1767_q_d_n8, eq139_e1767_q_d_n9, eq139_e1767_q_d_n10, eq139_e1767_q_d_n11, eq139_e1767_q_d_n12, eq139_e1767_q_d_n13, eq139_e1767_q_d_n14, eq139_e1767_q_d_n15, eq139_e1767_q_d_n16, eq139_e1767_q_d_n17, eq139_e1767_q_d_n18, eq139_e1767_q_d_n19, eq139_e1767_q_d_n20, eq139_e1767_q_d_n21, eq139_e1767_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq139_reactive_node_derivatives: [f64; 23] = [eq139_e1769_q_d_n0, eq139_e1769_q_d_n1, eq139_e1769_q_d_n2, eq139_e1769_q_d_n3, eq139_e1769_q_d_n4, eq139_e1769_q_d_n5, eq139_e1769_q_d_n6, eq139_e1769_q_d_n7, eq139_e1769_q_d_n8, eq139_e1769_q_d_n9, eq139_e1769_q_d_n10, eq139_e1769_q_d_n11, eq139_e1769_q_d_n12, eq139_e1769_q_d_n13, eq139_e1769_q_d_n14, eq139_e1769_q_d_n15, eq139_e1769_q_d_n16, eq139_e1769_q_d_n17, eq139_e1769_q_d_n18, eq139_e1769_q_d_n19, eq139_e1769_q_d_n20, eq139_e1769_q_d_n21, eq139_e1769_q_d_n22];
        let eq139_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            &nodes,
            &eq139_reactive_node_derivatives,
            &branches,
            &eq139_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_140_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq140_e1783, eq140_e1783_d_n0, eq140_e1783_d_n1, eq140_e1783_d_n2, eq140_e1783_d_n3, eq140_e1783_d_n4, eq140_e1783_d_n5, eq140_e1783_d_n6, eq140_e1783_d_n7, eq140_e1783_d_n8, eq140_e1783_d_n9, eq140_e1783_d_n10, eq140_e1783_d_n11, eq140_e1783_d_n12, eq140_e1783_d_n13, eq140_e1783_d_n14, eq140_e1783_d_n15, eq140_e1783_d_n16, eq140_e1783_d_n17, eq140_e1783_d_n18, eq140_e1783_d_n19, eq140_e1783_d_n20, eq140_e1783_d_n21, eq140_e1783_d_n22, eq140_e1783_q, eq140_e1783_q_d_n0, eq140_e1783_q_d_n1, eq140_e1783_q_d_n2, eq140_e1783_q_d_n3, eq140_e1783_q_d_n4, eq140_e1783_q_d_n5, eq140_e1783_q_d_n6, eq140_e1783_q_d_n7, eq140_e1783_q_d_n8, eq140_e1783_q_d_n9, eq140_e1783_q_d_n10, eq140_e1783_q_d_n11, eq140_e1783_q_d_n12, eq140_e1783_q_d_n13, eq140_e1783_q_d_n14, eq140_e1783_q_d_n15, eq140_e1783_q_d_n16, eq140_e1783_q_d_n17, eq140_e1783_q_d_n18, eq140_e1783_q_d_n19, eq140_e1783_q_d_n20, eq140_e1783_q_d_n21, eq140_e1783_q_d_n22,) = {
    if (((!(s.v[575] != 0.0)) && (s.v[578] != 0.0)) && (s.v[579] != 0.0)) {
        let eq140_e1778_q: f64 = s.v[240];
        let eq140_e1779: f64 = (p.p7 * s.v[240]);
        let eq140_e1779_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq140_e1779_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq140_e1779_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq140_e1779_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq140_e1779_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq140_e1779_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq140_e1779_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq140_e1779_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq140_e1779_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq140_e1779_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq140_e1779_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq140_e1779_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq140_e1779_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq140_e1779_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq140_e1779_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq140_e1779_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq140_e1779_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq140_e1779_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq140_e1779_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq140_e1779_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq140_e1779_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq140_e1779_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq140_e1779_d_n22: f64 = (p.p7 * s.dn[240][22]);
        let eq140_e1779_q: f64 = (p.p7 * eq140_e1778_q);
        let eq140_e1779_q_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq140_e1779_q_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq140_e1779_q_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq140_e1779_q_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq140_e1779_q_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq140_e1779_q_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq140_e1779_q_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq140_e1779_q_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq140_e1779_q_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq140_e1779_q_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq140_e1779_q_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq140_e1779_q_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq140_e1779_q_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq140_e1779_q_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq140_e1779_q_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq140_e1779_q_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq140_e1779_q_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq140_e1779_q_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq140_e1779_q_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq140_e1779_q_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq140_e1779_q_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq140_e1779_q_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq140_e1779_q_d_n22: f64 = (p.p7 * s.dn[240][22]);
        let eq140_e1781: f64 = (eq140_e1779 * p.p246);
        let eq140_e1781_d_n0: f64 = (eq140_e1779_d_n0 * p.p246);
        let eq140_e1781_d_n1: f64 = (eq140_e1779_d_n1 * p.p246);
        let eq140_e1781_d_n2: f64 = (eq140_e1779_d_n2 * p.p246);
        let eq140_e1781_d_n3: f64 = (eq140_e1779_d_n3 * p.p246);
        let eq140_e1781_d_n4: f64 = (eq140_e1779_d_n4 * p.p246);
        let eq140_e1781_d_n5: f64 = (eq140_e1779_d_n5 * p.p246);
        let eq140_e1781_d_n6: f64 = (eq140_e1779_d_n6 * p.p246);
        let eq140_e1781_d_n7: f64 = (eq140_e1779_d_n7 * p.p246);
        let eq140_e1781_d_n8: f64 = (eq140_e1779_d_n8 * p.p246);
        let eq140_e1781_d_n9: f64 = (eq140_e1779_d_n9 * p.p246);
        let eq140_e1781_d_n10: f64 = (eq140_e1779_d_n10 * p.p246);
        let eq140_e1781_d_n11: f64 = (eq140_e1779_d_n11 * p.p246);
        let eq140_e1781_d_n12: f64 = (eq140_e1779_d_n12 * p.p246);
        let eq140_e1781_d_n13: f64 = (eq140_e1779_d_n13 * p.p246);
        let eq140_e1781_d_n14: f64 = (eq140_e1779_d_n14 * p.p246);
        let eq140_e1781_d_n15: f64 = (eq140_e1779_d_n15 * p.p246);
        let eq140_e1781_d_n16: f64 = (eq140_e1779_d_n16 * p.p246);
        let eq140_e1781_d_n17: f64 = (eq140_e1779_d_n17 * p.p246);
        let eq140_e1781_d_n18: f64 = (eq140_e1779_d_n18 * p.p246);
        let eq140_e1781_d_n19: f64 = (eq140_e1779_d_n19 * p.p246);
        let eq140_e1781_d_n20: f64 = (eq140_e1779_d_n20 * p.p246);
        let eq140_e1781_d_n21: f64 = (eq140_e1779_d_n21 * p.p246);
        let eq140_e1781_d_n22: f64 = (eq140_e1779_d_n22 * p.p246);
        let eq140_e1781_q: f64 = (eq140_e1779_q * p.p246);
        let eq140_e1781_q_d_n0: f64 = (eq140_e1779_q_d_n0 * p.p246);
        let eq140_e1781_q_d_n1: f64 = (eq140_e1779_q_d_n1 * p.p246);
        let eq140_e1781_q_d_n2: f64 = (eq140_e1779_q_d_n2 * p.p246);
        let eq140_e1781_q_d_n3: f64 = (eq140_e1779_q_d_n3 * p.p246);
        let eq140_e1781_q_d_n4: f64 = (eq140_e1779_q_d_n4 * p.p246);
        let eq140_e1781_q_d_n5: f64 = (eq140_e1779_q_d_n5 * p.p246);
        let eq140_e1781_q_d_n6: f64 = (eq140_e1779_q_d_n6 * p.p246);
        let eq140_e1781_q_d_n7: f64 = (eq140_e1779_q_d_n7 * p.p246);
        let eq140_e1781_q_d_n8: f64 = (eq140_e1779_q_d_n8 * p.p246);
        let eq140_e1781_q_d_n9: f64 = (eq140_e1779_q_d_n9 * p.p246);
        let eq140_e1781_q_d_n10: f64 = (eq140_e1779_q_d_n10 * p.p246);
        let eq140_e1781_q_d_n11: f64 = (eq140_e1779_q_d_n11 * p.p246);
        let eq140_e1781_q_d_n12: f64 = (eq140_e1779_q_d_n12 * p.p246);
        let eq140_e1781_q_d_n13: f64 = (eq140_e1779_q_d_n13 * p.p246);
        let eq140_e1781_q_d_n14: f64 = (eq140_e1779_q_d_n14 * p.p246);
        let eq140_e1781_q_d_n15: f64 = (eq140_e1779_q_d_n15 * p.p246);
        let eq140_e1781_q_d_n16: f64 = (eq140_e1779_q_d_n16 * p.p246);
        let eq140_e1781_q_d_n17: f64 = (eq140_e1779_q_d_n17 * p.p246);
        let eq140_e1781_q_d_n18: f64 = (eq140_e1779_q_d_n18 * p.p246);
        let eq140_e1781_q_d_n19: f64 = (eq140_e1779_q_d_n19 * p.p246);
        let eq140_e1781_q_d_n20: f64 = (eq140_e1779_q_d_n20 * p.p246);
        let eq140_e1781_q_d_n21: f64 = (eq140_e1779_q_d_n21 * p.p246);
        let eq140_e1781_q_d_n22: f64 = (eq140_e1779_q_d_n22 * p.p246);
        (eq140_e1781, eq140_e1781_d_n0, eq140_e1781_d_n1, eq140_e1781_d_n2, eq140_e1781_d_n3, eq140_e1781_d_n4, eq140_e1781_d_n5, eq140_e1781_d_n6, eq140_e1781_d_n7, eq140_e1781_d_n8, eq140_e1781_d_n9, eq140_e1781_d_n10, eq140_e1781_d_n11, eq140_e1781_d_n12, eq140_e1781_d_n13, eq140_e1781_d_n14, eq140_e1781_d_n15, eq140_e1781_d_n16, eq140_e1781_d_n17, eq140_e1781_d_n18, eq140_e1781_d_n19, eq140_e1781_d_n20, eq140_e1781_d_n21, eq140_e1781_d_n22, eq140_e1781_q, eq140_e1781_q_d_n0, eq140_e1781_q_d_n1, eq140_e1781_q_d_n2, eq140_e1781_q_d_n3, eq140_e1781_q_d_n4, eq140_e1781_q_d_n5, eq140_e1781_q_d_n6, eq140_e1781_q_d_n7, eq140_e1781_q_d_n8, eq140_e1781_q_d_n9, eq140_e1781_q_d_n10, eq140_e1781_q_d_n11, eq140_e1781_q_d_n12, eq140_e1781_q_d_n13, eq140_e1781_q_d_n14, eq140_e1781_q_d_n15, eq140_e1781_q_d_n16, eq140_e1781_q_d_n17, eq140_e1781_q_d_n18, eq140_e1781_q_d_n19, eq140_e1781_q_d_n20, eq140_e1781_q_d_n21, eq140_e1781_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq140_reactive_node_derivatives: [f64; 23] = [eq140_e1783_q_d_n0, eq140_e1783_q_d_n1, eq140_e1783_q_d_n2, eq140_e1783_q_d_n3, eq140_e1783_q_d_n4, eq140_e1783_q_d_n5, eq140_e1783_q_d_n6, eq140_e1783_q_d_n7, eq140_e1783_q_d_n8, eq140_e1783_q_d_n9, eq140_e1783_q_d_n10, eq140_e1783_q_d_n11, eq140_e1783_q_d_n12, eq140_e1783_q_d_n13, eq140_e1783_q_d_n14, eq140_e1783_q_d_n15, eq140_e1783_q_d_n16, eq140_e1783_q_d_n17, eq140_e1783_q_d_n18, eq140_e1783_q_d_n19, eq140_e1783_q_d_n20, eq140_e1783_q_d_n21, eq140_e1783_q_d_n22];
        let eq140_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            &nodes,
            &eq140_reactive_node_derivatives,
            &branches,
            &eq140_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_141_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq141_e1796, eq141_e1796_d_n0, eq141_e1796_d_n1, eq141_e1796_d_n2, eq141_e1796_d_n3, eq141_e1796_d_n4, eq141_e1796_d_n5, eq141_e1796_d_n6, eq141_e1796_d_n7, eq141_e1796_d_n8, eq141_e1796_d_n9, eq141_e1796_d_n10, eq141_e1796_d_n11, eq141_e1796_d_n12, eq141_e1796_d_n13, eq141_e1796_d_n14, eq141_e1796_d_n15, eq141_e1796_d_n16, eq141_e1796_d_n17, eq141_e1796_d_n18, eq141_e1796_d_n19, eq141_e1796_d_n20, eq141_e1796_d_n21, eq141_e1796_d_n22, eq141_e1796_q, eq141_e1796_q_d_n0, eq141_e1796_q_d_n1, eq141_e1796_q_d_n2, eq141_e1796_q_d_n3, eq141_e1796_q_d_n4, eq141_e1796_q_d_n5, eq141_e1796_q_d_n6, eq141_e1796_q_d_n7, eq141_e1796_q_d_n8, eq141_e1796_q_d_n9, eq141_e1796_q_d_n10, eq141_e1796_q_d_n11, eq141_e1796_q_d_n12, eq141_e1796_q_d_n13, eq141_e1796_q_d_n14, eq141_e1796_q_d_n15, eq141_e1796_q_d_n16, eq141_e1796_q_d_n17, eq141_e1796_q_d_n18, eq141_e1796_q_d_n19, eq141_e1796_q_d_n20, eq141_e1796_q_d_n21, eq141_e1796_q_d_n22,) = {
    if (((!(s.v[575] != 0.0)) && (s.v[578] != 0.0)) && (!(s.v[579] != 0.0))) {
        let eq141_e1793_q: f64 = s.v[240];
        let eq141_e1794: f64 = (p.p7 * s.v[240]);
        let eq141_e1794_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq141_e1794_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq141_e1794_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq141_e1794_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq141_e1794_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq141_e1794_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq141_e1794_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq141_e1794_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq141_e1794_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq141_e1794_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq141_e1794_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq141_e1794_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq141_e1794_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq141_e1794_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq141_e1794_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq141_e1794_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq141_e1794_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq141_e1794_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq141_e1794_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq141_e1794_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq141_e1794_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq141_e1794_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq141_e1794_d_n22: f64 = (p.p7 * s.dn[240][22]);
        let eq141_e1794_q: f64 = (p.p7 * eq141_e1793_q);
        let eq141_e1794_q_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq141_e1794_q_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq141_e1794_q_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq141_e1794_q_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq141_e1794_q_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq141_e1794_q_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq141_e1794_q_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq141_e1794_q_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq141_e1794_q_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq141_e1794_q_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq141_e1794_q_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq141_e1794_q_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq141_e1794_q_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq141_e1794_q_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq141_e1794_q_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq141_e1794_q_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq141_e1794_q_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq141_e1794_q_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq141_e1794_q_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq141_e1794_q_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq141_e1794_q_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq141_e1794_q_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq141_e1794_q_d_n22: f64 = (p.p7 * s.dn[240][22]);
        (eq141_e1794, eq141_e1794_d_n0, eq141_e1794_d_n1, eq141_e1794_d_n2, eq141_e1794_d_n3, eq141_e1794_d_n4, eq141_e1794_d_n5, eq141_e1794_d_n6, eq141_e1794_d_n7, eq141_e1794_d_n8, eq141_e1794_d_n9, eq141_e1794_d_n10, eq141_e1794_d_n11, eq141_e1794_d_n12, eq141_e1794_d_n13, eq141_e1794_d_n14, eq141_e1794_d_n15, eq141_e1794_d_n16, eq141_e1794_d_n17, eq141_e1794_d_n18, eq141_e1794_d_n19, eq141_e1794_d_n20, eq141_e1794_d_n21, eq141_e1794_d_n22, eq141_e1794_q, eq141_e1794_q_d_n0, eq141_e1794_q_d_n1, eq141_e1794_q_d_n2, eq141_e1794_q_d_n3, eq141_e1794_q_d_n4, eq141_e1794_q_d_n5, eq141_e1794_q_d_n6, eq141_e1794_q_d_n7, eq141_e1794_q_d_n8, eq141_e1794_q_d_n9, eq141_e1794_q_d_n10, eq141_e1794_q_d_n11, eq141_e1794_q_d_n12, eq141_e1794_q_d_n13, eq141_e1794_q_d_n14, eq141_e1794_q_d_n15, eq141_e1794_q_d_n16, eq141_e1794_q_d_n17, eq141_e1794_q_d_n18, eq141_e1794_q_d_n19, eq141_e1794_q_d_n20, eq141_e1794_q_d_n21, eq141_e1794_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq141_reactive_node_derivatives: [f64; 23] = [eq141_e1796_q_d_n0, eq141_e1796_q_d_n1, eq141_e1796_q_d_n2, eq141_e1796_q_d_n3, eq141_e1796_q_d_n4, eq141_e1796_q_d_n5, eq141_e1796_q_d_n6, eq141_e1796_q_d_n7, eq141_e1796_q_d_n8, eq141_e1796_q_d_n9, eq141_e1796_q_d_n10, eq141_e1796_q_d_n11, eq141_e1796_q_d_n12, eq141_e1796_q_d_n13, eq141_e1796_q_d_n14, eq141_e1796_q_d_n15, eq141_e1796_q_d_n16, eq141_e1796_q_d_n17, eq141_e1796_q_d_n18, eq141_e1796_q_d_n19, eq141_e1796_q_d_n20, eq141_e1796_q_d_n21, eq141_e1796_q_d_n22];
        let eq141_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            &nodes,
            &eq141_reactive_node_derivatives,
            &branches,
            &eq141_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_142_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq142_e1811, eq142_e1811_d_n0, eq142_e1811_d_n1, eq142_e1811_d_n2, eq142_e1811_d_n3, eq142_e1811_d_n4, eq142_e1811_d_n5, eq142_e1811_d_n6, eq142_e1811_d_n7, eq142_e1811_d_n8, eq142_e1811_d_n9, eq142_e1811_d_n10, eq142_e1811_d_n11, eq142_e1811_d_n12, eq142_e1811_d_n13, eq142_e1811_d_n14, eq142_e1811_d_n15, eq142_e1811_d_n16, eq142_e1811_d_n17, eq142_e1811_d_n18, eq142_e1811_d_n19, eq142_e1811_d_n20, eq142_e1811_d_n21, eq142_e1811_d_n22, eq142_e1811_q, eq142_e1811_q_d_n0, eq142_e1811_q_d_n1, eq142_e1811_q_d_n2, eq142_e1811_q_d_n3, eq142_e1811_q_d_n4, eq142_e1811_q_d_n5, eq142_e1811_q_d_n6, eq142_e1811_q_d_n7, eq142_e1811_q_d_n8, eq142_e1811_q_d_n9, eq142_e1811_q_d_n10, eq142_e1811_q_d_n11, eq142_e1811_q_d_n12, eq142_e1811_q_d_n13, eq142_e1811_q_d_n14, eq142_e1811_q_d_n15, eq142_e1811_q_d_n16, eq142_e1811_q_d_n17, eq142_e1811_q_d_n18, eq142_e1811_q_d_n19, eq142_e1811_q_d_n20, eq142_e1811_q_d_n21, eq142_e1811_q_d_n22,) = {
    if (((!(s.v[575] != 0.0)) && (s.v[578] != 0.0)) && (!(s.v[579] != 0.0))) {
        let eq142_e1806_q: f64 = s.v[240];
        let eq142_e1807: f64 = (p.p7 * s.v[240]);
        let eq142_e1807_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq142_e1807_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq142_e1807_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq142_e1807_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq142_e1807_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq142_e1807_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq142_e1807_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq142_e1807_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq142_e1807_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq142_e1807_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq142_e1807_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq142_e1807_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq142_e1807_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq142_e1807_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq142_e1807_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq142_e1807_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq142_e1807_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq142_e1807_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq142_e1807_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq142_e1807_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq142_e1807_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq142_e1807_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq142_e1807_d_n22: f64 = (p.p7 * s.dn[240][22]);
        let eq142_e1807_q: f64 = (p.p7 * eq142_e1806_q);
        let eq142_e1807_q_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq142_e1807_q_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq142_e1807_q_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq142_e1807_q_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq142_e1807_q_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq142_e1807_q_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq142_e1807_q_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq142_e1807_q_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq142_e1807_q_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq142_e1807_q_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq142_e1807_q_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq142_e1807_q_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq142_e1807_q_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq142_e1807_q_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq142_e1807_q_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq142_e1807_q_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq142_e1807_q_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq142_e1807_q_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq142_e1807_q_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq142_e1807_q_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq142_e1807_q_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq142_e1807_q_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq142_e1807_q_d_n22: f64 = (p.p7 * s.dn[240][22]);
        let eq142_e1809: f64 = (eq142_e1807 * p.p246);
        let eq142_e1809_d_n0: f64 = (eq142_e1807_d_n0 * p.p246);
        let eq142_e1809_d_n1: f64 = (eq142_e1807_d_n1 * p.p246);
        let eq142_e1809_d_n2: f64 = (eq142_e1807_d_n2 * p.p246);
        let eq142_e1809_d_n3: f64 = (eq142_e1807_d_n3 * p.p246);
        let eq142_e1809_d_n4: f64 = (eq142_e1807_d_n4 * p.p246);
        let eq142_e1809_d_n5: f64 = (eq142_e1807_d_n5 * p.p246);
        let eq142_e1809_d_n6: f64 = (eq142_e1807_d_n6 * p.p246);
        let eq142_e1809_d_n7: f64 = (eq142_e1807_d_n7 * p.p246);
        let eq142_e1809_d_n8: f64 = (eq142_e1807_d_n8 * p.p246);
        let eq142_e1809_d_n9: f64 = (eq142_e1807_d_n9 * p.p246);
        let eq142_e1809_d_n10: f64 = (eq142_e1807_d_n10 * p.p246);
        let eq142_e1809_d_n11: f64 = (eq142_e1807_d_n11 * p.p246);
        let eq142_e1809_d_n12: f64 = (eq142_e1807_d_n12 * p.p246);
        let eq142_e1809_d_n13: f64 = (eq142_e1807_d_n13 * p.p246);
        let eq142_e1809_d_n14: f64 = (eq142_e1807_d_n14 * p.p246);
        let eq142_e1809_d_n15: f64 = (eq142_e1807_d_n15 * p.p246);
        let eq142_e1809_d_n16: f64 = (eq142_e1807_d_n16 * p.p246);
        let eq142_e1809_d_n17: f64 = (eq142_e1807_d_n17 * p.p246);
        let eq142_e1809_d_n18: f64 = (eq142_e1807_d_n18 * p.p246);
        let eq142_e1809_d_n19: f64 = (eq142_e1807_d_n19 * p.p246);
        let eq142_e1809_d_n20: f64 = (eq142_e1807_d_n20 * p.p246);
        let eq142_e1809_d_n21: f64 = (eq142_e1807_d_n21 * p.p246);
        let eq142_e1809_d_n22: f64 = (eq142_e1807_d_n22 * p.p246);
        let eq142_e1809_q: f64 = (eq142_e1807_q * p.p246);
        let eq142_e1809_q_d_n0: f64 = (eq142_e1807_q_d_n0 * p.p246);
        let eq142_e1809_q_d_n1: f64 = (eq142_e1807_q_d_n1 * p.p246);
        let eq142_e1809_q_d_n2: f64 = (eq142_e1807_q_d_n2 * p.p246);
        let eq142_e1809_q_d_n3: f64 = (eq142_e1807_q_d_n3 * p.p246);
        let eq142_e1809_q_d_n4: f64 = (eq142_e1807_q_d_n4 * p.p246);
        let eq142_e1809_q_d_n5: f64 = (eq142_e1807_q_d_n5 * p.p246);
        let eq142_e1809_q_d_n6: f64 = (eq142_e1807_q_d_n6 * p.p246);
        let eq142_e1809_q_d_n7: f64 = (eq142_e1807_q_d_n7 * p.p246);
        let eq142_e1809_q_d_n8: f64 = (eq142_e1807_q_d_n8 * p.p246);
        let eq142_e1809_q_d_n9: f64 = (eq142_e1807_q_d_n9 * p.p246);
        let eq142_e1809_q_d_n10: f64 = (eq142_e1807_q_d_n10 * p.p246);
        let eq142_e1809_q_d_n11: f64 = (eq142_e1807_q_d_n11 * p.p246);
        let eq142_e1809_q_d_n12: f64 = (eq142_e1807_q_d_n12 * p.p246);
        let eq142_e1809_q_d_n13: f64 = (eq142_e1807_q_d_n13 * p.p246);
        let eq142_e1809_q_d_n14: f64 = (eq142_e1807_q_d_n14 * p.p246);
        let eq142_e1809_q_d_n15: f64 = (eq142_e1807_q_d_n15 * p.p246);
        let eq142_e1809_q_d_n16: f64 = (eq142_e1807_q_d_n16 * p.p246);
        let eq142_e1809_q_d_n17: f64 = (eq142_e1807_q_d_n17 * p.p246);
        let eq142_e1809_q_d_n18: f64 = (eq142_e1807_q_d_n18 * p.p246);
        let eq142_e1809_q_d_n19: f64 = (eq142_e1807_q_d_n19 * p.p246);
        let eq142_e1809_q_d_n20: f64 = (eq142_e1807_q_d_n20 * p.p246);
        let eq142_e1809_q_d_n21: f64 = (eq142_e1807_q_d_n21 * p.p246);
        let eq142_e1809_q_d_n22: f64 = (eq142_e1807_q_d_n22 * p.p246);
        (eq142_e1809, eq142_e1809_d_n0, eq142_e1809_d_n1, eq142_e1809_d_n2, eq142_e1809_d_n3, eq142_e1809_d_n4, eq142_e1809_d_n5, eq142_e1809_d_n6, eq142_e1809_d_n7, eq142_e1809_d_n8, eq142_e1809_d_n9, eq142_e1809_d_n10, eq142_e1809_d_n11, eq142_e1809_d_n12, eq142_e1809_d_n13, eq142_e1809_d_n14, eq142_e1809_d_n15, eq142_e1809_d_n16, eq142_e1809_d_n17, eq142_e1809_d_n18, eq142_e1809_d_n19, eq142_e1809_d_n20, eq142_e1809_d_n21, eq142_e1809_d_n22, eq142_e1809_q, eq142_e1809_q_d_n0, eq142_e1809_q_d_n1, eq142_e1809_q_d_n2, eq142_e1809_q_d_n3, eq142_e1809_q_d_n4, eq142_e1809_q_d_n5, eq142_e1809_q_d_n6, eq142_e1809_q_d_n7, eq142_e1809_q_d_n8, eq142_e1809_q_d_n9, eq142_e1809_q_d_n10, eq142_e1809_q_d_n11, eq142_e1809_q_d_n12, eq142_e1809_q_d_n13, eq142_e1809_q_d_n14, eq142_e1809_q_d_n15, eq142_e1809_q_d_n16, eq142_e1809_q_d_n17, eq142_e1809_q_d_n18, eq142_e1809_q_d_n19, eq142_e1809_q_d_n20, eq142_e1809_q_d_n21, eq142_e1809_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq142_reactive_node_derivatives: [f64; 23] = [eq142_e1811_q_d_n0, eq142_e1811_q_d_n1, eq142_e1811_q_d_n2, eq142_e1811_q_d_n3, eq142_e1811_q_d_n4, eq142_e1811_q_d_n5, eq142_e1811_q_d_n6, eq142_e1811_q_d_n7, eq142_e1811_q_d_n8, eq142_e1811_q_d_n9, eq142_e1811_q_d_n10, eq142_e1811_q_d_n11, eq142_e1811_q_d_n12, eq142_e1811_q_d_n13, eq142_e1811_q_d_n14, eq142_e1811_q_d_n15, eq142_e1811_q_d_n16, eq142_e1811_q_d_n17, eq142_e1811_q_d_n18, eq142_e1811_q_d_n19, eq142_e1811_q_d_n20, eq142_e1811_q_d_n21, eq142_e1811_q_d_n22];
        let eq142_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            &nodes,
            &eq142_reactive_node_derivatives,
            &branches,
            &eq142_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_143_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq143_e1823, eq143_e1823_d_n0, eq143_e1823_d_n1, eq143_e1823_d_n2, eq143_e1823_d_n3, eq143_e1823_d_n4, eq143_e1823_d_n5, eq143_e1823_d_n6, eq143_e1823_d_n7, eq143_e1823_d_n8, eq143_e1823_d_n9, eq143_e1823_d_n10, eq143_e1823_d_n11, eq143_e1823_d_n12, eq143_e1823_d_n13, eq143_e1823_d_n14, eq143_e1823_d_n15, eq143_e1823_d_n16, eq143_e1823_d_n17, eq143_e1823_d_n18, eq143_e1823_d_n19, eq143_e1823_d_n20, eq143_e1823_d_n21, eq143_e1823_d_n22, eq143_e1823_q, eq143_e1823_q_d_n0, eq143_e1823_q_d_n1, eq143_e1823_q_d_n2, eq143_e1823_q_d_n3, eq143_e1823_q_d_n4, eq143_e1823_q_d_n5, eq143_e1823_q_d_n6, eq143_e1823_q_d_n7, eq143_e1823_q_d_n8, eq143_e1823_q_d_n9, eq143_e1823_q_d_n10, eq143_e1823_q_d_n11, eq143_e1823_q_d_n12, eq143_e1823_q_d_n13, eq143_e1823_q_d_n14, eq143_e1823_q_d_n15, eq143_e1823_q_d_n16, eq143_e1823_q_d_n17, eq143_e1823_q_d_n18, eq143_e1823_q_d_n19, eq143_e1823_q_d_n20, eq143_e1823_q_d_n21, eq143_e1823_q_d_n22,) = {
    if ((!(s.v[575] != 0.0)) && (s.v[578] != 0.0)) {
        let eq143_e1819: f64 = (p.p251 * s.v[240]);
        let eq143_e1819_d_n0: f64 = (p.p251 * s.dn[240][0]);
        let eq143_e1819_d_n1: f64 = (p.p251 * s.dn[240][1]);
        let eq143_e1819_d_n2: f64 = (p.p251 * s.dn[240][2]);
        let eq143_e1819_d_n3: f64 = (p.p251 * s.dn[240][3]);
        let eq143_e1819_d_n4: f64 = (p.p251 * s.dn[240][4]);
        let eq143_e1819_d_n5: f64 = (p.p251 * s.dn[240][5]);
        let eq143_e1819_d_n6: f64 = (p.p251 * s.dn[240][6]);
        let eq143_e1819_d_n7: f64 = (p.p251 * s.dn[240][7]);
        let eq143_e1819_d_n8: f64 = (p.p251 * s.dn[240][8]);
        let eq143_e1819_d_n9: f64 = (p.p251 * s.dn[240][9]);
        let eq143_e1819_d_n10: f64 = (p.p251 * s.dn[240][10]);
        let eq143_e1819_d_n11: f64 = (p.p251 * s.dn[240][11]);
        let eq143_e1819_d_n12: f64 = (p.p251 * s.dn[240][12]);
        let eq143_e1819_d_n13: f64 = (p.p251 * s.dn[240][13]);
        let eq143_e1819_d_n14: f64 = (p.p251 * s.dn[240][14]);
        let eq143_e1819_d_n15: f64 = (p.p251 * s.dn[240][15]);
        let eq143_e1819_d_n16: f64 = (p.p251 * s.dn[240][16]);
        let eq143_e1819_d_n17: f64 = (p.p251 * s.dn[240][17]);
        let eq143_e1819_d_n18: f64 = (p.p251 * s.dn[240][18]);
        let eq143_e1819_d_n19: f64 = (p.p251 * s.dn[240][19]);
        let eq143_e1819_d_n20: f64 = (p.p251 * s.dn[240][20]);
        let eq143_e1819_d_n21: f64 = (p.p251 * s.dn[240][21]);
        let eq143_e1819_d_n22: f64 = (p.p251 * s.dn[240][22]);
        let eq143_e1820_q: f64 = eq143_e1819;
        let eq143_e1821: f64 = (p.p7 * eq143_e1819);
        let eq143_e1821_d_n0: f64 = (p.p7 * eq143_e1819_d_n0);
        let eq143_e1821_d_n1: f64 = (p.p7 * eq143_e1819_d_n1);
        let eq143_e1821_d_n2: f64 = (p.p7 * eq143_e1819_d_n2);
        let eq143_e1821_d_n3: f64 = (p.p7 * eq143_e1819_d_n3);
        let eq143_e1821_d_n4: f64 = (p.p7 * eq143_e1819_d_n4);
        let eq143_e1821_d_n5: f64 = (p.p7 * eq143_e1819_d_n5);
        let eq143_e1821_d_n6: f64 = (p.p7 * eq143_e1819_d_n6);
        let eq143_e1821_d_n7: f64 = (p.p7 * eq143_e1819_d_n7);
        let eq143_e1821_d_n8: f64 = (p.p7 * eq143_e1819_d_n8);
        let eq143_e1821_d_n9: f64 = (p.p7 * eq143_e1819_d_n9);
        let eq143_e1821_d_n10: f64 = (p.p7 * eq143_e1819_d_n10);
        let eq143_e1821_d_n11: f64 = (p.p7 * eq143_e1819_d_n11);
        let eq143_e1821_d_n12: f64 = (p.p7 * eq143_e1819_d_n12);
        let eq143_e1821_d_n13: f64 = (p.p7 * eq143_e1819_d_n13);
        let eq143_e1821_d_n14: f64 = (p.p7 * eq143_e1819_d_n14);
        let eq143_e1821_d_n15: f64 = (p.p7 * eq143_e1819_d_n15);
        let eq143_e1821_d_n16: f64 = (p.p7 * eq143_e1819_d_n16);
        let eq143_e1821_d_n17: f64 = (p.p7 * eq143_e1819_d_n17);
        let eq143_e1821_d_n18: f64 = (p.p7 * eq143_e1819_d_n18);
        let eq143_e1821_d_n19: f64 = (p.p7 * eq143_e1819_d_n19);
        let eq143_e1821_d_n20: f64 = (p.p7 * eq143_e1819_d_n20);
        let eq143_e1821_d_n21: f64 = (p.p7 * eq143_e1819_d_n21);
        let eq143_e1821_d_n22: f64 = (p.p7 * eq143_e1819_d_n22);
        let eq143_e1821_q: f64 = (p.p7 * eq143_e1820_q);
        let eq143_e1821_q_d_n0: f64 = (p.p7 * eq143_e1819_d_n0);
        let eq143_e1821_q_d_n1: f64 = (p.p7 * eq143_e1819_d_n1);
        let eq143_e1821_q_d_n2: f64 = (p.p7 * eq143_e1819_d_n2);
        let eq143_e1821_q_d_n3: f64 = (p.p7 * eq143_e1819_d_n3);
        let eq143_e1821_q_d_n4: f64 = (p.p7 * eq143_e1819_d_n4);
        let eq143_e1821_q_d_n5: f64 = (p.p7 * eq143_e1819_d_n5);
        let eq143_e1821_q_d_n6: f64 = (p.p7 * eq143_e1819_d_n6);
        let eq143_e1821_q_d_n7: f64 = (p.p7 * eq143_e1819_d_n7);
        let eq143_e1821_q_d_n8: f64 = (p.p7 * eq143_e1819_d_n8);
        let eq143_e1821_q_d_n9: f64 = (p.p7 * eq143_e1819_d_n9);
        let eq143_e1821_q_d_n10: f64 = (p.p7 * eq143_e1819_d_n10);
        let eq143_e1821_q_d_n11: f64 = (p.p7 * eq143_e1819_d_n11);
        let eq143_e1821_q_d_n12: f64 = (p.p7 * eq143_e1819_d_n12);
        let eq143_e1821_q_d_n13: f64 = (p.p7 * eq143_e1819_d_n13);
        let eq143_e1821_q_d_n14: f64 = (p.p7 * eq143_e1819_d_n14);
        let eq143_e1821_q_d_n15: f64 = (p.p7 * eq143_e1819_d_n15);
        let eq143_e1821_q_d_n16: f64 = (p.p7 * eq143_e1819_d_n16);
        let eq143_e1821_q_d_n17: f64 = (p.p7 * eq143_e1819_d_n17);
        let eq143_e1821_q_d_n18: f64 = (p.p7 * eq143_e1819_d_n18);
        let eq143_e1821_q_d_n19: f64 = (p.p7 * eq143_e1819_d_n19);
        let eq143_e1821_q_d_n20: f64 = (p.p7 * eq143_e1819_d_n20);
        let eq143_e1821_q_d_n21: f64 = (p.p7 * eq143_e1819_d_n21);
        let eq143_e1821_q_d_n22: f64 = (p.p7 * eq143_e1819_d_n22);
        (eq143_e1821, eq143_e1821_d_n0, eq143_e1821_d_n1, eq143_e1821_d_n2, eq143_e1821_d_n3, eq143_e1821_d_n4, eq143_e1821_d_n5, eq143_e1821_d_n6, eq143_e1821_d_n7, eq143_e1821_d_n8, eq143_e1821_d_n9, eq143_e1821_d_n10, eq143_e1821_d_n11, eq143_e1821_d_n12, eq143_e1821_d_n13, eq143_e1821_d_n14, eq143_e1821_d_n15, eq143_e1821_d_n16, eq143_e1821_d_n17, eq143_e1821_d_n18, eq143_e1821_d_n19, eq143_e1821_d_n20, eq143_e1821_d_n21, eq143_e1821_d_n22, eq143_e1821_q, eq143_e1821_q_d_n0, eq143_e1821_q_d_n1, eq143_e1821_q_d_n2, eq143_e1821_q_d_n3, eq143_e1821_q_d_n4, eq143_e1821_q_d_n5, eq143_e1821_q_d_n6, eq143_e1821_q_d_n7, eq143_e1821_q_d_n8, eq143_e1821_q_d_n9, eq143_e1821_q_d_n10, eq143_e1821_q_d_n11, eq143_e1821_q_d_n12, eq143_e1821_q_d_n13, eq143_e1821_q_d_n14, eq143_e1821_q_d_n15, eq143_e1821_q_d_n16, eq143_e1821_q_d_n17, eq143_e1821_q_d_n18, eq143_e1821_q_d_n19, eq143_e1821_q_d_n20, eq143_e1821_q_d_n21, eq143_e1821_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq143_reactive_node_derivatives: [f64; 23] = [eq143_e1823_q_d_n0, eq143_e1823_q_d_n1, eq143_e1823_q_d_n2, eq143_e1823_q_d_n3, eq143_e1823_q_d_n4, eq143_e1823_q_d_n5, eq143_e1823_q_d_n6, eq143_e1823_q_d_n7, eq143_e1823_q_d_n8, eq143_e1823_q_d_n9, eq143_e1823_q_d_n10, eq143_e1823_q_d_n11, eq143_e1823_q_d_n12, eq143_e1823_q_d_n13, eq143_e1823_q_d_n14, eq143_e1823_q_d_n15, eq143_e1823_q_d_n16, eq143_e1823_q_d_n17, eq143_e1823_q_d_n18, eq143_e1823_q_d_n19, eq143_e1823_q_d_n20, eq143_e1823_q_d_n21, eq143_e1823_q_d_n22];
        let eq143_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            &nodes,
            &eq143_reactive_node_derivatives,
            &branches,
            &eq143_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_144_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq144_e1832, eq144_e1832_d_n0, eq144_e1832_d_n1, eq144_e1832_d_n2, eq144_e1832_d_n3, eq144_e1832_d_n4, eq144_e1832_d_n5, eq144_e1832_d_n6, eq144_e1832_d_n7, eq144_e1832_d_n8, eq144_e1832_d_n9, eq144_e1832_d_n10, eq144_e1832_d_n11, eq144_e1832_d_n12, eq144_e1832_d_n13, eq144_e1832_d_n14, eq144_e1832_d_n15, eq144_e1832_d_n16, eq144_e1832_d_n17, eq144_e1832_d_n18, eq144_e1832_d_n19, eq144_e1832_d_n20, eq144_e1832_d_n21, eq144_e1832_d_n22, eq144_e1832_q, eq144_e1832_q_d_n0, eq144_e1832_q_d_n1, eq144_e1832_q_d_n2, eq144_e1832_q_d_n3, eq144_e1832_q_d_n4, eq144_e1832_q_d_n5, eq144_e1832_q_d_n6, eq144_e1832_q_d_n7, eq144_e1832_q_d_n8, eq144_e1832_q_d_n9, eq144_e1832_q_d_n10, eq144_e1832_q_d_n11, eq144_e1832_q_d_n12, eq144_e1832_q_d_n13, eq144_e1832_q_d_n14, eq144_e1832_q_d_n15, eq144_e1832_q_d_n16, eq144_e1832_q_d_n17, eq144_e1832_q_d_n18, eq144_e1832_q_d_n19, eq144_e1832_q_d_n20, eq144_e1832_q_d_n21, eq144_e1832_q_d_n22,) = {
    if ((s.v[580] != 0.0) && (s.v[581] != 0.0)) {
        let eq144_e1829_q: f64 = s.v[253];
        let eq144_e1830: f64 = (p.p7 * s.v[253]);
        let eq144_e1830_d_n0: f64 = (p.p7 * s.dn[253][0]);
        let eq144_e1830_d_n1: f64 = (p.p7 * s.dn[253][1]);
        let eq144_e1830_d_n2: f64 = (p.p7 * s.dn[253][2]);
        let eq144_e1830_d_n3: f64 = (p.p7 * s.dn[253][3]);
        let eq144_e1830_d_n4: f64 = (p.p7 * s.dn[253][4]);
        let eq144_e1830_d_n5: f64 = (p.p7 * s.dn[253][5]);
        let eq144_e1830_d_n6: f64 = (p.p7 * s.dn[253][6]);
        let eq144_e1830_d_n7: f64 = (p.p7 * s.dn[253][7]);
        let eq144_e1830_d_n8: f64 = (p.p7 * s.dn[253][8]);
        let eq144_e1830_d_n9: f64 = (p.p7 * s.dn[253][9]);
        let eq144_e1830_d_n10: f64 = (p.p7 * s.dn[253][10]);
        let eq144_e1830_d_n11: f64 = (p.p7 * s.dn[253][11]);
        let eq144_e1830_d_n12: f64 = (p.p7 * s.dn[253][12]);
        let eq144_e1830_d_n13: f64 = (p.p7 * s.dn[253][13]);
        let eq144_e1830_d_n14: f64 = (p.p7 * s.dn[253][14]);
        let eq144_e1830_d_n15: f64 = (p.p7 * s.dn[253][15]);
        let eq144_e1830_d_n16: f64 = (p.p7 * s.dn[253][16]);
        let eq144_e1830_d_n17: f64 = (p.p7 * s.dn[253][17]);
        let eq144_e1830_d_n18: f64 = (p.p7 * s.dn[253][18]);
        let eq144_e1830_d_n19: f64 = (p.p7 * s.dn[253][19]);
        let eq144_e1830_d_n20: f64 = (p.p7 * s.dn[253][20]);
        let eq144_e1830_d_n21: f64 = (p.p7 * s.dn[253][21]);
        let eq144_e1830_d_n22: f64 = (p.p7 * s.dn[253][22]);
        let eq144_e1830_q: f64 = (p.p7 * eq144_e1829_q);
        let eq144_e1830_q_d_n0: f64 = (p.p7 * s.dn[253][0]);
        let eq144_e1830_q_d_n1: f64 = (p.p7 * s.dn[253][1]);
        let eq144_e1830_q_d_n2: f64 = (p.p7 * s.dn[253][2]);
        let eq144_e1830_q_d_n3: f64 = (p.p7 * s.dn[253][3]);
        let eq144_e1830_q_d_n4: f64 = (p.p7 * s.dn[253][4]);
        let eq144_e1830_q_d_n5: f64 = (p.p7 * s.dn[253][5]);
        let eq144_e1830_q_d_n6: f64 = (p.p7 * s.dn[253][6]);
        let eq144_e1830_q_d_n7: f64 = (p.p7 * s.dn[253][7]);
        let eq144_e1830_q_d_n8: f64 = (p.p7 * s.dn[253][8]);
        let eq144_e1830_q_d_n9: f64 = (p.p7 * s.dn[253][9]);
        let eq144_e1830_q_d_n10: f64 = (p.p7 * s.dn[253][10]);
        let eq144_e1830_q_d_n11: f64 = (p.p7 * s.dn[253][11]);
        let eq144_e1830_q_d_n12: f64 = (p.p7 * s.dn[253][12]);
        let eq144_e1830_q_d_n13: f64 = (p.p7 * s.dn[253][13]);
        let eq144_e1830_q_d_n14: f64 = (p.p7 * s.dn[253][14]);
        let eq144_e1830_q_d_n15: f64 = (p.p7 * s.dn[253][15]);
        let eq144_e1830_q_d_n16: f64 = (p.p7 * s.dn[253][16]);
        let eq144_e1830_q_d_n17: f64 = (p.p7 * s.dn[253][17]);
        let eq144_e1830_q_d_n18: f64 = (p.p7 * s.dn[253][18]);
        let eq144_e1830_q_d_n19: f64 = (p.p7 * s.dn[253][19]);
        let eq144_e1830_q_d_n20: f64 = (p.p7 * s.dn[253][20]);
        let eq144_e1830_q_d_n21: f64 = (p.p7 * s.dn[253][21]);
        let eq144_e1830_q_d_n22: f64 = (p.p7 * s.dn[253][22]);
        (eq144_e1830, eq144_e1830_d_n0, eq144_e1830_d_n1, eq144_e1830_d_n2, eq144_e1830_d_n3, eq144_e1830_d_n4, eq144_e1830_d_n5, eq144_e1830_d_n6, eq144_e1830_d_n7, eq144_e1830_d_n8, eq144_e1830_d_n9, eq144_e1830_d_n10, eq144_e1830_d_n11, eq144_e1830_d_n12, eq144_e1830_d_n13, eq144_e1830_d_n14, eq144_e1830_d_n15, eq144_e1830_d_n16, eq144_e1830_d_n17, eq144_e1830_d_n18, eq144_e1830_d_n19, eq144_e1830_d_n20, eq144_e1830_d_n21, eq144_e1830_d_n22, eq144_e1830_q, eq144_e1830_q_d_n0, eq144_e1830_q_d_n1, eq144_e1830_q_d_n2, eq144_e1830_q_d_n3, eq144_e1830_q_d_n4, eq144_e1830_q_d_n5, eq144_e1830_q_d_n6, eq144_e1830_q_d_n7, eq144_e1830_q_d_n8, eq144_e1830_q_d_n9, eq144_e1830_q_d_n10, eq144_e1830_q_d_n11, eq144_e1830_q_d_n12, eq144_e1830_q_d_n13, eq144_e1830_q_d_n14, eq144_e1830_q_d_n15, eq144_e1830_q_d_n16, eq144_e1830_q_d_n17, eq144_e1830_q_d_n18, eq144_e1830_q_d_n19, eq144_e1830_q_d_n20, eq144_e1830_q_d_n21, eq144_e1830_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq144_reactive_node_derivatives: [f64; 23] = [eq144_e1832_q_d_n0, eq144_e1832_q_d_n1, eq144_e1832_q_d_n2, eq144_e1832_q_d_n3, eq144_e1832_q_d_n4, eq144_e1832_q_d_n5, eq144_e1832_q_d_n6, eq144_e1832_q_d_n7, eq144_e1832_q_d_n8, eq144_e1832_q_d_n9, eq144_e1832_q_d_n10, eq144_e1832_q_d_n11, eq144_e1832_q_d_n12, eq144_e1832_q_d_n13, eq144_e1832_q_d_n14, eq144_e1832_q_d_n15, eq144_e1832_q_d_n16, eq144_e1832_q_d_n17, eq144_e1832_q_d_n18, eq144_e1832_q_d_n19, eq144_e1832_q_d_n20, eq144_e1832_q_d_n21, eq144_e1832_q_d_n22];
        let eq144_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[16]),
            Some(nodes[15]),
            &nodes,
            &eq144_reactive_node_derivatives,
            &branches,
            &eq144_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_145_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq145_e1843, eq145_e1843_d_n0, eq145_e1843_d_n1, eq145_e1843_d_n2, eq145_e1843_d_n3, eq145_e1843_d_n4, eq145_e1843_d_n5, eq145_e1843_d_n6, eq145_e1843_d_n7, eq145_e1843_d_n8, eq145_e1843_d_n9, eq145_e1843_d_n10, eq145_e1843_d_n11, eq145_e1843_d_n12, eq145_e1843_d_n13, eq145_e1843_d_n14, eq145_e1843_d_n15, eq145_e1843_d_n16, eq145_e1843_d_n17, eq145_e1843_d_n18, eq145_e1843_d_n19, eq145_e1843_d_n20, eq145_e1843_d_n21, eq145_e1843_d_n22, eq145_e1843_q, eq145_e1843_q_d_n0, eq145_e1843_q_d_n1, eq145_e1843_q_d_n2, eq145_e1843_q_d_n3, eq145_e1843_q_d_n4, eq145_e1843_q_d_n5, eq145_e1843_q_d_n6, eq145_e1843_q_d_n7, eq145_e1843_q_d_n8, eq145_e1843_q_d_n9, eq145_e1843_q_d_n10, eq145_e1843_q_d_n11, eq145_e1843_q_d_n12, eq145_e1843_q_d_n13, eq145_e1843_q_d_n14, eq145_e1843_q_d_n15, eq145_e1843_q_d_n16, eq145_e1843_q_d_n17, eq145_e1843_q_d_n18, eq145_e1843_q_d_n19, eq145_e1843_q_d_n20, eq145_e1843_q_d_n21, eq145_e1843_q_d_n22,) = {
    if (((s.v[580] != 0.0) && (s.v[581] != 0.0)) && (s.v[582] != 0.0)) {
        let eq145_e1840_q: f64 = s.v[252];
        let eq145_e1841: f64 = (p.p7 * s.v[252]);
        let eq145_e1841_d_n0: f64 = (p.p7 * s.dn[252][0]);
        let eq145_e1841_d_n1: f64 = (p.p7 * s.dn[252][1]);
        let eq145_e1841_d_n2: f64 = (p.p7 * s.dn[252][2]);
        let eq145_e1841_d_n3: f64 = (p.p7 * s.dn[252][3]);
        let eq145_e1841_d_n4: f64 = (p.p7 * s.dn[252][4]);
        let eq145_e1841_d_n5: f64 = (p.p7 * s.dn[252][5]);
        let eq145_e1841_d_n6: f64 = (p.p7 * s.dn[252][6]);
        let eq145_e1841_d_n7: f64 = (p.p7 * s.dn[252][7]);
        let eq145_e1841_d_n8: f64 = (p.p7 * s.dn[252][8]);
        let eq145_e1841_d_n9: f64 = (p.p7 * s.dn[252][9]);
        let eq145_e1841_d_n10: f64 = (p.p7 * s.dn[252][10]);
        let eq145_e1841_d_n11: f64 = (p.p7 * s.dn[252][11]);
        let eq145_e1841_d_n12: f64 = (p.p7 * s.dn[252][12]);
        let eq145_e1841_d_n13: f64 = (p.p7 * s.dn[252][13]);
        let eq145_e1841_d_n14: f64 = (p.p7 * s.dn[252][14]);
        let eq145_e1841_d_n15: f64 = (p.p7 * s.dn[252][15]);
        let eq145_e1841_d_n16: f64 = (p.p7 * s.dn[252][16]);
        let eq145_e1841_d_n17: f64 = (p.p7 * s.dn[252][17]);
        let eq145_e1841_d_n18: f64 = (p.p7 * s.dn[252][18]);
        let eq145_e1841_d_n19: f64 = (p.p7 * s.dn[252][19]);
        let eq145_e1841_d_n20: f64 = (p.p7 * s.dn[252][20]);
        let eq145_e1841_d_n21: f64 = (p.p7 * s.dn[252][21]);
        let eq145_e1841_d_n22: f64 = (p.p7 * s.dn[252][22]);
        let eq145_e1841_q: f64 = (p.p7 * eq145_e1840_q);
        let eq145_e1841_q_d_n0: f64 = (p.p7 * s.dn[252][0]);
        let eq145_e1841_q_d_n1: f64 = (p.p7 * s.dn[252][1]);
        let eq145_e1841_q_d_n2: f64 = (p.p7 * s.dn[252][2]);
        let eq145_e1841_q_d_n3: f64 = (p.p7 * s.dn[252][3]);
        let eq145_e1841_q_d_n4: f64 = (p.p7 * s.dn[252][4]);
        let eq145_e1841_q_d_n5: f64 = (p.p7 * s.dn[252][5]);
        let eq145_e1841_q_d_n6: f64 = (p.p7 * s.dn[252][6]);
        let eq145_e1841_q_d_n7: f64 = (p.p7 * s.dn[252][7]);
        let eq145_e1841_q_d_n8: f64 = (p.p7 * s.dn[252][8]);
        let eq145_e1841_q_d_n9: f64 = (p.p7 * s.dn[252][9]);
        let eq145_e1841_q_d_n10: f64 = (p.p7 * s.dn[252][10]);
        let eq145_e1841_q_d_n11: f64 = (p.p7 * s.dn[252][11]);
        let eq145_e1841_q_d_n12: f64 = (p.p7 * s.dn[252][12]);
        let eq145_e1841_q_d_n13: f64 = (p.p7 * s.dn[252][13]);
        let eq145_e1841_q_d_n14: f64 = (p.p7 * s.dn[252][14]);
        let eq145_e1841_q_d_n15: f64 = (p.p7 * s.dn[252][15]);
        let eq145_e1841_q_d_n16: f64 = (p.p7 * s.dn[252][16]);
        let eq145_e1841_q_d_n17: f64 = (p.p7 * s.dn[252][17]);
        let eq145_e1841_q_d_n18: f64 = (p.p7 * s.dn[252][18]);
        let eq145_e1841_q_d_n19: f64 = (p.p7 * s.dn[252][19]);
        let eq145_e1841_q_d_n20: f64 = (p.p7 * s.dn[252][20]);
        let eq145_e1841_q_d_n21: f64 = (p.p7 * s.dn[252][21]);
        let eq145_e1841_q_d_n22: f64 = (p.p7 * s.dn[252][22]);
        (eq145_e1841, eq145_e1841_d_n0, eq145_e1841_d_n1, eq145_e1841_d_n2, eq145_e1841_d_n3, eq145_e1841_d_n4, eq145_e1841_d_n5, eq145_e1841_d_n6, eq145_e1841_d_n7, eq145_e1841_d_n8, eq145_e1841_d_n9, eq145_e1841_d_n10, eq145_e1841_d_n11, eq145_e1841_d_n12, eq145_e1841_d_n13, eq145_e1841_d_n14, eq145_e1841_d_n15, eq145_e1841_d_n16, eq145_e1841_d_n17, eq145_e1841_d_n18, eq145_e1841_d_n19, eq145_e1841_d_n20, eq145_e1841_d_n21, eq145_e1841_d_n22, eq145_e1841_q, eq145_e1841_q_d_n0, eq145_e1841_q_d_n1, eq145_e1841_q_d_n2, eq145_e1841_q_d_n3, eq145_e1841_q_d_n4, eq145_e1841_q_d_n5, eq145_e1841_q_d_n6, eq145_e1841_q_d_n7, eq145_e1841_q_d_n8, eq145_e1841_q_d_n9, eq145_e1841_q_d_n10, eq145_e1841_q_d_n11, eq145_e1841_q_d_n12, eq145_e1841_q_d_n13, eq145_e1841_q_d_n14, eq145_e1841_q_d_n15, eq145_e1841_q_d_n16, eq145_e1841_q_d_n17, eq145_e1841_q_d_n18, eq145_e1841_q_d_n19, eq145_e1841_q_d_n20, eq145_e1841_q_d_n21, eq145_e1841_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq145_reactive_node_derivatives: [f64; 23] = [eq145_e1843_q_d_n0, eq145_e1843_q_d_n1, eq145_e1843_q_d_n2, eq145_e1843_q_d_n3, eq145_e1843_q_d_n4, eq145_e1843_q_d_n5, eq145_e1843_q_d_n6, eq145_e1843_q_d_n7, eq145_e1843_q_d_n8, eq145_e1843_q_d_n9, eq145_e1843_q_d_n10, eq145_e1843_q_d_n11, eq145_e1843_q_d_n12, eq145_e1843_q_d_n13, eq145_e1843_q_d_n14, eq145_e1843_q_d_n15, eq145_e1843_q_d_n16, eq145_e1843_q_d_n17, eq145_e1843_q_d_n18, eq145_e1843_q_d_n19, eq145_e1843_q_d_n20, eq145_e1843_q_d_n21, eq145_e1843_q_d_n22];
        let eq145_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[15]),
            &nodes,
            &eq145_reactive_node_derivatives,
            &branches,
            &eq145_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_146_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq146_e1856, eq146_e1856_d_n0, eq146_e1856_d_n1, eq146_e1856_d_n2, eq146_e1856_d_n3, eq146_e1856_d_n4, eq146_e1856_d_n5, eq146_e1856_d_n6, eq146_e1856_d_n7, eq146_e1856_d_n8, eq146_e1856_d_n9, eq146_e1856_d_n10, eq146_e1856_d_n11, eq146_e1856_d_n12, eq146_e1856_d_n13, eq146_e1856_d_n14, eq146_e1856_d_n15, eq146_e1856_d_n16, eq146_e1856_d_n17, eq146_e1856_d_n18, eq146_e1856_d_n19, eq146_e1856_d_n20, eq146_e1856_d_n21, eq146_e1856_d_n22, eq146_e1856_q, eq146_e1856_q_d_n0, eq146_e1856_q_d_n1, eq146_e1856_q_d_n2, eq146_e1856_q_d_n3, eq146_e1856_q_d_n4, eq146_e1856_q_d_n5, eq146_e1856_q_d_n6, eq146_e1856_q_d_n7, eq146_e1856_q_d_n8, eq146_e1856_q_d_n9, eq146_e1856_q_d_n10, eq146_e1856_q_d_n11, eq146_e1856_q_d_n12, eq146_e1856_q_d_n13, eq146_e1856_q_d_n14, eq146_e1856_q_d_n15, eq146_e1856_q_d_n16, eq146_e1856_q_d_n17, eq146_e1856_q_d_n18, eq146_e1856_q_d_n19, eq146_e1856_q_d_n20, eq146_e1856_q_d_n21, eq146_e1856_q_d_n22,) = {
    if (((s.v[580] != 0.0) && (s.v[581] != 0.0)) && (s.v[582] != 0.0)) {
        let eq146_e1851: f64 = (p.p7 * p.p247);
        let eq146_e1853_q: f64 = s.v[252];
        let eq146_e1854: f64 = (eq146_e1851 * s.v[252]);
        let eq146_e1854_d_n0: f64 = (eq146_e1851 * s.dn[252][0]);
        let eq146_e1854_d_n1: f64 = (eq146_e1851 * s.dn[252][1]);
        let eq146_e1854_d_n2: f64 = (eq146_e1851 * s.dn[252][2]);
        let eq146_e1854_d_n3: f64 = (eq146_e1851 * s.dn[252][3]);
        let eq146_e1854_d_n4: f64 = (eq146_e1851 * s.dn[252][4]);
        let eq146_e1854_d_n5: f64 = (eq146_e1851 * s.dn[252][5]);
        let eq146_e1854_d_n6: f64 = (eq146_e1851 * s.dn[252][6]);
        let eq146_e1854_d_n7: f64 = (eq146_e1851 * s.dn[252][7]);
        let eq146_e1854_d_n8: f64 = (eq146_e1851 * s.dn[252][8]);
        let eq146_e1854_d_n9: f64 = (eq146_e1851 * s.dn[252][9]);
        let eq146_e1854_d_n10: f64 = (eq146_e1851 * s.dn[252][10]);
        let eq146_e1854_d_n11: f64 = (eq146_e1851 * s.dn[252][11]);
        let eq146_e1854_d_n12: f64 = (eq146_e1851 * s.dn[252][12]);
        let eq146_e1854_d_n13: f64 = (eq146_e1851 * s.dn[252][13]);
        let eq146_e1854_d_n14: f64 = (eq146_e1851 * s.dn[252][14]);
        let eq146_e1854_d_n15: f64 = (eq146_e1851 * s.dn[252][15]);
        let eq146_e1854_d_n16: f64 = (eq146_e1851 * s.dn[252][16]);
        let eq146_e1854_d_n17: f64 = (eq146_e1851 * s.dn[252][17]);
        let eq146_e1854_d_n18: f64 = (eq146_e1851 * s.dn[252][18]);
        let eq146_e1854_d_n19: f64 = (eq146_e1851 * s.dn[252][19]);
        let eq146_e1854_d_n20: f64 = (eq146_e1851 * s.dn[252][20]);
        let eq146_e1854_d_n21: f64 = (eq146_e1851 * s.dn[252][21]);
        let eq146_e1854_d_n22: f64 = (eq146_e1851 * s.dn[252][22]);
        let eq146_e1854_q: f64 = (eq146_e1851 * eq146_e1853_q);
        let eq146_e1854_q_d_n0: f64 = (eq146_e1851 * s.dn[252][0]);
        let eq146_e1854_q_d_n1: f64 = (eq146_e1851 * s.dn[252][1]);
        let eq146_e1854_q_d_n2: f64 = (eq146_e1851 * s.dn[252][2]);
        let eq146_e1854_q_d_n3: f64 = (eq146_e1851 * s.dn[252][3]);
        let eq146_e1854_q_d_n4: f64 = (eq146_e1851 * s.dn[252][4]);
        let eq146_e1854_q_d_n5: f64 = (eq146_e1851 * s.dn[252][5]);
        let eq146_e1854_q_d_n6: f64 = (eq146_e1851 * s.dn[252][6]);
        let eq146_e1854_q_d_n7: f64 = (eq146_e1851 * s.dn[252][7]);
        let eq146_e1854_q_d_n8: f64 = (eq146_e1851 * s.dn[252][8]);
        let eq146_e1854_q_d_n9: f64 = (eq146_e1851 * s.dn[252][9]);
        let eq146_e1854_q_d_n10: f64 = (eq146_e1851 * s.dn[252][10]);
        let eq146_e1854_q_d_n11: f64 = (eq146_e1851 * s.dn[252][11]);
        let eq146_e1854_q_d_n12: f64 = (eq146_e1851 * s.dn[252][12]);
        let eq146_e1854_q_d_n13: f64 = (eq146_e1851 * s.dn[252][13]);
        let eq146_e1854_q_d_n14: f64 = (eq146_e1851 * s.dn[252][14]);
        let eq146_e1854_q_d_n15: f64 = (eq146_e1851 * s.dn[252][15]);
        let eq146_e1854_q_d_n16: f64 = (eq146_e1851 * s.dn[252][16]);
        let eq146_e1854_q_d_n17: f64 = (eq146_e1851 * s.dn[252][17]);
        let eq146_e1854_q_d_n18: f64 = (eq146_e1851 * s.dn[252][18]);
        let eq146_e1854_q_d_n19: f64 = (eq146_e1851 * s.dn[252][19]);
        let eq146_e1854_q_d_n20: f64 = (eq146_e1851 * s.dn[252][20]);
        let eq146_e1854_q_d_n21: f64 = (eq146_e1851 * s.dn[252][21]);
        let eq146_e1854_q_d_n22: f64 = (eq146_e1851 * s.dn[252][22]);
        (eq146_e1854, eq146_e1854_d_n0, eq146_e1854_d_n1, eq146_e1854_d_n2, eq146_e1854_d_n3, eq146_e1854_d_n4, eq146_e1854_d_n5, eq146_e1854_d_n6, eq146_e1854_d_n7, eq146_e1854_d_n8, eq146_e1854_d_n9, eq146_e1854_d_n10, eq146_e1854_d_n11, eq146_e1854_d_n12, eq146_e1854_d_n13, eq146_e1854_d_n14, eq146_e1854_d_n15, eq146_e1854_d_n16, eq146_e1854_d_n17, eq146_e1854_d_n18, eq146_e1854_d_n19, eq146_e1854_d_n20, eq146_e1854_d_n21, eq146_e1854_d_n22, eq146_e1854_q, eq146_e1854_q_d_n0, eq146_e1854_q_d_n1, eq146_e1854_q_d_n2, eq146_e1854_q_d_n3, eq146_e1854_q_d_n4, eq146_e1854_q_d_n5, eq146_e1854_q_d_n6, eq146_e1854_q_d_n7, eq146_e1854_q_d_n8, eq146_e1854_q_d_n9, eq146_e1854_q_d_n10, eq146_e1854_q_d_n11, eq146_e1854_q_d_n12, eq146_e1854_q_d_n13, eq146_e1854_q_d_n14, eq146_e1854_q_d_n15, eq146_e1854_q_d_n16, eq146_e1854_q_d_n17, eq146_e1854_q_d_n18, eq146_e1854_q_d_n19, eq146_e1854_q_d_n20, eq146_e1854_q_d_n21, eq146_e1854_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq146_reactive_node_derivatives: [f64; 23] = [eq146_e1856_q_d_n0, eq146_e1856_q_d_n1, eq146_e1856_q_d_n2, eq146_e1856_q_d_n3, eq146_e1856_q_d_n4, eq146_e1856_q_d_n5, eq146_e1856_q_d_n6, eq146_e1856_q_d_n7, eq146_e1856_q_d_n8, eq146_e1856_q_d_n9, eq146_e1856_q_d_n10, eq146_e1856_q_d_n11, eq146_e1856_q_d_n12, eq146_e1856_q_d_n13, eq146_e1856_q_d_n14, eq146_e1856_q_d_n15, eq146_e1856_q_d_n16, eq146_e1856_q_d_n17, eq146_e1856_q_d_n18, eq146_e1856_q_d_n19, eq146_e1856_q_d_n20, eq146_e1856_q_d_n21, eq146_e1856_q_d_n22];
        let eq146_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            &nodes,
            &eq146_reactive_node_derivatives,
            &branches,
            &eq146_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_147_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq147_e1868, eq147_e1868_d_n0, eq147_e1868_d_n1, eq147_e1868_d_n2, eq147_e1868_d_n3, eq147_e1868_d_n4, eq147_e1868_d_n5, eq147_e1868_d_n6, eq147_e1868_d_n7, eq147_e1868_d_n8, eq147_e1868_d_n9, eq147_e1868_d_n10, eq147_e1868_d_n11, eq147_e1868_d_n12, eq147_e1868_d_n13, eq147_e1868_d_n14, eq147_e1868_d_n15, eq147_e1868_d_n16, eq147_e1868_d_n17, eq147_e1868_d_n18, eq147_e1868_d_n19, eq147_e1868_d_n20, eq147_e1868_d_n21, eq147_e1868_d_n22, eq147_e1868_q, eq147_e1868_q_d_n0, eq147_e1868_q_d_n1, eq147_e1868_q_d_n2, eq147_e1868_q_d_n3, eq147_e1868_q_d_n4, eq147_e1868_q_d_n5, eq147_e1868_q_d_n6, eq147_e1868_q_d_n7, eq147_e1868_q_d_n8, eq147_e1868_q_d_n9, eq147_e1868_q_d_n10, eq147_e1868_q_d_n11, eq147_e1868_q_d_n12, eq147_e1868_q_d_n13, eq147_e1868_q_d_n14, eq147_e1868_q_d_n15, eq147_e1868_q_d_n16, eq147_e1868_q_d_n17, eq147_e1868_q_d_n18, eq147_e1868_q_d_n19, eq147_e1868_q_d_n20, eq147_e1868_q_d_n21, eq147_e1868_q_d_n22,) = {
    if (((s.v[580] != 0.0) && (s.v[581] != 0.0)) && (!(s.v[582] != 0.0))) {
        let eq147_e1865_q: f64 = s.v[252];
        let eq147_e1866: f64 = (p.p7 * s.v[252]);
        let eq147_e1866_d_n0: f64 = (p.p7 * s.dn[252][0]);
        let eq147_e1866_d_n1: f64 = (p.p7 * s.dn[252][1]);
        let eq147_e1866_d_n2: f64 = (p.p7 * s.dn[252][2]);
        let eq147_e1866_d_n3: f64 = (p.p7 * s.dn[252][3]);
        let eq147_e1866_d_n4: f64 = (p.p7 * s.dn[252][4]);
        let eq147_e1866_d_n5: f64 = (p.p7 * s.dn[252][5]);
        let eq147_e1866_d_n6: f64 = (p.p7 * s.dn[252][6]);
        let eq147_e1866_d_n7: f64 = (p.p7 * s.dn[252][7]);
        let eq147_e1866_d_n8: f64 = (p.p7 * s.dn[252][8]);
        let eq147_e1866_d_n9: f64 = (p.p7 * s.dn[252][9]);
        let eq147_e1866_d_n10: f64 = (p.p7 * s.dn[252][10]);
        let eq147_e1866_d_n11: f64 = (p.p7 * s.dn[252][11]);
        let eq147_e1866_d_n12: f64 = (p.p7 * s.dn[252][12]);
        let eq147_e1866_d_n13: f64 = (p.p7 * s.dn[252][13]);
        let eq147_e1866_d_n14: f64 = (p.p7 * s.dn[252][14]);
        let eq147_e1866_d_n15: f64 = (p.p7 * s.dn[252][15]);
        let eq147_e1866_d_n16: f64 = (p.p7 * s.dn[252][16]);
        let eq147_e1866_d_n17: f64 = (p.p7 * s.dn[252][17]);
        let eq147_e1866_d_n18: f64 = (p.p7 * s.dn[252][18]);
        let eq147_e1866_d_n19: f64 = (p.p7 * s.dn[252][19]);
        let eq147_e1866_d_n20: f64 = (p.p7 * s.dn[252][20]);
        let eq147_e1866_d_n21: f64 = (p.p7 * s.dn[252][21]);
        let eq147_e1866_d_n22: f64 = (p.p7 * s.dn[252][22]);
        let eq147_e1866_q: f64 = (p.p7 * eq147_e1865_q);
        let eq147_e1866_q_d_n0: f64 = (p.p7 * s.dn[252][0]);
        let eq147_e1866_q_d_n1: f64 = (p.p7 * s.dn[252][1]);
        let eq147_e1866_q_d_n2: f64 = (p.p7 * s.dn[252][2]);
        let eq147_e1866_q_d_n3: f64 = (p.p7 * s.dn[252][3]);
        let eq147_e1866_q_d_n4: f64 = (p.p7 * s.dn[252][4]);
        let eq147_e1866_q_d_n5: f64 = (p.p7 * s.dn[252][5]);
        let eq147_e1866_q_d_n6: f64 = (p.p7 * s.dn[252][6]);
        let eq147_e1866_q_d_n7: f64 = (p.p7 * s.dn[252][7]);
        let eq147_e1866_q_d_n8: f64 = (p.p7 * s.dn[252][8]);
        let eq147_e1866_q_d_n9: f64 = (p.p7 * s.dn[252][9]);
        let eq147_e1866_q_d_n10: f64 = (p.p7 * s.dn[252][10]);
        let eq147_e1866_q_d_n11: f64 = (p.p7 * s.dn[252][11]);
        let eq147_e1866_q_d_n12: f64 = (p.p7 * s.dn[252][12]);
        let eq147_e1866_q_d_n13: f64 = (p.p7 * s.dn[252][13]);
        let eq147_e1866_q_d_n14: f64 = (p.p7 * s.dn[252][14]);
        let eq147_e1866_q_d_n15: f64 = (p.p7 * s.dn[252][15]);
        let eq147_e1866_q_d_n16: f64 = (p.p7 * s.dn[252][16]);
        let eq147_e1866_q_d_n17: f64 = (p.p7 * s.dn[252][17]);
        let eq147_e1866_q_d_n18: f64 = (p.p7 * s.dn[252][18]);
        let eq147_e1866_q_d_n19: f64 = (p.p7 * s.dn[252][19]);
        let eq147_e1866_q_d_n20: f64 = (p.p7 * s.dn[252][20]);
        let eq147_e1866_q_d_n21: f64 = (p.p7 * s.dn[252][21]);
        let eq147_e1866_q_d_n22: f64 = (p.p7 * s.dn[252][22]);
        (eq147_e1866, eq147_e1866_d_n0, eq147_e1866_d_n1, eq147_e1866_d_n2, eq147_e1866_d_n3, eq147_e1866_d_n4, eq147_e1866_d_n5, eq147_e1866_d_n6, eq147_e1866_d_n7, eq147_e1866_d_n8, eq147_e1866_d_n9, eq147_e1866_d_n10, eq147_e1866_d_n11, eq147_e1866_d_n12, eq147_e1866_d_n13, eq147_e1866_d_n14, eq147_e1866_d_n15, eq147_e1866_d_n16, eq147_e1866_d_n17, eq147_e1866_d_n18, eq147_e1866_d_n19, eq147_e1866_d_n20, eq147_e1866_d_n21, eq147_e1866_d_n22, eq147_e1866_q, eq147_e1866_q_d_n0, eq147_e1866_q_d_n1, eq147_e1866_q_d_n2, eq147_e1866_q_d_n3, eq147_e1866_q_d_n4, eq147_e1866_q_d_n5, eq147_e1866_q_d_n6, eq147_e1866_q_d_n7, eq147_e1866_q_d_n8, eq147_e1866_q_d_n9, eq147_e1866_q_d_n10, eq147_e1866_q_d_n11, eq147_e1866_q_d_n12, eq147_e1866_q_d_n13, eq147_e1866_q_d_n14, eq147_e1866_q_d_n15, eq147_e1866_q_d_n16, eq147_e1866_q_d_n17, eq147_e1866_q_d_n18, eq147_e1866_q_d_n19, eq147_e1866_q_d_n20, eq147_e1866_q_d_n21, eq147_e1866_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq147_reactive_node_derivatives: [f64; 23] = [eq147_e1868_q_d_n0, eq147_e1868_q_d_n1, eq147_e1868_q_d_n2, eq147_e1868_q_d_n3, eq147_e1868_q_d_n4, eq147_e1868_q_d_n5, eq147_e1868_q_d_n6, eq147_e1868_q_d_n7, eq147_e1868_q_d_n8, eq147_e1868_q_d_n9, eq147_e1868_q_d_n10, eq147_e1868_q_d_n11, eq147_e1868_q_d_n12, eq147_e1868_q_d_n13, eq147_e1868_q_d_n14, eq147_e1868_q_d_n15, eq147_e1868_q_d_n16, eq147_e1868_q_d_n17, eq147_e1868_q_d_n18, eq147_e1868_q_d_n19, eq147_e1868_q_d_n20, eq147_e1868_q_d_n21, eq147_e1868_q_d_n22];
        let eq147_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            &nodes,
            &eq147_reactive_node_derivatives,
            &branches,
            &eq147_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_148_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq148_e1882, eq148_e1882_d_n0, eq148_e1882_d_n1, eq148_e1882_d_n2, eq148_e1882_d_n3, eq148_e1882_d_n4, eq148_e1882_d_n5, eq148_e1882_d_n6, eq148_e1882_d_n7, eq148_e1882_d_n8, eq148_e1882_d_n9, eq148_e1882_d_n10, eq148_e1882_d_n11, eq148_e1882_d_n12, eq148_e1882_d_n13, eq148_e1882_d_n14, eq148_e1882_d_n15, eq148_e1882_d_n16, eq148_e1882_d_n17, eq148_e1882_d_n18, eq148_e1882_d_n19, eq148_e1882_d_n20, eq148_e1882_d_n21, eq148_e1882_d_n22, eq148_e1882_q, eq148_e1882_q_d_n0, eq148_e1882_q_d_n1, eq148_e1882_q_d_n2, eq148_e1882_q_d_n3, eq148_e1882_q_d_n4, eq148_e1882_q_d_n5, eq148_e1882_q_d_n6, eq148_e1882_q_d_n7, eq148_e1882_q_d_n8, eq148_e1882_q_d_n9, eq148_e1882_q_d_n10, eq148_e1882_q_d_n11, eq148_e1882_q_d_n12, eq148_e1882_q_d_n13, eq148_e1882_q_d_n14, eq148_e1882_q_d_n15, eq148_e1882_q_d_n16, eq148_e1882_q_d_n17, eq148_e1882_q_d_n18, eq148_e1882_q_d_n19, eq148_e1882_q_d_n20, eq148_e1882_q_d_n21, eq148_e1882_q_d_n22,) = {
    if (((s.v[580] != 0.0) && (s.v[581] != 0.0)) && (!(s.v[582] != 0.0))) {
        let eq148_e1877: f64 = (p.p7 * p.p247);
        let eq148_e1879_q: f64 = s.v[252];
        let eq148_e1880: f64 = (eq148_e1877 * s.v[252]);
        let eq148_e1880_d_n0: f64 = (eq148_e1877 * s.dn[252][0]);
        let eq148_e1880_d_n1: f64 = (eq148_e1877 * s.dn[252][1]);
        let eq148_e1880_d_n2: f64 = (eq148_e1877 * s.dn[252][2]);
        let eq148_e1880_d_n3: f64 = (eq148_e1877 * s.dn[252][3]);
        let eq148_e1880_d_n4: f64 = (eq148_e1877 * s.dn[252][4]);
        let eq148_e1880_d_n5: f64 = (eq148_e1877 * s.dn[252][5]);
        let eq148_e1880_d_n6: f64 = (eq148_e1877 * s.dn[252][6]);
        let eq148_e1880_d_n7: f64 = (eq148_e1877 * s.dn[252][7]);
        let eq148_e1880_d_n8: f64 = (eq148_e1877 * s.dn[252][8]);
        let eq148_e1880_d_n9: f64 = (eq148_e1877 * s.dn[252][9]);
        let eq148_e1880_d_n10: f64 = (eq148_e1877 * s.dn[252][10]);
        let eq148_e1880_d_n11: f64 = (eq148_e1877 * s.dn[252][11]);
        let eq148_e1880_d_n12: f64 = (eq148_e1877 * s.dn[252][12]);
        let eq148_e1880_d_n13: f64 = (eq148_e1877 * s.dn[252][13]);
        let eq148_e1880_d_n14: f64 = (eq148_e1877 * s.dn[252][14]);
        let eq148_e1880_d_n15: f64 = (eq148_e1877 * s.dn[252][15]);
        let eq148_e1880_d_n16: f64 = (eq148_e1877 * s.dn[252][16]);
        let eq148_e1880_d_n17: f64 = (eq148_e1877 * s.dn[252][17]);
        let eq148_e1880_d_n18: f64 = (eq148_e1877 * s.dn[252][18]);
        let eq148_e1880_d_n19: f64 = (eq148_e1877 * s.dn[252][19]);
        let eq148_e1880_d_n20: f64 = (eq148_e1877 * s.dn[252][20]);
        let eq148_e1880_d_n21: f64 = (eq148_e1877 * s.dn[252][21]);
        let eq148_e1880_d_n22: f64 = (eq148_e1877 * s.dn[252][22]);
        let eq148_e1880_q: f64 = (eq148_e1877 * eq148_e1879_q);
        let eq148_e1880_q_d_n0: f64 = (eq148_e1877 * s.dn[252][0]);
        let eq148_e1880_q_d_n1: f64 = (eq148_e1877 * s.dn[252][1]);
        let eq148_e1880_q_d_n2: f64 = (eq148_e1877 * s.dn[252][2]);
        let eq148_e1880_q_d_n3: f64 = (eq148_e1877 * s.dn[252][3]);
        let eq148_e1880_q_d_n4: f64 = (eq148_e1877 * s.dn[252][4]);
        let eq148_e1880_q_d_n5: f64 = (eq148_e1877 * s.dn[252][5]);
        let eq148_e1880_q_d_n6: f64 = (eq148_e1877 * s.dn[252][6]);
        let eq148_e1880_q_d_n7: f64 = (eq148_e1877 * s.dn[252][7]);
        let eq148_e1880_q_d_n8: f64 = (eq148_e1877 * s.dn[252][8]);
        let eq148_e1880_q_d_n9: f64 = (eq148_e1877 * s.dn[252][9]);
        let eq148_e1880_q_d_n10: f64 = (eq148_e1877 * s.dn[252][10]);
        let eq148_e1880_q_d_n11: f64 = (eq148_e1877 * s.dn[252][11]);
        let eq148_e1880_q_d_n12: f64 = (eq148_e1877 * s.dn[252][12]);
        let eq148_e1880_q_d_n13: f64 = (eq148_e1877 * s.dn[252][13]);
        let eq148_e1880_q_d_n14: f64 = (eq148_e1877 * s.dn[252][14]);
        let eq148_e1880_q_d_n15: f64 = (eq148_e1877 * s.dn[252][15]);
        let eq148_e1880_q_d_n16: f64 = (eq148_e1877 * s.dn[252][16]);
        let eq148_e1880_q_d_n17: f64 = (eq148_e1877 * s.dn[252][17]);
        let eq148_e1880_q_d_n18: f64 = (eq148_e1877 * s.dn[252][18]);
        let eq148_e1880_q_d_n19: f64 = (eq148_e1877 * s.dn[252][19]);
        let eq148_e1880_q_d_n20: f64 = (eq148_e1877 * s.dn[252][20]);
        let eq148_e1880_q_d_n21: f64 = (eq148_e1877 * s.dn[252][21]);
        let eq148_e1880_q_d_n22: f64 = (eq148_e1877 * s.dn[252][22]);
        (eq148_e1880, eq148_e1880_d_n0, eq148_e1880_d_n1, eq148_e1880_d_n2, eq148_e1880_d_n3, eq148_e1880_d_n4, eq148_e1880_d_n5, eq148_e1880_d_n6, eq148_e1880_d_n7, eq148_e1880_d_n8, eq148_e1880_d_n9, eq148_e1880_d_n10, eq148_e1880_d_n11, eq148_e1880_d_n12, eq148_e1880_d_n13, eq148_e1880_d_n14, eq148_e1880_d_n15, eq148_e1880_d_n16, eq148_e1880_d_n17, eq148_e1880_d_n18, eq148_e1880_d_n19, eq148_e1880_d_n20, eq148_e1880_d_n21, eq148_e1880_d_n22, eq148_e1880_q, eq148_e1880_q_d_n0, eq148_e1880_q_d_n1, eq148_e1880_q_d_n2, eq148_e1880_q_d_n3, eq148_e1880_q_d_n4, eq148_e1880_q_d_n5, eq148_e1880_q_d_n6, eq148_e1880_q_d_n7, eq148_e1880_q_d_n8, eq148_e1880_q_d_n9, eq148_e1880_q_d_n10, eq148_e1880_q_d_n11, eq148_e1880_q_d_n12, eq148_e1880_q_d_n13, eq148_e1880_q_d_n14, eq148_e1880_q_d_n15, eq148_e1880_q_d_n16, eq148_e1880_q_d_n17, eq148_e1880_q_d_n18, eq148_e1880_q_d_n19, eq148_e1880_q_d_n20, eq148_e1880_q_d_n21, eq148_e1880_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq148_reactive_node_derivatives: [f64; 23] = [eq148_e1882_q_d_n0, eq148_e1882_q_d_n1, eq148_e1882_q_d_n2, eq148_e1882_q_d_n3, eq148_e1882_q_d_n4, eq148_e1882_q_d_n5, eq148_e1882_q_d_n6, eq148_e1882_q_d_n7, eq148_e1882_q_d_n8, eq148_e1882_q_d_n9, eq148_e1882_q_d_n10, eq148_e1882_q_d_n11, eq148_e1882_q_d_n12, eq148_e1882_q_d_n13, eq148_e1882_q_d_n14, eq148_e1882_q_d_n15, eq148_e1882_q_d_n16, eq148_e1882_q_d_n17, eq148_e1882_q_d_n18, eq148_e1882_q_d_n19, eq148_e1882_q_d_n20, eq148_e1882_q_d_n21, eq148_e1882_q_d_n22];
        let eq148_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[15]),
            &nodes,
            &eq148_reactive_node_derivatives,
            &branches,
            &eq148_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_149_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq149_e1893, eq149_e1893_d_n0, eq149_e1893_d_n1, eq149_e1893_d_n2, eq149_e1893_d_n3, eq149_e1893_d_n4, eq149_e1893_d_n5, eq149_e1893_d_n6, eq149_e1893_d_n7, eq149_e1893_d_n8, eq149_e1893_d_n9, eq149_e1893_d_n10, eq149_e1893_d_n11, eq149_e1893_d_n12, eq149_e1893_d_n13, eq149_e1893_d_n14, eq149_e1893_d_n15, eq149_e1893_d_n16, eq149_e1893_d_n17, eq149_e1893_d_n18, eq149_e1893_d_n19, eq149_e1893_d_n20, eq149_e1893_d_n21, eq149_e1893_d_n22, eq149_e1893_q, eq149_e1893_q_d_n0, eq149_e1893_q_d_n1, eq149_e1893_q_d_n2, eq149_e1893_q_d_n3, eq149_e1893_q_d_n4, eq149_e1893_q_d_n5, eq149_e1893_q_d_n6, eq149_e1893_q_d_n7, eq149_e1893_q_d_n8, eq149_e1893_q_d_n9, eq149_e1893_q_d_n10, eq149_e1893_q_d_n11, eq149_e1893_q_d_n12, eq149_e1893_q_d_n13, eq149_e1893_q_d_n14, eq149_e1893_q_d_n15, eq149_e1893_q_d_n16, eq149_e1893_q_d_n17, eq149_e1893_q_d_n18, eq149_e1893_q_d_n19, eq149_e1893_q_d_n20, eq149_e1893_q_d_n21, eq149_e1893_q_d_n22,) = {
    if ((s.v[580] != 0.0) && (s.v[581] != 0.0)) {
        let eq149_e1889: f64 = (p.p252 * s.v[252]);
        let eq149_e1889_d_n0: f64 = (p.p252 * s.dn[252][0]);
        let eq149_e1889_d_n1: f64 = (p.p252 * s.dn[252][1]);
        let eq149_e1889_d_n2: f64 = (p.p252 * s.dn[252][2]);
        let eq149_e1889_d_n3: f64 = (p.p252 * s.dn[252][3]);
        let eq149_e1889_d_n4: f64 = (p.p252 * s.dn[252][4]);
        let eq149_e1889_d_n5: f64 = (p.p252 * s.dn[252][5]);
        let eq149_e1889_d_n6: f64 = (p.p252 * s.dn[252][6]);
        let eq149_e1889_d_n7: f64 = (p.p252 * s.dn[252][7]);
        let eq149_e1889_d_n8: f64 = (p.p252 * s.dn[252][8]);
        let eq149_e1889_d_n9: f64 = (p.p252 * s.dn[252][9]);
        let eq149_e1889_d_n10: f64 = (p.p252 * s.dn[252][10]);
        let eq149_e1889_d_n11: f64 = (p.p252 * s.dn[252][11]);
        let eq149_e1889_d_n12: f64 = (p.p252 * s.dn[252][12]);
        let eq149_e1889_d_n13: f64 = (p.p252 * s.dn[252][13]);
        let eq149_e1889_d_n14: f64 = (p.p252 * s.dn[252][14]);
        let eq149_e1889_d_n15: f64 = (p.p252 * s.dn[252][15]);
        let eq149_e1889_d_n16: f64 = (p.p252 * s.dn[252][16]);
        let eq149_e1889_d_n17: f64 = (p.p252 * s.dn[252][17]);
        let eq149_e1889_d_n18: f64 = (p.p252 * s.dn[252][18]);
        let eq149_e1889_d_n19: f64 = (p.p252 * s.dn[252][19]);
        let eq149_e1889_d_n20: f64 = (p.p252 * s.dn[252][20]);
        let eq149_e1889_d_n21: f64 = (p.p252 * s.dn[252][21]);
        let eq149_e1889_d_n22: f64 = (p.p252 * s.dn[252][22]);
        let eq149_e1890_q: f64 = eq149_e1889;
        let eq149_e1891: f64 = (p.p7 * eq149_e1889);
        let eq149_e1891_d_n0: f64 = (p.p7 * eq149_e1889_d_n0);
        let eq149_e1891_d_n1: f64 = (p.p7 * eq149_e1889_d_n1);
        let eq149_e1891_d_n2: f64 = (p.p7 * eq149_e1889_d_n2);
        let eq149_e1891_d_n3: f64 = (p.p7 * eq149_e1889_d_n3);
        let eq149_e1891_d_n4: f64 = (p.p7 * eq149_e1889_d_n4);
        let eq149_e1891_d_n5: f64 = (p.p7 * eq149_e1889_d_n5);
        let eq149_e1891_d_n6: f64 = (p.p7 * eq149_e1889_d_n6);
        let eq149_e1891_d_n7: f64 = (p.p7 * eq149_e1889_d_n7);
        let eq149_e1891_d_n8: f64 = (p.p7 * eq149_e1889_d_n8);
        let eq149_e1891_d_n9: f64 = (p.p7 * eq149_e1889_d_n9);
        let eq149_e1891_d_n10: f64 = (p.p7 * eq149_e1889_d_n10);
        let eq149_e1891_d_n11: f64 = (p.p7 * eq149_e1889_d_n11);
        let eq149_e1891_d_n12: f64 = (p.p7 * eq149_e1889_d_n12);
        let eq149_e1891_d_n13: f64 = (p.p7 * eq149_e1889_d_n13);
        let eq149_e1891_d_n14: f64 = (p.p7 * eq149_e1889_d_n14);
        let eq149_e1891_d_n15: f64 = (p.p7 * eq149_e1889_d_n15);
        let eq149_e1891_d_n16: f64 = (p.p7 * eq149_e1889_d_n16);
        let eq149_e1891_d_n17: f64 = (p.p7 * eq149_e1889_d_n17);
        let eq149_e1891_d_n18: f64 = (p.p7 * eq149_e1889_d_n18);
        let eq149_e1891_d_n19: f64 = (p.p7 * eq149_e1889_d_n19);
        let eq149_e1891_d_n20: f64 = (p.p7 * eq149_e1889_d_n20);
        let eq149_e1891_d_n21: f64 = (p.p7 * eq149_e1889_d_n21);
        let eq149_e1891_d_n22: f64 = (p.p7 * eq149_e1889_d_n22);
        let eq149_e1891_q: f64 = (p.p7 * eq149_e1890_q);
        let eq149_e1891_q_d_n0: f64 = (p.p7 * eq149_e1889_d_n0);
        let eq149_e1891_q_d_n1: f64 = (p.p7 * eq149_e1889_d_n1);
        let eq149_e1891_q_d_n2: f64 = (p.p7 * eq149_e1889_d_n2);
        let eq149_e1891_q_d_n3: f64 = (p.p7 * eq149_e1889_d_n3);
        let eq149_e1891_q_d_n4: f64 = (p.p7 * eq149_e1889_d_n4);
        let eq149_e1891_q_d_n5: f64 = (p.p7 * eq149_e1889_d_n5);
        let eq149_e1891_q_d_n6: f64 = (p.p7 * eq149_e1889_d_n6);
        let eq149_e1891_q_d_n7: f64 = (p.p7 * eq149_e1889_d_n7);
        let eq149_e1891_q_d_n8: f64 = (p.p7 * eq149_e1889_d_n8);
        let eq149_e1891_q_d_n9: f64 = (p.p7 * eq149_e1889_d_n9);
        let eq149_e1891_q_d_n10: f64 = (p.p7 * eq149_e1889_d_n10);
        let eq149_e1891_q_d_n11: f64 = (p.p7 * eq149_e1889_d_n11);
        let eq149_e1891_q_d_n12: f64 = (p.p7 * eq149_e1889_d_n12);
        let eq149_e1891_q_d_n13: f64 = (p.p7 * eq149_e1889_d_n13);
        let eq149_e1891_q_d_n14: f64 = (p.p7 * eq149_e1889_d_n14);
        let eq149_e1891_q_d_n15: f64 = (p.p7 * eq149_e1889_d_n15);
        let eq149_e1891_q_d_n16: f64 = (p.p7 * eq149_e1889_d_n16);
        let eq149_e1891_q_d_n17: f64 = (p.p7 * eq149_e1889_d_n17);
        let eq149_e1891_q_d_n18: f64 = (p.p7 * eq149_e1889_d_n18);
        let eq149_e1891_q_d_n19: f64 = (p.p7 * eq149_e1889_d_n19);
        let eq149_e1891_q_d_n20: f64 = (p.p7 * eq149_e1889_d_n20);
        let eq149_e1891_q_d_n21: f64 = (p.p7 * eq149_e1889_d_n21);
        let eq149_e1891_q_d_n22: f64 = (p.p7 * eq149_e1889_d_n22);
        (eq149_e1891, eq149_e1891_d_n0, eq149_e1891_d_n1, eq149_e1891_d_n2, eq149_e1891_d_n3, eq149_e1891_d_n4, eq149_e1891_d_n5, eq149_e1891_d_n6, eq149_e1891_d_n7, eq149_e1891_d_n8, eq149_e1891_d_n9, eq149_e1891_d_n10, eq149_e1891_d_n11, eq149_e1891_d_n12, eq149_e1891_d_n13, eq149_e1891_d_n14, eq149_e1891_d_n15, eq149_e1891_d_n16, eq149_e1891_d_n17, eq149_e1891_d_n18, eq149_e1891_d_n19, eq149_e1891_d_n20, eq149_e1891_d_n21, eq149_e1891_d_n22, eq149_e1891_q, eq149_e1891_q_d_n0, eq149_e1891_q_d_n1, eq149_e1891_q_d_n2, eq149_e1891_q_d_n3, eq149_e1891_q_d_n4, eq149_e1891_q_d_n5, eq149_e1891_q_d_n6, eq149_e1891_q_d_n7, eq149_e1891_q_d_n8, eq149_e1891_q_d_n9, eq149_e1891_q_d_n10, eq149_e1891_q_d_n11, eq149_e1891_q_d_n12, eq149_e1891_q_d_n13, eq149_e1891_q_d_n14, eq149_e1891_q_d_n15, eq149_e1891_q_d_n16, eq149_e1891_q_d_n17, eq149_e1891_q_d_n18, eq149_e1891_q_d_n19, eq149_e1891_q_d_n20, eq149_e1891_q_d_n21, eq149_e1891_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq149_reactive_node_derivatives: [f64; 23] = [eq149_e1893_q_d_n0, eq149_e1893_q_d_n1, eq149_e1893_q_d_n2, eq149_e1893_q_d_n3, eq149_e1893_q_d_n4, eq149_e1893_q_d_n5, eq149_e1893_q_d_n6, eq149_e1893_q_d_n7, eq149_e1893_q_d_n8, eq149_e1893_q_d_n9, eq149_e1893_q_d_n10, eq149_e1893_q_d_n11, eq149_e1893_q_d_n12, eq149_e1893_q_d_n13, eq149_e1893_q_d_n14, eq149_e1893_q_d_n15, eq149_e1893_q_d_n16, eq149_e1893_q_d_n17, eq149_e1893_q_d_n18, eq149_e1893_q_d_n19, eq149_e1893_q_d_n20, eq149_e1893_q_d_n21, eq149_e1893_q_d_n22];
        let eq149_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[15]),
            &nodes,
            &eq149_reactive_node_derivatives,
            &branches,
            &eq149_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_150_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq150_e1903, eq150_e1903_d_n0, eq150_e1903_d_n1, eq150_e1903_d_n2, eq150_e1903_d_n3, eq150_e1903_d_n4, eq150_e1903_d_n5, eq150_e1903_d_n6, eq150_e1903_d_n7, eq150_e1903_d_n8, eq150_e1903_d_n9, eq150_e1903_d_n10, eq150_e1903_d_n11, eq150_e1903_d_n12, eq150_e1903_d_n13, eq150_e1903_d_n14, eq150_e1903_d_n15, eq150_e1903_d_n16, eq150_e1903_d_n17, eq150_e1903_d_n18, eq150_e1903_d_n19, eq150_e1903_d_n20, eq150_e1903_d_n21, eq150_e1903_d_n22, eq150_e1903_q, eq150_e1903_q_d_n0, eq150_e1903_q_d_n1, eq150_e1903_q_d_n2, eq150_e1903_q_d_n3, eq150_e1903_q_d_n4, eq150_e1903_q_d_n5, eq150_e1903_q_d_n6, eq150_e1903_q_d_n7, eq150_e1903_q_d_n8, eq150_e1903_q_d_n9, eq150_e1903_q_d_n10, eq150_e1903_q_d_n11, eq150_e1903_q_d_n12, eq150_e1903_q_d_n13, eq150_e1903_q_d_n14, eq150_e1903_q_d_n15, eq150_e1903_q_d_n16, eq150_e1903_q_d_n17, eq150_e1903_q_d_n18, eq150_e1903_q_d_n19, eq150_e1903_q_d_n20, eq150_e1903_q_d_n21, eq150_e1903_q_d_n22,) = {
    if ((!(s.v[580] != 0.0)) && (s.v[583] != 0.0)) {
        let eq150_e1900_q: f64 = s.v[253];
        let eq150_e1901: f64 = (p.p7 * s.v[253]);
        let eq150_e1901_d_n0: f64 = (p.p7 * s.dn[253][0]);
        let eq150_e1901_d_n1: f64 = (p.p7 * s.dn[253][1]);
        let eq150_e1901_d_n2: f64 = (p.p7 * s.dn[253][2]);
        let eq150_e1901_d_n3: f64 = (p.p7 * s.dn[253][3]);
        let eq150_e1901_d_n4: f64 = (p.p7 * s.dn[253][4]);
        let eq150_e1901_d_n5: f64 = (p.p7 * s.dn[253][5]);
        let eq150_e1901_d_n6: f64 = (p.p7 * s.dn[253][6]);
        let eq150_e1901_d_n7: f64 = (p.p7 * s.dn[253][7]);
        let eq150_e1901_d_n8: f64 = (p.p7 * s.dn[253][8]);
        let eq150_e1901_d_n9: f64 = (p.p7 * s.dn[253][9]);
        let eq150_e1901_d_n10: f64 = (p.p7 * s.dn[253][10]);
        let eq150_e1901_d_n11: f64 = (p.p7 * s.dn[253][11]);
        let eq150_e1901_d_n12: f64 = (p.p7 * s.dn[253][12]);
        let eq150_e1901_d_n13: f64 = (p.p7 * s.dn[253][13]);
        let eq150_e1901_d_n14: f64 = (p.p7 * s.dn[253][14]);
        let eq150_e1901_d_n15: f64 = (p.p7 * s.dn[253][15]);
        let eq150_e1901_d_n16: f64 = (p.p7 * s.dn[253][16]);
        let eq150_e1901_d_n17: f64 = (p.p7 * s.dn[253][17]);
        let eq150_e1901_d_n18: f64 = (p.p7 * s.dn[253][18]);
        let eq150_e1901_d_n19: f64 = (p.p7 * s.dn[253][19]);
        let eq150_e1901_d_n20: f64 = (p.p7 * s.dn[253][20]);
        let eq150_e1901_d_n21: f64 = (p.p7 * s.dn[253][21]);
        let eq150_e1901_d_n22: f64 = (p.p7 * s.dn[253][22]);
        let eq150_e1901_q: f64 = (p.p7 * eq150_e1900_q);
        let eq150_e1901_q_d_n0: f64 = (p.p7 * s.dn[253][0]);
        let eq150_e1901_q_d_n1: f64 = (p.p7 * s.dn[253][1]);
        let eq150_e1901_q_d_n2: f64 = (p.p7 * s.dn[253][2]);
        let eq150_e1901_q_d_n3: f64 = (p.p7 * s.dn[253][3]);
        let eq150_e1901_q_d_n4: f64 = (p.p7 * s.dn[253][4]);
        let eq150_e1901_q_d_n5: f64 = (p.p7 * s.dn[253][5]);
        let eq150_e1901_q_d_n6: f64 = (p.p7 * s.dn[253][6]);
        let eq150_e1901_q_d_n7: f64 = (p.p7 * s.dn[253][7]);
        let eq150_e1901_q_d_n8: f64 = (p.p7 * s.dn[253][8]);
        let eq150_e1901_q_d_n9: f64 = (p.p7 * s.dn[253][9]);
        let eq150_e1901_q_d_n10: f64 = (p.p7 * s.dn[253][10]);
        let eq150_e1901_q_d_n11: f64 = (p.p7 * s.dn[253][11]);
        let eq150_e1901_q_d_n12: f64 = (p.p7 * s.dn[253][12]);
        let eq150_e1901_q_d_n13: f64 = (p.p7 * s.dn[253][13]);
        let eq150_e1901_q_d_n14: f64 = (p.p7 * s.dn[253][14]);
        let eq150_e1901_q_d_n15: f64 = (p.p7 * s.dn[253][15]);
        let eq150_e1901_q_d_n16: f64 = (p.p7 * s.dn[253][16]);
        let eq150_e1901_q_d_n17: f64 = (p.p7 * s.dn[253][17]);
        let eq150_e1901_q_d_n18: f64 = (p.p7 * s.dn[253][18]);
        let eq150_e1901_q_d_n19: f64 = (p.p7 * s.dn[253][19]);
        let eq150_e1901_q_d_n20: f64 = (p.p7 * s.dn[253][20]);
        let eq150_e1901_q_d_n21: f64 = (p.p7 * s.dn[253][21]);
        let eq150_e1901_q_d_n22: f64 = (p.p7 * s.dn[253][22]);
        (eq150_e1901, eq150_e1901_d_n0, eq150_e1901_d_n1, eq150_e1901_d_n2, eq150_e1901_d_n3, eq150_e1901_d_n4, eq150_e1901_d_n5, eq150_e1901_d_n6, eq150_e1901_d_n7, eq150_e1901_d_n8, eq150_e1901_d_n9, eq150_e1901_d_n10, eq150_e1901_d_n11, eq150_e1901_d_n12, eq150_e1901_d_n13, eq150_e1901_d_n14, eq150_e1901_d_n15, eq150_e1901_d_n16, eq150_e1901_d_n17, eq150_e1901_d_n18, eq150_e1901_d_n19, eq150_e1901_d_n20, eq150_e1901_d_n21, eq150_e1901_d_n22, eq150_e1901_q, eq150_e1901_q_d_n0, eq150_e1901_q_d_n1, eq150_e1901_q_d_n2, eq150_e1901_q_d_n3, eq150_e1901_q_d_n4, eq150_e1901_q_d_n5, eq150_e1901_q_d_n6, eq150_e1901_q_d_n7, eq150_e1901_q_d_n8, eq150_e1901_q_d_n9, eq150_e1901_q_d_n10, eq150_e1901_q_d_n11, eq150_e1901_q_d_n12, eq150_e1901_q_d_n13, eq150_e1901_q_d_n14, eq150_e1901_q_d_n15, eq150_e1901_q_d_n16, eq150_e1901_q_d_n17, eq150_e1901_q_d_n18, eq150_e1901_q_d_n19, eq150_e1901_q_d_n20, eq150_e1901_q_d_n21, eq150_e1901_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq150_reactive_node_derivatives: [f64; 23] = [eq150_e1903_q_d_n0, eq150_e1903_q_d_n1, eq150_e1903_q_d_n2, eq150_e1903_q_d_n3, eq150_e1903_q_d_n4, eq150_e1903_q_d_n5, eq150_e1903_q_d_n6, eq150_e1903_q_d_n7, eq150_e1903_q_d_n8, eq150_e1903_q_d_n9, eq150_e1903_q_d_n10, eq150_e1903_q_d_n11, eq150_e1903_q_d_n12, eq150_e1903_q_d_n13, eq150_e1903_q_d_n14, eq150_e1903_q_d_n15, eq150_e1903_q_d_n16, eq150_e1903_q_d_n17, eq150_e1903_q_d_n18, eq150_e1903_q_d_n19, eq150_e1903_q_d_n20, eq150_e1903_q_d_n21, eq150_e1903_q_d_n22];
        let eq150_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[7]),
            &nodes,
            &eq150_reactive_node_derivatives,
            &branches,
            &eq150_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_151_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq151_e1915, eq151_e1915_d_n0, eq151_e1915_d_n1, eq151_e1915_d_n2, eq151_e1915_d_n3, eq151_e1915_d_n4, eq151_e1915_d_n5, eq151_e1915_d_n6, eq151_e1915_d_n7, eq151_e1915_d_n8, eq151_e1915_d_n9, eq151_e1915_d_n10, eq151_e1915_d_n11, eq151_e1915_d_n12, eq151_e1915_d_n13, eq151_e1915_d_n14, eq151_e1915_d_n15, eq151_e1915_d_n16, eq151_e1915_d_n17, eq151_e1915_d_n18, eq151_e1915_d_n19, eq151_e1915_d_n20, eq151_e1915_d_n21, eq151_e1915_d_n22, eq151_e1915_q, eq151_e1915_q_d_n0, eq151_e1915_q_d_n1, eq151_e1915_q_d_n2, eq151_e1915_q_d_n3, eq151_e1915_q_d_n4, eq151_e1915_q_d_n5, eq151_e1915_q_d_n6, eq151_e1915_q_d_n7, eq151_e1915_q_d_n8, eq151_e1915_q_d_n9, eq151_e1915_q_d_n10, eq151_e1915_q_d_n11, eq151_e1915_q_d_n12, eq151_e1915_q_d_n13, eq151_e1915_q_d_n14, eq151_e1915_q_d_n15, eq151_e1915_q_d_n16, eq151_e1915_q_d_n17, eq151_e1915_q_d_n18, eq151_e1915_q_d_n19, eq151_e1915_q_d_n20, eq151_e1915_q_d_n21, eq151_e1915_q_d_n22,) = {
    if (((!(s.v[580] != 0.0)) && (s.v[583] != 0.0)) && (s.v[584] != 0.0)) {
        let eq151_e1912_q: f64 = s.v[252];
        let eq151_e1913: f64 = (p.p7 * s.v[252]);
        let eq151_e1913_d_n0: f64 = (p.p7 * s.dn[252][0]);
        let eq151_e1913_d_n1: f64 = (p.p7 * s.dn[252][1]);
        let eq151_e1913_d_n2: f64 = (p.p7 * s.dn[252][2]);
        let eq151_e1913_d_n3: f64 = (p.p7 * s.dn[252][3]);
        let eq151_e1913_d_n4: f64 = (p.p7 * s.dn[252][4]);
        let eq151_e1913_d_n5: f64 = (p.p7 * s.dn[252][5]);
        let eq151_e1913_d_n6: f64 = (p.p7 * s.dn[252][6]);
        let eq151_e1913_d_n7: f64 = (p.p7 * s.dn[252][7]);
        let eq151_e1913_d_n8: f64 = (p.p7 * s.dn[252][8]);
        let eq151_e1913_d_n9: f64 = (p.p7 * s.dn[252][9]);
        let eq151_e1913_d_n10: f64 = (p.p7 * s.dn[252][10]);
        let eq151_e1913_d_n11: f64 = (p.p7 * s.dn[252][11]);
        let eq151_e1913_d_n12: f64 = (p.p7 * s.dn[252][12]);
        let eq151_e1913_d_n13: f64 = (p.p7 * s.dn[252][13]);
        let eq151_e1913_d_n14: f64 = (p.p7 * s.dn[252][14]);
        let eq151_e1913_d_n15: f64 = (p.p7 * s.dn[252][15]);
        let eq151_e1913_d_n16: f64 = (p.p7 * s.dn[252][16]);
        let eq151_e1913_d_n17: f64 = (p.p7 * s.dn[252][17]);
        let eq151_e1913_d_n18: f64 = (p.p7 * s.dn[252][18]);
        let eq151_e1913_d_n19: f64 = (p.p7 * s.dn[252][19]);
        let eq151_e1913_d_n20: f64 = (p.p7 * s.dn[252][20]);
        let eq151_e1913_d_n21: f64 = (p.p7 * s.dn[252][21]);
        let eq151_e1913_d_n22: f64 = (p.p7 * s.dn[252][22]);
        let eq151_e1913_q: f64 = (p.p7 * eq151_e1912_q);
        let eq151_e1913_q_d_n0: f64 = (p.p7 * s.dn[252][0]);
        let eq151_e1913_q_d_n1: f64 = (p.p7 * s.dn[252][1]);
        let eq151_e1913_q_d_n2: f64 = (p.p7 * s.dn[252][2]);
        let eq151_e1913_q_d_n3: f64 = (p.p7 * s.dn[252][3]);
        let eq151_e1913_q_d_n4: f64 = (p.p7 * s.dn[252][4]);
        let eq151_e1913_q_d_n5: f64 = (p.p7 * s.dn[252][5]);
        let eq151_e1913_q_d_n6: f64 = (p.p7 * s.dn[252][6]);
        let eq151_e1913_q_d_n7: f64 = (p.p7 * s.dn[252][7]);
        let eq151_e1913_q_d_n8: f64 = (p.p7 * s.dn[252][8]);
        let eq151_e1913_q_d_n9: f64 = (p.p7 * s.dn[252][9]);
        let eq151_e1913_q_d_n10: f64 = (p.p7 * s.dn[252][10]);
        let eq151_e1913_q_d_n11: f64 = (p.p7 * s.dn[252][11]);
        let eq151_e1913_q_d_n12: f64 = (p.p7 * s.dn[252][12]);
        let eq151_e1913_q_d_n13: f64 = (p.p7 * s.dn[252][13]);
        let eq151_e1913_q_d_n14: f64 = (p.p7 * s.dn[252][14]);
        let eq151_e1913_q_d_n15: f64 = (p.p7 * s.dn[252][15]);
        let eq151_e1913_q_d_n16: f64 = (p.p7 * s.dn[252][16]);
        let eq151_e1913_q_d_n17: f64 = (p.p7 * s.dn[252][17]);
        let eq151_e1913_q_d_n18: f64 = (p.p7 * s.dn[252][18]);
        let eq151_e1913_q_d_n19: f64 = (p.p7 * s.dn[252][19]);
        let eq151_e1913_q_d_n20: f64 = (p.p7 * s.dn[252][20]);
        let eq151_e1913_q_d_n21: f64 = (p.p7 * s.dn[252][21]);
        let eq151_e1913_q_d_n22: f64 = (p.p7 * s.dn[252][22]);
        (eq151_e1913, eq151_e1913_d_n0, eq151_e1913_d_n1, eq151_e1913_d_n2, eq151_e1913_d_n3, eq151_e1913_d_n4, eq151_e1913_d_n5, eq151_e1913_d_n6, eq151_e1913_d_n7, eq151_e1913_d_n8, eq151_e1913_d_n9, eq151_e1913_d_n10, eq151_e1913_d_n11, eq151_e1913_d_n12, eq151_e1913_d_n13, eq151_e1913_d_n14, eq151_e1913_d_n15, eq151_e1913_d_n16, eq151_e1913_d_n17, eq151_e1913_d_n18, eq151_e1913_d_n19, eq151_e1913_d_n20, eq151_e1913_d_n21, eq151_e1913_d_n22, eq151_e1913_q, eq151_e1913_q_d_n0, eq151_e1913_q_d_n1, eq151_e1913_q_d_n2, eq151_e1913_q_d_n3, eq151_e1913_q_d_n4, eq151_e1913_q_d_n5, eq151_e1913_q_d_n6, eq151_e1913_q_d_n7, eq151_e1913_q_d_n8, eq151_e1913_q_d_n9, eq151_e1913_q_d_n10, eq151_e1913_q_d_n11, eq151_e1913_q_d_n12, eq151_e1913_q_d_n13, eq151_e1913_q_d_n14, eq151_e1913_q_d_n15, eq151_e1913_q_d_n16, eq151_e1913_q_d_n17, eq151_e1913_q_d_n18, eq151_e1913_q_d_n19, eq151_e1913_q_d_n20, eq151_e1913_q_d_n21, eq151_e1913_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq151_reactive_node_derivatives: [f64; 23] = [eq151_e1915_q_d_n0, eq151_e1915_q_d_n1, eq151_e1915_q_d_n2, eq151_e1915_q_d_n3, eq151_e1915_q_d_n4, eq151_e1915_q_d_n5, eq151_e1915_q_d_n6, eq151_e1915_q_d_n7, eq151_e1915_q_d_n8, eq151_e1915_q_d_n9, eq151_e1915_q_d_n10, eq151_e1915_q_d_n11, eq151_e1915_q_d_n12, eq151_e1915_q_d_n13, eq151_e1915_q_d_n14, eq151_e1915_q_d_n15, eq151_e1915_q_d_n16, eq151_e1915_q_d_n17, eq151_e1915_q_d_n18, eq151_e1915_q_d_n19, eq151_e1915_q_d_n20, eq151_e1915_q_d_n21, eq151_e1915_q_d_n22];
        let eq151_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            &nodes,
            &eq151_reactive_node_derivatives,
            &branches,
            &eq151_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_152_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq152_e1929, eq152_e1929_d_n0, eq152_e1929_d_n1, eq152_e1929_d_n2, eq152_e1929_d_n3, eq152_e1929_d_n4, eq152_e1929_d_n5, eq152_e1929_d_n6, eq152_e1929_d_n7, eq152_e1929_d_n8, eq152_e1929_d_n9, eq152_e1929_d_n10, eq152_e1929_d_n11, eq152_e1929_d_n12, eq152_e1929_d_n13, eq152_e1929_d_n14, eq152_e1929_d_n15, eq152_e1929_d_n16, eq152_e1929_d_n17, eq152_e1929_d_n18, eq152_e1929_d_n19, eq152_e1929_d_n20, eq152_e1929_d_n21, eq152_e1929_d_n22, eq152_e1929_q, eq152_e1929_q_d_n0, eq152_e1929_q_d_n1, eq152_e1929_q_d_n2, eq152_e1929_q_d_n3, eq152_e1929_q_d_n4, eq152_e1929_q_d_n5, eq152_e1929_q_d_n6, eq152_e1929_q_d_n7, eq152_e1929_q_d_n8, eq152_e1929_q_d_n9, eq152_e1929_q_d_n10, eq152_e1929_q_d_n11, eq152_e1929_q_d_n12, eq152_e1929_q_d_n13, eq152_e1929_q_d_n14, eq152_e1929_q_d_n15, eq152_e1929_q_d_n16, eq152_e1929_q_d_n17, eq152_e1929_q_d_n18, eq152_e1929_q_d_n19, eq152_e1929_q_d_n20, eq152_e1929_q_d_n21, eq152_e1929_q_d_n22,) = {
    if (((!(s.v[580] != 0.0)) && (s.v[583] != 0.0)) && (s.v[584] != 0.0)) {
        let eq152_e1924: f64 = (p.p7 * p.p247);
        let eq152_e1926_q: f64 = s.v[252];
        let eq152_e1927: f64 = (eq152_e1924 * s.v[252]);
        let eq152_e1927_d_n0: f64 = (eq152_e1924 * s.dn[252][0]);
        let eq152_e1927_d_n1: f64 = (eq152_e1924 * s.dn[252][1]);
        let eq152_e1927_d_n2: f64 = (eq152_e1924 * s.dn[252][2]);
        let eq152_e1927_d_n3: f64 = (eq152_e1924 * s.dn[252][3]);
        let eq152_e1927_d_n4: f64 = (eq152_e1924 * s.dn[252][4]);
        let eq152_e1927_d_n5: f64 = (eq152_e1924 * s.dn[252][5]);
        let eq152_e1927_d_n6: f64 = (eq152_e1924 * s.dn[252][6]);
        let eq152_e1927_d_n7: f64 = (eq152_e1924 * s.dn[252][7]);
        let eq152_e1927_d_n8: f64 = (eq152_e1924 * s.dn[252][8]);
        let eq152_e1927_d_n9: f64 = (eq152_e1924 * s.dn[252][9]);
        let eq152_e1927_d_n10: f64 = (eq152_e1924 * s.dn[252][10]);
        let eq152_e1927_d_n11: f64 = (eq152_e1924 * s.dn[252][11]);
        let eq152_e1927_d_n12: f64 = (eq152_e1924 * s.dn[252][12]);
        let eq152_e1927_d_n13: f64 = (eq152_e1924 * s.dn[252][13]);
        let eq152_e1927_d_n14: f64 = (eq152_e1924 * s.dn[252][14]);
        let eq152_e1927_d_n15: f64 = (eq152_e1924 * s.dn[252][15]);
        let eq152_e1927_d_n16: f64 = (eq152_e1924 * s.dn[252][16]);
        let eq152_e1927_d_n17: f64 = (eq152_e1924 * s.dn[252][17]);
        let eq152_e1927_d_n18: f64 = (eq152_e1924 * s.dn[252][18]);
        let eq152_e1927_d_n19: f64 = (eq152_e1924 * s.dn[252][19]);
        let eq152_e1927_d_n20: f64 = (eq152_e1924 * s.dn[252][20]);
        let eq152_e1927_d_n21: f64 = (eq152_e1924 * s.dn[252][21]);
        let eq152_e1927_d_n22: f64 = (eq152_e1924 * s.dn[252][22]);
        let eq152_e1927_q: f64 = (eq152_e1924 * eq152_e1926_q);
        let eq152_e1927_q_d_n0: f64 = (eq152_e1924 * s.dn[252][0]);
        let eq152_e1927_q_d_n1: f64 = (eq152_e1924 * s.dn[252][1]);
        let eq152_e1927_q_d_n2: f64 = (eq152_e1924 * s.dn[252][2]);
        let eq152_e1927_q_d_n3: f64 = (eq152_e1924 * s.dn[252][3]);
        let eq152_e1927_q_d_n4: f64 = (eq152_e1924 * s.dn[252][4]);
        let eq152_e1927_q_d_n5: f64 = (eq152_e1924 * s.dn[252][5]);
        let eq152_e1927_q_d_n6: f64 = (eq152_e1924 * s.dn[252][6]);
        let eq152_e1927_q_d_n7: f64 = (eq152_e1924 * s.dn[252][7]);
        let eq152_e1927_q_d_n8: f64 = (eq152_e1924 * s.dn[252][8]);
        let eq152_e1927_q_d_n9: f64 = (eq152_e1924 * s.dn[252][9]);
        let eq152_e1927_q_d_n10: f64 = (eq152_e1924 * s.dn[252][10]);
        let eq152_e1927_q_d_n11: f64 = (eq152_e1924 * s.dn[252][11]);
        let eq152_e1927_q_d_n12: f64 = (eq152_e1924 * s.dn[252][12]);
        let eq152_e1927_q_d_n13: f64 = (eq152_e1924 * s.dn[252][13]);
        let eq152_e1927_q_d_n14: f64 = (eq152_e1924 * s.dn[252][14]);
        let eq152_e1927_q_d_n15: f64 = (eq152_e1924 * s.dn[252][15]);
        let eq152_e1927_q_d_n16: f64 = (eq152_e1924 * s.dn[252][16]);
        let eq152_e1927_q_d_n17: f64 = (eq152_e1924 * s.dn[252][17]);
        let eq152_e1927_q_d_n18: f64 = (eq152_e1924 * s.dn[252][18]);
        let eq152_e1927_q_d_n19: f64 = (eq152_e1924 * s.dn[252][19]);
        let eq152_e1927_q_d_n20: f64 = (eq152_e1924 * s.dn[252][20]);
        let eq152_e1927_q_d_n21: f64 = (eq152_e1924 * s.dn[252][21]);
        let eq152_e1927_q_d_n22: f64 = (eq152_e1924 * s.dn[252][22]);
        (eq152_e1927, eq152_e1927_d_n0, eq152_e1927_d_n1, eq152_e1927_d_n2, eq152_e1927_d_n3, eq152_e1927_d_n4, eq152_e1927_d_n5, eq152_e1927_d_n6, eq152_e1927_d_n7, eq152_e1927_d_n8, eq152_e1927_d_n9, eq152_e1927_d_n10, eq152_e1927_d_n11, eq152_e1927_d_n12, eq152_e1927_d_n13, eq152_e1927_d_n14, eq152_e1927_d_n15, eq152_e1927_d_n16, eq152_e1927_d_n17, eq152_e1927_d_n18, eq152_e1927_d_n19, eq152_e1927_d_n20, eq152_e1927_d_n21, eq152_e1927_d_n22, eq152_e1927_q, eq152_e1927_q_d_n0, eq152_e1927_q_d_n1, eq152_e1927_q_d_n2, eq152_e1927_q_d_n3, eq152_e1927_q_d_n4, eq152_e1927_q_d_n5, eq152_e1927_q_d_n6, eq152_e1927_q_d_n7, eq152_e1927_q_d_n8, eq152_e1927_q_d_n9, eq152_e1927_q_d_n10, eq152_e1927_q_d_n11, eq152_e1927_q_d_n12, eq152_e1927_q_d_n13, eq152_e1927_q_d_n14, eq152_e1927_q_d_n15, eq152_e1927_q_d_n16, eq152_e1927_q_d_n17, eq152_e1927_q_d_n18, eq152_e1927_q_d_n19, eq152_e1927_q_d_n20, eq152_e1927_q_d_n21, eq152_e1927_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq152_reactive_node_derivatives: [f64; 23] = [eq152_e1929_q_d_n0, eq152_e1929_q_d_n1, eq152_e1929_q_d_n2, eq152_e1929_q_d_n3, eq152_e1929_q_d_n4, eq152_e1929_q_d_n5, eq152_e1929_q_d_n6, eq152_e1929_q_d_n7, eq152_e1929_q_d_n8, eq152_e1929_q_d_n9, eq152_e1929_q_d_n10, eq152_e1929_q_d_n11, eq152_e1929_q_d_n12, eq152_e1929_q_d_n13, eq152_e1929_q_d_n14, eq152_e1929_q_d_n15, eq152_e1929_q_d_n16, eq152_e1929_q_d_n17, eq152_e1929_q_d_n18, eq152_e1929_q_d_n19, eq152_e1929_q_d_n20, eq152_e1929_q_d_n21, eq152_e1929_q_d_n22];
        let eq152_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            &nodes,
            &eq152_reactive_node_derivatives,
            &branches,
            &eq152_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_153_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq153_e1942, eq153_e1942_d_n0, eq153_e1942_d_n1, eq153_e1942_d_n2, eq153_e1942_d_n3, eq153_e1942_d_n4, eq153_e1942_d_n5, eq153_e1942_d_n6, eq153_e1942_d_n7, eq153_e1942_d_n8, eq153_e1942_d_n9, eq153_e1942_d_n10, eq153_e1942_d_n11, eq153_e1942_d_n12, eq153_e1942_d_n13, eq153_e1942_d_n14, eq153_e1942_d_n15, eq153_e1942_d_n16, eq153_e1942_d_n17, eq153_e1942_d_n18, eq153_e1942_d_n19, eq153_e1942_d_n20, eq153_e1942_d_n21, eq153_e1942_d_n22, eq153_e1942_q, eq153_e1942_q_d_n0, eq153_e1942_q_d_n1, eq153_e1942_q_d_n2, eq153_e1942_q_d_n3, eq153_e1942_q_d_n4, eq153_e1942_q_d_n5, eq153_e1942_q_d_n6, eq153_e1942_q_d_n7, eq153_e1942_q_d_n8, eq153_e1942_q_d_n9, eq153_e1942_q_d_n10, eq153_e1942_q_d_n11, eq153_e1942_q_d_n12, eq153_e1942_q_d_n13, eq153_e1942_q_d_n14, eq153_e1942_q_d_n15, eq153_e1942_q_d_n16, eq153_e1942_q_d_n17, eq153_e1942_q_d_n18, eq153_e1942_q_d_n19, eq153_e1942_q_d_n20, eq153_e1942_q_d_n21, eq153_e1942_q_d_n22,) = {
    if (((!(s.v[580] != 0.0)) && (s.v[583] != 0.0)) && (!(s.v[584] != 0.0))) {
        let eq153_e1939_q: f64 = s.v[252];
        let eq153_e1940: f64 = (p.p7 * s.v[252]);
        let eq153_e1940_d_n0: f64 = (p.p7 * s.dn[252][0]);
        let eq153_e1940_d_n1: f64 = (p.p7 * s.dn[252][1]);
        let eq153_e1940_d_n2: f64 = (p.p7 * s.dn[252][2]);
        let eq153_e1940_d_n3: f64 = (p.p7 * s.dn[252][3]);
        let eq153_e1940_d_n4: f64 = (p.p7 * s.dn[252][4]);
        let eq153_e1940_d_n5: f64 = (p.p7 * s.dn[252][5]);
        let eq153_e1940_d_n6: f64 = (p.p7 * s.dn[252][6]);
        let eq153_e1940_d_n7: f64 = (p.p7 * s.dn[252][7]);
        let eq153_e1940_d_n8: f64 = (p.p7 * s.dn[252][8]);
        let eq153_e1940_d_n9: f64 = (p.p7 * s.dn[252][9]);
        let eq153_e1940_d_n10: f64 = (p.p7 * s.dn[252][10]);
        let eq153_e1940_d_n11: f64 = (p.p7 * s.dn[252][11]);
        let eq153_e1940_d_n12: f64 = (p.p7 * s.dn[252][12]);
        let eq153_e1940_d_n13: f64 = (p.p7 * s.dn[252][13]);
        let eq153_e1940_d_n14: f64 = (p.p7 * s.dn[252][14]);
        let eq153_e1940_d_n15: f64 = (p.p7 * s.dn[252][15]);
        let eq153_e1940_d_n16: f64 = (p.p7 * s.dn[252][16]);
        let eq153_e1940_d_n17: f64 = (p.p7 * s.dn[252][17]);
        let eq153_e1940_d_n18: f64 = (p.p7 * s.dn[252][18]);
        let eq153_e1940_d_n19: f64 = (p.p7 * s.dn[252][19]);
        let eq153_e1940_d_n20: f64 = (p.p7 * s.dn[252][20]);
        let eq153_e1940_d_n21: f64 = (p.p7 * s.dn[252][21]);
        let eq153_e1940_d_n22: f64 = (p.p7 * s.dn[252][22]);
        let eq153_e1940_q: f64 = (p.p7 * eq153_e1939_q);
        let eq153_e1940_q_d_n0: f64 = (p.p7 * s.dn[252][0]);
        let eq153_e1940_q_d_n1: f64 = (p.p7 * s.dn[252][1]);
        let eq153_e1940_q_d_n2: f64 = (p.p7 * s.dn[252][2]);
        let eq153_e1940_q_d_n3: f64 = (p.p7 * s.dn[252][3]);
        let eq153_e1940_q_d_n4: f64 = (p.p7 * s.dn[252][4]);
        let eq153_e1940_q_d_n5: f64 = (p.p7 * s.dn[252][5]);
        let eq153_e1940_q_d_n6: f64 = (p.p7 * s.dn[252][6]);
        let eq153_e1940_q_d_n7: f64 = (p.p7 * s.dn[252][7]);
        let eq153_e1940_q_d_n8: f64 = (p.p7 * s.dn[252][8]);
        let eq153_e1940_q_d_n9: f64 = (p.p7 * s.dn[252][9]);
        let eq153_e1940_q_d_n10: f64 = (p.p7 * s.dn[252][10]);
        let eq153_e1940_q_d_n11: f64 = (p.p7 * s.dn[252][11]);
        let eq153_e1940_q_d_n12: f64 = (p.p7 * s.dn[252][12]);
        let eq153_e1940_q_d_n13: f64 = (p.p7 * s.dn[252][13]);
        let eq153_e1940_q_d_n14: f64 = (p.p7 * s.dn[252][14]);
        let eq153_e1940_q_d_n15: f64 = (p.p7 * s.dn[252][15]);
        let eq153_e1940_q_d_n16: f64 = (p.p7 * s.dn[252][16]);
        let eq153_e1940_q_d_n17: f64 = (p.p7 * s.dn[252][17]);
        let eq153_e1940_q_d_n18: f64 = (p.p7 * s.dn[252][18]);
        let eq153_e1940_q_d_n19: f64 = (p.p7 * s.dn[252][19]);
        let eq153_e1940_q_d_n20: f64 = (p.p7 * s.dn[252][20]);
        let eq153_e1940_q_d_n21: f64 = (p.p7 * s.dn[252][21]);
        let eq153_e1940_q_d_n22: f64 = (p.p7 * s.dn[252][22]);
        (eq153_e1940, eq153_e1940_d_n0, eq153_e1940_d_n1, eq153_e1940_d_n2, eq153_e1940_d_n3, eq153_e1940_d_n4, eq153_e1940_d_n5, eq153_e1940_d_n6, eq153_e1940_d_n7, eq153_e1940_d_n8, eq153_e1940_d_n9, eq153_e1940_d_n10, eq153_e1940_d_n11, eq153_e1940_d_n12, eq153_e1940_d_n13, eq153_e1940_d_n14, eq153_e1940_d_n15, eq153_e1940_d_n16, eq153_e1940_d_n17, eq153_e1940_d_n18, eq153_e1940_d_n19, eq153_e1940_d_n20, eq153_e1940_d_n21, eq153_e1940_d_n22, eq153_e1940_q, eq153_e1940_q_d_n0, eq153_e1940_q_d_n1, eq153_e1940_q_d_n2, eq153_e1940_q_d_n3, eq153_e1940_q_d_n4, eq153_e1940_q_d_n5, eq153_e1940_q_d_n6, eq153_e1940_q_d_n7, eq153_e1940_q_d_n8, eq153_e1940_q_d_n9, eq153_e1940_q_d_n10, eq153_e1940_q_d_n11, eq153_e1940_q_d_n12, eq153_e1940_q_d_n13, eq153_e1940_q_d_n14, eq153_e1940_q_d_n15, eq153_e1940_q_d_n16, eq153_e1940_q_d_n17, eq153_e1940_q_d_n18, eq153_e1940_q_d_n19, eq153_e1940_q_d_n20, eq153_e1940_q_d_n21, eq153_e1940_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq153_reactive_node_derivatives: [f64; 23] = [eq153_e1942_q_d_n0, eq153_e1942_q_d_n1, eq153_e1942_q_d_n2, eq153_e1942_q_d_n3, eq153_e1942_q_d_n4, eq153_e1942_q_d_n5, eq153_e1942_q_d_n6, eq153_e1942_q_d_n7, eq153_e1942_q_d_n8, eq153_e1942_q_d_n9, eq153_e1942_q_d_n10, eq153_e1942_q_d_n11, eq153_e1942_q_d_n12, eq153_e1942_q_d_n13, eq153_e1942_q_d_n14, eq153_e1942_q_d_n15, eq153_e1942_q_d_n16, eq153_e1942_q_d_n17, eq153_e1942_q_d_n18, eq153_e1942_q_d_n19, eq153_e1942_q_d_n20, eq153_e1942_q_d_n21, eq153_e1942_q_d_n22];
        let eq153_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            &nodes,
            &eq153_reactive_node_derivatives,
            &branches,
            &eq153_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_154_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq154_e1957, eq154_e1957_d_n0, eq154_e1957_d_n1, eq154_e1957_d_n2, eq154_e1957_d_n3, eq154_e1957_d_n4, eq154_e1957_d_n5, eq154_e1957_d_n6, eq154_e1957_d_n7, eq154_e1957_d_n8, eq154_e1957_d_n9, eq154_e1957_d_n10, eq154_e1957_d_n11, eq154_e1957_d_n12, eq154_e1957_d_n13, eq154_e1957_d_n14, eq154_e1957_d_n15, eq154_e1957_d_n16, eq154_e1957_d_n17, eq154_e1957_d_n18, eq154_e1957_d_n19, eq154_e1957_d_n20, eq154_e1957_d_n21, eq154_e1957_d_n22, eq154_e1957_q, eq154_e1957_q_d_n0, eq154_e1957_q_d_n1, eq154_e1957_q_d_n2, eq154_e1957_q_d_n3, eq154_e1957_q_d_n4, eq154_e1957_q_d_n5, eq154_e1957_q_d_n6, eq154_e1957_q_d_n7, eq154_e1957_q_d_n8, eq154_e1957_q_d_n9, eq154_e1957_q_d_n10, eq154_e1957_q_d_n11, eq154_e1957_q_d_n12, eq154_e1957_q_d_n13, eq154_e1957_q_d_n14, eq154_e1957_q_d_n15, eq154_e1957_q_d_n16, eq154_e1957_q_d_n17, eq154_e1957_q_d_n18, eq154_e1957_q_d_n19, eq154_e1957_q_d_n20, eq154_e1957_q_d_n21, eq154_e1957_q_d_n22,) = {
    if (((!(s.v[580] != 0.0)) && (s.v[583] != 0.0)) && (!(s.v[584] != 0.0))) {
        let eq154_e1952: f64 = (p.p7 * p.p247);
        let eq154_e1954_q: f64 = s.v[252];
        let eq154_e1955: f64 = (eq154_e1952 * s.v[252]);
        let eq154_e1955_d_n0: f64 = (eq154_e1952 * s.dn[252][0]);
        let eq154_e1955_d_n1: f64 = (eq154_e1952 * s.dn[252][1]);
        let eq154_e1955_d_n2: f64 = (eq154_e1952 * s.dn[252][2]);
        let eq154_e1955_d_n3: f64 = (eq154_e1952 * s.dn[252][3]);
        let eq154_e1955_d_n4: f64 = (eq154_e1952 * s.dn[252][4]);
        let eq154_e1955_d_n5: f64 = (eq154_e1952 * s.dn[252][5]);
        let eq154_e1955_d_n6: f64 = (eq154_e1952 * s.dn[252][6]);
        let eq154_e1955_d_n7: f64 = (eq154_e1952 * s.dn[252][7]);
        let eq154_e1955_d_n8: f64 = (eq154_e1952 * s.dn[252][8]);
        let eq154_e1955_d_n9: f64 = (eq154_e1952 * s.dn[252][9]);
        let eq154_e1955_d_n10: f64 = (eq154_e1952 * s.dn[252][10]);
        let eq154_e1955_d_n11: f64 = (eq154_e1952 * s.dn[252][11]);
        let eq154_e1955_d_n12: f64 = (eq154_e1952 * s.dn[252][12]);
        let eq154_e1955_d_n13: f64 = (eq154_e1952 * s.dn[252][13]);
        let eq154_e1955_d_n14: f64 = (eq154_e1952 * s.dn[252][14]);
        let eq154_e1955_d_n15: f64 = (eq154_e1952 * s.dn[252][15]);
        let eq154_e1955_d_n16: f64 = (eq154_e1952 * s.dn[252][16]);
        let eq154_e1955_d_n17: f64 = (eq154_e1952 * s.dn[252][17]);
        let eq154_e1955_d_n18: f64 = (eq154_e1952 * s.dn[252][18]);
        let eq154_e1955_d_n19: f64 = (eq154_e1952 * s.dn[252][19]);
        let eq154_e1955_d_n20: f64 = (eq154_e1952 * s.dn[252][20]);
        let eq154_e1955_d_n21: f64 = (eq154_e1952 * s.dn[252][21]);
        let eq154_e1955_d_n22: f64 = (eq154_e1952 * s.dn[252][22]);
        let eq154_e1955_q: f64 = (eq154_e1952 * eq154_e1954_q);
        let eq154_e1955_q_d_n0: f64 = (eq154_e1952 * s.dn[252][0]);
        let eq154_e1955_q_d_n1: f64 = (eq154_e1952 * s.dn[252][1]);
        let eq154_e1955_q_d_n2: f64 = (eq154_e1952 * s.dn[252][2]);
        let eq154_e1955_q_d_n3: f64 = (eq154_e1952 * s.dn[252][3]);
        let eq154_e1955_q_d_n4: f64 = (eq154_e1952 * s.dn[252][4]);
        let eq154_e1955_q_d_n5: f64 = (eq154_e1952 * s.dn[252][5]);
        let eq154_e1955_q_d_n6: f64 = (eq154_e1952 * s.dn[252][6]);
        let eq154_e1955_q_d_n7: f64 = (eq154_e1952 * s.dn[252][7]);
        let eq154_e1955_q_d_n8: f64 = (eq154_e1952 * s.dn[252][8]);
        let eq154_e1955_q_d_n9: f64 = (eq154_e1952 * s.dn[252][9]);
        let eq154_e1955_q_d_n10: f64 = (eq154_e1952 * s.dn[252][10]);
        let eq154_e1955_q_d_n11: f64 = (eq154_e1952 * s.dn[252][11]);
        let eq154_e1955_q_d_n12: f64 = (eq154_e1952 * s.dn[252][12]);
        let eq154_e1955_q_d_n13: f64 = (eq154_e1952 * s.dn[252][13]);
        let eq154_e1955_q_d_n14: f64 = (eq154_e1952 * s.dn[252][14]);
        let eq154_e1955_q_d_n15: f64 = (eq154_e1952 * s.dn[252][15]);
        let eq154_e1955_q_d_n16: f64 = (eq154_e1952 * s.dn[252][16]);
        let eq154_e1955_q_d_n17: f64 = (eq154_e1952 * s.dn[252][17]);
        let eq154_e1955_q_d_n18: f64 = (eq154_e1952 * s.dn[252][18]);
        let eq154_e1955_q_d_n19: f64 = (eq154_e1952 * s.dn[252][19]);
        let eq154_e1955_q_d_n20: f64 = (eq154_e1952 * s.dn[252][20]);
        let eq154_e1955_q_d_n21: f64 = (eq154_e1952 * s.dn[252][21]);
        let eq154_e1955_q_d_n22: f64 = (eq154_e1952 * s.dn[252][22]);
        (eq154_e1955, eq154_e1955_d_n0, eq154_e1955_d_n1, eq154_e1955_d_n2, eq154_e1955_d_n3, eq154_e1955_d_n4, eq154_e1955_d_n5, eq154_e1955_d_n6, eq154_e1955_d_n7, eq154_e1955_d_n8, eq154_e1955_d_n9, eq154_e1955_d_n10, eq154_e1955_d_n11, eq154_e1955_d_n12, eq154_e1955_d_n13, eq154_e1955_d_n14, eq154_e1955_d_n15, eq154_e1955_d_n16, eq154_e1955_d_n17, eq154_e1955_d_n18, eq154_e1955_d_n19, eq154_e1955_d_n20, eq154_e1955_d_n21, eq154_e1955_d_n22, eq154_e1955_q, eq154_e1955_q_d_n0, eq154_e1955_q_d_n1, eq154_e1955_q_d_n2, eq154_e1955_q_d_n3, eq154_e1955_q_d_n4, eq154_e1955_q_d_n5, eq154_e1955_q_d_n6, eq154_e1955_q_d_n7, eq154_e1955_q_d_n8, eq154_e1955_q_d_n9, eq154_e1955_q_d_n10, eq154_e1955_q_d_n11, eq154_e1955_q_d_n12, eq154_e1955_q_d_n13, eq154_e1955_q_d_n14, eq154_e1955_q_d_n15, eq154_e1955_q_d_n16, eq154_e1955_q_d_n17, eq154_e1955_q_d_n18, eq154_e1955_q_d_n19, eq154_e1955_q_d_n20, eq154_e1955_q_d_n21, eq154_e1955_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq154_reactive_node_derivatives: [f64; 23] = [eq154_e1957_q_d_n0, eq154_e1957_q_d_n1, eq154_e1957_q_d_n2, eq154_e1957_q_d_n3, eq154_e1957_q_d_n4, eq154_e1957_q_d_n5, eq154_e1957_q_d_n6, eq154_e1957_q_d_n7, eq154_e1957_q_d_n8, eq154_e1957_q_d_n9, eq154_e1957_q_d_n10, eq154_e1957_q_d_n11, eq154_e1957_q_d_n12, eq154_e1957_q_d_n13, eq154_e1957_q_d_n14, eq154_e1957_q_d_n15, eq154_e1957_q_d_n16, eq154_e1957_q_d_n17, eq154_e1957_q_d_n18, eq154_e1957_q_d_n19, eq154_e1957_q_d_n20, eq154_e1957_q_d_n21, eq154_e1957_q_d_n22];
        let eq154_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            &nodes,
            &eq154_reactive_node_derivatives,
            &branches,
            &eq154_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
