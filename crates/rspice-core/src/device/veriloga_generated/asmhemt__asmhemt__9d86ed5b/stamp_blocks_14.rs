#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_167_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq167_e2115, eq167_e2115_d_n0, eq167_e2115_d_n1, eq167_e2115_d_n2, eq167_e2115_d_n3, eq167_e2115_d_n4, eq167_e2115_d_n5, eq167_e2115_d_n6, eq167_e2115_d_n7, eq167_e2115_d_n8, eq167_e2115_d_n9, eq167_e2115_d_n10, eq167_e2115_d_n11, eq167_e2115_d_n12, eq167_e2115_d_n13, eq167_e2115_d_n14, eq167_e2115_d_n15, eq167_e2115_d_n16, eq167_e2115_d_n17, eq167_e2115_d_n18, eq167_e2115_d_n19, eq167_e2115_d_n20, eq167_e2115_d_n21, eq167_e2115_d_n22,) = {
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
        let eq167_e2112: f64 = self.eval_ddt(66, eq167_e2111);
        let eq167_e2112_d_n0: f64 = self.ddt_jacobian(eq167_e2111_d_n0);
        let eq167_e2112_d_n1: f64 = self.ddt_jacobian(eq167_e2111_d_n1);
        let eq167_e2112_d_n2: f64 = self.ddt_jacobian(eq167_e2111_d_n2);
        let eq167_e2112_d_n3: f64 = self.ddt_jacobian(eq167_e2111_d_n3);
        let eq167_e2112_d_n4: f64 = self.ddt_jacobian(eq167_e2111_d_n4);
        let eq167_e2112_d_n5: f64 = self.ddt_jacobian(eq167_e2111_d_n5);
        let eq167_e2112_d_n6: f64 = self.ddt_jacobian(eq167_e2111_d_n6);
        let eq167_e2112_d_n7: f64 = self.ddt_jacobian(eq167_e2111_d_n7);
        let eq167_e2112_d_n8: f64 = self.ddt_jacobian(eq167_e2111_d_n8);
        let eq167_e2112_d_n9: f64 = self.ddt_jacobian(eq167_e2111_d_n9);
        let eq167_e2112_d_n10: f64 = self.ddt_jacobian(eq167_e2111_d_n10);
        let eq167_e2112_d_n11: f64 = self.ddt_jacobian(eq167_e2111_d_n11);
        let eq167_e2112_d_n12: f64 = self.ddt_jacobian(eq167_e2111_d_n12);
        let eq167_e2112_d_n13: f64 = self.ddt_jacobian(eq167_e2111_d_n13);
        let eq167_e2112_d_n14: f64 = self.ddt_jacobian(eq167_e2111_d_n14);
        let eq167_e2112_d_n15: f64 = self.ddt_jacobian(eq167_e2111_d_n15);
        let eq167_e2112_d_n16: f64 = self.ddt_jacobian(eq167_e2111_d_n16);
        let eq167_e2112_d_n17: f64 = self.ddt_jacobian(eq167_e2111_d_n17);
        let eq167_e2112_d_n18: f64 = self.ddt_jacobian(eq167_e2111_d_n18);
        let eq167_e2112_d_n19: f64 = self.ddt_jacobian(eq167_e2111_d_n19);
        let eq167_e2112_d_n20: f64 = self.ddt_jacobian(eq167_e2111_d_n20);
        let eq167_e2112_d_n21: f64 = self.ddt_jacobian(eq167_e2111_d_n21);
        let eq167_e2112_d_n22: f64 = self.ddt_jacobian(eq167_e2111_d_n22);
        let eq167_e2113: f64 = (p.p7 * eq167_e2112);
        let eq167_e2113_d_n0: f64 = (p.p7 * eq167_e2112_d_n0);
        let eq167_e2113_d_n1: f64 = (p.p7 * eq167_e2112_d_n1);
        let eq167_e2113_d_n2: f64 = (p.p7 * eq167_e2112_d_n2);
        let eq167_e2113_d_n3: f64 = (p.p7 * eq167_e2112_d_n3);
        let eq167_e2113_d_n4: f64 = (p.p7 * eq167_e2112_d_n4);
        let eq167_e2113_d_n5: f64 = (p.p7 * eq167_e2112_d_n5);
        let eq167_e2113_d_n6: f64 = (p.p7 * eq167_e2112_d_n6);
        let eq167_e2113_d_n7: f64 = (p.p7 * eq167_e2112_d_n7);
        let eq167_e2113_d_n8: f64 = (p.p7 * eq167_e2112_d_n8);
        let eq167_e2113_d_n9: f64 = (p.p7 * eq167_e2112_d_n9);
        let eq167_e2113_d_n10: f64 = (p.p7 * eq167_e2112_d_n10);
        let eq167_e2113_d_n11: f64 = (p.p7 * eq167_e2112_d_n11);
        let eq167_e2113_d_n12: f64 = (p.p7 * eq167_e2112_d_n12);
        let eq167_e2113_d_n13: f64 = (p.p7 * eq167_e2112_d_n13);
        let eq167_e2113_d_n14: f64 = (p.p7 * eq167_e2112_d_n14);
        let eq167_e2113_d_n15: f64 = (p.p7 * eq167_e2112_d_n15);
        let eq167_e2113_d_n16: f64 = (p.p7 * eq167_e2112_d_n16);
        let eq167_e2113_d_n17: f64 = (p.p7 * eq167_e2112_d_n17);
        let eq167_e2113_d_n18: f64 = (p.p7 * eq167_e2112_d_n18);
        let eq167_e2113_d_n19: f64 = (p.p7 * eq167_e2112_d_n19);
        let eq167_e2113_d_n20: f64 = (p.p7 * eq167_e2112_d_n20);
        let eq167_e2113_d_n21: f64 = (p.p7 * eq167_e2112_d_n21);
        let eq167_e2113_d_n22: f64 = (p.p7 * eq167_e2112_d_n22);
        (eq167_e2113, eq167_e2113_d_n0, eq167_e2113_d_n1, eq167_e2113_d_n2, eq167_e2113_d_n3, eq167_e2113_d_n4, eq167_e2113_d_n5, eq167_e2113_d_n6, eq167_e2113_d_n7, eq167_e2113_d_n8, eq167_e2113_d_n9, eq167_e2113_d_n10, eq167_e2113_d_n11, eq167_e2113_d_n12, eq167_e2113_d_n13, eq167_e2113_d_n14, eq167_e2113_d_n15, eq167_e2113_d_n16, eq167_e2113_d_n17, eq167_e2113_d_n18, eq167_e2113_d_n19, eq167_e2113_d_n20, eq167_e2113_d_n21, eq167_e2113_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq167_value: f64 = eq167_e2115;
        let eq167_node_derivatives: [f64; 23] = [eq167_e2115_d_n0, eq167_e2115_d_n1, eq167_e2115_d_n2, eq167_e2115_d_n3, eq167_e2115_d_n4, eq167_e2115_d_n5, eq167_e2115_d_n6, eq167_e2115_d_n7, eq167_e2115_d_n8, eq167_e2115_d_n9, eq167_e2115_d_n10, eq167_e2115_d_n11, eq167_e2115_d_n12, eq167_e2115_d_n13, eq167_e2115_d_n14, eq167_e2115_d_n15, eq167_e2115_d_n16, eq167_e2115_d_n17, eq167_e2115_d_n18, eq167_e2115_d_n19, eq167_e2115_d_n20, eq167_e2115_d_n21, eq167_e2115_d_n22];
        let eq167_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            self.multiplicity * (eq167_value),
            &nodes,
            &eq167_node_derivatives,
            &branches,
            &eq167_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_168_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq168_e2124, eq168_e2124_d_n0, eq168_e2124_d_n1, eq168_e2124_d_n2, eq168_e2124_d_n3, eq168_e2124_d_n4, eq168_e2124_d_n5, eq168_e2124_d_n6, eq168_e2124_d_n7, eq168_e2124_d_n8, eq168_e2124_d_n9, eq168_e2124_d_n10, eq168_e2124_d_n11, eq168_e2124_d_n12, eq168_e2124_d_n13, eq168_e2124_d_n14, eq168_e2124_d_n15, eq168_e2124_d_n16, eq168_e2124_d_n17, eq168_e2124_d_n18, eq168_e2124_d_n19, eq168_e2124_d_n20, eq168_e2124_d_n21, eq168_e2124_d_n22,) = {
    if ((s.v[590] != 0.0) && (s.v[591] != 0.0)) {
        let eq168_e2121: f64 = self.eval_ddt(67, s.v[277]);
        let eq168_e2121_d_n0: f64 = self.ddt_jacobian(s.dn[277][0]);
        let eq168_e2121_d_n1: f64 = self.ddt_jacobian(s.dn[277][1]);
        let eq168_e2121_d_n2: f64 = self.ddt_jacobian(s.dn[277][2]);
        let eq168_e2121_d_n3: f64 = self.ddt_jacobian(s.dn[277][3]);
        let eq168_e2121_d_n4: f64 = self.ddt_jacobian(s.dn[277][4]);
        let eq168_e2121_d_n5: f64 = self.ddt_jacobian(s.dn[277][5]);
        let eq168_e2121_d_n6: f64 = self.ddt_jacobian(s.dn[277][6]);
        let eq168_e2121_d_n7: f64 = self.ddt_jacobian(s.dn[277][7]);
        let eq168_e2121_d_n8: f64 = self.ddt_jacobian(s.dn[277][8]);
        let eq168_e2121_d_n9: f64 = self.ddt_jacobian(s.dn[277][9]);
        let eq168_e2121_d_n10: f64 = self.ddt_jacobian(s.dn[277][10]);
        let eq168_e2121_d_n11: f64 = self.ddt_jacobian(s.dn[277][11]);
        let eq168_e2121_d_n12: f64 = self.ddt_jacobian(s.dn[277][12]);
        let eq168_e2121_d_n13: f64 = self.ddt_jacobian(s.dn[277][13]);
        let eq168_e2121_d_n14: f64 = self.ddt_jacobian(s.dn[277][14]);
        let eq168_e2121_d_n15: f64 = self.ddt_jacobian(s.dn[277][15]);
        let eq168_e2121_d_n16: f64 = self.ddt_jacobian(s.dn[277][16]);
        let eq168_e2121_d_n17: f64 = self.ddt_jacobian(s.dn[277][17]);
        let eq168_e2121_d_n18: f64 = self.ddt_jacobian(s.dn[277][18]);
        let eq168_e2121_d_n19: f64 = self.ddt_jacobian(s.dn[277][19]);
        let eq168_e2121_d_n20: f64 = self.ddt_jacobian(s.dn[277][20]);
        let eq168_e2121_d_n21: f64 = self.ddt_jacobian(s.dn[277][21]);
        let eq168_e2121_d_n22: f64 = self.ddt_jacobian(s.dn[277][22]);
        let eq168_e2122: f64 = (p.p7 * eq168_e2121);
        let eq168_e2122_d_n0: f64 = (p.p7 * eq168_e2121_d_n0);
        let eq168_e2122_d_n1: f64 = (p.p7 * eq168_e2121_d_n1);
        let eq168_e2122_d_n2: f64 = (p.p7 * eq168_e2121_d_n2);
        let eq168_e2122_d_n3: f64 = (p.p7 * eq168_e2121_d_n3);
        let eq168_e2122_d_n4: f64 = (p.p7 * eq168_e2121_d_n4);
        let eq168_e2122_d_n5: f64 = (p.p7 * eq168_e2121_d_n5);
        let eq168_e2122_d_n6: f64 = (p.p7 * eq168_e2121_d_n6);
        let eq168_e2122_d_n7: f64 = (p.p7 * eq168_e2121_d_n7);
        let eq168_e2122_d_n8: f64 = (p.p7 * eq168_e2121_d_n8);
        let eq168_e2122_d_n9: f64 = (p.p7 * eq168_e2121_d_n9);
        let eq168_e2122_d_n10: f64 = (p.p7 * eq168_e2121_d_n10);
        let eq168_e2122_d_n11: f64 = (p.p7 * eq168_e2121_d_n11);
        let eq168_e2122_d_n12: f64 = (p.p7 * eq168_e2121_d_n12);
        let eq168_e2122_d_n13: f64 = (p.p7 * eq168_e2121_d_n13);
        let eq168_e2122_d_n14: f64 = (p.p7 * eq168_e2121_d_n14);
        let eq168_e2122_d_n15: f64 = (p.p7 * eq168_e2121_d_n15);
        let eq168_e2122_d_n16: f64 = (p.p7 * eq168_e2121_d_n16);
        let eq168_e2122_d_n17: f64 = (p.p7 * eq168_e2121_d_n17);
        let eq168_e2122_d_n18: f64 = (p.p7 * eq168_e2121_d_n18);
        let eq168_e2122_d_n19: f64 = (p.p7 * eq168_e2121_d_n19);
        let eq168_e2122_d_n20: f64 = (p.p7 * eq168_e2121_d_n20);
        let eq168_e2122_d_n21: f64 = (p.p7 * eq168_e2121_d_n21);
        let eq168_e2122_d_n22: f64 = (p.p7 * eq168_e2121_d_n22);
        (eq168_e2122, eq168_e2122_d_n0, eq168_e2122_d_n1, eq168_e2122_d_n2, eq168_e2122_d_n3, eq168_e2122_d_n4, eq168_e2122_d_n5, eq168_e2122_d_n6, eq168_e2122_d_n7, eq168_e2122_d_n8, eq168_e2122_d_n9, eq168_e2122_d_n10, eq168_e2122_d_n11, eq168_e2122_d_n12, eq168_e2122_d_n13, eq168_e2122_d_n14, eq168_e2122_d_n15, eq168_e2122_d_n16, eq168_e2122_d_n17, eq168_e2122_d_n18, eq168_e2122_d_n19, eq168_e2122_d_n20, eq168_e2122_d_n21, eq168_e2122_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq168_value: f64 = eq168_e2124;
        let eq168_node_derivatives: [f64; 23] = [eq168_e2124_d_n0, eq168_e2124_d_n1, eq168_e2124_d_n2, eq168_e2124_d_n3, eq168_e2124_d_n4, eq168_e2124_d_n5, eq168_e2124_d_n6, eq168_e2124_d_n7, eq168_e2124_d_n8, eq168_e2124_d_n9, eq168_e2124_d_n10, eq168_e2124_d_n11, eq168_e2124_d_n12, eq168_e2124_d_n13, eq168_e2124_d_n14, eq168_e2124_d_n15, eq168_e2124_d_n16, eq168_e2124_d_n17, eq168_e2124_d_n18, eq168_e2124_d_n19, eq168_e2124_d_n20, eq168_e2124_d_n21, eq168_e2124_d_n22];
        let eq168_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[17]),
            Some(nodes[16]),
            self.multiplicity * (eq168_value),
            &nodes,
            &eq168_node_derivatives,
            &branches,
            &eq168_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_169_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq169_e2135, eq169_e2135_d_n0, eq169_e2135_d_n1, eq169_e2135_d_n2, eq169_e2135_d_n3, eq169_e2135_d_n4, eq169_e2135_d_n5, eq169_e2135_d_n6, eq169_e2135_d_n7, eq169_e2135_d_n8, eq169_e2135_d_n9, eq169_e2135_d_n10, eq169_e2135_d_n11, eq169_e2135_d_n12, eq169_e2135_d_n13, eq169_e2135_d_n14, eq169_e2135_d_n15, eq169_e2135_d_n16, eq169_e2135_d_n17, eq169_e2135_d_n18, eq169_e2135_d_n19, eq169_e2135_d_n20, eq169_e2135_d_n21, eq169_e2135_d_n22,) = {
    if (((s.v[590] != 0.0) && (s.v[591] != 0.0)) && (s.v[592] != 0.0)) {
        let eq169_e2132: f64 = self.eval_ddt(68, s.v[276]);
        let eq169_e2132_d_n0: f64 = self.ddt_jacobian(s.dn[276][0]);
        let eq169_e2132_d_n1: f64 = self.ddt_jacobian(s.dn[276][1]);
        let eq169_e2132_d_n2: f64 = self.ddt_jacobian(s.dn[276][2]);
        let eq169_e2132_d_n3: f64 = self.ddt_jacobian(s.dn[276][3]);
        let eq169_e2132_d_n4: f64 = self.ddt_jacobian(s.dn[276][4]);
        let eq169_e2132_d_n5: f64 = self.ddt_jacobian(s.dn[276][5]);
        let eq169_e2132_d_n6: f64 = self.ddt_jacobian(s.dn[276][6]);
        let eq169_e2132_d_n7: f64 = self.ddt_jacobian(s.dn[276][7]);
        let eq169_e2132_d_n8: f64 = self.ddt_jacobian(s.dn[276][8]);
        let eq169_e2132_d_n9: f64 = self.ddt_jacobian(s.dn[276][9]);
        let eq169_e2132_d_n10: f64 = self.ddt_jacobian(s.dn[276][10]);
        let eq169_e2132_d_n11: f64 = self.ddt_jacobian(s.dn[276][11]);
        let eq169_e2132_d_n12: f64 = self.ddt_jacobian(s.dn[276][12]);
        let eq169_e2132_d_n13: f64 = self.ddt_jacobian(s.dn[276][13]);
        let eq169_e2132_d_n14: f64 = self.ddt_jacobian(s.dn[276][14]);
        let eq169_e2132_d_n15: f64 = self.ddt_jacobian(s.dn[276][15]);
        let eq169_e2132_d_n16: f64 = self.ddt_jacobian(s.dn[276][16]);
        let eq169_e2132_d_n17: f64 = self.ddt_jacobian(s.dn[276][17]);
        let eq169_e2132_d_n18: f64 = self.ddt_jacobian(s.dn[276][18]);
        let eq169_e2132_d_n19: f64 = self.ddt_jacobian(s.dn[276][19]);
        let eq169_e2132_d_n20: f64 = self.ddt_jacobian(s.dn[276][20]);
        let eq169_e2132_d_n21: f64 = self.ddt_jacobian(s.dn[276][21]);
        let eq169_e2132_d_n22: f64 = self.ddt_jacobian(s.dn[276][22]);
        let eq169_e2133: f64 = (p.p7 * eq169_e2132);
        let eq169_e2133_d_n0: f64 = (p.p7 * eq169_e2132_d_n0);
        let eq169_e2133_d_n1: f64 = (p.p7 * eq169_e2132_d_n1);
        let eq169_e2133_d_n2: f64 = (p.p7 * eq169_e2132_d_n2);
        let eq169_e2133_d_n3: f64 = (p.p7 * eq169_e2132_d_n3);
        let eq169_e2133_d_n4: f64 = (p.p7 * eq169_e2132_d_n4);
        let eq169_e2133_d_n5: f64 = (p.p7 * eq169_e2132_d_n5);
        let eq169_e2133_d_n6: f64 = (p.p7 * eq169_e2132_d_n6);
        let eq169_e2133_d_n7: f64 = (p.p7 * eq169_e2132_d_n7);
        let eq169_e2133_d_n8: f64 = (p.p7 * eq169_e2132_d_n8);
        let eq169_e2133_d_n9: f64 = (p.p7 * eq169_e2132_d_n9);
        let eq169_e2133_d_n10: f64 = (p.p7 * eq169_e2132_d_n10);
        let eq169_e2133_d_n11: f64 = (p.p7 * eq169_e2132_d_n11);
        let eq169_e2133_d_n12: f64 = (p.p7 * eq169_e2132_d_n12);
        let eq169_e2133_d_n13: f64 = (p.p7 * eq169_e2132_d_n13);
        let eq169_e2133_d_n14: f64 = (p.p7 * eq169_e2132_d_n14);
        let eq169_e2133_d_n15: f64 = (p.p7 * eq169_e2132_d_n15);
        let eq169_e2133_d_n16: f64 = (p.p7 * eq169_e2132_d_n16);
        let eq169_e2133_d_n17: f64 = (p.p7 * eq169_e2132_d_n17);
        let eq169_e2133_d_n18: f64 = (p.p7 * eq169_e2132_d_n18);
        let eq169_e2133_d_n19: f64 = (p.p7 * eq169_e2132_d_n19);
        let eq169_e2133_d_n20: f64 = (p.p7 * eq169_e2132_d_n20);
        let eq169_e2133_d_n21: f64 = (p.p7 * eq169_e2132_d_n21);
        let eq169_e2133_d_n22: f64 = (p.p7 * eq169_e2132_d_n22);
        (eq169_e2133, eq169_e2133_d_n0, eq169_e2133_d_n1, eq169_e2133_d_n2, eq169_e2133_d_n3, eq169_e2133_d_n4, eq169_e2133_d_n5, eq169_e2133_d_n6, eq169_e2133_d_n7, eq169_e2133_d_n8, eq169_e2133_d_n9, eq169_e2133_d_n10, eq169_e2133_d_n11, eq169_e2133_d_n12, eq169_e2133_d_n13, eq169_e2133_d_n14, eq169_e2133_d_n15, eq169_e2133_d_n16, eq169_e2133_d_n17, eq169_e2133_d_n18, eq169_e2133_d_n19, eq169_e2133_d_n20, eq169_e2133_d_n21, eq169_e2133_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq169_value: f64 = eq169_e2135;
        let eq169_node_derivatives: [f64; 23] = [eq169_e2135_d_n0, eq169_e2135_d_n1, eq169_e2135_d_n2, eq169_e2135_d_n3, eq169_e2135_d_n4, eq169_e2135_d_n5, eq169_e2135_d_n6, eq169_e2135_d_n7, eq169_e2135_d_n8, eq169_e2135_d_n9, eq169_e2135_d_n10, eq169_e2135_d_n11, eq169_e2135_d_n12, eq169_e2135_d_n13, eq169_e2135_d_n14, eq169_e2135_d_n15, eq169_e2135_d_n16, eq169_e2135_d_n17, eq169_e2135_d_n18, eq169_e2135_d_n19, eq169_e2135_d_n20, eq169_e2135_d_n21, eq169_e2135_d_n22];
        let eq169_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[16]),
            self.multiplicity * (eq169_value),
            &nodes,
            &eq169_node_derivatives,
            &branches,
            &eq169_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_170_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq170_e2148, eq170_e2148_d_n0, eq170_e2148_d_n1, eq170_e2148_d_n2, eq170_e2148_d_n3, eq170_e2148_d_n4, eq170_e2148_d_n5, eq170_e2148_d_n6, eq170_e2148_d_n7, eq170_e2148_d_n8, eq170_e2148_d_n9, eq170_e2148_d_n10, eq170_e2148_d_n11, eq170_e2148_d_n12, eq170_e2148_d_n13, eq170_e2148_d_n14, eq170_e2148_d_n15, eq170_e2148_d_n16, eq170_e2148_d_n17, eq170_e2148_d_n18, eq170_e2148_d_n19, eq170_e2148_d_n20, eq170_e2148_d_n21, eq170_e2148_d_n22,) = {
    if (((s.v[590] != 0.0) && (s.v[591] != 0.0)) && (s.v[592] != 0.0)) {
        let eq170_e2143: f64 = self.eval_ddt(69, s.v[276]);
        let eq170_e2143_d_n0: f64 = self.ddt_jacobian(s.dn[276][0]);
        let eq170_e2143_d_n1: f64 = self.ddt_jacobian(s.dn[276][1]);
        let eq170_e2143_d_n2: f64 = self.ddt_jacobian(s.dn[276][2]);
        let eq170_e2143_d_n3: f64 = self.ddt_jacobian(s.dn[276][3]);
        let eq170_e2143_d_n4: f64 = self.ddt_jacobian(s.dn[276][4]);
        let eq170_e2143_d_n5: f64 = self.ddt_jacobian(s.dn[276][5]);
        let eq170_e2143_d_n6: f64 = self.ddt_jacobian(s.dn[276][6]);
        let eq170_e2143_d_n7: f64 = self.ddt_jacobian(s.dn[276][7]);
        let eq170_e2143_d_n8: f64 = self.ddt_jacobian(s.dn[276][8]);
        let eq170_e2143_d_n9: f64 = self.ddt_jacobian(s.dn[276][9]);
        let eq170_e2143_d_n10: f64 = self.ddt_jacobian(s.dn[276][10]);
        let eq170_e2143_d_n11: f64 = self.ddt_jacobian(s.dn[276][11]);
        let eq170_e2143_d_n12: f64 = self.ddt_jacobian(s.dn[276][12]);
        let eq170_e2143_d_n13: f64 = self.ddt_jacobian(s.dn[276][13]);
        let eq170_e2143_d_n14: f64 = self.ddt_jacobian(s.dn[276][14]);
        let eq170_e2143_d_n15: f64 = self.ddt_jacobian(s.dn[276][15]);
        let eq170_e2143_d_n16: f64 = self.ddt_jacobian(s.dn[276][16]);
        let eq170_e2143_d_n17: f64 = self.ddt_jacobian(s.dn[276][17]);
        let eq170_e2143_d_n18: f64 = self.ddt_jacobian(s.dn[276][18]);
        let eq170_e2143_d_n19: f64 = self.ddt_jacobian(s.dn[276][19]);
        let eq170_e2143_d_n20: f64 = self.ddt_jacobian(s.dn[276][20]);
        let eq170_e2143_d_n21: f64 = self.ddt_jacobian(s.dn[276][21]);
        let eq170_e2143_d_n22: f64 = self.ddt_jacobian(s.dn[276][22]);
        let eq170_e2144: f64 = (p.p7 * eq170_e2143);
        let eq170_e2144_d_n0: f64 = (p.p7 * eq170_e2143_d_n0);
        let eq170_e2144_d_n1: f64 = (p.p7 * eq170_e2143_d_n1);
        let eq170_e2144_d_n2: f64 = (p.p7 * eq170_e2143_d_n2);
        let eq170_e2144_d_n3: f64 = (p.p7 * eq170_e2143_d_n3);
        let eq170_e2144_d_n4: f64 = (p.p7 * eq170_e2143_d_n4);
        let eq170_e2144_d_n5: f64 = (p.p7 * eq170_e2143_d_n5);
        let eq170_e2144_d_n6: f64 = (p.p7 * eq170_e2143_d_n6);
        let eq170_e2144_d_n7: f64 = (p.p7 * eq170_e2143_d_n7);
        let eq170_e2144_d_n8: f64 = (p.p7 * eq170_e2143_d_n8);
        let eq170_e2144_d_n9: f64 = (p.p7 * eq170_e2143_d_n9);
        let eq170_e2144_d_n10: f64 = (p.p7 * eq170_e2143_d_n10);
        let eq170_e2144_d_n11: f64 = (p.p7 * eq170_e2143_d_n11);
        let eq170_e2144_d_n12: f64 = (p.p7 * eq170_e2143_d_n12);
        let eq170_e2144_d_n13: f64 = (p.p7 * eq170_e2143_d_n13);
        let eq170_e2144_d_n14: f64 = (p.p7 * eq170_e2143_d_n14);
        let eq170_e2144_d_n15: f64 = (p.p7 * eq170_e2143_d_n15);
        let eq170_e2144_d_n16: f64 = (p.p7 * eq170_e2143_d_n16);
        let eq170_e2144_d_n17: f64 = (p.p7 * eq170_e2143_d_n17);
        let eq170_e2144_d_n18: f64 = (p.p7 * eq170_e2143_d_n18);
        let eq170_e2144_d_n19: f64 = (p.p7 * eq170_e2143_d_n19);
        let eq170_e2144_d_n20: f64 = (p.p7 * eq170_e2143_d_n20);
        let eq170_e2144_d_n21: f64 = (p.p7 * eq170_e2143_d_n21);
        let eq170_e2144_d_n22: f64 = (p.p7 * eq170_e2143_d_n22);
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
        (eq170_e2146, eq170_e2146_d_n0, eq170_e2146_d_n1, eq170_e2146_d_n2, eq170_e2146_d_n3, eq170_e2146_d_n4, eq170_e2146_d_n5, eq170_e2146_d_n6, eq170_e2146_d_n7, eq170_e2146_d_n8, eq170_e2146_d_n9, eq170_e2146_d_n10, eq170_e2146_d_n11, eq170_e2146_d_n12, eq170_e2146_d_n13, eq170_e2146_d_n14, eq170_e2146_d_n15, eq170_e2146_d_n16, eq170_e2146_d_n17, eq170_e2146_d_n18, eq170_e2146_d_n19, eq170_e2146_d_n20, eq170_e2146_d_n21, eq170_e2146_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq170_value: f64 = eq170_e2148;
        let eq170_node_derivatives: [f64; 23] = [eq170_e2148_d_n0, eq170_e2148_d_n1, eq170_e2148_d_n2, eq170_e2148_d_n3, eq170_e2148_d_n4, eq170_e2148_d_n5, eq170_e2148_d_n6, eq170_e2148_d_n7, eq170_e2148_d_n8, eq170_e2148_d_n9, eq170_e2148_d_n10, eq170_e2148_d_n11, eq170_e2148_d_n12, eq170_e2148_d_n13, eq170_e2148_d_n14, eq170_e2148_d_n15, eq170_e2148_d_n16, eq170_e2148_d_n17, eq170_e2148_d_n18, eq170_e2148_d_n19, eq170_e2148_d_n20, eq170_e2148_d_n21, eq170_e2148_d_n22];
        let eq170_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[16]),
            self.multiplicity * (eq170_value),
            &nodes,
            &eq170_node_derivatives,
            &branches,
            &eq170_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_171_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq171_e2160, eq171_e2160_d_n0, eq171_e2160_d_n1, eq171_e2160_d_n2, eq171_e2160_d_n3, eq171_e2160_d_n4, eq171_e2160_d_n5, eq171_e2160_d_n6, eq171_e2160_d_n7, eq171_e2160_d_n8, eq171_e2160_d_n9, eq171_e2160_d_n10, eq171_e2160_d_n11, eq171_e2160_d_n12, eq171_e2160_d_n13, eq171_e2160_d_n14, eq171_e2160_d_n15, eq171_e2160_d_n16, eq171_e2160_d_n17, eq171_e2160_d_n18, eq171_e2160_d_n19, eq171_e2160_d_n20, eq171_e2160_d_n21, eq171_e2160_d_n22,) = {
    if (((s.v[590] != 0.0) && (s.v[591] != 0.0)) && (!(s.v[592] != 0.0))) {
        let eq171_e2157: f64 = self.eval_ddt(70, s.v[276]);
        let eq171_e2157_d_n0: f64 = self.ddt_jacobian(s.dn[276][0]);
        let eq171_e2157_d_n1: f64 = self.ddt_jacobian(s.dn[276][1]);
        let eq171_e2157_d_n2: f64 = self.ddt_jacobian(s.dn[276][2]);
        let eq171_e2157_d_n3: f64 = self.ddt_jacobian(s.dn[276][3]);
        let eq171_e2157_d_n4: f64 = self.ddt_jacobian(s.dn[276][4]);
        let eq171_e2157_d_n5: f64 = self.ddt_jacobian(s.dn[276][5]);
        let eq171_e2157_d_n6: f64 = self.ddt_jacobian(s.dn[276][6]);
        let eq171_e2157_d_n7: f64 = self.ddt_jacobian(s.dn[276][7]);
        let eq171_e2157_d_n8: f64 = self.ddt_jacobian(s.dn[276][8]);
        let eq171_e2157_d_n9: f64 = self.ddt_jacobian(s.dn[276][9]);
        let eq171_e2157_d_n10: f64 = self.ddt_jacobian(s.dn[276][10]);
        let eq171_e2157_d_n11: f64 = self.ddt_jacobian(s.dn[276][11]);
        let eq171_e2157_d_n12: f64 = self.ddt_jacobian(s.dn[276][12]);
        let eq171_e2157_d_n13: f64 = self.ddt_jacobian(s.dn[276][13]);
        let eq171_e2157_d_n14: f64 = self.ddt_jacobian(s.dn[276][14]);
        let eq171_e2157_d_n15: f64 = self.ddt_jacobian(s.dn[276][15]);
        let eq171_e2157_d_n16: f64 = self.ddt_jacobian(s.dn[276][16]);
        let eq171_e2157_d_n17: f64 = self.ddt_jacobian(s.dn[276][17]);
        let eq171_e2157_d_n18: f64 = self.ddt_jacobian(s.dn[276][18]);
        let eq171_e2157_d_n19: f64 = self.ddt_jacobian(s.dn[276][19]);
        let eq171_e2157_d_n20: f64 = self.ddt_jacobian(s.dn[276][20]);
        let eq171_e2157_d_n21: f64 = self.ddt_jacobian(s.dn[276][21]);
        let eq171_e2157_d_n22: f64 = self.ddt_jacobian(s.dn[276][22]);
        let eq171_e2158: f64 = (p.p7 * eq171_e2157);
        let eq171_e2158_d_n0: f64 = (p.p7 * eq171_e2157_d_n0);
        let eq171_e2158_d_n1: f64 = (p.p7 * eq171_e2157_d_n1);
        let eq171_e2158_d_n2: f64 = (p.p7 * eq171_e2157_d_n2);
        let eq171_e2158_d_n3: f64 = (p.p7 * eq171_e2157_d_n3);
        let eq171_e2158_d_n4: f64 = (p.p7 * eq171_e2157_d_n4);
        let eq171_e2158_d_n5: f64 = (p.p7 * eq171_e2157_d_n5);
        let eq171_e2158_d_n6: f64 = (p.p7 * eq171_e2157_d_n6);
        let eq171_e2158_d_n7: f64 = (p.p7 * eq171_e2157_d_n7);
        let eq171_e2158_d_n8: f64 = (p.p7 * eq171_e2157_d_n8);
        let eq171_e2158_d_n9: f64 = (p.p7 * eq171_e2157_d_n9);
        let eq171_e2158_d_n10: f64 = (p.p7 * eq171_e2157_d_n10);
        let eq171_e2158_d_n11: f64 = (p.p7 * eq171_e2157_d_n11);
        let eq171_e2158_d_n12: f64 = (p.p7 * eq171_e2157_d_n12);
        let eq171_e2158_d_n13: f64 = (p.p7 * eq171_e2157_d_n13);
        let eq171_e2158_d_n14: f64 = (p.p7 * eq171_e2157_d_n14);
        let eq171_e2158_d_n15: f64 = (p.p7 * eq171_e2157_d_n15);
        let eq171_e2158_d_n16: f64 = (p.p7 * eq171_e2157_d_n16);
        let eq171_e2158_d_n17: f64 = (p.p7 * eq171_e2157_d_n17);
        let eq171_e2158_d_n18: f64 = (p.p7 * eq171_e2157_d_n18);
        let eq171_e2158_d_n19: f64 = (p.p7 * eq171_e2157_d_n19);
        let eq171_e2158_d_n20: f64 = (p.p7 * eq171_e2157_d_n20);
        let eq171_e2158_d_n21: f64 = (p.p7 * eq171_e2157_d_n21);
        let eq171_e2158_d_n22: f64 = (p.p7 * eq171_e2157_d_n22);
        (eq171_e2158, eq171_e2158_d_n0, eq171_e2158_d_n1, eq171_e2158_d_n2, eq171_e2158_d_n3, eq171_e2158_d_n4, eq171_e2158_d_n5, eq171_e2158_d_n6, eq171_e2158_d_n7, eq171_e2158_d_n8, eq171_e2158_d_n9, eq171_e2158_d_n10, eq171_e2158_d_n11, eq171_e2158_d_n12, eq171_e2158_d_n13, eq171_e2158_d_n14, eq171_e2158_d_n15, eq171_e2158_d_n16, eq171_e2158_d_n17, eq171_e2158_d_n18, eq171_e2158_d_n19, eq171_e2158_d_n20, eq171_e2158_d_n21, eq171_e2158_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq171_value: f64 = eq171_e2160;
        let eq171_node_derivatives: [f64; 23] = [eq171_e2160_d_n0, eq171_e2160_d_n1, eq171_e2160_d_n2, eq171_e2160_d_n3, eq171_e2160_d_n4, eq171_e2160_d_n5, eq171_e2160_d_n6, eq171_e2160_d_n7, eq171_e2160_d_n8, eq171_e2160_d_n9, eq171_e2160_d_n10, eq171_e2160_d_n11, eq171_e2160_d_n12, eq171_e2160_d_n13, eq171_e2160_d_n14, eq171_e2160_d_n15, eq171_e2160_d_n16, eq171_e2160_d_n17, eq171_e2160_d_n18, eq171_e2160_d_n19, eq171_e2160_d_n20, eq171_e2160_d_n21, eq171_e2160_d_n22];
        let eq171_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[16]),
            self.multiplicity * (eq171_value),
            &nodes,
            &eq171_node_derivatives,
            &branches,
            &eq171_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_172_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq172_e2174, eq172_e2174_d_n0, eq172_e2174_d_n1, eq172_e2174_d_n2, eq172_e2174_d_n3, eq172_e2174_d_n4, eq172_e2174_d_n5, eq172_e2174_d_n6, eq172_e2174_d_n7, eq172_e2174_d_n8, eq172_e2174_d_n9, eq172_e2174_d_n10, eq172_e2174_d_n11, eq172_e2174_d_n12, eq172_e2174_d_n13, eq172_e2174_d_n14, eq172_e2174_d_n15, eq172_e2174_d_n16, eq172_e2174_d_n17, eq172_e2174_d_n18, eq172_e2174_d_n19, eq172_e2174_d_n20, eq172_e2174_d_n21, eq172_e2174_d_n22,) = {
    if (((s.v[590] != 0.0) && (s.v[591] != 0.0)) && (!(s.v[592] != 0.0))) {
        let eq172_e2169: f64 = self.eval_ddt(71, s.v[276]);
        let eq172_e2169_d_n0: f64 = self.ddt_jacobian(s.dn[276][0]);
        let eq172_e2169_d_n1: f64 = self.ddt_jacobian(s.dn[276][1]);
        let eq172_e2169_d_n2: f64 = self.ddt_jacobian(s.dn[276][2]);
        let eq172_e2169_d_n3: f64 = self.ddt_jacobian(s.dn[276][3]);
        let eq172_e2169_d_n4: f64 = self.ddt_jacobian(s.dn[276][4]);
        let eq172_e2169_d_n5: f64 = self.ddt_jacobian(s.dn[276][5]);
        let eq172_e2169_d_n6: f64 = self.ddt_jacobian(s.dn[276][6]);
        let eq172_e2169_d_n7: f64 = self.ddt_jacobian(s.dn[276][7]);
        let eq172_e2169_d_n8: f64 = self.ddt_jacobian(s.dn[276][8]);
        let eq172_e2169_d_n9: f64 = self.ddt_jacobian(s.dn[276][9]);
        let eq172_e2169_d_n10: f64 = self.ddt_jacobian(s.dn[276][10]);
        let eq172_e2169_d_n11: f64 = self.ddt_jacobian(s.dn[276][11]);
        let eq172_e2169_d_n12: f64 = self.ddt_jacobian(s.dn[276][12]);
        let eq172_e2169_d_n13: f64 = self.ddt_jacobian(s.dn[276][13]);
        let eq172_e2169_d_n14: f64 = self.ddt_jacobian(s.dn[276][14]);
        let eq172_e2169_d_n15: f64 = self.ddt_jacobian(s.dn[276][15]);
        let eq172_e2169_d_n16: f64 = self.ddt_jacobian(s.dn[276][16]);
        let eq172_e2169_d_n17: f64 = self.ddt_jacobian(s.dn[276][17]);
        let eq172_e2169_d_n18: f64 = self.ddt_jacobian(s.dn[276][18]);
        let eq172_e2169_d_n19: f64 = self.ddt_jacobian(s.dn[276][19]);
        let eq172_e2169_d_n20: f64 = self.ddt_jacobian(s.dn[276][20]);
        let eq172_e2169_d_n21: f64 = self.ddt_jacobian(s.dn[276][21]);
        let eq172_e2169_d_n22: f64 = self.ddt_jacobian(s.dn[276][22]);
        let eq172_e2170: f64 = (p.p7 * eq172_e2169);
        let eq172_e2170_d_n0: f64 = (p.p7 * eq172_e2169_d_n0);
        let eq172_e2170_d_n1: f64 = (p.p7 * eq172_e2169_d_n1);
        let eq172_e2170_d_n2: f64 = (p.p7 * eq172_e2169_d_n2);
        let eq172_e2170_d_n3: f64 = (p.p7 * eq172_e2169_d_n3);
        let eq172_e2170_d_n4: f64 = (p.p7 * eq172_e2169_d_n4);
        let eq172_e2170_d_n5: f64 = (p.p7 * eq172_e2169_d_n5);
        let eq172_e2170_d_n6: f64 = (p.p7 * eq172_e2169_d_n6);
        let eq172_e2170_d_n7: f64 = (p.p7 * eq172_e2169_d_n7);
        let eq172_e2170_d_n8: f64 = (p.p7 * eq172_e2169_d_n8);
        let eq172_e2170_d_n9: f64 = (p.p7 * eq172_e2169_d_n9);
        let eq172_e2170_d_n10: f64 = (p.p7 * eq172_e2169_d_n10);
        let eq172_e2170_d_n11: f64 = (p.p7 * eq172_e2169_d_n11);
        let eq172_e2170_d_n12: f64 = (p.p7 * eq172_e2169_d_n12);
        let eq172_e2170_d_n13: f64 = (p.p7 * eq172_e2169_d_n13);
        let eq172_e2170_d_n14: f64 = (p.p7 * eq172_e2169_d_n14);
        let eq172_e2170_d_n15: f64 = (p.p7 * eq172_e2169_d_n15);
        let eq172_e2170_d_n16: f64 = (p.p7 * eq172_e2169_d_n16);
        let eq172_e2170_d_n17: f64 = (p.p7 * eq172_e2169_d_n17);
        let eq172_e2170_d_n18: f64 = (p.p7 * eq172_e2169_d_n18);
        let eq172_e2170_d_n19: f64 = (p.p7 * eq172_e2169_d_n19);
        let eq172_e2170_d_n20: f64 = (p.p7 * eq172_e2169_d_n20);
        let eq172_e2170_d_n21: f64 = (p.p7 * eq172_e2169_d_n21);
        let eq172_e2170_d_n22: f64 = (p.p7 * eq172_e2169_d_n22);
        let eq172_e2172: f64 = (eq172_e2170 * p.p248);
        let eq172_e2172_d_n0: f64 = (eq172_e2170_d_n0 * p.p248);
        let eq172_e2172_d_n1: f64 = (eq172_e2170_d_n1 * p.p248);
        let eq172_e2172_d_n2: f64 = (eq172_e2170_d_n2 * p.p248);
        let eq172_e2172_d_n3: f64 = (eq172_e2170_d_n3 * p.p248);
        let eq172_e2172_d_n4: f64 = (eq172_e2170_d_n4 * p.p248);
        let eq172_e2172_d_n5: f64 = (eq172_e2170_d_n5 * p.p248);
        let eq172_e2172_d_n6: f64 = (eq172_e2170_d_n6 * p.p248);
        let eq172_e2172_d_n7: f64 = (eq172_e2170_d_n7 * p.p248);
        let eq172_e2172_d_n8: f64 = (eq172_e2170_d_n8 * p.p248);
        let eq172_e2172_d_n9: f64 = (eq172_e2170_d_n9 * p.p248);
        let eq172_e2172_d_n10: f64 = (eq172_e2170_d_n10 * p.p248);
        let eq172_e2172_d_n11: f64 = (eq172_e2170_d_n11 * p.p248);
        let eq172_e2172_d_n12: f64 = (eq172_e2170_d_n12 * p.p248);
        let eq172_e2172_d_n13: f64 = (eq172_e2170_d_n13 * p.p248);
        let eq172_e2172_d_n14: f64 = (eq172_e2170_d_n14 * p.p248);
        let eq172_e2172_d_n15: f64 = (eq172_e2170_d_n15 * p.p248);
        let eq172_e2172_d_n16: f64 = (eq172_e2170_d_n16 * p.p248);
        let eq172_e2172_d_n17: f64 = (eq172_e2170_d_n17 * p.p248);
        let eq172_e2172_d_n18: f64 = (eq172_e2170_d_n18 * p.p248);
        let eq172_e2172_d_n19: f64 = (eq172_e2170_d_n19 * p.p248);
        let eq172_e2172_d_n20: f64 = (eq172_e2170_d_n20 * p.p248);
        let eq172_e2172_d_n21: f64 = (eq172_e2170_d_n21 * p.p248);
        let eq172_e2172_d_n22: f64 = (eq172_e2170_d_n22 * p.p248);
        (eq172_e2172, eq172_e2172_d_n0, eq172_e2172_d_n1, eq172_e2172_d_n2, eq172_e2172_d_n3, eq172_e2172_d_n4, eq172_e2172_d_n5, eq172_e2172_d_n6, eq172_e2172_d_n7, eq172_e2172_d_n8, eq172_e2172_d_n9, eq172_e2172_d_n10, eq172_e2172_d_n11, eq172_e2172_d_n12, eq172_e2172_d_n13, eq172_e2172_d_n14, eq172_e2172_d_n15, eq172_e2172_d_n16, eq172_e2172_d_n17, eq172_e2172_d_n18, eq172_e2172_d_n19, eq172_e2172_d_n20, eq172_e2172_d_n21, eq172_e2172_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq172_value: f64 = eq172_e2174;
        let eq172_node_derivatives: [f64; 23] = [eq172_e2174_d_n0, eq172_e2174_d_n1, eq172_e2174_d_n2, eq172_e2174_d_n3, eq172_e2174_d_n4, eq172_e2174_d_n5, eq172_e2174_d_n6, eq172_e2174_d_n7, eq172_e2174_d_n8, eq172_e2174_d_n9, eq172_e2174_d_n10, eq172_e2174_d_n11, eq172_e2174_d_n12, eq172_e2174_d_n13, eq172_e2174_d_n14, eq172_e2174_d_n15, eq172_e2174_d_n16, eq172_e2174_d_n17, eq172_e2174_d_n18, eq172_e2174_d_n19, eq172_e2174_d_n20, eq172_e2174_d_n21, eq172_e2174_d_n22];
        let eq172_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[16]),
            self.multiplicity * (eq172_value),
            &nodes,
            &eq172_node_derivatives,
            &branches,
            &eq172_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_173_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq173_e2185, eq173_e2185_d_n0, eq173_e2185_d_n1, eq173_e2185_d_n2, eq173_e2185_d_n3, eq173_e2185_d_n4, eq173_e2185_d_n5, eq173_e2185_d_n6, eq173_e2185_d_n7, eq173_e2185_d_n8, eq173_e2185_d_n9, eq173_e2185_d_n10, eq173_e2185_d_n11, eq173_e2185_d_n12, eq173_e2185_d_n13, eq173_e2185_d_n14, eq173_e2185_d_n15, eq173_e2185_d_n16, eq173_e2185_d_n17, eq173_e2185_d_n18, eq173_e2185_d_n19, eq173_e2185_d_n20, eq173_e2185_d_n21, eq173_e2185_d_n22,) = {
    if ((s.v[590] != 0.0) && (s.v[591] != 0.0)) {
        let eq173_e2181: f64 = (p.p253 * s.v[276]);
        let eq173_e2181_d_n0: f64 = (p.p253 * s.dn[276][0]);
        let eq173_e2181_d_n1: f64 = (p.p253 * s.dn[276][1]);
        let eq173_e2181_d_n2: f64 = (p.p253 * s.dn[276][2]);
        let eq173_e2181_d_n3: f64 = (p.p253 * s.dn[276][3]);
        let eq173_e2181_d_n4: f64 = (p.p253 * s.dn[276][4]);
        let eq173_e2181_d_n5: f64 = (p.p253 * s.dn[276][5]);
        let eq173_e2181_d_n6: f64 = (p.p253 * s.dn[276][6]);
        let eq173_e2181_d_n7: f64 = (p.p253 * s.dn[276][7]);
        let eq173_e2181_d_n8: f64 = (p.p253 * s.dn[276][8]);
        let eq173_e2181_d_n9: f64 = (p.p253 * s.dn[276][9]);
        let eq173_e2181_d_n10: f64 = (p.p253 * s.dn[276][10]);
        let eq173_e2181_d_n11: f64 = (p.p253 * s.dn[276][11]);
        let eq173_e2181_d_n12: f64 = (p.p253 * s.dn[276][12]);
        let eq173_e2181_d_n13: f64 = (p.p253 * s.dn[276][13]);
        let eq173_e2181_d_n14: f64 = (p.p253 * s.dn[276][14]);
        let eq173_e2181_d_n15: f64 = (p.p253 * s.dn[276][15]);
        let eq173_e2181_d_n16: f64 = (p.p253 * s.dn[276][16]);
        let eq173_e2181_d_n17: f64 = (p.p253 * s.dn[276][17]);
        let eq173_e2181_d_n18: f64 = (p.p253 * s.dn[276][18]);
        let eq173_e2181_d_n19: f64 = (p.p253 * s.dn[276][19]);
        let eq173_e2181_d_n20: f64 = (p.p253 * s.dn[276][20]);
        let eq173_e2181_d_n21: f64 = (p.p253 * s.dn[276][21]);
        let eq173_e2181_d_n22: f64 = (p.p253 * s.dn[276][22]);
        let eq173_e2182: f64 = self.eval_ddt(72, eq173_e2181);
        let eq173_e2182_d_n0: f64 = self.ddt_jacobian(eq173_e2181_d_n0);
        let eq173_e2182_d_n1: f64 = self.ddt_jacobian(eq173_e2181_d_n1);
        let eq173_e2182_d_n2: f64 = self.ddt_jacobian(eq173_e2181_d_n2);
        let eq173_e2182_d_n3: f64 = self.ddt_jacobian(eq173_e2181_d_n3);
        let eq173_e2182_d_n4: f64 = self.ddt_jacobian(eq173_e2181_d_n4);
        let eq173_e2182_d_n5: f64 = self.ddt_jacobian(eq173_e2181_d_n5);
        let eq173_e2182_d_n6: f64 = self.ddt_jacobian(eq173_e2181_d_n6);
        let eq173_e2182_d_n7: f64 = self.ddt_jacobian(eq173_e2181_d_n7);
        let eq173_e2182_d_n8: f64 = self.ddt_jacobian(eq173_e2181_d_n8);
        let eq173_e2182_d_n9: f64 = self.ddt_jacobian(eq173_e2181_d_n9);
        let eq173_e2182_d_n10: f64 = self.ddt_jacobian(eq173_e2181_d_n10);
        let eq173_e2182_d_n11: f64 = self.ddt_jacobian(eq173_e2181_d_n11);
        let eq173_e2182_d_n12: f64 = self.ddt_jacobian(eq173_e2181_d_n12);
        let eq173_e2182_d_n13: f64 = self.ddt_jacobian(eq173_e2181_d_n13);
        let eq173_e2182_d_n14: f64 = self.ddt_jacobian(eq173_e2181_d_n14);
        let eq173_e2182_d_n15: f64 = self.ddt_jacobian(eq173_e2181_d_n15);
        let eq173_e2182_d_n16: f64 = self.ddt_jacobian(eq173_e2181_d_n16);
        let eq173_e2182_d_n17: f64 = self.ddt_jacobian(eq173_e2181_d_n17);
        let eq173_e2182_d_n18: f64 = self.ddt_jacobian(eq173_e2181_d_n18);
        let eq173_e2182_d_n19: f64 = self.ddt_jacobian(eq173_e2181_d_n19);
        let eq173_e2182_d_n20: f64 = self.ddt_jacobian(eq173_e2181_d_n20);
        let eq173_e2182_d_n21: f64 = self.ddt_jacobian(eq173_e2181_d_n21);
        let eq173_e2182_d_n22: f64 = self.ddt_jacobian(eq173_e2181_d_n22);
        let eq173_e2183: f64 = (p.p7 * eq173_e2182);
        let eq173_e2183_d_n0: f64 = (p.p7 * eq173_e2182_d_n0);
        let eq173_e2183_d_n1: f64 = (p.p7 * eq173_e2182_d_n1);
        let eq173_e2183_d_n2: f64 = (p.p7 * eq173_e2182_d_n2);
        let eq173_e2183_d_n3: f64 = (p.p7 * eq173_e2182_d_n3);
        let eq173_e2183_d_n4: f64 = (p.p7 * eq173_e2182_d_n4);
        let eq173_e2183_d_n5: f64 = (p.p7 * eq173_e2182_d_n5);
        let eq173_e2183_d_n6: f64 = (p.p7 * eq173_e2182_d_n6);
        let eq173_e2183_d_n7: f64 = (p.p7 * eq173_e2182_d_n7);
        let eq173_e2183_d_n8: f64 = (p.p7 * eq173_e2182_d_n8);
        let eq173_e2183_d_n9: f64 = (p.p7 * eq173_e2182_d_n9);
        let eq173_e2183_d_n10: f64 = (p.p7 * eq173_e2182_d_n10);
        let eq173_e2183_d_n11: f64 = (p.p7 * eq173_e2182_d_n11);
        let eq173_e2183_d_n12: f64 = (p.p7 * eq173_e2182_d_n12);
        let eq173_e2183_d_n13: f64 = (p.p7 * eq173_e2182_d_n13);
        let eq173_e2183_d_n14: f64 = (p.p7 * eq173_e2182_d_n14);
        let eq173_e2183_d_n15: f64 = (p.p7 * eq173_e2182_d_n15);
        let eq173_e2183_d_n16: f64 = (p.p7 * eq173_e2182_d_n16);
        let eq173_e2183_d_n17: f64 = (p.p7 * eq173_e2182_d_n17);
        let eq173_e2183_d_n18: f64 = (p.p7 * eq173_e2182_d_n18);
        let eq173_e2183_d_n19: f64 = (p.p7 * eq173_e2182_d_n19);
        let eq173_e2183_d_n20: f64 = (p.p7 * eq173_e2182_d_n20);
        let eq173_e2183_d_n21: f64 = (p.p7 * eq173_e2182_d_n21);
        let eq173_e2183_d_n22: f64 = (p.p7 * eq173_e2182_d_n22);
        (eq173_e2183, eq173_e2183_d_n0, eq173_e2183_d_n1, eq173_e2183_d_n2, eq173_e2183_d_n3, eq173_e2183_d_n4, eq173_e2183_d_n5, eq173_e2183_d_n6, eq173_e2183_d_n7, eq173_e2183_d_n8, eq173_e2183_d_n9, eq173_e2183_d_n10, eq173_e2183_d_n11, eq173_e2183_d_n12, eq173_e2183_d_n13, eq173_e2183_d_n14, eq173_e2183_d_n15, eq173_e2183_d_n16, eq173_e2183_d_n17, eq173_e2183_d_n18, eq173_e2183_d_n19, eq173_e2183_d_n20, eq173_e2183_d_n21, eq173_e2183_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq173_value: f64 = eq173_e2185;
        let eq173_node_derivatives: [f64; 23] = [eq173_e2185_d_n0, eq173_e2185_d_n1, eq173_e2185_d_n2, eq173_e2185_d_n3, eq173_e2185_d_n4, eq173_e2185_d_n5, eq173_e2185_d_n6, eq173_e2185_d_n7, eq173_e2185_d_n8, eq173_e2185_d_n9, eq173_e2185_d_n10, eq173_e2185_d_n11, eq173_e2185_d_n12, eq173_e2185_d_n13, eq173_e2185_d_n14, eq173_e2185_d_n15, eq173_e2185_d_n16, eq173_e2185_d_n17, eq173_e2185_d_n18, eq173_e2185_d_n19, eq173_e2185_d_n20, eq173_e2185_d_n21, eq173_e2185_d_n22];
        let eq173_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[16]),
            self.multiplicity * (eq173_value),
            &nodes,
            &eq173_node_derivatives,
            &branches,
            &eq173_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_174_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq174_e2195, eq174_e2195_d_n0, eq174_e2195_d_n1, eq174_e2195_d_n2, eq174_e2195_d_n3, eq174_e2195_d_n4, eq174_e2195_d_n5, eq174_e2195_d_n6, eq174_e2195_d_n7, eq174_e2195_d_n8, eq174_e2195_d_n9, eq174_e2195_d_n10, eq174_e2195_d_n11, eq174_e2195_d_n12, eq174_e2195_d_n13, eq174_e2195_d_n14, eq174_e2195_d_n15, eq174_e2195_d_n16, eq174_e2195_d_n17, eq174_e2195_d_n18, eq174_e2195_d_n19, eq174_e2195_d_n20, eq174_e2195_d_n21, eq174_e2195_d_n22,) = {
    if ((!(s.v[590] != 0.0)) && (s.v[593] != 0.0)) {
        let eq174_e2192: f64 = self.eval_ddt(73, s.v[277]);
        let eq174_e2192_d_n0: f64 = self.ddt_jacobian(s.dn[277][0]);
        let eq174_e2192_d_n1: f64 = self.ddt_jacobian(s.dn[277][1]);
        let eq174_e2192_d_n2: f64 = self.ddt_jacobian(s.dn[277][2]);
        let eq174_e2192_d_n3: f64 = self.ddt_jacobian(s.dn[277][3]);
        let eq174_e2192_d_n4: f64 = self.ddt_jacobian(s.dn[277][4]);
        let eq174_e2192_d_n5: f64 = self.ddt_jacobian(s.dn[277][5]);
        let eq174_e2192_d_n6: f64 = self.ddt_jacobian(s.dn[277][6]);
        let eq174_e2192_d_n7: f64 = self.ddt_jacobian(s.dn[277][7]);
        let eq174_e2192_d_n8: f64 = self.ddt_jacobian(s.dn[277][8]);
        let eq174_e2192_d_n9: f64 = self.ddt_jacobian(s.dn[277][9]);
        let eq174_e2192_d_n10: f64 = self.ddt_jacobian(s.dn[277][10]);
        let eq174_e2192_d_n11: f64 = self.ddt_jacobian(s.dn[277][11]);
        let eq174_e2192_d_n12: f64 = self.ddt_jacobian(s.dn[277][12]);
        let eq174_e2192_d_n13: f64 = self.ddt_jacobian(s.dn[277][13]);
        let eq174_e2192_d_n14: f64 = self.ddt_jacobian(s.dn[277][14]);
        let eq174_e2192_d_n15: f64 = self.ddt_jacobian(s.dn[277][15]);
        let eq174_e2192_d_n16: f64 = self.ddt_jacobian(s.dn[277][16]);
        let eq174_e2192_d_n17: f64 = self.ddt_jacobian(s.dn[277][17]);
        let eq174_e2192_d_n18: f64 = self.ddt_jacobian(s.dn[277][18]);
        let eq174_e2192_d_n19: f64 = self.ddt_jacobian(s.dn[277][19]);
        let eq174_e2192_d_n20: f64 = self.ddt_jacobian(s.dn[277][20]);
        let eq174_e2192_d_n21: f64 = self.ddt_jacobian(s.dn[277][21]);
        let eq174_e2192_d_n22: f64 = self.ddt_jacobian(s.dn[277][22]);
        let eq174_e2193: f64 = (p.p7 * eq174_e2192);
        let eq174_e2193_d_n0: f64 = (p.p7 * eq174_e2192_d_n0);
        let eq174_e2193_d_n1: f64 = (p.p7 * eq174_e2192_d_n1);
        let eq174_e2193_d_n2: f64 = (p.p7 * eq174_e2192_d_n2);
        let eq174_e2193_d_n3: f64 = (p.p7 * eq174_e2192_d_n3);
        let eq174_e2193_d_n4: f64 = (p.p7 * eq174_e2192_d_n4);
        let eq174_e2193_d_n5: f64 = (p.p7 * eq174_e2192_d_n5);
        let eq174_e2193_d_n6: f64 = (p.p7 * eq174_e2192_d_n6);
        let eq174_e2193_d_n7: f64 = (p.p7 * eq174_e2192_d_n7);
        let eq174_e2193_d_n8: f64 = (p.p7 * eq174_e2192_d_n8);
        let eq174_e2193_d_n9: f64 = (p.p7 * eq174_e2192_d_n9);
        let eq174_e2193_d_n10: f64 = (p.p7 * eq174_e2192_d_n10);
        let eq174_e2193_d_n11: f64 = (p.p7 * eq174_e2192_d_n11);
        let eq174_e2193_d_n12: f64 = (p.p7 * eq174_e2192_d_n12);
        let eq174_e2193_d_n13: f64 = (p.p7 * eq174_e2192_d_n13);
        let eq174_e2193_d_n14: f64 = (p.p7 * eq174_e2192_d_n14);
        let eq174_e2193_d_n15: f64 = (p.p7 * eq174_e2192_d_n15);
        let eq174_e2193_d_n16: f64 = (p.p7 * eq174_e2192_d_n16);
        let eq174_e2193_d_n17: f64 = (p.p7 * eq174_e2192_d_n17);
        let eq174_e2193_d_n18: f64 = (p.p7 * eq174_e2192_d_n18);
        let eq174_e2193_d_n19: f64 = (p.p7 * eq174_e2192_d_n19);
        let eq174_e2193_d_n20: f64 = (p.p7 * eq174_e2192_d_n20);
        let eq174_e2193_d_n21: f64 = (p.p7 * eq174_e2192_d_n21);
        let eq174_e2193_d_n22: f64 = (p.p7 * eq174_e2192_d_n22);
        (eq174_e2193, eq174_e2193_d_n0, eq174_e2193_d_n1, eq174_e2193_d_n2, eq174_e2193_d_n3, eq174_e2193_d_n4, eq174_e2193_d_n5, eq174_e2193_d_n6, eq174_e2193_d_n7, eq174_e2193_d_n8, eq174_e2193_d_n9, eq174_e2193_d_n10, eq174_e2193_d_n11, eq174_e2193_d_n12, eq174_e2193_d_n13, eq174_e2193_d_n14, eq174_e2193_d_n15, eq174_e2193_d_n16, eq174_e2193_d_n17, eq174_e2193_d_n18, eq174_e2193_d_n19, eq174_e2193_d_n20, eq174_e2193_d_n21, eq174_e2193_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq174_value: f64 = eq174_e2195;
        let eq174_node_derivatives: [f64; 23] = [eq174_e2195_d_n0, eq174_e2195_d_n1, eq174_e2195_d_n2, eq174_e2195_d_n3, eq174_e2195_d_n4, eq174_e2195_d_n5, eq174_e2195_d_n6, eq174_e2195_d_n7, eq174_e2195_d_n8, eq174_e2195_d_n9, eq174_e2195_d_n10, eq174_e2195_d_n11, eq174_e2195_d_n12, eq174_e2195_d_n13, eq174_e2195_d_n14, eq174_e2195_d_n15, eq174_e2195_d_n16, eq174_e2195_d_n17, eq174_e2195_d_n18, eq174_e2195_d_n19, eq174_e2195_d_n20, eq174_e2195_d_n21, eq174_e2195_d_n22];
        let eq174_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[7]),
            self.multiplicity * (eq174_value),
            &nodes,
            &eq174_node_derivatives,
            &branches,
            &eq174_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_175_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq175_e2207, eq175_e2207_d_n0, eq175_e2207_d_n1, eq175_e2207_d_n2, eq175_e2207_d_n3, eq175_e2207_d_n4, eq175_e2207_d_n5, eq175_e2207_d_n6, eq175_e2207_d_n7, eq175_e2207_d_n8, eq175_e2207_d_n9, eq175_e2207_d_n10, eq175_e2207_d_n11, eq175_e2207_d_n12, eq175_e2207_d_n13, eq175_e2207_d_n14, eq175_e2207_d_n15, eq175_e2207_d_n16, eq175_e2207_d_n17, eq175_e2207_d_n18, eq175_e2207_d_n19, eq175_e2207_d_n20, eq175_e2207_d_n21, eq175_e2207_d_n22,) = {
    if (((!(s.v[590] != 0.0)) && (s.v[593] != 0.0)) && (s.v[594] != 0.0)) {
        let eq175_e2204: f64 = self.eval_ddt(74, s.v[276]);
        let eq175_e2204_d_n0: f64 = self.ddt_jacobian(s.dn[276][0]);
        let eq175_e2204_d_n1: f64 = self.ddt_jacobian(s.dn[276][1]);
        let eq175_e2204_d_n2: f64 = self.ddt_jacobian(s.dn[276][2]);
        let eq175_e2204_d_n3: f64 = self.ddt_jacobian(s.dn[276][3]);
        let eq175_e2204_d_n4: f64 = self.ddt_jacobian(s.dn[276][4]);
        let eq175_e2204_d_n5: f64 = self.ddt_jacobian(s.dn[276][5]);
        let eq175_e2204_d_n6: f64 = self.ddt_jacobian(s.dn[276][6]);
        let eq175_e2204_d_n7: f64 = self.ddt_jacobian(s.dn[276][7]);
        let eq175_e2204_d_n8: f64 = self.ddt_jacobian(s.dn[276][8]);
        let eq175_e2204_d_n9: f64 = self.ddt_jacobian(s.dn[276][9]);
        let eq175_e2204_d_n10: f64 = self.ddt_jacobian(s.dn[276][10]);
        let eq175_e2204_d_n11: f64 = self.ddt_jacobian(s.dn[276][11]);
        let eq175_e2204_d_n12: f64 = self.ddt_jacobian(s.dn[276][12]);
        let eq175_e2204_d_n13: f64 = self.ddt_jacobian(s.dn[276][13]);
        let eq175_e2204_d_n14: f64 = self.ddt_jacobian(s.dn[276][14]);
        let eq175_e2204_d_n15: f64 = self.ddt_jacobian(s.dn[276][15]);
        let eq175_e2204_d_n16: f64 = self.ddt_jacobian(s.dn[276][16]);
        let eq175_e2204_d_n17: f64 = self.ddt_jacobian(s.dn[276][17]);
        let eq175_e2204_d_n18: f64 = self.ddt_jacobian(s.dn[276][18]);
        let eq175_e2204_d_n19: f64 = self.ddt_jacobian(s.dn[276][19]);
        let eq175_e2204_d_n20: f64 = self.ddt_jacobian(s.dn[276][20]);
        let eq175_e2204_d_n21: f64 = self.ddt_jacobian(s.dn[276][21]);
        let eq175_e2204_d_n22: f64 = self.ddt_jacobian(s.dn[276][22]);
        let eq175_e2205: f64 = (p.p7 * eq175_e2204);
        let eq175_e2205_d_n0: f64 = (p.p7 * eq175_e2204_d_n0);
        let eq175_e2205_d_n1: f64 = (p.p7 * eq175_e2204_d_n1);
        let eq175_e2205_d_n2: f64 = (p.p7 * eq175_e2204_d_n2);
        let eq175_e2205_d_n3: f64 = (p.p7 * eq175_e2204_d_n3);
        let eq175_e2205_d_n4: f64 = (p.p7 * eq175_e2204_d_n4);
        let eq175_e2205_d_n5: f64 = (p.p7 * eq175_e2204_d_n5);
        let eq175_e2205_d_n6: f64 = (p.p7 * eq175_e2204_d_n6);
        let eq175_e2205_d_n7: f64 = (p.p7 * eq175_e2204_d_n7);
        let eq175_e2205_d_n8: f64 = (p.p7 * eq175_e2204_d_n8);
        let eq175_e2205_d_n9: f64 = (p.p7 * eq175_e2204_d_n9);
        let eq175_e2205_d_n10: f64 = (p.p7 * eq175_e2204_d_n10);
        let eq175_e2205_d_n11: f64 = (p.p7 * eq175_e2204_d_n11);
        let eq175_e2205_d_n12: f64 = (p.p7 * eq175_e2204_d_n12);
        let eq175_e2205_d_n13: f64 = (p.p7 * eq175_e2204_d_n13);
        let eq175_e2205_d_n14: f64 = (p.p7 * eq175_e2204_d_n14);
        let eq175_e2205_d_n15: f64 = (p.p7 * eq175_e2204_d_n15);
        let eq175_e2205_d_n16: f64 = (p.p7 * eq175_e2204_d_n16);
        let eq175_e2205_d_n17: f64 = (p.p7 * eq175_e2204_d_n17);
        let eq175_e2205_d_n18: f64 = (p.p7 * eq175_e2204_d_n18);
        let eq175_e2205_d_n19: f64 = (p.p7 * eq175_e2204_d_n19);
        let eq175_e2205_d_n20: f64 = (p.p7 * eq175_e2204_d_n20);
        let eq175_e2205_d_n21: f64 = (p.p7 * eq175_e2204_d_n21);
        let eq175_e2205_d_n22: f64 = (p.p7 * eq175_e2204_d_n22);
        (eq175_e2205, eq175_e2205_d_n0, eq175_e2205_d_n1, eq175_e2205_d_n2, eq175_e2205_d_n3, eq175_e2205_d_n4, eq175_e2205_d_n5, eq175_e2205_d_n6, eq175_e2205_d_n7, eq175_e2205_d_n8, eq175_e2205_d_n9, eq175_e2205_d_n10, eq175_e2205_d_n11, eq175_e2205_d_n12, eq175_e2205_d_n13, eq175_e2205_d_n14, eq175_e2205_d_n15, eq175_e2205_d_n16, eq175_e2205_d_n17, eq175_e2205_d_n18, eq175_e2205_d_n19, eq175_e2205_d_n20, eq175_e2205_d_n21, eq175_e2205_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq175_value: f64 = eq175_e2207;
        let eq175_node_derivatives: [f64; 23] = [eq175_e2207_d_n0, eq175_e2207_d_n1, eq175_e2207_d_n2, eq175_e2207_d_n3, eq175_e2207_d_n4, eq175_e2207_d_n5, eq175_e2207_d_n6, eq175_e2207_d_n7, eq175_e2207_d_n8, eq175_e2207_d_n9, eq175_e2207_d_n10, eq175_e2207_d_n11, eq175_e2207_d_n12, eq175_e2207_d_n13, eq175_e2207_d_n14, eq175_e2207_d_n15, eq175_e2207_d_n16, eq175_e2207_d_n17, eq175_e2207_d_n18, eq175_e2207_d_n19, eq175_e2207_d_n20, eq175_e2207_d_n21, eq175_e2207_d_n22];
        let eq175_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq175_value),
            &nodes,
            &eq175_node_derivatives,
            &branches,
            &eq175_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_176_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq176_e2221, eq176_e2221_d_n0, eq176_e2221_d_n1, eq176_e2221_d_n2, eq176_e2221_d_n3, eq176_e2221_d_n4, eq176_e2221_d_n5, eq176_e2221_d_n6, eq176_e2221_d_n7, eq176_e2221_d_n8, eq176_e2221_d_n9, eq176_e2221_d_n10, eq176_e2221_d_n11, eq176_e2221_d_n12, eq176_e2221_d_n13, eq176_e2221_d_n14, eq176_e2221_d_n15, eq176_e2221_d_n16, eq176_e2221_d_n17, eq176_e2221_d_n18, eq176_e2221_d_n19, eq176_e2221_d_n20, eq176_e2221_d_n21, eq176_e2221_d_n22,) = {
    if (((!(s.v[590] != 0.0)) && (s.v[593] != 0.0)) && (s.v[594] != 0.0)) {
        let eq176_e2216: f64 = self.eval_ddt(75, s.v[276]);
        let eq176_e2216_d_n0: f64 = self.ddt_jacobian(s.dn[276][0]);
        let eq176_e2216_d_n1: f64 = self.ddt_jacobian(s.dn[276][1]);
        let eq176_e2216_d_n2: f64 = self.ddt_jacobian(s.dn[276][2]);
        let eq176_e2216_d_n3: f64 = self.ddt_jacobian(s.dn[276][3]);
        let eq176_e2216_d_n4: f64 = self.ddt_jacobian(s.dn[276][4]);
        let eq176_e2216_d_n5: f64 = self.ddt_jacobian(s.dn[276][5]);
        let eq176_e2216_d_n6: f64 = self.ddt_jacobian(s.dn[276][6]);
        let eq176_e2216_d_n7: f64 = self.ddt_jacobian(s.dn[276][7]);
        let eq176_e2216_d_n8: f64 = self.ddt_jacobian(s.dn[276][8]);
        let eq176_e2216_d_n9: f64 = self.ddt_jacobian(s.dn[276][9]);
        let eq176_e2216_d_n10: f64 = self.ddt_jacobian(s.dn[276][10]);
        let eq176_e2216_d_n11: f64 = self.ddt_jacobian(s.dn[276][11]);
        let eq176_e2216_d_n12: f64 = self.ddt_jacobian(s.dn[276][12]);
        let eq176_e2216_d_n13: f64 = self.ddt_jacobian(s.dn[276][13]);
        let eq176_e2216_d_n14: f64 = self.ddt_jacobian(s.dn[276][14]);
        let eq176_e2216_d_n15: f64 = self.ddt_jacobian(s.dn[276][15]);
        let eq176_e2216_d_n16: f64 = self.ddt_jacobian(s.dn[276][16]);
        let eq176_e2216_d_n17: f64 = self.ddt_jacobian(s.dn[276][17]);
        let eq176_e2216_d_n18: f64 = self.ddt_jacobian(s.dn[276][18]);
        let eq176_e2216_d_n19: f64 = self.ddt_jacobian(s.dn[276][19]);
        let eq176_e2216_d_n20: f64 = self.ddt_jacobian(s.dn[276][20]);
        let eq176_e2216_d_n21: f64 = self.ddt_jacobian(s.dn[276][21]);
        let eq176_e2216_d_n22: f64 = self.ddt_jacobian(s.dn[276][22]);
        let eq176_e2217: f64 = (p.p7 * eq176_e2216);
        let eq176_e2217_d_n0: f64 = (p.p7 * eq176_e2216_d_n0);
        let eq176_e2217_d_n1: f64 = (p.p7 * eq176_e2216_d_n1);
        let eq176_e2217_d_n2: f64 = (p.p7 * eq176_e2216_d_n2);
        let eq176_e2217_d_n3: f64 = (p.p7 * eq176_e2216_d_n3);
        let eq176_e2217_d_n4: f64 = (p.p7 * eq176_e2216_d_n4);
        let eq176_e2217_d_n5: f64 = (p.p7 * eq176_e2216_d_n5);
        let eq176_e2217_d_n6: f64 = (p.p7 * eq176_e2216_d_n6);
        let eq176_e2217_d_n7: f64 = (p.p7 * eq176_e2216_d_n7);
        let eq176_e2217_d_n8: f64 = (p.p7 * eq176_e2216_d_n8);
        let eq176_e2217_d_n9: f64 = (p.p7 * eq176_e2216_d_n9);
        let eq176_e2217_d_n10: f64 = (p.p7 * eq176_e2216_d_n10);
        let eq176_e2217_d_n11: f64 = (p.p7 * eq176_e2216_d_n11);
        let eq176_e2217_d_n12: f64 = (p.p7 * eq176_e2216_d_n12);
        let eq176_e2217_d_n13: f64 = (p.p7 * eq176_e2216_d_n13);
        let eq176_e2217_d_n14: f64 = (p.p7 * eq176_e2216_d_n14);
        let eq176_e2217_d_n15: f64 = (p.p7 * eq176_e2216_d_n15);
        let eq176_e2217_d_n16: f64 = (p.p7 * eq176_e2216_d_n16);
        let eq176_e2217_d_n17: f64 = (p.p7 * eq176_e2216_d_n17);
        let eq176_e2217_d_n18: f64 = (p.p7 * eq176_e2216_d_n18);
        let eq176_e2217_d_n19: f64 = (p.p7 * eq176_e2216_d_n19);
        let eq176_e2217_d_n20: f64 = (p.p7 * eq176_e2216_d_n20);
        let eq176_e2217_d_n21: f64 = (p.p7 * eq176_e2216_d_n21);
        let eq176_e2217_d_n22: f64 = (p.p7 * eq176_e2216_d_n22);
        let eq176_e2219: f64 = (eq176_e2217 * p.p248);
        let eq176_e2219_d_n0: f64 = (eq176_e2217_d_n0 * p.p248);
        let eq176_e2219_d_n1: f64 = (eq176_e2217_d_n1 * p.p248);
        let eq176_e2219_d_n2: f64 = (eq176_e2217_d_n2 * p.p248);
        let eq176_e2219_d_n3: f64 = (eq176_e2217_d_n3 * p.p248);
        let eq176_e2219_d_n4: f64 = (eq176_e2217_d_n4 * p.p248);
        let eq176_e2219_d_n5: f64 = (eq176_e2217_d_n5 * p.p248);
        let eq176_e2219_d_n6: f64 = (eq176_e2217_d_n6 * p.p248);
        let eq176_e2219_d_n7: f64 = (eq176_e2217_d_n7 * p.p248);
        let eq176_e2219_d_n8: f64 = (eq176_e2217_d_n8 * p.p248);
        let eq176_e2219_d_n9: f64 = (eq176_e2217_d_n9 * p.p248);
        let eq176_e2219_d_n10: f64 = (eq176_e2217_d_n10 * p.p248);
        let eq176_e2219_d_n11: f64 = (eq176_e2217_d_n11 * p.p248);
        let eq176_e2219_d_n12: f64 = (eq176_e2217_d_n12 * p.p248);
        let eq176_e2219_d_n13: f64 = (eq176_e2217_d_n13 * p.p248);
        let eq176_e2219_d_n14: f64 = (eq176_e2217_d_n14 * p.p248);
        let eq176_e2219_d_n15: f64 = (eq176_e2217_d_n15 * p.p248);
        let eq176_e2219_d_n16: f64 = (eq176_e2217_d_n16 * p.p248);
        let eq176_e2219_d_n17: f64 = (eq176_e2217_d_n17 * p.p248);
        let eq176_e2219_d_n18: f64 = (eq176_e2217_d_n18 * p.p248);
        let eq176_e2219_d_n19: f64 = (eq176_e2217_d_n19 * p.p248);
        let eq176_e2219_d_n20: f64 = (eq176_e2217_d_n20 * p.p248);
        let eq176_e2219_d_n21: f64 = (eq176_e2217_d_n21 * p.p248);
        let eq176_e2219_d_n22: f64 = (eq176_e2217_d_n22 * p.p248);
        (eq176_e2219, eq176_e2219_d_n0, eq176_e2219_d_n1, eq176_e2219_d_n2, eq176_e2219_d_n3, eq176_e2219_d_n4, eq176_e2219_d_n5, eq176_e2219_d_n6, eq176_e2219_d_n7, eq176_e2219_d_n8, eq176_e2219_d_n9, eq176_e2219_d_n10, eq176_e2219_d_n11, eq176_e2219_d_n12, eq176_e2219_d_n13, eq176_e2219_d_n14, eq176_e2219_d_n15, eq176_e2219_d_n16, eq176_e2219_d_n17, eq176_e2219_d_n18, eq176_e2219_d_n19, eq176_e2219_d_n20, eq176_e2219_d_n21, eq176_e2219_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq176_value: f64 = eq176_e2221;
        let eq176_node_derivatives: [f64; 23] = [eq176_e2221_d_n0, eq176_e2221_d_n1, eq176_e2221_d_n2, eq176_e2221_d_n3, eq176_e2221_d_n4, eq176_e2221_d_n5, eq176_e2221_d_n6, eq176_e2221_d_n7, eq176_e2221_d_n8, eq176_e2221_d_n9, eq176_e2221_d_n10, eq176_e2221_d_n11, eq176_e2221_d_n12, eq176_e2221_d_n13, eq176_e2221_d_n14, eq176_e2221_d_n15, eq176_e2221_d_n16, eq176_e2221_d_n17, eq176_e2221_d_n18, eq176_e2221_d_n19, eq176_e2221_d_n20, eq176_e2221_d_n21, eq176_e2221_d_n22];
        let eq176_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            self.multiplicity * (eq176_value),
            &nodes,
            &eq176_node_derivatives,
            &branches,
            &eq176_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_177_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq177_e2234, eq177_e2234_d_n0, eq177_e2234_d_n1, eq177_e2234_d_n2, eq177_e2234_d_n3, eq177_e2234_d_n4, eq177_e2234_d_n5, eq177_e2234_d_n6, eq177_e2234_d_n7, eq177_e2234_d_n8, eq177_e2234_d_n9, eq177_e2234_d_n10, eq177_e2234_d_n11, eq177_e2234_d_n12, eq177_e2234_d_n13, eq177_e2234_d_n14, eq177_e2234_d_n15, eq177_e2234_d_n16, eq177_e2234_d_n17, eq177_e2234_d_n18, eq177_e2234_d_n19, eq177_e2234_d_n20, eq177_e2234_d_n21, eq177_e2234_d_n22,) = {
    if (((!(s.v[590] != 0.0)) && (s.v[593] != 0.0)) && (!(s.v[594] != 0.0))) {
        let eq177_e2231: f64 = self.eval_ddt(76, s.v[276]);
        let eq177_e2231_d_n0: f64 = self.ddt_jacobian(s.dn[276][0]);
        let eq177_e2231_d_n1: f64 = self.ddt_jacobian(s.dn[276][1]);
        let eq177_e2231_d_n2: f64 = self.ddt_jacobian(s.dn[276][2]);
        let eq177_e2231_d_n3: f64 = self.ddt_jacobian(s.dn[276][3]);
        let eq177_e2231_d_n4: f64 = self.ddt_jacobian(s.dn[276][4]);
        let eq177_e2231_d_n5: f64 = self.ddt_jacobian(s.dn[276][5]);
        let eq177_e2231_d_n6: f64 = self.ddt_jacobian(s.dn[276][6]);
        let eq177_e2231_d_n7: f64 = self.ddt_jacobian(s.dn[276][7]);
        let eq177_e2231_d_n8: f64 = self.ddt_jacobian(s.dn[276][8]);
        let eq177_e2231_d_n9: f64 = self.ddt_jacobian(s.dn[276][9]);
        let eq177_e2231_d_n10: f64 = self.ddt_jacobian(s.dn[276][10]);
        let eq177_e2231_d_n11: f64 = self.ddt_jacobian(s.dn[276][11]);
        let eq177_e2231_d_n12: f64 = self.ddt_jacobian(s.dn[276][12]);
        let eq177_e2231_d_n13: f64 = self.ddt_jacobian(s.dn[276][13]);
        let eq177_e2231_d_n14: f64 = self.ddt_jacobian(s.dn[276][14]);
        let eq177_e2231_d_n15: f64 = self.ddt_jacobian(s.dn[276][15]);
        let eq177_e2231_d_n16: f64 = self.ddt_jacobian(s.dn[276][16]);
        let eq177_e2231_d_n17: f64 = self.ddt_jacobian(s.dn[276][17]);
        let eq177_e2231_d_n18: f64 = self.ddt_jacobian(s.dn[276][18]);
        let eq177_e2231_d_n19: f64 = self.ddt_jacobian(s.dn[276][19]);
        let eq177_e2231_d_n20: f64 = self.ddt_jacobian(s.dn[276][20]);
        let eq177_e2231_d_n21: f64 = self.ddt_jacobian(s.dn[276][21]);
        let eq177_e2231_d_n22: f64 = self.ddt_jacobian(s.dn[276][22]);
        let eq177_e2232: f64 = (p.p7 * eq177_e2231);
        let eq177_e2232_d_n0: f64 = (p.p7 * eq177_e2231_d_n0);
        let eq177_e2232_d_n1: f64 = (p.p7 * eq177_e2231_d_n1);
        let eq177_e2232_d_n2: f64 = (p.p7 * eq177_e2231_d_n2);
        let eq177_e2232_d_n3: f64 = (p.p7 * eq177_e2231_d_n3);
        let eq177_e2232_d_n4: f64 = (p.p7 * eq177_e2231_d_n4);
        let eq177_e2232_d_n5: f64 = (p.p7 * eq177_e2231_d_n5);
        let eq177_e2232_d_n6: f64 = (p.p7 * eq177_e2231_d_n6);
        let eq177_e2232_d_n7: f64 = (p.p7 * eq177_e2231_d_n7);
        let eq177_e2232_d_n8: f64 = (p.p7 * eq177_e2231_d_n8);
        let eq177_e2232_d_n9: f64 = (p.p7 * eq177_e2231_d_n9);
        let eq177_e2232_d_n10: f64 = (p.p7 * eq177_e2231_d_n10);
        let eq177_e2232_d_n11: f64 = (p.p7 * eq177_e2231_d_n11);
        let eq177_e2232_d_n12: f64 = (p.p7 * eq177_e2231_d_n12);
        let eq177_e2232_d_n13: f64 = (p.p7 * eq177_e2231_d_n13);
        let eq177_e2232_d_n14: f64 = (p.p7 * eq177_e2231_d_n14);
        let eq177_e2232_d_n15: f64 = (p.p7 * eq177_e2231_d_n15);
        let eq177_e2232_d_n16: f64 = (p.p7 * eq177_e2231_d_n16);
        let eq177_e2232_d_n17: f64 = (p.p7 * eq177_e2231_d_n17);
        let eq177_e2232_d_n18: f64 = (p.p7 * eq177_e2231_d_n18);
        let eq177_e2232_d_n19: f64 = (p.p7 * eq177_e2231_d_n19);
        let eq177_e2232_d_n20: f64 = (p.p7 * eq177_e2231_d_n20);
        let eq177_e2232_d_n21: f64 = (p.p7 * eq177_e2231_d_n21);
        let eq177_e2232_d_n22: f64 = (p.p7 * eq177_e2231_d_n22);
        (eq177_e2232, eq177_e2232_d_n0, eq177_e2232_d_n1, eq177_e2232_d_n2, eq177_e2232_d_n3, eq177_e2232_d_n4, eq177_e2232_d_n5, eq177_e2232_d_n6, eq177_e2232_d_n7, eq177_e2232_d_n8, eq177_e2232_d_n9, eq177_e2232_d_n10, eq177_e2232_d_n11, eq177_e2232_d_n12, eq177_e2232_d_n13, eq177_e2232_d_n14, eq177_e2232_d_n15, eq177_e2232_d_n16, eq177_e2232_d_n17, eq177_e2232_d_n18, eq177_e2232_d_n19, eq177_e2232_d_n20, eq177_e2232_d_n21, eq177_e2232_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq177_value: f64 = eq177_e2234;
        let eq177_node_derivatives: [f64; 23] = [eq177_e2234_d_n0, eq177_e2234_d_n1, eq177_e2234_d_n2, eq177_e2234_d_n3, eq177_e2234_d_n4, eq177_e2234_d_n5, eq177_e2234_d_n6, eq177_e2234_d_n7, eq177_e2234_d_n8, eq177_e2234_d_n9, eq177_e2234_d_n10, eq177_e2234_d_n11, eq177_e2234_d_n12, eq177_e2234_d_n13, eq177_e2234_d_n14, eq177_e2234_d_n15, eq177_e2234_d_n16, eq177_e2234_d_n17, eq177_e2234_d_n18, eq177_e2234_d_n19, eq177_e2234_d_n20, eq177_e2234_d_n21, eq177_e2234_d_n22];
        let eq177_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            self.multiplicity * (eq177_value),
            &nodes,
            &eq177_node_derivatives,
            &branches,
            &eq177_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_178_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq178_e2249, eq178_e2249_d_n0, eq178_e2249_d_n1, eq178_e2249_d_n2, eq178_e2249_d_n3, eq178_e2249_d_n4, eq178_e2249_d_n5, eq178_e2249_d_n6, eq178_e2249_d_n7, eq178_e2249_d_n8, eq178_e2249_d_n9, eq178_e2249_d_n10, eq178_e2249_d_n11, eq178_e2249_d_n12, eq178_e2249_d_n13, eq178_e2249_d_n14, eq178_e2249_d_n15, eq178_e2249_d_n16, eq178_e2249_d_n17, eq178_e2249_d_n18, eq178_e2249_d_n19, eq178_e2249_d_n20, eq178_e2249_d_n21, eq178_e2249_d_n22,) = {
    if (((!(s.v[590] != 0.0)) && (s.v[593] != 0.0)) && (!(s.v[594] != 0.0))) {
        let eq178_e2244: f64 = self.eval_ddt(77, s.v[276]);
        let eq178_e2244_d_n0: f64 = self.ddt_jacobian(s.dn[276][0]);
        let eq178_e2244_d_n1: f64 = self.ddt_jacobian(s.dn[276][1]);
        let eq178_e2244_d_n2: f64 = self.ddt_jacobian(s.dn[276][2]);
        let eq178_e2244_d_n3: f64 = self.ddt_jacobian(s.dn[276][3]);
        let eq178_e2244_d_n4: f64 = self.ddt_jacobian(s.dn[276][4]);
        let eq178_e2244_d_n5: f64 = self.ddt_jacobian(s.dn[276][5]);
        let eq178_e2244_d_n6: f64 = self.ddt_jacobian(s.dn[276][6]);
        let eq178_e2244_d_n7: f64 = self.ddt_jacobian(s.dn[276][7]);
        let eq178_e2244_d_n8: f64 = self.ddt_jacobian(s.dn[276][8]);
        let eq178_e2244_d_n9: f64 = self.ddt_jacobian(s.dn[276][9]);
        let eq178_e2244_d_n10: f64 = self.ddt_jacobian(s.dn[276][10]);
        let eq178_e2244_d_n11: f64 = self.ddt_jacobian(s.dn[276][11]);
        let eq178_e2244_d_n12: f64 = self.ddt_jacobian(s.dn[276][12]);
        let eq178_e2244_d_n13: f64 = self.ddt_jacobian(s.dn[276][13]);
        let eq178_e2244_d_n14: f64 = self.ddt_jacobian(s.dn[276][14]);
        let eq178_e2244_d_n15: f64 = self.ddt_jacobian(s.dn[276][15]);
        let eq178_e2244_d_n16: f64 = self.ddt_jacobian(s.dn[276][16]);
        let eq178_e2244_d_n17: f64 = self.ddt_jacobian(s.dn[276][17]);
        let eq178_e2244_d_n18: f64 = self.ddt_jacobian(s.dn[276][18]);
        let eq178_e2244_d_n19: f64 = self.ddt_jacobian(s.dn[276][19]);
        let eq178_e2244_d_n20: f64 = self.ddt_jacobian(s.dn[276][20]);
        let eq178_e2244_d_n21: f64 = self.ddt_jacobian(s.dn[276][21]);
        let eq178_e2244_d_n22: f64 = self.ddt_jacobian(s.dn[276][22]);
        let eq178_e2245: f64 = (p.p7 * eq178_e2244);
        let eq178_e2245_d_n0: f64 = (p.p7 * eq178_e2244_d_n0);
        let eq178_e2245_d_n1: f64 = (p.p7 * eq178_e2244_d_n1);
        let eq178_e2245_d_n2: f64 = (p.p7 * eq178_e2244_d_n2);
        let eq178_e2245_d_n3: f64 = (p.p7 * eq178_e2244_d_n3);
        let eq178_e2245_d_n4: f64 = (p.p7 * eq178_e2244_d_n4);
        let eq178_e2245_d_n5: f64 = (p.p7 * eq178_e2244_d_n5);
        let eq178_e2245_d_n6: f64 = (p.p7 * eq178_e2244_d_n6);
        let eq178_e2245_d_n7: f64 = (p.p7 * eq178_e2244_d_n7);
        let eq178_e2245_d_n8: f64 = (p.p7 * eq178_e2244_d_n8);
        let eq178_e2245_d_n9: f64 = (p.p7 * eq178_e2244_d_n9);
        let eq178_e2245_d_n10: f64 = (p.p7 * eq178_e2244_d_n10);
        let eq178_e2245_d_n11: f64 = (p.p7 * eq178_e2244_d_n11);
        let eq178_e2245_d_n12: f64 = (p.p7 * eq178_e2244_d_n12);
        let eq178_e2245_d_n13: f64 = (p.p7 * eq178_e2244_d_n13);
        let eq178_e2245_d_n14: f64 = (p.p7 * eq178_e2244_d_n14);
        let eq178_e2245_d_n15: f64 = (p.p7 * eq178_e2244_d_n15);
        let eq178_e2245_d_n16: f64 = (p.p7 * eq178_e2244_d_n16);
        let eq178_e2245_d_n17: f64 = (p.p7 * eq178_e2244_d_n17);
        let eq178_e2245_d_n18: f64 = (p.p7 * eq178_e2244_d_n18);
        let eq178_e2245_d_n19: f64 = (p.p7 * eq178_e2244_d_n19);
        let eq178_e2245_d_n20: f64 = (p.p7 * eq178_e2244_d_n20);
        let eq178_e2245_d_n21: f64 = (p.p7 * eq178_e2244_d_n21);
        let eq178_e2245_d_n22: f64 = (p.p7 * eq178_e2244_d_n22);
        let eq178_e2247: f64 = (eq178_e2245 * p.p248);
        let eq178_e2247_d_n0: f64 = (eq178_e2245_d_n0 * p.p248);
        let eq178_e2247_d_n1: f64 = (eq178_e2245_d_n1 * p.p248);
        let eq178_e2247_d_n2: f64 = (eq178_e2245_d_n2 * p.p248);
        let eq178_e2247_d_n3: f64 = (eq178_e2245_d_n3 * p.p248);
        let eq178_e2247_d_n4: f64 = (eq178_e2245_d_n4 * p.p248);
        let eq178_e2247_d_n5: f64 = (eq178_e2245_d_n5 * p.p248);
        let eq178_e2247_d_n6: f64 = (eq178_e2245_d_n6 * p.p248);
        let eq178_e2247_d_n7: f64 = (eq178_e2245_d_n7 * p.p248);
        let eq178_e2247_d_n8: f64 = (eq178_e2245_d_n8 * p.p248);
        let eq178_e2247_d_n9: f64 = (eq178_e2245_d_n9 * p.p248);
        let eq178_e2247_d_n10: f64 = (eq178_e2245_d_n10 * p.p248);
        let eq178_e2247_d_n11: f64 = (eq178_e2245_d_n11 * p.p248);
        let eq178_e2247_d_n12: f64 = (eq178_e2245_d_n12 * p.p248);
        let eq178_e2247_d_n13: f64 = (eq178_e2245_d_n13 * p.p248);
        let eq178_e2247_d_n14: f64 = (eq178_e2245_d_n14 * p.p248);
        let eq178_e2247_d_n15: f64 = (eq178_e2245_d_n15 * p.p248);
        let eq178_e2247_d_n16: f64 = (eq178_e2245_d_n16 * p.p248);
        let eq178_e2247_d_n17: f64 = (eq178_e2245_d_n17 * p.p248);
        let eq178_e2247_d_n18: f64 = (eq178_e2245_d_n18 * p.p248);
        let eq178_e2247_d_n19: f64 = (eq178_e2245_d_n19 * p.p248);
        let eq178_e2247_d_n20: f64 = (eq178_e2245_d_n20 * p.p248);
        let eq178_e2247_d_n21: f64 = (eq178_e2245_d_n21 * p.p248);
        let eq178_e2247_d_n22: f64 = (eq178_e2245_d_n22 * p.p248);
        (eq178_e2247, eq178_e2247_d_n0, eq178_e2247_d_n1, eq178_e2247_d_n2, eq178_e2247_d_n3, eq178_e2247_d_n4, eq178_e2247_d_n5, eq178_e2247_d_n6, eq178_e2247_d_n7, eq178_e2247_d_n8, eq178_e2247_d_n9, eq178_e2247_d_n10, eq178_e2247_d_n11, eq178_e2247_d_n12, eq178_e2247_d_n13, eq178_e2247_d_n14, eq178_e2247_d_n15, eq178_e2247_d_n16, eq178_e2247_d_n17, eq178_e2247_d_n18, eq178_e2247_d_n19, eq178_e2247_d_n20, eq178_e2247_d_n21, eq178_e2247_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq178_value: f64 = eq178_e2249;
        let eq178_node_derivatives: [f64; 23] = [eq178_e2249_d_n0, eq178_e2249_d_n1, eq178_e2249_d_n2, eq178_e2249_d_n3, eq178_e2249_d_n4, eq178_e2249_d_n5, eq178_e2249_d_n6, eq178_e2249_d_n7, eq178_e2249_d_n8, eq178_e2249_d_n9, eq178_e2249_d_n10, eq178_e2249_d_n11, eq178_e2249_d_n12, eq178_e2249_d_n13, eq178_e2249_d_n14, eq178_e2249_d_n15, eq178_e2249_d_n16, eq178_e2249_d_n17, eq178_e2249_d_n18, eq178_e2249_d_n19, eq178_e2249_d_n20, eq178_e2249_d_n21, eq178_e2249_d_n22];
        let eq178_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq178_value),
            &nodes,
            &eq178_node_derivatives,
            &branches,
            &eq178_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_179_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq179_e2261, eq179_e2261_d_n0, eq179_e2261_d_n1, eq179_e2261_d_n2, eq179_e2261_d_n3, eq179_e2261_d_n4, eq179_e2261_d_n5, eq179_e2261_d_n6, eq179_e2261_d_n7, eq179_e2261_d_n8, eq179_e2261_d_n9, eq179_e2261_d_n10, eq179_e2261_d_n11, eq179_e2261_d_n12, eq179_e2261_d_n13, eq179_e2261_d_n14, eq179_e2261_d_n15, eq179_e2261_d_n16, eq179_e2261_d_n17, eq179_e2261_d_n18, eq179_e2261_d_n19, eq179_e2261_d_n20, eq179_e2261_d_n21, eq179_e2261_d_n22,) = {
    if ((!(s.v[590] != 0.0)) && (s.v[593] != 0.0)) {
        let eq179_e2257: f64 = (p.p253 * s.v[276]);
        let eq179_e2257_d_n0: f64 = (p.p253 * s.dn[276][0]);
        let eq179_e2257_d_n1: f64 = (p.p253 * s.dn[276][1]);
        let eq179_e2257_d_n2: f64 = (p.p253 * s.dn[276][2]);
        let eq179_e2257_d_n3: f64 = (p.p253 * s.dn[276][3]);
        let eq179_e2257_d_n4: f64 = (p.p253 * s.dn[276][4]);
        let eq179_e2257_d_n5: f64 = (p.p253 * s.dn[276][5]);
        let eq179_e2257_d_n6: f64 = (p.p253 * s.dn[276][6]);
        let eq179_e2257_d_n7: f64 = (p.p253 * s.dn[276][7]);
        let eq179_e2257_d_n8: f64 = (p.p253 * s.dn[276][8]);
        let eq179_e2257_d_n9: f64 = (p.p253 * s.dn[276][9]);
        let eq179_e2257_d_n10: f64 = (p.p253 * s.dn[276][10]);
        let eq179_e2257_d_n11: f64 = (p.p253 * s.dn[276][11]);
        let eq179_e2257_d_n12: f64 = (p.p253 * s.dn[276][12]);
        let eq179_e2257_d_n13: f64 = (p.p253 * s.dn[276][13]);
        let eq179_e2257_d_n14: f64 = (p.p253 * s.dn[276][14]);
        let eq179_e2257_d_n15: f64 = (p.p253 * s.dn[276][15]);
        let eq179_e2257_d_n16: f64 = (p.p253 * s.dn[276][16]);
        let eq179_e2257_d_n17: f64 = (p.p253 * s.dn[276][17]);
        let eq179_e2257_d_n18: f64 = (p.p253 * s.dn[276][18]);
        let eq179_e2257_d_n19: f64 = (p.p253 * s.dn[276][19]);
        let eq179_e2257_d_n20: f64 = (p.p253 * s.dn[276][20]);
        let eq179_e2257_d_n21: f64 = (p.p253 * s.dn[276][21]);
        let eq179_e2257_d_n22: f64 = (p.p253 * s.dn[276][22]);
        let eq179_e2258: f64 = self.eval_ddt(78, eq179_e2257);
        let eq179_e2258_d_n0: f64 = self.ddt_jacobian(eq179_e2257_d_n0);
        let eq179_e2258_d_n1: f64 = self.ddt_jacobian(eq179_e2257_d_n1);
        let eq179_e2258_d_n2: f64 = self.ddt_jacobian(eq179_e2257_d_n2);
        let eq179_e2258_d_n3: f64 = self.ddt_jacobian(eq179_e2257_d_n3);
        let eq179_e2258_d_n4: f64 = self.ddt_jacobian(eq179_e2257_d_n4);
        let eq179_e2258_d_n5: f64 = self.ddt_jacobian(eq179_e2257_d_n5);
        let eq179_e2258_d_n6: f64 = self.ddt_jacobian(eq179_e2257_d_n6);
        let eq179_e2258_d_n7: f64 = self.ddt_jacobian(eq179_e2257_d_n7);
        let eq179_e2258_d_n8: f64 = self.ddt_jacobian(eq179_e2257_d_n8);
        let eq179_e2258_d_n9: f64 = self.ddt_jacobian(eq179_e2257_d_n9);
        let eq179_e2258_d_n10: f64 = self.ddt_jacobian(eq179_e2257_d_n10);
        let eq179_e2258_d_n11: f64 = self.ddt_jacobian(eq179_e2257_d_n11);
        let eq179_e2258_d_n12: f64 = self.ddt_jacobian(eq179_e2257_d_n12);
        let eq179_e2258_d_n13: f64 = self.ddt_jacobian(eq179_e2257_d_n13);
        let eq179_e2258_d_n14: f64 = self.ddt_jacobian(eq179_e2257_d_n14);
        let eq179_e2258_d_n15: f64 = self.ddt_jacobian(eq179_e2257_d_n15);
        let eq179_e2258_d_n16: f64 = self.ddt_jacobian(eq179_e2257_d_n16);
        let eq179_e2258_d_n17: f64 = self.ddt_jacobian(eq179_e2257_d_n17);
        let eq179_e2258_d_n18: f64 = self.ddt_jacobian(eq179_e2257_d_n18);
        let eq179_e2258_d_n19: f64 = self.ddt_jacobian(eq179_e2257_d_n19);
        let eq179_e2258_d_n20: f64 = self.ddt_jacobian(eq179_e2257_d_n20);
        let eq179_e2258_d_n21: f64 = self.ddt_jacobian(eq179_e2257_d_n21);
        let eq179_e2258_d_n22: f64 = self.ddt_jacobian(eq179_e2257_d_n22);
        let eq179_e2259: f64 = (p.p7 * eq179_e2258);
        let eq179_e2259_d_n0: f64 = (p.p7 * eq179_e2258_d_n0);
        let eq179_e2259_d_n1: f64 = (p.p7 * eq179_e2258_d_n1);
        let eq179_e2259_d_n2: f64 = (p.p7 * eq179_e2258_d_n2);
        let eq179_e2259_d_n3: f64 = (p.p7 * eq179_e2258_d_n3);
        let eq179_e2259_d_n4: f64 = (p.p7 * eq179_e2258_d_n4);
        let eq179_e2259_d_n5: f64 = (p.p7 * eq179_e2258_d_n5);
        let eq179_e2259_d_n6: f64 = (p.p7 * eq179_e2258_d_n6);
        let eq179_e2259_d_n7: f64 = (p.p7 * eq179_e2258_d_n7);
        let eq179_e2259_d_n8: f64 = (p.p7 * eq179_e2258_d_n8);
        let eq179_e2259_d_n9: f64 = (p.p7 * eq179_e2258_d_n9);
        let eq179_e2259_d_n10: f64 = (p.p7 * eq179_e2258_d_n10);
        let eq179_e2259_d_n11: f64 = (p.p7 * eq179_e2258_d_n11);
        let eq179_e2259_d_n12: f64 = (p.p7 * eq179_e2258_d_n12);
        let eq179_e2259_d_n13: f64 = (p.p7 * eq179_e2258_d_n13);
        let eq179_e2259_d_n14: f64 = (p.p7 * eq179_e2258_d_n14);
        let eq179_e2259_d_n15: f64 = (p.p7 * eq179_e2258_d_n15);
        let eq179_e2259_d_n16: f64 = (p.p7 * eq179_e2258_d_n16);
        let eq179_e2259_d_n17: f64 = (p.p7 * eq179_e2258_d_n17);
        let eq179_e2259_d_n18: f64 = (p.p7 * eq179_e2258_d_n18);
        let eq179_e2259_d_n19: f64 = (p.p7 * eq179_e2258_d_n19);
        let eq179_e2259_d_n20: f64 = (p.p7 * eq179_e2258_d_n20);
        let eq179_e2259_d_n21: f64 = (p.p7 * eq179_e2258_d_n21);
        let eq179_e2259_d_n22: f64 = (p.p7 * eq179_e2258_d_n22);
        (eq179_e2259, eq179_e2259_d_n0, eq179_e2259_d_n1, eq179_e2259_d_n2, eq179_e2259_d_n3, eq179_e2259_d_n4, eq179_e2259_d_n5, eq179_e2259_d_n6, eq179_e2259_d_n7, eq179_e2259_d_n8, eq179_e2259_d_n9, eq179_e2259_d_n10, eq179_e2259_d_n11, eq179_e2259_d_n12, eq179_e2259_d_n13, eq179_e2259_d_n14, eq179_e2259_d_n15, eq179_e2259_d_n16, eq179_e2259_d_n17, eq179_e2259_d_n18, eq179_e2259_d_n19, eq179_e2259_d_n20, eq179_e2259_d_n21, eq179_e2259_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq179_value: f64 = eq179_e2261;
        let eq179_node_derivatives: [f64; 23] = [eq179_e2261_d_n0, eq179_e2261_d_n1, eq179_e2261_d_n2, eq179_e2261_d_n3, eq179_e2261_d_n4, eq179_e2261_d_n5, eq179_e2261_d_n6, eq179_e2261_d_n7, eq179_e2261_d_n8, eq179_e2261_d_n9, eq179_e2261_d_n10, eq179_e2261_d_n11, eq179_e2261_d_n12, eq179_e2261_d_n13, eq179_e2261_d_n14, eq179_e2261_d_n15, eq179_e2261_d_n16, eq179_e2261_d_n17, eq179_e2261_d_n18, eq179_e2261_d_n19, eq179_e2261_d_n20, eq179_e2261_d_n21, eq179_e2261_d_n22];
        let eq179_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            self.multiplicity * (eq179_value),
            &nodes,
            &eq179_node_derivatives,
            &branches,
            &eq179_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_180_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq180_e2270, eq180_e2270_d_n0, eq180_e2270_d_n1, eq180_e2270_d_n2, eq180_e2270_d_n3, eq180_e2270_d_n4, eq180_e2270_d_n5, eq180_e2270_d_n6, eq180_e2270_d_n7, eq180_e2270_d_n8, eq180_e2270_d_n9, eq180_e2270_d_n10, eq180_e2270_d_n11, eq180_e2270_d_n12, eq180_e2270_d_n13, eq180_e2270_d_n14, eq180_e2270_d_n15, eq180_e2270_d_n16, eq180_e2270_d_n17, eq180_e2270_d_n18, eq180_e2270_d_n19, eq180_e2270_d_n20, eq180_e2270_d_n21, eq180_e2270_d_n22,) = {
    if ((s.v[595] != 0.0) && (s.v[596] != 0.0)) {
        let eq180_e2267: f64 = self.eval_ddt(79, s.v[289]);
        let eq180_e2267_d_n0: f64 = self.ddt_jacobian(s.dn[289][0]);
        let eq180_e2267_d_n1: f64 = self.ddt_jacobian(s.dn[289][1]);
        let eq180_e2267_d_n2: f64 = self.ddt_jacobian(s.dn[289][2]);
        let eq180_e2267_d_n3: f64 = self.ddt_jacobian(s.dn[289][3]);
        let eq180_e2267_d_n4: f64 = self.ddt_jacobian(s.dn[289][4]);
        let eq180_e2267_d_n5: f64 = self.ddt_jacobian(s.dn[289][5]);
        let eq180_e2267_d_n6: f64 = self.ddt_jacobian(s.dn[289][6]);
        let eq180_e2267_d_n7: f64 = self.ddt_jacobian(s.dn[289][7]);
        let eq180_e2267_d_n8: f64 = self.ddt_jacobian(s.dn[289][8]);
        let eq180_e2267_d_n9: f64 = self.ddt_jacobian(s.dn[289][9]);
        let eq180_e2267_d_n10: f64 = self.ddt_jacobian(s.dn[289][10]);
        let eq180_e2267_d_n11: f64 = self.ddt_jacobian(s.dn[289][11]);
        let eq180_e2267_d_n12: f64 = self.ddt_jacobian(s.dn[289][12]);
        let eq180_e2267_d_n13: f64 = self.ddt_jacobian(s.dn[289][13]);
        let eq180_e2267_d_n14: f64 = self.ddt_jacobian(s.dn[289][14]);
        let eq180_e2267_d_n15: f64 = self.ddt_jacobian(s.dn[289][15]);
        let eq180_e2267_d_n16: f64 = self.ddt_jacobian(s.dn[289][16]);
        let eq180_e2267_d_n17: f64 = self.ddt_jacobian(s.dn[289][17]);
        let eq180_e2267_d_n18: f64 = self.ddt_jacobian(s.dn[289][18]);
        let eq180_e2267_d_n19: f64 = self.ddt_jacobian(s.dn[289][19]);
        let eq180_e2267_d_n20: f64 = self.ddt_jacobian(s.dn[289][20]);
        let eq180_e2267_d_n21: f64 = self.ddt_jacobian(s.dn[289][21]);
        let eq180_e2267_d_n22: f64 = self.ddt_jacobian(s.dn[289][22]);
        let eq180_e2268: f64 = (p.p7 * eq180_e2267);
        let eq180_e2268_d_n0: f64 = (p.p7 * eq180_e2267_d_n0);
        let eq180_e2268_d_n1: f64 = (p.p7 * eq180_e2267_d_n1);
        let eq180_e2268_d_n2: f64 = (p.p7 * eq180_e2267_d_n2);
        let eq180_e2268_d_n3: f64 = (p.p7 * eq180_e2267_d_n3);
        let eq180_e2268_d_n4: f64 = (p.p7 * eq180_e2267_d_n4);
        let eq180_e2268_d_n5: f64 = (p.p7 * eq180_e2267_d_n5);
        let eq180_e2268_d_n6: f64 = (p.p7 * eq180_e2267_d_n6);
        let eq180_e2268_d_n7: f64 = (p.p7 * eq180_e2267_d_n7);
        let eq180_e2268_d_n8: f64 = (p.p7 * eq180_e2267_d_n8);
        let eq180_e2268_d_n9: f64 = (p.p7 * eq180_e2267_d_n9);
        let eq180_e2268_d_n10: f64 = (p.p7 * eq180_e2267_d_n10);
        let eq180_e2268_d_n11: f64 = (p.p7 * eq180_e2267_d_n11);
        let eq180_e2268_d_n12: f64 = (p.p7 * eq180_e2267_d_n12);
        let eq180_e2268_d_n13: f64 = (p.p7 * eq180_e2267_d_n13);
        let eq180_e2268_d_n14: f64 = (p.p7 * eq180_e2267_d_n14);
        let eq180_e2268_d_n15: f64 = (p.p7 * eq180_e2267_d_n15);
        let eq180_e2268_d_n16: f64 = (p.p7 * eq180_e2267_d_n16);
        let eq180_e2268_d_n17: f64 = (p.p7 * eq180_e2267_d_n17);
        let eq180_e2268_d_n18: f64 = (p.p7 * eq180_e2267_d_n18);
        let eq180_e2268_d_n19: f64 = (p.p7 * eq180_e2267_d_n19);
        let eq180_e2268_d_n20: f64 = (p.p7 * eq180_e2267_d_n20);
        let eq180_e2268_d_n21: f64 = (p.p7 * eq180_e2267_d_n21);
        let eq180_e2268_d_n22: f64 = (p.p7 * eq180_e2267_d_n22);
        (eq180_e2268, eq180_e2268_d_n0, eq180_e2268_d_n1, eq180_e2268_d_n2, eq180_e2268_d_n3, eq180_e2268_d_n4, eq180_e2268_d_n5, eq180_e2268_d_n6, eq180_e2268_d_n7, eq180_e2268_d_n8, eq180_e2268_d_n9, eq180_e2268_d_n10, eq180_e2268_d_n11, eq180_e2268_d_n12, eq180_e2268_d_n13, eq180_e2268_d_n14, eq180_e2268_d_n15, eq180_e2268_d_n16, eq180_e2268_d_n17, eq180_e2268_d_n18, eq180_e2268_d_n19, eq180_e2268_d_n20, eq180_e2268_d_n21, eq180_e2268_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq180_value: f64 = eq180_e2270;
        let eq180_node_derivatives: [f64; 23] = [eq180_e2270_d_n0, eq180_e2270_d_n1, eq180_e2270_d_n2, eq180_e2270_d_n3, eq180_e2270_d_n4, eq180_e2270_d_n5, eq180_e2270_d_n6, eq180_e2270_d_n7, eq180_e2270_d_n8, eq180_e2270_d_n9, eq180_e2270_d_n10, eq180_e2270_d_n11, eq180_e2270_d_n12, eq180_e2270_d_n13, eq180_e2270_d_n14, eq180_e2270_d_n15, eq180_e2270_d_n16, eq180_e2270_d_n17, eq180_e2270_d_n18, eq180_e2270_d_n19, eq180_e2270_d_n20, eq180_e2270_d_n21, eq180_e2270_d_n22];
        let eq180_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[21]),
            self.multiplicity * (eq180_value),
            &nodes,
            &eq180_node_derivatives,
            &branches,
            &eq180_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_181_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq181_e2281, eq181_e2281_d_n0, eq181_e2281_d_n1, eq181_e2281_d_n2, eq181_e2281_d_n3, eq181_e2281_d_n4, eq181_e2281_d_n5, eq181_e2281_d_n6, eq181_e2281_d_n7, eq181_e2281_d_n8, eq181_e2281_d_n9, eq181_e2281_d_n10, eq181_e2281_d_n11, eq181_e2281_d_n12, eq181_e2281_d_n13, eq181_e2281_d_n14, eq181_e2281_d_n15, eq181_e2281_d_n16, eq181_e2281_d_n17, eq181_e2281_d_n18, eq181_e2281_d_n19, eq181_e2281_d_n20, eq181_e2281_d_n21, eq181_e2281_d_n22,) = {
    if (((s.v[595] != 0.0) && (s.v[596] != 0.0)) && (s.v[597] != 0.0)) {
        let eq181_e2278: f64 = self.eval_ddt(80, s.v[288]);
        let eq181_e2278_d_n0: f64 = self.ddt_jacobian(s.dn[288][0]);
        let eq181_e2278_d_n1: f64 = self.ddt_jacobian(s.dn[288][1]);
        let eq181_e2278_d_n2: f64 = self.ddt_jacobian(s.dn[288][2]);
        let eq181_e2278_d_n3: f64 = self.ddt_jacobian(s.dn[288][3]);
        let eq181_e2278_d_n4: f64 = self.ddt_jacobian(s.dn[288][4]);
        let eq181_e2278_d_n5: f64 = self.ddt_jacobian(s.dn[288][5]);
        let eq181_e2278_d_n6: f64 = self.ddt_jacobian(s.dn[288][6]);
        let eq181_e2278_d_n7: f64 = self.ddt_jacobian(s.dn[288][7]);
        let eq181_e2278_d_n8: f64 = self.ddt_jacobian(s.dn[288][8]);
        let eq181_e2278_d_n9: f64 = self.ddt_jacobian(s.dn[288][9]);
        let eq181_e2278_d_n10: f64 = self.ddt_jacobian(s.dn[288][10]);
        let eq181_e2278_d_n11: f64 = self.ddt_jacobian(s.dn[288][11]);
        let eq181_e2278_d_n12: f64 = self.ddt_jacobian(s.dn[288][12]);
        let eq181_e2278_d_n13: f64 = self.ddt_jacobian(s.dn[288][13]);
        let eq181_e2278_d_n14: f64 = self.ddt_jacobian(s.dn[288][14]);
        let eq181_e2278_d_n15: f64 = self.ddt_jacobian(s.dn[288][15]);
        let eq181_e2278_d_n16: f64 = self.ddt_jacobian(s.dn[288][16]);
        let eq181_e2278_d_n17: f64 = self.ddt_jacobian(s.dn[288][17]);
        let eq181_e2278_d_n18: f64 = self.ddt_jacobian(s.dn[288][18]);
        let eq181_e2278_d_n19: f64 = self.ddt_jacobian(s.dn[288][19]);
        let eq181_e2278_d_n20: f64 = self.ddt_jacobian(s.dn[288][20]);
        let eq181_e2278_d_n21: f64 = self.ddt_jacobian(s.dn[288][21]);
        let eq181_e2278_d_n22: f64 = self.ddt_jacobian(s.dn[288][22]);
        let eq181_e2279: f64 = (p.p7 * eq181_e2278);
        let eq181_e2279_d_n0: f64 = (p.p7 * eq181_e2278_d_n0);
        let eq181_e2279_d_n1: f64 = (p.p7 * eq181_e2278_d_n1);
        let eq181_e2279_d_n2: f64 = (p.p7 * eq181_e2278_d_n2);
        let eq181_e2279_d_n3: f64 = (p.p7 * eq181_e2278_d_n3);
        let eq181_e2279_d_n4: f64 = (p.p7 * eq181_e2278_d_n4);
        let eq181_e2279_d_n5: f64 = (p.p7 * eq181_e2278_d_n5);
        let eq181_e2279_d_n6: f64 = (p.p7 * eq181_e2278_d_n6);
        let eq181_e2279_d_n7: f64 = (p.p7 * eq181_e2278_d_n7);
        let eq181_e2279_d_n8: f64 = (p.p7 * eq181_e2278_d_n8);
        let eq181_e2279_d_n9: f64 = (p.p7 * eq181_e2278_d_n9);
        let eq181_e2279_d_n10: f64 = (p.p7 * eq181_e2278_d_n10);
        let eq181_e2279_d_n11: f64 = (p.p7 * eq181_e2278_d_n11);
        let eq181_e2279_d_n12: f64 = (p.p7 * eq181_e2278_d_n12);
        let eq181_e2279_d_n13: f64 = (p.p7 * eq181_e2278_d_n13);
        let eq181_e2279_d_n14: f64 = (p.p7 * eq181_e2278_d_n14);
        let eq181_e2279_d_n15: f64 = (p.p7 * eq181_e2278_d_n15);
        let eq181_e2279_d_n16: f64 = (p.p7 * eq181_e2278_d_n16);
        let eq181_e2279_d_n17: f64 = (p.p7 * eq181_e2278_d_n17);
        let eq181_e2279_d_n18: f64 = (p.p7 * eq181_e2278_d_n18);
        let eq181_e2279_d_n19: f64 = (p.p7 * eq181_e2278_d_n19);
        let eq181_e2279_d_n20: f64 = (p.p7 * eq181_e2278_d_n20);
        let eq181_e2279_d_n21: f64 = (p.p7 * eq181_e2278_d_n21);
        let eq181_e2279_d_n22: f64 = (p.p7 * eq181_e2278_d_n22);
        (eq181_e2279, eq181_e2279_d_n0, eq181_e2279_d_n1, eq181_e2279_d_n2, eq181_e2279_d_n3, eq181_e2279_d_n4, eq181_e2279_d_n5, eq181_e2279_d_n6, eq181_e2279_d_n7, eq181_e2279_d_n8, eq181_e2279_d_n9, eq181_e2279_d_n10, eq181_e2279_d_n11, eq181_e2279_d_n12, eq181_e2279_d_n13, eq181_e2279_d_n14, eq181_e2279_d_n15, eq181_e2279_d_n16, eq181_e2279_d_n17, eq181_e2279_d_n18, eq181_e2279_d_n19, eq181_e2279_d_n20, eq181_e2279_d_n21, eq181_e2279_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq181_value: f64 = eq181_e2281;
        let eq181_node_derivatives: [f64; 23] = [eq181_e2281_d_n0, eq181_e2281_d_n1, eq181_e2281_d_n2, eq181_e2281_d_n3, eq181_e2281_d_n4, eq181_e2281_d_n5, eq181_e2281_d_n6, eq181_e2281_d_n7, eq181_e2281_d_n8, eq181_e2281_d_n9, eq181_e2281_d_n10, eq181_e2281_d_n11, eq181_e2281_d_n12, eq181_e2281_d_n13, eq181_e2281_d_n14, eq181_e2281_d_n15, eq181_e2281_d_n16, eq181_e2281_d_n17, eq181_e2281_d_n18, eq181_e2281_d_n19, eq181_e2281_d_n20, eq181_e2281_d_n21, eq181_e2281_d_n22];
        let eq181_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[21]),
            self.multiplicity * (eq181_value),
            &nodes,
            &eq181_node_derivatives,
            &branches,
            &eq181_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_182_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq182_e2294, eq182_e2294_d_n0, eq182_e2294_d_n1, eq182_e2294_d_n2, eq182_e2294_d_n3, eq182_e2294_d_n4, eq182_e2294_d_n5, eq182_e2294_d_n6, eq182_e2294_d_n7, eq182_e2294_d_n8, eq182_e2294_d_n9, eq182_e2294_d_n10, eq182_e2294_d_n11, eq182_e2294_d_n12, eq182_e2294_d_n13, eq182_e2294_d_n14, eq182_e2294_d_n15, eq182_e2294_d_n16, eq182_e2294_d_n17, eq182_e2294_d_n18, eq182_e2294_d_n19, eq182_e2294_d_n20, eq182_e2294_d_n21, eq182_e2294_d_n22,) = {
    if (((s.v[595] != 0.0) && (s.v[596] != 0.0)) && (s.v[597] != 0.0)) {
        let eq182_e2289: f64 = self.eval_ddt(81, s.v[288]);
        let eq182_e2289_d_n0: f64 = self.ddt_jacobian(s.dn[288][0]);
        let eq182_e2289_d_n1: f64 = self.ddt_jacobian(s.dn[288][1]);
        let eq182_e2289_d_n2: f64 = self.ddt_jacobian(s.dn[288][2]);
        let eq182_e2289_d_n3: f64 = self.ddt_jacobian(s.dn[288][3]);
        let eq182_e2289_d_n4: f64 = self.ddt_jacobian(s.dn[288][4]);
        let eq182_e2289_d_n5: f64 = self.ddt_jacobian(s.dn[288][5]);
        let eq182_e2289_d_n6: f64 = self.ddt_jacobian(s.dn[288][6]);
        let eq182_e2289_d_n7: f64 = self.ddt_jacobian(s.dn[288][7]);
        let eq182_e2289_d_n8: f64 = self.ddt_jacobian(s.dn[288][8]);
        let eq182_e2289_d_n9: f64 = self.ddt_jacobian(s.dn[288][9]);
        let eq182_e2289_d_n10: f64 = self.ddt_jacobian(s.dn[288][10]);
        let eq182_e2289_d_n11: f64 = self.ddt_jacobian(s.dn[288][11]);
        let eq182_e2289_d_n12: f64 = self.ddt_jacobian(s.dn[288][12]);
        let eq182_e2289_d_n13: f64 = self.ddt_jacobian(s.dn[288][13]);
        let eq182_e2289_d_n14: f64 = self.ddt_jacobian(s.dn[288][14]);
        let eq182_e2289_d_n15: f64 = self.ddt_jacobian(s.dn[288][15]);
        let eq182_e2289_d_n16: f64 = self.ddt_jacobian(s.dn[288][16]);
        let eq182_e2289_d_n17: f64 = self.ddt_jacobian(s.dn[288][17]);
        let eq182_e2289_d_n18: f64 = self.ddt_jacobian(s.dn[288][18]);
        let eq182_e2289_d_n19: f64 = self.ddt_jacobian(s.dn[288][19]);
        let eq182_e2289_d_n20: f64 = self.ddt_jacobian(s.dn[288][20]);
        let eq182_e2289_d_n21: f64 = self.ddt_jacobian(s.dn[288][21]);
        let eq182_e2289_d_n22: f64 = self.ddt_jacobian(s.dn[288][22]);
        let eq182_e2290: f64 = (p.p7 * eq182_e2289);
        let eq182_e2290_d_n0: f64 = (p.p7 * eq182_e2289_d_n0);
        let eq182_e2290_d_n1: f64 = (p.p7 * eq182_e2289_d_n1);
        let eq182_e2290_d_n2: f64 = (p.p7 * eq182_e2289_d_n2);
        let eq182_e2290_d_n3: f64 = (p.p7 * eq182_e2289_d_n3);
        let eq182_e2290_d_n4: f64 = (p.p7 * eq182_e2289_d_n4);
        let eq182_e2290_d_n5: f64 = (p.p7 * eq182_e2289_d_n5);
        let eq182_e2290_d_n6: f64 = (p.p7 * eq182_e2289_d_n6);
        let eq182_e2290_d_n7: f64 = (p.p7 * eq182_e2289_d_n7);
        let eq182_e2290_d_n8: f64 = (p.p7 * eq182_e2289_d_n8);
        let eq182_e2290_d_n9: f64 = (p.p7 * eq182_e2289_d_n9);
        let eq182_e2290_d_n10: f64 = (p.p7 * eq182_e2289_d_n10);
        let eq182_e2290_d_n11: f64 = (p.p7 * eq182_e2289_d_n11);
        let eq182_e2290_d_n12: f64 = (p.p7 * eq182_e2289_d_n12);
        let eq182_e2290_d_n13: f64 = (p.p7 * eq182_e2289_d_n13);
        let eq182_e2290_d_n14: f64 = (p.p7 * eq182_e2289_d_n14);
        let eq182_e2290_d_n15: f64 = (p.p7 * eq182_e2289_d_n15);
        let eq182_e2290_d_n16: f64 = (p.p7 * eq182_e2289_d_n16);
        let eq182_e2290_d_n17: f64 = (p.p7 * eq182_e2289_d_n17);
        let eq182_e2290_d_n18: f64 = (p.p7 * eq182_e2289_d_n18);
        let eq182_e2290_d_n19: f64 = (p.p7 * eq182_e2289_d_n19);
        let eq182_e2290_d_n20: f64 = (p.p7 * eq182_e2289_d_n20);
        let eq182_e2290_d_n21: f64 = (p.p7 * eq182_e2289_d_n21);
        let eq182_e2290_d_n22: f64 = (p.p7 * eq182_e2289_d_n22);
        let eq182_e2292: f64 = (eq182_e2290 * p.p248);
        let eq182_e2292_d_n0: f64 = (eq182_e2290_d_n0 * p.p248);
        let eq182_e2292_d_n1: f64 = (eq182_e2290_d_n1 * p.p248);
        let eq182_e2292_d_n2: f64 = (eq182_e2290_d_n2 * p.p248);
        let eq182_e2292_d_n3: f64 = (eq182_e2290_d_n3 * p.p248);
        let eq182_e2292_d_n4: f64 = (eq182_e2290_d_n4 * p.p248);
        let eq182_e2292_d_n5: f64 = (eq182_e2290_d_n5 * p.p248);
        let eq182_e2292_d_n6: f64 = (eq182_e2290_d_n6 * p.p248);
        let eq182_e2292_d_n7: f64 = (eq182_e2290_d_n7 * p.p248);
        let eq182_e2292_d_n8: f64 = (eq182_e2290_d_n8 * p.p248);
        let eq182_e2292_d_n9: f64 = (eq182_e2290_d_n9 * p.p248);
        let eq182_e2292_d_n10: f64 = (eq182_e2290_d_n10 * p.p248);
        let eq182_e2292_d_n11: f64 = (eq182_e2290_d_n11 * p.p248);
        let eq182_e2292_d_n12: f64 = (eq182_e2290_d_n12 * p.p248);
        let eq182_e2292_d_n13: f64 = (eq182_e2290_d_n13 * p.p248);
        let eq182_e2292_d_n14: f64 = (eq182_e2290_d_n14 * p.p248);
        let eq182_e2292_d_n15: f64 = (eq182_e2290_d_n15 * p.p248);
        let eq182_e2292_d_n16: f64 = (eq182_e2290_d_n16 * p.p248);
        let eq182_e2292_d_n17: f64 = (eq182_e2290_d_n17 * p.p248);
        let eq182_e2292_d_n18: f64 = (eq182_e2290_d_n18 * p.p248);
        let eq182_e2292_d_n19: f64 = (eq182_e2290_d_n19 * p.p248);
        let eq182_e2292_d_n20: f64 = (eq182_e2290_d_n20 * p.p248);
        let eq182_e2292_d_n21: f64 = (eq182_e2290_d_n21 * p.p248);
        let eq182_e2292_d_n22: f64 = (eq182_e2290_d_n22 * p.p248);
        (eq182_e2292, eq182_e2292_d_n0, eq182_e2292_d_n1, eq182_e2292_d_n2, eq182_e2292_d_n3, eq182_e2292_d_n4, eq182_e2292_d_n5, eq182_e2292_d_n6, eq182_e2292_d_n7, eq182_e2292_d_n8, eq182_e2292_d_n9, eq182_e2292_d_n10, eq182_e2292_d_n11, eq182_e2292_d_n12, eq182_e2292_d_n13, eq182_e2292_d_n14, eq182_e2292_d_n15, eq182_e2292_d_n16, eq182_e2292_d_n17, eq182_e2292_d_n18, eq182_e2292_d_n19, eq182_e2292_d_n20, eq182_e2292_d_n21, eq182_e2292_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq182_value: f64 = eq182_e2294;
        let eq182_node_derivatives: [f64; 23] = [eq182_e2294_d_n0, eq182_e2294_d_n1, eq182_e2294_d_n2, eq182_e2294_d_n3, eq182_e2294_d_n4, eq182_e2294_d_n5, eq182_e2294_d_n6, eq182_e2294_d_n7, eq182_e2294_d_n8, eq182_e2294_d_n9, eq182_e2294_d_n10, eq182_e2294_d_n11, eq182_e2294_d_n12, eq182_e2294_d_n13, eq182_e2294_d_n14, eq182_e2294_d_n15, eq182_e2294_d_n16, eq182_e2294_d_n17, eq182_e2294_d_n18, eq182_e2294_d_n19, eq182_e2294_d_n20, eq182_e2294_d_n21, eq182_e2294_d_n22];
        let eq182_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[21]),
            self.multiplicity * (eq182_value),
            &nodes,
            &eq182_node_derivatives,
            &branches,
            &eq182_branch_derivatives,
            self.multiplicity,
        );
    }
}
