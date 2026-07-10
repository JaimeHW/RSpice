#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_12(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);let nv12 = ctx.node_voltage(nodes[12]);let nv13 = ctx.node_voltage(nodes[13]);let nv18 = ctx.node_voltage(nodes[18]);
        let (eq40_e566, eq40_e566_d_n0, eq40_e566_d_n1, eq40_e566_d_n2, eq40_e566_d_n3, eq40_e566_d_n4, eq40_e566_d_n5, eq40_e566_d_n6, eq40_e566_d_n7, eq40_e566_d_n8, eq40_e566_d_n9, eq40_e566_d_n10, eq40_e566_d_n11, eq40_e566_d_n12, eq40_e566_d_n13, eq40_e566_d_n14, eq40_e566_d_n15, eq40_e566_d_n16, eq40_e566_d_n17, eq40_e566_d_n18, eq40_e566_d_b0, eq40_e566_d_b1, eq40_e566_d_b2, eq40_e566_d_b3, eq40_e566_d_b4, eq40_e566_d_b5, eq40_e566_d_b6, eq40_e566_d_b7, eq40_e566_d_b8, eq40_e566_d_b9, eq40_e566_d_b10, eq40_e566_d_b11, eq40_e566_d_b12, eq40_e566_d_b13, eq40_e566_d_b14, eq40_e566_d_b15,) = {
    if (s.b[1851] && (p.p262 != 0.0)) {
        let eq40_e564: f64 = (s.v[552] * (nv8 - nv12));let eq40_e564_d_n0: f64 = (s.dn[552][0] * (nv8 - nv12));let eq40_e564_d_n1: f64 = (s.dn[552][1] * (nv8 - nv12));let eq40_e564_d_n2: f64 = (s.dn[552][2] * (nv8 - nv12));let eq40_e564_d_n3: f64 = (s.dn[552][3] * (nv8 - nv12));let eq40_e564_d_n4: f64 = (s.dn[552][4] * (nv8 - nv12));let eq40_e564_d_n5: f64 = (s.dn[552][5] * (nv8 - nv12));let eq40_e564_d_n6: f64 = (s.dn[552][6] * (nv8 - nv12));let eq40_e564_d_n7: f64 = (s.dn[552][7] * (nv8 - nv12));let eq40_e564_d_n8: f64 = ((s.dn[552][8] * (nv8 - nv12)) + s.v[552]);let eq40_e564_d_n9: f64 = (s.dn[552][9] * (nv8 - nv12));let eq40_e564_d_n10: f64 = (s.dn[552][10] * (nv8 - nv12));let eq40_e564_d_n11: f64 = (s.dn[552][11] * (nv8 - nv12));let eq40_e564_d_n12: f64 = ((s.dn[552][12] * (nv8 - nv12)) + (-s.v[552]));let eq40_e564_d_n13: f64 = (s.dn[552][13] * (nv8 - nv12));let eq40_e564_d_n14: f64 = (s.dn[552][14] * (nv8 - nv12));let eq40_e564_d_n15: f64 = (s.dn[552][15] * (nv8 - nv12));let eq40_e564_d_n16: f64 = (s.dn[552][16] * (nv8 - nv12));let eq40_e564_d_n17: f64 = (s.dn[552][17] * (nv8 - nv12));let eq40_e564_d_n18: f64 = (s.dn[552][18] * (nv8 - nv12));let eq40_e564_d_b0: f64 = (s.db[552][0] * (nv8 - nv12));let eq40_e564_d_b1: f64 = (s.db[552][1] * (nv8 - nv12));let eq40_e564_d_b2: f64 = (s.db[552][2] * (nv8 - nv12));let eq40_e564_d_b3: f64 = (s.db[552][3] * (nv8 - nv12));let eq40_e564_d_b4: f64 = (s.db[552][4] * (nv8 - nv12));let eq40_e564_d_b5: f64 = (s.db[552][5] * (nv8 - nv12));let eq40_e564_d_b6: f64 = (s.db[552][6] * (nv8 - nv12));let eq40_e564_d_b7: f64 = (s.db[552][7] * (nv8 - nv12));let eq40_e564_d_b8: f64 = (s.db[552][8] * (nv8 - nv12));let eq40_e564_d_b9: f64 = (s.db[552][9] * (nv8 - nv12));let eq40_e564_d_b10: f64 = (s.db[552][10] * (nv8 - nv12));let eq40_e564_d_b11: f64 = (s.db[552][11] * (nv8 - nv12));let eq40_e564_d_b12: f64 = (s.db[552][12] * (nv8 - nv12));let eq40_e564_d_b13: f64 = (s.db[552][13] * (nv8 - nv12));let eq40_e564_d_b14: f64 = (s.db[552][14] * (nv8 - nv12));let eq40_e564_d_b15: f64 = (s.db[552][15] * (nv8 - nv12));
        (eq40_e564, eq40_e564_d_n0, eq40_e564_d_n1, eq40_e564_d_n2, eq40_e564_d_n3, eq40_e564_d_n4, eq40_e564_d_n5, eq40_e564_d_n6, eq40_e564_d_n7, eq40_e564_d_n8, eq40_e564_d_n9, eq40_e564_d_n10, eq40_e564_d_n11, eq40_e564_d_n12, eq40_e564_d_n13, eq40_e564_d_n14, eq40_e564_d_n15, eq40_e564_d_n16, eq40_e564_d_n17, eq40_e564_d_n18, eq40_e564_d_b0, eq40_e564_d_b1, eq40_e564_d_b2, eq40_e564_d_b3, eq40_e564_d_b4, eq40_e564_d_b5, eq40_e564_d_b6, eq40_e564_d_b7, eq40_e564_d_b8, eq40_e564_d_b9, eq40_e564_d_b10, eq40_e564_d_b11, eq40_e564_d_b12, eq40_e564_d_b13, eq40_e564_d_b14, eq40_e564_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_value: f64 = eq40_e566;let eq40_node_derivatives: [f64; 19] = [eq40_e566_d_n0, eq40_e566_d_n1, eq40_e566_d_n2, eq40_e566_d_n3, eq40_e566_d_n4, eq40_e566_d_n5, eq40_e566_d_n6, eq40_e566_d_n7, eq40_e566_d_n8, eq40_e566_d_n9, eq40_e566_d_n10, eq40_e566_d_n11, eq40_e566_d_n12, eq40_e566_d_n13, eq40_e566_d_n14, eq40_e566_d_n15, eq40_e566_d_n16, eq40_e566_d_n17, eq40_e566_d_n18];let eq40_branch_derivatives: [f64; 16] = [eq40_e566_d_b0, eq40_e566_d_b1, eq40_e566_d_b2, eq40_e566_d_b3, eq40_e566_d_b4, eq40_e566_d_b5, eq40_e566_d_b6, eq40_e566_d_b7, eq40_e566_d_b8, eq40_e566_d_b9, eq40_e566_d_b10, eq40_e566_d_b11, eq40_e566_d_b12, eq40_e566_d_b13, eq40_e566_d_b14, eq40_e566_d_b15];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(12),
            multiplicity * (eq40_value),
            &eq40_node_derivatives,
            &eq40_branch_derivatives,
            multiplicity,
        );
        let (eq41_e573,) = {
    if (s.b[1851] && (p.p262 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq41_value: f64 = eq41_e573;
        stamper.stamp_potential_const_local(
            7,
            eq41_value,
        );
        let (eq42_e580,) = {
    if (s.b[1851] && (p.p262 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq42_value: f64 = eq42_e580;
        stamper.stamp_potential_const_local(
            8,
            eq42_value,
        );
        let (eq43_e586, eq43_e586_d_n0, eq43_e586_d_n1, eq43_e586_d_n2, eq43_e586_d_n3, eq43_e586_d_n4, eq43_e586_d_n5, eq43_e586_d_n6, eq43_e586_d_n7, eq43_e586_d_n8, eq43_e586_d_n9, eq43_e586_d_n10, eq43_e586_d_n11, eq43_e586_d_n12, eq43_e586_d_n13, eq43_e586_d_n14, eq43_e586_d_n15, eq43_e586_d_n16, eq43_e586_d_n17, eq43_e586_d_n18, eq43_e586_d_b0, eq43_e586_d_b1, eq43_e586_d_b2, eq43_e586_d_b3, eq43_e586_d_b4, eq43_e586_d_b5, eq43_e586_d_b6, eq43_e586_d_b7, eq43_e586_d_b8, eq43_e586_d_b9, eq43_e586_d_b10, eq43_e586_d_b11, eq43_e586_d_b12, eq43_e586_d_b13, eq43_e586_d_b14, eq43_e586_d_b15,) = {
    if (s.b[1851] && (p.p34 != 0.0)) {
        (s.v[582], s.dn[582][0], s.dn[582][1], s.dn[582][2], s.dn[582][3], s.dn[582][4], s.dn[582][5], s.dn[582][6], s.dn[582][7], s.dn[582][8], s.dn[582][9], s.dn[582][10], s.dn[582][11], s.dn[582][12], s.dn[582][13], s.dn[582][14], s.dn[582][15], s.dn[582][16], s.dn[582][17], s.dn[582][18], s.db[582][0], s.db[582][1], s.db[582][2], s.db[582][3], s.db[582][4], s.db[582][5], s.db[582][6], s.db[582][7], s.db[582][8], s.db[582][9], s.db[582][10], s.db[582][11], s.db[582][12], s.db[582][13], s.db[582][14], s.db[582][15],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq43_value: f64 = eq43_e586;let eq43_node_derivatives: [f64; 19] = [eq43_e586_d_n0, eq43_e586_d_n1, eq43_e586_d_n2, eq43_e586_d_n3, eq43_e586_d_n4, eq43_e586_d_n5, eq43_e586_d_n6, eq43_e586_d_n7, eq43_e586_d_n8, eq43_e586_d_n9, eq43_e586_d_n10, eq43_e586_d_n11, eq43_e586_d_n12, eq43_e586_d_n13, eq43_e586_d_n14, eq43_e586_d_n15, eq43_e586_d_n16, eq43_e586_d_n17, eq43_e586_d_n18];let eq43_branch_derivatives: [f64; 16] = [eq43_e586_d_b0, eq43_e586_d_b1, eq43_e586_d_b2, eq43_e586_d_b3, eq43_e586_d_b4, eq43_e586_d_b5, eq43_e586_d_b6, eq43_e586_d_b7, eq43_e586_d_b8, eq43_e586_d_b9, eq43_e586_d_b10, eq43_e586_d_b11, eq43_e586_d_b12, eq43_e586_d_b13, eq43_e586_d_b14, eq43_e586_d_b15];
        stamper.stamp_current_dense_local(
            Some(18),
            None,
            multiplicity * (eq43_value),
            &eq43_node_derivatives,
            &eq43_branch_derivatives,
            multiplicity,
        );
        let (eq44_e592, eq44_e592_d_n0, eq44_e592_d_n1, eq44_e592_d_n2, eq44_e592_d_n3, eq44_e592_d_n4, eq44_e592_d_n5, eq44_e592_d_n6, eq44_e592_d_n7, eq44_e592_d_n8, eq44_e592_d_n9, eq44_e592_d_n10, eq44_e592_d_n11, eq44_e592_d_n12, eq44_e592_d_n13, eq44_e592_d_n14, eq44_e592_d_n15, eq44_e592_d_n16, eq44_e592_d_n17, eq44_e592_d_n18, eq44_e592_d_b0, eq44_e592_d_b1, eq44_e592_d_b2, eq44_e592_d_b3, eq44_e592_d_b4, eq44_e592_d_b5, eq44_e592_d_b6, eq44_e592_d_b7, eq44_e592_d_b8, eq44_e592_d_b9, eq44_e592_d_b10, eq44_e592_d_b11, eq44_e592_d_b12, eq44_e592_d_b13, eq44_e592_d_b14, eq44_e592_d_b15,) = {
    if (s.b[1851] && (p.p34 != 0.0)) {
        (s.v[583], s.dn[583][0], s.dn[583][1], s.dn[583][2], s.dn[583][3], s.dn[583][4], s.dn[583][5], s.dn[583][6], s.dn[583][7], s.dn[583][8], s.dn[583][9], s.dn[583][10], s.dn[583][11], s.dn[583][12], s.dn[583][13], s.dn[583][14], s.dn[583][15], s.dn[583][16], s.dn[583][17], s.dn[583][18], s.db[583][0], s.db[583][1], s.db[583][2], s.db[583][3], s.db[583][4], s.db[583][5], s.db[583][6], s.db[583][7], s.db[583][8], s.db[583][9], s.db[583][10], s.db[583][11], s.db[583][12], s.db[583][13], s.db[583][14], s.db[583][15],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e592;let eq44_node_derivatives: [f64; 19] = [eq44_e592_d_n0, eq44_e592_d_n1, eq44_e592_d_n2, eq44_e592_d_n3, eq44_e592_d_n4, eq44_e592_d_n5, eq44_e592_d_n6, eq44_e592_d_n7, eq44_e592_d_n8, eq44_e592_d_n9, eq44_e592_d_n10, eq44_e592_d_n11, eq44_e592_d_n12, eq44_e592_d_n13, eq44_e592_d_n14, eq44_e592_d_n15, eq44_e592_d_n16, eq44_e592_d_n17, eq44_e592_d_n18];let eq44_branch_derivatives: [f64; 16] = [eq44_e592_d_b0, eq44_e592_d_b1, eq44_e592_d_b2, eq44_e592_d_b3, eq44_e592_d_b4, eq44_e592_d_b5, eq44_e592_d_b6, eq44_e592_d_b7, eq44_e592_d_b8, eq44_e592_d_b9, eq44_e592_d_b10, eq44_e592_d_b11, eq44_e592_d_b12, eq44_e592_d_b13, eq44_e592_d_b14, eq44_e592_d_b15];
        stamper.stamp_current_dense_local(
            Some(13),
            None,
            multiplicity * (eq44_value),
            &eq44_node_derivatives,
            &eq44_branch_derivatives,
            multiplicity,
        );
        let (eq45_e600, eq45_e600_d_n18,) = {
    if (s.b[1851] && (p.p34 != 0.0)) {
        let eq45_e598: f64 = ((nv18 - 0.0) * 1e-12);
        (eq45_e598, 1e-12,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq45_value: f64 = eq45_e600;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (eq45_value),
            18,
            multiplicity * (eq45_e600_d_n18),
        );
        let (eq46_e608, eq46_e608_d_n13,) = {
    if (s.b[1851] && (p.p34 != 0.0)) {
        let eq46_e606: f64 = ((nv13 - 0.0) * 1e-12);
        (eq46_e606, 1e-12,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e608;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq46_value),
            13,
            multiplicity * (eq46_e608_d_n13),
        );
        let (eq47_e619, eq47_e619_d_n18,) = {
    if (s.b[1851] && (p.p34 != 0.0)) {
        let eq47_e614: f64 = (1e-9 / 0.0001);let eq47_e616: f64 = (eq47_e614 * (nv18 - 0.0));let eq47_e617: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq47_e616);
        (eq47_e617, (eq47_e614 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e619;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (eq47_value),
            18,
            multiplicity * (eq47_e619_d_n18),
        );
        let (eq48_e630, eq48_e630_d_n13,) = {
    if (s.b[1851] && (p.p34 != 0.0)) {
        let eq48_e625: f64 = (1e-9 / 0.0001);let eq48_e627: f64 = (eq48_e625 * (nv13 - 0.0));let eq48_e628: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq48_e627);
        (eq48_e628, (eq48_e625 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq48_value: f64 = eq48_e630;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq48_value),
            13,
            multiplicity * (eq48_e630_d_n13),
        );
        let (eq49_e637,) = {
    if (s.b[1851] && (p.p34 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e637;
        stamper.stamp_potential_const_local(
            9,
            eq49_value,
        );
        let (eq50_e644,) = {
    if (s.b[1851] && (p.p34 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq50_value: f64 = eq50_e644;
        stamper.stamp_potential_const_local(
            10,
            eq50_value,
        );
        let (eq51_e650, eq51_e650_d_n0, eq51_e650_d_n1, eq51_e650_d_n2, eq51_e650_d_n3, eq51_e650_d_n4, eq51_e650_d_n5, eq51_e650_d_n6, eq51_e650_d_n7, eq51_e650_d_n8, eq51_e650_d_n9, eq51_e650_d_n10, eq51_e650_d_n11, eq51_e650_d_n12, eq51_e650_d_n13, eq51_e650_d_n14, eq51_e650_d_n15, eq51_e650_d_n16, eq51_e650_d_n17, eq51_e650_d_n18, eq51_e650_d_b0, eq51_e650_d_b1, eq51_e650_d_b2, eq51_e650_d_b3, eq51_e650_d_b4, eq51_e650_d_b5, eq51_e650_d_b6, eq51_e650_d_b7, eq51_e650_d_b8, eq51_e650_d_b9, eq51_e650_d_b10, eq51_e650_d_b11, eq51_e650_d_b12, eq51_e650_d_b13, eq51_e650_d_b14, eq51_e650_d_b15,) = {
    if (s.b[1851] && s.b[1852]) {
        (s.v[592], s.dn[592][0], s.dn[592][1], s.dn[592][2], s.dn[592][3], s.dn[592][4], s.dn[592][5], s.dn[592][6], s.dn[592][7], s.dn[592][8], s.dn[592][9], s.dn[592][10], s.dn[592][11], s.dn[592][12], s.dn[592][13], s.dn[592][14], s.dn[592][15], s.dn[592][16], s.dn[592][17], s.dn[592][18], s.db[592][0], s.db[592][1], s.db[592][2], s.db[592][3], s.db[592][4], s.db[592][5], s.db[592][6], s.db[592][7], s.db[592][8], s.db[592][9], s.db[592][10], s.db[592][11], s.db[592][12], s.db[592][13], s.db[592][14], s.db[592][15],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e650;let eq51_node_derivatives: [f64; 19] = [eq51_e650_d_n0, eq51_e650_d_n1, eq51_e650_d_n2, eq51_e650_d_n3, eq51_e650_d_n4, eq51_e650_d_n5, eq51_e650_d_n6, eq51_e650_d_n7, eq51_e650_d_n8, eq51_e650_d_n9, eq51_e650_d_n10, eq51_e650_d_n11, eq51_e650_d_n12, eq51_e650_d_n13, eq51_e650_d_n14, eq51_e650_d_n15, eq51_e650_d_n16, eq51_e650_d_n17, eq51_e650_d_n18];let eq51_branch_derivatives: [f64; 16] = [eq51_e650_d_b0, eq51_e650_d_b1, eq51_e650_d_b2, eq51_e650_d_b3, eq51_e650_d_b4, eq51_e650_d_b5, eq51_e650_d_b6, eq51_e650_d_b7, eq51_e650_d_b8, eq51_e650_d_b9, eq51_e650_d_b10, eq51_e650_d_b11, eq51_e650_d_b12, eq51_e650_d_b13, eq51_e650_d_b14, eq51_e650_d_b15];
        stamper.stamp_current_dense_local(
            Some(17),
            None,
            multiplicity * (eq51_value),
            &eq51_node_derivatives,
            &eq51_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_13(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq52_e658, eq52_e658_d_n17,) = {
    if (s.b[1851] && s.b[1852]) {
        let eq52_e656: f64 = ((nv17 - 0.0) * 1e-12);
        (eq52_e656, 1e-12,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e658;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq52_value),
            17,
            multiplicity * (eq52_e658_d_n17),
        );
        let (eq53_e669, eq53_e669_d_n17,) = {
    if (s.b[1851] && s.b[1852]) {
        let eq53_e664: f64 = (1e-9 / 0.0001);let eq53_e666: f64 = (eq53_e664 * (nv17 - 0.0));let eq53_e667: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq53_e666);
        (eq53_e667, (eq53_e664 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e669;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq53_value),
            17,
            multiplicity * (eq53_e669_d_n17),
        );
        let (eq54_e676,) = {
    if (s.b[1851] && (!s.b[1852])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq54_value: f64 = eq54_e676;
        stamper.stamp_potential_const_local(
            11,
            eq54_value,
        );
        let (eq55_e685, eq55_e685_d_n0, eq55_e685_d_n1, eq55_e685_d_n2, eq55_e685_d_n3, eq55_e685_d_n4, eq55_e685_d_n5, eq55_e685_d_n6, eq55_e685_d_n7, eq55_e685_d_n8, eq55_e685_d_n9, eq55_e685_d_n10, eq55_e685_d_n11, eq55_e685_d_n12, eq55_e685_d_n13, eq55_e685_d_n14, eq55_e685_d_n15, eq55_e685_d_n16, eq55_e685_d_n17, eq55_e685_d_n18, eq55_e685_d_b0, eq55_e685_d_b1, eq55_e685_d_b2, eq55_e685_d_b3, eq55_e685_d_b4, eq55_e685_d_b5, eq55_e685_d_b6, eq55_e685_d_b7, eq55_e685_d_b8, eq55_e685_d_b9, eq55_e685_d_b10, eq55_e685_d_b11, eq55_e685_d_b12, eq55_e685_d_b13, eq55_e685_d_b14, eq55_e685_d_b15,) = {
    if (!s.b[1851]) {
        let eq55_e682: f64 = (s.v[311] + s.v[263]);let eq55_e682_d_n0: f64 = (s.dn[311][0] + s.dn[263][0]);let eq55_e682_d_n1: f64 = (s.dn[311][1] + s.dn[263][1]);let eq55_e682_d_n2: f64 = (s.dn[311][2] + s.dn[263][2]);let eq55_e682_d_n3: f64 = (s.dn[311][3] + s.dn[263][3]);let eq55_e682_d_n4: f64 = (s.dn[311][4] + s.dn[263][4]);let eq55_e682_d_n5: f64 = (s.dn[311][5] + s.dn[263][5]);let eq55_e682_d_n6: f64 = (s.dn[311][6] + s.dn[263][6]);let eq55_e682_d_n7: f64 = (s.dn[311][7] + s.dn[263][7]);let eq55_e682_d_n8: f64 = (s.dn[311][8] + s.dn[263][8]);let eq55_e682_d_n9: f64 = (s.dn[311][9] + s.dn[263][9]);let eq55_e682_d_n10: f64 = (s.dn[311][10] + s.dn[263][10]);let eq55_e682_d_n11: f64 = (s.dn[311][11] + s.dn[263][11]);let eq55_e682_d_n12: f64 = (s.dn[311][12] + s.dn[263][12]);let eq55_e682_d_n13: f64 = (s.dn[311][13] + s.dn[263][13]);let eq55_e682_d_n14: f64 = (s.dn[311][14] + s.dn[263][14]);let eq55_e682_d_n15: f64 = (s.dn[311][15] + s.dn[263][15]);let eq55_e682_d_n16: f64 = (s.dn[311][16] + s.dn[263][16]);let eq55_e682_d_n17: f64 = (s.dn[311][17] + s.dn[263][17]);let eq55_e682_d_n18: f64 = (s.dn[311][18] + s.dn[263][18]);let eq55_e682_d_b0: f64 = (s.db[311][0] + s.db[263][0]);let eq55_e682_d_b1: f64 = (s.db[311][1] + s.db[263][1]);let eq55_e682_d_b2: f64 = (s.db[311][2] + s.db[263][2]);let eq55_e682_d_b3: f64 = (s.db[311][3] + s.db[263][3]);let eq55_e682_d_b4: f64 = (s.db[311][4] + s.db[263][4]);let eq55_e682_d_b5: f64 = (s.db[311][5] + s.db[263][5]);let eq55_e682_d_b6: f64 = (s.db[311][6] + s.db[263][6]);let eq55_e682_d_b7: f64 = (s.db[311][7] + s.db[263][7]);let eq55_e682_d_b8: f64 = (s.db[311][8] + s.db[263][8]);let eq55_e682_d_b9: f64 = (s.db[311][9] + s.db[263][9]);let eq55_e682_d_b10: f64 = (s.db[311][10] + s.db[263][10]);let eq55_e682_d_b11: f64 = (s.db[311][11] + s.db[263][11]);let eq55_e682_d_b12: f64 = (s.db[311][12] + s.db[263][12]);let eq55_e682_d_b13: f64 = (s.db[311][13] + s.db[263][13]);let eq55_e682_d_b14: f64 = (s.db[311][14] + s.db[263][14]);let eq55_e682_d_b15: f64 = (s.db[311][15] + s.db[263][15]);let eq55_e683: f64 = (p.p50 * eq55_e682);let eq55_e683_d_n0: f64 = (p.p50 * eq55_e682_d_n0);let eq55_e683_d_n1: f64 = (p.p50 * eq55_e682_d_n1);let eq55_e683_d_n2: f64 = (p.p50 * eq55_e682_d_n2);let eq55_e683_d_n3: f64 = (p.p50 * eq55_e682_d_n3);let eq55_e683_d_n4: f64 = (p.p50 * eq55_e682_d_n4);let eq55_e683_d_n5: f64 = (p.p50 * eq55_e682_d_n5);let eq55_e683_d_n6: f64 = (p.p50 * eq55_e682_d_n6);let eq55_e683_d_n7: f64 = (p.p50 * eq55_e682_d_n7);let eq55_e683_d_n8: f64 = (p.p50 * eq55_e682_d_n8);let eq55_e683_d_n9: f64 = (p.p50 * eq55_e682_d_n9);let eq55_e683_d_n10: f64 = (p.p50 * eq55_e682_d_n10);let eq55_e683_d_n11: f64 = (p.p50 * eq55_e682_d_n11);let eq55_e683_d_n12: f64 = (p.p50 * eq55_e682_d_n12);let eq55_e683_d_n13: f64 = (p.p50 * eq55_e682_d_n13);let eq55_e683_d_n14: f64 = (p.p50 * eq55_e682_d_n14);let eq55_e683_d_n15: f64 = (p.p50 * eq55_e682_d_n15);let eq55_e683_d_n16: f64 = (p.p50 * eq55_e682_d_n16);let eq55_e683_d_n17: f64 = (p.p50 * eq55_e682_d_n17);let eq55_e683_d_n18: f64 = (p.p50 * eq55_e682_d_n18);let eq55_e683_d_b0: f64 = (p.p50 * eq55_e682_d_b0);let eq55_e683_d_b1: f64 = (p.p50 * eq55_e682_d_b1);let eq55_e683_d_b2: f64 = (p.p50 * eq55_e682_d_b2);let eq55_e683_d_b3: f64 = (p.p50 * eq55_e682_d_b3);let eq55_e683_d_b4: f64 = (p.p50 * eq55_e682_d_b4);let eq55_e683_d_b5: f64 = (p.p50 * eq55_e682_d_b5);let eq55_e683_d_b6: f64 = (p.p50 * eq55_e682_d_b6);let eq55_e683_d_b7: f64 = (p.p50 * eq55_e682_d_b7);let eq55_e683_d_b8: f64 = (p.p50 * eq55_e682_d_b8);let eq55_e683_d_b9: f64 = (p.p50 * eq55_e682_d_b9);let eq55_e683_d_b10: f64 = (p.p50 * eq55_e682_d_b10);let eq55_e683_d_b11: f64 = (p.p50 * eq55_e682_d_b11);let eq55_e683_d_b12: f64 = (p.p50 * eq55_e682_d_b12);let eq55_e683_d_b13: f64 = (p.p50 * eq55_e682_d_b13);let eq55_e683_d_b14: f64 = (p.p50 * eq55_e682_d_b14);let eq55_e683_d_b15: f64 = (p.p50 * eq55_e682_d_b15);
        (eq55_e683, eq55_e683_d_n0, eq55_e683_d_n1, eq55_e683_d_n2, eq55_e683_d_n3, eq55_e683_d_n4, eq55_e683_d_n5, eq55_e683_d_n6, eq55_e683_d_n7, eq55_e683_d_n8, eq55_e683_d_n9, eq55_e683_d_n10, eq55_e683_d_n11, eq55_e683_d_n12, eq55_e683_d_n13, eq55_e683_d_n14, eq55_e683_d_n15, eq55_e683_d_n16, eq55_e683_d_n17, eq55_e683_d_n18, eq55_e683_d_b0, eq55_e683_d_b1, eq55_e683_d_b2, eq55_e683_d_b3, eq55_e683_d_b4, eq55_e683_d_b5, eq55_e683_d_b6, eq55_e683_d_b7, eq55_e683_d_b8, eq55_e683_d_b9, eq55_e683_d_b10, eq55_e683_d_b11, eq55_e683_d_b12, eq55_e683_d_b13, eq55_e683_d_b14, eq55_e683_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e685;let eq55_node_derivatives: [f64; 19] = [eq55_e685_d_n0, eq55_e685_d_n1, eq55_e685_d_n2, eq55_e685_d_n3, eq55_e685_d_n4, eq55_e685_d_n5, eq55_e685_d_n6, eq55_e685_d_n7, eq55_e685_d_n8, eq55_e685_d_n9, eq55_e685_d_n10, eq55_e685_d_n11, eq55_e685_d_n12, eq55_e685_d_n13, eq55_e685_d_n14, eq55_e685_d_n15, eq55_e685_d_n16, eq55_e685_d_n17, eq55_e685_d_n18];let eq55_branch_derivatives: [f64; 16] = [eq55_e685_d_b0, eq55_e685_d_b1, eq55_e685_d_b2, eq55_e685_d_b3, eq55_e685_d_b4, eq55_e685_d_b5, eq55_e685_d_b6, eq55_e685_d_b7, eq55_e685_d_b8, eq55_e685_d_b9, eq55_e685_d_b10, eq55_e685_d_b11, eq55_e685_d_b12, eq55_e685_d_b13, eq55_e685_d_b14, eq55_e685_d_b15];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq55_value),
            &eq55_node_derivatives,
            &eq55_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_14(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq56_e694, eq56_e694_d_n0, eq56_e694_d_n1, eq56_e694_d_n2, eq56_e694_d_n3, eq56_e694_d_n4, eq56_e694_d_n5, eq56_e694_d_n6, eq56_e694_d_n7, eq56_e694_d_n8, eq56_e694_d_n9, eq56_e694_d_n10, eq56_e694_d_n11, eq56_e694_d_n12, eq56_e694_d_n13, eq56_e694_d_n14, eq56_e694_d_n15, eq56_e694_d_n16, eq56_e694_d_n17, eq56_e694_d_n18, eq56_e694_d_b0, eq56_e694_d_b1, eq56_e694_d_b2, eq56_e694_d_b3, eq56_e694_d_b4, eq56_e694_d_b5, eq56_e694_d_b6, eq56_e694_d_b7, eq56_e694_d_b8, eq56_e694_d_b9, eq56_e694_d_b10, eq56_e694_d_b11, eq56_e694_d_b12, eq56_e694_d_b13, eq56_e694_d_b14, eq56_e694_d_b15,) = {
    if (!s.b[1851]) {
        let eq56_e691: f64 = (s.v[312] + s.v[573]);let eq56_e691_d_n0: f64 = (s.dn[312][0] + s.dn[573][0]);let eq56_e691_d_n1: f64 = (s.dn[312][1] + s.dn[573][1]);let eq56_e691_d_n2: f64 = (s.dn[312][2] + s.dn[573][2]);let eq56_e691_d_n3: f64 = (s.dn[312][3] + s.dn[573][3]);let eq56_e691_d_n4: f64 = (s.dn[312][4] + s.dn[573][4]);let eq56_e691_d_n5: f64 = (s.dn[312][5] + s.dn[573][5]);let eq56_e691_d_n6: f64 = (s.dn[312][6] + s.dn[573][6]);let eq56_e691_d_n7: f64 = (s.dn[312][7] + s.dn[573][7]);let eq56_e691_d_n8: f64 = (s.dn[312][8] + s.dn[573][8]);let eq56_e691_d_n9: f64 = (s.dn[312][9] + s.dn[573][9]);let eq56_e691_d_n10: f64 = (s.dn[312][10] + s.dn[573][10]);let eq56_e691_d_n11: f64 = (s.dn[312][11] + s.dn[573][11]);let eq56_e691_d_n12: f64 = (s.dn[312][12] + s.dn[573][12]);let eq56_e691_d_n13: f64 = (s.dn[312][13] + s.dn[573][13]);let eq56_e691_d_n14: f64 = (s.dn[312][14] + s.dn[573][14]);let eq56_e691_d_n15: f64 = (s.dn[312][15] + s.dn[573][15]);let eq56_e691_d_n16: f64 = (s.dn[312][16] + s.dn[573][16]);let eq56_e691_d_n17: f64 = (s.dn[312][17] + s.dn[573][17]);let eq56_e691_d_n18: f64 = (s.dn[312][18] + s.dn[573][18]);let eq56_e691_d_b0: f64 = (s.db[312][0] + s.db[573][0]);let eq56_e691_d_b1: f64 = (s.db[312][1] + s.db[573][1]);let eq56_e691_d_b2: f64 = (s.db[312][2] + s.db[573][2]);let eq56_e691_d_b3: f64 = (s.db[312][3] + s.db[573][3]);let eq56_e691_d_b4: f64 = (s.db[312][4] + s.db[573][4]);let eq56_e691_d_b5: f64 = (s.db[312][5] + s.db[573][5]);let eq56_e691_d_b6: f64 = (s.db[312][6] + s.db[573][6]);let eq56_e691_d_b7: f64 = (s.db[312][7] + s.db[573][7]);let eq56_e691_d_b8: f64 = (s.db[312][8] + s.db[573][8]);let eq56_e691_d_b9: f64 = (s.db[312][9] + s.db[573][9]);let eq56_e691_d_b10: f64 = (s.db[312][10] + s.db[573][10]);let eq56_e691_d_b11: f64 = (s.db[312][11] + s.db[573][11]);let eq56_e691_d_b12: f64 = (s.db[312][12] + s.db[573][12]);let eq56_e691_d_b13: f64 = (s.db[312][13] + s.db[573][13]);let eq56_e691_d_b14: f64 = (s.db[312][14] + s.db[573][14]);let eq56_e691_d_b15: f64 = (s.db[312][15] + s.db[573][15]);let eq56_e692: f64 = (p.p50 * eq56_e691);let eq56_e692_d_n0: f64 = (p.p50 * eq56_e691_d_n0);let eq56_e692_d_n1: f64 = (p.p50 * eq56_e691_d_n1);let eq56_e692_d_n2: f64 = (p.p50 * eq56_e691_d_n2);let eq56_e692_d_n3: f64 = (p.p50 * eq56_e691_d_n3);let eq56_e692_d_n4: f64 = (p.p50 * eq56_e691_d_n4);let eq56_e692_d_n5: f64 = (p.p50 * eq56_e691_d_n5);let eq56_e692_d_n6: f64 = (p.p50 * eq56_e691_d_n6);let eq56_e692_d_n7: f64 = (p.p50 * eq56_e691_d_n7);let eq56_e692_d_n8: f64 = (p.p50 * eq56_e691_d_n8);let eq56_e692_d_n9: f64 = (p.p50 * eq56_e691_d_n9);let eq56_e692_d_n10: f64 = (p.p50 * eq56_e691_d_n10);let eq56_e692_d_n11: f64 = (p.p50 * eq56_e691_d_n11);let eq56_e692_d_n12: f64 = (p.p50 * eq56_e691_d_n12);let eq56_e692_d_n13: f64 = (p.p50 * eq56_e691_d_n13);let eq56_e692_d_n14: f64 = (p.p50 * eq56_e691_d_n14);let eq56_e692_d_n15: f64 = (p.p50 * eq56_e691_d_n15);let eq56_e692_d_n16: f64 = (p.p50 * eq56_e691_d_n16);let eq56_e692_d_n17: f64 = (p.p50 * eq56_e691_d_n17);let eq56_e692_d_n18: f64 = (p.p50 * eq56_e691_d_n18);let eq56_e692_d_b0: f64 = (p.p50 * eq56_e691_d_b0);let eq56_e692_d_b1: f64 = (p.p50 * eq56_e691_d_b1);let eq56_e692_d_b2: f64 = (p.p50 * eq56_e691_d_b2);let eq56_e692_d_b3: f64 = (p.p50 * eq56_e691_d_b3);let eq56_e692_d_b4: f64 = (p.p50 * eq56_e691_d_b4);let eq56_e692_d_b5: f64 = (p.p50 * eq56_e691_d_b5);let eq56_e692_d_b6: f64 = (p.p50 * eq56_e691_d_b6);let eq56_e692_d_b7: f64 = (p.p50 * eq56_e691_d_b7);let eq56_e692_d_b8: f64 = (p.p50 * eq56_e691_d_b8);let eq56_e692_d_b9: f64 = (p.p50 * eq56_e691_d_b9);let eq56_e692_d_b10: f64 = (p.p50 * eq56_e691_d_b10);let eq56_e692_d_b11: f64 = (p.p50 * eq56_e691_d_b11);let eq56_e692_d_b12: f64 = (p.p50 * eq56_e691_d_b12);let eq56_e692_d_b13: f64 = (p.p50 * eq56_e691_d_b13);let eq56_e692_d_b14: f64 = (p.p50 * eq56_e691_d_b14);let eq56_e692_d_b15: f64 = (p.p50 * eq56_e691_d_b15);
        (eq56_e692, eq56_e692_d_n0, eq56_e692_d_n1, eq56_e692_d_n2, eq56_e692_d_n3, eq56_e692_d_n4, eq56_e692_d_n5, eq56_e692_d_n6, eq56_e692_d_n7, eq56_e692_d_n8, eq56_e692_d_n9, eq56_e692_d_n10, eq56_e692_d_n11, eq56_e692_d_n12, eq56_e692_d_n13, eq56_e692_d_n14, eq56_e692_d_n15, eq56_e692_d_n16, eq56_e692_d_n17, eq56_e692_d_n18, eq56_e692_d_b0, eq56_e692_d_b1, eq56_e692_d_b2, eq56_e692_d_b3, eq56_e692_d_b4, eq56_e692_d_b5, eq56_e692_d_b6, eq56_e692_d_b7, eq56_e692_d_b8, eq56_e692_d_b9, eq56_e692_d_b10, eq56_e692_d_b11, eq56_e692_d_b12, eq56_e692_d_b13, eq56_e692_d_b14, eq56_e692_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_value: f64 = eq56_e694;let eq56_node_derivatives: [f64; 19] = [eq56_e694_d_n0, eq56_e694_d_n1, eq56_e694_d_n2, eq56_e694_d_n3, eq56_e694_d_n4, eq56_e694_d_n5, eq56_e694_d_n6, eq56_e694_d_n7, eq56_e694_d_n8, eq56_e694_d_n9, eq56_e694_d_n10, eq56_e694_d_n11, eq56_e694_d_n12, eq56_e694_d_n13, eq56_e694_d_n14, eq56_e694_d_n15, eq56_e694_d_n16, eq56_e694_d_n17, eq56_e694_d_n18];let eq56_branch_derivatives: [f64; 16] = [eq56_e694_d_b0, eq56_e694_d_b1, eq56_e694_d_b2, eq56_e694_d_b3, eq56_e694_d_b4, eq56_e694_d_b5, eq56_e694_d_b6, eq56_e694_d_b7, eq56_e694_d_b8, eq56_e694_d_b9, eq56_e694_d_b10, eq56_e694_d_b11, eq56_e694_d_b12, eq56_e694_d_b13, eq56_e694_d_b14, eq56_e694_d_b15];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq56_value),
            &eq56_node_derivatives,
            &eq56_branch_derivatives,
            multiplicity,
        );
        let (eq57_e699,) = {
    if (!s.b[1851]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq57_value: f64 = eq57_e699;
        stamper.stamp_potential_const_local(
            12,
            eq57_value,
        );
        let (eq58_e706, eq58_e706_d_n0, eq58_e706_d_n1, eq58_e706_d_n2, eq58_e706_d_n3, eq58_e706_d_n4, eq58_e706_d_n5, eq58_e706_d_n6, eq58_e706_d_n7, eq58_e706_d_n8, eq58_e706_d_n9, eq58_e706_d_n10, eq58_e706_d_n11, eq58_e706_d_n12, eq58_e706_d_n13, eq58_e706_d_n14, eq58_e706_d_n15, eq58_e706_d_n16, eq58_e706_d_n17, eq58_e706_d_n18, eq58_e706_d_b0, eq58_e706_d_b1, eq58_e706_d_b2, eq58_e706_d_b3, eq58_e706_d_b4, eq58_e706_d_b5, eq58_e706_d_b6, eq58_e706_d_b7, eq58_e706_d_b8, eq58_e706_d_b9, eq58_e706_d_b10, eq58_e706_d_b11, eq58_e706_d_b12, eq58_e706_d_b13, eq58_e706_d_b14, eq58_e706_d_b15,) = {
    if ((!s.b[1851]) && (p.p37 != 0.0)) {
        (s.v[592], s.dn[592][0], s.dn[592][1], s.dn[592][2], s.dn[592][3], s.dn[592][4], s.dn[592][5], s.dn[592][6], s.dn[592][7], s.dn[592][8], s.dn[592][9], s.dn[592][10], s.dn[592][11], s.dn[592][12], s.dn[592][13], s.dn[592][14], s.dn[592][15], s.dn[592][16], s.dn[592][17], s.dn[592][18], s.db[592][0], s.db[592][1], s.db[592][2], s.db[592][3], s.db[592][4], s.db[592][5], s.db[592][6], s.db[592][7], s.db[592][8], s.db[592][9], s.db[592][10], s.db[592][11], s.db[592][12], s.db[592][13], s.db[592][14], s.db[592][15],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e706;let eq58_node_derivatives: [f64; 19] = [eq58_e706_d_n0, eq58_e706_d_n1, eq58_e706_d_n2, eq58_e706_d_n3, eq58_e706_d_n4, eq58_e706_d_n5, eq58_e706_d_n6, eq58_e706_d_n7, eq58_e706_d_n8, eq58_e706_d_n9, eq58_e706_d_n10, eq58_e706_d_n11, eq58_e706_d_n12, eq58_e706_d_n13, eq58_e706_d_n14, eq58_e706_d_n15, eq58_e706_d_n16, eq58_e706_d_n17, eq58_e706_d_n18];let eq58_branch_derivatives: [f64; 16] = [eq58_e706_d_b0, eq58_e706_d_b1, eq58_e706_d_b2, eq58_e706_d_b3, eq58_e706_d_b4, eq58_e706_d_b5, eq58_e706_d_b6, eq58_e706_d_b7, eq58_e706_d_b8, eq58_e706_d_b9, eq58_e706_d_b10, eq58_e706_d_b11, eq58_e706_d_b12, eq58_e706_d_b13, eq58_e706_d_b14, eq58_e706_d_b15];
        stamper.stamp_current_dense_local(
            Some(17),
            None,
            multiplicity * (eq58_value),
            &eq58_node_derivatives,
            &eq58_branch_derivatives,
            multiplicity,
        );
        let (eq59_e715, eq59_e715_d_n17,) = {
    if ((!s.b[1851]) && (p.p37 != 0.0)) {
        let eq59_e713: f64 = ((nv17 - 0.0) * 1e-12);
        (eq59_e713, 1e-12,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e715;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq59_value),
            17,
            multiplicity * (eq59_e715_d_n17),
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_15(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let nv13 = ctx.node_voltage(nodes[13]);let nv15 = ctx.node_voltage(nodes[15]);let nv16 = ctx.node_voltage(nodes[16]);let nv17 = ctx.node_voltage(nodes[17]);
        let (eq60_e727, eq60_e727_d_n17,) = {
    if ((!s.b[1851]) && (p.p37 != 0.0)) {
        let eq60_e722: f64 = (1e-9 / 0.0001);let eq60_e724: f64 = (eq60_e722 * (nv17 - 0.0));let eq60_e725: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq60_e724);
        (eq60_e725, (eq60_e722 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e727;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq60_value),
            17,
            multiplicity * (eq60_e727_d_n17),
        );
        let (eq61_e735,) = {
    if ((!s.b[1851]) && (p.p37 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq61_value: f64 = eq61_e735;
        stamper.stamp_potential_const_local(
            13,
            eq61_value,
        );
        let (eq62_e742, eq62_e742_d_n0, eq62_e742_d_n1, eq62_e742_d_n2, eq62_e742_d_n3, eq62_e742_d_n4, eq62_e742_d_n5, eq62_e742_d_n6, eq62_e742_d_n7, eq62_e742_d_n8, eq62_e742_d_n9, eq62_e742_d_n10, eq62_e742_d_n11, eq62_e742_d_n12, eq62_e742_d_n13, eq62_e742_d_n14, eq62_e742_d_n15, eq62_e742_d_n16, eq62_e742_d_n17, eq62_e742_d_n18, eq62_e742_d_b0, eq62_e742_d_b1, eq62_e742_d_b2, eq62_e742_d_b3, eq62_e742_d_b4, eq62_e742_d_b5, eq62_e742_d_b6, eq62_e742_d_b7, eq62_e742_d_b8, eq62_e742_d_b9, eq62_e742_d_b10, eq62_e742_d_b11, eq62_e742_d_b12, eq62_e742_d_b13, eq62_e742_d_b14, eq62_e742_d_b15,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        (s.v[574], s.dn[574][0], s.dn[574][1], s.dn[574][2], s.dn[574][3], s.dn[574][4], s.dn[574][5], s.dn[574][6], s.dn[574][7], s.dn[574][8], s.dn[574][9], s.dn[574][10], s.dn[574][11], s.dn[574][12], s.dn[574][13], s.dn[574][14], s.dn[574][15], s.dn[574][16], s.dn[574][17], s.dn[574][18], s.db[574][0], s.db[574][1], s.db[574][2], s.db[574][3], s.db[574][4], s.db[574][5], s.db[574][6], s.db[574][7], s.db[574][8], s.db[574][9], s.db[574][10], s.db[574][11], s.db[574][12], s.db[574][13], s.db[574][14], s.db[574][15],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e742;let eq62_node_derivatives: [f64; 19] = [eq62_e742_d_n0, eq62_e742_d_n1, eq62_e742_d_n2, eq62_e742_d_n3, eq62_e742_d_n4, eq62_e742_d_n5, eq62_e742_d_n6, eq62_e742_d_n7, eq62_e742_d_n8, eq62_e742_d_n9, eq62_e742_d_n10, eq62_e742_d_n11, eq62_e742_d_n12, eq62_e742_d_n13, eq62_e742_d_n14, eq62_e742_d_n15, eq62_e742_d_n16, eq62_e742_d_n17, eq62_e742_d_n18];let eq62_branch_derivatives: [f64; 16] = [eq62_e742_d_b0, eq62_e742_d_b1, eq62_e742_d_b2, eq62_e742_d_b3, eq62_e742_d_b4, eq62_e742_d_b5, eq62_e742_d_b6, eq62_e742_d_b7, eq62_e742_d_b8, eq62_e742_d_b9, eq62_e742_d_b10, eq62_e742_d_b11, eq62_e742_d_b12, eq62_e742_d_b13, eq62_e742_d_b14, eq62_e742_d_b15];
        stamper.stamp_current_dense_local(
            Some(15),
            None,
            multiplicity * (eq62_value),
            &eq62_node_derivatives,
            &eq62_branch_derivatives,
            multiplicity,
        );
        let (eq63_e749, eq63_e749_d_n0, eq63_e749_d_n1, eq63_e749_d_n2, eq63_e749_d_n3, eq63_e749_d_n4, eq63_e749_d_n5, eq63_e749_d_n6, eq63_e749_d_n7, eq63_e749_d_n8, eq63_e749_d_n9, eq63_e749_d_n10, eq63_e749_d_n11, eq63_e749_d_n12, eq63_e749_d_n13, eq63_e749_d_n14, eq63_e749_d_n15, eq63_e749_d_n16, eq63_e749_d_n17, eq63_e749_d_n18, eq63_e749_d_b0, eq63_e749_d_b1, eq63_e749_d_b2, eq63_e749_d_b3, eq63_e749_d_b4, eq63_e749_d_b5, eq63_e749_d_b6, eq63_e749_d_b7, eq63_e749_d_b8, eq63_e749_d_b9, eq63_e749_d_b10, eq63_e749_d_b11, eq63_e749_d_b12, eq63_e749_d_b13, eq63_e749_d_b14, eq63_e749_d_b15,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        (s.v[575], s.dn[575][0], s.dn[575][1], s.dn[575][2], s.dn[575][3], s.dn[575][4], s.dn[575][5], s.dn[575][6], s.dn[575][7], s.dn[575][8], s.dn[575][9], s.dn[575][10], s.dn[575][11], s.dn[575][12], s.dn[575][13], s.dn[575][14], s.dn[575][15], s.dn[575][16], s.dn[575][17], s.dn[575][18], s.db[575][0], s.db[575][1], s.db[575][2], s.db[575][3], s.db[575][4], s.db[575][5], s.db[575][6], s.db[575][7], s.db[575][8], s.db[575][9], s.db[575][10], s.db[575][11], s.db[575][12], s.db[575][13], s.db[575][14], s.db[575][15],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e749;let eq63_node_derivatives: [f64; 19] = [eq63_e749_d_n0, eq63_e749_d_n1, eq63_e749_d_n2, eq63_e749_d_n3, eq63_e749_d_n4, eq63_e749_d_n5, eq63_e749_d_n6, eq63_e749_d_n7, eq63_e749_d_n8, eq63_e749_d_n9, eq63_e749_d_n10, eq63_e749_d_n11, eq63_e749_d_n12, eq63_e749_d_n13, eq63_e749_d_n14, eq63_e749_d_n15, eq63_e749_d_n16, eq63_e749_d_n17, eq63_e749_d_n18];let eq63_branch_derivatives: [f64; 16] = [eq63_e749_d_b0, eq63_e749_d_b1, eq63_e749_d_b2, eq63_e749_d_b3, eq63_e749_d_b4, eq63_e749_d_b5, eq63_e749_d_b6, eq63_e749_d_b7, eq63_e749_d_b8, eq63_e749_d_b9, eq63_e749_d_b10, eq63_e749_d_b11, eq63_e749_d_b12, eq63_e749_d_b13, eq63_e749_d_b14, eq63_e749_d_b15];
        stamper.stamp_current_dense_local(
            Some(16),
            None,
            multiplicity * (eq63_value),
            &eq63_node_derivatives,
            &eq63_branch_derivatives,
            multiplicity,
        );
        let (eq64_e756, eq64_e756_d_n0, eq64_e756_d_n1, eq64_e756_d_n2, eq64_e756_d_n3, eq64_e756_d_n4, eq64_e756_d_n5, eq64_e756_d_n6, eq64_e756_d_n7, eq64_e756_d_n8, eq64_e756_d_n9, eq64_e756_d_n10, eq64_e756_d_n11, eq64_e756_d_n12, eq64_e756_d_n13, eq64_e756_d_n14, eq64_e756_d_n15, eq64_e756_d_n16, eq64_e756_d_n17, eq64_e756_d_n18, eq64_e756_d_b0, eq64_e756_d_b1, eq64_e756_d_b2, eq64_e756_d_b3, eq64_e756_d_b4, eq64_e756_d_b5, eq64_e756_d_b6, eq64_e756_d_b7, eq64_e756_d_b8, eq64_e756_d_b9, eq64_e756_d_b10, eq64_e756_d_b11, eq64_e756_d_b12, eq64_e756_d_b13, eq64_e756_d_b14, eq64_e756_d_b15,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        (s.v[583], s.dn[583][0], s.dn[583][1], s.dn[583][2], s.dn[583][3], s.dn[583][4], s.dn[583][5], s.dn[583][6], s.dn[583][7], s.dn[583][8], s.dn[583][9], s.dn[583][10], s.dn[583][11], s.dn[583][12], s.dn[583][13], s.dn[583][14], s.dn[583][15], s.dn[583][16], s.dn[583][17], s.dn[583][18], s.db[583][0], s.db[583][1], s.db[583][2], s.db[583][3], s.db[583][4], s.db[583][5], s.db[583][6], s.db[583][7], s.db[583][8], s.db[583][9], s.db[583][10], s.db[583][11], s.db[583][12], s.db[583][13], s.db[583][14], s.db[583][15],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq64_value: f64 = eq64_e756;let eq64_node_derivatives: [f64; 19] = [eq64_e756_d_n0, eq64_e756_d_n1, eq64_e756_d_n2, eq64_e756_d_n3, eq64_e756_d_n4, eq64_e756_d_n5, eq64_e756_d_n6, eq64_e756_d_n7, eq64_e756_d_n8, eq64_e756_d_n9, eq64_e756_d_n10, eq64_e756_d_n11, eq64_e756_d_n12, eq64_e756_d_n13, eq64_e756_d_n14, eq64_e756_d_n15, eq64_e756_d_n16, eq64_e756_d_n17, eq64_e756_d_n18];let eq64_branch_derivatives: [f64; 16] = [eq64_e756_d_b0, eq64_e756_d_b1, eq64_e756_d_b2, eq64_e756_d_b3, eq64_e756_d_b4, eq64_e756_d_b5, eq64_e756_d_b6, eq64_e756_d_b7, eq64_e756_d_b8, eq64_e756_d_b9, eq64_e756_d_b10, eq64_e756_d_b11, eq64_e756_d_b12, eq64_e756_d_b13, eq64_e756_d_b14, eq64_e756_d_b15];
        stamper.stamp_current_dense_local(
            Some(13),
            None,
            multiplicity * (eq64_value),
            &eq64_node_derivatives,
            &eq64_branch_derivatives,
            multiplicity,
        );
        let (eq65_e765, eq65_e765_d_n15,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        let eq65_e763: f64 = ((nv15 - 0.0) * 1e-12);
        (eq65_e763, 1e-12,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e765;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq65_value),
            15,
            multiplicity * (eq65_e765_d_n15),
        );
        let (eq66_e774, eq66_e774_d_n16,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        let eq66_e772: f64 = ((nv16 - 0.0) * 1e-12);
        (eq66_e772, 1e-12,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq66_value: f64 = eq66_e774;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (eq66_value),
            16,
            multiplicity * (eq66_e774_d_n16),
        );
        let (eq67_e783, eq67_e783_d_n13,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        let eq67_e781: f64 = ((nv13 - 0.0) * 1e-12);
        (eq67_e781, 1e-12,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e783;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq67_value),
            13,
            multiplicity * (eq67_e783_d_n13),
        );
        let (eq68_e795, eq68_e795_d_n15,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        let eq68_e790: f64 = (1e-9 / 0.0001);let eq68_e792: f64 = (eq68_e790 * (nv15 - 0.0));let eq68_e793: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq68_e792);
        (eq68_e793, (eq68_e790 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq68_value: f64 = eq68_e795;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq68_value),
            15,
            multiplicity * (eq68_e795_d_n15),
        );
        let (eq69_e807, eq69_e807_d_n16,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        let eq69_e802: f64 = (1e-9 / 0.0001);let eq69_e804: f64 = (eq69_e802 * (nv16 - 0.0));let eq69_e805: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq69_e804);
        (eq69_e805, (eq69_e802 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e807;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (eq69_value),
            16,
            multiplicity * (eq69_e807_d_n16),
        );
        let (eq70_e819, eq70_e819_d_n13,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        let eq70_e814: f64 = (1e-9 / 0.0001);let eq70_e816: f64 = (eq70_e814 * (nv13 - 0.0));let eq70_e817: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq70_e816);
        (eq70_e817, (eq70_e814 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq70_value: f64 = eq70_e819;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq70_value),
            13,
            multiplicity * (eq70_e819_d_n13),
        );
        let (eq71_e827,) = {
    if ((!s.b[1851]) && (p.p34 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq71_value: f64 = eq71_e827;
        stamper.stamp_potential_const_local(
            14,
            eq71_value,
        );
        let (eq72_e835,) = {
    if ((!s.b[1851]) && (p.p34 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq72_value: f64 = eq72_e835;
        stamper.stamp_potential_const_local(
            15,
            eq72_value,
        );
        let (eq73_e843,) = {
    if ((!s.b[1851]) && (p.p34 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq73_value: f64 = eq73_e843;
        stamper.stamp_potential_const_local(
            16,
            eq73_value,
        );
        let (eq74_e847,) = {
    if s.b[1853] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq74_value: f64 = eq74_e847;
        stamper.stamp_potential_const_local(
            17,
            eq74_value,
        );
        let (eq75_e852,) = {
    if (!s.b[1853]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq75_value: f64 = eq75_e852;
        stamper.stamp_potential_const_local(
            18,
            eq75_value,
        );
        let (eq76_e857,) = {
    if (!s.b[1853]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq76_value: f64 = eq76_e857;
        stamper.stamp_potential_const_local(
            19,
            eq76_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv14 = ctx.node_voltage(nodes[14]);let eq11_e370_q: f64 = s.v[594];let eq11_e371: f64 = (p.p50 * s.v[594]);let eq11_e371_q: f64 = (p.p50 * eq11_e370_q);
        stamper.stamp_current_reactive_dense_local(
            Some(11),
            Some(7),
            &s.dn[594],
            &s.db[594],
            (multiplicity) * (p.p50),
        );let eq12_e374_q: f64 = s.v[198];let eq12_e375: f64 = (p.p50 * s.v[198]);let eq12_e375_q: f64 = (p.p50 * eq12_e374_q);
        stamper.stamp_current_reactive_dense_local(
            Some(6),
            Some(7),
            &s.dn[198],
            &s.db[198],
            (multiplicity) * (p.p50),
        );let eq13_e378_q: f64 = s.v[196];let eq13_e379: f64 = (p.p50 * s.v[196]);let eq13_e379_q: f64 = (p.p50 * eq13_e378_q);
        stamper.stamp_current_reactive_dense_local(
            Some(12),
            Some(7),
            &s.dn[196],
            &s.db[196],
            (multiplicity) * (p.p50),
        );let eq19_e408: f64 = ((nv14 - 0.0) * s.v[617]);let eq19_e408_d_n0: f64 = ((nv14 - 0.0) * s.dn[617][0]);let eq19_e408_d_n1: f64 = ((nv14 - 0.0) * s.dn[617][1]);let eq19_e408_d_n2: f64 = ((nv14 - 0.0) * s.dn[617][2]);let eq19_e408_d_n3: f64 = ((nv14 - 0.0) * s.dn[617][3]);let eq19_e408_d_n4: f64 = ((nv14 - 0.0) * s.dn[617][4]);let eq19_e408_d_n5: f64 = ((nv14 - 0.0) * s.dn[617][5]);let eq19_e408_d_n6: f64 = ((nv14 - 0.0) * s.dn[617][6]);let eq19_e408_d_n7: f64 = ((nv14 - 0.0) * s.dn[617][7]);let eq19_e408_d_n8: f64 = ((nv14 - 0.0) * s.dn[617][8]);let eq19_e408_d_n9: f64 = ((nv14 - 0.0) * s.dn[617][9]);let eq19_e408_d_n10: f64 = ((nv14 - 0.0) * s.dn[617][10]);let eq19_e408_d_n11: f64 = ((nv14 - 0.0) * s.dn[617][11]);let eq19_e408_d_n12: f64 = ((nv14 - 0.0) * s.dn[617][12]);let eq19_e408_d_n13: f64 = ((nv14 - 0.0) * s.dn[617][13]);let eq19_e408_d_n14: f64 = (s.v[617] + ((nv14 - 0.0) * s.dn[617][14]));let eq19_e408_d_n15: f64 = ((nv14 - 0.0) * s.dn[617][15]);let eq19_e408_d_n16: f64 = ((nv14 - 0.0) * s.dn[617][16]);let eq19_e408_d_n17: f64 = ((nv14 - 0.0) * s.dn[617][17]);let eq19_e408_d_n18: f64 = ((nv14 - 0.0) * s.dn[617][18]);let eq19_e408_d_b0: f64 = ((nv14 - 0.0) * s.db[617][0]);let eq19_e408_d_b1: f64 = ((nv14 - 0.0) * s.db[617][1]);let eq19_e408_d_b2: f64 = ((nv14 - 0.0) * s.db[617][2]);let eq19_e408_d_b3: f64 = ((nv14 - 0.0) * s.db[617][3]);let eq19_e408_d_b4: f64 = ((nv14 - 0.0) * s.db[617][4]);let eq19_e408_d_b5: f64 = ((nv14 - 0.0) * s.db[617][5]);let eq19_e408_d_b6: f64 = ((nv14 - 0.0) * s.db[617][6]);let eq19_e408_d_b7: f64 = ((nv14 - 0.0) * s.db[617][7]);let eq19_e408_d_b8: f64 = ((nv14 - 0.0) * s.db[617][8]);let eq19_e408_d_b9: f64 = ((nv14 - 0.0) * s.db[617][9]);let eq19_e408_d_b10: f64 = ((nv14 - 0.0) * s.db[617][10]);let eq19_e408_d_b11: f64 = ((nv14 - 0.0) * s.db[617][11]);let eq19_e408_d_b12: f64 = ((nv14 - 0.0) * s.db[617][12]);let eq19_e408_d_b13: f64 = ((nv14 - 0.0) * s.db[617][13]);let eq19_e408_d_b14: f64 = ((nv14 - 0.0) * s.db[617][14]);let eq19_e408_d_b15: f64 = ((nv14 - 0.0) * s.db[617][15]);let eq19_e409_q: f64 = eq19_e408;let eq19_reactive_node_derivatives: [f64; 19] = [eq19_e408_d_n0, eq19_e408_d_n1, eq19_e408_d_n2, eq19_e408_d_n3, eq19_e408_d_n4, eq19_e408_d_n5, eq19_e408_d_n6, eq19_e408_d_n7, eq19_e408_d_n8, eq19_e408_d_n9, eq19_e408_d_n10, eq19_e408_d_n11, eq19_e408_d_n12, eq19_e408_d_n13, eq19_e408_d_n14, eq19_e408_d_n15, eq19_e408_d_n16, eq19_e408_d_n17, eq19_e408_d_n18];let eq19_reactive_branch_derivatives: [f64; 16] = [eq19_e408_d_b0, eq19_e408_d_b1, eq19_e408_d_b2, eq19_e408_d_b3, eq19_e408_d_b4, eq19_e408_d_b5, eq19_e408_d_b6, eq19_e408_d_b7, eq19_e408_d_b8, eq19_e408_d_b9, eq19_e408_d_b10, eq19_e408_d_b11, eq19_e408_d_b12, eq19_e408_d_b13, eq19_e408_d_b14, eq19_e408_d_b15];
        stamper.stamp_current_reactive_dense_local(
            Some(11),
            Some(7),
            &eq19_reactive_node_derivatives,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );let eq20_e412: f64 = ((nv14 - 0.0) * s.v[618]);let eq20_e412_d_n0: f64 = ((nv14 - 0.0) * s.dn[618][0]);let eq20_e412_d_n1: f64 = ((nv14 - 0.0) * s.dn[618][1]);let eq20_e412_d_n2: f64 = ((nv14 - 0.0) * s.dn[618][2]);let eq20_e412_d_n3: f64 = ((nv14 - 0.0) * s.dn[618][3]);let eq20_e412_d_n4: f64 = ((nv14 - 0.0) * s.dn[618][4]);let eq20_e412_d_n5: f64 = ((nv14 - 0.0) * s.dn[618][5]);let eq20_e412_d_n6: f64 = ((nv14 - 0.0) * s.dn[618][6]);let eq20_e412_d_n7: f64 = ((nv14 - 0.0) * s.dn[618][7]);let eq20_e412_d_n8: f64 = ((nv14 - 0.0) * s.dn[618][8]);let eq20_e412_d_n9: f64 = ((nv14 - 0.0) * s.dn[618][9]);let eq20_e412_d_n10: f64 = ((nv14 - 0.0) * s.dn[618][10]);let eq20_e412_d_n11: f64 = ((nv14 - 0.0) * s.dn[618][11]);let eq20_e412_d_n12: f64 = ((nv14 - 0.0) * s.dn[618][12]);let eq20_e412_d_n13: f64 = ((nv14 - 0.0) * s.dn[618][13]);let eq20_e412_d_n14: f64 = (s.v[618] + ((nv14 - 0.0) * s.dn[618][14]));let eq20_e412_d_n15: f64 = ((nv14 - 0.0) * s.dn[618][15]);let eq20_e412_d_n16: f64 = ((nv14 - 0.0) * s.dn[618][16]);let eq20_e412_d_n17: f64 = ((nv14 - 0.0) * s.dn[618][17]);let eq20_e412_d_n18: f64 = ((nv14 - 0.0) * s.dn[618][18]);let eq20_e412_d_b0: f64 = ((nv14 - 0.0) * s.db[618][0]);let eq20_e412_d_b1: f64 = ((nv14 - 0.0) * s.db[618][1]);let eq20_e412_d_b2: f64 = ((nv14 - 0.0) * s.db[618][2]);let eq20_e412_d_b3: f64 = ((nv14 - 0.0) * s.db[618][3]);let eq20_e412_d_b4: f64 = ((nv14 - 0.0) * s.db[618][4]);let eq20_e412_d_b5: f64 = ((nv14 - 0.0) * s.db[618][5]);let eq20_e412_d_b6: f64 = ((nv14 - 0.0) * s.db[618][6]);let eq20_e412_d_b7: f64 = ((nv14 - 0.0) * s.db[618][7]);let eq20_e412_d_b8: f64 = ((nv14 - 0.0) * s.db[618][8]);let eq20_e412_d_b9: f64 = ((nv14 - 0.0) * s.db[618][9]);let eq20_e412_d_b10: f64 = ((nv14 - 0.0) * s.db[618][10]);let eq20_e412_d_b11: f64 = ((nv14 - 0.0) * s.db[618][11]);let eq20_e412_d_b12: f64 = ((nv14 - 0.0) * s.db[618][12]);let eq20_e412_d_b13: f64 = ((nv14 - 0.0) * s.db[618][13]);let eq20_e412_d_b14: f64 = ((nv14 - 0.0) * s.db[618][14]);let eq20_e412_d_b15: f64 = ((nv14 - 0.0) * s.db[618][15]);let eq20_e413_q: f64 = eq20_e412;let eq20_reactive_node_derivatives: [f64; 19] = [eq20_e412_d_n0, eq20_e412_d_n1, eq20_e412_d_n2, eq20_e412_d_n3, eq20_e412_d_n4, eq20_e412_d_n5, eq20_e412_d_n6, eq20_e412_d_n7, eq20_e412_d_n8, eq20_e412_d_n9, eq20_e412_d_n10, eq20_e412_d_n11, eq20_e412_d_n12, eq20_e412_d_n13, eq20_e412_d_n14, eq20_e412_d_n15, eq20_e412_d_n16, eq20_e412_d_n17, eq20_e412_d_n18];let eq20_reactive_branch_derivatives: [f64; 16] = [eq20_e412_d_b0, eq20_e412_d_b1, eq20_e412_d_b2, eq20_e412_d_b3, eq20_e412_d_b4, eq20_e412_d_b5, eq20_e412_d_b6, eq20_e412_d_b7, eq20_e412_d_b8, eq20_e412_d_b9, eq20_e412_d_b10, eq20_e412_d_b11, eq20_e412_d_b12, eq20_e412_d_b13, eq20_e412_d_b14, eq20_e412_d_b15];
        stamper.stamp_current_reactive_dense_local(
            Some(11),
            Some(6),
            &eq20_reactive_node_derivatives,
            &eq20_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq31_e494, eq31_e494_d_n0, eq31_e494_d_n1, eq31_e494_d_n2, eq31_e494_d_n3, eq31_e494_d_n4, eq31_e494_d_n5, eq31_e494_d_n6, eq31_e494_d_n7, eq31_e494_d_n8, eq31_e494_d_n9, eq31_e494_d_n10, eq31_e494_d_n11, eq31_e494_d_n12, eq31_e494_d_n13, eq31_e494_d_n14, eq31_e494_d_n15, eq31_e494_d_n16, eq31_e494_d_n17, eq31_e494_d_n18, eq31_e494_d_b0, eq31_e494_d_b1, eq31_e494_d_b2, eq31_e494_d_b3, eq31_e494_d_b4, eq31_e494_d_b5, eq31_e494_d_b6, eq31_e494_d_b7, eq31_e494_d_b8, eq31_e494_d_b9, eq31_e494_d_b10, eq31_e494_d_b11, eq31_e494_d_b12, eq31_e494_d_b13, eq31_e494_d_b14, eq31_e494_d_b15, eq31_e494_q,) = {
    if s.b[1850] {
        let eq31_e491: f64 = (s.v[563] * (nv10 - 0.0));let eq31_e491_d_n0: f64 = (s.dn[563][0] * (nv10 - 0.0));let eq31_e491_d_n1: f64 = (s.dn[563][1] * (nv10 - 0.0));let eq31_e491_d_n2: f64 = (s.dn[563][2] * (nv10 - 0.0));let eq31_e491_d_n3: f64 = (s.dn[563][3] * (nv10 - 0.0));let eq31_e491_d_n4: f64 = (s.dn[563][4] * (nv10 - 0.0));let eq31_e491_d_n5: f64 = (s.dn[563][5] * (nv10 - 0.0));let eq31_e491_d_n6: f64 = (s.dn[563][6] * (nv10 - 0.0));let eq31_e491_d_n7: f64 = (s.dn[563][7] * (nv10 - 0.0));let eq31_e491_d_n8: f64 = (s.dn[563][8] * (nv10 - 0.0));let eq31_e491_d_n9: f64 = (s.dn[563][9] * (nv10 - 0.0));let eq31_e491_d_n10: f64 = ((s.dn[563][10] * (nv10 - 0.0)) + s.v[563]);let eq31_e491_d_n11: f64 = (s.dn[563][11] * (nv10 - 0.0));let eq31_e491_d_n12: f64 = (s.dn[563][12] * (nv10 - 0.0));let eq31_e491_d_n13: f64 = (s.dn[563][13] * (nv10 - 0.0));let eq31_e491_d_n14: f64 = (s.dn[563][14] * (nv10 - 0.0));let eq31_e491_d_n15: f64 = (s.dn[563][15] * (nv10 - 0.0));let eq31_e491_d_n16: f64 = (s.dn[563][16] * (nv10 - 0.0));let eq31_e491_d_n17: f64 = (s.dn[563][17] * (nv10 - 0.0));let eq31_e491_d_n18: f64 = (s.dn[563][18] * (nv10 - 0.0));let eq31_e491_d_b0: f64 = (s.db[563][0] * (nv10 - 0.0));let eq31_e491_d_b1: f64 = (s.db[563][1] * (nv10 - 0.0));let eq31_e491_d_b2: f64 = (s.db[563][2] * (nv10 - 0.0));let eq31_e491_d_b3: f64 = (s.db[563][3] * (nv10 - 0.0));let eq31_e491_d_b4: f64 = (s.db[563][4] * (nv10 - 0.0));let eq31_e491_d_b5: f64 = (s.db[563][5] * (nv10 - 0.0));let eq31_e491_d_b6: f64 = (s.db[563][6] * (nv10 - 0.0));let eq31_e491_d_b7: f64 = (s.db[563][7] * (nv10 - 0.0));let eq31_e491_d_b8: f64 = (s.db[563][8] * (nv10 - 0.0));let eq31_e491_d_b9: f64 = (s.db[563][9] * (nv10 - 0.0));let eq31_e491_d_b10: f64 = (s.db[563][10] * (nv10 - 0.0));let eq31_e491_d_b11: f64 = (s.db[563][11] * (nv10 - 0.0));let eq31_e491_d_b12: f64 = (s.db[563][12] * (nv10 - 0.0));let eq31_e491_d_b13: f64 = (s.db[563][13] * (nv10 - 0.0));let eq31_e491_d_b14: f64 = (s.db[563][14] * (nv10 - 0.0));let eq31_e491_d_b15: f64 = (s.db[563][15] * (nv10 - 0.0));let eq31_e492_q: f64 = eq31_e491;
        (eq31_e491, eq31_e491_d_n0, eq31_e491_d_n1, eq31_e491_d_n2, eq31_e491_d_n3, eq31_e491_d_n4, eq31_e491_d_n5, eq31_e491_d_n6, eq31_e491_d_n7, eq31_e491_d_n8, eq31_e491_d_n9, eq31_e491_d_n10, eq31_e491_d_n11, eq31_e491_d_n12, eq31_e491_d_n13, eq31_e491_d_n14, eq31_e491_d_n15, eq31_e491_d_n16, eq31_e491_d_n17, eq31_e491_d_n18, eq31_e491_d_b0, eq31_e491_d_b1, eq31_e491_d_b2, eq31_e491_d_b3, eq31_e491_d_b4, eq31_e491_d_b5, eq31_e491_d_b6, eq31_e491_d_b7, eq31_e491_d_b8, eq31_e491_d_b9, eq31_e491_d_b10, eq31_e491_d_b11, eq31_e491_d_b12, eq31_e491_d_b13, eq31_e491_d_b14, eq31_e491_d_b15, eq31_e492_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_reactive_node_derivatives: [f64; 19] = [eq31_e494_d_n0, eq31_e494_d_n1, eq31_e494_d_n2, eq31_e494_d_n3, eq31_e494_d_n4, eq31_e494_d_n5, eq31_e494_d_n6, eq31_e494_d_n7, eq31_e494_d_n8, eq31_e494_d_n9, eq31_e494_d_n10, eq31_e494_d_n11, eq31_e494_d_n12, eq31_e494_d_n13, eq31_e494_d_n14, eq31_e494_d_n15, eq31_e494_d_n16, eq31_e494_d_n17, eq31_e494_d_n18];let eq31_reactive_branch_derivatives: [f64; 16] = [eq31_e494_d_b0, eq31_e494_d_b1, eq31_e494_d_b2, eq31_e494_d_b3, eq31_e494_d_b4, eq31_e494_d_b5, eq31_e494_d_b6, eq31_e494_d_b7, eq31_e494_d_b8, eq31_e494_d_b9, eq31_e494_d_b10, eq31_e494_d_b11, eq31_e494_d_b12, eq31_e494_d_b13, eq31_e494_d_b14, eq31_e494_d_b15];
        stamper.stamp_current_reactive_dense_local(
            Some(10),
            None,
            &eq31_reactive_node_derivatives,
            &eq31_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_2(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let (eq35_e526, eq35_e526_d_n0, eq35_e526_d_n1, eq35_e526_d_n2, eq35_e526_d_n3, eq35_e526_d_n4, eq35_e526_d_n5, eq35_e526_d_n6, eq35_e526_d_n7, eq35_e526_d_n8, eq35_e526_d_n9, eq35_e526_d_n10, eq35_e526_d_n11, eq35_e526_d_n12, eq35_e526_d_n13, eq35_e526_d_n14, eq35_e526_d_n15, eq35_e526_d_n16, eq35_e526_d_n17, eq35_e526_d_n18, eq35_e526_d_b0, eq35_e526_d_b1, eq35_e526_d_b2, eq35_e526_d_b3, eq35_e526_d_b4, eq35_e526_d_b5, eq35_e526_d_b6, eq35_e526_d_b7, eq35_e526_d_b8, eq35_e526_d_b9, eq35_e526_d_b10, eq35_e526_d_b11, eq35_e526_d_b12, eq35_e526_d_b13, eq35_e526_d_b14, eq35_e526_d_b15, eq35_e526_q, eq35_e526_q_d_n0, eq35_e526_q_d_n1, eq35_e526_q_d_n2, eq35_e526_q_d_n3, eq35_e526_q_d_n4, eq35_e526_q_d_n5, eq35_e526_q_d_n6, eq35_e526_q_d_n7, eq35_e526_q_d_n8, eq35_e526_q_d_n9, eq35_e526_q_d_n10, eq35_e526_q_d_n11, eq35_e526_q_d_n12, eq35_e526_q_d_n13, eq35_e526_q_d_n14, eq35_e526_q_d_n15, eq35_e526_q_d_n16, eq35_e526_q_d_n17, eq35_e526_q_d_n18, eq35_e526_q_d_b0, eq35_e526_q_d_b1, eq35_e526_q_d_b2, eq35_e526_q_d_b3, eq35_e526_q_d_b4, eq35_e526_q_d_b5, eq35_e526_q_d_b6, eq35_e526_q_d_b7, eq35_e526_q_d_b8, eq35_e526_q_d_b9, eq35_e526_q_d_b10, eq35_e526_q_d_b11, eq35_e526_q_d_b12, eq35_e526_q_d_b13, eq35_e526_q_d_b14, eq35_e526_q_d_b15,) = {
    if s.b[1851] {
        let eq35_e522_q: f64 = s.v[283];let eq35_e523: f64 = (s.v[281] + s.v[283]);let eq35_e523_d_n0: f64 = (s.dn[281][0] + s.dn[283][0]);let eq35_e523_d_n1: f64 = (s.dn[281][1] + s.dn[283][1]);let eq35_e523_d_n2: f64 = (s.dn[281][2] + s.dn[283][2]);let eq35_e523_d_n3: f64 = (s.dn[281][3] + s.dn[283][3]);let eq35_e523_d_n4: f64 = (s.dn[281][4] + s.dn[283][4]);let eq35_e523_d_n5: f64 = (s.dn[281][5] + s.dn[283][5]);let eq35_e523_d_n6: f64 = (s.dn[281][6] + s.dn[283][6]);let eq35_e523_d_n7: f64 = (s.dn[281][7] + s.dn[283][7]);let eq35_e523_d_n8: f64 = (s.dn[281][8] + s.dn[283][8]);let eq35_e523_d_n9: f64 = (s.dn[281][9] + s.dn[283][9]);let eq35_e523_d_n10: f64 = (s.dn[281][10] + s.dn[283][10]);let eq35_e523_d_n11: f64 = (s.dn[281][11] + s.dn[283][11]);let eq35_e523_d_n12: f64 = (s.dn[281][12] + s.dn[283][12]);let eq35_e523_d_n13: f64 = (s.dn[281][13] + s.dn[283][13]);let eq35_e523_d_n14: f64 = (s.dn[281][14] + s.dn[283][14]);let eq35_e523_d_n15: f64 = (s.dn[281][15] + s.dn[283][15]);let eq35_e523_d_n16: f64 = (s.dn[281][16] + s.dn[283][16]);let eq35_e523_d_n17: f64 = (s.dn[281][17] + s.dn[283][17]);let eq35_e523_d_n18: f64 = (s.dn[281][18] + s.dn[283][18]);let eq35_e523_d_b0: f64 = (s.db[281][0] + s.db[283][0]);let eq35_e523_d_b1: f64 = (s.db[281][1] + s.db[283][1]);let eq35_e523_d_b2: f64 = (s.db[281][2] + s.db[283][2]);let eq35_e523_d_b3: f64 = (s.db[281][3] + s.db[283][3]);let eq35_e523_d_b4: f64 = (s.db[281][4] + s.db[283][4]);let eq35_e523_d_b5: f64 = (s.db[281][5] + s.db[283][5]);let eq35_e523_d_b6: f64 = (s.db[281][6] + s.db[283][6]);let eq35_e523_d_b7: f64 = (s.db[281][7] + s.db[283][7]);let eq35_e523_d_b8: f64 = (s.db[281][8] + s.db[283][8]);let eq35_e523_d_b9: f64 = (s.db[281][9] + s.db[283][9]);let eq35_e523_d_b10: f64 = (s.db[281][10] + s.db[283][10]);let eq35_e523_d_b11: f64 = (s.db[281][11] + s.db[283][11]);let eq35_e523_d_b12: f64 = (s.db[281][12] + s.db[283][12]);let eq35_e523_d_b13: f64 = (s.db[281][13] + s.db[283][13]);let eq35_e523_d_b14: f64 = (s.db[281][14] + s.db[283][14]);let eq35_e523_d_b15: f64 = (s.db[281][15] + s.db[283][15]);let eq35_e523_q: f64 = eq35_e522_q;let eq35_e524: f64 = (p.p50 * eq35_e523);let eq35_e524_d_n0: f64 = (p.p50 * eq35_e523_d_n0);let eq35_e524_d_n1: f64 = (p.p50 * eq35_e523_d_n1);let eq35_e524_d_n2: f64 = (p.p50 * eq35_e523_d_n2);let eq35_e524_d_n3: f64 = (p.p50 * eq35_e523_d_n3);let eq35_e524_d_n4: f64 = (p.p50 * eq35_e523_d_n4);let eq35_e524_d_n5: f64 = (p.p50 * eq35_e523_d_n5);let eq35_e524_d_n6: f64 = (p.p50 * eq35_e523_d_n6);let eq35_e524_d_n7: f64 = (p.p50 * eq35_e523_d_n7);let eq35_e524_d_n8: f64 = (p.p50 * eq35_e523_d_n8);let eq35_e524_d_n9: f64 = (p.p50 * eq35_e523_d_n9);let eq35_e524_d_n10: f64 = (p.p50 * eq35_e523_d_n10);let eq35_e524_d_n11: f64 = (p.p50 * eq35_e523_d_n11);let eq35_e524_d_n12: f64 = (p.p50 * eq35_e523_d_n12);let eq35_e524_d_n13: f64 = (p.p50 * eq35_e523_d_n13);let eq35_e524_d_n14: f64 = (p.p50 * eq35_e523_d_n14);let eq35_e524_d_n15: f64 = (p.p50 * eq35_e523_d_n15);let eq35_e524_d_n16: f64 = (p.p50 * eq35_e523_d_n16);let eq35_e524_d_n17: f64 = (p.p50 * eq35_e523_d_n17);let eq35_e524_d_n18: f64 = (p.p50 * eq35_e523_d_n18);let eq35_e524_d_b0: f64 = (p.p50 * eq35_e523_d_b0);let eq35_e524_d_b1: f64 = (p.p50 * eq35_e523_d_b1);let eq35_e524_d_b2: f64 = (p.p50 * eq35_e523_d_b2);let eq35_e524_d_b3: f64 = (p.p50 * eq35_e523_d_b3);let eq35_e524_d_b4: f64 = (p.p50 * eq35_e523_d_b4);let eq35_e524_d_b5: f64 = (p.p50 * eq35_e523_d_b5);let eq35_e524_d_b6: f64 = (p.p50 * eq35_e523_d_b6);let eq35_e524_d_b7: f64 = (p.p50 * eq35_e523_d_b7);let eq35_e524_d_b8: f64 = (p.p50 * eq35_e523_d_b8);let eq35_e524_d_b9: f64 = (p.p50 * eq35_e523_d_b9);let eq35_e524_d_b10: f64 = (p.p50 * eq35_e523_d_b10);let eq35_e524_d_b11: f64 = (p.p50 * eq35_e523_d_b11);let eq35_e524_d_b12: f64 = (p.p50 * eq35_e523_d_b12);let eq35_e524_d_b13: f64 = (p.p50 * eq35_e523_d_b13);let eq35_e524_d_b14: f64 = (p.p50 * eq35_e523_d_b14);let eq35_e524_d_b15: f64 = (p.p50 * eq35_e523_d_b15);let eq35_e524_q: f64 = (p.p50 * eq35_e523_q);
        (eq35_e524, eq35_e524_d_n0, eq35_e524_d_n1, eq35_e524_d_n2, eq35_e524_d_n3, eq35_e524_d_n4, eq35_e524_d_n5, eq35_e524_d_n6, eq35_e524_d_n7, eq35_e524_d_n8, eq35_e524_d_n9, eq35_e524_d_n10, eq35_e524_d_n11, eq35_e524_d_n12, eq35_e524_d_n13, eq35_e524_d_n14, eq35_e524_d_n15, eq35_e524_d_n16, eq35_e524_d_n17, eq35_e524_d_n18, eq35_e524_d_b0, eq35_e524_d_b1, eq35_e524_d_b2, eq35_e524_d_b3, eq35_e524_d_b4, eq35_e524_d_b5, eq35_e524_d_b6, eq35_e524_d_b7, eq35_e524_d_b8, eq35_e524_d_b9, eq35_e524_d_b10, eq35_e524_d_b11, eq35_e524_d_b12, eq35_e524_d_b13, eq35_e524_d_b14, eq35_e524_d_b15, eq35_e524_q, (p.p50 * s.dn[283][0]), (p.p50 * s.dn[283][1]), (p.p50 * s.dn[283][2]), (p.p50 * s.dn[283][3]), (p.p50 * s.dn[283][4]), (p.p50 * s.dn[283][5]), (p.p50 * s.dn[283][6]), (p.p50 * s.dn[283][7]), (p.p50 * s.dn[283][8]), (p.p50 * s.dn[283][9]), (p.p50 * s.dn[283][10]), (p.p50 * s.dn[283][11]), (p.p50 * s.dn[283][12]), (p.p50 * s.dn[283][13]), (p.p50 * s.dn[283][14]), (p.p50 * s.dn[283][15]), (p.p50 * s.dn[283][16]), (p.p50 * s.dn[283][17]), (p.p50 * s.dn[283][18]), (p.p50 * s.db[283][0]), (p.p50 * s.db[283][1]), (p.p50 * s.db[283][2]), (p.p50 * s.db[283][3]), (p.p50 * s.db[283][4]), (p.p50 * s.db[283][5]), (p.p50 * s.db[283][6]), (p.p50 * s.db[283][7]), (p.p50 * s.db[283][8]), (p.p50 * s.db[283][9]), (p.p50 * s.db[283][10]), (p.p50 * s.db[283][11]), (p.p50 * s.db[283][12]), (p.p50 * s.db[283][13]), (p.p50 * s.db[283][14]), (p.p50 * s.db[283][15]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_reactive_node_derivatives: [f64; 19] = [eq35_e526_q_d_n0, eq35_e526_q_d_n1, eq35_e526_q_d_n2, eq35_e526_q_d_n3, eq35_e526_q_d_n4, eq35_e526_q_d_n5, eq35_e526_q_d_n6, eq35_e526_q_d_n7, eq35_e526_q_d_n8, eq35_e526_q_d_n9, eq35_e526_q_d_n10, eq35_e526_q_d_n11, eq35_e526_q_d_n12, eq35_e526_q_d_n13, eq35_e526_q_d_n14, eq35_e526_q_d_n15, eq35_e526_q_d_n16, eq35_e526_q_d_n17, eq35_e526_q_d_n18];let eq35_reactive_branch_derivatives: [f64; 16] = [eq35_e526_q_d_b0, eq35_e526_q_d_b1, eq35_e526_q_d_b2, eq35_e526_q_d_b3, eq35_e526_q_d_b4, eq35_e526_q_d_b5, eq35_e526_q_d_b6, eq35_e526_q_d_b7, eq35_e526_q_d_b8, eq35_e526_q_d_b9, eq35_e526_q_d_b10, eq35_e526_q_d_b11, eq35_e526_q_d_b12, eq35_e526_q_d_b13, eq35_e526_q_d_b14, eq35_e526_q_d_b15];
        stamper.stamp_current_reactive_dense_local(
            Some(9),
            Some(7),
            &eq35_reactive_node_derivatives,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv13 = ctx.node_voltage(nodes[13]);let nv17 = ctx.node_voltage(nodes[17]);let nv18 = ctx.node_voltage(nodes[18]);
        let (eq36_e535, eq36_e535_d_n0, eq36_e535_d_n1, eq36_e535_d_n2, eq36_e535_d_n3, eq36_e535_d_n4, eq36_e535_d_n5, eq36_e535_d_n6, eq36_e535_d_n7, eq36_e535_d_n8, eq36_e535_d_n9, eq36_e535_d_n10, eq36_e535_d_n11, eq36_e535_d_n12, eq36_e535_d_n13, eq36_e535_d_n14, eq36_e535_d_n15, eq36_e535_d_n16, eq36_e535_d_n17, eq36_e535_d_n18, eq36_e535_d_b0, eq36_e535_d_b1, eq36_e535_d_b2, eq36_e535_d_b3, eq36_e535_d_b4, eq36_e535_d_b5, eq36_e535_d_b6, eq36_e535_d_b7, eq36_e535_d_b8, eq36_e535_d_b9, eq36_e535_d_b10, eq36_e535_d_b11, eq36_e535_d_b12, eq36_e535_d_b13, eq36_e535_d_b14, eq36_e535_d_b15, eq36_e535_q, eq36_e535_q_d_n0, eq36_e535_q_d_n1, eq36_e535_q_d_n2, eq36_e535_q_d_n3, eq36_e535_q_d_n4, eq36_e535_q_d_n5, eq36_e535_q_d_n6, eq36_e535_q_d_n7, eq36_e535_q_d_n8, eq36_e535_q_d_n9, eq36_e535_q_d_n10, eq36_e535_q_d_n11, eq36_e535_q_d_n12, eq36_e535_q_d_n13, eq36_e535_q_d_n14, eq36_e535_q_d_n15, eq36_e535_q_d_n16, eq36_e535_q_d_n17, eq36_e535_q_d_n18, eq36_e535_q_d_b0, eq36_e535_q_d_b1, eq36_e535_q_d_b2, eq36_e535_q_d_b3, eq36_e535_q_d_b4, eq36_e535_q_d_b5, eq36_e535_q_d_b6, eq36_e535_q_d_b7, eq36_e535_q_d_b8, eq36_e535_q_d_b9, eq36_e535_q_d_b10, eq36_e535_q_d_b11, eq36_e535_q_d_b12, eq36_e535_q_d_b13, eq36_e535_q_d_b14, eq36_e535_q_d_b15,) = {
    if s.b[1851] {
        let eq36_e531_q: f64 = s.v[284];let eq36_e532: f64 = (s.v[282] + s.v[284]);let eq36_e532_d_n0: f64 = (s.dn[282][0] + s.dn[284][0]);let eq36_e532_d_n1: f64 = (s.dn[282][1] + s.dn[284][1]);let eq36_e532_d_n2: f64 = (s.dn[282][2] + s.dn[284][2]);let eq36_e532_d_n3: f64 = (s.dn[282][3] + s.dn[284][3]);let eq36_e532_d_n4: f64 = (s.dn[282][4] + s.dn[284][4]);let eq36_e532_d_n5: f64 = (s.dn[282][5] + s.dn[284][5]);let eq36_e532_d_n6: f64 = (s.dn[282][6] + s.dn[284][6]);let eq36_e532_d_n7: f64 = (s.dn[282][7] + s.dn[284][7]);let eq36_e532_d_n8: f64 = (s.dn[282][8] + s.dn[284][8]);let eq36_e532_d_n9: f64 = (s.dn[282][9] + s.dn[284][9]);let eq36_e532_d_n10: f64 = (s.dn[282][10] + s.dn[284][10]);let eq36_e532_d_n11: f64 = (s.dn[282][11] + s.dn[284][11]);let eq36_e532_d_n12: f64 = (s.dn[282][12] + s.dn[284][12]);let eq36_e532_d_n13: f64 = (s.dn[282][13] + s.dn[284][13]);let eq36_e532_d_n14: f64 = (s.dn[282][14] + s.dn[284][14]);let eq36_e532_d_n15: f64 = (s.dn[282][15] + s.dn[284][15]);let eq36_e532_d_n16: f64 = (s.dn[282][16] + s.dn[284][16]);let eq36_e532_d_n17: f64 = (s.dn[282][17] + s.dn[284][17]);let eq36_e532_d_n18: f64 = (s.dn[282][18] + s.dn[284][18]);let eq36_e532_d_b0: f64 = (s.db[282][0] + s.db[284][0]);let eq36_e532_d_b1: f64 = (s.db[282][1] + s.db[284][1]);let eq36_e532_d_b2: f64 = (s.db[282][2] + s.db[284][2]);let eq36_e532_d_b3: f64 = (s.db[282][3] + s.db[284][3]);let eq36_e532_d_b4: f64 = (s.db[282][4] + s.db[284][4]);let eq36_e532_d_b5: f64 = (s.db[282][5] + s.db[284][5]);let eq36_e532_d_b6: f64 = (s.db[282][6] + s.db[284][6]);let eq36_e532_d_b7: f64 = (s.db[282][7] + s.db[284][7]);let eq36_e532_d_b8: f64 = (s.db[282][8] + s.db[284][8]);let eq36_e532_d_b9: f64 = (s.db[282][9] + s.db[284][9]);let eq36_e532_d_b10: f64 = (s.db[282][10] + s.db[284][10]);let eq36_e532_d_b11: f64 = (s.db[282][11] + s.db[284][11]);let eq36_e532_d_b12: f64 = (s.db[282][12] + s.db[284][12]);let eq36_e532_d_b13: f64 = (s.db[282][13] + s.db[284][13]);let eq36_e532_d_b14: f64 = (s.db[282][14] + s.db[284][14]);let eq36_e532_d_b15: f64 = (s.db[282][15] + s.db[284][15]);let eq36_e532_q: f64 = eq36_e531_q;let eq36_e533: f64 = (p.p50 * eq36_e532);let eq36_e533_d_n0: f64 = (p.p50 * eq36_e532_d_n0);let eq36_e533_d_n1: f64 = (p.p50 * eq36_e532_d_n1);let eq36_e533_d_n2: f64 = (p.p50 * eq36_e532_d_n2);let eq36_e533_d_n3: f64 = (p.p50 * eq36_e532_d_n3);let eq36_e533_d_n4: f64 = (p.p50 * eq36_e532_d_n4);let eq36_e533_d_n5: f64 = (p.p50 * eq36_e532_d_n5);let eq36_e533_d_n6: f64 = (p.p50 * eq36_e532_d_n6);let eq36_e533_d_n7: f64 = (p.p50 * eq36_e532_d_n7);let eq36_e533_d_n8: f64 = (p.p50 * eq36_e532_d_n8);let eq36_e533_d_n9: f64 = (p.p50 * eq36_e532_d_n9);let eq36_e533_d_n10: f64 = (p.p50 * eq36_e532_d_n10);let eq36_e533_d_n11: f64 = (p.p50 * eq36_e532_d_n11);let eq36_e533_d_n12: f64 = (p.p50 * eq36_e532_d_n12);let eq36_e533_d_n13: f64 = (p.p50 * eq36_e532_d_n13);let eq36_e533_d_n14: f64 = (p.p50 * eq36_e532_d_n14);let eq36_e533_d_n15: f64 = (p.p50 * eq36_e532_d_n15);let eq36_e533_d_n16: f64 = (p.p50 * eq36_e532_d_n16);let eq36_e533_d_n17: f64 = (p.p50 * eq36_e532_d_n17);let eq36_e533_d_n18: f64 = (p.p50 * eq36_e532_d_n18);let eq36_e533_d_b0: f64 = (p.p50 * eq36_e532_d_b0);let eq36_e533_d_b1: f64 = (p.p50 * eq36_e532_d_b1);let eq36_e533_d_b2: f64 = (p.p50 * eq36_e532_d_b2);let eq36_e533_d_b3: f64 = (p.p50 * eq36_e532_d_b3);let eq36_e533_d_b4: f64 = (p.p50 * eq36_e532_d_b4);let eq36_e533_d_b5: f64 = (p.p50 * eq36_e532_d_b5);let eq36_e533_d_b6: f64 = (p.p50 * eq36_e532_d_b6);let eq36_e533_d_b7: f64 = (p.p50 * eq36_e532_d_b7);let eq36_e533_d_b8: f64 = (p.p50 * eq36_e532_d_b8);let eq36_e533_d_b9: f64 = (p.p50 * eq36_e532_d_b9);let eq36_e533_d_b10: f64 = (p.p50 * eq36_e532_d_b10);let eq36_e533_d_b11: f64 = (p.p50 * eq36_e532_d_b11);let eq36_e533_d_b12: f64 = (p.p50 * eq36_e532_d_b12);let eq36_e533_d_b13: f64 = (p.p50 * eq36_e532_d_b13);let eq36_e533_d_b14: f64 = (p.p50 * eq36_e532_d_b14);let eq36_e533_d_b15: f64 = (p.p50 * eq36_e532_d_b15);let eq36_e533_q: f64 = (p.p50 * eq36_e532_q);
        (eq36_e533, eq36_e533_d_n0, eq36_e533_d_n1, eq36_e533_d_n2, eq36_e533_d_n3, eq36_e533_d_n4, eq36_e533_d_n5, eq36_e533_d_n6, eq36_e533_d_n7, eq36_e533_d_n8, eq36_e533_d_n9, eq36_e533_d_n10, eq36_e533_d_n11, eq36_e533_d_n12, eq36_e533_d_n13, eq36_e533_d_n14, eq36_e533_d_n15, eq36_e533_d_n16, eq36_e533_d_n17, eq36_e533_d_n18, eq36_e533_d_b0, eq36_e533_d_b1, eq36_e533_d_b2, eq36_e533_d_b3, eq36_e533_d_b4, eq36_e533_d_b5, eq36_e533_d_b6, eq36_e533_d_b7, eq36_e533_d_b8, eq36_e533_d_b9, eq36_e533_d_b10, eq36_e533_d_b11, eq36_e533_d_b12, eq36_e533_d_b13, eq36_e533_d_b14, eq36_e533_d_b15, eq36_e533_q, (p.p50 * s.dn[284][0]), (p.p50 * s.dn[284][1]), (p.p50 * s.dn[284][2]), (p.p50 * s.dn[284][3]), (p.p50 * s.dn[284][4]), (p.p50 * s.dn[284][5]), (p.p50 * s.dn[284][6]), (p.p50 * s.dn[284][7]), (p.p50 * s.dn[284][8]), (p.p50 * s.dn[284][9]), (p.p50 * s.dn[284][10]), (p.p50 * s.dn[284][11]), (p.p50 * s.dn[284][12]), (p.p50 * s.dn[284][13]), (p.p50 * s.dn[284][14]), (p.p50 * s.dn[284][15]), (p.p50 * s.dn[284][16]), (p.p50 * s.dn[284][17]), (p.p50 * s.dn[284][18]), (p.p50 * s.db[284][0]), (p.p50 * s.db[284][1]), (p.p50 * s.db[284][2]), (p.p50 * s.db[284][3]), (p.p50 * s.db[284][4]), (p.p50 * s.db[284][5]), (p.p50 * s.db[284][6]), (p.p50 * s.db[284][7]), (p.p50 * s.db[284][8]), (p.p50 * s.db[284][9]), (p.p50 * s.db[284][10]), (p.p50 * s.db[284][11]), (p.p50 * s.db[284][12]), (p.p50 * s.db[284][13]), (p.p50 * s.db[284][14]), (p.p50 * s.db[284][15]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_reactive_node_derivatives: [f64; 19] = [eq36_e535_q_d_n0, eq36_e535_q_d_n1, eq36_e535_q_d_n2, eq36_e535_q_d_n3, eq36_e535_q_d_n4, eq36_e535_q_d_n5, eq36_e535_q_d_n6, eq36_e535_q_d_n7, eq36_e535_q_d_n8, eq36_e535_q_d_n9, eq36_e535_q_d_n10, eq36_e535_q_d_n11, eq36_e535_q_d_n12, eq36_e535_q_d_n13, eq36_e535_q_d_n14, eq36_e535_q_d_n15, eq36_e535_q_d_n16, eq36_e535_q_d_n17, eq36_e535_q_d_n18];let eq36_reactive_branch_derivatives: [f64; 16] = [eq36_e535_q_d_b0, eq36_e535_q_d_b1, eq36_e535_q_d_b2, eq36_e535_q_d_b3, eq36_e535_q_d_b4, eq36_e535_q_d_b5, eq36_e535_q_d_b6, eq36_e535_q_d_b7, eq36_e535_q_d_b8, eq36_e535_q_d_b9, eq36_e535_q_d_b10, eq36_e535_q_d_b11, eq36_e535_q_d_b12, eq36_e535_q_d_b13, eq36_e535_q_d_b14, eq36_e535_q_d_b15];
        stamper.stamp_current_reactive_dense_local(
            Some(8),
            Some(6),
            &eq36_reactive_node_derivatives,
            &eq36_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq47_e619, eq47_e619_d_n18, eq47_e619_q,) = {
    if (s.b[1851] && (p.p34 != 0.0)) {
        let eq47_e614: f64 = (1e-9 / 0.0001);let eq47_e616: f64 = (eq47_e614 * (nv18 - 0.0));let eq47_e617_q: f64 = eq47_e616;
        (eq47_e616, eq47_e614, eq47_e617_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(18),
            None,
            18,
            multiplicity * (eq47_e619_d_n18),
        );
        let (eq48_e630, eq48_e630_d_n13, eq48_e630_q,) = {
    if (s.b[1851] && (p.p34 != 0.0)) {
        let eq48_e625: f64 = (1e-9 / 0.0001);let eq48_e627: f64 = (eq48_e625 * (nv13 - 0.0));let eq48_e628_q: f64 = eq48_e627;
        (eq48_e627, eq48_e625, eq48_e628_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(13),
            None,
            13,
            multiplicity * (eq48_e630_d_n13),
        );
        let (eq53_e669, eq53_e669_d_n17, eq53_e669_q,) = {
    if (s.b[1851] && s.b[1852]) {
        let eq53_e664: f64 = (1e-9 / 0.0001);let eq53_e666: f64 = (eq53_e664 * (nv17 - 0.0));let eq53_e667_q: f64 = eq53_e666;
        (eq53_e666, eq53_e664, eq53_e667_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(17),
            None,
            17,
            multiplicity * (eq53_e669_d_n17),
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv13 = ctx.node_voltage(nodes[13]);let nv15 = ctx.node_voltage(nodes[15]);let nv16 = ctx.node_voltage(nodes[16]);let nv17 = ctx.node_voltage(nodes[17]);
        let (eq60_e727, eq60_e727_d_n17, eq60_e727_q,) = {
    if ((!s.b[1851]) && (p.p37 != 0.0)) {
        let eq60_e722: f64 = (1e-9 / 0.0001);let eq60_e724: f64 = (eq60_e722 * (nv17 - 0.0));let eq60_e725_q: f64 = eq60_e724;
        (eq60_e724, eq60_e722, eq60_e725_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(17),
            None,
            17,
            multiplicity * (eq60_e727_d_n17),
        );
        let (eq68_e795, eq68_e795_d_n15, eq68_e795_q,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        let eq68_e790: f64 = (1e-9 / 0.0001);let eq68_e792: f64 = (eq68_e790 * (nv15 - 0.0));let eq68_e793_q: f64 = eq68_e792;
        (eq68_e792, eq68_e790, eq68_e793_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(15),
            None,
            15,
            multiplicity * (eq68_e795_d_n15),
        );
        let (eq69_e807, eq69_e807_d_n16, eq69_e807_q,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        let eq69_e802: f64 = (1e-9 / 0.0001);let eq69_e804: f64 = (eq69_e802 * (nv16 - 0.0));let eq69_e805_q: f64 = eq69_e804;
        (eq69_e804, eq69_e802, eq69_e805_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(16),
            None,
            16,
            multiplicity * (eq69_e807_d_n16),
        );
        let (eq70_e819, eq70_e819_d_n13, eq70_e819_q,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        let eq70_e814: f64 = (1e-9 / 0.0001);let eq70_e816: f64 = (eq70_e814 * (nv13 - 0.0));let eq70_e817_q: f64 = eq70_e816;
        (eq70_e816, eq70_e814, eq70_e817_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(13),
            None,
            13,
            multiplicity * (eq70_e819_d_n13),
        );
    }
}
