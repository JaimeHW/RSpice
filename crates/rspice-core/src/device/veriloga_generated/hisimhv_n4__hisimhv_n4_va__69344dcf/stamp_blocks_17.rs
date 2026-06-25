#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_34_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq34_e1200: f64 = (-p.p87);
        let eq34_e1202_q: f64 = s.v[301];
        let eq34_e1203: f64 = (eq34_e1200 * s.v[301]);
        let eq34_e1203_d_n0: f64 = (eq34_e1200 * s.dn[301][0]);
        let eq34_e1203_d_n1: f64 = (eq34_e1200 * s.dn[301][1]);
        let eq34_e1203_d_n2: f64 = (eq34_e1200 * s.dn[301][2]);
        let eq34_e1203_d_n3: f64 = (eq34_e1200 * s.dn[301][3]);
        let eq34_e1203_d_n4: f64 = (eq34_e1200 * s.dn[301][4]);
        let eq34_e1203_d_n5: f64 = (eq34_e1200 * s.dn[301][5]);
        let eq34_e1203_d_n6: f64 = (eq34_e1200 * s.dn[301][6]);
        let eq34_e1203_d_n7: f64 = (eq34_e1200 * s.dn[301][7]);
        let eq34_e1203_d_n8: f64 = (eq34_e1200 * s.dn[301][8]);
        let eq34_e1203_d_n9: f64 = (eq34_e1200 * s.dn[301][9]);
        let eq34_e1203_d_n10: f64 = (eq34_e1200 * s.dn[301][10]);
        let eq34_e1203_d_n11: f64 = (eq34_e1200 * s.dn[301][11]);
        let eq34_e1203_d_n12: f64 = (eq34_e1200 * s.dn[301][12]);
        let eq34_e1203_d_n13: f64 = (eq34_e1200 * s.dn[301][13]);
        let eq34_e1203_d_n14: f64 = (eq34_e1200 * s.dn[301][14]);
        let eq34_e1203_d_n15: f64 = (eq34_e1200 * s.dn[301][15]);
        let eq34_e1203_d_n16: f64 = (eq34_e1200 * s.dn[301][16]);
        let eq34_e1203_d_n17: f64 = (eq34_e1200 * s.dn[301][17]);
        let eq34_e1203_d_b0: f64 = (eq34_e1200 * s.db[301][0]);
        let eq34_e1203_d_b1: f64 = (eq34_e1200 * s.db[301][1]);
        let eq34_e1203_d_b2: f64 = (eq34_e1200 * s.db[301][2]);
        let eq34_e1203_d_b3: f64 = (eq34_e1200 * s.db[301][3]);
        let eq34_e1203_d_b4: f64 = (eq34_e1200 * s.db[301][4]);
        let eq34_e1203_d_b5: f64 = (eq34_e1200 * s.db[301][5]);
        let eq34_e1203_d_b6: f64 = (eq34_e1200 * s.db[301][6]);
        let eq34_e1203_d_b7: f64 = (eq34_e1200 * s.db[301][7]);
        let eq34_e1203_d_b8: f64 = (eq34_e1200 * s.db[301][8]);
        let eq34_e1203_q: f64 = (eq34_e1200 * eq34_e1202_q);
        let eq34_e1203_q_d_n0: f64 = (eq34_e1200 * s.dn[301][0]);
        let eq34_e1203_q_d_n1: f64 = (eq34_e1200 * s.dn[301][1]);
        let eq34_e1203_q_d_n2: f64 = (eq34_e1200 * s.dn[301][2]);
        let eq34_e1203_q_d_n3: f64 = (eq34_e1200 * s.dn[301][3]);
        let eq34_e1203_q_d_n4: f64 = (eq34_e1200 * s.dn[301][4]);
        let eq34_e1203_q_d_n5: f64 = (eq34_e1200 * s.dn[301][5]);
        let eq34_e1203_q_d_n6: f64 = (eq34_e1200 * s.dn[301][6]);
        let eq34_e1203_q_d_n7: f64 = (eq34_e1200 * s.dn[301][7]);
        let eq34_e1203_q_d_n8: f64 = (eq34_e1200 * s.dn[301][8]);
        let eq34_e1203_q_d_n9: f64 = (eq34_e1200 * s.dn[301][9]);
        let eq34_e1203_q_d_n10: f64 = (eq34_e1200 * s.dn[301][10]);
        let eq34_e1203_q_d_n11: f64 = (eq34_e1200 * s.dn[301][11]);
        let eq34_e1203_q_d_n12: f64 = (eq34_e1200 * s.dn[301][12]);
        let eq34_e1203_q_d_n13: f64 = (eq34_e1200 * s.dn[301][13]);
        let eq34_e1203_q_d_n14: f64 = (eq34_e1200 * s.dn[301][14]);
        let eq34_e1203_q_d_n15: f64 = (eq34_e1200 * s.dn[301][15]);
        let eq34_e1203_q_d_n16: f64 = (eq34_e1200 * s.dn[301][16]);
        let eq34_e1203_q_d_n17: f64 = (eq34_e1200 * s.dn[301][17]);
        let eq34_e1203_q_d_b0: f64 = (eq34_e1200 * s.db[301][0]);
        let eq34_e1203_q_d_b1: f64 = (eq34_e1200 * s.db[301][1]);
        let eq34_e1203_q_d_b2: f64 = (eq34_e1200 * s.db[301][2]);
        let eq34_e1203_q_d_b3: f64 = (eq34_e1200 * s.db[301][3]);
        let eq34_e1203_q_d_b4: f64 = (eq34_e1200 * s.db[301][4]);
        let eq34_e1203_q_d_b5: f64 = (eq34_e1200 * s.db[301][5]);
        let eq34_e1203_q_d_b6: f64 = (eq34_e1200 * s.db[301][6]);
        let eq34_e1203_q_d_b7: f64 = (eq34_e1200 * s.db[301][7]);
        let eq34_e1203_q_d_b8: f64 = (eq34_e1200 * s.db[301][8]);
        let eq34_reactive_node_derivatives: [f64; 18] = [eq34_e1203_q_d_n0, eq34_e1203_q_d_n1, eq34_e1203_q_d_n2, eq34_e1203_q_d_n3, eq34_e1203_q_d_n4, eq34_e1203_q_d_n5, eq34_e1203_q_d_n6, eq34_e1203_q_d_n7, eq34_e1203_q_d_n8, eq34_e1203_q_d_n9, eq34_e1203_q_d_n10, eq34_e1203_q_d_n11, eq34_e1203_q_d_n12, eq34_e1203_q_d_n13, eq34_e1203_q_d_n14, eq34_e1203_q_d_n15, eq34_e1203_q_d_n16, eq34_e1203_q_d_n17];
        let eq34_reactive_branch_derivatives: [f64; 9] = [eq34_e1203_q_d_b0, eq34_e1203_q_d_b1, eq34_e1203_q_d_b2, eq34_e1203_q_d_b3, eq34_e1203_q_d_b4, eq34_e1203_q_d_b5, eq34_e1203_q_d_b6, eq34_e1203_q_d_b7, eq34_e1203_q_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[2]),
            &nodes,
            &eq34_reactive_node_derivatives,
            &branches,
            &eq34_reactive_branch_derivatives,
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
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq40_e1232: f64 = ((nv14 - 0.0) * s.v[954]);
        let eq40_e1232_d_n0: f64 = ((nv14 - 0.0) * s.dn[954][0]);
        let eq40_e1232_d_n1: f64 = ((nv14 - 0.0) * s.dn[954][1]);
        let eq40_e1232_d_n2: f64 = ((nv14 - 0.0) * s.dn[954][2]);
        let eq40_e1232_d_n3: f64 = ((nv14 - 0.0) * s.dn[954][3]);
        let eq40_e1232_d_n4: f64 = ((nv14 - 0.0) * s.dn[954][4]);
        let eq40_e1232_d_n5: f64 = ((nv14 - 0.0) * s.dn[954][5]);
        let eq40_e1232_d_n6: f64 = ((nv14 - 0.0) * s.dn[954][6]);
        let eq40_e1232_d_n7: f64 = ((nv14 - 0.0) * s.dn[954][7]);
        let eq40_e1232_d_n8: f64 = ((nv14 - 0.0) * s.dn[954][8]);
        let eq40_e1232_d_n9: f64 = ((nv14 - 0.0) * s.dn[954][9]);
        let eq40_e1232_d_n10: f64 = ((nv14 - 0.0) * s.dn[954][10]);
        let eq40_e1232_d_n11: f64 = ((nv14 - 0.0) * s.dn[954][11]);
        let eq40_e1232_d_n12: f64 = ((nv14 - 0.0) * s.dn[954][12]);
        let eq40_e1232_d_n13: f64 = ((nv14 - 0.0) * s.dn[954][13]);
        let eq40_e1232_d_n14: f64 = (s.v[954] + ((nv14 - 0.0) * s.dn[954][14]));
        let eq40_e1232_d_n15: f64 = ((nv14 - 0.0) * s.dn[954][15]);
        let eq40_e1232_d_n16: f64 = ((nv14 - 0.0) * s.dn[954][16]);
        let eq40_e1232_d_n17: f64 = ((nv14 - 0.0) * s.dn[954][17]);
        let eq40_e1232_d_b0: f64 = ((nv14 - 0.0) * s.db[954][0]);
        let eq40_e1232_d_b1: f64 = ((nv14 - 0.0) * s.db[954][1]);
        let eq40_e1232_d_b2: f64 = ((nv14 - 0.0) * s.db[954][2]);
        let eq40_e1232_d_b3: f64 = ((nv14 - 0.0) * s.db[954][3]);
        let eq40_e1232_d_b4: f64 = ((nv14 - 0.0) * s.db[954][4]);
        let eq40_e1232_d_b5: f64 = ((nv14 - 0.0) * s.db[954][5]);
        let eq40_e1232_d_b6: f64 = ((nv14 - 0.0) * s.db[954][6]);
        let eq40_e1232_d_b7: f64 = ((nv14 - 0.0) * s.db[954][7]);
        let eq40_e1232_d_b8: f64 = ((nv14 - 0.0) * s.db[954][8]);
        let eq40_e1233_q: f64 = eq40_e1232;
        let eq40_reactive_node_derivatives: [f64; 18] = [eq40_e1232_d_n0, eq40_e1232_d_n1, eq40_e1232_d_n2, eq40_e1232_d_n3, eq40_e1232_d_n4, eq40_e1232_d_n5, eq40_e1232_d_n6, eq40_e1232_d_n7, eq40_e1232_d_n8, eq40_e1232_d_n9, eq40_e1232_d_n10, eq40_e1232_d_n11, eq40_e1232_d_n12, eq40_e1232_d_n13, eq40_e1232_d_n14, eq40_e1232_d_n15, eq40_e1232_d_n16, eq40_e1232_d_n17];
        let eq40_reactive_branch_derivatives: [f64; 9] = [eq40_e1232_d_b0, eq40_e1232_d_b1, eq40_e1232_d_b2, eq40_e1232_d_b3, eq40_e1232_d_b4, eq40_e1232_d_b5, eq40_e1232_d_b6, eq40_e1232_d_b7, eq40_e1232_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
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
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq41_e1236: f64 = ((nv14 - 0.0) * s.v[955]);
        let eq41_e1236_d_n0: f64 = ((nv14 - 0.0) * s.dn[955][0]);
        let eq41_e1236_d_n1: f64 = ((nv14 - 0.0) * s.dn[955][1]);
        let eq41_e1236_d_n2: f64 = ((nv14 - 0.0) * s.dn[955][2]);
        let eq41_e1236_d_n3: f64 = ((nv14 - 0.0) * s.dn[955][3]);
        let eq41_e1236_d_n4: f64 = ((nv14 - 0.0) * s.dn[955][4]);
        let eq41_e1236_d_n5: f64 = ((nv14 - 0.0) * s.dn[955][5]);
        let eq41_e1236_d_n6: f64 = ((nv14 - 0.0) * s.dn[955][6]);
        let eq41_e1236_d_n7: f64 = ((nv14 - 0.0) * s.dn[955][7]);
        let eq41_e1236_d_n8: f64 = ((nv14 - 0.0) * s.dn[955][8]);
        let eq41_e1236_d_n9: f64 = ((nv14 - 0.0) * s.dn[955][9]);
        let eq41_e1236_d_n10: f64 = ((nv14 - 0.0) * s.dn[955][10]);
        let eq41_e1236_d_n11: f64 = ((nv14 - 0.0) * s.dn[955][11]);
        let eq41_e1236_d_n12: f64 = ((nv14 - 0.0) * s.dn[955][12]);
        let eq41_e1236_d_n13: f64 = ((nv14 - 0.0) * s.dn[955][13]);
        let eq41_e1236_d_n14: f64 = (s.v[955] + ((nv14 - 0.0) * s.dn[955][14]));
        let eq41_e1236_d_n15: f64 = ((nv14 - 0.0) * s.dn[955][15]);
        let eq41_e1236_d_n16: f64 = ((nv14 - 0.0) * s.dn[955][16]);
        let eq41_e1236_d_n17: f64 = ((nv14 - 0.0) * s.dn[955][17]);
        let eq41_e1236_d_b0: f64 = ((nv14 - 0.0) * s.db[955][0]);
        let eq41_e1236_d_b1: f64 = ((nv14 - 0.0) * s.db[955][1]);
        let eq41_e1236_d_b2: f64 = ((nv14 - 0.0) * s.db[955][2]);
        let eq41_e1236_d_b3: f64 = ((nv14 - 0.0) * s.db[955][3]);
        let eq41_e1236_d_b4: f64 = ((nv14 - 0.0) * s.db[955][4]);
        let eq41_e1236_d_b5: f64 = ((nv14 - 0.0) * s.db[955][5]);
        let eq41_e1236_d_b6: f64 = ((nv14 - 0.0) * s.db[955][6]);
        let eq41_e1236_d_b7: f64 = ((nv14 - 0.0) * s.db[955][7]);
        let eq41_e1236_d_b8: f64 = ((nv14 - 0.0) * s.db[955][8]);
        let eq41_e1237_q: f64 = eq41_e1236;
        let eq41_reactive_node_derivatives: [f64; 18] = [eq41_e1236_d_n0, eq41_e1236_d_n1, eq41_e1236_d_n2, eq41_e1236_d_n3, eq41_e1236_d_n4, eq41_e1236_d_n5, eq41_e1236_d_n6, eq41_e1236_d_n7, eq41_e1236_d_n8, eq41_e1236_d_n9, eq41_e1236_d_n10, eq41_e1236_d_n11, eq41_e1236_d_n12, eq41_e1236_d_n13, eq41_e1236_d_n14, eq41_e1236_d_n15, eq41_e1236_d_n16, eq41_e1236_d_n17];
        let eq41_reactive_branch_derivatives: [f64; 9] = [eq41_e1236_d_b0, eq41_e1236_d_b1, eq41_e1236_d_b2, eq41_e1236_d_b3, eq41_e1236_d_b4, eq41_e1236_d_b5, eq41_e1236_d_b6, eq41_e1236_d_b7, eq41_e1236_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[5]),
            &nodes,
            &eq41_reactive_node_derivatives,
            &branches,
            &eq41_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_58_block_0(
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
        let eq58_e1342: f64 = (s.v[767] * (nv4 - 0.0));
        let eq58_e1342_d_n0: f64 = (s.dn[767][0] * (nv4 - 0.0));
        let eq58_e1342_d_n1: f64 = (s.dn[767][1] * (nv4 - 0.0));
        let eq58_e1342_d_n2: f64 = (s.dn[767][2] * (nv4 - 0.0));
        let eq58_e1342_d_n3: f64 = (s.dn[767][3] * (nv4 - 0.0));
        let eq58_e1342_d_n4: f64 = ((s.dn[767][4] * (nv4 - 0.0)) + s.v[767]);
        let eq58_e1342_d_n5: f64 = (s.dn[767][5] * (nv4 - 0.0));
        let eq58_e1342_d_n6: f64 = (s.dn[767][6] * (nv4 - 0.0));
        let eq58_e1342_d_n7: f64 = (s.dn[767][7] * (nv4 - 0.0));
        let eq58_e1342_d_n8: f64 = (s.dn[767][8] * (nv4 - 0.0));
        let eq58_e1342_d_n9: f64 = (s.dn[767][9] * (nv4 - 0.0));
        let eq58_e1342_d_n10: f64 = (s.dn[767][10] * (nv4 - 0.0));
        let eq58_e1342_d_n11: f64 = (s.dn[767][11] * (nv4 - 0.0));
        let eq58_e1342_d_n12: f64 = (s.dn[767][12] * (nv4 - 0.0));
        let eq58_e1342_d_n13: f64 = (s.dn[767][13] * (nv4 - 0.0));
        let eq58_e1342_d_n14: f64 = (s.dn[767][14] * (nv4 - 0.0));
        let eq58_e1342_d_n15: f64 = (s.dn[767][15] * (nv4 - 0.0));
        let eq58_e1342_d_n16: f64 = (s.dn[767][16] * (nv4 - 0.0));
        let eq58_e1342_d_n17: f64 = (s.dn[767][17] * (nv4 - 0.0));
        let eq58_e1342_d_b0: f64 = (s.db[767][0] * (nv4 - 0.0));
        let eq58_e1342_d_b1: f64 = (s.db[767][1] * (nv4 - 0.0));
        let eq58_e1342_d_b2: f64 = (s.db[767][2] * (nv4 - 0.0));
        let eq58_e1342_d_b3: f64 = (s.db[767][3] * (nv4 - 0.0));
        let eq58_e1342_d_b4: f64 = (s.db[767][4] * (nv4 - 0.0));
        let eq58_e1342_d_b5: f64 = (s.db[767][5] * (nv4 - 0.0));
        let eq58_e1342_d_b6: f64 = (s.db[767][6] * (nv4 - 0.0));
        let eq58_e1342_d_b7: f64 = (s.db[767][7] * (nv4 - 0.0));
        let eq58_e1342_d_b8: f64 = (s.db[767][8] * (nv4 - 0.0));
        let eq58_e1343_q: f64 = eq58_e1342;
        let eq58_reactive_node_derivatives: [f64; 18] = [eq58_e1342_d_n0, eq58_e1342_d_n1, eq58_e1342_d_n2, eq58_e1342_d_n3, eq58_e1342_d_n4, eq58_e1342_d_n5, eq58_e1342_d_n6, eq58_e1342_d_n7, eq58_e1342_d_n8, eq58_e1342_d_n9, eq58_e1342_d_n10, eq58_e1342_d_n11, eq58_e1342_d_n12, eq58_e1342_d_n13, eq58_e1342_d_n14, eq58_e1342_d_n15, eq58_e1342_d_n16, eq58_e1342_d_n17];
        let eq58_reactive_branch_derivatives: [f64; 9] = [eq58_e1342_d_b0, eq58_e1342_d_b1, eq58_e1342_d_b2, eq58_e1342_d_b3, eq58_e1342_d_b4, eq58_e1342_d_b5, eq58_e1342_d_b6, eq58_e1342_d_b7, eq58_e1342_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            &nodes,
            &eq58_reactive_node_derivatives,
            &branches,
            &eq58_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_61_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq61_e1358, eq61_e1358_d_n0, eq61_e1358_d_n1, eq61_e1358_d_n2, eq61_e1358_d_n3, eq61_e1358_d_n4, eq61_e1358_d_n5, eq61_e1358_d_n6, eq61_e1358_d_n7, eq61_e1358_d_n8, eq61_e1358_d_n9, eq61_e1358_d_n10, eq61_e1358_d_n11, eq61_e1358_d_n12, eq61_e1358_d_n13, eq61_e1358_d_n14, eq61_e1358_d_n15, eq61_e1358_d_n16, eq61_e1358_d_n17, eq61_e1358_d_b0, eq61_e1358_d_b1, eq61_e1358_d_b2, eq61_e1358_d_b3, eq61_e1358_d_b4, eq61_e1358_d_b5, eq61_e1358_d_b6, eq61_e1358_d_b7, eq61_e1358_d_b8, eq61_e1358_q, eq61_e1358_q_d_n0, eq61_e1358_q_d_n1, eq61_e1358_q_d_n2, eq61_e1358_q_d_n3, eq61_e1358_q_d_n4, eq61_e1358_q_d_n5, eq61_e1358_q_d_n6, eq61_e1358_q_d_n7, eq61_e1358_q_d_n8, eq61_e1358_q_d_n9, eq61_e1358_q_d_n10, eq61_e1358_q_d_n11, eq61_e1358_q_d_n12, eq61_e1358_q_d_n13, eq61_e1358_q_d_n14, eq61_e1358_q_d_n15, eq61_e1358_q_d_n16, eq61_e1358_q_d_n17, eq61_e1358_q_d_b0, eq61_e1358_q_d_b1, eq61_e1358_q_d_b2, eq61_e1358_q_d_b3, eq61_e1358_q_d_b4, eq61_e1358_q_d_b5, eq61_e1358_q_d_b6, eq61_e1358_q_d_b7, eq61_e1358_q_d_b8,) = {
    if (p.p28 != 0.0) {
        let eq61_e1355: f64 = (s.v[800] * (nv11 - 0.0));
        let eq61_e1355_d_n0: f64 = (s.dn[800][0] * (nv11 - 0.0));
        let eq61_e1355_d_n1: f64 = (s.dn[800][1] * (nv11 - 0.0));
        let eq61_e1355_d_n2: f64 = (s.dn[800][2] * (nv11 - 0.0));
        let eq61_e1355_d_n3: f64 = (s.dn[800][3] * (nv11 - 0.0));
        let eq61_e1355_d_n4: f64 = (s.dn[800][4] * (nv11 - 0.0));
        let eq61_e1355_d_n5: f64 = (s.dn[800][5] * (nv11 - 0.0));
        let eq61_e1355_d_n6: f64 = (s.dn[800][6] * (nv11 - 0.0));
        let eq61_e1355_d_n7: f64 = (s.dn[800][7] * (nv11 - 0.0));
        let eq61_e1355_d_n8: f64 = (s.dn[800][8] * (nv11 - 0.0));
        let eq61_e1355_d_n9: f64 = (s.dn[800][9] * (nv11 - 0.0));
        let eq61_e1355_d_n10: f64 = (s.dn[800][10] * (nv11 - 0.0));
        let eq61_e1355_d_n11: f64 = ((s.dn[800][11] * (nv11 - 0.0)) + s.v[800]);
        let eq61_e1355_d_n12: f64 = (s.dn[800][12] * (nv11 - 0.0));
        let eq61_e1355_d_n13: f64 = (s.dn[800][13] * (nv11 - 0.0));
        let eq61_e1355_d_n14: f64 = (s.dn[800][14] * (nv11 - 0.0));
        let eq61_e1355_d_n15: f64 = (s.dn[800][15] * (nv11 - 0.0));
        let eq61_e1355_d_n16: f64 = (s.dn[800][16] * (nv11 - 0.0));
        let eq61_e1355_d_n17: f64 = (s.dn[800][17] * (nv11 - 0.0));
        let eq61_e1355_d_b0: f64 = (s.db[800][0] * (nv11 - 0.0));
        let eq61_e1355_d_b1: f64 = (s.db[800][1] * (nv11 - 0.0));
        let eq61_e1355_d_b2: f64 = (s.db[800][2] * (nv11 - 0.0));
        let eq61_e1355_d_b3: f64 = (s.db[800][3] * (nv11 - 0.0));
        let eq61_e1355_d_b4: f64 = (s.db[800][4] * (nv11 - 0.0));
        let eq61_e1355_d_b5: f64 = (s.db[800][5] * (nv11 - 0.0));
        let eq61_e1355_d_b6: f64 = (s.db[800][6] * (nv11 - 0.0));
        let eq61_e1355_d_b7: f64 = (s.db[800][7] * (nv11 - 0.0));
        let eq61_e1355_d_b8: f64 = (s.db[800][8] * (nv11 - 0.0));
        let eq61_e1356_q: f64 = eq61_e1355;
        (eq61_e1355, eq61_e1355_d_n0, eq61_e1355_d_n1, eq61_e1355_d_n2, eq61_e1355_d_n3, eq61_e1355_d_n4, eq61_e1355_d_n5, eq61_e1355_d_n6, eq61_e1355_d_n7, eq61_e1355_d_n8, eq61_e1355_d_n9, eq61_e1355_d_n10, eq61_e1355_d_n11, eq61_e1355_d_n12, eq61_e1355_d_n13, eq61_e1355_d_n14, eq61_e1355_d_n15, eq61_e1355_d_n16, eq61_e1355_d_n17, eq61_e1355_d_b0, eq61_e1355_d_b1, eq61_e1355_d_b2, eq61_e1355_d_b3, eq61_e1355_d_b4, eq61_e1355_d_b5, eq61_e1355_d_b6, eq61_e1355_d_b7, eq61_e1355_d_b8, eq61_e1356_q, eq61_e1355_d_n0, eq61_e1355_d_n1, eq61_e1355_d_n2, eq61_e1355_d_n3, eq61_e1355_d_n4, eq61_e1355_d_n5, eq61_e1355_d_n6, eq61_e1355_d_n7, eq61_e1355_d_n8, eq61_e1355_d_n9, eq61_e1355_d_n10, eq61_e1355_d_n11, eq61_e1355_d_n12, eq61_e1355_d_n13, eq61_e1355_d_n14, eq61_e1355_d_n15, eq61_e1355_d_n16, eq61_e1355_d_n17, eq61_e1355_d_b0, eq61_e1355_d_b1, eq61_e1355_d_b2, eq61_e1355_d_b3, eq61_e1355_d_b4, eq61_e1355_d_b5, eq61_e1355_d_b6, eq61_e1355_d_b7, eq61_e1355_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_reactive_node_derivatives: [f64; 18] = [eq61_e1358_q_d_n0, eq61_e1358_q_d_n1, eq61_e1358_q_d_n2, eq61_e1358_q_d_n3, eq61_e1358_q_d_n4, eq61_e1358_q_d_n5, eq61_e1358_q_d_n6, eq61_e1358_q_d_n7, eq61_e1358_q_d_n8, eq61_e1358_q_d_n9, eq61_e1358_q_d_n10, eq61_e1358_q_d_n11, eq61_e1358_q_d_n12, eq61_e1358_q_d_n13, eq61_e1358_q_d_n14, eq61_e1358_q_d_n15, eq61_e1358_q_d_n16, eq61_e1358_q_d_n17];
        let eq61_reactive_branch_derivatives: [f64; 9] = [eq61_e1358_q_d_b0, eq61_e1358_q_d_b1, eq61_e1358_q_d_b2, eq61_e1358_q_d_b3, eq61_e1358_q_d_b4, eq61_e1358_q_d_b5, eq61_e1358_q_d_b6, eq61_e1358_q_d_b7, eq61_e1358_q_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            None,
            &nodes,
            &eq61_reactive_node_derivatives,
            &branches,
            &eq61_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_62_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq62_e1365, eq62_e1365_d_n0, eq62_e1365_d_n1, eq62_e1365_d_n2, eq62_e1365_d_n3, eq62_e1365_d_n4, eq62_e1365_d_n5, eq62_e1365_d_n6, eq62_e1365_d_n7, eq62_e1365_d_n8, eq62_e1365_d_n9, eq62_e1365_d_n10, eq62_e1365_d_n11, eq62_e1365_d_n12, eq62_e1365_d_n13, eq62_e1365_d_n14, eq62_e1365_d_n15, eq62_e1365_d_n16, eq62_e1365_d_n17, eq62_e1365_d_b0, eq62_e1365_d_b1, eq62_e1365_d_b2, eq62_e1365_d_b3, eq62_e1365_d_b4, eq62_e1365_d_b5, eq62_e1365_d_b6, eq62_e1365_d_b7, eq62_e1365_d_b8, eq62_e1365_q, eq62_e1365_q_d_n0, eq62_e1365_q_d_n1, eq62_e1365_q_d_n2, eq62_e1365_q_d_n3, eq62_e1365_q_d_n4, eq62_e1365_q_d_n5, eq62_e1365_q_d_n6, eq62_e1365_q_d_n7, eq62_e1365_q_d_n8, eq62_e1365_q_d_n9, eq62_e1365_q_d_n10, eq62_e1365_q_d_n11, eq62_e1365_q_d_n12, eq62_e1365_q_d_n13, eq62_e1365_q_d_n14, eq62_e1365_q_d_n15, eq62_e1365_q_d_n16, eq62_e1365_q_d_n17, eq62_e1365_q_d_b0, eq62_e1365_q_d_b1, eq62_e1365_q_d_b2, eq62_e1365_q_d_b3, eq62_e1365_q_d_b4, eq62_e1365_q_d_b5, eq62_e1365_q_d_b6, eq62_e1365_q_d_b7, eq62_e1365_q_d_b8,) = {
    if (p.p28 != 0.0) {
        let eq62_e1362: f64 = (s.v[801] * (nv12 - 0.0));
        let eq62_e1362_d_n0: f64 = (s.dn[801][0] * (nv12 - 0.0));
        let eq62_e1362_d_n1: f64 = (s.dn[801][1] * (nv12 - 0.0));
        let eq62_e1362_d_n2: f64 = (s.dn[801][2] * (nv12 - 0.0));
        let eq62_e1362_d_n3: f64 = (s.dn[801][3] * (nv12 - 0.0));
        let eq62_e1362_d_n4: f64 = (s.dn[801][4] * (nv12 - 0.0));
        let eq62_e1362_d_n5: f64 = (s.dn[801][5] * (nv12 - 0.0));
        let eq62_e1362_d_n6: f64 = (s.dn[801][6] * (nv12 - 0.0));
        let eq62_e1362_d_n7: f64 = (s.dn[801][7] * (nv12 - 0.0));
        let eq62_e1362_d_n8: f64 = (s.dn[801][8] * (nv12 - 0.0));
        let eq62_e1362_d_n9: f64 = (s.dn[801][9] * (nv12 - 0.0));
        let eq62_e1362_d_n10: f64 = (s.dn[801][10] * (nv12 - 0.0));
        let eq62_e1362_d_n11: f64 = (s.dn[801][11] * (nv12 - 0.0));
        let eq62_e1362_d_n12: f64 = ((s.dn[801][12] * (nv12 - 0.0)) + s.v[801]);
        let eq62_e1362_d_n13: f64 = (s.dn[801][13] * (nv12 - 0.0));
        let eq62_e1362_d_n14: f64 = (s.dn[801][14] * (nv12 - 0.0));
        let eq62_e1362_d_n15: f64 = (s.dn[801][15] * (nv12 - 0.0));
        let eq62_e1362_d_n16: f64 = (s.dn[801][16] * (nv12 - 0.0));
        let eq62_e1362_d_n17: f64 = (s.dn[801][17] * (nv12 - 0.0));
        let eq62_e1362_d_b0: f64 = (s.db[801][0] * (nv12 - 0.0));
        let eq62_e1362_d_b1: f64 = (s.db[801][1] * (nv12 - 0.0));
        let eq62_e1362_d_b2: f64 = (s.db[801][2] * (nv12 - 0.0));
        let eq62_e1362_d_b3: f64 = (s.db[801][3] * (nv12 - 0.0));
        let eq62_e1362_d_b4: f64 = (s.db[801][4] * (nv12 - 0.0));
        let eq62_e1362_d_b5: f64 = (s.db[801][5] * (nv12 - 0.0));
        let eq62_e1362_d_b6: f64 = (s.db[801][6] * (nv12 - 0.0));
        let eq62_e1362_d_b7: f64 = (s.db[801][7] * (nv12 - 0.0));
        let eq62_e1362_d_b8: f64 = (s.db[801][8] * (nv12 - 0.0));
        let eq62_e1363_q: f64 = eq62_e1362;
        (eq62_e1362, eq62_e1362_d_n0, eq62_e1362_d_n1, eq62_e1362_d_n2, eq62_e1362_d_n3, eq62_e1362_d_n4, eq62_e1362_d_n5, eq62_e1362_d_n6, eq62_e1362_d_n7, eq62_e1362_d_n8, eq62_e1362_d_n9, eq62_e1362_d_n10, eq62_e1362_d_n11, eq62_e1362_d_n12, eq62_e1362_d_n13, eq62_e1362_d_n14, eq62_e1362_d_n15, eq62_e1362_d_n16, eq62_e1362_d_n17, eq62_e1362_d_b0, eq62_e1362_d_b1, eq62_e1362_d_b2, eq62_e1362_d_b3, eq62_e1362_d_b4, eq62_e1362_d_b5, eq62_e1362_d_b6, eq62_e1362_d_b7, eq62_e1362_d_b8, eq62_e1363_q, eq62_e1362_d_n0, eq62_e1362_d_n1, eq62_e1362_d_n2, eq62_e1362_d_n3, eq62_e1362_d_n4, eq62_e1362_d_n5, eq62_e1362_d_n6, eq62_e1362_d_n7, eq62_e1362_d_n8, eq62_e1362_d_n9, eq62_e1362_d_n10, eq62_e1362_d_n11, eq62_e1362_d_n12, eq62_e1362_d_n13, eq62_e1362_d_n14, eq62_e1362_d_n15, eq62_e1362_d_n16, eq62_e1362_d_n17, eq62_e1362_d_b0, eq62_e1362_d_b1, eq62_e1362_d_b2, eq62_e1362_d_b3, eq62_e1362_d_b4, eq62_e1362_d_b5, eq62_e1362_d_b6, eq62_e1362_d_b7, eq62_e1362_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_reactive_node_derivatives: [f64; 18] = [eq62_e1365_q_d_n0, eq62_e1365_q_d_n1, eq62_e1365_q_d_n2, eq62_e1365_q_d_n3, eq62_e1365_q_d_n4, eq62_e1365_q_d_n5, eq62_e1365_q_d_n6, eq62_e1365_q_d_n7, eq62_e1365_q_d_n8, eq62_e1365_q_d_n9, eq62_e1365_q_d_n10, eq62_e1365_q_d_n11, eq62_e1365_q_d_n12, eq62_e1365_q_d_n13, eq62_e1365_q_d_n14, eq62_e1365_q_d_n15, eq62_e1365_q_d_n16, eq62_e1365_q_d_n17];
        let eq62_reactive_branch_derivatives: [f64; 9] = [eq62_e1365_q_d_b0, eq62_e1365_q_d_b1, eq62_e1365_q_d_b2, eq62_e1365_q_d_b3, eq62_e1365_q_d_b4, eq62_e1365_q_d_b5, eq62_e1365_q_d_b6, eq62_e1365_q_d_b7, eq62_e1365_q_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            None,
            &nodes,
            &eq62_reactive_node_derivatives,
            &branches,
            &eq62_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_66_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq66_e1384, eq66_e1384_d_n13, eq66_e1384_q, eq66_e1384_q_d_n13,) = {
    if (p.p29 != 0.0) {
        let eq66_e1382_q: f64 = (nv13 - 0.0);
        ((nv13 - 0.0), 1.0, eq66_e1382_q, 1.0,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[13]),
            None,
            &[
                GeneratedDerivative::node(nodes[13], self.multiplicity * (eq66_e1384_q_d_n13)),
            ],
        );
    }
}
