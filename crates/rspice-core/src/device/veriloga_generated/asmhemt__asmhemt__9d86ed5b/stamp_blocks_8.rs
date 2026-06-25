#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_71_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq71_e1150,) = {
    if (s.v[432] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq71_value: f64 = eq71_e1150;
        stamper.stamp_current(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq71_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_72_block_0(
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq72_e1166, eq72_e1166_d_n0, eq72_e1166_d_n1, eq72_e1166_d_n2, eq72_e1166_d_n3, eq72_e1166_d_n4, eq72_e1166_d_n5, eq72_e1166_d_n6, eq72_e1166_d_n7, eq72_e1166_d_n8, eq72_e1166_d_n9, eq72_e1166_d_n10, eq72_e1166_d_n11, eq72_e1166_d_n12, eq72_e1166_d_n13, eq72_e1166_d_n14, eq72_e1166_d_n15, eq72_e1166_d_n16, eq72_e1166_d_n17, eq72_e1166_d_n18, eq72_e1166_d_n19, eq72_e1166_d_n20, eq72_e1166_d_n21, eq72_e1166_d_n22,) = {
    if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
        let eq72_e1156: f64 = (p.p6 * s.v[48]);
        let eq72_e1156_d_n0: f64 = (p.p6 * s.dn[48][0]);
        let eq72_e1156_d_n1: f64 = (p.p6 * s.dn[48][1]);
        let eq72_e1156_d_n2: f64 = (p.p6 * s.dn[48][2]);
        let eq72_e1156_d_n3: f64 = (p.p6 * s.dn[48][3]);
        let eq72_e1156_d_n4: f64 = (p.p6 * s.dn[48][4]);
        let eq72_e1156_d_n5: f64 = (p.p6 * s.dn[48][5]);
        let eq72_e1156_d_n6: f64 = (p.p6 * s.dn[48][6]);
        let eq72_e1156_d_n7: f64 = (p.p6 * s.dn[48][7]);
        let eq72_e1156_d_n8: f64 = (p.p6 * s.dn[48][8]);
        let eq72_e1156_d_n9: f64 = (p.p6 * s.dn[48][9]);
        let eq72_e1156_d_n10: f64 = (p.p6 * s.dn[48][10]);
        let eq72_e1156_d_n11: f64 = (p.p6 * s.dn[48][11]);
        let eq72_e1156_d_n12: f64 = (p.p6 * s.dn[48][12]);
        let eq72_e1156_d_n13: f64 = (p.p6 * s.dn[48][13]);
        let eq72_e1156_d_n14: f64 = (p.p6 * s.dn[48][14]);
        let eq72_e1156_d_n15: f64 = (p.p6 * s.dn[48][15]);
        let eq72_e1156_d_n16: f64 = (p.p6 * s.dn[48][16]);
        let eq72_e1156_d_n17: f64 = (p.p6 * s.dn[48][17]);
        let eq72_e1156_d_n18: f64 = (p.p6 * s.dn[48][18]);
        let eq72_e1156_d_n19: f64 = (p.p6 * s.dn[48][19]);
        let eq72_e1156_d_n20: f64 = (p.p6 * s.dn[48][20]);
        let eq72_e1156_d_n21: f64 = (p.p6 * s.dn[48][21]);
        let eq72_e1156_d_n22: f64 = (p.p6 * s.dn[48][22]);
        let eq72_e1158: f64 = (eq72_e1156 * s.v[233]);
        let eq72_e1158_d_n0: f64 = ((eq72_e1156_d_n0 * s.v[233]) + (eq72_e1156 * s.dn[233][0]));
        let eq72_e1158_d_n1: f64 = ((eq72_e1156_d_n1 * s.v[233]) + (eq72_e1156 * s.dn[233][1]));
        let eq72_e1158_d_n2: f64 = ((eq72_e1156_d_n2 * s.v[233]) + (eq72_e1156 * s.dn[233][2]));
        let eq72_e1158_d_n3: f64 = ((eq72_e1156_d_n3 * s.v[233]) + (eq72_e1156 * s.dn[233][3]));
        let eq72_e1158_d_n4: f64 = ((eq72_e1156_d_n4 * s.v[233]) + (eq72_e1156 * s.dn[233][4]));
        let eq72_e1158_d_n5: f64 = ((eq72_e1156_d_n5 * s.v[233]) + (eq72_e1156 * s.dn[233][5]));
        let eq72_e1158_d_n6: f64 = ((eq72_e1156_d_n6 * s.v[233]) + (eq72_e1156 * s.dn[233][6]));
        let eq72_e1158_d_n7: f64 = ((eq72_e1156_d_n7 * s.v[233]) + (eq72_e1156 * s.dn[233][7]));
        let eq72_e1158_d_n8: f64 = ((eq72_e1156_d_n8 * s.v[233]) + (eq72_e1156 * s.dn[233][8]));
        let eq72_e1158_d_n9: f64 = ((eq72_e1156_d_n9 * s.v[233]) + (eq72_e1156 * s.dn[233][9]));
        let eq72_e1158_d_n10: f64 = ((eq72_e1156_d_n10 * s.v[233]) + (eq72_e1156 * s.dn[233][10]));
        let eq72_e1158_d_n11: f64 = ((eq72_e1156_d_n11 * s.v[233]) + (eq72_e1156 * s.dn[233][11]));
        let eq72_e1158_d_n12: f64 = ((eq72_e1156_d_n12 * s.v[233]) + (eq72_e1156 * s.dn[233][12]));
        let eq72_e1158_d_n13: f64 = ((eq72_e1156_d_n13 * s.v[233]) + (eq72_e1156 * s.dn[233][13]));
        let eq72_e1158_d_n14: f64 = ((eq72_e1156_d_n14 * s.v[233]) + (eq72_e1156 * s.dn[233][14]));
        let eq72_e1158_d_n15: f64 = ((eq72_e1156_d_n15 * s.v[233]) + (eq72_e1156 * s.dn[233][15]));
        let eq72_e1158_d_n16: f64 = ((eq72_e1156_d_n16 * s.v[233]) + (eq72_e1156 * s.dn[233][16]));
        let eq72_e1158_d_n17: f64 = ((eq72_e1156_d_n17 * s.v[233]) + (eq72_e1156 * s.dn[233][17]));
        let eq72_e1158_d_n18: f64 = ((eq72_e1156_d_n18 * s.v[233]) + (eq72_e1156 * s.dn[233][18]));
        let eq72_e1158_d_n19: f64 = ((eq72_e1156_d_n19 * s.v[233]) + (eq72_e1156 * s.dn[233][19]));
        let eq72_e1158_d_n20: f64 = ((eq72_e1156_d_n20 * s.v[233]) + (eq72_e1156 * s.dn[233][20]));
        let eq72_e1158_d_n21: f64 = ((eq72_e1156_d_n21 * s.v[233]) + (eq72_e1156 * s.dn[233][21]));
        let eq72_e1158_d_n22: f64 = ((eq72_e1156_d_n22 * s.v[233]) + (eq72_e1156 * s.dn[233][22]));
        let eq72_e1161: f64 = (p.p6 * s.v[379]);
        let eq72_e1161_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq72_e1161_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq72_e1161_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq72_e1161_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq72_e1161_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq72_e1161_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq72_e1161_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq72_e1161_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq72_e1161_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq72_e1161_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq72_e1161_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq72_e1161_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq72_e1161_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq72_e1161_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq72_e1161_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq72_e1161_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq72_e1161_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq72_e1161_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq72_e1161_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq72_e1161_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq72_e1161_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq72_e1161_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq72_e1161_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq72_e1163: f64 = (eq72_e1161 * (nv15 - nv7));
        let eq72_e1163_d_n0: f64 = (eq72_e1161_d_n0 * (nv15 - nv7));
        let eq72_e1163_d_n1: f64 = (eq72_e1161_d_n1 * (nv15 - nv7));
        let eq72_e1163_d_n2: f64 = (eq72_e1161_d_n2 * (nv15 - nv7));
        let eq72_e1163_d_n3: f64 = (eq72_e1161_d_n3 * (nv15 - nv7));
        let eq72_e1163_d_n4: f64 = (eq72_e1161_d_n4 * (nv15 - nv7));
        let eq72_e1163_d_n5: f64 = (eq72_e1161_d_n5 * (nv15 - nv7));
        let eq72_e1163_d_n6: f64 = (eq72_e1161_d_n6 * (nv15 - nv7));
        let eq72_e1163_d_n7: f64 = ((eq72_e1161_d_n7 * (nv15 - nv7)) + (-eq72_e1161));
        let eq72_e1163_d_n8: f64 = (eq72_e1161_d_n8 * (nv15 - nv7));
        let eq72_e1163_d_n9: f64 = (eq72_e1161_d_n9 * (nv15 - nv7));
        let eq72_e1163_d_n10: f64 = (eq72_e1161_d_n10 * (nv15 - nv7));
        let eq72_e1163_d_n11: f64 = (eq72_e1161_d_n11 * (nv15 - nv7));
        let eq72_e1163_d_n12: f64 = (eq72_e1161_d_n12 * (nv15 - nv7));
        let eq72_e1163_d_n13: f64 = (eq72_e1161_d_n13 * (nv15 - nv7));
        let eq72_e1163_d_n14: f64 = (eq72_e1161_d_n14 * (nv15 - nv7));
        let eq72_e1163_d_n15: f64 = ((eq72_e1161_d_n15 * (nv15 - nv7)) + eq72_e1161);
        let eq72_e1163_d_n16: f64 = (eq72_e1161_d_n16 * (nv15 - nv7));
        let eq72_e1163_d_n17: f64 = (eq72_e1161_d_n17 * (nv15 - nv7));
        let eq72_e1163_d_n18: f64 = (eq72_e1161_d_n18 * (nv15 - nv7));
        let eq72_e1163_d_n19: f64 = (eq72_e1161_d_n19 * (nv15 - nv7));
        let eq72_e1163_d_n20: f64 = (eq72_e1161_d_n20 * (nv15 - nv7));
        let eq72_e1163_d_n21: f64 = (eq72_e1161_d_n21 * (nv15 - nv7));
        let eq72_e1163_d_n22: f64 = (eq72_e1161_d_n22 * (nv15 - nv7));
        let eq72_e1164: f64 = (eq72_e1158 + eq72_e1163);
        let eq72_e1164_d_n0: f64 = (eq72_e1158_d_n0 + eq72_e1163_d_n0);
        let eq72_e1164_d_n1: f64 = (eq72_e1158_d_n1 + eq72_e1163_d_n1);
        let eq72_e1164_d_n2: f64 = (eq72_e1158_d_n2 + eq72_e1163_d_n2);
        let eq72_e1164_d_n3: f64 = (eq72_e1158_d_n3 + eq72_e1163_d_n3);
        let eq72_e1164_d_n4: f64 = (eq72_e1158_d_n4 + eq72_e1163_d_n4);
        let eq72_e1164_d_n5: f64 = (eq72_e1158_d_n5 + eq72_e1163_d_n5);
        let eq72_e1164_d_n6: f64 = (eq72_e1158_d_n6 + eq72_e1163_d_n6);
        let eq72_e1164_d_n7: f64 = (eq72_e1158_d_n7 + eq72_e1163_d_n7);
        let eq72_e1164_d_n8: f64 = (eq72_e1158_d_n8 + eq72_e1163_d_n8);
        let eq72_e1164_d_n9: f64 = (eq72_e1158_d_n9 + eq72_e1163_d_n9);
        let eq72_e1164_d_n10: f64 = (eq72_e1158_d_n10 + eq72_e1163_d_n10);
        let eq72_e1164_d_n11: f64 = (eq72_e1158_d_n11 + eq72_e1163_d_n11);
        let eq72_e1164_d_n12: f64 = (eq72_e1158_d_n12 + eq72_e1163_d_n12);
        let eq72_e1164_d_n13: f64 = (eq72_e1158_d_n13 + eq72_e1163_d_n13);
        let eq72_e1164_d_n14: f64 = (eq72_e1158_d_n14 + eq72_e1163_d_n14);
        let eq72_e1164_d_n15: f64 = (eq72_e1158_d_n15 + eq72_e1163_d_n15);
        let eq72_e1164_d_n16: f64 = (eq72_e1158_d_n16 + eq72_e1163_d_n16);
        let eq72_e1164_d_n17: f64 = (eq72_e1158_d_n17 + eq72_e1163_d_n17);
        let eq72_e1164_d_n18: f64 = (eq72_e1158_d_n18 + eq72_e1163_d_n18);
        let eq72_e1164_d_n19: f64 = (eq72_e1158_d_n19 + eq72_e1163_d_n19);
        let eq72_e1164_d_n20: f64 = (eq72_e1158_d_n20 + eq72_e1163_d_n20);
        let eq72_e1164_d_n21: f64 = (eq72_e1158_d_n21 + eq72_e1163_d_n21);
        let eq72_e1164_d_n22: f64 = (eq72_e1158_d_n22 + eq72_e1163_d_n22);
        (eq72_e1164, eq72_e1164_d_n0, eq72_e1164_d_n1, eq72_e1164_d_n2, eq72_e1164_d_n3, eq72_e1164_d_n4, eq72_e1164_d_n5, eq72_e1164_d_n6, eq72_e1164_d_n7, eq72_e1164_d_n8, eq72_e1164_d_n9, eq72_e1164_d_n10, eq72_e1164_d_n11, eq72_e1164_d_n12, eq72_e1164_d_n13, eq72_e1164_d_n14, eq72_e1164_d_n15, eq72_e1164_d_n16, eq72_e1164_d_n17, eq72_e1164_d_n18, eq72_e1164_d_n19, eq72_e1164_d_n20, eq72_e1164_d_n21, eq72_e1164_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_value: f64 = eq72_e1166;
        let eq72_node_derivatives: [f64; 23] = [eq72_e1166_d_n0, eq72_e1166_d_n1, eq72_e1166_d_n2, eq72_e1166_d_n3, eq72_e1166_d_n4, eq72_e1166_d_n5, eq72_e1166_d_n6, eq72_e1166_d_n7, eq72_e1166_d_n8, eq72_e1166_d_n9, eq72_e1166_d_n10, eq72_e1166_d_n11, eq72_e1166_d_n12, eq72_e1166_d_n13, eq72_e1166_d_n14, eq72_e1166_d_n15, eq72_e1166_d_n16, eq72_e1166_d_n17, eq72_e1166_d_n18, eq72_e1166_d_n19, eq72_e1166_d_n20, eq72_e1166_d_n21, eq72_e1166_d_n22];
        let eq72_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[15]),
            Some(nodes[7]),
            self.multiplicity * (eq72_value),
            &nodes,
            &eq72_node_derivatives,
            &branches,
            &eq72_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_73_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq73_e1173,) = {
    if ((s.v[433] != 0.0) && (!(s.v[434] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq73_value: f64 = eq73_e1173;
        stamper.stamp_potential(
            branches[33],
            eq73_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_74_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq74_e1178,) = {
    if (!(s.v[433] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq74_value: f64 = eq74_e1178;
        stamper.stamp_potential(
            branches[34],
            eq74_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_75_block_0(
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
        let nv19 = ctx.node_voltage(nodes[19]);
        let (eq75_e1194, eq75_e1194_d_n0, eq75_e1194_d_n1, eq75_e1194_d_n2, eq75_e1194_d_n3, eq75_e1194_d_n4, eq75_e1194_d_n5, eq75_e1194_d_n6, eq75_e1194_d_n7, eq75_e1194_d_n8, eq75_e1194_d_n9, eq75_e1194_d_n10, eq75_e1194_d_n11, eq75_e1194_d_n12, eq75_e1194_d_n13, eq75_e1194_d_n14, eq75_e1194_d_n15, eq75_e1194_d_n16, eq75_e1194_d_n17, eq75_e1194_d_n18, eq75_e1194_d_n19, eq75_e1194_d_n20, eq75_e1194_d_n21, eq75_e1194_d_n22,) = {
    if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
        let eq75_e1184: f64 = (p.p6 * s.v[52]);
        let eq75_e1184_d_n0: f64 = (p.p6 * s.dn[52][0]);
        let eq75_e1184_d_n1: f64 = (p.p6 * s.dn[52][1]);
        let eq75_e1184_d_n2: f64 = (p.p6 * s.dn[52][2]);
        let eq75_e1184_d_n3: f64 = (p.p6 * s.dn[52][3]);
        let eq75_e1184_d_n4: f64 = (p.p6 * s.dn[52][4]);
        let eq75_e1184_d_n5: f64 = (p.p6 * s.dn[52][5]);
        let eq75_e1184_d_n6: f64 = (p.p6 * s.dn[52][6]);
        let eq75_e1184_d_n7: f64 = (p.p6 * s.dn[52][7]);
        let eq75_e1184_d_n8: f64 = (p.p6 * s.dn[52][8]);
        let eq75_e1184_d_n9: f64 = (p.p6 * s.dn[52][9]);
        let eq75_e1184_d_n10: f64 = (p.p6 * s.dn[52][10]);
        let eq75_e1184_d_n11: f64 = (p.p6 * s.dn[52][11]);
        let eq75_e1184_d_n12: f64 = (p.p6 * s.dn[52][12]);
        let eq75_e1184_d_n13: f64 = (p.p6 * s.dn[52][13]);
        let eq75_e1184_d_n14: f64 = (p.p6 * s.dn[52][14]);
        let eq75_e1184_d_n15: f64 = (p.p6 * s.dn[52][15]);
        let eq75_e1184_d_n16: f64 = (p.p6 * s.dn[52][16]);
        let eq75_e1184_d_n17: f64 = (p.p6 * s.dn[52][17]);
        let eq75_e1184_d_n18: f64 = (p.p6 * s.dn[52][18]);
        let eq75_e1184_d_n19: f64 = (p.p6 * s.dn[52][19]);
        let eq75_e1184_d_n20: f64 = (p.p6 * s.dn[52][20]);
        let eq75_e1184_d_n21: f64 = (p.p6 * s.dn[52][21]);
        let eq75_e1184_d_n22: f64 = (p.p6 * s.dn[52][22]);
        let eq75_e1186: f64 = (eq75_e1184 * s.v[245]);
        let eq75_e1186_d_n0: f64 = ((eq75_e1184_d_n0 * s.v[245]) + (eq75_e1184 * s.dn[245][0]));
        let eq75_e1186_d_n1: f64 = ((eq75_e1184_d_n1 * s.v[245]) + (eq75_e1184 * s.dn[245][1]));
        let eq75_e1186_d_n2: f64 = ((eq75_e1184_d_n2 * s.v[245]) + (eq75_e1184 * s.dn[245][2]));
        let eq75_e1186_d_n3: f64 = ((eq75_e1184_d_n3 * s.v[245]) + (eq75_e1184 * s.dn[245][3]));
        let eq75_e1186_d_n4: f64 = ((eq75_e1184_d_n4 * s.v[245]) + (eq75_e1184 * s.dn[245][4]));
        let eq75_e1186_d_n5: f64 = ((eq75_e1184_d_n5 * s.v[245]) + (eq75_e1184 * s.dn[245][5]));
        let eq75_e1186_d_n6: f64 = ((eq75_e1184_d_n6 * s.v[245]) + (eq75_e1184 * s.dn[245][6]));
        let eq75_e1186_d_n7: f64 = ((eq75_e1184_d_n7 * s.v[245]) + (eq75_e1184 * s.dn[245][7]));
        let eq75_e1186_d_n8: f64 = ((eq75_e1184_d_n8 * s.v[245]) + (eq75_e1184 * s.dn[245][8]));
        let eq75_e1186_d_n9: f64 = ((eq75_e1184_d_n9 * s.v[245]) + (eq75_e1184 * s.dn[245][9]));
        let eq75_e1186_d_n10: f64 = ((eq75_e1184_d_n10 * s.v[245]) + (eq75_e1184 * s.dn[245][10]));
        let eq75_e1186_d_n11: f64 = ((eq75_e1184_d_n11 * s.v[245]) + (eq75_e1184 * s.dn[245][11]));
        let eq75_e1186_d_n12: f64 = ((eq75_e1184_d_n12 * s.v[245]) + (eq75_e1184 * s.dn[245][12]));
        let eq75_e1186_d_n13: f64 = ((eq75_e1184_d_n13 * s.v[245]) + (eq75_e1184 * s.dn[245][13]));
        let eq75_e1186_d_n14: f64 = ((eq75_e1184_d_n14 * s.v[245]) + (eq75_e1184 * s.dn[245][14]));
        let eq75_e1186_d_n15: f64 = ((eq75_e1184_d_n15 * s.v[245]) + (eq75_e1184 * s.dn[245][15]));
        let eq75_e1186_d_n16: f64 = ((eq75_e1184_d_n16 * s.v[245]) + (eq75_e1184 * s.dn[245][16]));
        let eq75_e1186_d_n17: f64 = ((eq75_e1184_d_n17 * s.v[245]) + (eq75_e1184 * s.dn[245][17]));
        let eq75_e1186_d_n18: f64 = ((eq75_e1184_d_n18 * s.v[245]) + (eq75_e1184 * s.dn[245][18]));
        let eq75_e1186_d_n19: f64 = ((eq75_e1184_d_n19 * s.v[245]) + (eq75_e1184 * s.dn[245][19]));
        let eq75_e1186_d_n20: f64 = ((eq75_e1184_d_n20 * s.v[245]) + (eq75_e1184 * s.dn[245][20]));
        let eq75_e1186_d_n21: f64 = ((eq75_e1184_d_n21 * s.v[245]) + (eq75_e1184 * s.dn[245][21]));
        let eq75_e1186_d_n22: f64 = ((eq75_e1184_d_n22 * s.v[245]) + (eq75_e1184 * s.dn[245][22]));
        let eq75_e1189: f64 = (p.p6 * s.v[379]);
        let eq75_e1189_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq75_e1189_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq75_e1189_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq75_e1189_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq75_e1189_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq75_e1189_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq75_e1189_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq75_e1189_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq75_e1189_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq75_e1189_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq75_e1189_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq75_e1189_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq75_e1189_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq75_e1189_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq75_e1189_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq75_e1189_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq75_e1189_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq75_e1189_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq75_e1189_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq75_e1189_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq75_e1189_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq75_e1189_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq75_e1189_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq75_e1191: f64 = (eq75_e1189 * (nv8 - nv19));
        let eq75_e1191_d_n0: f64 = (eq75_e1189_d_n0 * (nv8 - nv19));
        let eq75_e1191_d_n1: f64 = (eq75_e1189_d_n1 * (nv8 - nv19));
        let eq75_e1191_d_n2: f64 = (eq75_e1189_d_n2 * (nv8 - nv19));
        let eq75_e1191_d_n3: f64 = (eq75_e1189_d_n3 * (nv8 - nv19));
        let eq75_e1191_d_n4: f64 = (eq75_e1189_d_n4 * (nv8 - nv19));
        let eq75_e1191_d_n5: f64 = (eq75_e1189_d_n5 * (nv8 - nv19));
        let eq75_e1191_d_n6: f64 = (eq75_e1189_d_n6 * (nv8 - nv19));
        let eq75_e1191_d_n7: f64 = (eq75_e1189_d_n7 * (nv8 - nv19));
        let eq75_e1191_d_n8: f64 = ((eq75_e1189_d_n8 * (nv8 - nv19)) + eq75_e1189);
        let eq75_e1191_d_n9: f64 = (eq75_e1189_d_n9 * (nv8 - nv19));
        let eq75_e1191_d_n10: f64 = (eq75_e1189_d_n10 * (nv8 - nv19));
        let eq75_e1191_d_n11: f64 = (eq75_e1189_d_n11 * (nv8 - nv19));
        let eq75_e1191_d_n12: f64 = (eq75_e1189_d_n12 * (nv8 - nv19));
        let eq75_e1191_d_n13: f64 = (eq75_e1189_d_n13 * (nv8 - nv19));
        let eq75_e1191_d_n14: f64 = (eq75_e1189_d_n14 * (nv8 - nv19));
        let eq75_e1191_d_n15: f64 = (eq75_e1189_d_n15 * (nv8 - nv19));
        let eq75_e1191_d_n16: f64 = (eq75_e1189_d_n16 * (nv8 - nv19));
        let eq75_e1191_d_n17: f64 = (eq75_e1189_d_n17 * (nv8 - nv19));
        let eq75_e1191_d_n18: f64 = (eq75_e1189_d_n18 * (nv8 - nv19));
        let eq75_e1191_d_n19: f64 = ((eq75_e1189_d_n19 * (nv8 - nv19)) + (-eq75_e1189));
        let eq75_e1191_d_n20: f64 = (eq75_e1189_d_n20 * (nv8 - nv19));
        let eq75_e1191_d_n21: f64 = (eq75_e1189_d_n21 * (nv8 - nv19));
        let eq75_e1191_d_n22: f64 = (eq75_e1189_d_n22 * (nv8 - nv19));
        let eq75_e1192: f64 = (eq75_e1186 + eq75_e1191);
        let eq75_e1192_d_n0: f64 = (eq75_e1186_d_n0 + eq75_e1191_d_n0);
        let eq75_e1192_d_n1: f64 = (eq75_e1186_d_n1 + eq75_e1191_d_n1);
        let eq75_e1192_d_n2: f64 = (eq75_e1186_d_n2 + eq75_e1191_d_n2);
        let eq75_e1192_d_n3: f64 = (eq75_e1186_d_n3 + eq75_e1191_d_n3);
        let eq75_e1192_d_n4: f64 = (eq75_e1186_d_n4 + eq75_e1191_d_n4);
        let eq75_e1192_d_n5: f64 = (eq75_e1186_d_n5 + eq75_e1191_d_n5);
        let eq75_e1192_d_n6: f64 = (eq75_e1186_d_n6 + eq75_e1191_d_n6);
        let eq75_e1192_d_n7: f64 = (eq75_e1186_d_n7 + eq75_e1191_d_n7);
        let eq75_e1192_d_n8: f64 = (eq75_e1186_d_n8 + eq75_e1191_d_n8);
        let eq75_e1192_d_n9: f64 = (eq75_e1186_d_n9 + eq75_e1191_d_n9);
        let eq75_e1192_d_n10: f64 = (eq75_e1186_d_n10 + eq75_e1191_d_n10);
        let eq75_e1192_d_n11: f64 = (eq75_e1186_d_n11 + eq75_e1191_d_n11);
        let eq75_e1192_d_n12: f64 = (eq75_e1186_d_n12 + eq75_e1191_d_n12);
        let eq75_e1192_d_n13: f64 = (eq75_e1186_d_n13 + eq75_e1191_d_n13);
        let eq75_e1192_d_n14: f64 = (eq75_e1186_d_n14 + eq75_e1191_d_n14);
        let eq75_e1192_d_n15: f64 = (eq75_e1186_d_n15 + eq75_e1191_d_n15);
        let eq75_e1192_d_n16: f64 = (eq75_e1186_d_n16 + eq75_e1191_d_n16);
        let eq75_e1192_d_n17: f64 = (eq75_e1186_d_n17 + eq75_e1191_d_n17);
        let eq75_e1192_d_n18: f64 = (eq75_e1186_d_n18 + eq75_e1191_d_n18);
        let eq75_e1192_d_n19: f64 = (eq75_e1186_d_n19 + eq75_e1191_d_n19);
        let eq75_e1192_d_n20: f64 = (eq75_e1186_d_n20 + eq75_e1191_d_n20);
        let eq75_e1192_d_n21: f64 = (eq75_e1186_d_n21 + eq75_e1191_d_n21);
        let eq75_e1192_d_n22: f64 = (eq75_e1186_d_n22 + eq75_e1191_d_n22);
        (eq75_e1192, eq75_e1192_d_n0, eq75_e1192_d_n1, eq75_e1192_d_n2, eq75_e1192_d_n3, eq75_e1192_d_n4, eq75_e1192_d_n5, eq75_e1192_d_n6, eq75_e1192_d_n7, eq75_e1192_d_n8, eq75_e1192_d_n9, eq75_e1192_d_n10, eq75_e1192_d_n11, eq75_e1192_d_n12, eq75_e1192_d_n13, eq75_e1192_d_n14, eq75_e1192_d_n15, eq75_e1192_d_n16, eq75_e1192_d_n17, eq75_e1192_d_n18, eq75_e1192_d_n19, eq75_e1192_d_n20, eq75_e1192_d_n21, eq75_e1192_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq75_value: f64 = eq75_e1194;
        let eq75_node_derivatives: [f64; 23] = [eq75_e1194_d_n0, eq75_e1194_d_n1, eq75_e1194_d_n2, eq75_e1194_d_n3, eq75_e1194_d_n4, eq75_e1194_d_n5, eq75_e1194_d_n6, eq75_e1194_d_n7, eq75_e1194_d_n8, eq75_e1194_d_n9, eq75_e1194_d_n10, eq75_e1194_d_n11, eq75_e1194_d_n12, eq75_e1194_d_n13, eq75_e1194_d_n14, eq75_e1194_d_n15, eq75_e1194_d_n16, eq75_e1194_d_n17, eq75_e1194_d_n18, eq75_e1194_d_n19, eq75_e1194_d_n20, eq75_e1194_d_n21, eq75_e1194_d_n22];
        let eq75_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[19]),
            self.multiplicity * (eq75_value),
            &nodes,
            &eq75_node_derivatives,
            &branches,
            &eq75_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_76_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq76_e1201,) = {
    if ((s.v[448] != 0.0) && (!(s.v[449] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq76_value: f64 = eq76_e1201;
        stamper.stamp_potential(
            branches[35],
            eq76_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_77_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq77_e1206,) = {
    if (!(s.v[448] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq77_value: f64 = eq77_e1206;
        stamper.stamp_potential(
            branches[36],
            eq77_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_78_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq78_e1214,) = {
    if ((!(s.v[448] != 0.0)) && (!(s.v[457] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq78_value: f64 = eq78_e1214;
        stamper.stamp_potential(
            branches[37],
            eq78_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_79_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq79_e1230, eq79_e1230_d_n0, eq79_e1230_d_n1, eq79_e1230_d_n2, eq79_e1230_d_n3, eq79_e1230_d_n4, eq79_e1230_d_n5, eq79_e1230_d_n6, eq79_e1230_d_n7, eq79_e1230_d_n8, eq79_e1230_d_n9, eq79_e1230_d_n10, eq79_e1230_d_n11, eq79_e1230_d_n12, eq79_e1230_d_n13, eq79_e1230_d_n14, eq79_e1230_d_n15, eq79_e1230_d_n16, eq79_e1230_d_n17, eq79_e1230_d_n18, eq79_e1230_d_n19, eq79_e1230_d_n20, eq79_e1230_d_n21, eq79_e1230_d_n22,) = {
    if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
        let eq79_e1220: f64 = (p.p6 * s.v[56]);
        let eq79_e1220_d_n0: f64 = (p.p6 * s.dn[56][0]);
        let eq79_e1220_d_n1: f64 = (p.p6 * s.dn[56][1]);
        let eq79_e1220_d_n2: f64 = (p.p6 * s.dn[56][2]);
        let eq79_e1220_d_n3: f64 = (p.p6 * s.dn[56][3]);
        let eq79_e1220_d_n4: f64 = (p.p6 * s.dn[56][4]);
        let eq79_e1220_d_n5: f64 = (p.p6 * s.dn[56][5]);
        let eq79_e1220_d_n6: f64 = (p.p6 * s.dn[56][6]);
        let eq79_e1220_d_n7: f64 = (p.p6 * s.dn[56][7]);
        let eq79_e1220_d_n8: f64 = (p.p6 * s.dn[56][8]);
        let eq79_e1220_d_n9: f64 = (p.p6 * s.dn[56][9]);
        let eq79_e1220_d_n10: f64 = (p.p6 * s.dn[56][10]);
        let eq79_e1220_d_n11: f64 = (p.p6 * s.dn[56][11]);
        let eq79_e1220_d_n12: f64 = (p.p6 * s.dn[56][12]);
        let eq79_e1220_d_n13: f64 = (p.p6 * s.dn[56][13]);
        let eq79_e1220_d_n14: f64 = (p.p6 * s.dn[56][14]);
        let eq79_e1220_d_n15: f64 = (p.p6 * s.dn[56][15]);
        let eq79_e1220_d_n16: f64 = (p.p6 * s.dn[56][16]);
        let eq79_e1220_d_n17: f64 = (p.p6 * s.dn[56][17]);
        let eq79_e1220_d_n18: f64 = (p.p6 * s.dn[56][18]);
        let eq79_e1220_d_n19: f64 = (p.p6 * s.dn[56][19]);
        let eq79_e1220_d_n20: f64 = (p.p6 * s.dn[56][20]);
        let eq79_e1220_d_n21: f64 = (p.p6 * s.dn[56][21]);
        let eq79_e1220_d_n22: f64 = (p.p6 * s.dn[56][22]);
        let eq79_e1222: f64 = (eq79_e1220 * s.v[257]);
        let eq79_e1222_d_n0: f64 = ((eq79_e1220_d_n0 * s.v[257]) + (eq79_e1220 * s.dn[257][0]));
        let eq79_e1222_d_n1: f64 = ((eq79_e1220_d_n1 * s.v[257]) + (eq79_e1220 * s.dn[257][1]));
        let eq79_e1222_d_n2: f64 = ((eq79_e1220_d_n2 * s.v[257]) + (eq79_e1220 * s.dn[257][2]));
        let eq79_e1222_d_n3: f64 = ((eq79_e1220_d_n3 * s.v[257]) + (eq79_e1220 * s.dn[257][3]));
        let eq79_e1222_d_n4: f64 = ((eq79_e1220_d_n4 * s.v[257]) + (eq79_e1220 * s.dn[257][4]));
        let eq79_e1222_d_n5: f64 = ((eq79_e1220_d_n5 * s.v[257]) + (eq79_e1220 * s.dn[257][5]));
        let eq79_e1222_d_n6: f64 = ((eq79_e1220_d_n6 * s.v[257]) + (eq79_e1220 * s.dn[257][6]));
        let eq79_e1222_d_n7: f64 = ((eq79_e1220_d_n7 * s.v[257]) + (eq79_e1220 * s.dn[257][7]));
        let eq79_e1222_d_n8: f64 = ((eq79_e1220_d_n8 * s.v[257]) + (eq79_e1220 * s.dn[257][8]));
        let eq79_e1222_d_n9: f64 = ((eq79_e1220_d_n9 * s.v[257]) + (eq79_e1220 * s.dn[257][9]));
        let eq79_e1222_d_n10: f64 = ((eq79_e1220_d_n10 * s.v[257]) + (eq79_e1220 * s.dn[257][10]));
        let eq79_e1222_d_n11: f64 = ((eq79_e1220_d_n11 * s.v[257]) + (eq79_e1220 * s.dn[257][11]));
        let eq79_e1222_d_n12: f64 = ((eq79_e1220_d_n12 * s.v[257]) + (eq79_e1220 * s.dn[257][12]));
        let eq79_e1222_d_n13: f64 = ((eq79_e1220_d_n13 * s.v[257]) + (eq79_e1220 * s.dn[257][13]));
        let eq79_e1222_d_n14: f64 = ((eq79_e1220_d_n14 * s.v[257]) + (eq79_e1220 * s.dn[257][14]));
        let eq79_e1222_d_n15: f64 = ((eq79_e1220_d_n15 * s.v[257]) + (eq79_e1220 * s.dn[257][15]));
        let eq79_e1222_d_n16: f64 = ((eq79_e1220_d_n16 * s.v[257]) + (eq79_e1220 * s.dn[257][16]));
        let eq79_e1222_d_n17: f64 = ((eq79_e1220_d_n17 * s.v[257]) + (eq79_e1220 * s.dn[257][17]));
        let eq79_e1222_d_n18: f64 = ((eq79_e1220_d_n18 * s.v[257]) + (eq79_e1220 * s.dn[257][18]));
        let eq79_e1222_d_n19: f64 = ((eq79_e1220_d_n19 * s.v[257]) + (eq79_e1220 * s.dn[257][19]));
        let eq79_e1222_d_n20: f64 = ((eq79_e1220_d_n20 * s.v[257]) + (eq79_e1220 * s.dn[257][20]));
        let eq79_e1222_d_n21: f64 = ((eq79_e1220_d_n21 * s.v[257]) + (eq79_e1220 * s.dn[257][21]));
        let eq79_e1222_d_n22: f64 = ((eq79_e1220_d_n22 * s.v[257]) + (eq79_e1220 * s.dn[257][22]));
        let eq79_e1225: f64 = (p.p6 * s.v[379]);
        let eq79_e1225_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq79_e1225_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq79_e1225_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq79_e1225_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq79_e1225_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq79_e1225_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq79_e1225_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq79_e1225_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq79_e1225_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq79_e1225_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq79_e1225_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq79_e1225_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq79_e1225_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq79_e1225_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq79_e1225_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq79_e1225_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq79_e1225_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq79_e1225_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq79_e1225_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq79_e1225_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq79_e1225_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq79_e1225_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq79_e1225_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq79_e1227: f64 = (eq79_e1225 * (nv16 - nv15));
        let eq79_e1227_d_n0: f64 = (eq79_e1225_d_n0 * (nv16 - nv15));
        let eq79_e1227_d_n1: f64 = (eq79_e1225_d_n1 * (nv16 - nv15));
        let eq79_e1227_d_n2: f64 = (eq79_e1225_d_n2 * (nv16 - nv15));
        let eq79_e1227_d_n3: f64 = (eq79_e1225_d_n3 * (nv16 - nv15));
        let eq79_e1227_d_n4: f64 = (eq79_e1225_d_n4 * (nv16 - nv15));
        let eq79_e1227_d_n5: f64 = (eq79_e1225_d_n5 * (nv16 - nv15));
        let eq79_e1227_d_n6: f64 = (eq79_e1225_d_n6 * (nv16 - nv15));
        let eq79_e1227_d_n7: f64 = (eq79_e1225_d_n7 * (nv16 - nv15));
        let eq79_e1227_d_n8: f64 = (eq79_e1225_d_n8 * (nv16 - nv15));
        let eq79_e1227_d_n9: f64 = (eq79_e1225_d_n9 * (nv16 - nv15));
        let eq79_e1227_d_n10: f64 = (eq79_e1225_d_n10 * (nv16 - nv15));
        let eq79_e1227_d_n11: f64 = (eq79_e1225_d_n11 * (nv16 - nv15));
        let eq79_e1227_d_n12: f64 = (eq79_e1225_d_n12 * (nv16 - nv15));
        let eq79_e1227_d_n13: f64 = (eq79_e1225_d_n13 * (nv16 - nv15));
        let eq79_e1227_d_n14: f64 = (eq79_e1225_d_n14 * (nv16 - nv15));
        let eq79_e1227_d_n15: f64 = ((eq79_e1225_d_n15 * (nv16 - nv15)) + (-eq79_e1225));
        let eq79_e1227_d_n16: f64 = ((eq79_e1225_d_n16 * (nv16 - nv15)) + eq79_e1225);
        let eq79_e1227_d_n17: f64 = (eq79_e1225_d_n17 * (nv16 - nv15));
        let eq79_e1227_d_n18: f64 = (eq79_e1225_d_n18 * (nv16 - nv15));
        let eq79_e1227_d_n19: f64 = (eq79_e1225_d_n19 * (nv16 - nv15));
        let eq79_e1227_d_n20: f64 = (eq79_e1225_d_n20 * (nv16 - nv15));
        let eq79_e1227_d_n21: f64 = (eq79_e1225_d_n21 * (nv16 - nv15));
        let eq79_e1227_d_n22: f64 = (eq79_e1225_d_n22 * (nv16 - nv15));
        let eq79_e1228: f64 = (eq79_e1222 + eq79_e1227);
        let eq79_e1228_d_n0: f64 = (eq79_e1222_d_n0 + eq79_e1227_d_n0);
        let eq79_e1228_d_n1: f64 = (eq79_e1222_d_n1 + eq79_e1227_d_n1);
        let eq79_e1228_d_n2: f64 = (eq79_e1222_d_n2 + eq79_e1227_d_n2);
        let eq79_e1228_d_n3: f64 = (eq79_e1222_d_n3 + eq79_e1227_d_n3);
        let eq79_e1228_d_n4: f64 = (eq79_e1222_d_n4 + eq79_e1227_d_n4);
        let eq79_e1228_d_n5: f64 = (eq79_e1222_d_n5 + eq79_e1227_d_n5);
        let eq79_e1228_d_n6: f64 = (eq79_e1222_d_n6 + eq79_e1227_d_n6);
        let eq79_e1228_d_n7: f64 = (eq79_e1222_d_n7 + eq79_e1227_d_n7);
        let eq79_e1228_d_n8: f64 = (eq79_e1222_d_n8 + eq79_e1227_d_n8);
        let eq79_e1228_d_n9: f64 = (eq79_e1222_d_n9 + eq79_e1227_d_n9);
        let eq79_e1228_d_n10: f64 = (eq79_e1222_d_n10 + eq79_e1227_d_n10);
        let eq79_e1228_d_n11: f64 = (eq79_e1222_d_n11 + eq79_e1227_d_n11);
        let eq79_e1228_d_n12: f64 = (eq79_e1222_d_n12 + eq79_e1227_d_n12);
        let eq79_e1228_d_n13: f64 = (eq79_e1222_d_n13 + eq79_e1227_d_n13);
        let eq79_e1228_d_n14: f64 = (eq79_e1222_d_n14 + eq79_e1227_d_n14);
        let eq79_e1228_d_n15: f64 = (eq79_e1222_d_n15 + eq79_e1227_d_n15);
        let eq79_e1228_d_n16: f64 = (eq79_e1222_d_n16 + eq79_e1227_d_n16);
        let eq79_e1228_d_n17: f64 = (eq79_e1222_d_n17 + eq79_e1227_d_n17);
        let eq79_e1228_d_n18: f64 = (eq79_e1222_d_n18 + eq79_e1227_d_n18);
        let eq79_e1228_d_n19: f64 = (eq79_e1222_d_n19 + eq79_e1227_d_n19);
        let eq79_e1228_d_n20: f64 = (eq79_e1222_d_n20 + eq79_e1227_d_n20);
        let eq79_e1228_d_n21: f64 = (eq79_e1222_d_n21 + eq79_e1227_d_n21);
        let eq79_e1228_d_n22: f64 = (eq79_e1222_d_n22 + eq79_e1227_d_n22);
        (eq79_e1228, eq79_e1228_d_n0, eq79_e1228_d_n1, eq79_e1228_d_n2, eq79_e1228_d_n3, eq79_e1228_d_n4, eq79_e1228_d_n5, eq79_e1228_d_n6, eq79_e1228_d_n7, eq79_e1228_d_n8, eq79_e1228_d_n9, eq79_e1228_d_n10, eq79_e1228_d_n11, eq79_e1228_d_n12, eq79_e1228_d_n13, eq79_e1228_d_n14, eq79_e1228_d_n15, eq79_e1228_d_n16, eq79_e1228_d_n17, eq79_e1228_d_n18, eq79_e1228_d_n19, eq79_e1228_d_n20, eq79_e1228_d_n21, eq79_e1228_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq79_value: f64 = eq79_e1230;
        let eq79_node_derivatives: [f64; 23] = [eq79_e1230_d_n0, eq79_e1230_d_n1, eq79_e1230_d_n2, eq79_e1230_d_n3, eq79_e1230_d_n4, eq79_e1230_d_n5, eq79_e1230_d_n6, eq79_e1230_d_n7, eq79_e1230_d_n8, eq79_e1230_d_n9, eq79_e1230_d_n10, eq79_e1230_d_n11, eq79_e1230_d_n12, eq79_e1230_d_n13, eq79_e1230_d_n14, eq79_e1230_d_n15, eq79_e1230_d_n16, eq79_e1230_d_n17, eq79_e1230_d_n18, eq79_e1230_d_n19, eq79_e1230_d_n20, eq79_e1230_d_n21, eq79_e1230_d_n22];
        let eq79_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[16]),
            Some(nodes[15]),
            self.multiplicity * (eq79_value),
            &nodes,
            &eq79_node_derivatives,
            &branches,
            &eq79_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_80_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq80_e1237,) = {
    if ((s.v[463] != 0.0) && (!(s.v[464] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq80_value: f64 = eq80_e1237;
        stamper.stamp_potential(
            branches[38],
            eq80_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_81_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq81_e1242,) = {
    if (!(s.v[463] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq81_value: f64 = eq81_e1242;
        stamper.stamp_potential(
            branches[39],
            eq81_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_82_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv19 = ctx.node_voltage(nodes[19]);
        let nv20 = ctx.node_voltage(nodes[20]);
        let (eq82_e1258, eq82_e1258_d_n0, eq82_e1258_d_n1, eq82_e1258_d_n2, eq82_e1258_d_n3, eq82_e1258_d_n4, eq82_e1258_d_n5, eq82_e1258_d_n6, eq82_e1258_d_n7, eq82_e1258_d_n8, eq82_e1258_d_n9, eq82_e1258_d_n10, eq82_e1258_d_n11, eq82_e1258_d_n12, eq82_e1258_d_n13, eq82_e1258_d_n14, eq82_e1258_d_n15, eq82_e1258_d_n16, eq82_e1258_d_n17, eq82_e1258_d_n18, eq82_e1258_d_n19, eq82_e1258_d_n20, eq82_e1258_d_n21, eq82_e1258_d_n22,) = {
    if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
        let eq82_e1248: f64 = (p.p6 * s.v[60]);
        let eq82_e1248_d_n0: f64 = (p.p6 * s.dn[60][0]);
        let eq82_e1248_d_n1: f64 = (p.p6 * s.dn[60][1]);
        let eq82_e1248_d_n2: f64 = (p.p6 * s.dn[60][2]);
        let eq82_e1248_d_n3: f64 = (p.p6 * s.dn[60][3]);
        let eq82_e1248_d_n4: f64 = (p.p6 * s.dn[60][4]);
        let eq82_e1248_d_n5: f64 = (p.p6 * s.dn[60][5]);
        let eq82_e1248_d_n6: f64 = (p.p6 * s.dn[60][6]);
        let eq82_e1248_d_n7: f64 = (p.p6 * s.dn[60][7]);
        let eq82_e1248_d_n8: f64 = (p.p6 * s.dn[60][8]);
        let eq82_e1248_d_n9: f64 = (p.p6 * s.dn[60][9]);
        let eq82_e1248_d_n10: f64 = (p.p6 * s.dn[60][10]);
        let eq82_e1248_d_n11: f64 = (p.p6 * s.dn[60][11]);
        let eq82_e1248_d_n12: f64 = (p.p6 * s.dn[60][12]);
        let eq82_e1248_d_n13: f64 = (p.p6 * s.dn[60][13]);
        let eq82_e1248_d_n14: f64 = (p.p6 * s.dn[60][14]);
        let eq82_e1248_d_n15: f64 = (p.p6 * s.dn[60][15]);
        let eq82_e1248_d_n16: f64 = (p.p6 * s.dn[60][16]);
        let eq82_e1248_d_n17: f64 = (p.p6 * s.dn[60][17]);
        let eq82_e1248_d_n18: f64 = (p.p6 * s.dn[60][18]);
        let eq82_e1248_d_n19: f64 = (p.p6 * s.dn[60][19]);
        let eq82_e1248_d_n20: f64 = (p.p6 * s.dn[60][20]);
        let eq82_e1248_d_n21: f64 = (p.p6 * s.dn[60][21]);
        let eq82_e1248_d_n22: f64 = (p.p6 * s.dn[60][22]);
        let eq82_e1250: f64 = (eq82_e1248 * s.v[269]);
        let eq82_e1250_d_n0: f64 = ((eq82_e1248_d_n0 * s.v[269]) + (eq82_e1248 * s.dn[269][0]));
        let eq82_e1250_d_n1: f64 = ((eq82_e1248_d_n1 * s.v[269]) + (eq82_e1248 * s.dn[269][1]));
        let eq82_e1250_d_n2: f64 = ((eq82_e1248_d_n2 * s.v[269]) + (eq82_e1248 * s.dn[269][2]));
        let eq82_e1250_d_n3: f64 = ((eq82_e1248_d_n3 * s.v[269]) + (eq82_e1248 * s.dn[269][3]));
        let eq82_e1250_d_n4: f64 = ((eq82_e1248_d_n4 * s.v[269]) + (eq82_e1248 * s.dn[269][4]));
        let eq82_e1250_d_n5: f64 = ((eq82_e1248_d_n5 * s.v[269]) + (eq82_e1248 * s.dn[269][5]));
        let eq82_e1250_d_n6: f64 = ((eq82_e1248_d_n6 * s.v[269]) + (eq82_e1248 * s.dn[269][6]));
        let eq82_e1250_d_n7: f64 = ((eq82_e1248_d_n7 * s.v[269]) + (eq82_e1248 * s.dn[269][7]));
        let eq82_e1250_d_n8: f64 = ((eq82_e1248_d_n8 * s.v[269]) + (eq82_e1248 * s.dn[269][8]));
        let eq82_e1250_d_n9: f64 = ((eq82_e1248_d_n9 * s.v[269]) + (eq82_e1248 * s.dn[269][9]));
        let eq82_e1250_d_n10: f64 = ((eq82_e1248_d_n10 * s.v[269]) + (eq82_e1248 * s.dn[269][10]));
        let eq82_e1250_d_n11: f64 = ((eq82_e1248_d_n11 * s.v[269]) + (eq82_e1248 * s.dn[269][11]));
        let eq82_e1250_d_n12: f64 = ((eq82_e1248_d_n12 * s.v[269]) + (eq82_e1248 * s.dn[269][12]));
        let eq82_e1250_d_n13: f64 = ((eq82_e1248_d_n13 * s.v[269]) + (eq82_e1248 * s.dn[269][13]));
        let eq82_e1250_d_n14: f64 = ((eq82_e1248_d_n14 * s.v[269]) + (eq82_e1248 * s.dn[269][14]));
        let eq82_e1250_d_n15: f64 = ((eq82_e1248_d_n15 * s.v[269]) + (eq82_e1248 * s.dn[269][15]));
        let eq82_e1250_d_n16: f64 = ((eq82_e1248_d_n16 * s.v[269]) + (eq82_e1248 * s.dn[269][16]));
        let eq82_e1250_d_n17: f64 = ((eq82_e1248_d_n17 * s.v[269]) + (eq82_e1248 * s.dn[269][17]));
        let eq82_e1250_d_n18: f64 = ((eq82_e1248_d_n18 * s.v[269]) + (eq82_e1248 * s.dn[269][18]));
        let eq82_e1250_d_n19: f64 = ((eq82_e1248_d_n19 * s.v[269]) + (eq82_e1248 * s.dn[269][19]));
        let eq82_e1250_d_n20: f64 = ((eq82_e1248_d_n20 * s.v[269]) + (eq82_e1248 * s.dn[269][20]));
        let eq82_e1250_d_n21: f64 = ((eq82_e1248_d_n21 * s.v[269]) + (eq82_e1248 * s.dn[269][21]));
        let eq82_e1250_d_n22: f64 = ((eq82_e1248_d_n22 * s.v[269]) + (eq82_e1248 * s.dn[269][22]));
        let eq82_e1253: f64 = (p.p6 * s.v[379]);
        let eq82_e1253_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq82_e1253_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq82_e1253_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq82_e1253_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq82_e1253_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq82_e1253_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq82_e1253_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq82_e1253_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq82_e1253_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq82_e1253_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq82_e1253_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq82_e1253_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq82_e1253_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq82_e1253_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq82_e1253_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq82_e1253_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq82_e1253_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq82_e1253_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq82_e1253_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq82_e1253_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq82_e1253_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq82_e1253_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq82_e1253_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq82_e1255: f64 = (eq82_e1253 * (nv19 - nv20));
        let eq82_e1255_d_n0: f64 = (eq82_e1253_d_n0 * (nv19 - nv20));
        let eq82_e1255_d_n1: f64 = (eq82_e1253_d_n1 * (nv19 - nv20));
        let eq82_e1255_d_n2: f64 = (eq82_e1253_d_n2 * (nv19 - nv20));
        let eq82_e1255_d_n3: f64 = (eq82_e1253_d_n3 * (nv19 - nv20));
        let eq82_e1255_d_n4: f64 = (eq82_e1253_d_n4 * (nv19 - nv20));
        let eq82_e1255_d_n5: f64 = (eq82_e1253_d_n5 * (nv19 - nv20));
        let eq82_e1255_d_n6: f64 = (eq82_e1253_d_n6 * (nv19 - nv20));
        let eq82_e1255_d_n7: f64 = (eq82_e1253_d_n7 * (nv19 - nv20));
        let eq82_e1255_d_n8: f64 = (eq82_e1253_d_n8 * (nv19 - nv20));
        let eq82_e1255_d_n9: f64 = (eq82_e1253_d_n9 * (nv19 - nv20));
        let eq82_e1255_d_n10: f64 = (eq82_e1253_d_n10 * (nv19 - nv20));
        let eq82_e1255_d_n11: f64 = (eq82_e1253_d_n11 * (nv19 - nv20));
        let eq82_e1255_d_n12: f64 = (eq82_e1253_d_n12 * (nv19 - nv20));
        let eq82_e1255_d_n13: f64 = (eq82_e1253_d_n13 * (nv19 - nv20));
        let eq82_e1255_d_n14: f64 = (eq82_e1253_d_n14 * (nv19 - nv20));
        let eq82_e1255_d_n15: f64 = (eq82_e1253_d_n15 * (nv19 - nv20));
        let eq82_e1255_d_n16: f64 = (eq82_e1253_d_n16 * (nv19 - nv20));
        let eq82_e1255_d_n17: f64 = (eq82_e1253_d_n17 * (nv19 - nv20));
        let eq82_e1255_d_n18: f64 = (eq82_e1253_d_n18 * (nv19 - nv20));
        let eq82_e1255_d_n19: f64 = ((eq82_e1253_d_n19 * (nv19 - nv20)) + eq82_e1253);
        let eq82_e1255_d_n20: f64 = ((eq82_e1253_d_n20 * (nv19 - nv20)) + (-eq82_e1253));
        let eq82_e1255_d_n21: f64 = (eq82_e1253_d_n21 * (nv19 - nv20));
        let eq82_e1255_d_n22: f64 = (eq82_e1253_d_n22 * (nv19 - nv20));
        let eq82_e1256: f64 = (eq82_e1250 + eq82_e1255);
        let eq82_e1256_d_n0: f64 = (eq82_e1250_d_n0 + eq82_e1255_d_n0);
        let eq82_e1256_d_n1: f64 = (eq82_e1250_d_n1 + eq82_e1255_d_n1);
        let eq82_e1256_d_n2: f64 = (eq82_e1250_d_n2 + eq82_e1255_d_n2);
        let eq82_e1256_d_n3: f64 = (eq82_e1250_d_n3 + eq82_e1255_d_n3);
        let eq82_e1256_d_n4: f64 = (eq82_e1250_d_n4 + eq82_e1255_d_n4);
        let eq82_e1256_d_n5: f64 = (eq82_e1250_d_n5 + eq82_e1255_d_n5);
        let eq82_e1256_d_n6: f64 = (eq82_e1250_d_n6 + eq82_e1255_d_n6);
        let eq82_e1256_d_n7: f64 = (eq82_e1250_d_n7 + eq82_e1255_d_n7);
        let eq82_e1256_d_n8: f64 = (eq82_e1250_d_n8 + eq82_e1255_d_n8);
        let eq82_e1256_d_n9: f64 = (eq82_e1250_d_n9 + eq82_e1255_d_n9);
        let eq82_e1256_d_n10: f64 = (eq82_e1250_d_n10 + eq82_e1255_d_n10);
        let eq82_e1256_d_n11: f64 = (eq82_e1250_d_n11 + eq82_e1255_d_n11);
        let eq82_e1256_d_n12: f64 = (eq82_e1250_d_n12 + eq82_e1255_d_n12);
        let eq82_e1256_d_n13: f64 = (eq82_e1250_d_n13 + eq82_e1255_d_n13);
        let eq82_e1256_d_n14: f64 = (eq82_e1250_d_n14 + eq82_e1255_d_n14);
        let eq82_e1256_d_n15: f64 = (eq82_e1250_d_n15 + eq82_e1255_d_n15);
        let eq82_e1256_d_n16: f64 = (eq82_e1250_d_n16 + eq82_e1255_d_n16);
        let eq82_e1256_d_n17: f64 = (eq82_e1250_d_n17 + eq82_e1255_d_n17);
        let eq82_e1256_d_n18: f64 = (eq82_e1250_d_n18 + eq82_e1255_d_n18);
        let eq82_e1256_d_n19: f64 = (eq82_e1250_d_n19 + eq82_e1255_d_n19);
        let eq82_e1256_d_n20: f64 = (eq82_e1250_d_n20 + eq82_e1255_d_n20);
        let eq82_e1256_d_n21: f64 = (eq82_e1250_d_n21 + eq82_e1255_d_n21);
        let eq82_e1256_d_n22: f64 = (eq82_e1250_d_n22 + eq82_e1255_d_n22);
        (eq82_e1256, eq82_e1256_d_n0, eq82_e1256_d_n1, eq82_e1256_d_n2, eq82_e1256_d_n3, eq82_e1256_d_n4, eq82_e1256_d_n5, eq82_e1256_d_n6, eq82_e1256_d_n7, eq82_e1256_d_n8, eq82_e1256_d_n9, eq82_e1256_d_n10, eq82_e1256_d_n11, eq82_e1256_d_n12, eq82_e1256_d_n13, eq82_e1256_d_n14, eq82_e1256_d_n15, eq82_e1256_d_n16, eq82_e1256_d_n17, eq82_e1256_d_n18, eq82_e1256_d_n19, eq82_e1256_d_n20, eq82_e1256_d_n21, eq82_e1256_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq82_value: f64 = eq82_e1258;
        let eq82_node_derivatives: [f64; 23] = [eq82_e1258_d_n0, eq82_e1258_d_n1, eq82_e1258_d_n2, eq82_e1258_d_n3, eq82_e1258_d_n4, eq82_e1258_d_n5, eq82_e1258_d_n6, eq82_e1258_d_n7, eq82_e1258_d_n8, eq82_e1258_d_n9, eq82_e1258_d_n10, eq82_e1258_d_n11, eq82_e1258_d_n12, eq82_e1258_d_n13, eq82_e1258_d_n14, eq82_e1258_d_n15, eq82_e1258_d_n16, eq82_e1258_d_n17, eq82_e1258_d_n18, eq82_e1258_d_n19, eq82_e1258_d_n20, eq82_e1258_d_n21, eq82_e1258_d_n22];
        let eq82_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[19]),
            Some(nodes[20]),
            self.multiplicity * (eq82_value),
            &nodes,
            &eq82_node_derivatives,
            &branches,
            &eq82_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_83_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq83_e1265,) = {
    if ((s.v[478] != 0.0) && (!(s.v[479] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq83_value: f64 = eq83_e1265;
        stamper.stamp_potential(
            branches[40],
            eq83_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_84_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq84_e1270,) = {
    if (!(s.v[478] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq84_value: f64 = eq84_e1270;
        stamper.stamp_potential(
            branches[41],
            eq84_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_85_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq85_e1278,) = {
    if ((!(s.v[478] != 0.0)) && (!(s.v[487] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq85_value: f64 = eq85_e1278;
        stamper.stamp_potential(
            branches[42],
            eq85_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_86_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq86_e1294, eq86_e1294_d_n0, eq86_e1294_d_n1, eq86_e1294_d_n2, eq86_e1294_d_n3, eq86_e1294_d_n4, eq86_e1294_d_n5, eq86_e1294_d_n6, eq86_e1294_d_n7, eq86_e1294_d_n8, eq86_e1294_d_n9, eq86_e1294_d_n10, eq86_e1294_d_n11, eq86_e1294_d_n12, eq86_e1294_d_n13, eq86_e1294_d_n14, eq86_e1294_d_n15, eq86_e1294_d_n16, eq86_e1294_d_n17, eq86_e1294_d_n18, eq86_e1294_d_n19, eq86_e1294_d_n20, eq86_e1294_d_n21, eq86_e1294_d_n22,) = {
    if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
        let eq86_e1284: f64 = (p.p6 * s.v[64]);
        let eq86_e1284_d_n0: f64 = (p.p6 * s.dn[64][0]);
        let eq86_e1284_d_n1: f64 = (p.p6 * s.dn[64][1]);
        let eq86_e1284_d_n2: f64 = (p.p6 * s.dn[64][2]);
        let eq86_e1284_d_n3: f64 = (p.p6 * s.dn[64][3]);
        let eq86_e1284_d_n4: f64 = (p.p6 * s.dn[64][4]);
        let eq86_e1284_d_n5: f64 = (p.p6 * s.dn[64][5]);
        let eq86_e1284_d_n6: f64 = (p.p6 * s.dn[64][6]);
        let eq86_e1284_d_n7: f64 = (p.p6 * s.dn[64][7]);
        let eq86_e1284_d_n8: f64 = (p.p6 * s.dn[64][8]);
        let eq86_e1284_d_n9: f64 = (p.p6 * s.dn[64][9]);
        let eq86_e1284_d_n10: f64 = (p.p6 * s.dn[64][10]);
        let eq86_e1284_d_n11: f64 = (p.p6 * s.dn[64][11]);
        let eq86_e1284_d_n12: f64 = (p.p6 * s.dn[64][12]);
        let eq86_e1284_d_n13: f64 = (p.p6 * s.dn[64][13]);
        let eq86_e1284_d_n14: f64 = (p.p6 * s.dn[64][14]);
        let eq86_e1284_d_n15: f64 = (p.p6 * s.dn[64][15]);
        let eq86_e1284_d_n16: f64 = (p.p6 * s.dn[64][16]);
        let eq86_e1284_d_n17: f64 = (p.p6 * s.dn[64][17]);
        let eq86_e1284_d_n18: f64 = (p.p6 * s.dn[64][18]);
        let eq86_e1284_d_n19: f64 = (p.p6 * s.dn[64][19]);
        let eq86_e1284_d_n20: f64 = (p.p6 * s.dn[64][20]);
        let eq86_e1284_d_n21: f64 = (p.p6 * s.dn[64][21]);
        let eq86_e1284_d_n22: f64 = (p.p6 * s.dn[64][22]);
        let eq86_e1286: f64 = (eq86_e1284 * s.v[281]);
        let eq86_e1286_d_n0: f64 = ((eq86_e1284_d_n0 * s.v[281]) + (eq86_e1284 * s.dn[281][0]));
        let eq86_e1286_d_n1: f64 = ((eq86_e1284_d_n1 * s.v[281]) + (eq86_e1284 * s.dn[281][1]));
        let eq86_e1286_d_n2: f64 = ((eq86_e1284_d_n2 * s.v[281]) + (eq86_e1284 * s.dn[281][2]));
        let eq86_e1286_d_n3: f64 = ((eq86_e1284_d_n3 * s.v[281]) + (eq86_e1284 * s.dn[281][3]));
        let eq86_e1286_d_n4: f64 = ((eq86_e1284_d_n4 * s.v[281]) + (eq86_e1284 * s.dn[281][4]));
        let eq86_e1286_d_n5: f64 = ((eq86_e1284_d_n5 * s.v[281]) + (eq86_e1284 * s.dn[281][5]));
        let eq86_e1286_d_n6: f64 = ((eq86_e1284_d_n6 * s.v[281]) + (eq86_e1284 * s.dn[281][6]));
        let eq86_e1286_d_n7: f64 = ((eq86_e1284_d_n7 * s.v[281]) + (eq86_e1284 * s.dn[281][7]));
        let eq86_e1286_d_n8: f64 = ((eq86_e1284_d_n8 * s.v[281]) + (eq86_e1284 * s.dn[281][8]));
        let eq86_e1286_d_n9: f64 = ((eq86_e1284_d_n9 * s.v[281]) + (eq86_e1284 * s.dn[281][9]));
        let eq86_e1286_d_n10: f64 = ((eq86_e1284_d_n10 * s.v[281]) + (eq86_e1284 * s.dn[281][10]));
        let eq86_e1286_d_n11: f64 = ((eq86_e1284_d_n11 * s.v[281]) + (eq86_e1284 * s.dn[281][11]));
        let eq86_e1286_d_n12: f64 = ((eq86_e1284_d_n12 * s.v[281]) + (eq86_e1284 * s.dn[281][12]));
        let eq86_e1286_d_n13: f64 = ((eq86_e1284_d_n13 * s.v[281]) + (eq86_e1284 * s.dn[281][13]));
        let eq86_e1286_d_n14: f64 = ((eq86_e1284_d_n14 * s.v[281]) + (eq86_e1284 * s.dn[281][14]));
        let eq86_e1286_d_n15: f64 = ((eq86_e1284_d_n15 * s.v[281]) + (eq86_e1284 * s.dn[281][15]));
        let eq86_e1286_d_n16: f64 = ((eq86_e1284_d_n16 * s.v[281]) + (eq86_e1284 * s.dn[281][16]));
        let eq86_e1286_d_n17: f64 = ((eq86_e1284_d_n17 * s.v[281]) + (eq86_e1284 * s.dn[281][17]));
        let eq86_e1286_d_n18: f64 = ((eq86_e1284_d_n18 * s.v[281]) + (eq86_e1284 * s.dn[281][18]));
        let eq86_e1286_d_n19: f64 = ((eq86_e1284_d_n19 * s.v[281]) + (eq86_e1284 * s.dn[281][19]));
        let eq86_e1286_d_n20: f64 = ((eq86_e1284_d_n20 * s.v[281]) + (eq86_e1284 * s.dn[281][20]));
        let eq86_e1286_d_n21: f64 = ((eq86_e1284_d_n21 * s.v[281]) + (eq86_e1284 * s.dn[281][21]));
        let eq86_e1286_d_n22: f64 = ((eq86_e1284_d_n22 * s.v[281]) + (eq86_e1284 * s.dn[281][22]));
        let eq86_e1289: f64 = (p.p6 * s.v[379]);
        let eq86_e1289_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq86_e1289_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq86_e1289_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq86_e1289_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq86_e1289_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq86_e1289_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq86_e1289_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq86_e1289_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq86_e1289_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq86_e1289_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq86_e1289_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq86_e1289_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq86_e1289_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq86_e1289_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq86_e1289_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq86_e1289_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq86_e1289_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq86_e1289_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq86_e1289_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq86_e1289_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq86_e1289_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq86_e1289_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq86_e1289_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq86_e1291: f64 = (eq86_e1289 * (nv17 - nv16));
        let eq86_e1291_d_n0: f64 = (eq86_e1289_d_n0 * (nv17 - nv16));
        let eq86_e1291_d_n1: f64 = (eq86_e1289_d_n1 * (nv17 - nv16));
        let eq86_e1291_d_n2: f64 = (eq86_e1289_d_n2 * (nv17 - nv16));
        let eq86_e1291_d_n3: f64 = (eq86_e1289_d_n3 * (nv17 - nv16));
        let eq86_e1291_d_n4: f64 = (eq86_e1289_d_n4 * (nv17 - nv16));
        let eq86_e1291_d_n5: f64 = (eq86_e1289_d_n5 * (nv17 - nv16));
        let eq86_e1291_d_n6: f64 = (eq86_e1289_d_n6 * (nv17 - nv16));
        let eq86_e1291_d_n7: f64 = (eq86_e1289_d_n7 * (nv17 - nv16));
        let eq86_e1291_d_n8: f64 = (eq86_e1289_d_n8 * (nv17 - nv16));
        let eq86_e1291_d_n9: f64 = (eq86_e1289_d_n9 * (nv17 - nv16));
        let eq86_e1291_d_n10: f64 = (eq86_e1289_d_n10 * (nv17 - nv16));
        let eq86_e1291_d_n11: f64 = (eq86_e1289_d_n11 * (nv17 - nv16));
        let eq86_e1291_d_n12: f64 = (eq86_e1289_d_n12 * (nv17 - nv16));
        let eq86_e1291_d_n13: f64 = (eq86_e1289_d_n13 * (nv17 - nv16));
        let eq86_e1291_d_n14: f64 = (eq86_e1289_d_n14 * (nv17 - nv16));
        let eq86_e1291_d_n15: f64 = (eq86_e1289_d_n15 * (nv17 - nv16));
        let eq86_e1291_d_n16: f64 = ((eq86_e1289_d_n16 * (nv17 - nv16)) + (-eq86_e1289));
        let eq86_e1291_d_n17: f64 = ((eq86_e1289_d_n17 * (nv17 - nv16)) + eq86_e1289);
        let eq86_e1291_d_n18: f64 = (eq86_e1289_d_n18 * (nv17 - nv16));
        let eq86_e1291_d_n19: f64 = (eq86_e1289_d_n19 * (nv17 - nv16));
        let eq86_e1291_d_n20: f64 = (eq86_e1289_d_n20 * (nv17 - nv16));
        let eq86_e1291_d_n21: f64 = (eq86_e1289_d_n21 * (nv17 - nv16));
        let eq86_e1291_d_n22: f64 = (eq86_e1289_d_n22 * (nv17 - nv16));
        let eq86_e1292: f64 = (eq86_e1286 + eq86_e1291);
        let eq86_e1292_d_n0: f64 = (eq86_e1286_d_n0 + eq86_e1291_d_n0);
        let eq86_e1292_d_n1: f64 = (eq86_e1286_d_n1 + eq86_e1291_d_n1);
        let eq86_e1292_d_n2: f64 = (eq86_e1286_d_n2 + eq86_e1291_d_n2);
        let eq86_e1292_d_n3: f64 = (eq86_e1286_d_n3 + eq86_e1291_d_n3);
        let eq86_e1292_d_n4: f64 = (eq86_e1286_d_n4 + eq86_e1291_d_n4);
        let eq86_e1292_d_n5: f64 = (eq86_e1286_d_n5 + eq86_e1291_d_n5);
        let eq86_e1292_d_n6: f64 = (eq86_e1286_d_n6 + eq86_e1291_d_n6);
        let eq86_e1292_d_n7: f64 = (eq86_e1286_d_n7 + eq86_e1291_d_n7);
        let eq86_e1292_d_n8: f64 = (eq86_e1286_d_n8 + eq86_e1291_d_n8);
        let eq86_e1292_d_n9: f64 = (eq86_e1286_d_n9 + eq86_e1291_d_n9);
        let eq86_e1292_d_n10: f64 = (eq86_e1286_d_n10 + eq86_e1291_d_n10);
        let eq86_e1292_d_n11: f64 = (eq86_e1286_d_n11 + eq86_e1291_d_n11);
        let eq86_e1292_d_n12: f64 = (eq86_e1286_d_n12 + eq86_e1291_d_n12);
        let eq86_e1292_d_n13: f64 = (eq86_e1286_d_n13 + eq86_e1291_d_n13);
        let eq86_e1292_d_n14: f64 = (eq86_e1286_d_n14 + eq86_e1291_d_n14);
        let eq86_e1292_d_n15: f64 = (eq86_e1286_d_n15 + eq86_e1291_d_n15);
        let eq86_e1292_d_n16: f64 = (eq86_e1286_d_n16 + eq86_e1291_d_n16);
        let eq86_e1292_d_n17: f64 = (eq86_e1286_d_n17 + eq86_e1291_d_n17);
        let eq86_e1292_d_n18: f64 = (eq86_e1286_d_n18 + eq86_e1291_d_n18);
        let eq86_e1292_d_n19: f64 = (eq86_e1286_d_n19 + eq86_e1291_d_n19);
        let eq86_e1292_d_n20: f64 = (eq86_e1286_d_n20 + eq86_e1291_d_n20);
        let eq86_e1292_d_n21: f64 = (eq86_e1286_d_n21 + eq86_e1291_d_n21);
        let eq86_e1292_d_n22: f64 = (eq86_e1286_d_n22 + eq86_e1291_d_n22);
        (eq86_e1292, eq86_e1292_d_n0, eq86_e1292_d_n1, eq86_e1292_d_n2, eq86_e1292_d_n3, eq86_e1292_d_n4, eq86_e1292_d_n5, eq86_e1292_d_n6, eq86_e1292_d_n7, eq86_e1292_d_n8, eq86_e1292_d_n9, eq86_e1292_d_n10, eq86_e1292_d_n11, eq86_e1292_d_n12, eq86_e1292_d_n13, eq86_e1292_d_n14, eq86_e1292_d_n15, eq86_e1292_d_n16, eq86_e1292_d_n17, eq86_e1292_d_n18, eq86_e1292_d_n19, eq86_e1292_d_n20, eq86_e1292_d_n21, eq86_e1292_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq86_value: f64 = eq86_e1294;
        let eq86_node_derivatives: [f64; 23] = [eq86_e1294_d_n0, eq86_e1294_d_n1, eq86_e1294_d_n2, eq86_e1294_d_n3, eq86_e1294_d_n4, eq86_e1294_d_n5, eq86_e1294_d_n6, eq86_e1294_d_n7, eq86_e1294_d_n8, eq86_e1294_d_n9, eq86_e1294_d_n10, eq86_e1294_d_n11, eq86_e1294_d_n12, eq86_e1294_d_n13, eq86_e1294_d_n14, eq86_e1294_d_n15, eq86_e1294_d_n16, eq86_e1294_d_n17, eq86_e1294_d_n18, eq86_e1294_d_n19, eq86_e1294_d_n20, eq86_e1294_d_n21, eq86_e1294_d_n22];
        let eq86_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[17]),
            Some(nodes[16]),
            self.multiplicity * (eq86_value),
            &nodes,
            &eq86_node_derivatives,
            &branches,
            &eq86_branch_derivatives,
            self.multiplicity,
        );
    }
}
