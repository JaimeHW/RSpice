#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_148_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq148_e1709, eq148_e1709_d_n0, eq148_e1709_d_n1, eq148_e1709_d_n2, eq148_e1709_d_n3, eq148_e1709_d_n4, eq148_e1709_d_n5, eq148_e1709_d_n6, eq148_e1709_d_n7, eq148_e1709_d_n8, eq148_e1709_d_n9, eq148_e1709_d_n10, eq148_e1709_d_n11, eq148_e1709_d_n12, eq148_e1709_d_n13, eq148_e1709_d_n14, eq148_e1709_d_n15, eq148_e1709_d_n16, eq148_e1709_d_n17, eq148_e1709_d_n18, eq148_e1709_d_n19, eq148_e1709_d_n20, eq148_e1709_d_n21, eq148_e1709_d_n22, eq148_e1709_d_n23, eq148_e1709_d_n24, eq148_e1709_d_n25, eq148_e1709_d_n26, eq148_e1709_d_n27, eq148_e1709_d_n28, eq148_e1709_d_n29,) = {
    if (s.v[1934] != 0.0) {
        let eq148_e1706: f64 = (s.v[0] * (nv8 - nv17));
        let eq148_e1706_d_n8: f64 = s.v[0];
        let eq148_e1706_d_n17: f64 = (-s.v[0]);
        let eq148_e1707: f64 = (s.v[123] + eq148_e1706);
        let eq148_e1707_d_n8: f64 = (s.dn[123][8] + eq148_e1706_d_n8);
        let eq148_e1707_d_n17: f64 = (s.dn[123][17] + eq148_e1706_d_n17);
        (eq148_e1707, s.dn[123][0], s.dn[123][1], s.dn[123][2], s.dn[123][3], s.dn[123][4], s.dn[123][5], s.dn[123][6], s.dn[123][7], eq148_e1707_d_n8, s.dn[123][9], s.dn[123][10], s.dn[123][11], s.dn[123][12], s.dn[123][13], s.dn[123][14], s.dn[123][15], s.dn[123][16], eq148_e1707_d_n17, s.dn[123][18], s.dn[123][19], s.dn[123][20], s.dn[123][21], s.dn[123][22], s.dn[123][23], s.dn[123][24], s.dn[123][25], s.dn[123][26], s.dn[123][27], s.dn[123][28], s.dn[123][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq148_value: f64 = eq148_e1709;
        let eq148_node_derivatives: [f64; 30] = [eq148_e1709_d_n0, eq148_e1709_d_n1, eq148_e1709_d_n2, eq148_e1709_d_n3, eq148_e1709_d_n4, eq148_e1709_d_n5, eq148_e1709_d_n6, eq148_e1709_d_n7, eq148_e1709_d_n8, eq148_e1709_d_n9, eq148_e1709_d_n10, eq148_e1709_d_n11, eq148_e1709_d_n12, eq148_e1709_d_n13, eq148_e1709_d_n14, eq148_e1709_d_n15, eq148_e1709_d_n16, eq148_e1709_d_n17, eq148_e1709_d_n18, eq148_e1709_d_n19, eq148_e1709_d_n20, eq148_e1709_d_n21, eq148_e1709_d_n22, eq148_e1709_d_n23, eq148_e1709_d_n24, eq148_e1709_d_n25, eq148_e1709_d_n26, eq148_e1709_d_n27, eq148_e1709_d_n28, eq148_e1709_d_n29];
        let eq148_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[17]),
            self.multiplicity * (eq148_value),
            &nodes,
            &eq148_node_derivatives,
            &branches,
            &eq148_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_149_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq149_e1719, eq149_e1719_d_n0, eq149_e1719_d_n1, eq149_e1719_d_n2, eq149_e1719_d_n3, eq149_e1719_d_n4, eq149_e1719_d_n5, eq149_e1719_d_n6, eq149_e1719_d_n7, eq149_e1719_d_n8, eq149_e1719_d_n9, eq149_e1719_d_n10, eq149_e1719_d_n11, eq149_e1719_d_n12, eq149_e1719_d_n13, eq149_e1719_d_n14, eq149_e1719_d_n15, eq149_e1719_d_n16, eq149_e1719_d_n17, eq149_e1719_d_n18, eq149_e1719_d_n19, eq149_e1719_d_n20, eq149_e1719_d_n21, eq149_e1719_d_n22, eq149_e1719_d_n23, eq149_e1719_d_n24, eq149_e1719_d_n25, eq149_e1719_d_n26, eq149_e1719_d_n27, eq149_e1719_d_n28, eq149_e1719_d_n29,) = {
    if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
        let eq149_e1716: f64 = (s.v[0] * (nv8 - nv13));
        let eq149_e1716_d_n8: f64 = s.v[0];
        let eq149_e1716_d_n13: f64 = (-s.v[0]);
        let eq149_e1717: f64 = (s.v[134] + eq149_e1716);
        let eq149_e1717_d_n8: f64 = (s.dn[134][8] + eq149_e1716_d_n8);
        let eq149_e1717_d_n13: f64 = (s.dn[134][13] + eq149_e1716_d_n13);
        (eq149_e1717, s.dn[134][0], s.dn[134][1], s.dn[134][2], s.dn[134][3], s.dn[134][4], s.dn[134][5], s.dn[134][6], s.dn[134][7], eq149_e1717_d_n8, s.dn[134][9], s.dn[134][10], s.dn[134][11], s.dn[134][12], eq149_e1717_d_n13, s.dn[134][14], s.dn[134][15], s.dn[134][16], s.dn[134][17], s.dn[134][18], s.dn[134][19], s.dn[134][20], s.dn[134][21], s.dn[134][22], s.dn[134][23], s.dn[134][24], s.dn[134][25], s.dn[134][26], s.dn[134][27], s.dn[134][28], s.dn[134][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq149_value: f64 = eq149_e1719;
        let eq149_node_derivatives: [f64; 30] = [eq149_e1719_d_n0, eq149_e1719_d_n1, eq149_e1719_d_n2, eq149_e1719_d_n3, eq149_e1719_d_n4, eq149_e1719_d_n5, eq149_e1719_d_n6, eq149_e1719_d_n7, eq149_e1719_d_n8, eq149_e1719_d_n9, eq149_e1719_d_n10, eq149_e1719_d_n11, eq149_e1719_d_n12, eq149_e1719_d_n13, eq149_e1719_d_n14, eq149_e1719_d_n15, eq149_e1719_d_n16, eq149_e1719_d_n17, eq149_e1719_d_n18, eq149_e1719_d_n19, eq149_e1719_d_n20, eq149_e1719_d_n21, eq149_e1719_d_n22, eq149_e1719_d_n23, eq149_e1719_d_n24, eq149_e1719_d_n25, eq149_e1719_d_n26, eq149_e1719_d_n27, eq149_e1719_d_n28, eq149_e1719_d_n29];
        let eq149_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[13]),
            self.multiplicity * (eq149_value),
            &nodes,
            &eq149_node_derivatives,
            &branches,
            &eq149_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_150_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq150_e1729, eq150_e1729_d_n0, eq150_e1729_d_n1, eq150_e1729_d_n2, eq150_e1729_d_n3, eq150_e1729_d_n4, eq150_e1729_d_n5, eq150_e1729_d_n6, eq150_e1729_d_n7, eq150_e1729_d_n8, eq150_e1729_d_n9, eq150_e1729_d_n10, eq150_e1729_d_n11, eq150_e1729_d_n12, eq150_e1729_d_n13, eq150_e1729_d_n14, eq150_e1729_d_n15, eq150_e1729_d_n16, eq150_e1729_d_n17, eq150_e1729_d_n18, eq150_e1729_d_n19, eq150_e1729_d_n20, eq150_e1729_d_n21, eq150_e1729_d_n22, eq150_e1729_d_n23, eq150_e1729_d_n24, eq150_e1729_d_n25, eq150_e1729_d_n26, eq150_e1729_d_n27, eq150_e1729_d_n28, eq150_e1729_d_n29,) = {
    if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
        let eq150_e1726: f64 = (s.v[0] * (nv8 - nv17));
        let eq150_e1726_d_n8: f64 = s.v[0];
        let eq150_e1726_d_n17: f64 = (-s.v[0]);
        let eq150_e1727: f64 = (s.v[135] + eq150_e1726);
        let eq150_e1727_d_n8: f64 = (s.dn[135][8] + eq150_e1726_d_n8);
        let eq150_e1727_d_n17: f64 = (s.dn[135][17] + eq150_e1726_d_n17);
        (eq150_e1727, s.dn[135][0], s.dn[135][1], s.dn[135][2], s.dn[135][3], s.dn[135][4], s.dn[135][5], s.dn[135][6], s.dn[135][7], eq150_e1727_d_n8, s.dn[135][9], s.dn[135][10], s.dn[135][11], s.dn[135][12], s.dn[135][13], s.dn[135][14], s.dn[135][15], s.dn[135][16], eq150_e1727_d_n17, s.dn[135][18], s.dn[135][19], s.dn[135][20], s.dn[135][21], s.dn[135][22], s.dn[135][23], s.dn[135][24], s.dn[135][25], s.dn[135][26], s.dn[135][27], s.dn[135][28], s.dn[135][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq150_value: f64 = eq150_e1729;
        let eq150_node_derivatives: [f64; 30] = [eq150_e1729_d_n0, eq150_e1729_d_n1, eq150_e1729_d_n2, eq150_e1729_d_n3, eq150_e1729_d_n4, eq150_e1729_d_n5, eq150_e1729_d_n6, eq150_e1729_d_n7, eq150_e1729_d_n8, eq150_e1729_d_n9, eq150_e1729_d_n10, eq150_e1729_d_n11, eq150_e1729_d_n12, eq150_e1729_d_n13, eq150_e1729_d_n14, eq150_e1729_d_n15, eq150_e1729_d_n16, eq150_e1729_d_n17, eq150_e1729_d_n18, eq150_e1729_d_n19, eq150_e1729_d_n20, eq150_e1729_d_n21, eq150_e1729_d_n22, eq150_e1729_d_n23, eq150_e1729_d_n24, eq150_e1729_d_n25, eq150_e1729_d_n26, eq150_e1729_d_n27, eq150_e1729_d_n28, eq150_e1729_d_n29];
        let eq150_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[17]),
            self.multiplicity * (eq150_value),
            &nodes,
            &eq150_node_derivatives,
            &branches,
            &eq150_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_151_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq151_e1739, eq151_e1739_d_n0, eq151_e1739_d_n1, eq151_e1739_d_n2, eq151_e1739_d_n3, eq151_e1739_d_n4, eq151_e1739_d_n5, eq151_e1739_d_n6, eq151_e1739_d_n7, eq151_e1739_d_n8, eq151_e1739_d_n9, eq151_e1739_d_n10, eq151_e1739_d_n11, eq151_e1739_d_n12, eq151_e1739_d_n13, eq151_e1739_d_n14, eq151_e1739_d_n15, eq151_e1739_d_n16, eq151_e1739_d_n17, eq151_e1739_d_n18, eq151_e1739_d_n19, eq151_e1739_d_n20, eq151_e1739_d_n21, eq151_e1739_d_n22, eq151_e1739_d_n23, eq151_e1739_d_n24, eq151_e1739_d_n25, eq151_e1739_d_n26, eq151_e1739_d_n27, eq151_e1739_d_n28, eq151_e1739_d_n29,) = {
    if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
        let eq151_e1736: f64 = (s.v[0] * (nv8 - nv9));
        let eq151_e1736_d_n8: f64 = s.v[0];
        let eq151_e1736_d_n9: f64 = (-s.v[0]);
        let eq151_e1737: f64 = (s.v[128] + eq151_e1736);
        let eq151_e1737_d_n8: f64 = (s.dn[128][8] + eq151_e1736_d_n8);
        let eq151_e1737_d_n9: f64 = (s.dn[128][9] + eq151_e1736_d_n9);
        (eq151_e1737, s.dn[128][0], s.dn[128][1], s.dn[128][2], s.dn[128][3], s.dn[128][4], s.dn[128][5], s.dn[128][6], s.dn[128][7], eq151_e1737_d_n8, eq151_e1737_d_n9, s.dn[128][10], s.dn[128][11], s.dn[128][12], s.dn[128][13], s.dn[128][14], s.dn[128][15], s.dn[128][16], s.dn[128][17], s.dn[128][18], s.dn[128][19], s.dn[128][20], s.dn[128][21], s.dn[128][22], s.dn[128][23], s.dn[128][24], s.dn[128][25], s.dn[128][26], s.dn[128][27], s.dn[128][28], s.dn[128][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq151_value: f64 = eq151_e1739;
        let eq151_node_derivatives: [f64; 30] = [eq151_e1739_d_n0, eq151_e1739_d_n1, eq151_e1739_d_n2, eq151_e1739_d_n3, eq151_e1739_d_n4, eq151_e1739_d_n5, eq151_e1739_d_n6, eq151_e1739_d_n7, eq151_e1739_d_n8, eq151_e1739_d_n9, eq151_e1739_d_n10, eq151_e1739_d_n11, eq151_e1739_d_n12, eq151_e1739_d_n13, eq151_e1739_d_n14, eq151_e1739_d_n15, eq151_e1739_d_n16, eq151_e1739_d_n17, eq151_e1739_d_n18, eq151_e1739_d_n19, eq151_e1739_d_n20, eq151_e1739_d_n21, eq151_e1739_d_n22, eq151_e1739_d_n23, eq151_e1739_d_n24, eq151_e1739_d_n25, eq151_e1739_d_n26, eq151_e1739_d_n27, eq151_e1739_d_n28, eq151_e1739_d_n29];
        let eq151_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[9]),
            self.multiplicity * (eq151_value),
            &nodes,
            &eq151_node_derivatives,
            &branches,
            &eq151_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_152_block_0(
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
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq152_e1749, eq152_e1749_d_n0, eq152_e1749_d_n1, eq152_e1749_d_n2, eq152_e1749_d_n3, eq152_e1749_d_n4, eq152_e1749_d_n5, eq152_e1749_d_n6, eq152_e1749_d_n7, eq152_e1749_d_n8, eq152_e1749_d_n9, eq152_e1749_d_n10, eq152_e1749_d_n11, eq152_e1749_d_n12, eq152_e1749_d_n13, eq152_e1749_d_n14, eq152_e1749_d_n15, eq152_e1749_d_n16, eq152_e1749_d_n17, eq152_e1749_d_n18, eq152_e1749_d_n19, eq152_e1749_d_n20, eq152_e1749_d_n21, eq152_e1749_d_n22, eq152_e1749_d_n23, eq152_e1749_d_n24, eq152_e1749_d_n25, eq152_e1749_d_n26, eq152_e1749_d_n27, eq152_e1749_d_n28, eq152_e1749_d_n29,) = {
    if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
        let eq152_e1746: f64 = (s.v[0] * (nv8 - nv5));
        let eq152_e1746_d_n5: f64 = (-s.v[0]);
        let eq152_e1746_d_n8: f64 = s.v[0];
        let eq152_e1747: f64 = (s.v[129] + eq152_e1746);
        let eq152_e1747_d_n5: f64 = (s.dn[129][5] + eq152_e1746_d_n5);
        let eq152_e1747_d_n8: f64 = (s.dn[129][8] + eq152_e1746_d_n8);
        (eq152_e1747, s.dn[129][0], s.dn[129][1], s.dn[129][2], s.dn[129][3], s.dn[129][4], eq152_e1747_d_n5, s.dn[129][6], s.dn[129][7], eq152_e1747_d_n8, s.dn[129][9], s.dn[129][10], s.dn[129][11], s.dn[129][12], s.dn[129][13], s.dn[129][14], s.dn[129][15], s.dn[129][16], s.dn[129][17], s.dn[129][18], s.dn[129][19], s.dn[129][20], s.dn[129][21], s.dn[129][22], s.dn[129][23], s.dn[129][24], s.dn[129][25], s.dn[129][26], s.dn[129][27], s.dn[129][28], s.dn[129][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq152_value: f64 = eq152_e1749;
        let eq152_node_derivatives: [f64; 30] = [eq152_e1749_d_n0, eq152_e1749_d_n1, eq152_e1749_d_n2, eq152_e1749_d_n3, eq152_e1749_d_n4, eq152_e1749_d_n5, eq152_e1749_d_n6, eq152_e1749_d_n7, eq152_e1749_d_n8, eq152_e1749_d_n9, eq152_e1749_d_n10, eq152_e1749_d_n11, eq152_e1749_d_n12, eq152_e1749_d_n13, eq152_e1749_d_n14, eq152_e1749_d_n15, eq152_e1749_d_n16, eq152_e1749_d_n17, eq152_e1749_d_n18, eq152_e1749_d_n19, eq152_e1749_d_n20, eq152_e1749_d_n21, eq152_e1749_d_n22, eq152_e1749_d_n23, eq152_e1749_d_n24, eq152_e1749_d_n25, eq152_e1749_d_n26, eq152_e1749_d_n27, eq152_e1749_d_n28, eq152_e1749_d_n29];
        let eq152_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            self.multiplicity * (eq152_value),
            &nodes,
            &eq152_node_derivatives,
            &branches,
            &eq152_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_153_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq153_e1761, eq153_e1761_d_n0, eq153_e1761_d_n1, eq153_e1761_d_n2, eq153_e1761_d_n3, eq153_e1761_d_n4, eq153_e1761_d_n5, eq153_e1761_d_n6, eq153_e1761_d_n7, eq153_e1761_d_n8, eq153_e1761_d_n9, eq153_e1761_d_n10, eq153_e1761_d_n11, eq153_e1761_d_n12, eq153_e1761_d_n13, eq153_e1761_d_n14, eq153_e1761_d_n15, eq153_e1761_d_n16, eq153_e1761_d_n17, eq153_e1761_d_n18, eq153_e1761_d_n19, eq153_e1761_d_n20, eq153_e1761_d_n21, eq153_e1761_d_n22, eq153_e1761_d_n23, eq153_e1761_d_n24, eq153_e1761_d_n25, eq153_e1761_d_n26, eq153_e1761_d_n27, eq153_e1761_d_n28, eq153_e1761_d_n29,) = {
    if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
        let eq153_e1758: f64 = (s.v[0] * (nv8 - nv9));
        let eq153_e1758_d_n8: f64 = s.v[0];
        let eq153_e1758_d_n9: f64 = (-s.v[0]);
        let eq153_e1759: f64 = (s.v[140] + eq153_e1758);
        let eq153_e1759_d_n8: f64 = (s.dn[140][8] + eq153_e1758_d_n8);
        let eq153_e1759_d_n9: f64 = (s.dn[140][9] + eq153_e1758_d_n9);
        (eq153_e1759, s.dn[140][0], s.dn[140][1], s.dn[140][2], s.dn[140][3], s.dn[140][4], s.dn[140][5], s.dn[140][6], s.dn[140][7], eq153_e1759_d_n8, eq153_e1759_d_n9, s.dn[140][10], s.dn[140][11], s.dn[140][12], s.dn[140][13], s.dn[140][14], s.dn[140][15], s.dn[140][16], s.dn[140][17], s.dn[140][18], s.dn[140][19], s.dn[140][20], s.dn[140][21], s.dn[140][22], s.dn[140][23], s.dn[140][24], s.dn[140][25], s.dn[140][26], s.dn[140][27], s.dn[140][28], s.dn[140][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq153_value: f64 = eq153_e1761;
        let eq153_node_derivatives: [f64; 30] = [eq153_e1761_d_n0, eq153_e1761_d_n1, eq153_e1761_d_n2, eq153_e1761_d_n3, eq153_e1761_d_n4, eq153_e1761_d_n5, eq153_e1761_d_n6, eq153_e1761_d_n7, eq153_e1761_d_n8, eq153_e1761_d_n9, eq153_e1761_d_n10, eq153_e1761_d_n11, eq153_e1761_d_n12, eq153_e1761_d_n13, eq153_e1761_d_n14, eq153_e1761_d_n15, eq153_e1761_d_n16, eq153_e1761_d_n17, eq153_e1761_d_n18, eq153_e1761_d_n19, eq153_e1761_d_n20, eq153_e1761_d_n21, eq153_e1761_d_n22, eq153_e1761_d_n23, eq153_e1761_d_n24, eq153_e1761_d_n25, eq153_e1761_d_n26, eq153_e1761_d_n27, eq153_e1761_d_n28, eq153_e1761_d_n29];
        let eq153_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[9]),
            self.multiplicity * (eq153_value),
            &nodes,
            &eq153_node_derivatives,
            &branches,
            &eq153_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_154_block_0(
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
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq154_e1773, eq154_e1773_d_n0, eq154_e1773_d_n1, eq154_e1773_d_n2, eq154_e1773_d_n3, eq154_e1773_d_n4, eq154_e1773_d_n5, eq154_e1773_d_n6, eq154_e1773_d_n7, eq154_e1773_d_n8, eq154_e1773_d_n9, eq154_e1773_d_n10, eq154_e1773_d_n11, eq154_e1773_d_n12, eq154_e1773_d_n13, eq154_e1773_d_n14, eq154_e1773_d_n15, eq154_e1773_d_n16, eq154_e1773_d_n17, eq154_e1773_d_n18, eq154_e1773_d_n19, eq154_e1773_d_n20, eq154_e1773_d_n21, eq154_e1773_d_n22, eq154_e1773_d_n23, eq154_e1773_d_n24, eq154_e1773_d_n25, eq154_e1773_d_n26, eq154_e1773_d_n27, eq154_e1773_d_n28, eq154_e1773_d_n29,) = {
    if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
        let eq154_e1770: f64 = (s.v[0] * (nv8 - nv5));
        let eq154_e1770_d_n5: f64 = (-s.v[0]);
        let eq154_e1770_d_n8: f64 = s.v[0];
        let eq154_e1771: f64 = (s.v[141] + eq154_e1770);
        let eq154_e1771_d_n5: f64 = (s.dn[141][5] + eq154_e1770_d_n5);
        let eq154_e1771_d_n8: f64 = (s.dn[141][8] + eq154_e1770_d_n8);
        (eq154_e1771, s.dn[141][0], s.dn[141][1], s.dn[141][2], s.dn[141][3], s.dn[141][4], eq154_e1771_d_n5, s.dn[141][6], s.dn[141][7], eq154_e1771_d_n8, s.dn[141][9], s.dn[141][10], s.dn[141][11], s.dn[141][12], s.dn[141][13], s.dn[141][14], s.dn[141][15], s.dn[141][16], s.dn[141][17], s.dn[141][18], s.dn[141][19], s.dn[141][20], s.dn[141][21], s.dn[141][22], s.dn[141][23], s.dn[141][24], s.dn[141][25], s.dn[141][26], s.dn[141][27], s.dn[141][28], s.dn[141][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq154_value: f64 = eq154_e1773;
        let eq154_node_derivatives: [f64; 30] = [eq154_e1773_d_n0, eq154_e1773_d_n1, eq154_e1773_d_n2, eq154_e1773_d_n3, eq154_e1773_d_n4, eq154_e1773_d_n5, eq154_e1773_d_n6, eq154_e1773_d_n7, eq154_e1773_d_n8, eq154_e1773_d_n9, eq154_e1773_d_n10, eq154_e1773_d_n11, eq154_e1773_d_n12, eq154_e1773_d_n13, eq154_e1773_d_n14, eq154_e1773_d_n15, eq154_e1773_d_n16, eq154_e1773_d_n17, eq154_e1773_d_n18, eq154_e1773_d_n19, eq154_e1773_d_n20, eq154_e1773_d_n21, eq154_e1773_d_n22, eq154_e1773_d_n23, eq154_e1773_d_n24, eq154_e1773_d_n25, eq154_e1773_d_n26, eq154_e1773_d_n27, eq154_e1773_d_n28, eq154_e1773_d_n29];
        let eq154_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            self.multiplicity * (eq154_value),
            &nodes,
            &eq154_node_derivatives,
            &branches,
            &eq154_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_155_block_0(
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
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq155_e1781, eq155_e1781_d_n0, eq155_e1781_d_n1, eq155_e1781_d_n2, eq155_e1781_d_n3, eq155_e1781_d_n4, eq155_e1781_d_n5, eq155_e1781_d_n6, eq155_e1781_d_n7, eq155_e1781_d_n8, eq155_e1781_d_n9, eq155_e1781_d_n10, eq155_e1781_d_n11, eq155_e1781_d_n12, eq155_e1781_d_n13, eq155_e1781_d_n14, eq155_e1781_d_n15, eq155_e1781_d_n16, eq155_e1781_d_n17, eq155_e1781_d_n18, eq155_e1781_d_n19, eq155_e1781_d_n20, eq155_e1781_d_n21, eq155_e1781_d_n22, eq155_e1781_d_n23, eq155_e1781_d_n24, eq155_e1781_d_n25, eq155_e1781_d_n26, eq155_e1781_d_n27, eq155_e1781_d_n28, eq155_e1781_d_n29,) = {
    if (s.v[2418] != 0.0) {
        let eq155_e1778: f64 = (s.v[0] * (nv8 - nv7));
        let eq155_e1778_d_n7: f64 = (-s.v[0]);
        let eq155_e1778_d_n8: f64 = s.v[0];
        let eq155_e1779: f64 = (s.v[235] + eq155_e1778);
        let eq155_e1779_d_n7: f64 = (s.dn[235][7] + eq155_e1778_d_n7);
        let eq155_e1779_d_n8: f64 = (s.dn[235][8] + eq155_e1778_d_n8);
        (eq155_e1779, s.dn[235][0], s.dn[235][1], s.dn[235][2], s.dn[235][3], s.dn[235][4], s.dn[235][5], s.dn[235][6], eq155_e1779_d_n7, eq155_e1779_d_n8, s.dn[235][9], s.dn[235][10], s.dn[235][11], s.dn[235][12], s.dn[235][13], s.dn[235][14], s.dn[235][15], s.dn[235][16], s.dn[235][17], s.dn[235][18], s.dn[235][19], s.dn[235][20], s.dn[235][21], s.dn[235][22], s.dn[235][23], s.dn[235][24], s.dn[235][25], s.dn[235][26], s.dn[235][27], s.dn[235][28], s.dn[235][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq155_value: f64 = eq155_e1781;
        let eq155_node_derivatives: [f64; 30] = [eq155_e1781_d_n0, eq155_e1781_d_n1, eq155_e1781_d_n2, eq155_e1781_d_n3, eq155_e1781_d_n4, eq155_e1781_d_n5, eq155_e1781_d_n6, eq155_e1781_d_n7, eq155_e1781_d_n8, eq155_e1781_d_n9, eq155_e1781_d_n10, eq155_e1781_d_n11, eq155_e1781_d_n12, eq155_e1781_d_n13, eq155_e1781_d_n14, eq155_e1781_d_n15, eq155_e1781_d_n16, eq155_e1781_d_n17, eq155_e1781_d_n18, eq155_e1781_d_n19, eq155_e1781_d_n20, eq155_e1781_d_n21, eq155_e1781_d_n22, eq155_e1781_d_n23, eq155_e1781_d_n24, eq155_e1781_d_n25, eq155_e1781_d_n26, eq155_e1781_d_n27, eq155_e1781_d_n28, eq155_e1781_d_n29];
        let eq155_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            self.multiplicity * (eq155_value),
            &nodes,
            &eq155_node_derivatives,
            &branches,
            &eq155_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_156_block_0(
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
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq156_e1791, eq156_e1791_d_n0, eq156_e1791_d_n1, eq156_e1791_d_n2, eq156_e1791_d_n3, eq156_e1791_d_n4, eq156_e1791_d_n5, eq156_e1791_d_n6, eq156_e1791_d_n7, eq156_e1791_d_n8, eq156_e1791_d_n9, eq156_e1791_d_n10, eq156_e1791_d_n11, eq156_e1791_d_n12, eq156_e1791_d_n13, eq156_e1791_d_n14, eq156_e1791_d_n15, eq156_e1791_d_n16, eq156_e1791_d_n17, eq156_e1791_d_n18, eq156_e1791_d_n19, eq156_e1791_d_n20, eq156_e1791_d_n21, eq156_e1791_d_n22, eq156_e1791_d_n23, eq156_e1791_d_n24, eq156_e1791_d_n25, eq156_e1791_d_n26, eq156_e1791_d_n27, eq156_e1791_d_n28, eq156_e1791_d_n29,) = {
    if ((s.v[2418] != 0.0) && (s.v[2479] != 0.0)) {
        let eq156_e1788: f64 = (s.v[0] * (nv8 - nv7));
        let eq156_e1788_d_n7: f64 = (-s.v[0]);
        let eq156_e1788_d_n8: f64 = s.v[0];
        let eq156_e1789: f64 = (s.v[238] + eq156_e1788);
        let eq156_e1789_d_n7: f64 = (s.dn[238][7] + eq156_e1788_d_n7);
        let eq156_e1789_d_n8: f64 = (s.dn[238][8] + eq156_e1788_d_n8);
        (eq156_e1789, s.dn[238][0], s.dn[238][1], s.dn[238][2], s.dn[238][3], s.dn[238][4], s.dn[238][5], s.dn[238][6], eq156_e1789_d_n7, eq156_e1789_d_n8, s.dn[238][9], s.dn[238][10], s.dn[238][11], s.dn[238][12], s.dn[238][13], s.dn[238][14], s.dn[238][15], s.dn[238][16], s.dn[238][17], s.dn[238][18], s.dn[238][19], s.dn[238][20], s.dn[238][21], s.dn[238][22], s.dn[238][23], s.dn[238][24], s.dn[238][25], s.dn[238][26], s.dn[238][27], s.dn[238][28], s.dn[238][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq156_value: f64 = eq156_e1791;
        let eq156_node_derivatives: [f64; 30] = [eq156_e1791_d_n0, eq156_e1791_d_n1, eq156_e1791_d_n2, eq156_e1791_d_n3, eq156_e1791_d_n4, eq156_e1791_d_n5, eq156_e1791_d_n6, eq156_e1791_d_n7, eq156_e1791_d_n8, eq156_e1791_d_n9, eq156_e1791_d_n10, eq156_e1791_d_n11, eq156_e1791_d_n12, eq156_e1791_d_n13, eq156_e1791_d_n14, eq156_e1791_d_n15, eq156_e1791_d_n16, eq156_e1791_d_n17, eq156_e1791_d_n18, eq156_e1791_d_n19, eq156_e1791_d_n20, eq156_e1791_d_n21, eq156_e1791_d_n22, eq156_e1791_d_n23, eq156_e1791_d_n24, eq156_e1791_d_n25, eq156_e1791_d_n26, eq156_e1791_d_n27, eq156_e1791_d_n28, eq156_e1791_d_n29];
        let eq156_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            self.multiplicity * (eq156_value),
            &nodes,
            &eq156_node_derivatives,
            &branches,
            &eq156_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_157_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq157_e1796, eq157_e1796_d_n0, eq157_e1796_d_n1, eq157_e1796_d_n2, eq157_e1796_d_n3, eq157_e1796_d_n4, eq157_e1796_d_n5, eq157_e1796_d_n6, eq157_e1796_d_n7, eq157_e1796_d_n8, eq157_e1796_d_n9, eq157_e1796_d_n10, eq157_e1796_d_n11, eq157_e1796_d_n12, eq157_e1796_d_n13, eq157_e1796_d_n14, eq157_e1796_d_n15, eq157_e1796_d_n16, eq157_e1796_d_n17, eq157_e1796_d_n18, eq157_e1796_d_n19, eq157_e1796_d_n20, eq157_e1796_d_n21, eq157_e1796_d_n22, eq157_e1796_d_n23, eq157_e1796_d_n24, eq157_e1796_d_n25, eq157_e1796_d_n26, eq157_e1796_d_n27, eq157_e1796_d_n28, eq157_e1796_d_n29,) = {
    if (s.v[2418] != 0.0) {
        let eq157_e1794: f64 = self.eval_ddt(138, s.v[242]);
        let eq157_e1794_d_n0: f64 = self.ddt_jacobian(s.dn[242][0]);
        let eq157_e1794_d_n1: f64 = self.ddt_jacobian(s.dn[242][1]);
        let eq157_e1794_d_n2: f64 = self.ddt_jacobian(s.dn[242][2]);
        let eq157_e1794_d_n3: f64 = self.ddt_jacobian(s.dn[242][3]);
        let eq157_e1794_d_n4: f64 = self.ddt_jacobian(s.dn[242][4]);
        let eq157_e1794_d_n5: f64 = self.ddt_jacobian(s.dn[242][5]);
        let eq157_e1794_d_n6: f64 = self.ddt_jacobian(s.dn[242][6]);
        let eq157_e1794_d_n7: f64 = self.ddt_jacobian(s.dn[242][7]);
        let eq157_e1794_d_n8: f64 = self.ddt_jacobian(s.dn[242][8]);
        let eq157_e1794_d_n9: f64 = self.ddt_jacobian(s.dn[242][9]);
        let eq157_e1794_d_n10: f64 = self.ddt_jacobian(s.dn[242][10]);
        let eq157_e1794_d_n11: f64 = self.ddt_jacobian(s.dn[242][11]);
        let eq157_e1794_d_n12: f64 = self.ddt_jacobian(s.dn[242][12]);
        let eq157_e1794_d_n13: f64 = self.ddt_jacobian(s.dn[242][13]);
        let eq157_e1794_d_n14: f64 = self.ddt_jacobian(s.dn[242][14]);
        let eq157_e1794_d_n15: f64 = self.ddt_jacobian(s.dn[242][15]);
        let eq157_e1794_d_n16: f64 = self.ddt_jacobian(s.dn[242][16]);
        let eq157_e1794_d_n17: f64 = self.ddt_jacobian(s.dn[242][17]);
        let eq157_e1794_d_n18: f64 = self.ddt_jacobian(s.dn[242][18]);
        let eq157_e1794_d_n19: f64 = self.ddt_jacobian(s.dn[242][19]);
        let eq157_e1794_d_n20: f64 = self.ddt_jacobian(s.dn[242][20]);
        let eq157_e1794_d_n21: f64 = self.ddt_jacobian(s.dn[242][21]);
        let eq157_e1794_d_n22: f64 = self.ddt_jacobian(s.dn[242][22]);
        let eq157_e1794_d_n23: f64 = self.ddt_jacobian(s.dn[242][23]);
        let eq157_e1794_d_n24: f64 = self.ddt_jacobian(s.dn[242][24]);
        let eq157_e1794_d_n25: f64 = self.ddt_jacobian(s.dn[242][25]);
        let eq157_e1794_d_n26: f64 = self.ddt_jacobian(s.dn[242][26]);
        let eq157_e1794_d_n27: f64 = self.ddt_jacobian(s.dn[242][27]);
        let eq157_e1794_d_n28: f64 = self.ddt_jacobian(s.dn[242][28]);
        let eq157_e1794_d_n29: f64 = self.ddt_jacobian(s.dn[242][29]);
        (eq157_e1794, eq157_e1794_d_n0, eq157_e1794_d_n1, eq157_e1794_d_n2, eq157_e1794_d_n3, eq157_e1794_d_n4, eq157_e1794_d_n5, eq157_e1794_d_n6, eq157_e1794_d_n7, eq157_e1794_d_n8, eq157_e1794_d_n9, eq157_e1794_d_n10, eq157_e1794_d_n11, eq157_e1794_d_n12, eq157_e1794_d_n13, eq157_e1794_d_n14, eq157_e1794_d_n15, eq157_e1794_d_n16, eq157_e1794_d_n17, eq157_e1794_d_n18, eq157_e1794_d_n19, eq157_e1794_d_n20, eq157_e1794_d_n21, eq157_e1794_d_n22, eq157_e1794_d_n23, eq157_e1794_d_n24, eq157_e1794_d_n25, eq157_e1794_d_n26, eq157_e1794_d_n27, eq157_e1794_d_n28, eq157_e1794_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq157_value: f64 = eq157_e1796;
        let eq157_node_derivatives: [f64; 30] = [eq157_e1796_d_n0, eq157_e1796_d_n1, eq157_e1796_d_n2, eq157_e1796_d_n3, eq157_e1796_d_n4, eq157_e1796_d_n5, eq157_e1796_d_n6, eq157_e1796_d_n7, eq157_e1796_d_n8, eq157_e1796_d_n9, eq157_e1796_d_n10, eq157_e1796_d_n11, eq157_e1796_d_n12, eq157_e1796_d_n13, eq157_e1796_d_n14, eq157_e1796_d_n15, eq157_e1796_d_n16, eq157_e1796_d_n17, eq157_e1796_d_n18, eq157_e1796_d_n19, eq157_e1796_d_n20, eq157_e1796_d_n21, eq157_e1796_d_n22, eq157_e1796_d_n23, eq157_e1796_d_n24, eq157_e1796_d_n25, eq157_e1796_d_n26, eq157_e1796_d_n27, eq157_e1796_d_n28, eq157_e1796_d_n29];
        let eq157_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            self.multiplicity * (eq157_value),
            &nodes,
            &eq157_node_derivatives,
            &branches,
            &eq157_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_158_block_0(
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
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq158_e1804, eq158_e1804_d_n0, eq158_e1804_d_n1, eq158_e1804_d_n2, eq158_e1804_d_n3, eq158_e1804_d_n4, eq158_e1804_d_n5, eq158_e1804_d_n6, eq158_e1804_d_n7, eq158_e1804_d_n8, eq158_e1804_d_n9, eq158_e1804_d_n10, eq158_e1804_d_n11, eq158_e1804_d_n12, eq158_e1804_d_n13, eq158_e1804_d_n14, eq158_e1804_d_n15, eq158_e1804_d_n16, eq158_e1804_d_n17, eq158_e1804_d_n18, eq158_e1804_d_n19, eq158_e1804_d_n20, eq158_e1804_d_n21, eq158_e1804_d_n22, eq158_e1804_d_n23, eq158_e1804_d_n24, eq158_e1804_d_n25, eq158_e1804_d_n26, eq158_e1804_d_n27, eq158_e1804_d_n28, eq158_e1804_d_n29,) = {
    if ((s.v[2418] != 0.0) && (s.v[2546] != 0.0)) {
        let eq158_e1802: f64 = ((nv8 - nv7) / s.v[241]);
        let eq158_e1802_d_n0: f64 = (-(((nv8 - nv7) * s.dn[241][0]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n1: f64 = (-(((nv8 - nv7) * s.dn[241][1]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n2: f64 = (-(((nv8 - nv7) * s.dn[241][2]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n3: f64 = (-(((nv8 - nv7) * s.dn[241][3]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n4: f64 = (-(((nv8 - nv7) * s.dn[241][4]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n5: f64 = (-(((nv8 - nv7) * s.dn[241][5]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n6: f64 = (-(((nv8 - nv7) * s.dn[241][6]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n7: f64 = (((-s.v[241]) - ((nv8 - nv7) * s.dn[241][7])) / (s.v[241] * s.v[241]));
        let eq158_e1802_d_n8: f64 = ((s.v[241] - ((nv8 - nv7) * s.dn[241][8])) / (s.v[241] * s.v[241]));
        let eq158_e1802_d_n9: f64 = (-(((nv8 - nv7) * s.dn[241][9]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n10: f64 = (-(((nv8 - nv7) * s.dn[241][10]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n11: f64 = (-(((nv8 - nv7) * s.dn[241][11]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n12: f64 = (-(((nv8 - nv7) * s.dn[241][12]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n13: f64 = (-(((nv8 - nv7) * s.dn[241][13]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n14: f64 = (-(((nv8 - nv7) * s.dn[241][14]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n15: f64 = (-(((nv8 - nv7) * s.dn[241][15]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n16: f64 = (-(((nv8 - nv7) * s.dn[241][16]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n17: f64 = (-(((nv8 - nv7) * s.dn[241][17]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n18: f64 = (-(((nv8 - nv7) * s.dn[241][18]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n19: f64 = (-(((nv8 - nv7) * s.dn[241][19]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n20: f64 = (-(((nv8 - nv7) * s.dn[241][20]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n21: f64 = (-(((nv8 - nv7) * s.dn[241][21]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n22: f64 = (-(((nv8 - nv7) * s.dn[241][22]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n23: f64 = (-(((nv8 - nv7) * s.dn[241][23]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n24: f64 = (-(((nv8 - nv7) * s.dn[241][24]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n25: f64 = (-(((nv8 - nv7) * s.dn[241][25]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n26: f64 = (-(((nv8 - nv7) * s.dn[241][26]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n27: f64 = (-(((nv8 - nv7) * s.dn[241][27]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n28: f64 = (-(((nv8 - nv7) * s.dn[241][28]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n29: f64 = (-(((nv8 - nv7) * s.dn[241][29]) / (s.v[241] * s.v[241])));
        (eq158_e1802, eq158_e1802_d_n0, eq158_e1802_d_n1, eq158_e1802_d_n2, eq158_e1802_d_n3, eq158_e1802_d_n4, eq158_e1802_d_n5, eq158_e1802_d_n6, eq158_e1802_d_n7, eq158_e1802_d_n8, eq158_e1802_d_n9, eq158_e1802_d_n10, eq158_e1802_d_n11, eq158_e1802_d_n12, eq158_e1802_d_n13, eq158_e1802_d_n14, eq158_e1802_d_n15, eq158_e1802_d_n16, eq158_e1802_d_n17, eq158_e1802_d_n18, eq158_e1802_d_n19, eq158_e1802_d_n20, eq158_e1802_d_n21, eq158_e1802_d_n22, eq158_e1802_d_n23, eq158_e1802_d_n24, eq158_e1802_d_n25, eq158_e1802_d_n26, eq158_e1802_d_n27, eq158_e1802_d_n28, eq158_e1802_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq158_value: f64 = eq158_e1804;
        let eq158_node_derivatives: [f64; 30] = [eq158_e1804_d_n0, eq158_e1804_d_n1, eq158_e1804_d_n2, eq158_e1804_d_n3, eq158_e1804_d_n4, eq158_e1804_d_n5, eq158_e1804_d_n6, eq158_e1804_d_n7, eq158_e1804_d_n8, eq158_e1804_d_n9, eq158_e1804_d_n10, eq158_e1804_d_n11, eq158_e1804_d_n12, eq158_e1804_d_n13, eq158_e1804_d_n14, eq158_e1804_d_n15, eq158_e1804_d_n16, eq158_e1804_d_n17, eq158_e1804_d_n18, eq158_e1804_d_n19, eq158_e1804_d_n20, eq158_e1804_d_n21, eq158_e1804_d_n22, eq158_e1804_d_n23, eq158_e1804_d_n24, eq158_e1804_d_n25, eq158_e1804_d_n26, eq158_e1804_d_n27, eq158_e1804_d_n28, eq158_e1804_d_n29];
        let eq158_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            self.multiplicity * (eq158_value),
            &nodes,
            &eq158_node_derivatives,
            &branches,
            &eq158_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_159_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq159_e1809,) = {
    if (!(s.v[2418] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq159_value: f64 = eq159_e1809;
        stamper.stamp_potential(
            branches[30],
            eq159_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_160_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq160_e1815, eq160_e1815_d_n0, eq160_e1815_d_n1, eq160_e1815_d_n2, eq160_e1815_d_n3, eq160_e1815_d_n4, eq160_e1815_d_n5, eq160_e1815_d_n6, eq160_e1815_d_n7, eq160_e1815_d_n8, eq160_e1815_d_n9, eq160_e1815_d_n10, eq160_e1815_d_n11, eq160_e1815_d_n12, eq160_e1815_d_n13, eq160_e1815_d_n14, eq160_e1815_d_n15, eq160_e1815_d_n16, eq160_e1815_d_n17, eq160_e1815_d_n18, eq160_e1815_d_n19, eq160_e1815_d_n20, eq160_e1815_d_n21, eq160_e1815_d_n22, eq160_e1815_d_n23, eq160_e1815_d_n24, eq160_e1815_d_n25, eq160_e1815_d_n26, eq160_e1815_d_n27, eq160_e1815_d_n28, eq160_e1815_d_n29,) = {
    if ((s.v[2547] != 0.0) && (s.v[2669] != 0.0)) {
        (s.v[148], s.dn[148][0], s.dn[148][1], s.dn[148][2], s.dn[148][3], s.dn[148][4], s.dn[148][5], s.dn[148][6], s.dn[148][7], s.dn[148][8], s.dn[148][9], s.dn[148][10], s.dn[148][11], s.dn[148][12], s.dn[148][13], s.dn[148][14], s.dn[148][15], s.dn[148][16], s.dn[148][17], s.dn[148][18], s.dn[148][19], s.dn[148][20], s.dn[148][21], s.dn[148][22], s.dn[148][23], s.dn[148][24], s.dn[148][25], s.dn[148][26], s.dn[148][27], s.dn[148][28], s.dn[148][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq160_value: f64 = eq160_e1815;
        let eq160_node_derivatives: [f64; 30] = [eq160_e1815_d_n0, eq160_e1815_d_n1, eq160_e1815_d_n2, eq160_e1815_d_n3, eq160_e1815_d_n4, eq160_e1815_d_n5, eq160_e1815_d_n6, eq160_e1815_d_n7, eq160_e1815_d_n8, eq160_e1815_d_n9, eq160_e1815_d_n10, eq160_e1815_d_n11, eq160_e1815_d_n12, eq160_e1815_d_n13, eq160_e1815_d_n14, eq160_e1815_d_n15, eq160_e1815_d_n16, eq160_e1815_d_n17, eq160_e1815_d_n18, eq160_e1815_d_n19, eq160_e1815_d_n20, eq160_e1815_d_n21, eq160_e1815_d_n22, eq160_e1815_d_n23, eq160_e1815_d_n24, eq160_e1815_d_n25, eq160_e1815_d_n26, eq160_e1815_d_n27, eq160_e1815_d_n28, eq160_e1815_d_n29];
        let eq160_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[0]),
            self.multiplicity * (eq160_value),
            &nodes,
            &eq160_node_derivatives,
            &branches,
            &eq160_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_161_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq161_e1821, eq161_e1821_d_n0, eq161_e1821_d_n1, eq161_e1821_d_n2, eq161_e1821_d_n3, eq161_e1821_d_n4, eq161_e1821_d_n5, eq161_e1821_d_n6, eq161_e1821_d_n7, eq161_e1821_d_n8, eq161_e1821_d_n9, eq161_e1821_d_n10, eq161_e1821_d_n11, eq161_e1821_d_n12, eq161_e1821_d_n13, eq161_e1821_d_n14, eq161_e1821_d_n15, eq161_e1821_d_n16, eq161_e1821_d_n17, eq161_e1821_d_n18, eq161_e1821_d_n19, eq161_e1821_d_n20, eq161_e1821_d_n21, eq161_e1821_d_n22, eq161_e1821_d_n23, eq161_e1821_d_n24, eq161_e1821_d_n25, eq161_e1821_d_n26, eq161_e1821_d_n27, eq161_e1821_d_n28, eq161_e1821_d_n29,) = {
    if ((s.v[2547] != 0.0) && (s.v[2669] != 0.0)) {
        (s.v[149], s.dn[149][0], s.dn[149][1], s.dn[149][2], s.dn[149][3], s.dn[149][4], s.dn[149][5], s.dn[149][6], s.dn[149][7], s.dn[149][8], s.dn[149][9], s.dn[149][10], s.dn[149][11], s.dn[149][12], s.dn[149][13], s.dn[149][14], s.dn[149][15], s.dn[149][16], s.dn[149][17], s.dn[149][18], s.dn[149][19], s.dn[149][20], s.dn[149][21], s.dn[149][22], s.dn[149][23], s.dn[149][24], s.dn[149][25], s.dn[149][26], s.dn[149][27], s.dn[149][28], s.dn[149][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq161_value: f64 = eq161_e1821;
        let eq161_node_derivatives: [f64; 30] = [eq161_e1821_d_n0, eq161_e1821_d_n1, eq161_e1821_d_n2, eq161_e1821_d_n3, eq161_e1821_d_n4, eq161_e1821_d_n5, eq161_e1821_d_n6, eq161_e1821_d_n7, eq161_e1821_d_n8, eq161_e1821_d_n9, eq161_e1821_d_n10, eq161_e1821_d_n11, eq161_e1821_d_n12, eq161_e1821_d_n13, eq161_e1821_d_n14, eq161_e1821_d_n15, eq161_e1821_d_n16, eq161_e1821_d_n17, eq161_e1821_d_n18, eq161_e1821_d_n19, eq161_e1821_d_n20, eq161_e1821_d_n21, eq161_e1821_d_n22, eq161_e1821_d_n23, eq161_e1821_d_n24, eq161_e1821_d_n25, eq161_e1821_d_n26, eq161_e1821_d_n27, eq161_e1821_d_n28, eq161_e1821_d_n29];
        let eq161_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            self.multiplicity * (eq161_value),
            &nodes,
            &eq161_node_derivatives,
            &branches,
            &eq161_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_162_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq162_e1828, eq162_e1828_d_n0, eq162_e1828_d_n1, eq162_e1828_d_n2, eq162_e1828_d_n3, eq162_e1828_d_n4, eq162_e1828_d_n5, eq162_e1828_d_n6, eq162_e1828_d_n7, eq162_e1828_d_n8, eq162_e1828_d_n9, eq162_e1828_d_n10, eq162_e1828_d_n11, eq162_e1828_d_n12, eq162_e1828_d_n13, eq162_e1828_d_n14, eq162_e1828_d_n15, eq162_e1828_d_n16, eq162_e1828_d_n17, eq162_e1828_d_n18, eq162_e1828_d_n19, eq162_e1828_d_n20, eq162_e1828_d_n21, eq162_e1828_d_n22, eq162_e1828_d_n23, eq162_e1828_d_n24, eq162_e1828_d_n25, eq162_e1828_d_n26, eq162_e1828_d_n27, eq162_e1828_d_n28, eq162_e1828_d_n29,) = {
    if ((s.v[2547] != 0.0) && (!(s.v[2669] != 0.0))) {
        (s.v[148], s.dn[148][0], s.dn[148][1], s.dn[148][2], s.dn[148][3], s.dn[148][4], s.dn[148][5], s.dn[148][6], s.dn[148][7], s.dn[148][8], s.dn[148][9], s.dn[148][10], s.dn[148][11], s.dn[148][12], s.dn[148][13], s.dn[148][14], s.dn[148][15], s.dn[148][16], s.dn[148][17], s.dn[148][18], s.dn[148][19], s.dn[148][20], s.dn[148][21], s.dn[148][22], s.dn[148][23], s.dn[148][24], s.dn[148][25], s.dn[148][26], s.dn[148][27], s.dn[148][28], s.dn[148][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq162_value: f64 = eq162_e1828;
        let eq162_node_derivatives: [f64; 30] = [eq162_e1828_d_n0, eq162_e1828_d_n1, eq162_e1828_d_n2, eq162_e1828_d_n3, eq162_e1828_d_n4, eq162_e1828_d_n5, eq162_e1828_d_n6, eq162_e1828_d_n7, eq162_e1828_d_n8, eq162_e1828_d_n9, eq162_e1828_d_n10, eq162_e1828_d_n11, eq162_e1828_d_n12, eq162_e1828_d_n13, eq162_e1828_d_n14, eq162_e1828_d_n15, eq162_e1828_d_n16, eq162_e1828_d_n17, eq162_e1828_d_n18, eq162_e1828_d_n19, eq162_e1828_d_n20, eq162_e1828_d_n21, eq162_e1828_d_n22, eq162_e1828_d_n23, eq162_e1828_d_n24, eq162_e1828_d_n25, eq162_e1828_d_n26, eq162_e1828_d_n27, eq162_e1828_d_n28, eq162_e1828_d_n29];
        let eq162_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[19]),
            Some(nodes[18]),
            self.multiplicity * (eq162_value),
            &nodes,
            &eq162_node_derivatives,
            &branches,
            &eq162_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_163_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq163_e1835, eq163_e1835_d_n0, eq163_e1835_d_n1, eq163_e1835_d_n2, eq163_e1835_d_n3, eq163_e1835_d_n4, eq163_e1835_d_n5, eq163_e1835_d_n6, eq163_e1835_d_n7, eq163_e1835_d_n8, eq163_e1835_d_n9, eq163_e1835_d_n10, eq163_e1835_d_n11, eq163_e1835_d_n12, eq163_e1835_d_n13, eq163_e1835_d_n14, eq163_e1835_d_n15, eq163_e1835_d_n16, eq163_e1835_d_n17, eq163_e1835_d_n18, eq163_e1835_d_n19, eq163_e1835_d_n20, eq163_e1835_d_n21, eq163_e1835_d_n22, eq163_e1835_d_n23, eq163_e1835_d_n24, eq163_e1835_d_n25, eq163_e1835_d_n26, eq163_e1835_d_n27, eq163_e1835_d_n28, eq163_e1835_d_n29,) = {
    if ((s.v[2547] != 0.0) && (!(s.v[2669] != 0.0))) {
        (s.v[149], s.dn[149][0], s.dn[149][1], s.dn[149][2], s.dn[149][3], s.dn[149][4], s.dn[149][5], s.dn[149][6], s.dn[149][7], s.dn[149][8], s.dn[149][9], s.dn[149][10], s.dn[149][11], s.dn[149][12], s.dn[149][13], s.dn[149][14], s.dn[149][15], s.dn[149][16], s.dn[149][17], s.dn[149][18], s.dn[149][19], s.dn[149][20], s.dn[149][21], s.dn[149][22], s.dn[149][23], s.dn[149][24], s.dn[149][25], s.dn[149][26], s.dn[149][27], s.dn[149][28], s.dn[149][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq163_value: f64 = eq163_e1835;
        let eq163_node_derivatives: [f64; 30] = [eq163_e1835_d_n0, eq163_e1835_d_n1, eq163_e1835_d_n2, eq163_e1835_d_n3, eq163_e1835_d_n4, eq163_e1835_d_n5, eq163_e1835_d_n6, eq163_e1835_d_n7, eq163_e1835_d_n8, eq163_e1835_d_n9, eq163_e1835_d_n10, eq163_e1835_d_n11, eq163_e1835_d_n12, eq163_e1835_d_n13, eq163_e1835_d_n14, eq163_e1835_d_n15, eq163_e1835_d_n16, eq163_e1835_d_n17, eq163_e1835_d_n18, eq163_e1835_d_n19, eq163_e1835_d_n20, eq163_e1835_d_n21, eq163_e1835_d_n22, eq163_e1835_d_n23, eq163_e1835_d_n24, eq163_e1835_d_n25, eq163_e1835_d_n26, eq163_e1835_d_n27, eq163_e1835_d_n28, eq163_e1835_d_n29];
        let eq163_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[18]),
            Some(nodes[19]),
            self.multiplicity * (eq163_value),
            &nodes,
            &eq163_node_derivatives,
            &branches,
            &eq163_branch_derivatives,
            self.multiplicity,
        );
    }
}
