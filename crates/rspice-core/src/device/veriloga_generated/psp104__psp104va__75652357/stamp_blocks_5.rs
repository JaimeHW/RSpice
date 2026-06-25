#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_15_block_0(
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq15_e1085, eq15_e1085_d_n0, eq15_e1085_d_n1, eq15_e1085_d_n2, eq15_e1085_d_n3, eq15_e1085_d_n4, eq15_e1085_d_n5, eq15_e1085_d_n6, eq15_e1085_d_n7, eq15_e1085_d_n8, eq15_e1085_d_n9, eq15_e1085_d_n10, eq15_e1085_d_n11,) = {
    if (s.v[2702] != 0.0) {
        let eq15_e1079: f64 = (s.v[15] * p.p32);
        let eq15_e1079_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq15_e1079_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq15_e1079_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq15_e1079_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq15_e1079_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq15_e1079_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq15_e1079_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq15_e1079_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq15_e1079_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq15_e1079_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq15_e1079_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq15_e1079_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq15_e1081: f64 = (eq15_e1079 * s.v[805]);
        let eq15_e1081_d_n0: f64 = ((eq15_e1079_d_n0 * s.v[805]) + (eq15_e1079 * s.dn[805][0]));
        let eq15_e1081_d_n1: f64 = ((eq15_e1079_d_n1 * s.v[805]) + (eq15_e1079 * s.dn[805][1]));
        let eq15_e1081_d_n2: f64 = ((eq15_e1079_d_n2 * s.v[805]) + (eq15_e1079 * s.dn[805][2]));
        let eq15_e1081_d_n3: f64 = ((eq15_e1079_d_n3 * s.v[805]) + (eq15_e1079 * s.dn[805][3]));
        let eq15_e1081_d_n4: f64 = ((eq15_e1079_d_n4 * s.v[805]) + (eq15_e1079 * s.dn[805][4]));
        let eq15_e1081_d_n5: f64 = ((eq15_e1079_d_n5 * s.v[805]) + (eq15_e1079 * s.dn[805][5]));
        let eq15_e1081_d_n6: f64 = ((eq15_e1079_d_n6 * s.v[805]) + (eq15_e1079 * s.dn[805][6]));
        let eq15_e1081_d_n7: f64 = ((eq15_e1079_d_n7 * s.v[805]) + (eq15_e1079 * s.dn[805][7]));
        let eq15_e1081_d_n8: f64 = ((eq15_e1079_d_n8 * s.v[805]) + (eq15_e1079 * s.dn[805][8]));
        let eq15_e1081_d_n9: f64 = ((eq15_e1079_d_n9 * s.v[805]) + (eq15_e1079 * s.dn[805][9]));
        let eq15_e1081_d_n10: f64 = ((eq15_e1079_d_n10 * s.v[805]) + (eq15_e1079 * s.dn[805][10]));
        let eq15_e1081_d_n11: f64 = ((eq15_e1079_d_n11 * s.v[805]) + (eq15_e1079 * s.dn[805][11]));
        let eq15_e1083: f64 = (eq15_e1081 * (nv1 - nv5));
        let eq15_e1083_d_n0: f64 = (eq15_e1081_d_n0 * (nv1 - nv5));
        let eq15_e1083_d_n1: f64 = ((eq15_e1081_d_n1 * (nv1 - nv5)) + eq15_e1081);
        let eq15_e1083_d_n2: f64 = (eq15_e1081_d_n2 * (nv1 - nv5));
        let eq15_e1083_d_n3: f64 = (eq15_e1081_d_n3 * (nv1 - nv5));
        let eq15_e1083_d_n4: f64 = (eq15_e1081_d_n4 * (nv1 - nv5));
        let eq15_e1083_d_n5: f64 = ((eq15_e1081_d_n5 * (nv1 - nv5)) + (-eq15_e1081));
        let eq15_e1083_d_n6: f64 = (eq15_e1081_d_n6 * (nv1 - nv5));
        let eq15_e1083_d_n7: f64 = (eq15_e1081_d_n7 * (nv1 - nv5));
        let eq15_e1083_d_n8: f64 = (eq15_e1081_d_n8 * (nv1 - nv5));
        let eq15_e1083_d_n9: f64 = (eq15_e1081_d_n9 * (nv1 - nv5));
        let eq15_e1083_d_n10: f64 = (eq15_e1081_d_n10 * (nv1 - nv5));
        let eq15_e1083_d_n11: f64 = (eq15_e1081_d_n11 * (nv1 - nv5));
        (eq15_e1083, eq15_e1083_d_n0, eq15_e1083_d_n1, eq15_e1083_d_n2, eq15_e1083_d_n3, eq15_e1083_d_n4, eq15_e1083_d_n5, eq15_e1083_d_n6, eq15_e1083_d_n7, eq15_e1083_d_n8, eq15_e1083_d_n9, eq15_e1083_d_n10, eq15_e1083_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e1085;
        let eq15_node_derivatives: [f64; 12] = [eq15_e1085_d_n0, eq15_e1085_d_n1, eq15_e1085_d_n2, eq15_e1085_d_n3, eq15_e1085_d_n4, eq15_e1085_d_n5, eq15_e1085_d_n6, eq15_e1085_d_n7, eq15_e1085_d_n8, eq15_e1085_d_n9, eq15_e1085_d_n10, eq15_e1085_d_n11];
        let eq15_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[5]),
            self.multiplicity * (eq15_value),
            &nodes,
            &eq15_node_derivatives,
            &branches,
            &eq15_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_16_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq16_e1095,) = {
    if (s.v[2702] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq16_value: f64 = eq16_e1095;
        stamper.stamp_current(
            Some(nodes[1]),
            Some(nodes[5]),
            self.multiplicity * (eq16_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_17_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq17_e1100,) = {
    if (!(s.v[2702] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq17_value: f64 = eq17_e1100;
        stamper.stamp_potential(
            branches[0],
            eq17_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_18_block_0(
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
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq18_e1110, eq18_e1110_d_n0, eq18_e1110_d_n1, eq18_e1110_d_n2, eq18_e1110_d_n3, eq18_e1110_d_n4, eq18_e1110_d_n5, eq18_e1110_d_n6, eq18_e1110_d_n7, eq18_e1110_d_n8, eq18_e1110_d_n9, eq18_e1110_d_n10, eq18_e1110_d_n11,) = {
    if (s.v[2703] != 0.0) {
        let eq18_e1104: f64 = (s.v[15] * p.p32);
        let eq18_e1104_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq18_e1104_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq18_e1104_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq18_e1104_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq18_e1104_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq18_e1104_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq18_e1104_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq18_e1104_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq18_e1104_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq18_e1104_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq18_e1104_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq18_e1104_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq18_e1106: f64 = (eq18_e1104 * s.v[806]);
        let eq18_e1106_d_n0: f64 = ((eq18_e1104_d_n0 * s.v[806]) + (eq18_e1104 * s.dn[806][0]));
        let eq18_e1106_d_n1: f64 = ((eq18_e1104_d_n1 * s.v[806]) + (eq18_e1104 * s.dn[806][1]));
        let eq18_e1106_d_n2: f64 = ((eq18_e1104_d_n2 * s.v[806]) + (eq18_e1104 * s.dn[806][2]));
        let eq18_e1106_d_n3: f64 = ((eq18_e1104_d_n3 * s.v[806]) + (eq18_e1104 * s.dn[806][3]));
        let eq18_e1106_d_n4: f64 = ((eq18_e1104_d_n4 * s.v[806]) + (eq18_e1104 * s.dn[806][4]));
        let eq18_e1106_d_n5: f64 = ((eq18_e1104_d_n5 * s.v[806]) + (eq18_e1104 * s.dn[806][5]));
        let eq18_e1106_d_n6: f64 = ((eq18_e1104_d_n6 * s.v[806]) + (eq18_e1104 * s.dn[806][6]));
        let eq18_e1106_d_n7: f64 = ((eq18_e1104_d_n7 * s.v[806]) + (eq18_e1104 * s.dn[806][7]));
        let eq18_e1106_d_n8: f64 = ((eq18_e1104_d_n8 * s.v[806]) + (eq18_e1104 * s.dn[806][8]));
        let eq18_e1106_d_n9: f64 = ((eq18_e1104_d_n9 * s.v[806]) + (eq18_e1104 * s.dn[806][9]));
        let eq18_e1106_d_n10: f64 = ((eq18_e1104_d_n10 * s.v[806]) + (eq18_e1104 * s.dn[806][10]));
        let eq18_e1106_d_n11: f64 = ((eq18_e1104_d_n11 * s.v[806]) + (eq18_e1104 * s.dn[806][11]));
        let eq18_e1108: f64 = (eq18_e1106 * (nv2 - nv6));
        let eq18_e1108_d_n0: f64 = (eq18_e1106_d_n0 * (nv2 - nv6));
        let eq18_e1108_d_n1: f64 = (eq18_e1106_d_n1 * (nv2 - nv6));
        let eq18_e1108_d_n2: f64 = ((eq18_e1106_d_n2 * (nv2 - nv6)) + eq18_e1106);
        let eq18_e1108_d_n3: f64 = (eq18_e1106_d_n3 * (nv2 - nv6));
        let eq18_e1108_d_n4: f64 = (eq18_e1106_d_n4 * (nv2 - nv6));
        let eq18_e1108_d_n5: f64 = (eq18_e1106_d_n5 * (nv2 - nv6));
        let eq18_e1108_d_n6: f64 = ((eq18_e1106_d_n6 * (nv2 - nv6)) + (-eq18_e1106));
        let eq18_e1108_d_n7: f64 = (eq18_e1106_d_n7 * (nv2 - nv6));
        let eq18_e1108_d_n8: f64 = (eq18_e1106_d_n8 * (nv2 - nv6));
        let eq18_e1108_d_n9: f64 = (eq18_e1106_d_n9 * (nv2 - nv6));
        let eq18_e1108_d_n10: f64 = (eq18_e1106_d_n10 * (nv2 - nv6));
        let eq18_e1108_d_n11: f64 = (eq18_e1106_d_n11 * (nv2 - nv6));
        (eq18_e1108, eq18_e1108_d_n0, eq18_e1108_d_n1, eq18_e1108_d_n2, eq18_e1108_d_n3, eq18_e1108_d_n4, eq18_e1108_d_n5, eq18_e1108_d_n6, eq18_e1108_d_n7, eq18_e1108_d_n8, eq18_e1108_d_n9, eq18_e1108_d_n10, eq18_e1108_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1110;
        let eq18_node_derivatives: [f64; 12] = [eq18_e1110_d_n0, eq18_e1110_d_n1, eq18_e1110_d_n2, eq18_e1110_d_n3, eq18_e1110_d_n4, eq18_e1110_d_n5, eq18_e1110_d_n6, eq18_e1110_d_n7, eq18_e1110_d_n8, eq18_e1110_d_n9, eq18_e1110_d_n10, eq18_e1110_d_n11];
        let eq18_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[6]),
            self.multiplicity * (eq18_value),
            &nodes,
            &eq18_node_derivatives,
            &branches,
            &eq18_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_19_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq19_e1120,) = {
    if (s.v[2703] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e1120;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[6]),
            self.multiplicity * (eq19_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_20_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq20_e1125,) = {
    if (!(s.v[2703] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e1125;
        stamper.stamp_potential(
            branches[1],
            eq20_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_21_block_0(
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq21_e1135, eq21_e1135_d_n0, eq21_e1135_d_n1, eq21_e1135_d_n2, eq21_e1135_d_n3, eq21_e1135_d_n4, eq21_e1135_d_n5, eq21_e1135_d_n6, eq21_e1135_d_n7, eq21_e1135_d_n8, eq21_e1135_d_n9, eq21_e1135_d_n10, eq21_e1135_d_n11,) = {
    if (s.v[2704] != 0.0) {
        let eq21_e1129: f64 = (s.v[15] * p.p32);
        let eq21_e1129_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq21_e1129_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq21_e1129_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq21_e1129_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq21_e1129_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq21_e1129_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq21_e1129_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq21_e1129_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq21_e1129_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq21_e1129_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq21_e1129_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq21_e1129_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq21_e1131: f64 = (eq21_e1129 * s.v[807]);
        let eq21_e1131_d_n0: f64 = ((eq21_e1129_d_n0 * s.v[807]) + (eq21_e1129 * s.dn[807][0]));
        let eq21_e1131_d_n1: f64 = ((eq21_e1129_d_n1 * s.v[807]) + (eq21_e1129 * s.dn[807][1]));
        let eq21_e1131_d_n2: f64 = ((eq21_e1129_d_n2 * s.v[807]) + (eq21_e1129 * s.dn[807][2]));
        let eq21_e1131_d_n3: f64 = ((eq21_e1129_d_n3 * s.v[807]) + (eq21_e1129 * s.dn[807][3]));
        let eq21_e1131_d_n4: f64 = ((eq21_e1129_d_n4 * s.v[807]) + (eq21_e1129 * s.dn[807][4]));
        let eq21_e1131_d_n5: f64 = ((eq21_e1129_d_n5 * s.v[807]) + (eq21_e1129 * s.dn[807][5]));
        let eq21_e1131_d_n6: f64 = ((eq21_e1129_d_n6 * s.v[807]) + (eq21_e1129 * s.dn[807][6]));
        let eq21_e1131_d_n7: f64 = ((eq21_e1129_d_n7 * s.v[807]) + (eq21_e1129 * s.dn[807][7]));
        let eq21_e1131_d_n8: f64 = ((eq21_e1129_d_n8 * s.v[807]) + (eq21_e1129 * s.dn[807][8]));
        let eq21_e1131_d_n9: f64 = ((eq21_e1129_d_n9 * s.v[807]) + (eq21_e1129 * s.dn[807][9]));
        let eq21_e1131_d_n10: f64 = ((eq21_e1129_d_n10 * s.v[807]) + (eq21_e1129 * s.dn[807][10]));
        let eq21_e1131_d_n11: f64 = ((eq21_e1129_d_n11 * s.v[807]) + (eq21_e1129 * s.dn[807][11]));
        let eq21_e1133: f64 = (eq21_e1131 * (nv0 - nv7));
        let eq21_e1133_d_n0: f64 = ((eq21_e1131_d_n0 * (nv0 - nv7)) + eq21_e1131);
        let eq21_e1133_d_n1: f64 = (eq21_e1131_d_n1 * (nv0 - nv7));
        let eq21_e1133_d_n2: f64 = (eq21_e1131_d_n2 * (nv0 - nv7));
        let eq21_e1133_d_n3: f64 = (eq21_e1131_d_n3 * (nv0 - nv7));
        let eq21_e1133_d_n4: f64 = (eq21_e1131_d_n4 * (nv0 - nv7));
        let eq21_e1133_d_n5: f64 = (eq21_e1131_d_n5 * (nv0 - nv7));
        let eq21_e1133_d_n6: f64 = (eq21_e1131_d_n6 * (nv0 - nv7));
        let eq21_e1133_d_n7: f64 = ((eq21_e1131_d_n7 * (nv0 - nv7)) + (-eq21_e1131));
        let eq21_e1133_d_n8: f64 = (eq21_e1131_d_n8 * (nv0 - nv7));
        let eq21_e1133_d_n9: f64 = (eq21_e1131_d_n9 * (nv0 - nv7));
        let eq21_e1133_d_n10: f64 = (eq21_e1131_d_n10 * (nv0 - nv7));
        let eq21_e1133_d_n11: f64 = (eq21_e1131_d_n11 * (nv0 - nv7));
        (eq21_e1133, eq21_e1133_d_n0, eq21_e1133_d_n1, eq21_e1133_d_n2, eq21_e1133_d_n3, eq21_e1133_d_n4, eq21_e1133_d_n5, eq21_e1133_d_n6, eq21_e1133_d_n7, eq21_e1133_d_n8, eq21_e1133_d_n9, eq21_e1133_d_n10, eq21_e1133_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1135;
        let eq21_node_derivatives: [f64; 12] = [eq21_e1135_d_n0, eq21_e1135_d_n1, eq21_e1135_d_n2, eq21_e1135_d_n3, eq21_e1135_d_n4, eq21_e1135_d_n5, eq21_e1135_d_n6, eq21_e1135_d_n7, eq21_e1135_d_n8, eq21_e1135_d_n9, eq21_e1135_d_n10, eq21_e1135_d_n11];
        let eq21_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[7]),
            self.multiplicity * (eq21_value),
            &nodes,
            &eq21_node_derivatives,
            &branches,
            &eq21_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_22_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq22_e1145,) = {
    if (s.v[2704] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e1145;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[7]),
            self.multiplicity * (eq22_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_23_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq23_e1150,) = {
    if (!(s.v[2704] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e1150;
        stamper.stamp_potential(
            branches[2],
            eq23_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_24_block_0(
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
        let (eq24_e1160, eq24_e1160_d_n0, eq24_e1160_d_n1, eq24_e1160_d_n2, eq24_e1160_d_n3, eq24_e1160_d_n4, eq24_e1160_d_n5, eq24_e1160_d_n6, eq24_e1160_d_n7, eq24_e1160_d_n8, eq24_e1160_d_n9, eq24_e1160_d_n10, eq24_e1160_d_n11,) = {
    if (s.v[2705] != 0.0) {
        let eq24_e1154: f64 = (s.v[15] * p.p32);
        let eq24_e1154_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq24_e1154_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq24_e1154_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq24_e1154_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq24_e1154_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq24_e1154_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq24_e1154_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq24_e1154_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq24_e1154_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq24_e1154_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq24_e1154_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq24_e1154_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq24_e1156: f64 = (eq24_e1154 * s.v[808]);
        let eq24_e1156_d_n0: f64 = ((eq24_e1154_d_n0 * s.v[808]) + (eq24_e1154 * s.dn[808][0]));
        let eq24_e1156_d_n1: f64 = ((eq24_e1154_d_n1 * s.v[808]) + (eq24_e1154 * s.dn[808][1]));
        let eq24_e1156_d_n2: f64 = ((eq24_e1154_d_n2 * s.v[808]) + (eq24_e1154 * s.dn[808][2]));
        let eq24_e1156_d_n3: f64 = ((eq24_e1154_d_n3 * s.v[808]) + (eq24_e1154 * s.dn[808][3]));
        let eq24_e1156_d_n4: f64 = ((eq24_e1154_d_n4 * s.v[808]) + (eq24_e1154 * s.dn[808][4]));
        let eq24_e1156_d_n5: f64 = ((eq24_e1154_d_n5 * s.v[808]) + (eq24_e1154 * s.dn[808][5]));
        let eq24_e1156_d_n6: f64 = ((eq24_e1154_d_n6 * s.v[808]) + (eq24_e1154 * s.dn[808][6]));
        let eq24_e1156_d_n7: f64 = ((eq24_e1154_d_n7 * s.v[808]) + (eq24_e1154 * s.dn[808][7]));
        let eq24_e1156_d_n8: f64 = ((eq24_e1154_d_n8 * s.v[808]) + (eq24_e1154 * s.dn[808][8]));
        let eq24_e1156_d_n9: f64 = ((eq24_e1154_d_n9 * s.v[808]) + (eq24_e1154 * s.dn[808][9]));
        let eq24_e1156_d_n10: f64 = ((eq24_e1154_d_n10 * s.v[808]) + (eq24_e1154 * s.dn[808][10]));
        let eq24_e1156_d_n11: f64 = ((eq24_e1154_d_n11 * s.v[808]) + (eq24_e1154 * s.dn[808][11]));
        let eq24_e1158: f64 = (eq24_e1156 * (nv8 - nv9));
        let eq24_e1158_d_n0: f64 = (eq24_e1156_d_n0 * (nv8 - nv9));
        let eq24_e1158_d_n1: f64 = (eq24_e1156_d_n1 * (nv8 - nv9));
        let eq24_e1158_d_n2: f64 = (eq24_e1156_d_n2 * (nv8 - nv9));
        let eq24_e1158_d_n3: f64 = (eq24_e1156_d_n3 * (nv8 - nv9));
        let eq24_e1158_d_n4: f64 = (eq24_e1156_d_n4 * (nv8 - nv9));
        let eq24_e1158_d_n5: f64 = (eq24_e1156_d_n5 * (nv8 - nv9));
        let eq24_e1158_d_n6: f64 = (eq24_e1156_d_n6 * (nv8 - nv9));
        let eq24_e1158_d_n7: f64 = (eq24_e1156_d_n7 * (nv8 - nv9));
        let eq24_e1158_d_n8: f64 = ((eq24_e1156_d_n8 * (nv8 - nv9)) + eq24_e1156);
        let eq24_e1158_d_n9: f64 = ((eq24_e1156_d_n9 * (nv8 - nv9)) + (-eq24_e1156));
        let eq24_e1158_d_n10: f64 = (eq24_e1156_d_n10 * (nv8 - nv9));
        let eq24_e1158_d_n11: f64 = (eq24_e1156_d_n11 * (nv8 - nv9));
        (eq24_e1158, eq24_e1158_d_n0, eq24_e1158_d_n1, eq24_e1158_d_n2, eq24_e1158_d_n3, eq24_e1158_d_n4, eq24_e1158_d_n5, eq24_e1158_d_n6, eq24_e1158_d_n7, eq24_e1158_d_n8, eq24_e1158_d_n9, eq24_e1158_d_n10, eq24_e1158_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1160;
        let eq24_node_derivatives: [f64; 12] = [eq24_e1160_d_n0, eq24_e1160_d_n1, eq24_e1160_d_n2, eq24_e1160_d_n3, eq24_e1160_d_n4, eq24_e1160_d_n5, eq24_e1160_d_n6, eq24_e1160_d_n7, eq24_e1160_d_n8, eq24_e1160_d_n9, eq24_e1160_d_n10, eq24_e1160_d_n11];
        let eq24_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[9]),
            self.multiplicity * (eq24_value),
            &nodes,
            &eq24_node_derivatives,
            &branches,
            &eq24_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_25_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq25_e1170,) = {
    if (s.v[2705] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq25_value: f64 = eq25_e1170;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[9]),
            self.multiplicity * (eq25_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_26_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq26_e1175,) = {
    if (!(s.v[2705] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e1175;
        stamper.stamp_potential(
            branches[3],
            eq26_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_27_block_0(
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
        let (eq27_e1185, eq27_e1185_d_n0, eq27_e1185_d_n1, eq27_e1185_d_n2, eq27_e1185_d_n3, eq27_e1185_d_n4, eq27_e1185_d_n5, eq27_e1185_d_n6, eq27_e1185_d_n7, eq27_e1185_d_n8, eq27_e1185_d_n9, eq27_e1185_d_n10, eq27_e1185_d_n11,) = {
    if (s.v[2706] != 0.0) {
        let eq27_e1179: f64 = (s.v[15] * p.p32);
        let eq27_e1179_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq27_e1179_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq27_e1179_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq27_e1179_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq27_e1179_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq27_e1179_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq27_e1179_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq27_e1179_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq27_e1179_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq27_e1179_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq27_e1179_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq27_e1179_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq27_e1181: f64 = (eq27_e1179 * s.v[809]);
        let eq27_e1181_d_n0: f64 = ((eq27_e1179_d_n0 * s.v[809]) + (eq27_e1179 * s.dn[809][0]));
        let eq27_e1181_d_n1: f64 = ((eq27_e1179_d_n1 * s.v[809]) + (eq27_e1179 * s.dn[809][1]));
        let eq27_e1181_d_n2: f64 = ((eq27_e1179_d_n2 * s.v[809]) + (eq27_e1179 * s.dn[809][2]));
        let eq27_e1181_d_n3: f64 = ((eq27_e1179_d_n3 * s.v[809]) + (eq27_e1179 * s.dn[809][3]));
        let eq27_e1181_d_n4: f64 = ((eq27_e1179_d_n4 * s.v[809]) + (eq27_e1179 * s.dn[809][4]));
        let eq27_e1181_d_n5: f64 = ((eq27_e1179_d_n5 * s.v[809]) + (eq27_e1179 * s.dn[809][5]));
        let eq27_e1181_d_n6: f64 = ((eq27_e1179_d_n6 * s.v[809]) + (eq27_e1179 * s.dn[809][6]));
        let eq27_e1181_d_n7: f64 = ((eq27_e1179_d_n7 * s.v[809]) + (eq27_e1179 * s.dn[809][7]));
        let eq27_e1181_d_n8: f64 = ((eq27_e1179_d_n8 * s.v[809]) + (eq27_e1179 * s.dn[809][8]));
        let eq27_e1181_d_n9: f64 = ((eq27_e1179_d_n9 * s.v[809]) + (eq27_e1179 * s.dn[809][9]));
        let eq27_e1181_d_n10: f64 = ((eq27_e1179_d_n10 * s.v[809]) + (eq27_e1179 * s.dn[809][10]));
        let eq27_e1181_d_n11: f64 = ((eq27_e1179_d_n11 * s.v[809]) + (eq27_e1179 * s.dn[809][11]));
        let eq27_e1183: f64 = (eq27_e1181 * (nv10 - nv9));
        let eq27_e1183_d_n0: f64 = (eq27_e1181_d_n0 * (nv10 - nv9));
        let eq27_e1183_d_n1: f64 = (eq27_e1181_d_n1 * (nv10 - nv9));
        let eq27_e1183_d_n2: f64 = (eq27_e1181_d_n2 * (nv10 - nv9));
        let eq27_e1183_d_n3: f64 = (eq27_e1181_d_n3 * (nv10 - nv9));
        let eq27_e1183_d_n4: f64 = (eq27_e1181_d_n4 * (nv10 - nv9));
        let eq27_e1183_d_n5: f64 = (eq27_e1181_d_n5 * (nv10 - nv9));
        let eq27_e1183_d_n6: f64 = (eq27_e1181_d_n6 * (nv10 - nv9));
        let eq27_e1183_d_n7: f64 = (eq27_e1181_d_n7 * (nv10 - nv9));
        let eq27_e1183_d_n8: f64 = (eq27_e1181_d_n8 * (nv10 - nv9));
        let eq27_e1183_d_n9: f64 = ((eq27_e1181_d_n9 * (nv10 - nv9)) + (-eq27_e1181));
        let eq27_e1183_d_n10: f64 = ((eq27_e1181_d_n10 * (nv10 - nv9)) + eq27_e1181);
        let eq27_e1183_d_n11: f64 = (eq27_e1181_d_n11 * (nv10 - nv9));
        (eq27_e1183, eq27_e1183_d_n0, eq27_e1183_d_n1, eq27_e1183_d_n2, eq27_e1183_d_n3, eq27_e1183_d_n4, eq27_e1183_d_n5, eq27_e1183_d_n6, eq27_e1183_d_n7, eq27_e1183_d_n8, eq27_e1183_d_n9, eq27_e1183_d_n10, eq27_e1183_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1185;
        let eq27_node_derivatives: [f64; 12] = [eq27_e1185_d_n0, eq27_e1185_d_n1, eq27_e1185_d_n2, eq27_e1185_d_n3, eq27_e1185_d_n4, eq27_e1185_d_n5, eq27_e1185_d_n6, eq27_e1185_d_n7, eq27_e1185_d_n8, eq27_e1185_d_n9, eq27_e1185_d_n10, eq27_e1185_d_n11];
        let eq27_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[9]),
            self.multiplicity * (eq27_value),
            &nodes,
            &eq27_node_derivatives,
            &branches,
            &eq27_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_28_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq28_e1195,) = {
    if (s.v[2706] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq28_value: f64 = eq28_e1195;
        stamper.stamp_current(
            Some(nodes[10]),
            Some(nodes[9]),
            self.multiplicity * (eq28_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_29_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq29_e1200,) = {
    if (!(s.v[2706] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e1200;
        stamper.stamp_potential(
            branches[4],
            eq29_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_30_block_0(
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
        let (eq30_e1210, eq30_e1210_d_n0, eq30_e1210_d_n1, eq30_e1210_d_n2, eq30_e1210_d_n3, eq30_e1210_d_n4, eq30_e1210_d_n5, eq30_e1210_d_n6, eq30_e1210_d_n7, eq30_e1210_d_n8, eq30_e1210_d_n9, eq30_e1210_d_n10, eq30_e1210_d_n11,) = {
    if (s.v[2707] != 0.0) {
        let eq30_e1204: f64 = (s.v[15] * p.p32);
        let eq30_e1204_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq30_e1204_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq30_e1204_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq30_e1204_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq30_e1204_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq30_e1204_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq30_e1204_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq30_e1204_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq30_e1204_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq30_e1204_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq30_e1204_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq30_e1204_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq30_e1206: f64 = (eq30_e1204 * s.v[810]);
        let eq30_e1206_d_n0: f64 = ((eq30_e1204_d_n0 * s.v[810]) + (eq30_e1204 * s.dn[810][0]));
        let eq30_e1206_d_n1: f64 = ((eq30_e1204_d_n1 * s.v[810]) + (eq30_e1204 * s.dn[810][1]));
        let eq30_e1206_d_n2: f64 = ((eq30_e1204_d_n2 * s.v[810]) + (eq30_e1204 * s.dn[810][2]));
        let eq30_e1206_d_n3: f64 = ((eq30_e1204_d_n3 * s.v[810]) + (eq30_e1204 * s.dn[810][3]));
        let eq30_e1206_d_n4: f64 = ((eq30_e1204_d_n4 * s.v[810]) + (eq30_e1204 * s.dn[810][4]));
        let eq30_e1206_d_n5: f64 = ((eq30_e1204_d_n5 * s.v[810]) + (eq30_e1204 * s.dn[810][5]));
        let eq30_e1206_d_n6: f64 = ((eq30_e1204_d_n6 * s.v[810]) + (eq30_e1204 * s.dn[810][6]));
        let eq30_e1206_d_n7: f64 = ((eq30_e1204_d_n7 * s.v[810]) + (eq30_e1204 * s.dn[810][7]));
        let eq30_e1206_d_n8: f64 = ((eq30_e1204_d_n8 * s.v[810]) + (eq30_e1204 * s.dn[810][8]));
        let eq30_e1206_d_n9: f64 = ((eq30_e1204_d_n9 * s.v[810]) + (eq30_e1204 * s.dn[810][9]));
        let eq30_e1206_d_n10: f64 = ((eq30_e1204_d_n10 * s.v[810]) + (eq30_e1204 * s.dn[810][10]));
        let eq30_e1206_d_n11: f64 = ((eq30_e1204_d_n11 * s.v[810]) + (eq30_e1204 * s.dn[810][11]));
        let eq30_e1208: f64 = (eq30_e1206 * (nv11 - nv9));
        let eq30_e1208_d_n0: f64 = (eq30_e1206_d_n0 * (nv11 - nv9));
        let eq30_e1208_d_n1: f64 = (eq30_e1206_d_n1 * (nv11 - nv9));
        let eq30_e1208_d_n2: f64 = (eq30_e1206_d_n2 * (nv11 - nv9));
        let eq30_e1208_d_n3: f64 = (eq30_e1206_d_n3 * (nv11 - nv9));
        let eq30_e1208_d_n4: f64 = (eq30_e1206_d_n4 * (nv11 - nv9));
        let eq30_e1208_d_n5: f64 = (eq30_e1206_d_n5 * (nv11 - nv9));
        let eq30_e1208_d_n6: f64 = (eq30_e1206_d_n6 * (nv11 - nv9));
        let eq30_e1208_d_n7: f64 = (eq30_e1206_d_n7 * (nv11 - nv9));
        let eq30_e1208_d_n8: f64 = (eq30_e1206_d_n8 * (nv11 - nv9));
        let eq30_e1208_d_n9: f64 = ((eq30_e1206_d_n9 * (nv11 - nv9)) + (-eq30_e1206));
        let eq30_e1208_d_n10: f64 = (eq30_e1206_d_n10 * (nv11 - nv9));
        let eq30_e1208_d_n11: f64 = ((eq30_e1206_d_n11 * (nv11 - nv9)) + eq30_e1206);
        (eq30_e1208, eq30_e1208_d_n0, eq30_e1208_d_n1, eq30_e1208_d_n2, eq30_e1208_d_n3, eq30_e1208_d_n4, eq30_e1208_d_n5, eq30_e1208_d_n6, eq30_e1208_d_n7, eq30_e1208_d_n8, eq30_e1208_d_n9, eq30_e1208_d_n10, eq30_e1208_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e1210;
        let eq30_node_derivatives: [f64; 12] = [eq30_e1210_d_n0, eq30_e1210_d_n1, eq30_e1210_d_n2, eq30_e1210_d_n3, eq30_e1210_d_n4, eq30_e1210_d_n5, eq30_e1210_d_n6, eq30_e1210_d_n7, eq30_e1210_d_n8, eq30_e1210_d_n9, eq30_e1210_d_n10, eq30_e1210_d_n11];
        let eq30_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[9]),
            self.multiplicity * (eq30_value),
            &nodes,
            &eq30_node_derivatives,
            &branches,
            &eq30_branch_derivatives,
            self.multiplicity,
        );
    }
}
