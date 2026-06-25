#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_84_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq84_e1187,) = {
    if (!(s.v[908] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq84_value: f64 = eq84_e1187;
        stamper.stamp_potential(
            branches[22],
            eq84_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_85_block_0(
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq85_e1197, eq85_e1197_d_n0, eq85_e1197_d_n1, eq85_e1197_d_n2, eq85_e1197_d_n3, eq85_e1197_d_n4, eq85_e1197_d_n5, eq85_e1197_d_n6, eq85_e1197_d_n7, eq85_e1197_d_n8, eq85_e1197_d_n9, eq85_e1197_d_n10, eq85_e1197_d_n11, eq85_e1197_d_n12, eq85_e1197_d_n13, eq85_e1197_d_n14, eq85_e1197_d_n15, eq85_e1197_d_n16, eq85_e1197_d_n17, eq85_e1197_d_n18, eq85_e1197_d_n19, eq85_e1197_d_n20, eq85_e1197_d_n21, eq85_e1197_d_n22, eq85_e1197_d_n23, eq85_e1197_d_n24, eq85_e1197_d_n25, eq85_e1197_d_n26, eq85_e1197_d_n27, eq85_e1197_d_n28, eq85_e1197_d_n29,) = {
    if (s.v[1054] != 0.0) {
        let eq85_e1190: f64 = self.eval_ddt(68, s.v[167]);
        let eq85_e1190_d_n0: f64 = self.ddt_jacobian(s.dn[167][0]);
        let eq85_e1190_d_n1: f64 = self.ddt_jacobian(s.dn[167][1]);
        let eq85_e1190_d_n2: f64 = self.ddt_jacobian(s.dn[167][2]);
        let eq85_e1190_d_n3: f64 = self.ddt_jacobian(s.dn[167][3]);
        let eq85_e1190_d_n4: f64 = self.ddt_jacobian(s.dn[167][4]);
        let eq85_e1190_d_n5: f64 = self.ddt_jacobian(s.dn[167][5]);
        let eq85_e1190_d_n6: f64 = self.ddt_jacobian(s.dn[167][6]);
        let eq85_e1190_d_n7: f64 = self.ddt_jacobian(s.dn[167][7]);
        let eq85_e1190_d_n8: f64 = self.ddt_jacobian(s.dn[167][8]);
        let eq85_e1190_d_n9: f64 = self.ddt_jacobian(s.dn[167][9]);
        let eq85_e1190_d_n10: f64 = self.ddt_jacobian(s.dn[167][10]);
        let eq85_e1190_d_n11: f64 = self.ddt_jacobian(s.dn[167][11]);
        let eq85_e1190_d_n12: f64 = self.ddt_jacobian(s.dn[167][12]);
        let eq85_e1190_d_n13: f64 = self.ddt_jacobian(s.dn[167][13]);
        let eq85_e1190_d_n14: f64 = self.ddt_jacobian(s.dn[167][14]);
        let eq85_e1190_d_n15: f64 = self.ddt_jacobian(s.dn[167][15]);
        let eq85_e1190_d_n16: f64 = self.ddt_jacobian(s.dn[167][16]);
        let eq85_e1190_d_n17: f64 = self.ddt_jacobian(s.dn[167][17]);
        let eq85_e1190_d_n18: f64 = self.ddt_jacobian(s.dn[167][18]);
        let eq85_e1190_d_n19: f64 = self.ddt_jacobian(s.dn[167][19]);
        let eq85_e1190_d_n20: f64 = self.ddt_jacobian(s.dn[167][20]);
        let eq85_e1190_d_n21: f64 = self.ddt_jacobian(s.dn[167][21]);
        let eq85_e1190_d_n22: f64 = self.ddt_jacobian(s.dn[167][22]);
        let eq85_e1190_d_n23: f64 = self.ddt_jacobian(s.dn[167][23]);
        let eq85_e1190_d_n24: f64 = self.ddt_jacobian(s.dn[167][24]);
        let eq85_e1190_d_n25: f64 = self.ddt_jacobian(s.dn[167][25]);
        let eq85_e1190_d_n26: f64 = self.ddt_jacobian(s.dn[167][26]);
        let eq85_e1190_d_n27: f64 = self.ddt_jacobian(s.dn[167][27]);
        let eq85_e1190_d_n28: f64 = self.ddt_jacobian(s.dn[167][28]);
        let eq85_e1190_d_n29: f64 = self.ddt_jacobian(s.dn[167][29]);
        let eq85_e1193: f64 = (p.p355 * (nv7 - nv10));
        let eq85_e1193_d_n7: f64 = p.p355;
        let eq85_e1193_d_n10: f64 = (-p.p355);
        let eq85_e1194: f64 = self.eval_ddt(69, eq85_e1193);
        let eq85_e1194_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n7: f64 = self.ddt_jacobian(eq85_e1193_d_n7);
        let eq85_e1194_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n10: f64 = self.ddt_jacobian(eq85_e1193_d_n10);
        let eq85_e1194_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq85_e1194_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq85_e1195: f64 = (eq85_e1190 + eq85_e1194);
        let eq85_e1195_d_n0: f64 = (eq85_e1190_d_n0 + eq85_e1194_d_n0);
        let eq85_e1195_d_n1: f64 = (eq85_e1190_d_n1 + eq85_e1194_d_n1);
        let eq85_e1195_d_n2: f64 = (eq85_e1190_d_n2 + eq85_e1194_d_n2);
        let eq85_e1195_d_n3: f64 = (eq85_e1190_d_n3 + eq85_e1194_d_n3);
        let eq85_e1195_d_n4: f64 = (eq85_e1190_d_n4 + eq85_e1194_d_n4);
        let eq85_e1195_d_n5: f64 = (eq85_e1190_d_n5 + eq85_e1194_d_n5);
        let eq85_e1195_d_n6: f64 = (eq85_e1190_d_n6 + eq85_e1194_d_n6);
        let eq85_e1195_d_n7: f64 = (eq85_e1190_d_n7 + eq85_e1194_d_n7);
        let eq85_e1195_d_n8: f64 = (eq85_e1190_d_n8 + eq85_e1194_d_n8);
        let eq85_e1195_d_n9: f64 = (eq85_e1190_d_n9 + eq85_e1194_d_n9);
        let eq85_e1195_d_n10: f64 = (eq85_e1190_d_n10 + eq85_e1194_d_n10);
        let eq85_e1195_d_n11: f64 = (eq85_e1190_d_n11 + eq85_e1194_d_n11);
        let eq85_e1195_d_n12: f64 = (eq85_e1190_d_n12 + eq85_e1194_d_n12);
        let eq85_e1195_d_n13: f64 = (eq85_e1190_d_n13 + eq85_e1194_d_n13);
        let eq85_e1195_d_n14: f64 = (eq85_e1190_d_n14 + eq85_e1194_d_n14);
        let eq85_e1195_d_n15: f64 = (eq85_e1190_d_n15 + eq85_e1194_d_n15);
        let eq85_e1195_d_n16: f64 = (eq85_e1190_d_n16 + eq85_e1194_d_n16);
        let eq85_e1195_d_n17: f64 = (eq85_e1190_d_n17 + eq85_e1194_d_n17);
        let eq85_e1195_d_n18: f64 = (eq85_e1190_d_n18 + eq85_e1194_d_n18);
        let eq85_e1195_d_n19: f64 = (eq85_e1190_d_n19 + eq85_e1194_d_n19);
        let eq85_e1195_d_n20: f64 = (eq85_e1190_d_n20 + eq85_e1194_d_n20);
        let eq85_e1195_d_n21: f64 = (eq85_e1190_d_n21 + eq85_e1194_d_n21);
        let eq85_e1195_d_n22: f64 = (eq85_e1190_d_n22 + eq85_e1194_d_n22);
        let eq85_e1195_d_n23: f64 = (eq85_e1190_d_n23 + eq85_e1194_d_n23);
        let eq85_e1195_d_n24: f64 = (eq85_e1190_d_n24 + eq85_e1194_d_n24);
        let eq85_e1195_d_n25: f64 = (eq85_e1190_d_n25 + eq85_e1194_d_n25);
        let eq85_e1195_d_n26: f64 = (eq85_e1190_d_n26 + eq85_e1194_d_n26);
        let eq85_e1195_d_n27: f64 = (eq85_e1190_d_n27 + eq85_e1194_d_n27);
        let eq85_e1195_d_n28: f64 = (eq85_e1190_d_n28 + eq85_e1194_d_n28);
        let eq85_e1195_d_n29: f64 = (eq85_e1190_d_n29 + eq85_e1194_d_n29);
        (eq85_e1195, eq85_e1195_d_n0, eq85_e1195_d_n1, eq85_e1195_d_n2, eq85_e1195_d_n3, eq85_e1195_d_n4, eq85_e1195_d_n5, eq85_e1195_d_n6, eq85_e1195_d_n7, eq85_e1195_d_n8, eq85_e1195_d_n9, eq85_e1195_d_n10, eq85_e1195_d_n11, eq85_e1195_d_n12, eq85_e1195_d_n13, eq85_e1195_d_n14, eq85_e1195_d_n15, eq85_e1195_d_n16, eq85_e1195_d_n17, eq85_e1195_d_n18, eq85_e1195_d_n19, eq85_e1195_d_n20, eq85_e1195_d_n21, eq85_e1195_d_n22, eq85_e1195_d_n23, eq85_e1195_d_n24, eq85_e1195_d_n25, eq85_e1195_d_n26, eq85_e1195_d_n27, eq85_e1195_d_n28, eq85_e1195_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq85_value: f64 = eq85_e1197;
        let eq85_node_derivatives: [f64; 30] = [eq85_e1197_d_n0, eq85_e1197_d_n1, eq85_e1197_d_n2, eq85_e1197_d_n3, eq85_e1197_d_n4, eq85_e1197_d_n5, eq85_e1197_d_n6, eq85_e1197_d_n7, eq85_e1197_d_n8, eq85_e1197_d_n9, eq85_e1197_d_n10, eq85_e1197_d_n11, eq85_e1197_d_n12, eq85_e1197_d_n13, eq85_e1197_d_n14, eq85_e1197_d_n15, eq85_e1197_d_n16, eq85_e1197_d_n17, eq85_e1197_d_n18, eq85_e1197_d_n19, eq85_e1197_d_n20, eq85_e1197_d_n21, eq85_e1197_d_n22, eq85_e1197_d_n23, eq85_e1197_d_n24, eq85_e1197_d_n25, eq85_e1197_d_n26, eq85_e1197_d_n27, eq85_e1197_d_n28, eq85_e1197_d_n29];
        let eq85_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            self.multiplicity * (eq85_value),
            &nodes,
            &eq85_node_derivatives,
            &branches,
            &eq85_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_86_block_0(
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
        let (eq86_e1207, eq86_e1207_d_n0, eq86_e1207_d_n1, eq86_e1207_d_n2, eq86_e1207_d_n3, eq86_e1207_d_n4, eq86_e1207_d_n5, eq86_e1207_d_n6, eq86_e1207_d_n7, eq86_e1207_d_n8, eq86_e1207_d_n9, eq86_e1207_d_n10, eq86_e1207_d_n11, eq86_e1207_d_n12, eq86_e1207_d_n13, eq86_e1207_d_n14, eq86_e1207_d_n15, eq86_e1207_d_n16, eq86_e1207_d_n17, eq86_e1207_d_n18, eq86_e1207_d_n19, eq86_e1207_d_n20, eq86_e1207_d_n21, eq86_e1207_d_n22, eq86_e1207_d_n23, eq86_e1207_d_n24, eq86_e1207_d_n25, eq86_e1207_d_n26, eq86_e1207_d_n27, eq86_e1207_d_n28, eq86_e1207_d_n29,) = {
    if (s.v[1054] != 0.0) {
        let eq86_e1200: f64 = self.eval_ddt(70, s.v[168]);
        let eq86_e1200_d_n0: f64 = self.ddt_jacobian(s.dn[168][0]);
        let eq86_e1200_d_n1: f64 = self.ddt_jacobian(s.dn[168][1]);
        let eq86_e1200_d_n2: f64 = self.ddt_jacobian(s.dn[168][2]);
        let eq86_e1200_d_n3: f64 = self.ddt_jacobian(s.dn[168][3]);
        let eq86_e1200_d_n4: f64 = self.ddt_jacobian(s.dn[168][4]);
        let eq86_e1200_d_n5: f64 = self.ddt_jacobian(s.dn[168][5]);
        let eq86_e1200_d_n6: f64 = self.ddt_jacobian(s.dn[168][6]);
        let eq86_e1200_d_n7: f64 = self.ddt_jacobian(s.dn[168][7]);
        let eq86_e1200_d_n8: f64 = self.ddt_jacobian(s.dn[168][8]);
        let eq86_e1200_d_n9: f64 = self.ddt_jacobian(s.dn[168][9]);
        let eq86_e1200_d_n10: f64 = self.ddt_jacobian(s.dn[168][10]);
        let eq86_e1200_d_n11: f64 = self.ddt_jacobian(s.dn[168][11]);
        let eq86_e1200_d_n12: f64 = self.ddt_jacobian(s.dn[168][12]);
        let eq86_e1200_d_n13: f64 = self.ddt_jacobian(s.dn[168][13]);
        let eq86_e1200_d_n14: f64 = self.ddt_jacobian(s.dn[168][14]);
        let eq86_e1200_d_n15: f64 = self.ddt_jacobian(s.dn[168][15]);
        let eq86_e1200_d_n16: f64 = self.ddt_jacobian(s.dn[168][16]);
        let eq86_e1200_d_n17: f64 = self.ddt_jacobian(s.dn[168][17]);
        let eq86_e1200_d_n18: f64 = self.ddt_jacobian(s.dn[168][18]);
        let eq86_e1200_d_n19: f64 = self.ddt_jacobian(s.dn[168][19]);
        let eq86_e1200_d_n20: f64 = self.ddt_jacobian(s.dn[168][20]);
        let eq86_e1200_d_n21: f64 = self.ddt_jacobian(s.dn[168][21]);
        let eq86_e1200_d_n22: f64 = self.ddt_jacobian(s.dn[168][22]);
        let eq86_e1200_d_n23: f64 = self.ddt_jacobian(s.dn[168][23]);
        let eq86_e1200_d_n24: f64 = self.ddt_jacobian(s.dn[168][24]);
        let eq86_e1200_d_n25: f64 = self.ddt_jacobian(s.dn[168][25]);
        let eq86_e1200_d_n26: f64 = self.ddt_jacobian(s.dn[168][26]);
        let eq86_e1200_d_n27: f64 = self.ddt_jacobian(s.dn[168][27]);
        let eq86_e1200_d_n28: f64 = self.ddt_jacobian(s.dn[168][28]);
        let eq86_e1200_d_n29: f64 = self.ddt_jacobian(s.dn[168][29]);
        let eq86_e1203: f64 = (p.p355 * (nv7 - nv9));
        let eq86_e1203_d_n7: f64 = p.p355;
        let eq86_e1203_d_n9: f64 = (-p.p355);
        let eq86_e1204: f64 = self.eval_ddt(71, eq86_e1203);
        let eq86_e1204_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n7: f64 = self.ddt_jacobian(eq86_e1203_d_n7);
        let eq86_e1204_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n9: f64 = self.ddt_jacobian(eq86_e1203_d_n9);
        let eq86_e1204_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq86_e1204_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq86_e1205: f64 = (eq86_e1200 + eq86_e1204);
        let eq86_e1205_d_n0: f64 = (eq86_e1200_d_n0 + eq86_e1204_d_n0);
        let eq86_e1205_d_n1: f64 = (eq86_e1200_d_n1 + eq86_e1204_d_n1);
        let eq86_e1205_d_n2: f64 = (eq86_e1200_d_n2 + eq86_e1204_d_n2);
        let eq86_e1205_d_n3: f64 = (eq86_e1200_d_n3 + eq86_e1204_d_n3);
        let eq86_e1205_d_n4: f64 = (eq86_e1200_d_n4 + eq86_e1204_d_n4);
        let eq86_e1205_d_n5: f64 = (eq86_e1200_d_n5 + eq86_e1204_d_n5);
        let eq86_e1205_d_n6: f64 = (eq86_e1200_d_n6 + eq86_e1204_d_n6);
        let eq86_e1205_d_n7: f64 = (eq86_e1200_d_n7 + eq86_e1204_d_n7);
        let eq86_e1205_d_n8: f64 = (eq86_e1200_d_n8 + eq86_e1204_d_n8);
        let eq86_e1205_d_n9: f64 = (eq86_e1200_d_n9 + eq86_e1204_d_n9);
        let eq86_e1205_d_n10: f64 = (eq86_e1200_d_n10 + eq86_e1204_d_n10);
        let eq86_e1205_d_n11: f64 = (eq86_e1200_d_n11 + eq86_e1204_d_n11);
        let eq86_e1205_d_n12: f64 = (eq86_e1200_d_n12 + eq86_e1204_d_n12);
        let eq86_e1205_d_n13: f64 = (eq86_e1200_d_n13 + eq86_e1204_d_n13);
        let eq86_e1205_d_n14: f64 = (eq86_e1200_d_n14 + eq86_e1204_d_n14);
        let eq86_e1205_d_n15: f64 = (eq86_e1200_d_n15 + eq86_e1204_d_n15);
        let eq86_e1205_d_n16: f64 = (eq86_e1200_d_n16 + eq86_e1204_d_n16);
        let eq86_e1205_d_n17: f64 = (eq86_e1200_d_n17 + eq86_e1204_d_n17);
        let eq86_e1205_d_n18: f64 = (eq86_e1200_d_n18 + eq86_e1204_d_n18);
        let eq86_e1205_d_n19: f64 = (eq86_e1200_d_n19 + eq86_e1204_d_n19);
        let eq86_e1205_d_n20: f64 = (eq86_e1200_d_n20 + eq86_e1204_d_n20);
        let eq86_e1205_d_n21: f64 = (eq86_e1200_d_n21 + eq86_e1204_d_n21);
        let eq86_e1205_d_n22: f64 = (eq86_e1200_d_n22 + eq86_e1204_d_n22);
        let eq86_e1205_d_n23: f64 = (eq86_e1200_d_n23 + eq86_e1204_d_n23);
        let eq86_e1205_d_n24: f64 = (eq86_e1200_d_n24 + eq86_e1204_d_n24);
        let eq86_e1205_d_n25: f64 = (eq86_e1200_d_n25 + eq86_e1204_d_n25);
        let eq86_e1205_d_n26: f64 = (eq86_e1200_d_n26 + eq86_e1204_d_n26);
        let eq86_e1205_d_n27: f64 = (eq86_e1200_d_n27 + eq86_e1204_d_n27);
        let eq86_e1205_d_n28: f64 = (eq86_e1200_d_n28 + eq86_e1204_d_n28);
        let eq86_e1205_d_n29: f64 = (eq86_e1200_d_n29 + eq86_e1204_d_n29);
        (eq86_e1205, eq86_e1205_d_n0, eq86_e1205_d_n1, eq86_e1205_d_n2, eq86_e1205_d_n3, eq86_e1205_d_n4, eq86_e1205_d_n5, eq86_e1205_d_n6, eq86_e1205_d_n7, eq86_e1205_d_n8, eq86_e1205_d_n9, eq86_e1205_d_n10, eq86_e1205_d_n11, eq86_e1205_d_n12, eq86_e1205_d_n13, eq86_e1205_d_n14, eq86_e1205_d_n15, eq86_e1205_d_n16, eq86_e1205_d_n17, eq86_e1205_d_n18, eq86_e1205_d_n19, eq86_e1205_d_n20, eq86_e1205_d_n21, eq86_e1205_d_n22, eq86_e1205_d_n23, eq86_e1205_d_n24, eq86_e1205_d_n25, eq86_e1205_d_n26, eq86_e1205_d_n27, eq86_e1205_d_n28, eq86_e1205_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq86_value: f64 = eq86_e1207;
        let eq86_node_derivatives: [f64; 30] = [eq86_e1207_d_n0, eq86_e1207_d_n1, eq86_e1207_d_n2, eq86_e1207_d_n3, eq86_e1207_d_n4, eq86_e1207_d_n5, eq86_e1207_d_n6, eq86_e1207_d_n7, eq86_e1207_d_n8, eq86_e1207_d_n9, eq86_e1207_d_n10, eq86_e1207_d_n11, eq86_e1207_d_n12, eq86_e1207_d_n13, eq86_e1207_d_n14, eq86_e1207_d_n15, eq86_e1207_d_n16, eq86_e1207_d_n17, eq86_e1207_d_n18, eq86_e1207_d_n19, eq86_e1207_d_n20, eq86_e1207_d_n21, eq86_e1207_d_n22, eq86_e1207_d_n23, eq86_e1207_d_n24, eq86_e1207_d_n25, eq86_e1207_d_n26, eq86_e1207_d_n27, eq86_e1207_d_n28, eq86_e1207_d_n29];
        let eq86_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            self.multiplicity * (eq86_value),
            &nodes,
            &eq86_node_derivatives,
            &branches,
            &eq86_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_87_block_0(
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
        let (eq87_e1217, eq87_e1217_d_n0, eq87_e1217_d_n1, eq87_e1217_d_n2, eq87_e1217_d_n3, eq87_e1217_d_n4, eq87_e1217_d_n5, eq87_e1217_d_n6, eq87_e1217_d_n7, eq87_e1217_d_n8, eq87_e1217_d_n9, eq87_e1217_d_n10, eq87_e1217_d_n11, eq87_e1217_d_n12, eq87_e1217_d_n13, eq87_e1217_d_n14, eq87_e1217_d_n15, eq87_e1217_d_n16, eq87_e1217_d_n17, eq87_e1217_d_n18, eq87_e1217_d_n19, eq87_e1217_d_n20, eq87_e1217_d_n21, eq87_e1217_d_n22, eq87_e1217_d_n23, eq87_e1217_d_n24, eq87_e1217_d_n25, eq87_e1217_d_n26, eq87_e1217_d_n27, eq87_e1217_d_n28, eq87_e1217_d_n29,) = {
    if (s.v[1054] != 0.0) {
        let eq87_e1210: f64 = self.eval_ddt(72, s.v[169]);
        let eq87_e1210_d_n0: f64 = self.ddt_jacobian(s.dn[169][0]);
        let eq87_e1210_d_n1: f64 = self.ddt_jacobian(s.dn[169][1]);
        let eq87_e1210_d_n2: f64 = self.ddt_jacobian(s.dn[169][2]);
        let eq87_e1210_d_n3: f64 = self.ddt_jacobian(s.dn[169][3]);
        let eq87_e1210_d_n4: f64 = self.ddt_jacobian(s.dn[169][4]);
        let eq87_e1210_d_n5: f64 = self.ddt_jacobian(s.dn[169][5]);
        let eq87_e1210_d_n6: f64 = self.ddt_jacobian(s.dn[169][6]);
        let eq87_e1210_d_n7: f64 = self.ddt_jacobian(s.dn[169][7]);
        let eq87_e1210_d_n8: f64 = self.ddt_jacobian(s.dn[169][8]);
        let eq87_e1210_d_n9: f64 = self.ddt_jacobian(s.dn[169][9]);
        let eq87_e1210_d_n10: f64 = self.ddt_jacobian(s.dn[169][10]);
        let eq87_e1210_d_n11: f64 = self.ddt_jacobian(s.dn[169][11]);
        let eq87_e1210_d_n12: f64 = self.ddt_jacobian(s.dn[169][12]);
        let eq87_e1210_d_n13: f64 = self.ddt_jacobian(s.dn[169][13]);
        let eq87_e1210_d_n14: f64 = self.ddt_jacobian(s.dn[169][14]);
        let eq87_e1210_d_n15: f64 = self.ddt_jacobian(s.dn[169][15]);
        let eq87_e1210_d_n16: f64 = self.ddt_jacobian(s.dn[169][16]);
        let eq87_e1210_d_n17: f64 = self.ddt_jacobian(s.dn[169][17]);
        let eq87_e1210_d_n18: f64 = self.ddt_jacobian(s.dn[169][18]);
        let eq87_e1210_d_n19: f64 = self.ddt_jacobian(s.dn[169][19]);
        let eq87_e1210_d_n20: f64 = self.ddt_jacobian(s.dn[169][20]);
        let eq87_e1210_d_n21: f64 = self.ddt_jacobian(s.dn[169][21]);
        let eq87_e1210_d_n22: f64 = self.ddt_jacobian(s.dn[169][22]);
        let eq87_e1210_d_n23: f64 = self.ddt_jacobian(s.dn[169][23]);
        let eq87_e1210_d_n24: f64 = self.ddt_jacobian(s.dn[169][24]);
        let eq87_e1210_d_n25: f64 = self.ddt_jacobian(s.dn[169][25]);
        let eq87_e1210_d_n26: f64 = self.ddt_jacobian(s.dn[169][26]);
        let eq87_e1210_d_n27: f64 = self.ddt_jacobian(s.dn[169][27]);
        let eq87_e1210_d_n28: f64 = self.ddt_jacobian(s.dn[169][28]);
        let eq87_e1210_d_n29: f64 = self.ddt_jacobian(s.dn[169][29]);
        let eq87_e1213: f64 = (p.p355 * (nv2 - nv10));
        let eq87_e1213_d_n2: f64 = p.p355;
        let eq87_e1213_d_n10: f64 = (-p.p355);
        let eq87_e1214: f64 = self.eval_ddt(73, eq87_e1213);
        let eq87_e1214_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n2: f64 = self.ddt_jacobian(eq87_e1213_d_n2);
        let eq87_e1214_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n10: f64 = self.ddt_jacobian(eq87_e1213_d_n10);
        let eq87_e1214_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq87_e1214_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq87_e1215: f64 = (eq87_e1210 + eq87_e1214);
        let eq87_e1215_d_n0: f64 = (eq87_e1210_d_n0 + eq87_e1214_d_n0);
        let eq87_e1215_d_n1: f64 = (eq87_e1210_d_n1 + eq87_e1214_d_n1);
        let eq87_e1215_d_n2: f64 = (eq87_e1210_d_n2 + eq87_e1214_d_n2);
        let eq87_e1215_d_n3: f64 = (eq87_e1210_d_n3 + eq87_e1214_d_n3);
        let eq87_e1215_d_n4: f64 = (eq87_e1210_d_n4 + eq87_e1214_d_n4);
        let eq87_e1215_d_n5: f64 = (eq87_e1210_d_n5 + eq87_e1214_d_n5);
        let eq87_e1215_d_n6: f64 = (eq87_e1210_d_n6 + eq87_e1214_d_n6);
        let eq87_e1215_d_n7: f64 = (eq87_e1210_d_n7 + eq87_e1214_d_n7);
        let eq87_e1215_d_n8: f64 = (eq87_e1210_d_n8 + eq87_e1214_d_n8);
        let eq87_e1215_d_n9: f64 = (eq87_e1210_d_n9 + eq87_e1214_d_n9);
        let eq87_e1215_d_n10: f64 = (eq87_e1210_d_n10 + eq87_e1214_d_n10);
        let eq87_e1215_d_n11: f64 = (eq87_e1210_d_n11 + eq87_e1214_d_n11);
        let eq87_e1215_d_n12: f64 = (eq87_e1210_d_n12 + eq87_e1214_d_n12);
        let eq87_e1215_d_n13: f64 = (eq87_e1210_d_n13 + eq87_e1214_d_n13);
        let eq87_e1215_d_n14: f64 = (eq87_e1210_d_n14 + eq87_e1214_d_n14);
        let eq87_e1215_d_n15: f64 = (eq87_e1210_d_n15 + eq87_e1214_d_n15);
        let eq87_e1215_d_n16: f64 = (eq87_e1210_d_n16 + eq87_e1214_d_n16);
        let eq87_e1215_d_n17: f64 = (eq87_e1210_d_n17 + eq87_e1214_d_n17);
        let eq87_e1215_d_n18: f64 = (eq87_e1210_d_n18 + eq87_e1214_d_n18);
        let eq87_e1215_d_n19: f64 = (eq87_e1210_d_n19 + eq87_e1214_d_n19);
        let eq87_e1215_d_n20: f64 = (eq87_e1210_d_n20 + eq87_e1214_d_n20);
        let eq87_e1215_d_n21: f64 = (eq87_e1210_d_n21 + eq87_e1214_d_n21);
        let eq87_e1215_d_n22: f64 = (eq87_e1210_d_n22 + eq87_e1214_d_n22);
        let eq87_e1215_d_n23: f64 = (eq87_e1210_d_n23 + eq87_e1214_d_n23);
        let eq87_e1215_d_n24: f64 = (eq87_e1210_d_n24 + eq87_e1214_d_n24);
        let eq87_e1215_d_n25: f64 = (eq87_e1210_d_n25 + eq87_e1214_d_n25);
        let eq87_e1215_d_n26: f64 = (eq87_e1210_d_n26 + eq87_e1214_d_n26);
        let eq87_e1215_d_n27: f64 = (eq87_e1210_d_n27 + eq87_e1214_d_n27);
        let eq87_e1215_d_n28: f64 = (eq87_e1210_d_n28 + eq87_e1214_d_n28);
        let eq87_e1215_d_n29: f64 = (eq87_e1210_d_n29 + eq87_e1214_d_n29);
        (eq87_e1215, eq87_e1215_d_n0, eq87_e1215_d_n1, eq87_e1215_d_n2, eq87_e1215_d_n3, eq87_e1215_d_n4, eq87_e1215_d_n5, eq87_e1215_d_n6, eq87_e1215_d_n7, eq87_e1215_d_n8, eq87_e1215_d_n9, eq87_e1215_d_n10, eq87_e1215_d_n11, eq87_e1215_d_n12, eq87_e1215_d_n13, eq87_e1215_d_n14, eq87_e1215_d_n15, eq87_e1215_d_n16, eq87_e1215_d_n17, eq87_e1215_d_n18, eq87_e1215_d_n19, eq87_e1215_d_n20, eq87_e1215_d_n21, eq87_e1215_d_n22, eq87_e1215_d_n23, eq87_e1215_d_n24, eq87_e1215_d_n25, eq87_e1215_d_n26, eq87_e1215_d_n27, eq87_e1215_d_n28, eq87_e1215_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq87_value: f64 = eq87_e1217;
        let eq87_node_derivatives: [f64; 30] = [eq87_e1217_d_n0, eq87_e1217_d_n1, eq87_e1217_d_n2, eq87_e1217_d_n3, eq87_e1217_d_n4, eq87_e1217_d_n5, eq87_e1217_d_n6, eq87_e1217_d_n7, eq87_e1217_d_n8, eq87_e1217_d_n9, eq87_e1217_d_n10, eq87_e1217_d_n11, eq87_e1217_d_n12, eq87_e1217_d_n13, eq87_e1217_d_n14, eq87_e1217_d_n15, eq87_e1217_d_n16, eq87_e1217_d_n17, eq87_e1217_d_n18, eq87_e1217_d_n19, eq87_e1217_d_n20, eq87_e1217_d_n21, eq87_e1217_d_n22, eq87_e1217_d_n23, eq87_e1217_d_n24, eq87_e1217_d_n25, eq87_e1217_d_n26, eq87_e1217_d_n27, eq87_e1217_d_n28, eq87_e1217_d_n29];
        let eq87_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[10]),
            self.multiplicity * (eq87_value),
            &nodes,
            &eq87_node_derivatives,
            &branches,
            &eq87_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_88_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq88_e1221,) = {
    if (s.v[1054] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq88_value: f64 = eq88_e1221;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[9]),
            self.multiplicity * (eq88_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_89_block_0(
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
        let (eq89_e1231, eq89_e1231_d_n0, eq89_e1231_d_n1, eq89_e1231_d_n2, eq89_e1231_d_n3, eq89_e1231_d_n4, eq89_e1231_d_n5, eq89_e1231_d_n6, eq89_e1231_d_n7, eq89_e1231_d_n8, eq89_e1231_d_n9, eq89_e1231_d_n10, eq89_e1231_d_n11, eq89_e1231_d_n12, eq89_e1231_d_n13, eq89_e1231_d_n14, eq89_e1231_d_n15, eq89_e1231_d_n16, eq89_e1231_d_n17, eq89_e1231_d_n18, eq89_e1231_d_n19, eq89_e1231_d_n20, eq89_e1231_d_n21, eq89_e1231_d_n22, eq89_e1231_d_n23, eq89_e1231_d_n24, eq89_e1231_d_n25, eq89_e1231_d_n26, eq89_e1231_d_n27, eq89_e1231_d_n28, eq89_e1231_d_n29,) = {
    if (s.v[1054] != 0.0) {
        let eq89_e1224: f64 = self.eval_ddt(74, s.v[171]);
        let eq89_e1224_d_n0: f64 = self.ddt_jacobian(s.dn[171][0]);
        let eq89_e1224_d_n1: f64 = self.ddt_jacobian(s.dn[171][1]);
        let eq89_e1224_d_n2: f64 = self.ddt_jacobian(s.dn[171][2]);
        let eq89_e1224_d_n3: f64 = self.ddt_jacobian(s.dn[171][3]);
        let eq89_e1224_d_n4: f64 = self.ddt_jacobian(s.dn[171][4]);
        let eq89_e1224_d_n5: f64 = self.ddt_jacobian(s.dn[171][5]);
        let eq89_e1224_d_n6: f64 = self.ddt_jacobian(s.dn[171][6]);
        let eq89_e1224_d_n7: f64 = self.ddt_jacobian(s.dn[171][7]);
        let eq89_e1224_d_n8: f64 = self.ddt_jacobian(s.dn[171][8]);
        let eq89_e1224_d_n9: f64 = self.ddt_jacobian(s.dn[171][9]);
        let eq89_e1224_d_n10: f64 = self.ddt_jacobian(s.dn[171][10]);
        let eq89_e1224_d_n11: f64 = self.ddt_jacobian(s.dn[171][11]);
        let eq89_e1224_d_n12: f64 = self.ddt_jacobian(s.dn[171][12]);
        let eq89_e1224_d_n13: f64 = self.ddt_jacobian(s.dn[171][13]);
        let eq89_e1224_d_n14: f64 = self.ddt_jacobian(s.dn[171][14]);
        let eq89_e1224_d_n15: f64 = self.ddt_jacobian(s.dn[171][15]);
        let eq89_e1224_d_n16: f64 = self.ddt_jacobian(s.dn[171][16]);
        let eq89_e1224_d_n17: f64 = self.ddt_jacobian(s.dn[171][17]);
        let eq89_e1224_d_n18: f64 = self.ddt_jacobian(s.dn[171][18]);
        let eq89_e1224_d_n19: f64 = self.ddt_jacobian(s.dn[171][19]);
        let eq89_e1224_d_n20: f64 = self.ddt_jacobian(s.dn[171][20]);
        let eq89_e1224_d_n21: f64 = self.ddt_jacobian(s.dn[171][21]);
        let eq89_e1224_d_n22: f64 = self.ddt_jacobian(s.dn[171][22]);
        let eq89_e1224_d_n23: f64 = self.ddt_jacobian(s.dn[171][23]);
        let eq89_e1224_d_n24: f64 = self.ddt_jacobian(s.dn[171][24]);
        let eq89_e1224_d_n25: f64 = self.ddt_jacobian(s.dn[171][25]);
        let eq89_e1224_d_n26: f64 = self.ddt_jacobian(s.dn[171][26]);
        let eq89_e1224_d_n27: f64 = self.ddt_jacobian(s.dn[171][27]);
        let eq89_e1224_d_n28: f64 = self.ddt_jacobian(s.dn[171][28]);
        let eq89_e1224_d_n29: f64 = self.ddt_jacobian(s.dn[171][29]);
        let eq89_e1227: f64 = (p.p355 * (nv7 - nv9));
        let eq89_e1227_d_n7: f64 = p.p355;
        let eq89_e1227_d_n9: f64 = (-p.p355);
        let eq89_e1228: f64 = self.eval_ddt(75, eq89_e1227);
        let eq89_e1228_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n7: f64 = self.ddt_jacobian(eq89_e1227_d_n7);
        let eq89_e1228_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n9: f64 = self.ddt_jacobian(eq89_e1227_d_n9);
        let eq89_e1228_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq89_e1228_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq89_e1229: f64 = (eq89_e1224 + eq89_e1228);
        let eq89_e1229_d_n0: f64 = (eq89_e1224_d_n0 + eq89_e1228_d_n0);
        let eq89_e1229_d_n1: f64 = (eq89_e1224_d_n1 + eq89_e1228_d_n1);
        let eq89_e1229_d_n2: f64 = (eq89_e1224_d_n2 + eq89_e1228_d_n2);
        let eq89_e1229_d_n3: f64 = (eq89_e1224_d_n3 + eq89_e1228_d_n3);
        let eq89_e1229_d_n4: f64 = (eq89_e1224_d_n4 + eq89_e1228_d_n4);
        let eq89_e1229_d_n5: f64 = (eq89_e1224_d_n5 + eq89_e1228_d_n5);
        let eq89_e1229_d_n6: f64 = (eq89_e1224_d_n6 + eq89_e1228_d_n6);
        let eq89_e1229_d_n7: f64 = (eq89_e1224_d_n7 + eq89_e1228_d_n7);
        let eq89_e1229_d_n8: f64 = (eq89_e1224_d_n8 + eq89_e1228_d_n8);
        let eq89_e1229_d_n9: f64 = (eq89_e1224_d_n9 + eq89_e1228_d_n9);
        let eq89_e1229_d_n10: f64 = (eq89_e1224_d_n10 + eq89_e1228_d_n10);
        let eq89_e1229_d_n11: f64 = (eq89_e1224_d_n11 + eq89_e1228_d_n11);
        let eq89_e1229_d_n12: f64 = (eq89_e1224_d_n12 + eq89_e1228_d_n12);
        let eq89_e1229_d_n13: f64 = (eq89_e1224_d_n13 + eq89_e1228_d_n13);
        let eq89_e1229_d_n14: f64 = (eq89_e1224_d_n14 + eq89_e1228_d_n14);
        let eq89_e1229_d_n15: f64 = (eq89_e1224_d_n15 + eq89_e1228_d_n15);
        let eq89_e1229_d_n16: f64 = (eq89_e1224_d_n16 + eq89_e1228_d_n16);
        let eq89_e1229_d_n17: f64 = (eq89_e1224_d_n17 + eq89_e1228_d_n17);
        let eq89_e1229_d_n18: f64 = (eq89_e1224_d_n18 + eq89_e1228_d_n18);
        let eq89_e1229_d_n19: f64 = (eq89_e1224_d_n19 + eq89_e1228_d_n19);
        let eq89_e1229_d_n20: f64 = (eq89_e1224_d_n20 + eq89_e1228_d_n20);
        let eq89_e1229_d_n21: f64 = (eq89_e1224_d_n21 + eq89_e1228_d_n21);
        let eq89_e1229_d_n22: f64 = (eq89_e1224_d_n22 + eq89_e1228_d_n22);
        let eq89_e1229_d_n23: f64 = (eq89_e1224_d_n23 + eq89_e1228_d_n23);
        let eq89_e1229_d_n24: f64 = (eq89_e1224_d_n24 + eq89_e1228_d_n24);
        let eq89_e1229_d_n25: f64 = (eq89_e1224_d_n25 + eq89_e1228_d_n25);
        let eq89_e1229_d_n26: f64 = (eq89_e1224_d_n26 + eq89_e1228_d_n26);
        let eq89_e1229_d_n27: f64 = (eq89_e1224_d_n27 + eq89_e1228_d_n27);
        let eq89_e1229_d_n28: f64 = (eq89_e1224_d_n28 + eq89_e1228_d_n28);
        let eq89_e1229_d_n29: f64 = (eq89_e1224_d_n29 + eq89_e1228_d_n29);
        (eq89_e1229, eq89_e1229_d_n0, eq89_e1229_d_n1, eq89_e1229_d_n2, eq89_e1229_d_n3, eq89_e1229_d_n4, eq89_e1229_d_n5, eq89_e1229_d_n6, eq89_e1229_d_n7, eq89_e1229_d_n8, eq89_e1229_d_n9, eq89_e1229_d_n10, eq89_e1229_d_n11, eq89_e1229_d_n12, eq89_e1229_d_n13, eq89_e1229_d_n14, eq89_e1229_d_n15, eq89_e1229_d_n16, eq89_e1229_d_n17, eq89_e1229_d_n18, eq89_e1229_d_n19, eq89_e1229_d_n20, eq89_e1229_d_n21, eq89_e1229_d_n22, eq89_e1229_d_n23, eq89_e1229_d_n24, eq89_e1229_d_n25, eq89_e1229_d_n26, eq89_e1229_d_n27, eq89_e1229_d_n28, eq89_e1229_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq89_value: f64 = eq89_e1231;
        let eq89_node_derivatives: [f64; 30] = [eq89_e1231_d_n0, eq89_e1231_d_n1, eq89_e1231_d_n2, eq89_e1231_d_n3, eq89_e1231_d_n4, eq89_e1231_d_n5, eq89_e1231_d_n6, eq89_e1231_d_n7, eq89_e1231_d_n8, eq89_e1231_d_n9, eq89_e1231_d_n10, eq89_e1231_d_n11, eq89_e1231_d_n12, eq89_e1231_d_n13, eq89_e1231_d_n14, eq89_e1231_d_n15, eq89_e1231_d_n16, eq89_e1231_d_n17, eq89_e1231_d_n18, eq89_e1231_d_n19, eq89_e1231_d_n20, eq89_e1231_d_n21, eq89_e1231_d_n22, eq89_e1231_d_n23, eq89_e1231_d_n24, eq89_e1231_d_n25, eq89_e1231_d_n26, eq89_e1231_d_n27, eq89_e1231_d_n28, eq89_e1231_d_n29];
        let eq89_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            self.multiplicity * (eq89_value),
            &nodes,
            &eq89_node_derivatives,
            &branches,
            &eq89_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_90_block_0(
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
        let (eq90_e1242, eq90_e1242_d_n0, eq90_e1242_d_n1, eq90_e1242_d_n2, eq90_e1242_d_n3, eq90_e1242_d_n4, eq90_e1242_d_n5, eq90_e1242_d_n6, eq90_e1242_d_n7, eq90_e1242_d_n8, eq90_e1242_d_n9, eq90_e1242_d_n10, eq90_e1242_d_n11, eq90_e1242_d_n12, eq90_e1242_d_n13, eq90_e1242_d_n14, eq90_e1242_d_n15, eq90_e1242_d_n16, eq90_e1242_d_n17, eq90_e1242_d_n18, eq90_e1242_d_n19, eq90_e1242_d_n20, eq90_e1242_d_n21, eq90_e1242_d_n22, eq90_e1242_d_n23, eq90_e1242_d_n24, eq90_e1242_d_n25, eq90_e1242_d_n26, eq90_e1242_d_n27, eq90_e1242_d_n28, eq90_e1242_d_n29,) = {
    if (!(s.v[1054] != 0.0)) {
        let eq90_e1235: f64 = self.eval_ddt(76, s.v[167]);
        let eq90_e1235_d_n0: f64 = self.ddt_jacobian(s.dn[167][0]);
        let eq90_e1235_d_n1: f64 = self.ddt_jacobian(s.dn[167][1]);
        let eq90_e1235_d_n2: f64 = self.ddt_jacobian(s.dn[167][2]);
        let eq90_e1235_d_n3: f64 = self.ddt_jacobian(s.dn[167][3]);
        let eq90_e1235_d_n4: f64 = self.ddt_jacobian(s.dn[167][4]);
        let eq90_e1235_d_n5: f64 = self.ddt_jacobian(s.dn[167][5]);
        let eq90_e1235_d_n6: f64 = self.ddt_jacobian(s.dn[167][6]);
        let eq90_e1235_d_n7: f64 = self.ddt_jacobian(s.dn[167][7]);
        let eq90_e1235_d_n8: f64 = self.ddt_jacobian(s.dn[167][8]);
        let eq90_e1235_d_n9: f64 = self.ddt_jacobian(s.dn[167][9]);
        let eq90_e1235_d_n10: f64 = self.ddt_jacobian(s.dn[167][10]);
        let eq90_e1235_d_n11: f64 = self.ddt_jacobian(s.dn[167][11]);
        let eq90_e1235_d_n12: f64 = self.ddt_jacobian(s.dn[167][12]);
        let eq90_e1235_d_n13: f64 = self.ddt_jacobian(s.dn[167][13]);
        let eq90_e1235_d_n14: f64 = self.ddt_jacobian(s.dn[167][14]);
        let eq90_e1235_d_n15: f64 = self.ddt_jacobian(s.dn[167][15]);
        let eq90_e1235_d_n16: f64 = self.ddt_jacobian(s.dn[167][16]);
        let eq90_e1235_d_n17: f64 = self.ddt_jacobian(s.dn[167][17]);
        let eq90_e1235_d_n18: f64 = self.ddt_jacobian(s.dn[167][18]);
        let eq90_e1235_d_n19: f64 = self.ddt_jacobian(s.dn[167][19]);
        let eq90_e1235_d_n20: f64 = self.ddt_jacobian(s.dn[167][20]);
        let eq90_e1235_d_n21: f64 = self.ddt_jacobian(s.dn[167][21]);
        let eq90_e1235_d_n22: f64 = self.ddt_jacobian(s.dn[167][22]);
        let eq90_e1235_d_n23: f64 = self.ddt_jacobian(s.dn[167][23]);
        let eq90_e1235_d_n24: f64 = self.ddt_jacobian(s.dn[167][24]);
        let eq90_e1235_d_n25: f64 = self.ddt_jacobian(s.dn[167][25]);
        let eq90_e1235_d_n26: f64 = self.ddt_jacobian(s.dn[167][26]);
        let eq90_e1235_d_n27: f64 = self.ddt_jacobian(s.dn[167][27]);
        let eq90_e1235_d_n28: f64 = self.ddt_jacobian(s.dn[167][28]);
        let eq90_e1235_d_n29: f64 = self.ddt_jacobian(s.dn[167][29]);
        let eq90_e1238: f64 = (p.p355 * (nv2 - nv10));
        let eq90_e1238_d_n2: f64 = p.p355;
        let eq90_e1238_d_n10: f64 = (-p.p355);
        let eq90_e1239: f64 = self.eval_ddt(77, eq90_e1238);
        let eq90_e1239_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n2: f64 = self.ddt_jacobian(eq90_e1238_d_n2);
        let eq90_e1239_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n10: f64 = self.ddt_jacobian(eq90_e1238_d_n10);
        let eq90_e1239_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq90_e1239_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq90_e1240: f64 = (eq90_e1235 + eq90_e1239);
        let eq90_e1240_d_n0: f64 = (eq90_e1235_d_n0 + eq90_e1239_d_n0);
        let eq90_e1240_d_n1: f64 = (eq90_e1235_d_n1 + eq90_e1239_d_n1);
        let eq90_e1240_d_n2: f64 = (eq90_e1235_d_n2 + eq90_e1239_d_n2);
        let eq90_e1240_d_n3: f64 = (eq90_e1235_d_n3 + eq90_e1239_d_n3);
        let eq90_e1240_d_n4: f64 = (eq90_e1235_d_n4 + eq90_e1239_d_n4);
        let eq90_e1240_d_n5: f64 = (eq90_e1235_d_n5 + eq90_e1239_d_n5);
        let eq90_e1240_d_n6: f64 = (eq90_e1235_d_n6 + eq90_e1239_d_n6);
        let eq90_e1240_d_n7: f64 = (eq90_e1235_d_n7 + eq90_e1239_d_n7);
        let eq90_e1240_d_n8: f64 = (eq90_e1235_d_n8 + eq90_e1239_d_n8);
        let eq90_e1240_d_n9: f64 = (eq90_e1235_d_n9 + eq90_e1239_d_n9);
        let eq90_e1240_d_n10: f64 = (eq90_e1235_d_n10 + eq90_e1239_d_n10);
        let eq90_e1240_d_n11: f64 = (eq90_e1235_d_n11 + eq90_e1239_d_n11);
        let eq90_e1240_d_n12: f64 = (eq90_e1235_d_n12 + eq90_e1239_d_n12);
        let eq90_e1240_d_n13: f64 = (eq90_e1235_d_n13 + eq90_e1239_d_n13);
        let eq90_e1240_d_n14: f64 = (eq90_e1235_d_n14 + eq90_e1239_d_n14);
        let eq90_e1240_d_n15: f64 = (eq90_e1235_d_n15 + eq90_e1239_d_n15);
        let eq90_e1240_d_n16: f64 = (eq90_e1235_d_n16 + eq90_e1239_d_n16);
        let eq90_e1240_d_n17: f64 = (eq90_e1235_d_n17 + eq90_e1239_d_n17);
        let eq90_e1240_d_n18: f64 = (eq90_e1235_d_n18 + eq90_e1239_d_n18);
        let eq90_e1240_d_n19: f64 = (eq90_e1235_d_n19 + eq90_e1239_d_n19);
        let eq90_e1240_d_n20: f64 = (eq90_e1235_d_n20 + eq90_e1239_d_n20);
        let eq90_e1240_d_n21: f64 = (eq90_e1235_d_n21 + eq90_e1239_d_n21);
        let eq90_e1240_d_n22: f64 = (eq90_e1235_d_n22 + eq90_e1239_d_n22);
        let eq90_e1240_d_n23: f64 = (eq90_e1235_d_n23 + eq90_e1239_d_n23);
        let eq90_e1240_d_n24: f64 = (eq90_e1235_d_n24 + eq90_e1239_d_n24);
        let eq90_e1240_d_n25: f64 = (eq90_e1235_d_n25 + eq90_e1239_d_n25);
        let eq90_e1240_d_n26: f64 = (eq90_e1235_d_n26 + eq90_e1239_d_n26);
        let eq90_e1240_d_n27: f64 = (eq90_e1235_d_n27 + eq90_e1239_d_n27);
        let eq90_e1240_d_n28: f64 = (eq90_e1235_d_n28 + eq90_e1239_d_n28);
        let eq90_e1240_d_n29: f64 = (eq90_e1235_d_n29 + eq90_e1239_d_n29);
        (eq90_e1240, eq90_e1240_d_n0, eq90_e1240_d_n1, eq90_e1240_d_n2, eq90_e1240_d_n3, eq90_e1240_d_n4, eq90_e1240_d_n5, eq90_e1240_d_n6, eq90_e1240_d_n7, eq90_e1240_d_n8, eq90_e1240_d_n9, eq90_e1240_d_n10, eq90_e1240_d_n11, eq90_e1240_d_n12, eq90_e1240_d_n13, eq90_e1240_d_n14, eq90_e1240_d_n15, eq90_e1240_d_n16, eq90_e1240_d_n17, eq90_e1240_d_n18, eq90_e1240_d_n19, eq90_e1240_d_n20, eq90_e1240_d_n21, eq90_e1240_d_n22, eq90_e1240_d_n23, eq90_e1240_d_n24, eq90_e1240_d_n25, eq90_e1240_d_n26, eq90_e1240_d_n27, eq90_e1240_d_n28, eq90_e1240_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq90_value: f64 = eq90_e1242;
        let eq90_node_derivatives: [f64; 30] = [eq90_e1242_d_n0, eq90_e1242_d_n1, eq90_e1242_d_n2, eq90_e1242_d_n3, eq90_e1242_d_n4, eq90_e1242_d_n5, eq90_e1242_d_n6, eq90_e1242_d_n7, eq90_e1242_d_n8, eq90_e1242_d_n9, eq90_e1242_d_n10, eq90_e1242_d_n11, eq90_e1242_d_n12, eq90_e1242_d_n13, eq90_e1242_d_n14, eq90_e1242_d_n15, eq90_e1242_d_n16, eq90_e1242_d_n17, eq90_e1242_d_n18, eq90_e1242_d_n19, eq90_e1242_d_n20, eq90_e1242_d_n21, eq90_e1242_d_n22, eq90_e1242_d_n23, eq90_e1242_d_n24, eq90_e1242_d_n25, eq90_e1242_d_n26, eq90_e1242_d_n27, eq90_e1242_d_n28, eq90_e1242_d_n29];
        let eq90_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[10]),
            self.multiplicity * (eq90_value),
            &nodes,
            &eq90_node_derivatives,
            &branches,
            &eq90_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_91_block_0(
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
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq91_e1253, eq91_e1253_d_n0, eq91_e1253_d_n1, eq91_e1253_d_n2, eq91_e1253_d_n3, eq91_e1253_d_n4, eq91_e1253_d_n5, eq91_e1253_d_n6, eq91_e1253_d_n7, eq91_e1253_d_n8, eq91_e1253_d_n9, eq91_e1253_d_n10, eq91_e1253_d_n11, eq91_e1253_d_n12, eq91_e1253_d_n13, eq91_e1253_d_n14, eq91_e1253_d_n15, eq91_e1253_d_n16, eq91_e1253_d_n17, eq91_e1253_d_n18, eq91_e1253_d_n19, eq91_e1253_d_n20, eq91_e1253_d_n21, eq91_e1253_d_n22, eq91_e1253_d_n23, eq91_e1253_d_n24, eq91_e1253_d_n25, eq91_e1253_d_n26, eq91_e1253_d_n27, eq91_e1253_d_n28, eq91_e1253_d_n29,) = {
    if (!(s.v[1054] != 0.0)) {
        let eq91_e1246: f64 = self.eval_ddt(78, s.v[168]);
        let eq91_e1246_d_n0: f64 = self.ddt_jacobian(s.dn[168][0]);
        let eq91_e1246_d_n1: f64 = self.ddt_jacobian(s.dn[168][1]);
        let eq91_e1246_d_n2: f64 = self.ddt_jacobian(s.dn[168][2]);
        let eq91_e1246_d_n3: f64 = self.ddt_jacobian(s.dn[168][3]);
        let eq91_e1246_d_n4: f64 = self.ddt_jacobian(s.dn[168][4]);
        let eq91_e1246_d_n5: f64 = self.ddt_jacobian(s.dn[168][5]);
        let eq91_e1246_d_n6: f64 = self.ddt_jacobian(s.dn[168][6]);
        let eq91_e1246_d_n7: f64 = self.ddt_jacobian(s.dn[168][7]);
        let eq91_e1246_d_n8: f64 = self.ddt_jacobian(s.dn[168][8]);
        let eq91_e1246_d_n9: f64 = self.ddt_jacobian(s.dn[168][9]);
        let eq91_e1246_d_n10: f64 = self.ddt_jacobian(s.dn[168][10]);
        let eq91_e1246_d_n11: f64 = self.ddt_jacobian(s.dn[168][11]);
        let eq91_e1246_d_n12: f64 = self.ddt_jacobian(s.dn[168][12]);
        let eq91_e1246_d_n13: f64 = self.ddt_jacobian(s.dn[168][13]);
        let eq91_e1246_d_n14: f64 = self.ddt_jacobian(s.dn[168][14]);
        let eq91_e1246_d_n15: f64 = self.ddt_jacobian(s.dn[168][15]);
        let eq91_e1246_d_n16: f64 = self.ddt_jacobian(s.dn[168][16]);
        let eq91_e1246_d_n17: f64 = self.ddt_jacobian(s.dn[168][17]);
        let eq91_e1246_d_n18: f64 = self.ddt_jacobian(s.dn[168][18]);
        let eq91_e1246_d_n19: f64 = self.ddt_jacobian(s.dn[168][19]);
        let eq91_e1246_d_n20: f64 = self.ddt_jacobian(s.dn[168][20]);
        let eq91_e1246_d_n21: f64 = self.ddt_jacobian(s.dn[168][21]);
        let eq91_e1246_d_n22: f64 = self.ddt_jacobian(s.dn[168][22]);
        let eq91_e1246_d_n23: f64 = self.ddt_jacobian(s.dn[168][23]);
        let eq91_e1246_d_n24: f64 = self.ddt_jacobian(s.dn[168][24]);
        let eq91_e1246_d_n25: f64 = self.ddt_jacobian(s.dn[168][25]);
        let eq91_e1246_d_n26: f64 = self.ddt_jacobian(s.dn[168][26]);
        let eq91_e1246_d_n27: f64 = self.ddt_jacobian(s.dn[168][27]);
        let eq91_e1246_d_n28: f64 = self.ddt_jacobian(s.dn[168][28]);
        let eq91_e1246_d_n29: f64 = self.ddt_jacobian(s.dn[168][29]);
        let eq91_e1249: f64 = (p.p355 * (nv2 - nv9));
        let eq91_e1249_d_n2: f64 = p.p355;
        let eq91_e1249_d_n9: f64 = (-p.p355);
        let eq91_e1250: f64 = self.eval_ddt(79, eq91_e1249);
        let eq91_e1250_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n2: f64 = self.ddt_jacobian(eq91_e1249_d_n2);
        let eq91_e1250_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n9: f64 = self.ddt_jacobian(eq91_e1249_d_n9);
        let eq91_e1250_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq91_e1250_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq91_e1251: f64 = (eq91_e1246 + eq91_e1250);
        let eq91_e1251_d_n0: f64 = (eq91_e1246_d_n0 + eq91_e1250_d_n0);
        let eq91_e1251_d_n1: f64 = (eq91_e1246_d_n1 + eq91_e1250_d_n1);
        let eq91_e1251_d_n2: f64 = (eq91_e1246_d_n2 + eq91_e1250_d_n2);
        let eq91_e1251_d_n3: f64 = (eq91_e1246_d_n3 + eq91_e1250_d_n3);
        let eq91_e1251_d_n4: f64 = (eq91_e1246_d_n4 + eq91_e1250_d_n4);
        let eq91_e1251_d_n5: f64 = (eq91_e1246_d_n5 + eq91_e1250_d_n5);
        let eq91_e1251_d_n6: f64 = (eq91_e1246_d_n6 + eq91_e1250_d_n6);
        let eq91_e1251_d_n7: f64 = (eq91_e1246_d_n7 + eq91_e1250_d_n7);
        let eq91_e1251_d_n8: f64 = (eq91_e1246_d_n8 + eq91_e1250_d_n8);
        let eq91_e1251_d_n9: f64 = (eq91_e1246_d_n9 + eq91_e1250_d_n9);
        let eq91_e1251_d_n10: f64 = (eq91_e1246_d_n10 + eq91_e1250_d_n10);
        let eq91_e1251_d_n11: f64 = (eq91_e1246_d_n11 + eq91_e1250_d_n11);
        let eq91_e1251_d_n12: f64 = (eq91_e1246_d_n12 + eq91_e1250_d_n12);
        let eq91_e1251_d_n13: f64 = (eq91_e1246_d_n13 + eq91_e1250_d_n13);
        let eq91_e1251_d_n14: f64 = (eq91_e1246_d_n14 + eq91_e1250_d_n14);
        let eq91_e1251_d_n15: f64 = (eq91_e1246_d_n15 + eq91_e1250_d_n15);
        let eq91_e1251_d_n16: f64 = (eq91_e1246_d_n16 + eq91_e1250_d_n16);
        let eq91_e1251_d_n17: f64 = (eq91_e1246_d_n17 + eq91_e1250_d_n17);
        let eq91_e1251_d_n18: f64 = (eq91_e1246_d_n18 + eq91_e1250_d_n18);
        let eq91_e1251_d_n19: f64 = (eq91_e1246_d_n19 + eq91_e1250_d_n19);
        let eq91_e1251_d_n20: f64 = (eq91_e1246_d_n20 + eq91_e1250_d_n20);
        let eq91_e1251_d_n21: f64 = (eq91_e1246_d_n21 + eq91_e1250_d_n21);
        let eq91_e1251_d_n22: f64 = (eq91_e1246_d_n22 + eq91_e1250_d_n22);
        let eq91_e1251_d_n23: f64 = (eq91_e1246_d_n23 + eq91_e1250_d_n23);
        let eq91_e1251_d_n24: f64 = (eq91_e1246_d_n24 + eq91_e1250_d_n24);
        let eq91_e1251_d_n25: f64 = (eq91_e1246_d_n25 + eq91_e1250_d_n25);
        let eq91_e1251_d_n26: f64 = (eq91_e1246_d_n26 + eq91_e1250_d_n26);
        let eq91_e1251_d_n27: f64 = (eq91_e1246_d_n27 + eq91_e1250_d_n27);
        let eq91_e1251_d_n28: f64 = (eq91_e1246_d_n28 + eq91_e1250_d_n28);
        let eq91_e1251_d_n29: f64 = (eq91_e1246_d_n29 + eq91_e1250_d_n29);
        (eq91_e1251, eq91_e1251_d_n0, eq91_e1251_d_n1, eq91_e1251_d_n2, eq91_e1251_d_n3, eq91_e1251_d_n4, eq91_e1251_d_n5, eq91_e1251_d_n6, eq91_e1251_d_n7, eq91_e1251_d_n8, eq91_e1251_d_n9, eq91_e1251_d_n10, eq91_e1251_d_n11, eq91_e1251_d_n12, eq91_e1251_d_n13, eq91_e1251_d_n14, eq91_e1251_d_n15, eq91_e1251_d_n16, eq91_e1251_d_n17, eq91_e1251_d_n18, eq91_e1251_d_n19, eq91_e1251_d_n20, eq91_e1251_d_n21, eq91_e1251_d_n22, eq91_e1251_d_n23, eq91_e1251_d_n24, eq91_e1251_d_n25, eq91_e1251_d_n26, eq91_e1251_d_n27, eq91_e1251_d_n28, eq91_e1251_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq91_value: f64 = eq91_e1253;
        let eq91_node_derivatives: [f64; 30] = [eq91_e1253_d_n0, eq91_e1253_d_n1, eq91_e1253_d_n2, eq91_e1253_d_n3, eq91_e1253_d_n4, eq91_e1253_d_n5, eq91_e1253_d_n6, eq91_e1253_d_n7, eq91_e1253_d_n8, eq91_e1253_d_n9, eq91_e1253_d_n10, eq91_e1253_d_n11, eq91_e1253_d_n12, eq91_e1253_d_n13, eq91_e1253_d_n14, eq91_e1253_d_n15, eq91_e1253_d_n16, eq91_e1253_d_n17, eq91_e1253_d_n18, eq91_e1253_d_n19, eq91_e1253_d_n20, eq91_e1253_d_n21, eq91_e1253_d_n22, eq91_e1253_d_n23, eq91_e1253_d_n24, eq91_e1253_d_n25, eq91_e1253_d_n26, eq91_e1253_d_n27, eq91_e1253_d_n28, eq91_e1253_d_n29];
        let eq91_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[9]),
            self.multiplicity * (eq91_value),
            &nodes,
            &eq91_node_derivatives,
            &branches,
            &eq91_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_92_block_0(
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq92_e1264, eq92_e1264_d_n0, eq92_e1264_d_n1, eq92_e1264_d_n2, eq92_e1264_d_n3, eq92_e1264_d_n4, eq92_e1264_d_n5, eq92_e1264_d_n6, eq92_e1264_d_n7, eq92_e1264_d_n8, eq92_e1264_d_n9, eq92_e1264_d_n10, eq92_e1264_d_n11, eq92_e1264_d_n12, eq92_e1264_d_n13, eq92_e1264_d_n14, eq92_e1264_d_n15, eq92_e1264_d_n16, eq92_e1264_d_n17, eq92_e1264_d_n18, eq92_e1264_d_n19, eq92_e1264_d_n20, eq92_e1264_d_n21, eq92_e1264_d_n22, eq92_e1264_d_n23, eq92_e1264_d_n24, eq92_e1264_d_n25, eq92_e1264_d_n26, eq92_e1264_d_n27, eq92_e1264_d_n28, eq92_e1264_d_n29,) = {
    if (!(s.v[1054] != 0.0)) {
        let eq92_e1257: f64 = self.eval_ddt(80, s.v[169]);
        let eq92_e1257_d_n0: f64 = self.ddt_jacobian(s.dn[169][0]);
        let eq92_e1257_d_n1: f64 = self.ddt_jacobian(s.dn[169][1]);
        let eq92_e1257_d_n2: f64 = self.ddt_jacobian(s.dn[169][2]);
        let eq92_e1257_d_n3: f64 = self.ddt_jacobian(s.dn[169][3]);
        let eq92_e1257_d_n4: f64 = self.ddt_jacobian(s.dn[169][4]);
        let eq92_e1257_d_n5: f64 = self.ddt_jacobian(s.dn[169][5]);
        let eq92_e1257_d_n6: f64 = self.ddt_jacobian(s.dn[169][6]);
        let eq92_e1257_d_n7: f64 = self.ddt_jacobian(s.dn[169][7]);
        let eq92_e1257_d_n8: f64 = self.ddt_jacobian(s.dn[169][8]);
        let eq92_e1257_d_n9: f64 = self.ddt_jacobian(s.dn[169][9]);
        let eq92_e1257_d_n10: f64 = self.ddt_jacobian(s.dn[169][10]);
        let eq92_e1257_d_n11: f64 = self.ddt_jacobian(s.dn[169][11]);
        let eq92_e1257_d_n12: f64 = self.ddt_jacobian(s.dn[169][12]);
        let eq92_e1257_d_n13: f64 = self.ddt_jacobian(s.dn[169][13]);
        let eq92_e1257_d_n14: f64 = self.ddt_jacobian(s.dn[169][14]);
        let eq92_e1257_d_n15: f64 = self.ddt_jacobian(s.dn[169][15]);
        let eq92_e1257_d_n16: f64 = self.ddt_jacobian(s.dn[169][16]);
        let eq92_e1257_d_n17: f64 = self.ddt_jacobian(s.dn[169][17]);
        let eq92_e1257_d_n18: f64 = self.ddt_jacobian(s.dn[169][18]);
        let eq92_e1257_d_n19: f64 = self.ddt_jacobian(s.dn[169][19]);
        let eq92_e1257_d_n20: f64 = self.ddt_jacobian(s.dn[169][20]);
        let eq92_e1257_d_n21: f64 = self.ddt_jacobian(s.dn[169][21]);
        let eq92_e1257_d_n22: f64 = self.ddt_jacobian(s.dn[169][22]);
        let eq92_e1257_d_n23: f64 = self.ddt_jacobian(s.dn[169][23]);
        let eq92_e1257_d_n24: f64 = self.ddt_jacobian(s.dn[169][24]);
        let eq92_e1257_d_n25: f64 = self.ddt_jacobian(s.dn[169][25]);
        let eq92_e1257_d_n26: f64 = self.ddt_jacobian(s.dn[169][26]);
        let eq92_e1257_d_n27: f64 = self.ddt_jacobian(s.dn[169][27]);
        let eq92_e1257_d_n28: f64 = self.ddt_jacobian(s.dn[169][28]);
        let eq92_e1257_d_n29: f64 = self.ddt_jacobian(s.dn[169][29]);
        let eq92_e1260: f64 = (p.p355 * (nv7 - nv10));
        let eq92_e1260_d_n7: f64 = p.p355;
        let eq92_e1260_d_n10: f64 = (-p.p355);
        let eq92_e1261: f64 = self.eval_ddt(81, eq92_e1260);
        let eq92_e1261_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n7: f64 = self.ddt_jacobian(eq92_e1260_d_n7);
        let eq92_e1261_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n10: f64 = self.ddt_jacobian(eq92_e1260_d_n10);
        let eq92_e1261_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq92_e1261_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq92_e1262: f64 = (eq92_e1257 + eq92_e1261);
        let eq92_e1262_d_n0: f64 = (eq92_e1257_d_n0 + eq92_e1261_d_n0);
        let eq92_e1262_d_n1: f64 = (eq92_e1257_d_n1 + eq92_e1261_d_n1);
        let eq92_e1262_d_n2: f64 = (eq92_e1257_d_n2 + eq92_e1261_d_n2);
        let eq92_e1262_d_n3: f64 = (eq92_e1257_d_n3 + eq92_e1261_d_n3);
        let eq92_e1262_d_n4: f64 = (eq92_e1257_d_n4 + eq92_e1261_d_n4);
        let eq92_e1262_d_n5: f64 = (eq92_e1257_d_n5 + eq92_e1261_d_n5);
        let eq92_e1262_d_n6: f64 = (eq92_e1257_d_n6 + eq92_e1261_d_n6);
        let eq92_e1262_d_n7: f64 = (eq92_e1257_d_n7 + eq92_e1261_d_n7);
        let eq92_e1262_d_n8: f64 = (eq92_e1257_d_n8 + eq92_e1261_d_n8);
        let eq92_e1262_d_n9: f64 = (eq92_e1257_d_n9 + eq92_e1261_d_n9);
        let eq92_e1262_d_n10: f64 = (eq92_e1257_d_n10 + eq92_e1261_d_n10);
        let eq92_e1262_d_n11: f64 = (eq92_e1257_d_n11 + eq92_e1261_d_n11);
        let eq92_e1262_d_n12: f64 = (eq92_e1257_d_n12 + eq92_e1261_d_n12);
        let eq92_e1262_d_n13: f64 = (eq92_e1257_d_n13 + eq92_e1261_d_n13);
        let eq92_e1262_d_n14: f64 = (eq92_e1257_d_n14 + eq92_e1261_d_n14);
        let eq92_e1262_d_n15: f64 = (eq92_e1257_d_n15 + eq92_e1261_d_n15);
        let eq92_e1262_d_n16: f64 = (eq92_e1257_d_n16 + eq92_e1261_d_n16);
        let eq92_e1262_d_n17: f64 = (eq92_e1257_d_n17 + eq92_e1261_d_n17);
        let eq92_e1262_d_n18: f64 = (eq92_e1257_d_n18 + eq92_e1261_d_n18);
        let eq92_e1262_d_n19: f64 = (eq92_e1257_d_n19 + eq92_e1261_d_n19);
        let eq92_e1262_d_n20: f64 = (eq92_e1257_d_n20 + eq92_e1261_d_n20);
        let eq92_e1262_d_n21: f64 = (eq92_e1257_d_n21 + eq92_e1261_d_n21);
        let eq92_e1262_d_n22: f64 = (eq92_e1257_d_n22 + eq92_e1261_d_n22);
        let eq92_e1262_d_n23: f64 = (eq92_e1257_d_n23 + eq92_e1261_d_n23);
        let eq92_e1262_d_n24: f64 = (eq92_e1257_d_n24 + eq92_e1261_d_n24);
        let eq92_e1262_d_n25: f64 = (eq92_e1257_d_n25 + eq92_e1261_d_n25);
        let eq92_e1262_d_n26: f64 = (eq92_e1257_d_n26 + eq92_e1261_d_n26);
        let eq92_e1262_d_n27: f64 = (eq92_e1257_d_n27 + eq92_e1261_d_n27);
        let eq92_e1262_d_n28: f64 = (eq92_e1257_d_n28 + eq92_e1261_d_n28);
        let eq92_e1262_d_n29: f64 = (eq92_e1257_d_n29 + eq92_e1261_d_n29);
        (eq92_e1262, eq92_e1262_d_n0, eq92_e1262_d_n1, eq92_e1262_d_n2, eq92_e1262_d_n3, eq92_e1262_d_n4, eq92_e1262_d_n5, eq92_e1262_d_n6, eq92_e1262_d_n7, eq92_e1262_d_n8, eq92_e1262_d_n9, eq92_e1262_d_n10, eq92_e1262_d_n11, eq92_e1262_d_n12, eq92_e1262_d_n13, eq92_e1262_d_n14, eq92_e1262_d_n15, eq92_e1262_d_n16, eq92_e1262_d_n17, eq92_e1262_d_n18, eq92_e1262_d_n19, eq92_e1262_d_n20, eq92_e1262_d_n21, eq92_e1262_d_n22, eq92_e1262_d_n23, eq92_e1262_d_n24, eq92_e1262_d_n25, eq92_e1262_d_n26, eq92_e1262_d_n27, eq92_e1262_d_n28, eq92_e1262_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq92_value: f64 = eq92_e1264;
        let eq92_node_derivatives: [f64; 30] = [eq92_e1264_d_n0, eq92_e1264_d_n1, eq92_e1264_d_n2, eq92_e1264_d_n3, eq92_e1264_d_n4, eq92_e1264_d_n5, eq92_e1264_d_n6, eq92_e1264_d_n7, eq92_e1264_d_n8, eq92_e1264_d_n9, eq92_e1264_d_n10, eq92_e1264_d_n11, eq92_e1264_d_n12, eq92_e1264_d_n13, eq92_e1264_d_n14, eq92_e1264_d_n15, eq92_e1264_d_n16, eq92_e1264_d_n17, eq92_e1264_d_n18, eq92_e1264_d_n19, eq92_e1264_d_n20, eq92_e1264_d_n21, eq92_e1264_d_n22, eq92_e1264_d_n23, eq92_e1264_d_n24, eq92_e1264_d_n25, eq92_e1264_d_n26, eq92_e1264_d_n27, eq92_e1264_d_n28, eq92_e1264_d_n29];
        let eq92_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            self.multiplicity * (eq92_value),
            &nodes,
            &eq92_node_derivatives,
            &branches,
            &eq92_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_93_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq93_e1269,) = {
    if (!(s.v[1054] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq93_value: f64 = eq93_e1269;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[9]),
            self.multiplicity * (eq93_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_94_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq94_e1274,) = {
    if (!(s.v[1054] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq94_value: f64 = eq94_e1274;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[9]),
            self.multiplicity * (eq94_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_95_block_0(
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq95_e1276: f64 = self.eval_ddt(82, s.v[170]);
        let eq95_e1276_d_n0: f64 = self.ddt_jacobian(s.dn[170][0]);
        let eq95_e1276_d_n1: f64 = self.ddt_jacobian(s.dn[170][1]);
        let eq95_e1276_d_n2: f64 = self.ddt_jacobian(s.dn[170][2]);
        let eq95_e1276_d_n3: f64 = self.ddt_jacobian(s.dn[170][3]);
        let eq95_e1276_d_n4: f64 = self.ddt_jacobian(s.dn[170][4]);
        let eq95_e1276_d_n5: f64 = self.ddt_jacobian(s.dn[170][5]);
        let eq95_e1276_d_n6: f64 = self.ddt_jacobian(s.dn[170][6]);
        let eq95_e1276_d_n7: f64 = self.ddt_jacobian(s.dn[170][7]);
        let eq95_e1276_d_n8: f64 = self.ddt_jacobian(s.dn[170][8]);
        let eq95_e1276_d_n9: f64 = self.ddt_jacobian(s.dn[170][9]);
        let eq95_e1276_d_n10: f64 = self.ddt_jacobian(s.dn[170][10]);
        let eq95_e1276_d_n11: f64 = self.ddt_jacobian(s.dn[170][11]);
        let eq95_e1276_d_n12: f64 = self.ddt_jacobian(s.dn[170][12]);
        let eq95_e1276_d_n13: f64 = self.ddt_jacobian(s.dn[170][13]);
        let eq95_e1276_d_n14: f64 = self.ddt_jacobian(s.dn[170][14]);
        let eq95_e1276_d_n15: f64 = self.ddt_jacobian(s.dn[170][15]);
        let eq95_e1276_d_n16: f64 = self.ddt_jacobian(s.dn[170][16]);
        let eq95_e1276_d_n17: f64 = self.ddt_jacobian(s.dn[170][17]);
        let eq95_e1276_d_n18: f64 = self.ddt_jacobian(s.dn[170][18]);
        let eq95_e1276_d_n19: f64 = self.ddt_jacobian(s.dn[170][19]);
        let eq95_e1276_d_n20: f64 = self.ddt_jacobian(s.dn[170][20]);
        let eq95_e1276_d_n21: f64 = self.ddt_jacobian(s.dn[170][21]);
        let eq95_e1276_d_n22: f64 = self.ddt_jacobian(s.dn[170][22]);
        let eq95_e1276_d_n23: f64 = self.ddt_jacobian(s.dn[170][23]);
        let eq95_e1276_d_n24: f64 = self.ddt_jacobian(s.dn[170][24]);
        let eq95_e1276_d_n25: f64 = self.ddt_jacobian(s.dn[170][25]);
        let eq95_e1276_d_n26: f64 = self.ddt_jacobian(s.dn[170][26]);
        let eq95_e1276_d_n27: f64 = self.ddt_jacobian(s.dn[170][27]);
        let eq95_e1276_d_n28: f64 = self.ddt_jacobian(s.dn[170][28]);
        let eq95_e1276_d_n29: f64 = self.ddt_jacobian(s.dn[170][29]);
        let eq95_e1279: f64 = (p.p355 * (nv3 - nv10));
        let eq95_e1279_d_n3: f64 = p.p355;
        let eq95_e1279_d_n10: f64 = (-p.p355);
        let eq95_e1280: f64 = self.eval_ddt(83, eq95_e1279);
        let eq95_e1280_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n3: f64 = self.ddt_jacobian(eq95_e1279_d_n3);
        let eq95_e1280_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n10: f64 = self.ddt_jacobian(eq95_e1279_d_n10);
        let eq95_e1280_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq95_e1280_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq95_e1281: f64 = (eq95_e1276 + eq95_e1280);
        let eq95_e1281_d_n0: f64 = (eq95_e1276_d_n0 + eq95_e1280_d_n0);
        let eq95_e1281_d_n1: f64 = (eq95_e1276_d_n1 + eq95_e1280_d_n1);
        let eq95_e1281_d_n2: f64 = (eq95_e1276_d_n2 + eq95_e1280_d_n2);
        let eq95_e1281_d_n3: f64 = (eq95_e1276_d_n3 + eq95_e1280_d_n3);
        let eq95_e1281_d_n4: f64 = (eq95_e1276_d_n4 + eq95_e1280_d_n4);
        let eq95_e1281_d_n5: f64 = (eq95_e1276_d_n5 + eq95_e1280_d_n5);
        let eq95_e1281_d_n6: f64 = (eq95_e1276_d_n6 + eq95_e1280_d_n6);
        let eq95_e1281_d_n7: f64 = (eq95_e1276_d_n7 + eq95_e1280_d_n7);
        let eq95_e1281_d_n8: f64 = (eq95_e1276_d_n8 + eq95_e1280_d_n8);
        let eq95_e1281_d_n9: f64 = (eq95_e1276_d_n9 + eq95_e1280_d_n9);
        let eq95_e1281_d_n10: f64 = (eq95_e1276_d_n10 + eq95_e1280_d_n10);
        let eq95_e1281_d_n11: f64 = (eq95_e1276_d_n11 + eq95_e1280_d_n11);
        let eq95_e1281_d_n12: f64 = (eq95_e1276_d_n12 + eq95_e1280_d_n12);
        let eq95_e1281_d_n13: f64 = (eq95_e1276_d_n13 + eq95_e1280_d_n13);
        let eq95_e1281_d_n14: f64 = (eq95_e1276_d_n14 + eq95_e1280_d_n14);
        let eq95_e1281_d_n15: f64 = (eq95_e1276_d_n15 + eq95_e1280_d_n15);
        let eq95_e1281_d_n16: f64 = (eq95_e1276_d_n16 + eq95_e1280_d_n16);
        let eq95_e1281_d_n17: f64 = (eq95_e1276_d_n17 + eq95_e1280_d_n17);
        let eq95_e1281_d_n18: f64 = (eq95_e1276_d_n18 + eq95_e1280_d_n18);
        let eq95_e1281_d_n19: f64 = (eq95_e1276_d_n19 + eq95_e1280_d_n19);
        let eq95_e1281_d_n20: f64 = (eq95_e1276_d_n20 + eq95_e1280_d_n20);
        let eq95_e1281_d_n21: f64 = (eq95_e1276_d_n21 + eq95_e1280_d_n21);
        let eq95_e1281_d_n22: f64 = (eq95_e1276_d_n22 + eq95_e1280_d_n22);
        let eq95_e1281_d_n23: f64 = (eq95_e1276_d_n23 + eq95_e1280_d_n23);
        let eq95_e1281_d_n24: f64 = (eq95_e1276_d_n24 + eq95_e1280_d_n24);
        let eq95_e1281_d_n25: f64 = (eq95_e1276_d_n25 + eq95_e1280_d_n25);
        let eq95_e1281_d_n26: f64 = (eq95_e1276_d_n26 + eq95_e1280_d_n26);
        let eq95_e1281_d_n27: f64 = (eq95_e1276_d_n27 + eq95_e1280_d_n27);
        let eq95_e1281_d_n28: f64 = (eq95_e1276_d_n28 + eq95_e1280_d_n28);
        let eq95_e1281_d_n29: f64 = (eq95_e1276_d_n29 + eq95_e1280_d_n29);
        let eq95_value: f64 = eq95_e1281;
        let eq95_node_derivatives: [f64; 30] = [eq95_e1281_d_n0, eq95_e1281_d_n1, eq95_e1281_d_n2, eq95_e1281_d_n3, eq95_e1281_d_n4, eq95_e1281_d_n5, eq95_e1281_d_n6, eq95_e1281_d_n7, eq95_e1281_d_n8, eq95_e1281_d_n9, eq95_e1281_d_n10, eq95_e1281_d_n11, eq95_e1281_d_n12, eq95_e1281_d_n13, eq95_e1281_d_n14, eq95_e1281_d_n15, eq95_e1281_d_n16, eq95_e1281_d_n17, eq95_e1281_d_n18, eq95_e1281_d_n19, eq95_e1281_d_n20, eq95_e1281_d_n21, eq95_e1281_d_n22, eq95_e1281_d_n23, eq95_e1281_d_n24, eq95_e1281_d_n25, eq95_e1281_d_n26, eq95_e1281_d_n27, eq95_e1281_d_n28, eq95_e1281_d_n29];
        let eq95_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[10]),
            self.multiplicity * (eq95_value),
            &nodes,
            &eq95_node_derivatives,
            &branches,
            &eq95_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_96_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq96_e1289, eq96_e1289_d_n0, eq96_e1289_d_n1, eq96_e1289_d_n2, eq96_e1289_d_n3, eq96_e1289_d_n4, eq96_e1289_d_n5, eq96_e1289_d_n6, eq96_e1289_d_n7, eq96_e1289_d_n8, eq96_e1289_d_n9, eq96_e1289_d_n10, eq96_e1289_d_n11, eq96_e1289_d_n12, eq96_e1289_d_n13, eq96_e1289_d_n14, eq96_e1289_d_n15, eq96_e1289_d_n16, eq96_e1289_d_n17, eq96_e1289_d_n18, eq96_e1289_d_n19, eq96_e1289_d_n20, eq96_e1289_d_n21, eq96_e1289_d_n22, eq96_e1289_d_n23, eq96_e1289_d_n24, eq96_e1289_d_n25, eq96_e1289_d_n26, eq96_e1289_d_n27, eq96_e1289_d_n28, eq96_e1289_d_n29,) = {
    if (s.v[1055] != 0.0) {
        let eq96_e1286: f64 = (s.v[0] * (nv10 - nv11));
        let eq96_e1286_d_n10: f64 = s.v[0];
        let eq96_e1286_d_n11: f64 = (-s.v[0]);
        let eq96_e1287: f64 = (s.v[172] + eq96_e1286);
        let eq96_e1287_d_n10: f64 = (s.dn[172][10] + eq96_e1286_d_n10);
        let eq96_e1287_d_n11: f64 = (s.dn[172][11] + eq96_e1286_d_n11);
        (eq96_e1287, s.dn[172][0], s.dn[172][1], s.dn[172][2], s.dn[172][3], s.dn[172][4], s.dn[172][5], s.dn[172][6], s.dn[172][7], s.dn[172][8], s.dn[172][9], eq96_e1287_d_n10, eq96_e1287_d_n11, s.dn[172][12], s.dn[172][13], s.dn[172][14], s.dn[172][15], s.dn[172][16], s.dn[172][17], s.dn[172][18], s.dn[172][19], s.dn[172][20], s.dn[172][21], s.dn[172][22], s.dn[172][23], s.dn[172][24], s.dn[172][25], s.dn[172][26], s.dn[172][27], s.dn[172][28], s.dn[172][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq96_value: f64 = eq96_e1289;
        let eq96_node_derivatives: [f64; 30] = [eq96_e1289_d_n0, eq96_e1289_d_n1, eq96_e1289_d_n2, eq96_e1289_d_n3, eq96_e1289_d_n4, eq96_e1289_d_n5, eq96_e1289_d_n6, eq96_e1289_d_n7, eq96_e1289_d_n8, eq96_e1289_d_n9, eq96_e1289_d_n10, eq96_e1289_d_n11, eq96_e1289_d_n12, eq96_e1289_d_n13, eq96_e1289_d_n14, eq96_e1289_d_n15, eq96_e1289_d_n16, eq96_e1289_d_n17, eq96_e1289_d_n18, eq96_e1289_d_n19, eq96_e1289_d_n20, eq96_e1289_d_n21, eq96_e1289_d_n22, eq96_e1289_d_n23, eq96_e1289_d_n24, eq96_e1289_d_n25, eq96_e1289_d_n26, eq96_e1289_d_n27, eq96_e1289_d_n28, eq96_e1289_d_n29];
        let eq96_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[11]),
            self.multiplicity * (eq96_value),
            &nodes,
            &eq96_node_derivatives,
            &branches,
            &eq96_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_97_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq97_e1294,) = {
    if (!(s.v[1055] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq97_value: f64 = eq97_e1294;
        stamper.stamp_potential(
            branches[23],
            eq97_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_98_block_0(
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
        let (eq98_e1304, eq98_e1304_d_n0, eq98_e1304_d_n1, eq98_e1304_d_n2, eq98_e1304_d_n3, eq98_e1304_d_n4, eq98_e1304_d_n5, eq98_e1304_d_n6, eq98_e1304_d_n7, eq98_e1304_d_n8, eq98_e1304_d_n9, eq98_e1304_d_n10, eq98_e1304_d_n11, eq98_e1304_d_n12, eq98_e1304_d_n13, eq98_e1304_d_n14, eq98_e1304_d_n15, eq98_e1304_d_n16, eq98_e1304_d_n17, eq98_e1304_d_n18, eq98_e1304_d_n19, eq98_e1304_d_n20, eq98_e1304_d_n21, eq98_e1304_d_n22, eq98_e1304_d_n23, eq98_e1304_d_n24, eq98_e1304_d_n25, eq98_e1304_d_n26, eq98_e1304_d_n27, eq98_e1304_d_n28, eq98_e1304_d_n29,) = {
    if (s.v[1201] != 0.0) {
        let eq98_e1297: f64 = self.eval_ddt(84, s.v[173]);
        let eq98_e1297_d_n0: f64 = self.ddt_jacobian(s.dn[173][0]);
        let eq98_e1297_d_n1: f64 = self.ddt_jacobian(s.dn[173][1]);
        let eq98_e1297_d_n2: f64 = self.ddt_jacobian(s.dn[173][2]);
        let eq98_e1297_d_n3: f64 = self.ddt_jacobian(s.dn[173][3]);
        let eq98_e1297_d_n4: f64 = self.ddt_jacobian(s.dn[173][4]);
        let eq98_e1297_d_n5: f64 = self.ddt_jacobian(s.dn[173][5]);
        let eq98_e1297_d_n6: f64 = self.ddt_jacobian(s.dn[173][6]);
        let eq98_e1297_d_n7: f64 = self.ddt_jacobian(s.dn[173][7]);
        let eq98_e1297_d_n8: f64 = self.ddt_jacobian(s.dn[173][8]);
        let eq98_e1297_d_n9: f64 = self.ddt_jacobian(s.dn[173][9]);
        let eq98_e1297_d_n10: f64 = self.ddt_jacobian(s.dn[173][10]);
        let eq98_e1297_d_n11: f64 = self.ddt_jacobian(s.dn[173][11]);
        let eq98_e1297_d_n12: f64 = self.ddt_jacobian(s.dn[173][12]);
        let eq98_e1297_d_n13: f64 = self.ddt_jacobian(s.dn[173][13]);
        let eq98_e1297_d_n14: f64 = self.ddt_jacobian(s.dn[173][14]);
        let eq98_e1297_d_n15: f64 = self.ddt_jacobian(s.dn[173][15]);
        let eq98_e1297_d_n16: f64 = self.ddt_jacobian(s.dn[173][16]);
        let eq98_e1297_d_n17: f64 = self.ddt_jacobian(s.dn[173][17]);
        let eq98_e1297_d_n18: f64 = self.ddt_jacobian(s.dn[173][18]);
        let eq98_e1297_d_n19: f64 = self.ddt_jacobian(s.dn[173][19]);
        let eq98_e1297_d_n20: f64 = self.ddt_jacobian(s.dn[173][20]);
        let eq98_e1297_d_n21: f64 = self.ddt_jacobian(s.dn[173][21]);
        let eq98_e1297_d_n22: f64 = self.ddt_jacobian(s.dn[173][22]);
        let eq98_e1297_d_n23: f64 = self.ddt_jacobian(s.dn[173][23]);
        let eq98_e1297_d_n24: f64 = self.ddt_jacobian(s.dn[173][24]);
        let eq98_e1297_d_n25: f64 = self.ddt_jacobian(s.dn[173][25]);
        let eq98_e1297_d_n26: f64 = self.ddt_jacobian(s.dn[173][26]);
        let eq98_e1297_d_n27: f64 = self.ddt_jacobian(s.dn[173][27]);
        let eq98_e1297_d_n28: f64 = self.ddt_jacobian(s.dn[173][28]);
        let eq98_e1297_d_n29: f64 = self.ddt_jacobian(s.dn[173][29]);
        let eq98_e1300: f64 = (p.p355 * (nv7 - nv11));
        let eq98_e1300_d_n7: f64 = p.p355;
        let eq98_e1300_d_n11: f64 = (-p.p355);
        let eq98_e1301: f64 = self.eval_ddt(85, eq98_e1300);
        let eq98_e1301_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n7: f64 = self.ddt_jacobian(eq98_e1300_d_n7);
        let eq98_e1301_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n11: f64 = self.ddt_jacobian(eq98_e1300_d_n11);
        let eq98_e1301_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq98_e1301_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq98_e1302: f64 = (eq98_e1297 + eq98_e1301);
        let eq98_e1302_d_n0: f64 = (eq98_e1297_d_n0 + eq98_e1301_d_n0);
        let eq98_e1302_d_n1: f64 = (eq98_e1297_d_n1 + eq98_e1301_d_n1);
        let eq98_e1302_d_n2: f64 = (eq98_e1297_d_n2 + eq98_e1301_d_n2);
        let eq98_e1302_d_n3: f64 = (eq98_e1297_d_n3 + eq98_e1301_d_n3);
        let eq98_e1302_d_n4: f64 = (eq98_e1297_d_n4 + eq98_e1301_d_n4);
        let eq98_e1302_d_n5: f64 = (eq98_e1297_d_n5 + eq98_e1301_d_n5);
        let eq98_e1302_d_n6: f64 = (eq98_e1297_d_n6 + eq98_e1301_d_n6);
        let eq98_e1302_d_n7: f64 = (eq98_e1297_d_n7 + eq98_e1301_d_n7);
        let eq98_e1302_d_n8: f64 = (eq98_e1297_d_n8 + eq98_e1301_d_n8);
        let eq98_e1302_d_n9: f64 = (eq98_e1297_d_n9 + eq98_e1301_d_n9);
        let eq98_e1302_d_n10: f64 = (eq98_e1297_d_n10 + eq98_e1301_d_n10);
        let eq98_e1302_d_n11: f64 = (eq98_e1297_d_n11 + eq98_e1301_d_n11);
        let eq98_e1302_d_n12: f64 = (eq98_e1297_d_n12 + eq98_e1301_d_n12);
        let eq98_e1302_d_n13: f64 = (eq98_e1297_d_n13 + eq98_e1301_d_n13);
        let eq98_e1302_d_n14: f64 = (eq98_e1297_d_n14 + eq98_e1301_d_n14);
        let eq98_e1302_d_n15: f64 = (eq98_e1297_d_n15 + eq98_e1301_d_n15);
        let eq98_e1302_d_n16: f64 = (eq98_e1297_d_n16 + eq98_e1301_d_n16);
        let eq98_e1302_d_n17: f64 = (eq98_e1297_d_n17 + eq98_e1301_d_n17);
        let eq98_e1302_d_n18: f64 = (eq98_e1297_d_n18 + eq98_e1301_d_n18);
        let eq98_e1302_d_n19: f64 = (eq98_e1297_d_n19 + eq98_e1301_d_n19);
        let eq98_e1302_d_n20: f64 = (eq98_e1297_d_n20 + eq98_e1301_d_n20);
        let eq98_e1302_d_n21: f64 = (eq98_e1297_d_n21 + eq98_e1301_d_n21);
        let eq98_e1302_d_n22: f64 = (eq98_e1297_d_n22 + eq98_e1301_d_n22);
        let eq98_e1302_d_n23: f64 = (eq98_e1297_d_n23 + eq98_e1301_d_n23);
        let eq98_e1302_d_n24: f64 = (eq98_e1297_d_n24 + eq98_e1301_d_n24);
        let eq98_e1302_d_n25: f64 = (eq98_e1297_d_n25 + eq98_e1301_d_n25);
        let eq98_e1302_d_n26: f64 = (eq98_e1297_d_n26 + eq98_e1301_d_n26);
        let eq98_e1302_d_n27: f64 = (eq98_e1297_d_n27 + eq98_e1301_d_n27);
        let eq98_e1302_d_n28: f64 = (eq98_e1297_d_n28 + eq98_e1301_d_n28);
        let eq98_e1302_d_n29: f64 = (eq98_e1297_d_n29 + eq98_e1301_d_n29);
        (eq98_e1302, eq98_e1302_d_n0, eq98_e1302_d_n1, eq98_e1302_d_n2, eq98_e1302_d_n3, eq98_e1302_d_n4, eq98_e1302_d_n5, eq98_e1302_d_n6, eq98_e1302_d_n7, eq98_e1302_d_n8, eq98_e1302_d_n9, eq98_e1302_d_n10, eq98_e1302_d_n11, eq98_e1302_d_n12, eq98_e1302_d_n13, eq98_e1302_d_n14, eq98_e1302_d_n15, eq98_e1302_d_n16, eq98_e1302_d_n17, eq98_e1302_d_n18, eq98_e1302_d_n19, eq98_e1302_d_n20, eq98_e1302_d_n21, eq98_e1302_d_n22, eq98_e1302_d_n23, eq98_e1302_d_n24, eq98_e1302_d_n25, eq98_e1302_d_n26, eq98_e1302_d_n27, eq98_e1302_d_n28, eq98_e1302_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq98_value: f64 = eq98_e1304;
        let eq98_node_derivatives: [f64; 30] = [eq98_e1304_d_n0, eq98_e1304_d_n1, eq98_e1304_d_n2, eq98_e1304_d_n3, eq98_e1304_d_n4, eq98_e1304_d_n5, eq98_e1304_d_n6, eq98_e1304_d_n7, eq98_e1304_d_n8, eq98_e1304_d_n9, eq98_e1304_d_n10, eq98_e1304_d_n11, eq98_e1304_d_n12, eq98_e1304_d_n13, eq98_e1304_d_n14, eq98_e1304_d_n15, eq98_e1304_d_n16, eq98_e1304_d_n17, eq98_e1304_d_n18, eq98_e1304_d_n19, eq98_e1304_d_n20, eq98_e1304_d_n21, eq98_e1304_d_n22, eq98_e1304_d_n23, eq98_e1304_d_n24, eq98_e1304_d_n25, eq98_e1304_d_n26, eq98_e1304_d_n27, eq98_e1304_d_n28, eq98_e1304_d_n29];
        let eq98_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            self.multiplicity * (eq98_value),
            &nodes,
            &eq98_node_derivatives,
            &branches,
            &eq98_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_99_block_0(
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq99_e1314, eq99_e1314_d_n0, eq99_e1314_d_n1, eq99_e1314_d_n2, eq99_e1314_d_n3, eq99_e1314_d_n4, eq99_e1314_d_n5, eq99_e1314_d_n6, eq99_e1314_d_n7, eq99_e1314_d_n8, eq99_e1314_d_n9, eq99_e1314_d_n10, eq99_e1314_d_n11, eq99_e1314_d_n12, eq99_e1314_d_n13, eq99_e1314_d_n14, eq99_e1314_d_n15, eq99_e1314_d_n16, eq99_e1314_d_n17, eq99_e1314_d_n18, eq99_e1314_d_n19, eq99_e1314_d_n20, eq99_e1314_d_n21, eq99_e1314_d_n22, eq99_e1314_d_n23, eq99_e1314_d_n24, eq99_e1314_d_n25, eq99_e1314_d_n26, eq99_e1314_d_n27, eq99_e1314_d_n28, eq99_e1314_d_n29,) = {
    if (s.v[1201] != 0.0) {
        let eq99_e1307: f64 = self.eval_ddt(86, s.v[174]);
        let eq99_e1307_d_n0: f64 = self.ddt_jacobian(s.dn[174][0]);
        let eq99_e1307_d_n1: f64 = self.ddt_jacobian(s.dn[174][1]);
        let eq99_e1307_d_n2: f64 = self.ddt_jacobian(s.dn[174][2]);
        let eq99_e1307_d_n3: f64 = self.ddt_jacobian(s.dn[174][3]);
        let eq99_e1307_d_n4: f64 = self.ddt_jacobian(s.dn[174][4]);
        let eq99_e1307_d_n5: f64 = self.ddt_jacobian(s.dn[174][5]);
        let eq99_e1307_d_n6: f64 = self.ddt_jacobian(s.dn[174][6]);
        let eq99_e1307_d_n7: f64 = self.ddt_jacobian(s.dn[174][7]);
        let eq99_e1307_d_n8: f64 = self.ddt_jacobian(s.dn[174][8]);
        let eq99_e1307_d_n9: f64 = self.ddt_jacobian(s.dn[174][9]);
        let eq99_e1307_d_n10: f64 = self.ddt_jacobian(s.dn[174][10]);
        let eq99_e1307_d_n11: f64 = self.ddt_jacobian(s.dn[174][11]);
        let eq99_e1307_d_n12: f64 = self.ddt_jacobian(s.dn[174][12]);
        let eq99_e1307_d_n13: f64 = self.ddt_jacobian(s.dn[174][13]);
        let eq99_e1307_d_n14: f64 = self.ddt_jacobian(s.dn[174][14]);
        let eq99_e1307_d_n15: f64 = self.ddt_jacobian(s.dn[174][15]);
        let eq99_e1307_d_n16: f64 = self.ddt_jacobian(s.dn[174][16]);
        let eq99_e1307_d_n17: f64 = self.ddt_jacobian(s.dn[174][17]);
        let eq99_e1307_d_n18: f64 = self.ddt_jacobian(s.dn[174][18]);
        let eq99_e1307_d_n19: f64 = self.ddt_jacobian(s.dn[174][19]);
        let eq99_e1307_d_n20: f64 = self.ddt_jacobian(s.dn[174][20]);
        let eq99_e1307_d_n21: f64 = self.ddt_jacobian(s.dn[174][21]);
        let eq99_e1307_d_n22: f64 = self.ddt_jacobian(s.dn[174][22]);
        let eq99_e1307_d_n23: f64 = self.ddt_jacobian(s.dn[174][23]);
        let eq99_e1307_d_n24: f64 = self.ddt_jacobian(s.dn[174][24]);
        let eq99_e1307_d_n25: f64 = self.ddt_jacobian(s.dn[174][25]);
        let eq99_e1307_d_n26: f64 = self.ddt_jacobian(s.dn[174][26]);
        let eq99_e1307_d_n27: f64 = self.ddt_jacobian(s.dn[174][27]);
        let eq99_e1307_d_n28: f64 = self.ddt_jacobian(s.dn[174][28]);
        let eq99_e1307_d_n29: f64 = self.ddt_jacobian(s.dn[174][29]);
        let eq99_e1310: f64 = (p.p355 * (nv7 - nv10));
        let eq99_e1310_d_n7: f64 = p.p355;
        let eq99_e1310_d_n10: f64 = (-p.p355);
        let eq99_e1311: f64 = self.eval_ddt(87, eq99_e1310);
        let eq99_e1311_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n7: f64 = self.ddt_jacobian(eq99_e1310_d_n7);
        let eq99_e1311_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n10: f64 = self.ddt_jacobian(eq99_e1310_d_n10);
        let eq99_e1311_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq99_e1311_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq99_e1312: f64 = (eq99_e1307 + eq99_e1311);
        let eq99_e1312_d_n0: f64 = (eq99_e1307_d_n0 + eq99_e1311_d_n0);
        let eq99_e1312_d_n1: f64 = (eq99_e1307_d_n1 + eq99_e1311_d_n1);
        let eq99_e1312_d_n2: f64 = (eq99_e1307_d_n2 + eq99_e1311_d_n2);
        let eq99_e1312_d_n3: f64 = (eq99_e1307_d_n3 + eq99_e1311_d_n3);
        let eq99_e1312_d_n4: f64 = (eq99_e1307_d_n4 + eq99_e1311_d_n4);
        let eq99_e1312_d_n5: f64 = (eq99_e1307_d_n5 + eq99_e1311_d_n5);
        let eq99_e1312_d_n6: f64 = (eq99_e1307_d_n6 + eq99_e1311_d_n6);
        let eq99_e1312_d_n7: f64 = (eq99_e1307_d_n7 + eq99_e1311_d_n7);
        let eq99_e1312_d_n8: f64 = (eq99_e1307_d_n8 + eq99_e1311_d_n8);
        let eq99_e1312_d_n9: f64 = (eq99_e1307_d_n9 + eq99_e1311_d_n9);
        let eq99_e1312_d_n10: f64 = (eq99_e1307_d_n10 + eq99_e1311_d_n10);
        let eq99_e1312_d_n11: f64 = (eq99_e1307_d_n11 + eq99_e1311_d_n11);
        let eq99_e1312_d_n12: f64 = (eq99_e1307_d_n12 + eq99_e1311_d_n12);
        let eq99_e1312_d_n13: f64 = (eq99_e1307_d_n13 + eq99_e1311_d_n13);
        let eq99_e1312_d_n14: f64 = (eq99_e1307_d_n14 + eq99_e1311_d_n14);
        let eq99_e1312_d_n15: f64 = (eq99_e1307_d_n15 + eq99_e1311_d_n15);
        let eq99_e1312_d_n16: f64 = (eq99_e1307_d_n16 + eq99_e1311_d_n16);
        let eq99_e1312_d_n17: f64 = (eq99_e1307_d_n17 + eq99_e1311_d_n17);
        let eq99_e1312_d_n18: f64 = (eq99_e1307_d_n18 + eq99_e1311_d_n18);
        let eq99_e1312_d_n19: f64 = (eq99_e1307_d_n19 + eq99_e1311_d_n19);
        let eq99_e1312_d_n20: f64 = (eq99_e1307_d_n20 + eq99_e1311_d_n20);
        let eq99_e1312_d_n21: f64 = (eq99_e1307_d_n21 + eq99_e1311_d_n21);
        let eq99_e1312_d_n22: f64 = (eq99_e1307_d_n22 + eq99_e1311_d_n22);
        let eq99_e1312_d_n23: f64 = (eq99_e1307_d_n23 + eq99_e1311_d_n23);
        let eq99_e1312_d_n24: f64 = (eq99_e1307_d_n24 + eq99_e1311_d_n24);
        let eq99_e1312_d_n25: f64 = (eq99_e1307_d_n25 + eq99_e1311_d_n25);
        let eq99_e1312_d_n26: f64 = (eq99_e1307_d_n26 + eq99_e1311_d_n26);
        let eq99_e1312_d_n27: f64 = (eq99_e1307_d_n27 + eq99_e1311_d_n27);
        let eq99_e1312_d_n28: f64 = (eq99_e1307_d_n28 + eq99_e1311_d_n28);
        let eq99_e1312_d_n29: f64 = (eq99_e1307_d_n29 + eq99_e1311_d_n29);
        (eq99_e1312, eq99_e1312_d_n0, eq99_e1312_d_n1, eq99_e1312_d_n2, eq99_e1312_d_n3, eq99_e1312_d_n4, eq99_e1312_d_n5, eq99_e1312_d_n6, eq99_e1312_d_n7, eq99_e1312_d_n8, eq99_e1312_d_n9, eq99_e1312_d_n10, eq99_e1312_d_n11, eq99_e1312_d_n12, eq99_e1312_d_n13, eq99_e1312_d_n14, eq99_e1312_d_n15, eq99_e1312_d_n16, eq99_e1312_d_n17, eq99_e1312_d_n18, eq99_e1312_d_n19, eq99_e1312_d_n20, eq99_e1312_d_n21, eq99_e1312_d_n22, eq99_e1312_d_n23, eq99_e1312_d_n24, eq99_e1312_d_n25, eq99_e1312_d_n26, eq99_e1312_d_n27, eq99_e1312_d_n28, eq99_e1312_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq99_value: f64 = eq99_e1314;
        let eq99_node_derivatives: [f64; 30] = [eq99_e1314_d_n0, eq99_e1314_d_n1, eq99_e1314_d_n2, eq99_e1314_d_n3, eq99_e1314_d_n4, eq99_e1314_d_n5, eq99_e1314_d_n6, eq99_e1314_d_n7, eq99_e1314_d_n8, eq99_e1314_d_n9, eq99_e1314_d_n10, eq99_e1314_d_n11, eq99_e1314_d_n12, eq99_e1314_d_n13, eq99_e1314_d_n14, eq99_e1314_d_n15, eq99_e1314_d_n16, eq99_e1314_d_n17, eq99_e1314_d_n18, eq99_e1314_d_n19, eq99_e1314_d_n20, eq99_e1314_d_n21, eq99_e1314_d_n22, eq99_e1314_d_n23, eq99_e1314_d_n24, eq99_e1314_d_n25, eq99_e1314_d_n26, eq99_e1314_d_n27, eq99_e1314_d_n28, eq99_e1314_d_n29];
        let eq99_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            self.multiplicity * (eq99_value),
            &nodes,
            &eq99_node_derivatives,
            &branches,
            &eq99_branch_derivatives,
            self.multiplicity,
        );
    }
}
