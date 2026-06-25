#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let (eq24_e346,) = {
    if (!(s.v[554] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq24_value: f64 = eq24_e346;
        stamper.stamp_potential(
            branches[1],
            eq24_value,
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
        let eq25_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[10]),
            None,
            self.multiplicity * (eq25_value),
            &[
            ],
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq26_value: f64 = (nv10 - 0.0);
        stamper.stamp_current(
            Some(nodes[10]),
            None,
            self.multiplicity * (eq26_value),
            &[
                GeneratedDerivative::node(nodes[10], self.multiplicity * 1.0),
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq27_e355: f64 = self.eval_ddt(8, (nv10 - 0.0));
        let eq27_e355_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq27_e355_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq27_e355_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq27_e355_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq27_e355_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq27_e355_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq27_e355_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq27_e355_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq27_e355_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq27_e355_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq27_e355_d_n10: f64 = self.ddt_jacobian(1.0);
        let eq27_e356: f64 = (s.v[306] * eq27_e355);
        let eq27_e356_d_n0: f64 = ((s.dn[306][0] * eq27_e355) + (s.v[306] * eq27_e355_d_n0));
        let eq27_e356_d_n1: f64 = ((s.dn[306][1] * eq27_e355) + (s.v[306] * eq27_e355_d_n1));
        let eq27_e356_d_n2: f64 = ((s.dn[306][2] * eq27_e355) + (s.v[306] * eq27_e355_d_n2));
        let eq27_e356_d_n3: f64 = ((s.dn[306][3] * eq27_e355) + (s.v[306] * eq27_e355_d_n3));
        let eq27_e356_d_n4: f64 = ((s.dn[306][4] * eq27_e355) + (s.v[306] * eq27_e355_d_n4));
        let eq27_e356_d_n5: f64 = ((s.dn[306][5] * eq27_e355) + (s.v[306] * eq27_e355_d_n5));
        let eq27_e356_d_n6: f64 = ((s.dn[306][6] * eq27_e355) + (s.v[306] * eq27_e355_d_n6));
        let eq27_e356_d_n7: f64 = ((s.dn[306][7] * eq27_e355) + (s.v[306] * eq27_e355_d_n7));
        let eq27_e356_d_n8: f64 = ((s.dn[306][8] * eq27_e355) + (s.v[306] * eq27_e355_d_n8));
        let eq27_e356_d_n9: f64 = ((s.dn[306][9] * eq27_e355) + (s.v[306] * eq27_e355_d_n9));
        let eq27_e356_d_n10: f64 = ((s.dn[306][10] * eq27_e355) + (s.v[306] * eq27_e355_d_n10));
        let eq27_value: f64 = eq27_e356;
        let eq27_node_derivatives: [f64; 11] = [eq27_e356_d_n0, eq27_e356_d_n1, eq27_e356_d_n2, eq27_e356_d_n3, eq27_e356_d_n4, eq27_e356_d_n5, eq27_e356_d_n6, eq27_e356_d_n7, eq27_e356_d_n8, eq27_e356_d_n9, eq27_e356_d_n10];
        let eq27_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[3]),
            self.multiplicity * (eq27_value),
            &nodes,
            &eq27_node_derivatives,
            &branches,
            &eq27_branch_derivatives,
            self.multiplicity,
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq28_e359: f64 = (s.v[304] * (nv10 - 0.0));
        let eq28_e359_d_n0: f64 = (s.dn[304][0] * (nv10 - 0.0));
        let eq28_e359_d_n1: f64 = (s.dn[304][1] * (nv10 - 0.0));
        let eq28_e359_d_n2: f64 = (s.dn[304][2] * (nv10 - 0.0));
        let eq28_e359_d_n3: f64 = (s.dn[304][3] * (nv10 - 0.0));
        let eq28_e359_d_n4: f64 = (s.dn[304][4] * (nv10 - 0.0));
        let eq28_e359_d_n5: f64 = (s.dn[304][5] * (nv10 - 0.0));
        let eq28_e359_d_n6: f64 = (s.dn[304][6] * (nv10 - 0.0));
        let eq28_e359_d_n7: f64 = (s.dn[304][7] * (nv10 - 0.0));
        let eq28_e359_d_n8: f64 = (s.dn[304][8] * (nv10 - 0.0));
        let eq28_e359_d_n9: f64 = (s.dn[304][9] * (nv10 - 0.0));
        let eq28_e359_d_n10: f64 = ((s.dn[304][10] * (nv10 - 0.0)) + s.v[304]);
        let eq28_value: f64 = eq28_e359;
        let eq28_node_derivatives: [f64; 11] = [eq28_e359_d_n0, eq28_e359_d_n1, eq28_e359_d_n2, eq28_e359_d_n3, eq28_e359_d_n4, eq28_e359_d_n5, eq28_e359_d_n6, eq28_e359_d_n7, eq28_e359_d_n8, eq28_e359_d_n9, eq28_e359_d_n10];
        let eq28_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[5]),
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq29_value: f64 = (nv10 - 0.0);
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[3]),
            self.multiplicity * (eq29_value),
            &[
                GeneratedDerivative::node(nodes[10], self.multiplicity * 1.0),
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
            Some(nodes[7]),
            Some(nodes[5]),
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
            Some(nodes[3]),
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
        let eq32_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[3]),
            self.multiplicity * (eq32_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_33_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq33_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[1]),
            Some(nodes[4]),
            self.multiplicity * (eq33_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_34_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq34_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[4]),
            Some(nodes[5]),
            self.multiplicity * (eq34_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_35_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq35_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[3]),
            self.multiplicity * (eq35_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_36_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq36_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[4]),
            Some(nodes[3]),
            self.multiplicity * (eq36_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_37_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq37_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[4]),
            Some(nodes[3]),
            self.multiplicity * (eq37_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_38_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq38_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[4]),
            Some(nodes[9]),
            self.multiplicity * (eq38_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_39_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq39_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[4]),
            Some(nodes[9]),
            self.multiplicity * (eq39_value),
            &[
            ],
        );
    }
}
