#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_42_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq42_e1310: f64 = (s.v[0] * s.v[15]);
        let eq42_e1310_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq42_e1310_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq42_e1310_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq42_e1310_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq42_e1310_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq42_e1310_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq42_e1310_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq42_e1310_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq42_e1310_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq42_e1310_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq42_e1310_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq42_e1310_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq42_e1310_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq42_e1312: f64 = (eq42_e1310 * p.p33);
        let eq42_e1312_d_n0: f64 = (eq42_e1310_d_n0 * p.p33);
        let eq42_e1312_d_n1: f64 = (eq42_e1310_d_n1 * p.p33);
        let eq42_e1312_d_n2: f64 = (eq42_e1310_d_n2 * p.p33);
        let eq42_e1312_d_n3: f64 = (eq42_e1310_d_n3 * p.p33);
        let eq42_e1312_d_n4: f64 = (eq42_e1310_d_n4 * p.p33);
        let eq42_e1312_d_n5: f64 = (eq42_e1310_d_n5 * p.p33);
        let eq42_e1312_d_n6: f64 = (eq42_e1310_d_n6 * p.p33);
        let eq42_e1312_d_n7: f64 = (eq42_e1310_d_n7 * p.p33);
        let eq42_e1312_d_n8: f64 = (eq42_e1310_d_n8 * p.p33);
        let eq42_e1312_d_n9: f64 = (eq42_e1310_d_n9 * p.p33);
        let eq42_e1312_d_n10: f64 = (eq42_e1310_d_n10 * p.p33);
        let eq42_e1312_d_n11: f64 = (eq42_e1310_d_n11 * p.p33);
        let eq42_e1312_d_n12: f64 = (eq42_e1310_d_n12 * p.p33);
        let eq42_e1314: f64 = (eq42_e1312 * s.v[841]);
        let eq42_e1314_d_n0: f64 = ((eq42_e1312_d_n0 * s.v[841]) + (eq42_e1312 * s.dn[841][0]));
        let eq42_e1314_d_n1: f64 = ((eq42_e1312_d_n1 * s.v[841]) + (eq42_e1312 * s.dn[841][1]));
        let eq42_e1314_d_n2: f64 = ((eq42_e1312_d_n2 * s.v[841]) + (eq42_e1312 * s.dn[841][2]));
        let eq42_e1314_d_n3: f64 = ((eq42_e1312_d_n3 * s.v[841]) + (eq42_e1312 * s.dn[841][3]));
        let eq42_e1314_d_n4: f64 = ((eq42_e1312_d_n4 * s.v[841]) + (eq42_e1312 * s.dn[841][4]));
        let eq42_e1314_d_n5: f64 = ((eq42_e1312_d_n5 * s.v[841]) + (eq42_e1312 * s.dn[841][5]));
        let eq42_e1314_d_n6: f64 = ((eq42_e1312_d_n6 * s.v[841]) + (eq42_e1312 * s.dn[841][6]));
        let eq42_e1314_d_n7: f64 = ((eq42_e1312_d_n7 * s.v[841]) + (eq42_e1312 * s.dn[841][7]));
        let eq42_e1314_d_n8: f64 = ((eq42_e1312_d_n8 * s.v[841]) + (eq42_e1312 * s.dn[841][8]));
        let eq42_e1314_d_n9: f64 = ((eq42_e1312_d_n9 * s.v[841]) + (eq42_e1312 * s.dn[841][9]));
        let eq42_e1314_d_n10: f64 = ((eq42_e1312_d_n10 * s.v[841]) + (eq42_e1312 * s.dn[841][10]));
        let eq42_e1314_d_n11: f64 = ((eq42_e1312_d_n11 * s.v[841]) + (eq42_e1312 * s.dn[841][11]));
        let eq42_e1314_d_n12: f64 = ((eq42_e1312_d_n12 * s.v[841]) + (eq42_e1312 * s.dn[841][12]));
        let eq42_e1315_q: f64 = eq42_e1314;
        let eq42_reactive_node_derivatives: [f64; 13] = [eq42_e1314_d_n0, eq42_e1314_d_n1, eq42_e1314_d_n2, eq42_e1314_d_n3, eq42_e1314_d_n4, eq42_e1314_d_n5, eq42_e1314_d_n6, eq42_e1314_d_n7, eq42_e1314_d_n8, eq42_e1314_d_n9, eq42_e1314_d_n10, eq42_e1314_d_n11, eq42_e1314_d_n12];
        let eq42_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            &nodes,
            &eq42_reactive_node_derivatives,
            &branches,
            &eq42_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_43_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq43_e1318: f64 = (s.v[0] * s.v[15]);
        let eq43_e1318_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq43_e1318_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq43_e1318_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq43_e1318_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq43_e1318_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq43_e1318_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq43_e1318_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq43_e1318_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq43_e1318_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq43_e1318_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq43_e1318_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq43_e1318_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq43_e1318_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq43_e1320: f64 = (eq43_e1318 * p.p33);
        let eq43_e1320_d_n0: f64 = (eq43_e1318_d_n0 * p.p33);
        let eq43_e1320_d_n1: f64 = (eq43_e1318_d_n1 * p.p33);
        let eq43_e1320_d_n2: f64 = (eq43_e1318_d_n2 * p.p33);
        let eq43_e1320_d_n3: f64 = (eq43_e1318_d_n3 * p.p33);
        let eq43_e1320_d_n4: f64 = (eq43_e1318_d_n4 * p.p33);
        let eq43_e1320_d_n5: f64 = (eq43_e1318_d_n5 * p.p33);
        let eq43_e1320_d_n6: f64 = (eq43_e1318_d_n6 * p.p33);
        let eq43_e1320_d_n7: f64 = (eq43_e1318_d_n7 * p.p33);
        let eq43_e1320_d_n8: f64 = (eq43_e1318_d_n8 * p.p33);
        let eq43_e1320_d_n9: f64 = (eq43_e1318_d_n9 * p.p33);
        let eq43_e1320_d_n10: f64 = (eq43_e1318_d_n10 * p.p33);
        let eq43_e1320_d_n11: f64 = (eq43_e1318_d_n11 * p.p33);
        let eq43_e1320_d_n12: f64 = (eq43_e1318_d_n12 * p.p33);
        let eq43_e1322: f64 = (eq43_e1320 * s.v[842]);
        let eq43_e1322_d_n0: f64 = ((eq43_e1320_d_n0 * s.v[842]) + (eq43_e1320 * s.dn[842][0]));
        let eq43_e1322_d_n1: f64 = ((eq43_e1320_d_n1 * s.v[842]) + (eq43_e1320 * s.dn[842][1]));
        let eq43_e1322_d_n2: f64 = ((eq43_e1320_d_n2 * s.v[842]) + (eq43_e1320 * s.dn[842][2]));
        let eq43_e1322_d_n3: f64 = ((eq43_e1320_d_n3 * s.v[842]) + (eq43_e1320 * s.dn[842][3]));
        let eq43_e1322_d_n4: f64 = ((eq43_e1320_d_n4 * s.v[842]) + (eq43_e1320 * s.dn[842][4]));
        let eq43_e1322_d_n5: f64 = ((eq43_e1320_d_n5 * s.v[842]) + (eq43_e1320 * s.dn[842][5]));
        let eq43_e1322_d_n6: f64 = ((eq43_e1320_d_n6 * s.v[842]) + (eq43_e1320 * s.dn[842][6]));
        let eq43_e1322_d_n7: f64 = ((eq43_e1320_d_n7 * s.v[842]) + (eq43_e1320 * s.dn[842][7]));
        let eq43_e1322_d_n8: f64 = ((eq43_e1320_d_n8 * s.v[842]) + (eq43_e1320 * s.dn[842][8]));
        let eq43_e1322_d_n9: f64 = ((eq43_e1320_d_n9 * s.v[842]) + (eq43_e1320 * s.dn[842][9]));
        let eq43_e1322_d_n10: f64 = ((eq43_e1320_d_n10 * s.v[842]) + (eq43_e1320 * s.dn[842][10]));
        let eq43_e1322_d_n11: f64 = ((eq43_e1320_d_n11 * s.v[842]) + (eq43_e1320 * s.dn[842][11]));
        let eq43_e1322_d_n12: f64 = ((eq43_e1320_d_n12 * s.v[842]) + (eq43_e1320 * s.dn[842][12]));
        let eq43_e1323_q: f64 = eq43_e1322;
        let eq43_reactive_node_derivatives: [f64; 13] = [eq43_e1322_d_n0, eq43_e1322_d_n1, eq43_e1322_d_n2, eq43_e1322_d_n3, eq43_e1322_d_n4, eq43_e1322_d_n5, eq43_e1322_d_n6, eq43_e1322_d_n7, eq43_e1322_d_n8, eq43_e1322_d_n9, eq43_e1322_d_n10, eq43_e1322_d_n11, eq43_e1322_d_n12];
        let eq43_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            &nodes,
            &eq43_reactive_node_derivatives,
            &branches,
            &eq43_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_44_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq44_e1326: f64 = (s.v[0] * s.v[15]);
        let eq44_e1326_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq44_e1326_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq44_e1326_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq44_e1326_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq44_e1326_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq44_e1326_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq44_e1326_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq44_e1326_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq44_e1326_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq44_e1326_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq44_e1326_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq44_e1326_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq44_e1326_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq44_e1328: f64 = (eq44_e1326 * p.p33);
        let eq44_e1328_d_n0: f64 = (eq44_e1326_d_n0 * p.p33);
        let eq44_e1328_d_n1: f64 = (eq44_e1326_d_n1 * p.p33);
        let eq44_e1328_d_n2: f64 = (eq44_e1326_d_n2 * p.p33);
        let eq44_e1328_d_n3: f64 = (eq44_e1326_d_n3 * p.p33);
        let eq44_e1328_d_n4: f64 = (eq44_e1326_d_n4 * p.p33);
        let eq44_e1328_d_n5: f64 = (eq44_e1326_d_n5 * p.p33);
        let eq44_e1328_d_n6: f64 = (eq44_e1326_d_n6 * p.p33);
        let eq44_e1328_d_n7: f64 = (eq44_e1326_d_n7 * p.p33);
        let eq44_e1328_d_n8: f64 = (eq44_e1326_d_n8 * p.p33);
        let eq44_e1328_d_n9: f64 = (eq44_e1326_d_n9 * p.p33);
        let eq44_e1328_d_n10: f64 = (eq44_e1326_d_n10 * p.p33);
        let eq44_e1328_d_n11: f64 = (eq44_e1326_d_n11 * p.p33);
        let eq44_e1328_d_n12: f64 = (eq44_e1326_d_n12 * p.p33);
        let eq44_e1330: f64 = (eq44_e1328 * s.v[843]);
        let eq44_e1330_d_n0: f64 = ((eq44_e1328_d_n0 * s.v[843]) + (eq44_e1328 * s.dn[843][0]));
        let eq44_e1330_d_n1: f64 = ((eq44_e1328_d_n1 * s.v[843]) + (eq44_e1328 * s.dn[843][1]));
        let eq44_e1330_d_n2: f64 = ((eq44_e1328_d_n2 * s.v[843]) + (eq44_e1328 * s.dn[843][2]));
        let eq44_e1330_d_n3: f64 = ((eq44_e1328_d_n3 * s.v[843]) + (eq44_e1328 * s.dn[843][3]));
        let eq44_e1330_d_n4: f64 = ((eq44_e1328_d_n4 * s.v[843]) + (eq44_e1328 * s.dn[843][4]));
        let eq44_e1330_d_n5: f64 = ((eq44_e1328_d_n5 * s.v[843]) + (eq44_e1328 * s.dn[843][5]));
        let eq44_e1330_d_n6: f64 = ((eq44_e1328_d_n6 * s.v[843]) + (eq44_e1328 * s.dn[843][6]));
        let eq44_e1330_d_n7: f64 = ((eq44_e1328_d_n7 * s.v[843]) + (eq44_e1328 * s.dn[843][7]));
        let eq44_e1330_d_n8: f64 = ((eq44_e1328_d_n8 * s.v[843]) + (eq44_e1328 * s.dn[843][8]));
        let eq44_e1330_d_n9: f64 = ((eq44_e1328_d_n9 * s.v[843]) + (eq44_e1328 * s.dn[843][9]));
        let eq44_e1330_d_n10: f64 = ((eq44_e1328_d_n10 * s.v[843]) + (eq44_e1328 * s.dn[843][10]));
        let eq44_e1330_d_n11: f64 = ((eq44_e1328_d_n11 * s.v[843]) + (eq44_e1328 * s.dn[843][11]));
        let eq44_e1330_d_n12: f64 = ((eq44_e1328_d_n12 * s.v[843]) + (eq44_e1328 * s.dn[843][12]));
        let eq44_e1331_q: f64 = eq44_e1330;
        let eq44_reactive_node_derivatives: [f64; 13] = [eq44_e1330_d_n0, eq44_e1330_d_n1, eq44_e1330_d_n2, eq44_e1330_d_n3, eq44_e1330_d_n4, eq44_e1330_d_n5, eq44_e1330_d_n6, eq44_e1330_d_n7, eq44_e1330_d_n8, eq44_e1330_d_n9, eq44_e1330_d_n10, eq44_e1330_d_n11, eq44_e1330_d_n12];
        let eq44_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            &nodes,
            &eq44_reactive_node_derivatives,
            &branches,
            &eq44_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_45_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq45_e1334: f64 = (s.v[0] * s.v[15]);
        let eq45_e1334_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq45_e1334_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq45_e1334_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq45_e1334_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq45_e1334_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq45_e1334_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq45_e1334_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq45_e1334_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq45_e1334_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq45_e1334_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq45_e1334_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq45_e1334_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq45_e1334_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq45_e1336: f64 = (eq45_e1334 * p.p33);
        let eq45_e1336_d_n0: f64 = (eq45_e1334_d_n0 * p.p33);
        let eq45_e1336_d_n1: f64 = (eq45_e1334_d_n1 * p.p33);
        let eq45_e1336_d_n2: f64 = (eq45_e1334_d_n2 * p.p33);
        let eq45_e1336_d_n3: f64 = (eq45_e1334_d_n3 * p.p33);
        let eq45_e1336_d_n4: f64 = (eq45_e1334_d_n4 * p.p33);
        let eq45_e1336_d_n5: f64 = (eq45_e1334_d_n5 * p.p33);
        let eq45_e1336_d_n6: f64 = (eq45_e1334_d_n6 * p.p33);
        let eq45_e1336_d_n7: f64 = (eq45_e1334_d_n7 * p.p33);
        let eq45_e1336_d_n8: f64 = (eq45_e1334_d_n8 * p.p33);
        let eq45_e1336_d_n9: f64 = (eq45_e1334_d_n9 * p.p33);
        let eq45_e1336_d_n10: f64 = (eq45_e1334_d_n10 * p.p33);
        let eq45_e1336_d_n11: f64 = (eq45_e1334_d_n11 * p.p33);
        let eq45_e1336_d_n12: f64 = (eq45_e1334_d_n12 * p.p33);
        let eq45_e1338: f64 = (eq45_e1336 * s.v[844]);
        let eq45_e1338_d_n0: f64 = ((eq45_e1336_d_n0 * s.v[844]) + (eq45_e1336 * s.dn[844][0]));
        let eq45_e1338_d_n1: f64 = ((eq45_e1336_d_n1 * s.v[844]) + (eq45_e1336 * s.dn[844][1]));
        let eq45_e1338_d_n2: f64 = ((eq45_e1336_d_n2 * s.v[844]) + (eq45_e1336 * s.dn[844][2]));
        let eq45_e1338_d_n3: f64 = ((eq45_e1336_d_n3 * s.v[844]) + (eq45_e1336 * s.dn[844][3]));
        let eq45_e1338_d_n4: f64 = ((eq45_e1336_d_n4 * s.v[844]) + (eq45_e1336 * s.dn[844][4]));
        let eq45_e1338_d_n5: f64 = ((eq45_e1336_d_n5 * s.v[844]) + (eq45_e1336 * s.dn[844][5]));
        let eq45_e1338_d_n6: f64 = ((eq45_e1336_d_n6 * s.v[844]) + (eq45_e1336 * s.dn[844][6]));
        let eq45_e1338_d_n7: f64 = ((eq45_e1336_d_n7 * s.v[844]) + (eq45_e1336 * s.dn[844][7]));
        let eq45_e1338_d_n8: f64 = ((eq45_e1336_d_n8 * s.v[844]) + (eq45_e1336 * s.dn[844][8]));
        let eq45_e1338_d_n9: f64 = ((eq45_e1336_d_n9 * s.v[844]) + (eq45_e1336 * s.dn[844][9]));
        let eq45_e1338_d_n10: f64 = ((eq45_e1336_d_n10 * s.v[844]) + (eq45_e1336 * s.dn[844][10]));
        let eq45_e1338_d_n11: f64 = ((eq45_e1336_d_n11 * s.v[844]) + (eq45_e1336 * s.dn[844][11]));
        let eq45_e1338_d_n12: f64 = ((eq45_e1336_d_n12 * s.v[844]) + (eq45_e1336 * s.dn[844][12]));
        let eq45_e1339_q: f64 = eq45_e1338;
        let eq45_reactive_node_derivatives: [f64; 13] = [eq45_e1338_d_n0, eq45_e1338_d_n1, eq45_e1338_d_n2, eq45_e1338_d_n3, eq45_e1338_d_n4, eq45_e1338_d_n5, eq45_e1338_d_n6, eq45_e1338_d_n7, eq45_e1338_d_n8, eq45_e1338_d_n9, eq45_e1338_d_n10, eq45_e1338_d_n11, eq45_e1338_d_n12];
        let eq45_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            &nodes,
            &eq45_reactive_node_derivatives,
            &branches,
            &eq45_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_46_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq46_e1342: f64 = (s.v[0] * s.v[15]);
        let eq46_e1342_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq46_e1342_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq46_e1342_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq46_e1342_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq46_e1342_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq46_e1342_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq46_e1342_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq46_e1342_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq46_e1342_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq46_e1342_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq46_e1342_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq46_e1342_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq46_e1342_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq46_e1344: f64 = (eq46_e1342 * p.p33);
        let eq46_e1344_d_n0: f64 = (eq46_e1342_d_n0 * p.p33);
        let eq46_e1344_d_n1: f64 = (eq46_e1342_d_n1 * p.p33);
        let eq46_e1344_d_n2: f64 = (eq46_e1342_d_n2 * p.p33);
        let eq46_e1344_d_n3: f64 = (eq46_e1342_d_n3 * p.p33);
        let eq46_e1344_d_n4: f64 = (eq46_e1342_d_n4 * p.p33);
        let eq46_e1344_d_n5: f64 = (eq46_e1342_d_n5 * p.p33);
        let eq46_e1344_d_n6: f64 = (eq46_e1342_d_n6 * p.p33);
        let eq46_e1344_d_n7: f64 = (eq46_e1342_d_n7 * p.p33);
        let eq46_e1344_d_n8: f64 = (eq46_e1342_d_n8 * p.p33);
        let eq46_e1344_d_n9: f64 = (eq46_e1342_d_n9 * p.p33);
        let eq46_e1344_d_n10: f64 = (eq46_e1342_d_n10 * p.p33);
        let eq46_e1344_d_n11: f64 = (eq46_e1342_d_n11 * p.p33);
        let eq46_e1344_d_n12: f64 = (eq46_e1342_d_n12 * p.p33);
        let eq46_e1346: f64 = (eq46_e1344 * s.v[845]);
        let eq46_e1346_d_n0: f64 = ((eq46_e1344_d_n0 * s.v[845]) + (eq46_e1344 * s.dn[845][0]));
        let eq46_e1346_d_n1: f64 = ((eq46_e1344_d_n1 * s.v[845]) + (eq46_e1344 * s.dn[845][1]));
        let eq46_e1346_d_n2: f64 = ((eq46_e1344_d_n2 * s.v[845]) + (eq46_e1344 * s.dn[845][2]));
        let eq46_e1346_d_n3: f64 = ((eq46_e1344_d_n3 * s.v[845]) + (eq46_e1344 * s.dn[845][3]));
        let eq46_e1346_d_n4: f64 = ((eq46_e1344_d_n4 * s.v[845]) + (eq46_e1344 * s.dn[845][4]));
        let eq46_e1346_d_n5: f64 = ((eq46_e1344_d_n5 * s.v[845]) + (eq46_e1344 * s.dn[845][5]));
        let eq46_e1346_d_n6: f64 = ((eq46_e1344_d_n6 * s.v[845]) + (eq46_e1344 * s.dn[845][6]));
        let eq46_e1346_d_n7: f64 = ((eq46_e1344_d_n7 * s.v[845]) + (eq46_e1344 * s.dn[845][7]));
        let eq46_e1346_d_n8: f64 = ((eq46_e1344_d_n8 * s.v[845]) + (eq46_e1344 * s.dn[845][8]));
        let eq46_e1346_d_n9: f64 = ((eq46_e1344_d_n9 * s.v[845]) + (eq46_e1344 * s.dn[845][9]));
        let eq46_e1346_d_n10: f64 = ((eq46_e1344_d_n10 * s.v[845]) + (eq46_e1344 * s.dn[845][10]));
        let eq46_e1346_d_n11: f64 = ((eq46_e1344_d_n11 * s.v[845]) + (eq46_e1344 * s.dn[845][11]));
        let eq46_e1346_d_n12: f64 = ((eq46_e1344_d_n12 * s.v[845]) + (eq46_e1344 * s.dn[845][12]));
        let eq46_e1347_q: f64 = eq46_e1346;
        let eq46_reactive_node_derivatives: [f64; 13] = [eq46_e1346_d_n0, eq46_e1346_d_n1, eq46_e1346_d_n2, eq46_e1346_d_n3, eq46_e1346_d_n4, eq46_e1346_d_n5, eq46_e1346_d_n6, eq46_e1346_d_n7, eq46_e1346_d_n8, eq46_e1346_d_n9, eq46_e1346_d_n10, eq46_e1346_d_n11, eq46_e1346_d_n12];
        let eq46_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[9]),
            &nodes,
            &eq46_reactive_node_derivatives,
            &branches,
            &eq46_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_47_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq47_e1350: f64 = (s.v[0] * s.v[15]);
        let eq47_e1350_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq47_e1350_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq47_e1350_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq47_e1350_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq47_e1350_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq47_e1350_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq47_e1350_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq47_e1350_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq47_e1350_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq47_e1350_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq47_e1350_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq47_e1350_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq47_e1350_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq47_e1352: f64 = (eq47_e1350 * p.p33);
        let eq47_e1352_d_n0: f64 = (eq47_e1350_d_n0 * p.p33);
        let eq47_e1352_d_n1: f64 = (eq47_e1350_d_n1 * p.p33);
        let eq47_e1352_d_n2: f64 = (eq47_e1350_d_n2 * p.p33);
        let eq47_e1352_d_n3: f64 = (eq47_e1350_d_n3 * p.p33);
        let eq47_e1352_d_n4: f64 = (eq47_e1350_d_n4 * p.p33);
        let eq47_e1352_d_n5: f64 = (eq47_e1350_d_n5 * p.p33);
        let eq47_e1352_d_n6: f64 = (eq47_e1350_d_n6 * p.p33);
        let eq47_e1352_d_n7: f64 = (eq47_e1350_d_n7 * p.p33);
        let eq47_e1352_d_n8: f64 = (eq47_e1350_d_n8 * p.p33);
        let eq47_e1352_d_n9: f64 = (eq47_e1350_d_n9 * p.p33);
        let eq47_e1352_d_n10: f64 = (eq47_e1350_d_n10 * p.p33);
        let eq47_e1352_d_n11: f64 = (eq47_e1350_d_n11 * p.p33);
        let eq47_e1352_d_n12: f64 = (eq47_e1350_d_n12 * p.p33);
        let eq47_e1354: f64 = (eq47_e1352 * s.v[846]);
        let eq47_e1354_d_n0: f64 = ((eq47_e1352_d_n0 * s.v[846]) + (eq47_e1352 * s.dn[846][0]));
        let eq47_e1354_d_n1: f64 = ((eq47_e1352_d_n1 * s.v[846]) + (eq47_e1352 * s.dn[846][1]));
        let eq47_e1354_d_n2: f64 = ((eq47_e1352_d_n2 * s.v[846]) + (eq47_e1352 * s.dn[846][2]));
        let eq47_e1354_d_n3: f64 = ((eq47_e1352_d_n3 * s.v[846]) + (eq47_e1352 * s.dn[846][3]));
        let eq47_e1354_d_n4: f64 = ((eq47_e1352_d_n4 * s.v[846]) + (eq47_e1352 * s.dn[846][4]));
        let eq47_e1354_d_n5: f64 = ((eq47_e1352_d_n5 * s.v[846]) + (eq47_e1352 * s.dn[846][5]));
        let eq47_e1354_d_n6: f64 = ((eq47_e1352_d_n6 * s.v[846]) + (eq47_e1352 * s.dn[846][6]));
        let eq47_e1354_d_n7: f64 = ((eq47_e1352_d_n7 * s.v[846]) + (eq47_e1352 * s.dn[846][7]));
        let eq47_e1354_d_n8: f64 = ((eq47_e1352_d_n8 * s.v[846]) + (eq47_e1352 * s.dn[846][8]));
        let eq47_e1354_d_n9: f64 = ((eq47_e1352_d_n9 * s.v[846]) + (eq47_e1352 * s.dn[846][9]));
        let eq47_e1354_d_n10: f64 = ((eq47_e1352_d_n10 * s.v[846]) + (eq47_e1352 * s.dn[846][10]));
        let eq47_e1354_d_n11: f64 = ((eq47_e1352_d_n11 * s.v[846]) + (eq47_e1352 * s.dn[846][11]));
        let eq47_e1354_d_n12: f64 = ((eq47_e1352_d_n12 * s.v[846]) + (eq47_e1352 * s.dn[846][12]));
        let eq47_e1355_q: f64 = eq47_e1354;
        let eq47_reactive_node_derivatives: [f64; 13] = [eq47_e1354_d_n0, eq47_e1354_d_n1, eq47_e1354_d_n2, eq47_e1354_d_n3, eq47_e1354_d_n4, eq47_e1354_d_n5, eq47_e1354_d_n6, eq47_e1354_d_n7, eq47_e1354_d_n8, eq47_e1354_d_n9, eq47_e1354_d_n10, eq47_e1354_d_n11, eq47_e1354_d_n12];
        let eq47_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            &nodes,
            &eq47_reactive_node_derivatives,
            &branches,
            &eq47_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

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
        let eq48_e1358: f64 = (s.v[0] * s.v[15]);
        let eq48_e1358_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq48_e1358_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq48_e1358_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq48_e1358_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq48_e1358_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq48_e1358_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq48_e1358_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq48_e1358_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq48_e1358_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq48_e1358_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq48_e1358_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq48_e1358_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq48_e1358_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq48_e1360: f64 = (eq48_e1358 * p.p33);
        let eq48_e1360_d_n0: f64 = (eq48_e1358_d_n0 * p.p33);
        let eq48_e1360_d_n1: f64 = (eq48_e1358_d_n1 * p.p33);
        let eq48_e1360_d_n2: f64 = (eq48_e1358_d_n2 * p.p33);
        let eq48_e1360_d_n3: f64 = (eq48_e1358_d_n3 * p.p33);
        let eq48_e1360_d_n4: f64 = (eq48_e1358_d_n4 * p.p33);
        let eq48_e1360_d_n5: f64 = (eq48_e1358_d_n5 * p.p33);
        let eq48_e1360_d_n6: f64 = (eq48_e1358_d_n6 * p.p33);
        let eq48_e1360_d_n7: f64 = (eq48_e1358_d_n7 * p.p33);
        let eq48_e1360_d_n8: f64 = (eq48_e1358_d_n8 * p.p33);
        let eq48_e1360_d_n9: f64 = (eq48_e1358_d_n9 * p.p33);
        let eq48_e1360_d_n10: f64 = (eq48_e1358_d_n10 * p.p33);
        let eq48_e1360_d_n11: f64 = (eq48_e1358_d_n11 * p.p33);
        let eq48_e1360_d_n12: f64 = (eq48_e1358_d_n12 * p.p33);
        let eq48_e1362: f64 = (eq48_e1360 * s.v[847]);
        let eq48_e1362_d_n0: f64 = ((eq48_e1360_d_n0 * s.v[847]) + (eq48_e1360 * s.dn[847][0]));
        let eq48_e1362_d_n1: f64 = ((eq48_e1360_d_n1 * s.v[847]) + (eq48_e1360 * s.dn[847][1]));
        let eq48_e1362_d_n2: f64 = ((eq48_e1360_d_n2 * s.v[847]) + (eq48_e1360 * s.dn[847][2]));
        let eq48_e1362_d_n3: f64 = ((eq48_e1360_d_n3 * s.v[847]) + (eq48_e1360 * s.dn[847][3]));
        let eq48_e1362_d_n4: f64 = ((eq48_e1360_d_n4 * s.v[847]) + (eq48_e1360 * s.dn[847][4]));
        let eq48_e1362_d_n5: f64 = ((eq48_e1360_d_n5 * s.v[847]) + (eq48_e1360 * s.dn[847][5]));
        let eq48_e1362_d_n6: f64 = ((eq48_e1360_d_n6 * s.v[847]) + (eq48_e1360 * s.dn[847][6]));
        let eq48_e1362_d_n7: f64 = ((eq48_e1360_d_n7 * s.v[847]) + (eq48_e1360 * s.dn[847][7]));
        let eq48_e1362_d_n8: f64 = ((eq48_e1360_d_n8 * s.v[847]) + (eq48_e1360 * s.dn[847][8]));
        let eq48_e1362_d_n9: f64 = ((eq48_e1360_d_n9 * s.v[847]) + (eq48_e1360 * s.dn[847][9]));
        let eq48_e1362_d_n10: f64 = ((eq48_e1360_d_n10 * s.v[847]) + (eq48_e1360 * s.dn[847][10]));
        let eq48_e1362_d_n11: f64 = ((eq48_e1360_d_n11 * s.v[847]) + (eq48_e1360 * s.dn[847][11]));
        let eq48_e1362_d_n12: f64 = ((eq48_e1360_d_n12 * s.v[847]) + (eq48_e1360 * s.dn[847][12]));
        let eq48_e1363_q: f64 = eq48_e1362;
        let eq48_reactive_node_derivatives: [f64; 13] = [eq48_e1362_d_n0, eq48_e1362_d_n1, eq48_e1362_d_n2, eq48_e1362_d_n3, eq48_e1362_d_n4, eq48_e1362_d_n5, eq48_e1362_d_n6, eq48_e1362_d_n7, eq48_e1362_d_n8, eq48_e1362_d_n9, eq48_e1362_d_n10, eq48_e1362_d_n11, eq48_e1362_d_n12];
        let eq48_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[8]),
            &nodes,
            &eq48_reactive_node_derivatives,
            &branches,
            &eq48_reactive_branch_derivatives,
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq51_e1374: f64 = (s.v[849] * (nv5 - 0.0));
        let eq51_e1374_d_n0: f64 = (s.dn[849][0] * (nv5 - 0.0));
        let eq51_e1374_d_n1: f64 = (s.dn[849][1] * (nv5 - 0.0));
        let eq51_e1374_d_n2: f64 = (s.dn[849][2] * (nv5 - 0.0));
        let eq51_e1374_d_n3: f64 = (s.dn[849][3] * (nv5 - 0.0));
        let eq51_e1374_d_n4: f64 = (s.dn[849][4] * (nv5 - 0.0));
        let eq51_e1374_d_n5: f64 = ((s.dn[849][5] * (nv5 - 0.0)) + s.v[849]);
        let eq51_e1374_d_n6: f64 = (s.dn[849][6] * (nv5 - 0.0));
        let eq51_e1374_d_n7: f64 = (s.dn[849][7] * (nv5 - 0.0));
        let eq51_e1374_d_n8: f64 = (s.dn[849][8] * (nv5 - 0.0));
        let eq51_e1374_d_n9: f64 = (s.dn[849][9] * (nv5 - 0.0));
        let eq51_e1374_d_n10: f64 = (s.dn[849][10] * (nv5 - 0.0));
        let eq51_e1374_d_n11: f64 = (s.dn[849][11] * (nv5 - 0.0));
        let eq51_e1374_d_n12: f64 = (s.dn[849][12] * (nv5 - 0.0));
        let eq51_e1375_q: f64 = eq51_e1374;
        let eq51_reactive_node_derivatives: [f64; 13] = [eq51_e1374_d_n0, eq51_e1374_d_n1, eq51_e1374_d_n2, eq51_e1374_d_n3, eq51_e1374_d_n4, eq51_e1374_d_n5, eq51_e1374_d_n6, eq51_e1374_d_n7, eq51_e1374_d_n8, eq51_e1374_d_n9, eq51_e1374_d_n10, eq51_e1374_d_n11, eq51_e1374_d_n12];
        let eq51_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq52_e1378: f64 = (s.v[15] * p.p32);
        let eq52_e1378_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq52_e1378_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq52_e1378_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq52_e1378_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq52_e1378_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq52_e1378_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq52_e1378_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq52_e1378_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq52_e1378_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq52_e1378_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq52_e1378_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq52_e1378_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq52_e1378_d_n12: f64 = (s.dn[15][12] * p.p32);
        let eq52_e1379: f64 = (eq52_e1378).sqrt();
        let eq52_e1379_d_n0: f64 = (eq52_e1378_d_n0 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n1: f64 = (eq52_e1378_d_n1 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n2: f64 = (eq52_e1378_d_n2 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n3: f64 = (eq52_e1378_d_n3 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n4: f64 = (eq52_e1378_d_n4 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n5: f64 = (eq52_e1378_d_n5 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n6: f64 = (eq52_e1378_d_n6 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n7: f64 = (eq52_e1378_d_n7 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n8: f64 = (eq52_e1378_d_n8 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n9: f64 = (eq52_e1378_d_n9 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n10: f64 = (eq52_e1378_d_n10 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n11: f64 = (eq52_e1378_d_n11 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n12: f64 = (eq52_e1378_d_n12 / (2.0 * eq52_e1379));
        let eq52_e1381: f64 = (eq52_e1379 * 0.5);
        let eq52_e1381_d_n0: f64 = (eq52_e1379_d_n0 * 0.5);
        let eq52_e1381_d_n1: f64 = (eq52_e1379_d_n1 * 0.5);
        let eq52_e1381_d_n2: f64 = (eq52_e1379_d_n2 * 0.5);
        let eq52_e1381_d_n3: f64 = (eq52_e1379_d_n3 * 0.5);
        let eq52_e1381_d_n4: f64 = (eq52_e1379_d_n4 * 0.5);
        let eq52_e1381_d_n5: f64 = (eq52_e1379_d_n5 * 0.5);
        let eq52_e1381_d_n6: f64 = (eq52_e1379_d_n6 * 0.5);
        let eq52_e1381_d_n7: f64 = (eq52_e1379_d_n7 * 0.5);
        let eq52_e1381_d_n8: f64 = (eq52_e1379_d_n8 * 0.5);
        let eq52_e1381_d_n9: f64 = (eq52_e1379_d_n9 * 0.5);
        let eq52_e1381_d_n10: f64 = (eq52_e1379_d_n10 * 0.5);
        let eq52_e1381_d_n11: f64 = (eq52_e1379_d_n11 * 0.5);
        let eq52_e1381_d_n12: f64 = (eq52_e1379_d_n12 * 0.5);
        let eq52_e1383: f64 = (eq52_e1381 * s.v[849]);
        let eq52_e1383_d_n0: f64 = ((eq52_e1381_d_n0 * s.v[849]) + (eq52_e1381 * s.dn[849][0]));
        let eq52_e1383_d_n1: f64 = ((eq52_e1381_d_n1 * s.v[849]) + (eq52_e1381 * s.dn[849][1]));
        let eq52_e1383_d_n2: f64 = ((eq52_e1381_d_n2 * s.v[849]) + (eq52_e1381 * s.dn[849][2]));
        let eq52_e1383_d_n3: f64 = ((eq52_e1381_d_n3 * s.v[849]) + (eq52_e1381 * s.dn[849][3]));
        let eq52_e1383_d_n4: f64 = ((eq52_e1381_d_n4 * s.v[849]) + (eq52_e1381 * s.dn[849][4]));
        let eq52_e1383_d_n5: f64 = ((eq52_e1381_d_n5 * s.v[849]) + (eq52_e1381 * s.dn[849][5]));
        let eq52_e1383_d_n6: f64 = ((eq52_e1381_d_n6 * s.v[849]) + (eq52_e1381 * s.dn[849][6]));
        let eq52_e1383_d_n7: f64 = ((eq52_e1381_d_n7 * s.v[849]) + (eq52_e1381 * s.dn[849][7]));
        let eq52_e1383_d_n8: f64 = ((eq52_e1381_d_n8 * s.v[849]) + (eq52_e1381 * s.dn[849][8]));
        let eq52_e1383_d_n9: f64 = ((eq52_e1381_d_n9 * s.v[849]) + (eq52_e1381 * s.dn[849][9]));
        let eq52_e1383_d_n10: f64 = ((eq52_e1381_d_n10 * s.v[849]) + (eq52_e1381 * s.dn[849][10]));
        let eq52_e1383_d_n11: f64 = ((eq52_e1381_d_n11 * s.v[849]) + (eq52_e1381 * s.dn[849][11]));
        let eq52_e1383_d_n12: f64 = ((eq52_e1381_d_n12 * s.v[849]) + (eq52_e1381 * s.dn[849][12]));
        let eq52_e1385: f64 = (eq52_e1383 * (nv5 - 0.0));
        let eq52_e1385_d_n0: f64 = (eq52_e1383_d_n0 * (nv5 - 0.0));
        let eq52_e1385_d_n1: f64 = (eq52_e1383_d_n1 * (nv5 - 0.0));
        let eq52_e1385_d_n2: f64 = (eq52_e1383_d_n2 * (nv5 - 0.0));
        let eq52_e1385_d_n3: f64 = (eq52_e1383_d_n3 * (nv5 - 0.0));
        let eq52_e1385_d_n4: f64 = (eq52_e1383_d_n4 * (nv5 - 0.0));
        let eq52_e1385_d_n5: f64 = ((eq52_e1383_d_n5 * (nv5 - 0.0)) + eq52_e1383);
        let eq52_e1385_d_n6: f64 = (eq52_e1383_d_n6 * (nv5 - 0.0));
        let eq52_e1385_d_n7: f64 = (eq52_e1383_d_n7 * (nv5 - 0.0));
        let eq52_e1385_d_n8: f64 = (eq52_e1383_d_n8 * (nv5 - 0.0));
        let eq52_e1385_d_n9: f64 = (eq52_e1383_d_n9 * (nv5 - 0.0));
        let eq52_e1385_d_n10: f64 = (eq52_e1383_d_n10 * (nv5 - 0.0));
        let eq52_e1385_d_n11: f64 = (eq52_e1383_d_n11 * (nv5 - 0.0));
        let eq52_e1385_d_n12: f64 = (eq52_e1383_d_n12 * (nv5 - 0.0));
        let eq52_e1386_q: f64 = eq52_e1385;
        let eq52_e1387: f64 = (-eq52_e1385);
        let eq52_e1387_d_n0: f64 = (-eq52_e1385_d_n0);
        let eq52_e1387_d_n1: f64 = (-eq52_e1385_d_n1);
        let eq52_e1387_d_n2: f64 = (-eq52_e1385_d_n2);
        let eq52_e1387_d_n3: f64 = (-eq52_e1385_d_n3);
        let eq52_e1387_d_n4: f64 = (-eq52_e1385_d_n4);
        let eq52_e1387_d_n5: f64 = (-eq52_e1385_d_n5);
        let eq52_e1387_d_n6: f64 = (-eq52_e1385_d_n6);
        let eq52_e1387_d_n7: f64 = (-eq52_e1385_d_n7);
        let eq52_e1387_d_n8: f64 = (-eq52_e1385_d_n8);
        let eq52_e1387_d_n9: f64 = (-eq52_e1385_d_n9);
        let eq52_e1387_d_n10: f64 = (-eq52_e1385_d_n10);
        let eq52_e1387_d_n11: f64 = (-eq52_e1385_d_n11);
        let eq52_e1387_d_n12: f64 = (-eq52_e1385_d_n12);
        let eq52_e1387_q: f64 = (-eq52_e1386_q);
        let eq52_e1387_q_d_n0: f64 = (-eq52_e1385_d_n0);
        let eq52_e1387_q_d_n1: f64 = (-eq52_e1385_d_n1);
        let eq52_e1387_q_d_n2: f64 = (-eq52_e1385_d_n2);
        let eq52_e1387_q_d_n3: f64 = (-eq52_e1385_d_n3);
        let eq52_e1387_q_d_n4: f64 = (-eq52_e1385_d_n4);
        let eq52_e1387_q_d_n5: f64 = (-eq52_e1385_d_n5);
        let eq52_e1387_q_d_n6: f64 = (-eq52_e1385_d_n6);
        let eq52_e1387_q_d_n7: f64 = (-eq52_e1385_d_n7);
        let eq52_e1387_q_d_n8: f64 = (-eq52_e1385_d_n8);
        let eq52_e1387_q_d_n9: f64 = (-eq52_e1385_d_n9);
        let eq52_e1387_q_d_n10: f64 = (-eq52_e1385_d_n10);
        let eq52_e1387_q_d_n11: f64 = (-eq52_e1385_d_n11);
        let eq52_e1387_q_d_n12: f64 = (-eq52_e1385_d_n12);
        let eq52_reactive_node_derivatives: [f64; 13] = [eq52_e1387_q_d_n0, eq52_e1387_q_d_n1, eq52_e1387_q_d_n2, eq52_e1387_q_d_n3, eq52_e1387_q_d_n4, eq52_e1387_q_d_n5, eq52_e1387_q_d_n6, eq52_e1387_q_d_n7, eq52_e1387_q_d_n8, eq52_e1387_q_d_n9, eq52_e1387_q_d_n10, eq52_e1387_q_d_n11, eq52_e1387_q_d_n12];
        let eq52_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq53_e1390: f64 = (s.v[15] * p.p32);
        let eq53_e1390_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq53_e1390_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq53_e1390_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq53_e1390_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq53_e1390_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq53_e1390_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq53_e1390_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq53_e1390_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq53_e1390_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq53_e1390_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq53_e1390_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq53_e1390_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq53_e1390_d_n12: f64 = (s.dn[15][12] * p.p32);
        let eq53_e1391: f64 = (eq53_e1390).sqrt();
        let eq53_e1391_d_n0: f64 = (eq53_e1390_d_n0 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n1: f64 = (eq53_e1390_d_n1 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n2: f64 = (eq53_e1390_d_n2 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n3: f64 = (eq53_e1390_d_n3 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n4: f64 = (eq53_e1390_d_n4 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n5: f64 = (eq53_e1390_d_n5 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n6: f64 = (eq53_e1390_d_n6 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n7: f64 = (eq53_e1390_d_n7 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n8: f64 = (eq53_e1390_d_n8 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n9: f64 = (eq53_e1390_d_n9 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n10: f64 = (eq53_e1390_d_n10 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n11: f64 = (eq53_e1390_d_n11 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n12: f64 = (eq53_e1390_d_n12 / (2.0 * eq53_e1391));
        let eq53_e1393: f64 = (eq53_e1391 * 0.5);
        let eq53_e1393_d_n0: f64 = (eq53_e1391_d_n0 * 0.5);
        let eq53_e1393_d_n1: f64 = (eq53_e1391_d_n1 * 0.5);
        let eq53_e1393_d_n2: f64 = (eq53_e1391_d_n2 * 0.5);
        let eq53_e1393_d_n3: f64 = (eq53_e1391_d_n3 * 0.5);
        let eq53_e1393_d_n4: f64 = (eq53_e1391_d_n4 * 0.5);
        let eq53_e1393_d_n5: f64 = (eq53_e1391_d_n5 * 0.5);
        let eq53_e1393_d_n6: f64 = (eq53_e1391_d_n6 * 0.5);
        let eq53_e1393_d_n7: f64 = (eq53_e1391_d_n7 * 0.5);
        let eq53_e1393_d_n8: f64 = (eq53_e1391_d_n8 * 0.5);
        let eq53_e1393_d_n9: f64 = (eq53_e1391_d_n9 * 0.5);
        let eq53_e1393_d_n10: f64 = (eq53_e1391_d_n10 * 0.5);
        let eq53_e1393_d_n11: f64 = (eq53_e1391_d_n11 * 0.5);
        let eq53_e1393_d_n12: f64 = (eq53_e1391_d_n12 * 0.5);
        let eq53_e1395: f64 = (eq53_e1393 * s.v[849]);
        let eq53_e1395_d_n0: f64 = ((eq53_e1393_d_n0 * s.v[849]) + (eq53_e1393 * s.dn[849][0]));
        let eq53_e1395_d_n1: f64 = ((eq53_e1393_d_n1 * s.v[849]) + (eq53_e1393 * s.dn[849][1]));
        let eq53_e1395_d_n2: f64 = ((eq53_e1393_d_n2 * s.v[849]) + (eq53_e1393 * s.dn[849][2]));
        let eq53_e1395_d_n3: f64 = ((eq53_e1393_d_n3 * s.v[849]) + (eq53_e1393 * s.dn[849][3]));
        let eq53_e1395_d_n4: f64 = ((eq53_e1393_d_n4 * s.v[849]) + (eq53_e1393 * s.dn[849][4]));
        let eq53_e1395_d_n5: f64 = ((eq53_e1393_d_n5 * s.v[849]) + (eq53_e1393 * s.dn[849][5]));
        let eq53_e1395_d_n6: f64 = ((eq53_e1393_d_n6 * s.v[849]) + (eq53_e1393 * s.dn[849][6]));
        let eq53_e1395_d_n7: f64 = ((eq53_e1393_d_n7 * s.v[849]) + (eq53_e1393 * s.dn[849][7]));
        let eq53_e1395_d_n8: f64 = ((eq53_e1393_d_n8 * s.v[849]) + (eq53_e1393 * s.dn[849][8]));
        let eq53_e1395_d_n9: f64 = ((eq53_e1393_d_n9 * s.v[849]) + (eq53_e1393 * s.dn[849][9]));
        let eq53_e1395_d_n10: f64 = ((eq53_e1393_d_n10 * s.v[849]) + (eq53_e1393 * s.dn[849][10]));
        let eq53_e1395_d_n11: f64 = ((eq53_e1393_d_n11 * s.v[849]) + (eq53_e1393 * s.dn[849][11]));
        let eq53_e1395_d_n12: f64 = ((eq53_e1393_d_n12 * s.v[849]) + (eq53_e1393 * s.dn[849][12]));
        let eq53_e1397: f64 = (eq53_e1395 * (nv5 - 0.0));
        let eq53_e1397_d_n0: f64 = (eq53_e1395_d_n0 * (nv5 - 0.0));
        let eq53_e1397_d_n1: f64 = (eq53_e1395_d_n1 * (nv5 - 0.0));
        let eq53_e1397_d_n2: f64 = (eq53_e1395_d_n2 * (nv5 - 0.0));
        let eq53_e1397_d_n3: f64 = (eq53_e1395_d_n3 * (nv5 - 0.0));
        let eq53_e1397_d_n4: f64 = (eq53_e1395_d_n4 * (nv5 - 0.0));
        let eq53_e1397_d_n5: f64 = ((eq53_e1395_d_n5 * (nv5 - 0.0)) + eq53_e1395);
        let eq53_e1397_d_n6: f64 = (eq53_e1395_d_n6 * (nv5 - 0.0));
        let eq53_e1397_d_n7: f64 = (eq53_e1395_d_n7 * (nv5 - 0.0));
        let eq53_e1397_d_n8: f64 = (eq53_e1395_d_n8 * (nv5 - 0.0));
        let eq53_e1397_d_n9: f64 = (eq53_e1395_d_n9 * (nv5 - 0.0));
        let eq53_e1397_d_n10: f64 = (eq53_e1395_d_n10 * (nv5 - 0.0));
        let eq53_e1397_d_n11: f64 = (eq53_e1395_d_n11 * (nv5 - 0.0));
        let eq53_e1397_d_n12: f64 = (eq53_e1395_d_n12 * (nv5 - 0.0));
        let eq53_e1398_q: f64 = eq53_e1397;
        let eq53_e1399: f64 = (-eq53_e1397);
        let eq53_e1399_d_n0: f64 = (-eq53_e1397_d_n0);
        let eq53_e1399_d_n1: f64 = (-eq53_e1397_d_n1);
        let eq53_e1399_d_n2: f64 = (-eq53_e1397_d_n2);
        let eq53_e1399_d_n3: f64 = (-eq53_e1397_d_n3);
        let eq53_e1399_d_n4: f64 = (-eq53_e1397_d_n4);
        let eq53_e1399_d_n5: f64 = (-eq53_e1397_d_n5);
        let eq53_e1399_d_n6: f64 = (-eq53_e1397_d_n6);
        let eq53_e1399_d_n7: f64 = (-eq53_e1397_d_n7);
        let eq53_e1399_d_n8: f64 = (-eq53_e1397_d_n8);
        let eq53_e1399_d_n9: f64 = (-eq53_e1397_d_n9);
        let eq53_e1399_d_n10: f64 = (-eq53_e1397_d_n10);
        let eq53_e1399_d_n11: f64 = (-eq53_e1397_d_n11);
        let eq53_e1399_d_n12: f64 = (-eq53_e1397_d_n12);
        let eq53_e1399_q: f64 = (-eq53_e1398_q);
        let eq53_e1399_q_d_n0: f64 = (-eq53_e1397_d_n0);
        let eq53_e1399_q_d_n1: f64 = (-eq53_e1397_d_n1);
        let eq53_e1399_q_d_n2: f64 = (-eq53_e1397_d_n2);
        let eq53_e1399_q_d_n3: f64 = (-eq53_e1397_d_n3);
        let eq53_e1399_q_d_n4: f64 = (-eq53_e1397_d_n4);
        let eq53_e1399_q_d_n5: f64 = (-eq53_e1397_d_n5);
        let eq53_e1399_q_d_n6: f64 = (-eq53_e1397_d_n6);
        let eq53_e1399_q_d_n7: f64 = (-eq53_e1397_d_n7);
        let eq53_e1399_q_d_n8: f64 = (-eq53_e1397_d_n8);
        let eq53_e1399_q_d_n9: f64 = (-eq53_e1397_d_n9);
        let eq53_e1399_q_d_n10: f64 = (-eq53_e1397_d_n10);
        let eq53_e1399_q_d_n11: f64 = (-eq53_e1397_d_n11);
        let eq53_e1399_q_d_n12: f64 = (-eq53_e1397_d_n12);
        let eq53_reactive_node_derivatives: [f64; 13] = [eq53_e1399_q_d_n0, eq53_e1399_q_d_n1, eq53_e1399_q_d_n2, eq53_e1399_q_d_n3, eq53_e1399_q_d_n4, eq53_e1399_q_d_n5, eq53_e1399_q_d_n6, eq53_e1399_q_d_n7, eq53_e1399_q_d_n8, eq53_e1399_q_d_n9, eq53_e1399_q_d_n10, eq53_e1399_q_d_n11, eq53_e1399_q_d_n12];
        let eq53_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            &nodes,
            &eq53_reactive_node_derivatives,
            &branches,
            &eq53_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
