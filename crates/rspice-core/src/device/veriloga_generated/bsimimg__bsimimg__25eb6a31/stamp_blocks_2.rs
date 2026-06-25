#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_17_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq17_e887: f64 = self.eval_ddt(5, s.v[238]);
        let eq17_e887_d_n0: f64 = self.ddt_jacobian(s.dn[238][0]);
        let eq17_e887_d_n1: f64 = self.ddt_jacobian(s.dn[238][1]);
        let eq17_e887_d_n2: f64 = self.ddt_jacobian(s.dn[238][2]);
        let eq17_e887_d_n3: f64 = self.ddt_jacobian(s.dn[238][3]);
        let eq17_e887_d_n4: f64 = self.ddt_jacobian(s.dn[238][4]);
        let eq17_e887_d_n5: f64 = self.ddt_jacobian(s.dn[238][5]);
        let eq17_e887_d_n6: f64 = self.ddt_jacobian(s.dn[238][6]);
        let eq17_e887_d_n7: f64 = self.ddt_jacobian(s.dn[238][7]);
        let eq17_e887_d_n8: f64 = self.ddt_jacobian(s.dn[238][8]);
        let eq17_e888: f64 = (s.v[212] * eq17_e887);
        let eq17_e888_d_n0: f64 = ((s.dn[212][0] * eq17_e887) + (s.v[212] * eq17_e887_d_n0));
        let eq17_e888_d_n1: f64 = ((s.dn[212][1] * eq17_e887) + (s.v[212] * eq17_e887_d_n1));
        let eq17_e888_d_n2: f64 = ((s.dn[212][2] * eq17_e887) + (s.v[212] * eq17_e887_d_n2));
        let eq17_e888_d_n3: f64 = ((s.dn[212][3] * eq17_e887) + (s.v[212] * eq17_e887_d_n3));
        let eq17_e888_d_n4: f64 = ((s.dn[212][4] * eq17_e887) + (s.v[212] * eq17_e887_d_n4));
        let eq17_e888_d_n5: f64 = ((s.dn[212][5] * eq17_e887) + (s.v[212] * eq17_e887_d_n5));
        let eq17_e888_d_n6: f64 = ((s.dn[212][6] * eq17_e887) + (s.v[212] * eq17_e887_d_n6));
        let eq17_e888_d_n7: f64 = ((s.dn[212][7] * eq17_e887) + (s.v[212] * eq17_e887_d_n7));
        let eq17_e888_d_n8: f64 = ((s.dn[212][8] * eq17_e887) + (s.v[212] * eq17_e887_d_n8));
        let eq17_value: f64 = eq17_e888;
        let eq17_node_derivatives: [f64; 9] = [eq17_e888_d_n0, eq17_e888_d_n1, eq17_e888_d_n2, eq17_e888_d_n3, eq17_e888_d_n4, eq17_e888_d_n5, eq17_e888_d_n6, eq17_e888_d_n7, eq17_e888_d_n8];
        let eq17_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[3]),
            self.multiplicity * (eq17_value),
            &nodes,
            &eq17_node_derivatives,
            &branches,
            &eq17_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_18_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq18_e891: f64 = self.eval_ddt(6, s.v[239]);
        let eq18_e891_d_n0: f64 = self.ddt_jacobian(s.dn[239][0]);
        let eq18_e891_d_n1: f64 = self.ddt_jacobian(s.dn[239][1]);
        let eq18_e891_d_n2: f64 = self.ddt_jacobian(s.dn[239][2]);
        let eq18_e891_d_n3: f64 = self.ddt_jacobian(s.dn[239][3]);
        let eq18_e891_d_n4: f64 = self.ddt_jacobian(s.dn[239][4]);
        let eq18_e891_d_n5: f64 = self.ddt_jacobian(s.dn[239][5]);
        let eq18_e891_d_n6: f64 = self.ddt_jacobian(s.dn[239][6]);
        let eq18_e891_d_n7: f64 = self.ddt_jacobian(s.dn[239][7]);
        let eq18_e891_d_n8: f64 = self.ddt_jacobian(s.dn[239][8]);
        let eq18_e892: f64 = (s.v[212] * eq18_e891);
        let eq18_e892_d_n0: f64 = ((s.dn[212][0] * eq18_e891) + (s.v[212] * eq18_e891_d_n0));
        let eq18_e892_d_n1: f64 = ((s.dn[212][1] * eq18_e891) + (s.v[212] * eq18_e891_d_n1));
        let eq18_e892_d_n2: f64 = ((s.dn[212][2] * eq18_e891) + (s.v[212] * eq18_e891_d_n2));
        let eq18_e892_d_n3: f64 = ((s.dn[212][3] * eq18_e891) + (s.v[212] * eq18_e891_d_n3));
        let eq18_e892_d_n4: f64 = ((s.dn[212][4] * eq18_e891) + (s.v[212] * eq18_e891_d_n4));
        let eq18_e892_d_n5: f64 = ((s.dn[212][5] * eq18_e891) + (s.v[212] * eq18_e891_d_n5));
        let eq18_e892_d_n6: f64 = ((s.dn[212][6] * eq18_e891) + (s.v[212] * eq18_e891_d_n6));
        let eq18_e892_d_n7: f64 = ((s.dn[212][7] * eq18_e891) + (s.v[212] * eq18_e891_d_n7));
        let eq18_e892_d_n8: f64 = ((s.dn[212][8] * eq18_e891) + (s.v[212] * eq18_e891_d_n8));
        let eq18_value: f64 = eq18_e892;
        let eq18_node_derivatives: [f64; 9] = [eq18_e892_d_n0, eq18_e892_d_n1, eq18_e892_d_n2, eq18_e892_d_n3, eq18_e892_d_n4, eq18_e892_d_n5, eq18_e892_d_n6, eq18_e892_d_n7, eq18_e892_d_n8];
        let eq18_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[3]),
            self.multiplicity * (eq18_value),
            &nodes,
            &eq18_node_derivatives,
            &branches,
            &eq18_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_19_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq19_e896,) = {
    if (s.v[663] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e896;
        stamper.stamp_potential(
            branches[0],
            eq19_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_20_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq20_e900,) = {
    if (s.v[663] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e900;
        stamper.stamp_potential(
            branches[1],
            eq20_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_21_block_0(
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq21_e907, eq21_e907_d_n0, eq21_e907_d_n1, eq21_e907_d_n2, eq21_e907_d_n3, eq21_e907_d_n4, eq21_e907_d_n5, eq21_e907_d_n6, eq21_e907_d_n7, eq21_e907_d_n8,) = {
    if (!(s.v[663] != 0.0)) {
        let eq21_e905: f64 = ((nv0 - nv5) * s.v[149]);
        let eq21_e905_d_n0: f64 = (s.v[149] + ((nv0 - nv5) * s.dn[149][0]));
        let eq21_e905_d_n1: f64 = ((nv0 - nv5) * s.dn[149][1]);
        let eq21_e905_d_n2: f64 = ((nv0 - nv5) * s.dn[149][2]);
        let eq21_e905_d_n3: f64 = ((nv0 - nv5) * s.dn[149][3]);
        let eq21_e905_d_n4: f64 = ((nv0 - nv5) * s.dn[149][4]);
        let eq21_e905_d_n5: f64 = ((-s.v[149]) + ((nv0 - nv5) * s.dn[149][5]));
        let eq21_e905_d_n6: f64 = ((nv0 - nv5) * s.dn[149][6]);
        let eq21_e905_d_n7: f64 = ((nv0 - nv5) * s.dn[149][7]);
        let eq21_e905_d_n8: f64 = ((nv0 - nv5) * s.dn[149][8]);
        (eq21_e905, eq21_e905_d_n0, eq21_e905_d_n1, eq21_e905_d_n2, eq21_e905_d_n3, eq21_e905_d_n4, eq21_e905_d_n5, eq21_e905_d_n6, eq21_e905_d_n7, eq21_e905_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e907;
        let eq21_node_derivatives: [f64; 9] = [eq21_e907_d_n0, eq21_e907_d_n1, eq21_e907_d_n2, eq21_e907_d_n3, eq21_e907_d_n4, eq21_e907_d_n5, eq21_e907_d_n6, eq21_e907_d_n7, eq21_e907_d_n8];
        let eq21_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[5]),
            self.multiplicity * (eq21_value),
            &nodes,
            &eq21_node_derivatives,
            &branches,
            &eq21_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_22_block_0(
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
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq22_e914, eq22_e914_d_n0, eq22_e914_d_n1, eq22_e914_d_n2, eq22_e914_d_n3, eq22_e914_d_n4, eq22_e914_d_n5, eq22_e914_d_n6, eq22_e914_d_n7, eq22_e914_d_n8,) = {
    if (!(s.v[663] != 0.0)) {
        let eq22_e912: f64 = ((nv2 - nv6) * s.v[148]);
        let eq22_e912_d_n0: f64 = ((nv2 - nv6) * s.dn[148][0]);
        let eq22_e912_d_n1: f64 = ((nv2 - nv6) * s.dn[148][1]);
        let eq22_e912_d_n2: f64 = (s.v[148] + ((nv2 - nv6) * s.dn[148][2]));
        let eq22_e912_d_n3: f64 = ((nv2 - nv6) * s.dn[148][3]);
        let eq22_e912_d_n4: f64 = ((nv2 - nv6) * s.dn[148][4]);
        let eq22_e912_d_n5: f64 = ((nv2 - nv6) * s.dn[148][5]);
        let eq22_e912_d_n6: f64 = ((-s.v[148]) + ((nv2 - nv6) * s.dn[148][6]));
        let eq22_e912_d_n7: f64 = ((nv2 - nv6) * s.dn[148][7]);
        let eq22_e912_d_n8: f64 = ((nv2 - nv6) * s.dn[148][8]);
        (eq22_e912, eq22_e912_d_n0, eq22_e912_d_n1, eq22_e912_d_n2, eq22_e912_d_n3, eq22_e912_d_n4, eq22_e912_d_n5, eq22_e912_d_n6, eq22_e912_d_n7, eq22_e912_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq22_value: f64 = eq22_e914;
        let eq22_node_derivatives: [f64; 9] = [eq22_e914_d_n0, eq22_e914_d_n1, eq22_e914_d_n2, eq22_e914_d_n3, eq22_e914_d_n4, eq22_e914_d_n5, eq22_e914_d_n6, eq22_e914_d_n7, eq22_e914_d_n8];
        let eq22_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[6]),
            self.multiplicity * (eq22_value),
            &nodes,
            &eq22_node_derivatives,
            &branches,
            &eq22_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_23_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq23_e923,) = {
    if (!(s.v[663] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e923;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[5]),
            self.multiplicity * (eq23_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_24_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq24_e932,) = {
    if (!(s.v[663] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq24_value: f64 = eq24_e932;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[6]),
            self.multiplicity * (eq24_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_25_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq25_e938, eq25_e938_d_n0, eq25_e938_d_n1, eq25_e938_d_n2, eq25_e938_d_n3, eq25_e938_d_n4, eq25_e938_d_n5, eq25_e938_d_n6, eq25_e938_d_n7, eq25_e938_d_n8,) = {
    if (s.v[664] != 0.0) {
        let eq25_e936: f64 = ((nv7 - nv8) * s.v[274]);
        let eq25_e936_d_n0: f64 = ((nv7 - nv8) * s.dn[274][0]);
        let eq25_e936_d_n1: f64 = ((nv7 - nv8) * s.dn[274][1]);
        let eq25_e936_d_n2: f64 = ((nv7 - nv8) * s.dn[274][2]);
        let eq25_e936_d_n3: f64 = ((nv7 - nv8) * s.dn[274][3]);
        let eq25_e936_d_n4: f64 = ((nv7 - nv8) * s.dn[274][4]);
        let eq25_e936_d_n5: f64 = ((nv7 - nv8) * s.dn[274][5]);
        let eq25_e936_d_n6: f64 = ((nv7 - nv8) * s.dn[274][6]);
        let eq25_e936_d_n7: f64 = (s.v[274] + ((nv7 - nv8) * s.dn[274][7]));
        let eq25_e936_d_n8: f64 = ((-s.v[274]) + ((nv7 - nv8) * s.dn[274][8]));
        (eq25_e936, eq25_e936_d_n0, eq25_e936_d_n1, eq25_e936_d_n2, eq25_e936_d_n3, eq25_e936_d_n4, eq25_e936_d_n5, eq25_e936_d_n6, eq25_e936_d_n7, eq25_e936_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e938;
        let eq25_node_derivatives: [f64; 9] = [eq25_e938_d_n0, eq25_e938_d_n1, eq25_e938_d_n2, eq25_e938_d_n3, eq25_e938_d_n4, eq25_e938_d_n5, eq25_e938_d_n6, eq25_e938_d_n7, eq25_e938_d_n8];
        let eq25_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[8]),
            self.multiplicity * (eq25_value),
            &nodes,
            &eq25_node_derivatives,
            &branches,
            &eq25_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_26_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq26_e943,) = {
    if (!(s.v[664] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e943;
        stamper.stamp_potential(
            branches[2],
            eq26_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_27_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq27_e947,) = {
    if (s.v[665] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq27_value: f64 = eq27_e947;
        stamper.stamp_potential(
            branches[3],
            eq27_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_28_block_0(
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq28_e954, eq28_e954_d_n0, eq28_e954_d_n1, eq28_e954_d_n2, eq28_e954_d_n3, eq28_e954_d_n4, eq28_e954_d_n5, eq28_e954_d_n6, eq28_e954_d_n7, eq28_e954_d_n8,) = {
    if (!(s.v[665] != 0.0)) {
        let eq28_e952: f64 = ((nv1 - nv7) * s.v[666]);
        let eq28_e952_d_n0: f64 = ((nv1 - nv7) * s.dn[666][0]);
        let eq28_e952_d_n1: f64 = (s.v[666] + ((nv1 - nv7) * s.dn[666][1]));
        let eq28_e952_d_n2: f64 = ((nv1 - nv7) * s.dn[666][2]);
        let eq28_e952_d_n3: f64 = ((nv1 - nv7) * s.dn[666][3]);
        let eq28_e952_d_n4: f64 = ((nv1 - nv7) * s.dn[666][4]);
        let eq28_e952_d_n5: f64 = ((nv1 - nv7) * s.dn[666][5]);
        let eq28_e952_d_n6: f64 = ((nv1 - nv7) * s.dn[666][6]);
        let eq28_e952_d_n7: f64 = ((-s.v[666]) + ((nv1 - nv7) * s.dn[666][7]));
        let eq28_e952_d_n8: f64 = ((nv1 - nv7) * s.dn[666][8]);
        (eq28_e952, eq28_e952_d_n0, eq28_e952_d_n1, eq28_e952_d_n2, eq28_e952_d_n3, eq28_e952_d_n4, eq28_e952_d_n5, eq28_e952_d_n6, eq28_e952_d_n7, eq28_e952_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e954;
        let eq28_node_derivatives: [f64; 9] = [eq28_e954_d_n0, eq28_e954_d_n1, eq28_e954_d_n2, eq28_e954_d_n3, eq28_e954_d_n4, eq28_e954_d_n5, eq28_e954_d_n6, eq28_e954_d_n7, eq28_e954_d_n8];
        let eq28_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[7]),
            self.multiplicity * (eq28_value),
            &nodes,
            &eq28_node_derivatives,
            &branches,
            &eq28_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_29_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq29_e963,) = {
    if (!(s.v[665] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e963;
        stamper.stamp_current(
            Some(nodes[1]),
            Some(nodes[7]),
            self.multiplicity * (eq29_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_30_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq30_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq30_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_31_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq31_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq31_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_32_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq32_e985,) = {
    if ((s.v[668] != 0.0) && (s.v[669] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e985;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[6]),
            self.multiplicity * (eq32_value),
            &[
            ],
        );
    }
}
