#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_183_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq183_e2306, eq183_e2306_d_n0, eq183_e2306_d_n1, eq183_e2306_d_n2, eq183_e2306_d_n3, eq183_e2306_d_n4, eq183_e2306_d_n5, eq183_e2306_d_n6, eq183_e2306_d_n7, eq183_e2306_d_n8, eq183_e2306_d_n9, eq183_e2306_d_n10, eq183_e2306_d_n11, eq183_e2306_d_n12, eq183_e2306_d_n13, eq183_e2306_d_n14, eq183_e2306_d_n15, eq183_e2306_d_n16, eq183_e2306_d_n17, eq183_e2306_d_n18, eq183_e2306_d_n19, eq183_e2306_d_n20, eq183_e2306_d_n21, eq183_e2306_d_n22,) = {
    if (((s.v[595] != 0.0) && (s.v[596] != 0.0)) && (!(s.v[597] != 0.0))) {
        let eq183_e2303: f64 = self.eval_ddt(82, s.v[288]);
        let eq183_e2303_d_n0: f64 = self.ddt_jacobian(s.dn[288][0]);
        let eq183_e2303_d_n1: f64 = self.ddt_jacobian(s.dn[288][1]);
        let eq183_e2303_d_n2: f64 = self.ddt_jacobian(s.dn[288][2]);
        let eq183_e2303_d_n3: f64 = self.ddt_jacobian(s.dn[288][3]);
        let eq183_e2303_d_n4: f64 = self.ddt_jacobian(s.dn[288][4]);
        let eq183_e2303_d_n5: f64 = self.ddt_jacobian(s.dn[288][5]);
        let eq183_e2303_d_n6: f64 = self.ddt_jacobian(s.dn[288][6]);
        let eq183_e2303_d_n7: f64 = self.ddt_jacobian(s.dn[288][7]);
        let eq183_e2303_d_n8: f64 = self.ddt_jacobian(s.dn[288][8]);
        let eq183_e2303_d_n9: f64 = self.ddt_jacobian(s.dn[288][9]);
        let eq183_e2303_d_n10: f64 = self.ddt_jacobian(s.dn[288][10]);
        let eq183_e2303_d_n11: f64 = self.ddt_jacobian(s.dn[288][11]);
        let eq183_e2303_d_n12: f64 = self.ddt_jacobian(s.dn[288][12]);
        let eq183_e2303_d_n13: f64 = self.ddt_jacobian(s.dn[288][13]);
        let eq183_e2303_d_n14: f64 = self.ddt_jacobian(s.dn[288][14]);
        let eq183_e2303_d_n15: f64 = self.ddt_jacobian(s.dn[288][15]);
        let eq183_e2303_d_n16: f64 = self.ddt_jacobian(s.dn[288][16]);
        let eq183_e2303_d_n17: f64 = self.ddt_jacobian(s.dn[288][17]);
        let eq183_e2303_d_n18: f64 = self.ddt_jacobian(s.dn[288][18]);
        let eq183_e2303_d_n19: f64 = self.ddt_jacobian(s.dn[288][19]);
        let eq183_e2303_d_n20: f64 = self.ddt_jacobian(s.dn[288][20]);
        let eq183_e2303_d_n21: f64 = self.ddt_jacobian(s.dn[288][21]);
        let eq183_e2303_d_n22: f64 = self.ddt_jacobian(s.dn[288][22]);
        let eq183_e2304: f64 = (p.p7 * eq183_e2303);
        let eq183_e2304_d_n0: f64 = (p.p7 * eq183_e2303_d_n0);
        let eq183_e2304_d_n1: f64 = (p.p7 * eq183_e2303_d_n1);
        let eq183_e2304_d_n2: f64 = (p.p7 * eq183_e2303_d_n2);
        let eq183_e2304_d_n3: f64 = (p.p7 * eq183_e2303_d_n3);
        let eq183_e2304_d_n4: f64 = (p.p7 * eq183_e2303_d_n4);
        let eq183_e2304_d_n5: f64 = (p.p7 * eq183_e2303_d_n5);
        let eq183_e2304_d_n6: f64 = (p.p7 * eq183_e2303_d_n6);
        let eq183_e2304_d_n7: f64 = (p.p7 * eq183_e2303_d_n7);
        let eq183_e2304_d_n8: f64 = (p.p7 * eq183_e2303_d_n8);
        let eq183_e2304_d_n9: f64 = (p.p7 * eq183_e2303_d_n9);
        let eq183_e2304_d_n10: f64 = (p.p7 * eq183_e2303_d_n10);
        let eq183_e2304_d_n11: f64 = (p.p7 * eq183_e2303_d_n11);
        let eq183_e2304_d_n12: f64 = (p.p7 * eq183_e2303_d_n12);
        let eq183_e2304_d_n13: f64 = (p.p7 * eq183_e2303_d_n13);
        let eq183_e2304_d_n14: f64 = (p.p7 * eq183_e2303_d_n14);
        let eq183_e2304_d_n15: f64 = (p.p7 * eq183_e2303_d_n15);
        let eq183_e2304_d_n16: f64 = (p.p7 * eq183_e2303_d_n16);
        let eq183_e2304_d_n17: f64 = (p.p7 * eq183_e2303_d_n17);
        let eq183_e2304_d_n18: f64 = (p.p7 * eq183_e2303_d_n18);
        let eq183_e2304_d_n19: f64 = (p.p7 * eq183_e2303_d_n19);
        let eq183_e2304_d_n20: f64 = (p.p7 * eq183_e2303_d_n20);
        let eq183_e2304_d_n21: f64 = (p.p7 * eq183_e2303_d_n21);
        let eq183_e2304_d_n22: f64 = (p.p7 * eq183_e2303_d_n22);
        (eq183_e2304, eq183_e2304_d_n0, eq183_e2304_d_n1, eq183_e2304_d_n2, eq183_e2304_d_n3, eq183_e2304_d_n4, eq183_e2304_d_n5, eq183_e2304_d_n6, eq183_e2304_d_n7, eq183_e2304_d_n8, eq183_e2304_d_n9, eq183_e2304_d_n10, eq183_e2304_d_n11, eq183_e2304_d_n12, eq183_e2304_d_n13, eq183_e2304_d_n14, eq183_e2304_d_n15, eq183_e2304_d_n16, eq183_e2304_d_n17, eq183_e2304_d_n18, eq183_e2304_d_n19, eq183_e2304_d_n20, eq183_e2304_d_n21, eq183_e2304_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq183_value: f64 = eq183_e2306;
        let eq183_node_derivatives: [f64; 23] = [eq183_e2306_d_n0, eq183_e2306_d_n1, eq183_e2306_d_n2, eq183_e2306_d_n3, eq183_e2306_d_n4, eq183_e2306_d_n5, eq183_e2306_d_n6, eq183_e2306_d_n7, eq183_e2306_d_n8, eq183_e2306_d_n9, eq183_e2306_d_n10, eq183_e2306_d_n11, eq183_e2306_d_n12, eq183_e2306_d_n13, eq183_e2306_d_n14, eq183_e2306_d_n15, eq183_e2306_d_n16, eq183_e2306_d_n17, eq183_e2306_d_n18, eq183_e2306_d_n19, eq183_e2306_d_n20, eq183_e2306_d_n21, eq183_e2306_d_n22];
        let eq183_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[21]),
            self.multiplicity * (eq183_value),
            &nodes,
            &eq183_node_derivatives,
            &branches,
            &eq183_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_184_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq184_e2320, eq184_e2320_d_n0, eq184_e2320_d_n1, eq184_e2320_d_n2, eq184_e2320_d_n3, eq184_e2320_d_n4, eq184_e2320_d_n5, eq184_e2320_d_n6, eq184_e2320_d_n7, eq184_e2320_d_n8, eq184_e2320_d_n9, eq184_e2320_d_n10, eq184_e2320_d_n11, eq184_e2320_d_n12, eq184_e2320_d_n13, eq184_e2320_d_n14, eq184_e2320_d_n15, eq184_e2320_d_n16, eq184_e2320_d_n17, eq184_e2320_d_n18, eq184_e2320_d_n19, eq184_e2320_d_n20, eq184_e2320_d_n21, eq184_e2320_d_n22,) = {
    if (((s.v[595] != 0.0) && (s.v[596] != 0.0)) && (!(s.v[597] != 0.0))) {
        let eq184_e2315: f64 = self.eval_ddt(83, s.v[288]);
        let eq184_e2315_d_n0: f64 = self.ddt_jacobian(s.dn[288][0]);
        let eq184_e2315_d_n1: f64 = self.ddt_jacobian(s.dn[288][1]);
        let eq184_e2315_d_n2: f64 = self.ddt_jacobian(s.dn[288][2]);
        let eq184_e2315_d_n3: f64 = self.ddt_jacobian(s.dn[288][3]);
        let eq184_e2315_d_n4: f64 = self.ddt_jacobian(s.dn[288][4]);
        let eq184_e2315_d_n5: f64 = self.ddt_jacobian(s.dn[288][5]);
        let eq184_e2315_d_n6: f64 = self.ddt_jacobian(s.dn[288][6]);
        let eq184_e2315_d_n7: f64 = self.ddt_jacobian(s.dn[288][7]);
        let eq184_e2315_d_n8: f64 = self.ddt_jacobian(s.dn[288][8]);
        let eq184_e2315_d_n9: f64 = self.ddt_jacobian(s.dn[288][9]);
        let eq184_e2315_d_n10: f64 = self.ddt_jacobian(s.dn[288][10]);
        let eq184_e2315_d_n11: f64 = self.ddt_jacobian(s.dn[288][11]);
        let eq184_e2315_d_n12: f64 = self.ddt_jacobian(s.dn[288][12]);
        let eq184_e2315_d_n13: f64 = self.ddt_jacobian(s.dn[288][13]);
        let eq184_e2315_d_n14: f64 = self.ddt_jacobian(s.dn[288][14]);
        let eq184_e2315_d_n15: f64 = self.ddt_jacobian(s.dn[288][15]);
        let eq184_e2315_d_n16: f64 = self.ddt_jacobian(s.dn[288][16]);
        let eq184_e2315_d_n17: f64 = self.ddt_jacobian(s.dn[288][17]);
        let eq184_e2315_d_n18: f64 = self.ddt_jacobian(s.dn[288][18]);
        let eq184_e2315_d_n19: f64 = self.ddt_jacobian(s.dn[288][19]);
        let eq184_e2315_d_n20: f64 = self.ddt_jacobian(s.dn[288][20]);
        let eq184_e2315_d_n21: f64 = self.ddt_jacobian(s.dn[288][21]);
        let eq184_e2315_d_n22: f64 = self.ddt_jacobian(s.dn[288][22]);
        let eq184_e2316: f64 = (p.p7 * eq184_e2315);
        let eq184_e2316_d_n0: f64 = (p.p7 * eq184_e2315_d_n0);
        let eq184_e2316_d_n1: f64 = (p.p7 * eq184_e2315_d_n1);
        let eq184_e2316_d_n2: f64 = (p.p7 * eq184_e2315_d_n2);
        let eq184_e2316_d_n3: f64 = (p.p7 * eq184_e2315_d_n3);
        let eq184_e2316_d_n4: f64 = (p.p7 * eq184_e2315_d_n4);
        let eq184_e2316_d_n5: f64 = (p.p7 * eq184_e2315_d_n5);
        let eq184_e2316_d_n6: f64 = (p.p7 * eq184_e2315_d_n6);
        let eq184_e2316_d_n7: f64 = (p.p7 * eq184_e2315_d_n7);
        let eq184_e2316_d_n8: f64 = (p.p7 * eq184_e2315_d_n8);
        let eq184_e2316_d_n9: f64 = (p.p7 * eq184_e2315_d_n9);
        let eq184_e2316_d_n10: f64 = (p.p7 * eq184_e2315_d_n10);
        let eq184_e2316_d_n11: f64 = (p.p7 * eq184_e2315_d_n11);
        let eq184_e2316_d_n12: f64 = (p.p7 * eq184_e2315_d_n12);
        let eq184_e2316_d_n13: f64 = (p.p7 * eq184_e2315_d_n13);
        let eq184_e2316_d_n14: f64 = (p.p7 * eq184_e2315_d_n14);
        let eq184_e2316_d_n15: f64 = (p.p7 * eq184_e2315_d_n15);
        let eq184_e2316_d_n16: f64 = (p.p7 * eq184_e2315_d_n16);
        let eq184_e2316_d_n17: f64 = (p.p7 * eq184_e2315_d_n17);
        let eq184_e2316_d_n18: f64 = (p.p7 * eq184_e2315_d_n18);
        let eq184_e2316_d_n19: f64 = (p.p7 * eq184_e2315_d_n19);
        let eq184_e2316_d_n20: f64 = (p.p7 * eq184_e2315_d_n20);
        let eq184_e2316_d_n21: f64 = (p.p7 * eq184_e2315_d_n21);
        let eq184_e2316_d_n22: f64 = (p.p7 * eq184_e2315_d_n22);
        let eq184_e2318: f64 = (eq184_e2316 * p.p248);
        let eq184_e2318_d_n0: f64 = (eq184_e2316_d_n0 * p.p248);
        let eq184_e2318_d_n1: f64 = (eq184_e2316_d_n1 * p.p248);
        let eq184_e2318_d_n2: f64 = (eq184_e2316_d_n2 * p.p248);
        let eq184_e2318_d_n3: f64 = (eq184_e2316_d_n3 * p.p248);
        let eq184_e2318_d_n4: f64 = (eq184_e2316_d_n4 * p.p248);
        let eq184_e2318_d_n5: f64 = (eq184_e2316_d_n5 * p.p248);
        let eq184_e2318_d_n6: f64 = (eq184_e2316_d_n6 * p.p248);
        let eq184_e2318_d_n7: f64 = (eq184_e2316_d_n7 * p.p248);
        let eq184_e2318_d_n8: f64 = (eq184_e2316_d_n8 * p.p248);
        let eq184_e2318_d_n9: f64 = (eq184_e2316_d_n9 * p.p248);
        let eq184_e2318_d_n10: f64 = (eq184_e2316_d_n10 * p.p248);
        let eq184_e2318_d_n11: f64 = (eq184_e2316_d_n11 * p.p248);
        let eq184_e2318_d_n12: f64 = (eq184_e2316_d_n12 * p.p248);
        let eq184_e2318_d_n13: f64 = (eq184_e2316_d_n13 * p.p248);
        let eq184_e2318_d_n14: f64 = (eq184_e2316_d_n14 * p.p248);
        let eq184_e2318_d_n15: f64 = (eq184_e2316_d_n15 * p.p248);
        let eq184_e2318_d_n16: f64 = (eq184_e2316_d_n16 * p.p248);
        let eq184_e2318_d_n17: f64 = (eq184_e2316_d_n17 * p.p248);
        let eq184_e2318_d_n18: f64 = (eq184_e2316_d_n18 * p.p248);
        let eq184_e2318_d_n19: f64 = (eq184_e2316_d_n19 * p.p248);
        let eq184_e2318_d_n20: f64 = (eq184_e2316_d_n20 * p.p248);
        let eq184_e2318_d_n21: f64 = (eq184_e2316_d_n21 * p.p248);
        let eq184_e2318_d_n22: f64 = (eq184_e2316_d_n22 * p.p248);
        (eq184_e2318, eq184_e2318_d_n0, eq184_e2318_d_n1, eq184_e2318_d_n2, eq184_e2318_d_n3, eq184_e2318_d_n4, eq184_e2318_d_n5, eq184_e2318_d_n6, eq184_e2318_d_n7, eq184_e2318_d_n8, eq184_e2318_d_n9, eq184_e2318_d_n10, eq184_e2318_d_n11, eq184_e2318_d_n12, eq184_e2318_d_n13, eq184_e2318_d_n14, eq184_e2318_d_n15, eq184_e2318_d_n16, eq184_e2318_d_n17, eq184_e2318_d_n18, eq184_e2318_d_n19, eq184_e2318_d_n20, eq184_e2318_d_n21, eq184_e2318_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq184_value: f64 = eq184_e2320;
        let eq184_node_derivatives: [f64; 23] = [eq184_e2320_d_n0, eq184_e2320_d_n1, eq184_e2320_d_n2, eq184_e2320_d_n3, eq184_e2320_d_n4, eq184_e2320_d_n5, eq184_e2320_d_n6, eq184_e2320_d_n7, eq184_e2320_d_n8, eq184_e2320_d_n9, eq184_e2320_d_n10, eq184_e2320_d_n11, eq184_e2320_d_n12, eq184_e2320_d_n13, eq184_e2320_d_n14, eq184_e2320_d_n15, eq184_e2320_d_n16, eq184_e2320_d_n17, eq184_e2320_d_n18, eq184_e2320_d_n19, eq184_e2320_d_n20, eq184_e2320_d_n21, eq184_e2320_d_n22];
        let eq184_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[21]),
            self.multiplicity * (eq184_value),
            &nodes,
            &eq184_node_derivatives,
            &branches,
            &eq184_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_185_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq185_e2331, eq185_e2331_d_n0, eq185_e2331_d_n1, eq185_e2331_d_n2, eq185_e2331_d_n3, eq185_e2331_d_n4, eq185_e2331_d_n5, eq185_e2331_d_n6, eq185_e2331_d_n7, eq185_e2331_d_n8, eq185_e2331_d_n9, eq185_e2331_d_n10, eq185_e2331_d_n11, eq185_e2331_d_n12, eq185_e2331_d_n13, eq185_e2331_d_n14, eq185_e2331_d_n15, eq185_e2331_d_n16, eq185_e2331_d_n17, eq185_e2331_d_n18, eq185_e2331_d_n19, eq185_e2331_d_n20, eq185_e2331_d_n21, eq185_e2331_d_n22,) = {
    if ((s.v[595] != 0.0) && (s.v[596] != 0.0)) {
        let eq185_e2327: f64 = (p.p253 * s.v[288]);
        let eq185_e2327_d_n0: f64 = (p.p253 * s.dn[288][0]);
        let eq185_e2327_d_n1: f64 = (p.p253 * s.dn[288][1]);
        let eq185_e2327_d_n2: f64 = (p.p253 * s.dn[288][2]);
        let eq185_e2327_d_n3: f64 = (p.p253 * s.dn[288][3]);
        let eq185_e2327_d_n4: f64 = (p.p253 * s.dn[288][4]);
        let eq185_e2327_d_n5: f64 = (p.p253 * s.dn[288][5]);
        let eq185_e2327_d_n6: f64 = (p.p253 * s.dn[288][6]);
        let eq185_e2327_d_n7: f64 = (p.p253 * s.dn[288][7]);
        let eq185_e2327_d_n8: f64 = (p.p253 * s.dn[288][8]);
        let eq185_e2327_d_n9: f64 = (p.p253 * s.dn[288][9]);
        let eq185_e2327_d_n10: f64 = (p.p253 * s.dn[288][10]);
        let eq185_e2327_d_n11: f64 = (p.p253 * s.dn[288][11]);
        let eq185_e2327_d_n12: f64 = (p.p253 * s.dn[288][12]);
        let eq185_e2327_d_n13: f64 = (p.p253 * s.dn[288][13]);
        let eq185_e2327_d_n14: f64 = (p.p253 * s.dn[288][14]);
        let eq185_e2327_d_n15: f64 = (p.p253 * s.dn[288][15]);
        let eq185_e2327_d_n16: f64 = (p.p253 * s.dn[288][16]);
        let eq185_e2327_d_n17: f64 = (p.p253 * s.dn[288][17]);
        let eq185_e2327_d_n18: f64 = (p.p253 * s.dn[288][18]);
        let eq185_e2327_d_n19: f64 = (p.p253 * s.dn[288][19]);
        let eq185_e2327_d_n20: f64 = (p.p253 * s.dn[288][20]);
        let eq185_e2327_d_n21: f64 = (p.p253 * s.dn[288][21]);
        let eq185_e2327_d_n22: f64 = (p.p253 * s.dn[288][22]);
        let eq185_e2328: f64 = self.eval_ddt(84, eq185_e2327);
        let eq185_e2328_d_n0: f64 = self.ddt_jacobian(eq185_e2327_d_n0);
        let eq185_e2328_d_n1: f64 = self.ddt_jacobian(eq185_e2327_d_n1);
        let eq185_e2328_d_n2: f64 = self.ddt_jacobian(eq185_e2327_d_n2);
        let eq185_e2328_d_n3: f64 = self.ddt_jacobian(eq185_e2327_d_n3);
        let eq185_e2328_d_n4: f64 = self.ddt_jacobian(eq185_e2327_d_n4);
        let eq185_e2328_d_n5: f64 = self.ddt_jacobian(eq185_e2327_d_n5);
        let eq185_e2328_d_n6: f64 = self.ddt_jacobian(eq185_e2327_d_n6);
        let eq185_e2328_d_n7: f64 = self.ddt_jacobian(eq185_e2327_d_n7);
        let eq185_e2328_d_n8: f64 = self.ddt_jacobian(eq185_e2327_d_n8);
        let eq185_e2328_d_n9: f64 = self.ddt_jacobian(eq185_e2327_d_n9);
        let eq185_e2328_d_n10: f64 = self.ddt_jacobian(eq185_e2327_d_n10);
        let eq185_e2328_d_n11: f64 = self.ddt_jacobian(eq185_e2327_d_n11);
        let eq185_e2328_d_n12: f64 = self.ddt_jacobian(eq185_e2327_d_n12);
        let eq185_e2328_d_n13: f64 = self.ddt_jacobian(eq185_e2327_d_n13);
        let eq185_e2328_d_n14: f64 = self.ddt_jacobian(eq185_e2327_d_n14);
        let eq185_e2328_d_n15: f64 = self.ddt_jacobian(eq185_e2327_d_n15);
        let eq185_e2328_d_n16: f64 = self.ddt_jacobian(eq185_e2327_d_n16);
        let eq185_e2328_d_n17: f64 = self.ddt_jacobian(eq185_e2327_d_n17);
        let eq185_e2328_d_n18: f64 = self.ddt_jacobian(eq185_e2327_d_n18);
        let eq185_e2328_d_n19: f64 = self.ddt_jacobian(eq185_e2327_d_n19);
        let eq185_e2328_d_n20: f64 = self.ddt_jacobian(eq185_e2327_d_n20);
        let eq185_e2328_d_n21: f64 = self.ddt_jacobian(eq185_e2327_d_n21);
        let eq185_e2328_d_n22: f64 = self.ddt_jacobian(eq185_e2327_d_n22);
        let eq185_e2329: f64 = (p.p7 * eq185_e2328);
        let eq185_e2329_d_n0: f64 = (p.p7 * eq185_e2328_d_n0);
        let eq185_e2329_d_n1: f64 = (p.p7 * eq185_e2328_d_n1);
        let eq185_e2329_d_n2: f64 = (p.p7 * eq185_e2328_d_n2);
        let eq185_e2329_d_n3: f64 = (p.p7 * eq185_e2328_d_n3);
        let eq185_e2329_d_n4: f64 = (p.p7 * eq185_e2328_d_n4);
        let eq185_e2329_d_n5: f64 = (p.p7 * eq185_e2328_d_n5);
        let eq185_e2329_d_n6: f64 = (p.p7 * eq185_e2328_d_n6);
        let eq185_e2329_d_n7: f64 = (p.p7 * eq185_e2328_d_n7);
        let eq185_e2329_d_n8: f64 = (p.p7 * eq185_e2328_d_n8);
        let eq185_e2329_d_n9: f64 = (p.p7 * eq185_e2328_d_n9);
        let eq185_e2329_d_n10: f64 = (p.p7 * eq185_e2328_d_n10);
        let eq185_e2329_d_n11: f64 = (p.p7 * eq185_e2328_d_n11);
        let eq185_e2329_d_n12: f64 = (p.p7 * eq185_e2328_d_n12);
        let eq185_e2329_d_n13: f64 = (p.p7 * eq185_e2328_d_n13);
        let eq185_e2329_d_n14: f64 = (p.p7 * eq185_e2328_d_n14);
        let eq185_e2329_d_n15: f64 = (p.p7 * eq185_e2328_d_n15);
        let eq185_e2329_d_n16: f64 = (p.p7 * eq185_e2328_d_n16);
        let eq185_e2329_d_n17: f64 = (p.p7 * eq185_e2328_d_n17);
        let eq185_e2329_d_n18: f64 = (p.p7 * eq185_e2328_d_n18);
        let eq185_e2329_d_n19: f64 = (p.p7 * eq185_e2328_d_n19);
        let eq185_e2329_d_n20: f64 = (p.p7 * eq185_e2328_d_n20);
        let eq185_e2329_d_n21: f64 = (p.p7 * eq185_e2328_d_n21);
        let eq185_e2329_d_n22: f64 = (p.p7 * eq185_e2328_d_n22);
        (eq185_e2329, eq185_e2329_d_n0, eq185_e2329_d_n1, eq185_e2329_d_n2, eq185_e2329_d_n3, eq185_e2329_d_n4, eq185_e2329_d_n5, eq185_e2329_d_n6, eq185_e2329_d_n7, eq185_e2329_d_n8, eq185_e2329_d_n9, eq185_e2329_d_n10, eq185_e2329_d_n11, eq185_e2329_d_n12, eq185_e2329_d_n13, eq185_e2329_d_n14, eq185_e2329_d_n15, eq185_e2329_d_n16, eq185_e2329_d_n17, eq185_e2329_d_n18, eq185_e2329_d_n19, eq185_e2329_d_n20, eq185_e2329_d_n21, eq185_e2329_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq185_value: f64 = eq185_e2331;
        let eq185_node_derivatives: [f64; 23] = [eq185_e2331_d_n0, eq185_e2331_d_n1, eq185_e2331_d_n2, eq185_e2331_d_n3, eq185_e2331_d_n4, eq185_e2331_d_n5, eq185_e2331_d_n6, eq185_e2331_d_n7, eq185_e2331_d_n8, eq185_e2331_d_n9, eq185_e2331_d_n10, eq185_e2331_d_n11, eq185_e2331_d_n12, eq185_e2331_d_n13, eq185_e2331_d_n14, eq185_e2331_d_n15, eq185_e2331_d_n16, eq185_e2331_d_n17, eq185_e2331_d_n18, eq185_e2331_d_n19, eq185_e2331_d_n20, eq185_e2331_d_n21, eq185_e2331_d_n22];
        let eq185_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[21]),
            self.multiplicity * (eq185_value),
            &nodes,
            &eq185_node_derivatives,
            &branches,
            &eq185_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_186_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq186_e2341, eq186_e2341_d_n0, eq186_e2341_d_n1, eq186_e2341_d_n2, eq186_e2341_d_n3, eq186_e2341_d_n4, eq186_e2341_d_n5, eq186_e2341_d_n6, eq186_e2341_d_n7, eq186_e2341_d_n8, eq186_e2341_d_n9, eq186_e2341_d_n10, eq186_e2341_d_n11, eq186_e2341_d_n12, eq186_e2341_d_n13, eq186_e2341_d_n14, eq186_e2341_d_n15, eq186_e2341_d_n16, eq186_e2341_d_n17, eq186_e2341_d_n18, eq186_e2341_d_n19, eq186_e2341_d_n20, eq186_e2341_d_n21, eq186_e2341_d_n22,) = {
    if ((!(s.v[595] != 0.0)) && (s.v[598] != 0.0)) {
        let eq186_e2338: f64 = self.eval_ddt(85, s.v[289]);
        let eq186_e2338_d_n0: f64 = self.ddt_jacobian(s.dn[289][0]);
        let eq186_e2338_d_n1: f64 = self.ddt_jacobian(s.dn[289][1]);
        let eq186_e2338_d_n2: f64 = self.ddt_jacobian(s.dn[289][2]);
        let eq186_e2338_d_n3: f64 = self.ddt_jacobian(s.dn[289][3]);
        let eq186_e2338_d_n4: f64 = self.ddt_jacobian(s.dn[289][4]);
        let eq186_e2338_d_n5: f64 = self.ddt_jacobian(s.dn[289][5]);
        let eq186_e2338_d_n6: f64 = self.ddt_jacobian(s.dn[289][6]);
        let eq186_e2338_d_n7: f64 = self.ddt_jacobian(s.dn[289][7]);
        let eq186_e2338_d_n8: f64 = self.ddt_jacobian(s.dn[289][8]);
        let eq186_e2338_d_n9: f64 = self.ddt_jacobian(s.dn[289][9]);
        let eq186_e2338_d_n10: f64 = self.ddt_jacobian(s.dn[289][10]);
        let eq186_e2338_d_n11: f64 = self.ddt_jacobian(s.dn[289][11]);
        let eq186_e2338_d_n12: f64 = self.ddt_jacobian(s.dn[289][12]);
        let eq186_e2338_d_n13: f64 = self.ddt_jacobian(s.dn[289][13]);
        let eq186_e2338_d_n14: f64 = self.ddt_jacobian(s.dn[289][14]);
        let eq186_e2338_d_n15: f64 = self.ddt_jacobian(s.dn[289][15]);
        let eq186_e2338_d_n16: f64 = self.ddt_jacobian(s.dn[289][16]);
        let eq186_e2338_d_n17: f64 = self.ddt_jacobian(s.dn[289][17]);
        let eq186_e2338_d_n18: f64 = self.ddt_jacobian(s.dn[289][18]);
        let eq186_e2338_d_n19: f64 = self.ddt_jacobian(s.dn[289][19]);
        let eq186_e2338_d_n20: f64 = self.ddt_jacobian(s.dn[289][20]);
        let eq186_e2338_d_n21: f64 = self.ddt_jacobian(s.dn[289][21]);
        let eq186_e2338_d_n22: f64 = self.ddt_jacobian(s.dn[289][22]);
        let eq186_e2339: f64 = (p.p7 * eq186_e2338);
        let eq186_e2339_d_n0: f64 = (p.p7 * eq186_e2338_d_n0);
        let eq186_e2339_d_n1: f64 = (p.p7 * eq186_e2338_d_n1);
        let eq186_e2339_d_n2: f64 = (p.p7 * eq186_e2338_d_n2);
        let eq186_e2339_d_n3: f64 = (p.p7 * eq186_e2338_d_n3);
        let eq186_e2339_d_n4: f64 = (p.p7 * eq186_e2338_d_n4);
        let eq186_e2339_d_n5: f64 = (p.p7 * eq186_e2338_d_n5);
        let eq186_e2339_d_n6: f64 = (p.p7 * eq186_e2338_d_n6);
        let eq186_e2339_d_n7: f64 = (p.p7 * eq186_e2338_d_n7);
        let eq186_e2339_d_n8: f64 = (p.p7 * eq186_e2338_d_n8);
        let eq186_e2339_d_n9: f64 = (p.p7 * eq186_e2338_d_n9);
        let eq186_e2339_d_n10: f64 = (p.p7 * eq186_e2338_d_n10);
        let eq186_e2339_d_n11: f64 = (p.p7 * eq186_e2338_d_n11);
        let eq186_e2339_d_n12: f64 = (p.p7 * eq186_e2338_d_n12);
        let eq186_e2339_d_n13: f64 = (p.p7 * eq186_e2338_d_n13);
        let eq186_e2339_d_n14: f64 = (p.p7 * eq186_e2338_d_n14);
        let eq186_e2339_d_n15: f64 = (p.p7 * eq186_e2338_d_n15);
        let eq186_e2339_d_n16: f64 = (p.p7 * eq186_e2338_d_n16);
        let eq186_e2339_d_n17: f64 = (p.p7 * eq186_e2338_d_n17);
        let eq186_e2339_d_n18: f64 = (p.p7 * eq186_e2338_d_n18);
        let eq186_e2339_d_n19: f64 = (p.p7 * eq186_e2338_d_n19);
        let eq186_e2339_d_n20: f64 = (p.p7 * eq186_e2338_d_n20);
        let eq186_e2339_d_n21: f64 = (p.p7 * eq186_e2338_d_n21);
        let eq186_e2339_d_n22: f64 = (p.p7 * eq186_e2338_d_n22);
        (eq186_e2339, eq186_e2339_d_n0, eq186_e2339_d_n1, eq186_e2339_d_n2, eq186_e2339_d_n3, eq186_e2339_d_n4, eq186_e2339_d_n5, eq186_e2339_d_n6, eq186_e2339_d_n7, eq186_e2339_d_n8, eq186_e2339_d_n9, eq186_e2339_d_n10, eq186_e2339_d_n11, eq186_e2339_d_n12, eq186_e2339_d_n13, eq186_e2339_d_n14, eq186_e2339_d_n15, eq186_e2339_d_n16, eq186_e2339_d_n17, eq186_e2339_d_n18, eq186_e2339_d_n19, eq186_e2339_d_n20, eq186_e2339_d_n21, eq186_e2339_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq186_value: f64 = eq186_e2341;
        let eq186_node_derivatives: [f64; 23] = [eq186_e2341_d_n0, eq186_e2341_d_n1, eq186_e2341_d_n2, eq186_e2341_d_n3, eq186_e2341_d_n4, eq186_e2341_d_n5, eq186_e2341_d_n6, eq186_e2341_d_n7, eq186_e2341_d_n8, eq186_e2341_d_n9, eq186_e2341_d_n10, eq186_e2341_d_n11, eq186_e2341_d_n12, eq186_e2341_d_n13, eq186_e2341_d_n14, eq186_e2341_d_n15, eq186_e2341_d_n16, eq186_e2341_d_n17, eq186_e2341_d_n18, eq186_e2341_d_n19, eq186_e2341_d_n20, eq186_e2341_d_n21, eq186_e2341_d_n22];
        let eq186_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            self.multiplicity * (eq186_value),
            &nodes,
            &eq186_node_derivatives,
            &branches,
            &eq186_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_187_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq187_e2353, eq187_e2353_d_n0, eq187_e2353_d_n1, eq187_e2353_d_n2, eq187_e2353_d_n3, eq187_e2353_d_n4, eq187_e2353_d_n5, eq187_e2353_d_n6, eq187_e2353_d_n7, eq187_e2353_d_n8, eq187_e2353_d_n9, eq187_e2353_d_n10, eq187_e2353_d_n11, eq187_e2353_d_n12, eq187_e2353_d_n13, eq187_e2353_d_n14, eq187_e2353_d_n15, eq187_e2353_d_n16, eq187_e2353_d_n17, eq187_e2353_d_n18, eq187_e2353_d_n19, eq187_e2353_d_n20, eq187_e2353_d_n21, eq187_e2353_d_n22,) = {
    if (((!(s.v[595] != 0.0)) && (s.v[598] != 0.0)) && (s.v[599] != 0.0)) {
        let eq187_e2350: f64 = self.eval_ddt(86, s.v[288]);
        let eq187_e2350_d_n0: f64 = self.ddt_jacobian(s.dn[288][0]);
        let eq187_e2350_d_n1: f64 = self.ddt_jacobian(s.dn[288][1]);
        let eq187_e2350_d_n2: f64 = self.ddt_jacobian(s.dn[288][2]);
        let eq187_e2350_d_n3: f64 = self.ddt_jacobian(s.dn[288][3]);
        let eq187_e2350_d_n4: f64 = self.ddt_jacobian(s.dn[288][4]);
        let eq187_e2350_d_n5: f64 = self.ddt_jacobian(s.dn[288][5]);
        let eq187_e2350_d_n6: f64 = self.ddt_jacobian(s.dn[288][6]);
        let eq187_e2350_d_n7: f64 = self.ddt_jacobian(s.dn[288][7]);
        let eq187_e2350_d_n8: f64 = self.ddt_jacobian(s.dn[288][8]);
        let eq187_e2350_d_n9: f64 = self.ddt_jacobian(s.dn[288][9]);
        let eq187_e2350_d_n10: f64 = self.ddt_jacobian(s.dn[288][10]);
        let eq187_e2350_d_n11: f64 = self.ddt_jacobian(s.dn[288][11]);
        let eq187_e2350_d_n12: f64 = self.ddt_jacobian(s.dn[288][12]);
        let eq187_e2350_d_n13: f64 = self.ddt_jacobian(s.dn[288][13]);
        let eq187_e2350_d_n14: f64 = self.ddt_jacobian(s.dn[288][14]);
        let eq187_e2350_d_n15: f64 = self.ddt_jacobian(s.dn[288][15]);
        let eq187_e2350_d_n16: f64 = self.ddt_jacobian(s.dn[288][16]);
        let eq187_e2350_d_n17: f64 = self.ddt_jacobian(s.dn[288][17]);
        let eq187_e2350_d_n18: f64 = self.ddt_jacobian(s.dn[288][18]);
        let eq187_e2350_d_n19: f64 = self.ddt_jacobian(s.dn[288][19]);
        let eq187_e2350_d_n20: f64 = self.ddt_jacobian(s.dn[288][20]);
        let eq187_e2350_d_n21: f64 = self.ddt_jacobian(s.dn[288][21]);
        let eq187_e2350_d_n22: f64 = self.ddt_jacobian(s.dn[288][22]);
        let eq187_e2351: f64 = (p.p7 * eq187_e2350);
        let eq187_e2351_d_n0: f64 = (p.p7 * eq187_e2350_d_n0);
        let eq187_e2351_d_n1: f64 = (p.p7 * eq187_e2350_d_n1);
        let eq187_e2351_d_n2: f64 = (p.p7 * eq187_e2350_d_n2);
        let eq187_e2351_d_n3: f64 = (p.p7 * eq187_e2350_d_n3);
        let eq187_e2351_d_n4: f64 = (p.p7 * eq187_e2350_d_n4);
        let eq187_e2351_d_n5: f64 = (p.p7 * eq187_e2350_d_n5);
        let eq187_e2351_d_n6: f64 = (p.p7 * eq187_e2350_d_n6);
        let eq187_e2351_d_n7: f64 = (p.p7 * eq187_e2350_d_n7);
        let eq187_e2351_d_n8: f64 = (p.p7 * eq187_e2350_d_n8);
        let eq187_e2351_d_n9: f64 = (p.p7 * eq187_e2350_d_n9);
        let eq187_e2351_d_n10: f64 = (p.p7 * eq187_e2350_d_n10);
        let eq187_e2351_d_n11: f64 = (p.p7 * eq187_e2350_d_n11);
        let eq187_e2351_d_n12: f64 = (p.p7 * eq187_e2350_d_n12);
        let eq187_e2351_d_n13: f64 = (p.p7 * eq187_e2350_d_n13);
        let eq187_e2351_d_n14: f64 = (p.p7 * eq187_e2350_d_n14);
        let eq187_e2351_d_n15: f64 = (p.p7 * eq187_e2350_d_n15);
        let eq187_e2351_d_n16: f64 = (p.p7 * eq187_e2350_d_n16);
        let eq187_e2351_d_n17: f64 = (p.p7 * eq187_e2350_d_n17);
        let eq187_e2351_d_n18: f64 = (p.p7 * eq187_e2350_d_n18);
        let eq187_e2351_d_n19: f64 = (p.p7 * eq187_e2350_d_n19);
        let eq187_e2351_d_n20: f64 = (p.p7 * eq187_e2350_d_n20);
        let eq187_e2351_d_n21: f64 = (p.p7 * eq187_e2350_d_n21);
        let eq187_e2351_d_n22: f64 = (p.p7 * eq187_e2350_d_n22);
        (eq187_e2351, eq187_e2351_d_n0, eq187_e2351_d_n1, eq187_e2351_d_n2, eq187_e2351_d_n3, eq187_e2351_d_n4, eq187_e2351_d_n5, eq187_e2351_d_n6, eq187_e2351_d_n7, eq187_e2351_d_n8, eq187_e2351_d_n9, eq187_e2351_d_n10, eq187_e2351_d_n11, eq187_e2351_d_n12, eq187_e2351_d_n13, eq187_e2351_d_n14, eq187_e2351_d_n15, eq187_e2351_d_n16, eq187_e2351_d_n17, eq187_e2351_d_n18, eq187_e2351_d_n19, eq187_e2351_d_n20, eq187_e2351_d_n21, eq187_e2351_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq187_value: f64 = eq187_e2353;
        let eq187_node_derivatives: [f64; 23] = [eq187_e2353_d_n0, eq187_e2353_d_n1, eq187_e2353_d_n2, eq187_e2353_d_n3, eq187_e2353_d_n4, eq187_e2353_d_n5, eq187_e2353_d_n6, eq187_e2353_d_n7, eq187_e2353_d_n8, eq187_e2353_d_n9, eq187_e2353_d_n10, eq187_e2353_d_n11, eq187_e2353_d_n12, eq187_e2353_d_n13, eq187_e2353_d_n14, eq187_e2353_d_n15, eq187_e2353_d_n16, eq187_e2353_d_n17, eq187_e2353_d_n18, eq187_e2353_d_n19, eq187_e2353_d_n20, eq187_e2353_d_n21, eq187_e2353_d_n22];
        let eq187_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[2]),
            self.multiplicity * (eq187_value),
            &nodes,
            &eq187_node_derivatives,
            &branches,
            &eq187_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_188_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq188_e2367, eq188_e2367_d_n0, eq188_e2367_d_n1, eq188_e2367_d_n2, eq188_e2367_d_n3, eq188_e2367_d_n4, eq188_e2367_d_n5, eq188_e2367_d_n6, eq188_e2367_d_n7, eq188_e2367_d_n8, eq188_e2367_d_n9, eq188_e2367_d_n10, eq188_e2367_d_n11, eq188_e2367_d_n12, eq188_e2367_d_n13, eq188_e2367_d_n14, eq188_e2367_d_n15, eq188_e2367_d_n16, eq188_e2367_d_n17, eq188_e2367_d_n18, eq188_e2367_d_n19, eq188_e2367_d_n20, eq188_e2367_d_n21, eq188_e2367_d_n22,) = {
    if (((!(s.v[595] != 0.0)) && (s.v[598] != 0.0)) && (s.v[599] != 0.0)) {
        let eq188_e2362: f64 = self.eval_ddt(87, s.v[288]);
        let eq188_e2362_d_n0: f64 = self.ddt_jacobian(s.dn[288][0]);
        let eq188_e2362_d_n1: f64 = self.ddt_jacobian(s.dn[288][1]);
        let eq188_e2362_d_n2: f64 = self.ddt_jacobian(s.dn[288][2]);
        let eq188_e2362_d_n3: f64 = self.ddt_jacobian(s.dn[288][3]);
        let eq188_e2362_d_n4: f64 = self.ddt_jacobian(s.dn[288][4]);
        let eq188_e2362_d_n5: f64 = self.ddt_jacobian(s.dn[288][5]);
        let eq188_e2362_d_n6: f64 = self.ddt_jacobian(s.dn[288][6]);
        let eq188_e2362_d_n7: f64 = self.ddt_jacobian(s.dn[288][7]);
        let eq188_e2362_d_n8: f64 = self.ddt_jacobian(s.dn[288][8]);
        let eq188_e2362_d_n9: f64 = self.ddt_jacobian(s.dn[288][9]);
        let eq188_e2362_d_n10: f64 = self.ddt_jacobian(s.dn[288][10]);
        let eq188_e2362_d_n11: f64 = self.ddt_jacobian(s.dn[288][11]);
        let eq188_e2362_d_n12: f64 = self.ddt_jacobian(s.dn[288][12]);
        let eq188_e2362_d_n13: f64 = self.ddt_jacobian(s.dn[288][13]);
        let eq188_e2362_d_n14: f64 = self.ddt_jacobian(s.dn[288][14]);
        let eq188_e2362_d_n15: f64 = self.ddt_jacobian(s.dn[288][15]);
        let eq188_e2362_d_n16: f64 = self.ddt_jacobian(s.dn[288][16]);
        let eq188_e2362_d_n17: f64 = self.ddt_jacobian(s.dn[288][17]);
        let eq188_e2362_d_n18: f64 = self.ddt_jacobian(s.dn[288][18]);
        let eq188_e2362_d_n19: f64 = self.ddt_jacobian(s.dn[288][19]);
        let eq188_e2362_d_n20: f64 = self.ddt_jacobian(s.dn[288][20]);
        let eq188_e2362_d_n21: f64 = self.ddt_jacobian(s.dn[288][21]);
        let eq188_e2362_d_n22: f64 = self.ddt_jacobian(s.dn[288][22]);
        let eq188_e2363: f64 = (p.p7 * eq188_e2362);
        let eq188_e2363_d_n0: f64 = (p.p7 * eq188_e2362_d_n0);
        let eq188_e2363_d_n1: f64 = (p.p7 * eq188_e2362_d_n1);
        let eq188_e2363_d_n2: f64 = (p.p7 * eq188_e2362_d_n2);
        let eq188_e2363_d_n3: f64 = (p.p7 * eq188_e2362_d_n3);
        let eq188_e2363_d_n4: f64 = (p.p7 * eq188_e2362_d_n4);
        let eq188_e2363_d_n5: f64 = (p.p7 * eq188_e2362_d_n5);
        let eq188_e2363_d_n6: f64 = (p.p7 * eq188_e2362_d_n6);
        let eq188_e2363_d_n7: f64 = (p.p7 * eq188_e2362_d_n7);
        let eq188_e2363_d_n8: f64 = (p.p7 * eq188_e2362_d_n8);
        let eq188_e2363_d_n9: f64 = (p.p7 * eq188_e2362_d_n9);
        let eq188_e2363_d_n10: f64 = (p.p7 * eq188_e2362_d_n10);
        let eq188_e2363_d_n11: f64 = (p.p7 * eq188_e2362_d_n11);
        let eq188_e2363_d_n12: f64 = (p.p7 * eq188_e2362_d_n12);
        let eq188_e2363_d_n13: f64 = (p.p7 * eq188_e2362_d_n13);
        let eq188_e2363_d_n14: f64 = (p.p7 * eq188_e2362_d_n14);
        let eq188_e2363_d_n15: f64 = (p.p7 * eq188_e2362_d_n15);
        let eq188_e2363_d_n16: f64 = (p.p7 * eq188_e2362_d_n16);
        let eq188_e2363_d_n17: f64 = (p.p7 * eq188_e2362_d_n17);
        let eq188_e2363_d_n18: f64 = (p.p7 * eq188_e2362_d_n18);
        let eq188_e2363_d_n19: f64 = (p.p7 * eq188_e2362_d_n19);
        let eq188_e2363_d_n20: f64 = (p.p7 * eq188_e2362_d_n20);
        let eq188_e2363_d_n21: f64 = (p.p7 * eq188_e2362_d_n21);
        let eq188_e2363_d_n22: f64 = (p.p7 * eq188_e2362_d_n22);
        let eq188_e2365: f64 = (eq188_e2363 * p.p248);
        let eq188_e2365_d_n0: f64 = (eq188_e2363_d_n0 * p.p248);
        let eq188_e2365_d_n1: f64 = (eq188_e2363_d_n1 * p.p248);
        let eq188_e2365_d_n2: f64 = (eq188_e2363_d_n2 * p.p248);
        let eq188_e2365_d_n3: f64 = (eq188_e2363_d_n3 * p.p248);
        let eq188_e2365_d_n4: f64 = (eq188_e2363_d_n4 * p.p248);
        let eq188_e2365_d_n5: f64 = (eq188_e2363_d_n5 * p.p248);
        let eq188_e2365_d_n6: f64 = (eq188_e2363_d_n6 * p.p248);
        let eq188_e2365_d_n7: f64 = (eq188_e2363_d_n7 * p.p248);
        let eq188_e2365_d_n8: f64 = (eq188_e2363_d_n8 * p.p248);
        let eq188_e2365_d_n9: f64 = (eq188_e2363_d_n9 * p.p248);
        let eq188_e2365_d_n10: f64 = (eq188_e2363_d_n10 * p.p248);
        let eq188_e2365_d_n11: f64 = (eq188_e2363_d_n11 * p.p248);
        let eq188_e2365_d_n12: f64 = (eq188_e2363_d_n12 * p.p248);
        let eq188_e2365_d_n13: f64 = (eq188_e2363_d_n13 * p.p248);
        let eq188_e2365_d_n14: f64 = (eq188_e2363_d_n14 * p.p248);
        let eq188_e2365_d_n15: f64 = (eq188_e2363_d_n15 * p.p248);
        let eq188_e2365_d_n16: f64 = (eq188_e2363_d_n16 * p.p248);
        let eq188_e2365_d_n17: f64 = (eq188_e2363_d_n17 * p.p248);
        let eq188_e2365_d_n18: f64 = (eq188_e2363_d_n18 * p.p248);
        let eq188_e2365_d_n19: f64 = (eq188_e2363_d_n19 * p.p248);
        let eq188_e2365_d_n20: f64 = (eq188_e2363_d_n20 * p.p248);
        let eq188_e2365_d_n21: f64 = (eq188_e2363_d_n21 * p.p248);
        let eq188_e2365_d_n22: f64 = (eq188_e2363_d_n22 * p.p248);
        (eq188_e2365, eq188_e2365_d_n0, eq188_e2365_d_n1, eq188_e2365_d_n2, eq188_e2365_d_n3, eq188_e2365_d_n4, eq188_e2365_d_n5, eq188_e2365_d_n6, eq188_e2365_d_n7, eq188_e2365_d_n8, eq188_e2365_d_n9, eq188_e2365_d_n10, eq188_e2365_d_n11, eq188_e2365_d_n12, eq188_e2365_d_n13, eq188_e2365_d_n14, eq188_e2365_d_n15, eq188_e2365_d_n16, eq188_e2365_d_n17, eq188_e2365_d_n18, eq188_e2365_d_n19, eq188_e2365_d_n20, eq188_e2365_d_n21, eq188_e2365_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq188_value: f64 = eq188_e2367;
        let eq188_node_derivatives: [f64; 23] = [eq188_e2367_d_n0, eq188_e2367_d_n1, eq188_e2367_d_n2, eq188_e2367_d_n3, eq188_e2367_d_n4, eq188_e2367_d_n5, eq188_e2367_d_n6, eq188_e2367_d_n7, eq188_e2367_d_n8, eq188_e2367_d_n9, eq188_e2367_d_n10, eq188_e2367_d_n11, eq188_e2367_d_n12, eq188_e2367_d_n13, eq188_e2367_d_n14, eq188_e2367_d_n15, eq188_e2367_d_n16, eq188_e2367_d_n17, eq188_e2367_d_n18, eq188_e2367_d_n19, eq188_e2367_d_n20, eq188_e2367_d_n21, eq188_e2367_d_n22];
        let eq188_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            self.multiplicity * (eq188_value),
            &nodes,
            &eq188_node_derivatives,
            &branches,
            &eq188_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_189_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq189_e2380, eq189_e2380_d_n0, eq189_e2380_d_n1, eq189_e2380_d_n2, eq189_e2380_d_n3, eq189_e2380_d_n4, eq189_e2380_d_n5, eq189_e2380_d_n6, eq189_e2380_d_n7, eq189_e2380_d_n8, eq189_e2380_d_n9, eq189_e2380_d_n10, eq189_e2380_d_n11, eq189_e2380_d_n12, eq189_e2380_d_n13, eq189_e2380_d_n14, eq189_e2380_d_n15, eq189_e2380_d_n16, eq189_e2380_d_n17, eq189_e2380_d_n18, eq189_e2380_d_n19, eq189_e2380_d_n20, eq189_e2380_d_n21, eq189_e2380_d_n22,) = {
    if (((!(s.v[595] != 0.0)) && (s.v[598] != 0.0)) && (!(s.v[599] != 0.0))) {
        let eq189_e2377: f64 = self.eval_ddt(88, s.v[288]);
        let eq189_e2377_d_n0: f64 = self.ddt_jacobian(s.dn[288][0]);
        let eq189_e2377_d_n1: f64 = self.ddt_jacobian(s.dn[288][1]);
        let eq189_e2377_d_n2: f64 = self.ddt_jacobian(s.dn[288][2]);
        let eq189_e2377_d_n3: f64 = self.ddt_jacobian(s.dn[288][3]);
        let eq189_e2377_d_n4: f64 = self.ddt_jacobian(s.dn[288][4]);
        let eq189_e2377_d_n5: f64 = self.ddt_jacobian(s.dn[288][5]);
        let eq189_e2377_d_n6: f64 = self.ddt_jacobian(s.dn[288][6]);
        let eq189_e2377_d_n7: f64 = self.ddt_jacobian(s.dn[288][7]);
        let eq189_e2377_d_n8: f64 = self.ddt_jacobian(s.dn[288][8]);
        let eq189_e2377_d_n9: f64 = self.ddt_jacobian(s.dn[288][9]);
        let eq189_e2377_d_n10: f64 = self.ddt_jacobian(s.dn[288][10]);
        let eq189_e2377_d_n11: f64 = self.ddt_jacobian(s.dn[288][11]);
        let eq189_e2377_d_n12: f64 = self.ddt_jacobian(s.dn[288][12]);
        let eq189_e2377_d_n13: f64 = self.ddt_jacobian(s.dn[288][13]);
        let eq189_e2377_d_n14: f64 = self.ddt_jacobian(s.dn[288][14]);
        let eq189_e2377_d_n15: f64 = self.ddt_jacobian(s.dn[288][15]);
        let eq189_e2377_d_n16: f64 = self.ddt_jacobian(s.dn[288][16]);
        let eq189_e2377_d_n17: f64 = self.ddt_jacobian(s.dn[288][17]);
        let eq189_e2377_d_n18: f64 = self.ddt_jacobian(s.dn[288][18]);
        let eq189_e2377_d_n19: f64 = self.ddt_jacobian(s.dn[288][19]);
        let eq189_e2377_d_n20: f64 = self.ddt_jacobian(s.dn[288][20]);
        let eq189_e2377_d_n21: f64 = self.ddt_jacobian(s.dn[288][21]);
        let eq189_e2377_d_n22: f64 = self.ddt_jacobian(s.dn[288][22]);
        let eq189_e2378: f64 = (p.p7 * eq189_e2377);
        let eq189_e2378_d_n0: f64 = (p.p7 * eq189_e2377_d_n0);
        let eq189_e2378_d_n1: f64 = (p.p7 * eq189_e2377_d_n1);
        let eq189_e2378_d_n2: f64 = (p.p7 * eq189_e2377_d_n2);
        let eq189_e2378_d_n3: f64 = (p.p7 * eq189_e2377_d_n3);
        let eq189_e2378_d_n4: f64 = (p.p7 * eq189_e2377_d_n4);
        let eq189_e2378_d_n5: f64 = (p.p7 * eq189_e2377_d_n5);
        let eq189_e2378_d_n6: f64 = (p.p7 * eq189_e2377_d_n6);
        let eq189_e2378_d_n7: f64 = (p.p7 * eq189_e2377_d_n7);
        let eq189_e2378_d_n8: f64 = (p.p7 * eq189_e2377_d_n8);
        let eq189_e2378_d_n9: f64 = (p.p7 * eq189_e2377_d_n9);
        let eq189_e2378_d_n10: f64 = (p.p7 * eq189_e2377_d_n10);
        let eq189_e2378_d_n11: f64 = (p.p7 * eq189_e2377_d_n11);
        let eq189_e2378_d_n12: f64 = (p.p7 * eq189_e2377_d_n12);
        let eq189_e2378_d_n13: f64 = (p.p7 * eq189_e2377_d_n13);
        let eq189_e2378_d_n14: f64 = (p.p7 * eq189_e2377_d_n14);
        let eq189_e2378_d_n15: f64 = (p.p7 * eq189_e2377_d_n15);
        let eq189_e2378_d_n16: f64 = (p.p7 * eq189_e2377_d_n16);
        let eq189_e2378_d_n17: f64 = (p.p7 * eq189_e2377_d_n17);
        let eq189_e2378_d_n18: f64 = (p.p7 * eq189_e2377_d_n18);
        let eq189_e2378_d_n19: f64 = (p.p7 * eq189_e2377_d_n19);
        let eq189_e2378_d_n20: f64 = (p.p7 * eq189_e2377_d_n20);
        let eq189_e2378_d_n21: f64 = (p.p7 * eq189_e2377_d_n21);
        let eq189_e2378_d_n22: f64 = (p.p7 * eq189_e2377_d_n22);
        (eq189_e2378, eq189_e2378_d_n0, eq189_e2378_d_n1, eq189_e2378_d_n2, eq189_e2378_d_n3, eq189_e2378_d_n4, eq189_e2378_d_n5, eq189_e2378_d_n6, eq189_e2378_d_n7, eq189_e2378_d_n8, eq189_e2378_d_n9, eq189_e2378_d_n10, eq189_e2378_d_n11, eq189_e2378_d_n12, eq189_e2378_d_n13, eq189_e2378_d_n14, eq189_e2378_d_n15, eq189_e2378_d_n16, eq189_e2378_d_n17, eq189_e2378_d_n18, eq189_e2378_d_n19, eq189_e2378_d_n20, eq189_e2378_d_n21, eq189_e2378_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq189_value: f64 = eq189_e2380;
        let eq189_node_derivatives: [f64; 23] = [eq189_e2380_d_n0, eq189_e2380_d_n1, eq189_e2380_d_n2, eq189_e2380_d_n3, eq189_e2380_d_n4, eq189_e2380_d_n5, eq189_e2380_d_n6, eq189_e2380_d_n7, eq189_e2380_d_n8, eq189_e2380_d_n9, eq189_e2380_d_n10, eq189_e2380_d_n11, eq189_e2380_d_n12, eq189_e2380_d_n13, eq189_e2380_d_n14, eq189_e2380_d_n15, eq189_e2380_d_n16, eq189_e2380_d_n17, eq189_e2380_d_n18, eq189_e2380_d_n19, eq189_e2380_d_n20, eq189_e2380_d_n21, eq189_e2380_d_n22];
        let eq189_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            self.multiplicity * (eq189_value),
            &nodes,
            &eq189_node_derivatives,
            &branches,
            &eq189_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_190_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq190_e2395, eq190_e2395_d_n0, eq190_e2395_d_n1, eq190_e2395_d_n2, eq190_e2395_d_n3, eq190_e2395_d_n4, eq190_e2395_d_n5, eq190_e2395_d_n6, eq190_e2395_d_n7, eq190_e2395_d_n8, eq190_e2395_d_n9, eq190_e2395_d_n10, eq190_e2395_d_n11, eq190_e2395_d_n12, eq190_e2395_d_n13, eq190_e2395_d_n14, eq190_e2395_d_n15, eq190_e2395_d_n16, eq190_e2395_d_n17, eq190_e2395_d_n18, eq190_e2395_d_n19, eq190_e2395_d_n20, eq190_e2395_d_n21, eq190_e2395_d_n22,) = {
    if (((!(s.v[595] != 0.0)) && (s.v[598] != 0.0)) && (!(s.v[599] != 0.0))) {
        let eq190_e2390: f64 = self.eval_ddt(89, s.v[288]);
        let eq190_e2390_d_n0: f64 = self.ddt_jacobian(s.dn[288][0]);
        let eq190_e2390_d_n1: f64 = self.ddt_jacobian(s.dn[288][1]);
        let eq190_e2390_d_n2: f64 = self.ddt_jacobian(s.dn[288][2]);
        let eq190_e2390_d_n3: f64 = self.ddt_jacobian(s.dn[288][3]);
        let eq190_e2390_d_n4: f64 = self.ddt_jacobian(s.dn[288][4]);
        let eq190_e2390_d_n5: f64 = self.ddt_jacobian(s.dn[288][5]);
        let eq190_e2390_d_n6: f64 = self.ddt_jacobian(s.dn[288][6]);
        let eq190_e2390_d_n7: f64 = self.ddt_jacobian(s.dn[288][7]);
        let eq190_e2390_d_n8: f64 = self.ddt_jacobian(s.dn[288][8]);
        let eq190_e2390_d_n9: f64 = self.ddt_jacobian(s.dn[288][9]);
        let eq190_e2390_d_n10: f64 = self.ddt_jacobian(s.dn[288][10]);
        let eq190_e2390_d_n11: f64 = self.ddt_jacobian(s.dn[288][11]);
        let eq190_e2390_d_n12: f64 = self.ddt_jacobian(s.dn[288][12]);
        let eq190_e2390_d_n13: f64 = self.ddt_jacobian(s.dn[288][13]);
        let eq190_e2390_d_n14: f64 = self.ddt_jacobian(s.dn[288][14]);
        let eq190_e2390_d_n15: f64 = self.ddt_jacobian(s.dn[288][15]);
        let eq190_e2390_d_n16: f64 = self.ddt_jacobian(s.dn[288][16]);
        let eq190_e2390_d_n17: f64 = self.ddt_jacobian(s.dn[288][17]);
        let eq190_e2390_d_n18: f64 = self.ddt_jacobian(s.dn[288][18]);
        let eq190_e2390_d_n19: f64 = self.ddt_jacobian(s.dn[288][19]);
        let eq190_e2390_d_n20: f64 = self.ddt_jacobian(s.dn[288][20]);
        let eq190_e2390_d_n21: f64 = self.ddt_jacobian(s.dn[288][21]);
        let eq190_e2390_d_n22: f64 = self.ddt_jacobian(s.dn[288][22]);
        let eq190_e2391: f64 = (p.p7 * eq190_e2390);
        let eq190_e2391_d_n0: f64 = (p.p7 * eq190_e2390_d_n0);
        let eq190_e2391_d_n1: f64 = (p.p7 * eq190_e2390_d_n1);
        let eq190_e2391_d_n2: f64 = (p.p7 * eq190_e2390_d_n2);
        let eq190_e2391_d_n3: f64 = (p.p7 * eq190_e2390_d_n3);
        let eq190_e2391_d_n4: f64 = (p.p7 * eq190_e2390_d_n4);
        let eq190_e2391_d_n5: f64 = (p.p7 * eq190_e2390_d_n5);
        let eq190_e2391_d_n6: f64 = (p.p7 * eq190_e2390_d_n6);
        let eq190_e2391_d_n7: f64 = (p.p7 * eq190_e2390_d_n7);
        let eq190_e2391_d_n8: f64 = (p.p7 * eq190_e2390_d_n8);
        let eq190_e2391_d_n9: f64 = (p.p7 * eq190_e2390_d_n9);
        let eq190_e2391_d_n10: f64 = (p.p7 * eq190_e2390_d_n10);
        let eq190_e2391_d_n11: f64 = (p.p7 * eq190_e2390_d_n11);
        let eq190_e2391_d_n12: f64 = (p.p7 * eq190_e2390_d_n12);
        let eq190_e2391_d_n13: f64 = (p.p7 * eq190_e2390_d_n13);
        let eq190_e2391_d_n14: f64 = (p.p7 * eq190_e2390_d_n14);
        let eq190_e2391_d_n15: f64 = (p.p7 * eq190_e2390_d_n15);
        let eq190_e2391_d_n16: f64 = (p.p7 * eq190_e2390_d_n16);
        let eq190_e2391_d_n17: f64 = (p.p7 * eq190_e2390_d_n17);
        let eq190_e2391_d_n18: f64 = (p.p7 * eq190_e2390_d_n18);
        let eq190_e2391_d_n19: f64 = (p.p7 * eq190_e2390_d_n19);
        let eq190_e2391_d_n20: f64 = (p.p7 * eq190_e2390_d_n20);
        let eq190_e2391_d_n21: f64 = (p.p7 * eq190_e2390_d_n21);
        let eq190_e2391_d_n22: f64 = (p.p7 * eq190_e2390_d_n22);
        let eq190_e2393: f64 = (eq190_e2391 * p.p248);
        let eq190_e2393_d_n0: f64 = (eq190_e2391_d_n0 * p.p248);
        let eq190_e2393_d_n1: f64 = (eq190_e2391_d_n1 * p.p248);
        let eq190_e2393_d_n2: f64 = (eq190_e2391_d_n2 * p.p248);
        let eq190_e2393_d_n3: f64 = (eq190_e2391_d_n3 * p.p248);
        let eq190_e2393_d_n4: f64 = (eq190_e2391_d_n4 * p.p248);
        let eq190_e2393_d_n5: f64 = (eq190_e2391_d_n5 * p.p248);
        let eq190_e2393_d_n6: f64 = (eq190_e2391_d_n6 * p.p248);
        let eq190_e2393_d_n7: f64 = (eq190_e2391_d_n7 * p.p248);
        let eq190_e2393_d_n8: f64 = (eq190_e2391_d_n8 * p.p248);
        let eq190_e2393_d_n9: f64 = (eq190_e2391_d_n9 * p.p248);
        let eq190_e2393_d_n10: f64 = (eq190_e2391_d_n10 * p.p248);
        let eq190_e2393_d_n11: f64 = (eq190_e2391_d_n11 * p.p248);
        let eq190_e2393_d_n12: f64 = (eq190_e2391_d_n12 * p.p248);
        let eq190_e2393_d_n13: f64 = (eq190_e2391_d_n13 * p.p248);
        let eq190_e2393_d_n14: f64 = (eq190_e2391_d_n14 * p.p248);
        let eq190_e2393_d_n15: f64 = (eq190_e2391_d_n15 * p.p248);
        let eq190_e2393_d_n16: f64 = (eq190_e2391_d_n16 * p.p248);
        let eq190_e2393_d_n17: f64 = (eq190_e2391_d_n17 * p.p248);
        let eq190_e2393_d_n18: f64 = (eq190_e2391_d_n18 * p.p248);
        let eq190_e2393_d_n19: f64 = (eq190_e2391_d_n19 * p.p248);
        let eq190_e2393_d_n20: f64 = (eq190_e2391_d_n20 * p.p248);
        let eq190_e2393_d_n21: f64 = (eq190_e2391_d_n21 * p.p248);
        let eq190_e2393_d_n22: f64 = (eq190_e2391_d_n22 * p.p248);
        (eq190_e2393, eq190_e2393_d_n0, eq190_e2393_d_n1, eq190_e2393_d_n2, eq190_e2393_d_n3, eq190_e2393_d_n4, eq190_e2393_d_n5, eq190_e2393_d_n6, eq190_e2393_d_n7, eq190_e2393_d_n8, eq190_e2393_d_n9, eq190_e2393_d_n10, eq190_e2393_d_n11, eq190_e2393_d_n12, eq190_e2393_d_n13, eq190_e2393_d_n14, eq190_e2393_d_n15, eq190_e2393_d_n16, eq190_e2393_d_n17, eq190_e2393_d_n18, eq190_e2393_d_n19, eq190_e2393_d_n20, eq190_e2393_d_n21, eq190_e2393_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq190_value: f64 = eq190_e2395;
        let eq190_node_derivatives: [f64; 23] = [eq190_e2395_d_n0, eq190_e2395_d_n1, eq190_e2395_d_n2, eq190_e2395_d_n3, eq190_e2395_d_n4, eq190_e2395_d_n5, eq190_e2395_d_n6, eq190_e2395_d_n7, eq190_e2395_d_n8, eq190_e2395_d_n9, eq190_e2395_d_n10, eq190_e2395_d_n11, eq190_e2395_d_n12, eq190_e2395_d_n13, eq190_e2395_d_n14, eq190_e2395_d_n15, eq190_e2395_d_n16, eq190_e2395_d_n17, eq190_e2395_d_n18, eq190_e2395_d_n19, eq190_e2395_d_n20, eq190_e2395_d_n21, eq190_e2395_d_n22];
        let eq190_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            self.multiplicity * (eq190_value),
            &nodes,
            &eq190_node_derivatives,
            &branches,
            &eq190_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_191_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq191_e2407, eq191_e2407_d_n0, eq191_e2407_d_n1, eq191_e2407_d_n2, eq191_e2407_d_n3, eq191_e2407_d_n4, eq191_e2407_d_n5, eq191_e2407_d_n6, eq191_e2407_d_n7, eq191_e2407_d_n8, eq191_e2407_d_n9, eq191_e2407_d_n10, eq191_e2407_d_n11, eq191_e2407_d_n12, eq191_e2407_d_n13, eq191_e2407_d_n14, eq191_e2407_d_n15, eq191_e2407_d_n16, eq191_e2407_d_n17, eq191_e2407_d_n18, eq191_e2407_d_n19, eq191_e2407_d_n20, eq191_e2407_d_n21, eq191_e2407_d_n22,) = {
    if ((!(s.v[595] != 0.0)) && (s.v[598] != 0.0)) {
        let eq191_e2403: f64 = (p.p253 * s.v[288]);
        let eq191_e2403_d_n0: f64 = (p.p253 * s.dn[288][0]);
        let eq191_e2403_d_n1: f64 = (p.p253 * s.dn[288][1]);
        let eq191_e2403_d_n2: f64 = (p.p253 * s.dn[288][2]);
        let eq191_e2403_d_n3: f64 = (p.p253 * s.dn[288][3]);
        let eq191_e2403_d_n4: f64 = (p.p253 * s.dn[288][4]);
        let eq191_e2403_d_n5: f64 = (p.p253 * s.dn[288][5]);
        let eq191_e2403_d_n6: f64 = (p.p253 * s.dn[288][6]);
        let eq191_e2403_d_n7: f64 = (p.p253 * s.dn[288][7]);
        let eq191_e2403_d_n8: f64 = (p.p253 * s.dn[288][8]);
        let eq191_e2403_d_n9: f64 = (p.p253 * s.dn[288][9]);
        let eq191_e2403_d_n10: f64 = (p.p253 * s.dn[288][10]);
        let eq191_e2403_d_n11: f64 = (p.p253 * s.dn[288][11]);
        let eq191_e2403_d_n12: f64 = (p.p253 * s.dn[288][12]);
        let eq191_e2403_d_n13: f64 = (p.p253 * s.dn[288][13]);
        let eq191_e2403_d_n14: f64 = (p.p253 * s.dn[288][14]);
        let eq191_e2403_d_n15: f64 = (p.p253 * s.dn[288][15]);
        let eq191_e2403_d_n16: f64 = (p.p253 * s.dn[288][16]);
        let eq191_e2403_d_n17: f64 = (p.p253 * s.dn[288][17]);
        let eq191_e2403_d_n18: f64 = (p.p253 * s.dn[288][18]);
        let eq191_e2403_d_n19: f64 = (p.p253 * s.dn[288][19]);
        let eq191_e2403_d_n20: f64 = (p.p253 * s.dn[288][20]);
        let eq191_e2403_d_n21: f64 = (p.p253 * s.dn[288][21]);
        let eq191_e2403_d_n22: f64 = (p.p253 * s.dn[288][22]);
        let eq191_e2404: f64 = self.eval_ddt(90, eq191_e2403);
        let eq191_e2404_d_n0: f64 = self.ddt_jacobian(eq191_e2403_d_n0);
        let eq191_e2404_d_n1: f64 = self.ddt_jacobian(eq191_e2403_d_n1);
        let eq191_e2404_d_n2: f64 = self.ddt_jacobian(eq191_e2403_d_n2);
        let eq191_e2404_d_n3: f64 = self.ddt_jacobian(eq191_e2403_d_n3);
        let eq191_e2404_d_n4: f64 = self.ddt_jacobian(eq191_e2403_d_n4);
        let eq191_e2404_d_n5: f64 = self.ddt_jacobian(eq191_e2403_d_n5);
        let eq191_e2404_d_n6: f64 = self.ddt_jacobian(eq191_e2403_d_n6);
        let eq191_e2404_d_n7: f64 = self.ddt_jacobian(eq191_e2403_d_n7);
        let eq191_e2404_d_n8: f64 = self.ddt_jacobian(eq191_e2403_d_n8);
        let eq191_e2404_d_n9: f64 = self.ddt_jacobian(eq191_e2403_d_n9);
        let eq191_e2404_d_n10: f64 = self.ddt_jacobian(eq191_e2403_d_n10);
        let eq191_e2404_d_n11: f64 = self.ddt_jacobian(eq191_e2403_d_n11);
        let eq191_e2404_d_n12: f64 = self.ddt_jacobian(eq191_e2403_d_n12);
        let eq191_e2404_d_n13: f64 = self.ddt_jacobian(eq191_e2403_d_n13);
        let eq191_e2404_d_n14: f64 = self.ddt_jacobian(eq191_e2403_d_n14);
        let eq191_e2404_d_n15: f64 = self.ddt_jacobian(eq191_e2403_d_n15);
        let eq191_e2404_d_n16: f64 = self.ddt_jacobian(eq191_e2403_d_n16);
        let eq191_e2404_d_n17: f64 = self.ddt_jacobian(eq191_e2403_d_n17);
        let eq191_e2404_d_n18: f64 = self.ddt_jacobian(eq191_e2403_d_n18);
        let eq191_e2404_d_n19: f64 = self.ddt_jacobian(eq191_e2403_d_n19);
        let eq191_e2404_d_n20: f64 = self.ddt_jacobian(eq191_e2403_d_n20);
        let eq191_e2404_d_n21: f64 = self.ddt_jacobian(eq191_e2403_d_n21);
        let eq191_e2404_d_n22: f64 = self.ddt_jacobian(eq191_e2403_d_n22);
        let eq191_e2405: f64 = (p.p7 * eq191_e2404);
        let eq191_e2405_d_n0: f64 = (p.p7 * eq191_e2404_d_n0);
        let eq191_e2405_d_n1: f64 = (p.p7 * eq191_e2404_d_n1);
        let eq191_e2405_d_n2: f64 = (p.p7 * eq191_e2404_d_n2);
        let eq191_e2405_d_n3: f64 = (p.p7 * eq191_e2404_d_n3);
        let eq191_e2405_d_n4: f64 = (p.p7 * eq191_e2404_d_n4);
        let eq191_e2405_d_n5: f64 = (p.p7 * eq191_e2404_d_n5);
        let eq191_e2405_d_n6: f64 = (p.p7 * eq191_e2404_d_n6);
        let eq191_e2405_d_n7: f64 = (p.p7 * eq191_e2404_d_n7);
        let eq191_e2405_d_n8: f64 = (p.p7 * eq191_e2404_d_n8);
        let eq191_e2405_d_n9: f64 = (p.p7 * eq191_e2404_d_n9);
        let eq191_e2405_d_n10: f64 = (p.p7 * eq191_e2404_d_n10);
        let eq191_e2405_d_n11: f64 = (p.p7 * eq191_e2404_d_n11);
        let eq191_e2405_d_n12: f64 = (p.p7 * eq191_e2404_d_n12);
        let eq191_e2405_d_n13: f64 = (p.p7 * eq191_e2404_d_n13);
        let eq191_e2405_d_n14: f64 = (p.p7 * eq191_e2404_d_n14);
        let eq191_e2405_d_n15: f64 = (p.p7 * eq191_e2404_d_n15);
        let eq191_e2405_d_n16: f64 = (p.p7 * eq191_e2404_d_n16);
        let eq191_e2405_d_n17: f64 = (p.p7 * eq191_e2404_d_n17);
        let eq191_e2405_d_n18: f64 = (p.p7 * eq191_e2404_d_n18);
        let eq191_e2405_d_n19: f64 = (p.p7 * eq191_e2404_d_n19);
        let eq191_e2405_d_n20: f64 = (p.p7 * eq191_e2404_d_n20);
        let eq191_e2405_d_n21: f64 = (p.p7 * eq191_e2404_d_n21);
        let eq191_e2405_d_n22: f64 = (p.p7 * eq191_e2404_d_n22);
        (eq191_e2405, eq191_e2405_d_n0, eq191_e2405_d_n1, eq191_e2405_d_n2, eq191_e2405_d_n3, eq191_e2405_d_n4, eq191_e2405_d_n5, eq191_e2405_d_n6, eq191_e2405_d_n7, eq191_e2405_d_n8, eq191_e2405_d_n9, eq191_e2405_d_n10, eq191_e2405_d_n11, eq191_e2405_d_n12, eq191_e2405_d_n13, eq191_e2405_d_n14, eq191_e2405_d_n15, eq191_e2405_d_n16, eq191_e2405_d_n17, eq191_e2405_d_n18, eq191_e2405_d_n19, eq191_e2405_d_n20, eq191_e2405_d_n21, eq191_e2405_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq191_value: f64 = eq191_e2407;
        let eq191_node_derivatives: [f64; 23] = [eq191_e2407_d_n0, eq191_e2407_d_n1, eq191_e2407_d_n2, eq191_e2407_d_n3, eq191_e2407_d_n4, eq191_e2407_d_n5, eq191_e2407_d_n6, eq191_e2407_d_n7, eq191_e2407_d_n8, eq191_e2407_d_n9, eq191_e2407_d_n10, eq191_e2407_d_n11, eq191_e2407_d_n12, eq191_e2407_d_n13, eq191_e2407_d_n14, eq191_e2407_d_n15, eq191_e2407_d_n16, eq191_e2407_d_n17, eq191_e2407_d_n18, eq191_e2407_d_n19, eq191_e2407_d_n20, eq191_e2407_d_n21, eq191_e2407_d_n22];
        let eq191_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            self.multiplicity * (eq191_value),
            &nodes,
            &eq191_node_derivatives,
            &branches,
            &eq191_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_192_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq192_e2416, eq192_e2416_d_n0, eq192_e2416_d_n1, eq192_e2416_d_n2, eq192_e2416_d_n3, eq192_e2416_d_n4, eq192_e2416_d_n5, eq192_e2416_d_n6, eq192_e2416_d_n7, eq192_e2416_d_n8, eq192_e2416_d_n9, eq192_e2416_d_n10, eq192_e2416_d_n11, eq192_e2416_d_n12, eq192_e2416_d_n13, eq192_e2416_d_n14, eq192_e2416_d_n15, eq192_e2416_d_n16, eq192_e2416_d_n17, eq192_e2416_d_n18, eq192_e2416_d_n19, eq192_e2416_d_n20, eq192_e2416_d_n21, eq192_e2416_d_n22,) = {
    if ((s.v[600] != 0.0) && (s.v[601] != 0.0)) {
        let eq192_e2413: f64 = self.eval_ddt(91, s.v[301]);
        let eq192_e2413_d_n0: f64 = self.ddt_jacobian(s.dn[301][0]);
        let eq192_e2413_d_n1: f64 = self.ddt_jacobian(s.dn[301][1]);
        let eq192_e2413_d_n2: f64 = self.ddt_jacobian(s.dn[301][2]);
        let eq192_e2413_d_n3: f64 = self.ddt_jacobian(s.dn[301][3]);
        let eq192_e2413_d_n4: f64 = self.ddt_jacobian(s.dn[301][4]);
        let eq192_e2413_d_n5: f64 = self.ddt_jacobian(s.dn[301][5]);
        let eq192_e2413_d_n6: f64 = self.ddt_jacobian(s.dn[301][6]);
        let eq192_e2413_d_n7: f64 = self.ddt_jacobian(s.dn[301][7]);
        let eq192_e2413_d_n8: f64 = self.ddt_jacobian(s.dn[301][8]);
        let eq192_e2413_d_n9: f64 = self.ddt_jacobian(s.dn[301][9]);
        let eq192_e2413_d_n10: f64 = self.ddt_jacobian(s.dn[301][10]);
        let eq192_e2413_d_n11: f64 = self.ddt_jacobian(s.dn[301][11]);
        let eq192_e2413_d_n12: f64 = self.ddt_jacobian(s.dn[301][12]);
        let eq192_e2413_d_n13: f64 = self.ddt_jacobian(s.dn[301][13]);
        let eq192_e2413_d_n14: f64 = self.ddt_jacobian(s.dn[301][14]);
        let eq192_e2413_d_n15: f64 = self.ddt_jacobian(s.dn[301][15]);
        let eq192_e2413_d_n16: f64 = self.ddt_jacobian(s.dn[301][16]);
        let eq192_e2413_d_n17: f64 = self.ddt_jacobian(s.dn[301][17]);
        let eq192_e2413_d_n18: f64 = self.ddt_jacobian(s.dn[301][18]);
        let eq192_e2413_d_n19: f64 = self.ddt_jacobian(s.dn[301][19]);
        let eq192_e2413_d_n20: f64 = self.ddt_jacobian(s.dn[301][20]);
        let eq192_e2413_d_n21: f64 = self.ddt_jacobian(s.dn[301][21]);
        let eq192_e2413_d_n22: f64 = self.ddt_jacobian(s.dn[301][22]);
        let eq192_e2414: f64 = (p.p7 * eq192_e2413);
        let eq192_e2414_d_n0: f64 = (p.p7 * eq192_e2413_d_n0);
        let eq192_e2414_d_n1: f64 = (p.p7 * eq192_e2413_d_n1);
        let eq192_e2414_d_n2: f64 = (p.p7 * eq192_e2413_d_n2);
        let eq192_e2414_d_n3: f64 = (p.p7 * eq192_e2413_d_n3);
        let eq192_e2414_d_n4: f64 = (p.p7 * eq192_e2413_d_n4);
        let eq192_e2414_d_n5: f64 = (p.p7 * eq192_e2413_d_n5);
        let eq192_e2414_d_n6: f64 = (p.p7 * eq192_e2413_d_n6);
        let eq192_e2414_d_n7: f64 = (p.p7 * eq192_e2413_d_n7);
        let eq192_e2414_d_n8: f64 = (p.p7 * eq192_e2413_d_n8);
        let eq192_e2414_d_n9: f64 = (p.p7 * eq192_e2413_d_n9);
        let eq192_e2414_d_n10: f64 = (p.p7 * eq192_e2413_d_n10);
        let eq192_e2414_d_n11: f64 = (p.p7 * eq192_e2413_d_n11);
        let eq192_e2414_d_n12: f64 = (p.p7 * eq192_e2413_d_n12);
        let eq192_e2414_d_n13: f64 = (p.p7 * eq192_e2413_d_n13);
        let eq192_e2414_d_n14: f64 = (p.p7 * eq192_e2413_d_n14);
        let eq192_e2414_d_n15: f64 = (p.p7 * eq192_e2413_d_n15);
        let eq192_e2414_d_n16: f64 = (p.p7 * eq192_e2413_d_n16);
        let eq192_e2414_d_n17: f64 = (p.p7 * eq192_e2413_d_n17);
        let eq192_e2414_d_n18: f64 = (p.p7 * eq192_e2413_d_n18);
        let eq192_e2414_d_n19: f64 = (p.p7 * eq192_e2413_d_n19);
        let eq192_e2414_d_n20: f64 = (p.p7 * eq192_e2413_d_n20);
        let eq192_e2414_d_n21: f64 = (p.p7 * eq192_e2413_d_n21);
        let eq192_e2414_d_n22: f64 = (p.p7 * eq192_e2413_d_n22);
        (eq192_e2414, eq192_e2414_d_n0, eq192_e2414_d_n1, eq192_e2414_d_n2, eq192_e2414_d_n3, eq192_e2414_d_n4, eq192_e2414_d_n5, eq192_e2414_d_n6, eq192_e2414_d_n7, eq192_e2414_d_n8, eq192_e2414_d_n9, eq192_e2414_d_n10, eq192_e2414_d_n11, eq192_e2414_d_n12, eq192_e2414_d_n13, eq192_e2414_d_n14, eq192_e2414_d_n15, eq192_e2414_d_n16, eq192_e2414_d_n17, eq192_e2414_d_n18, eq192_e2414_d_n19, eq192_e2414_d_n20, eq192_e2414_d_n21, eq192_e2414_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq192_value: f64 = eq192_e2416;
        let eq192_node_derivatives: [f64; 23] = [eq192_e2416_d_n0, eq192_e2416_d_n1, eq192_e2416_d_n2, eq192_e2416_d_n3, eq192_e2416_d_n4, eq192_e2416_d_n5, eq192_e2416_d_n6, eq192_e2416_d_n7, eq192_e2416_d_n8, eq192_e2416_d_n9, eq192_e2416_d_n10, eq192_e2416_d_n11, eq192_e2416_d_n12, eq192_e2416_d_n13, eq192_e2416_d_n14, eq192_e2416_d_n15, eq192_e2416_d_n16, eq192_e2416_d_n17, eq192_e2416_d_n18, eq192_e2416_d_n19, eq192_e2416_d_n20, eq192_e2416_d_n21, eq192_e2416_d_n22];
        let eq192_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[18]),
            Some(nodes[17]),
            self.multiplicity * (eq192_value),
            &nodes,
            &eq192_node_derivatives,
            &branches,
            &eq192_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_193_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq193_e2427, eq193_e2427_d_n0, eq193_e2427_d_n1, eq193_e2427_d_n2, eq193_e2427_d_n3, eq193_e2427_d_n4, eq193_e2427_d_n5, eq193_e2427_d_n6, eq193_e2427_d_n7, eq193_e2427_d_n8, eq193_e2427_d_n9, eq193_e2427_d_n10, eq193_e2427_d_n11, eq193_e2427_d_n12, eq193_e2427_d_n13, eq193_e2427_d_n14, eq193_e2427_d_n15, eq193_e2427_d_n16, eq193_e2427_d_n17, eq193_e2427_d_n18, eq193_e2427_d_n19, eq193_e2427_d_n20, eq193_e2427_d_n21, eq193_e2427_d_n22,) = {
    if (((s.v[600] != 0.0) && (s.v[601] != 0.0)) && (s.v[602] != 0.0)) {
        let eq193_e2424: f64 = self.eval_ddt(92, s.v[300]);
        let eq193_e2424_d_n0: f64 = self.ddt_jacobian(s.dn[300][0]);
        let eq193_e2424_d_n1: f64 = self.ddt_jacobian(s.dn[300][1]);
        let eq193_e2424_d_n2: f64 = self.ddt_jacobian(s.dn[300][2]);
        let eq193_e2424_d_n3: f64 = self.ddt_jacobian(s.dn[300][3]);
        let eq193_e2424_d_n4: f64 = self.ddt_jacobian(s.dn[300][4]);
        let eq193_e2424_d_n5: f64 = self.ddt_jacobian(s.dn[300][5]);
        let eq193_e2424_d_n6: f64 = self.ddt_jacobian(s.dn[300][6]);
        let eq193_e2424_d_n7: f64 = self.ddt_jacobian(s.dn[300][7]);
        let eq193_e2424_d_n8: f64 = self.ddt_jacobian(s.dn[300][8]);
        let eq193_e2424_d_n9: f64 = self.ddt_jacobian(s.dn[300][9]);
        let eq193_e2424_d_n10: f64 = self.ddt_jacobian(s.dn[300][10]);
        let eq193_e2424_d_n11: f64 = self.ddt_jacobian(s.dn[300][11]);
        let eq193_e2424_d_n12: f64 = self.ddt_jacobian(s.dn[300][12]);
        let eq193_e2424_d_n13: f64 = self.ddt_jacobian(s.dn[300][13]);
        let eq193_e2424_d_n14: f64 = self.ddt_jacobian(s.dn[300][14]);
        let eq193_e2424_d_n15: f64 = self.ddt_jacobian(s.dn[300][15]);
        let eq193_e2424_d_n16: f64 = self.ddt_jacobian(s.dn[300][16]);
        let eq193_e2424_d_n17: f64 = self.ddt_jacobian(s.dn[300][17]);
        let eq193_e2424_d_n18: f64 = self.ddt_jacobian(s.dn[300][18]);
        let eq193_e2424_d_n19: f64 = self.ddt_jacobian(s.dn[300][19]);
        let eq193_e2424_d_n20: f64 = self.ddt_jacobian(s.dn[300][20]);
        let eq193_e2424_d_n21: f64 = self.ddt_jacobian(s.dn[300][21]);
        let eq193_e2424_d_n22: f64 = self.ddt_jacobian(s.dn[300][22]);
        let eq193_e2425: f64 = (p.p7 * eq193_e2424);
        let eq193_e2425_d_n0: f64 = (p.p7 * eq193_e2424_d_n0);
        let eq193_e2425_d_n1: f64 = (p.p7 * eq193_e2424_d_n1);
        let eq193_e2425_d_n2: f64 = (p.p7 * eq193_e2424_d_n2);
        let eq193_e2425_d_n3: f64 = (p.p7 * eq193_e2424_d_n3);
        let eq193_e2425_d_n4: f64 = (p.p7 * eq193_e2424_d_n4);
        let eq193_e2425_d_n5: f64 = (p.p7 * eq193_e2424_d_n5);
        let eq193_e2425_d_n6: f64 = (p.p7 * eq193_e2424_d_n6);
        let eq193_e2425_d_n7: f64 = (p.p7 * eq193_e2424_d_n7);
        let eq193_e2425_d_n8: f64 = (p.p7 * eq193_e2424_d_n8);
        let eq193_e2425_d_n9: f64 = (p.p7 * eq193_e2424_d_n9);
        let eq193_e2425_d_n10: f64 = (p.p7 * eq193_e2424_d_n10);
        let eq193_e2425_d_n11: f64 = (p.p7 * eq193_e2424_d_n11);
        let eq193_e2425_d_n12: f64 = (p.p7 * eq193_e2424_d_n12);
        let eq193_e2425_d_n13: f64 = (p.p7 * eq193_e2424_d_n13);
        let eq193_e2425_d_n14: f64 = (p.p7 * eq193_e2424_d_n14);
        let eq193_e2425_d_n15: f64 = (p.p7 * eq193_e2424_d_n15);
        let eq193_e2425_d_n16: f64 = (p.p7 * eq193_e2424_d_n16);
        let eq193_e2425_d_n17: f64 = (p.p7 * eq193_e2424_d_n17);
        let eq193_e2425_d_n18: f64 = (p.p7 * eq193_e2424_d_n18);
        let eq193_e2425_d_n19: f64 = (p.p7 * eq193_e2424_d_n19);
        let eq193_e2425_d_n20: f64 = (p.p7 * eq193_e2424_d_n20);
        let eq193_e2425_d_n21: f64 = (p.p7 * eq193_e2424_d_n21);
        let eq193_e2425_d_n22: f64 = (p.p7 * eq193_e2424_d_n22);
        (eq193_e2425, eq193_e2425_d_n0, eq193_e2425_d_n1, eq193_e2425_d_n2, eq193_e2425_d_n3, eq193_e2425_d_n4, eq193_e2425_d_n5, eq193_e2425_d_n6, eq193_e2425_d_n7, eq193_e2425_d_n8, eq193_e2425_d_n9, eq193_e2425_d_n10, eq193_e2425_d_n11, eq193_e2425_d_n12, eq193_e2425_d_n13, eq193_e2425_d_n14, eq193_e2425_d_n15, eq193_e2425_d_n16, eq193_e2425_d_n17, eq193_e2425_d_n18, eq193_e2425_d_n19, eq193_e2425_d_n20, eq193_e2425_d_n21, eq193_e2425_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq193_value: f64 = eq193_e2427;
        let eq193_node_derivatives: [f64; 23] = [eq193_e2427_d_n0, eq193_e2427_d_n1, eq193_e2427_d_n2, eq193_e2427_d_n3, eq193_e2427_d_n4, eq193_e2427_d_n5, eq193_e2427_d_n6, eq193_e2427_d_n7, eq193_e2427_d_n8, eq193_e2427_d_n9, eq193_e2427_d_n10, eq193_e2427_d_n11, eq193_e2427_d_n12, eq193_e2427_d_n13, eq193_e2427_d_n14, eq193_e2427_d_n15, eq193_e2427_d_n16, eq193_e2427_d_n17, eq193_e2427_d_n18, eq193_e2427_d_n19, eq193_e2427_d_n20, eq193_e2427_d_n21, eq193_e2427_d_n22];
        let eq193_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[17]),
            self.multiplicity * (eq193_value),
            &nodes,
            &eq193_node_derivatives,
            &branches,
            &eq193_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_194_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq194_e2440, eq194_e2440_d_n0, eq194_e2440_d_n1, eq194_e2440_d_n2, eq194_e2440_d_n3, eq194_e2440_d_n4, eq194_e2440_d_n5, eq194_e2440_d_n6, eq194_e2440_d_n7, eq194_e2440_d_n8, eq194_e2440_d_n9, eq194_e2440_d_n10, eq194_e2440_d_n11, eq194_e2440_d_n12, eq194_e2440_d_n13, eq194_e2440_d_n14, eq194_e2440_d_n15, eq194_e2440_d_n16, eq194_e2440_d_n17, eq194_e2440_d_n18, eq194_e2440_d_n19, eq194_e2440_d_n20, eq194_e2440_d_n21, eq194_e2440_d_n22,) = {
    if (((s.v[600] != 0.0) && (s.v[601] != 0.0)) && (s.v[602] != 0.0)) {
        let eq194_e2435: f64 = self.eval_ddt(93, s.v[300]);
        let eq194_e2435_d_n0: f64 = self.ddt_jacobian(s.dn[300][0]);
        let eq194_e2435_d_n1: f64 = self.ddt_jacobian(s.dn[300][1]);
        let eq194_e2435_d_n2: f64 = self.ddt_jacobian(s.dn[300][2]);
        let eq194_e2435_d_n3: f64 = self.ddt_jacobian(s.dn[300][3]);
        let eq194_e2435_d_n4: f64 = self.ddt_jacobian(s.dn[300][4]);
        let eq194_e2435_d_n5: f64 = self.ddt_jacobian(s.dn[300][5]);
        let eq194_e2435_d_n6: f64 = self.ddt_jacobian(s.dn[300][6]);
        let eq194_e2435_d_n7: f64 = self.ddt_jacobian(s.dn[300][7]);
        let eq194_e2435_d_n8: f64 = self.ddt_jacobian(s.dn[300][8]);
        let eq194_e2435_d_n9: f64 = self.ddt_jacobian(s.dn[300][9]);
        let eq194_e2435_d_n10: f64 = self.ddt_jacobian(s.dn[300][10]);
        let eq194_e2435_d_n11: f64 = self.ddt_jacobian(s.dn[300][11]);
        let eq194_e2435_d_n12: f64 = self.ddt_jacobian(s.dn[300][12]);
        let eq194_e2435_d_n13: f64 = self.ddt_jacobian(s.dn[300][13]);
        let eq194_e2435_d_n14: f64 = self.ddt_jacobian(s.dn[300][14]);
        let eq194_e2435_d_n15: f64 = self.ddt_jacobian(s.dn[300][15]);
        let eq194_e2435_d_n16: f64 = self.ddt_jacobian(s.dn[300][16]);
        let eq194_e2435_d_n17: f64 = self.ddt_jacobian(s.dn[300][17]);
        let eq194_e2435_d_n18: f64 = self.ddt_jacobian(s.dn[300][18]);
        let eq194_e2435_d_n19: f64 = self.ddt_jacobian(s.dn[300][19]);
        let eq194_e2435_d_n20: f64 = self.ddt_jacobian(s.dn[300][20]);
        let eq194_e2435_d_n21: f64 = self.ddt_jacobian(s.dn[300][21]);
        let eq194_e2435_d_n22: f64 = self.ddt_jacobian(s.dn[300][22]);
        let eq194_e2436: f64 = (p.p7 * eq194_e2435);
        let eq194_e2436_d_n0: f64 = (p.p7 * eq194_e2435_d_n0);
        let eq194_e2436_d_n1: f64 = (p.p7 * eq194_e2435_d_n1);
        let eq194_e2436_d_n2: f64 = (p.p7 * eq194_e2435_d_n2);
        let eq194_e2436_d_n3: f64 = (p.p7 * eq194_e2435_d_n3);
        let eq194_e2436_d_n4: f64 = (p.p7 * eq194_e2435_d_n4);
        let eq194_e2436_d_n5: f64 = (p.p7 * eq194_e2435_d_n5);
        let eq194_e2436_d_n6: f64 = (p.p7 * eq194_e2435_d_n6);
        let eq194_e2436_d_n7: f64 = (p.p7 * eq194_e2435_d_n7);
        let eq194_e2436_d_n8: f64 = (p.p7 * eq194_e2435_d_n8);
        let eq194_e2436_d_n9: f64 = (p.p7 * eq194_e2435_d_n9);
        let eq194_e2436_d_n10: f64 = (p.p7 * eq194_e2435_d_n10);
        let eq194_e2436_d_n11: f64 = (p.p7 * eq194_e2435_d_n11);
        let eq194_e2436_d_n12: f64 = (p.p7 * eq194_e2435_d_n12);
        let eq194_e2436_d_n13: f64 = (p.p7 * eq194_e2435_d_n13);
        let eq194_e2436_d_n14: f64 = (p.p7 * eq194_e2435_d_n14);
        let eq194_e2436_d_n15: f64 = (p.p7 * eq194_e2435_d_n15);
        let eq194_e2436_d_n16: f64 = (p.p7 * eq194_e2435_d_n16);
        let eq194_e2436_d_n17: f64 = (p.p7 * eq194_e2435_d_n17);
        let eq194_e2436_d_n18: f64 = (p.p7 * eq194_e2435_d_n18);
        let eq194_e2436_d_n19: f64 = (p.p7 * eq194_e2435_d_n19);
        let eq194_e2436_d_n20: f64 = (p.p7 * eq194_e2435_d_n20);
        let eq194_e2436_d_n21: f64 = (p.p7 * eq194_e2435_d_n21);
        let eq194_e2436_d_n22: f64 = (p.p7 * eq194_e2435_d_n22);
        let eq194_e2438: f64 = (eq194_e2436 * p.p249);
        let eq194_e2438_d_n0: f64 = (eq194_e2436_d_n0 * p.p249);
        let eq194_e2438_d_n1: f64 = (eq194_e2436_d_n1 * p.p249);
        let eq194_e2438_d_n2: f64 = (eq194_e2436_d_n2 * p.p249);
        let eq194_e2438_d_n3: f64 = (eq194_e2436_d_n3 * p.p249);
        let eq194_e2438_d_n4: f64 = (eq194_e2436_d_n4 * p.p249);
        let eq194_e2438_d_n5: f64 = (eq194_e2436_d_n5 * p.p249);
        let eq194_e2438_d_n6: f64 = (eq194_e2436_d_n6 * p.p249);
        let eq194_e2438_d_n7: f64 = (eq194_e2436_d_n7 * p.p249);
        let eq194_e2438_d_n8: f64 = (eq194_e2436_d_n8 * p.p249);
        let eq194_e2438_d_n9: f64 = (eq194_e2436_d_n9 * p.p249);
        let eq194_e2438_d_n10: f64 = (eq194_e2436_d_n10 * p.p249);
        let eq194_e2438_d_n11: f64 = (eq194_e2436_d_n11 * p.p249);
        let eq194_e2438_d_n12: f64 = (eq194_e2436_d_n12 * p.p249);
        let eq194_e2438_d_n13: f64 = (eq194_e2436_d_n13 * p.p249);
        let eq194_e2438_d_n14: f64 = (eq194_e2436_d_n14 * p.p249);
        let eq194_e2438_d_n15: f64 = (eq194_e2436_d_n15 * p.p249);
        let eq194_e2438_d_n16: f64 = (eq194_e2436_d_n16 * p.p249);
        let eq194_e2438_d_n17: f64 = (eq194_e2436_d_n17 * p.p249);
        let eq194_e2438_d_n18: f64 = (eq194_e2436_d_n18 * p.p249);
        let eq194_e2438_d_n19: f64 = (eq194_e2436_d_n19 * p.p249);
        let eq194_e2438_d_n20: f64 = (eq194_e2436_d_n20 * p.p249);
        let eq194_e2438_d_n21: f64 = (eq194_e2436_d_n21 * p.p249);
        let eq194_e2438_d_n22: f64 = (eq194_e2436_d_n22 * p.p249);
        (eq194_e2438, eq194_e2438_d_n0, eq194_e2438_d_n1, eq194_e2438_d_n2, eq194_e2438_d_n3, eq194_e2438_d_n4, eq194_e2438_d_n5, eq194_e2438_d_n6, eq194_e2438_d_n7, eq194_e2438_d_n8, eq194_e2438_d_n9, eq194_e2438_d_n10, eq194_e2438_d_n11, eq194_e2438_d_n12, eq194_e2438_d_n13, eq194_e2438_d_n14, eq194_e2438_d_n15, eq194_e2438_d_n16, eq194_e2438_d_n17, eq194_e2438_d_n18, eq194_e2438_d_n19, eq194_e2438_d_n20, eq194_e2438_d_n21, eq194_e2438_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq194_value: f64 = eq194_e2440;
        let eq194_node_derivatives: [f64; 23] = [eq194_e2440_d_n0, eq194_e2440_d_n1, eq194_e2440_d_n2, eq194_e2440_d_n3, eq194_e2440_d_n4, eq194_e2440_d_n5, eq194_e2440_d_n6, eq194_e2440_d_n7, eq194_e2440_d_n8, eq194_e2440_d_n9, eq194_e2440_d_n10, eq194_e2440_d_n11, eq194_e2440_d_n12, eq194_e2440_d_n13, eq194_e2440_d_n14, eq194_e2440_d_n15, eq194_e2440_d_n16, eq194_e2440_d_n17, eq194_e2440_d_n18, eq194_e2440_d_n19, eq194_e2440_d_n20, eq194_e2440_d_n21, eq194_e2440_d_n22];
        let eq194_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[17]),
            self.multiplicity * (eq194_value),
            &nodes,
            &eq194_node_derivatives,
            &branches,
            &eq194_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_195_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq195_e2452, eq195_e2452_d_n0, eq195_e2452_d_n1, eq195_e2452_d_n2, eq195_e2452_d_n3, eq195_e2452_d_n4, eq195_e2452_d_n5, eq195_e2452_d_n6, eq195_e2452_d_n7, eq195_e2452_d_n8, eq195_e2452_d_n9, eq195_e2452_d_n10, eq195_e2452_d_n11, eq195_e2452_d_n12, eq195_e2452_d_n13, eq195_e2452_d_n14, eq195_e2452_d_n15, eq195_e2452_d_n16, eq195_e2452_d_n17, eq195_e2452_d_n18, eq195_e2452_d_n19, eq195_e2452_d_n20, eq195_e2452_d_n21, eq195_e2452_d_n22,) = {
    if (((s.v[600] != 0.0) && (s.v[601] != 0.0)) && (!(s.v[602] != 0.0))) {
        let eq195_e2449: f64 = self.eval_ddt(94, s.v[300]);
        let eq195_e2449_d_n0: f64 = self.ddt_jacobian(s.dn[300][0]);
        let eq195_e2449_d_n1: f64 = self.ddt_jacobian(s.dn[300][1]);
        let eq195_e2449_d_n2: f64 = self.ddt_jacobian(s.dn[300][2]);
        let eq195_e2449_d_n3: f64 = self.ddt_jacobian(s.dn[300][3]);
        let eq195_e2449_d_n4: f64 = self.ddt_jacobian(s.dn[300][4]);
        let eq195_e2449_d_n5: f64 = self.ddt_jacobian(s.dn[300][5]);
        let eq195_e2449_d_n6: f64 = self.ddt_jacobian(s.dn[300][6]);
        let eq195_e2449_d_n7: f64 = self.ddt_jacobian(s.dn[300][7]);
        let eq195_e2449_d_n8: f64 = self.ddt_jacobian(s.dn[300][8]);
        let eq195_e2449_d_n9: f64 = self.ddt_jacobian(s.dn[300][9]);
        let eq195_e2449_d_n10: f64 = self.ddt_jacobian(s.dn[300][10]);
        let eq195_e2449_d_n11: f64 = self.ddt_jacobian(s.dn[300][11]);
        let eq195_e2449_d_n12: f64 = self.ddt_jacobian(s.dn[300][12]);
        let eq195_e2449_d_n13: f64 = self.ddt_jacobian(s.dn[300][13]);
        let eq195_e2449_d_n14: f64 = self.ddt_jacobian(s.dn[300][14]);
        let eq195_e2449_d_n15: f64 = self.ddt_jacobian(s.dn[300][15]);
        let eq195_e2449_d_n16: f64 = self.ddt_jacobian(s.dn[300][16]);
        let eq195_e2449_d_n17: f64 = self.ddt_jacobian(s.dn[300][17]);
        let eq195_e2449_d_n18: f64 = self.ddt_jacobian(s.dn[300][18]);
        let eq195_e2449_d_n19: f64 = self.ddt_jacobian(s.dn[300][19]);
        let eq195_e2449_d_n20: f64 = self.ddt_jacobian(s.dn[300][20]);
        let eq195_e2449_d_n21: f64 = self.ddt_jacobian(s.dn[300][21]);
        let eq195_e2449_d_n22: f64 = self.ddt_jacobian(s.dn[300][22]);
        let eq195_e2450: f64 = (p.p7 * eq195_e2449);
        let eq195_e2450_d_n0: f64 = (p.p7 * eq195_e2449_d_n0);
        let eq195_e2450_d_n1: f64 = (p.p7 * eq195_e2449_d_n1);
        let eq195_e2450_d_n2: f64 = (p.p7 * eq195_e2449_d_n2);
        let eq195_e2450_d_n3: f64 = (p.p7 * eq195_e2449_d_n3);
        let eq195_e2450_d_n4: f64 = (p.p7 * eq195_e2449_d_n4);
        let eq195_e2450_d_n5: f64 = (p.p7 * eq195_e2449_d_n5);
        let eq195_e2450_d_n6: f64 = (p.p7 * eq195_e2449_d_n6);
        let eq195_e2450_d_n7: f64 = (p.p7 * eq195_e2449_d_n7);
        let eq195_e2450_d_n8: f64 = (p.p7 * eq195_e2449_d_n8);
        let eq195_e2450_d_n9: f64 = (p.p7 * eq195_e2449_d_n9);
        let eq195_e2450_d_n10: f64 = (p.p7 * eq195_e2449_d_n10);
        let eq195_e2450_d_n11: f64 = (p.p7 * eq195_e2449_d_n11);
        let eq195_e2450_d_n12: f64 = (p.p7 * eq195_e2449_d_n12);
        let eq195_e2450_d_n13: f64 = (p.p7 * eq195_e2449_d_n13);
        let eq195_e2450_d_n14: f64 = (p.p7 * eq195_e2449_d_n14);
        let eq195_e2450_d_n15: f64 = (p.p7 * eq195_e2449_d_n15);
        let eq195_e2450_d_n16: f64 = (p.p7 * eq195_e2449_d_n16);
        let eq195_e2450_d_n17: f64 = (p.p7 * eq195_e2449_d_n17);
        let eq195_e2450_d_n18: f64 = (p.p7 * eq195_e2449_d_n18);
        let eq195_e2450_d_n19: f64 = (p.p7 * eq195_e2449_d_n19);
        let eq195_e2450_d_n20: f64 = (p.p7 * eq195_e2449_d_n20);
        let eq195_e2450_d_n21: f64 = (p.p7 * eq195_e2449_d_n21);
        let eq195_e2450_d_n22: f64 = (p.p7 * eq195_e2449_d_n22);
        (eq195_e2450, eq195_e2450_d_n0, eq195_e2450_d_n1, eq195_e2450_d_n2, eq195_e2450_d_n3, eq195_e2450_d_n4, eq195_e2450_d_n5, eq195_e2450_d_n6, eq195_e2450_d_n7, eq195_e2450_d_n8, eq195_e2450_d_n9, eq195_e2450_d_n10, eq195_e2450_d_n11, eq195_e2450_d_n12, eq195_e2450_d_n13, eq195_e2450_d_n14, eq195_e2450_d_n15, eq195_e2450_d_n16, eq195_e2450_d_n17, eq195_e2450_d_n18, eq195_e2450_d_n19, eq195_e2450_d_n20, eq195_e2450_d_n21, eq195_e2450_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq195_value: f64 = eq195_e2452;
        let eq195_node_derivatives: [f64; 23] = [eq195_e2452_d_n0, eq195_e2452_d_n1, eq195_e2452_d_n2, eq195_e2452_d_n3, eq195_e2452_d_n4, eq195_e2452_d_n5, eq195_e2452_d_n6, eq195_e2452_d_n7, eq195_e2452_d_n8, eq195_e2452_d_n9, eq195_e2452_d_n10, eq195_e2452_d_n11, eq195_e2452_d_n12, eq195_e2452_d_n13, eq195_e2452_d_n14, eq195_e2452_d_n15, eq195_e2452_d_n16, eq195_e2452_d_n17, eq195_e2452_d_n18, eq195_e2452_d_n19, eq195_e2452_d_n20, eq195_e2452_d_n21, eq195_e2452_d_n22];
        let eq195_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[17]),
            self.multiplicity * (eq195_value),
            &nodes,
            &eq195_node_derivatives,
            &branches,
            &eq195_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_196_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq196_e2466, eq196_e2466_d_n0, eq196_e2466_d_n1, eq196_e2466_d_n2, eq196_e2466_d_n3, eq196_e2466_d_n4, eq196_e2466_d_n5, eq196_e2466_d_n6, eq196_e2466_d_n7, eq196_e2466_d_n8, eq196_e2466_d_n9, eq196_e2466_d_n10, eq196_e2466_d_n11, eq196_e2466_d_n12, eq196_e2466_d_n13, eq196_e2466_d_n14, eq196_e2466_d_n15, eq196_e2466_d_n16, eq196_e2466_d_n17, eq196_e2466_d_n18, eq196_e2466_d_n19, eq196_e2466_d_n20, eq196_e2466_d_n21, eq196_e2466_d_n22,) = {
    if (((s.v[600] != 0.0) && (s.v[601] != 0.0)) && (!(s.v[602] != 0.0))) {
        let eq196_e2461: f64 = self.eval_ddt(95, s.v[300]);
        let eq196_e2461_d_n0: f64 = self.ddt_jacobian(s.dn[300][0]);
        let eq196_e2461_d_n1: f64 = self.ddt_jacobian(s.dn[300][1]);
        let eq196_e2461_d_n2: f64 = self.ddt_jacobian(s.dn[300][2]);
        let eq196_e2461_d_n3: f64 = self.ddt_jacobian(s.dn[300][3]);
        let eq196_e2461_d_n4: f64 = self.ddt_jacobian(s.dn[300][4]);
        let eq196_e2461_d_n5: f64 = self.ddt_jacobian(s.dn[300][5]);
        let eq196_e2461_d_n6: f64 = self.ddt_jacobian(s.dn[300][6]);
        let eq196_e2461_d_n7: f64 = self.ddt_jacobian(s.dn[300][7]);
        let eq196_e2461_d_n8: f64 = self.ddt_jacobian(s.dn[300][8]);
        let eq196_e2461_d_n9: f64 = self.ddt_jacobian(s.dn[300][9]);
        let eq196_e2461_d_n10: f64 = self.ddt_jacobian(s.dn[300][10]);
        let eq196_e2461_d_n11: f64 = self.ddt_jacobian(s.dn[300][11]);
        let eq196_e2461_d_n12: f64 = self.ddt_jacobian(s.dn[300][12]);
        let eq196_e2461_d_n13: f64 = self.ddt_jacobian(s.dn[300][13]);
        let eq196_e2461_d_n14: f64 = self.ddt_jacobian(s.dn[300][14]);
        let eq196_e2461_d_n15: f64 = self.ddt_jacobian(s.dn[300][15]);
        let eq196_e2461_d_n16: f64 = self.ddt_jacobian(s.dn[300][16]);
        let eq196_e2461_d_n17: f64 = self.ddt_jacobian(s.dn[300][17]);
        let eq196_e2461_d_n18: f64 = self.ddt_jacobian(s.dn[300][18]);
        let eq196_e2461_d_n19: f64 = self.ddt_jacobian(s.dn[300][19]);
        let eq196_e2461_d_n20: f64 = self.ddt_jacobian(s.dn[300][20]);
        let eq196_e2461_d_n21: f64 = self.ddt_jacobian(s.dn[300][21]);
        let eq196_e2461_d_n22: f64 = self.ddt_jacobian(s.dn[300][22]);
        let eq196_e2462: f64 = (p.p7 * eq196_e2461);
        let eq196_e2462_d_n0: f64 = (p.p7 * eq196_e2461_d_n0);
        let eq196_e2462_d_n1: f64 = (p.p7 * eq196_e2461_d_n1);
        let eq196_e2462_d_n2: f64 = (p.p7 * eq196_e2461_d_n2);
        let eq196_e2462_d_n3: f64 = (p.p7 * eq196_e2461_d_n3);
        let eq196_e2462_d_n4: f64 = (p.p7 * eq196_e2461_d_n4);
        let eq196_e2462_d_n5: f64 = (p.p7 * eq196_e2461_d_n5);
        let eq196_e2462_d_n6: f64 = (p.p7 * eq196_e2461_d_n6);
        let eq196_e2462_d_n7: f64 = (p.p7 * eq196_e2461_d_n7);
        let eq196_e2462_d_n8: f64 = (p.p7 * eq196_e2461_d_n8);
        let eq196_e2462_d_n9: f64 = (p.p7 * eq196_e2461_d_n9);
        let eq196_e2462_d_n10: f64 = (p.p7 * eq196_e2461_d_n10);
        let eq196_e2462_d_n11: f64 = (p.p7 * eq196_e2461_d_n11);
        let eq196_e2462_d_n12: f64 = (p.p7 * eq196_e2461_d_n12);
        let eq196_e2462_d_n13: f64 = (p.p7 * eq196_e2461_d_n13);
        let eq196_e2462_d_n14: f64 = (p.p7 * eq196_e2461_d_n14);
        let eq196_e2462_d_n15: f64 = (p.p7 * eq196_e2461_d_n15);
        let eq196_e2462_d_n16: f64 = (p.p7 * eq196_e2461_d_n16);
        let eq196_e2462_d_n17: f64 = (p.p7 * eq196_e2461_d_n17);
        let eq196_e2462_d_n18: f64 = (p.p7 * eq196_e2461_d_n18);
        let eq196_e2462_d_n19: f64 = (p.p7 * eq196_e2461_d_n19);
        let eq196_e2462_d_n20: f64 = (p.p7 * eq196_e2461_d_n20);
        let eq196_e2462_d_n21: f64 = (p.p7 * eq196_e2461_d_n21);
        let eq196_e2462_d_n22: f64 = (p.p7 * eq196_e2461_d_n22);
        let eq196_e2464: f64 = (eq196_e2462 * p.p249);
        let eq196_e2464_d_n0: f64 = (eq196_e2462_d_n0 * p.p249);
        let eq196_e2464_d_n1: f64 = (eq196_e2462_d_n1 * p.p249);
        let eq196_e2464_d_n2: f64 = (eq196_e2462_d_n2 * p.p249);
        let eq196_e2464_d_n3: f64 = (eq196_e2462_d_n3 * p.p249);
        let eq196_e2464_d_n4: f64 = (eq196_e2462_d_n4 * p.p249);
        let eq196_e2464_d_n5: f64 = (eq196_e2462_d_n5 * p.p249);
        let eq196_e2464_d_n6: f64 = (eq196_e2462_d_n6 * p.p249);
        let eq196_e2464_d_n7: f64 = (eq196_e2462_d_n7 * p.p249);
        let eq196_e2464_d_n8: f64 = (eq196_e2462_d_n8 * p.p249);
        let eq196_e2464_d_n9: f64 = (eq196_e2462_d_n9 * p.p249);
        let eq196_e2464_d_n10: f64 = (eq196_e2462_d_n10 * p.p249);
        let eq196_e2464_d_n11: f64 = (eq196_e2462_d_n11 * p.p249);
        let eq196_e2464_d_n12: f64 = (eq196_e2462_d_n12 * p.p249);
        let eq196_e2464_d_n13: f64 = (eq196_e2462_d_n13 * p.p249);
        let eq196_e2464_d_n14: f64 = (eq196_e2462_d_n14 * p.p249);
        let eq196_e2464_d_n15: f64 = (eq196_e2462_d_n15 * p.p249);
        let eq196_e2464_d_n16: f64 = (eq196_e2462_d_n16 * p.p249);
        let eq196_e2464_d_n17: f64 = (eq196_e2462_d_n17 * p.p249);
        let eq196_e2464_d_n18: f64 = (eq196_e2462_d_n18 * p.p249);
        let eq196_e2464_d_n19: f64 = (eq196_e2462_d_n19 * p.p249);
        let eq196_e2464_d_n20: f64 = (eq196_e2462_d_n20 * p.p249);
        let eq196_e2464_d_n21: f64 = (eq196_e2462_d_n21 * p.p249);
        let eq196_e2464_d_n22: f64 = (eq196_e2462_d_n22 * p.p249);
        (eq196_e2464, eq196_e2464_d_n0, eq196_e2464_d_n1, eq196_e2464_d_n2, eq196_e2464_d_n3, eq196_e2464_d_n4, eq196_e2464_d_n5, eq196_e2464_d_n6, eq196_e2464_d_n7, eq196_e2464_d_n8, eq196_e2464_d_n9, eq196_e2464_d_n10, eq196_e2464_d_n11, eq196_e2464_d_n12, eq196_e2464_d_n13, eq196_e2464_d_n14, eq196_e2464_d_n15, eq196_e2464_d_n16, eq196_e2464_d_n17, eq196_e2464_d_n18, eq196_e2464_d_n19, eq196_e2464_d_n20, eq196_e2464_d_n21, eq196_e2464_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq196_value: f64 = eq196_e2466;
        let eq196_node_derivatives: [f64; 23] = [eq196_e2466_d_n0, eq196_e2466_d_n1, eq196_e2466_d_n2, eq196_e2466_d_n3, eq196_e2466_d_n4, eq196_e2466_d_n5, eq196_e2466_d_n6, eq196_e2466_d_n7, eq196_e2466_d_n8, eq196_e2466_d_n9, eq196_e2466_d_n10, eq196_e2466_d_n11, eq196_e2466_d_n12, eq196_e2466_d_n13, eq196_e2466_d_n14, eq196_e2466_d_n15, eq196_e2466_d_n16, eq196_e2466_d_n17, eq196_e2466_d_n18, eq196_e2466_d_n19, eq196_e2466_d_n20, eq196_e2466_d_n21, eq196_e2466_d_n22];
        let eq196_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[17]),
            self.multiplicity * (eq196_value),
            &nodes,
            &eq196_node_derivatives,
            &branches,
            &eq196_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_197_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq197_e2477, eq197_e2477_d_n0, eq197_e2477_d_n1, eq197_e2477_d_n2, eq197_e2477_d_n3, eq197_e2477_d_n4, eq197_e2477_d_n5, eq197_e2477_d_n6, eq197_e2477_d_n7, eq197_e2477_d_n8, eq197_e2477_d_n9, eq197_e2477_d_n10, eq197_e2477_d_n11, eq197_e2477_d_n12, eq197_e2477_d_n13, eq197_e2477_d_n14, eq197_e2477_d_n15, eq197_e2477_d_n16, eq197_e2477_d_n17, eq197_e2477_d_n18, eq197_e2477_d_n19, eq197_e2477_d_n20, eq197_e2477_d_n21, eq197_e2477_d_n22,) = {
    if ((s.v[600] != 0.0) && (s.v[601] != 0.0)) {
        let eq197_e2473: f64 = (p.p254 * s.v[300]);
        let eq197_e2473_d_n0: f64 = (p.p254 * s.dn[300][0]);
        let eq197_e2473_d_n1: f64 = (p.p254 * s.dn[300][1]);
        let eq197_e2473_d_n2: f64 = (p.p254 * s.dn[300][2]);
        let eq197_e2473_d_n3: f64 = (p.p254 * s.dn[300][3]);
        let eq197_e2473_d_n4: f64 = (p.p254 * s.dn[300][4]);
        let eq197_e2473_d_n5: f64 = (p.p254 * s.dn[300][5]);
        let eq197_e2473_d_n6: f64 = (p.p254 * s.dn[300][6]);
        let eq197_e2473_d_n7: f64 = (p.p254 * s.dn[300][7]);
        let eq197_e2473_d_n8: f64 = (p.p254 * s.dn[300][8]);
        let eq197_e2473_d_n9: f64 = (p.p254 * s.dn[300][9]);
        let eq197_e2473_d_n10: f64 = (p.p254 * s.dn[300][10]);
        let eq197_e2473_d_n11: f64 = (p.p254 * s.dn[300][11]);
        let eq197_e2473_d_n12: f64 = (p.p254 * s.dn[300][12]);
        let eq197_e2473_d_n13: f64 = (p.p254 * s.dn[300][13]);
        let eq197_e2473_d_n14: f64 = (p.p254 * s.dn[300][14]);
        let eq197_e2473_d_n15: f64 = (p.p254 * s.dn[300][15]);
        let eq197_e2473_d_n16: f64 = (p.p254 * s.dn[300][16]);
        let eq197_e2473_d_n17: f64 = (p.p254 * s.dn[300][17]);
        let eq197_e2473_d_n18: f64 = (p.p254 * s.dn[300][18]);
        let eq197_e2473_d_n19: f64 = (p.p254 * s.dn[300][19]);
        let eq197_e2473_d_n20: f64 = (p.p254 * s.dn[300][20]);
        let eq197_e2473_d_n21: f64 = (p.p254 * s.dn[300][21]);
        let eq197_e2473_d_n22: f64 = (p.p254 * s.dn[300][22]);
        let eq197_e2474: f64 = self.eval_ddt(96, eq197_e2473);
        let eq197_e2474_d_n0: f64 = self.ddt_jacobian(eq197_e2473_d_n0);
        let eq197_e2474_d_n1: f64 = self.ddt_jacobian(eq197_e2473_d_n1);
        let eq197_e2474_d_n2: f64 = self.ddt_jacobian(eq197_e2473_d_n2);
        let eq197_e2474_d_n3: f64 = self.ddt_jacobian(eq197_e2473_d_n3);
        let eq197_e2474_d_n4: f64 = self.ddt_jacobian(eq197_e2473_d_n4);
        let eq197_e2474_d_n5: f64 = self.ddt_jacobian(eq197_e2473_d_n5);
        let eq197_e2474_d_n6: f64 = self.ddt_jacobian(eq197_e2473_d_n6);
        let eq197_e2474_d_n7: f64 = self.ddt_jacobian(eq197_e2473_d_n7);
        let eq197_e2474_d_n8: f64 = self.ddt_jacobian(eq197_e2473_d_n8);
        let eq197_e2474_d_n9: f64 = self.ddt_jacobian(eq197_e2473_d_n9);
        let eq197_e2474_d_n10: f64 = self.ddt_jacobian(eq197_e2473_d_n10);
        let eq197_e2474_d_n11: f64 = self.ddt_jacobian(eq197_e2473_d_n11);
        let eq197_e2474_d_n12: f64 = self.ddt_jacobian(eq197_e2473_d_n12);
        let eq197_e2474_d_n13: f64 = self.ddt_jacobian(eq197_e2473_d_n13);
        let eq197_e2474_d_n14: f64 = self.ddt_jacobian(eq197_e2473_d_n14);
        let eq197_e2474_d_n15: f64 = self.ddt_jacobian(eq197_e2473_d_n15);
        let eq197_e2474_d_n16: f64 = self.ddt_jacobian(eq197_e2473_d_n16);
        let eq197_e2474_d_n17: f64 = self.ddt_jacobian(eq197_e2473_d_n17);
        let eq197_e2474_d_n18: f64 = self.ddt_jacobian(eq197_e2473_d_n18);
        let eq197_e2474_d_n19: f64 = self.ddt_jacobian(eq197_e2473_d_n19);
        let eq197_e2474_d_n20: f64 = self.ddt_jacobian(eq197_e2473_d_n20);
        let eq197_e2474_d_n21: f64 = self.ddt_jacobian(eq197_e2473_d_n21);
        let eq197_e2474_d_n22: f64 = self.ddt_jacobian(eq197_e2473_d_n22);
        let eq197_e2475: f64 = (p.p7 * eq197_e2474);
        let eq197_e2475_d_n0: f64 = (p.p7 * eq197_e2474_d_n0);
        let eq197_e2475_d_n1: f64 = (p.p7 * eq197_e2474_d_n1);
        let eq197_e2475_d_n2: f64 = (p.p7 * eq197_e2474_d_n2);
        let eq197_e2475_d_n3: f64 = (p.p7 * eq197_e2474_d_n3);
        let eq197_e2475_d_n4: f64 = (p.p7 * eq197_e2474_d_n4);
        let eq197_e2475_d_n5: f64 = (p.p7 * eq197_e2474_d_n5);
        let eq197_e2475_d_n6: f64 = (p.p7 * eq197_e2474_d_n6);
        let eq197_e2475_d_n7: f64 = (p.p7 * eq197_e2474_d_n7);
        let eq197_e2475_d_n8: f64 = (p.p7 * eq197_e2474_d_n8);
        let eq197_e2475_d_n9: f64 = (p.p7 * eq197_e2474_d_n9);
        let eq197_e2475_d_n10: f64 = (p.p7 * eq197_e2474_d_n10);
        let eq197_e2475_d_n11: f64 = (p.p7 * eq197_e2474_d_n11);
        let eq197_e2475_d_n12: f64 = (p.p7 * eq197_e2474_d_n12);
        let eq197_e2475_d_n13: f64 = (p.p7 * eq197_e2474_d_n13);
        let eq197_e2475_d_n14: f64 = (p.p7 * eq197_e2474_d_n14);
        let eq197_e2475_d_n15: f64 = (p.p7 * eq197_e2474_d_n15);
        let eq197_e2475_d_n16: f64 = (p.p7 * eq197_e2474_d_n16);
        let eq197_e2475_d_n17: f64 = (p.p7 * eq197_e2474_d_n17);
        let eq197_e2475_d_n18: f64 = (p.p7 * eq197_e2474_d_n18);
        let eq197_e2475_d_n19: f64 = (p.p7 * eq197_e2474_d_n19);
        let eq197_e2475_d_n20: f64 = (p.p7 * eq197_e2474_d_n20);
        let eq197_e2475_d_n21: f64 = (p.p7 * eq197_e2474_d_n21);
        let eq197_e2475_d_n22: f64 = (p.p7 * eq197_e2474_d_n22);
        (eq197_e2475, eq197_e2475_d_n0, eq197_e2475_d_n1, eq197_e2475_d_n2, eq197_e2475_d_n3, eq197_e2475_d_n4, eq197_e2475_d_n5, eq197_e2475_d_n6, eq197_e2475_d_n7, eq197_e2475_d_n8, eq197_e2475_d_n9, eq197_e2475_d_n10, eq197_e2475_d_n11, eq197_e2475_d_n12, eq197_e2475_d_n13, eq197_e2475_d_n14, eq197_e2475_d_n15, eq197_e2475_d_n16, eq197_e2475_d_n17, eq197_e2475_d_n18, eq197_e2475_d_n19, eq197_e2475_d_n20, eq197_e2475_d_n21, eq197_e2475_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq197_value: f64 = eq197_e2477;
        let eq197_node_derivatives: [f64; 23] = [eq197_e2477_d_n0, eq197_e2477_d_n1, eq197_e2477_d_n2, eq197_e2477_d_n3, eq197_e2477_d_n4, eq197_e2477_d_n5, eq197_e2477_d_n6, eq197_e2477_d_n7, eq197_e2477_d_n8, eq197_e2477_d_n9, eq197_e2477_d_n10, eq197_e2477_d_n11, eq197_e2477_d_n12, eq197_e2477_d_n13, eq197_e2477_d_n14, eq197_e2477_d_n15, eq197_e2477_d_n16, eq197_e2477_d_n17, eq197_e2477_d_n18, eq197_e2477_d_n19, eq197_e2477_d_n20, eq197_e2477_d_n21, eq197_e2477_d_n22];
        let eq197_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[17]),
            self.multiplicity * (eq197_value),
            &nodes,
            &eq197_node_derivatives,
            &branches,
            &eq197_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_198_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq198_e2487, eq198_e2487_d_n0, eq198_e2487_d_n1, eq198_e2487_d_n2, eq198_e2487_d_n3, eq198_e2487_d_n4, eq198_e2487_d_n5, eq198_e2487_d_n6, eq198_e2487_d_n7, eq198_e2487_d_n8, eq198_e2487_d_n9, eq198_e2487_d_n10, eq198_e2487_d_n11, eq198_e2487_d_n12, eq198_e2487_d_n13, eq198_e2487_d_n14, eq198_e2487_d_n15, eq198_e2487_d_n16, eq198_e2487_d_n17, eq198_e2487_d_n18, eq198_e2487_d_n19, eq198_e2487_d_n20, eq198_e2487_d_n21, eq198_e2487_d_n22,) = {
    if ((!(s.v[600] != 0.0)) && (s.v[603] != 0.0)) {
        let eq198_e2484: f64 = self.eval_ddt(97, s.v[301]);
        let eq198_e2484_d_n0: f64 = self.ddt_jacobian(s.dn[301][0]);
        let eq198_e2484_d_n1: f64 = self.ddt_jacobian(s.dn[301][1]);
        let eq198_e2484_d_n2: f64 = self.ddt_jacobian(s.dn[301][2]);
        let eq198_e2484_d_n3: f64 = self.ddt_jacobian(s.dn[301][3]);
        let eq198_e2484_d_n4: f64 = self.ddt_jacobian(s.dn[301][4]);
        let eq198_e2484_d_n5: f64 = self.ddt_jacobian(s.dn[301][5]);
        let eq198_e2484_d_n6: f64 = self.ddt_jacobian(s.dn[301][6]);
        let eq198_e2484_d_n7: f64 = self.ddt_jacobian(s.dn[301][7]);
        let eq198_e2484_d_n8: f64 = self.ddt_jacobian(s.dn[301][8]);
        let eq198_e2484_d_n9: f64 = self.ddt_jacobian(s.dn[301][9]);
        let eq198_e2484_d_n10: f64 = self.ddt_jacobian(s.dn[301][10]);
        let eq198_e2484_d_n11: f64 = self.ddt_jacobian(s.dn[301][11]);
        let eq198_e2484_d_n12: f64 = self.ddt_jacobian(s.dn[301][12]);
        let eq198_e2484_d_n13: f64 = self.ddt_jacobian(s.dn[301][13]);
        let eq198_e2484_d_n14: f64 = self.ddt_jacobian(s.dn[301][14]);
        let eq198_e2484_d_n15: f64 = self.ddt_jacobian(s.dn[301][15]);
        let eq198_e2484_d_n16: f64 = self.ddt_jacobian(s.dn[301][16]);
        let eq198_e2484_d_n17: f64 = self.ddt_jacobian(s.dn[301][17]);
        let eq198_e2484_d_n18: f64 = self.ddt_jacobian(s.dn[301][18]);
        let eq198_e2484_d_n19: f64 = self.ddt_jacobian(s.dn[301][19]);
        let eq198_e2484_d_n20: f64 = self.ddt_jacobian(s.dn[301][20]);
        let eq198_e2484_d_n21: f64 = self.ddt_jacobian(s.dn[301][21]);
        let eq198_e2484_d_n22: f64 = self.ddt_jacobian(s.dn[301][22]);
        let eq198_e2485: f64 = (p.p7 * eq198_e2484);
        let eq198_e2485_d_n0: f64 = (p.p7 * eq198_e2484_d_n0);
        let eq198_e2485_d_n1: f64 = (p.p7 * eq198_e2484_d_n1);
        let eq198_e2485_d_n2: f64 = (p.p7 * eq198_e2484_d_n2);
        let eq198_e2485_d_n3: f64 = (p.p7 * eq198_e2484_d_n3);
        let eq198_e2485_d_n4: f64 = (p.p7 * eq198_e2484_d_n4);
        let eq198_e2485_d_n5: f64 = (p.p7 * eq198_e2484_d_n5);
        let eq198_e2485_d_n6: f64 = (p.p7 * eq198_e2484_d_n6);
        let eq198_e2485_d_n7: f64 = (p.p7 * eq198_e2484_d_n7);
        let eq198_e2485_d_n8: f64 = (p.p7 * eq198_e2484_d_n8);
        let eq198_e2485_d_n9: f64 = (p.p7 * eq198_e2484_d_n9);
        let eq198_e2485_d_n10: f64 = (p.p7 * eq198_e2484_d_n10);
        let eq198_e2485_d_n11: f64 = (p.p7 * eq198_e2484_d_n11);
        let eq198_e2485_d_n12: f64 = (p.p7 * eq198_e2484_d_n12);
        let eq198_e2485_d_n13: f64 = (p.p7 * eq198_e2484_d_n13);
        let eq198_e2485_d_n14: f64 = (p.p7 * eq198_e2484_d_n14);
        let eq198_e2485_d_n15: f64 = (p.p7 * eq198_e2484_d_n15);
        let eq198_e2485_d_n16: f64 = (p.p7 * eq198_e2484_d_n16);
        let eq198_e2485_d_n17: f64 = (p.p7 * eq198_e2484_d_n17);
        let eq198_e2485_d_n18: f64 = (p.p7 * eq198_e2484_d_n18);
        let eq198_e2485_d_n19: f64 = (p.p7 * eq198_e2484_d_n19);
        let eq198_e2485_d_n20: f64 = (p.p7 * eq198_e2484_d_n20);
        let eq198_e2485_d_n21: f64 = (p.p7 * eq198_e2484_d_n21);
        let eq198_e2485_d_n22: f64 = (p.p7 * eq198_e2484_d_n22);
        (eq198_e2485, eq198_e2485_d_n0, eq198_e2485_d_n1, eq198_e2485_d_n2, eq198_e2485_d_n3, eq198_e2485_d_n4, eq198_e2485_d_n5, eq198_e2485_d_n6, eq198_e2485_d_n7, eq198_e2485_d_n8, eq198_e2485_d_n9, eq198_e2485_d_n10, eq198_e2485_d_n11, eq198_e2485_d_n12, eq198_e2485_d_n13, eq198_e2485_d_n14, eq198_e2485_d_n15, eq198_e2485_d_n16, eq198_e2485_d_n17, eq198_e2485_d_n18, eq198_e2485_d_n19, eq198_e2485_d_n20, eq198_e2485_d_n21, eq198_e2485_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq198_value: f64 = eq198_e2487;
        let eq198_node_derivatives: [f64; 23] = [eq198_e2487_d_n0, eq198_e2487_d_n1, eq198_e2487_d_n2, eq198_e2487_d_n3, eq198_e2487_d_n4, eq198_e2487_d_n5, eq198_e2487_d_n6, eq198_e2487_d_n7, eq198_e2487_d_n8, eq198_e2487_d_n9, eq198_e2487_d_n10, eq198_e2487_d_n11, eq198_e2487_d_n12, eq198_e2487_d_n13, eq198_e2487_d_n14, eq198_e2487_d_n15, eq198_e2487_d_n16, eq198_e2487_d_n17, eq198_e2487_d_n18, eq198_e2487_d_n19, eq198_e2487_d_n20, eq198_e2487_d_n21, eq198_e2487_d_n22];
        let eq198_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[7]),
            self.multiplicity * (eq198_value),
            &nodes,
            &eq198_node_derivatives,
            &branches,
            &eq198_branch_derivatives,
            self.multiplicity,
        );
    }
}
