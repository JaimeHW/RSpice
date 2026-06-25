#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_47_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
        let eq47_e1355: f64 = self.eval_ddt(7, eq47_e1354);
        let eq47_e1355_d_n0: f64 = self.ddt_jacobian(eq47_e1354_d_n0);
        let eq47_e1355_d_n1: f64 = self.ddt_jacobian(eq47_e1354_d_n1);
        let eq47_e1355_d_n2: f64 = self.ddt_jacobian(eq47_e1354_d_n2);
        let eq47_e1355_d_n3: f64 = self.ddt_jacobian(eq47_e1354_d_n3);
        let eq47_e1355_d_n4: f64 = self.ddt_jacobian(eq47_e1354_d_n4);
        let eq47_e1355_d_n5: f64 = self.ddt_jacobian(eq47_e1354_d_n5);
        let eq47_e1355_d_n6: f64 = self.ddt_jacobian(eq47_e1354_d_n6);
        let eq47_e1355_d_n7: f64 = self.ddt_jacobian(eq47_e1354_d_n7);
        let eq47_e1355_d_n8: f64 = self.ddt_jacobian(eq47_e1354_d_n8);
        let eq47_e1355_d_n9: f64 = self.ddt_jacobian(eq47_e1354_d_n9);
        let eq47_e1355_d_n10: f64 = self.ddt_jacobian(eq47_e1354_d_n10);
        let eq47_e1355_d_n11: f64 = self.ddt_jacobian(eq47_e1354_d_n11);
        let eq47_e1355_d_n12: f64 = self.ddt_jacobian(eq47_e1354_d_n12);
        let eq47_value: f64 = eq47_e1355;
        let eq47_node_derivatives: [f64; 13] = [eq47_e1355_d_n0, eq47_e1355_d_n1, eq47_e1355_d_n2, eq47_e1355_d_n3, eq47_e1355_d_n4, eq47_e1355_d_n5, eq47_e1355_d_n6, eq47_e1355_d_n7, eq47_e1355_d_n8, eq47_e1355_d_n9, eq47_e1355_d_n10, eq47_e1355_d_n11, eq47_e1355_d_n12];
        let eq47_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            self.multiplicity * (eq47_value),
            &nodes,
            &eq47_node_derivatives,
            &branches,
            &eq47_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_48_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
        let eq48_e1363: f64 = self.eval_ddt(8, eq48_e1362);
        let eq48_e1363_d_n0: f64 = self.ddt_jacobian(eq48_e1362_d_n0);
        let eq48_e1363_d_n1: f64 = self.ddt_jacobian(eq48_e1362_d_n1);
        let eq48_e1363_d_n2: f64 = self.ddt_jacobian(eq48_e1362_d_n2);
        let eq48_e1363_d_n3: f64 = self.ddt_jacobian(eq48_e1362_d_n3);
        let eq48_e1363_d_n4: f64 = self.ddt_jacobian(eq48_e1362_d_n4);
        let eq48_e1363_d_n5: f64 = self.ddt_jacobian(eq48_e1362_d_n5);
        let eq48_e1363_d_n6: f64 = self.ddt_jacobian(eq48_e1362_d_n6);
        let eq48_e1363_d_n7: f64 = self.ddt_jacobian(eq48_e1362_d_n7);
        let eq48_e1363_d_n8: f64 = self.ddt_jacobian(eq48_e1362_d_n8);
        let eq48_e1363_d_n9: f64 = self.ddt_jacobian(eq48_e1362_d_n9);
        let eq48_e1363_d_n10: f64 = self.ddt_jacobian(eq48_e1362_d_n10);
        let eq48_e1363_d_n11: f64 = self.ddt_jacobian(eq48_e1362_d_n11);
        let eq48_e1363_d_n12: f64 = self.ddt_jacobian(eq48_e1362_d_n12);
        let eq48_value: f64 = eq48_e1363;
        let eq48_node_derivatives: [f64; 13] = [eq48_e1363_d_n0, eq48_e1363_d_n1, eq48_e1363_d_n2, eq48_e1363_d_n3, eq48_e1363_d_n4, eq48_e1363_d_n5, eq48_e1363_d_n6, eq48_e1363_d_n7, eq48_e1363_d_n8, eq48_e1363_d_n9, eq48_e1363_d_n10, eq48_e1363_d_n11, eq48_e1363_d_n12];
        let eq48_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            Some(nodes[8]),
            self.multiplicity * (eq48_value),
            &nodes,
            &eq48_node_derivatives,
            &branches,
            &eq48_branch_derivatives,
            self.multiplicity,
        );
    }

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
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq50_e1371: f64 = ((nv5 - 0.0) / s.v[848]);
        let eq50_e1371_d_n0: f64 = (-(((nv5 - 0.0) * s.dn[848][0]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n1: f64 = (-(((nv5 - 0.0) * s.dn[848][1]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n2: f64 = (-(((nv5 - 0.0) * s.dn[848][2]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n3: f64 = (-(((nv5 - 0.0) * s.dn[848][3]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n4: f64 = (-(((nv5 - 0.0) * s.dn[848][4]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n5: f64 = ((s.v[848] - ((nv5 - 0.0) * s.dn[848][5])) / (s.v[848] * s.v[848]));
        let eq50_e1371_d_n6: f64 = (-(((nv5 - 0.0) * s.dn[848][6]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n7: f64 = (-(((nv5 - 0.0) * s.dn[848][7]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n8: f64 = (-(((nv5 - 0.0) * s.dn[848][8]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n9: f64 = (-(((nv5 - 0.0) * s.dn[848][9]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n10: f64 = (-(((nv5 - 0.0) * s.dn[848][10]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n11: f64 = (-(((nv5 - 0.0) * s.dn[848][11]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n12: f64 = (-(((nv5 - 0.0) * s.dn[848][12]) / (s.v[848] * s.v[848])));
        let eq50_value: f64 = eq50_e1371;
        let eq50_node_derivatives: [f64; 13] = [eq50_e1371_d_n0, eq50_e1371_d_n1, eq50_e1371_d_n2, eq50_e1371_d_n3, eq50_e1371_d_n4, eq50_e1371_d_n5, eq50_e1371_d_n6, eq50_e1371_d_n7, eq50_e1371_d_n8, eq50_e1371_d_n9, eq50_e1371_d_n10, eq50_e1371_d_n11, eq50_e1371_d_n12];
        let eq50_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            None,
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
        let eq51_e1375: f64 = self.eval_ddt(9, eq51_e1374);
        let eq51_e1375_d_n0: f64 = self.ddt_jacobian(eq51_e1374_d_n0);
        let eq51_e1375_d_n1: f64 = self.ddt_jacobian(eq51_e1374_d_n1);
        let eq51_e1375_d_n2: f64 = self.ddt_jacobian(eq51_e1374_d_n2);
        let eq51_e1375_d_n3: f64 = self.ddt_jacobian(eq51_e1374_d_n3);
        let eq51_e1375_d_n4: f64 = self.ddt_jacobian(eq51_e1374_d_n4);
        let eq51_e1375_d_n5: f64 = self.ddt_jacobian(eq51_e1374_d_n5);
        let eq51_e1375_d_n6: f64 = self.ddt_jacobian(eq51_e1374_d_n6);
        let eq51_e1375_d_n7: f64 = self.ddt_jacobian(eq51_e1374_d_n7);
        let eq51_e1375_d_n8: f64 = self.ddt_jacobian(eq51_e1374_d_n8);
        let eq51_e1375_d_n9: f64 = self.ddt_jacobian(eq51_e1374_d_n9);
        let eq51_e1375_d_n10: f64 = self.ddt_jacobian(eq51_e1374_d_n10);
        let eq51_e1375_d_n11: f64 = self.ddt_jacobian(eq51_e1374_d_n11);
        let eq51_e1375_d_n12: f64 = self.ddt_jacobian(eq51_e1374_d_n12);
        let eq51_value: f64 = eq51_e1375;
        let eq51_node_derivatives: [f64; 13] = [eq51_e1375_d_n0, eq51_e1375_d_n1, eq51_e1375_d_n2, eq51_e1375_d_n3, eq51_e1375_d_n4, eq51_e1375_d_n5, eq51_e1375_d_n6, eq51_e1375_d_n7, eq51_e1375_d_n8, eq51_e1375_d_n9, eq51_e1375_d_n10, eq51_e1375_d_n11, eq51_e1375_d_n12];
        let eq51_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            None,
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
        let eq52_e1386: f64 = self.eval_ddt(10, eq52_e1385);
        let eq52_e1386_d_n0: f64 = self.ddt_jacobian(eq52_e1385_d_n0);
        let eq52_e1386_d_n1: f64 = self.ddt_jacobian(eq52_e1385_d_n1);
        let eq52_e1386_d_n2: f64 = self.ddt_jacobian(eq52_e1385_d_n2);
        let eq52_e1386_d_n3: f64 = self.ddt_jacobian(eq52_e1385_d_n3);
        let eq52_e1386_d_n4: f64 = self.ddt_jacobian(eq52_e1385_d_n4);
        let eq52_e1386_d_n5: f64 = self.ddt_jacobian(eq52_e1385_d_n5);
        let eq52_e1386_d_n6: f64 = self.ddt_jacobian(eq52_e1385_d_n6);
        let eq52_e1386_d_n7: f64 = self.ddt_jacobian(eq52_e1385_d_n7);
        let eq52_e1386_d_n8: f64 = self.ddt_jacobian(eq52_e1385_d_n8);
        let eq52_e1386_d_n9: f64 = self.ddt_jacobian(eq52_e1385_d_n9);
        let eq52_e1386_d_n10: f64 = self.ddt_jacobian(eq52_e1385_d_n10);
        let eq52_e1386_d_n11: f64 = self.ddt_jacobian(eq52_e1385_d_n11);
        let eq52_e1386_d_n12: f64 = self.ddt_jacobian(eq52_e1385_d_n12);
        let eq52_e1387: f64 = (-eq52_e1386);
        let eq52_e1387_d_n0: f64 = (-eq52_e1386_d_n0);
        let eq52_e1387_d_n1: f64 = (-eq52_e1386_d_n1);
        let eq52_e1387_d_n2: f64 = (-eq52_e1386_d_n2);
        let eq52_e1387_d_n3: f64 = (-eq52_e1386_d_n3);
        let eq52_e1387_d_n4: f64 = (-eq52_e1386_d_n4);
        let eq52_e1387_d_n5: f64 = (-eq52_e1386_d_n5);
        let eq52_e1387_d_n6: f64 = (-eq52_e1386_d_n6);
        let eq52_e1387_d_n7: f64 = (-eq52_e1386_d_n7);
        let eq52_e1387_d_n8: f64 = (-eq52_e1386_d_n8);
        let eq52_e1387_d_n9: f64 = (-eq52_e1386_d_n9);
        let eq52_e1387_d_n10: f64 = (-eq52_e1386_d_n10);
        let eq52_e1387_d_n11: f64 = (-eq52_e1386_d_n11);
        let eq52_e1387_d_n12: f64 = (-eq52_e1386_d_n12);
        let eq52_value: f64 = eq52_e1387;
        let eq52_node_derivatives: [f64; 13] = [eq52_e1387_d_n0, eq52_e1387_d_n1, eq52_e1387_d_n2, eq52_e1387_d_n3, eq52_e1387_d_n4, eq52_e1387_d_n5, eq52_e1387_d_n6, eq52_e1387_d_n7, eq52_e1387_d_n8, eq52_e1387_d_n9, eq52_e1387_d_n10, eq52_e1387_d_n11, eq52_e1387_d_n12];
        let eq52_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[7]),
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
        let eq53_e1398: f64 = self.eval_ddt(11, eq53_e1397);
        let eq53_e1398_d_n0: f64 = self.ddt_jacobian(eq53_e1397_d_n0);
        let eq53_e1398_d_n1: f64 = self.ddt_jacobian(eq53_e1397_d_n1);
        let eq53_e1398_d_n2: f64 = self.ddt_jacobian(eq53_e1397_d_n2);
        let eq53_e1398_d_n3: f64 = self.ddt_jacobian(eq53_e1397_d_n3);
        let eq53_e1398_d_n4: f64 = self.ddt_jacobian(eq53_e1397_d_n4);
        let eq53_e1398_d_n5: f64 = self.ddt_jacobian(eq53_e1397_d_n5);
        let eq53_e1398_d_n6: f64 = self.ddt_jacobian(eq53_e1397_d_n6);
        let eq53_e1398_d_n7: f64 = self.ddt_jacobian(eq53_e1397_d_n7);
        let eq53_e1398_d_n8: f64 = self.ddt_jacobian(eq53_e1397_d_n8);
        let eq53_e1398_d_n9: f64 = self.ddt_jacobian(eq53_e1397_d_n9);
        let eq53_e1398_d_n10: f64 = self.ddt_jacobian(eq53_e1397_d_n10);
        let eq53_e1398_d_n11: f64 = self.ddt_jacobian(eq53_e1397_d_n11);
        let eq53_e1398_d_n12: f64 = self.ddt_jacobian(eq53_e1397_d_n12);
        let eq53_e1399: f64 = (-eq53_e1398);
        let eq53_e1399_d_n0: f64 = (-eq53_e1398_d_n0);
        let eq53_e1399_d_n1: f64 = (-eq53_e1398_d_n1);
        let eq53_e1399_d_n2: f64 = (-eq53_e1398_d_n2);
        let eq53_e1399_d_n3: f64 = (-eq53_e1398_d_n3);
        let eq53_e1399_d_n4: f64 = (-eq53_e1398_d_n4);
        let eq53_e1399_d_n5: f64 = (-eq53_e1398_d_n5);
        let eq53_e1399_d_n6: f64 = (-eq53_e1398_d_n6);
        let eq53_e1399_d_n7: f64 = (-eq53_e1398_d_n7);
        let eq53_e1399_d_n8: f64 = (-eq53_e1398_d_n8);
        let eq53_e1399_d_n9: f64 = (-eq53_e1398_d_n9);
        let eq53_e1399_d_n10: f64 = (-eq53_e1398_d_n10);
        let eq53_e1399_d_n11: f64 = (-eq53_e1398_d_n11);
        let eq53_e1399_d_n12: f64 = (-eq53_e1398_d_n12);
        let eq53_value: f64 = eq53_e1399;
        let eq53_node_derivatives: [f64; 13] = [eq53_e1399_d_n0, eq53_e1399_d_n1, eq53_e1399_d_n2, eq53_e1399_d_n3, eq53_e1399_d_n4, eq53_e1399_d_n5, eq53_e1399_d_n6, eq53_e1399_d_n7, eq53_e1399_d_n8, eq53_e1399_d_n9, eq53_e1399_d_n10, eq53_e1399_d_n11, eq53_e1399_d_n12];
        let eq53_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            self.multiplicity * (eq53_value),
            &nodes,
            &eq53_node_derivatives,
            &branches,
            &eq53_branch_derivatives,
            self.multiplicity,
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
        let eq55_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[7]),
            self.multiplicity * (eq55_value),
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
        let eq56_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[7]),
            self.multiplicity * (eq56_value),
            &[
            ],
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
        let eq57_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq57_value),
            &[
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
        let eq58_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[8]),
            self.multiplicity * (eq58_value),
            &[
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
        let eq59_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[11]),
            Some(nodes[7]),
            self.multiplicity * (eq59_value),
            &[
            ],
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
        let eq60_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[12]),
            Some(nodes[8]),
            self.multiplicity * (eq60_value),
            &[
            ],
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
        let eq61_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[7]),
            self.multiplicity * (eq61_value),
            &[
            ],
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
        let eq62_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[7]),
            self.multiplicity * (eq62_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_39_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq39_e1291: f64 = (s.v[15] * s.v[306]);
        let eq39_e1291_d_n0: f64 = ((s.dn[15][0] * s.v[306]) + (s.v[15] * s.dn[306][0]));
        let eq39_e1291_d_n1: f64 = ((s.dn[15][1] * s.v[306]) + (s.v[15] * s.dn[306][1]));
        let eq39_e1291_d_n2: f64 = ((s.dn[15][2] * s.v[306]) + (s.v[15] * s.dn[306][2]));
        let eq39_e1291_d_n3: f64 = ((s.dn[15][3] * s.v[306]) + (s.v[15] * s.dn[306][3]));
        let eq39_e1291_d_n4: f64 = ((s.dn[15][4] * s.v[306]) + (s.v[15] * s.dn[306][4]));
        let eq39_e1291_d_n5: f64 = ((s.dn[15][5] * s.v[306]) + (s.v[15] * s.dn[306][5]));
        let eq39_e1291_d_n6: f64 = ((s.dn[15][6] * s.v[306]) + (s.v[15] * s.dn[306][6]));
        let eq39_e1291_d_n7: f64 = ((s.dn[15][7] * s.v[306]) + (s.v[15] * s.dn[306][7]));
        let eq39_e1291_d_n8: f64 = ((s.dn[15][8] * s.v[306]) + (s.v[15] * s.dn[306][8]));
        let eq39_e1291_d_n9: f64 = ((s.dn[15][9] * s.v[306]) + (s.v[15] * s.dn[306][9]));
        let eq39_e1291_d_n10: f64 = ((s.dn[15][10] * s.v[306]) + (s.v[15] * s.dn[306][10]));
        let eq39_e1291_d_n11: f64 = ((s.dn[15][11] * s.v[306]) + (s.v[15] * s.dn[306][11]));
        let eq39_e1291_d_n12: f64 = ((s.dn[15][12] * s.v[306]) + (s.v[15] * s.dn[306][12]));
        let eq39_e1293: f64 = (eq39_e1291 * (nv4 - 0.0));
        let eq39_e1293_d_n0: f64 = (eq39_e1291_d_n0 * (nv4 - 0.0));
        let eq39_e1293_d_n1: f64 = (eq39_e1291_d_n1 * (nv4 - 0.0));
        let eq39_e1293_d_n2: f64 = (eq39_e1291_d_n2 * (nv4 - 0.0));
        let eq39_e1293_d_n3: f64 = (eq39_e1291_d_n3 * (nv4 - 0.0));
        let eq39_e1293_d_n4: f64 = ((eq39_e1291_d_n4 * (nv4 - 0.0)) + eq39_e1291);
        let eq39_e1293_d_n5: f64 = (eq39_e1291_d_n5 * (nv4 - 0.0));
        let eq39_e1293_d_n6: f64 = (eq39_e1291_d_n6 * (nv4 - 0.0));
        let eq39_e1293_d_n7: f64 = (eq39_e1291_d_n7 * (nv4 - 0.0));
        let eq39_e1293_d_n8: f64 = (eq39_e1291_d_n8 * (nv4 - 0.0));
        let eq39_e1293_d_n9: f64 = (eq39_e1291_d_n9 * (nv4 - 0.0));
        let eq39_e1293_d_n10: f64 = (eq39_e1291_d_n10 * (nv4 - 0.0));
        let eq39_e1293_d_n11: f64 = (eq39_e1291_d_n11 * (nv4 - 0.0));
        let eq39_e1293_d_n12: f64 = (eq39_e1291_d_n12 * (nv4 - 0.0));
        let eq39_e1294_q: f64 = eq39_e1293;
        let eq39_reactive_node_derivatives: [f64; 13] = [eq39_e1293_d_n0, eq39_e1293_d_n1, eq39_e1293_d_n2, eq39_e1293_d_n3, eq39_e1293_d_n4, eq39_e1293_d_n5, eq39_e1293_d_n6, eq39_e1293_d_n7, eq39_e1293_d_n8, eq39_e1293_d_n9, eq39_e1293_d_n10, eq39_e1293_d_n11, eq39_e1293_d_n12];
        let eq39_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            &nodes,
            &eq39_reactive_node_derivatives,
            &branches,
            &eq39_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_41_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq41_e1302: f64 = (s.v[0] * s.v[15]);
        let eq41_e1302_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq41_e1302_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq41_e1302_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq41_e1302_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq41_e1302_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq41_e1302_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq41_e1302_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq41_e1302_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq41_e1302_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq41_e1302_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq41_e1302_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq41_e1302_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq41_e1302_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq41_e1304: f64 = (eq41_e1302 * p.p33);
        let eq41_e1304_d_n0: f64 = (eq41_e1302_d_n0 * p.p33);
        let eq41_e1304_d_n1: f64 = (eq41_e1302_d_n1 * p.p33);
        let eq41_e1304_d_n2: f64 = (eq41_e1302_d_n2 * p.p33);
        let eq41_e1304_d_n3: f64 = (eq41_e1302_d_n3 * p.p33);
        let eq41_e1304_d_n4: f64 = (eq41_e1302_d_n4 * p.p33);
        let eq41_e1304_d_n5: f64 = (eq41_e1302_d_n5 * p.p33);
        let eq41_e1304_d_n6: f64 = (eq41_e1302_d_n6 * p.p33);
        let eq41_e1304_d_n7: f64 = (eq41_e1302_d_n7 * p.p33);
        let eq41_e1304_d_n8: f64 = (eq41_e1302_d_n8 * p.p33);
        let eq41_e1304_d_n9: f64 = (eq41_e1302_d_n9 * p.p33);
        let eq41_e1304_d_n10: f64 = (eq41_e1302_d_n10 * p.p33);
        let eq41_e1304_d_n11: f64 = (eq41_e1302_d_n11 * p.p33);
        let eq41_e1304_d_n12: f64 = (eq41_e1302_d_n12 * p.p33);
        let eq41_e1306: f64 = (eq41_e1304 * s.v[840]);
        let eq41_e1306_d_n0: f64 = ((eq41_e1304_d_n0 * s.v[840]) + (eq41_e1304 * s.dn[840][0]));
        let eq41_e1306_d_n1: f64 = ((eq41_e1304_d_n1 * s.v[840]) + (eq41_e1304 * s.dn[840][1]));
        let eq41_e1306_d_n2: f64 = ((eq41_e1304_d_n2 * s.v[840]) + (eq41_e1304 * s.dn[840][2]));
        let eq41_e1306_d_n3: f64 = ((eq41_e1304_d_n3 * s.v[840]) + (eq41_e1304 * s.dn[840][3]));
        let eq41_e1306_d_n4: f64 = ((eq41_e1304_d_n4 * s.v[840]) + (eq41_e1304 * s.dn[840][4]));
        let eq41_e1306_d_n5: f64 = ((eq41_e1304_d_n5 * s.v[840]) + (eq41_e1304 * s.dn[840][5]));
        let eq41_e1306_d_n6: f64 = ((eq41_e1304_d_n6 * s.v[840]) + (eq41_e1304 * s.dn[840][6]));
        let eq41_e1306_d_n7: f64 = ((eq41_e1304_d_n7 * s.v[840]) + (eq41_e1304 * s.dn[840][7]));
        let eq41_e1306_d_n8: f64 = ((eq41_e1304_d_n8 * s.v[840]) + (eq41_e1304 * s.dn[840][8]));
        let eq41_e1306_d_n9: f64 = ((eq41_e1304_d_n9 * s.v[840]) + (eq41_e1304 * s.dn[840][9]));
        let eq41_e1306_d_n10: f64 = ((eq41_e1304_d_n10 * s.v[840]) + (eq41_e1304 * s.dn[840][10]));
        let eq41_e1306_d_n11: f64 = ((eq41_e1304_d_n11 * s.v[840]) + (eq41_e1304 * s.dn[840][11]));
        let eq41_e1306_d_n12: f64 = ((eq41_e1304_d_n12 * s.v[840]) + (eq41_e1304 * s.dn[840][12]));
        let eq41_e1307_q: f64 = eq41_e1306;
        let eq41_reactive_node_derivatives: [f64; 13] = [eq41_e1306_d_n0, eq41_e1306_d_n1, eq41_e1306_d_n2, eq41_e1306_d_n3, eq41_e1306_d_n4, eq41_e1306_d_n5, eq41_e1306_d_n6, eq41_e1306_d_n7, eq41_e1306_d_n8, eq41_e1306_d_n9, eq41_e1306_d_n10, eq41_e1306_d_n11, eq41_e1306_d_n12];
        let eq41_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            &nodes,
            &eq41_reactive_node_derivatives,
            &branches,
            &eq41_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
