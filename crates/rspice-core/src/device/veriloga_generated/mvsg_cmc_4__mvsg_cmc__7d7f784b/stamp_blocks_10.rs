#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_100_block_0(
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq100_e1324, eq100_e1324_d_n0, eq100_e1324_d_n1, eq100_e1324_d_n2, eq100_e1324_d_n3, eq100_e1324_d_n4, eq100_e1324_d_n5, eq100_e1324_d_n6, eq100_e1324_d_n7, eq100_e1324_d_n8, eq100_e1324_d_n9, eq100_e1324_d_n10, eq100_e1324_d_n11, eq100_e1324_d_n12, eq100_e1324_d_n13, eq100_e1324_d_n14, eq100_e1324_d_n15, eq100_e1324_d_n16, eq100_e1324_d_n17, eq100_e1324_d_n18, eq100_e1324_d_n19, eq100_e1324_d_n20, eq100_e1324_d_n21, eq100_e1324_d_n22, eq100_e1324_d_n23, eq100_e1324_d_n24, eq100_e1324_d_n25, eq100_e1324_d_n26, eq100_e1324_d_n27, eq100_e1324_d_n28, eq100_e1324_d_n29,) = {
    if (s.v[1201] != 0.0) {
        let eq100_e1317: f64 = self.eval_ddt(88, s.v[175]);
        let eq100_e1317_d_n0: f64 = self.ddt_jacobian(s.dn[175][0]);
        let eq100_e1317_d_n1: f64 = self.ddt_jacobian(s.dn[175][1]);
        let eq100_e1317_d_n2: f64 = self.ddt_jacobian(s.dn[175][2]);
        let eq100_e1317_d_n3: f64 = self.ddt_jacobian(s.dn[175][3]);
        let eq100_e1317_d_n4: f64 = self.ddt_jacobian(s.dn[175][4]);
        let eq100_e1317_d_n5: f64 = self.ddt_jacobian(s.dn[175][5]);
        let eq100_e1317_d_n6: f64 = self.ddt_jacobian(s.dn[175][6]);
        let eq100_e1317_d_n7: f64 = self.ddt_jacobian(s.dn[175][7]);
        let eq100_e1317_d_n8: f64 = self.ddt_jacobian(s.dn[175][8]);
        let eq100_e1317_d_n9: f64 = self.ddt_jacobian(s.dn[175][9]);
        let eq100_e1317_d_n10: f64 = self.ddt_jacobian(s.dn[175][10]);
        let eq100_e1317_d_n11: f64 = self.ddt_jacobian(s.dn[175][11]);
        let eq100_e1317_d_n12: f64 = self.ddt_jacobian(s.dn[175][12]);
        let eq100_e1317_d_n13: f64 = self.ddt_jacobian(s.dn[175][13]);
        let eq100_e1317_d_n14: f64 = self.ddt_jacobian(s.dn[175][14]);
        let eq100_e1317_d_n15: f64 = self.ddt_jacobian(s.dn[175][15]);
        let eq100_e1317_d_n16: f64 = self.ddt_jacobian(s.dn[175][16]);
        let eq100_e1317_d_n17: f64 = self.ddt_jacobian(s.dn[175][17]);
        let eq100_e1317_d_n18: f64 = self.ddt_jacobian(s.dn[175][18]);
        let eq100_e1317_d_n19: f64 = self.ddt_jacobian(s.dn[175][19]);
        let eq100_e1317_d_n20: f64 = self.ddt_jacobian(s.dn[175][20]);
        let eq100_e1317_d_n21: f64 = self.ddt_jacobian(s.dn[175][21]);
        let eq100_e1317_d_n22: f64 = self.ddt_jacobian(s.dn[175][22]);
        let eq100_e1317_d_n23: f64 = self.ddt_jacobian(s.dn[175][23]);
        let eq100_e1317_d_n24: f64 = self.ddt_jacobian(s.dn[175][24]);
        let eq100_e1317_d_n25: f64 = self.ddt_jacobian(s.dn[175][25]);
        let eq100_e1317_d_n26: f64 = self.ddt_jacobian(s.dn[175][26]);
        let eq100_e1317_d_n27: f64 = self.ddt_jacobian(s.dn[175][27]);
        let eq100_e1317_d_n28: f64 = self.ddt_jacobian(s.dn[175][28]);
        let eq100_e1317_d_n29: f64 = self.ddt_jacobian(s.dn[175][29]);
        let eq100_e1320: f64 = (p.p355 * (nv2 - nv11));
        let eq100_e1320_d_n2: f64 = p.p355;
        let eq100_e1320_d_n11: f64 = (-p.p355);
        let eq100_e1321: f64 = self.eval_ddt(89, eq100_e1320);
        let eq100_e1321_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n2: f64 = self.ddt_jacobian(eq100_e1320_d_n2);
        let eq100_e1321_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n11: f64 = self.ddt_jacobian(eq100_e1320_d_n11);
        let eq100_e1321_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq100_e1321_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq100_e1322: f64 = (eq100_e1317 + eq100_e1321);
        let eq100_e1322_d_n0: f64 = (eq100_e1317_d_n0 + eq100_e1321_d_n0);
        let eq100_e1322_d_n1: f64 = (eq100_e1317_d_n1 + eq100_e1321_d_n1);
        let eq100_e1322_d_n2: f64 = (eq100_e1317_d_n2 + eq100_e1321_d_n2);
        let eq100_e1322_d_n3: f64 = (eq100_e1317_d_n3 + eq100_e1321_d_n3);
        let eq100_e1322_d_n4: f64 = (eq100_e1317_d_n4 + eq100_e1321_d_n4);
        let eq100_e1322_d_n5: f64 = (eq100_e1317_d_n5 + eq100_e1321_d_n5);
        let eq100_e1322_d_n6: f64 = (eq100_e1317_d_n6 + eq100_e1321_d_n6);
        let eq100_e1322_d_n7: f64 = (eq100_e1317_d_n7 + eq100_e1321_d_n7);
        let eq100_e1322_d_n8: f64 = (eq100_e1317_d_n8 + eq100_e1321_d_n8);
        let eq100_e1322_d_n9: f64 = (eq100_e1317_d_n9 + eq100_e1321_d_n9);
        let eq100_e1322_d_n10: f64 = (eq100_e1317_d_n10 + eq100_e1321_d_n10);
        let eq100_e1322_d_n11: f64 = (eq100_e1317_d_n11 + eq100_e1321_d_n11);
        let eq100_e1322_d_n12: f64 = (eq100_e1317_d_n12 + eq100_e1321_d_n12);
        let eq100_e1322_d_n13: f64 = (eq100_e1317_d_n13 + eq100_e1321_d_n13);
        let eq100_e1322_d_n14: f64 = (eq100_e1317_d_n14 + eq100_e1321_d_n14);
        let eq100_e1322_d_n15: f64 = (eq100_e1317_d_n15 + eq100_e1321_d_n15);
        let eq100_e1322_d_n16: f64 = (eq100_e1317_d_n16 + eq100_e1321_d_n16);
        let eq100_e1322_d_n17: f64 = (eq100_e1317_d_n17 + eq100_e1321_d_n17);
        let eq100_e1322_d_n18: f64 = (eq100_e1317_d_n18 + eq100_e1321_d_n18);
        let eq100_e1322_d_n19: f64 = (eq100_e1317_d_n19 + eq100_e1321_d_n19);
        let eq100_e1322_d_n20: f64 = (eq100_e1317_d_n20 + eq100_e1321_d_n20);
        let eq100_e1322_d_n21: f64 = (eq100_e1317_d_n21 + eq100_e1321_d_n21);
        let eq100_e1322_d_n22: f64 = (eq100_e1317_d_n22 + eq100_e1321_d_n22);
        let eq100_e1322_d_n23: f64 = (eq100_e1317_d_n23 + eq100_e1321_d_n23);
        let eq100_e1322_d_n24: f64 = (eq100_e1317_d_n24 + eq100_e1321_d_n24);
        let eq100_e1322_d_n25: f64 = (eq100_e1317_d_n25 + eq100_e1321_d_n25);
        let eq100_e1322_d_n26: f64 = (eq100_e1317_d_n26 + eq100_e1321_d_n26);
        let eq100_e1322_d_n27: f64 = (eq100_e1317_d_n27 + eq100_e1321_d_n27);
        let eq100_e1322_d_n28: f64 = (eq100_e1317_d_n28 + eq100_e1321_d_n28);
        let eq100_e1322_d_n29: f64 = (eq100_e1317_d_n29 + eq100_e1321_d_n29);
        (eq100_e1322, eq100_e1322_d_n0, eq100_e1322_d_n1, eq100_e1322_d_n2, eq100_e1322_d_n3, eq100_e1322_d_n4, eq100_e1322_d_n5, eq100_e1322_d_n6, eq100_e1322_d_n7, eq100_e1322_d_n8, eq100_e1322_d_n9, eq100_e1322_d_n10, eq100_e1322_d_n11, eq100_e1322_d_n12, eq100_e1322_d_n13, eq100_e1322_d_n14, eq100_e1322_d_n15, eq100_e1322_d_n16, eq100_e1322_d_n17, eq100_e1322_d_n18, eq100_e1322_d_n19, eq100_e1322_d_n20, eq100_e1322_d_n21, eq100_e1322_d_n22, eq100_e1322_d_n23, eq100_e1322_d_n24, eq100_e1322_d_n25, eq100_e1322_d_n26, eq100_e1322_d_n27, eq100_e1322_d_n28, eq100_e1322_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq100_value: f64 = eq100_e1324;
        let eq100_node_derivatives: [f64; 30] = [eq100_e1324_d_n0, eq100_e1324_d_n1, eq100_e1324_d_n2, eq100_e1324_d_n3, eq100_e1324_d_n4, eq100_e1324_d_n5, eq100_e1324_d_n6, eq100_e1324_d_n7, eq100_e1324_d_n8, eq100_e1324_d_n9, eq100_e1324_d_n10, eq100_e1324_d_n11, eq100_e1324_d_n12, eq100_e1324_d_n13, eq100_e1324_d_n14, eq100_e1324_d_n15, eq100_e1324_d_n16, eq100_e1324_d_n17, eq100_e1324_d_n18, eq100_e1324_d_n19, eq100_e1324_d_n20, eq100_e1324_d_n21, eq100_e1324_d_n22, eq100_e1324_d_n23, eq100_e1324_d_n24, eq100_e1324_d_n25, eq100_e1324_d_n26, eq100_e1324_d_n27, eq100_e1324_d_n28, eq100_e1324_d_n29];
        let eq100_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[11]),
            self.multiplicity * (eq100_value),
            &nodes,
            &eq100_node_derivatives,
            &branches,
            &eq100_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_101_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq101_e1328,) = {
    if (s.v[1201] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq101_value: f64 = eq101_e1328;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[10]),
            self.multiplicity * (eq101_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_102_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq102_e1338, eq102_e1338_d_n0, eq102_e1338_d_n1, eq102_e1338_d_n2, eq102_e1338_d_n3, eq102_e1338_d_n4, eq102_e1338_d_n5, eq102_e1338_d_n6, eq102_e1338_d_n7, eq102_e1338_d_n8, eq102_e1338_d_n9, eq102_e1338_d_n10, eq102_e1338_d_n11, eq102_e1338_d_n12, eq102_e1338_d_n13, eq102_e1338_d_n14, eq102_e1338_d_n15, eq102_e1338_d_n16, eq102_e1338_d_n17, eq102_e1338_d_n18, eq102_e1338_d_n19, eq102_e1338_d_n20, eq102_e1338_d_n21, eq102_e1338_d_n22, eq102_e1338_d_n23, eq102_e1338_d_n24, eq102_e1338_d_n25, eq102_e1338_d_n26, eq102_e1338_d_n27, eq102_e1338_d_n28, eq102_e1338_d_n29,) = {
    if (s.v[1201] != 0.0) {
        let eq102_e1331: f64 = self.eval_ddt(90, s.v[177]);
        let eq102_e1331_d_n0: f64 = self.ddt_jacobian(s.dn[177][0]);
        let eq102_e1331_d_n1: f64 = self.ddt_jacobian(s.dn[177][1]);
        let eq102_e1331_d_n2: f64 = self.ddt_jacobian(s.dn[177][2]);
        let eq102_e1331_d_n3: f64 = self.ddt_jacobian(s.dn[177][3]);
        let eq102_e1331_d_n4: f64 = self.ddt_jacobian(s.dn[177][4]);
        let eq102_e1331_d_n5: f64 = self.ddt_jacobian(s.dn[177][5]);
        let eq102_e1331_d_n6: f64 = self.ddt_jacobian(s.dn[177][6]);
        let eq102_e1331_d_n7: f64 = self.ddt_jacobian(s.dn[177][7]);
        let eq102_e1331_d_n8: f64 = self.ddt_jacobian(s.dn[177][8]);
        let eq102_e1331_d_n9: f64 = self.ddt_jacobian(s.dn[177][9]);
        let eq102_e1331_d_n10: f64 = self.ddt_jacobian(s.dn[177][10]);
        let eq102_e1331_d_n11: f64 = self.ddt_jacobian(s.dn[177][11]);
        let eq102_e1331_d_n12: f64 = self.ddt_jacobian(s.dn[177][12]);
        let eq102_e1331_d_n13: f64 = self.ddt_jacobian(s.dn[177][13]);
        let eq102_e1331_d_n14: f64 = self.ddt_jacobian(s.dn[177][14]);
        let eq102_e1331_d_n15: f64 = self.ddt_jacobian(s.dn[177][15]);
        let eq102_e1331_d_n16: f64 = self.ddt_jacobian(s.dn[177][16]);
        let eq102_e1331_d_n17: f64 = self.ddt_jacobian(s.dn[177][17]);
        let eq102_e1331_d_n18: f64 = self.ddt_jacobian(s.dn[177][18]);
        let eq102_e1331_d_n19: f64 = self.ddt_jacobian(s.dn[177][19]);
        let eq102_e1331_d_n20: f64 = self.ddt_jacobian(s.dn[177][20]);
        let eq102_e1331_d_n21: f64 = self.ddt_jacobian(s.dn[177][21]);
        let eq102_e1331_d_n22: f64 = self.ddt_jacobian(s.dn[177][22]);
        let eq102_e1331_d_n23: f64 = self.ddt_jacobian(s.dn[177][23]);
        let eq102_e1331_d_n24: f64 = self.ddt_jacobian(s.dn[177][24]);
        let eq102_e1331_d_n25: f64 = self.ddt_jacobian(s.dn[177][25]);
        let eq102_e1331_d_n26: f64 = self.ddt_jacobian(s.dn[177][26]);
        let eq102_e1331_d_n27: f64 = self.ddt_jacobian(s.dn[177][27]);
        let eq102_e1331_d_n28: f64 = self.ddt_jacobian(s.dn[177][28]);
        let eq102_e1331_d_n29: f64 = self.ddt_jacobian(s.dn[177][29]);
        let eq102_e1334: f64 = (p.p355 * (nv7 - nv9));
        let eq102_e1334_d_n7: f64 = p.p355;
        let eq102_e1334_d_n9: f64 = (-p.p355);
        let eq102_e1335: f64 = self.eval_ddt(91, eq102_e1334);
        let eq102_e1335_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n7: f64 = self.ddt_jacobian(eq102_e1334_d_n7);
        let eq102_e1335_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n9: f64 = self.ddt_jacobian(eq102_e1334_d_n9);
        let eq102_e1335_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq102_e1335_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq102_e1336: f64 = (eq102_e1331 + eq102_e1335);
        let eq102_e1336_d_n0: f64 = (eq102_e1331_d_n0 + eq102_e1335_d_n0);
        let eq102_e1336_d_n1: f64 = (eq102_e1331_d_n1 + eq102_e1335_d_n1);
        let eq102_e1336_d_n2: f64 = (eq102_e1331_d_n2 + eq102_e1335_d_n2);
        let eq102_e1336_d_n3: f64 = (eq102_e1331_d_n3 + eq102_e1335_d_n3);
        let eq102_e1336_d_n4: f64 = (eq102_e1331_d_n4 + eq102_e1335_d_n4);
        let eq102_e1336_d_n5: f64 = (eq102_e1331_d_n5 + eq102_e1335_d_n5);
        let eq102_e1336_d_n6: f64 = (eq102_e1331_d_n6 + eq102_e1335_d_n6);
        let eq102_e1336_d_n7: f64 = (eq102_e1331_d_n7 + eq102_e1335_d_n7);
        let eq102_e1336_d_n8: f64 = (eq102_e1331_d_n8 + eq102_e1335_d_n8);
        let eq102_e1336_d_n9: f64 = (eq102_e1331_d_n9 + eq102_e1335_d_n9);
        let eq102_e1336_d_n10: f64 = (eq102_e1331_d_n10 + eq102_e1335_d_n10);
        let eq102_e1336_d_n11: f64 = (eq102_e1331_d_n11 + eq102_e1335_d_n11);
        let eq102_e1336_d_n12: f64 = (eq102_e1331_d_n12 + eq102_e1335_d_n12);
        let eq102_e1336_d_n13: f64 = (eq102_e1331_d_n13 + eq102_e1335_d_n13);
        let eq102_e1336_d_n14: f64 = (eq102_e1331_d_n14 + eq102_e1335_d_n14);
        let eq102_e1336_d_n15: f64 = (eq102_e1331_d_n15 + eq102_e1335_d_n15);
        let eq102_e1336_d_n16: f64 = (eq102_e1331_d_n16 + eq102_e1335_d_n16);
        let eq102_e1336_d_n17: f64 = (eq102_e1331_d_n17 + eq102_e1335_d_n17);
        let eq102_e1336_d_n18: f64 = (eq102_e1331_d_n18 + eq102_e1335_d_n18);
        let eq102_e1336_d_n19: f64 = (eq102_e1331_d_n19 + eq102_e1335_d_n19);
        let eq102_e1336_d_n20: f64 = (eq102_e1331_d_n20 + eq102_e1335_d_n20);
        let eq102_e1336_d_n21: f64 = (eq102_e1331_d_n21 + eq102_e1335_d_n21);
        let eq102_e1336_d_n22: f64 = (eq102_e1331_d_n22 + eq102_e1335_d_n22);
        let eq102_e1336_d_n23: f64 = (eq102_e1331_d_n23 + eq102_e1335_d_n23);
        let eq102_e1336_d_n24: f64 = (eq102_e1331_d_n24 + eq102_e1335_d_n24);
        let eq102_e1336_d_n25: f64 = (eq102_e1331_d_n25 + eq102_e1335_d_n25);
        let eq102_e1336_d_n26: f64 = (eq102_e1331_d_n26 + eq102_e1335_d_n26);
        let eq102_e1336_d_n27: f64 = (eq102_e1331_d_n27 + eq102_e1335_d_n27);
        let eq102_e1336_d_n28: f64 = (eq102_e1331_d_n28 + eq102_e1335_d_n28);
        let eq102_e1336_d_n29: f64 = (eq102_e1331_d_n29 + eq102_e1335_d_n29);
        (eq102_e1336, eq102_e1336_d_n0, eq102_e1336_d_n1, eq102_e1336_d_n2, eq102_e1336_d_n3, eq102_e1336_d_n4, eq102_e1336_d_n5, eq102_e1336_d_n6, eq102_e1336_d_n7, eq102_e1336_d_n8, eq102_e1336_d_n9, eq102_e1336_d_n10, eq102_e1336_d_n11, eq102_e1336_d_n12, eq102_e1336_d_n13, eq102_e1336_d_n14, eq102_e1336_d_n15, eq102_e1336_d_n16, eq102_e1336_d_n17, eq102_e1336_d_n18, eq102_e1336_d_n19, eq102_e1336_d_n20, eq102_e1336_d_n21, eq102_e1336_d_n22, eq102_e1336_d_n23, eq102_e1336_d_n24, eq102_e1336_d_n25, eq102_e1336_d_n26, eq102_e1336_d_n27, eq102_e1336_d_n28, eq102_e1336_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq102_value: f64 = eq102_e1338;
        let eq102_node_derivatives: [f64; 30] = [eq102_e1338_d_n0, eq102_e1338_d_n1, eq102_e1338_d_n2, eq102_e1338_d_n3, eq102_e1338_d_n4, eq102_e1338_d_n5, eq102_e1338_d_n6, eq102_e1338_d_n7, eq102_e1338_d_n8, eq102_e1338_d_n9, eq102_e1338_d_n10, eq102_e1338_d_n11, eq102_e1338_d_n12, eq102_e1338_d_n13, eq102_e1338_d_n14, eq102_e1338_d_n15, eq102_e1338_d_n16, eq102_e1338_d_n17, eq102_e1338_d_n18, eq102_e1338_d_n19, eq102_e1338_d_n20, eq102_e1338_d_n21, eq102_e1338_d_n22, eq102_e1338_d_n23, eq102_e1338_d_n24, eq102_e1338_d_n25, eq102_e1338_d_n26, eq102_e1338_d_n27, eq102_e1338_d_n28, eq102_e1338_d_n29];
        let eq102_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            self.multiplicity * (eq102_value),
            &nodes,
            &eq102_node_derivatives,
            &branches,
            &eq102_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_103_block_0(
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq103_e1349, eq103_e1349_d_n0, eq103_e1349_d_n1, eq103_e1349_d_n2, eq103_e1349_d_n3, eq103_e1349_d_n4, eq103_e1349_d_n5, eq103_e1349_d_n6, eq103_e1349_d_n7, eq103_e1349_d_n8, eq103_e1349_d_n9, eq103_e1349_d_n10, eq103_e1349_d_n11, eq103_e1349_d_n12, eq103_e1349_d_n13, eq103_e1349_d_n14, eq103_e1349_d_n15, eq103_e1349_d_n16, eq103_e1349_d_n17, eq103_e1349_d_n18, eq103_e1349_d_n19, eq103_e1349_d_n20, eq103_e1349_d_n21, eq103_e1349_d_n22, eq103_e1349_d_n23, eq103_e1349_d_n24, eq103_e1349_d_n25, eq103_e1349_d_n26, eq103_e1349_d_n27, eq103_e1349_d_n28, eq103_e1349_d_n29,) = {
    if (!(s.v[1201] != 0.0)) {
        let eq103_e1342: f64 = self.eval_ddt(92, s.v[173]);
        let eq103_e1342_d_n0: f64 = self.ddt_jacobian(s.dn[173][0]);
        let eq103_e1342_d_n1: f64 = self.ddt_jacobian(s.dn[173][1]);
        let eq103_e1342_d_n2: f64 = self.ddt_jacobian(s.dn[173][2]);
        let eq103_e1342_d_n3: f64 = self.ddt_jacobian(s.dn[173][3]);
        let eq103_e1342_d_n4: f64 = self.ddt_jacobian(s.dn[173][4]);
        let eq103_e1342_d_n5: f64 = self.ddt_jacobian(s.dn[173][5]);
        let eq103_e1342_d_n6: f64 = self.ddt_jacobian(s.dn[173][6]);
        let eq103_e1342_d_n7: f64 = self.ddt_jacobian(s.dn[173][7]);
        let eq103_e1342_d_n8: f64 = self.ddt_jacobian(s.dn[173][8]);
        let eq103_e1342_d_n9: f64 = self.ddt_jacobian(s.dn[173][9]);
        let eq103_e1342_d_n10: f64 = self.ddt_jacobian(s.dn[173][10]);
        let eq103_e1342_d_n11: f64 = self.ddt_jacobian(s.dn[173][11]);
        let eq103_e1342_d_n12: f64 = self.ddt_jacobian(s.dn[173][12]);
        let eq103_e1342_d_n13: f64 = self.ddt_jacobian(s.dn[173][13]);
        let eq103_e1342_d_n14: f64 = self.ddt_jacobian(s.dn[173][14]);
        let eq103_e1342_d_n15: f64 = self.ddt_jacobian(s.dn[173][15]);
        let eq103_e1342_d_n16: f64 = self.ddt_jacobian(s.dn[173][16]);
        let eq103_e1342_d_n17: f64 = self.ddt_jacobian(s.dn[173][17]);
        let eq103_e1342_d_n18: f64 = self.ddt_jacobian(s.dn[173][18]);
        let eq103_e1342_d_n19: f64 = self.ddt_jacobian(s.dn[173][19]);
        let eq103_e1342_d_n20: f64 = self.ddt_jacobian(s.dn[173][20]);
        let eq103_e1342_d_n21: f64 = self.ddt_jacobian(s.dn[173][21]);
        let eq103_e1342_d_n22: f64 = self.ddt_jacobian(s.dn[173][22]);
        let eq103_e1342_d_n23: f64 = self.ddt_jacobian(s.dn[173][23]);
        let eq103_e1342_d_n24: f64 = self.ddt_jacobian(s.dn[173][24]);
        let eq103_e1342_d_n25: f64 = self.ddt_jacobian(s.dn[173][25]);
        let eq103_e1342_d_n26: f64 = self.ddt_jacobian(s.dn[173][26]);
        let eq103_e1342_d_n27: f64 = self.ddt_jacobian(s.dn[173][27]);
        let eq103_e1342_d_n28: f64 = self.ddt_jacobian(s.dn[173][28]);
        let eq103_e1342_d_n29: f64 = self.ddt_jacobian(s.dn[173][29]);
        let eq103_e1345: f64 = (p.p355 * (nv2 - nv11));
        let eq103_e1345_d_n2: f64 = p.p355;
        let eq103_e1345_d_n11: f64 = (-p.p355);
        let eq103_e1346: f64 = self.eval_ddt(93, eq103_e1345);
        let eq103_e1346_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n2: f64 = self.ddt_jacobian(eq103_e1345_d_n2);
        let eq103_e1346_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n11: f64 = self.ddt_jacobian(eq103_e1345_d_n11);
        let eq103_e1346_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq103_e1346_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq103_e1347: f64 = (eq103_e1342 + eq103_e1346);
        let eq103_e1347_d_n0: f64 = (eq103_e1342_d_n0 + eq103_e1346_d_n0);
        let eq103_e1347_d_n1: f64 = (eq103_e1342_d_n1 + eq103_e1346_d_n1);
        let eq103_e1347_d_n2: f64 = (eq103_e1342_d_n2 + eq103_e1346_d_n2);
        let eq103_e1347_d_n3: f64 = (eq103_e1342_d_n3 + eq103_e1346_d_n3);
        let eq103_e1347_d_n4: f64 = (eq103_e1342_d_n4 + eq103_e1346_d_n4);
        let eq103_e1347_d_n5: f64 = (eq103_e1342_d_n5 + eq103_e1346_d_n5);
        let eq103_e1347_d_n6: f64 = (eq103_e1342_d_n6 + eq103_e1346_d_n6);
        let eq103_e1347_d_n7: f64 = (eq103_e1342_d_n7 + eq103_e1346_d_n7);
        let eq103_e1347_d_n8: f64 = (eq103_e1342_d_n8 + eq103_e1346_d_n8);
        let eq103_e1347_d_n9: f64 = (eq103_e1342_d_n9 + eq103_e1346_d_n9);
        let eq103_e1347_d_n10: f64 = (eq103_e1342_d_n10 + eq103_e1346_d_n10);
        let eq103_e1347_d_n11: f64 = (eq103_e1342_d_n11 + eq103_e1346_d_n11);
        let eq103_e1347_d_n12: f64 = (eq103_e1342_d_n12 + eq103_e1346_d_n12);
        let eq103_e1347_d_n13: f64 = (eq103_e1342_d_n13 + eq103_e1346_d_n13);
        let eq103_e1347_d_n14: f64 = (eq103_e1342_d_n14 + eq103_e1346_d_n14);
        let eq103_e1347_d_n15: f64 = (eq103_e1342_d_n15 + eq103_e1346_d_n15);
        let eq103_e1347_d_n16: f64 = (eq103_e1342_d_n16 + eq103_e1346_d_n16);
        let eq103_e1347_d_n17: f64 = (eq103_e1342_d_n17 + eq103_e1346_d_n17);
        let eq103_e1347_d_n18: f64 = (eq103_e1342_d_n18 + eq103_e1346_d_n18);
        let eq103_e1347_d_n19: f64 = (eq103_e1342_d_n19 + eq103_e1346_d_n19);
        let eq103_e1347_d_n20: f64 = (eq103_e1342_d_n20 + eq103_e1346_d_n20);
        let eq103_e1347_d_n21: f64 = (eq103_e1342_d_n21 + eq103_e1346_d_n21);
        let eq103_e1347_d_n22: f64 = (eq103_e1342_d_n22 + eq103_e1346_d_n22);
        let eq103_e1347_d_n23: f64 = (eq103_e1342_d_n23 + eq103_e1346_d_n23);
        let eq103_e1347_d_n24: f64 = (eq103_e1342_d_n24 + eq103_e1346_d_n24);
        let eq103_e1347_d_n25: f64 = (eq103_e1342_d_n25 + eq103_e1346_d_n25);
        let eq103_e1347_d_n26: f64 = (eq103_e1342_d_n26 + eq103_e1346_d_n26);
        let eq103_e1347_d_n27: f64 = (eq103_e1342_d_n27 + eq103_e1346_d_n27);
        let eq103_e1347_d_n28: f64 = (eq103_e1342_d_n28 + eq103_e1346_d_n28);
        let eq103_e1347_d_n29: f64 = (eq103_e1342_d_n29 + eq103_e1346_d_n29);
        (eq103_e1347, eq103_e1347_d_n0, eq103_e1347_d_n1, eq103_e1347_d_n2, eq103_e1347_d_n3, eq103_e1347_d_n4, eq103_e1347_d_n5, eq103_e1347_d_n6, eq103_e1347_d_n7, eq103_e1347_d_n8, eq103_e1347_d_n9, eq103_e1347_d_n10, eq103_e1347_d_n11, eq103_e1347_d_n12, eq103_e1347_d_n13, eq103_e1347_d_n14, eq103_e1347_d_n15, eq103_e1347_d_n16, eq103_e1347_d_n17, eq103_e1347_d_n18, eq103_e1347_d_n19, eq103_e1347_d_n20, eq103_e1347_d_n21, eq103_e1347_d_n22, eq103_e1347_d_n23, eq103_e1347_d_n24, eq103_e1347_d_n25, eq103_e1347_d_n26, eq103_e1347_d_n27, eq103_e1347_d_n28, eq103_e1347_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq103_value: f64 = eq103_e1349;
        let eq103_node_derivatives: [f64; 30] = [eq103_e1349_d_n0, eq103_e1349_d_n1, eq103_e1349_d_n2, eq103_e1349_d_n3, eq103_e1349_d_n4, eq103_e1349_d_n5, eq103_e1349_d_n6, eq103_e1349_d_n7, eq103_e1349_d_n8, eq103_e1349_d_n9, eq103_e1349_d_n10, eq103_e1349_d_n11, eq103_e1349_d_n12, eq103_e1349_d_n13, eq103_e1349_d_n14, eq103_e1349_d_n15, eq103_e1349_d_n16, eq103_e1349_d_n17, eq103_e1349_d_n18, eq103_e1349_d_n19, eq103_e1349_d_n20, eq103_e1349_d_n21, eq103_e1349_d_n22, eq103_e1349_d_n23, eq103_e1349_d_n24, eq103_e1349_d_n25, eq103_e1349_d_n26, eq103_e1349_d_n27, eq103_e1349_d_n28, eq103_e1349_d_n29];
        let eq103_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[11]),
            self.multiplicity * (eq103_value),
            &nodes,
            &eq103_node_derivatives,
            &branches,
            &eq103_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_104_block_0(
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq104_e1360, eq104_e1360_d_n0, eq104_e1360_d_n1, eq104_e1360_d_n2, eq104_e1360_d_n3, eq104_e1360_d_n4, eq104_e1360_d_n5, eq104_e1360_d_n6, eq104_e1360_d_n7, eq104_e1360_d_n8, eq104_e1360_d_n9, eq104_e1360_d_n10, eq104_e1360_d_n11, eq104_e1360_d_n12, eq104_e1360_d_n13, eq104_e1360_d_n14, eq104_e1360_d_n15, eq104_e1360_d_n16, eq104_e1360_d_n17, eq104_e1360_d_n18, eq104_e1360_d_n19, eq104_e1360_d_n20, eq104_e1360_d_n21, eq104_e1360_d_n22, eq104_e1360_d_n23, eq104_e1360_d_n24, eq104_e1360_d_n25, eq104_e1360_d_n26, eq104_e1360_d_n27, eq104_e1360_d_n28, eq104_e1360_d_n29,) = {
    if (!(s.v[1201] != 0.0)) {
        let eq104_e1353: f64 = self.eval_ddt(94, s.v[174]);
        let eq104_e1353_d_n0: f64 = self.ddt_jacobian(s.dn[174][0]);
        let eq104_e1353_d_n1: f64 = self.ddt_jacobian(s.dn[174][1]);
        let eq104_e1353_d_n2: f64 = self.ddt_jacobian(s.dn[174][2]);
        let eq104_e1353_d_n3: f64 = self.ddt_jacobian(s.dn[174][3]);
        let eq104_e1353_d_n4: f64 = self.ddt_jacobian(s.dn[174][4]);
        let eq104_e1353_d_n5: f64 = self.ddt_jacobian(s.dn[174][5]);
        let eq104_e1353_d_n6: f64 = self.ddt_jacobian(s.dn[174][6]);
        let eq104_e1353_d_n7: f64 = self.ddt_jacobian(s.dn[174][7]);
        let eq104_e1353_d_n8: f64 = self.ddt_jacobian(s.dn[174][8]);
        let eq104_e1353_d_n9: f64 = self.ddt_jacobian(s.dn[174][9]);
        let eq104_e1353_d_n10: f64 = self.ddt_jacobian(s.dn[174][10]);
        let eq104_e1353_d_n11: f64 = self.ddt_jacobian(s.dn[174][11]);
        let eq104_e1353_d_n12: f64 = self.ddt_jacobian(s.dn[174][12]);
        let eq104_e1353_d_n13: f64 = self.ddt_jacobian(s.dn[174][13]);
        let eq104_e1353_d_n14: f64 = self.ddt_jacobian(s.dn[174][14]);
        let eq104_e1353_d_n15: f64 = self.ddt_jacobian(s.dn[174][15]);
        let eq104_e1353_d_n16: f64 = self.ddt_jacobian(s.dn[174][16]);
        let eq104_e1353_d_n17: f64 = self.ddt_jacobian(s.dn[174][17]);
        let eq104_e1353_d_n18: f64 = self.ddt_jacobian(s.dn[174][18]);
        let eq104_e1353_d_n19: f64 = self.ddt_jacobian(s.dn[174][19]);
        let eq104_e1353_d_n20: f64 = self.ddt_jacobian(s.dn[174][20]);
        let eq104_e1353_d_n21: f64 = self.ddt_jacobian(s.dn[174][21]);
        let eq104_e1353_d_n22: f64 = self.ddt_jacobian(s.dn[174][22]);
        let eq104_e1353_d_n23: f64 = self.ddt_jacobian(s.dn[174][23]);
        let eq104_e1353_d_n24: f64 = self.ddt_jacobian(s.dn[174][24]);
        let eq104_e1353_d_n25: f64 = self.ddt_jacobian(s.dn[174][25]);
        let eq104_e1353_d_n26: f64 = self.ddt_jacobian(s.dn[174][26]);
        let eq104_e1353_d_n27: f64 = self.ddt_jacobian(s.dn[174][27]);
        let eq104_e1353_d_n28: f64 = self.ddt_jacobian(s.dn[174][28]);
        let eq104_e1353_d_n29: f64 = self.ddt_jacobian(s.dn[174][29]);
        let eq104_e1356: f64 = (p.p355 * (nv2 - nv10));
        let eq104_e1356_d_n2: f64 = p.p355;
        let eq104_e1356_d_n10: f64 = (-p.p355);
        let eq104_e1357: f64 = self.eval_ddt(95, eq104_e1356);
        let eq104_e1357_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n2: f64 = self.ddt_jacobian(eq104_e1356_d_n2);
        let eq104_e1357_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n10: f64 = self.ddt_jacobian(eq104_e1356_d_n10);
        let eq104_e1357_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq104_e1357_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq104_e1358: f64 = (eq104_e1353 + eq104_e1357);
        let eq104_e1358_d_n0: f64 = (eq104_e1353_d_n0 + eq104_e1357_d_n0);
        let eq104_e1358_d_n1: f64 = (eq104_e1353_d_n1 + eq104_e1357_d_n1);
        let eq104_e1358_d_n2: f64 = (eq104_e1353_d_n2 + eq104_e1357_d_n2);
        let eq104_e1358_d_n3: f64 = (eq104_e1353_d_n3 + eq104_e1357_d_n3);
        let eq104_e1358_d_n4: f64 = (eq104_e1353_d_n4 + eq104_e1357_d_n4);
        let eq104_e1358_d_n5: f64 = (eq104_e1353_d_n5 + eq104_e1357_d_n5);
        let eq104_e1358_d_n6: f64 = (eq104_e1353_d_n6 + eq104_e1357_d_n6);
        let eq104_e1358_d_n7: f64 = (eq104_e1353_d_n7 + eq104_e1357_d_n7);
        let eq104_e1358_d_n8: f64 = (eq104_e1353_d_n8 + eq104_e1357_d_n8);
        let eq104_e1358_d_n9: f64 = (eq104_e1353_d_n9 + eq104_e1357_d_n9);
        let eq104_e1358_d_n10: f64 = (eq104_e1353_d_n10 + eq104_e1357_d_n10);
        let eq104_e1358_d_n11: f64 = (eq104_e1353_d_n11 + eq104_e1357_d_n11);
        let eq104_e1358_d_n12: f64 = (eq104_e1353_d_n12 + eq104_e1357_d_n12);
        let eq104_e1358_d_n13: f64 = (eq104_e1353_d_n13 + eq104_e1357_d_n13);
        let eq104_e1358_d_n14: f64 = (eq104_e1353_d_n14 + eq104_e1357_d_n14);
        let eq104_e1358_d_n15: f64 = (eq104_e1353_d_n15 + eq104_e1357_d_n15);
        let eq104_e1358_d_n16: f64 = (eq104_e1353_d_n16 + eq104_e1357_d_n16);
        let eq104_e1358_d_n17: f64 = (eq104_e1353_d_n17 + eq104_e1357_d_n17);
        let eq104_e1358_d_n18: f64 = (eq104_e1353_d_n18 + eq104_e1357_d_n18);
        let eq104_e1358_d_n19: f64 = (eq104_e1353_d_n19 + eq104_e1357_d_n19);
        let eq104_e1358_d_n20: f64 = (eq104_e1353_d_n20 + eq104_e1357_d_n20);
        let eq104_e1358_d_n21: f64 = (eq104_e1353_d_n21 + eq104_e1357_d_n21);
        let eq104_e1358_d_n22: f64 = (eq104_e1353_d_n22 + eq104_e1357_d_n22);
        let eq104_e1358_d_n23: f64 = (eq104_e1353_d_n23 + eq104_e1357_d_n23);
        let eq104_e1358_d_n24: f64 = (eq104_e1353_d_n24 + eq104_e1357_d_n24);
        let eq104_e1358_d_n25: f64 = (eq104_e1353_d_n25 + eq104_e1357_d_n25);
        let eq104_e1358_d_n26: f64 = (eq104_e1353_d_n26 + eq104_e1357_d_n26);
        let eq104_e1358_d_n27: f64 = (eq104_e1353_d_n27 + eq104_e1357_d_n27);
        let eq104_e1358_d_n28: f64 = (eq104_e1353_d_n28 + eq104_e1357_d_n28);
        let eq104_e1358_d_n29: f64 = (eq104_e1353_d_n29 + eq104_e1357_d_n29);
        (eq104_e1358, eq104_e1358_d_n0, eq104_e1358_d_n1, eq104_e1358_d_n2, eq104_e1358_d_n3, eq104_e1358_d_n4, eq104_e1358_d_n5, eq104_e1358_d_n6, eq104_e1358_d_n7, eq104_e1358_d_n8, eq104_e1358_d_n9, eq104_e1358_d_n10, eq104_e1358_d_n11, eq104_e1358_d_n12, eq104_e1358_d_n13, eq104_e1358_d_n14, eq104_e1358_d_n15, eq104_e1358_d_n16, eq104_e1358_d_n17, eq104_e1358_d_n18, eq104_e1358_d_n19, eq104_e1358_d_n20, eq104_e1358_d_n21, eq104_e1358_d_n22, eq104_e1358_d_n23, eq104_e1358_d_n24, eq104_e1358_d_n25, eq104_e1358_d_n26, eq104_e1358_d_n27, eq104_e1358_d_n28, eq104_e1358_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq104_value: f64 = eq104_e1360;
        let eq104_node_derivatives: [f64; 30] = [eq104_e1360_d_n0, eq104_e1360_d_n1, eq104_e1360_d_n2, eq104_e1360_d_n3, eq104_e1360_d_n4, eq104_e1360_d_n5, eq104_e1360_d_n6, eq104_e1360_d_n7, eq104_e1360_d_n8, eq104_e1360_d_n9, eq104_e1360_d_n10, eq104_e1360_d_n11, eq104_e1360_d_n12, eq104_e1360_d_n13, eq104_e1360_d_n14, eq104_e1360_d_n15, eq104_e1360_d_n16, eq104_e1360_d_n17, eq104_e1360_d_n18, eq104_e1360_d_n19, eq104_e1360_d_n20, eq104_e1360_d_n21, eq104_e1360_d_n22, eq104_e1360_d_n23, eq104_e1360_d_n24, eq104_e1360_d_n25, eq104_e1360_d_n26, eq104_e1360_d_n27, eq104_e1360_d_n28, eq104_e1360_d_n29];
        let eq104_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[10]),
            self.multiplicity * (eq104_value),
            &nodes,
            &eq104_node_derivatives,
            &branches,
            &eq104_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_105_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq105_e1371, eq105_e1371_d_n0, eq105_e1371_d_n1, eq105_e1371_d_n2, eq105_e1371_d_n3, eq105_e1371_d_n4, eq105_e1371_d_n5, eq105_e1371_d_n6, eq105_e1371_d_n7, eq105_e1371_d_n8, eq105_e1371_d_n9, eq105_e1371_d_n10, eq105_e1371_d_n11, eq105_e1371_d_n12, eq105_e1371_d_n13, eq105_e1371_d_n14, eq105_e1371_d_n15, eq105_e1371_d_n16, eq105_e1371_d_n17, eq105_e1371_d_n18, eq105_e1371_d_n19, eq105_e1371_d_n20, eq105_e1371_d_n21, eq105_e1371_d_n22, eq105_e1371_d_n23, eq105_e1371_d_n24, eq105_e1371_d_n25, eq105_e1371_d_n26, eq105_e1371_d_n27, eq105_e1371_d_n28, eq105_e1371_d_n29,) = {
    if (!(s.v[1201] != 0.0)) {
        let eq105_e1364: f64 = self.eval_ddt(96, s.v[175]);
        let eq105_e1364_d_n0: f64 = self.ddt_jacobian(s.dn[175][0]);
        let eq105_e1364_d_n1: f64 = self.ddt_jacobian(s.dn[175][1]);
        let eq105_e1364_d_n2: f64 = self.ddt_jacobian(s.dn[175][2]);
        let eq105_e1364_d_n3: f64 = self.ddt_jacobian(s.dn[175][3]);
        let eq105_e1364_d_n4: f64 = self.ddt_jacobian(s.dn[175][4]);
        let eq105_e1364_d_n5: f64 = self.ddt_jacobian(s.dn[175][5]);
        let eq105_e1364_d_n6: f64 = self.ddt_jacobian(s.dn[175][6]);
        let eq105_e1364_d_n7: f64 = self.ddt_jacobian(s.dn[175][7]);
        let eq105_e1364_d_n8: f64 = self.ddt_jacobian(s.dn[175][8]);
        let eq105_e1364_d_n9: f64 = self.ddt_jacobian(s.dn[175][9]);
        let eq105_e1364_d_n10: f64 = self.ddt_jacobian(s.dn[175][10]);
        let eq105_e1364_d_n11: f64 = self.ddt_jacobian(s.dn[175][11]);
        let eq105_e1364_d_n12: f64 = self.ddt_jacobian(s.dn[175][12]);
        let eq105_e1364_d_n13: f64 = self.ddt_jacobian(s.dn[175][13]);
        let eq105_e1364_d_n14: f64 = self.ddt_jacobian(s.dn[175][14]);
        let eq105_e1364_d_n15: f64 = self.ddt_jacobian(s.dn[175][15]);
        let eq105_e1364_d_n16: f64 = self.ddt_jacobian(s.dn[175][16]);
        let eq105_e1364_d_n17: f64 = self.ddt_jacobian(s.dn[175][17]);
        let eq105_e1364_d_n18: f64 = self.ddt_jacobian(s.dn[175][18]);
        let eq105_e1364_d_n19: f64 = self.ddt_jacobian(s.dn[175][19]);
        let eq105_e1364_d_n20: f64 = self.ddt_jacobian(s.dn[175][20]);
        let eq105_e1364_d_n21: f64 = self.ddt_jacobian(s.dn[175][21]);
        let eq105_e1364_d_n22: f64 = self.ddt_jacobian(s.dn[175][22]);
        let eq105_e1364_d_n23: f64 = self.ddt_jacobian(s.dn[175][23]);
        let eq105_e1364_d_n24: f64 = self.ddt_jacobian(s.dn[175][24]);
        let eq105_e1364_d_n25: f64 = self.ddt_jacobian(s.dn[175][25]);
        let eq105_e1364_d_n26: f64 = self.ddt_jacobian(s.dn[175][26]);
        let eq105_e1364_d_n27: f64 = self.ddt_jacobian(s.dn[175][27]);
        let eq105_e1364_d_n28: f64 = self.ddt_jacobian(s.dn[175][28]);
        let eq105_e1364_d_n29: f64 = self.ddt_jacobian(s.dn[175][29]);
        let eq105_e1367: f64 = (p.p355 * (nv7 - nv11));
        let eq105_e1367_d_n7: f64 = p.p355;
        let eq105_e1367_d_n11: f64 = (-p.p355);
        let eq105_e1368: f64 = self.eval_ddt(97, eq105_e1367);
        let eq105_e1368_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n7: f64 = self.ddt_jacobian(eq105_e1367_d_n7);
        let eq105_e1368_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n11: f64 = self.ddt_jacobian(eq105_e1367_d_n11);
        let eq105_e1368_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq105_e1368_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq105_e1369: f64 = (eq105_e1364 + eq105_e1368);
        let eq105_e1369_d_n0: f64 = (eq105_e1364_d_n0 + eq105_e1368_d_n0);
        let eq105_e1369_d_n1: f64 = (eq105_e1364_d_n1 + eq105_e1368_d_n1);
        let eq105_e1369_d_n2: f64 = (eq105_e1364_d_n2 + eq105_e1368_d_n2);
        let eq105_e1369_d_n3: f64 = (eq105_e1364_d_n3 + eq105_e1368_d_n3);
        let eq105_e1369_d_n4: f64 = (eq105_e1364_d_n4 + eq105_e1368_d_n4);
        let eq105_e1369_d_n5: f64 = (eq105_e1364_d_n5 + eq105_e1368_d_n5);
        let eq105_e1369_d_n6: f64 = (eq105_e1364_d_n6 + eq105_e1368_d_n6);
        let eq105_e1369_d_n7: f64 = (eq105_e1364_d_n7 + eq105_e1368_d_n7);
        let eq105_e1369_d_n8: f64 = (eq105_e1364_d_n8 + eq105_e1368_d_n8);
        let eq105_e1369_d_n9: f64 = (eq105_e1364_d_n9 + eq105_e1368_d_n9);
        let eq105_e1369_d_n10: f64 = (eq105_e1364_d_n10 + eq105_e1368_d_n10);
        let eq105_e1369_d_n11: f64 = (eq105_e1364_d_n11 + eq105_e1368_d_n11);
        let eq105_e1369_d_n12: f64 = (eq105_e1364_d_n12 + eq105_e1368_d_n12);
        let eq105_e1369_d_n13: f64 = (eq105_e1364_d_n13 + eq105_e1368_d_n13);
        let eq105_e1369_d_n14: f64 = (eq105_e1364_d_n14 + eq105_e1368_d_n14);
        let eq105_e1369_d_n15: f64 = (eq105_e1364_d_n15 + eq105_e1368_d_n15);
        let eq105_e1369_d_n16: f64 = (eq105_e1364_d_n16 + eq105_e1368_d_n16);
        let eq105_e1369_d_n17: f64 = (eq105_e1364_d_n17 + eq105_e1368_d_n17);
        let eq105_e1369_d_n18: f64 = (eq105_e1364_d_n18 + eq105_e1368_d_n18);
        let eq105_e1369_d_n19: f64 = (eq105_e1364_d_n19 + eq105_e1368_d_n19);
        let eq105_e1369_d_n20: f64 = (eq105_e1364_d_n20 + eq105_e1368_d_n20);
        let eq105_e1369_d_n21: f64 = (eq105_e1364_d_n21 + eq105_e1368_d_n21);
        let eq105_e1369_d_n22: f64 = (eq105_e1364_d_n22 + eq105_e1368_d_n22);
        let eq105_e1369_d_n23: f64 = (eq105_e1364_d_n23 + eq105_e1368_d_n23);
        let eq105_e1369_d_n24: f64 = (eq105_e1364_d_n24 + eq105_e1368_d_n24);
        let eq105_e1369_d_n25: f64 = (eq105_e1364_d_n25 + eq105_e1368_d_n25);
        let eq105_e1369_d_n26: f64 = (eq105_e1364_d_n26 + eq105_e1368_d_n26);
        let eq105_e1369_d_n27: f64 = (eq105_e1364_d_n27 + eq105_e1368_d_n27);
        let eq105_e1369_d_n28: f64 = (eq105_e1364_d_n28 + eq105_e1368_d_n28);
        let eq105_e1369_d_n29: f64 = (eq105_e1364_d_n29 + eq105_e1368_d_n29);
        (eq105_e1369, eq105_e1369_d_n0, eq105_e1369_d_n1, eq105_e1369_d_n2, eq105_e1369_d_n3, eq105_e1369_d_n4, eq105_e1369_d_n5, eq105_e1369_d_n6, eq105_e1369_d_n7, eq105_e1369_d_n8, eq105_e1369_d_n9, eq105_e1369_d_n10, eq105_e1369_d_n11, eq105_e1369_d_n12, eq105_e1369_d_n13, eq105_e1369_d_n14, eq105_e1369_d_n15, eq105_e1369_d_n16, eq105_e1369_d_n17, eq105_e1369_d_n18, eq105_e1369_d_n19, eq105_e1369_d_n20, eq105_e1369_d_n21, eq105_e1369_d_n22, eq105_e1369_d_n23, eq105_e1369_d_n24, eq105_e1369_d_n25, eq105_e1369_d_n26, eq105_e1369_d_n27, eq105_e1369_d_n28, eq105_e1369_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq105_value: f64 = eq105_e1371;
        let eq105_node_derivatives: [f64; 30] = [eq105_e1371_d_n0, eq105_e1371_d_n1, eq105_e1371_d_n2, eq105_e1371_d_n3, eq105_e1371_d_n4, eq105_e1371_d_n5, eq105_e1371_d_n6, eq105_e1371_d_n7, eq105_e1371_d_n8, eq105_e1371_d_n9, eq105_e1371_d_n10, eq105_e1371_d_n11, eq105_e1371_d_n12, eq105_e1371_d_n13, eq105_e1371_d_n14, eq105_e1371_d_n15, eq105_e1371_d_n16, eq105_e1371_d_n17, eq105_e1371_d_n18, eq105_e1371_d_n19, eq105_e1371_d_n20, eq105_e1371_d_n21, eq105_e1371_d_n22, eq105_e1371_d_n23, eq105_e1371_d_n24, eq105_e1371_d_n25, eq105_e1371_d_n26, eq105_e1371_d_n27, eq105_e1371_d_n28, eq105_e1371_d_n29];
        let eq105_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            self.multiplicity * (eq105_value),
            &nodes,
            &eq105_node_derivatives,
            &branches,
            &eq105_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_106_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq106_e1376,) = {
    if (!(s.v[1201] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq106_value: f64 = eq106_e1376;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[10]),
            self.multiplicity * (eq106_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_107_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq107_e1381,) = {
    if (!(s.v[1201] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq107_value: f64 = eq107_e1381;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[9]),
            self.multiplicity * (eq107_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_108_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let eq108_e1383: f64 = self.eval_ddt(98, s.v[176]);
        let eq108_e1383_d_n0: f64 = self.ddt_jacobian(s.dn[176][0]);
        let eq108_e1383_d_n1: f64 = self.ddt_jacobian(s.dn[176][1]);
        let eq108_e1383_d_n2: f64 = self.ddt_jacobian(s.dn[176][2]);
        let eq108_e1383_d_n3: f64 = self.ddt_jacobian(s.dn[176][3]);
        let eq108_e1383_d_n4: f64 = self.ddt_jacobian(s.dn[176][4]);
        let eq108_e1383_d_n5: f64 = self.ddt_jacobian(s.dn[176][5]);
        let eq108_e1383_d_n6: f64 = self.ddt_jacobian(s.dn[176][6]);
        let eq108_e1383_d_n7: f64 = self.ddt_jacobian(s.dn[176][7]);
        let eq108_e1383_d_n8: f64 = self.ddt_jacobian(s.dn[176][8]);
        let eq108_e1383_d_n9: f64 = self.ddt_jacobian(s.dn[176][9]);
        let eq108_e1383_d_n10: f64 = self.ddt_jacobian(s.dn[176][10]);
        let eq108_e1383_d_n11: f64 = self.ddt_jacobian(s.dn[176][11]);
        let eq108_e1383_d_n12: f64 = self.ddt_jacobian(s.dn[176][12]);
        let eq108_e1383_d_n13: f64 = self.ddt_jacobian(s.dn[176][13]);
        let eq108_e1383_d_n14: f64 = self.ddt_jacobian(s.dn[176][14]);
        let eq108_e1383_d_n15: f64 = self.ddt_jacobian(s.dn[176][15]);
        let eq108_e1383_d_n16: f64 = self.ddt_jacobian(s.dn[176][16]);
        let eq108_e1383_d_n17: f64 = self.ddt_jacobian(s.dn[176][17]);
        let eq108_e1383_d_n18: f64 = self.ddt_jacobian(s.dn[176][18]);
        let eq108_e1383_d_n19: f64 = self.ddt_jacobian(s.dn[176][19]);
        let eq108_e1383_d_n20: f64 = self.ddt_jacobian(s.dn[176][20]);
        let eq108_e1383_d_n21: f64 = self.ddt_jacobian(s.dn[176][21]);
        let eq108_e1383_d_n22: f64 = self.ddt_jacobian(s.dn[176][22]);
        let eq108_e1383_d_n23: f64 = self.ddt_jacobian(s.dn[176][23]);
        let eq108_e1383_d_n24: f64 = self.ddt_jacobian(s.dn[176][24]);
        let eq108_e1383_d_n25: f64 = self.ddt_jacobian(s.dn[176][25]);
        let eq108_e1383_d_n26: f64 = self.ddt_jacobian(s.dn[176][26]);
        let eq108_e1383_d_n27: f64 = self.ddt_jacobian(s.dn[176][27]);
        let eq108_e1383_d_n28: f64 = self.ddt_jacobian(s.dn[176][28]);
        let eq108_e1383_d_n29: f64 = self.ddt_jacobian(s.dn[176][29]);
        let eq108_e1386: f64 = (p.p355 * (nv3 - nv11));
        let eq108_e1386_d_n3: f64 = p.p355;
        let eq108_e1386_d_n11: f64 = (-p.p355);
        let eq108_e1387: f64 = self.eval_ddt(99, eq108_e1386);
        let eq108_e1387_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n3: f64 = self.ddt_jacobian(eq108_e1386_d_n3);
        let eq108_e1387_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n11: f64 = self.ddt_jacobian(eq108_e1386_d_n11);
        let eq108_e1387_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq108_e1387_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq108_e1388: f64 = (eq108_e1383 + eq108_e1387);
        let eq108_e1388_d_n0: f64 = (eq108_e1383_d_n0 + eq108_e1387_d_n0);
        let eq108_e1388_d_n1: f64 = (eq108_e1383_d_n1 + eq108_e1387_d_n1);
        let eq108_e1388_d_n2: f64 = (eq108_e1383_d_n2 + eq108_e1387_d_n2);
        let eq108_e1388_d_n3: f64 = (eq108_e1383_d_n3 + eq108_e1387_d_n3);
        let eq108_e1388_d_n4: f64 = (eq108_e1383_d_n4 + eq108_e1387_d_n4);
        let eq108_e1388_d_n5: f64 = (eq108_e1383_d_n5 + eq108_e1387_d_n5);
        let eq108_e1388_d_n6: f64 = (eq108_e1383_d_n6 + eq108_e1387_d_n6);
        let eq108_e1388_d_n7: f64 = (eq108_e1383_d_n7 + eq108_e1387_d_n7);
        let eq108_e1388_d_n8: f64 = (eq108_e1383_d_n8 + eq108_e1387_d_n8);
        let eq108_e1388_d_n9: f64 = (eq108_e1383_d_n9 + eq108_e1387_d_n9);
        let eq108_e1388_d_n10: f64 = (eq108_e1383_d_n10 + eq108_e1387_d_n10);
        let eq108_e1388_d_n11: f64 = (eq108_e1383_d_n11 + eq108_e1387_d_n11);
        let eq108_e1388_d_n12: f64 = (eq108_e1383_d_n12 + eq108_e1387_d_n12);
        let eq108_e1388_d_n13: f64 = (eq108_e1383_d_n13 + eq108_e1387_d_n13);
        let eq108_e1388_d_n14: f64 = (eq108_e1383_d_n14 + eq108_e1387_d_n14);
        let eq108_e1388_d_n15: f64 = (eq108_e1383_d_n15 + eq108_e1387_d_n15);
        let eq108_e1388_d_n16: f64 = (eq108_e1383_d_n16 + eq108_e1387_d_n16);
        let eq108_e1388_d_n17: f64 = (eq108_e1383_d_n17 + eq108_e1387_d_n17);
        let eq108_e1388_d_n18: f64 = (eq108_e1383_d_n18 + eq108_e1387_d_n18);
        let eq108_e1388_d_n19: f64 = (eq108_e1383_d_n19 + eq108_e1387_d_n19);
        let eq108_e1388_d_n20: f64 = (eq108_e1383_d_n20 + eq108_e1387_d_n20);
        let eq108_e1388_d_n21: f64 = (eq108_e1383_d_n21 + eq108_e1387_d_n21);
        let eq108_e1388_d_n22: f64 = (eq108_e1383_d_n22 + eq108_e1387_d_n22);
        let eq108_e1388_d_n23: f64 = (eq108_e1383_d_n23 + eq108_e1387_d_n23);
        let eq108_e1388_d_n24: f64 = (eq108_e1383_d_n24 + eq108_e1387_d_n24);
        let eq108_e1388_d_n25: f64 = (eq108_e1383_d_n25 + eq108_e1387_d_n25);
        let eq108_e1388_d_n26: f64 = (eq108_e1383_d_n26 + eq108_e1387_d_n26);
        let eq108_e1388_d_n27: f64 = (eq108_e1383_d_n27 + eq108_e1387_d_n27);
        let eq108_e1388_d_n28: f64 = (eq108_e1383_d_n28 + eq108_e1387_d_n28);
        let eq108_e1388_d_n29: f64 = (eq108_e1383_d_n29 + eq108_e1387_d_n29);
        let eq108_value: f64 = eq108_e1388;
        let eq108_node_derivatives: [f64; 30] = [eq108_e1388_d_n0, eq108_e1388_d_n1, eq108_e1388_d_n2, eq108_e1388_d_n3, eq108_e1388_d_n4, eq108_e1388_d_n5, eq108_e1388_d_n6, eq108_e1388_d_n7, eq108_e1388_d_n8, eq108_e1388_d_n9, eq108_e1388_d_n10, eq108_e1388_d_n11, eq108_e1388_d_n12, eq108_e1388_d_n13, eq108_e1388_d_n14, eq108_e1388_d_n15, eq108_e1388_d_n16, eq108_e1388_d_n17, eq108_e1388_d_n18, eq108_e1388_d_n19, eq108_e1388_d_n20, eq108_e1388_d_n21, eq108_e1388_d_n22, eq108_e1388_d_n23, eq108_e1388_d_n24, eq108_e1388_d_n25, eq108_e1388_d_n26, eq108_e1388_d_n27, eq108_e1388_d_n28, eq108_e1388_d_n29];
        let eq108_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[11]),
            self.multiplicity * (eq108_value),
            &nodes,
            &eq108_node_derivatives,
            &branches,
            &eq108_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_109_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq109_e1396, eq109_e1396_d_n0, eq109_e1396_d_n1, eq109_e1396_d_n2, eq109_e1396_d_n3, eq109_e1396_d_n4, eq109_e1396_d_n5, eq109_e1396_d_n6, eq109_e1396_d_n7, eq109_e1396_d_n8, eq109_e1396_d_n9, eq109_e1396_d_n10, eq109_e1396_d_n11, eq109_e1396_d_n12, eq109_e1396_d_n13, eq109_e1396_d_n14, eq109_e1396_d_n15, eq109_e1396_d_n16, eq109_e1396_d_n17, eq109_e1396_d_n18, eq109_e1396_d_n19, eq109_e1396_d_n20, eq109_e1396_d_n21, eq109_e1396_d_n22, eq109_e1396_d_n23, eq109_e1396_d_n24, eq109_e1396_d_n25, eq109_e1396_d_n26, eq109_e1396_d_n27, eq109_e1396_d_n28, eq109_e1396_d_n29,) = {
    if (s.v[1202] != 0.0) {
        let eq109_e1393: f64 = (s.v[0] * (nv11 - nv12));
        let eq109_e1393_d_n11: f64 = s.v[0];
        let eq109_e1393_d_n12: f64 = (-s.v[0]);
        let eq109_e1394: f64 = (s.v[178] + eq109_e1393);
        let eq109_e1394_d_n11: f64 = (s.dn[178][11] + eq109_e1393_d_n11);
        let eq109_e1394_d_n12: f64 = (s.dn[178][12] + eq109_e1393_d_n12);
        (eq109_e1394, s.dn[178][0], s.dn[178][1], s.dn[178][2], s.dn[178][3], s.dn[178][4], s.dn[178][5], s.dn[178][6], s.dn[178][7], s.dn[178][8], s.dn[178][9], s.dn[178][10], eq109_e1394_d_n11, eq109_e1394_d_n12, s.dn[178][13], s.dn[178][14], s.dn[178][15], s.dn[178][16], s.dn[178][17], s.dn[178][18], s.dn[178][19], s.dn[178][20], s.dn[178][21], s.dn[178][22], s.dn[178][23], s.dn[178][24], s.dn[178][25], s.dn[178][26], s.dn[178][27], s.dn[178][28], s.dn[178][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq109_value: f64 = eq109_e1396;
        let eq109_node_derivatives: [f64; 30] = [eq109_e1396_d_n0, eq109_e1396_d_n1, eq109_e1396_d_n2, eq109_e1396_d_n3, eq109_e1396_d_n4, eq109_e1396_d_n5, eq109_e1396_d_n6, eq109_e1396_d_n7, eq109_e1396_d_n8, eq109_e1396_d_n9, eq109_e1396_d_n10, eq109_e1396_d_n11, eq109_e1396_d_n12, eq109_e1396_d_n13, eq109_e1396_d_n14, eq109_e1396_d_n15, eq109_e1396_d_n16, eq109_e1396_d_n17, eq109_e1396_d_n18, eq109_e1396_d_n19, eq109_e1396_d_n20, eq109_e1396_d_n21, eq109_e1396_d_n22, eq109_e1396_d_n23, eq109_e1396_d_n24, eq109_e1396_d_n25, eq109_e1396_d_n26, eq109_e1396_d_n27, eq109_e1396_d_n28, eq109_e1396_d_n29];
        let eq109_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[12]),
            self.multiplicity * (eq109_value),
            &nodes,
            &eq109_node_derivatives,
            &branches,
            &eq109_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_110_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq110_e1401,) = {
    if (!(s.v[1202] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq110_value: f64 = eq110_e1401;
        stamper.stamp_potential(
            branches[24],
            eq110_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_111_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq111_e1411, eq111_e1411_d_n0, eq111_e1411_d_n1, eq111_e1411_d_n2, eq111_e1411_d_n3, eq111_e1411_d_n4, eq111_e1411_d_n5, eq111_e1411_d_n6, eq111_e1411_d_n7, eq111_e1411_d_n8, eq111_e1411_d_n9, eq111_e1411_d_n10, eq111_e1411_d_n11, eq111_e1411_d_n12, eq111_e1411_d_n13, eq111_e1411_d_n14, eq111_e1411_d_n15, eq111_e1411_d_n16, eq111_e1411_d_n17, eq111_e1411_d_n18, eq111_e1411_d_n19, eq111_e1411_d_n20, eq111_e1411_d_n21, eq111_e1411_d_n22, eq111_e1411_d_n23, eq111_e1411_d_n24, eq111_e1411_d_n25, eq111_e1411_d_n26, eq111_e1411_d_n27, eq111_e1411_d_n28, eq111_e1411_d_n29,) = {
    if (s.v[1348] != 0.0) {
        let eq111_e1404: f64 = self.eval_ddt(100, s.v[179]);
        let eq111_e1404_d_n0: f64 = self.ddt_jacobian(s.dn[179][0]);
        let eq111_e1404_d_n1: f64 = self.ddt_jacobian(s.dn[179][1]);
        let eq111_e1404_d_n2: f64 = self.ddt_jacobian(s.dn[179][2]);
        let eq111_e1404_d_n3: f64 = self.ddt_jacobian(s.dn[179][3]);
        let eq111_e1404_d_n4: f64 = self.ddt_jacobian(s.dn[179][4]);
        let eq111_e1404_d_n5: f64 = self.ddt_jacobian(s.dn[179][5]);
        let eq111_e1404_d_n6: f64 = self.ddt_jacobian(s.dn[179][6]);
        let eq111_e1404_d_n7: f64 = self.ddt_jacobian(s.dn[179][7]);
        let eq111_e1404_d_n8: f64 = self.ddt_jacobian(s.dn[179][8]);
        let eq111_e1404_d_n9: f64 = self.ddt_jacobian(s.dn[179][9]);
        let eq111_e1404_d_n10: f64 = self.ddt_jacobian(s.dn[179][10]);
        let eq111_e1404_d_n11: f64 = self.ddt_jacobian(s.dn[179][11]);
        let eq111_e1404_d_n12: f64 = self.ddt_jacobian(s.dn[179][12]);
        let eq111_e1404_d_n13: f64 = self.ddt_jacobian(s.dn[179][13]);
        let eq111_e1404_d_n14: f64 = self.ddt_jacobian(s.dn[179][14]);
        let eq111_e1404_d_n15: f64 = self.ddt_jacobian(s.dn[179][15]);
        let eq111_e1404_d_n16: f64 = self.ddt_jacobian(s.dn[179][16]);
        let eq111_e1404_d_n17: f64 = self.ddt_jacobian(s.dn[179][17]);
        let eq111_e1404_d_n18: f64 = self.ddt_jacobian(s.dn[179][18]);
        let eq111_e1404_d_n19: f64 = self.ddt_jacobian(s.dn[179][19]);
        let eq111_e1404_d_n20: f64 = self.ddt_jacobian(s.dn[179][20]);
        let eq111_e1404_d_n21: f64 = self.ddt_jacobian(s.dn[179][21]);
        let eq111_e1404_d_n22: f64 = self.ddt_jacobian(s.dn[179][22]);
        let eq111_e1404_d_n23: f64 = self.ddt_jacobian(s.dn[179][23]);
        let eq111_e1404_d_n24: f64 = self.ddt_jacobian(s.dn[179][24]);
        let eq111_e1404_d_n25: f64 = self.ddt_jacobian(s.dn[179][25]);
        let eq111_e1404_d_n26: f64 = self.ddt_jacobian(s.dn[179][26]);
        let eq111_e1404_d_n27: f64 = self.ddt_jacobian(s.dn[179][27]);
        let eq111_e1404_d_n28: f64 = self.ddt_jacobian(s.dn[179][28]);
        let eq111_e1404_d_n29: f64 = self.ddt_jacobian(s.dn[179][29]);
        let eq111_e1407: f64 = (p.p355 * (nv7 - nv12));
        let eq111_e1407_d_n7: f64 = p.p355;
        let eq111_e1407_d_n12: f64 = (-p.p355);
        let eq111_e1408: f64 = self.eval_ddt(101, eq111_e1407);
        let eq111_e1408_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n7: f64 = self.ddt_jacobian(eq111_e1407_d_n7);
        let eq111_e1408_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n12: f64 = self.ddt_jacobian(eq111_e1407_d_n12);
        let eq111_e1408_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq111_e1408_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq111_e1409: f64 = (eq111_e1404 + eq111_e1408);
        let eq111_e1409_d_n0: f64 = (eq111_e1404_d_n0 + eq111_e1408_d_n0);
        let eq111_e1409_d_n1: f64 = (eq111_e1404_d_n1 + eq111_e1408_d_n1);
        let eq111_e1409_d_n2: f64 = (eq111_e1404_d_n2 + eq111_e1408_d_n2);
        let eq111_e1409_d_n3: f64 = (eq111_e1404_d_n3 + eq111_e1408_d_n3);
        let eq111_e1409_d_n4: f64 = (eq111_e1404_d_n4 + eq111_e1408_d_n4);
        let eq111_e1409_d_n5: f64 = (eq111_e1404_d_n5 + eq111_e1408_d_n5);
        let eq111_e1409_d_n6: f64 = (eq111_e1404_d_n6 + eq111_e1408_d_n6);
        let eq111_e1409_d_n7: f64 = (eq111_e1404_d_n7 + eq111_e1408_d_n7);
        let eq111_e1409_d_n8: f64 = (eq111_e1404_d_n8 + eq111_e1408_d_n8);
        let eq111_e1409_d_n9: f64 = (eq111_e1404_d_n9 + eq111_e1408_d_n9);
        let eq111_e1409_d_n10: f64 = (eq111_e1404_d_n10 + eq111_e1408_d_n10);
        let eq111_e1409_d_n11: f64 = (eq111_e1404_d_n11 + eq111_e1408_d_n11);
        let eq111_e1409_d_n12: f64 = (eq111_e1404_d_n12 + eq111_e1408_d_n12);
        let eq111_e1409_d_n13: f64 = (eq111_e1404_d_n13 + eq111_e1408_d_n13);
        let eq111_e1409_d_n14: f64 = (eq111_e1404_d_n14 + eq111_e1408_d_n14);
        let eq111_e1409_d_n15: f64 = (eq111_e1404_d_n15 + eq111_e1408_d_n15);
        let eq111_e1409_d_n16: f64 = (eq111_e1404_d_n16 + eq111_e1408_d_n16);
        let eq111_e1409_d_n17: f64 = (eq111_e1404_d_n17 + eq111_e1408_d_n17);
        let eq111_e1409_d_n18: f64 = (eq111_e1404_d_n18 + eq111_e1408_d_n18);
        let eq111_e1409_d_n19: f64 = (eq111_e1404_d_n19 + eq111_e1408_d_n19);
        let eq111_e1409_d_n20: f64 = (eq111_e1404_d_n20 + eq111_e1408_d_n20);
        let eq111_e1409_d_n21: f64 = (eq111_e1404_d_n21 + eq111_e1408_d_n21);
        let eq111_e1409_d_n22: f64 = (eq111_e1404_d_n22 + eq111_e1408_d_n22);
        let eq111_e1409_d_n23: f64 = (eq111_e1404_d_n23 + eq111_e1408_d_n23);
        let eq111_e1409_d_n24: f64 = (eq111_e1404_d_n24 + eq111_e1408_d_n24);
        let eq111_e1409_d_n25: f64 = (eq111_e1404_d_n25 + eq111_e1408_d_n25);
        let eq111_e1409_d_n26: f64 = (eq111_e1404_d_n26 + eq111_e1408_d_n26);
        let eq111_e1409_d_n27: f64 = (eq111_e1404_d_n27 + eq111_e1408_d_n27);
        let eq111_e1409_d_n28: f64 = (eq111_e1404_d_n28 + eq111_e1408_d_n28);
        let eq111_e1409_d_n29: f64 = (eq111_e1404_d_n29 + eq111_e1408_d_n29);
        (eq111_e1409, eq111_e1409_d_n0, eq111_e1409_d_n1, eq111_e1409_d_n2, eq111_e1409_d_n3, eq111_e1409_d_n4, eq111_e1409_d_n5, eq111_e1409_d_n6, eq111_e1409_d_n7, eq111_e1409_d_n8, eq111_e1409_d_n9, eq111_e1409_d_n10, eq111_e1409_d_n11, eq111_e1409_d_n12, eq111_e1409_d_n13, eq111_e1409_d_n14, eq111_e1409_d_n15, eq111_e1409_d_n16, eq111_e1409_d_n17, eq111_e1409_d_n18, eq111_e1409_d_n19, eq111_e1409_d_n20, eq111_e1409_d_n21, eq111_e1409_d_n22, eq111_e1409_d_n23, eq111_e1409_d_n24, eq111_e1409_d_n25, eq111_e1409_d_n26, eq111_e1409_d_n27, eq111_e1409_d_n28, eq111_e1409_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq111_value: f64 = eq111_e1411;
        let eq111_node_derivatives: [f64; 30] = [eq111_e1411_d_n0, eq111_e1411_d_n1, eq111_e1411_d_n2, eq111_e1411_d_n3, eq111_e1411_d_n4, eq111_e1411_d_n5, eq111_e1411_d_n6, eq111_e1411_d_n7, eq111_e1411_d_n8, eq111_e1411_d_n9, eq111_e1411_d_n10, eq111_e1411_d_n11, eq111_e1411_d_n12, eq111_e1411_d_n13, eq111_e1411_d_n14, eq111_e1411_d_n15, eq111_e1411_d_n16, eq111_e1411_d_n17, eq111_e1411_d_n18, eq111_e1411_d_n19, eq111_e1411_d_n20, eq111_e1411_d_n21, eq111_e1411_d_n22, eq111_e1411_d_n23, eq111_e1411_d_n24, eq111_e1411_d_n25, eq111_e1411_d_n26, eq111_e1411_d_n27, eq111_e1411_d_n28, eq111_e1411_d_n29];
        let eq111_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[12]),
            self.multiplicity * (eq111_value),
            &nodes,
            &eq111_node_derivatives,
            &branches,
            &eq111_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_112_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq112_e1421, eq112_e1421_d_n0, eq112_e1421_d_n1, eq112_e1421_d_n2, eq112_e1421_d_n3, eq112_e1421_d_n4, eq112_e1421_d_n5, eq112_e1421_d_n6, eq112_e1421_d_n7, eq112_e1421_d_n8, eq112_e1421_d_n9, eq112_e1421_d_n10, eq112_e1421_d_n11, eq112_e1421_d_n12, eq112_e1421_d_n13, eq112_e1421_d_n14, eq112_e1421_d_n15, eq112_e1421_d_n16, eq112_e1421_d_n17, eq112_e1421_d_n18, eq112_e1421_d_n19, eq112_e1421_d_n20, eq112_e1421_d_n21, eq112_e1421_d_n22, eq112_e1421_d_n23, eq112_e1421_d_n24, eq112_e1421_d_n25, eq112_e1421_d_n26, eq112_e1421_d_n27, eq112_e1421_d_n28, eq112_e1421_d_n29,) = {
    if (s.v[1348] != 0.0) {
        let eq112_e1414: f64 = self.eval_ddt(102, s.v[180]);
        let eq112_e1414_d_n0: f64 = self.ddt_jacobian(s.dn[180][0]);
        let eq112_e1414_d_n1: f64 = self.ddt_jacobian(s.dn[180][1]);
        let eq112_e1414_d_n2: f64 = self.ddt_jacobian(s.dn[180][2]);
        let eq112_e1414_d_n3: f64 = self.ddt_jacobian(s.dn[180][3]);
        let eq112_e1414_d_n4: f64 = self.ddt_jacobian(s.dn[180][4]);
        let eq112_e1414_d_n5: f64 = self.ddt_jacobian(s.dn[180][5]);
        let eq112_e1414_d_n6: f64 = self.ddt_jacobian(s.dn[180][6]);
        let eq112_e1414_d_n7: f64 = self.ddt_jacobian(s.dn[180][7]);
        let eq112_e1414_d_n8: f64 = self.ddt_jacobian(s.dn[180][8]);
        let eq112_e1414_d_n9: f64 = self.ddt_jacobian(s.dn[180][9]);
        let eq112_e1414_d_n10: f64 = self.ddt_jacobian(s.dn[180][10]);
        let eq112_e1414_d_n11: f64 = self.ddt_jacobian(s.dn[180][11]);
        let eq112_e1414_d_n12: f64 = self.ddt_jacobian(s.dn[180][12]);
        let eq112_e1414_d_n13: f64 = self.ddt_jacobian(s.dn[180][13]);
        let eq112_e1414_d_n14: f64 = self.ddt_jacobian(s.dn[180][14]);
        let eq112_e1414_d_n15: f64 = self.ddt_jacobian(s.dn[180][15]);
        let eq112_e1414_d_n16: f64 = self.ddt_jacobian(s.dn[180][16]);
        let eq112_e1414_d_n17: f64 = self.ddt_jacobian(s.dn[180][17]);
        let eq112_e1414_d_n18: f64 = self.ddt_jacobian(s.dn[180][18]);
        let eq112_e1414_d_n19: f64 = self.ddt_jacobian(s.dn[180][19]);
        let eq112_e1414_d_n20: f64 = self.ddt_jacobian(s.dn[180][20]);
        let eq112_e1414_d_n21: f64 = self.ddt_jacobian(s.dn[180][21]);
        let eq112_e1414_d_n22: f64 = self.ddt_jacobian(s.dn[180][22]);
        let eq112_e1414_d_n23: f64 = self.ddt_jacobian(s.dn[180][23]);
        let eq112_e1414_d_n24: f64 = self.ddt_jacobian(s.dn[180][24]);
        let eq112_e1414_d_n25: f64 = self.ddt_jacobian(s.dn[180][25]);
        let eq112_e1414_d_n26: f64 = self.ddt_jacobian(s.dn[180][26]);
        let eq112_e1414_d_n27: f64 = self.ddt_jacobian(s.dn[180][27]);
        let eq112_e1414_d_n28: f64 = self.ddt_jacobian(s.dn[180][28]);
        let eq112_e1414_d_n29: f64 = self.ddt_jacobian(s.dn[180][29]);
        let eq112_e1417: f64 = (p.p355 * (nv7 - nv11));
        let eq112_e1417_d_n7: f64 = p.p355;
        let eq112_e1417_d_n11: f64 = (-p.p355);
        let eq112_e1418: f64 = self.eval_ddt(103, eq112_e1417);
        let eq112_e1418_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n7: f64 = self.ddt_jacobian(eq112_e1417_d_n7);
        let eq112_e1418_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n11: f64 = self.ddt_jacobian(eq112_e1417_d_n11);
        let eq112_e1418_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq112_e1418_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq112_e1419: f64 = (eq112_e1414 + eq112_e1418);
        let eq112_e1419_d_n0: f64 = (eq112_e1414_d_n0 + eq112_e1418_d_n0);
        let eq112_e1419_d_n1: f64 = (eq112_e1414_d_n1 + eq112_e1418_d_n1);
        let eq112_e1419_d_n2: f64 = (eq112_e1414_d_n2 + eq112_e1418_d_n2);
        let eq112_e1419_d_n3: f64 = (eq112_e1414_d_n3 + eq112_e1418_d_n3);
        let eq112_e1419_d_n4: f64 = (eq112_e1414_d_n4 + eq112_e1418_d_n4);
        let eq112_e1419_d_n5: f64 = (eq112_e1414_d_n5 + eq112_e1418_d_n5);
        let eq112_e1419_d_n6: f64 = (eq112_e1414_d_n6 + eq112_e1418_d_n6);
        let eq112_e1419_d_n7: f64 = (eq112_e1414_d_n7 + eq112_e1418_d_n7);
        let eq112_e1419_d_n8: f64 = (eq112_e1414_d_n8 + eq112_e1418_d_n8);
        let eq112_e1419_d_n9: f64 = (eq112_e1414_d_n9 + eq112_e1418_d_n9);
        let eq112_e1419_d_n10: f64 = (eq112_e1414_d_n10 + eq112_e1418_d_n10);
        let eq112_e1419_d_n11: f64 = (eq112_e1414_d_n11 + eq112_e1418_d_n11);
        let eq112_e1419_d_n12: f64 = (eq112_e1414_d_n12 + eq112_e1418_d_n12);
        let eq112_e1419_d_n13: f64 = (eq112_e1414_d_n13 + eq112_e1418_d_n13);
        let eq112_e1419_d_n14: f64 = (eq112_e1414_d_n14 + eq112_e1418_d_n14);
        let eq112_e1419_d_n15: f64 = (eq112_e1414_d_n15 + eq112_e1418_d_n15);
        let eq112_e1419_d_n16: f64 = (eq112_e1414_d_n16 + eq112_e1418_d_n16);
        let eq112_e1419_d_n17: f64 = (eq112_e1414_d_n17 + eq112_e1418_d_n17);
        let eq112_e1419_d_n18: f64 = (eq112_e1414_d_n18 + eq112_e1418_d_n18);
        let eq112_e1419_d_n19: f64 = (eq112_e1414_d_n19 + eq112_e1418_d_n19);
        let eq112_e1419_d_n20: f64 = (eq112_e1414_d_n20 + eq112_e1418_d_n20);
        let eq112_e1419_d_n21: f64 = (eq112_e1414_d_n21 + eq112_e1418_d_n21);
        let eq112_e1419_d_n22: f64 = (eq112_e1414_d_n22 + eq112_e1418_d_n22);
        let eq112_e1419_d_n23: f64 = (eq112_e1414_d_n23 + eq112_e1418_d_n23);
        let eq112_e1419_d_n24: f64 = (eq112_e1414_d_n24 + eq112_e1418_d_n24);
        let eq112_e1419_d_n25: f64 = (eq112_e1414_d_n25 + eq112_e1418_d_n25);
        let eq112_e1419_d_n26: f64 = (eq112_e1414_d_n26 + eq112_e1418_d_n26);
        let eq112_e1419_d_n27: f64 = (eq112_e1414_d_n27 + eq112_e1418_d_n27);
        let eq112_e1419_d_n28: f64 = (eq112_e1414_d_n28 + eq112_e1418_d_n28);
        let eq112_e1419_d_n29: f64 = (eq112_e1414_d_n29 + eq112_e1418_d_n29);
        (eq112_e1419, eq112_e1419_d_n0, eq112_e1419_d_n1, eq112_e1419_d_n2, eq112_e1419_d_n3, eq112_e1419_d_n4, eq112_e1419_d_n5, eq112_e1419_d_n6, eq112_e1419_d_n7, eq112_e1419_d_n8, eq112_e1419_d_n9, eq112_e1419_d_n10, eq112_e1419_d_n11, eq112_e1419_d_n12, eq112_e1419_d_n13, eq112_e1419_d_n14, eq112_e1419_d_n15, eq112_e1419_d_n16, eq112_e1419_d_n17, eq112_e1419_d_n18, eq112_e1419_d_n19, eq112_e1419_d_n20, eq112_e1419_d_n21, eq112_e1419_d_n22, eq112_e1419_d_n23, eq112_e1419_d_n24, eq112_e1419_d_n25, eq112_e1419_d_n26, eq112_e1419_d_n27, eq112_e1419_d_n28, eq112_e1419_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq112_value: f64 = eq112_e1421;
        let eq112_node_derivatives: [f64; 30] = [eq112_e1421_d_n0, eq112_e1421_d_n1, eq112_e1421_d_n2, eq112_e1421_d_n3, eq112_e1421_d_n4, eq112_e1421_d_n5, eq112_e1421_d_n6, eq112_e1421_d_n7, eq112_e1421_d_n8, eq112_e1421_d_n9, eq112_e1421_d_n10, eq112_e1421_d_n11, eq112_e1421_d_n12, eq112_e1421_d_n13, eq112_e1421_d_n14, eq112_e1421_d_n15, eq112_e1421_d_n16, eq112_e1421_d_n17, eq112_e1421_d_n18, eq112_e1421_d_n19, eq112_e1421_d_n20, eq112_e1421_d_n21, eq112_e1421_d_n22, eq112_e1421_d_n23, eq112_e1421_d_n24, eq112_e1421_d_n25, eq112_e1421_d_n26, eq112_e1421_d_n27, eq112_e1421_d_n28, eq112_e1421_d_n29];
        let eq112_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            self.multiplicity * (eq112_value),
            &nodes,
            &eq112_node_derivatives,
            &branches,
            &eq112_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_113_block_0(
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq113_e1431, eq113_e1431_d_n0, eq113_e1431_d_n1, eq113_e1431_d_n2, eq113_e1431_d_n3, eq113_e1431_d_n4, eq113_e1431_d_n5, eq113_e1431_d_n6, eq113_e1431_d_n7, eq113_e1431_d_n8, eq113_e1431_d_n9, eq113_e1431_d_n10, eq113_e1431_d_n11, eq113_e1431_d_n12, eq113_e1431_d_n13, eq113_e1431_d_n14, eq113_e1431_d_n15, eq113_e1431_d_n16, eq113_e1431_d_n17, eq113_e1431_d_n18, eq113_e1431_d_n19, eq113_e1431_d_n20, eq113_e1431_d_n21, eq113_e1431_d_n22, eq113_e1431_d_n23, eq113_e1431_d_n24, eq113_e1431_d_n25, eq113_e1431_d_n26, eq113_e1431_d_n27, eq113_e1431_d_n28, eq113_e1431_d_n29,) = {
    if (s.v[1348] != 0.0) {
        let eq113_e1424: f64 = self.eval_ddt(104, s.v[181]);
        let eq113_e1424_d_n0: f64 = self.ddt_jacobian(s.dn[181][0]);
        let eq113_e1424_d_n1: f64 = self.ddt_jacobian(s.dn[181][1]);
        let eq113_e1424_d_n2: f64 = self.ddt_jacobian(s.dn[181][2]);
        let eq113_e1424_d_n3: f64 = self.ddt_jacobian(s.dn[181][3]);
        let eq113_e1424_d_n4: f64 = self.ddt_jacobian(s.dn[181][4]);
        let eq113_e1424_d_n5: f64 = self.ddt_jacobian(s.dn[181][5]);
        let eq113_e1424_d_n6: f64 = self.ddt_jacobian(s.dn[181][6]);
        let eq113_e1424_d_n7: f64 = self.ddt_jacobian(s.dn[181][7]);
        let eq113_e1424_d_n8: f64 = self.ddt_jacobian(s.dn[181][8]);
        let eq113_e1424_d_n9: f64 = self.ddt_jacobian(s.dn[181][9]);
        let eq113_e1424_d_n10: f64 = self.ddt_jacobian(s.dn[181][10]);
        let eq113_e1424_d_n11: f64 = self.ddt_jacobian(s.dn[181][11]);
        let eq113_e1424_d_n12: f64 = self.ddt_jacobian(s.dn[181][12]);
        let eq113_e1424_d_n13: f64 = self.ddt_jacobian(s.dn[181][13]);
        let eq113_e1424_d_n14: f64 = self.ddt_jacobian(s.dn[181][14]);
        let eq113_e1424_d_n15: f64 = self.ddt_jacobian(s.dn[181][15]);
        let eq113_e1424_d_n16: f64 = self.ddt_jacobian(s.dn[181][16]);
        let eq113_e1424_d_n17: f64 = self.ddt_jacobian(s.dn[181][17]);
        let eq113_e1424_d_n18: f64 = self.ddt_jacobian(s.dn[181][18]);
        let eq113_e1424_d_n19: f64 = self.ddt_jacobian(s.dn[181][19]);
        let eq113_e1424_d_n20: f64 = self.ddt_jacobian(s.dn[181][20]);
        let eq113_e1424_d_n21: f64 = self.ddt_jacobian(s.dn[181][21]);
        let eq113_e1424_d_n22: f64 = self.ddt_jacobian(s.dn[181][22]);
        let eq113_e1424_d_n23: f64 = self.ddt_jacobian(s.dn[181][23]);
        let eq113_e1424_d_n24: f64 = self.ddt_jacobian(s.dn[181][24]);
        let eq113_e1424_d_n25: f64 = self.ddt_jacobian(s.dn[181][25]);
        let eq113_e1424_d_n26: f64 = self.ddt_jacobian(s.dn[181][26]);
        let eq113_e1424_d_n27: f64 = self.ddt_jacobian(s.dn[181][27]);
        let eq113_e1424_d_n28: f64 = self.ddt_jacobian(s.dn[181][28]);
        let eq113_e1424_d_n29: f64 = self.ddt_jacobian(s.dn[181][29]);
        let eq113_e1427: f64 = (p.p355 * (nv2 - nv12));
        let eq113_e1427_d_n2: f64 = p.p355;
        let eq113_e1427_d_n12: f64 = (-p.p355);
        let eq113_e1428: f64 = self.eval_ddt(105, eq113_e1427);
        let eq113_e1428_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n2: f64 = self.ddt_jacobian(eq113_e1427_d_n2);
        let eq113_e1428_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n12: f64 = self.ddt_jacobian(eq113_e1427_d_n12);
        let eq113_e1428_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq113_e1428_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq113_e1429: f64 = (eq113_e1424 + eq113_e1428);
        let eq113_e1429_d_n0: f64 = (eq113_e1424_d_n0 + eq113_e1428_d_n0);
        let eq113_e1429_d_n1: f64 = (eq113_e1424_d_n1 + eq113_e1428_d_n1);
        let eq113_e1429_d_n2: f64 = (eq113_e1424_d_n2 + eq113_e1428_d_n2);
        let eq113_e1429_d_n3: f64 = (eq113_e1424_d_n3 + eq113_e1428_d_n3);
        let eq113_e1429_d_n4: f64 = (eq113_e1424_d_n4 + eq113_e1428_d_n4);
        let eq113_e1429_d_n5: f64 = (eq113_e1424_d_n5 + eq113_e1428_d_n5);
        let eq113_e1429_d_n6: f64 = (eq113_e1424_d_n6 + eq113_e1428_d_n6);
        let eq113_e1429_d_n7: f64 = (eq113_e1424_d_n7 + eq113_e1428_d_n7);
        let eq113_e1429_d_n8: f64 = (eq113_e1424_d_n8 + eq113_e1428_d_n8);
        let eq113_e1429_d_n9: f64 = (eq113_e1424_d_n9 + eq113_e1428_d_n9);
        let eq113_e1429_d_n10: f64 = (eq113_e1424_d_n10 + eq113_e1428_d_n10);
        let eq113_e1429_d_n11: f64 = (eq113_e1424_d_n11 + eq113_e1428_d_n11);
        let eq113_e1429_d_n12: f64 = (eq113_e1424_d_n12 + eq113_e1428_d_n12);
        let eq113_e1429_d_n13: f64 = (eq113_e1424_d_n13 + eq113_e1428_d_n13);
        let eq113_e1429_d_n14: f64 = (eq113_e1424_d_n14 + eq113_e1428_d_n14);
        let eq113_e1429_d_n15: f64 = (eq113_e1424_d_n15 + eq113_e1428_d_n15);
        let eq113_e1429_d_n16: f64 = (eq113_e1424_d_n16 + eq113_e1428_d_n16);
        let eq113_e1429_d_n17: f64 = (eq113_e1424_d_n17 + eq113_e1428_d_n17);
        let eq113_e1429_d_n18: f64 = (eq113_e1424_d_n18 + eq113_e1428_d_n18);
        let eq113_e1429_d_n19: f64 = (eq113_e1424_d_n19 + eq113_e1428_d_n19);
        let eq113_e1429_d_n20: f64 = (eq113_e1424_d_n20 + eq113_e1428_d_n20);
        let eq113_e1429_d_n21: f64 = (eq113_e1424_d_n21 + eq113_e1428_d_n21);
        let eq113_e1429_d_n22: f64 = (eq113_e1424_d_n22 + eq113_e1428_d_n22);
        let eq113_e1429_d_n23: f64 = (eq113_e1424_d_n23 + eq113_e1428_d_n23);
        let eq113_e1429_d_n24: f64 = (eq113_e1424_d_n24 + eq113_e1428_d_n24);
        let eq113_e1429_d_n25: f64 = (eq113_e1424_d_n25 + eq113_e1428_d_n25);
        let eq113_e1429_d_n26: f64 = (eq113_e1424_d_n26 + eq113_e1428_d_n26);
        let eq113_e1429_d_n27: f64 = (eq113_e1424_d_n27 + eq113_e1428_d_n27);
        let eq113_e1429_d_n28: f64 = (eq113_e1424_d_n28 + eq113_e1428_d_n28);
        let eq113_e1429_d_n29: f64 = (eq113_e1424_d_n29 + eq113_e1428_d_n29);
        (eq113_e1429, eq113_e1429_d_n0, eq113_e1429_d_n1, eq113_e1429_d_n2, eq113_e1429_d_n3, eq113_e1429_d_n4, eq113_e1429_d_n5, eq113_e1429_d_n6, eq113_e1429_d_n7, eq113_e1429_d_n8, eq113_e1429_d_n9, eq113_e1429_d_n10, eq113_e1429_d_n11, eq113_e1429_d_n12, eq113_e1429_d_n13, eq113_e1429_d_n14, eq113_e1429_d_n15, eq113_e1429_d_n16, eq113_e1429_d_n17, eq113_e1429_d_n18, eq113_e1429_d_n19, eq113_e1429_d_n20, eq113_e1429_d_n21, eq113_e1429_d_n22, eq113_e1429_d_n23, eq113_e1429_d_n24, eq113_e1429_d_n25, eq113_e1429_d_n26, eq113_e1429_d_n27, eq113_e1429_d_n28, eq113_e1429_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq113_value: f64 = eq113_e1431;
        let eq113_node_derivatives: [f64; 30] = [eq113_e1431_d_n0, eq113_e1431_d_n1, eq113_e1431_d_n2, eq113_e1431_d_n3, eq113_e1431_d_n4, eq113_e1431_d_n5, eq113_e1431_d_n6, eq113_e1431_d_n7, eq113_e1431_d_n8, eq113_e1431_d_n9, eq113_e1431_d_n10, eq113_e1431_d_n11, eq113_e1431_d_n12, eq113_e1431_d_n13, eq113_e1431_d_n14, eq113_e1431_d_n15, eq113_e1431_d_n16, eq113_e1431_d_n17, eq113_e1431_d_n18, eq113_e1431_d_n19, eq113_e1431_d_n20, eq113_e1431_d_n21, eq113_e1431_d_n22, eq113_e1431_d_n23, eq113_e1431_d_n24, eq113_e1431_d_n25, eq113_e1431_d_n26, eq113_e1431_d_n27, eq113_e1431_d_n28, eq113_e1431_d_n29];
        let eq113_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[12]),
            self.multiplicity * (eq113_value),
            &nodes,
            &eq113_node_derivatives,
            &branches,
            &eq113_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_114_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq114_e1435,) = {
    if (s.v[1348] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq114_value: f64 = eq114_e1435;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[11]),
            self.multiplicity * (eq114_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_115_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq115_e1445, eq115_e1445_d_n0, eq115_e1445_d_n1, eq115_e1445_d_n2, eq115_e1445_d_n3, eq115_e1445_d_n4, eq115_e1445_d_n5, eq115_e1445_d_n6, eq115_e1445_d_n7, eq115_e1445_d_n8, eq115_e1445_d_n9, eq115_e1445_d_n10, eq115_e1445_d_n11, eq115_e1445_d_n12, eq115_e1445_d_n13, eq115_e1445_d_n14, eq115_e1445_d_n15, eq115_e1445_d_n16, eq115_e1445_d_n17, eq115_e1445_d_n18, eq115_e1445_d_n19, eq115_e1445_d_n20, eq115_e1445_d_n21, eq115_e1445_d_n22, eq115_e1445_d_n23, eq115_e1445_d_n24, eq115_e1445_d_n25, eq115_e1445_d_n26, eq115_e1445_d_n27, eq115_e1445_d_n28, eq115_e1445_d_n29,) = {
    if (s.v[1348] != 0.0) {
        let eq115_e1438: f64 = self.eval_ddt(106, s.v[183]);
        let eq115_e1438_d_n0: f64 = self.ddt_jacobian(s.dn[183][0]);
        let eq115_e1438_d_n1: f64 = self.ddt_jacobian(s.dn[183][1]);
        let eq115_e1438_d_n2: f64 = self.ddt_jacobian(s.dn[183][2]);
        let eq115_e1438_d_n3: f64 = self.ddt_jacobian(s.dn[183][3]);
        let eq115_e1438_d_n4: f64 = self.ddt_jacobian(s.dn[183][4]);
        let eq115_e1438_d_n5: f64 = self.ddt_jacobian(s.dn[183][5]);
        let eq115_e1438_d_n6: f64 = self.ddt_jacobian(s.dn[183][6]);
        let eq115_e1438_d_n7: f64 = self.ddt_jacobian(s.dn[183][7]);
        let eq115_e1438_d_n8: f64 = self.ddt_jacobian(s.dn[183][8]);
        let eq115_e1438_d_n9: f64 = self.ddt_jacobian(s.dn[183][9]);
        let eq115_e1438_d_n10: f64 = self.ddt_jacobian(s.dn[183][10]);
        let eq115_e1438_d_n11: f64 = self.ddt_jacobian(s.dn[183][11]);
        let eq115_e1438_d_n12: f64 = self.ddt_jacobian(s.dn[183][12]);
        let eq115_e1438_d_n13: f64 = self.ddt_jacobian(s.dn[183][13]);
        let eq115_e1438_d_n14: f64 = self.ddt_jacobian(s.dn[183][14]);
        let eq115_e1438_d_n15: f64 = self.ddt_jacobian(s.dn[183][15]);
        let eq115_e1438_d_n16: f64 = self.ddt_jacobian(s.dn[183][16]);
        let eq115_e1438_d_n17: f64 = self.ddt_jacobian(s.dn[183][17]);
        let eq115_e1438_d_n18: f64 = self.ddt_jacobian(s.dn[183][18]);
        let eq115_e1438_d_n19: f64 = self.ddt_jacobian(s.dn[183][19]);
        let eq115_e1438_d_n20: f64 = self.ddt_jacobian(s.dn[183][20]);
        let eq115_e1438_d_n21: f64 = self.ddt_jacobian(s.dn[183][21]);
        let eq115_e1438_d_n22: f64 = self.ddt_jacobian(s.dn[183][22]);
        let eq115_e1438_d_n23: f64 = self.ddt_jacobian(s.dn[183][23]);
        let eq115_e1438_d_n24: f64 = self.ddt_jacobian(s.dn[183][24]);
        let eq115_e1438_d_n25: f64 = self.ddt_jacobian(s.dn[183][25]);
        let eq115_e1438_d_n26: f64 = self.ddt_jacobian(s.dn[183][26]);
        let eq115_e1438_d_n27: f64 = self.ddt_jacobian(s.dn[183][27]);
        let eq115_e1438_d_n28: f64 = self.ddt_jacobian(s.dn[183][28]);
        let eq115_e1438_d_n29: f64 = self.ddt_jacobian(s.dn[183][29]);
        let eq115_e1441: f64 = (p.p355 * (nv7 - nv9));
        let eq115_e1441_d_n7: f64 = p.p355;
        let eq115_e1441_d_n9: f64 = (-p.p355);
        let eq115_e1442: f64 = self.eval_ddt(107, eq115_e1441);
        let eq115_e1442_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n7: f64 = self.ddt_jacobian(eq115_e1441_d_n7);
        let eq115_e1442_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n9: f64 = self.ddt_jacobian(eq115_e1441_d_n9);
        let eq115_e1442_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq115_e1442_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq115_e1443: f64 = (eq115_e1438 + eq115_e1442);
        let eq115_e1443_d_n0: f64 = (eq115_e1438_d_n0 + eq115_e1442_d_n0);
        let eq115_e1443_d_n1: f64 = (eq115_e1438_d_n1 + eq115_e1442_d_n1);
        let eq115_e1443_d_n2: f64 = (eq115_e1438_d_n2 + eq115_e1442_d_n2);
        let eq115_e1443_d_n3: f64 = (eq115_e1438_d_n3 + eq115_e1442_d_n3);
        let eq115_e1443_d_n4: f64 = (eq115_e1438_d_n4 + eq115_e1442_d_n4);
        let eq115_e1443_d_n5: f64 = (eq115_e1438_d_n5 + eq115_e1442_d_n5);
        let eq115_e1443_d_n6: f64 = (eq115_e1438_d_n6 + eq115_e1442_d_n6);
        let eq115_e1443_d_n7: f64 = (eq115_e1438_d_n7 + eq115_e1442_d_n7);
        let eq115_e1443_d_n8: f64 = (eq115_e1438_d_n8 + eq115_e1442_d_n8);
        let eq115_e1443_d_n9: f64 = (eq115_e1438_d_n9 + eq115_e1442_d_n9);
        let eq115_e1443_d_n10: f64 = (eq115_e1438_d_n10 + eq115_e1442_d_n10);
        let eq115_e1443_d_n11: f64 = (eq115_e1438_d_n11 + eq115_e1442_d_n11);
        let eq115_e1443_d_n12: f64 = (eq115_e1438_d_n12 + eq115_e1442_d_n12);
        let eq115_e1443_d_n13: f64 = (eq115_e1438_d_n13 + eq115_e1442_d_n13);
        let eq115_e1443_d_n14: f64 = (eq115_e1438_d_n14 + eq115_e1442_d_n14);
        let eq115_e1443_d_n15: f64 = (eq115_e1438_d_n15 + eq115_e1442_d_n15);
        let eq115_e1443_d_n16: f64 = (eq115_e1438_d_n16 + eq115_e1442_d_n16);
        let eq115_e1443_d_n17: f64 = (eq115_e1438_d_n17 + eq115_e1442_d_n17);
        let eq115_e1443_d_n18: f64 = (eq115_e1438_d_n18 + eq115_e1442_d_n18);
        let eq115_e1443_d_n19: f64 = (eq115_e1438_d_n19 + eq115_e1442_d_n19);
        let eq115_e1443_d_n20: f64 = (eq115_e1438_d_n20 + eq115_e1442_d_n20);
        let eq115_e1443_d_n21: f64 = (eq115_e1438_d_n21 + eq115_e1442_d_n21);
        let eq115_e1443_d_n22: f64 = (eq115_e1438_d_n22 + eq115_e1442_d_n22);
        let eq115_e1443_d_n23: f64 = (eq115_e1438_d_n23 + eq115_e1442_d_n23);
        let eq115_e1443_d_n24: f64 = (eq115_e1438_d_n24 + eq115_e1442_d_n24);
        let eq115_e1443_d_n25: f64 = (eq115_e1438_d_n25 + eq115_e1442_d_n25);
        let eq115_e1443_d_n26: f64 = (eq115_e1438_d_n26 + eq115_e1442_d_n26);
        let eq115_e1443_d_n27: f64 = (eq115_e1438_d_n27 + eq115_e1442_d_n27);
        let eq115_e1443_d_n28: f64 = (eq115_e1438_d_n28 + eq115_e1442_d_n28);
        let eq115_e1443_d_n29: f64 = (eq115_e1438_d_n29 + eq115_e1442_d_n29);
        (eq115_e1443, eq115_e1443_d_n0, eq115_e1443_d_n1, eq115_e1443_d_n2, eq115_e1443_d_n3, eq115_e1443_d_n4, eq115_e1443_d_n5, eq115_e1443_d_n6, eq115_e1443_d_n7, eq115_e1443_d_n8, eq115_e1443_d_n9, eq115_e1443_d_n10, eq115_e1443_d_n11, eq115_e1443_d_n12, eq115_e1443_d_n13, eq115_e1443_d_n14, eq115_e1443_d_n15, eq115_e1443_d_n16, eq115_e1443_d_n17, eq115_e1443_d_n18, eq115_e1443_d_n19, eq115_e1443_d_n20, eq115_e1443_d_n21, eq115_e1443_d_n22, eq115_e1443_d_n23, eq115_e1443_d_n24, eq115_e1443_d_n25, eq115_e1443_d_n26, eq115_e1443_d_n27, eq115_e1443_d_n28, eq115_e1443_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq115_value: f64 = eq115_e1445;
        let eq115_node_derivatives: [f64; 30] = [eq115_e1445_d_n0, eq115_e1445_d_n1, eq115_e1445_d_n2, eq115_e1445_d_n3, eq115_e1445_d_n4, eq115_e1445_d_n5, eq115_e1445_d_n6, eq115_e1445_d_n7, eq115_e1445_d_n8, eq115_e1445_d_n9, eq115_e1445_d_n10, eq115_e1445_d_n11, eq115_e1445_d_n12, eq115_e1445_d_n13, eq115_e1445_d_n14, eq115_e1445_d_n15, eq115_e1445_d_n16, eq115_e1445_d_n17, eq115_e1445_d_n18, eq115_e1445_d_n19, eq115_e1445_d_n20, eq115_e1445_d_n21, eq115_e1445_d_n22, eq115_e1445_d_n23, eq115_e1445_d_n24, eq115_e1445_d_n25, eq115_e1445_d_n26, eq115_e1445_d_n27, eq115_e1445_d_n28, eq115_e1445_d_n29];
        let eq115_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            self.multiplicity * (eq115_value),
            &nodes,
            &eq115_node_derivatives,
            &branches,
            &eq115_branch_derivatives,
            self.multiplicity,
        );
    }
}
