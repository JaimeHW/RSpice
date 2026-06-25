#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_33_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq33_e1000,) = {
    if ((s.v[668] != 0.0) && (s.v[669] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq33_value: f64 = eq33_e1000;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[5]),
            self.multiplicity * (eq33_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_34_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq34_e1016,) = {
    if ((s.v[668] != 0.0) && (!(s.v[669] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq34_value: f64 = eq34_e1016;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[5]),
            self.multiplicity * (eq34_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_35_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq35_e1032,) = {
    if ((s.v[668] != 0.0) && (!(s.v[669] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e1032;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[6]),
            self.multiplicity * (eq35_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_36_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq36_e1043,) = {
    if (s.v[670] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq36_value: f64 = eq36_e1043;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[6]),
            self.multiplicity * (eq36_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_37_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq37_e1054,) = {
    if (s.v[670] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq37_value: f64 = eq37_e1054;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[5]),
            self.multiplicity * (eq37_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_38_block_0(
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq38_e1079, eq38_e1079_d_n0, eq38_e1079_d_n1, eq38_e1079_d_n2, eq38_e1079_d_n3, eq38_e1079_d_n4, eq38_e1079_d_n5, eq38_e1079_d_n6, eq38_e1079_d_n7, eq38_e1079_d_n8,) = {
    if ((s.v[671] != 0.0) && (s.v[672] != 0.0)) {
        let eq38_e1060: f64 = (s.v[212] * s.v[27]);
        let eq38_e1060_d_n0: f64 = ((s.dn[212][0] * s.v[27]) + (s.v[212] * s.dn[27][0]));
        let eq38_e1060_d_n1: f64 = ((s.dn[212][1] * s.v[27]) + (s.v[212] * s.dn[27][1]));
        let eq38_e1060_d_n2: f64 = ((s.dn[212][2] * s.v[27]) + (s.v[212] * s.dn[27][2]));
        let eq38_e1060_d_n3: f64 = ((s.dn[212][3] * s.v[27]) + (s.v[212] * s.dn[27][3]));
        let eq38_e1060_d_n4: f64 = ((s.dn[212][4] * s.v[27]) + (s.v[212] * s.dn[27][4]));
        let eq38_e1060_d_n5: f64 = ((s.dn[212][5] * s.v[27]) + (s.v[212] * s.dn[27][5]));
        let eq38_e1060_d_n6: f64 = ((s.dn[212][6] * s.v[27]) + (s.v[212] * s.dn[27][6]));
        let eq38_e1060_d_n7: f64 = ((s.dn[212][7] * s.v[27]) + (s.v[212] * s.dn[27][7]));
        let eq38_e1060_d_n8: f64 = ((s.dn[212][8] * s.v[27]) + (s.v[212] * s.dn[27][8]));
        let eq38_e1062: f64 = (eq38_e1060 * (nv5 - nv6));
        let eq38_e1062_d_n0: f64 = (eq38_e1060_d_n0 * (nv5 - nv6));
        let eq38_e1062_d_n1: f64 = (eq38_e1060_d_n1 * (nv5 - nv6));
        let eq38_e1062_d_n2: f64 = (eq38_e1060_d_n2 * (nv5 - nv6));
        let eq38_e1062_d_n3: f64 = (eq38_e1060_d_n3 * (nv5 - nv6));
        let eq38_e1062_d_n4: f64 = (eq38_e1060_d_n4 * (nv5 - nv6));
        let eq38_e1062_d_n5: f64 = ((eq38_e1060_d_n5 * (nv5 - nv6)) + eq38_e1060);
        let eq38_e1062_d_n6: f64 = ((eq38_e1060_d_n6 * (nv5 - nv6)) + (-eq38_e1060));
        let eq38_e1062_d_n7: f64 = (eq38_e1060_d_n7 * (nv5 - nv6));
        let eq38_e1062_d_n8: f64 = (eq38_e1060_d_n8 * (nv5 - nv6));
        let eq38_e1064: f64 = (eq38_e1062 * s.v[214]);
        let eq38_e1064_d_n0: f64 = ((eq38_e1062_d_n0 * s.v[214]) + (eq38_e1062 * s.dn[214][0]));
        let eq38_e1064_d_n1: f64 = ((eq38_e1062_d_n1 * s.v[214]) + (eq38_e1062 * s.dn[214][1]));
        let eq38_e1064_d_n2: f64 = ((eq38_e1062_d_n2 * s.v[214]) + (eq38_e1062 * s.dn[214][2]));
        let eq38_e1064_d_n3: f64 = ((eq38_e1062_d_n3 * s.v[214]) + (eq38_e1062 * s.dn[214][3]));
        let eq38_e1064_d_n4: f64 = ((eq38_e1062_d_n4 * s.v[214]) + (eq38_e1062 * s.dn[214][4]));
        let eq38_e1064_d_n5: f64 = ((eq38_e1062_d_n5 * s.v[214]) + (eq38_e1062 * s.dn[214][5]));
        let eq38_e1064_d_n6: f64 = ((eq38_e1062_d_n6 * s.v[214]) + (eq38_e1062 * s.dn[214][6]));
        let eq38_e1064_d_n7: f64 = ((eq38_e1062_d_n7 * s.v[214]) + (eq38_e1062 * s.dn[214][7]));
        let eq38_e1064_d_n8: f64 = ((eq38_e1062_d_n8 * s.v[214]) + (eq38_e1062 * s.dn[214][8]));
        let eq38_e1067: f64 = ((nv0 - nv5) * (nv0 - nv5));
        let eq38_e1067_d_n0: f64 = ((nv0 - nv5) + (nv0 - nv5));
        let eq38_e1067_d_n5: f64 = ((-(nv0 - nv5)) + (-(nv0 - nv5)));
        let eq38_e1069: f64 = (eq38_e1067 / s.v[146]);
        let eq38_e1069_d_n0: f64 = (((eq38_e1067_d_n0 * s.v[146]) - (eq38_e1067 * s.dn[146][0])) / (s.v[146] * s.v[146]));
        let eq38_e1069_d_n1: f64 = (-((eq38_e1067 * s.dn[146][1]) / (s.v[146] * s.v[146])));
        let eq38_e1069_d_n2: f64 = (-((eq38_e1067 * s.dn[146][2]) / (s.v[146] * s.v[146])));
        let eq38_e1069_d_n3: f64 = (-((eq38_e1067 * s.dn[146][3]) / (s.v[146] * s.v[146])));
        let eq38_e1069_d_n4: f64 = (-((eq38_e1067 * s.dn[146][4]) / (s.v[146] * s.v[146])));
        let eq38_e1069_d_n5: f64 = (((eq38_e1067_d_n5 * s.v[146]) - (eq38_e1067 * s.dn[146][5])) / (s.v[146] * s.v[146]));
        let eq38_e1069_d_n6: f64 = (-((eq38_e1067 * s.dn[146][6]) / (s.v[146] * s.v[146])));
        let eq38_e1069_d_n7: f64 = (-((eq38_e1067 * s.dn[146][7]) / (s.v[146] * s.v[146])));
        let eq38_e1069_d_n8: f64 = (-((eq38_e1067 * s.dn[146][8]) / (s.v[146] * s.v[146])));
        let eq38_e1070: f64 = (eq38_e1064 + eq38_e1069);
        let eq38_e1070_d_n0: f64 = (eq38_e1064_d_n0 + eq38_e1069_d_n0);
        let eq38_e1070_d_n1: f64 = (eq38_e1064_d_n1 + eq38_e1069_d_n1);
        let eq38_e1070_d_n2: f64 = (eq38_e1064_d_n2 + eq38_e1069_d_n2);
        let eq38_e1070_d_n3: f64 = (eq38_e1064_d_n3 + eq38_e1069_d_n3);
        let eq38_e1070_d_n4: f64 = (eq38_e1064_d_n4 + eq38_e1069_d_n4);
        let eq38_e1070_d_n5: f64 = (eq38_e1064_d_n5 + eq38_e1069_d_n5);
        let eq38_e1070_d_n6: f64 = (eq38_e1064_d_n6 + eq38_e1069_d_n6);
        let eq38_e1070_d_n7: f64 = (eq38_e1064_d_n7 + eq38_e1069_d_n7);
        let eq38_e1070_d_n8: f64 = (eq38_e1064_d_n8 + eq38_e1069_d_n8);
        let eq38_e1073: f64 = ((nv2 - nv6) * (nv2 - nv6));
        let eq38_e1073_d_n2: f64 = ((nv2 - nv6) + (nv2 - nv6));
        let eq38_e1073_d_n6: f64 = ((-(nv2 - nv6)) + (-(nv2 - nv6)));
        let eq38_e1075: f64 = (eq38_e1073 / s.v[147]);
        let eq38_e1075_d_n0: f64 = (-((eq38_e1073 * s.dn[147][0]) / (s.v[147] * s.v[147])));
        let eq38_e1075_d_n1: f64 = (-((eq38_e1073 * s.dn[147][1]) / (s.v[147] * s.v[147])));
        let eq38_e1075_d_n2: f64 = (((eq38_e1073_d_n2 * s.v[147]) - (eq38_e1073 * s.dn[147][2])) / (s.v[147] * s.v[147]));
        let eq38_e1075_d_n3: f64 = (-((eq38_e1073 * s.dn[147][3]) / (s.v[147] * s.v[147])));
        let eq38_e1075_d_n4: f64 = (-((eq38_e1073 * s.dn[147][4]) / (s.v[147] * s.v[147])));
        let eq38_e1075_d_n5: f64 = (-((eq38_e1073 * s.dn[147][5]) / (s.v[147] * s.v[147])));
        let eq38_e1075_d_n6: f64 = (((eq38_e1073_d_n6 * s.v[147]) - (eq38_e1073 * s.dn[147][6])) / (s.v[147] * s.v[147]));
        let eq38_e1075_d_n7: f64 = (-((eq38_e1073 * s.dn[147][7]) / (s.v[147] * s.v[147])));
        let eq38_e1075_d_n8: f64 = (-((eq38_e1073 * s.dn[147][8]) / (s.v[147] * s.v[147])));
        let eq38_e1076: f64 = (eq38_e1070 + eq38_e1075);
        let eq38_e1076_d_n0: f64 = (eq38_e1070_d_n0 + eq38_e1075_d_n0);
        let eq38_e1076_d_n1: f64 = (eq38_e1070_d_n1 + eq38_e1075_d_n1);
        let eq38_e1076_d_n2: f64 = (eq38_e1070_d_n2 + eq38_e1075_d_n2);
        let eq38_e1076_d_n3: f64 = (eq38_e1070_d_n3 + eq38_e1075_d_n3);
        let eq38_e1076_d_n4: f64 = (eq38_e1070_d_n4 + eq38_e1075_d_n4);
        let eq38_e1076_d_n5: f64 = (eq38_e1070_d_n5 + eq38_e1075_d_n5);
        let eq38_e1076_d_n6: f64 = (eq38_e1070_d_n6 + eq38_e1075_d_n6);
        let eq38_e1076_d_n7: f64 = (eq38_e1070_d_n7 + eq38_e1075_d_n7);
        let eq38_e1076_d_n8: f64 = (eq38_e1070_d_n8 + eq38_e1075_d_n8);
        let eq38_e1077: f64 = (-eq38_e1076);
        let eq38_e1077_d_n0: f64 = (-eq38_e1076_d_n0);
        let eq38_e1077_d_n1: f64 = (-eq38_e1076_d_n1);
        let eq38_e1077_d_n2: f64 = (-eq38_e1076_d_n2);
        let eq38_e1077_d_n3: f64 = (-eq38_e1076_d_n3);
        let eq38_e1077_d_n4: f64 = (-eq38_e1076_d_n4);
        let eq38_e1077_d_n5: f64 = (-eq38_e1076_d_n5);
        let eq38_e1077_d_n6: f64 = (-eq38_e1076_d_n6);
        let eq38_e1077_d_n7: f64 = (-eq38_e1076_d_n7);
        let eq38_e1077_d_n8: f64 = (-eq38_e1076_d_n8);
        (eq38_e1077, eq38_e1077_d_n0, eq38_e1077_d_n1, eq38_e1077_d_n2, eq38_e1077_d_n3, eq38_e1077_d_n4, eq38_e1077_d_n5, eq38_e1077_d_n6, eq38_e1077_d_n7, eq38_e1077_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e1079;
        let eq38_node_derivatives: [f64; 9] = [eq38_e1079_d_n0, eq38_e1079_d_n1, eq38_e1079_d_n2, eq38_e1079_d_n3, eq38_e1079_d_n4, eq38_e1079_d_n5, eq38_e1079_d_n6, eq38_e1079_d_n7, eq38_e1079_d_n8];
        let eq38_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq38_value),
            &nodes,
            &eq38_node_derivatives,
            &branches,
            &eq38_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_39_block_0(
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
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq39_e1093, eq39_e1093_d_n0, eq39_e1093_d_n1, eq39_e1093_d_n2, eq39_e1093_d_n3, eq39_e1093_d_n4, eq39_e1093_d_n5, eq39_e1093_d_n6, eq39_e1093_d_n7, eq39_e1093_d_n8,) = {
    if ((s.v[671] != 0.0) && (!(s.v[672] != 0.0))) {
        let eq39_e1086: f64 = (s.v[212] * s.v[27]);
        let eq39_e1086_d_n0: f64 = ((s.dn[212][0] * s.v[27]) + (s.v[212] * s.dn[27][0]));
        let eq39_e1086_d_n1: f64 = ((s.dn[212][1] * s.v[27]) + (s.v[212] * s.dn[27][1]));
        let eq39_e1086_d_n2: f64 = ((s.dn[212][2] * s.v[27]) + (s.v[212] * s.dn[27][2]));
        let eq39_e1086_d_n3: f64 = ((s.dn[212][3] * s.v[27]) + (s.v[212] * s.dn[27][3]));
        let eq39_e1086_d_n4: f64 = ((s.dn[212][4] * s.v[27]) + (s.v[212] * s.dn[27][4]));
        let eq39_e1086_d_n5: f64 = ((s.dn[212][5] * s.v[27]) + (s.v[212] * s.dn[27][5]));
        let eq39_e1086_d_n6: f64 = ((s.dn[212][6] * s.v[27]) + (s.v[212] * s.dn[27][6]));
        let eq39_e1086_d_n7: f64 = ((s.dn[212][7] * s.v[27]) + (s.v[212] * s.dn[27][7]));
        let eq39_e1086_d_n8: f64 = ((s.dn[212][8] * s.v[27]) + (s.v[212] * s.dn[27][8]));
        let eq39_e1088: f64 = (eq39_e1086 * (nv5 - nv6));
        let eq39_e1088_d_n0: f64 = (eq39_e1086_d_n0 * (nv5 - nv6));
        let eq39_e1088_d_n1: f64 = (eq39_e1086_d_n1 * (nv5 - nv6));
        let eq39_e1088_d_n2: f64 = (eq39_e1086_d_n2 * (nv5 - nv6));
        let eq39_e1088_d_n3: f64 = (eq39_e1086_d_n3 * (nv5 - nv6));
        let eq39_e1088_d_n4: f64 = (eq39_e1086_d_n4 * (nv5 - nv6));
        let eq39_e1088_d_n5: f64 = ((eq39_e1086_d_n5 * (nv5 - nv6)) + eq39_e1086);
        let eq39_e1088_d_n6: f64 = ((eq39_e1086_d_n6 * (nv5 - nv6)) + (-eq39_e1086));
        let eq39_e1088_d_n7: f64 = (eq39_e1086_d_n7 * (nv5 - nv6));
        let eq39_e1088_d_n8: f64 = (eq39_e1086_d_n8 * (nv5 - nv6));
        let eq39_e1090: f64 = (eq39_e1088 * s.v[214]);
        let eq39_e1090_d_n0: f64 = ((eq39_e1088_d_n0 * s.v[214]) + (eq39_e1088 * s.dn[214][0]));
        let eq39_e1090_d_n1: f64 = ((eq39_e1088_d_n1 * s.v[214]) + (eq39_e1088 * s.dn[214][1]));
        let eq39_e1090_d_n2: f64 = ((eq39_e1088_d_n2 * s.v[214]) + (eq39_e1088 * s.dn[214][2]));
        let eq39_e1090_d_n3: f64 = ((eq39_e1088_d_n3 * s.v[214]) + (eq39_e1088 * s.dn[214][3]));
        let eq39_e1090_d_n4: f64 = ((eq39_e1088_d_n4 * s.v[214]) + (eq39_e1088 * s.dn[214][4]));
        let eq39_e1090_d_n5: f64 = ((eq39_e1088_d_n5 * s.v[214]) + (eq39_e1088 * s.dn[214][5]));
        let eq39_e1090_d_n6: f64 = ((eq39_e1088_d_n6 * s.v[214]) + (eq39_e1088 * s.dn[214][6]));
        let eq39_e1090_d_n7: f64 = ((eq39_e1088_d_n7 * s.v[214]) + (eq39_e1088 * s.dn[214][7]));
        let eq39_e1090_d_n8: f64 = ((eq39_e1088_d_n8 * s.v[214]) + (eq39_e1088 * s.dn[214][8]));
        let eq39_e1091: f64 = (-eq39_e1090);
        let eq39_e1091_d_n0: f64 = (-eq39_e1090_d_n0);
        let eq39_e1091_d_n1: f64 = (-eq39_e1090_d_n1);
        let eq39_e1091_d_n2: f64 = (-eq39_e1090_d_n2);
        let eq39_e1091_d_n3: f64 = (-eq39_e1090_d_n3);
        let eq39_e1091_d_n4: f64 = (-eq39_e1090_d_n4);
        let eq39_e1091_d_n5: f64 = (-eq39_e1090_d_n5);
        let eq39_e1091_d_n6: f64 = (-eq39_e1090_d_n6);
        let eq39_e1091_d_n7: f64 = (-eq39_e1090_d_n7);
        let eq39_e1091_d_n8: f64 = (-eq39_e1090_d_n8);
        (eq39_e1091, eq39_e1091_d_n0, eq39_e1091_d_n1, eq39_e1091_d_n2, eq39_e1091_d_n3, eq39_e1091_d_n4, eq39_e1091_d_n5, eq39_e1091_d_n6, eq39_e1091_d_n7, eq39_e1091_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e1093;
        let eq39_node_derivatives: [f64; 9] = [eq39_e1093_d_n0, eq39_e1093_d_n1, eq39_e1093_d_n2, eq39_e1093_d_n3, eq39_e1093_d_n4, eq39_e1093_d_n5, eq39_e1093_d_n6, eq39_e1093_d_n7, eq39_e1093_d_n8];
        let eq39_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq39_value),
            &nodes,
            &eq39_node_derivatives,
            &branches,
            &eq39_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_40_block_0(
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
        let (eq40_e1099, eq40_e1099_d_n0, eq40_e1099_d_n1, eq40_e1099_d_n2, eq40_e1099_d_n3, eq40_e1099_d_n4, eq40_e1099_d_n5, eq40_e1099_d_n6, eq40_e1099_d_n7, eq40_e1099_d_n8,) = {
    if (s.v[671] != 0.0) {
        let eq40_e1097: f64 = ((nv4 - 0.0) * s.v[269]);
        let eq40_e1097_d_n0: f64 = ((nv4 - 0.0) * s.dn[269][0]);
        let eq40_e1097_d_n1: f64 = ((nv4 - 0.0) * s.dn[269][1]);
        let eq40_e1097_d_n2: f64 = ((nv4 - 0.0) * s.dn[269][2]);
        let eq40_e1097_d_n3: f64 = ((nv4 - 0.0) * s.dn[269][3]);
        let eq40_e1097_d_n4: f64 = (s.v[269] + ((nv4 - 0.0) * s.dn[269][4]));
        let eq40_e1097_d_n5: f64 = ((nv4 - 0.0) * s.dn[269][5]);
        let eq40_e1097_d_n6: f64 = ((nv4 - 0.0) * s.dn[269][6]);
        let eq40_e1097_d_n7: f64 = ((nv4 - 0.0) * s.dn[269][7]);
        let eq40_e1097_d_n8: f64 = ((nv4 - 0.0) * s.dn[269][8]);
        (eq40_e1097, eq40_e1097_d_n0, eq40_e1097_d_n1, eq40_e1097_d_n2, eq40_e1097_d_n3, eq40_e1097_d_n4, eq40_e1097_d_n5, eq40_e1097_d_n6, eq40_e1097_d_n7, eq40_e1097_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_value: f64 = eq40_e1099;
        let eq40_node_derivatives: [f64; 9] = [eq40_e1099_d_n0, eq40_e1099_d_n1, eq40_e1099_d_n2, eq40_e1099_d_n3, eq40_e1099_d_n4, eq40_e1099_d_n5, eq40_e1099_d_n6, eq40_e1099_d_n7, eq40_e1099_d_n8];
        let eq40_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq40_value),
            &nodes,
            &eq40_node_derivatives,
            &branches,
            &eq40_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_41_block_0(
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
        let (eq41_e1106, eq41_e1106_d_n0, eq41_e1106_d_n1, eq41_e1106_d_n2, eq41_e1106_d_n3, eq41_e1106_d_n4, eq41_e1106_d_n5, eq41_e1106_d_n6, eq41_e1106_d_n7, eq41_e1106_d_n8,) = {
    if (s.v[671] != 0.0) {
        let eq41_e1103: f64 = ((nv4 - 0.0) * s.v[270]);
        let eq41_e1103_d_n0: f64 = ((nv4 - 0.0) * s.dn[270][0]);
        let eq41_e1103_d_n1: f64 = ((nv4 - 0.0) * s.dn[270][1]);
        let eq41_e1103_d_n2: f64 = ((nv4 - 0.0) * s.dn[270][2]);
        let eq41_e1103_d_n3: f64 = ((nv4 - 0.0) * s.dn[270][3]);
        let eq41_e1103_d_n4: f64 = (s.v[270] + ((nv4 - 0.0) * s.dn[270][4]));
        let eq41_e1103_d_n5: f64 = ((nv4 - 0.0) * s.dn[270][5]);
        let eq41_e1103_d_n6: f64 = ((nv4 - 0.0) * s.dn[270][6]);
        let eq41_e1103_d_n7: f64 = ((nv4 - 0.0) * s.dn[270][7]);
        let eq41_e1103_d_n8: f64 = ((nv4 - 0.0) * s.dn[270][8]);
        let eq41_e1104: f64 = self.eval_ddt(7, eq41_e1103);
        let eq41_e1104_d_n0: f64 = self.ddt_jacobian(eq41_e1103_d_n0);
        let eq41_e1104_d_n1: f64 = self.ddt_jacobian(eq41_e1103_d_n1);
        let eq41_e1104_d_n2: f64 = self.ddt_jacobian(eq41_e1103_d_n2);
        let eq41_e1104_d_n3: f64 = self.ddt_jacobian(eq41_e1103_d_n3);
        let eq41_e1104_d_n4: f64 = self.ddt_jacobian(eq41_e1103_d_n4);
        let eq41_e1104_d_n5: f64 = self.ddt_jacobian(eq41_e1103_d_n5);
        let eq41_e1104_d_n6: f64 = self.ddt_jacobian(eq41_e1103_d_n6);
        let eq41_e1104_d_n7: f64 = self.ddt_jacobian(eq41_e1103_d_n7);
        let eq41_e1104_d_n8: f64 = self.ddt_jacobian(eq41_e1103_d_n8);
        (eq41_e1104, eq41_e1104_d_n0, eq41_e1104_d_n1, eq41_e1104_d_n2, eq41_e1104_d_n3, eq41_e1104_d_n4, eq41_e1104_d_n5, eq41_e1104_d_n6, eq41_e1104_d_n7, eq41_e1104_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_value: f64 = eq41_e1106;
        let eq41_node_derivatives: [f64; 9] = [eq41_e1106_d_n0, eq41_e1106_d_n1, eq41_e1106_d_n2, eq41_e1106_d_n3, eq41_e1106_d_n4, eq41_e1106_d_n5, eq41_e1106_d_n6, eq41_e1106_d_n7, eq41_e1106_d_n8];
        let eq41_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq41_value),
            &nodes,
            &eq41_node_derivatives,
            &branches,
            &eq41_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_42_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq42_e1111,) = {
    if (!(s.v[671] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq42_value: f64 = eq42_e1111;
        stamper.stamp_potential(
            branches[4],
            eq42_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_12_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq12_e873_q: f64 = s.v[223];
        let eq12_e874: f64 = (s.v[212] * s.v[223]);
        let eq12_e874_d_n0: f64 = ((s.dn[212][0] * s.v[223]) + (s.v[212] * s.dn[223][0]));
        let eq12_e874_d_n1: f64 = ((s.dn[212][1] * s.v[223]) + (s.v[212] * s.dn[223][1]));
        let eq12_e874_d_n2: f64 = ((s.dn[212][2] * s.v[223]) + (s.v[212] * s.dn[223][2]));
        let eq12_e874_d_n3: f64 = ((s.dn[212][3] * s.v[223]) + (s.v[212] * s.dn[223][3]));
        let eq12_e874_d_n4: f64 = ((s.dn[212][4] * s.v[223]) + (s.v[212] * s.dn[223][4]));
        let eq12_e874_d_n5: f64 = ((s.dn[212][5] * s.v[223]) + (s.v[212] * s.dn[223][5]));
        let eq12_e874_d_n6: f64 = ((s.dn[212][6] * s.v[223]) + (s.v[212] * s.dn[223][6]));
        let eq12_e874_d_n7: f64 = ((s.dn[212][7] * s.v[223]) + (s.v[212] * s.dn[223][7]));
        let eq12_e874_d_n8: f64 = ((s.dn[212][8] * s.v[223]) + (s.v[212] * s.dn[223][8]));
        let eq12_e874_q: f64 = (s.v[212] * eq12_e873_q);
        let eq12_e874_q_d_n0: f64 = ((s.dn[212][0] * eq12_e873_q) + (s.v[212] * s.dn[223][0]));
        let eq12_e874_q_d_n1: f64 = ((s.dn[212][1] * eq12_e873_q) + (s.v[212] * s.dn[223][1]));
        let eq12_e874_q_d_n2: f64 = ((s.dn[212][2] * eq12_e873_q) + (s.v[212] * s.dn[223][2]));
        let eq12_e874_q_d_n3: f64 = ((s.dn[212][3] * eq12_e873_q) + (s.v[212] * s.dn[223][3]));
        let eq12_e874_q_d_n4: f64 = ((s.dn[212][4] * eq12_e873_q) + (s.v[212] * s.dn[223][4]));
        let eq12_e874_q_d_n5: f64 = ((s.dn[212][5] * eq12_e873_q) + (s.v[212] * s.dn[223][5]));
        let eq12_e874_q_d_n6: f64 = ((s.dn[212][6] * eq12_e873_q) + (s.v[212] * s.dn[223][6]));
        let eq12_e874_q_d_n7: f64 = ((s.dn[212][7] * eq12_e873_q) + (s.v[212] * s.dn[223][7]));
        let eq12_e874_q_d_n8: f64 = ((s.dn[212][8] * eq12_e873_q) + (s.v[212] * s.dn[223][8]));
        let eq12_reactive_node_derivatives: [f64; 9] = [eq12_e874_q_d_n0, eq12_e874_q_d_n1, eq12_e874_q_d_n2, eq12_e874_q_d_n3, eq12_e874_q_d_n4, eq12_e874_q_d_n5, eq12_e874_q_d_n6, eq12_e874_q_d_n7, eq12_e874_q_d_n8];
        let eq12_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            &nodes,
            &eq12_reactive_node_derivatives,
            &branches,
            &eq12_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_13_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq13_e876_q: f64 = s.v[224];
        let eq13_reactive_node_derivatives: [f64; 9] = [s.dn[224][0], s.dn[224][1], s.dn[224][2], s.dn[224][3], s.dn[224][4], s.dn[224][5], s.dn[224][6], s.dn[224][7], s.dn[224][8]];
        let eq13_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            &nodes,
            &eq13_reactive_node_derivatives,
            &branches,
            &eq13_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_14_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq14_e879_q: f64 = s.v[225];
        let eq14_e880: f64 = (s.v[212] * s.v[225]);
        let eq14_e880_d_n0: f64 = ((s.dn[212][0] * s.v[225]) + (s.v[212] * s.dn[225][0]));
        let eq14_e880_d_n1: f64 = ((s.dn[212][1] * s.v[225]) + (s.v[212] * s.dn[225][1]));
        let eq14_e880_d_n2: f64 = ((s.dn[212][2] * s.v[225]) + (s.v[212] * s.dn[225][2]));
        let eq14_e880_d_n3: f64 = ((s.dn[212][3] * s.v[225]) + (s.v[212] * s.dn[225][3]));
        let eq14_e880_d_n4: f64 = ((s.dn[212][4] * s.v[225]) + (s.v[212] * s.dn[225][4]));
        let eq14_e880_d_n5: f64 = ((s.dn[212][5] * s.v[225]) + (s.v[212] * s.dn[225][5]));
        let eq14_e880_d_n6: f64 = ((s.dn[212][6] * s.v[225]) + (s.v[212] * s.dn[225][6]));
        let eq14_e880_d_n7: f64 = ((s.dn[212][7] * s.v[225]) + (s.v[212] * s.dn[225][7]));
        let eq14_e880_d_n8: f64 = ((s.dn[212][8] * s.v[225]) + (s.v[212] * s.dn[225][8]));
        let eq14_e880_q: f64 = (s.v[212] * eq14_e879_q);
        let eq14_e880_q_d_n0: f64 = ((s.dn[212][0] * eq14_e879_q) + (s.v[212] * s.dn[225][0]));
        let eq14_e880_q_d_n1: f64 = ((s.dn[212][1] * eq14_e879_q) + (s.v[212] * s.dn[225][1]));
        let eq14_e880_q_d_n2: f64 = ((s.dn[212][2] * eq14_e879_q) + (s.v[212] * s.dn[225][2]));
        let eq14_e880_q_d_n3: f64 = ((s.dn[212][3] * eq14_e879_q) + (s.v[212] * s.dn[225][3]));
        let eq14_e880_q_d_n4: f64 = ((s.dn[212][4] * eq14_e879_q) + (s.v[212] * s.dn[225][4]));
        let eq14_e880_q_d_n5: f64 = ((s.dn[212][5] * eq14_e879_q) + (s.v[212] * s.dn[225][5]));
        let eq14_e880_q_d_n6: f64 = ((s.dn[212][6] * eq14_e879_q) + (s.v[212] * s.dn[225][6]));
        let eq14_e880_q_d_n7: f64 = ((s.dn[212][7] * eq14_e879_q) + (s.v[212] * s.dn[225][7]));
        let eq14_e880_q_d_n8: f64 = ((s.dn[212][8] * eq14_e879_q) + (s.v[212] * s.dn[225][8]));
        let eq14_reactive_node_derivatives: [f64; 9] = [eq14_e880_q_d_n0, eq14_e880_q_d_n1, eq14_e880_q_d_n2, eq14_e880_q_d_n3, eq14_e880_q_d_n4, eq14_e880_q_d_n5, eq14_e880_q_d_n6, eq14_e880_q_d_n7, eq14_e880_q_d_n8];
        let eq14_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[6]),
            &nodes,
            &eq14_reactive_node_derivatives,
            &branches,
            &eq14_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_15_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq15_e882_q: f64 = s.v[226];
        let eq15_reactive_node_derivatives: [f64; 9] = [s.dn[226][0], s.dn[226][1], s.dn[226][2], s.dn[226][3], s.dn[226][4], s.dn[226][5], s.dn[226][6], s.dn[226][7], s.dn[226][8]];
        let eq15_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            &nodes,
            &eq15_reactive_node_derivatives,
            &branches,
            &eq15_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_16_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq16_e884_q: f64 = s.v[227];
        let eq16_reactive_node_derivatives: [f64; 9] = [s.dn[227][0], s.dn[227][1], s.dn[227][2], s.dn[227][3], s.dn[227][4], s.dn[227][5], s.dn[227][6], s.dn[227][7], s.dn[227][8]];
        let eq16_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            &nodes,
            &eq16_reactive_node_derivatives,
            &branches,
            &eq16_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_17_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq17_e887_q: f64 = s.v[238];
        let eq17_e888: f64 = (s.v[212] * s.v[238]);
        let eq17_e888_d_n0: f64 = ((s.dn[212][0] * s.v[238]) + (s.v[212] * s.dn[238][0]));
        let eq17_e888_d_n1: f64 = ((s.dn[212][1] * s.v[238]) + (s.v[212] * s.dn[238][1]));
        let eq17_e888_d_n2: f64 = ((s.dn[212][2] * s.v[238]) + (s.v[212] * s.dn[238][2]));
        let eq17_e888_d_n3: f64 = ((s.dn[212][3] * s.v[238]) + (s.v[212] * s.dn[238][3]));
        let eq17_e888_d_n4: f64 = ((s.dn[212][4] * s.v[238]) + (s.v[212] * s.dn[238][4]));
        let eq17_e888_d_n5: f64 = ((s.dn[212][5] * s.v[238]) + (s.v[212] * s.dn[238][5]));
        let eq17_e888_d_n6: f64 = ((s.dn[212][6] * s.v[238]) + (s.v[212] * s.dn[238][6]));
        let eq17_e888_d_n7: f64 = ((s.dn[212][7] * s.v[238]) + (s.v[212] * s.dn[238][7]));
        let eq17_e888_d_n8: f64 = ((s.dn[212][8] * s.v[238]) + (s.v[212] * s.dn[238][8]));
        let eq17_e888_q: f64 = (s.v[212] * eq17_e887_q);
        let eq17_e888_q_d_n0: f64 = ((s.dn[212][0] * eq17_e887_q) + (s.v[212] * s.dn[238][0]));
        let eq17_e888_q_d_n1: f64 = ((s.dn[212][1] * eq17_e887_q) + (s.v[212] * s.dn[238][1]));
        let eq17_e888_q_d_n2: f64 = ((s.dn[212][2] * eq17_e887_q) + (s.v[212] * s.dn[238][2]));
        let eq17_e888_q_d_n3: f64 = ((s.dn[212][3] * eq17_e887_q) + (s.v[212] * s.dn[238][3]));
        let eq17_e888_q_d_n4: f64 = ((s.dn[212][4] * eq17_e887_q) + (s.v[212] * s.dn[238][4]));
        let eq17_e888_q_d_n5: f64 = ((s.dn[212][5] * eq17_e887_q) + (s.v[212] * s.dn[238][5]));
        let eq17_e888_q_d_n6: f64 = ((s.dn[212][6] * eq17_e887_q) + (s.v[212] * s.dn[238][6]));
        let eq17_e888_q_d_n7: f64 = ((s.dn[212][7] * eq17_e887_q) + (s.v[212] * s.dn[238][7]));
        let eq17_e888_q_d_n8: f64 = ((s.dn[212][8] * eq17_e887_q) + (s.v[212] * s.dn[238][8]));
        let eq17_reactive_node_derivatives: [f64; 9] = [eq17_e888_q_d_n0, eq17_e888_q_d_n1, eq17_e888_q_d_n2, eq17_e888_q_d_n3, eq17_e888_q_d_n4, eq17_e888_q_d_n5, eq17_e888_q_d_n6, eq17_e888_q_d_n7, eq17_e888_q_d_n8];
        let eq17_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[3]),
            &nodes,
            &eq17_reactive_node_derivatives,
            &branches,
            &eq17_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
