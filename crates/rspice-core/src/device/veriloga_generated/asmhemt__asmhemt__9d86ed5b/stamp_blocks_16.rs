#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_199_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq199_e2499, eq199_e2499_d_n0, eq199_e2499_d_n1, eq199_e2499_d_n2, eq199_e2499_d_n3, eq199_e2499_d_n4, eq199_e2499_d_n5, eq199_e2499_d_n6, eq199_e2499_d_n7, eq199_e2499_d_n8, eq199_e2499_d_n9, eq199_e2499_d_n10, eq199_e2499_d_n11, eq199_e2499_d_n12, eq199_e2499_d_n13, eq199_e2499_d_n14, eq199_e2499_d_n15, eq199_e2499_d_n16, eq199_e2499_d_n17, eq199_e2499_d_n18, eq199_e2499_d_n19, eq199_e2499_d_n20, eq199_e2499_d_n21, eq199_e2499_d_n22,) = {
    if (((!(s.v[600] != 0.0)) && (s.v[603] != 0.0)) && (s.v[604] != 0.0)) {
        let eq199_e2496: f64 = self.eval_ddt(98, s.v[300]);
        let eq199_e2496_d_n0: f64 = self.ddt_jacobian(s.dn[300][0]);
        let eq199_e2496_d_n1: f64 = self.ddt_jacobian(s.dn[300][1]);
        let eq199_e2496_d_n2: f64 = self.ddt_jacobian(s.dn[300][2]);
        let eq199_e2496_d_n3: f64 = self.ddt_jacobian(s.dn[300][3]);
        let eq199_e2496_d_n4: f64 = self.ddt_jacobian(s.dn[300][4]);
        let eq199_e2496_d_n5: f64 = self.ddt_jacobian(s.dn[300][5]);
        let eq199_e2496_d_n6: f64 = self.ddt_jacobian(s.dn[300][6]);
        let eq199_e2496_d_n7: f64 = self.ddt_jacobian(s.dn[300][7]);
        let eq199_e2496_d_n8: f64 = self.ddt_jacobian(s.dn[300][8]);
        let eq199_e2496_d_n9: f64 = self.ddt_jacobian(s.dn[300][9]);
        let eq199_e2496_d_n10: f64 = self.ddt_jacobian(s.dn[300][10]);
        let eq199_e2496_d_n11: f64 = self.ddt_jacobian(s.dn[300][11]);
        let eq199_e2496_d_n12: f64 = self.ddt_jacobian(s.dn[300][12]);
        let eq199_e2496_d_n13: f64 = self.ddt_jacobian(s.dn[300][13]);
        let eq199_e2496_d_n14: f64 = self.ddt_jacobian(s.dn[300][14]);
        let eq199_e2496_d_n15: f64 = self.ddt_jacobian(s.dn[300][15]);
        let eq199_e2496_d_n16: f64 = self.ddt_jacobian(s.dn[300][16]);
        let eq199_e2496_d_n17: f64 = self.ddt_jacobian(s.dn[300][17]);
        let eq199_e2496_d_n18: f64 = self.ddt_jacobian(s.dn[300][18]);
        let eq199_e2496_d_n19: f64 = self.ddt_jacobian(s.dn[300][19]);
        let eq199_e2496_d_n20: f64 = self.ddt_jacobian(s.dn[300][20]);
        let eq199_e2496_d_n21: f64 = self.ddt_jacobian(s.dn[300][21]);
        let eq199_e2496_d_n22: f64 = self.ddt_jacobian(s.dn[300][22]);
        let eq199_e2497: f64 = (p.p7 * eq199_e2496);
        let eq199_e2497_d_n0: f64 = (p.p7 * eq199_e2496_d_n0);
        let eq199_e2497_d_n1: f64 = (p.p7 * eq199_e2496_d_n1);
        let eq199_e2497_d_n2: f64 = (p.p7 * eq199_e2496_d_n2);
        let eq199_e2497_d_n3: f64 = (p.p7 * eq199_e2496_d_n3);
        let eq199_e2497_d_n4: f64 = (p.p7 * eq199_e2496_d_n4);
        let eq199_e2497_d_n5: f64 = (p.p7 * eq199_e2496_d_n5);
        let eq199_e2497_d_n6: f64 = (p.p7 * eq199_e2496_d_n6);
        let eq199_e2497_d_n7: f64 = (p.p7 * eq199_e2496_d_n7);
        let eq199_e2497_d_n8: f64 = (p.p7 * eq199_e2496_d_n8);
        let eq199_e2497_d_n9: f64 = (p.p7 * eq199_e2496_d_n9);
        let eq199_e2497_d_n10: f64 = (p.p7 * eq199_e2496_d_n10);
        let eq199_e2497_d_n11: f64 = (p.p7 * eq199_e2496_d_n11);
        let eq199_e2497_d_n12: f64 = (p.p7 * eq199_e2496_d_n12);
        let eq199_e2497_d_n13: f64 = (p.p7 * eq199_e2496_d_n13);
        let eq199_e2497_d_n14: f64 = (p.p7 * eq199_e2496_d_n14);
        let eq199_e2497_d_n15: f64 = (p.p7 * eq199_e2496_d_n15);
        let eq199_e2497_d_n16: f64 = (p.p7 * eq199_e2496_d_n16);
        let eq199_e2497_d_n17: f64 = (p.p7 * eq199_e2496_d_n17);
        let eq199_e2497_d_n18: f64 = (p.p7 * eq199_e2496_d_n18);
        let eq199_e2497_d_n19: f64 = (p.p7 * eq199_e2496_d_n19);
        let eq199_e2497_d_n20: f64 = (p.p7 * eq199_e2496_d_n20);
        let eq199_e2497_d_n21: f64 = (p.p7 * eq199_e2496_d_n21);
        let eq199_e2497_d_n22: f64 = (p.p7 * eq199_e2496_d_n22);
        (eq199_e2497, eq199_e2497_d_n0, eq199_e2497_d_n1, eq199_e2497_d_n2, eq199_e2497_d_n3, eq199_e2497_d_n4, eq199_e2497_d_n5, eq199_e2497_d_n6, eq199_e2497_d_n7, eq199_e2497_d_n8, eq199_e2497_d_n9, eq199_e2497_d_n10, eq199_e2497_d_n11, eq199_e2497_d_n12, eq199_e2497_d_n13, eq199_e2497_d_n14, eq199_e2497_d_n15, eq199_e2497_d_n16, eq199_e2497_d_n17, eq199_e2497_d_n18, eq199_e2497_d_n19, eq199_e2497_d_n20, eq199_e2497_d_n21, eq199_e2497_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq199_value: f64 = eq199_e2499;
        let eq199_node_derivatives: [f64; 23] = [eq199_e2499_d_n0, eq199_e2499_d_n1, eq199_e2499_d_n2, eq199_e2499_d_n3, eq199_e2499_d_n4, eq199_e2499_d_n5, eq199_e2499_d_n6, eq199_e2499_d_n7, eq199_e2499_d_n8, eq199_e2499_d_n9, eq199_e2499_d_n10, eq199_e2499_d_n11, eq199_e2499_d_n12, eq199_e2499_d_n13, eq199_e2499_d_n14, eq199_e2499_d_n15, eq199_e2499_d_n16, eq199_e2499_d_n17, eq199_e2499_d_n18, eq199_e2499_d_n19, eq199_e2499_d_n20, eq199_e2499_d_n21, eq199_e2499_d_n22];
        let eq199_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq199_value),
            &nodes,
            &eq199_node_derivatives,
            &branches,
            &eq199_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_200_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq200_e2513, eq200_e2513_d_n0, eq200_e2513_d_n1, eq200_e2513_d_n2, eq200_e2513_d_n3, eq200_e2513_d_n4, eq200_e2513_d_n5, eq200_e2513_d_n6, eq200_e2513_d_n7, eq200_e2513_d_n8, eq200_e2513_d_n9, eq200_e2513_d_n10, eq200_e2513_d_n11, eq200_e2513_d_n12, eq200_e2513_d_n13, eq200_e2513_d_n14, eq200_e2513_d_n15, eq200_e2513_d_n16, eq200_e2513_d_n17, eq200_e2513_d_n18, eq200_e2513_d_n19, eq200_e2513_d_n20, eq200_e2513_d_n21, eq200_e2513_d_n22,) = {
    if (((!(s.v[600] != 0.0)) && (s.v[603] != 0.0)) && (s.v[604] != 0.0)) {
        let eq200_e2508: f64 = self.eval_ddt(99, s.v[300]);
        let eq200_e2508_d_n0: f64 = self.ddt_jacobian(s.dn[300][0]);
        let eq200_e2508_d_n1: f64 = self.ddt_jacobian(s.dn[300][1]);
        let eq200_e2508_d_n2: f64 = self.ddt_jacobian(s.dn[300][2]);
        let eq200_e2508_d_n3: f64 = self.ddt_jacobian(s.dn[300][3]);
        let eq200_e2508_d_n4: f64 = self.ddt_jacobian(s.dn[300][4]);
        let eq200_e2508_d_n5: f64 = self.ddt_jacobian(s.dn[300][5]);
        let eq200_e2508_d_n6: f64 = self.ddt_jacobian(s.dn[300][6]);
        let eq200_e2508_d_n7: f64 = self.ddt_jacobian(s.dn[300][7]);
        let eq200_e2508_d_n8: f64 = self.ddt_jacobian(s.dn[300][8]);
        let eq200_e2508_d_n9: f64 = self.ddt_jacobian(s.dn[300][9]);
        let eq200_e2508_d_n10: f64 = self.ddt_jacobian(s.dn[300][10]);
        let eq200_e2508_d_n11: f64 = self.ddt_jacobian(s.dn[300][11]);
        let eq200_e2508_d_n12: f64 = self.ddt_jacobian(s.dn[300][12]);
        let eq200_e2508_d_n13: f64 = self.ddt_jacobian(s.dn[300][13]);
        let eq200_e2508_d_n14: f64 = self.ddt_jacobian(s.dn[300][14]);
        let eq200_e2508_d_n15: f64 = self.ddt_jacobian(s.dn[300][15]);
        let eq200_e2508_d_n16: f64 = self.ddt_jacobian(s.dn[300][16]);
        let eq200_e2508_d_n17: f64 = self.ddt_jacobian(s.dn[300][17]);
        let eq200_e2508_d_n18: f64 = self.ddt_jacobian(s.dn[300][18]);
        let eq200_e2508_d_n19: f64 = self.ddt_jacobian(s.dn[300][19]);
        let eq200_e2508_d_n20: f64 = self.ddt_jacobian(s.dn[300][20]);
        let eq200_e2508_d_n21: f64 = self.ddt_jacobian(s.dn[300][21]);
        let eq200_e2508_d_n22: f64 = self.ddt_jacobian(s.dn[300][22]);
        let eq200_e2509: f64 = (p.p7 * eq200_e2508);
        let eq200_e2509_d_n0: f64 = (p.p7 * eq200_e2508_d_n0);
        let eq200_e2509_d_n1: f64 = (p.p7 * eq200_e2508_d_n1);
        let eq200_e2509_d_n2: f64 = (p.p7 * eq200_e2508_d_n2);
        let eq200_e2509_d_n3: f64 = (p.p7 * eq200_e2508_d_n3);
        let eq200_e2509_d_n4: f64 = (p.p7 * eq200_e2508_d_n4);
        let eq200_e2509_d_n5: f64 = (p.p7 * eq200_e2508_d_n5);
        let eq200_e2509_d_n6: f64 = (p.p7 * eq200_e2508_d_n6);
        let eq200_e2509_d_n7: f64 = (p.p7 * eq200_e2508_d_n7);
        let eq200_e2509_d_n8: f64 = (p.p7 * eq200_e2508_d_n8);
        let eq200_e2509_d_n9: f64 = (p.p7 * eq200_e2508_d_n9);
        let eq200_e2509_d_n10: f64 = (p.p7 * eq200_e2508_d_n10);
        let eq200_e2509_d_n11: f64 = (p.p7 * eq200_e2508_d_n11);
        let eq200_e2509_d_n12: f64 = (p.p7 * eq200_e2508_d_n12);
        let eq200_e2509_d_n13: f64 = (p.p7 * eq200_e2508_d_n13);
        let eq200_e2509_d_n14: f64 = (p.p7 * eq200_e2508_d_n14);
        let eq200_e2509_d_n15: f64 = (p.p7 * eq200_e2508_d_n15);
        let eq200_e2509_d_n16: f64 = (p.p7 * eq200_e2508_d_n16);
        let eq200_e2509_d_n17: f64 = (p.p7 * eq200_e2508_d_n17);
        let eq200_e2509_d_n18: f64 = (p.p7 * eq200_e2508_d_n18);
        let eq200_e2509_d_n19: f64 = (p.p7 * eq200_e2508_d_n19);
        let eq200_e2509_d_n20: f64 = (p.p7 * eq200_e2508_d_n20);
        let eq200_e2509_d_n21: f64 = (p.p7 * eq200_e2508_d_n21);
        let eq200_e2509_d_n22: f64 = (p.p7 * eq200_e2508_d_n22);
        let eq200_e2511: f64 = (eq200_e2509 * p.p249);
        let eq200_e2511_d_n0: f64 = (eq200_e2509_d_n0 * p.p249);
        let eq200_e2511_d_n1: f64 = (eq200_e2509_d_n1 * p.p249);
        let eq200_e2511_d_n2: f64 = (eq200_e2509_d_n2 * p.p249);
        let eq200_e2511_d_n3: f64 = (eq200_e2509_d_n3 * p.p249);
        let eq200_e2511_d_n4: f64 = (eq200_e2509_d_n4 * p.p249);
        let eq200_e2511_d_n5: f64 = (eq200_e2509_d_n5 * p.p249);
        let eq200_e2511_d_n6: f64 = (eq200_e2509_d_n6 * p.p249);
        let eq200_e2511_d_n7: f64 = (eq200_e2509_d_n7 * p.p249);
        let eq200_e2511_d_n8: f64 = (eq200_e2509_d_n8 * p.p249);
        let eq200_e2511_d_n9: f64 = (eq200_e2509_d_n9 * p.p249);
        let eq200_e2511_d_n10: f64 = (eq200_e2509_d_n10 * p.p249);
        let eq200_e2511_d_n11: f64 = (eq200_e2509_d_n11 * p.p249);
        let eq200_e2511_d_n12: f64 = (eq200_e2509_d_n12 * p.p249);
        let eq200_e2511_d_n13: f64 = (eq200_e2509_d_n13 * p.p249);
        let eq200_e2511_d_n14: f64 = (eq200_e2509_d_n14 * p.p249);
        let eq200_e2511_d_n15: f64 = (eq200_e2509_d_n15 * p.p249);
        let eq200_e2511_d_n16: f64 = (eq200_e2509_d_n16 * p.p249);
        let eq200_e2511_d_n17: f64 = (eq200_e2509_d_n17 * p.p249);
        let eq200_e2511_d_n18: f64 = (eq200_e2509_d_n18 * p.p249);
        let eq200_e2511_d_n19: f64 = (eq200_e2509_d_n19 * p.p249);
        let eq200_e2511_d_n20: f64 = (eq200_e2509_d_n20 * p.p249);
        let eq200_e2511_d_n21: f64 = (eq200_e2509_d_n21 * p.p249);
        let eq200_e2511_d_n22: f64 = (eq200_e2509_d_n22 * p.p249);
        (eq200_e2511, eq200_e2511_d_n0, eq200_e2511_d_n1, eq200_e2511_d_n2, eq200_e2511_d_n3, eq200_e2511_d_n4, eq200_e2511_d_n5, eq200_e2511_d_n6, eq200_e2511_d_n7, eq200_e2511_d_n8, eq200_e2511_d_n9, eq200_e2511_d_n10, eq200_e2511_d_n11, eq200_e2511_d_n12, eq200_e2511_d_n13, eq200_e2511_d_n14, eq200_e2511_d_n15, eq200_e2511_d_n16, eq200_e2511_d_n17, eq200_e2511_d_n18, eq200_e2511_d_n19, eq200_e2511_d_n20, eq200_e2511_d_n21, eq200_e2511_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq200_value: f64 = eq200_e2513;
        let eq200_node_derivatives: [f64; 23] = [eq200_e2513_d_n0, eq200_e2513_d_n1, eq200_e2513_d_n2, eq200_e2513_d_n3, eq200_e2513_d_n4, eq200_e2513_d_n5, eq200_e2513_d_n6, eq200_e2513_d_n7, eq200_e2513_d_n8, eq200_e2513_d_n9, eq200_e2513_d_n10, eq200_e2513_d_n11, eq200_e2513_d_n12, eq200_e2513_d_n13, eq200_e2513_d_n14, eq200_e2513_d_n15, eq200_e2513_d_n16, eq200_e2513_d_n17, eq200_e2513_d_n18, eq200_e2513_d_n19, eq200_e2513_d_n20, eq200_e2513_d_n21, eq200_e2513_d_n22];
        let eq200_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            self.multiplicity * (eq200_value),
            &nodes,
            &eq200_node_derivatives,
            &branches,
            &eq200_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_201_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq201_e2526, eq201_e2526_d_n0, eq201_e2526_d_n1, eq201_e2526_d_n2, eq201_e2526_d_n3, eq201_e2526_d_n4, eq201_e2526_d_n5, eq201_e2526_d_n6, eq201_e2526_d_n7, eq201_e2526_d_n8, eq201_e2526_d_n9, eq201_e2526_d_n10, eq201_e2526_d_n11, eq201_e2526_d_n12, eq201_e2526_d_n13, eq201_e2526_d_n14, eq201_e2526_d_n15, eq201_e2526_d_n16, eq201_e2526_d_n17, eq201_e2526_d_n18, eq201_e2526_d_n19, eq201_e2526_d_n20, eq201_e2526_d_n21, eq201_e2526_d_n22,) = {
    if (((!(s.v[600] != 0.0)) && (s.v[603] != 0.0)) && (!(s.v[604] != 0.0))) {
        let eq201_e2523: f64 = self.eval_ddt(100, s.v[300]);
        let eq201_e2523_d_n0: f64 = self.ddt_jacobian(s.dn[300][0]);
        let eq201_e2523_d_n1: f64 = self.ddt_jacobian(s.dn[300][1]);
        let eq201_e2523_d_n2: f64 = self.ddt_jacobian(s.dn[300][2]);
        let eq201_e2523_d_n3: f64 = self.ddt_jacobian(s.dn[300][3]);
        let eq201_e2523_d_n4: f64 = self.ddt_jacobian(s.dn[300][4]);
        let eq201_e2523_d_n5: f64 = self.ddt_jacobian(s.dn[300][5]);
        let eq201_e2523_d_n6: f64 = self.ddt_jacobian(s.dn[300][6]);
        let eq201_e2523_d_n7: f64 = self.ddt_jacobian(s.dn[300][7]);
        let eq201_e2523_d_n8: f64 = self.ddt_jacobian(s.dn[300][8]);
        let eq201_e2523_d_n9: f64 = self.ddt_jacobian(s.dn[300][9]);
        let eq201_e2523_d_n10: f64 = self.ddt_jacobian(s.dn[300][10]);
        let eq201_e2523_d_n11: f64 = self.ddt_jacobian(s.dn[300][11]);
        let eq201_e2523_d_n12: f64 = self.ddt_jacobian(s.dn[300][12]);
        let eq201_e2523_d_n13: f64 = self.ddt_jacobian(s.dn[300][13]);
        let eq201_e2523_d_n14: f64 = self.ddt_jacobian(s.dn[300][14]);
        let eq201_e2523_d_n15: f64 = self.ddt_jacobian(s.dn[300][15]);
        let eq201_e2523_d_n16: f64 = self.ddt_jacobian(s.dn[300][16]);
        let eq201_e2523_d_n17: f64 = self.ddt_jacobian(s.dn[300][17]);
        let eq201_e2523_d_n18: f64 = self.ddt_jacobian(s.dn[300][18]);
        let eq201_e2523_d_n19: f64 = self.ddt_jacobian(s.dn[300][19]);
        let eq201_e2523_d_n20: f64 = self.ddt_jacobian(s.dn[300][20]);
        let eq201_e2523_d_n21: f64 = self.ddt_jacobian(s.dn[300][21]);
        let eq201_e2523_d_n22: f64 = self.ddt_jacobian(s.dn[300][22]);
        let eq201_e2524: f64 = (p.p7 * eq201_e2523);
        let eq201_e2524_d_n0: f64 = (p.p7 * eq201_e2523_d_n0);
        let eq201_e2524_d_n1: f64 = (p.p7 * eq201_e2523_d_n1);
        let eq201_e2524_d_n2: f64 = (p.p7 * eq201_e2523_d_n2);
        let eq201_e2524_d_n3: f64 = (p.p7 * eq201_e2523_d_n3);
        let eq201_e2524_d_n4: f64 = (p.p7 * eq201_e2523_d_n4);
        let eq201_e2524_d_n5: f64 = (p.p7 * eq201_e2523_d_n5);
        let eq201_e2524_d_n6: f64 = (p.p7 * eq201_e2523_d_n6);
        let eq201_e2524_d_n7: f64 = (p.p7 * eq201_e2523_d_n7);
        let eq201_e2524_d_n8: f64 = (p.p7 * eq201_e2523_d_n8);
        let eq201_e2524_d_n9: f64 = (p.p7 * eq201_e2523_d_n9);
        let eq201_e2524_d_n10: f64 = (p.p7 * eq201_e2523_d_n10);
        let eq201_e2524_d_n11: f64 = (p.p7 * eq201_e2523_d_n11);
        let eq201_e2524_d_n12: f64 = (p.p7 * eq201_e2523_d_n12);
        let eq201_e2524_d_n13: f64 = (p.p7 * eq201_e2523_d_n13);
        let eq201_e2524_d_n14: f64 = (p.p7 * eq201_e2523_d_n14);
        let eq201_e2524_d_n15: f64 = (p.p7 * eq201_e2523_d_n15);
        let eq201_e2524_d_n16: f64 = (p.p7 * eq201_e2523_d_n16);
        let eq201_e2524_d_n17: f64 = (p.p7 * eq201_e2523_d_n17);
        let eq201_e2524_d_n18: f64 = (p.p7 * eq201_e2523_d_n18);
        let eq201_e2524_d_n19: f64 = (p.p7 * eq201_e2523_d_n19);
        let eq201_e2524_d_n20: f64 = (p.p7 * eq201_e2523_d_n20);
        let eq201_e2524_d_n21: f64 = (p.p7 * eq201_e2523_d_n21);
        let eq201_e2524_d_n22: f64 = (p.p7 * eq201_e2523_d_n22);
        (eq201_e2524, eq201_e2524_d_n0, eq201_e2524_d_n1, eq201_e2524_d_n2, eq201_e2524_d_n3, eq201_e2524_d_n4, eq201_e2524_d_n5, eq201_e2524_d_n6, eq201_e2524_d_n7, eq201_e2524_d_n8, eq201_e2524_d_n9, eq201_e2524_d_n10, eq201_e2524_d_n11, eq201_e2524_d_n12, eq201_e2524_d_n13, eq201_e2524_d_n14, eq201_e2524_d_n15, eq201_e2524_d_n16, eq201_e2524_d_n17, eq201_e2524_d_n18, eq201_e2524_d_n19, eq201_e2524_d_n20, eq201_e2524_d_n21, eq201_e2524_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq201_value: f64 = eq201_e2526;
        let eq201_node_derivatives: [f64; 23] = [eq201_e2526_d_n0, eq201_e2526_d_n1, eq201_e2526_d_n2, eq201_e2526_d_n3, eq201_e2526_d_n4, eq201_e2526_d_n5, eq201_e2526_d_n6, eq201_e2526_d_n7, eq201_e2526_d_n8, eq201_e2526_d_n9, eq201_e2526_d_n10, eq201_e2526_d_n11, eq201_e2526_d_n12, eq201_e2526_d_n13, eq201_e2526_d_n14, eq201_e2526_d_n15, eq201_e2526_d_n16, eq201_e2526_d_n17, eq201_e2526_d_n18, eq201_e2526_d_n19, eq201_e2526_d_n20, eq201_e2526_d_n21, eq201_e2526_d_n22];
        let eq201_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            self.multiplicity * (eq201_value),
            &nodes,
            &eq201_node_derivatives,
            &branches,
            &eq201_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_202_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq202_e2541, eq202_e2541_d_n0, eq202_e2541_d_n1, eq202_e2541_d_n2, eq202_e2541_d_n3, eq202_e2541_d_n4, eq202_e2541_d_n5, eq202_e2541_d_n6, eq202_e2541_d_n7, eq202_e2541_d_n8, eq202_e2541_d_n9, eq202_e2541_d_n10, eq202_e2541_d_n11, eq202_e2541_d_n12, eq202_e2541_d_n13, eq202_e2541_d_n14, eq202_e2541_d_n15, eq202_e2541_d_n16, eq202_e2541_d_n17, eq202_e2541_d_n18, eq202_e2541_d_n19, eq202_e2541_d_n20, eq202_e2541_d_n21, eq202_e2541_d_n22,) = {
    if (((!(s.v[600] != 0.0)) && (s.v[603] != 0.0)) && (!(s.v[604] != 0.0))) {
        let eq202_e2536: f64 = self.eval_ddt(101, s.v[300]);
        let eq202_e2536_d_n0: f64 = self.ddt_jacobian(s.dn[300][0]);
        let eq202_e2536_d_n1: f64 = self.ddt_jacobian(s.dn[300][1]);
        let eq202_e2536_d_n2: f64 = self.ddt_jacobian(s.dn[300][2]);
        let eq202_e2536_d_n3: f64 = self.ddt_jacobian(s.dn[300][3]);
        let eq202_e2536_d_n4: f64 = self.ddt_jacobian(s.dn[300][4]);
        let eq202_e2536_d_n5: f64 = self.ddt_jacobian(s.dn[300][5]);
        let eq202_e2536_d_n6: f64 = self.ddt_jacobian(s.dn[300][6]);
        let eq202_e2536_d_n7: f64 = self.ddt_jacobian(s.dn[300][7]);
        let eq202_e2536_d_n8: f64 = self.ddt_jacobian(s.dn[300][8]);
        let eq202_e2536_d_n9: f64 = self.ddt_jacobian(s.dn[300][9]);
        let eq202_e2536_d_n10: f64 = self.ddt_jacobian(s.dn[300][10]);
        let eq202_e2536_d_n11: f64 = self.ddt_jacobian(s.dn[300][11]);
        let eq202_e2536_d_n12: f64 = self.ddt_jacobian(s.dn[300][12]);
        let eq202_e2536_d_n13: f64 = self.ddt_jacobian(s.dn[300][13]);
        let eq202_e2536_d_n14: f64 = self.ddt_jacobian(s.dn[300][14]);
        let eq202_e2536_d_n15: f64 = self.ddt_jacobian(s.dn[300][15]);
        let eq202_e2536_d_n16: f64 = self.ddt_jacobian(s.dn[300][16]);
        let eq202_e2536_d_n17: f64 = self.ddt_jacobian(s.dn[300][17]);
        let eq202_e2536_d_n18: f64 = self.ddt_jacobian(s.dn[300][18]);
        let eq202_e2536_d_n19: f64 = self.ddt_jacobian(s.dn[300][19]);
        let eq202_e2536_d_n20: f64 = self.ddt_jacobian(s.dn[300][20]);
        let eq202_e2536_d_n21: f64 = self.ddt_jacobian(s.dn[300][21]);
        let eq202_e2536_d_n22: f64 = self.ddt_jacobian(s.dn[300][22]);
        let eq202_e2537: f64 = (p.p7 * eq202_e2536);
        let eq202_e2537_d_n0: f64 = (p.p7 * eq202_e2536_d_n0);
        let eq202_e2537_d_n1: f64 = (p.p7 * eq202_e2536_d_n1);
        let eq202_e2537_d_n2: f64 = (p.p7 * eq202_e2536_d_n2);
        let eq202_e2537_d_n3: f64 = (p.p7 * eq202_e2536_d_n3);
        let eq202_e2537_d_n4: f64 = (p.p7 * eq202_e2536_d_n4);
        let eq202_e2537_d_n5: f64 = (p.p7 * eq202_e2536_d_n5);
        let eq202_e2537_d_n6: f64 = (p.p7 * eq202_e2536_d_n6);
        let eq202_e2537_d_n7: f64 = (p.p7 * eq202_e2536_d_n7);
        let eq202_e2537_d_n8: f64 = (p.p7 * eq202_e2536_d_n8);
        let eq202_e2537_d_n9: f64 = (p.p7 * eq202_e2536_d_n9);
        let eq202_e2537_d_n10: f64 = (p.p7 * eq202_e2536_d_n10);
        let eq202_e2537_d_n11: f64 = (p.p7 * eq202_e2536_d_n11);
        let eq202_e2537_d_n12: f64 = (p.p7 * eq202_e2536_d_n12);
        let eq202_e2537_d_n13: f64 = (p.p7 * eq202_e2536_d_n13);
        let eq202_e2537_d_n14: f64 = (p.p7 * eq202_e2536_d_n14);
        let eq202_e2537_d_n15: f64 = (p.p7 * eq202_e2536_d_n15);
        let eq202_e2537_d_n16: f64 = (p.p7 * eq202_e2536_d_n16);
        let eq202_e2537_d_n17: f64 = (p.p7 * eq202_e2536_d_n17);
        let eq202_e2537_d_n18: f64 = (p.p7 * eq202_e2536_d_n18);
        let eq202_e2537_d_n19: f64 = (p.p7 * eq202_e2536_d_n19);
        let eq202_e2537_d_n20: f64 = (p.p7 * eq202_e2536_d_n20);
        let eq202_e2537_d_n21: f64 = (p.p7 * eq202_e2536_d_n21);
        let eq202_e2537_d_n22: f64 = (p.p7 * eq202_e2536_d_n22);
        let eq202_e2539: f64 = (eq202_e2537 * p.p249);
        let eq202_e2539_d_n0: f64 = (eq202_e2537_d_n0 * p.p249);
        let eq202_e2539_d_n1: f64 = (eq202_e2537_d_n1 * p.p249);
        let eq202_e2539_d_n2: f64 = (eq202_e2537_d_n2 * p.p249);
        let eq202_e2539_d_n3: f64 = (eq202_e2537_d_n3 * p.p249);
        let eq202_e2539_d_n4: f64 = (eq202_e2537_d_n4 * p.p249);
        let eq202_e2539_d_n5: f64 = (eq202_e2537_d_n5 * p.p249);
        let eq202_e2539_d_n6: f64 = (eq202_e2537_d_n6 * p.p249);
        let eq202_e2539_d_n7: f64 = (eq202_e2537_d_n7 * p.p249);
        let eq202_e2539_d_n8: f64 = (eq202_e2537_d_n8 * p.p249);
        let eq202_e2539_d_n9: f64 = (eq202_e2537_d_n9 * p.p249);
        let eq202_e2539_d_n10: f64 = (eq202_e2537_d_n10 * p.p249);
        let eq202_e2539_d_n11: f64 = (eq202_e2537_d_n11 * p.p249);
        let eq202_e2539_d_n12: f64 = (eq202_e2537_d_n12 * p.p249);
        let eq202_e2539_d_n13: f64 = (eq202_e2537_d_n13 * p.p249);
        let eq202_e2539_d_n14: f64 = (eq202_e2537_d_n14 * p.p249);
        let eq202_e2539_d_n15: f64 = (eq202_e2537_d_n15 * p.p249);
        let eq202_e2539_d_n16: f64 = (eq202_e2537_d_n16 * p.p249);
        let eq202_e2539_d_n17: f64 = (eq202_e2537_d_n17 * p.p249);
        let eq202_e2539_d_n18: f64 = (eq202_e2537_d_n18 * p.p249);
        let eq202_e2539_d_n19: f64 = (eq202_e2537_d_n19 * p.p249);
        let eq202_e2539_d_n20: f64 = (eq202_e2537_d_n20 * p.p249);
        let eq202_e2539_d_n21: f64 = (eq202_e2537_d_n21 * p.p249);
        let eq202_e2539_d_n22: f64 = (eq202_e2537_d_n22 * p.p249);
        (eq202_e2539, eq202_e2539_d_n0, eq202_e2539_d_n1, eq202_e2539_d_n2, eq202_e2539_d_n3, eq202_e2539_d_n4, eq202_e2539_d_n5, eq202_e2539_d_n6, eq202_e2539_d_n7, eq202_e2539_d_n8, eq202_e2539_d_n9, eq202_e2539_d_n10, eq202_e2539_d_n11, eq202_e2539_d_n12, eq202_e2539_d_n13, eq202_e2539_d_n14, eq202_e2539_d_n15, eq202_e2539_d_n16, eq202_e2539_d_n17, eq202_e2539_d_n18, eq202_e2539_d_n19, eq202_e2539_d_n20, eq202_e2539_d_n21, eq202_e2539_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq202_value: f64 = eq202_e2541;
        let eq202_node_derivatives: [f64; 23] = [eq202_e2541_d_n0, eq202_e2541_d_n1, eq202_e2541_d_n2, eq202_e2541_d_n3, eq202_e2541_d_n4, eq202_e2541_d_n5, eq202_e2541_d_n6, eq202_e2541_d_n7, eq202_e2541_d_n8, eq202_e2541_d_n9, eq202_e2541_d_n10, eq202_e2541_d_n11, eq202_e2541_d_n12, eq202_e2541_d_n13, eq202_e2541_d_n14, eq202_e2541_d_n15, eq202_e2541_d_n16, eq202_e2541_d_n17, eq202_e2541_d_n18, eq202_e2541_d_n19, eq202_e2541_d_n20, eq202_e2541_d_n21, eq202_e2541_d_n22];
        let eq202_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq202_value),
            &nodes,
            &eq202_node_derivatives,
            &branches,
            &eq202_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_203_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq203_e2553, eq203_e2553_d_n0, eq203_e2553_d_n1, eq203_e2553_d_n2, eq203_e2553_d_n3, eq203_e2553_d_n4, eq203_e2553_d_n5, eq203_e2553_d_n6, eq203_e2553_d_n7, eq203_e2553_d_n8, eq203_e2553_d_n9, eq203_e2553_d_n10, eq203_e2553_d_n11, eq203_e2553_d_n12, eq203_e2553_d_n13, eq203_e2553_d_n14, eq203_e2553_d_n15, eq203_e2553_d_n16, eq203_e2553_d_n17, eq203_e2553_d_n18, eq203_e2553_d_n19, eq203_e2553_d_n20, eq203_e2553_d_n21, eq203_e2553_d_n22,) = {
    if ((!(s.v[600] != 0.0)) && (s.v[603] != 0.0)) {
        let eq203_e2549: f64 = (p.p254 * s.v[300]);
        let eq203_e2549_d_n0: f64 = (p.p254 * s.dn[300][0]);
        let eq203_e2549_d_n1: f64 = (p.p254 * s.dn[300][1]);
        let eq203_e2549_d_n2: f64 = (p.p254 * s.dn[300][2]);
        let eq203_e2549_d_n3: f64 = (p.p254 * s.dn[300][3]);
        let eq203_e2549_d_n4: f64 = (p.p254 * s.dn[300][4]);
        let eq203_e2549_d_n5: f64 = (p.p254 * s.dn[300][5]);
        let eq203_e2549_d_n6: f64 = (p.p254 * s.dn[300][6]);
        let eq203_e2549_d_n7: f64 = (p.p254 * s.dn[300][7]);
        let eq203_e2549_d_n8: f64 = (p.p254 * s.dn[300][8]);
        let eq203_e2549_d_n9: f64 = (p.p254 * s.dn[300][9]);
        let eq203_e2549_d_n10: f64 = (p.p254 * s.dn[300][10]);
        let eq203_e2549_d_n11: f64 = (p.p254 * s.dn[300][11]);
        let eq203_e2549_d_n12: f64 = (p.p254 * s.dn[300][12]);
        let eq203_e2549_d_n13: f64 = (p.p254 * s.dn[300][13]);
        let eq203_e2549_d_n14: f64 = (p.p254 * s.dn[300][14]);
        let eq203_e2549_d_n15: f64 = (p.p254 * s.dn[300][15]);
        let eq203_e2549_d_n16: f64 = (p.p254 * s.dn[300][16]);
        let eq203_e2549_d_n17: f64 = (p.p254 * s.dn[300][17]);
        let eq203_e2549_d_n18: f64 = (p.p254 * s.dn[300][18]);
        let eq203_e2549_d_n19: f64 = (p.p254 * s.dn[300][19]);
        let eq203_e2549_d_n20: f64 = (p.p254 * s.dn[300][20]);
        let eq203_e2549_d_n21: f64 = (p.p254 * s.dn[300][21]);
        let eq203_e2549_d_n22: f64 = (p.p254 * s.dn[300][22]);
        let eq203_e2550: f64 = self.eval_ddt(102, eq203_e2549);
        let eq203_e2550_d_n0: f64 = self.ddt_jacobian(eq203_e2549_d_n0);
        let eq203_e2550_d_n1: f64 = self.ddt_jacobian(eq203_e2549_d_n1);
        let eq203_e2550_d_n2: f64 = self.ddt_jacobian(eq203_e2549_d_n2);
        let eq203_e2550_d_n3: f64 = self.ddt_jacobian(eq203_e2549_d_n3);
        let eq203_e2550_d_n4: f64 = self.ddt_jacobian(eq203_e2549_d_n4);
        let eq203_e2550_d_n5: f64 = self.ddt_jacobian(eq203_e2549_d_n5);
        let eq203_e2550_d_n6: f64 = self.ddt_jacobian(eq203_e2549_d_n6);
        let eq203_e2550_d_n7: f64 = self.ddt_jacobian(eq203_e2549_d_n7);
        let eq203_e2550_d_n8: f64 = self.ddt_jacobian(eq203_e2549_d_n8);
        let eq203_e2550_d_n9: f64 = self.ddt_jacobian(eq203_e2549_d_n9);
        let eq203_e2550_d_n10: f64 = self.ddt_jacobian(eq203_e2549_d_n10);
        let eq203_e2550_d_n11: f64 = self.ddt_jacobian(eq203_e2549_d_n11);
        let eq203_e2550_d_n12: f64 = self.ddt_jacobian(eq203_e2549_d_n12);
        let eq203_e2550_d_n13: f64 = self.ddt_jacobian(eq203_e2549_d_n13);
        let eq203_e2550_d_n14: f64 = self.ddt_jacobian(eq203_e2549_d_n14);
        let eq203_e2550_d_n15: f64 = self.ddt_jacobian(eq203_e2549_d_n15);
        let eq203_e2550_d_n16: f64 = self.ddt_jacobian(eq203_e2549_d_n16);
        let eq203_e2550_d_n17: f64 = self.ddt_jacobian(eq203_e2549_d_n17);
        let eq203_e2550_d_n18: f64 = self.ddt_jacobian(eq203_e2549_d_n18);
        let eq203_e2550_d_n19: f64 = self.ddt_jacobian(eq203_e2549_d_n19);
        let eq203_e2550_d_n20: f64 = self.ddt_jacobian(eq203_e2549_d_n20);
        let eq203_e2550_d_n21: f64 = self.ddt_jacobian(eq203_e2549_d_n21);
        let eq203_e2550_d_n22: f64 = self.ddt_jacobian(eq203_e2549_d_n22);
        let eq203_e2551: f64 = (p.p7 * eq203_e2550);
        let eq203_e2551_d_n0: f64 = (p.p7 * eq203_e2550_d_n0);
        let eq203_e2551_d_n1: f64 = (p.p7 * eq203_e2550_d_n1);
        let eq203_e2551_d_n2: f64 = (p.p7 * eq203_e2550_d_n2);
        let eq203_e2551_d_n3: f64 = (p.p7 * eq203_e2550_d_n3);
        let eq203_e2551_d_n4: f64 = (p.p7 * eq203_e2550_d_n4);
        let eq203_e2551_d_n5: f64 = (p.p7 * eq203_e2550_d_n5);
        let eq203_e2551_d_n6: f64 = (p.p7 * eq203_e2550_d_n6);
        let eq203_e2551_d_n7: f64 = (p.p7 * eq203_e2550_d_n7);
        let eq203_e2551_d_n8: f64 = (p.p7 * eq203_e2550_d_n8);
        let eq203_e2551_d_n9: f64 = (p.p7 * eq203_e2550_d_n9);
        let eq203_e2551_d_n10: f64 = (p.p7 * eq203_e2550_d_n10);
        let eq203_e2551_d_n11: f64 = (p.p7 * eq203_e2550_d_n11);
        let eq203_e2551_d_n12: f64 = (p.p7 * eq203_e2550_d_n12);
        let eq203_e2551_d_n13: f64 = (p.p7 * eq203_e2550_d_n13);
        let eq203_e2551_d_n14: f64 = (p.p7 * eq203_e2550_d_n14);
        let eq203_e2551_d_n15: f64 = (p.p7 * eq203_e2550_d_n15);
        let eq203_e2551_d_n16: f64 = (p.p7 * eq203_e2550_d_n16);
        let eq203_e2551_d_n17: f64 = (p.p7 * eq203_e2550_d_n17);
        let eq203_e2551_d_n18: f64 = (p.p7 * eq203_e2550_d_n18);
        let eq203_e2551_d_n19: f64 = (p.p7 * eq203_e2550_d_n19);
        let eq203_e2551_d_n20: f64 = (p.p7 * eq203_e2550_d_n20);
        let eq203_e2551_d_n21: f64 = (p.p7 * eq203_e2550_d_n21);
        let eq203_e2551_d_n22: f64 = (p.p7 * eq203_e2550_d_n22);
        (eq203_e2551, eq203_e2551_d_n0, eq203_e2551_d_n1, eq203_e2551_d_n2, eq203_e2551_d_n3, eq203_e2551_d_n4, eq203_e2551_d_n5, eq203_e2551_d_n6, eq203_e2551_d_n7, eq203_e2551_d_n8, eq203_e2551_d_n9, eq203_e2551_d_n10, eq203_e2551_d_n11, eq203_e2551_d_n12, eq203_e2551_d_n13, eq203_e2551_d_n14, eq203_e2551_d_n15, eq203_e2551_d_n16, eq203_e2551_d_n17, eq203_e2551_d_n18, eq203_e2551_d_n19, eq203_e2551_d_n20, eq203_e2551_d_n21, eq203_e2551_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq203_value: f64 = eq203_e2553;
        let eq203_node_derivatives: [f64; 23] = [eq203_e2553_d_n0, eq203_e2553_d_n1, eq203_e2553_d_n2, eq203_e2553_d_n3, eq203_e2553_d_n4, eq203_e2553_d_n5, eq203_e2553_d_n6, eq203_e2553_d_n7, eq203_e2553_d_n8, eq203_e2553_d_n9, eq203_e2553_d_n10, eq203_e2553_d_n11, eq203_e2553_d_n12, eq203_e2553_d_n13, eq203_e2553_d_n14, eq203_e2553_d_n15, eq203_e2553_d_n16, eq203_e2553_d_n17, eq203_e2553_d_n18, eq203_e2553_d_n19, eq203_e2553_d_n20, eq203_e2553_d_n21, eq203_e2553_d_n22];
        let eq203_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            self.multiplicity * (eq203_value),
            &nodes,
            &eq203_node_derivatives,
            &branches,
            &eq203_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_204_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq204_e2562, eq204_e2562_d_n0, eq204_e2562_d_n1, eq204_e2562_d_n2, eq204_e2562_d_n3, eq204_e2562_d_n4, eq204_e2562_d_n5, eq204_e2562_d_n6, eq204_e2562_d_n7, eq204_e2562_d_n8, eq204_e2562_d_n9, eq204_e2562_d_n10, eq204_e2562_d_n11, eq204_e2562_d_n12, eq204_e2562_d_n13, eq204_e2562_d_n14, eq204_e2562_d_n15, eq204_e2562_d_n16, eq204_e2562_d_n17, eq204_e2562_d_n18, eq204_e2562_d_n19, eq204_e2562_d_n20, eq204_e2562_d_n21, eq204_e2562_d_n22,) = {
    if ((s.v[605] != 0.0) && (s.v[606] != 0.0)) {
        let eq204_e2559: f64 = self.eval_ddt(103, s.v[313]);
        let eq204_e2559_d_n0: f64 = self.ddt_jacobian(s.dn[313][0]);
        let eq204_e2559_d_n1: f64 = self.ddt_jacobian(s.dn[313][1]);
        let eq204_e2559_d_n2: f64 = self.ddt_jacobian(s.dn[313][2]);
        let eq204_e2559_d_n3: f64 = self.ddt_jacobian(s.dn[313][3]);
        let eq204_e2559_d_n4: f64 = self.ddt_jacobian(s.dn[313][4]);
        let eq204_e2559_d_n5: f64 = self.ddt_jacobian(s.dn[313][5]);
        let eq204_e2559_d_n6: f64 = self.ddt_jacobian(s.dn[313][6]);
        let eq204_e2559_d_n7: f64 = self.ddt_jacobian(s.dn[313][7]);
        let eq204_e2559_d_n8: f64 = self.ddt_jacobian(s.dn[313][8]);
        let eq204_e2559_d_n9: f64 = self.ddt_jacobian(s.dn[313][9]);
        let eq204_e2559_d_n10: f64 = self.ddt_jacobian(s.dn[313][10]);
        let eq204_e2559_d_n11: f64 = self.ddt_jacobian(s.dn[313][11]);
        let eq204_e2559_d_n12: f64 = self.ddt_jacobian(s.dn[313][12]);
        let eq204_e2559_d_n13: f64 = self.ddt_jacobian(s.dn[313][13]);
        let eq204_e2559_d_n14: f64 = self.ddt_jacobian(s.dn[313][14]);
        let eq204_e2559_d_n15: f64 = self.ddt_jacobian(s.dn[313][15]);
        let eq204_e2559_d_n16: f64 = self.ddt_jacobian(s.dn[313][16]);
        let eq204_e2559_d_n17: f64 = self.ddt_jacobian(s.dn[313][17]);
        let eq204_e2559_d_n18: f64 = self.ddt_jacobian(s.dn[313][18]);
        let eq204_e2559_d_n19: f64 = self.ddt_jacobian(s.dn[313][19]);
        let eq204_e2559_d_n20: f64 = self.ddt_jacobian(s.dn[313][20]);
        let eq204_e2559_d_n21: f64 = self.ddt_jacobian(s.dn[313][21]);
        let eq204_e2559_d_n22: f64 = self.ddt_jacobian(s.dn[313][22]);
        let eq204_e2560: f64 = (p.p7 * eq204_e2559);
        let eq204_e2560_d_n0: f64 = (p.p7 * eq204_e2559_d_n0);
        let eq204_e2560_d_n1: f64 = (p.p7 * eq204_e2559_d_n1);
        let eq204_e2560_d_n2: f64 = (p.p7 * eq204_e2559_d_n2);
        let eq204_e2560_d_n3: f64 = (p.p7 * eq204_e2559_d_n3);
        let eq204_e2560_d_n4: f64 = (p.p7 * eq204_e2559_d_n4);
        let eq204_e2560_d_n5: f64 = (p.p7 * eq204_e2559_d_n5);
        let eq204_e2560_d_n6: f64 = (p.p7 * eq204_e2559_d_n6);
        let eq204_e2560_d_n7: f64 = (p.p7 * eq204_e2559_d_n7);
        let eq204_e2560_d_n8: f64 = (p.p7 * eq204_e2559_d_n8);
        let eq204_e2560_d_n9: f64 = (p.p7 * eq204_e2559_d_n9);
        let eq204_e2560_d_n10: f64 = (p.p7 * eq204_e2559_d_n10);
        let eq204_e2560_d_n11: f64 = (p.p7 * eq204_e2559_d_n11);
        let eq204_e2560_d_n12: f64 = (p.p7 * eq204_e2559_d_n12);
        let eq204_e2560_d_n13: f64 = (p.p7 * eq204_e2559_d_n13);
        let eq204_e2560_d_n14: f64 = (p.p7 * eq204_e2559_d_n14);
        let eq204_e2560_d_n15: f64 = (p.p7 * eq204_e2559_d_n15);
        let eq204_e2560_d_n16: f64 = (p.p7 * eq204_e2559_d_n16);
        let eq204_e2560_d_n17: f64 = (p.p7 * eq204_e2559_d_n17);
        let eq204_e2560_d_n18: f64 = (p.p7 * eq204_e2559_d_n18);
        let eq204_e2560_d_n19: f64 = (p.p7 * eq204_e2559_d_n19);
        let eq204_e2560_d_n20: f64 = (p.p7 * eq204_e2559_d_n20);
        let eq204_e2560_d_n21: f64 = (p.p7 * eq204_e2559_d_n21);
        let eq204_e2560_d_n22: f64 = (p.p7 * eq204_e2559_d_n22);
        (eq204_e2560, eq204_e2560_d_n0, eq204_e2560_d_n1, eq204_e2560_d_n2, eq204_e2560_d_n3, eq204_e2560_d_n4, eq204_e2560_d_n5, eq204_e2560_d_n6, eq204_e2560_d_n7, eq204_e2560_d_n8, eq204_e2560_d_n9, eq204_e2560_d_n10, eq204_e2560_d_n11, eq204_e2560_d_n12, eq204_e2560_d_n13, eq204_e2560_d_n14, eq204_e2560_d_n15, eq204_e2560_d_n16, eq204_e2560_d_n17, eq204_e2560_d_n18, eq204_e2560_d_n19, eq204_e2560_d_n20, eq204_e2560_d_n21, eq204_e2560_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq204_value: f64 = eq204_e2562;
        let eq204_node_derivatives: [f64; 23] = [eq204_e2562_d_n0, eq204_e2562_d_n1, eq204_e2562_d_n2, eq204_e2562_d_n3, eq204_e2562_d_n4, eq204_e2562_d_n5, eq204_e2562_d_n6, eq204_e2562_d_n7, eq204_e2562_d_n8, eq204_e2562_d_n9, eq204_e2562_d_n10, eq204_e2562_d_n11, eq204_e2562_d_n12, eq204_e2562_d_n13, eq204_e2562_d_n14, eq204_e2562_d_n15, eq204_e2562_d_n16, eq204_e2562_d_n17, eq204_e2562_d_n18, eq204_e2562_d_n19, eq204_e2562_d_n20, eq204_e2562_d_n21, eq204_e2562_d_n22];
        let eq204_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[22]),
            self.multiplicity * (eq204_value),
            &nodes,
            &eq204_node_derivatives,
            &branches,
            &eq204_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_205_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq205_e2573, eq205_e2573_d_n0, eq205_e2573_d_n1, eq205_e2573_d_n2, eq205_e2573_d_n3, eq205_e2573_d_n4, eq205_e2573_d_n5, eq205_e2573_d_n6, eq205_e2573_d_n7, eq205_e2573_d_n8, eq205_e2573_d_n9, eq205_e2573_d_n10, eq205_e2573_d_n11, eq205_e2573_d_n12, eq205_e2573_d_n13, eq205_e2573_d_n14, eq205_e2573_d_n15, eq205_e2573_d_n16, eq205_e2573_d_n17, eq205_e2573_d_n18, eq205_e2573_d_n19, eq205_e2573_d_n20, eq205_e2573_d_n21, eq205_e2573_d_n22,) = {
    if (((s.v[605] != 0.0) && (s.v[606] != 0.0)) && (s.v[607] != 0.0)) {
        let eq205_e2570: f64 = self.eval_ddt(104, s.v[312]);
        let eq205_e2570_d_n0: f64 = self.ddt_jacobian(s.dn[312][0]);
        let eq205_e2570_d_n1: f64 = self.ddt_jacobian(s.dn[312][1]);
        let eq205_e2570_d_n2: f64 = self.ddt_jacobian(s.dn[312][2]);
        let eq205_e2570_d_n3: f64 = self.ddt_jacobian(s.dn[312][3]);
        let eq205_e2570_d_n4: f64 = self.ddt_jacobian(s.dn[312][4]);
        let eq205_e2570_d_n5: f64 = self.ddt_jacobian(s.dn[312][5]);
        let eq205_e2570_d_n6: f64 = self.ddt_jacobian(s.dn[312][6]);
        let eq205_e2570_d_n7: f64 = self.ddt_jacobian(s.dn[312][7]);
        let eq205_e2570_d_n8: f64 = self.ddt_jacobian(s.dn[312][8]);
        let eq205_e2570_d_n9: f64 = self.ddt_jacobian(s.dn[312][9]);
        let eq205_e2570_d_n10: f64 = self.ddt_jacobian(s.dn[312][10]);
        let eq205_e2570_d_n11: f64 = self.ddt_jacobian(s.dn[312][11]);
        let eq205_e2570_d_n12: f64 = self.ddt_jacobian(s.dn[312][12]);
        let eq205_e2570_d_n13: f64 = self.ddt_jacobian(s.dn[312][13]);
        let eq205_e2570_d_n14: f64 = self.ddt_jacobian(s.dn[312][14]);
        let eq205_e2570_d_n15: f64 = self.ddt_jacobian(s.dn[312][15]);
        let eq205_e2570_d_n16: f64 = self.ddt_jacobian(s.dn[312][16]);
        let eq205_e2570_d_n17: f64 = self.ddt_jacobian(s.dn[312][17]);
        let eq205_e2570_d_n18: f64 = self.ddt_jacobian(s.dn[312][18]);
        let eq205_e2570_d_n19: f64 = self.ddt_jacobian(s.dn[312][19]);
        let eq205_e2570_d_n20: f64 = self.ddt_jacobian(s.dn[312][20]);
        let eq205_e2570_d_n21: f64 = self.ddt_jacobian(s.dn[312][21]);
        let eq205_e2570_d_n22: f64 = self.ddt_jacobian(s.dn[312][22]);
        let eq205_e2571: f64 = (p.p7 * eq205_e2570);
        let eq205_e2571_d_n0: f64 = (p.p7 * eq205_e2570_d_n0);
        let eq205_e2571_d_n1: f64 = (p.p7 * eq205_e2570_d_n1);
        let eq205_e2571_d_n2: f64 = (p.p7 * eq205_e2570_d_n2);
        let eq205_e2571_d_n3: f64 = (p.p7 * eq205_e2570_d_n3);
        let eq205_e2571_d_n4: f64 = (p.p7 * eq205_e2570_d_n4);
        let eq205_e2571_d_n5: f64 = (p.p7 * eq205_e2570_d_n5);
        let eq205_e2571_d_n6: f64 = (p.p7 * eq205_e2570_d_n6);
        let eq205_e2571_d_n7: f64 = (p.p7 * eq205_e2570_d_n7);
        let eq205_e2571_d_n8: f64 = (p.p7 * eq205_e2570_d_n8);
        let eq205_e2571_d_n9: f64 = (p.p7 * eq205_e2570_d_n9);
        let eq205_e2571_d_n10: f64 = (p.p7 * eq205_e2570_d_n10);
        let eq205_e2571_d_n11: f64 = (p.p7 * eq205_e2570_d_n11);
        let eq205_e2571_d_n12: f64 = (p.p7 * eq205_e2570_d_n12);
        let eq205_e2571_d_n13: f64 = (p.p7 * eq205_e2570_d_n13);
        let eq205_e2571_d_n14: f64 = (p.p7 * eq205_e2570_d_n14);
        let eq205_e2571_d_n15: f64 = (p.p7 * eq205_e2570_d_n15);
        let eq205_e2571_d_n16: f64 = (p.p7 * eq205_e2570_d_n16);
        let eq205_e2571_d_n17: f64 = (p.p7 * eq205_e2570_d_n17);
        let eq205_e2571_d_n18: f64 = (p.p7 * eq205_e2570_d_n18);
        let eq205_e2571_d_n19: f64 = (p.p7 * eq205_e2570_d_n19);
        let eq205_e2571_d_n20: f64 = (p.p7 * eq205_e2570_d_n20);
        let eq205_e2571_d_n21: f64 = (p.p7 * eq205_e2570_d_n21);
        let eq205_e2571_d_n22: f64 = (p.p7 * eq205_e2570_d_n22);
        (eq205_e2571, eq205_e2571_d_n0, eq205_e2571_d_n1, eq205_e2571_d_n2, eq205_e2571_d_n3, eq205_e2571_d_n4, eq205_e2571_d_n5, eq205_e2571_d_n6, eq205_e2571_d_n7, eq205_e2571_d_n8, eq205_e2571_d_n9, eq205_e2571_d_n10, eq205_e2571_d_n11, eq205_e2571_d_n12, eq205_e2571_d_n13, eq205_e2571_d_n14, eq205_e2571_d_n15, eq205_e2571_d_n16, eq205_e2571_d_n17, eq205_e2571_d_n18, eq205_e2571_d_n19, eq205_e2571_d_n20, eq205_e2571_d_n21, eq205_e2571_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq205_value: f64 = eq205_e2573;
        let eq205_node_derivatives: [f64; 23] = [eq205_e2573_d_n0, eq205_e2573_d_n1, eq205_e2573_d_n2, eq205_e2573_d_n3, eq205_e2573_d_n4, eq205_e2573_d_n5, eq205_e2573_d_n6, eq205_e2573_d_n7, eq205_e2573_d_n8, eq205_e2573_d_n9, eq205_e2573_d_n10, eq205_e2573_d_n11, eq205_e2573_d_n12, eq205_e2573_d_n13, eq205_e2573_d_n14, eq205_e2573_d_n15, eq205_e2573_d_n16, eq205_e2573_d_n17, eq205_e2573_d_n18, eq205_e2573_d_n19, eq205_e2573_d_n20, eq205_e2573_d_n21, eq205_e2573_d_n22];
        let eq205_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[22]),
            self.multiplicity * (eq205_value),
            &nodes,
            &eq205_node_derivatives,
            &branches,
            &eq205_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_206_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq206_e2586, eq206_e2586_d_n0, eq206_e2586_d_n1, eq206_e2586_d_n2, eq206_e2586_d_n3, eq206_e2586_d_n4, eq206_e2586_d_n5, eq206_e2586_d_n6, eq206_e2586_d_n7, eq206_e2586_d_n8, eq206_e2586_d_n9, eq206_e2586_d_n10, eq206_e2586_d_n11, eq206_e2586_d_n12, eq206_e2586_d_n13, eq206_e2586_d_n14, eq206_e2586_d_n15, eq206_e2586_d_n16, eq206_e2586_d_n17, eq206_e2586_d_n18, eq206_e2586_d_n19, eq206_e2586_d_n20, eq206_e2586_d_n21, eq206_e2586_d_n22,) = {
    if (((s.v[605] != 0.0) && (s.v[606] != 0.0)) && (s.v[607] != 0.0)) {
        let eq206_e2581: f64 = self.eval_ddt(105, s.v[312]);
        let eq206_e2581_d_n0: f64 = self.ddt_jacobian(s.dn[312][0]);
        let eq206_e2581_d_n1: f64 = self.ddt_jacobian(s.dn[312][1]);
        let eq206_e2581_d_n2: f64 = self.ddt_jacobian(s.dn[312][2]);
        let eq206_e2581_d_n3: f64 = self.ddt_jacobian(s.dn[312][3]);
        let eq206_e2581_d_n4: f64 = self.ddt_jacobian(s.dn[312][4]);
        let eq206_e2581_d_n5: f64 = self.ddt_jacobian(s.dn[312][5]);
        let eq206_e2581_d_n6: f64 = self.ddt_jacobian(s.dn[312][6]);
        let eq206_e2581_d_n7: f64 = self.ddt_jacobian(s.dn[312][7]);
        let eq206_e2581_d_n8: f64 = self.ddt_jacobian(s.dn[312][8]);
        let eq206_e2581_d_n9: f64 = self.ddt_jacobian(s.dn[312][9]);
        let eq206_e2581_d_n10: f64 = self.ddt_jacobian(s.dn[312][10]);
        let eq206_e2581_d_n11: f64 = self.ddt_jacobian(s.dn[312][11]);
        let eq206_e2581_d_n12: f64 = self.ddt_jacobian(s.dn[312][12]);
        let eq206_e2581_d_n13: f64 = self.ddt_jacobian(s.dn[312][13]);
        let eq206_e2581_d_n14: f64 = self.ddt_jacobian(s.dn[312][14]);
        let eq206_e2581_d_n15: f64 = self.ddt_jacobian(s.dn[312][15]);
        let eq206_e2581_d_n16: f64 = self.ddt_jacobian(s.dn[312][16]);
        let eq206_e2581_d_n17: f64 = self.ddt_jacobian(s.dn[312][17]);
        let eq206_e2581_d_n18: f64 = self.ddt_jacobian(s.dn[312][18]);
        let eq206_e2581_d_n19: f64 = self.ddt_jacobian(s.dn[312][19]);
        let eq206_e2581_d_n20: f64 = self.ddt_jacobian(s.dn[312][20]);
        let eq206_e2581_d_n21: f64 = self.ddt_jacobian(s.dn[312][21]);
        let eq206_e2581_d_n22: f64 = self.ddt_jacobian(s.dn[312][22]);
        let eq206_e2582: f64 = (p.p7 * eq206_e2581);
        let eq206_e2582_d_n0: f64 = (p.p7 * eq206_e2581_d_n0);
        let eq206_e2582_d_n1: f64 = (p.p7 * eq206_e2581_d_n1);
        let eq206_e2582_d_n2: f64 = (p.p7 * eq206_e2581_d_n2);
        let eq206_e2582_d_n3: f64 = (p.p7 * eq206_e2581_d_n3);
        let eq206_e2582_d_n4: f64 = (p.p7 * eq206_e2581_d_n4);
        let eq206_e2582_d_n5: f64 = (p.p7 * eq206_e2581_d_n5);
        let eq206_e2582_d_n6: f64 = (p.p7 * eq206_e2581_d_n6);
        let eq206_e2582_d_n7: f64 = (p.p7 * eq206_e2581_d_n7);
        let eq206_e2582_d_n8: f64 = (p.p7 * eq206_e2581_d_n8);
        let eq206_e2582_d_n9: f64 = (p.p7 * eq206_e2581_d_n9);
        let eq206_e2582_d_n10: f64 = (p.p7 * eq206_e2581_d_n10);
        let eq206_e2582_d_n11: f64 = (p.p7 * eq206_e2581_d_n11);
        let eq206_e2582_d_n12: f64 = (p.p7 * eq206_e2581_d_n12);
        let eq206_e2582_d_n13: f64 = (p.p7 * eq206_e2581_d_n13);
        let eq206_e2582_d_n14: f64 = (p.p7 * eq206_e2581_d_n14);
        let eq206_e2582_d_n15: f64 = (p.p7 * eq206_e2581_d_n15);
        let eq206_e2582_d_n16: f64 = (p.p7 * eq206_e2581_d_n16);
        let eq206_e2582_d_n17: f64 = (p.p7 * eq206_e2581_d_n17);
        let eq206_e2582_d_n18: f64 = (p.p7 * eq206_e2581_d_n18);
        let eq206_e2582_d_n19: f64 = (p.p7 * eq206_e2581_d_n19);
        let eq206_e2582_d_n20: f64 = (p.p7 * eq206_e2581_d_n20);
        let eq206_e2582_d_n21: f64 = (p.p7 * eq206_e2581_d_n21);
        let eq206_e2582_d_n22: f64 = (p.p7 * eq206_e2581_d_n22);
        let eq206_e2584: f64 = (eq206_e2582 * p.p249);
        let eq206_e2584_d_n0: f64 = (eq206_e2582_d_n0 * p.p249);
        let eq206_e2584_d_n1: f64 = (eq206_e2582_d_n1 * p.p249);
        let eq206_e2584_d_n2: f64 = (eq206_e2582_d_n2 * p.p249);
        let eq206_e2584_d_n3: f64 = (eq206_e2582_d_n3 * p.p249);
        let eq206_e2584_d_n4: f64 = (eq206_e2582_d_n4 * p.p249);
        let eq206_e2584_d_n5: f64 = (eq206_e2582_d_n5 * p.p249);
        let eq206_e2584_d_n6: f64 = (eq206_e2582_d_n6 * p.p249);
        let eq206_e2584_d_n7: f64 = (eq206_e2582_d_n7 * p.p249);
        let eq206_e2584_d_n8: f64 = (eq206_e2582_d_n8 * p.p249);
        let eq206_e2584_d_n9: f64 = (eq206_e2582_d_n9 * p.p249);
        let eq206_e2584_d_n10: f64 = (eq206_e2582_d_n10 * p.p249);
        let eq206_e2584_d_n11: f64 = (eq206_e2582_d_n11 * p.p249);
        let eq206_e2584_d_n12: f64 = (eq206_e2582_d_n12 * p.p249);
        let eq206_e2584_d_n13: f64 = (eq206_e2582_d_n13 * p.p249);
        let eq206_e2584_d_n14: f64 = (eq206_e2582_d_n14 * p.p249);
        let eq206_e2584_d_n15: f64 = (eq206_e2582_d_n15 * p.p249);
        let eq206_e2584_d_n16: f64 = (eq206_e2582_d_n16 * p.p249);
        let eq206_e2584_d_n17: f64 = (eq206_e2582_d_n17 * p.p249);
        let eq206_e2584_d_n18: f64 = (eq206_e2582_d_n18 * p.p249);
        let eq206_e2584_d_n19: f64 = (eq206_e2582_d_n19 * p.p249);
        let eq206_e2584_d_n20: f64 = (eq206_e2582_d_n20 * p.p249);
        let eq206_e2584_d_n21: f64 = (eq206_e2582_d_n21 * p.p249);
        let eq206_e2584_d_n22: f64 = (eq206_e2582_d_n22 * p.p249);
        (eq206_e2584, eq206_e2584_d_n0, eq206_e2584_d_n1, eq206_e2584_d_n2, eq206_e2584_d_n3, eq206_e2584_d_n4, eq206_e2584_d_n5, eq206_e2584_d_n6, eq206_e2584_d_n7, eq206_e2584_d_n8, eq206_e2584_d_n9, eq206_e2584_d_n10, eq206_e2584_d_n11, eq206_e2584_d_n12, eq206_e2584_d_n13, eq206_e2584_d_n14, eq206_e2584_d_n15, eq206_e2584_d_n16, eq206_e2584_d_n17, eq206_e2584_d_n18, eq206_e2584_d_n19, eq206_e2584_d_n20, eq206_e2584_d_n21, eq206_e2584_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq206_value: f64 = eq206_e2586;
        let eq206_node_derivatives: [f64; 23] = [eq206_e2586_d_n0, eq206_e2586_d_n1, eq206_e2586_d_n2, eq206_e2586_d_n3, eq206_e2586_d_n4, eq206_e2586_d_n5, eq206_e2586_d_n6, eq206_e2586_d_n7, eq206_e2586_d_n8, eq206_e2586_d_n9, eq206_e2586_d_n10, eq206_e2586_d_n11, eq206_e2586_d_n12, eq206_e2586_d_n13, eq206_e2586_d_n14, eq206_e2586_d_n15, eq206_e2586_d_n16, eq206_e2586_d_n17, eq206_e2586_d_n18, eq206_e2586_d_n19, eq206_e2586_d_n20, eq206_e2586_d_n21, eq206_e2586_d_n22];
        let eq206_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[22]),
            self.multiplicity * (eq206_value),
            &nodes,
            &eq206_node_derivatives,
            &branches,
            &eq206_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_207_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq207_e2598, eq207_e2598_d_n0, eq207_e2598_d_n1, eq207_e2598_d_n2, eq207_e2598_d_n3, eq207_e2598_d_n4, eq207_e2598_d_n5, eq207_e2598_d_n6, eq207_e2598_d_n7, eq207_e2598_d_n8, eq207_e2598_d_n9, eq207_e2598_d_n10, eq207_e2598_d_n11, eq207_e2598_d_n12, eq207_e2598_d_n13, eq207_e2598_d_n14, eq207_e2598_d_n15, eq207_e2598_d_n16, eq207_e2598_d_n17, eq207_e2598_d_n18, eq207_e2598_d_n19, eq207_e2598_d_n20, eq207_e2598_d_n21, eq207_e2598_d_n22,) = {
    if (((s.v[605] != 0.0) && (s.v[606] != 0.0)) && (!(s.v[607] != 0.0))) {
        let eq207_e2595: f64 = self.eval_ddt(106, s.v[312]);
        let eq207_e2595_d_n0: f64 = self.ddt_jacobian(s.dn[312][0]);
        let eq207_e2595_d_n1: f64 = self.ddt_jacobian(s.dn[312][1]);
        let eq207_e2595_d_n2: f64 = self.ddt_jacobian(s.dn[312][2]);
        let eq207_e2595_d_n3: f64 = self.ddt_jacobian(s.dn[312][3]);
        let eq207_e2595_d_n4: f64 = self.ddt_jacobian(s.dn[312][4]);
        let eq207_e2595_d_n5: f64 = self.ddt_jacobian(s.dn[312][5]);
        let eq207_e2595_d_n6: f64 = self.ddt_jacobian(s.dn[312][6]);
        let eq207_e2595_d_n7: f64 = self.ddt_jacobian(s.dn[312][7]);
        let eq207_e2595_d_n8: f64 = self.ddt_jacobian(s.dn[312][8]);
        let eq207_e2595_d_n9: f64 = self.ddt_jacobian(s.dn[312][9]);
        let eq207_e2595_d_n10: f64 = self.ddt_jacobian(s.dn[312][10]);
        let eq207_e2595_d_n11: f64 = self.ddt_jacobian(s.dn[312][11]);
        let eq207_e2595_d_n12: f64 = self.ddt_jacobian(s.dn[312][12]);
        let eq207_e2595_d_n13: f64 = self.ddt_jacobian(s.dn[312][13]);
        let eq207_e2595_d_n14: f64 = self.ddt_jacobian(s.dn[312][14]);
        let eq207_e2595_d_n15: f64 = self.ddt_jacobian(s.dn[312][15]);
        let eq207_e2595_d_n16: f64 = self.ddt_jacobian(s.dn[312][16]);
        let eq207_e2595_d_n17: f64 = self.ddt_jacobian(s.dn[312][17]);
        let eq207_e2595_d_n18: f64 = self.ddt_jacobian(s.dn[312][18]);
        let eq207_e2595_d_n19: f64 = self.ddt_jacobian(s.dn[312][19]);
        let eq207_e2595_d_n20: f64 = self.ddt_jacobian(s.dn[312][20]);
        let eq207_e2595_d_n21: f64 = self.ddt_jacobian(s.dn[312][21]);
        let eq207_e2595_d_n22: f64 = self.ddt_jacobian(s.dn[312][22]);
        let eq207_e2596: f64 = (p.p7 * eq207_e2595);
        let eq207_e2596_d_n0: f64 = (p.p7 * eq207_e2595_d_n0);
        let eq207_e2596_d_n1: f64 = (p.p7 * eq207_e2595_d_n1);
        let eq207_e2596_d_n2: f64 = (p.p7 * eq207_e2595_d_n2);
        let eq207_e2596_d_n3: f64 = (p.p7 * eq207_e2595_d_n3);
        let eq207_e2596_d_n4: f64 = (p.p7 * eq207_e2595_d_n4);
        let eq207_e2596_d_n5: f64 = (p.p7 * eq207_e2595_d_n5);
        let eq207_e2596_d_n6: f64 = (p.p7 * eq207_e2595_d_n6);
        let eq207_e2596_d_n7: f64 = (p.p7 * eq207_e2595_d_n7);
        let eq207_e2596_d_n8: f64 = (p.p7 * eq207_e2595_d_n8);
        let eq207_e2596_d_n9: f64 = (p.p7 * eq207_e2595_d_n9);
        let eq207_e2596_d_n10: f64 = (p.p7 * eq207_e2595_d_n10);
        let eq207_e2596_d_n11: f64 = (p.p7 * eq207_e2595_d_n11);
        let eq207_e2596_d_n12: f64 = (p.p7 * eq207_e2595_d_n12);
        let eq207_e2596_d_n13: f64 = (p.p7 * eq207_e2595_d_n13);
        let eq207_e2596_d_n14: f64 = (p.p7 * eq207_e2595_d_n14);
        let eq207_e2596_d_n15: f64 = (p.p7 * eq207_e2595_d_n15);
        let eq207_e2596_d_n16: f64 = (p.p7 * eq207_e2595_d_n16);
        let eq207_e2596_d_n17: f64 = (p.p7 * eq207_e2595_d_n17);
        let eq207_e2596_d_n18: f64 = (p.p7 * eq207_e2595_d_n18);
        let eq207_e2596_d_n19: f64 = (p.p7 * eq207_e2595_d_n19);
        let eq207_e2596_d_n20: f64 = (p.p7 * eq207_e2595_d_n20);
        let eq207_e2596_d_n21: f64 = (p.p7 * eq207_e2595_d_n21);
        let eq207_e2596_d_n22: f64 = (p.p7 * eq207_e2595_d_n22);
        (eq207_e2596, eq207_e2596_d_n0, eq207_e2596_d_n1, eq207_e2596_d_n2, eq207_e2596_d_n3, eq207_e2596_d_n4, eq207_e2596_d_n5, eq207_e2596_d_n6, eq207_e2596_d_n7, eq207_e2596_d_n8, eq207_e2596_d_n9, eq207_e2596_d_n10, eq207_e2596_d_n11, eq207_e2596_d_n12, eq207_e2596_d_n13, eq207_e2596_d_n14, eq207_e2596_d_n15, eq207_e2596_d_n16, eq207_e2596_d_n17, eq207_e2596_d_n18, eq207_e2596_d_n19, eq207_e2596_d_n20, eq207_e2596_d_n21, eq207_e2596_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq207_value: f64 = eq207_e2598;
        let eq207_node_derivatives: [f64; 23] = [eq207_e2598_d_n0, eq207_e2598_d_n1, eq207_e2598_d_n2, eq207_e2598_d_n3, eq207_e2598_d_n4, eq207_e2598_d_n5, eq207_e2598_d_n6, eq207_e2598_d_n7, eq207_e2598_d_n8, eq207_e2598_d_n9, eq207_e2598_d_n10, eq207_e2598_d_n11, eq207_e2598_d_n12, eq207_e2598_d_n13, eq207_e2598_d_n14, eq207_e2598_d_n15, eq207_e2598_d_n16, eq207_e2598_d_n17, eq207_e2598_d_n18, eq207_e2598_d_n19, eq207_e2598_d_n20, eq207_e2598_d_n21, eq207_e2598_d_n22];
        let eq207_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[22]),
            self.multiplicity * (eq207_value),
            &nodes,
            &eq207_node_derivatives,
            &branches,
            &eq207_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_208_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq208_e2612, eq208_e2612_d_n0, eq208_e2612_d_n1, eq208_e2612_d_n2, eq208_e2612_d_n3, eq208_e2612_d_n4, eq208_e2612_d_n5, eq208_e2612_d_n6, eq208_e2612_d_n7, eq208_e2612_d_n8, eq208_e2612_d_n9, eq208_e2612_d_n10, eq208_e2612_d_n11, eq208_e2612_d_n12, eq208_e2612_d_n13, eq208_e2612_d_n14, eq208_e2612_d_n15, eq208_e2612_d_n16, eq208_e2612_d_n17, eq208_e2612_d_n18, eq208_e2612_d_n19, eq208_e2612_d_n20, eq208_e2612_d_n21, eq208_e2612_d_n22,) = {
    if (((s.v[605] != 0.0) && (s.v[606] != 0.0)) && (!(s.v[607] != 0.0))) {
        let eq208_e2607: f64 = self.eval_ddt(107, s.v[312]);
        let eq208_e2607_d_n0: f64 = self.ddt_jacobian(s.dn[312][0]);
        let eq208_e2607_d_n1: f64 = self.ddt_jacobian(s.dn[312][1]);
        let eq208_e2607_d_n2: f64 = self.ddt_jacobian(s.dn[312][2]);
        let eq208_e2607_d_n3: f64 = self.ddt_jacobian(s.dn[312][3]);
        let eq208_e2607_d_n4: f64 = self.ddt_jacobian(s.dn[312][4]);
        let eq208_e2607_d_n5: f64 = self.ddt_jacobian(s.dn[312][5]);
        let eq208_e2607_d_n6: f64 = self.ddt_jacobian(s.dn[312][6]);
        let eq208_e2607_d_n7: f64 = self.ddt_jacobian(s.dn[312][7]);
        let eq208_e2607_d_n8: f64 = self.ddt_jacobian(s.dn[312][8]);
        let eq208_e2607_d_n9: f64 = self.ddt_jacobian(s.dn[312][9]);
        let eq208_e2607_d_n10: f64 = self.ddt_jacobian(s.dn[312][10]);
        let eq208_e2607_d_n11: f64 = self.ddt_jacobian(s.dn[312][11]);
        let eq208_e2607_d_n12: f64 = self.ddt_jacobian(s.dn[312][12]);
        let eq208_e2607_d_n13: f64 = self.ddt_jacobian(s.dn[312][13]);
        let eq208_e2607_d_n14: f64 = self.ddt_jacobian(s.dn[312][14]);
        let eq208_e2607_d_n15: f64 = self.ddt_jacobian(s.dn[312][15]);
        let eq208_e2607_d_n16: f64 = self.ddt_jacobian(s.dn[312][16]);
        let eq208_e2607_d_n17: f64 = self.ddt_jacobian(s.dn[312][17]);
        let eq208_e2607_d_n18: f64 = self.ddt_jacobian(s.dn[312][18]);
        let eq208_e2607_d_n19: f64 = self.ddt_jacobian(s.dn[312][19]);
        let eq208_e2607_d_n20: f64 = self.ddt_jacobian(s.dn[312][20]);
        let eq208_e2607_d_n21: f64 = self.ddt_jacobian(s.dn[312][21]);
        let eq208_e2607_d_n22: f64 = self.ddt_jacobian(s.dn[312][22]);
        let eq208_e2608: f64 = (p.p7 * eq208_e2607);
        let eq208_e2608_d_n0: f64 = (p.p7 * eq208_e2607_d_n0);
        let eq208_e2608_d_n1: f64 = (p.p7 * eq208_e2607_d_n1);
        let eq208_e2608_d_n2: f64 = (p.p7 * eq208_e2607_d_n2);
        let eq208_e2608_d_n3: f64 = (p.p7 * eq208_e2607_d_n3);
        let eq208_e2608_d_n4: f64 = (p.p7 * eq208_e2607_d_n4);
        let eq208_e2608_d_n5: f64 = (p.p7 * eq208_e2607_d_n5);
        let eq208_e2608_d_n6: f64 = (p.p7 * eq208_e2607_d_n6);
        let eq208_e2608_d_n7: f64 = (p.p7 * eq208_e2607_d_n7);
        let eq208_e2608_d_n8: f64 = (p.p7 * eq208_e2607_d_n8);
        let eq208_e2608_d_n9: f64 = (p.p7 * eq208_e2607_d_n9);
        let eq208_e2608_d_n10: f64 = (p.p7 * eq208_e2607_d_n10);
        let eq208_e2608_d_n11: f64 = (p.p7 * eq208_e2607_d_n11);
        let eq208_e2608_d_n12: f64 = (p.p7 * eq208_e2607_d_n12);
        let eq208_e2608_d_n13: f64 = (p.p7 * eq208_e2607_d_n13);
        let eq208_e2608_d_n14: f64 = (p.p7 * eq208_e2607_d_n14);
        let eq208_e2608_d_n15: f64 = (p.p7 * eq208_e2607_d_n15);
        let eq208_e2608_d_n16: f64 = (p.p7 * eq208_e2607_d_n16);
        let eq208_e2608_d_n17: f64 = (p.p7 * eq208_e2607_d_n17);
        let eq208_e2608_d_n18: f64 = (p.p7 * eq208_e2607_d_n18);
        let eq208_e2608_d_n19: f64 = (p.p7 * eq208_e2607_d_n19);
        let eq208_e2608_d_n20: f64 = (p.p7 * eq208_e2607_d_n20);
        let eq208_e2608_d_n21: f64 = (p.p7 * eq208_e2607_d_n21);
        let eq208_e2608_d_n22: f64 = (p.p7 * eq208_e2607_d_n22);
        let eq208_e2610: f64 = (eq208_e2608 * p.p249);
        let eq208_e2610_d_n0: f64 = (eq208_e2608_d_n0 * p.p249);
        let eq208_e2610_d_n1: f64 = (eq208_e2608_d_n1 * p.p249);
        let eq208_e2610_d_n2: f64 = (eq208_e2608_d_n2 * p.p249);
        let eq208_e2610_d_n3: f64 = (eq208_e2608_d_n3 * p.p249);
        let eq208_e2610_d_n4: f64 = (eq208_e2608_d_n4 * p.p249);
        let eq208_e2610_d_n5: f64 = (eq208_e2608_d_n5 * p.p249);
        let eq208_e2610_d_n6: f64 = (eq208_e2608_d_n6 * p.p249);
        let eq208_e2610_d_n7: f64 = (eq208_e2608_d_n7 * p.p249);
        let eq208_e2610_d_n8: f64 = (eq208_e2608_d_n8 * p.p249);
        let eq208_e2610_d_n9: f64 = (eq208_e2608_d_n9 * p.p249);
        let eq208_e2610_d_n10: f64 = (eq208_e2608_d_n10 * p.p249);
        let eq208_e2610_d_n11: f64 = (eq208_e2608_d_n11 * p.p249);
        let eq208_e2610_d_n12: f64 = (eq208_e2608_d_n12 * p.p249);
        let eq208_e2610_d_n13: f64 = (eq208_e2608_d_n13 * p.p249);
        let eq208_e2610_d_n14: f64 = (eq208_e2608_d_n14 * p.p249);
        let eq208_e2610_d_n15: f64 = (eq208_e2608_d_n15 * p.p249);
        let eq208_e2610_d_n16: f64 = (eq208_e2608_d_n16 * p.p249);
        let eq208_e2610_d_n17: f64 = (eq208_e2608_d_n17 * p.p249);
        let eq208_e2610_d_n18: f64 = (eq208_e2608_d_n18 * p.p249);
        let eq208_e2610_d_n19: f64 = (eq208_e2608_d_n19 * p.p249);
        let eq208_e2610_d_n20: f64 = (eq208_e2608_d_n20 * p.p249);
        let eq208_e2610_d_n21: f64 = (eq208_e2608_d_n21 * p.p249);
        let eq208_e2610_d_n22: f64 = (eq208_e2608_d_n22 * p.p249);
        (eq208_e2610, eq208_e2610_d_n0, eq208_e2610_d_n1, eq208_e2610_d_n2, eq208_e2610_d_n3, eq208_e2610_d_n4, eq208_e2610_d_n5, eq208_e2610_d_n6, eq208_e2610_d_n7, eq208_e2610_d_n8, eq208_e2610_d_n9, eq208_e2610_d_n10, eq208_e2610_d_n11, eq208_e2610_d_n12, eq208_e2610_d_n13, eq208_e2610_d_n14, eq208_e2610_d_n15, eq208_e2610_d_n16, eq208_e2610_d_n17, eq208_e2610_d_n18, eq208_e2610_d_n19, eq208_e2610_d_n20, eq208_e2610_d_n21, eq208_e2610_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq208_value: f64 = eq208_e2612;
        let eq208_node_derivatives: [f64; 23] = [eq208_e2612_d_n0, eq208_e2612_d_n1, eq208_e2612_d_n2, eq208_e2612_d_n3, eq208_e2612_d_n4, eq208_e2612_d_n5, eq208_e2612_d_n6, eq208_e2612_d_n7, eq208_e2612_d_n8, eq208_e2612_d_n9, eq208_e2612_d_n10, eq208_e2612_d_n11, eq208_e2612_d_n12, eq208_e2612_d_n13, eq208_e2612_d_n14, eq208_e2612_d_n15, eq208_e2612_d_n16, eq208_e2612_d_n17, eq208_e2612_d_n18, eq208_e2612_d_n19, eq208_e2612_d_n20, eq208_e2612_d_n21, eq208_e2612_d_n22];
        let eq208_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[22]),
            self.multiplicity * (eq208_value),
            &nodes,
            &eq208_node_derivatives,
            &branches,
            &eq208_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_209_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq209_e2623, eq209_e2623_d_n0, eq209_e2623_d_n1, eq209_e2623_d_n2, eq209_e2623_d_n3, eq209_e2623_d_n4, eq209_e2623_d_n5, eq209_e2623_d_n6, eq209_e2623_d_n7, eq209_e2623_d_n8, eq209_e2623_d_n9, eq209_e2623_d_n10, eq209_e2623_d_n11, eq209_e2623_d_n12, eq209_e2623_d_n13, eq209_e2623_d_n14, eq209_e2623_d_n15, eq209_e2623_d_n16, eq209_e2623_d_n17, eq209_e2623_d_n18, eq209_e2623_d_n19, eq209_e2623_d_n20, eq209_e2623_d_n21, eq209_e2623_d_n22,) = {
    if ((s.v[605] != 0.0) && (s.v[606] != 0.0)) {
        let eq209_e2619: f64 = (p.p254 * s.v[312]);
        let eq209_e2619_d_n0: f64 = (p.p254 * s.dn[312][0]);
        let eq209_e2619_d_n1: f64 = (p.p254 * s.dn[312][1]);
        let eq209_e2619_d_n2: f64 = (p.p254 * s.dn[312][2]);
        let eq209_e2619_d_n3: f64 = (p.p254 * s.dn[312][3]);
        let eq209_e2619_d_n4: f64 = (p.p254 * s.dn[312][4]);
        let eq209_e2619_d_n5: f64 = (p.p254 * s.dn[312][5]);
        let eq209_e2619_d_n6: f64 = (p.p254 * s.dn[312][6]);
        let eq209_e2619_d_n7: f64 = (p.p254 * s.dn[312][7]);
        let eq209_e2619_d_n8: f64 = (p.p254 * s.dn[312][8]);
        let eq209_e2619_d_n9: f64 = (p.p254 * s.dn[312][9]);
        let eq209_e2619_d_n10: f64 = (p.p254 * s.dn[312][10]);
        let eq209_e2619_d_n11: f64 = (p.p254 * s.dn[312][11]);
        let eq209_e2619_d_n12: f64 = (p.p254 * s.dn[312][12]);
        let eq209_e2619_d_n13: f64 = (p.p254 * s.dn[312][13]);
        let eq209_e2619_d_n14: f64 = (p.p254 * s.dn[312][14]);
        let eq209_e2619_d_n15: f64 = (p.p254 * s.dn[312][15]);
        let eq209_e2619_d_n16: f64 = (p.p254 * s.dn[312][16]);
        let eq209_e2619_d_n17: f64 = (p.p254 * s.dn[312][17]);
        let eq209_e2619_d_n18: f64 = (p.p254 * s.dn[312][18]);
        let eq209_e2619_d_n19: f64 = (p.p254 * s.dn[312][19]);
        let eq209_e2619_d_n20: f64 = (p.p254 * s.dn[312][20]);
        let eq209_e2619_d_n21: f64 = (p.p254 * s.dn[312][21]);
        let eq209_e2619_d_n22: f64 = (p.p254 * s.dn[312][22]);
        let eq209_e2620: f64 = self.eval_ddt(108, eq209_e2619);
        let eq209_e2620_d_n0: f64 = self.ddt_jacobian(eq209_e2619_d_n0);
        let eq209_e2620_d_n1: f64 = self.ddt_jacobian(eq209_e2619_d_n1);
        let eq209_e2620_d_n2: f64 = self.ddt_jacobian(eq209_e2619_d_n2);
        let eq209_e2620_d_n3: f64 = self.ddt_jacobian(eq209_e2619_d_n3);
        let eq209_e2620_d_n4: f64 = self.ddt_jacobian(eq209_e2619_d_n4);
        let eq209_e2620_d_n5: f64 = self.ddt_jacobian(eq209_e2619_d_n5);
        let eq209_e2620_d_n6: f64 = self.ddt_jacobian(eq209_e2619_d_n6);
        let eq209_e2620_d_n7: f64 = self.ddt_jacobian(eq209_e2619_d_n7);
        let eq209_e2620_d_n8: f64 = self.ddt_jacobian(eq209_e2619_d_n8);
        let eq209_e2620_d_n9: f64 = self.ddt_jacobian(eq209_e2619_d_n9);
        let eq209_e2620_d_n10: f64 = self.ddt_jacobian(eq209_e2619_d_n10);
        let eq209_e2620_d_n11: f64 = self.ddt_jacobian(eq209_e2619_d_n11);
        let eq209_e2620_d_n12: f64 = self.ddt_jacobian(eq209_e2619_d_n12);
        let eq209_e2620_d_n13: f64 = self.ddt_jacobian(eq209_e2619_d_n13);
        let eq209_e2620_d_n14: f64 = self.ddt_jacobian(eq209_e2619_d_n14);
        let eq209_e2620_d_n15: f64 = self.ddt_jacobian(eq209_e2619_d_n15);
        let eq209_e2620_d_n16: f64 = self.ddt_jacobian(eq209_e2619_d_n16);
        let eq209_e2620_d_n17: f64 = self.ddt_jacobian(eq209_e2619_d_n17);
        let eq209_e2620_d_n18: f64 = self.ddt_jacobian(eq209_e2619_d_n18);
        let eq209_e2620_d_n19: f64 = self.ddt_jacobian(eq209_e2619_d_n19);
        let eq209_e2620_d_n20: f64 = self.ddt_jacobian(eq209_e2619_d_n20);
        let eq209_e2620_d_n21: f64 = self.ddt_jacobian(eq209_e2619_d_n21);
        let eq209_e2620_d_n22: f64 = self.ddt_jacobian(eq209_e2619_d_n22);
        let eq209_e2621: f64 = (p.p7 * eq209_e2620);
        let eq209_e2621_d_n0: f64 = (p.p7 * eq209_e2620_d_n0);
        let eq209_e2621_d_n1: f64 = (p.p7 * eq209_e2620_d_n1);
        let eq209_e2621_d_n2: f64 = (p.p7 * eq209_e2620_d_n2);
        let eq209_e2621_d_n3: f64 = (p.p7 * eq209_e2620_d_n3);
        let eq209_e2621_d_n4: f64 = (p.p7 * eq209_e2620_d_n4);
        let eq209_e2621_d_n5: f64 = (p.p7 * eq209_e2620_d_n5);
        let eq209_e2621_d_n6: f64 = (p.p7 * eq209_e2620_d_n6);
        let eq209_e2621_d_n7: f64 = (p.p7 * eq209_e2620_d_n7);
        let eq209_e2621_d_n8: f64 = (p.p7 * eq209_e2620_d_n8);
        let eq209_e2621_d_n9: f64 = (p.p7 * eq209_e2620_d_n9);
        let eq209_e2621_d_n10: f64 = (p.p7 * eq209_e2620_d_n10);
        let eq209_e2621_d_n11: f64 = (p.p7 * eq209_e2620_d_n11);
        let eq209_e2621_d_n12: f64 = (p.p7 * eq209_e2620_d_n12);
        let eq209_e2621_d_n13: f64 = (p.p7 * eq209_e2620_d_n13);
        let eq209_e2621_d_n14: f64 = (p.p7 * eq209_e2620_d_n14);
        let eq209_e2621_d_n15: f64 = (p.p7 * eq209_e2620_d_n15);
        let eq209_e2621_d_n16: f64 = (p.p7 * eq209_e2620_d_n16);
        let eq209_e2621_d_n17: f64 = (p.p7 * eq209_e2620_d_n17);
        let eq209_e2621_d_n18: f64 = (p.p7 * eq209_e2620_d_n18);
        let eq209_e2621_d_n19: f64 = (p.p7 * eq209_e2620_d_n19);
        let eq209_e2621_d_n20: f64 = (p.p7 * eq209_e2620_d_n20);
        let eq209_e2621_d_n21: f64 = (p.p7 * eq209_e2620_d_n21);
        let eq209_e2621_d_n22: f64 = (p.p7 * eq209_e2620_d_n22);
        (eq209_e2621, eq209_e2621_d_n0, eq209_e2621_d_n1, eq209_e2621_d_n2, eq209_e2621_d_n3, eq209_e2621_d_n4, eq209_e2621_d_n5, eq209_e2621_d_n6, eq209_e2621_d_n7, eq209_e2621_d_n8, eq209_e2621_d_n9, eq209_e2621_d_n10, eq209_e2621_d_n11, eq209_e2621_d_n12, eq209_e2621_d_n13, eq209_e2621_d_n14, eq209_e2621_d_n15, eq209_e2621_d_n16, eq209_e2621_d_n17, eq209_e2621_d_n18, eq209_e2621_d_n19, eq209_e2621_d_n20, eq209_e2621_d_n21, eq209_e2621_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq209_value: f64 = eq209_e2623;
        let eq209_node_derivatives: [f64; 23] = [eq209_e2623_d_n0, eq209_e2623_d_n1, eq209_e2623_d_n2, eq209_e2623_d_n3, eq209_e2623_d_n4, eq209_e2623_d_n5, eq209_e2623_d_n6, eq209_e2623_d_n7, eq209_e2623_d_n8, eq209_e2623_d_n9, eq209_e2623_d_n10, eq209_e2623_d_n11, eq209_e2623_d_n12, eq209_e2623_d_n13, eq209_e2623_d_n14, eq209_e2623_d_n15, eq209_e2623_d_n16, eq209_e2623_d_n17, eq209_e2623_d_n18, eq209_e2623_d_n19, eq209_e2623_d_n20, eq209_e2623_d_n21, eq209_e2623_d_n22];
        let eq209_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[22]),
            self.multiplicity * (eq209_value),
            &nodes,
            &eq209_node_derivatives,
            &branches,
            &eq209_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_210_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq210_e2633, eq210_e2633_d_n0, eq210_e2633_d_n1, eq210_e2633_d_n2, eq210_e2633_d_n3, eq210_e2633_d_n4, eq210_e2633_d_n5, eq210_e2633_d_n6, eq210_e2633_d_n7, eq210_e2633_d_n8, eq210_e2633_d_n9, eq210_e2633_d_n10, eq210_e2633_d_n11, eq210_e2633_d_n12, eq210_e2633_d_n13, eq210_e2633_d_n14, eq210_e2633_d_n15, eq210_e2633_d_n16, eq210_e2633_d_n17, eq210_e2633_d_n18, eq210_e2633_d_n19, eq210_e2633_d_n20, eq210_e2633_d_n21, eq210_e2633_d_n22,) = {
    if ((!(s.v[605] != 0.0)) && (s.v[608] != 0.0)) {
        let eq210_e2630: f64 = self.eval_ddt(109, s.v[313]);
        let eq210_e2630_d_n0: f64 = self.ddt_jacobian(s.dn[313][0]);
        let eq210_e2630_d_n1: f64 = self.ddt_jacobian(s.dn[313][1]);
        let eq210_e2630_d_n2: f64 = self.ddt_jacobian(s.dn[313][2]);
        let eq210_e2630_d_n3: f64 = self.ddt_jacobian(s.dn[313][3]);
        let eq210_e2630_d_n4: f64 = self.ddt_jacobian(s.dn[313][4]);
        let eq210_e2630_d_n5: f64 = self.ddt_jacobian(s.dn[313][5]);
        let eq210_e2630_d_n6: f64 = self.ddt_jacobian(s.dn[313][6]);
        let eq210_e2630_d_n7: f64 = self.ddt_jacobian(s.dn[313][7]);
        let eq210_e2630_d_n8: f64 = self.ddt_jacobian(s.dn[313][8]);
        let eq210_e2630_d_n9: f64 = self.ddt_jacobian(s.dn[313][9]);
        let eq210_e2630_d_n10: f64 = self.ddt_jacobian(s.dn[313][10]);
        let eq210_e2630_d_n11: f64 = self.ddt_jacobian(s.dn[313][11]);
        let eq210_e2630_d_n12: f64 = self.ddt_jacobian(s.dn[313][12]);
        let eq210_e2630_d_n13: f64 = self.ddt_jacobian(s.dn[313][13]);
        let eq210_e2630_d_n14: f64 = self.ddt_jacobian(s.dn[313][14]);
        let eq210_e2630_d_n15: f64 = self.ddt_jacobian(s.dn[313][15]);
        let eq210_e2630_d_n16: f64 = self.ddt_jacobian(s.dn[313][16]);
        let eq210_e2630_d_n17: f64 = self.ddt_jacobian(s.dn[313][17]);
        let eq210_e2630_d_n18: f64 = self.ddt_jacobian(s.dn[313][18]);
        let eq210_e2630_d_n19: f64 = self.ddt_jacobian(s.dn[313][19]);
        let eq210_e2630_d_n20: f64 = self.ddt_jacobian(s.dn[313][20]);
        let eq210_e2630_d_n21: f64 = self.ddt_jacobian(s.dn[313][21]);
        let eq210_e2630_d_n22: f64 = self.ddt_jacobian(s.dn[313][22]);
        let eq210_e2631: f64 = (p.p7 * eq210_e2630);
        let eq210_e2631_d_n0: f64 = (p.p7 * eq210_e2630_d_n0);
        let eq210_e2631_d_n1: f64 = (p.p7 * eq210_e2630_d_n1);
        let eq210_e2631_d_n2: f64 = (p.p7 * eq210_e2630_d_n2);
        let eq210_e2631_d_n3: f64 = (p.p7 * eq210_e2630_d_n3);
        let eq210_e2631_d_n4: f64 = (p.p7 * eq210_e2630_d_n4);
        let eq210_e2631_d_n5: f64 = (p.p7 * eq210_e2630_d_n5);
        let eq210_e2631_d_n6: f64 = (p.p7 * eq210_e2630_d_n6);
        let eq210_e2631_d_n7: f64 = (p.p7 * eq210_e2630_d_n7);
        let eq210_e2631_d_n8: f64 = (p.p7 * eq210_e2630_d_n8);
        let eq210_e2631_d_n9: f64 = (p.p7 * eq210_e2630_d_n9);
        let eq210_e2631_d_n10: f64 = (p.p7 * eq210_e2630_d_n10);
        let eq210_e2631_d_n11: f64 = (p.p7 * eq210_e2630_d_n11);
        let eq210_e2631_d_n12: f64 = (p.p7 * eq210_e2630_d_n12);
        let eq210_e2631_d_n13: f64 = (p.p7 * eq210_e2630_d_n13);
        let eq210_e2631_d_n14: f64 = (p.p7 * eq210_e2630_d_n14);
        let eq210_e2631_d_n15: f64 = (p.p7 * eq210_e2630_d_n15);
        let eq210_e2631_d_n16: f64 = (p.p7 * eq210_e2630_d_n16);
        let eq210_e2631_d_n17: f64 = (p.p7 * eq210_e2630_d_n17);
        let eq210_e2631_d_n18: f64 = (p.p7 * eq210_e2630_d_n18);
        let eq210_e2631_d_n19: f64 = (p.p7 * eq210_e2630_d_n19);
        let eq210_e2631_d_n20: f64 = (p.p7 * eq210_e2630_d_n20);
        let eq210_e2631_d_n21: f64 = (p.p7 * eq210_e2630_d_n21);
        let eq210_e2631_d_n22: f64 = (p.p7 * eq210_e2630_d_n22);
        (eq210_e2631, eq210_e2631_d_n0, eq210_e2631_d_n1, eq210_e2631_d_n2, eq210_e2631_d_n3, eq210_e2631_d_n4, eq210_e2631_d_n5, eq210_e2631_d_n6, eq210_e2631_d_n7, eq210_e2631_d_n8, eq210_e2631_d_n9, eq210_e2631_d_n10, eq210_e2631_d_n11, eq210_e2631_d_n12, eq210_e2631_d_n13, eq210_e2631_d_n14, eq210_e2631_d_n15, eq210_e2631_d_n16, eq210_e2631_d_n17, eq210_e2631_d_n18, eq210_e2631_d_n19, eq210_e2631_d_n20, eq210_e2631_d_n21, eq210_e2631_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq210_value: f64 = eq210_e2633;
        let eq210_node_derivatives: [f64; 23] = [eq210_e2633_d_n0, eq210_e2633_d_n1, eq210_e2633_d_n2, eq210_e2633_d_n3, eq210_e2633_d_n4, eq210_e2633_d_n5, eq210_e2633_d_n6, eq210_e2633_d_n7, eq210_e2633_d_n8, eq210_e2633_d_n9, eq210_e2633_d_n10, eq210_e2633_d_n11, eq210_e2633_d_n12, eq210_e2633_d_n13, eq210_e2633_d_n14, eq210_e2633_d_n15, eq210_e2633_d_n16, eq210_e2633_d_n17, eq210_e2633_d_n18, eq210_e2633_d_n19, eq210_e2633_d_n20, eq210_e2633_d_n21, eq210_e2633_d_n22];
        let eq210_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            self.multiplicity * (eq210_value),
            &nodes,
            &eq210_node_derivatives,
            &branches,
            &eq210_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_211_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq211_e2645, eq211_e2645_d_n0, eq211_e2645_d_n1, eq211_e2645_d_n2, eq211_e2645_d_n3, eq211_e2645_d_n4, eq211_e2645_d_n5, eq211_e2645_d_n6, eq211_e2645_d_n7, eq211_e2645_d_n8, eq211_e2645_d_n9, eq211_e2645_d_n10, eq211_e2645_d_n11, eq211_e2645_d_n12, eq211_e2645_d_n13, eq211_e2645_d_n14, eq211_e2645_d_n15, eq211_e2645_d_n16, eq211_e2645_d_n17, eq211_e2645_d_n18, eq211_e2645_d_n19, eq211_e2645_d_n20, eq211_e2645_d_n21, eq211_e2645_d_n22,) = {
    if (((!(s.v[605] != 0.0)) && (s.v[608] != 0.0)) && (s.v[609] != 0.0)) {
        let eq211_e2642: f64 = self.eval_ddt(110, s.v[312]);
        let eq211_e2642_d_n0: f64 = self.ddt_jacobian(s.dn[312][0]);
        let eq211_e2642_d_n1: f64 = self.ddt_jacobian(s.dn[312][1]);
        let eq211_e2642_d_n2: f64 = self.ddt_jacobian(s.dn[312][2]);
        let eq211_e2642_d_n3: f64 = self.ddt_jacobian(s.dn[312][3]);
        let eq211_e2642_d_n4: f64 = self.ddt_jacobian(s.dn[312][4]);
        let eq211_e2642_d_n5: f64 = self.ddt_jacobian(s.dn[312][5]);
        let eq211_e2642_d_n6: f64 = self.ddt_jacobian(s.dn[312][6]);
        let eq211_e2642_d_n7: f64 = self.ddt_jacobian(s.dn[312][7]);
        let eq211_e2642_d_n8: f64 = self.ddt_jacobian(s.dn[312][8]);
        let eq211_e2642_d_n9: f64 = self.ddt_jacobian(s.dn[312][9]);
        let eq211_e2642_d_n10: f64 = self.ddt_jacobian(s.dn[312][10]);
        let eq211_e2642_d_n11: f64 = self.ddt_jacobian(s.dn[312][11]);
        let eq211_e2642_d_n12: f64 = self.ddt_jacobian(s.dn[312][12]);
        let eq211_e2642_d_n13: f64 = self.ddt_jacobian(s.dn[312][13]);
        let eq211_e2642_d_n14: f64 = self.ddt_jacobian(s.dn[312][14]);
        let eq211_e2642_d_n15: f64 = self.ddt_jacobian(s.dn[312][15]);
        let eq211_e2642_d_n16: f64 = self.ddt_jacobian(s.dn[312][16]);
        let eq211_e2642_d_n17: f64 = self.ddt_jacobian(s.dn[312][17]);
        let eq211_e2642_d_n18: f64 = self.ddt_jacobian(s.dn[312][18]);
        let eq211_e2642_d_n19: f64 = self.ddt_jacobian(s.dn[312][19]);
        let eq211_e2642_d_n20: f64 = self.ddt_jacobian(s.dn[312][20]);
        let eq211_e2642_d_n21: f64 = self.ddt_jacobian(s.dn[312][21]);
        let eq211_e2642_d_n22: f64 = self.ddt_jacobian(s.dn[312][22]);
        let eq211_e2643: f64 = (p.p7 * eq211_e2642);
        let eq211_e2643_d_n0: f64 = (p.p7 * eq211_e2642_d_n0);
        let eq211_e2643_d_n1: f64 = (p.p7 * eq211_e2642_d_n1);
        let eq211_e2643_d_n2: f64 = (p.p7 * eq211_e2642_d_n2);
        let eq211_e2643_d_n3: f64 = (p.p7 * eq211_e2642_d_n3);
        let eq211_e2643_d_n4: f64 = (p.p7 * eq211_e2642_d_n4);
        let eq211_e2643_d_n5: f64 = (p.p7 * eq211_e2642_d_n5);
        let eq211_e2643_d_n6: f64 = (p.p7 * eq211_e2642_d_n6);
        let eq211_e2643_d_n7: f64 = (p.p7 * eq211_e2642_d_n7);
        let eq211_e2643_d_n8: f64 = (p.p7 * eq211_e2642_d_n8);
        let eq211_e2643_d_n9: f64 = (p.p7 * eq211_e2642_d_n9);
        let eq211_e2643_d_n10: f64 = (p.p7 * eq211_e2642_d_n10);
        let eq211_e2643_d_n11: f64 = (p.p7 * eq211_e2642_d_n11);
        let eq211_e2643_d_n12: f64 = (p.p7 * eq211_e2642_d_n12);
        let eq211_e2643_d_n13: f64 = (p.p7 * eq211_e2642_d_n13);
        let eq211_e2643_d_n14: f64 = (p.p7 * eq211_e2642_d_n14);
        let eq211_e2643_d_n15: f64 = (p.p7 * eq211_e2642_d_n15);
        let eq211_e2643_d_n16: f64 = (p.p7 * eq211_e2642_d_n16);
        let eq211_e2643_d_n17: f64 = (p.p7 * eq211_e2642_d_n17);
        let eq211_e2643_d_n18: f64 = (p.p7 * eq211_e2642_d_n18);
        let eq211_e2643_d_n19: f64 = (p.p7 * eq211_e2642_d_n19);
        let eq211_e2643_d_n20: f64 = (p.p7 * eq211_e2642_d_n20);
        let eq211_e2643_d_n21: f64 = (p.p7 * eq211_e2642_d_n21);
        let eq211_e2643_d_n22: f64 = (p.p7 * eq211_e2642_d_n22);
        (eq211_e2643, eq211_e2643_d_n0, eq211_e2643_d_n1, eq211_e2643_d_n2, eq211_e2643_d_n3, eq211_e2643_d_n4, eq211_e2643_d_n5, eq211_e2643_d_n6, eq211_e2643_d_n7, eq211_e2643_d_n8, eq211_e2643_d_n9, eq211_e2643_d_n10, eq211_e2643_d_n11, eq211_e2643_d_n12, eq211_e2643_d_n13, eq211_e2643_d_n14, eq211_e2643_d_n15, eq211_e2643_d_n16, eq211_e2643_d_n17, eq211_e2643_d_n18, eq211_e2643_d_n19, eq211_e2643_d_n20, eq211_e2643_d_n21, eq211_e2643_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq211_value: f64 = eq211_e2645;
        let eq211_node_derivatives: [f64; 23] = [eq211_e2645_d_n0, eq211_e2645_d_n1, eq211_e2645_d_n2, eq211_e2645_d_n3, eq211_e2645_d_n4, eq211_e2645_d_n5, eq211_e2645_d_n6, eq211_e2645_d_n7, eq211_e2645_d_n8, eq211_e2645_d_n9, eq211_e2645_d_n10, eq211_e2645_d_n11, eq211_e2645_d_n12, eq211_e2645_d_n13, eq211_e2645_d_n14, eq211_e2645_d_n15, eq211_e2645_d_n16, eq211_e2645_d_n17, eq211_e2645_d_n18, eq211_e2645_d_n19, eq211_e2645_d_n20, eq211_e2645_d_n21, eq211_e2645_d_n22];
        let eq211_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[2]),
            self.multiplicity * (eq211_value),
            &nodes,
            &eq211_node_derivatives,
            &branches,
            &eq211_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_212_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq212_e2659, eq212_e2659_d_n0, eq212_e2659_d_n1, eq212_e2659_d_n2, eq212_e2659_d_n3, eq212_e2659_d_n4, eq212_e2659_d_n5, eq212_e2659_d_n6, eq212_e2659_d_n7, eq212_e2659_d_n8, eq212_e2659_d_n9, eq212_e2659_d_n10, eq212_e2659_d_n11, eq212_e2659_d_n12, eq212_e2659_d_n13, eq212_e2659_d_n14, eq212_e2659_d_n15, eq212_e2659_d_n16, eq212_e2659_d_n17, eq212_e2659_d_n18, eq212_e2659_d_n19, eq212_e2659_d_n20, eq212_e2659_d_n21, eq212_e2659_d_n22,) = {
    if (((!(s.v[605] != 0.0)) && (s.v[608] != 0.0)) && (s.v[609] != 0.0)) {
        let eq212_e2654: f64 = self.eval_ddt(111, s.v[312]);
        let eq212_e2654_d_n0: f64 = self.ddt_jacobian(s.dn[312][0]);
        let eq212_e2654_d_n1: f64 = self.ddt_jacobian(s.dn[312][1]);
        let eq212_e2654_d_n2: f64 = self.ddt_jacobian(s.dn[312][2]);
        let eq212_e2654_d_n3: f64 = self.ddt_jacobian(s.dn[312][3]);
        let eq212_e2654_d_n4: f64 = self.ddt_jacobian(s.dn[312][4]);
        let eq212_e2654_d_n5: f64 = self.ddt_jacobian(s.dn[312][5]);
        let eq212_e2654_d_n6: f64 = self.ddt_jacobian(s.dn[312][6]);
        let eq212_e2654_d_n7: f64 = self.ddt_jacobian(s.dn[312][7]);
        let eq212_e2654_d_n8: f64 = self.ddt_jacobian(s.dn[312][8]);
        let eq212_e2654_d_n9: f64 = self.ddt_jacobian(s.dn[312][9]);
        let eq212_e2654_d_n10: f64 = self.ddt_jacobian(s.dn[312][10]);
        let eq212_e2654_d_n11: f64 = self.ddt_jacobian(s.dn[312][11]);
        let eq212_e2654_d_n12: f64 = self.ddt_jacobian(s.dn[312][12]);
        let eq212_e2654_d_n13: f64 = self.ddt_jacobian(s.dn[312][13]);
        let eq212_e2654_d_n14: f64 = self.ddt_jacobian(s.dn[312][14]);
        let eq212_e2654_d_n15: f64 = self.ddt_jacobian(s.dn[312][15]);
        let eq212_e2654_d_n16: f64 = self.ddt_jacobian(s.dn[312][16]);
        let eq212_e2654_d_n17: f64 = self.ddt_jacobian(s.dn[312][17]);
        let eq212_e2654_d_n18: f64 = self.ddt_jacobian(s.dn[312][18]);
        let eq212_e2654_d_n19: f64 = self.ddt_jacobian(s.dn[312][19]);
        let eq212_e2654_d_n20: f64 = self.ddt_jacobian(s.dn[312][20]);
        let eq212_e2654_d_n21: f64 = self.ddt_jacobian(s.dn[312][21]);
        let eq212_e2654_d_n22: f64 = self.ddt_jacobian(s.dn[312][22]);
        let eq212_e2655: f64 = (p.p7 * eq212_e2654);
        let eq212_e2655_d_n0: f64 = (p.p7 * eq212_e2654_d_n0);
        let eq212_e2655_d_n1: f64 = (p.p7 * eq212_e2654_d_n1);
        let eq212_e2655_d_n2: f64 = (p.p7 * eq212_e2654_d_n2);
        let eq212_e2655_d_n3: f64 = (p.p7 * eq212_e2654_d_n3);
        let eq212_e2655_d_n4: f64 = (p.p7 * eq212_e2654_d_n4);
        let eq212_e2655_d_n5: f64 = (p.p7 * eq212_e2654_d_n5);
        let eq212_e2655_d_n6: f64 = (p.p7 * eq212_e2654_d_n6);
        let eq212_e2655_d_n7: f64 = (p.p7 * eq212_e2654_d_n7);
        let eq212_e2655_d_n8: f64 = (p.p7 * eq212_e2654_d_n8);
        let eq212_e2655_d_n9: f64 = (p.p7 * eq212_e2654_d_n9);
        let eq212_e2655_d_n10: f64 = (p.p7 * eq212_e2654_d_n10);
        let eq212_e2655_d_n11: f64 = (p.p7 * eq212_e2654_d_n11);
        let eq212_e2655_d_n12: f64 = (p.p7 * eq212_e2654_d_n12);
        let eq212_e2655_d_n13: f64 = (p.p7 * eq212_e2654_d_n13);
        let eq212_e2655_d_n14: f64 = (p.p7 * eq212_e2654_d_n14);
        let eq212_e2655_d_n15: f64 = (p.p7 * eq212_e2654_d_n15);
        let eq212_e2655_d_n16: f64 = (p.p7 * eq212_e2654_d_n16);
        let eq212_e2655_d_n17: f64 = (p.p7 * eq212_e2654_d_n17);
        let eq212_e2655_d_n18: f64 = (p.p7 * eq212_e2654_d_n18);
        let eq212_e2655_d_n19: f64 = (p.p7 * eq212_e2654_d_n19);
        let eq212_e2655_d_n20: f64 = (p.p7 * eq212_e2654_d_n20);
        let eq212_e2655_d_n21: f64 = (p.p7 * eq212_e2654_d_n21);
        let eq212_e2655_d_n22: f64 = (p.p7 * eq212_e2654_d_n22);
        let eq212_e2657: f64 = (eq212_e2655 * p.p249);
        let eq212_e2657_d_n0: f64 = (eq212_e2655_d_n0 * p.p249);
        let eq212_e2657_d_n1: f64 = (eq212_e2655_d_n1 * p.p249);
        let eq212_e2657_d_n2: f64 = (eq212_e2655_d_n2 * p.p249);
        let eq212_e2657_d_n3: f64 = (eq212_e2655_d_n3 * p.p249);
        let eq212_e2657_d_n4: f64 = (eq212_e2655_d_n4 * p.p249);
        let eq212_e2657_d_n5: f64 = (eq212_e2655_d_n5 * p.p249);
        let eq212_e2657_d_n6: f64 = (eq212_e2655_d_n6 * p.p249);
        let eq212_e2657_d_n7: f64 = (eq212_e2655_d_n7 * p.p249);
        let eq212_e2657_d_n8: f64 = (eq212_e2655_d_n8 * p.p249);
        let eq212_e2657_d_n9: f64 = (eq212_e2655_d_n9 * p.p249);
        let eq212_e2657_d_n10: f64 = (eq212_e2655_d_n10 * p.p249);
        let eq212_e2657_d_n11: f64 = (eq212_e2655_d_n11 * p.p249);
        let eq212_e2657_d_n12: f64 = (eq212_e2655_d_n12 * p.p249);
        let eq212_e2657_d_n13: f64 = (eq212_e2655_d_n13 * p.p249);
        let eq212_e2657_d_n14: f64 = (eq212_e2655_d_n14 * p.p249);
        let eq212_e2657_d_n15: f64 = (eq212_e2655_d_n15 * p.p249);
        let eq212_e2657_d_n16: f64 = (eq212_e2655_d_n16 * p.p249);
        let eq212_e2657_d_n17: f64 = (eq212_e2655_d_n17 * p.p249);
        let eq212_e2657_d_n18: f64 = (eq212_e2655_d_n18 * p.p249);
        let eq212_e2657_d_n19: f64 = (eq212_e2655_d_n19 * p.p249);
        let eq212_e2657_d_n20: f64 = (eq212_e2655_d_n20 * p.p249);
        let eq212_e2657_d_n21: f64 = (eq212_e2655_d_n21 * p.p249);
        let eq212_e2657_d_n22: f64 = (eq212_e2655_d_n22 * p.p249);
        (eq212_e2657, eq212_e2657_d_n0, eq212_e2657_d_n1, eq212_e2657_d_n2, eq212_e2657_d_n3, eq212_e2657_d_n4, eq212_e2657_d_n5, eq212_e2657_d_n6, eq212_e2657_d_n7, eq212_e2657_d_n8, eq212_e2657_d_n9, eq212_e2657_d_n10, eq212_e2657_d_n11, eq212_e2657_d_n12, eq212_e2657_d_n13, eq212_e2657_d_n14, eq212_e2657_d_n15, eq212_e2657_d_n16, eq212_e2657_d_n17, eq212_e2657_d_n18, eq212_e2657_d_n19, eq212_e2657_d_n20, eq212_e2657_d_n21, eq212_e2657_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq212_value: f64 = eq212_e2659;
        let eq212_node_derivatives: [f64; 23] = [eq212_e2659_d_n0, eq212_e2659_d_n1, eq212_e2659_d_n2, eq212_e2659_d_n3, eq212_e2659_d_n4, eq212_e2659_d_n5, eq212_e2659_d_n6, eq212_e2659_d_n7, eq212_e2659_d_n8, eq212_e2659_d_n9, eq212_e2659_d_n10, eq212_e2659_d_n11, eq212_e2659_d_n12, eq212_e2659_d_n13, eq212_e2659_d_n14, eq212_e2659_d_n15, eq212_e2659_d_n16, eq212_e2659_d_n17, eq212_e2659_d_n18, eq212_e2659_d_n19, eq212_e2659_d_n20, eq212_e2659_d_n21, eq212_e2659_d_n22];
        let eq212_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            self.multiplicity * (eq212_value),
            &nodes,
            &eq212_node_derivatives,
            &branches,
            &eq212_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_213_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq213_e2672, eq213_e2672_d_n0, eq213_e2672_d_n1, eq213_e2672_d_n2, eq213_e2672_d_n3, eq213_e2672_d_n4, eq213_e2672_d_n5, eq213_e2672_d_n6, eq213_e2672_d_n7, eq213_e2672_d_n8, eq213_e2672_d_n9, eq213_e2672_d_n10, eq213_e2672_d_n11, eq213_e2672_d_n12, eq213_e2672_d_n13, eq213_e2672_d_n14, eq213_e2672_d_n15, eq213_e2672_d_n16, eq213_e2672_d_n17, eq213_e2672_d_n18, eq213_e2672_d_n19, eq213_e2672_d_n20, eq213_e2672_d_n21, eq213_e2672_d_n22,) = {
    if (((!(s.v[605] != 0.0)) && (s.v[608] != 0.0)) && (!(s.v[609] != 0.0))) {
        let eq213_e2669: f64 = self.eval_ddt(112, s.v[312]);
        let eq213_e2669_d_n0: f64 = self.ddt_jacobian(s.dn[312][0]);
        let eq213_e2669_d_n1: f64 = self.ddt_jacobian(s.dn[312][1]);
        let eq213_e2669_d_n2: f64 = self.ddt_jacobian(s.dn[312][2]);
        let eq213_e2669_d_n3: f64 = self.ddt_jacobian(s.dn[312][3]);
        let eq213_e2669_d_n4: f64 = self.ddt_jacobian(s.dn[312][4]);
        let eq213_e2669_d_n5: f64 = self.ddt_jacobian(s.dn[312][5]);
        let eq213_e2669_d_n6: f64 = self.ddt_jacobian(s.dn[312][6]);
        let eq213_e2669_d_n7: f64 = self.ddt_jacobian(s.dn[312][7]);
        let eq213_e2669_d_n8: f64 = self.ddt_jacobian(s.dn[312][8]);
        let eq213_e2669_d_n9: f64 = self.ddt_jacobian(s.dn[312][9]);
        let eq213_e2669_d_n10: f64 = self.ddt_jacobian(s.dn[312][10]);
        let eq213_e2669_d_n11: f64 = self.ddt_jacobian(s.dn[312][11]);
        let eq213_e2669_d_n12: f64 = self.ddt_jacobian(s.dn[312][12]);
        let eq213_e2669_d_n13: f64 = self.ddt_jacobian(s.dn[312][13]);
        let eq213_e2669_d_n14: f64 = self.ddt_jacobian(s.dn[312][14]);
        let eq213_e2669_d_n15: f64 = self.ddt_jacobian(s.dn[312][15]);
        let eq213_e2669_d_n16: f64 = self.ddt_jacobian(s.dn[312][16]);
        let eq213_e2669_d_n17: f64 = self.ddt_jacobian(s.dn[312][17]);
        let eq213_e2669_d_n18: f64 = self.ddt_jacobian(s.dn[312][18]);
        let eq213_e2669_d_n19: f64 = self.ddt_jacobian(s.dn[312][19]);
        let eq213_e2669_d_n20: f64 = self.ddt_jacobian(s.dn[312][20]);
        let eq213_e2669_d_n21: f64 = self.ddt_jacobian(s.dn[312][21]);
        let eq213_e2669_d_n22: f64 = self.ddt_jacobian(s.dn[312][22]);
        let eq213_e2670: f64 = (p.p7 * eq213_e2669);
        let eq213_e2670_d_n0: f64 = (p.p7 * eq213_e2669_d_n0);
        let eq213_e2670_d_n1: f64 = (p.p7 * eq213_e2669_d_n1);
        let eq213_e2670_d_n2: f64 = (p.p7 * eq213_e2669_d_n2);
        let eq213_e2670_d_n3: f64 = (p.p7 * eq213_e2669_d_n3);
        let eq213_e2670_d_n4: f64 = (p.p7 * eq213_e2669_d_n4);
        let eq213_e2670_d_n5: f64 = (p.p7 * eq213_e2669_d_n5);
        let eq213_e2670_d_n6: f64 = (p.p7 * eq213_e2669_d_n6);
        let eq213_e2670_d_n7: f64 = (p.p7 * eq213_e2669_d_n7);
        let eq213_e2670_d_n8: f64 = (p.p7 * eq213_e2669_d_n8);
        let eq213_e2670_d_n9: f64 = (p.p7 * eq213_e2669_d_n9);
        let eq213_e2670_d_n10: f64 = (p.p7 * eq213_e2669_d_n10);
        let eq213_e2670_d_n11: f64 = (p.p7 * eq213_e2669_d_n11);
        let eq213_e2670_d_n12: f64 = (p.p7 * eq213_e2669_d_n12);
        let eq213_e2670_d_n13: f64 = (p.p7 * eq213_e2669_d_n13);
        let eq213_e2670_d_n14: f64 = (p.p7 * eq213_e2669_d_n14);
        let eq213_e2670_d_n15: f64 = (p.p7 * eq213_e2669_d_n15);
        let eq213_e2670_d_n16: f64 = (p.p7 * eq213_e2669_d_n16);
        let eq213_e2670_d_n17: f64 = (p.p7 * eq213_e2669_d_n17);
        let eq213_e2670_d_n18: f64 = (p.p7 * eq213_e2669_d_n18);
        let eq213_e2670_d_n19: f64 = (p.p7 * eq213_e2669_d_n19);
        let eq213_e2670_d_n20: f64 = (p.p7 * eq213_e2669_d_n20);
        let eq213_e2670_d_n21: f64 = (p.p7 * eq213_e2669_d_n21);
        let eq213_e2670_d_n22: f64 = (p.p7 * eq213_e2669_d_n22);
        (eq213_e2670, eq213_e2670_d_n0, eq213_e2670_d_n1, eq213_e2670_d_n2, eq213_e2670_d_n3, eq213_e2670_d_n4, eq213_e2670_d_n5, eq213_e2670_d_n6, eq213_e2670_d_n7, eq213_e2670_d_n8, eq213_e2670_d_n9, eq213_e2670_d_n10, eq213_e2670_d_n11, eq213_e2670_d_n12, eq213_e2670_d_n13, eq213_e2670_d_n14, eq213_e2670_d_n15, eq213_e2670_d_n16, eq213_e2670_d_n17, eq213_e2670_d_n18, eq213_e2670_d_n19, eq213_e2670_d_n20, eq213_e2670_d_n21, eq213_e2670_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq213_value: f64 = eq213_e2672;
        let eq213_node_derivatives: [f64; 23] = [eq213_e2672_d_n0, eq213_e2672_d_n1, eq213_e2672_d_n2, eq213_e2672_d_n3, eq213_e2672_d_n4, eq213_e2672_d_n5, eq213_e2672_d_n6, eq213_e2672_d_n7, eq213_e2672_d_n8, eq213_e2672_d_n9, eq213_e2672_d_n10, eq213_e2672_d_n11, eq213_e2672_d_n12, eq213_e2672_d_n13, eq213_e2672_d_n14, eq213_e2672_d_n15, eq213_e2672_d_n16, eq213_e2672_d_n17, eq213_e2672_d_n18, eq213_e2672_d_n19, eq213_e2672_d_n20, eq213_e2672_d_n21, eq213_e2672_d_n22];
        let eq213_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            self.multiplicity * (eq213_value),
            &nodes,
            &eq213_node_derivatives,
            &branches,
            &eq213_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_214_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq214_e2687, eq214_e2687_d_n0, eq214_e2687_d_n1, eq214_e2687_d_n2, eq214_e2687_d_n3, eq214_e2687_d_n4, eq214_e2687_d_n5, eq214_e2687_d_n6, eq214_e2687_d_n7, eq214_e2687_d_n8, eq214_e2687_d_n9, eq214_e2687_d_n10, eq214_e2687_d_n11, eq214_e2687_d_n12, eq214_e2687_d_n13, eq214_e2687_d_n14, eq214_e2687_d_n15, eq214_e2687_d_n16, eq214_e2687_d_n17, eq214_e2687_d_n18, eq214_e2687_d_n19, eq214_e2687_d_n20, eq214_e2687_d_n21, eq214_e2687_d_n22,) = {
    if (((!(s.v[605] != 0.0)) && (s.v[608] != 0.0)) && (!(s.v[609] != 0.0))) {
        let eq214_e2682: f64 = self.eval_ddt(113, s.v[312]);
        let eq214_e2682_d_n0: f64 = self.ddt_jacobian(s.dn[312][0]);
        let eq214_e2682_d_n1: f64 = self.ddt_jacobian(s.dn[312][1]);
        let eq214_e2682_d_n2: f64 = self.ddt_jacobian(s.dn[312][2]);
        let eq214_e2682_d_n3: f64 = self.ddt_jacobian(s.dn[312][3]);
        let eq214_e2682_d_n4: f64 = self.ddt_jacobian(s.dn[312][4]);
        let eq214_e2682_d_n5: f64 = self.ddt_jacobian(s.dn[312][5]);
        let eq214_e2682_d_n6: f64 = self.ddt_jacobian(s.dn[312][6]);
        let eq214_e2682_d_n7: f64 = self.ddt_jacobian(s.dn[312][7]);
        let eq214_e2682_d_n8: f64 = self.ddt_jacobian(s.dn[312][8]);
        let eq214_e2682_d_n9: f64 = self.ddt_jacobian(s.dn[312][9]);
        let eq214_e2682_d_n10: f64 = self.ddt_jacobian(s.dn[312][10]);
        let eq214_e2682_d_n11: f64 = self.ddt_jacobian(s.dn[312][11]);
        let eq214_e2682_d_n12: f64 = self.ddt_jacobian(s.dn[312][12]);
        let eq214_e2682_d_n13: f64 = self.ddt_jacobian(s.dn[312][13]);
        let eq214_e2682_d_n14: f64 = self.ddt_jacobian(s.dn[312][14]);
        let eq214_e2682_d_n15: f64 = self.ddt_jacobian(s.dn[312][15]);
        let eq214_e2682_d_n16: f64 = self.ddt_jacobian(s.dn[312][16]);
        let eq214_e2682_d_n17: f64 = self.ddt_jacobian(s.dn[312][17]);
        let eq214_e2682_d_n18: f64 = self.ddt_jacobian(s.dn[312][18]);
        let eq214_e2682_d_n19: f64 = self.ddt_jacobian(s.dn[312][19]);
        let eq214_e2682_d_n20: f64 = self.ddt_jacobian(s.dn[312][20]);
        let eq214_e2682_d_n21: f64 = self.ddt_jacobian(s.dn[312][21]);
        let eq214_e2682_d_n22: f64 = self.ddt_jacobian(s.dn[312][22]);
        let eq214_e2683: f64 = (p.p7 * eq214_e2682);
        let eq214_e2683_d_n0: f64 = (p.p7 * eq214_e2682_d_n0);
        let eq214_e2683_d_n1: f64 = (p.p7 * eq214_e2682_d_n1);
        let eq214_e2683_d_n2: f64 = (p.p7 * eq214_e2682_d_n2);
        let eq214_e2683_d_n3: f64 = (p.p7 * eq214_e2682_d_n3);
        let eq214_e2683_d_n4: f64 = (p.p7 * eq214_e2682_d_n4);
        let eq214_e2683_d_n5: f64 = (p.p7 * eq214_e2682_d_n5);
        let eq214_e2683_d_n6: f64 = (p.p7 * eq214_e2682_d_n6);
        let eq214_e2683_d_n7: f64 = (p.p7 * eq214_e2682_d_n7);
        let eq214_e2683_d_n8: f64 = (p.p7 * eq214_e2682_d_n8);
        let eq214_e2683_d_n9: f64 = (p.p7 * eq214_e2682_d_n9);
        let eq214_e2683_d_n10: f64 = (p.p7 * eq214_e2682_d_n10);
        let eq214_e2683_d_n11: f64 = (p.p7 * eq214_e2682_d_n11);
        let eq214_e2683_d_n12: f64 = (p.p7 * eq214_e2682_d_n12);
        let eq214_e2683_d_n13: f64 = (p.p7 * eq214_e2682_d_n13);
        let eq214_e2683_d_n14: f64 = (p.p7 * eq214_e2682_d_n14);
        let eq214_e2683_d_n15: f64 = (p.p7 * eq214_e2682_d_n15);
        let eq214_e2683_d_n16: f64 = (p.p7 * eq214_e2682_d_n16);
        let eq214_e2683_d_n17: f64 = (p.p7 * eq214_e2682_d_n17);
        let eq214_e2683_d_n18: f64 = (p.p7 * eq214_e2682_d_n18);
        let eq214_e2683_d_n19: f64 = (p.p7 * eq214_e2682_d_n19);
        let eq214_e2683_d_n20: f64 = (p.p7 * eq214_e2682_d_n20);
        let eq214_e2683_d_n21: f64 = (p.p7 * eq214_e2682_d_n21);
        let eq214_e2683_d_n22: f64 = (p.p7 * eq214_e2682_d_n22);
        let eq214_e2685: f64 = (eq214_e2683 * p.p249);
        let eq214_e2685_d_n0: f64 = (eq214_e2683_d_n0 * p.p249);
        let eq214_e2685_d_n1: f64 = (eq214_e2683_d_n1 * p.p249);
        let eq214_e2685_d_n2: f64 = (eq214_e2683_d_n2 * p.p249);
        let eq214_e2685_d_n3: f64 = (eq214_e2683_d_n3 * p.p249);
        let eq214_e2685_d_n4: f64 = (eq214_e2683_d_n4 * p.p249);
        let eq214_e2685_d_n5: f64 = (eq214_e2683_d_n5 * p.p249);
        let eq214_e2685_d_n6: f64 = (eq214_e2683_d_n6 * p.p249);
        let eq214_e2685_d_n7: f64 = (eq214_e2683_d_n7 * p.p249);
        let eq214_e2685_d_n8: f64 = (eq214_e2683_d_n8 * p.p249);
        let eq214_e2685_d_n9: f64 = (eq214_e2683_d_n9 * p.p249);
        let eq214_e2685_d_n10: f64 = (eq214_e2683_d_n10 * p.p249);
        let eq214_e2685_d_n11: f64 = (eq214_e2683_d_n11 * p.p249);
        let eq214_e2685_d_n12: f64 = (eq214_e2683_d_n12 * p.p249);
        let eq214_e2685_d_n13: f64 = (eq214_e2683_d_n13 * p.p249);
        let eq214_e2685_d_n14: f64 = (eq214_e2683_d_n14 * p.p249);
        let eq214_e2685_d_n15: f64 = (eq214_e2683_d_n15 * p.p249);
        let eq214_e2685_d_n16: f64 = (eq214_e2683_d_n16 * p.p249);
        let eq214_e2685_d_n17: f64 = (eq214_e2683_d_n17 * p.p249);
        let eq214_e2685_d_n18: f64 = (eq214_e2683_d_n18 * p.p249);
        let eq214_e2685_d_n19: f64 = (eq214_e2683_d_n19 * p.p249);
        let eq214_e2685_d_n20: f64 = (eq214_e2683_d_n20 * p.p249);
        let eq214_e2685_d_n21: f64 = (eq214_e2683_d_n21 * p.p249);
        let eq214_e2685_d_n22: f64 = (eq214_e2683_d_n22 * p.p249);
        (eq214_e2685, eq214_e2685_d_n0, eq214_e2685_d_n1, eq214_e2685_d_n2, eq214_e2685_d_n3, eq214_e2685_d_n4, eq214_e2685_d_n5, eq214_e2685_d_n6, eq214_e2685_d_n7, eq214_e2685_d_n8, eq214_e2685_d_n9, eq214_e2685_d_n10, eq214_e2685_d_n11, eq214_e2685_d_n12, eq214_e2685_d_n13, eq214_e2685_d_n14, eq214_e2685_d_n15, eq214_e2685_d_n16, eq214_e2685_d_n17, eq214_e2685_d_n18, eq214_e2685_d_n19, eq214_e2685_d_n20, eq214_e2685_d_n21, eq214_e2685_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq214_value: f64 = eq214_e2687;
        let eq214_node_derivatives: [f64; 23] = [eq214_e2687_d_n0, eq214_e2687_d_n1, eq214_e2687_d_n2, eq214_e2687_d_n3, eq214_e2687_d_n4, eq214_e2687_d_n5, eq214_e2687_d_n6, eq214_e2687_d_n7, eq214_e2687_d_n8, eq214_e2687_d_n9, eq214_e2687_d_n10, eq214_e2687_d_n11, eq214_e2687_d_n12, eq214_e2687_d_n13, eq214_e2687_d_n14, eq214_e2687_d_n15, eq214_e2687_d_n16, eq214_e2687_d_n17, eq214_e2687_d_n18, eq214_e2687_d_n19, eq214_e2687_d_n20, eq214_e2687_d_n21, eq214_e2687_d_n22];
        let eq214_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            self.multiplicity * (eq214_value),
            &nodes,
            &eq214_node_derivatives,
            &branches,
            &eq214_branch_derivatives,
            self.multiplicity,
        );
    }
}
