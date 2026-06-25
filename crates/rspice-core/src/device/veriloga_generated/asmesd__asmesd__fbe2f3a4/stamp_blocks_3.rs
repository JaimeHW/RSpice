#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_44_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq44_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq44_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_45_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq45_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq45_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_46_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq46_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[4]),
            Some(nodes[6]),
            self.multiplicity * (eq46_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_2_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv9 = ctx.node_voltage(nodes[9]);
        let eq2_e98_q: f64 = (nv9 - 0.0);
        let eq2_e99: f64 = (p.p83 * (nv9 - 0.0));
        let eq2_e99_d_n9: f64 = p.p83;
        let eq2_e99_q: f64 = (p.p83 * eq2_e98_q);
        let eq2_e99_q_d_n9: f64 = p.p83;
        stamper.stamp_current_reactive(
            Some(nodes[9]),
            None,
            &[
                GeneratedDerivative::node(nodes[9], self.multiplicity * (eq2_e99_q_d_n9)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_5_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq5_e121, eq5_e121_d_n0, eq5_e121_d_n1, eq5_e121_d_n2, eq5_e121_d_n3, eq5_e121_d_n4, eq5_e121_d_n5, eq5_e121_d_n6, eq5_e121_d_n7, eq5_e121_d_n8, eq5_e121_d_n9, eq5_e121_d_b0, eq5_e121_d_b1, eq5_e121_d_b2, eq5_e121_d_b3, eq5_e121_d_b4, eq5_e121_d_b5, eq5_e121_d_b6, eq5_e121_d_b7, eq5_e121_q, eq5_e121_q_d_n0, eq5_e121_q_d_n1, eq5_e121_q_d_n2, eq5_e121_q_d_n3, eq5_e121_q_d_n4, eq5_e121_q_d_n5, eq5_e121_q_d_n6, eq5_e121_q_d_n7, eq5_e121_q_d_n8, eq5_e121_q_d_n9, eq5_e121_q_d_b0, eq5_e121_q_d_b1, eq5_e121_q_d_b2, eq5_e121_q_d_b3, eq5_e121_q_d_b4, eq5_e121_q_d_b5, eq5_e121_q_d_b6, eq5_e121_q_d_b7,) = {
    if (s.v[115] != 0.0) {
        let eq5_e118_q: f64 = (nv8 - 0.0);
        let eq5_e119: f64 = (s.v[54] * (nv8 - 0.0));
        let eq5_e119_d_n0: f64 = (s.dn[54][0] * (nv8 - 0.0));
        let eq5_e119_d_n1: f64 = (s.dn[54][1] * (nv8 - 0.0));
        let eq5_e119_d_n2: f64 = (s.dn[54][2] * (nv8 - 0.0));
        let eq5_e119_d_n3: f64 = (s.dn[54][3] * (nv8 - 0.0));
        let eq5_e119_d_n4: f64 = (s.dn[54][4] * (nv8 - 0.0));
        let eq5_e119_d_n5: f64 = (s.dn[54][5] * (nv8 - 0.0));
        let eq5_e119_d_n6: f64 = (s.dn[54][6] * (nv8 - 0.0));
        let eq5_e119_d_n7: f64 = (s.dn[54][7] * (nv8 - 0.0));
        let eq5_e119_d_n8: f64 = ((s.dn[54][8] * (nv8 - 0.0)) + s.v[54]);
        let eq5_e119_d_n9: f64 = (s.dn[54][9] * (nv8 - 0.0));
        let eq5_e119_d_b0: f64 = (s.db[54][0] * (nv8 - 0.0));
        let eq5_e119_d_b1: f64 = (s.db[54][1] * (nv8 - 0.0));
        let eq5_e119_d_b2: f64 = (s.db[54][2] * (nv8 - 0.0));
        let eq5_e119_d_b3: f64 = (s.db[54][3] * (nv8 - 0.0));
        let eq5_e119_d_b4: f64 = (s.db[54][4] * (nv8 - 0.0));
        let eq5_e119_d_b5: f64 = (s.db[54][5] * (nv8 - 0.0));
        let eq5_e119_d_b6: f64 = (s.db[54][6] * (nv8 - 0.0));
        let eq5_e119_d_b7: f64 = (s.db[54][7] * (nv8 - 0.0));
        let eq5_e119_q: f64 = (s.v[54] * eq5_e118_q);
        let eq5_e119_q_d_n0: f64 = (s.dn[54][0] * eq5_e118_q);
        let eq5_e119_q_d_n1: f64 = (s.dn[54][1] * eq5_e118_q);
        let eq5_e119_q_d_n2: f64 = (s.dn[54][2] * eq5_e118_q);
        let eq5_e119_q_d_n3: f64 = (s.dn[54][3] * eq5_e118_q);
        let eq5_e119_q_d_n4: f64 = (s.dn[54][4] * eq5_e118_q);
        let eq5_e119_q_d_n5: f64 = (s.dn[54][5] * eq5_e118_q);
        let eq5_e119_q_d_n6: f64 = (s.dn[54][6] * eq5_e118_q);
        let eq5_e119_q_d_n7: f64 = (s.dn[54][7] * eq5_e118_q);
        let eq5_e119_q_d_n8: f64 = ((s.dn[54][8] * eq5_e118_q) + s.v[54]);
        let eq5_e119_q_d_n9: f64 = (s.dn[54][9] * eq5_e118_q);
        let eq5_e119_q_d_b0: f64 = (s.db[54][0] * eq5_e118_q);
        let eq5_e119_q_d_b1: f64 = (s.db[54][1] * eq5_e118_q);
        let eq5_e119_q_d_b2: f64 = (s.db[54][2] * eq5_e118_q);
        let eq5_e119_q_d_b3: f64 = (s.db[54][3] * eq5_e118_q);
        let eq5_e119_q_d_b4: f64 = (s.db[54][4] * eq5_e118_q);
        let eq5_e119_q_d_b5: f64 = (s.db[54][5] * eq5_e118_q);
        let eq5_e119_q_d_b6: f64 = (s.db[54][6] * eq5_e118_q);
        let eq5_e119_q_d_b7: f64 = (s.db[54][7] * eq5_e118_q);
        (eq5_e119, eq5_e119_d_n0, eq5_e119_d_n1, eq5_e119_d_n2, eq5_e119_d_n3, eq5_e119_d_n4, eq5_e119_d_n5, eq5_e119_d_n6, eq5_e119_d_n7, eq5_e119_d_n8, eq5_e119_d_n9, eq5_e119_d_b0, eq5_e119_d_b1, eq5_e119_d_b2, eq5_e119_d_b3, eq5_e119_d_b4, eq5_e119_d_b5, eq5_e119_d_b6, eq5_e119_d_b7, eq5_e119_q, eq5_e119_q_d_n0, eq5_e119_q_d_n1, eq5_e119_q_d_n2, eq5_e119_q_d_n3, eq5_e119_q_d_n4, eq5_e119_q_d_n5, eq5_e119_q_d_n6, eq5_e119_q_d_n7, eq5_e119_q_d_n8, eq5_e119_q_d_n9, eq5_e119_q_d_b0, eq5_e119_q_d_b1, eq5_e119_q_d_b2, eq5_e119_q_d_b3, eq5_e119_q_d_b4, eq5_e119_q_d_b5, eq5_e119_q_d_b6, eq5_e119_q_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_reactive_node_derivatives: [f64; 10] = [eq5_e121_q_d_n0, eq5_e121_q_d_n1, eq5_e121_q_d_n2, eq5_e121_q_d_n3, eq5_e121_q_d_n4, eq5_e121_q_d_n5, eq5_e121_q_d_n6, eq5_e121_q_d_n7, eq5_e121_q_d_n8, eq5_e121_q_d_n9];
        let eq5_reactive_branch_derivatives: [f64; 8] = [eq5_e121_q_d_b0, eq5_e121_q_d_b1, eq5_e121_q_d_b2, eq5_e121_q_d_b3, eq5_e121_q_d_b4, eq5_e121_q_d_b5, eq5_e121_q_d_b6, eq5_e121_q_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            None,
            &nodes,
            &eq5_reactive_node_derivatives,
            &branches,
            &eq5_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_9_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv3 = ctx.node_voltage(nodes[3]);
        let (eq9_e154, eq9_e154_d_n3, eq9_e154_q, eq9_e154_q_d_n3,) = {
    if (s.v[122] != 0.0) {
        let eq9_e151: f64 = ((nv3 - 0.0) * p.p34);
        let eq9_e151_d_n3: f64 = p.p34;
        let eq9_e152_q: f64 = eq9_e151;
        (eq9_e151, eq9_e151_d_n3, eq9_e152_q, eq9_e151_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[3]),
            None,
            &[
                GeneratedDerivative::node(nodes[3], self.multiplicity * (eq9_e154_q_d_n3)),
            ],
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let (eq13_e195, eq13_e195_d_n3, eq13_e195_q, eq13_e195_q_d_n3,) = {
    if ((!(s.v[122] != 0.0)) && (s.v[123] != 0.0)) {
        let eq13_e192: f64 = (p.p34 * (nv3 - 0.0));
        let eq13_e192_d_n3: f64 = p.p34;
        let eq13_e193_q: f64 = eq13_e192;
        (eq13_e192, eq13_e192_d_n3, eq13_e193_q, eq13_e192_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[3]),
            None,
            &[
                GeneratedDerivative::node(nodes[3], self.multiplicity * (eq13_e195_q_d_n3)),
            ],
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq15_e214, eq15_e214_d_n7, eq15_e214_q, eq15_e214_q_d_n7,) = {
    if ((!(s.v[122] != 0.0)) && (s.v[123] != 0.0)) {
        let eq15_e211: f64 = (p.p36 * (nv7 - 0.0));
        let eq15_e211_d_n7: f64 = p.p36;
        let eq15_e212_q: f64 = eq15_e211;
        (eq15_e211, eq15_e211_d_n7, eq15_e212_q, eq15_e211_d_n7,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[7]),
            None,
            &[
                GeneratedDerivative::node(nodes[7], self.multiplicity * (eq15_e214_q_d_n7)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_36_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq36_e384: f64 = (s.v[9] * s.v[58]);
        let eq36_e384_d_n0: f64 = (s.v[9] * s.dn[58][0]);
        let eq36_e384_d_n1: f64 = (s.v[9] * s.dn[58][1]);
        let eq36_e384_d_n2: f64 = (s.v[9] * s.dn[58][2]);
        let eq36_e384_d_n3: f64 = (s.v[9] * s.dn[58][3]);
        let eq36_e384_d_n4: f64 = (s.v[9] * s.dn[58][4]);
        let eq36_e384_d_n5: f64 = (s.v[9] * s.dn[58][5]);
        let eq36_e384_d_n6: f64 = (s.v[9] * s.dn[58][6]);
        let eq36_e384_d_n7: f64 = (s.v[9] * s.dn[58][7]);
        let eq36_e384_d_n8: f64 = (s.v[9] * s.dn[58][8]);
        let eq36_e384_d_n9: f64 = (s.v[9] * s.dn[58][9]);
        let eq36_e384_d_b0: f64 = (s.v[9] * s.db[58][0]);
        let eq36_e384_d_b1: f64 = (s.v[9] * s.db[58][1]);
        let eq36_e384_d_b2: f64 = (s.v[9] * s.db[58][2]);
        let eq36_e384_d_b3: f64 = (s.v[9] * s.db[58][3]);
        let eq36_e384_d_b4: f64 = (s.v[9] * s.db[58][4]);
        let eq36_e384_d_b5: f64 = (s.v[9] * s.db[58][5]);
        let eq36_e384_d_b6: f64 = (s.v[9] * s.db[58][6]);
        let eq36_e384_d_b7: f64 = (s.v[9] * s.db[58][7]);
        let eq36_e386: f64 = (eq36_e384 * s.v[3]);
        let eq36_e386_d_n0: f64 = (eq36_e384_d_n0 * s.v[3]);
        let eq36_e386_d_n1: f64 = (eq36_e384_d_n1 * s.v[3]);
        let eq36_e386_d_n2: f64 = (eq36_e384_d_n2 * s.v[3]);
        let eq36_e386_d_n3: f64 = (eq36_e384_d_n3 * s.v[3]);
        let eq36_e386_d_n4: f64 = (eq36_e384_d_n4 * s.v[3]);
        let eq36_e386_d_n5: f64 = (eq36_e384_d_n5 * s.v[3]);
        let eq36_e386_d_n6: f64 = (eq36_e384_d_n6 * s.v[3]);
        let eq36_e386_d_n7: f64 = (eq36_e384_d_n7 * s.v[3]);
        let eq36_e386_d_n8: f64 = (eq36_e384_d_n8 * s.v[3]);
        let eq36_e386_d_n9: f64 = (eq36_e384_d_n9 * s.v[3]);
        let eq36_e386_d_b0: f64 = (eq36_e384_d_b0 * s.v[3]);
        let eq36_e386_d_b1: f64 = (eq36_e384_d_b1 * s.v[3]);
        let eq36_e386_d_b2: f64 = (eq36_e384_d_b2 * s.v[3]);
        let eq36_e386_d_b3: f64 = (eq36_e384_d_b3 * s.v[3]);
        let eq36_e386_d_b4: f64 = (eq36_e384_d_b4 * s.v[3]);
        let eq36_e386_d_b5: f64 = (eq36_e384_d_b5 * s.v[3]);
        let eq36_e386_d_b6: f64 = (eq36_e384_d_b6 * s.v[3]);
        let eq36_e386_d_b7: f64 = (eq36_e384_d_b7 * s.v[3]);
        let eq36_e387_q: f64 = eq36_e386;
        let eq36_reactive_node_derivatives: [f64; 10] = [eq36_e386_d_n0, eq36_e386_d_n1, eq36_e386_d_n2, eq36_e386_d_n3, eq36_e386_d_n4, eq36_e386_d_n5, eq36_e386_d_n6, eq36_e386_d_n7, eq36_e386_d_n8, eq36_e386_d_n9];
        let eq36_reactive_branch_derivatives: [f64; 8] = [eq36_e386_d_b0, eq36_e386_d_b1, eq36_e386_d_b2, eq36_e386_d_b3, eq36_e386_d_b4, eq36_e386_d_b5, eq36_e386_d_b6, eq36_e386_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            &nodes,
            &eq36_reactive_node_derivatives,
            &branches,
            &eq36_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_37_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq37_e390: f64 = (s.v[9] * s.v[55]);
        let eq37_e390_d_n0: f64 = (s.v[9] * s.dn[55][0]);
        let eq37_e390_d_n1: f64 = (s.v[9] * s.dn[55][1]);
        let eq37_e390_d_n2: f64 = (s.v[9] * s.dn[55][2]);
        let eq37_e390_d_n3: f64 = (s.v[9] * s.dn[55][3]);
        let eq37_e390_d_n4: f64 = (s.v[9] * s.dn[55][4]);
        let eq37_e390_d_n5: f64 = (s.v[9] * s.dn[55][5]);
        let eq37_e390_d_n6: f64 = (s.v[9] * s.dn[55][6]);
        let eq37_e390_d_n7: f64 = (s.v[9] * s.dn[55][7]);
        let eq37_e390_d_n8: f64 = (s.v[9] * s.dn[55][8]);
        let eq37_e390_d_n9: f64 = (s.v[9] * s.dn[55][9]);
        let eq37_e390_d_b0: f64 = (s.v[9] * s.db[55][0]);
        let eq37_e390_d_b1: f64 = (s.v[9] * s.db[55][1]);
        let eq37_e390_d_b2: f64 = (s.v[9] * s.db[55][2]);
        let eq37_e390_d_b3: f64 = (s.v[9] * s.db[55][3]);
        let eq37_e390_d_b4: f64 = (s.v[9] * s.db[55][4]);
        let eq37_e390_d_b5: f64 = (s.v[9] * s.db[55][5]);
        let eq37_e390_d_b6: f64 = (s.v[9] * s.db[55][6]);
        let eq37_e390_d_b7: f64 = (s.v[9] * s.db[55][7]);
        let eq37_e392: f64 = (eq37_e390 * s.v[3]);
        let eq37_e392_d_n0: f64 = (eq37_e390_d_n0 * s.v[3]);
        let eq37_e392_d_n1: f64 = (eq37_e390_d_n1 * s.v[3]);
        let eq37_e392_d_n2: f64 = (eq37_e390_d_n2 * s.v[3]);
        let eq37_e392_d_n3: f64 = (eq37_e390_d_n3 * s.v[3]);
        let eq37_e392_d_n4: f64 = (eq37_e390_d_n4 * s.v[3]);
        let eq37_e392_d_n5: f64 = (eq37_e390_d_n5 * s.v[3]);
        let eq37_e392_d_n6: f64 = (eq37_e390_d_n6 * s.v[3]);
        let eq37_e392_d_n7: f64 = (eq37_e390_d_n7 * s.v[3]);
        let eq37_e392_d_n8: f64 = (eq37_e390_d_n8 * s.v[3]);
        let eq37_e392_d_n9: f64 = (eq37_e390_d_n9 * s.v[3]);
        let eq37_e392_d_b0: f64 = (eq37_e390_d_b0 * s.v[3]);
        let eq37_e392_d_b1: f64 = (eq37_e390_d_b1 * s.v[3]);
        let eq37_e392_d_b2: f64 = (eq37_e390_d_b2 * s.v[3]);
        let eq37_e392_d_b3: f64 = (eq37_e390_d_b3 * s.v[3]);
        let eq37_e392_d_b4: f64 = (eq37_e390_d_b4 * s.v[3]);
        let eq37_e392_d_b5: f64 = (eq37_e390_d_b5 * s.v[3]);
        let eq37_e392_d_b6: f64 = (eq37_e390_d_b6 * s.v[3]);
        let eq37_e392_d_b7: f64 = (eq37_e390_d_b7 * s.v[3]);
        let eq37_e393_q: f64 = eq37_e392;
        let eq37_reactive_node_derivatives: [f64; 10] = [eq37_e392_d_n0, eq37_e392_d_n1, eq37_e392_d_n2, eq37_e392_d_n3, eq37_e392_d_n4, eq37_e392_d_n5, eq37_e392_d_n6, eq37_e392_d_n7, eq37_e392_d_n8, eq37_e392_d_n9];
        let eq37_reactive_branch_derivatives: [f64; 8] = [eq37_e392_d_b0, eq37_e392_d_b1, eq37_e392_d_b2, eq37_e392_d_b3, eq37_e392_d_b4, eq37_e392_d_b5, eq37_e392_d_b6, eq37_e392_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            &nodes,
            &eq37_reactive_node_derivatives,
            &branches,
            &eq37_reactive_branch_derivatives,
            self.multiplicity,
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
        let eq38_e396: f64 = (s.v[9] * s.v[60]);
        let eq38_e396_d_n0: f64 = (s.v[9] * s.dn[60][0]);
        let eq38_e396_d_n1: f64 = (s.v[9] * s.dn[60][1]);
        let eq38_e396_d_n2: f64 = (s.v[9] * s.dn[60][2]);
        let eq38_e396_d_n3: f64 = (s.v[9] * s.dn[60][3]);
        let eq38_e396_d_n4: f64 = (s.v[9] * s.dn[60][4]);
        let eq38_e396_d_n5: f64 = (s.v[9] * s.dn[60][5]);
        let eq38_e396_d_n6: f64 = (s.v[9] * s.dn[60][6]);
        let eq38_e396_d_n7: f64 = (s.v[9] * s.dn[60][7]);
        let eq38_e396_d_n8: f64 = (s.v[9] * s.dn[60][8]);
        let eq38_e396_d_n9: f64 = (s.v[9] * s.dn[60][9]);
        let eq38_e396_d_b0: f64 = (s.v[9] * s.db[60][0]);
        let eq38_e396_d_b1: f64 = (s.v[9] * s.db[60][1]);
        let eq38_e396_d_b2: f64 = (s.v[9] * s.db[60][2]);
        let eq38_e396_d_b3: f64 = (s.v[9] * s.db[60][3]);
        let eq38_e396_d_b4: f64 = (s.v[9] * s.db[60][4]);
        let eq38_e396_d_b5: f64 = (s.v[9] * s.db[60][5]);
        let eq38_e396_d_b6: f64 = (s.v[9] * s.db[60][6]);
        let eq38_e396_d_b7: f64 = (s.v[9] * s.db[60][7]);
        let eq38_e398: f64 = (eq38_e396 * s.v[3]);
        let eq38_e398_d_n0: f64 = (eq38_e396_d_n0 * s.v[3]);
        let eq38_e398_d_n1: f64 = (eq38_e396_d_n1 * s.v[3]);
        let eq38_e398_d_n2: f64 = (eq38_e396_d_n2 * s.v[3]);
        let eq38_e398_d_n3: f64 = (eq38_e396_d_n3 * s.v[3]);
        let eq38_e398_d_n4: f64 = (eq38_e396_d_n4 * s.v[3]);
        let eq38_e398_d_n5: f64 = (eq38_e396_d_n5 * s.v[3]);
        let eq38_e398_d_n6: f64 = (eq38_e396_d_n6 * s.v[3]);
        let eq38_e398_d_n7: f64 = (eq38_e396_d_n7 * s.v[3]);
        let eq38_e398_d_n8: f64 = (eq38_e396_d_n8 * s.v[3]);
        let eq38_e398_d_n9: f64 = (eq38_e396_d_n9 * s.v[3]);
        let eq38_e398_d_b0: f64 = (eq38_e396_d_b0 * s.v[3]);
        let eq38_e398_d_b1: f64 = (eq38_e396_d_b1 * s.v[3]);
        let eq38_e398_d_b2: f64 = (eq38_e396_d_b2 * s.v[3]);
        let eq38_e398_d_b3: f64 = (eq38_e396_d_b3 * s.v[3]);
        let eq38_e398_d_b4: f64 = (eq38_e396_d_b4 * s.v[3]);
        let eq38_e398_d_b5: f64 = (eq38_e396_d_b5 * s.v[3]);
        let eq38_e398_d_b6: f64 = (eq38_e396_d_b6 * s.v[3]);
        let eq38_e398_d_b7: f64 = (eq38_e396_d_b7 * s.v[3]);
        let eq38_e399_q: f64 = eq38_e398;
        let eq38_reactive_node_derivatives: [f64; 10] = [eq38_e398_d_n0, eq38_e398_d_n1, eq38_e398_d_n2, eq38_e398_d_n3, eq38_e398_d_n4, eq38_e398_d_n5, eq38_e398_d_n6, eq38_e398_d_n7, eq38_e398_d_n8, eq38_e398_d_n9];
        let eq38_reactive_branch_derivatives: [f64; 8] = [eq38_e398_d_b0, eq38_e398_d_b1, eq38_e398_d_b2, eq38_e398_d_b3, eq38_e398_d_b4, eq38_e398_d_b5, eq38_e398_d_b6, eq38_e398_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[4]),
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
        let eq39_e402: f64 = (s.v[9] * s.v[62]);
        let eq39_e402_d_n0: f64 = (s.v[9] * s.dn[62][0]);
        let eq39_e402_d_n1: f64 = (s.v[9] * s.dn[62][1]);
        let eq39_e402_d_n2: f64 = (s.v[9] * s.dn[62][2]);
        let eq39_e402_d_n3: f64 = (s.v[9] * s.dn[62][3]);
        let eq39_e402_d_n4: f64 = (s.v[9] * s.dn[62][4]);
        let eq39_e402_d_n5: f64 = (s.v[9] * s.dn[62][5]);
        let eq39_e402_d_n6: f64 = (s.v[9] * s.dn[62][6]);
        let eq39_e402_d_n7: f64 = (s.v[9] * s.dn[62][7]);
        let eq39_e402_d_n8: f64 = (s.v[9] * s.dn[62][8]);
        let eq39_e402_d_n9: f64 = (s.v[9] * s.dn[62][9]);
        let eq39_e402_d_b0: f64 = (s.v[9] * s.db[62][0]);
        let eq39_e402_d_b1: f64 = (s.v[9] * s.db[62][1]);
        let eq39_e402_d_b2: f64 = (s.v[9] * s.db[62][2]);
        let eq39_e402_d_b3: f64 = (s.v[9] * s.db[62][3]);
        let eq39_e402_d_b4: f64 = (s.v[9] * s.db[62][4]);
        let eq39_e402_d_b5: f64 = (s.v[9] * s.db[62][5]);
        let eq39_e402_d_b6: f64 = (s.v[9] * s.db[62][6]);
        let eq39_e402_d_b7: f64 = (s.v[9] * s.db[62][7]);
        let eq39_e404: f64 = (eq39_e402 * s.v[3]);
        let eq39_e404_d_n0: f64 = (eq39_e402_d_n0 * s.v[3]);
        let eq39_e404_d_n1: f64 = (eq39_e402_d_n1 * s.v[3]);
        let eq39_e404_d_n2: f64 = (eq39_e402_d_n2 * s.v[3]);
        let eq39_e404_d_n3: f64 = (eq39_e402_d_n3 * s.v[3]);
        let eq39_e404_d_n4: f64 = (eq39_e402_d_n4 * s.v[3]);
        let eq39_e404_d_n5: f64 = (eq39_e402_d_n5 * s.v[3]);
        let eq39_e404_d_n6: f64 = (eq39_e402_d_n6 * s.v[3]);
        let eq39_e404_d_n7: f64 = (eq39_e402_d_n7 * s.v[3]);
        let eq39_e404_d_n8: f64 = (eq39_e402_d_n8 * s.v[3]);
        let eq39_e404_d_n9: f64 = (eq39_e402_d_n9 * s.v[3]);
        let eq39_e404_d_b0: f64 = (eq39_e402_d_b0 * s.v[3]);
        let eq39_e404_d_b1: f64 = (eq39_e402_d_b1 * s.v[3]);
        let eq39_e404_d_b2: f64 = (eq39_e402_d_b2 * s.v[3]);
        let eq39_e404_d_b3: f64 = (eq39_e402_d_b3 * s.v[3]);
        let eq39_e404_d_b4: f64 = (eq39_e402_d_b4 * s.v[3]);
        let eq39_e404_d_b5: f64 = (eq39_e402_d_b5 * s.v[3]);
        let eq39_e404_d_b6: f64 = (eq39_e402_d_b6 * s.v[3]);
        let eq39_e404_d_b7: f64 = (eq39_e402_d_b7 * s.v[3]);
        let eq39_e405_q: f64 = eq39_e404;
        let eq39_reactive_node_derivatives: [f64; 10] = [eq39_e404_d_n0, eq39_e404_d_n1, eq39_e404_d_n2, eq39_e404_d_n3, eq39_e404_d_n4, eq39_e404_d_n5, eq39_e404_d_n6, eq39_e404_d_n7, eq39_e404_d_n8, eq39_e404_d_n9];
        let eq39_reactive_branch_derivatives: [f64; 8] = [eq39_e404_d_b0, eq39_e404_d_b1, eq39_e404_d_b2, eq39_e404_d_b3, eq39_e404_d_b4, eq39_e404_d_b5, eq39_e404_d_b6, eq39_e404_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
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
        let eq40_e408: f64 = (s.v[9] * s.v[56]);
        let eq40_e408_d_n0: f64 = (s.v[9] * s.dn[56][0]);
        let eq40_e408_d_n1: f64 = (s.v[9] * s.dn[56][1]);
        let eq40_e408_d_n2: f64 = (s.v[9] * s.dn[56][2]);
        let eq40_e408_d_n3: f64 = (s.v[9] * s.dn[56][3]);
        let eq40_e408_d_n4: f64 = (s.v[9] * s.dn[56][4]);
        let eq40_e408_d_n5: f64 = (s.v[9] * s.dn[56][5]);
        let eq40_e408_d_n6: f64 = (s.v[9] * s.dn[56][6]);
        let eq40_e408_d_n7: f64 = (s.v[9] * s.dn[56][7]);
        let eq40_e408_d_n8: f64 = (s.v[9] * s.dn[56][8]);
        let eq40_e408_d_n9: f64 = (s.v[9] * s.dn[56][9]);
        let eq40_e408_d_b0: f64 = (s.v[9] * s.db[56][0]);
        let eq40_e408_d_b1: f64 = (s.v[9] * s.db[56][1]);
        let eq40_e408_d_b2: f64 = (s.v[9] * s.db[56][2]);
        let eq40_e408_d_b3: f64 = (s.v[9] * s.db[56][3]);
        let eq40_e408_d_b4: f64 = (s.v[9] * s.db[56][4]);
        let eq40_e408_d_b5: f64 = (s.v[9] * s.db[56][5]);
        let eq40_e408_d_b6: f64 = (s.v[9] * s.db[56][6]);
        let eq40_e408_d_b7: f64 = (s.v[9] * s.db[56][7]);
        let eq40_e410: f64 = (eq40_e408 * s.v[3]);
        let eq40_e410_d_n0: f64 = (eq40_e408_d_n0 * s.v[3]);
        let eq40_e410_d_n1: f64 = (eq40_e408_d_n1 * s.v[3]);
        let eq40_e410_d_n2: f64 = (eq40_e408_d_n2 * s.v[3]);
        let eq40_e410_d_n3: f64 = (eq40_e408_d_n3 * s.v[3]);
        let eq40_e410_d_n4: f64 = (eq40_e408_d_n4 * s.v[3]);
        let eq40_e410_d_n5: f64 = (eq40_e408_d_n5 * s.v[3]);
        let eq40_e410_d_n6: f64 = (eq40_e408_d_n6 * s.v[3]);
        let eq40_e410_d_n7: f64 = (eq40_e408_d_n7 * s.v[3]);
        let eq40_e410_d_n8: f64 = (eq40_e408_d_n8 * s.v[3]);
        let eq40_e410_d_n9: f64 = (eq40_e408_d_n9 * s.v[3]);
        let eq40_e410_d_b0: f64 = (eq40_e408_d_b0 * s.v[3]);
        let eq40_e410_d_b1: f64 = (eq40_e408_d_b1 * s.v[3]);
        let eq40_e410_d_b2: f64 = (eq40_e408_d_b2 * s.v[3]);
        let eq40_e410_d_b3: f64 = (eq40_e408_d_b3 * s.v[3]);
        let eq40_e410_d_b4: f64 = (eq40_e408_d_b4 * s.v[3]);
        let eq40_e410_d_b5: f64 = (eq40_e408_d_b5 * s.v[3]);
        let eq40_e410_d_b6: f64 = (eq40_e408_d_b6 * s.v[3]);
        let eq40_e410_d_b7: f64 = (eq40_e408_d_b7 * s.v[3]);
        let eq40_e411_q: f64 = eq40_e410;
        let eq40_reactive_node_derivatives: [f64; 10] = [eq40_e410_d_n0, eq40_e410_d_n1, eq40_e410_d_n2, eq40_e410_d_n3, eq40_e410_d_n4, eq40_e410_d_n5, eq40_e410_d_n6, eq40_e410_d_n7, eq40_e410_d_n8, eq40_e410_d_n9];
        let eq40_reactive_branch_derivatives: [f64; 8] = [eq40_e410_d_b0, eq40_e410_d_b1, eq40_e410_d_b2, eq40_e410_d_b3, eq40_e410_d_b4, eq40_e410_d_b5, eq40_e410_d_b6, eq40_e410_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
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
        let eq41_e414: f64 = (s.v[9] * s.v[57]);
        let eq41_e414_d_n0: f64 = (s.v[9] * s.dn[57][0]);
        let eq41_e414_d_n1: f64 = (s.v[9] * s.dn[57][1]);
        let eq41_e414_d_n2: f64 = (s.v[9] * s.dn[57][2]);
        let eq41_e414_d_n3: f64 = (s.v[9] * s.dn[57][3]);
        let eq41_e414_d_n4: f64 = (s.v[9] * s.dn[57][4]);
        let eq41_e414_d_n5: f64 = (s.v[9] * s.dn[57][5]);
        let eq41_e414_d_n6: f64 = (s.v[9] * s.dn[57][6]);
        let eq41_e414_d_n7: f64 = (s.v[9] * s.dn[57][7]);
        let eq41_e414_d_n8: f64 = (s.v[9] * s.dn[57][8]);
        let eq41_e414_d_n9: f64 = (s.v[9] * s.dn[57][9]);
        let eq41_e414_d_b0: f64 = (s.v[9] * s.db[57][0]);
        let eq41_e414_d_b1: f64 = (s.v[9] * s.db[57][1]);
        let eq41_e414_d_b2: f64 = (s.v[9] * s.db[57][2]);
        let eq41_e414_d_b3: f64 = (s.v[9] * s.db[57][3]);
        let eq41_e414_d_b4: f64 = (s.v[9] * s.db[57][4]);
        let eq41_e414_d_b5: f64 = (s.v[9] * s.db[57][5]);
        let eq41_e414_d_b6: f64 = (s.v[9] * s.db[57][6]);
        let eq41_e414_d_b7: f64 = (s.v[9] * s.db[57][7]);
        let eq41_e416: f64 = (eq41_e414 * s.v[3]);
        let eq41_e416_d_n0: f64 = (eq41_e414_d_n0 * s.v[3]);
        let eq41_e416_d_n1: f64 = (eq41_e414_d_n1 * s.v[3]);
        let eq41_e416_d_n2: f64 = (eq41_e414_d_n2 * s.v[3]);
        let eq41_e416_d_n3: f64 = (eq41_e414_d_n3 * s.v[3]);
        let eq41_e416_d_n4: f64 = (eq41_e414_d_n4 * s.v[3]);
        let eq41_e416_d_n5: f64 = (eq41_e414_d_n5 * s.v[3]);
        let eq41_e416_d_n6: f64 = (eq41_e414_d_n6 * s.v[3]);
        let eq41_e416_d_n7: f64 = (eq41_e414_d_n7 * s.v[3]);
        let eq41_e416_d_n8: f64 = (eq41_e414_d_n8 * s.v[3]);
        let eq41_e416_d_n9: f64 = (eq41_e414_d_n9 * s.v[3]);
        let eq41_e416_d_b0: f64 = (eq41_e414_d_b0 * s.v[3]);
        let eq41_e416_d_b1: f64 = (eq41_e414_d_b1 * s.v[3]);
        let eq41_e416_d_b2: f64 = (eq41_e414_d_b2 * s.v[3]);
        let eq41_e416_d_b3: f64 = (eq41_e414_d_b3 * s.v[3]);
        let eq41_e416_d_b4: f64 = (eq41_e414_d_b4 * s.v[3]);
        let eq41_e416_d_b5: f64 = (eq41_e414_d_b5 * s.v[3]);
        let eq41_e416_d_b6: f64 = (eq41_e414_d_b6 * s.v[3]);
        let eq41_e416_d_b7: f64 = (eq41_e414_d_b7 * s.v[3]);
        let eq41_e417_q: f64 = eq41_e416;
        let eq41_reactive_node_derivatives: [f64; 10] = [eq41_e416_d_n0, eq41_e416_d_n1, eq41_e416_d_n2, eq41_e416_d_n3, eq41_e416_d_n4, eq41_e416_d_n5, eq41_e416_d_n6, eq41_e416_d_n7, eq41_e416_d_n8, eq41_e416_d_n9];
        let eq41_reactive_branch_derivatives: [f64; 8] = [eq41_e416_d_b0, eq41_e416_d_b1, eq41_e416_d_b2, eq41_e416_d_b3, eq41_e416_d_b4, eq41_e416_d_b5, eq41_e416_d_b6, eq41_e416_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[4]),
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
        let eq42_e419: f64 = (-s.v[63]);
        let eq42_e419_d_n0: f64 = (-s.dn[63][0]);
        let eq42_e419_d_n1: f64 = (-s.dn[63][1]);
        let eq42_e419_d_n2: f64 = (-s.dn[63][2]);
        let eq42_e419_d_n3: f64 = (-s.dn[63][3]);
        let eq42_e419_d_n4: f64 = (-s.dn[63][4]);
        let eq42_e419_d_n5: f64 = (-s.dn[63][5]);
        let eq42_e419_d_n6: f64 = (-s.dn[63][6]);
        let eq42_e419_d_n7: f64 = (-s.dn[63][7]);
        let eq42_e419_d_n8: f64 = (-s.dn[63][8]);
        let eq42_e419_d_n9: f64 = (-s.dn[63][9]);
        let eq42_e419_d_b0: f64 = (-s.db[63][0]);
        let eq42_e419_d_b1: f64 = (-s.db[63][1]);
        let eq42_e419_d_b2: f64 = (-s.db[63][2]);
        let eq42_e419_d_b3: f64 = (-s.db[63][3]);
        let eq42_e419_d_b4: f64 = (-s.db[63][4]);
        let eq42_e419_d_b5: f64 = (-s.db[63][5]);
        let eq42_e419_d_b6: f64 = (-s.db[63][6]);
        let eq42_e419_d_b7: f64 = (-s.db[63][7]);
        let eq42_e421: f64 = (eq42_e419 * s.v[3]);
        let eq42_e421_d_n0: f64 = (eq42_e419_d_n0 * s.v[3]);
        let eq42_e421_d_n1: f64 = (eq42_e419_d_n1 * s.v[3]);
        let eq42_e421_d_n2: f64 = (eq42_e419_d_n2 * s.v[3]);
        let eq42_e421_d_n3: f64 = (eq42_e419_d_n3 * s.v[3]);
        let eq42_e421_d_n4: f64 = (eq42_e419_d_n4 * s.v[3]);
        let eq42_e421_d_n5: f64 = (eq42_e419_d_n5 * s.v[3]);
        let eq42_e421_d_n6: f64 = (eq42_e419_d_n6 * s.v[3]);
        let eq42_e421_d_n7: f64 = (eq42_e419_d_n7 * s.v[3]);
        let eq42_e421_d_n8: f64 = (eq42_e419_d_n8 * s.v[3]);
        let eq42_e421_d_n9: f64 = (eq42_e419_d_n9 * s.v[3]);
        let eq42_e421_d_b0: f64 = (eq42_e419_d_b0 * s.v[3]);
        let eq42_e421_d_b1: f64 = (eq42_e419_d_b1 * s.v[3]);
        let eq42_e421_d_b2: f64 = (eq42_e419_d_b2 * s.v[3]);
        let eq42_e421_d_b3: f64 = (eq42_e419_d_b3 * s.v[3]);
        let eq42_e421_d_b4: f64 = (eq42_e419_d_b4 * s.v[3]);
        let eq42_e421_d_b5: f64 = (eq42_e419_d_b5 * s.v[3]);
        let eq42_e421_d_b6: f64 = (eq42_e419_d_b6 * s.v[3]);
        let eq42_e421_d_b7: f64 = (eq42_e419_d_b7 * s.v[3]);
        let eq42_e422_q: f64 = eq42_e421;
        let eq42_reactive_node_derivatives: [f64; 10] = [eq42_e421_d_n0, eq42_e421_d_n1, eq42_e421_d_n2, eq42_e421_d_n3, eq42_e421_d_n4, eq42_e421_d_n5, eq42_e421_d_n6, eq42_e421_d_n7, eq42_e421_d_n8, eq42_e421_d_n9];
        let eq42_reactive_branch_derivatives: [f64; 8] = [eq42_e421_d_b0, eq42_e421_d_b1, eq42_e421_d_b2, eq42_e421_d_b3, eq42_e421_d_b4, eq42_e421_d_b5, eq42_e421_d_b6, eq42_e421_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
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
        let eq43_e425: f64 = (s.v[63] * s.v[3]);
        let eq43_e425_d_n0: f64 = (s.dn[63][0] * s.v[3]);
        let eq43_e425_d_n1: f64 = (s.dn[63][1] * s.v[3]);
        let eq43_e425_d_n2: f64 = (s.dn[63][2] * s.v[3]);
        let eq43_e425_d_n3: f64 = (s.dn[63][3] * s.v[3]);
        let eq43_e425_d_n4: f64 = (s.dn[63][4] * s.v[3]);
        let eq43_e425_d_n5: f64 = (s.dn[63][5] * s.v[3]);
        let eq43_e425_d_n6: f64 = (s.dn[63][6] * s.v[3]);
        let eq43_e425_d_n7: f64 = (s.dn[63][7] * s.v[3]);
        let eq43_e425_d_n8: f64 = (s.dn[63][8] * s.v[3]);
        let eq43_e425_d_n9: f64 = (s.dn[63][9] * s.v[3]);
        let eq43_e425_d_b0: f64 = (s.db[63][0] * s.v[3]);
        let eq43_e425_d_b1: f64 = (s.db[63][1] * s.v[3]);
        let eq43_e425_d_b2: f64 = (s.db[63][2] * s.v[3]);
        let eq43_e425_d_b3: f64 = (s.db[63][3] * s.v[3]);
        let eq43_e425_d_b4: f64 = (s.db[63][4] * s.v[3]);
        let eq43_e425_d_b5: f64 = (s.db[63][5] * s.v[3]);
        let eq43_e425_d_b6: f64 = (s.db[63][6] * s.v[3]);
        let eq43_e425_d_b7: f64 = (s.db[63][7] * s.v[3]);
        let eq43_e426_q: f64 = eq43_e425;
        let eq43_reactive_node_derivatives: [f64; 10] = [eq43_e425_d_n0, eq43_e425_d_n1, eq43_e425_d_n2, eq43_e425_d_n3, eq43_e425_d_n4, eq43_e425_d_n5, eq43_e425_d_n6, eq43_e425_d_n7, eq43_e425_d_n8, eq43_e425_d_n9];
        let eq43_reactive_branch_derivatives: [f64; 8] = [eq43_e425_d_b0, eq43_e425_d_b1, eq43_e425_d_b2, eq43_e425_d_b3, eq43_e425_d_b4, eq43_e425_d_b5, eq43_e425_d_b6, eq43_e425_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            &nodes,
            &eq43_reactive_node_derivatives,
            &branches,
            &eq43_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
