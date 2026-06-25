#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let eq23_e213: f64 = self.eval_ddt(7, s.v[210]);
        let eq23_e213_d_n0: f64 = self.ddt_jacobian(s.dn[210][0]);
        let eq23_e213_d_n1: f64 = self.ddt_jacobian(s.dn[210][1]);
        let eq23_e213_d_n2: f64 = self.ddt_jacobian(s.dn[210][2]);
        let eq23_e213_d_n3: f64 = self.ddt_jacobian(s.dn[210][3]);
        let eq23_e213_d_n4: f64 = self.ddt_jacobian(s.dn[210][4]);
        let eq23_e213_d_n5: f64 = self.ddt_jacobian(s.dn[210][5]);
        let eq23_e213_d_n6: f64 = self.ddt_jacobian(s.dn[210][6]);
        let eq23_e213_d_n7: f64 = self.ddt_jacobian(s.dn[210][7]);
        let eq23_e213_d_n8: f64 = self.ddt_jacobian(s.dn[210][8]);
        let eq23_e213_d_n9: f64 = self.ddt_jacobian(s.dn[210][9]);
        let eq23_e213_d_b0: f64 = self.ddt_jacobian(s.db[210][0]);
        let eq23_e213_d_b1: f64 = self.ddt_jacobian(s.db[210][1]);
        let eq23_e213_d_b2: f64 = self.ddt_jacobian(s.db[210][2]);
        let eq23_e213_d_b3: f64 = self.ddt_jacobian(s.db[210][3]);
        let eq23_value: f64 = eq23_e213;
        let eq23_node_derivatives: [f64; 10] = [eq23_e213_d_n0, eq23_e213_d_n1, eq23_e213_d_n2, eq23_e213_d_n3, eq23_e213_d_n4, eq23_e213_d_n5, eq23_e213_d_n6, eq23_e213_d_n7, eq23_e213_d_n8, eq23_e213_d_n9];
        let eq23_branch_derivatives: [f64; 4] = [eq23_e213_d_b0, eq23_e213_d_b1, eq23_e213_d_b2, eq23_e213_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            None,
            self.multiplicity * (eq23_value),
            &nodes,
            &eq23_node_derivatives,
            &branches,
            &eq23_branch_derivatives,
            self.multiplicity,
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
        let eq24_value: f64 = s.v[212];
        let eq24_node_derivatives: [f64; 10] = [s.dn[212][0], s.dn[212][1], s.dn[212][2], s.dn[212][3], s.dn[212][4], s.dn[212][5], s.dn[212][6], s.dn[212][7], s.dn[212][8], s.dn[212][9]];
        let eq24_branch_derivatives: [f64; 4] = [s.db[212][0], s.db[212][1], s.db[212][2], s.db[212][3]];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            None,
            self.multiplicity * (eq24_value),
            &nodes,
            &eq24_node_derivatives,
            &branches,
            &eq24_branch_derivatives,
            self.multiplicity,
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
        let eq25_e216: f64 = self.eval_ddt(8, s.v[213]);
        let eq25_e216_d_n0: f64 = self.ddt_jacobian(s.dn[213][0]);
        let eq25_e216_d_n1: f64 = self.ddt_jacobian(s.dn[213][1]);
        let eq25_e216_d_n2: f64 = self.ddt_jacobian(s.dn[213][2]);
        let eq25_e216_d_n3: f64 = self.ddt_jacobian(s.dn[213][3]);
        let eq25_e216_d_n4: f64 = self.ddt_jacobian(s.dn[213][4]);
        let eq25_e216_d_n5: f64 = self.ddt_jacobian(s.dn[213][5]);
        let eq25_e216_d_n6: f64 = self.ddt_jacobian(s.dn[213][6]);
        let eq25_e216_d_n7: f64 = self.ddt_jacobian(s.dn[213][7]);
        let eq25_e216_d_n8: f64 = self.ddt_jacobian(s.dn[213][8]);
        let eq25_e216_d_n9: f64 = self.ddt_jacobian(s.dn[213][9]);
        let eq25_e216_d_b0: f64 = self.ddt_jacobian(s.db[213][0]);
        let eq25_e216_d_b1: f64 = self.ddt_jacobian(s.db[213][1]);
        let eq25_e216_d_b2: f64 = self.ddt_jacobian(s.db[213][2]);
        let eq25_e216_d_b3: f64 = self.ddt_jacobian(s.db[213][3]);
        let eq25_value: f64 = eq25_e216;
        let eq25_node_derivatives: [f64; 10] = [eq25_e216_d_n0, eq25_e216_d_n1, eq25_e216_d_n2, eq25_e216_d_n3, eq25_e216_d_n4, eq25_e216_d_n5, eq25_e216_d_n6, eq25_e216_d_n7, eq25_e216_d_n8, eq25_e216_d_n9];
        let eq25_branch_derivatives: [f64; 4] = [eq25_e216_d_b0, eq25_e216_d_b1, eq25_e216_d_b2, eq25_e216_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            None,
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
        let (eq26_e224,) = {
    if (s.v[364] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e224;
        stamper.stamp_current(
            Some(nodes[1]),
            Some(nodes[6]),
            self.multiplicity * (eq26_value),
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
        let (eq27_e232,) = {
    if (s.v[365] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq27_value: f64 = eq27_e232;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[0]),
            self.multiplicity * (eq27_value),
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
        let (eq28_e240,) = {
    if (s.v[366] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq28_value: f64 = eq28_e240;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[2]),
            self.multiplicity * (eq28_value),
            &[
            ],
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
        let eq29_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[6]),
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
            Some(nodes[6]),
            Some(nodes[7]),
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
            Some(nodes[7]),
            self.multiplicity * (eq31_value),
            &[
            ],
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
        let eq4_e146_q: f64 = s.v[162];
        let eq4_reactive_node_derivatives: [f64; 10] = [s.dn[162][0], s.dn[162][1], s.dn[162][2], s.dn[162][3], s.dn[162][4], s.dn[162][5], s.dn[162][6], s.dn[162][7], s.dn[162][8], s.dn[162][9]];
        let eq4_reactive_branch_derivatives: [f64; 4] = [s.db[162][0], s.db[162][1], s.db[162][2], s.db[162][3]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            &nodes,
            &eq4_reactive_node_derivatives,
            &branches,
            &eq4_reactive_branch_derivatives,
            self.multiplicity,
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
        let eq5_e148_q: f64 = s.v[105];
        let eq5_reactive_node_derivatives: [f64; 10] = [s.dn[105][0], s.dn[105][1], s.dn[105][2], s.dn[105][3], s.dn[105][4], s.dn[105][5], s.dn[105][6], s.dn[105][7], s.dn[105][8], s.dn[105][9]];
        let eq5_reactive_branch_derivatives: [f64; 4] = [s.db[105][0], s.db[105][1], s.db[105][2], s.db[105][3]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[5]),
            &nodes,
            &eq5_reactive_node_derivatives,
            &branches,
            &eq5_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_6_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq6_e150_q: f64 = s.v[196];
        let eq6_reactive_node_derivatives: [f64; 10] = [s.dn[196][0], s.dn[196][1], s.dn[196][2], s.dn[196][3], s.dn[196][4], s.dn[196][5], s.dn[196][6], s.dn[196][7], s.dn[196][8], s.dn[196][9]];
        let eq6_reactive_branch_derivatives: [f64; 4] = [s.db[196][0], s.db[196][1], s.db[196][2], s.db[196][3]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[5]),
            &nodes,
            &eq6_reactive_node_derivatives,
            &branches,
            &eq6_reactive_branch_derivatives,
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
        let eq7_e152_q: f64 = s.v[197];
        let eq7_reactive_node_derivatives: [f64; 10] = [s.dn[197][0], s.dn[197][1], s.dn[197][2], s.dn[197][3], s.dn[197][4], s.dn[197][5], s.dn[197][6], s.dn[197][7], s.dn[197][8], s.dn[197][9]];
        let eq7_reactive_branch_derivatives: [f64; 4] = [s.db[197][0], s.db[197][1], s.db[197][2], s.db[197][3]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            &nodes,
            &eq7_reactive_node_derivatives,
            &branches,
            &eq7_reactive_branch_derivatives,
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
        let eq15_e188_q: f64 = s.v[198];
        let eq15_reactive_node_derivatives: [f64; 10] = [s.dn[198][0], s.dn[198][1], s.dn[198][2], s.dn[198][3], s.dn[198][4], s.dn[198][5], s.dn[198][6], s.dn[198][7], s.dn[198][8], s.dn[198][9]];
        let eq15_reactive_branch_derivatives: [f64; 4] = [s.db[198][0], s.db[198][1], s.db[198][2], s.db[198][3]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[5]),
            &nodes,
            &eq15_reactive_node_derivatives,
            &branches,
            &eq15_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_17_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq17_e191_q: f64 = s.v[199];
        let eq17_reactive_node_derivatives: [f64; 10] = [s.dn[199][0], s.dn[199][1], s.dn[199][2], s.dn[199][3], s.dn[199][4], s.dn[199][5], s.dn[199][6], s.dn[199][7], s.dn[199][8], s.dn[199][9]];
        let eq17_reactive_branch_derivatives: [f64; 4] = [s.db[199][0], s.db[199][1], s.db[199][2], s.db[199][3]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            &nodes,
            &eq17_reactive_node_derivatives,
            &branches,
            &eq17_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_21_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq21_e210, eq21_e210_d_n0, eq21_e210_d_n1, eq21_e210_d_n2, eq21_e210_d_n3, eq21_e210_d_n4, eq21_e210_d_n5, eq21_e210_d_n6, eq21_e210_d_n7, eq21_e210_d_n8, eq21_e210_d_n9, eq21_e210_d_b0, eq21_e210_d_b1, eq21_e210_d_b2, eq21_e210_d_b3, eq21_e210_q, eq21_e210_q_d_n4,) = {
    if (!(s.v[360] != 0.0)) {
        let eq21_e208_q: f64 = s.rv[167];
        (s.v[167], s.dn[167][0], s.dn[167][1], s.dn[167][2], s.dn[167][3], s.dn[167][4], s.dn[167][5], s.dn[167][6], s.dn[167][7], s.dn[167][8], s.dn[167][9], s.db[167][0], s.db[167][1], s.db[167][2], s.db[167][3], eq21_e208_q, s.rdn[167][4],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[4]),
            None,
            &[
                GeneratedDerivative::node(nodes[4], self.multiplicity * (eq21_e210_q_d_n4)),
            ],
        );
    }
}
