#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_87_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq87_e2639,) = {
    if (s.v[1728] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq87_value: f64 = eq87_e2639;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[8]),
            self.multiplicity * (eq87_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_88_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq88_e2653,) = {
    if ((s.v[1728] != 0.0) && (s.v[1729] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq88_value: f64 = eq88_e2653;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[6]),
            self.multiplicity * (eq88_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_89_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq89_e2665,) = {
    if (s.v[1730] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq89_value: f64 = eq89_e2665;
        stamper.stamp_current(
            Some(nodes[1]),
            Some(nodes[10]),
            self.multiplicity * (eq89_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_90_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq90_e2671,) = {
    if (s.v[1731] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq90_value: f64 = eq90_e2671;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq90_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_91_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq91_e2675,) = {
    if (s.v[1731] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq91_value: f64 = eq91_e2675;
        stamper.stamp_potential(
            branches[16],
            eq91_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_92_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq92_e2680, eq92_e2680_d_n16,) = {
    if (!(s.v[1731] != 0.0)) {
        ((nv16 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq92_value: f64 = eq92_e2680;
        stamper.stamp_current(
            Some(nodes[16]),
            None,
            self.multiplicity * (eq92_value),
            &[
                GeneratedDerivative::node(nodes[16], self.multiplicity * eq92_e2680_d_n16),
            ],
        );
    }

    pub(super) fn stamp_transient_equation_93_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq93_e2687,) = {
    if (!(s.v[1731] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq93_value: f64 = eq93_e2687;
        stamper.stamp_current(
            Some(nodes[16]),
            None,
            self.multiplicity * (eq93_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_94_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq94_e2700,) = {
    if (!(s.v[1731] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq94_value: f64 = eq94_e2700;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq94_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_95_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq95_e2707, eq95_e2707_d_n0, eq95_e2707_d_n1, eq95_e2707_d_n2, eq95_e2707_d_n3, eq95_e2707_d_n4, eq95_e2707_d_n5, eq95_e2707_d_n6, eq95_e2707_d_n7, eq95_e2707_d_n8, eq95_e2707_d_n9, eq95_e2707_d_n10, eq95_e2707_d_n11, eq95_e2707_d_n12, eq95_e2707_d_n13, eq95_e2707_d_n14, eq95_e2707_d_n15, eq95_e2707_d_n16,) = {
    if (!(s.v[1731] != 0.0)) {
        let eq95_e2705: f64 = (s.v[631] * (nv16 - 0.0));
        let eq95_e2705_d_n0: f64 = (s.dn[631][0] * (nv16 - 0.0));
        let eq95_e2705_d_n1: f64 = (s.dn[631][1] * (nv16 - 0.0));
        let eq95_e2705_d_n2: f64 = (s.dn[631][2] * (nv16 - 0.0));
        let eq95_e2705_d_n3: f64 = (s.dn[631][3] * (nv16 - 0.0));
        let eq95_e2705_d_n4: f64 = (s.dn[631][4] * (nv16 - 0.0));
        let eq95_e2705_d_n5: f64 = (s.dn[631][5] * (nv16 - 0.0));
        let eq95_e2705_d_n6: f64 = (s.dn[631][6] * (nv16 - 0.0));
        let eq95_e2705_d_n7: f64 = (s.dn[631][7] * (nv16 - 0.0));
        let eq95_e2705_d_n8: f64 = (s.dn[631][8] * (nv16 - 0.0));
        let eq95_e2705_d_n9: f64 = (s.dn[631][9] * (nv16 - 0.0));
        let eq95_e2705_d_n10: f64 = (s.dn[631][10] * (nv16 - 0.0));
        let eq95_e2705_d_n11: f64 = (s.dn[631][11] * (nv16 - 0.0));
        let eq95_e2705_d_n12: f64 = (s.dn[631][12] * (nv16 - 0.0));
        let eq95_e2705_d_n13: f64 = (s.dn[631][13] * (nv16 - 0.0));
        let eq95_e2705_d_n14: f64 = (s.dn[631][14] * (nv16 - 0.0));
        let eq95_e2705_d_n15: f64 = (s.dn[631][15] * (nv16 - 0.0));
        let eq95_e2705_d_n16: f64 = ((s.dn[631][16] * (nv16 - 0.0)) + s.v[631]);
        (eq95_e2705, eq95_e2705_d_n0, eq95_e2705_d_n1, eq95_e2705_d_n2, eq95_e2705_d_n3, eq95_e2705_d_n4, eq95_e2705_d_n5, eq95_e2705_d_n6, eq95_e2705_d_n7, eq95_e2705_d_n8, eq95_e2705_d_n9, eq95_e2705_d_n10, eq95_e2705_d_n11, eq95_e2705_d_n12, eq95_e2705_d_n13, eq95_e2705_d_n14, eq95_e2705_d_n15, eq95_e2705_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq95_value: f64 = eq95_e2707;
        let eq95_node_derivatives: [f64; 17] = [eq95_e2707_d_n0, eq95_e2707_d_n1, eq95_e2707_d_n2, eq95_e2707_d_n3, eq95_e2707_d_n4, eq95_e2707_d_n5, eq95_e2707_d_n6, eq95_e2707_d_n7, eq95_e2707_d_n8, eq95_e2707_d_n9, eq95_e2707_d_n10, eq95_e2707_d_n11, eq95_e2707_d_n12, eq95_e2707_d_n13, eq95_e2707_d_n14, eq95_e2707_d_n15, eq95_e2707_d_n16];
        let eq95_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq95_value),
            &nodes,
            &eq95_node_derivatives,
            &branches,
            &eq95_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_96_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq96_e2717, eq96_e2717_d_n0, eq96_e2717_d_n1, eq96_e2717_d_n2, eq96_e2717_d_n3, eq96_e2717_d_n4, eq96_e2717_d_n5, eq96_e2717_d_n6, eq96_e2717_d_n7, eq96_e2717_d_n8, eq96_e2717_d_n9, eq96_e2717_d_n10, eq96_e2717_d_n11, eq96_e2717_d_n12, eq96_e2717_d_n13, eq96_e2717_d_n14, eq96_e2717_d_n15, eq96_e2717_d_n16,) = {
    if (!(s.v[1731] != 0.0)) {
        let eq96_e2712: f64 = (0.7071 * s.v[632]);
        let eq96_e2712_d_n0: f64 = (0.7071 * s.dn[632][0]);
        let eq96_e2712_d_n1: f64 = (0.7071 * s.dn[632][1]);
        let eq96_e2712_d_n2: f64 = (0.7071 * s.dn[632][2]);
        let eq96_e2712_d_n3: f64 = (0.7071 * s.dn[632][3]);
        let eq96_e2712_d_n4: f64 = (0.7071 * s.dn[632][4]);
        let eq96_e2712_d_n5: f64 = (0.7071 * s.dn[632][5]);
        let eq96_e2712_d_n6: f64 = (0.7071 * s.dn[632][6]);
        let eq96_e2712_d_n7: f64 = (0.7071 * s.dn[632][7]);
        let eq96_e2712_d_n8: f64 = (0.7071 * s.dn[632][8]);
        let eq96_e2712_d_n9: f64 = (0.7071 * s.dn[632][9]);
        let eq96_e2712_d_n10: f64 = (0.7071 * s.dn[632][10]);
        let eq96_e2712_d_n11: f64 = (0.7071 * s.dn[632][11]);
        let eq96_e2712_d_n12: f64 = (0.7071 * s.dn[632][12]);
        let eq96_e2712_d_n13: f64 = (0.7071 * s.dn[632][13]);
        let eq96_e2712_d_n14: f64 = (0.7071 * s.dn[632][14]);
        let eq96_e2712_d_n15: f64 = (0.7071 * s.dn[632][15]);
        let eq96_e2712_d_n16: f64 = (0.7071 * s.dn[632][16]);
        let eq96_e2714: f64 = (eq96_e2712 * (nv16 - 0.0));
        let eq96_e2714_d_n0: f64 = (eq96_e2712_d_n0 * (nv16 - 0.0));
        let eq96_e2714_d_n1: f64 = (eq96_e2712_d_n1 * (nv16 - 0.0));
        let eq96_e2714_d_n2: f64 = (eq96_e2712_d_n2 * (nv16 - 0.0));
        let eq96_e2714_d_n3: f64 = (eq96_e2712_d_n3 * (nv16 - 0.0));
        let eq96_e2714_d_n4: f64 = (eq96_e2712_d_n4 * (nv16 - 0.0));
        let eq96_e2714_d_n5: f64 = (eq96_e2712_d_n5 * (nv16 - 0.0));
        let eq96_e2714_d_n6: f64 = (eq96_e2712_d_n6 * (nv16 - 0.0));
        let eq96_e2714_d_n7: f64 = (eq96_e2712_d_n7 * (nv16 - 0.0));
        let eq96_e2714_d_n8: f64 = (eq96_e2712_d_n8 * (nv16 - 0.0));
        let eq96_e2714_d_n9: f64 = (eq96_e2712_d_n9 * (nv16 - 0.0));
        let eq96_e2714_d_n10: f64 = (eq96_e2712_d_n10 * (nv16 - 0.0));
        let eq96_e2714_d_n11: f64 = (eq96_e2712_d_n11 * (nv16 - 0.0));
        let eq96_e2714_d_n12: f64 = (eq96_e2712_d_n12 * (nv16 - 0.0));
        let eq96_e2714_d_n13: f64 = (eq96_e2712_d_n13 * (nv16 - 0.0));
        let eq96_e2714_d_n14: f64 = (eq96_e2712_d_n14 * (nv16 - 0.0));
        let eq96_e2714_d_n15: f64 = (eq96_e2712_d_n15 * (nv16 - 0.0));
        let eq96_e2714_d_n16: f64 = ((eq96_e2712_d_n16 * (nv16 - 0.0)) + eq96_e2712);
        let eq96_e2715: f64 = self.eval_ddt(25, eq96_e2714);
        let eq96_e2715_d_n0: f64 = self.ddt_jacobian(eq96_e2714_d_n0);
        let eq96_e2715_d_n1: f64 = self.ddt_jacobian(eq96_e2714_d_n1);
        let eq96_e2715_d_n2: f64 = self.ddt_jacobian(eq96_e2714_d_n2);
        let eq96_e2715_d_n3: f64 = self.ddt_jacobian(eq96_e2714_d_n3);
        let eq96_e2715_d_n4: f64 = self.ddt_jacobian(eq96_e2714_d_n4);
        let eq96_e2715_d_n5: f64 = self.ddt_jacobian(eq96_e2714_d_n5);
        let eq96_e2715_d_n6: f64 = self.ddt_jacobian(eq96_e2714_d_n6);
        let eq96_e2715_d_n7: f64 = self.ddt_jacobian(eq96_e2714_d_n7);
        let eq96_e2715_d_n8: f64 = self.ddt_jacobian(eq96_e2714_d_n8);
        let eq96_e2715_d_n9: f64 = self.ddt_jacobian(eq96_e2714_d_n9);
        let eq96_e2715_d_n10: f64 = self.ddt_jacobian(eq96_e2714_d_n10);
        let eq96_e2715_d_n11: f64 = self.ddt_jacobian(eq96_e2714_d_n11);
        let eq96_e2715_d_n12: f64 = self.ddt_jacobian(eq96_e2714_d_n12);
        let eq96_e2715_d_n13: f64 = self.ddt_jacobian(eq96_e2714_d_n13);
        let eq96_e2715_d_n14: f64 = self.ddt_jacobian(eq96_e2714_d_n14);
        let eq96_e2715_d_n15: f64 = self.ddt_jacobian(eq96_e2714_d_n15);
        let eq96_e2715_d_n16: f64 = self.ddt_jacobian(eq96_e2714_d_n16);
        (eq96_e2715, eq96_e2715_d_n0, eq96_e2715_d_n1, eq96_e2715_d_n2, eq96_e2715_d_n3, eq96_e2715_d_n4, eq96_e2715_d_n5, eq96_e2715_d_n6, eq96_e2715_d_n7, eq96_e2715_d_n8, eq96_e2715_d_n9, eq96_e2715_d_n10, eq96_e2715_d_n11, eq96_e2715_d_n12, eq96_e2715_d_n13, eq96_e2715_d_n14, eq96_e2715_d_n15, eq96_e2715_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq96_value: f64 = eq96_e2717;
        let eq96_node_derivatives: [f64; 17] = [eq96_e2717_d_n0, eq96_e2717_d_n1, eq96_e2717_d_n2, eq96_e2717_d_n3, eq96_e2717_d_n4, eq96_e2717_d_n5, eq96_e2717_d_n6, eq96_e2717_d_n7, eq96_e2717_d_n8, eq96_e2717_d_n9, eq96_e2717_d_n10, eq96_e2717_d_n11, eq96_e2717_d_n12, eq96_e2717_d_n13, eq96_e2717_d_n14, eq96_e2717_d_n15, eq96_e2717_d_n16];
        let eq96_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            self.multiplicity * (eq96_value),
            &nodes,
            &eq96_node_derivatives,
            &branches,
            &eq96_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_97_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq97_e2727, eq97_e2727_d_n0, eq97_e2727_d_n1, eq97_e2727_d_n2, eq97_e2727_d_n3, eq97_e2727_d_n4, eq97_e2727_d_n5, eq97_e2727_d_n6, eq97_e2727_d_n7, eq97_e2727_d_n8, eq97_e2727_d_n9, eq97_e2727_d_n10, eq97_e2727_d_n11, eq97_e2727_d_n12, eq97_e2727_d_n13, eq97_e2727_d_n14, eq97_e2727_d_n15, eq97_e2727_d_n16,) = {
    if (!(s.v[1731] != 0.0)) {
        let eq97_e2722: f64 = (0.7071 * s.v[632]);
        let eq97_e2722_d_n0: f64 = (0.7071 * s.dn[632][0]);
        let eq97_e2722_d_n1: f64 = (0.7071 * s.dn[632][1]);
        let eq97_e2722_d_n2: f64 = (0.7071 * s.dn[632][2]);
        let eq97_e2722_d_n3: f64 = (0.7071 * s.dn[632][3]);
        let eq97_e2722_d_n4: f64 = (0.7071 * s.dn[632][4]);
        let eq97_e2722_d_n5: f64 = (0.7071 * s.dn[632][5]);
        let eq97_e2722_d_n6: f64 = (0.7071 * s.dn[632][6]);
        let eq97_e2722_d_n7: f64 = (0.7071 * s.dn[632][7]);
        let eq97_e2722_d_n8: f64 = (0.7071 * s.dn[632][8]);
        let eq97_e2722_d_n9: f64 = (0.7071 * s.dn[632][9]);
        let eq97_e2722_d_n10: f64 = (0.7071 * s.dn[632][10]);
        let eq97_e2722_d_n11: f64 = (0.7071 * s.dn[632][11]);
        let eq97_e2722_d_n12: f64 = (0.7071 * s.dn[632][12]);
        let eq97_e2722_d_n13: f64 = (0.7071 * s.dn[632][13]);
        let eq97_e2722_d_n14: f64 = (0.7071 * s.dn[632][14]);
        let eq97_e2722_d_n15: f64 = (0.7071 * s.dn[632][15]);
        let eq97_e2722_d_n16: f64 = (0.7071 * s.dn[632][16]);
        let eq97_e2724: f64 = (eq97_e2722 * (nv16 - 0.0));
        let eq97_e2724_d_n0: f64 = (eq97_e2722_d_n0 * (nv16 - 0.0));
        let eq97_e2724_d_n1: f64 = (eq97_e2722_d_n1 * (nv16 - 0.0));
        let eq97_e2724_d_n2: f64 = (eq97_e2722_d_n2 * (nv16 - 0.0));
        let eq97_e2724_d_n3: f64 = (eq97_e2722_d_n3 * (nv16 - 0.0));
        let eq97_e2724_d_n4: f64 = (eq97_e2722_d_n4 * (nv16 - 0.0));
        let eq97_e2724_d_n5: f64 = (eq97_e2722_d_n5 * (nv16 - 0.0));
        let eq97_e2724_d_n6: f64 = (eq97_e2722_d_n6 * (nv16 - 0.0));
        let eq97_e2724_d_n7: f64 = (eq97_e2722_d_n7 * (nv16 - 0.0));
        let eq97_e2724_d_n8: f64 = (eq97_e2722_d_n8 * (nv16 - 0.0));
        let eq97_e2724_d_n9: f64 = (eq97_e2722_d_n9 * (nv16 - 0.0));
        let eq97_e2724_d_n10: f64 = (eq97_e2722_d_n10 * (nv16 - 0.0));
        let eq97_e2724_d_n11: f64 = (eq97_e2722_d_n11 * (nv16 - 0.0));
        let eq97_e2724_d_n12: f64 = (eq97_e2722_d_n12 * (nv16 - 0.0));
        let eq97_e2724_d_n13: f64 = (eq97_e2722_d_n13 * (nv16 - 0.0));
        let eq97_e2724_d_n14: f64 = (eq97_e2722_d_n14 * (nv16 - 0.0));
        let eq97_e2724_d_n15: f64 = (eq97_e2722_d_n15 * (nv16 - 0.0));
        let eq97_e2724_d_n16: f64 = ((eq97_e2722_d_n16 * (nv16 - 0.0)) + eq97_e2722);
        let eq97_e2725: f64 = self.eval_ddt(26, eq97_e2724);
        let eq97_e2725_d_n0: f64 = self.ddt_jacobian(eq97_e2724_d_n0);
        let eq97_e2725_d_n1: f64 = self.ddt_jacobian(eq97_e2724_d_n1);
        let eq97_e2725_d_n2: f64 = self.ddt_jacobian(eq97_e2724_d_n2);
        let eq97_e2725_d_n3: f64 = self.ddt_jacobian(eq97_e2724_d_n3);
        let eq97_e2725_d_n4: f64 = self.ddt_jacobian(eq97_e2724_d_n4);
        let eq97_e2725_d_n5: f64 = self.ddt_jacobian(eq97_e2724_d_n5);
        let eq97_e2725_d_n6: f64 = self.ddt_jacobian(eq97_e2724_d_n6);
        let eq97_e2725_d_n7: f64 = self.ddt_jacobian(eq97_e2724_d_n7);
        let eq97_e2725_d_n8: f64 = self.ddt_jacobian(eq97_e2724_d_n8);
        let eq97_e2725_d_n9: f64 = self.ddt_jacobian(eq97_e2724_d_n9);
        let eq97_e2725_d_n10: f64 = self.ddt_jacobian(eq97_e2724_d_n10);
        let eq97_e2725_d_n11: f64 = self.ddt_jacobian(eq97_e2724_d_n11);
        let eq97_e2725_d_n12: f64 = self.ddt_jacobian(eq97_e2724_d_n12);
        let eq97_e2725_d_n13: f64 = self.ddt_jacobian(eq97_e2724_d_n13);
        let eq97_e2725_d_n14: f64 = self.ddt_jacobian(eq97_e2724_d_n14);
        let eq97_e2725_d_n15: f64 = self.ddt_jacobian(eq97_e2724_d_n15);
        let eq97_e2725_d_n16: f64 = self.ddt_jacobian(eq97_e2724_d_n16);
        (eq97_e2725, eq97_e2725_d_n0, eq97_e2725_d_n1, eq97_e2725_d_n2, eq97_e2725_d_n3, eq97_e2725_d_n4, eq97_e2725_d_n5, eq97_e2725_d_n6, eq97_e2725_d_n7, eq97_e2725_d_n8, eq97_e2725_d_n9, eq97_e2725_d_n10, eq97_e2725_d_n11, eq97_e2725_d_n12, eq97_e2725_d_n13, eq97_e2725_d_n14, eq97_e2725_d_n15, eq97_e2725_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq97_value: f64 = eq97_e2727;
        let eq97_node_derivatives: [f64; 17] = [eq97_e2727_d_n0, eq97_e2727_d_n1, eq97_e2727_d_n2, eq97_e2727_d_n3, eq97_e2727_d_n4, eq97_e2727_d_n5, eq97_e2727_d_n6, eq97_e2727_d_n7, eq97_e2727_d_n8, eq97_e2727_d_n9, eq97_e2727_d_n10, eq97_e2727_d_n11, eq97_e2727_d_n12, eq97_e2727_d_n13, eq97_e2727_d_n14, eq97_e2727_d_n15, eq97_e2727_d_n16];
        let eq97_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[5]),
            self.multiplicity * (eq97_value),
            &nodes,
            &eq97_node_derivatives,
            &branches,
            &eq97_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_98_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq98_e2742,) = {
    if ((s.v[1732] != 0.0) && (s.v[1733] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq98_value: f64 = eq98_e2742;
        stamper.stamp_current(
            Some(nodes[11]),
            Some(nodes[6]),
            self.multiplicity * (eq98_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_99_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq99_e2757,) = {
    if ((s.v[1732] != 0.0) && (s.v[1733] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq99_value: f64 = eq99_e2757;
        stamper.stamp_current(
            Some(nodes[11]),
            Some(nodes[5]),
            self.multiplicity * (eq99_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_100_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq100_e2773,) = {
    if ((s.v[1732] != 0.0) && (!(s.v[1733] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq100_value: f64 = eq100_e2773;
        stamper.stamp_current(
            Some(nodes[11]),
            Some(nodes[5]),
            self.multiplicity * (eq100_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_101_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq101_e2789,) = {
    if ((s.v[1732] != 0.0) && (!(s.v[1733] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq101_value: f64 = eq101_e2789;
        stamper.stamp_current(
            Some(nodes[11]),
            Some(nodes[6]),
            self.multiplicity * (eq101_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_102_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq102_e2804,) = {
    if ((s.v[1734] != 0.0) && (s.v[1735] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq102_value: f64 = eq102_e2804;
        stamper.stamp_current(
            Some(nodes[11]),
            Some(nodes[3]),
            self.multiplicity * (eq102_value),
            &[
            ],
        );
    }
}
