#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let (eq36_e793,) = {
    if (s.v[466] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq36_value: f64 = eq36_e793;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[17]),
            self.multiplicity * (eq36_value),
            &[
            ],
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq37_e803, eq37_e803_d_n0, eq37_e803_d_n1, eq37_e803_d_n2, eq37_e803_d_n3, eq37_e803_d_n4, eq37_e803_d_n5, eq37_e803_d_n6, eq37_e803_d_n7, eq37_e803_d_n8, eq37_e803_d_n9, eq37_e803_d_n10, eq37_e803_d_n11, eq37_e803_d_n12, eq37_e803_d_n13, eq37_e803_d_n14, eq37_e803_d_n15, eq37_e803_d_n16, eq37_e803_d_n17, eq37_e803_d_n18, eq37_e803_d_n19, eq37_e803_d_n20, eq37_e803_d_n21, eq37_e803_d_n22, eq37_e803_d_n23, eq37_e803_d_n24, eq37_e803_d_n25, eq37_e803_d_n26, eq37_e803_d_n27, eq37_e803_d_n28, eq37_e803_d_n29,) = {
    if (s.v[466] != 0.0) {
        let eq37_e796: f64 = self.eval_ddt(10, s.v[213]);
        let eq37_e796_d_n0: f64 = self.ddt_jacobian(s.dn[213][0]);
        let eq37_e796_d_n1: f64 = self.ddt_jacobian(s.dn[213][1]);
        let eq37_e796_d_n2: f64 = self.ddt_jacobian(s.dn[213][2]);
        let eq37_e796_d_n3: f64 = self.ddt_jacobian(s.dn[213][3]);
        let eq37_e796_d_n4: f64 = self.ddt_jacobian(s.dn[213][4]);
        let eq37_e796_d_n5: f64 = self.ddt_jacobian(s.dn[213][5]);
        let eq37_e796_d_n6: f64 = self.ddt_jacobian(s.dn[213][6]);
        let eq37_e796_d_n7: f64 = self.ddt_jacobian(s.dn[213][7]);
        let eq37_e796_d_n8: f64 = self.ddt_jacobian(s.dn[213][8]);
        let eq37_e796_d_n9: f64 = self.ddt_jacobian(s.dn[213][9]);
        let eq37_e796_d_n10: f64 = self.ddt_jacobian(s.dn[213][10]);
        let eq37_e796_d_n11: f64 = self.ddt_jacobian(s.dn[213][11]);
        let eq37_e796_d_n12: f64 = self.ddt_jacobian(s.dn[213][12]);
        let eq37_e796_d_n13: f64 = self.ddt_jacobian(s.dn[213][13]);
        let eq37_e796_d_n14: f64 = self.ddt_jacobian(s.dn[213][14]);
        let eq37_e796_d_n15: f64 = self.ddt_jacobian(s.dn[213][15]);
        let eq37_e796_d_n16: f64 = self.ddt_jacobian(s.dn[213][16]);
        let eq37_e796_d_n17: f64 = self.ddt_jacobian(s.dn[213][17]);
        let eq37_e796_d_n18: f64 = self.ddt_jacobian(s.dn[213][18]);
        let eq37_e796_d_n19: f64 = self.ddt_jacobian(s.dn[213][19]);
        let eq37_e796_d_n20: f64 = self.ddt_jacobian(s.dn[213][20]);
        let eq37_e796_d_n21: f64 = self.ddt_jacobian(s.dn[213][21]);
        let eq37_e796_d_n22: f64 = self.ddt_jacobian(s.dn[213][22]);
        let eq37_e796_d_n23: f64 = self.ddt_jacobian(s.dn[213][23]);
        let eq37_e796_d_n24: f64 = self.ddt_jacobian(s.dn[213][24]);
        let eq37_e796_d_n25: f64 = self.ddt_jacobian(s.dn[213][25]);
        let eq37_e796_d_n26: f64 = self.ddt_jacobian(s.dn[213][26]);
        let eq37_e796_d_n27: f64 = self.ddt_jacobian(s.dn[213][27]);
        let eq37_e796_d_n28: f64 = self.ddt_jacobian(s.dn[213][28]);
        let eq37_e796_d_n29: f64 = self.ddt_jacobian(s.dn[213][29]);
        let eq37_e799: f64 = (p.p355 * (nv7 - nv9));
        let eq37_e799_d_n7: f64 = p.p355;
        let eq37_e799_d_n9: f64 = (-p.p355);
        let eq37_e800: f64 = self.eval_ddt(11, eq37_e799);
        let eq37_e800_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n7: f64 = self.ddt_jacobian(eq37_e799_d_n7);
        let eq37_e800_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n9: f64 = self.ddt_jacobian(eq37_e799_d_n9);
        let eq37_e800_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq37_e800_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq37_e801: f64 = (eq37_e796 + eq37_e800);
        let eq37_e801_d_n0: f64 = (eq37_e796_d_n0 + eq37_e800_d_n0);
        let eq37_e801_d_n1: f64 = (eq37_e796_d_n1 + eq37_e800_d_n1);
        let eq37_e801_d_n2: f64 = (eq37_e796_d_n2 + eq37_e800_d_n2);
        let eq37_e801_d_n3: f64 = (eq37_e796_d_n3 + eq37_e800_d_n3);
        let eq37_e801_d_n4: f64 = (eq37_e796_d_n4 + eq37_e800_d_n4);
        let eq37_e801_d_n5: f64 = (eq37_e796_d_n5 + eq37_e800_d_n5);
        let eq37_e801_d_n6: f64 = (eq37_e796_d_n6 + eq37_e800_d_n6);
        let eq37_e801_d_n7: f64 = (eq37_e796_d_n7 + eq37_e800_d_n7);
        let eq37_e801_d_n8: f64 = (eq37_e796_d_n8 + eq37_e800_d_n8);
        let eq37_e801_d_n9: f64 = (eq37_e796_d_n9 + eq37_e800_d_n9);
        let eq37_e801_d_n10: f64 = (eq37_e796_d_n10 + eq37_e800_d_n10);
        let eq37_e801_d_n11: f64 = (eq37_e796_d_n11 + eq37_e800_d_n11);
        let eq37_e801_d_n12: f64 = (eq37_e796_d_n12 + eq37_e800_d_n12);
        let eq37_e801_d_n13: f64 = (eq37_e796_d_n13 + eq37_e800_d_n13);
        let eq37_e801_d_n14: f64 = (eq37_e796_d_n14 + eq37_e800_d_n14);
        let eq37_e801_d_n15: f64 = (eq37_e796_d_n15 + eq37_e800_d_n15);
        let eq37_e801_d_n16: f64 = (eq37_e796_d_n16 + eq37_e800_d_n16);
        let eq37_e801_d_n17: f64 = (eq37_e796_d_n17 + eq37_e800_d_n17);
        let eq37_e801_d_n18: f64 = (eq37_e796_d_n18 + eq37_e800_d_n18);
        let eq37_e801_d_n19: f64 = (eq37_e796_d_n19 + eq37_e800_d_n19);
        let eq37_e801_d_n20: f64 = (eq37_e796_d_n20 + eq37_e800_d_n20);
        let eq37_e801_d_n21: f64 = (eq37_e796_d_n21 + eq37_e800_d_n21);
        let eq37_e801_d_n22: f64 = (eq37_e796_d_n22 + eq37_e800_d_n22);
        let eq37_e801_d_n23: f64 = (eq37_e796_d_n23 + eq37_e800_d_n23);
        let eq37_e801_d_n24: f64 = (eq37_e796_d_n24 + eq37_e800_d_n24);
        let eq37_e801_d_n25: f64 = (eq37_e796_d_n25 + eq37_e800_d_n25);
        let eq37_e801_d_n26: f64 = (eq37_e796_d_n26 + eq37_e800_d_n26);
        let eq37_e801_d_n27: f64 = (eq37_e796_d_n27 + eq37_e800_d_n27);
        let eq37_e801_d_n28: f64 = (eq37_e796_d_n28 + eq37_e800_d_n28);
        let eq37_e801_d_n29: f64 = (eq37_e796_d_n29 + eq37_e800_d_n29);
        (eq37_e801, eq37_e801_d_n0, eq37_e801_d_n1, eq37_e801_d_n2, eq37_e801_d_n3, eq37_e801_d_n4, eq37_e801_d_n5, eq37_e801_d_n6, eq37_e801_d_n7, eq37_e801_d_n8, eq37_e801_d_n9, eq37_e801_d_n10, eq37_e801_d_n11, eq37_e801_d_n12, eq37_e801_d_n13, eq37_e801_d_n14, eq37_e801_d_n15, eq37_e801_d_n16, eq37_e801_d_n17, eq37_e801_d_n18, eq37_e801_d_n19, eq37_e801_d_n20, eq37_e801_d_n21, eq37_e801_d_n22, eq37_e801_d_n23, eq37_e801_d_n24, eq37_e801_d_n25, eq37_e801_d_n26, eq37_e801_d_n27, eq37_e801_d_n28, eq37_e801_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_value: f64 = eq37_e803;
        let eq37_node_derivatives: [f64; 30] = [eq37_e803_d_n0, eq37_e803_d_n1, eq37_e803_d_n2, eq37_e803_d_n3, eq37_e803_d_n4, eq37_e803_d_n5, eq37_e803_d_n6, eq37_e803_d_n7, eq37_e803_d_n8, eq37_e803_d_n9, eq37_e803_d_n10, eq37_e803_d_n11, eq37_e803_d_n12, eq37_e803_d_n13, eq37_e803_d_n14, eq37_e803_d_n15, eq37_e803_d_n16, eq37_e803_d_n17, eq37_e803_d_n18, eq37_e803_d_n19, eq37_e803_d_n20, eq37_e803_d_n21, eq37_e803_d_n22, eq37_e803_d_n23, eq37_e803_d_n24, eq37_e803_d_n25, eq37_e803_d_n26, eq37_e803_d_n27, eq37_e803_d_n28, eq37_e803_d_n29];
        let eq37_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            self.multiplicity * (eq37_value),
            &nodes,
            &eq37_node_derivatives,
            &branches,
            &eq37_branch_derivatives,
            self.multiplicity,
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq38_e814, eq38_e814_d_n0, eq38_e814_d_n1, eq38_e814_d_n2, eq38_e814_d_n3, eq38_e814_d_n4, eq38_e814_d_n5, eq38_e814_d_n6, eq38_e814_d_n7, eq38_e814_d_n8, eq38_e814_d_n9, eq38_e814_d_n10, eq38_e814_d_n11, eq38_e814_d_n12, eq38_e814_d_n13, eq38_e814_d_n14, eq38_e814_d_n15, eq38_e814_d_n16, eq38_e814_d_n17, eq38_e814_d_n18, eq38_e814_d_n19, eq38_e814_d_n20, eq38_e814_d_n21, eq38_e814_d_n22, eq38_e814_d_n23, eq38_e814_d_n24, eq38_e814_d_n25, eq38_e814_d_n26, eq38_e814_d_n27, eq38_e814_d_n28, eq38_e814_d_n29,) = {
    if (!(s.v[466] != 0.0)) {
        let eq38_e807: f64 = self.eval_ddt(12, s.v[209]);
        let eq38_e807_d_n0: f64 = self.ddt_jacobian(s.dn[209][0]);
        let eq38_e807_d_n1: f64 = self.ddt_jacobian(s.dn[209][1]);
        let eq38_e807_d_n2: f64 = self.ddt_jacobian(s.dn[209][2]);
        let eq38_e807_d_n3: f64 = self.ddt_jacobian(s.dn[209][3]);
        let eq38_e807_d_n4: f64 = self.ddt_jacobian(s.dn[209][4]);
        let eq38_e807_d_n5: f64 = self.ddt_jacobian(s.dn[209][5]);
        let eq38_e807_d_n6: f64 = self.ddt_jacobian(s.dn[209][6]);
        let eq38_e807_d_n7: f64 = self.ddt_jacobian(s.dn[209][7]);
        let eq38_e807_d_n8: f64 = self.ddt_jacobian(s.dn[209][8]);
        let eq38_e807_d_n9: f64 = self.ddt_jacobian(s.dn[209][9]);
        let eq38_e807_d_n10: f64 = self.ddt_jacobian(s.dn[209][10]);
        let eq38_e807_d_n11: f64 = self.ddt_jacobian(s.dn[209][11]);
        let eq38_e807_d_n12: f64 = self.ddt_jacobian(s.dn[209][12]);
        let eq38_e807_d_n13: f64 = self.ddt_jacobian(s.dn[209][13]);
        let eq38_e807_d_n14: f64 = self.ddt_jacobian(s.dn[209][14]);
        let eq38_e807_d_n15: f64 = self.ddt_jacobian(s.dn[209][15]);
        let eq38_e807_d_n16: f64 = self.ddt_jacobian(s.dn[209][16]);
        let eq38_e807_d_n17: f64 = self.ddt_jacobian(s.dn[209][17]);
        let eq38_e807_d_n18: f64 = self.ddt_jacobian(s.dn[209][18]);
        let eq38_e807_d_n19: f64 = self.ddt_jacobian(s.dn[209][19]);
        let eq38_e807_d_n20: f64 = self.ddt_jacobian(s.dn[209][20]);
        let eq38_e807_d_n21: f64 = self.ddt_jacobian(s.dn[209][21]);
        let eq38_e807_d_n22: f64 = self.ddt_jacobian(s.dn[209][22]);
        let eq38_e807_d_n23: f64 = self.ddt_jacobian(s.dn[209][23]);
        let eq38_e807_d_n24: f64 = self.ddt_jacobian(s.dn[209][24]);
        let eq38_e807_d_n25: f64 = self.ddt_jacobian(s.dn[209][25]);
        let eq38_e807_d_n26: f64 = self.ddt_jacobian(s.dn[209][26]);
        let eq38_e807_d_n27: f64 = self.ddt_jacobian(s.dn[209][27]);
        let eq38_e807_d_n28: f64 = self.ddt_jacobian(s.dn[209][28]);
        let eq38_e807_d_n29: f64 = self.ddt_jacobian(s.dn[209][29]);
        let eq38_e810: f64 = (p.p355 * (nv2 - nv16));
        let eq38_e810_d_n2: f64 = p.p355;
        let eq38_e810_d_n16: f64 = (-p.p355);
        let eq38_e811: f64 = self.eval_ddt(13, eq38_e810);
        let eq38_e811_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n2: f64 = self.ddt_jacobian(eq38_e810_d_n2);
        let eq38_e811_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n16: f64 = self.ddt_jacobian(eq38_e810_d_n16);
        let eq38_e811_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq38_e811_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq38_e812: f64 = (eq38_e807 + eq38_e811);
        let eq38_e812_d_n0: f64 = (eq38_e807_d_n0 + eq38_e811_d_n0);
        let eq38_e812_d_n1: f64 = (eq38_e807_d_n1 + eq38_e811_d_n1);
        let eq38_e812_d_n2: f64 = (eq38_e807_d_n2 + eq38_e811_d_n2);
        let eq38_e812_d_n3: f64 = (eq38_e807_d_n3 + eq38_e811_d_n3);
        let eq38_e812_d_n4: f64 = (eq38_e807_d_n4 + eq38_e811_d_n4);
        let eq38_e812_d_n5: f64 = (eq38_e807_d_n5 + eq38_e811_d_n5);
        let eq38_e812_d_n6: f64 = (eq38_e807_d_n6 + eq38_e811_d_n6);
        let eq38_e812_d_n7: f64 = (eq38_e807_d_n7 + eq38_e811_d_n7);
        let eq38_e812_d_n8: f64 = (eq38_e807_d_n8 + eq38_e811_d_n8);
        let eq38_e812_d_n9: f64 = (eq38_e807_d_n9 + eq38_e811_d_n9);
        let eq38_e812_d_n10: f64 = (eq38_e807_d_n10 + eq38_e811_d_n10);
        let eq38_e812_d_n11: f64 = (eq38_e807_d_n11 + eq38_e811_d_n11);
        let eq38_e812_d_n12: f64 = (eq38_e807_d_n12 + eq38_e811_d_n12);
        let eq38_e812_d_n13: f64 = (eq38_e807_d_n13 + eq38_e811_d_n13);
        let eq38_e812_d_n14: f64 = (eq38_e807_d_n14 + eq38_e811_d_n14);
        let eq38_e812_d_n15: f64 = (eq38_e807_d_n15 + eq38_e811_d_n15);
        let eq38_e812_d_n16: f64 = (eq38_e807_d_n16 + eq38_e811_d_n16);
        let eq38_e812_d_n17: f64 = (eq38_e807_d_n17 + eq38_e811_d_n17);
        let eq38_e812_d_n18: f64 = (eq38_e807_d_n18 + eq38_e811_d_n18);
        let eq38_e812_d_n19: f64 = (eq38_e807_d_n19 + eq38_e811_d_n19);
        let eq38_e812_d_n20: f64 = (eq38_e807_d_n20 + eq38_e811_d_n20);
        let eq38_e812_d_n21: f64 = (eq38_e807_d_n21 + eq38_e811_d_n21);
        let eq38_e812_d_n22: f64 = (eq38_e807_d_n22 + eq38_e811_d_n22);
        let eq38_e812_d_n23: f64 = (eq38_e807_d_n23 + eq38_e811_d_n23);
        let eq38_e812_d_n24: f64 = (eq38_e807_d_n24 + eq38_e811_d_n24);
        let eq38_e812_d_n25: f64 = (eq38_e807_d_n25 + eq38_e811_d_n25);
        let eq38_e812_d_n26: f64 = (eq38_e807_d_n26 + eq38_e811_d_n26);
        let eq38_e812_d_n27: f64 = (eq38_e807_d_n27 + eq38_e811_d_n27);
        let eq38_e812_d_n28: f64 = (eq38_e807_d_n28 + eq38_e811_d_n28);
        let eq38_e812_d_n29: f64 = (eq38_e807_d_n29 + eq38_e811_d_n29);
        (eq38_e812, eq38_e812_d_n0, eq38_e812_d_n1, eq38_e812_d_n2, eq38_e812_d_n3, eq38_e812_d_n4, eq38_e812_d_n5, eq38_e812_d_n6, eq38_e812_d_n7, eq38_e812_d_n8, eq38_e812_d_n9, eq38_e812_d_n10, eq38_e812_d_n11, eq38_e812_d_n12, eq38_e812_d_n13, eq38_e812_d_n14, eq38_e812_d_n15, eq38_e812_d_n16, eq38_e812_d_n17, eq38_e812_d_n18, eq38_e812_d_n19, eq38_e812_d_n20, eq38_e812_d_n21, eq38_e812_d_n22, eq38_e812_d_n23, eq38_e812_d_n24, eq38_e812_d_n25, eq38_e812_d_n26, eq38_e812_d_n27, eq38_e812_d_n28, eq38_e812_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e814;
        let eq38_node_derivatives: [f64; 30] = [eq38_e814_d_n0, eq38_e814_d_n1, eq38_e814_d_n2, eq38_e814_d_n3, eq38_e814_d_n4, eq38_e814_d_n5, eq38_e814_d_n6, eq38_e814_d_n7, eq38_e814_d_n8, eq38_e814_d_n9, eq38_e814_d_n10, eq38_e814_d_n11, eq38_e814_d_n12, eq38_e814_d_n13, eq38_e814_d_n14, eq38_e814_d_n15, eq38_e814_d_n16, eq38_e814_d_n17, eq38_e814_d_n18, eq38_e814_d_n19, eq38_e814_d_n20, eq38_e814_d_n21, eq38_e814_d_n22, eq38_e814_d_n23, eq38_e814_d_n24, eq38_e814_d_n25, eq38_e814_d_n26, eq38_e814_d_n27, eq38_e814_d_n28, eq38_e814_d_n29];
        let eq38_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[16]),
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq39_e825, eq39_e825_d_n0, eq39_e825_d_n1, eq39_e825_d_n2, eq39_e825_d_n3, eq39_e825_d_n4, eq39_e825_d_n5, eq39_e825_d_n6, eq39_e825_d_n7, eq39_e825_d_n8, eq39_e825_d_n9, eq39_e825_d_n10, eq39_e825_d_n11, eq39_e825_d_n12, eq39_e825_d_n13, eq39_e825_d_n14, eq39_e825_d_n15, eq39_e825_d_n16, eq39_e825_d_n17, eq39_e825_d_n18, eq39_e825_d_n19, eq39_e825_d_n20, eq39_e825_d_n21, eq39_e825_d_n22, eq39_e825_d_n23, eq39_e825_d_n24, eq39_e825_d_n25, eq39_e825_d_n26, eq39_e825_d_n27, eq39_e825_d_n28, eq39_e825_d_n29,) = {
    if (!(s.v[466] != 0.0)) {
        let eq39_e818: f64 = self.eval_ddt(14, s.v[210]);
        let eq39_e818_d_n0: f64 = self.ddt_jacobian(s.dn[210][0]);
        let eq39_e818_d_n1: f64 = self.ddt_jacobian(s.dn[210][1]);
        let eq39_e818_d_n2: f64 = self.ddt_jacobian(s.dn[210][2]);
        let eq39_e818_d_n3: f64 = self.ddt_jacobian(s.dn[210][3]);
        let eq39_e818_d_n4: f64 = self.ddt_jacobian(s.dn[210][4]);
        let eq39_e818_d_n5: f64 = self.ddt_jacobian(s.dn[210][5]);
        let eq39_e818_d_n6: f64 = self.ddt_jacobian(s.dn[210][6]);
        let eq39_e818_d_n7: f64 = self.ddt_jacobian(s.dn[210][7]);
        let eq39_e818_d_n8: f64 = self.ddt_jacobian(s.dn[210][8]);
        let eq39_e818_d_n9: f64 = self.ddt_jacobian(s.dn[210][9]);
        let eq39_e818_d_n10: f64 = self.ddt_jacobian(s.dn[210][10]);
        let eq39_e818_d_n11: f64 = self.ddt_jacobian(s.dn[210][11]);
        let eq39_e818_d_n12: f64 = self.ddt_jacobian(s.dn[210][12]);
        let eq39_e818_d_n13: f64 = self.ddt_jacobian(s.dn[210][13]);
        let eq39_e818_d_n14: f64 = self.ddt_jacobian(s.dn[210][14]);
        let eq39_e818_d_n15: f64 = self.ddt_jacobian(s.dn[210][15]);
        let eq39_e818_d_n16: f64 = self.ddt_jacobian(s.dn[210][16]);
        let eq39_e818_d_n17: f64 = self.ddt_jacobian(s.dn[210][17]);
        let eq39_e818_d_n18: f64 = self.ddt_jacobian(s.dn[210][18]);
        let eq39_e818_d_n19: f64 = self.ddt_jacobian(s.dn[210][19]);
        let eq39_e818_d_n20: f64 = self.ddt_jacobian(s.dn[210][20]);
        let eq39_e818_d_n21: f64 = self.ddt_jacobian(s.dn[210][21]);
        let eq39_e818_d_n22: f64 = self.ddt_jacobian(s.dn[210][22]);
        let eq39_e818_d_n23: f64 = self.ddt_jacobian(s.dn[210][23]);
        let eq39_e818_d_n24: f64 = self.ddt_jacobian(s.dn[210][24]);
        let eq39_e818_d_n25: f64 = self.ddt_jacobian(s.dn[210][25]);
        let eq39_e818_d_n26: f64 = self.ddt_jacobian(s.dn[210][26]);
        let eq39_e818_d_n27: f64 = self.ddt_jacobian(s.dn[210][27]);
        let eq39_e818_d_n28: f64 = self.ddt_jacobian(s.dn[210][28]);
        let eq39_e818_d_n29: f64 = self.ddt_jacobian(s.dn[210][29]);
        let eq39_e821: f64 = (p.p355 * (nv2 - nv17));
        let eq39_e821_d_n2: f64 = p.p355;
        let eq39_e821_d_n17: f64 = (-p.p355);
        let eq39_e822: f64 = self.eval_ddt(15, eq39_e821);
        let eq39_e822_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n2: f64 = self.ddt_jacobian(eq39_e821_d_n2);
        let eq39_e822_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n17: f64 = self.ddt_jacobian(eq39_e821_d_n17);
        let eq39_e822_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq39_e822_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq39_e823: f64 = (eq39_e818 + eq39_e822);
        let eq39_e823_d_n0: f64 = (eq39_e818_d_n0 + eq39_e822_d_n0);
        let eq39_e823_d_n1: f64 = (eq39_e818_d_n1 + eq39_e822_d_n1);
        let eq39_e823_d_n2: f64 = (eq39_e818_d_n2 + eq39_e822_d_n2);
        let eq39_e823_d_n3: f64 = (eq39_e818_d_n3 + eq39_e822_d_n3);
        let eq39_e823_d_n4: f64 = (eq39_e818_d_n4 + eq39_e822_d_n4);
        let eq39_e823_d_n5: f64 = (eq39_e818_d_n5 + eq39_e822_d_n5);
        let eq39_e823_d_n6: f64 = (eq39_e818_d_n6 + eq39_e822_d_n6);
        let eq39_e823_d_n7: f64 = (eq39_e818_d_n7 + eq39_e822_d_n7);
        let eq39_e823_d_n8: f64 = (eq39_e818_d_n8 + eq39_e822_d_n8);
        let eq39_e823_d_n9: f64 = (eq39_e818_d_n9 + eq39_e822_d_n9);
        let eq39_e823_d_n10: f64 = (eq39_e818_d_n10 + eq39_e822_d_n10);
        let eq39_e823_d_n11: f64 = (eq39_e818_d_n11 + eq39_e822_d_n11);
        let eq39_e823_d_n12: f64 = (eq39_e818_d_n12 + eq39_e822_d_n12);
        let eq39_e823_d_n13: f64 = (eq39_e818_d_n13 + eq39_e822_d_n13);
        let eq39_e823_d_n14: f64 = (eq39_e818_d_n14 + eq39_e822_d_n14);
        let eq39_e823_d_n15: f64 = (eq39_e818_d_n15 + eq39_e822_d_n15);
        let eq39_e823_d_n16: f64 = (eq39_e818_d_n16 + eq39_e822_d_n16);
        let eq39_e823_d_n17: f64 = (eq39_e818_d_n17 + eq39_e822_d_n17);
        let eq39_e823_d_n18: f64 = (eq39_e818_d_n18 + eq39_e822_d_n18);
        let eq39_e823_d_n19: f64 = (eq39_e818_d_n19 + eq39_e822_d_n19);
        let eq39_e823_d_n20: f64 = (eq39_e818_d_n20 + eq39_e822_d_n20);
        let eq39_e823_d_n21: f64 = (eq39_e818_d_n21 + eq39_e822_d_n21);
        let eq39_e823_d_n22: f64 = (eq39_e818_d_n22 + eq39_e822_d_n22);
        let eq39_e823_d_n23: f64 = (eq39_e818_d_n23 + eq39_e822_d_n23);
        let eq39_e823_d_n24: f64 = (eq39_e818_d_n24 + eq39_e822_d_n24);
        let eq39_e823_d_n25: f64 = (eq39_e818_d_n25 + eq39_e822_d_n25);
        let eq39_e823_d_n26: f64 = (eq39_e818_d_n26 + eq39_e822_d_n26);
        let eq39_e823_d_n27: f64 = (eq39_e818_d_n27 + eq39_e822_d_n27);
        let eq39_e823_d_n28: f64 = (eq39_e818_d_n28 + eq39_e822_d_n28);
        let eq39_e823_d_n29: f64 = (eq39_e818_d_n29 + eq39_e822_d_n29);
        (eq39_e823, eq39_e823_d_n0, eq39_e823_d_n1, eq39_e823_d_n2, eq39_e823_d_n3, eq39_e823_d_n4, eq39_e823_d_n5, eq39_e823_d_n6, eq39_e823_d_n7, eq39_e823_d_n8, eq39_e823_d_n9, eq39_e823_d_n10, eq39_e823_d_n11, eq39_e823_d_n12, eq39_e823_d_n13, eq39_e823_d_n14, eq39_e823_d_n15, eq39_e823_d_n16, eq39_e823_d_n17, eq39_e823_d_n18, eq39_e823_d_n19, eq39_e823_d_n20, eq39_e823_d_n21, eq39_e823_d_n22, eq39_e823_d_n23, eq39_e823_d_n24, eq39_e823_d_n25, eq39_e823_d_n26, eq39_e823_d_n27, eq39_e823_d_n28, eq39_e823_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e825;
        let eq39_node_derivatives: [f64; 30] = [eq39_e825_d_n0, eq39_e825_d_n1, eq39_e825_d_n2, eq39_e825_d_n3, eq39_e825_d_n4, eq39_e825_d_n5, eq39_e825_d_n6, eq39_e825_d_n7, eq39_e825_d_n8, eq39_e825_d_n9, eq39_e825_d_n10, eq39_e825_d_n11, eq39_e825_d_n12, eq39_e825_d_n13, eq39_e825_d_n14, eq39_e825_d_n15, eq39_e825_d_n16, eq39_e825_d_n17, eq39_e825_d_n18, eq39_e825_d_n19, eq39_e825_d_n20, eq39_e825_d_n21, eq39_e825_d_n22, eq39_e825_d_n23, eq39_e825_d_n24, eq39_e825_d_n25, eq39_e825_d_n26, eq39_e825_d_n27, eq39_e825_d_n28, eq39_e825_d_n29];
        let eq39_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[17]),
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq40_e836, eq40_e836_d_n0, eq40_e836_d_n1, eq40_e836_d_n2, eq40_e836_d_n3, eq40_e836_d_n4, eq40_e836_d_n5, eq40_e836_d_n6, eq40_e836_d_n7, eq40_e836_d_n8, eq40_e836_d_n9, eq40_e836_d_n10, eq40_e836_d_n11, eq40_e836_d_n12, eq40_e836_d_n13, eq40_e836_d_n14, eq40_e836_d_n15, eq40_e836_d_n16, eq40_e836_d_n17, eq40_e836_d_n18, eq40_e836_d_n19, eq40_e836_d_n20, eq40_e836_d_n21, eq40_e836_d_n22, eq40_e836_d_n23, eq40_e836_d_n24, eq40_e836_d_n25, eq40_e836_d_n26, eq40_e836_d_n27, eq40_e836_d_n28, eq40_e836_d_n29,) = {
    if (!(s.v[466] != 0.0)) {
        let eq40_e829: f64 = self.eval_ddt(16, s.v[211]);
        let eq40_e829_d_n0: f64 = self.ddt_jacobian(s.dn[211][0]);
        let eq40_e829_d_n1: f64 = self.ddt_jacobian(s.dn[211][1]);
        let eq40_e829_d_n2: f64 = self.ddt_jacobian(s.dn[211][2]);
        let eq40_e829_d_n3: f64 = self.ddt_jacobian(s.dn[211][3]);
        let eq40_e829_d_n4: f64 = self.ddt_jacobian(s.dn[211][4]);
        let eq40_e829_d_n5: f64 = self.ddt_jacobian(s.dn[211][5]);
        let eq40_e829_d_n6: f64 = self.ddt_jacobian(s.dn[211][6]);
        let eq40_e829_d_n7: f64 = self.ddt_jacobian(s.dn[211][7]);
        let eq40_e829_d_n8: f64 = self.ddt_jacobian(s.dn[211][8]);
        let eq40_e829_d_n9: f64 = self.ddt_jacobian(s.dn[211][9]);
        let eq40_e829_d_n10: f64 = self.ddt_jacobian(s.dn[211][10]);
        let eq40_e829_d_n11: f64 = self.ddt_jacobian(s.dn[211][11]);
        let eq40_e829_d_n12: f64 = self.ddt_jacobian(s.dn[211][12]);
        let eq40_e829_d_n13: f64 = self.ddt_jacobian(s.dn[211][13]);
        let eq40_e829_d_n14: f64 = self.ddt_jacobian(s.dn[211][14]);
        let eq40_e829_d_n15: f64 = self.ddt_jacobian(s.dn[211][15]);
        let eq40_e829_d_n16: f64 = self.ddt_jacobian(s.dn[211][16]);
        let eq40_e829_d_n17: f64 = self.ddt_jacobian(s.dn[211][17]);
        let eq40_e829_d_n18: f64 = self.ddt_jacobian(s.dn[211][18]);
        let eq40_e829_d_n19: f64 = self.ddt_jacobian(s.dn[211][19]);
        let eq40_e829_d_n20: f64 = self.ddt_jacobian(s.dn[211][20]);
        let eq40_e829_d_n21: f64 = self.ddt_jacobian(s.dn[211][21]);
        let eq40_e829_d_n22: f64 = self.ddt_jacobian(s.dn[211][22]);
        let eq40_e829_d_n23: f64 = self.ddt_jacobian(s.dn[211][23]);
        let eq40_e829_d_n24: f64 = self.ddt_jacobian(s.dn[211][24]);
        let eq40_e829_d_n25: f64 = self.ddt_jacobian(s.dn[211][25]);
        let eq40_e829_d_n26: f64 = self.ddt_jacobian(s.dn[211][26]);
        let eq40_e829_d_n27: f64 = self.ddt_jacobian(s.dn[211][27]);
        let eq40_e829_d_n28: f64 = self.ddt_jacobian(s.dn[211][28]);
        let eq40_e829_d_n29: f64 = self.ddt_jacobian(s.dn[211][29]);
        let eq40_e832: f64 = (p.p355 * (nv7 - nv16));
        let eq40_e832_d_n7: f64 = p.p355;
        let eq40_e832_d_n16: f64 = (-p.p355);
        let eq40_e833: f64 = self.eval_ddt(17, eq40_e832);
        let eq40_e833_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n7: f64 = self.ddt_jacobian(eq40_e832_d_n7);
        let eq40_e833_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n16: f64 = self.ddt_jacobian(eq40_e832_d_n16);
        let eq40_e833_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq40_e833_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq40_e834: f64 = (eq40_e829 + eq40_e833);
        let eq40_e834_d_n0: f64 = (eq40_e829_d_n0 + eq40_e833_d_n0);
        let eq40_e834_d_n1: f64 = (eq40_e829_d_n1 + eq40_e833_d_n1);
        let eq40_e834_d_n2: f64 = (eq40_e829_d_n2 + eq40_e833_d_n2);
        let eq40_e834_d_n3: f64 = (eq40_e829_d_n3 + eq40_e833_d_n3);
        let eq40_e834_d_n4: f64 = (eq40_e829_d_n4 + eq40_e833_d_n4);
        let eq40_e834_d_n5: f64 = (eq40_e829_d_n5 + eq40_e833_d_n5);
        let eq40_e834_d_n6: f64 = (eq40_e829_d_n6 + eq40_e833_d_n6);
        let eq40_e834_d_n7: f64 = (eq40_e829_d_n7 + eq40_e833_d_n7);
        let eq40_e834_d_n8: f64 = (eq40_e829_d_n8 + eq40_e833_d_n8);
        let eq40_e834_d_n9: f64 = (eq40_e829_d_n9 + eq40_e833_d_n9);
        let eq40_e834_d_n10: f64 = (eq40_e829_d_n10 + eq40_e833_d_n10);
        let eq40_e834_d_n11: f64 = (eq40_e829_d_n11 + eq40_e833_d_n11);
        let eq40_e834_d_n12: f64 = (eq40_e829_d_n12 + eq40_e833_d_n12);
        let eq40_e834_d_n13: f64 = (eq40_e829_d_n13 + eq40_e833_d_n13);
        let eq40_e834_d_n14: f64 = (eq40_e829_d_n14 + eq40_e833_d_n14);
        let eq40_e834_d_n15: f64 = (eq40_e829_d_n15 + eq40_e833_d_n15);
        let eq40_e834_d_n16: f64 = (eq40_e829_d_n16 + eq40_e833_d_n16);
        let eq40_e834_d_n17: f64 = (eq40_e829_d_n17 + eq40_e833_d_n17);
        let eq40_e834_d_n18: f64 = (eq40_e829_d_n18 + eq40_e833_d_n18);
        let eq40_e834_d_n19: f64 = (eq40_e829_d_n19 + eq40_e833_d_n19);
        let eq40_e834_d_n20: f64 = (eq40_e829_d_n20 + eq40_e833_d_n20);
        let eq40_e834_d_n21: f64 = (eq40_e829_d_n21 + eq40_e833_d_n21);
        let eq40_e834_d_n22: f64 = (eq40_e829_d_n22 + eq40_e833_d_n22);
        let eq40_e834_d_n23: f64 = (eq40_e829_d_n23 + eq40_e833_d_n23);
        let eq40_e834_d_n24: f64 = (eq40_e829_d_n24 + eq40_e833_d_n24);
        let eq40_e834_d_n25: f64 = (eq40_e829_d_n25 + eq40_e833_d_n25);
        let eq40_e834_d_n26: f64 = (eq40_e829_d_n26 + eq40_e833_d_n26);
        let eq40_e834_d_n27: f64 = (eq40_e829_d_n27 + eq40_e833_d_n27);
        let eq40_e834_d_n28: f64 = (eq40_e829_d_n28 + eq40_e833_d_n28);
        let eq40_e834_d_n29: f64 = (eq40_e829_d_n29 + eq40_e833_d_n29);
        (eq40_e834, eq40_e834_d_n0, eq40_e834_d_n1, eq40_e834_d_n2, eq40_e834_d_n3, eq40_e834_d_n4, eq40_e834_d_n5, eq40_e834_d_n6, eq40_e834_d_n7, eq40_e834_d_n8, eq40_e834_d_n9, eq40_e834_d_n10, eq40_e834_d_n11, eq40_e834_d_n12, eq40_e834_d_n13, eq40_e834_d_n14, eq40_e834_d_n15, eq40_e834_d_n16, eq40_e834_d_n17, eq40_e834_d_n18, eq40_e834_d_n19, eq40_e834_d_n20, eq40_e834_d_n21, eq40_e834_d_n22, eq40_e834_d_n23, eq40_e834_d_n24, eq40_e834_d_n25, eq40_e834_d_n26, eq40_e834_d_n27, eq40_e834_d_n28, eq40_e834_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_value: f64 = eq40_e836;
        let eq40_node_derivatives: [f64; 30] = [eq40_e836_d_n0, eq40_e836_d_n1, eq40_e836_d_n2, eq40_e836_d_n3, eq40_e836_d_n4, eq40_e836_d_n5, eq40_e836_d_n6, eq40_e836_d_n7, eq40_e836_d_n8, eq40_e836_d_n9, eq40_e836_d_n10, eq40_e836_d_n11, eq40_e836_d_n12, eq40_e836_d_n13, eq40_e836_d_n14, eq40_e836_d_n15, eq40_e836_d_n16, eq40_e836_d_n17, eq40_e836_d_n18, eq40_e836_d_n19, eq40_e836_d_n20, eq40_e836_d_n21, eq40_e836_d_n22, eq40_e836_d_n23, eq40_e836_d_n24, eq40_e836_d_n25, eq40_e836_d_n26, eq40_e836_d_n27, eq40_e836_d_n28, eq40_e836_d_n29];
        let eq40_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[16]),
            self.multiplicity * (eq40_value),
            &nodes,
            &eq40_node_derivatives,
            &branches,
            &eq40_branch_derivatives,
            self.multiplicity,
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
        let (eq41_e841,) = {
    if (!(s.v[466] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq41_value: f64 = eq41_e841;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[17]),
            self.multiplicity * (eq41_value),
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
        let (eq42_e846,) = {
    if (!(s.v[466] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq42_value: f64 = eq42_e846;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[9]),
            self.multiplicity * (eq42_value),
            &[
            ],
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let eq43_e848: f64 = self.eval_ddt(18, s.v[212]);
        let eq43_e848_d_n0: f64 = self.ddt_jacobian(s.dn[212][0]);
        let eq43_e848_d_n1: f64 = self.ddt_jacobian(s.dn[212][1]);
        let eq43_e848_d_n2: f64 = self.ddt_jacobian(s.dn[212][2]);
        let eq43_e848_d_n3: f64 = self.ddt_jacobian(s.dn[212][3]);
        let eq43_e848_d_n4: f64 = self.ddt_jacobian(s.dn[212][4]);
        let eq43_e848_d_n5: f64 = self.ddt_jacobian(s.dn[212][5]);
        let eq43_e848_d_n6: f64 = self.ddt_jacobian(s.dn[212][6]);
        let eq43_e848_d_n7: f64 = self.ddt_jacobian(s.dn[212][7]);
        let eq43_e848_d_n8: f64 = self.ddt_jacobian(s.dn[212][8]);
        let eq43_e848_d_n9: f64 = self.ddt_jacobian(s.dn[212][9]);
        let eq43_e848_d_n10: f64 = self.ddt_jacobian(s.dn[212][10]);
        let eq43_e848_d_n11: f64 = self.ddt_jacobian(s.dn[212][11]);
        let eq43_e848_d_n12: f64 = self.ddt_jacobian(s.dn[212][12]);
        let eq43_e848_d_n13: f64 = self.ddt_jacobian(s.dn[212][13]);
        let eq43_e848_d_n14: f64 = self.ddt_jacobian(s.dn[212][14]);
        let eq43_e848_d_n15: f64 = self.ddt_jacobian(s.dn[212][15]);
        let eq43_e848_d_n16: f64 = self.ddt_jacobian(s.dn[212][16]);
        let eq43_e848_d_n17: f64 = self.ddt_jacobian(s.dn[212][17]);
        let eq43_e848_d_n18: f64 = self.ddt_jacobian(s.dn[212][18]);
        let eq43_e848_d_n19: f64 = self.ddt_jacobian(s.dn[212][19]);
        let eq43_e848_d_n20: f64 = self.ddt_jacobian(s.dn[212][20]);
        let eq43_e848_d_n21: f64 = self.ddt_jacobian(s.dn[212][21]);
        let eq43_e848_d_n22: f64 = self.ddt_jacobian(s.dn[212][22]);
        let eq43_e848_d_n23: f64 = self.ddt_jacobian(s.dn[212][23]);
        let eq43_e848_d_n24: f64 = self.ddt_jacobian(s.dn[212][24]);
        let eq43_e848_d_n25: f64 = self.ddt_jacobian(s.dn[212][25]);
        let eq43_e848_d_n26: f64 = self.ddt_jacobian(s.dn[212][26]);
        let eq43_e848_d_n27: f64 = self.ddt_jacobian(s.dn[212][27]);
        let eq43_e848_d_n28: f64 = self.ddt_jacobian(s.dn[212][28]);
        let eq43_e848_d_n29: f64 = self.ddt_jacobian(s.dn[212][29]);
        let eq43_e851: f64 = (p.p355 * (nv3 - nv16));
        let eq43_e851_d_n3: f64 = p.p355;
        let eq43_e851_d_n16: f64 = (-p.p355);
        let eq43_e852: f64 = self.eval_ddt(19, eq43_e851);
        let eq43_e852_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n3: f64 = self.ddt_jacobian(eq43_e851_d_n3);
        let eq43_e852_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n16: f64 = self.ddt_jacobian(eq43_e851_d_n16);
        let eq43_e852_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq43_e852_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq43_e853: f64 = (eq43_e848 + eq43_e852);
        let eq43_e853_d_n0: f64 = (eq43_e848_d_n0 + eq43_e852_d_n0);
        let eq43_e853_d_n1: f64 = (eq43_e848_d_n1 + eq43_e852_d_n1);
        let eq43_e853_d_n2: f64 = (eq43_e848_d_n2 + eq43_e852_d_n2);
        let eq43_e853_d_n3: f64 = (eq43_e848_d_n3 + eq43_e852_d_n3);
        let eq43_e853_d_n4: f64 = (eq43_e848_d_n4 + eq43_e852_d_n4);
        let eq43_e853_d_n5: f64 = (eq43_e848_d_n5 + eq43_e852_d_n5);
        let eq43_e853_d_n6: f64 = (eq43_e848_d_n6 + eq43_e852_d_n6);
        let eq43_e853_d_n7: f64 = (eq43_e848_d_n7 + eq43_e852_d_n7);
        let eq43_e853_d_n8: f64 = (eq43_e848_d_n8 + eq43_e852_d_n8);
        let eq43_e853_d_n9: f64 = (eq43_e848_d_n9 + eq43_e852_d_n9);
        let eq43_e853_d_n10: f64 = (eq43_e848_d_n10 + eq43_e852_d_n10);
        let eq43_e853_d_n11: f64 = (eq43_e848_d_n11 + eq43_e852_d_n11);
        let eq43_e853_d_n12: f64 = (eq43_e848_d_n12 + eq43_e852_d_n12);
        let eq43_e853_d_n13: f64 = (eq43_e848_d_n13 + eq43_e852_d_n13);
        let eq43_e853_d_n14: f64 = (eq43_e848_d_n14 + eq43_e852_d_n14);
        let eq43_e853_d_n15: f64 = (eq43_e848_d_n15 + eq43_e852_d_n15);
        let eq43_e853_d_n16: f64 = (eq43_e848_d_n16 + eq43_e852_d_n16);
        let eq43_e853_d_n17: f64 = (eq43_e848_d_n17 + eq43_e852_d_n17);
        let eq43_e853_d_n18: f64 = (eq43_e848_d_n18 + eq43_e852_d_n18);
        let eq43_e853_d_n19: f64 = (eq43_e848_d_n19 + eq43_e852_d_n19);
        let eq43_e853_d_n20: f64 = (eq43_e848_d_n20 + eq43_e852_d_n20);
        let eq43_e853_d_n21: f64 = (eq43_e848_d_n21 + eq43_e852_d_n21);
        let eq43_e853_d_n22: f64 = (eq43_e848_d_n22 + eq43_e852_d_n22);
        let eq43_e853_d_n23: f64 = (eq43_e848_d_n23 + eq43_e852_d_n23);
        let eq43_e853_d_n24: f64 = (eq43_e848_d_n24 + eq43_e852_d_n24);
        let eq43_e853_d_n25: f64 = (eq43_e848_d_n25 + eq43_e852_d_n25);
        let eq43_e853_d_n26: f64 = (eq43_e848_d_n26 + eq43_e852_d_n26);
        let eq43_e853_d_n27: f64 = (eq43_e848_d_n27 + eq43_e852_d_n27);
        let eq43_e853_d_n28: f64 = (eq43_e848_d_n28 + eq43_e852_d_n28);
        let eq43_e853_d_n29: f64 = (eq43_e848_d_n29 + eq43_e852_d_n29);
        let eq43_value: f64 = eq43_e853;
        let eq43_node_derivatives: [f64; 30] = [eq43_e853_d_n0, eq43_e853_d_n1, eq43_e853_d_n2, eq43_e853_d_n3, eq43_e853_d_n4, eq43_e853_d_n5, eq43_e853_d_n6, eq43_e853_d_n7, eq43_e853_d_n8, eq43_e853_d_n9, eq43_e853_d_n10, eq43_e853_d_n11, eq43_e853_d_n12, eq43_e853_d_n13, eq43_e853_d_n14, eq43_e853_d_n15, eq43_e853_d_n16, eq43_e853_d_n17, eq43_e853_d_n18, eq43_e853_d_n19, eq43_e853_d_n20, eq43_e853_d_n21, eq43_e853_d_n22, eq43_e853_d_n23, eq43_e853_d_n24, eq43_e853_d_n25, eq43_e853_d_n26, eq43_e853_d_n27, eq43_e853_d_n28, eq43_e853_d_n29];
        let eq43_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[16]),
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq44_e861, eq44_e861_d_n0, eq44_e861_d_n1, eq44_e861_d_n2, eq44_e861_d_n3, eq44_e861_d_n4, eq44_e861_d_n5, eq44_e861_d_n6, eq44_e861_d_n7, eq44_e861_d_n8, eq44_e861_d_n9, eq44_e861_d_n10, eq44_e861_d_n11, eq44_e861_d_n12, eq44_e861_d_n13, eq44_e861_d_n14, eq44_e861_d_n15, eq44_e861_d_n16, eq44_e861_d_n17, eq44_e861_d_n18, eq44_e861_d_n19, eq44_e861_d_n20, eq44_e861_d_n21, eq44_e861_d_n22, eq44_e861_d_n23, eq44_e861_d_n24, eq44_e861_d_n25, eq44_e861_d_n26, eq44_e861_d_n27, eq44_e861_d_n28, eq44_e861_d_n29,) = {
    if (s.v[467] != 0.0) {
        let eq44_e858: f64 = (s.v[0] * (nv16 - nv15));
        let eq44_e858_d_n15: f64 = (-s.v[0]);
        let eq44_e858_d_n16: f64 = s.v[0];
        let eq44_e859: f64 = (s.v[202] + eq44_e858);
        let eq44_e859_d_n15: f64 = (s.dn[202][15] + eq44_e858_d_n15);
        let eq44_e859_d_n16: f64 = (s.dn[202][16] + eq44_e858_d_n16);
        (eq44_e859, s.dn[202][0], s.dn[202][1], s.dn[202][2], s.dn[202][3], s.dn[202][4], s.dn[202][5], s.dn[202][6], s.dn[202][7], s.dn[202][8], s.dn[202][9], s.dn[202][10], s.dn[202][11], s.dn[202][12], s.dn[202][13], s.dn[202][14], eq44_e859_d_n15, eq44_e859_d_n16, s.dn[202][17], s.dn[202][18], s.dn[202][19], s.dn[202][20], s.dn[202][21], s.dn[202][22], s.dn[202][23], s.dn[202][24], s.dn[202][25], s.dn[202][26], s.dn[202][27], s.dn[202][28], s.dn[202][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e861;
        let eq44_node_derivatives: [f64; 30] = [eq44_e861_d_n0, eq44_e861_d_n1, eq44_e861_d_n2, eq44_e861_d_n3, eq44_e861_d_n4, eq44_e861_d_n5, eq44_e861_d_n6, eq44_e861_d_n7, eq44_e861_d_n8, eq44_e861_d_n9, eq44_e861_d_n10, eq44_e861_d_n11, eq44_e861_d_n12, eq44_e861_d_n13, eq44_e861_d_n14, eq44_e861_d_n15, eq44_e861_d_n16, eq44_e861_d_n17, eq44_e861_d_n18, eq44_e861_d_n19, eq44_e861_d_n20, eq44_e861_d_n21, eq44_e861_d_n22, eq44_e861_d_n23, eq44_e861_d_n24, eq44_e861_d_n25, eq44_e861_d_n26, eq44_e861_d_n27, eq44_e861_d_n28, eq44_e861_d_n29];
        let eq44_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[16]),
            Some(nodes[15]),
            self.multiplicity * (eq44_value),
            &nodes,
            &eq44_node_derivatives,
            &branches,
            &eq44_branch_derivatives,
            self.multiplicity,
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
        let (eq45_e866,) = {
    if (!(s.v[467] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq45_value: f64 = eq45_e866;
        stamper.stamp_potential(
            branches[19],
            eq45_value,
            &[
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq46_e876, eq46_e876_d_n0, eq46_e876_d_n1, eq46_e876_d_n2, eq46_e876_d_n3, eq46_e876_d_n4, eq46_e876_d_n5, eq46_e876_d_n6, eq46_e876_d_n7, eq46_e876_d_n8, eq46_e876_d_n9, eq46_e876_d_n10, eq46_e876_d_n11, eq46_e876_d_n12, eq46_e876_d_n13, eq46_e876_d_n14, eq46_e876_d_n15, eq46_e876_d_n16, eq46_e876_d_n17, eq46_e876_d_n18, eq46_e876_d_n19, eq46_e876_d_n20, eq46_e876_d_n21, eq46_e876_d_n22, eq46_e876_d_n23, eq46_e876_d_n24, eq46_e876_d_n25, eq46_e876_d_n26, eq46_e876_d_n27, eq46_e876_d_n28, eq46_e876_d_n29,) = {
    if (s.v[613] != 0.0) {
        let eq46_e869: f64 = self.eval_ddt(20, s.v[203]);
        let eq46_e869_d_n0: f64 = self.ddt_jacobian(s.dn[203][0]);
        let eq46_e869_d_n1: f64 = self.ddt_jacobian(s.dn[203][1]);
        let eq46_e869_d_n2: f64 = self.ddt_jacobian(s.dn[203][2]);
        let eq46_e869_d_n3: f64 = self.ddt_jacobian(s.dn[203][3]);
        let eq46_e869_d_n4: f64 = self.ddt_jacobian(s.dn[203][4]);
        let eq46_e869_d_n5: f64 = self.ddt_jacobian(s.dn[203][5]);
        let eq46_e869_d_n6: f64 = self.ddt_jacobian(s.dn[203][6]);
        let eq46_e869_d_n7: f64 = self.ddt_jacobian(s.dn[203][7]);
        let eq46_e869_d_n8: f64 = self.ddt_jacobian(s.dn[203][8]);
        let eq46_e869_d_n9: f64 = self.ddt_jacobian(s.dn[203][9]);
        let eq46_e869_d_n10: f64 = self.ddt_jacobian(s.dn[203][10]);
        let eq46_e869_d_n11: f64 = self.ddt_jacobian(s.dn[203][11]);
        let eq46_e869_d_n12: f64 = self.ddt_jacobian(s.dn[203][12]);
        let eq46_e869_d_n13: f64 = self.ddt_jacobian(s.dn[203][13]);
        let eq46_e869_d_n14: f64 = self.ddt_jacobian(s.dn[203][14]);
        let eq46_e869_d_n15: f64 = self.ddt_jacobian(s.dn[203][15]);
        let eq46_e869_d_n16: f64 = self.ddt_jacobian(s.dn[203][16]);
        let eq46_e869_d_n17: f64 = self.ddt_jacobian(s.dn[203][17]);
        let eq46_e869_d_n18: f64 = self.ddt_jacobian(s.dn[203][18]);
        let eq46_e869_d_n19: f64 = self.ddt_jacobian(s.dn[203][19]);
        let eq46_e869_d_n20: f64 = self.ddt_jacobian(s.dn[203][20]);
        let eq46_e869_d_n21: f64 = self.ddt_jacobian(s.dn[203][21]);
        let eq46_e869_d_n22: f64 = self.ddt_jacobian(s.dn[203][22]);
        let eq46_e869_d_n23: f64 = self.ddt_jacobian(s.dn[203][23]);
        let eq46_e869_d_n24: f64 = self.ddt_jacobian(s.dn[203][24]);
        let eq46_e869_d_n25: f64 = self.ddt_jacobian(s.dn[203][25]);
        let eq46_e869_d_n26: f64 = self.ddt_jacobian(s.dn[203][26]);
        let eq46_e869_d_n27: f64 = self.ddt_jacobian(s.dn[203][27]);
        let eq46_e869_d_n28: f64 = self.ddt_jacobian(s.dn[203][28]);
        let eq46_e869_d_n29: f64 = self.ddt_jacobian(s.dn[203][29]);
        let eq46_e872: f64 = (p.p355 * (nv7 - nv15));
        let eq46_e872_d_n7: f64 = p.p355;
        let eq46_e872_d_n15: f64 = (-p.p355);
        let eq46_e873: f64 = self.eval_ddt(21, eq46_e872);
        let eq46_e873_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n7: f64 = self.ddt_jacobian(eq46_e872_d_n7);
        let eq46_e873_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n15: f64 = self.ddt_jacobian(eq46_e872_d_n15);
        let eq46_e873_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq46_e873_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq46_e874: f64 = (eq46_e869 + eq46_e873);
        let eq46_e874_d_n0: f64 = (eq46_e869_d_n0 + eq46_e873_d_n0);
        let eq46_e874_d_n1: f64 = (eq46_e869_d_n1 + eq46_e873_d_n1);
        let eq46_e874_d_n2: f64 = (eq46_e869_d_n2 + eq46_e873_d_n2);
        let eq46_e874_d_n3: f64 = (eq46_e869_d_n3 + eq46_e873_d_n3);
        let eq46_e874_d_n4: f64 = (eq46_e869_d_n4 + eq46_e873_d_n4);
        let eq46_e874_d_n5: f64 = (eq46_e869_d_n5 + eq46_e873_d_n5);
        let eq46_e874_d_n6: f64 = (eq46_e869_d_n6 + eq46_e873_d_n6);
        let eq46_e874_d_n7: f64 = (eq46_e869_d_n7 + eq46_e873_d_n7);
        let eq46_e874_d_n8: f64 = (eq46_e869_d_n8 + eq46_e873_d_n8);
        let eq46_e874_d_n9: f64 = (eq46_e869_d_n9 + eq46_e873_d_n9);
        let eq46_e874_d_n10: f64 = (eq46_e869_d_n10 + eq46_e873_d_n10);
        let eq46_e874_d_n11: f64 = (eq46_e869_d_n11 + eq46_e873_d_n11);
        let eq46_e874_d_n12: f64 = (eq46_e869_d_n12 + eq46_e873_d_n12);
        let eq46_e874_d_n13: f64 = (eq46_e869_d_n13 + eq46_e873_d_n13);
        let eq46_e874_d_n14: f64 = (eq46_e869_d_n14 + eq46_e873_d_n14);
        let eq46_e874_d_n15: f64 = (eq46_e869_d_n15 + eq46_e873_d_n15);
        let eq46_e874_d_n16: f64 = (eq46_e869_d_n16 + eq46_e873_d_n16);
        let eq46_e874_d_n17: f64 = (eq46_e869_d_n17 + eq46_e873_d_n17);
        let eq46_e874_d_n18: f64 = (eq46_e869_d_n18 + eq46_e873_d_n18);
        let eq46_e874_d_n19: f64 = (eq46_e869_d_n19 + eq46_e873_d_n19);
        let eq46_e874_d_n20: f64 = (eq46_e869_d_n20 + eq46_e873_d_n20);
        let eq46_e874_d_n21: f64 = (eq46_e869_d_n21 + eq46_e873_d_n21);
        let eq46_e874_d_n22: f64 = (eq46_e869_d_n22 + eq46_e873_d_n22);
        let eq46_e874_d_n23: f64 = (eq46_e869_d_n23 + eq46_e873_d_n23);
        let eq46_e874_d_n24: f64 = (eq46_e869_d_n24 + eq46_e873_d_n24);
        let eq46_e874_d_n25: f64 = (eq46_e869_d_n25 + eq46_e873_d_n25);
        let eq46_e874_d_n26: f64 = (eq46_e869_d_n26 + eq46_e873_d_n26);
        let eq46_e874_d_n27: f64 = (eq46_e869_d_n27 + eq46_e873_d_n27);
        let eq46_e874_d_n28: f64 = (eq46_e869_d_n28 + eq46_e873_d_n28);
        let eq46_e874_d_n29: f64 = (eq46_e869_d_n29 + eq46_e873_d_n29);
        (eq46_e874, eq46_e874_d_n0, eq46_e874_d_n1, eq46_e874_d_n2, eq46_e874_d_n3, eq46_e874_d_n4, eq46_e874_d_n5, eq46_e874_d_n6, eq46_e874_d_n7, eq46_e874_d_n8, eq46_e874_d_n9, eq46_e874_d_n10, eq46_e874_d_n11, eq46_e874_d_n12, eq46_e874_d_n13, eq46_e874_d_n14, eq46_e874_d_n15, eq46_e874_d_n16, eq46_e874_d_n17, eq46_e874_d_n18, eq46_e874_d_n19, eq46_e874_d_n20, eq46_e874_d_n21, eq46_e874_d_n22, eq46_e874_d_n23, eq46_e874_d_n24, eq46_e874_d_n25, eq46_e874_d_n26, eq46_e874_d_n27, eq46_e874_d_n28, eq46_e874_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e876;
        let eq46_node_derivatives: [f64; 30] = [eq46_e876_d_n0, eq46_e876_d_n1, eq46_e876_d_n2, eq46_e876_d_n3, eq46_e876_d_n4, eq46_e876_d_n5, eq46_e876_d_n6, eq46_e876_d_n7, eq46_e876_d_n8, eq46_e876_d_n9, eq46_e876_d_n10, eq46_e876_d_n11, eq46_e876_d_n12, eq46_e876_d_n13, eq46_e876_d_n14, eq46_e876_d_n15, eq46_e876_d_n16, eq46_e876_d_n17, eq46_e876_d_n18, eq46_e876_d_n19, eq46_e876_d_n20, eq46_e876_d_n21, eq46_e876_d_n22, eq46_e876_d_n23, eq46_e876_d_n24, eq46_e876_d_n25, eq46_e876_d_n26, eq46_e876_d_n27, eq46_e876_d_n28, eq46_e876_d_n29];
        let eq46_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[15]),
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq47_e886, eq47_e886_d_n0, eq47_e886_d_n1, eq47_e886_d_n2, eq47_e886_d_n3, eq47_e886_d_n4, eq47_e886_d_n5, eq47_e886_d_n6, eq47_e886_d_n7, eq47_e886_d_n8, eq47_e886_d_n9, eq47_e886_d_n10, eq47_e886_d_n11, eq47_e886_d_n12, eq47_e886_d_n13, eq47_e886_d_n14, eq47_e886_d_n15, eq47_e886_d_n16, eq47_e886_d_n17, eq47_e886_d_n18, eq47_e886_d_n19, eq47_e886_d_n20, eq47_e886_d_n21, eq47_e886_d_n22, eq47_e886_d_n23, eq47_e886_d_n24, eq47_e886_d_n25, eq47_e886_d_n26, eq47_e886_d_n27, eq47_e886_d_n28, eq47_e886_d_n29,) = {
    if (s.v[613] != 0.0) {
        let eq47_e879: f64 = self.eval_ddt(22, s.v[204]);
        let eq47_e879_d_n0: f64 = self.ddt_jacobian(s.dn[204][0]);
        let eq47_e879_d_n1: f64 = self.ddt_jacobian(s.dn[204][1]);
        let eq47_e879_d_n2: f64 = self.ddt_jacobian(s.dn[204][2]);
        let eq47_e879_d_n3: f64 = self.ddt_jacobian(s.dn[204][3]);
        let eq47_e879_d_n4: f64 = self.ddt_jacobian(s.dn[204][4]);
        let eq47_e879_d_n5: f64 = self.ddt_jacobian(s.dn[204][5]);
        let eq47_e879_d_n6: f64 = self.ddt_jacobian(s.dn[204][6]);
        let eq47_e879_d_n7: f64 = self.ddt_jacobian(s.dn[204][7]);
        let eq47_e879_d_n8: f64 = self.ddt_jacobian(s.dn[204][8]);
        let eq47_e879_d_n9: f64 = self.ddt_jacobian(s.dn[204][9]);
        let eq47_e879_d_n10: f64 = self.ddt_jacobian(s.dn[204][10]);
        let eq47_e879_d_n11: f64 = self.ddt_jacobian(s.dn[204][11]);
        let eq47_e879_d_n12: f64 = self.ddt_jacobian(s.dn[204][12]);
        let eq47_e879_d_n13: f64 = self.ddt_jacobian(s.dn[204][13]);
        let eq47_e879_d_n14: f64 = self.ddt_jacobian(s.dn[204][14]);
        let eq47_e879_d_n15: f64 = self.ddt_jacobian(s.dn[204][15]);
        let eq47_e879_d_n16: f64 = self.ddt_jacobian(s.dn[204][16]);
        let eq47_e879_d_n17: f64 = self.ddt_jacobian(s.dn[204][17]);
        let eq47_e879_d_n18: f64 = self.ddt_jacobian(s.dn[204][18]);
        let eq47_e879_d_n19: f64 = self.ddt_jacobian(s.dn[204][19]);
        let eq47_e879_d_n20: f64 = self.ddt_jacobian(s.dn[204][20]);
        let eq47_e879_d_n21: f64 = self.ddt_jacobian(s.dn[204][21]);
        let eq47_e879_d_n22: f64 = self.ddt_jacobian(s.dn[204][22]);
        let eq47_e879_d_n23: f64 = self.ddt_jacobian(s.dn[204][23]);
        let eq47_e879_d_n24: f64 = self.ddt_jacobian(s.dn[204][24]);
        let eq47_e879_d_n25: f64 = self.ddt_jacobian(s.dn[204][25]);
        let eq47_e879_d_n26: f64 = self.ddt_jacobian(s.dn[204][26]);
        let eq47_e879_d_n27: f64 = self.ddt_jacobian(s.dn[204][27]);
        let eq47_e879_d_n28: f64 = self.ddt_jacobian(s.dn[204][28]);
        let eq47_e879_d_n29: f64 = self.ddt_jacobian(s.dn[204][29]);
        let eq47_e882: f64 = (p.p355 * (nv7 - nv16));
        let eq47_e882_d_n7: f64 = p.p355;
        let eq47_e882_d_n16: f64 = (-p.p355);
        let eq47_e883: f64 = self.eval_ddt(23, eq47_e882);
        let eq47_e883_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n7: f64 = self.ddt_jacobian(eq47_e882_d_n7);
        let eq47_e883_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n16: f64 = self.ddt_jacobian(eq47_e882_d_n16);
        let eq47_e883_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq47_e883_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq47_e884: f64 = (eq47_e879 + eq47_e883);
        let eq47_e884_d_n0: f64 = (eq47_e879_d_n0 + eq47_e883_d_n0);
        let eq47_e884_d_n1: f64 = (eq47_e879_d_n1 + eq47_e883_d_n1);
        let eq47_e884_d_n2: f64 = (eq47_e879_d_n2 + eq47_e883_d_n2);
        let eq47_e884_d_n3: f64 = (eq47_e879_d_n3 + eq47_e883_d_n3);
        let eq47_e884_d_n4: f64 = (eq47_e879_d_n4 + eq47_e883_d_n4);
        let eq47_e884_d_n5: f64 = (eq47_e879_d_n5 + eq47_e883_d_n5);
        let eq47_e884_d_n6: f64 = (eq47_e879_d_n6 + eq47_e883_d_n6);
        let eq47_e884_d_n7: f64 = (eq47_e879_d_n7 + eq47_e883_d_n7);
        let eq47_e884_d_n8: f64 = (eq47_e879_d_n8 + eq47_e883_d_n8);
        let eq47_e884_d_n9: f64 = (eq47_e879_d_n9 + eq47_e883_d_n9);
        let eq47_e884_d_n10: f64 = (eq47_e879_d_n10 + eq47_e883_d_n10);
        let eq47_e884_d_n11: f64 = (eq47_e879_d_n11 + eq47_e883_d_n11);
        let eq47_e884_d_n12: f64 = (eq47_e879_d_n12 + eq47_e883_d_n12);
        let eq47_e884_d_n13: f64 = (eq47_e879_d_n13 + eq47_e883_d_n13);
        let eq47_e884_d_n14: f64 = (eq47_e879_d_n14 + eq47_e883_d_n14);
        let eq47_e884_d_n15: f64 = (eq47_e879_d_n15 + eq47_e883_d_n15);
        let eq47_e884_d_n16: f64 = (eq47_e879_d_n16 + eq47_e883_d_n16);
        let eq47_e884_d_n17: f64 = (eq47_e879_d_n17 + eq47_e883_d_n17);
        let eq47_e884_d_n18: f64 = (eq47_e879_d_n18 + eq47_e883_d_n18);
        let eq47_e884_d_n19: f64 = (eq47_e879_d_n19 + eq47_e883_d_n19);
        let eq47_e884_d_n20: f64 = (eq47_e879_d_n20 + eq47_e883_d_n20);
        let eq47_e884_d_n21: f64 = (eq47_e879_d_n21 + eq47_e883_d_n21);
        let eq47_e884_d_n22: f64 = (eq47_e879_d_n22 + eq47_e883_d_n22);
        let eq47_e884_d_n23: f64 = (eq47_e879_d_n23 + eq47_e883_d_n23);
        let eq47_e884_d_n24: f64 = (eq47_e879_d_n24 + eq47_e883_d_n24);
        let eq47_e884_d_n25: f64 = (eq47_e879_d_n25 + eq47_e883_d_n25);
        let eq47_e884_d_n26: f64 = (eq47_e879_d_n26 + eq47_e883_d_n26);
        let eq47_e884_d_n27: f64 = (eq47_e879_d_n27 + eq47_e883_d_n27);
        let eq47_e884_d_n28: f64 = (eq47_e879_d_n28 + eq47_e883_d_n28);
        let eq47_e884_d_n29: f64 = (eq47_e879_d_n29 + eq47_e883_d_n29);
        (eq47_e884, eq47_e884_d_n0, eq47_e884_d_n1, eq47_e884_d_n2, eq47_e884_d_n3, eq47_e884_d_n4, eq47_e884_d_n5, eq47_e884_d_n6, eq47_e884_d_n7, eq47_e884_d_n8, eq47_e884_d_n9, eq47_e884_d_n10, eq47_e884_d_n11, eq47_e884_d_n12, eq47_e884_d_n13, eq47_e884_d_n14, eq47_e884_d_n15, eq47_e884_d_n16, eq47_e884_d_n17, eq47_e884_d_n18, eq47_e884_d_n19, eq47_e884_d_n20, eq47_e884_d_n21, eq47_e884_d_n22, eq47_e884_d_n23, eq47_e884_d_n24, eq47_e884_d_n25, eq47_e884_d_n26, eq47_e884_d_n27, eq47_e884_d_n28, eq47_e884_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e886;
        let eq47_node_derivatives: [f64; 30] = [eq47_e886_d_n0, eq47_e886_d_n1, eq47_e886_d_n2, eq47_e886_d_n3, eq47_e886_d_n4, eq47_e886_d_n5, eq47_e886_d_n6, eq47_e886_d_n7, eq47_e886_d_n8, eq47_e886_d_n9, eq47_e886_d_n10, eq47_e886_d_n11, eq47_e886_d_n12, eq47_e886_d_n13, eq47_e886_d_n14, eq47_e886_d_n15, eq47_e886_d_n16, eq47_e886_d_n17, eq47_e886_d_n18, eq47_e886_d_n19, eq47_e886_d_n20, eq47_e886_d_n21, eq47_e886_d_n22, eq47_e886_d_n23, eq47_e886_d_n24, eq47_e886_d_n25, eq47_e886_d_n26, eq47_e886_d_n27, eq47_e886_d_n28, eq47_e886_d_n29];
        let eq47_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[16]),
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq48_e896, eq48_e896_d_n0, eq48_e896_d_n1, eq48_e896_d_n2, eq48_e896_d_n3, eq48_e896_d_n4, eq48_e896_d_n5, eq48_e896_d_n6, eq48_e896_d_n7, eq48_e896_d_n8, eq48_e896_d_n9, eq48_e896_d_n10, eq48_e896_d_n11, eq48_e896_d_n12, eq48_e896_d_n13, eq48_e896_d_n14, eq48_e896_d_n15, eq48_e896_d_n16, eq48_e896_d_n17, eq48_e896_d_n18, eq48_e896_d_n19, eq48_e896_d_n20, eq48_e896_d_n21, eq48_e896_d_n22, eq48_e896_d_n23, eq48_e896_d_n24, eq48_e896_d_n25, eq48_e896_d_n26, eq48_e896_d_n27, eq48_e896_d_n28, eq48_e896_d_n29,) = {
    if (s.v[613] != 0.0) {
        let eq48_e889: f64 = self.eval_ddt(24, s.v[205]);
        let eq48_e889_d_n0: f64 = self.ddt_jacobian(s.dn[205][0]);
        let eq48_e889_d_n1: f64 = self.ddt_jacobian(s.dn[205][1]);
        let eq48_e889_d_n2: f64 = self.ddt_jacobian(s.dn[205][2]);
        let eq48_e889_d_n3: f64 = self.ddt_jacobian(s.dn[205][3]);
        let eq48_e889_d_n4: f64 = self.ddt_jacobian(s.dn[205][4]);
        let eq48_e889_d_n5: f64 = self.ddt_jacobian(s.dn[205][5]);
        let eq48_e889_d_n6: f64 = self.ddt_jacobian(s.dn[205][6]);
        let eq48_e889_d_n7: f64 = self.ddt_jacobian(s.dn[205][7]);
        let eq48_e889_d_n8: f64 = self.ddt_jacobian(s.dn[205][8]);
        let eq48_e889_d_n9: f64 = self.ddt_jacobian(s.dn[205][9]);
        let eq48_e889_d_n10: f64 = self.ddt_jacobian(s.dn[205][10]);
        let eq48_e889_d_n11: f64 = self.ddt_jacobian(s.dn[205][11]);
        let eq48_e889_d_n12: f64 = self.ddt_jacobian(s.dn[205][12]);
        let eq48_e889_d_n13: f64 = self.ddt_jacobian(s.dn[205][13]);
        let eq48_e889_d_n14: f64 = self.ddt_jacobian(s.dn[205][14]);
        let eq48_e889_d_n15: f64 = self.ddt_jacobian(s.dn[205][15]);
        let eq48_e889_d_n16: f64 = self.ddt_jacobian(s.dn[205][16]);
        let eq48_e889_d_n17: f64 = self.ddt_jacobian(s.dn[205][17]);
        let eq48_e889_d_n18: f64 = self.ddt_jacobian(s.dn[205][18]);
        let eq48_e889_d_n19: f64 = self.ddt_jacobian(s.dn[205][19]);
        let eq48_e889_d_n20: f64 = self.ddt_jacobian(s.dn[205][20]);
        let eq48_e889_d_n21: f64 = self.ddt_jacobian(s.dn[205][21]);
        let eq48_e889_d_n22: f64 = self.ddt_jacobian(s.dn[205][22]);
        let eq48_e889_d_n23: f64 = self.ddt_jacobian(s.dn[205][23]);
        let eq48_e889_d_n24: f64 = self.ddt_jacobian(s.dn[205][24]);
        let eq48_e889_d_n25: f64 = self.ddt_jacobian(s.dn[205][25]);
        let eq48_e889_d_n26: f64 = self.ddt_jacobian(s.dn[205][26]);
        let eq48_e889_d_n27: f64 = self.ddt_jacobian(s.dn[205][27]);
        let eq48_e889_d_n28: f64 = self.ddt_jacobian(s.dn[205][28]);
        let eq48_e889_d_n29: f64 = self.ddt_jacobian(s.dn[205][29]);
        let eq48_e892: f64 = (p.p355 * (nv2 - nv15));
        let eq48_e892_d_n2: f64 = p.p355;
        let eq48_e892_d_n15: f64 = (-p.p355);
        let eq48_e893: f64 = self.eval_ddt(25, eq48_e892);
        let eq48_e893_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n2: f64 = self.ddt_jacobian(eq48_e892_d_n2);
        let eq48_e893_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n15: f64 = self.ddt_jacobian(eq48_e892_d_n15);
        let eq48_e893_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq48_e893_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq48_e894: f64 = (eq48_e889 + eq48_e893);
        let eq48_e894_d_n0: f64 = (eq48_e889_d_n0 + eq48_e893_d_n0);
        let eq48_e894_d_n1: f64 = (eq48_e889_d_n1 + eq48_e893_d_n1);
        let eq48_e894_d_n2: f64 = (eq48_e889_d_n2 + eq48_e893_d_n2);
        let eq48_e894_d_n3: f64 = (eq48_e889_d_n3 + eq48_e893_d_n3);
        let eq48_e894_d_n4: f64 = (eq48_e889_d_n4 + eq48_e893_d_n4);
        let eq48_e894_d_n5: f64 = (eq48_e889_d_n5 + eq48_e893_d_n5);
        let eq48_e894_d_n6: f64 = (eq48_e889_d_n6 + eq48_e893_d_n6);
        let eq48_e894_d_n7: f64 = (eq48_e889_d_n7 + eq48_e893_d_n7);
        let eq48_e894_d_n8: f64 = (eq48_e889_d_n8 + eq48_e893_d_n8);
        let eq48_e894_d_n9: f64 = (eq48_e889_d_n9 + eq48_e893_d_n9);
        let eq48_e894_d_n10: f64 = (eq48_e889_d_n10 + eq48_e893_d_n10);
        let eq48_e894_d_n11: f64 = (eq48_e889_d_n11 + eq48_e893_d_n11);
        let eq48_e894_d_n12: f64 = (eq48_e889_d_n12 + eq48_e893_d_n12);
        let eq48_e894_d_n13: f64 = (eq48_e889_d_n13 + eq48_e893_d_n13);
        let eq48_e894_d_n14: f64 = (eq48_e889_d_n14 + eq48_e893_d_n14);
        let eq48_e894_d_n15: f64 = (eq48_e889_d_n15 + eq48_e893_d_n15);
        let eq48_e894_d_n16: f64 = (eq48_e889_d_n16 + eq48_e893_d_n16);
        let eq48_e894_d_n17: f64 = (eq48_e889_d_n17 + eq48_e893_d_n17);
        let eq48_e894_d_n18: f64 = (eq48_e889_d_n18 + eq48_e893_d_n18);
        let eq48_e894_d_n19: f64 = (eq48_e889_d_n19 + eq48_e893_d_n19);
        let eq48_e894_d_n20: f64 = (eq48_e889_d_n20 + eq48_e893_d_n20);
        let eq48_e894_d_n21: f64 = (eq48_e889_d_n21 + eq48_e893_d_n21);
        let eq48_e894_d_n22: f64 = (eq48_e889_d_n22 + eq48_e893_d_n22);
        let eq48_e894_d_n23: f64 = (eq48_e889_d_n23 + eq48_e893_d_n23);
        let eq48_e894_d_n24: f64 = (eq48_e889_d_n24 + eq48_e893_d_n24);
        let eq48_e894_d_n25: f64 = (eq48_e889_d_n25 + eq48_e893_d_n25);
        let eq48_e894_d_n26: f64 = (eq48_e889_d_n26 + eq48_e893_d_n26);
        let eq48_e894_d_n27: f64 = (eq48_e889_d_n27 + eq48_e893_d_n27);
        let eq48_e894_d_n28: f64 = (eq48_e889_d_n28 + eq48_e893_d_n28);
        let eq48_e894_d_n29: f64 = (eq48_e889_d_n29 + eq48_e893_d_n29);
        (eq48_e894, eq48_e894_d_n0, eq48_e894_d_n1, eq48_e894_d_n2, eq48_e894_d_n3, eq48_e894_d_n4, eq48_e894_d_n5, eq48_e894_d_n6, eq48_e894_d_n7, eq48_e894_d_n8, eq48_e894_d_n9, eq48_e894_d_n10, eq48_e894_d_n11, eq48_e894_d_n12, eq48_e894_d_n13, eq48_e894_d_n14, eq48_e894_d_n15, eq48_e894_d_n16, eq48_e894_d_n17, eq48_e894_d_n18, eq48_e894_d_n19, eq48_e894_d_n20, eq48_e894_d_n21, eq48_e894_d_n22, eq48_e894_d_n23, eq48_e894_d_n24, eq48_e894_d_n25, eq48_e894_d_n26, eq48_e894_d_n27, eq48_e894_d_n28, eq48_e894_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_value: f64 = eq48_e896;
        let eq48_node_derivatives: [f64; 30] = [eq48_e896_d_n0, eq48_e896_d_n1, eq48_e896_d_n2, eq48_e896_d_n3, eq48_e896_d_n4, eq48_e896_d_n5, eq48_e896_d_n6, eq48_e896_d_n7, eq48_e896_d_n8, eq48_e896_d_n9, eq48_e896_d_n10, eq48_e896_d_n11, eq48_e896_d_n12, eq48_e896_d_n13, eq48_e896_d_n14, eq48_e896_d_n15, eq48_e896_d_n16, eq48_e896_d_n17, eq48_e896_d_n18, eq48_e896_d_n19, eq48_e896_d_n20, eq48_e896_d_n21, eq48_e896_d_n22, eq48_e896_d_n23, eq48_e896_d_n24, eq48_e896_d_n25, eq48_e896_d_n26, eq48_e896_d_n27, eq48_e896_d_n28, eq48_e896_d_n29];
        let eq48_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            self.multiplicity * (eq48_value),
            &nodes,
            &eq48_node_derivatives,
            &branches,
            &eq48_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_49_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq49_e900,) = {
    if (s.v[613] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e900;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[16]),
            self.multiplicity * (eq49_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_50_block_0(
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
        let (eq50_e910, eq50_e910_d_n0, eq50_e910_d_n1, eq50_e910_d_n2, eq50_e910_d_n3, eq50_e910_d_n4, eq50_e910_d_n5, eq50_e910_d_n6, eq50_e910_d_n7, eq50_e910_d_n8, eq50_e910_d_n9, eq50_e910_d_n10, eq50_e910_d_n11, eq50_e910_d_n12, eq50_e910_d_n13, eq50_e910_d_n14, eq50_e910_d_n15, eq50_e910_d_n16, eq50_e910_d_n17, eq50_e910_d_n18, eq50_e910_d_n19, eq50_e910_d_n20, eq50_e910_d_n21, eq50_e910_d_n22, eq50_e910_d_n23, eq50_e910_d_n24, eq50_e910_d_n25, eq50_e910_d_n26, eq50_e910_d_n27, eq50_e910_d_n28, eq50_e910_d_n29,) = {
    if (s.v[613] != 0.0) {
        let eq50_e903: f64 = self.eval_ddt(26, s.v[207]);
        let eq50_e903_d_n0: f64 = self.ddt_jacobian(s.dn[207][0]);
        let eq50_e903_d_n1: f64 = self.ddt_jacobian(s.dn[207][1]);
        let eq50_e903_d_n2: f64 = self.ddt_jacobian(s.dn[207][2]);
        let eq50_e903_d_n3: f64 = self.ddt_jacobian(s.dn[207][3]);
        let eq50_e903_d_n4: f64 = self.ddt_jacobian(s.dn[207][4]);
        let eq50_e903_d_n5: f64 = self.ddt_jacobian(s.dn[207][5]);
        let eq50_e903_d_n6: f64 = self.ddt_jacobian(s.dn[207][6]);
        let eq50_e903_d_n7: f64 = self.ddt_jacobian(s.dn[207][7]);
        let eq50_e903_d_n8: f64 = self.ddt_jacobian(s.dn[207][8]);
        let eq50_e903_d_n9: f64 = self.ddt_jacobian(s.dn[207][9]);
        let eq50_e903_d_n10: f64 = self.ddt_jacobian(s.dn[207][10]);
        let eq50_e903_d_n11: f64 = self.ddt_jacobian(s.dn[207][11]);
        let eq50_e903_d_n12: f64 = self.ddt_jacobian(s.dn[207][12]);
        let eq50_e903_d_n13: f64 = self.ddt_jacobian(s.dn[207][13]);
        let eq50_e903_d_n14: f64 = self.ddt_jacobian(s.dn[207][14]);
        let eq50_e903_d_n15: f64 = self.ddt_jacobian(s.dn[207][15]);
        let eq50_e903_d_n16: f64 = self.ddt_jacobian(s.dn[207][16]);
        let eq50_e903_d_n17: f64 = self.ddt_jacobian(s.dn[207][17]);
        let eq50_e903_d_n18: f64 = self.ddt_jacobian(s.dn[207][18]);
        let eq50_e903_d_n19: f64 = self.ddt_jacobian(s.dn[207][19]);
        let eq50_e903_d_n20: f64 = self.ddt_jacobian(s.dn[207][20]);
        let eq50_e903_d_n21: f64 = self.ddt_jacobian(s.dn[207][21]);
        let eq50_e903_d_n22: f64 = self.ddt_jacobian(s.dn[207][22]);
        let eq50_e903_d_n23: f64 = self.ddt_jacobian(s.dn[207][23]);
        let eq50_e903_d_n24: f64 = self.ddt_jacobian(s.dn[207][24]);
        let eq50_e903_d_n25: f64 = self.ddt_jacobian(s.dn[207][25]);
        let eq50_e903_d_n26: f64 = self.ddt_jacobian(s.dn[207][26]);
        let eq50_e903_d_n27: f64 = self.ddt_jacobian(s.dn[207][27]);
        let eq50_e903_d_n28: f64 = self.ddt_jacobian(s.dn[207][28]);
        let eq50_e903_d_n29: f64 = self.ddt_jacobian(s.dn[207][29]);
        let eq50_e906: f64 = (p.p355 * (nv7 - nv9));
        let eq50_e906_d_n7: f64 = p.p355;
        let eq50_e906_d_n9: f64 = (-p.p355);
        let eq50_e907: f64 = self.eval_ddt(27, eq50_e906);
        let eq50_e907_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n7: f64 = self.ddt_jacobian(eq50_e906_d_n7);
        let eq50_e907_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n9: f64 = self.ddt_jacobian(eq50_e906_d_n9);
        let eq50_e907_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq50_e907_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq50_e908: f64 = (eq50_e903 + eq50_e907);
        let eq50_e908_d_n0: f64 = (eq50_e903_d_n0 + eq50_e907_d_n0);
        let eq50_e908_d_n1: f64 = (eq50_e903_d_n1 + eq50_e907_d_n1);
        let eq50_e908_d_n2: f64 = (eq50_e903_d_n2 + eq50_e907_d_n2);
        let eq50_e908_d_n3: f64 = (eq50_e903_d_n3 + eq50_e907_d_n3);
        let eq50_e908_d_n4: f64 = (eq50_e903_d_n4 + eq50_e907_d_n4);
        let eq50_e908_d_n5: f64 = (eq50_e903_d_n5 + eq50_e907_d_n5);
        let eq50_e908_d_n6: f64 = (eq50_e903_d_n6 + eq50_e907_d_n6);
        let eq50_e908_d_n7: f64 = (eq50_e903_d_n7 + eq50_e907_d_n7);
        let eq50_e908_d_n8: f64 = (eq50_e903_d_n8 + eq50_e907_d_n8);
        let eq50_e908_d_n9: f64 = (eq50_e903_d_n9 + eq50_e907_d_n9);
        let eq50_e908_d_n10: f64 = (eq50_e903_d_n10 + eq50_e907_d_n10);
        let eq50_e908_d_n11: f64 = (eq50_e903_d_n11 + eq50_e907_d_n11);
        let eq50_e908_d_n12: f64 = (eq50_e903_d_n12 + eq50_e907_d_n12);
        let eq50_e908_d_n13: f64 = (eq50_e903_d_n13 + eq50_e907_d_n13);
        let eq50_e908_d_n14: f64 = (eq50_e903_d_n14 + eq50_e907_d_n14);
        let eq50_e908_d_n15: f64 = (eq50_e903_d_n15 + eq50_e907_d_n15);
        let eq50_e908_d_n16: f64 = (eq50_e903_d_n16 + eq50_e907_d_n16);
        let eq50_e908_d_n17: f64 = (eq50_e903_d_n17 + eq50_e907_d_n17);
        let eq50_e908_d_n18: f64 = (eq50_e903_d_n18 + eq50_e907_d_n18);
        let eq50_e908_d_n19: f64 = (eq50_e903_d_n19 + eq50_e907_d_n19);
        let eq50_e908_d_n20: f64 = (eq50_e903_d_n20 + eq50_e907_d_n20);
        let eq50_e908_d_n21: f64 = (eq50_e903_d_n21 + eq50_e907_d_n21);
        let eq50_e908_d_n22: f64 = (eq50_e903_d_n22 + eq50_e907_d_n22);
        let eq50_e908_d_n23: f64 = (eq50_e903_d_n23 + eq50_e907_d_n23);
        let eq50_e908_d_n24: f64 = (eq50_e903_d_n24 + eq50_e907_d_n24);
        let eq50_e908_d_n25: f64 = (eq50_e903_d_n25 + eq50_e907_d_n25);
        let eq50_e908_d_n26: f64 = (eq50_e903_d_n26 + eq50_e907_d_n26);
        let eq50_e908_d_n27: f64 = (eq50_e903_d_n27 + eq50_e907_d_n27);
        let eq50_e908_d_n28: f64 = (eq50_e903_d_n28 + eq50_e907_d_n28);
        let eq50_e908_d_n29: f64 = (eq50_e903_d_n29 + eq50_e907_d_n29);
        (eq50_e908, eq50_e908_d_n0, eq50_e908_d_n1, eq50_e908_d_n2, eq50_e908_d_n3, eq50_e908_d_n4, eq50_e908_d_n5, eq50_e908_d_n6, eq50_e908_d_n7, eq50_e908_d_n8, eq50_e908_d_n9, eq50_e908_d_n10, eq50_e908_d_n11, eq50_e908_d_n12, eq50_e908_d_n13, eq50_e908_d_n14, eq50_e908_d_n15, eq50_e908_d_n16, eq50_e908_d_n17, eq50_e908_d_n18, eq50_e908_d_n19, eq50_e908_d_n20, eq50_e908_d_n21, eq50_e908_d_n22, eq50_e908_d_n23, eq50_e908_d_n24, eq50_e908_d_n25, eq50_e908_d_n26, eq50_e908_d_n27, eq50_e908_d_n28, eq50_e908_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e910;
        let eq50_node_derivatives: [f64; 30] = [eq50_e910_d_n0, eq50_e910_d_n1, eq50_e910_d_n2, eq50_e910_d_n3, eq50_e910_d_n4, eq50_e910_d_n5, eq50_e910_d_n6, eq50_e910_d_n7, eq50_e910_d_n8, eq50_e910_d_n9, eq50_e910_d_n10, eq50_e910_d_n11, eq50_e910_d_n12, eq50_e910_d_n13, eq50_e910_d_n14, eq50_e910_d_n15, eq50_e910_d_n16, eq50_e910_d_n17, eq50_e910_d_n18, eq50_e910_d_n19, eq50_e910_d_n20, eq50_e910_d_n21, eq50_e910_d_n22, eq50_e910_d_n23, eq50_e910_d_n24, eq50_e910_d_n25, eq50_e910_d_n26, eq50_e910_d_n27, eq50_e910_d_n28, eq50_e910_d_n29];
        let eq50_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            self.multiplicity * (eq50_value),
            &nodes,
            &eq50_node_derivatives,
            &branches,
            &eq50_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_51_block_0(
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq51_e921, eq51_e921_d_n0, eq51_e921_d_n1, eq51_e921_d_n2, eq51_e921_d_n3, eq51_e921_d_n4, eq51_e921_d_n5, eq51_e921_d_n6, eq51_e921_d_n7, eq51_e921_d_n8, eq51_e921_d_n9, eq51_e921_d_n10, eq51_e921_d_n11, eq51_e921_d_n12, eq51_e921_d_n13, eq51_e921_d_n14, eq51_e921_d_n15, eq51_e921_d_n16, eq51_e921_d_n17, eq51_e921_d_n18, eq51_e921_d_n19, eq51_e921_d_n20, eq51_e921_d_n21, eq51_e921_d_n22, eq51_e921_d_n23, eq51_e921_d_n24, eq51_e921_d_n25, eq51_e921_d_n26, eq51_e921_d_n27, eq51_e921_d_n28, eq51_e921_d_n29,) = {
    if (!(s.v[613] != 0.0)) {
        let eq51_e914: f64 = self.eval_ddt(28, s.v[203]);
        let eq51_e914_d_n0: f64 = self.ddt_jacobian(s.dn[203][0]);
        let eq51_e914_d_n1: f64 = self.ddt_jacobian(s.dn[203][1]);
        let eq51_e914_d_n2: f64 = self.ddt_jacobian(s.dn[203][2]);
        let eq51_e914_d_n3: f64 = self.ddt_jacobian(s.dn[203][3]);
        let eq51_e914_d_n4: f64 = self.ddt_jacobian(s.dn[203][4]);
        let eq51_e914_d_n5: f64 = self.ddt_jacobian(s.dn[203][5]);
        let eq51_e914_d_n6: f64 = self.ddt_jacobian(s.dn[203][6]);
        let eq51_e914_d_n7: f64 = self.ddt_jacobian(s.dn[203][7]);
        let eq51_e914_d_n8: f64 = self.ddt_jacobian(s.dn[203][8]);
        let eq51_e914_d_n9: f64 = self.ddt_jacobian(s.dn[203][9]);
        let eq51_e914_d_n10: f64 = self.ddt_jacobian(s.dn[203][10]);
        let eq51_e914_d_n11: f64 = self.ddt_jacobian(s.dn[203][11]);
        let eq51_e914_d_n12: f64 = self.ddt_jacobian(s.dn[203][12]);
        let eq51_e914_d_n13: f64 = self.ddt_jacobian(s.dn[203][13]);
        let eq51_e914_d_n14: f64 = self.ddt_jacobian(s.dn[203][14]);
        let eq51_e914_d_n15: f64 = self.ddt_jacobian(s.dn[203][15]);
        let eq51_e914_d_n16: f64 = self.ddt_jacobian(s.dn[203][16]);
        let eq51_e914_d_n17: f64 = self.ddt_jacobian(s.dn[203][17]);
        let eq51_e914_d_n18: f64 = self.ddt_jacobian(s.dn[203][18]);
        let eq51_e914_d_n19: f64 = self.ddt_jacobian(s.dn[203][19]);
        let eq51_e914_d_n20: f64 = self.ddt_jacobian(s.dn[203][20]);
        let eq51_e914_d_n21: f64 = self.ddt_jacobian(s.dn[203][21]);
        let eq51_e914_d_n22: f64 = self.ddt_jacobian(s.dn[203][22]);
        let eq51_e914_d_n23: f64 = self.ddt_jacobian(s.dn[203][23]);
        let eq51_e914_d_n24: f64 = self.ddt_jacobian(s.dn[203][24]);
        let eq51_e914_d_n25: f64 = self.ddt_jacobian(s.dn[203][25]);
        let eq51_e914_d_n26: f64 = self.ddt_jacobian(s.dn[203][26]);
        let eq51_e914_d_n27: f64 = self.ddt_jacobian(s.dn[203][27]);
        let eq51_e914_d_n28: f64 = self.ddt_jacobian(s.dn[203][28]);
        let eq51_e914_d_n29: f64 = self.ddt_jacobian(s.dn[203][29]);
        let eq51_e917: f64 = (p.p355 * (nv2 - nv15));
        let eq51_e917_d_n2: f64 = p.p355;
        let eq51_e917_d_n15: f64 = (-p.p355);
        let eq51_e918: f64 = self.eval_ddt(29, eq51_e917);
        let eq51_e918_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n2: f64 = self.ddt_jacobian(eq51_e917_d_n2);
        let eq51_e918_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n15: f64 = self.ddt_jacobian(eq51_e917_d_n15);
        let eq51_e918_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq51_e918_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq51_e919: f64 = (eq51_e914 + eq51_e918);
        let eq51_e919_d_n0: f64 = (eq51_e914_d_n0 + eq51_e918_d_n0);
        let eq51_e919_d_n1: f64 = (eq51_e914_d_n1 + eq51_e918_d_n1);
        let eq51_e919_d_n2: f64 = (eq51_e914_d_n2 + eq51_e918_d_n2);
        let eq51_e919_d_n3: f64 = (eq51_e914_d_n3 + eq51_e918_d_n3);
        let eq51_e919_d_n4: f64 = (eq51_e914_d_n4 + eq51_e918_d_n4);
        let eq51_e919_d_n5: f64 = (eq51_e914_d_n5 + eq51_e918_d_n5);
        let eq51_e919_d_n6: f64 = (eq51_e914_d_n6 + eq51_e918_d_n6);
        let eq51_e919_d_n7: f64 = (eq51_e914_d_n7 + eq51_e918_d_n7);
        let eq51_e919_d_n8: f64 = (eq51_e914_d_n8 + eq51_e918_d_n8);
        let eq51_e919_d_n9: f64 = (eq51_e914_d_n9 + eq51_e918_d_n9);
        let eq51_e919_d_n10: f64 = (eq51_e914_d_n10 + eq51_e918_d_n10);
        let eq51_e919_d_n11: f64 = (eq51_e914_d_n11 + eq51_e918_d_n11);
        let eq51_e919_d_n12: f64 = (eq51_e914_d_n12 + eq51_e918_d_n12);
        let eq51_e919_d_n13: f64 = (eq51_e914_d_n13 + eq51_e918_d_n13);
        let eq51_e919_d_n14: f64 = (eq51_e914_d_n14 + eq51_e918_d_n14);
        let eq51_e919_d_n15: f64 = (eq51_e914_d_n15 + eq51_e918_d_n15);
        let eq51_e919_d_n16: f64 = (eq51_e914_d_n16 + eq51_e918_d_n16);
        let eq51_e919_d_n17: f64 = (eq51_e914_d_n17 + eq51_e918_d_n17);
        let eq51_e919_d_n18: f64 = (eq51_e914_d_n18 + eq51_e918_d_n18);
        let eq51_e919_d_n19: f64 = (eq51_e914_d_n19 + eq51_e918_d_n19);
        let eq51_e919_d_n20: f64 = (eq51_e914_d_n20 + eq51_e918_d_n20);
        let eq51_e919_d_n21: f64 = (eq51_e914_d_n21 + eq51_e918_d_n21);
        let eq51_e919_d_n22: f64 = (eq51_e914_d_n22 + eq51_e918_d_n22);
        let eq51_e919_d_n23: f64 = (eq51_e914_d_n23 + eq51_e918_d_n23);
        let eq51_e919_d_n24: f64 = (eq51_e914_d_n24 + eq51_e918_d_n24);
        let eq51_e919_d_n25: f64 = (eq51_e914_d_n25 + eq51_e918_d_n25);
        let eq51_e919_d_n26: f64 = (eq51_e914_d_n26 + eq51_e918_d_n26);
        let eq51_e919_d_n27: f64 = (eq51_e914_d_n27 + eq51_e918_d_n27);
        let eq51_e919_d_n28: f64 = (eq51_e914_d_n28 + eq51_e918_d_n28);
        let eq51_e919_d_n29: f64 = (eq51_e914_d_n29 + eq51_e918_d_n29);
        (eq51_e919, eq51_e919_d_n0, eq51_e919_d_n1, eq51_e919_d_n2, eq51_e919_d_n3, eq51_e919_d_n4, eq51_e919_d_n5, eq51_e919_d_n6, eq51_e919_d_n7, eq51_e919_d_n8, eq51_e919_d_n9, eq51_e919_d_n10, eq51_e919_d_n11, eq51_e919_d_n12, eq51_e919_d_n13, eq51_e919_d_n14, eq51_e919_d_n15, eq51_e919_d_n16, eq51_e919_d_n17, eq51_e919_d_n18, eq51_e919_d_n19, eq51_e919_d_n20, eq51_e919_d_n21, eq51_e919_d_n22, eq51_e919_d_n23, eq51_e919_d_n24, eq51_e919_d_n25, eq51_e919_d_n26, eq51_e919_d_n27, eq51_e919_d_n28, eq51_e919_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e921;
        let eq51_node_derivatives: [f64; 30] = [eq51_e921_d_n0, eq51_e921_d_n1, eq51_e921_d_n2, eq51_e921_d_n3, eq51_e921_d_n4, eq51_e921_d_n5, eq51_e921_d_n6, eq51_e921_d_n7, eq51_e921_d_n8, eq51_e921_d_n9, eq51_e921_d_n10, eq51_e921_d_n11, eq51_e921_d_n12, eq51_e921_d_n13, eq51_e921_d_n14, eq51_e921_d_n15, eq51_e921_d_n16, eq51_e921_d_n17, eq51_e921_d_n18, eq51_e921_d_n19, eq51_e921_d_n20, eq51_e921_d_n21, eq51_e921_d_n22, eq51_e921_d_n23, eq51_e921_d_n24, eq51_e921_d_n25, eq51_e921_d_n26, eq51_e921_d_n27, eq51_e921_d_n28, eq51_e921_d_n29];
        let eq51_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            self.multiplicity * (eq51_value),
            &nodes,
            &eq51_node_derivatives,
            &branches,
            &eq51_branch_derivatives,
            self.multiplicity,
        );
    }
}
