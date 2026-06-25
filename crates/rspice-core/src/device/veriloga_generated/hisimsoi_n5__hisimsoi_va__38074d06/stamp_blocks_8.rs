#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_65_block_0(
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
        let (eq65_e763, eq65_e763_d_n16,) = {
    if ((!(s.v[1849] != 0.0)) && (p.p34 != 0.0)) {
        let eq65_e761: f64 = ((nv16 - 0.0) * 1e-12);
        let eq65_e761_d_n16: f64 = 1e-12;
        (eq65_e761, eq65_e761_d_n16,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e763;
        stamper.stamp_current(
            Some(nodes[16]),
            None,
            self.multiplicity * (eq65_value),
            &[
                GeneratedDerivative::node(nodes[16], self.multiplicity * eq65_e763_d_n16),
            ],
        );
    }

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
        let (eq66_e772, eq66_e772_d_n13,) = {
    if ((!(s.v[1849] != 0.0)) && (p.p34 != 0.0)) {
        let eq66_e770: f64 = ((nv13 - 0.0) * 1e-12);
        let eq66_e770_d_n13: f64 = 1e-12;
        (eq66_e770, eq66_e770_d_n13,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq66_value: f64 = eq66_e772;
        stamper.stamp_current(
            Some(nodes[13]),
            None,
            self.multiplicity * (eq66_value),
            &[
                GeneratedDerivative::node(nodes[13], self.multiplicity * eq66_e772_d_n13),
            ],
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq67_e784, eq67_e784_d_n0, eq67_e784_d_n1, eq67_e784_d_n2, eq67_e784_d_n3, eq67_e784_d_n4, eq67_e784_d_n5, eq67_e784_d_n6, eq67_e784_d_n7, eq67_e784_d_n8, eq67_e784_d_n9, eq67_e784_d_n10, eq67_e784_d_n11, eq67_e784_d_n12, eq67_e784_d_n13, eq67_e784_d_n14, eq67_e784_d_n15, eq67_e784_d_n16, eq67_e784_d_n17, eq67_e784_d_n18, eq67_e784_d_b0, eq67_e784_d_b1, eq67_e784_d_b2, eq67_e784_d_b3, eq67_e784_d_b4, eq67_e784_d_b5, eq67_e784_d_b6, eq67_e784_d_b7, eq67_e784_d_b8, eq67_e784_d_b9, eq67_e784_d_b10, eq67_e784_d_b11,) = {
    if ((!(s.v[1849] != 0.0)) && (p.p34 != 0.0)) {
        let eq67_e779: f64 = (1e-9 / 0.0001);
        let eq67_e781: f64 = (eq67_e779 * (nv15 - 0.0));
        let eq67_e782: f64 = self.eval_ddt(12, eq67_e781);
        let eq67_e782_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_n15: f64 = self.ddt_jacobian(eq67_e779);
        let eq67_e782_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_b4: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_b5: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_b6: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_b7: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_b8: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_b9: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_b10: f64 = self.ddt_jacobian(0.0);
        let eq67_e782_d_b11: f64 = self.ddt_jacobian(0.0);
        (eq67_e782, eq67_e782_d_n0, eq67_e782_d_n1, eq67_e782_d_n2, eq67_e782_d_n3, eq67_e782_d_n4, eq67_e782_d_n5, eq67_e782_d_n6, eq67_e782_d_n7, eq67_e782_d_n8, eq67_e782_d_n9, eq67_e782_d_n10, eq67_e782_d_n11, eq67_e782_d_n12, eq67_e782_d_n13, eq67_e782_d_n14, eq67_e782_d_n15, eq67_e782_d_n16, eq67_e782_d_n17, eq67_e782_d_n18, eq67_e782_d_b0, eq67_e782_d_b1, eq67_e782_d_b2, eq67_e782_d_b3, eq67_e782_d_b4, eq67_e782_d_b5, eq67_e782_d_b6, eq67_e782_d_b7, eq67_e782_d_b8, eq67_e782_d_b9, eq67_e782_d_b10, eq67_e782_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e784;
        let eq67_node_derivatives: [f64; 19] = [eq67_e784_d_n0, eq67_e784_d_n1, eq67_e784_d_n2, eq67_e784_d_n3, eq67_e784_d_n4, eq67_e784_d_n5, eq67_e784_d_n6, eq67_e784_d_n7, eq67_e784_d_n8, eq67_e784_d_n9, eq67_e784_d_n10, eq67_e784_d_n11, eq67_e784_d_n12, eq67_e784_d_n13, eq67_e784_d_n14, eq67_e784_d_n15, eq67_e784_d_n16, eq67_e784_d_n17, eq67_e784_d_n18];
        let eq67_branch_derivatives: [f64; 12] = [eq67_e784_d_b0, eq67_e784_d_b1, eq67_e784_d_b2, eq67_e784_d_b3, eq67_e784_d_b4, eq67_e784_d_b5, eq67_e784_d_b6, eq67_e784_d_b7, eq67_e784_d_b8, eq67_e784_d_b9, eq67_e784_d_b10, eq67_e784_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[15]),
            None,
            self.multiplicity * (eq67_value),
            &nodes,
            &eq67_node_derivatives,
            &branches,
            &eq67_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_68_block_0(
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
        let (eq68_e796, eq68_e796_d_n0, eq68_e796_d_n1, eq68_e796_d_n2, eq68_e796_d_n3, eq68_e796_d_n4, eq68_e796_d_n5, eq68_e796_d_n6, eq68_e796_d_n7, eq68_e796_d_n8, eq68_e796_d_n9, eq68_e796_d_n10, eq68_e796_d_n11, eq68_e796_d_n12, eq68_e796_d_n13, eq68_e796_d_n14, eq68_e796_d_n15, eq68_e796_d_n16, eq68_e796_d_n17, eq68_e796_d_n18, eq68_e796_d_b0, eq68_e796_d_b1, eq68_e796_d_b2, eq68_e796_d_b3, eq68_e796_d_b4, eq68_e796_d_b5, eq68_e796_d_b6, eq68_e796_d_b7, eq68_e796_d_b8, eq68_e796_d_b9, eq68_e796_d_b10, eq68_e796_d_b11,) = {
    if ((!(s.v[1849] != 0.0)) && (p.p34 != 0.0)) {
        let eq68_e791: f64 = (1e-9 / 0.0001);
        let eq68_e793: f64 = (eq68_e791 * (nv16 - 0.0));
        let eq68_e794: f64 = self.eval_ddt(13, eq68_e793);
        let eq68_e794_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_n16: f64 = self.ddt_jacobian(eq68_e791);
        let eq68_e794_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_b4: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_b5: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_b6: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_b7: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_b8: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_b9: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_b10: f64 = self.ddt_jacobian(0.0);
        let eq68_e794_d_b11: f64 = self.ddt_jacobian(0.0);
        (eq68_e794, eq68_e794_d_n0, eq68_e794_d_n1, eq68_e794_d_n2, eq68_e794_d_n3, eq68_e794_d_n4, eq68_e794_d_n5, eq68_e794_d_n6, eq68_e794_d_n7, eq68_e794_d_n8, eq68_e794_d_n9, eq68_e794_d_n10, eq68_e794_d_n11, eq68_e794_d_n12, eq68_e794_d_n13, eq68_e794_d_n14, eq68_e794_d_n15, eq68_e794_d_n16, eq68_e794_d_n17, eq68_e794_d_n18, eq68_e794_d_b0, eq68_e794_d_b1, eq68_e794_d_b2, eq68_e794_d_b3, eq68_e794_d_b4, eq68_e794_d_b5, eq68_e794_d_b6, eq68_e794_d_b7, eq68_e794_d_b8, eq68_e794_d_b9, eq68_e794_d_b10, eq68_e794_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq68_value: f64 = eq68_e796;
        let eq68_node_derivatives: [f64; 19] = [eq68_e796_d_n0, eq68_e796_d_n1, eq68_e796_d_n2, eq68_e796_d_n3, eq68_e796_d_n4, eq68_e796_d_n5, eq68_e796_d_n6, eq68_e796_d_n7, eq68_e796_d_n8, eq68_e796_d_n9, eq68_e796_d_n10, eq68_e796_d_n11, eq68_e796_d_n12, eq68_e796_d_n13, eq68_e796_d_n14, eq68_e796_d_n15, eq68_e796_d_n16, eq68_e796_d_n17, eq68_e796_d_n18];
        let eq68_branch_derivatives: [f64; 12] = [eq68_e796_d_b0, eq68_e796_d_b1, eq68_e796_d_b2, eq68_e796_d_b3, eq68_e796_d_b4, eq68_e796_d_b5, eq68_e796_d_b6, eq68_e796_d_b7, eq68_e796_d_b8, eq68_e796_d_b9, eq68_e796_d_b10, eq68_e796_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[16]),
            None,
            self.multiplicity * (eq68_value),
            &nodes,
            &eq68_node_derivatives,
            &branches,
            &eq68_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_69_block_0(
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
        let (eq69_e808, eq69_e808_d_n0, eq69_e808_d_n1, eq69_e808_d_n2, eq69_e808_d_n3, eq69_e808_d_n4, eq69_e808_d_n5, eq69_e808_d_n6, eq69_e808_d_n7, eq69_e808_d_n8, eq69_e808_d_n9, eq69_e808_d_n10, eq69_e808_d_n11, eq69_e808_d_n12, eq69_e808_d_n13, eq69_e808_d_n14, eq69_e808_d_n15, eq69_e808_d_n16, eq69_e808_d_n17, eq69_e808_d_n18, eq69_e808_d_b0, eq69_e808_d_b1, eq69_e808_d_b2, eq69_e808_d_b3, eq69_e808_d_b4, eq69_e808_d_b5, eq69_e808_d_b6, eq69_e808_d_b7, eq69_e808_d_b8, eq69_e808_d_b9, eq69_e808_d_b10, eq69_e808_d_b11,) = {
    if ((!(s.v[1849] != 0.0)) && (p.p34 != 0.0)) {
        let eq69_e803: f64 = (1e-9 / 0.0001);
        let eq69_e805: f64 = (eq69_e803 * (nv13 - 0.0));
        let eq69_e806: f64 = self.eval_ddt(14, eq69_e805);
        let eq69_e806_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_n13: f64 = self.ddt_jacobian(eq69_e803);
        let eq69_e806_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_b4: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_b5: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_b6: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_b7: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_b8: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_b9: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_b10: f64 = self.ddt_jacobian(0.0);
        let eq69_e806_d_b11: f64 = self.ddt_jacobian(0.0);
        (eq69_e806, eq69_e806_d_n0, eq69_e806_d_n1, eq69_e806_d_n2, eq69_e806_d_n3, eq69_e806_d_n4, eq69_e806_d_n5, eq69_e806_d_n6, eq69_e806_d_n7, eq69_e806_d_n8, eq69_e806_d_n9, eq69_e806_d_n10, eq69_e806_d_n11, eq69_e806_d_n12, eq69_e806_d_n13, eq69_e806_d_n14, eq69_e806_d_n15, eq69_e806_d_n16, eq69_e806_d_n17, eq69_e806_d_n18, eq69_e806_d_b0, eq69_e806_d_b1, eq69_e806_d_b2, eq69_e806_d_b3, eq69_e806_d_b4, eq69_e806_d_b5, eq69_e806_d_b6, eq69_e806_d_b7, eq69_e806_d_b8, eq69_e806_d_b9, eq69_e806_d_b10, eq69_e806_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e808;
        let eq69_node_derivatives: [f64; 19] = [eq69_e808_d_n0, eq69_e808_d_n1, eq69_e808_d_n2, eq69_e808_d_n3, eq69_e808_d_n4, eq69_e808_d_n5, eq69_e808_d_n6, eq69_e808_d_n7, eq69_e808_d_n8, eq69_e808_d_n9, eq69_e808_d_n10, eq69_e808_d_n11, eq69_e808_d_n12, eq69_e808_d_n13, eq69_e808_d_n14, eq69_e808_d_n15, eq69_e808_d_n16, eq69_e808_d_n17, eq69_e808_d_n18];
        let eq69_branch_derivatives: [f64; 12] = [eq69_e808_d_b0, eq69_e808_d_b1, eq69_e808_d_b2, eq69_e808_d_b3, eq69_e808_d_b4, eq69_e808_d_b5, eq69_e808_d_b6, eq69_e808_d_b7, eq69_e808_d_b8, eq69_e808_d_b9, eq69_e808_d_b10, eq69_e808_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            None,
            self.multiplicity * (eq69_value),
            &nodes,
            &eq69_node_derivatives,
            &branches,
            &eq69_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_70_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq70_e816,) = {
    if ((!(s.v[1849] != 0.0)) && (!(p.p34 != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq70_value: f64 = eq70_e816;
        stamper.stamp_potential(
            branches[13],
            eq70_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_71_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq71_e824,) = {
    if ((!(s.v[1849] != 0.0)) && (!(p.p34 != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq71_value: f64 = eq71_e824;
        stamper.stamp_potential(
            branches[14],
            eq71_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_72_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq72_e832,) = {
    if ((!(s.v[1849] != 0.0)) && (!(p.p34 != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq72_value: f64 = eq72_e832;
        stamper.stamp_potential(
            branches[15],
            eq72_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_73_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq73_e836,) = {
    if (s.v[1851] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq73_value: f64 = eq73_e836;
        stamper.stamp_potential(
            branches[16],
            eq73_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_74_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq74_e841,) = {
    if (!(s.v[1851] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq74_value: f64 = eq74_e841;
        stamper.stamp_potential(
            branches[17],
            eq74_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_75_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq75_e846,) = {
    if (!(s.v[1851] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq75_value: f64 = eq75_e846;
        stamper.stamp_potential(
            branches[18],
            eq75_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_10_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq10_e359_q: f64 = s.v[594];
        let eq10_e360: f64 = (p.p50 * s.v[594]);
        let eq10_e360_d_n0: f64 = (p.p50 * s.dn[594][0]);
        let eq10_e360_d_n1: f64 = (p.p50 * s.dn[594][1]);
        let eq10_e360_d_n2: f64 = (p.p50 * s.dn[594][2]);
        let eq10_e360_d_n3: f64 = (p.p50 * s.dn[594][3]);
        let eq10_e360_d_n4: f64 = (p.p50 * s.dn[594][4]);
        let eq10_e360_d_n5: f64 = (p.p50 * s.dn[594][5]);
        let eq10_e360_d_n6: f64 = (p.p50 * s.dn[594][6]);
        let eq10_e360_d_n7: f64 = (p.p50 * s.dn[594][7]);
        let eq10_e360_d_n8: f64 = (p.p50 * s.dn[594][8]);
        let eq10_e360_d_n9: f64 = (p.p50 * s.dn[594][9]);
        let eq10_e360_d_n10: f64 = (p.p50 * s.dn[594][10]);
        let eq10_e360_d_n11: f64 = (p.p50 * s.dn[594][11]);
        let eq10_e360_d_n12: f64 = (p.p50 * s.dn[594][12]);
        let eq10_e360_d_n13: f64 = (p.p50 * s.dn[594][13]);
        let eq10_e360_d_n14: f64 = (p.p50 * s.dn[594][14]);
        let eq10_e360_d_n15: f64 = (p.p50 * s.dn[594][15]);
        let eq10_e360_d_n16: f64 = (p.p50 * s.dn[594][16]);
        let eq10_e360_d_n17: f64 = (p.p50 * s.dn[594][17]);
        let eq10_e360_d_n18: f64 = (p.p50 * s.dn[594][18]);
        let eq10_e360_d_b0: f64 = (p.p50 * s.db[594][0]);
        let eq10_e360_d_b1: f64 = (p.p50 * s.db[594][1]);
        let eq10_e360_d_b2: f64 = (p.p50 * s.db[594][2]);
        let eq10_e360_d_b3: f64 = (p.p50 * s.db[594][3]);
        let eq10_e360_d_b4: f64 = (p.p50 * s.db[594][4]);
        let eq10_e360_d_b5: f64 = (p.p50 * s.db[594][5]);
        let eq10_e360_d_b6: f64 = (p.p50 * s.db[594][6]);
        let eq10_e360_d_b7: f64 = (p.p50 * s.db[594][7]);
        let eq10_e360_d_b8: f64 = (p.p50 * s.db[594][8]);
        let eq10_e360_d_b9: f64 = (p.p50 * s.db[594][9]);
        let eq10_e360_d_b10: f64 = (p.p50 * s.db[594][10]);
        let eq10_e360_d_b11: f64 = (p.p50 * s.db[594][11]);
        let eq10_e360_q: f64 = (p.p50 * eq10_e359_q);
        let eq10_e360_q_d_n0: f64 = (p.p50 * s.dn[594][0]);
        let eq10_e360_q_d_n1: f64 = (p.p50 * s.dn[594][1]);
        let eq10_e360_q_d_n2: f64 = (p.p50 * s.dn[594][2]);
        let eq10_e360_q_d_n3: f64 = (p.p50 * s.dn[594][3]);
        let eq10_e360_q_d_n4: f64 = (p.p50 * s.dn[594][4]);
        let eq10_e360_q_d_n5: f64 = (p.p50 * s.dn[594][5]);
        let eq10_e360_q_d_n6: f64 = (p.p50 * s.dn[594][6]);
        let eq10_e360_q_d_n7: f64 = (p.p50 * s.dn[594][7]);
        let eq10_e360_q_d_n8: f64 = (p.p50 * s.dn[594][8]);
        let eq10_e360_q_d_n9: f64 = (p.p50 * s.dn[594][9]);
        let eq10_e360_q_d_n10: f64 = (p.p50 * s.dn[594][10]);
        let eq10_e360_q_d_n11: f64 = (p.p50 * s.dn[594][11]);
        let eq10_e360_q_d_n12: f64 = (p.p50 * s.dn[594][12]);
        let eq10_e360_q_d_n13: f64 = (p.p50 * s.dn[594][13]);
        let eq10_e360_q_d_n14: f64 = (p.p50 * s.dn[594][14]);
        let eq10_e360_q_d_n15: f64 = (p.p50 * s.dn[594][15]);
        let eq10_e360_q_d_n16: f64 = (p.p50 * s.dn[594][16]);
        let eq10_e360_q_d_n17: f64 = (p.p50 * s.dn[594][17]);
        let eq10_e360_q_d_n18: f64 = (p.p50 * s.dn[594][18]);
        let eq10_e360_q_d_b0: f64 = (p.p50 * s.db[594][0]);
        let eq10_e360_q_d_b1: f64 = (p.p50 * s.db[594][1]);
        let eq10_e360_q_d_b2: f64 = (p.p50 * s.db[594][2]);
        let eq10_e360_q_d_b3: f64 = (p.p50 * s.db[594][3]);
        let eq10_e360_q_d_b4: f64 = (p.p50 * s.db[594][4]);
        let eq10_e360_q_d_b5: f64 = (p.p50 * s.db[594][5]);
        let eq10_e360_q_d_b6: f64 = (p.p50 * s.db[594][6]);
        let eq10_e360_q_d_b7: f64 = (p.p50 * s.db[594][7]);
        let eq10_e360_q_d_b8: f64 = (p.p50 * s.db[594][8]);
        let eq10_e360_q_d_b9: f64 = (p.p50 * s.db[594][9]);
        let eq10_e360_q_d_b10: f64 = (p.p50 * s.db[594][10]);
        let eq10_e360_q_d_b11: f64 = (p.p50 * s.db[594][11]);
        let eq10_reactive_node_derivatives: [f64; 19] = [eq10_e360_q_d_n0, eq10_e360_q_d_n1, eq10_e360_q_d_n2, eq10_e360_q_d_n3, eq10_e360_q_d_n4, eq10_e360_q_d_n5, eq10_e360_q_d_n6, eq10_e360_q_d_n7, eq10_e360_q_d_n8, eq10_e360_q_d_n9, eq10_e360_q_d_n10, eq10_e360_q_d_n11, eq10_e360_q_d_n12, eq10_e360_q_d_n13, eq10_e360_q_d_n14, eq10_e360_q_d_n15, eq10_e360_q_d_n16, eq10_e360_q_d_n17, eq10_e360_q_d_n18];
        let eq10_reactive_branch_derivatives: [f64; 12] = [eq10_e360_q_d_b0, eq10_e360_q_d_b1, eq10_e360_q_d_b2, eq10_e360_q_d_b3, eq10_e360_q_d_b4, eq10_e360_q_d_b5, eq10_e360_q_d_b6, eq10_e360_q_d_b7, eq10_e360_q_d_b8, eq10_e360_q_d_b9, eq10_e360_q_d_b10, eq10_e360_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            &nodes,
            &eq10_reactive_node_derivatives,
            &branches,
            &eq10_reactive_branch_derivatives,
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
        let eq11_e363_q: f64 = s.v[198];
        let eq11_e364: f64 = (p.p50 * s.v[198]);
        let eq11_e364_d_n0: f64 = (p.p50 * s.dn[198][0]);
        let eq11_e364_d_n1: f64 = (p.p50 * s.dn[198][1]);
        let eq11_e364_d_n2: f64 = (p.p50 * s.dn[198][2]);
        let eq11_e364_d_n3: f64 = (p.p50 * s.dn[198][3]);
        let eq11_e364_d_n4: f64 = (p.p50 * s.dn[198][4]);
        let eq11_e364_d_n5: f64 = (p.p50 * s.dn[198][5]);
        let eq11_e364_d_n6: f64 = (p.p50 * s.dn[198][6]);
        let eq11_e364_d_n7: f64 = (p.p50 * s.dn[198][7]);
        let eq11_e364_d_n8: f64 = (p.p50 * s.dn[198][8]);
        let eq11_e364_d_n9: f64 = (p.p50 * s.dn[198][9]);
        let eq11_e364_d_n10: f64 = (p.p50 * s.dn[198][10]);
        let eq11_e364_d_n11: f64 = (p.p50 * s.dn[198][11]);
        let eq11_e364_d_n12: f64 = (p.p50 * s.dn[198][12]);
        let eq11_e364_d_n13: f64 = (p.p50 * s.dn[198][13]);
        let eq11_e364_d_n14: f64 = (p.p50 * s.dn[198][14]);
        let eq11_e364_d_n15: f64 = (p.p50 * s.dn[198][15]);
        let eq11_e364_d_n16: f64 = (p.p50 * s.dn[198][16]);
        let eq11_e364_d_n17: f64 = (p.p50 * s.dn[198][17]);
        let eq11_e364_d_n18: f64 = (p.p50 * s.dn[198][18]);
        let eq11_e364_d_b0: f64 = (p.p50 * s.db[198][0]);
        let eq11_e364_d_b1: f64 = (p.p50 * s.db[198][1]);
        let eq11_e364_d_b2: f64 = (p.p50 * s.db[198][2]);
        let eq11_e364_d_b3: f64 = (p.p50 * s.db[198][3]);
        let eq11_e364_d_b4: f64 = (p.p50 * s.db[198][4]);
        let eq11_e364_d_b5: f64 = (p.p50 * s.db[198][5]);
        let eq11_e364_d_b6: f64 = (p.p50 * s.db[198][6]);
        let eq11_e364_d_b7: f64 = (p.p50 * s.db[198][7]);
        let eq11_e364_d_b8: f64 = (p.p50 * s.db[198][8]);
        let eq11_e364_d_b9: f64 = (p.p50 * s.db[198][9]);
        let eq11_e364_d_b10: f64 = (p.p50 * s.db[198][10]);
        let eq11_e364_d_b11: f64 = (p.p50 * s.db[198][11]);
        let eq11_e364_q: f64 = (p.p50 * eq11_e363_q);
        let eq11_e364_q_d_n0: f64 = (p.p50 * s.dn[198][0]);
        let eq11_e364_q_d_n1: f64 = (p.p50 * s.dn[198][1]);
        let eq11_e364_q_d_n2: f64 = (p.p50 * s.dn[198][2]);
        let eq11_e364_q_d_n3: f64 = (p.p50 * s.dn[198][3]);
        let eq11_e364_q_d_n4: f64 = (p.p50 * s.dn[198][4]);
        let eq11_e364_q_d_n5: f64 = (p.p50 * s.dn[198][5]);
        let eq11_e364_q_d_n6: f64 = (p.p50 * s.dn[198][6]);
        let eq11_e364_q_d_n7: f64 = (p.p50 * s.dn[198][7]);
        let eq11_e364_q_d_n8: f64 = (p.p50 * s.dn[198][8]);
        let eq11_e364_q_d_n9: f64 = (p.p50 * s.dn[198][9]);
        let eq11_e364_q_d_n10: f64 = (p.p50 * s.dn[198][10]);
        let eq11_e364_q_d_n11: f64 = (p.p50 * s.dn[198][11]);
        let eq11_e364_q_d_n12: f64 = (p.p50 * s.dn[198][12]);
        let eq11_e364_q_d_n13: f64 = (p.p50 * s.dn[198][13]);
        let eq11_e364_q_d_n14: f64 = (p.p50 * s.dn[198][14]);
        let eq11_e364_q_d_n15: f64 = (p.p50 * s.dn[198][15]);
        let eq11_e364_q_d_n16: f64 = (p.p50 * s.dn[198][16]);
        let eq11_e364_q_d_n17: f64 = (p.p50 * s.dn[198][17]);
        let eq11_e364_q_d_n18: f64 = (p.p50 * s.dn[198][18]);
        let eq11_e364_q_d_b0: f64 = (p.p50 * s.db[198][0]);
        let eq11_e364_q_d_b1: f64 = (p.p50 * s.db[198][1]);
        let eq11_e364_q_d_b2: f64 = (p.p50 * s.db[198][2]);
        let eq11_e364_q_d_b3: f64 = (p.p50 * s.db[198][3]);
        let eq11_e364_q_d_b4: f64 = (p.p50 * s.db[198][4]);
        let eq11_e364_q_d_b5: f64 = (p.p50 * s.db[198][5]);
        let eq11_e364_q_d_b6: f64 = (p.p50 * s.db[198][6]);
        let eq11_e364_q_d_b7: f64 = (p.p50 * s.db[198][7]);
        let eq11_e364_q_d_b8: f64 = (p.p50 * s.db[198][8]);
        let eq11_e364_q_d_b9: f64 = (p.p50 * s.db[198][9]);
        let eq11_e364_q_d_b10: f64 = (p.p50 * s.db[198][10]);
        let eq11_e364_q_d_b11: f64 = (p.p50 * s.db[198][11]);
        let eq11_reactive_node_derivatives: [f64; 19] = [eq11_e364_q_d_n0, eq11_e364_q_d_n1, eq11_e364_q_d_n2, eq11_e364_q_d_n3, eq11_e364_q_d_n4, eq11_e364_q_d_n5, eq11_e364_q_d_n6, eq11_e364_q_d_n7, eq11_e364_q_d_n8, eq11_e364_q_d_n9, eq11_e364_q_d_n10, eq11_e364_q_d_n11, eq11_e364_q_d_n12, eq11_e364_q_d_n13, eq11_e364_q_d_n14, eq11_e364_q_d_n15, eq11_e364_q_d_n16, eq11_e364_q_d_n17, eq11_e364_q_d_n18];
        let eq11_reactive_branch_derivatives: [f64; 12] = [eq11_e364_q_d_b0, eq11_e364_q_d_b1, eq11_e364_q_d_b2, eq11_e364_q_d_b3, eq11_e364_q_d_b4, eq11_e364_q_d_b5, eq11_e364_q_d_b6, eq11_e364_q_d_b7, eq11_e364_q_d_b8, eq11_e364_q_d_b9, eq11_e364_q_d_b10, eq11_e364_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            &nodes,
            &eq11_reactive_node_derivatives,
            &branches,
            &eq11_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_12_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq12_e367_q: f64 = s.v[196];
        let eq12_e368: f64 = (p.p50 * s.v[196]);
        let eq12_e368_d_n0: f64 = (p.p50 * s.dn[196][0]);
        let eq12_e368_d_n1: f64 = (p.p50 * s.dn[196][1]);
        let eq12_e368_d_n2: f64 = (p.p50 * s.dn[196][2]);
        let eq12_e368_d_n3: f64 = (p.p50 * s.dn[196][3]);
        let eq12_e368_d_n4: f64 = (p.p50 * s.dn[196][4]);
        let eq12_e368_d_n5: f64 = (p.p50 * s.dn[196][5]);
        let eq12_e368_d_n6: f64 = (p.p50 * s.dn[196][6]);
        let eq12_e368_d_n7: f64 = (p.p50 * s.dn[196][7]);
        let eq12_e368_d_n8: f64 = (p.p50 * s.dn[196][8]);
        let eq12_e368_d_n9: f64 = (p.p50 * s.dn[196][9]);
        let eq12_e368_d_n10: f64 = (p.p50 * s.dn[196][10]);
        let eq12_e368_d_n11: f64 = (p.p50 * s.dn[196][11]);
        let eq12_e368_d_n12: f64 = (p.p50 * s.dn[196][12]);
        let eq12_e368_d_n13: f64 = (p.p50 * s.dn[196][13]);
        let eq12_e368_d_n14: f64 = (p.p50 * s.dn[196][14]);
        let eq12_e368_d_n15: f64 = (p.p50 * s.dn[196][15]);
        let eq12_e368_d_n16: f64 = (p.p50 * s.dn[196][16]);
        let eq12_e368_d_n17: f64 = (p.p50 * s.dn[196][17]);
        let eq12_e368_d_n18: f64 = (p.p50 * s.dn[196][18]);
        let eq12_e368_d_b0: f64 = (p.p50 * s.db[196][0]);
        let eq12_e368_d_b1: f64 = (p.p50 * s.db[196][1]);
        let eq12_e368_d_b2: f64 = (p.p50 * s.db[196][2]);
        let eq12_e368_d_b3: f64 = (p.p50 * s.db[196][3]);
        let eq12_e368_d_b4: f64 = (p.p50 * s.db[196][4]);
        let eq12_e368_d_b5: f64 = (p.p50 * s.db[196][5]);
        let eq12_e368_d_b6: f64 = (p.p50 * s.db[196][6]);
        let eq12_e368_d_b7: f64 = (p.p50 * s.db[196][7]);
        let eq12_e368_d_b8: f64 = (p.p50 * s.db[196][8]);
        let eq12_e368_d_b9: f64 = (p.p50 * s.db[196][9]);
        let eq12_e368_d_b10: f64 = (p.p50 * s.db[196][10]);
        let eq12_e368_d_b11: f64 = (p.p50 * s.db[196][11]);
        let eq12_e368_q: f64 = (p.p50 * eq12_e367_q);
        let eq12_e368_q_d_n0: f64 = (p.p50 * s.dn[196][0]);
        let eq12_e368_q_d_n1: f64 = (p.p50 * s.dn[196][1]);
        let eq12_e368_q_d_n2: f64 = (p.p50 * s.dn[196][2]);
        let eq12_e368_q_d_n3: f64 = (p.p50 * s.dn[196][3]);
        let eq12_e368_q_d_n4: f64 = (p.p50 * s.dn[196][4]);
        let eq12_e368_q_d_n5: f64 = (p.p50 * s.dn[196][5]);
        let eq12_e368_q_d_n6: f64 = (p.p50 * s.dn[196][6]);
        let eq12_e368_q_d_n7: f64 = (p.p50 * s.dn[196][7]);
        let eq12_e368_q_d_n8: f64 = (p.p50 * s.dn[196][8]);
        let eq12_e368_q_d_n9: f64 = (p.p50 * s.dn[196][9]);
        let eq12_e368_q_d_n10: f64 = (p.p50 * s.dn[196][10]);
        let eq12_e368_q_d_n11: f64 = (p.p50 * s.dn[196][11]);
        let eq12_e368_q_d_n12: f64 = (p.p50 * s.dn[196][12]);
        let eq12_e368_q_d_n13: f64 = (p.p50 * s.dn[196][13]);
        let eq12_e368_q_d_n14: f64 = (p.p50 * s.dn[196][14]);
        let eq12_e368_q_d_n15: f64 = (p.p50 * s.dn[196][15]);
        let eq12_e368_q_d_n16: f64 = (p.p50 * s.dn[196][16]);
        let eq12_e368_q_d_n17: f64 = (p.p50 * s.dn[196][17]);
        let eq12_e368_q_d_n18: f64 = (p.p50 * s.dn[196][18]);
        let eq12_e368_q_d_b0: f64 = (p.p50 * s.db[196][0]);
        let eq12_e368_q_d_b1: f64 = (p.p50 * s.db[196][1]);
        let eq12_e368_q_d_b2: f64 = (p.p50 * s.db[196][2]);
        let eq12_e368_q_d_b3: f64 = (p.p50 * s.db[196][3]);
        let eq12_e368_q_d_b4: f64 = (p.p50 * s.db[196][4]);
        let eq12_e368_q_d_b5: f64 = (p.p50 * s.db[196][5]);
        let eq12_e368_q_d_b6: f64 = (p.p50 * s.db[196][6]);
        let eq12_e368_q_d_b7: f64 = (p.p50 * s.db[196][7]);
        let eq12_e368_q_d_b8: f64 = (p.p50 * s.db[196][8]);
        let eq12_e368_q_d_b9: f64 = (p.p50 * s.db[196][9]);
        let eq12_e368_q_d_b10: f64 = (p.p50 * s.db[196][10]);
        let eq12_e368_q_d_b11: f64 = (p.p50 * s.db[196][11]);
        let eq12_reactive_node_derivatives: [f64; 19] = [eq12_e368_q_d_n0, eq12_e368_q_d_n1, eq12_e368_q_d_n2, eq12_e368_q_d_n3, eq12_e368_q_d_n4, eq12_e368_q_d_n5, eq12_e368_q_d_n6, eq12_e368_q_d_n7, eq12_e368_q_d_n8, eq12_e368_q_d_n9, eq12_e368_q_d_n10, eq12_e368_q_d_n11, eq12_e368_q_d_n12, eq12_e368_q_d_n13, eq12_e368_q_d_n14, eq12_e368_q_d_n15, eq12_e368_q_d_n16, eq12_e368_q_d_n17, eq12_e368_q_d_n18];
        let eq12_reactive_branch_derivatives: [f64; 12] = [eq12_e368_q_d_b0, eq12_e368_q_d_b1, eq12_e368_q_d_b2, eq12_e368_q_d_b3, eq12_e368_q_d_b4, eq12_e368_q_d_b5, eq12_e368_q_d_b6, eq12_e368_q_d_b7, eq12_e368_q_d_b8, eq12_e368_q_d_b9, eq12_e368_q_d_b10, eq12_e368_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            &nodes,
            &eq12_reactive_node_derivatives,
            &branches,
            &eq12_reactive_branch_derivatives,
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
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq18_e397: f64 = ((nv14 - 0.0) * s.v[617]);
        let eq18_e397_d_n0: f64 = ((nv14 - 0.0) * s.dn[617][0]);
        let eq18_e397_d_n1: f64 = ((nv14 - 0.0) * s.dn[617][1]);
        let eq18_e397_d_n2: f64 = ((nv14 - 0.0) * s.dn[617][2]);
        let eq18_e397_d_n3: f64 = ((nv14 - 0.0) * s.dn[617][3]);
        let eq18_e397_d_n4: f64 = ((nv14 - 0.0) * s.dn[617][4]);
        let eq18_e397_d_n5: f64 = ((nv14 - 0.0) * s.dn[617][5]);
        let eq18_e397_d_n6: f64 = ((nv14 - 0.0) * s.dn[617][6]);
        let eq18_e397_d_n7: f64 = ((nv14 - 0.0) * s.dn[617][7]);
        let eq18_e397_d_n8: f64 = ((nv14 - 0.0) * s.dn[617][8]);
        let eq18_e397_d_n9: f64 = ((nv14 - 0.0) * s.dn[617][9]);
        let eq18_e397_d_n10: f64 = ((nv14 - 0.0) * s.dn[617][10]);
        let eq18_e397_d_n11: f64 = ((nv14 - 0.0) * s.dn[617][11]);
        let eq18_e397_d_n12: f64 = ((nv14 - 0.0) * s.dn[617][12]);
        let eq18_e397_d_n13: f64 = ((nv14 - 0.0) * s.dn[617][13]);
        let eq18_e397_d_n14: f64 = (s.v[617] + ((nv14 - 0.0) * s.dn[617][14]));
        let eq18_e397_d_n15: f64 = ((nv14 - 0.0) * s.dn[617][15]);
        let eq18_e397_d_n16: f64 = ((nv14 - 0.0) * s.dn[617][16]);
        let eq18_e397_d_n17: f64 = ((nv14 - 0.0) * s.dn[617][17]);
        let eq18_e397_d_n18: f64 = ((nv14 - 0.0) * s.dn[617][18]);
        let eq18_e397_d_b0: f64 = ((nv14 - 0.0) * s.db[617][0]);
        let eq18_e397_d_b1: f64 = ((nv14 - 0.0) * s.db[617][1]);
        let eq18_e397_d_b2: f64 = ((nv14 - 0.0) * s.db[617][2]);
        let eq18_e397_d_b3: f64 = ((nv14 - 0.0) * s.db[617][3]);
        let eq18_e397_d_b4: f64 = ((nv14 - 0.0) * s.db[617][4]);
        let eq18_e397_d_b5: f64 = ((nv14 - 0.0) * s.db[617][5]);
        let eq18_e397_d_b6: f64 = ((nv14 - 0.0) * s.db[617][6]);
        let eq18_e397_d_b7: f64 = ((nv14 - 0.0) * s.db[617][7]);
        let eq18_e397_d_b8: f64 = ((nv14 - 0.0) * s.db[617][8]);
        let eq18_e397_d_b9: f64 = ((nv14 - 0.0) * s.db[617][9]);
        let eq18_e397_d_b10: f64 = ((nv14 - 0.0) * s.db[617][10]);
        let eq18_e397_d_b11: f64 = ((nv14 - 0.0) * s.db[617][11]);
        let eq18_e398_q: f64 = eq18_e397;
        let eq18_reactive_node_derivatives: [f64; 19] = [eq18_e397_d_n0, eq18_e397_d_n1, eq18_e397_d_n2, eq18_e397_d_n3, eq18_e397_d_n4, eq18_e397_d_n5, eq18_e397_d_n6, eq18_e397_d_n7, eq18_e397_d_n8, eq18_e397_d_n9, eq18_e397_d_n10, eq18_e397_d_n11, eq18_e397_d_n12, eq18_e397_d_n13, eq18_e397_d_n14, eq18_e397_d_n15, eq18_e397_d_n16, eq18_e397_d_n17, eq18_e397_d_n18];
        let eq18_reactive_branch_derivatives: [f64; 12] = [eq18_e397_d_b0, eq18_e397_d_b1, eq18_e397_d_b2, eq18_e397_d_b3, eq18_e397_d_b4, eq18_e397_d_b5, eq18_e397_d_b6, eq18_e397_d_b7, eq18_e397_d_b8, eq18_e397_d_b9, eq18_e397_d_b10, eq18_e397_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
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
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq19_e401: f64 = ((nv14 - 0.0) * s.v[618]);
        let eq19_e401_d_n0: f64 = ((nv14 - 0.0) * s.dn[618][0]);
        let eq19_e401_d_n1: f64 = ((nv14 - 0.0) * s.dn[618][1]);
        let eq19_e401_d_n2: f64 = ((nv14 - 0.0) * s.dn[618][2]);
        let eq19_e401_d_n3: f64 = ((nv14 - 0.0) * s.dn[618][3]);
        let eq19_e401_d_n4: f64 = ((nv14 - 0.0) * s.dn[618][4]);
        let eq19_e401_d_n5: f64 = ((nv14 - 0.0) * s.dn[618][5]);
        let eq19_e401_d_n6: f64 = ((nv14 - 0.0) * s.dn[618][6]);
        let eq19_e401_d_n7: f64 = ((nv14 - 0.0) * s.dn[618][7]);
        let eq19_e401_d_n8: f64 = ((nv14 - 0.0) * s.dn[618][8]);
        let eq19_e401_d_n9: f64 = ((nv14 - 0.0) * s.dn[618][9]);
        let eq19_e401_d_n10: f64 = ((nv14 - 0.0) * s.dn[618][10]);
        let eq19_e401_d_n11: f64 = ((nv14 - 0.0) * s.dn[618][11]);
        let eq19_e401_d_n12: f64 = ((nv14 - 0.0) * s.dn[618][12]);
        let eq19_e401_d_n13: f64 = ((nv14 - 0.0) * s.dn[618][13]);
        let eq19_e401_d_n14: f64 = (s.v[618] + ((nv14 - 0.0) * s.dn[618][14]));
        let eq19_e401_d_n15: f64 = ((nv14 - 0.0) * s.dn[618][15]);
        let eq19_e401_d_n16: f64 = ((nv14 - 0.0) * s.dn[618][16]);
        let eq19_e401_d_n17: f64 = ((nv14 - 0.0) * s.dn[618][17]);
        let eq19_e401_d_n18: f64 = ((nv14 - 0.0) * s.dn[618][18]);
        let eq19_e401_d_b0: f64 = ((nv14 - 0.0) * s.db[618][0]);
        let eq19_e401_d_b1: f64 = ((nv14 - 0.0) * s.db[618][1]);
        let eq19_e401_d_b2: f64 = ((nv14 - 0.0) * s.db[618][2]);
        let eq19_e401_d_b3: f64 = ((nv14 - 0.0) * s.db[618][3]);
        let eq19_e401_d_b4: f64 = ((nv14 - 0.0) * s.db[618][4]);
        let eq19_e401_d_b5: f64 = ((nv14 - 0.0) * s.db[618][5]);
        let eq19_e401_d_b6: f64 = ((nv14 - 0.0) * s.db[618][6]);
        let eq19_e401_d_b7: f64 = ((nv14 - 0.0) * s.db[618][7]);
        let eq19_e401_d_b8: f64 = ((nv14 - 0.0) * s.db[618][8]);
        let eq19_e401_d_b9: f64 = ((nv14 - 0.0) * s.db[618][9]);
        let eq19_e401_d_b10: f64 = ((nv14 - 0.0) * s.db[618][10]);
        let eq19_e401_d_b11: f64 = ((nv14 - 0.0) * s.db[618][11]);
        let eq19_e402_q: f64 = eq19_e401;
        let eq19_reactive_node_derivatives: [f64; 19] = [eq19_e401_d_n0, eq19_e401_d_n1, eq19_e401_d_n2, eq19_e401_d_n3, eq19_e401_d_n4, eq19_e401_d_n5, eq19_e401_d_n6, eq19_e401_d_n7, eq19_e401_d_n8, eq19_e401_d_n9, eq19_e401_d_n10, eq19_e401_d_n11, eq19_e401_d_n12, eq19_e401_d_n13, eq19_e401_d_n14, eq19_e401_d_n15, eq19_e401_d_n16, eq19_e401_d_n17, eq19_e401_d_n18];
        let eq19_reactive_branch_derivatives: [f64; 12] = [eq19_e401_d_b0, eq19_e401_d_b1, eq19_e401_d_b2, eq19_e401_d_b3, eq19_e401_d_b4, eq19_e401_d_b5, eq19_e401_d_b6, eq19_e401_d_b7, eq19_e401_d_b8, eq19_e401_d_b9, eq19_e401_d_b10, eq19_e401_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            &nodes,
            &eq19_reactive_node_derivatives,
            &branches,
            &eq19_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
