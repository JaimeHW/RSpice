#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_68_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq68_e1060,) = {
    if (!(s.v[760] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq68_value: f64 = eq68_e1060;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[9]),
            self.multiplicity * (eq68_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_69_block_0(
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
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq69_e1062: f64 = self.eval_ddt(50, s.v[200]);
        let eq69_e1062_d_n0: f64 = self.ddt_jacobian(s.dn[200][0]);
        let eq69_e1062_d_n1: f64 = self.ddt_jacobian(s.dn[200][1]);
        let eq69_e1062_d_n2: f64 = self.ddt_jacobian(s.dn[200][2]);
        let eq69_e1062_d_n3: f64 = self.ddt_jacobian(s.dn[200][3]);
        let eq69_e1062_d_n4: f64 = self.ddt_jacobian(s.dn[200][4]);
        let eq69_e1062_d_n5: f64 = self.ddt_jacobian(s.dn[200][5]);
        let eq69_e1062_d_n6: f64 = self.ddt_jacobian(s.dn[200][6]);
        let eq69_e1062_d_n7: f64 = self.ddt_jacobian(s.dn[200][7]);
        let eq69_e1062_d_n8: f64 = self.ddt_jacobian(s.dn[200][8]);
        let eq69_e1062_d_n9: f64 = self.ddt_jacobian(s.dn[200][9]);
        let eq69_e1062_d_n10: f64 = self.ddt_jacobian(s.dn[200][10]);
        let eq69_e1062_d_n11: f64 = self.ddt_jacobian(s.dn[200][11]);
        let eq69_e1062_d_n12: f64 = self.ddt_jacobian(s.dn[200][12]);
        let eq69_e1062_d_n13: f64 = self.ddt_jacobian(s.dn[200][13]);
        let eq69_e1062_d_n14: f64 = self.ddt_jacobian(s.dn[200][14]);
        let eq69_e1062_d_n15: f64 = self.ddt_jacobian(s.dn[200][15]);
        let eq69_e1062_d_n16: f64 = self.ddt_jacobian(s.dn[200][16]);
        let eq69_e1062_d_n17: f64 = self.ddt_jacobian(s.dn[200][17]);
        let eq69_e1062_d_n18: f64 = self.ddt_jacobian(s.dn[200][18]);
        let eq69_e1062_d_n19: f64 = self.ddt_jacobian(s.dn[200][19]);
        let eq69_e1062_d_n20: f64 = self.ddt_jacobian(s.dn[200][20]);
        let eq69_e1062_d_n21: f64 = self.ddt_jacobian(s.dn[200][21]);
        let eq69_e1062_d_n22: f64 = self.ddt_jacobian(s.dn[200][22]);
        let eq69_e1062_d_n23: f64 = self.ddt_jacobian(s.dn[200][23]);
        let eq69_e1062_d_n24: f64 = self.ddt_jacobian(s.dn[200][24]);
        let eq69_e1062_d_n25: f64 = self.ddt_jacobian(s.dn[200][25]);
        let eq69_e1062_d_n26: f64 = self.ddt_jacobian(s.dn[200][26]);
        let eq69_e1062_d_n27: f64 = self.ddt_jacobian(s.dn[200][27]);
        let eq69_e1062_d_n28: f64 = self.ddt_jacobian(s.dn[200][28]);
        let eq69_e1062_d_n29: f64 = self.ddt_jacobian(s.dn[200][29]);
        let eq69_e1065: f64 = (p.p355 * (nv3 - nv14));
        let eq69_e1065_d_n3: f64 = p.p355;
        let eq69_e1065_d_n14: f64 = (-p.p355);
        let eq69_e1066: f64 = self.eval_ddt(51, eq69_e1065);
        let eq69_e1066_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n3: f64 = self.ddt_jacobian(eq69_e1065_d_n3);
        let eq69_e1066_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n14: f64 = self.ddt_jacobian(eq69_e1065_d_n14);
        let eq69_e1066_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq69_e1066_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq69_e1067: f64 = (eq69_e1062 + eq69_e1066);
        let eq69_e1067_d_n0: f64 = (eq69_e1062_d_n0 + eq69_e1066_d_n0);
        let eq69_e1067_d_n1: f64 = (eq69_e1062_d_n1 + eq69_e1066_d_n1);
        let eq69_e1067_d_n2: f64 = (eq69_e1062_d_n2 + eq69_e1066_d_n2);
        let eq69_e1067_d_n3: f64 = (eq69_e1062_d_n3 + eq69_e1066_d_n3);
        let eq69_e1067_d_n4: f64 = (eq69_e1062_d_n4 + eq69_e1066_d_n4);
        let eq69_e1067_d_n5: f64 = (eq69_e1062_d_n5 + eq69_e1066_d_n5);
        let eq69_e1067_d_n6: f64 = (eq69_e1062_d_n6 + eq69_e1066_d_n6);
        let eq69_e1067_d_n7: f64 = (eq69_e1062_d_n7 + eq69_e1066_d_n7);
        let eq69_e1067_d_n8: f64 = (eq69_e1062_d_n8 + eq69_e1066_d_n8);
        let eq69_e1067_d_n9: f64 = (eq69_e1062_d_n9 + eq69_e1066_d_n9);
        let eq69_e1067_d_n10: f64 = (eq69_e1062_d_n10 + eq69_e1066_d_n10);
        let eq69_e1067_d_n11: f64 = (eq69_e1062_d_n11 + eq69_e1066_d_n11);
        let eq69_e1067_d_n12: f64 = (eq69_e1062_d_n12 + eq69_e1066_d_n12);
        let eq69_e1067_d_n13: f64 = (eq69_e1062_d_n13 + eq69_e1066_d_n13);
        let eq69_e1067_d_n14: f64 = (eq69_e1062_d_n14 + eq69_e1066_d_n14);
        let eq69_e1067_d_n15: f64 = (eq69_e1062_d_n15 + eq69_e1066_d_n15);
        let eq69_e1067_d_n16: f64 = (eq69_e1062_d_n16 + eq69_e1066_d_n16);
        let eq69_e1067_d_n17: f64 = (eq69_e1062_d_n17 + eq69_e1066_d_n17);
        let eq69_e1067_d_n18: f64 = (eq69_e1062_d_n18 + eq69_e1066_d_n18);
        let eq69_e1067_d_n19: f64 = (eq69_e1062_d_n19 + eq69_e1066_d_n19);
        let eq69_e1067_d_n20: f64 = (eq69_e1062_d_n20 + eq69_e1066_d_n20);
        let eq69_e1067_d_n21: f64 = (eq69_e1062_d_n21 + eq69_e1066_d_n21);
        let eq69_e1067_d_n22: f64 = (eq69_e1062_d_n22 + eq69_e1066_d_n22);
        let eq69_e1067_d_n23: f64 = (eq69_e1062_d_n23 + eq69_e1066_d_n23);
        let eq69_e1067_d_n24: f64 = (eq69_e1062_d_n24 + eq69_e1066_d_n24);
        let eq69_e1067_d_n25: f64 = (eq69_e1062_d_n25 + eq69_e1066_d_n25);
        let eq69_e1067_d_n26: f64 = (eq69_e1062_d_n26 + eq69_e1066_d_n26);
        let eq69_e1067_d_n27: f64 = (eq69_e1062_d_n27 + eq69_e1066_d_n27);
        let eq69_e1067_d_n28: f64 = (eq69_e1062_d_n28 + eq69_e1066_d_n28);
        let eq69_e1067_d_n29: f64 = (eq69_e1062_d_n29 + eq69_e1066_d_n29);
        let eq69_value: f64 = eq69_e1067;
        let eq69_node_derivatives: [f64; 30] = [eq69_e1067_d_n0, eq69_e1067_d_n1, eq69_e1067_d_n2, eq69_e1067_d_n3, eq69_e1067_d_n4, eq69_e1067_d_n5, eq69_e1067_d_n6, eq69_e1067_d_n7, eq69_e1067_d_n8, eq69_e1067_d_n9, eq69_e1067_d_n10, eq69_e1067_d_n11, eq69_e1067_d_n12, eq69_e1067_d_n13, eq69_e1067_d_n14, eq69_e1067_d_n15, eq69_e1067_d_n16, eq69_e1067_d_n17, eq69_e1067_d_n18, eq69_e1067_d_n19, eq69_e1067_d_n20, eq69_e1067_d_n21, eq69_e1067_d_n22, eq69_e1067_d_n23, eq69_e1067_d_n24, eq69_e1067_d_n25, eq69_e1067_d_n26, eq69_e1067_d_n27, eq69_e1067_d_n28, eq69_e1067_d_n29];
        let eq69_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[14]),
            self.multiplicity * (eq69_value),
            &nodes,
            &eq69_node_derivatives,
            &branches,
            &eq69_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_70_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq70_e1075, eq70_e1075_d_n0, eq70_e1075_d_n1, eq70_e1075_d_n2, eq70_e1075_d_n3, eq70_e1075_d_n4, eq70_e1075_d_n5, eq70_e1075_d_n6, eq70_e1075_d_n7, eq70_e1075_d_n8, eq70_e1075_d_n9, eq70_e1075_d_n10, eq70_e1075_d_n11, eq70_e1075_d_n12, eq70_e1075_d_n13, eq70_e1075_d_n14, eq70_e1075_d_n15, eq70_e1075_d_n16, eq70_e1075_d_n17, eq70_e1075_d_n18, eq70_e1075_d_n19, eq70_e1075_d_n20, eq70_e1075_d_n21, eq70_e1075_d_n22, eq70_e1075_d_n23, eq70_e1075_d_n24, eq70_e1075_d_n25, eq70_e1075_d_n26, eq70_e1075_d_n27, eq70_e1075_d_n28, eq70_e1075_d_n29,) = {
    if (s.v[761] != 0.0) {
        let eq70_e1072: f64 = (s.v[0] * (nv14 - nv5));
        let eq70_e1072_d_n5: f64 = (-s.v[0]);
        let eq70_e1072_d_n14: f64 = s.v[0];
        let eq70_e1073: f64 = (s.v[190] + eq70_e1072);
        let eq70_e1073_d_n5: f64 = (s.dn[190][5] + eq70_e1072_d_n5);
        let eq70_e1073_d_n14: f64 = (s.dn[190][14] + eq70_e1072_d_n14);
        (eq70_e1073, s.dn[190][0], s.dn[190][1], s.dn[190][2], s.dn[190][3], s.dn[190][4], eq70_e1073_d_n5, s.dn[190][6], s.dn[190][7], s.dn[190][8], s.dn[190][9], s.dn[190][10], s.dn[190][11], s.dn[190][12], s.dn[190][13], eq70_e1073_d_n14, s.dn[190][15], s.dn[190][16], s.dn[190][17], s.dn[190][18], s.dn[190][19], s.dn[190][20], s.dn[190][21], s.dn[190][22], s.dn[190][23], s.dn[190][24], s.dn[190][25], s.dn[190][26], s.dn[190][27], s.dn[190][28], s.dn[190][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq70_value: f64 = eq70_e1075;
        let eq70_node_derivatives: [f64; 30] = [eq70_e1075_d_n0, eq70_e1075_d_n1, eq70_e1075_d_n2, eq70_e1075_d_n3, eq70_e1075_d_n4, eq70_e1075_d_n5, eq70_e1075_d_n6, eq70_e1075_d_n7, eq70_e1075_d_n8, eq70_e1075_d_n9, eq70_e1075_d_n10, eq70_e1075_d_n11, eq70_e1075_d_n12, eq70_e1075_d_n13, eq70_e1075_d_n14, eq70_e1075_d_n15, eq70_e1075_d_n16, eq70_e1075_d_n17, eq70_e1075_d_n18, eq70_e1075_d_n19, eq70_e1075_d_n20, eq70_e1075_d_n21, eq70_e1075_d_n22, eq70_e1075_d_n23, eq70_e1075_d_n24, eq70_e1075_d_n25, eq70_e1075_d_n26, eq70_e1075_d_n27, eq70_e1075_d_n28, eq70_e1075_d_n29];
        let eq70_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[14]),
            Some(nodes[5]),
            self.multiplicity * (eq70_value),
            &nodes,
            &eq70_node_derivatives,
            &branches,
            &eq70_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_71_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq71_e1080,) = {
    if (!(s.v[761] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq71_value: f64 = eq71_e1080;
        stamper.stamp_potential(
            branches[21],
            eq71_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_72_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq72_e1090, eq72_e1090_d_n0, eq72_e1090_d_n1, eq72_e1090_d_n2, eq72_e1090_d_n3, eq72_e1090_d_n4, eq72_e1090_d_n5, eq72_e1090_d_n6, eq72_e1090_d_n7, eq72_e1090_d_n8, eq72_e1090_d_n9, eq72_e1090_d_n10, eq72_e1090_d_n11, eq72_e1090_d_n12, eq72_e1090_d_n13, eq72_e1090_d_n14, eq72_e1090_d_n15, eq72_e1090_d_n16, eq72_e1090_d_n17, eq72_e1090_d_n18, eq72_e1090_d_n19, eq72_e1090_d_n20, eq72_e1090_d_n21, eq72_e1090_d_n22, eq72_e1090_d_n23, eq72_e1090_d_n24, eq72_e1090_d_n25, eq72_e1090_d_n26, eq72_e1090_d_n27, eq72_e1090_d_n28, eq72_e1090_d_n29,) = {
    if (s.v[907] != 0.0) {
        let eq72_e1083: f64 = self.eval_ddt(52, s.v[191]);
        let eq72_e1083_d_n0: f64 = self.ddt_jacobian(s.dn[191][0]);
        let eq72_e1083_d_n1: f64 = self.ddt_jacobian(s.dn[191][1]);
        let eq72_e1083_d_n2: f64 = self.ddt_jacobian(s.dn[191][2]);
        let eq72_e1083_d_n3: f64 = self.ddt_jacobian(s.dn[191][3]);
        let eq72_e1083_d_n4: f64 = self.ddt_jacobian(s.dn[191][4]);
        let eq72_e1083_d_n5: f64 = self.ddt_jacobian(s.dn[191][5]);
        let eq72_e1083_d_n6: f64 = self.ddt_jacobian(s.dn[191][6]);
        let eq72_e1083_d_n7: f64 = self.ddt_jacobian(s.dn[191][7]);
        let eq72_e1083_d_n8: f64 = self.ddt_jacobian(s.dn[191][8]);
        let eq72_e1083_d_n9: f64 = self.ddt_jacobian(s.dn[191][9]);
        let eq72_e1083_d_n10: f64 = self.ddt_jacobian(s.dn[191][10]);
        let eq72_e1083_d_n11: f64 = self.ddt_jacobian(s.dn[191][11]);
        let eq72_e1083_d_n12: f64 = self.ddt_jacobian(s.dn[191][12]);
        let eq72_e1083_d_n13: f64 = self.ddt_jacobian(s.dn[191][13]);
        let eq72_e1083_d_n14: f64 = self.ddt_jacobian(s.dn[191][14]);
        let eq72_e1083_d_n15: f64 = self.ddt_jacobian(s.dn[191][15]);
        let eq72_e1083_d_n16: f64 = self.ddt_jacobian(s.dn[191][16]);
        let eq72_e1083_d_n17: f64 = self.ddt_jacobian(s.dn[191][17]);
        let eq72_e1083_d_n18: f64 = self.ddt_jacobian(s.dn[191][18]);
        let eq72_e1083_d_n19: f64 = self.ddt_jacobian(s.dn[191][19]);
        let eq72_e1083_d_n20: f64 = self.ddt_jacobian(s.dn[191][20]);
        let eq72_e1083_d_n21: f64 = self.ddt_jacobian(s.dn[191][21]);
        let eq72_e1083_d_n22: f64 = self.ddt_jacobian(s.dn[191][22]);
        let eq72_e1083_d_n23: f64 = self.ddt_jacobian(s.dn[191][23]);
        let eq72_e1083_d_n24: f64 = self.ddt_jacobian(s.dn[191][24]);
        let eq72_e1083_d_n25: f64 = self.ddt_jacobian(s.dn[191][25]);
        let eq72_e1083_d_n26: f64 = self.ddt_jacobian(s.dn[191][26]);
        let eq72_e1083_d_n27: f64 = self.ddt_jacobian(s.dn[191][27]);
        let eq72_e1083_d_n28: f64 = self.ddt_jacobian(s.dn[191][28]);
        let eq72_e1083_d_n29: f64 = self.ddt_jacobian(s.dn[191][29]);
        let eq72_e1086: f64 = (p.p355 * (nv7 - nv5));
        let eq72_e1086_d_n5: f64 = (-p.p355);
        let eq72_e1086_d_n7: f64 = p.p355;
        let eq72_e1087: f64 = self.eval_ddt(53, eq72_e1086);
        let eq72_e1087_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n5: f64 = self.ddt_jacobian(eq72_e1086_d_n5);
        let eq72_e1087_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n7: f64 = self.ddt_jacobian(eq72_e1086_d_n7);
        let eq72_e1087_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq72_e1087_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq72_e1088: f64 = (eq72_e1083 + eq72_e1087);
        let eq72_e1088_d_n0: f64 = (eq72_e1083_d_n0 + eq72_e1087_d_n0);
        let eq72_e1088_d_n1: f64 = (eq72_e1083_d_n1 + eq72_e1087_d_n1);
        let eq72_e1088_d_n2: f64 = (eq72_e1083_d_n2 + eq72_e1087_d_n2);
        let eq72_e1088_d_n3: f64 = (eq72_e1083_d_n3 + eq72_e1087_d_n3);
        let eq72_e1088_d_n4: f64 = (eq72_e1083_d_n4 + eq72_e1087_d_n4);
        let eq72_e1088_d_n5: f64 = (eq72_e1083_d_n5 + eq72_e1087_d_n5);
        let eq72_e1088_d_n6: f64 = (eq72_e1083_d_n6 + eq72_e1087_d_n6);
        let eq72_e1088_d_n7: f64 = (eq72_e1083_d_n7 + eq72_e1087_d_n7);
        let eq72_e1088_d_n8: f64 = (eq72_e1083_d_n8 + eq72_e1087_d_n8);
        let eq72_e1088_d_n9: f64 = (eq72_e1083_d_n9 + eq72_e1087_d_n9);
        let eq72_e1088_d_n10: f64 = (eq72_e1083_d_n10 + eq72_e1087_d_n10);
        let eq72_e1088_d_n11: f64 = (eq72_e1083_d_n11 + eq72_e1087_d_n11);
        let eq72_e1088_d_n12: f64 = (eq72_e1083_d_n12 + eq72_e1087_d_n12);
        let eq72_e1088_d_n13: f64 = (eq72_e1083_d_n13 + eq72_e1087_d_n13);
        let eq72_e1088_d_n14: f64 = (eq72_e1083_d_n14 + eq72_e1087_d_n14);
        let eq72_e1088_d_n15: f64 = (eq72_e1083_d_n15 + eq72_e1087_d_n15);
        let eq72_e1088_d_n16: f64 = (eq72_e1083_d_n16 + eq72_e1087_d_n16);
        let eq72_e1088_d_n17: f64 = (eq72_e1083_d_n17 + eq72_e1087_d_n17);
        let eq72_e1088_d_n18: f64 = (eq72_e1083_d_n18 + eq72_e1087_d_n18);
        let eq72_e1088_d_n19: f64 = (eq72_e1083_d_n19 + eq72_e1087_d_n19);
        let eq72_e1088_d_n20: f64 = (eq72_e1083_d_n20 + eq72_e1087_d_n20);
        let eq72_e1088_d_n21: f64 = (eq72_e1083_d_n21 + eq72_e1087_d_n21);
        let eq72_e1088_d_n22: f64 = (eq72_e1083_d_n22 + eq72_e1087_d_n22);
        let eq72_e1088_d_n23: f64 = (eq72_e1083_d_n23 + eq72_e1087_d_n23);
        let eq72_e1088_d_n24: f64 = (eq72_e1083_d_n24 + eq72_e1087_d_n24);
        let eq72_e1088_d_n25: f64 = (eq72_e1083_d_n25 + eq72_e1087_d_n25);
        let eq72_e1088_d_n26: f64 = (eq72_e1083_d_n26 + eq72_e1087_d_n26);
        let eq72_e1088_d_n27: f64 = (eq72_e1083_d_n27 + eq72_e1087_d_n27);
        let eq72_e1088_d_n28: f64 = (eq72_e1083_d_n28 + eq72_e1087_d_n28);
        let eq72_e1088_d_n29: f64 = (eq72_e1083_d_n29 + eq72_e1087_d_n29);
        (eq72_e1088, eq72_e1088_d_n0, eq72_e1088_d_n1, eq72_e1088_d_n2, eq72_e1088_d_n3, eq72_e1088_d_n4, eq72_e1088_d_n5, eq72_e1088_d_n6, eq72_e1088_d_n7, eq72_e1088_d_n8, eq72_e1088_d_n9, eq72_e1088_d_n10, eq72_e1088_d_n11, eq72_e1088_d_n12, eq72_e1088_d_n13, eq72_e1088_d_n14, eq72_e1088_d_n15, eq72_e1088_d_n16, eq72_e1088_d_n17, eq72_e1088_d_n18, eq72_e1088_d_n19, eq72_e1088_d_n20, eq72_e1088_d_n21, eq72_e1088_d_n22, eq72_e1088_d_n23, eq72_e1088_d_n24, eq72_e1088_d_n25, eq72_e1088_d_n26, eq72_e1088_d_n27, eq72_e1088_d_n28, eq72_e1088_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_value: f64 = eq72_e1090;
        let eq72_node_derivatives: [f64; 30] = [eq72_e1090_d_n0, eq72_e1090_d_n1, eq72_e1090_d_n2, eq72_e1090_d_n3, eq72_e1090_d_n4, eq72_e1090_d_n5, eq72_e1090_d_n6, eq72_e1090_d_n7, eq72_e1090_d_n8, eq72_e1090_d_n9, eq72_e1090_d_n10, eq72_e1090_d_n11, eq72_e1090_d_n12, eq72_e1090_d_n13, eq72_e1090_d_n14, eq72_e1090_d_n15, eq72_e1090_d_n16, eq72_e1090_d_n17, eq72_e1090_d_n18, eq72_e1090_d_n19, eq72_e1090_d_n20, eq72_e1090_d_n21, eq72_e1090_d_n22, eq72_e1090_d_n23, eq72_e1090_d_n24, eq72_e1090_d_n25, eq72_e1090_d_n26, eq72_e1090_d_n27, eq72_e1090_d_n28, eq72_e1090_d_n29];
        let eq72_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            self.multiplicity * (eq72_value),
            &nodes,
            &eq72_node_derivatives,
            &branches,
            &eq72_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_73_block_0(
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
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq73_e1100, eq73_e1100_d_n0, eq73_e1100_d_n1, eq73_e1100_d_n2, eq73_e1100_d_n3, eq73_e1100_d_n4, eq73_e1100_d_n5, eq73_e1100_d_n6, eq73_e1100_d_n7, eq73_e1100_d_n8, eq73_e1100_d_n9, eq73_e1100_d_n10, eq73_e1100_d_n11, eq73_e1100_d_n12, eq73_e1100_d_n13, eq73_e1100_d_n14, eq73_e1100_d_n15, eq73_e1100_d_n16, eq73_e1100_d_n17, eq73_e1100_d_n18, eq73_e1100_d_n19, eq73_e1100_d_n20, eq73_e1100_d_n21, eq73_e1100_d_n22, eq73_e1100_d_n23, eq73_e1100_d_n24, eq73_e1100_d_n25, eq73_e1100_d_n26, eq73_e1100_d_n27, eq73_e1100_d_n28, eq73_e1100_d_n29,) = {
    if (s.v[907] != 0.0) {
        let eq73_e1093: f64 = self.eval_ddt(54, s.v[192]);
        let eq73_e1093_d_n0: f64 = self.ddt_jacobian(s.dn[192][0]);
        let eq73_e1093_d_n1: f64 = self.ddt_jacobian(s.dn[192][1]);
        let eq73_e1093_d_n2: f64 = self.ddt_jacobian(s.dn[192][2]);
        let eq73_e1093_d_n3: f64 = self.ddt_jacobian(s.dn[192][3]);
        let eq73_e1093_d_n4: f64 = self.ddt_jacobian(s.dn[192][4]);
        let eq73_e1093_d_n5: f64 = self.ddt_jacobian(s.dn[192][5]);
        let eq73_e1093_d_n6: f64 = self.ddt_jacobian(s.dn[192][6]);
        let eq73_e1093_d_n7: f64 = self.ddt_jacobian(s.dn[192][7]);
        let eq73_e1093_d_n8: f64 = self.ddt_jacobian(s.dn[192][8]);
        let eq73_e1093_d_n9: f64 = self.ddt_jacobian(s.dn[192][9]);
        let eq73_e1093_d_n10: f64 = self.ddt_jacobian(s.dn[192][10]);
        let eq73_e1093_d_n11: f64 = self.ddt_jacobian(s.dn[192][11]);
        let eq73_e1093_d_n12: f64 = self.ddt_jacobian(s.dn[192][12]);
        let eq73_e1093_d_n13: f64 = self.ddt_jacobian(s.dn[192][13]);
        let eq73_e1093_d_n14: f64 = self.ddt_jacobian(s.dn[192][14]);
        let eq73_e1093_d_n15: f64 = self.ddt_jacobian(s.dn[192][15]);
        let eq73_e1093_d_n16: f64 = self.ddt_jacobian(s.dn[192][16]);
        let eq73_e1093_d_n17: f64 = self.ddt_jacobian(s.dn[192][17]);
        let eq73_e1093_d_n18: f64 = self.ddt_jacobian(s.dn[192][18]);
        let eq73_e1093_d_n19: f64 = self.ddt_jacobian(s.dn[192][19]);
        let eq73_e1093_d_n20: f64 = self.ddt_jacobian(s.dn[192][20]);
        let eq73_e1093_d_n21: f64 = self.ddt_jacobian(s.dn[192][21]);
        let eq73_e1093_d_n22: f64 = self.ddt_jacobian(s.dn[192][22]);
        let eq73_e1093_d_n23: f64 = self.ddt_jacobian(s.dn[192][23]);
        let eq73_e1093_d_n24: f64 = self.ddt_jacobian(s.dn[192][24]);
        let eq73_e1093_d_n25: f64 = self.ddt_jacobian(s.dn[192][25]);
        let eq73_e1093_d_n26: f64 = self.ddt_jacobian(s.dn[192][26]);
        let eq73_e1093_d_n27: f64 = self.ddt_jacobian(s.dn[192][27]);
        let eq73_e1093_d_n28: f64 = self.ddt_jacobian(s.dn[192][28]);
        let eq73_e1093_d_n29: f64 = self.ddt_jacobian(s.dn[192][29]);
        let eq73_e1096: f64 = (p.p355 * (nv7 - nv14));
        let eq73_e1096_d_n7: f64 = p.p355;
        let eq73_e1096_d_n14: f64 = (-p.p355);
        let eq73_e1097: f64 = self.eval_ddt(55, eq73_e1096);
        let eq73_e1097_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n7: f64 = self.ddt_jacobian(eq73_e1096_d_n7);
        let eq73_e1097_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n14: f64 = self.ddt_jacobian(eq73_e1096_d_n14);
        let eq73_e1097_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq73_e1097_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq73_e1098: f64 = (eq73_e1093 + eq73_e1097);
        let eq73_e1098_d_n0: f64 = (eq73_e1093_d_n0 + eq73_e1097_d_n0);
        let eq73_e1098_d_n1: f64 = (eq73_e1093_d_n1 + eq73_e1097_d_n1);
        let eq73_e1098_d_n2: f64 = (eq73_e1093_d_n2 + eq73_e1097_d_n2);
        let eq73_e1098_d_n3: f64 = (eq73_e1093_d_n3 + eq73_e1097_d_n3);
        let eq73_e1098_d_n4: f64 = (eq73_e1093_d_n4 + eq73_e1097_d_n4);
        let eq73_e1098_d_n5: f64 = (eq73_e1093_d_n5 + eq73_e1097_d_n5);
        let eq73_e1098_d_n6: f64 = (eq73_e1093_d_n6 + eq73_e1097_d_n6);
        let eq73_e1098_d_n7: f64 = (eq73_e1093_d_n7 + eq73_e1097_d_n7);
        let eq73_e1098_d_n8: f64 = (eq73_e1093_d_n8 + eq73_e1097_d_n8);
        let eq73_e1098_d_n9: f64 = (eq73_e1093_d_n9 + eq73_e1097_d_n9);
        let eq73_e1098_d_n10: f64 = (eq73_e1093_d_n10 + eq73_e1097_d_n10);
        let eq73_e1098_d_n11: f64 = (eq73_e1093_d_n11 + eq73_e1097_d_n11);
        let eq73_e1098_d_n12: f64 = (eq73_e1093_d_n12 + eq73_e1097_d_n12);
        let eq73_e1098_d_n13: f64 = (eq73_e1093_d_n13 + eq73_e1097_d_n13);
        let eq73_e1098_d_n14: f64 = (eq73_e1093_d_n14 + eq73_e1097_d_n14);
        let eq73_e1098_d_n15: f64 = (eq73_e1093_d_n15 + eq73_e1097_d_n15);
        let eq73_e1098_d_n16: f64 = (eq73_e1093_d_n16 + eq73_e1097_d_n16);
        let eq73_e1098_d_n17: f64 = (eq73_e1093_d_n17 + eq73_e1097_d_n17);
        let eq73_e1098_d_n18: f64 = (eq73_e1093_d_n18 + eq73_e1097_d_n18);
        let eq73_e1098_d_n19: f64 = (eq73_e1093_d_n19 + eq73_e1097_d_n19);
        let eq73_e1098_d_n20: f64 = (eq73_e1093_d_n20 + eq73_e1097_d_n20);
        let eq73_e1098_d_n21: f64 = (eq73_e1093_d_n21 + eq73_e1097_d_n21);
        let eq73_e1098_d_n22: f64 = (eq73_e1093_d_n22 + eq73_e1097_d_n22);
        let eq73_e1098_d_n23: f64 = (eq73_e1093_d_n23 + eq73_e1097_d_n23);
        let eq73_e1098_d_n24: f64 = (eq73_e1093_d_n24 + eq73_e1097_d_n24);
        let eq73_e1098_d_n25: f64 = (eq73_e1093_d_n25 + eq73_e1097_d_n25);
        let eq73_e1098_d_n26: f64 = (eq73_e1093_d_n26 + eq73_e1097_d_n26);
        let eq73_e1098_d_n27: f64 = (eq73_e1093_d_n27 + eq73_e1097_d_n27);
        let eq73_e1098_d_n28: f64 = (eq73_e1093_d_n28 + eq73_e1097_d_n28);
        let eq73_e1098_d_n29: f64 = (eq73_e1093_d_n29 + eq73_e1097_d_n29);
        (eq73_e1098, eq73_e1098_d_n0, eq73_e1098_d_n1, eq73_e1098_d_n2, eq73_e1098_d_n3, eq73_e1098_d_n4, eq73_e1098_d_n5, eq73_e1098_d_n6, eq73_e1098_d_n7, eq73_e1098_d_n8, eq73_e1098_d_n9, eq73_e1098_d_n10, eq73_e1098_d_n11, eq73_e1098_d_n12, eq73_e1098_d_n13, eq73_e1098_d_n14, eq73_e1098_d_n15, eq73_e1098_d_n16, eq73_e1098_d_n17, eq73_e1098_d_n18, eq73_e1098_d_n19, eq73_e1098_d_n20, eq73_e1098_d_n21, eq73_e1098_d_n22, eq73_e1098_d_n23, eq73_e1098_d_n24, eq73_e1098_d_n25, eq73_e1098_d_n26, eq73_e1098_d_n27, eq73_e1098_d_n28, eq73_e1098_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_value: f64 = eq73_e1100;
        let eq73_node_derivatives: [f64; 30] = [eq73_e1100_d_n0, eq73_e1100_d_n1, eq73_e1100_d_n2, eq73_e1100_d_n3, eq73_e1100_d_n4, eq73_e1100_d_n5, eq73_e1100_d_n6, eq73_e1100_d_n7, eq73_e1100_d_n8, eq73_e1100_d_n9, eq73_e1100_d_n10, eq73_e1100_d_n11, eq73_e1100_d_n12, eq73_e1100_d_n13, eq73_e1100_d_n14, eq73_e1100_d_n15, eq73_e1100_d_n16, eq73_e1100_d_n17, eq73_e1100_d_n18, eq73_e1100_d_n19, eq73_e1100_d_n20, eq73_e1100_d_n21, eq73_e1100_d_n22, eq73_e1100_d_n23, eq73_e1100_d_n24, eq73_e1100_d_n25, eq73_e1100_d_n26, eq73_e1100_d_n27, eq73_e1100_d_n28, eq73_e1100_d_n29];
        let eq73_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[14]),
            self.multiplicity * (eq73_value),
            &nodes,
            &eq73_node_derivatives,
            &branches,
            &eq73_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_74_block_0(
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq74_e1110, eq74_e1110_d_n0, eq74_e1110_d_n1, eq74_e1110_d_n2, eq74_e1110_d_n3, eq74_e1110_d_n4, eq74_e1110_d_n5, eq74_e1110_d_n6, eq74_e1110_d_n7, eq74_e1110_d_n8, eq74_e1110_d_n9, eq74_e1110_d_n10, eq74_e1110_d_n11, eq74_e1110_d_n12, eq74_e1110_d_n13, eq74_e1110_d_n14, eq74_e1110_d_n15, eq74_e1110_d_n16, eq74_e1110_d_n17, eq74_e1110_d_n18, eq74_e1110_d_n19, eq74_e1110_d_n20, eq74_e1110_d_n21, eq74_e1110_d_n22, eq74_e1110_d_n23, eq74_e1110_d_n24, eq74_e1110_d_n25, eq74_e1110_d_n26, eq74_e1110_d_n27, eq74_e1110_d_n28, eq74_e1110_d_n29,) = {
    if (s.v[907] != 0.0) {
        let eq74_e1103: f64 = self.eval_ddt(56, s.v[193]);
        let eq74_e1103_d_n0: f64 = self.ddt_jacobian(s.dn[193][0]);
        let eq74_e1103_d_n1: f64 = self.ddt_jacobian(s.dn[193][1]);
        let eq74_e1103_d_n2: f64 = self.ddt_jacobian(s.dn[193][2]);
        let eq74_e1103_d_n3: f64 = self.ddt_jacobian(s.dn[193][3]);
        let eq74_e1103_d_n4: f64 = self.ddt_jacobian(s.dn[193][4]);
        let eq74_e1103_d_n5: f64 = self.ddt_jacobian(s.dn[193][5]);
        let eq74_e1103_d_n6: f64 = self.ddt_jacobian(s.dn[193][6]);
        let eq74_e1103_d_n7: f64 = self.ddt_jacobian(s.dn[193][7]);
        let eq74_e1103_d_n8: f64 = self.ddt_jacobian(s.dn[193][8]);
        let eq74_e1103_d_n9: f64 = self.ddt_jacobian(s.dn[193][9]);
        let eq74_e1103_d_n10: f64 = self.ddt_jacobian(s.dn[193][10]);
        let eq74_e1103_d_n11: f64 = self.ddt_jacobian(s.dn[193][11]);
        let eq74_e1103_d_n12: f64 = self.ddt_jacobian(s.dn[193][12]);
        let eq74_e1103_d_n13: f64 = self.ddt_jacobian(s.dn[193][13]);
        let eq74_e1103_d_n14: f64 = self.ddt_jacobian(s.dn[193][14]);
        let eq74_e1103_d_n15: f64 = self.ddt_jacobian(s.dn[193][15]);
        let eq74_e1103_d_n16: f64 = self.ddt_jacobian(s.dn[193][16]);
        let eq74_e1103_d_n17: f64 = self.ddt_jacobian(s.dn[193][17]);
        let eq74_e1103_d_n18: f64 = self.ddt_jacobian(s.dn[193][18]);
        let eq74_e1103_d_n19: f64 = self.ddt_jacobian(s.dn[193][19]);
        let eq74_e1103_d_n20: f64 = self.ddt_jacobian(s.dn[193][20]);
        let eq74_e1103_d_n21: f64 = self.ddt_jacobian(s.dn[193][21]);
        let eq74_e1103_d_n22: f64 = self.ddt_jacobian(s.dn[193][22]);
        let eq74_e1103_d_n23: f64 = self.ddt_jacobian(s.dn[193][23]);
        let eq74_e1103_d_n24: f64 = self.ddt_jacobian(s.dn[193][24]);
        let eq74_e1103_d_n25: f64 = self.ddt_jacobian(s.dn[193][25]);
        let eq74_e1103_d_n26: f64 = self.ddt_jacobian(s.dn[193][26]);
        let eq74_e1103_d_n27: f64 = self.ddt_jacobian(s.dn[193][27]);
        let eq74_e1103_d_n28: f64 = self.ddt_jacobian(s.dn[193][28]);
        let eq74_e1103_d_n29: f64 = self.ddt_jacobian(s.dn[193][29]);
        let eq74_e1106: f64 = (p.p355 * (nv2 - nv5));
        let eq74_e1106_d_n2: f64 = p.p355;
        let eq74_e1106_d_n5: f64 = (-p.p355);
        let eq74_e1107: f64 = self.eval_ddt(57, eq74_e1106);
        let eq74_e1107_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n2: f64 = self.ddt_jacobian(eq74_e1106_d_n2);
        let eq74_e1107_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n5: f64 = self.ddt_jacobian(eq74_e1106_d_n5);
        let eq74_e1107_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq74_e1107_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq74_e1108: f64 = (eq74_e1103 + eq74_e1107);
        let eq74_e1108_d_n0: f64 = (eq74_e1103_d_n0 + eq74_e1107_d_n0);
        let eq74_e1108_d_n1: f64 = (eq74_e1103_d_n1 + eq74_e1107_d_n1);
        let eq74_e1108_d_n2: f64 = (eq74_e1103_d_n2 + eq74_e1107_d_n2);
        let eq74_e1108_d_n3: f64 = (eq74_e1103_d_n3 + eq74_e1107_d_n3);
        let eq74_e1108_d_n4: f64 = (eq74_e1103_d_n4 + eq74_e1107_d_n4);
        let eq74_e1108_d_n5: f64 = (eq74_e1103_d_n5 + eq74_e1107_d_n5);
        let eq74_e1108_d_n6: f64 = (eq74_e1103_d_n6 + eq74_e1107_d_n6);
        let eq74_e1108_d_n7: f64 = (eq74_e1103_d_n7 + eq74_e1107_d_n7);
        let eq74_e1108_d_n8: f64 = (eq74_e1103_d_n8 + eq74_e1107_d_n8);
        let eq74_e1108_d_n9: f64 = (eq74_e1103_d_n9 + eq74_e1107_d_n9);
        let eq74_e1108_d_n10: f64 = (eq74_e1103_d_n10 + eq74_e1107_d_n10);
        let eq74_e1108_d_n11: f64 = (eq74_e1103_d_n11 + eq74_e1107_d_n11);
        let eq74_e1108_d_n12: f64 = (eq74_e1103_d_n12 + eq74_e1107_d_n12);
        let eq74_e1108_d_n13: f64 = (eq74_e1103_d_n13 + eq74_e1107_d_n13);
        let eq74_e1108_d_n14: f64 = (eq74_e1103_d_n14 + eq74_e1107_d_n14);
        let eq74_e1108_d_n15: f64 = (eq74_e1103_d_n15 + eq74_e1107_d_n15);
        let eq74_e1108_d_n16: f64 = (eq74_e1103_d_n16 + eq74_e1107_d_n16);
        let eq74_e1108_d_n17: f64 = (eq74_e1103_d_n17 + eq74_e1107_d_n17);
        let eq74_e1108_d_n18: f64 = (eq74_e1103_d_n18 + eq74_e1107_d_n18);
        let eq74_e1108_d_n19: f64 = (eq74_e1103_d_n19 + eq74_e1107_d_n19);
        let eq74_e1108_d_n20: f64 = (eq74_e1103_d_n20 + eq74_e1107_d_n20);
        let eq74_e1108_d_n21: f64 = (eq74_e1103_d_n21 + eq74_e1107_d_n21);
        let eq74_e1108_d_n22: f64 = (eq74_e1103_d_n22 + eq74_e1107_d_n22);
        let eq74_e1108_d_n23: f64 = (eq74_e1103_d_n23 + eq74_e1107_d_n23);
        let eq74_e1108_d_n24: f64 = (eq74_e1103_d_n24 + eq74_e1107_d_n24);
        let eq74_e1108_d_n25: f64 = (eq74_e1103_d_n25 + eq74_e1107_d_n25);
        let eq74_e1108_d_n26: f64 = (eq74_e1103_d_n26 + eq74_e1107_d_n26);
        let eq74_e1108_d_n27: f64 = (eq74_e1103_d_n27 + eq74_e1107_d_n27);
        let eq74_e1108_d_n28: f64 = (eq74_e1103_d_n28 + eq74_e1107_d_n28);
        let eq74_e1108_d_n29: f64 = (eq74_e1103_d_n29 + eq74_e1107_d_n29);
        (eq74_e1108, eq74_e1108_d_n0, eq74_e1108_d_n1, eq74_e1108_d_n2, eq74_e1108_d_n3, eq74_e1108_d_n4, eq74_e1108_d_n5, eq74_e1108_d_n6, eq74_e1108_d_n7, eq74_e1108_d_n8, eq74_e1108_d_n9, eq74_e1108_d_n10, eq74_e1108_d_n11, eq74_e1108_d_n12, eq74_e1108_d_n13, eq74_e1108_d_n14, eq74_e1108_d_n15, eq74_e1108_d_n16, eq74_e1108_d_n17, eq74_e1108_d_n18, eq74_e1108_d_n19, eq74_e1108_d_n20, eq74_e1108_d_n21, eq74_e1108_d_n22, eq74_e1108_d_n23, eq74_e1108_d_n24, eq74_e1108_d_n25, eq74_e1108_d_n26, eq74_e1108_d_n27, eq74_e1108_d_n28, eq74_e1108_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq74_value: f64 = eq74_e1110;
        let eq74_node_derivatives: [f64; 30] = [eq74_e1110_d_n0, eq74_e1110_d_n1, eq74_e1110_d_n2, eq74_e1110_d_n3, eq74_e1110_d_n4, eq74_e1110_d_n5, eq74_e1110_d_n6, eq74_e1110_d_n7, eq74_e1110_d_n8, eq74_e1110_d_n9, eq74_e1110_d_n10, eq74_e1110_d_n11, eq74_e1110_d_n12, eq74_e1110_d_n13, eq74_e1110_d_n14, eq74_e1110_d_n15, eq74_e1110_d_n16, eq74_e1110_d_n17, eq74_e1110_d_n18, eq74_e1110_d_n19, eq74_e1110_d_n20, eq74_e1110_d_n21, eq74_e1110_d_n22, eq74_e1110_d_n23, eq74_e1110_d_n24, eq74_e1110_d_n25, eq74_e1110_d_n26, eq74_e1110_d_n27, eq74_e1110_d_n28, eq74_e1110_d_n29];
        let eq74_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[5]),
            self.multiplicity * (eq74_value),
            &nodes,
            &eq74_node_derivatives,
            &branches,
            &eq74_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_75_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq75_e1114,) = {
    if (s.v[907] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq75_value: f64 = eq75_e1114;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[14]),
            self.multiplicity * (eq75_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_76_block_0(
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
        let (eq76_e1124, eq76_e1124_d_n0, eq76_e1124_d_n1, eq76_e1124_d_n2, eq76_e1124_d_n3, eq76_e1124_d_n4, eq76_e1124_d_n5, eq76_e1124_d_n6, eq76_e1124_d_n7, eq76_e1124_d_n8, eq76_e1124_d_n9, eq76_e1124_d_n10, eq76_e1124_d_n11, eq76_e1124_d_n12, eq76_e1124_d_n13, eq76_e1124_d_n14, eq76_e1124_d_n15, eq76_e1124_d_n16, eq76_e1124_d_n17, eq76_e1124_d_n18, eq76_e1124_d_n19, eq76_e1124_d_n20, eq76_e1124_d_n21, eq76_e1124_d_n22, eq76_e1124_d_n23, eq76_e1124_d_n24, eq76_e1124_d_n25, eq76_e1124_d_n26, eq76_e1124_d_n27, eq76_e1124_d_n28, eq76_e1124_d_n29,) = {
    if (s.v[907] != 0.0) {
        let eq76_e1117: f64 = self.eval_ddt(58, s.v[195]);
        let eq76_e1117_d_n0: f64 = self.ddt_jacobian(s.dn[195][0]);
        let eq76_e1117_d_n1: f64 = self.ddt_jacobian(s.dn[195][1]);
        let eq76_e1117_d_n2: f64 = self.ddt_jacobian(s.dn[195][2]);
        let eq76_e1117_d_n3: f64 = self.ddt_jacobian(s.dn[195][3]);
        let eq76_e1117_d_n4: f64 = self.ddt_jacobian(s.dn[195][4]);
        let eq76_e1117_d_n5: f64 = self.ddt_jacobian(s.dn[195][5]);
        let eq76_e1117_d_n6: f64 = self.ddt_jacobian(s.dn[195][6]);
        let eq76_e1117_d_n7: f64 = self.ddt_jacobian(s.dn[195][7]);
        let eq76_e1117_d_n8: f64 = self.ddt_jacobian(s.dn[195][8]);
        let eq76_e1117_d_n9: f64 = self.ddt_jacobian(s.dn[195][9]);
        let eq76_e1117_d_n10: f64 = self.ddt_jacobian(s.dn[195][10]);
        let eq76_e1117_d_n11: f64 = self.ddt_jacobian(s.dn[195][11]);
        let eq76_e1117_d_n12: f64 = self.ddt_jacobian(s.dn[195][12]);
        let eq76_e1117_d_n13: f64 = self.ddt_jacobian(s.dn[195][13]);
        let eq76_e1117_d_n14: f64 = self.ddt_jacobian(s.dn[195][14]);
        let eq76_e1117_d_n15: f64 = self.ddt_jacobian(s.dn[195][15]);
        let eq76_e1117_d_n16: f64 = self.ddt_jacobian(s.dn[195][16]);
        let eq76_e1117_d_n17: f64 = self.ddt_jacobian(s.dn[195][17]);
        let eq76_e1117_d_n18: f64 = self.ddt_jacobian(s.dn[195][18]);
        let eq76_e1117_d_n19: f64 = self.ddt_jacobian(s.dn[195][19]);
        let eq76_e1117_d_n20: f64 = self.ddt_jacobian(s.dn[195][20]);
        let eq76_e1117_d_n21: f64 = self.ddt_jacobian(s.dn[195][21]);
        let eq76_e1117_d_n22: f64 = self.ddt_jacobian(s.dn[195][22]);
        let eq76_e1117_d_n23: f64 = self.ddt_jacobian(s.dn[195][23]);
        let eq76_e1117_d_n24: f64 = self.ddt_jacobian(s.dn[195][24]);
        let eq76_e1117_d_n25: f64 = self.ddt_jacobian(s.dn[195][25]);
        let eq76_e1117_d_n26: f64 = self.ddt_jacobian(s.dn[195][26]);
        let eq76_e1117_d_n27: f64 = self.ddt_jacobian(s.dn[195][27]);
        let eq76_e1117_d_n28: f64 = self.ddt_jacobian(s.dn[195][28]);
        let eq76_e1117_d_n29: f64 = self.ddt_jacobian(s.dn[195][29]);
        let eq76_e1120: f64 = (p.p355 * (nv7 - nv9));
        let eq76_e1120_d_n7: f64 = p.p355;
        let eq76_e1120_d_n9: f64 = (-p.p355);
        let eq76_e1121: f64 = self.eval_ddt(59, eq76_e1120);
        let eq76_e1121_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n7: f64 = self.ddt_jacobian(eq76_e1120_d_n7);
        let eq76_e1121_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n9: f64 = self.ddt_jacobian(eq76_e1120_d_n9);
        let eq76_e1121_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq76_e1121_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq76_e1122: f64 = (eq76_e1117 + eq76_e1121);
        let eq76_e1122_d_n0: f64 = (eq76_e1117_d_n0 + eq76_e1121_d_n0);
        let eq76_e1122_d_n1: f64 = (eq76_e1117_d_n1 + eq76_e1121_d_n1);
        let eq76_e1122_d_n2: f64 = (eq76_e1117_d_n2 + eq76_e1121_d_n2);
        let eq76_e1122_d_n3: f64 = (eq76_e1117_d_n3 + eq76_e1121_d_n3);
        let eq76_e1122_d_n4: f64 = (eq76_e1117_d_n4 + eq76_e1121_d_n4);
        let eq76_e1122_d_n5: f64 = (eq76_e1117_d_n5 + eq76_e1121_d_n5);
        let eq76_e1122_d_n6: f64 = (eq76_e1117_d_n6 + eq76_e1121_d_n6);
        let eq76_e1122_d_n7: f64 = (eq76_e1117_d_n7 + eq76_e1121_d_n7);
        let eq76_e1122_d_n8: f64 = (eq76_e1117_d_n8 + eq76_e1121_d_n8);
        let eq76_e1122_d_n9: f64 = (eq76_e1117_d_n9 + eq76_e1121_d_n9);
        let eq76_e1122_d_n10: f64 = (eq76_e1117_d_n10 + eq76_e1121_d_n10);
        let eq76_e1122_d_n11: f64 = (eq76_e1117_d_n11 + eq76_e1121_d_n11);
        let eq76_e1122_d_n12: f64 = (eq76_e1117_d_n12 + eq76_e1121_d_n12);
        let eq76_e1122_d_n13: f64 = (eq76_e1117_d_n13 + eq76_e1121_d_n13);
        let eq76_e1122_d_n14: f64 = (eq76_e1117_d_n14 + eq76_e1121_d_n14);
        let eq76_e1122_d_n15: f64 = (eq76_e1117_d_n15 + eq76_e1121_d_n15);
        let eq76_e1122_d_n16: f64 = (eq76_e1117_d_n16 + eq76_e1121_d_n16);
        let eq76_e1122_d_n17: f64 = (eq76_e1117_d_n17 + eq76_e1121_d_n17);
        let eq76_e1122_d_n18: f64 = (eq76_e1117_d_n18 + eq76_e1121_d_n18);
        let eq76_e1122_d_n19: f64 = (eq76_e1117_d_n19 + eq76_e1121_d_n19);
        let eq76_e1122_d_n20: f64 = (eq76_e1117_d_n20 + eq76_e1121_d_n20);
        let eq76_e1122_d_n21: f64 = (eq76_e1117_d_n21 + eq76_e1121_d_n21);
        let eq76_e1122_d_n22: f64 = (eq76_e1117_d_n22 + eq76_e1121_d_n22);
        let eq76_e1122_d_n23: f64 = (eq76_e1117_d_n23 + eq76_e1121_d_n23);
        let eq76_e1122_d_n24: f64 = (eq76_e1117_d_n24 + eq76_e1121_d_n24);
        let eq76_e1122_d_n25: f64 = (eq76_e1117_d_n25 + eq76_e1121_d_n25);
        let eq76_e1122_d_n26: f64 = (eq76_e1117_d_n26 + eq76_e1121_d_n26);
        let eq76_e1122_d_n27: f64 = (eq76_e1117_d_n27 + eq76_e1121_d_n27);
        let eq76_e1122_d_n28: f64 = (eq76_e1117_d_n28 + eq76_e1121_d_n28);
        let eq76_e1122_d_n29: f64 = (eq76_e1117_d_n29 + eq76_e1121_d_n29);
        (eq76_e1122, eq76_e1122_d_n0, eq76_e1122_d_n1, eq76_e1122_d_n2, eq76_e1122_d_n3, eq76_e1122_d_n4, eq76_e1122_d_n5, eq76_e1122_d_n6, eq76_e1122_d_n7, eq76_e1122_d_n8, eq76_e1122_d_n9, eq76_e1122_d_n10, eq76_e1122_d_n11, eq76_e1122_d_n12, eq76_e1122_d_n13, eq76_e1122_d_n14, eq76_e1122_d_n15, eq76_e1122_d_n16, eq76_e1122_d_n17, eq76_e1122_d_n18, eq76_e1122_d_n19, eq76_e1122_d_n20, eq76_e1122_d_n21, eq76_e1122_d_n22, eq76_e1122_d_n23, eq76_e1122_d_n24, eq76_e1122_d_n25, eq76_e1122_d_n26, eq76_e1122_d_n27, eq76_e1122_d_n28, eq76_e1122_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_value: f64 = eq76_e1124;
        let eq76_node_derivatives: [f64; 30] = [eq76_e1124_d_n0, eq76_e1124_d_n1, eq76_e1124_d_n2, eq76_e1124_d_n3, eq76_e1124_d_n4, eq76_e1124_d_n5, eq76_e1124_d_n6, eq76_e1124_d_n7, eq76_e1124_d_n8, eq76_e1124_d_n9, eq76_e1124_d_n10, eq76_e1124_d_n11, eq76_e1124_d_n12, eq76_e1124_d_n13, eq76_e1124_d_n14, eq76_e1124_d_n15, eq76_e1124_d_n16, eq76_e1124_d_n17, eq76_e1124_d_n18, eq76_e1124_d_n19, eq76_e1124_d_n20, eq76_e1124_d_n21, eq76_e1124_d_n22, eq76_e1124_d_n23, eq76_e1124_d_n24, eq76_e1124_d_n25, eq76_e1124_d_n26, eq76_e1124_d_n27, eq76_e1124_d_n28, eq76_e1124_d_n29];
        let eq76_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            self.multiplicity * (eq76_value),
            &nodes,
            &eq76_node_derivatives,
            &branches,
            &eq76_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_77_block_0(
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq77_e1135, eq77_e1135_d_n0, eq77_e1135_d_n1, eq77_e1135_d_n2, eq77_e1135_d_n3, eq77_e1135_d_n4, eq77_e1135_d_n5, eq77_e1135_d_n6, eq77_e1135_d_n7, eq77_e1135_d_n8, eq77_e1135_d_n9, eq77_e1135_d_n10, eq77_e1135_d_n11, eq77_e1135_d_n12, eq77_e1135_d_n13, eq77_e1135_d_n14, eq77_e1135_d_n15, eq77_e1135_d_n16, eq77_e1135_d_n17, eq77_e1135_d_n18, eq77_e1135_d_n19, eq77_e1135_d_n20, eq77_e1135_d_n21, eq77_e1135_d_n22, eq77_e1135_d_n23, eq77_e1135_d_n24, eq77_e1135_d_n25, eq77_e1135_d_n26, eq77_e1135_d_n27, eq77_e1135_d_n28, eq77_e1135_d_n29,) = {
    if (!(s.v[907] != 0.0)) {
        let eq77_e1128: f64 = self.eval_ddt(60, s.v[191]);
        let eq77_e1128_d_n0: f64 = self.ddt_jacobian(s.dn[191][0]);
        let eq77_e1128_d_n1: f64 = self.ddt_jacobian(s.dn[191][1]);
        let eq77_e1128_d_n2: f64 = self.ddt_jacobian(s.dn[191][2]);
        let eq77_e1128_d_n3: f64 = self.ddt_jacobian(s.dn[191][3]);
        let eq77_e1128_d_n4: f64 = self.ddt_jacobian(s.dn[191][4]);
        let eq77_e1128_d_n5: f64 = self.ddt_jacobian(s.dn[191][5]);
        let eq77_e1128_d_n6: f64 = self.ddt_jacobian(s.dn[191][6]);
        let eq77_e1128_d_n7: f64 = self.ddt_jacobian(s.dn[191][7]);
        let eq77_e1128_d_n8: f64 = self.ddt_jacobian(s.dn[191][8]);
        let eq77_e1128_d_n9: f64 = self.ddt_jacobian(s.dn[191][9]);
        let eq77_e1128_d_n10: f64 = self.ddt_jacobian(s.dn[191][10]);
        let eq77_e1128_d_n11: f64 = self.ddt_jacobian(s.dn[191][11]);
        let eq77_e1128_d_n12: f64 = self.ddt_jacobian(s.dn[191][12]);
        let eq77_e1128_d_n13: f64 = self.ddt_jacobian(s.dn[191][13]);
        let eq77_e1128_d_n14: f64 = self.ddt_jacobian(s.dn[191][14]);
        let eq77_e1128_d_n15: f64 = self.ddt_jacobian(s.dn[191][15]);
        let eq77_e1128_d_n16: f64 = self.ddt_jacobian(s.dn[191][16]);
        let eq77_e1128_d_n17: f64 = self.ddt_jacobian(s.dn[191][17]);
        let eq77_e1128_d_n18: f64 = self.ddt_jacobian(s.dn[191][18]);
        let eq77_e1128_d_n19: f64 = self.ddt_jacobian(s.dn[191][19]);
        let eq77_e1128_d_n20: f64 = self.ddt_jacobian(s.dn[191][20]);
        let eq77_e1128_d_n21: f64 = self.ddt_jacobian(s.dn[191][21]);
        let eq77_e1128_d_n22: f64 = self.ddt_jacobian(s.dn[191][22]);
        let eq77_e1128_d_n23: f64 = self.ddt_jacobian(s.dn[191][23]);
        let eq77_e1128_d_n24: f64 = self.ddt_jacobian(s.dn[191][24]);
        let eq77_e1128_d_n25: f64 = self.ddt_jacobian(s.dn[191][25]);
        let eq77_e1128_d_n26: f64 = self.ddt_jacobian(s.dn[191][26]);
        let eq77_e1128_d_n27: f64 = self.ddt_jacobian(s.dn[191][27]);
        let eq77_e1128_d_n28: f64 = self.ddt_jacobian(s.dn[191][28]);
        let eq77_e1128_d_n29: f64 = self.ddt_jacobian(s.dn[191][29]);
        let eq77_e1131: f64 = (p.p355 * (nv2 - nv5));
        let eq77_e1131_d_n2: f64 = p.p355;
        let eq77_e1131_d_n5: f64 = (-p.p355);
        let eq77_e1132: f64 = self.eval_ddt(61, eq77_e1131);
        let eq77_e1132_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n2: f64 = self.ddt_jacobian(eq77_e1131_d_n2);
        let eq77_e1132_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n5: f64 = self.ddt_jacobian(eq77_e1131_d_n5);
        let eq77_e1132_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq77_e1132_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq77_e1133: f64 = (eq77_e1128 + eq77_e1132);
        let eq77_e1133_d_n0: f64 = (eq77_e1128_d_n0 + eq77_e1132_d_n0);
        let eq77_e1133_d_n1: f64 = (eq77_e1128_d_n1 + eq77_e1132_d_n1);
        let eq77_e1133_d_n2: f64 = (eq77_e1128_d_n2 + eq77_e1132_d_n2);
        let eq77_e1133_d_n3: f64 = (eq77_e1128_d_n3 + eq77_e1132_d_n3);
        let eq77_e1133_d_n4: f64 = (eq77_e1128_d_n4 + eq77_e1132_d_n4);
        let eq77_e1133_d_n5: f64 = (eq77_e1128_d_n5 + eq77_e1132_d_n5);
        let eq77_e1133_d_n6: f64 = (eq77_e1128_d_n6 + eq77_e1132_d_n6);
        let eq77_e1133_d_n7: f64 = (eq77_e1128_d_n7 + eq77_e1132_d_n7);
        let eq77_e1133_d_n8: f64 = (eq77_e1128_d_n8 + eq77_e1132_d_n8);
        let eq77_e1133_d_n9: f64 = (eq77_e1128_d_n9 + eq77_e1132_d_n9);
        let eq77_e1133_d_n10: f64 = (eq77_e1128_d_n10 + eq77_e1132_d_n10);
        let eq77_e1133_d_n11: f64 = (eq77_e1128_d_n11 + eq77_e1132_d_n11);
        let eq77_e1133_d_n12: f64 = (eq77_e1128_d_n12 + eq77_e1132_d_n12);
        let eq77_e1133_d_n13: f64 = (eq77_e1128_d_n13 + eq77_e1132_d_n13);
        let eq77_e1133_d_n14: f64 = (eq77_e1128_d_n14 + eq77_e1132_d_n14);
        let eq77_e1133_d_n15: f64 = (eq77_e1128_d_n15 + eq77_e1132_d_n15);
        let eq77_e1133_d_n16: f64 = (eq77_e1128_d_n16 + eq77_e1132_d_n16);
        let eq77_e1133_d_n17: f64 = (eq77_e1128_d_n17 + eq77_e1132_d_n17);
        let eq77_e1133_d_n18: f64 = (eq77_e1128_d_n18 + eq77_e1132_d_n18);
        let eq77_e1133_d_n19: f64 = (eq77_e1128_d_n19 + eq77_e1132_d_n19);
        let eq77_e1133_d_n20: f64 = (eq77_e1128_d_n20 + eq77_e1132_d_n20);
        let eq77_e1133_d_n21: f64 = (eq77_e1128_d_n21 + eq77_e1132_d_n21);
        let eq77_e1133_d_n22: f64 = (eq77_e1128_d_n22 + eq77_e1132_d_n22);
        let eq77_e1133_d_n23: f64 = (eq77_e1128_d_n23 + eq77_e1132_d_n23);
        let eq77_e1133_d_n24: f64 = (eq77_e1128_d_n24 + eq77_e1132_d_n24);
        let eq77_e1133_d_n25: f64 = (eq77_e1128_d_n25 + eq77_e1132_d_n25);
        let eq77_e1133_d_n26: f64 = (eq77_e1128_d_n26 + eq77_e1132_d_n26);
        let eq77_e1133_d_n27: f64 = (eq77_e1128_d_n27 + eq77_e1132_d_n27);
        let eq77_e1133_d_n28: f64 = (eq77_e1128_d_n28 + eq77_e1132_d_n28);
        let eq77_e1133_d_n29: f64 = (eq77_e1128_d_n29 + eq77_e1132_d_n29);
        (eq77_e1133, eq77_e1133_d_n0, eq77_e1133_d_n1, eq77_e1133_d_n2, eq77_e1133_d_n3, eq77_e1133_d_n4, eq77_e1133_d_n5, eq77_e1133_d_n6, eq77_e1133_d_n7, eq77_e1133_d_n8, eq77_e1133_d_n9, eq77_e1133_d_n10, eq77_e1133_d_n11, eq77_e1133_d_n12, eq77_e1133_d_n13, eq77_e1133_d_n14, eq77_e1133_d_n15, eq77_e1133_d_n16, eq77_e1133_d_n17, eq77_e1133_d_n18, eq77_e1133_d_n19, eq77_e1133_d_n20, eq77_e1133_d_n21, eq77_e1133_d_n22, eq77_e1133_d_n23, eq77_e1133_d_n24, eq77_e1133_d_n25, eq77_e1133_d_n26, eq77_e1133_d_n27, eq77_e1133_d_n28, eq77_e1133_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_value: f64 = eq77_e1135;
        let eq77_node_derivatives: [f64; 30] = [eq77_e1135_d_n0, eq77_e1135_d_n1, eq77_e1135_d_n2, eq77_e1135_d_n3, eq77_e1135_d_n4, eq77_e1135_d_n5, eq77_e1135_d_n6, eq77_e1135_d_n7, eq77_e1135_d_n8, eq77_e1135_d_n9, eq77_e1135_d_n10, eq77_e1135_d_n11, eq77_e1135_d_n12, eq77_e1135_d_n13, eq77_e1135_d_n14, eq77_e1135_d_n15, eq77_e1135_d_n16, eq77_e1135_d_n17, eq77_e1135_d_n18, eq77_e1135_d_n19, eq77_e1135_d_n20, eq77_e1135_d_n21, eq77_e1135_d_n22, eq77_e1135_d_n23, eq77_e1135_d_n24, eq77_e1135_d_n25, eq77_e1135_d_n26, eq77_e1135_d_n27, eq77_e1135_d_n28, eq77_e1135_d_n29];
        let eq77_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[5]),
            self.multiplicity * (eq77_value),
            &nodes,
            &eq77_node_derivatives,
            &branches,
            &eq77_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_78_block_0(
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
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq78_e1146, eq78_e1146_d_n0, eq78_e1146_d_n1, eq78_e1146_d_n2, eq78_e1146_d_n3, eq78_e1146_d_n4, eq78_e1146_d_n5, eq78_e1146_d_n6, eq78_e1146_d_n7, eq78_e1146_d_n8, eq78_e1146_d_n9, eq78_e1146_d_n10, eq78_e1146_d_n11, eq78_e1146_d_n12, eq78_e1146_d_n13, eq78_e1146_d_n14, eq78_e1146_d_n15, eq78_e1146_d_n16, eq78_e1146_d_n17, eq78_e1146_d_n18, eq78_e1146_d_n19, eq78_e1146_d_n20, eq78_e1146_d_n21, eq78_e1146_d_n22, eq78_e1146_d_n23, eq78_e1146_d_n24, eq78_e1146_d_n25, eq78_e1146_d_n26, eq78_e1146_d_n27, eq78_e1146_d_n28, eq78_e1146_d_n29,) = {
    if (!(s.v[907] != 0.0)) {
        let eq78_e1139: f64 = self.eval_ddt(62, s.v[192]);
        let eq78_e1139_d_n0: f64 = self.ddt_jacobian(s.dn[192][0]);
        let eq78_e1139_d_n1: f64 = self.ddt_jacobian(s.dn[192][1]);
        let eq78_e1139_d_n2: f64 = self.ddt_jacobian(s.dn[192][2]);
        let eq78_e1139_d_n3: f64 = self.ddt_jacobian(s.dn[192][3]);
        let eq78_e1139_d_n4: f64 = self.ddt_jacobian(s.dn[192][4]);
        let eq78_e1139_d_n5: f64 = self.ddt_jacobian(s.dn[192][5]);
        let eq78_e1139_d_n6: f64 = self.ddt_jacobian(s.dn[192][6]);
        let eq78_e1139_d_n7: f64 = self.ddt_jacobian(s.dn[192][7]);
        let eq78_e1139_d_n8: f64 = self.ddt_jacobian(s.dn[192][8]);
        let eq78_e1139_d_n9: f64 = self.ddt_jacobian(s.dn[192][9]);
        let eq78_e1139_d_n10: f64 = self.ddt_jacobian(s.dn[192][10]);
        let eq78_e1139_d_n11: f64 = self.ddt_jacobian(s.dn[192][11]);
        let eq78_e1139_d_n12: f64 = self.ddt_jacobian(s.dn[192][12]);
        let eq78_e1139_d_n13: f64 = self.ddt_jacobian(s.dn[192][13]);
        let eq78_e1139_d_n14: f64 = self.ddt_jacobian(s.dn[192][14]);
        let eq78_e1139_d_n15: f64 = self.ddt_jacobian(s.dn[192][15]);
        let eq78_e1139_d_n16: f64 = self.ddt_jacobian(s.dn[192][16]);
        let eq78_e1139_d_n17: f64 = self.ddt_jacobian(s.dn[192][17]);
        let eq78_e1139_d_n18: f64 = self.ddt_jacobian(s.dn[192][18]);
        let eq78_e1139_d_n19: f64 = self.ddt_jacobian(s.dn[192][19]);
        let eq78_e1139_d_n20: f64 = self.ddt_jacobian(s.dn[192][20]);
        let eq78_e1139_d_n21: f64 = self.ddt_jacobian(s.dn[192][21]);
        let eq78_e1139_d_n22: f64 = self.ddt_jacobian(s.dn[192][22]);
        let eq78_e1139_d_n23: f64 = self.ddt_jacobian(s.dn[192][23]);
        let eq78_e1139_d_n24: f64 = self.ddt_jacobian(s.dn[192][24]);
        let eq78_e1139_d_n25: f64 = self.ddt_jacobian(s.dn[192][25]);
        let eq78_e1139_d_n26: f64 = self.ddt_jacobian(s.dn[192][26]);
        let eq78_e1139_d_n27: f64 = self.ddt_jacobian(s.dn[192][27]);
        let eq78_e1139_d_n28: f64 = self.ddt_jacobian(s.dn[192][28]);
        let eq78_e1139_d_n29: f64 = self.ddt_jacobian(s.dn[192][29]);
        let eq78_e1142: f64 = (p.p355 * (nv2 - nv14));
        let eq78_e1142_d_n2: f64 = p.p355;
        let eq78_e1142_d_n14: f64 = (-p.p355);
        let eq78_e1143: f64 = self.eval_ddt(63, eq78_e1142);
        let eq78_e1143_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n2: f64 = self.ddt_jacobian(eq78_e1142_d_n2);
        let eq78_e1143_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n14: f64 = self.ddt_jacobian(eq78_e1142_d_n14);
        let eq78_e1143_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq78_e1143_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq78_e1144: f64 = (eq78_e1139 + eq78_e1143);
        let eq78_e1144_d_n0: f64 = (eq78_e1139_d_n0 + eq78_e1143_d_n0);
        let eq78_e1144_d_n1: f64 = (eq78_e1139_d_n1 + eq78_e1143_d_n1);
        let eq78_e1144_d_n2: f64 = (eq78_e1139_d_n2 + eq78_e1143_d_n2);
        let eq78_e1144_d_n3: f64 = (eq78_e1139_d_n3 + eq78_e1143_d_n3);
        let eq78_e1144_d_n4: f64 = (eq78_e1139_d_n4 + eq78_e1143_d_n4);
        let eq78_e1144_d_n5: f64 = (eq78_e1139_d_n5 + eq78_e1143_d_n5);
        let eq78_e1144_d_n6: f64 = (eq78_e1139_d_n6 + eq78_e1143_d_n6);
        let eq78_e1144_d_n7: f64 = (eq78_e1139_d_n7 + eq78_e1143_d_n7);
        let eq78_e1144_d_n8: f64 = (eq78_e1139_d_n8 + eq78_e1143_d_n8);
        let eq78_e1144_d_n9: f64 = (eq78_e1139_d_n9 + eq78_e1143_d_n9);
        let eq78_e1144_d_n10: f64 = (eq78_e1139_d_n10 + eq78_e1143_d_n10);
        let eq78_e1144_d_n11: f64 = (eq78_e1139_d_n11 + eq78_e1143_d_n11);
        let eq78_e1144_d_n12: f64 = (eq78_e1139_d_n12 + eq78_e1143_d_n12);
        let eq78_e1144_d_n13: f64 = (eq78_e1139_d_n13 + eq78_e1143_d_n13);
        let eq78_e1144_d_n14: f64 = (eq78_e1139_d_n14 + eq78_e1143_d_n14);
        let eq78_e1144_d_n15: f64 = (eq78_e1139_d_n15 + eq78_e1143_d_n15);
        let eq78_e1144_d_n16: f64 = (eq78_e1139_d_n16 + eq78_e1143_d_n16);
        let eq78_e1144_d_n17: f64 = (eq78_e1139_d_n17 + eq78_e1143_d_n17);
        let eq78_e1144_d_n18: f64 = (eq78_e1139_d_n18 + eq78_e1143_d_n18);
        let eq78_e1144_d_n19: f64 = (eq78_e1139_d_n19 + eq78_e1143_d_n19);
        let eq78_e1144_d_n20: f64 = (eq78_e1139_d_n20 + eq78_e1143_d_n20);
        let eq78_e1144_d_n21: f64 = (eq78_e1139_d_n21 + eq78_e1143_d_n21);
        let eq78_e1144_d_n22: f64 = (eq78_e1139_d_n22 + eq78_e1143_d_n22);
        let eq78_e1144_d_n23: f64 = (eq78_e1139_d_n23 + eq78_e1143_d_n23);
        let eq78_e1144_d_n24: f64 = (eq78_e1139_d_n24 + eq78_e1143_d_n24);
        let eq78_e1144_d_n25: f64 = (eq78_e1139_d_n25 + eq78_e1143_d_n25);
        let eq78_e1144_d_n26: f64 = (eq78_e1139_d_n26 + eq78_e1143_d_n26);
        let eq78_e1144_d_n27: f64 = (eq78_e1139_d_n27 + eq78_e1143_d_n27);
        let eq78_e1144_d_n28: f64 = (eq78_e1139_d_n28 + eq78_e1143_d_n28);
        let eq78_e1144_d_n29: f64 = (eq78_e1139_d_n29 + eq78_e1143_d_n29);
        (eq78_e1144, eq78_e1144_d_n0, eq78_e1144_d_n1, eq78_e1144_d_n2, eq78_e1144_d_n3, eq78_e1144_d_n4, eq78_e1144_d_n5, eq78_e1144_d_n6, eq78_e1144_d_n7, eq78_e1144_d_n8, eq78_e1144_d_n9, eq78_e1144_d_n10, eq78_e1144_d_n11, eq78_e1144_d_n12, eq78_e1144_d_n13, eq78_e1144_d_n14, eq78_e1144_d_n15, eq78_e1144_d_n16, eq78_e1144_d_n17, eq78_e1144_d_n18, eq78_e1144_d_n19, eq78_e1144_d_n20, eq78_e1144_d_n21, eq78_e1144_d_n22, eq78_e1144_d_n23, eq78_e1144_d_n24, eq78_e1144_d_n25, eq78_e1144_d_n26, eq78_e1144_d_n27, eq78_e1144_d_n28, eq78_e1144_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq78_value: f64 = eq78_e1146;
        let eq78_node_derivatives: [f64; 30] = [eq78_e1146_d_n0, eq78_e1146_d_n1, eq78_e1146_d_n2, eq78_e1146_d_n3, eq78_e1146_d_n4, eq78_e1146_d_n5, eq78_e1146_d_n6, eq78_e1146_d_n7, eq78_e1146_d_n8, eq78_e1146_d_n9, eq78_e1146_d_n10, eq78_e1146_d_n11, eq78_e1146_d_n12, eq78_e1146_d_n13, eq78_e1146_d_n14, eq78_e1146_d_n15, eq78_e1146_d_n16, eq78_e1146_d_n17, eq78_e1146_d_n18, eq78_e1146_d_n19, eq78_e1146_d_n20, eq78_e1146_d_n21, eq78_e1146_d_n22, eq78_e1146_d_n23, eq78_e1146_d_n24, eq78_e1146_d_n25, eq78_e1146_d_n26, eq78_e1146_d_n27, eq78_e1146_d_n28, eq78_e1146_d_n29];
        let eq78_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[14]),
            self.multiplicity * (eq78_value),
            &nodes,
            &eq78_node_derivatives,
            &branches,
            &eq78_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_79_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq79_e1157, eq79_e1157_d_n0, eq79_e1157_d_n1, eq79_e1157_d_n2, eq79_e1157_d_n3, eq79_e1157_d_n4, eq79_e1157_d_n5, eq79_e1157_d_n6, eq79_e1157_d_n7, eq79_e1157_d_n8, eq79_e1157_d_n9, eq79_e1157_d_n10, eq79_e1157_d_n11, eq79_e1157_d_n12, eq79_e1157_d_n13, eq79_e1157_d_n14, eq79_e1157_d_n15, eq79_e1157_d_n16, eq79_e1157_d_n17, eq79_e1157_d_n18, eq79_e1157_d_n19, eq79_e1157_d_n20, eq79_e1157_d_n21, eq79_e1157_d_n22, eq79_e1157_d_n23, eq79_e1157_d_n24, eq79_e1157_d_n25, eq79_e1157_d_n26, eq79_e1157_d_n27, eq79_e1157_d_n28, eq79_e1157_d_n29,) = {
    if (!(s.v[907] != 0.0)) {
        let eq79_e1150: f64 = self.eval_ddt(64, s.v[193]);
        let eq79_e1150_d_n0: f64 = self.ddt_jacobian(s.dn[193][0]);
        let eq79_e1150_d_n1: f64 = self.ddt_jacobian(s.dn[193][1]);
        let eq79_e1150_d_n2: f64 = self.ddt_jacobian(s.dn[193][2]);
        let eq79_e1150_d_n3: f64 = self.ddt_jacobian(s.dn[193][3]);
        let eq79_e1150_d_n4: f64 = self.ddt_jacobian(s.dn[193][4]);
        let eq79_e1150_d_n5: f64 = self.ddt_jacobian(s.dn[193][5]);
        let eq79_e1150_d_n6: f64 = self.ddt_jacobian(s.dn[193][6]);
        let eq79_e1150_d_n7: f64 = self.ddt_jacobian(s.dn[193][7]);
        let eq79_e1150_d_n8: f64 = self.ddt_jacobian(s.dn[193][8]);
        let eq79_e1150_d_n9: f64 = self.ddt_jacobian(s.dn[193][9]);
        let eq79_e1150_d_n10: f64 = self.ddt_jacobian(s.dn[193][10]);
        let eq79_e1150_d_n11: f64 = self.ddt_jacobian(s.dn[193][11]);
        let eq79_e1150_d_n12: f64 = self.ddt_jacobian(s.dn[193][12]);
        let eq79_e1150_d_n13: f64 = self.ddt_jacobian(s.dn[193][13]);
        let eq79_e1150_d_n14: f64 = self.ddt_jacobian(s.dn[193][14]);
        let eq79_e1150_d_n15: f64 = self.ddt_jacobian(s.dn[193][15]);
        let eq79_e1150_d_n16: f64 = self.ddt_jacobian(s.dn[193][16]);
        let eq79_e1150_d_n17: f64 = self.ddt_jacobian(s.dn[193][17]);
        let eq79_e1150_d_n18: f64 = self.ddt_jacobian(s.dn[193][18]);
        let eq79_e1150_d_n19: f64 = self.ddt_jacobian(s.dn[193][19]);
        let eq79_e1150_d_n20: f64 = self.ddt_jacobian(s.dn[193][20]);
        let eq79_e1150_d_n21: f64 = self.ddt_jacobian(s.dn[193][21]);
        let eq79_e1150_d_n22: f64 = self.ddt_jacobian(s.dn[193][22]);
        let eq79_e1150_d_n23: f64 = self.ddt_jacobian(s.dn[193][23]);
        let eq79_e1150_d_n24: f64 = self.ddt_jacobian(s.dn[193][24]);
        let eq79_e1150_d_n25: f64 = self.ddt_jacobian(s.dn[193][25]);
        let eq79_e1150_d_n26: f64 = self.ddt_jacobian(s.dn[193][26]);
        let eq79_e1150_d_n27: f64 = self.ddt_jacobian(s.dn[193][27]);
        let eq79_e1150_d_n28: f64 = self.ddt_jacobian(s.dn[193][28]);
        let eq79_e1150_d_n29: f64 = self.ddt_jacobian(s.dn[193][29]);
        let eq79_e1153: f64 = (p.p355 * (nv7 - nv5));
        let eq79_e1153_d_n5: f64 = (-p.p355);
        let eq79_e1153_d_n7: f64 = p.p355;
        let eq79_e1154: f64 = self.eval_ddt(65, eq79_e1153);
        let eq79_e1154_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n5: f64 = self.ddt_jacobian(eq79_e1153_d_n5);
        let eq79_e1154_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n7: f64 = self.ddt_jacobian(eq79_e1153_d_n7);
        let eq79_e1154_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq79_e1154_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq79_e1155: f64 = (eq79_e1150 + eq79_e1154);
        let eq79_e1155_d_n0: f64 = (eq79_e1150_d_n0 + eq79_e1154_d_n0);
        let eq79_e1155_d_n1: f64 = (eq79_e1150_d_n1 + eq79_e1154_d_n1);
        let eq79_e1155_d_n2: f64 = (eq79_e1150_d_n2 + eq79_e1154_d_n2);
        let eq79_e1155_d_n3: f64 = (eq79_e1150_d_n3 + eq79_e1154_d_n3);
        let eq79_e1155_d_n4: f64 = (eq79_e1150_d_n4 + eq79_e1154_d_n4);
        let eq79_e1155_d_n5: f64 = (eq79_e1150_d_n5 + eq79_e1154_d_n5);
        let eq79_e1155_d_n6: f64 = (eq79_e1150_d_n6 + eq79_e1154_d_n6);
        let eq79_e1155_d_n7: f64 = (eq79_e1150_d_n7 + eq79_e1154_d_n7);
        let eq79_e1155_d_n8: f64 = (eq79_e1150_d_n8 + eq79_e1154_d_n8);
        let eq79_e1155_d_n9: f64 = (eq79_e1150_d_n9 + eq79_e1154_d_n9);
        let eq79_e1155_d_n10: f64 = (eq79_e1150_d_n10 + eq79_e1154_d_n10);
        let eq79_e1155_d_n11: f64 = (eq79_e1150_d_n11 + eq79_e1154_d_n11);
        let eq79_e1155_d_n12: f64 = (eq79_e1150_d_n12 + eq79_e1154_d_n12);
        let eq79_e1155_d_n13: f64 = (eq79_e1150_d_n13 + eq79_e1154_d_n13);
        let eq79_e1155_d_n14: f64 = (eq79_e1150_d_n14 + eq79_e1154_d_n14);
        let eq79_e1155_d_n15: f64 = (eq79_e1150_d_n15 + eq79_e1154_d_n15);
        let eq79_e1155_d_n16: f64 = (eq79_e1150_d_n16 + eq79_e1154_d_n16);
        let eq79_e1155_d_n17: f64 = (eq79_e1150_d_n17 + eq79_e1154_d_n17);
        let eq79_e1155_d_n18: f64 = (eq79_e1150_d_n18 + eq79_e1154_d_n18);
        let eq79_e1155_d_n19: f64 = (eq79_e1150_d_n19 + eq79_e1154_d_n19);
        let eq79_e1155_d_n20: f64 = (eq79_e1150_d_n20 + eq79_e1154_d_n20);
        let eq79_e1155_d_n21: f64 = (eq79_e1150_d_n21 + eq79_e1154_d_n21);
        let eq79_e1155_d_n22: f64 = (eq79_e1150_d_n22 + eq79_e1154_d_n22);
        let eq79_e1155_d_n23: f64 = (eq79_e1150_d_n23 + eq79_e1154_d_n23);
        let eq79_e1155_d_n24: f64 = (eq79_e1150_d_n24 + eq79_e1154_d_n24);
        let eq79_e1155_d_n25: f64 = (eq79_e1150_d_n25 + eq79_e1154_d_n25);
        let eq79_e1155_d_n26: f64 = (eq79_e1150_d_n26 + eq79_e1154_d_n26);
        let eq79_e1155_d_n27: f64 = (eq79_e1150_d_n27 + eq79_e1154_d_n27);
        let eq79_e1155_d_n28: f64 = (eq79_e1150_d_n28 + eq79_e1154_d_n28);
        let eq79_e1155_d_n29: f64 = (eq79_e1150_d_n29 + eq79_e1154_d_n29);
        (eq79_e1155, eq79_e1155_d_n0, eq79_e1155_d_n1, eq79_e1155_d_n2, eq79_e1155_d_n3, eq79_e1155_d_n4, eq79_e1155_d_n5, eq79_e1155_d_n6, eq79_e1155_d_n7, eq79_e1155_d_n8, eq79_e1155_d_n9, eq79_e1155_d_n10, eq79_e1155_d_n11, eq79_e1155_d_n12, eq79_e1155_d_n13, eq79_e1155_d_n14, eq79_e1155_d_n15, eq79_e1155_d_n16, eq79_e1155_d_n17, eq79_e1155_d_n18, eq79_e1155_d_n19, eq79_e1155_d_n20, eq79_e1155_d_n21, eq79_e1155_d_n22, eq79_e1155_d_n23, eq79_e1155_d_n24, eq79_e1155_d_n25, eq79_e1155_d_n26, eq79_e1155_d_n27, eq79_e1155_d_n28, eq79_e1155_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq79_value: f64 = eq79_e1157;
        let eq79_node_derivatives: [f64; 30] = [eq79_e1157_d_n0, eq79_e1157_d_n1, eq79_e1157_d_n2, eq79_e1157_d_n3, eq79_e1157_d_n4, eq79_e1157_d_n5, eq79_e1157_d_n6, eq79_e1157_d_n7, eq79_e1157_d_n8, eq79_e1157_d_n9, eq79_e1157_d_n10, eq79_e1157_d_n11, eq79_e1157_d_n12, eq79_e1157_d_n13, eq79_e1157_d_n14, eq79_e1157_d_n15, eq79_e1157_d_n16, eq79_e1157_d_n17, eq79_e1157_d_n18, eq79_e1157_d_n19, eq79_e1157_d_n20, eq79_e1157_d_n21, eq79_e1157_d_n22, eq79_e1157_d_n23, eq79_e1157_d_n24, eq79_e1157_d_n25, eq79_e1157_d_n26, eq79_e1157_d_n27, eq79_e1157_d_n28, eq79_e1157_d_n29];
        let eq79_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            self.multiplicity * (eq79_value),
            &nodes,
            &eq79_node_derivatives,
            &branches,
            &eq79_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_80_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq80_e1162,) = {
    if (!(s.v[907] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq80_value: f64 = eq80_e1162;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[14]),
            self.multiplicity * (eq80_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_81_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq81_e1167,) = {
    if (!(s.v[907] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq81_value: f64 = eq81_e1167;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[9]),
            self.multiplicity * (eq81_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_82_block_0(
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq82_e1169: f64 = self.eval_ddt(66, s.v[194]);
        let eq82_e1169_d_n0: f64 = self.ddt_jacobian(s.dn[194][0]);
        let eq82_e1169_d_n1: f64 = self.ddt_jacobian(s.dn[194][1]);
        let eq82_e1169_d_n2: f64 = self.ddt_jacobian(s.dn[194][2]);
        let eq82_e1169_d_n3: f64 = self.ddt_jacobian(s.dn[194][3]);
        let eq82_e1169_d_n4: f64 = self.ddt_jacobian(s.dn[194][4]);
        let eq82_e1169_d_n5: f64 = self.ddt_jacobian(s.dn[194][5]);
        let eq82_e1169_d_n6: f64 = self.ddt_jacobian(s.dn[194][6]);
        let eq82_e1169_d_n7: f64 = self.ddt_jacobian(s.dn[194][7]);
        let eq82_e1169_d_n8: f64 = self.ddt_jacobian(s.dn[194][8]);
        let eq82_e1169_d_n9: f64 = self.ddt_jacobian(s.dn[194][9]);
        let eq82_e1169_d_n10: f64 = self.ddt_jacobian(s.dn[194][10]);
        let eq82_e1169_d_n11: f64 = self.ddt_jacobian(s.dn[194][11]);
        let eq82_e1169_d_n12: f64 = self.ddt_jacobian(s.dn[194][12]);
        let eq82_e1169_d_n13: f64 = self.ddt_jacobian(s.dn[194][13]);
        let eq82_e1169_d_n14: f64 = self.ddt_jacobian(s.dn[194][14]);
        let eq82_e1169_d_n15: f64 = self.ddt_jacobian(s.dn[194][15]);
        let eq82_e1169_d_n16: f64 = self.ddt_jacobian(s.dn[194][16]);
        let eq82_e1169_d_n17: f64 = self.ddt_jacobian(s.dn[194][17]);
        let eq82_e1169_d_n18: f64 = self.ddt_jacobian(s.dn[194][18]);
        let eq82_e1169_d_n19: f64 = self.ddt_jacobian(s.dn[194][19]);
        let eq82_e1169_d_n20: f64 = self.ddt_jacobian(s.dn[194][20]);
        let eq82_e1169_d_n21: f64 = self.ddt_jacobian(s.dn[194][21]);
        let eq82_e1169_d_n22: f64 = self.ddt_jacobian(s.dn[194][22]);
        let eq82_e1169_d_n23: f64 = self.ddt_jacobian(s.dn[194][23]);
        let eq82_e1169_d_n24: f64 = self.ddt_jacobian(s.dn[194][24]);
        let eq82_e1169_d_n25: f64 = self.ddt_jacobian(s.dn[194][25]);
        let eq82_e1169_d_n26: f64 = self.ddt_jacobian(s.dn[194][26]);
        let eq82_e1169_d_n27: f64 = self.ddt_jacobian(s.dn[194][27]);
        let eq82_e1169_d_n28: f64 = self.ddt_jacobian(s.dn[194][28]);
        let eq82_e1169_d_n29: f64 = self.ddt_jacobian(s.dn[194][29]);
        let eq82_e1172: f64 = (p.p355 * (nv3 - nv5));
        let eq82_e1172_d_n3: f64 = p.p355;
        let eq82_e1172_d_n5: f64 = (-p.p355);
        let eq82_e1173: f64 = self.eval_ddt(67, eq82_e1172);
        let eq82_e1173_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n3: f64 = self.ddt_jacobian(eq82_e1172_d_n3);
        let eq82_e1173_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n5: f64 = self.ddt_jacobian(eq82_e1172_d_n5);
        let eq82_e1173_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq82_e1173_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq82_e1174: f64 = (eq82_e1169 + eq82_e1173);
        let eq82_e1174_d_n0: f64 = (eq82_e1169_d_n0 + eq82_e1173_d_n0);
        let eq82_e1174_d_n1: f64 = (eq82_e1169_d_n1 + eq82_e1173_d_n1);
        let eq82_e1174_d_n2: f64 = (eq82_e1169_d_n2 + eq82_e1173_d_n2);
        let eq82_e1174_d_n3: f64 = (eq82_e1169_d_n3 + eq82_e1173_d_n3);
        let eq82_e1174_d_n4: f64 = (eq82_e1169_d_n4 + eq82_e1173_d_n4);
        let eq82_e1174_d_n5: f64 = (eq82_e1169_d_n5 + eq82_e1173_d_n5);
        let eq82_e1174_d_n6: f64 = (eq82_e1169_d_n6 + eq82_e1173_d_n6);
        let eq82_e1174_d_n7: f64 = (eq82_e1169_d_n7 + eq82_e1173_d_n7);
        let eq82_e1174_d_n8: f64 = (eq82_e1169_d_n8 + eq82_e1173_d_n8);
        let eq82_e1174_d_n9: f64 = (eq82_e1169_d_n9 + eq82_e1173_d_n9);
        let eq82_e1174_d_n10: f64 = (eq82_e1169_d_n10 + eq82_e1173_d_n10);
        let eq82_e1174_d_n11: f64 = (eq82_e1169_d_n11 + eq82_e1173_d_n11);
        let eq82_e1174_d_n12: f64 = (eq82_e1169_d_n12 + eq82_e1173_d_n12);
        let eq82_e1174_d_n13: f64 = (eq82_e1169_d_n13 + eq82_e1173_d_n13);
        let eq82_e1174_d_n14: f64 = (eq82_e1169_d_n14 + eq82_e1173_d_n14);
        let eq82_e1174_d_n15: f64 = (eq82_e1169_d_n15 + eq82_e1173_d_n15);
        let eq82_e1174_d_n16: f64 = (eq82_e1169_d_n16 + eq82_e1173_d_n16);
        let eq82_e1174_d_n17: f64 = (eq82_e1169_d_n17 + eq82_e1173_d_n17);
        let eq82_e1174_d_n18: f64 = (eq82_e1169_d_n18 + eq82_e1173_d_n18);
        let eq82_e1174_d_n19: f64 = (eq82_e1169_d_n19 + eq82_e1173_d_n19);
        let eq82_e1174_d_n20: f64 = (eq82_e1169_d_n20 + eq82_e1173_d_n20);
        let eq82_e1174_d_n21: f64 = (eq82_e1169_d_n21 + eq82_e1173_d_n21);
        let eq82_e1174_d_n22: f64 = (eq82_e1169_d_n22 + eq82_e1173_d_n22);
        let eq82_e1174_d_n23: f64 = (eq82_e1169_d_n23 + eq82_e1173_d_n23);
        let eq82_e1174_d_n24: f64 = (eq82_e1169_d_n24 + eq82_e1173_d_n24);
        let eq82_e1174_d_n25: f64 = (eq82_e1169_d_n25 + eq82_e1173_d_n25);
        let eq82_e1174_d_n26: f64 = (eq82_e1169_d_n26 + eq82_e1173_d_n26);
        let eq82_e1174_d_n27: f64 = (eq82_e1169_d_n27 + eq82_e1173_d_n27);
        let eq82_e1174_d_n28: f64 = (eq82_e1169_d_n28 + eq82_e1173_d_n28);
        let eq82_e1174_d_n29: f64 = (eq82_e1169_d_n29 + eq82_e1173_d_n29);
        let eq82_value: f64 = eq82_e1174;
        let eq82_node_derivatives: [f64; 30] = [eq82_e1174_d_n0, eq82_e1174_d_n1, eq82_e1174_d_n2, eq82_e1174_d_n3, eq82_e1174_d_n4, eq82_e1174_d_n5, eq82_e1174_d_n6, eq82_e1174_d_n7, eq82_e1174_d_n8, eq82_e1174_d_n9, eq82_e1174_d_n10, eq82_e1174_d_n11, eq82_e1174_d_n12, eq82_e1174_d_n13, eq82_e1174_d_n14, eq82_e1174_d_n15, eq82_e1174_d_n16, eq82_e1174_d_n17, eq82_e1174_d_n18, eq82_e1174_d_n19, eq82_e1174_d_n20, eq82_e1174_d_n21, eq82_e1174_d_n22, eq82_e1174_d_n23, eq82_e1174_d_n24, eq82_e1174_d_n25, eq82_e1174_d_n26, eq82_e1174_d_n27, eq82_e1174_d_n28, eq82_e1174_d_n29];
        let eq82_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            self.multiplicity * (eq82_value),
            &nodes,
            &eq82_node_derivatives,
            &branches,
            &eq82_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_83_block_0(
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq83_e1182, eq83_e1182_d_n0, eq83_e1182_d_n1, eq83_e1182_d_n2, eq83_e1182_d_n3, eq83_e1182_d_n4, eq83_e1182_d_n5, eq83_e1182_d_n6, eq83_e1182_d_n7, eq83_e1182_d_n8, eq83_e1182_d_n9, eq83_e1182_d_n10, eq83_e1182_d_n11, eq83_e1182_d_n12, eq83_e1182_d_n13, eq83_e1182_d_n14, eq83_e1182_d_n15, eq83_e1182_d_n16, eq83_e1182_d_n17, eq83_e1182_d_n18, eq83_e1182_d_n19, eq83_e1182_d_n20, eq83_e1182_d_n21, eq83_e1182_d_n22, eq83_e1182_d_n23, eq83_e1182_d_n24, eq83_e1182_d_n25, eq83_e1182_d_n26, eq83_e1182_d_n27, eq83_e1182_d_n28, eq83_e1182_d_n29,) = {
    if (s.v[908] != 0.0) {
        let eq83_e1179: f64 = (s.v[0] * (nv9 - nv10));
        let eq83_e1179_d_n9: f64 = s.v[0];
        let eq83_e1179_d_n10: f64 = (-s.v[0]);
        let eq83_e1180: f64 = (s.v[166] + eq83_e1179);
        let eq83_e1180_d_n9: f64 = (s.dn[166][9] + eq83_e1179_d_n9);
        let eq83_e1180_d_n10: f64 = (s.dn[166][10] + eq83_e1179_d_n10);
        (eq83_e1180, s.dn[166][0], s.dn[166][1], s.dn[166][2], s.dn[166][3], s.dn[166][4], s.dn[166][5], s.dn[166][6], s.dn[166][7], s.dn[166][8], eq83_e1180_d_n9, eq83_e1180_d_n10, s.dn[166][11], s.dn[166][12], s.dn[166][13], s.dn[166][14], s.dn[166][15], s.dn[166][16], s.dn[166][17], s.dn[166][18], s.dn[166][19], s.dn[166][20], s.dn[166][21], s.dn[166][22], s.dn[166][23], s.dn[166][24], s.dn[166][25], s.dn[166][26], s.dn[166][27], s.dn[166][28], s.dn[166][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq83_value: f64 = eq83_e1182;
        let eq83_node_derivatives: [f64; 30] = [eq83_e1182_d_n0, eq83_e1182_d_n1, eq83_e1182_d_n2, eq83_e1182_d_n3, eq83_e1182_d_n4, eq83_e1182_d_n5, eq83_e1182_d_n6, eq83_e1182_d_n7, eq83_e1182_d_n8, eq83_e1182_d_n9, eq83_e1182_d_n10, eq83_e1182_d_n11, eq83_e1182_d_n12, eq83_e1182_d_n13, eq83_e1182_d_n14, eq83_e1182_d_n15, eq83_e1182_d_n16, eq83_e1182_d_n17, eq83_e1182_d_n18, eq83_e1182_d_n19, eq83_e1182_d_n20, eq83_e1182_d_n21, eq83_e1182_d_n22, eq83_e1182_d_n23, eq83_e1182_d_n24, eq83_e1182_d_n25, eq83_e1182_d_n26, eq83_e1182_d_n27, eq83_e1182_d_n28, eq83_e1182_d_n29];
        let eq83_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[10]),
            self.multiplicity * (eq83_value),
            &nodes,
            &eq83_node_derivatives,
            &branches,
            &eq83_branch_derivatives,
            self.multiplicity,
        );
    }
}
