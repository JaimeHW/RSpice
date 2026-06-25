#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_100_block_0(
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq100_e1324, eq100_e1324_d_n0, eq100_e1324_d_n1, eq100_e1324_d_n2, eq100_e1324_d_n3, eq100_e1324_d_n4, eq100_e1324_d_n5, eq100_e1324_d_n6, eq100_e1324_d_n7, eq100_e1324_d_n8, eq100_e1324_d_n9, eq100_e1324_d_n10, eq100_e1324_d_n11, eq100_e1324_d_n12, eq100_e1324_d_n13, eq100_e1324_d_n14, eq100_e1324_d_n15, eq100_e1324_d_n16, eq100_e1324_d_n17, eq100_e1324_d_n18, eq100_e1324_d_n19, eq100_e1324_d_n20, eq100_e1324_d_n21, eq100_e1324_d_n22, eq100_e1324_d_n23, eq100_e1324_d_n24, eq100_e1324_d_n25, eq100_e1324_d_n26, eq100_e1324_d_n27, eq100_e1324_d_n28, eq100_e1324_d_n29, eq100_e1324_q, eq100_e1324_q_d_n0, eq100_e1324_q_d_n1, eq100_e1324_q_d_n2, eq100_e1324_q_d_n3, eq100_e1324_q_d_n4, eq100_e1324_q_d_n5, eq100_e1324_q_d_n6, eq100_e1324_q_d_n7, eq100_e1324_q_d_n8, eq100_e1324_q_d_n9, eq100_e1324_q_d_n10, eq100_e1324_q_d_n11, eq100_e1324_q_d_n12, eq100_e1324_q_d_n13, eq100_e1324_q_d_n14, eq100_e1324_q_d_n15, eq100_e1324_q_d_n16, eq100_e1324_q_d_n17, eq100_e1324_q_d_n18, eq100_e1324_q_d_n19, eq100_e1324_q_d_n20, eq100_e1324_q_d_n21, eq100_e1324_q_d_n22, eq100_e1324_q_d_n23, eq100_e1324_q_d_n24, eq100_e1324_q_d_n25, eq100_e1324_q_d_n26, eq100_e1324_q_d_n27, eq100_e1324_q_d_n28, eq100_e1324_q_d_n29,) = {
    if (s.v[1201] != 0.0) {
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
            &nodes,
            &eq100_reactive_node_derivatives,
            &branches,
            &eq100_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_102_block_0(
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
        let (eq102_e1338, eq102_e1338_d_n0, eq102_e1338_d_n1, eq102_e1338_d_n2, eq102_e1338_d_n3, eq102_e1338_d_n4, eq102_e1338_d_n5, eq102_e1338_d_n6, eq102_e1338_d_n7, eq102_e1338_d_n8, eq102_e1338_d_n9, eq102_e1338_d_n10, eq102_e1338_d_n11, eq102_e1338_d_n12, eq102_e1338_d_n13, eq102_e1338_d_n14, eq102_e1338_d_n15, eq102_e1338_d_n16, eq102_e1338_d_n17, eq102_e1338_d_n18, eq102_e1338_d_n19, eq102_e1338_d_n20, eq102_e1338_d_n21, eq102_e1338_d_n22, eq102_e1338_d_n23, eq102_e1338_d_n24, eq102_e1338_d_n25, eq102_e1338_d_n26, eq102_e1338_d_n27, eq102_e1338_d_n28, eq102_e1338_d_n29, eq102_e1338_q, eq102_e1338_q_d_n0, eq102_e1338_q_d_n1, eq102_e1338_q_d_n2, eq102_e1338_q_d_n3, eq102_e1338_q_d_n4, eq102_e1338_q_d_n5, eq102_e1338_q_d_n6, eq102_e1338_q_d_n7, eq102_e1338_q_d_n8, eq102_e1338_q_d_n9, eq102_e1338_q_d_n10, eq102_e1338_q_d_n11, eq102_e1338_q_d_n12, eq102_e1338_q_d_n13, eq102_e1338_q_d_n14, eq102_e1338_q_d_n15, eq102_e1338_q_d_n16, eq102_e1338_q_d_n17, eq102_e1338_q_d_n18, eq102_e1338_q_d_n19, eq102_e1338_q_d_n20, eq102_e1338_q_d_n21, eq102_e1338_q_d_n22, eq102_e1338_q_d_n23, eq102_e1338_q_d_n24, eq102_e1338_q_d_n25, eq102_e1338_q_d_n26, eq102_e1338_q_d_n27, eq102_e1338_q_d_n28, eq102_e1338_q_d_n29,) = {
    if (s.v[1201] != 0.0) {
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
            &nodes,
            &eq102_reactive_node_derivatives,
            &branches,
            &eq102_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_103_block_0(
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq103_e1349, eq103_e1349_d_n0, eq103_e1349_d_n1, eq103_e1349_d_n2, eq103_e1349_d_n3, eq103_e1349_d_n4, eq103_e1349_d_n5, eq103_e1349_d_n6, eq103_e1349_d_n7, eq103_e1349_d_n8, eq103_e1349_d_n9, eq103_e1349_d_n10, eq103_e1349_d_n11, eq103_e1349_d_n12, eq103_e1349_d_n13, eq103_e1349_d_n14, eq103_e1349_d_n15, eq103_e1349_d_n16, eq103_e1349_d_n17, eq103_e1349_d_n18, eq103_e1349_d_n19, eq103_e1349_d_n20, eq103_e1349_d_n21, eq103_e1349_d_n22, eq103_e1349_d_n23, eq103_e1349_d_n24, eq103_e1349_d_n25, eq103_e1349_d_n26, eq103_e1349_d_n27, eq103_e1349_d_n28, eq103_e1349_d_n29, eq103_e1349_q, eq103_e1349_q_d_n0, eq103_e1349_q_d_n1, eq103_e1349_q_d_n2, eq103_e1349_q_d_n3, eq103_e1349_q_d_n4, eq103_e1349_q_d_n5, eq103_e1349_q_d_n6, eq103_e1349_q_d_n7, eq103_e1349_q_d_n8, eq103_e1349_q_d_n9, eq103_e1349_q_d_n10, eq103_e1349_q_d_n11, eq103_e1349_q_d_n12, eq103_e1349_q_d_n13, eq103_e1349_q_d_n14, eq103_e1349_q_d_n15, eq103_e1349_q_d_n16, eq103_e1349_q_d_n17, eq103_e1349_q_d_n18, eq103_e1349_q_d_n19, eq103_e1349_q_d_n20, eq103_e1349_q_d_n21, eq103_e1349_q_d_n22, eq103_e1349_q_d_n23, eq103_e1349_q_d_n24, eq103_e1349_q_d_n25, eq103_e1349_q_d_n26, eq103_e1349_q_d_n27, eq103_e1349_q_d_n28, eq103_e1349_q_d_n29,) = {
    if (!(s.v[1201] != 0.0)) {
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
            &nodes,
            &eq103_reactive_node_derivatives,
            &branches,
            &eq103_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_104_block_0(
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
        let (eq104_e1360, eq104_e1360_d_n0, eq104_e1360_d_n1, eq104_e1360_d_n2, eq104_e1360_d_n3, eq104_e1360_d_n4, eq104_e1360_d_n5, eq104_e1360_d_n6, eq104_e1360_d_n7, eq104_e1360_d_n8, eq104_e1360_d_n9, eq104_e1360_d_n10, eq104_e1360_d_n11, eq104_e1360_d_n12, eq104_e1360_d_n13, eq104_e1360_d_n14, eq104_e1360_d_n15, eq104_e1360_d_n16, eq104_e1360_d_n17, eq104_e1360_d_n18, eq104_e1360_d_n19, eq104_e1360_d_n20, eq104_e1360_d_n21, eq104_e1360_d_n22, eq104_e1360_d_n23, eq104_e1360_d_n24, eq104_e1360_d_n25, eq104_e1360_d_n26, eq104_e1360_d_n27, eq104_e1360_d_n28, eq104_e1360_d_n29, eq104_e1360_q, eq104_e1360_q_d_n0, eq104_e1360_q_d_n1, eq104_e1360_q_d_n2, eq104_e1360_q_d_n3, eq104_e1360_q_d_n4, eq104_e1360_q_d_n5, eq104_e1360_q_d_n6, eq104_e1360_q_d_n7, eq104_e1360_q_d_n8, eq104_e1360_q_d_n9, eq104_e1360_q_d_n10, eq104_e1360_q_d_n11, eq104_e1360_q_d_n12, eq104_e1360_q_d_n13, eq104_e1360_q_d_n14, eq104_e1360_q_d_n15, eq104_e1360_q_d_n16, eq104_e1360_q_d_n17, eq104_e1360_q_d_n18, eq104_e1360_q_d_n19, eq104_e1360_q_d_n20, eq104_e1360_q_d_n21, eq104_e1360_q_d_n22, eq104_e1360_q_d_n23, eq104_e1360_q_d_n24, eq104_e1360_q_d_n25, eq104_e1360_q_d_n26, eq104_e1360_q_d_n27, eq104_e1360_q_d_n28, eq104_e1360_q_d_n29,) = {
    if (!(s.v[1201] != 0.0)) {
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
            &nodes,
            &eq104_reactive_node_derivatives,
            &branches,
            &eq104_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_105_block_0(
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
        let (eq105_e1371, eq105_e1371_d_n0, eq105_e1371_d_n1, eq105_e1371_d_n2, eq105_e1371_d_n3, eq105_e1371_d_n4, eq105_e1371_d_n5, eq105_e1371_d_n6, eq105_e1371_d_n7, eq105_e1371_d_n8, eq105_e1371_d_n9, eq105_e1371_d_n10, eq105_e1371_d_n11, eq105_e1371_d_n12, eq105_e1371_d_n13, eq105_e1371_d_n14, eq105_e1371_d_n15, eq105_e1371_d_n16, eq105_e1371_d_n17, eq105_e1371_d_n18, eq105_e1371_d_n19, eq105_e1371_d_n20, eq105_e1371_d_n21, eq105_e1371_d_n22, eq105_e1371_d_n23, eq105_e1371_d_n24, eq105_e1371_d_n25, eq105_e1371_d_n26, eq105_e1371_d_n27, eq105_e1371_d_n28, eq105_e1371_d_n29, eq105_e1371_q, eq105_e1371_q_d_n0, eq105_e1371_q_d_n1, eq105_e1371_q_d_n2, eq105_e1371_q_d_n3, eq105_e1371_q_d_n4, eq105_e1371_q_d_n5, eq105_e1371_q_d_n6, eq105_e1371_q_d_n7, eq105_e1371_q_d_n8, eq105_e1371_q_d_n9, eq105_e1371_q_d_n10, eq105_e1371_q_d_n11, eq105_e1371_q_d_n12, eq105_e1371_q_d_n13, eq105_e1371_q_d_n14, eq105_e1371_q_d_n15, eq105_e1371_q_d_n16, eq105_e1371_q_d_n17, eq105_e1371_q_d_n18, eq105_e1371_q_d_n19, eq105_e1371_q_d_n20, eq105_e1371_q_d_n21, eq105_e1371_q_d_n22, eq105_e1371_q_d_n23, eq105_e1371_q_d_n24, eq105_e1371_q_d_n25, eq105_e1371_q_d_n26, eq105_e1371_q_d_n27, eq105_e1371_q_d_n28, eq105_e1371_q_d_n29,) = {
    if (!(s.v[1201] != 0.0)) {
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
            &nodes,
            &eq105_reactive_node_derivatives,
            &branches,
            &eq105_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_108_block_0(
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
        let nv11 = ctx.node_voltage(nodes[11]);
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
            &nodes,
            &eq108_reactive_node_derivatives,
            &branches,
            &eq108_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_111_block_0(
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq111_e1411, eq111_e1411_d_n0, eq111_e1411_d_n1, eq111_e1411_d_n2, eq111_e1411_d_n3, eq111_e1411_d_n4, eq111_e1411_d_n5, eq111_e1411_d_n6, eq111_e1411_d_n7, eq111_e1411_d_n8, eq111_e1411_d_n9, eq111_e1411_d_n10, eq111_e1411_d_n11, eq111_e1411_d_n12, eq111_e1411_d_n13, eq111_e1411_d_n14, eq111_e1411_d_n15, eq111_e1411_d_n16, eq111_e1411_d_n17, eq111_e1411_d_n18, eq111_e1411_d_n19, eq111_e1411_d_n20, eq111_e1411_d_n21, eq111_e1411_d_n22, eq111_e1411_d_n23, eq111_e1411_d_n24, eq111_e1411_d_n25, eq111_e1411_d_n26, eq111_e1411_d_n27, eq111_e1411_d_n28, eq111_e1411_d_n29, eq111_e1411_q, eq111_e1411_q_d_n0, eq111_e1411_q_d_n1, eq111_e1411_q_d_n2, eq111_e1411_q_d_n3, eq111_e1411_q_d_n4, eq111_e1411_q_d_n5, eq111_e1411_q_d_n6, eq111_e1411_q_d_n7, eq111_e1411_q_d_n8, eq111_e1411_q_d_n9, eq111_e1411_q_d_n10, eq111_e1411_q_d_n11, eq111_e1411_q_d_n12, eq111_e1411_q_d_n13, eq111_e1411_q_d_n14, eq111_e1411_q_d_n15, eq111_e1411_q_d_n16, eq111_e1411_q_d_n17, eq111_e1411_q_d_n18, eq111_e1411_q_d_n19, eq111_e1411_q_d_n20, eq111_e1411_q_d_n21, eq111_e1411_q_d_n22, eq111_e1411_q_d_n23, eq111_e1411_q_d_n24, eq111_e1411_q_d_n25, eq111_e1411_q_d_n26, eq111_e1411_q_d_n27, eq111_e1411_q_d_n28, eq111_e1411_q_d_n29,) = {
    if (s.v[1348] != 0.0) {
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
            &nodes,
            &eq111_reactive_node_derivatives,
            &branches,
            &eq111_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_112_block_0(
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
        let (eq112_e1421, eq112_e1421_d_n0, eq112_e1421_d_n1, eq112_e1421_d_n2, eq112_e1421_d_n3, eq112_e1421_d_n4, eq112_e1421_d_n5, eq112_e1421_d_n6, eq112_e1421_d_n7, eq112_e1421_d_n8, eq112_e1421_d_n9, eq112_e1421_d_n10, eq112_e1421_d_n11, eq112_e1421_d_n12, eq112_e1421_d_n13, eq112_e1421_d_n14, eq112_e1421_d_n15, eq112_e1421_d_n16, eq112_e1421_d_n17, eq112_e1421_d_n18, eq112_e1421_d_n19, eq112_e1421_d_n20, eq112_e1421_d_n21, eq112_e1421_d_n22, eq112_e1421_d_n23, eq112_e1421_d_n24, eq112_e1421_d_n25, eq112_e1421_d_n26, eq112_e1421_d_n27, eq112_e1421_d_n28, eq112_e1421_d_n29, eq112_e1421_q, eq112_e1421_q_d_n0, eq112_e1421_q_d_n1, eq112_e1421_q_d_n2, eq112_e1421_q_d_n3, eq112_e1421_q_d_n4, eq112_e1421_q_d_n5, eq112_e1421_q_d_n6, eq112_e1421_q_d_n7, eq112_e1421_q_d_n8, eq112_e1421_q_d_n9, eq112_e1421_q_d_n10, eq112_e1421_q_d_n11, eq112_e1421_q_d_n12, eq112_e1421_q_d_n13, eq112_e1421_q_d_n14, eq112_e1421_q_d_n15, eq112_e1421_q_d_n16, eq112_e1421_q_d_n17, eq112_e1421_q_d_n18, eq112_e1421_q_d_n19, eq112_e1421_q_d_n20, eq112_e1421_q_d_n21, eq112_e1421_q_d_n22, eq112_e1421_q_d_n23, eq112_e1421_q_d_n24, eq112_e1421_q_d_n25, eq112_e1421_q_d_n26, eq112_e1421_q_d_n27, eq112_e1421_q_d_n28, eq112_e1421_q_d_n29,) = {
    if (s.v[1348] != 0.0) {
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
            &nodes,
            &eq112_reactive_node_derivatives,
            &branches,
            &eq112_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_113_block_0(
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq113_e1431, eq113_e1431_d_n0, eq113_e1431_d_n1, eq113_e1431_d_n2, eq113_e1431_d_n3, eq113_e1431_d_n4, eq113_e1431_d_n5, eq113_e1431_d_n6, eq113_e1431_d_n7, eq113_e1431_d_n8, eq113_e1431_d_n9, eq113_e1431_d_n10, eq113_e1431_d_n11, eq113_e1431_d_n12, eq113_e1431_d_n13, eq113_e1431_d_n14, eq113_e1431_d_n15, eq113_e1431_d_n16, eq113_e1431_d_n17, eq113_e1431_d_n18, eq113_e1431_d_n19, eq113_e1431_d_n20, eq113_e1431_d_n21, eq113_e1431_d_n22, eq113_e1431_d_n23, eq113_e1431_d_n24, eq113_e1431_d_n25, eq113_e1431_d_n26, eq113_e1431_d_n27, eq113_e1431_d_n28, eq113_e1431_d_n29, eq113_e1431_q, eq113_e1431_q_d_n0, eq113_e1431_q_d_n1, eq113_e1431_q_d_n2, eq113_e1431_q_d_n3, eq113_e1431_q_d_n4, eq113_e1431_q_d_n5, eq113_e1431_q_d_n6, eq113_e1431_q_d_n7, eq113_e1431_q_d_n8, eq113_e1431_q_d_n9, eq113_e1431_q_d_n10, eq113_e1431_q_d_n11, eq113_e1431_q_d_n12, eq113_e1431_q_d_n13, eq113_e1431_q_d_n14, eq113_e1431_q_d_n15, eq113_e1431_q_d_n16, eq113_e1431_q_d_n17, eq113_e1431_q_d_n18, eq113_e1431_q_d_n19, eq113_e1431_q_d_n20, eq113_e1431_q_d_n21, eq113_e1431_q_d_n22, eq113_e1431_q_d_n23, eq113_e1431_q_d_n24, eq113_e1431_q_d_n25, eq113_e1431_q_d_n26, eq113_e1431_q_d_n27, eq113_e1431_q_d_n28, eq113_e1431_q_d_n29,) = {
    if (s.v[1348] != 0.0) {
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
            &nodes,
            &eq113_reactive_node_derivatives,
            &branches,
            &eq113_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_115_block_0(
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
        let (eq115_e1445, eq115_e1445_d_n0, eq115_e1445_d_n1, eq115_e1445_d_n2, eq115_e1445_d_n3, eq115_e1445_d_n4, eq115_e1445_d_n5, eq115_e1445_d_n6, eq115_e1445_d_n7, eq115_e1445_d_n8, eq115_e1445_d_n9, eq115_e1445_d_n10, eq115_e1445_d_n11, eq115_e1445_d_n12, eq115_e1445_d_n13, eq115_e1445_d_n14, eq115_e1445_d_n15, eq115_e1445_d_n16, eq115_e1445_d_n17, eq115_e1445_d_n18, eq115_e1445_d_n19, eq115_e1445_d_n20, eq115_e1445_d_n21, eq115_e1445_d_n22, eq115_e1445_d_n23, eq115_e1445_d_n24, eq115_e1445_d_n25, eq115_e1445_d_n26, eq115_e1445_d_n27, eq115_e1445_d_n28, eq115_e1445_d_n29, eq115_e1445_q, eq115_e1445_q_d_n0, eq115_e1445_q_d_n1, eq115_e1445_q_d_n2, eq115_e1445_q_d_n3, eq115_e1445_q_d_n4, eq115_e1445_q_d_n5, eq115_e1445_q_d_n6, eq115_e1445_q_d_n7, eq115_e1445_q_d_n8, eq115_e1445_q_d_n9, eq115_e1445_q_d_n10, eq115_e1445_q_d_n11, eq115_e1445_q_d_n12, eq115_e1445_q_d_n13, eq115_e1445_q_d_n14, eq115_e1445_q_d_n15, eq115_e1445_q_d_n16, eq115_e1445_q_d_n17, eq115_e1445_q_d_n18, eq115_e1445_q_d_n19, eq115_e1445_q_d_n20, eq115_e1445_q_d_n21, eq115_e1445_q_d_n22, eq115_e1445_q_d_n23, eq115_e1445_q_d_n24, eq115_e1445_q_d_n25, eq115_e1445_q_d_n26, eq115_e1445_q_d_n27, eq115_e1445_q_d_n28, eq115_e1445_q_d_n29,) = {
    if (s.v[1348] != 0.0) {
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
            &nodes,
            &eq115_reactive_node_derivatives,
            &branches,
            &eq115_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_116_block_0(
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq116_e1456, eq116_e1456_d_n0, eq116_e1456_d_n1, eq116_e1456_d_n2, eq116_e1456_d_n3, eq116_e1456_d_n4, eq116_e1456_d_n5, eq116_e1456_d_n6, eq116_e1456_d_n7, eq116_e1456_d_n8, eq116_e1456_d_n9, eq116_e1456_d_n10, eq116_e1456_d_n11, eq116_e1456_d_n12, eq116_e1456_d_n13, eq116_e1456_d_n14, eq116_e1456_d_n15, eq116_e1456_d_n16, eq116_e1456_d_n17, eq116_e1456_d_n18, eq116_e1456_d_n19, eq116_e1456_d_n20, eq116_e1456_d_n21, eq116_e1456_d_n22, eq116_e1456_d_n23, eq116_e1456_d_n24, eq116_e1456_d_n25, eq116_e1456_d_n26, eq116_e1456_d_n27, eq116_e1456_d_n28, eq116_e1456_d_n29, eq116_e1456_q, eq116_e1456_q_d_n0, eq116_e1456_q_d_n1, eq116_e1456_q_d_n2, eq116_e1456_q_d_n3, eq116_e1456_q_d_n4, eq116_e1456_q_d_n5, eq116_e1456_q_d_n6, eq116_e1456_q_d_n7, eq116_e1456_q_d_n8, eq116_e1456_q_d_n9, eq116_e1456_q_d_n10, eq116_e1456_q_d_n11, eq116_e1456_q_d_n12, eq116_e1456_q_d_n13, eq116_e1456_q_d_n14, eq116_e1456_q_d_n15, eq116_e1456_q_d_n16, eq116_e1456_q_d_n17, eq116_e1456_q_d_n18, eq116_e1456_q_d_n19, eq116_e1456_q_d_n20, eq116_e1456_q_d_n21, eq116_e1456_q_d_n22, eq116_e1456_q_d_n23, eq116_e1456_q_d_n24, eq116_e1456_q_d_n25, eq116_e1456_q_d_n26, eq116_e1456_q_d_n27, eq116_e1456_q_d_n28, eq116_e1456_q_d_n29,) = {
    if (!(s.v[1348] != 0.0)) {
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
            &nodes,
            &eq116_reactive_node_derivatives,
            &branches,
            &eq116_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_117_block_0(
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq117_e1467, eq117_e1467_d_n0, eq117_e1467_d_n1, eq117_e1467_d_n2, eq117_e1467_d_n3, eq117_e1467_d_n4, eq117_e1467_d_n5, eq117_e1467_d_n6, eq117_e1467_d_n7, eq117_e1467_d_n8, eq117_e1467_d_n9, eq117_e1467_d_n10, eq117_e1467_d_n11, eq117_e1467_d_n12, eq117_e1467_d_n13, eq117_e1467_d_n14, eq117_e1467_d_n15, eq117_e1467_d_n16, eq117_e1467_d_n17, eq117_e1467_d_n18, eq117_e1467_d_n19, eq117_e1467_d_n20, eq117_e1467_d_n21, eq117_e1467_d_n22, eq117_e1467_d_n23, eq117_e1467_d_n24, eq117_e1467_d_n25, eq117_e1467_d_n26, eq117_e1467_d_n27, eq117_e1467_d_n28, eq117_e1467_d_n29, eq117_e1467_q, eq117_e1467_q_d_n0, eq117_e1467_q_d_n1, eq117_e1467_q_d_n2, eq117_e1467_q_d_n3, eq117_e1467_q_d_n4, eq117_e1467_q_d_n5, eq117_e1467_q_d_n6, eq117_e1467_q_d_n7, eq117_e1467_q_d_n8, eq117_e1467_q_d_n9, eq117_e1467_q_d_n10, eq117_e1467_q_d_n11, eq117_e1467_q_d_n12, eq117_e1467_q_d_n13, eq117_e1467_q_d_n14, eq117_e1467_q_d_n15, eq117_e1467_q_d_n16, eq117_e1467_q_d_n17, eq117_e1467_q_d_n18, eq117_e1467_q_d_n19, eq117_e1467_q_d_n20, eq117_e1467_q_d_n21, eq117_e1467_q_d_n22, eq117_e1467_q_d_n23, eq117_e1467_q_d_n24, eq117_e1467_q_d_n25, eq117_e1467_q_d_n26, eq117_e1467_q_d_n27, eq117_e1467_q_d_n28, eq117_e1467_q_d_n29,) = {
    if (!(s.v[1348] != 0.0)) {
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
            &nodes,
            &eq117_reactive_node_derivatives,
            &branches,
            &eq117_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_118_block_0(
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq118_e1478, eq118_e1478_d_n0, eq118_e1478_d_n1, eq118_e1478_d_n2, eq118_e1478_d_n3, eq118_e1478_d_n4, eq118_e1478_d_n5, eq118_e1478_d_n6, eq118_e1478_d_n7, eq118_e1478_d_n8, eq118_e1478_d_n9, eq118_e1478_d_n10, eq118_e1478_d_n11, eq118_e1478_d_n12, eq118_e1478_d_n13, eq118_e1478_d_n14, eq118_e1478_d_n15, eq118_e1478_d_n16, eq118_e1478_d_n17, eq118_e1478_d_n18, eq118_e1478_d_n19, eq118_e1478_d_n20, eq118_e1478_d_n21, eq118_e1478_d_n22, eq118_e1478_d_n23, eq118_e1478_d_n24, eq118_e1478_d_n25, eq118_e1478_d_n26, eq118_e1478_d_n27, eq118_e1478_d_n28, eq118_e1478_d_n29, eq118_e1478_q, eq118_e1478_q_d_n0, eq118_e1478_q_d_n1, eq118_e1478_q_d_n2, eq118_e1478_q_d_n3, eq118_e1478_q_d_n4, eq118_e1478_q_d_n5, eq118_e1478_q_d_n6, eq118_e1478_q_d_n7, eq118_e1478_q_d_n8, eq118_e1478_q_d_n9, eq118_e1478_q_d_n10, eq118_e1478_q_d_n11, eq118_e1478_q_d_n12, eq118_e1478_q_d_n13, eq118_e1478_q_d_n14, eq118_e1478_q_d_n15, eq118_e1478_q_d_n16, eq118_e1478_q_d_n17, eq118_e1478_q_d_n18, eq118_e1478_q_d_n19, eq118_e1478_q_d_n20, eq118_e1478_q_d_n21, eq118_e1478_q_d_n22, eq118_e1478_q_d_n23, eq118_e1478_q_d_n24, eq118_e1478_q_d_n25, eq118_e1478_q_d_n26, eq118_e1478_q_d_n27, eq118_e1478_q_d_n28, eq118_e1478_q_d_n29,) = {
    if (!(s.v[1348] != 0.0)) {
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
            &nodes,
            &eq118_reactive_node_derivatives,
            &branches,
            &eq118_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_121_block_0(
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
        let nv12 = ctx.node_voltage(nodes[12]);
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
            &nodes,
            &eq121_reactive_node_derivatives,
            &branches,
            &eq121_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_124_block_0(
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
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq124_e1518, eq124_e1518_d_n0, eq124_e1518_d_n1, eq124_e1518_d_n2, eq124_e1518_d_n3, eq124_e1518_d_n4, eq124_e1518_d_n5, eq124_e1518_d_n6, eq124_e1518_d_n7, eq124_e1518_d_n8, eq124_e1518_d_n9, eq124_e1518_d_n10, eq124_e1518_d_n11, eq124_e1518_d_n12, eq124_e1518_d_n13, eq124_e1518_d_n14, eq124_e1518_d_n15, eq124_e1518_d_n16, eq124_e1518_d_n17, eq124_e1518_d_n18, eq124_e1518_d_n19, eq124_e1518_d_n20, eq124_e1518_d_n21, eq124_e1518_d_n22, eq124_e1518_d_n23, eq124_e1518_d_n24, eq124_e1518_d_n25, eq124_e1518_d_n26, eq124_e1518_d_n27, eq124_e1518_d_n28, eq124_e1518_d_n29, eq124_e1518_q, eq124_e1518_q_d_n0, eq124_e1518_q_d_n1, eq124_e1518_q_d_n2, eq124_e1518_q_d_n3, eq124_e1518_q_d_n4, eq124_e1518_q_d_n5, eq124_e1518_q_d_n6, eq124_e1518_q_d_n7, eq124_e1518_q_d_n8, eq124_e1518_q_d_n9, eq124_e1518_q_d_n10, eq124_e1518_q_d_n11, eq124_e1518_q_d_n12, eq124_e1518_q_d_n13, eq124_e1518_q_d_n14, eq124_e1518_q_d_n15, eq124_e1518_q_d_n16, eq124_e1518_q_d_n17, eq124_e1518_q_d_n18, eq124_e1518_q_d_n19, eq124_e1518_q_d_n20, eq124_e1518_q_d_n21, eq124_e1518_q_d_n22, eq124_e1518_q_d_n23, eq124_e1518_q_d_n24, eq124_e1518_q_d_n25, eq124_e1518_q_d_n26, eq124_e1518_q_d_n27, eq124_e1518_q_d_n28, eq124_e1518_q_d_n29,) = {
    if (s.v[1495] != 0.0) {
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
            &nodes,
            &eq124_reactive_node_derivatives,
            &branches,
            &eq124_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_125_block_0(
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq125_e1528, eq125_e1528_d_n0, eq125_e1528_d_n1, eq125_e1528_d_n2, eq125_e1528_d_n3, eq125_e1528_d_n4, eq125_e1528_d_n5, eq125_e1528_d_n6, eq125_e1528_d_n7, eq125_e1528_d_n8, eq125_e1528_d_n9, eq125_e1528_d_n10, eq125_e1528_d_n11, eq125_e1528_d_n12, eq125_e1528_d_n13, eq125_e1528_d_n14, eq125_e1528_d_n15, eq125_e1528_d_n16, eq125_e1528_d_n17, eq125_e1528_d_n18, eq125_e1528_d_n19, eq125_e1528_d_n20, eq125_e1528_d_n21, eq125_e1528_d_n22, eq125_e1528_d_n23, eq125_e1528_d_n24, eq125_e1528_d_n25, eq125_e1528_d_n26, eq125_e1528_d_n27, eq125_e1528_d_n28, eq125_e1528_d_n29, eq125_e1528_q, eq125_e1528_q_d_n0, eq125_e1528_q_d_n1, eq125_e1528_q_d_n2, eq125_e1528_q_d_n3, eq125_e1528_q_d_n4, eq125_e1528_q_d_n5, eq125_e1528_q_d_n6, eq125_e1528_q_d_n7, eq125_e1528_q_d_n8, eq125_e1528_q_d_n9, eq125_e1528_q_d_n10, eq125_e1528_q_d_n11, eq125_e1528_q_d_n12, eq125_e1528_q_d_n13, eq125_e1528_q_d_n14, eq125_e1528_q_d_n15, eq125_e1528_q_d_n16, eq125_e1528_q_d_n17, eq125_e1528_q_d_n18, eq125_e1528_q_d_n19, eq125_e1528_q_d_n20, eq125_e1528_q_d_n21, eq125_e1528_q_d_n22, eq125_e1528_q_d_n23, eq125_e1528_q_d_n24, eq125_e1528_q_d_n25, eq125_e1528_q_d_n26, eq125_e1528_q_d_n27, eq125_e1528_q_d_n28, eq125_e1528_q_d_n29,) = {
    if (s.v[1495] != 0.0) {
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
            &nodes,
            &eq125_reactive_node_derivatives,
            &branches,
            &eq125_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
