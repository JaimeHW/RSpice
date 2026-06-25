#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_66_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq66_e1384, eq66_e1384_d_n0, eq66_e1384_d_n1, eq66_e1384_d_n2, eq66_e1384_d_n3, eq66_e1384_d_n4, eq66_e1384_d_n5, eq66_e1384_d_n6, eq66_e1384_d_n7, eq66_e1384_d_n8, eq66_e1384_d_n9, eq66_e1384_d_n10, eq66_e1384_d_n11, eq66_e1384_d_n12, eq66_e1384_d_n13, eq66_e1384_d_n14, eq66_e1384_d_n15, eq66_e1384_d_n16, eq66_e1384_d_n17, eq66_e1384_d_b0, eq66_e1384_d_b1, eq66_e1384_d_b2, eq66_e1384_d_b3, eq66_e1384_d_b4, eq66_e1384_d_b5, eq66_e1384_d_b6, eq66_e1384_d_b7, eq66_e1384_d_b8,) = {
    if (p.p29 != 0.0) {
        let eq66_e1382: f64 = self.eval_ddt(20, (nv13 - 0.0));
        let eq66_e1382_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq66_e1382_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq66_e1382_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq66_e1382_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq66_e1382_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq66_e1382_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq66_e1382_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq66_e1382_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq66_e1382_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq66_e1382_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq66_e1382_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq66_e1382_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq66_e1382_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq66_e1382_d_n13: f64 = self.ddt_jacobian(1.0);
        let eq66_e1382_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq66_e1382_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq66_e1382_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq66_e1382_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq66_e1382_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq66_e1382_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq66_e1382_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq66_e1382_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq66_e1382_d_b4: f64 = self.ddt_jacobian(0.0);
        let eq66_e1382_d_b5: f64 = self.ddt_jacobian(0.0);
        let eq66_e1382_d_b6: f64 = self.ddt_jacobian(0.0);
        let eq66_e1382_d_b7: f64 = self.ddt_jacobian(0.0);
        let eq66_e1382_d_b8: f64 = self.ddt_jacobian(0.0);
        (eq66_e1382, eq66_e1382_d_n0, eq66_e1382_d_n1, eq66_e1382_d_n2, eq66_e1382_d_n3, eq66_e1382_d_n4, eq66_e1382_d_n5, eq66_e1382_d_n6, eq66_e1382_d_n7, eq66_e1382_d_n8, eq66_e1382_d_n9, eq66_e1382_d_n10, eq66_e1382_d_n11, eq66_e1382_d_n12, eq66_e1382_d_n13, eq66_e1382_d_n14, eq66_e1382_d_n15, eq66_e1382_d_n16, eq66_e1382_d_n17, eq66_e1382_d_b0, eq66_e1382_d_b1, eq66_e1382_d_b2, eq66_e1382_d_b3, eq66_e1382_d_b4, eq66_e1382_d_b5, eq66_e1382_d_b6, eq66_e1382_d_b7, eq66_e1382_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq66_value: f64 = eq66_e1384;
        let eq66_node_derivatives: [f64; 18] = [eq66_e1384_d_n0, eq66_e1384_d_n1, eq66_e1384_d_n2, eq66_e1384_d_n3, eq66_e1384_d_n4, eq66_e1384_d_n5, eq66_e1384_d_n6, eq66_e1384_d_n7, eq66_e1384_d_n8, eq66_e1384_d_n9, eq66_e1384_d_n10, eq66_e1384_d_n11, eq66_e1384_d_n12, eq66_e1384_d_n13, eq66_e1384_d_n14, eq66_e1384_d_n15, eq66_e1384_d_n16, eq66_e1384_d_n17];
        let eq66_branch_derivatives: [f64; 9] = [eq66_e1384_d_b0, eq66_e1384_d_b1, eq66_e1384_d_b2, eq66_e1384_d_b3, eq66_e1384_d_b4, eq66_e1384_d_b5, eq66_e1384_d_b6, eq66_e1384_d_b7, eq66_e1384_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            None,
            self.multiplicity * (eq66_value),
            &nodes,
            &eq66_node_derivatives,
            &branches,
            &eq66_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_67_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq67_e1389,) = {
    if (!(p.p29 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq67_value: f64 = eq67_e1389;
        stamper.stamp_potential(
            branches[11],
            eq67_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_0_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq0_e1018, eq0_e1018_d_n0, eq0_e1018_d_n1, eq0_e1018_d_n2, eq0_e1018_d_n3, eq0_e1018_d_n4, eq0_e1018_d_n5, eq0_e1018_d_n6, eq0_e1018_d_n7, eq0_e1018_d_n8, eq0_e1018_d_n9, eq0_e1018_d_n10, eq0_e1018_d_n11, eq0_e1018_d_n12, eq0_e1018_d_n13, eq0_e1018_d_n14, eq0_e1018_d_n15, eq0_e1018_d_n16, eq0_e1018_d_n17, eq0_e1018_d_b0, eq0_e1018_d_b1, eq0_e1018_d_b2, eq0_e1018_d_b3, eq0_e1018_d_b4, eq0_e1018_d_b5, eq0_e1018_d_b6, eq0_e1018_d_b7, eq0_e1018_d_b8, eq0_e1018_q, eq0_e1018_q_d_n0, eq0_e1018_q_d_n1, eq0_e1018_q_d_n2, eq0_e1018_q_d_n3, eq0_e1018_q_d_n4, eq0_e1018_q_d_n5, eq0_e1018_q_d_n6, eq0_e1018_q_d_n7, eq0_e1018_q_d_n8, eq0_e1018_q_d_n9, eq0_e1018_q_d_n10, eq0_e1018_q_d_n11, eq0_e1018_q_d_n12, eq0_e1018_q_d_n13, eq0_e1018_q_d_n14, eq0_e1018_q_d_n15, eq0_e1018_q_d_n16, eq0_e1018_q_d_n17, eq0_e1018_q_d_b0, eq0_e1018_q_d_b1, eq0_e1018_q_d_b2, eq0_e1018_q_d_b3, eq0_e1018_q_d_b4, eq0_e1018_q_d_b5, eq0_e1018_q_d_b6, eq0_e1018_q_d_b7, eq0_e1018_q_d_b8,) = {
    if (s.v[3305] != 0.0) {
        let eq0_e1015_q: f64 = s.v[924];
        let eq0_e1016: f64 = (s.v[926] + s.v[924]);
        let eq0_e1016_d_n0: f64 = (s.dn[926][0] + s.dn[924][0]);
        let eq0_e1016_d_n1: f64 = (s.dn[926][1] + s.dn[924][1]);
        let eq0_e1016_d_n2: f64 = (s.dn[926][2] + s.dn[924][2]);
        let eq0_e1016_d_n3: f64 = (s.dn[926][3] + s.dn[924][3]);
        let eq0_e1016_d_n4: f64 = (s.dn[926][4] + s.dn[924][4]);
        let eq0_e1016_d_n5: f64 = (s.dn[926][5] + s.dn[924][5]);
        let eq0_e1016_d_n6: f64 = (s.dn[926][6] + s.dn[924][6]);
        let eq0_e1016_d_n7: f64 = (s.dn[926][7] + s.dn[924][7]);
        let eq0_e1016_d_n8: f64 = (s.dn[926][8] + s.dn[924][8]);
        let eq0_e1016_d_n9: f64 = (s.dn[926][9] + s.dn[924][9]);
        let eq0_e1016_d_n10: f64 = (s.dn[926][10] + s.dn[924][10]);
        let eq0_e1016_d_n11: f64 = (s.dn[926][11] + s.dn[924][11]);
        let eq0_e1016_d_n12: f64 = (s.dn[926][12] + s.dn[924][12]);
        let eq0_e1016_d_n13: f64 = (s.dn[926][13] + s.dn[924][13]);
        let eq0_e1016_d_n14: f64 = (s.dn[926][14] + s.dn[924][14]);
        let eq0_e1016_d_n15: f64 = (s.dn[926][15] + s.dn[924][15]);
        let eq0_e1016_d_n16: f64 = (s.dn[926][16] + s.dn[924][16]);
        let eq0_e1016_d_n17: f64 = (s.dn[926][17] + s.dn[924][17]);
        let eq0_e1016_d_b0: f64 = (s.db[926][0] + s.db[924][0]);
        let eq0_e1016_d_b1: f64 = (s.db[926][1] + s.db[924][1]);
        let eq0_e1016_d_b2: f64 = (s.db[926][2] + s.db[924][2]);
        let eq0_e1016_d_b3: f64 = (s.db[926][3] + s.db[924][3]);
        let eq0_e1016_d_b4: f64 = (s.db[926][4] + s.db[924][4]);
        let eq0_e1016_d_b5: f64 = (s.db[926][5] + s.db[924][5]);
        let eq0_e1016_d_b6: f64 = (s.db[926][6] + s.db[924][6]);
        let eq0_e1016_d_b7: f64 = (s.db[926][7] + s.db[924][7]);
        let eq0_e1016_d_b8: f64 = (s.db[926][8] + s.db[924][8]);
        let eq0_e1016_q: f64 = eq0_e1015_q;
        (eq0_e1016, eq0_e1016_d_n0, eq0_e1016_d_n1, eq0_e1016_d_n2, eq0_e1016_d_n3, eq0_e1016_d_n4, eq0_e1016_d_n5, eq0_e1016_d_n6, eq0_e1016_d_n7, eq0_e1016_d_n8, eq0_e1016_d_n9, eq0_e1016_d_n10, eq0_e1016_d_n11, eq0_e1016_d_n12, eq0_e1016_d_n13, eq0_e1016_d_n14, eq0_e1016_d_n15, eq0_e1016_d_n16, eq0_e1016_d_n17, eq0_e1016_d_b0, eq0_e1016_d_b1, eq0_e1016_d_b2, eq0_e1016_d_b3, eq0_e1016_d_b4, eq0_e1016_d_b5, eq0_e1016_d_b6, eq0_e1016_d_b7, eq0_e1016_d_b8, eq0_e1016_q, s.dn[924][0], s.dn[924][1], s.dn[924][2], s.dn[924][3], s.dn[924][4], s.dn[924][5], s.dn[924][6], s.dn[924][7], s.dn[924][8], s.dn[924][9], s.dn[924][10], s.dn[924][11], s.dn[924][12], s.dn[924][13], s.dn[924][14], s.dn[924][15], s.dn[924][16], s.dn[924][17], s.db[924][0], s.db[924][1], s.db[924][2], s.db[924][3], s.db[924][4], s.db[924][5], s.db[924][6], s.db[924][7], s.db[924][8],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_reactive_node_derivatives: [f64; 18] = [eq0_e1018_q_d_n0, eq0_e1018_q_d_n1, eq0_e1018_q_d_n2, eq0_e1018_q_d_n3, eq0_e1018_q_d_n4, eq0_e1018_q_d_n5, eq0_e1018_q_d_n6, eq0_e1018_q_d_n7, eq0_e1018_q_d_n8, eq0_e1018_q_d_n9, eq0_e1018_q_d_n10, eq0_e1018_q_d_n11, eq0_e1018_q_d_n12, eq0_e1018_q_d_n13, eq0_e1018_q_d_n14, eq0_e1018_q_d_n15, eq0_e1018_q_d_n16, eq0_e1018_q_d_n17];
        let eq0_reactive_branch_derivatives: [f64; 9] = [eq0_e1018_q_d_b0, eq0_e1018_q_d_b1, eq0_e1018_q_d_b2, eq0_e1018_q_d_b3, eq0_e1018_q_d_b4, eq0_e1018_q_d_b5, eq0_e1018_q_d_b6, eq0_e1018_q_d_b7, eq0_e1018_q_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[15]),
            None,
            &nodes,
            &eq0_reactive_node_derivatives,
            &branches,
            &eq0_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_1_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq1_e1025, eq1_e1025_d_n0, eq1_e1025_d_n1, eq1_e1025_d_n2, eq1_e1025_d_n3, eq1_e1025_d_n4, eq1_e1025_d_n5, eq1_e1025_d_n6, eq1_e1025_d_n7, eq1_e1025_d_n8, eq1_e1025_d_n9, eq1_e1025_d_n10, eq1_e1025_d_n11, eq1_e1025_d_n12, eq1_e1025_d_n13, eq1_e1025_d_n14, eq1_e1025_d_n15, eq1_e1025_d_n16, eq1_e1025_d_n17, eq1_e1025_d_b0, eq1_e1025_d_b1, eq1_e1025_d_b2, eq1_e1025_d_b3, eq1_e1025_d_b4, eq1_e1025_d_b5, eq1_e1025_d_b6, eq1_e1025_d_b7, eq1_e1025_d_b8, eq1_e1025_q, eq1_e1025_q_d_n0, eq1_e1025_q_d_n1, eq1_e1025_q_d_n2, eq1_e1025_q_d_n3, eq1_e1025_q_d_n4, eq1_e1025_q_d_n5, eq1_e1025_q_d_n6, eq1_e1025_q_d_n7, eq1_e1025_q_d_n8, eq1_e1025_q_d_n9, eq1_e1025_q_d_n10, eq1_e1025_q_d_n11, eq1_e1025_q_d_n12, eq1_e1025_q_d_n13, eq1_e1025_q_d_n14, eq1_e1025_q_d_n15, eq1_e1025_q_d_n16, eq1_e1025_q_d_n17, eq1_e1025_q_d_b0, eq1_e1025_q_d_b1, eq1_e1025_q_d_b2, eq1_e1025_q_d_b3, eq1_e1025_q_d_b4, eq1_e1025_q_d_b5, eq1_e1025_q_d_b6, eq1_e1025_q_d_b7, eq1_e1025_q_d_b8,) = {
    if (s.v[3305] != 0.0) {
        let eq1_e1022_q: f64 = s.v[925];
        let eq1_e1023: f64 = (s.v[927] + s.v[925]);
        let eq1_e1023_d_n0: f64 = (s.dn[927][0] + s.dn[925][0]);
        let eq1_e1023_d_n1: f64 = (s.dn[927][1] + s.dn[925][1]);
        let eq1_e1023_d_n2: f64 = (s.dn[927][2] + s.dn[925][2]);
        let eq1_e1023_d_n3: f64 = (s.dn[927][3] + s.dn[925][3]);
        let eq1_e1023_d_n4: f64 = (s.dn[927][4] + s.dn[925][4]);
        let eq1_e1023_d_n5: f64 = (s.dn[927][5] + s.dn[925][5]);
        let eq1_e1023_d_n6: f64 = (s.dn[927][6] + s.dn[925][6]);
        let eq1_e1023_d_n7: f64 = (s.dn[927][7] + s.dn[925][7]);
        let eq1_e1023_d_n8: f64 = (s.dn[927][8] + s.dn[925][8]);
        let eq1_e1023_d_n9: f64 = (s.dn[927][9] + s.dn[925][9]);
        let eq1_e1023_d_n10: f64 = (s.dn[927][10] + s.dn[925][10]);
        let eq1_e1023_d_n11: f64 = (s.dn[927][11] + s.dn[925][11]);
        let eq1_e1023_d_n12: f64 = (s.dn[927][12] + s.dn[925][12]);
        let eq1_e1023_d_n13: f64 = (s.dn[927][13] + s.dn[925][13]);
        let eq1_e1023_d_n14: f64 = (s.dn[927][14] + s.dn[925][14]);
        let eq1_e1023_d_n15: f64 = (s.dn[927][15] + s.dn[925][15]);
        let eq1_e1023_d_n16: f64 = (s.dn[927][16] + s.dn[925][16]);
        let eq1_e1023_d_n17: f64 = (s.dn[927][17] + s.dn[925][17]);
        let eq1_e1023_d_b0: f64 = (s.db[927][0] + s.db[925][0]);
        let eq1_e1023_d_b1: f64 = (s.db[927][1] + s.db[925][1]);
        let eq1_e1023_d_b2: f64 = (s.db[927][2] + s.db[925][2]);
        let eq1_e1023_d_b3: f64 = (s.db[927][3] + s.db[925][3]);
        let eq1_e1023_d_b4: f64 = (s.db[927][4] + s.db[925][4]);
        let eq1_e1023_d_b5: f64 = (s.db[927][5] + s.db[925][5]);
        let eq1_e1023_d_b6: f64 = (s.db[927][6] + s.db[925][6]);
        let eq1_e1023_d_b7: f64 = (s.db[927][7] + s.db[925][7]);
        let eq1_e1023_d_b8: f64 = (s.db[927][8] + s.db[925][8]);
        let eq1_e1023_q: f64 = eq1_e1022_q;
        (eq1_e1023, eq1_e1023_d_n0, eq1_e1023_d_n1, eq1_e1023_d_n2, eq1_e1023_d_n3, eq1_e1023_d_n4, eq1_e1023_d_n5, eq1_e1023_d_n6, eq1_e1023_d_n7, eq1_e1023_d_n8, eq1_e1023_d_n9, eq1_e1023_d_n10, eq1_e1023_d_n11, eq1_e1023_d_n12, eq1_e1023_d_n13, eq1_e1023_d_n14, eq1_e1023_d_n15, eq1_e1023_d_n16, eq1_e1023_d_n17, eq1_e1023_d_b0, eq1_e1023_d_b1, eq1_e1023_d_b2, eq1_e1023_d_b3, eq1_e1023_d_b4, eq1_e1023_d_b5, eq1_e1023_d_b6, eq1_e1023_d_b7, eq1_e1023_d_b8, eq1_e1023_q, s.dn[925][0], s.dn[925][1], s.dn[925][2], s.dn[925][3], s.dn[925][4], s.dn[925][5], s.dn[925][6], s.dn[925][7], s.dn[925][8], s.dn[925][9], s.dn[925][10], s.dn[925][11], s.dn[925][12], s.dn[925][13], s.dn[925][14], s.dn[925][15], s.dn[925][16], s.dn[925][17], s.db[925][0], s.db[925][1], s.db[925][2], s.db[925][3], s.db[925][4], s.db[925][5], s.db[925][6], s.db[925][7], s.db[925][8],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_reactive_node_derivatives: [f64; 18] = [eq1_e1025_q_d_n0, eq1_e1025_q_d_n1, eq1_e1025_q_d_n2, eq1_e1025_q_d_n3, eq1_e1025_q_d_n4, eq1_e1025_q_d_n5, eq1_e1025_q_d_n6, eq1_e1025_q_d_n7, eq1_e1025_q_d_n8, eq1_e1025_q_d_n9, eq1_e1025_q_d_n10, eq1_e1025_q_d_n11, eq1_e1025_q_d_n12, eq1_e1025_q_d_n13, eq1_e1025_q_d_n14, eq1_e1025_q_d_n15, eq1_e1025_q_d_n16, eq1_e1025_q_d_n17];
        let eq1_reactive_branch_derivatives: [f64; 9] = [eq1_e1025_q_d_b0, eq1_e1025_q_d_b1, eq1_e1025_q_d_b2, eq1_e1025_q_d_b3, eq1_e1025_q_d_b4, eq1_e1025_q_d_b5, eq1_e1025_q_d_b6, eq1_e1025_q_d_b7, eq1_e1025_q_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[16]),
            None,
            &nodes,
            &eq1_reactive_node_derivatives,
            &branches,
            &eq1_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_4_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq4_e1042, eq4_e1042_d_n0, eq4_e1042_d_n1, eq4_e1042_d_n2, eq4_e1042_d_n3, eq4_e1042_d_n4, eq4_e1042_d_n5, eq4_e1042_d_n6, eq4_e1042_d_n7, eq4_e1042_d_n8, eq4_e1042_d_n9, eq4_e1042_d_n10, eq4_e1042_d_n11, eq4_e1042_d_n12, eq4_e1042_d_n13, eq4_e1042_d_n14, eq4_e1042_d_n15, eq4_e1042_d_n16, eq4_e1042_d_n17, eq4_e1042_d_b0, eq4_e1042_d_b1, eq4_e1042_d_b2, eq4_e1042_d_b3, eq4_e1042_d_b4, eq4_e1042_d_b5, eq4_e1042_d_b6, eq4_e1042_d_b7, eq4_e1042_d_b8, eq4_e1042_q, eq4_e1042_q_d_n0, eq4_e1042_q_d_n1, eq4_e1042_q_d_n2, eq4_e1042_q_d_n3, eq4_e1042_q_d_n4, eq4_e1042_q_d_n5, eq4_e1042_q_d_n6, eq4_e1042_q_d_n7, eq4_e1042_q_d_n8, eq4_e1042_q_d_n9, eq4_e1042_q_d_n10, eq4_e1042_q_d_n11, eq4_e1042_q_d_n12, eq4_e1042_q_d_n13, eq4_e1042_q_d_n14, eq4_e1042_q_d_n15, eq4_e1042_q_d_n16, eq4_e1042_q_d_n17, eq4_e1042_q_d_b0, eq4_e1042_q_d_b1, eq4_e1042_q_d_b2, eq4_e1042_q_d_b3, eq4_e1042_q_d_b4, eq4_e1042_q_d_b5, eq4_e1042_q_d_b6, eq4_e1042_q_d_b7, eq4_e1042_q_d_b8,) = {
    if (s.v[3306] != 0.0) {
        let eq4_e1039_q: f64 = s.v[931];
        let eq4_e1040: f64 = (s.v[932] + s.v[931]);
        let eq4_e1040_d_n0: f64 = (s.dn[932][0] + s.dn[931][0]);
        let eq4_e1040_d_n1: f64 = (s.dn[932][1] + s.dn[931][1]);
        let eq4_e1040_d_n2: f64 = (s.dn[932][2] + s.dn[931][2]);
        let eq4_e1040_d_n3: f64 = (s.dn[932][3] + s.dn[931][3]);
        let eq4_e1040_d_n4: f64 = (s.dn[932][4] + s.dn[931][4]);
        let eq4_e1040_d_n5: f64 = (s.dn[932][5] + s.dn[931][5]);
        let eq4_e1040_d_n6: f64 = (s.dn[932][6] + s.dn[931][6]);
        let eq4_e1040_d_n7: f64 = (s.dn[932][7] + s.dn[931][7]);
        let eq4_e1040_d_n8: f64 = (s.dn[932][8] + s.dn[931][8]);
        let eq4_e1040_d_n9: f64 = (s.dn[932][9] + s.dn[931][9]);
        let eq4_e1040_d_n10: f64 = (s.dn[932][10] + s.dn[931][10]);
        let eq4_e1040_d_n11: f64 = (s.dn[932][11] + s.dn[931][11]);
        let eq4_e1040_d_n12: f64 = (s.dn[932][12] + s.dn[931][12]);
        let eq4_e1040_d_n13: f64 = (s.dn[932][13] + s.dn[931][13]);
        let eq4_e1040_d_n14: f64 = (s.dn[932][14] + s.dn[931][14]);
        let eq4_e1040_d_n15: f64 = (s.dn[932][15] + s.dn[931][15]);
        let eq4_e1040_d_n16: f64 = (s.dn[932][16] + s.dn[931][16]);
        let eq4_e1040_d_n17: f64 = (s.dn[932][17] + s.dn[931][17]);
        let eq4_e1040_d_b0: f64 = (s.db[932][0] + s.db[931][0]);
        let eq4_e1040_d_b1: f64 = (s.db[932][1] + s.db[931][1]);
        let eq4_e1040_d_b2: f64 = (s.db[932][2] + s.db[931][2]);
        let eq4_e1040_d_b3: f64 = (s.db[932][3] + s.db[931][3]);
        let eq4_e1040_d_b4: f64 = (s.db[932][4] + s.db[931][4]);
        let eq4_e1040_d_b5: f64 = (s.db[932][5] + s.db[931][5]);
        let eq4_e1040_d_b6: f64 = (s.db[932][6] + s.db[931][6]);
        let eq4_e1040_d_b7: f64 = (s.db[932][7] + s.db[931][7]);
        let eq4_e1040_d_b8: f64 = (s.db[932][8] + s.db[931][8]);
        let eq4_e1040_q: f64 = eq4_e1039_q;
        (eq4_e1040, eq4_e1040_d_n0, eq4_e1040_d_n1, eq4_e1040_d_n2, eq4_e1040_d_n3, eq4_e1040_d_n4, eq4_e1040_d_n5, eq4_e1040_d_n6, eq4_e1040_d_n7, eq4_e1040_d_n8, eq4_e1040_d_n9, eq4_e1040_d_n10, eq4_e1040_d_n11, eq4_e1040_d_n12, eq4_e1040_d_n13, eq4_e1040_d_n14, eq4_e1040_d_n15, eq4_e1040_d_n16, eq4_e1040_d_n17, eq4_e1040_d_b0, eq4_e1040_d_b1, eq4_e1040_d_b2, eq4_e1040_d_b3, eq4_e1040_d_b4, eq4_e1040_d_b5, eq4_e1040_d_b6, eq4_e1040_d_b7, eq4_e1040_d_b8, eq4_e1040_q, s.dn[931][0], s.dn[931][1], s.dn[931][2], s.dn[931][3], s.dn[931][4], s.dn[931][5], s.dn[931][6], s.dn[931][7], s.dn[931][8], s.dn[931][9], s.dn[931][10], s.dn[931][11], s.dn[931][12], s.dn[931][13], s.dn[931][14], s.dn[931][15], s.dn[931][16], s.dn[931][17], s.db[931][0], s.db[931][1], s.db[931][2], s.db[931][3], s.db[931][4], s.db[931][5], s.db[931][6], s.db[931][7], s.db[931][8],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_reactive_node_derivatives: [f64; 18] = [eq4_e1042_q_d_n0, eq4_e1042_q_d_n1, eq4_e1042_q_d_n2, eq4_e1042_q_d_n3, eq4_e1042_q_d_n4, eq4_e1042_q_d_n5, eq4_e1042_q_d_n6, eq4_e1042_q_d_n7, eq4_e1042_q_d_n8, eq4_e1042_q_d_n9, eq4_e1042_q_d_n10, eq4_e1042_q_d_n11, eq4_e1042_q_d_n12, eq4_e1042_q_d_n13, eq4_e1042_q_d_n14, eq4_e1042_q_d_n15, eq4_e1042_q_d_n16, eq4_e1042_q_d_n17];
        let eq4_reactive_branch_derivatives: [f64; 9] = [eq4_e1042_q_d_b0, eq4_e1042_q_d_b1, eq4_e1042_q_d_b2, eq4_e1042_q_d_b3, eq4_e1042_q_d_b4, eq4_e1042_q_d_b5, eq4_e1042_q_d_b6, eq4_e1042_q_d_b7, eq4_e1042_q_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[17]),
            None,
            &nodes,
            &eq4_reactive_node_derivatives,
            &branches,
            &eq4_reactive_branch_derivatives,
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
        let eq14_e1088_q: f64 = s.v[66];
        let eq14_e1089: f64 = (p.p87 * s.v[66]);
        let eq14_e1089_d_n0: f64 = (p.p87 * s.dn[66][0]);
        let eq14_e1089_d_n1: f64 = (p.p87 * s.dn[66][1]);
        let eq14_e1089_d_n2: f64 = (p.p87 * s.dn[66][2]);
        let eq14_e1089_d_n3: f64 = (p.p87 * s.dn[66][3]);
        let eq14_e1089_d_n4: f64 = (p.p87 * s.dn[66][4]);
        let eq14_e1089_d_n5: f64 = (p.p87 * s.dn[66][5]);
        let eq14_e1089_d_n6: f64 = (p.p87 * s.dn[66][6]);
        let eq14_e1089_d_n7: f64 = (p.p87 * s.dn[66][7]);
        let eq14_e1089_d_n8: f64 = (p.p87 * s.dn[66][8]);
        let eq14_e1089_d_n9: f64 = (p.p87 * s.dn[66][9]);
        let eq14_e1089_d_n10: f64 = (p.p87 * s.dn[66][10]);
        let eq14_e1089_d_n11: f64 = (p.p87 * s.dn[66][11]);
        let eq14_e1089_d_n12: f64 = (p.p87 * s.dn[66][12]);
        let eq14_e1089_d_n13: f64 = (p.p87 * s.dn[66][13]);
        let eq14_e1089_d_n14: f64 = (p.p87 * s.dn[66][14]);
        let eq14_e1089_d_n15: f64 = (p.p87 * s.dn[66][15]);
        let eq14_e1089_d_n16: f64 = (p.p87 * s.dn[66][16]);
        let eq14_e1089_d_n17: f64 = (p.p87 * s.dn[66][17]);
        let eq14_e1089_d_b0: f64 = (p.p87 * s.db[66][0]);
        let eq14_e1089_d_b1: f64 = (p.p87 * s.db[66][1]);
        let eq14_e1089_d_b2: f64 = (p.p87 * s.db[66][2]);
        let eq14_e1089_d_b3: f64 = (p.p87 * s.db[66][3]);
        let eq14_e1089_d_b4: f64 = (p.p87 * s.db[66][4]);
        let eq14_e1089_d_b5: f64 = (p.p87 * s.db[66][5]);
        let eq14_e1089_d_b6: f64 = (p.p87 * s.db[66][6]);
        let eq14_e1089_d_b7: f64 = (p.p87 * s.db[66][7]);
        let eq14_e1089_d_b8: f64 = (p.p87 * s.db[66][8]);
        let eq14_e1089_q: f64 = (p.p87 * eq14_e1088_q);
        let eq14_e1089_q_d_n0: f64 = (p.p87 * s.dn[66][0]);
        let eq14_e1089_q_d_n1: f64 = (p.p87 * s.dn[66][1]);
        let eq14_e1089_q_d_n2: f64 = (p.p87 * s.dn[66][2]);
        let eq14_e1089_q_d_n3: f64 = (p.p87 * s.dn[66][3]);
        let eq14_e1089_q_d_n4: f64 = (p.p87 * s.dn[66][4]);
        let eq14_e1089_q_d_n5: f64 = (p.p87 * s.dn[66][5]);
        let eq14_e1089_q_d_n6: f64 = (p.p87 * s.dn[66][6]);
        let eq14_e1089_q_d_n7: f64 = (p.p87 * s.dn[66][7]);
        let eq14_e1089_q_d_n8: f64 = (p.p87 * s.dn[66][8]);
        let eq14_e1089_q_d_n9: f64 = (p.p87 * s.dn[66][9]);
        let eq14_e1089_q_d_n10: f64 = (p.p87 * s.dn[66][10]);
        let eq14_e1089_q_d_n11: f64 = (p.p87 * s.dn[66][11]);
        let eq14_e1089_q_d_n12: f64 = (p.p87 * s.dn[66][12]);
        let eq14_e1089_q_d_n13: f64 = (p.p87 * s.dn[66][13]);
        let eq14_e1089_q_d_n14: f64 = (p.p87 * s.dn[66][14]);
        let eq14_e1089_q_d_n15: f64 = (p.p87 * s.dn[66][15]);
        let eq14_e1089_q_d_n16: f64 = (p.p87 * s.dn[66][16]);
        let eq14_e1089_q_d_n17: f64 = (p.p87 * s.dn[66][17]);
        let eq14_e1089_q_d_b0: f64 = (p.p87 * s.db[66][0]);
        let eq14_e1089_q_d_b1: f64 = (p.p87 * s.db[66][1]);
        let eq14_e1089_q_d_b2: f64 = (p.p87 * s.db[66][2]);
        let eq14_e1089_q_d_b3: f64 = (p.p87 * s.db[66][3]);
        let eq14_e1089_q_d_b4: f64 = (p.p87 * s.db[66][4]);
        let eq14_e1089_q_d_b5: f64 = (p.p87 * s.db[66][5]);
        let eq14_e1089_q_d_b6: f64 = (p.p87 * s.db[66][6]);
        let eq14_e1089_q_d_b7: f64 = (p.p87 * s.db[66][7]);
        let eq14_e1089_q_d_b8: f64 = (p.p87 * s.db[66][8]);
        let eq14_reactive_node_derivatives: [f64; 18] = [eq14_e1089_q_d_n0, eq14_e1089_q_d_n1, eq14_e1089_q_d_n2, eq14_e1089_q_d_n3, eq14_e1089_q_d_n4, eq14_e1089_q_d_n5, eq14_e1089_q_d_n6, eq14_e1089_q_d_n7, eq14_e1089_q_d_n8, eq14_e1089_q_d_n9, eq14_e1089_q_d_n10, eq14_e1089_q_d_n11, eq14_e1089_q_d_n12, eq14_e1089_q_d_n13, eq14_e1089_q_d_n14, eq14_e1089_q_d_n15, eq14_e1089_q_d_n16, eq14_e1089_q_d_n17];
        let eq14_reactive_branch_derivatives: [f64; 9] = [eq14_e1089_q_d_b0, eq14_e1089_q_d_b1, eq14_e1089_q_d_b2, eq14_e1089_q_d_b3, eq14_e1089_q_d_b4, eq14_e1089_q_d_b5, eq14_e1089_q_d_b6, eq14_e1089_q_d_b7, eq14_e1089_q_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[2]),
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
        let eq15_e1092_q: f64 = s.v[65];
        let eq15_e1093: f64 = (p.p87 * s.v[65]);
        let eq15_e1093_d_n0: f64 = (p.p87 * s.dn[65][0]);
        let eq15_e1093_d_n1: f64 = (p.p87 * s.dn[65][1]);
        let eq15_e1093_d_n2: f64 = (p.p87 * s.dn[65][2]);
        let eq15_e1093_d_n3: f64 = (p.p87 * s.dn[65][3]);
        let eq15_e1093_d_n4: f64 = (p.p87 * s.dn[65][4]);
        let eq15_e1093_d_n5: f64 = (p.p87 * s.dn[65][5]);
        let eq15_e1093_d_n6: f64 = (p.p87 * s.dn[65][6]);
        let eq15_e1093_d_n7: f64 = (p.p87 * s.dn[65][7]);
        let eq15_e1093_d_n8: f64 = (p.p87 * s.dn[65][8]);
        let eq15_e1093_d_n9: f64 = (p.p87 * s.dn[65][9]);
        let eq15_e1093_d_n10: f64 = (p.p87 * s.dn[65][10]);
        let eq15_e1093_d_n11: f64 = (p.p87 * s.dn[65][11]);
        let eq15_e1093_d_n12: f64 = (p.p87 * s.dn[65][12]);
        let eq15_e1093_d_n13: f64 = (p.p87 * s.dn[65][13]);
        let eq15_e1093_d_n14: f64 = (p.p87 * s.dn[65][14]);
        let eq15_e1093_d_n15: f64 = (p.p87 * s.dn[65][15]);
        let eq15_e1093_d_n16: f64 = (p.p87 * s.dn[65][16]);
        let eq15_e1093_d_n17: f64 = (p.p87 * s.dn[65][17]);
        let eq15_e1093_d_b0: f64 = (p.p87 * s.db[65][0]);
        let eq15_e1093_d_b1: f64 = (p.p87 * s.db[65][1]);
        let eq15_e1093_d_b2: f64 = (p.p87 * s.db[65][2]);
        let eq15_e1093_d_b3: f64 = (p.p87 * s.db[65][3]);
        let eq15_e1093_d_b4: f64 = (p.p87 * s.db[65][4]);
        let eq15_e1093_d_b5: f64 = (p.p87 * s.db[65][5]);
        let eq15_e1093_d_b6: f64 = (p.p87 * s.db[65][6]);
        let eq15_e1093_d_b7: f64 = (p.p87 * s.db[65][7]);
        let eq15_e1093_d_b8: f64 = (p.p87 * s.db[65][8]);
        let eq15_e1093_q: f64 = (p.p87 * eq15_e1092_q);
        let eq15_e1093_q_d_n0: f64 = (p.p87 * s.dn[65][0]);
        let eq15_e1093_q_d_n1: f64 = (p.p87 * s.dn[65][1]);
        let eq15_e1093_q_d_n2: f64 = (p.p87 * s.dn[65][2]);
        let eq15_e1093_q_d_n3: f64 = (p.p87 * s.dn[65][3]);
        let eq15_e1093_q_d_n4: f64 = (p.p87 * s.dn[65][4]);
        let eq15_e1093_q_d_n5: f64 = (p.p87 * s.dn[65][5]);
        let eq15_e1093_q_d_n6: f64 = (p.p87 * s.dn[65][6]);
        let eq15_e1093_q_d_n7: f64 = (p.p87 * s.dn[65][7]);
        let eq15_e1093_q_d_n8: f64 = (p.p87 * s.dn[65][8]);
        let eq15_e1093_q_d_n9: f64 = (p.p87 * s.dn[65][9]);
        let eq15_e1093_q_d_n10: f64 = (p.p87 * s.dn[65][10]);
        let eq15_e1093_q_d_n11: f64 = (p.p87 * s.dn[65][11]);
        let eq15_e1093_q_d_n12: f64 = (p.p87 * s.dn[65][12]);
        let eq15_e1093_q_d_n13: f64 = (p.p87 * s.dn[65][13]);
        let eq15_e1093_q_d_n14: f64 = (p.p87 * s.dn[65][14]);
        let eq15_e1093_q_d_n15: f64 = (p.p87 * s.dn[65][15]);
        let eq15_e1093_q_d_n16: f64 = (p.p87 * s.dn[65][16]);
        let eq15_e1093_q_d_n17: f64 = (p.p87 * s.dn[65][17]);
        let eq15_e1093_q_d_b0: f64 = (p.p87 * s.db[65][0]);
        let eq15_e1093_q_d_b1: f64 = (p.p87 * s.db[65][1]);
        let eq15_e1093_q_d_b2: f64 = (p.p87 * s.db[65][2]);
        let eq15_e1093_q_d_b3: f64 = (p.p87 * s.db[65][3]);
        let eq15_e1093_q_d_b4: f64 = (p.p87 * s.db[65][4]);
        let eq15_e1093_q_d_b5: f64 = (p.p87 * s.db[65][5]);
        let eq15_e1093_q_d_b6: f64 = (p.p87 * s.db[65][6]);
        let eq15_e1093_q_d_b7: f64 = (p.p87 * s.db[65][7]);
        let eq15_e1093_q_d_b8: f64 = (p.p87 * s.db[65][8]);
        let eq15_reactive_node_derivatives: [f64; 18] = [eq15_e1093_q_d_n0, eq15_e1093_q_d_n1, eq15_e1093_q_d_n2, eq15_e1093_q_d_n3, eq15_e1093_q_d_n4, eq15_e1093_q_d_n5, eq15_e1093_q_d_n6, eq15_e1093_q_d_n7, eq15_e1093_q_d_n8, eq15_e1093_q_d_n9, eq15_e1093_q_d_n10, eq15_e1093_q_d_n11, eq15_e1093_q_d_n12, eq15_e1093_q_d_n13, eq15_e1093_q_d_n14, eq15_e1093_q_d_n15, eq15_e1093_q_d_n16, eq15_e1093_q_d_n17];
        let eq15_reactive_branch_derivatives: [f64; 9] = [eq15_e1093_q_d_b0, eq15_e1093_q_d_b1, eq15_e1093_q_d_b2, eq15_e1093_q_d_b3, eq15_e1093_q_d_b4, eq15_e1093_q_d_b5, eq15_e1093_q_d_b6, eq15_e1093_q_d_b7, eq15_e1093_q_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[0]),
            &nodes,
            &eq15_reactive_node_derivatives,
            &branches,
            &eq15_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_18_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq18_e1112, eq18_e1112_d_n0, eq18_e1112_d_n1, eq18_e1112_d_n2, eq18_e1112_d_n3, eq18_e1112_d_n4, eq18_e1112_d_n5, eq18_e1112_d_n6, eq18_e1112_d_n7, eq18_e1112_d_n8, eq18_e1112_d_n9, eq18_e1112_d_n10, eq18_e1112_d_n11, eq18_e1112_d_n12, eq18_e1112_d_n13, eq18_e1112_d_n14, eq18_e1112_d_n15, eq18_e1112_d_n16, eq18_e1112_d_n17, eq18_e1112_d_b0, eq18_e1112_d_b1, eq18_e1112_d_b2, eq18_e1112_d_b3, eq18_e1112_d_b4, eq18_e1112_d_b5, eq18_e1112_d_b6, eq18_e1112_d_b7, eq18_e1112_d_b8, eq18_e1112_q, eq18_e1112_q_d_n0, eq18_e1112_q_d_n1, eq18_e1112_q_d_n2, eq18_e1112_q_d_n3, eq18_e1112_q_d_n4, eq18_e1112_q_d_n5, eq18_e1112_q_d_n6, eq18_e1112_q_d_n7, eq18_e1112_q_d_n8, eq18_e1112_q_d_n9, eq18_e1112_q_d_n10, eq18_e1112_q_d_n11, eq18_e1112_q_d_n12, eq18_e1112_q_d_n13, eq18_e1112_q_d_n14, eq18_e1112_q_d_n15, eq18_e1112_q_d_n16, eq18_e1112_q_d_n17, eq18_e1112_q_d_b0, eq18_e1112_q_d_b1, eq18_e1112_q_d_b2, eq18_e1112_q_d_b3, eq18_e1112_q_d_b4, eq18_e1112_q_d_b5, eq18_e1112_q_d_b6, eq18_e1112_q_d_b7, eq18_e1112_q_d_b8,) = {
    if (s.v[3405] != 0.0) {
        let eq18_e1109_q: f64 = s.v[68];
        let eq18_e1110: f64 = (p.p87 * s.v[68]);
        let eq18_e1110_d_n0: f64 = (p.p87 * s.dn[68][0]);
        let eq18_e1110_d_n1: f64 = (p.p87 * s.dn[68][1]);
        let eq18_e1110_d_n2: f64 = (p.p87 * s.dn[68][2]);
        let eq18_e1110_d_n3: f64 = (p.p87 * s.dn[68][3]);
        let eq18_e1110_d_n4: f64 = (p.p87 * s.dn[68][4]);
        let eq18_e1110_d_n5: f64 = (p.p87 * s.dn[68][5]);
        let eq18_e1110_d_n6: f64 = (p.p87 * s.dn[68][6]);
        let eq18_e1110_d_n7: f64 = (p.p87 * s.dn[68][7]);
        let eq18_e1110_d_n8: f64 = (p.p87 * s.dn[68][8]);
        let eq18_e1110_d_n9: f64 = (p.p87 * s.dn[68][9]);
        let eq18_e1110_d_n10: f64 = (p.p87 * s.dn[68][10]);
        let eq18_e1110_d_n11: f64 = (p.p87 * s.dn[68][11]);
        let eq18_e1110_d_n12: f64 = (p.p87 * s.dn[68][12]);
        let eq18_e1110_d_n13: f64 = (p.p87 * s.dn[68][13]);
        let eq18_e1110_d_n14: f64 = (p.p87 * s.dn[68][14]);
        let eq18_e1110_d_n15: f64 = (p.p87 * s.dn[68][15]);
        let eq18_e1110_d_n16: f64 = (p.p87 * s.dn[68][16]);
        let eq18_e1110_d_n17: f64 = (p.p87 * s.dn[68][17]);
        let eq18_e1110_d_b0: f64 = (p.p87 * s.db[68][0]);
        let eq18_e1110_d_b1: f64 = (p.p87 * s.db[68][1]);
        let eq18_e1110_d_b2: f64 = (p.p87 * s.db[68][2]);
        let eq18_e1110_d_b3: f64 = (p.p87 * s.db[68][3]);
        let eq18_e1110_d_b4: f64 = (p.p87 * s.db[68][4]);
        let eq18_e1110_d_b5: f64 = (p.p87 * s.db[68][5]);
        let eq18_e1110_d_b6: f64 = (p.p87 * s.db[68][6]);
        let eq18_e1110_d_b7: f64 = (p.p87 * s.db[68][7]);
        let eq18_e1110_d_b8: f64 = (p.p87 * s.db[68][8]);
        let eq18_e1110_q: f64 = (p.p87 * eq18_e1109_q);
        let eq18_e1110_q_d_n0: f64 = (p.p87 * s.dn[68][0]);
        let eq18_e1110_q_d_n1: f64 = (p.p87 * s.dn[68][1]);
        let eq18_e1110_q_d_n2: f64 = (p.p87 * s.dn[68][2]);
        let eq18_e1110_q_d_n3: f64 = (p.p87 * s.dn[68][3]);
        let eq18_e1110_q_d_n4: f64 = (p.p87 * s.dn[68][4]);
        let eq18_e1110_q_d_n5: f64 = (p.p87 * s.dn[68][5]);
        let eq18_e1110_q_d_n6: f64 = (p.p87 * s.dn[68][6]);
        let eq18_e1110_q_d_n7: f64 = (p.p87 * s.dn[68][7]);
        let eq18_e1110_q_d_n8: f64 = (p.p87 * s.dn[68][8]);
        let eq18_e1110_q_d_n9: f64 = (p.p87 * s.dn[68][9]);
        let eq18_e1110_q_d_n10: f64 = (p.p87 * s.dn[68][10]);
        let eq18_e1110_q_d_n11: f64 = (p.p87 * s.dn[68][11]);
        let eq18_e1110_q_d_n12: f64 = (p.p87 * s.dn[68][12]);
        let eq18_e1110_q_d_n13: f64 = (p.p87 * s.dn[68][13]);
        let eq18_e1110_q_d_n14: f64 = (p.p87 * s.dn[68][14]);
        let eq18_e1110_q_d_n15: f64 = (p.p87 * s.dn[68][15]);
        let eq18_e1110_q_d_n16: f64 = (p.p87 * s.dn[68][16]);
        let eq18_e1110_q_d_n17: f64 = (p.p87 * s.dn[68][17]);
        let eq18_e1110_q_d_b0: f64 = (p.p87 * s.db[68][0]);
        let eq18_e1110_q_d_b1: f64 = (p.p87 * s.db[68][1]);
        let eq18_e1110_q_d_b2: f64 = (p.p87 * s.db[68][2]);
        let eq18_e1110_q_d_b3: f64 = (p.p87 * s.db[68][3]);
        let eq18_e1110_q_d_b4: f64 = (p.p87 * s.db[68][4]);
        let eq18_e1110_q_d_b5: f64 = (p.p87 * s.db[68][5]);
        let eq18_e1110_q_d_b6: f64 = (p.p87 * s.db[68][6]);
        let eq18_e1110_q_d_b7: f64 = (p.p87 * s.db[68][7]);
        let eq18_e1110_q_d_b8: f64 = (p.p87 * s.db[68][8]);
        (eq18_e1110, eq18_e1110_d_n0, eq18_e1110_d_n1, eq18_e1110_d_n2, eq18_e1110_d_n3, eq18_e1110_d_n4, eq18_e1110_d_n5, eq18_e1110_d_n6, eq18_e1110_d_n7, eq18_e1110_d_n8, eq18_e1110_d_n9, eq18_e1110_d_n10, eq18_e1110_d_n11, eq18_e1110_d_n12, eq18_e1110_d_n13, eq18_e1110_d_n14, eq18_e1110_d_n15, eq18_e1110_d_n16, eq18_e1110_d_n17, eq18_e1110_d_b0, eq18_e1110_d_b1, eq18_e1110_d_b2, eq18_e1110_d_b3, eq18_e1110_d_b4, eq18_e1110_d_b5, eq18_e1110_d_b6, eq18_e1110_d_b7, eq18_e1110_d_b8, eq18_e1110_q, eq18_e1110_q_d_n0, eq18_e1110_q_d_n1, eq18_e1110_q_d_n2, eq18_e1110_q_d_n3, eq18_e1110_q_d_n4, eq18_e1110_q_d_n5, eq18_e1110_q_d_n6, eq18_e1110_q_d_n7, eq18_e1110_q_d_n8, eq18_e1110_q_d_n9, eq18_e1110_q_d_n10, eq18_e1110_q_d_n11, eq18_e1110_q_d_n12, eq18_e1110_q_d_n13, eq18_e1110_q_d_n14, eq18_e1110_q_d_n15, eq18_e1110_q_d_n16, eq18_e1110_q_d_n17, eq18_e1110_q_d_b0, eq18_e1110_q_d_b1, eq18_e1110_q_d_b2, eq18_e1110_q_d_b3, eq18_e1110_q_d_b4, eq18_e1110_q_d_b5, eq18_e1110_q_d_b6, eq18_e1110_q_d_b7, eq18_e1110_q_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_reactive_node_derivatives: [f64; 18] = [eq18_e1112_q_d_n0, eq18_e1112_q_d_n1, eq18_e1112_q_d_n2, eq18_e1112_q_d_n3, eq18_e1112_q_d_n4, eq18_e1112_q_d_n5, eq18_e1112_q_d_n6, eq18_e1112_q_d_n7, eq18_e1112_q_d_n8, eq18_e1112_q_d_n9, eq18_e1112_q_d_n10, eq18_e1112_q_d_n11, eq18_e1112_q_d_n12, eq18_e1112_q_d_n13, eq18_e1112_q_d_n14, eq18_e1112_q_d_n15, eq18_e1112_q_d_n16, eq18_e1112_q_d_n17];
        let eq18_reactive_branch_derivatives: [f64; 9] = [eq18_e1112_q_d_b0, eq18_e1112_q_d_b1, eq18_e1112_q_d_b2, eq18_e1112_q_d_b3, eq18_e1112_q_d_b4, eq18_e1112_q_d_b5, eq18_e1112_q_d_b6, eq18_e1112_q_d_b7, eq18_e1112_q_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            &nodes,
            &eq18_reactive_node_derivatives,
            &branches,
            &eq18_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_19_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq19_e1119, eq19_e1119_d_n0, eq19_e1119_d_n1, eq19_e1119_d_n2, eq19_e1119_d_n3, eq19_e1119_d_n4, eq19_e1119_d_n5, eq19_e1119_d_n6, eq19_e1119_d_n7, eq19_e1119_d_n8, eq19_e1119_d_n9, eq19_e1119_d_n10, eq19_e1119_d_n11, eq19_e1119_d_n12, eq19_e1119_d_n13, eq19_e1119_d_n14, eq19_e1119_d_n15, eq19_e1119_d_n16, eq19_e1119_d_n17, eq19_e1119_d_b0, eq19_e1119_d_b1, eq19_e1119_d_b2, eq19_e1119_d_b3, eq19_e1119_d_b4, eq19_e1119_d_b5, eq19_e1119_d_b6, eq19_e1119_d_b7, eq19_e1119_d_b8, eq19_e1119_q, eq19_e1119_q_d_n0, eq19_e1119_q_d_n1, eq19_e1119_q_d_n2, eq19_e1119_q_d_n3, eq19_e1119_q_d_n4, eq19_e1119_q_d_n5, eq19_e1119_q_d_n6, eq19_e1119_q_d_n7, eq19_e1119_q_d_n8, eq19_e1119_q_d_n9, eq19_e1119_q_d_n10, eq19_e1119_q_d_n11, eq19_e1119_q_d_n12, eq19_e1119_q_d_n13, eq19_e1119_q_d_n14, eq19_e1119_q_d_n15, eq19_e1119_q_d_n16, eq19_e1119_q_d_n17, eq19_e1119_q_d_b0, eq19_e1119_q_d_b1, eq19_e1119_q_d_b2, eq19_e1119_q_d_b3, eq19_e1119_q_d_b4, eq19_e1119_q_d_b5, eq19_e1119_q_d_b6, eq19_e1119_q_d_b7, eq19_e1119_q_d_b8,) = {
    if (s.v[3405] != 0.0) {
        let eq19_e1116_q: f64 = s.v[67];
        let eq19_e1117: f64 = (p.p87 * s.v[67]);
        let eq19_e1117_d_n0: f64 = (p.p87 * s.dn[67][0]);
        let eq19_e1117_d_n1: f64 = (p.p87 * s.dn[67][1]);
        let eq19_e1117_d_n2: f64 = (p.p87 * s.dn[67][2]);
        let eq19_e1117_d_n3: f64 = (p.p87 * s.dn[67][3]);
        let eq19_e1117_d_n4: f64 = (p.p87 * s.dn[67][4]);
        let eq19_e1117_d_n5: f64 = (p.p87 * s.dn[67][5]);
        let eq19_e1117_d_n6: f64 = (p.p87 * s.dn[67][6]);
        let eq19_e1117_d_n7: f64 = (p.p87 * s.dn[67][7]);
        let eq19_e1117_d_n8: f64 = (p.p87 * s.dn[67][8]);
        let eq19_e1117_d_n9: f64 = (p.p87 * s.dn[67][9]);
        let eq19_e1117_d_n10: f64 = (p.p87 * s.dn[67][10]);
        let eq19_e1117_d_n11: f64 = (p.p87 * s.dn[67][11]);
        let eq19_e1117_d_n12: f64 = (p.p87 * s.dn[67][12]);
        let eq19_e1117_d_n13: f64 = (p.p87 * s.dn[67][13]);
        let eq19_e1117_d_n14: f64 = (p.p87 * s.dn[67][14]);
        let eq19_e1117_d_n15: f64 = (p.p87 * s.dn[67][15]);
        let eq19_e1117_d_n16: f64 = (p.p87 * s.dn[67][16]);
        let eq19_e1117_d_n17: f64 = (p.p87 * s.dn[67][17]);
        let eq19_e1117_d_b0: f64 = (p.p87 * s.db[67][0]);
        let eq19_e1117_d_b1: f64 = (p.p87 * s.db[67][1]);
        let eq19_e1117_d_b2: f64 = (p.p87 * s.db[67][2]);
        let eq19_e1117_d_b3: f64 = (p.p87 * s.db[67][3]);
        let eq19_e1117_d_b4: f64 = (p.p87 * s.db[67][4]);
        let eq19_e1117_d_b5: f64 = (p.p87 * s.db[67][5]);
        let eq19_e1117_d_b6: f64 = (p.p87 * s.db[67][6]);
        let eq19_e1117_d_b7: f64 = (p.p87 * s.db[67][7]);
        let eq19_e1117_d_b8: f64 = (p.p87 * s.db[67][8]);
        let eq19_e1117_q: f64 = (p.p87 * eq19_e1116_q);
        let eq19_e1117_q_d_n0: f64 = (p.p87 * s.dn[67][0]);
        let eq19_e1117_q_d_n1: f64 = (p.p87 * s.dn[67][1]);
        let eq19_e1117_q_d_n2: f64 = (p.p87 * s.dn[67][2]);
        let eq19_e1117_q_d_n3: f64 = (p.p87 * s.dn[67][3]);
        let eq19_e1117_q_d_n4: f64 = (p.p87 * s.dn[67][4]);
        let eq19_e1117_q_d_n5: f64 = (p.p87 * s.dn[67][5]);
        let eq19_e1117_q_d_n6: f64 = (p.p87 * s.dn[67][6]);
        let eq19_e1117_q_d_n7: f64 = (p.p87 * s.dn[67][7]);
        let eq19_e1117_q_d_n8: f64 = (p.p87 * s.dn[67][8]);
        let eq19_e1117_q_d_n9: f64 = (p.p87 * s.dn[67][9]);
        let eq19_e1117_q_d_n10: f64 = (p.p87 * s.dn[67][10]);
        let eq19_e1117_q_d_n11: f64 = (p.p87 * s.dn[67][11]);
        let eq19_e1117_q_d_n12: f64 = (p.p87 * s.dn[67][12]);
        let eq19_e1117_q_d_n13: f64 = (p.p87 * s.dn[67][13]);
        let eq19_e1117_q_d_n14: f64 = (p.p87 * s.dn[67][14]);
        let eq19_e1117_q_d_n15: f64 = (p.p87 * s.dn[67][15]);
        let eq19_e1117_q_d_n16: f64 = (p.p87 * s.dn[67][16]);
        let eq19_e1117_q_d_n17: f64 = (p.p87 * s.dn[67][17]);
        let eq19_e1117_q_d_b0: f64 = (p.p87 * s.db[67][0]);
        let eq19_e1117_q_d_b1: f64 = (p.p87 * s.db[67][1]);
        let eq19_e1117_q_d_b2: f64 = (p.p87 * s.db[67][2]);
        let eq19_e1117_q_d_b3: f64 = (p.p87 * s.db[67][3]);
        let eq19_e1117_q_d_b4: f64 = (p.p87 * s.db[67][4]);
        let eq19_e1117_q_d_b5: f64 = (p.p87 * s.db[67][5]);
        let eq19_e1117_q_d_b6: f64 = (p.p87 * s.db[67][6]);
        let eq19_e1117_q_d_b7: f64 = (p.p87 * s.db[67][7]);
        let eq19_e1117_q_d_b8: f64 = (p.p87 * s.db[67][8]);
        (eq19_e1117, eq19_e1117_d_n0, eq19_e1117_d_n1, eq19_e1117_d_n2, eq19_e1117_d_n3, eq19_e1117_d_n4, eq19_e1117_d_n5, eq19_e1117_d_n6, eq19_e1117_d_n7, eq19_e1117_d_n8, eq19_e1117_d_n9, eq19_e1117_d_n10, eq19_e1117_d_n11, eq19_e1117_d_n12, eq19_e1117_d_n13, eq19_e1117_d_n14, eq19_e1117_d_n15, eq19_e1117_d_n16, eq19_e1117_d_n17, eq19_e1117_d_b0, eq19_e1117_d_b1, eq19_e1117_d_b2, eq19_e1117_d_b3, eq19_e1117_d_b4, eq19_e1117_d_b5, eq19_e1117_d_b6, eq19_e1117_d_b7, eq19_e1117_d_b8, eq19_e1117_q, eq19_e1117_q_d_n0, eq19_e1117_q_d_n1, eq19_e1117_q_d_n2, eq19_e1117_q_d_n3, eq19_e1117_q_d_n4, eq19_e1117_q_d_n5, eq19_e1117_q_d_n6, eq19_e1117_q_d_n7, eq19_e1117_q_d_n8, eq19_e1117_q_d_n9, eq19_e1117_q_d_n10, eq19_e1117_q_d_n11, eq19_e1117_q_d_n12, eq19_e1117_q_d_n13, eq19_e1117_q_d_n14, eq19_e1117_q_d_n15, eq19_e1117_q_d_n16, eq19_e1117_q_d_n17, eq19_e1117_q_d_b0, eq19_e1117_q_d_b1, eq19_e1117_q_d_b2, eq19_e1117_q_d_b3, eq19_e1117_q_d_b4, eq19_e1117_q_d_b5, eq19_e1117_q_d_b6, eq19_e1117_q_d_b7, eq19_e1117_q_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_reactive_node_derivatives: [f64; 18] = [eq19_e1119_q_d_n0, eq19_e1119_q_d_n1, eq19_e1119_q_d_n2, eq19_e1119_q_d_n3, eq19_e1119_q_d_n4, eq19_e1119_q_d_n5, eq19_e1119_q_d_n6, eq19_e1119_q_d_n7, eq19_e1119_q_d_n8, eq19_e1119_q_d_n9, eq19_e1119_q_d_n10, eq19_e1119_q_d_n11, eq19_e1119_q_d_n12, eq19_e1119_q_d_n13, eq19_e1119_q_d_n14, eq19_e1119_q_d_n15, eq19_e1119_q_d_n16, eq19_e1119_q_d_n17];
        let eq19_reactive_branch_derivatives: [f64; 9] = [eq19_e1119_q_d_b0, eq19_e1119_q_d_b1, eq19_e1119_q_d_b2, eq19_e1119_q_d_b3, eq19_e1119_q_d_b4, eq19_e1119_q_d_b5, eq19_e1119_q_d_b6, eq19_e1119_q_d_b7, eq19_e1119_q_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            &nodes,
            &eq19_reactive_node_derivatives,
            &branches,
            &eq19_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_27_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq27_e1163: f64 = (s.v[18] + s.v[753]);
        let eq27_e1163_d_n0: f64 = (s.dn[18][0] + s.dn[753][0]);
        let eq27_e1163_d_n1: f64 = (s.dn[18][1] + s.dn[753][1]);
        let eq27_e1163_d_n2: f64 = (s.dn[18][2] + s.dn[753][2]);
        let eq27_e1163_d_n3: f64 = (s.dn[18][3] + s.dn[753][3]);
        let eq27_e1163_d_n4: f64 = (s.dn[18][4] + s.dn[753][4]);
        let eq27_e1163_d_n5: f64 = (s.dn[18][5] + s.dn[753][5]);
        let eq27_e1163_d_n6: f64 = (s.dn[18][6] + s.dn[753][6]);
        let eq27_e1163_d_n7: f64 = (s.dn[18][7] + s.dn[753][7]);
        let eq27_e1163_d_n8: f64 = (s.dn[18][8] + s.dn[753][8]);
        let eq27_e1163_d_n9: f64 = (s.dn[18][9] + s.dn[753][9]);
        let eq27_e1163_d_n10: f64 = (s.dn[18][10] + s.dn[753][10]);
        let eq27_e1163_d_n11: f64 = (s.dn[18][11] + s.dn[753][11]);
        let eq27_e1163_d_n12: f64 = (s.dn[18][12] + s.dn[753][12]);
        let eq27_e1163_d_n13: f64 = (s.dn[18][13] + s.dn[753][13]);
        let eq27_e1163_d_n14: f64 = (s.dn[18][14] + s.dn[753][14]);
        let eq27_e1163_d_n15: f64 = (s.dn[18][15] + s.dn[753][15]);
        let eq27_e1163_d_n16: f64 = (s.dn[18][16] + s.dn[753][16]);
        let eq27_e1163_d_n17: f64 = (s.dn[18][17] + s.dn[753][17]);
        let eq27_e1163_d_b0: f64 = (s.db[18][0] + s.db[753][0]);
        let eq27_e1163_d_b1: f64 = (s.db[18][1] + s.db[753][1]);
        let eq27_e1163_d_b2: f64 = (s.db[18][2] + s.db[753][2]);
        let eq27_e1163_d_b3: f64 = (s.db[18][3] + s.db[753][3]);
        let eq27_e1163_d_b4: f64 = (s.db[18][4] + s.db[753][4]);
        let eq27_e1163_d_b5: f64 = (s.db[18][5] + s.db[753][5]);
        let eq27_e1163_d_b6: f64 = (s.db[18][6] + s.db[753][6]);
        let eq27_e1163_d_b7: f64 = (s.db[18][7] + s.db[753][7]);
        let eq27_e1163_d_b8: f64 = (s.db[18][8] + s.db[753][8]);
        let eq27_e1164_q: f64 = eq27_e1163;
        let eq27_e1165: f64 = (p.p87 * eq27_e1163);
        let eq27_e1165_d_n0: f64 = (p.p87 * eq27_e1163_d_n0);
        let eq27_e1165_d_n1: f64 = (p.p87 * eq27_e1163_d_n1);
        let eq27_e1165_d_n2: f64 = (p.p87 * eq27_e1163_d_n2);
        let eq27_e1165_d_n3: f64 = (p.p87 * eq27_e1163_d_n3);
        let eq27_e1165_d_n4: f64 = (p.p87 * eq27_e1163_d_n4);
        let eq27_e1165_d_n5: f64 = (p.p87 * eq27_e1163_d_n5);
        let eq27_e1165_d_n6: f64 = (p.p87 * eq27_e1163_d_n6);
        let eq27_e1165_d_n7: f64 = (p.p87 * eq27_e1163_d_n7);
        let eq27_e1165_d_n8: f64 = (p.p87 * eq27_e1163_d_n8);
        let eq27_e1165_d_n9: f64 = (p.p87 * eq27_e1163_d_n9);
        let eq27_e1165_d_n10: f64 = (p.p87 * eq27_e1163_d_n10);
        let eq27_e1165_d_n11: f64 = (p.p87 * eq27_e1163_d_n11);
        let eq27_e1165_d_n12: f64 = (p.p87 * eq27_e1163_d_n12);
        let eq27_e1165_d_n13: f64 = (p.p87 * eq27_e1163_d_n13);
        let eq27_e1165_d_n14: f64 = (p.p87 * eq27_e1163_d_n14);
        let eq27_e1165_d_n15: f64 = (p.p87 * eq27_e1163_d_n15);
        let eq27_e1165_d_n16: f64 = (p.p87 * eq27_e1163_d_n16);
        let eq27_e1165_d_n17: f64 = (p.p87 * eq27_e1163_d_n17);
        let eq27_e1165_d_b0: f64 = (p.p87 * eq27_e1163_d_b0);
        let eq27_e1165_d_b1: f64 = (p.p87 * eq27_e1163_d_b1);
        let eq27_e1165_d_b2: f64 = (p.p87 * eq27_e1163_d_b2);
        let eq27_e1165_d_b3: f64 = (p.p87 * eq27_e1163_d_b3);
        let eq27_e1165_d_b4: f64 = (p.p87 * eq27_e1163_d_b4);
        let eq27_e1165_d_b5: f64 = (p.p87 * eq27_e1163_d_b5);
        let eq27_e1165_d_b6: f64 = (p.p87 * eq27_e1163_d_b6);
        let eq27_e1165_d_b7: f64 = (p.p87 * eq27_e1163_d_b7);
        let eq27_e1165_d_b8: f64 = (p.p87 * eq27_e1163_d_b8);
        let eq27_e1165_q: f64 = (p.p87 * eq27_e1164_q);
        let eq27_e1165_q_d_n0: f64 = (p.p87 * eq27_e1163_d_n0);
        let eq27_e1165_q_d_n1: f64 = (p.p87 * eq27_e1163_d_n1);
        let eq27_e1165_q_d_n2: f64 = (p.p87 * eq27_e1163_d_n2);
        let eq27_e1165_q_d_n3: f64 = (p.p87 * eq27_e1163_d_n3);
        let eq27_e1165_q_d_n4: f64 = (p.p87 * eq27_e1163_d_n4);
        let eq27_e1165_q_d_n5: f64 = (p.p87 * eq27_e1163_d_n5);
        let eq27_e1165_q_d_n6: f64 = (p.p87 * eq27_e1163_d_n6);
        let eq27_e1165_q_d_n7: f64 = (p.p87 * eq27_e1163_d_n7);
        let eq27_e1165_q_d_n8: f64 = (p.p87 * eq27_e1163_d_n8);
        let eq27_e1165_q_d_n9: f64 = (p.p87 * eq27_e1163_d_n9);
        let eq27_e1165_q_d_n10: f64 = (p.p87 * eq27_e1163_d_n10);
        let eq27_e1165_q_d_n11: f64 = (p.p87 * eq27_e1163_d_n11);
        let eq27_e1165_q_d_n12: f64 = (p.p87 * eq27_e1163_d_n12);
        let eq27_e1165_q_d_n13: f64 = (p.p87 * eq27_e1163_d_n13);
        let eq27_e1165_q_d_n14: f64 = (p.p87 * eq27_e1163_d_n14);
        let eq27_e1165_q_d_n15: f64 = (p.p87 * eq27_e1163_d_n15);
        let eq27_e1165_q_d_n16: f64 = (p.p87 * eq27_e1163_d_n16);
        let eq27_e1165_q_d_n17: f64 = (p.p87 * eq27_e1163_d_n17);
        let eq27_e1165_q_d_b0: f64 = (p.p87 * eq27_e1163_d_b0);
        let eq27_e1165_q_d_b1: f64 = (p.p87 * eq27_e1163_d_b1);
        let eq27_e1165_q_d_b2: f64 = (p.p87 * eq27_e1163_d_b2);
        let eq27_e1165_q_d_b3: f64 = (p.p87 * eq27_e1163_d_b3);
        let eq27_e1165_q_d_b4: f64 = (p.p87 * eq27_e1163_d_b4);
        let eq27_e1165_q_d_b5: f64 = (p.p87 * eq27_e1163_d_b5);
        let eq27_e1165_q_d_b6: f64 = (p.p87 * eq27_e1163_d_b6);
        let eq27_e1165_q_d_b7: f64 = (p.p87 * eq27_e1163_d_b7);
        let eq27_e1165_q_d_b8: f64 = (p.p87 * eq27_e1163_d_b8);
        let eq27_reactive_node_derivatives: [f64; 18] = [eq27_e1165_q_d_n0, eq27_e1165_q_d_n1, eq27_e1165_q_d_n2, eq27_e1165_q_d_n3, eq27_e1165_q_d_n4, eq27_e1165_q_d_n5, eq27_e1165_q_d_n6, eq27_e1165_q_d_n7, eq27_e1165_q_d_n8, eq27_e1165_q_d_n9, eq27_e1165_q_d_n10, eq27_e1165_q_d_n11, eq27_e1165_q_d_n12, eq27_e1165_q_d_n13, eq27_e1165_q_d_n14, eq27_e1165_q_d_n15, eq27_e1165_q_d_n16, eq27_e1165_q_d_n17];
        let eq27_reactive_branch_derivatives: [f64; 9] = [eq27_e1165_q_d_b0, eq27_e1165_q_d_b1, eq27_e1165_q_d_b2, eq27_e1165_q_d_b3, eq27_e1165_q_d_b4, eq27_e1165_q_d_b5, eq27_e1165_q_d_b6, eq27_e1165_q_d_b7, eq27_e1165_q_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            &nodes,
            &eq27_reactive_node_derivatives,
            &branches,
            &eq27_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_28_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq28_e1169: f64 = (s.v[19] + s.v[751]);
        let eq28_e1169_d_n0: f64 = (s.dn[19][0] + s.dn[751][0]);
        let eq28_e1169_d_n1: f64 = (s.dn[19][1] + s.dn[751][1]);
        let eq28_e1169_d_n2: f64 = (s.dn[19][2] + s.dn[751][2]);
        let eq28_e1169_d_n3: f64 = (s.dn[19][3] + s.dn[751][3]);
        let eq28_e1169_d_n4: f64 = (s.dn[19][4] + s.dn[751][4]);
        let eq28_e1169_d_n5: f64 = (s.dn[19][5] + s.dn[751][5]);
        let eq28_e1169_d_n6: f64 = (s.dn[19][6] + s.dn[751][6]);
        let eq28_e1169_d_n7: f64 = (s.dn[19][7] + s.dn[751][7]);
        let eq28_e1169_d_n8: f64 = (s.dn[19][8] + s.dn[751][8]);
        let eq28_e1169_d_n9: f64 = (s.dn[19][9] + s.dn[751][9]);
        let eq28_e1169_d_n10: f64 = (s.dn[19][10] + s.dn[751][10]);
        let eq28_e1169_d_n11: f64 = (s.dn[19][11] + s.dn[751][11]);
        let eq28_e1169_d_n12: f64 = (s.dn[19][12] + s.dn[751][12]);
        let eq28_e1169_d_n13: f64 = (s.dn[19][13] + s.dn[751][13]);
        let eq28_e1169_d_n14: f64 = (s.dn[19][14] + s.dn[751][14]);
        let eq28_e1169_d_n15: f64 = (s.dn[19][15] + s.dn[751][15]);
        let eq28_e1169_d_n16: f64 = (s.dn[19][16] + s.dn[751][16]);
        let eq28_e1169_d_n17: f64 = (s.dn[19][17] + s.dn[751][17]);
        let eq28_e1169_d_b0: f64 = (s.db[19][0] + s.db[751][0]);
        let eq28_e1169_d_b1: f64 = (s.db[19][1] + s.db[751][1]);
        let eq28_e1169_d_b2: f64 = (s.db[19][2] + s.db[751][2]);
        let eq28_e1169_d_b3: f64 = (s.db[19][3] + s.db[751][3]);
        let eq28_e1169_d_b4: f64 = (s.db[19][4] + s.db[751][4]);
        let eq28_e1169_d_b5: f64 = (s.db[19][5] + s.db[751][5]);
        let eq28_e1169_d_b6: f64 = (s.db[19][6] + s.db[751][6]);
        let eq28_e1169_d_b7: f64 = (s.db[19][7] + s.db[751][7]);
        let eq28_e1169_d_b8: f64 = (s.db[19][8] + s.db[751][8]);
        let eq28_e1170_q: f64 = eq28_e1169;
        let eq28_e1171: f64 = (p.p87 * eq28_e1169);
        let eq28_e1171_d_n0: f64 = (p.p87 * eq28_e1169_d_n0);
        let eq28_e1171_d_n1: f64 = (p.p87 * eq28_e1169_d_n1);
        let eq28_e1171_d_n2: f64 = (p.p87 * eq28_e1169_d_n2);
        let eq28_e1171_d_n3: f64 = (p.p87 * eq28_e1169_d_n3);
        let eq28_e1171_d_n4: f64 = (p.p87 * eq28_e1169_d_n4);
        let eq28_e1171_d_n5: f64 = (p.p87 * eq28_e1169_d_n5);
        let eq28_e1171_d_n6: f64 = (p.p87 * eq28_e1169_d_n6);
        let eq28_e1171_d_n7: f64 = (p.p87 * eq28_e1169_d_n7);
        let eq28_e1171_d_n8: f64 = (p.p87 * eq28_e1169_d_n8);
        let eq28_e1171_d_n9: f64 = (p.p87 * eq28_e1169_d_n9);
        let eq28_e1171_d_n10: f64 = (p.p87 * eq28_e1169_d_n10);
        let eq28_e1171_d_n11: f64 = (p.p87 * eq28_e1169_d_n11);
        let eq28_e1171_d_n12: f64 = (p.p87 * eq28_e1169_d_n12);
        let eq28_e1171_d_n13: f64 = (p.p87 * eq28_e1169_d_n13);
        let eq28_e1171_d_n14: f64 = (p.p87 * eq28_e1169_d_n14);
        let eq28_e1171_d_n15: f64 = (p.p87 * eq28_e1169_d_n15);
        let eq28_e1171_d_n16: f64 = (p.p87 * eq28_e1169_d_n16);
        let eq28_e1171_d_n17: f64 = (p.p87 * eq28_e1169_d_n17);
        let eq28_e1171_d_b0: f64 = (p.p87 * eq28_e1169_d_b0);
        let eq28_e1171_d_b1: f64 = (p.p87 * eq28_e1169_d_b1);
        let eq28_e1171_d_b2: f64 = (p.p87 * eq28_e1169_d_b2);
        let eq28_e1171_d_b3: f64 = (p.p87 * eq28_e1169_d_b3);
        let eq28_e1171_d_b4: f64 = (p.p87 * eq28_e1169_d_b4);
        let eq28_e1171_d_b5: f64 = (p.p87 * eq28_e1169_d_b5);
        let eq28_e1171_d_b6: f64 = (p.p87 * eq28_e1169_d_b6);
        let eq28_e1171_d_b7: f64 = (p.p87 * eq28_e1169_d_b7);
        let eq28_e1171_d_b8: f64 = (p.p87 * eq28_e1169_d_b8);
        let eq28_e1171_q: f64 = (p.p87 * eq28_e1170_q);
        let eq28_e1171_q_d_n0: f64 = (p.p87 * eq28_e1169_d_n0);
        let eq28_e1171_q_d_n1: f64 = (p.p87 * eq28_e1169_d_n1);
        let eq28_e1171_q_d_n2: f64 = (p.p87 * eq28_e1169_d_n2);
        let eq28_e1171_q_d_n3: f64 = (p.p87 * eq28_e1169_d_n3);
        let eq28_e1171_q_d_n4: f64 = (p.p87 * eq28_e1169_d_n4);
        let eq28_e1171_q_d_n5: f64 = (p.p87 * eq28_e1169_d_n5);
        let eq28_e1171_q_d_n6: f64 = (p.p87 * eq28_e1169_d_n6);
        let eq28_e1171_q_d_n7: f64 = (p.p87 * eq28_e1169_d_n7);
        let eq28_e1171_q_d_n8: f64 = (p.p87 * eq28_e1169_d_n8);
        let eq28_e1171_q_d_n9: f64 = (p.p87 * eq28_e1169_d_n9);
        let eq28_e1171_q_d_n10: f64 = (p.p87 * eq28_e1169_d_n10);
        let eq28_e1171_q_d_n11: f64 = (p.p87 * eq28_e1169_d_n11);
        let eq28_e1171_q_d_n12: f64 = (p.p87 * eq28_e1169_d_n12);
        let eq28_e1171_q_d_n13: f64 = (p.p87 * eq28_e1169_d_n13);
        let eq28_e1171_q_d_n14: f64 = (p.p87 * eq28_e1169_d_n14);
        let eq28_e1171_q_d_n15: f64 = (p.p87 * eq28_e1169_d_n15);
        let eq28_e1171_q_d_n16: f64 = (p.p87 * eq28_e1169_d_n16);
        let eq28_e1171_q_d_n17: f64 = (p.p87 * eq28_e1169_d_n17);
        let eq28_e1171_q_d_b0: f64 = (p.p87 * eq28_e1169_d_b0);
        let eq28_e1171_q_d_b1: f64 = (p.p87 * eq28_e1169_d_b1);
        let eq28_e1171_q_d_b2: f64 = (p.p87 * eq28_e1169_d_b2);
        let eq28_e1171_q_d_b3: f64 = (p.p87 * eq28_e1169_d_b3);
        let eq28_e1171_q_d_b4: f64 = (p.p87 * eq28_e1169_d_b4);
        let eq28_e1171_q_d_b5: f64 = (p.p87 * eq28_e1169_d_b5);
        let eq28_e1171_q_d_b6: f64 = (p.p87 * eq28_e1169_d_b6);
        let eq28_e1171_q_d_b7: f64 = (p.p87 * eq28_e1169_d_b7);
        let eq28_e1171_q_d_b8: f64 = (p.p87 * eq28_e1169_d_b8);
        let eq28_reactive_node_derivatives: [f64; 18] = [eq28_e1171_q_d_n0, eq28_e1171_q_d_n1, eq28_e1171_q_d_n2, eq28_e1171_q_d_n3, eq28_e1171_q_d_n4, eq28_e1171_q_d_n5, eq28_e1171_q_d_n6, eq28_e1171_q_d_n7, eq28_e1171_q_d_n8, eq28_e1171_q_d_n9, eq28_e1171_q_d_n10, eq28_e1171_q_d_n11, eq28_e1171_q_d_n12, eq28_e1171_q_d_n13, eq28_e1171_q_d_n14, eq28_e1171_q_d_n15, eq28_e1171_q_d_n16, eq28_e1171_q_d_n17];
        let eq28_reactive_branch_derivatives: [f64; 9] = [eq28_e1171_q_d_b0, eq28_e1171_q_d_b1, eq28_e1171_q_d_b2, eq28_e1171_q_d_b3, eq28_e1171_q_d_b4, eq28_e1171_q_d_b5, eq28_e1171_q_d_b6, eq28_e1171_q_d_b7, eq28_e1171_q_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            &nodes,
            &eq28_reactive_node_derivatives,
            &branches,
            &eq28_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_29_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq29_e1176: f64 = (s.v[753] + s.v[751]);
        let eq29_e1176_d_n0: f64 = (s.dn[753][0] + s.dn[751][0]);
        let eq29_e1176_d_n1: f64 = (s.dn[753][1] + s.dn[751][1]);
        let eq29_e1176_d_n2: f64 = (s.dn[753][2] + s.dn[751][2]);
        let eq29_e1176_d_n3: f64 = (s.dn[753][3] + s.dn[751][3]);
        let eq29_e1176_d_n4: f64 = (s.dn[753][4] + s.dn[751][4]);
        let eq29_e1176_d_n5: f64 = (s.dn[753][5] + s.dn[751][5]);
        let eq29_e1176_d_n6: f64 = (s.dn[753][6] + s.dn[751][6]);
        let eq29_e1176_d_n7: f64 = (s.dn[753][7] + s.dn[751][7]);
        let eq29_e1176_d_n8: f64 = (s.dn[753][8] + s.dn[751][8]);
        let eq29_e1176_d_n9: f64 = (s.dn[753][9] + s.dn[751][9]);
        let eq29_e1176_d_n10: f64 = (s.dn[753][10] + s.dn[751][10]);
        let eq29_e1176_d_n11: f64 = (s.dn[753][11] + s.dn[751][11]);
        let eq29_e1176_d_n12: f64 = (s.dn[753][12] + s.dn[751][12]);
        let eq29_e1176_d_n13: f64 = (s.dn[753][13] + s.dn[751][13]);
        let eq29_e1176_d_n14: f64 = (s.dn[753][14] + s.dn[751][14]);
        let eq29_e1176_d_n15: f64 = (s.dn[753][15] + s.dn[751][15]);
        let eq29_e1176_d_n16: f64 = (s.dn[753][16] + s.dn[751][16]);
        let eq29_e1176_d_n17: f64 = (s.dn[753][17] + s.dn[751][17]);
        let eq29_e1176_d_b0: f64 = (s.db[753][0] + s.db[751][0]);
        let eq29_e1176_d_b1: f64 = (s.db[753][1] + s.db[751][1]);
        let eq29_e1176_d_b2: f64 = (s.db[753][2] + s.db[751][2]);
        let eq29_e1176_d_b3: f64 = (s.db[753][3] + s.db[751][3]);
        let eq29_e1176_d_b4: f64 = (s.db[753][4] + s.db[751][4]);
        let eq29_e1176_d_b5: f64 = (s.db[753][5] + s.db[751][5]);
        let eq29_e1176_d_b6: f64 = (s.db[753][6] + s.db[751][6]);
        let eq29_e1176_d_b7: f64 = (s.db[753][7] + s.db[751][7]);
        let eq29_e1176_d_b8: f64 = (s.db[753][8] + s.db[751][8]);
        let eq29_e1178: f64 = (eq29_e1176 + s.v[752]);
        let eq29_e1178_d_n0: f64 = (eq29_e1176_d_n0 + s.dn[752][0]);
        let eq29_e1178_d_n1: f64 = (eq29_e1176_d_n1 + s.dn[752][1]);
        let eq29_e1178_d_n2: f64 = (eq29_e1176_d_n2 + s.dn[752][2]);
        let eq29_e1178_d_n3: f64 = (eq29_e1176_d_n3 + s.dn[752][3]);
        let eq29_e1178_d_n4: f64 = (eq29_e1176_d_n4 + s.dn[752][4]);
        let eq29_e1178_d_n5: f64 = (eq29_e1176_d_n5 + s.dn[752][5]);
        let eq29_e1178_d_n6: f64 = (eq29_e1176_d_n6 + s.dn[752][6]);
        let eq29_e1178_d_n7: f64 = (eq29_e1176_d_n7 + s.dn[752][7]);
        let eq29_e1178_d_n8: f64 = (eq29_e1176_d_n8 + s.dn[752][8]);
        let eq29_e1178_d_n9: f64 = (eq29_e1176_d_n9 + s.dn[752][9]);
        let eq29_e1178_d_n10: f64 = (eq29_e1176_d_n10 + s.dn[752][10]);
        let eq29_e1178_d_n11: f64 = (eq29_e1176_d_n11 + s.dn[752][11]);
        let eq29_e1178_d_n12: f64 = (eq29_e1176_d_n12 + s.dn[752][12]);
        let eq29_e1178_d_n13: f64 = (eq29_e1176_d_n13 + s.dn[752][13]);
        let eq29_e1178_d_n14: f64 = (eq29_e1176_d_n14 + s.dn[752][14]);
        let eq29_e1178_d_n15: f64 = (eq29_e1176_d_n15 + s.dn[752][15]);
        let eq29_e1178_d_n16: f64 = (eq29_e1176_d_n16 + s.dn[752][16]);
        let eq29_e1178_d_n17: f64 = (eq29_e1176_d_n17 + s.dn[752][17]);
        let eq29_e1178_d_b0: f64 = (eq29_e1176_d_b0 + s.db[752][0]);
        let eq29_e1178_d_b1: f64 = (eq29_e1176_d_b1 + s.db[752][1]);
        let eq29_e1178_d_b2: f64 = (eq29_e1176_d_b2 + s.db[752][2]);
        let eq29_e1178_d_b3: f64 = (eq29_e1176_d_b3 + s.db[752][3]);
        let eq29_e1178_d_b4: f64 = (eq29_e1176_d_b4 + s.db[752][4]);
        let eq29_e1178_d_b5: f64 = (eq29_e1176_d_b5 + s.db[752][5]);
        let eq29_e1178_d_b6: f64 = (eq29_e1176_d_b6 + s.db[752][6]);
        let eq29_e1178_d_b7: f64 = (eq29_e1176_d_b7 + s.db[752][7]);
        let eq29_e1178_d_b8: f64 = (eq29_e1176_d_b8 + s.db[752][8]);
        let eq29_e1179: f64 = (s.v[20] - eq29_e1178);
        let eq29_e1179_d_n0: f64 = (s.dn[20][0] - eq29_e1178_d_n0);
        let eq29_e1179_d_n1: f64 = (s.dn[20][1] - eq29_e1178_d_n1);
        let eq29_e1179_d_n2: f64 = (s.dn[20][2] - eq29_e1178_d_n2);
        let eq29_e1179_d_n3: f64 = (s.dn[20][3] - eq29_e1178_d_n3);
        let eq29_e1179_d_n4: f64 = (s.dn[20][4] - eq29_e1178_d_n4);
        let eq29_e1179_d_n5: f64 = (s.dn[20][5] - eq29_e1178_d_n5);
        let eq29_e1179_d_n6: f64 = (s.dn[20][6] - eq29_e1178_d_n6);
        let eq29_e1179_d_n7: f64 = (s.dn[20][7] - eq29_e1178_d_n7);
        let eq29_e1179_d_n8: f64 = (s.dn[20][8] - eq29_e1178_d_n8);
        let eq29_e1179_d_n9: f64 = (s.dn[20][9] - eq29_e1178_d_n9);
        let eq29_e1179_d_n10: f64 = (s.dn[20][10] - eq29_e1178_d_n10);
        let eq29_e1179_d_n11: f64 = (s.dn[20][11] - eq29_e1178_d_n11);
        let eq29_e1179_d_n12: f64 = (s.dn[20][12] - eq29_e1178_d_n12);
        let eq29_e1179_d_n13: f64 = (s.dn[20][13] - eq29_e1178_d_n13);
        let eq29_e1179_d_n14: f64 = (s.dn[20][14] - eq29_e1178_d_n14);
        let eq29_e1179_d_n15: f64 = (s.dn[20][15] - eq29_e1178_d_n15);
        let eq29_e1179_d_n16: f64 = (s.dn[20][16] - eq29_e1178_d_n16);
        let eq29_e1179_d_n17: f64 = (s.dn[20][17] - eq29_e1178_d_n17);
        let eq29_e1179_d_b0: f64 = (s.db[20][0] - eq29_e1178_d_b0);
        let eq29_e1179_d_b1: f64 = (s.db[20][1] - eq29_e1178_d_b1);
        let eq29_e1179_d_b2: f64 = (s.db[20][2] - eq29_e1178_d_b2);
        let eq29_e1179_d_b3: f64 = (s.db[20][3] - eq29_e1178_d_b3);
        let eq29_e1179_d_b4: f64 = (s.db[20][4] - eq29_e1178_d_b4);
        let eq29_e1179_d_b5: f64 = (s.db[20][5] - eq29_e1178_d_b5);
        let eq29_e1179_d_b6: f64 = (s.db[20][6] - eq29_e1178_d_b6);
        let eq29_e1179_d_b7: f64 = (s.db[20][7] - eq29_e1178_d_b7);
        let eq29_e1179_d_b8: f64 = (s.db[20][8] - eq29_e1178_d_b8);
        let eq29_e1180_q: f64 = eq29_e1179;
        let eq29_e1181: f64 = (p.p87 * eq29_e1179);
        let eq29_e1181_d_n0: f64 = (p.p87 * eq29_e1179_d_n0);
        let eq29_e1181_d_n1: f64 = (p.p87 * eq29_e1179_d_n1);
        let eq29_e1181_d_n2: f64 = (p.p87 * eq29_e1179_d_n2);
        let eq29_e1181_d_n3: f64 = (p.p87 * eq29_e1179_d_n3);
        let eq29_e1181_d_n4: f64 = (p.p87 * eq29_e1179_d_n4);
        let eq29_e1181_d_n5: f64 = (p.p87 * eq29_e1179_d_n5);
        let eq29_e1181_d_n6: f64 = (p.p87 * eq29_e1179_d_n6);
        let eq29_e1181_d_n7: f64 = (p.p87 * eq29_e1179_d_n7);
        let eq29_e1181_d_n8: f64 = (p.p87 * eq29_e1179_d_n8);
        let eq29_e1181_d_n9: f64 = (p.p87 * eq29_e1179_d_n9);
        let eq29_e1181_d_n10: f64 = (p.p87 * eq29_e1179_d_n10);
        let eq29_e1181_d_n11: f64 = (p.p87 * eq29_e1179_d_n11);
        let eq29_e1181_d_n12: f64 = (p.p87 * eq29_e1179_d_n12);
        let eq29_e1181_d_n13: f64 = (p.p87 * eq29_e1179_d_n13);
        let eq29_e1181_d_n14: f64 = (p.p87 * eq29_e1179_d_n14);
        let eq29_e1181_d_n15: f64 = (p.p87 * eq29_e1179_d_n15);
        let eq29_e1181_d_n16: f64 = (p.p87 * eq29_e1179_d_n16);
        let eq29_e1181_d_n17: f64 = (p.p87 * eq29_e1179_d_n17);
        let eq29_e1181_d_b0: f64 = (p.p87 * eq29_e1179_d_b0);
        let eq29_e1181_d_b1: f64 = (p.p87 * eq29_e1179_d_b1);
        let eq29_e1181_d_b2: f64 = (p.p87 * eq29_e1179_d_b2);
        let eq29_e1181_d_b3: f64 = (p.p87 * eq29_e1179_d_b3);
        let eq29_e1181_d_b4: f64 = (p.p87 * eq29_e1179_d_b4);
        let eq29_e1181_d_b5: f64 = (p.p87 * eq29_e1179_d_b5);
        let eq29_e1181_d_b6: f64 = (p.p87 * eq29_e1179_d_b6);
        let eq29_e1181_d_b7: f64 = (p.p87 * eq29_e1179_d_b7);
        let eq29_e1181_d_b8: f64 = (p.p87 * eq29_e1179_d_b8);
        let eq29_e1181_q: f64 = (p.p87 * eq29_e1180_q);
        let eq29_e1181_q_d_n0: f64 = (p.p87 * eq29_e1179_d_n0);
        let eq29_e1181_q_d_n1: f64 = (p.p87 * eq29_e1179_d_n1);
        let eq29_e1181_q_d_n2: f64 = (p.p87 * eq29_e1179_d_n2);
        let eq29_e1181_q_d_n3: f64 = (p.p87 * eq29_e1179_d_n3);
        let eq29_e1181_q_d_n4: f64 = (p.p87 * eq29_e1179_d_n4);
        let eq29_e1181_q_d_n5: f64 = (p.p87 * eq29_e1179_d_n5);
        let eq29_e1181_q_d_n6: f64 = (p.p87 * eq29_e1179_d_n6);
        let eq29_e1181_q_d_n7: f64 = (p.p87 * eq29_e1179_d_n7);
        let eq29_e1181_q_d_n8: f64 = (p.p87 * eq29_e1179_d_n8);
        let eq29_e1181_q_d_n9: f64 = (p.p87 * eq29_e1179_d_n9);
        let eq29_e1181_q_d_n10: f64 = (p.p87 * eq29_e1179_d_n10);
        let eq29_e1181_q_d_n11: f64 = (p.p87 * eq29_e1179_d_n11);
        let eq29_e1181_q_d_n12: f64 = (p.p87 * eq29_e1179_d_n12);
        let eq29_e1181_q_d_n13: f64 = (p.p87 * eq29_e1179_d_n13);
        let eq29_e1181_q_d_n14: f64 = (p.p87 * eq29_e1179_d_n14);
        let eq29_e1181_q_d_n15: f64 = (p.p87 * eq29_e1179_d_n15);
        let eq29_e1181_q_d_n16: f64 = (p.p87 * eq29_e1179_d_n16);
        let eq29_e1181_q_d_n17: f64 = (p.p87 * eq29_e1179_d_n17);
        let eq29_e1181_q_d_b0: f64 = (p.p87 * eq29_e1179_d_b0);
        let eq29_e1181_q_d_b1: f64 = (p.p87 * eq29_e1179_d_b1);
        let eq29_e1181_q_d_b2: f64 = (p.p87 * eq29_e1179_d_b2);
        let eq29_e1181_q_d_b3: f64 = (p.p87 * eq29_e1179_d_b3);
        let eq29_e1181_q_d_b4: f64 = (p.p87 * eq29_e1179_d_b4);
        let eq29_e1181_q_d_b5: f64 = (p.p87 * eq29_e1179_d_b5);
        let eq29_e1181_q_d_b6: f64 = (p.p87 * eq29_e1179_d_b6);
        let eq29_e1181_q_d_b7: f64 = (p.p87 * eq29_e1179_d_b7);
        let eq29_e1181_q_d_b8: f64 = (p.p87 * eq29_e1179_d_b8);
        let eq29_reactive_node_derivatives: [f64; 18] = [eq29_e1181_q_d_n0, eq29_e1181_q_d_n1, eq29_e1181_q_d_n2, eq29_e1181_q_d_n3, eq29_e1181_q_d_n4, eq29_e1181_q_d_n5, eq29_e1181_q_d_n6, eq29_e1181_q_d_n7, eq29_e1181_q_d_n8, eq29_e1181_q_d_n9, eq29_e1181_q_d_n10, eq29_e1181_q_d_n11, eq29_e1181_q_d_n12, eq29_e1181_q_d_n13, eq29_e1181_q_d_n14, eq29_e1181_q_d_n15, eq29_e1181_q_d_n16, eq29_e1181_q_d_n17];
        let eq29_reactive_branch_derivatives: [f64; 9] = [eq29_e1181_q_d_b0, eq29_e1181_q_d_b1, eq29_e1181_q_d_b2, eq29_e1181_q_d_b3, eq29_e1181_q_d_b4, eq29_e1181_q_d_b5, eq29_e1181_q_d_b6, eq29_e1181_q_d_b7, eq29_e1181_q_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            &nodes,
            &eq29_reactive_node_derivatives,
            &branches,
            &eq29_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_30_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq30_e1184_q: f64 = s.v[743];
        let eq30_e1185: f64 = (p.p87 * s.v[743]);
        let eq30_e1185_d_n0: f64 = (p.p87 * s.dn[743][0]);
        let eq30_e1185_d_n1: f64 = (p.p87 * s.dn[743][1]);
        let eq30_e1185_d_n2: f64 = (p.p87 * s.dn[743][2]);
        let eq30_e1185_d_n3: f64 = (p.p87 * s.dn[743][3]);
        let eq30_e1185_d_n4: f64 = (p.p87 * s.dn[743][4]);
        let eq30_e1185_d_n5: f64 = (p.p87 * s.dn[743][5]);
        let eq30_e1185_d_n6: f64 = (p.p87 * s.dn[743][6]);
        let eq30_e1185_d_n7: f64 = (p.p87 * s.dn[743][7]);
        let eq30_e1185_d_n8: f64 = (p.p87 * s.dn[743][8]);
        let eq30_e1185_d_n9: f64 = (p.p87 * s.dn[743][9]);
        let eq30_e1185_d_n10: f64 = (p.p87 * s.dn[743][10]);
        let eq30_e1185_d_n11: f64 = (p.p87 * s.dn[743][11]);
        let eq30_e1185_d_n12: f64 = (p.p87 * s.dn[743][12]);
        let eq30_e1185_d_n13: f64 = (p.p87 * s.dn[743][13]);
        let eq30_e1185_d_n14: f64 = (p.p87 * s.dn[743][14]);
        let eq30_e1185_d_n15: f64 = (p.p87 * s.dn[743][15]);
        let eq30_e1185_d_n16: f64 = (p.p87 * s.dn[743][16]);
        let eq30_e1185_d_n17: f64 = (p.p87 * s.dn[743][17]);
        let eq30_e1185_d_b0: f64 = (p.p87 * s.db[743][0]);
        let eq30_e1185_d_b1: f64 = (p.p87 * s.db[743][1]);
        let eq30_e1185_d_b2: f64 = (p.p87 * s.db[743][2]);
        let eq30_e1185_d_b3: f64 = (p.p87 * s.db[743][3]);
        let eq30_e1185_d_b4: f64 = (p.p87 * s.db[743][4]);
        let eq30_e1185_d_b5: f64 = (p.p87 * s.db[743][5]);
        let eq30_e1185_d_b6: f64 = (p.p87 * s.db[743][6]);
        let eq30_e1185_d_b7: f64 = (p.p87 * s.db[743][7]);
        let eq30_e1185_d_b8: f64 = (p.p87 * s.db[743][8]);
        let eq30_e1185_q: f64 = (p.p87 * eq30_e1184_q);
        let eq30_e1185_q_d_n0: f64 = (p.p87 * s.dn[743][0]);
        let eq30_e1185_q_d_n1: f64 = (p.p87 * s.dn[743][1]);
        let eq30_e1185_q_d_n2: f64 = (p.p87 * s.dn[743][2]);
        let eq30_e1185_q_d_n3: f64 = (p.p87 * s.dn[743][3]);
        let eq30_e1185_q_d_n4: f64 = (p.p87 * s.dn[743][4]);
        let eq30_e1185_q_d_n5: f64 = (p.p87 * s.dn[743][5]);
        let eq30_e1185_q_d_n6: f64 = (p.p87 * s.dn[743][6]);
        let eq30_e1185_q_d_n7: f64 = (p.p87 * s.dn[743][7]);
        let eq30_e1185_q_d_n8: f64 = (p.p87 * s.dn[743][8]);
        let eq30_e1185_q_d_n9: f64 = (p.p87 * s.dn[743][9]);
        let eq30_e1185_q_d_n10: f64 = (p.p87 * s.dn[743][10]);
        let eq30_e1185_q_d_n11: f64 = (p.p87 * s.dn[743][11]);
        let eq30_e1185_q_d_n12: f64 = (p.p87 * s.dn[743][12]);
        let eq30_e1185_q_d_n13: f64 = (p.p87 * s.dn[743][13]);
        let eq30_e1185_q_d_n14: f64 = (p.p87 * s.dn[743][14]);
        let eq30_e1185_q_d_n15: f64 = (p.p87 * s.dn[743][15]);
        let eq30_e1185_q_d_n16: f64 = (p.p87 * s.dn[743][16]);
        let eq30_e1185_q_d_n17: f64 = (p.p87 * s.dn[743][17]);
        let eq30_e1185_q_d_b0: f64 = (p.p87 * s.db[743][0]);
        let eq30_e1185_q_d_b1: f64 = (p.p87 * s.db[743][1]);
        let eq30_e1185_q_d_b2: f64 = (p.p87 * s.db[743][2]);
        let eq30_e1185_q_d_b3: f64 = (p.p87 * s.db[743][3]);
        let eq30_e1185_q_d_b4: f64 = (p.p87 * s.db[743][4]);
        let eq30_e1185_q_d_b5: f64 = (p.p87 * s.db[743][5]);
        let eq30_e1185_q_d_b6: f64 = (p.p87 * s.db[743][6]);
        let eq30_e1185_q_d_b7: f64 = (p.p87 * s.db[743][7]);
        let eq30_e1185_q_d_b8: f64 = (p.p87 * s.db[743][8]);
        let eq30_reactive_node_derivatives: [f64; 18] = [eq30_e1185_q_d_n0, eq30_e1185_q_d_n1, eq30_e1185_q_d_n2, eq30_e1185_q_d_n3, eq30_e1185_q_d_n4, eq30_e1185_q_d_n5, eq30_e1185_q_d_n6, eq30_e1185_q_d_n7, eq30_e1185_q_d_n8, eq30_e1185_q_d_n9, eq30_e1185_q_d_n10, eq30_e1185_q_d_n11, eq30_e1185_q_d_n12, eq30_e1185_q_d_n13, eq30_e1185_q_d_n14, eq30_e1185_q_d_n15, eq30_e1185_q_d_n16, eq30_e1185_q_d_n17];
        let eq30_reactive_branch_derivatives: [f64; 9] = [eq30_e1185_q_d_b0, eq30_e1185_q_d_b1, eq30_e1185_q_d_b2, eq30_e1185_q_d_b3, eq30_e1185_q_d_b4, eq30_e1185_q_d_b5, eq30_e1185_q_d_b6, eq30_e1185_q_d_b7, eq30_e1185_q_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[2]),
            &nodes,
            &eq30_reactive_node_derivatives,
            &branches,
            &eq30_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_31_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq31_e1188_q: f64 = s.v[742];
        let eq31_e1189: f64 = (p.p87 * s.v[742]);
        let eq31_e1189_d_n0: f64 = (p.p87 * s.dn[742][0]);
        let eq31_e1189_d_n1: f64 = (p.p87 * s.dn[742][1]);
        let eq31_e1189_d_n2: f64 = (p.p87 * s.dn[742][2]);
        let eq31_e1189_d_n3: f64 = (p.p87 * s.dn[742][3]);
        let eq31_e1189_d_n4: f64 = (p.p87 * s.dn[742][4]);
        let eq31_e1189_d_n5: f64 = (p.p87 * s.dn[742][5]);
        let eq31_e1189_d_n6: f64 = (p.p87 * s.dn[742][6]);
        let eq31_e1189_d_n7: f64 = (p.p87 * s.dn[742][7]);
        let eq31_e1189_d_n8: f64 = (p.p87 * s.dn[742][8]);
        let eq31_e1189_d_n9: f64 = (p.p87 * s.dn[742][9]);
        let eq31_e1189_d_n10: f64 = (p.p87 * s.dn[742][10]);
        let eq31_e1189_d_n11: f64 = (p.p87 * s.dn[742][11]);
        let eq31_e1189_d_n12: f64 = (p.p87 * s.dn[742][12]);
        let eq31_e1189_d_n13: f64 = (p.p87 * s.dn[742][13]);
        let eq31_e1189_d_n14: f64 = (p.p87 * s.dn[742][14]);
        let eq31_e1189_d_n15: f64 = (p.p87 * s.dn[742][15]);
        let eq31_e1189_d_n16: f64 = (p.p87 * s.dn[742][16]);
        let eq31_e1189_d_n17: f64 = (p.p87 * s.dn[742][17]);
        let eq31_e1189_d_b0: f64 = (p.p87 * s.db[742][0]);
        let eq31_e1189_d_b1: f64 = (p.p87 * s.db[742][1]);
        let eq31_e1189_d_b2: f64 = (p.p87 * s.db[742][2]);
        let eq31_e1189_d_b3: f64 = (p.p87 * s.db[742][3]);
        let eq31_e1189_d_b4: f64 = (p.p87 * s.db[742][4]);
        let eq31_e1189_d_b5: f64 = (p.p87 * s.db[742][5]);
        let eq31_e1189_d_b6: f64 = (p.p87 * s.db[742][6]);
        let eq31_e1189_d_b7: f64 = (p.p87 * s.db[742][7]);
        let eq31_e1189_d_b8: f64 = (p.p87 * s.db[742][8]);
        let eq31_e1189_q: f64 = (p.p87 * eq31_e1188_q);
        let eq31_e1189_q_d_n0: f64 = (p.p87 * s.dn[742][0]);
        let eq31_e1189_q_d_n1: f64 = (p.p87 * s.dn[742][1]);
        let eq31_e1189_q_d_n2: f64 = (p.p87 * s.dn[742][2]);
        let eq31_e1189_q_d_n3: f64 = (p.p87 * s.dn[742][3]);
        let eq31_e1189_q_d_n4: f64 = (p.p87 * s.dn[742][4]);
        let eq31_e1189_q_d_n5: f64 = (p.p87 * s.dn[742][5]);
        let eq31_e1189_q_d_n6: f64 = (p.p87 * s.dn[742][6]);
        let eq31_e1189_q_d_n7: f64 = (p.p87 * s.dn[742][7]);
        let eq31_e1189_q_d_n8: f64 = (p.p87 * s.dn[742][8]);
        let eq31_e1189_q_d_n9: f64 = (p.p87 * s.dn[742][9]);
        let eq31_e1189_q_d_n10: f64 = (p.p87 * s.dn[742][10]);
        let eq31_e1189_q_d_n11: f64 = (p.p87 * s.dn[742][11]);
        let eq31_e1189_q_d_n12: f64 = (p.p87 * s.dn[742][12]);
        let eq31_e1189_q_d_n13: f64 = (p.p87 * s.dn[742][13]);
        let eq31_e1189_q_d_n14: f64 = (p.p87 * s.dn[742][14]);
        let eq31_e1189_q_d_n15: f64 = (p.p87 * s.dn[742][15]);
        let eq31_e1189_q_d_n16: f64 = (p.p87 * s.dn[742][16]);
        let eq31_e1189_q_d_n17: f64 = (p.p87 * s.dn[742][17]);
        let eq31_e1189_q_d_b0: f64 = (p.p87 * s.db[742][0]);
        let eq31_e1189_q_d_b1: f64 = (p.p87 * s.db[742][1]);
        let eq31_e1189_q_d_b2: f64 = (p.p87 * s.db[742][2]);
        let eq31_e1189_q_d_b3: f64 = (p.p87 * s.db[742][3]);
        let eq31_e1189_q_d_b4: f64 = (p.p87 * s.db[742][4]);
        let eq31_e1189_q_d_b5: f64 = (p.p87 * s.db[742][5]);
        let eq31_e1189_q_d_b6: f64 = (p.p87 * s.db[742][6]);
        let eq31_e1189_q_d_b7: f64 = (p.p87 * s.db[742][7]);
        let eq31_e1189_q_d_b8: f64 = (p.p87 * s.db[742][8]);
        let eq31_reactive_node_derivatives: [f64; 18] = [eq31_e1189_q_d_n0, eq31_e1189_q_d_n1, eq31_e1189_q_d_n2, eq31_e1189_q_d_n3, eq31_e1189_q_d_n4, eq31_e1189_q_d_n5, eq31_e1189_q_d_n6, eq31_e1189_q_d_n7, eq31_e1189_q_d_n8, eq31_e1189_q_d_n9, eq31_e1189_q_d_n10, eq31_e1189_q_d_n11, eq31_e1189_q_d_n12, eq31_e1189_q_d_n13, eq31_e1189_q_d_n14, eq31_e1189_q_d_n15, eq31_e1189_q_d_n16, eq31_e1189_q_d_n17];
        let eq31_reactive_branch_derivatives: [f64; 9] = [eq31_e1189_q_d_b0, eq31_e1189_q_d_b1, eq31_e1189_q_d_b2, eq31_e1189_q_d_b3, eq31_e1189_q_d_b4, eq31_e1189_q_d_b5, eq31_e1189_q_d_b6, eq31_e1189_q_d_b7, eq31_e1189_q_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            &nodes,
            &eq31_reactive_node_derivatives,
            &branches,
            &eq31_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_32_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq32_e1192_q: f64 = s.v[744];
        let eq32_e1193: f64 = (p.p87 * s.v[744]);
        let eq32_e1193_d_n0: f64 = (p.p87 * s.dn[744][0]);
        let eq32_e1193_d_n1: f64 = (p.p87 * s.dn[744][1]);
        let eq32_e1193_d_n2: f64 = (p.p87 * s.dn[744][2]);
        let eq32_e1193_d_n3: f64 = (p.p87 * s.dn[744][3]);
        let eq32_e1193_d_n4: f64 = (p.p87 * s.dn[744][4]);
        let eq32_e1193_d_n5: f64 = (p.p87 * s.dn[744][5]);
        let eq32_e1193_d_n6: f64 = (p.p87 * s.dn[744][6]);
        let eq32_e1193_d_n7: f64 = (p.p87 * s.dn[744][7]);
        let eq32_e1193_d_n8: f64 = (p.p87 * s.dn[744][8]);
        let eq32_e1193_d_n9: f64 = (p.p87 * s.dn[744][9]);
        let eq32_e1193_d_n10: f64 = (p.p87 * s.dn[744][10]);
        let eq32_e1193_d_n11: f64 = (p.p87 * s.dn[744][11]);
        let eq32_e1193_d_n12: f64 = (p.p87 * s.dn[744][12]);
        let eq32_e1193_d_n13: f64 = (p.p87 * s.dn[744][13]);
        let eq32_e1193_d_n14: f64 = (p.p87 * s.dn[744][14]);
        let eq32_e1193_d_n15: f64 = (p.p87 * s.dn[744][15]);
        let eq32_e1193_d_n16: f64 = (p.p87 * s.dn[744][16]);
        let eq32_e1193_d_n17: f64 = (p.p87 * s.dn[744][17]);
        let eq32_e1193_d_b0: f64 = (p.p87 * s.db[744][0]);
        let eq32_e1193_d_b1: f64 = (p.p87 * s.db[744][1]);
        let eq32_e1193_d_b2: f64 = (p.p87 * s.db[744][2]);
        let eq32_e1193_d_b3: f64 = (p.p87 * s.db[744][3]);
        let eq32_e1193_d_b4: f64 = (p.p87 * s.db[744][4]);
        let eq32_e1193_d_b5: f64 = (p.p87 * s.db[744][5]);
        let eq32_e1193_d_b6: f64 = (p.p87 * s.db[744][6]);
        let eq32_e1193_d_b7: f64 = (p.p87 * s.db[744][7]);
        let eq32_e1193_d_b8: f64 = (p.p87 * s.db[744][8]);
        let eq32_e1193_q: f64 = (p.p87 * eq32_e1192_q);
        let eq32_e1193_q_d_n0: f64 = (p.p87 * s.dn[744][0]);
        let eq32_e1193_q_d_n1: f64 = (p.p87 * s.dn[744][1]);
        let eq32_e1193_q_d_n2: f64 = (p.p87 * s.dn[744][2]);
        let eq32_e1193_q_d_n3: f64 = (p.p87 * s.dn[744][3]);
        let eq32_e1193_q_d_n4: f64 = (p.p87 * s.dn[744][4]);
        let eq32_e1193_q_d_n5: f64 = (p.p87 * s.dn[744][5]);
        let eq32_e1193_q_d_n6: f64 = (p.p87 * s.dn[744][6]);
        let eq32_e1193_q_d_n7: f64 = (p.p87 * s.dn[744][7]);
        let eq32_e1193_q_d_n8: f64 = (p.p87 * s.dn[744][8]);
        let eq32_e1193_q_d_n9: f64 = (p.p87 * s.dn[744][9]);
        let eq32_e1193_q_d_n10: f64 = (p.p87 * s.dn[744][10]);
        let eq32_e1193_q_d_n11: f64 = (p.p87 * s.dn[744][11]);
        let eq32_e1193_q_d_n12: f64 = (p.p87 * s.dn[744][12]);
        let eq32_e1193_q_d_n13: f64 = (p.p87 * s.dn[744][13]);
        let eq32_e1193_q_d_n14: f64 = (p.p87 * s.dn[744][14]);
        let eq32_e1193_q_d_n15: f64 = (p.p87 * s.dn[744][15]);
        let eq32_e1193_q_d_n16: f64 = (p.p87 * s.dn[744][16]);
        let eq32_e1193_q_d_n17: f64 = (p.p87 * s.dn[744][17]);
        let eq32_e1193_q_d_b0: f64 = (p.p87 * s.db[744][0]);
        let eq32_e1193_q_d_b1: f64 = (p.p87 * s.db[744][1]);
        let eq32_e1193_q_d_b2: f64 = (p.p87 * s.db[744][2]);
        let eq32_e1193_q_d_b3: f64 = (p.p87 * s.db[744][3]);
        let eq32_e1193_q_d_b4: f64 = (p.p87 * s.db[744][4]);
        let eq32_e1193_q_d_b5: f64 = (p.p87 * s.db[744][5]);
        let eq32_e1193_q_d_b6: f64 = (p.p87 * s.db[744][6]);
        let eq32_e1193_q_d_b7: f64 = (p.p87 * s.db[744][7]);
        let eq32_e1193_q_d_b8: f64 = (p.p87 * s.db[744][8]);
        let eq32_reactive_node_derivatives: [f64; 18] = [eq32_e1193_q_d_n0, eq32_e1193_q_d_n1, eq32_e1193_q_d_n2, eq32_e1193_q_d_n3, eq32_e1193_q_d_n4, eq32_e1193_q_d_n5, eq32_e1193_q_d_n6, eq32_e1193_q_d_n7, eq32_e1193_q_d_n8, eq32_e1193_q_d_n9, eq32_e1193_q_d_n10, eq32_e1193_q_d_n11, eq32_e1193_q_d_n12, eq32_e1193_q_d_n13, eq32_e1193_q_d_n14, eq32_e1193_q_d_n15, eq32_e1193_q_d_n16, eq32_e1193_q_d_n17];
        let eq32_reactive_branch_derivatives: [f64; 9] = [eq32_e1193_q_d_b0, eq32_e1193_q_d_b1, eq32_e1193_q_d_b2, eq32_e1193_q_d_b3, eq32_e1193_q_d_b4, eq32_e1193_q_d_b5, eq32_e1193_q_d_b6, eq32_e1193_q_d_b7, eq32_e1193_q_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            &nodes,
            &eq32_reactive_node_derivatives,
            &branches,
            &eq32_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_33_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq33_e1195: f64 = (-p.p87);
        let eq33_e1197_q: f64 = s.v[299];
        let eq33_e1198: f64 = (eq33_e1195 * s.v[299]);
        let eq33_e1198_d_n0: f64 = (eq33_e1195 * s.dn[299][0]);
        let eq33_e1198_d_n1: f64 = (eq33_e1195 * s.dn[299][1]);
        let eq33_e1198_d_n2: f64 = (eq33_e1195 * s.dn[299][2]);
        let eq33_e1198_d_n3: f64 = (eq33_e1195 * s.dn[299][3]);
        let eq33_e1198_d_n4: f64 = (eq33_e1195 * s.dn[299][4]);
        let eq33_e1198_d_n5: f64 = (eq33_e1195 * s.dn[299][5]);
        let eq33_e1198_d_n6: f64 = (eq33_e1195 * s.dn[299][6]);
        let eq33_e1198_d_n7: f64 = (eq33_e1195 * s.dn[299][7]);
        let eq33_e1198_d_n8: f64 = (eq33_e1195 * s.dn[299][8]);
        let eq33_e1198_d_n9: f64 = (eq33_e1195 * s.dn[299][9]);
        let eq33_e1198_d_n10: f64 = (eq33_e1195 * s.dn[299][10]);
        let eq33_e1198_d_n11: f64 = (eq33_e1195 * s.dn[299][11]);
        let eq33_e1198_d_n12: f64 = (eq33_e1195 * s.dn[299][12]);
        let eq33_e1198_d_n13: f64 = (eq33_e1195 * s.dn[299][13]);
        let eq33_e1198_d_n14: f64 = (eq33_e1195 * s.dn[299][14]);
        let eq33_e1198_d_n15: f64 = (eq33_e1195 * s.dn[299][15]);
        let eq33_e1198_d_n16: f64 = (eq33_e1195 * s.dn[299][16]);
        let eq33_e1198_d_n17: f64 = (eq33_e1195 * s.dn[299][17]);
        let eq33_e1198_d_b0: f64 = (eq33_e1195 * s.db[299][0]);
        let eq33_e1198_d_b1: f64 = (eq33_e1195 * s.db[299][1]);
        let eq33_e1198_d_b2: f64 = (eq33_e1195 * s.db[299][2]);
        let eq33_e1198_d_b3: f64 = (eq33_e1195 * s.db[299][3]);
        let eq33_e1198_d_b4: f64 = (eq33_e1195 * s.db[299][4]);
        let eq33_e1198_d_b5: f64 = (eq33_e1195 * s.db[299][5]);
        let eq33_e1198_d_b6: f64 = (eq33_e1195 * s.db[299][6]);
        let eq33_e1198_d_b7: f64 = (eq33_e1195 * s.db[299][7]);
        let eq33_e1198_d_b8: f64 = (eq33_e1195 * s.db[299][8]);
        let eq33_e1198_q: f64 = (eq33_e1195 * eq33_e1197_q);
        let eq33_e1198_q_d_n0: f64 = (eq33_e1195 * s.dn[299][0]);
        let eq33_e1198_q_d_n1: f64 = (eq33_e1195 * s.dn[299][1]);
        let eq33_e1198_q_d_n2: f64 = (eq33_e1195 * s.dn[299][2]);
        let eq33_e1198_q_d_n3: f64 = (eq33_e1195 * s.dn[299][3]);
        let eq33_e1198_q_d_n4: f64 = (eq33_e1195 * s.dn[299][4]);
        let eq33_e1198_q_d_n5: f64 = (eq33_e1195 * s.dn[299][5]);
        let eq33_e1198_q_d_n6: f64 = (eq33_e1195 * s.dn[299][6]);
        let eq33_e1198_q_d_n7: f64 = (eq33_e1195 * s.dn[299][7]);
        let eq33_e1198_q_d_n8: f64 = (eq33_e1195 * s.dn[299][8]);
        let eq33_e1198_q_d_n9: f64 = (eq33_e1195 * s.dn[299][9]);
        let eq33_e1198_q_d_n10: f64 = (eq33_e1195 * s.dn[299][10]);
        let eq33_e1198_q_d_n11: f64 = (eq33_e1195 * s.dn[299][11]);
        let eq33_e1198_q_d_n12: f64 = (eq33_e1195 * s.dn[299][12]);
        let eq33_e1198_q_d_n13: f64 = (eq33_e1195 * s.dn[299][13]);
        let eq33_e1198_q_d_n14: f64 = (eq33_e1195 * s.dn[299][14]);
        let eq33_e1198_q_d_n15: f64 = (eq33_e1195 * s.dn[299][15]);
        let eq33_e1198_q_d_n16: f64 = (eq33_e1195 * s.dn[299][16]);
        let eq33_e1198_q_d_n17: f64 = (eq33_e1195 * s.dn[299][17]);
        let eq33_e1198_q_d_b0: f64 = (eq33_e1195 * s.db[299][0]);
        let eq33_e1198_q_d_b1: f64 = (eq33_e1195 * s.db[299][1]);
        let eq33_e1198_q_d_b2: f64 = (eq33_e1195 * s.db[299][2]);
        let eq33_e1198_q_d_b3: f64 = (eq33_e1195 * s.db[299][3]);
        let eq33_e1198_q_d_b4: f64 = (eq33_e1195 * s.db[299][4]);
        let eq33_e1198_q_d_b5: f64 = (eq33_e1195 * s.db[299][5]);
        let eq33_e1198_q_d_b6: f64 = (eq33_e1195 * s.db[299][6]);
        let eq33_e1198_q_d_b7: f64 = (eq33_e1195 * s.db[299][7]);
        let eq33_e1198_q_d_b8: f64 = (eq33_e1195 * s.db[299][8]);
        let eq33_reactive_node_derivatives: [f64; 18] = [eq33_e1198_q_d_n0, eq33_e1198_q_d_n1, eq33_e1198_q_d_n2, eq33_e1198_q_d_n3, eq33_e1198_q_d_n4, eq33_e1198_q_d_n5, eq33_e1198_q_d_n6, eq33_e1198_q_d_n7, eq33_e1198_q_d_n8, eq33_e1198_q_d_n9, eq33_e1198_q_d_n10, eq33_e1198_q_d_n11, eq33_e1198_q_d_n12, eq33_e1198_q_d_n13, eq33_e1198_q_d_n14, eq33_e1198_q_d_n15, eq33_e1198_q_d_n16, eq33_e1198_q_d_n17];
        let eq33_reactive_branch_derivatives: [f64; 9] = [eq33_e1198_q_d_b0, eq33_e1198_q_d_b1, eq33_e1198_q_d_b2, eq33_e1198_q_d_b3, eq33_e1198_q_d_b4, eq33_e1198_q_d_b5, eq33_e1198_q_d_b6, eq33_e1198_q_d_b7, eq33_e1198_q_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[0]),
            &nodes,
            &eq33_reactive_node_derivatives,
            &branches,
            &eq33_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
