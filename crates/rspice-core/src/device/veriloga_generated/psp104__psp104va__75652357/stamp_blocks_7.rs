#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq48_e1335: f64 = (s.v[854] * (nv4 - 0.0));
        let eq48_e1335_d_n0: f64 = (s.dn[854][0] * (nv4 - 0.0));
        let eq48_e1335_d_n1: f64 = (s.dn[854][1] * (nv4 - 0.0));
        let eq48_e1335_d_n2: f64 = (s.dn[854][2] * (nv4 - 0.0));
        let eq48_e1335_d_n3: f64 = (s.dn[854][3] * (nv4 - 0.0));
        let eq48_e1335_d_n4: f64 = ((s.dn[854][4] * (nv4 - 0.0)) + s.v[854]);
        let eq48_e1335_d_n5: f64 = (s.dn[854][5] * (nv4 - 0.0));
        let eq48_e1335_d_n6: f64 = (s.dn[854][6] * (nv4 - 0.0));
        let eq48_e1335_d_n7: f64 = (s.dn[854][7] * (nv4 - 0.0));
        let eq48_e1335_d_n8: f64 = (s.dn[854][8] * (nv4 - 0.0));
        let eq48_e1335_d_n9: f64 = (s.dn[854][9] * (nv4 - 0.0));
        let eq48_e1335_d_n10: f64 = (s.dn[854][10] * (nv4 - 0.0));
        let eq48_e1335_d_n11: f64 = (s.dn[854][11] * (nv4 - 0.0));
        let eq48_e1336: f64 = self.eval_ddt(8, eq48_e1335);
        let eq48_e1336_d_n0: f64 = self.ddt_jacobian(eq48_e1335_d_n0);
        let eq48_e1336_d_n1: f64 = self.ddt_jacobian(eq48_e1335_d_n1);
        let eq48_e1336_d_n2: f64 = self.ddt_jacobian(eq48_e1335_d_n2);
        let eq48_e1336_d_n3: f64 = self.ddt_jacobian(eq48_e1335_d_n3);
        let eq48_e1336_d_n4: f64 = self.ddt_jacobian(eq48_e1335_d_n4);
        let eq48_e1336_d_n5: f64 = self.ddt_jacobian(eq48_e1335_d_n5);
        let eq48_e1336_d_n6: f64 = self.ddt_jacobian(eq48_e1335_d_n6);
        let eq48_e1336_d_n7: f64 = self.ddt_jacobian(eq48_e1335_d_n7);
        let eq48_e1336_d_n8: f64 = self.ddt_jacobian(eq48_e1335_d_n8);
        let eq48_e1336_d_n9: f64 = self.ddt_jacobian(eq48_e1335_d_n9);
        let eq48_e1336_d_n10: f64 = self.ddt_jacobian(eq48_e1335_d_n10);
        let eq48_e1336_d_n11: f64 = self.ddt_jacobian(eq48_e1335_d_n11);
        let eq48_value: f64 = eq48_e1336;
        let eq48_node_derivatives: [f64; 12] = [eq48_e1336_d_n0, eq48_e1336_d_n1, eq48_e1336_d_n2, eq48_e1336_d_n3, eq48_e1336_d_n4, eq48_e1336_d_n5, eq48_e1336_d_n6, eq48_e1336_d_n7, eq48_e1336_d_n8, eq48_e1336_d_n9, eq48_e1336_d_n10, eq48_e1336_d_n11];
        let eq48_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq48_value),
            &nodes,
            &eq48_node_derivatives,
            &branches,
            &eq48_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_49_block_0(
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
        let eq49_e1339: f64 = (s.v[15] * p.p32);
        let eq49_e1339_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq49_e1339_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq49_e1339_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq49_e1339_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq49_e1339_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq49_e1339_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq49_e1339_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq49_e1339_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq49_e1339_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq49_e1339_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq49_e1339_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq49_e1339_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq49_e1340: f64 = (eq49_e1339).sqrt();
        let eq49_e1340_d_n0: f64 = (eq49_e1339_d_n0 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n1: f64 = (eq49_e1339_d_n1 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n2: f64 = (eq49_e1339_d_n2 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n3: f64 = (eq49_e1339_d_n3 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n4: f64 = (eq49_e1339_d_n4 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n5: f64 = (eq49_e1339_d_n5 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n6: f64 = (eq49_e1339_d_n6 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n7: f64 = (eq49_e1339_d_n7 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n8: f64 = (eq49_e1339_d_n8 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n9: f64 = (eq49_e1339_d_n9 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n10: f64 = (eq49_e1339_d_n10 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n11: f64 = (eq49_e1339_d_n11 / (2.0 * eq49_e1340));
        let eq49_e1342: f64 = (eq49_e1340 * 0.5);
        let eq49_e1342_d_n0: f64 = (eq49_e1340_d_n0 * 0.5);
        let eq49_e1342_d_n1: f64 = (eq49_e1340_d_n1 * 0.5);
        let eq49_e1342_d_n2: f64 = (eq49_e1340_d_n2 * 0.5);
        let eq49_e1342_d_n3: f64 = (eq49_e1340_d_n3 * 0.5);
        let eq49_e1342_d_n4: f64 = (eq49_e1340_d_n4 * 0.5);
        let eq49_e1342_d_n5: f64 = (eq49_e1340_d_n5 * 0.5);
        let eq49_e1342_d_n6: f64 = (eq49_e1340_d_n6 * 0.5);
        let eq49_e1342_d_n7: f64 = (eq49_e1340_d_n7 * 0.5);
        let eq49_e1342_d_n8: f64 = (eq49_e1340_d_n8 * 0.5);
        let eq49_e1342_d_n9: f64 = (eq49_e1340_d_n9 * 0.5);
        let eq49_e1342_d_n10: f64 = (eq49_e1340_d_n10 * 0.5);
        let eq49_e1342_d_n11: f64 = (eq49_e1340_d_n11 * 0.5);
        let eq49_e1344: f64 = (eq49_e1342 * s.v[854]);
        let eq49_e1344_d_n0: f64 = ((eq49_e1342_d_n0 * s.v[854]) + (eq49_e1342 * s.dn[854][0]));
        let eq49_e1344_d_n1: f64 = ((eq49_e1342_d_n1 * s.v[854]) + (eq49_e1342 * s.dn[854][1]));
        let eq49_e1344_d_n2: f64 = ((eq49_e1342_d_n2 * s.v[854]) + (eq49_e1342 * s.dn[854][2]));
        let eq49_e1344_d_n3: f64 = ((eq49_e1342_d_n3 * s.v[854]) + (eq49_e1342 * s.dn[854][3]));
        let eq49_e1344_d_n4: f64 = ((eq49_e1342_d_n4 * s.v[854]) + (eq49_e1342 * s.dn[854][4]));
        let eq49_e1344_d_n5: f64 = ((eq49_e1342_d_n5 * s.v[854]) + (eq49_e1342 * s.dn[854][5]));
        let eq49_e1344_d_n6: f64 = ((eq49_e1342_d_n6 * s.v[854]) + (eq49_e1342 * s.dn[854][6]));
        let eq49_e1344_d_n7: f64 = ((eq49_e1342_d_n7 * s.v[854]) + (eq49_e1342 * s.dn[854][7]));
        let eq49_e1344_d_n8: f64 = ((eq49_e1342_d_n8 * s.v[854]) + (eq49_e1342 * s.dn[854][8]));
        let eq49_e1344_d_n9: f64 = ((eq49_e1342_d_n9 * s.v[854]) + (eq49_e1342 * s.dn[854][9]));
        let eq49_e1344_d_n10: f64 = ((eq49_e1342_d_n10 * s.v[854]) + (eq49_e1342 * s.dn[854][10]));
        let eq49_e1344_d_n11: f64 = ((eq49_e1342_d_n11 * s.v[854]) + (eq49_e1342 * s.dn[854][11]));
        let eq49_e1346: f64 = (eq49_e1344 * (nv4 - 0.0));
        let eq49_e1346_d_n0: f64 = (eq49_e1344_d_n0 * (nv4 - 0.0));
        let eq49_e1346_d_n1: f64 = (eq49_e1344_d_n1 * (nv4 - 0.0));
        let eq49_e1346_d_n2: f64 = (eq49_e1344_d_n2 * (nv4 - 0.0));
        let eq49_e1346_d_n3: f64 = (eq49_e1344_d_n3 * (nv4 - 0.0));
        let eq49_e1346_d_n4: f64 = ((eq49_e1344_d_n4 * (nv4 - 0.0)) + eq49_e1344);
        let eq49_e1346_d_n5: f64 = (eq49_e1344_d_n5 * (nv4 - 0.0));
        let eq49_e1346_d_n6: f64 = (eq49_e1344_d_n6 * (nv4 - 0.0));
        let eq49_e1346_d_n7: f64 = (eq49_e1344_d_n7 * (nv4 - 0.0));
        let eq49_e1346_d_n8: f64 = (eq49_e1344_d_n8 * (nv4 - 0.0));
        let eq49_e1346_d_n9: f64 = (eq49_e1344_d_n9 * (nv4 - 0.0));
        let eq49_e1346_d_n10: f64 = (eq49_e1344_d_n10 * (nv4 - 0.0));
        let eq49_e1346_d_n11: f64 = (eq49_e1344_d_n11 * (nv4 - 0.0));
        let eq49_e1347: f64 = self.eval_ddt(9, eq49_e1346);
        let eq49_e1347_d_n0: f64 = self.ddt_jacobian(eq49_e1346_d_n0);
        let eq49_e1347_d_n1: f64 = self.ddt_jacobian(eq49_e1346_d_n1);
        let eq49_e1347_d_n2: f64 = self.ddt_jacobian(eq49_e1346_d_n2);
        let eq49_e1347_d_n3: f64 = self.ddt_jacobian(eq49_e1346_d_n3);
        let eq49_e1347_d_n4: f64 = self.ddt_jacobian(eq49_e1346_d_n4);
        let eq49_e1347_d_n5: f64 = self.ddt_jacobian(eq49_e1346_d_n5);
        let eq49_e1347_d_n6: f64 = self.ddt_jacobian(eq49_e1346_d_n6);
        let eq49_e1347_d_n7: f64 = self.ddt_jacobian(eq49_e1346_d_n7);
        let eq49_e1347_d_n8: f64 = self.ddt_jacobian(eq49_e1346_d_n8);
        let eq49_e1347_d_n9: f64 = self.ddt_jacobian(eq49_e1346_d_n9);
        let eq49_e1347_d_n10: f64 = self.ddt_jacobian(eq49_e1346_d_n10);
        let eq49_e1347_d_n11: f64 = self.ddt_jacobian(eq49_e1346_d_n11);
        let eq49_e1348: f64 = (-eq49_e1347);
        let eq49_e1348_d_n0: f64 = (-eq49_e1347_d_n0);
        let eq49_e1348_d_n1: f64 = (-eq49_e1347_d_n1);
        let eq49_e1348_d_n2: f64 = (-eq49_e1347_d_n2);
        let eq49_e1348_d_n3: f64 = (-eq49_e1347_d_n3);
        let eq49_e1348_d_n4: f64 = (-eq49_e1347_d_n4);
        let eq49_e1348_d_n5: f64 = (-eq49_e1347_d_n5);
        let eq49_e1348_d_n6: f64 = (-eq49_e1347_d_n6);
        let eq49_e1348_d_n7: f64 = (-eq49_e1347_d_n7);
        let eq49_e1348_d_n8: f64 = (-eq49_e1347_d_n8);
        let eq49_e1348_d_n9: f64 = (-eq49_e1347_d_n9);
        let eq49_e1348_d_n10: f64 = (-eq49_e1347_d_n10);
        let eq49_e1348_d_n11: f64 = (-eq49_e1347_d_n11);
        let eq49_value: f64 = eq49_e1348;
        let eq49_node_derivatives: [f64; 12] = [eq49_e1348_d_n0, eq49_e1348_d_n1, eq49_e1348_d_n2, eq49_e1348_d_n3, eq49_e1348_d_n4, eq49_e1348_d_n5, eq49_e1348_d_n6, eq49_e1348_d_n7, eq49_e1348_d_n8, eq49_e1348_d_n9, eq49_e1348_d_n10, eq49_e1348_d_n11];
        let eq49_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq49_value),
            &nodes,
            &eq49_node_derivatives,
            &branches,
            &eq49_branch_derivatives,
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq50_e1351: f64 = (s.v[15] * p.p32);
        let eq50_e1351_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq50_e1351_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq50_e1351_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq50_e1351_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq50_e1351_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq50_e1351_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq50_e1351_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq50_e1351_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq50_e1351_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq50_e1351_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq50_e1351_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq50_e1351_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq50_e1352: f64 = (eq50_e1351).sqrt();
        let eq50_e1352_d_n0: f64 = (eq50_e1351_d_n0 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n1: f64 = (eq50_e1351_d_n1 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n2: f64 = (eq50_e1351_d_n2 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n3: f64 = (eq50_e1351_d_n3 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n4: f64 = (eq50_e1351_d_n4 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n5: f64 = (eq50_e1351_d_n5 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n6: f64 = (eq50_e1351_d_n6 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n7: f64 = (eq50_e1351_d_n7 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n8: f64 = (eq50_e1351_d_n8 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n9: f64 = (eq50_e1351_d_n9 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n10: f64 = (eq50_e1351_d_n10 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n11: f64 = (eq50_e1351_d_n11 / (2.0 * eq50_e1352));
        let eq50_e1354: f64 = (eq50_e1352 * 0.5);
        let eq50_e1354_d_n0: f64 = (eq50_e1352_d_n0 * 0.5);
        let eq50_e1354_d_n1: f64 = (eq50_e1352_d_n1 * 0.5);
        let eq50_e1354_d_n2: f64 = (eq50_e1352_d_n2 * 0.5);
        let eq50_e1354_d_n3: f64 = (eq50_e1352_d_n3 * 0.5);
        let eq50_e1354_d_n4: f64 = (eq50_e1352_d_n4 * 0.5);
        let eq50_e1354_d_n5: f64 = (eq50_e1352_d_n5 * 0.5);
        let eq50_e1354_d_n6: f64 = (eq50_e1352_d_n6 * 0.5);
        let eq50_e1354_d_n7: f64 = (eq50_e1352_d_n7 * 0.5);
        let eq50_e1354_d_n8: f64 = (eq50_e1352_d_n8 * 0.5);
        let eq50_e1354_d_n9: f64 = (eq50_e1352_d_n9 * 0.5);
        let eq50_e1354_d_n10: f64 = (eq50_e1352_d_n10 * 0.5);
        let eq50_e1354_d_n11: f64 = (eq50_e1352_d_n11 * 0.5);
        let eq50_e1356: f64 = (eq50_e1354 * s.v[854]);
        let eq50_e1356_d_n0: f64 = ((eq50_e1354_d_n0 * s.v[854]) + (eq50_e1354 * s.dn[854][0]));
        let eq50_e1356_d_n1: f64 = ((eq50_e1354_d_n1 * s.v[854]) + (eq50_e1354 * s.dn[854][1]));
        let eq50_e1356_d_n2: f64 = ((eq50_e1354_d_n2 * s.v[854]) + (eq50_e1354 * s.dn[854][2]));
        let eq50_e1356_d_n3: f64 = ((eq50_e1354_d_n3 * s.v[854]) + (eq50_e1354 * s.dn[854][3]));
        let eq50_e1356_d_n4: f64 = ((eq50_e1354_d_n4 * s.v[854]) + (eq50_e1354 * s.dn[854][4]));
        let eq50_e1356_d_n5: f64 = ((eq50_e1354_d_n5 * s.v[854]) + (eq50_e1354 * s.dn[854][5]));
        let eq50_e1356_d_n6: f64 = ((eq50_e1354_d_n6 * s.v[854]) + (eq50_e1354 * s.dn[854][6]));
        let eq50_e1356_d_n7: f64 = ((eq50_e1354_d_n7 * s.v[854]) + (eq50_e1354 * s.dn[854][7]));
        let eq50_e1356_d_n8: f64 = ((eq50_e1354_d_n8 * s.v[854]) + (eq50_e1354 * s.dn[854][8]));
        let eq50_e1356_d_n9: f64 = ((eq50_e1354_d_n9 * s.v[854]) + (eq50_e1354 * s.dn[854][9]));
        let eq50_e1356_d_n10: f64 = ((eq50_e1354_d_n10 * s.v[854]) + (eq50_e1354 * s.dn[854][10]));
        let eq50_e1356_d_n11: f64 = ((eq50_e1354_d_n11 * s.v[854]) + (eq50_e1354 * s.dn[854][11]));
        let eq50_e1358: f64 = (eq50_e1356 * (nv4 - 0.0));
        let eq50_e1358_d_n0: f64 = (eq50_e1356_d_n0 * (nv4 - 0.0));
        let eq50_e1358_d_n1: f64 = (eq50_e1356_d_n1 * (nv4 - 0.0));
        let eq50_e1358_d_n2: f64 = (eq50_e1356_d_n2 * (nv4 - 0.0));
        let eq50_e1358_d_n3: f64 = (eq50_e1356_d_n3 * (nv4 - 0.0));
        let eq50_e1358_d_n4: f64 = ((eq50_e1356_d_n4 * (nv4 - 0.0)) + eq50_e1356);
        let eq50_e1358_d_n5: f64 = (eq50_e1356_d_n5 * (nv4 - 0.0));
        let eq50_e1358_d_n6: f64 = (eq50_e1356_d_n6 * (nv4 - 0.0));
        let eq50_e1358_d_n7: f64 = (eq50_e1356_d_n7 * (nv4 - 0.0));
        let eq50_e1358_d_n8: f64 = (eq50_e1356_d_n8 * (nv4 - 0.0));
        let eq50_e1358_d_n9: f64 = (eq50_e1356_d_n9 * (nv4 - 0.0));
        let eq50_e1358_d_n10: f64 = (eq50_e1356_d_n10 * (nv4 - 0.0));
        let eq50_e1358_d_n11: f64 = (eq50_e1356_d_n11 * (nv4 - 0.0));
        let eq50_e1359: f64 = self.eval_ddt(10, eq50_e1358);
        let eq50_e1359_d_n0: f64 = self.ddt_jacobian(eq50_e1358_d_n0);
        let eq50_e1359_d_n1: f64 = self.ddt_jacobian(eq50_e1358_d_n1);
        let eq50_e1359_d_n2: f64 = self.ddt_jacobian(eq50_e1358_d_n2);
        let eq50_e1359_d_n3: f64 = self.ddt_jacobian(eq50_e1358_d_n3);
        let eq50_e1359_d_n4: f64 = self.ddt_jacobian(eq50_e1358_d_n4);
        let eq50_e1359_d_n5: f64 = self.ddt_jacobian(eq50_e1358_d_n5);
        let eq50_e1359_d_n6: f64 = self.ddt_jacobian(eq50_e1358_d_n6);
        let eq50_e1359_d_n7: f64 = self.ddt_jacobian(eq50_e1358_d_n7);
        let eq50_e1359_d_n8: f64 = self.ddt_jacobian(eq50_e1358_d_n8);
        let eq50_e1359_d_n9: f64 = self.ddt_jacobian(eq50_e1358_d_n9);
        let eq50_e1359_d_n10: f64 = self.ddt_jacobian(eq50_e1358_d_n10);
        let eq50_e1359_d_n11: f64 = self.ddt_jacobian(eq50_e1358_d_n11);
        let eq50_e1360: f64 = (-eq50_e1359);
        let eq50_e1360_d_n0: f64 = (-eq50_e1359_d_n0);
        let eq50_e1360_d_n1: f64 = (-eq50_e1359_d_n1);
        let eq50_e1360_d_n2: f64 = (-eq50_e1359_d_n2);
        let eq50_e1360_d_n3: f64 = (-eq50_e1359_d_n3);
        let eq50_e1360_d_n4: f64 = (-eq50_e1359_d_n4);
        let eq50_e1360_d_n5: f64 = (-eq50_e1359_d_n5);
        let eq50_e1360_d_n6: f64 = (-eq50_e1359_d_n6);
        let eq50_e1360_d_n7: f64 = (-eq50_e1359_d_n7);
        let eq50_e1360_d_n8: f64 = (-eq50_e1359_d_n8);
        let eq50_e1360_d_n9: f64 = (-eq50_e1359_d_n9);
        let eq50_e1360_d_n10: f64 = (-eq50_e1359_d_n10);
        let eq50_e1360_d_n11: f64 = (-eq50_e1359_d_n11);
        let eq50_value: f64 = eq50_e1360;
        let eq50_node_derivatives: [f64; 12] = [eq50_e1360_d_n0, eq50_e1360_d_n1, eq50_e1360_d_n2, eq50_e1360_d_n3, eq50_e1360_d_n4, eq50_e1360_d_n5, eq50_e1360_d_n6, eq50_e1360_d_n7, eq50_e1360_d_n8, eq50_e1360_d_n9, eq50_e1360_d_n10, eq50_e1360_d_n11];
        let eq50_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            self.multiplicity * (eq50_value),
            &nodes,
            &eq50_node_derivatives,
            &branches,
            &eq50_branch_derivatives,
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
        let eq52_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[6]),
            self.multiplicity * (eq52_value),
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
        let eq53_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[6]),
            self.multiplicity * (eq53_value),
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
        let eq54_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq54_value),
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
        let eq55_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[5]),
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
            Some(nodes[10]),
            Some(nodes[6]),
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
            Some(nodes[11]),
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
            Some(nodes[7]),
            Some(nodes[6]),
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
            Some(nodes[7]),
            Some(nodes[6]),
            self.multiplicity * (eq59_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_38_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq38_e1263: f64 = (s.v[0] * s.v[15]);
        let eq38_e1263_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq38_e1263_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq38_e1263_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq38_e1263_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq38_e1263_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq38_e1263_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq38_e1263_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq38_e1263_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq38_e1263_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq38_e1263_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq38_e1263_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq38_e1263_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq38_e1265: f64 = (eq38_e1263 * p.p33);
        let eq38_e1265_d_n0: f64 = (eq38_e1263_d_n0 * p.p33);
        let eq38_e1265_d_n1: f64 = (eq38_e1263_d_n1 * p.p33);
        let eq38_e1265_d_n2: f64 = (eq38_e1263_d_n2 * p.p33);
        let eq38_e1265_d_n3: f64 = (eq38_e1263_d_n3 * p.p33);
        let eq38_e1265_d_n4: f64 = (eq38_e1263_d_n4 * p.p33);
        let eq38_e1265_d_n5: f64 = (eq38_e1263_d_n5 * p.p33);
        let eq38_e1265_d_n6: f64 = (eq38_e1263_d_n6 * p.p33);
        let eq38_e1265_d_n7: f64 = (eq38_e1263_d_n7 * p.p33);
        let eq38_e1265_d_n8: f64 = (eq38_e1263_d_n8 * p.p33);
        let eq38_e1265_d_n9: f64 = (eq38_e1263_d_n9 * p.p33);
        let eq38_e1265_d_n10: f64 = (eq38_e1263_d_n10 * p.p33);
        let eq38_e1265_d_n11: f64 = (eq38_e1263_d_n11 * p.p33);
        let eq38_e1267: f64 = (eq38_e1265 * s.v[845]);
        let eq38_e1267_d_n0: f64 = ((eq38_e1265_d_n0 * s.v[845]) + (eq38_e1265 * s.dn[845][0]));
        let eq38_e1267_d_n1: f64 = ((eq38_e1265_d_n1 * s.v[845]) + (eq38_e1265 * s.dn[845][1]));
        let eq38_e1267_d_n2: f64 = ((eq38_e1265_d_n2 * s.v[845]) + (eq38_e1265 * s.dn[845][2]));
        let eq38_e1267_d_n3: f64 = ((eq38_e1265_d_n3 * s.v[845]) + (eq38_e1265 * s.dn[845][3]));
        let eq38_e1267_d_n4: f64 = ((eq38_e1265_d_n4 * s.v[845]) + (eq38_e1265 * s.dn[845][4]));
        let eq38_e1267_d_n5: f64 = ((eq38_e1265_d_n5 * s.v[845]) + (eq38_e1265 * s.dn[845][5]));
        let eq38_e1267_d_n6: f64 = ((eq38_e1265_d_n6 * s.v[845]) + (eq38_e1265 * s.dn[845][6]));
        let eq38_e1267_d_n7: f64 = ((eq38_e1265_d_n7 * s.v[845]) + (eq38_e1265 * s.dn[845][7]));
        let eq38_e1267_d_n8: f64 = ((eq38_e1265_d_n8 * s.v[845]) + (eq38_e1265 * s.dn[845][8]));
        let eq38_e1267_d_n9: f64 = ((eq38_e1265_d_n9 * s.v[845]) + (eq38_e1265 * s.dn[845][9]));
        let eq38_e1267_d_n10: f64 = ((eq38_e1265_d_n10 * s.v[845]) + (eq38_e1265 * s.dn[845][10]));
        let eq38_e1267_d_n11: f64 = ((eq38_e1265_d_n11 * s.v[845]) + (eq38_e1265 * s.dn[845][11]));
        let eq38_e1268_q: f64 = eq38_e1267;
        let eq38_reactive_node_derivatives: [f64; 12] = [eq38_e1267_d_n0, eq38_e1267_d_n1, eq38_e1267_d_n2, eq38_e1267_d_n3, eq38_e1267_d_n4, eq38_e1267_d_n5, eq38_e1267_d_n6, eq38_e1267_d_n7, eq38_e1267_d_n8, eq38_e1267_d_n9, eq38_e1267_d_n10, eq38_e1267_d_n11];
        let eq38_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            &nodes,
            &eq38_reactive_node_derivatives,
            &branches,
            &eq38_reactive_branch_derivatives,
            self.multiplicity,
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
        let eq39_e1271: f64 = (s.v[0] * s.v[15]);
        let eq39_e1271_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq39_e1271_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq39_e1271_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq39_e1271_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq39_e1271_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq39_e1271_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq39_e1271_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq39_e1271_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq39_e1271_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq39_e1271_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq39_e1271_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq39_e1271_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq39_e1273: f64 = (eq39_e1271 * p.p33);
        let eq39_e1273_d_n0: f64 = (eq39_e1271_d_n0 * p.p33);
        let eq39_e1273_d_n1: f64 = (eq39_e1271_d_n1 * p.p33);
        let eq39_e1273_d_n2: f64 = (eq39_e1271_d_n2 * p.p33);
        let eq39_e1273_d_n3: f64 = (eq39_e1271_d_n3 * p.p33);
        let eq39_e1273_d_n4: f64 = (eq39_e1271_d_n4 * p.p33);
        let eq39_e1273_d_n5: f64 = (eq39_e1271_d_n5 * p.p33);
        let eq39_e1273_d_n6: f64 = (eq39_e1271_d_n6 * p.p33);
        let eq39_e1273_d_n7: f64 = (eq39_e1271_d_n7 * p.p33);
        let eq39_e1273_d_n8: f64 = (eq39_e1271_d_n8 * p.p33);
        let eq39_e1273_d_n9: f64 = (eq39_e1271_d_n9 * p.p33);
        let eq39_e1273_d_n10: f64 = (eq39_e1271_d_n10 * p.p33);
        let eq39_e1273_d_n11: f64 = (eq39_e1271_d_n11 * p.p33);
        let eq39_e1275: f64 = (eq39_e1273 * s.v[846]);
        let eq39_e1275_d_n0: f64 = ((eq39_e1273_d_n0 * s.v[846]) + (eq39_e1273 * s.dn[846][0]));
        let eq39_e1275_d_n1: f64 = ((eq39_e1273_d_n1 * s.v[846]) + (eq39_e1273 * s.dn[846][1]));
        let eq39_e1275_d_n2: f64 = ((eq39_e1273_d_n2 * s.v[846]) + (eq39_e1273 * s.dn[846][2]));
        let eq39_e1275_d_n3: f64 = ((eq39_e1273_d_n3 * s.v[846]) + (eq39_e1273 * s.dn[846][3]));
        let eq39_e1275_d_n4: f64 = ((eq39_e1273_d_n4 * s.v[846]) + (eq39_e1273 * s.dn[846][4]));
        let eq39_e1275_d_n5: f64 = ((eq39_e1273_d_n5 * s.v[846]) + (eq39_e1273 * s.dn[846][5]));
        let eq39_e1275_d_n6: f64 = ((eq39_e1273_d_n6 * s.v[846]) + (eq39_e1273 * s.dn[846][6]));
        let eq39_e1275_d_n7: f64 = ((eq39_e1273_d_n7 * s.v[846]) + (eq39_e1273 * s.dn[846][7]));
        let eq39_e1275_d_n8: f64 = ((eq39_e1273_d_n8 * s.v[846]) + (eq39_e1273 * s.dn[846][8]));
        let eq39_e1275_d_n9: f64 = ((eq39_e1273_d_n9 * s.v[846]) + (eq39_e1273 * s.dn[846][9]));
        let eq39_e1275_d_n10: f64 = ((eq39_e1273_d_n10 * s.v[846]) + (eq39_e1273 * s.dn[846][10]));
        let eq39_e1275_d_n11: f64 = ((eq39_e1273_d_n11 * s.v[846]) + (eq39_e1273 * s.dn[846][11]));
        let eq39_e1276_q: f64 = eq39_e1275;
        let eq39_reactive_node_derivatives: [f64; 12] = [eq39_e1275_d_n0, eq39_e1275_d_n1, eq39_e1275_d_n2, eq39_e1275_d_n3, eq39_e1275_d_n4, eq39_e1275_d_n5, eq39_e1275_d_n6, eq39_e1275_d_n7, eq39_e1275_d_n8, eq39_e1275_d_n9, eq39_e1275_d_n10, eq39_e1275_d_n11];
        let eq39_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            &nodes,
            &eq39_reactive_node_derivatives,
            &branches,
            &eq39_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_40_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq40_e1279: f64 = (s.v[0] * s.v[15]);
        let eq40_e1279_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq40_e1279_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq40_e1279_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq40_e1279_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq40_e1279_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq40_e1279_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq40_e1279_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq40_e1279_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq40_e1279_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq40_e1279_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq40_e1279_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq40_e1279_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq40_e1281: f64 = (eq40_e1279 * p.p33);
        let eq40_e1281_d_n0: f64 = (eq40_e1279_d_n0 * p.p33);
        let eq40_e1281_d_n1: f64 = (eq40_e1279_d_n1 * p.p33);
        let eq40_e1281_d_n2: f64 = (eq40_e1279_d_n2 * p.p33);
        let eq40_e1281_d_n3: f64 = (eq40_e1279_d_n3 * p.p33);
        let eq40_e1281_d_n4: f64 = (eq40_e1279_d_n4 * p.p33);
        let eq40_e1281_d_n5: f64 = (eq40_e1279_d_n5 * p.p33);
        let eq40_e1281_d_n6: f64 = (eq40_e1279_d_n6 * p.p33);
        let eq40_e1281_d_n7: f64 = (eq40_e1279_d_n7 * p.p33);
        let eq40_e1281_d_n8: f64 = (eq40_e1279_d_n8 * p.p33);
        let eq40_e1281_d_n9: f64 = (eq40_e1279_d_n9 * p.p33);
        let eq40_e1281_d_n10: f64 = (eq40_e1279_d_n10 * p.p33);
        let eq40_e1281_d_n11: f64 = (eq40_e1279_d_n11 * p.p33);
        let eq40_e1283: f64 = (eq40_e1281 * s.v[847]);
        let eq40_e1283_d_n0: f64 = ((eq40_e1281_d_n0 * s.v[847]) + (eq40_e1281 * s.dn[847][0]));
        let eq40_e1283_d_n1: f64 = ((eq40_e1281_d_n1 * s.v[847]) + (eq40_e1281 * s.dn[847][1]));
        let eq40_e1283_d_n2: f64 = ((eq40_e1281_d_n2 * s.v[847]) + (eq40_e1281 * s.dn[847][2]));
        let eq40_e1283_d_n3: f64 = ((eq40_e1281_d_n3 * s.v[847]) + (eq40_e1281 * s.dn[847][3]));
        let eq40_e1283_d_n4: f64 = ((eq40_e1281_d_n4 * s.v[847]) + (eq40_e1281 * s.dn[847][4]));
        let eq40_e1283_d_n5: f64 = ((eq40_e1281_d_n5 * s.v[847]) + (eq40_e1281 * s.dn[847][5]));
        let eq40_e1283_d_n6: f64 = ((eq40_e1281_d_n6 * s.v[847]) + (eq40_e1281 * s.dn[847][6]));
        let eq40_e1283_d_n7: f64 = ((eq40_e1281_d_n7 * s.v[847]) + (eq40_e1281 * s.dn[847][7]));
        let eq40_e1283_d_n8: f64 = ((eq40_e1281_d_n8 * s.v[847]) + (eq40_e1281 * s.dn[847][8]));
        let eq40_e1283_d_n9: f64 = ((eq40_e1281_d_n9 * s.v[847]) + (eq40_e1281 * s.dn[847][9]));
        let eq40_e1283_d_n10: f64 = ((eq40_e1281_d_n10 * s.v[847]) + (eq40_e1281 * s.dn[847][10]));
        let eq40_e1283_d_n11: f64 = ((eq40_e1281_d_n11 * s.v[847]) + (eq40_e1281 * s.dn[847][11]));
        let eq40_e1284_q: f64 = eq40_e1283;
        let eq40_reactive_node_derivatives: [f64; 12] = [eq40_e1283_d_n0, eq40_e1283_d_n1, eq40_e1283_d_n2, eq40_e1283_d_n3, eq40_e1283_d_n4, eq40_e1283_d_n5, eq40_e1283_d_n6, eq40_e1283_d_n7, eq40_e1283_d_n8, eq40_e1283_d_n9, eq40_e1283_d_n10, eq40_e1283_d_n11];
        let eq40_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            &nodes,
            &eq40_reactive_node_derivatives,
            &branches,
            &eq40_reactive_branch_derivatives,
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
        let eq41_e1287: f64 = (s.v[0] * s.v[15]);
        let eq41_e1287_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq41_e1287_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq41_e1287_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq41_e1287_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq41_e1287_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq41_e1287_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq41_e1287_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq41_e1287_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq41_e1287_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq41_e1287_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq41_e1287_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq41_e1287_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq41_e1289: f64 = (eq41_e1287 * p.p33);
        let eq41_e1289_d_n0: f64 = (eq41_e1287_d_n0 * p.p33);
        let eq41_e1289_d_n1: f64 = (eq41_e1287_d_n1 * p.p33);
        let eq41_e1289_d_n2: f64 = (eq41_e1287_d_n2 * p.p33);
        let eq41_e1289_d_n3: f64 = (eq41_e1287_d_n3 * p.p33);
        let eq41_e1289_d_n4: f64 = (eq41_e1287_d_n4 * p.p33);
        let eq41_e1289_d_n5: f64 = (eq41_e1287_d_n5 * p.p33);
        let eq41_e1289_d_n6: f64 = (eq41_e1287_d_n6 * p.p33);
        let eq41_e1289_d_n7: f64 = (eq41_e1287_d_n7 * p.p33);
        let eq41_e1289_d_n8: f64 = (eq41_e1287_d_n8 * p.p33);
        let eq41_e1289_d_n9: f64 = (eq41_e1287_d_n9 * p.p33);
        let eq41_e1289_d_n10: f64 = (eq41_e1287_d_n10 * p.p33);
        let eq41_e1289_d_n11: f64 = (eq41_e1287_d_n11 * p.p33);
        let eq41_e1291: f64 = (eq41_e1289 * s.v[848]);
        let eq41_e1291_d_n0: f64 = ((eq41_e1289_d_n0 * s.v[848]) + (eq41_e1289 * s.dn[848][0]));
        let eq41_e1291_d_n1: f64 = ((eq41_e1289_d_n1 * s.v[848]) + (eq41_e1289 * s.dn[848][1]));
        let eq41_e1291_d_n2: f64 = ((eq41_e1289_d_n2 * s.v[848]) + (eq41_e1289 * s.dn[848][2]));
        let eq41_e1291_d_n3: f64 = ((eq41_e1289_d_n3 * s.v[848]) + (eq41_e1289 * s.dn[848][3]));
        let eq41_e1291_d_n4: f64 = ((eq41_e1289_d_n4 * s.v[848]) + (eq41_e1289 * s.dn[848][4]));
        let eq41_e1291_d_n5: f64 = ((eq41_e1289_d_n5 * s.v[848]) + (eq41_e1289 * s.dn[848][5]));
        let eq41_e1291_d_n6: f64 = ((eq41_e1289_d_n6 * s.v[848]) + (eq41_e1289 * s.dn[848][6]));
        let eq41_e1291_d_n7: f64 = ((eq41_e1289_d_n7 * s.v[848]) + (eq41_e1289 * s.dn[848][7]));
        let eq41_e1291_d_n8: f64 = ((eq41_e1289_d_n8 * s.v[848]) + (eq41_e1289 * s.dn[848][8]));
        let eq41_e1291_d_n9: f64 = ((eq41_e1289_d_n9 * s.v[848]) + (eq41_e1289 * s.dn[848][9]));
        let eq41_e1291_d_n10: f64 = ((eq41_e1289_d_n10 * s.v[848]) + (eq41_e1289 * s.dn[848][10]));
        let eq41_e1291_d_n11: f64 = ((eq41_e1289_d_n11 * s.v[848]) + (eq41_e1289 * s.dn[848][11]));
        let eq41_e1292_q: f64 = eq41_e1291;
        let eq41_reactive_node_derivatives: [f64; 12] = [eq41_e1291_d_n0, eq41_e1291_d_n1, eq41_e1291_d_n2, eq41_e1291_d_n3, eq41_e1291_d_n4, eq41_e1291_d_n5, eq41_e1291_d_n6, eq41_e1291_d_n7, eq41_e1291_d_n8, eq41_e1291_d_n9, eq41_e1291_d_n10, eq41_e1291_d_n11];
        let eq41_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            &nodes,
            &eq41_reactive_node_derivatives,
            &branches,
            &eq41_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

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
        let eq42_e1295: f64 = (s.v[0] * s.v[15]);
        let eq42_e1295_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq42_e1295_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq42_e1295_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq42_e1295_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq42_e1295_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq42_e1295_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq42_e1295_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq42_e1295_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq42_e1295_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq42_e1295_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq42_e1295_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq42_e1295_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq42_e1297: f64 = (eq42_e1295 * p.p33);
        let eq42_e1297_d_n0: f64 = (eq42_e1295_d_n0 * p.p33);
        let eq42_e1297_d_n1: f64 = (eq42_e1295_d_n1 * p.p33);
        let eq42_e1297_d_n2: f64 = (eq42_e1295_d_n2 * p.p33);
        let eq42_e1297_d_n3: f64 = (eq42_e1295_d_n3 * p.p33);
        let eq42_e1297_d_n4: f64 = (eq42_e1295_d_n4 * p.p33);
        let eq42_e1297_d_n5: f64 = (eq42_e1295_d_n5 * p.p33);
        let eq42_e1297_d_n6: f64 = (eq42_e1295_d_n6 * p.p33);
        let eq42_e1297_d_n7: f64 = (eq42_e1295_d_n7 * p.p33);
        let eq42_e1297_d_n8: f64 = (eq42_e1295_d_n8 * p.p33);
        let eq42_e1297_d_n9: f64 = (eq42_e1295_d_n9 * p.p33);
        let eq42_e1297_d_n10: f64 = (eq42_e1295_d_n10 * p.p33);
        let eq42_e1297_d_n11: f64 = (eq42_e1295_d_n11 * p.p33);
        let eq42_e1299: f64 = (eq42_e1297 * s.v[849]);
        let eq42_e1299_d_n0: f64 = ((eq42_e1297_d_n0 * s.v[849]) + (eq42_e1297 * s.dn[849][0]));
        let eq42_e1299_d_n1: f64 = ((eq42_e1297_d_n1 * s.v[849]) + (eq42_e1297 * s.dn[849][1]));
        let eq42_e1299_d_n2: f64 = ((eq42_e1297_d_n2 * s.v[849]) + (eq42_e1297 * s.dn[849][2]));
        let eq42_e1299_d_n3: f64 = ((eq42_e1297_d_n3 * s.v[849]) + (eq42_e1297 * s.dn[849][3]));
        let eq42_e1299_d_n4: f64 = ((eq42_e1297_d_n4 * s.v[849]) + (eq42_e1297 * s.dn[849][4]));
        let eq42_e1299_d_n5: f64 = ((eq42_e1297_d_n5 * s.v[849]) + (eq42_e1297 * s.dn[849][5]));
        let eq42_e1299_d_n6: f64 = ((eq42_e1297_d_n6 * s.v[849]) + (eq42_e1297 * s.dn[849][6]));
        let eq42_e1299_d_n7: f64 = ((eq42_e1297_d_n7 * s.v[849]) + (eq42_e1297 * s.dn[849][7]));
        let eq42_e1299_d_n8: f64 = ((eq42_e1297_d_n8 * s.v[849]) + (eq42_e1297 * s.dn[849][8]));
        let eq42_e1299_d_n9: f64 = ((eq42_e1297_d_n9 * s.v[849]) + (eq42_e1297 * s.dn[849][9]));
        let eq42_e1299_d_n10: f64 = ((eq42_e1297_d_n10 * s.v[849]) + (eq42_e1297 * s.dn[849][10]));
        let eq42_e1299_d_n11: f64 = ((eq42_e1297_d_n11 * s.v[849]) + (eq42_e1297 * s.dn[849][11]));
        let eq42_e1300_q: f64 = eq42_e1299;
        let eq42_reactive_node_derivatives: [f64; 12] = [eq42_e1299_d_n0, eq42_e1299_d_n1, eq42_e1299_d_n2, eq42_e1299_d_n3, eq42_e1299_d_n4, eq42_e1299_d_n5, eq42_e1299_d_n6, eq42_e1299_d_n7, eq42_e1299_d_n8, eq42_e1299_d_n9, eq42_e1299_d_n10, eq42_e1299_d_n11];
        let eq42_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            &nodes,
            &eq42_reactive_node_derivatives,
            &branches,
            &eq42_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
