#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_87_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq87_e1301,) = {
    if ((s.v[493] != 0.0) && (!(s.v[494] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq87_value: f64 = eq87_e1301;
        stamper.stamp_potential(
            branches[43],
            eq87_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_88_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq88_e1306,) = {
    if (!(s.v[493] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq88_value: f64 = eq88_e1306;
        stamper.stamp_potential(
            branches[44],
            eq88_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_89_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv20 = ctx.node_voltage(nodes[20]);
        let nv21 = ctx.node_voltage(nodes[21]);
        let (eq89_e1322, eq89_e1322_d_n0, eq89_e1322_d_n1, eq89_e1322_d_n2, eq89_e1322_d_n3, eq89_e1322_d_n4, eq89_e1322_d_n5, eq89_e1322_d_n6, eq89_e1322_d_n7, eq89_e1322_d_n8, eq89_e1322_d_n9, eq89_e1322_d_n10, eq89_e1322_d_n11, eq89_e1322_d_n12, eq89_e1322_d_n13, eq89_e1322_d_n14, eq89_e1322_d_n15, eq89_e1322_d_n16, eq89_e1322_d_n17, eq89_e1322_d_n18, eq89_e1322_d_n19, eq89_e1322_d_n20, eq89_e1322_d_n21, eq89_e1322_d_n22,) = {
    if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
        let eq89_e1312: f64 = (p.p6 * s.v[68]);
        let eq89_e1312_d_n0: f64 = (p.p6 * s.dn[68][0]);
        let eq89_e1312_d_n1: f64 = (p.p6 * s.dn[68][1]);
        let eq89_e1312_d_n2: f64 = (p.p6 * s.dn[68][2]);
        let eq89_e1312_d_n3: f64 = (p.p6 * s.dn[68][3]);
        let eq89_e1312_d_n4: f64 = (p.p6 * s.dn[68][4]);
        let eq89_e1312_d_n5: f64 = (p.p6 * s.dn[68][5]);
        let eq89_e1312_d_n6: f64 = (p.p6 * s.dn[68][6]);
        let eq89_e1312_d_n7: f64 = (p.p6 * s.dn[68][7]);
        let eq89_e1312_d_n8: f64 = (p.p6 * s.dn[68][8]);
        let eq89_e1312_d_n9: f64 = (p.p6 * s.dn[68][9]);
        let eq89_e1312_d_n10: f64 = (p.p6 * s.dn[68][10]);
        let eq89_e1312_d_n11: f64 = (p.p6 * s.dn[68][11]);
        let eq89_e1312_d_n12: f64 = (p.p6 * s.dn[68][12]);
        let eq89_e1312_d_n13: f64 = (p.p6 * s.dn[68][13]);
        let eq89_e1312_d_n14: f64 = (p.p6 * s.dn[68][14]);
        let eq89_e1312_d_n15: f64 = (p.p6 * s.dn[68][15]);
        let eq89_e1312_d_n16: f64 = (p.p6 * s.dn[68][16]);
        let eq89_e1312_d_n17: f64 = (p.p6 * s.dn[68][17]);
        let eq89_e1312_d_n18: f64 = (p.p6 * s.dn[68][18]);
        let eq89_e1312_d_n19: f64 = (p.p6 * s.dn[68][19]);
        let eq89_e1312_d_n20: f64 = (p.p6 * s.dn[68][20]);
        let eq89_e1312_d_n21: f64 = (p.p6 * s.dn[68][21]);
        let eq89_e1312_d_n22: f64 = (p.p6 * s.dn[68][22]);
        let eq89_e1314: f64 = (eq89_e1312 * s.v[293]);
        let eq89_e1314_d_n0: f64 = ((eq89_e1312_d_n0 * s.v[293]) + (eq89_e1312 * s.dn[293][0]));
        let eq89_e1314_d_n1: f64 = ((eq89_e1312_d_n1 * s.v[293]) + (eq89_e1312 * s.dn[293][1]));
        let eq89_e1314_d_n2: f64 = ((eq89_e1312_d_n2 * s.v[293]) + (eq89_e1312 * s.dn[293][2]));
        let eq89_e1314_d_n3: f64 = ((eq89_e1312_d_n3 * s.v[293]) + (eq89_e1312 * s.dn[293][3]));
        let eq89_e1314_d_n4: f64 = ((eq89_e1312_d_n4 * s.v[293]) + (eq89_e1312 * s.dn[293][4]));
        let eq89_e1314_d_n5: f64 = ((eq89_e1312_d_n5 * s.v[293]) + (eq89_e1312 * s.dn[293][5]));
        let eq89_e1314_d_n6: f64 = ((eq89_e1312_d_n6 * s.v[293]) + (eq89_e1312 * s.dn[293][6]));
        let eq89_e1314_d_n7: f64 = ((eq89_e1312_d_n7 * s.v[293]) + (eq89_e1312 * s.dn[293][7]));
        let eq89_e1314_d_n8: f64 = ((eq89_e1312_d_n8 * s.v[293]) + (eq89_e1312 * s.dn[293][8]));
        let eq89_e1314_d_n9: f64 = ((eq89_e1312_d_n9 * s.v[293]) + (eq89_e1312 * s.dn[293][9]));
        let eq89_e1314_d_n10: f64 = ((eq89_e1312_d_n10 * s.v[293]) + (eq89_e1312 * s.dn[293][10]));
        let eq89_e1314_d_n11: f64 = ((eq89_e1312_d_n11 * s.v[293]) + (eq89_e1312 * s.dn[293][11]));
        let eq89_e1314_d_n12: f64 = ((eq89_e1312_d_n12 * s.v[293]) + (eq89_e1312 * s.dn[293][12]));
        let eq89_e1314_d_n13: f64 = ((eq89_e1312_d_n13 * s.v[293]) + (eq89_e1312 * s.dn[293][13]));
        let eq89_e1314_d_n14: f64 = ((eq89_e1312_d_n14 * s.v[293]) + (eq89_e1312 * s.dn[293][14]));
        let eq89_e1314_d_n15: f64 = ((eq89_e1312_d_n15 * s.v[293]) + (eq89_e1312 * s.dn[293][15]));
        let eq89_e1314_d_n16: f64 = ((eq89_e1312_d_n16 * s.v[293]) + (eq89_e1312 * s.dn[293][16]));
        let eq89_e1314_d_n17: f64 = ((eq89_e1312_d_n17 * s.v[293]) + (eq89_e1312 * s.dn[293][17]));
        let eq89_e1314_d_n18: f64 = ((eq89_e1312_d_n18 * s.v[293]) + (eq89_e1312 * s.dn[293][18]));
        let eq89_e1314_d_n19: f64 = ((eq89_e1312_d_n19 * s.v[293]) + (eq89_e1312 * s.dn[293][19]));
        let eq89_e1314_d_n20: f64 = ((eq89_e1312_d_n20 * s.v[293]) + (eq89_e1312 * s.dn[293][20]));
        let eq89_e1314_d_n21: f64 = ((eq89_e1312_d_n21 * s.v[293]) + (eq89_e1312 * s.dn[293][21]));
        let eq89_e1314_d_n22: f64 = ((eq89_e1312_d_n22 * s.v[293]) + (eq89_e1312 * s.dn[293][22]));
        let eq89_e1317: f64 = (p.p6 * s.v[379]);
        let eq89_e1317_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq89_e1317_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq89_e1317_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq89_e1317_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq89_e1317_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq89_e1317_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq89_e1317_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq89_e1317_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq89_e1317_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq89_e1317_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq89_e1317_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq89_e1317_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq89_e1317_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq89_e1317_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq89_e1317_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq89_e1317_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq89_e1317_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq89_e1317_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq89_e1317_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq89_e1317_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq89_e1317_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq89_e1317_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq89_e1317_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq89_e1319: f64 = (eq89_e1317 * (nv20 - nv21));
        let eq89_e1319_d_n0: f64 = (eq89_e1317_d_n0 * (nv20 - nv21));
        let eq89_e1319_d_n1: f64 = (eq89_e1317_d_n1 * (nv20 - nv21));
        let eq89_e1319_d_n2: f64 = (eq89_e1317_d_n2 * (nv20 - nv21));
        let eq89_e1319_d_n3: f64 = (eq89_e1317_d_n3 * (nv20 - nv21));
        let eq89_e1319_d_n4: f64 = (eq89_e1317_d_n4 * (nv20 - nv21));
        let eq89_e1319_d_n5: f64 = (eq89_e1317_d_n5 * (nv20 - nv21));
        let eq89_e1319_d_n6: f64 = (eq89_e1317_d_n6 * (nv20 - nv21));
        let eq89_e1319_d_n7: f64 = (eq89_e1317_d_n7 * (nv20 - nv21));
        let eq89_e1319_d_n8: f64 = (eq89_e1317_d_n8 * (nv20 - nv21));
        let eq89_e1319_d_n9: f64 = (eq89_e1317_d_n9 * (nv20 - nv21));
        let eq89_e1319_d_n10: f64 = (eq89_e1317_d_n10 * (nv20 - nv21));
        let eq89_e1319_d_n11: f64 = (eq89_e1317_d_n11 * (nv20 - nv21));
        let eq89_e1319_d_n12: f64 = (eq89_e1317_d_n12 * (nv20 - nv21));
        let eq89_e1319_d_n13: f64 = (eq89_e1317_d_n13 * (nv20 - nv21));
        let eq89_e1319_d_n14: f64 = (eq89_e1317_d_n14 * (nv20 - nv21));
        let eq89_e1319_d_n15: f64 = (eq89_e1317_d_n15 * (nv20 - nv21));
        let eq89_e1319_d_n16: f64 = (eq89_e1317_d_n16 * (nv20 - nv21));
        let eq89_e1319_d_n17: f64 = (eq89_e1317_d_n17 * (nv20 - nv21));
        let eq89_e1319_d_n18: f64 = (eq89_e1317_d_n18 * (nv20 - nv21));
        let eq89_e1319_d_n19: f64 = (eq89_e1317_d_n19 * (nv20 - nv21));
        let eq89_e1319_d_n20: f64 = ((eq89_e1317_d_n20 * (nv20 - nv21)) + eq89_e1317);
        let eq89_e1319_d_n21: f64 = ((eq89_e1317_d_n21 * (nv20 - nv21)) + (-eq89_e1317));
        let eq89_e1319_d_n22: f64 = (eq89_e1317_d_n22 * (nv20 - nv21));
        let eq89_e1320: f64 = (eq89_e1314 + eq89_e1319);
        let eq89_e1320_d_n0: f64 = (eq89_e1314_d_n0 + eq89_e1319_d_n0);
        let eq89_e1320_d_n1: f64 = (eq89_e1314_d_n1 + eq89_e1319_d_n1);
        let eq89_e1320_d_n2: f64 = (eq89_e1314_d_n2 + eq89_e1319_d_n2);
        let eq89_e1320_d_n3: f64 = (eq89_e1314_d_n3 + eq89_e1319_d_n3);
        let eq89_e1320_d_n4: f64 = (eq89_e1314_d_n4 + eq89_e1319_d_n4);
        let eq89_e1320_d_n5: f64 = (eq89_e1314_d_n5 + eq89_e1319_d_n5);
        let eq89_e1320_d_n6: f64 = (eq89_e1314_d_n6 + eq89_e1319_d_n6);
        let eq89_e1320_d_n7: f64 = (eq89_e1314_d_n7 + eq89_e1319_d_n7);
        let eq89_e1320_d_n8: f64 = (eq89_e1314_d_n8 + eq89_e1319_d_n8);
        let eq89_e1320_d_n9: f64 = (eq89_e1314_d_n9 + eq89_e1319_d_n9);
        let eq89_e1320_d_n10: f64 = (eq89_e1314_d_n10 + eq89_e1319_d_n10);
        let eq89_e1320_d_n11: f64 = (eq89_e1314_d_n11 + eq89_e1319_d_n11);
        let eq89_e1320_d_n12: f64 = (eq89_e1314_d_n12 + eq89_e1319_d_n12);
        let eq89_e1320_d_n13: f64 = (eq89_e1314_d_n13 + eq89_e1319_d_n13);
        let eq89_e1320_d_n14: f64 = (eq89_e1314_d_n14 + eq89_e1319_d_n14);
        let eq89_e1320_d_n15: f64 = (eq89_e1314_d_n15 + eq89_e1319_d_n15);
        let eq89_e1320_d_n16: f64 = (eq89_e1314_d_n16 + eq89_e1319_d_n16);
        let eq89_e1320_d_n17: f64 = (eq89_e1314_d_n17 + eq89_e1319_d_n17);
        let eq89_e1320_d_n18: f64 = (eq89_e1314_d_n18 + eq89_e1319_d_n18);
        let eq89_e1320_d_n19: f64 = (eq89_e1314_d_n19 + eq89_e1319_d_n19);
        let eq89_e1320_d_n20: f64 = (eq89_e1314_d_n20 + eq89_e1319_d_n20);
        let eq89_e1320_d_n21: f64 = (eq89_e1314_d_n21 + eq89_e1319_d_n21);
        let eq89_e1320_d_n22: f64 = (eq89_e1314_d_n22 + eq89_e1319_d_n22);
        (eq89_e1320, eq89_e1320_d_n0, eq89_e1320_d_n1, eq89_e1320_d_n2, eq89_e1320_d_n3, eq89_e1320_d_n4, eq89_e1320_d_n5, eq89_e1320_d_n6, eq89_e1320_d_n7, eq89_e1320_d_n8, eq89_e1320_d_n9, eq89_e1320_d_n10, eq89_e1320_d_n11, eq89_e1320_d_n12, eq89_e1320_d_n13, eq89_e1320_d_n14, eq89_e1320_d_n15, eq89_e1320_d_n16, eq89_e1320_d_n17, eq89_e1320_d_n18, eq89_e1320_d_n19, eq89_e1320_d_n20, eq89_e1320_d_n21, eq89_e1320_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq89_value: f64 = eq89_e1322;
        let eq89_node_derivatives: [f64; 23] = [eq89_e1322_d_n0, eq89_e1322_d_n1, eq89_e1322_d_n2, eq89_e1322_d_n3, eq89_e1322_d_n4, eq89_e1322_d_n5, eq89_e1322_d_n6, eq89_e1322_d_n7, eq89_e1322_d_n8, eq89_e1322_d_n9, eq89_e1322_d_n10, eq89_e1322_d_n11, eq89_e1322_d_n12, eq89_e1322_d_n13, eq89_e1322_d_n14, eq89_e1322_d_n15, eq89_e1322_d_n16, eq89_e1322_d_n17, eq89_e1322_d_n18, eq89_e1322_d_n19, eq89_e1322_d_n20, eq89_e1322_d_n21, eq89_e1322_d_n22];
        let eq89_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[20]),
            Some(nodes[21]),
            self.multiplicity * (eq89_value),
            &nodes,
            &eq89_node_derivatives,
            &branches,
            &eq89_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_90_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq90_e1329,) = {
    if ((s.v[508] != 0.0) && (!(s.v[509] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq90_value: f64 = eq90_e1329;
        stamper.stamp_potential(
            branches[45],
            eq90_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_91_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq91_e1334,) = {
    if (!(s.v[508] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq91_value: f64 = eq91_e1334;
        stamper.stamp_potential(
            branches[46],
            eq91_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_92_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq92_e1342,) = {
    if ((!(s.v[508] != 0.0)) && (!(s.v[517] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq92_value: f64 = eq92_e1342;
        stamper.stamp_potential(
            branches[47],
            eq92_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_93_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let (eq93_e1358, eq93_e1358_d_n0, eq93_e1358_d_n1, eq93_e1358_d_n2, eq93_e1358_d_n3, eq93_e1358_d_n4, eq93_e1358_d_n5, eq93_e1358_d_n6, eq93_e1358_d_n7, eq93_e1358_d_n8, eq93_e1358_d_n9, eq93_e1358_d_n10, eq93_e1358_d_n11, eq93_e1358_d_n12, eq93_e1358_d_n13, eq93_e1358_d_n14, eq93_e1358_d_n15, eq93_e1358_d_n16, eq93_e1358_d_n17, eq93_e1358_d_n18, eq93_e1358_d_n19, eq93_e1358_d_n20, eq93_e1358_d_n21, eq93_e1358_d_n22,) = {
    if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
        let eq93_e1348: f64 = (p.p6 * s.v[72]);
        let eq93_e1348_d_n0: f64 = (p.p6 * s.dn[72][0]);
        let eq93_e1348_d_n1: f64 = (p.p6 * s.dn[72][1]);
        let eq93_e1348_d_n2: f64 = (p.p6 * s.dn[72][2]);
        let eq93_e1348_d_n3: f64 = (p.p6 * s.dn[72][3]);
        let eq93_e1348_d_n4: f64 = (p.p6 * s.dn[72][4]);
        let eq93_e1348_d_n5: f64 = (p.p6 * s.dn[72][5]);
        let eq93_e1348_d_n6: f64 = (p.p6 * s.dn[72][6]);
        let eq93_e1348_d_n7: f64 = (p.p6 * s.dn[72][7]);
        let eq93_e1348_d_n8: f64 = (p.p6 * s.dn[72][8]);
        let eq93_e1348_d_n9: f64 = (p.p6 * s.dn[72][9]);
        let eq93_e1348_d_n10: f64 = (p.p6 * s.dn[72][10]);
        let eq93_e1348_d_n11: f64 = (p.p6 * s.dn[72][11]);
        let eq93_e1348_d_n12: f64 = (p.p6 * s.dn[72][12]);
        let eq93_e1348_d_n13: f64 = (p.p6 * s.dn[72][13]);
        let eq93_e1348_d_n14: f64 = (p.p6 * s.dn[72][14]);
        let eq93_e1348_d_n15: f64 = (p.p6 * s.dn[72][15]);
        let eq93_e1348_d_n16: f64 = (p.p6 * s.dn[72][16]);
        let eq93_e1348_d_n17: f64 = (p.p6 * s.dn[72][17]);
        let eq93_e1348_d_n18: f64 = (p.p6 * s.dn[72][18]);
        let eq93_e1348_d_n19: f64 = (p.p6 * s.dn[72][19]);
        let eq93_e1348_d_n20: f64 = (p.p6 * s.dn[72][20]);
        let eq93_e1348_d_n21: f64 = (p.p6 * s.dn[72][21]);
        let eq93_e1348_d_n22: f64 = (p.p6 * s.dn[72][22]);
        let eq93_e1350: f64 = (eq93_e1348 * s.v[305]);
        let eq93_e1350_d_n0: f64 = ((eq93_e1348_d_n0 * s.v[305]) + (eq93_e1348 * s.dn[305][0]));
        let eq93_e1350_d_n1: f64 = ((eq93_e1348_d_n1 * s.v[305]) + (eq93_e1348 * s.dn[305][1]));
        let eq93_e1350_d_n2: f64 = ((eq93_e1348_d_n2 * s.v[305]) + (eq93_e1348 * s.dn[305][2]));
        let eq93_e1350_d_n3: f64 = ((eq93_e1348_d_n3 * s.v[305]) + (eq93_e1348 * s.dn[305][3]));
        let eq93_e1350_d_n4: f64 = ((eq93_e1348_d_n4 * s.v[305]) + (eq93_e1348 * s.dn[305][4]));
        let eq93_e1350_d_n5: f64 = ((eq93_e1348_d_n5 * s.v[305]) + (eq93_e1348 * s.dn[305][5]));
        let eq93_e1350_d_n6: f64 = ((eq93_e1348_d_n6 * s.v[305]) + (eq93_e1348 * s.dn[305][6]));
        let eq93_e1350_d_n7: f64 = ((eq93_e1348_d_n7 * s.v[305]) + (eq93_e1348 * s.dn[305][7]));
        let eq93_e1350_d_n8: f64 = ((eq93_e1348_d_n8 * s.v[305]) + (eq93_e1348 * s.dn[305][8]));
        let eq93_e1350_d_n9: f64 = ((eq93_e1348_d_n9 * s.v[305]) + (eq93_e1348 * s.dn[305][9]));
        let eq93_e1350_d_n10: f64 = ((eq93_e1348_d_n10 * s.v[305]) + (eq93_e1348 * s.dn[305][10]));
        let eq93_e1350_d_n11: f64 = ((eq93_e1348_d_n11 * s.v[305]) + (eq93_e1348 * s.dn[305][11]));
        let eq93_e1350_d_n12: f64 = ((eq93_e1348_d_n12 * s.v[305]) + (eq93_e1348 * s.dn[305][12]));
        let eq93_e1350_d_n13: f64 = ((eq93_e1348_d_n13 * s.v[305]) + (eq93_e1348 * s.dn[305][13]));
        let eq93_e1350_d_n14: f64 = ((eq93_e1348_d_n14 * s.v[305]) + (eq93_e1348 * s.dn[305][14]));
        let eq93_e1350_d_n15: f64 = ((eq93_e1348_d_n15 * s.v[305]) + (eq93_e1348 * s.dn[305][15]));
        let eq93_e1350_d_n16: f64 = ((eq93_e1348_d_n16 * s.v[305]) + (eq93_e1348 * s.dn[305][16]));
        let eq93_e1350_d_n17: f64 = ((eq93_e1348_d_n17 * s.v[305]) + (eq93_e1348 * s.dn[305][17]));
        let eq93_e1350_d_n18: f64 = ((eq93_e1348_d_n18 * s.v[305]) + (eq93_e1348 * s.dn[305][18]));
        let eq93_e1350_d_n19: f64 = ((eq93_e1348_d_n19 * s.v[305]) + (eq93_e1348 * s.dn[305][19]));
        let eq93_e1350_d_n20: f64 = ((eq93_e1348_d_n20 * s.v[305]) + (eq93_e1348 * s.dn[305][20]));
        let eq93_e1350_d_n21: f64 = ((eq93_e1348_d_n21 * s.v[305]) + (eq93_e1348 * s.dn[305][21]));
        let eq93_e1350_d_n22: f64 = ((eq93_e1348_d_n22 * s.v[305]) + (eq93_e1348 * s.dn[305][22]));
        let eq93_e1353: f64 = (p.p6 * s.v[379]);
        let eq93_e1353_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq93_e1353_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq93_e1353_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq93_e1353_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq93_e1353_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq93_e1353_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq93_e1353_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq93_e1353_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq93_e1353_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq93_e1353_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq93_e1353_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq93_e1353_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq93_e1353_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq93_e1353_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq93_e1353_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq93_e1353_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq93_e1353_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq93_e1353_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq93_e1353_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq93_e1353_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq93_e1353_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq93_e1353_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq93_e1353_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq93_e1355: f64 = (eq93_e1353 * (nv18 - nv17));
        let eq93_e1355_d_n0: f64 = (eq93_e1353_d_n0 * (nv18 - nv17));
        let eq93_e1355_d_n1: f64 = (eq93_e1353_d_n1 * (nv18 - nv17));
        let eq93_e1355_d_n2: f64 = (eq93_e1353_d_n2 * (nv18 - nv17));
        let eq93_e1355_d_n3: f64 = (eq93_e1353_d_n3 * (nv18 - nv17));
        let eq93_e1355_d_n4: f64 = (eq93_e1353_d_n4 * (nv18 - nv17));
        let eq93_e1355_d_n5: f64 = (eq93_e1353_d_n5 * (nv18 - nv17));
        let eq93_e1355_d_n6: f64 = (eq93_e1353_d_n6 * (nv18 - nv17));
        let eq93_e1355_d_n7: f64 = (eq93_e1353_d_n7 * (nv18 - nv17));
        let eq93_e1355_d_n8: f64 = (eq93_e1353_d_n8 * (nv18 - nv17));
        let eq93_e1355_d_n9: f64 = (eq93_e1353_d_n9 * (nv18 - nv17));
        let eq93_e1355_d_n10: f64 = (eq93_e1353_d_n10 * (nv18 - nv17));
        let eq93_e1355_d_n11: f64 = (eq93_e1353_d_n11 * (nv18 - nv17));
        let eq93_e1355_d_n12: f64 = (eq93_e1353_d_n12 * (nv18 - nv17));
        let eq93_e1355_d_n13: f64 = (eq93_e1353_d_n13 * (nv18 - nv17));
        let eq93_e1355_d_n14: f64 = (eq93_e1353_d_n14 * (nv18 - nv17));
        let eq93_e1355_d_n15: f64 = (eq93_e1353_d_n15 * (nv18 - nv17));
        let eq93_e1355_d_n16: f64 = (eq93_e1353_d_n16 * (nv18 - nv17));
        let eq93_e1355_d_n17: f64 = ((eq93_e1353_d_n17 * (nv18 - nv17)) + (-eq93_e1353));
        let eq93_e1355_d_n18: f64 = ((eq93_e1353_d_n18 * (nv18 - nv17)) + eq93_e1353);
        let eq93_e1355_d_n19: f64 = (eq93_e1353_d_n19 * (nv18 - nv17));
        let eq93_e1355_d_n20: f64 = (eq93_e1353_d_n20 * (nv18 - nv17));
        let eq93_e1355_d_n21: f64 = (eq93_e1353_d_n21 * (nv18 - nv17));
        let eq93_e1355_d_n22: f64 = (eq93_e1353_d_n22 * (nv18 - nv17));
        let eq93_e1356: f64 = (eq93_e1350 + eq93_e1355);
        let eq93_e1356_d_n0: f64 = (eq93_e1350_d_n0 + eq93_e1355_d_n0);
        let eq93_e1356_d_n1: f64 = (eq93_e1350_d_n1 + eq93_e1355_d_n1);
        let eq93_e1356_d_n2: f64 = (eq93_e1350_d_n2 + eq93_e1355_d_n2);
        let eq93_e1356_d_n3: f64 = (eq93_e1350_d_n3 + eq93_e1355_d_n3);
        let eq93_e1356_d_n4: f64 = (eq93_e1350_d_n4 + eq93_e1355_d_n4);
        let eq93_e1356_d_n5: f64 = (eq93_e1350_d_n5 + eq93_e1355_d_n5);
        let eq93_e1356_d_n6: f64 = (eq93_e1350_d_n6 + eq93_e1355_d_n6);
        let eq93_e1356_d_n7: f64 = (eq93_e1350_d_n7 + eq93_e1355_d_n7);
        let eq93_e1356_d_n8: f64 = (eq93_e1350_d_n8 + eq93_e1355_d_n8);
        let eq93_e1356_d_n9: f64 = (eq93_e1350_d_n9 + eq93_e1355_d_n9);
        let eq93_e1356_d_n10: f64 = (eq93_e1350_d_n10 + eq93_e1355_d_n10);
        let eq93_e1356_d_n11: f64 = (eq93_e1350_d_n11 + eq93_e1355_d_n11);
        let eq93_e1356_d_n12: f64 = (eq93_e1350_d_n12 + eq93_e1355_d_n12);
        let eq93_e1356_d_n13: f64 = (eq93_e1350_d_n13 + eq93_e1355_d_n13);
        let eq93_e1356_d_n14: f64 = (eq93_e1350_d_n14 + eq93_e1355_d_n14);
        let eq93_e1356_d_n15: f64 = (eq93_e1350_d_n15 + eq93_e1355_d_n15);
        let eq93_e1356_d_n16: f64 = (eq93_e1350_d_n16 + eq93_e1355_d_n16);
        let eq93_e1356_d_n17: f64 = (eq93_e1350_d_n17 + eq93_e1355_d_n17);
        let eq93_e1356_d_n18: f64 = (eq93_e1350_d_n18 + eq93_e1355_d_n18);
        let eq93_e1356_d_n19: f64 = (eq93_e1350_d_n19 + eq93_e1355_d_n19);
        let eq93_e1356_d_n20: f64 = (eq93_e1350_d_n20 + eq93_e1355_d_n20);
        let eq93_e1356_d_n21: f64 = (eq93_e1350_d_n21 + eq93_e1355_d_n21);
        let eq93_e1356_d_n22: f64 = (eq93_e1350_d_n22 + eq93_e1355_d_n22);
        (eq93_e1356, eq93_e1356_d_n0, eq93_e1356_d_n1, eq93_e1356_d_n2, eq93_e1356_d_n3, eq93_e1356_d_n4, eq93_e1356_d_n5, eq93_e1356_d_n6, eq93_e1356_d_n7, eq93_e1356_d_n8, eq93_e1356_d_n9, eq93_e1356_d_n10, eq93_e1356_d_n11, eq93_e1356_d_n12, eq93_e1356_d_n13, eq93_e1356_d_n14, eq93_e1356_d_n15, eq93_e1356_d_n16, eq93_e1356_d_n17, eq93_e1356_d_n18, eq93_e1356_d_n19, eq93_e1356_d_n20, eq93_e1356_d_n21, eq93_e1356_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq93_value: f64 = eq93_e1358;
        let eq93_node_derivatives: [f64; 23] = [eq93_e1358_d_n0, eq93_e1358_d_n1, eq93_e1358_d_n2, eq93_e1358_d_n3, eq93_e1358_d_n4, eq93_e1358_d_n5, eq93_e1358_d_n6, eq93_e1358_d_n7, eq93_e1358_d_n8, eq93_e1358_d_n9, eq93_e1358_d_n10, eq93_e1358_d_n11, eq93_e1358_d_n12, eq93_e1358_d_n13, eq93_e1358_d_n14, eq93_e1358_d_n15, eq93_e1358_d_n16, eq93_e1358_d_n17, eq93_e1358_d_n18, eq93_e1358_d_n19, eq93_e1358_d_n20, eq93_e1358_d_n21, eq93_e1358_d_n22];
        let eq93_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[18]),
            Some(nodes[17]),
            self.multiplicity * (eq93_value),
            &nodes,
            &eq93_node_derivatives,
            &branches,
            &eq93_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_94_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq94_e1365,) = {
    if ((s.v[523] != 0.0) && (!(s.v[524] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq94_value: f64 = eq94_e1365;
        stamper.stamp_potential(
            branches[48],
            eq94_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_95_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq95_e1370,) = {
    if (!(s.v[523] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq95_value: f64 = eq95_e1370;
        stamper.stamp_potential(
            branches[49],
            eq95_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_96_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv21 = ctx.node_voltage(nodes[21]);
        let nv22 = ctx.node_voltage(nodes[22]);
        let (eq96_e1386, eq96_e1386_d_n0, eq96_e1386_d_n1, eq96_e1386_d_n2, eq96_e1386_d_n3, eq96_e1386_d_n4, eq96_e1386_d_n5, eq96_e1386_d_n6, eq96_e1386_d_n7, eq96_e1386_d_n8, eq96_e1386_d_n9, eq96_e1386_d_n10, eq96_e1386_d_n11, eq96_e1386_d_n12, eq96_e1386_d_n13, eq96_e1386_d_n14, eq96_e1386_d_n15, eq96_e1386_d_n16, eq96_e1386_d_n17, eq96_e1386_d_n18, eq96_e1386_d_n19, eq96_e1386_d_n20, eq96_e1386_d_n21, eq96_e1386_d_n22,) = {
    if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
        let eq96_e1376: f64 = (p.p6 * s.v[76]);
        let eq96_e1376_d_n0: f64 = (p.p6 * s.dn[76][0]);
        let eq96_e1376_d_n1: f64 = (p.p6 * s.dn[76][1]);
        let eq96_e1376_d_n2: f64 = (p.p6 * s.dn[76][2]);
        let eq96_e1376_d_n3: f64 = (p.p6 * s.dn[76][3]);
        let eq96_e1376_d_n4: f64 = (p.p6 * s.dn[76][4]);
        let eq96_e1376_d_n5: f64 = (p.p6 * s.dn[76][5]);
        let eq96_e1376_d_n6: f64 = (p.p6 * s.dn[76][6]);
        let eq96_e1376_d_n7: f64 = (p.p6 * s.dn[76][7]);
        let eq96_e1376_d_n8: f64 = (p.p6 * s.dn[76][8]);
        let eq96_e1376_d_n9: f64 = (p.p6 * s.dn[76][9]);
        let eq96_e1376_d_n10: f64 = (p.p6 * s.dn[76][10]);
        let eq96_e1376_d_n11: f64 = (p.p6 * s.dn[76][11]);
        let eq96_e1376_d_n12: f64 = (p.p6 * s.dn[76][12]);
        let eq96_e1376_d_n13: f64 = (p.p6 * s.dn[76][13]);
        let eq96_e1376_d_n14: f64 = (p.p6 * s.dn[76][14]);
        let eq96_e1376_d_n15: f64 = (p.p6 * s.dn[76][15]);
        let eq96_e1376_d_n16: f64 = (p.p6 * s.dn[76][16]);
        let eq96_e1376_d_n17: f64 = (p.p6 * s.dn[76][17]);
        let eq96_e1376_d_n18: f64 = (p.p6 * s.dn[76][18]);
        let eq96_e1376_d_n19: f64 = (p.p6 * s.dn[76][19]);
        let eq96_e1376_d_n20: f64 = (p.p6 * s.dn[76][20]);
        let eq96_e1376_d_n21: f64 = (p.p6 * s.dn[76][21]);
        let eq96_e1376_d_n22: f64 = (p.p6 * s.dn[76][22]);
        let eq96_e1378: f64 = (eq96_e1376 * s.v[317]);
        let eq96_e1378_d_n0: f64 = ((eq96_e1376_d_n0 * s.v[317]) + (eq96_e1376 * s.dn[317][0]));
        let eq96_e1378_d_n1: f64 = ((eq96_e1376_d_n1 * s.v[317]) + (eq96_e1376 * s.dn[317][1]));
        let eq96_e1378_d_n2: f64 = ((eq96_e1376_d_n2 * s.v[317]) + (eq96_e1376 * s.dn[317][2]));
        let eq96_e1378_d_n3: f64 = ((eq96_e1376_d_n3 * s.v[317]) + (eq96_e1376 * s.dn[317][3]));
        let eq96_e1378_d_n4: f64 = ((eq96_e1376_d_n4 * s.v[317]) + (eq96_e1376 * s.dn[317][4]));
        let eq96_e1378_d_n5: f64 = ((eq96_e1376_d_n5 * s.v[317]) + (eq96_e1376 * s.dn[317][5]));
        let eq96_e1378_d_n6: f64 = ((eq96_e1376_d_n6 * s.v[317]) + (eq96_e1376 * s.dn[317][6]));
        let eq96_e1378_d_n7: f64 = ((eq96_e1376_d_n7 * s.v[317]) + (eq96_e1376 * s.dn[317][7]));
        let eq96_e1378_d_n8: f64 = ((eq96_e1376_d_n8 * s.v[317]) + (eq96_e1376 * s.dn[317][8]));
        let eq96_e1378_d_n9: f64 = ((eq96_e1376_d_n9 * s.v[317]) + (eq96_e1376 * s.dn[317][9]));
        let eq96_e1378_d_n10: f64 = ((eq96_e1376_d_n10 * s.v[317]) + (eq96_e1376 * s.dn[317][10]));
        let eq96_e1378_d_n11: f64 = ((eq96_e1376_d_n11 * s.v[317]) + (eq96_e1376 * s.dn[317][11]));
        let eq96_e1378_d_n12: f64 = ((eq96_e1376_d_n12 * s.v[317]) + (eq96_e1376 * s.dn[317][12]));
        let eq96_e1378_d_n13: f64 = ((eq96_e1376_d_n13 * s.v[317]) + (eq96_e1376 * s.dn[317][13]));
        let eq96_e1378_d_n14: f64 = ((eq96_e1376_d_n14 * s.v[317]) + (eq96_e1376 * s.dn[317][14]));
        let eq96_e1378_d_n15: f64 = ((eq96_e1376_d_n15 * s.v[317]) + (eq96_e1376 * s.dn[317][15]));
        let eq96_e1378_d_n16: f64 = ((eq96_e1376_d_n16 * s.v[317]) + (eq96_e1376 * s.dn[317][16]));
        let eq96_e1378_d_n17: f64 = ((eq96_e1376_d_n17 * s.v[317]) + (eq96_e1376 * s.dn[317][17]));
        let eq96_e1378_d_n18: f64 = ((eq96_e1376_d_n18 * s.v[317]) + (eq96_e1376 * s.dn[317][18]));
        let eq96_e1378_d_n19: f64 = ((eq96_e1376_d_n19 * s.v[317]) + (eq96_e1376 * s.dn[317][19]));
        let eq96_e1378_d_n20: f64 = ((eq96_e1376_d_n20 * s.v[317]) + (eq96_e1376 * s.dn[317][20]));
        let eq96_e1378_d_n21: f64 = ((eq96_e1376_d_n21 * s.v[317]) + (eq96_e1376 * s.dn[317][21]));
        let eq96_e1378_d_n22: f64 = ((eq96_e1376_d_n22 * s.v[317]) + (eq96_e1376 * s.dn[317][22]));
        let eq96_e1381: f64 = (p.p6 * s.v[379]);
        let eq96_e1381_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq96_e1381_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq96_e1381_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq96_e1381_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq96_e1381_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq96_e1381_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq96_e1381_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq96_e1381_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq96_e1381_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq96_e1381_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq96_e1381_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq96_e1381_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq96_e1381_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq96_e1381_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq96_e1381_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq96_e1381_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq96_e1381_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq96_e1381_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq96_e1381_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq96_e1381_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq96_e1381_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq96_e1381_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq96_e1381_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq96_e1383: f64 = (eq96_e1381 * (nv21 - nv22));
        let eq96_e1383_d_n0: f64 = (eq96_e1381_d_n0 * (nv21 - nv22));
        let eq96_e1383_d_n1: f64 = (eq96_e1381_d_n1 * (nv21 - nv22));
        let eq96_e1383_d_n2: f64 = (eq96_e1381_d_n2 * (nv21 - nv22));
        let eq96_e1383_d_n3: f64 = (eq96_e1381_d_n3 * (nv21 - nv22));
        let eq96_e1383_d_n4: f64 = (eq96_e1381_d_n4 * (nv21 - nv22));
        let eq96_e1383_d_n5: f64 = (eq96_e1381_d_n5 * (nv21 - nv22));
        let eq96_e1383_d_n6: f64 = (eq96_e1381_d_n6 * (nv21 - nv22));
        let eq96_e1383_d_n7: f64 = (eq96_e1381_d_n7 * (nv21 - nv22));
        let eq96_e1383_d_n8: f64 = (eq96_e1381_d_n8 * (nv21 - nv22));
        let eq96_e1383_d_n9: f64 = (eq96_e1381_d_n9 * (nv21 - nv22));
        let eq96_e1383_d_n10: f64 = (eq96_e1381_d_n10 * (nv21 - nv22));
        let eq96_e1383_d_n11: f64 = (eq96_e1381_d_n11 * (nv21 - nv22));
        let eq96_e1383_d_n12: f64 = (eq96_e1381_d_n12 * (nv21 - nv22));
        let eq96_e1383_d_n13: f64 = (eq96_e1381_d_n13 * (nv21 - nv22));
        let eq96_e1383_d_n14: f64 = (eq96_e1381_d_n14 * (nv21 - nv22));
        let eq96_e1383_d_n15: f64 = (eq96_e1381_d_n15 * (nv21 - nv22));
        let eq96_e1383_d_n16: f64 = (eq96_e1381_d_n16 * (nv21 - nv22));
        let eq96_e1383_d_n17: f64 = (eq96_e1381_d_n17 * (nv21 - nv22));
        let eq96_e1383_d_n18: f64 = (eq96_e1381_d_n18 * (nv21 - nv22));
        let eq96_e1383_d_n19: f64 = (eq96_e1381_d_n19 * (nv21 - nv22));
        let eq96_e1383_d_n20: f64 = (eq96_e1381_d_n20 * (nv21 - nv22));
        let eq96_e1383_d_n21: f64 = ((eq96_e1381_d_n21 * (nv21 - nv22)) + eq96_e1381);
        let eq96_e1383_d_n22: f64 = ((eq96_e1381_d_n22 * (nv21 - nv22)) + (-eq96_e1381));
        let eq96_e1384: f64 = (eq96_e1378 + eq96_e1383);
        let eq96_e1384_d_n0: f64 = (eq96_e1378_d_n0 + eq96_e1383_d_n0);
        let eq96_e1384_d_n1: f64 = (eq96_e1378_d_n1 + eq96_e1383_d_n1);
        let eq96_e1384_d_n2: f64 = (eq96_e1378_d_n2 + eq96_e1383_d_n2);
        let eq96_e1384_d_n3: f64 = (eq96_e1378_d_n3 + eq96_e1383_d_n3);
        let eq96_e1384_d_n4: f64 = (eq96_e1378_d_n4 + eq96_e1383_d_n4);
        let eq96_e1384_d_n5: f64 = (eq96_e1378_d_n5 + eq96_e1383_d_n5);
        let eq96_e1384_d_n6: f64 = (eq96_e1378_d_n6 + eq96_e1383_d_n6);
        let eq96_e1384_d_n7: f64 = (eq96_e1378_d_n7 + eq96_e1383_d_n7);
        let eq96_e1384_d_n8: f64 = (eq96_e1378_d_n8 + eq96_e1383_d_n8);
        let eq96_e1384_d_n9: f64 = (eq96_e1378_d_n9 + eq96_e1383_d_n9);
        let eq96_e1384_d_n10: f64 = (eq96_e1378_d_n10 + eq96_e1383_d_n10);
        let eq96_e1384_d_n11: f64 = (eq96_e1378_d_n11 + eq96_e1383_d_n11);
        let eq96_e1384_d_n12: f64 = (eq96_e1378_d_n12 + eq96_e1383_d_n12);
        let eq96_e1384_d_n13: f64 = (eq96_e1378_d_n13 + eq96_e1383_d_n13);
        let eq96_e1384_d_n14: f64 = (eq96_e1378_d_n14 + eq96_e1383_d_n14);
        let eq96_e1384_d_n15: f64 = (eq96_e1378_d_n15 + eq96_e1383_d_n15);
        let eq96_e1384_d_n16: f64 = (eq96_e1378_d_n16 + eq96_e1383_d_n16);
        let eq96_e1384_d_n17: f64 = (eq96_e1378_d_n17 + eq96_e1383_d_n17);
        let eq96_e1384_d_n18: f64 = (eq96_e1378_d_n18 + eq96_e1383_d_n18);
        let eq96_e1384_d_n19: f64 = (eq96_e1378_d_n19 + eq96_e1383_d_n19);
        let eq96_e1384_d_n20: f64 = (eq96_e1378_d_n20 + eq96_e1383_d_n20);
        let eq96_e1384_d_n21: f64 = (eq96_e1378_d_n21 + eq96_e1383_d_n21);
        let eq96_e1384_d_n22: f64 = (eq96_e1378_d_n22 + eq96_e1383_d_n22);
        (eq96_e1384, eq96_e1384_d_n0, eq96_e1384_d_n1, eq96_e1384_d_n2, eq96_e1384_d_n3, eq96_e1384_d_n4, eq96_e1384_d_n5, eq96_e1384_d_n6, eq96_e1384_d_n7, eq96_e1384_d_n8, eq96_e1384_d_n9, eq96_e1384_d_n10, eq96_e1384_d_n11, eq96_e1384_d_n12, eq96_e1384_d_n13, eq96_e1384_d_n14, eq96_e1384_d_n15, eq96_e1384_d_n16, eq96_e1384_d_n17, eq96_e1384_d_n18, eq96_e1384_d_n19, eq96_e1384_d_n20, eq96_e1384_d_n21, eq96_e1384_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq96_value: f64 = eq96_e1386;
        let eq96_node_derivatives: [f64; 23] = [eq96_e1386_d_n0, eq96_e1386_d_n1, eq96_e1386_d_n2, eq96_e1386_d_n3, eq96_e1386_d_n4, eq96_e1386_d_n5, eq96_e1386_d_n6, eq96_e1386_d_n7, eq96_e1386_d_n8, eq96_e1386_d_n9, eq96_e1386_d_n10, eq96_e1386_d_n11, eq96_e1386_d_n12, eq96_e1386_d_n13, eq96_e1386_d_n14, eq96_e1386_d_n15, eq96_e1386_d_n16, eq96_e1386_d_n17, eq96_e1386_d_n18, eq96_e1386_d_n19, eq96_e1386_d_n20, eq96_e1386_d_n21, eq96_e1386_d_n22];
        let eq96_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[21]),
            Some(nodes[22]),
            self.multiplicity * (eq96_value),
            &nodes,
            &eq96_node_derivatives,
            &branches,
            &eq96_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_97_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq97_e1393,) = {
    if ((s.v[538] != 0.0) && (!(s.v[539] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq97_value: f64 = eq97_e1393;
        stamper.stamp_potential(
            branches[50],
            eq97_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_98_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq98_e1398,) = {
    if (!(s.v[538] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq98_value: f64 = eq98_e1398;
        stamper.stamp_potential(
            branches[51],
            eq98_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_99_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq99_e1406,) = {
    if ((!(s.v[538] != 0.0)) && (!(s.v[547] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq99_value: f64 = eq99_e1406;
        stamper.stamp_potential(
            branches[52],
            eq99_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_100_block_0(
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
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq100_e1414, eq100_e1414_d_n0, eq100_e1414_d_n1, eq100_e1414_d_n2, eq100_e1414_d_n3, eq100_e1414_d_n4, eq100_e1414_d_n5, eq100_e1414_d_n6, eq100_e1414_d_n7, eq100_e1414_d_n8, eq100_e1414_d_n9, eq100_e1414_d_n10, eq100_e1414_d_n11, eq100_e1414_d_n12, eq100_e1414_d_n13, eq100_e1414_d_n14, eq100_e1414_d_n15, eq100_e1414_d_n16, eq100_e1414_d_n17, eq100_e1414_d_n18, eq100_e1414_d_n19, eq100_e1414_d_n20, eq100_e1414_d_n21, eq100_e1414_d_n22,) = {
    if (s.v[553] != 0.0) {
        let eq100_e1410: f64 = (p.p6 * s.v[318]);
        let eq100_e1410_d_n0: f64 = (p.p6 * s.dn[318][0]);
        let eq100_e1410_d_n1: f64 = (p.p6 * s.dn[318][1]);
        let eq100_e1410_d_n2: f64 = (p.p6 * s.dn[318][2]);
        let eq100_e1410_d_n3: f64 = (p.p6 * s.dn[318][3]);
        let eq100_e1410_d_n4: f64 = (p.p6 * s.dn[318][4]);
        let eq100_e1410_d_n5: f64 = (p.p6 * s.dn[318][5]);
        let eq100_e1410_d_n6: f64 = (p.p6 * s.dn[318][6]);
        let eq100_e1410_d_n7: f64 = (p.p6 * s.dn[318][7]);
        let eq100_e1410_d_n8: f64 = (p.p6 * s.dn[318][8]);
        let eq100_e1410_d_n9: f64 = (p.p6 * s.dn[318][9]);
        let eq100_e1410_d_n10: f64 = (p.p6 * s.dn[318][10]);
        let eq100_e1410_d_n11: f64 = (p.p6 * s.dn[318][11]);
        let eq100_e1410_d_n12: f64 = (p.p6 * s.dn[318][12]);
        let eq100_e1410_d_n13: f64 = (p.p6 * s.dn[318][13]);
        let eq100_e1410_d_n14: f64 = (p.p6 * s.dn[318][14]);
        let eq100_e1410_d_n15: f64 = (p.p6 * s.dn[318][15]);
        let eq100_e1410_d_n16: f64 = (p.p6 * s.dn[318][16]);
        let eq100_e1410_d_n17: f64 = (p.p6 * s.dn[318][17]);
        let eq100_e1410_d_n18: f64 = (p.p6 * s.dn[318][18]);
        let eq100_e1410_d_n19: f64 = (p.p6 * s.dn[318][19]);
        let eq100_e1410_d_n20: f64 = (p.p6 * s.dn[318][20]);
        let eq100_e1410_d_n21: f64 = (p.p6 * s.dn[318][21]);
        let eq100_e1410_d_n22: f64 = (p.p6 * s.dn[318][22]);
        let eq100_e1412: f64 = (eq100_e1410 * (nv1 - nv9));
        let eq100_e1412_d_n0: f64 = (eq100_e1410_d_n0 * (nv1 - nv9));
        let eq100_e1412_d_n1: f64 = ((eq100_e1410_d_n1 * (nv1 - nv9)) + eq100_e1410);
        let eq100_e1412_d_n2: f64 = (eq100_e1410_d_n2 * (nv1 - nv9));
        let eq100_e1412_d_n3: f64 = (eq100_e1410_d_n3 * (nv1 - nv9));
        let eq100_e1412_d_n4: f64 = (eq100_e1410_d_n4 * (nv1 - nv9));
        let eq100_e1412_d_n5: f64 = (eq100_e1410_d_n5 * (nv1 - nv9));
        let eq100_e1412_d_n6: f64 = (eq100_e1410_d_n6 * (nv1 - nv9));
        let eq100_e1412_d_n7: f64 = (eq100_e1410_d_n7 * (nv1 - nv9));
        let eq100_e1412_d_n8: f64 = (eq100_e1410_d_n8 * (nv1 - nv9));
        let eq100_e1412_d_n9: f64 = ((eq100_e1410_d_n9 * (nv1 - nv9)) + (-eq100_e1410));
        let eq100_e1412_d_n10: f64 = (eq100_e1410_d_n10 * (nv1 - nv9));
        let eq100_e1412_d_n11: f64 = (eq100_e1410_d_n11 * (nv1 - nv9));
        let eq100_e1412_d_n12: f64 = (eq100_e1410_d_n12 * (nv1 - nv9));
        let eq100_e1412_d_n13: f64 = (eq100_e1410_d_n13 * (nv1 - nv9));
        let eq100_e1412_d_n14: f64 = (eq100_e1410_d_n14 * (nv1 - nv9));
        let eq100_e1412_d_n15: f64 = (eq100_e1410_d_n15 * (nv1 - nv9));
        let eq100_e1412_d_n16: f64 = (eq100_e1410_d_n16 * (nv1 - nv9));
        let eq100_e1412_d_n17: f64 = (eq100_e1410_d_n17 * (nv1 - nv9));
        let eq100_e1412_d_n18: f64 = (eq100_e1410_d_n18 * (nv1 - nv9));
        let eq100_e1412_d_n19: f64 = (eq100_e1410_d_n19 * (nv1 - nv9));
        let eq100_e1412_d_n20: f64 = (eq100_e1410_d_n20 * (nv1 - nv9));
        let eq100_e1412_d_n21: f64 = (eq100_e1410_d_n21 * (nv1 - nv9));
        let eq100_e1412_d_n22: f64 = (eq100_e1410_d_n22 * (nv1 - nv9));
        (eq100_e1412, eq100_e1412_d_n0, eq100_e1412_d_n1, eq100_e1412_d_n2, eq100_e1412_d_n3, eq100_e1412_d_n4, eq100_e1412_d_n5, eq100_e1412_d_n6, eq100_e1412_d_n7, eq100_e1412_d_n8, eq100_e1412_d_n9, eq100_e1412_d_n10, eq100_e1412_d_n11, eq100_e1412_d_n12, eq100_e1412_d_n13, eq100_e1412_d_n14, eq100_e1412_d_n15, eq100_e1412_d_n16, eq100_e1412_d_n17, eq100_e1412_d_n18, eq100_e1412_d_n19, eq100_e1412_d_n20, eq100_e1412_d_n21, eq100_e1412_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq100_value: f64 = eq100_e1414;
        let eq100_node_derivatives: [f64; 23] = [eq100_e1414_d_n0, eq100_e1414_d_n1, eq100_e1414_d_n2, eq100_e1414_d_n3, eq100_e1414_d_n4, eq100_e1414_d_n5, eq100_e1414_d_n6, eq100_e1414_d_n7, eq100_e1414_d_n8, eq100_e1414_d_n9, eq100_e1414_d_n10, eq100_e1414_d_n11, eq100_e1414_d_n12, eq100_e1414_d_n13, eq100_e1414_d_n14, eq100_e1414_d_n15, eq100_e1414_d_n16, eq100_e1414_d_n17, eq100_e1414_d_n18, eq100_e1414_d_n19, eq100_e1414_d_n20, eq100_e1414_d_n21, eq100_e1414_d_n22];
        let eq100_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[9]),
            self.multiplicity * (eq100_value),
            &nodes,
            &eq100_node_derivatives,
            &branches,
            &eq100_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_101_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq101_e1418,) = {
    if (s.v[553] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq101_value: f64 = eq101_e1418;
        stamper.stamp_potential(
            branches[53],
            eq101_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_102_block_0(
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq102_e1429, eq102_e1429_d_n0, eq102_e1429_d_n1, eq102_e1429_d_n2, eq102_e1429_d_n3, eq102_e1429_d_n4, eq102_e1429_d_n5, eq102_e1429_d_n6, eq102_e1429_d_n7, eq102_e1429_d_n8, eq102_e1429_d_n9, eq102_e1429_d_n10, eq102_e1429_d_n11, eq102_e1429_d_n12, eq102_e1429_d_n13, eq102_e1429_d_n14, eq102_e1429_d_n15, eq102_e1429_d_n16, eq102_e1429_d_n17, eq102_e1429_d_n18, eq102_e1429_d_n19, eq102_e1429_d_n20, eq102_e1429_d_n21, eq102_e1429_d_n22,) = {
    if ((!(s.v[553] != 0.0)) && (s.v[555] != 0.0)) {
        let eq102_e1425: f64 = (p.p6 * s.v[319]);
        let eq102_e1425_d_n0: f64 = (p.p6 * s.dn[319][0]);
        let eq102_e1425_d_n1: f64 = (p.p6 * s.dn[319][1]);
        let eq102_e1425_d_n2: f64 = (p.p6 * s.dn[319][2]);
        let eq102_e1425_d_n3: f64 = (p.p6 * s.dn[319][3]);
        let eq102_e1425_d_n4: f64 = (p.p6 * s.dn[319][4]);
        let eq102_e1425_d_n5: f64 = (p.p6 * s.dn[319][5]);
        let eq102_e1425_d_n6: f64 = (p.p6 * s.dn[319][6]);
        let eq102_e1425_d_n7: f64 = (p.p6 * s.dn[319][7]);
        let eq102_e1425_d_n8: f64 = (p.p6 * s.dn[319][8]);
        let eq102_e1425_d_n9: f64 = (p.p6 * s.dn[319][9]);
        let eq102_e1425_d_n10: f64 = (p.p6 * s.dn[319][10]);
        let eq102_e1425_d_n11: f64 = (p.p6 * s.dn[319][11]);
        let eq102_e1425_d_n12: f64 = (p.p6 * s.dn[319][12]);
        let eq102_e1425_d_n13: f64 = (p.p6 * s.dn[319][13]);
        let eq102_e1425_d_n14: f64 = (p.p6 * s.dn[319][14]);
        let eq102_e1425_d_n15: f64 = (p.p6 * s.dn[319][15]);
        let eq102_e1425_d_n16: f64 = (p.p6 * s.dn[319][16]);
        let eq102_e1425_d_n17: f64 = (p.p6 * s.dn[319][17]);
        let eq102_e1425_d_n18: f64 = (p.p6 * s.dn[319][18]);
        let eq102_e1425_d_n19: f64 = (p.p6 * s.dn[319][19]);
        let eq102_e1425_d_n20: f64 = (p.p6 * s.dn[319][20]);
        let eq102_e1425_d_n21: f64 = (p.p6 * s.dn[319][21]);
        let eq102_e1425_d_n22: f64 = (p.p6 * s.dn[319][22]);
        let eq102_e1427: f64 = (eq102_e1425 * (nv1 - nv10));
        let eq102_e1427_d_n0: f64 = (eq102_e1425_d_n0 * (nv1 - nv10));
        let eq102_e1427_d_n1: f64 = ((eq102_e1425_d_n1 * (nv1 - nv10)) + eq102_e1425);
        let eq102_e1427_d_n2: f64 = (eq102_e1425_d_n2 * (nv1 - nv10));
        let eq102_e1427_d_n3: f64 = (eq102_e1425_d_n3 * (nv1 - nv10));
        let eq102_e1427_d_n4: f64 = (eq102_e1425_d_n4 * (nv1 - nv10));
        let eq102_e1427_d_n5: f64 = (eq102_e1425_d_n5 * (nv1 - nv10));
        let eq102_e1427_d_n6: f64 = (eq102_e1425_d_n6 * (nv1 - nv10));
        let eq102_e1427_d_n7: f64 = (eq102_e1425_d_n7 * (nv1 - nv10));
        let eq102_e1427_d_n8: f64 = (eq102_e1425_d_n8 * (nv1 - nv10));
        let eq102_e1427_d_n9: f64 = (eq102_e1425_d_n9 * (nv1 - nv10));
        let eq102_e1427_d_n10: f64 = ((eq102_e1425_d_n10 * (nv1 - nv10)) + (-eq102_e1425));
        let eq102_e1427_d_n11: f64 = (eq102_e1425_d_n11 * (nv1 - nv10));
        let eq102_e1427_d_n12: f64 = (eq102_e1425_d_n12 * (nv1 - nv10));
        let eq102_e1427_d_n13: f64 = (eq102_e1425_d_n13 * (nv1 - nv10));
        let eq102_e1427_d_n14: f64 = (eq102_e1425_d_n14 * (nv1 - nv10));
        let eq102_e1427_d_n15: f64 = (eq102_e1425_d_n15 * (nv1 - nv10));
        let eq102_e1427_d_n16: f64 = (eq102_e1425_d_n16 * (nv1 - nv10));
        let eq102_e1427_d_n17: f64 = (eq102_e1425_d_n17 * (nv1 - nv10));
        let eq102_e1427_d_n18: f64 = (eq102_e1425_d_n18 * (nv1 - nv10));
        let eq102_e1427_d_n19: f64 = (eq102_e1425_d_n19 * (nv1 - nv10));
        let eq102_e1427_d_n20: f64 = (eq102_e1425_d_n20 * (nv1 - nv10));
        let eq102_e1427_d_n21: f64 = (eq102_e1425_d_n21 * (nv1 - nv10));
        let eq102_e1427_d_n22: f64 = (eq102_e1425_d_n22 * (nv1 - nv10));
        (eq102_e1427, eq102_e1427_d_n0, eq102_e1427_d_n1, eq102_e1427_d_n2, eq102_e1427_d_n3, eq102_e1427_d_n4, eq102_e1427_d_n5, eq102_e1427_d_n6, eq102_e1427_d_n7, eq102_e1427_d_n8, eq102_e1427_d_n9, eq102_e1427_d_n10, eq102_e1427_d_n11, eq102_e1427_d_n12, eq102_e1427_d_n13, eq102_e1427_d_n14, eq102_e1427_d_n15, eq102_e1427_d_n16, eq102_e1427_d_n17, eq102_e1427_d_n18, eq102_e1427_d_n19, eq102_e1427_d_n20, eq102_e1427_d_n21, eq102_e1427_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq102_value: f64 = eq102_e1429;
        let eq102_node_derivatives: [f64; 23] = [eq102_e1429_d_n0, eq102_e1429_d_n1, eq102_e1429_d_n2, eq102_e1429_d_n3, eq102_e1429_d_n4, eq102_e1429_d_n5, eq102_e1429_d_n6, eq102_e1429_d_n7, eq102_e1429_d_n8, eq102_e1429_d_n9, eq102_e1429_d_n10, eq102_e1429_d_n11, eq102_e1429_d_n12, eq102_e1429_d_n13, eq102_e1429_d_n14, eq102_e1429_d_n15, eq102_e1429_d_n16, eq102_e1429_d_n17, eq102_e1429_d_n18, eq102_e1429_d_n19, eq102_e1429_d_n20, eq102_e1429_d_n21, eq102_e1429_d_n22];
        let eq102_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[10]),
            self.multiplicity * (eq102_value),
            &nodes,
            &eq102_node_derivatives,
            &branches,
            &eq102_branch_derivatives,
            self.multiplicity,
        );
    }
}
