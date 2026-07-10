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
        let (eq39_e555, eq39_e555_d_n0, eq39_e555_d_n1, eq39_e555_d_n2, eq39_e555_d_n3, eq39_e555_d_n4, eq39_e555_d_n5, eq39_e555_d_n6, eq39_e555_d_n7, eq39_e555_d_n8, eq39_e555_d_n9, eq39_e555_d_n10, eq39_e555_d_n11, eq39_e555_d_n12, eq39_e555_d_n13, eq39_e555_d_n14, eq39_e555_d_n15, eq39_e555_d_n16, eq39_e555_d_n17, eq39_e555_d_n18, eq39_e555_d_b0, eq39_e555_d_b1, eq39_e555_d_b2, eq39_e555_d_b3, eq39_e555_d_b4, eq39_e555_d_b5, eq39_e555_d_b6, eq39_e555_d_b7, eq39_e555_d_b8, eq39_e555_d_b9, eq39_e555_d_b10, eq39_e555_d_b11, eq39_e555_d_b12, eq39_e555_d_b13, eq39_e555_d_b14,) = {
    if (s.b[1847] && (p.p262 != 0.0)) {
        let eq39_e553: f64 = (s.v[552] * (nv8 - nv12));let eq39_e553_d_n0: f64 = (s.dn[552][0] * (nv8 - nv12));let eq39_e553_d_n1: f64 = (s.dn[552][1] * (nv8 - nv12));let eq39_e553_d_n2: f64 = (s.dn[552][2] * (nv8 - nv12));let eq39_e553_d_n3: f64 = (s.dn[552][3] * (nv8 - nv12));let eq39_e553_d_n4: f64 = (s.dn[552][4] * (nv8 - nv12));let eq39_e553_d_n5: f64 = (s.dn[552][5] * (nv8 - nv12));let eq39_e553_d_n6: f64 = (s.dn[552][6] * (nv8 - nv12));let eq39_e553_d_n7: f64 = (s.dn[552][7] * (nv8 - nv12));let eq39_e553_d_n8: f64 = ((s.dn[552][8] * (nv8 - nv12)) + s.v[552]);let eq39_e553_d_n9: f64 = (s.dn[552][9] * (nv8 - nv12));let eq39_e553_d_n10: f64 = (s.dn[552][10] * (nv8 - nv12));let eq39_e553_d_n11: f64 = (s.dn[552][11] * (nv8 - nv12));let eq39_e553_d_n12: f64 = ((s.dn[552][12] * (nv8 - nv12)) + (-s.v[552]));let eq39_e553_d_n13: f64 = (s.dn[552][13] * (nv8 - nv12));let eq39_e553_d_n14: f64 = (s.dn[552][14] * (nv8 - nv12));let eq39_e553_d_n15: f64 = (s.dn[552][15] * (nv8 - nv12));let eq39_e553_d_n16: f64 = (s.dn[552][16] * (nv8 - nv12));let eq39_e553_d_n17: f64 = (s.dn[552][17] * (nv8 - nv12));let eq39_e553_d_n18: f64 = (s.dn[552][18] * (nv8 - nv12));let eq39_e553_d_b0: f64 = (s.db[552][0] * (nv8 - nv12));let eq39_e553_d_b1: f64 = (s.db[552][1] * (nv8 - nv12));let eq39_e553_d_b2: f64 = (s.db[552][2] * (nv8 - nv12));let eq39_e553_d_b3: f64 = (s.db[552][3] * (nv8 - nv12));let eq39_e553_d_b4: f64 = (s.db[552][4] * (nv8 - nv12));let eq39_e553_d_b5: f64 = (s.db[552][5] * (nv8 - nv12));let eq39_e553_d_b6: f64 = (s.db[552][6] * (nv8 - nv12));let eq39_e553_d_b7: f64 = (s.db[552][7] * (nv8 - nv12));let eq39_e553_d_b8: f64 = (s.db[552][8] * (nv8 - nv12));let eq39_e553_d_b9: f64 = (s.db[552][9] * (nv8 - nv12));let eq39_e553_d_b10: f64 = (s.db[552][10] * (nv8 - nv12));let eq39_e553_d_b11: f64 = (s.db[552][11] * (nv8 - nv12));let eq39_e553_d_b12: f64 = (s.db[552][12] * (nv8 - nv12));let eq39_e553_d_b13: f64 = (s.db[552][13] * (nv8 - nv12));let eq39_e553_d_b14: f64 = (s.db[552][14] * (nv8 - nv12));
        (eq39_e553, eq39_e553_d_n0, eq39_e553_d_n1, eq39_e553_d_n2, eq39_e553_d_n3, eq39_e553_d_n4, eq39_e553_d_n5, eq39_e553_d_n6, eq39_e553_d_n7, eq39_e553_d_n8, eq39_e553_d_n9, eq39_e553_d_n10, eq39_e553_d_n11, eq39_e553_d_n12, eq39_e553_d_n13, eq39_e553_d_n14, eq39_e553_d_n15, eq39_e553_d_n16, eq39_e553_d_n17, eq39_e553_d_n18, eq39_e553_d_b0, eq39_e553_d_b1, eq39_e553_d_b2, eq39_e553_d_b3, eq39_e553_d_b4, eq39_e553_d_b5, eq39_e553_d_b6, eq39_e553_d_b7, eq39_e553_d_b8, eq39_e553_d_b9, eq39_e553_d_b10, eq39_e553_d_b11, eq39_e553_d_b12, eq39_e553_d_b13, eq39_e553_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e555;let eq39_node_derivatives: [f64; 19] = [eq39_e555_d_n0, eq39_e555_d_n1, eq39_e555_d_n2, eq39_e555_d_n3, eq39_e555_d_n4, eq39_e555_d_n5, eq39_e555_d_n6, eq39_e555_d_n7, eq39_e555_d_n8, eq39_e555_d_n9, eq39_e555_d_n10, eq39_e555_d_n11, eq39_e555_d_n12, eq39_e555_d_n13, eq39_e555_d_n14, eq39_e555_d_n15, eq39_e555_d_n16, eq39_e555_d_n17, eq39_e555_d_n18];let eq39_branch_derivatives: [f64; 15] = [eq39_e555_d_b0, eq39_e555_d_b1, eq39_e555_d_b2, eq39_e555_d_b3, eq39_e555_d_b4, eq39_e555_d_b5, eq39_e555_d_b6, eq39_e555_d_b7, eq39_e555_d_b8, eq39_e555_d_b9, eq39_e555_d_b10, eq39_e555_d_b11, eq39_e555_d_b12, eq39_e555_d_b13, eq39_e555_d_b14];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(12),
            multiplicity * (eq39_value),
            &eq39_node_derivatives,
            &eq39_branch_derivatives,
            multiplicity,
        );
        let (eq40_e562,) = {
    if (s.b[1847] && (p.p262 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq40_value: f64 = eq40_e562;
        stamper.stamp_potential_const_local(
            6,
            eq40_value,
        );
        let (eq41_e569,) = {
    if (s.b[1847] && (p.p262 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq41_value: f64 = eq41_e569;
        stamper.stamp_potential_const_local(
            7,
            eq41_value,
        );
        let (eq42_e575, eq42_e575_d_n0, eq42_e575_d_n1, eq42_e575_d_n2, eq42_e575_d_n3, eq42_e575_d_n4, eq42_e575_d_n5, eq42_e575_d_n6, eq42_e575_d_n7, eq42_e575_d_n8, eq42_e575_d_n9, eq42_e575_d_n10, eq42_e575_d_n11, eq42_e575_d_n12, eq42_e575_d_n13, eq42_e575_d_n14, eq42_e575_d_n15, eq42_e575_d_n16, eq42_e575_d_n17, eq42_e575_d_n18, eq42_e575_d_b0, eq42_e575_d_b1, eq42_e575_d_b2, eq42_e575_d_b3, eq42_e575_d_b4, eq42_e575_d_b5, eq42_e575_d_b6, eq42_e575_d_b7, eq42_e575_d_b8, eq42_e575_d_b9, eq42_e575_d_b10, eq42_e575_d_b11, eq42_e575_d_b12, eq42_e575_d_b13, eq42_e575_d_b14,) = {
    if (s.b[1847] && (p.p34 != 0.0)) {
        (s.v[582], s.dn[582][0], s.dn[582][1], s.dn[582][2], s.dn[582][3], s.dn[582][4], s.dn[582][5], s.dn[582][6], s.dn[582][7], s.dn[582][8], s.dn[582][9], s.dn[582][10], s.dn[582][11], s.dn[582][12], s.dn[582][13], s.dn[582][14], s.dn[582][15], s.dn[582][16], s.dn[582][17], s.dn[582][18], s.db[582][0], s.db[582][1], s.db[582][2], s.db[582][3], s.db[582][4], s.db[582][5], s.db[582][6], s.db[582][7], s.db[582][8], s.db[582][9], s.db[582][10], s.db[582][11], s.db[582][12], s.db[582][13], s.db[582][14],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq42_value: f64 = eq42_e575;let eq42_node_derivatives: [f64; 19] = [eq42_e575_d_n0, eq42_e575_d_n1, eq42_e575_d_n2, eq42_e575_d_n3, eq42_e575_d_n4, eq42_e575_d_n5, eq42_e575_d_n6, eq42_e575_d_n7, eq42_e575_d_n8, eq42_e575_d_n9, eq42_e575_d_n10, eq42_e575_d_n11, eq42_e575_d_n12, eq42_e575_d_n13, eq42_e575_d_n14, eq42_e575_d_n15, eq42_e575_d_n16, eq42_e575_d_n17, eq42_e575_d_n18];let eq42_branch_derivatives: [f64; 15] = [eq42_e575_d_b0, eq42_e575_d_b1, eq42_e575_d_b2, eq42_e575_d_b3, eq42_e575_d_b4, eq42_e575_d_b5, eq42_e575_d_b6, eq42_e575_d_b7, eq42_e575_d_b8, eq42_e575_d_b9, eq42_e575_d_b10, eq42_e575_d_b11, eq42_e575_d_b12, eq42_e575_d_b13, eq42_e575_d_b14];
        stamper.stamp_current_dense_local(
            Some(18),
            None,
            multiplicity * (eq42_value),
            &eq42_node_derivatives,
            &eq42_branch_derivatives,
            multiplicity,
        );
        let (eq43_e581, eq43_e581_d_n0, eq43_e581_d_n1, eq43_e581_d_n2, eq43_e581_d_n3, eq43_e581_d_n4, eq43_e581_d_n5, eq43_e581_d_n6, eq43_e581_d_n7, eq43_e581_d_n8, eq43_e581_d_n9, eq43_e581_d_n10, eq43_e581_d_n11, eq43_e581_d_n12, eq43_e581_d_n13, eq43_e581_d_n14, eq43_e581_d_n15, eq43_e581_d_n16, eq43_e581_d_n17, eq43_e581_d_n18, eq43_e581_d_b0, eq43_e581_d_b1, eq43_e581_d_b2, eq43_e581_d_b3, eq43_e581_d_b4, eq43_e581_d_b5, eq43_e581_d_b6, eq43_e581_d_b7, eq43_e581_d_b8, eq43_e581_d_b9, eq43_e581_d_b10, eq43_e581_d_b11, eq43_e581_d_b12, eq43_e581_d_b13, eq43_e581_d_b14,) = {
    if (s.b[1847] && (p.p34 != 0.0)) {
        (s.v[583], s.dn[583][0], s.dn[583][1], s.dn[583][2], s.dn[583][3], s.dn[583][4], s.dn[583][5], s.dn[583][6], s.dn[583][7], s.dn[583][8], s.dn[583][9], s.dn[583][10], s.dn[583][11], s.dn[583][12], s.dn[583][13], s.dn[583][14], s.dn[583][15], s.dn[583][16], s.dn[583][17], s.dn[583][18], s.db[583][0], s.db[583][1], s.db[583][2], s.db[583][3], s.db[583][4], s.db[583][5], s.db[583][6], s.db[583][7], s.db[583][8], s.db[583][9], s.db[583][10], s.db[583][11], s.db[583][12], s.db[583][13], s.db[583][14],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq43_value: f64 = eq43_e581;let eq43_node_derivatives: [f64; 19] = [eq43_e581_d_n0, eq43_e581_d_n1, eq43_e581_d_n2, eq43_e581_d_n3, eq43_e581_d_n4, eq43_e581_d_n5, eq43_e581_d_n6, eq43_e581_d_n7, eq43_e581_d_n8, eq43_e581_d_n9, eq43_e581_d_n10, eq43_e581_d_n11, eq43_e581_d_n12, eq43_e581_d_n13, eq43_e581_d_n14, eq43_e581_d_n15, eq43_e581_d_n16, eq43_e581_d_n17, eq43_e581_d_n18];let eq43_branch_derivatives: [f64; 15] = [eq43_e581_d_b0, eq43_e581_d_b1, eq43_e581_d_b2, eq43_e581_d_b3, eq43_e581_d_b4, eq43_e581_d_b5, eq43_e581_d_b6, eq43_e581_d_b7, eq43_e581_d_b8, eq43_e581_d_b9, eq43_e581_d_b10, eq43_e581_d_b11, eq43_e581_d_b12, eq43_e581_d_b13, eq43_e581_d_b14];
        stamper.stamp_current_dense_local(
            Some(13),
            None,
            multiplicity * (eq43_value),
            &eq43_node_derivatives,
            &eq43_branch_derivatives,
            multiplicity,
        );
        let (eq44_e589, eq44_e589_d_n18,) = {
    if (s.b[1847] && (p.p34 != 0.0)) {
        let eq44_e587: f64 = ((nv18 - 0.0) * 1e-12);
        (eq44_e587, 1e-12,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e589;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (eq44_value),
            18,
            multiplicity * (eq44_e589_d_n18),
        );
        let (eq45_e597, eq45_e597_d_n13,) = {
    if (s.b[1847] && (p.p34 != 0.0)) {
        let eq45_e595: f64 = ((nv13 - 0.0) * 1e-12);
        (eq45_e595, 1e-12,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq45_value: f64 = eq45_e597;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq45_value),
            13,
            multiplicity * (eq45_e597_d_n13),
        );
        let (eq46_e608, eq46_e608_d_n18,) = {
    if (s.b[1847] && (p.p34 != 0.0)) {
        let eq46_e603: f64 = (1e-9 / 0.0001);let eq46_e605: f64 = (eq46_e603 * (nv18 - 0.0));let eq46_e606: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq46_e605);
        (eq46_e606, (eq46_e603 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e608;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (eq46_value),
            18,
            multiplicity * (eq46_e608_d_n18),
        );
        let (eq47_e619, eq47_e619_d_n13,) = {
    if (s.b[1847] && (p.p34 != 0.0)) {
        let eq47_e614: f64 = (1e-9 / 0.0001);let eq47_e616: f64 = (eq47_e614 * (nv13 - 0.0));let eq47_e617: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq47_e616);
        (eq47_e617, (eq47_e614 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e619;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq47_value),
            13,
            multiplicity * (eq47_e619_d_n13),
        );
        let (eq48_e626,) = {
    if (s.b[1847] && (p.p34 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq48_value: f64 = eq48_e626;
        stamper.stamp_potential_const_local(
            8,
            eq48_value,
        );
        let (eq49_e633,) = {
    if (s.b[1847] && (p.p34 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e633;
        stamper.stamp_potential_const_local(
            9,
            eq49_value,
        );
        let (eq50_e639, eq50_e639_d_n0, eq50_e639_d_n1, eq50_e639_d_n2, eq50_e639_d_n3, eq50_e639_d_n4, eq50_e639_d_n5, eq50_e639_d_n6, eq50_e639_d_n7, eq50_e639_d_n8, eq50_e639_d_n9, eq50_e639_d_n10, eq50_e639_d_n11, eq50_e639_d_n12, eq50_e639_d_n13, eq50_e639_d_n14, eq50_e639_d_n15, eq50_e639_d_n16, eq50_e639_d_n17, eq50_e639_d_n18, eq50_e639_d_b0, eq50_e639_d_b1, eq50_e639_d_b2, eq50_e639_d_b3, eq50_e639_d_b4, eq50_e639_d_b5, eq50_e639_d_b6, eq50_e639_d_b7, eq50_e639_d_b8, eq50_e639_d_b9, eq50_e639_d_b10, eq50_e639_d_b11, eq50_e639_d_b12, eq50_e639_d_b13, eq50_e639_d_b14,) = {
    if (s.b[1847] && s.b[1848]) {
        (s.v[592], s.dn[592][0], s.dn[592][1], s.dn[592][2], s.dn[592][3], s.dn[592][4], s.dn[592][5], s.dn[592][6], s.dn[592][7], s.dn[592][8], s.dn[592][9], s.dn[592][10], s.dn[592][11], s.dn[592][12], s.dn[592][13], s.dn[592][14], s.dn[592][15], s.dn[592][16], s.dn[592][17], s.dn[592][18], s.db[592][0], s.db[592][1], s.db[592][2], s.db[592][3], s.db[592][4], s.db[592][5], s.db[592][6], s.db[592][7], s.db[592][8], s.db[592][9], s.db[592][10], s.db[592][11], s.db[592][12], s.db[592][13], s.db[592][14],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e639;let eq50_node_derivatives: [f64; 19] = [eq50_e639_d_n0, eq50_e639_d_n1, eq50_e639_d_n2, eq50_e639_d_n3, eq50_e639_d_n4, eq50_e639_d_n5, eq50_e639_d_n6, eq50_e639_d_n7, eq50_e639_d_n8, eq50_e639_d_n9, eq50_e639_d_n10, eq50_e639_d_n11, eq50_e639_d_n12, eq50_e639_d_n13, eq50_e639_d_n14, eq50_e639_d_n15, eq50_e639_d_n16, eq50_e639_d_n17, eq50_e639_d_n18];let eq50_branch_derivatives: [f64; 15] = [eq50_e639_d_b0, eq50_e639_d_b1, eq50_e639_d_b2, eq50_e639_d_b3, eq50_e639_d_b4, eq50_e639_d_b5, eq50_e639_d_b6, eq50_e639_d_b7, eq50_e639_d_b8, eq50_e639_d_b9, eq50_e639_d_b10, eq50_e639_d_b11, eq50_e639_d_b12, eq50_e639_d_b13, eq50_e639_d_b14];
        stamper.stamp_current_dense_local(
            Some(17),
            None,
            multiplicity * (eq50_value),
            &eq50_node_derivatives,
            &eq50_branch_derivatives,
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
        let (eq51_e647, eq51_e647_d_n17,) = {
    if (s.b[1847] && s.b[1848]) {
        let eq51_e645: f64 = ((nv17 - 0.0) * 1e-12);
        (eq51_e645, 1e-12,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e647;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq51_value),
            17,
            multiplicity * (eq51_e647_d_n17),
        );
        let (eq52_e658, eq52_e658_d_n17,) = {
    if (s.b[1847] && s.b[1848]) {
        let eq52_e653: f64 = (1e-9 / 0.0001);let eq52_e655: f64 = (eq52_e653 * (nv17 - 0.0));let eq52_e656: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq52_e655);
        (eq52_e656, (eq52_e653 * ddt_scale),)
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
        let (eq53_e665,) = {
    if (s.b[1847] && (!s.b[1848])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq53_value: f64 = eq53_e665;
        stamper.stamp_potential_const_local(
            10,
            eq53_value,
        );
        let (eq54_e674, eq54_e674_d_n0, eq54_e674_d_n1, eq54_e674_d_n2, eq54_e674_d_n3, eq54_e674_d_n4, eq54_e674_d_n5, eq54_e674_d_n6, eq54_e674_d_n7, eq54_e674_d_n8, eq54_e674_d_n9, eq54_e674_d_n10, eq54_e674_d_n11, eq54_e674_d_n12, eq54_e674_d_n13, eq54_e674_d_n14, eq54_e674_d_n15, eq54_e674_d_n16, eq54_e674_d_n17, eq54_e674_d_n18, eq54_e674_d_b0, eq54_e674_d_b1, eq54_e674_d_b2, eq54_e674_d_b3, eq54_e674_d_b4, eq54_e674_d_b5, eq54_e674_d_b6, eq54_e674_d_b7, eq54_e674_d_b8, eq54_e674_d_b9, eq54_e674_d_b10, eq54_e674_d_b11, eq54_e674_d_b12, eq54_e674_d_b13, eq54_e674_d_b14,) = {
    if (!s.b[1847]) {
        let eq54_e671: f64 = (s.v[311] + s.v[263]);let eq54_e671_d_n0: f64 = (s.dn[311][0] + s.dn[263][0]);let eq54_e671_d_n1: f64 = (s.dn[311][1] + s.dn[263][1]);let eq54_e671_d_n2: f64 = (s.dn[311][2] + s.dn[263][2]);let eq54_e671_d_n3: f64 = (s.dn[311][3] + s.dn[263][3]);let eq54_e671_d_n4: f64 = (s.dn[311][4] + s.dn[263][4]);let eq54_e671_d_n5: f64 = (s.dn[311][5] + s.dn[263][5]);let eq54_e671_d_n6: f64 = (s.dn[311][6] + s.dn[263][6]);let eq54_e671_d_n7: f64 = (s.dn[311][7] + s.dn[263][7]);let eq54_e671_d_n8: f64 = (s.dn[311][8] + s.dn[263][8]);let eq54_e671_d_n9: f64 = (s.dn[311][9] + s.dn[263][9]);let eq54_e671_d_n10: f64 = (s.dn[311][10] + s.dn[263][10]);let eq54_e671_d_n11: f64 = (s.dn[311][11] + s.dn[263][11]);let eq54_e671_d_n12: f64 = (s.dn[311][12] + s.dn[263][12]);let eq54_e671_d_n13: f64 = (s.dn[311][13] + s.dn[263][13]);let eq54_e671_d_n14: f64 = (s.dn[311][14] + s.dn[263][14]);let eq54_e671_d_n15: f64 = (s.dn[311][15] + s.dn[263][15]);let eq54_e671_d_n16: f64 = (s.dn[311][16] + s.dn[263][16]);let eq54_e671_d_n17: f64 = (s.dn[311][17] + s.dn[263][17]);let eq54_e671_d_n18: f64 = (s.dn[311][18] + s.dn[263][18]);let eq54_e671_d_b0: f64 = (s.db[311][0] + s.db[263][0]);let eq54_e671_d_b1: f64 = (s.db[311][1] + s.db[263][1]);let eq54_e671_d_b2: f64 = (s.db[311][2] + s.db[263][2]);let eq54_e671_d_b3: f64 = (s.db[311][3] + s.db[263][3]);let eq54_e671_d_b4: f64 = (s.db[311][4] + s.db[263][4]);let eq54_e671_d_b5: f64 = (s.db[311][5] + s.db[263][5]);let eq54_e671_d_b6: f64 = (s.db[311][6] + s.db[263][6]);let eq54_e671_d_b7: f64 = (s.db[311][7] + s.db[263][7]);let eq54_e671_d_b8: f64 = (s.db[311][8] + s.db[263][8]);let eq54_e671_d_b9: f64 = (s.db[311][9] + s.db[263][9]);let eq54_e671_d_b10: f64 = (s.db[311][10] + s.db[263][10]);let eq54_e671_d_b11: f64 = (s.db[311][11] + s.db[263][11]);let eq54_e671_d_b12: f64 = (s.db[311][12] + s.db[263][12]);let eq54_e671_d_b13: f64 = (s.db[311][13] + s.db[263][13]);let eq54_e671_d_b14: f64 = (s.db[311][14] + s.db[263][14]);let eq54_e672: f64 = (p.p50 * eq54_e671);let eq54_e672_d_n0: f64 = (p.p50 * eq54_e671_d_n0);let eq54_e672_d_n1: f64 = (p.p50 * eq54_e671_d_n1);let eq54_e672_d_n2: f64 = (p.p50 * eq54_e671_d_n2);let eq54_e672_d_n3: f64 = (p.p50 * eq54_e671_d_n3);let eq54_e672_d_n4: f64 = (p.p50 * eq54_e671_d_n4);let eq54_e672_d_n5: f64 = (p.p50 * eq54_e671_d_n5);let eq54_e672_d_n6: f64 = (p.p50 * eq54_e671_d_n6);let eq54_e672_d_n7: f64 = (p.p50 * eq54_e671_d_n7);let eq54_e672_d_n8: f64 = (p.p50 * eq54_e671_d_n8);let eq54_e672_d_n9: f64 = (p.p50 * eq54_e671_d_n9);let eq54_e672_d_n10: f64 = (p.p50 * eq54_e671_d_n10);let eq54_e672_d_n11: f64 = (p.p50 * eq54_e671_d_n11);let eq54_e672_d_n12: f64 = (p.p50 * eq54_e671_d_n12);let eq54_e672_d_n13: f64 = (p.p50 * eq54_e671_d_n13);let eq54_e672_d_n14: f64 = (p.p50 * eq54_e671_d_n14);let eq54_e672_d_n15: f64 = (p.p50 * eq54_e671_d_n15);let eq54_e672_d_n16: f64 = (p.p50 * eq54_e671_d_n16);let eq54_e672_d_n17: f64 = (p.p50 * eq54_e671_d_n17);let eq54_e672_d_n18: f64 = (p.p50 * eq54_e671_d_n18);let eq54_e672_d_b0: f64 = (p.p50 * eq54_e671_d_b0);let eq54_e672_d_b1: f64 = (p.p50 * eq54_e671_d_b1);let eq54_e672_d_b2: f64 = (p.p50 * eq54_e671_d_b2);let eq54_e672_d_b3: f64 = (p.p50 * eq54_e671_d_b3);let eq54_e672_d_b4: f64 = (p.p50 * eq54_e671_d_b4);let eq54_e672_d_b5: f64 = (p.p50 * eq54_e671_d_b5);let eq54_e672_d_b6: f64 = (p.p50 * eq54_e671_d_b6);let eq54_e672_d_b7: f64 = (p.p50 * eq54_e671_d_b7);let eq54_e672_d_b8: f64 = (p.p50 * eq54_e671_d_b8);let eq54_e672_d_b9: f64 = (p.p50 * eq54_e671_d_b9);let eq54_e672_d_b10: f64 = (p.p50 * eq54_e671_d_b10);let eq54_e672_d_b11: f64 = (p.p50 * eq54_e671_d_b11);let eq54_e672_d_b12: f64 = (p.p50 * eq54_e671_d_b12);let eq54_e672_d_b13: f64 = (p.p50 * eq54_e671_d_b13);let eq54_e672_d_b14: f64 = (p.p50 * eq54_e671_d_b14);
        (eq54_e672, eq54_e672_d_n0, eq54_e672_d_n1, eq54_e672_d_n2, eq54_e672_d_n3, eq54_e672_d_n4, eq54_e672_d_n5, eq54_e672_d_n6, eq54_e672_d_n7, eq54_e672_d_n8, eq54_e672_d_n9, eq54_e672_d_n10, eq54_e672_d_n11, eq54_e672_d_n12, eq54_e672_d_n13, eq54_e672_d_n14, eq54_e672_d_n15, eq54_e672_d_n16, eq54_e672_d_n17, eq54_e672_d_n18, eq54_e672_d_b0, eq54_e672_d_b1, eq54_e672_d_b2, eq54_e672_d_b3, eq54_e672_d_b4, eq54_e672_d_b5, eq54_e672_d_b6, eq54_e672_d_b7, eq54_e672_d_b8, eq54_e672_d_b9, eq54_e672_d_b10, eq54_e672_d_b11, eq54_e672_d_b12, eq54_e672_d_b13, eq54_e672_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e674;let eq54_node_derivatives: [f64; 19] = [eq54_e674_d_n0, eq54_e674_d_n1, eq54_e674_d_n2, eq54_e674_d_n3, eq54_e674_d_n4, eq54_e674_d_n5, eq54_e674_d_n6, eq54_e674_d_n7, eq54_e674_d_n8, eq54_e674_d_n9, eq54_e674_d_n10, eq54_e674_d_n11, eq54_e674_d_n12, eq54_e674_d_n13, eq54_e674_d_n14, eq54_e674_d_n15, eq54_e674_d_n16, eq54_e674_d_n17, eq54_e674_d_n18];let eq54_branch_derivatives: [f64; 15] = [eq54_e674_d_b0, eq54_e674_d_b1, eq54_e674_d_b2, eq54_e674_d_b3, eq54_e674_d_b4, eq54_e674_d_b5, eq54_e674_d_b6, eq54_e674_d_b7, eq54_e674_d_b8, eq54_e674_d_b9, eq54_e674_d_b10, eq54_e674_d_b11, eq54_e674_d_b12, eq54_e674_d_b13, eq54_e674_d_b14];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq54_value),
            &eq54_node_derivatives,
            &eq54_branch_derivatives,
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
        let (eq55_e683, eq55_e683_d_n0, eq55_e683_d_n1, eq55_e683_d_n2, eq55_e683_d_n3, eq55_e683_d_n4, eq55_e683_d_n5, eq55_e683_d_n6, eq55_e683_d_n7, eq55_e683_d_n8, eq55_e683_d_n9, eq55_e683_d_n10, eq55_e683_d_n11, eq55_e683_d_n12, eq55_e683_d_n13, eq55_e683_d_n14, eq55_e683_d_n15, eq55_e683_d_n16, eq55_e683_d_n17, eq55_e683_d_n18, eq55_e683_d_b0, eq55_e683_d_b1, eq55_e683_d_b2, eq55_e683_d_b3, eq55_e683_d_b4, eq55_e683_d_b5, eq55_e683_d_b6, eq55_e683_d_b7, eq55_e683_d_b8, eq55_e683_d_b9, eq55_e683_d_b10, eq55_e683_d_b11, eq55_e683_d_b12, eq55_e683_d_b13, eq55_e683_d_b14,) = {
    if (!s.b[1847]) {
        let eq55_e680: f64 = (s.v[312] + s.v[573]);let eq55_e680_d_n0: f64 = (s.dn[312][0] + s.dn[573][0]);let eq55_e680_d_n1: f64 = (s.dn[312][1] + s.dn[573][1]);let eq55_e680_d_n2: f64 = (s.dn[312][2] + s.dn[573][2]);let eq55_e680_d_n3: f64 = (s.dn[312][3] + s.dn[573][3]);let eq55_e680_d_n4: f64 = (s.dn[312][4] + s.dn[573][4]);let eq55_e680_d_n5: f64 = (s.dn[312][5] + s.dn[573][5]);let eq55_e680_d_n6: f64 = (s.dn[312][6] + s.dn[573][6]);let eq55_e680_d_n7: f64 = (s.dn[312][7] + s.dn[573][7]);let eq55_e680_d_n8: f64 = (s.dn[312][8] + s.dn[573][8]);let eq55_e680_d_n9: f64 = (s.dn[312][9] + s.dn[573][9]);let eq55_e680_d_n10: f64 = (s.dn[312][10] + s.dn[573][10]);let eq55_e680_d_n11: f64 = (s.dn[312][11] + s.dn[573][11]);let eq55_e680_d_n12: f64 = (s.dn[312][12] + s.dn[573][12]);let eq55_e680_d_n13: f64 = (s.dn[312][13] + s.dn[573][13]);let eq55_e680_d_n14: f64 = (s.dn[312][14] + s.dn[573][14]);let eq55_e680_d_n15: f64 = (s.dn[312][15] + s.dn[573][15]);let eq55_e680_d_n16: f64 = (s.dn[312][16] + s.dn[573][16]);let eq55_e680_d_n17: f64 = (s.dn[312][17] + s.dn[573][17]);let eq55_e680_d_n18: f64 = (s.dn[312][18] + s.dn[573][18]);let eq55_e680_d_b0: f64 = (s.db[312][0] + s.db[573][0]);let eq55_e680_d_b1: f64 = (s.db[312][1] + s.db[573][1]);let eq55_e680_d_b2: f64 = (s.db[312][2] + s.db[573][2]);let eq55_e680_d_b3: f64 = (s.db[312][3] + s.db[573][3]);let eq55_e680_d_b4: f64 = (s.db[312][4] + s.db[573][4]);let eq55_e680_d_b5: f64 = (s.db[312][5] + s.db[573][5]);let eq55_e680_d_b6: f64 = (s.db[312][6] + s.db[573][6]);let eq55_e680_d_b7: f64 = (s.db[312][7] + s.db[573][7]);let eq55_e680_d_b8: f64 = (s.db[312][8] + s.db[573][8]);let eq55_e680_d_b9: f64 = (s.db[312][9] + s.db[573][9]);let eq55_e680_d_b10: f64 = (s.db[312][10] + s.db[573][10]);let eq55_e680_d_b11: f64 = (s.db[312][11] + s.db[573][11]);let eq55_e680_d_b12: f64 = (s.db[312][12] + s.db[573][12]);let eq55_e680_d_b13: f64 = (s.db[312][13] + s.db[573][13]);let eq55_e680_d_b14: f64 = (s.db[312][14] + s.db[573][14]);let eq55_e681: f64 = (p.p50 * eq55_e680);let eq55_e681_d_n0: f64 = (p.p50 * eq55_e680_d_n0);let eq55_e681_d_n1: f64 = (p.p50 * eq55_e680_d_n1);let eq55_e681_d_n2: f64 = (p.p50 * eq55_e680_d_n2);let eq55_e681_d_n3: f64 = (p.p50 * eq55_e680_d_n3);let eq55_e681_d_n4: f64 = (p.p50 * eq55_e680_d_n4);let eq55_e681_d_n5: f64 = (p.p50 * eq55_e680_d_n5);let eq55_e681_d_n6: f64 = (p.p50 * eq55_e680_d_n6);let eq55_e681_d_n7: f64 = (p.p50 * eq55_e680_d_n7);let eq55_e681_d_n8: f64 = (p.p50 * eq55_e680_d_n8);let eq55_e681_d_n9: f64 = (p.p50 * eq55_e680_d_n9);let eq55_e681_d_n10: f64 = (p.p50 * eq55_e680_d_n10);let eq55_e681_d_n11: f64 = (p.p50 * eq55_e680_d_n11);let eq55_e681_d_n12: f64 = (p.p50 * eq55_e680_d_n12);let eq55_e681_d_n13: f64 = (p.p50 * eq55_e680_d_n13);let eq55_e681_d_n14: f64 = (p.p50 * eq55_e680_d_n14);let eq55_e681_d_n15: f64 = (p.p50 * eq55_e680_d_n15);let eq55_e681_d_n16: f64 = (p.p50 * eq55_e680_d_n16);let eq55_e681_d_n17: f64 = (p.p50 * eq55_e680_d_n17);let eq55_e681_d_n18: f64 = (p.p50 * eq55_e680_d_n18);let eq55_e681_d_b0: f64 = (p.p50 * eq55_e680_d_b0);let eq55_e681_d_b1: f64 = (p.p50 * eq55_e680_d_b1);let eq55_e681_d_b2: f64 = (p.p50 * eq55_e680_d_b2);let eq55_e681_d_b3: f64 = (p.p50 * eq55_e680_d_b3);let eq55_e681_d_b4: f64 = (p.p50 * eq55_e680_d_b4);let eq55_e681_d_b5: f64 = (p.p50 * eq55_e680_d_b5);let eq55_e681_d_b6: f64 = (p.p50 * eq55_e680_d_b6);let eq55_e681_d_b7: f64 = (p.p50 * eq55_e680_d_b7);let eq55_e681_d_b8: f64 = (p.p50 * eq55_e680_d_b8);let eq55_e681_d_b9: f64 = (p.p50 * eq55_e680_d_b9);let eq55_e681_d_b10: f64 = (p.p50 * eq55_e680_d_b10);let eq55_e681_d_b11: f64 = (p.p50 * eq55_e680_d_b11);let eq55_e681_d_b12: f64 = (p.p50 * eq55_e680_d_b12);let eq55_e681_d_b13: f64 = (p.p50 * eq55_e680_d_b13);let eq55_e681_d_b14: f64 = (p.p50 * eq55_e680_d_b14);
        (eq55_e681, eq55_e681_d_n0, eq55_e681_d_n1, eq55_e681_d_n2, eq55_e681_d_n3, eq55_e681_d_n4, eq55_e681_d_n5, eq55_e681_d_n6, eq55_e681_d_n7, eq55_e681_d_n8, eq55_e681_d_n9, eq55_e681_d_n10, eq55_e681_d_n11, eq55_e681_d_n12, eq55_e681_d_n13, eq55_e681_d_n14, eq55_e681_d_n15, eq55_e681_d_n16, eq55_e681_d_n17, eq55_e681_d_n18, eq55_e681_d_b0, eq55_e681_d_b1, eq55_e681_d_b2, eq55_e681_d_b3, eq55_e681_d_b4, eq55_e681_d_b5, eq55_e681_d_b6, eq55_e681_d_b7, eq55_e681_d_b8, eq55_e681_d_b9, eq55_e681_d_b10, eq55_e681_d_b11, eq55_e681_d_b12, eq55_e681_d_b13, eq55_e681_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e683;let eq55_node_derivatives: [f64; 19] = [eq55_e683_d_n0, eq55_e683_d_n1, eq55_e683_d_n2, eq55_e683_d_n3, eq55_e683_d_n4, eq55_e683_d_n5, eq55_e683_d_n6, eq55_e683_d_n7, eq55_e683_d_n8, eq55_e683_d_n9, eq55_e683_d_n10, eq55_e683_d_n11, eq55_e683_d_n12, eq55_e683_d_n13, eq55_e683_d_n14, eq55_e683_d_n15, eq55_e683_d_n16, eq55_e683_d_n17, eq55_e683_d_n18];let eq55_branch_derivatives: [f64; 15] = [eq55_e683_d_b0, eq55_e683_d_b1, eq55_e683_d_b2, eq55_e683_d_b3, eq55_e683_d_b4, eq55_e683_d_b5, eq55_e683_d_b6, eq55_e683_d_b7, eq55_e683_d_b8, eq55_e683_d_b9, eq55_e683_d_b10, eq55_e683_d_b11, eq55_e683_d_b12, eq55_e683_d_b13, eq55_e683_d_b14];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq55_value),
            &eq55_node_derivatives,
            &eq55_branch_derivatives,
            multiplicity,
        );
        let (eq56_e688,) = {
    if (!s.b[1847]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq56_value: f64 = eq56_e688;
        stamper.stamp_potential_const_local(
            11,
            eq56_value,
        );
        let (eq57_e695, eq57_e695_d_n0, eq57_e695_d_n1, eq57_e695_d_n2, eq57_e695_d_n3, eq57_e695_d_n4, eq57_e695_d_n5, eq57_e695_d_n6, eq57_e695_d_n7, eq57_e695_d_n8, eq57_e695_d_n9, eq57_e695_d_n10, eq57_e695_d_n11, eq57_e695_d_n12, eq57_e695_d_n13, eq57_e695_d_n14, eq57_e695_d_n15, eq57_e695_d_n16, eq57_e695_d_n17, eq57_e695_d_n18, eq57_e695_d_b0, eq57_e695_d_b1, eq57_e695_d_b2, eq57_e695_d_b3, eq57_e695_d_b4, eq57_e695_d_b5, eq57_e695_d_b6, eq57_e695_d_b7, eq57_e695_d_b8, eq57_e695_d_b9, eq57_e695_d_b10, eq57_e695_d_b11, eq57_e695_d_b12, eq57_e695_d_b13, eq57_e695_d_b14,) = {
    if ((!s.b[1847]) && (p.p37 != 0.0)) {
        (s.v[592], s.dn[592][0], s.dn[592][1], s.dn[592][2], s.dn[592][3], s.dn[592][4], s.dn[592][5], s.dn[592][6], s.dn[592][7], s.dn[592][8], s.dn[592][9], s.dn[592][10], s.dn[592][11], s.dn[592][12], s.dn[592][13], s.dn[592][14], s.dn[592][15], s.dn[592][16], s.dn[592][17], s.dn[592][18], s.db[592][0], s.db[592][1], s.db[592][2], s.db[592][3], s.db[592][4], s.db[592][5], s.db[592][6], s.db[592][7], s.db[592][8], s.db[592][9], s.db[592][10], s.db[592][11], s.db[592][12], s.db[592][13], s.db[592][14],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e695;let eq57_node_derivatives: [f64; 19] = [eq57_e695_d_n0, eq57_e695_d_n1, eq57_e695_d_n2, eq57_e695_d_n3, eq57_e695_d_n4, eq57_e695_d_n5, eq57_e695_d_n6, eq57_e695_d_n7, eq57_e695_d_n8, eq57_e695_d_n9, eq57_e695_d_n10, eq57_e695_d_n11, eq57_e695_d_n12, eq57_e695_d_n13, eq57_e695_d_n14, eq57_e695_d_n15, eq57_e695_d_n16, eq57_e695_d_n17, eq57_e695_d_n18];let eq57_branch_derivatives: [f64; 15] = [eq57_e695_d_b0, eq57_e695_d_b1, eq57_e695_d_b2, eq57_e695_d_b3, eq57_e695_d_b4, eq57_e695_d_b5, eq57_e695_d_b6, eq57_e695_d_b7, eq57_e695_d_b8, eq57_e695_d_b9, eq57_e695_d_b10, eq57_e695_d_b11, eq57_e695_d_b12, eq57_e695_d_b13, eq57_e695_d_b14];
        stamper.stamp_current_dense_local(
            Some(17),
            None,
            multiplicity * (eq57_value),
            &eq57_node_derivatives,
            &eq57_branch_derivatives,
            multiplicity,
        );
        let (eq58_e704, eq58_e704_d_n17,) = {
    if ((!s.b[1847]) && (p.p37 != 0.0)) {
        let eq58_e702: f64 = ((nv17 - 0.0) * 1e-12);
        (eq58_e702, 1e-12,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e704;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq58_value),
            17,
            multiplicity * (eq58_e704_d_n17),
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
        let (eq59_e716, eq59_e716_d_n17,) = {
    if ((!s.b[1847]) && (p.p37 != 0.0)) {
        let eq59_e711: f64 = (1e-9 / 0.0001);let eq59_e713: f64 = (eq59_e711 * (nv17 - 0.0));let eq59_e714: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq59_e713);
        (eq59_e714, (eq59_e711 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e716;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq59_value),
            17,
            multiplicity * (eq59_e716_d_n17),
        );
        let (eq60_e724,) = {
    if ((!s.b[1847]) && (p.p37 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq60_value: f64 = eq60_e724;
        stamper.stamp_potential_const_local(
            12,
            eq60_value,
        );
        let (eq61_e731, eq61_e731_d_n0, eq61_e731_d_n1, eq61_e731_d_n2, eq61_e731_d_n3, eq61_e731_d_n4, eq61_e731_d_n5, eq61_e731_d_n6, eq61_e731_d_n7, eq61_e731_d_n8, eq61_e731_d_n9, eq61_e731_d_n10, eq61_e731_d_n11, eq61_e731_d_n12, eq61_e731_d_n13, eq61_e731_d_n14, eq61_e731_d_n15, eq61_e731_d_n16, eq61_e731_d_n17, eq61_e731_d_n18, eq61_e731_d_b0, eq61_e731_d_b1, eq61_e731_d_b2, eq61_e731_d_b3, eq61_e731_d_b4, eq61_e731_d_b5, eq61_e731_d_b6, eq61_e731_d_b7, eq61_e731_d_b8, eq61_e731_d_b9, eq61_e731_d_b10, eq61_e731_d_b11, eq61_e731_d_b12, eq61_e731_d_b13, eq61_e731_d_b14,) = {
    if ((!s.b[1847]) && (p.p34 != 0.0)) {
        (s.v[574], s.dn[574][0], s.dn[574][1], s.dn[574][2], s.dn[574][3], s.dn[574][4], s.dn[574][5], s.dn[574][6], s.dn[574][7], s.dn[574][8], s.dn[574][9], s.dn[574][10], s.dn[574][11], s.dn[574][12], s.dn[574][13], s.dn[574][14], s.dn[574][15], s.dn[574][16], s.dn[574][17], s.dn[574][18], s.db[574][0], s.db[574][1], s.db[574][2], s.db[574][3], s.db[574][4], s.db[574][5], s.db[574][6], s.db[574][7], s.db[574][8], s.db[574][9], s.db[574][10], s.db[574][11], s.db[574][12], s.db[574][13], s.db[574][14],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_value: f64 = eq61_e731;let eq61_node_derivatives: [f64; 19] = [eq61_e731_d_n0, eq61_e731_d_n1, eq61_e731_d_n2, eq61_e731_d_n3, eq61_e731_d_n4, eq61_e731_d_n5, eq61_e731_d_n6, eq61_e731_d_n7, eq61_e731_d_n8, eq61_e731_d_n9, eq61_e731_d_n10, eq61_e731_d_n11, eq61_e731_d_n12, eq61_e731_d_n13, eq61_e731_d_n14, eq61_e731_d_n15, eq61_e731_d_n16, eq61_e731_d_n17, eq61_e731_d_n18];let eq61_branch_derivatives: [f64; 15] = [eq61_e731_d_b0, eq61_e731_d_b1, eq61_e731_d_b2, eq61_e731_d_b3, eq61_e731_d_b4, eq61_e731_d_b5, eq61_e731_d_b6, eq61_e731_d_b7, eq61_e731_d_b8, eq61_e731_d_b9, eq61_e731_d_b10, eq61_e731_d_b11, eq61_e731_d_b12, eq61_e731_d_b13, eq61_e731_d_b14];
        stamper.stamp_current_dense_local(
            Some(15),
            None,
            multiplicity * (eq61_value),
            &eq61_node_derivatives,
            &eq61_branch_derivatives,
            multiplicity,
        );
        let (eq62_e738, eq62_e738_d_n0, eq62_e738_d_n1, eq62_e738_d_n2, eq62_e738_d_n3, eq62_e738_d_n4, eq62_e738_d_n5, eq62_e738_d_n6, eq62_e738_d_n7, eq62_e738_d_n8, eq62_e738_d_n9, eq62_e738_d_n10, eq62_e738_d_n11, eq62_e738_d_n12, eq62_e738_d_n13, eq62_e738_d_n14, eq62_e738_d_n15, eq62_e738_d_n16, eq62_e738_d_n17, eq62_e738_d_n18, eq62_e738_d_b0, eq62_e738_d_b1, eq62_e738_d_b2, eq62_e738_d_b3, eq62_e738_d_b4, eq62_e738_d_b5, eq62_e738_d_b6, eq62_e738_d_b7, eq62_e738_d_b8, eq62_e738_d_b9, eq62_e738_d_b10, eq62_e738_d_b11, eq62_e738_d_b12, eq62_e738_d_b13, eq62_e738_d_b14,) = {
    if ((!s.b[1847]) && (p.p34 != 0.0)) {
        (s.v[575], s.dn[575][0], s.dn[575][1], s.dn[575][2], s.dn[575][3], s.dn[575][4], s.dn[575][5], s.dn[575][6], s.dn[575][7], s.dn[575][8], s.dn[575][9], s.dn[575][10], s.dn[575][11], s.dn[575][12], s.dn[575][13], s.dn[575][14], s.dn[575][15], s.dn[575][16], s.dn[575][17], s.dn[575][18], s.db[575][0], s.db[575][1], s.db[575][2], s.db[575][3], s.db[575][4], s.db[575][5], s.db[575][6], s.db[575][7], s.db[575][8], s.db[575][9], s.db[575][10], s.db[575][11], s.db[575][12], s.db[575][13], s.db[575][14],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e738;let eq62_node_derivatives: [f64; 19] = [eq62_e738_d_n0, eq62_e738_d_n1, eq62_e738_d_n2, eq62_e738_d_n3, eq62_e738_d_n4, eq62_e738_d_n5, eq62_e738_d_n6, eq62_e738_d_n7, eq62_e738_d_n8, eq62_e738_d_n9, eq62_e738_d_n10, eq62_e738_d_n11, eq62_e738_d_n12, eq62_e738_d_n13, eq62_e738_d_n14, eq62_e738_d_n15, eq62_e738_d_n16, eq62_e738_d_n17, eq62_e738_d_n18];let eq62_branch_derivatives: [f64; 15] = [eq62_e738_d_b0, eq62_e738_d_b1, eq62_e738_d_b2, eq62_e738_d_b3, eq62_e738_d_b4, eq62_e738_d_b5, eq62_e738_d_b6, eq62_e738_d_b7, eq62_e738_d_b8, eq62_e738_d_b9, eq62_e738_d_b10, eq62_e738_d_b11, eq62_e738_d_b12, eq62_e738_d_b13, eq62_e738_d_b14];
        stamper.stamp_current_dense_local(
            Some(16),
            None,
            multiplicity * (eq62_value),
            &eq62_node_derivatives,
            &eq62_branch_derivatives,
            multiplicity,
        );
        let (eq63_e745, eq63_e745_d_n0, eq63_e745_d_n1, eq63_e745_d_n2, eq63_e745_d_n3, eq63_e745_d_n4, eq63_e745_d_n5, eq63_e745_d_n6, eq63_e745_d_n7, eq63_e745_d_n8, eq63_e745_d_n9, eq63_e745_d_n10, eq63_e745_d_n11, eq63_e745_d_n12, eq63_e745_d_n13, eq63_e745_d_n14, eq63_e745_d_n15, eq63_e745_d_n16, eq63_e745_d_n17, eq63_e745_d_n18, eq63_e745_d_b0, eq63_e745_d_b1, eq63_e745_d_b2, eq63_e745_d_b3, eq63_e745_d_b4, eq63_e745_d_b5, eq63_e745_d_b6, eq63_e745_d_b7, eq63_e745_d_b8, eq63_e745_d_b9, eq63_e745_d_b10, eq63_e745_d_b11, eq63_e745_d_b12, eq63_e745_d_b13, eq63_e745_d_b14,) = {
    if ((!s.b[1847]) && (p.p34 != 0.0)) {
        (s.v[583], s.dn[583][0], s.dn[583][1], s.dn[583][2], s.dn[583][3], s.dn[583][4], s.dn[583][5], s.dn[583][6], s.dn[583][7], s.dn[583][8], s.dn[583][9], s.dn[583][10], s.dn[583][11], s.dn[583][12], s.dn[583][13], s.dn[583][14], s.dn[583][15], s.dn[583][16], s.dn[583][17], s.dn[583][18], s.db[583][0], s.db[583][1], s.db[583][2], s.db[583][3], s.db[583][4], s.db[583][5], s.db[583][6], s.db[583][7], s.db[583][8], s.db[583][9], s.db[583][10], s.db[583][11], s.db[583][12], s.db[583][13], s.db[583][14],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e745;let eq63_node_derivatives: [f64; 19] = [eq63_e745_d_n0, eq63_e745_d_n1, eq63_e745_d_n2, eq63_e745_d_n3, eq63_e745_d_n4, eq63_e745_d_n5, eq63_e745_d_n6, eq63_e745_d_n7, eq63_e745_d_n8, eq63_e745_d_n9, eq63_e745_d_n10, eq63_e745_d_n11, eq63_e745_d_n12, eq63_e745_d_n13, eq63_e745_d_n14, eq63_e745_d_n15, eq63_e745_d_n16, eq63_e745_d_n17, eq63_e745_d_n18];let eq63_branch_derivatives: [f64; 15] = [eq63_e745_d_b0, eq63_e745_d_b1, eq63_e745_d_b2, eq63_e745_d_b3, eq63_e745_d_b4, eq63_e745_d_b5, eq63_e745_d_b6, eq63_e745_d_b7, eq63_e745_d_b8, eq63_e745_d_b9, eq63_e745_d_b10, eq63_e745_d_b11, eq63_e745_d_b12, eq63_e745_d_b13, eq63_e745_d_b14];
        stamper.stamp_current_dense_local(
            Some(13),
            None,
            multiplicity * (eq63_value),
            &eq63_node_derivatives,
            &eq63_branch_derivatives,
            multiplicity,
        );
        let (eq64_e754, eq64_e754_d_n15,) = {
    if ((!s.b[1847]) && (p.p34 != 0.0)) {
        let eq64_e752: f64 = ((nv15 - 0.0) * 1e-12);
        (eq64_e752, 1e-12,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq64_value: f64 = eq64_e754;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq64_value),
            15,
            multiplicity * (eq64_e754_d_n15),
        );
        let (eq65_e763, eq65_e763_d_n16,) = {
    if ((!s.b[1847]) && (p.p34 != 0.0)) {
        let eq65_e761: f64 = ((nv16 - 0.0) * 1e-12);
        (eq65_e761, 1e-12,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e763;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (eq65_value),
            16,
            multiplicity * (eq65_e763_d_n16),
        );
        let (eq66_e772, eq66_e772_d_n13,) = {
    if ((!s.b[1847]) && (p.p34 != 0.0)) {
        let eq66_e770: f64 = ((nv13 - 0.0) * 1e-12);
        (eq66_e770, 1e-12,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq66_value: f64 = eq66_e772;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq66_value),
            13,
            multiplicity * (eq66_e772_d_n13),
        );
        let (eq67_e784, eq67_e784_d_n15,) = {
    if ((!s.b[1847]) && (p.p34 != 0.0)) {
        let eq67_e779: f64 = (1e-9 / 0.0001);let eq67_e781: f64 = (eq67_e779 * (nv15 - 0.0));let eq67_e782: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq67_e781);
        (eq67_e782, (eq67_e779 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e784;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq67_value),
            15,
            multiplicity * (eq67_e784_d_n15),
        );
        let (eq68_e796, eq68_e796_d_n16,) = {
    if ((!s.b[1847]) && (p.p34 != 0.0)) {
        let eq68_e791: f64 = (1e-9 / 0.0001);let eq68_e793: f64 = (eq68_e791 * (nv16 - 0.0));let eq68_e794: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq68_e793);
        (eq68_e794, (eq68_e791 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq68_value: f64 = eq68_e796;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (eq68_value),
            16,
            multiplicity * (eq68_e796_d_n16),
        );
        let (eq69_e808, eq69_e808_d_n13,) = {
    if ((!s.b[1847]) && (p.p34 != 0.0)) {
        let eq69_e803: f64 = (1e-9 / 0.0001);let eq69_e805: f64 = (eq69_e803 * (nv13 - 0.0));let eq69_e806: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq69_e805);
        (eq69_e806, (eq69_e803 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e808;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq69_value),
            13,
            multiplicity * (eq69_e808_d_n13),
        );
        let (eq70_e816,) = {
    if ((!s.b[1847]) && (p.p34 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq70_value: f64 = eq70_e816;
        stamper.stamp_potential_const_local(
            13,
            eq70_value,
        );
        let (eq71_e824,) = {
    if ((!s.b[1847]) && (p.p34 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq71_value: f64 = eq71_e824;
        stamper.stamp_potential_const_local(
            14,
            eq71_value,
        );
        let (eq72_e832,) = {
    if ((!s.b[1847]) && (p.p34 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq72_value: f64 = eq72_e832;
        stamper.stamp_potential_const_local(
            15,
            eq72_value,
        );
        let (eq73_e836,) = {
    if s.b[1849] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq73_value: f64 = eq73_e836;
        stamper.stamp_potential_const_local(
            16,
            eq73_value,
        );
        let (eq74_e841,) = {
    if (!s.b[1849]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq74_value: f64 = eq74_e841;
        stamper.stamp_potential_const_local(
            17,
            eq74_value,
        );
        let (eq75_e846,) = {
    if (!s.b[1849]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq75_value: f64 = eq75_e846;
        stamper.stamp_potential_const_local(
            18,
            eq75_value,
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
        let nv14 = ctx.node_voltage(nodes[14]);let eq10_e359_q: f64 = s.v[594];let eq10_e360: f64 = (p.p50 * s.v[594]);let eq10_e360_q: f64 = (p.p50 * eq10_e359_q);
        stamper.stamp_current_reactive_dense_local(
            Some(11),
            Some(7),
            &s.dn[594],
            &s.db[594],
            (multiplicity) * (p.p50),
        );let eq11_e363_q: f64 = s.v[198];let eq11_e364: f64 = (p.p50 * s.v[198]);let eq11_e364_q: f64 = (p.p50 * eq11_e363_q);
        stamper.stamp_current_reactive_dense_local(
            Some(6),
            Some(7),
            &s.dn[198],
            &s.db[198],
            (multiplicity) * (p.p50),
        );let eq12_e367_q: f64 = s.v[196];let eq12_e368: f64 = (p.p50 * s.v[196]);let eq12_e368_q: f64 = (p.p50 * eq12_e367_q);
        stamper.stamp_current_reactive_dense_local(
            Some(12),
            Some(7),
            &s.dn[196],
            &s.db[196],
            (multiplicity) * (p.p50),
        );let eq18_e397: f64 = ((nv14 - 0.0) * s.v[617]);let eq18_e397_d_n0: f64 = ((nv14 - 0.0) * s.dn[617][0]);let eq18_e397_d_n1: f64 = ((nv14 - 0.0) * s.dn[617][1]);let eq18_e397_d_n2: f64 = ((nv14 - 0.0) * s.dn[617][2]);let eq18_e397_d_n3: f64 = ((nv14 - 0.0) * s.dn[617][3]);let eq18_e397_d_n4: f64 = ((nv14 - 0.0) * s.dn[617][4]);let eq18_e397_d_n5: f64 = ((nv14 - 0.0) * s.dn[617][5]);let eq18_e397_d_n6: f64 = ((nv14 - 0.0) * s.dn[617][6]);let eq18_e397_d_n7: f64 = ((nv14 - 0.0) * s.dn[617][7]);let eq18_e397_d_n8: f64 = ((nv14 - 0.0) * s.dn[617][8]);let eq18_e397_d_n9: f64 = ((nv14 - 0.0) * s.dn[617][9]);let eq18_e397_d_n10: f64 = ((nv14 - 0.0) * s.dn[617][10]);let eq18_e397_d_n11: f64 = ((nv14 - 0.0) * s.dn[617][11]);let eq18_e397_d_n12: f64 = ((nv14 - 0.0) * s.dn[617][12]);let eq18_e397_d_n13: f64 = ((nv14 - 0.0) * s.dn[617][13]);let eq18_e397_d_n14: f64 = (s.v[617] + ((nv14 - 0.0) * s.dn[617][14]));let eq18_e397_d_n15: f64 = ((nv14 - 0.0) * s.dn[617][15]);let eq18_e397_d_n16: f64 = ((nv14 - 0.0) * s.dn[617][16]);let eq18_e397_d_n17: f64 = ((nv14 - 0.0) * s.dn[617][17]);let eq18_e397_d_n18: f64 = ((nv14 - 0.0) * s.dn[617][18]);let eq18_e397_d_b0: f64 = ((nv14 - 0.0) * s.db[617][0]);let eq18_e397_d_b1: f64 = ((nv14 - 0.0) * s.db[617][1]);let eq18_e397_d_b2: f64 = ((nv14 - 0.0) * s.db[617][2]);let eq18_e397_d_b3: f64 = ((nv14 - 0.0) * s.db[617][3]);let eq18_e397_d_b4: f64 = ((nv14 - 0.0) * s.db[617][4]);let eq18_e397_d_b5: f64 = ((nv14 - 0.0) * s.db[617][5]);let eq18_e397_d_b6: f64 = ((nv14 - 0.0) * s.db[617][6]);let eq18_e397_d_b7: f64 = ((nv14 - 0.0) * s.db[617][7]);let eq18_e397_d_b8: f64 = ((nv14 - 0.0) * s.db[617][8]);let eq18_e397_d_b9: f64 = ((nv14 - 0.0) * s.db[617][9]);let eq18_e397_d_b10: f64 = ((nv14 - 0.0) * s.db[617][10]);let eq18_e397_d_b11: f64 = ((nv14 - 0.0) * s.db[617][11]);let eq18_e397_d_b12: f64 = ((nv14 - 0.0) * s.db[617][12]);let eq18_e397_d_b13: f64 = ((nv14 - 0.0) * s.db[617][13]);let eq18_e397_d_b14: f64 = ((nv14 - 0.0) * s.db[617][14]);let eq18_e398_q: f64 = eq18_e397;let eq18_reactive_node_derivatives: [f64; 19] = [eq18_e397_d_n0, eq18_e397_d_n1, eq18_e397_d_n2, eq18_e397_d_n3, eq18_e397_d_n4, eq18_e397_d_n5, eq18_e397_d_n6, eq18_e397_d_n7, eq18_e397_d_n8, eq18_e397_d_n9, eq18_e397_d_n10, eq18_e397_d_n11, eq18_e397_d_n12, eq18_e397_d_n13, eq18_e397_d_n14, eq18_e397_d_n15, eq18_e397_d_n16, eq18_e397_d_n17, eq18_e397_d_n18];let eq18_reactive_branch_derivatives: [f64; 15] = [eq18_e397_d_b0, eq18_e397_d_b1, eq18_e397_d_b2, eq18_e397_d_b3, eq18_e397_d_b4, eq18_e397_d_b5, eq18_e397_d_b6, eq18_e397_d_b7, eq18_e397_d_b8, eq18_e397_d_b9, eq18_e397_d_b10, eq18_e397_d_b11, eq18_e397_d_b12, eq18_e397_d_b13, eq18_e397_d_b14];
        stamper.stamp_current_reactive_dense_local(
            Some(11),
            Some(7),
            &eq18_reactive_node_derivatives,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );let eq19_e401: f64 = ((nv14 - 0.0) * s.v[618]);let eq19_e401_d_n0: f64 = ((nv14 - 0.0) * s.dn[618][0]);let eq19_e401_d_n1: f64 = ((nv14 - 0.0) * s.dn[618][1]);let eq19_e401_d_n2: f64 = ((nv14 - 0.0) * s.dn[618][2]);let eq19_e401_d_n3: f64 = ((nv14 - 0.0) * s.dn[618][3]);let eq19_e401_d_n4: f64 = ((nv14 - 0.0) * s.dn[618][4]);let eq19_e401_d_n5: f64 = ((nv14 - 0.0) * s.dn[618][5]);let eq19_e401_d_n6: f64 = ((nv14 - 0.0) * s.dn[618][6]);let eq19_e401_d_n7: f64 = ((nv14 - 0.0) * s.dn[618][7]);let eq19_e401_d_n8: f64 = ((nv14 - 0.0) * s.dn[618][8]);let eq19_e401_d_n9: f64 = ((nv14 - 0.0) * s.dn[618][9]);let eq19_e401_d_n10: f64 = ((nv14 - 0.0) * s.dn[618][10]);let eq19_e401_d_n11: f64 = ((nv14 - 0.0) * s.dn[618][11]);let eq19_e401_d_n12: f64 = ((nv14 - 0.0) * s.dn[618][12]);let eq19_e401_d_n13: f64 = ((nv14 - 0.0) * s.dn[618][13]);let eq19_e401_d_n14: f64 = (s.v[618] + ((nv14 - 0.0) * s.dn[618][14]));let eq19_e401_d_n15: f64 = ((nv14 - 0.0) * s.dn[618][15]);let eq19_e401_d_n16: f64 = ((nv14 - 0.0) * s.dn[618][16]);let eq19_e401_d_n17: f64 = ((nv14 - 0.0) * s.dn[618][17]);let eq19_e401_d_n18: f64 = ((nv14 - 0.0) * s.dn[618][18]);let eq19_e401_d_b0: f64 = ((nv14 - 0.0) * s.db[618][0]);let eq19_e401_d_b1: f64 = ((nv14 - 0.0) * s.db[618][1]);let eq19_e401_d_b2: f64 = ((nv14 - 0.0) * s.db[618][2]);let eq19_e401_d_b3: f64 = ((nv14 - 0.0) * s.db[618][3]);let eq19_e401_d_b4: f64 = ((nv14 - 0.0) * s.db[618][4]);let eq19_e401_d_b5: f64 = ((nv14 - 0.0) * s.db[618][5]);let eq19_e401_d_b6: f64 = ((nv14 - 0.0) * s.db[618][6]);let eq19_e401_d_b7: f64 = ((nv14 - 0.0) * s.db[618][7]);let eq19_e401_d_b8: f64 = ((nv14 - 0.0) * s.db[618][8]);let eq19_e401_d_b9: f64 = ((nv14 - 0.0) * s.db[618][9]);let eq19_e401_d_b10: f64 = ((nv14 - 0.0) * s.db[618][10]);let eq19_e401_d_b11: f64 = ((nv14 - 0.0) * s.db[618][11]);let eq19_e401_d_b12: f64 = ((nv14 - 0.0) * s.db[618][12]);let eq19_e401_d_b13: f64 = ((nv14 - 0.0) * s.db[618][13]);let eq19_e401_d_b14: f64 = ((nv14 - 0.0) * s.db[618][14]);let eq19_e402_q: f64 = eq19_e401;let eq19_reactive_node_derivatives: [f64; 19] = [eq19_e401_d_n0, eq19_e401_d_n1, eq19_e401_d_n2, eq19_e401_d_n3, eq19_e401_d_n4, eq19_e401_d_n5, eq19_e401_d_n6, eq19_e401_d_n7, eq19_e401_d_n8, eq19_e401_d_n9, eq19_e401_d_n10, eq19_e401_d_n11, eq19_e401_d_n12, eq19_e401_d_n13, eq19_e401_d_n14, eq19_e401_d_n15, eq19_e401_d_n16, eq19_e401_d_n17, eq19_e401_d_n18];let eq19_reactive_branch_derivatives: [f64; 15] = [eq19_e401_d_b0, eq19_e401_d_b1, eq19_e401_d_b2, eq19_e401_d_b3, eq19_e401_d_b4, eq19_e401_d_b5, eq19_e401_d_b6, eq19_e401_d_b7, eq19_e401_d_b8, eq19_e401_d_b9, eq19_e401_d_b10, eq19_e401_d_b11, eq19_e401_d_b12, eq19_e401_d_b13, eq19_e401_d_b14];
        stamper.stamp_current_reactive_dense_local(
            Some(11),
            Some(6),
            &eq19_reactive_node_derivatives,
            &eq19_reactive_branch_derivatives,
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
        let (eq30_e483, eq30_e483_d_n0, eq30_e483_d_n1, eq30_e483_d_n2, eq30_e483_d_n3, eq30_e483_d_n4, eq30_e483_d_n5, eq30_e483_d_n6, eq30_e483_d_n7, eq30_e483_d_n8, eq30_e483_d_n9, eq30_e483_d_n10, eq30_e483_d_n11, eq30_e483_d_n12, eq30_e483_d_n13, eq30_e483_d_n14, eq30_e483_d_n15, eq30_e483_d_n16, eq30_e483_d_n17, eq30_e483_d_n18, eq30_e483_d_b0, eq30_e483_d_b1, eq30_e483_d_b2, eq30_e483_d_b3, eq30_e483_d_b4, eq30_e483_d_b5, eq30_e483_d_b6, eq30_e483_d_b7, eq30_e483_d_b8, eq30_e483_d_b9, eq30_e483_d_b10, eq30_e483_d_b11, eq30_e483_d_b12, eq30_e483_d_b13, eq30_e483_d_b14, eq30_e483_q,) = {
    if s.b[1846] {
        let eq30_e480: f64 = (s.v[563] * (nv10 - 0.0));let eq30_e480_d_n0: f64 = (s.dn[563][0] * (nv10 - 0.0));let eq30_e480_d_n1: f64 = (s.dn[563][1] * (nv10 - 0.0));let eq30_e480_d_n2: f64 = (s.dn[563][2] * (nv10 - 0.0));let eq30_e480_d_n3: f64 = (s.dn[563][3] * (nv10 - 0.0));let eq30_e480_d_n4: f64 = (s.dn[563][4] * (nv10 - 0.0));let eq30_e480_d_n5: f64 = (s.dn[563][5] * (nv10 - 0.0));let eq30_e480_d_n6: f64 = (s.dn[563][6] * (nv10 - 0.0));let eq30_e480_d_n7: f64 = (s.dn[563][7] * (nv10 - 0.0));let eq30_e480_d_n8: f64 = (s.dn[563][8] * (nv10 - 0.0));let eq30_e480_d_n9: f64 = (s.dn[563][9] * (nv10 - 0.0));let eq30_e480_d_n10: f64 = ((s.dn[563][10] * (nv10 - 0.0)) + s.v[563]);let eq30_e480_d_n11: f64 = (s.dn[563][11] * (nv10 - 0.0));let eq30_e480_d_n12: f64 = (s.dn[563][12] * (nv10 - 0.0));let eq30_e480_d_n13: f64 = (s.dn[563][13] * (nv10 - 0.0));let eq30_e480_d_n14: f64 = (s.dn[563][14] * (nv10 - 0.0));let eq30_e480_d_n15: f64 = (s.dn[563][15] * (nv10 - 0.0));let eq30_e480_d_n16: f64 = (s.dn[563][16] * (nv10 - 0.0));let eq30_e480_d_n17: f64 = (s.dn[563][17] * (nv10 - 0.0));let eq30_e480_d_n18: f64 = (s.dn[563][18] * (nv10 - 0.0));let eq30_e480_d_b0: f64 = (s.db[563][0] * (nv10 - 0.0));let eq30_e480_d_b1: f64 = (s.db[563][1] * (nv10 - 0.0));let eq30_e480_d_b2: f64 = (s.db[563][2] * (nv10 - 0.0));let eq30_e480_d_b3: f64 = (s.db[563][3] * (nv10 - 0.0));let eq30_e480_d_b4: f64 = (s.db[563][4] * (nv10 - 0.0));let eq30_e480_d_b5: f64 = (s.db[563][5] * (nv10 - 0.0));let eq30_e480_d_b6: f64 = (s.db[563][6] * (nv10 - 0.0));let eq30_e480_d_b7: f64 = (s.db[563][7] * (nv10 - 0.0));let eq30_e480_d_b8: f64 = (s.db[563][8] * (nv10 - 0.0));let eq30_e480_d_b9: f64 = (s.db[563][9] * (nv10 - 0.0));let eq30_e480_d_b10: f64 = (s.db[563][10] * (nv10 - 0.0));let eq30_e480_d_b11: f64 = (s.db[563][11] * (nv10 - 0.0));let eq30_e480_d_b12: f64 = (s.db[563][12] * (nv10 - 0.0));let eq30_e480_d_b13: f64 = (s.db[563][13] * (nv10 - 0.0));let eq30_e480_d_b14: f64 = (s.db[563][14] * (nv10 - 0.0));let eq30_e481_q: f64 = eq30_e480;
        (eq30_e480, eq30_e480_d_n0, eq30_e480_d_n1, eq30_e480_d_n2, eq30_e480_d_n3, eq30_e480_d_n4, eq30_e480_d_n5, eq30_e480_d_n6, eq30_e480_d_n7, eq30_e480_d_n8, eq30_e480_d_n9, eq30_e480_d_n10, eq30_e480_d_n11, eq30_e480_d_n12, eq30_e480_d_n13, eq30_e480_d_n14, eq30_e480_d_n15, eq30_e480_d_n16, eq30_e480_d_n17, eq30_e480_d_n18, eq30_e480_d_b0, eq30_e480_d_b1, eq30_e480_d_b2, eq30_e480_d_b3, eq30_e480_d_b4, eq30_e480_d_b5, eq30_e480_d_b6, eq30_e480_d_b7, eq30_e480_d_b8, eq30_e480_d_b9, eq30_e480_d_b10, eq30_e480_d_b11, eq30_e480_d_b12, eq30_e480_d_b13, eq30_e480_d_b14, eq30_e481_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_reactive_node_derivatives: [f64; 19] = [eq30_e483_d_n0, eq30_e483_d_n1, eq30_e483_d_n2, eq30_e483_d_n3, eq30_e483_d_n4, eq30_e483_d_n5, eq30_e483_d_n6, eq30_e483_d_n7, eq30_e483_d_n8, eq30_e483_d_n9, eq30_e483_d_n10, eq30_e483_d_n11, eq30_e483_d_n12, eq30_e483_d_n13, eq30_e483_d_n14, eq30_e483_d_n15, eq30_e483_d_n16, eq30_e483_d_n17, eq30_e483_d_n18];let eq30_reactive_branch_derivatives: [f64; 15] = [eq30_e483_d_b0, eq30_e483_d_b1, eq30_e483_d_b2, eq30_e483_d_b3, eq30_e483_d_b4, eq30_e483_d_b5, eq30_e483_d_b6, eq30_e483_d_b7, eq30_e483_d_b8, eq30_e483_d_b9, eq30_e483_d_b10, eq30_e483_d_b11, eq30_e483_d_b12, eq30_e483_d_b13, eq30_e483_d_b14];
        stamper.stamp_current_reactive_dense_local(
            Some(10),
            None,
            &eq30_reactive_node_derivatives,
            &eq30_reactive_branch_derivatives,
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
        let (eq34_e515, eq34_e515_d_n0, eq34_e515_d_n1, eq34_e515_d_n2, eq34_e515_d_n3, eq34_e515_d_n4, eq34_e515_d_n5, eq34_e515_d_n6, eq34_e515_d_n7, eq34_e515_d_n8, eq34_e515_d_n9, eq34_e515_d_n10, eq34_e515_d_n11, eq34_e515_d_n12, eq34_e515_d_n13, eq34_e515_d_n14, eq34_e515_d_n15, eq34_e515_d_n16, eq34_e515_d_n17, eq34_e515_d_n18, eq34_e515_d_b0, eq34_e515_d_b1, eq34_e515_d_b2, eq34_e515_d_b3, eq34_e515_d_b4, eq34_e515_d_b5, eq34_e515_d_b6, eq34_e515_d_b7, eq34_e515_d_b8, eq34_e515_d_b9, eq34_e515_d_b10, eq34_e515_d_b11, eq34_e515_d_b12, eq34_e515_d_b13, eq34_e515_d_b14, eq34_e515_q, eq34_e515_q_d_n0, eq34_e515_q_d_n1, eq34_e515_q_d_n2, eq34_e515_q_d_n3, eq34_e515_q_d_n4, eq34_e515_q_d_n5, eq34_e515_q_d_n6, eq34_e515_q_d_n7, eq34_e515_q_d_n8, eq34_e515_q_d_n9, eq34_e515_q_d_n10, eq34_e515_q_d_n11, eq34_e515_q_d_n12, eq34_e515_q_d_n13, eq34_e515_q_d_n14, eq34_e515_q_d_n15, eq34_e515_q_d_n16, eq34_e515_q_d_n17, eq34_e515_q_d_n18, eq34_e515_q_d_b0, eq34_e515_q_d_b1, eq34_e515_q_d_b2, eq34_e515_q_d_b3, eq34_e515_q_d_b4, eq34_e515_q_d_b5, eq34_e515_q_d_b6, eq34_e515_q_d_b7, eq34_e515_q_d_b8, eq34_e515_q_d_b9, eq34_e515_q_d_b10, eq34_e515_q_d_b11, eq34_e515_q_d_b12, eq34_e515_q_d_b13, eq34_e515_q_d_b14,) = {
    if s.b[1847] {
        let eq34_e511_q: f64 = s.v[283];let eq34_e512: f64 = (s.v[281] + s.v[283]);let eq34_e512_d_n0: f64 = (s.dn[281][0] + s.dn[283][0]);let eq34_e512_d_n1: f64 = (s.dn[281][1] + s.dn[283][1]);let eq34_e512_d_n2: f64 = (s.dn[281][2] + s.dn[283][2]);let eq34_e512_d_n3: f64 = (s.dn[281][3] + s.dn[283][3]);let eq34_e512_d_n4: f64 = (s.dn[281][4] + s.dn[283][4]);let eq34_e512_d_n5: f64 = (s.dn[281][5] + s.dn[283][5]);let eq34_e512_d_n6: f64 = (s.dn[281][6] + s.dn[283][6]);let eq34_e512_d_n7: f64 = (s.dn[281][7] + s.dn[283][7]);let eq34_e512_d_n8: f64 = (s.dn[281][8] + s.dn[283][8]);let eq34_e512_d_n9: f64 = (s.dn[281][9] + s.dn[283][9]);let eq34_e512_d_n10: f64 = (s.dn[281][10] + s.dn[283][10]);let eq34_e512_d_n11: f64 = (s.dn[281][11] + s.dn[283][11]);let eq34_e512_d_n12: f64 = (s.dn[281][12] + s.dn[283][12]);let eq34_e512_d_n13: f64 = (s.dn[281][13] + s.dn[283][13]);let eq34_e512_d_n14: f64 = (s.dn[281][14] + s.dn[283][14]);let eq34_e512_d_n15: f64 = (s.dn[281][15] + s.dn[283][15]);let eq34_e512_d_n16: f64 = (s.dn[281][16] + s.dn[283][16]);let eq34_e512_d_n17: f64 = (s.dn[281][17] + s.dn[283][17]);let eq34_e512_d_n18: f64 = (s.dn[281][18] + s.dn[283][18]);let eq34_e512_d_b0: f64 = (s.db[281][0] + s.db[283][0]);let eq34_e512_d_b1: f64 = (s.db[281][1] + s.db[283][1]);let eq34_e512_d_b2: f64 = (s.db[281][2] + s.db[283][2]);let eq34_e512_d_b3: f64 = (s.db[281][3] + s.db[283][3]);let eq34_e512_d_b4: f64 = (s.db[281][4] + s.db[283][4]);let eq34_e512_d_b5: f64 = (s.db[281][5] + s.db[283][5]);let eq34_e512_d_b6: f64 = (s.db[281][6] + s.db[283][6]);let eq34_e512_d_b7: f64 = (s.db[281][7] + s.db[283][7]);let eq34_e512_d_b8: f64 = (s.db[281][8] + s.db[283][8]);let eq34_e512_d_b9: f64 = (s.db[281][9] + s.db[283][9]);let eq34_e512_d_b10: f64 = (s.db[281][10] + s.db[283][10]);let eq34_e512_d_b11: f64 = (s.db[281][11] + s.db[283][11]);let eq34_e512_d_b12: f64 = (s.db[281][12] + s.db[283][12]);let eq34_e512_d_b13: f64 = (s.db[281][13] + s.db[283][13]);let eq34_e512_d_b14: f64 = (s.db[281][14] + s.db[283][14]);let eq34_e512_q: f64 = eq34_e511_q;let eq34_e513: f64 = (p.p50 * eq34_e512);let eq34_e513_d_n0: f64 = (p.p50 * eq34_e512_d_n0);let eq34_e513_d_n1: f64 = (p.p50 * eq34_e512_d_n1);let eq34_e513_d_n2: f64 = (p.p50 * eq34_e512_d_n2);let eq34_e513_d_n3: f64 = (p.p50 * eq34_e512_d_n3);let eq34_e513_d_n4: f64 = (p.p50 * eq34_e512_d_n4);let eq34_e513_d_n5: f64 = (p.p50 * eq34_e512_d_n5);let eq34_e513_d_n6: f64 = (p.p50 * eq34_e512_d_n6);let eq34_e513_d_n7: f64 = (p.p50 * eq34_e512_d_n7);let eq34_e513_d_n8: f64 = (p.p50 * eq34_e512_d_n8);let eq34_e513_d_n9: f64 = (p.p50 * eq34_e512_d_n9);let eq34_e513_d_n10: f64 = (p.p50 * eq34_e512_d_n10);let eq34_e513_d_n11: f64 = (p.p50 * eq34_e512_d_n11);let eq34_e513_d_n12: f64 = (p.p50 * eq34_e512_d_n12);let eq34_e513_d_n13: f64 = (p.p50 * eq34_e512_d_n13);let eq34_e513_d_n14: f64 = (p.p50 * eq34_e512_d_n14);let eq34_e513_d_n15: f64 = (p.p50 * eq34_e512_d_n15);let eq34_e513_d_n16: f64 = (p.p50 * eq34_e512_d_n16);let eq34_e513_d_n17: f64 = (p.p50 * eq34_e512_d_n17);let eq34_e513_d_n18: f64 = (p.p50 * eq34_e512_d_n18);let eq34_e513_d_b0: f64 = (p.p50 * eq34_e512_d_b0);let eq34_e513_d_b1: f64 = (p.p50 * eq34_e512_d_b1);let eq34_e513_d_b2: f64 = (p.p50 * eq34_e512_d_b2);let eq34_e513_d_b3: f64 = (p.p50 * eq34_e512_d_b3);let eq34_e513_d_b4: f64 = (p.p50 * eq34_e512_d_b4);let eq34_e513_d_b5: f64 = (p.p50 * eq34_e512_d_b5);let eq34_e513_d_b6: f64 = (p.p50 * eq34_e512_d_b6);let eq34_e513_d_b7: f64 = (p.p50 * eq34_e512_d_b7);let eq34_e513_d_b8: f64 = (p.p50 * eq34_e512_d_b8);let eq34_e513_d_b9: f64 = (p.p50 * eq34_e512_d_b9);let eq34_e513_d_b10: f64 = (p.p50 * eq34_e512_d_b10);let eq34_e513_d_b11: f64 = (p.p50 * eq34_e512_d_b11);let eq34_e513_d_b12: f64 = (p.p50 * eq34_e512_d_b12);let eq34_e513_d_b13: f64 = (p.p50 * eq34_e512_d_b13);let eq34_e513_d_b14: f64 = (p.p50 * eq34_e512_d_b14);let eq34_e513_q: f64 = (p.p50 * eq34_e512_q);
        (eq34_e513, eq34_e513_d_n0, eq34_e513_d_n1, eq34_e513_d_n2, eq34_e513_d_n3, eq34_e513_d_n4, eq34_e513_d_n5, eq34_e513_d_n6, eq34_e513_d_n7, eq34_e513_d_n8, eq34_e513_d_n9, eq34_e513_d_n10, eq34_e513_d_n11, eq34_e513_d_n12, eq34_e513_d_n13, eq34_e513_d_n14, eq34_e513_d_n15, eq34_e513_d_n16, eq34_e513_d_n17, eq34_e513_d_n18, eq34_e513_d_b0, eq34_e513_d_b1, eq34_e513_d_b2, eq34_e513_d_b3, eq34_e513_d_b4, eq34_e513_d_b5, eq34_e513_d_b6, eq34_e513_d_b7, eq34_e513_d_b8, eq34_e513_d_b9, eq34_e513_d_b10, eq34_e513_d_b11, eq34_e513_d_b12, eq34_e513_d_b13, eq34_e513_d_b14, eq34_e513_q, (p.p50 * s.dn[283][0]), (p.p50 * s.dn[283][1]), (p.p50 * s.dn[283][2]), (p.p50 * s.dn[283][3]), (p.p50 * s.dn[283][4]), (p.p50 * s.dn[283][5]), (p.p50 * s.dn[283][6]), (p.p50 * s.dn[283][7]), (p.p50 * s.dn[283][8]), (p.p50 * s.dn[283][9]), (p.p50 * s.dn[283][10]), (p.p50 * s.dn[283][11]), (p.p50 * s.dn[283][12]), (p.p50 * s.dn[283][13]), (p.p50 * s.dn[283][14]), (p.p50 * s.dn[283][15]), (p.p50 * s.dn[283][16]), (p.p50 * s.dn[283][17]), (p.p50 * s.dn[283][18]), (p.p50 * s.db[283][0]), (p.p50 * s.db[283][1]), (p.p50 * s.db[283][2]), (p.p50 * s.db[283][3]), (p.p50 * s.db[283][4]), (p.p50 * s.db[283][5]), (p.p50 * s.db[283][6]), (p.p50 * s.db[283][7]), (p.p50 * s.db[283][8]), (p.p50 * s.db[283][9]), (p.p50 * s.db[283][10]), (p.p50 * s.db[283][11]), (p.p50 * s.db[283][12]), (p.p50 * s.db[283][13]), (p.p50 * s.db[283][14]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_reactive_node_derivatives: [f64; 19] = [eq34_e515_q_d_n0, eq34_e515_q_d_n1, eq34_e515_q_d_n2, eq34_e515_q_d_n3, eq34_e515_q_d_n4, eq34_e515_q_d_n5, eq34_e515_q_d_n6, eq34_e515_q_d_n7, eq34_e515_q_d_n8, eq34_e515_q_d_n9, eq34_e515_q_d_n10, eq34_e515_q_d_n11, eq34_e515_q_d_n12, eq34_e515_q_d_n13, eq34_e515_q_d_n14, eq34_e515_q_d_n15, eq34_e515_q_d_n16, eq34_e515_q_d_n17, eq34_e515_q_d_n18];let eq34_reactive_branch_derivatives: [f64; 15] = [eq34_e515_q_d_b0, eq34_e515_q_d_b1, eq34_e515_q_d_b2, eq34_e515_q_d_b3, eq34_e515_q_d_b4, eq34_e515_q_d_b5, eq34_e515_q_d_b6, eq34_e515_q_d_b7, eq34_e515_q_d_b8, eq34_e515_q_d_b9, eq34_e515_q_d_b10, eq34_e515_q_d_b11, eq34_e515_q_d_b12, eq34_e515_q_d_b13, eq34_e515_q_d_b14];
        stamper.stamp_current_reactive_dense_local(
            Some(9),
            Some(7),
            &eq34_reactive_node_derivatives,
            &eq34_reactive_branch_derivatives,
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
        let (eq35_e524, eq35_e524_d_n0, eq35_e524_d_n1, eq35_e524_d_n2, eq35_e524_d_n3, eq35_e524_d_n4, eq35_e524_d_n5, eq35_e524_d_n6, eq35_e524_d_n7, eq35_e524_d_n8, eq35_e524_d_n9, eq35_e524_d_n10, eq35_e524_d_n11, eq35_e524_d_n12, eq35_e524_d_n13, eq35_e524_d_n14, eq35_e524_d_n15, eq35_e524_d_n16, eq35_e524_d_n17, eq35_e524_d_n18, eq35_e524_d_b0, eq35_e524_d_b1, eq35_e524_d_b2, eq35_e524_d_b3, eq35_e524_d_b4, eq35_e524_d_b5, eq35_e524_d_b6, eq35_e524_d_b7, eq35_e524_d_b8, eq35_e524_d_b9, eq35_e524_d_b10, eq35_e524_d_b11, eq35_e524_d_b12, eq35_e524_d_b13, eq35_e524_d_b14, eq35_e524_q, eq35_e524_q_d_n0, eq35_e524_q_d_n1, eq35_e524_q_d_n2, eq35_e524_q_d_n3, eq35_e524_q_d_n4, eq35_e524_q_d_n5, eq35_e524_q_d_n6, eq35_e524_q_d_n7, eq35_e524_q_d_n8, eq35_e524_q_d_n9, eq35_e524_q_d_n10, eq35_e524_q_d_n11, eq35_e524_q_d_n12, eq35_e524_q_d_n13, eq35_e524_q_d_n14, eq35_e524_q_d_n15, eq35_e524_q_d_n16, eq35_e524_q_d_n17, eq35_e524_q_d_n18, eq35_e524_q_d_b0, eq35_e524_q_d_b1, eq35_e524_q_d_b2, eq35_e524_q_d_b3, eq35_e524_q_d_b4, eq35_e524_q_d_b5, eq35_e524_q_d_b6, eq35_e524_q_d_b7, eq35_e524_q_d_b8, eq35_e524_q_d_b9, eq35_e524_q_d_b10, eq35_e524_q_d_b11, eq35_e524_q_d_b12, eq35_e524_q_d_b13, eq35_e524_q_d_b14,) = {
    if s.b[1847] {
        let eq35_e520_q: f64 = s.v[284];let eq35_e521: f64 = (s.v[282] + s.v[284]);let eq35_e521_d_n0: f64 = (s.dn[282][0] + s.dn[284][0]);let eq35_e521_d_n1: f64 = (s.dn[282][1] + s.dn[284][1]);let eq35_e521_d_n2: f64 = (s.dn[282][2] + s.dn[284][2]);let eq35_e521_d_n3: f64 = (s.dn[282][3] + s.dn[284][3]);let eq35_e521_d_n4: f64 = (s.dn[282][4] + s.dn[284][4]);let eq35_e521_d_n5: f64 = (s.dn[282][5] + s.dn[284][5]);let eq35_e521_d_n6: f64 = (s.dn[282][6] + s.dn[284][6]);let eq35_e521_d_n7: f64 = (s.dn[282][7] + s.dn[284][7]);let eq35_e521_d_n8: f64 = (s.dn[282][8] + s.dn[284][8]);let eq35_e521_d_n9: f64 = (s.dn[282][9] + s.dn[284][9]);let eq35_e521_d_n10: f64 = (s.dn[282][10] + s.dn[284][10]);let eq35_e521_d_n11: f64 = (s.dn[282][11] + s.dn[284][11]);let eq35_e521_d_n12: f64 = (s.dn[282][12] + s.dn[284][12]);let eq35_e521_d_n13: f64 = (s.dn[282][13] + s.dn[284][13]);let eq35_e521_d_n14: f64 = (s.dn[282][14] + s.dn[284][14]);let eq35_e521_d_n15: f64 = (s.dn[282][15] + s.dn[284][15]);let eq35_e521_d_n16: f64 = (s.dn[282][16] + s.dn[284][16]);let eq35_e521_d_n17: f64 = (s.dn[282][17] + s.dn[284][17]);let eq35_e521_d_n18: f64 = (s.dn[282][18] + s.dn[284][18]);let eq35_e521_d_b0: f64 = (s.db[282][0] + s.db[284][0]);let eq35_e521_d_b1: f64 = (s.db[282][1] + s.db[284][1]);let eq35_e521_d_b2: f64 = (s.db[282][2] + s.db[284][2]);let eq35_e521_d_b3: f64 = (s.db[282][3] + s.db[284][3]);let eq35_e521_d_b4: f64 = (s.db[282][4] + s.db[284][4]);let eq35_e521_d_b5: f64 = (s.db[282][5] + s.db[284][5]);let eq35_e521_d_b6: f64 = (s.db[282][6] + s.db[284][6]);let eq35_e521_d_b7: f64 = (s.db[282][7] + s.db[284][7]);let eq35_e521_d_b8: f64 = (s.db[282][8] + s.db[284][8]);let eq35_e521_d_b9: f64 = (s.db[282][9] + s.db[284][9]);let eq35_e521_d_b10: f64 = (s.db[282][10] + s.db[284][10]);let eq35_e521_d_b11: f64 = (s.db[282][11] + s.db[284][11]);let eq35_e521_d_b12: f64 = (s.db[282][12] + s.db[284][12]);let eq35_e521_d_b13: f64 = (s.db[282][13] + s.db[284][13]);let eq35_e521_d_b14: f64 = (s.db[282][14] + s.db[284][14]);let eq35_e521_q: f64 = eq35_e520_q;let eq35_e522: f64 = (p.p50 * eq35_e521);let eq35_e522_d_n0: f64 = (p.p50 * eq35_e521_d_n0);let eq35_e522_d_n1: f64 = (p.p50 * eq35_e521_d_n1);let eq35_e522_d_n2: f64 = (p.p50 * eq35_e521_d_n2);let eq35_e522_d_n3: f64 = (p.p50 * eq35_e521_d_n3);let eq35_e522_d_n4: f64 = (p.p50 * eq35_e521_d_n4);let eq35_e522_d_n5: f64 = (p.p50 * eq35_e521_d_n5);let eq35_e522_d_n6: f64 = (p.p50 * eq35_e521_d_n6);let eq35_e522_d_n7: f64 = (p.p50 * eq35_e521_d_n7);let eq35_e522_d_n8: f64 = (p.p50 * eq35_e521_d_n8);let eq35_e522_d_n9: f64 = (p.p50 * eq35_e521_d_n9);let eq35_e522_d_n10: f64 = (p.p50 * eq35_e521_d_n10);let eq35_e522_d_n11: f64 = (p.p50 * eq35_e521_d_n11);let eq35_e522_d_n12: f64 = (p.p50 * eq35_e521_d_n12);let eq35_e522_d_n13: f64 = (p.p50 * eq35_e521_d_n13);let eq35_e522_d_n14: f64 = (p.p50 * eq35_e521_d_n14);let eq35_e522_d_n15: f64 = (p.p50 * eq35_e521_d_n15);let eq35_e522_d_n16: f64 = (p.p50 * eq35_e521_d_n16);let eq35_e522_d_n17: f64 = (p.p50 * eq35_e521_d_n17);let eq35_e522_d_n18: f64 = (p.p50 * eq35_e521_d_n18);let eq35_e522_d_b0: f64 = (p.p50 * eq35_e521_d_b0);let eq35_e522_d_b1: f64 = (p.p50 * eq35_e521_d_b1);let eq35_e522_d_b2: f64 = (p.p50 * eq35_e521_d_b2);let eq35_e522_d_b3: f64 = (p.p50 * eq35_e521_d_b3);let eq35_e522_d_b4: f64 = (p.p50 * eq35_e521_d_b4);let eq35_e522_d_b5: f64 = (p.p50 * eq35_e521_d_b5);let eq35_e522_d_b6: f64 = (p.p50 * eq35_e521_d_b6);let eq35_e522_d_b7: f64 = (p.p50 * eq35_e521_d_b7);let eq35_e522_d_b8: f64 = (p.p50 * eq35_e521_d_b8);let eq35_e522_d_b9: f64 = (p.p50 * eq35_e521_d_b9);let eq35_e522_d_b10: f64 = (p.p50 * eq35_e521_d_b10);let eq35_e522_d_b11: f64 = (p.p50 * eq35_e521_d_b11);let eq35_e522_d_b12: f64 = (p.p50 * eq35_e521_d_b12);let eq35_e522_d_b13: f64 = (p.p50 * eq35_e521_d_b13);let eq35_e522_d_b14: f64 = (p.p50 * eq35_e521_d_b14);let eq35_e522_q: f64 = (p.p50 * eq35_e521_q);
        (eq35_e522, eq35_e522_d_n0, eq35_e522_d_n1, eq35_e522_d_n2, eq35_e522_d_n3, eq35_e522_d_n4, eq35_e522_d_n5, eq35_e522_d_n6, eq35_e522_d_n7, eq35_e522_d_n8, eq35_e522_d_n9, eq35_e522_d_n10, eq35_e522_d_n11, eq35_e522_d_n12, eq35_e522_d_n13, eq35_e522_d_n14, eq35_e522_d_n15, eq35_e522_d_n16, eq35_e522_d_n17, eq35_e522_d_n18, eq35_e522_d_b0, eq35_e522_d_b1, eq35_e522_d_b2, eq35_e522_d_b3, eq35_e522_d_b4, eq35_e522_d_b5, eq35_e522_d_b6, eq35_e522_d_b7, eq35_e522_d_b8, eq35_e522_d_b9, eq35_e522_d_b10, eq35_e522_d_b11, eq35_e522_d_b12, eq35_e522_d_b13, eq35_e522_d_b14, eq35_e522_q, (p.p50 * s.dn[284][0]), (p.p50 * s.dn[284][1]), (p.p50 * s.dn[284][2]), (p.p50 * s.dn[284][3]), (p.p50 * s.dn[284][4]), (p.p50 * s.dn[284][5]), (p.p50 * s.dn[284][6]), (p.p50 * s.dn[284][7]), (p.p50 * s.dn[284][8]), (p.p50 * s.dn[284][9]), (p.p50 * s.dn[284][10]), (p.p50 * s.dn[284][11]), (p.p50 * s.dn[284][12]), (p.p50 * s.dn[284][13]), (p.p50 * s.dn[284][14]), (p.p50 * s.dn[284][15]), (p.p50 * s.dn[284][16]), (p.p50 * s.dn[284][17]), (p.p50 * s.dn[284][18]), (p.p50 * s.db[284][0]), (p.p50 * s.db[284][1]), (p.p50 * s.db[284][2]), (p.p50 * s.db[284][3]), (p.p50 * s.db[284][4]), (p.p50 * s.db[284][5]), (p.p50 * s.db[284][6]), (p.p50 * s.db[284][7]), (p.p50 * s.db[284][8]), (p.p50 * s.db[284][9]), (p.p50 * s.db[284][10]), (p.p50 * s.db[284][11]), (p.p50 * s.db[284][12]), (p.p50 * s.db[284][13]), (p.p50 * s.db[284][14]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_reactive_node_derivatives: [f64; 19] = [eq35_e524_q_d_n0, eq35_e524_q_d_n1, eq35_e524_q_d_n2, eq35_e524_q_d_n3, eq35_e524_q_d_n4, eq35_e524_q_d_n5, eq35_e524_q_d_n6, eq35_e524_q_d_n7, eq35_e524_q_d_n8, eq35_e524_q_d_n9, eq35_e524_q_d_n10, eq35_e524_q_d_n11, eq35_e524_q_d_n12, eq35_e524_q_d_n13, eq35_e524_q_d_n14, eq35_e524_q_d_n15, eq35_e524_q_d_n16, eq35_e524_q_d_n17, eq35_e524_q_d_n18];let eq35_reactive_branch_derivatives: [f64; 15] = [eq35_e524_q_d_b0, eq35_e524_q_d_b1, eq35_e524_q_d_b2, eq35_e524_q_d_b3, eq35_e524_q_d_b4, eq35_e524_q_d_b5, eq35_e524_q_d_b6, eq35_e524_q_d_b7, eq35_e524_q_d_b8, eq35_e524_q_d_b9, eq35_e524_q_d_b10, eq35_e524_q_d_b11, eq35_e524_q_d_b12, eq35_e524_q_d_b13, eq35_e524_q_d_b14];
        stamper.stamp_current_reactive_dense_local(
            Some(8),
            Some(6),
            &eq35_reactive_node_derivatives,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq46_e608, eq46_e608_d_n18, eq46_e608_q,) = {
    if (s.b[1847] && (p.p34 != 0.0)) {
        let eq46_e603: f64 = (1e-9 / 0.0001);let eq46_e605: f64 = (eq46_e603 * (nv18 - 0.0));let eq46_e606_q: f64 = eq46_e605;
        (eq46_e605, eq46_e603, eq46_e606_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(18),
            None,
            18,
            multiplicity * (eq46_e608_d_n18),
        );
        let (eq47_e619, eq47_e619_d_n13, eq47_e619_q,) = {
    if (s.b[1847] && (p.p34 != 0.0)) {
        let eq47_e614: f64 = (1e-9 / 0.0001);let eq47_e616: f64 = (eq47_e614 * (nv13 - 0.0));let eq47_e617_q: f64 = eq47_e616;
        (eq47_e616, eq47_e614, eq47_e617_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(13),
            None,
            13,
            multiplicity * (eq47_e619_d_n13),
        );
        let (eq52_e658, eq52_e658_d_n17, eq52_e658_q,) = {
    if (s.b[1847] && s.b[1848]) {
        let eq52_e653: f64 = (1e-9 / 0.0001);let eq52_e655: f64 = (eq52_e653 * (nv17 - 0.0));let eq52_e656_q: f64 = eq52_e655;
        (eq52_e655, eq52_e653, eq52_e656_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(17),
            None,
            17,
            multiplicity * (eq52_e658_d_n17),
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
        let (eq59_e716, eq59_e716_d_n17, eq59_e716_q,) = {
    if ((!s.b[1847]) && (p.p37 != 0.0)) {
        let eq59_e711: f64 = (1e-9 / 0.0001);let eq59_e713: f64 = (eq59_e711 * (nv17 - 0.0));let eq59_e714_q: f64 = eq59_e713;
        (eq59_e713, eq59_e711, eq59_e714_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(17),
            None,
            17,
            multiplicity * (eq59_e716_d_n17),
        );
        let (eq67_e784, eq67_e784_d_n15, eq67_e784_q,) = {
    if ((!s.b[1847]) && (p.p34 != 0.0)) {
        let eq67_e779: f64 = (1e-9 / 0.0001);let eq67_e781: f64 = (eq67_e779 * (nv15 - 0.0));let eq67_e782_q: f64 = eq67_e781;
        (eq67_e781, eq67_e779, eq67_e782_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(15),
            None,
            15,
            multiplicity * (eq67_e784_d_n15),
        );
        let (eq68_e796, eq68_e796_d_n16, eq68_e796_q,) = {
    if ((!s.b[1847]) && (p.p34 != 0.0)) {
        let eq68_e791: f64 = (1e-9 / 0.0001);let eq68_e793: f64 = (eq68_e791 * (nv16 - 0.0));let eq68_e794_q: f64 = eq68_e793;
        (eq68_e793, eq68_e791, eq68_e794_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(16),
            None,
            16,
            multiplicity * (eq68_e796_d_n16),
        );
        let (eq69_e808, eq69_e808_d_n13, eq69_e808_q,) = {
    if ((!s.b[1847]) && (p.p34 != 0.0)) {
        let eq69_e803: f64 = (1e-9 / 0.0001);let eq69_e805: f64 = (eq69_e803 * (nv13 - 0.0));let eq69_e806_q: f64 = eq69_e805;
        (eq69_e805, eq69_e803, eq69_e806_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(13),
            None,
            13,
            multiplicity * (eq69_e808_d_n13),
        );
    }
}
