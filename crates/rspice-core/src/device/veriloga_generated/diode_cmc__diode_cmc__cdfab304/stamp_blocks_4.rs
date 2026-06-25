#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let (eq10_e163,) = {
    if (!(s.v[958] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq10_value: f64 = eq10_e163;
        stamper.stamp_potential(
            branches[2],
            eq10_value,
            &[
            ],
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
        let (eq11_e172, eq11_e172_d_n0, eq11_e172_d_n1, eq11_e172_d_n2, eq11_e172_d_n3, eq11_e172_d_n4, eq11_e172_d_n5,) = {
    if (s.v[959] != 0.0) {
        let eq11_e168: f64 = self.eval_ddt(2, s.v[344]);
        let eq11_e168_d_n0: f64 = self.ddt_jacobian(s.dn[344][0]);
        let eq11_e168_d_n1: f64 = self.ddt_jacobian(s.dn[344][1]);
        let eq11_e168_d_n2: f64 = self.ddt_jacobian(s.dn[344][2]);
        let eq11_e168_d_n3: f64 = self.ddt_jacobian(s.dn[344][3]);
        let eq11_e168_d_n4: f64 = self.ddt_jacobian(s.dn[344][4]);
        let eq11_e168_d_n5: f64 = self.ddt_jacobian(s.dn[344][5]);
        let eq11_e169: f64 = (s.v[345] + eq11_e168);
        let eq11_e169_d_n0: f64 = (s.dn[345][0] + eq11_e168_d_n0);
        let eq11_e169_d_n1: f64 = (s.dn[345][1] + eq11_e168_d_n1);
        let eq11_e169_d_n2: f64 = (s.dn[345][2] + eq11_e168_d_n2);
        let eq11_e169_d_n3: f64 = (s.dn[345][3] + eq11_e168_d_n3);
        let eq11_e169_d_n4: f64 = (s.dn[345][4] + eq11_e168_d_n4);
        let eq11_e169_d_n5: f64 = (s.dn[345][5] + eq11_e168_d_n5);
        let eq11_e170: f64 = (1e-13 * eq11_e169);
        let eq11_e170_d_n0: f64 = (1e-13 * eq11_e169_d_n0);
        let eq11_e170_d_n1: f64 = (1e-13 * eq11_e169_d_n1);
        let eq11_e170_d_n2: f64 = (1e-13 * eq11_e169_d_n2);
        let eq11_e170_d_n3: f64 = (1e-13 * eq11_e169_d_n3);
        let eq11_e170_d_n4: f64 = (1e-13 * eq11_e169_d_n4);
        let eq11_e170_d_n5: f64 = (1e-13 * eq11_e169_d_n5);
        (eq11_e170, eq11_e170_d_n0, eq11_e170_d_n1, eq11_e170_d_n2, eq11_e170_d_n3, eq11_e170_d_n4, eq11_e170_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e172;
        let eq11_node_derivatives: [f64; 6] = [eq11_e172_d_n0, eq11_e172_d_n1, eq11_e172_d_n2, eq11_e172_d_n3, eq11_e172_d_n4, eq11_e172_d_n5];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            None,
            self.multiplicity * (eq11_value),
            &nodes,
            &eq11_node_derivatives,
            &branches,
            &eq11_branch_derivatives,
            self.multiplicity,
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
        let (eq12_e177,) = {
    if (!(s.v[959] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq12_value: f64 = eq12_e177;
        stamper.stamp_potential(
            branches[3],
            eq12_value,
            &[
            ],
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
        let eq13_e179: f64 = self.eval_ddt(3, s.v[275]);
        let eq13_e179_d_n0: f64 = self.ddt_jacobian(s.dn[275][0]);
        let eq13_e179_d_n1: f64 = self.ddt_jacobian(s.dn[275][1]);
        let eq13_e179_d_n2: f64 = self.ddt_jacobian(s.dn[275][2]);
        let eq13_e179_d_n3: f64 = self.ddt_jacobian(s.dn[275][3]);
        let eq13_e179_d_n4: f64 = self.ddt_jacobian(s.dn[275][4]);
        let eq13_e179_d_n5: f64 = self.ddt_jacobian(s.dn[275][5]);
        let eq13_value: f64 = eq13_e179;
        let eq13_node_derivatives: [f64; 6] = [eq13_e179_d_n0, eq13_e179_d_n1, eq13_e179_d_n2, eq13_e179_d_n3, eq13_e179_d_n4, eq13_e179_d_n5];
        let eq13_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[2]),
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
        let eq14_e183: f64 = (s.v[274] - s.v[290]);
        let eq14_e183_d_n0: f64 = (s.dn[274][0] - s.dn[290][0]);
        let eq14_e183_d_n1: f64 = (s.dn[274][1] - s.dn[290][1]);
        let eq14_e183_d_n2: f64 = (s.dn[274][2] - s.dn[290][2]);
        let eq14_e183_d_n3: f64 = (s.dn[274][3] - s.dn[290][3]);
        let eq14_e183_d_n4: f64 = (s.dn[274][4] - s.dn[290][4]);
        let eq14_e183_d_n5: f64 = (s.dn[274][5] - s.dn[290][5]);
        let eq14_e184: f64 = (s.v[55] * eq14_e183);
        let eq14_e184_d_n0: f64 = ((s.dn[55][0] * eq14_e183) + (s.v[55] * eq14_e183_d_n0));
        let eq14_e184_d_n1: f64 = ((s.dn[55][1] * eq14_e183) + (s.v[55] * eq14_e183_d_n1));
        let eq14_e184_d_n2: f64 = ((s.dn[55][2] * eq14_e183) + (s.v[55] * eq14_e183_d_n2));
        let eq14_e184_d_n3: f64 = ((s.dn[55][3] * eq14_e183) + (s.v[55] * eq14_e183_d_n3));
        let eq14_e184_d_n4: f64 = ((s.dn[55][4] * eq14_e183) + (s.v[55] * eq14_e183_d_n4));
        let eq14_e184_d_n5: f64 = ((s.dn[55][5] * eq14_e183) + (s.v[55] * eq14_e183_d_n5));
        let eq14_e185: f64 = self.eval_ddt(4, eq14_e184);
        let eq14_e185_d_n0: f64 = self.ddt_jacobian(eq14_e184_d_n0);
        let eq14_e185_d_n1: f64 = self.ddt_jacobian(eq14_e184_d_n1);
        let eq14_e185_d_n2: f64 = self.ddt_jacobian(eq14_e184_d_n2);
        let eq14_e185_d_n3: f64 = self.ddt_jacobian(eq14_e184_d_n3);
        let eq14_e185_d_n4: f64 = self.ddt_jacobian(eq14_e184_d_n4);
        let eq14_e185_d_n5: f64 = self.ddt_jacobian(eq14_e184_d_n5);
        let eq14_value: f64 = eq14_e185;
        let eq14_node_derivatives: [f64; 6] = [eq14_e185_d_n0, eq14_e185_d_n1, eq14_e185_d_n2, eq14_e185_d_n3, eq14_e185_d_n4, eq14_e185_d_n5];
        let eq14_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            self.multiplicity * (eq14_value),
            &nodes,
            &eq14_node_derivatives,
            &branches,
            &eq14_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_7_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq7_e144, eq7_e144_d_n0, eq7_e144_d_n1, eq7_e144_d_n2, eq7_e144_d_n3, eq7_e144_d_n4, eq7_e144_d_n5, eq7_e144_q, eq7_e144_q_d_n0, eq7_e144_q_d_n1, eq7_e144_q_d_n2, eq7_e144_q_d_n3, eq7_e144_q_d_n4, eq7_e144_q_d_n5,) = {
    if (s.v[958] != 0.0) {
        let eq7_e140_q: f64 = s.v[336];
        let eq7_e141: f64 = (s.v[338] + s.v[336]);
        let eq7_e141_d_n0: f64 = (s.dn[338][0] + s.dn[336][0]);
        let eq7_e141_d_n1: f64 = (s.dn[338][1] + s.dn[336][1]);
        let eq7_e141_d_n2: f64 = (s.dn[338][2] + s.dn[336][2]);
        let eq7_e141_d_n3: f64 = (s.dn[338][3] + s.dn[336][3]);
        let eq7_e141_d_n4: f64 = (s.dn[338][4] + s.dn[336][4]);
        let eq7_e141_d_n5: f64 = (s.dn[338][5] + s.dn[336][5]);
        let eq7_e141_q: f64 = eq7_e140_q;
        let eq7_e142: f64 = (1e-12 * eq7_e141);
        let eq7_e142_d_n0: f64 = (1e-12 * eq7_e141_d_n0);
        let eq7_e142_d_n1: f64 = (1e-12 * eq7_e141_d_n1);
        let eq7_e142_d_n2: f64 = (1e-12 * eq7_e141_d_n2);
        let eq7_e142_d_n3: f64 = (1e-12 * eq7_e141_d_n3);
        let eq7_e142_d_n4: f64 = (1e-12 * eq7_e141_d_n4);
        let eq7_e142_d_n5: f64 = (1e-12 * eq7_e141_d_n5);
        let eq7_e142_q: f64 = (1e-12 * eq7_e141_q);
        let eq7_e142_q_d_n0: f64 = (1e-12 * s.dn[336][0]);
        let eq7_e142_q_d_n1: f64 = (1e-12 * s.dn[336][1]);
        let eq7_e142_q_d_n2: f64 = (1e-12 * s.dn[336][2]);
        let eq7_e142_q_d_n3: f64 = (1e-12 * s.dn[336][3]);
        let eq7_e142_q_d_n4: f64 = (1e-12 * s.dn[336][4]);
        let eq7_e142_q_d_n5: f64 = (1e-12 * s.dn[336][5]);
        (eq7_e142, eq7_e142_d_n0, eq7_e142_d_n1, eq7_e142_d_n2, eq7_e142_d_n3, eq7_e142_d_n4, eq7_e142_d_n5, eq7_e142_q, eq7_e142_q_d_n0, eq7_e142_q_d_n1, eq7_e142_q_d_n2, eq7_e142_q_d_n3, eq7_e142_q_d_n4, eq7_e142_q_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_reactive_node_derivatives: [f64; 6] = [eq7_e144_q_d_n0, eq7_e144_q_d_n1, eq7_e144_q_d_n2, eq7_e144_q_d_n3, eq7_e144_q_d_n4, eq7_e144_q_d_n5];
        let eq7_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            None,
            &nodes,
            &eq7_reactive_node_derivatives,
            &branches,
            &eq7_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_8_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq8_e153, eq8_e153_d_n0, eq8_e153_d_n1, eq8_e153_d_n2, eq8_e153_d_n3, eq8_e153_d_n4, eq8_e153_d_n5, eq8_e153_q, eq8_e153_q_d_n0, eq8_e153_q_d_n1, eq8_e153_q_d_n2, eq8_e153_q_d_n3, eq8_e153_q_d_n4, eq8_e153_q_d_n5,) = {
    if (s.v[958] != 0.0) {
        let eq8_e149_q: f64 = s.v[337];
        let eq8_e150: f64 = (s.v[339] + s.v[337]);
        let eq8_e150_d_n0: f64 = (s.dn[339][0] + s.dn[337][0]);
        let eq8_e150_d_n1: f64 = (s.dn[339][1] + s.dn[337][1]);
        let eq8_e150_d_n2: f64 = (s.dn[339][2] + s.dn[337][2]);
        let eq8_e150_d_n3: f64 = (s.dn[339][3] + s.dn[337][3]);
        let eq8_e150_d_n4: f64 = (s.dn[339][4] + s.dn[337][4]);
        let eq8_e150_d_n5: f64 = (s.dn[339][5] + s.dn[337][5]);
        let eq8_e150_q: f64 = eq8_e149_q;
        let eq8_e151: f64 = (1e-12 * eq8_e150);
        let eq8_e151_d_n0: f64 = (1e-12 * eq8_e150_d_n0);
        let eq8_e151_d_n1: f64 = (1e-12 * eq8_e150_d_n1);
        let eq8_e151_d_n2: f64 = (1e-12 * eq8_e150_d_n2);
        let eq8_e151_d_n3: f64 = (1e-12 * eq8_e150_d_n3);
        let eq8_e151_d_n4: f64 = (1e-12 * eq8_e150_d_n4);
        let eq8_e151_d_n5: f64 = (1e-12 * eq8_e150_d_n5);
        let eq8_e151_q: f64 = (1e-12 * eq8_e150_q);
        let eq8_e151_q_d_n0: f64 = (1e-12 * s.dn[337][0]);
        let eq8_e151_q_d_n1: f64 = (1e-12 * s.dn[337][1]);
        let eq8_e151_q_d_n2: f64 = (1e-12 * s.dn[337][2]);
        let eq8_e151_q_d_n3: f64 = (1e-12 * s.dn[337][3]);
        let eq8_e151_q_d_n4: f64 = (1e-12 * s.dn[337][4]);
        let eq8_e151_q_d_n5: f64 = (1e-12 * s.dn[337][5]);
        (eq8_e151, eq8_e151_d_n0, eq8_e151_d_n1, eq8_e151_d_n2, eq8_e151_d_n3, eq8_e151_d_n4, eq8_e151_d_n5, eq8_e151_q, eq8_e151_q_d_n0, eq8_e151_q_d_n1, eq8_e151_q_d_n2, eq8_e151_q_d_n3, eq8_e151_q_d_n4, eq8_e151_q_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_reactive_node_derivatives: [f64; 6] = [eq8_e153_q_d_n0, eq8_e153_q_d_n1, eq8_e153_q_d_n2, eq8_e153_q_d_n3, eq8_e153_q_d_n4, eq8_e153_q_d_n5];
        let eq8_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            &nodes,
            &eq8_reactive_node_derivatives,
            &branches,
            &eq8_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_11_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq11_e172, eq11_e172_d_n0, eq11_e172_d_n1, eq11_e172_d_n2, eq11_e172_d_n3, eq11_e172_d_n4, eq11_e172_d_n5, eq11_e172_q, eq11_e172_q_d_n0, eq11_e172_q_d_n1, eq11_e172_q_d_n2, eq11_e172_q_d_n3, eq11_e172_q_d_n4, eq11_e172_q_d_n5,) = {
    if (s.v[959] != 0.0) {
        let eq11_e168_q: f64 = s.v[344];
        let eq11_e169: f64 = (s.v[345] + s.v[344]);
        let eq11_e169_d_n0: f64 = (s.dn[345][0] + s.dn[344][0]);
        let eq11_e169_d_n1: f64 = (s.dn[345][1] + s.dn[344][1]);
        let eq11_e169_d_n2: f64 = (s.dn[345][2] + s.dn[344][2]);
        let eq11_e169_d_n3: f64 = (s.dn[345][3] + s.dn[344][3]);
        let eq11_e169_d_n4: f64 = (s.dn[345][4] + s.dn[344][4]);
        let eq11_e169_d_n5: f64 = (s.dn[345][5] + s.dn[344][5]);
        let eq11_e169_q: f64 = eq11_e168_q;
        let eq11_e170: f64 = (1e-13 * eq11_e169);
        let eq11_e170_d_n0: f64 = (1e-13 * eq11_e169_d_n0);
        let eq11_e170_d_n1: f64 = (1e-13 * eq11_e169_d_n1);
        let eq11_e170_d_n2: f64 = (1e-13 * eq11_e169_d_n2);
        let eq11_e170_d_n3: f64 = (1e-13 * eq11_e169_d_n3);
        let eq11_e170_d_n4: f64 = (1e-13 * eq11_e169_d_n4);
        let eq11_e170_d_n5: f64 = (1e-13 * eq11_e169_d_n5);
        let eq11_e170_q: f64 = (1e-13 * eq11_e169_q);
        let eq11_e170_q_d_n0: f64 = (1e-13 * s.dn[344][0]);
        let eq11_e170_q_d_n1: f64 = (1e-13 * s.dn[344][1]);
        let eq11_e170_q_d_n2: f64 = (1e-13 * s.dn[344][2]);
        let eq11_e170_q_d_n3: f64 = (1e-13 * s.dn[344][3]);
        let eq11_e170_q_d_n4: f64 = (1e-13 * s.dn[344][4]);
        let eq11_e170_q_d_n5: f64 = (1e-13 * s.dn[344][5]);
        (eq11_e170, eq11_e170_d_n0, eq11_e170_d_n1, eq11_e170_d_n2, eq11_e170_d_n3, eq11_e170_d_n4, eq11_e170_d_n5, eq11_e170_q, eq11_e170_q_d_n0, eq11_e170_q_d_n1, eq11_e170_q_d_n2, eq11_e170_q_d_n3, eq11_e170_q_d_n4, eq11_e170_q_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_reactive_node_derivatives: [f64; 6] = [eq11_e172_q_d_n0, eq11_e172_q_d_n1, eq11_e172_q_d_n2, eq11_e172_q_d_n3, eq11_e172_q_d_n4, eq11_e172_q_d_n5];
        let eq11_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            &nodes,
            &eq11_reactive_node_derivatives,
            &branches,
            &eq11_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_13_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq13_e179_q: f64 = s.v[275];
        let eq13_reactive_node_derivatives: [f64; 6] = [s.dn[275][0], s.dn[275][1], s.dn[275][2], s.dn[275][3], s.dn[275][4], s.dn[275][5]];
        let eq13_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            &nodes,
            &eq13_reactive_node_derivatives,
            &branches,
            &eq13_reactive_branch_derivatives,
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
        let eq14_e183: f64 = (s.v[274] - s.v[290]);
        let eq14_e183_d_n0: f64 = (s.dn[274][0] - s.dn[290][0]);
        let eq14_e183_d_n1: f64 = (s.dn[274][1] - s.dn[290][1]);
        let eq14_e183_d_n2: f64 = (s.dn[274][2] - s.dn[290][2]);
        let eq14_e183_d_n3: f64 = (s.dn[274][3] - s.dn[290][3]);
        let eq14_e183_d_n4: f64 = (s.dn[274][4] - s.dn[290][4]);
        let eq14_e183_d_n5: f64 = (s.dn[274][5] - s.dn[290][5]);
        let eq14_e184: f64 = (s.v[55] * eq14_e183);
        let eq14_e184_d_n0: f64 = ((s.dn[55][0] * eq14_e183) + (s.v[55] * eq14_e183_d_n0));
        let eq14_e184_d_n1: f64 = ((s.dn[55][1] * eq14_e183) + (s.v[55] * eq14_e183_d_n1));
        let eq14_e184_d_n2: f64 = ((s.dn[55][2] * eq14_e183) + (s.v[55] * eq14_e183_d_n2));
        let eq14_e184_d_n3: f64 = ((s.dn[55][3] * eq14_e183) + (s.v[55] * eq14_e183_d_n3));
        let eq14_e184_d_n4: f64 = ((s.dn[55][4] * eq14_e183) + (s.v[55] * eq14_e183_d_n4));
        let eq14_e184_d_n5: f64 = ((s.dn[55][5] * eq14_e183) + (s.v[55] * eq14_e183_d_n5));
        let eq14_e185_q: f64 = eq14_e184;
        let eq14_reactive_node_derivatives: [f64; 6] = [eq14_e184_d_n0, eq14_e184_d_n1, eq14_e184_d_n2, eq14_e184_d_n3, eq14_e184_d_n4, eq14_e184_d_n5];
        let eq14_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            &nodes,
            &eq14_reactive_node_derivatives,
            &branches,
            &eq14_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
