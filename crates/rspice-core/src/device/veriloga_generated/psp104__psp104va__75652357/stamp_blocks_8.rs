#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let eq43_e1303: f64 = (s.v[0] * s.v[15]);
        let eq43_e1303_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq43_e1303_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq43_e1303_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq43_e1303_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq43_e1303_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq43_e1303_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq43_e1303_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq43_e1303_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq43_e1303_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq43_e1303_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq43_e1303_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq43_e1303_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq43_e1305: f64 = (eq43_e1303 * p.p33);
        let eq43_e1305_d_n0: f64 = (eq43_e1303_d_n0 * p.p33);
        let eq43_e1305_d_n1: f64 = (eq43_e1303_d_n1 * p.p33);
        let eq43_e1305_d_n2: f64 = (eq43_e1303_d_n2 * p.p33);
        let eq43_e1305_d_n3: f64 = (eq43_e1303_d_n3 * p.p33);
        let eq43_e1305_d_n4: f64 = (eq43_e1303_d_n4 * p.p33);
        let eq43_e1305_d_n5: f64 = (eq43_e1303_d_n5 * p.p33);
        let eq43_e1305_d_n6: f64 = (eq43_e1303_d_n6 * p.p33);
        let eq43_e1305_d_n7: f64 = (eq43_e1303_d_n7 * p.p33);
        let eq43_e1305_d_n8: f64 = (eq43_e1303_d_n8 * p.p33);
        let eq43_e1305_d_n9: f64 = (eq43_e1303_d_n9 * p.p33);
        let eq43_e1305_d_n10: f64 = (eq43_e1303_d_n10 * p.p33);
        let eq43_e1305_d_n11: f64 = (eq43_e1303_d_n11 * p.p33);
        let eq43_e1307: f64 = (eq43_e1305 * s.v[850]);
        let eq43_e1307_d_n0: f64 = ((eq43_e1305_d_n0 * s.v[850]) + (eq43_e1305 * s.dn[850][0]));
        let eq43_e1307_d_n1: f64 = ((eq43_e1305_d_n1 * s.v[850]) + (eq43_e1305 * s.dn[850][1]));
        let eq43_e1307_d_n2: f64 = ((eq43_e1305_d_n2 * s.v[850]) + (eq43_e1305 * s.dn[850][2]));
        let eq43_e1307_d_n3: f64 = ((eq43_e1305_d_n3 * s.v[850]) + (eq43_e1305 * s.dn[850][3]));
        let eq43_e1307_d_n4: f64 = ((eq43_e1305_d_n4 * s.v[850]) + (eq43_e1305 * s.dn[850][4]));
        let eq43_e1307_d_n5: f64 = ((eq43_e1305_d_n5 * s.v[850]) + (eq43_e1305 * s.dn[850][5]));
        let eq43_e1307_d_n6: f64 = ((eq43_e1305_d_n6 * s.v[850]) + (eq43_e1305 * s.dn[850][6]));
        let eq43_e1307_d_n7: f64 = ((eq43_e1305_d_n7 * s.v[850]) + (eq43_e1305 * s.dn[850][7]));
        let eq43_e1307_d_n8: f64 = ((eq43_e1305_d_n8 * s.v[850]) + (eq43_e1305 * s.dn[850][8]));
        let eq43_e1307_d_n9: f64 = ((eq43_e1305_d_n9 * s.v[850]) + (eq43_e1305 * s.dn[850][9]));
        let eq43_e1307_d_n10: f64 = ((eq43_e1305_d_n10 * s.v[850]) + (eq43_e1305 * s.dn[850][10]));
        let eq43_e1307_d_n11: f64 = ((eq43_e1305_d_n11 * s.v[850]) + (eq43_e1305 * s.dn[850][11]));
        let eq43_e1308_q: f64 = eq43_e1307;
        let eq43_reactive_node_derivatives: [f64; 12] = [eq43_e1307_d_n0, eq43_e1307_d_n1, eq43_e1307_d_n2, eq43_e1307_d_n3, eq43_e1307_d_n4, eq43_e1307_d_n5, eq43_e1307_d_n6, eq43_e1307_d_n7, eq43_e1307_d_n8, eq43_e1307_d_n9, eq43_e1307_d_n10, eq43_e1307_d_n11];
        let eq43_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[8]),
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
        let eq44_e1311: f64 = (s.v[0] * s.v[15]);
        let eq44_e1311_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq44_e1311_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq44_e1311_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq44_e1311_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq44_e1311_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq44_e1311_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq44_e1311_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq44_e1311_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq44_e1311_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq44_e1311_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq44_e1311_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq44_e1311_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq44_e1313: f64 = (eq44_e1311 * p.p33);
        let eq44_e1313_d_n0: f64 = (eq44_e1311_d_n0 * p.p33);
        let eq44_e1313_d_n1: f64 = (eq44_e1311_d_n1 * p.p33);
        let eq44_e1313_d_n2: f64 = (eq44_e1311_d_n2 * p.p33);
        let eq44_e1313_d_n3: f64 = (eq44_e1311_d_n3 * p.p33);
        let eq44_e1313_d_n4: f64 = (eq44_e1311_d_n4 * p.p33);
        let eq44_e1313_d_n5: f64 = (eq44_e1311_d_n5 * p.p33);
        let eq44_e1313_d_n6: f64 = (eq44_e1311_d_n6 * p.p33);
        let eq44_e1313_d_n7: f64 = (eq44_e1311_d_n7 * p.p33);
        let eq44_e1313_d_n8: f64 = (eq44_e1311_d_n8 * p.p33);
        let eq44_e1313_d_n9: f64 = (eq44_e1311_d_n9 * p.p33);
        let eq44_e1313_d_n10: f64 = (eq44_e1311_d_n10 * p.p33);
        let eq44_e1313_d_n11: f64 = (eq44_e1311_d_n11 * p.p33);
        let eq44_e1315: f64 = (eq44_e1313 * s.v[851]);
        let eq44_e1315_d_n0: f64 = ((eq44_e1313_d_n0 * s.v[851]) + (eq44_e1313 * s.dn[851][0]));
        let eq44_e1315_d_n1: f64 = ((eq44_e1313_d_n1 * s.v[851]) + (eq44_e1313 * s.dn[851][1]));
        let eq44_e1315_d_n2: f64 = ((eq44_e1313_d_n2 * s.v[851]) + (eq44_e1313 * s.dn[851][2]));
        let eq44_e1315_d_n3: f64 = ((eq44_e1313_d_n3 * s.v[851]) + (eq44_e1313 * s.dn[851][3]));
        let eq44_e1315_d_n4: f64 = ((eq44_e1313_d_n4 * s.v[851]) + (eq44_e1313 * s.dn[851][4]));
        let eq44_e1315_d_n5: f64 = ((eq44_e1313_d_n5 * s.v[851]) + (eq44_e1313 * s.dn[851][5]));
        let eq44_e1315_d_n6: f64 = ((eq44_e1313_d_n6 * s.v[851]) + (eq44_e1313 * s.dn[851][6]));
        let eq44_e1315_d_n7: f64 = ((eq44_e1313_d_n7 * s.v[851]) + (eq44_e1313 * s.dn[851][7]));
        let eq44_e1315_d_n8: f64 = ((eq44_e1313_d_n8 * s.v[851]) + (eq44_e1313 * s.dn[851][8]));
        let eq44_e1315_d_n9: f64 = ((eq44_e1313_d_n9 * s.v[851]) + (eq44_e1313 * s.dn[851][9]));
        let eq44_e1315_d_n10: f64 = ((eq44_e1313_d_n10 * s.v[851]) + (eq44_e1313 * s.dn[851][10]));
        let eq44_e1315_d_n11: f64 = ((eq44_e1313_d_n11 * s.v[851]) + (eq44_e1313 * s.dn[851][11]));
        let eq44_e1316_q: f64 = eq44_e1315;
        let eq44_reactive_node_derivatives: [f64; 12] = [eq44_e1315_d_n0, eq44_e1315_d_n1, eq44_e1315_d_n2, eq44_e1315_d_n3, eq44_e1315_d_n4, eq44_e1315_d_n5, eq44_e1315_d_n6, eq44_e1315_d_n7, eq44_e1315_d_n8, eq44_e1315_d_n9, eq44_e1315_d_n10, eq44_e1315_d_n11];
        let eq44_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[6]),
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
        let eq45_e1319: f64 = (s.v[0] * s.v[15]);
        let eq45_e1319_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq45_e1319_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq45_e1319_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq45_e1319_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq45_e1319_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq45_e1319_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq45_e1319_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq45_e1319_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq45_e1319_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq45_e1319_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq45_e1319_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq45_e1319_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq45_e1321: f64 = (eq45_e1319 * p.p33);
        let eq45_e1321_d_n0: f64 = (eq45_e1319_d_n0 * p.p33);
        let eq45_e1321_d_n1: f64 = (eq45_e1319_d_n1 * p.p33);
        let eq45_e1321_d_n2: f64 = (eq45_e1319_d_n2 * p.p33);
        let eq45_e1321_d_n3: f64 = (eq45_e1319_d_n3 * p.p33);
        let eq45_e1321_d_n4: f64 = (eq45_e1319_d_n4 * p.p33);
        let eq45_e1321_d_n5: f64 = (eq45_e1319_d_n5 * p.p33);
        let eq45_e1321_d_n6: f64 = (eq45_e1319_d_n6 * p.p33);
        let eq45_e1321_d_n7: f64 = (eq45_e1319_d_n7 * p.p33);
        let eq45_e1321_d_n8: f64 = (eq45_e1319_d_n8 * p.p33);
        let eq45_e1321_d_n9: f64 = (eq45_e1319_d_n9 * p.p33);
        let eq45_e1321_d_n10: f64 = (eq45_e1319_d_n10 * p.p33);
        let eq45_e1321_d_n11: f64 = (eq45_e1319_d_n11 * p.p33);
        let eq45_e1323: f64 = (eq45_e1321 * s.v[852]);
        let eq45_e1323_d_n0: f64 = ((eq45_e1321_d_n0 * s.v[852]) + (eq45_e1321 * s.dn[852][0]));
        let eq45_e1323_d_n1: f64 = ((eq45_e1321_d_n1 * s.v[852]) + (eq45_e1321 * s.dn[852][1]));
        let eq45_e1323_d_n2: f64 = ((eq45_e1321_d_n2 * s.v[852]) + (eq45_e1321 * s.dn[852][2]));
        let eq45_e1323_d_n3: f64 = ((eq45_e1321_d_n3 * s.v[852]) + (eq45_e1321 * s.dn[852][3]));
        let eq45_e1323_d_n4: f64 = ((eq45_e1321_d_n4 * s.v[852]) + (eq45_e1321 * s.dn[852][4]));
        let eq45_e1323_d_n5: f64 = ((eq45_e1321_d_n5 * s.v[852]) + (eq45_e1321 * s.dn[852][5]));
        let eq45_e1323_d_n6: f64 = ((eq45_e1321_d_n6 * s.v[852]) + (eq45_e1321 * s.dn[852][6]));
        let eq45_e1323_d_n7: f64 = ((eq45_e1321_d_n7 * s.v[852]) + (eq45_e1321 * s.dn[852][7]));
        let eq45_e1323_d_n8: f64 = ((eq45_e1321_d_n8 * s.v[852]) + (eq45_e1321 * s.dn[852][8]));
        let eq45_e1323_d_n9: f64 = ((eq45_e1321_d_n9 * s.v[852]) + (eq45_e1321 * s.dn[852][9]));
        let eq45_e1323_d_n10: f64 = ((eq45_e1321_d_n10 * s.v[852]) + (eq45_e1321 * s.dn[852][10]));
        let eq45_e1323_d_n11: f64 = ((eq45_e1321_d_n11 * s.v[852]) + (eq45_e1321 * s.dn[852][11]));
        let eq45_e1324_q: f64 = eq45_e1323;
        let eq45_reactive_node_derivatives: [f64; 12] = [eq45_e1323_d_n0, eq45_e1323_d_n1, eq45_e1323_d_n2, eq45_e1323_d_n3, eq45_e1323_d_n4, eq45_e1323_d_n5, eq45_e1323_d_n6, eq45_e1323_d_n7, eq45_e1323_d_n8, eq45_e1323_d_n9, eq45_e1323_d_n10, eq45_e1323_d_n11];
        let eq45_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            &nodes,
            &eq45_reactive_node_derivatives,
            &branches,
            &eq45_reactive_branch_derivatives,
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
        let eq48_e1336_q: f64 = eq48_e1335;
        let eq48_reactive_node_derivatives: [f64; 12] = [eq48_e1335_d_n0, eq48_e1335_d_n1, eq48_e1335_d_n2, eq48_e1335_d_n3, eq48_e1335_d_n4, eq48_e1335_d_n5, eq48_e1335_d_n6, eq48_e1335_d_n7, eq48_e1335_d_n8, eq48_e1335_d_n9, eq48_e1335_d_n10, eq48_e1335_d_n11];
        let eq48_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            &nodes,
            &eq48_reactive_node_derivatives,
            &branches,
            &eq48_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_49_block_0(
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
        let eq49_e1347_q: f64 = eq49_e1346;
        let eq49_e1348: f64 = (-eq49_e1346);
        let eq49_e1348_d_n0: f64 = (-eq49_e1346_d_n0);
        let eq49_e1348_d_n1: f64 = (-eq49_e1346_d_n1);
        let eq49_e1348_d_n2: f64 = (-eq49_e1346_d_n2);
        let eq49_e1348_d_n3: f64 = (-eq49_e1346_d_n3);
        let eq49_e1348_d_n4: f64 = (-eq49_e1346_d_n4);
        let eq49_e1348_d_n5: f64 = (-eq49_e1346_d_n5);
        let eq49_e1348_d_n6: f64 = (-eq49_e1346_d_n6);
        let eq49_e1348_d_n7: f64 = (-eq49_e1346_d_n7);
        let eq49_e1348_d_n8: f64 = (-eq49_e1346_d_n8);
        let eq49_e1348_d_n9: f64 = (-eq49_e1346_d_n9);
        let eq49_e1348_d_n10: f64 = (-eq49_e1346_d_n10);
        let eq49_e1348_d_n11: f64 = (-eq49_e1346_d_n11);
        let eq49_e1348_q: f64 = (-eq49_e1347_q);
        let eq49_e1348_q_d_n0: f64 = (-eq49_e1346_d_n0);
        let eq49_e1348_q_d_n1: f64 = (-eq49_e1346_d_n1);
        let eq49_e1348_q_d_n2: f64 = (-eq49_e1346_d_n2);
        let eq49_e1348_q_d_n3: f64 = (-eq49_e1346_d_n3);
        let eq49_e1348_q_d_n4: f64 = (-eq49_e1346_d_n4);
        let eq49_e1348_q_d_n5: f64 = (-eq49_e1346_d_n5);
        let eq49_e1348_q_d_n6: f64 = (-eq49_e1346_d_n6);
        let eq49_e1348_q_d_n7: f64 = (-eq49_e1346_d_n7);
        let eq49_e1348_q_d_n8: f64 = (-eq49_e1346_d_n8);
        let eq49_e1348_q_d_n9: f64 = (-eq49_e1346_d_n9);
        let eq49_e1348_q_d_n10: f64 = (-eq49_e1346_d_n10);
        let eq49_e1348_q_d_n11: f64 = (-eq49_e1346_d_n11);
        let eq49_reactive_node_derivatives: [f64; 12] = [eq49_e1348_q_d_n0, eq49_e1348_q_d_n1, eq49_e1348_q_d_n2, eq49_e1348_q_d_n3, eq49_e1348_q_d_n4, eq49_e1348_q_d_n5, eq49_e1348_q_d_n6, eq49_e1348_q_d_n7, eq49_e1348_q_d_n8, eq49_e1348_q_d_n9, eq49_e1348_q_d_n10, eq49_e1348_q_d_n11];
        let eq49_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            &nodes,
            &eq49_reactive_node_derivatives,
            &branches,
            &eq49_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_50_block_0(
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
        let eq50_e1359_q: f64 = eq50_e1358;
        let eq50_e1360: f64 = (-eq50_e1358);
        let eq50_e1360_d_n0: f64 = (-eq50_e1358_d_n0);
        let eq50_e1360_d_n1: f64 = (-eq50_e1358_d_n1);
        let eq50_e1360_d_n2: f64 = (-eq50_e1358_d_n2);
        let eq50_e1360_d_n3: f64 = (-eq50_e1358_d_n3);
        let eq50_e1360_d_n4: f64 = (-eq50_e1358_d_n4);
        let eq50_e1360_d_n5: f64 = (-eq50_e1358_d_n5);
        let eq50_e1360_d_n6: f64 = (-eq50_e1358_d_n6);
        let eq50_e1360_d_n7: f64 = (-eq50_e1358_d_n7);
        let eq50_e1360_d_n8: f64 = (-eq50_e1358_d_n8);
        let eq50_e1360_d_n9: f64 = (-eq50_e1358_d_n9);
        let eq50_e1360_d_n10: f64 = (-eq50_e1358_d_n10);
        let eq50_e1360_d_n11: f64 = (-eq50_e1358_d_n11);
        let eq50_e1360_q: f64 = (-eq50_e1359_q);
        let eq50_e1360_q_d_n0: f64 = (-eq50_e1358_d_n0);
        let eq50_e1360_q_d_n1: f64 = (-eq50_e1358_d_n1);
        let eq50_e1360_q_d_n2: f64 = (-eq50_e1358_d_n2);
        let eq50_e1360_q_d_n3: f64 = (-eq50_e1358_d_n3);
        let eq50_e1360_q_d_n4: f64 = (-eq50_e1358_d_n4);
        let eq50_e1360_q_d_n5: f64 = (-eq50_e1358_d_n5);
        let eq50_e1360_q_d_n6: f64 = (-eq50_e1358_d_n6);
        let eq50_e1360_q_d_n7: f64 = (-eq50_e1358_d_n7);
        let eq50_e1360_q_d_n8: f64 = (-eq50_e1358_d_n8);
        let eq50_e1360_q_d_n9: f64 = (-eq50_e1358_d_n9);
        let eq50_e1360_q_d_n10: f64 = (-eq50_e1358_d_n10);
        let eq50_e1360_q_d_n11: f64 = (-eq50_e1358_d_n11);
        let eq50_reactive_node_derivatives: [f64; 12] = [eq50_e1360_q_d_n0, eq50_e1360_q_d_n1, eq50_e1360_q_d_n2, eq50_e1360_q_d_n3, eq50_e1360_q_d_n4, eq50_e1360_q_d_n5, eq50_e1360_q_d_n6, eq50_e1360_q_d_n7, eq50_e1360_q_d_n8, eq50_e1360_q_d_n9, eq50_e1360_q_d_n10, eq50_e1360_q_d_n11];
        let eq50_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            &nodes,
            &eq50_reactive_node_derivatives,
            &branches,
            &eq50_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
