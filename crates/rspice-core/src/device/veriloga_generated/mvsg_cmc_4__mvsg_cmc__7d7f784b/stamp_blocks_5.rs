#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq74_e1110, eq74_e1110_d_n0, eq74_e1110_d_n1, eq74_e1110_d_n2, eq74_e1110_d_n3, eq74_e1110_d_n4, eq74_e1110_d_n5, eq74_e1110_d_n6, eq74_e1110_d_n7, eq74_e1110_d_n8, eq74_e1110_d_n9, eq74_e1110_d_n10, eq74_e1110_d_n11, eq74_e1110_d_n12, eq74_e1110_d_n13, eq74_e1110_d_n14, eq74_e1110_d_n15, eq74_e1110_d_n16, eq74_e1110_d_n17, eq74_e1110_d_n18, eq74_e1110_d_n19, eq74_e1110_d_n20, eq74_e1110_d_n21, eq74_e1110_d_n22, eq74_e1110_d_n23, eq74_e1110_d_n24, eq74_e1110_d_n25, eq74_e1110_d_n26, eq74_e1110_d_n27, eq74_e1110_d_n28, eq74_e1110_d_n29, eq74_e1110_q, eq74_e1110_q_d_n0, eq74_e1110_q_d_n1, eq74_e1110_q_d_n2, eq74_e1110_q_d_n3, eq74_e1110_q_d_n4, eq74_e1110_q_d_n5, eq74_e1110_q_d_n6, eq74_e1110_q_d_n7, eq74_e1110_q_d_n8, eq74_e1110_q_d_n9, eq74_e1110_q_d_n10, eq74_e1110_q_d_n11, eq74_e1110_q_d_n12, eq74_e1110_q_d_n13, eq74_e1110_q_d_n14, eq74_e1110_q_d_n15, eq74_e1110_q_d_n16, eq74_e1110_q_d_n17, eq74_e1110_q_d_n18, eq74_e1110_q_d_n19, eq74_e1110_q_d_n20, eq74_e1110_q_d_n21, eq74_e1110_q_d_n22, eq74_e1110_q_d_n23, eq74_e1110_q_d_n24, eq74_e1110_q_d_n25, eq74_e1110_q_d_n26, eq74_e1110_q_d_n27, eq74_e1110_q_d_n28, eq74_e1110_q_d_n29,) = {
    if s.b[907] {
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
            nodes,
            &eq74_reactive_node_derivatives,
            branches,
            &eq74_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq76_e1124, eq76_e1124_d_n0, eq76_e1124_d_n1, eq76_e1124_d_n2, eq76_e1124_d_n3, eq76_e1124_d_n4, eq76_e1124_d_n5, eq76_e1124_d_n6, eq76_e1124_d_n7, eq76_e1124_d_n8, eq76_e1124_d_n9, eq76_e1124_d_n10, eq76_e1124_d_n11, eq76_e1124_d_n12, eq76_e1124_d_n13, eq76_e1124_d_n14, eq76_e1124_d_n15, eq76_e1124_d_n16, eq76_e1124_d_n17, eq76_e1124_d_n18, eq76_e1124_d_n19, eq76_e1124_d_n20, eq76_e1124_d_n21, eq76_e1124_d_n22, eq76_e1124_d_n23, eq76_e1124_d_n24, eq76_e1124_d_n25, eq76_e1124_d_n26, eq76_e1124_d_n27, eq76_e1124_d_n28, eq76_e1124_d_n29, eq76_e1124_q, eq76_e1124_q_d_n0, eq76_e1124_q_d_n1, eq76_e1124_q_d_n2, eq76_e1124_q_d_n3, eq76_e1124_q_d_n4, eq76_e1124_q_d_n5, eq76_e1124_q_d_n6, eq76_e1124_q_d_n7, eq76_e1124_q_d_n8, eq76_e1124_q_d_n9, eq76_e1124_q_d_n10, eq76_e1124_q_d_n11, eq76_e1124_q_d_n12, eq76_e1124_q_d_n13, eq76_e1124_q_d_n14, eq76_e1124_q_d_n15, eq76_e1124_q_d_n16, eq76_e1124_q_d_n17, eq76_e1124_q_d_n18, eq76_e1124_q_d_n19, eq76_e1124_q_d_n20, eq76_e1124_q_d_n21, eq76_e1124_q_d_n22, eq76_e1124_q_d_n23, eq76_e1124_q_d_n24, eq76_e1124_q_d_n25, eq76_e1124_q_d_n26, eq76_e1124_q_d_n27, eq76_e1124_q_d_n28, eq76_e1124_q_d_n29,) = {
    if s.b[907] {
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
            nodes,
            &eq76_reactive_node_derivatives,
            branches,
            &eq76_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq77_e1135, eq77_e1135_d_n0, eq77_e1135_d_n1, eq77_e1135_d_n2, eq77_e1135_d_n3, eq77_e1135_d_n4, eq77_e1135_d_n5, eq77_e1135_d_n6, eq77_e1135_d_n7, eq77_e1135_d_n8, eq77_e1135_d_n9, eq77_e1135_d_n10, eq77_e1135_d_n11, eq77_e1135_d_n12, eq77_e1135_d_n13, eq77_e1135_d_n14, eq77_e1135_d_n15, eq77_e1135_d_n16, eq77_e1135_d_n17, eq77_e1135_d_n18, eq77_e1135_d_n19, eq77_e1135_d_n20, eq77_e1135_d_n21, eq77_e1135_d_n22, eq77_e1135_d_n23, eq77_e1135_d_n24, eq77_e1135_d_n25, eq77_e1135_d_n26, eq77_e1135_d_n27, eq77_e1135_d_n28, eq77_e1135_d_n29, eq77_e1135_q, eq77_e1135_q_d_n0, eq77_e1135_q_d_n1, eq77_e1135_q_d_n2, eq77_e1135_q_d_n3, eq77_e1135_q_d_n4, eq77_e1135_q_d_n5, eq77_e1135_q_d_n6, eq77_e1135_q_d_n7, eq77_e1135_q_d_n8, eq77_e1135_q_d_n9, eq77_e1135_q_d_n10, eq77_e1135_q_d_n11, eq77_e1135_q_d_n12, eq77_e1135_q_d_n13, eq77_e1135_q_d_n14, eq77_e1135_q_d_n15, eq77_e1135_q_d_n16, eq77_e1135_q_d_n17, eq77_e1135_q_d_n18, eq77_e1135_q_d_n19, eq77_e1135_q_d_n20, eq77_e1135_q_d_n21, eq77_e1135_q_d_n22, eq77_e1135_q_d_n23, eq77_e1135_q_d_n24, eq77_e1135_q_d_n25, eq77_e1135_q_d_n26, eq77_e1135_q_d_n27, eq77_e1135_q_d_n28, eq77_e1135_q_d_n29,) = {
    if (!s.b[907]) {
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
            nodes,
            &eq77_reactive_node_derivatives,
            branches,
            &eq77_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq78_e1146, eq78_e1146_d_n0, eq78_e1146_d_n1, eq78_e1146_d_n2, eq78_e1146_d_n3, eq78_e1146_d_n4, eq78_e1146_d_n5, eq78_e1146_d_n6, eq78_e1146_d_n7, eq78_e1146_d_n8, eq78_e1146_d_n9, eq78_e1146_d_n10, eq78_e1146_d_n11, eq78_e1146_d_n12, eq78_e1146_d_n13, eq78_e1146_d_n14, eq78_e1146_d_n15, eq78_e1146_d_n16, eq78_e1146_d_n17, eq78_e1146_d_n18, eq78_e1146_d_n19, eq78_e1146_d_n20, eq78_e1146_d_n21, eq78_e1146_d_n22, eq78_e1146_d_n23, eq78_e1146_d_n24, eq78_e1146_d_n25, eq78_e1146_d_n26, eq78_e1146_d_n27, eq78_e1146_d_n28, eq78_e1146_d_n29, eq78_e1146_q, eq78_e1146_q_d_n0, eq78_e1146_q_d_n1, eq78_e1146_q_d_n2, eq78_e1146_q_d_n3, eq78_e1146_q_d_n4, eq78_e1146_q_d_n5, eq78_e1146_q_d_n6, eq78_e1146_q_d_n7, eq78_e1146_q_d_n8, eq78_e1146_q_d_n9, eq78_e1146_q_d_n10, eq78_e1146_q_d_n11, eq78_e1146_q_d_n12, eq78_e1146_q_d_n13, eq78_e1146_q_d_n14, eq78_e1146_q_d_n15, eq78_e1146_q_d_n16, eq78_e1146_q_d_n17, eq78_e1146_q_d_n18, eq78_e1146_q_d_n19, eq78_e1146_q_d_n20, eq78_e1146_q_d_n21, eq78_e1146_q_d_n22, eq78_e1146_q_d_n23, eq78_e1146_q_d_n24, eq78_e1146_q_d_n25, eq78_e1146_q_d_n26, eq78_e1146_q_d_n27, eq78_e1146_q_d_n28, eq78_e1146_q_d_n29,) = {
    if (!s.b[907]) {
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
            nodes,
            &eq78_reactive_node_derivatives,
            branches,
            &eq78_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq79_e1157, eq79_e1157_d_n0, eq79_e1157_d_n1, eq79_e1157_d_n2, eq79_e1157_d_n3, eq79_e1157_d_n4, eq79_e1157_d_n5, eq79_e1157_d_n6, eq79_e1157_d_n7, eq79_e1157_d_n8, eq79_e1157_d_n9, eq79_e1157_d_n10, eq79_e1157_d_n11, eq79_e1157_d_n12, eq79_e1157_d_n13, eq79_e1157_d_n14, eq79_e1157_d_n15, eq79_e1157_d_n16, eq79_e1157_d_n17, eq79_e1157_d_n18, eq79_e1157_d_n19, eq79_e1157_d_n20, eq79_e1157_d_n21, eq79_e1157_d_n22, eq79_e1157_d_n23, eq79_e1157_d_n24, eq79_e1157_d_n25, eq79_e1157_d_n26, eq79_e1157_d_n27, eq79_e1157_d_n28, eq79_e1157_d_n29, eq79_e1157_q, eq79_e1157_q_d_n0, eq79_e1157_q_d_n1, eq79_e1157_q_d_n2, eq79_e1157_q_d_n3, eq79_e1157_q_d_n4, eq79_e1157_q_d_n5, eq79_e1157_q_d_n6, eq79_e1157_q_d_n7, eq79_e1157_q_d_n8, eq79_e1157_q_d_n9, eq79_e1157_q_d_n10, eq79_e1157_q_d_n11, eq79_e1157_q_d_n12, eq79_e1157_q_d_n13, eq79_e1157_q_d_n14, eq79_e1157_q_d_n15, eq79_e1157_q_d_n16, eq79_e1157_q_d_n17, eq79_e1157_q_d_n18, eq79_e1157_q_d_n19, eq79_e1157_q_d_n20, eq79_e1157_q_d_n21, eq79_e1157_q_d_n22, eq79_e1157_q_d_n23, eq79_e1157_q_d_n24, eq79_e1157_q_d_n25, eq79_e1157_q_d_n26, eq79_e1157_q_d_n27, eq79_e1157_q_d_n28, eq79_e1157_q_d_n29,) = {
    if (!s.b[907]) {
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
            nodes,
            &eq79_reactive_node_derivatives,
            branches,
            &eq79_reactive_branch_derivatives,
            multiplicity,
        );
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
            nodes,
            &eq82_reactive_node_derivatives,
            branches,
            &eq82_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq85_e1197, eq85_e1197_d_n0, eq85_e1197_d_n1, eq85_e1197_d_n2, eq85_e1197_d_n3, eq85_e1197_d_n4, eq85_e1197_d_n5, eq85_e1197_d_n6, eq85_e1197_d_n7, eq85_e1197_d_n8, eq85_e1197_d_n9, eq85_e1197_d_n10, eq85_e1197_d_n11, eq85_e1197_d_n12, eq85_e1197_d_n13, eq85_e1197_d_n14, eq85_e1197_d_n15, eq85_e1197_d_n16, eq85_e1197_d_n17, eq85_e1197_d_n18, eq85_e1197_d_n19, eq85_e1197_d_n20, eq85_e1197_d_n21, eq85_e1197_d_n22, eq85_e1197_d_n23, eq85_e1197_d_n24, eq85_e1197_d_n25, eq85_e1197_d_n26, eq85_e1197_d_n27, eq85_e1197_d_n28, eq85_e1197_d_n29, eq85_e1197_q, eq85_e1197_q_d_n0, eq85_e1197_q_d_n1, eq85_e1197_q_d_n2, eq85_e1197_q_d_n3, eq85_e1197_q_d_n4, eq85_e1197_q_d_n5, eq85_e1197_q_d_n6, eq85_e1197_q_d_n7, eq85_e1197_q_d_n8, eq85_e1197_q_d_n9, eq85_e1197_q_d_n10, eq85_e1197_q_d_n11, eq85_e1197_q_d_n12, eq85_e1197_q_d_n13, eq85_e1197_q_d_n14, eq85_e1197_q_d_n15, eq85_e1197_q_d_n16, eq85_e1197_q_d_n17, eq85_e1197_q_d_n18, eq85_e1197_q_d_n19, eq85_e1197_q_d_n20, eq85_e1197_q_d_n21, eq85_e1197_q_d_n22, eq85_e1197_q_d_n23, eq85_e1197_q_d_n24, eq85_e1197_q_d_n25, eq85_e1197_q_d_n26, eq85_e1197_q_d_n27, eq85_e1197_q_d_n28, eq85_e1197_q_d_n29,) = {
    if s.b[1054] {
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
            nodes,
            &eq85_reactive_node_derivatives,
            branches,
            &eq85_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq86_e1207, eq86_e1207_d_n0, eq86_e1207_d_n1, eq86_e1207_d_n2, eq86_e1207_d_n3, eq86_e1207_d_n4, eq86_e1207_d_n5, eq86_e1207_d_n6, eq86_e1207_d_n7, eq86_e1207_d_n8, eq86_e1207_d_n9, eq86_e1207_d_n10, eq86_e1207_d_n11, eq86_e1207_d_n12, eq86_e1207_d_n13, eq86_e1207_d_n14, eq86_e1207_d_n15, eq86_e1207_d_n16, eq86_e1207_d_n17, eq86_e1207_d_n18, eq86_e1207_d_n19, eq86_e1207_d_n20, eq86_e1207_d_n21, eq86_e1207_d_n22, eq86_e1207_d_n23, eq86_e1207_d_n24, eq86_e1207_d_n25, eq86_e1207_d_n26, eq86_e1207_d_n27, eq86_e1207_d_n28, eq86_e1207_d_n29, eq86_e1207_q, eq86_e1207_q_d_n0, eq86_e1207_q_d_n1, eq86_e1207_q_d_n2, eq86_e1207_q_d_n3, eq86_e1207_q_d_n4, eq86_e1207_q_d_n5, eq86_e1207_q_d_n6, eq86_e1207_q_d_n7, eq86_e1207_q_d_n8, eq86_e1207_q_d_n9, eq86_e1207_q_d_n10, eq86_e1207_q_d_n11, eq86_e1207_q_d_n12, eq86_e1207_q_d_n13, eq86_e1207_q_d_n14, eq86_e1207_q_d_n15, eq86_e1207_q_d_n16, eq86_e1207_q_d_n17, eq86_e1207_q_d_n18, eq86_e1207_q_d_n19, eq86_e1207_q_d_n20, eq86_e1207_q_d_n21, eq86_e1207_q_d_n22, eq86_e1207_q_d_n23, eq86_e1207_q_d_n24, eq86_e1207_q_d_n25, eq86_e1207_q_d_n26, eq86_e1207_q_d_n27, eq86_e1207_q_d_n28, eq86_e1207_q_d_n29,) = {
    if s.b[1054] {
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
            nodes,
            &eq86_reactive_node_derivatives,
            branches,
            &eq86_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq87_e1217, eq87_e1217_d_n0, eq87_e1217_d_n1, eq87_e1217_d_n2, eq87_e1217_d_n3, eq87_e1217_d_n4, eq87_e1217_d_n5, eq87_e1217_d_n6, eq87_e1217_d_n7, eq87_e1217_d_n8, eq87_e1217_d_n9, eq87_e1217_d_n10, eq87_e1217_d_n11, eq87_e1217_d_n12, eq87_e1217_d_n13, eq87_e1217_d_n14, eq87_e1217_d_n15, eq87_e1217_d_n16, eq87_e1217_d_n17, eq87_e1217_d_n18, eq87_e1217_d_n19, eq87_e1217_d_n20, eq87_e1217_d_n21, eq87_e1217_d_n22, eq87_e1217_d_n23, eq87_e1217_d_n24, eq87_e1217_d_n25, eq87_e1217_d_n26, eq87_e1217_d_n27, eq87_e1217_d_n28, eq87_e1217_d_n29, eq87_e1217_q, eq87_e1217_q_d_n0, eq87_e1217_q_d_n1, eq87_e1217_q_d_n2, eq87_e1217_q_d_n3, eq87_e1217_q_d_n4, eq87_e1217_q_d_n5, eq87_e1217_q_d_n6, eq87_e1217_q_d_n7, eq87_e1217_q_d_n8, eq87_e1217_q_d_n9, eq87_e1217_q_d_n10, eq87_e1217_q_d_n11, eq87_e1217_q_d_n12, eq87_e1217_q_d_n13, eq87_e1217_q_d_n14, eq87_e1217_q_d_n15, eq87_e1217_q_d_n16, eq87_e1217_q_d_n17, eq87_e1217_q_d_n18, eq87_e1217_q_d_n19, eq87_e1217_q_d_n20, eq87_e1217_q_d_n21, eq87_e1217_q_d_n22, eq87_e1217_q_d_n23, eq87_e1217_q_d_n24, eq87_e1217_q_d_n25, eq87_e1217_q_d_n26, eq87_e1217_q_d_n27, eq87_e1217_q_d_n28, eq87_e1217_q_d_n29,) = {
    if s.b[1054] {
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
            nodes,
            &eq87_reactive_node_derivatives,
            branches,
            &eq87_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq89_e1231, eq89_e1231_d_n0, eq89_e1231_d_n1, eq89_e1231_d_n2, eq89_e1231_d_n3, eq89_e1231_d_n4, eq89_e1231_d_n5, eq89_e1231_d_n6, eq89_e1231_d_n7, eq89_e1231_d_n8, eq89_e1231_d_n9, eq89_e1231_d_n10, eq89_e1231_d_n11, eq89_e1231_d_n12, eq89_e1231_d_n13, eq89_e1231_d_n14, eq89_e1231_d_n15, eq89_e1231_d_n16, eq89_e1231_d_n17, eq89_e1231_d_n18, eq89_e1231_d_n19, eq89_e1231_d_n20, eq89_e1231_d_n21, eq89_e1231_d_n22, eq89_e1231_d_n23, eq89_e1231_d_n24, eq89_e1231_d_n25, eq89_e1231_d_n26, eq89_e1231_d_n27, eq89_e1231_d_n28, eq89_e1231_d_n29, eq89_e1231_q, eq89_e1231_q_d_n0, eq89_e1231_q_d_n1, eq89_e1231_q_d_n2, eq89_e1231_q_d_n3, eq89_e1231_q_d_n4, eq89_e1231_q_d_n5, eq89_e1231_q_d_n6, eq89_e1231_q_d_n7, eq89_e1231_q_d_n8, eq89_e1231_q_d_n9, eq89_e1231_q_d_n10, eq89_e1231_q_d_n11, eq89_e1231_q_d_n12, eq89_e1231_q_d_n13, eq89_e1231_q_d_n14, eq89_e1231_q_d_n15, eq89_e1231_q_d_n16, eq89_e1231_q_d_n17, eq89_e1231_q_d_n18, eq89_e1231_q_d_n19, eq89_e1231_q_d_n20, eq89_e1231_q_d_n21, eq89_e1231_q_d_n22, eq89_e1231_q_d_n23, eq89_e1231_q_d_n24, eq89_e1231_q_d_n25, eq89_e1231_q_d_n26, eq89_e1231_q_d_n27, eq89_e1231_q_d_n28, eq89_e1231_q_d_n29,) = {
    if s.b[1054] {
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
            nodes,
            &eq89_reactive_node_derivatives,
            branches,
            &eq89_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq90_e1242, eq90_e1242_d_n0, eq90_e1242_d_n1, eq90_e1242_d_n2, eq90_e1242_d_n3, eq90_e1242_d_n4, eq90_e1242_d_n5, eq90_e1242_d_n6, eq90_e1242_d_n7, eq90_e1242_d_n8, eq90_e1242_d_n9, eq90_e1242_d_n10, eq90_e1242_d_n11, eq90_e1242_d_n12, eq90_e1242_d_n13, eq90_e1242_d_n14, eq90_e1242_d_n15, eq90_e1242_d_n16, eq90_e1242_d_n17, eq90_e1242_d_n18, eq90_e1242_d_n19, eq90_e1242_d_n20, eq90_e1242_d_n21, eq90_e1242_d_n22, eq90_e1242_d_n23, eq90_e1242_d_n24, eq90_e1242_d_n25, eq90_e1242_d_n26, eq90_e1242_d_n27, eq90_e1242_d_n28, eq90_e1242_d_n29, eq90_e1242_q, eq90_e1242_q_d_n0, eq90_e1242_q_d_n1, eq90_e1242_q_d_n2, eq90_e1242_q_d_n3, eq90_e1242_q_d_n4, eq90_e1242_q_d_n5, eq90_e1242_q_d_n6, eq90_e1242_q_d_n7, eq90_e1242_q_d_n8, eq90_e1242_q_d_n9, eq90_e1242_q_d_n10, eq90_e1242_q_d_n11, eq90_e1242_q_d_n12, eq90_e1242_q_d_n13, eq90_e1242_q_d_n14, eq90_e1242_q_d_n15, eq90_e1242_q_d_n16, eq90_e1242_q_d_n17, eq90_e1242_q_d_n18, eq90_e1242_q_d_n19, eq90_e1242_q_d_n20, eq90_e1242_q_d_n21, eq90_e1242_q_d_n22, eq90_e1242_q_d_n23, eq90_e1242_q_d_n24, eq90_e1242_q_d_n25, eq90_e1242_q_d_n26, eq90_e1242_q_d_n27, eq90_e1242_q_d_n28, eq90_e1242_q_d_n29,) = {
    if (!s.b[1054]) {
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
            nodes,
            &eq90_reactive_node_derivatives,
            branches,
            &eq90_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq91_e1253, eq91_e1253_d_n0, eq91_e1253_d_n1, eq91_e1253_d_n2, eq91_e1253_d_n3, eq91_e1253_d_n4, eq91_e1253_d_n5, eq91_e1253_d_n6, eq91_e1253_d_n7, eq91_e1253_d_n8, eq91_e1253_d_n9, eq91_e1253_d_n10, eq91_e1253_d_n11, eq91_e1253_d_n12, eq91_e1253_d_n13, eq91_e1253_d_n14, eq91_e1253_d_n15, eq91_e1253_d_n16, eq91_e1253_d_n17, eq91_e1253_d_n18, eq91_e1253_d_n19, eq91_e1253_d_n20, eq91_e1253_d_n21, eq91_e1253_d_n22, eq91_e1253_d_n23, eq91_e1253_d_n24, eq91_e1253_d_n25, eq91_e1253_d_n26, eq91_e1253_d_n27, eq91_e1253_d_n28, eq91_e1253_d_n29, eq91_e1253_q, eq91_e1253_q_d_n0, eq91_e1253_q_d_n1, eq91_e1253_q_d_n2, eq91_e1253_q_d_n3, eq91_e1253_q_d_n4, eq91_e1253_q_d_n5, eq91_e1253_q_d_n6, eq91_e1253_q_d_n7, eq91_e1253_q_d_n8, eq91_e1253_q_d_n9, eq91_e1253_q_d_n10, eq91_e1253_q_d_n11, eq91_e1253_q_d_n12, eq91_e1253_q_d_n13, eq91_e1253_q_d_n14, eq91_e1253_q_d_n15, eq91_e1253_q_d_n16, eq91_e1253_q_d_n17, eq91_e1253_q_d_n18, eq91_e1253_q_d_n19, eq91_e1253_q_d_n20, eq91_e1253_q_d_n21, eq91_e1253_q_d_n22, eq91_e1253_q_d_n23, eq91_e1253_q_d_n24, eq91_e1253_q_d_n25, eq91_e1253_q_d_n26, eq91_e1253_q_d_n27, eq91_e1253_q_d_n28, eq91_e1253_q_d_n29,) = {
    if (!s.b[1054]) {
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
            nodes,
            &eq91_reactive_node_derivatives,
            branches,
            &eq91_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq92_e1264, eq92_e1264_d_n0, eq92_e1264_d_n1, eq92_e1264_d_n2, eq92_e1264_d_n3, eq92_e1264_d_n4, eq92_e1264_d_n5, eq92_e1264_d_n6, eq92_e1264_d_n7, eq92_e1264_d_n8, eq92_e1264_d_n9, eq92_e1264_d_n10, eq92_e1264_d_n11, eq92_e1264_d_n12, eq92_e1264_d_n13, eq92_e1264_d_n14, eq92_e1264_d_n15, eq92_e1264_d_n16, eq92_e1264_d_n17, eq92_e1264_d_n18, eq92_e1264_d_n19, eq92_e1264_d_n20, eq92_e1264_d_n21, eq92_e1264_d_n22, eq92_e1264_d_n23, eq92_e1264_d_n24, eq92_e1264_d_n25, eq92_e1264_d_n26, eq92_e1264_d_n27, eq92_e1264_d_n28, eq92_e1264_d_n29, eq92_e1264_q, eq92_e1264_q_d_n0, eq92_e1264_q_d_n1, eq92_e1264_q_d_n2, eq92_e1264_q_d_n3, eq92_e1264_q_d_n4, eq92_e1264_q_d_n5, eq92_e1264_q_d_n6, eq92_e1264_q_d_n7, eq92_e1264_q_d_n8, eq92_e1264_q_d_n9, eq92_e1264_q_d_n10, eq92_e1264_q_d_n11, eq92_e1264_q_d_n12, eq92_e1264_q_d_n13, eq92_e1264_q_d_n14, eq92_e1264_q_d_n15, eq92_e1264_q_d_n16, eq92_e1264_q_d_n17, eq92_e1264_q_d_n18, eq92_e1264_q_d_n19, eq92_e1264_q_d_n20, eq92_e1264_q_d_n21, eq92_e1264_q_d_n22, eq92_e1264_q_d_n23, eq92_e1264_q_d_n24, eq92_e1264_q_d_n25, eq92_e1264_q_d_n26, eq92_e1264_q_d_n27, eq92_e1264_q_d_n28, eq92_e1264_q_d_n29,) = {
    if (!s.b[1054]) {
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
            nodes,
            &eq92_reactive_node_derivatives,
            branches,
            &eq92_reactive_branch_derivatives,
            multiplicity,
        );
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
            nodes,
            &eq95_reactive_node_derivatives,
            branches,
            &eq95_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq98_e1304, eq98_e1304_d_n0, eq98_e1304_d_n1, eq98_e1304_d_n2, eq98_e1304_d_n3, eq98_e1304_d_n4, eq98_e1304_d_n5, eq98_e1304_d_n6, eq98_e1304_d_n7, eq98_e1304_d_n8, eq98_e1304_d_n9, eq98_e1304_d_n10, eq98_e1304_d_n11, eq98_e1304_d_n12, eq98_e1304_d_n13, eq98_e1304_d_n14, eq98_e1304_d_n15, eq98_e1304_d_n16, eq98_e1304_d_n17, eq98_e1304_d_n18, eq98_e1304_d_n19, eq98_e1304_d_n20, eq98_e1304_d_n21, eq98_e1304_d_n22, eq98_e1304_d_n23, eq98_e1304_d_n24, eq98_e1304_d_n25, eq98_e1304_d_n26, eq98_e1304_d_n27, eq98_e1304_d_n28, eq98_e1304_d_n29, eq98_e1304_q, eq98_e1304_q_d_n0, eq98_e1304_q_d_n1, eq98_e1304_q_d_n2, eq98_e1304_q_d_n3, eq98_e1304_q_d_n4, eq98_e1304_q_d_n5, eq98_e1304_q_d_n6, eq98_e1304_q_d_n7, eq98_e1304_q_d_n8, eq98_e1304_q_d_n9, eq98_e1304_q_d_n10, eq98_e1304_q_d_n11, eq98_e1304_q_d_n12, eq98_e1304_q_d_n13, eq98_e1304_q_d_n14, eq98_e1304_q_d_n15, eq98_e1304_q_d_n16, eq98_e1304_q_d_n17, eq98_e1304_q_d_n18, eq98_e1304_q_d_n19, eq98_e1304_q_d_n20, eq98_e1304_q_d_n21, eq98_e1304_q_d_n22, eq98_e1304_q_d_n23, eq98_e1304_q_d_n24, eq98_e1304_q_d_n25, eq98_e1304_q_d_n26, eq98_e1304_q_d_n27, eq98_e1304_q_d_n28, eq98_e1304_q_d_n29,) = {
    if s.b[1201] {
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
            nodes,
            &eq98_reactive_node_derivatives,
            branches,
            &eq98_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq99_e1314, eq99_e1314_d_n0, eq99_e1314_d_n1, eq99_e1314_d_n2, eq99_e1314_d_n3, eq99_e1314_d_n4, eq99_e1314_d_n5, eq99_e1314_d_n6, eq99_e1314_d_n7, eq99_e1314_d_n8, eq99_e1314_d_n9, eq99_e1314_d_n10, eq99_e1314_d_n11, eq99_e1314_d_n12, eq99_e1314_d_n13, eq99_e1314_d_n14, eq99_e1314_d_n15, eq99_e1314_d_n16, eq99_e1314_d_n17, eq99_e1314_d_n18, eq99_e1314_d_n19, eq99_e1314_d_n20, eq99_e1314_d_n21, eq99_e1314_d_n22, eq99_e1314_d_n23, eq99_e1314_d_n24, eq99_e1314_d_n25, eq99_e1314_d_n26, eq99_e1314_d_n27, eq99_e1314_d_n28, eq99_e1314_d_n29, eq99_e1314_q, eq99_e1314_q_d_n0, eq99_e1314_q_d_n1, eq99_e1314_q_d_n2, eq99_e1314_q_d_n3, eq99_e1314_q_d_n4, eq99_e1314_q_d_n5, eq99_e1314_q_d_n6, eq99_e1314_q_d_n7, eq99_e1314_q_d_n8, eq99_e1314_q_d_n9, eq99_e1314_q_d_n10, eq99_e1314_q_d_n11, eq99_e1314_q_d_n12, eq99_e1314_q_d_n13, eq99_e1314_q_d_n14, eq99_e1314_q_d_n15, eq99_e1314_q_d_n16, eq99_e1314_q_d_n17, eq99_e1314_q_d_n18, eq99_e1314_q_d_n19, eq99_e1314_q_d_n20, eq99_e1314_q_d_n21, eq99_e1314_q_d_n22, eq99_e1314_q_d_n23, eq99_e1314_q_d_n24, eq99_e1314_q_d_n25, eq99_e1314_q_d_n26, eq99_e1314_q_d_n27, eq99_e1314_q_d_n28, eq99_e1314_q_d_n29,) = {
    if s.b[1201] {
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
            nodes,
            &eq99_reactive_node_derivatives,
            branches,
            &eq99_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq100_e1324, eq100_e1324_d_n0, eq100_e1324_d_n1, eq100_e1324_d_n2, eq100_e1324_d_n3, eq100_e1324_d_n4, eq100_e1324_d_n5, eq100_e1324_d_n6, eq100_e1324_d_n7, eq100_e1324_d_n8, eq100_e1324_d_n9, eq100_e1324_d_n10, eq100_e1324_d_n11, eq100_e1324_d_n12, eq100_e1324_d_n13, eq100_e1324_d_n14, eq100_e1324_d_n15, eq100_e1324_d_n16, eq100_e1324_d_n17, eq100_e1324_d_n18, eq100_e1324_d_n19, eq100_e1324_d_n20, eq100_e1324_d_n21, eq100_e1324_d_n22, eq100_e1324_d_n23, eq100_e1324_d_n24, eq100_e1324_d_n25, eq100_e1324_d_n26, eq100_e1324_d_n27, eq100_e1324_d_n28, eq100_e1324_d_n29, eq100_e1324_q, eq100_e1324_q_d_n0, eq100_e1324_q_d_n1, eq100_e1324_q_d_n2, eq100_e1324_q_d_n3, eq100_e1324_q_d_n4, eq100_e1324_q_d_n5, eq100_e1324_q_d_n6, eq100_e1324_q_d_n7, eq100_e1324_q_d_n8, eq100_e1324_q_d_n9, eq100_e1324_q_d_n10, eq100_e1324_q_d_n11, eq100_e1324_q_d_n12, eq100_e1324_q_d_n13, eq100_e1324_q_d_n14, eq100_e1324_q_d_n15, eq100_e1324_q_d_n16, eq100_e1324_q_d_n17, eq100_e1324_q_d_n18, eq100_e1324_q_d_n19, eq100_e1324_q_d_n20, eq100_e1324_q_d_n21, eq100_e1324_q_d_n22, eq100_e1324_q_d_n23, eq100_e1324_q_d_n24, eq100_e1324_q_d_n25, eq100_e1324_q_d_n26, eq100_e1324_q_d_n27, eq100_e1324_q_d_n28, eq100_e1324_q_d_n29,) = {
    if s.b[1201] {
        let eq100_e1317_q: f64 = s.v[175];
        let eq100_e1320: f64 = (p.p355 * (nv2 - nv11));
        let eq100_e1320_d_n2: f64 = p.p355;
        let eq100_e1320_d_n11: f64 = (-p.p355);
        let eq100_e1321_q: f64 = eq100_e1320;
        let eq100_e1322: f64 = (s.v[175] + eq100_e1320);
        let eq100_e1322_d_n2: f64 = (s.dn[175][2] + eq100_e1320_d_n2);
        let eq100_e1322_d_n11: f64 = (s.dn[175][11] + eq100_e1320_d_n11);
        let eq100_e1322_q: f64 = (eq100_e1317_q + eq100_e1321_q);
        let eq100_e1322_q_d_n2: f64 = (s.dn[175][2] + eq100_e1320_d_n2);
        let eq100_e1322_q_d_n11: f64 = (s.dn[175][11] + eq100_e1320_d_n11);
        (eq100_e1322, s.dn[175][0], s.dn[175][1], eq100_e1322_d_n2, s.dn[175][3], s.dn[175][4], s.dn[175][5], s.dn[175][6], s.dn[175][7], s.dn[175][8], s.dn[175][9], s.dn[175][10], eq100_e1322_d_n11, s.dn[175][12], s.dn[175][13], s.dn[175][14], s.dn[175][15], s.dn[175][16], s.dn[175][17], s.dn[175][18], s.dn[175][19], s.dn[175][20], s.dn[175][21], s.dn[175][22], s.dn[175][23], s.dn[175][24], s.dn[175][25], s.dn[175][26], s.dn[175][27], s.dn[175][28], s.dn[175][29], eq100_e1322_q, s.dn[175][0], s.dn[175][1], eq100_e1322_q_d_n2, s.dn[175][3], s.dn[175][4], s.dn[175][5], s.dn[175][6], s.dn[175][7], s.dn[175][8], s.dn[175][9], s.dn[175][10], eq100_e1322_q_d_n11, s.dn[175][12], s.dn[175][13], s.dn[175][14], s.dn[175][15], s.dn[175][16], s.dn[175][17], s.dn[175][18], s.dn[175][19], s.dn[175][20], s.dn[175][21], s.dn[175][22], s.dn[175][23], s.dn[175][24], s.dn[175][25], s.dn[175][26], s.dn[175][27], s.dn[175][28], s.dn[175][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq100_reactive_node_derivatives: [f64; 30] = [eq100_e1324_q_d_n0, eq100_e1324_q_d_n1, eq100_e1324_q_d_n2, eq100_e1324_q_d_n3, eq100_e1324_q_d_n4, eq100_e1324_q_d_n5, eq100_e1324_q_d_n6, eq100_e1324_q_d_n7, eq100_e1324_q_d_n8, eq100_e1324_q_d_n9, eq100_e1324_q_d_n10, eq100_e1324_q_d_n11, eq100_e1324_q_d_n12, eq100_e1324_q_d_n13, eq100_e1324_q_d_n14, eq100_e1324_q_d_n15, eq100_e1324_q_d_n16, eq100_e1324_q_d_n17, eq100_e1324_q_d_n18, eq100_e1324_q_d_n19, eq100_e1324_q_d_n20, eq100_e1324_q_d_n21, eq100_e1324_q_d_n22, eq100_e1324_q_d_n23, eq100_e1324_q_d_n24, eq100_e1324_q_d_n25, eq100_e1324_q_d_n26, eq100_e1324_q_d_n27, eq100_e1324_q_d_n28, eq100_e1324_q_d_n29];
        let eq100_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[11]),
            nodes,
            &eq100_reactive_node_derivatives,
            branches,
            &eq100_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq102_e1338, eq102_e1338_d_n0, eq102_e1338_d_n1, eq102_e1338_d_n2, eq102_e1338_d_n3, eq102_e1338_d_n4, eq102_e1338_d_n5, eq102_e1338_d_n6, eq102_e1338_d_n7, eq102_e1338_d_n8, eq102_e1338_d_n9, eq102_e1338_d_n10, eq102_e1338_d_n11, eq102_e1338_d_n12, eq102_e1338_d_n13, eq102_e1338_d_n14, eq102_e1338_d_n15, eq102_e1338_d_n16, eq102_e1338_d_n17, eq102_e1338_d_n18, eq102_e1338_d_n19, eq102_e1338_d_n20, eq102_e1338_d_n21, eq102_e1338_d_n22, eq102_e1338_d_n23, eq102_e1338_d_n24, eq102_e1338_d_n25, eq102_e1338_d_n26, eq102_e1338_d_n27, eq102_e1338_d_n28, eq102_e1338_d_n29, eq102_e1338_q, eq102_e1338_q_d_n0, eq102_e1338_q_d_n1, eq102_e1338_q_d_n2, eq102_e1338_q_d_n3, eq102_e1338_q_d_n4, eq102_e1338_q_d_n5, eq102_e1338_q_d_n6, eq102_e1338_q_d_n7, eq102_e1338_q_d_n8, eq102_e1338_q_d_n9, eq102_e1338_q_d_n10, eq102_e1338_q_d_n11, eq102_e1338_q_d_n12, eq102_e1338_q_d_n13, eq102_e1338_q_d_n14, eq102_e1338_q_d_n15, eq102_e1338_q_d_n16, eq102_e1338_q_d_n17, eq102_e1338_q_d_n18, eq102_e1338_q_d_n19, eq102_e1338_q_d_n20, eq102_e1338_q_d_n21, eq102_e1338_q_d_n22, eq102_e1338_q_d_n23, eq102_e1338_q_d_n24, eq102_e1338_q_d_n25, eq102_e1338_q_d_n26, eq102_e1338_q_d_n27, eq102_e1338_q_d_n28, eq102_e1338_q_d_n29,) = {
    if s.b[1201] {
        let eq102_e1331_q: f64 = s.v[177];
        let eq102_e1334: f64 = (p.p355 * (nv7 - nv9));
        let eq102_e1334_d_n7: f64 = p.p355;
        let eq102_e1334_d_n9: f64 = (-p.p355);
        let eq102_e1335_q: f64 = eq102_e1334;
        let eq102_e1336: f64 = (s.v[177] + eq102_e1334);
        let eq102_e1336_d_n7: f64 = (s.dn[177][7] + eq102_e1334_d_n7);
        let eq102_e1336_d_n9: f64 = (s.dn[177][9] + eq102_e1334_d_n9);
        let eq102_e1336_q: f64 = (eq102_e1331_q + eq102_e1335_q);
        let eq102_e1336_q_d_n7: f64 = (s.dn[177][7] + eq102_e1334_d_n7);
        let eq102_e1336_q_d_n9: f64 = (s.dn[177][9] + eq102_e1334_d_n9);
        (eq102_e1336, s.dn[177][0], s.dn[177][1], s.dn[177][2], s.dn[177][3], s.dn[177][4], s.dn[177][5], s.dn[177][6], eq102_e1336_d_n7, s.dn[177][8], eq102_e1336_d_n9, s.dn[177][10], s.dn[177][11], s.dn[177][12], s.dn[177][13], s.dn[177][14], s.dn[177][15], s.dn[177][16], s.dn[177][17], s.dn[177][18], s.dn[177][19], s.dn[177][20], s.dn[177][21], s.dn[177][22], s.dn[177][23], s.dn[177][24], s.dn[177][25], s.dn[177][26], s.dn[177][27], s.dn[177][28], s.dn[177][29], eq102_e1336_q, s.dn[177][0], s.dn[177][1], s.dn[177][2], s.dn[177][3], s.dn[177][4], s.dn[177][5], s.dn[177][6], eq102_e1336_q_d_n7, s.dn[177][8], eq102_e1336_q_d_n9, s.dn[177][10], s.dn[177][11], s.dn[177][12], s.dn[177][13], s.dn[177][14], s.dn[177][15], s.dn[177][16], s.dn[177][17], s.dn[177][18], s.dn[177][19], s.dn[177][20], s.dn[177][21], s.dn[177][22], s.dn[177][23], s.dn[177][24], s.dn[177][25], s.dn[177][26], s.dn[177][27], s.dn[177][28], s.dn[177][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq102_reactive_node_derivatives: [f64; 30] = [eq102_e1338_q_d_n0, eq102_e1338_q_d_n1, eq102_e1338_q_d_n2, eq102_e1338_q_d_n3, eq102_e1338_q_d_n4, eq102_e1338_q_d_n5, eq102_e1338_q_d_n6, eq102_e1338_q_d_n7, eq102_e1338_q_d_n8, eq102_e1338_q_d_n9, eq102_e1338_q_d_n10, eq102_e1338_q_d_n11, eq102_e1338_q_d_n12, eq102_e1338_q_d_n13, eq102_e1338_q_d_n14, eq102_e1338_q_d_n15, eq102_e1338_q_d_n16, eq102_e1338_q_d_n17, eq102_e1338_q_d_n18, eq102_e1338_q_d_n19, eq102_e1338_q_d_n20, eq102_e1338_q_d_n21, eq102_e1338_q_d_n22, eq102_e1338_q_d_n23, eq102_e1338_q_d_n24, eq102_e1338_q_d_n25, eq102_e1338_q_d_n26, eq102_e1338_q_d_n27, eq102_e1338_q_d_n28, eq102_e1338_q_d_n29];
        let eq102_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq102_reactive_node_derivatives,
            branches,
            &eq102_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq103_e1349, eq103_e1349_d_n0, eq103_e1349_d_n1, eq103_e1349_d_n2, eq103_e1349_d_n3, eq103_e1349_d_n4, eq103_e1349_d_n5, eq103_e1349_d_n6, eq103_e1349_d_n7, eq103_e1349_d_n8, eq103_e1349_d_n9, eq103_e1349_d_n10, eq103_e1349_d_n11, eq103_e1349_d_n12, eq103_e1349_d_n13, eq103_e1349_d_n14, eq103_e1349_d_n15, eq103_e1349_d_n16, eq103_e1349_d_n17, eq103_e1349_d_n18, eq103_e1349_d_n19, eq103_e1349_d_n20, eq103_e1349_d_n21, eq103_e1349_d_n22, eq103_e1349_d_n23, eq103_e1349_d_n24, eq103_e1349_d_n25, eq103_e1349_d_n26, eq103_e1349_d_n27, eq103_e1349_d_n28, eq103_e1349_d_n29, eq103_e1349_q, eq103_e1349_q_d_n0, eq103_e1349_q_d_n1, eq103_e1349_q_d_n2, eq103_e1349_q_d_n3, eq103_e1349_q_d_n4, eq103_e1349_q_d_n5, eq103_e1349_q_d_n6, eq103_e1349_q_d_n7, eq103_e1349_q_d_n8, eq103_e1349_q_d_n9, eq103_e1349_q_d_n10, eq103_e1349_q_d_n11, eq103_e1349_q_d_n12, eq103_e1349_q_d_n13, eq103_e1349_q_d_n14, eq103_e1349_q_d_n15, eq103_e1349_q_d_n16, eq103_e1349_q_d_n17, eq103_e1349_q_d_n18, eq103_e1349_q_d_n19, eq103_e1349_q_d_n20, eq103_e1349_q_d_n21, eq103_e1349_q_d_n22, eq103_e1349_q_d_n23, eq103_e1349_q_d_n24, eq103_e1349_q_d_n25, eq103_e1349_q_d_n26, eq103_e1349_q_d_n27, eq103_e1349_q_d_n28, eq103_e1349_q_d_n29,) = {
    if (!s.b[1201]) {
        let eq103_e1342_q: f64 = s.v[173];
        let eq103_e1345: f64 = (p.p355 * (nv2 - nv11));
        let eq103_e1345_d_n2: f64 = p.p355;
        let eq103_e1345_d_n11: f64 = (-p.p355);
        let eq103_e1346_q: f64 = eq103_e1345;
        let eq103_e1347: f64 = (s.v[173] + eq103_e1345);
        let eq103_e1347_d_n2: f64 = (s.dn[173][2] + eq103_e1345_d_n2);
        let eq103_e1347_d_n11: f64 = (s.dn[173][11] + eq103_e1345_d_n11);
        let eq103_e1347_q: f64 = (eq103_e1342_q + eq103_e1346_q);
        let eq103_e1347_q_d_n2: f64 = (s.dn[173][2] + eq103_e1345_d_n2);
        let eq103_e1347_q_d_n11: f64 = (s.dn[173][11] + eq103_e1345_d_n11);
        (eq103_e1347, s.dn[173][0], s.dn[173][1], eq103_e1347_d_n2, s.dn[173][3], s.dn[173][4], s.dn[173][5], s.dn[173][6], s.dn[173][7], s.dn[173][8], s.dn[173][9], s.dn[173][10], eq103_e1347_d_n11, s.dn[173][12], s.dn[173][13], s.dn[173][14], s.dn[173][15], s.dn[173][16], s.dn[173][17], s.dn[173][18], s.dn[173][19], s.dn[173][20], s.dn[173][21], s.dn[173][22], s.dn[173][23], s.dn[173][24], s.dn[173][25], s.dn[173][26], s.dn[173][27], s.dn[173][28], s.dn[173][29], eq103_e1347_q, s.dn[173][0], s.dn[173][1], eq103_e1347_q_d_n2, s.dn[173][3], s.dn[173][4], s.dn[173][5], s.dn[173][6], s.dn[173][7], s.dn[173][8], s.dn[173][9], s.dn[173][10], eq103_e1347_q_d_n11, s.dn[173][12], s.dn[173][13], s.dn[173][14], s.dn[173][15], s.dn[173][16], s.dn[173][17], s.dn[173][18], s.dn[173][19], s.dn[173][20], s.dn[173][21], s.dn[173][22], s.dn[173][23], s.dn[173][24], s.dn[173][25], s.dn[173][26], s.dn[173][27], s.dn[173][28], s.dn[173][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq103_reactive_node_derivatives: [f64; 30] = [eq103_e1349_q_d_n0, eq103_e1349_q_d_n1, eq103_e1349_q_d_n2, eq103_e1349_q_d_n3, eq103_e1349_q_d_n4, eq103_e1349_q_d_n5, eq103_e1349_q_d_n6, eq103_e1349_q_d_n7, eq103_e1349_q_d_n8, eq103_e1349_q_d_n9, eq103_e1349_q_d_n10, eq103_e1349_q_d_n11, eq103_e1349_q_d_n12, eq103_e1349_q_d_n13, eq103_e1349_q_d_n14, eq103_e1349_q_d_n15, eq103_e1349_q_d_n16, eq103_e1349_q_d_n17, eq103_e1349_q_d_n18, eq103_e1349_q_d_n19, eq103_e1349_q_d_n20, eq103_e1349_q_d_n21, eq103_e1349_q_d_n22, eq103_e1349_q_d_n23, eq103_e1349_q_d_n24, eq103_e1349_q_d_n25, eq103_e1349_q_d_n26, eq103_e1349_q_d_n27, eq103_e1349_q_d_n28, eq103_e1349_q_d_n29];
        let eq103_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[11]),
            nodes,
            &eq103_reactive_node_derivatives,
            branches,
            &eq103_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq104_e1360, eq104_e1360_d_n0, eq104_e1360_d_n1, eq104_e1360_d_n2, eq104_e1360_d_n3, eq104_e1360_d_n4, eq104_e1360_d_n5, eq104_e1360_d_n6, eq104_e1360_d_n7, eq104_e1360_d_n8, eq104_e1360_d_n9, eq104_e1360_d_n10, eq104_e1360_d_n11, eq104_e1360_d_n12, eq104_e1360_d_n13, eq104_e1360_d_n14, eq104_e1360_d_n15, eq104_e1360_d_n16, eq104_e1360_d_n17, eq104_e1360_d_n18, eq104_e1360_d_n19, eq104_e1360_d_n20, eq104_e1360_d_n21, eq104_e1360_d_n22, eq104_e1360_d_n23, eq104_e1360_d_n24, eq104_e1360_d_n25, eq104_e1360_d_n26, eq104_e1360_d_n27, eq104_e1360_d_n28, eq104_e1360_d_n29, eq104_e1360_q, eq104_e1360_q_d_n0, eq104_e1360_q_d_n1, eq104_e1360_q_d_n2, eq104_e1360_q_d_n3, eq104_e1360_q_d_n4, eq104_e1360_q_d_n5, eq104_e1360_q_d_n6, eq104_e1360_q_d_n7, eq104_e1360_q_d_n8, eq104_e1360_q_d_n9, eq104_e1360_q_d_n10, eq104_e1360_q_d_n11, eq104_e1360_q_d_n12, eq104_e1360_q_d_n13, eq104_e1360_q_d_n14, eq104_e1360_q_d_n15, eq104_e1360_q_d_n16, eq104_e1360_q_d_n17, eq104_e1360_q_d_n18, eq104_e1360_q_d_n19, eq104_e1360_q_d_n20, eq104_e1360_q_d_n21, eq104_e1360_q_d_n22, eq104_e1360_q_d_n23, eq104_e1360_q_d_n24, eq104_e1360_q_d_n25, eq104_e1360_q_d_n26, eq104_e1360_q_d_n27, eq104_e1360_q_d_n28, eq104_e1360_q_d_n29,) = {
    if (!s.b[1201]) {
        let eq104_e1353_q: f64 = s.v[174];
        let eq104_e1356: f64 = (p.p355 * (nv2 - nv10));
        let eq104_e1356_d_n2: f64 = p.p355;
        let eq104_e1356_d_n10: f64 = (-p.p355);
        let eq104_e1357_q: f64 = eq104_e1356;
        let eq104_e1358: f64 = (s.v[174] + eq104_e1356);
        let eq104_e1358_d_n2: f64 = (s.dn[174][2] + eq104_e1356_d_n2);
        let eq104_e1358_d_n10: f64 = (s.dn[174][10] + eq104_e1356_d_n10);
        let eq104_e1358_q: f64 = (eq104_e1353_q + eq104_e1357_q);
        let eq104_e1358_q_d_n2: f64 = (s.dn[174][2] + eq104_e1356_d_n2);
        let eq104_e1358_q_d_n10: f64 = (s.dn[174][10] + eq104_e1356_d_n10);
        (eq104_e1358, s.dn[174][0], s.dn[174][1], eq104_e1358_d_n2, s.dn[174][3], s.dn[174][4], s.dn[174][5], s.dn[174][6], s.dn[174][7], s.dn[174][8], s.dn[174][9], eq104_e1358_d_n10, s.dn[174][11], s.dn[174][12], s.dn[174][13], s.dn[174][14], s.dn[174][15], s.dn[174][16], s.dn[174][17], s.dn[174][18], s.dn[174][19], s.dn[174][20], s.dn[174][21], s.dn[174][22], s.dn[174][23], s.dn[174][24], s.dn[174][25], s.dn[174][26], s.dn[174][27], s.dn[174][28], s.dn[174][29], eq104_e1358_q, s.dn[174][0], s.dn[174][1], eq104_e1358_q_d_n2, s.dn[174][3], s.dn[174][4], s.dn[174][5], s.dn[174][6], s.dn[174][7], s.dn[174][8], s.dn[174][9], eq104_e1358_q_d_n10, s.dn[174][11], s.dn[174][12], s.dn[174][13], s.dn[174][14], s.dn[174][15], s.dn[174][16], s.dn[174][17], s.dn[174][18], s.dn[174][19], s.dn[174][20], s.dn[174][21], s.dn[174][22], s.dn[174][23], s.dn[174][24], s.dn[174][25], s.dn[174][26], s.dn[174][27], s.dn[174][28], s.dn[174][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq104_reactive_node_derivatives: [f64; 30] = [eq104_e1360_q_d_n0, eq104_e1360_q_d_n1, eq104_e1360_q_d_n2, eq104_e1360_q_d_n3, eq104_e1360_q_d_n4, eq104_e1360_q_d_n5, eq104_e1360_q_d_n6, eq104_e1360_q_d_n7, eq104_e1360_q_d_n8, eq104_e1360_q_d_n9, eq104_e1360_q_d_n10, eq104_e1360_q_d_n11, eq104_e1360_q_d_n12, eq104_e1360_q_d_n13, eq104_e1360_q_d_n14, eq104_e1360_q_d_n15, eq104_e1360_q_d_n16, eq104_e1360_q_d_n17, eq104_e1360_q_d_n18, eq104_e1360_q_d_n19, eq104_e1360_q_d_n20, eq104_e1360_q_d_n21, eq104_e1360_q_d_n22, eq104_e1360_q_d_n23, eq104_e1360_q_d_n24, eq104_e1360_q_d_n25, eq104_e1360_q_d_n26, eq104_e1360_q_d_n27, eq104_e1360_q_d_n28, eq104_e1360_q_d_n29];
        let eq104_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[10]),
            nodes,
            &eq104_reactive_node_derivatives,
            branches,
            &eq104_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq105_e1371, eq105_e1371_d_n0, eq105_e1371_d_n1, eq105_e1371_d_n2, eq105_e1371_d_n3, eq105_e1371_d_n4, eq105_e1371_d_n5, eq105_e1371_d_n6, eq105_e1371_d_n7, eq105_e1371_d_n8, eq105_e1371_d_n9, eq105_e1371_d_n10, eq105_e1371_d_n11, eq105_e1371_d_n12, eq105_e1371_d_n13, eq105_e1371_d_n14, eq105_e1371_d_n15, eq105_e1371_d_n16, eq105_e1371_d_n17, eq105_e1371_d_n18, eq105_e1371_d_n19, eq105_e1371_d_n20, eq105_e1371_d_n21, eq105_e1371_d_n22, eq105_e1371_d_n23, eq105_e1371_d_n24, eq105_e1371_d_n25, eq105_e1371_d_n26, eq105_e1371_d_n27, eq105_e1371_d_n28, eq105_e1371_d_n29, eq105_e1371_q, eq105_e1371_q_d_n0, eq105_e1371_q_d_n1, eq105_e1371_q_d_n2, eq105_e1371_q_d_n3, eq105_e1371_q_d_n4, eq105_e1371_q_d_n5, eq105_e1371_q_d_n6, eq105_e1371_q_d_n7, eq105_e1371_q_d_n8, eq105_e1371_q_d_n9, eq105_e1371_q_d_n10, eq105_e1371_q_d_n11, eq105_e1371_q_d_n12, eq105_e1371_q_d_n13, eq105_e1371_q_d_n14, eq105_e1371_q_d_n15, eq105_e1371_q_d_n16, eq105_e1371_q_d_n17, eq105_e1371_q_d_n18, eq105_e1371_q_d_n19, eq105_e1371_q_d_n20, eq105_e1371_q_d_n21, eq105_e1371_q_d_n22, eq105_e1371_q_d_n23, eq105_e1371_q_d_n24, eq105_e1371_q_d_n25, eq105_e1371_q_d_n26, eq105_e1371_q_d_n27, eq105_e1371_q_d_n28, eq105_e1371_q_d_n29,) = {
    if (!s.b[1201]) {
        let eq105_e1364_q: f64 = s.v[175];
        let eq105_e1367: f64 = (p.p355 * (nv7 - nv11));
        let eq105_e1367_d_n7: f64 = p.p355;
        let eq105_e1367_d_n11: f64 = (-p.p355);
        let eq105_e1368_q: f64 = eq105_e1367;
        let eq105_e1369: f64 = (s.v[175] + eq105_e1367);
        let eq105_e1369_d_n7: f64 = (s.dn[175][7] + eq105_e1367_d_n7);
        let eq105_e1369_d_n11: f64 = (s.dn[175][11] + eq105_e1367_d_n11);
        let eq105_e1369_q: f64 = (eq105_e1364_q + eq105_e1368_q);
        let eq105_e1369_q_d_n7: f64 = (s.dn[175][7] + eq105_e1367_d_n7);
        let eq105_e1369_q_d_n11: f64 = (s.dn[175][11] + eq105_e1367_d_n11);
        (eq105_e1369, s.dn[175][0], s.dn[175][1], s.dn[175][2], s.dn[175][3], s.dn[175][4], s.dn[175][5], s.dn[175][6], eq105_e1369_d_n7, s.dn[175][8], s.dn[175][9], s.dn[175][10], eq105_e1369_d_n11, s.dn[175][12], s.dn[175][13], s.dn[175][14], s.dn[175][15], s.dn[175][16], s.dn[175][17], s.dn[175][18], s.dn[175][19], s.dn[175][20], s.dn[175][21], s.dn[175][22], s.dn[175][23], s.dn[175][24], s.dn[175][25], s.dn[175][26], s.dn[175][27], s.dn[175][28], s.dn[175][29], eq105_e1369_q, s.dn[175][0], s.dn[175][1], s.dn[175][2], s.dn[175][3], s.dn[175][4], s.dn[175][5], s.dn[175][6], eq105_e1369_q_d_n7, s.dn[175][8], s.dn[175][9], s.dn[175][10], eq105_e1369_q_d_n11, s.dn[175][12], s.dn[175][13], s.dn[175][14], s.dn[175][15], s.dn[175][16], s.dn[175][17], s.dn[175][18], s.dn[175][19], s.dn[175][20], s.dn[175][21], s.dn[175][22], s.dn[175][23], s.dn[175][24], s.dn[175][25], s.dn[175][26], s.dn[175][27], s.dn[175][28], s.dn[175][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq105_reactive_node_derivatives: [f64; 30] = [eq105_e1371_q_d_n0, eq105_e1371_q_d_n1, eq105_e1371_q_d_n2, eq105_e1371_q_d_n3, eq105_e1371_q_d_n4, eq105_e1371_q_d_n5, eq105_e1371_q_d_n6, eq105_e1371_q_d_n7, eq105_e1371_q_d_n8, eq105_e1371_q_d_n9, eq105_e1371_q_d_n10, eq105_e1371_q_d_n11, eq105_e1371_q_d_n12, eq105_e1371_q_d_n13, eq105_e1371_q_d_n14, eq105_e1371_q_d_n15, eq105_e1371_q_d_n16, eq105_e1371_q_d_n17, eq105_e1371_q_d_n18, eq105_e1371_q_d_n19, eq105_e1371_q_d_n20, eq105_e1371_q_d_n21, eq105_e1371_q_d_n22, eq105_e1371_q_d_n23, eq105_e1371_q_d_n24, eq105_e1371_q_d_n25, eq105_e1371_q_d_n26, eq105_e1371_q_d_n27, eq105_e1371_q_d_n28, eq105_e1371_q_d_n29];
        let eq105_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            nodes,
            &eq105_reactive_node_derivatives,
            branches,
            &eq105_reactive_branch_derivatives,
            multiplicity,
        );
        let eq108_e1383_q: f64 = s.v[176];
        let eq108_e1386: f64 = (p.p355 * (nv3 - nv11));
        let eq108_e1386_d_n3: f64 = p.p355;
        let eq108_e1386_d_n11: f64 = (-p.p355);
        let eq108_e1387_q: f64 = eq108_e1386;
        let eq108_e1388: f64 = (s.v[176] + eq108_e1386);
        let eq108_e1388_d_n3: f64 = (s.dn[176][3] + eq108_e1386_d_n3);
        let eq108_e1388_d_n11: f64 = (s.dn[176][11] + eq108_e1386_d_n11);
        let eq108_e1388_q: f64 = (eq108_e1383_q + eq108_e1387_q);
        let eq108_e1388_q_d_n3: f64 = (s.dn[176][3] + eq108_e1386_d_n3);
        let eq108_e1388_q_d_n11: f64 = (s.dn[176][11] + eq108_e1386_d_n11);
        let eq108_reactive_node_derivatives: [f64; 30] = [s.dn[176][0], s.dn[176][1], s.dn[176][2], eq108_e1388_q_d_n3, s.dn[176][4], s.dn[176][5], s.dn[176][6], s.dn[176][7], s.dn[176][8], s.dn[176][9], s.dn[176][10], eq108_e1388_q_d_n11, s.dn[176][12], s.dn[176][13], s.dn[176][14], s.dn[176][15], s.dn[176][16], s.dn[176][17], s.dn[176][18], s.dn[176][19], s.dn[176][20], s.dn[176][21], s.dn[176][22], s.dn[176][23], s.dn[176][24], s.dn[176][25], s.dn[176][26], s.dn[176][27], s.dn[176][28], s.dn[176][29]];
        let eq108_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[11]),
            nodes,
            &eq108_reactive_node_derivatives,
            branches,
            &eq108_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq111_e1411, eq111_e1411_d_n0, eq111_e1411_d_n1, eq111_e1411_d_n2, eq111_e1411_d_n3, eq111_e1411_d_n4, eq111_e1411_d_n5, eq111_e1411_d_n6, eq111_e1411_d_n7, eq111_e1411_d_n8, eq111_e1411_d_n9, eq111_e1411_d_n10, eq111_e1411_d_n11, eq111_e1411_d_n12, eq111_e1411_d_n13, eq111_e1411_d_n14, eq111_e1411_d_n15, eq111_e1411_d_n16, eq111_e1411_d_n17, eq111_e1411_d_n18, eq111_e1411_d_n19, eq111_e1411_d_n20, eq111_e1411_d_n21, eq111_e1411_d_n22, eq111_e1411_d_n23, eq111_e1411_d_n24, eq111_e1411_d_n25, eq111_e1411_d_n26, eq111_e1411_d_n27, eq111_e1411_d_n28, eq111_e1411_d_n29, eq111_e1411_q, eq111_e1411_q_d_n0, eq111_e1411_q_d_n1, eq111_e1411_q_d_n2, eq111_e1411_q_d_n3, eq111_e1411_q_d_n4, eq111_e1411_q_d_n5, eq111_e1411_q_d_n6, eq111_e1411_q_d_n7, eq111_e1411_q_d_n8, eq111_e1411_q_d_n9, eq111_e1411_q_d_n10, eq111_e1411_q_d_n11, eq111_e1411_q_d_n12, eq111_e1411_q_d_n13, eq111_e1411_q_d_n14, eq111_e1411_q_d_n15, eq111_e1411_q_d_n16, eq111_e1411_q_d_n17, eq111_e1411_q_d_n18, eq111_e1411_q_d_n19, eq111_e1411_q_d_n20, eq111_e1411_q_d_n21, eq111_e1411_q_d_n22, eq111_e1411_q_d_n23, eq111_e1411_q_d_n24, eq111_e1411_q_d_n25, eq111_e1411_q_d_n26, eq111_e1411_q_d_n27, eq111_e1411_q_d_n28, eq111_e1411_q_d_n29,) = {
    if s.b[1348] {
        let eq111_e1404_q: f64 = s.v[179];
        let eq111_e1407: f64 = (p.p355 * (nv7 - nv12));
        let eq111_e1407_d_n7: f64 = p.p355;
        let eq111_e1407_d_n12: f64 = (-p.p355);
        let eq111_e1408_q: f64 = eq111_e1407;
        let eq111_e1409: f64 = (s.v[179] + eq111_e1407);
        let eq111_e1409_d_n7: f64 = (s.dn[179][7] + eq111_e1407_d_n7);
        let eq111_e1409_d_n12: f64 = (s.dn[179][12] + eq111_e1407_d_n12);
        let eq111_e1409_q: f64 = (eq111_e1404_q + eq111_e1408_q);
        let eq111_e1409_q_d_n7: f64 = (s.dn[179][7] + eq111_e1407_d_n7);
        let eq111_e1409_q_d_n12: f64 = (s.dn[179][12] + eq111_e1407_d_n12);
        (eq111_e1409, s.dn[179][0], s.dn[179][1], s.dn[179][2], s.dn[179][3], s.dn[179][4], s.dn[179][5], s.dn[179][6], eq111_e1409_d_n7, s.dn[179][8], s.dn[179][9], s.dn[179][10], s.dn[179][11], eq111_e1409_d_n12, s.dn[179][13], s.dn[179][14], s.dn[179][15], s.dn[179][16], s.dn[179][17], s.dn[179][18], s.dn[179][19], s.dn[179][20], s.dn[179][21], s.dn[179][22], s.dn[179][23], s.dn[179][24], s.dn[179][25], s.dn[179][26], s.dn[179][27], s.dn[179][28], s.dn[179][29], eq111_e1409_q, s.dn[179][0], s.dn[179][1], s.dn[179][2], s.dn[179][3], s.dn[179][4], s.dn[179][5], s.dn[179][6], eq111_e1409_q_d_n7, s.dn[179][8], s.dn[179][9], s.dn[179][10], s.dn[179][11], eq111_e1409_q_d_n12, s.dn[179][13], s.dn[179][14], s.dn[179][15], s.dn[179][16], s.dn[179][17], s.dn[179][18], s.dn[179][19], s.dn[179][20], s.dn[179][21], s.dn[179][22], s.dn[179][23], s.dn[179][24], s.dn[179][25], s.dn[179][26], s.dn[179][27], s.dn[179][28], s.dn[179][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq111_reactive_node_derivatives: [f64; 30] = [eq111_e1411_q_d_n0, eq111_e1411_q_d_n1, eq111_e1411_q_d_n2, eq111_e1411_q_d_n3, eq111_e1411_q_d_n4, eq111_e1411_q_d_n5, eq111_e1411_q_d_n6, eq111_e1411_q_d_n7, eq111_e1411_q_d_n8, eq111_e1411_q_d_n9, eq111_e1411_q_d_n10, eq111_e1411_q_d_n11, eq111_e1411_q_d_n12, eq111_e1411_q_d_n13, eq111_e1411_q_d_n14, eq111_e1411_q_d_n15, eq111_e1411_q_d_n16, eq111_e1411_q_d_n17, eq111_e1411_q_d_n18, eq111_e1411_q_d_n19, eq111_e1411_q_d_n20, eq111_e1411_q_d_n21, eq111_e1411_q_d_n22, eq111_e1411_q_d_n23, eq111_e1411_q_d_n24, eq111_e1411_q_d_n25, eq111_e1411_q_d_n26, eq111_e1411_q_d_n27, eq111_e1411_q_d_n28, eq111_e1411_q_d_n29];
        let eq111_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[12]),
            nodes,
            &eq111_reactive_node_derivatives,
            branches,
            &eq111_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq112_e1421, eq112_e1421_d_n0, eq112_e1421_d_n1, eq112_e1421_d_n2, eq112_e1421_d_n3, eq112_e1421_d_n4, eq112_e1421_d_n5, eq112_e1421_d_n6, eq112_e1421_d_n7, eq112_e1421_d_n8, eq112_e1421_d_n9, eq112_e1421_d_n10, eq112_e1421_d_n11, eq112_e1421_d_n12, eq112_e1421_d_n13, eq112_e1421_d_n14, eq112_e1421_d_n15, eq112_e1421_d_n16, eq112_e1421_d_n17, eq112_e1421_d_n18, eq112_e1421_d_n19, eq112_e1421_d_n20, eq112_e1421_d_n21, eq112_e1421_d_n22, eq112_e1421_d_n23, eq112_e1421_d_n24, eq112_e1421_d_n25, eq112_e1421_d_n26, eq112_e1421_d_n27, eq112_e1421_d_n28, eq112_e1421_d_n29, eq112_e1421_q, eq112_e1421_q_d_n0, eq112_e1421_q_d_n1, eq112_e1421_q_d_n2, eq112_e1421_q_d_n3, eq112_e1421_q_d_n4, eq112_e1421_q_d_n5, eq112_e1421_q_d_n6, eq112_e1421_q_d_n7, eq112_e1421_q_d_n8, eq112_e1421_q_d_n9, eq112_e1421_q_d_n10, eq112_e1421_q_d_n11, eq112_e1421_q_d_n12, eq112_e1421_q_d_n13, eq112_e1421_q_d_n14, eq112_e1421_q_d_n15, eq112_e1421_q_d_n16, eq112_e1421_q_d_n17, eq112_e1421_q_d_n18, eq112_e1421_q_d_n19, eq112_e1421_q_d_n20, eq112_e1421_q_d_n21, eq112_e1421_q_d_n22, eq112_e1421_q_d_n23, eq112_e1421_q_d_n24, eq112_e1421_q_d_n25, eq112_e1421_q_d_n26, eq112_e1421_q_d_n27, eq112_e1421_q_d_n28, eq112_e1421_q_d_n29,) = {
    if s.b[1348] {
        let eq112_e1414_q: f64 = s.v[180];
        let eq112_e1417: f64 = (p.p355 * (nv7 - nv11));
        let eq112_e1417_d_n7: f64 = p.p355;
        let eq112_e1417_d_n11: f64 = (-p.p355);
        let eq112_e1418_q: f64 = eq112_e1417;
        let eq112_e1419: f64 = (s.v[180] + eq112_e1417);
        let eq112_e1419_d_n7: f64 = (s.dn[180][7] + eq112_e1417_d_n7);
        let eq112_e1419_d_n11: f64 = (s.dn[180][11] + eq112_e1417_d_n11);
        let eq112_e1419_q: f64 = (eq112_e1414_q + eq112_e1418_q);
        let eq112_e1419_q_d_n7: f64 = (s.dn[180][7] + eq112_e1417_d_n7);
        let eq112_e1419_q_d_n11: f64 = (s.dn[180][11] + eq112_e1417_d_n11);
        (eq112_e1419, s.dn[180][0], s.dn[180][1], s.dn[180][2], s.dn[180][3], s.dn[180][4], s.dn[180][5], s.dn[180][6], eq112_e1419_d_n7, s.dn[180][8], s.dn[180][9], s.dn[180][10], eq112_e1419_d_n11, s.dn[180][12], s.dn[180][13], s.dn[180][14], s.dn[180][15], s.dn[180][16], s.dn[180][17], s.dn[180][18], s.dn[180][19], s.dn[180][20], s.dn[180][21], s.dn[180][22], s.dn[180][23], s.dn[180][24], s.dn[180][25], s.dn[180][26], s.dn[180][27], s.dn[180][28], s.dn[180][29], eq112_e1419_q, s.dn[180][0], s.dn[180][1], s.dn[180][2], s.dn[180][3], s.dn[180][4], s.dn[180][5], s.dn[180][6], eq112_e1419_q_d_n7, s.dn[180][8], s.dn[180][9], s.dn[180][10], eq112_e1419_q_d_n11, s.dn[180][12], s.dn[180][13], s.dn[180][14], s.dn[180][15], s.dn[180][16], s.dn[180][17], s.dn[180][18], s.dn[180][19], s.dn[180][20], s.dn[180][21], s.dn[180][22], s.dn[180][23], s.dn[180][24], s.dn[180][25], s.dn[180][26], s.dn[180][27], s.dn[180][28], s.dn[180][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq112_reactive_node_derivatives: [f64; 30] = [eq112_e1421_q_d_n0, eq112_e1421_q_d_n1, eq112_e1421_q_d_n2, eq112_e1421_q_d_n3, eq112_e1421_q_d_n4, eq112_e1421_q_d_n5, eq112_e1421_q_d_n6, eq112_e1421_q_d_n7, eq112_e1421_q_d_n8, eq112_e1421_q_d_n9, eq112_e1421_q_d_n10, eq112_e1421_q_d_n11, eq112_e1421_q_d_n12, eq112_e1421_q_d_n13, eq112_e1421_q_d_n14, eq112_e1421_q_d_n15, eq112_e1421_q_d_n16, eq112_e1421_q_d_n17, eq112_e1421_q_d_n18, eq112_e1421_q_d_n19, eq112_e1421_q_d_n20, eq112_e1421_q_d_n21, eq112_e1421_q_d_n22, eq112_e1421_q_d_n23, eq112_e1421_q_d_n24, eq112_e1421_q_d_n25, eq112_e1421_q_d_n26, eq112_e1421_q_d_n27, eq112_e1421_q_d_n28, eq112_e1421_q_d_n29];
        let eq112_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            nodes,
            &eq112_reactive_node_derivatives,
            branches,
            &eq112_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq113_e1431, eq113_e1431_d_n0, eq113_e1431_d_n1, eq113_e1431_d_n2, eq113_e1431_d_n3, eq113_e1431_d_n4, eq113_e1431_d_n5, eq113_e1431_d_n6, eq113_e1431_d_n7, eq113_e1431_d_n8, eq113_e1431_d_n9, eq113_e1431_d_n10, eq113_e1431_d_n11, eq113_e1431_d_n12, eq113_e1431_d_n13, eq113_e1431_d_n14, eq113_e1431_d_n15, eq113_e1431_d_n16, eq113_e1431_d_n17, eq113_e1431_d_n18, eq113_e1431_d_n19, eq113_e1431_d_n20, eq113_e1431_d_n21, eq113_e1431_d_n22, eq113_e1431_d_n23, eq113_e1431_d_n24, eq113_e1431_d_n25, eq113_e1431_d_n26, eq113_e1431_d_n27, eq113_e1431_d_n28, eq113_e1431_d_n29, eq113_e1431_q, eq113_e1431_q_d_n0, eq113_e1431_q_d_n1, eq113_e1431_q_d_n2, eq113_e1431_q_d_n3, eq113_e1431_q_d_n4, eq113_e1431_q_d_n5, eq113_e1431_q_d_n6, eq113_e1431_q_d_n7, eq113_e1431_q_d_n8, eq113_e1431_q_d_n9, eq113_e1431_q_d_n10, eq113_e1431_q_d_n11, eq113_e1431_q_d_n12, eq113_e1431_q_d_n13, eq113_e1431_q_d_n14, eq113_e1431_q_d_n15, eq113_e1431_q_d_n16, eq113_e1431_q_d_n17, eq113_e1431_q_d_n18, eq113_e1431_q_d_n19, eq113_e1431_q_d_n20, eq113_e1431_q_d_n21, eq113_e1431_q_d_n22, eq113_e1431_q_d_n23, eq113_e1431_q_d_n24, eq113_e1431_q_d_n25, eq113_e1431_q_d_n26, eq113_e1431_q_d_n27, eq113_e1431_q_d_n28, eq113_e1431_q_d_n29,) = {
    if s.b[1348] {
        let eq113_e1424_q: f64 = s.v[181];
        let eq113_e1427: f64 = (p.p355 * (nv2 - nv12));
        let eq113_e1427_d_n2: f64 = p.p355;
        let eq113_e1427_d_n12: f64 = (-p.p355);
        let eq113_e1428_q: f64 = eq113_e1427;
        let eq113_e1429: f64 = (s.v[181] + eq113_e1427);
        let eq113_e1429_d_n2: f64 = (s.dn[181][2] + eq113_e1427_d_n2);
        let eq113_e1429_d_n12: f64 = (s.dn[181][12] + eq113_e1427_d_n12);
        let eq113_e1429_q: f64 = (eq113_e1424_q + eq113_e1428_q);
        let eq113_e1429_q_d_n2: f64 = (s.dn[181][2] + eq113_e1427_d_n2);
        let eq113_e1429_q_d_n12: f64 = (s.dn[181][12] + eq113_e1427_d_n12);
        (eq113_e1429, s.dn[181][0], s.dn[181][1], eq113_e1429_d_n2, s.dn[181][3], s.dn[181][4], s.dn[181][5], s.dn[181][6], s.dn[181][7], s.dn[181][8], s.dn[181][9], s.dn[181][10], s.dn[181][11], eq113_e1429_d_n12, s.dn[181][13], s.dn[181][14], s.dn[181][15], s.dn[181][16], s.dn[181][17], s.dn[181][18], s.dn[181][19], s.dn[181][20], s.dn[181][21], s.dn[181][22], s.dn[181][23], s.dn[181][24], s.dn[181][25], s.dn[181][26], s.dn[181][27], s.dn[181][28], s.dn[181][29], eq113_e1429_q, s.dn[181][0], s.dn[181][1], eq113_e1429_q_d_n2, s.dn[181][3], s.dn[181][4], s.dn[181][5], s.dn[181][6], s.dn[181][7], s.dn[181][8], s.dn[181][9], s.dn[181][10], s.dn[181][11], eq113_e1429_q_d_n12, s.dn[181][13], s.dn[181][14], s.dn[181][15], s.dn[181][16], s.dn[181][17], s.dn[181][18], s.dn[181][19], s.dn[181][20], s.dn[181][21], s.dn[181][22], s.dn[181][23], s.dn[181][24], s.dn[181][25], s.dn[181][26], s.dn[181][27], s.dn[181][28], s.dn[181][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq113_reactive_node_derivatives: [f64; 30] = [eq113_e1431_q_d_n0, eq113_e1431_q_d_n1, eq113_e1431_q_d_n2, eq113_e1431_q_d_n3, eq113_e1431_q_d_n4, eq113_e1431_q_d_n5, eq113_e1431_q_d_n6, eq113_e1431_q_d_n7, eq113_e1431_q_d_n8, eq113_e1431_q_d_n9, eq113_e1431_q_d_n10, eq113_e1431_q_d_n11, eq113_e1431_q_d_n12, eq113_e1431_q_d_n13, eq113_e1431_q_d_n14, eq113_e1431_q_d_n15, eq113_e1431_q_d_n16, eq113_e1431_q_d_n17, eq113_e1431_q_d_n18, eq113_e1431_q_d_n19, eq113_e1431_q_d_n20, eq113_e1431_q_d_n21, eq113_e1431_q_d_n22, eq113_e1431_q_d_n23, eq113_e1431_q_d_n24, eq113_e1431_q_d_n25, eq113_e1431_q_d_n26, eq113_e1431_q_d_n27, eq113_e1431_q_d_n28, eq113_e1431_q_d_n29];
        let eq113_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[12]),
            nodes,
            &eq113_reactive_node_derivatives,
            branches,
            &eq113_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq115_e1445, eq115_e1445_d_n0, eq115_e1445_d_n1, eq115_e1445_d_n2, eq115_e1445_d_n3, eq115_e1445_d_n4, eq115_e1445_d_n5, eq115_e1445_d_n6, eq115_e1445_d_n7, eq115_e1445_d_n8, eq115_e1445_d_n9, eq115_e1445_d_n10, eq115_e1445_d_n11, eq115_e1445_d_n12, eq115_e1445_d_n13, eq115_e1445_d_n14, eq115_e1445_d_n15, eq115_e1445_d_n16, eq115_e1445_d_n17, eq115_e1445_d_n18, eq115_e1445_d_n19, eq115_e1445_d_n20, eq115_e1445_d_n21, eq115_e1445_d_n22, eq115_e1445_d_n23, eq115_e1445_d_n24, eq115_e1445_d_n25, eq115_e1445_d_n26, eq115_e1445_d_n27, eq115_e1445_d_n28, eq115_e1445_d_n29, eq115_e1445_q, eq115_e1445_q_d_n0, eq115_e1445_q_d_n1, eq115_e1445_q_d_n2, eq115_e1445_q_d_n3, eq115_e1445_q_d_n4, eq115_e1445_q_d_n5, eq115_e1445_q_d_n6, eq115_e1445_q_d_n7, eq115_e1445_q_d_n8, eq115_e1445_q_d_n9, eq115_e1445_q_d_n10, eq115_e1445_q_d_n11, eq115_e1445_q_d_n12, eq115_e1445_q_d_n13, eq115_e1445_q_d_n14, eq115_e1445_q_d_n15, eq115_e1445_q_d_n16, eq115_e1445_q_d_n17, eq115_e1445_q_d_n18, eq115_e1445_q_d_n19, eq115_e1445_q_d_n20, eq115_e1445_q_d_n21, eq115_e1445_q_d_n22, eq115_e1445_q_d_n23, eq115_e1445_q_d_n24, eq115_e1445_q_d_n25, eq115_e1445_q_d_n26, eq115_e1445_q_d_n27, eq115_e1445_q_d_n28, eq115_e1445_q_d_n29,) = {
    if s.b[1348] {
        let eq115_e1438_q: f64 = s.v[183];
        let eq115_e1441: f64 = (p.p355 * (nv7 - nv9));
        let eq115_e1441_d_n7: f64 = p.p355;
        let eq115_e1441_d_n9: f64 = (-p.p355);
        let eq115_e1442_q: f64 = eq115_e1441;
        let eq115_e1443: f64 = (s.v[183] + eq115_e1441);
        let eq115_e1443_d_n7: f64 = (s.dn[183][7] + eq115_e1441_d_n7);
        let eq115_e1443_d_n9: f64 = (s.dn[183][9] + eq115_e1441_d_n9);
        let eq115_e1443_q: f64 = (eq115_e1438_q + eq115_e1442_q);
        let eq115_e1443_q_d_n7: f64 = (s.dn[183][7] + eq115_e1441_d_n7);
        let eq115_e1443_q_d_n9: f64 = (s.dn[183][9] + eq115_e1441_d_n9);
        (eq115_e1443, s.dn[183][0], s.dn[183][1], s.dn[183][2], s.dn[183][3], s.dn[183][4], s.dn[183][5], s.dn[183][6], eq115_e1443_d_n7, s.dn[183][8], eq115_e1443_d_n9, s.dn[183][10], s.dn[183][11], s.dn[183][12], s.dn[183][13], s.dn[183][14], s.dn[183][15], s.dn[183][16], s.dn[183][17], s.dn[183][18], s.dn[183][19], s.dn[183][20], s.dn[183][21], s.dn[183][22], s.dn[183][23], s.dn[183][24], s.dn[183][25], s.dn[183][26], s.dn[183][27], s.dn[183][28], s.dn[183][29], eq115_e1443_q, s.dn[183][0], s.dn[183][1], s.dn[183][2], s.dn[183][3], s.dn[183][4], s.dn[183][5], s.dn[183][6], eq115_e1443_q_d_n7, s.dn[183][8], eq115_e1443_q_d_n9, s.dn[183][10], s.dn[183][11], s.dn[183][12], s.dn[183][13], s.dn[183][14], s.dn[183][15], s.dn[183][16], s.dn[183][17], s.dn[183][18], s.dn[183][19], s.dn[183][20], s.dn[183][21], s.dn[183][22], s.dn[183][23], s.dn[183][24], s.dn[183][25], s.dn[183][26], s.dn[183][27], s.dn[183][28], s.dn[183][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq115_reactive_node_derivatives: [f64; 30] = [eq115_e1445_q_d_n0, eq115_e1445_q_d_n1, eq115_e1445_q_d_n2, eq115_e1445_q_d_n3, eq115_e1445_q_d_n4, eq115_e1445_q_d_n5, eq115_e1445_q_d_n6, eq115_e1445_q_d_n7, eq115_e1445_q_d_n8, eq115_e1445_q_d_n9, eq115_e1445_q_d_n10, eq115_e1445_q_d_n11, eq115_e1445_q_d_n12, eq115_e1445_q_d_n13, eq115_e1445_q_d_n14, eq115_e1445_q_d_n15, eq115_e1445_q_d_n16, eq115_e1445_q_d_n17, eq115_e1445_q_d_n18, eq115_e1445_q_d_n19, eq115_e1445_q_d_n20, eq115_e1445_q_d_n21, eq115_e1445_q_d_n22, eq115_e1445_q_d_n23, eq115_e1445_q_d_n24, eq115_e1445_q_d_n25, eq115_e1445_q_d_n26, eq115_e1445_q_d_n27, eq115_e1445_q_d_n28, eq115_e1445_q_d_n29];
        let eq115_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq115_reactive_node_derivatives,
            branches,
            &eq115_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq116_e1456, eq116_e1456_d_n0, eq116_e1456_d_n1, eq116_e1456_d_n2, eq116_e1456_d_n3, eq116_e1456_d_n4, eq116_e1456_d_n5, eq116_e1456_d_n6, eq116_e1456_d_n7, eq116_e1456_d_n8, eq116_e1456_d_n9, eq116_e1456_d_n10, eq116_e1456_d_n11, eq116_e1456_d_n12, eq116_e1456_d_n13, eq116_e1456_d_n14, eq116_e1456_d_n15, eq116_e1456_d_n16, eq116_e1456_d_n17, eq116_e1456_d_n18, eq116_e1456_d_n19, eq116_e1456_d_n20, eq116_e1456_d_n21, eq116_e1456_d_n22, eq116_e1456_d_n23, eq116_e1456_d_n24, eq116_e1456_d_n25, eq116_e1456_d_n26, eq116_e1456_d_n27, eq116_e1456_d_n28, eq116_e1456_d_n29, eq116_e1456_q, eq116_e1456_q_d_n0, eq116_e1456_q_d_n1, eq116_e1456_q_d_n2, eq116_e1456_q_d_n3, eq116_e1456_q_d_n4, eq116_e1456_q_d_n5, eq116_e1456_q_d_n6, eq116_e1456_q_d_n7, eq116_e1456_q_d_n8, eq116_e1456_q_d_n9, eq116_e1456_q_d_n10, eq116_e1456_q_d_n11, eq116_e1456_q_d_n12, eq116_e1456_q_d_n13, eq116_e1456_q_d_n14, eq116_e1456_q_d_n15, eq116_e1456_q_d_n16, eq116_e1456_q_d_n17, eq116_e1456_q_d_n18, eq116_e1456_q_d_n19, eq116_e1456_q_d_n20, eq116_e1456_q_d_n21, eq116_e1456_q_d_n22, eq116_e1456_q_d_n23, eq116_e1456_q_d_n24, eq116_e1456_q_d_n25, eq116_e1456_q_d_n26, eq116_e1456_q_d_n27, eq116_e1456_q_d_n28, eq116_e1456_q_d_n29,) = {
    if (!s.b[1348]) {
        let eq116_e1449_q: f64 = s.v[179];
        let eq116_e1452: f64 = (p.p355 * (nv2 - nv12));
        let eq116_e1452_d_n2: f64 = p.p355;
        let eq116_e1452_d_n12: f64 = (-p.p355);
        let eq116_e1453_q: f64 = eq116_e1452;
        let eq116_e1454: f64 = (s.v[179] + eq116_e1452);
        let eq116_e1454_d_n2: f64 = (s.dn[179][2] + eq116_e1452_d_n2);
        let eq116_e1454_d_n12: f64 = (s.dn[179][12] + eq116_e1452_d_n12);
        let eq116_e1454_q: f64 = (eq116_e1449_q + eq116_e1453_q);
        let eq116_e1454_q_d_n2: f64 = (s.dn[179][2] + eq116_e1452_d_n2);
        let eq116_e1454_q_d_n12: f64 = (s.dn[179][12] + eq116_e1452_d_n12);
        (eq116_e1454, s.dn[179][0], s.dn[179][1], eq116_e1454_d_n2, s.dn[179][3], s.dn[179][4], s.dn[179][5], s.dn[179][6], s.dn[179][7], s.dn[179][8], s.dn[179][9], s.dn[179][10], s.dn[179][11], eq116_e1454_d_n12, s.dn[179][13], s.dn[179][14], s.dn[179][15], s.dn[179][16], s.dn[179][17], s.dn[179][18], s.dn[179][19], s.dn[179][20], s.dn[179][21], s.dn[179][22], s.dn[179][23], s.dn[179][24], s.dn[179][25], s.dn[179][26], s.dn[179][27], s.dn[179][28], s.dn[179][29], eq116_e1454_q, s.dn[179][0], s.dn[179][1], eq116_e1454_q_d_n2, s.dn[179][3], s.dn[179][4], s.dn[179][5], s.dn[179][6], s.dn[179][7], s.dn[179][8], s.dn[179][9], s.dn[179][10], s.dn[179][11], eq116_e1454_q_d_n12, s.dn[179][13], s.dn[179][14], s.dn[179][15], s.dn[179][16], s.dn[179][17], s.dn[179][18], s.dn[179][19], s.dn[179][20], s.dn[179][21], s.dn[179][22], s.dn[179][23], s.dn[179][24], s.dn[179][25], s.dn[179][26], s.dn[179][27], s.dn[179][28], s.dn[179][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq116_reactive_node_derivatives: [f64; 30] = [eq116_e1456_q_d_n0, eq116_e1456_q_d_n1, eq116_e1456_q_d_n2, eq116_e1456_q_d_n3, eq116_e1456_q_d_n4, eq116_e1456_q_d_n5, eq116_e1456_q_d_n6, eq116_e1456_q_d_n7, eq116_e1456_q_d_n8, eq116_e1456_q_d_n9, eq116_e1456_q_d_n10, eq116_e1456_q_d_n11, eq116_e1456_q_d_n12, eq116_e1456_q_d_n13, eq116_e1456_q_d_n14, eq116_e1456_q_d_n15, eq116_e1456_q_d_n16, eq116_e1456_q_d_n17, eq116_e1456_q_d_n18, eq116_e1456_q_d_n19, eq116_e1456_q_d_n20, eq116_e1456_q_d_n21, eq116_e1456_q_d_n22, eq116_e1456_q_d_n23, eq116_e1456_q_d_n24, eq116_e1456_q_d_n25, eq116_e1456_q_d_n26, eq116_e1456_q_d_n27, eq116_e1456_q_d_n28, eq116_e1456_q_d_n29];
        let eq116_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[12]),
            nodes,
            &eq116_reactive_node_derivatives,
            branches,
            &eq116_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq117_e1467, eq117_e1467_d_n0, eq117_e1467_d_n1, eq117_e1467_d_n2, eq117_e1467_d_n3, eq117_e1467_d_n4, eq117_e1467_d_n5, eq117_e1467_d_n6, eq117_e1467_d_n7, eq117_e1467_d_n8, eq117_e1467_d_n9, eq117_e1467_d_n10, eq117_e1467_d_n11, eq117_e1467_d_n12, eq117_e1467_d_n13, eq117_e1467_d_n14, eq117_e1467_d_n15, eq117_e1467_d_n16, eq117_e1467_d_n17, eq117_e1467_d_n18, eq117_e1467_d_n19, eq117_e1467_d_n20, eq117_e1467_d_n21, eq117_e1467_d_n22, eq117_e1467_d_n23, eq117_e1467_d_n24, eq117_e1467_d_n25, eq117_e1467_d_n26, eq117_e1467_d_n27, eq117_e1467_d_n28, eq117_e1467_d_n29, eq117_e1467_q, eq117_e1467_q_d_n0, eq117_e1467_q_d_n1, eq117_e1467_q_d_n2, eq117_e1467_q_d_n3, eq117_e1467_q_d_n4, eq117_e1467_q_d_n5, eq117_e1467_q_d_n6, eq117_e1467_q_d_n7, eq117_e1467_q_d_n8, eq117_e1467_q_d_n9, eq117_e1467_q_d_n10, eq117_e1467_q_d_n11, eq117_e1467_q_d_n12, eq117_e1467_q_d_n13, eq117_e1467_q_d_n14, eq117_e1467_q_d_n15, eq117_e1467_q_d_n16, eq117_e1467_q_d_n17, eq117_e1467_q_d_n18, eq117_e1467_q_d_n19, eq117_e1467_q_d_n20, eq117_e1467_q_d_n21, eq117_e1467_q_d_n22, eq117_e1467_q_d_n23, eq117_e1467_q_d_n24, eq117_e1467_q_d_n25, eq117_e1467_q_d_n26, eq117_e1467_q_d_n27, eq117_e1467_q_d_n28, eq117_e1467_q_d_n29,) = {
    if (!s.b[1348]) {
        let eq117_e1460_q: f64 = s.v[180];
        let eq117_e1463: f64 = (p.p355 * (nv2 - nv11));
        let eq117_e1463_d_n2: f64 = p.p355;
        let eq117_e1463_d_n11: f64 = (-p.p355);
        let eq117_e1464_q: f64 = eq117_e1463;
        let eq117_e1465: f64 = (s.v[180] + eq117_e1463);
        let eq117_e1465_d_n2: f64 = (s.dn[180][2] + eq117_e1463_d_n2);
        let eq117_e1465_d_n11: f64 = (s.dn[180][11] + eq117_e1463_d_n11);
        let eq117_e1465_q: f64 = (eq117_e1460_q + eq117_e1464_q);
        let eq117_e1465_q_d_n2: f64 = (s.dn[180][2] + eq117_e1463_d_n2);
        let eq117_e1465_q_d_n11: f64 = (s.dn[180][11] + eq117_e1463_d_n11);
        (eq117_e1465, s.dn[180][0], s.dn[180][1], eq117_e1465_d_n2, s.dn[180][3], s.dn[180][4], s.dn[180][5], s.dn[180][6], s.dn[180][7], s.dn[180][8], s.dn[180][9], s.dn[180][10], eq117_e1465_d_n11, s.dn[180][12], s.dn[180][13], s.dn[180][14], s.dn[180][15], s.dn[180][16], s.dn[180][17], s.dn[180][18], s.dn[180][19], s.dn[180][20], s.dn[180][21], s.dn[180][22], s.dn[180][23], s.dn[180][24], s.dn[180][25], s.dn[180][26], s.dn[180][27], s.dn[180][28], s.dn[180][29], eq117_e1465_q, s.dn[180][0], s.dn[180][1], eq117_e1465_q_d_n2, s.dn[180][3], s.dn[180][4], s.dn[180][5], s.dn[180][6], s.dn[180][7], s.dn[180][8], s.dn[180][9], s.dn[180][10], eq117_e1465_q_d_n11, s.dn[180][12], s.dn[180][13], s.dn[180][14], s.dn[180][15], s.dn[180][16], s.dn[180][17], s.dn[180][18], s.dn[180][19], s.dn[180][20], s.dn[180][21], s.dn[180][22], s.dn[180][23], s.dn[180][24], s.dn[180][25], s.dn[180][26], s.dn[180][27], s.dn[180][28], s.dn[180][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq117_reactive_node_derivatives: [f64; 30] = [eq117_e1467_q_d_n0, eq117_e1467_q_d_n1, eq117_e1467_q_d_n2, eq117_e1467_q_d_n3, eq117_e1467_q_d_n4, eq117_e1467_q_d_n5, eq117_e1467_q_d_n6, eq117_e1467_q_d_n7, eq117_e1467_q_d_n8, eq117_e1467_q_d_n9, eq117_e1467_q_d_n10, eq117_e1467_q_d_n11, eq117_e1467_q_d_n12, eq117_e1467_q_d_n13, eq117_e1467_q_d_n14, eq117_e1467_q_d_n15, eq117_e1467_q_d_n16, eq117_e1467_q_d_n17, eq117_e1467_q_d_n18, eq117_e1467_q_d_n19, eq117_e1467_q_d_n20, eq117_e1467_q_d_n21, eq117_e1467_q_d_n22, eq117_e1467_q_d_n23, eq117_e1467_q_d_n24, eq117_e1467_q_d_n25, eq117_e1467_q_d_n26, eq117_e1467_q_d_n27, eq117_e1467_q_d_n28, eq117_e1467_q_d_n29];
        let eq117_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[11]),
            nodes,
            &eq117_reactive_node_derivatives,
            branches,
            &eq117_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq118_e1478, eq118_e1478_d_n0, eq118_e1478_d_n1, eq118_e1478_d_n2, eq118_e1478_d_n3, eq118_e1478_d_n4, eq118_e1478_d_n5, eq118_e1478_d_n6, eq118_e1478_d_n7, eq118_e1478_d_n8, eq118_e1478_d_n9, eq118_e1478_d_n10, eq118_e1478_d_n11, eq118_e1478_d_n12, eq118_e1478_d_n13, eq118_e1478_d_n14, eq118_e1478_d_n15, eq118_e1478_d_n16, eq118_e1478_d_n17, eq118_e1478_d_n18, eq118_e1478_d_n19, eq118_e1478_d_n20, eq118_e1478_d_n21, eq118_e1478_d_n22, eq118_e1478_d_n23, eq118_e1478_d_n24, eq118_e1478_d_n25, eq118_e1478_d_n26, eq118_e1478_d_n27, eq118_e1478_d_n28, eq118_e1478_d_n29, eq118_e1478_q, eq118_e1478_q_d_n0, eq118_e1478_q_d_n1, eq118_e1478_q_d_n2, eq118_e1478_q_d_n3, eq118_e1478_q_d_n4, eq118_e1478_q_d_n5, eq118_e1478_q_d_n6, eq118_e1478_q_d_n7, eq118_e1478_q_d_n8, eq118_e1478_q_d_n9, eq118_e1478_q_d_n10, eq118_e1478_q_d_n11, eq118_e1478_q_d_n12, eq118_e1478_q_d_n13, eq118_e1478_q_d_n14, eq118_e1478_q_d_n15, eq118_e1478_q_d_n16, eq118_e1478_q_d_n17, eq118_e1478_q_d_n18, eq118_e1478_q_d_n19, eq118_e1478_q_d_n20, eq118_e1478_q_d_n21, eq118_e1478_q_d_n22, eq118_e1478_q_d_n23, eq118_e1478_q_d_n24, eq118_e1478_q_d_n25, eq118_e1478_q_d_n26, eq118_e1478_q_d_n27, eq118_e1478_q_d_n28, eq118_e1478_q_d_n29,) = {
    if (!s.b[1348]) {
        let eq118_e1471_q: f64 = s.v[181];
        let eq118_e1474: f64 = (p.p355 * (nv7 - nv12));
        let eq118_e1474_d_n7: f64 = p.p355;
        let eq118_e1474_d_n12: f64 = (-p.p355);
        let eq118_e1475_q: f64 = eq118_e1474;
        let eq118_e1476: f64 = (s.v[181] + eq118_e1474);
        let eq118_e1476_d_n7: f64 = (s.dn[181][7] + eq118_e1474_d_n7);
        let eq118_e1476_d_n12: f64 = (s.dn[181][12] + eq118_e1474_d_n12);
        let eq118_e1476_q: f64 = (eq118_e1471_q + eq118_e1475_q);
        let eq118_e1476_q_d_n7: f64 = (s.dn[181][7] + eq118_e1474_d_n7);
        let eq118_e1476_q_d_n12: f64 = (s.dn[181][12] + eq118_e1474_d_n12);
        (eq118_e1476, s.dn[181][0], s.dn[181][1], s.dn[181][2], s.dn[181][3], s.dn[181][4], s.dn[181][5], s.dn[181][6], eq118_e1476_d_n7, s.dn[181][8], s.dn[181][9], s.dn[181][10], s.dn[181][11], eq118_e1476_d_n12, s.dn[181][13], s.dn[181][14], s.dn[181][15], s.dn[181][16], s.dn[181][17], s.dn[181][18], s.dn[181][19], s.dn[181][20], s.dn[181][21], s.dn[181][22], s.dn[181][23], s.dn[181][24], s.dn[181][25], s.dn[181][26], s.dn[181][27], s.dn[181][28], s.dn[181][29], eq118_e1476_q, s.dn[181][0], s.dn[181][1], s.dn[181][2], s.dn[181][3], s.dn[181][4], s.dn[181][5], s.dn[181][6], eq118_e1476_q_d_n7, s.dn[181][8], s.dn[181][9], s.dn[181][10], s.dn[181][11], eq118_e1476_q_d_n12, s.dn[181][13], s.dn[181][14], s.dn[181][15], s.dn[181][16], s.dn[181][17], s.dn[181][18], s.dn[181][19], s.dn[181][20], s.dn[181][21], s.dn[181][22], s.dn[181][23], s.dn[181][24], s.dn[181][25], s.dn[181][26], s.dn[181][27], s.dn[181][28], s.dn[181][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq118_reactive_node_derivatives: [f64; 30] = [eq118_e1478_q_d_n0, eq118_e1478_q_d_n1, eq118_e1478_q_d_n2, eq118_e1478_q_d_n3, eq118_e1478_q_d_n4, eq118_e1478_q_d_n5, eq118_e1478_q_d_n6, eq118_e1478_q_d_n7, eq118_e1478_q_d_n8, eq118_e1478_q_d_n9, eq118_e1478_q_d_n10, eq118_e1478_q_d_n11, eq118_e1478_q_d_n12, eq118_e1478_q_d_n13, eq118_e1478_q_d_n14, eq118_e1478_q_d_n15, eq118_e1478_q_d_n16, eq118_e1478_q_d_n17, eq118_e1478_q_d_n18, eq118_e1478_q_d_n19, eq118_e1478_q_d_n20, eq118_e1478_q_d_n21, eq118_e1478_q_d_n22, eq118_e1478_q_d_n23, eq118_e1478_q_d_n24, eq118_e1478_q_d_n25, eq118_e1478_q_d_n26, eq118_e1478_q_d_n27, eq118_e1478_q_d_n28, eq118_e1478_q_d_n29];
        let eq118_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[12]),
            nodes,
            &eq118_reactive_node_derivatives,
            branches,
            &eq118_reactive_branch_derivatives,
            multiplicity,
        );
        let eq121_e1490_q: f64 = s.v[182];
        let eq121_e1493: f64 = (p.p355 * (nv3 - nv12));
        let eq121_e1493_d_n3: f64 = p.p355;
        let eq121_e1493_d_n12: f64 = (-p.p355);
        let eq121_e1494_q: f64 = eq121_e1493;
        let eq121_e1495: f64 = (s.v[182] + eq121_e1493);
        let eq121_e1495_d_n3: f64 = (s.dn[182][3] + eq121_e1493_d_n3);
        let eq121_e1495_d_n12: f64 = (s.dn[182][12] + eq121_e1493_d_n12);
        let eq121_e1495_q: f64 = (eq121_e1490_q + eq121_e1494_q);
        let eq121_e1495_q_d_n3: f64 = (s.dn[182][3] + eq121_e1493_d_n3);
        let eq121_e1495_q_d_n12: f64 = (s.dn[182][12] + eq121_e1493_d_n12);
        let eq121_reactive_node_derivatives: [f64; 30] = [s.dn[182][0], s.dn[182][1], s.dn[182][2], eq121_e1495_q_d_n3, s.dn[182][4], s.dn[182][5], s.dn[182][6], s.dn[182][7], s.dn[182][8], s.dn[182][9], s.dn[182][10], s.dn[182][11], eq121_e1495_q_d_n12, s.dn[182][13], s.dn[182][14], s.dn[182][15], s.dn[182][16], s.dn[182][17], s.dn[182][18], s.dn[182][19], s.dn[182][20], s.dn[182][21], s.dn[182][22], s.dn[182][23], s.dn[182][24], s.dn[182][25], s.dn[182][26], s.dn[182][27], s.dn[182][28], s.dn[182][29]];
        let eq121_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[12]),
            nodes,
            &eq121_reactive_node_derivatives,
            branches,
            &eq121_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq124_e1518, eq124_e1518_d_n0, eq124_e1518_d_n1, eq124_e1518_d_n2, eq124_e1518_d_n3, eq124_e1518_d_n4, eq124_e1518_d_n5, eq124_e1518_d_n6, eq124_e1518_d_n7, eq124_e1518_d_n8, eq124_e1518_d_n9, eq124_e1518_d_n10, eq124_e1518_d_n11, eq124_e1518_d_n12, eq124_e1518_d_n13, eq124_e1518_d_n14, eq124_e1518_d_n15, eq124_e1518_d_n16, eq124_e1518_d_n17, eq124_e1518_d_n18, eq124_e1518_d_n19, eq124_e1518_d_n20, eq124_e1518_d_n21, eq124_e1518_d_n22, eq124_e1518_d_n23, eq124_e1518_d_n24, eq124_e1518_d_n25, eq124_e1518_d_n26, eq124_e1518_d_n27, eq124_e1518_d_n28, eq124_e1518_d_n29, eq124_e1518_q, eq124_e1518_q_d_n0, eq124_e1518_q_d_n1, eq124_e1518_q_d_n2, eq124_e1518_q_d_n3, eq124_e1518_q_d_n4, eq124_e1518_q_d_n5, eq124_e1518_q_d_n6, eq124_e1518_q_d_n7, eq124_e1518_q_d_n8, eq124_e1518_q_d_n9, eq124_e1518_q_d_n10, eq124_e1518_q_d_n11, eq124_e1518_q_d_n12, eq124_e1518_q_d_n13, eq124_e1518_q_d_n14, eq124_e1518_q_d_n15, eq124_e1518_q_d_n16, eq124_e1518_q_d_n17, eq124_e1518_q_d_n18, eq124_e1518_q_d_n19, eq124_e1518_q_d_n20, eq124_e1518_q_d_n21, eq124_e1518_q_d_n22, eq124_e1518_q_d_n23, eq124_e1518_q_d_n24, eq124_e1518_q_d_n25, eq124_e1518_q_d_n26, eq124_e1518_q_d_n27, eq124_e1518_q_d_n28, eq124_e1518_q_d_n29,) = {
    if s.b[1495] {
        let eq124_e1511_q: f64 = s.v[185];
        let eq124_e1514: f64 = (p.p355 * (nv7 - nv13));
        let eq124_e1514_d_n7: f64 = p.p355;
        let eq124_e1514_d_n13: f64 = (-p.p355);
        let eq124_e1515_q: f64 = eq124_e1514;
        let eq124_e1516: f64 = (s.v[185] + eq124_e1514);
        let eq124_e1516_d_n7: f64 = (s.dn[185][7] + eq124_e1514_d_n7);
        let eq124_e1516_d_n13: f64 = (s.dn[185][13] + eq124_e1514_d_n13);
        let eq124_e1516_q: f64 = (eq124_e1511_q + eq124_e1515_q);
        let eq124_e1516_q_d_n7: f64 = (s.dn[185][7] + eq124_e1514_d_n7);
        let eq124_e1516_q_d_n13: f64 = (s.dn[185][13] + eq124_e1514_d_n13);
        (eq124_e1516, s.dn[185][0], s.dn[185][1], s.dn[185][2], s.dn[185][3], s.dn[185][4], s.dn[185][5], s.dn[185][6], eq124_e1516_d_n7, s.dn[185][8], s.dn[185][9], s.dn[185][10], s.dn[185][11], s.dn[185][12], eq124_e1516_d_n13, s.dn[185][14], s.dn[185][15], s.dn[185][16], s.dn[185][17], s.dn[185][18], s.dn[185][19], s.dn[185][20], s.dn[185][21], s.dn[185][22], s.dn[185][23], s.dn[185][24], s.dn[185][25], s.dn[185][26], s.dn[185][27], s.dn[185][28], s.dn[185][29], eq124_e1516_q, s.dn[185][0], s.dn[185][1], s.dn[185][2], s.dn[185][3], s.dn[185][4], s.dn[185][5], s.dn[185][6], eq124_e1516_q_d_n7, s.dn[185][8], s.dn[185][9], s.dn[185][10], s.dn[185][11], s.dn[185][12], eq124_e1516_q_d_n13, s.dn[185][14], s.dn[185][15], s.dn[185][16], s.dn[185][17], s.dn[185][18], s.dn[185][19], s.dn[185][20], s.dn[185][21], s.dn[185][22], s.dn[185][23], s.dn[185][24], s.dn[185][25], s.dn[185][26], s.dn[185][27], s.dn[185][28], s.dn[185][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq124_reactive_node_derivatives: [f64; 30] = [eq124_e1518_q_d_n0, eq124_e1518_q_d_n1, eq124_e1518_q_d_n2, eq124_e1518_q_d_n3, eq124_e1518_q_d_n4, eq124_e1518_q_d_n5, eq124_e1518_q_d_n6, eq124_e1518_q_d_n7, eq124_e1518_q_d_n8, eq124_e1518_q_d_n9, eq124_e1518_q_d_n10, eq124_e1518_q_d_n11, eq124_e1518_q_d_n12, eq124_e1518_q_d_n13, eq124_e1518_q_d_n14, eq124_e1518_q_d_n15, eq124_e1518_q_d_n16, eq124_e1518_q_d_n17, eq124_e1518_q_d_n18, eq124_e1518_q_d_n19, eq124_e1518_q_d_n20, eq124_e1518_q_d_n21, eq124_e1518_q_d_n22, eq124_e1518_q_d_n23, eq124_e1518_q_d_n24, eq124_e1518_q_d_n25, eq124_e1518_q_d_n26, eq124_e1518_q_d_n27, eq124_e1518_q_d_n28, eq124_e1518_q_d_n29];
        let eq124_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[13]),
            nodes,
            &eq124_reactive_node_derivatives,
            branches,
            &eq124_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq125_e1528, eq125_e1528_d_n0, eq125_e1528_d_n1, eq125_e1528_d_n2, eq125_e1528_d_n3, eq125_e1528_d_n4, eq125_e1528_d_n5, eq125_e1528_d_n6, eq125_e1528_d_n7, eq125_e1528_d_n8, eq125_e1528_d_n9, eq125_e1528_d_n10, eq125_e1528_d_n11, eq125_e1528_d_n12, eq125_e1528_d_n13, eq125_e1528_d_n14, eq125_e1528_d_n15, eq125_e1528_d_n16, eq125_e1528_d_n17, eq125_e1528_d_n18, eq125_e1528_d_n19, eq125_e1528_d_n20, eq125_e1528_d_n21, eq125_e1528_d_n22, eq125_e1528_d_n23, eq125_e1528_d_n24, eq125_e1528_d_n25, eq125_e1528_d_n26, eq125_e1528_d_n27, eq125_e1528_d_n28, eq125_e1528_d_n29, eq125_e1528_q, eq125_e1528_q_d_n0, eq125_e1528_q_d_n1, eq125_e1528_q_d_n2, eq125_e1528_q_d_n3, eq125_e1528_q_d_n4, eq125_e1528_q_d_n5, eq125_e1528_q_d_n6, eq125_e1528_q_d_n7, eq125_e1528_q_d_n8, eq125_e1528_q_d_n9, eq125_e1528_q_d_n10, eq125_e1528_q_d_n11, eq125_e1528_q_d_n12, eq125_e1528_q_d_n13, eq125_e1528_q_d_n14, eq125_e1528_q_d_n15, eq125_e1528_q_d_n16, eq125_e1528_q_d_n17, eq125_e1528_q_d_n18, eq125_e1528_q_d_n19, eq125_e1528_q_d_n20, eq125_e1528_q_d_n21, eq125_e1528_q_d_n22, eq125_e1528_q_d_n23, eq125_e1528_q_d_n24, eq125_e1528_q_d_n25, eq125_e1528_q_d_n26, eq125_e1528_q_d_n27, eq125_e1528_q_d_n28, eq125_e1528_q_d_n29,) = {
    if s.b[1495] {
        let eq125_e1521_q: f64 = s.v[186];
        let eq125_e1524: f64 = (p.p355 * (nv7 - nv12));
        let eq125_e1524_d_n7: f64 = p.p355;
        let eq125_e1524_d_n12: f64 = (-p.p355);
        let eq125_e1525_q: f64 = eq125_e1524;
        let eq125_e1526: f64 = (s.v[186] + eq125_e1524);
        let eq125_e1526_d_n7: f64 = (s.dn[186][7] + eq125_e1524_d_n7);
        let eq125_e1526_d_n12: f64 = (s.dn[186][12] + eq125_e1524_d_n12);
        let eq125_e1526_q: f64 = (eq125_e1521_q + eq125_e1525_q);
        let eq125_e1526_q_d_n7: f64 = (s.dn[186][7] + eq125_e1524_d_n7);
        let eq125_e1526_q_d_n12: f64 = (s.dn[186][12] + eq125_e1524_d_n12);
        (eq125_e1526, s.dn[186][0], s.dn[186][1], s.dn[186][2], s.dn[186][3], s.dn[186][4], s.dn[186][5], s.dn[186][6], eq125_e1526_d_n7, s.dn[186][8], s.dn[186][9], s.dn[186][10], s.dn[186][11], eq125_e1526_d_n12, s.dn[186][13], s.dn[186][14], s.dn[186][15], s.dn[186][16], s.dn[186][17], s.dn[186][18], s.dn[186][19], s.dn[186][20], s.dn[186][21], s.dn[186][22], s.dn[186][23], s.dn[186][24], s.dn[186][25], s.dn[186][26], s.dn[186][27], s.dn[186][28], s.dn[186][29], eq125_e1526_q, s.dn[186][0], s.dn[186][1], s.dn[186][2], s.dn[186][3], s.dn[186][4], s.dn[186][5], s.dn[186][6], eq125_e1526_q_d_n7, s.dn[186][8], s.dn[186][9], s.dn[186][10], s.dn[186][11], eq125_e1526_q_d_n12, s.dn[186][13], s.dn[186][14], s.dn[186][15], s.dn[186][16], s.dn[186][17], s.dn[186][18], s.dn[186][19], s.dn[186][20], s.dn[186][21], s.dn[186][22], s.dn[186][23], s.dn[186][24], s.dn[186][25], s.dn[186][26], s.dn[186][27], s.dn[186][28], s.dn[186][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq125_reactive_node_derivatives: [f64; 30] = [eq125_e1528_q_d_n0, eq125_e1528_q_d_n1, eq125_e1528_q_d_n2, eq125_e1528_q_d_n3, eq125_e1528_q_d_n4, eq125_e1528_q_d_n5, eq125_e1528_q_d_n6, eq125_e1528_q_d_n7, eq125_e1528_q_d_n8, eq125_e1528_q_d_n9, eq125_e1528_q_d_n10, eq125_e1528_q_d_n11, eq125_e1528_q_d_n12, eq125_e1528_q_d_n13, eq125_e1528_q_d_n14, eq125_e1528_q_d_n15, eq125_e1528_q_d_n16, eq125_e1528_q_d_n17, eq125_e1528_q_d_n18, eq125_e1528_q_d_n19, eq125_e1528_q_d_n20, eq125_e1528_q_d_n21, eq125_e1528_q_d_n22, eq125_e1528_q_d_n23, eq125_e1528_q_d_n24, eq125_e1528_q_d_n25, eq125_e1528_q_d_n26, eq125_e1528_q_d_n27, eq125_e1528_q_d_n28, eq125_e1528_q_d_n29];
        let eq125_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[12]),
            nodes,
            &eq125_reactive_node_derivatives,
            branches,
            &eq125_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq126_e1538, eq126_e1538_d_n0, eq126_e1538_d_n1, eq126_e1538_d_n2, eq126_e1538_d_n3, eq126_e1538_d_n4, eq126_e1538_d_n5, eq126_e1538_d_n6, eq126_e1538_d_n7, eq126_e1538_d_n8, eq126_e1538_d_n9, eq126_e1538_d_n10, eq126_e1538_d_n11, eq126_e1538_d_n12, eq126_e1538_d_n13, eq126_e1538_d_n14, eq126_e1538_d_n15, eq126_e1538_d_n16, eq126_e1538_d_n17, eq126_e1538_d_n18, eq126_e1538_d_n19, eq126_e1538_d_n20, eq126_e1538_d_n21, eq126_e1538_d_n22, eq126_e1538_d_n23, eq126_e1538_d_n24, eq126_e1538_d_n25, eq126_e1538_d_n26, eq126_e1538_d_n27, eq126_e1538_d_n28, eq126_e1538_d_n29, eq126_e1538_q, eq126_e1538_q_d_n0, eq126_e1538_q_d_n1, eq126_e1538_q_d_n2, eq126_e1538_q_d_n3, eq126_e1538_q_d_n4, eq126_e1538_q_d_n5, eq126_e1538_q_d_n6, eq126_e1538_q_d_n7, eq126_e1538_q_d_n8, eq126_e1538_q_d_n9, eq126_e1538_q_d_n10, eq126_e1538_q_d_n11, eq126_e1538_q_d_n12, eq126_e1538_q_d_n13, eq126_e1538_q_d_n14, eq126_e1538_q_d_n15, eq126_e1538_q_d_n16, eq126_e1538_q_d_n17, eq126_e1538_q_d_n18, eq126_e1538_q_d_n19, eq126_e1538_q_d_n20, eq126_e1538_q_d_n21, eq126_e1538_q_d_n22, eq126_e1538_q_d_n23, eq126_e1538_q_d_n24, eq126_e1538_q_d_n25, eq126_e1538_q_d_n26, eq126_e1538_q_d_n27, eq126_e1538_q_d_n28, eq126_e1538_q_d_n29,) = {
    if s.b[1495] {
        let eq126_e1531_q: f64 = s.v[187];
        let eq126_e1534: f64 = (p.p355 * (nv2 - nv13));
        let eq126_e1534_d_n2: f64 = p.p355;
        let eq126_e1534_d_n13: f64 = (-p.p355);
        let eq126_e1535_q: f64 = eq126_e1534;
        let eq126_e1536: f64 = (s.v[187] + eq126_e1534);
        let eq126_e1536_d_n2: f64 = (s.dn[187][2] + eq126_e1534_d_n2);
        let eq126_e1536_d_n13: f64 = (s.dn[187][13] + eq126_e1534_d_n13);
        let eq126_e1536_q: f64 = (eq126_e1531_q + eq126_e1535_q);
        let eq126_e1536_q_d_n2: f64 = (s.dn[187][2] + eq126_e1534_d_n2);
        let eq126_e1536_q_d_n13: f64 = (s.dn[187][13] + eq126_e1534_d_n13);
        (eq126_e1536, s.dn[187][0], s.dn[187][1], eq126_e1536_d_n2, s.dn[187][3], s.dn[187][4], s.dn[187][5], s.dn[187][6], s.dn[187][7], s.dn[187][8], s.dn[187][9], s.dn[187][10], s.dn[187][11], s.dn[187][12], eq126_e1536_d_n13, s.dn[187][14], s.dn[187][15], s.dn[187][16], s.dn[187][17], s.dn[187][18], s.dn[187][19], s.dn[187][20], s.dn[187][21], s.dn[187][22], s.dn[187][23], s.dn[187][24], s.dn[187][25], s.dn[187][26], s.dn[187][27], s.dn[187][28], s.dn[187][29], eq126_e1536_q, s.dn[187][0], s.dn[187][1], eq126_e1536_q_d_n2, s.dn[187][3], s.dn[187][4], s.dn[187][5], s.dn[187][6], s.dn[187][7], s.dn[187][8], s.dn[187][9], s.dn[187][10], s.dn[187][11], s.dn[187][12], eq126_e1536_q_d_n13, s.dn[187][14], s.dn[187][15], s.dn[187][16], s.dn[187][17], s.dn[187][18], s.dn[187][19], s.dn[187][20], s.dn[187][21], s.dn[187][22], s.dn[187][23], s.dn[187][24], s.dn[187][25], s.dn[187][26], s.dn[187][27], s.dn[187][28], s.dn[187][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq126_reactive_node_derivatives: [f64; 30] = [eq126_e1538_q_d_n0, eq126_e1538_q_d_n1, eq126_e1538_q_d_n2, eq126_e1538_q_d_n3, eq126_e1538_q_d_n4, eq126_e1538_q_d_n5, eq126_e1538_q_d_n6, eq126_e1538_q_d_n7, eq126_e1538_q_d_n8, eq126_e1538_q_d_n9, eq126_e1538_q_d_n10, eq126_e1538_q_d_n11, eq126_e1538_q_d_n12, eq126_e1538_q_d_n13, eq126_e1538_q_d_n14, eq126_e1538_q_d_n15, eq126_e1538_q_d_n16, eq126_e1538_q_d_n17, eq126_e1538_q_d_n18, eq126_e1538_q_d_n19, eq126_e1538_q_d_n20, eq126_e1538_q_d_n21, eq126_e1538_q_d_n22, eq126_e1538_q_d_n23, eq126_e1538_q_d_n24, eq126_e1538_q_d_n25, eq126_e1538_q_d_n26, eq126_e1538_q_d_n27, eq126_e1538_q_d_n28, eq126_e1538_q_d_n29];
        let eq126_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[13]),
            nodes,
            &eq126_reactive_node_derivatives,
            branches,
            &eq126_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq128_e1552, eq128_e1552_d_n0, eq128_e1552_d_n1, eq128_e1552_d_n2, eq128_e1552_d_n3, eq128_e1552_d_n4, eq128_e1552_d_n5, eq128_e1552_d_n6, eq128_e1552_d_n7, eq128_e1552_d_n8, eq128_e1552_d_n9, eq128_e1552_d_n10, eq128_e1552_d_n11, eq128_e1552_d_n12, eq128_e1552_d_n13, eq128_e1552_d_n14, eq128_e1552_d_n15, eq128_e1552_d_n16, eq128_e1552_d_n17, eq128_e1552_d_n18, eq128_e1552_d_n19, eq128_e1552_d_n20, eq128_e1552_d_n21, eq128_e1552_d_n22, eq128_e1552_d_n23, eq128_e1552_d_n24, eq128_e1552_d_n25, eq128_e1552_d_n26, eq128_e1552_d_n27, eq128_e1552_d_n28, eq128_e1552_d_n29, eq128_e1552_q, eq128_e1552_q_d_n0, eq128_e1552_q_d_n1, eq128_e1552_q_d_n2, eq128_e1552_q_d_n3, eq128_e1552_q_d_n4, eq128_e1552_q_d_n5, eq128_e1552_q_d_n6, eq128_e1552_q_d_n7, eq128_e1552_q_d_n8, eq128_e1552_q_d_n9, eq128_e1552_q_d_n10, eq128_e1552_q_d_n11, eq128_e1552_q_d_n12, eq128_e1552_q_d_n13, eq128_e1552_q_d_n14, eq128_e1552_q_d_n15, eq128_e1552_q_d_n16, eq128_e1552_q_d_n17, eq128_e1552_q_d_n18, eq128_e1552_q_d_n19, eq128_e1552_q_d_n20, eq128_e1552_q_d_n21, eq128_e1552_q_d_n22, eq128_e1552_q_d_n23, eq128_e1552_q_d_n24, eq128_e1552_q_d_n25, eq128_e1552_q_d_n26, eq128_e1552_q_d_n27, eq128_e1552_q_d_n28, eq128_e1552_q_d_n29,) = {
    if s.b[1495] {
        let eq128_e1545_q: f64 = s.v[189];
        let eq128_e1548: f64 = (p.p355 * (nv7 - nv9));
        let eq128_e1548_d_n7: f64 = p.p355;
        let eq128_e1548_d_n9: f64 = (-p.p355);
        let eq128_e1549_q: f64 = eq128_e1548;
        let eq128_e1550: f64 = (s.v[189] + eq128_e1548);
        let eq128_e1550_d_n7: f64 = (s.dn[189][7] + eq128_e1548_d_n7);
        let eq128_e1550_d_n9: f64 = (s.dn[189][9] + eq128_e1548_d_n9);
        let eq128_e1550_q: f64 = (eq128_e1545_q + eq128_e1549_q);
        let eq128_e1550_q_d_n7: f64 = (s.dn[189][7] + eq128_e1548_d_n7);
        let eq128_e1550_q_d_n9: f64 = (s.dn[189][9] + eq128_e1548_d_n9);
        (eq128_e1550, s.dn[189][0], s.dn[189][1], s.dn[189][2], s.dn[189][3], s.dn[189][4], s.dn[189][5], s.dn[189][6], eq128_e1550_d_n7, s.dn[189][8], eq128_e1550_d_n9, s.dn[189][10], s.dn[189][11], s.dn[189][12], s.dn[189][13], s.dn[189][14], s.dn[189][15], s.dn[189][16], s.dn[189][17], s.dn[189][18], s.dn[189][19], s.dn[189][20], s.dn[189][21], s.dn[189][22], s.dn[189][23], s.dn[189][24], s.dn[189][25], s.dn[189][26], s.dn[189][27], s.dn[189][28], s.dn[189][29], eq128_e1550_q, s.dn[189][0], s.dn[189][1], s.dn[189][2], s.dn[189][3], s.dn[189][4], s.dn[189][5], s.dn[189][6], eq128_e1550_q_d_n7, s.dn[189][8], eq128_e1550_q_d_n9, s.dn[189][10], s.dn[189][11], s.dn[189][12], s.dn[189][13], s.dn[189][14], s.dn[189][15], s.dn[189][16], s.dn[189][17], s.dn[189][18], s.dn[189][19], s.dn[189][20], s.dn[189][21], s.dn[189][22], s.dn[189][23], s.dn[189][24], s.dn[189][25], s.dn[189][26], s.dn[189][27], s.dn[189][28], s.dn[189][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq128_reactive_node_derivatives: [f64; 30] = [eq128_e1552_q_d_n0, eq128_e1552_q_d_n1, eq128_e1552_q_d_n2, eq128_e1552_q_d_n3, eq128_e1552_q_d_n4, eq128_e1552_q_d_n5, eq128_e1552_q_d_n6, eq128_e1552_q_d_n7, eq128_e1552_q_d_n8, eq128_e1552_q_d_n9, eq128_e1552_q_d_n10, eq128_e1552_q_d_n11, eq128_e1552_q_d_n12, eq128_e1552_q_d_n13, eq128_e1552_q_d_n14, eq128_e1552_q_d_n15, eq128_e1552_q_d_n16, eq128_e1552_q_d_n17, eq128_e1552_q_d_n18, eq128_e1552_q_d_n19, eq128_e1552_q_d_n20, eq128_e1552_q_d_n21, eq128_e1552_q_d_n22, eq128_e1552_q_d_n23, eq128_e1552_q_d_n24, eq128_e1552_q_d_n25, eq128_e1552_q_d_n26, eq128_e1552_q_d_n27, eq128_e1552_q_d_n28, eq128_e1552_q_d_n29];
        let eq128_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq128_reactive_node_derivatives,
            branches,
            &eq128_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq129_e1563, eq129_e1563_d_n0, eq129_e1563_d_n1, eq129_e1563_d_n2, eq129_e1563_d_n3, eq129_e1563_d_n4, eq129_e1563_d_n5, eq129_e1563_d_n6, eq129_e1563_d_n7, eq129_e1563_d_n8, eq129_e1563_d_n9, eq129_e1563_d_n10, eq129_e1563_d_n11, eq129_e1563_d_n12, eq129_e1563_d_n13, eq129_e1563_d_n14, eq129_e1563_d_n15, eq129_e1563_d_n16, eq129_e1563_d_n17, eq129_e1563_d_n18, eq129_e1563_d_n19, eq129_e1563_d_n20, eq129_e1563_d_n21, eq129_e1563_d_n22, eq129_e1563_d_n23, eq129_e1563_d_n24, eq129_e1563_d_n25, eq129_e1563_d_n26, eq129_e1563_d_n27, eq129_e1563_d_n28, eq129_e1563_d_n29, eq129_e1563_q, eq129_e1563_q_d_n0, eq129_e1563_q_d_n1, eq129_e1563_q_d_n2, eq129_e1563_q_d_n3, eq129_e1563_q_d_n4, eq129_e1563_q_d_n5, eq129_e1563_q_d_n6, eq129_e1563_q_d_n7, eq129_e1563_q_d_n8, eq129_e1563_q_d_n9, eq129_e1563_q_d_n10, eq129_e1563_q_d_n11, eq129_e1563_q_d_n12, eq129_e1563_q_d_n13, eq129_e1563_q_d_n14, eq129_e1563_q_d_n15, eq129_e1563_q_d_n16, eq129_e1563_q_d_n17, eq129_e1563_q_d_n18, eq129_e1563_q_d_n19, eq129_e1563_q_d_n20, eq129_e1563_q_d_n21, eq129_e1563_q_d_n22, eq129_e1563_q_d_n23, eq129_e1563_q_d_n24, eq129_e1563_q_d_n25, eq129_e1563_q_d_n26, eq129_e1563_q_d_n27, eq129_e1563_q_d_n28, eq129_e1563_q_d_n29,) = {
    if (!s.b[1495]) {
        let eq129_e1556_q: f64 = s.v[185];
        let eq129_e1559: f64 = (p.p355 * (nv2 - nv13));
        let eq129_e1559_d_n2: f64 = p.p355;
        let eq129_e1559_d_n13: f64 = (-p.p355);
        let eq129_e1560_q: f64 = eq129_e1559;
        let eq129_e1561: f64 = (s.v[185] + eq129_e1559);
        let eq129_e1561_d_n2: f64 = (s.dn[185][2] + eq129_e1559_d_n2);
        let eq129_e1561_d_n13: f64 = (s.dn[185][13] + eq129_e1559_d_n13);
        let eq129_e1561_q: f64 = (eq129_e1556_q + eq129_e1560_q);
        let eq129_e1561_q_d_n2: f64 = (s.dn[185][2] + eq129_e1559_d_n2);
        let eq129_e1561_q_d_n13: f64 = (s.dn[185][13] + eq129_e1559_d_n13);
        (eq129_e1561, s.dn[185][0], s.dn[185][1], eq129_e1561_d_n2, s.dn[185][3], s.dn[185][4], s.dn[185][5], s.dn[185][6], s.dn[185][7], s.dn[185][8], s.dn[185][9], s.dn[185][10], s.dn[185][11], s.dn[185][12], eq129_e1561_d_n13, s.dn[185][14], s.dn[185][15], s.dn[185][16], s.dn[185][17], s.dn[185][18], s.dn[185][19], s.dn[185][20], s.dn[185][21], s.dn[185][22], s.dn[185][23], s.dn[185][24], s.dn[185][25], s.dn[185][26], s.dn[185][27], s.dn[185][28], s.dn[185][29], eq129_e1561_q, s.dn[185][0], s.dn[185][1], eq129_e1561_q_d_n2, s.dn[185][3], s.dn[185][4], s.dn[185][5], s.dn[185][6], s.dn[185][7], s.dn[185][8], s.dn[185][9], s.dn[185][10], s.dn[185][11], s.dn[185][12], eq129_e1561_q_d_n13, s.dn[185][14], s.dn[185][15], s.dn[185][16], s.dn[185][17], s.dn[185][18], s.dn[185][19], s.dn[185][20], s.dn[185][21], s.dn[185][22], s.dn[185][23], s.dn[185][24], s.dn[185][25], s.dn[185][26], s.dn[185][27], s.dn[185][28], s.dn[185][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq129_reactive_node_derivatives: [f64; 30] = [eq129_e1563_q_d_n0, eq129_e1563_q_d_n1, eq129_e1563_q_d_n2, eq129_e1563_q_d_n3, eq129_e1563_q_d_n4, eq129_e1563_q_d_n5, eq129_e1563_q_d_n6, eq129_e1563_q_d_n7, eq129_e1563_q_d_n8, eq129_e1563_q_d_n9, eq129_e1563_q_d_n10, eq129_e1563_q_d_n11, eq129_e1563_q_d_n12, eq129_e1563_q_d_n13, eq129_e1563_q_d_n14, eq129_e1563_q_d_n15, eq129_e1563_q_d_n16, eq129_e1563_q_d_n17, eq129_e1563_q_d_n18, eq129_e1563_q_d_n19, eq129_e1563_q_d_n20, eq129_e1563_q_d_n21, eq129_e1563_q_d_n22, eq129_e1563_q_d_n23, eq129_e1563_q_d_n24, eq129_e1563_q_d_n25, eq129_e1563_q_d_n26, eq129_e1563_q_d_n27, eq129_e1563_q_d_n28, eq129_e1563_q_d_n29];
        let eq129_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[13]),
            nodes,
            &eq129_reactive_node_derivatives,
            branches,
            &eq129_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq130_e1574, eq130_e1574_d_n0, eq130_e1574_d_n1, eq130_e1574_d_n2, eq130_e1574_d_n3, eq130_e1574_d_n4, eq130_e1574_d_n5, eq130_e1574_d_n6, eq130_e1574_d_n7, eq130_e1574_d_n8, eq130_e1574_d_n9, eq130_e1574_d_n10, eq130_e1574_d_n11, eq130_e1574_d_n12, eq130_e1574_d_n13, eq130_e1574_d_n14, eq130_e1574_d_n15, eq130_e1574_d_n16, eq130_e1574_d_n17, eq130_e1574_d_n18, eq130_e1574_d_n19, eq130_e1574_d_n20, eq130_e1574_d_n21, eq130_e1574_d_n22, eq130_e1574_d_n23, eq130_e1574_d_n24, eq130_e1574_d_n25, eq130_e1574_d_n26, eq130_e1574_d_n27, eq130_e1574_d_n28, eq130_e1574_d_n29, eq130_e1574_q, eq130_e1574_q_d_n0, eq130_e1574_q_d_n1, eq130_e1574_q_d_n2, eq130_e1574_q_d_n3, eq130_e1574_q_d_n4, eq130_e1574_q_d_n5, eq130_e1574_q_d_n6, eq130_e1574_q_d_n7, eq130_e1574_q_d_n8, eq130_e1574_q_d_n9, eq130_e1574_q_d_n10, eq130_e1574_q_d_n11, eq130_e1574_q_d_n12, eq130_e1574_q_d_n13, eq130_e1574_q_d_n14, eq130_e1574_q_d_n15, eq130_e1574_q_d_n16, eq130_e1574_q_d_n17, eq130_e1574_q_d_n18, eq130_e1574_q_d_n19, eq130_e1574_q_d_n20, eq130_e1574_q_d_n21, eq130_e1574_q_d_n22, eq130_e1574_q_d_n23, eq130_e1574_q_d_n24, eq130_e1574_q_d_n25, eq130_e1574_q_d_n26, eq130_e1574_q_d_n27, eq130_e1574_q_d_n28, eq130_e1574_q_d_n29,) = {
    if (!s.b[1495]) {
        let eq130_e1567_q: f64 = s.v[186];
        let eq130_e1570: f64 = (p.p355 * (nv2 - nv12));
        let eq130_e1570_d_n2: f64 = p.p355;
        let eq130_e1570_d_n12: f64 = (-p.p355);
        let eq130_e1571_q: f64 = eq130_e1570;
        let eq130_e1572: f64 = (s.v[186] + eq130_e1570);
        let eq130_e1572_d_n2: f64 = (s.dn[186][2] + eq130_e1570_d_n2);
        let eq130_e1572_d_n12: f64 = (s.dn[186][12] + eq130_e1570_d_n12);
        let eq130_e1572_q: f64 = (eq130_e1567_q + eq130_e1571_q);
        let eq130_e1572_q_d_n2: f64 = (s.dn[186][2] + eq130_e1570_d_n2);
        let eq130_e1572_q_d_n12: f64 = (s.dn[186][12] + eq130_e1570_d_n12);
        (eq130_e1572, s.dn[186][0], s.dn[186][1], eq130_e1572_d_n2, s.dn[186][3], s.dn[186][4], s.dn[186][5], s.dn[186][6], s.dn[186][7], s.dn[186][8], s.dn[186][9], s.dn[186][10], s.dn[186][11], eq130_e1572_d_n12, s.dn[186][13], s.dn[186][14], s.dn[186][15], s.dn[186][16], s.dn[186][17], s.dn[186][18], s.dn[186][19], s.dn[186][20], s.dn[186][21], s.dn[186][22], s.dn[186][23], s.dn[186][24], s.dn[186][25], s.dn[186][26], s.dn[186][27], s.dn[186][28], s.dn[186][29], eq130_e1572_q, s.dn[186][0], s.dn[186][1], eq130_e1572_q_d_n2, s.dn[186][3], s.dn[186][4], s.dn[186][5], s.dn[186][6], s.dn[186][7], s.dn[186][8], s.dn[186][9], s.dn[186][10], s.dn[186][11], eq130_e1572_q_d_n12, s.dn[186][13], s.dn[186][14], s.dn[186][15], s.dn[186][16], s.dn[186][17], s.dn[186][18], s.dn[186][19], s.dn[186][20], s.dn[186][21], s.dn[186][22], s.dn[186][23], s.dn[186][24], s.dn[186][25], s.dn[186][26], s.dn[186][27], s.dn[186][28], s.dn[186][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq130_reactive_node_derivatives: [f64; 30] = [eq130_e1574_q_d_n0, eq130_e1574_q_d_n1, eq130_e1574_q_d_n2, eq130_e1574_q_d_n3, eq130_e1574_q_d_n4, eq130_e1574_q_d_n5, eq130_e1574_q_d_n6, eq130_e1574_q_d_n7, eq130_e1574_q_d_n8, eq130_e1574_q_d_n9, eq130_e1574_q_d_n10, eq130_e1574_q_d_n11, eq130_e1574_q_d_n12, eq130_e1574_q_d_n13, eq130_e1574_q_d_n14, eq130_e1574_q_d_n15, eq130_e1574_q_d_n16, eq130_e1574_q_d_n17, eq130_e1574_q_d_n18, eq130_e1574_q_d_n19, eq130_e1574_q_d_n20, eq130_e1574_q_d_n21, eq130_e1574_q_d_n22, eq130_e1574_q_d_n23, eq130_e1574_q_d_n24, eq130_e1574_q_d_n25, eq130_e1574_q_d_n26, eq130_e1574_q_d_n27, eq130_e1574_q_d_n28, eq130_e1574_q_d_n29];
        let eq130_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[12]),
            nodes,
            &eq130_reactive_node_derivatives,
            branches,
            &eq130_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_5(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv28 = ctx.node_voltage(nodes[28]);
        let nv29 = ctx.node_voltage(nodes[29]);
        let (eq131_e1585, eq131_e1585_d_n0, eq131_e1585_d_n1, eq131_e1585_d_n2, eq131_e1585_d_n3, eq131_e1585_d_n4, eq131_e1585_d_n5, eq131_e1585_d_n6, eq131_e1585_d_n7, eq131_e1585_d_n8, eq131_e1585_d_n9, eq131_e1585_d_n10, eq131_e1585_d_n11, eq131_e1585_d_n12, eq131_e1585_d_n13, eq131_e1585_d_n14, eq131_e1585_d_n15, eq131_e1585_d_n16, eq131_e1585_d_n17, eq131_e1585_d_n18, eq131_e1585_d_n19, eq131_e1585_d_n20, eq131_e1585_d_n21, eq131_e1585_d_n22, eq131_e1585_d_n23, eq131_e1585_d_n24, eq131_e1585_d_n25, eq131_e1585_d_n26, eq131_e1585_d_n27, eq131_e1585_d_n28, eq131_e1585_d_n29, eq131_e1585_q, eq131_e1585_q_d_n0, eq131_e1585_q_d_n1, eq131_e1585_q_d_n2, eq131_e1585_q_d_n3, eq131_e1585_q_d_n4, eq131_e1585_q_d_n5, eq131_e1585_q_d_n6, eq131_e1585_q_d_n7, eq131_e1585_q_d_n8, eq131_e1585_q_d_n9, eq131_e1585_q_d_n10, eq131_e1585_q_d_n11, eq131_e1585_q_d_n12, eq131_e1585_q_d_n13, eq131_e1585_q_d_n14, eq131_e1585_q_d_n15, eq131_e1585_q_d_n16, eq131_e1585_q_d_n17, eq131_e1585_q_d_n18, eq131_e1585_q_d_n19, eq131_e1585_q_d_n20, eq131_e1585_q_d_n21, eq131_e1585_q_d_n22, eq131_e1585_q_d_n23, eq131_e1585_q_d_n24, eq131_e1585_q_d_n25, eq131_e1585_q_d_n26, eq131_e1585_q_d_n27, eq131_e1585_q_d_n28, eq131_e1585_q_d_n29,) = {
    if (!s.b[1495]) {
        let eq131_e1578_q: f64 = s.v[187];
        let eq131_e1581: f64 = (p.p355 * (nv7 - nv13));
        let eq131_e1581_d_n7: f64 = p.p355;
        let eq131_e1581_d_n13: f64 = (-p.p355);
        let eq131_e1582_q: f64 = eq131_e1581;
        let eq131_e1583: f64 = (s.v[187] + eq131_e1581);
        let eq131_e1583_d_n7: f64 = (s.dn[187][7] + eq131_e1581_d_n7);
        let eq131_e1583_d_n13: f64 = (s.dn[187][13] + eq131_e1581_d_n13);
        let eq131_e1583_q: f64 = (eq131_e1578_q + eq131_e1582_q);
        let eq131_e1583_q_d_n7: f64 = (s.dn[187][7] + eq131_e1581_d_n7);
        let eq131_e1583_q_d_n13: f64 = (s.dn[187][13] + eq131_e1581_d_n13);
        (eq131_e1583, s.dn[187][0], s.dn[187][1], s.dn[187][2], s.dn[187][3], s.dn[187][4], s.dn[187][5], s.dn[187][6], eq131_e1583_d_n7, s.dn[187][8], s.dn[187][9], s.dn[187][10], s.dn[187][11], s.dn[187][12], eq131_e1583_d_n13, s.dn[187][14], s.dn[187][15], s.dn[187][16], s.dn[187][17], s.dn[187][18], s.dn[187][19], s.dn[187][20], s.dn[187][21], s.dn[187][22], s.dn[187][23], s.dn[187][24], s.dn[187][25], s.dn[187][26], s.dn[187][27], s.dn[187][28], s.dn[187][29], eq131_e1583_q, s.dn[187][0], s.dn[187][1], s.dn[187][2], s.dn[187][3], s.dn[187][4], s.dn[187][5], s.dn[187][6], eq131_e1583_q_d_n7, s.dn[187][8], s.dn[187][9], s.dn[187][10], s.dn[187][11], s.dn[187][12], eq131_e1583_q_d_n13, s.dn[187][14], s.dn[187][15], s.dn[187][16], s.dn[187][17], s.dn[187][18], s.dn[187][19], s.dn[187][20], s.dn[187][21], s.dn[187][22], s.dn[187][23], s.dn[187][24], s.dn[187][25], s.dn[187][26], s.dn[187][27], s.dn[187][28], s.dn[187][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq131_reactive_node_derivatives: [f64; 30] = [eq131_e1585_q_d_n0, eq131_e1585_q_d_n1, eq131_e1585_q_d_n2, eq131_e1585_q_d_n3, eq131_e1585_q_d_n4, eq131_e1585_q_d_n5, eq131_e1585_q_d_n6, eq131_e1585_q_d_n7, eq131_e1585_q_d_n8, eq131_e1585_q_d_n9, eq131_e1585_q_d_n10, eq131_e1585_q_d_n11, eq131_e1585_q_d_n12, eq131_e1585_q_d_n13, eq131_e1585_q_d_n14, eq131_e1585_q_d_n15, eq131_e1585_q_d_n16, eq131_e1585_q_d_n17, eq131_e1585_q_d_n18, eq131_e1585_q_d_n19, eq131_e1585_q_d_n20, eq131_e1585_q_d_n21, eq131_e1585_q_d_n22, eq131_e1585_q_d_n23, eq131_e1585_q_d_n24, eq131_e1585_q_d_n25, eq131_e1585_q_d_n26, eq131_e1585_q_d_n27, eq131_e1585_q_d_n28, eq131_e1585_q_d_n29];
        let eq131_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[13]),
            nodes,
            &eq131_reactive_node_derivatives,
            branches,
            &eq131_reactive_branch_derivatives,
            multiplicity,
        );
        let eq134_e1597_q: f64 = s.v[188];
        let eq134_e1600: f64 = (p.p355 * (nv3 - nv13));
        let eq134_e1600_d_n3: f64 = p.p355;
        let eq134_e1600_d_n13: f64 = (-p.p355);
        let eq134_e1601_q: f64 = eq134_e1600;
        let eq134_e1602: f64 = (s.v[188] + eq134_e1600);
        let eq134_e1602_d_n3: f64 = (s.dn[188][3] + eq134_e1600_d_n3);
        let eq134_e1602_d_n13: f64 = (s.dn[188][13] + eq134_e1600_d_n13);
        let eq134_e1602_q: f64 = (eq134_e1597_q + eq134_e1601_q);
        let eq134_e1602_q_d_n3: f64 = (s.dn[188][3] + eq134_e1600_d_n3);
        let eq134_e1602_q_d_n13: f64 = (s.dn[188][13] + eq134_e1600_d_n13);
        let eq134_reactive_node_derivatives: [f64; 30] = [s.dn[188][0], s.dn[188][1], s.dn[188][2], eq134_e1602_q_d_n3, s.dn[188][4], s.dn[188][5], s.dn[188][6], s.dn[188][7], s.dn[188][8], s.dn[188][9], s.dn[188][10], s.dn[188][11], s.dn[188][12], eq134_e1602_q_d_n13, s.dn[188][14], s.dn[188][15], s.dn[188][16], s.dn[188][17], s.dn[188][18], s.dn[188][19], s.dn[188][20], s.dn[188][21], s.dn[188][22], s.dn[188][23], s.dn[188][24], s.dn[188][25], s.dn[188][26], s.dn[188][27], s.dn[188][28], s.dn[188][29]];
        let eq134_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[13]),
            nodes,
            &eq134_reactive_node_derivatives,
            branches,
            &eq134_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq142_e1656, eq142_e1656_d_n0, eq142_e1656_d_n1, eq142_e1656_d_n2, eq142_e1656_d_n3, eq142_e1656_d_n4, eq142_e1656_d_n5, eq142_e1656_d_n6, eq142_e1656_d_n7, eq142_e1656_d_n8, eq142_e1656_d_n9, eq142_e1656_d_n10, eq142_e1656_d_n11, eq142_e1656_d_n12, eq142_e1656_d_n13, eq142_e1656_d_n14, eq142_e1656_d_n15, eq142_e1656_d_n16, eq142_e1656_d_n17, eq142_e1656_d_n18, eq142_e1656_d_n19, eq142_e1656_d_n20, eq142_e1656_d_n21, eq142_e1656_d_n22, eq142_e1656_d_n23, eq142_e1656_d_n24, eq142_e1656_d_n25, eq142_e1656_d_n26, eq142_e1656_d_n27, eq142_e1656_d_n28, eq142_e1656_d_n29, eq142_e1656_q, eq142_e1656_q_d_n28,) = {
    if (!s.b[1933]) {
        let eq142_e1649: f64 = (s.v[115] - (nv29 - 0.0));
        let eq142_e1649_d_n29: f64 = (s.dn[115][29] - 1.0);
        let eq142_e1652: f64 = (p.p323 * (nv28 - 0.0));
        let eq142_e1652_d_n28: f64 = p.p323;
        let eq142_e1653_q: f64 = eq142_e1652;
        let eq142_e1654: f64 = (eq142_e1649 - eq142_e1652);
        let eq142_e1654_d_n28: f64 = (s.dn[115][28] - eq142_e1652_d_n28);
        let eq142_e1654_q: f64 = (-eq142_e1653_q);
        let eq142_e1654_q_d_n28: f64 = (-eq142_e1652_d_n28);
        (eq142_e1654, s.dn[115][0], s.dn[115][1], s.dn[115][2], s.dn[115][3], s.dn[115][4], s.dn[115][5], s.dn[115][6], s.dn[115][7], s.dn[115][8], s.dn[115][9], s.dn[115][10], s.dn[115][11], s.dn[115][12], s.dn[115][13], s.dn[115][14], s.dn[115][15], s.dn[115][16], s.dn[115][17], s.dn[115][18], s.dn[115][19], s.dn[115][20], s.dn[115][21], s.dn[115][22], s.dn[115][23], s.dn[115][24], s.dn[115][25], s.dn[115][26], s.dn[115][27], eq142_e1654_d_n28, eq142_e1649_d_n29, eq142_e1654_q, eq142_e1654_q_d_n28,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[28]),
            None,
            nodes[28],
            multiplicity * (eq142_e1656_q_d_n28),
        );
        let (eq143_e1670, eq143_e1670_d_n28, eq143_e1670_d_n29, eq143_e1670_q, eq143_e1670_q_d_n29,) = {
    if (!s.b[1933]) {
        let eq143_e1661: f64 = ((nv28 - 0.0) - (nv29 - 0.0));
        let eq143_e1661_d_n29: f64 = (-1.0);
        let eq143_e1664: f64 = (p.p323 / 3.0);
        let eq143_e1666: f64 = (eq143_e1664 * (nv29 - 0.0));
        let eq143_e1667_q: f64 = eq143_e1666;
        let eq143_e1668: f64 = (eq143_e1661 - eq143_e1666);
        let eq143_e1668_d_n29: f64 = (eq143_e1661_d_n29 - eq143_e1664);
        let eq143_e1668_q: f64 = (-eq143_e1667_q);
        let eq143_e1668_q_d_n29: f64 = (-eq143_e1664);
        (eq143_e1668, 1.0, eq143_e1668_d_n29, eq143_e1668_q, eq143_e1668_q_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[29]),
            None,
            nodes[29],
            multiplicity * (eq143_e1670_q_d_n29),
        );
        let eq145_e1681_q: f64 = s.v[117];
        let eq145_e1684: f64 = (p.p355 * (nv8 - nv9));
        let eq145_e1684_d_n8: f64 = p.p355;
        let eq145_e1684_d_n9: f64 = (-p.p355);
        let eq145_e1685_q: f64 = eq145_e1684;
        let eq145_e1686: f64 = (s.v[117] + eq145_e1684);
        let eq145_e1686_d_n8: f64 = (s.dn[117][8] + eq145_e1684_d_n8);
        let eq145_e1686_d_n9: f64 = (s.dn[117][9] + eq145_e1684_d_n9);
        let eq145_e1686_q: f64 = (eq145_e1681_q + eq145_e1685_q);
        let eq145_e1686_q_d_n8: f64 = (s.dn[117][8] + eq145_e1684_d_n8);
        let eq145_e1686_q_d_n9: f64 = (s.dn[117][9] + eq145_e1684_d_n9);
        let eq145_reactive_node_derivatives: [f64; 30] = [s.dn[117][0], s.dn[117][1], s.dn[117][2], s.dn[117][3], s.dn[117][4], s.dn[117][5], s.dn[117][6], s.dn[117][7], eq145_e1686_q_d_n8, eq145_e1686_q_d_n9, s.dn[117][10], s.dn[117][11], s.dn[117][12], s.dn[117][13], s.dn[117][14], s.dn[117][15], s.dn[117][16], s.dn[117][17], s.dn[117][18], s.dn[117][19], s.dn[117][20], s.dn[117][21], s.dn[117][22], s.dn[117][23], s.dn[117][24], s.dn[117][25], s.dn[117][26], s.dn[117][27], s.dn[117][28], s.dn[117][29]];
        let eq145_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[9]),
            nodes,
            &eq145_reactive_node_derivatives,
            branches,
            &eq145_reactive_branch_derivatives,
            multiplicity,
        );
        let eq146_e1688_q: f64 = s.v[118];
        let eq146_e1691: f64 = (p.p355 * (nv8 - nv5));
        let eq146_e1691_d_n5: f64 = (-p.p355);
        let eq146_e1691_d_n8: f64 = p.p355;
        let eq146_e1692_q: f64 = eq146_e1691;
        let eq146_e1693: f64 = (s.v[118] + eq146_e1691);
        let eq146_e1693_d_n5: f64 = (s.dn[118][5] + eq146_e1691_d_n5);
        let eq146_e1693_d_n8: f64 = (s.dn[118][8] + eq146_e1691_d_n8);
        let eq146_e1693_q: f64 = (eq146_e1688_q + eq146_e1692_q);
        let eq146_e1693_q_d_n5: f64 = (s.dn[118][5] + eq146_e1691_d_n5);
        let eq146_e1693_q_d_n8: f64 = (s.dn[118][8] + eq146_e1691_d_n8);
        let eq146_reactive_node_derivatives: [f64; 30] = [s.dn[118][0], s.dn[118][1], s.dn[118][2], s.dn[118][3], s.dn[118][4], eq146_e1693_q_d_n5, s.dn[118][6], s.dn[118][7], eq146_e1693_q_d_n8, s.dn[118][9], s.dn[118][10], s.dn[118][11], s.dn[118][12], s.dn[118][13], s.dn[118][14], s.dn[118][15], s.dn[118][16], s.dn[118][17], s.dn[118][18], s.dn[118][19], s.dn[118][20], s.dn[118][21], s.dn[118][22], s.dn[118][23], s.dn[118][24], s.dn[118][25], s.dn[118][26], s.dn[118][27], s.dn[118][28], s.dn[118][29]];
        let eq146_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &eq146_reactive_node_derivatives,
            branches,
            &eq146_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq157_e1796, eq157_e1796_d_n0, eq157_e1796_d_n1, eq157_e1796_d_n2, eq157_e1796_d_n3, eq157_e1796_d_n4, eq157_e1796_d_n5, eq157_e1796_d_n6, eq157_e1796_d_n7, eq157_e1796_d_n8, eq157_e1796_d_n9, eq157_e1796_d_n10, eq157_e1796_d_n11, eq157_e1796_d_n12, eq157_e1796_d_n13, eq157_e1796_d_n14, eq157_e1796_d_n15, eq157_e1796_d_n16, eq157_e1796_d_n17, eq157_e1796_d_n18, eq157_e1796_d_n19, eq157_e1796_d_n20, eq157_e1796_d_n21, eq157_e1796_d_n22, eq157_e1796_d_n23, eq157_e1796_d_n24, eq157_e1796_d_n25, eq157_e1796_d_n26, eq157_e1796_d_n27, eq157_e1796_d_n28, eq157_e1796_d_n29, eq157_e1796_q, eq157_e1796_q_d_n0, eq157_e1796_q_d_n1, eq157_e1796_q_d_n2, eq157_e1796_q_d_n3, eq157_e1796_q_d_n4, eq157_e1796_q_d_n5, eq157_e1796_q_d_n6, eq157_e1796_q_d_n7, eq157_e1796_q_d_n8, eq157_e1796_q_d_n9, eq157_e1796_q_d_n10, eq157_e1796_q_d_n11, eq157_e1796_q_d_n12, eq157_e1796_q_d_n13, eq157_e1796_q_d_n14, eq157_e1796_q_d_n15, eq157_e1796_q_d_n16, eq157_e1796_q_d_n17, eq157_e1796_q_d_n18, eq157_e1796_q_d_n19, eq157_e1796_q_d_n20, eq157_e1796_q_d_n21, eq157_e1796_q_d_n22, eq157_e1796_q_d_n23, eq157_e1796_q_d_n24, eq157_e1796_q_d_n25, eq157_e1796_q_d_n26, eq157_e1796_q_d_n27, eq157_e1796_q_d_n28, eq157_e1796_q_d_n29,) = {
    if s.b[2418] {
        let eq157_e1794_q: f64 = s.v[242];
        (s.v[242], s.dn[242][0], s.dn[242][1], s.dn[242][2], s.dn[242][3], s.dn[242][4], s.dn[242][5], s.dn[242][6], s.dn[242][7], s.dn[242][8], s.dn[242][9], s.dn[242][10], s.dn[242][11], s.dn[242][12], s.dn[242][13], s.dn[242][14], s.dn[242][15], s.dn[242][16], s.dn[242][17], s.dn[242][18], s.dn[242][19], s.dn[242][20], s.dn[242][21], s.dn[242][22], s.dn[242][23], s.dn[242][24], s.dn[242][25], s.dn[242][26], s.dn[242][27], s.dn[242][28], s.dn[242][29], eq157_e1794_q, s.dn[242][0], s.dn[242][1], s.dn[242][2], s.dn[242][3], s.dn[242][4], s.dn[242][5], s.dn[242][6], s.dn[242][7], s.dn[242][8], s.dn[242][9], s.dn[242][10], s.dn[242][11], s.dn[242][12], s.dn[242][13], s.dn[242][14], s.dn[242][15], s.dn[242][16], s.dn[242][17], s.dn[242][18], s.dn[242][19], s.dn[242][20], s.dn[242][21], s.dn[242][22], s.dn[242][23], s.dn[242][24], s.dn[242][25], s.dn[242][26], s.dn[242][27], s.dn[242][28], s.dn[242][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq157_reactive_node_derivatives: [f64; 30] = [eq157_e1796_q_d_n0, eq157_e1796_q_d_n1, eq157_e1796_q_d_n2, eq157_e1796_q_d_n3, eq157_e1796_q_d_n4, eq157_e1796_q_d_n5, eq157_e1796_q_d_n6, eq157_e1796_q_d_n7, eq157_e1796_q_d_n8, eq157_e1796_q_d_n9, eq157_e1796_q_d_n10, eq157_e1796_q_d_n11, eq157_e1796_q_d_n12, eq157_e1796_q_d_n13, eq157_e1796_q_d_n14, eq157_e1796_q_d_n15, eq157_e1796_q_d_n16, eq157_e1796_q_d_n17, eq157_e1796_q_d_n18, eq157_e1796_q_d_n19, eq157_e1796_q_d_n20, eq157_e1796_q_d_n21, eq157_e1796_q_d_n22, eq157_e1796_q_d_n23, eq157_e1796_q_d_n24, eq157_e1796_q_d_n25, eq157_e1796_q_d_n26, eq157_e1796_q_d_n27, eq157_e1796_q_d_n28, eq157_e1796_q_d_n29];
        let eq157_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq157_reactive_node_derivatives,
            branches,
            &eq157_reactive_branch_derivatives,
            multiplicity,
        );
        let eq172_e1881_q: f64 = s.v[214];
        let eq172_reactive_node_derivatives: [f64; 30] = [s.dn[214][0], s.dn[214][1], s.dn[214][2], s.dn[214][3], s.dn[214][4], s.dn[214][5], s.dn[214][6], s.dn[214][7], s.dn[214][8], s.dn[214][9], s.dn[214][10], s.dn[214][11], s.dn[214][12], s.dn[214][13], s.dn[214][14], s.dn[214][15], s.dn[214][16], s.dn[214][17], s.dn[214][18], s.dn[214][19], s.dn[214][20], s.dn[214][21], s.dn[214][22], s.dn[214][23], s.dn[214][24], s.dn[214][25], s.dn[214][26], s.dn[214][27], s.dn[214][28], s.dn[214][29]];
        let eq172_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[2]),
            nodes,
            &eq172_reactive_node_derivatives,
            branches,
            &eq172_reactive_branch_derivatives,
            multiplicity,
        );
        let eq173_e1883_q: f64 = s.v[215];
        let eq173_reactive_node_derivatives: [f64; 30] = [s.dn[215][0], s.dn[215][1], s.dn[215][2], s.dn[215][3], s.dn[215][4], s.dn[215][5], s.dn[215][6], s.dn[215][7], s.dn[215][8], s.dn[215][9], s.dn[215][10], s.dn[215][11], s.dn[215][12], s.dn[215][13], s.dn[215][14], s.dn[215][15], s.dn[215][16], s.dn[215][17], s.dn[215][18], s.dn[215][19], s.dn[215][20], s.dn[215][21], s.dn[215][22], s.dn[215][23], s.dn[215][24], s.dn[215][25], s.dn[215][26], s.dn[215][27], s.dn[215][28], s.dn[215][29]];
        let eq173_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[0]),
            nodes,
            &eq173_reactive_node_derivatives,
            branches,
            &eq173_reactive_branch_derivatives,
            multiplicity,
        );
        let eq174_e1885_q: f64 = s.v[216];
        let eq174_reactive_node_derivatives: [f64; 30] = [s.dn[216][0], s.dn[216][1], s.dn[216][2], s.dn[216][3], s.dn[216][4], s.dn[216][5], s.dn[216][6], s.dn[216][7], s.dn[216][8], s.dn[216][9], s.dn[216][10], s.dn[216][11], s.dn[216][12], s.dn[216][13], s.dn[216][14], s.dn[216][15], s.dn[216][16], s.dn[216][17], s.dn[216][18], s.dn[216][19], s.dn[216][20], s.dn[216][21], s.dn[216][22], s.dn[216][23], s.dn[216][24], s.dn[216][25], s.dn[216][26], s.dn[216][27], s.dn[216][28], s.dn[216][29]];
        let eq174_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[0]),
            nodes,
            &eq174_reactive_node_derivatives,
            branches,
            &eq174_reactive_branch_derivatives,
            multiplicity,
        );
        let eq175_e1887_q: f64 = s.v[218];
        let eq175_reactive_node_derivatives: [f64; 30] = [s.dn[218][0], s.dn[218][1], s.dn[218][2], s.dn[218][3], s.dn[218][4], s.dn[218][5], s.dn[218][6], s.dn[218][7], s.dn[218][8], s.dn[218][9], s.dn[218][10], s.dn[218][11], s.dn[218][12], s.dn[218][13], s.dn[218][14], s.dn[218][15], s.dn[218][16], s.dn[218][17], s.dn[218][18], s.dn[218][19], s.dn[218][20], s.dn[218][21], s.dn[218][22], s.dn[218][23], s.dn[218][24], s.dn[218][25], s.dn[218][26], s.dn[218][27], s.dn[218][28], s.dn[218][29]];
        let eq175_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[2]),
            nodes,
            &eq175_reactive_node_derivatives,
            branches,
            &eq175_reactive_branch_derivatives,
            multiplicity,
        );
        let eq176_e1889_q: f64 = s.v[217];
        let eq176_reactive_node_derivatives: [f64; 30] = [s.dn[217][0], s.dn[217][1], s.dn[217][2], s.dn[217][3], s.dn[217][4], s.dn[217][5], s.dn[217][6], s.dn[217][7], s.dn[217][8], s.dn[217][9], s.dn[217][10], s.dn[217][11], s.dn[217][12], s.dn[217][13], s.dn[217][14], s.dn[217][15], s.dn[217][16], s.dn[217][17], s.dn[217][18], s.dn[217][19], s.dn[217][20], s.dn[217][21], s.dn[217][22], s.dn[217][23], s.dn[217][24], s.dn[217][25], s.dn[217][26], s.dn[217][27], s.dn[217][28], s.dn[217][29]];
        let eq176_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[0]),
            nodes,
            &eq176_reactive_node_derivatives,
            branches,
            &eq176_reactive_branch_derivatives,
            multiplicity,
        );
        let eq177_e1891_q: f64 = s.v[219];
        let eq177_reactive_node_derivatives: [f64; 30] = [s.dn[219][0], s.dn[219][1], s.dn[219][2], s.dn[219][3], s.dn[219][4], s.dn[219][5], s.dn[219][6], s.dn[219][7], s.dn[219][8], s.dn[219][9], s.dn[219][10], s.dn[219][11], s.dn[219][12], s.dn[219][13], s.dn[219][14], s.dn[219][15], s.dn[219][16], s.dn[219][17], s.dn[219][18], s.dn[219][19], s.dn[219][20], s.dn[219][21], s.dn[219][22], s.dn[219][23], s.dn[219][24], s.dn[219][25], s.dn[219][26], s.dn[219][27], s.dn[219][28], s.dn[219][29]];
        let eq177_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[3]),
            nodes,
            &eq177_reactive_node_derivatives,
            branches,
            &eq177_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq194_e2167, eq194_e2167_d_n4, eq194_e2167_q, eq194_e2167_q_d_n4,) = {
    if s.b[2700] {
        let eq194_e2164: f64 = (p.p321 * (nv4 - 0.0));
        let eq194_e2164_d_n4: f64 = p.p321;
        let eq194_e2165_q: f64 = eq194_e2164;
        (eq194_e2164, eq194_e2164_d_n4, eq194_e2165_q, eq194_e2164_d_n4,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (eq194_e2167_q_d_n4),
        );
    }
}
