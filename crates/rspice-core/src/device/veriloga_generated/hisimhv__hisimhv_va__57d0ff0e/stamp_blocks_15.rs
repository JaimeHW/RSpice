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
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq50_e1298, eq50_e1298_d_n0, eq50_e1298_d_n1, eq50_e1298_d_n2, eq50_e1298_d_n3, eq50_e1298_d_n4, eq50_e1298_d_n5, eq50_e1298_d_n6, eq50_e1298_d_n7, eq50_e1298_d_n8, eq50_e1298_d_n9, eq50_e1298_d_n10, eq50_e1298_d_n11, eq50_e1298_d_n12, eq50_e1298_d_n13, eq50_e1298_d_n14, eq50_e1298_d_n15, eq50_e1298_d_n16, eq50_e1298_d_n17, eq50_e1298_d_n18, eq50_e1298_d_b0, eq50_e1298_d_b1, eq50_e1298_d_b2, eq50_e1298_d_b3, eq50_e1298_d_b4, eq50_e1298_d_b5, eq50_e1298_d_b6, eq50_e1298_d_b7, eq50_e1298_d_b8, eq50_e1298_d_b9,) = {
    if (p.p52 != 0.0) {
        let eq50_e1296: f64 = (s.v[656] * (nv11 - nv9));
        let eq50_e1296_d_n0: f64 = (s.dn[656][0] * (nv11 - nv9));
        let eq50_e1296_d_n1: f64 = (s.dn[656][1] * (nv11 - nv9));
        let eq50_e1296_d_n2: f64 = (s.dn[656][2] * (nv11 - nv9));
        let eq50_e1296_d_n3: f64 = (s.dn[656][3] * (nv11 - nv9));
        let eq50_e1296_d_n4: f64 = (s.dn[656][4] * (nv11 - nv9));
        let eq50_e1296_d_n5: f64 = (s.dn[656][5] * (nv11 - nv9));
        let eq50_e1296_d_n6: f64 = (s.dn[656][6] * (nv11 - nv9));
        let eq50_e1296_d_n7: f64 = (s.dn[656][7] * (nv11 - nv9));
        let eq50_e1296_d_n8: f64 = (s.dn[656][8] * (nv11 - nv9));
        let eq50_e1296_d_n9: f64 = ((s.dn[656][9] * (nv11 - nv9)) + (-s.v[656]));
        let eq50_e1296_d_n10: f64 = (s.dn[656][10] * (nv11 - nv9));
        let eq50_e1296_d_n11: f64 = ((s.dn[656][11] * (nv11 - nv9)) + s.v[656]);
        let eq50_e1296_d_n12: f64 = (s.dn[656][12] * (nv11 - nv9));
        let eq50_e1296_d_n13: f64 = (s.dn[656][13] * (nv11 - nv9));
        let eq50_e1296_d_n14: f64 = (s.dn[656][14] * (nv11 - nv9));
        let eq50_e1296_d_n15: f64 = (s.dn[656][15] * (nv11 - nv9));
        let eq50_e1296_d_n16: f64 = (s.dn[656][16] * (nv11 - nv9));
        let eq50_e1296_d_n17: f64 = (s.dn[656][17] * (nv11 - nv9));
        let eq50_e1296_d_n18: f64 = (s.dn[656][18] * (nv11 - nv9));
        let eq50_e1296_d_b0: f64 = (s.db[656][0] * (nv11 - nv9));
        let eq50_e1296_d_b1: f64 = (s.db[656][1] * (nv11 - nv9));
        let eq50_e1296_d_b2: f64 = (s.db[656][2] * (nv11 - nv9));
        let eq50_e1296_d_b3: f64 = (s.db[656][3] * (nv11 - nv9));
        let eq50_e1296_d_b4: f64 = (s.db[656][4] * (nv11 - nv9));
        let eq50_e1296_d_b5: f64 = (s.db[656][5] * (nv11 - nv9));
        let eq50_e1296_d_b6: f64 = (s.db[656][6] * (nv11 - nv9));
        let eq50_e1296_d_b7: f64 = (s.db[656][7] * (nv11 - nv9));
        let eq50_e1296_d_b8: f64 = (s.db[656][8] * (nv11 - nv9));
        let eq50_e1296_d_b9: f64 = (s.db[656][9] * (nv11 - nv9));
        (eq50_e1296, eq50_e1296_d_n0, eq50_e1296_d_n1, eq50_e1296_d_n2, eq50_e1296_d_n3, eq50_e1296_d_n4, eq50_e1296_d_n5, eq50_e1296_d_n6, eq50_e1296_d_n7, eq50_e1296_d_n8, eq50_e1296_d_n9, eq50_e1296_d_n10, eq50_e1296_d_n11, eq50_e1296_d_n12, eq50_e1296_d_n13, eq50_e1296_d_n14, eq50_e1296_d_n15, eq50_e1296_d_n16, eq50_e1296_d_n17, eq50_e1296_d_n18, eq50_e1296_d_b0, eq50_e1296_d_b1, eq50_e1296_d_b2, eq50_e1296_d_b3, eq50_e1296_d_b4, eq50_e1296_d_b5, eq50_e1296_d_b6, eq50_e1296_d_b7, eq50_e1296_d_b8, eq50_e1296_d_b9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e1298;
        let eq50_node_derivatives: [f64; 19] = [eq50_e1298_d_n0, eq50_e1298_d_n1, eq50_e1298_d_n2, eq50_e1298_d_n3, eq50_e1298_d_n4, eq50_e1298_d_n5, eq50_e1298_d_n6, eq50_e1298_d_n7, eq50_e1298_d_n8, eq50_e1298_d_n9, eq50_e1298_d_n10, eq50_e1298_d_n11, eq50_e1298_d_n12, eq50_e1298_d_n13, eq50_e1298_d_n14, eq50_e1298_d_n15, eq50_e1298_d_n16, eq50_e1298_d_n17, eq50_e1298_d_n18];
        let eq50_branch_derivatives: [f64; 10] = [eq50_e1298_d_b0, eq50_e1298_d_b1, eq50_e1298_d_b2, eq50_e1298_d_b3, eq50_e1298_d_b4, eq50_e1298_d_b5, eq50_e1298_d_b6, eq50_e1298_d_b7, eq50_e1298_d_b8, eq50_e1298_d_b9];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[9]),
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
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq51_e1304, eq51_e1304_d_n0, eq51_e1304_d_n1, eq51_e1304_d_n2, eq51_e1304_d_n3, eq51_e1304_d_n4, eq51_e1304_d_n5, eq51_e1304_d_n6, eq51_e1304_d_n7, eq51_e1304_d_n8, eq51_e1304_d_n9, eq51_e1304_d_n10, eq51_e1304_d_n11, eq51_e1304_d_n12, eq51_e1304_d_n13, eq51_e1304_d_n14, eq51_e1304_d_n15, eq51_e1304_d_n16, eq51_e1304_d_n17, eq51_e1304_d_n18, eq51_e1304_d_b0, eq51_e1304_d_b1, eq51_e1304_d_b2, eq51_e1304_d_b3, eq51_e1304_d_b4, eq51_e1304_d_b5, eq51_e1304_d_b6, eq51_e1304_d_b7, eq51_e1304_d_b8, eq51_e1304_d_b9,) = {
    if (p.p52 != 0.0) {
        let eq51_e1302: f64 = (s.v[657] * (nv10 - nv9));
        let eq51_e1302_d_n0: f64 = (s.dn[657][0] * (nv10 - nv9));
        let eq51_e1302_d_n1: f64 = (s.dn[657][1] * (nv10 - nv9));
        let eq51_e1302_d_n2: f64 = (s.dn[657][2] * (nv10 - nv9));
        let eq51_e1302_d_n3: f64 = (s.dn[657][3] * (nv10 - nv9));
        let eq51_e1302_d_n4: f64 = (s.dn[657][4] * (nv10 - nv9));
        let eq51_e1302_d_n5: f64 = (s.dn[657][5] * (nv10 - nv9));
        let eq51_e1302_d_n6: f64 = (s.dn[657][6] * (nv10 - nv9));
        let eq51_e1302_d_n7: f64 = (s.dn[657][7] * (nv10 - nv9));
        let eq51_e1302_d_n8: f64 = (s.dn[657][8] * (nv10 - nv9));
        let eq51_e1302_d_n9: f64 = ((s.dn[657][9] * (nv10 - nv9)) + (-s.v[657]));
        let eq51_e1302_d_n10: f64 = ((s.dn[657][10] * (nv10 - nv9)) + s.v[657]);
        let eq51_e1302_d_n11: f64 = (s.dn[657][11] * (nv10 - nv9));
        let eq51_e1302_d_n12: f64 = (s.dn[657][12] * (nv10 - nv9));
        let eq51_e1302_d_n13: f64 = (s.dn[657][13] * (nv10 - nv9));
        let eq51_e1302_d_n14: f64 = (s.dn[657][14] * (nv10 - nv9));
        let eq51_e1302_d_n15: f64 = (s.dn[657][15] * (nv10 - nv9));
        let eq51_e1302_d_n16: f64 = (s.dn[657][16] * (nv10 - nv9));
        let eq51_e1302_d_n17: f64 = (s.dn[657][17] * (nv10 - nv9));
        let eq51_e1302_d_n18: f64 = (s.dn[657][18] * (nv10 - nv9));
        let eq51_e1302_d_b0: f64 = (s.db[657][0] * (nv10 - nv9));
        let eq51_e1302_d_b1: f64 = (s.db[657][1] * (nv10 - nv9));
        let eq51_e1302_d_b2: f64 = (s.db[657][2] * (nv10 - nv9));
        let eq51_e1302_d_b3: f64 = (s.db[657][3] * (nv10 - nv9));
        let eq51_e1302_d_b4: f64 = (s.db[657][4] * (nv10 - nv9));
        let eq51_e1302_d_b5: f64 = (s.db[657][5] * (nv10 - nv9));
        let eq51_e1302_d_b6: f64 = (s.db[657][6] * (nv10 - nv9));
        let eq51_e1302_d_b7: f64 = (s.db[657][7] * (nv10 - nv9));
        let eq51_e1302_d_b8: f64 = (s.db[657][8] * (nv10 - nv9));
        let eq51_e1302_d_b9: f64 = (s.db[657][9] * (nv10 - nv9));
        (eq51_e1302, eq51_e1302_d_n0, eq51_e1302_d_n1, eq51_e1302_d_n2, eq51_e1302_d_n3, eq51_e1302_d_n4, eq51_e1302_d_n5, eq51_e1302_d_n6, eq51_e1302_d_n7, eq51_e1302_d_n8, eq51_e1302_d_n9, eq51_e1302_d_n10, eq51_e1302_d_n11, eq51_e1302_d_n12, eq51_e1302_d_n13, eq51_e1302_d_n14, eq51_e1302_d_n15, eq51_e1302_d_n16, eq51_e1302_d_n17, eq51_e1302_d_n18, eq51_e1302_d_b0, eq51_e1302_d_b1, eq51_e1302_d_b2, eq51_e1302_d_b3, eq51_e1302_d_b4, eq51_e1302_d_b5, eq51_e1302_d_b6, eq51_e1302_d_b7, eq51_e1302_d_b8, eq51_e1302_d_b9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e1304;
        let eq51_node_derivatives: [f64; 19] = [eq51_e1304_d_n0, eq51_e1304_d_n1, eq51_e1304_d_n2, eq51_e1304_d_n3, eq51_e1304_d_n4, eq51_e1304_d_n5, eq51_e1304_d_n6, eq51_e1304_d_n7, eq51_e1304_d_n8, eq51_e1304_d_n9, eq51_e1304_d_n10, eq51_e1304_d_n11, eq51_e1304_d_n12, eq51_e1304_d_n13, eq51_e1304_d_n14, eq51_e1304_d_n15, eq51_e1304_d_n16, eq51_e1304_d_n17, eq51_e1304_d_n18];
        let eq51_branch_derivatives: [f64; 10] = [eq51_e1304_d_b0, eq51_e1304_d_b1, eq51_e1304_d_b2, eq51_e1304_d_b3, eq51_e1304_d_b4, eq51_e1304_d_b5, eq51_e1304_d_b6, eq51_e1304_d_b7, eq51_e1304_d_b8, eq51_e1304_d_b9];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[9]),
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq52_e1310, eq52_e1310_d_n0, eq52_e1310_d_n1, eq52_e1310_d_n2, eq52_e1310_d_n3, eq52_e1310_d_n4, eq52_e1310_d_n5, eq52_e1310_d_n6, eq52_e1310_d_n7, eq52_e1310_d_n8, eq52_e1310_d_n9, eq52_e1310_d_n10, eq52_e1310_d_n11, eq52_e1310_d_n12, eq52_e1310_d_n13, eq52_e1310_d_n14, eq52_e1310_d_n15, eq52_e1310_d_n16, eq52_e1310_d_n17, eq52_e1310_d_n18, eq52_e1310_d_b0, eq52_e1310_d_b1, eq52_e1310_d_b2, eq52_e1310_d_b3, eq52_e1310_d_b4, eq52_e1310_d_b5, eq52_e1310_d_b6, eq52_e1310_d_b7, eq52_e1310_d_b8, eq52_e1310_d_b9,) = {
    if (p.p52 != 0.0) {
        let eq52_e1308: f64 = (s.v[655] * (nv3 - nv9));
        let eq52_e1308_d_n0: f64 = (s.dn[655][0] * (nv3 - nv9));
        let eq52_e1308_d_n1: f64 = (s.dn[655][1] * (nv3 - nv9));
        let eq52_e1308_d_n2: f64 = (s.dn[655][2] * (nv3 - nv9));
        let eq52_e1308_d_n3: f64 = ((s.dn[655][3] * (nv3 - nv9)) + s.v[655]);
        let eq52_e1308_d_n4: f64 = (s.dn[655][4] * (nv3 - nv9));
        let eq52_e1308_d_n5: f64 = (s.dn[655][5] * (nv3 - nv9));
        let eq52_e1308_d_n6: f64 = (s.dn[655][6] * (nv3 - nv9));
        let eq52_e1308_d_n7: f64 = (s.dn[655][7] * (nv3 - nv9));
        let eq52_e1308_d_n8: f64 = (s.dn[655][8] * (nv3 - nv9));
        let eq52_e1308_d_n9: f64 = ((s.dn[655][9] * (nv3 - nv9)) + (-s.v[655]));
        let eq52_e1308_d_n10: f64 = (s.dn[655][10] * (nv3 - nv9));
        let eq52_e1308_d_n11: f64 = (s.dn[655][11] * (nv3 - nv9));
        let eq52_e1308_d_n12: f64 = (s.dn[655][12] * (nv3 - nv9));
        let eq52_e1308_d_n13: f64 = (s.dn[655][13] * (nv3 - nv9));
        let eq52_e1308_d_n14: f64 = (s.dn[655][14] * (nv3 - nv9));
        let eq52_e1308_d_n15: f64 = (s.dn[655][15] * (nv3 - nv9));
        let eq52_e1308_d_n16: f64 = (s.dn[655][16] * (nv3 - nv9));
        let eq52_e1308_d_n17: f64 = (s.dn[655][17] * (nv3 - nv9));
        let eq52_e1308_d_n18: f64 = (s.dn[655][18] * (nv3 - nv9));
        let eq52_e1308_d_b0: f64 = (s.db[655][0] * (nv3 - nv9));
        let eq52_e1308_d_b1: f64 = (s.db[655][1] * (nv3 - nv9));
        let eq52_e1308_d_b2: f64 = (s.db[655][2] * (nv3 - nv9));
        let eq52_e1308_d_b3: f64 = (s.db[655][3] * (nv3 - nv9));
        let eq52_e1308_d_b4: f64 = (s.db[655][4] * (nv3 - nv9));
        let eq52_e1308_d_b5: f64 = (s.db[655][5] * (nv3 - nv9));
        let eq52_e1308_d_b6: f64 = (s.db[655][6] * (nv3 - nv9));
        let eq52_e1308_d_b7: f64 = (s.db[655][7] * (nv3 - nv9));
        let eq52_e1308_d_b8: f64 = (s.db[655][8] * (nv3 - nv9));
        let eq52_e1308_d_b9: f64 = (s.db[655][9] * (nv3 - nv9));
        (eq52_e1308, eq52_e1308_d_n0, eq52_e1308_d_n1, eq52_e1308_d_n2, eq52_e1308_d_n3, eq52_e1308_d_n4, eq52_e1308_d_n5, eq52_e1308_d_n6, eq52_e1308_d_n7, eq52_e1308_d_n8, eq52_e1308_d_n9, eq52_e1308_d_n10, eq52_e1308_d_n11, eq52_e1308_d_n12, eq52_e1308_d_n13, eq52_e1308_d_n14, eq52_e1308_d_n15, eq52_e1308_d_n16, eq52_e1308_d_n17, eq52_e1308_d_n18, eq52_e1308_d_b0, eq52_e1308_d_b1, eq52_e1308_d_b2, eq52_e1308_d_b3, eq52_e1308_d_b4, eq52_e1308_d_b5, eq52_e1308_d_b6, eq52_e1308_d_b7, eq52_e1308_d_b8, eq52_e1308_d_b9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e1310;
        let eq52_node_derivatives: [f64; 19] = [eq52_e1310_d_n0, eq52_e1310_d_n1, eq52_e1310_d_n2, eq52_e1310_d_n3, eq52_e1310_d_n4, eq52_e1310_d_n5, eq52_e1310_d_n6, eq52_e1310_d_n7, eq52_e1310_d_n8, eq52_e1310_d_n9, eq52_e1310_d_n10, eq52_e1310_d_n11, eq52_e1310_d_n12, eq52_e1310_d_n13, eq52_e1310_d_n14, eq52_e1310_d_n15, eq52_e1310_d_n16, eq52_e1310_d_n17, eq52_e1310_d_n18];
        let eq52_branch_derivatives: [f64; 10] = [eq52_e1310_d_b0, eq52_e1310_d_b1, eq52_e1310_d_b2, eq52_e1310_d_b3, eq52_e1310_d_b4, eq52_e1310_d_b5, eq52_e1310_d_b6, eq52_e1310_d_b7, eq52_e1310_d_b8, eq52_e1310_d_b9];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[9]),
            self.multiplicity * (eq52_value),
            &nodes,
            &eq52_node_derivatives,
            &branches,
            &eq52_branch_derivatives,
            self.multiplicity,
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
        let (eq53_e1315,) = {
    if (!(p.p52 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq53_value: f64 = eq53_e1315;
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
        let (eq54_e1320,) = {
    if (!(p.p52 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq54_value: f64 = eq54_e1320;
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
        let (eq55_e1325,) = {
    if (!(p.p52 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq55_value: f64 = eq55_e1325;
        stamper.stamp_potential(
            branches[9],
            eq55_value,
            &[
            ],
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq56_e1331, eq56_e1331_d_n0, eq56_e1331_d_n1, eq56_e1331_d_n2, eq56_e1331_d_n3, eq56_e1331_d_n4, eq56_e1331_d_n5, eq56_e1331_d_n6, eq56_e1331_d_n7, eq56_e1331_d_n8, eq56_e1331_d_n9, eq56_e1331_d_n10, eq56_e1331_d_n11, eq56_e1331_d_n12, eq56_e1331_d_n13, eq56_e1331_d_n14, eq56_e1331_d_n15, eq56_e1331_d_n16, eq56_e1331_d_n17, eq56_e1331_d_n18, eq56_e1331_d_b0, eq56_e1331_d_b1, eq56_e1331_d_b2, eq56_e1331_d_b3, eq56_e1331_d_b4, eq56_e1331_d_b5, eq56_e1331_d_b6, eq56_e1331_d_b7, eq56_e1331_d_b8, eq56_e1331_d_b9,) = {
    if (s.v[3413] != 0.0) {
        let eq56_e1329: f64 = ((nv5 - 0.0) * s.v[740]);
        let eq56_e1329_d_n0: f64 = ((nv5 - 0.0) * s.dn[740][0]);
        let eq56_e1329_d_n1: f64 = ((nv5 - 0.0) * s.dn[740][1]);
        let eq56_e1329_d_n2: f64 = ((nv5 - 0.0) * s.dn[740][2]);
        let eq56_e1329_d_n3: f64 = ((nv5 - 0.0) * s.dn[740][3]);
        let eq56_e1329_d_n4: f64 = ((nv5 - 0.0) * s.dn[740][4]);
        let eq56_e1329_d_n5: f64 = (s.v[740] + ((nv5 - 0.0) * s.dn[740][5]));
        let eq56_e1329_d_n6: f64 = ((nv5 - 0.0) * s.dn[740][6]);
        let eq56_e1329_d_n7: f64 = ((nv5 - 0.0) * s.dn[740][7]);
        let eq56_e1329_d_n8: f64 = ((nv5 - 0.0) * s.dn[740][8]);
        let eq56_e1329_d_n9: f64 = ((nv5 - 0.0) * s.dn[740][9]);
        let eq56_e1329_d_n10: f64 = ((nv5 - 0.0) * s.dn[740][10]);
        let eq56_e1329_d_n11: f64 = ((nv5 - 0.0) * s.dn[740][11]);
        let eq56_e1329_d_n12: f64 = ((nv5 - 0.0) * s.dn[740][12]);
        let eq56_e1329_d_n13: f64 = ((nv5 - 0.0) * s.dn[740][13]);
        let eq56_e1329_d_n14: f64 = ((nv5 - 0.0) * s.dn[740][14]);
        let eq56_e1329_d_n15: f64 = ((nv5 - 0.0) * s.dn[740][15]);
        let eq56_e1329_d_n16: f64 = ((nv5 - 0.0) * s.dn[740][16]);
        let eq56_e1329_d_n17: f64 = ((nv5 - 0.0) * s.dn[740][17]);
        let eq56_e1329_d_n18: f64 = ((nv5 - 0.0) * s.dn[740][18]);
        let eq56_e1329_d_b0: f64 = ((nv5 - 0.0) * s.db[740][0]);
        let eq56_e1329_d_b1: f64 = ((nv5 - 0.0) * s.db[740][1]);
        let eq56_e1329_d_b2: f64 = ((nv5 - 0.0) * s.db[740][2]);
        let eq56_e1329_d_b3: f64 = ((nv5 - 0.0) * s.db[740][3]);
        let eq56_e1329_d_b4: f64 = ((nv5 - 0.0) * s.db[740][4]);
        let eq56_e1329_d_b5: f64 = ((nv5 - 0.0) * s.db[740][5]);
        let eq56_e1329_d_b6: f64 = ((nv5 - 0.0) * s.db[740][6]);
        let eq56_e1329_d_b7: f64 = ((nv5 - 0.0) * s.db[740][7]);
        let eq56_e1329_d_b8: f64 = ((nv5 - 0.0) * s.db[740][8]);
        let eq56_e1329_d_b9: f64 = ((nv5 - 0.0) * s.db[740][9]);
        (eq56_e1329, eq56_e1329_d_n0, eq56_e1329_d_n1, eq56_e1329_d_n2, eq56_e1329_d_n3, eq56_e1329_d_n4, eq56_e1329_d_n5, eq56_e1329_d_n6, eq56_e1329_d_n7, eq56_e1329_d_n8, eq56_e1329_d_n9, eq56_e1329_d_n10, eq56_e1329_d_n11, eq56_e1329_d_n12, eq56_e1329_d_n13, eq56_e1329_d_n14, eq56_e1329_d_n15, eq56_e1329_d_n16, eq56_e1329_d_n17, eq56_e1329_d_n18, eq56_e1329_d_b0, eq56_e1329_d_b1, eq56_e1329_d_b2, eq56_e1329_d_b3, eq56_e1329_d_b4, eq56_e1329_d_b5, eq56_e1329_d_b6, eq56_e1329_d_b7, eq56_e1329_d_b8, eq56_e1329_d_b9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_value: f64 = eq56_e1331;
        let eq56_node_derivatives: [f64; 19] = [eq56_e1331_d_n0, eq56_e1331_d_n1, eq56_e1331_d_n2, eq56_e1331_d_n3, eq56_e1331_d_n4, eq56_e1331_d_n5, eq56_e1331_d_n6, eq56_e1331_d_n7, eq56_e1331_d_n8, eq56_e1331_d_n9, eq56_e1331_d_n10, eq56_e1331_d_n11, eq56_e1331_d_n12, eq56_e1331_d_n13, eq56_e1331_d_n14, eq56_e1331_d_n15, eq56_e1331_d_n16, eq56_e1331_d_n17, eq56_e1331_d_n18];
        let eq56_branch_derivatives: [f64; 10] = [eq56_e1331_d_b0, eq56_e1331_d_b1, eq56_e1331_d_b2, eq56_e1331_d_b3, eq56_e1331_d_b4, eq56_e1331_d_b5, eq56_e1331_d_b6, eq56_e1331_d_b7, eq56_e1331_d_b8, eq56_e1331_d_b9];
        stamper.stamp_current_dense(
            Some(nodes[5]),
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
        let (eq57_e1336, eq57_e1336_d_n0, eq57_e1336_d_n1, eq57_e1336_d_n2, eq57_e1336_d_n3, eq57_e1336_d_n4, eq57_e1336_d_n5, eq57_e1336_d_n6, eq57_e1336_d_n7, eq57_e1336_d_n8, eq57_e1336_d_n9, eq57_e1336_d_n10, eq57_e1336_d_n11, eq57_e1336_d_n12, eq57_e1336_d_n13, eq57_e1336_d_n14, eq57_e1336_d_n15, eq57_e1336_d_n16, eq57_e1336_d_n17, eq57_e1336_d_n18, eq57_e1336_d_b0, eq57_e1336_d_b1, eq57_e1336_d_b2, eq57_e1336_d_b3, eq57_e1336_d_b4, eq57_e1336_d_b5, eq57_e1336_d_b6, eq57_e1336_d_b7, eq57_e1336_d_b8, eq57_e1336_d_b9,) = {
    if (s.v[3413] != 0.0) {
        let eq57_e1334: f64 = (-s.v[802]);
        let eq57_e1334_d_n0: f64 = (-s.dn[802][0]);
        let eq57_e1334_d_n1: f64 = (-s.dn[802][1]);
        let eq57_e1334_d_n2: f64 = (-s.dn[802][2]);
        let eq57_e1334_d_n3: f64 = (-s.dn[802][3]);
        let eq57_e1334_d_n4: f64 = (-s.dn[802][4]);
        let eq57_e1334_d_n5: f64 = (-s.dn[802][5]);
        let eq57_e1334_d_n6: f64 = (-s.dn[802][6]);
        let eq57_e1334_d_n7: f64 = (-s.dn[802][7]);
        let eq57_e1334_d_n8: f64 = (-s.dn[802][8]);
        let eq57_e1334_d_n9: f64 = (-s.dn[802][9]);
        let eq57_e1334_d_n10: f64 = (-s.dn[802][10]);
        let eq57_e1334_d_n11: f64 = (-s.dn[802][11]);
        let eq57_e1334_d_n12: f64 = (-s.dn[802][12]);
        let eq57_e1334_d_n13: f64 = (-s.dn[802][13]);
        let eq57_e1334_d_n14: f64 = (-s.dn[802][14]);
        let eq57_e1334_d_n15: f64 = (-s.dn[802][15]);
        let eq57_e1334_d_n16: f64 = (-s.dn[802][16]);
        let eq57_e1334_d_n17: f64 = (-s.dn[802][17]);
        let eq57_e1334_d_n18: f64 = (-s.dn[802][18]);
        let eq57_e1334_d_b0: f64 = (-s.db[802][0]);
        let eq57_e1334_d_b1: f64 = (-s.db[802][1]);
        let eq57_e1334_d_b2: f64 = (-s.db[802][2]);
        let eq57_e1334_d_b3: f64 = (-s.db[802][3]);
        let eq57_e1334_d_b4: f64 = (-s.db[802][4]);
        let eq57_e1334_d_b5: f64 = (-s.db[802][5]);
        let eq57_e1334_d_b6: f64 = (-s.db[802][6]);
        let eq57_e1334_d_b7: f64 = (-s.db[802][7]);
        let eq57_e1334_d_b8: f64 = (-s.db[802][8]);
        let eq57_e1334_d_b9: f64 = (-s.db[802][9]);
        (eq57_e1334, eq57_e1334_d_n0, eq57_e1334_d_n1, eq57_e1334_d_n2, eq57_e1334_d_n3, eq57_e1334_d_n4, eq57_e1334_d_n5, eq57_e1334_d_n6, eq57_e1334_d_n7, eq57_e1334_d_n8, eq57_e1334_d_n9, eq57_e1334_d_n10, eq57_e1334_d_n11, eq57_e1334_d_n12, eq57_e1334_d_n13, eq57_e1334_d_n14, eq57_e1334_d_n15, eq57_e1334_d_n16, eq57_e1334_d_n17, eq57_e1334_d_n18, eq57_e1334_d_b0, eq57_e1334_d_b1, eq57_e1334_d_b2, eq57_e1334_d_b3, eq57_e1334_d_b4, eq57_e1334_d_b5, eq57_e1334_d_b6, eq57_e1334_d_b7, eq57_e1334_d_b8, eq57_e1334_d_b9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e1336;
        let eq57_node_derivatives: [f64; 19] = [eq57_e1336_d_n0, eq57_e1336_d_n1, eq57_e1336_d_n2, eq57_e1336_d_n3, eq57_e1336_d_n4, eq57_e1336_d_n5, eq57_e1336_d_n6, eq57_e1336_d_n7, eq57_e1336_d_n8, eq57_e1336_d_n9, eq57_e1336_d_n10, eq57_e1336_d_n11, eq57_e1336_d_n12, eq57_e1336_d_n13, eq57_e1336_d_n14, eq57_e1336_d_n15, eq57_e1336_d_n16, eq57_e1336_d_n17, eq57_e1336_d_n18];
        let eq57_branch_derivatives: [f64; 10] = [eq57_e1336_d_b0, eq57_e1336_d_b1, eq57_e1336_d_b2, eq57_e1336_d_b3, eq57_e1336_d_b4, eq57_e1336_d_b5, eq57_e1336_d_b6, eq57_e1336_d_b7, eq57_e1336_d_b8, eq57_e1336_d_b9];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            None,
            self.multiplicity * (eq57_value),
            &nodes,
            &eq57_node_derivatives,
            &branches,
            &eq57_branch_derivatives,
            self.multiplicity,
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq58_e1343, eq58_e1343_d_n5,) = {
    if (!(s.v[3413] != 0.0)) {
        let eq58_e1341: f64 = ((nv5 - 0.0) * 10000.0);
        let eq58_e1341_d_n5: f64 = 10000.0;
        (eq58_e1341, eq58_e1341_d_n5,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e1343;
        stamper.stamp_current(
            Some(nodes[5]),
            None,
            self.multiplicity * (eq58_value),
            &[
                GeneratedDerivative::node(nodes[5], self.multiplicity * eq58_e1343_d_n5),
            ],
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq59_e1346: f64 = (s.v[767] * (nv5 - 0.0));
        let eq59_e1346_d_n0: f64 = (s.dn[767][0] * (nv5 - 0.0));
        let eq59_e1346_d_n1: f64 = (s.dn[767][1] * (nv5 - 0.0));
        let eq59_e1346_d_n2: f64 = (s.dn[767][2] * (nv5 - 0.0));
        let eq59_e1346_d_n3: f64 = (s.dn[767][3] * (nv5 - 0.0));
        let eq59_e1346_d_n4: f64 = (s.dn[767][4] * (nv5 - 0.0));
        let eq59_e1346_d_n5: f64 = ((s.dn[767][5] * (nv5 - 0.0)) + s.v[767]);
        let eq59_e1346_d_n6: f64 = (s.dn[767][6] * (nv5 - 0.0));
        let eq59_e1346_d_n7: f64 = (s.dn[767][7] * (nv5 - 0.0));
        let eq59_e1346_d_n8: f64 = (s.dn[767][8] * (nv5 - 0.0));
        let eq59_e1346_d_n9: f64 = (s.dn[767][9] * (nv5 - 0.0));
        let eq59_e1346_d_n10: f64 = (s.dn[767][10] * (nv5 - 0.0));
        let eq59_e1346_d_n11: f64 = (s.dn[767][11] * (nv5 - 0.0));
        let eq59_e1346_d_n12: f64 = (s.dn[767][12] * (nv5 - 0.0));
        let eq59_e1346_d_n13: f64 = (s.dn[767][13] * (nv5 - 0.0));
        let eq59_e1346_d_n14: f64 = (s.dn[767][14] * (nv5 - 0.0));
        let eq59_e1346_d_n15: f64 = (s.dn[767][15] * (nv5 - 0.0));
        let eq59_e1346_d_n16: f64 = (s.dn[767][16] * (nv5 - 0.0));
        let eq59_e1346_d_n17: f64 = (s.dn[767][17] * (nv5 - 0.0));
        let eq59_e1346_d_n18: f64 = (s.dn[767][18] * (nv5 - 0.0));
        let eq59_e1346_d_b0: f64 = (s.db[767][0] * (nv5 - 0.0));
        let eq59_e1346_d_b1: f64 = (s.db[767][1] * (nv5 - 0.0));
        let eq59_e1346_d_b2: f64 = (s.db[767][2] * (nv5 - 0.0));
        let eq59_e1346_d_b3: f64 = (s.db[767][3] * (nv5 - 0.0));
        let eq59_e1346_d_b4: f64 = (s.db[767][4] * (nv5 - 0.0));
        let eq59_e1346_d_b5: f64 = (s.db[767][5] * (nv5 - 0.0));
        let eq59_e1346_d_b6: f64 = (s.db[767][6] * (nv5 - 0.0));
        let eq59_e1346_d_b7: f64 = (s.db[767][7] * (nv5 - 0.0));
        let eq59_e1346_d_b8: f64 = (s.db[767][8] * (nv5 - 0.0));
        let eq59_e1346_d_b9: f64 = (s.db[767][9] * (nv5 - 0.0));
        let eq59_e1347: f64 = self.eval_ddt(17, eq59_e1346);
        let eq59_e1347_d_n0: f64 = self.ddt_jacobian(eq59_e1346_d_n0);
        let eq59_e1347_d_n1: f64 = self.ddt_jacobian(eq59_e1346_d_n1);
        let eq59_e1347_d_n2: f64 = self.ddt_jacobian(eq59_e1346_d_n2);
        let eq59_e1347_d_n3: f64 = self.ddt_jacobian(eq59_e1346_d_n3);
        let eq59_e1347_d_n4: f64 = self.ddt_jacobian(eq59_e1346_d_n4);
        let eq59_e1347_d_n5: f64 = self.ddt_jacobian(eq59_e1346_d_n5);
        let eq59_e1347_d_n6: f64 = self.ddt_jacobian(eq59_e1346_d_n6);
        let eq59_e1347_d_n7: f64 = self.ddt_jacobian(eq59_e1346_d_n7);
        let eq59_e1347_d_n8: f64 = self.ddt_jacobian(eq59_e1346_d_n8);
        let eq59_e1347_d_n9: f64 = self.ddt_jacobian(eq59_e1346_d_n9);
        let eq59_e1347_d_n10: f64 = self.ddt_jacobian(eq59_e1346_d_n10);
        let eq59_e1347_d_n11: f64 = self.ddt_jacobian(eq59_e1346_d_n11);
        let eq59_e1347_d_n12: f64 = self.ddt_jacobian(eq59_e1346_d_n12);
        let eq59_e1347_d_n13: f64 = self.ddt_jacobian(eq59_e1346_d_n13);
        let eq59_e1347_d_n14: f64 = self.ddt_jacobian(eq59_e1346_d_n14);
        let eq59_e1347_d_n15: f64 = self.ddt_jacobian(eq59_e1346_d_n15);
        let eq59_e1347_d_n16: f64 = self.ddt_jacobian(eq59_e1346_d_n16);
        let eq59_e1347_d_n17: f64 = self.ddt_jacobian(eq59_e1346_d_n17);
        let eq59_e1347_d_n18: f64 = self.ddt_jacobian(eq59_e1346_d_n18);
        let eq59_e1347_d_b0: f64 = self.ddt_jacobian(eq59_e1346_d_b0);
        let eq59_e1347_d_b1: f64 = self.ddt_jacobian(eq59_e1346_d_b1);
        let eq59_e1347_d_b2: f64 = self.ddt_jacobian(eq59_e1346_d_b2);
        let eq59_e1347_d_b3: f64 = self.ddt_jacobian(eq59_e1346_d_b3);
        let eq59_e1347_d_b4: f64 = self.ddt_jacobian(eq59_e1346_d_b4);
        let eq59_e1347_d_b5: f64 = self.ddt_jacobian(eq59_e1346_d_b5);
        let eq59_e1347_d_b6: f64 = self.ddt_jacobian(eq59_e1346_d_b6);
        let eq59_e1347_d_b7: f64 = self.ddt_jacobian(eq59_e1346_d_b7);
        let eq59_e1347_d_b8: f64 = self.ddt_jacobian(eq59_e1346_d_b8);
        let eq59_e1347_d_b9: f64 = self.ddt_jacobian(eq59_e1346_d_b9);
        let eq59_value: f64 = eq59_e1347;
        let eq59_node_derivatives: [f64; 19] = [eq59_e1347_d_n0, eq59_e1347_d_n1, eq59_e1347_d_n2, eq59_e1347_d_n3, eq59_e1347_d_n4, eq59_e1347_d_n5, eq59_e1347_d_n6, eq59_e1347_d_n7, eq59_e1347_d_n8, eq59_e1347_d_n9, eq59_e1347_d_n10, eq59_e1347_d_n11, eq59_e1347_d_n12, eq59_e1347_d_n13, eq59_e1347_d_n14, eq59_e1347_d_n15, eq59_e1347_d_n16, eq59_e1347_d_n17, eq59_e1347_d_n18];
        let eq59_branch_derivatives: [f64; 10] = [eq59_e1347_d_b0, eq59_e1347_d_b1, eq59_e1347_d_b2, eq59_e1347_d_b3, eq59_e1347_d_b4, eq59_e1347_d_b5, eq59_e1347_d_b6, eq59_e1347_d_b7, eq59_e1347_d_b8, eq59_e1347_d_b9];
        stamper.stamp_current_dense(
            Some(nodes[5]),
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
        let (eq60_e1351, eq60_e1351_d_n0, eq60_e1351_d_n1, eq60_e1351_d_n2, eq60_e1351_d_n3, eq60_e1351_d_n4, eq60_e1351_d_n5, eq60_e1351_d_n6, eq60_e1351_d_n7, eq60_e1351_d_n8, eq60_e1351_d_n9, eq60_e1351_d_n10, eq60_e1351_d_n11, eq60_e1351_d_n12, eq60_e1351_d_n13, eq60_e1351_d_n14, eq60_e1351_d_n15, eq60_e1351_d_n16, eq60_e1351_d_n17, eq60_e1351_d_n18, eq60_e1351_d_b0, eq60_e1351_d_b1, eq60_e1351_d_b2, eq60_e1351_d_b3, eq60_e1351_d_b4, eq60_e1351_d_b5, eq60_e1351_d_b6, eq60_e1351_d_b7, eq60_e1351_d_b8, eq60_e1351_d_b9,) = {
    if (p.p28 != 0.0) {
        (s.v[749], s.dn[749][0], s.dn[749][1], s.dn[749][2], s.dn[749][3], s.dn[749][4], s.dn[749][5], s.dn[749][6], s.dn[749][7], s.dn[749][8], s.dn[749][9], s.dn[749][10], s.dn[749][11], s.dn[749][12], s.dn[749][13], s.dn[749][14], s.dn[749][15], s.dn[749][16], s.dn[749][17], s.dn[749][18], s.db[749][0], s.db[749][1], s.db[749][2], s.db[749][3], s.db[749][4], s.db[749][5], s.db[749][6], s.db[749][7], s.db[749][8], s.db[749][9],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e1351;
        let eq60_node_derivatives: [f64; 19] = [eq60_e1351_d_n0, eq60_e1351_d_n1, eq60_e1351_d_n2, eq60_e1351_d_n3, eq60_e1351_d_n4, eq60_e1351_d_n5, eq60_e1351_d_n6, eq60_e1351_d_n7, eq60_e1351_d_n8, eq60_e1351_d_n9, eq60_e1351_d_n10, eq60_e1351_d_n11, eq60_e1351_d_n12, eq60_e1351_d_n13, eq60_e1351_d_n14, eq60_e1351_d_n15, eq60_e1351_d_n16, eq60_e1351_d_n17, eq60_e1351_d_n18];
        let eq60_branch_derivatives: [f64; 10] = [eq60_e1351_d_b0, eq60_e1351_d_b1, eq60_e1351_d_b2, eq60_e1351_d_b3, eq60_e1351_d_b4, eq60_e1351_d_b5, eq60_e1351_d_b6, eq60_e1351_d_b7, eq60_e1351_d_b8, eq60_e1351_d_b9];
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
        let (eq61_e1355, eq61_e1355_d_n0, eq61_e1355_d_n1, eq61_e1355_d_n2, eq61_e1355_d_n3, eq61_e1355_d_n4, eq61_e1355_d_n5, eq61_e1355_d_n6, eq61_e1355_d_n7, eq61_e1355_d_n8, eq61_e1355_d_n9, eq61_e1355_d_n10, eq61_e1355_d_n11, eq61_e1355_d_n12, eq61_e1355_d_n13, eq61_e1355_d_n14, eq61_e1355_d_n15, eq61_e1355_d_n16, eq61_e1355_d_n17, eq61_e1355_d_n18, eq61_e1355_d_b0, eq61_e1355_d_b1, eq61_e1355_d_b2, eq61_e1355_d_b3, eq61_e1355_d_b4, eq61_e1355_d_b5, eq61_e1355_d_b6, eq61_e1355_d_b7, eq61_e1355_d_b8, eq61_e1355_d_b9,) = {
    if (p.p28 != 0.0) {
        (s.v[750], s.dn[750][0], s.dn[750][1], s.dn[750][2], s.dn[750][3], s.dn[750][4], s.dn[750][5], s.dn[750][6], s.dn[750][7], s.dn[750][8], s.dn[750][9], s.dn[750][10], s.dn[750][11], s.dn[750][12], s.dn[750][13], s.dn[750][14], s.dn[750][15], s.dn[750][16], s.dn[750][17], s.dn[750][18], s.db[750][0], s.db[750][1], s.db[750][2], s.db[750][3], s.db[750][4], s.db[750][5], s.db[750][6], s.db[750][7], s.db[750][8], s.db[750][9],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_value: f64 = eq61_e1355;
        let eq61_node_derivatives: [f64; 19] = [eq61_e1355_d_n0, eq61_e1355_d_n1, eq61_e1355_d_n2, eq61_e1355_d_n3, eq61_e1355_d_n4, eq61_e1355_d_n5, eq61_e1355_d_n6, eq61_e1355_d_n7, eq61_e1355_d_n8, eq61_e1355_d_n9, eq61_e1355_d_n10, eq61_e1355_d_n11, eq61_e1355_d_n12, eq61_e1355_d_n13, eq61_e1355_d_n14, eq61_e1355_d_n15, eq61_e1355_d_n16, eq61_e1355_d_n17, eq61_e1355_d_n18];
        let eq61_branch_derivatives: [f64; 10] = [eq61_e1355_d_b0, eq61_e1355_d_b1, eq61_e1355_d_b2, eq61_e1355_d_b3, eq61_e1355_d_b4, eq61_e1355_d_b5, eq61_e1355_d_b6, eq61_e1355_d_b7, eq61_e1355_d_b8, eq61_e1355_d_b9];
        stamper.stamp_current_dense(
            Some(nodes[13]),
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
        let (eq62_e1362, eq62_e1362_d_n0, eq62_e1362_d_n1, eq62_e1362_d_n2, eq62_e1362_d_n3, eq62_e1362_d_n4, eq62_e1362_d_n5, eq62_e1362_d_n6, eq62_e1362_d_n7, eq62_e1362_d_n8, eq62_e1362_d_n9, eq62_e1362_d_n10, eq62_e1362_d_n11, eq62_e1362_d_n12, eq62_e1362_d_n13, eq62_e1362_d_n14, eq62_e1362_d_n15, eq62_e1362_d_n16, eq62_e1362_d_n17, eq62_e1362_d_n18, eq62_e1362_d_b0, eq62_e1362_d_b1, eq62_e1362_d_b2, eq62_e1362_d_b3, eq62_e1362_d_b4, eq62_e1362_d_b5, eq62_e1362_d_b6, eq62_e1362_d_b7, eq62_e1362_d_b8, eq62_e1362_d_b9,) = {
    if (p.p28 != 0.0) {
        let eq62_e1359: f64 = (s.v[800] * (nv12 - 0.0));
        let eq62_e1359_d_n0: f64 = (s.dn[800][0] * (nv12 - 0.0));
        let eq62_e1359_d_n1: f64 = (s.dn[800][1] * (nv12 - 0.0));
        let eq62_e1359_d_n2: f64 = (s.dn[800][2] * (nv12 - 0.0));
        let eq62_e1359_d_n3: f64 = (s.dn[800][3] * (nv12 - 0.0));
        let eq62_e1359_d_n4: f64 = (s.dn[800][4] * (nv12 - 0.0));
        let eq62_e1359_d_n5: f64 = (s.dn[800][5] * (nv12 - 0.0));
        let eq62_e1359_d_n6: f64 = (s.dn[800][6] * (nv12 - 0.0));
        let eq62_e1359_d_n7: f64 = (s.dn[800][7] * (nv12 - 0.0));
        let eq62_e1359_d_n8: f64 = (s.dn[800][8] * (nv12 - 0.0));
        let eq62_e1359_d_n9: f64 = (s.dn[800][9] * (nv12 - 0.0));
        let eq62_e1359_d_n10: f64 = (s.dn[800][10] * (nv12 - 0.0));
        let eq62_e1359_d_n11: f64 = (s.dn[800][11] * (nv12 - 0.0));
        let eq62_e1359_d_n12: f64 = ((s.dn[800][12] * (nv12 - 0.0)) + s.v[800]);
        let eq62_e1359_d_n13: f64 = (s.dn[800][13] * (nv12 - 0.0));
        let eq62_e1359_d_n14: f64 = (s.dn[800][14] * (nv12 - 0.0));
        let eq62_e1359_d_n15: f64 = (s.dn[800][15] * (nv12 - 0.0));
        let eq62_e1359_d_n16: f64 = (s.dn[800][16] * (nv12 - 0.0));
        let eq62_e1359_d_n17: f64 = (s.dn[800][17] * (nv12 - 0.0));
        let eq62_e1359_d_n18: f64 = (s.dn[800][18] * (nv12 - 0.0));
        let eq62_e1359_d_b0: f64 = (s.db[800][0] * (nv12 - 0.0));
        let eq62_e1359_d_b1: f64 = (s.db[800][1] * (nv12 - 0.0));
        let eq62_e1359_d_b2: f64 = (s.db[800][2] * (nv12 - 0.0));
        let eq62_e1359_d_b3: f64 = (s.db[800][3] * (nv12 - 0.0));
        let eq62_e1359_d_b4: f64 = (s.db[800][4] * (nv12 - 0.0));
        let eq62_e1359_d_b5: f64 = (s.db[800][5] * (nv12 - 0.0));
        let eq62_e1359_d_b6: f64 = (s.db[800][6] * (nv12 - 0.0));
        let eq62_e1359_d_b7: f64 = (s.db[800][7] * (nv12 - 0.0));
        let eq62_e1359_d_b8: f64 = (s.db[800][8] * (nv12 - 0.0));
        let eq62_e1359_d_b9: f64 = (s.db[800][9] * (nv12 - 0.0));
        let eq62_e1360: f64 = self.eval_ddt(18, eq62_e1359);
        let eq62_e1360_d_n0: f64 = self.ddt_jacobian(eq62_e1359_d_n0);
        let eq62_e1360_d_n1: f64 = self.ddt_jacobian(eq62_e1359_d_n1);
        let eq62_e1360_d_n2: f64 = self.ddt_jacobian(eq62_e1359_d_n2);
        let eq62_e1360_d_n3: f64 = self.ddt_jacobian(eq62_e1359_d_n3);
        let eq62_e1360_d_n4: f64 = self.ddt_jacobian(eq62_e1359_d_n4);
        let eq62_e1360_d_n5: f64 = self.ddt_jacobian(eq62_e1359_d_n5);
        let eq62_e1360_d_n6: f64 = self.ddt_jacobian(eq62_e1359_d_n6);
        let eq62_e1360_d_n7: f64 = self.ddt_jacobian(eq62_e1359_d_n7);
        let eq62_e1360_d_n8: f64 = self.ddt_jacobian(eq62_e1359_d_n8);
        let eq62_e1360_d_n9: f64 = self.ddt_jacobian(eq62_e1359_d_n9);
        let eq62_e1360_d_n10: f64 = self.ddt_jacobian(eq62_e1359_d_n10);
        let eq62_e1360_d_n11: f64 = self.ddt_jacobian(eq62_e1359_d_n11);
        let eq62_e1360_d_n12: f64 = self.ddt_jacobian(eq62_e1359_d_n12);
        let eq62_e1360_d_n13: f64 = self.ddt_jacobian(eq62_e1359_d_n13);
        let eq62_e1360_d_n14: f64 = self.ddt_jacobian(eq62_e1359_d_n14);
        let eq62_e1360_d_n15: f64 = self.ddt_jacobian(eq62_e1359_d_n15);
        let eq62_e1360_d_n16: f64 = self.ddt_jacobian(eq62_e1359_d_n16);
        let eq62_e1360_d_n17: f64 = self.ddt_jacobian(eq62_e1359_d_n17);
        let eq62_e1360_d_n18: f64 = self.ddt_jacobian(eq62_e1359_d_n18);
        let eq62_e1360_d_b0: f64 = self.ddt_jacobian(eq62_e1359_d_b0);
        let eq62_e1360_d_b1: f64 = self.ddt_jacobian(eq62_e1359_d_b1);
        let eq62_e1360_d_b2: f64 = self.ddt_jacobian(eq62_e1359_d_b2);
        let eq62_e1360_d_b3: f64 = self.ddt_jacobian(eq62_e1359_d_b3);
        let eq62_e1360_d_b4: f64 = self.ddt_jacobian(eq62_e1359_d_b4);
        let eq62_e1360_d_b5: f64 = self.ddt_jacobian(eq62_e1359_d_b5);
        let eq62_e1360_d_b6: f64 = self.ddt_jacobian(eq62_e1359_d_b6);
        let eq62_e1360_d_b7: f64 = self.ddt_jacobian(eq62_e1359_d_b7);
        let eq62_e1360_d_b8: f64 = self.ddt_jacobian(eq62_e1359_d_b8);
        let eq62_e1360_d_b9: f64 = self.ddt_jacobian(eq62_e1359_d_b9);
        (eq62_e1360, eq62_e1360_d_n0, eq62_e1360_d_n1, eq62_e1360_d_n2, eq62_e1360_d_n3, eq62_e1360_d_n4, eq62_e1360_d_n5, eq62_e1360_d_n6, eq62_e1360_d_n7, eq62_e1360_d_n8, eq62_e1360_d_n9, eq62_e1360_d_n10, eq62_e1360_d_n11, eq62_e1360_d_n12, eq62_e1360_d_n13, eq62_e1360_d_n14, eq62_e1360_d_n15, eq62_e1360_d_n16, eq62_e1360_d_n17, eq62_e1360_d_n18, eq62_e1360_d_b0, eq62_e1360_d_b1, eq62_e1360_d_b2, eq62_e1360_d_b3, eq62_e1360_d_b4, eq62_e1360_d_b5, eq62_e1360_d_b6, eq62_e1360_d_b7, eq62_e1360_d_b8, eq62_e1360_d_b9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e1362;
        let eq62_node_derivatives: [f64; 19] = [eq62_e1362_d_n0, eq62_e1362_d_n1, eq62_e1362_d_n2, eq62_e1362_d_n3, eq62_e1362_d_n4, eq62_e1362_d_n5, eq62_e1362_d_n6, eq62_e1362_d_n7, eq62_e1362_d_n8, eq62_e1362_d_n9, eq62_e1362_d_n10, eq62_e1362_d_n11, eq62_e1362_d_n12, eq62_e1362_d_n13, eq62_e1362_d_n14, eq62_e1362_d_n15, eq62_e1362_d_n16, eq62_e1362_d_n17, eq62_e1362_d_n18];
        let eq62_branch_derivatives: [f64; 10] = [eq62_e1362_d_b0, eq62_e1362_d_b1, eq62_e1362_d_b2, eq62_e1362_d_b3, eq62_e1362_d_b4, eq62_e1362_d_b5, eq62_e1362_d_b6, eq62_e1362_d_b7, eq62_e1362_d_b8, eq62_e1362_d_b9];
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
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq63_e1369, eq63_e1369_d_n0, eq63_e1369_d_n1, eq63_e1369_d_n2, eq63_e1369_d_n3, eq63_e1369_d_n4, eq63_e1369_d_n5, eq63_e1369_d_n6, eq63_e1369_d_n7, eq63_e1369_d_n8, eq63_e1369_d_n9, eq63_e1369_d_n10, eq63_e1369_d_n11, eq63_e1369_d_n12, eq63_e1369_d_n13, eq63_e1369_d_n14, eq63_e1369_d_n15, eq63_e1369_d_n16, eq63_e1369_d_n17, eq63_e1369_d_n18, eq63_e1369_d_b0, eq63_e1369_d_b1, eq63_e1369_d_b2, eq63_e1369_d_b3, eq63_e1369_d_b4, eq63_e1369_d_b5, eq63_e1369_d_b6, eq63_e1369_d_b7, eq63_e1369_d_b8, eq63_e1369_d_b9,) = {
    if (p.p28 != 0.0) {
        let eq63_e1366: f64 = (s.v[801] * (nv13 - 0.0));
        let eq63_e1366_d_n0: f64 = (s.dn[801][0] * (nv13 - 0.0));
        let eq63_e1366_d_n1: f64 = (s.dn[801][1] * (nv13 - 0.0));
        let eq63_e1366_d_n2: f64 = (s.dn[801][2] * (nv13 - 0.0));
        let eq63_e1366_d_n3: f64 = (s.dn[801][3] * (nv13 - 0.0));
        let eq63_e1366_d_n4: f64 = (s.dn[801][4] * (nv13 - 0.0));
        let eq63_e1366_d_n5: f64 = (s.dn[801][5] * (nv13 - 0.0));
        let eq63_e1366_d_n6: f64 = (s.dn[801][6] * (nv13 - 0.0));
        let eq63_e1366_d_n7: f64 = (s.dn[801][7] * (nv13 - 0.0));
        let eq63_e1366_d_n8: f64 = (s.dn[801][8] * (nv13 - 0.0));
        let eq63_e1366_d_n9: f64 = (s.dn[801][9] * (nv13 - 0.0));
        let eq63_e1366_d_n10: f64 = (s.dn[801][10] * (nv13 - 0.0));
        let eq63_e1366_d_n11: f64 = (s.dn[801][11] * (nv13 - 0.0));
        let eq63_e1366_d_n12: f64 = (s.dn[801][12] * (nv13 - 0.0));
        let eq63_e1366_d_n13: f64 = ((s.dn[801][13] * (nv13 - 0.0)) + s.v[801]);
        let eq63_e1366_d_n14: f64 = (s.dn[801][14] * (nv13 - 0.0));
        let eq63_e1366_d_n15: f64 = (s.dn[801][15] * (nv13 - 0.0));
        let eq63_e1366_d_n16: f64 = (s.dn[801][16] * (nv13 - 0.0));
        let eq63_e1366_d_n17: f64 = (s.dn[801][17] * (nv13 - 0.0));
        let eq63_e1366_d_n18: f64 = (s.dn[801][18] * (nv13 - 0.0));
        let eq63_e1366_d_b0: f64 = (s.db[801][0] * (nv13 - 0.0));
        let eq63_e1366_d_b1: f64 = (s.db[801][1] * (nv13 - 0.0));
        let eq63_e1366_d_b2: f64 = (s.db[801][2] * (nv13 - 0.0));
        let eq63_e1366_d_b3: f64 = (s.db[801][3] * (nv13 - 0.0));
        let eq63_e1366_d_b4: f64 = (s.db[801][4] * (nv13 - 0.0));
        let eq63_e1366_d_b5: f64 = (s.db[801][5] * (nv13 - 0.0));
        let eq63_e1366_d_b6: f64 = (s.db[801][6] * (nv13 - 0.0));
        let eq63_e1366_d_b7: f64 = (s.db[801][7] * (nv13 - 0.0));
        let eq63_e1366_d_b8: f64 = (s.db[801][8] * (nv13 - 0.0));
        let eq63_e1366_d_b9: f64 = (s.db[801][9] * (nv13 - 0.0));
        let eq63_e1367: f64 = self.eval_ddt(19, eq63_e1366);
        let eq63_e1367_d_n0: f64 = self.ddt_jacobian(eq63_e1366_d_n0);
        let eq63_e1367_d_n1: f64 = self.ddt_jacobian(eq63_e1366_d_n1);
        let eq63_e1367_d_n2: f64 = self.ddt_jacobian(eq63_e1366_d_n2);
        let eq63_e1367_d_n3: f64 = self.ddt_jacobian(eq63_e1366_d_n3);
        let eq63_e1367_d_n4: f64 = self.ddt_jacobian(eq63_e1366_d_n4);
        let eq63_e1367_d_n5: f64 = self.ddt_jacobian(eq63_e1366_d_n5);
        let eq63_e1367_d_n6: f64 = self.ddt_jacobian(eq63_e1366_d_n6);
        let eq63_e1367_d_n7: f64 = self.ddt_jacobian(eq63_e1366_d_n7);
        let eq63_e1367_d_n8: f64 = self.ddt_jacobian(eq63_e1366_d_n8);
        let eq63_e1367_d_n9: f64 = self.ddt_jacobian(eq63_e1366_d_n9);
        let eq63_e1367_d_n10: f64 = self.ddt_jacobian(eq63_e1366_d_n10);
        let eq63_e1367_d_n11: f64 = self.ddt_jacobian(eq63_e1366_d_n11);
        let eq63_e1367_d_n12: f64 = self.ddt_jacobian(eq63_e1366_d_n12);
        let eq63_e1367_d_n13: f64 = self.ddt_jacobian(eq63_e1366_d_n13);
        let eq63_e1367_d_n14: f64 = self.ddt_jacobian(eq63_e1366_d_n14);
        let eq63_e1367_d_n15: f64 = self.ddt_jacobian(eq63_e1366_d_n15);
        let eq63_e1367_d_n16: f64 = self.ddt_jacobian(eq63_e1366_d_n16);
        let eq63_e1367_d_n17: f64 = self.ddt_jacobian(eq63_e1366_d_n17);
        let eq63_e1367_d_n18: f64 = self.ddt_jacobian(eq63_e1366_d_n18);
        let eq63_e1367_d_b0: f64 = self.ddt_jacobian(eq63_e1366_d_b0);
        let eq63_e1367_d_b1: f64 = self.ddt_jacobian(eq63_e1366_d_b1);
        let eq63_e1367_d_b2: f64 = self.ddt_jacobian(eq63_e1366_d_b2);
        let eq63_e1367_d_b3: f64 = self.ddt_jacobian(eq63_e1366_d_b3);
        let eq63_e1367_d_b4: f64 = self.ddt_jacobian(eq63_e1366_d_b4);
        let eq63_e1367_d_b5: f64 = self.ddt_jacobian(eq63_e1366_d_b5);
        let eq63_e1367_d_b6: f64 = self.ddt_jacobian(eq63_e1366_d_b6);
        let eq63_e1367_d_b7: f64 = self.ddt_jacobian(eq63_e1366_d_b7);
        let eq63_e1367_d_b8: f64 = self.ddt_jacobian(eq63_e1366_d_b8);
        let eq63_e1367_d_b9: f64 = self.ddt_jacobian(eq63_e1366_d_b9);
        (eq63_e1367, eq63_e1367_d_n0, eq63_e1367_d_n1, eq63_e1367_d_n2, eq63_e1367_d_n3, eq63_e1367_d_n4, eq63_e1367_d_n5, eq63_e1367_d_n6, eq63_e1367_d_n7, eq63_e1367_d_n8, eq63_e1367_d_n9, eq63_e1367_d_n10, eq63_e1367_d_n11, eq63_e1367_d_n12, eq63_e1367_d_n13, eq63_e1367_d_n14, eq63_e1367_d_n15, eq63_e1367_d_n16, eq63_e1367_d_n17, eq63_e1367_d_n18, eq63_e1367_d_b0, eq63_e1367_d_b1, eq63_e1367_d_b2, eq63_e1367_d_b3, eq63_e1367_d_b4, eq63_e1367_d_b5, eq63_e1367_d_b6, eq63_e1367_d_b7, eq63_e1367_d_b8, eq63_e1367_d_b9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e1369;
        let eq63_node_derivatives: [f64; 19] = [eq63_e1369_d_n0, eq63_e1369_d_n1, eq63_e1369_d_n2, eq63_e1369_d_n3, eq63_e1369_d_n4, eq63_e1369_d_n5, eq63_e1369_d_n6, eq63_e1369_d_n7, eq63_e1369_d_n8, eq63_e1369_d_n9, eq63_e1369_d_n10, eq63_e1369_d_n11, eq63_e1369_d_n12, eq63_e1369_d_n13, eq63_e1369_d_n14, eq63_e1369_d_n15, eq63_e1369_d_n16, eq63_e1369_d_n17, eq63_e1369_d_n18];
        let eq63_branch_derivatives: [f64; 10] = [eq63_e1369_d_b0, eq63_e1369_d_b1, eq63_e1369_d_b2, eq63_e1369_d_b3, eq63_e1369_d_b4, eq63_e1369_d_b5, eq63_e1369_d_b6, eq63_e1369_d_b7, eq63_e1369_d_b8, eq63_e1369_d_b9];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            None,
            self.multiplicity * (eq63_value),
            &nodes,
            &eq63_node_derivatives,
            &branches,
            &eq63_branch_derivatives,
            self.multiplicity,
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
        let (eq64_e1374,) = {
    if (!(p.p28 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq64_value: f64 = eq64_e1374;
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
        let (eq65_e1379,) = {
    if (!(p.p28 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq65_value: f64 = eq65_e1379;
        stamper.stamp_potential(
            branches[11],
            eq65_value,
            &[
            ],
        );
    }
}
