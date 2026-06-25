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
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq15_e1109, eq15_e1109_d_n0, eq15_e1109_d_n1, eq15_e1109_d_n2, eq15_e1109_d_n3, eq15_e1109_d_n4, eq15_e1109_d_n5, eq15_e1109_d_n6, eq15_e1109_d_n7, eq15_e1109_d_n8, eq15_e1109_d_n9, eq15_e1109_d_n10, eq15_e1109_d_n11, eq15_e1109_d_n12,) = {
    if (s.v[2716] != 0.0) {
        let eq15_e1103: f64 = (s.v[15] * p.p32);
        let eq15_e1103_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq15_e1103_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq15_e1103_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq15_e1103_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq15_e1103_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq15_e1103_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq15_e1103_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq15_e1103_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq15_e1103_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq15_e1103_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq15_e1103_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq15_e1103_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq15_e1103_d_n12: f64 = (s.dn[15][12] * p.p32);
        let eq15_e1105: f64 = (eq15_e1103 * s.v[800]);
        let eq15_e1105_d_n0: f64 = ((eq15_e1103_d_n0 * s.v[800]) + (eq15_e1103 * s.dn[800][0]));
        let eq15_e1105_d_n1: f64 = ((eq15_e1103_d_n1 * s.v[800]) + (eq15_e1103 * s.dn[800][1]));
        let eq15_e1105_d_n2: f64 = ((eq15_e1103_d_n2 * s.v[800]) + (eq15_e1103 * s.dn[800][2]));
        let eq15_e1105_d_n3: f64 = ((eq15_e1103_d_n3 * s.v[800]) + (eq15_e1103 * s.dn[800][3]));
        let eq15_e1105_d_n4: f64 = ((eq15_e1103_d_n4 * s.v[800]) + (eq15_e1103 * s.dn[800][4]));
        let eq15_e1105_d_n5: f64 = ((eq15_e1103_d_n5 * s.v[800]) + (eq15_e1103 * s.dn[800][5]));
        let eq15_e1105_d_n6: f64 = ((eq15_e1103_d_n6 * s.v[800]) + (eq15_e1103 * s.dn[800][6]));
        let eq15_e1105_d_n7: f64 = ((eq15_e1103_d_n7 * s.v[800]) + (eq15_e1103 * s.dn[800][7]));
        let eq15_e1105_d_n8: f64 = ((eq15_e1103_d_n8 * s.v[800]) + (eq15_e1103 * s.dn[800][8]));
        let eq15_e1105_d_n9: f64 = ((eq15_e1103_d_n9 * s.v[800]) + (eq15_e1103 * s.dn[800][9]));
        let eq15_e1105_d_n10: f64 = ((eq15_e1103_d_n10 * s.v[800]) + (eq15_e1103 * s.dn[800][10]));
        let eq15_e1105_d_n11: f64 = ((eq15_e1103_d_n11 * s.v[800]) + (eq15_e1103 * s.dn[800][11]));
        let eq15_e1105_d_n12: f64 = ((eq15_e1103_d_n12 * s.v[800]) + (eq15_e1103 * s.dn[800][12]));
        let eq15_e1107: f64 = (eq15_e1105 * (nv1 - nv6));
        let eq15_e1107_d_n0: f64 = (eq15_e1105_d_n0 * (nv1 - nv6));
        let eq15_e1107_d_n1: f64 = ((eq15_e1105_d_n1 * (nv1 - nv6)) + eq15_e1105);
        let eq15_e1107_d_n2: f64 = (eq15_e1105_d_n2 * (nv1 - nv6));
        let eq15_e1107_d_n3: f64 = (eq15_e1105_d_n3 * (nv1 - nv6));
        let eq15_e1107_d_n4: f64 = (eq15_e1105_d_n4 * (nv1 - nv6));
        let eq15_e1107_d_n5: f64 = (eq15_e1105_d_n5 * (nv1 - nv6));
        let eq15_e1107_d_n6: f64 = ((eq15_e1105_d_n6 * (nv1 - nv6)) + (-eq15_e1105));
        let eq15_e1107_d_n7: f64 = (eq15_e1105_d_n7 * (nv1 - nv6));
        let eq15_e1107_d_n8: f64 = (eq15_e1105_d_n8 * (nv1 - nv6));
        let eq15_e1107_d_n9: f64 = (eq15_e1105_d_n9 * (nv1 - nv6));
        let eq15_e1107_d_n10: f64 = (eq15_e1105_d_n10 * (nv1 - nv6));
        let eq15_e1107_d_n11: f64 = (eq15_e1105_d_n11 * (nv1 - nv6));
        let eq15_e1107_d_n12: f64 = (eq15_e1105_d_n12 * (nv1 - nv6));
        (eq15_e1107, eq15_e1107_d_n0, eq15_e1107_d_n1, eq15_e1107_d_n2, eq15_e1107_d_n3, eq15_e1107_d_n4, eq15_e1107_d_n5, eq15_e1107_d_n6, eq15_e1107_d_n7, eq15_e1107_d_n8, eq15_e1107_d_n9, eq15_e1107_d_n10, eq15_e1107_d_n11, eq15_e1107_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e1109;
        let eq15_node_derivatives: [f64; 13] = [eq15_e1109_d_n0, eq15_e1109_d_n1, eq15_e1109_d_n2, eq15_e1109_d_n3, eq15_e1109_d_n4, eq15_e1109_d_n5, eq15_e1109_d_n6, eq15_e1109_d_n7, eq15_e1109_d_n8, eq15_e1109_d_n9, eq15_e1109_d_n10, eq15_e1109_d_n11, eq15_e1109_d_n12];
        let eq15_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[6]),
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
        let (eq16_e1119,) = {
    if (s.v[2716] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq16_value: f64 = eq16_e1119;
        stamper.stamp_current(
            Some(nodes[1]),
            Some(nodes[6]),
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
        let (eq17_e1124,) = {
    if (!(s.v[2716] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq17_value: f64 = eq17_e1124;
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq18_e1134, eq18_e1134_d_n0, eq18_e1134_d_n1, eq18_e1134_d_n2, eq18_e1134_d_n3, eq18_e1134_d_n4, eq18_e1134_d_n5, eq18_e1134_d_n6, eq18_e1134_d_n7, eq18_e1134_d_n8, eq18_e1134_d_n9, eq18_e1134_d_n10, eq18_e1134_d_n11, eq18_e1134_d_n12,) = {
    if (s.v[2717] != 0.0) {
        let eq18_e1128: f64 = (s.v[15] * p.p32);
        let eq18_e1128_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq18_e1128_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq18_e1128_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq18_e1128_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq18_e1128_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq18_e1128_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq18_e1128_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq18_e1128_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq18_e1128_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq18_e1128_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq18_e1128_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq18_e1128_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq18_e1128_d_n12: f64 = (s.dn[15][12] * p.p32);
        let eq18_e1130: f64 = (eq18_e1128 * s.v[801]);
        let eq18_e1130_d_n0: f64 = ((eq18_e1128_d_n0 * s.v[801]) + (eq18_e1128 * s.dn[801][0]));
        let eq18_e1130_d_n1: f64 = ((eq18_e1128_d_n1 * s.v[801]) + (eq18_e1128 * s.dn[801][1]));
        let eq18_e1130_d_n2: f64 = ((eq18_e1128_d_n2 * s.v[801]) + (eq18_e1128 * s.dn[801][2]));
        let eq18_e1130_d_n3: f64 = ((eq18_e1128_d_n3 * s.v[801]) + (eq18_e1128 * s.dn[801][3]));
        let eq18_e1130_d_n4: f64 = ((eq18_e1128_d_n4 * s.v[801]) + (eq18_e1128 * s.dn[801][4]));
        let eq18_e1130_d_n5: f64 = ((eq18_e1128_d_n5 * s.v[801]) + (eq18_e1128 * s.dn[801][5]));
        let eq18_e1130_d_n6: f64 = ((eq18_e1128_d_n6 * s.v[801]) + (eq18_e1128 * s.dn[801][6]));
        let eq18_e1130_d_n7: f64 = ((eq18_e1128_d_n7 * s.v[801]) + (eq18_e1128 * s.dn[801][7]));
        let eq18_e1130_d_n8: f64 = ((eq18_e1128_d_n8 * s.v[801]) + (eq18_e1128 * s.dn[801][8]));
        let eq18_e1130_d_n9: f64 = ((eq18_e1128_d_n9 * s.v[801]) + (eq18_e1128 * s.dn[801][9]));
        let eq18_e1130_d_n10: f64 = ((eq18_e1128_d_n10 * s.v[801]) + (eq18_e1128 * s.dn[801][10]));
        let eq18_e1130_d_n11: f64 = ((eq18_e1128_d_n11 * s.v[801]) + (eq18_e1128 * s.dn[801][11]));
        let eq18_e1130_d_n12: f64 = ((eq18_e1128_d_n12 * s.v[801]) + (eq18_e1128 * s.dn[801][12]));
        let eq18_e1132: f64 = (eq18_e1130 * (nv2 - nv7));
        let eq18_e1132_d_n0: f64 = (eq18_e1130_d_n0 * (nv2 - nv7));
        let eq18_e1132_d_n1: f64 = (eq18_e1130_d_n1 * (nv2 - nv7));
        let eq18_e1132_d_n2: f64 = ((eq18_e1130_d_n2 * (nv2 - nv7)) + eq18_e1130);
        let eq18_e1132_d_n3: f64 = (eq18_e1130_d_n3 * (nv2 - nv7));
        let eq18_e1132_d_n4: f64 = (eq18_e1130_d_n4 * (nv2 - nv7));
        let eq18_e1132_d_n5: f64 = (eq18_e1130_d_n5 * (nv2 - nv7));
        let eq18_e1132_d_n6: f64 = (eq18_e1130_d_n6 * (nv2 - nv7));
        let eq18_e1132_d_n7: f64 = ((eq18_e1130_d_n7 * (nv2 - nv7)) + (-eq18_e1130));
        let eq18_e1132_d_n8: f64 = (eq18_e1130_d_n8 * (nv2 - nv7));
        let eq18_e1132_d_n9: f64 = (eq18_e1130_d_n9 * (nv2 - nv7));
        let eq18_e1132_d_n10: f64 = (eq18_e1130_d_n10 * (nv2 - nv7));
        let eq18_e1132_d_n11: f64 = (eq18_e1130_d_n11 * (nv2 - nv7));
        let eq18_e1132_d_n12: f64 = (eq18_e1130_d_n12 * (nv2 - nv7));
        (eq18_e1132, eq18_e1132_d_n0, eq18_e1132_d_n1, eq18_e1132_d_n2, eq18_e1132_d_n3, eq18_e1132_d_n4, eq18_e1132_d_n5, eq18_e1132_d_n6, eq18_e1132_d_n7, eq18_e1132_d_n8, eq18_e1132_d_n9, eq18_e1132_d_n10, eq18_e1132_d_n11, eq18_e1132_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1134;
        let eq18_node_derivatives: [f64; 13] = [eq18_e1134_d_n0, eq18_e1134_d_n1, eq18_e1134_d_n2, eq18_e1134_d_n3, eq18_e1134_d_n4, eq18_e1134_d_n5, eq18_e1134_d_n6, eq18_e1134_d_n7, eq18_e1134_d_n8, eq18_e1134_d_n9, eq18_e1134_d_n10, eq18_e1134_d_n11, eq18_e1134_d_n12];
        let eq18_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[7]),
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
        let (eq19_e1144,) = {
    if (s.v[2717] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e1144;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[7]),
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
        let (eq20_e1149,) = {
    if (!(s.v[2717] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e1149;
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
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq21_e1159, eq21_e1159_d_n0, eq21_e1159_d_n1, eq21_e1159_d_n2, eq21_e1159_d_n3, eq21_e1159_d_n4, eq21_e1159_d_n5, eq21_e1159_d_n6, eq21_e1159_d_n7, eq21_e1159_d_n8, eq21_e1159_d_n9, eq21_e1159_d_n10, eq21_e1159_d_n11, eq21_e1159_d_n12,) = {
    if (s.v[2718] != 0.0) {
        let eq21_e1153: f64 = (s.v[15] * p.p32);
        let eq21_e1153_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq21_e1153_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq21_e1153_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq21_e1153_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq21_e1153_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq21_e1153_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq21_e1153_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq21_e1153_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq21_e1153_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq21_e1153_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq21_e1153_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq21_e1153_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq21_e1153_d_n12: f64 = (s.dn[15][12] * p.p32);
        let eq21_e1155: f64 = (eq21_e1153 * s.v[802]);
        let eq21_e1155_d_n0: f64 = ((eq21_e1153_d_n0 * s.v[802]) + (eq21_e1153 * s.dn[802][0]));
        let eq21_e1155_d_n1: f64 = ((eq21_e1153_d_n1 * s.v[802]) + (eq21_e1153 * s.dn[802][1]));
        let eq21_e1155_d_n2: f64 = ((eq21_e1153_d_n2 * s.v[802]) + (eq21_e1153 * s.dn[802][2]));
        let eq21_e1155_d_n3: f64 = ((eq21_e1153_d_n3 * s.v[802]) + (eq21_e1153 * s.dn[802][3]));
        let eq21_e1155_d_n4: f64 = ((eq21_e1153_d_n4 * s.v[802]) + (eq21_e1153 * s.dn[802][4]));
        let eq21_e1155_d_n5: f64 = ((eq21_e1153_d_n5 * s.v[802]) + (eq21_e1153 * s.dn[802][5]));
        let eq21_e1155_d_n6: f64 = ((eq21_e1153_d_n6 * s.v[802]) + (eq21_e1153 * s.dn[802][6]));
        let eq21_e1155_d_n7: f64 = ((eq21_e1153_d_n7 * s.v[802]) + (eq21_e1153 * s.dn[802][7]));
        let eq21_e1155_d_n8: f64 = ((eq21_e1153_d_n8 * s.v[802]) + (eq21_e1153 * s.dn[802][8]));
        let eq21_e1155_d_n9: f64 = ((eq21_e1153_d_n9 * s.v[802]) + (eq21_e1153 * s.dn[802][9]));
        let eq21_e1155_d_n10: f64 = ((eq21_e1153_d_n10 * s.v[802]) + (eq21_e1153 * s.dn[802][10]));
        let eq21_e1155_d_n11: f64 = ((eq21_e1153_d_n11 * s.v[802]) + (eq21_e1153 * s.dn[802][11]));
        let eq21_e1155_d_n12: f64 = ((eq21_e1153_d_n12 * s.v[802]) + (eq21_e1153 * s.dn[802][12]));
        let eq21_e1157: f64 = (eq21_e1155 * (nv0 - nv8));
        let eq21_e1157_d_n0: f64 = ((eq21_e1155_d_n0 * (nv0 - nv8)) + eq21_e1155);
        let eq21_e1157_d_n1: f64 = (eq21_e1155_d_n1 * (nv0 - nv8));
        let eq21_e1157_d_n2: f64 = (eq21_e1155_d_n2 * (nv0 - nv8));
        let eq21_e1157_d_n3: f64 = (eq21_e1155_d_n3 * (nv0 - nv8));
        let eq21_e1157_d_n4: f64 = (eq21_e1155_d_n4 * (nv0 - nv8));
        let eq21_e1157_d_n5: f64 = (eq21_e1155_d_n5 * (nv0 - nv8));
        let eq21_e1157_d_n6: f64 = (eq21_e1155_d_n6 * (nv0 - nv8));
        let eq21_e1157_d_n7: f64 = (eq21_e1155_d_n7 * (nv0 - nv8));
        let eq21_e1157_d_n8: f64 = ((eq21_e1155_d_n8 * (nv0 - nv8)) + (-eq21_e1155));
        let eq21_e1157_d_n9: f64 = (eq21_e1155_d_n9 * (nv0 - nv8));
        let eq21_e1157_d_n10: f64 = (eq21_e1155_d_n10 * (nv0 - nv8));
        let eq21_e1157_d_n11: f64 = (eq21_e1155_d_n11 * (nv0 - nv8));
        let eq21_e1157_d_n12: f64 = (eq21_e1155_d_n12 * (nv0 - nv8));
        (eq21_e1157, eq21_e1157_d_n0, eq21_e1157_d_n1, eq21_e1157_d_n2, eq21_e1157_d_n3, eq21_e1157_d_n4, eq21_e1157_d_n5, eq21_e1157_d_n6, eq21_e1157_d_n7, eq21_e1157_d_n8, eq21_e1157_d_n9, eq21_e1157_d_n10, eq21_e1157_d_n11, eq21_e1157_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1159;
        let eq21_node_derivatives: [f64; 13] = [eq21_e1159_d_n0, eq21_e1159_d_n1, eq21_e1159_d_n2, eq21_e1159_d_n3, eq21_e1159_d_n4, eq21_e1159_d_n5, eq21_e1159_d_n6, eq21_e1159_d_n7, eq21_e1159_d_n8, eq21_e1159_d_n9, eq21_e1159_d_n10, eq21_e1159_d_n11, eq21_e1159_d_n12];
        let eq21_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[8]),
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
        let (eq22_e1169,) = {
    if (s.v[2718] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e1169;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[8]),
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
        let (eq23_e1174,) = {
    if (!(s.v[2718] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e1174;
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
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq24_e1184, eq24_e1184_d_n0, eq24_e1184_d_n1, eq24_e1184_d_n2, eq24_e1184_d_n3, eq24_e1184_d_n4, eq24_e1184_d_n5, eq24_e1184_d_n6, eq24_e1184_d_n7, eq24_e1184_d_n8, eq24_e1184_d_n9, eq24_e1184_d_n10, eq24_e1184_d_n11, eq24_e1184_d_n12,) = {
    if (s.v[2719] != 0.0) {
        let eq24_e1178: f64 = (s.v[15] * p.p32);
        let eq24_e1178_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq24_e1178_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq24_e1178_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq24_e1178_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq24_e1178_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq24_e1178_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq24_e1178_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq24_e1178_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq24_e1178_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq24_e1178_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq24_e1178_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq24_e1178_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq24_e1178_d_n12: f64 = (s.dn[15][12] * p.p32);
        let eq24_e1180: f64 = (eq24_e1178 * s.v[803]);
        let eq24_e1180_d_n0: f64 = ((eq24_e1178_d_n0 * s.v[803]) + (eq24_e1178 * s.dn[803][0]));
        let eq24_e1180_d_n1: f64 = ((eq24_e1178_d_n1 * s.v[803]) + (eq24_e1178 * s.dn[803][1]));
        let eq24_e1180_d_n2: f64 = ((eq24_e1178_d_n2 * s.v[803]) + (eq24_e1178 * s.dn[803][2]));
        let eq24_e1180_d_n3: f64 = ((eq24_e1178_d_n3 * s.v[803]) + (eq24_e1178 * s.dn[803][3]));
        let eq24_e1180_d_n4: f64 = ((eq24_e1178_d_n4 * s.v[803]) + (eq24_e1178 * s.dn[803][4]));
        let eq24_e1180_d_n5: f64 = ((eq24_e1178_d_n5 * s.v[803]) + (eq24_e1178 * s.dn[803][5]));
        let eq24_e1180_d_n6: f64 = ((eq24_e1178_d_n6 * s.v[803]) + (eq24_e1178 * s.dn[803][6]));
        let eq24_e1180_d_n7: f64 = ((eq24_e1178_d_n7 * s.v[803]) + (eq24_e1178 * s.dn[803][7]));
        let eq24_e1180_d_n8: f64 = ((eq24_e1178_d_n8 * s.v[803]) + (eq24_e1178 * s.dn[803][8]));
        let eq24_e1180_d_n9: f64 = ((eq24_e1178_d_n9 * s.v[803]) + (eq24_e1178 * s.dn[803][9]));
        let eq24_e1180_d_n10: f64 = ((eq24_e1178_d_n10 * s.v[803]) + (eq24_e1178 * s.dn[803][10]));
        let eq24_e1180_d_n11: f64 = ((eq24_e1178_d_n11 * s.v[803]) + (eq24_e1178 * s.dn[803][11]));
        let eq24_e1180_d_n12: f64 = ((eq24_e1178_d_n12 * s.v[803]) + (eq24_e1178 * s.dn[803][12]));
        let eq24_e1182: f64 = (eq24_e1180 * (nv9 - nv10));
        let eq24_e1182_d_n0: f64 = (eq24_e1180_d_n0 * (nv9 - nv10));
        let eq24_e1182_d_n1: f64 = (eq24_e1180_d_n1 * (nv9 - nv10));
        let eq24_e1182_d_n2: f64 = (eq24_e1180_d_n2 * (nv9 - nv10));
        let eq24_e1182_d_n3: f64 = (eq24_e1180_d_n3 * (nv9 - nv10));
        let eq24_e1182_d_n4: f64 = (eq24_e1180_d_n4 * (nv9 - nv10));
        let eq24_e1182_d_n5: f64 = (eq24_e1180_d_n5 * (nv9 - nv10));
        let eq24_e1182_d_n6: f64 = (eq24_e1180_d_n6 * (nv9 - nv10));
        let eq24_e1182_d_n7: f64 = (eq24_e1180_d_n7 * (nv9 - nv10));
        let eq24_e1182_d_n8: f64 = (eq24_e1180_d_n8 * (nv9 - nv10));
        let eq24_e1182_d_n9: f64 = ((eq24_e1180_d_n9 * (nv9 - nv10)) + eq24_e1180);
        let eq24_e1182_d_n10: f64 = ((eq24_e1180_d_n10 * (nv9 - nv10)) + (-eq24_e1180));
        let eq24_e1182_d_n11: f64 = (eq24_e1180_d_n11 * (nv9 - nv10));
        let eq24_e1182_d_n12: f64 = (eq24_e1180_d_n12 * (nv9 - nv10));
        (eq24_e1182, eq24_e1182_d_n0, eq24_e1182_d_n1, eq24_e1182_d_n2, eq24_e1182_d_n3, eq24_e1182_d_n4, eq24_e1182_d_n5, eq24_e1182_d_n6, eq24_e1182_d_n7, eq24_e1182_d_n8, eq24_e1182_d_n9, eq24_e1182_d_n10, eq24_e1182_d_n11, eq24_e1182_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1184;
        let eq24_node_derivatives: [f64; 13] = [eq24_e1184_d_n0, eq24_e1184_d_n1, eq24_e1184_d_n2, eq24_e1184_d_n3, eq24_e1184_d_n4, eq24_e1184_d_n5, eq24_e1184_d_n6, eq24_e1184_d_n7, eq24_e1184_d_n8, eq24_e1184_d_n9, eq24_e1184_d_n10, eq24_e1184_d_n11, eq24_e1184_d_n12];
        let eq24_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[10]),
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
        let (eq25_e1194,) = {
    if (s.v[2719] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq25_value: f64 = eq25_e1194;
        stamper.stamp_current(
            Some(nodes[9]),
            Some(nodes[10]),
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
        let (eq26_e1199,) = {
    if (!(s.v[2719] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e1199;
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq27_e1209, eq27_e1209_d_n0, eq27_e1209_d_n1, eq27_e1209_d_n2, eq27_e1209_d_n3, eq27_e1209_d_n4, eq27_e1209_d_n5, eq27_e1209_d_n6, eq27_e1209_d_n7, eq27_e1209_d_n8, eq27_e1209_d_n9, eq27_e1209_d_n10, eq27_e1209_d_n11, eq27_e1209_d_n12,) = {
    if (s.v[2720] != 0.0) {
        let eq27_e1203: f64 = (s.v[15] * p.p32);
        let eq27_e1203_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq27_e1203_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq27_e1203_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq27_e1203_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq27_e1203_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq27_e1203_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq27_e1203_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq27_e1203_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq27_e1203_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq27_e1203_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq27_e1203_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq27_e1203_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq27_e1203_d_n12: f64 = (s.dn[15][12] * p.p32);
        let eq27_e1205: f64 = (eq27_e1203 * s.v[804]);
        let eq27_e1205_d_n0: f64 = ((eq27_e1203_d_n0 * s.v[804]) + (eq27_e1203 * s.dn[804][0]));
        let eq27_e1205_d_n1: f64 = ((eq27_e1203_d_n1 * s.v[804]) + (eq27_e1203 * s.dn[804][1]));
        let eq27_e1205_d_n2: f64 = ((eq27_e1203_d_n2 * s.v[804]) + (eq27_e1203 * s.dn[804][2]));
        let eq27_e1205_d_n3: f64 = ((eq27_e1203_d_n3 * s.v[804]) + (eq27_e1203 * s.dn[804][3]));
        let eq27_e1205_d_n4: f64 = ((eq27_e1203_d_n4 * s.v[804]) + (eq27_e1203 * s.dn[804][4]));
        let eq27_e1205_d_n5: f64 = ((eq27_e1203_d_n5 * s.v[804]) + (eq27_e1203 * s.dn[804][5]));
        let eq27_e1205_d_n6: f64 = ((eq27_e1203_d_n6 * s.v[804]) + (eq27_e1203 * s.dn[804][6]));
        let eq27_e1205_d_n7: f64 = ((eq27_e1203_d_n7 * s.v[804]) + (eq27_e1203 * s.dn[804][7]));
        let eq27_e1205_d_n8: f64 = ((eq27_e1203_d_n8 * s.v[804]) + (eq27_e1203 * s.dn[804][8]));
        let eq27_e1205_d_n9: f64 = ((eq27_e1203_d_n9 * s.v[804]) + (eq27_e1203 * s.dn[804][9]));
        let eq27_e1205_d_n10: f64 = ((eq27_e1203_d_n10 * s.v[804]) + (eq27_e1203 * s.dn[804][10]));
        let eq27_e1205_d_n11: f64 = ((eq27_e1203_d_n11 * s.v[804]) + (eq27_e1203 * s.dn[804][11]));
        let eq27_e1205_d_n12: f64 = ((eq27_e1203_d_n12 * s.v[804]) + (eq27_e1203 * s.dn[804][12]));
        let eq27_e1207: f64 = (eq27_e1205 * (nv11 - nv10));
        let eq27_e1207_d_n0: f64 = (eq27_e1205_d_n0 * (nv11 - nv10));
        let eq27_e1207_d_n1: f64 = (eq27_e1205_d_n1 * (nv11 - nv10));
        let eq27_e1207_d_n2: f64 = (eq27_e1205_d_n2 * (nv11 - nv10));
        let eq27_e1207_d_n3: f64 = (eq27_e1205_d_n3 * (nv11 - nv10));
        let eq27_e1207_d_n4: f64 = (eq27_e1205_d_n4 * (nv11 - nv10));
        let eq27_e1207_d_n5: f64 = (eq27_e1205_d_n5 * (nv11 - nv10));
        let eq27_e1207_d_n6: f64 = (eq27_e1205_d_n6 * (nv11 - nv10));
        let eq27_e1207_d_n7: f64 = (eq27_e1205_d_n7 * (nv11 - nv10));
        let eq27_e1207_d_n8: f64 = (eq27_e1205_d_n8 * (nv11 - nv10));
        let eq27_e1207_d_n9: f64 = (eq27_e1205_d_n9 * (nv11 - nv10));
        let eq27_e1207_d_n10: f64 = ((eq27_e1205_d_n10 * (nv11 - nv10)) + (-eq27_e1205));
        let eq27_e1207_d_n11: f64 = ((eq27_e1205_d_n11 * (nv11 - nv10)) + eq27_e1205);
        let eq27_e1207_d_n12: f64 = (eq27_e1205_d_n12 * (nv11 - nv10));
        (eq27_e1207, eq27_e1207_d_n0, eq27_e1207_d_n1, eq27_e1207_d_n2, eq27_e1207_d_n3, eq27_e1207_d_n4, eq27_e1207_d_n5, eq27_e1207_d_n6, eq27_e1207_d_n7, eq27_e1207_d_n8, eq27_e1207_d_n9, eq27_e1207_d_n10, eq27_e1207_d_n11, eq27_e1207_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1209;
        let eq27_node_derivatives: [f64; 13] = [eq27_e1209_d_n0, eq27_e1209_d_n1, eq27_e1209_d_n2, eq27_e1209_d_n3, eq27_e1209_d_n4, eq27_e1209_d_n5, eq27_e1209_d_n6, eq27_e1209_d_n7, eq27_e1209_d_n8, eq27_e1209_d_n9, eq27_e1209_d_n10, eq27_e1209_d_n11, eq27_e1209_d_n12];
        let eq27_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[10]),
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
        let (eq28_e1219,) = {
    if (s.v[2720] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq28_value: f64 = eq28_e1219;
        stamper.stamp_current(
            Some(nodes[11]),
            Some(nodes[10]),
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
        let (eq29_e1224,) = {
    if (!(s.v[2720] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e1224;
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq30_e1234, eq30_e1234_d_n0, eq30_e1234_d_n1, eq30_e1234_d_n2, eq30_e1234_d_n3, eq30_e1234_d_n4, eq30_e1234_d_n5, eq30_e1234_d_n6, eq30_e1234_d_n7, eq30_e1234_d_n8, eq30_e1234_d_n9, eq30_e1234_d_n10, eq30_e1234_d_n11, eq30_e1234_d_n12,) = {
    if (s.v[2721] != 0.0) {
        let eq30_e1228: f64 = (s.v[15] * p.p32);
        let eq30_e1228_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq30_e1228_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq30_e1228_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq30_e1228_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq30_e1228_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq30_e1228_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq30_e1228_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq30_e1228_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq30_e1228_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq30_e1228_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq30_e1228_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq30_e1228_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq30_e1228_d_n12: f64 = (s.dn[15][12] * p.p32);
        let eq30_e1230: f64 = (eq30_e1228 * s.v[805]);
        let eq30_e1230_d_n0: f64 = ((eq30_e1228_d_n0 * s.v[805]) + (eq30_e1228 * s.dn[805][0]));
        let eq30_e1230_d_n1: f64 = ((eq30_e1228_d_n1 * s.v[805]) + (eq30_e1228 * s.dn[805][1]));
        let eq30_e1230_d_n2: f64 = ((eq30_e1228_d_n2 * s.v[805]) + (eq30_e1228 * s.dn[805][2]));
        let eq30_e1230_d_n3: f64 = ((eq30_e1228_d_n3 * s.v[805]) + (eq30_e1228 * s.dn[805][3]));
        let eq30_e1230_d_n4: f64 = ((eq30_e1228_d_n4 * s.v[805]) + (eq30_e1228 * s.dn[805][4]));
        let eq30_e1230_d_n5: f64 = ((eq30_e1228_d_n5 * s.v[805]) + (eq30_e1228 * s.dn[805][5]));
        let eq30_e1230_d_n6: f64 = ((eq30_e1228_d_n6 * s.v[805]) + (eq30_e1228 * s.dn[805][6]));
        let eq30_e1230_d_n7: f64 = ((eq30_e1228_d_n7 * s.v[805]) + (eq30_e1228 * s.dn[805][7]));
        let eq30_e1230_d_n8: f64 = ((eq30_e1228_d_n8 * s.v[805]) + (eq30_e1228 * s.dn[805][8]));
        let eq30_e1230_d_n9: f64 = ((eq30_e1228_d_n9 * s.v[805]) + (eq30_e1228 * s.dn[805][9]));
        let eq30_e1230_d_n10: f64 = ((eq30_e1228_d_n10 * s.v[805]) + (eq30_e1228 * s.dn[805][10]));
        let eq30_e1230_d_n11: f64 = ((eq30_e1228_d_n11 * s.v[805]) + (eq30_e1228 * s.dn[805][11]));
        let eq30_e1230_d_n12: f64 = ((eq30_e1228_d_n12 * s.v[805]) + (eq30_e1228 * s.dn[805][12]));
        let eq30_e1232: f64 = (eq30_e1230 * (nv12 - nv10));
        let eq30_e1232_d_n0: f64 = (eq30_e1230_d_n0 * (nv12 - nv10));
        let eq30_e1232_d_n1: f64 = (eq30_e1230_d_n1 * (nv12 - nv10));
        let eq30_e1232_d_n2: f64 = (eq30_e1230_d_n2 * (nv12 - nv10));
        let eq30_e1232_d_n3: f64 = (eq30_e1230_d_n3 * (nv12 - nv10));
        let eq30_e1232_d_n4: f64 = (eq30_e1230_d_n4 * (nv12 - nv10));
        let eq30_e1232_d_n5: f64 = (eq30_e1230_d_n5 * (nv12 - nv10));
        let eq30_e1232_d_n6: f64 = (eq30_e1230_d_n6 * (nv12 - nv10));
        let eq30_e1232_d_n7: f64 = (eq30_e1230_d_n7 * (nv12 - nv10));
        let eq30_e1232_d_n8: f64 = (eq30_e1230_d_n8 * (nv12 - nv10));
        let eq30_e1232_d_n9: f64 = (eq30_e1230_d_n9 * (nv12 - nv10));
        let eq30_e1232_d_n10: f64 = ((eq30_e1230_d_n10 * (nv12 - nv10)) + (-eq30_e1230));
        let eq30_e1232_d_n11: f64 = (eq30_e1230_d_n11 * (nv12 - nv10));
        let eq30_e1232_d_n12: f64 = ((eq30_e1230_d_n12 * (nv12 - nv10)) + eq30_e1230);
        (eq30_e1232, eq30_e1232_d_n0, eq30_e1232_d_n1, eq30_e1232_d_n2, eq30_e1232_d_n3, eq30_e1232_d_n4, eq30_e1232_d_n5, eq30_e1232_d_n6, eq30_e1232_d_n7, eq30_e1232_d_n8, eq30_e1232_d_n9, eq30_e1232_d_n10, eq30_e1232_d_n11, eq30_e1232_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e1234;
        let eq30_node_derivatives: [f64; 13] = [eq30_e1234_d_n0, eq30_e1234_d_n1, eq30_e1234_d_n2, eq30_e1234_d_n3, eq30_e1234_d_n4, eq30_e1234_d_n5, eq30_e1234_d_n6, eq30_e1234_d_n7, eq30_e1234_d_n8, eq30_e1234_d_n9, eq30_e1234_d_n10, eq30_e1234_d_n11, eq30_e1234_d_n12];
        let eq30_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            Some(nodes[10]),
            self.multiplicity * (eq30_value),
            &nodes,
            &eq30_node_derivatives,
            &branches,
            &eq30_branch_derivatives,
            self.multiplicity,
        );
    }
}
