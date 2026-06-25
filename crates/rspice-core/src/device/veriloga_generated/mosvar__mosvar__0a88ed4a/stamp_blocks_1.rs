#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let (eq6_e106,) = {
    if (!(p.p16 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq6_value: f64 = eq6_e106;
        stamper.stamp_potential(
            branches[1],
            eq6_value,
            &[
            ],
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
        let (eq7_e111,) = {
    if (!(p.p16 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq7_value: f64 = eq7_e111;
        stamper.stamp_potential(
            branches[2],
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
        let (eq8_e116,) = {
    if (!(p.p16 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq8_value: f64 = eq8_e116;
        stamper.stamp_potential(
            branches[3],
            eq8_value,
            &[
            ],
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
        let eq9_e119: f64 = (p.p17 * s.v[4]);
        let eq9_e119_d_n0: f64 = (p.p17 * s.dn[4][0]);
        let eq9_e119_d_n1: f64 = (p.p17 * s.dn[4][1]);
        let eq9_e119_d_n2: f64 = (p.p17 * s.dn[4][2]);
        let eq9_e119_d_n3: f64 = (p.p17 * s.dn[4][3]);
        let eq9_e119_d_n4: f64 = (p.p17 * s.dn[4][4]);
        let eq9_e119_d_n5: f64 = (p.p17 * s.dn[4][5]);
        let eq9_e119_d_n6: f64 = (p.p17 * s.dn[4][6]);
        let eq9_e119_d_b0: f64 = (p.p17 * s.db[4][0]);
        let eq9_e119_d_b1: f64 = (p.p17 * s.db[4][1]);
        let eq9_e119_d_b2: f64 = (p.p17 * s.db[4][2]);
        let eq9_e119_d_b3: f64 = (p.p17 * s.db[4][3]);
        let eq9_value: f64 = eq9_e119;
        let eq9_node_derivatives: [f64; 7] = [eq9_e119_d_n0, eq9_e119_d_n1, eq9_e119_d_n2, eq9_e119_d_n3, eq9_e119_d_n4, eq9_e119_d_n5, eq9_e119_d_n6];
        let eq9_branch_derivatives: [f64; 4] = [eq9_e119_d_b0, eq9_e119_d_b1, eq9_e119_d_b2, eq9_e119_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            Some(nodes[5]),
            self.multiplicity * (eq9_value),
            &nodes,
            &eq9_node_derivatives,
            &branches,
            &eq9_branch_derivatives,
            self.multiplicity,
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
        let eq10_e122: f64 = (p.p17 * s.v[5]);
        let eq10_e122_d_n0: f64 = (p.p17 * s.dn[5][0]);
        let eq10_e122_d_n1: f64 = (p.p17 * s.dn[5][1]);
        let eq10_e122_d_n2: f64 = (p.p17 * s.dn[5][2]);
        let eq10_e122_d_n3: f64 = (p.p17 * s.dn[5][3]);
        let eq10_e122_d_n4: f64 = (p.p17 * s.dn[5][4]);
        let eq10_e122_d_n5: f64 = (p.p17 * s.dn[5][5]);
        let eq10_e122_d_n6: f64 = (p.p17 * s.dn[5][6]);
        let eq10_e122_d_b0: f64 = (p.p17 * s.db[5][0]);
        let eq10_e122_d_b1: f64 = (p.p17 * s.db[5][1]);
        let eq10_e122_d_b2: f64 = (p.p17 * s.db[5][2]);
        let eq10_e122_d_b3: f64 = (p.p17 * s.db[5][3]);
        let eq10_value: f64 = eq10_e122;
        let eq10_node_derivatives: [f64; 7] = [eq10_e122_d_n0, eq10_e122_d_n1, eq10_e122_d_n2, eq10_e122_d_n3, eq10_e122_d_n4, eq10_e122_d_n5, eq10_e122_d_n6];
        let eq10_branch_derivatives: [f64; 4] = [eq10_e122_d_b0, eq10_e122_d_b1, eq10_e122_d_b2, eq10_e122_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            Some(nodes[1]),
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
        let eq11_e124: f64 = self.eval_ddt(0, s.v[3]);
        let eq11_e124_d_n0: f64 = self.ddt_jacobian(s.dn[3][0]);
        let eq11_e124_d_n1: f64 = self.ddt_jacobian(s.dn[3][1]);
        let eq11_e124_d_n2: f64 = self.ddt_jacobian(s.dn[3][2]);
        let eq11_e124_d_n3: f64 = self.ddt_jacobian(s.dn[3][3]);
        let eq11_e124_d_n4: f64 = self.ddt_jacobian(s.dn[3][4]);
        let eq11_e124_d_n5: f64 = self.ddt_jacobian(s.dn[3][5]);
        let eq11_e124_d_n6: f64 = self.ddt_jacobian(s.dn[3][6]);
        let eq11_e124_d_b0: f64 = self.ddt_jacobian(s.db[3][0]);
        let eq11_e124_d_b1: f64 = self.ddt_jacobian(s.db[3][1]);
        let eq11_e124_d_b2: f64 = self.ddt_jacobian(s.db[3][2]);
        let eq11_e124_d_b3: f64 = self.ddt_jacobian(s.db[3][3]);
        let eq11_value: f64 = eq11_e124;
        let eq11_node_derivatives: [f64; 7] = [eq11_e124_d_n0, eq11_e124_d_n1, eq11_e124_d_n2, eq11_e124_d_n3, eq11_e124_d_n4, eq11_e124_d_n5, eq11_e124_d_n6];
        let eq11_branch_derivatives: [f64; 4] = [eq11_e124_d_b0, eq11_e124_d_b1, eq11_e124_d_b2, eq11_e124_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            Some(nodes[5]),
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
        let eq12_e126: f64 = (-s.v[92]);
        let eq12_e126_d_n0: f64 = (-s.dn[92][0]);
        let eq12_e126_d_n1: f64 = (-s.dn[92][1]);
        let eq12_e126_d_n2: f64 = (-s.dn[92][2]);
        let eq12_e126_d_n3: f64 = (-s.dn[92][3]);
        let eq12_e126_d_n4: f64 = (-s.dn[92][4]);
        let eq12_e126_d_n5: f64 = (-s.dn[92][5]);
        let eq12_e126_d_n6: f64 = (-s.dn[92][6]);
        let eq12_e126_d_b0: f64 = (-s.db[92][0]);
        let eq12_e126_d_b1: f64 = (-s.db[92][1]);
        let eq12_e126_d_b2: f64 = (-s.db[92][2]);
        let eq12_e126_d_b3: f64 = (-s.db[92][3]);
        let eq12_value: f64 = eq12_e126;
        let eq12_node_derivatives: [f64; 7] = [eq12_e126_d_n0, eq12_e126_d_n1, eq12_e126_d_n2, eq12_e126_d_n3, eq12_e126_d_n4, eq12_e126_d_n5, eq12_e126_d_n6];
        let eq12_branch_derivatives: [f64; 4] = [eq12_e126_d_b0, eq12_e126_d_b1, eq12_e126_d_b2, eq12_e126_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[6]),
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
        let eq13_e128: f64 = self.eval_ddt(1, s.v[105]);
        let eq13_e128_d_n0: f64 = self.ddt_jacobian(s.dn[105][0]);
        let eq13_e128_d_n1: f64 = self.ddt_jacobian(s.dn[105][1]);
        let eq13_e128_d_n2: f64 = self.ddt_jacobian(s.dn[105][2]);
        let eq13_e128_d_n3: f64 = self.ddt_jacobian(s.dn[105][3]);
        let eq13_e128_d_n4: f64 = self.ddt_jacobian(s.dn[105][4]);
        let eq13_e128_d_n5: f64 = self.ddt_jacobian(s.dn[105][5]);
        let eq13_e128_d_n6: f64 = self.ddt_jacobian(s.dn[105][6]);
        let eq13_e128_d_b0: f64 = self.ddt_jacobian(s.db[105][0]);
        let eq13_e128_d_b1: f64 = self.ddt_jacobian(s.db[105][1]);
        let eq13_e128_d_b2: f64 = self.ddt_jacobian(s.db[105][2]);
        let eq13_e128_d_b3: f64 = self.ddt_jacobian(s.db[105][3]);
        let eq13_value: f64 = eq13_e128;
        let eq13_node_derivatives: [f64; 7] = [eq13_e128_d_n0, eq13_e128_d_n1, eq13_e128_d_n2, eq13_e128_d_n3, eq13_e128_d_n4, eq13_e128_d_n5, eq13_e128_d_n6];
        let eq13_branch_derivatives: [f64; 4] = [eq13_e128_d_b0, eq13_e128_d_b1, eq13_e128_d_b2, eq13_e128_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[6]),
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
        let eq14_e130: f64 = self.eval_ddt(2, s.v[106]);
        let eq14_e130_d_n0: f64 = self.ddt_jacobian(s.dn[106][0]);
        let eq14_e130_d_n1: f64 = self.ddt_jacobian(s.dn[106][1]);
        let eq14_e130_d_n2: f64 = self.ddt_jacobian(s.dn[106][2]);
        let eq14_e130_d_n3: f64 = self.ddt_jacobian(s.dn[106][3]);
        let eq14_e130_d_n4: f64 = self.ddt_jacobian(s.dn[106][4]);
        let eq14_e130_d_n5: f64 = self.ddt_jacobian(s.dn[106][5]);
        let eq14_e130_d_n6: f64 = self.ddt_jacobian(s.dn[106][6]);
        let eq14_e130_d_b0: f64 = self.ddt_jacobian(s.db[106][0]);
        let eq14_e130_d_b1: f64 = self.ddt_jacobian(s.db[106][1]);
        let eq14_e130_d_b2: f64 = self.ddt_jacobian(s.db[106][2]);
        let eq14_e130_d_b3: f64 = self.ddt_jacobian(s.db[106][3]);
        let eq14_value: f64 = eq14_e130;
        let eq14_node_derivatives: [f64; 7] = [eq14_e130_d_n0, eq14_e130_d_n1, eq14_e130_d_n2, eq14_e130_d_n3, eq14_e130_d_n4, eq14_e130_d_n5, eq14_e130_d_n6];
        let eq14_branch_derivatives: [f64; 4] = [eq14_e130_d_b0, eq14_e130_d_b1, eq14_e130_d_b2, eq14_e130_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[1]),
            self.multiplicity * (eq14_value),
            &nodes,
            &eq14_node_derivatives,
            &branches,
            &eq14_branch_derivatives,
            self.multiplicity,
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
        let (eq15_e141,) = {
    if (p.p49 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq15_value: f64 = eq15_e141;
        stamper.stamp_current(
            Some(nodes[4]),
            Some(nodes[5]),
            self.multiplicity * (eq15_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_16_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq16_e152,) = {
    if (p.p49 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq16_value: f64 = eq16_e152;
        stamper.stamp_current(
            Some(nodes[4]),
            Some(nodes[1]),
            self.multiplicity * (eq16_value),
            &[
            ],
        );
    }

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
        let (eq17_e158,) = {
    if (p.p16 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq17_value: f64 = eq17_e158;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[3]),
            self.multiplicity * (eq17_value),
            &[
            ],
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
        let (eq18_e164,) = {
    if (p.p16 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq18_value: f64 = eq18_e164;
        stamper.stamp_current(
            Some(nodes[3]),
            Some(nodes[4]),
            self.multiplicity * (eq18_value),
            &[
            ],
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
        let (eq19_e170,) = {
    if (p.p16 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e170;
        stamper.stamp_current(
            Some(nodes[1]),
            Some(nodes[2]),
            self.multiplicity * (eq19_value),
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
        let (eq20_e176,) = {
    if (p.p16 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e176;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[1]),
            self.multiplicity * (eq20_value),
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
        let (eq21_e182,) = {
    if (p.p16 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq21_value: f64 = eq21_e182;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[1]),
            self.multiplicity * (eq21_value),
            &[
            ],
        );
    }
}
