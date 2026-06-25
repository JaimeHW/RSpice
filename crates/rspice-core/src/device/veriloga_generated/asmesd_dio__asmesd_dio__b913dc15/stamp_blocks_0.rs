#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_0_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq0_e60, eq0_e60_d_n0, eq0_e60_d_n1, eq0_e60_d_n2, eq0_e60_d_n3, eq0_e60_d_n4, eq0_e60_d_n5, eq0_e60_d_n6, eq0_e60_d_b0, eq0_e60_d_b1, eq0_e60_d_b2, eq0_e60_d_b3, eq0_e60_d_b4, eq0_e60_d_b5, eq0_e60_d_b6,) = {
    if (s.v[68] != 0.0) {
        let eq0_e56: f64 = (-s.v[23]);
        let eq0_e56_d_n0: f64 = (-s.dn[23][0]);
        let eq0_e56_d_n1: f64 = (-s.dn[23][1]);
        let eq0_e56_d_n2: f64 = (-s.dn[23][2]);
        let eq0_e56_d_n3: f64 = (-s.dn[23][3]);
        let eq0_e56_d_n4: f64 = (-s.dn[23][4]);
        let eq0_e56_d_n5: f64 = (-s.dn[23][5]);
        let eq0_e56_d_n6: f64 = (-s.dn[23][6]);
        let eq0_e56_d_b0: f64 = (-s.db[23][0]);
        let eq0_e56_d_b1: f64 = (-s.db[23][1]);
        let eq0_e56_d_b2: f64 = (-s.db[23][2]);
        let eq0_e56_d_b3: f64 = (-s.db[23][3]);
        let eq0_e56_d_b4: f64 = (-s.db[23][4]);
        let eq0_e56_d_b5: f64 = (-s.db[23][5]);
        let eq0_e56_d_b6: f64 = (-s.db[23][6]);
        let eq0_e58: f64 = (eq0_e56 * s.v[31]);
        let eq0_e58_d_n0: f64 = ((eq0_e56_d_n0 * s.v[31]) + (eq0_e56 * s.dn[31][0]));
        let eq0_e58_d_n1: f64 = ((eq0_e56_d_n1 * s.v[31]) + (eq0_e56 * s.dn[31][1]));
        let eq0_e58_d_n2: f64 = ((eq0_e56_d_n2 * s.v[31]) + (eq0_e56 * s.dn[31][2]));
        let eq0_e58_d_n3: f64 = ((eq0_e56_d_n3 * s.v[31]) + (eq0_e56 * s.dn[31][3]));
        let eq0_e58_d_n4: f64 = ((eq0_e56_d_n4 * s.v[31]) + (eq0_e56 * s.dn[31][4]));
        let eq0_e58_d_n5: f64 = ((eq0_e56_d_n5 * s.v[31]) + (eq0_e56 * s.dn[31][5]));
        let eq0_e58_d_n6: f64 = ((eq0_e56_d_n6 * s.v[31]) + (eq0_e56 * s.dn[31][6]));
        let eq0_e58_d_b0: f64 = ((eq0_e56_d_b0 * s.v[31]) + (eq0_e56 * s.db[31][0]));
        let eq0_e58_d_b1: f64 = ((eq0_e56_d_b1 * s.v[31]) + (eq0_e56 * s.db[31][1]));
        let eq0_e58_d_b2: f64 = ((eq0_e56_d_b2 * s.v[31]) + (eq0_e56 * s.db[31][2]));
        let eq0_e58_d_b3: f64 = ((eq0_e56_d_b3 * s.v[31]) + (eq0_e56 * s.db[31][3]));
        let eq0_e58_d_b4: f64 = ((eq0_e56_d_b4 * s.v[31]) + (eq0_e56 * s.db[31][4]));
        let eq0_e58_d_b5: f64 = ((eq0_e56_d_b5 * s.v[31]) + (eq0_e56 * s.db[31][5]));
        let eq0_e58_d_b6: f64 = ((eq0_e56_d_b6 * s.v[31]) + (eq0_e56 * s.db[31][6]));
        (eq0_e58, eq0_e58_d_n0, eq0_e58_d_n1, eq0_e58_d_n2, eq0_e58_d_n3, eq0_e58_d_n4, eq0_e58_d_n5, eq0_e58_d_n6, eq0_e58_d_b0, eq0_e58_d_b1, eq0_e58_d_b2, eq0_e58_d_b3, eq0_e58_d_b4, eq0_e58_d_b5, eq0_e58_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e60;
        let eq0_node_derivatives: [f64; 7] = [eq0_e60_d_n0, eq0_e60_d_n1, eq0_e60_d_n2, eq0_e60_d_n3, eq0_e60_d_n4, eq0_e60_d_n5, eq0_e60_d_n6];
        let eq0_branch_derivatives: [f64; 7] = [eq0_e60_d_b0, eq0_e60_d_b1, eq0_e60_d_b2, eq0_e60_d_b3, eq0_e60_d_b4, eq0_e60_d_b5, eq0_e60_d_b6];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            None,
            self.multiplicity * (eq0_value),
            &nodes,
            &eq0_node_derivatives,
            &branches,
            &eq0_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_1_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq1_e66, eq1_e66_d_n6,) = {
    if (s.v[68] != 0.0) {
        let eq1_e64: f64 = (nv6 - 0.0);
        (eq1_e64, 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e66;
        stamper.stamp_current(
            Some(nodes[6]),
            None,
            self.multiplicity * (eq1_value),
            &[
                GeneratedDerivative::node(nodes[6], self.multiplicity * eq1_e66_d_n6),
            ],
        );
    }

    pub(super) fn stamp_transient_equation_2_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq2_e73, eq2_e73_d_n0, eq2_e73_d_n1, eq2_e73_d_n2, eq2_e73_d_n3, eq2_e73_d_n4, eq2_e73_d_n5, eq2_e73_d_n6, eq2_e73_d_b0, eq2_e73_d_b1, eq2_e73_d_b2, eq2_e73_d_b3, eq2_e73_d_b4, eq2_e73_d_b5, eq2_e73_d_b6,) = {
    if (s.v[68] != 0.0) {
        let eq2_e70: f64 = self.eval_ddt(0, (nv6 - 0.0));
        let eq2_e70_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq2_e70_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq2_e70_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq2_e70_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq2_e70_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq2_e70_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq2_e70_d_n6: f64 = self.ddt_jacobian(1.0);
        let eq2_e70_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq2_e70_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq2_e70_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq2_e70_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq2_e70_d_b4: f64 = self.ddt_jacobian(0.0);
        let eq2_e70_d_b5: f64 = self.ddt_jacobian(0.0);
        let eq2_e70_d_b6: f64 = self.ddt_jacobian(0.0);
        let eq2_e71: f64 = (s.v[31] * eq2_e70);
        let eq2_e71_d_n0: f64 = ((s.dn[31][0] * eq2_e70) + (s.v[31] * eq2_e70_d_n0));
        let eq2_e71_d_n1: f64 = ((s.dn[31][1] * eq2_e70) + (s.v[31] * eq2_e70_d_n1));
        let eq2_e71_d_n2: f64 = ((s.dn[31][2] * eq2_e70) + (s.v[31] * eq2_e70_d_n2));
        let eq2_e71_d_n3: f64 = ((s.dn[31][3] * eq2_e70) + (s.v[31] * eq2_e70_d_n3));
        let eq2_e71_d_n4: f64 = ((s.dn[31][4] * eq2_e70) + (s.v[31] * eq2_e70_d_n4));
        let eq2_e71_d_n5: f64 = ((s.dn[31][5] * eq2_e70) + (s.v[31] * eq2_e70_d_n5));
        let eq2_e71_d_n6: f64 = ((s.dn[31][6] * eq2_e70) + (s.v[31] * eq2_e70_d_n6));
        let eq2_e71_d_b0: f64 = ((s.db[31][0] * eq2_e70) + (s.v[31] * eq2_e70_d_b0));
        let eq2_e71_d_b1: f64 = ((s.db[31][1] * eq2_e70) + (s.v[31] * eq2_e70_d_b1));
        let eq2_e71_d_b2: f64 = ((s.db[31][2] * eq2_e70) + (s.v[31] * eq2_e70_d_b2));
        let eq2_e71_d_b3: f64 = ((s.db[31][3] * eq2_e70) + (s.v[31] * eq2_e70_d_b3));
        let eq2_e71_d_b4: f64 = ((s.db[31][4] * eq2_e70) + (s.v[31] * eq2_e70_d_b4));
        let eq2_e71_d_b5: f64 = ((s.db[31][5] * eq2_e70) + (s.v[31] * eq2_e70_d_b5));
        let eq2_e71_d_b6: f64 = ((s.db[31][6] * eq2_e70) + (s.v[31] * eq2_e70_d_b6));
        (eq2_e71, eq2_e71_d_n0, eq2_e71_d_n1, eq2_e71_d_n2, eq2_e71_d_n3, eq2_e71_d_n4, eq2_e71_d_n5, eq2_e71_d_n6, eq2_e71_d_b0, eq2_e71_d_b1, eq2_e71_d_b2, eq2_e71_d_b3, eq2_e71_d_b4, eq2_e71_d_b5, eq2_e71_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e73;
        let eq2_node_derivatives: [f64; 7] = [eq2_e73_d_n0, eq2_e73_d_n1, eq2_e73_d_n2, eq2_e73_d_n3, eq2_e73_d_n4, eq2_e73_d_n5, eq2_e73_d_n6];
        let eq2_branch_derivatives: [f64; 7] = [eq2_e73_d_b0, eq2_e73_d_b1, eq2_e73_d_b2, eq2_e73_d_b3, eq2_e73_d_b4, eq2_e73_d_b5, eq2_e73_d_b6];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            None,
            self.multiplicity * (eq2_value),
            &nodes,
            &eq2_node_derivatives,
            &branches,
            &eq2_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_3_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq3_e78,) = {
    if (!(s.v[68] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq3_value: f64 = eq3_e78;
        stamper.stamp_potential(
            branches[0],
            eq3_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_4_block_0(
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
        let nv1 = ctx.node_voltage(nodes[1]);
        let (eq4_e88, eq4_e88_d_n0, eq4_e88_d_n1, eq4_e88_d_n2, eq4_e88_d_n3, eq4_e88_d_n4, eq4_e88_d_n5, eq4_e88_d_n6, eq4_e88_d_b0, eq4_e88_d_b1, eq4_e88_d_b2, eq4_e88_d_b3, eq4_e88_d_b4, eq4_e88_d_b5, eq4_e88_d_b6,) = {
    if (s.v[70] != 0.0) {
        let eq4_e81: f64 = (-1.0);
        let eq4_e84: f64 = (s.v[24] * (nv0 - nv1));
        let eq4_e84_d_n0: f64 = ((s.dn[24][0] * (nv0 - nv1)) + s.v[24]);
        let eq4_e84_d_n1: f64 = ((s.dn[24][1] * (nv0 - nv1)) + (-s.v[24]));
        let eq4_e84_d_n2: f64 = (s.dn[24][2] * (nv0 - nv1));
        let eq4_e84_d_n3: f64 = (s.dn[24][3] * (nv0 - nv1));
        let eq4_e84_d_n4: f64 = (s.dn[24][4] * (nv0 - nv1));
        let eq4_e84_d_n5: f64 = (s.dn[24][5] * (nv0 - nv1));
        let eq4_e84_d_n6: f64 = (s.dn[24][6] * (nv0 - nv1));
        let eq4_e84_d_b0: f64 = (s.db[24][0] * (nv0 - nv1));
        let eq4_e84_d_b1: f64 = (s.db[24][1] * (nv0 - nv1));
        let eq4_e84_d_b2: f64 = (s.db[24][2] * (nv0 - nv1));
        let eq4_e84_d_b3: f64 = (s.db[24][3] * (nv0 - nv1));
        let eq4_e84_d_b4: f64 = (s.db[24][4] * (nv0 - nv1));
        let eq4_e84_d_b5: f64 = (s.db[24][5] * (nv0 - nv1));
        let eq4_e84_d_b6: f64 = (s.db[24][6] * (nv0 - nv1));
        let eq4_e85: f64 = (eq4_e84).abs();
        let eq4_e85_d_n0: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_n0 } else { (-eq4_e84_d_n0) };
        let eq4_e85_d_n1: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_n1 } else { (-eq4_e84_d_n1) };
        let eq4_e85_d_n2: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_n2 } else { (-eq4_e84_d_n2) };
        let eq4_e85_d_n3: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_n3 } else { (-eq4_e84_d_n3) };
        let eq4_e85_d_n4: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_n4 } else { (-eq4_e84_d_n4) };
        let eq4_e85_d_n5: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_n5 } else { (-eq4_e84_d_n5) };
        let eq4_e85_d_n6: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_n6 } else { (-eq4_e84_d_n6) };
        let eq4_e85_d_b0: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_b0 } else { (-eq4_e84_d_b0) };
        let eq4_e85_d_b1: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_b1 } else { (-eq4_e84_d_b1) };
        let eq4_e85_d_b2: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_b2 } else { (-eq4_e84_d_b2) };
        let eq4_e85_d_b3: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_b3 } else { (-eq4_e84_d_b3) };
        let eq4_e85_d_b4: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_b4 } else { (-eq4_e84_d_b4) };
        let eq4_e85_d_b5: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_b5 } else { (-eq4_e84_d_b5) };
        let eq4_e85_d_b6: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_b6 } else { (-eq4_e84_d_b6) };
        let eq4_e86: f64 = (eq4_e81 * eq4_e85);
        let eq4_e86_d_n0: f64 = (eq4_e81 * eq4_e85_d_n0);
        let eq4_e86_d_n1: f64 = (eq4_e81 * eq4_e85_d_n1);
        let eq4_e86_d_n2: f64 = (eq4_e81 * eq4_e85_d_n2);
        let eq4_e86_d_n3: f64 = (eq4_e81 * eq4_e85_d_n3);
        let eq4_e86_d_n4: f64 = (eq4_e81 * eq4_e85_d_n4);
        let eq4_e86_d_n5: f64 = (eq4_e81 * eq4_e85_d_n5);
        let eq4_e86_d_n6: f64 = (eq4_e81 * eq4_e85_d_n6);
        let eq4_e86_d_b0: f64 = (eq4_e81 * eq4_e85_d_b0);
        let eq4_e86_d_b1: f64 = (eq4_e81 * eq4_e85_d_b1);
        let eq4_e86_d_b2: f64 = (eq4_e81 * eq4_e85_d_b2);
        let eq4_e86_d_b3: f64 = (eq4_e81 * eq4_e85_d_b3);
        let eq4_e86_d_b4: f64 = (eq4_e81 * eq4_e85_d_b4);
        let eq4_e86_d_b5: f64 = (eq4_e81 * eq4_e85_d_b5);
        let eq4_e86_d_b6: f64 = (eq4_e81 * eq4_e85_d_b6);
        (eq4_e86, eq4_e86_d_n0, eq4_e86_d_n1, eq4_e86_d_n2, eq4_e86_d_n3, eq4_e86_d_n4, eq4_e86_d_n5, eq4_e86_d_n6, eq4_e86_d_b0, eq4_e86_d_b1, eq4_e86_d_b2, eq4_e86_d_b3, eq4_e86_d_b4, eq4_e86_d_b5, eq4_e86_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e88;
        let eq4_node_derivatives: [f64; 7] = [eq4_e88_d_n0, eq4_e88_d_n1, eq4_e88_d_n2, eq4_e88_d_n3, eq4_e88_d_n4, eq4_e88_d_n5, eq4_e88_d_n6];
        let eq4_branch_derivatives: [f64; 7] = [eq4_e88_d_b0, eq4_e88_d_b1, eq4_e88_d_b2, eq4_e88_d_b3, eq4_e88_d_b4, eq4_e88_d_b5, eq4_e88_d_b6];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            None,
            self.multiplicity * (eq4_value),
            &nodes,
            &eq4_node_derivatives,
            &branches,
            &eq4_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_5_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv2 = ctx.node_voltage(nodes[2]);
        let (eq5_e94, eq5_e94_d_n2,) = {
    if (s.v[70] != 0.0) {
        let eq5_e92: f64 = ((nv2 - 0.0) / p.p33);
        let eq5_e92_d_n2: f64 = (1.0 / p.p33);
        (eq5_e92, eq5_e92_d_n2,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e94;
        stamper.stamp_current(
            Some(nodes[2]),
            None,
            self.multiplicity * (eq5_value),
            &[
                GeneratedDerivative::node(nodes[2], self.multiplicity * eq5_e94_d_n2),
            ],
        );
    }

    pub(super) fn stamp_transient_equation_6_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv2 = ctx.node_voltage(nodes[2]);
        let (eq6_e101, eq6_e101_d_n0, eq6_e101_d_n1, eq6_e101_d_n2, eq6_e101_d_n3, eq6_e101_d_n4, eq6_e101_d_n5, eq6_e101_d_n6, eq6_e101_d_b0, eq6_e101_d_b1, eq6_e101_d_b2, eq6_e101_d_b3, eq6_e101_d_b4, eq6_e101_d_b5, eq6_e101_d_b6,) = {
    if (s.v[70] != 0.0) {
        let eq6_e98: f64 = ((nv2 - 0.0) * p.p34);
        let eq6_e98_d_n2: f64 = p.p34;
        let eq6_e99: f64 = self.eval_ddt(1, eq6_e98);
        let eq6_e99_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq6_e99_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq6_e99_d_n2: f64 = self.ddt_jacobian(eq6_e98_d_n2);
        let eq6_e99_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq6_e99_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq6_e99_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq6_e99_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq6_e99_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq6_e99_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq6_e99_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq6_e99_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq6_e99_d_b4: f64 = self.ddt_jacobian(0.0);
        let eq6_e99_d_b5: f64 = self.ddt_jacobian(0.0);
        let eq6_e99_d_b6: f64 = self.ddt_jacobian(0.0);
        (eq6_e99, eq6_e99_d_n0, eq6_e99_d_n1, eq6_e99_d_n2, eq6_e99_d_n3, eq6_e99_d_n4, eq6_e99_d_n5, eq6_e99_d_n6, eq6_e99_d_b0, eq6_e99_d_b1, eq6_e99_d_b2, eq6_e99_d_b3, eq6_e99_d_b4, eq6_e99_d_b5, eq6_e99_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e101;
        let eq6_node_derivatives: [f64; 7] = [eq6_e101_d_n0, eq6_e101_d_n1, eq6_e101_d_n2, eq6_e101_d_n3, eq6_e101_d_n4, eq6_e101_d_n5, eq6_e101_d_n6];
        let eq6_branch_derivatives: [f64; 7] = [eq6_e101_d_b0, eq6_e101_d_b1, eq6_e101_d_b2, eq6_e101_d_b3, eq6_e101_d_b4, eq6_e101_d_b5, eq6_e101_d_b6];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            None,
            self.multiplicity * (eq6_value),
            &nodes,
            &eq6_node_derivatives,
            &branches,
            &eq6_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_7_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq7_e105,) = {
    if (s.v[70] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq7_value: f64 = eq7_e105;
        stamper.stamp_potential(
            branches[1],
            eq7_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_8_block_0(
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
        let nv1 = ctx.node_voltage(nodes[1]);
        let (eq8_e118, eq8_e118_d_n0, eq8_e118_d_n1, eq8_e118_d_n2, eq8_e118_d_n3, eq8_e118_d_n4, eq8_e118_d_n5, eq8_e118_d_n6, eq8_e118_d_b0, eq8_e118_d_b1, eq8_e118_d_b2, eq8_e118_d_b3, eq8_e118_d_b4, eq8_e118_d_b5, eq8_e118_d_b6,) = {
    if ((!(s.v[70] != 0.0)) && (s.v[71] != 0.0)) {
        let eq8_e111: f64 = (-1.0);
        let eq8_e114: f64 = (s.v[24] * (nv0 - nv1));
        let eq8_e114_d_n0: f64 = ((s.dn[24][0] * (nv0 - nv1)) + s.v[24]);
        let eq8_e114_d_n1: f64 = ((s.dn[24][1] * (nv0 - nv1)) + (-s.v[24]));
        let eq8_e114_d_n2: f64 = (s.dn[24][2] * (nv0 - nv1));
        let eq8_e114_d_n3: f64 = (s.dn[24][3] * (nv0 - nv1));
        let eq8_e114_d_n4: f64 = (s.dn[24][4] * (nv0 - nv1));
        let eq8_e114_d_n5: f64 = (s.dn[24][5] * (nv0 - nv1));
        let eq8_e114_d_n6: f64 = (s.dn[24][6] * (nv0 - nv1));
        let eq8_e114_d_b0: f64 = (s.db[24][0] * (nv0 - nv1));
        let eq8_e114_d_b1: f64 = (s.db[24][1] * (nv0 - nv1));
        let eq8_e114_d_b2: f64 = (s.db[24][2] * (nv0 - nv1));
        let eq8_e114_d_b3: f64 = (s.db[24][3] * (nv0 - nv1));
        let eq8_e114_d_b4: f64 = (s.db[24][4] * (nv0 - nv1));
        let eq8_e114_d_b5: f64 = (s.db[24][5] * (nv0 - nv1));
        let eq8_e114_d_b6: f64 = (s.db[24][6] * (nv0 - nv1));
        let eq8_e115: f64 = (eq8_e114).abs();
        let eq8_e115_d_n0: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_n0 } else { (-eq8_e114_d_n0) };
        let eq8_e115_d_n1: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_n1 } else { (-eq8_e114_d_n1) };
        let eq8_e115_d_n2: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_n2 } else { (-eq8_e114_d_n2) };
        let eq8_e115_d_n3: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_n3 } else { (-eq8_e114_d_n3) };
        let eq8_e115_d_n4: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_n4 } else { (-eq8_e114_d_n4) };
        let eq8_e115_d_n5: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_n5 } else { (-eq8_e114_d_n5) };
        let eq8_e115_d_n6: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_n6 } else { (-eq8_e114_d_n6) };
        let eq8_e115_d_b0: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_b0 } else { (-eq8_e114_d_b0) };
        let eq8_e115_d_b1: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_b1 } else { (-eq8_e114_d_b1) };
        let eq8_e115_d_b2: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_b2 } else { (-eq8_e114_d_b2) };
        let eq8_e115_d_b3: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_b3 } else { (-eq8_e114_d_b3) };
        let eq8_e115_d_b4: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_b4 } else { (-eq8_e114_d_b4) };
        let eq8_e115_d_b5: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_b5 } else { (-eq8_e114_d_b5) };
        let eq8_e115_d_b6: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_b6 } else { (-eq8_e114_d_b6) };
        let eq8_e116: f64 = (eq8_e111 * eq8_e115);
        let eq8_e116_d_n0: f64 = (eq8_e111 * eq8_e115_d_n0);
        let eq8_e116_d_n1: f64 = (eq8_e111 * eq8_e115_d_n1);
        let eq8_e116_d_n2: f64 = (eq8_e111 * eq8_e115_d_n2);
        let eq8_e116_d_n3: f64 = (eq8_e111 * eq8_e115_d_n3);
        let eq8_e116_d_n4: f64 = (eq8_e111 * eq8_e115_d_n4);
        let eq8_e116_d_n5: f64 = (eq8_e111 * eq8_e115_d_n5);
        let eq8_e116_d_n6: f64 = (eq8_e111 * eq8_e115_d_n6);
        let eq8_e116_d_b0: f64 = (eq8_e111 * eq8_e115_d_b0);
        let eq8_e116_d_b1: f64 = (eq8_e111 * eq8_e115_d_b1);
        let eq8_e116_d_b2: f64 = (eq8_e111 * eq8_e115_d_b2);
        let eq8_e116_d_b3: f64 = (eq8_e111 * eq8_e115_d_b3);
        let eq8_e116_d_b4: f64 = (eq8_e111 * eq8_e115_d_b4);
        let eq8_e116_d_b5: f64 = (eq8_e111 * eq8_e115_d_b5);
        let eq8_e116_d_b6: f64 = (eq8_e111 * eq8_e115_d_b6);
        (eq8_e116, eq8_e116_d_n0, eq8_e116_d_n1, eq8_e116_d_n2, eq8_e116_d_n3, eq8_e116_d_n4, eq8_e116_d_n5, eq8_e116_d_n6, eq8_e116_d_b0, eq8_e116_d_b1, eq8_e116_d_b2, eq8_e116_d_b3, eq8_e116_d_b4, eq8_e116_d_b5, eq8_e116_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e118;
        let eq8_node_derivatives: [f64; 7] = [eq8_e118_d_n0, eq8_e118_d_n1, eq8_e118_d_n2, eq8_e118_d_n3, eq8_e118_d_n4, eq8_e118_d_n5, eq8_e118_d_n6];
        let eq8_branch_derivatives: [f64; 7] = [eq8_e118_d_b0, eq8_e118_d_b1, eq8_e118_d_b2, eq8_e118_d_b3, eq8_e118_d_b4, eq8_e118_d_b5, eq8_e118_d_b6];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            None,
            self.multiplicity * (eq8_value),
            &nodes,
            &eq8_node_derivatives,
            &branches,
            &eq8_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_9_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq9_e127, eq9_e127_d_n2, eq9_e127_d_n5,) = {
    if ((!(s.v[70] != 0.0)) && (s.v[71] != 0.0)) {
        let eq9_e125: f64 = ((nv2 - nv5) / p.p33);
        let eq9_e125_d_n2: f64 = (1.0 / p.p33);
        let eq9_e125_d_n5: f64 = (-1.0 / p.p33);
        (eq9_e125, eq9_e125_d_n2, eq9_e125_d_n5,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e127;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[5]),
            self.multiplicity * (eq9_value),
            &[
                GeneratedDerivative::node(nodes[2], self.multiplicity * eq9_e127_d_n2),
                GeneratedDerivative::node(nodes[5], self.multiplicity * eq9_e127_d_n5),
            ],
        );
    }

    pub(super) fn stamp_transient_equation_10_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv2 = ctx.node_voltage(nodes[2]);
        let (eq10_e137, eq10_e137_d_n0, eq10_e137_d_n1, eq10_e137_d_n2, eq10_e137_d_n3, eq10_e137_d_n4, eq10_e137_d_n5, eq10_e137_d_n6, eq10_e137_d_b0, eq10_e137_d_b1, eq10_e137_d_b2, eq10_e137_d_b3, eq10_e137_d_b4, eq10_e137_d_b5, eq10_e137_d_b6,) = {
    if ((!(s.v[70] != 0.0)) && (s.v[71] != 0.0)) {
        let eq10_e134: f64 = (p.p34 * (nv2 - 0.0));
        let eq10_e134_d_n2: f64 = p.p34;
        let eq10_e135: f64 = self.eval_ddt(2, eq10_e134);
        let eq10_e135_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq10_e135_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq10_e135_d_n2: f64 = self.ddt_jacobian(eq10_e134_d_n2);
        let eq10_e135_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq10_e135_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq10_e135_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq10_e135_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq10_e135_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq10_e135_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq10_e135_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq10_e135_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq10_e135_d_b4: f64 = self.ddt_jacobian(0.0);
        let eq10_e135_d_b5: f64 = self.ddt_jacobian(0.0);
        let eq10_e135_d_b6: f64 = self.ddt_jacobian(0.0);
        (eq10_e135, eq10_e135_d_n0, eq10_e135_d_n1, eq10_e135_d_n2, eq10_e135_d_n3, eq10_e135_d_n4, eq10_e135_d_n5, eq10_e135_d_n6, eq10_e135_d_b0, eq10_e135_d_b1, eq10_e135_d_b2, eq10_e135_d_b3, eq10_e135_d_b4, eq10_e135_d_b5, eq10_e135_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e137;
        let eq10_node_derivatives: [f64; 7] = [eq10_e137_d_n0, eq10_e137_d_n1, eq10_e137_d_n2, eq10_e137_d_n3, eq10_e137_d_n4, eq10_e137_d_n5, eq10_e137_d_n6];
        let eq10_branch_derivatives: [f64; 7] = [eq10_e137_d_b0, eq10_e137_d_b1, eq10_e137_d_b2, eq10_e137_d_b3, eq10_e137_d_b4, eq10_e137_d_b5, eq10_e137_d_b6];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            None,
            self.multiplicity * (eq10_value),
            &nodes,
            &eq10_node_derivatives,
            &branches,
            &eq10_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_11_block_0(
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
        let (eq11_e146, eq11_e146_d_n5,) = {
    if ((!(s.v[70] != 0.0)) && (s.v[71] != 0.0)) {
        let eq11_e144: f64 = ((nv5 - 0.0) / p.p35);
        let eq11_e144_d_n5: f64 = (1.0 / p.p35);
        (eq11_e144, eq11_e144_d_n5,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e146;
        stamper.stamp_current(
            Some(nodes[5]),
            None,
            self.multiplicity * (eq11_value),
            &[
                GeneratedDerivative::node(nodes[5], self.multiplicity * eq11_e146_d_n5),
            ],
        );
    }

    pub(super) fn stamp_transient_equation_12_block_0(
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
        let (eq12_e156, eq12_e156_d_n0, eq12_e156_d_n1, eq12_e156_d_n2, eq12_e156_d_n3, eq12_e156_d_n4, eq12_e156_d_n5, eq12_e156_d_n6, eq12_e156_d_b0, eq12_e156_d_b1, eq12_e156_d_b2, eq12_e156_d_b3, eq12_e156_d_b4, eq12_e156_d_b5, eq12_e156_d_b6,) = {
    if ((!(s.v[70] != 0.0)) && (s.v[71] != 0.0)) {
        let eq12_e153: f64 = (p.p36 * (nv5 - 0.0));
        let eq12_e153_d_n5: f64 = p.p36;
        let eq12_e154: f64 = self.eval_ddt(3, eq12_e153);
        let eq12_e154_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq12_e154_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq12_e154_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq12_e154_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq12_e154_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq12_e154_d_n5: f64 = self.ddt_jacobian(eq12_e153_d_n5);
        let eq12_e154_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq12_e154_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq12_e154_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq12_e154_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq12_e154_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq12_e154_d_b4: f64 = self.ddt_jacobian(0.0);
        let eq12_e154_d_b5: f64 = self.ddt_jacobian(0.0);
        let eq12_e154_d_b6: f64 = self.ddt_jacobian(0.0);
        (eq12_e154, eq12_e154_d_n0, eq12_e154_d_n1, eq12_e154_d_n2, eq12_e154_d_n3, eq12_e154_d_n4, eq12_e154_d_n5, eq12_e154_d_n6, eq12_e154_d_b0, eq12_e154_d_b1, eq12_e154_d_b2, eq12_e154_d_b3, eq12_e154_d_b4, eq12_e154_d_b5, eq12_e154_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq12_value: f64 = eq12_e156;
        let eq12_node_derivatives: [f64; 7] = [eq12_e156_d_n0, eq12_e156_d_n1, eq12_e156_d_n2, eq12_e156_d_n3, eq12_e156_d_n4, eq12_e156_d_n5, eq12_e156_d_n6];
        let eq12_branch_derivatives: [f64; 7] = [eq12_e156_d_b0, eq12_e156_d_b1, eq12_e156_d_b2, eq12_e156_d_b3, eq12_e156_d_b4, eq12_e156_d_b5, eq12_e156_d_b6];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            None,
            self.multiplicity * (eq12_value),
            &nodes,
            &eq12_node_derivatives,
            &branches,
            &eq12_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_13_block_0(
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
        let nv1 = ctx.node_voltage(nodes[1]);
        let (eq13_e172, eq13_e172_d_n0, eq13_e172_d_n1, eq13_e172_d_n2, eq13_e172_d_n3, eq13_e172_d_n4, eq13_e172_d_n5, eq13_e172_d_n6, eq13_e172_d_b0, eq13_e172_d_b1, eq13_e172_d_b2, eq13_e172_d_b3, eq13_e172_d_b4, eq13_e172_d_b5, eq13_e172_d_b6,) = {
    if (((!(s.v[70] != 0.0)) && (!(s.v[71] != 0.0))) && (s.v[72] != 0.0)) {
        let eq13_e165: f64 = (-1.0);
        let eq13_e168: f64 = (s.v[24] * (nv0 - nv1));
        let eq13_e168_d_n0: f64 = ((s.dn[24][0] * (nv0 - nv1)) + s.v[24]);
        let eq13_e168_d_n1: f64 = ((s.dn[24][1] * (nv0 - nv1)) + (-s.v[24]));
        let eq13_e168_d_n2: f64 = (s.dn[24][2] * (nv0 - nv1));
        let eq13_e168_d_n3: f64 = (s.dn[24][3] * (nv0 - nv1));
        let eq13_e168_d_n4: f64 = (s.dn[24][4] * (nv0 - nv1));
        let eq13_e168_d_n5: f64 = (s.dn[24][5] * (nv0 - nv1));
        let eq13_e168_d_n6: f64 = (s.dn[24][6] * (nv0 - nv1));
        let eq13_e168_d_b0: f64 = (s.db[24][0] * (nv0 - nv1));
        let eq13_e168_d_b1: f64 = (s.db[24][1] * (nv0 - nv1));
        let eq13_e168_d_b2: f64 = (s.db[24][2] * (nv0 - nv1));
        let eq13_e168_d_b3: f64 = (s.db[24][3] * (nv0 - nv1));
        let eq13_e168_d_b4: f64 = (s.db[24][4] * (nv0 - nv1));
        let eq13_e168_d_b5: f64 = (s.db[24][5] * (nv0 - nv1));
        let eq13_e168_d_b6: f64 = (s.db[24][6] * (nv0 - nv1));
        let eq13_e169: f64 = (eq13_e168).abs();
        let eq13_e169_d_n0: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_n0 } else { (-eq13_e168_d_n0) };
        let eq13_e169_d_n1: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_n1 } else { (-eq13_e168_d_n1) };
        let eq13_e169_d_n2: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_n2 } else { (-eq13_e168_d_n2) };
        let eq13_e169_d_n3: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_n3 } else { (-eq13_e168_d_n3) };
        let eq13_e169_d_n4: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_n4 } else { (-eq13_e168_d_n4) };
        let eq13_e169_d_n5: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_n5 } else { (-eq13_e168_d_n5) };
        let eq13_e169_d_n6: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_n6 } else { (-eq13_e168_d_n6) };
        let eq13_e169_d_b0: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_b0 } else { (-eq13_e168_d_b0) };
        let eq13_e169_d_b1: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_b1 } else { (-eq13_e168_d_b1) };
        let eq13_e169_d_b2: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_b2 } else { (-eq13_e168_d_b2) };
        let eq13_e169_d_b3: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_b3 } else { (-eq13_e168_d_b3) };
        let eq13_e169_d_b4: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_b4 } else { (-eq13_e168_d_b4) };
        let eq13_e169_d_b5: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_b5 } else { (-eq13_e168_d_b5) };
        let eq13_e169_d_b6: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_b6 } else { (-eq13_e168_d_b6) };
        let eq13_e170: f64 = (eq13_e165 * eq13_e169);
        let eq13_e170_d_n0: f64 = (eq13_e165 * eq13_e169_d_n0);
        let eq13_e170_d_n1: f64 = (eq13_e165 * eq13_e169_d_n1);
        let eq13_e170_d_n2: f64 = (eq13_e165 * eq13_e169_d_n2);
        let eq13_e170_d_n3: f64 = (eq13_e165 * eq13_e169_d_n3);
        let eq13_e170_d_n4: f64 = (eq13_e165 * eq13_e169_d_n4);
        let eq13_e170_d_n5: f64 = (eq13_e165 * eq13_e169_d_n5);
        let eq13_e170_d_n6: f64 = (eq13_e165 * eq13_e169_d_n6);
        let eq13_e170_d_b0: f64 = (eq13_e165 * eq13_e169_d_b0);
        let eq13_e170_d_b1: f64 = (eq13_e165 * eq13_e169_d_b1);
        let eq13_e170_d_b2: f64 = (eq13_e165 * eq13_e169_d_b2);
        let eq13_e170_d_b3: f64 = (eq13_e165 * eq13_e169_d_b3);
        let eq13_e170_d_b4: f64 = (eq13_e165 * eq13_e169_d_b4);
        let eq13_e170_d_b5: f64 = (eq13_e165 * eq13_e169_d_b5);
        let eq13_e170_d_b6: f64 = (eq13_e165 * eq13_e169_d_b6);
        (eq13_e170, eq13_e170_d_n0, eq13_e170_d_n1, eq13_e170_d_n2, eq13_e170_d_n3, eq13_e170_d_n4, eq13_e170_d_n5, eq13_e170_d_n6, eq13_e170_d_b0, eq13_e170_d_b1, eq13_e170_d_b2, eq13_e170_d_b3, eq13_e170_d_b4, eq13_e170_d_b5, eq13_e170_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e172;
        let eq13_node_derivatives: [f64; 7] = [eq13_e172_d_n0, eq13_e172_d_n1, eq13_e172_d_n2, eq13_e172_d_n3, eq13_e172_d_n4, eq13_e172_d_n5, eq13_e172_d_n6];
        let eq13_branch_derivatives: [f64; 7] = [eq13_e172_d_b0, eq13_e172_d_b1, eq13_e172_d_b2, eq13_e172_d_b3, eq13_e172_d_b4, eq13_e172_d_b5, eq13_e172_d_b6];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            None,
            self.multiplicity * (eq13_value),
            &nodes,
            &eq13_node_derivatives,
            &branches,
            &eq13_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_14_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq14_e182,) = {
    if (((!(s.v[70] != 0.0)) && (!(s.v[71] != 0.0))) && (s.v[72] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq14_value: f64 = eq14_e182;
        stamper.stamp_potential(
            branches[2],
            eq14_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_15_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq15_e193,) = {
    if (((!(s.v[70] != 0.0)) && (!(s.v[71] != 0.0))) && (!(s.v[72] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq15_value: f64 = eq15_e193;
        stamper.stamp_potential(
            branches[3],
            eq15_value,
            &[
            ],
        );
    }
}
