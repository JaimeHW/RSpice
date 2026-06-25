#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let (eq33_e503, eq33_e503_d_n0, eq33_e503_d_n1, eq33_e503_d_n2, eq33_e503_d_n3, eq33_e503_d_n4, eq33_e503_d_n5, eq33_e503_d_n6, eq33_e503_d_n7, eq33_e503_d_n8, eq33_e503_d_n9, eq33_e503_d_n10, eq33_e503_d_n11, eq33_e503_d_n12, eq33_e503_d_n13, eq33_e503_d_n14, eq33_e503_d_n15, eq33_e503_d_n16, eq33_e503_d_n17, eq33_e503_d_n18, eq33_e503_d_b0, eq33_e503_d_b1, eq33_e503_d_b2, eq33_e503_d_b3, eq33_e503_d_b4, eq33_e503_d_b5, eq33_e503_d_b6, eq33_e503_d_b7, eq33_e503_d_b8, eq33_e503_d_b9, eq33_e503_d_b10, eq33_e503_d_b11,) = {
    if (s.v[1847] != 0.0) {
        let eq33_e500: f64 = (s.v[312] + s.v[573]);
        let eq33_e500_d_n0: f64 = (s.dn[312][0] + s.dn[573][0]);
        let eq33_e500_d_n1: f64 = (s.dn[312][1] + s.dn[573][1]);
        let eq33_e500_d_n2: f64 = (s.dn[312][2] + s.dn[573][2]);
        let eq33_e500_d_n3: f64 = (s.dn[312][3] + s.dn[573][3]);
        let eq33_e500_d_n4: f64 = (s.dn[312][4] + s.dn[573][4]);
        let eq33_e500_d_n5: f64 = (s.dn[312][5] + s.dn[573][5]);
        let eq33_e500_d_n6: f64 = (s.dn[312][6] + s.dn[573][6]);
        let eq33_e500_d_n7: f64 = (s.dn[312][7] + s.dn[573][7]);
        let eq33_e500_d_n8: f64 = (s.dn[312][8] + s.dn[573][8]);
        let eq33_e500_d_n9: f64 = (s.dn[312][9] + s.dn[573][9]);
        let eq33_e500_d_n10: f64 = (s.dn[312][10] + s.dn[573][10]);
        let eq33_e500_d_n11: f64 = (s.dn[312][11] + s.dn[573][11]);
        let eq33_e500_d_n12: f64 = (s.dn[312][12] + s.dn[573][12]);
        let eq33_e500_d_n13: f64 = (s.dn[312][13] + s.dn[573][13]);
        let eq33_e500_d_n14: f64 = (s.dn[312][14] + s.dn[573][14]);
        let eq33_e500_d_n15: f64 = (s.dn[312][15] + s.dn[573][15]);
        let eq33_e500_d_n16: f64 = (s.dn[312][16] + s.dn[573][16]);
        let eq33_e500_d_n17: f64 = (s.dn[312][17] + s.dn[573][17]);
        let eq33_e500_d_n18: f64 = (s.dn[312][18] + s.dn[573][18]);
        let eq33_e500_d_b0: f64 = (s.db[312][0] + s.db[573][0]);
        let eq33_e500_d_b1: f64 = (s.db[312][1] + s.db[573][1]);
        let eq33_e500_d_b2: f64 = (s.db[312][2] + s.db[573][2]);
        let eq33_e500_d_b3: f64 = (s.db[312][3] + s.db[573][3]);
        let eq33_e500_d_b4: f64 = (s.db[312][4] + s.db[573][4]);
        let eq33_e500_d_b5: f64 = (s.db[312][5] + s.db[573][5]);
        let eq33_e500_d_b6: f64 = (s.db[312][6] + s.db[573][6]);
        let eq33_e500_d_b7: f64 = (s.db[312][7] + s.db[573][7]);
        let eq33_e500_d_b8: f64 = (s.db[312][8] + s.db[573][8]);
        let eq33_e500_d_b9: f64 = (s.db[312][9] + s.db[573][9]);
        let eq33_e500_d_b10: f64 = (s.db[312][10] + s.db[573][10]);
        let eq33_e500_d_b11: f64 = (s.db[312][11] + s.db[573][11]);
        let eq33_e501: f64 = (p.p50 * eq33_e500);
        let eq33_e501_d_n0: f64 = (p.p50 * eq33_e500_d_n0);
        let eq33_e501_d_n1: f64 = (p.p50 * eq33_e500_d_n1);
        let eq33_e501_d_n2: f64 = (p.p50 * eq33_e500_d_n2);
        let eq33_e501_d_n3: f64 = (p.p50 * eq33_e500_d_n3);
        let eq33_e501_d_n4: f64 = (p.p50 * eq33_e500_d_n4);
        let eq33_e501_d_n5: f64 = (p.p50 * eq33_e500_d_n5);
        let eq33_e501_d_n6: f64 = (p.p50 * eq33_e500_d_n6);
        let eq33_e501_d_n7: f64 = (p.p50 * eq33_e500_d_n7);
        let eq33_e501_d_n8: f64 = (p.p50 * eq33_e500_d_n8);
        let eq33_e501_d_n9: f64 = (p.p50 * eq33_e500_d_n9);
        let eq33_e501_d_n10: f64 = (p.p50 * eq33_e500_d_n10);
        let eq33_e501_d_n11: f64 = (p.p50 * eq33_e500_d_n11);
        let eq33_e501_d_n12: f64 = (p.p50 * eq33_e500_d_n12);
        let eq33_e501_d_n13: f64 = (p.p50 * eq33_e500_d_n13);
        let eq33_e501_d_n14: f64 = (p.p50 * eq33_e500_d_n14);
        let eq33_e501_d_n15: f64 = (p.p50 * eq33_e500_d_n15);
        let eq33_e501_d_n16: f64 = (p.p50 * eq33_e500_d_n16);
        let eq33_e501_d_n17: f64 = (p.p50 * eq33_e500_d_n17);
        let eq33_e501_d_n18: f64 = (p.p50 * eq33_e500_d_n18);
        let eq33_e501_d_b0: f64 = (p.p50 * eq33_e500_d_b0);
        let eq33_e501_d_b1: f64 = (p.p50 * eq33_e500_d_b1);
        let eq33_e501_d_b2: f64 = (p.p50 * eq33_e500_d_b2);
        let eq33_e501_d_b3: f64 = (p.p50 * eq33_e500_d_b3);
        let eq33_e501_d_b4: f64 = (p.p50 * eq33_e500_d_b4);
        let eq33_e501_d_b5: f64 = (p.p50 * eq33_e500_d_b5);
        let eq33_e501_d_b6: f64 = (p.p50 * eq33_e500_d_b6);
        let eq33_e501_d_b7: f64 = (p.p50 * eq33_e500_d_b7);
        let eq33_e501_d_b8: f64 = (p.p50 * eq33_e500_d_b8);
        let eq33_e501_d_b9: f64 = (p.p50 * eq33_e500_d_b9);
        let eq33_e501_d_b10: f64 = (p.p50 * eq33_e500_d_b10);
        let eq33_e501_d_b11: f64 = (p.p50 * eq33_e500_d_b11);
        (eq33_e501, eq33_e501_d_n0, eq33_e501_d_n1, eq33_e501_d_n2, eq33_e501_d_n3, eq33_e501_d_n4, eq33_e501_d_n5, eq33_e501_d_n6, eq33_e501_d_n7, eq33_e501_d_n8, eq33_e501_d_n9, eq33_e501_d_n10, eq33_e501_d_n11, eq33_e501_d_n12, eq33_e501_d_n13, eq33_e501_d_n14, eq33_e501_d_n15, eq33_e501_d_n16, eq33_e501_d_n17, eq33_e501_d_n18, eq33_e501_d_b0, eq33_e501_d_b1, eq33_e501_d_b2, eq33_e501_d_b3, eq33_e501_d_b4, eq33_e501_d_b5, eq33_e501_d_b6, eq33_e501_d_b7, eq33_e501_d_b8, eq33_e501_d_b9, eq33_e501_d_b10, eq33_e501_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e503;
        let eq33_node_derivatives: [f64; 19] = [eq33_e503_d_n0, eq33_e503_d_n1, eq33_e503_d_n2, eq33_e503_d_n3, eq33_e503_d_n4, eq33_e503_d_n5, eq33_e503_d_n6, eq33_e503_d_n7, eq33_e503_d_n8, eq33_e503_d_n9, eq33_e503_d_n10, eq33_e503_d_n11, eq33_e503_d_n12, eq33_e503_d_n13, eq33_e503_d_n14, eq33_e503_d_n15, eq33_e503_d_n16, eq33_e503_d_n17, eq33_e503_d_n18];
        let eq33_branch_derivatives: [f64; 12] = [eq33_e503_d_b0, eq33_e503_d_b1, eq33_e503_d_b2, eq33_e503_d_b3, eq33_e503_d_b4, eq33_e503_d_b5, eq33_e503_d_b6, eq33_e503_d_b7, eq33_e503_d_b8, eq33_e503_d_b9, eq33_e503_d_b10, eq33_e503_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[12]),
            self.multiplicity * (eq33_value),
            &nodes,
            &eq33_node_derivatives,
            &branches,
            &eq33_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_34_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq34_e512, eq34_e512_d_n0, eq34_e512_d_n1, eq34_e512_d_n2, eq34_e512_d_n3, eq34_e512_d_n4, eq34_e512_d_n5, eq34_e512_d_n6, eq34_e512_d_n7, eq34_e512_d_n8, eq34_e512_d_n9, eq34_e512_d_n10, eq34_e512_d_n11, eq34_e512_d_n12, eq34_e512_d_n13, eq34_e512_d_n14, eq34_e512_d_n15, eq34_e512_d_n16, eq34_e512_d_n17, eq34_e512_d_n18, eq34_e512_d_b0, eq34_e512_d_b1, eq34_e512_d_b2, eq34_e512_d_b3, eq34_e512_d_b4, eq34_e512_d_b5, eq34_e512_d_b6, eq34_e512_d_b7, eq34_e512_d_b8, eq34_e512_d_b9, eq34_e512_d_b10, eq34_e512_d_b11,) = {
    if (s.v[1847] != 0.0) {
        let eq34_e508: f64 = self.eval_ddt(6, s.v[283]);
        let eq34_e508_d_n0: f64 = self.ddt_jacobian(s.dn[283][0]);
        let eq34_e508_d_n1: f64 = self.ddt_jacobian(s.dn[283][1]);
        let eq34_e508_d_n2: f64 = self.ddt_jacobian(s.dn[283][2]);
        let eq34_e508_d_n3: f64 = self.ddt_jacobian(s.dn[283][3]);
        let eq34_e508_d_n4: f64 = self.ddt_jacobian(s.dn[283][4]);
        let eq34_e508_d_n5: f64 = self.ddt_jacobian(s.dn[283][5]);
        let eq34_e508_d_n6: f64 = self.ddt_jacobian(s.dn[283][6]);
        let eq34_e508_d_n7: f64 = self.ddt_jacobian(s.dn[283][7]);
        let eq34_e508_d_n8: f64 = self.ddt_jacobian(s.dn[283][8]);
        let eq34_e508_d_n9: f64 = self.ddt_jacobian(s.dn[283][9]);
        let eq34_e508_d_n10: f64 = self.ddt_jacobian(s.dn[283][10]);
        let eq34_e508_d_n11: f64 = self.ddt_jacobian(s.dn[283][11]);
        let eq34_e508_d_n12: f64 = self.ddt_jacobian(s.dn[283][12]);
        let eq34_e508_d_n13: f64 = self.ddt_jacobian(s.dn[283][13]);
        let eq34_e508_d_n14: f64 = self.ddt_jacobian(s.dn[283][14]);
        let eq34_e508_d_n15: f64 = self.ddt_jacobian(s.dn[283][15]);
        let eq34_e508_d_n16: f64 = self.ddt_jacobian(s.dn[283][16]);
        let eq34_e508_d_n17: f64 = self.ddt_jacobian(s.dn[283][17]);
        let eq34_e508_d_n18: f64 = self.ddt_jacobian(s.dn[283][18]);
        let eq34_e508_d_b0: f64 = self.ddt_jacobian(s.db[283][0]);
        let eq34_e508_d_b1: f64 = self.ddt_jacobian(s.db[283][1]);
        let eq34_e508_d_b2: f64 = self.ddt_jacobian(s.db[283][2]);
        let eq34_e508_d_b3: f64 = self.ddt_jacobian(s.db[283][3]);
        let eq34_e508_d_b4: f64 = self.ddt_jacobian(s.db[283][4]);
        let eq34_e508_d_b5: f64 = self.ddt_jacobian(s.db[283][5]);
        let eq34_e508_d_b6: f64 = self.ddt_jacobian(s.db[283][6]);
        let eq34_e508_d_b7: f64 = self.ddt_jacobian(s.db[283][7]);
        let eq34_e508_d_b8: f64 = self.ddt_jacobian(s.db[283][8]);
        let eq34_e508_d_b9: f64 = self.ddt_jacobian(s.db[283][9]);
        let eq34_e508_d_b10: f64 = self.ddt_jacobian(s.db[283][10]);
        let eq34_e508_d_b11: f64 = self.ddt_jacobian(s.db[283][11]);
        let eq34_e509: f64 = (s.v[281] + eq34_e508);
        let eq34_e509_d_n0: f64 = (s.dn[281][0] + eq34_e508_d_n0);
        let eq34_e509_d_n1: f64 = (s.dn[281][1] + eq34_e508_d_n1);
        let eq34_e509_d_n2: f64 = (s.dn[281][2] + eq34_e508_d_n2);
        let eq34_e509_d_n3: f64 = (s.dn[281][3] + eq34_e508_d_n3);
        let eq34_e509_d_n4: f64 = (s.dn[281][4] + eq34_e508_d_n4);
        let eq34_e509_d_n5: f64 = (s.dn[281][5] + eq34_e508_d_n5);
        let eq34_e509_d_n6: f64 = (s.dn[281][6] + eq34_e508_d_n6);
        let eq34_e509_d_n7: f64 = (s.dn[281][7] + eq34_e508_d_n7);
        let eq34_e509_d_n8: f64 = (s.dn[281][8] + eq34_e508_d_n8);
        let eq34_e509_d_n9: f64 = (s.dn[281][9] + eq34_e508_d_n9);
        let eq34_e509_d_n10: f64 = (s.dn[281][10] + eq34_e508_d_n10);
        let eq34_e509_d_n11: f64 = (s.dn[281][11] + eq34_e508_d_n11);
        let eq34_e509_d_n12: f64 = (s.dn[281][12] + eq34_e508_d_n12);
        let eq34_e509_d_n13: f64 = (s.dn[281][13] + eq34_e508_d_n13);
        let eq34_e509_d_n14: f64 = (s.dn[281][14] + eq34_e508_d_n14);
        let eq34_e509_d_n15: f64 = (s.dn[281][15] + eq34_e508_d_n15);
        let eq34_e509_d_n16: f64 = (s.dn[281][16] + eq34_e508_d_n16);
        let eq34_e509_d_n17: f64 = (s.dn[281][17] + eq34_e508_d_n17);
        let eq34_e509_d_n18: f64 = (s.dn[281][18] + eq34_e508_d_n18);
        let eq34_e509_d_b0: f64 = (s.db[281][0] + eq34_e508_d_b0);
        let eq34_e509_d_b1: f64 = (s.db[281][1] + eq34_e508_d_b1);
        let eq34_e509_d_b2: f64 = (s.db[281][2] + eq34_e508_d_b2);
        let eq34_e509_d_b3: f64 = (s.db[281][3] + eq34_e508_d_b3);
        let eq34_e509_d_b4: f64 = (s.db[281][4] + eq34_e508_d_b4);
        let eq34_e509_d_b5: f64 = (s.db[281][5] + eq34_e508_d_b5);
        let eq34_e509_d_b6: f64 = (s.db[281][6] + eq34_e508_d_b6);
        let eq34_e509_d_b7: f64 = (s.db[281][7] + eq34_e508_d_b7);
        let eq34_e509_d_b8: f64 = (s.db[281][8] + eq34_e508_d_b8);
        let eq34_e509_d_b9: f64 = (s.db[281][9] + eq34_e508_d_b9);
        let eq34_e509_d_b10: f64 = (s.db[281][10] + eq34_e508_d_b10);
        let eq34_e509_d_b11: f64 = (s.db[281][11] + eq34_e508_d_b11);
        let eq34_e510: f64 = (p.p50 * eq34_e509);
        let eq34_e510_d_n0: f64 = (p.p50 * eq34_e509_d_n0);
        let eq34_e510_d_n1: f64 = (p.p50 * eq34_e509_d_n1);
        let eq34_e510_d_n2: f64 = (p.p50 * eq34_e509_d_n2);
        let eq34_e510_d_n3: f64 = (p.p50 * eq34_e509_d_n3);
        let eq34_e510_d_n4: f64 = (p.p50 * eq34_e509_d_n4);
        let eq34_e510_d_n5: f64 = (p.p50 * eq34_e509_d_n5);
        let eq34_e510_d_n6: f64 = (p.p50 * eq34_e509_d_n6);
        let eq34_e510_d_n7: f64 = (p.p50 * eq34_e509_d_n7);
        let eq34_e510_d_n8: f64 = (p.p50 * eq34_e509_d_n8);
        let eq34_e510_d_n9: f64 = (p.p50 * eq34_e509_d_n9);
        let eq34_e510_d_n10: f64 = (p.p50 * eq34_e509_d_n10);
        let eq34_e510_d_n11: f64 = (p.p50 * eq34_e509_d_n11);
        let eq34_e510_d_n12: f64 = (p.p50 * eq34_e509_d_n12);
        let eq34_e510_d_n13: f64 = (p.p50 * eq34_e509_d_n13);
        let eq34_e510_d_n14: f64 = (p.p50 * eq34_e509_d_n14);
        let eq34_e510_d_n15: f64 = (p.p50 * eq34_e509_d_n15);
        let eq34_e510_d_n16: f64 = (p.p50 * eq34_e509_d_n16);
        let eq34_e510_d_n17: f64 = (p.p50 * eq34_e509_d_n17);
        let eq34_e510_d_n18: f64 = (p.p50 * eq34_e509_d_n18);
        let eq34_e510_d_b0: f64 = (p.p50 * eq34_e509_d_b0);
        let eq34_e510_d_b1: f64 = (p.p50 * eq34_e509_d_b1);
        let eq34_e510_d_b2: f64 = (p.p50 * eq34_e509_d_b2);
        let eq34_e510_d_b3: f64 = (p.p50 * eq34_e509_d_b3);
        let eq34_e510_d_b4: f64 = (p.p50 * eq34_e509_d_b4);
        let eq34_e510_d_b5: f64 = (p.p50 * eq34_e509_d_b5);
        let eq34_e510_d_b6: f64 = (p.p50 * eq34_e509_d_b6);
        let eq34_e510_d_b7: f64 = (p.p50 * eq34_e509_d_b7);
        let eq34_e510_d_b8: f64 = (p.p50 * eq34_e509_d_b8);
        let eq34_e510_d_b9: f64 = (p.p50 * eq34_e509_d_b9);
        let eq34_e510_d_b10: f64 = (p.p50 * eq34_e509_d_b10);
        let eq34_e510_d_b11: f64 = (p.p50 * eq34_e509_d_b11);
        (eq34_e510, eq34_e510_d_n0, eq34_e510_d_n1, eq34_e510_d_n2, eq34_e510_d_n3, eq34_e510_d_n4, eq34_e510_d_n5, eq34_e510_d_n6, eq34_e510_d_n7, eq34_e510_d_n8, eq34_e510_d_n9, eq34_e510_d_n10, eq34_e510_d_n11, eq34_e510_d_n12, eq34_e510_d_n13, eq34_e510_d_n14, eq34_e510_d_n15, eq34_e510_d_n16, eq34_e510_d_n17, eq34_e510_d_n18, eq34_e510_d_b0, eq34_e510_d_b1, eq34_e510_d_b2, eq34_e510_d_b3, eq34_e510_d_b4, eq34_e510_d_b5, eq34_e510_d_b6, eq34_e510_d_b7, eq34_e510_d_b8, eq34_e510_d_b9, eq34_e510_d_b10, eq34_e510_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e512;
        let eq34_node_derivatives: [f64; 19] = [eq34_e512_d_n0, eq34_e512_d_n1, eq34_e512_d_n2, eq34_e512_d_n3, eq34_e512_d_n4, eq34_e512_d_n5, eq34_e512_d_n6, eq34_e512_d_n7, eq34_e512_d_n8, eq34_e512_d_n9, eq34_e512_d_n10, eq34_e512_d_n11, eq34_e512_d_n12, eq34_e512_d_n13, eq34_e512_d_n14, eq34_e512_d_n15, eq34_e512_d_n16, eq34_e512_d_n17, eq34_e512_d_n18];
        let eq34_branch_derivatives: [f64; 12] = [eq34_e512_d_b0, eq34_e512_d_b1, eq34_e512_d_b2, eq34_e512_d_b3, eq34_e512_d_b4, eq34_e512_d_b5, eq34_e512_d_b6, eq34_e512_d_b7, eq34_e512_d_b8, eq34_e512_d_b9, eq34_e512_d_b10, eq34_e512_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq34_value),
            &nodes,
            &eq34_node_derivatives,
            &branches,
            &eq34_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_35_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq35_e521, eq35_e521_d_n0, eq35_e521_d_n1, eq35_e521_d_n2, eq35_e521_d_n3, eq35_e521_d_n4, eq35_e521_d_n5, eq35_e521_d_n6, eq35_e521_d_n7, eq35_e521_d_n8, eq35_e521_d_n9, eq35_e521_d_n10, eq35_e521_d_n11, eq35_e521_d_n12, eq35_e521_d_n13, eq35_e521_d_n14, eq35_e521_d_n15, eq35_e521_d_n16, eq35_e521_d_n17, eq35_e521_d_n18, eq35_e521_d_b0, eq35_e521_d_b1, eq35_e521_d_b2, eq35_e521_d_b3, eq35_e521_d_b4, eq35_e521_d_b5, eq35_e521_d_b6, eq35_e521_d_b7, eq35_e521_d_b8, eq35_e521_d_b9, eq35_e521_d_b10, eq35_e521_d_b11,) = {
    if (s.v[1847] != 0.0) {
        let eq35_e517: f64 = self.eval_ddt(7, s.v[284]);
        let eq35_e517_d_n0: f64 = self.ddt_jacobian(s.dn[284][0]);
        let eq35_e517_d_n1: f64 = self.ddt_jacobian(s.dn[284][1]);
        let eq35_e517_d_n2: f64 = self.ddt_jacobian(s.dn[284][2]);
        let eq35_e517_d_n3: f64 = self.ddt_jacobian(s.dn[284][3]);
        let eq35_e517_d_n4: f64 = self.ddt_jacobian(s.dn[284][4]);
        let eq35_e517_d_n5: f64 = self.ddt_jacobian(s.dn[284][5]);
        let eq35_e517_d_n6: f64 = self.ddt_jacobian(s.dn[284][6]);
        let eq35_e517_d_n7: f64 = self.ddt_jacobian(s.dn[284][7]);
        let eq35_e517_d_n8: f64 = self.ddt_jacobian(s.dn[284][8]);
        let eq35_e517_d_n9: f64 = self.ddt_jacobian(s.dn[284][9]);
        let eq35_e517_d_n10: f64 = self.ddt_jacobian(s.dn[284][10]);
        let eq35_e517_d_n11: f64 = self.ddt_jacobian(s.dn[284][11]);
        let eq35_e517_d_n12: f64 = self.ddt_jacobian(s.dn[284][12]);
        let eq35_e517_d_n13: f64 = self.ddt_jacobian(s.dn[284][13]);
        let eq35_e517_d_n14: f64 = self.ddt_jacobian(s.dn[284][14]);
        let eq35_e517_d_n15: f64 = self.ddt_jacobian(s.dn[284][15]);
        let eq35_e517_d_n16: f64 = self.ddt_jacobian(s.dn[284][16]);
        let eq35_e517_d_n17: f64 = self.ddt_jacobian(s.dn[284][17]);
        let eq35_e517_d_n18: f64 = self.ddt_jacobian(s.dn[284][18]);
        let eq35_e517_d_b0: f64 = self.ddt_jacobian(s.db[284][0]);
        let eq35_e517_d_b1: f64 = self.ddt_jacobian(s.db[284][1]);
        let eq35_e517_d_b2: f64 = self.ddt_jacobian(s.db[284][2]);
        let eq35_e517_d_b3: f64 = self.ddt_jacobian(s.db[284][3]);
        let eq35_e517_d_b4: f64 = self.ddt_jacobian(s.db[284][4]);
        let eq35_e517_d_b5: f64 = self.ddt_jacobian(s.db[284][5]);
        let eq35_e517_d_b6: f64 = self.ddt_jacobian(s.db[284][6]);
        let eq35_e517_d_b7: f64 = self.ddt_jacobian(s.db[284][7]);
        let eq35_e517_d_b8: f64 = self.ddt_jacobian(s.db[284][8]);
        let eq35_e517_d_b9: f64 = self.ddt_jacobian(s.db[284][9]);
        let eq35_e517_d_b10: f64 = self.ddt_jacobian(s.db[284][10]);
        let eq35_e517_d_b11: f64 = self.ddt_jacobian(s.db[284][11]);
        let eq35_e518: f64 = (s.v[282] + eq35_e517);
        let eq35_e518_d_n0: f64 = (s.dn[282][0] + eq35_e517_d_n0);
        let eq35_e518_d_n1: f64 = (s.dn[282][1] + eq35_e517_d_n1);
        let eq35_e518_d_n2: f64 = (s.dn[282][2] + eq35_e517_d_n2);
        let eq35_e518_d_n3: f64 = (s.dn[282][3] + eq35_e517_d_n3);
        let eq35_e518_d_n4: f64 = (s.dn[282][4] + eq35_e517_d_n4);
        let eq35_e518_d_n5: f64 = (s.dn[282][5] + eq35_e517_d_n5);
        let eq35_e518_d_n6: f64 = (s.dn[282][6] + eq35_e517_d_n6);
        let eq35_e518_d_n7: f64 = (s.dn[282][7] + eq35_e517_d_n7);
        let eq35_e518_d_n8: f64 = (s.dn[282][8] + eq35_e517_d_n8);
        let eq35_e518_d_n9: f64 = (s.dn[282][9] + eq35_e517_d_n9);
        let eq35_e518_d_n10: f64 = (s.dn[282][10] + eq35_e517_d_n10);
        let eq35_e518_d_n11: f64 = (s.dn[282][11] + eq35_e517_d_n11);
        let eq35_e518_d_n12: f64 = (s.dn[282][12] + eq35_e517_d_n12);
        let eq35_e518_d_n13: f64 = (s.dn[282][13] + eq35_e517_d_n13);
        let eq35_e518_d_n14: f64 = (s.dn[282][14] + eq35_e517_d_n14);
        let eq35_e518_d_n15: f64 = (s.dn[282][15] + eq35_e517_d_n15);
        let eq35_e518_d_n16: f64 = (s.dn[282][16] + eq35_e517_d_n16);
        let eq35_e518_d_n17: f64 = (s.dn[282][17] + eq35_e517_d_n17);
        let eq35_e518_d_n18: f64 = (s.dn[282][18] + eq35_e517_d_n18);
        let eq35_e518_d_b0: f64 = (s.db[282][0] + eq35_e517_d_b0);
        let eq35_e518_d_b1: f64 = (s.db[282][1] + eq35_e517_d_b1);
        let eq35_e518_d_b2: f64 = (s.db[282][2] + eq35_e517_d_b2);
        let eq35_e518_d_b3: f64 = (s.db[282][3] + eq35_e517_d_b3);
        let eq35_e518_d_b4: f64 = (s.db[282][4] + eq35_e517_d_b4);
        let eq35_e518_d_b5: f64 = (s.db[282][5] + eq35_e517_d_b5);
        let eq35_e518_d_b6: f64 = (s.db[282][6] + eq35_e517_d_b6);
        let eq35_e518_d_b7: f64 = (s.db[282][7] + eq35_e517_d_b7);
        let eq35_e518_d_b8: f64 = (s.db[282][8] + eq35_e517_d_b8);
        let eq35_e518_d_b9: f64 = (s.db[282][9] + eq35_e517_d_b9);
        let eq35_e518_d_b10: f64 = (s.db[282][10] + eq35_e517_d_b10);
        let eq35_e518_d_b11: f64 = (s.db[282][11] + eq35_e517_d_b11);
        let eq35_e519: f64 = (p.p50 * eq35_e518);
        let eq35_e519_d_n0: f64 = (p.p50 * eq35_e518_d_n0);
        let eq35_e519_d_n1: f64 = (p.p50 * eq35_e518_d_n1);
        let eq35_e519_d_n2: f64 = (p.p50 * eq35_e518_d_n2);
        let eq35_e519_d_n3: f64 = (p.p50 * eq35_e518_d_n3);
        let eq35_e519_d_n4: f64 = (p.p50 * eq35_e518_d_n4);
        let eq35_e519_d_n5: f64 = (p.p50 * eq35_e518_d_n5);
        let eq35_e519_d_n6: f64 = (p.p50 * eq35_e518_d_n6);
        let eq35_e519_d_n7: f64 = (p.p50 * eq35_e518_d_n7);
        let eq35_e519_d_n8: f64 = (p.p50 * eq35_e518_d_n8);
        let eq35_e519_d_n9: f64 = (p.p50 * eq35_e518_d_n9);
        let eq35_e519_d_n10: f64 = (p.p50 * eq35_e518_d_n10);
        let eq35_e519_d_n11: f64 = (p.p50 * eq35_e518_d_n11);
        let eq35_e519_d_n12: f64 = (p.p50 * eq35_e518_d_n12);
        let eq35_e519_d_n13: f64 = (p.p50 * eq35_e518_d_n13);
        let eq35_e519_d_n14: f64 = (p.p50 * eq35_e518_d_n14);
        let eq35_e519_d_n15: f64 = (p.p50 * eq35_e518_d_n15);
        let eq35_e519_d_n16: f64 = (p.p50 * eq35_e518_d_n16);
        let eq35_e519_d_n17: f64 = (p.p50 * eq35_e518_d_n17);
        let eq35_e519_d_n18: f64 = (p.p50 * eq35_e518_d_n18);
        let eq35_e519_d_b0: f64 = (p.p50 * eq35_e518_d_b0);
        let eq35_e519_d_b1: f64 = (p.p50 * eq35_e518_d_b1);
        let eq35_e519_d_b2: f64 = (p.p50 * eq35_e518_d_b2);
        let eq35_e519_d_b3: f64 = (p.p50 * eq35_e518_d_b3);
        let eq35_e519_d_b4: f64 = (p.p50 * eq35_e518_d_b4);
        let eq35_e519_d_b5: f64 = (p.p50 * eq35_e518_d_b5);
        let eq35_e519_d_b6: f64 = (p.p50 * eq35_e518_d_b6);
        let eq35_e519_d_b7: f64 = (p.p50 * eq35_e518_d_b7);
        let eq35_e519_d_b8: f64 = (p.p50 * eq35_e518_d_b8);
        let eq35_e519_d_b9: f64 = (p.p50 * eq35_e518_d_b9);
        let eq35_e519_d_b10: f64 = (p.p50 * eq35_e518_d_b10);
        let eq35_e519_d_b11: f64 = (p.p50 * eq35_e518_d_b11);
        (eq35_e519, eq35_e519_d_n0, eq35_e519_d_n1, eq35_e519_d_n2, eq35_e519_d_n3, eq35_e519_d_n4, eq35_e519_d_n5, eq35_e519_d_n6, eq35_e519_d_n7, eq35_e519_d_n8, eq35_e519_d_n9, eq35_e519_d_n10, eq35_e519_d_n11, eq35_e519_d_n12, eq35_e519_d_n13, eq35_e519_d_n14, eq35_e519_d_n15, eq35_e519_d_n16, eq35_e519_d_n17, eq35_e519_d_n18, eq35_e519_d_b0, eq35_e519_d_b1, eq35_e519_d_b2, eq35_e519_d_b3, eq35_e519_d_b4, eq35_e519_d_b5, eq35_e519_d_b6, eq35_e519_d_b7, eq35_e519_d_b8, eq35_e519_d_b9, eq35_e519_d_b10, eq35_e519_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e521;
        let eq35_node_derivatives: [f64; 19] = [eq35_e521_d_n0, eq35_e521_d_n1, eq35_e521_d_n2, eq35_e521_d_n3, eq35_e521_d_n4, eq35_e521_d_n5, eq35_e521_d_n6, eq35_e521_d_n7, eq35_e521_d_n8, eq35_e521_d_n9, eq35_e521_d_n10, eq35_e521_d_n11, eq35_e521_d_n12, eq35_e521_d_n13, eq35_e521_d_n14, eq35_e521_d_n15, eq35_e521_d_n16, eq35_e521_d_n17, eq35_e521_d_n18];
        let eq35_branch_derivatives: [f64; 12] = [eq35_e521_d_b0, eq35_e521_d_b1, eq35_e521_d_b2, eq35_e521_d_b3, eq35_e521_d_b4, eq35_e521_d_b5, eq35_e521_d_b6, eq35_e521_d_b7, eq35_e521_d_b8, eq35_e521_d_b9, eq35_e521_d_b10, eq35_e521_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            self.multiplicity * (eq35_value),
            &nodes,
            &eq35_node_derivatives,
            &branches,
            &eq35_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_36_block_0(
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq36_e529, eq36_e529_d_n0, eq36_e529_d_n1, eq36_e529_d_n2, eq36_e529_d_n3, eq36_e529_d_n4, eq36_e529_d_n5, eq36_e529_d_n6, eq36_e529_d_n7, eq36_e529_d_n8, eq36_e529_d_n9, eq36_e529_d_n10, eq36_e529_d_n11, eq36_e529_d_n12, eq36_e529_d_n13, eq36_e529_d_n14, eq36_e529_d_n15, eq36_e529_d_n16, eq36_e529_d_n17, eq36_e529_d_n18, eq36_e529_d_b0, eq36_e529_d_b1, eq36_e529_d_b2, eq36_e529_d_b3, eq36_e529_d_b4, eq36_e529_d_b5, eq36_e529_d_b6, eq36_e529_d_b7, eq36_e529_d_b8, eq36_e529_d_b9, eq36_e529_d_b10, eq36_e529_d_b11,) = {
    if ((s.v[1847] != 0.0) && (p.p261 != 0.0)) {
        let eq36_e527: f64 = ((nv4 - nv12) / s.v[2]);
        let eq36_e527_d_n0: f64 = (-(((nv4 - nv12) * s.dn[2][0]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_n1: f64 = (-(((nv4 - nv12) * s.dn[2][1]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_n2: f64 = (-(((nv4 - nv12) * s.dn[2][2]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_n3: f64 = (-(((nv4 - nv12) * s.dn[2][3]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_n4: f64 = ((s.v[2] - ((nv4 - nv12) * s.dn[2][4])) / (s.v[2] * s.v[2]));
        let eq36_e527_d_n5: f64 = (-(((nv4 - nv12) * s.dn[2][5]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_n6: f64 = (-(((nv4 - nv12) * s.dn[2][6]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_n7: f64 = (-(((nv4 - nv12) * s.dn[2][7]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_n8: f64 = (-(((nv4 - nv12) * s.dn[2][8]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_n9: f64 = (-(((nv4 - nv12) * s.dn[2][9]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_n10: f64 = (-(((nv4 - nv12) * s.dn[2][10]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_n11: f64 = (-(((nv4 - nv12) * s.dn[2][11]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_n12: f64 = (((-s.v[2]) - ((nv4 - nv12) * s.dn[2][12])) / (s.v[2] * s.v[2]));
        let eq36_e527_d_n13: f64 = (-(((nv4 - nv12) * s.dn[2][13]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_n14: f64 = (-(((nv4 - nv12) * s.dn[2][14]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_n15: f64 = (-(((nv4 - nv12) * s.dn[2][15]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_n16: f64 = (-(((nv4 - nv12) * s.dn[2][16]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_n17: f64 = (-(((nv4 - nv12) * s.dn[2][17]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_n18: f64 = (-(((nv4 - nv12) * s.dn[2][18]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_b0: f64 = (-(((nv4 - nv12) * s.db[2][0]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_b1: f64 = (-(((nv4 - nv12) * s.db[2][1]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_b2: f64 = (-(((nv4 - nv12) * s.db[2][2]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_b3: f64 = (-(((nv4 - nv12) * s.db[2][3]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_b4: f64 = (-(((nv4 - nv12) * s.db[2][4]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_b5: f64 = (-(((nv4 - nv12) * s.db[2][5]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_b6: f64 = (-(((nv4 - nv12) * s.db[2][6]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_b7: f64 = (-(((nv4 - nv12) * s.db[2][7]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_b8: f64 = (-(((nv4 - nv12) * s.db[2][8]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_b9: f64 = (-(((nv4 - nv12) * s.db[2][9]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_b10: f64 = (-(((nv4 - nv12) * s.db[2][10]) / (s.v[2] * s.v[2])));
        let eq36_e527_d_b11: f64 = (-(((nv4 - nv12) * s.db[2][11]) / (s.v[2] * s.v[2])));
        (eq36_e527, eq36_e527_d_n0, eq36_e527_d_n1, eq36_e527_d_n2, eq36_e527_d_n3, eq36_e527_d_n4, eq36_e527_d_n5, eq36_e527_d_n6, eq36_e527_d_n7, eq36_e527_d_n8, eq36_e527_d_n9, eq36_e527_d_n10, eq36_e527_d_n11, eq36_e527_d_n12, eq36_e527_d_n13, eq36_e527_d_n14, eq36_e527_d_n15, eq36_e527_d_n16, eq36_e527_d_n17, eq36_e527_d_n18, eq36_e527_d_b0, eq36_e527_d_b1, eq36_e527_d_b2, eq36_e527_d_b3, eq36_e527_d_b4, eq36_e527_d_b5, eq36_e527_d_b6, eq36_e527_d_b7, eq36_e527_d_b8, eq36_e527_d_b9, eq36_e527_d_b10, eq36_e527_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e529;
        let eq36_node_derivatives: [f64; 19] = [eq36_e529_d_n0, eq36_e529_d_n1, eq36_e529_d_n2, eq36_e529_d_n3, eq36_e529_d_n4, eq36_e529_d_n5, eq36_e529_d_n6, eq36_e529_d_n7, eq36_e529_d_n8, eq36_e529_d_n9, eq36_e529_d_n10, eq36_e529_d_n11, eq36_e529_d_n12, eq36_e529_d_n13, eq36_e529_d_n14, eq36_e529_d_n15, eq36_e529_d_n16, eq36_e529_d_n17, eq36_e529_d_n18];
        let eq36_branch_derivatives: [f64; 12] = [eq36_e529_d_b0, eq36_e529_d_b1, eq36_e529_d_b2, eq36_e529_d_b3, eq36_e529_d_b4, eq36_e529_d_b5, eq36_e529_d_b6, eq36_e529_d_b7, eq36_e529_d_b8, eq36_e529_d_b9, eq36_e529_d_b10, eq36_e529_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            Some(nodes[12]),
            self.multiplicity * (eq36_value),
            &nodes,
            &eq36_node_derivatives,
            &branches,
            &eq36_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_37_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq37_e536,) = {
    if ((s.v[1847] != 0.0) && (!(p.p261 != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq37_value: f64 = eq37_e536;
        stamper.stamp_potential(
            branches[5],
            eq37_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_38_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq38_e544, eq38_e544_d_n0, eq38_e544_d_n1, eq38_e544_d_n2, eq38_e544_d_n3, eq38_e544_d_n4, eq38_e544_d_n5, eq38_e544_d_n6, eq38_e544_d_n7, eq38_e544_d_n8, eq38_e544_d_n9, eq38_e544_d_n10, eq38_e544_d_n11, eq38_e544_d_n12, eq38_e544_d_n13, eq38_e544_d_n14, eq38_e544_d_n15, eq38_e544_d_n16, eq38_e544_d_n17, eq38_e544_d_n18, eq38_e544_d_b0, eq38_e544_d_b1, eq38_e544_d_b2, eq38_e544_d_b3, eq38_e544_d_b4, eq38_e544_d_b5, eq38_e544_d_b6, eq38_e544_d_b7, eq38_e544_d_b8, eq38_e544_d_b9, eq38_e544_d_b10, eq38_e544_d_b11,) = {
    if ((s.v[1847] != 0.0) && (p.p262 != 0.0)) {
        let eq38_e542: f64 = (s.v[553] * (nv9 - nv12));
        let eq38_e542_d_n0: f64 = (s.dn[553][0] * (nv9 - nv12));
        let eq38_e542_d_n1: f64 = (s.dn[553][1] * (nv9 - nv12));
        let eq38_e542_d_n2: f64 = (s.dn[553][2] * (nv9 - nv12));
        let eq38_e542_d_n3: f64 = (s.dn[553][3] * (nv9 - nv12));
        let eq38_e542_d_n4: f64 = (s.dn[553][4] * (nv9 - nv12));
        let eq38_e542_d_n5: f64 = (s.dn[553][5] * (nv9 - nv12));
        let eq38_e542_d_n6: f64 = (s.dn[553][6] * (nv9 - nv12));
        let eq38_e542_d_n7: f64 = (s.dn[553][7] * (nv9 - nv12));
        let eq38_e542_d_n8: f64 = (s.dn[553][8] * (nv9 - nv12));
        let eq38_e542_d_n9: f64 = ((s.dn[553][9] * (nv9 - nv12)) + s.v[553]);
        let eq38_e542_d_n10: f64 = (s.dn[553][10] * (nv9 - nv12));
        let eq38_e542_d_n11: f64 = (s.dn[553][11] * (nv9 - nv12));
        let eq38_e542_d_n12: f64 = ((s.dn[553][12] * (nv9 - nv12)) + (-s.v[553]));
        let eq38_e542_d_n13: f64 = (s.dn[553][13] * (nv9 - nv12));
        let eq38_e542_d_n14: f64 = (s.dn[553][14] * (nv9 - nv12));
        let eq38_e542_d_n15: f64 = (s.dn[553][15] * (nv9 - nv12));
        let eq38_e542_d_n16: f64 = (s.dn[553][16] * (nv9 - nv12));
        let eq38_e542_d_n17: f64 = (s.dn[553][17] * (nv9 - nv12));
        let eq38_e542_d_n18: f64 = (s.dn[553][18] * (nv9 - nv12));
        let eq38_e542_d_b0: f64 = (s.db[553][0] * (nv9 - nv12));
        let eq38_e542_d_b1: f64 = (s.db[553][1] * (nv9 - nv12));
        let eq38_e542_d_b2: f64 = (s.db[553][2] * (nv9 - nv12));
        let eq38_e542_d_b3: f64 = (s.db[553][3] * (nv9 - nv12));
        let eq38_e542_d_b4: f64 = (s.db[553][4] * (nv9 - nv12));
        let eq38_e542_d_b5: f64 = (s.db[553][5] * (nv9 - nv12));
        let eq38_e542_d_b6: f64 = (s.db[553][6] * (nv9 - nv12));
        let eq38_e542_d_b7: f64 = (s.db[553][7] * (nv9 - nv12));
        let eq38_e542_d_b8: f64 = (s.db[553][8] * (nv9 - nv12));
        let eq38_e542_d_b9: f64 = (s.db[553][9] * (nv9 - nv12));
        let eq38_e542_d_b10: f64 = (s.db[553][10] * (nv9 - nv12));
        let eq38_e542_d_b11: f64 = (s.db[553][11] * (nv9 - nv12));
        (eq38_e542, eq38_e542_d_n0, eq38_e542_d_n1, eq38_e542_d_n2, eq38_e542_d_n3, eq38_e542_d_n4, eq38_e542_d_n5, eq38_e542_d_n6, eq38_e542_d_n7, eq38_e542_d_n8, eq38_e542_d_n9, eq38_e542_d_n10, eq38_e542_d_n11, eq38_e542_d_n12, eq38_e542_d_n13, eq38_e542_d_n14, eq38_e542_d_n15, eq38_e542_d_n16, eq38_e542_d_n17, eq38_e542_d_n18, eq38_e542_d_b0, eq38_e542_d_b1, eq38_e542_d_b2, eq38_e542_d_b3, eq38_e542_d_b4, eq38_e542_d_b5, eq38_e542_d_b6, eq38_e542_d_b7, eq38_e542_d_b8, eq38_e542_d_b9, eq38_e542_d_b10, eq38_e542_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e544;
        let eq38_node_derivatives: [f64; 19] = [eq38_e544_d_n0, eq38_e544_d_n1, eq38_e544_d_n2, eq38_e544_d_n3, eq38_e544_d_n4, eq38_e544_d_n5, eq38_e544_d_n6, eq38_e544_d_n7, eq38_e544_d_n8, eq38_e544_d_n9, eq38_e544_d_n10, eq38_e544_d_n11, eq38_e544_d_n12, eq38_e544_d_n13, eq38_e544_d_n14, eq38_e544_d_n15, eq38_e544_d_n16, eq38_e544_d_n17, eq38_e544_d_n18];
        let eq38_branch_derivatives: [f64; 12] = [eq38_e544_d_b0, eq38_e544_d_b1, eq38_e544_d_b2, eq38_e544_d_b3, eq38_e544_d_b4, eq38_e544_d_b5, eq38_e544_d_b6, eq38_e544_d_b7, eq38_e544_d_b8, eq38_e544_d_b9, eq38_e544_d_b10, eq38_e544_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[12]),
            self.multiplicity * (eq38_value),
            &nodes,
            &eq38_node_derivatives,
            &branches,
            &eq38_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_39_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq39_e552, eq39_e552_d_n0, eq39_e552_d_n1, eq39_e552_d_n2, eq39_e552_d_n3, eq39_e552_d_n4, eq39_e552_d_n5, eq39_e552_d_n6, eq39_e552_d_n7, eq39_e552_d_n8, eq39_e552_d_n9, eq39_e552_d_n10, eq39_e552_d_n11, eq39_e552_d_n12, eq39_e552_d_n13, eq39_e552_d_n14, eq39_e552_d_n15, eq39_e552_d_n16, eq39_e552_d_n17, eq39_e552_d_n18, eq39_e552_d_b0, eq39_e552_d_b1, eq39_e552_d_b2, eq39_e552_d_b3, eq39_e552_d_b4, eq39_e552_d_b5, eq39_e552_d_b6, eq39_e552_d_b7, eq39_e552_d_b8, eq39_e552_d_b9, eq39_e552_d_b10, eq39_e552_d_b11,) = {
    if ((s.v[1847] != 0.0) && (p.p262 != 0.0)) {
        let eq39_e550: f64 = (s.v[552] * (nv8 - nv12));
        let eq39_e550_d_n0: f64 = (s.dn[552][0] * (nv8 - nv12));
        let eq39_e550_d_n1: f64 = (s.dn[552][1] * (nv8 - nv12));
        let eq39_e550_d_n2: f64 = (s.dn[552][2] * (nv8 - nv12));
        let eq39_e550_d_n3: f64 = (s.dn[552][3] * (nv8 - nv12));
        let eq39_e550_d_n4: f64 = (s.dn[552][4] * (nv8 - nv12));
        let eq39_e550_d_n5: f64 = (s.dn[552][5] * (nv8 - nv12));
        let eq39_e550_d_n6: f64 = (s.dn[552][6] * (nv8 - nv12));
        let eq39_e550_d_n7: f64 = (s.dn[552][7] * (nv8 - nv12));
        let eq39_e550_d_n8: f64 = ((s.dn[552][8] * (nv8 - nv12)) + s.v[552]);
        let eq39_e550_d_n9: f64 = (s.dn[552][9] * (nv8 - nv12));
        let eq39_e550_d_n10: f64 = (s.dn[552][10] * (nv8 - nv12));
        let eq39_e550_d_n11: f64 = (s.dn[552][11] * (nv8 - nv12));
        let eq39_e550_d_n12: f64 = ((s.dn[552][12] * (nv8 - nv12)) + (-s.v[552]));
        let eq39_e550_d_n13: f64 = (s.dn[552][13] * (nv8 - nv12));
        let eq39_e550_d_n14: f64 = (s.dn[552][14] * (nv8 - nv12));
        let eq39_e550_d_n15: f64 = (s.dn[552][15] * (nv8 - nv12));
        let eq39_e550_d_n16: f64 = (s.dn[552][16] * (nv8 - nv12));
        let eq39_e550_d_n17: f64 = (s.dn[552][17] * (nv8 - nv12));
        let eq39_e550_d_n18: f64 = (s.dn[552][18] * (nv8 - nv12));
        let eq39_e550_d_b0: f64 = (s.db[552][0] * (nv8 - nv12));
        let eq39_e550_d_b1: f64 = (s.db[552][1] * (nv8 - nv12));
        let eq39_e550_d_b2: f64 = (s.db[552][2] * (nv8 - nv12));
        let eq39_e550_d_b3: f64 = (s.db[552][3] * (nv8 - nv12));
        let eq39_e550_d_b4: f64 = (s.db[552][4] * (nv8 - nv12));
        let eq39_e550_d_b5: f64 = (s.db[552][5] * (nv8 - nv12));
        let eq39_e550_d_b6: f64 = (s.db[552][6] * (nv8 - nv12));
        let eq39_e550_d_b7: f64 = (s.db[552][7] * (nv8 - nv12));
        let eq39_e550_d_b8: f64 = (s.db[552][8] * (nv8 - nv12));
        let eq39_e550_d_b9: f64 = (s.db[552][9] * (nv8 - nv12));
        let eq39_e550_d_b10: f64 = (s.db[552][10] * (nv8 - nv12));
        let eq39_e550_d_b11: f64 = (s.db[552][11] * (nv8 - nv12));
        (eq39_e550, eq39_e550_d_n0, eq39_e550_d_n1, eq39_e550_d_n2, eq39_e550_d_n3, eq39_e550_d_n4, eq39_e550_d_n5, eq39_e550_d_n6, eq39_e550_d_n7, eq39_e550_d_n8, eq39_e550_d_n9, eq39_e550_d_n10, eq39_e550_d_n11, eq39_e550_d_n12, eq39_e550_d_n13, eq39_e550_d_n14, eq39_e550_d_n15, eq39_e550_d_n16, eq39_e550_d_n17, eq39_e550_d_n18, eq39_e550_d_b0, eq39_e550_d_b1, eq39_e550_d_b2, eq39_e550_d_b3, eq39_e550_d_b4, eq39_e550_d_b5, eq39_e550_d_b6, eq39_e550_d_b7, eq39_e550_d_b8, eq39_e550_d_b9, eq39_e550_d_b10, eq39_e550_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e552;
        let eq39_node_derivatives: [f64; 19] = [eq39_e552_d_n0, eq39_e552_d_n1, eq39_e552_d_n2, eq39_e552_d_n3, eq39_e552_d_n4, eq39_e552_d_n5, eq39_e552_d_n6, eq39_e552_d_n7, eq39_e552_d_n8, eq39_e552_d_n9, eq39_e552_d_n10, eq39_e552_d_n11, eq39_e552_d_n12, eq39_e552_d_n13, eq39_e552_d_n14, eq39_e552_d_n15, eq39_e552_d_n16, eq39_e552_d_n17, eq39_e552_d_n18];
        let eq39_branch_derivatives: [f64; 12] = [eq39_e552_d_b0, eq39_e552_d_b1, eq39_e552_d_b2, eq39_e552_d_b3, eq39_e552_d_b4, eq39_e552_d_b5, eq39_e552_d_b6, eq39_e552_d_b7, eq39_e552_d_b8, eq39_e552_d_b9, eq39_e552_d_b10, eq39_e552_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[12]),
            self.multiplicity * (eq39_value),
            &nodes,
            &eq39_node_derivatives,
            &branches,
            &eq39_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_40_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq40_e559,) = {
    if ((s.v[1847] != 0.0) && (!(p.p262 != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq40_value: f64 = eq40_e559;
        stamper.stamp_potential(
            branches[6],
            eq40_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_41_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq41_e566,) = {
    if ((s.v[1847] != 0.0) && (!(p.p262 != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq41_value: f64 = eq41_e566;
        stamper.stamp_potential(
            branches[7],
            eq41_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_42_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq42_e572, eq42_e572_d_n0, eq42_e572_d_n1, eq42_e572_d_n2, eq42_e572_d_n3, eq42_e572_d_n4, eq42_e572_d_n5, eq42_e572_d_n6, eq42_e572_d_n7, eq42_e572_d_n8, eq42_e572_d_n9, eq42_e572_d_n10, eq42_e572_d_n11, eq42_e572_d_n12, eq42_e572_d_n13, eq42_e572_d_n14, eq42_e572_d_n15, eq42_e572_d_n16, eq42_e572_d_n17, eq42_e572_d_n18, eq42_e572_d_b0, eq42_e572_d_b1, eq42_e572_d_b2, eq42_e572_d_b3, eq42_e572_d_b4, eq42_e572_d_b5, eq42_e572_d_b6, eq42_e572_d_b7, eq42_e572_d_b8, eq42_e572_d_b9, eq42_e572_d_b10, eq42_e572_d_b11,) = {
    if ((s.v[1847] != 0.0) && (p.p34 != 0.0)) {
        (s.v[582], s.dn[582][0], s.dn[582][1], s.dn[582][2], s.dn[582][3], s.dn[582][4], s.dn[582][5], s.dn[582][6], s.dn[582][7], s.dn[582][8], s.dn[582][9], s.dn[582][10], s.dn[582][11], s.dn[582][12], s.dn[582][13], s.dn[582][14], s.dn[582][15], s.dn[582][16], s.dn[582][17], s.dn[582][18], s.db[582][0], s.db[582][1], s.db[582][2], s.db[582][3], s.db[582][4], s.db[582][5], s.db[582][6], s.db[582][7], s.db[582][8], s.db[582][9], s.db[582][10], s.db[582][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq42_value: f64 = eq42_e572;
        let eq42_node_derivatives: [f64; 19] = [eq42_e572_d_n0, eq42_e572_d_n1, eq42_e572_d_n2, eq42_e572_d_n3, eq42_e572_d_n4, eq42_e572_d_n5, eq42_e572_d_n6, eq42_e572_d_n7, eq42_e572_d_n8, eq42_e572_d_n9, eq42_e572_d_n10, eq42_e572_d_n11, eq42_e572_d_n12, eq42_e572_d_n13, eq42_e572_d_n14, eq42_e572_d_n15, eq42_e572_d_n16, eq42_e572_d_n17, eq42_e572_d_n18];
        let eq42_branch_derivatives: [f64; 12] = [eq42_e572_d_b0, eq42_e572_d_b1, eq42_e572_d_b2, eq42_e572_d_b3, eq42_e572_d_b4, eq42_e572_d_b5, eq42_e572_d_b6, eq42_e572_d_b7, eq42_e572_d_b8, eq42_e572_d_b9, eq42_e572_d_b10, eq42_e572_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[18]),
            None,
            self.multiplicity * (eq42_value),
            &nodes,
            &eq42_node_derivatives,
            &branches,
            &eq42_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_43_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq43_e578, eq43_e578_d_n0, eq43_e578_d_n1, eq43_e578_d_n2, eq43_e578_d_n3, eq43_e578_d_n4, eq43_e578_d_n5, eq43_e578_d_n6, eq43_e578_d_n7, eq43_e578_d_n8, eq43_e578_d_n9, eq43_e578_d_n10, eq43_e578_d_n11, eq43_e578_d_n12, eq43_e578_d_n13, eq43_e578_d_n14, eq43_e578_d_n15, eq43_e578_d_n16, eq43_e578_d_n17, eq43_e578_d_n18, eq43_e578_d_b0, eq43_e578_d_b1, eq43_e578_d_b2, eq43_e578_d_b3, eq43_e578_d_b4, eq43_e578_d_b5, eq43_e578_d_b6, eq43_e578_d_b7, eq43_e578_d_b8, eq43_e578_d_b9, eq43_e578_d_b10, eq43_e578_d_b11,) = {
    if ((s.v[1847] != 0.0) && (p.p34 != 0.0)) {
        (s.v[583], s.dn[583][0], s.dn[583][1], s.dn[583][2], s.dn[583][3], s.dn[583][4], s.dn[583][5], s.dn[583][6], s.dn[583][7], s.dn[583][8], s.dn[583][9], s.dn[583][10], s.dn[583][11], s.dn[583][12], s.dn[583][13], s.dn[583][14], s.dn[583][15], s.dn[583][16], s.dn[583][17], s.dn[583][18], s.db[583][0], s.db[583][1], s.db[583][2], s.db[583][3], s.db[583][4], s.db[583][5], s.db[583][6], s.db[583][7], s.db[583][8], s.db[583][9], s.db[583][10], s.db[583][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq43_value: f64 = eq43_e578;
        let eq43_node_derivatives: [f64; 19] = [eq43_e578_d_n0, eq43_e578_d_n1, eq43_e578_d_n2, eq43_e578_d_n3, eq43_e578_d_n4, eq43_e578_d_n5, eq43_e578_d_n6, eq43_e578_d_n7, eq43_e578_d_n8, eq43_e578_d_n9, eq43_e578_d_n10, eq43_e578_d_n11, eq43_e578_d_n12, eq43_e578_d_n13, eq43_e578_d_n14, eq43_e578_d_n15, eq43_e578_d_n16, eq43_e578_d_n17, eq43_e578_d_n18];
        let eq43_branch_derivatives: [f64; 12] = [eq43_e578_d_b0, eq43_e578_d_b1, eq43_e578_d_b2, eq43_e578_d_b3, eq43_e578_d_b4, eq43_e578_d_b5, eq43_e578_d_b6, eq43_e578_d_b7, eq43_e578_d_b8, eq43_e578_d_b9, eq43_e578_d_b10, eq43_e578_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            None,
            self.multiplicity * (eq43_value),
            &nodes,
            &eq43_node_derivatives,
            &branches,
            &eq43_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_44_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv18 = ctx.node_voltage(nodes[18]);
        let (eq44_e586, eq44_e586_d_n18,) = {
    if ((s.v[1847] != 0.0) && (p.p34 != 0.0)) {
        let eq44_e584: f64 = ((nv18 - 0.0) * 1e-12);
        let eq44_e584_d_n18: f64 = 1e-12;
        (eq44_e584, eq44_e584_d_n18,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e586;
        stamper.stamp_current(
            Some(nodes[18]),
            None,
            self.multiplicity * (eq44_value),
            &[
                GeneratedDerivative::node(nodes[18], self.multiplicity * eq44_e586_d_n18),
            ],
        );
    }

    pub(super) fn stamp_transient_equation_45_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq45_e594, eq45_e594_d_n13,) = {
    if ((s.v[1847] != 0.0) && (p.p34 != 0.0)) {
        let eq45_e592: f64 = ((nv13 - 0.0) * 1e-12);
        let eq45_e592_d_n13: f64 = 1e-12;
        (eq45_e592, eq45_e592_d_n13,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq45_value: f64 = eq45_e594;
        stamper.stamp_current(
            Some(nodes[13]),
            None,
            self.multiplicity * (eq45_value),
            &[
                GeneratedDerivative::node(nodes[13], self.multiplicity * eq45_e594_d_n13),
            ],
        );
    }

    pub(super) fn stamp_transient_equation_46_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv18 = ctx.node_voltage(nodes[18]);
        let (eq46_e605, eq46_e605_d_n0, eq46_e605_d_n1, eq46_e605_d_n2, eq46_e605_d_n3, eq46_e605_d_n4, eq46_e605_d_n5, eq46_e605_d_n6, eq46_e605_d_n7, eq46_e605_d_n8, eq46_e605_d_n9, eq46_e605_d_n10, eq46_e605_d_n11, eq46_e605_d_n12, eq46_e605_d_n13, eq46_e605_d_n14, eq46_e605_d_n15, eq46_e605_d_n16, eq46_e605_d_n17, eq46_e605_d_n18, eq46_e605_d_b0, eq46_e605_d_b1, eq46_e605_d_b2, eq46_e605_d_b3, eq46_e605_d_b4, eq46_e605_d_b5, eq46_e605_d_b6, eq46_e605_d_b7, eq46_e605_d_b8, eq46_e605_d_b9, eq46_e605_d_b10, eq46_e605_d_b11,) = {
    if ((s.v[1847] != 0.0) && (p.p34 != 0.0)) {
        let eq46_e600: f64 = (1e-9 / 0.0001);
        let eq46_e602: f64 = (eq46_e600 * (nv18 - 0.0));
        let eq46_e603: f64 = self.eval_ddt(8, eq46_e602);
        let eq46_e603_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_n18: f64 = self.ddt_jacobian(eq46_e600);
        let eq46_e603_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_b4: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_b5: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_b6: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_b7: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_b8: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_b9: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_b10: f64 = self.ddt_jacobian(0.0);
        let eq46_e603_d_b11: f64 = self.ddt_jacobian(0.0);
        (eq46_e603, eq46_e603_d_n0, eq46_e603_d_n1, eq46_e603_d_n2, eq46_e603_d_n3, eq46_e603_d_n4, eq46_e603_d_n5, eq46_e603_d_n6, eq46_e603_d_n7, eq46_e603_d_n8, eq46_e603_d_n9, eq46_e603_d_n10, eq46_e603_d_n11, eq46_e603_d_n12, eq46_e603_d_n13, eq46_e603_d_n14, eq46_e603_d_n15, eq46_e603_d_n16, eq46_e603_d_n17, eq46_e603_d_n18, eq46_e603_d_b0, eq46_e603_d_b1, eq46_e603_d_b2, eq46_e603_d_b3, eq46_e603_d_b4, eq46_e603_d_b5, eq46_e603_d_b6, eq46_e603_d_b7, eq46_e603_d_b8, eq46_e603_d_b9, eq46_e603_d_b10, eq46_e603_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e605;
        let eq46_node_derivatives: [f64; 19] = [eq46_e605_d_n0, eq46_e605_d_n1, eq46_e605_d_n2, eq46_e605_d_n3, eq46_e605_d_n4, eq46_e605_d_n5, eq46_e605_d_n6, eq46_e605_d_n7, eq46_e605_d_n8, eq46_e605_d_n9, eq46_e605_d_n10, eq46_e605_d_n11, eq46_e605_d_n12, eq46_e605_d_n13, eq46_e605_d_n14, eq46_e605_d_n15, eq46_e605_d_n16, eq46_e605_d_n17, eq46_e605_d_n18];
        let eq46_branch_derivatives: [f64; 12] = [eq46_e605_d_b0, eq46_e605_d_b1, eq46_e605_d_b2, eq46_e605_d_b3, eq46_e605_d_b4, eq46_e605_d_b5, eq46_e605_d_b6, eq46_e605_d_b7, eq46_e605_d_b8, eq46_e605_d_b9, eq46_e605_d_b10, eq46_e605_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[18]),
            None,
            self.multiplicity * (eq46_value),
            &nodes,
            &eq46_node_derivatives,
            &branches,
            &eq46_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_47_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq47_e616, eq47_e616_d_n0, eq47_e616_d_n1, eq47_e616_d_n2, eq47_e616_d_n3, eq47_e616_d_n4, eq47_e616_d_n5, eq47_e616_d_n6, eq47_e616_d_n7, eq47_e616_d_n8, eq47_e616_d_n9, eq47_e616_d_n10, eq47_e616_d_n11, eq47_e616_d_n12, eq47_e616_d_n13, eq47_e616_d_n14, eq47_e616_d_n15, eq47_e616_d_n16, eq47_e616_d_n17, eq47_e616_d_n18, eq47_e616_d_b0, eq47_e616_d_b1, eq47_e616_d_b2, eq47_e616_d_b3, eq47_e616_d_b4, eq47_e616_d_b5, eq47_e616_d_b6, eq47_e616_d_b7, eq47_e616_d_b8, eq47_e616_d_b9, eq47_e616_d_b10, eq47_e616_d_b11,) = {
    if ((s.v[1847] != 0.0) && (p.p34 != 0.0)) {
        let eq47_e611: f64 = (1e-9 / 0.0001);
        let eq47_e613: f64 = (eq47_e611 * (nv13 - 0.0));
        let eq47_e614: f64 = self.eval_ddt(9, eq47_e613);
        let eq47_e614_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_n13: f64 = self.ddt_jacobian(eq47_e611);
        let eq47_e614_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_b4: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_b5: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_b6: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_b7: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_b8: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_b9: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_b10: f64 = self.ddt_jacobian(0.0);
        let eq47_e614_d_b11: f64 = self.ddt_jacobian(0.0);
        (eq47_e614, eq47_e614_d_n0, eq47_e614_d_n1, eq47_e614_d_n2, eq47_e614_d_n3, eq47_e614_d_n4, eq47_e614_d_n5, eq47_e614_d_n6, eq47_e614_d_n7, eq47_e614_d_n8, eq47_e614_d_n9, eq47_e614_d_n10, eq47_e614_d_n11, eq47_e614_d_n12, eq47_e614_d_n13, eq47_e614_d_n14, eq47_e614_d_n15, eq47_e614_d_n16, eq47_e614_d_n17, eq47_e614_d_n18, eq47_e614_d_b0, eq47_e614_d_b1, eq47_e614_d_b2, eq47_e614_d_b3, eq47_e614_d_b4, eq47_e614_d_b5, eq47_e614_d_b6, eq47_e614_d_b7, eq47_e614_d_b8, eq47_e614_d_b9, eq47_e614_d_b10, eq47_e614_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e616;
        let eq47_node_derivatives: [f64; 19] = [eq47_e616_d_n0, eq47_e616_d_n1, eq47_e616_d_n2, eq47_e616_d_n3, eq47_e616_d_n4, eq47_e616_d_n5, eq47_e616_d_n6, eq47_e616_d_n7, eq47_e616_d_n8, eq47_e616_d_n9, eq47_e616_d_n10, eq47_e616_d_n11, eq47_e616_d_n12, eq47_e616_d_n13, eq47_e616_d_n14, eq47_e616_d_n15, eq47_e616_d_n16, eq47_e616_d_n17, eq47_e616_d_n18];
        let eq47_branch_derivatives: [f64; 12] = [eq47_e616_d_b0, eq47_e616_d_b1, eq47_e616_d_b2, eq47_e616_d_b3, eq47_e616_d_b4, eq47_e616_d_b5, eq47_e616_d_b6, eq47_e616_d_b7, eq47_e616_d_b8, eq47_e616_d_b9, eq47_e616_d_b10, eq47_e616_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            None,
            self.multiplicity * (eq47_value),
            &nodes,
            &eq47_node_derivatives,
            &branches,
            &eq47_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_48_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq48_e623,) = {
    if ((s.v[1847] != 0.0) && (!(p.p34 != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq48_value: f64 = eq48_e623;
        stamper.stamp_potential(
            branches[8],
            eq48_value,
            &[
            ],
        );
    }
}
