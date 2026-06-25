#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_74_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq74_e1110, eq74_e1110_d_n0, eq74_e1110_d_n1, eq74_e1110_d_n2, eq74_e1110_d_n3, eq74_e1110_d_n4, eq74_e1110_d_n5, eq74_e1110_d_n6, eq74_e1110_d_n7, eq74_e1110_d_n8, eq74_e1110_d_n9, eq74_e1110_d_n10, eq74_e1110_d_n11, eq74_e1110_d_n12, eq74_e1110_d_n13, eq74_e1110_d_n14, eq74_e1110_d_n15, eq74_e1110_d_n16, eq74_e1110_d_n17, eq74_e1110_d_n18, eq74_e1110_d_n19, eq74_e1110_d_n20, eq74_e1110_d_n21, eq74_e1110_d_n22, eq74_e1110_d_n23, eq74_e1110_d_n24, eq74_e1110_d_n25, eq74_e1110_d_n26, eq74_e1110_d_n27, eq74_e1110_d_n28, eq74_e1110_d_n29, eq74_e1110_q, eq74_e1110_q_d_n0, eq74_e1110_q_d_n1, eq74_e1110_q_d_n2, eq74_e1110_q_d_n3, eq74_e1110_q_d_n4, eq74_e1110_q_d_n5, eq74_e1110_q_d_n6, eq74_e1110_q_d_n7, eq74_e1110_q_d_n8, eq74_e1110_q_d_n9, eq74_e1110_q_d_n10, eq74_e1110_q_d_n11, eq74_e1110_q_d_n12, eq74_e1110_q_d_n13, eq74_e1110_q_d_n14, eq74_e1110_q_d_n15, eq74_e1110_q_d_n16, eq74_e1110_q_d_n17, eq74_e1110_q_d_n18, eq74_e1110_q_d_n19, eq74_e1110_q_d_n20, eq74_e1110_q_d_n21, eq74_e1110_q_d_n22, eq74_e1110_q_d_n23, eq74_e1110_q_d_n24, eq74_e1110_q_d_n25, eq74_e1110_q_d_n26, eq74_e1110_q_d_n27, eq74_e1110_q_d_n28, eq74_e1110_q_d_n29,) = {
    if (s.v[907] != 0.0) {
        let eq74_e1103_q: f64 = s.v[193];
        let eq74_e1106: f64 = (p.p355 * (nv2 - nv5));
        let eq74_e1106_d_n2: f64 = p.p355;
        let eq74_e1106_d_n5: f64 = (-p.p355);
        let eq74_e1107_q: f64 = eq74_e1106;
        let eq74_e1108: f64 = (s.v[193] + eq74_e1106);
        let eq74_e1108_d_n2: f64 = (s.dn[193][2] + eq74_e1106_d_n2);
        let eq74_e1108_d_n5: f64 = (s.dn[193][5] + eq74_e1106_d_n5);
        let eq74_e1108_q: f64 = (eq74_e1103_q + eq74_e1107_q);
        let eq74_e1108_q_d_n2: f64 = (s.dn[193][2] + eq74_e1106_d_n2);
        let eq74_e1108_q_d_n5: f64 = (s.dn[193][5] + eq74_e1106_d_n5);
        (eq74_e1108, s.dn[193][0], s.dn[193][1], eq74_e1108_d_n2, s.dn[193][3], s.dn[193][4], eq74_e1108_d_n5, s.dn[193][6], s.dn[193][7], s.dn[193][8], s.dn[193][9], s.dn[193][10], s.dn[193][11], s.dn[193][12], s.dn[193][13], s.dn[193][14], s.dn[193][15], s.dn[193][16], s.dn[193][17], s.dn[193][18], s.dn[193][19], s.dn[193][20], s.dn[193][21], s.dn[193][22], s.dn[193][23], s.dn[193][24], s.dn[193][25], s.dn[193][26], s.dn[193][27], s.dn[193][28], s.dn[193][29], eq74_e1108_q, s.dn[193][0], s.dn[193][1], eq74_e1108_q_d_n2, s.dn[193][3], s.dn[193][4], eq74_e1108_q_d_n5, s.dn[193][6], s.dn[193][7], s.dn[193][8], s.dn[193][9], s.dn[193][10], s.dn[193][11], s.dn[193][12], s.dn[193][13], s.dn[193][14], s.dn[193][15], s.dn[193][16], s.dn[193][17], s.dn[193][18], s.dn[193][19], s.dn[193][20], s.dn[193][21], s.dn[193][22], s.dn[193][23], s.dn[193][24], s.dn[193][25], s.dn[193][26], s.dn[193][27], s.dn[193][28], s.dn[193][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq74_reactive_node_derivatives: [f64; 30] = [eq74_e1110_q_d_n0, eq74_e1110_q_d_n1, eq74_e1110_q_d_n2, eq74_e1110_q_d_n3, eq74_e1110_q_d_n4, eq74_e1110_q_d_n5, eq74_e1110_q_d_n6, eq74_e1110_q_d_n7, eq74_e1110_q_d_n8, eq74_e1110_q_d_n9, eq74_e1110_q_d_n10, eq74_e1110_q_d_n11, eq74_e1110_q_d_n12, eq74_e1110_q_d_n13, eq74_e1110_q_d_n14, eq74_e1110_q_d_n15, eq74_e1110_q_d_n16, eq74_e1110_q_d_n17, eq74_e1110_q_d_n18, eq74_e1110_q_d_n19, eq74_e1110_q_d_n20, eq74_e1110_q_d_n21, eq74_e1110_q_d_n22, eq74_e1110_q_d_n23, eq74_e1110_q_d_n24, eq74_e1110_q_d_n25, eq74_e1110_q_d_n26, eq74_e1110_q_d_n27, eq74_e1110_q_d_n28, eq74_e1110_q_d_n29];
        let eq74_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[5]),
            &nodes,
            &eq74_reactive_node_derivatives,
            &branches,
            &eq74_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_76_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq76_e1124, eq76_e1124_d_n0, eq76_e1124_d_n1, eq76_e1124_d_n2, eq76_e1124_d_n3, eq76_e1124_d_n4, eq76_e1124_d_n5, eq76_e1124_d_n6, eq76_e1124_d_n7, eq76_e1124_d_n8, eq76_e1124_d_n9, eq76_e1124_d_n10, eq76_e1124_d_n11, eq76_e1124_d_n12, eq76_e1124_d_n13, eq76_e1124_d_n14, eq76_e1124_d_n15, eq76_e1124_d_n16, eq76_e1124_d_n17, eq76_e1124_d_n18, eq76_e1124_d_n19, eq76_e1124_d_n20, eq76_e1124_d_n21, eq76_e1124_d_n22, eq76_e1124_d_n23, eq76_e1124_d_n24, eq76_e1124_d_n25, eq76_e1124_d_n26, eq76_e1124_d_n27, eq76_e1124_d_n28, eq76_e1124_d_n29, eq76_e1124_q, eq76_e1124_q_d_n0, eq76_e1124_q_d_n1, eq76_e1124_q_d_n2, eq76_e1124_q_d_n3, eq76_e1124_q_d_n4, eq76_e1124_q_d_n5, eq76_e1124_q_d_n6, eq76_e1124_q_d_n7, eq76_e1124_q_d_n8, eq76_e1124_q_d_n9, eq76_e1124_q_d_n10, eq76_e1124_q_d_n11, eq76_e1124_q_d_n12, eq76_e1124_q_d_n13, eq76_e1124_q_d_n14, eq76_e1124_q_d_n15, eq76_e1124_q_d_n16, eq76_e1124_q_d_n17, eq76_e1124_q_d_n18, eq76_e1124_q_d_n19, eq76_e1124_q_d_n20, eq76_e1124_q_d_n21, eq76_e1124_q_d_n22, eq76_e1124_q_d_n23, eq76_e1124_q_d_n24, eq76_e1124_q_d_n25, eq76_e1124_q_d_n26, eq76_e1124_q_d_n27, eq76_e1124_q_d_n28, eq76_e1124_q_d_n29,) = {
    if (s.v[907] != 0.0) {
        let eq76_e1117_q: f64 = s.v[195];
        let eq76_e1120: f64 = (p.p355 * (nv7 - nv9));
        let eq76_e1120_d_n7: f64 = p.p355;
        let eq76_e1120_d_n9: f64 = (-p.p355);
        let eq76_e1121_q: f64 = eq76_e1120;
        let eq76_e1122: f64 = (s.v[195] + eq76_e1120);
        let eq76_e1122_d_n7: f64 = (s.dn[195][7] + eq76_e1120_d_n7);
        let eq76_e1122_d_n9: f64 = (s.dn[195][9] + eq76_e1120_d_n9);
        let eq76_e1122_q: f64 = (eq76_e1117_q + eq76_e1121_q);
        let eq76_e1122_q_d_n7: f64 = (s.dn[195][7] + eq76_e1120_d_n7);
        let eq76_e1122_q_d_n9: f64 = (s.dn[195][9] + eq76_e1120_d_n9);
        (eq76_e1122, s.dn[195][0], s.dn[195][1], s.dn[195][2], s.dn[195][3], s.dn[195][4], s.dn[195][5], s.dn[195][6], eq76_e1122_d_n7, s.dn[195][8], eq76_e1122_d_n9, s.dn[195][10], s.dn[195][11], s.dn[195][12], s.dn[195][13], s.dn[195][14], s.dn[195][15], s.dn[195][16], s.dn[195][17], s.dn[195][18], s.dn[195][19], s.dn[195][20], s.dn[195][21], s.dn[195][22], s.dn[195][23], s.dn[195][24], s.dn[195][25], s.dn[195][26], s.dn[195][27], s.dn[195][28], s.dn[195][29], eq76_e1122_q, s.dn[195][0], s.dn[195][1], s.dn[195][2], s.dn[195][3], s.dn[195][4], s.dn[195][5], s.dn[195][6], eq76_e1122_q_d_n7, s.dn[195][8], eq76_e1122_q_d_n9, s.dn[195][10], s.dn[195][11], s.dn[195][12], s.dn[195][13], s.dn[195][14], s.dn[195][15], s.dn[195][16], s.dn[195][17], s.dn[195][18], s.dn[195][19], s.dn[195][20], s.dn[195][21], s.dn[195][22], s.dn[195][23], s.dn[195][24], s.dn[195][25], s.dn[195][26], s.dn[195][27], s.dn[195][28], s.dn[195][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_reactive_node_derivatives: [f64; 30] = [eq76_e1124_q_d_n0, eq76_e1124_q_d_n1, eq76_e1124_q_d_n2, eq76_e1124_q_d_n3, eq76_e1124_q_d_n4, eq76_e1124_q_d_n5, eq76_e1124_q_d_n6, eq76_e1124_q_d_n7, eq76_e1124_q_d_n8, eq76_e1124_q_d_n9, eq76_e1124_q_d_n10, eq76_e1124_q_d_n11, eq76_e1124_q_d_n12, eq76_e1124_q_d_n13, eq76_e1124_q_d_n14, eq76_e1124_q_d_n15, eq76_e1124_q_d_n16, eq76_e1124_q_d_n17, eq76_e1124_q_d_n18, eq76_e1124_q_d_n19, eq76_e1124_q_d_n20, eq76_e1124_q_d_n21, eq76_e1124_q_d_n22, eq76_e1124_q_d_n23, eq76_e1124_q_d_n24, eq76_e1124_q_d_n25, eq76_e1124_q_d_n26, eq76_e1124_q_d_n27, eq76_e1124_q_d_n28, eq76_e1124_q_d_n29];
        let eq76_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            &nodes,
            &eq76_reactive_node_derivatives,
            &branches,
            &eq76_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_77_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq77_e1135, eq77_e1135_d_n0, eq77_e1135_d_n1, eq77_e1135_d_n2, eq77_e1135_d_n3, eq77_e1135_d_n4, eq77_e1135_d_n5, eq77_e1135_d_n6, eq77_e1135_d_n7, eq77_e1135_d_n8, eq77_e1135_d_n9, eq77_e1135_d_n10, eq77_e1135_d_n11, eq77_e1135_d_n12, eq77_e1135_d_n13, eq77_e1135_d_n14, eq77_e1135_d_n15, eq77_e1135_d_n16, eq77_e1135_d_n17, eq77_e1135_d_n18, eq77_e1135_d_n19, eq77_e1135_d_n20, eq77_e1135_d_n21, eq77_e1135_d_n22, eq77_e1135_d_n23, eq77_e1135_d_n24, eq77_e1135_d_n25, eq77_e1135_d_n26, eq77_e1135_d_n27, eq77_e1135_d_n28, eq77_e1135_d_n29, eq77_e1135_q, eq77_e1135_q_d_n0, eq77_e1135_q_d_n1, eq77_e1135_q_d_n2, eq77_e1135_q_d_n3, eq77_e1135_q_d_n4, eq77_e1135_q_d_n5, eq77_e1135_q_d_n6, eq77_e1135_q_d_n7, eq77_e1135_q_d_n8, eq77_e1135_q_d_n9, eq77_e1135_q_d_n10, eq77_e1135_q_d_n11, eq77_e1135_q_d_n12, eq77_e1135_q_d_n13, eq77_e1135_q_d_n14, eq77_e1135_q_d_n15, eq77_e1135_q_d_n16, eq77_e1135_q_d_n17, eq77_e1135_q_d_n18, eq77_e1135_q_d_n19, eq77_e1135_q_d_n20, eq77_e1135_q_d_n21, eq77_e1135_q_d_n22, eq77_e1135_q_d_n23, eq77_e1135_q_d_n24, eq77_e1135_q_d_n25, eq77_e1135_q_d_n26, eq77_e1135_q_d_n27, eq77_e1135_q_d_n28, eq77_e1135_q_d_n29,) = {
    if (!(s.v[907] != 0.0)) {
        let eq77_e1128_q: f64 = s.v[191];
        let eq77_e1131: f64 = (p.p355 * (nv2 - nv5));
        let eq77_e1131_d_n2: f64 = p.p355;
        let eq77_e1131_d_n5: f64 = (-p.p355);
        let eq77_e1132_q: f64 = eq77_e1131;
        let eq77_e1133: f64 = (s.v[191] + eq77_e1131);
        let eq77_e1133_d_n2: f64 = (s.dn[191][2] + eq77_e1131_d_n2);
        let eq77_e1133_d_n5: f64 = (s.dn[191][5] + eq77_e1131_d_n5);
        let eq77_e1133_q: f64 = (eq77_e1128_q + eq77_e1132_q);
        let eq77_e1133_q_d_n2: f64 = (s.dn[191][2] + eq77_e1131_d_n2);
        let eq77_e1133_q_d_n5: f64 = (s.dn[191][5] + eq77_e1131_d_n5);
        (eq77_e1133, s.dn[191][0], s.dn[191][1], eq77_e1133_d_n2, s.dn[191][3], s.dn[191][4], eq77_e1133_d_n5, s.dn[191][6], s.dn[191][7], s.dn[191][8], s.dn[191][9], s.dn[191][10], s.dn[191][11], s.dn[191][12], s.dn[191][13], s.dn[191][14], s.dn[191][15], s.dn[191][16], s.dn[191][17], s.dn[191][18], s.dn[191][19], s.dn[191][20], s.dn[191][21], s.dn[191][22], s.dn[191][23], s.dn[191][24], s.dn[191][25], s.dn[191][26], s.dn[191][27], s.dn[191][28], s.dn[191][29], eq77_e1133_q, s.dn[191][0], s.dn[191][1], eq77_e1133_q_d_n2, s.dn[191][3], s.dn[191][4], eq77_e1133_q_d_n5, s.dn[191][6], s.dn[191][7], s.dn[191][8], s.dn[191][9], s.dn[191][10], s.dn[191][11], s.dn[191][12], s.dn[191][13], s.dn[191][14], s.dn[191][15], s.dn[191][16], s.dn[191][17], s.dn[191][18], s.dn[191][19], s.dn[191][20], s.dn[191][21], s.dn[191][22], s.dn[191][23], s.dn[191][24], s.dn[191][25], s.dn[191][26], s.dn[191][27], s.dn[191][28], s.dn[191][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_reactive_node_derivatives: [f64; 30] = [eq77_e1135_q_d_n0, eq77_e1135_q_d_n1, eq77_e1135_q_d_n2, eq77_e1135_q_d_n3, eq77_e1135_q_d_n4, eq77_e1135_q_d_n5, eq77_e1135_q_d_n6, eq77_e1135_q_d_n7, eq77_e1135_q_d_n8, eq77_e1135_q_d_n9, eq77_e1135_q_d_n10, eq77_e1135_q_d_n11, eq77_e1135_q_d_n12, eq77_e1135_q_d_n13, eq77_e1135_q_d_n14, eq77_e1135_q_d_n15, eq77_e1135_q_d_n16, eq77_e1135_q_d_n17, eq77_e1135_q_d_n18, eq77_e1135_q_d_n19, eq77_e1135_q_d_n20, eq77_e1135_q_d_n21, eq77_e1135_q_d_n22, eq77_e1135_q_d_n23, eq77_e1135_q_d_n24, eq77_e1135_q_d_n25, eq77_e1135_q_d_n26, eq77_e1135_q_d_n27, eq77_e1135_q_d_n28, eq77_e1135_q_d_n29];
        let eq77_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[5]),
            &nodes,
            &eq77_reactive_node_derivatives,
            &branches,
            &eq77_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_78_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq78_e1146, eq78_e1146_d_n0, eq78_e1146_d_n1, eq78_e1146_d_n2, eq78_e1146_d_n3, eq78_e1146_d_n4, eq78_e1146_d_n5, eq78_e1146_d_n6, eq78_e1146_d_n7, eq78_e1146_d_n8, eq78_e1146_d_n9, eq78_e1146_d_n10, eq78_e1146_d_n11, eq78_e1146_d_n12, eq78_e1146_d_n13, eq78_e1146_d_n14, eq78_e1146_d_n15, eq78_e1146_d_n16, eq78_e1146_d_n17, eq78_e1146_d_n18, eq78_e1146_d_n19, eq78_e1146_d_n20, eq78_e1146_d_n21, eq78_e1146_d_n22, eq78_e1146_d_n23, eq78_e1146_d_n24, eq78_e1146_d_n25, eq78_e1146_d_n26, eq78_e1146_d_n27, eq78_e1146_d_n28, eq78_e1146_d_n29, eq78_e1146_q, eq78_e1146_q_d_n0, eq78_e1146_q_d_n1, eq78_e1146_q_d_n2, eq78_e1146_q_d_n3, eq78_e1146_q_d_n4, eq78_e1146_q_d_n5, eq78_e1146_q_d_n6, eq78_e1146_q_d_n7, eq78_e1146_q_d_n8, eq78_e1146_q_d_n9, eq78_e1146_q_d_n10, eq78_e1146_q_d_n11, eq78_e1146_q_d_n12, eq78_e1146_q_d_n13, eq78_e1146_q_d_n14, eq78_e1146_q_d_n15, eq78_e1146_q_d_n16, eq78_e1146_q_d_n17, eq78_e1146_q_d_n18, eq78_e1146_q_d_n19, eq78_e1146_q_d_n20, eq78_e1146_q_d_n21, eq78_e1146_q_d_n22, eq78_e1146_q_d_n23, eq78_e1146_q_d_n24, eq78_e1146_q_d_n25, eq78_e1146_q_d_n26, eq78_e1146_q_d_n27, eq78_e1146_q_d_n28, eq78_e1146_q_d_n29,) = {
    if (!(s.v[907] != 0.0)) {
        let eq78_e1139_q: f64 = s.v[192];
        let eq78_e1142: f64 = (p.p355 * (nv2 - nv14));
        let eq78_e1142_d_n2: f64 = p.p355;
        let eq78_e1142_d_n14: f64 = (-p.p355);
        let eq78_e1143_q: f64 = eq78_e1142;
        let eq78_e1144: f64 = (s.v[192] + eq78_e1142);
        let eq78_e1144_d_n2: f64 = (s.dn[192][2] + eq78_e1142_d_n2);
        let eq78_e1144_d_n14: f64 = (s.dn[192][14] + eq78_e1142_d_n14);
        let eq78_e1144_q: f64 = (eq78_e1139_q + eq78_e1143_q);
        let eq78_e1144_q_d_n2: f64 = (s.dn[192][2] + eq78_e1142_d_n2);
        let eq78_e1144_q_d_n14: f64 = (s.dn[192][14] + eq78_e1142_d_n14);
        (eq78_e1144, s.dn[192][0], s.dn[192][1], eq78_e1144_d_n2, s.dn[192][3], s.dn[192][4], s.dn[192][5], s.dn[192][6], s.dn[192][7], s.dn[192][8], s.dn[192][9], s.dn[192][10], s.dn[192][11], s.dn[192][12], s.dn[192][13], eq78_e1144_d_n14, s.dn[192][15], s.dn[192][16], s.dn[192][17], s.dn[192][18], s.dn[192][19], s.dn[192][20], s.dn[192][21], s.dn[192][22], s.dn[192][23], s.dn[192][24], s.dn[192][25], s.dn[192][26], s.dn[192][27], s.dn[192][28], s.dn[192][29], eq78_e1144_q, s.dn[192][0], s.dn[192][1], eq78_e1144_q_d_n2, s.dn[192][3], s.dn[192][4], s.dn[192][5], s.dn[192][6], s.dn[192][7], s.dn[192][8], s.dn[192][9], s.dn[192][10], s.dn[192][11], s.dn[192][12], s.dn[192][13], eq78_e1144_q_d_n14, s.dn[192][15], s.dn[192][16], s.dn[192][17], s.dn[192][18], s.dn[192][19], s.dn[192][20], s.dn[192][21], s.dn[192][22], s.dn[192][23], s.dn[192][24], s.dn[192][25], s.dn[192][26], s.dn[192][27], s.dn[192][28], s.dn[192][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq78_reactive_node_derivatives: [f64; 30] = [eq78_e1146_q_d_n0, eq78_e1146_q_d_n1, eq78_e1146_q_d_n2, eq78_e1146_q_d_n3, eq78_e1146_q_d_n4, eq78_e1146_q_d_n5, eq78_e1146_q_d_n6, eq78_e1146_q_d_n7, eq78_e1146_q_d_n8, eq78_e1146_q_d_n9, eq78_e1146_q_d_n10, eq78_e1146_q_d_n11, eq78_e1146_q_d_n12, eq78_e1146_q_d_n13, eq78_e1146_q_d_n14, eq78_e1146_q_d_n15, eq78_e1146_q_d_n16, eq78_e1146_q_d_n17, eq78_e1146_q_d_n18, eq78_e1146_q_d_n19, eq78_e1146_q_d_n20, eq78_e1146_q_d_n21, eq78_e1146_q_d_n22, eq78_e1146_q_d_n23, eq78_e1146_q_d_n24, eq78_e1146_q_d_n25, eq78_e1146_q_d_n26, eq78_e1146_q_d_n27, eq78_e1146_q_d_n28, eq78_e1146_q_d_n29];
        let eq78_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[14]),
            &nodes,
            &eq78_reactive_node_derivatives,
            &branches,
            &eq78_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_79_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq79_e1157, eq79_e1157_d_n0, eq79_e1157_d_n1, eq79_e1157_d_n2, eq79_e1157_d_n3, eq79_e1157_d_n4, eq79_e1157_d_n5, eq79_e1157_d_n6, eq79_e1157_d_n7, eq79_e1157_d_n8, eq79_e1157_d_n9, eq79_e1157_d_n10, eq79_e1157_d_n11, eq79_e1157_d_n12, eq79_e1157_d_n13, eq79_e1157_d_n14, eq79_e1157_d_n15, eq79_e1157_d_n16, eq79_e1157_d_n17, eq79_e1157_d_n18, eq79_e1157_d_n19, eq79_e1157_d_n20, eq79_e1157_d_n21, eq79_e1157_d_n22, eq79_e1157_d_n23, eq79_e1157_d_n24, eq79_e1157_d_n25, eq79_e1157_d_n26, eq79_e1157_d_n27, eq79_e1157_d_n28, eq79_e1157_d_n29, eq79_e1157_q, eq79_e1157_q_d_n0, eq79_e1157_q_d_n1, eq79_e1157_q_d_n2, eq79_e1157_q_d_n3, eq79_e1157_q_d_n4, eq79_e1157_q_d_n5, eq79_e1157_q_d_n6, eq79_e1157_q_d_n7, eq79_e1157_q_d_n8, eq79_e1157_q_d_n9, eq79_e1157_q_d_n10, eq79_e1157_q_d_n11, eq79_e1157_q_d_n12, eq79_e1157_q_d_n13, eq79_e1157_q_d_n14, eq79_e1157_q_d_n15, eq79_e1157_q_d_n16, eq79_e1157_q_d_n17, eq79_e1157_q_d_n18, eq79_e1157_q_d_n19, eq79_e1157_q_d_n20, eq79_e1157_q_d_n21, eq79_e1157_q_d_n22, eq79_e1157_q_d_n23, eq79_e1157_q_d_n24, eq79_e1157_q_d_n25, eq79_e1157_q_d_n26, eq79_e1157_q_d_n27, eq79_e1157_q_d_n28, eq79_e1157_q_d_n29,) = {
    if (!(s.v[907] != 0.0)) {
        let eq79_e1150_q: f64 = s.v[193];
        let eq79_e1153: f64 = (p.p355 * (nv7 - nv5));
        let eq79_e1153_d_n5: f64 = (-p.p355);
        let eq79_e1153_d_n7: f64 = p.p355;
        let eq79_e1154_q: f64 = eq79_e1153;
        let eq79_e1155: f64 = (s.v[193] + eq79_e1153);
        let eq79_e1155_d_n5: f64 = (s.dn[193][5] + eq79_e1153_d_n5);
        let eq79_e1155_d_n7: f64 = (s.dn[193][7] + eq79_e1153_d_n7);
        let eq79_e1155_q: f64 = (eq79_e1150_q + eq79_e1154_q);
        let eq79_e1155_q_d_n5: f64 = (s.dn[193][5] + eq79_e1153_d_n5);
        let eq79_e1155_q_d_n7: f64 = (s.dn[193][7] + eq79_e1153_d_n7);
        (eq79_e1155, s.dn[193][0], s.dn[193][1], s.dn[193][2], s.dn[193][3], s.dn[193][4], eq79_e1155_d_n5, s.dn[193][6], eq79_e1155_d_n7, s.dn[193][8], s.dn[193][9], s.dn[193][10], s.dn[193][11], s.dn[193][12], s.dn[193][13], s.dn[193][14], s.dn[193][15], s.dn[193][16], s.dn[193][17], s.dn[193][18], s.dn[193][19], s.dn[193][20], s.dn[193][21], s.dn[193][22], s.dn[193][23], s.dn[193][24], s.dn[193][25], s.dn[193][26], s.dn[193][27], s.dn[193][28], s.dn[193][29], eq79_e1155_q, s.dn[193][0], s.dn[193][1], s.dn[193][2], s.dn[193][3], s.dn[193][4], eq79_e1155_q_d_n5, s.dn[193][6], eq79_e1155_q_d_n7, s.dn[193][8], s.dn[193][9], s.dn[193][10], s.dn[193][11], s.dn[193][12], s.dn[193][13], s.dn[193][14], s.dn[193][15], s.dn[193][16], s.dn[193][17], s.dn[193][18], s.dn[193][19], s.dn[193][20], s.dn[193][21], s.dn[193][22], s.dn[193][23], s.dn[193][24], s.dn[193][25], s.dn[193][26], s.dn[193][27], s.dn[193][28], s.dn[193][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq79_reactive_node_derivatives: [f64; 30] = [eq79_e1157_q_d_n0, eq79_e1157_q_d_n1, eq79_e1157_q_d_n2, eq79_e1157_q_d_n3, eq79_e1157_q_d_n4, eq79_e1157_q_d_n5, eq79_e1157_q_d_n6, eq79_e1157_q_d_n7, eq79_e1157_q_d_n8, eq79_e1157_q_d_n9, eq79_e1157_q_d_n10, eq79_e1157_q_d_n11, eq79_e1157_q_d_n12, eq79_e1157_q_d_n13, eq79_e1157_q_d_n14, eq79_e1157_q_d_n15, eq79_e1157_q_d_n16, eq79_e1157_q_d_n17, eq79_e1157_q_d_n18, eq79_e1157_q_d_n19, eq79_e1157_q_d_n20, eq79_e1157_q_d_n21, eq79_e1157_q_d_n22, eq79_e1157_q_d_n23, eq79_e1157_q_d_n24, eq79_e1157_q_d_n25, eq79_e1157_q_d_n26, eq79_e1157_q_d_n27, eq79_e1157_q_d_n28, eq79_e1157_q_d_n29];
        let eq79_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            &nodes,
            &eq79_reactive_node_derivatives,
            &branches,
            &eq79_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_82_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq82_e1169_q: f64 = s.v[194];
        let eq82_e1172: f64 = (p.p355 * (nv3 - nv5));
        let eq82_e1172_d_n3: f64 = p.p355;
        let eq82_e1172_d_n5: f64 = (-p.p355);
        let eq82_e1173_q: f64 = eq82_e1172;
        let eq82_e1174: f64 = (s.v[194] + eq82_e1172);
        let eq82_e1174_d_n3: f64 = (s.dn[194][3] + eq82_e1172_d_n3);
        let eq82_e1174_d_n5: f64 = (s.dn[194][5] + eq82_e1172_d_n5);
        let eq82_e1174_q: f64 = (eq82_e1169_q + eq82_e1173_q);
        let eq82_e1174_q_d_n3: f64 = (s.dn[194][3] + eq82_e1172_d_n3);
        let eq82_e1174_q_d_n5: f64 = (s.dn[194][5] + eq82_e1172_d_n5);
        let eq82_reactive_node_derivatives: [f64; 30] = [s.dn[194][0], s.dn[194][1], s.dn[194][2], eq82_e1174_q_d_n3, s.dn[194][4], eq82_e1174_q_d_n5, s.dn[194][6], s.dn[194][7], s.dn[194][8], s.dn[194][9], s.dn[194][10], s.dn[194][11], s.dn[194][12], s.dn[194][13], s.dn[194][14], s.dn[194][15], s.dn[194][16], s.dn[194][17], s.dn[194][18], s.dn[194][19], s.dn[194][20], s.dn[194][21], s.dn[194][22], s.dn[194][23], s.dn[194][24], s.dn[194][25], s.dn[194][26], s.dn[194][27], s.dn[194][28], s.dn[194][29]];
        let eq82_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            &nodes,
            &eq82_reactive_node_derivatives,
            &branches,
            &eq82_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_85_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq85_e1197, eq85_e1197_d_n0, eq85_e1197_d_n1, eq85_e1197_d_n2, eq85_e1197_d_n3, eq85_e1197_d_n4, eq85_e1197_d_n5, eq85_e1197_d_n6, eq85_e1197_d_n7, eq85_e1197_d_n8, eq85_e1197_d_n9, eq85_e1197_d_n10, eq85_e1197_d_n11, eq85_e1197_d_n12, eq85_e1197_d_n13, eq85_e1197_d_n14, eq85_e1197_d_n15, eq85_e1197_d_n16, eq85_e1197_d_n17, eq85_e1197_d_n18, eq85_e1197_d_n19, eq85_e1197_d_n20, eq85_e1197_d_n21, eq85_e1197_d_n22, eq85_e1197_d_n23, eq85_e1197_d_n24, eq85_e1197_d_n25, eq85_e1197_d_n26, eq85_e1197_d_n27, eq85_e1197_d_n28, eq85_e1197_d_n29, eq85_e1197_q, eq85_e1197_q_d_n0, eq85_e1197_q_d_n1, eq85_e1197_q_d_n2, eq85_e1197_q_d_n3, eq85_e1197_q_d_n4, eq85_e1197_q_d_n5, eq85_e1197_q_d_n6, eq85_e1197_q_d_n7, eq85_e1197_q_d_n8, eq85_e1197_q_d_n9, eq85_e1197_q_d_n10, eq85_e1197_q_d_n11, eq85_e1197_q_d_n12, eq85_e1197_q_d_n13, eq85_e1197_q_d_n14, eq85_e1197_q_d_n15, eq85_e1197_q_d_n16, eq85_e1197_q_d_n17, eq85_e1197_q_d_n18, eq85_e1197_q_d_n19, eq85_e1197_q_d_n20, eq85_e1197_q_d_n21, eq85_e1197_q_d_n22, eq85_e1197_q_d_n23, eq85_e1197_q_d_n24, eq85_e1197_q_d_n25, eq85_e1197_q_d_n26, eq85_e1197_q_d_n27, eq85_e1197_q_d_n28, eq85_e1197_q_d_n29,) = {
    if (s.v[1054] != 0.0) {
        let eq85_e1190_q: f64 = s.v[167];
        let eq85_e1193: f64 = (p.p355 * (nv7 - nv10));
        let eq85_e1193_d_n7: f64 = p.p355;
        let eq85_e1193_d_n10: f64 = (-p.p355);
        let eq85_e1194_q: f64 = eq85_e1193;
        let eq85_e1195: f64 = (s.v[167] + eq85_e1193);
        let eq85_e1195_d_n7: f64 = (s.dn[167][7] + eq85_e1193_d_n7);
        let eq85_e1195_d_n10: f64 = (s.dn[167][10] + eq85_e1193_d_n10);
        let eq85_e1195_q: f64 = (eq85_e1190_q + eq85_e1194_q);
        let eq85_e1195_q_d_n7: f64 = (s.dn[167][7] + eq85_e1193_d_n7);
        let eq85_e1195_q_d_n10: f64 = (s.dn[167][10] + eq85_e1193_d_n10);
        (eq85_e1195, s.dn[167][0], s.dn[167][1], s.dn[167][2], s.dn[167][3], s.dn[167][4], s.dn[167][5], s.dn[167][6], eq85_e1195_d_n7, s.dn[167][8], s.dn[167][9], eq85_e1195_d_n10, s.dn[167][11], s.dn[167][12], s.dn[167][13], s.dn[167][14], s.dn[167][15], s.dn[167][16], s.dn[167][17], s.dn[167][18], s.dn[167][19], s.dn[167][20], s.dn[167][21], s.dn[167][22], s.dn[167][23], s.dn[167][24], s.dn[167][25], s.dn[167][26], s.dn[167][27], s.dn[167][28], s.dn[167][29], eq85_e1195_q, s.dn[167][0], s.dn[167][1], s.dn[167][2], s.dn[167][3], s.dn[167][4], s.dn[167][5], s.dn[167][6], eq85_e1195_q_d_n7, s.dn[167][8], s.dn[167][9], eq85_e1195_q_d_n10, s.dn[167][11], s.dn[167][12], s.dn[167][13], s.dn[167][14], s.dn[167][15], s.dn[167][16], s.dn[167][17], s.dn[167][18], s.dn[167][19], s.dn[167][20], s.dn[167][21], s.dn[167][22], s.dn[167][23], s.dn[167][24], s.dn[167][25], s.dn[167][26], s.dn[167][27], s.dn[167][28], s.dn[167][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq85_reactive_node_derivatives: [f64; 30] = [eq85_e1197_q_d_n0, eq85_e1197_q_d_n1, eq85_e1197_q_d_n2, eq85_e1197_q_d_n3, eq85_e1197_q_d_n4, eq85_e1197_q_d_n5, eq85_e1197_q_d_n6, eq85_e1197_q_d_n7, eq85_e1197_q_d_n8, eq85_e1197_q_d_n9, eq85_e1197_q_d_n10, eq85_e1197_q_d_n11, eq85_e1197_q_d_n12, eq85_e1197_q_d_n13, eq85_e1197_q_d_n14, eq85_e1197_q_d_n15, eq85_e1197_q_d_n16, eq85_e1197_q_d_n17, eq85_e1197_q_d_n18, eq85_e1197_q_d_n19, eq85_e1197_q_d_n20, eq85_e1197_q_d_n21, eq85_e1197_q_d_n22, eq85_e1197_q_d_n23, eq85_e1197_q_d_n24, eq85_e1197_q_d_n25, eq85_e1197_q_d_n26, eq85_e1197_q_d_n27, eq85_e1197_q_d_n28, eq85_e1197_q_d_n29];
        let eq85_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            &nodes,
            &eq85_reactive_node_derivatives,
            &branches,
            &eq85_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_86_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq86_e1207, eq86_e1207_d_n0, eq86_e1207_d_n1, eq86_e1207_d_n2, eq86_e1207_d_n3, eq86_e1207_d_n4, eq86_e1207_d_n5, eq86_e1207_d_n6, eq86_e1207_d_n7, eq86_e1207_d_n8, eq86_e1207_d_n9, eq86_e1207_d_n10, eq86_e1207_d_n11, eq86_e1207_d_n12, eq86_e1207_d_n13, eq86_e1207_d_n14, eq86_e1207_d_n15, eq86_e1207_d_n16, eq86_e1207_d_n17, eq86_e1207_d_n18, eq86_e1207_d_n19, eq86_e1207_d_n20, eq86_e1207_d_n21, eq86_e1207_d_n22, eq86_e1207_d_n23, eq86_e1207_d_n24, eq86_e1207_d_n25, eq86_e1207_d_n26, eq86_e1207_d_n27, eq86_e1207_d_n28, eq86_e1207_d_n29, eq86_e1207_q, eq86_e1207_q_d_n0, eq86_e1207_q_d_n1, eq86_e1207_q_d_n2, eq86_e1207_q_d_n3, eq86_e1207_q_d_n4, eq86_e1207_q_d_n5, eq86_e1207_q_d_n6, eq86_e1207_q_d_n7, eq86_e1207_q_d_n8, eq86_e1207_q_d_n9, eq86_e1207_q_d_n10, eq86_e1207_q_d_n11, eq86_e1207_q_d_n12, eq86_e1207_q_d_n13, eq86_e1207_q_d_n14, eq86_e1207_q_d_n15, eq86_e1207_q_d_n16, eq86_e1207_q_d_n17, eq86_e1207_q_d_n18, eq86_e1207_q_d_n19, eq86_e1207_q_d_n20, eq86_e1207_q_d_n21, eq86_e1207_q_d_n22, eq86_e1207_q_d_n23, eq86_e1207_q_d_n24, eq86_e1207_q_d_n25, eq86_e1207_q_d_n26, eq86_e1207_q_d_n27, eq86_e1207_q_d_n28, eq86_e1207_q_d_n29,) = {
    if (s.v[1054] != 0.0) {
        let eq86_e1200_q: f64 = s.v[168];
        let eq86_e1203: f64 = (p.p355 * (nv7 - nv9));
        let eq86_e1203_d_n7: f64 = p.p355;
        let eq86_e1203_d_n9: f64 = (-p.p355);
        let eq86_e1204_q: f64 = eq86_e1203;
        let eq86_e1205: f64 = (s.v[168] + eq86_e1203);
        let eq86_e1205_d_n7: f64 = (s.dn[168][7] + eq86_e1203_d_n7);
        let eq86_e1205_d_n9: f64 = (s.dn[168][9] + eq86_e1203_d_n9);
        let eq86_e1205_q: f64 = (eq86_e1200_q + eq86_e1204_q);
        let eq86_e1205_q_d_n7: f64 = (s.dn[168][7] + eq86_e1203_d_n7);
        let eq86_e1205_q_d_n9: f64 = (s.dn[168][9] + eq86_e1203_d_n9);
        (eq86_e1205, s.dn[168][0], s.dn[168][1], s.dn[168][2], s.dn[168][3], s.dn[168][4], s.dn[168][5], s.dn[168][6], eq86_e1205_d_n7, s.dn[168][8], eq86_e1205_d_n9, s.dn[168][10], s.dn[168][11], s.dn[168][12], s.dn[168][13], s.dn[168][14], s.dn[168][15], s.dn[168][16], s.dn[168][17], s.dn[168][18], s.dn[168][19], s.dn[168][20], s.dn[168][21], s.dn[168][22], s.dn[168][23], s.dn[168][24], s.dn[168][25], s.dn[168][26], s.dn[168][27], s.dn[168][28], s.dn[168][29], eq86_e1205_q, s.dn[168][0], s.dn[168][1], s.dn[168][2], s.dn[168][3], s.dn[168][4], s.dn[168][5], s.dn[168][6], eq86_e1205_q_d_n7, s.dn[168][8], eq86_e1205_q_d_n9, s.dn[168][10], s.dn[168][11], s.dn[168][12], s.dn[168][13], s.dn[168][14], s.dn[168][15], s.dn[168][16], s.dn[168][17], s.dn[168][18], s.dn[168][19], s.dn[168][20], s.dn[168][21], s.dn[168][22], s.dn[168][23], s.dn[168][24], s.dn[168][25], s.dn[168][26], s.dn[168][27], s.dn[168][28], s.dn[168][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq86_reactive_node_derivatives: [f64; 30] = [eq86_e1207_q_d_n0, eq86_e1207_q_d_n1, eq86_e1207_q_d_n2, eq86_e1207_q_d_n3, eq86_e1207_q_d_n4, eq86_e1207_q_d_n5, eq86_e1207_q_d_n6, eq86_e1207_q_d_n7, eq86_e1207_q_d_n8, eq86_e1207_q_d_n9, eq86_e1207_q_d_n10, eq86_e1207_q_d_n11, eq86_e1207_q_d_n12, eq86_e1207_q_d_n13, eq86_e1207_q_d_n14, eq86_e1207_q_d_n15, eq86_e1207_q_d_n16, eq86_e1207_q_d_n17, eq86_e1207_q_d_n18, eq86_e1207_q_d_n19, eq86_e1207_q_d_n20, eq86_e1207_q_d_n21, eq86_e1207_q_d_n22, eq86_e1207_q_d_n23, eq86_e1207_q_d_n24, eq86_e1207_q_d_n25, eq86_e1207_q_d_n26, eq86_e1207_q_d_n27, eq86_e1207_q_d_n28, eq86_e1207_q_d_n29];
        let eq86_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            &nodes,
            &eq86_reactive_node_derivatives,
            &branches,
            &eq86_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_87_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq87_e1217, eq87_e1217_d_n0, eq87_e1217_d_n1, eq87_e1217_d_n2, eq87_e1217_d_n3, eq87_e1217_d_n4, eq87_e1217_d_n5, eq87_e1217_d_n6, eq87_e1217_d_n7, eq87_e1217_d_n8, eq87_e1217_d_n9, eq87_e1217_d_n10, eq87_e1217_d_n11, eq87_e1217_d_n12, eq87_e1217_d_n13, eq87_e1217_d_n14, eq87_e1217_d_n15, eq87_e1217_d_n16, eq87_e1217_d_n17, eq87_e1217_d_n18, eq87_e1217_d_n19, eq87_e1217_d_n20, eq87_e1217_d_n21, eq87_e1217_d_n22, eq87_e1217_d_n23, eq87_e1217_d_n24, eq87_e1217_d_n25, eq87_e1217_d_n26, eq87_e1217_d_n27, eq87_e1217_d_n28, eq87_e1217_d_n29, eq87_e1217_q, eq87_e1217_q_d_n0, eq87_e1217_q_d_n1, eq87_e1217_q_d_n2, eq87_e1217_q_d_n3, eq87_e1217_q_d_n4, eq87_e1217_q_d_n5, eq87_e1217_q_d_n6, eq87_e1217_q_d_n7, eq87_e1217_q_d_n8, eq87_e1217_q_d_n9, eq87_e1217_q_d_n10, eq87_e1217_q_d_n11, eq87_e1217_q_d_n12, eq87_e1217_q_d_n13, eq87_e1217_q_d_n14, eq87_e1217_q_d_n15, eq87_e1217_q_d_n16, eq87_e1217_q_d_n17, eq87_e1217_q_d_n18, eq87_e1217_q_d_n19, eq87_e1217_q_d_n20, eq87_e1217_q_d_n21, eq87_e1217_q_d_n22, eq87_e1217_q_d_n23, eq87_e1217_q_d_n24, eq87_e1217_q_d_n25, eq87_e1217_q_d_n26, eq87_e1217_q_d_n27, eq87_e1217_q_d_n28, eq87_e1217_q_d_n29,) = {
    if (s.v[1054] != 0.0) {
        let eq87_e1210_q: f64 = s.v[169];
        let eq87_e1213: f64 = (p.p355 * (nv2 - nv10));
        let eq87_e1213_d_n2: f64 = p.p355;
        let eq87_e1213_d_n10: f64 = (-p.p355);
        let eq87_e1214_q: f64 = eq87_e1213;
        let eq87_e1215: f64 = (s.v[169] + eq87_e1213);
        let eq87_e1215_d_n2: f64 = (s.dn[169][2] + eq87_e1213_d_n2);
        let eq87_e1215_d_n10: f64 = (s.dn[169][10] + eq87_e1213_d_n10);
        let eq87_e1215_q: f64 = (eq87_e1210_q + eq87_e1214_q);
        let eq87_e1215_q_d_n2: f64 = (s.dn[169][2] + eq87_e1213_d_n2);
        let eq87_e1215_q_d_n10: f64 = (s.dn[169][10] + eq87_e1213_d_n10);
        (eq87_e1215, s.dn[169][0], s.dn[169][1], eq87_e1215_d_n2, s.dn[169][3], s.dn[169][4], s.dn[169][5], s.dn[169][6], s.dn[169][7], s.dn[169][8], s.dn[169][9], eq87_e1215_d_n10, s.dn[169][11], s.dn[169][12], s.dn[169][13], s.dn[169][14], s.dn[169][15], s.dn[169][16], s.dn[169][17], s.dn[169][18], s.dn[169][19], s.dn[169][20], s.dn[169][21], s.dn[169][22], s.dn[169][23], s.dn[169][24], s.dn[169][25], s.dn[169][26], s.dn[169][27], s.dn[169][28], s.dn[169][29], eq87_e1215_q, s.dn[169][0], s.dn[169][1], eq87_e1215_q_d_n2, s.dn[169][3], s.dn[169][4], s.dn[169][5], s.dn[169][6], s.dn[169][7], s.dn[169][8], s.dn[169][9], eq87_e1215_q_d_n10, s.dn[169][11], s.dn[169][12], s.dn[169][13], s.dn[169][14], s.dn[169][15], s.dn[169][16], s.dn[169][17], s.dn[169][18], s.dn[169][19], s.dn[169][20], s.dn[169][21], s.dn[169][22], s.dn[169][23], s.dn[169][24], s.dn[169][25], s.dn[169][26], s.dn[169][27], s.dn[169][28], s.dn[169][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq87_reactive_node_derivatives: [f64; 30] = [eq87_e1217_q_d_n0, eq87_e1217_q_d_n1, eq87_e1217_q_d_n2, eq87_e1217_q_d_n3, eq87_e1217_q_d_n4, eq87_e1217_q_d_n5, eq87_e1217_q_d_n6, eq87_e1217_q_d_n7, eq87_e1217_q_d_n8, eq87_e1217_q_d_n9, eq87_e1217_q_d_n10, eq87_e1217_q_d_n11, eq87_e1217_q_d_n12, eq87_e1217_q_d_n13, eq87_e1217_q_d_n14, eq87_e1217_q_d_n15, eq87_e1217_q_d_n16, eq87_e1217_q_d_n17, eq87_e1217_q_d_n18, eq87_e1217_q_d_n19, eq87_e1217_q_d_n20, eq87_e1217_q_d_n21, eq87_e1217_q_d_n22, eq87_e1217_q_d_n23, eq87_e1217_q_d_n24, eq87_e1217_q_d_n25, eq87_e1217_q_d_n26, eq87_e1217_q_d_n27, eq87_e1217_q_d_n28, eq87_e1217_q_d_n29];
        let eq87_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[10]),
            &nodes,
            &eq87_reactive_node_derivatives,
            &branches,
            &eq87_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_89_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq89_e1231, eq89_e1231_d_n0, eq89_e1231_d_n1, eq89_e1231_d_n2, eq89_e1231_d_n3, eq89_e1231_d_n4, eq89_e1231_d_n5, eq89_e1231_d_n6, eq89_e1231_d_n7, eq89_e1231_d_n8, eq89_e1231_d_n9, eq89_e1231_d_n10, eq89_e1231_d_n11, eq89_e1231_d_n12, eq89_e1231_d_n13, eq89_e1231_d_n14, eq89_e1231_d_n15, eq89_e1231_d_n16, eq89_e1231_d_n17, eq89_e1231_d_n18, eq89_e1231_d_n19, eq89_e1231_d_n20, eq89_e1231_d_n21, eq89_e1231_d_n22, eq89_e1231_d_n23, eq89_e1231_d_n24, eq89_e1231_d_n25, eq89_e1231_d_n26, eq89_e1231_d_n27, eq89_e1231_d_n28, eq89_e1231_d_n29, eq89_e1231_q, eq89_e1231_q_d_n0, eq89_e1231_q_d_n1, eq89_e1231_q_d_n2, eq89_e1231_q_d_n3, eq89_e1231_q_d_n4, eq89_e1231_q_d_n5, eq89_e1231_q_d_n6, eq89_e1231_q_d_n7, eq89_e1231_q_d_n8, eq89_e1231_q_d_n9, eq89_e1231_q_d_n10, eq89_e1231_q_d_n11, eq89_e1231_q_d_n12, eq89_e1231_q_d_n13, eq89_e1231_q_d_n14, eq89_e1231_q_d_n15, eq89_e1231_q_d_n16, eq89_e1231_q_d_n17, eq89_e1231_q_d_n18, eq89_e1231_q_d_n19, eq89_e1231_q_d_n20, eq89_e1231_q_d_n21, eq89_e1231_q_d_n22, eq89_e1231_q_d_n23, eq89_e1231_q_d_n24, eq89_e1231_q_d_n25, eq89_e1231_q_d_n26, eq89_e1231_q_d_n27, eq89_e1231_q_d_n28, eq89_e1231_q_d_n29,) = {
    if (s.v[1054] != 0.0) {
        let eq89_e1224_q: f64 = s.v[171];
        let eq89_e1227: f64 = (p.p355 * (nv7 - nv9));
        let eq89_e1227_d_n7: f64 = p.p355;
        let eq89_e1227_d_n9: f64 = (-p.p355);
        let eq89_e1228_q: f64 = eq89_e1227;
        let eq89_e1229: f64 = (s.v[171] + eq89_e1227);
        let eq89_e1229_d_n7: f64 = (s.dn[171][7] + eq89_e1227_d_n7);
        let eq89_e1229_d_n9: f64 = (s.dn[171][9] + eq89_e1227_d_n9);
        let eq89_e1229_q: f64 = (eq89_e1224_q + eq89_e1228_q);
        let eq89_e1229_q_d_n7: f64 = (s.dn[171][7] + eq89_e1227_d_n7);
        let eq89_e1229_q_d_n9: f64 = (s.dn[171][9] + eq89_e1227_d_n9);
        (eq89_e1229, s.dn[171][0], s.dn[171][1], s.dn[171][2], s.dn[171][3], s.dn[171][4], s.dn[171][5], s.dn[171][6], eq89_e1229_d_n7, s.dn[171][8], eq89_e1229_d_n9, s.dn[171][10], s.dn[171][11], s.dn[171][12], s.dn[171][13], s.dn[171][14], s.dn[171][15], s.dn[171][16], s.dn[171][17], s.dn[171][18], s.dn[171][19], s.dn[171][20], s.dn[171][21], s.dn[171][22], s.dn[171][23], s.dn[171][24], s.dn[171][25], s.dn[171][26], s.dn[171][27], s.dn[171][28], s.dn[171][29], eq89_e1229_q, s.dn[171][0], s.dn[171][1], s.dn[171][2], s.dn[171][3], s.dn[171][4], s.dn[171][5], s.dn[171][6], eq89_e1229_q_d_n7, s.dn[171][8], eq89_e1229_q_d_n9, s.dn[171][10], s.dn[171][11], s.dn[171][12], s.dn[171][13], s.dn[171][14], s.dn[171][15], s.dn[171][16], s.dn[171][17], s.dn[171][18], s.dn[171][19], s.dn[171][20], s.dn[171][21], s.dn[171][22], s.dn[171][23], s.dn[171][24], s.dn[171][25], s.dn[171][26], s.dn[171][27], s.dn[171][28], s.dn[171][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq89_reactive_node_derivatives: [f64; 30] = [eq89_e1231_q_d_n0, eq89_e1231_q_d_n1, eq89_e1231_q_d_n2, eq89_e1231_q_d_n3, eq89_e1231_q_d_n4, eq89_e1231_q_d_n5, eq89_e1231_q_d_n6, eq89_e1231_q_d_n7, eq89_e1231_q_d_n8, eq89_e1231_q_d_n9, eq89_e1231_q_d_n10, eq89_e1231_q_d_n11, eq89_e1231_q_d_n12, eq89_e1231_q_d_n13, eq89_e1231_q_d_n14, eq89_e1231_q_d_n15, eq89_e1231_q_d_n16, eq89_e1231_q_d_n17, eq89_e1231_q_d_n18, eq89_e1231_q_d_n19, eq89_e1231_q_d_n20, eq89_e1231_q_d_n21, eq89_e1231_q_d_n22, eq89_e1231_q_d_n23, eq89_e1231_q_d_n24, eq89_e1231_q_d_n25, eq89_e1231_q_d_n26, eq89_e1231_q_d_n27, eq89_e1231_q_d_n28, eq89_e1231_q_d_n29];
        let eq89_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            &nodes,
            &eq89_reactive_node_derivatives,
            &branches,
            &eq89_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_90_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq90_e1242, eq90_e1242_d_n0, eq90_e1242_d_n1, eq90_e1242_d_n2, eq90_e1242_d_n3, eq90_e1242_d_n4, eq90_e1242_d_n5, eq90_e1242_d_n6, eq90_e1242_d_n7, eq90_e1242_d_n8, eq90_e1242_d_n9, eq90_e1242_d_n10, eq90_e1242_d_n11, eq90_e1242_d_n12, eq90_e1242_d_n13, eq90_e1242_d_n14, eq90_e1242_d_n15, eq90_e1242_d_n16, eq90_e1242_d_n17, eq90_e1242_d_n18, eq90_e1242_d_n19, eq90_e1242_d_n20, eq90_e1242_d_n21, eq90_e1242_d_n22, eq90_e1242_d_n23, eq90_e1242_d_n24, eq90_e1242_d_n25, eq90_e1242_d_n26, eq90_e1242_d_n27, eq90_e1242_d_n28, eq90_e1242_d_n29, eq90_e1242_q, eq90_e1242_q_d_n0, eq90_e1242_q_d_n1, eq90_e1242_q_d_n2, eq90_e1242_q_d_n3, eq90_e1242_q_d_n4, eq90_e1242_q_d_n5, eq90_e1242_q_d_n6, eq90_e1242_q_d_n7, eq90_e1242_q_d_n8, eq90_e1242_q_d_n9, eq90_e1242_q_d_n10, eq90_e1242_q_d_n11, eq90_e1242_q_d_n12, eq90_e1242_q_d_n13, eq90_e1242_q_d_n14, eq90_e1242_q_d_n15, eq90_e1242_q_d_n16, eq90_e1242_q_d_n17, eq90_e1242_q_d_n18, eq90_e1242_q_d_n19, eq90_e1242_q_d_n20, eq90_e1242_q_d_n21, eq90_e1242_q_d_n22, eq90_e1242_q_d_n23, eq90_e1242_q_d_n24, eq90_e1242_q_d_n25, eq90_e1242_q_d_n26, eq90_e1242_q_d_n27, eq90_e1242_q_d_n28, eq90_e1242_q_d_n29,) = {
    if (!(s.v[1054] != 0.0)) {
        let eq90_e1235_q: f64 = s.v[167];
        let eq90_e1238: f64 = (p.p355 * (nv2 - nv10));
        let eq90_e1238_d_n2: f64 = p.p355;
        let eq90_e1238_d_n10: f64 = (-p.p355);
        let eq90_e1239_q: f64 = eq90_e1238;
        let eq90_e1240: f64 = (s.v[167] + eq90_e1238);
        let eq90_e1240_d_n2: f64 = (s.dn[167][2] + eq90_e1238_d_n2);
        let eq90_e1240_d_n10: f64 = (s.dn[167][10] + eq90_e1238_d_n10);
        let eq90_e1240_q: f64 = (eq90_e1235_q + eq90_e1239_q);
        let eq90_e1240_q_d_n2: f64 = (s.dn[167][2] + eq90_e1238_d_n2);
        let eq90_e1240_q_d_n10: f64 = (s.dn[167][10] + eq90_e1238_d_n10);
        (eq90_e1240, s.dn[167][0], s.dn[167][1], eq90_e1240_d_n2, s.dn[167][3], s.dn[167][4], s.dn[167][5], s.dn[167][6], s.dn[167][7], s.dn[167][8], s.dn[167][9], eq90_e1240_d_n10, s.dn[167][11], s.dn[167][12], s.dn[167][13], s.dn[167][14], s.dn[167][15], s.dn[167][16], s.dn[167][17], s.dn[167][18], s.dn[167][19], s.dn[167][20], s.dn[167][21], s.dn[167][22], s.dn[167][23], s.dn[167][24], s.dn[167][25], s.dn[167][26], s.dn[167][27], s.dn[167][28], s.dn[167][29], eq90_e1240_q, s.dn[167][0], s.dn[167][1], eq90_e1240_q_d_n2, s.dn[167][3], s.dn[167][4], s.dn[167][5], s.dn[167][6], s.dn[167][7], s.dn[167][8], s.dn[167][9], eq90_e1240_q_d_n10, s.dn[167][11], s.dn[167][12], s.dn[167][13], s.dn[167][14], s.dn[167][15], s.dn[167][16], s.dn[167][17], s.dn[167][18], s.dn[167][19], s.dn[167][20], s.dn[167][21], s.dn[167][22], s.dn[167][23], s.dn[167][24], s.dn[167][25], s.dn[167][26], s.dn[167][27], s.dn[167][28], s.dn[167][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq90_reactive_node_derivatives: [f64; 30] = [eq90_e1242_q_d_n0, eq90_e1242_q_d_n1, eq90_e1242_q_d_n2, eq90_e1242_q_d_n3, eq90_e1242_q_d_n4, eq90_e1242_q_d_n5, eq90_e1242_q_d_n6, eq90_e1242_q_d_n7, eq90_e1242_q_d_n8, eq90_e1242_q_d_n9, eq90_e1242_q_d_n10, eq90_e1242_q_d_n11, eq90_e1242_q_d_n12, eq90_e1242_q_d_n13, eq90_e1242_q_d_n14, eq90_e1242_q_d_n15, eq90_e1242_q_d_n16, eq90_e1242_q_d_n17, eq90_e1242_q_d_n18, eq90_e1242_q_d_n19, eq90_e1242_q_d_n20, eq90_e1242_q_d_n21, eq90_e1242_q_d_n22, eq90_e1242_q_d_n23, eq90_e1242_q_d_n24, eq90_e1242_q_d_n25, eq90_e1242_q_d_n26, eq90_e1242_q_d_n27, eq90_e1242_q_d_n28, eq90_e1242_q_d_n29];
        let eq90_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[10]),
            &nodes,
            &eq90_reactive_node_derivatives,
            &branches,
            &eq90_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_91_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq91_e1253, eq91_e1253_d_n0, eq91_e1253_d_n1, eq91_e1253_d_n2, eq91_e1253_d_n3, eq91_e1253_d_n4, eq91_e1253_d_n5, eq91_e1253_d_n6, eq91_e1253_d_n7, eq91_e1253_d_n8, eq91_e1253_d_n9, eq91_e1253_d_n10, eq91_e1253_d_n11, eq91_e1253_d_n12, eq91_e1253_d_n13, eq91_e1253_d_n14, eq91_e1253_d_n15, eq91_e1253_d_n16, eq91_e1253_d_n17, eq91_e1253_d_n18, eq91_e1253_d_n19, eq91_e1253_d_n20, eq91_e1253_d_n21, eq91_e1253_d_n22, eq91_e1253_d_n23, eq91_e1253_d_n24, eq91_e1253_d_n25, eq91_e1253_d_n26, eq91_e1253_d_n27, eq91_e1253_d_n28, eq91_e1253_d_n29, eq91_e1253_q, eq91_e1253_q_d_n0, eq91_e1253_q_d_n1, eq91_e1253_q_d_n2, eq91_e1253_q_d_n3, eq91_e1253_q_d_n4, eq91_e1253_q_d_n5, eq91_e1253_q_d_n6, eq91_e1253_q_d_n7, eq91_e1253_q_d_n8, eq91_e1253_q_d_n9, eq91_e1253_q_d_n10, eq91_e1253_q_d_n11, eq91_e1253_q_d_n12, eq91_e1253_q_d_n13, eq91_e1253_q_d_n14, eq91_e1253_q_d_n15, eq91_e1253_q_d_n16, eq91_e1253_q_d_n17, eq91_e1253_q_d_n18, eq91_e1253_q_d_n19, eq91_e1253_q_d_n20, eq91_e1253_q_d_n21, eq91_e1253_q_d_n22, eq91_e1253_q_d_n23, eq91_e1253_q_d_n24, eq91_e1253_q_d_n25, eq91_e1253_q_d_n26, eq91_e1253_q_d_n27, eq91_e1253_q_d_n28, eq91_e1253_q_d_n29,) = {
    if (!(s.v[1054] != 0.0)) {
        let eq91_e1246_q: f64 = s.v[168];
        let eq91_e1249: f64 = (p.p355 * (nv2 - nv9));
        let eq91_e1249_d_n2: f64 = p.p355;
        let eq91_e1249_d_n9: f64 = (-p.p355);
        let eq91_e1250_q: f64 = eq91_e1249;
        let eq91_e1251: f64 = (s.v[168] + eq91_e1249);
        let eq91_e1251_d_n2: f64 = (s.dn[168][2] + eq91_e1249_d_n2);
        let eq91_e1251_d_n9: f64 = (s.dn[168][9] + eq91_e1249_d_n9);
        let eq91_e1251_q: f64 = (eq91_e1246_q + eq91_e1250_q);
        let eq91_e1251_q_d_n2: f64 = (s.dn[168][2] + eq91_e1249_d_n2);
        let eq91_e1251_q_d_n9: f64 = (s.dn[168][9] + eq91_e1249_d_n9);
        (eq91_e1251, s.dn[168][0], s.dn[168][1], eq91_e1251_d_n2, s.dn[168][3], s.dn[168][4], s.dn[168][5], s.dn[168][6], s.dn[168][7], s.dn[168][8], eq91_e1251_d_n9, s.dn[168][10], s.dn[168][11], s.dn[168][12], s.dn[168][13], s.dn[168][14], s.dn[168][15], s.dn[168][16], s.dn[168][17], s.dn[168][18], s.dn[168][19], s.dn[168][20], s.dn[168][21], s.dn[168][22], s.dn[168][23], s.dn[168][24], s.dn[168][25], s.dn[168][26], s.dn[168][27], s.dn[168][28], s.dn[168][29], eq91_e1251_q, s.dn[168][0], s.dn[168][1], eq91_e1251_q_d_n2, s.dn[168][3], s.dn[168][4], s.dn[168][5], s.dn[168][6], s.dn[168][7], s.dn[168][8], eq91_e1251_q_d_n9, s.dn[168][10], s.dn[168][11], s.dn[168][12], s.dn[168][13], s.dn[168][14], s.dn[168][15], s.dn[168][16], s.dn[168][17], s.dn[168][18], s.dn[168][19], s.dn[168][20], s.dn[168][21], s.dn[168][22], s.dn[168][23], s.dn[168][24], s.dn[168][25], s.dn[168][26], s.dn[168][27], s.dn[168][28], s.dn[168][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq91_reactive_node_derivatives: [f64; 30] = [eq91_e1253_q_d_n0, eq91_e1253_q_d_n1, eq91_e1253_q_d_n2, eq91_e1253_q_d_n3, eq91_e1253_q_d_n4, eq91_e1253_q_d_n5, eq91_e1253_q_d_n6, eq91_e1253_q_d_n7, eq91_e1253_q_d_n8, eq91_e1253_q_d_n9, eq91_e1253_q_d_n10, eq91_e1253_q_d_n11, eq91_e1253_q_d_n12, eq91_e1253_q_d_n13, eq91_e1253_q_d_n14, eq91_e1253_q_d_n15, eq91_e1253_q_d_n16, eq91_e1253_q_d_n17, eq91_e1253_q_d_n18, eq91_e1253_q_d_n19, eq91_e1253_q_d_n20, eq91_e1253_q_d_n21, eq91_e1253_q_d_n22, eq91_e1253_q_d_n23, eq91_e1253_q_d_n24, eq91_e1253_q_d_n25, eq91_e1253_q_d_n26, eq91_e1253_q_d_n27, eq91_e1253_q_d_n28, eq91_e1253_q_d_n29];
        let eq91_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[9]),
            &nodes,
            &eq91_reactive_node_derivatives,
            &branches,
            &eq91_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_92_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq92_e1264, eq92_e1264_d_n0, eq92_e1264_d_n1, eq92_e1264_d_n2, eq92_e1264_d_n3, eq92_e1264_d_n4, eq92_e1264_d_n5, eq92_e1264_d_n6, eq92_e1264_d_n7, eq92_e1264_d_n8, eq92_e1264_d_n9, eq92_e1264_d_n10, eq92_e1264_d_n11, eq92_e1264_d_n12, eq92_e1264_d_n13, eq92_e1264_d_n14, eq92_e1264_d_n15, eq92_e1264_d_n16, eq92_e1264_d_n17, eq92_e1264_d_n18, eq92_e1264_d_n19, eq92_e1264_d_n20, eq92_e1264_d_n21, eq92_e1264_d_n22, eq92_e1264_d_n23, eq92_e1264_d_n24, eq92_e1264_d_n25, eq92_e1264_d_n26, eq92_e1264_d_n27, eq92_e1264_d_n28, eq92_e1264_d_n29, eq92_e1264_q, eq92_e1264_q_d_n0, eq92_e1264_q_d_n1, eq92_e1264_q_d_n2, eq92_e1264_q_d_n3, eq92_e1264_q_d_n4, eq92_e1264_q_d_n5, eq92_e1264_q_d_n6, eq92_e1264_q_d_n7, eq92_e1264_q_d_n8, eq92_e1264_q_d_n9, eq92_e1264_q_d_n10, eq92_e1264_q_d_n11, eq92_e1264_q_d_n12, eq92_e1264_q_d_n13, eq92_e1264_q_d_n14, eq92_e1264_q_d_n15, eq92_e1264_q_d_n16, eq92_e1264_q_d_n17, eq92_e1264_q_d_n18, eq92_e1264_q_d_n19, eq92_e1264_q_d_n20, eq92_e1264_q_d_n21, eq92_e1264_q_d_n22, eq92_e1264_q_d_n23, eq92_e1264_q_d_n24, eq92_e1264_q_d_n25, eq92_e1264_q_d_n26, eq92_e1264_q_d_n27, eq92_e1264_q_d_n28, eq92_e1264_q_d_n29,) = {
    if (!(s.v[1054] != 0.0)) {
        let eq92_e1257_q: f64 = s.v[169];
        let eq92_e1260: f64 = (p.p355 * (nv7 - nv10));
        let eq92_e1260_d_n7: f64 = p.p355;
        let eq92_e1260_d_n10: f64 = (-p.p355);
        let eq92_e1261_q: f64 = eq92_e1260;
        let eq92_e1262: f64 = (s.v[169] + eq92_e1260);
        let eq92_e1262_d_n7: f64 = (s.dn[169][7] + eq92_e1260_d_n7);
        let eq92_e1262_d_n10: f64 = (s.dn[169][10] + eq92_e1260_d_n10);
        let eq92_e1262_q: f64 = (eq92_e1257_q + eq92_e1261_q);
        let eq92_e1262_q_d_n7: f64 = (s.dn[169][7] + eq92_e1260_d_n7);
        let eq92_e1262_q_d_n10: f64 = (s.dn[169][10] + eq92_e1260_d_n10);
        (eq92_e1262, s.dn[169][0], s.dn[169][1], s.dn[169][2], s.dn[169][3], s.dn[169][4], s.dn[169][5], s.dn[169][6], eq92_e1262_d_n7, s.dn[169][8], s.dn[169][9], eq92_e1262_d_n10, s.dn[169][11], s.dn[169][12], s.dn[169][13], s.dn[169][14], s.dn[169][15], s.dn[169][16], s.dn[169][17], s.dn[169][18], s.dn[169][19], s.dn[169][20], s.dn[169][21], s.dn[169][22], s.dn[169][23], s.dn[169][24], s.dn[169][25], s.dn[169][26], s.dn[169][27], s.dn[169][28], s.dn[169][29], eq92_e1262_q, s.dn[169][0], s.dn[169][1], s.dn[169][2], s.dn[169][3], s.dn[169][4], s.dn[169][5], s.dn[169][6], eq92_e1262_q_d_n7, s.dn[169][8], s.dn[169][9], eq92_e1262_q_d_n10, s.dn[169][11], s.dn[169][12], s.dn[169][13], s.dn[169][14], s.dn[169][15], s.dn[169][16], s.dn[169][17], s.dn[169][18], s.dn[169][19], s.dn[169][20], s.dn[169][21], s.dn[169][22], s.dn[169][23], s.dn[169][24], s.dn[169][25], s.dn[169][26], s.dn[169][27], s.dn[169][28], s.dn[169][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq92_reactive_node_derivatives: [f64; 30] = [eq92_e1264_q_d_n0, eq92_e1264_q_d_n1, eq92_e1264_q_d_n2, eq92_e1264_q_d_n3, eq92_e1264_q_d_n4, eq92_e1264_q_d_n5, eq92_e1264_q_d_n6, eq92_e1264_q_d_n7, eq92_e1264_q_d_n8, eq92_e1264_q_d_n9, eq92_e1264_q_d_n10, eq92_e1264_q_d_n11, eq92_e1264_q_d_n12, eq92_e1264_q_d_n13, eq92_e1264_q_d_n14, eq92_e1264_q_d_n15, eq92_e1264_q_d_n16, eq92_e1264_q_d_n17, eq92_e1264_q_d_n18, eq92_e1264_q_d_n19, eq92_e1264_q_d_n20, eq92_e1264_q_d_n21, eq92_e1264_q_d_n22, eq92_e1264_q_d_n23, eq92_e1264_q_d_n24, eq92_e1264_q_d_n25, eq92_e1264_q_d_n26, eq92_e1264_q_d_n27, eq92_e1264_q_d_n28, eq92_e1264_q_d_n29];
        let eq92_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            &nodes,
            &eq92_reactive_node_derivatives,
            &branches,
            &eq92_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_95_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq95_e1276_q: f64 = s.v[170];
        let eq95_e1279: f64 = (p.p355 * (nv3 - nv10));
        let eq95_e1279_d_n3: f64 = p.p355;
        let eq95_e1279_d_n10: f64 = (-p.p355);
        let eq95_e1280_q: f64 = eq95_e1279;
        let eq95_e1281: f64 = (s.v[170] + eq95_e1279);
        let eq95_e1281_d_n3: f64 = (s.dn[170][3] + eq95_e1279_d_n3);
        let eq95_e1281_d_n10: f64 = (s.dn[170][10] + eq95_e1279_d_n10);
        let eq95_e1281_q: f64 = (eq95_e1276_q + eq95_e1280_q);
        let eq95_e1281_q_d_n3: f64 = (s.dn[170][3] + eq95_e1279_d_n3);
        let eq95_e1281_q_d_n10: f64 = (s.dn[170][10] + eq95_e1279_d_n10);
        let eq95_reactive_node_derivatives: [f64; 30] = [s.dn[170][0], s.dn[170][1], s.dn[170][2], eq95_e1281_q_d_n3, s.dn[170][4], s.dn[170][5], s.dn[170][6], s.dn[170][7], s.dn[170][8], s.dn[170][9], eq95_e1281_q_d_n10, s.dn[170][11], s.dn[170][12], s.dn[170][13], s.dn[170][14], s.dn[170][15], s.dn[170][16], s.dn[170][17], s.dn[170][18], s.dn[170][19], s.dn[170][20], s.dn[170][21], s.dn[170][22], s.dn[170][23], s.dn[170][24], s.dn[170][25], s.dn[170][26], s.dn[170][27], s.dn[170][28], s.dn[170][29]];
        let eq95_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[10]),
            &nodes,
            &eq95_reactive_node_derivatives,
            &branches,
            &eq95_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_98_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq98_e1304, eq98_e1304_d_n0, eq98_e1304_d_n1, eq98_e1304_d_n2, eq98_e1304_d_n3, eq98_e1304_d_n4, eq98_e1304_d_n5, eq98_e1304_d_n6, eq98_e1304_d_n7, eq98_e1304_d_n8, eq98_e1304_d_n9, eq98_e1304_d_n10, eq98_e1304_d_n11, eq98_e1304_d_n12, eq98_e1304_d_n13, eq98_e1304_d_n14, eq98_e1304_d_n15, eq98_e1304_d_n16, eq98_e1304_d_n17, eq98_e1304_d_n18, eq98_e1304_d_n19, eq98_e1304_d_n20, eq98_e1304_d_n21, eq98_e1304_d_n22, eq98_e1304_d_n23, eq98_e1304_d_n24, eq98_e1304_d_n25, eq98_e1304_d_n26, eq98_e1304_d_n27, eq98_e1304_d_n28, eq98_e1304_d_n29, eq98_e1304_q, eq98_e1304_q_d_n0, eq98_e1304_q_d_n1, eq98_e1304_q_d_n2, eq98_e1304_q_d_n3, eq98_e1304_q_d_n4, eq98_e1304_q_d_n5, eq98_e1304_q_d_n6, eq98_e1304_q_d_n7, eq98_e1304_q_d_n8, eq98_e1304_q_d_n9, eq98_e1304_q_d_n10, eq98_e1304_q_d_n11, eq98_e1304_q_d_n12, eq98_e1304_q_d_n13, eq98_e1304_q_d_n14, eq98_e1304_q_d_n15, eq98_e1304_q_d_n16, eq98_e1304_q_d_n17, eq98_e1304_q_d_n18, eq98_e1304_q_d_n19, eq98_e1304_q_d_n20, eq98_e1304_q_d_n21, eq98_e1304_q_d_n22, eq98_e1304_q_d_n23, eq98_e1304_q_d_n24, eq98_e1304_q_d_n25, eq98_e1304_q_d_n26, eq98_e1304_q_d_n27, eq98_e1304_q_d_n28, eq98_e1304_q_d_n29,) = {
    if (s.v[1201] != 0.0) {
        let eq98_e1297_q: f64 = s.v[173];
        let eq98_e1300: f64 = (p.p355 * (nv7 - nv11));
        let eq98_e1300_d_n7: f64 = p.p355;
        let eq98_e1300_d_n11: f64 = (-p.p355);
        let eq98_e1301_q: f64 = eq98_e1300;
        let eq98_e1302: f64 = (s.v[173] + eq98_e1300);
        let eq98_e1302_d_n7: f64 = (s.dn[173][7] + eq98_e1300_d_n7);
        let eq98_e1302_d_n11: f64 = (s.dn[173][11] + eq98_e1300_d_n11);
        let eq98_e1302_q: f64 = (eq98_e1297_q + eq98_e1301_q);
        let eq98_e1302_q_d_n7: f64 = (s.dn[173][7] + eq98_e1300_d_n7);
        let eq98_e1302_q_d_n11: f64 = (s.dn[173][11] + eq98_e1300_d_n11);
        (eq98_e1302, s.dn[173][0], s.dn[173][1], s.dn[173][2], s.dn[173][3], s.dn[173][4], s.dn[173][5], s.dn[173][6], eq98_e1302_d_n7, s.dn[173][8], s.dn[173][9], s.dn[173][10], eq98_e1302_d_n11, s.dn[173][12], s.dn[173][13], s.dn[173][14], s.dn[173][15], s.dn[173][16], s.dn[173][17], s.dn[173][18], s.dn[173][19], s.dn[173][20], s.dn[173][21], s.dn[173][22], s.dn[173][23], s.dn[173][24], s.dn[173][25], s.dn[173][26], s.dn[173][27], s.dn[173][28], s.dn[173][29], eq98_e1302_q, s.dn[173][0], s.dn[173][1], s.dn[173][2], s.dn[173][3], s.dn[173][4], s.dn[173][5], s.dn[173][6], eq98_e1302_q_d_n7, s.dn[173][8], s.dn[173][9], s.dn[173][10], eq98_e1302_q_d_n11, s.dn[173][12], s.dn[173][13], s.dn[173][14], s.dn[173][15], s.dn[173][16], s.dn[173][17], s.dn[173][18], s.dn[173][19], s.dn[173][20], s.dn[173][21], s.dn[173][22], s.dn[173][23], s.dn[173][24], s.dn[173][25], s.dn[173][26], s.dn[173][27], s.dn[173][28], s.dn[173][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq98_reactive_node_derivatives: [f64; 30] = [eq98_e1304_q_d_n0, eq98_e1304_q_d_n1, eq98_e1304_q_d_n2, eq98_e1304_q_d_n3, eq98_e1304_q_d_n4, eq98_e1304_q_d_n5, eq98_e1304_q_d_n6, eq98_e1304_q_d_n7, eq98_e1304_q_d_n8, eq98_e1304_q_d_n9, eq98_e1304_q_d_n10, eq98_e1304_q_d_n11, eq98_e1304_q_d_n12, eq98_e1304_q_d_n13, eq98_e1304_q_d_n14, eq98_e1304_q_d_n15, eq98_e1304_q_d_n16, eq98_e1304_q_d_n17, eq98_e1304_q_d_n18, eq98_e1304_q_d_n19, eq98_e1304_q_d_n20, eq98_e1304_q_d_n21, eq98_e1304_q_d_n22, eq98_e1304_q_d_n23, eq98_e1304_q_d_n24, eq98_e1304_q_d_n25, eq98_e1304_q_d_n26, eq98_e1304_q_d_n27, eq98_e1304_q_d_n28, eq98_e1304_q_d_n29];
        let eq98_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            &nodes,
            &eq98_reactive_node_derivatives,
            &branches,
            &eq98_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_99_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq99_e1314, eq99_e1314_d_n0, eq99_e1314_d_n1, eq99_e1314_d_n2, eq99_e1314_d_n3, eq99_e1314_d_n4, eq99_e1314_d_n5, eq99_e1314_d_n6, eq99_e1314_d_n7, eq99_e1314_d_n8, eq99_e1314_d_n9, eq99_e1314_d_n10, eq99_e1314_d_n11, eq99_e1314_d_n12, eq99_e1314_d_n13, eq99_e1314_d_n14, eq99_e1314_d_n15, eq99_e1314_d_n16, eq99_e1314_d_n17, eq99_e1314_d_n18, eq99_e1314_d_n19, eq99_e1314_d_n20, eq99_e1314_d_n21, eq99_e1314_d_n22, eq99_e1314_d_n23, eq99_e1314_d_n24, eq99_e1314_d_n25, eq99_e1314_d_n26, eq99_e1314_d_n27, eq99_e1314_d_n28, eq99_e1314_d_n29, eq99_e1314_q, eq99_e1314_q_d_n0, eq99_e1314_q_d_n1, eq99_e1314_q_d_n2, eq99_e1314_q_d_n3, eq99_e1314_q_d_n4, eq99_e1314_q_d_n5, eq99_e1314_q_d_n6, eq99_e1314_q_d_n7, eq99_e1314_q_d_n8, eq99_e1314_q_d_n9, eq99_e1314_q_d_n10, eq99_e1314_q_d_n11, eq99_e1314_q_d_n12, eq99_e1314_q_d_n13, eq99_e1314_q_d_n14, eq99_e1314_q_d_n15, eq99_e1314_q_d_n16, eq99_e1314_q_d_n17, eq99_e1314_q_d_n18, eq99_e1314_q_d_n19, eq99_e1314_q_d_n20, eq99_e1314_q_d_n21, eq99_e1314_q_d_n22, eq99_e1314_q_d_n23, eq99_e1314_q_d_n24, eq99_e1314_q_d_n25, eq99_e1314_q_d_n26, eq99_e1314_q_d_n27, eq99_e1314_q_d_n28, eq99_e1314_q_d_n29,) = {
    if (s.v[1201] != 0.0) {
        let eq99_e1307_q: f64 = s.v[174];
        let eq99_e1310: f64 = (p.p355 * (nv7 - nv10));
        let eq99_e1310_d_n7: f64 = p.p355;
        let eq99_e1310_d_n10: f64 = (-p.p355);
        let eq99_e1311_q: f64 = eq99_e1310;
        let eq99_e1312: f64 = (s.v[174] + eq99_e1310);
        let eq99_e1312_d_n7: f64 = (s.dn[174][7] + eq99_e1310_d_n7);
        let eq99_e1312_d_n10: f64 = (s.dn[174][10] + eq99_e1310_d_n10);
        let eq99_e1312_q: f64 = (eq99_e1307_q + eq99_e1311_q);
        let eq99_e1312_q_d_n7: f64 = (s.dn[174][7] + eq99_e1310_d_n7);
        let eq99_e1312_q_d_n10: f64 = (s.dn[174][10] + eq99_e1310_d_n10);
        (eq99_e1312, s.dn[174][0], s.dn[174][1], s.dn[174][2], s.dn[174][3], s.dn[174][4], s.dn[174][5], s.dn[174][6], eq99_e1312_d_n7, s.dn[174][8], s.dn[174][9], eq99_e1312_d_n10, s.dn[174][11], s.dn[174][12], s.dn[174][13], s.dn[174][14], s.dn[174][15], s.dn[174][16], s.dn[174][17], s.dn[174][18], s.dn[174][19], s.dn[174][20], s.dn[174][21], s.dn[174][22], s.dn[174][23], s.dn[174][24], s.dn[174][25], s.dn[174][26], s.dn[174][27], s.dn[174][28], s.dn[174][29], eq99_e1312_q, s.dn[174][0], s.dn[174][1], s.dn[174][2], s.dn[174][3], s.dn[174][4], s.dn[174][5], s.dn[174][6], eq99_e1312_q_d_n7, s.dn[174][8], s.dn[174][9], eq99_e1312_q_d_n10, s.dn[174][11], s.dn[174][12], s.dn[174][13], s.dn[174][14], s.dn[174][15], s.dn[174][16], s.dn[174][17], s.dn[174][18], s.dn[174][19], s.dn[174][20], s.dn[174][21], s.dn[174][22], s.dn[174][23], s.dn[174][24], s.dn[174][25], s.dn[174][26], s.dn[174][27], s.dn[174][28], s.dn[174][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq99_reactive_node_derivatives: [f64; 30] = [eq99_e1314_q_d_n0, eq99_e1314_q_d_n1, eq99_e1314_q_d_n2, eq99_e1314_q_d_n3, eq99_e1314_q_d_n4, eq99_e1314_q_d_n5, eq99_e1314_q_d_n6, eq99_e1314_q_d_n7, eq99_e1314_q_d_n8, eq99_e1314_q_d_n9, eq99_e1314_q_d_n10, eq99_e1314_q_d_n11, eq99_e1314_q_d_n12, eq99_e1314_q_d_n13, eq99_e1314_q_d_n14, eq99_e1314_q_d_n15, eq99_e1314_q_d_n16, eq99_e1314_q_d_n17, eq99_e1314_q_d_n18, eq99_e1314_q_d_n19, eq99_e1314_q_d_n20, eq99_e1314_q_d_n21, eq99_e1314_q_d_n22, eq99_e1314_q_d_n23, eq99_e1314_q_d_n24, eq99_e1314_q_d_n25, eq99_e1314_q_d_n26, eq99_e1314_q_d_n27, eq99_e1314_q_d_n28, eq99_e1314_q_d_n29];
        let eq99_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            &nodes,
            &eq99_reactive_node_derivatives,
            &branches,
            &eq99_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
