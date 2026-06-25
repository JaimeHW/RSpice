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
        let eq34_e1199: f64 = (-p.p87);
        let eq34_e1201_q: f64 = s.v[299];
        let eq34_e1202: f64 = (eq34_e1199 * s.v[299]);
        let eq34_e1202_d_n0: f64 = (eq34_e1199 * s.dn[299][0]);
        let eq34_e1202_d_n1: f64 = (eq34_e1199 * s.dn[299][1]);
        let eq34_e1202_d_n2: f64 = (eq34_e1199 * s.dn[299][2]);
        let eq34_e1202_d_n3: f64 = (eq34_e1199 * s.dn[299][3]);
        let eq34_e1202_d_n4: f64 = (eq34_e1199 * s.dn[299][4]);
        let eq34_e1202_d_n5: f64 = (eq34_e1199 * s.dn[299][5]);
        let eq34_e1202_d_n6: f64 = (eq34_e1199 * s.dn[299][6]);
        let eq34_e1202_d_n7: f64 = (eq34_e1199 * s.dn[299][7]);
        let eq34_e1202_d_n8: f64 = (eq34_e1199 * s.dn[299][8]);
        let eq34_e1202_d_n9: f64 = (eq34_e1199 * s.dn[299][9]);
        let eq34_e1202_d_n10: f64 = (eq34_e1199 * s.dn[299][10]);
        let eq34_e1202_d_n11: f64 = (eq34_e1199 * s.dn[299][11]);
        let eq34_e1202_d_n12: f64 = (eq34_e1199 * s.dn[299][12]);
        let eq34_e1202_d_n13: f64 = (eq34_e1199 * s.dn[299][13]);
        let eq34_e1202_d_n14: f64 = (eq34_e1199 * s.dn[299][14]);
        let eq34_e1202_d_n15: f64 = (eq34_e1199 * s.dn[299][15]);
        let eq34_e1202_d_n16: f64 = (eq34_e1199 * s.dn[299][16]);
        let eq34_e1202_d_n17: f64 = (eq34_e1199 * s.dn[299][17]);
        let eq34_e1202_d_n18: f64 = (eq34_e1199 * s.dn[299][18]);
        let eq34_e1202_d_b0: f64 = (eq34_e1199 * s.db[299][0]);
        let eq34_e1202_d_b1: f64 = (eq34_e1199 * s.db[299][1]);
        let eq34_e1202_d_b2: f64 = (eq34_e1199 * s.db[299][2]);
        let eq34_e1202_d_b3: f64 = (eq34_e1199 * s.db[299][3]);
        let eq34_e1202_d_b4: f64 = (eq34_e1199 * s.db[299][4]);
        let eq34_e1202_d_b5: f64 = (eq34_e1199 * s.db[299][5]);
        let eq34_e1202_d_b6: f64 = (eq34_e1199 * s.db[299][6]);
        let eq34_e1202_d_b7: f64 = (eq34_e1199 * s.db[299][7]);
        let eq34_e1202_d_b8: f64 = (eq34_e1199 * s.db[299][8]);
        let eq34_e1202_d_b9: f64 = (eq34_e1199 * s.db[299][9]);
        let eq34_e1202_q: f64 = (eq34_e1199 * eq34_e1201_q);
        let eq34_e1202_q_d_n0: f64 = (eq34_e1199 * s.dn[299][0]);
        let eq34_e1202_q_d_n1: f64 = (eq34_e1199 * s.dn[299][1]);
        let eq34_e1202_q_d_n2: f64 = (eq34_e1199 * s.dn[299][2]);
        let eq34_e1202_q_d_n3: f64 = (eq34_e1199 * s.dn[299][3]);
        let eq34_e1202_q_d_n4: f64 = (eq34_e1199 * s.dn[299][4]);
        let eq34_e1202_q_d_n5: f64 = (eq34_e1199 * s.dn[299][5]);
        let eq34_e1202_q_d_n6: f64 = (eq34_e1199 * s.dn[299][6]);
        let eq34_e1202_q_d_n7: f64 = (eq34_e1199 * s.dn[299][7]);
        let eq34_e1202_q_d_n8: f64 = (eq34_e1199 * s.dn[299][8]);
        let eq34_e1202_q_d_n9: f64 = (eq34_e1199 * s.dn[299][9]);
        let eq34_e1202_q_d_n10: f64 = (eq34_e1199 * s.dn[299][10]);
        let eq34_e1202_q_d_n11: f64 = (eq34_e1199 * s.dn[299][11]);
        let eq34_e1202_q_d_n12: f64 = (eq34_e1199 * s.dn[299][12]);
        let eq34_e1202_q_d_n13: f64 = (eq34_e1199 * s.dn[299][13]);
        let eq34_e1202_q_d_n14: f64 = (eq34_e1199 * s.dn[299][14]);
        let eq34_e1202_q_d_n15: f64 = (eq34_e1199 * s.dn[299][15]);
        let eq34_e1202_q_d_n16: f64 = (eq34_e1199 * s.dn[299][16]);
        let eq34_e1202_q_d_n17: f64 = (eq34_e1199 * s.dn[299][17]);
        let eq34_e1202_q_d_n18: f64 = (eq34_e1199 * s.dn[299][18]);
        let eq34_e1202_q_d_b0: f64 = (eq34_e1199 * s.db[299][0]);
        let eq34_e1202_q_d_b1: f64 = (eq34_e1199 * s.db[299][1]);
        let eq34_e1202_q_d_b2: f64 = (eq34_e1199 * s.db[299][2]);
        let eq34_e1202_q_d_b3: f64 = (eq34_e1199 * s.db[299][3]);
        let eq34_e1202_q_d_b4: f64 = (eq34_e1199 * s.db[299][4]);
        let eq34_e1202_q_d_b5: f64 = (eq34_e1199 * s.db[299][5]);
        let eq34_e1202_q_d_b6: f64 = (eq34_e1199 * s.db[299][6]);
        let eq34_e1202_q_d_b7: f64 = (eq34_e1199 * s.db[299][7]);
        let eq34_e1202_q_d_b8: f64 = (eq34_e1199 * s.db[299][8]);
        let eq34_e1202_q_d_b9: f64 = (eq34_e1199 * s.db[299][9]);
        let eq34_reactive_node_derivatives: [f64; 19] = [eq34_e1202_q_d_n0, eq34_e1202_q_d_n1, eq34_e1202_q_d_n2, eq34_e1202_q_d_n3, eq34_e1202_q_d_n4, eq34_e1202_q_d_n5, eq34_e1202_q_d_n6, eq34_e1202_q_d_n7, eq34_e1202_q_d_n8, eq34_e1202_q_d_n9, eq34_e1202_q_d_n10, eq34_e1202_q_d_n11, eq34_e1202_q_d_n12, eq34_e1202_q_d_n13, eq34_e1202_q_d_n14, eq34_e1202_q_d_n15, eq34_e1202_q_d_n16, eq34_e1202_q_d_n17, eq34_e1202_q_d_n18];
        let eq34_reactive_branch_derivatives: [f64; 10] = [eq34_e1202_q_d_b0, eq34_e1202_q_d_b1, eq34_e1202_q_d_b2, eq34_e1202_q_d_b3, eq34_e1202_q_d_b4, eq34_e1202_q_d_b5, eq34_e1202_q_d_b6, eq34_e1202_q_d_b7, eq34_e1202_q_d_b8, eq34_e1202_q_d_b9];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[0]),
            &nodes,
            &eq34_reactive_node_derivatives,
            &branches,
            &eq34_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_35_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq35_e1204: f64 = (-p.p87);
        let eq35_e1206_q: f64 = s.v[301];
        let eq35_e1207: f64 = (eq35_e1204 * s.v[301]);
        let eq35_e1207_d_n0: f64 = (eq35_e1204 * s.dn[301][0]);
        let eq35_e1207_d_n1: f64 = (eq35_e1204 * s.dn[301][1]);
        let eq35_e1207_d_n2: f64 = (eq35_e1204 * s.dn[301][2]);
        let eq35_e1207_d_n3: f64 = (eq35_e1204 * s.dn[301][3]);
        let eq35_e1207_d_n4: f64 = (eq35_e1204 * s.dn[301][4]);
        let eq35_e1207_d_n5: f64 = (eq35_e1204 * s.dn[301][5]);
        let eq35_e1207_d_n6: f64 = (eq35_e1204 * s.dn[301][6]);
        let eq35_e1207_d_n7: f64 = (eq35_e1204 * s.dn[301][7]);
        let eq35_e1207_d_n8: f64 = (eq35_e1204 * s.dn[301][8]);
        let eq35_e1207_d_n9: f64 = (eq35_e1204 * s.dn[301][9]);
        let eq35_e1207_d_n10: f64 = (eq35_e1204 * s.dn[301][10]);
        let eq35_e1207_d_n11: f64 = (eq35_e1204 * s.dn[301][11]);
        let eq35_e1207_d_n12: f64 = (eq35_e1204 * s.dn[301][12]);
        let eq35_e1207_d_n13: f64 = (eq35_e1204 * s.dn[301][13]);
        let eq35_e1207_d_n14: f64 = (eq35_e1204 * s.dn[301][14]);
        let eq35_e1207_d_n15: f64 = (eq35_e1204 * s.dn[301][15]);
        let eq35_e1207_d_n16: f64 = (eq35_e1204 * s.dn[301][16]);
        let eq35_e1207_d_n17: f64 = (eq35_e1204 * s.dn[301][17]);
        let eq35_e1207_d_n18: f64 = (eq35_e1204 * s.dn[301][18]);
        let eq35_e1207_d_b0: f64 = (eq35_e1204 * s.db[301][0]);
        let eq35_e1207_d_b1: f64 = (eq35_e1204 * s.db[301][1]);
        let eq35_e1207_d_b2: f64 = (eq35_e1204 * s.db[301][2]);
        let eq35_e1207_d_b3: f64 = (eq35_e1204 * s.db[301][3]);
        let eq35_e1207_d_b4: f64 = (eq35_e1204 * s.db[301][4]);
        let eq35_e1207_d_b5: f64 = (eq35_e1204 * s.db[301][5]);
        let eq35_e1207_d_b6: f64 = (eq35_e1204 * s.db[301][6]);
        let eq35_e1207_d_b7: f64 = (eq35_e1204 * s.db[301][7]);
        let eq35_e1207_d_b8: f64 = (eq35_e1204 * s.db[301][8]);
        let eq35_e1207_d_b9: f64 = (eq35_e1204 * s.db[301][9]);
        let eq35_e1207_q: f64 = (eq35_e1204 * eq35_e1206_q);
        let eq35_e1207_q_d_n0: f64 = (eq35_e1204 * s.dn[301][0]);
        let eq35_e1207_q_d_n1: f64 = (eq35_e1204 * s.dn[301][1]);
        let eq35_e1207_q_d_n2: f64 = (eq35_e1204 * s.dn[301][2]);
        let eq35_e1207_q_d_n3: f64 = (eq35_e1204 * s.dn[301][3]);
        let eq35_e1207_q_d_n4: f64 = (eq35_e1204 * s.dn[301][4]);
        let eq35_e1207_q_d_n5: f64 = (eq35_e1204 * s.dn[301][5]);
        let eq35_e1207_q_d_n6: f64 = (eq35_e1204 * s.dn[301][6]);
        let eq35_e1207_q_d_n7: f64 = (eq35_e1204 * s.dn[301][7]);
        let eq35_e1207_q_d_n8: f64 = (eq35_e1204 * s.dn[301][8]);
        let eq35_e1207_q_d_n9: f64 = (eq35_e1204 * s.dn[301][9]);
        let eq35_e1207_q_d_n10: f64 = (eq35_e1204 * s.dn[301][10]);
        let eq35_e1207_q_d_n11: f64 = (eq35_e1204 * s.dn[301][11]);
        let eq35_e1207_q_d_n12: f64 = (eq35_e1204 * s.dn[301][12]);
        let eq35_e1207_q_d_n13: f64 = (eq35_e1204 * s.dn[301][13]);
        let eq35_e1207_q_d_n14: f64 = (eq35_e1204 * s.dn[301][14]);
        let eq35_e1207_q_d_n15: f64 = (eq35_e1204 * s.dn[301][15]);
        let eq35_e1207_q_d_n16: f64 = (eq35_e1204 * s.dn[301][16]);
        let eq35_e1207_q_d_n17: f64 = (eq35_e1204 * s.dn[301][17]);
        let eq35_e1207_q_d_n18: f64 = (eq35_e1204 * s.dn[301][18]);
        let eq35_e1207_q_d_b0: f64 = (eq35_e1204 * s.db[301][0]);
        let eq35_e1207_q_d_b1: f64 = (eq35_e1204 * s.db[301][1]);
        let eq35_e1207_q_d_b2: f64 = (eq35_e1204 * s.db[301][2]);
        let eq35_e1207_q_d_b3: f64 = (eq35_e1204 * s.db[301][3]);
        let eq35_e1207_q_d_b4: f64 = (eq35_e1204 * s.db[301][4]);
        let eq35_e1207_q_d_b5: f64 = (eq35_e1204 * s.db[301][5]);
        let eq35_e1207_q_d_b6: f64 = (eq35_e1204 * s.db[301][6]);
        let eq35_e1207_q_d_b7: f64 = (eq35_e1204 * s.db[301][7]);
        let eq35_e1207_q_d_b8: f64 = (eq35_e1204 * s.db[301][8]);
        let eq35_e1207_q_d_b9: f64 = (eq35_e1204 * s.db[301][9]);
        let eq35_reactive_node_derivatives: [f64; 19] = [eq35_e1207_q_d_n0, eq35_e1207_q_d_n1, eq35_e1207_q_d_n2, eq35_e1207_q_d_n3, eq35_e1207_q_d_n4, eq35_e1207_q_d_n5, eq35_e1207_q_d_n6, eq35_e1207_q_d_n7, eq35_e1207_q_d_n8, eq35_e1207_q_d_n9, eq35_e1207_q_d_n10, eq35_e1207_q_d_n11, eq35_e1207_q_d_n12, eq35_e1207_q_d_n13, eq35_e1207_q_d_n14, eq35_e1207_q_d_n15, eq35_e1207_q_d_n16, eq35_e1207_q_d_n17, eq35_e1207_q_d_n18];
        let eq35_reactive_branch_derivatives: [f64; 10] = [eq35_e1207_q_d_b0, eq35_e1207_q_d_b1, eq35_e1207_q_d_b2, eq35_e1207_q_d_b3, eq35_e1207_q_d_b4, eq35_e1207_q_d_b5, eq35_e1207_q_d_b6, eq35_e1207_q_d_b7, eq35_e1207_q_d_b8, eq35_e1207_q_d_b9];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[2]),
            &nodes,
            &eq35_reactive_node_derivatives,
            &branches,
            &eq35_reactive_branch_derivatives,
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let eq41_e1236: f64 = ((nv15 - 0.0) * s.v[954]);
        let eq41_e1236_d_n0: f64 = ((nv15 - 0.0) * s.dn[954][0]);
        let eq41_e1236_d_n1: f64 = ((nv15 - 0.0) * s.dn[954][1]);
        let eq41_e1236_d_n2: f64 = ((nv15 - 0.0) * s.dn[954][2]);
        let eq41_e1236_d_n3: f64 = ((nv15 - 0.0) * s.dn[954][3]);
        let eq41_e1236_d_n4: f64 = ((nv15 - 0.0) * s.dn[954][4]);
        let eq41_e1236_d_n5: f64 = ((nv15 - 0.0) * s.dn[954][5]);
        let eq41_e1236_d_n6: f64 = ((nv15 - 0.0) * s.dn[954][6]);
        let eq41_e1236_d_n7: f64 = ((nv15 - 0.0) * s.dn[954][7]);
        let eq41_e1236_d_n8: f64 = ((nv15 - 0.0) * s.dn[954][8]);
        let eq41_e1236_d_n9: f64 = ((nv15 - 0.0) * s.dn[954][9]);
        let eq41_e1236_d_n10: f64 = ((nv15 - 0.0) * s.dn[954][10]);
        let eq41_e1236_d_n11: f64 = ((nv15 - 0.0) * s.dn[954][11]);
        let eq41_e1236_d_n12: f64 = ((nv15 - 0.0) * s.dn[954][12]);
        let eq41_e1236_d_n13: f64 = ((nv15 - 0.0) * s.dn[954][13]);
        let eq41_e1236_d_n14: f64 = ((nv15 - 0.0) * s.dn[954][14]);
        let eq41_e1236_d_n15: f64 = (s.v[954] + ((nv15 - 0.0) * s.dn[954][15]));
        let eq41_e1236_d_n16: f64 = ((nv15 - 0.0) * s.dn[954][16]);
        let eq41_e1236_d_n17: f64 = ((nv15 - 0.0) * s.dn[954][17]);
        let eq41_e1236_d_n18: f64 = ((nv15 - 0.0) * s.dn[954][18]);
        let eq41_e1236_d_b0: f64 = ((nv15 - 0.0) * s.db[954][0]);
        let eq41_e1236_d_b1: f64 = ((nv15 - 0.0) * s.db[954][1]);
        let eq41_e1236_d_b2: f64 = ((nv15 - 0.0) * s.db[954][2]);
        let eq41_e1236_d_b3: f64 = ((nv15 - 0.0) * s.db[954][3]);
        let eq41_e1236_d_b4: f64 = ((nv15 - 0.0) * s.db[954][4]);
        let eq41_e1236_d_b5: f64 = ((nv15 - 0.0) * s.db[954][5]);
        let eq41_e1236_d_b6: f64 = ((nv15 - 0.0) * s.db[954][6]);
        let eq41_e1236_d_b7: f64 = ((nv15 - 0.0) * s.db[954][7]);
        let eq41_e1236_d_b8: f64 = ((nv15 - 0.0) * s.db[954][8]);
        let eq41_e1236_d_b9: f64 = ((nv15 - 0.0) * s.db[954][9]);
        let eq41_e1237_q: f64 = eq41_e1236;
        let eq41_reactive_node_derivatives: [f64; 19] = [eq41_e1236_d_n0, eq41_e1236_d_n1, eq41_e1236_d_n2, eq41_e1236_d_n3, eq41_e1236_d_n4, eq41_e1236_d_n5, eq41_e1236_d_n6, eq41_e1236_d_n7, eq41_e1236_d_n8, eq41_e1236_d_n9, eq41_e1236_d_n10, eq41_e1236_d_n11, eq41_e1236_d_n12, eq41_e1236_d_n13, eq41_e1236_d_n14, eq41_e1236_d_n15, eq41_e1236_d_n16, eq41_e1236_d_n17, eq41_e1236_d_n18];
        let eq41_reactive_branch_derivatives: [f64; 10] = [eq41_e1236_d_b0, eq41_e1236_d_b1, eq41_e1236_d_b2, eq41_e1236_d_b3, eq41_e1236_d_b4, eq41_e1236_d_b5, eq41_e1236_d_b6, eq41_e1236_d_b7, eq41_e1236_d_b8, eq41_e1236_d_b9];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[8]),
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let eq42_e1240: f64 = ((nv15 - 0.0) * s.v[955]);
        let eq42_e1240_d_n0: f64 = ((nv15 - 0.0) * s.dn[955][0]);
        let eq42_e1240_d_n1: f64 = ((nv15 - 0.0) * s.dn[955][1]);
        let eq42_e1240_d_n2: f64 = ((nv15 - 0.0) * s.dn[955][2]);
        let eq42_e1240_d_n3: f64 = ((nv15 - 0.0) * s.dn[955][3]);
        let eq42_e1240_d_n4: f64 = ((nv15 - 0.0) * s.dn[955][4]);
        let eq42_e1240_d_n5: f64 = ((nv15 - 0.0) * s.dn[955][5]);
        let eq42_e1240_d_n6: f64 = ((nv15 - 0.0) * s.dn[955][6]);
        let eq42_e1240_d_n7: f64 = ((nv15 - 0.0) * s.dn[955][7]);
        let eq42_e1240_d_n8: f64 = ((nv15 - 0.0) * s.dn[955][8]);
        let eq42_e1240_d_n9: f64 = ((nv15 - 0.0) * s.dn[955][9]);
        let eq42_e1240_d_n10: f64 = ((nv15 - 0.0) * s.dn[955][10]);
        let eq42_e1240_d_n11: f64 = ((nv15 - 0.0) * s.dn[955][11]);
        let eq42_e1240_d_n12: f64 = ((nv15 - 0.0) * s.dn[955][12]);
        let eq42_e1240_d_n13: f64 = ((nv15 - 0.0) * s.dn[955][13]);
        let eq42_e1240_d_n14: f64 = ((nv15 - 0.0) * s.dn[955][14]);
        let eq42_e1240_d_n15: f64 = (s.v[955] + ((nv15 - 0.0) * s.dn[955][15]));
        let eq42_e1240_d_n16: f64 = ((nv15 - 0.0) * s.dn[955][16]);
        let eq42_e1240_d_n17: f64 = ((nv15 - 0.0) * s.dn[955][17]);
        let eq42_e1240_d_n18: f64 = ((nv15 - 0.0) * s.dn[955][18]);
        let eq42_e1240_d_b0: f64 = ((nv15 - 0.0) * s.db[955][0]);
        let eq42_e1240_d_b1: f64 = ((nv15 - 0.0) * s.db[955][1]);
        let eq42_e1240_d_b2: f64 = ((nv15 - 0.0) * s.db[955][2]);
        let eq42_e1240_d_b3: f64 = ((nv15 - 0.0) * s.db[955][3]);
        let eq42_e1240_d_b4: f64 = ((nv15 - 0.0) * s.db[955][4]);
        let eq42_e1240_d_b5: f64 = ((nv15 - 0.0) * s.db[955][5]);
        let eq42_e1240_d_b6: f64 = ((nv15 - 0.0) * s.db[955][6]);
        let eq42_e1240_d_b7: f64 = ((nv15 - 0.0) * s.db[955][7]);
        let eq42_e1240_d_b8: f64 = ((nv15 - 0.0) * s.db[955][8]);
        let eq42_e1240_d_b9: f64 = ((nv15 - 0.0) * s.db[955][9]);
        let eq42_e1241_q: f64 = eq42_e1240;
        let eq42_reactive_node_derivatives: [f64; 19] = [eq42_e1240_d_n0, eq42_e1240_d_n1, eq42_e1240_d_n2, eq42_e1240_d_n3, eq42_e1240_d_n4, eq42_e1240_d_n5, eq42_e1240_d_n6, eq42_e1240_d_n7, eq42_e1240_d_n8, eq42_e1240_d_n9, eq42_e1240_d_n10, eq42_e1240_d_n11, eq42_e1240_d_n12, eq42_e1240_d_n13, eq42_e1240_d_n14, eq42_e1240_d_n15, eq42_e1240_d_n16, eq42_e1240_d_n17, eq42_e1240_d_n18];
        let eq42_reactive_branch_derivatives: [f64; 10] = [eq42_e1240_d_b0, eq42_e1240_d_b1, eq42_e1240_d_b2, eq42_e1240_d_b3, eq42_e1240_d_b4, eq42_e1240_d_b5, eq42_e1240_d_b6, eq42_e1240_d_b7, eq42_e1240_d_b8, eq42_e1240_d_b9];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            &nodes,
            &eq42_reactive_node_derivatives,
            &branches,
            &eq42_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_59_block_0(
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
        let eq59_e1346: f64 = (s.v[767] * (nv5 - 0.0));
        let eq59_e1346_d_n0: f64 = (s.dn[767][0] * (nv5 - 0.0));
        let eq59_e1346_d_n1: f64 = (s.dn[767][1] * (nv5 - 0.0));
        let eq59_e1346_d_n2: f64 = (s.dn[767][2] * (nv5 - 0.0));
        let eq59_e1346_d_n3: f64 = (s.dn[767][3] * (nv5 - 0.0));
        let eq59_e1346_d_n4: f64 = (s.dn[767][4] * (nv5 - 0.0));
        let eq59_e1346_d_n5: f64 = ((s.dn[767][5] * (nv5 - 0.0)) + s.v[767]);
        let eq59_e1346_d_n6: f64 = (s.dn[767][6] * (nv5 - 0.0));
        let eq59_e1346_d_n7: f64 = (s.dn[767][7] * (nv5 - 0.0));
        let eq59_e1346_d_n8: f64 = (s.dn[767][8] * (nv5 - 0.0));
        let eq59_e1346_d_n9: f64 = (s.dn[767][9] * (nv5 - 0.0));
        let eq59_e1346_d_n10: f64 = (s.dn[767][10] * (nv5 - 0.0));
        let eq59_e1346_d_n11: f64 = (s.dn[767][11] * (nv5 - 0.0));
        let eq59_e1346_d_n12: f64 = (s.dn[767][12] * (nv5 - 0.0));
        let eq59_e1346_d_n13: f64 = (s.dn[767][13] * (nv5 - 0.0));
        let eq59_e1346_d_n14: f64 = (s.dn[767][14] * (nv5 - 0.0));
        let eq59_e1346_d_n15: f64 = (s.dn[767][15] * (nv5 - 0.0));
        let eq59_e1346_d_n16: f64 = (s.dn[767][16] * (nv5 - 0.0));
        let eq59_e1346_d_n17: f64 = (s.dn[767][17] * (nv5 - 0.0));
        let eq59_e1346_d_n18: f64 = (s.dn[767][18] * (nv5 - 0.0));
        let eq59_e1346_d_b0: f64 = (s.db[767][0] * (nv5 - 0.0));
        let eq59_e1346_d_b1: f64 = (s.db[767][1] * (nv5 - 0.0));
        let eq59_e1346_d_b2: f64 = (s.db[767][2] * (nv5 - 0.0));
        let eq59_e1346_d_b3: f64 = (s.db[767][3] * (nv5 - 0.0));
        let eq59_e1346_d_b4: f64 = (s.db[767][4] * (nv5 - 0.0));
        let eq59_e1346_d_b5: f64 = (s.db[767][5] * (nv5 - 0.0));
        let eq59_e1346_d_b6: f64 = (s.db[767][6] * (nv5 - 0.0));
        let eq59_e1346_d_b7: f64 = (s.db[767][7] * (nv5 - 0.0));
        let eq59_e1346_d_b8: f64 = (s.db[767][8] * (nv5 - 0.0));
        let eq59_e1346_d_b9: f64 = (s.db[767][9] * (nv5 - 0.0));
        let eq59_e1347_q: f64 = eq59_e1346;
        let eq59_reactive_node_derivatives: [f64; 19] = [eq59_e1346_d_n0, eq59_e1346_d_n1, eq59_e1346_d_n2, eq59_e1346_d_n3, eq59_e1346_d_n4, eq59_e1346_d_n5, eq59_e1346_d_n6, eq59_e1346_d_n7, eq59_e1346_d_n8, eq59_e1346_d_n9, eq59_e1346_d_n10, eq59_e1346_d_n11, eq59_e1346_d_n12, eq59_e1346_d_n13, eq59_e1346_d_n14, eq59_e1346_d_n15, eq59_e1346_d_n16, eq59_e1346_d_n17, eq59_e1346_d_n18];
        let eq59_reactive_branch_derivatives: [f64; 10] = [eq59_e1346_d_b0, eq59_e1346_d_b1, eq59_e1346_d_b2, eq59_e1346_d_b3, eq59_e1346_d_b4, eq59_e1346_d_b5, eq59_e1346_d_b6, eq59_e1346_d_b7, eq59_e1346_d_b8, eq59_e1346_d_b9];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            &nodes,
            &eq59_reactive_node_derivatives,
            &branches,
            &eq59_reactive_branch_derivatives,
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
        let (eq62_e1362, eq62_e1362_d_n0, eq62_e1362_d_n1, eq62_e1362_d_n2, eq62_e1362_d_n3, eq62_e1362_d_n4, eq62_e1362_d_n5, eq62_e1362_d_n6, eq62_e1362_d_n7, eq62_e1362_d_n8, eq62_e1362_d_n9, eq62_e1362_d_n10, eq62_e1362_d_n11, eq62_e1362_d_n12, eq62_e1362_d_n13, eq62_e1362_d_n14, eq62_e1362_d_n15, eq62_e1362_d_n16, eq62_e1362_d_n17, eq62_e1362_d_n18, eq62_e1362_d_b0, eq62_e1362_d_b1, eq62_e1362_d_b2, eq62_e1362_d_b3, eq62_e1362_d_b4, eq62_e1362_d_b5, eq62_e1362_d_b6, eq62_e1362_d_b7, eq62_e1362_d_b8, eq62_e1362_d_b9, eq62_e1362_q, eq62_e1362_q_d_n0, eq62_e1362_q_d_n1, eq62_e1362_q_d_n2, eq62_e1362_q_d_n3, eq62_e1362_q_d_n4, eq62_e1362_q_d_n5, eq62_e1362_q_d_n6, eq62_e1362_q_d_n7, eq62_e1362_q_d_n8, eq62_e1362_q_d_n9, eq62_e1362_q_d_n10, eq62_e1362_q_d_n11, eq62_e1362_q_d_n12, eq62_e1362_q_d_n13, eq62_e1362_q_d_n14, eq62_e1362_q_d_n15, eq62_e1362_q_d_n16, eq62_e1362_q_d_n17, eq62_e1362_q_d_n18, eq62_e1362_q_d_b0, eq62_e1362_q_d_b1, eq62_e1362_q_d_b2, eq62_e1362_q_d_b3, eq62_e1362_q_d_b4, eq62_e1362_q_d_b5, eq62_e1362_q_d_b6, eq62_e1362_q_d_b7, eq62_e1362_q_d_b8, eq62_e1362_q_d_b9,) = {
    if (p.p28 != 0.0) {
        let eq62_e1359: f64 = (s.v[800] * (nv12 - 0.0));
        let eq62_e1359_d_n0: f64 = (s.dn[800][0] * (nv12 - 0.0));
        let eq62_e1359_d_n1: f64 = (s.dn[800][1] * (nv12 - 0.0));
        let eq62_e1359_d_n2: f64 = (s.dn[800][2] * (nv12 - 0.0));
        let eq62_e1359_d_n3: f64 = (s.dn[800][3] * (nv12 - 0.0));
        let eq62_e1359_d_n4: f64 = (s.dn[800][4] * (nv12 - 0.0));
        let eq62_e1359_d_n5: f64 = (s.dn[800][5] * (nv12 - 0.0));
        let eq62_e1359_d_n6: f64 = (s.dn[800][6] * (nv12 - 0.0));
        let eq62_e1359_d_n7: f64 = (s.dn[800][7] * (nv12 - 0.0));
        let eq62_e1359_d_n8: f64 = (s.dn[800][8] * (nv12 - 0.0));
        let eq62_e1359_d_n9: f64 = (s.dn[800][9] * (nv12 - 0.0));
        let eq62_e1359_d_n10: f64 = (s.dn[800][10] * (nv12 - 0.0));
        let eq62_e1359_d_n11: f64 = (s.dn[800][11] * (nv12 - 0.0));
        let eq62_e1359_d_n12: f64 = ((s.dn[800][12] * (nv12 - 0.0)) + s.v[800]);
        let eq62_e1359_d_n13: f64 = (s.dn[800][13] * (nv12 - 0.0));
        let eq62_e1359_d_n14: f64 = (s.dn[800][14] * (nv12 - 0.0));
        let eq62_e1359_d_n15: f64 = (s.dn[800][15] * (nv12 - 0.0));
        let eq62_e1359_d_n16: f64 = (s.dn[800][16] * (nv12 - 0.0));
        let eq62_e1359_d_n17: f64 = (s.dn[800][17] * (nv12 - 0.0));
        let eq62_e1359_d_n18: f64 = (s.dn[800][18] * (nv12 - 0.0));
        let eq62_e1359_d_b0: f64 = (s.db[800][0] * (nv12 - 0.0));
        let eq62_e1359_d_b1: f64 = (s.db[800][1] * (nv12 - 0.0));
        let eq62_e1359_d_b2: f64 = (s.db[800][2] * (nv12 - 0.0));
        let eq62_e1359_d_b3: f64 = (s.db[800][3] * (nv12 - 0.0));
        let eq62_e1359_d_b4: f64 = (s.db[800][4] * (nv12 - 0.0));
        let eq62_e1359_d_b5: f64 = (s.db[800][5] * (nv12 - 0.0));
        let eq62_e1359_d_b6: f64 = (s.db[800][6] * (nv12 - 0.0));
        let eq62_e1359_d_b7: f64 = (s.db[800][7] * (nv12 - 0.0));
        let eq62_e1359_d_b8: f64 = (s.db[800][8] * (nv12 - 0.0));
        let eq62_e1359_d_b9: f64 = (s.db[800][9] * (nv12 - 0.0));
        let eq62_e1360_q: f64 = eq62_e1359;
        (eq62_e1359, eq62_e1359_d_n0, eq62_e1359_d_n1, eq62_e1359_d_n2, eq62_e1359_d_n3, eq62_e1359_d_n4, eq62_e1359_d_n5, eq62_e1359_d_n6, eq62_e1359_d_n7, eq62_e1359_d_n8, eq62_e1359_d_n9, eq62_e1359_d_n10, eq62_e1359_d_n11, eq62_e1359_d_n12, eq62_e1359_d_n13, eq62_e1359_d_n14, eq62_e1359_d_n15, eq62_e1359_d_n16, eq62_e1359_d_n17, eq62_e1359_d_n18, eq62_e1359_d_b0, eq62_e1359_d_b1, eq62_e1359_d_b2, eq62_e1359_d_b3, eq62_e1359_d_b4, eq62_e1359_d_b5, eq62_e1359_d_b6, eq62_e1359_d_b7, eq62_e1359_d_b8, eq62_e1359_d_b9, eq62_e1360_q, eq62_e1359_d_n0, eq62_e1359_d_n1, eq62_e1359_d_n2, eq62_e1359_d_n3, eq62_e1359_d_n4, eq62_e1359_d_n5, eq62_e1359_d_n6, eq62_e1359_d_n7, eq62_e1359_d_n8, eq62_e1359_d_n9, eq62_e1359_d_n10, eq62_e1359_d_n11, eq62_e1359_d_n12, eq62_e1359_d_n13, eq62_e1359_d_n14, eq62_e1359_d_n15, eq62_e1359_d_n16, eq62_e1359_d_n17, eq62_e1359_d_n18, eq62_e1359_d_b0, eq62_e1359_d_b1, eq62_e1359_d_b2, eq62_e1359_d_b3, eq62_e1359_d_b4, eq62_e1359_d_b5, eq62_e1359_d_b6, eq62_e1359_d_b7, eq62_e1359_d_b8, eq62_e1359_d_b9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_reactive_node_derivatives: [f64; 19] = [eq62_e1362_q_d_n0, eq62_e1362_q_d_n1, eq62_e1362_q_d_n2, eq62_e1362_q_d_n3, eq62_e1362_q_d_n4, eq62_e1362_q_d_n5, eq62_e1362_q_d_n6, eq62_e1362_q_d_n7, eq62_e1362_q_d_n8, eq62_e1362_q_d_n9, eq62_e1362_q_d_n10, eq62_e1362_q_d_n11, eq62_e1362_q_d_n12, eq62_e1362_q_d_n13, eq62_e1362_q_d_n14, eq62_e1362_q_d_n15, eq62_e1362_q_d_n16, eq62_e1362_q_d_n17, eq62_e1362_q_d_n18];
        let eq62_reactive_branch_derivatives: [f64; 10] = [eq62_e1362_q_d_b0, eq62_e1362_q_d_b1, eq62_e1362_q_d_b2, eq62_e1362_q_d_b3, eq62_e1362_q_d_b4, eq62_e1362_q_d_b5, eq62_e1362_q_d_b6, eq62_e1362_q_d_b7, eq62_e1362_q_d_b8, eq62_e1362_q_d_b9];
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

    pub(super) fn stamp_reactive_equation_63_block_0(
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
        let (eq63_e1369, eq63_e1369_d_n0, eq63_e1369_d_n1, eq63_e1369_d_n2, eq63_e1369_d_n3, eq63_e1369_d_n4, eq63_e1369_d_n5, eq63_e1369_d_n6, eq63_e1369_d_n7, eq63_e1369_d_n8, eq63_e1369_d_n9, eq63_e1369_d_n10, eq63_e1369_d_n11, eq63_e1369_d_n12, eq63_e1369_d_n13, eq63_e1369_d_n14, eq63_e1369_d_n15, eq63_e1369_d_n16, eq63_e1369_d_n17, eq63_e1369_d_n18, eq63_e1369_d_b0, eq63_e1369_d_b1, eq63_e1369_d_b2, eq63_e1369_d_b3, eq63_e1369_d_b4, eq63_e1369_d_b5, eq63_e1369_d_b6, eq63_e1369_d_b7, eq63_e1369_d_b8, eq63_e1369_d_b9, eq63_e1369_q, eq63_e1369_q_d_n0, eq63_e1369_q_d_n1, eq63_e1369_q_d_n2, eq63_e1369_q_d_n3, eq63_e1369_q_d_n4, eq63_e1369_q_d_n5, eq63_e1369_q_d_n6, eq63_e1369_q_d_n7, eq63_e1369_q_d_n8, eq63_e1369_q_d_n9, eq63_e1369_q_d_n10, eq63_e1369_q_d_n11, eq63_e1369_q_d_n12, eq63_e1369_q_d_n13, eq63_e1369_q_d_n14, eq63_e1369_q_d_n15, eq63_e1369_q_d_n16, eq63_e1369_q_d_n17, eq63_e1369_q_d_n18, eq63_e1369_q_d_b0, eq63_e1369_q_d_b1, eq63_e1369_q_d_b2, eq63_e1369_q_d_b3, eq63_e1369_q_d_b4, eq63_e1369_q_d_b5, eq63_e1369_q_d_b6, eq63_e1369_q_d_b7, eq63_e1369_q_d_b8, eq63_e1369_q_d_b9,) = {
    if (p.p28 != 0.0) {
        let eq63_e1366: f64 = (s.v[801] * (nv13 - 0.0));
        let eq63_e1366_d_n0: f64 = (s.dn[801][0] * (nv13 - 0.0));
        let eq63_e1366_d_n1: f64 = (s.dn[801][1] * (nv13 - 0.0));
        let eq63_e1366_d_n2: f64 = (s.dn[801][2] * (nv13 - 0.0));
        let eq63_e1366_d_n3: f64 = (s.dn[801][3] * (nv13 - 0.0));
        let eq63_e1366_d_n4: f64 = (s.dn[801][4] * (nv13 - 0.0));
        let eq63_e1366_d_n5: f64 = (s.dn[801][5] * (nv13 - 0.0));
        let eq63_e1366_d_n6: f64 = (s.dn[801][6] * (nv13 - 0.0));
        let eq63_e1366_d_n7: f64 = (s.dn[801][7] * (nv13 - 0.0));
        let eq63_e1366_d_n8: f64 = (s.dn[801][8] * (nv13 - 0.0));
        let eq63_e1366_d_n9: f64 = (s.dn[801][9] * (nv13 - 0.0));
        let eq63_e1366_d_n10: f64 = (s.dn[801][10] * (nv13 - 0.0));
        let eq63_e1366_d_n11: f64 = (s.dn[801][11] * (nv13 - 0.0));
        let eq63_e1366_d_n12: f64 = (s.dn[801][12] * (nv13 - 0.0));
        let eq63_e1366_d_n13: f64 = ((s.dn[801][13] * (nv13 - 0.0)) + s.v[801]);
        let eq63_e1366_d_n14: f64 = (s.dn[801][14] * (nv13 - 0.0));
        let eq63_e1366_d_n15: f64 = (s.dn[801][15] * (nv13 - 0.0));
        let eq63_e1366_d_n16: f64 = (s.dn[801][16] * (nv13 - 0.0));
        let eq63_e1366_d_n17: f64 = (s.dn[801][17] * (nv13 - 0.0));
        let eq63_e1366_d_n18: f64 = (s.dn[801][18] * (nv13 - 0.0));
        let eq63_e1366_d_b0: f64 = (s.db[801][0] * (nv13 - 0.0));
        let eq63_e1366_d_b1: f64 = (s.db[801][1] * (nv13 - 0.0));
        let eq63_e1366_d_b2: f64 = (s.db[801][2] * (nv13 - 0.0));
        let eq63_e1366_d_b3: f64 = (s.db[801][3] * (nv13 - 0.0));
        let eq63_e1366_d_b4: f64 = (s.db[801][4] * (nv13 - 0.0));
        let eq63_e1366_d_b5: f64 = (s.db[801][5] * (nv13 - 0.0));
        let eq63_e1366_d_b6: f64 = (s.db[801][6] * (nv13 - 0.0));
        let eq63_e1366_d_b7: f64 = (s.db[801][7] * (nv13 - 0.0));
        let eq63_e1366_d_b8: f64 = (s.db[801][8] * (nv13 - 0.0));
        let eq63_e1366_d_b9: f64 = (s.db[801][9] * (nv13 - 0.0));
        let eq63_e1367_q: f64 = eq63_e1366;
        (eq63_e1366, eq63_e1366_d_n0, eq63_e1366_d_n1, eq63_e1366_d_n2, eq63_e1366_d_n3, eq63_e1366_d_n4, eq63_e1366_d_n5, eq63_e1366_d_n6, eq63_e1366_d_n7, eq63_e1366_d_n8, eq63_e1366_d_n9, eq63_e1366_d_n10, eq63_e1366_d_n11, eq63_e1366_d_n12, eq63_e1366_d_n13, eq63_e1366_d_n14, eq63_e1366_d_n15, eq63_e1366_d_n16, eq63_e1366_d_n17, eq63_e1366_d_n18, eq63_e1366_d_b0, eq63_e1366_d_b1, eq63_e1366_d_b2, eq63_e1366_d_b3, eq63_e1366_d_b4, eq63_e1366_d_b5, eq63_e1366_d_b6, eq63_e1366_d_b7, eq63_e1366_d_b8, eq63_e1366_d_b9, eq63_e1367_q, eq63_e1366_d_n0, eq63_e1366_d_n1, eq63_e1366_d_n2, eq63_e1366_d_n3, eq63_e1366_d_n4, eq63_e1366_d_n5, eq63_e1366_d_n6, eq63_e1366_d_n7, eq63_e1366_d_n8, eq63_e1366_d_n9, eq63_e1366_d_n10, eq63_e1366_d_n11, eq63_e1366_d_n12, eq63_e1366_d_n13, eq63_e1366_d_n14, eq63_e1366_d_n15, eq63_e1366_d_n16, eq63_e1366_d_n17, eq63_e1366_d_n18, eq63_e1366_d_b0, eq63_e1366_d_b1, eq63_e1366_d_b2, eq63_e1366_d_b3, eq63_e1366_d_b4, eq63_e1366_d_b5, eq63_e1366_d_b6, eq63_e1366_d_b7, eq63_e1366_d_b8, eq63_e1366_d_b9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_reactive_node_derivatives: [f64; 19] = [eq63_e1369_q_d_n0, eq63_e1369_q_d_n1, eq63_e1369_q_d_n2, eq63_e1369_q_d_n3, eq63_e1369_q_d_n4, eq63_e1369_q_d_n5, eq63_e1369_q_d_n6, eq63_e1369_q_d_n7, eq63_e1369_q_d_n8, eq63_e1369_q_d_n9, eq63_e1369_q_d_n10, eq63_e1369_q_d_n11, eq63_e1369_q_d_n12, eq63_e1369_q_d_n13, eq63_e1369_q_d_n14, eq63_e1369_q_d_n15, eq63_e1369_q_d_n16, eq63_e1369_q_d_n17, eq63_e1369_q_d_n18];
        let eq63_reactive_branch_derivatives: [f64; 10] = [eq63_e1369_q_d_b0, eq63_e1369_q_d_b1, eq63_e1369_q_d_b2, eq63_e1369_q_d_b3, eq63_e1369_q_d_b4, eq63_e1369_q_d_b5, eq63_e1369_q_d_b6, eq63_e1369_q_d_b7, eq63_e1369_q_d_b8, eq63_e1369_q_d_b9];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            None,
            &nodes,
            &eq63_reactive_node_derivatives,
            &branches,
            &eq63_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_67_block_0(
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
        let (eq67_e1388, eq67_e1388_d_n14, eq67_e1388_q, eq67_e1388_q_d_n14,) = {
    if (p.p29 != 0.0) {
        let eq67_e1386_q: f64 = (nv14 - 0.0);
        ((nv14 - 0.0), 1.0, eq67_e1386_q, 1.0,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[14]),
            None,
            &[
                GeneratedDerivative::node(nodes[14], self.multiplicity * (eq67_e1388_q_d_n14)),
            ],
        );
    }
}
