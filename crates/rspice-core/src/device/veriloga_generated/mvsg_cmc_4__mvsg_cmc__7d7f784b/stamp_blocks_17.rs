#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_48_block_0(
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq48_e896, eq48_e896_d_n0, eq48_e896_d_n1, eq48_e896_d_n2, eq48_e896_d_n3, eq48_e896_d_n4, eq48_e896_d_n5, eq48_e896_d_n6, eq48_e896_d_n7, eq48_e896_d_n8, eq48_e896_d_n9, eq48_e896_d_n10, eq48_e896_d_n11, eq48_e896_d_n12, eq48_e896_d_n13, eq48_e896_d_n14, eq48_e896_d_n15, eq48_e896_d_n16, eq48_e896_d_n17, eq48_e896_d_n18, eq48_e896_d_n19, eq48_e896_d_n20, eq48_e896_d_n21, eq48_e896_d_n22, eq48_e896_d_n23, eq48_e896_d_n24, eq48_e896_d_n25, eq48_e896_d_n26, eq48_e896_d_n27, eq48_e896_d_n28, eq48_e896_d_n29, eq48_e896_q, eq48_e896_q_d_n0, eq48_e896_q_d_n1, eq48_e896_q_d_n2, eq48_e896_q_d_n3, eq48_e896_q_d_n4, eq48_e896_q_d_n5, eq48_e896_q_d_n6, eq48_e896_q_d_n7, eq48_e896_q_d_n8, eq48_e896_q_d_n9, eq48_e896_q_d_n10, eq48_e896_q_d_n11, eq48_e896_q_d_n12, eq48_e896_q_d_n13, eq48_e896_q_d_n14, eq48_e896_q_d_n15, eq48_e896_q_d_n16, eq48_e896_q_d_n17, eq48_e896_q_d_n18, eq48_e896_q_d_n19, eq48_e896_q_d_n20, eq48_e896_q_d_n21, eq48_e896_q_d_n22, eq48_e896_q_d_n23, eq48_e896_q_d_n24, eq48_e896_q_d_n25, eq48_e896_q_d_n26, eq48_e896_q_d_n27, eq48_e896_q_d_n28, eq48_e896_q_d_n29,) = {
    if (s.v[613] != 0.0) {
        let eq48_e889_q: f64 = s.v[205];
        let eq48_e892: f64 = (p.p355 * (nv2 - nv15));
        let eq48_e892_d_n2: f64 = p.p355;
        let eq48_e892_d_n15: f64 = (-p.p355);
        let eq48_e893_q: f64 = eq48_e892;
        let eq48_e894: f64 = (s.v[205] + eq48_e892);
        let eq48_e894_d_n2: f64 = (s.dn[205][2] + eq48_e892_d_n2);
        let eq48_e894_d_n15: f64 = (s.dn[205][15] + eq48_e892_d_n15);
        let eq48_e894_q: f64 = (eq48_e889_q + eq48_e893_q);
        let eq48_e894_q_d_n2: f64 = (s.dn[205][2] + eq48_e892_d_n2);
        let eq48_e894_q_d_n15: f64 = (s.dn[205][15] + eq48_e892_d_n15);
        (eq48_e894, s.dn[205][0], s.dn[205][1], eq48_e894_d_n2, s.dn[205][3], s.dn[205][4], s.dn[205][5], s.dn[205][6], s.dn[205][7], s.dn[205][8], s.dn[205][9], s.dn[205][10], s.dn[205][11], s.dn[205][12], s.dn[205][13], s.dn[205][14], eq48_e894_d_n15, s.dn[205][16], s.dn[205][17], s.dn[205][18], s.dn[205][19], s.dn[205][20], s.dn[205][21], s.dn[205][22], s.dn[205][23], s.dn[205][24], s.dn[205][25], s.dn[205][26], s.dn[205][27], s.dn[205][28], s.dn[205][29], eq48_e894_q, s.dn[205][0], s.dn[205][1], eq48_e894_q_d_n2, s.dn[205][3], s.dn[205][4], s.dn[205][5], s.dn[205][6], s.dn[205][7], s.dn[205][8], s.dn[205][9], s.dn[205][10], s.dn[205][11], s.dn[205][12], s.dn[205][13], s.dn[205][14], eq48_e894_q_d_n15, s.dn[205][16], s.dn[205][17], s.dn[205][18], s.dn[205][19], s.dn[205][20], s.dn[205][21], s.dn[205][22], s.dn[205][23], s.dn[205][24], s.dn[205][25], s.dn[205][26], s.dn[205][27], s.dn[205][28], s.dn[205][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_reactive_node_derivatives: [f64; 30] = [eq48_e896_q_d_n0, eq48_e896_q_d_n1, eq48_e896_q_d_n2, eq48_e896_q_d_n3, eq48_e896_q_d_n4, eq48_e896_q_d_n5, eq48_e896_q_d_n6, eq48_e896_q_d_n7, eq48_e896_q_d_n8, eq48_e896_q_d_n9, eq48_e896_q_d_n10, eq48_e896_q_d_n11, eq48_e896_q_d_n12, eq48_e896_q_d_n13, eq48_e896_q_d_n14, eq48_e896_q_d_n15, eq48_e896_q_d_n16, eq48_e896_q_d_n17, eq48_e896_q_d_n18, eq48_e896_q_d_n19, eq48_e896_q_d_n20, eq48_e896_q_d_n21, eq48_e896_q_d_n22, eq48_e896_q_d_n23, eq48_e896_q_d_n24, eq48_e896_q_d_n25, eq48_e896_q_d_n26, eq48_e896_q_d_n27, eq48_e896_q_d_n28, eq48_e896_q_d_n29];
        let eq48_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            &nodes,
            &eq48_reactive_node_derivatives,
            &branches,
            &eq48_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_50_block_0(
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
        let (eq50_e910, eq50_e910_d_n0, eq50_e910_d_n1, eq50_e910_d_n2, eq50_e910_d_n3, eq50_e910_d_n4, eq50_e910_d_n5, eq50_e910_d_n6, eq50_e910_d_n7, eq50_e910_d_n8, eq50_e910_d_n9, eq50_e910_d_n10, eq50_e910_d_n11, eq50_e910_d_n12, eq50_e910_d_n13, eq50_e910_d_n14, eq50_e910_d_n15, eq50_e910_d_n16, eq50_e910_d_n17, eq50_e910_d_n18, eq50_e910_d_n19, eq50_e910_d_n20, eq50_e910_d_n21, eq50_e910_d_n22, eq50_e910_d_n23, eq50_e910_d_n24, eq50_e910_d_n25, eq50_e910_d_n26, eq50_e910_d_n27, eq50_e910_d_n28, eq50_e910_d_n29, eq50_e910_q, eq50_e910_q_d_n0, eq50_e910_q_d_n1, eq50_e910_q_d_n2, eq50_e910_q_d_n3, eq50_e910_q_d_n4, eq50_e910_q_d_n5, eq50_e910_q_d_n6, eq50_e910_q_d_n7, eq50_e910_q_d_n8, eq50_e910_q_d_n9, eq50_e910_q_d_n10, eq50_e910_q_d_n11, eq50_e910_q_d_n12, eq50_e910_q_d_n13, eq50_e910_q_d_n14, eq50_e910_q_d_n15, eq50_e910_q_d_n16, eq50_e910_q_d_n17, eq50_e910_q_d_n18, eq50_e910_q_d_n19, eq50_e910_q_d_n20, eq50_e910_q_d_n21, eq50_e910_q_d_n22, eq50_e910_q_d_n23, eq50_e910_q_d_n24, eq50_e910_q_d_n25, eq50_e910_q_d_n26, eq50_e910_q_d_n27, eq50_e910_q_d_n28, eq50_e910_q_d_n29,) = {
    if (s.v[613] != 0.0) {
        let eq50_e903_q: f64 = s.v[207];
        let eq50_e906: f64 = (p.p355 * (nv7 - nv9));
        let eq50_e906_d_n7: f64 = p.p355;
        let eq50_e906_d_n9: f64 = (-p.p355);
        let eq50_e907_q: f64 = eq50_e906;
        let eq50_e908: f64 = (s.v[207] + eq50_e906);
        let eq50_e908_d_n7: f64 = (s.dn[207][7] + eq50_e906_d_n7);
        let eq50_e908_d_n9: f64 = (s.dn[207][9] + eq50_e906_d_n9);
        let eq50_e908_q: f64 = (eq50_e903_q + eq50_e907_q);
        let eq50_e908_q_d_n7: f64 = (s.dn[207][7] + eq50_e906_d_n7);
        let eq50_e908_q_d_n9: f64 = (s.dn[207][9] + eq50_e906_d_n9);
        (eq50_e908, s.dn[207][0], s.dn[207][1], s.dn[207][2], s.dn[207][3], s.dn[207][4], s.dn[207][5], s.dn[207][6], eq50_e908_d_n7, s.dn[207][8], eq50_e908_d_n9, s.dn[207][10], s.dn[207][11], s.dn[207][12], s.dn[207][13], s.dn[207][14], s.dn[207][15], s.dn[207][16], s.dn[207][17], s.dn[207][18], s.dn[207][19], s.dn[207][20], s.dn[207][21], s.dn[207][22], s.dn[207][23], s.dn[207][24], s.dn[207][25], s.dn[207][26], s.dn[207][27], s.dn[207][28], s.dn[207][29], eq50_e908_q, s.dn[207][0], s.dn[207][1], s.dn[207][2], s.dn[207][3], s.dn[207][4], s.dn[207][5], s.dn[207][6], eq50_e908_q_d_n7, s.dn[207][8], eq50_e908_q_d_n9, s.dn[207][10], s.dn[207][11], s.dn[207][12], s.dn[207][13], s.dn[207][14], s.dn[207][15], s.dn[207][16], s.dn[207][17], s.dn[207][18], s.dn[207][19], s.dn[207][20], s.dn[207][21], s.dn[207][22], s.dn[207][23], s.dn[207][24], s.dn[207][25], s.dn[207][26], s.dn[207][27], s.dn[207][28], s.dn[207][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_reactive_node_derivatives: [f64; 30] = [eq50_e910_q_d_n0, eq50_e910_q_d_n1, eq50_e910_q_d_n2, eq50_e910_q_d_n3, eq50_e910_q_d_n4, eq50_e910_q_d_n5, eq50_e910_q_d_n6, eq50_e910_q_d_n7, eq50_e910_q_d_n8, eq50_e910_q_d_n9, eq50_e910_q_d_n10, eq50_e910_q_d_n11, eq50_e910_q_d_n12, eq50_e910_q_d_n13, eq50_e910_q_d_n14, eq50_e910_q_d_n15, eq50_e910_q_d_n16, eq50_e910_q_d_n17, eq50_e910_q_d_n18, eq50_e910_q_d_n19, eq50_e910_q_d_n20, eq50_e910_q_d_n21, eq50_e910_q_d_n22, eq50_e910_q_d_n23, eq50_e910_q_d_n24, eq50_e910_q_d_n25, eq50_e910_q_d_n26, eq50_e910_q_d_n27, eq50_e910_q_d_n28, eq50_e910_q_d_n29];
        let eq50_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            &nodes,
            &eq50_reactive_node_derivatives,
            &branches,
            &eq50_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_51_block_0(
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq51_e921, eq51_e921_d_n0, eq51_e921_d_n1, eq51_e921_d_n2, eq51_e921_d_n3, eq51_e921_d_n4, eq51_e921_d_n5, eq51_e921_d_n6, eq51_e921_d_n7, eq51_e921_d_n8, eq51_e921_d_n9, eq51_e921_d_n10, eq51_e921_d_n11, eq51_e921_d_n12, eq51_e921_d_n13, eq51_e921_d_n14, eq51_e921_d_n15, eq51_e921_d_n16, eq51_e921_d_n17, eq51_e921_d_n18, eq51_e921_d_n19, eq51_e921_d_n20, eq51_e921_d_n21, eq51_e921_d_n22, eq51_e921_d_n23, eq51_e921_d_n24, eq51_e921_d_n25, eq51_e921_d_n26, eq51_e921_d_n27, eq51_e921_d_n28, eq51_e921_d_n29, eq51_e921_q, eq51_e921_q_d_n0, eq51_e921_q_d_n1, eq51_e921_q_d_n2, eq51_e921_q_d_n3, eq51_e921_q_d_n4, eq51_e921_q_d_n5, eq51_e921_q_d_n6, eq51_e921_q_d_n7, eq51_e921_q_d_n8, eq51_e921_q_d_n9, eq51_e921_q_d_n10, eq51_e921_q_d_n11, eq51_e921_q_d_n12, eq51_e921_q_d_n13, eq51_e921_q_d_n14, eq51_e921_q_d_n15, eq51_e921_q_d_n16, eq51_e921_q_d_n17, eq51_e921_q_d_n18, eq51_e921_q_d_n19, eq51_e921_q_d_n20, eq51_e921_q_d_n21, eq51_e921_q_d_n22, eq51_e921_q_d_n23, eq51_e921_q_d_n24, eq51_e921_q_d_n25, eq51_e921_q_d_n26, eq51_e921_q_d_n27, eq51_e921_q_d_n28, eq51_e921_q_d_n29,) = {
    if (!(s.v[613] != 0.0)) {
        let eq51_e914_q: f64 = s.v[203];
        let eq51_e917: f64 = (p.p355 * (nv2 - nv15));
        let eq51_e917_d_n2: f64 = p.p355;
        let eq51_e917_d_n15: f64 = (-p.p355);
        let eq51_e918_q: f64 = eq51_e917;
        let eq51_e919: f64 = (s.v[203] + eq51_e917);
        let eq51_e919_d_n2: f64 = (s.dn[203][2] + eq51_e917_d_n2);
        let eq51_e919_d_n15: f64 = (s.dn[203][15] + eq51_e917_d_n15);
        let eq51_e919_q: f64 = (eq51_e914_q + eq51_e918_q);
        let eq51_e919_q_d_n2: f64 = (s.dn[203][2] + eq51_e917_d_n2);
        let eq51_e919_q_d_n15: f64 = (s.dn[203][15] + eq51_e917_d_n15);
        (eq51_e919, s.dn[203][0], s.dn[203][1], eq51_e919_d_n2, s.dn[203][3], s.dn[203][4], s.dn[203][5], s.dn[203][6], s.dn[203][7], s.dn[203][8], s.dn[203][9], s.dn[203][10], s.dn[203][11], s.dn[203][12], s.dn[203][13], s.dn[203][14], eq51_e919_d_n15, s.dn[203][16], s.dn[203][17], s.dn[203][18], s.dn[203][19], s.dn[203][20], s.dn[203][21], s.dn[203][22], s.dn[203][23], s.dn[203][24], s.dn[203][25], s.dn[203][26], s.dn[203][27], s.dn[203][28], s.dn[203][29], eq51_e919_q, s.dn[203][0], s.dn[203][1], eq51_e919_q_d_n2, s.dn[203][3], s.dn[203][4], s.dn[203][5], s.dn[203][6], s.dn[203][7], s.dn[203][8], s.dn[203][9], s.dn[203][10], s.dn[203][11], s.dn[203][12], s.dn[203][13], s.dn[203][14], eq51_e919_q_d_n15, s.dn[203][16], s.dn[203][17], s.dn[203][18], s.dn[203][19], s.dn[203][20], s.dn[203][21], s.dn[203][22], s.dn[203][23], s.dn[203][24], s.dn[203][25], s.dn[203][26], s.dn[203][27], s.dn[203][28], s.dn[203][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_reactive_node_derivatives: [f64; 30] = [eq51_e921_q_d_n0, eq51_e921_q_d_n1, eq51_e921_q_d_n2, eq51_e921_q_d_n3, eq51_e921_q_d_n4, eq51_e921_q_d_n5, eq51_e921_q_d_n6, eq51_e921_q_d_n7, eq51_e921_q_d_n8, eq51_e921_q_d_n9, eq51_e921_q_d_n10, eq51_e921_q_d_n11, eq51_e921_q_d_n12, eq51_e921_q_d_n13, eq51_e921_q_d_n14, eq51_e921_q_d_n15, eq51_e921_q_d_n16, eq51_e921_q_d_n17, eq51_e921_q_d_n18, eq51_e921_q_d_n19, eq51_e921_q_d_n20, eq51_e921_q_d_n21, eq51_e921_q_d_n22, eq51_e921_q_d_n23, eq51_e921_q_d_n24, eq51_e921_q_d_n25, eq51_e921_q_d_n26, eq51_e921_q_d_n27, eq51_e921_q_d_n28, eq51_e921_q_d_n29];
        let eq51_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            &nodes,
            &eq51_reactive_node_derivatives,
            &branches,
            &eq51_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_52_block_0(
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
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq52_e932, eq52_e932_d_n0, eq52_e932_d_n1, eq52_e932_d_n2, eq52_e932_d_n3, eq52_e932_d_n4, eq52_e932_d_n5, eq52_e932_d_n6, eq52_e932_d_n7, eq52_e932_d_n8, eq52_e932_d_n9, eq52_e932_d_n10, eq52_e932_d_n11, eq52_e932_d_n12, eq52_e932_d_n13, eq52_e932_d_n14, eq52_e932_d_n15, eq52_e932_d_n16, eq52_e932_d_n17, eq52_e932_d_n18, eq52_e932_d_n19, eq52_e932_d_n20, eq52_e932_d_n21, eq52_e932_d_n22, eq52_e932_d_n23, eq52_e932_d_n24, eq52_e932_d_n25, eq52_e932_d_n26, eq52_e932_d_n27, eq52_e932_d_n28, eq52_e932_d_n29, eq52_e932_q, eq52_e932_q_d_n0, eq52_e932_q_d_n1, eq52_e932_q_d_n2, eq52_e932_q_d_n3, eq52_e932_q_d_n4, eq52_e932_q_d_n5, eq52_e932_q_d_n6, eq52_e932_q_d_n7, eq52_e932_q_d_n8, eq52_e932_q_d_n9, eq52_e932_q_d_n10, eq52_e932_q_d_n11, eq52_e932_q_d_n12, eq52_e932_q_d_n13, eq52_e932_q_d_n14, eq52_e932_q_d_n15, eq52_e932_q_d_n16, eq52_e932_q_d_n17, eq52_e932_q_d_n18, eq52_e932_q_d_n19, eq52_e932_q_d_n20, eq52_e932_q_d_n21, eq52_e932_q_d_n22, eq52_e932_q_d_n23, eq52_e932_q_d_n24, eq52_e932_q_d_n25, eq52_e932_q_d_n26, eq52_e932_q_d_n27, eq52_e932_q_d_n28, eq52_e932_q_d_n29,) = {
    if (!(s.v[613] != 0.0)) {
        let eq52_e925_q: f64 = s.v[204];
        let eq52_e928: f64 = (p.p355 * (nv2 - nv16));
        let eq52_e928_d_n2: f64 = p.p355;
        let eq52_e928_d_n16: f64 = (-p.p355);
        let eq52_e929_q: f64 = eq52_e928;
        let eq52_e930: f64 = (s.v[204] + eq52_e928);
        let eq52_e930_d_n2: f64 = (s.dn[204][2] + eq52_e928_d_n2);
        let eq52_e930_d_n16: f64 = (s.dn[204][16] + eq52_e928_d_n16);
        let eq52_e930_q: f64 = (eq52_e925_q + eq52_e929_q);
        let eq52_e930_q_d_n2: f64 = (s.dn[204][2] + eq52_e928_d_n2);
        let eq52_e930_q_d_n16: f64 = (s.dn[204][16] + eq52_e928_d_n16);
        (eq52_e930, s.dn[204][0], s.dn[204][1], eq52_e930_d_n2, s.dn[204][3], s.dn[204][4], s.dn[204][5], s.dn[204][6], s.dn[204][7], s.dn[204][8], s.dn[204][9], s.dn[204][10], s.dn[204][11], s.dn[204][12], s.dn[204][13], s.dn[204][14], s.dn[204][15], eq52_e930_d_n16, s.dn[204][17], s.dn[204][18], s.dn[204][19], s.dn[204][20], s.dn[204][21], s.dn[204][22], s.dn[204][23], s.dn[204][24], s.dn[204][25], s.dn[204][26], s.dn[204][27], s.dn[204][28], s.dn[204][29], eq52_e930_q, s.dn[204][0], s.dn[204][1], eq52_e930_q_d_n2, s.dn[204][3], s.dn[204][4], s.dn[204][5], s.dn[204][6], s.dn[204][7], s.dn[204][8], s.dn[204][9], s.dn[204][10], s.dn[204][11], s.dn[204][12], s.dn[204][13], s.dn[204][14], s.dn[204][15], eq52_e930_q_d_n16, s.dn[204][17], s.dn[204][18], s.dn[204][19], s.dn[204][20], s.dn[204][21], s.dn[204][22], s.dn[204][23], s.dn[204][24], s.dn[204][25], s.dn[204][26], s.dn[204][27], s.dn[204][28], s.dn[204][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_reactive_node_derivatives: [f64; 30] = [eq52_e932_q_d_n0, eq52_e932_q_d_n1, eq52_e932_q_d_n2, eq52_e932_q_d_n3, eq52_e932_q_d_n4, eq52_e932_q_d_n5, eq52_e932_q_d_n6, eq52_e932_q_d_n7, eq52_e932_q_d_n8, eq52_e932_q_d_n9, eq52_e932_q_d_n10, eq52_e932_q_d_n11, eq52_e932_q_d_n12, eq52_e932_q_d_n13, eq52_e932_q_d_n14, eq52_e932_q_d_n15, eq52_e932_q_d_n16, eq52_e932_q_d_n17, eq52_e932_q_d_n18, eq52_e932_q_d_n19, eq52_e932_q_d_n20, eq52_e932_q_d_n21, eq52_e932_q_d_n22, eq52_e932_q_d_n23, eq52_e932_q_d_n24, eq52_e932_q_d_n25, eq52_e932_q_d_n26, eq52_e932_q_d_n27, eq52_e932_q_d_n28, eq52_e932_q_d_n29];
        let eq52_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[16]),
            &nodes,
            &eq52_reactive_node_derivatives,
            &branches,
            &eq52_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_53_block_0(
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq53_e943, eq53_e943_d_n0, eq53_e943_d_n1, eq53_e943_d_n2, eq53_e943_d_n3, eq53_e943_d_n4, eq53_e943_d_n5, eq53_e943_d_n6, eq53_e943_d_n7, eq53_e943_d_n8, eq53_e943_d_n9, eq53_e943_d_n10, eq53_e943_d_n11, eq53_e943_d_n12, eq53_e943_d_n13, eq53_e943_d_n14, eq53_e943_d_n15, eq53_e943_d_n16, eq53_e943_d_n17, eq53_e943_d_n18, eq53_e943_d_n19, eq53_e943_d_n20, eq53_e943_d_n21, eq53_e943_d_n22, eq53_e943_d_n23, eq53_e943_d_n24, eq53_e943_d_n25, eq53_e943_d_n26, eq53_e943_d_n27, eq53_e943_d_n28, eq53_e943_d_n29, eq53_e943_q, eq53_e943_q_d_n0, eq53_e943_q_d_n1, eq53_e943_q_d_n2, eq53_e943_q_d_n3, eq53_e943_q_d_n4, eq53_e943_q_d_n5, eq53_e943_q_d_n6, eq53_e943_q_d_n7, eq53_e943_q_d_n8, eq53_e943_q_d_n9, eq53_e943_q_d_n10, eq53_e943_q_d_n11, eq53_e943_q_d_n12, eq53_e943_q_d_n13, eq53_e943_q_d_n14, eq53_e943_q_d_n15, eq53_e943_q_d_n16, eq53_e943_q_d_n17, eq53_e943_q_d_n18, eq53_e943_q_d_n19, eq53_e943_q_d_n20, eq53_e943_q_d_n21, eq53_e943_q_d_n22, eq53_e943_q_d_n23, eq53_e943_q_d_n24, eq53_e943_q_d_n25, eq53_e943_q_d_n26, eq53_e943_q_d_n27, eq53_e943_q_d_n28, eq53_e943_q_d_n29,) = {
    if (!(s.v[613] != 0.0)) {
        let eq53_e936_q: f64 = s.v[205];
        let eq53_e939: f64 = (p.p355 * (nv7 - nv15));
        let eq53_e939_d_n7: f64 = p.p355;
        let eq53_e939_d_n15: f64 = (-p.p355);
        let eq53_e940_q: f64 = eq53_e939;
        let eq53_e941: f64 = (s.v[205] + eq53_e939);
        let eq53_e941_d_n7: f64 = (s.dn[205][7] + eq53_e939_d_n7);
        let eq53_e941_d_n15: f64 = (s.dn[205][15] + eq53_e939_d_n15);
        let eq53_e941_q: f64 = (eq53_e936_q + eq53_e940_q);
        let eq53_e941_q_d_n7: f64 = (s.dn[205][7] + eq53_e939_d_n7);
        let eq53_e941_q_d_n15: f64 = (s.dn[205][15] + eq53_e939_d_n15);
        (eq53_e941, s.dn[205][0], s.dn[205][1], s.dn[205][2], s.dn[205][3], s.dn[205][4], s.dn[205][5], s.dn[205][6], eq53_e941_d_n7, s.dn[205][8], s.dn[205][9], s.dn[205][10], s.dn[205][11], s.dn[205][12], s.dn[205][13], s.dn[205][14], eq53_e941_d_n15, s.dn[205][16], s.dn[205][17], s.dn[205][18], s.dn[205][19], s.dn[205][20], s.dn[205][21], s.dn[205][22], s.dn[205][23], s.dn[205][24], s.dn[205][25], s.dn[205][26], s.dn[205][27], s.dn[205][28], s.dn[205][29], eq53_e941_q, s.dn[205][0], s.dn[205][1], s.dn[205][2], s.dn[205][3], s.dn[205][4], s.dn[205][5], s.dn[205][6], eq53_e941_q_d_n7, s.dn[205][8], s.dn[205][9], s.dn[205][10], s.dn[205][11], s.dn[205][12], s.dn[205][13], s.dn[205][14], eq53_e941_q_d_n15, s.dn[205][16], s.dn[205][17], s.dn[205][18], s.dn[205][19], s.dn[205][20], s.dn[205][21], s.dn[205][22], s.dn[205][23], s.dn[205][24], s.dn[205][25], s.dn[205][26], s.dn[205][27], s.dn[205][28], s.dn[205][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_reactive_node_derivatives: [f64; 30] = [eq53_e943_q_d_n0, eq53_e943_q_d_n1, eq53_e943_q_d_n2, eq53_e943_q_d_n3, eq53_e943_q_d_n4, eq53_e943_q_d_n5, eq53_e943_q_d_n6, eq53_e943_q_d_n7, eq53_e943_q_d_n8, eq53_e943_q_d_n9, eq53_e943_q_d_n10, eq53_e943_q_d_n11, eq53_e943_q_d_n12, eq53_e943_q_d_n13, eq53_e943_q_d_n14, eq53_e943_q_d_n15, eq53_e943_q_d_n16, eq53_e943_q_d_n17, eq53_e943_q_d_n18, eq53_e943_q_d_n19, eq53_e943_q_d_n20, eq53_e943_q_d_n21, eq53_e943_q_d_n22, eq53_e943_q_d_n23, eq53_e943_q_d_n24, eq53_e943_q_d_n25, eq53_e943_q_d_n26, eq53_e943_q_d_n27, eq53_e943_q_d_n28, eq53_e943_q_d_n29];
        let eq53_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[15]),
            &nodes,
            &eq53_reactive_node_derivatives,
            &branches,
            &eq53_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_56_block_0(
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let eq56_e955_q: f64 = s.v[206];
        let eq56_e958: f64 = (p.p355 * (nv3 - nv15));
        let eq56_e958_d_n3: f64 = p.p355;
        let eq56_e958_d_n15: f64 = (-p.p355);
        let eq56_e959_q: f64 = eq56_e958;
        let eq56_e960: f64 = (s.v[206] + eq56_e958);
        let eq56_e960_d_n3: f64 = (s.dn[206][3] + eq56_e958_d_n3);
        let eq56_e960_d_n15: f64 = (s.dn[206][15] + eq56_e958_d_n15);
        let eq56_e960_q: f64 = (eq56_e955_q + eq56_e959_q);
        let eq56_e960_q_d_n3: f64 = (s.dn[206][3] + eq56_e958_d_n3);
        let eq56_e960_q_d_n15: f64 = (s.dn[206][15] + eq56_e958_d_n15);
        let eq56_reactive_node_derivatives: [f64; 30] = [s.dn[206][0], s.dn[206][1], s.dn[206][2], eq56_e960_q_d_n3, s.dn[206][4], s.dn[206][5], s.dn[206][6], s.dn[206][7], s.dn[206][8], s.dn[206][9], s.dn[206][10], s.dn[206][11], s.dn[206][12], s.dn[206][13], s.dn[206][14], eq56_e960_q_d_n15, s.dn[206][16], s.dn[206][17], s.dn[206][18], s.dn[206][19], s.dn[206][20], s.dn[206][21], s.dn[206][22], s.dn[206][23], s.dn[206][24], s.dn[206][25], s.dn[206][26], s.dn[206][27], s.dn[206][28], s.dn[206][29]];
        let eq56_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[15]),
            &nodes,
            &eq56_reactive_node_derivatives,
            &branches,
            &eq56_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_59_block_0(
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
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq59_e983, eq59_e983_d_n0, eq59_e983_d_n1, eq59_e983_d_n2, eq59_e983_d_n3, eq59_e983_d_n4, eq59_e983_d_n5, eq59_e983_d_n6, eq59_e983_d_n7, eq59_e983_d_n8, eq59_e983_d_n9, eq59_e983_d_n10, eq59_e983_d_n11, eq59_e983_d_n12, eq59_e983_d_n13, eq59_e983_d_n14, eq59_e983_d_n15, eq59_e983_d_n16, eq59_e983_d_n17, eq59_e983_d_n18, eq59_e983_d_n19, eq59_e983_d_n20, eq59_e983_d_n21, eq59_e983_d_n22, eq59_e983_d_n23, eq59_e983_d_n24, eq59_e983_d_n25, eq59_e983_d_n26, eq59_e983_d_n27, eq59_e983_d_n28, eq59_e983_d_n29, eq59_e983_q, eq59_e983_q_d_n0, eq59_e983_q_d_n1, eq59_e983_q_d_n2, eq59_e983_q_d_n3, eq59_e983_q_d_n4, eq59_e983_q_d_n5, eq59_e983_q_d_n6, eq59_e983_q_d_n7, eq59_e983_q_d_n8, eq59_e983_q_d_n9, eq59_e983_q_d_n10, eq59_e983_q_d_n11, eq59_e983_q_d_n12, eq59_e983_q_d_n13, eq59_e983_q_d_n14, eq59_e983_q_d_n15, eq59_e983_q_d_n16, eq59_e983_q_d_n17, eq59_e983_q_d_n18, eq59_e983_q_d_n19, eq59_e983_q_d_n20, eq59_e983_q_d_n21, eq59_e983_q_d_n22, eq59_e983_q_d_n23, eq59_e983_q_d_n24, eq59_e983_q_d_n25, eq59_e983_q_d_n26, eq59_e983_q_d_n27, eq59_e983_q_d_n28, eq59_e983_q_d_n29,) = {
    if (s.v[760] != 0.0) {
        let eq59_e976_q: f64 = s.v[197];
        let eq59_e979: f64 = (p.p355 * (nv7 - nv14));
        let eq59_e979_d_n7: f64 = p.p355;
        let eq59_e979_d_n14: f64 = (-p.p355);
        let eq59_e980_q: f64 = eq59_e979;
        let eq59_e981: f64 = (s.v[197] + eq59_e979);
        let eq59_e981_d_n7: f64 = (s.dn[197][7] + eq59_e979_d_n7);
        let eq59_e981_d_n14: f64 = (s.dn[197][14] + eq59_e979_d_n14);
        let eq59_e981_q: f64 = (eq59_e976_q + eq59_e980_q);
        let eq59_e981_q_d_n7: f64 = (s.dn[197][7] + eq59_e979_d_n7);
        let eq59_e981_q_d_n14: f64 = (s.dn[197][14] + eq59_e979_d_n14);
        (eq59_e981, s.dn[197][0], s.dn[197][1], s.dn[197][2], s.dn[197][3], s.dn[197][4], s.dn[197][5], s.dn[197][6], eq59_e981_d_n7, s.dn[197][8], s.dn[197][9], s.dn[197][10], s.dn[197][11], s.dn[197][12], s.dn[197][13], eq59_e981_d_n14, s.dn[197][15], s.dn[197][16], s.dn[197][17], s.dn[197][18], s.dn[197][19], s.dn[197][20], s.dn[197][21], s.dn[197][22], s.dn[197][23], s.dn[197][24], s.dn[197][25], s.dn[197][26], s.dn[197][27], s.dn[197][28], s.dn[197][29], eq59_e981_q, s.dn[197][0], s.dn[197][1], s.dn[197][2], s.dn[197][3], s.dn[197][4], s.dn[197][5], s.dn[197][6], eq59_e981_q_d_n7, s.dn[197][8], s.dn[197][9], s.dn[197][10], s.dn[197][11], s.dn[197][12], s.dn[197][13], eq59_e981_q_d_n14, s.dn[197][15], s.dn[197][16], s.dn[197][17], s.dn[197][18], s.dn[197][19], s.dn[197][20], s.dn[197][21], s.dn[197][22], s.dn[197][23], s.dn[197][24], s.dn[197][25], s.dn[197][26], s.dn[197][27], s.dn[197][28], s.dn[197][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_reactive_node_derivatives: [f64; 30] = [eq59_e983_q_d_n0, eq59_e983_q_d_n1, eq59_e983_q_d_n2, eq59_e983_q_d_n3, eq59_e983_q_d_n4, eq59_e983_q_d_n5, eq59_e983_q_d_n6, eq59_e983_q_d_n7, eq59_e983_q_d_n8, eq59_e983_q_d_n9, eq59_e983_q_d_n10, eq59_e983_q_d_n11, eq59_e983_q_d_n12, eq59_e983_q_d_n13, eq59_e983_q_d_n14, eq59_e983_q_d_n15, eq59_e983_q_d_n16, eq59_e983_q_d_n17, eq59_e983_q_d_n18, eq59_e983_q_d_n19, eq59_e983_q_d_n20, eq59_e983_q_d_n21, eq59_e983_q_d_n22, eq59_e983_q_d_n23, eq59_e983_q_d_n24, eq59_e983_q_d_n25, eq59_e983_q_d_n26, eq59_e983_q_d_n27, eq59_e983_q_d_n28, eq59_e983_q_d_n29];
        let eq59_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[14]),
            &nodes,
            &eq59_reactive_node_derivatives,
            &branches,
            &eq59_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_60_block_0(
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq60_e993, eq60_e993_d_n0, eq60_e993_d_n1, eq60_e993_d_n2, eq60_e993_d_n3, eq60_e993_d_n4, eq60_e993_d_n5, eq60_e993_d_n6, eq60_e993_d_n7, eq60_e993_d_n8, eq60_e993_d_n9, eq60_e993_d_n10, eq60_e993_d_n11, eq60_e993_d_n12, eq60_e993_d_n13, eq60_e993_d_n14, eq60_e993_d_n15, eq60_e993_d_n16, eq60_e993_d_n17, eq60_e993_d_n18, eq60_e993_d_n19, eq60_e993_d_n20, eq60_e993_d_n21, eq60_e993_d_n22, eq60_e993_d_n23, eq60_e993_d_n24, eq60_e993_d_n25, eq60_e993_d_n26, eq60_e993_d_n27, eq60_e993_d_n28, eq60_e993_d_n29, eq60_e993_q, eq60_e993_q_d_n0, eq60_e993_q_d_n1, eq60_e993_q_d_n2, eq60_e993_q_d_n3, eq60_e993_q_d_n4, eq60_e993_q_d_n5, eq60_e993_q_d_n6, eq60_e993_q_d_n7, eq60_e993_q_d_n8, eq60_e993_q_d_n9, eq60_e993_q_d_n10, eq60_e993_q_d_n11, eq60_e993_q_d_n12, eq60_e993_q_d_n13, eq60_e993_q_d_n14, eq60_e993_q_d_n15, eq60_e993_q_d_n16, eq60_e993_q_d_n17, eq60_e993_q_d_n18, eq60_e993_q_d_n19, eq60_e993_q_d_n20, eq60_e993_q_d_n21, eq60_e993_q_d_n22, eq60_e993_q_d_n23, eq60_e993_q_d_n24, eq60_e993_q_d_n25, eq60_e993_q_d_n26, eq60_e993_q_d_n27, eq60_e993_q_d_n28, eq60_e993_q_d_n29,) = {
    if (s.v[760] != 0.0) {
        let eq60_e986_q: f64 = s.v[198];
        let eq60_e989: f64 = (p.p355 * (nv7 - nv15));
        let eq60_e989_d_n7: f64 = p.p355;
        let eq60_e989_d_n15: f64 = (-p.p355);
        let eq60_e990_q: f64 = eq60_e989;
        let eq60_e991: f64 = (s.v[198] + eq60_e989);
        let eq60_e991_d_n7: f64 = (s.dn[198][7] + eq60_e989_d_n7);
        let eq60_e991_d_n15: f64 = (s.dn[198][15] + eq60_e989_d_n15);
        let eq60_e991_q: f64 = (eq60_e986_q + eq60_e990_q);
        let eq60_e991_q_d_n7: f64 = (s.dn[198][7] + eq60_e989_d_n7);
        let eq60_e991_q_d_n15: f64 = (s.dn[198][15] + eq60_e989_d_n15);
        (eq60_e991, s.dn[198][0], s.dn[198][1], s.dn[198][2], s.dn[198][3], s.dn[198][4], s.dn[198][5], s.dn[198][6], eq60_e991_d_n7, s.dn[198][8], s.dn[198][9], s.dn[198][10], s.dn[198][11], s.dn[198][12], s.dn[198][13], s.dn[198][14], eq60_e991_d_n15, s.dn[198][16], s.dn[198][17], s.dn[198][18], s.dn[198][19], s.dn[198][20], s.dn[198][21], s.dn[198][22], s.dn[198][23], s.dn[198][24], s.dn[198][25], s.dn[198][26], s.dn[198][27], s.dn[198][28], s.dn[198][29], eq60_e991_q, s.dn[198][0], s.dn[198][1], s.dn[198][2], s.dn[198][3], s.dn[198][4], s.dn[198][5], s.dn[198][6], eq60_e991_q_d_n7, s.dn[198][8], s.dn[198][9], s.dn[198][10], s.dn[198][11], s.dn[198][12], s.dn[198][13], s.dn[198][14], eq60_e991_q_d_n15, s.dn[198][16], s.dn[198][17], s.dn[198][18], s.dn[198][19], s.dn[198][20], s.dn[198][21], s.dn[198][22], s.dn[198][23], s.dn[198][24], s.dn[198][25], s.dn[198][26], s.dn[198][27], s.dn[198][28], s.dn[198][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_reactive_node_derivatives: [f64; 30] = [eq60_e993_q_d_n0, eq60_e993_q_d_n1, eq60_e993_q_d_n2, eq60_e993_q_d_n3, eq60_e993_q_d_n4, eq60_e993_q_d_n5, eq60_e993_q_d_n6, eq60_e993_q_d_n7, eq60_e993_q_d_n8, eq60_e993_q_d_n9, eq60_e993_q_d_n10, eq60_e993_q_d_n11, eq60_e993_q_d_n12, eq60_e993_q_d_n13, eq60_e993_q_d_n14, eq60_e993_q_d_n15, eq60_e993_q_d_n16, eq60_e993_q_d_n17, eq60_e993_q_d_n18, eq60_e993_q_d_n19, eq60_e993_q_d_n20, eq60_e993_q_d_n21, eq60_e993_q_d_n22, eq60_e993_q_d_n23, eq60_e993_q_d_n24, eq60_e993_q_d_n25, eq60_e993_q_d_n26, eq60_e993_q_d_n27, eq60_e993_q_d_n28, eq60_e993_q_d_n29];
        let eq60_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[15]),
            &nodes,
            &eq60_reactive_node_derivatives,
            &branches,
            &eq60_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_61_block_0(
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
        let (eq61_e1003, eq61_e1003_d_n0, eq61_e1003_d_n1, eq61_e1003_d_n2, eq61_e1003_d_n3, eq61_e1003_d_n4, eq61_e1003_d_n5, eq61_e1003_d_n6, eq61_e1003_d_n7, eq61_e1003_d_n8, eq61_e1003_d_n9, eq61_e1003_d_n10, eq61_e1003_d_n11, eq61_e1003_d_n12, eq61_e1003_d_n13, eq61_e1003_d_n14, eq61_e1003_d_n15, eq61_e1003_d_n16, eq61_e1003_d_n17, eq61_e1003_d_n18, eq61_e1003_d_n19, eq61_e1003_d_n20, eq61_e1003_d_n21, eq61_e1003_d_n22, eq61_e1003_d_n23, eq61_e1003_d_n24, eq61_e1003_d_n25, eq61_e1003_d_n26, eq61_e1003_d_n27, eq61_e1003_d_n28, eq61_e1003_d_n29, eq61_e1003_q, eq61_e1003_q_d_n0, eq61_e1003_q_d_n1, eq61_e1003_q_d_n2, eq61_e1003_q_d_n3, eq61_e1003_q_d_n4, eq61_e1003_q_d_n5, eq61_e1003_q_d_n6, eq61_e1003_q_d_n7, eq61_e1003_q_d_n8, eq61_e1003_q_d_n9, eq61_e1003_q_d_n10, eq61_e1003_q_d_n11, eq61_e1003_q_d_n12, eq61_e1003_q_d_n13, eq61_e1003_q_d_n14, eq61_e1003_q_d_n15, eq61_e1003_q_d_n16, eq61_e1003_q_d_n17, eq61_e1003_q_d_n18, eq61_e1003_q_d_n19, eq61_e1003_q_d_n20, eq61_e1003_q_d_n21, eq61_e1003_q_d_n22, eq61_e1003_q_d_n23, eq61_e1003_q_d_n24, eq61_e1003_q_d_n25, eq61_e1003_q_d_n26, eq61_e1003_q_d_n27, eq61_e1003_q_d_n28, eq61_e1003_q_d_n29,) = {
    if (s.v[760] != 0.0) {
        let eq61_e996_q: f64 = s.v[199];
        let eq61_e999: f64 = (p.p355 * (nv2 - nv14));
        let eq61_e999_d_n2: f64 = p.p355;
        let eq61_e999_d_n14: f64 = (-p.p355);
        let eq61_e1000_q: f64 = eq61_e999;
        let eq61_e1001: f64 = (s.v[199] + eq61_e999);
        let eq61_e1001_d_n2: f64 = (s.dn[199][2] + eq61_e999_d_n2);
        let eq61_e1001_d_n14: f64 = (s.dn[199][14] + eq61_e999_d_n14);
        let eq61_e1001_q: f64 = (eq61_e996_q + eq61_e1000_q);
        let eq61_e1001_q_d_n2: f64 = (s.dn[199][2] + eq61_e999_d_n2);
        let eq61_e1001_q_d_n14: f64 = (s.dn[199][14] + eq61_e999_d_n14);
        (eq61_e1001, s.dn[199][0], s.dn[199][1], eq61_e1001_d_n2, s.dn[199][3], s.dn[199][4], s.dn[199][5], s.dn[199][6], s.dn[199][7], s.dn[199][8], s.dn[199][9], s.dn[199][10], s.dn[199][11], s.dn[199][12], s.dn[199][13], eq61_e1001_d_n14, s.dn[199][15], s.dn[199][16], s.dn[199][17], s.dn[199][18], s.dn[199][19], s.dn[199][20], s.dn[199][21], s.dn[199][22], s.dn[199][23], s.dn[199][24], s.dn[199][25], s.dn[199][26], s.dn[199][27], s.dn[199][28], s.dn[199][29], eq61_e1001_q, s.dn[199][0], s.dn[199][1], eq61_e1001_q_d_n2, s.dn[199][3], s.dn[199][4], s.dn[199][5], s.dn[199][6], s.dn[199][7], s.dn[199][8], s.dn[199][9], s.dn[199][10], s.dn[199][11], s.dn[199][12], s.dn[199][13], eq61_e1001_q_d_n14, s.dn[199][15], s.dn[199][16], s.dn[199][17], s.dn[199][18], s.dn[199][19], s.dn[199][20], s.dn[199][21], s.dn[199][22], s.dn[199][23], s.dn[199][24], s.dn[199][25], s.dn[199][26], s.dn[199][27], s.dn[199][28], s.dn[199][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_reactive_node_derivatives: [f64; 30] = [eq61_e1003_q_d_n0, eq61_e1003_q_d_n1, eq61_e1003_q_d_n2, eq61_e1003_q_d_n3, eq61_e1003_q_d_n4, eq61_e1003_q_d_n5, eq61_e1003_q_d_n6, eq61_e1003_q_d_n7, eq61_e1003_q_d_n8, eq61_e1003_q_d_n9, eq61_e1003_q_d_n10, eq61_e1003_q_d_n11, eq61_e1003_q_d_n12, eq61_e1003_q_d_n13, eq61_e1003_q_d_n14, eq61_e1003_q_d_n15, eq61_e1003_q_d_n16, eq61_e1003_q_d_n17, eq61_e1003_q_d_n18, eq61_e1003_q_d_n19, eq61_e1003_q_d_n20, eq61_e1003_q_d_n21, eq61_e1003_q_d_n22, eq61_e1003_q_d_n23, eq61_e1003_q_d_n24, eq61_e1003_q_d_n25, eq61_e1003_q_d_n26, eq61_e1003_q_d_n27, eq61_e1003_q_d_n28, eq61_e1003_q_d_n29];
        let eq61_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[14]),
            &nodes,
            &eq61_reactive_node_derivatives,
            &branches,
            &eq61_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_63_block_0(
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
        let (eq63_e1017, eq63_e1017_d_n0, eq63_e1017_d_n1, eq63_e1017_d_n2, eq63_e1017_d_n3, eq63_e1017_d_n4, eq63_e1017_d_n5, eq63_e1017_d_n6, eq63_e1017_d_n7, eq63_e1017_d_n8, eq63_e1017_d_n9, eq63_e1017_d_n10, eq63_e1017_d_n11, eq63_e1017_d_n12, eq63_e1017_d_n13, eq63_e1017_d_n14, eq63_e1017_d_n15, eq63_e1017_d_n16, eq63_e1017_d_n17, eq63_e1017_d_n18, eq63_e1017_d_n19, eq63_e1017_d_n20, eq63_e1017_d_n21, eq63_e1017_d_n22, eq63_e1017_d_n23, eq63_e1017_d_n24, eq63_e1017_d_n25, eq63_e1017_d_n26, eq63_e1017_d_n27, eq63_e1017_d_n28, eq63_e1017_d_n29, eq63_e1017_q, eq63_e1017_q_d_n0, eq63_e1017_q_d_n1, eq63_e1017_q_d_n2, eq63_e1017_q_d_n3, eq63_e1017_q_d_n4, eq63_e1017_q_d_n5, eq63_e1017_q_d_n6, eq63_e1017_q_d_n7, eq63_e1017_q_d_n8, eq63_e1017_q_d_n9, eq63_e1017_q_d_n10, eq63_e1017_q_d_n11, eq63_e1017_q_d_n12, eq63_e1017_q_d_n13, eq63_e1017_q_d_n14, eq63_e1017_q_d_n15, eq63_e1017_q_d_n16, eq63_e1017_q_d_n17, eq63_e1017_q_d_n18, eq63_e1017_q_d_n19, eq63_e1017_q_d_n20, eq63_e1017_q_d_n21, eq63_e1017_q_d_n22, eq63_e1017_q_d_n23, eq63_e1017_q_d_n24, eq63_e1017_q_d_n25, eq63_e1017_q_d_n26, eq63_e1017_q_d_n27, eq63_e1017_q_d_n28, eq63_e1017_q_d_n29,) = {
    if (s.v[760] != 0.0) {
        let eq63_e1010_q: f64 = s.v[201];
        let eq63_e1013: f64 = (p.p355 * (nv7 - nv9));
        let eq63_e1013_d_n7: f64 = p.p355;
        let eq63_e1013_d_n9: f64 = (-p.p355);
        let eq63_e1014_q: f64 = eq63_e1013;
        let eq63_e1015: f64 = (s.v[201] + eq63_e1013);
        let eq63_e1015_d_n7: f64 = (s.dn[201][7] + eq63_e1013_d_n7);
        let eq63_e1015_d_n9: f64 = (s.dn[201][9] + eq63_e1013_d_n9);
        let eq63_e1015_q: f64 = (eq63_e1010_q + eq63_e1014_q);
        let eq63_e1015_q_d_n7: f64 = (s.dn[201][7] + eq63_e1013_d_n7);
        let eq63_e1015_q_d_n9: f64 = (s.dn[201][9] + eq63_e1013_d_n9);
        (eq63_e1015, s.dn[201][0], s.dn[201][1], s.dn[201][2], s.dn[201][3], s.dn[201][4], s.dn[201][5], s.dn[201][6], eq63_e1015_d_n7, s.dn[201][8], eq63_e1015_d_n9, s.dn[201][10], s.dn[201][11], s.dn[201][12], s.dn[201][13], s.dn[201][14], s.dn[201][15], s.dn[201][16], s.dn[201][17], s.dn[201][18], s.dn[201][19], s.dn[201][20], s.dn[201][21], s.dn[201][22], s.dn[201][23], s.dn[201][24], s.dn[201][25], s.dn[201][26], s.dn[201][27], s.dn[201][28], s.dn[201][29], eq63_e1015_q, s.dn[201][0], s.dn[201][1], s.dn[201][2], s.dn[201][3], s.dn[201][4], s.dn[201][5], s.dn[201][6], eq63_e1015_q_d_n7, s.dn[201][8], eq63_e1015_q_d_n9, s.dn[201][10], s.dn[201][11], s.dn[201][12], s.dn[201][13], s.dn[201][14], s.dn[201][15], s.dn[201][16], s.dn[201][17], s.dn[201][18], s.dn[201][19], s.dn[201][20], s.dn[201][21], s.dn[201][22], s.dn[201][23], s.dn[201][24], s.dn[201][25], s.dn[201][26], s.dn[201][27], s.dn[201][28], s.dn[201][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_reactive_node_derivatives: [f64; 30] = [eq63_e1017_q_d_n0, eq63_e1017_q_d_n1, eq63_e1017_q_d_n2, eq63_e1017_q_d_n3, eq63_e1017_q_d_n4, eq63_e1017_q_d_n5, eq63_e1017_q_d_n6, eq63_e1017_q_d_n7, eq63_e1017_q_d_n8, eq63_e1017_q_d_n9, eq63_e1017_q_d_n10, eq63_e1017_q_d_n11, eq63_e1017_q_d_n12, eq63_e1017_q_d_n13, eq63_e1017_q_d_n14, eq63_e1017_q_d_n15, eq63_e1017_q_d_n16, eq63_e1017_q_d_n17, eq63_e1017_q_d_n18, eq63_e1017_q_d_n19, eq63_e1017_q_d_n20, eq63_e1017_q_d_n21, eq63_e1017_q_d_n22, eq63_e1017_q_d_n23, eq63_e1017_q_d_n24, eq63_e1017_q_d_n25, eq63_e1017_q_d_n26, eq63_e1017_q_d_n27, eq63_e1017_q_d_n28, eq63_e1017_q_d_n29];
        let eq63_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            &nodes,
            &eq63_reactive_node_derivatives,
            &branches,
            &eq63_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_64_block_0(
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
        let (eq64_e1028, eq64_e1028_d_n0, eq64_e1028_d_n1, eq64_e1028_d_n2, eq64_e1028_d_n3, eq64_e1028_d_n4, eq64_e1028_d_n5, eq64_e1028_d_n6, eq64_e1028_d_n7, eq64_e1028_d_n8, eq64_e1028_d_n9, eq64_e1028_d_n10, eq64_e1028_d_n11, eq64_e1028_d_n12, eq64_e1028_d_n13, eq64_e1028_d_n14, eq64_e1028_d_n15, eq64_e1028_d_n16, eq64_e1028_d_n17, eq64_e1028_d_n18, eq64_e1028_d_n19, eq64_e1028_d_n20, eq64_e1028_d_n21, eq64_e1028_d_n22, eq64_e1028_d_n23, eq64_e1028_d_n24, eq64_e1028_d_n25, eq64_e1028_d_n26, eq64_e1028_d_n27, eq64_e1028_d_n28, eq64_e1028_d_n29, eq64_e1028_q, eq64_e1028_q_d_n0, eq64_e1028_q_d_n1, eq64_e1028_q_d_n2, eq64_e1028_q_d_n3, eq64_e1028_q_d_n4, eq64_e1028_q_d_n5, eq64_e1028_q_d_n6, eq64_e1028_q_d_n7, eq64_e1028_q_d_n8, eq64_e1028_q_d_n9, eq64_e1028_q_d_n10, eq64_e1028_q_d_n11, eq64_e1028_q_d_n12, eq64_e1028_q_d_n13, eq64_e1028_q_d_n14, eq64_e1028_q_d_n15, eq64_e1028_q_d_n16, eq64_e1028_q_d_n17, eq64_e1028_q_d_n18, eq64_e1028_q_d_n19, eq64_e1028_q_d_n20, eq64_e1028_q_d_n21, eq64_e1028_q_d_n22, eq64_e1028_q_d_n23, eq64_e1028_q_d_n24, eq64_e1028_q_d_n25, eq64_e1028_q_d_n26, eq64_e1028_q_d_n27, eq64_e1028_q_d_n28, eq64_e1028_q_d_n29,) = {
    if (!(s.v[760] != 0.0)) {
        let eq64_e1021_q: f64 = s.v[197];
        let eq64_e1024: f64 = (p.p355 * (nv2 - nv14));
        let eq64_e1024_d_n2: f64 = p.p355;
        let eq64_e1024_d_n14: f64 = (-p.p355);
        let eq64_e1025_q: f64 = eq64_e1024;
        let eq64_e1026: f64 = (s.v[197] + eq64_e1024);
        let eq64_e1026_d_n2: f64 = (s.dn[197][2] + eq64_e1024_d_n2);
        let eq64_e1026_d_n14: f64 = (s.dn[197][14] + eq64_e1024_d_n14);
        let eq64_e1026_q: f64 = (eq64_e1021_q + eq64_e1025_q);
        let eq64_e1026_q_d_n2: f64 = (s.dn[197][2] + eq64_e1024_d_n2);
        let eq64_e1026_q_d_n14: f64 = (s.dn[197][14] + eq64_e1024_d_n14);
        (eq64_e1026, s.dn[197][0], s.dn[197][1], eq64_e1026_d_n2, s.dn[197][3], s.dn[197][4], s.dn[197][5], s.dn[197][6], s.dn[197][7], s.dn[197][8], s.dn[197][9], s.dn[197][10], s.dn[197][11], s.dn[197][12], s.dn[197][13], eq64_e1026_d_n14, s.dn[197][15], s.dn[197][16], s.dn[197][17], s.dn[197][18], s.dn[197][19], s.dn[197][20], s.dn[197][21], s.dn[197][22], s.dn[197][23], s.dn[197][24], s.dn[197][25], s.dn[197][26], s.dn[197][27], s.dn[197][28], s.dn[197][29], eq64_e1026_q, s.dn[197][0], s.dn[197][1], eq64_e1026_q_d_n2, s.dn[197][3], s.dn[197][4], s.dn[197][5], s.dn[197][6], s.dn[197][7], s.dn[197][8], s.dn[197][9], s.dn[197][10], s.dn[197][11], s.dn[197][12], s.dn[197][13], eq64_e1026_q_d_n14, s.dn[197][15], s.dn[197][16], s.dn[197][17], s.dn[197][18], s.dn[197][19], s.dn[197][20], s.dn[197][21], s.dn[197][22], s.dn[197][23], s.dn[197][24], s.dn[197][25], s.dn[197][26], s.dn[197][27], s.dn[197][28], s.dn[197][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq64_reactive_node_derivatives: [f64; 30] = [eq64_e1028_q_d_n0, eq64_e1028_q_d_n1, eq64_e1028_q_d_n2, eq64_e1028_q_d_n3, eq64_e1028_q_d_n4, eq64_e1028_q_d_n5, eq64_e1028_q_d_n6, eq64_e1028_q_d_n7, eq64_e1028_q_d_n8, eq64_e1028_q_d_n9, eq64_e1028_q_d_n10, eq64_e1028_q_d_n11, eq64_e1028_q_d_n12, eq64_e1028_q_d_n13, eq64_e1028_q_d_n14, eq64_e1028_q_d_n15, eq64_e1028_q_d_n16, eq64_e1028_q_d_n17, eq64_e1028_q_d_n18, eq64_e1028_q_d_n19, eq64_e1028_q_d_n20, eq64_e1028_q_d_n21, eq64_e1028_q_d_n22, eq64_e1028_q_d_n23, eq64_e1028_q_d_n24, eq64_e1028_q_d_n25, eq64_e1028_q_d_n26, eq64_e1028_q_d_n27, eq64_e1028_q_d_n28, eq64_e1028_q_d_n29];
        let eq64_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[14]),
            &nodes,
            &eq64_reactive_node_derivatives,
            &branches,
            &eq64_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_65_block_0(
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq65_e1039, eq65_e1039_d_n0, eq65_e1039_d_n1, eq65_e1039_d_n2, eq65_e1039_d_n3, eq65_e1039_d_n4, eq65_e1039_d_n5, eq65_e1039_d_n6, eq65_e1039_d_n7, eq65_e1039_d_n8, eq65_e1039_d_n9, eq65_e1039_d_n10, eq65_e1039_d_n11, eq65_e1039_d_n12, eq65_e1039_d_n13, eq65_e1039_d_n14, eq65_e1039_d_n15, eq65_e1039_d_n16, eq65_e1039_d_n17, eq65_e1039_d_n18, eq65_e1039_d_n19, eq65_e1039_d_n20, eq65_e1039_d_n21, eq65_e1039_d_n22, eq65_e1039_d_n23, eq65_e1039_d_n24, eq65_e1039_d_n25, eq65_e1039_d_n26, eq65_e1039_d_n27, eq65_e1039_d_n28, eq65_e1039_d_n29, eq65_e1039_q, eq65_e1039_q_d_n0, eq65_e1039_q_d_n1, eq65_e1039_q_d_n2, eq65_e1039_q_d_n3, eq65_e1039_q_d_n4, eq65_e1039_q_d_n5, eq65_e1039_q_d_n6, eq65_e1039_q_d_n7, eq65_e1039_q_d_n8, eq65_e1039_q_d_n9, eq65_e1039_q_d_n10, eq65_e1039_q_d_n11, eq65_e1039_q_d_n12, eq65_e1039_q_d_n13, eq65_e1039_q_d_n14, eq65_e1039_q_d_n15, eq65_e1039_q_d_n16, eq65_e1039_q_d_n17, eq65_e1039_q_d_n18, eq65_e1039_q_d_n19, eq65_e1039_q_d_n20, eq65_e1039_q_d_n21, eq65_e1039_q_d_n22, eq65_e1039_q_d_n23, eq65_e1039_q_d_n24, eq65_e1039_q_d_n25, eq65_e1039_q_d_n26, eq65_e1039_q_d_n27, eq65_e1039_q_d_n28, eq65_e1039_q_d_n29,) = {
    if (!(s.v[760] != 0.0)) {
        let eq65_e1032_q: f64 = s.v[198];
        let eq65_e1035: f64 = (p.p355 * (nv2 - nv15));
        let eq65_e1035_d_n2: f64 = p.p355;
        let eq65_e1035_d_n15: f64 = (-p.p355);
        let eq65_e1036_q: f64 = eq65_e1035;
        let eq65_e1037: f64 = (s.v[198] + eq65_e1035);
        let eq65_e1037_d_n2: f64 = (s.dn[198][2] + eq65_e1035_d_n2);
        let eq65_e1037_d_n15: f64 = (s.dn[198][15] + eq65_e1035_d_n15);
        let eq65_e1037_q: f64 = (eq65_e1032_q + eq65_e1036_q);
        let eq65_e1037_q_d_n2: f64 = (s.dn[198][2] + eq65_e1035_d_n2);
        let eq65_e1037_q_d_n15: f64 = (s.dn[198][15] + eq65_e1035_d_n15);
        (eq65_e1037, s.dn[198][0], s.dn[198][1], eq65_e1037_d_n2, s.dn[198][3], s.dn[198][4], s.dn[198][5], s.dn[198][6], s.dn[198][7], s.dn[198][8], s.dn[198][9], s.dn[198][10], s.dn[198][11], s.dn[198][12], s.dn[198][13], s.dn[198][14], eq65_e1037_d_n15, s.dn[198][16], s.dn[198][17], s.dn[198][18], s.dn[198][19], s.dn[198][20], s.dn[198][21], s.dn[198][22], s.dn[198][23], s.dn[198][24], s.dn[198][25], s.dn[198][26], s.dn[198][27], s.dn[198][28], s.dn[198][29], eq65_e1037_q, s.dn[198][0], s.dn[198][1], eq65_e1037_q_d_n2, s.dn[198][3], s.dn[198][4], s.dn[198][5], s.dn[198][6], s.dn[198][7], s.dn[198][8], s.dn[198][9], s.dn[198][10], s.dn[198][11], s.dn[198][12], s.dn[198][13], s.dn[198][14], eq65_e1037_q_d_n15, s.dn[198][16], s.dn[198][17], s.dn[198][18], s.dn[198][19], s.dn[198][20], s.dn[198][21], s.dn[198][22], s.dn[198][23], s.dn[198][24], s.dn[198][25], s.dn[198][26], s.dn[198][27], s.dn[198][28], s.dn[198][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_reactive_node_derivatives: [f64; 30] = [eq65_e1039_q_d_n0, eq65_e1039_q_d_n1, eq65_e1039_q_d_n2, eq65_e1039_q_d_n3, eq65_e1039_q_d_n4, eq65_e1039_q_d_n5, eq65_e1039_q_d_n6, eq65_e1039_q_d_n7, eq65_e1039_q_d_n8, eq65_e1039_q_d_n9, eq65_e1039_q_d_n10, eq65_e1039_q_d_n11, eq65_e1039_q_d_n12, eq65_e1039_q_d_n13, eq65_e1039_q_d_n14, eq65_e1039_q_d_n15, eq65_e1039_q_d_n16, eq65_e1039_q_d_n17, eq65_e1039_q_d_n18, eq65_e1039_q_d_n19, eq65_e1039_q_d_n20, eq65_e1039_q_d_n21, eq65_e1039_q_d_n22, eq65_e1039_q_d_n23, eq65_e1039_q_d_n24, eq65_e1039_q_d_n25, eq65_e1039_q_d_n26, eq65_e1039_q_d_n27, eq65_e1039_q_d_n28, eq65_e1039_q_d_n29];
        let eq65_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            &nodes,
            &eq65_reactive_node_derivatives,
            &branches,
            &eq65_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_66_block_0(
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
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq66_e1050, eq66_e1050_d_n0, eq66_e1050_d_n1, eq66_e1050_d_n2, eq66_e1050_d_n3, eq66_e1050_d_n4, eq66_e1050_d_n5, eq66_e1050_d_n6, eq66_e1050_d_n7, eq66_e1050_d_n8, eq66_e1050_d_n9, eq66_e1050_d_n10, eq66_e1050_d_n11, eq66_e1050_d_n12, eq66_e1050_d_n13, eq66_e1050_d_n14, eq66_e1050_d_n15, eq66_e1050_d_n16, eq66_e1050_d_n17, eq66_e1050_d_n18, eq66_e1050_d_n19, eq66_e1050_d_n20, eq66_e1050_d_n21, eq66_e1050_d_n22, eq66_e1050_d_n23, eq66_e1050_d_n24, eq66_e1050_d_n25, eq66_e1050_d_n26, eq66_e1050_d_n27, eq66_e1050_d_n28, eq66_e1050_d_n29, eq66_e1050_q, eq66_e1050_q_d_n0, eq66_e1050_q_d_n1, eq66_e1050_q_d_n2, eq66_e1050_q_d_n3, eq66_e1050_q_d_n4, eq66_e1050_q_d_n5, eq66_e1050_q_d_n6, eq66_e1050_q_d_n7, eq66_e1050_q_d_n8, eq66_e1050_q_d_n9, eq66_e1050_q_d_n10, eq66_e1050_q_d_n11, eq66_e1050_q_d_n12, eq66_e1050_q_d_n13, eq66_e1050_q_d_n14, eq66_e1050_q_d_n15, eq66_e1050_q_d_n16, eq66_e1050_q_d_n17, eq66_e1050_q_d_n18, eq66_e1050_q_d_n19, eq66_e1050_q_d_n20, eq66_e1050_q_d_n21, eq66_e1050_q_d_n22, eq66_e1050_q_d_n23, eq66_e1050_q_d_n24, eq66_e1050_q_d_n25, eq66_e1050_q_d_n26, eq66_e1050_q_d_n27, eq66_e1050_q_d_n28, eq66_e1050_q_d_n29,) = {
    if (!(s.v[760] != 0.0)) {
        let eq66_e1043_q: f64 = s.v[199];
        let eq66_e1046: f64 = (p.p355 * (nv7 - nv14));
        let eq66_e1046_d_n7: f64 = p.p355;
        let eq66_e1046_d_n14: f64 = (-p.p355);
        let eq66_e1047_q: f64 = eq66_e1046;
        let eq66_e1048: f64 = (s.v[199] + eq66_e1046);
        let eq66_e1048_d_n7: f64 = (s.dn[199][7] + eq66_e1046_d_n7);
        let eq66_e1048_d_n14: f64 = (s.dn[199][14] + eq66_e1046_d_n14);
        let eq66_e1048_q: f64 = (eq66_e1043_q + eq66_e1047_q);
        let eq66_e1048_q_d_n7: f64 = (s.dn[199][7] + eq66_e1046_d_n7);
        let eq66_e1048_q_d_n14: f64 = (s.dn[199][14] + eq66_e1046_d_n14);
        (eq66_e1048, s.dn[199][0], s.dn[199][1], s.dn[199][2], s.dn[199][3], s.dn[199][4], s.dn[199][5], s.dn[199][6], eq66_e1048_d_n7, s.dn[199][8], s.dn[199][9], s.dn[199][10], s.dn[199][11], s.dn[199][12], s.dn[199][13], eq66_e1048_d_n14, s.dn[199][15], s.dn[199][16], s.dn[199][17], s.dn[199][18], s.dn[199][19], s.dn[199][20], s.dn[199][21], s.dn[199][22], s.dn[199][23], s.dn[199][24], s.dn[199][25], s.dn[199][26], s.dn[199][27], s.dn[199][28], s.dn[199][29], eq66_e1048_q, s.dn[199][0], s.dn[199][1], s.dn[199][2], s.dn[199][3], s.dn[199][4], s.dn[199][5], s.dn[199][6], eq66_e1048_q_d_n7, s.dn[199][8], s.dn[199][9], s.dn[199][10], s.dn[199][11], s.dn[199][12], s.dn[199][13], eq66_e1048_q_d_n14, s.dn[199][15], s.dn[199][16], s.dn[199][17], s.dn[199][18], s.dn[199][19], s.dn[199][20], s.dn[199][21], s.dn[199][22], s.dn[199][23], s.dn[199][24], s.dn[199][25], s.dn[199][26], s.dn[199][27], s.dn[199][28], s.dn[199][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq66_reactive_node_derivatives: [f64; 30] = [eq66_e1050_q_d_n0, eq66_e1050_q_d_n1, eq66_e1050_q_d_n2, eq66_e1050_q_d_n3, eq66_e1050_q_d_n4, eq66_e1050_q_d_n5, eq66_e1050_q_d_n6, eq66_e1050_q_d_n7, eq66_e1050_q_d_n8, eq66_e1050_q_d_n9, eq66_e1050_q_d_n10, eq66_e1050_q_d_n11, eq66_e1050_q_d_n12, eq66_e1050_q_d_n13, eq66_e1050_q_d_n14, eq66_e1050_q_d_n15, eq66_e1050_q_d_n16, eq66_e1050_q_d_n17, eq66_e1050_q_d_n18, eq66_e1050_q_d_n19, eq66_e1050_q_d_n20, eq66_e1050_q_d_n21, eq66_e1050_q_d_n22, eq66_e1050_q_d_n23, eq66_e1050_q_d_n24, eq66_e1050_q_d_n25, eq66_e1050_q_d_n26, eq66_e1050_q_d_n27, eq66_e1050_q_d_n28, eq66_e1050_q_d_n29];
        let eq66_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[14]),
            &nodes,
            &eq66_reactive_node_derivatives,
            &branches,
            &eq66_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_69_block_0(
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
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq69_e1062_q: f64 = s.v[200];
        let eq69_e1065: f64 = (p.p355 * (nv3 - nv14));
        let eq69_e1065_d_n3: f64 = p.p355;
        let eq69_e1065_d_n14: f64 = (-p.p355);
        let eq69_e1066_q: f64 = eq69_e1065;
        let eq69_e1067: f64 = (s.v[200] + eq69_e1065);
        let eq69_e1067_d_n3: f64 = (s.dn[200][3] + eq69_e1065_d_n3);
        let eq69_e1067_d_n14: f64 = (s.dn[200][14] + eq69_e1065_d_n14);
        let eq69_e1067_q: f64 = (eq69_e1062_q + eq69_e1066_q);
        let eq69_e1067_q_d_n3: f64 = (s.dn[200][3] + eq69_e1065_d_n3);
        let eq69_e1067_q_d_n14: f64 = (s.dn[200][14] + eq69_e1065_d_n14);
        let eq69_reactive_node_derivatives: [f64; 30] = [s.dn[200][0], s.dn[200][1], s.dn[200][2], eq69_e1067_q_d_n3, s.dn[200][4], s.dn[200][5], s.dn[200][6], s.dn[200][7], s.dn[200][8], s.dn[200][9], s.dn[200][10], s.dn[200][11], s.dn[200][12], s.dn[200][13], eq69_e1067_q_d_n14, s.dn[200][15], s.dn[200][16], s.dn[200][17], s.dn[200][18], s.dn[200][19], s.dn[200][20], s.dn[200][21], s.dn[200][22], s.dn[200][23], s.dn[200][24], s.dn[200][25], s.dn[200][26], s.dn[200][27], s.dn[200][28], s.dn[200][29]];
        let eq69_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[14]),
            &nodes,
            &eq69_reactive_node_derivatives,
            &branches,
            &eq69_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_72_block_0(
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
        let (eq72_e1090, eq72_e1090_d_n0, eq72_e1090_d_n1, eq72_e1090_d_n2, eq72_e1090_d_n3, eq72_e1090_d_n4, eq72_e1090_d_n5, eq72_e1090_d_n6, eq72_e1090_d_n7, eq72_e1090_d_n8, eq72_e1090_d_n9, eq72_e1090_d_n10, eq72_e1090_d_n11, eq72_e1090_d_n12, eq72_e1090_d_n13, eq72_e1090_d_n14, eq72_e1090_d_n15, eq72_e1090_d_n16, eq72_e1090_d_n17, eq72_e1090_d_n18, eq72_e1090_d_n19, eq72_e1090_d_n20, eq72_e1090_d_n21, eq72_e1090_d_n22, eq72_e1090_d_n23, eq72_e1090_d_n24, eq72_e1090_d_n25, eq72_e1090_d_n26, eq72_e1090_d_n27, eq72_e1090_d_n28, eq72_e1090_d_n29, eq72_e1090_q, eq72_e1090_q_d_n0, eq72_e1090_q_d_n1, eq72_e1090_q_d_n2, eq72_e1090_q_d_n3, eq72_e1090_q_d_n4, eq72_e1090_q_d_n5, eq72_e1090_q_d_n6, eq72_e1090_q_d_n7, eq72_e1090_q_d_n8, eq72_e1090_q_d_n9, eq72_e1090_q_d_n10, eq72_e1090_q_d_n11, eq72_e1090_q_d_n12, eq72_e1090_q_d_n13, eq72_e1090_q_d_n14, eq72_e1090_q_d_n15, eq72_e1090_q_d_n16, eq72_e1090_q_d_n17, eq72_e1090_q_d_n18, eq72_e1090_q_d_n19, eq72_e1090_q_d_n20, eq72_e1090_q_d_n21, eq72_e1090_q_d_n22, eq72_e1090_q_d_n23, eq72_e1090_q_d_n24, eq72_e1090_q_d_n25, eq72_e1090_q_d_n26, eq72_e1090_q_d_n27, eq72_e1090_q_d_n28, eq72_e1090_q_d_n29,) = {
    if (s.v[907] != 0.0) {
        let eq72_e1083_q: f64 = s.v[191];
        let eq72_e1086: f64 = (p.p355 * (nv7 - nv5));
        let eq72_e1086_d_n5: f64 = (-p.p355);
        let eq72_e1086_d_n7: f64 = p.p355;
        let eq72_e1087_q: f64 = eq72_e1086;
        let eq72_e1088: f64 = (s.v[191] + eq72_e1086);
        let eq72_e1088_d_n5: f64 = (s.dn[191][5] + eq72_e1086_d_n5);
        let eq72_e1088_d_n7: f64 = (s.dn[191][7] + eq72_e1086_d_n7);
        let eq72_e1088_q: f64 = (eq72_e1083_q + eq72_e1087_q);
        let eq72_e1088_q_d_n5: f64 = (s.dn[191][5] + eq72_e1086_d_n5);
        let eq72_e1088_q_d_n7: f64 = (s.dn[191][7] + eq72_e1086_d_n7);
        (eq72_e1088, s.dn[191][0], s.dn[191][1], s.dn[191][2], s.dn[191][3], s.dn[191][4], eq72_e1088_d_n5, s.dn[191][6], eq72_e1088_d_n7, s.dn[191][8], s.dn[191][9], s.dn[191][10], s.dn[191][11], s.dn[191][12], s.dn[191][13], s.dn[191][14], s.dn[191][15], s.dn[191][16], s.dn[191][17], s.dn[191][18], s.dn[191][19], s.dn[191][20], s.dn[191][21], s.dn[191][22], s.dn[191][23], s.dn[191][24], s.dn[191][25], s.dn[191][26], s.dn[191][27], s.dn[191][28], s.dn[191][29], eq72_e1088_q, s.dn[191][0], s.dn[191][1], s.dn[191][2], s.dn[191][3], s.dn[191][4], eq72_e1088_q_d_n5, s.dn[191][6], eq72_e1088_q_d_n7, s.dn[191][8], s.dn[191][9], s.dn[191][10], s.dn[191][11], s.dn[191][12], s.dn[191][13], s.dn[191][14], s.dn[191][15], s.dn[191][16], s.dn[191][17], s.dn[191][18], s.dn[191][19], s.dn[191][20], s.dn[191][21], s.dn[191][22], s.dn[191][23], s.dn[191][24], s.dn[191][25], s.dn[191][26], s.dn[191][27], s.dn[191][28], s.dn[191][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_reactive_node_derivatives: [f64; 30] = [eq72_e1090_q_d_n0, eq72_e1090_q_d_n1, eq72_e1090_q_d_n2, eq72_e1090_q_d_n3, eq72_e1090_q_d_n4, eq72_e1090_q_d_n5, eq72_e1090_q_d_n6, eq72_e1090_q_d_n7, eq72_e1090_q_d_n8, eq72_e1090_q_d_n9, eq72_e1090_q_d_n10, eq72_e1090_q_d_n11, eq72_e1090_q_d_n12, eq72_e1090_q_d_n13, eq72_e1090_q_d_n14, eq72_e1090_q_d_n15, eq72_e1090_q_d_n16, eq72_e1090_q_d_n17, eq72_e1090_q_d_n18, eq72_e1090_q_d_n19, eq72_e1090_q_d_n20, eq72_e1090_q_d_n21, eq72_e1090_q_d_n22, eq72_e1090_q_d_n23, eq72_e1090_q_d_n24, eq72_e1090_q_d_n25, eq72_e1090_q_d_n26, eq72_e1090_q_d_n27, eq72_e1090_q_d_n28, eq72_e1090_q_d_n29];
        let eq72_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            &nodes,
            &eq72_reactive_node_derivatives,
            &branches,
            &eq72_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_73_block_0(
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
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq73_e1100, eq73_e1100_d_n0, eq73_e1100_d_n1, eq73_e1100_d_n2, eq73_e1100_d_n3, eq73_e1100_d_n4, eq73_e1100_d_n5, eq73_e1100_d_n6, eq73_e1100_d_n7, eq73_e1100_d_n8, eq73_e1100_d_n9, eq73_e1100_d_n10, eq73_e1100_d_n11, eq73_e1100_d_n12, eq73_e1100_d_n13, eq73_e1100_d_n14, eq73_e1100_d_n15, eq73_e1100_d_n16, eq73_e1100_d_n17, eq73_e1100_d_n18, eq73_e1100_d_n19, eq73_e1100_d_n20, eq73_e1100_d_n21, eq73_e1100_d_n22, eq73_e1100_d_n23, eq73_e1100_d_n24, eq73_e1100_d_n25, eq73_e1100_d_n26, eq73_e1100_d_n27, eq73_e1100_d_n28, eq73_e1100_d_n29, eq73_e1100_q, eq73_e1100_q_d_n0, eq73_e1100_q_d_n1, eq73_e1100_q_d_n2, eq73_e1100_q_d_n3, eq73_e1100_q_d_n4, eq73_e1100_q_d_n5, eq73_e1100_q_d_n6, eq73_e1100_q_d_n7, eq73_e1100_q_d_n8, eq73_e1100_q_d_n9, eq73_e1100_q_d_n10, eq73_e1100_q_d_n11, eq73_e1100_q_d_n12, eq73_e1100_q_d_n13, eq73_e1100_q_d_n14, eq73_e1100_q_d_n15, eq73_e1100_q_d_n16, eq73_e1100_q_d_n17, eq73_e1100_q_d_n18, eq73_e1100_q_d_n19, eq73_e1100_q_d_n20, eq73_e1100_q_d_n21, eq73_e1100_q_d_n22, eq73_e1100_q_d_n23, eq73_e1100_q_d_n24, eq73_e1100_q_d_n25, eq73_e1100_q_d_n26, eq73_e1100_q_d_n27, eq73_e1100_q_d_n28, eq73_e1100_q_d_n29,) = {
    if (s.v[907] != 0.0) {
        let eq73_e1093_q: f64 = s.v[192];
        let eq73_e1096: f64 = (p.p355 * (nv7 - nv14));
        let eq73_e1096_d_n7: f64 = p.p355;
        let eq73_e1096_d_n14: f64 = (-p.p355);
        let eq73_e1097_q: f64 = eq73_e1096;
        let eq73_e1098: f64 = (s.v[192] + eq73_e1096);
        let eq73_e1098_d_n7: f64 = (s.dn[192][7] + eq73_e1096_d_n7);
        let eq73_e1098_d_n14: f64 = (s.dn[192][14] + eq73_e1096_d_n14);
        let eq73_e1098_q: f64 = (eq73_e1093_q + eq73_e1097_q);
        let eq73_e1098_q_d_n7: f64 = (s.dn[192][7] + eq73_e1096_d_n7);
        let eq73_e1098_q_d_n14: f64 = (s.dn[192][14] + eq73_e1096_d_n14);
        (eq73_e1098, s.dn[192][0], s.dn[192][1], s.dn[192][2], s.dn[192][3], s.dn[192][4], s.dn[192][5], s.dn[192][6], eq73_e1098_d_n7, s.dn[192][8], s.dn[192][9], s.dn[192][10], s.dn[192][11], s.dn[192][12], s.dn[192][13], eq73_e1098_d_n14, s.dn[192][15], s.dn[192][16], s.dn[192][17], s.dn[192][18], s.dn[192][19], s.dn[192][20], s.dn[192][21], s.dn[192][22], s.dn[192][23], s.dn[192][24], s.dn[192][25], s.dn[192][26], s.dn[192][27], s.dn[192][28], s.dn[192][29], eq73_e1098_q, s.dn[192][0], s.dn[192][1], s.dn[192][2], s.dn[192][3], s.dn[192][4], s.dn[192][5], s.dn[192][6], eq73_e1098_q_d_n7, s.dn[192][8], s.dn[192][9], s.dn[192][10], s.dn[192][11], s.dn[192][12], s.dn[192][13], eq73_e1098_q_d_n14, s.dn[192][15], s.dn[192][16], s.dn[192][17], s.dn[192][18], s.dn[192][19], s.dn[192][20], s.dn[192][21], s.dn[192][22], s.dn[192][23], s.dn[192][24], s.dn[192][25], s.dn[192][26], s.dn[192][27], s.dn[192][28], s.dn[192][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_reactive_node_derivatives: [f64; 30] = [eq73_e1100_q_d_n0, eq73_e1100_q_d_n1, eq73_e1100_q_d_n2, eq73_e1100_q_d_n3, eq73_e1100_q_d_n4, eq73_e1100_q_d_n5, eq73_e1100_q_d_n6, eq73_e1100_q_d_n7, eq73_e1100_q_d_n8, eq73_e1100_q_d_n9, eq73_e1100_q_d_n10, eq73_e1100_q_d_n11, eq73_e1100_q_d_n12, eq73_e1100_q_d_n13, eq73_e1100_q_d_n14, eq73_e1100_q_d_n15, eq73_e1100_q_d_n16, eq73_e1100_q_d_n17, eq73_e1100_q_d_n18, eq73_e1100_q_d_n19, eq73_e1100_q_d_n20, eq73_e1100_q_d_n21, eq73_e1100_q_d_n22, eq73_e1100_q_d_n23, eq73_e1100_q_d_n24, eq73_e1100_q_d_n25, eq73_e1100_q_d_n26, eq73_e1100_q_d_n27, eq73_e1100_q_d_n28, eq73_e1100_q_d_n29];
        let eq73_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[14]),
            &nodes,
            &eq73_reactive_node_derivatives,
            &branches,
            &eq73_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
