#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq50_e1300, eq50_e1300_d_n0, eq50_e1300_d_n1, eq50_e1300_d_n2, eq50_e1300_d_n3, eq50_e1300_d_n4, eq50_e1300_d_n5, eq50_e1300_d_n6, eq50_e1300_d_n7, eq50_e1300_d_n8, eq50_e1300_d_n9, eq50_e1300_d_n10, eq50_e1300_d_n11, eq50_e1300_d_n12, eq50_e1300_d_n13, eq50_e1300_d_n14, eq50_e1300_d_n15, eq50_e1300_d_n16, eq50_e1300_d_n17, eq50_e1300_d_b0, eq50_e1300_d_b1, eq50_e1300_d_b2, eq50_e1300_d_b3, eq50_e1300_d_b4, eq50_e1300_d_b5, eq50_e1300_d_b6, eq50_e1300_d_b7, eq50_e1300_d_b8,) = {
    if (p.p52 != 0.0) {
        let eq50_e1298: f64 = (s.v[657] * (nv9 - nv8));
        let eq50_e1298_d_n0: f64 = (s.dn[657][0] * (nv9 - nv8));
        let eq50_e1298_d_n1: f64 = (s.dn[657][1] * (nv9 - nv8));
        let eq50_e1298_d_n2: f64 = (s.dn[657][2] * (nv9 - nv8));
        let eq50_e1298_d_n3: f64 = (s.dn[657][3] * (nv9 - nv8));
        let eq50_e1298_d_n4: f64 = (s.dn[657][4] * (nv9 - nv8));
        let eq50_e1298_d_n5: f64 = (s.dn[657][5] * (nv9 - nv8));
        let eq50_e1298_d_n6: f64 = (s.dn[657][6] * (nv9 - nv8));
        let eq50_e1298_d_n7: f64 = (s.dn[657][7] * (nv9 - nv8));
        let eq50_e1298_d_n8: f64 = ((s.dn[657][8] * (nv9 - nv8)) + (-s.v[657]));
        let eq50_e1298_d_n9: f64 = ((s.dn[657][9] * (nv9 - nv8)) + s.v[657]);
        let eq50_e1298_d_n10: f64 = (s.dn[657][10] * (nv9 - nv8));
        let eq50_e1298_d_n11: f64 = (s.dn[657][11] * (nv9 - nv8));
        let eq50_e1298_d_n12: f64 = (s.dn[657][12] * (nv9 - nv8));
        let eq50_e1298_d_n13: f64 = (s.dn[657][13] * (nv9 - nv8));
        let eq50_e1298_d_n14: f64 = (s.dn[657][14] * (nv9 - nv8));
        let eq50_e1298_d_n15: f64 = (s.dn[657][15] * (nv9 - nv8));
        let eq50_e1298_d_n16: f64 = (s.dn[657][16] * (nv9 - nv8));
        let eq50_e1298_d_n17: f64 = (s.dn[657][17] * (nv9 - nv8));
        let eq50_e1298_d_b0: f64 = (s.db[657][0] * (nv9 - nv8));
        let eq50_e1298_d_b1: f64 = (s.db[657][1] * (nv9 - nv8));
        let eq50_e1298_d_b2: f64 = (s.db[657][2] * (nv9 - nv8));
        let eq50_e1298_d_b3: f64 = (s.db[657][3] * (nv9 - nv8));
        let eq50_e1298_d_b4: f64 = (s.db[657][4] * (nv9 - nv8));
        let eq50_e1298_d_b5: f64 = (s.db[657][5] * (nv9 - nv8));
        let eq50_e1298_d_b6: f64 = (s.db[657][6] * (nv9 - nv8));
        let eq50_e1298_d_b7: f64 = (s.db[657][7] * (nv9 - nv8));
        let eq50_e1298_d_b8: f64 = (s.db[657][8] * (nv9 - nv8));
        (eq50_e1298, eq50_e1298_d_n0, eq50_e1298_d_n1, eq50_e1298_d_n2, eq50_e1298_d_n3, eq50_e1298_d_n4, eq50_e1298_d_n5, eq50_e1298_d_n6, eq50_e1298_d_n7, eq50_e1298_d_n8, eq50_e1298_d_n9, eq50_e1298_d_n10, eq50_e1298_d_n11, eq50_e1298_d_n12, eq50_e1298_d_n13, eq50_e1298_d_n14, eq50_e1298_d_n15, eq50_e1298_d_n16, eq50_e1298_d_n17, eq50_e1298_d_b0, eq50_e1298_d_b1, eq50_e1298_d_b2, eq50_e1298_d_b3, eq50_e1298_d_b4, eq50_e1298_d_b5, eq50_e1298_d_b6, eq50_e1298_d_b7, eq50_e1298_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e1300;
        let eq50_node_derivatives: [f64; 18] = [eq50_e1300_d_n0, eq50_e1300_d_n1, eq50_e1300_d_n2, eq50_e1300_d_n3, eq50_e1300_d_n4, eq50_e1300_d_n5, eq50_e1300_d_n6, eq50_e1300_d_n7, eq50_e1300_d_n8, eq50_e1300_d_n9, eq50_e1300_d_n10, eq50_e1300_d_n11, eq50_e1300_d_n12, eq50_e1300_d_n13, eq50_e1300_d_n14, eq50_e1300_d_n15, eq50_e1300_d_n16, eq50_e1300_d_n17];
        let eq50_branch_derivatives: [f64; 9] = [eq50_e1300_d_b0, eq50_e1300_d_b1, eq50_e1300_d_b2, eq50_e1300_d_b3, eq50_e1300_d_b4, eq50_e1300_d_b5, eq50_e1300_d_b6, eq50_e1300_d_b7, eq50_e1300_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[8]),
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq51_e1306, eq51_e1306_d_n0, eq51_e1306_d_n1, eq51_e1306_d_n2, eq51_e1306_d_n3, eq51_e1306_d_n4, eq51_e1306_d_n5, eq51_e1306_d_n6, eq51_e1306_d_n7, eq51_e1306_d_n8, eq51_e1306_d_n9, eq51_e1306_d_n10, eq51_e1306_d_n11, eq51_e1306_d_n12, eq51_e1306_d_n13, eq51_e1306_d_n14, eq51_e1306_d_n15, eq51_e1306_d_n16, eq51_e1306_d_n17, eq51_e1306_d_b0, eq51_e1306_d_b1, eq51_e1306_d_b2, eq51_e1306_d_b3, eq51_e1306_d_b4, eq51_e1306_d_b5, eq51_e1306_d_b6, eq51_e1306_d_b7, eq51_e1306_d_b8,) = {
    if (p.p52 != 0.0) {
        let eq51_e1304: f64 = (s.v[655] * (nv3 - nv8));
        let eq51_e1304_d_n0: f64 = (s.dn[655][0] * (nv3 - nv8));
        let eq51_e1304_d_n1: f64 = (s.dn[655][1] * (nv3 - nv8));
        let eq51_e1304_d_n2: f64 = (s.dn[655][2] * (nv3 - nv8));
        let eq51_e1304_d_n3: f64 = ((s.dn[655][3] * (nv3 - nv8)) + s.v[655]);
        let eq51_e1304_d_n4: f64 = (s.dn[655][4] * (nv3 - nv8));
        let eq51_e1304_d_n5: f64 = (s.dn[655][5] * (nv3 - nv8));
        let eq51_e1304_d_n6: f64 = (s.dn[655][6] * (nv3 - nv8));
        let eq51_e1304_d_n7: f64 = (s.dn[655][7] * (nv3 - nv8));
        let eq51_e1304_d_n8: f64 = ((s.dn[655][8] * (nv3 - nv8)) + (-s.v[655]));
        let eq51_e1304_d_n9: f64 = (s.dn[655][9] * (nv3 - nv8));
        let eq51_e1304_d_n10: f64 = (s.dn[655][10] * (nv3 - nv8));
        let eq51_e1304_d_n11: f64 = (s.dn[655][11] * (nv3 - nv8));
        let eq51_e1304_d_n12: f64 = (s.dn[655][12] * (nv3 - nv8));
        let eq51_e1304_d_n13: f64 = (s.dn[655][13] * (nv3 - nv8));
        let eq51_e1304_d_n14: f64 = (s.dn[655][14] * (nv3 - nv8));
        let eq51_e1304_d_n15: f64 = (s.dn[655][15] * (nv3 - nv8));
        let eq51_e1304_d_n16: f64 = (s.dn[655][16] * (nv3 - nv8));
        let eq51_e1304_d_n17: f64 = (s.dn[655][17] * (nv3 - nv8));
        let eq51_e1304_d_b0: f64 = (s.db[655][0] * (nv3 - nv8));
        let eq51_e1304_d_b1: f64 = (s.db[655][1] * (nv3 - nv8));
        let eq51_e1304_d_b2: f64 = (s.db[655][2] * (nv3 - nv8));
        let eq51_e1304_d_b3: f64 = (s.db[655][3] * (nv3 - nv8));
        let eq51_e1304_d_b4: f64 = (s.db[655][4] * (nv3 - nv8));
        let eq51_e1304_d_b5: f64 = (s.db[655][5] * (nv3 - nv8));
        let eq51_e1304_d_b6: f64 = (s.db[655][6] * (nv3 - nv8));
        let eq51_e1304_d_b7: f64 = (s.db[655][7] * (nv3 - nv8));
        let eq51_e1304_d_b8: f64 = (s.db[655][8] * (nv3 - nv8));
        (eq51_e1304, eq51_e1304_d_n0, eq51_e1304_d_n1, eq51_e1304_d_n2, eq51_e1304_d_n3, eq51_e1304_d_n4, eq51_e1304_d_n5, eq51_e1304_d_n6, eq51_e1304_d_n7, eq51_e1304_d_n8, eq51_e1304_d_n9, eq51_e1304_d_n10, eq51_e1304_d_n11, eq51_e1304_d_n12, eq51_e1304_d_n13, eq51_e1304_d_n14, eq51_e1304_d_n15, eq51_e1304_d_n16, eq51_e1304_d_n17, eq51_e1304_d_b0, eq51_e1304_d_b1, eq51_e1304_d_b2, eq51_e1304_d_b3, eq51_e1304_d_b4, eq51_e1304_d_b5, eq51_e1304_d_b6, eq51_e1304_d_b7, eq51_e1304_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e1306;
        let eq51_node_derivatives: [f64; 18] = [eq51_e1306_d_n0, eq51_e1306_d_n1, eq51_e1306_d_n2, eq51_e1306_d_n3, eq51_e1306_d_n4, eq51_e1306_d_n5, eq51_e1306_d_n6, eq51_e1306_d_n7, eq51_e1306_d_n8, eq51_e1306_d_n9, eq51_e1306_d_n10, eq51_e1306_d_n11, eq51_e1306_d_n12, eq51_e1306_d_n13, eq51_e1306_d_n14, eq51_e1306_d_n15, eq51_e1306_d_n16, eq51_e1306_d_n17];
        let eq51_branch_derivatives: [f64; 9] = [eq51_e1306_d_b0, eq51_e1306_d_b1, eq51_e1306_d_b2, eq51_e1306_d_b3, eq51_e1306_d_b4, eq51_e1306_d_b5, eq51_e1306_d_b6, eq51_e1306_d_b7, eq51_e1306_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            self.multiplicity * (eq51_value),
            &nodes,
            &eq51_node_derivatives,
            &branches,
            &eq51_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_52_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq52_e1311,) = {
    if (!(p.p52 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq52_value: f64 = eq52_e1311;
        stamper.stamp_potential(
            branches[6],
            eq52_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_53_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq53_e1316,) = {
    if (!(p.p52 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq53_value: f64 = eq53_e1316;
        stamper.stamp_potential(
            branches[7],
            eq53_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_54_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq54_e1321,) = {
    if (!(p.p52 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq54_value: f64 = eq54_e1321;
        stamper.stamp_potential(
            branches[8],
            eq54_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_55_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq55_e1327, eq55_e1327_d_n0, eq55_e1327_d_n1, eq55_e1327_d_n2, eq55_e1327_d_n3, eq55_e1327_d_n4, eq55_e1327_d_n5, eq55_e1327_d_n6, eq55_e1327_d_n7, eq55_e1327_d_n8, eq55_e1327_d_n9, eq55_e1327_d_n10, eq55_e1327_d_n11, eq55_e1327_d_n12, eq55_e1327_d_n13, eq55_e1327_d_n14, eq55_e1327_d_n15, eq55_e1327_d_n16, eq55_e1327_d_n17, eq55_e1327_d_b0, eq55_e1327_d_b1, eq55_e1327_d_b2, eq55_e1327_d_b3, eq55_e1327_d_b4, eq55_e1327_d_b5, eq55_e1327_d_b6, eq55_e1327_d_b7, eq55_e1327_d_b8,) = {
    if (s.v[3409] != 0.0) {
        let eq55_e1325: f64 = ((nv4 - 0.0) * s.v[740]);
        let eq55_e1325_d_n0: f64 = ((nv4 - 0.0) * s.dn[740][0]);
        let eq55_e1325_d_n1: f64 = ((nv4 - 0.0) * s.dn[740][1]);
        let eq55_e1325_d_n2: f64 = ((nv4 - 0.0) * s.dn[740][2]);
        let eq55_e1325_d_n3: f64 = ((nv4 - 0.0) * s.dn[740][3]);
        let eq55_e1325_d_n4: f64 = (s.v[740] + ((nv4 - 0.0) * s.dn[740][4]));
        let eq55_e1325_d_n5: f64 = ((nv4 - 0.0) * s.dn[740][5]);
        let eq55_e1325_d_n6: f64 = ((nv4 - 0.0) * s.dn[740][6]);
        let eq55_e1325_d_n7: f64 = ((nv4 - 0.0) * s.dn[740][7]);
        let eq55_e1325_d_n8: f64 = ((nv4 - 0.0) * s.dn[740][8]);
        let eq55_e1325_d_n9: f64 = ((nv4 - 0.0) * s.dn[740][9]);
        let eq55_e1325_d_n10: f64 = ((nv4 - 0.0) * s.dn[740][10]);
        let eq55_e1325_d_n11: f64 = ((nv4 - 0.0) * s.dn[740][11]);
        let eq55_e1325_d_n12: f64 = ((nv4 - 0.0) * s.dn[740][12]);
        let eq55_e1325_d_n13: f64 = ((nv4 - 0.0) * s.dn[740][13]);
        let eq55_e1325_d_n14: f64 = ((nv4 - 0.0) * s.dn[740][14]);
        let eq55_e1325_d_n15: f64 = ((nv4 - 0.0) * s.dn[740][15]);
        let eq55_e1325_d_n16: f64 = ((nv4 - 0.0) * s.dn[740][16]);
        let eq55_e1325_d_n17: f64 = ((nv4 - 0.0) * s.dn[740][17]);
        let eq55_e1325_d_b0: f64 = ((nv4 - 0.0) * s.db[740][0]);
        let eq55_e1325_d_b1: f64 = ((nv4 - 0.0) * s.db[740][1]);
        let eq55_e1325_d_b2: f64 = ((nv4 - 0.0) * s.db[740][2]);
        let eq55_e1325_d_b3: f64 = ((nv4 - 0.0) * s.db[740][3]);
        let eq55_e1325_d_b4: f64 = ((nv4 - 0.0) * s.db[740][4]);
        let eq55_e1325_d_b5: f64 = ((nv4 - 0.0) * s.db[740][5]);
        let eq55_e1325_d_b6: f64 = ((nv4 - 0.0) * s.db[740][6]);
        let eq55_e1325_d_b7: f64 = ((nv4 - 0.0) * s.db[740][7]);
        let eq55_e1325_d_b8: f64 = ((nv4 - 0.0) * s.db[740][8]);
        (eq55_e1325, eq55_e1325_d_n0, eq55_e1325_d_n1, eq55_e1325_d_n2, eq55_e1325_d_n3, eq55_e1325_d_n4, eq55_e1325_d_n5, eq55_e1325_d_n6, eq55_e1325_d_n7, eq55_e1325_d_n8, eq55_e1325_d_n9, eq55_e1325_d_n10, eq55_e1325_d_n11, eq55_e1325_d_n12, eq55_e1325_d_n13, eq55_e1325_d_n14, eq55_e1325_d_n15, eq55_e1325_d_n16, eq55_e1325_d_n17, eq55_e1325_d_b0, eq55_e1325_d_b1, eq55_e1325_d_b2, eq55_e1325_d_b3, eq55_e1325_d_b4, eq55_e1325_d_b5, eq55_e1325_d_b6, eq55_e1325_d_b7, eq55_e1325_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e1327;
        let eq55_node_derivatives: [f64; 18] = [eq55_e1327_d_n0, eq55_e1327_d_n1, eq55_e1327_d_n2, eq55_e1327_d_n3, eq55_e1327_d_n4, eq55_e1327_d_n5, eq55_e1327_d_n6, eq55_e1327_d_n7, eq55_e1327_d_n8, eq55_e1327_d_n9, eq55_e1327_d_n10, eq55_e1327_d_n11, eq55_e1327_d_n12, eq55_e1327_d_n13, eq55_e1327_d_n14, eq55_e1327_d_n15, eq55_e1327_d_n16, eq55_e1327_d_n17];
        let eq55_branch_derivatives: [f64; 9] = [eq55_e1327_d_b0, eq55_e1327_d_b1, eq55_e1327_d_b2, eq55_e1327_d_b3, eq55_e1327_d_b4, eq55_e1327_d_b5, eq55_e1327_d_b6, eq55_e1327_d_b7, eq55_e1327_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq55_value),
            &nodes,
            &eq55_node_derivatives,
            &branches,
            &eq55_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_56_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq56_e1332, eq56_e1332_d_n0, eq56_e1332_d_n1, eq56_e1332_d_n2, eq56_e1332_d_n3, eq56_e1332_d_n4, eq56_e1332_d_n5, eq56_e1332_d_n6, eq56_e1332_d_n7, eq56_e1332_d_n8, eq56_e1332_d_n9, eq56_e1332_d_n10, eq56_e1332_d_n11, eq56_e1332_d_n12, eq56_e1332_d_n13, eq56_e1332_d_n14, eq56_e1332_d_n15, eq56_e1332_d_n16, eq56_e1332_d_n17, eq56_e1332_d_b0, eq56_e1332_d_b1, eq56_e1332_d_b2, eq56_e1332_d_b3, eq56_e1332_d_b4, eq56_e1332_d_b5, eq56_e1332_d_b6, eq56_e1332_d_b7, eq56_e1332_d_b8,) = {
    if (s.v[3409] != 0.0) {
        let eq56_e1330: f64 = (-s.v[802]);
        let eq56_e1330_d_n0: f64 = (-s.dn[802][0]);
        let eq56_e1330_d_n1: f64 = (-s.dn[802][1]);
        let eq56_e1330_d_n2: f64 = (-s.dn[802][2]);
        let eq56_e1330_d_n3: f64 = (-s.dn[802][3]);
        let eq56_e1330_d_n4: f64 = (-s.dn[802][4]);
        let eq56_e1330_d_n5: f64 = (-s.dn[802][5]);
        let eq56_e1330_d_n6: f64 = (-s.dn[802][6]);
        let eq56_e1330_d_n7: f64 = (-s.dn[802][7]);
        let eq56_e1330_d_n8: f64 = (-s.dn[802][8]);
        let eq56_e1330_d_n9: f64 = (-s.dn[802][9]);
        let eq56_e1330_d_n10: f64 = (-s.dn[802][10]);
        let eq56_e1330_d_n11: f64 = (-s.dn[802][11]);
        let eq56_e1330_d_n12: f64 = (-s.dn[802][12]);
        let eq56_e1330_d_n13: f64 = (-s.dn[802][13]);
        let eq56_e1330_d_n14: f64 = (-s.dn[802][14]);
        let eq56_e1330_d_n15: f64 = (-s.dn[802][15]);
        let eq56_e1330_d_n16: f64 = (-s.dn[802][16]);
        let eq56_e1330_d_n17: f64 = (-s.dn[802][17]);
        let eq56_e1330_d_b0: f64 = (-s.db[802][0]);
        let eq56_e1330_d_b1: f64 = (-s.db[802][1]);
        let eq56_e1330_d_b2: f64 = (-s.db[802][2]);
        let eq56_e1330_d_b3: f64 = (-s.db[802][3]);
        let eq56_e1330_d_b4: f64 = (-s.db[802][4]);
        let eq56_e1330_d_b5: f64 = (-s.db[802][5]);
        let eq56_e1330_d_b6: f64 = (-s.db[802][6]);
        let eq56_e1330_d_b7: f64 = (-s.db[802][7]);
        let eq56_e1330_d_b8: f64 = (-s.db[802][8]);
        (eq56_e1330, eq56_e1330_d_n0, eq56_e1330_d_n1, eq56_e1330_d_n2, eq56_e1330_d_n3, eq56_e1330_d_n4, eq56_e1330_d_n5, eq56_e1330_d_n6, eq56_e1330_d_n7, eq56_e1330_d_n8, eq56_e1330_d_n9, eq56_e1330_d_n10, eq56_e1330_d_n11, eq56_e1330_d_n12, eq56_e1330_d_n13, eq56_e1330_d_n14, eq56_e1330_d_n15, eq56_e1330_d_n16, eq56_e1330_d_n17, eq56_e1330_d_b0, eq56_e1330_d_b1, eq56_e1330_d_b2, eq56_e1330_d_b3, eq56_e1330_d_b4, eq56_e1330_d_b5, eq56_e1330_d_b6, eq56_e1330_d_b7, eq56_e1330_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_value: f64 = eq56_e1332;
        let eq56_node_derivatives: [f64; 18] = [eq56_e1332_d_n0, eq56_e1332_d_n1, eq56_e1332_d_n2, eq56_e1332_d_n3, eq56_e1332_d_n4, eq56_e1332_d_n5, eq56_e1332_d_n6, eq56_e1332_d_n7, eq56_e1332_d_n8, eq56_e1332_d_n9, eq56_e1332_d_n10, eq56_e1332_d_n11, eq56_e1332_d_n12, eq56_e1332_d_n13, eq56_e1332_d_n14, eq56_e1332_d_n15, eq56_e1332_d_n16, eq56_e1332_d_n17];
        let eq56_branch_derivatives: [f64; 9] = [eq56_e1332_d_b0, eq56_e1332_d_b1, eq56_e1332_d_b2, eq56_e1332_d_b3, eq56_e1332_d_b4, eq56_e1332_d_b5, eq56_e1332_d_b6, eq56_e1332_d_b7, eq56_e1332_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq56_value),
            &nodes,
            &eq56_node_derivatives,
            &branches,
            &eq56_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_57_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq57_e1339, eq57_e1339_d_n4,) = {
    if (!(s.v[3409] != 0.0)) {
        let eq57_e1337: f64 = ((nv4 - 0.0) * 10000.0);
        let eq57_e1337_d_n4: f64 = 10000.0;
        (eq57_e1337, eq57_e1337_d_n4,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e1339;
        stamper.stamp_current(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq57_value),
            &[
                GeneratedDerivative::node(nodes[4], self.multiplicity * eq57_e1339_d_n4),
            ],
        );
    }

    pub(super) fn stamp_transient_equation_58_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq58_e1342: f64 = (s.v[767] * (nv4 - 0.0));
        let eq58_e1342_d_n0: f64 = (s.dn[767][0] * (nv4 - 0.0));
        let eq58_e1342_d_n1: f64 = (s.dn[767][1] * (nv4 - 0.0));
        let eq58_e1342_d_n2: f64 = (s.dn[767][2] * (nv4 - 0.0));
        let eq58_e1342_d_n3: f64 = (s.dn[767][3] * (nv4 - 0.0));
        let eq58_e1342_d_n4: f64 = ((s.dn[767][4] * (nv4 - 0.0)) + s.v[767]);
        let eq58_e1342_d_n5: f64 = (s.dn[767][5] * (nv4 - 0.0));
        let eq58_e1342_d_n6: f64 = (s.dn[767][6] * (nv4 - 0.0));
        let eq58_e1342_d_n7: f64 = (s.dn[767][7] * (nv4 - 0.0));
        let eq58_e1342_d_n8: f64 = (s.dn[767][8] * (nv4 - 0.0));
        let eq58_e1342_d_n9: f64 = (s.dn[767][9] * (nv4 - 0.0));
        let eq58_e1342_d_n10: f64 = (s.dn[767][10] * (nv4 - 0.0));
        let eq58_e1342_d_n11: f64 = (s.dn[767][11] * (nv4 - 0.0));
        let eq58_e1342_d_n12: f64 = (s.dn[767][12] * (nv4 - 0.0));
        let eq58_e1342_d_n13: f64 = (s.dn[767][13] * (nv4 - 0.0));
        let eq58_e1342_d_n14: f64 = (s.dn[767][14] * (nv4 - 0.0));
        let eq58_e1342_d_n15: f64 = (s.dn[767][15] * (nv4 - 0.0));
        let eq58_e1342_d_n16: f64 = (s.dn[767][16] * (nv4 - 0.0));
        let eq58_e1342_d_n17: f64 = (s.dn[767][17] * (nv4 - 0.0));
        let eq58_e1342_d_b0: f64 = (s.db[767][0] * (nv4 - 0.0));
        let eq58_e1342_d_b1: f64 = (s.db[767][1] * (nv4 - 0.0));
        let eq58_e1342_d_b2: f64 = (s.db[767][2] * (nv4 - 0.0));
        let eq58_e1342_d_b3: f64 = (s.db[767][3] * (nv4 - 0.0));
        let eq58_e1342_d_b4: f64 = (s.db[767][4] * (nv4 - 0.0));
        let eq58_e1342_d_b5: f64 = (s.db[767][5] * (nv4 - 0.0));
        let eq58_e1342_d_b6: f64 = (s.db[767][6] * (nv4 - 0.0));
        let eq58_e1342_d_b7: f64 = (s.db[767][7] * (nv4 - 0.0));
        let eq58_e1342_d_b8: f64 = (s.db[767][8] * (nv4 - 0.0));
        let eq58_e1343: f64 = self.eval_ddt(17, eq58_e1342);
        let eq58_e1343_d_n0: f64 = self.ddt_jacobian(eq58_e1342_d_n0);
        let eq58_e1343_d_n1: f64 = self.ddt_jacobian(eq58_e1342_d_n1);
        let eq58_e1343_d_n2: f64 = self.ddt_jacobian(eq58_e1342_d_n2);
        let eq58_e1343_d_n3: f64 = self.ddt_jacobian(eq58_e1342_d_n3);
        let eq58_e1343_d_n4: f64 = self.ddt_jacobian(eq58_e1342_d_n4);
        let eq58_e1343_d_n5: f64 = self.ddt_jacobian(eq58_e1342_d_n5);
        let eq58_e1343_d_n6: f64 = self.ddt_jacobian(eq58_e1342_d_n6);
        let eq58_e1343_d_n7: f64 = self.ddt_jacobian(eq58_e1342_d_n7);
        let eq58_e1343_d_n8: f64 = self.ddt_jacobian(eq58_e1342_d_n8);
        let eq58_e1343_d_n9: f64 = self.ddt_jacobian(eq58_e1342_d_n9);
        let eq58_e1343_d_n10: f64 = self.ddt_jacobian(eq58_e1342_d_n10);
        let eq58_e1343_d_n11: f64 = self.ddt_jacobian(eq58_e1342_d_n11);
        let eq58_e1343_d_n12: f64 = self.ddt_jacobian(eq58_e1342_d_n12);
        let eq58_e1343_d_n13: f64 = self.ddt_jacobian(eq58_e1342_d_n13);
        let eq58_e1343_d_n14: f64 = self.ddt_jacobian(eq58_e1342_d_n14);
        let eq58_e1343_d_n15: f64 = self.ddt_jacobian(eq58_e1342_d_n15);
        let eq58_e1343_d_n16: f64 = self.ddt_jacobian(eq58_e1342_d_n16);
        let eq58_e1343_d_n17: f64 = self.ddt_jacobian(eq58_e1342_d_n17);
        let eq58_e1343_d_b0: f64 = self.ddt_jacobian(eq58_e1342_d_b0);
        let eq58_e1343_d_b1: f64 = self.ddt_jacobian(eq58_e1342_d_b1);
        let eq58_e1343_d_b2: f64 = self.ddt_jacobian(eq58_e1342_d_b2);
        let eq58_e1343_d_b3: f64 = self.ddt_jacobian(eq58_e1342_d_b3);
        let eq58_e1343_d_b4: f64 = self.ddt_jacobian(eq58_e1342_d_b4);
        let eq58_e1343_d_b5: f64 = self.ddt_jacobian(eq58_e1342_d_b5);
        let eq58_e1343_d_b6: f64 = self.ddt_jacobian(eq58_e1342_d_b6);
        let eq58_e1343_d_b7: f64 = self.ddt_jacobian(eq58_e1342_d_b7);
        let eq58_e1343_d_b8: f64 = self.ddt_jacobian(eq58_e1342_d_b8);
        let eq58_value: f64 = eq58_e1343;
        let eq58_node_derivatives: [f64; 18] = [eq58_e1343_d_n0, eq58_e1343_d_n1, eq58_e1343_d_n2, eq58_e1343_d_n3, eq58_e1343_d_n4, eq58_e1343_d_n5, eq58_e1343_d_n6, eq58_e1343_d_n7, eq58_e1343_d_n8, eq58_e1343_d_n9, eq58_e1343_d_n10, eq58_e1343_d_n11, eq58_e1343_d_n12, eq58_e1343_d_n13, eq58_e1343_d_n14, eq58_e1343_d_n15, eq58_e1343_d_n16, eq58_e1343_d_n17];
        let eq58_branch_derivatives: [f64; 9] = [eq58_e1343_d_b0, eq58_e1343_d_b1, eq58_e1343_d_b2, eq58_e1343_d_b3, eq58_e1343_d_b4, eq58_e1343_d_b5, eq58_e1343_d_b6, eq58_e1343_d_b7, eq58_e1343_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq58_value),
            &nodes,
            &eq58_node_derivatives,
            &branches,
            &eq58_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_59_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq59_e1347, eq59_e1347_d_n0, eq59_e1347_d_n1, eq59_e1347_d_n2, eq59_e1347_d_n3, eq59_e1347_d_n4, eq59_e1347_d_n5, eq59_e1347_d_n6, eq59_e1347_d_n7, eq59_e1347_d_n8, eq59_e1347_d_n9, eq59_e1347_d_n10, eq59_e1347_d_n11, eq59_e1347_d_n12, eq59_e1347_d_n13, eq59_e1347_d_n14, eq59_e1347_d_n15, eq59_e1347_d_n16, eq59_e1347_d_n17, eq59_e1347_d_b0, eq59_e1347_d_b1, eq59_e1347_d_b2, eq59_e1347_d_b3, eq59_e1347_d_b4, eq59_e1347_d_b5, eq59_e1347_d_b6, eq59_e1347_d_b7, eq59_e1347_d_b8,) = {
    if (p.p28 != 0.0) {
        (s.v[749], s.dn[749][0], s.dn[749][1], s.dn[749][2], s.dn[749][3], s.dn[749][4], s.dn[749][5], s.dn[749][6], s.dn[749][7], s.dn[749][8], s.dn[749][9], s.dn[749][10], s.dn[749][11], s.dn[749][12], s.dn[749][13], s.dn[749][14], s.dn[749][15], s.dn[749][16], s.dn[749][17], s.db[749][0], s.db[749][1], s.db[749][2], s.db[749][3], s.db[749][4], s.db[749][5], s.db[749][6], s.db[749][7], s.db[749][8],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e1347;
        let eq59_node_derivatives: [f64; 18] = [eq59_e1347_d_n0, eq59_e1347_d_n1, eq59_e1347_d_n2, eq59_e1347_d_n3, eq59_e1347_d_n4, eq59_e1347_d_n5, eq59_e1347_d_n6, eq59_e1347_d_n7, eq59_e1347_d_n8, eq59_e1347_d_n9, eq59_e1347_d_n10, eq59_e1347_d_n11, eq59_e1347_d_n12, eq59_e1347_d_n13, eq59_e1347_d_n14, eq59_e1347_d_n15, eq59_e1347_d_n16, eq59_e1347_d_n17];
        let eq59_branch_derivatives: [f64; 9] = [eq59_e1347_d_b0, eq59_e1347_d_b1, eq59_e1347_d_b2, eq59_e1347_d_b3, eq59_e1347_d_b4, eq59_e1347_d_b5, eq59_e1347_d_b6, eq59_e1347_d_b7, eq59_e1347_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            None,
            self.multiplicity * (eq59_value),
            &nodes,
            &eq59_node_derivatives,
            &branches,
            &eq59_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_60_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq60_e1351, eq60_e1351_d_n0, eq60_e1351_d_n1, eq60_e1351_d_n2, eq60_e1351_d_n3, eq60_e1351_d_n4, eq60_e1351_d_n5, eq60_e1351_d_n6, eq60_e1351_d_n7, eq60_e1351_d_n8, eq60_e1351_d_n9, eq60_e1351_d_n10, eq60_e1351_d_n11, eq60_e1351_d_n12, eq60_e1351_d_n13, eq60_e1351_d_n14, eq60_e1351_d_n15, eq60_e1351_d_n16, eq60_e1351_d_n17, eq60_e1351_d_b0, eq60_e1351_d_b1, eq60_e1351_d_b2, eq60_e1351_d_b3, eq60_e1351_d_b4, eq60_e1351_d_b5, eq60_e1351_d_b6, eq60_e1351_d_b7, eq60_e1351_d_b8,) = {
    if (p.p28 != 0.0) {
        (s.v[750], s.dn[750][0], s.dn[750][1], s.dn[750][2], s.dn[750][3], s.dn[750][4], s.dn[750][5], s.dn[750][6], s.dn[750][7], s.dn[750][8], s.dn[750][9], s.dn[750][10], s.dn[750][11], s.dn[750][12], s.dn[750][13], s.dn[750][14], s.dn[750][15], s.dn[750][16], s.dn[750][17], s.db[750][0], s.db[750][1], s.db[750][2], s.db[750][3], s.db[750][4], s.db[750][5], s.db[750][6], s.db[750][7], s.db[750][8],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e1351;
        let eq60_node_derivatives: [f64; 18] = [eq60_e1351_d_n0, eq60_e1351_d_n1, eq60_e1351_d_n2, eq60_e1351_d_n3, eq60_e1351_d_n4, eq60_e1351_d_n5, eq60_e1351_d_n6, eq60_e1351_d_n7, eq60_e1351_d_n8, eq60_e1351_d_n9, eq60_e1351_d_n10, eq60_e1351_d_n11, eq60_e1351_d_n12, eq60_e1351_d_n13, eq60_e1351_d_n14, eq60_e1351_d_n15, eq60_e1351_d_n16, eq60_e1351_d_n17];
        let eq60_branch_derivatives: [f64; 9] = [eq60_e1351_d_b0, eq60_e1351_d_b1, eq60_e1351_d_b2, eq60_e1351_d_b3, eq60_e1351_d_b4, eq60_e1351_d_b5, eq60_e1351_d_b6, eq60_e1351_d_b7, eq60_e1351_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            None,
            self.multiplicity * (eq60_value),
            &nodes,
            &eq60_node_derivatives,
            &branches,
            &eq60_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_61_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq61_e1358, eq61_e1358_d_n0, eq61_e1358_d_n1, eq61_e1358_d_n2, eq61_e1358_d_n3, eq61_e1358_d_n4, eq61_e1358_d_n5, eq61_e1358_d_n6, eq61_e1358_d_n7, eq61_e1358_d_n8, eq61_e1358_d_n9, eq61_e1358_d_n10, eq61_e1358_d_n11, eq61_e1358_d_n12, eq61_e1358_d_n13, eq61_e1358_d_n14, eq61_e1358_d_n15, eq61_e1358_d_n16, eq61_e1358_d_n17, eq61_e1358_d_b0, eq61_e1358_d_b1, eq61_e1358_d_b2, eq61_e1358_d_b3, eq61_e1358_d_b4, eq61_e1358_d_b5, eq61_e1358_d_b6, eq61_e1358_d_b7, eq61_e1358_d_b8,) = {
    if (p.p28 != 0.0) {
        let eq61_e1355: f64 = (s.v[800] * (nv11 - 0.0));
        let eq61_e1355_d_n0: f64 = (s.dn[800][0] * (nv11 - 0.0));
        let eq61_e1355_d_n1: f64 = (s.dn[800][1] * (nv11 - 0.0));
        let eq61_e1355_d_n2: f64 = (s.dn[800][2] * (nv11 - 0.0));
        let eq61_e1355_d_n3: f64 = (s.dn[800][3] * (nv11 - 0.0));
        let eq61_e1355_d_n4: f64 = (s.dn[800][4] * (nv11 - 0.0));
        let eq61_e1355_d_n5: f64 = (s.dn[800][5] * (nv11 - 0.0));
        let eq61_e1355_d_n6: f64 = (s.dn[800][6] * (nv11 - 0.0));
        let eq61_e1355_d_n7: f64 = (s.dn[800][7] * (nv11 - 0.0));
        let eq61_e1355_d_n8: f64 = (s.dn[800][8] * (nv11 - 0.0));
        let eq61_e1355_d_n9: f64 = (s.dn[800][9] * (nv11 - 0.0));
        let eq61_e1355_d_n10: f64 = (s.dn[800][10] * (nv11 - 0.0));
        let eq61_e1355_d_n11: f64 = ((s.dn[800][11] * (nv11 - 0.0)) + s.v[800]);
        let eq61_e1355_d_n12: f64 = (s.dn[800][12] * (nv11 - 0.0));
        let eq61_e1355_d_n13: f64 = (s.dn[800][13] * (nv11 - 0.0));
        let eq61_e1355_d_n14: f64 = (s.dn[800][14] * (nv11 - 0.0));
        let eq61_e1355_d_n15: f64 = (s.dn[800][15] * (nv11 - 0.0));
        let eq61_e1355_d_n16: f64 = (s.dn[800][16] * (nv11 - 0.0));
        let eq61_e1355_d_n17: f64 = (s.dn[800][17] * (nv11 - 0.0));
        let eq61_e1355_d_b0: f64 = (s.db[800][0] * (nv11 - 0.0));
        let eq61_e1355_d_b1: f64 = (s.db[800][1] * (nv11 - 0.0));
        let eq61_e1355_d_b2: f64 = (s.db[800][2] * (nv11 - 0.0));
        let eq61_e1355_d_b3: f64 = (s.db[800][3] * (nv11 - 0.0));
        let eq61_e1355_d_b4: f64 = (s.db[800][4] * (nv11 - 0.0));
        let eq61_e1355_d_b5: f64 = (s.db[800][5] * (nv11 - 0.0));
        let eq61_e1355_d_b6: f64 = (s.db[800][6] * (nv11 - 0.0));
        let eq61_e1355_d_b7: f64 = (s.db[800][7] * (nv11 - 0.0));
        let eq61_e1355_d_b8: f64 = (s.db[800][8] * (nv11 - 0.0));
        let eq61_e1356: f64 = self.eval_ddt(18, eq61_e1355);
        let eq61_e1356_d_n0: f64 = self.ddt_jacobian(eq61_e1355_d_n0);
        let eq61_e1356_d_n1: f64 = self.ddt_jacobian(eq61_e1355_d_n1);
        let eq61_e1356_d_n2: f64 = self.ddt_jacobian(eq61_e1355_d_n2);
        let eq61_e1356_d_n3: f64 = self.ddt_jacobian(eq61_e1355_d_n3);
        let eq61_e1356_d_n4: f64 = self.ddt_jacobian(eq61_e1355_d_n4);
        let eq61_e1356_d_n5: f64 = self.ddt_jacobian(eq61_e1355_d_n5);
        let eq61_e1356_d_n6: f64 = self.ddt_jacobian(eq61_e1355_d_n6);
        let eq61_e1356_d_n7: f64 = self.ddt_jacobian(eq61_e1355_d_n7);
        let eq61_e1356_d_n8: f64 = self.ddt_jacobian(eq61_e1355_d_n8);
        let eq61_e1356_d_n9: f64 = self.ddt_jacobian(eq61_e1355_d_n9);
        let eq61_e1356_d_n10: f64 = self.ddt_jacobian(eq61_e1355_d_n10);
        let eq61_e1356_d_n11: f64 = self.ddt_jacobian(eq61_e1355_d_n11);
        let eq61_e1356_d_n12: f64 = self.ddt_jacobian(eq61_e1355_d_n12);
        let eq61_e1356_d_n13: f64 = self.ddt_jacobian(eq61_e1355_d_n13);
        let eq61_e1356_d_n14: f64 = self.ddt_jacobian(eq61_e1355_d_n14);
        let eq61_e1356_d_n15: f64 = self.ddt_jacobian(eq61_e1355_d_n15);
        let eq61_e1356_d_n16: f64 = self.ddt_jacobian(eq61_e1355_d_n16);
        let eq61_e1356_d_n17: f64 = self.ddt_jacobian(eq61_e1355_d_n17);
        let eq61_e1356_d_b0: f64 = self.ddt_jacobian(eq61_e1355_d_b0);
        let eq61_e1356_d_b1: f64 = self.ddt_jacobian(eq61_e1355_d_b1);
        let eq61_e1356_d_b2: f64 = self.ddt_jacobian(eq61_e1355_d_b2);
        let eq61_e1356_d_b3: f64 = self.ddt_jacobian(eq61_e1355_d_b3);
        let eq61_e1356_d_b4: f64 = self.ddt_jacobian(eq61_e1355_d_b4);
        let eq61_e1356_d_b5: f64 = self.ddt_jacobian(eq61_e1355_d_b5);
        let eq61_e1356_d_b6: f64 = self.ddt_jacobian(eq61_e1355_d_b6);
        let eq61_e1356_d_b7: f64 = self.ddt_jacobian(eq61_e1355_d_b7);
        let eq61_e1356_d_b8: f64 = self.ddt_jacobian(eq61_e1355_d_b8);
        (eq61_e1356, eq61_e1356_d_n0, eq61_e1356_d_n1, eq61_e1356_d_n2, eq61_e1356_d_n3, eq61_e1356_d_n4, eq61_e1356_d_n5, eq61_e1356_d_n6, eq61_e1356_d_n7, eq61_e1356_d_n8, eq61_e1356_d_n9, eq61_e1356_d_n10, eq61_e1356_d_n11, eq61_e1356_d_n12, eq61_e1356_d_n13, eq61_e1356_d_n14, eq61_e1356_d_n15, eq61_e1356_d_n16, eq61_e1356_d_n17, eq61_e1356_d_b0, eq61_e1356_d_b1, eq61_e1356_d_b2, eq61_e1356_d_b3, eq61_e1356_d_b4, eq61_e1356_d_b5, eq61_e1356_d_b6, eq61_e1356_d_b7, eq61_e1356_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_value: f64 = eq61_e1358;
        let eq61_node_derivatives: [f64; 18] = [eq61_e1358_d_n0, eq61_e1358_d_n1, eq61_e1358_d_n2, eq61_e1358_d_n3, eq61_e1358_d_n4, eq61_e1358_d_n5, eq61_e1358_d_n6, eq61_e1358_d_n7, eq61_e1358_d_n8, eq61_e1358_d_n9, eq61_e1358_d_n10, eq61_e1358_d_n11, eq61_e1358_d_n12, eq61_e1358_d_n13, eq61_e1358_d_n14, eq61_e1358_d_n15, eq61_e1358_d_n16, eq61_e1358_d_n17];
        let eq61_branch_derivatives: [f64; 9] = [eq61_e1358_d_b0, eq61_e1358_d_b1, eq61_e1358_d_b2, eq61_e1358_d_b3, eq61_e1358_d_b4, eq61_e1358_d_b5, eq61_e1358_d_b6, eq61_e1358_d_b7, eq61_e1358_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            None,
            self.multiplicity * (eq61_value),
            &nodes,
            &eq61_node_derivatives,
            &branches,
            &eq61_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_62_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq62_e1365, eq62_e1365_d_n0, eq62_e1365_d_n1, eq62_e1365_d_n2, eq62_e1365_d_n3, eq62_e1365_d_n4, eq62_e1365_d_n5, eq62_e1365_d_n6, eq62_e1365_d_n7, eq62_e1365_d_n8, eq62_e1365_d_n9, eq62_e1365_d_n10, eq62_e1365_d_n11, eq62_e1365_d_n12, eq62_e1365_d_n13, eq62_e1365_d_n14, eq62_e1365_d_n15, eq62_e1365_d_n16, eq62_e1365_d_n17, eq62_e1365_d_b0, eq62_e1365_d_b1, eq62_e1365_d_b2, eq62_e1365_d_b3, eq62_e1365_d_b4, eq62_e1365_d_b5, eq62_e1365_d_b6, eq62_e1365_d_b7, eq62_e1365_d_b8,) = {
    if (p.p28 != 0.0) {
        let eq62_e1362: f64 = (s.v[801] * (nv12 - 0.0));
        let eq62_e1362_d_n0: f64 = (s.dn[801][0] * (nv12 - 0.0));
        let eq62_e1362_d_n1: f64 = (s.dn[801][1] * (nv12 - 0.0));
        let eq62_e1362_d_n2: f64 = (s.dn[801][2] * (nv12 - 0.0));
        let eq62_e1362_d_n3: f64 = (s.dn[801][3] * (nv12 - 0.0));
        let eq62_e1362_d_n4: f64 = (s.dn[801][4] * (nv12 - 0.0));
        let eq62_e1362_d_n5: f64 = (s.dn[801][5] * (nv12 - 0.0));
        let eq62_e1362_d_n6: f64 = (s.dn[801][6] * (nv12 - 0.0));
        let eq62_e1362_d_n7: f64 = (s.dn[801][7] * (nv12 - 0.0));
        let eq62_e1362_d_n8: f64 = (s.dn[801][8] * (nv12 - 0.0));
        let eq62_e1362_d_n9: f64 = (s.dn[801][9] * (nv12 - 0.0));
        let eq62_e1362_d_n10: f64 = (s.dn[801][10] * (nv12 - 0.0));
        let eq62_e1362_d_n11: f64 = (s.dn[801][11] * (nv12 - 0.0));
        let eq62_e1362_d_n12: f64 = ((s.dn[801][12] * (nv12 - 0.0)) + s.v[801]);
        let eq62_e1362_d_n13: f64 = (s.dn[801][13] * (nv12 - 0.0));
        let eq62_e1362_d_n14: f64 = (s.dn[801][14] * (nv12 - 0.0));
        let eq62_e1362_d_n15: f64 = (s.dn[801][15] * (nv12 - 0.0));
        let eq62_e1362_d_n16: f64 = (s.dn[801][16] * (nv12 - 0.0));
        let eq62_e1362_d_n17: f64 = (s.dn[801][17] * (nv12 - 0.0));
        let eq62_e1362_d_b0: f64 = (s.db[801][0] * (nv12 - 0.0));
        let eq62_e1362_d_b1: f64 = (s.db[801][1] * (nv12 - 0.0));
        let eq62_e1362_d_b2: f64 = (s.db[801][2] * (nv12 - 0.0));
        let eq62_e1362_d_b3: f64 = (s.db[801][3] * (nv12 - 0.0));
        let eq62_e1362_d_b4: f64 = (s.db[801][4] * (nv12 - 0.0));
        let eq62_e1362_d_b5: f64 = (s.db[801][5] * (nv12 - 0.0));
        let eq62_e1362_d_b6: f64 = (s.db[801][6] * (nv12 - 0.0));
        let eq62_e1362_d_b7: f64 = (s.db[801][7] * (nv12 - 0.0));
        let eq62_e1362_d_b8: f64 = (s.db[801][8] * (nv12 - 0.0));
        let eq62_e1363: f64 = self.eval_ddt(19, eq62_e1362);
        let eq62_e1363_d_n0: f64 = self.ddt_jacobian(eq62_e1362_d_n0);
        let eq62_e1363_d_n1: f64 = self.ddt_jacobian(eq62_e1362_d_n1);
        let eq62_e1363_d_n2: f64 = self.ddt_jacobian(eq62_e1362_d_n2);
        let eq62_e1363_d_n3: f64 = self.ddt_jacobian(eq62_e1362_d_n3);
        let eq62_e1363_d_n4: f64 = self.ddt_jacobian(eq62_e1362_d_n4);
        let eq62_e1363_d_n5: f64 = self.ddt_jacobian(eq62_e1362_d_n5);
        let eq62_e1363_d_n6: f64 = self.ddt_jacobian(eq62_e1362_d_n6);
        let eq62_e1363_d_n7: f64 = self.ddt_jacobian(eq62_e1362_d_n7);
        let eq62_e1363_d_n8: f64 = self.ddt_jacobian(eq62_e1362_d_n8);
        let eq62_e1363_d_n9: f64 = self.ddt_jacobian(eq62_e1362_d_n9);
        let eq62_e1363_d_n10: f64 = self.ddt_jacobian(eq62_e1362_d_n10);
        let eq62_e1363_d_n11: f64 = self.ddt_jacobian(eq62_e1362_d_n11);
        let eq62_e1363_d_n12: f64 = self.ddt_jacobian(eq62_e1362_d_n12);
        let eq62_e1363_d_n13: f64 = self.ddt_jacobian(eq62_e1362_d_n13);
        let eq62_e1363_d_n14: f64 = self.ddt_jacobian(eq62_e1362_d_n14);
        let eq62_e1363_d_n15: f64 = self.ddt_jacobian(eq62_e1362_d_n15);
        let eq62_e1363_d_n16: f64 = self.ddt_jacobian(eq62_e1362_d_n16);
        let eq62_e1363_d_n17: f64 = self.ddt_jacobian(eq62_e1362_d_n17);
        let eq62_e1363_d_b0: f64 = self.ddt_jacobian(eq62_e1362_d_b0);
        let eq62_e1363_d_b1: f64 = self.ddt_jacobian(eq62_e1362_d_b1);
        let eq62_e1363_d_b2: f64 = self.ddt_jacobian(eq62_e1362_d_b2);
        let eq62_e1363_d_b3: f64 = self.ddt_jacobian(eq62_e1362_d_b3);
        let eq62_e1363_d_b4: f64 = self.ddt_jacobian(eq62_e1362_d_b4);
        let eq62_e1363_d_b5: f64 = self.ddt_jacobian(eq62_e1362_d_b5);
        let eq62_e1363_d_b6: f64 = self.ddt_jacobian(eq62_e1362_d_b6);
        let eq62_e1363_d_b7: f64 = self.ddt_jacobian(eq62_e1362_d_b7);
        let eq62_e1363_d_b8: f64 = self.ddt_jacobian(eq62_e1362_d_b8);
        (eq62_e1363, eq62_e1363_d_n0, eq62_e1363_d_n1, eq62_e1363_d_n2, eq62_e1363_d_n3, eq62_e1363_d_n4, eq62_e1363_d_n5, eq62_e1363_d_n6, eq62_e1363_d_n7, eq62_e1363_d_n8, eq62_e1363_d_n9, eq62_e1363_d_n10, eq62_e1363_d_n11, eq62_e1363_d_n12, eq62_e1363_d_n13, eq62_e1363_d_n14, eq62_e1363_d_n15, eq62_e1363_d_n16, eq62_e1363_d_n17, eq62_e1363_d_b0, eq62_e1363_d_b1, eq62_e1363_d_b2, eq62_e1363_d_b3, eq62_e1363_d_b4, eq62_e1363_d_b5, eq62_e1363_d_b6, eq62_e1363_d_b7, eq62_e1363_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e1365;
        let eq62_node_derivatives: [f64; 18] = [eq62_e1365_d_n0, eq62_e1365_d_n1, eq62_e1365_d_n2, eq62_e1365_d_n3, eq62_e1365_d_n4, eq62_e1365_d_n5, eq62_e1365_d_n6, eq62_e1365_d_n7, eq62_e1365_d_n8, eq62_e1365_d_n9, eq62_e1365_d_n10, eq62_e1365_d_n11, eq62_e1365_d_n12, eq62_e1365_d_n13, eq62_e1365_d_n14, eq62_e1365_d_n15, eq62_e1365_d_n16, eq62_e1365_d_n17];
        let eq62_branch_derivatives: [f64; 9] = [eq62_e1365_d_b0, eq62_e1365_d_b1, eq62_e1365_d_b2, eq62_e1365_d_b3, eq62_e1365_d_b4, eq62_e1365_d_b5, eq62_e1365_d_b6, eq62_e1365_d_b7, eq62_e1365_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            None,
            self.multiplicity * (eq62_value),
            &nodes,
            &eq62_node_derivatives,
            &branches,
            &eq62_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_63_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq63_e1370,) = {
    if (!(p.p28 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq63_value: f64 = eq63_e1370;
        stamper.stamp_potential(
            branches[9],
            eq63_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_64_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq64_e1375,) = {
    if (!(p.p28 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq64_value: f64 = eq64_e1375;
        stamper.stamp_potential(
            branches[10],
            eq64_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_65_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq65_e1379, eq65_e1379_d_n0, eq65_e1379_d_n1, eq65_e1379_d_n2, eq65_e1379_d_n3, eq65_e1379_d_n4, eq65_e1379_d_n5, eq65_e1379_d_n6, eq65_e1379_d_n7, eq65_e1379_d_n8, eq65_e1379_d_n9, eq65_e1379_d_n10, eq65_e1379_d_n11, eq65_e1379_d_n12, eq65_e1379_d_n13, eq65_e1379_d_n14, eq65_e1379_d_n15, eq65_e1379_d_n16, eq65_e1379_d_n17, eq65_e1379_d_b0, eq65_e1379_d_b1, eq65_e1379_d_b2, eq65_e1379_d_b3, eq65_e1379_d_b4, eq65_e1379_d_b5, eq65_e1379_d_b6, eq65_e1379_d_b7, eq65_e1379_d_b8,) = {
    if (p.p29 != 0.0) {
        (s.v[815], s.dn[815][0], s.dn[815][1], s.dn[815][2], s.dn[815][3], s.dn[815][4], s.dn[815][5], s.dn[815][6], s.dn[815][7], s.dn[815][8], s.dn[815][9], s.dn[815][10], s.dn[815][11], s.dn[815][12], s.dn[815][13], s.dn[815][14], s.dn[815][15], s.dn[815][16], s.dn[815][17], s.db[815][0], s.db[815][1], s.db[815][2], s.db[815][3], s.db[815][4], s.db[815][5], s.db[815][6], s.db[815][7], s.db[815][8],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e1379;
        let eq65_node_derivatives: [f64; 18] = [eq65_e1379_d_n0, eq65_e1379_d_n1, eq65_e1379_d_n2, eq65_e1379_d_n3, eq65_e1379_d_n4, eq65_e1379_d_n5, eq65_e1379_d_n6, eq65_e1379_d_n7, eq65_e1379_d_n8, eq65_e1379_d_n9, eq65_e1379_d_n10, eq65_e1379_d_n11, eq65_e1379_d_n12, eq65_e1379_d_n13, eq65_e1379_d_n14, eq65_e1379_d_n15, eq65_e1379_d_n16, eq65_e1379_d_n17];
        let eq65_branch_derivatives: [f64; 9] = [eq65_e1379_d_b0, eq65_e1379_d_b1, eq65_e1379_d_b2, eq65_e1379_d_b3, eq65_e1379_d_b4, eq65_e1379_d_b5, eq65_e1379_d_b6, eq65_e1379_d_b7, eq65_e1379_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            None,
            self.multiplicity * (eq65_value),
            &nodes,
            &eq65_node_derivatives,
            &branches,
            &eq65_branch_derivatives,
            self.multiplicity,
        );
    }
}
