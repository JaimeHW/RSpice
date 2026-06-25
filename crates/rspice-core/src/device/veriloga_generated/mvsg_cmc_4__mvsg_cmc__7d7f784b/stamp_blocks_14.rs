#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_164_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let (eq164_e1841, eq164_e1841_d_n0, eq164_e1841_d_n1, eq164_e1841_d_n2, eq164_e1841_d_n3, eq164_e1841_d_n4, eq164_e1841_d_n5, eq164_e1841_d_n6, eq164_e1841_d_n7, eq164_e1841_d_n8, eq164_e1841_d_n9, eq164_e1841_d_n10, eq164_e1841_d_n11, eq164_e1841_d_n12, eq164_e1841_d_n13, eq164_e1841_d_n14, eq164_e1841_d_n15, eq164_e1841_d_n16, eq164_e1841_d_n17, eq164_e1841_d_n18, eq164_e1841_d_n19, eq164_e1841_d_n20, eq164_e1841_d_n21, eq164_e1841_d_n22, eq164_e1841_d_n23, eq164_e1841_d_n24, eq164_e1841_d_n25, eq164_e1841_d_n26, eq164_e1841_d_n27, eq164_e1841_d_n28, eq164_e1841_d_n29,) = {
    if (s.v[2670] != 0.0) {
        let eq164_e1839: f64 = ((nv0 - nv18) / s.v[1]);
        let eq164_e1839_d_n0: f64 = ((s.v[1] - ((nv0 - nv18) * s.dn[1][0])) / (s.v[1] * s.v[1]));
        let eq164_e1839_d_n1: f64 = (-(((nv0 - nv18) * s.dn[1][1]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n2: f64 = (-(((nv0 - nv18) * s.dn[1][2]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n3: f64 = (-(((nv0 - nv18) * s.dn[1][3]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n4: f64 = (-(((nv0 - nv18) * s.dn[1][4]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n5: f64 = (-(((nv0 - nv18) * s.dn[1][5]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n6: f64 = (-(((nv0 - nv18) * s.dn[1][6]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n7: f64 = (-(((nv0 - nv18) * s.dn[1][7]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n8: f64 = (-(((nv0 - nv18) * s.dn[1][8]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n9: f64 = (-(((nv0 - nv18) * s.dn[1][9]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n10: f64 = (-(((nv0 - nv18) * s.dn[1][10]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n11: f64 = (-(((nv0 - nv18) * s.dn[1][11]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n12: f64 = (-(((nv0 - nv18) * s.dn[1][12]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n13: f64 = (-(((nv0 - nv18) * s.dn[1][13]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n14: f64 = (-(((nv0 - nv18) * s.dn[1][14]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n15: f64 = (-(((nv0 - nv18) * s.dn[1][15]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n16: f64 = (-(((nv0 - nv18) * s.dn[1][16]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n17: f64 = (-(((nv0 - nv18) * s.dn[1][17]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n18: f64 = (((-s.v[1]) - ((nv0 - nv18) * s.dn[1][18])) / (s.v[1] * s.v[1]));
        let eq164_e1839_d_n19: f64 = (-(((nv0 - nv18) * s.dn[1][19]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n20: f64 = (-(((nv0 - nv18) * s.dn[1][20]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n21: f64 = (-(((nv0 - nv18) * s.dn[1][21]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n22: f64 = (-(((nv0 - nv18) * s.dn[1][22]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n23: f64 = (-(((nv0 - nv18) * s.dn[1][23]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n24: f64 = (-(((nv0 - nv18) * s.dn[1][24]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n25: f64 = (-(((nv0 - nv18) * s.dn[1][25]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n26: f64 = (-(((nv0 - nv18) * s.dn[1][26]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n27: f64 = (-(((nv0 - nv18) * s.dn[1][27]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n28: f64 = (-(((nv0 - nv18) * s.dn[1][28]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n29: f64 = (-(((nv0 - nv18) * s.dn[1][29]) / (s.v[1] * s.v[1])));
        (eq164_e1839, eq164_e1839_d_n0, eq164_e1839_d_n1, eq164_e1839_d_n2, eq164_e1839_d_n3, eq164_e1839_d_n4, eq164_e1839_d_n5, eq164_e1839_d_n6, eq164_e1839_d_n7, eq164_e1839_d_n8, eq164_e1839_d_n9, eq164_e1839_d_n10, eq164_e1839_d_n11, eq164_e1839_d_n12, eq164_e1839_d_n13, eq164_e1839_d_n14, eq164_e1839_d_n15, eq164_e1839_d_n16, eq164_e1839_d_n17, eq164_e1839_d_n18, eq164_e1839_d_n19, eq164_e1839_d_n20, eq164_e1839_d_n21, eq164_e1839_d_n22, eq164_e1839_d_n23, eq164_e1839_d_n24, eq164_e1839_d_n25, eq164_e1839_d_n26, eq164_e1839_d_n27, eq164_e1839_d_n28, eq164_e1839_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq164_value: f64 = eq164_e1841;
        let eq164_node_derivatives: [f64; 30] = [eq164_e1841_d_n0, eq164_e1841_d_n1, eq164_e1841_d_n2, eq164_e1841_d_n3, eq164_e1841_d_n4, eq164_e1841_d_n5, eq164_e1841_d_n6, eq164_e1841_d_n7, eq164_e1841_d_n8, eq164_e1841_d_n9, eq164_e1841_d_n10, eq164_e1841_d_n11, eq164_e1841_d_n12, eq164_e1841_d_n13, eq164_e1841_d_n14, eq164_e1841_d_n15, eq164_e1841_d_n16, eq164_e1841_d_n17, eq164_e1841_d_n18, eq164_e1841_d_n19, eq164_e1841_d_n20, eq164_e1841_d_n21, eq164_e1841_d_n22, eq164_e1841_d_n23, eq164_e1841_d_n24, eq164_e1841_d_n25, eq164_e1841_d_n26, eq164_e1841_d_n27, eq164_e1841_d_n28, eq164_e1841_d_n29];
        let eq164_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[18]),
            self.multiplicity * (eq164_value),
            &nodes,
            &eq164_node_derivatives,
            &branches,
            &eq164_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_165_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq165_e1846,) = {
    if (!(s.v[2670] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq165_value: f64 = eq165_e1846;
        stamper.stamp_potential(
            branches[31],
            eq165_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_166_block_0(
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
        let nv19 = ctx.node_voltage(nodes[19]);
        let (eq166_e1852, eq166_e1852_d_n0, eq166_e1852_d_n1, eq166_e1852_d_n2, eq166_e1852_d_n3, eq166_e1852_d_n4, eq166_e1852_d_n5, eq166_e1852_d_n6, eq166_e1852_d_n7, eq166_e1852_d_n8, eq166_e1852_d_n9, eq166_e1852_d_n10, eq166_e1852_d_n11, eq166_e1852_d_n12, eq166_e1852_d_n13, eq166_e1852_d_n14, eq166_e1852_d_n15, eq166_e1852_d_n16, eq166_e1852_d_n17, eq166_e1852_d_n18, eq166_e1852_d_n19, eq166_e1852_d_n20, eq166_e1852_d_n21, eq166_e1852_d_n22, eq166_e1852_d_n23, eq166_e1852_d_n24, eq166_e1852_d_n25, eq166_e1852_d_n26, eq166_e1852_d_n27, eq166_e1852_d_n28, eq166_e1852_d_n29,) = {
    if (s.v[2671] != 0.0) {
        let eq166_e1850: f64 = ((nv19 - nv2) / s.v[2]);
        let eq166_e1850_d_n0: f64 = (-(((nv19 - nv2) * s.dn[2][0]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n1: f64 = (-(((nv19 - nv2) * s.dn[2][1]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n2: f64 = (((-s.v[2]) - ((nv19 - nv2) * s.dn[2][2])) / (s.v[2] * s.v[2]));
        let eq166_e1850_d_n3: f64 = (-(((nv19 - nv2) * s.dn[2][3]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n4: f64 = (-(((nv19 - nv2) * s.dn[2][4]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n5: f64 = (-(((nv19 - nv2) * s.dn[2][5]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n6: f64 = (-(((nv19 - nv2) * s.dn[2][6]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n7: f64 = (-(((nv19 - nv2) * s.dn[2][7]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n8: f64 = (-(((nv19 - nv2) * s.dn[2][8]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n9: f64 = (-(((nv19 - nv2) * s.dn[2][9]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n10: f64 = (-(((nv19 - nv2) * s.dn[2][10]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n11: f64 = (-(((nv19 - nv2) * s.dn[2][11]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n12: f64 = (-(((nv19 - nv2) * s.dn[2][12]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n13: f64 = (-(((nv19 - nv2) * s.dn[2][13]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n14: f64 = (-(((nv19 - nv2) * s.dn[2][14]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n15: f64 = (-(((nv19 - nv2) * s.dn[2][15]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n16: f64 = (-(((nv19 - nv2) * s.dn[2][16]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n17: f64 = (-(((nv19 - nv2) * s.dn[2][17]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n18: f64 = (-(((nv19 - nv2) * s.dn[2][18]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n19: f64 = ((s.v[2] - ((nv19 - nv2) * s.dn[2][19])) / (s.v[2] * s.v[2]));
        let eq166_e1850_d_n20: f64 = (-(((nv19 - nv2) * s.dn[2][20]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n21: f64 = (-(((nv19 - nv2) * s.dn[2][21]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n22: f64 = (-(((nv19 - nv2) * s.dn[2][22]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n23: f64 = (-(((nv19 - nv2) * s.dn[2][23]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n24: f64 = (-(((nv19 - nv2) * s.dn[2][24]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n25: f64 = (-(((nv19 - nv2) * s.dn[2][25]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n26: f64 = (-(((nv19 - nv2) * s.dn[2][26]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n27: f64 = (-(((nv19 - nv2) * s.dn[2][27]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n28: f64 = (-(((nv19 - nv2) * s.dn[2][28]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n29: f64 = (-(((nv19 - nv2) * s.dn[2][29]) / (s.v[2] * s.v[2])));
        (eq166_e1850, eq166_e1850_d_n0, eq166_e1850_d_n1, eq166_e1850_d_n2, eq166_e1850_d_n3, eq166_e1850_d_n4, eq166_e1850_d_n5, eq166_e1850_d_n6, eq166_e1850_d_n7, eq166_e1850_d_n8, eq166_e1850_d_n9, eq166_e1850_d_n10, eq166_e1850_d_n11, eq166_e1850_d_n12, eq166_e1850_d_n13, eq166_e1850_d_n14, eq166_e1850_d_n15, eq166_e1850_d_n16, eq166_e1850_d_n17, eq166_e1850_d_n18, eq166_e1850_d_n19, eq166_e1850_d_n20, eq166_e1850_d_n21, eq166_e1850_d_n22, eq166_e1850_d_n23, eq166_e1850_d_n24, eq166_e1850_d_n25, eq166_e1850_d_n26, eq166_e1850_d_n27, eq166_e1850_d_n28, eq166_e1850_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq166_value: f64 = eq166_e1852;
        let eq166_node_derivatives: [f64; 30] = [eq166_e1852_d_n0, eq166_e1852_d_n1, eq166_e1852_d_n2, eq166_e1852_d_n3, eq166_e1852_d_n4, eq166_e1852_d_n5, eq166_e1852_d_n6, eq166_e1852_d_n7, eq166_e1852_d_n8, eq166_e1852_d_n9, eq166_e1852_d_n10, eq166_e1852_d_n11, eq166_e1852_d_n12, eq166_e1852_d_n13, eq166_e1852_d_n14, eq166_e1852_d_n15, eq166_e1852_d_n16, eq166_e1852_d_n17, eq166_e1852_d_n18, eq166_e1852_d_n19, eq166_e1852_d_n20, eq166_e1852_d_n21, eq166_e1852_d_n22, eq166_e1852_d_n23, eq166_e1852_d_n24, eq166_e1852_d_n25, eq166_e1852_d_n26, eq166_e1852_d_n27, eq166_e1852_d_n28, eq166_e1852_d_n29];
        let eq166_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[19]),
            Some(nodes[2]),
            self.multiplicity * (eq166_value),
            &nodes,
            &eq166_node_derivatives,
            &branches,
            &eq166_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_167_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq167_e1857,) = {
    if (!(s.v[2671] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq167_value: f64 = eq167_e1857;
        stamper.stamp_potential(
            branches[32],
            eq167_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_168_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq168_e1863, eq168_e1863_d_n1, eq168_e1863_d_n6,) = {
    if (s.v[2672] != 0.0) {
        let eq168_e1861: f64 = ((nv1 - nv6) / s.v[5]);
        let eq168_e1861_d_n1: f64 = (1.0 / s.v[5]);
        let eq168_e1861_d_n6: f64 = (-1.0 / s.v[5]);
        (eq168_e1861, eq168_e1861_d_n1, eq168_e1861_d_n6,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq168_value: f64 = eq168_e1863;
        stamper.stamp_current(
            Some(nodes[1]),
            Some(nodes[6]),
            self.multiplicity * (eq168_value),
            &[
                GeneratedDerivative::node(nodes[1], self.multiplicity * eq168_e1863_d_n1),
                GeneratedDerivative::node(nodes[6], self.multiplicity * eq168_e1863_d_n6),
            ],
        );
    }

    pub(super) fn stamp_transient_equation_169_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq169_e1868,) = {
    if (!(s.v[2672] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq169_value: f64 = eq169_e1868;
        stamper.stamp_potential(
            branches[33],
            eq169_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_170_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq170_e1874, eq170_e1874_d_n6, eq170_e1874_d_n7,) = {
    if (s.v[2673] != 0.0) {
        let eq170_e1872: f64 = ((nv6 - nv7) / s.v[6]);
        let eq170_e1872_d_n6: f64 = (1.0 / s.v[6]);
        let eq170_e1872_d_n7: f64 = (-1.0 / s.v[6]);
        (eq170_e1872, eq170_e1872_d_n6, eq170_e1872_d_n7,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq170_value: f64 = eq170_e1874;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq170_value),
            &[
                GeneratedDerivative::node(nodes[6], self.multiplicity * eq170_e1874_d_n6),
                GeneratedDerivative::node(nodes[7], self.multiplicity * eq170_e1874_d_n7),
            ],
        );
    }

    pub(super) fn stamp_transient_equation_171_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq171_e1879,) = {
    if (!(s.v[2673] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq171_value: f64 = eq171_e1879;
        stamper.stamp_potential(
            branches[34],
            eq171_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_172_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq172_e1881: f64 = self.eval_ddt(139, s.v[214]);
        let eq172_e1881_d_n0: f64 = self.ddt_jacobian(s.dn[214][0]);
        let eq172_e1881_d_n1: f64 = self.ddt_jacobian(s.dn[214][1]);
        let eq172_e1881_d_n2: f64 = self.ddt_jacobian(s.dn[214][2]);
        let eq172_e1881_d_n3: f64 = self.ddt_jacobian(s.dn[214][3]);
        let eq172_e1881_d_n4: f64 = self.ddt_jacobian(s.dn[214][4]);
        let eq172_e1881_d_n5: f64 = self.ddt_jacobian(s.dn[214][5]);
        let eq172_e1881_d_n6: f64 = self.ddt_jacobian(s.dn[214][6]);
        let eq172_e1881_d_n7: f64 = self.ddt_jacobian(s.dn[214][7]);
        let eq172_e1881_d_n8: f64 = self.ddt_jacobian(s.dn[214][8]);
        let eq172_e1881_d_n9: f64 = self.ddt_jacobian(s.dn[214][9]);
        let eq172_e1881_d_n10: f64 = self.ddt_jacobian(s.dn[214][10]);
        let eq172_e1881_d_n11: f64 = self.ddt_jacobian(s.dn[214][11]);
        let eq172_e1881_d_n12: f64 = self.ddt_jacobian(s.dn[214][12]);
        let eq172_e1881_d_n13: f64 = self.ddt_jacobian(s.dn[214][13]);
        let eq172_e1881_d_n14: f64 = self.ddt_jacobian(s.dn[214][14]);
        let eq172_e1881_d_n15: f64 = self.ddt_jacobian(s.dn[214][15]);
        let eq172_e1881_d_n16: f64 = self.ddt_jacobian(s.dn[214][16]);
        let eq172_e1881_d_n17: f64 = self.ddt_jacobian(s.dn[214][17]);
        let eq172_e1881_d_n18: f64 = self.ddt_jacobian(s.dn[214][18]);
        let eq172_e1881_d_n19: f64 = self.ddt_jacobian(s.dn[214][19]);
        let eq172_e1881_d_n20: f64 = self.ddt_jacobian(s.dn[214][20]);
        let eq172_e1881_d_n21: f64 = self.ddt_jacobian(s.dn[214][21]);
        let eq172_e1881_d_n22: f64 = self.ddt_jacobian(s.dn[214][22]);
        let eq172_e1881_d_n23: f64 = self.ddt_jacobian(s.dn[214][23]);
        let eq172_e1881_d_n24: f64 = self.ddt_jacobian(s.dn[214][24]);
        let eq172_e1881_d_n25: f64 = self.ddt_jacobian(s.dn[214][25]);
        let eq172_e1881_d_n26: f64 = self.ddt_jacobian(s.dn[214][26]);
        let eq172_e1881_d_n27: f64 = self.ddt_jacobian(s.dn[214][27]);
        let eq172_e1881_d_n28: f64 = self.ddt_jacobian(s.dn[214][28]);
        let eq172_e1881_d_n29: f64 = self.ddt_jacobian(s.dn[214][29]);
        let eq172_value: f64 = eq172_e1881;
        let eq172_node_derivatives: [f64; 30] = [eq172_e1881_d_n0, eq172_e1881_d_n1, eq172_e1881_d_n2, eq172_e1881_d_n3, eq172_e1881_d_n4, eq172_e1881_d_n5, eq172_e1881_d_n6, eq172_e1881_d_n7, eq172_e1881_d_n8, eq172_e1881_d_n9, eq172_e1881_d_n10, eq172_e1881_d_n11, eq172_e1881_d_n12, eq172_e1881_d_n13, eq172_e1881_d_n14, eq172_e1881_d_n15, eq172_e1881_d_n16, eq172_e1881_d_n17, eq172_e1881_d_n18, eq172_e1881_d_n19, eq172_e1881_d_n20, eq172_e1881_d_n21, eq172_e1881_d_n22, eq172_e1881_d_n23, eq172_e1881_d_n24, eq172_e1881_d_n25, eq172_e1881_d_n26, eq172_e1881_d_n27, eq172_e1881_d_n28, eq172_e1881_d_n29];
        let eq172_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[2]),
            self.multiplicity * (eq172_value),
            &nodes,
            &eq172_node_derivatives,
            &branches,
            &eq172_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_173_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq173_e1883: f64 = self.eval_ddt(140, s.v[215]);
        let eq173_e1883_d_n0: f64 = self.ddt_jacobian(s.dn[215][0]);
        let eq173_e1883_d_n1: f64 = self.ddt_jacobian(s.dn[215][1]);
        let eq173_e1883_d_n2: f64 = self.ddt_jacobian(s.dn[215][2]);
        let eq173_e1883_d_n3: f64 = self.ddt_jacobian(s.dn[215][3]);
        let eq173_e1883_d_n4: f64 = self.ddt_jacobian(s.dn[215][4]);
        let eq173_e1883_d_n5: f64 = self.ddt_jacobian(s.dn[215][5]);
        let eq173_e1883_d_n6: f64 = self.ddt_jacobian(s.dn[215][6]);
        let eq173_e1883_d_n7: f64 = self.ddt_jacobian(s.dn[215][7]);
        let eq173_e1883_d_n8: f64 = self.ddt_jacobian(s.dn[215][8]);
        let eq173_e1883_d_n9: f64 = self.ddt_jacobian(s.dn[215][9]);
        let eq173_e1883_d_n10: f64 = self.ddt_jacobian(s.dn[215][10]);
        let eq173_e1883_d_n11: f64 = self.ddt_jacobian(s.dn[215][11]);
        let eq173_e1883_d_n12: f64 = self.ddt_jacobian(s.dn[215][12]);
        let eq173_e1883_d_n13: f64 = self.ddt_jacobian(s.dn[215][13]);
        let eq173_e1883_d_n14: f64 = self.ddt_jacobian(s.dn[215][14]);
        let eq173_e1883_d_n15: f64 = self.ddt_jacobian(s.dn[215][15]);
        let eq173_e1883_d_n16: f64 = self.ddt_jacobian(s.dn[215][16]);
        let eq173_e1883_d_n17: f64 = self.ddt_jacobian(s.dn[215][17]);
        let eq173_e1883_d_n18: f64 = self.ddt_jacobian(s.dn[215][18]);
        let eq173_e1883_d_n19: f64 = self.ddt_jacobian(s.dn[215][19]);
        let eq173_e1883_d_n20: f64 = self.ddt_jacobian(s.dn[215][20]);
        let eq173_e1883_d_n21: f64 = self.ddt_jacobian(s.dn[215][21]);
        let eq173_e1883_d_n22: f64 = self.ddt_jacobian(s.dn[215][22]);
        let eq173_e1883_d_n23: f64 = self.ddt_jacobian(s.dn[215][23]);
        let eq173_e1883_d_n24: f64 = self.ddt_jacobian(s.dn[215][24]);
        let eq173_e1883_d_n25: f64 = self.ddt_jacobian(s.dn[215][25]);
        let eq173_e1883_d_n26: f64 = self.ddt_jacobian(s.dn[215][26]);
        let eq173_e1883_d_n27: f64 = self.ddt_jacobian(s.dn[215][27]);
        let eq173_e1883_d_n28: f64 = self.ddt_jacobian(s.dn[215][28]);
        let eq173_e1883_d_n29: f64 = self.ddt_jacobian(s.dn[215][29]);
        let eq173_value: f64 = eq173_e1883;
        let eq173_node_derivatives: [f64; 30] = [eq173_e1883_d_n0, eq173_e1883_d_n1, eq173_e1883_d_n2, eq173_e1883_d_n3, eq173_e1883_d_n4, eq173_e1883_d_n5, eq173_e1883_d_n6, eq173_e1883_d_n7, eq173_e1883_d_n8, eq173_e1883_d_n9, eq173_e1883_d_n10, eq173_e1883_d_n11, eq173_e1883_d_n12, eq173_e1883_d_n13, eq173_e1883_d_n14, eq173_e1883_d_n15, eq173_e1883_d_n16, eq173_e1883_d_n17, eq173_e1883_d_n18, eq173_e1883_d_n19, eq173_e1883_d_n20, eq173_e1883_d_n21, eq173_e1883_d_n22, eq173_e1883_d_n23, eq173_e1883_d_n24, eq173_e1883_d_n25, eq173_e1883_d_n26, eq173_e1883_d_n27, eq173_e1883_d_n28, eq173_e1883_d_n29];
        let eq173_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[0]),
            self.multiplicity * (eq173_value),
            &nodes,
            &eq173_node_derivatives,
            &branches,
            &eq173_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_174_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq174_e1885: f64 = self.eval_ddt(141, s.v[216]);
        let eq174_e1885_d_n0: f64 = self.ddt_jacobian(s.dn[216][0]);
        let eq174_e1885_d_n1: f64 = self.ddt_jacobian(s.dn[216][1]);
        let eq174_e1885_d_n2: f64 = self.ddt_jacobian(s.dn[216][2]);
        let eq174_e1885_d_n3: f64 = self.ddt_jacobian(s.dn[216][3]);
        let eq174_e1885_d_n4: f64 = self.ddt_jacobian(s.dn[216][4]);
        let eq174_e1885_d_n5: f64 = self.ddt_jacobian(s.dn[216][5]);
        let eq174_e1885_d_n6: f64 = self.ddt_jacobian(s.dn[216][6]);
        let eq174_e1885_d_n7: f64 = self.ddt_jacobian(s.dn[216][7]);
        let eq174_e1885_d_n8: f64 = self.ddt_jacobian(s.dn[216][8]);
        let eq174_e1885_d_n9: f64 = self.ddt_jacobian(s.dn[216][9]);
        let eq174_e1885_d_n10: f64 = self.ddt_jacobian(s.dn[216][10]);
        let eq174_e1885_d_n11: f64 = self.ddt_jacobian(s.dn[216][11]);
        let eq174_e1885_d_n12: f64 = self.ddt_jacobian(s.dn[216][12]);
        let eq174_e1885_d_n13: f64 = self.ddt_jacobian(s.dn[216][13]);
        let eq174_e1885_d_n14: f64 = self.ddt_jacobian(s.dn[216][14]);
        let eq174_e1885_d_n15: f64 = self.ddt_jacobian(s.dn[216][15]);
        let eq174_e1885_d_n16: f64 = self.ddt_jacobian(s.dn[216][16]);
        let eq174_e1885_d_n17: f64 = self.ddt_jacobian(s.dn[216][17]);
        let eq174_e1885_d_n18: f64 = self.ddt_jacobian(s.dn[216][18]);
        let eq174_e1885_d_n19: f64 = self.ddt_jacobian(s.dn[216][19]);
        let eq174_e1885_d_n20: f64 = self.ddt_jacobian(s.dn[216][20]);
        let eq174_e1885_d_n21: f64 = self.ddt_jacobian(s.dn[216][21]);
        let eq174_e1885_d_n22: f64 = self.ddt_jacobian(s.dn[216][22]);
        let eq174_e1885_d_n23: f64 = self.ddt_jacobian(s.dn[216][23]);
        let eq174_e1885_d_n24: f64 = self.ddt_jacobian(s.dn[216][24]);
        let eq174_e1885_d_n25: f64 = self.ddt_jacobian(s.dn[216][25]);
        let eq174_e1885_d_n26: f64 = self.ddt_jacobian(s.dn[216][26]);
        let eq174_e1885_d_n27: f64 = self.ddt_jacobian(s.dn[216][27]);
        let eq174_e1885_d_n28: f64 = self.ddt_jacobian(s.dn[216][28]);
        let eq174_e1885_d_n29: f64 = self.ddt_jacobian(s.dn[216][29]);
        let eq174_value: f64 = eq174_e1885;
        let eq174_node_derivatives: [f64; 30] = [eq174_e1885_d_n0, eq174_e1885_d_n1, eq174_e1885_d_n2, eq174_e1885_d_n3, eq174_e1885_d_n4, eq174_e1885_d_n5, eq174_e1885_d_n6, eq174_e1885_d_n7, eq174_e1885_d_n8, eq174_e1885_d_n9, eq174_e1885_d_n10, eq174_e1885_d_n11, eq174_e1885_d_n12, eq174_e1885_d_n13, eq174_e1885_d_n14, eq174_e1885_d_n15, eq174_e1885_d_n16, eq174_e1885_d_n17, eq174_e1885_d_n18, eq174_e1885_d_n19, eq174_e1885_d_n20, eq174_e1885_d_n21, eq174_e1885_d_n22, eq174_e1885_d_n23, eq174_e1885_d_n24, eq174_e1885_d_n25, eq174_e1885_d_n26, eq174_e1885_d_n27, eq174_e1885_d_n28, eq174_e1885_d_n29];
        let eq174_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[0]),
            self.multiplicity * (eq174_value),
            &nodes,
            &eq174_node_derivatives,
            &branches,
            &eq174_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_175_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq175_e1887: f64 = self.eval_ddt(142, s.v[218]);
        let eq175_e1887_d_n0: f64 = self.ddt_jacobian(s.dn[218][0]);
        let eq175_e1887_d_n1: f64 = self.ddt_jacobian(s.dn[218][1]);
        let eq175_e1887_d_n2: f64 = self.ddt_jacobian(s.dn[218][2]);
        let eq175_e1887_d_n3: f64 = self.ddt_jacobian(s.dn[218][3]);
        let eq175_e1887_d_n4: f64 = self.ddt_jacobian(s.dn[218][4]);
        let eq175_e1887_d_n5: f64 = self.ddt_jacobian(s.dn[218][5]);
        let eq175_e1887_d_n6: f64 = self.ddt_jacobian(s.dn[218][6]);
        let eq175_e1887_d_n7: f64 = self.ddt_jacobian(s.dn[218][7]);
        let eq175_e1887_d_n8: f64 = self.ddt_jacobian(s.dn[218][8]);
        let eq175_e1887_d_n9: f64 = self.ddt_jacobian(s.dn[218][9]);
        let eq175_e1887_d_n10: f64 = self.ddt_jacobian(s.dn[218][10]);
        let eq175_e1887_d_n11: f64 = self.ddt_jacobian(s.dn[218][11]);
        let eq175_e1887_d_n12: f64 = self.ddt_jacobian(s.dn[218][12]);
        let eq175_e1887_d_n13: f64 = self.ddt_jacobian(s.dn[218][13]);
        let eq175_e1887_d_n14: f64 = self.ddt_jacobian(s.dn[218][14]);
        let eq175_e1887_d_n15: f64 = self.ddt_jacobian(s.dn[218][15]);
        let eq175_e1887_d_n16: f64 = self.ddt_jacobian(s.dn[218][16]);
        let eq175_e1887_d_n17: f64 = self.ddt_jacobian(s.dn[218][17]);
        let eq175_e1887_d_n18: f64 = self.ddt_jacobian(s.dn[218][18]);
        let eq175_e1887_d_n19: f64 = self.ddt_jacobian(s.dn[218][19]);
        let eq175_e1887_d_n20: f64 = self.ddt_jacobian(s.dn[218][20]);
        let eq175_e1887_d_n21: f64 = self.ddt_jacobian(s.dn[218][21]);
        let eq175_e1887_d_n22: f64 = self.ddt_jacobian(s.dn[218][22]);
        let eq175_e1887_d_n23: f64 = self.ddt_jacobian(s.dn[218][23]);
        let eq175_e1887_d_n24: f64 = self.ddt_jacobian(s.dn[218][24]);
        let eq175_e1887_d_n25: f64 = self.ddt_jacobian(s.dn[218][25]);
        let eq175_e1887_d_n26: f64 = self.ddt_jacobian(s.dn[218][26]);
        let eq175_e1887_d_n27: f64 = self.ddt_jacobian(s.dn[218][27]);
        let eq175_e1887_d_n28: f64 = self.ddt_jacobian(s.dn[218][28]);
        let eq175_e1887_d_n29: f64 = self.ddt_jacobian(s.dn[218][29]);
        let eq175_value: f64 = eq175_e1887;
        let eq175_node_derivatives: [f64; 30] = [eq175_e1887_d_n0, eq175_e1887_d_n1, eq175_e1887_d_n2, eq175_e1887_d_n3, eq175_e1887_d_n4, eq175_e1887_d_n5, eq175_e1887_d_n6, eq175_e1887_d_n7, eq175_e1887_d_n8, eq175_e1887_d_n9, eq175_e1887_d_n10, eq175_e1887_d_n11, eq175_e1887_d_n12, eq175_e1887_d_n13, eq175_e1887_d_n14, eq175_e1887_d_n15, eq175_e1887_d_n16, eq175_e1887_d_n17, eq175_e1887_d_n18, eq175_e1887_d_n19, eq175_e1887_d_n20, eq175_e1887_d_n21, eq175_e1887_d_n22, eq175_e1887_d_n23, eq175_e1887_d_n24, eq175_e1887_d_n25, eq175_e1887_d_n26, eq175_e1887_d_n27, eq175_e1887_d_n28, eq175_e1887_d_n29];
        let eq175_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[2]),
            self.multiplicity * (eq175_value),
            &nodes,
            &eq175_node_derivatives,
            &branches,
            &eq175_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_176_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq176_e1889: f64 = self.eval_ddt(143, s.v[217]);
        let eq176_e1889_d_n0: f64 = self.ddt_jacobian(s.dn[217][0]);
        let eq176_e1889_d_n1: f64 = self.ddt_jacobian(s.dn[217][1]);
        let eq176_e1889_d_n2: f64 = self.ddt_jacobian(s.dn[217][2]);
        let eq176_e1889_d_n3: f64 = self.ddt_jacobian(s.dn[217][3]);
        let eq176_e1889_d_n4: f64 = self.ddt_jacobian(s.dn[217][4]);
        let eq176_e1889_d_n5: f64 = self.ddt_jacobian(s.dn[217][5]);
        let eq176_e1889_d_n6: f64 = self.ddt_jacobian(s.dn[217][6]);
        let eq176_e1889_d_n7: f64 = self.ddt_jacobian(s.dn[217][7]);
        let eq176_e1889_d_n8: f64 = self.ddt_jacobian(s.dn[217][8]);
        let eq176_e1889_d_n9: f64 = self.ddt_jacobian(s.dn[217][9]);
        let eq176_e1889_d_n10: f64 = self.ddt_jacobian(s.dn[217][10]);
        let eq176_e1889_d_n11: f64 = self.ddt_jacobian(s.dn[217][11]);
        let eq176_e1889_d_n12: f64 = self.ddt_jacobian(s.dn[217][12]);
        let eq176_e1889_d_n13: f64 = self.ddt_jacobian(s.dn[217][13]);
        let eq176_e1889_d_n14: f64 = self.ddt_jacobian(s.dn[217][14]);
        let eq176_e1889_d_n15: f64 = self.ddt_jacobian(s.dn[217][15]);
        let eq176_e1889_d_n16: f64 = self.ddt_jacobian(s.dn[217][16]);
        let eq176_e1889_d_n17: f64 = self.ddt_jacobian(s.dn[217][17]);
        let eq176_e1889_d_n18: f64 = self.ddt_jacobian(s.dn[217][18]);
        let eq176_e1889_d_n19: f64 = self.ddt_jacobian(s.dn[217][19]);
        let eq176_e1889_d_n20: f64 = self.ddt_jacobian(s.dn[217][20]);
        let eq176_e1889_d_n21: f64 = self.ddt_jacobian(s.dn[217][21]);
        let eq176_e1889_d_n22: f64 = self.ddt_jacobian(s.dn[217][22]);
        let eq176_e1889_d_n23: f64 = self.ddt_jacobian(s.dn[217][23]);
        let eq176_e1889_d_n24: f64 = self.ddt_jacobian(s.dn[217][24]);
        let eq176_e1889_d_n25: f64 = self.ddt_jacobian(s.dn[217][25]);
        let eq176_e1889_d_n26: f64 = self.ddt_jacobian(s.dn[217][26]);
        let eq176_e1889_d_n27: f64 = self.ddt_jacobian(s.dn[217][27]);
        let eq176_e1889_d_n28: f64 = self.ddt_jacobian(s.dn[217][28]);
        let eq176_e1889_d_n29: f64 = self.ddt_jacobian(s.dn[217][29]);
        let eq176_value: f64 = eq176_e1889;
        let eq176_node_derivatives: [f64; 30] = [eq176_e1889_d_n0, eq176_e1889_d_n1, eq176_e1889_d_n2, eq176_e1889_d_n3, eq176_e1889_d_n4, eq176_e1889_d_n5, eq176_e1889_d_n6, eq176_e1889_d_n7, eq176_e1889_d_n8, eq176_e1889_d_n9, eq176_e1889_d_n10, eq176_e1889_d_n11, eq176_e1889_d_n12, eq176_e1889_d_n13, eq176_e1889_d_n14, eq176_e1889_d_n15, eq176_e1889_d_n16, eq176_e1889_d_n17, eq176_e1889_d_n18, eq176_e1889_d_n19, eq176_e1889_d_n20, eq176_e1889_d_n21, eq176_e1889_d_n22, eq176_e1889_d_n23, eq176_e1889_d_n24, eq176_e1889_d_n25, eq176_e1889_d_n26, eq176_e1889_d_n27, eq176_e1889_d_n28, eq176_e1889_d_n29];
        let eq176_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[0]),
            self.multiplicity * (eq176_value),
            &nodes,
            &eq176_node_derivatives,
            &branches,
            &eq176_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_177_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq177_e1891: f64 = self.eval_ddt(144, s.v[219]);
        let eq177_e1891_d_n0: f64 = self.ddt_jacobian(s.dn[219][0]);
        let eq177_e1891_d_n1: f64 = self.ddt_jacobian(s.dn[219][1]);
        let eq177_e1891_d_n2: f64 = self.ddt_jacobian(s.dn[219][2]);
        let eq177_e1891_d_n3: f64 = self.ddt_jacobian(s.dn[219][3]);
        let eq177_e1891_d_n4: f64 = self.ddt_jacobian(s.dn[219][4]);
        let eq177_e1891_d_n5: f64 = self.ddt_jacobian(s.dn[219][5]);
        let eq177_e1891_d_n6: f64 = self.ddt_jacobian(s.dn[219][6]);
        let eq177_e1891_d_n7: f64 = self.ddt_jacobian(s.dn[219][7]);
        let eq177_e1891_d_n8: f64 = self.ddt_jacobian(s.dn[219][8]);
        let eq177_e1891_d_n9: f64 = self.ddt_jacobian(s.dn[219][9]);
        let eq177_e1891_d_n10: f64 = self.ddt_jacobian(s.dn[219][10]);
        let eq177_e1891_d_n11: f64 = self.ddt_jacobian(s.dn[219][11]);
        let eq177_e1891_d_n12: f64 = self.ddt_jacobian(s.dn[219][12]);
        let eq177_e1891_d_n13: f64 = self.ddt_jacobian(s.dn[219][13]);
        let eq177_e1891_d_n14: f64 = self.ddt_jacobian(s.dn[219][14]);
        let eq177_e1891_d_n15: f64 = self.ddt_jacobian(s.dn[219][15]);
        let eq177_e1891_d_n16: f64 = self.ddt_jacobian(s.dn[219][16]);
        let eq177_e1891_d_n17: f64 = self.ddt_jacobian(s.dn[219][17]);
        let eq177_e1891_d_n18: f64 = self.ddt_jacobian(s.dn[219][18]);
        let eq177_e1891_d_n19: f64 = self.ddt_jacobian(s.dn[219][19]);
        let eq177_e1891_d_n20: f64 = self.ddt_jacobian(s.dn[219][20]);
        let eq177_e1891_d_n21: f64 = self.ddt_jacobian(s.dn[219][21]);
        let eq177_e1891_d_n22: f64 = self.ddt_jacobian(s.dn[219][22]);
        let eq177_e1891_d_n23: f64 = self.ddt_jacobian(s.dn[219][23]);
        let eq177_e1891_d_n24: f64 = self.ddt_jacobian(s.dn[219][24]);
        let eq177_e1891_d_n25: f64 = self.ddt_jacobian(s.dn[219][25]);
        let eq177_e1891_d_n26: f64 = self.ddt_jacobian(s.dn[219][26]);
        let eq177_e1891_d_n27: f64 = self.ddt_jacobian(s.dn[219][27]);
        let eq177_e1891_d_n28: f64 = self.ddt_jacobian(s.dn[219][28]);
        let eq177_e1891_d_n29: f64 = self.ddt_jacobian(s.dn[219][29]);
        let eq177_value: f64 = eq177_e1891;
        let eq177_node_derivatives: [f64; 30] = [eq177_e1891_d_n0, eq177_e1891_d_n1, eq177_e1891_d_n2, eq177_e1891_d_n3, eq177_e1891_d_n4, eq177_e1891_d_n5, eq177_e1891_d_n6, eq177_e1891_d_n7, eq177_e1891_d_n8, eq177_e1891_d_n9, eq177_e1891_d_n10, eq177_e1891_d_n11, eq177_e1891_d_n12, eq177_e1891_d_n13, eq177_e1891_d_n14, eq177_e1891_d_n15, eq177_e1891_d_n16, eq177_e1891_d_n17, eq177_e1891_d_n18, eq177_e1891_d_n19, eq177_e1891_d_n20, eq177_e1891_d_n21, eq177_e1891_d_n22, eq177_e1891_d_n23, eq177_e1891_d_n24, eq177_e1891_d_n25, eq177_e1891_d_n26, eq177_e1891_d_n27, eq177_e1891_d_n28, eq177_e1891_d_n29];
        let eq177_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[3]),
            self.multiplicity * (eq177_value),
            &nodes,
            &eq177_node_derivatives,
            &branches,
            &eq177_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_178_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq178_e1908,) = {
    if (s.v[2686] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq178_value: f64 = eq178_e1908;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[9]),
            self.multiplicity * (eq178_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_179_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq179_e1925,) = {
    if (s.v[2686] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq179_value: f64 = eq179_e1925;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[5]),
            self.multiplicity * (eq179_value),
            &[
            ],
        );
    }
}
