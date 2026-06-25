#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_126_block_0(
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
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq126_e1538, eq126_e1538_d_n0, eq126_e1538_d_n1, eq126_e1538_d_n2, eq126_e1538_d_n3, eq126_e1538_d_n4, eq126_e1538_d_n5, eq126_e1538_d_n6, eq126_e1538_d_n7, eq126_e1538_d_n8, eq126_e1538_d_n9, eq126_e1538_d_n10, eq126_e1538_d_n11, eq126_e1538_d_n12, eq126_e1538_d_n13, eq126_e1538_d_n14, eq126_e1538_d_n15, eq126_e1538_d_n16, eq126_e1538_d_n17, eq126_e1538_d_n18, eq126_e1538_d_n19, eq126_e1538_d_n20, eq126_e1538_d_n21, eq126_e1538_d_n22, eq126_e1538_d_n23, eq126_e1538_d_n24, eq126_e1538_d_n25, eq126_e1538_d_n26, eq126_e1538_d_n27, eq126_e1538_d_n28, eq126_e1538_d_n29, eq126_e1538_q, eq126_e1538_q_d_n0, eq126_e1538_q_d_n1, eq126_e1538_q_d_n2, eq126_e1538_q_d_n3, eq126_e1538_q_d_n4, eq126_e1538_q_d_n5, eq126_e1538_q_d_n6, eq126_e1538_q_d_n7, eq126_e1538_q_d_n8, eq126_e1538_q_d_n9, eq126_e1538_q_d_n10, eq126_e1538_q_d_n11, eq126_e1538_q_d_n12, eq126_e1538_q_d_n13, eq126_e1538_q_d_n14, eq126_e1538_q_d_n15, eq126_e1538_q_d_n16, eq126_e1538_q_d_n17, eq126_e1538_q_d_n18, eq126_e1538_q_d_n19, eq126_e1538_q_d_n20, eq126_e1538_q_d_n21, eq126_e1538_q_d_n22, eq126_e1538_q_d_n23, eq126_e1538_q_d_n24, eq126_e1538_q_d_n25, eq126_e1538_q_d_n26, eq126_e1538_q_d_n27, eq126_e1538_q_d_n28, eq126_e1538_q_d_n29,) = {
    if (s.v[1495] != 0.0) {
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
            &nodes,
            &eq126_reactive_node_derivatives,
            &branches,
            &eq126_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_128_block_0(
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
        let (eq128_e1552, eq128_e1552_d_n0, eq128_e1552_d_n1, eq128_e1552_d_n2, eq128_e1552_d_n3, eq128_e1552_d_n4, eq128_e1552_d_n5, eq128_e1552_d_n6, eq128_e1552_d_n7, eq128_e1552_d_n8, eq128_e1552_d_n9, eq128_e1552_d_n10, eq128_e1552_d_n11, eq128_e1552_d_n12, eq128_e1552_d_n13, eq128_e1552_d_n14, eq128_e1552_d_n15, eq128_e1552_d_n16, eq128_e1552_d_n17, eq128_e1552_d_n18, eq128_e1552_d_n19, eq128_e1552_d_n20, eq128_e1552_d_n21, eq128_e1552_d_n22, eq128_e1552_d_n23, eq128_e1552_d_n24, eq128_e1552_d_n25, eq128_e1552_d_n26, eq128_e1552_d_n27, eq128_e1552_d_n28, eq128_e1552_d_n29, eq128_e1552_q, eq128_e1552_q_d_n0, eq128_e1552_q_d_n1, eq128_e1552_q_d_n2, eq128_e1552_q_d_n3, eq128_e1552_q_d_n4, eq128_e1552_q_d_n5, eq128_e1552_q_d_n6, eq128_e1552_q_d_n7, eq128_e1552_q_d_n8, eq128_e1552_q_d_n9, eq128_e1552_q_d_n10, eq128_e1552_q_d_n11, eq128_e1552_q_d_n12, eq128_e1552_q_d_n13, eq128_e1552_q_d_n14, eq128_e1552_q_d_n15, eq128_e1552_q_d_n16, eq128_e1552_q_d_n17, eq128_e1552_q_d_n18, eq128_e1552_q_d_n19, eq128_e1552_q_d_n20, eq128_e1552_q_d_n21, eq128_e1552_q_d_n22, eq128_e1552_q_d_n23, eq128_e1552_q_d_n24, eq128_e1552_q_d_n25, eq128_e1552_q_d_n26, eq128_e1552_q_d_n27, eq128_e1552_q_d_n28, eq128_e1552_q_d_n29,) = {
    if (s.v[1495] != 0.0) {
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
            &nodes,
            &eq128_reactive_node_derivatives,
            &branches,
            &eq128_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_129_block_0(
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
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq129_e1563, eq129_e1563_d_n0, eq129_e1563_d_n1, eq129_e1563_d_n2, eq129_e1563_d_n3, eq129_e1563_d_n4, eq129_e1563_d_n5, eq129_e1563_d_n6, eq129_e1563_d_n7, eq129_e1563_d_n8, eq129_e1563_d_n9, eq129_e1563_d_n10, eq129_e1563_d_n11, eq129_e1563_d_n12, eq129_e1563_d_n13, eq129_e1563_d_n14, eq129_e1563_d_n15, eq129_e1563_d_n16, eq129_e1563_d_n17, eq129_e1563_d_n18, eq129_e1563_d_n19, eq129_e1563_d_n20, eq129_e1563_d_n21, eq129_e1563_d_n22, eq129_e1563_d_n23, eq129_e1563_d_n24, eq129_e1563_d_n25, eq129_e1563_d_n26, eq129_e1563_d_n27, eq129_e1563_d_n28, eq129_e1563_d_n29, eq129_e1563_q, eq129_e1563_q_d_n0, eq129_e1563_q_d_n1, eq129_e1563_q_d_n2, eq129_e1563_q_d_n3, eq129_e1563_q_d_n4, eq129_e1563_q_d_n5, eq129_e1563_q_d_n6, eq129_e1563_q_d_n7, eq129_e1563_q_d_n8, eq129_e1563_q_d_n9, eq129_e1563_q_d_n10, eq129_e1563_q_d_n11, eq129_e1563_q_d_n12, eq129_e1563_q_d_n13, eq129_e1563_q_d_n14, eq129_e1563_q_d_n15, eq129_e1563_q_d_n16, eq129_e1563_q_d_n17, eq129_e1563_q_d_n18, eq129_e1563_q_d_n19, eq129_e1563_q_d_n20, eq129_e1563_q_d_n21, eq129_e1563_q_d_n22, eq129_e1563_q_d_n23, eq129_e1563_q_d_n24, eq129_e1563_q_d_n25, eq129_e1563_q_d_n26, eq129_e1563_q_d_n27, eq129_e1563_q_d_n28, eq129_e1563_q_d_n29,) = {
    if (!(s.v[1495] != 0.0)) {
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
            &nodes,
            &eq129_reactive_node_derivatives,
            &branches,
            &eq129_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_130_block_0(
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
        let (eq130_e1574, eq130_e1574_d_n0, eq130_e1574_d_n1, eq130_e1574_d_n2, eq130_e1574_d_n3, eq130_e1574_d_n4, eq130_e1574_d_n5, eq130_e1574_d_n6, eq130_e1574_d_n7, eq130_e1574_d_n8, eq130_e1574_d_n9, eq130_e1574_d_n10, eq130_e1574_d_n11, eq130_e1574_d_n12, eq130_e1574_d_n13, eq130_e1574_d_n14, eq130_e1574_d_n15, eq130_e1574_d_n16, eq130_e1574_d_n17, eq130_e1574_d_n18, eq130_e1574_d_n19, eq130_e1574_d_n20, eq130_e1574_d_n21, eq130_e1574_d_n22, eq130_e1574_d_n23, eq130_e1574_d_n24, eq130_e1574_d_n25, eq130_e1574_d_n26, eq130_e1574_d_n27, eq130_e1574_d_n28, eq130_e1574_d_n29, eq130_e1574_q, eq130_e1574_q_d_n0, eq130_e1574_q_d_n1, eq130_e1574_q_d_n2, eq130_e1574_q_d_n3, eq130_e1574_q_d_n4, eq130_e1574_q_d_n5, eq130_e1574_q_d_n6, eq130_e1574_q_d_n7, eq130_e1574_q_d_n8, eq130_e1574_q_d_n9, eq130_e1574_q_d_n10, eq130_e1574_q_d_n11, eq130_e1574_q_d_n12, eq130_e1574_q_d_n13, eq130_e1574_q_d_n14, eq130_e1574_q_d_n15, eq130_e1574_q_d_n16, eq130_e1574_q_d_n17, eq130_e1574_q_d_n18, eq130_e1574_q_d_n19, eq130_e1574_q_d_n20, eq130_e1574_q_d_n21, eq130_e1574_q_d_n22, eq130_e1574_q_d_n23, eq130_e1574_q_d_n24, eq130_e1574_q_d_n25, eq130_e1574_q_d_n26, eq130_e1574_q_d_n27, eq130_e1574_q_d_n28, eq130_e1574_q_d_n29,) = {
    if (!(s.v[1495] != 0.0)) {
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
            &nodes,
            &eq130_reactive_node_derivatives,
            &branches,
            &eq130_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_131_block_0(
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
        let (eq131_e1585, eq131_e1585_d_n0, eq131_e1585_d_n1, eq131_e1585_d_n2, eq131_e1585_d_n3, eq131_e1585_d_n4, eq131_e1585_d_n5, eq131_e1585_d_n6, eq131_e1585_d_n7, eq131_e1585_d_n8, eq131_e1585_d_n9, eq131_e1585_d_n10, eq131_e1585_d_n11, eq131_e1585_d_n12, eq131_e1585_d_n13, eq131_e1585_d_n14, eq131_e1585_d_n15, eq131_e1585_d_n16, eq131_e1585_d_n17, eq131_e1585_d_n18, eq131_e1585_d_n19, eq131_e1585_d_n20, eq131_e1585_d_n21, eq131_e1585_d_n22, eq131_e1585_d_n23, eq131_e1585_d_n24, eq131_e1585_d_n25, eq131_e1585_d_n26, eq131_e1585_d_n27, eq131_e1585_d_n28, eq131_e1585_d_n29, eq131_e1585_q, eq131_e1585_q_d_n0, eq131_e1585_q_d_n1, eq131_e1585_q_d_n2, eq131_e1585_q_d_n3, eq131_e1585_q_d_n4, eq131_e1585_q_d_n5, eq131_e1585_q_d_n6, eq131_e1585_q_d_n7, eq131_e1585_q_d_n8, eq131_e1585_q_d_n9, eq131_e1585_q_d_n10, eq131_e1585_q_d_n11, eq131_e1585_q_d_n12, eq131_e1585_q_d_n13, eq131_e1585_q_d_n14, eq131_e1585_q_d_n15, eq131_e1585_q_d_n16, eq131_e1585_q_d_n17, eq131_e1585_q_d_n18, eq131_e1585_q_d_n19, eq131_e1585_q_d_n20, eq131_e1585_q_d_n21, eq131_e1585_q_d_n22, eq131_e1585_q_d_n23, eq131_e1585_q_d_n24, eq131_e1585_q_d_n25, eq131_e1585_q_d_n26, eq131_e1585_q_d_n27, eq131_e1585_q_d_n28, eq131_e1585_q_d_n29,) = {
    if (!(s.v[1495] != 0.0)) {
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
            &nodes,
            &eq131_reactive_node_derivatives,
            &branches,
            &eq131_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_134_block_0(
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
        let nv13 = ctx.node_voltage(nodes[13]);
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
            &nodes,
            &eq134_reactive_node_derivatives,
            &branches,
            &eq134_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_142_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv28 = ctx.node_voltage(nodes[28]);
        let nv29 = ctx.node_voltage(nodes[29]);
        let (eq142_e1656, eq142_e1656_d_n0, eq142_e1656_d_n1, eq142_e1656_d_n2, eq142_e1656_d_n3, eq142_e1656_d_n4, eq142_e1656_d_n5, eq142_e1656_d_n6, eq142_e1656_d_n7, eq142_e1656_d_n8, eq142_e1656_d_n9, eq142_e1656_d_n10, eq142_e1656_d_n11, eq142_e1656_d_n12, eq142_e1656_d_n13, eq142_e1656_d_n14, eq142_e1656_d_n15, eq142_e1656_d_n16, eq142_e1656_d_n17, eq142_e1656_d_n18, eq142_e1656_d_n19, eq142_e1656_d_n20, eq142_e1656_d_n21, eq142_e1656_d_n22, eq142_e1656_d_n23, eq142_e1656_d_n24, eq142_e1656_d_n25, eq142_e1656_d_n26, eq142_e1656_d_n27, eq142_e1656_d_n28, eq142_e1656_d_n29, eq142_e1656_q, eq142_e1656_q_d_n28,) = {
    if (!(s.v[1933] != 0.0)) {
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
        stamper.stamp_current_reactive(
            Some(nodes[28]),
            None,
            &[
                GeneratedDerivative::node(nodes[28], self.multiplicity * (eq142_e1656_q_d_n28)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_143_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv28 = ctx.node_voltage(nodes[28]);
        let nv29 = ctx.node_voltage(nodes[29]);
        let (eq143_e1670, eq143_e1670_d_n28, eq143_e1670_d_n29, eq143_e1670_q, eq143_e1670_q_d_n29,) = {
    if (!(s.v[1933] != 0.0)) {
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
        stamper.stamp_current_reactive(
            Some(nodes[29]),
            None,
            &[
                GeneratedDerivative::node(nodes[29], self.multiplicity * (eq143_e1670_q_d_n29)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_145_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
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
            &nodes,
            &eq145_reactive_node_derivatives,
            &branches,
            &eq145_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_146_block_0(
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
        let nv8 = ctx.node_voltage(nodes[8]);
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
            &nodes,
            &eq146_reactive_node_derivatives,
            &branches,
            &eq146_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_157_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq157_e1796, eq157_e1796_d_n0, eq157_e1796_d_n1, eq157_e1796_d_n2, eq157_e1796_d_n3, eq157_e1796_d_n4, eq157_e1796_d_n5, eq157_e1796_d_n6, eq157_e1796_d_n7, eq157_e1796_d_n8, eq157_e1796_d_n9, eq157_e1796_d_n10, eq157_e1796_d_n11, eq157_e1796_d_n12, eq157_e1796_d_n13, eq157_e1796_d_n14, eq157_e1796_d_n15, eq157_e1796_d_n16, eq157_e1796_d_n17, eq157_e1796_d_n18, eq157_e1796_d_n19, eq157_e1796_d_n20, eq157_e1796_d_n21, eq157_e1796_d_n22, eq157_e1796_d_n23, eq157_e1796_d_n24, eq157_e1796_d_n25, eq157_e1796_d_n26, eq157_e1796_d_n27, eq157_e1796_d_n28, eq157_e1796_d_n29, eq157_e1796_q, eq157_e1796_q_d_n0, eq157_e1796_q_d_n1, eq157_e1796_q_d_n2, eq157_e1796_q_d_n3, eq157_e1796_q_d_n4, eq157_e1796_q_d_n5, eq157_e1796_q_d_n6, eq157_e1796_q_d_n7, eq157_e1796_q_d_n8, eq157_e1796_q_d_n9, eq157_e1796_q_d_n10, eq157_e1796_q_d_n11, eq157_e1796_q_d_n12, eq157_e1796_q_d_n13, eq157_e1796_q_d_n14, eq157_e1796_q_d_n15, eq157_e1796_q_d_n16, eq157_e1796_q_d_n17, eq157_e1796_q_d_n18, eq157_e1796_q_d_n19, eq157_e1796_q_d_n20, eq157_e1796_q_d_n21, eq157_e1796_q_d_n22, eq157_e1796_q_d_n23, eq157_e1796_q_d_n24, eq157_e1796_q_d_n25, eq157_e1796_q_d_n26, eq157_e1796_q_d_n27, eq157_e1796_q_d_n28, eq157_e1796_q_d_n29,) = {
    if (s.v[2418] != 0.0) {
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
            &nodes,
            &eq157_reactive_node_derivatives,
            &branches,
            &eq157_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_172_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq172_e1881_q: f64 = s.v[214];
        let eq172_reactive_node_derivatives: [f64; 30] = [s.dn[214][0], s.dn[214][1], s.dn[214][2], s.dn[214][3], s.dn[214][4], s.dn[214][5], s.dn[214][6], s.dn[214][7], s.dn[214][8], s.dn[214][9], s.dn[214][10], s.dn[214][11], s.dn[214][12], s.dn[214][13], s.dn[214][14], s.dn[214][15], s.dn[214][16], s.dn[214][17], s.dn[214][18], s.dn[214][19], s.dn[214][20], s.dn[214][21], s.dn[214][22], s.dn[214][23], s.dn[214][24], s.dn[214][25], s.dn[214][26], s.dn[214][27], s.dn[214][28], s.dn[214][29]];
        let eq172_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[2]),
            &nodes,
            &eq172_reactive_node_derivatives,
            &branches,
            &eq172_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_173_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq173_e1883_q: f64 = s.v[215];
        let eq173_reactive_node_derivatives: [f64; 30] = [s.dn[215][0], s.dn[215][1], s.dn[215][2], s.dn[215][3], s.dn[215][4], s.dn[215][5], s.dn[215][6], s.dn[215][7], s.dn[215][8], s.dn[215][9], s.dn[215][10], s.dn[215][11], s.dn[215][12], s.dn[215][13], s.dn[215][14], s.dn[215][15], s.dn[215][16], s.dn[215][17], s.dn[215][18], s.dn[215][19], s.dn[215][20], s.dn[215][21], s.dn[215][22], s.dn[215][23], s.dn[215][24], s.dn[215][25], s.dn[215][26], s.dn[215][27], s.dn[215][28], s.dn[215][29]];
        let eq173_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[0]),
            &nodes,
            &eq173_reactive_node_derivatives,
            &branches,
            &eq173_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_174_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq174_e1885_q: f64 = s.v[216];
        let eq174_reactive_node_derivatives: [f64; 30] = [s.dn[216][0], s.dn[216][1], s.dn[216][2], s.dn[216][3], s.dn[216][4], s.dn[216][5], s.dn[216][6], s.dn[216][7], s.dn[216][8], s.dn[216][9], s.dn[216][10], s.dn[216][11], s.dn[216][12], s.dn[216][13], s.dn[216][14], s.dn[216][15], s.dn[216][16], s.dn[216][17], s.dn[216][18], s.dn[216][19], s.dn[216][20], s.dn[216][21], s.dn[216][22], s.dn[216][23], s.dn[216][24], s.dn[216][25], s.dn[216][26], s.dn[216][27], s.dn[216][28], s.dn[216][29]];
        let eq174_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[0]),
            &nodes,
            &eq174_reactive_node_derivatives,
            &branches,
            &eq174_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_175_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq175_e1887_q: f64 = s.v[218];
        let eq175_reactive_node_derivatives: [f64; 30] = [s.dn[218][0], s.dn[218][1], s.dn[218][2], s.dn[218][3], s.dn[218][4], s.dn[218][5], s.dn[218][6], s.dn[218][7], s.dn[218][8], s.dn[218][9], s.dn[218][10], s.dn[218][11], s.dn[218][12], s.dn[218][13], s.dn[218][14], s.dn[218][15], s.dn[218][16], s.dn[218][17], s.dn[218][18], s.dn[218][19], s.dn[218][20], s.dn[218][21], s.dn[218][22], s.dn[218][23], s.dn[218][24], s.dn[218][25], s.dn[218][26], s.dn[218][27], s.dn[218][28], s.dn[218][29]];
        let eq175_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[2]),
            &nodes,
            &eq175_reactive_node_derivatives,
            &branches,
            &eq175_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_176_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq176_e1889_q: f64 = s.v[217];
        let eq176_reactive_node_derivatives: [f64; 30] = [s.dn[217][0], s.dn[217][1], s.dn[217][2], s.dn[217][3], s.dn[217][4], s.dn[217][5], s.dn[217][6], s.dn[217][7], s.dn[217][8], s.dn[217][9], s.dn[217][10], s.dn[217][11], s.dn[217][12], s.dn[217][13], s.dn[217][14], s.dn[217][15], s.dn[217][16], s.dn[217][17], s.dn[217][18], s.dn[217][19], s.dn[217][20], s.dn[217][21], s.dn[217][22], s.dn[217][23], s.dn[217][24], s.dn[217][25], s.dn[217][26], s.dn[217][27], s.dn[217][28], s.dn[217][29]];
        let eq176_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[0]),
            &nodes,
            &eq176_reactive_node_derivatives,
            &branches,
            &eq176_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
