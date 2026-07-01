#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_13(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
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
        var_guard571: f64,
        var_guard572: f64,
        var_guard574: f64,
        var_guard575: f64,
        var_qd_fp4s: f64,
        var_qd_fp4s_dn0: f64,
        var_qd_fp4s_dn1: f64,
        var_qd_fp4s_dn12: f64,
        var_qd_fp4s_dn14: f64,
        var_qd_fp4s_dn15: f64,
        var_qd_fp4s_dn16: f64,
        var_qd_fp4s_dn17: f64,
        var_qd_fp4s_dn18: f64,
        var_qd_fp4s_dn19: f64,
        var_qd_fp4s_dn2: f64,
        var_qd_fp4s_dn20: f64,
        var_qd_fp4s_dn21: f64,
        var_qd_fp4s_dn22: f64,
        var_qd_fp4s_dn3: f64,
        var_qd_fp4s_dn4: f64,
        var_qd_fp4s_dn5: f64,
        var_qd_fp4s_dn6: f64,
        var_qd_fp4s_dn7: f64,
        var_qd_fp4s_dn8: f64,
        var_qd_fp4s_dn9: f64,
        var_qfr: f64,
        var_qfr2: f64,
        var_qfr2_dn0: f64,
        var_qfr2_dn1: f64,
        var_qfr2_dn12: f64,
        var_qfr2_dn14: f64,
        var_qfr2_dn15: f64,
        var_qfr2_dn16: f64,
        var_qfr2_dn17: f64,
        var_qfr2_dn18: f64,
        var_qfr2_dn19: f64,
        var_qfr2_dn2: f64,
        var_qfr2_dn20: f64,
        var_qfr2_dn21: f64,
        var_qfr2_dn22: f64,
        var_qfr2_dn3: f64,
        var_qfr2_dn4: f64,
        var_qfr2_dn5: f64,
        var_qfr2_dn6: f64,
        var_qfr2_dn7: f64,
        var_qfr2_dn8: f64,
        var_qfr2_dn9: f64,
        var_qfr3: f64,
        var_qfr3_dn0: f64,
        var_qfr3_dn2: f64,
        var_qfr_dn0: f64,
        var_qfr_dn2: f64,
        var_qfr_dn4: f64,
        var_qg_fp4s: f64,
        var_qg_fp4s_dn0: f64,
        var_qg_fp4s_dn1: f64,
        var_qg_fp4s_dn12: f64,
        var_qg_fp4s_dn14: f64,
        var_qg_fp4s_dn15: f64,
        var_qg_fp4s_dn16: f64,
        var_qg_fp4s_dn17: f64,
        var_qg_fp4s_dn18: f64,
        var_qg_fp4s_dn19: f64,
        var_qg_fp4s_dn2: f64,
        var_qg_fp4s_dn20: f64,
        var_qg_fp4s_dn21: f64,
        var_qg_fp4s_dn22: f64,
        var_qg_fp4s_dn3: f64,
        var_qg_fp4s_dn4: f64,
        var_qg_fp4s_dn5: f64,
        var_qg_fp4s_dn6: f64,
        var_qg_fp4s_dn7: f64,
        var_qg_fp4s_dn8: f64,
        var_qg_fp4s_dn9: f64,
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let (eq209_e2623, eq209_e2623_d_n0, eq209_e2623_d_n1, eq209_e2623_d_n2, eq209_e2623_d_n3, eq209_e2623_d_n4, eq209_e2623_d_n5, eq209_e2623_d_n6, eq209_e2623_d_n7, eq209_e2623_d_n8, eq209_e2623_d_n9, eq209_e2623_d_n12, eq209_e2623_d_n14, eq209_e2623_d_n15, eq209_e2623_d_n16, eq209_e2623_d_n17, eq209_e2623_d_n18, eq209_e2623_d_n19, eq209_e2623_d_n20, eq209_e2623_d_n21, eq209_e2623_d_n22,) = {
    if ((var_guard571 != 0.0) && (var_guard572 != 0.0)) {
        let eq209_e2619: f64 = (p.p254 * var_qg_fp4s);
        let eq209_e2619_d_n0: f64 = (p.p254 * var_qg_fp4s_dn0);
        let eq209_e2619_d_n1: f64 = (p.p254 * var_qg_fp4s_dn1);
        let eq209_e2619_d_n2: f64 = (p.p254 * var_qg_fp4s_dn2);
        let eq209_e2619_d_n3: f64 = (p.p254 * var_qg_fp4s_dn3);
        let eq209_e2619_d_n4: f64 = (p.p254 * var_qg_fp4s_dn4);
        let eq209_e2619_d_n5: f64 = (p.p254 * var_qg_fp4s_dn5);
        let eq209_e2619_d_n6: f64 = (p.p254 * var_qg_fp4s_dn6);
        let eq209_e2619_d_n7: f64 = (p.p254 * var_qg_fp4s_dn7);
        let eq209_e2619_d_n8: f64 = (p.p254 * var_qg_fp4s_dn8);
        let eq209_e2619_d_n9: f64 = (p.p254 * var_qg_fp4s_dn9);
        let eq209_e2619_d_n12: f64 = (p.p254 * var_qg_fp4s_dn12);
        let eq209_e2619_d_n14: f64 = (p.p254 * var_qg_fp4s_dn14);
        let eq209_e2619_d_n15: f64 = (p.p254 * var_qg_fp4s_dn15);
        let eq209_e2619_d_n16: f64 = (p.p254 * var_qg_fp4s_dn16);
        let eq209_e2619_d_n17: f64 = (p.p254 * var_qg_fp4s_dn17);
        let eq209_e2619_d_n18: f64 = (p.p254 * var_qg_fp4s_dn18);
        let eq209_e2619_d_n19: f64 = (p.p254 * var_qg_fp4s_dn19);
        let eq209_e2619_d_n20: f64 = (p.p254 * var_qg_fp4s_dn20);
        let eq209_e2619_d_n21: f64 = (p.p254 * var_qg_fp4s_dn21);
        let eq209_e2619_d_n22: f64 = (p.p254 * var_qg_fp4s_dn22);
        let eq209_e2620: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 108, eq209_e2619);
        let eq209_e2621: f64 = (p.p7 * eq209_e2620);
        let eq209_e2621_d_n0: f64 = (p.p7 * (eq209_e2619_d_n0 * ddt_scale));
        let eq209_e2621_d_n1: f64 = (p.p7 * (eq209_e2619_d_n1 * ddt_scale));
        let eq209_e2621_d_n2: f64 = (p.p7 * (eq209_e2619_d_n2 * ddt_scale));
        let eq209_e2621_d_n3: f64 = (p.p7 * (eq209_e2619_d_n3 * ddt_scale));
        let eq209_e2621_d_n4: f64 = (p.p7 * (eq209_e2619_d_n4 * ddt_scale));
        let eq209_e2621_d_n5: f64 = (p.p7 * (eq209_e2619_d_n5 * ddt_scale));
        let eq209_e2621_d_n6: f64 = (p.p7 * (eq209_e2619_d_n6 * ddt_scale));
        let eq209_e2621_d_n7: f64 = (p.p7 * (eq209_e2619_d_n7 * ddt_scale));
        let eq209_e2621_d_n8: f64 = (p.p7 * (eq209_e2619_d_n8 * ddt_scale));
        let eq209_e2621_d_n9: f64 = (p.p7 * (eq209_e2619_d_n9 * ddt_scale));
        let eq209_e2621_d_n12: f64 = (p.p7 * (eq209_e2619_d_n12 * ddt_scale));
        let eq209_e2621_d_n14: f64 = (p.p7 * (eq209_e2619_d_n14 * ddt_scale));
        let eq209_e2621_d_n15: f64 = (p.p7 * (eq209_e2619_d_n15 * ddt_scale));
        let eq209_e2621_d_n16: f64 = (p.p7 * (eq209_e2619_d_n16 * ddt_scale));
        let eq209_e2621_d_n17: f64 = (p.p7 * (eq209_e2619_d_n17 * ddt_scale));
        let eq209_e2621_d_n18: f64 = (p.p7 * (eq209_e2619_d_n18 * ddt_scale));
        let eq209_e2621_d_n19: f64 = (p.p7 * (eq209_e2619_d_n19 * ddt_scale));
        let eq209_e2621_d_n20: f64 = (p.p7 * (eq209_e2619_d_n20 * ddt_scale));
        let eq209_e2621_d_n21: f64 = (p.p7 * (eq209_e2619_d_n21 * ddt_scale));
        let eq209_e2621_d_n22: f64 = (p.p7 * (eq209_e2619_d_n22 * ddt_scale));
        (eq209_e2621, eq209_e2621_d_n0, eq209_e2621_d_n1, eq209_e2621_d_n2, eq209_e2621_d_n3, eq209_e2621_d_n4, eq209_e2621_d_n5, eq209_e2621_d_n6, eq209_e2621_d_n7, eq209_e2621_d_n8, eq209_e2621_d_n9, eq209_e2621_d_n12, eq209_e2621_d_n14, eq209_e2621_d_n15, eq209_e2621_d_n16, eq209_e2621_d_n17, eq209_e2621_d_n18, eq209_e2621_d_n19, eq209_e2621_d_n20, eq209_e2621_d_n21, eq209_e2621_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq209_value: f64 = eq209_e2623;
        let eq209_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq209_node_derivatives: [f64; 20] = [eq209_e2623_d_n0, eq209_e2623_d_n1, eq209_e2623_d_n2, eq209_e2623_d_n3, eq209_e2623_d_n4, eq209_e2623_d_n5, eq209_e2623_d_n6, eq209_e2623_d_n7, eq209_e2623_d_n8, eq209_e2623_d_n9, eq209_e2623_d_n12, eq209_e2623_d_n14, eq209_e2623_d_n15, eq209_e2623_d_n16, eq209_e2623_d_n17, eq209_e2623_d_n18, eq209_e2623_d_n19, eq209_e2623_d_n20, eq209_e2623_d_n21, eq209_e2623_d_n22];
        let eq209_branch_derivative_indices: [usize; 0] = [];
        let eq209_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(22),
            multiplicity * (eq209_value),
            &eq209_node_derivative_indices,
            &eq209_node_derivatives,
            &eq209_branch_derivative_indices,
            &eq209_branch_derivatives,
            multiplicity,
        );
        let (eq210_e2633, eq210_e2633_d_n0, eq210_e2633_d_n1, eq210_e2633_d_n2, eq210_e2633_d_n3, eq210_e2633_d_n4, eq210_e2633_d_n5, eq210_e2633_d_n6, eq210_e2633_d_n7, eq210_e2633_d_n8, eq210_e2633_d_n9, eq210_e2633_d_n12, eq210_e2633_d_n14, eq210_e2633_d_n15, eq210_e2633_d_n16, eq210_e2633_d_n17, eq210_e2633_d_n18, eq210_e2633_d_n19, eq210_e2633_d_n20, eq210_e2633_d_n21, eq210_e2633_d_n22,) = {
    if ((var_guard571 == 0.0) && (var_guard574 != 0.0)) {
        let eq210_e2630: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 109, var_qd_fp4s);
        let eq210_e2631: f64 = (p.p7 * eq210_e2630);
        let eq210_e2631_d_n0: f64 = (p.p7 * (var_qd_fp4s_dn0 * ddt_scale));
        let eq210_e2631_d_n1: f64 = (p.p7 * (var_qd_fp4s_dn1 * ddt_scale));
        let eq210_e2631_d_n2: f64 = (p.p7 * (var_qd_fp4s_dn2 * ddt_scale));
        let eq210_e2631_d_n3: f64 = (p.p7 * (var_qd_fp4s_dn3 * ddt_scale));
        let eq210_e2631_d_n4: f64 = (p.p7 * (var_qd_fp4s_dn4 * ddt_scale));
        let eq210_e2631_d_n5: f64 = (p.p7 * (var_qd_fp4s_dn5 * ddt_scale));
        let eq210_e2631_d_n6: f64 = (p.p7 * (var_qd_fp4s_dn6 * ddt_scale));
        let eq210_e2631_d_n7: f64 = (p.p7 * (var_qd_fp4s_dn7 * ddt_scale));
        let eq210_e2631_d_n8: f64 = (p.p7 * (var_qd_fp4s_dn8 * ddt_scale));
        let eq210_e2631_d_n9: f64 = (p.p7 * (var_qd_fp4s_dn9 * ddt_scale));
        let eq210_e2631_d_n12: f64 = (p.p7 * (var_qd_fp4s_dn12 * ddt_scale));
        let eq210_e2631_d_n14: f64 = (p.p7 * (var_qd_fp4s_dn14 * ddt_scale));
        let eq210_e2631_d_n15: f64 = (p.p7 * (var_qd_fp4s_dn15 * ddt_scale));
        let eq210_e2631_d_n16: f64 = (p.p7 * (var_qd_fp4s_dn16 * ddt_scale));
        let eq210_e2631_d_n17: f64 = (p.p7 * (var_qd_fp4s_dn17 * ddt_scale));
        let eq210_e2631_d_n18: f64 = (p.p7 * (var_qd_fp4s_dn18 * ddt_scale));
        let eq210_e2631_d_n19: f64 = (p.p7 * (var_qd_fp4s_dn19 * ddt_scale));
        let eq210_e2631_d_n20: f64 = (p.p7 * (var_qd_fp4s_dn20 * ddt_scale));
        let eq210_e2631_d_n21: f64 = (p.p7 * (var_qd_fp4s_dn21 * ddt_scale));
        let eq210_e2631_d_n22: f64 = (p.p7 * (var_qd_fp4s_dn22 * ddt_scale));
        (eq210_e2631, eq210_e2631_d_n0, eq210_e2631_d_n1, eq210_e2631_d_n2, eq210_e2631_d_n3, eq210_e2631_d_n4, eq210_e2631_d_n5, eq210_e2631_d_n6, eq210_e2631_d_n7, eq210_e2631_d_n8, eq210_e2631_d_n9, eq210_e2631_d_n12, eq210_e2631_d_n14, eq210_e2631_d_n15, eq210_e2631_d_n16, eq210_e2631_d_n17, eq210_e2631_d_n18, eq210_e2631_d_n19, eq210_e2631_d_n20, eq210_e2631_d_n21, eq210_e2631_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq210_value: f64 = eq210_e2633;
        let eq210_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq210_node_derivatives: [f64; 20] = [eq210_e2633_d_n0, eq210_e2633_d_n1, eq210_e2633_d_n2, eq210_e2633_d_n3, eq210_e2633_d_n4, eq210_e2633_d_n5, eq210_e2633_d_n6, eq210_e2633_d_n7, eq210_e2633_d_n8, eq210_e2633_d_n9, eq210_e2633_d_n12, eq210_e2633_d_n14, eq210_e2633_d_n15, eq210_e2633_d_n16, eq210_e2633_d_n17, eq210_e2633_d_n18, eq210_e2633_d_n19, eq210_e2633_d_n20, eq210_e2633_d_n21, eq210_e2633_d_n22];
        let eq210_branch_derivative_indices: [usize; 0] = [];
        let eq210_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(2),
            multiplicity * (eq210_value),
            &eq210_node_derivative_indices,
            &eq210_node_derivatives,
            &eq210_branch_derivative_indices,
            &eq210_branch_derivatives,
            multiplicity,
        );
        let (eq211_e2645, eq211_e2645_d_n0, eq211_e2645_d_n1, eq211_e2645_d_n2, eq211_e2645_d_n3, eq211_e2645_d_n4, eq211_e2645_d_n5, eq211_e2645_d_n6, eq211_e2645_d_n7, eq211_e2645_d_n8, eq211_e2645_d_n9, eq211_e2645_d_n12, eq211_e2645_d_n14, eq211_e2645_d_n15, eq211_e2645_d_n16, eq211_e2645_d_n17, eq211_e2645_d_n18, eq211_e2645_d_n19, eq211_e2645_d_n20, eq211_e2645_d_n21, eq211_e2645_d_n22,) = {
    if (((var_guard571 == 0.0) && (var_guard574 != 0.0)) && (var_guard575 != 0.0)) {
        let eq211_e2642: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 110, var_qg_fp4s);
        let eq211_e2643: f64 = (p.p7 * eq211_e2642);
        let eq211_e2643_d_n0: f64 = (p.p7 * (var_qg_fp4s_dn0 * ddt_scale));
        let eq211_e2643_d_n1: f64 = (p.p7 * (var_qg_fp4s_dn1 * ddt_scale));
        let eq211_e2643_d_n2: f64 = (p.p7 * (var_qg_fp4s_dn2 * ddt_scale));
        let eq211_e2643_d_n3: f64 = (p.p7 * (var_qg_fp4s_dn3 * ddt_scale));
        let eq211_e2643_d_n4: f64 = (p.p7 * (var_qg_fp4s_dn4 * ddt_scale));
        let eq211_e2643_d_n5: f64 = (p.p7 * (var_qg_fp4s_dn5 * ddt_scale));
        let eq211_e2643_d_n6: f64 = (p.p7 * (var_qg_fp4s_dn6 * ddt_scale));
        let eq211_e2643_d_n7: f64 = (p.p7 * (var_qg_fp4s_dn7 * ddt_scale));
        let eq211_e2643_d_n8: f64 = (p.p7 * (var_qg_fp4s_dn8 * ddt_scale));
        let eq211_e2643_d_n9: f64 = (p.p7 * (var_qg_fp4s_dn9 * ddt_scale));
        let eq211_e2643_d_n12: f64 = (p.p7 * (var_qg_fp4s_dn12 * ddt_scale));
        let eq211_e2643_d_n14: f64 = (p.p7 * (var_qg_fp4s_dn14 * ddt_scale));
        let eq211_e2643_d_n15: f64 = (p.p7 * (var_qg_fp4s_dn15 * ddt_scale));
        let eq211_e2643_d_n16: f64 = (p.p7 * (var_qg_fp4s_dn16 * ddt_scale));
        let eq211_e2643_d_n17: f64 = (p.p7 * (var_qg_fp4s_dn17 * ddt_scale));
        let eq211_e2643_d_n18: f64 = (p.p7 * (var_qg_fp4s_dn18 * ddt_scale));
        let eq211_e2643_d_n19: f64 = (p.p7 * (var_qg_fp4s_dn19 * ddt_scale));
        let eq211_e2643_d_n20: f64 = (p.p7 * (var_qg_fp4s_dn20 * ddt_scale));
        let eq211_e2643_d_n21: f64 = (p.p7 * (var_qg_fp4s_dn21 * ddt_scale));
        let eq211_e2643_d_n22: f64 = (p.p7 * (var_qg_fp4s_dn22 * ddt_scale));
        (eq211_e2643, eq211_e2643_d_n0, eq211_e2643_d_n1, eq211_e2643_d_n2, eq211_e2643_d_n3, eq211_e2643_d_n4, eq211_e2643_d_n5, eq211_e2643_d_n6, eq211_e2643_d_n7, eq211_e2643_d_n8, eq211_e2643_d_n9, eq211_e2643_d_n12, eq211_e2643_d_n14, eq211_e2643_d_n15, eq211_e2643_d_n16, eq211_e2643_d_n17, eq211_e2643_d_n18, eq211_e2643_d_n19, eq211_e2643_d_n20, eq211_e2643_d_n21, eq211_e2643_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq211_value: f64 = eq211_e2645;
        let eq211_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq211_node_derivatives: [f64; 20] = [eq211_e2645_d_n0, eq211_e2645_d_n1, eq211_e2645_d_n2, eq211_e2645_d_n3, eq211_e2645_d_n4, eq211_e2645_d_n5, eq211_e2645_d_n6, eq211_e2645_d_n7, eq211_e2645_d_n8, eq211_e2645_d_n9, eq211_e2645_d_n12, eq211_e2645_d_n14, eq211_e2645_d_n15, eq211_e2645_d_n16, eq211_e2645_d_n17, eq211_e2645_d_n18, eq211_e2645_d_n19, eq211_e2645_d_n20, eq211_e2645_d_n21, eq211_e2645_d_n22];
        let eq211_branch_derivative_indices: [usize; 0] = [];
        let eq211_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(2),
            multiplicity * (eq211_value),
            &eq211_node_derivative_indices,
            &eq211_node_derivatives,
            &eq211_branch_derivative_indices,
            &eq211_branch_derivatives,
            multiplicity,
        );
        let (eq212_e2659, eq212_e2659_d_n0, eq212_e2659_d_n1, eq212_e2659_d_n2, eq212_e2659_d_n3, eq212_e2659_d_n4, eq212_e2659_d_n5, eq212_e2659_d_n6, eq212_e2659_d_n7, eq212_e2659_d_n8, eq212_e2659_d_n9, eq212_e2659_d_n12, eq212_e2659_d_n14, eq212_e2659_d_n15, eq212_e2659_d_n16, eq212_e2659_d_n17, eq212_e2659_d_n18, eq212_e2659_d_n19, eq212_e2659_d_n20, eq212_e2659_d_n21, eq212_e2659_d_n22,) = {
    if (((var_guard571 == 0.0) && (var_guard574 != 0.0)) && (var_guard575 != 0.0)) {
        let eq212_e2654: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 111, var_qg_fp4s);
        let eq212_e2655: f64 = (p.p7 * eq212_e2654);
        let eq212_e2655_d_n0: f64 = (p.p7 * (var_qg_fp4s_dn0 * ddt_scale));
        let eq212_e2655_d_n1: f64 = (p.p7 * (var_qg_fp4s_dn1 * ddt_scale));
        let eq212_e2655_d_n2: f64 = (p.p7 * (var_qg_fp4s_dn2 * ddt_scale));
        let eq212_e2655_d_n3: f64 = (p.p7 * (var_qg_fp4s_dn3 * ddt_scale));
        let eq212_e2655_d_n4: f64 = (p.p7 * (var_qg_fp4s_dn4 * ddt_scale));
        let eq212_e2655_d_n5: f64 = (p.p7 * (var_qg_fp4s_dn5 * ddt_scale));
        let eq212_e2655_d_n6: f64 = (p.p7 * (var_qg_fp4s_dn6 * ddt_scale));
        let eq212_e2655_d_n7: f64 = (p.p7 * (var_qg_fp4s_dn7 * ddt_scale));
        let eq212_e2655_d_n8: f64 = (p.p7 * (var_qg_fp4s_dn8 * ddt_scale));
        let eq212_e2655_d_n9: f64 = (p.p7 * (var_qg_fp4s_dn9 * ddt_scale));
        let eq212_e2655_d_n12: f64 = (p.p7 * (var_qg_fp4s_dn12 * ddt_scale));
        let eq212_e2655_d_n14: f64 = (p.p7 * (var_qg_fp4s_dn14 * ddt_scale));
        let eq212_e2655_d_n15: f64 = (p.p7 * (var_qg_fp4s_dn15 * ddt_scale));
        let eq212_e2655_d_n16: f64 = (p.p7 * (var_qg_fp4s_dn16 * ddt_scale));
        let eq212_e2655_d_n17: f64 = (p.p7 * (var_qg_fp4s_dn17 * ddt_scale));
        let eq212_e2655_d_n18: f64 = (p.p7 * (var_qg_fp4s_dn18 * ddt_scale));
        let eq212_e2655_d_n19: f64 = (p.p7 * (var_qg_fp4s_dn19 * ddt_scale));
        let eq212_e2655_d_n20: f64 = (p.p7 * (var_qg_fp4s_dn20 * ddt_scale));
        let eq212_e2655_d_n21: f64 = (p.p7 * (var_qg_fp4s_dn21 * ddt_scale));
        let eq212_e2655_d_n22: f64 = (p.p7 * (var_qg_fp4s_dn22 * ddt_scale));
        let eq212_e2657: f64 = (eq212_e2655 * p.p249);
        let eq212_e2657_d_n0: f64 = (eq212_e2655_d_n0 * p.p249);
        let eq212_e2657_d_n1: f64 = (eq212_e2655_d_n1 * p.p249);
        let eq212_e2657_d_n2: f64 = (eq212_e2655_d_n2 * p.p249);
        let eq212_e2657_d_n3: f64 = (eq212_e2655_d_n3 * p.p249);
        let eq212_e2657_d_n4: f64 = (eq212_e2655_d_n4 * p.p249);
        let eq212_e2657_d_n5: f64 = (eq212_e2655_d_n5 * p.p249);
        let eq212_e2657_d_n6: f64 = (eq212_e2655_d_n6 * p.p249);
        let eq212_e2657_d_n7: f64 = (eq212_e2655_d_n7 * p.p249);
        let eq212_e2657_d_n8: f64 = (eq212_e2655_d_n8 * p.p249);
        let eq212_e2657_d_n9: f64 = (eq212_e2655_d_n9 * p.p249);
        let eq212_e2657_d_n12: f64 = (eq212_e2655_d_n12 * p.p249);
        let eq212_e2657_d_n14: f64 = (eq212_e2655_d_n14 * p.p249);
        let eq212_e2657_d_n15: f64 = (eq212_e2655_d_n15 * p.p249);
        let eq212_e2657_d_n16: f64 = (eq212_e2655_d_n16 * p.p249);
        let eq212_e2657_d_n17: f64 = (eq212_e2655_d_n17 * p.p249);
        let eq212_e2657_d_n18: f64 = (eq212_e2655_d_n18 * p.p249);
        let eq212_e2657_d_n19: f64 = (eq212_e2655_d_n19 * p.p249);
        let eq212_e2657_d_n20: f64 = (eq212_e2655_d_n20 * p.p249);
        let eq212_e2657_d_n21: f64 = (eq212_e2655_d_n21 * p.p249);
        let eq212_e2657_d_n22: f64 = (eq212_e2655_d_n22 * p.p249);
        (eq212_e2657, eq212_e2657_d_n0, eq212_e2657_d_n1, eq212_e2657_d_n2, eq212_e2657_d_n3, eq212_e2657_d_n4, eq212_e2657_d_n5, eq212_e2657_d_n6, eq212_e2657_d_n7, eq212_e2657_d_n8, eq212_e2657_d_n9, eq212_e2657_d_n12, eq212_e2657_d_n14, eq212_e2657_d_n15, eq212_e2657_d_n16, eq212_e2657_d_n17, eq212_e2657_d_n18, eq212_e2657_d_n19, eq212_e2657_d_n20, eq212_e2657_d_n21, eq212_e2657_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq212_value: f64 = eq212_e2659;
        let eq212_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq212_node_derivatives: [f64; 20] = [eq212_e2659_d_n0, eq212_e2659_d_n1, eq212_e2659_d_n2, eq212_e2659_d_n3, eq212_e2659_d_n4, eq212_e2659_d_n5, eq212_e2659_d_n6, eq212_e2659_d_n7, eq212_e2659_d_n8, eq212_e2659_d_n9, eq212_e2659_d_n12, eq212_e2659_d_n14, eq212_e2659_d_n15, eq212_e2659_d_n16, eq212_e2659_d_n17, eq212_e2659_d_n18, eq212_e2659_d_n19, eq212_e2659_d_n20, eq212_e2659_d_n21, eq212_e2659_d_n22];
        let eq212_branch_derivative_indices: [usize; 0] = [];
        let eq212_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq212_value),
            &eq212_node_derivative_indices,
            &eq212_node_derivatives,
            &eq212_branch_derivative_indices,
            &eq212_branch_derivatives,
            multiplicity,
        );
        let (eq213_e2672, eq213_e2672_d_n0, eq213_e2672_d_n1, eq213_e2672_d_n2, eq213_e2672_d_n3, eq213_e2672_d_n4, eq213_e2672_d_n5, eq213_e2672_d_n6, eq213_e2672_d_n7, eq213_e2672_d_n8, eq213_e2672_d_n9, eq213_e2672_d_n12, eq213_e2672_d_n14, eq213_e2672_d_n15, eq213_e2672_d_n16, eq213_e2672_d_n17, eq213_e2672_d_n18, eq213_e2672_d_n19, eq213_e2672_d_n20, eq213_e2672_d_n21, eq213_e2672_d_n22,) = {
    if (((var_guard571 == 0.0) && (var_guard574 != 0.0)) && (var_guard575 == 0.0)) {
        let eq213_e2669: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 112, var_qg_fp4s);
        let eq213_e2670: f64 = (p.p7 * eq213_e2669);
        let eq213_e2670_d_n0: f64 = (p.p7 * (var_qg_fp4s_dn0 * ddt_scale));
        let eq213_e2670_d_n1: f64 = (p.p7 * (var_qg_fp4s_dn1 * ddt_scale));
        let eq213_e2670_d_n2: f64 = (p.p7 * (var_qg_fp4s_dn2 * ddt_scale));
        let eq213_e2670_d_n3: f64 = (p.p7 * (var_qg_fp4s_dn3 * ddt_scale));
        let eq213_e2670_d_n4: f64 = (p.p7 * (var_qg_fp4s_dn4 * ddt_scale));
        let eq213_e2670_d_n5: f64 = (p.p7 * (var_qg_fp4s_dn5 * ddt_scale));
        let eq213_e2670_d_n6: f64 = (p.p7 * (var_qg_fp4s_dn6 * ddt_scale));
        let eq213_e2670_d_n7: f64 = (p.p7 * (var_qg_fp4s_dn7 * ddt_scale));
        let eq213_e2670_d_n8: f64 = (p.p7 * (var_qg_fp4s_dn8 * ddt_scale));
        let eq213_e2670_d_n9: f64 = (p.p7 * (var_qg_fp4s_dn9 * ddt_scale));
        let eq213_e2670_d_n12: f64 = (p.p7 * (var_qg_fp4s_dn12 * ddt_scale));
        let eq213_e2670_d_n14: f64 = (p.p7 * (var_qg_fp4s_dn14 * ddt_scale));
        let eq213_e2670_d_n15: f64 = (p.p7 * (var_qg_fp4s_dn15 * ddt_scale));
        let eq213_e2670_d_n16: f64 = (p.p7 * (var_qg_fp4s_dn16 * ddt_scale));
        let eq213_e2670_d_n17: f64 = (p.p7 * (var_qg_fp4s_dn17 * ddt_scale));
        let eq213_e2670_d_n18: f64 = (p.p7 * (var_qg_fp4s_dn18 * ddt_scale));
        let eq213_e2670_d_n19: f64 = (p.p7 * (var_qg_fp4s_dn19 * ddt_scale));
        let eq213_e2670_d_n20: f64 = (p.p7 * (var_qg_fp4s_dn20 * ddt_scale));
        let eq213_e2670_d_n21: f64 = (p.p7 * (var_qg_fp4s_dn21 * ddt_scale));
        let eq213_e2670_d_n22: f64 = (p.p7 * (var_qg_fp4s_dn22 * ddt_scale));
        (eq213_e2670, eq213_e2670_d_n0, eq213_e2670_d_n1, eq213_e2670_d_n2, eq213_e2670_d_n3, eq213_e2670_d_n4, eq213_e2670_d_n5, eq213_e2670_d_n6, eq213_e2670_d_n7, eq213_e2670_d_n8, eq213_e2670_d_n9, eq213_e2670_d_n12, eq213_e2670_d_n14, eq213_e2670_d_n15, eq213_e2670_d_n16, eq213_e2670_d_n17, eq213_e2670_d_n18, eq213_e2670_d_n19, eq213_e2670_d_n20, eq213_e2670_d_n21, eq213_e2670_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq213_value: f64 = eq213_e2672;
        let eq213_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq213_node_derivatives: [f64; 20] = [eq213_e2672_d_n0, eq213_e2672_d_n1, eq213_e2672_d_n2, eq213_e2672_d_n3, eq213_e2672_d_n4, eq213_e2672_d_n5, eq213_e2672_d_n6, eq213_e2672_d_n7, eq213_e2672_d_n8, eq213_e2672_d_n9, eq213_e2672_d_n12, eq213_e2672_d_n14, eq213_e2672_d_n15, eq213_e2672_d_n16, eq213_e2672_d_n17, eq213_e2672_d_n18, eq213_e2672_d_n19, eq213_e2672_d_n20, eq213_e2672_d_n21, eq213_e2672_d_n22];
        let eq213_branch_derivative_indices: [usize; 0] = [];
        let eq213_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq213_value),
            &eq213_node_derivative_indices,
            &eq213_node_derivatives,
            &eq213_branch_derivative_indices,
            &eq213_branch_derivatives,
            multiplicity,
        );
        let (eq214_e2687, eq214_e2687_d_n0, eq214_e2687_d_n1, eq214_e2687_d_n2, eq214_e2687_d_n3, eq214_e2687_d_n4, eq214_e2687_d_n5, eq214_e2687_d_n6, eq214_e2687_d_n7, eq214_e2687_d_n8, eq214_e2687_d_n9, eq214_e2687_d_n12, eq214_e2687_d_n14, eq214_e2687_d_n15, eq214_e2687_d_n16, eq214_e2687_d_n17, eq214_e2687_d_n18, eq214_e2687_d_n19, eq214_e2687_d_n20, eq214_e2687_d_n21, eq214_e2687_d_n22,) = {
    if (((var_guard571 == 0.0) && (var_guard574 != 0.0)) && (var_guard575 == 0.0)) {
        let eq214_e2682: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 113, var_qg_fp4s);
        let eq214_e2683: f64 = (p.p7 * eq214_e2682);
        let eq214_e2683_d_n0: f64 = (p.p7 * (var_qg_fp4s_dn0 * ddt_scale));
        let eq214_e2683_d_n1: f64 = (p.p7 * (var_qg_fp4s_dn1 * ddt_scale));
        let eq214_e2683_d_n2: f64 = (p.p7 * (var_qg_fp4s_dn2 * ddt_scale));
        let eq214_e2683_d_n3: f64 = (p.p7 * (var_qg_fp4s_dn3 * ddt_scale));
        let eq214_e2683_d_n4: f64 = (p.p7 * (var_qg_fp4s_dn4 * ddt_scale));
        let eq214_e2683_d_n5: f64 = (p.p7 * (var_qg_fp4s_dn5 * ddt_scale));
        let eq214_e2683_d_n6: f64 = (p.p7 * (var_qg_fp4s_dn6 * ddt_scale));
        let eq214_e2683_d_n7: f64 = (p.p7 * (var_qg_fp4s_dn7 * ddt_scale));
        let eq214_e2683_d_n8: f64 = (p.p7 * (var_qg_fp4s_dn8 * ddt_scale));
        let eq214_e2683_d_n9: f64 = (p.p7 * (var_qg_fp4s_dn9 * ddt_scale));
        let eq214_e2683_d_n12: f64 = (p.p7 * (var_qg_fp4s_dn12 * ddt_scale));
        let eq214_e2683_d_n14: f64 = (p.p7 * (var_qg_fp4s_dn14 * ddt_scale));
        let eq214_e2683_d_n15: f64 = (p.p7 * (var_qg_fp4s_dn15 * ddt_scale));
        let eq214_e2683_d_n16: f64 = (p.p7 * (var_qg_fp4s_dn16 * ddt_scale));
        let eq214_e2683_d_n17: f64 = (p.p7 * (var_qg_fp4s_dn17 * ddt_scale));
        let eq214_e2683_d_n18: f64 = (p.p7 * (var_qg_fp4s_dn18 * ddt_scale));
        let eq214_e2683_d_n19: f64 = (p.p7 * (var_qg_fp4s_dn19 * ddt_scale));
        let eq214_e2683_d_n20: f64 = (p.p7 * (var_qg_fp4s_dn20 * ddt_scale));
        let eq214_e2683_d_n21: f64 = (p.p7 * (var_qg_fp4s_dn21 * ddt_scale));
        let eq214_e2683_d_n22: f64 = (p.p7 * (var_qg_fp4s_dn22 * ddt_scale));
        let eq214_e2685: f64 = (eq214_e2683 * p.p249);
        let eq214_e2685_d_n0: f64 = (eq214_e2683_d_n0 * p.p249);
        let eq214_e2685_d_n1: f64 = (eq214_e2683_d_n1 * p.p249);
        let eq214_e2685_d_n2: f64 = (eq214_e2683_d_n2 * p.p249);
        let eq214_e2685_d_n3: f64 = (eq214_e2683_d_n3 * p.p249);
        let eq214_e2685_d_n4: f64 = (eq214_e2683_d_n4 * p.p249);
        let eq214_e2685_d_n5: f64 = (eq214_e2683_d_n5 * p.p249);
        let eq214_e2685_d_n6: f64 = (eq214_e2683_d_n6 * p.p249);
        let eq214_e2685_d_n7: f64 = (eq214_e2683_d_n7 * p.p249);
        let eq214_e2685_d_n8: f64 = (eq214_e2683_d_n8 * p.p249);
        let eq214_e2685_d_n9: f64 = (eq214_e2683_d_n9 * p.p249);
        let eq214_e2685_d_n12: f64 = (eq214_e2683_d_n12 * p.p249);
        let eq214_e2685_d_n14: f64 = (eq214_e2683_d_n14 * p.p249);
        let eq214_e2685_d_n15: f64 = (eq214_e2683_d_n15 * p.p249);
        let eq214_e2685_d_n16: f64 = (eq214_e2683_d_n16 * p.p249);
        let eq214_e2685_d_n17: f64 = (eq214_e2683_d_n17 * p.p249);
        let eq214_e2685_d_n18: f64 = (eq214_e2683_d_n18 * p.p249);
        let eq214_e2685_d_n19: f64 = (eq214_e2683_d_n19 * p.p249);
        let eq214_e2685_d_n20: f64 = (eq214_e2683_d_n20 * p.p249);
        let eq214_e2685_d_n21: f64 = (eq214_e2683_d_n21 * p.p249);
        let eq214_e2685_d_n22: f64 = (eq214_e2683_d_n22 * p.p249);
        (eq214_e2685, eq214_e2685_d_n0, eq214_e2685_d_n1, eq214_e2685_d_n2, eq214_e2685_d_n3, eq214_e2685_d_n4, eq214_e2685_d_n5, eq214_e2685_d_n6, eq214_e2685_d_n7, eq214_e2685_d_n8, eq214_e2685_d_n9, eq214_e2685_d_n12, eq214_e2685_d_n14, eq214_e2685_d_n15, eq214_e2685_d_n16, eq214_e2685_d_n17, eq214_e2685_d_n18, eq214_e2685_d_n19, eq214_e2685_d_n20, eq214_e2685_d_n21, eq214_e2685_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq214_value: f64 = eq214_e2687;
        let eq214_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq214_node_derivatives: [f64; 20] = [eq214_e2687_d_n0, eq214_e2687_d_n1, eq214_e2687_d_n2, eq214_e2687_d_n3, eq214_e2687_d_n4, eq214_e2687_d_n5, eq214_e2687_d_n6, eq214_e2687_d_n7, eq214_e2687_d_n8, eq214_e2687_d_n9, eq214_e2687_d_n12, eq214_e2687_d_n14, eq214_e2687_d_n15, eq214_e2687_d_n16, eq214_e2687_d_n17, eq214_e2687_d_n18, eq214_e2687_d_n19, eq214_e2687_d_n20, eq214_e2687_d_n21, eq214_e2687_d_n22];
        let eq214_branch_derivative_indices: [usize; 0] = [];
        let eq214_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq214_value),
            &eq214_node_derivative_indices,
            &eq214_node_derivatives,
            &eq214_branch_derivative_indices,
            &eq214_branch_derivatives,
            multiplicity,
        );
        let (eq215_e2699, eq215_e2699_d_n0, eq215_e2699_d_n1, eq215_e2699_d_n2, eq215_e2699_d_n3, eq215_e2699_d_n4, eq215_e2699_d_n5, eq215_e2699_d_n6, eq215_e2699_d_n7, eq215_e2699_d_n8, eq215_e2699_d_n9, eq215_e2699_d_n12, eq215_e2699_d_n14, eq215_e2699_d_n15, eq215_e2699_d_n16, eq215_e2699_d_n17, eq215_e2699_d_n18, eq215_e2699_d_n19, eq215_e2699_d_n20, eq215_e2699_d_n21, eq215_e2699_d_n22,) = {
    if ((var_guard571 == 0.0) && (var_guard574 != 0.0)) {
        let eq215_e2695: f64 = (p.p254 * var_qg_fp4s);
        let eq215_e2695_d_n0: f64 = (p.p254 * var_qg_fp4s_dn0);
        let eq215_e2695_d_n1: f64 = (p.p254 * var_qg_fp4s_dn1);
        let eq215_e2695_d_n2: f64 = (p.p254 * var_qg_fp4s_dn2);
        let eq215_e2695_d_n3: f64 = (p.p254 * var_qg_fp4s_dn3);
        let eq215_e2695_d_n4: f64 = (p.p254 * var_qg_fp4s_dn4);
        let eq215_e2695_d_n5: f64 = (p.p254 * var_qg_fp4s_dn5);
        let eq215_e2695_d_n6: f64 = (p.p254 * var_qg_fp4s_dn6);
        let eq215_e2695_d_n7: f64 = (p.p254 * var_qg_fp4s_dn7);
        let eq215_e2695_d_n8: f64 = (p.p254 * var_qg_fp4s_dn8);
        let eq215_e2695_d_n9: f64 = (p.p254 * var_qg_fp4s_dn9);
        let eq215_e2695_d_n12: f64 = (p.p254 * var_qg_fp4s_dn12);
        let eq215_e2695_d_n14: f64 = (p.p254 * var_qg_fp4s_dn14);
        let eq215_e2695_d_n15: f64 = (p.p254 * var_qg_fp4s_dn15);
        let eq215_e2695_d_n16: f64 = (p.p254 * var_qg_fp4s_dn16);
        let eq215_e2695_d_n17: f64 = (p.p254 * var_qg_fp4s_dn17);
        let eq215_e2695_d_n18: f64 = (p.p254 * var_qg_fp4s_dn18);
        let eq215_e2695_d_n19: f64 = (p.p254 * var_qg_fp4s_dn19);
        let eq215_e2695_d_n20: f64 = (p.p254 * var_qg_fp4s_dn20);
        let eq215_e2695_d_n21: f64 = (p.p254 * var_qg_fp4s_dn21);
        let eq215_e2695_d_n22: f64 = (p.p254 * var_qg_fp4s_dn22);
        let eq215_e2696: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 114, eq215_e2695);
        let eq215_e2697: f64 = (p.p7 * eq215_e2696);
        let eq215_e2697_d_n0: f64 = (p.p7 * (eq215_e2695_d_n0 * ddt_scale));
        let eq215_e2697_d_n1: f64 = (p.p7 * (eq215_e2695_d_n1 * ddt_scale));
        let eq215_e2697_d_n2: f64 = (p.p7 * (eq215_e2695_d_n2 * ddt_scale));
        let eq215_e2697_d_n3: f64 = (p.p7 * (eq215_e2695_d_n3 * ddt_scale));
        let eq215_e2697_d_n4: f64 = (p.p7 * (eq215_e2695_d_n4 * ddt_scale));
        let eq215_e2697_d_n5: f64 = (p.p7 * (eq215_e2695_d_n5 * ddt_scale));
        let eq215_e2697_d_n6: f64 = (p.p7 * (eq215_e2695_d_n6 * ddt_scale));
        let eq215_e2697_d_n7: f64 = (p.p7 * (eq215_e2695_d_n7 * ddt_scale));
        let eq215_e2697_d_n8: f64 = (p.p7 * (eq215_e2695_d_n8 * ddt_scale));
        let eq215_e2697_d_n9: f64 = (p.p7 * (eq215_e2695_d_n9 * ddt_scale));
        let eq215_e2697_d_n12: f64 = (p.p7 * (eq215_e2695_d_n12 * ddt_scale));
        let eq215_e2697_d_n14: f64 = (p.p7 * (eq215_e2695_d_n14 * ddt_scale));
        let eq215_e2697_d_n15: f64 = (p.p7 * (eq215_e2695_d_n15 * ddt_scale));
        let eq215_e2697_d_n16: f64 = (p.p7 * (eq215_e2695_d_n16 * ddt_scale));
        let eq215_e2697_d_n17: f64 = (p.p7 * (eq215_e2695_d_n17 * ddt_scale));
        let eq215_e2697_d_n18: f64 = (p.p7 * (eq215_e2695_d_n18 * ddt_scale));
        let eq215_e2697_d_n19: f64 = (p.p7 * (eq215_e2695_d_n19 * ddt_scale));
        let eq215_e2697_d_n20: f64 = (p.p7 * (eq215_e2695_d_n20 * ddt_scale));
        let eq215_e2697_d_n21: f64 = (p.p7 * (eq215_e2695_d_n21 * ddt_scale));
        let eq215_e2697_d_n22: f64 = (p.p7 * (eq215_e2695_d_n22 * ddt_scale));
        (eq215_e2697, eq215_e2697_d_n0, eq215_e2697_d_n1, eq215_e2697_d_n2, eq215_e2697_d_n3, eq215_e2697_d_n4, eq215_e2697_d_n5, eq215_e2697_d_n6, eq215_e2697_d_n7, eq215_e2697_d_n8, eq215_e2697_d_n9, eq215_e2697_d_n12, eq215_e2697_d_n14, eq215_e2697_d_n15, eq215_e2697_d_n16, eq215_e2697_d_n17, eq215_e2697_d_n18, eq215_e2697_d_n19, eq215_e2697_d_n20, eq215_e2697_d_n21, eq215_e2697_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq215_value: f64 = eq215_e2699;
        let eq215_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq215_node_derivatives: [f64; 20] = [eq215_e2699_d_n0, eq215_e2699_d_n1, eq215_e2699_d_n2, eq215_e2699_d_n3, eq215_e2699_d_n4, eq215_e2699_d_n5, eq215_e2699_d_n6, eq215_e2699_d_n7, eq215_e2699_d_n8, eq215_e2699_d_n9, eq215_e2699_d_n12, eq215_e2699_d_n14, eq215_e2699_d_n15, eq215_e2699_d_n16, eq215_e2699_d_n17, eq215_e2699_d_n18, eq215_e2699_d_n19, eq215_e2699_d_n20, eq215_e2699_d_n21, eq215_e2699_d_n22];
        let eq215_branch_derivative_indices: [usize; 0] = [];
        let eq215_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(8),
            multiplicity * (eq215_value),
            &eq215_node_derivative_indices,
            &eq215_node_derivatives,
            &eq215_branch_derivative_indices,
            &eq215_branch_derivatives,
            multiplicity,
        );
        let eq216_e2702: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 115, var_qfr);
        let eq216_e2703: f64 = (p.p7 * eq216_e2702);
        let eq216_e2703_d_n0: f64 = (p.p7 * (var_qfr_dn0 * ddt_scale));
        let eq216_e2703_d_n2: f64 = (p.p7 * (var_qfr_dn2 * ddt_scale));
        let eq216_e2703_d_n4: f64 = (p.p7 * (var_qfr_dn4 * ddt_scale));
        let eq216_value: f64 = eq216_e2703;
        stamper.stamp_current_node3_local(
            Some(1),
            Some(0),
            multiplicity * (eq216_value),
            0,
            multiplicity * (eq216_e2703_d_n0),
            2,
            multiplicity * (eq216_e2703_d_n2),
            4,
            multiplicity * (eq216_e2703_d_n4),
        );
        let eq217_e2707: f64 = (p.p4 * p.p5);
        let eq217_e2709: f64 = (eq217_e2707 * p.p220);
        let eq217_e2711: f64 = (eq217_e2709 * (nv1 - nv2));
        let eq217_e2712: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 116, eq217_e2711);
        let eq217_e2713: f64 = (p.p7 * eq217_e2712);
        let eq217_e2713_d_n1: f64 = (p.p7 * (eq217_e2709 * ddt_scale));
        let eq217_e2713_d_n2: f64 = (p.p7 * ((-eq217_e2709) * ddt_scale));
        let eq217_value: f64 = eq217_e2713;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (eq217_value),
            1,
            multiplicity * (eq217_e2713_d_n1),
            2,
            multiplicity * (eq217_e2713_d_n2),
        );
        let eq218_e2716: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 117, var_qfr2);
        let eq218_e2717: f64 = (p.p7 * eq218_e2716);
        let eq218_e2717_d_n0: f64 = (p.p7 * (var_qfr2_dn0 * ddt_scale));
        let eq218_e2717_d_n1: f64 = (p.p7 * (var_qfr2_dn1 * ddt_scale));
        let eq218_e2717_d_n2: f64 = (p.p7 * (var_qfr2_dn2 * ddt_scale));
        let eq218_e2717_d_n3: f64 = (p.p7 * (var_qfr2_dn3 * ddt_scale));
        let eq218_e2717_d_n4: f64 = (p.p7 * (var_qfr2_dn4 * ddt_scale));
        let eq218_e2717_d_n5: f64 = (p.p7 * (var_qfr2_dn5 * ddt_scale));
        let eq218_e2717_d_n6: f64 = (p.p7 * (var_qfr2_dn6 * ddt_scale));
        let eq218_e2717_d_n7: f64 = (p.p7 * (var_qfr2_dn7 * ddt_scale));
        let eq218_e2717_d_n8: f64 = (p.p7 * (var_qfr2_dn8 * ddt_scale));
        let eq218_e2717_d_n9: f64 = (p.p7 * (var_qfr2_dn9 * ddt_scale));
        let eq218_e2717_d_n12: f64 = (p.p7 * (var_qfr2_dn12 * ddt_scale));
        let eq218_e2717_d_n14: f64 = (p.p7 * (var_qfr2_dn14 * ddt_scale));
        let eq218_e2717_d_n15: f64 = (p.p7 * (var_qfr2_dn15 * ddt_scale));
        let eq218_e2717_d_n16: f64 = (p.p7 * (var_qfr2_dn16 * ddt_scale));
        let eq218_e2717_d_n17: f64 = (p.p7 * (var_qfr2_dn17 * ddt_scale));
        let eq218_e2717_d_n18: f64 = (p.p7 * (var_qfr2_dn18 * ddt_scale));
        let eq218_e2717_d_n19: f64 = (p.p7 * (var_qfr2_dn19 * ddt_scale));
        let eq218_e2717_d_n20: f64 = (p.p7 * (var_qfr2_dn20 * ddt_scale));
        let eq218_e2717_d_n21: f64 = (p.p7 * (var_qfr2_dn21 * ddt_scale));
        let eq218_e2717_d_n22: f64 = (p.p7 * (var_qfr2_dn22 * ddt_scale));
        let eq218_value: f64 = eq218_e2717;
        let eq218_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq218_node_derivatives: [f64; 20] = [eq218_e2717_d_n0, eq218_e2717_d_n1, eq218_e2717_d_n2, eq218_e2717_d_n3, eq218_e2717_d_n4, eq218_e2717_d_n5, eq218_e2717_d_n6, eq218_e2717_d_n7, eq218_e2717_d_n8, eq218_e2717_d_n9, eq218_e2717_d_n12, eq218_e2717_d_n14, eq218_e2717_d_n15, eq218_e2717_d_n16, eq218_e2717_d_n17, eq218_e2717_d_n18, eq218_e2717_d_n19, eq218_e2717_d_n20, eq218_e2717_d_n21, eq218_e2717_d_n22];
        let eq218_branch_derivative_indices: [usize; 0] = [];
        let eq218_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(2),
            multiplicity * (eq218_value),
            &eq218_node_derivative_indices,
            &eq218_node_derivatives,
            &eq218_branch_derivative_indices,
            &eq218_branch_derivatives,
            multiplicity,
        );
        let eq219_e2720: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 118, var_qfr3);
        let eq219_e2721: f64 = (p.p7 * eq219_e2720);
        let eq219_e2721_d_n0: f64 = (p.p7 * (var_qfr3_dn0 * ddt_scale));
        let eq219_e2721_d_n2: f64 = (p.p7 * (var_qfr3_dn2 * ddt_scale));
        let eq219_value: f64 = eq219_e2721;
        stamper.stamp_current_node2_local(
            Some(2),
            Some(0),
            multiplicity * (eq219_value),
            0,
            multiplicity * (eq219_e2721_d_n0),
            2,
            multiplicity * (eq219_e2721_d_n2),
        );
    }

    pub(super) fn stamp_transient_equations_block_14(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
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
        var_guard576: f64,
        var_ids: f64,
        var_ids_dn0: f64,
        var_ids_dn1: f64,
        var_ids_dn12: f64,
        var_ids_dn14: f64,
        var_ids_dn15: f64,
        var_ids_dn16: f64,
        var_ids_dn17: f64,
        var_ids_dn18: f64,
        var_ids_dn19: f64,
        var_ids_dn2: f64,
        var_ids_dn20: f64,
        var_ids_dn21: f64,
        var_ids_dn22: f64,
        var_ids_dn3: f64,
        var_ids_dn4: f64,
        var_ids_dn5: f64,
        var_ids_dn6: f64,
        var_ids_dn7: f64,
        var_ids_dn8: f64,
        var_ids_dn9: f64,
        var_ids_fp1: f64,
        var_ids_fp1_dn0: f64,
        var_ids_fp1_dn1: f64,
        var_ids_fp1_dn12: f64,
        var_ids_fp1_dn14: f64,
        var_ids_fp1_dn15: f64,
        var_ids_fp1_dn16: f64,
        var_ids_fp1_dn17: f64,
        var_ids_fp1_dn18: f64,
        var_ids_fp1_dn19: f64,
        var_ids_fp1_dn2: f64,
        var_ids_fp1_dn20: f64,
        var_ids_fp1_dn21: f64,
        var_ids_fp1_dn22: f64,
        var_ids_fp1_dn3: f64,
        var_ids_fp1_dn4: f64,
        var_ids_fp1_dn5: f64,
        var_ids_fp1_dn6: f64,
        var_ids_fp1_dn7: f64,
        var_ids_fp1_dn8: f64,
        var_ids_fp1_dn9: f64,
        var_ids_fp2: f64,
        var_ids_fp2_dn0: f64,
        var_ids_fp2_dn1: f64,
        var_ids_fp2_dn12: f64,
        var_ids_fp2_dn14: f64,
        var_ids_fp2_dn15: f64,
        var_ids_fp2_dn16: f64,
        var_ids_fp2_dn17: f64,
        var_ids_fp2_dn18: f64,
        var_ids_fp2_dn19: f64,
        var_ids_fp2_dn2: f64,
        var_ids_fp2_dn20: f64,
        var_ids_fp2_dn21: f64,
        var_ids_fp2_dn22: f64,
        var_ids_fp2_dn3: f64,
        var_ids_fp2_dn4: f64,
        var_ids_fp2_dn5: f64,
        var_ids_fp2_dn6: f64,
        var_ids_fp2_dn7: f64,
        var_ids_fp2_dn8: f64,
        var_ids_fp2_dn9: f64,
        var_ids_fp3: f64,
        var_ids_fp3_dn0: f64,
        var_ids_fp3_dn1: f64,
        var_ids_fp3_dn12: f64,
        var_ids_fp3_dn14: f64,
        var_ids_fp3_dn15: f64,
        var_ids_fp3_dn16: f64,
        var_ids_fp3_dn17: f64,
        var_ids_fp3_dn18: f64,
        var_ids_fp3_dn19: f64,
        var_ids_fp3_dn2: f64,
        var_ids_fp3_dn20: f64,
        var_ids_fp3_dn21: f64,
        var_ids_fp3_dn22: f64,
        var_ids_fp3_dn3: f64,
        var_ids_fp3_dn4: f64,
        var_ids_fp3_dn5: f64,
        var_ids_fp3_dn6: f64,
        var_ids_fp3_dn7: f64,
        var_ids_fp3_dn8: f64,
        var_ids_fp3_dn9: f64,
        var_ids_fp4: f64,
        var_ids_fp4_dn0: f64,
        var_ids_fp4_dn1: f64,
        var_ids_fp4_dn12: f64,
        var_ids_fp4_dn14: f64,
        var_ids_fp4_dn15: f64,
        var_ids_fp4_dn16: f64,
        var_ids_fp4_dn17: f64,
        var_ids_fp4_dn18: f64,
        var_ids_fp4_dn19: f64,
        var_ids_fp4_dn2: f64,
        var_ids_fp4_dn20: f64,
        var_ids_fp4_dn21: f64,
        var_ids_fp4_dn22: f64,
        var_ids_fp4_dn3: f64,
        var_ids_fp4_dn4: f64,
        var_ids_fp4_dn5: f64,
        var_ids_fp4_dn6: f64,
        var_ids_fp4_dn7: f64,
        var_ids_fp4_dn8: f64,
        var_ids_fp4_dn9: f64,
        var_qdep: f64,
        var_qdep_dn0: f64,
        var_qdep_dn1: f64,
        var_qdep_dn12: f64,
        var_qdep_dn14: f64,
        var_qdep_dn15: f64,
        var_qdep_dn16: f64,
        var_qdep_dn17: f64,
        var_qdep_dn18: f64,
        var_qdep_dn19: f64,
        var_qdep_dn2: f64,
        var_qdep_dn20: f64,
        var_qdep_dn21: f64,
        var_qdep_dn22: f64,
        var_qdep_dn3: f64,
        var_qdep_dn4: f64,
        var_qdep_dn5: f64,
        var_qdep_dn6: f64,
        var_qdep_dn7: f64,
        var_qdep_dn8: f64,
        var_qdep_dn9: f64,
        var_vds: f64,
        var_vds_dn7: f64,
        var_vds_dn8: f64,
        var_vds_fp1: f64,
        var_vds_fp1_dn15: f64,
        var_vds_fp1_dn7: f64,
        var_vds_fp2: f64,
        var_vds_fp2_dn15: f64,
        var_vds_fp2_dn16: f64,
        var_vds_fp3: f64,
        var_vds_fp3_dn16: f64,
        var_vds_fp3_dn17: f64,
        var_vds_fp4: f64,
        var_vds_fp4_dn17: f64,
        var_vds_fp4_dn18: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq220_e2724: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 119, var_qdep);
        let eq220_e2725: f64 = (p.p7 * eq220_e2724);
        let eq220_e2725_d_n0: f64 = (p.p7 * (var_qdep_dn0 * ddt_scale));
        let eq220_e2725_d_n1: f64 = (p.p7 * (var_qdep_dn1 * ddt_scale));
        let eq220_e2725_d_n2: f64 = (p.p7 * (var_qdep_dn2 * ddt_scale));
        let eq220_e2725_d_n3: f64 = (p.p7 * (var_qdep_dn3 * ddt_scale));
        let eq220_e2725_d_n4: f64 = (p.p7 * (var_qdep_dn4 * ddt_scale));
        let eq220_e2725_d_n5: f64 = (p.p7 * (var_qdep_dn5 * ddt_scale));
        let eq220_e2725_d_n6: f64 = (p.p7 * (var_qdep_dn6 * ddt_scale));
        let eq220_e2725_d_n7: f64 = (p.p7 * (var_qdep_dn7 * ddt_scale));
        let eq220_e2725_d_n8: f64 = (p.p7 * (var_qdep_dn8 * ddt_scale));
        let eq220_e2725_d_n9: f64 = (p.p7 * (var_qdep_dn9 * ddt_scale));
        let eq220_e2725_d_n12: f64 = (p.p7 * (var_qdep_dn12 * ddt_scale));
        let eq220_e2725_d_n14: f64 = (p.p7 * (var_qdep_dn14 * ddt_scale));
        let eq220_e2725_d_n15: f64 = (p.p7 * (var_qdep_dn15 * ddt_scale));
        let eq220_e2725_d_n16: f64 = (p.p7 * (var_qdep_dn16 * ddt_scale));
        let eq220_e2725_d_n17: f64 = (p.p7 * (var_qdep_dn17 * ddt_scale));
        let eq220_e2725_d_n18: f64 = (p.p7 * (var_qdep_dn18 * ddt_scale));
        let eq220_e2725_d_n19: f64 = (p.p7 * (var_qdep_dn19 * ddt_scale));
        let eq220_e2725_d_n20: f64 = (p.p7 * (var_qdep_dn20 * ddt_scale));
        let eq220_e2725_d_n21: f64 = (p.p7 * (var_qdep_dn21 * ddt_scale));
        let eq220_e2725_d_n22: f64 = (p.p7 * (var_qdep_dn22 * ddt_scale));
        let eq220_value: f64 = eq220_e2725;
        let eq220_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq220_node_derivatives: [f64; 20] = [eq220_e2725_d_n0, eq220_e2725_d_n1, eq220_e2725_d_n2, eq220_e2725_d_n3, eq220_e2725_d_n4, eq220_e2725_d_n5, eq220_e2725_d_n6, eq220_e2725_d_n7, eq220_e2725_d_n8, eq220_e2725_d_n9, eq220_e2725_d_n12, eq220_e2725_d_n14, eq220_e2725_d_n15, eq220_e2725_d_n16, eq220_e2725_d_n17, eq220_e2725_d_n18, eq220_e2725_d_n19, eq220_e2725_d_n20, eq220_e2725_d_n21, eq220_e2725_d_n22];
        let eq220_branch_derivative_indices: [usize; 0] = [];
        let eq220_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(0),
            multiplicity * (eq220_value),
            &eq220_node_derivative_indices,
            &eq220_node_derivatives,
            &eq220_branch_derivative_indices,
            &eq220_branch_derivatives,
            multiplicity,
        );
        let (eq221_e2758, eq221_e2758_d_n0, eq221_e2758_d_n1, eq221_e2758_d_n2, eq221_e2758_d_n3, eq221_e2758_d_n4, eq221_e2758_d_n5, eq221_e2758_d_n6, eq221_e2758_d_n7, eq221_e2758_d_n8, eq221_e2758_d_n9, eq221_e2758_d_n12, eq221_e2758_d_n14, eq221_e2758_d_n15, eq221_e2758_d_n16, eq221_e2758_d_n17, eq221_e2758_d_n18, eq221_e2758_d_n19, eq221_e2758_d_n20, eq221_e2758_d_n21, eq221_e2758_d_n22,) = {
    if (var_guard576 != 0.0) {
        let eq221_e2728: f64 = (-1.0);
        let eq221_e2730: f64 = (eq221_e2728 * var_ids);
        let eq221_e2730_d_n0: f64 = (eq221_e2728 * var_ids_dn0);
        let eq221_e2730_d_n1: f64 = (eq221_e2728 * var_ids_dn1);
        let eq221_e2730_d_n2: f64 = (eq221_e2728 * var_ids_dn2);
        let eq221_e2730_d_n3: f64 = (eq221_e2728 * var_ids_dn3);
        let eq221_e2730_d_n4: f64 = (eq221_e2728 * var_ids_dn4);
        let eq221_e2730_d_n5: f64 = (eq221_e2728 * var_ids_dn5);
        let eq221_e2730_d_n6: f64 = (eq221_e2728 * var_ids_dn6);
        let eq221_e2730_d_n7: f64 = (eq221_e2728 * var_ids_dn7);
        let eq221_e2730_d_n8: f64 = (eq221_e2728 * var_ids_dn8);
        let eq221_e2730_d_n9: f64 = (eq221_e2728 * var_ids_dn9);
        let eq221_e2730_d_n12: f64 = (eq221_e2728 * var_ids_dn12);
        let eq221_e2730_d_n14: f64 = (eq221_e2728 * var_ids_dn14);
        let eq221_e2730_d_n15: f64 = (eq221_e2728 * var_ids_dn15);
        let eq221_e2730_d_n16: f64 = (eq221_e2728 * var_ids_dn16);
        let eq221_e2730_d_n17: f64 = (eq221_e2728 * var_ids_dn17);
        let eq221_e2730_d_n18: f64 = (eq221_e2728 * var_ids_dn18);
        let eq221_e2730_d_n19: f64 = (eq221_e2728 * var_ids_dn19);
        let eq221_e2730_d_n20: f64 = (eq221_e2728 * var_ids_dn20);
        let eq221_e2730_d_n21: f64 = (eq221_e2728 * var_ids_dn21);
        let eq221_e2730_d_n22: f64 = (eq221_e2728 * var_ids_dn22);
        let eq221_e2732: f64 = (eq221_e2730 * var_vds);
        let eq221_e2732_d_n0: f64 = (eq221_e2730_d_n0 * var_vds);
        let eq221_e2732_d_n1: f64 = (eq221_e2730_d_n1 * var_vds);
        let eq221_e2732_d_n2: f64 = (eq221_e2730_d_n2 * var_vds);
        let eq221_e2732_d_n3: f64 = (eq221_e2730_d_n3 * var_vds);
        let eq221_e2732_d_n4: f64 = (eq221_e2730_d_n4 * var_vds);
        let eq221_e2732_d_n5: f64 = (eq221_e2730_d_n5 * var_vds);
        let eq221_e2732_d_n6: f64 = (eq221_e2730_d_n6 * var_vds);
        let eq221_e2732_d_n7: f64 = ((eq221_e2730_d_n7 * var_vds) + (eq221_e2730 * var_vds_dn7));
        let eq221_e2732_d_n8: f64 = ((eq221_e2730_d_n8 * var_vds) + (eq221_e2730 * var_vds_dn8));
        let eq221_e2732_d_n9: f64 = (eq221_e2730_d_n9 * var_vds);
        let eq221_e2732_d_n12: f64 = (eq221_e2730_d_n12 * var_vds);
        let eq221_e2732_d_n14: f64 = (eq221_e2730_d_n14 * var_vds);
        let eq221_e2732_d_n15: f64 = (eq221_e2730_d_n15 * var_vds);
        let eq221_e2732_d_n16: f64 = (eq221_e2730_d_n16 * var_vds);
        let eq221_e2732_d_n17: f64 = (eq221_e2730_d_n17 * var_vds);
        let eq221_e2732_d_n18: f64 = (eq221_e2730_d_n18 * var_vds);
        let eq221_e2732_d_n19: f64 = (eq221_e2730_d_n19 * var_vds);
        let eq221_e2732_d_n20: f64 = (eq221_e2730_d_n20 * var_vds);
        let eq221_e2732_d_n21: f64 = (eq221_e2730_d_n21 * var_vds);
        let eq221_e2732_d_n22: f64 = (eq221_e2730_d_n22 * var_vds);
        let eq221_e2735: f64 = var_ids_fp1;
        let eq221_e2737: f64 = (eq221_e2735 * var_vds_fp1);
        let eq221_e2737_d_n0: f64 = (var_ids_fp1_dn0 * var_vds_fp1);
        let eq221_e2737_d_n1: f64 = (var_ids_fp1_dn1 * var_vds_fp1);
        let eq221_e2737_d_n2: f64 = (var_ids_fp1_dn2 * var_vds_fp1);
        let eq221_e2737_d_n3: f64 = (var_ids_fp1_dn3 * var_vds_fp1);
        let eq221_e2737_d_n4: f64 = (var_ids_fp1_dn4 * var_vds_fp1);
        let eq221_e2737_d_n5: f64 = (var_ids_fp1_dn5 * var_vds_fp1);
        let eq221_e2737_d_n6: f64 = (var_ids_fp1_dn6 * var_vds_fp1);
        let eq221_e2737_d_n7: f64 = ((var_ids_fp1_dn7 * var_vds_fp1) + (eq221_e2735 * var_vds_fp1_dn7));
        let eq221_e2737_d_n8: f64 = (var_ids_fp1_dn8 * var_vds_fp1);
        let eq221_e2737_d_n9: f64 = (var_ids_fp1_dn9 * var_vds_fp1);
        let eq221_e2737_d_n12: f64 = (var_ids_fp1_dn12 * var_vds_fp1);
        let eq221_e2737_d_n14: f64 = (var_ids_fp1_dn14 * var_vds_fp1);
        let eq221_e2737_d_n15: f64 = ((var_ids_fp1_dn15 * var_vds_fp1) + (eq221_e2735 * var_vds_fp1_dn15));
        let eq221_e2737_d_n16: f64 = (var_ids_fp1_dn16 * var_vds_fp1);
        let eq221_e2737_d_n17: f64 = (var_ids_fp1_dn17 * var_vds_fp1);
        let eq221_e2737_d_n18: f64 = (var_ids_fp1_dn18 * var_vds_fp1);
        let eq221_e2737_d_n19: f64 = (var_ids_fp1_dn19 * var_vds_fp1);
        let eq221_e2737_d_n20: f64 = (var_ids_fp1_dn20 * var_vds_fp1);
        let eq221_e2737_d_n21: f64 = (var_ids_fp1_dn21 * var_vds_fp1);
        let eq221_e2737_d_n22: f64 = (var_ids_fp1_dn22 * var_vds_fp1);
        let eq221_e2738: f64 = (eq221_e2732 - eq221_e2737);
        let eq221_e2738_d_n0: f64 = (eq221_e2732_d_n0 - eq221_e2737_d_n0);
        let eq221_e2738_d_n1: f64 = (eq221_e2732_d_n1 - eq221_e2737_d_n1);
        let eq221_e2738_d_n2: f64 = (eq221_e2732_d_n2 - eq221_e2737_d_n2);
        let eq221_e2738_d_n3: f64 = (eq221_e2732_d_n3 - eq221_e2737_d_n3);
        let eq221_e2738_d_n4: f64 = (eq221_e2732_d_n4 - eq221_e2737_d_n4);
        let eq221_e2738_d_n5: f64 = (eq221_e2732_d_n5 - eq221_e2737_d_n5);
        let eq221_e2738_d_n6: f64 = (eq221_e2732_d_n6 - eq221_e2737_d_n6);
        let eq221_e2738_d_n7: f64 = (eq221_e2732_d_n7 - eq221_e2737_d_n7);
        let eq221_e2738_d_n8: f64 = (eq221_e2732_d_n8 - eq221_e2737_d_n8);
        let eq221_e2738_d_n9: f64 = (eq221_e2732_d_n9 - eq221_e2737_d_n9);
        let eq221_e2738_d_n12: f64 = (eq221_e2732_d_n12 - eq221_e2737_d_n12);
        let eq221_e2738_d_n14: f64 = (eq221_e2732_d_n14 - eq221_e2737_d_n14);
        let eq221_e2738_d_n15: f64 = (eq221_e2732_d_n15 - eq221_e2737_d_n15);
        let eq221_e2738_d_n16: f64 = (eq221_e2732_d_n16 - eq221_e2737_d_n16);
        let eq221_e2738_d_n17: f64 = (eq221_e2732_d_n17 - eq221_e2737_d_n17);
        let eq221_e2738_d_n18: f64 = (eq221_e2732_d_n18 - eq221_e2737_d_n18);
        let eq221_e2738_d_n19: f64 = (eq221_e2732_d_n19 - eq221_e2737_d_n19);
        let eq221_e2738_d_n20: f64 = (eq221_e2732_d_n20 - eq221_e2737_d_n20);
        let eq221_e2738_d_n21: f64 = (eq221_e2732_d_n21 - eq221_e2737_d_n21);
        let eq221_e2738_d_n22: f64 = (eq221_e2732_d_n22 - eq221_e2737_d_n22);
        let eq221_e2741: f64 = var_ids_fp2;
        let eq221_e2743: f64 = (eq221_e2741 * var_vds_fp2);
        let eq221_e2743_d_n0: f64 = (var_ids_fp2_dn0 * var_vds_fp2);
        let eq221_e2743_d_n1: f64 = (var_ids_fp2_dn1 * var_vds_fp2);
        let eq221_e2743_d_n2: f64 = (var_ids_fp2_dn2 * var_vds_fp2);
        let eq221_e2743_d_n3: f64 = (var_ids_fp2_dn3 * var_vds_fp2);
        let eq221_e2743_d_n4: f64 = (var_ids_fp2_dn4 * var_vds_fp2);
        let eq221_e2743_d_n5: f64 = (var_ids_fp2_dn5 * var_vds_fp2);
        let eq221_e2743_d_n6: f64 = (var_ids_fp2_dn6 * var_vds_fp2);
        let eq221_e2743_d_n7: f64 = (var_ids_fp2_dn7 * var_vds_fp2);
        let eq221_e2743_d_n8: f64 = (var_ids_fp2_dn8 * var_vds_fp2);
        let eq221_e2743_d_n9: f64 = (var_ids_fp2_dn9 * var_vds_fp2);
        let eq221_e2743_d_n12: f64 = (var_ids_fp2_dn12 * var_vds_fp2);
        let eq221_e2743_d_n14: f64 = (var_ids_fp2_dn14 * var_vds_fp2);
        let eq221_e2743_d_n15: f64 = ((var_ids_fp2_dn15 * var_vds_fp2) + (eq221_e2741 * var_vds_fp2_dn15));
        let eq221_e2743_d_n16: f64 = ((var_ids_fp2_dn16 * var_vds_fp2) + (eq221_e2741 * var_vds_fp2_dn16));
        let eq221_e2743_d_n17: f64 = (var_ids_fp2_dn17 * var_vds_fp2);
        let eq221_e2743_d_n18: f64 = (var_ids_fp2_dn18 * var_vds_fp2);
        let eq221_e2743_d_n19: f64 = (var_ids_fp2_dn19 * var_vds_fp2);
        let eq221_e2743_d_n20: f64 = (var_ids_fp2_dn20 * var_vds_fp2);
        let eq221_e2743_d_n21: f64 = (var_ids_fp2_dn21 * var_vds_fp2);
        let eq221_e2743_d_n22: f64 = (var_ids_fp2_dn22 * var_vds_fp2);
        let eq221_e2744: f64 = (eq221_e2738 - eq221_e2743);
        let eq221_e2744_d_n0: f64 = (eq221_e2738_d_n0 - eq221_e2743_d_n0);
        let eq221_e2744_d_n1: f64 = (eq221_e2738_d_n1 - eq221_e2743_d_n1);
        let eq221_e2744_d_n2: f64 = (eq221_e2738_d_n2 - eq221_e2743_d_n2);
        let eq221_e2744_d_n3: f64 = (eq221_e2738_d_n3 - eq221_e2743_d_n3);
        let eq221_e2744_d_n4: f64 = (eq221_e2738_d_n4 - eq221_e2743_d_n4);
        let eq221_e2744_d_n5: f64 = (eq221_e2738_d_n5 - eq221_e2743_d_n5);
        let eq221_e2744_d_n6: f64 = (eq221_e2738_d_n6 - eq221_e2743_d_n6);
        let eq221_e2744_d_n7: f64 = (eq221_e2738_d_n7 - eq221_e2743_d_n7);
        let eq221_e2744_d_n8: f64 = (eq221_e2738_d_n8 - eq221_e2743_d_n8);
        let eq221_e2744_d_n9: f64 = (eq221_e2738_d_n9 - eq221_e2743_d_n9);
        let eq221_e2744_d_n12: f64 = (eq221_e2738_d_n12 - eq221_e2743_d_n12);
        let eq221_e2744_d_n14: f64 = (eq221_e2738_d_n14 - eq221_e2743_d_n14);
        let eq221_e2744_d_n15: f64 = (eq221_e2738_d_n15 - eq221_e2743_d_n15);
        let eq221_e2744_d_n16: f64 = (eq221_e2738_d_n16 - eq221_e2743_d_n16);
        let eq221_e2744_d_n17: f64 = (eq221_e2738_d_n17 - eq221_e2743_d_n17);
        let eq221_e2744_d_n18: f64 = (eq221_e2738_d_n18 - eq221_e2743_d_n18);
        let eq221_e2744_d_n19: f64 = (eq221_e2738_d_n19 - eq221_e2743_d_n19);
        let eq221_e2744_d_n20: f64 = (eq221_e2738_d_n20 - eq221_e2743_d_n20);
        let eq221_e2744_d_n21: f64 = (eq221_e2738_d_n21 - eq221_e2743_d_n21);
        let eq221_e2744_d_n22: f64 = (eq221_e2738_d_n22 - eq221_e2743_d_n22);
        let eq221_e2747: f64 = var_ids_fp3;
        let eq221_e2749: f64 = (eq221_e2747 * var_vds_fp3);
        let eq221_e2749_d_n0: f64 = (var_ids_fp3_dn0 * var_vds_fp3);
        let eq221_e2749_d_n1: f64 = (var_ids_fp3_dn1 * var_vds_fp3);
        let eq221_e2749_d_n2: f64 = (var_ids_fp3_dn2 * var_vds_fp3);
        let eq221_e2749_d_n3: f64 = (var_ids_fp3_dn3 * var_vds_fp3);
        let eq221_e2749_d_n4: f64 = (var_ids_fp3_dn4 * var_vds_fp3);
        let eq221_e2749_d_n5: f64 = (var_ids_fp3_dn5 * var_vds_fp3);
        let eq221_e2749_d_n6: f64 = (var_ids_fp3_dn6 * var_vds_fp3);
        let eq221_e2749_d_n7: f64 = (var_ids_fp3_dn7 * var_vds_fp3);
        let eq221_e2749_d_n8: f64 = (var_ids_fp3_dn8 * var_vds_fp3);
        let eq221_e2749_d_n9: f64 = (var_ids_fp3_dn9 * var_vds_fp3);
        let eq221_e2749_d_n12: f64 = (var_ids_fp3_dn12 * var_vds_fp3);
        let eq221_e2749_d_n14: f64 = (var_ids_fp3_dn14 * var_vds_fp3);
        let eq221_e2749_d_n15: f64 = (var_ids_fp3_dn15 * var_vds_fp3);
        let eq221_e2749_d_n16: f64 = ((var_ids_fp3_dn16 * var_vds_fp3) + (eq221_e2747 * var_vds_fp3_dn16));
        let eq221_e2749_d_n17: f64 = ((var_ids_fp3_dn17 * var_vds_fp3) + (eq221_e2747 * var_vds_fp3_dn17));
        let eq221_e2749_d_n18: f64 = (var_ids_fp3_dn18 * var_vds_fp3);
        let eq221_e2749_d_n19: f64 = (var_ids_fp3_dn19 * var_vds_fp3);
        let eq221_e2749_d_n20: f64 = (var_ids_fp3_dn20 * var_vds_fp3);
        let eq221_e2749_d_n21: f64 = (var_ids_fp3_dn21 * var_vds_fp3);
        let eq221_e2749_d_n22: f64 = (var_ids_fp3_dn22 * var_vds_fp3);
        let eq221_e2750: f64 = (eq221_e2744 - eq221_e2749);
        let eq221_e2750_d_n0: f64 = (eq221_e2744_d_n0 - eq221_e2749_d_n0);
        let eq221_e2750_d_n1: f64 = (eq221_e2744_d_n1 - eq221_e2749_d_n1);
        let eq221_e2750_d_n2: f64 = (eq221_e2744_d_n2 - eq221_e2749_d_n2);
        let eq221_e2750_d_n3: f64 = (eq221_e2744_d_n3 - eq221_e2749_d_n3);
        let eq221_e2750_d_n4: f64 = (eq221_e2744_d_n4 - eq221_e2749_d_n4);
        let eq221_e2750_d_n5: f64 = (eq221_e2744_d_n5 - eq221_e2749_d_n5);
        let eq221_e2750_d_n6: f64 = (eq221_e2744_d_n6 - eq221_e2749_d_n6);
        let eq221_e2750_d_n7: f64 = (eq221_e2744_d_n7 - eq221_e2749_d_n7);
        let eq221_e2750_d_n8: f64 = (eq221_e2744_d_n8 - eq221_e2749_d_n8);
        let eq221_e2750_d_n9: f64 = (eq221_e2744_d_n9 - eq221_e2749_d_n9);
        let eq221_e2750_d_n12: f64 = (eq221_e2744_d_n12 - eq221_e2749_d_n12);
        let eq221_e2750_d_n14: f64 = (eq221_e2744_d_n14 - eq221_e2749_d_n14);
        let eq221_e2750_d_n15: f64 = (eq221_e2744_d_n15 - eq221_e2749_d_n15);
        let eq221_e2750_d_n16: f64 = (eq221_e2744_d_n16 - eq221_e2749_d_n16);
        let eq221_e2750_d_n17: f64 = (eq221_e2744_d_n17 - eq221_e2749_d_n17);
        let eq221_e2750_d_n18: f64 = (eq221_e2744_d_n18 - eq221_e2749_d_n18);
        let eq221_e2750_d_n19: f64 = (eq221_e2744_d_n19 - eq221_e2749_d_n19);
        let eq221_e2750_d_n20: f64 = (eq221_e2744_d_n20 - eq221_e2749_d_n20);
        let eq221_e2750_d_n21: f64 = (eq221_e2744_d_n21 - eq221_e2749_d_n21);
        let eq221_e2750_d_n22: f64 = (eq221_e2744_d_n22 - eq221_e2749_d_n22);
        let eq221_e2753: f64 = var_ids_fp4;
        let eq221_e2755: f64 = (eq221_e2753 * var_vds_fp4);
        let eq221_e2755_d_n0: f64 = (var_ids_fp4_dn0 * var_vds_fp4);
        let eq221_e2755_d_n1: f64 = (var_ids_fp4_dn1 * var_vds_fp4);
        let eq221_e2755_d_n2: f64 = (var_ids_fp4_dn2 * var_vds_fp4);
        let eq221_e2755_d_n3: f64 = (var_ids_fp4_dn3 * var_vds_fp4);
        let eq221_e2755_d_n4: f64 = (var_ids_fp4_dn4 * var_vds_fp4);
        let eq221_e2755_d_n5: f64 = (var_ids_fp4_dn5 * var_vds_fp4);
        let eq221_e2755_d_n6: f64 = (var_ids_fp4_dn6 * var_vds_fp4);
        let eq221_e2755_d_n7: f64 = (var_ids_fp4_dn7 * var_vds_fp4);
        let eq221_e2755_d_n8: f64 = (var_ids_fp4_dn8 * var_vds_fp4);
        let eq221_e2755_d_n9: f64 = (var_ids_fp4_dn9 * var_vds_fp4);
        let eq221_e2755_d_n12: f64 = (var_ids_fp4_dn12 * var_vds_fp4);
        let eq221_e2755_d_n14: f64 = (var_ids_fp4_dn14 * var_vds_fp4);
        let eq221_e2755_d_n15: f64 = (var_ids_fp4_dn15 * var_vds_fp4);
        let eq221_e2755_d_n16: f64 = (var_ids_fp4_dn16 * var_vds_fp4);
        let eq221_e2755_d_n17: f64 = ((var_ids_fp4_dn17 * var_vds_fp4) + (eq221_e2753 * var_vds_fp4_dn17));
        let eq221_e2755_d_n18: f64 = ((var_ids_fp4_dn18 * var_vds_fp4) + (eq221_e2753 * var_vds_fp4_dn18));
        let eq221_e2755_d_n19: f64 = (var_ids_fp4_dn19 * var_vds_fp4);
        let eq221_e2755_d_n20: f64 = (var_ids_fp4_dn20 * var_vds_fp4);
        let eq221_e2755_d_n21: f64 = (var_ids_fp4_dn21 * var_vds_fp4);
        let eq221_e2755_d_n22: f64 = (var_ids_fp4_dn22 * var_vds_fp4);
        let eq221_e2756: f64 = (eq221_e2750 - eq221_e2755);
        let eq221_e2756_d_n0: f64 = (eq221_e2750_d_n0 - eq221_e2755_d_n0);
        let eq221_e2756_d_n1: f64 = (eq221_e2750_d_n1 - eq221_e2755_d_n1);
        let eq221_e2756_d_n2: f64 = (eq221_e2750_d_n2 - eq221_e2755_d_n2);
        let eq221_e2756_d_n3: f64 = (eq221_e2750_d_n3 - eq221_e2755_d_n3);
        let eq221_e2756_d_n4: f64 = (eq221_e2750_d_n4 - eq221_e2755_d_n4);
        let eq221_e2756_d_n5: f64 = (eq221_e2750_d_n5 - eq221_e2755_d_n5);
        let eq221_e2756_d_n6: f64 = (eq221_e2750_d_n6 - eq221_e2755_d_n6);
        let eq221_e2756_d_n7: f64 = (eq221_e2750_d_n7 - eq221_e2755_d_n7);
        let eq221_e2756_d_n8: f64 = (eq221_e2750_d_n8 - eq221_e2755_d_n8);
        let eq221_e2756_d_n9: f64 = (eq221_e2750_d_n9 - eq221_e2755_d_n9);
        let eq221_e2756_d_n12: f64 = (eq221_e2750_d_n12 - eq221_e2755_d_n12);
        let eq221_e2756_d_n14: f64 = (eq221_e2750_d_n14 - eq221_e2755_d_n14);
        let eq221_e2756_d_n15: f64 = (eq221_e2750_d_n15 - eq221_e2755_d_n15);
        let eq221_e2756_d_n16: f64 = (eq221_e2750_d_n16 - eq221_e2755_d_n16);
        let eq221_e2756_d_n17: f64 = (eq221_e2750_d_n17 - eq221_e2755_d_n17);
        let eq221_e2756_d_n18: f64 = (eq221_e2750_d_n18 - eq221_e2755_d_n18);
        let eq221_e2756_d_n19: f64 = (eq221_e2750_d_n19 - eq221_e2755_d_n19);
        let eq221_e2756_d_n20: f64 = (eq221_e2750_d_n20 - eq221_e2755_d_n20);
        let eq221_e2756_d_n21: f64 = (eq221_e2750_d_n21 - eq221_e2755_d_n21);
        let eq221_e2756_d_n22: f64 = (eq221_e2750_d_n22 - eq221_e2755_d_n22);
        (eq221_e2756, eq221_e2756_d_n0, eq221_e2756_d_n1, eq221_e2756_d_n2, eq221_e2756_d_n3, eq221_e2756_d_n4, eq221_e2756_d_n5, eq221_e2756_d_n6, eq221_e2756_d_n7, eq221_e2756_d_n8, eq221_e2756_d_n9, eq221_e2756_d_n12, eq221_e2756_d_n14, eq221_e2756_d_n15, eq221_e2756_d_n16, eq221_e2756_d_n17, eq221_e2756_d_n18, eq221_e2756_d_n19, eq221_e2756_d_n20, eq221_e2756_d_n21, eq221_e2756_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq221_value: f64 = eq221_e2758;
        let eq221_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq221_node_derivatives: [f64; 20] = [eq221_e2758_d_n0, eq221_e2758_d_n1, eq221_e2758_d_n2, eq221_e2758_d_n3, eq221_e2758_d_n4, eq221_e2758_d_n5, eq221_e2758_d_n6, eq221_e2758_d_n7, eq221_e2758_d_n8, eq221_e2758_d_n9, eq221_e2758_d_n12, eq221_e2758_d_n14, eq221_e2758_d_n15, eq221_e2758_d_n16, eq221_e2758_d_n17, eq221_e2758_d_n18, eq221_e2758_d_n19, eq221_e2758_d_n20, eq221_e2758_d_n21, eq221_e2758_d_n22];
        let eq221_branch_derivative_indices: [usize; 0] = [];
        let eq221_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq221_value),
            &eq221_node_derivative_indices,
            &eq221_node_derivatives,
            &eq221_branch_derivative_indices,
            &eq221_branch_derivatives,
            multiplicity,
        );
        let (eq222_e2764, eq222_e2764_d_n4,) = {
    if (var_guard576 != 0.0) {
        let __rspice_inv_cse_0: f64 = 1.0 / p.p32;
        let eq222_e2762: f64 = ((nv4 - 0.0) * __rspice_inv_cse_0);
        let eq222_e2762_d_n4: f64 = (1.0 * __rspice_inv_cse_0);
        (eq222_e2762, eq222_e2762_d_n4,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq222_value: f64 = eq222_e2764;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq222_value),
            4,
            multiplicity * (eq222_e2764_d_n4),
        );
        let (eq223_e2771, eq223_e2771_d_n4,) = {
    if (var_guard576 != 0.0) {
        let eq223_e2768: f64 = ((nv4 - 0.0) * p.p33);
        let eq223_e2769: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 120, eq223_e2768);
        (eq223_e2769, (p.p33 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq223_value: f64 = eq223_e2771;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq223_value),
            4,
            multiplicity * (eq223_e2771_d_n4),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_ct: f64,
        var_ct_dn0: f64,
        var_ct_dn1: f64,
        var_ct_dn12: f64,
        var_ct_dn14: f64,
        var_ct_dn15: f64,
        var_ct_dn16: f64,
        var_ct_dn17: f64,
        var_ct_dn18: f64,
        var_ct_dn19: f64,
        var_ct_dn2: f64,
        var_ct_dn20: f64,
        var_ct_dn21: f64,
        var_ct_dn22: f64,
        var_ct_dn3: f64,
        var_ct_dn4: f64,
        var_ct_dn5: f64,
        var_ct_dn6: f64,
        var_ct_dn7: f64,
        var_ct_dn8: f64,
        var_ct_dn9: f64,
        var_guard353: f64,
        var_guard354: f64,
        var_guard355: f64,
        var_guard356: f64,
        var_guard357: f64,
        var_guard358: f64,
        var_guard535: f64,
        var_guard536: f64,
        var_guard537: f64,
        var_qbdov: f64,
        var_qbdov_dn0: f64,
        var_qbdov_dn3: f64,
        var_qbgov: f64,
        var_qbgov_dn1: f64,
        var_qbgov_dn3: f64,
        var_qbsov: f64,
        var_qbsov_dn2: f64,
        var_qbsov_dn3: f64,
        var_qd_fp1: f64,
        var_qd_fp1_dn0: f64,
        var_qd_fp1_dn1: f64,
        var_qd_fp1_dn12: f64,
        var_qd_fp1_dn14: f64,
        var_qd_fp1_dn15: f64,
        var_qd_fp1_dn16: f64,
        var_qd_fp1_dn17: f64,
        var_qd_fp1_dn18: f64,
        var_qd_fp1_dn19: f64,
        var_qd_fp1_dn2: f64,
        var_qd_fp1_dn20: f64,
        var_qd_fp1_dn21: f64,
        var_qd_fp1_dn22: f64,
        var_qd_fp1_dn3: f64,
        var_qd_fp1_dn4: f64,
        var_qd_fp1_dn5: f64,
        var_qd_fp1_dn6: f64,
        var_qd_fp1_dn7: f64,
        var_qd_fp1_dn8: f64,
        var_qd_fp1_dn9: f64,
        var_qdint: f64,
        var_qdint_dn0: f64,
        var_qdint_dn1: f64,
        var_qdint_dn12: f64,
        var_qdint_dn14: f64,
        var_qdint_dn15: f64,
        var_qdint_dn16: f64,
        var_qdint_dn17: f64,
        var_qdint_dn18: f64,
        var_qdint_dn19: f64,
        var_qdint_dn2: f64,
        var_qdint_dn20: f64,
        var_qdint_dn21: f64,
        var_qdint_dn22: f64,
        var_qdint_dn3: f64,
        var_qdint_dn4: f64,
        var_qdint_dn5: f64,
        var_qdint_dn6: f64,
        var_qdint_dn7: f64,
        var_qdint_dn8: f64,
        var_qdint_dn9: f64,
        var_qdov: f64,
        var_qdov_dn0: f64,
        var_qdov_dn1: f64,
        var_qdov_dn10: f64,
        var_qdov_dn2: f64,
        var_qdsov: f64,
        var_qdsov_dn0: f64,
        var_qdsov_dn2: f64,
        var_qgint: f64,
        var_qgint_dn0: f64,
        var_qgint_dn1: f64,
        var_qgint_dn12: f64,
        var_qgint_dn14: f64,
        var_qgint_dn15: f64,
        var_qgint_dn16: f64,
        var_qgint_dn17: f64,
        var_qgint_dn18: f64,
        var_qgint_dn19: f64,
        var_qgint_dn2: f64,
        var_qgint_dn20: f64,
        var_qgint_dn21: f64,
        var_qgint_dn22: f64,
        var_qgint_dn3: f64,
        var_qgint_dn4: f64,
        var_qgint_dn5: f64,
        var_qgint_dn6: f64,
        var_qgint_dn7: f64,
        var_qgint_dn8: f64,
        var_qgint_dn9: f64,
        var_qsov: f64,
        var_qsov_dn1: f64,
        var_qsov_dn10: f64,
        var_qsov_dn2: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq9_e355, eq9_e355_d_n5, eq9_e355_q,) = {
    if ((var_guard354 != 0.0) && (var_guard353 == 0.0)) {
        let eq9_e352_q: f64 = (nv5 - 0.0);
        let eq9_e353: f64 = (p.p97 * (nv5 - 0.0));
        let eq9_e353_q: f64 = (p.p97 * eq9_e352_q);
        (eq9_e353, p.p97, eq9_e353_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[5]),
            None,
            nodes[5],
            multiplicity * (eq9_e355_d_n5),
        );
        let (eq17_e427, eq17_e427_d_n5, eq17_e427_q,) = {
    if ((var_guard355 != 0.0) && (!((var_guard353 != 0.0) || (var_guard354 != 0.0)))) {
        let eq17_e424_q: f64 = (nv5 - 0.0);
        let eq17_e425: f64 = (p.p110 * (nv5 - 0.0));
        let eq17_e425_q: f64 = (p.p110 * eq17_e424_q);
        (eq17_e425, p.p110, eq17_e425_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[5]),
            None,
            nodes[5],
            multiplicity * (eq17_e427_d_n5),
        );
        let (eq20_e462, eq20_e462_d_n6, eq20_e462_q,) = {
    if ((var_guard355 != 0.0) && (!((var_guard353 != 0.0) || (var_guard354 != 0.0)))) {
        let eq20_e459_q: f64 = (nv6 - 0.0);
        let eq20_e460: f64 = (p.p111 * (nv6 - 0.0));
        let eq20_e460_q: f64 = (p.p111 * eq20_e459_q);
        (eq20_e460, p.p111, eq20_e460_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[6]),
            None,
            nodes[6],
            multiplicity * (eq20_e462_d_n6),
        );
        let (eq27_e539, eq27_e539_d_n0, eq27_e539_d_n1, eq27_e539_d_n2, eq27_e539_d_n3, eq27_e539_d_n4, eq27_e539_d_n5, eq27_e539_d_n6, eq27_e539_d_n7, eq27_e539_d_n8, eq27_e539_d_n9, eq27_e539_d_n12, eq27_e539_d_n14, eq27_e539_d_n15, eq27_e539_d_n16, eq27_e539_d_n17, eq27_e539_d_n18, eq27_e539_d_n19, eq27_e539_d_n20, eq27_e539_d_n21, eq27_e539_d_n22, eq27_e539_q, eq27_e539_q_d_n0, eq27_e539_q_d_n1, eq27_e539_q_d_n2, eq27_e539_q_d_n3, eq27_e539_q_d_n4, eq27_e539_q_d_n5, eq27_e539_q_d_n6, eq27_e539_q_d_n7, eq27_e539_q_d_n8, eq27_e539_q_d_n9, eq27_e539_q_d_n12, eq27_e539_q_d_n14, eq27_e539_q_d_n15, eq27_e539_q_d_n16, eq27_e539_q_d_n17, eq27_e539_q_d_n18, eq27_e539_q_d_n19, eq27_e539_q_d_n20, eq27_e539_q_d_n21, eq27_e539_q_d_n22,) = {
    if ((var_guard356 != 0.0) && (!(((var_guard353 != 0.0) || (var_guard354 != 0.0)) || (var_guard355 != 0.0)))) {
        let eq27_e536_q: f64 = (nv5 - 0.0);
        let eq27_e537: f64 = (var_ct * (nv5 - 0.0));
        let eq27_e537_d_n0: f64 = (var_ct_dn0 * (nv5 - 0.0));
        let eq27_e537_d_n1: f64 = (var_ct_dn1 * (nv5 - 0.0));
        let eq27_e537_d_n2: f64 = (var_ct_dn2 * (nv5 - 0.0));
        let eq27_e537_d_n3: f64 = (var_ct_dn3 * (nv5 - 0.0));
        let eq27_e537_d_n4: f64 = (var_ct_dn4 * (nv5 - 0.0));
        let eq27_e537_d_n5: f64 = ((var_ct_dn5 * (nv5 - 0.0)) + var_ct);
        let eq27_e537_d_n6: f64 = (var_ct_dn6 * (nv5 - 0.0));
        let eq27_e537_d_n7: f64 = (var_ct_dn7 * (nv5 - 0.0));
        let eq27_e537_d_n8: f64 = (var_ct_dn8 * (nv5 - 0.0));
        let eq27_e537_d_n9: f64 = (var_ct_dn9 * (nv5 - 0.0));
        let eq27_e537_d_n12: f64 = (var_ct_dn12 * (nv5 - 0.0));
        let eq27_e537_d_n14: f64 = (var_ct_dn14 * (nv5 - 0.0));
        let eq27_e537_d_n15: f64 = (var_ct_dn15 * (nv5 - 0.0));
        let eq27_e537_d_n16: f64 = (var_ct_dn16 * (nv5 - 0.0));
        let eq27_e537_d_n17: f64 = (var_ct_dn17 * (nv5 - 0.0));
        let eq27_e537_d_n18: f64 = (var_ct_dn18 * (nv5 - 0.0));
        let eq27_e537_d_n19: f64 = (var_ct_dn19 * (nv5 - 0.0));
        let eq27_e537_d_n20: f64 = (var_ct_dn20 * (nv5 - 0.0));
        let eq27_e537_d_n21: f64 = (var_ct_dn21 * (nv5 - 0.0));
        let eq27_e537_d_n22: f64 = (var_ct_dn22 * (nv5 - 0.0));
        let eq27_e537_q: f64 = (var_ct * eq27_e536_q);
        let eq27_e537_q_d_n0: f64 = (var_ct_dn0 * eq27_e536_q);
        let eq27_e537_q_d_n1: f64 = (var_ct_dn1 * eq27_e536_q);
        let eq27_e537_q_d_n2: f64 = (var_ct_dn2 * eq27_e536_q);
        let eq27_e537_q_d_n3: f64 = (var_ct_dn3 * eq27_e536_q);
        let eq27_e537_q_d_n4: f64 = (var_ct_dn4 * eq27_e536_q);
        let eq27_e537_q_d_n5: f64 = ((var_ct_dn5 * eq27_e536_q) + var_ct);
        let eq27_e537_q_d_n6: f64 = (var_ct_dn6 * eq27_e536_q);
        let eq27_e537_q_d_n7: f64 = (var_ct_dn7 * eq27_e536_q);
        let eq27_e537_q_d_n8: f64 = (var_ct_dn8 * eq27_e536_q);
        let eq27_e537_q_d_n9: f64 = (var_ct_dn9 * eq27_e536_q);
        let eq27_e537_q_d_n12: f64 = (var_ct_dn12 * eq27_e536_q);
        let eq27_e537_q_d_n14: f64 = (var_ct_dn14 * eq27_e536_q);
        let eq27_e537_q_d_n15: f64 = (var_ct_dn15 * eq27_e536_q);
        let eq27_e537_q_d_n16: f64 = (var_ct_dn16 * eq27_e536_q);
        let eq27_e537_q_d_n17: f64 = (var_ct_dn17 * eq27_e536_q);
        let eq27_e537_q_d_n18: f64 = (var_ct_dn18 * eq27_e536_q);
        let eq27_e537_q_d_n19: f64 = (var_ct_dn19 * eq27_e536_q);
        let eq27_e537_q_d_n20: f64 = (var_ct_dn20 * eq27_e536_q);
        let eq27_e537_q_d_n21: f64 = (var_ct_dn21 * eq27_e536_q);
        let eq27_e537_q_d_n22: f64 = (var_ct_dn22 * eq27_e536_q);
        (eq27_e537, eq27_e537_d_n0, eq27_e537_d_n1, eq27_e537_d_n2, eq27_e537_d_n3, eq27_e537_d_n4, eq27_e537_d_n5, eq27_e537_d_n6, eq27_e537_d_n7, eq27_e537_d_n8, eq27_e537_d_n9, eq27_e537_d_n12, eq27_e537_d_n14, eq27_e537_d_n15, eq27_e537_d_n16, eq27_e537_d_n17, eq27_e537_d_n18, eq27_e537_d_n19, eq27_e537_d_n20, eq27_e537_d_n21, eq27_e537_d_n22, eq27_e537_q, eq27_e537_q_d_n0, eq27_e537_q_d_n1, eq27_e537_q_d_n2, eq27_e537_q_d_n3, eq27_e537_q_d_n4, eq27_e537_q_d_n5, eq27_e537_q_d_n6, eq27_e537_q_d_n7, eq27_e537_q_d_n8, eq27_e537_q_d_n9, eq27_e537_q_d_n12, eq27_e537_q_d_n14, eq27_e537_q_d_n15, eq27_e537_q_d_n16, eq27_e537_q_d_n17, eq27_e537_q_d_n18, eq27_e537_q_d_n19, eq27_e537_q_d_n20, eq27_e537_q_d_n21, eq27_e537_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_reactive_node_derivatives: [f64; 23] = [eq27_e539_q_d_n0, eq27_e539_q_d_n1, eq27_e539_q_d_n2, eq27_e539_q_d_n3, eq27_e539_q_d_n4, eq27_e539_q_d_n5, eq27_e539_q_d_n6, eq27_e539_q_d_n7, eq27_e539_q_d_n8, eq27_e539_q_d_n9, 0.0, 0.0, eq27_e539_q_d_n12, 0.0, eq27_e539_q_d_n14, eq27_e539_q_d_n15, eq27_e539_q_d_n16, eq27_e539_q_d_n17, eq27_e539_q_d_n18, eq27_e539_q_d_n19, eq27_e539_q_d_n20, eq27_e539_q_d_n21, eq27_e539_q_d_n22];
        let eq27_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq27_reactive_node_derivatives,
            branches,
            &eq27_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq37_e668, eq37_e668_d_n12, eq37_e668_q, eq37_e668_q_d_n12,) = {
    if ((var_guard357 != 0.0) && (!((((var_guard353 != 0.0) || (var_guard354 != 0.0)) || (var_guard355 != 0.0)) || (var_guard356 != 0.0)))) {
        let eq37_e661_q: f64 = (nv12 - 0.0);
        let eq37_e662: f64 = (p.p97 * (nv12 - 0.0));
        let eq37_e662_q: f64 = (p.p97 * eq37_e661_q);
        let eq37_e665: f64 = (1e-12 * (nv12 - 0.0));
        let eq37_e666: f64 = (eq37_e662 + eq37_e665);
        let eq37_e666_d_n12: f64 = (p.p97 + 1e-12);
        let eq37_e666_q: f64 = eq37_e662_q;
        (eq37_e666, eq37_e666_d_n12, eq37_e666_q, p.p97,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[12]),
            None,
            nodes[12],
            multiplicity * (eq37_e668_q_d_n12),
        );
        let (eq40_e716, eq40_e716_d_n14, eq40_e716_q, eq40_e716_q_d_n14,) = {
    if ((var_guard357 != 0.0) && (!((((var_guard353 != 0.0) || (var_guard354 != 0.0)) || (var_guard355 != 0.0)) || (var_guard356 != 0.0)))) {
        let eq40_e709_q: f64 = (nv14 - 0.0);
        let eq40_e710: f64 = (p.p83 * (nv14 - 0.0));
        let eq40_e710_q: f64 = (p.p83 * eq40_e709_q);
        let eq40_e713: f64 = (1e-12 * (nv14 - 0.0));
        let eq40_e714: f64 = (eq40_e710 + eq40_e713);
        let eq40_e714_d_n14: f64 = (p.p83 + 1e-12);
        let eq40_e714_q: f64 = eq40_e710_q;
        (eq40_e714, eq40_e714_d_n14, eq40_e714_q, p.p83,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[14]),
            None,
            nodes[14],
            multiplicity * (eq40_e716_q_d_n14),
        );
        let (eq43_e784, eq43_e784_d_n5, eq43_e784_q,) = {
    if ((var_guard358 != 0.0) && (!(((((var_guard353 != 0.0) || (var_guard354 != 0.0)) || (var_guard355 != 0.0)) || (var_guard356 != 0.0)) || (var_guard357 != 0.0)))) {
        let eq43_e781_q: f64 = (nv5 - 0.0);
        let eq43_e782: f64 = (p.p135 * (nv5 - 0.0));
        let eq43_e782_q: f64 = (p.p135 * eq43_e781_q);
        (eq43_e782, p.p135, eq43_e782_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[5]),
            None,
            nodes[5],
            multiplicity * (eq43_e784_d_n5),
        );
        let (eq46_e852, eq46_e852_d_n6, eq46_e852_q,) = {
    if ((var_guard358 != 0.0) && (!(((((var_guard353 != 0.0) || (var_guard354 != 0.0)) || (var_guard355 != 0.0)) || (var_guard356 != 0.0)) || (var_guard357 != 0.0)))) {
        let eq46_e849_q: f64 = (nv6 - 0.0);
        let eq46_e850: f64 = (p.p144 * (nv6 - 0.0));
        let eq46_e850_q: f64 = (p.p144 * eq46_e849_q);
        (eq46_e850, p.p144, eq46_e850_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[6]),
            None,
            nodes[6],
            multiplicity * (eq46_e852_d_n6),
        );
        let eq109_e1474_q: f64 = var_qdint;
        let eq109_e1475: f64 = (p.p7 * var_qdint);
        let eq109_e1475_d_n0: f64 = (p.p7 * var_qdint_dn0);
        let eq109_e1475_d_n1: f64 = (p.p7 * var_qdint_dn1);
        let eq109_e1475_d_n2: f64 = (p.p7 * var_qdint_dn2);
        let eq109_e1475_d_n3: f64 = (p.p7 * var_qdint_dn3);
        let eq109_e1475_d_n4: f64 = (p.p7 * var_qdint_dn4);
        let eq109_e1475_d_n5: f64 = (p.p7 * var_qdint_dn5);
        let eq109_e1475_d_n6: f64 = (p.p7 * var_qdint_dn6);
        let eq109_e1475_d_n7: f64 = (p.p7 * var_qdint_dn7);
        let eq109_e1475_d_n8: f64 = (p.p7 * var_qdint_dn8);
        let eq109_e1475_d_n9: f64 = (p.p7 * var_qdint_dn9);
        let eq109_e1475_d_n12: f64 = (p.p7 * var_qdint_dn12);
        let eq109_e1475_d_n14: f64 = (p.p7 * var_qdint_dn14);
        let eq109_e1475_d_n15: f64 = (p.p7 * var_qdint_dn15);
        let eq109_e1475_d_n16: f64 = (p.p7 * var_qdint_dn16);
        let eq109_e1475_d_n17: f64 = (p.p7 * var_qdint_dn17);
        let eq109_e1475_d_n18: f64 = (p.p7 * var_qdint_dn18);
        let eq109_e1475_d_n19: f64 = (p.p7 * var_qdint_dn19);
        let eq109_e1475_d_n20: f64 = (p.p7 * var_qdint_dn20);
        let eq109_e1475_d_n21: f64 = (p.p7 * var_qdint_dn21);
        let eq109_e1475_d_n22: f64 = (p.p7 * var_qdint_dn22);
        let eq109_e1475_q: f64 = (p.p7 * eq109_e1474_q);
        let eq109_reactive_node_derivatives: [f64; 23] = [eq109_e1475_d_n0, eq109_e1475_d_n1, eq109_e1475_d_n2, eq109_e1475_d_n3, eq109_e1475_d_n4, eq109_e1475_d_n5, eq109_e1475_d_n6, eq109_e1475_d_n7, eq109_e1475_d_n8, eq109_e1475_d_n9, 0.0, 0.0, eq109_e1475_d_n12, 0.0, eq109_e1475_d_n14, eq109_e1475_d_n15, eq109_e1475_d_n16, eq109_e1475_d_n17, eq109_e1475_d_n18, eq109_e1475_d_n19, eq109_e1475_d_n20, eq109_e1475_d_n21, eq109_e1475_d_n22];
        let eq109_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[8]),
            nodes,
            &eq109_reactive_node_derivatives,
            branches,
            &eq109_reactive_branch_derivatives,
            multiplicity,
        );
        let eq110_e1478_q: f64 = var_qgint;
        let eq110_e1479: f64 = (p.p7 * var_qgint);
        let eq110_e1479_d_n0: f64 = (p.p7 * var_qgint_dn0);
        let eq110_e1479_d_n1: f64 = (p.p7 * var_qgint_dn1);
        let eq110_e1479_d_n2: f64 = (p.p7 * var_qgint_dn2);
        let eq110_e1479_d_n3: f64 = (p.p7 * var_qgint_dn3);
        let eq110_e1479_d_n4: f64 = (p.p7 * var_qgint_dn4);
        let eq110_e1479_d_n5: f64 = (p.p7 * var_qgint_dn5);
        let eq110_e1479_d_n6: f64 = (p.p7 * var_qgint_dn6);
        let eq110_e1479_d_n7: f64 = (p.p7 * var_qgint_dn7);
        let eq110_e1479_d_n8: f64 = (p.p7 * var_qgint_dn8);
        let eq110_e1479_d_n9: f64 = (p.p7 * var_qgint_dn9);
        let eq110_e1479_d_n12: f64 = (p.p7 * var_qgint_dn12);
        let eq110_e1479_d_n14: f64 = (p.p7 * var_qgint_dn14);
        let eq110_e1479_d_n15: f64 = (p.p7 * var_qgint_dn15);
        let eq110_e1479_d_n16: f64 = (p.p7 * var_qgint_dn16);
        let eq110_e1479_d_n17: f64 = (p.p7 * var_qgint_dn17);
        let eq110_e1479_d_n18: f64 = (p.p7 * var_qgint_dn18);
        let eq110_e1479_d_n19: f64 = (p.p7 * var_qgint_dn19);
        let eq110_e1479_d_n20: f64 = (p.p7 * var_qgint_dn20);
        let eq110_e1479_d_n21: f64 = (p.p7 * var_qgint_dn21);
        let eq110_e1479_d_n22: f64 = (p.p7 * var_qgint_dn22);
        let eq110_e1479_q: f64 = (p.p7 * eq110_e1478_q);
        let eq110_reactive_node_derivatives: [f64; 23] = [eq110_e1479_d_n0, eq110_e1479_d_n1, eq110_e1479_d_n2, eq110_e1479_d_n3, eq110_e1479_d_n4, eq110_e1479_d_n5, eq110_e1479_d_n6, eq110_e1479_d_n7, eq110_e1479_d_n8, eq110_e1479_d_n9, 0.0, 0.0, eq110_e1479_d_n12, 0.0, eq110_e1479_d_n14, eq110_e1479_d_n15, eq110_e1479_d_n16, eq110_e1479_d_n17, eq110_e1479_d_n18, eq110_e1479_d_n19, eq110_e1479_d_n20, eq110_e1479_d_n21, eq110_e1479_d_n22];
        let eq110_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq110_reactive_node_derivatives,
            branches,
            &eq110_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq111_e1486, eq111_e1486_d_n1, eq111_e1486_d_n2, eq111_e1486_d_n10, eq111_e1486_q,) = {
    if (var_guard535 != 0.0) {
        let eq111_e1483_q: f64 = var_qsov;
        let eq111_e1484: f64 = (p.p7 * var_qsov);
        let eq111_e1484_d_n1: f64 = (p.p7 * var_qsov_dn1);
        let eq111_e1484_d_n2: f64 = (p.p7 * var_qsov_dn2);
        let eq111_e1484_d_n10: f64 = (p.p7 * var_qsov_dn10);
        let eq111_e1484_q: f64 = (p.p7 * eq111_e1483_q);
        (eq111_e1484, eq111_e1484_d_n1, eq111_e1484_d_n2, eq111_e1484_d_n10, eq111_e1484_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node3(
            Some(nodes[10]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (eq111_e1486_d_n1),
            nodes[2],
            multiplicity * (eq111_e1486_d_n2),
            nodes[10],
            multiplicity * (eq111_e1486_d_n10),
        );
        let (eq112_e1493, eq112_e1493_d_n0, eq112_e1493_d_n1, eq112_e1493_d_n2, eq112_e1493_d_n10, eq112_e1493_q,) = {
    if (var_guard535 != 0.0) {
        let eq112_e1490_q: f64 = var_qdov;
        let eq112_e1491: f64 = (p.p7 * var_qdov);
        let eq112_e1491_d_n0: f64 = (p.p7 * var_qdov_dn0);
        let eq112_e1491_d_n1: f64 = (p.p7 * var_qdov_dn1);
        let eq112_e1491_d_n2: f64 = (p.p7 * var_qdov_dn2);
        let eq112_e1491_d_n10: f64 = (p.p7 * var_qdov_dn10);
        let eq112_e1491_q: f64 = (p.p7 * eq112_e1490_q);
        (eq112_e1491, eq112_e1491_d_n0, eq112_e1491_d_n1, eq112_e1491_d_n2, eq112_e1491_d_n10, eq112_e1491_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[10]),
            Some(nodes[0]),
            &[
                GeneratedDerivative::node(nodes[0], multiplicity * (eq112_e1493_d_n0)),
                GeneratedDerivative::node(nodes[1], multiplicity * (eq112_e1493_d_n1)),
                GeneratedDerivative::node(nodes[2], multiplicity * (eq112_e1493_d_n2)),
                GeneratedDerivative::node(nodes[10], multiplicity * (eq112_e1493_d_n10)),
            ],
        );
        let (eq113_e1501, eq113_e1501_d_n1, eq113_e1501_d_n2, eq113_e1501_d_n10, eq113_e1501_q,) = {
    if (var_guard535 == 0.0) {
        let eq113_e1498_q: f64 = var_qsov;
        let eq113_e1499: f64 = (p.p7 * var_qsov);
        let eq113_e1499_d_n1: f64 = (p.p7 * var_qsov_dn1);
        let eq113_e1499_d_n2: f64 = (p.p7 * var_qsov_dn2);
        let eq113_e1499_d_n10: f64 = (p.p7 * var_qsov_dn10);
        let eq113_e1499_q: f64 = (p.p7 * eq113_e1498_q);
        (eq113_e1499, eq113_e1499_d_n1, eq113_e1499_d_n2, eq113_e1499_d_n10, eq113_e1499_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node3(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (eq113_e1501_d_n1),
            nodes[2],
            multiplicity * (eq113_e1501_d_n2),
            nodes[10],
            multiplicity * (eq113_e1501_d_n10),
        );
        let (eq114_e1509, eq114_e1509_d_n0, eq114_e1509_d_n1, eq114_e1509_d_n2, eq114_e1509_d_n10, eq114_e1509_q,) = {
    if (var_guard535 == 0.0) {
        let eq114_e1506_q: f64 = var_qdov;
        let eq114_e1507: f64 = (p.p7 * var_qdov);
        let eq114_e1507_d_n0: f64 = (p.p7 * var_qdov_dn0);
        let eq114_e1507_d_n1: f64 = (p.p7 * var_qdov_dn1);
        let eq114_e1507_d_n2: f64 = (p.p7 * var_qdov_dn2);
        let eq114_e1507_d_n10: f64 = (p.p7 * var_qdov_dn10);
        let eq114_e1507_q: f64 = (p.p7 * eq114_e1506_q);
        (eq114_e1507, eq114_e1507_d_n0, eq114_e1507_d_n1, eq114_e1507_d_n2, eq114_e1507_d_n10, eq114_e1507_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[1]),
            Some(nodes[0]),
            &[
                GeneratedDerivative::node(nodes[0], multiplicity * (eq114_e1509_d_n0)),
                GeneratedDerivative::node(nodes[1], multiplicity * (eq114_e1509_d_n1)),
                GeneratedDerivative::node(nodes[2], multiplicity * (eq114_e1509_d_n2)),
                GeneratedDerivative::node(nodes[10], multiplicity * (eq114_e1509_d_n10)),
            ],
        );
        let eq115_e1512_q: f64 = var_qdsov;
        let eq115_e1513: f64 = (p.p7 * var_qdsov);
        let eq115_e1513_d_n0: f64 = (p.p7 * var_qdsov_dn0);
        let eq115_e1513_d_n2: f64 = (p.p7 * var_qdsov_dn2);
        let eq115_e1513_q: f64 = (p.p7 * eq115_e1512_q);
        stamper.stamp_current_reactive_node2(
            Some(nodes[0]),
            Some(nodes[2]),
            nodes[0],
            multiplicity * (eq115_e1513_d_n0),
            nodes[2],
            multiplicity * (eq115_e1513_d_n2),
        );
        let eq116_e1516_q: f64 = var_qbdov;
        let eq116_e1517: f64 = (p.p7 * var_qbdov);
        let eq116_e1517_d_n0: f64 = (p.p7 * var_qbdov_dn0);
        let eq116_e1517_d_n3: f64 = (p.p7 * var_qbdov_dn3);
        let eq116_e1517_q: f64 = (p.p7 * eq116_e1516_q);
        stamper.stamp_current_reactive_node2(
            Some(nodes[3]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (eq116_e1517_d_n0),
            nodes[3],
            multiplicity * (eq116_e1517_d_n3),
        );
        let eq117_e1520_q: f64 = var_qbsov;
        let eq117_e1521: f64 = (p.p7 * var_qbsov);
        let eq117_e1521_d_n2: f64 = (p.p7 * var_qbsov_dn2);
        let eq117_e1521_d_n3: f64 = (p.p7 * var_qbsov_dn3);
        let eq117_e1521_q: f64 = (p.p7 * eq117_e1520_q);
        stamper.stamp_current_reactive_node2(
            Some(nodes[3]),
            Some(nodes[2]),
            nodes[2],
            multiplicity * (eq117_e1521_d_n2),
            nodes[3],
            multiplicity * (eq117_e1521_d_n3),
        );
        let eq118_e1524_q: f64 = var_qbgov;
        let eq118_e1525: f64 = (p.p7 * var_qbgov);
        let eq118_e1525_d_n1: f64 = (p.p7 * var_qbgov_dn1);
        let eq118_e1525_d_n3: f64 = (p.p7 * var_qbgov_dn3);
        let eq118_e1525_q: f64 = (p.p7 * eq118_e1524_q);
        stamper.stamp_current_reactive_node2(
            Some(nodes[3]),
            Some(nodes[1]),
            nodes[1],
            multiplicity * (eq118_e1525_d_n1),
            nodes[3],
            multiplicity * (eq118_e1525_d_n3),
        );
        let eq119_e1529: f64 = (p.p250 * var_qgint);
        let eq119_e1529_d_n0: f64 = (p.p250 * var_qgint_dn0);
        let eq119_e1529_d_n1: f64 = (p.p250 * var_qgint_dn1);
        let eq119_e1529_d_n2: f64 = (p.p250 * var_qgint_dn2);
        let eq119_e1529_d_n3: f64 = (p.p250 * var_qgint_dn3);
        let eq119_e1529_d_n4: f64 = (p.p250 * var_qgint_dn4);
        let eq119_e1529_d_n5: f64 = (p.p250 * var_qgint_dn5);
        let eq119_e1529_d_n6: f64 = (p.p250 * var_qgint_dn6);
        let eq119_e1529_d_n7: f64 = (p.p250 * var_qgint_dn7);
        let eq119_e1529_d_n8: f64 = (p.p250 * var_qgint_dn8);
        let eq119_e1529_d_n9: f64 = (p.p250 * var_qgint_dn9);
        let eq119_e1529_d_n12: f64 = (p.p250 * var_qgint_dn12);
        let eq119_e1529_d_n14: f64 = (p.p250 * var_qgint_dn14);
        let eq119_e1529_d_n15: f64 = (p.p250 * var_qgint_dn15);
        let eq119_e1529_d_n16: f64 = (p.p250 * var_qgint_dn16);
        let eq119_e1529_d_n17: f64 = (p.p250 * var_qgint_dn17);
        let eq119_e1529_d_n18: f64 = (p.p250 * var_qgint_dn18);
        let eq119_e1529_d_n19: f64 = (p.p250 * var_qgint_dn19);
        let eq119_e1529_d_n20: f64 = (p.p250 * var_qgint_dn20);
        let eq119_e1529_d_n21: f64 = (p.p250 * var_qgint_dn21);
        let eq119_e1529_d_n22: f64 = (p.p250 * var_qgint_dn22);
        let eq119_e1530_q: f64 = eq119_e1529;
        let eq119_e1531: f64 = (p.p7 * eq119_e1529);
        let eq119_e1531_d_n0: f64 = (p.p7 * eq119_e1529_d_n0);
        let eq119_e1531_d_n1: f64 = (p.p7 * eq119_e1529_d_n1);
        let eq119_e1531_d_n2: f64 = (p.p7 * eq119_e1529_d_n2);
        let eq119_e1531_d_n3: f64 = (p.p7 * eq119_e1529_d_n3);
        let eq119_e1531_d_n4: f64 = (p.p7 * eq119_e1529_d_n4);
        let eq119_e1531_d_n5: f64 = (p.p7 * eq119_e1529_d_n5);
        let eq119_e1531_d_n6: f64 = (p.p7 * eq119_e1529_d_n6);
        let eq119_e1531_d_n7: f64 = (p.p7 * eq119_e1529_d_n7);
        let eq119_e1531_d_n8: f64 = (p.p7 * eq119_e1529_d_n8);
        let eq119_e1531_d_n9: f64 = (p.p7 * eq119_e1529_d_n9);
        let eq119_e1531_d_n12: f64 = (p.p7 * eq119_e1529_d_n12);
        let eq119_e1531_d_n14: f64 = (p.p7 * eq119_e1529_d_n14);
        let eq119_e1531_d_n15: f64 = (p.p7 * eq119_e1529_d_n15);
        let eq119_e1531_d_n16: f64 = (p.p7 * eq119_e1529_d_n16);
        let eq119_e1531_d_n17: f64 = (p.p7 * eq119_e1529_d_n17);
        let eq119_e1531_d_n18: f64 = (p.p7 * eq119_e1529_d_n18);
        let eq119_e1531_d_n19: f64 = (p.p7 * eq119_e1529_d_n19);
        let eq119_e1531_d_n20: f64 = (p.p7 * eq119_e1529_d_n20);
        let eq119_e1531_d_n21: f64 = (p.p7 * eq119_e1529_d_n21);
        let eq119_e1531_d_n22: f64 = (p.p7 * eq119_e1529_d_n22);
        let eq119_e1531_q: f64 = (p.p7 * eq119_e1530_q);
        let eq119_reactive_node_derivatives: [f64; 23] = [eq119_e1531_d_n0, eq119_e1531_d_n1, eq119_e1531_d_n2, eq119_e1531_d_n3, eq119_e1531_d_n4, eq119_e1531_d_n5, eq119_e1531_d_n6, eq119_e1531_d_n7, eq119_e1531_d_n8, eq119_e1531_d_n9, 0.0, 0.0, eq119_e1531_d_n12, 0.0, eq119_e1531_d_n14, eq119_e1531_d_n15, eq119_e1531_d_n16, eq119_e1531_d_n17, eq119_e1531_d_n18, eq119_e1531_d_n19, eq119_e1531_d_n20, eq119_e1531_d_n21, eq119_e1531_d_n22];
        let eq119_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            nodes,
            &eq119_reactive_node_derivatives,
            branches,
            &eq119_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq120_e1540, eq120_e1540_d_n0, eq120_e1540_d_n1, eq120_e1540_d_n2, eq120_e1540_d_n3, eq120_e1540_d_n4, eq120_e1540_d_n5, eq120_e1540_d_n6, eq120_e1540_d_n7, eq120_e1540_d_n8, eq120_e1540_d_n9, eq120_e1540_d_n12, eq120_e1540_d_n14, eq120_e1540_d_n15, eq120_e1540_d_n16, eq120_e1540_d_n17, eq120_e1540_d_n18, eq120_e1540_d_n19, eq120_e1540_d_n20, eq120_e1540_d_n21, eq120_e1540_d_n22, eq120_e1540_q,) = {
    if ((var_guard536 != 0.0) && (var_guard537 != 0.0)) {
        let eq120_e1537_q: f64 = var_qd_fp1;
        let eq120_e1538: f64 = (p.p7 * var_qd_fp1);
        let eq120_e1538_d_n0: f64 = (p.p7 * var_qd_fp1_dn0);
        let eq120_e1538_d_n1: f64 = (p.p7 * var_qd_fp1_dn1);
        let eq120_e1538_d_n2: f64 = (p.p7 * var_qd_fp1_dn2);
        let eq120_e1538_d_n3: f64 = (p.p7 * var_qd_fp1_dn3);
        let eq120_e1538_d_n4: f64 = (p.p7 * var_qd_fp1_dn4);
        let eq120_e1538_d_n5: f64 = (p.p7 * var_qd_fp1_dn5);
        let eq120_e1538_d_n6: f64 = (p.p7 * var_qd_fp1_dn6);
        let eq120_e1538_d_n7: f64 = (p.p7 * var_qd_fp1_dn7);
        let eq120_e1538_d_n8: f64 = (p.p7 * var_qd_fp1_dn8);
        let eq120_e1538_d_n9: f64 = (p.p7 * var_qd_fp1_dn9);
        let eq120_e1538_d_n12: f64 = (p.p7 * var_qd_fp1_dn12);
        let eq120_e1538_d_n14: f64 = (p.p7 * var_qd_fp1_dn14);
        let eq120_e1538_d_n15: f64 = (p.p7 * var_qd_fp1_dn15);
        let eq120_e1538_d_n16: f64 = (p.p7 * var_qd_fp1_dn16);
        let eq120_e1538_d_n17: f64 = (p.p7 * var_qd_fp1_dn17);
        let eq120_e1538_d_n18: f64 = (p.p7 * var_qd_fp1_dn18);
        let eq120_e1538_d_n19: f64 = (p.p7 * var_qd_fp1_dn19);
        let eq120_e1538_d_n20: f64 = (p.p7 * var_qd_fp1_dn20);
        let eq120_e1538_d_n21: f64 = (p.p7 * var_qd_fp1_dn21);
        let eq120_e1538_d_n22: f64 = (p.p7 * var_qd_fp1_dn22);
        let eq120_e1538_q: f64 = (p.p7 * eq120_e1537_q);
        (eq120_e1538, eq120_e1538_d_n0, eq120_e1538_d_n1, eq120_e1538_d_n2, eq120_e1538_d_n3, eq120_e1538_d_n4, eq120_e1538_d_n5, eq120_e1538_d_n6, eq120_e1538_d_n7, eq120_e1538_d_n8, eq120_e1538_d_n9, eq120_e1538_d_n12, eq120_e1538_d_n14, eq120_e1538_d_n15, eq120_e1538_d_n16, eq120_e1538_d_n17, eq120_e1538_d_n18, eq120_e1538_d_n19, eq120_e1538_d_n20, eq120_e1538_d_n21, eq120_e1538_d_n22, eq120_e1538_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq120_reactive_node_derivatives: [f64; 23] = [eq120_e1540_d_n0, eq120_e1540_d_n1, eq120_e1540_d_n2, eq120_e1540_d_n3, eq120_e1540_d_n4, eq120_e1540_d_n5, eq120_e1540_d_n6, eq120_e1540_d_n7, eq120_e1540_d_n8, eq120_e1540_d_n9, 0.0, 0.0, eq120_e1540_d_n12, 0.0, eq120_e1540_d_n14, eq120_e1540_d_n15, eq120_e1540_d_n16, eq120_e1540_d_n17, eq120_e1540_d_n18, eq120_e1540_d_n19, eq120_e1540_d_n20, eq120_e1540_d_n21, eq120_e1540_d_n22];
        let eq120_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[15]),
            Some(nodes[7]),
            nodes,
            &eq120_reactive_node_derivatives,
            branches,
            &eq120_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_guard536: f64,
        var_guard537: f64,
        var_guard538: f64,
        var_guard539: f64,
        var_guard540: f64,
        var_qd_fp1: f64,
        var_qd_fp1_dn0: f64,
        var_qd_fp1_dn1: f64,
        var_qd_fp1_dn12: f64,
        var_qd_fp1_dn14: f64,
        var_qd_fp1_dn15: f64,
        var_qd_fp1_dn16: f64,
        var_qd_fp1_dn17: f64,
        var_qd_fp1_dn18: f64,
        var_qd_fp1_dn19: f64,
        var_qd_fp1_dn2: f64,
        var_qd_fp1_dn20: f64,
        var_qd_fp1_dn21: f64,
        var_qd_fp1_dn22: f64,
        var_qd_fp1_dn3: f64,
        var_qd_fp1_dn4: f64,
        var_qd_fp1_dn5: f64,
        var_qd_fp1_dn6: f64,
        var_qd_fp1_dn7: f64,
        var_qd_fp1_dn8: f64,
        var_qd_fp1_dn9: f64,
        var_qg_fp1: f64,
        var_qg_fp1_dn0: f64,
        var_qg_fp1_dn1: f64,
        var_qg_fp1_dn12: f64,
        var_qg_fp1_dn14: f64,
        var_qg_fp1_dn15: f64,
        var_qg_fp1_dn16: f64,
        var_qg_fp1_dn17: f64,
        var_qg_fp1_dn18: f64,
        var_qg_fp1_dn19: f64,
        var_qg_fp1_dn2: f64,
        var_qg_fp1_dn20: f64,
        var_qg_fp1_dn21: f64,
        var_qg_fp1_dn22: f64,
        var_qg_fp1_dn3: f64,
        var_qg_fp1_dn4: f64,
        var_qg_fp1_dn5: f64,
        var_qg_fp1_dn6: f64,
        var_qg_fp1_dn7: f64,
        var_qg_fp1_dn8: f64,
        var_qg_fp1_dn9: f64,
    ) {
        let (eq121_e1551, eq121_e1551_d_n0, eq121_e1551_d_n1, eq121_e1551_d_n2, eq121_e1551_d_n3, eq121_e1551_d_n4, eq121_e1551_d_n5, eq121_e1551_d_n6, eq121_e1551_d_n7, eq121_e1551_d_n8, eq121_e1551_d_n9, eq121_e1551_d_n12, eq121_e1551_d_n14, eq121_e1551_d_n15, eq121_e1551_d_n16, eq121_e1551_d_n17, eq121_e1551_d_n18, eq121_e1551_d_n19, eq121_e1551_d_n20, eq121_e1551_d_n21, eq121_e1551_d_n22, eq121_e1551_q,) = {
    if (((var_guard536 != 0.0) && (var_guard537 != 0.0)) && (var_guard538 != 0.0)) {
        let eq121_e1548_q: f64 = var_qg_fp1;
        let eq121_e1549: f64 = (p.p7 * var_qg_fp1);
        let eq121_e1549_d_n0: f64 = (p.p7 * var_qg_fp1_dn0);
        let eq121_e1549_d_n1: f64 = (p.p7 * var_qg_fp1_dn1);
        let eq121_e1549_d_n2: f64 = (p.p7 * var_qg_fp1_dn2);
        let eq121_e1549_d_n3: f64 = (p.p7 * var_qg_fp1_dn3);
        let eq121_e1549_d_n4: f64 = (p.p7 * var_qg_fp1_dn4);
        let eq121_e1549_d_n5: f64 = (p.p7 * var_qg_fp1_dn5);
        let eq121_e1549_d_n6: f64 = (p.p7 * var_qg_fp1_dn6);
        let eq121_e1549_d_n7: f64 = (p.p7 * var_qg_fp1_dn7);
        let eq121_e1549_d_n8: f64 = (p.p7 * var_qg_fp1_dn8);
        let eq121_e1549_d_n9: f64 = (p.p7 * var_qg_fp1_dn9);
        let eq121_e1549_d_n12: f64 = (p.p7 * var_qg_fp1_dn12);
        let eq121_e1549_d_n14: f64 = (p.p7 * var_qg_fp1_dn14);
        let eq121_e1549_d_n15: f64 = (p.p7 * var_qg_fp1_dn15);
        let eq121_e1549_d_n16: f64 = (p.p7 * var_qg_fp1_dn16);
        let eq121_e1549_d_n17: f64 = (p.p7 * var_qg_fp1_dn17);
        let eq121_e1549_d_n18: f64 = (p.p7 * var_qg_fp1_dn18);
        let eq121_e1549_d_n19: f64 = (p.p7 * var_qg_fp1_dn19);
        let eq121_e1549_d_n20: f64 = (p.p7 * var_qg_fp1_dn20);
        let eq121_e1549_d_n21: f64 = (p.p7 * var_qg_fp1_dn21);
        let eq121_e1549_d_n22: f64 = (p.p7 * var_qg_fp1_dn22);
        let eq121_e1549_q: f64 = (p.p7 * eq121_e1548_q);
        (eq121_e1549, eq121_e1549_d_n0, eq121_e1549_d_n1, eq121_e1549_d_n2, eq121_e1549_d_n3, eq121_e1549_d_n4, eq121_e1549_d_n5, eq121_e1549_d_n6, eq121_e1549_d_n7, eq121_e1549_d_n8, eq121_e1549_d_n9, eq121_e1549_d_n12, eq121_e1549_d_n14, eq121_e1549_d_n15, eq121_e1549_d_n16, eq121_e1549_d_n17, eq121_e1549_d_n18, eq121_e1549_d_n19, eq121_e1549_d_n20, eq121_e1549_d_n21, eq121_e1549_d_n22, eq121_e1549_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq121_reactive_node_derivatives: [f64; 23] = [eq121_e1551_d_n0, eq121_e1551_d_n1, eq121_e1551_d_n2, eq121_e1551_d_n3, eq121_e1551_d_n4, eq121_e1551_d_n5, eq121_e1551_d_n6, eq121_e1551_d_n7, eq121_e1551_d_n8, eq121_e1551_d_n9, 0.0, 0.0, eq121_e1551_d_n12, 0.0, eq121_e1551_d_n14, eq121_e1551_d_n15, eq121_e1551_d_n16, eq121_e1551_d_n17, eq121_e1551_d_n18, eq121_e1551_d_n19, eq121_e1551_d_n20, eq121_e1551_d_n21, eq121_e1551_d_n22];
        let eq121_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq121_reactive_node_derivatives,
            branches,
            &eq121_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq122_e1564, eq122_e1564_d_n0, eq122_e1564_d_n1, eq122_e1564_d_n2, eq122_e1564_d_n3, eq122_e1564_d_n4, eq122_e1564_d_n5, eq122_e1564_d_n6, eq122_e1564_d_n7, eq122_e1564_d_n8, eq122_e1564_d_n9, eq122_e1564_d_n12, eq122_e1564_d_n14, eq122_e1564_d_n15, eq122_e1564_d_n16, eq122_e1564_d_n17, eq122_e1564_d_n18, eq122_e1564_d_n19, eq122_e1564_d_n20, eq122_e1564_d_n21, eq122_e1564_d_n22, eq122_e1564_q,) = {
    if (((var_guard536 != 0.0) && (var_guard537 != 0.0)) && (var_guard538 != 0.0)) {
        let eq122_e1559_q: f64 = var_qg_fp1;
        let eq122_e1560: f64 = (p.p7 * var_qg_fp1);
        let eq122_e1560_d_n0: f64 = (p.p7 * var_qg_fp1_dn0);
        let eq122_e1560_d_n1: f64 = (p.p7 * var_qg_fp1_dn1);
        let eq122_e1560_d_n2: f64 = (p.p7 * var_qg_fp1_dn2);
        let eq122_e1560_d_n3: f64 = (p.p7 * var_qg_fp1_dn3);
        let eq122_e1560_d_n4: f64 = (p.p7 * var_qg_fp1_dn4);
        let eq122_e1560_d_n5: f64 = (p.p7 * var_qg_fp1_dn5);
        let eq122_e1560_d_n6: f64 = (p.p7 * var_qg_fp1_dn6);
        let eq122_e1560_d_n7: f64 = (p.p7 * var_qg_fp1_dn7);
        let eq122_e1560_d_n8: f64 = (p.p7 * var_qg_fp1_dn8);
        let eq122_e1560_d_n9: f64 = (p.p7 * var_qg_fp1_dn9);
        let eq122_e1560_d_n12: f64 = (p.p7 * var_qg_fp1_dn12);
        let eq122_e1560_d_n14: f64 = (p.p7 * var_qg_fp1_dn14);
        let eq122_e1560_d_n15: f64 = (p.p7 * var_qg_fp1_dn15);
        let eq122_e1560_d_n16: f64 = (p.p7 * var_qg_fp1_dn16);
        let eq122_e1560_d_n17: f64 = (p.p7 * var_qg_fp1_dn17);
        let eq122_e1560_d_n18: f64 = (p.p7 * var_qg_fp1_dn18);
        let eq122_e1560_d_n19: f64 = (p.p7 * var_qg_fp1_dn19);
        let eq122_e1560_d_n20: f64 = (p.p7 * var_qg_fp1_dn20);
        let eq122_e1560_d_n21: f64 = (p.p7 * var_qg_fp1_dn21);
        let eq122_e1560_d_n22: f64 = (p.p7 * var_qg_fp1_dn22);
        let eq122_e1560_q: f64 = (p.p7 * eq122_e1559_q);
        let eq122_e1562: f64 = (eq122_e1560 * p.p246);
        let eq122_e1562_d_n0: f64 = (eq122_e1560_d_n0 * p.p246);
        let eq122_e1562_d_n1: f64 = (eq122_e1560_d_n1 * p.p246);
        let eq122_e1562_d_n2: f64 = (eq122_e1560_d_n2 * p.p246);
        let eq122_e1562_d_n3: f64 = (eq122_e1560_d_n3 * p.p246);
        let eq122_e1562_d_n4: f64 = (eq122_e1560_d_n4 * p.p246);
        let eq122_e1562_d_n5: f64 = (eq122_e1560_d_n5 * p.p246);
        let eq122_e1562_d_n6: f64 = (eq122_e1560_d_n6 * p.p246);
        let eq122_e1562_d_n7: f64 = (eq122_e1560_d_n7 * p.p246);
        let eq122_e1562_d_n8: f64 = (eq122_e1560_d_n8 * p.p246);
        let eq122_e1562_d_n9: f64 = (eq122_e1560_d_n9 * p.p246);
        let eq122_e1562_d_n12: f64 = (eq122_e1560_d_n12 * p.p246);
        let eq122_e1562_d_n14: f64 = (eq122_e1560_d_n14 * p.p246);
        let eq122_e1562_d_n15: f64 = (eq122_e1560_d_n15 * p.p246);
        let eq122_e1562_d_n16: f64 = (eq122_e1560_d_n16 * p.p246);
        let eq122_e1562_d_n17: f64 = (eq122_e1560_d_n17 * p.p246);
        let eq122_e1562_d_n18: f64 = (eq122_e1560_d_n18 * p.p246);
        let eq122_e1562_d_n19: f64 = (eq122_e1560_d_n19 * p.p246);
        let eq122_e1562_d_n20: f64 = (eq122_e1560_d_n20 * p.p246);
        let eq122_e1562_d_n21: f64 = (eq122_e1560_d_n21 * p.p246);
        let eq122_e1562_d_n22: f64 = (eq122_e1560_d_n22 * p.p246);
        let eq122_e1562_q: f64 = (eq122_e1560_q * p.p246);
        (eq122_e1562, eq122_e1562_d_n0, eq122_e1562_d_n1, eq122_e1562_d_n2, eq122_e1562_d_n3, eq122_e1562_d_n4, eq122_e1562_d_n5, eq122_e1562_d_n6, eq122_e1562_d_n7, eq122_e1562_d_n8, eq122_e1562_d_n9, eq122_e1562_d_n12, eq122_e1562_d_n14, eq122_e1562_d_n15, eq122_e1562_d_n16, eq122_e1562_d_n17, eq122_e1562_d_n18, eq122_e1562_d_n19, eq122_e1562_d_n20, eq122_e1562_d_n21, eq122_e1562_d_n22, eq122_e1562_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq122_reactive_node_derivatives: [f64; 23] = [eq122_e1564_d_n0, eq122_e1564_d_n1, eq122_e1564_d_n2, eq122_e1564_d_n3, eq122_e1564_d_n4, eq122_e1564_d_n5, eq122_e1564_d_n6, eq122_e1564_d_n7, eq122_e1564_d_n8, eq122_e1564_d_n9, 0.0, 0.0, eq122_e1564_d_n12, 0.0, eq122_e1564_d_n14, eq122_e1564_d_n15, eq122_e1564_d_n16, eq122_e1564_d_n17, eq122_e1564_d_n18, eq122_e1564_d_n19, eq122_e1564_d_n20, eq122_e1564_d_n21, eq122_e1564_d_n22];
        let eq122_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq122_reactive_node_derivatives,
            branches,
            &eq122_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq123_e1576, eq123_e1576_d_n0, eq123_e1576_d_n1, eq123_e1576_d_n2, eq123_e1576_d_n3, eq123_e1576_d_n4, eq123_e1576_d_n5, eq123_e1576_d_n6, eq123_e1576_d_n7, eq123_e1576_d_n8, eq123_e1576_d_n9, eq123_e1576_d_n12, eq123_e1576_d_n14, eq123_e1576_d_n15, eq123_e1576_d_n16, eq123_e1576_d_n17, eq123_e1576_d_n18, eq123_e1576_d_n19, eq123_e1576_d_n20, eq123_e1576_d_n21, eq123_e1576_d_n22, eq123_e1576_q,) = {
    if (((var_guard536 != 0.0) && (var_guard537 != 0.0)) && (var_guard538 == 0.0)) {
        let eq123_e1573_q: f64 = var_qg_fp1;
        let eq123_e1574: f64 = (p.p7 * var_qg_fp1);
        let eq123_e1574_d_n0: f64 = (p.p7 * var_qg_fp1_dn0);
        let eq123_e1574_d_n1: f64 = (p.p7 * var_qg_fp1_dn1);
        let eq123_e1574_d_n2: f64 = (p.p7 * var_qg_fp1_dn2);
        let eq123_e1574_d_n3: f64 = (p.p7 * var_qg_fp1_dn3);
        let eq123_e1574_d_n4: f64 = (p.p7 * var_qg_fp1_dn4);
        let eq123_e1574_d_n5: f64 = (p.p7 * var_qg_fp1_dn5);
        let eq123_e1574_d_n6: f64 = (p.p7 * var_qg_fp1_dn6);
        let eq123_e1574_d_n7: f64 = (p.p7 * var_qg_fp1_dn7);
        let eq123_e1574_d_n8: f64 = (p.p7 * var_qg_fp1_dn8);
        let eq123_e1574_d_n9: f64 = (p.p7 * var_qg_fp1_dn9);
        let eq123_e1574_d_n12: f64 = (p.p7 * var_qg_fp1_dn12);
        let eq123_e1574_d_n14: f64 = (p.p7 * var_qg_fp1_dn14);
        let eq123_e1574_d_n15: f64 = (p.p7 * var_qg_fp1_dn15);
        let eq123_e1574_d_n16: f64 = (p.p7 * var_qg_fp1_dn16);
        let eq123_e1574_d_n17: f64 = (p.p7 * var_qg_fp1_dn17);
        let eq123_e1574_d_n18: f64 = (p.p7 * var_qg_fp1_dn18);
        let eq123_e1574_d_n19: f64 = (p.p7 * var_qg_fp1_dn19);
        let eq123_e1574_d_n20: f64 = (p.p7 * var_qg_fp1_dn20);
        let eq123_e1574_d_n21: f64 = (p.p7 * var_qg_fp1_dn21);
        let eq123_e1574_d_n22: f64 = (p.p7 * var_qg_fp1_dn22);
        let eq123_e1574_q: f64 = (p.p7 * eq123_e1573_q);
        (eq123_e1574, eq123_e1574_d_n0, eq123_e1574_d_n1, eq123_e1574_d_n2, eq123_e1574_d_n3, eq123_e1574_d_n4, eq123_e1574_d_n5, eq123_e1574_d_n6, eq123_e1574_d_n7, eq123_e1574_d_n8, eq123_e1574_d_n9, eq123_e1574_d_n12, eq123_e1574_d_n14, eq123_e1574_d_n15, eq123_e1574_d_n16, eq123_e1574_d_n17, eq123_e1574_d_n18, eq123_e1574_d_n19, eq123_e1574_d_n20, eq123_e1574_d_n21, eq123_e1574_d_n22, eq123_e1574_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq123_reactive_node_derivatives: [f64; 23] = [eq123_e1576_d_n0, eq123_e1576_d_n1, eq123_e1576_d_n2, eq123_e1576_d_n3, eq123_e1576_d_n4, eq123_e1576_d_n5, eq123_e1576_d_n6, eq123_e1576_d_n7, eq123_e1576_d_n8, eq123_e1576_d_n9, 0.0, 0.0, eq123_e1576_d_n12, 0.0, eq123_e1576_d_n14, eq123_e1576_d_n15, eq123_e1576_d_n16, eq123_e1576_d_n17, eq123_e1576_d_n18, eq123_e1576_d_n19, eq123_e1576_d_n20, eq123_e1576_d_n21, eq123_e1576_d_n22];
        let eq123_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq123_reactive_node_derivatives,
            branches,
            &eq123_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq124_e1590, eq124_e1590_d_n0, eq124_e1590_d_n1, eq124_e1590_d_n2, eq124_e1590_d_n3, eq124_e1590_d_n4, eq124_e1590_d_n5, eq124_e1590_d_n6, eq124_e1590_d_n7, eq124_e1590_d_n8, eq124_e1590_d_n9, eq124_e1590_d_n12, eq124_e1590_d_n14, eq124_e1590_d_n15, eq124_e1590_d_n16, eq124_e1590_d_n17, eq124_e1590_d_n18, eq124_e1590_d_n19, eq124_e1590_d_n20, eq124_e1590_d_n21, eq124_e1590_d_n22, eq124_e1590_q,) = {
    if (((var_guard536 != 0.0) && (var_guard537 != 0.0)) && (var_guard538 == 0.0)) {
        let eq124_e1585_q: f64 = var_qg_fp1;
        let eq124_e1586: f64 = (p.p7 * var_qg_fp1);
        let eq124_e1586_d_n0: f64 = (p.p7 * var_qg_fp1_dn0);
        let eq124_e1586_d_n1: f64 = (p.p7 * var_qg_fp1_dn1);
        let eq124_e1586_d_n2: f64 = (p.p7 * var_qg_fp1_dn2);
        let eq124_e1586_d_n3: f64 = (p.p7 * var_qg_fp1_dn3);
        let eq124_e1586_d_n4: f64 = (p.p7 * var_qg_fp1_dn4);
        let eq124_e1586_d_n5: f64 = (p.p7 * var_qg_fp1_dn5);
        let eq124_e1586_d_n6: f64 = (p.p7 * var_qg_fp1_dn6);
        let eq124_e1586_d_n7: f64 = (p.p7 * var_qg_fp1_dn7);
        let eq124_e1586_d_n8: f64 = (p.p7 * var_qg_fp1_dn8);
        let eq124_e1586_d_n9: f64 = (p.p7 * var_qg_fp1_dn9);
        let eq124_e1586_d_n12: f64 = (p.p7 * var_qg_fp1_dn12);
        let eq124_e1586_d_n14: f64 = (p.p7 * var_qg_fp1_dn14);
        let eq124_e1586_d_n15: f64 = (p.p7 * var_qg_fp1_dn15);
        let eq124_e1586_d_n16: f64 = (p.p7 * var_qg_fp1_dn16);
        let eq124_e1586_d_n17: f64 = (p.p7 * var_qg_fp1_dn17);
        let eq124_e1586_d_n18: f64 = (p.p7 * var_qg_fp1_dn18);
        let eq124_e1586_d_n19: f64 = (p.p7 * var_qg_fp1_dn19);
        let eq124_e1586_d_n20: f64 = (p.p7 * var_qg_fp1_dn20);
        let eq124_e1586_d_n21: f64 = (p.p7 * var_qg_fp1_dn21);
        let eq124_e1586_d_n22: f64 = (p.p7 * var_qg_fp1_dn22);
        let eq124_e1586_q: f64 = (p.p7 * eq124_e1585_q);
        let eq124_e1588: f64 = (eq124_e1586 * p.p246);
        let eq124_e1588_d_n0: f64 = (eq124_e1586_d_n0 * p.p246);
        let eq124_e1588_d_n1: f64 = (eq124_e1586_d_n1 * p.p246);
        let eq124_e1588_d_n2: f64 = (eq124_e1586_d_n2 * p.p246);
        let eq124_e1588_d_n3: f64 = (eq124_e1586_d_n3 * p.p246);
        let eq124_e1588_d_n4: f64 = (eq124_e1586_d_n4 * p.p246);
        let eq124_e1588_d_n5: f64 = (eq124_e1586_d_n5 * p.p246);
        let eq124_e1588_d_n6: f64 = (eq124_e1586_d_n6 * p.p246);
        let eq124_e1588_d_n7: f64 = (eq124_e1586_d_n7 * p.p246);
        let eq124_e1588_d_n8: f64 = (eq124_e1586_d_n8 * p.p246);
        let eq124_e1588_d_n9: f64 = (eq124_e1586_d_n9 * p.p246);
        let eq124_e1588_d_n12: f64 = (eq124_e1586_d_n12 * p.p246);
        let eq124_e1588_d_n14: f64 = (eq124_e1586_d_n14 * p.p246);
        let eq124_e1588_d_n15: f64 = (eq124_e1586_d_n15 * p.p246);
        let eq124_e1588_d_n16: f64 = (eq124_e1586_d_n16 * p.p246);
        let eq124_e1588_d_n17: f64 = (eq124_e1586_d_n17 * p.p246);
        let eq124_e1588_d_n18: f64 = (eq124_e1586_d_n18 * p.p246);
        let eq124_e1588_d_n19: f64 = (eq124_e1586_d_n19 * p.p246);
        let eq124_e1588_d_n20: f64 = (eq124_e1586_d_n20 * p.p246);
        let eq124_e1588_d_n21: f64 = (eq124_e1586_d_n21 * p.p246);
        let eq124_e1588_d_n22: f64 = (eq124_e1586_d_n22 * p.p246);
        let eq124_e1588_q: f64 = (eq124_e1586_q * p.p246);
        (eq124_e1588, eq124_e1588_d_n0, eq124_e1588_d_n1, eq124_e1588_d_n2, eq124_e1588_d_n3, eq124_e1588_d_n4, eq124_e1588_d_n5, eq124_e1588_d_n6, eq124_e1588_d_n7, eq124_e1588_d_n8, eq124_e1588_d_n9, eq124_e1588_d_n12, eq124_e1588_d_n14, eq124_e1588_d_n15, eq124_e1588_d_n16, eq124_e1588_d_n17, eq124_e1588_d_n18, eq124_e1588_d_n19, eq124_e1588_d_n20, eq124_e1588_d_n21, eq124_e1588_d_n22, eq124_e1588_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq124_reactive_node_derivatives: [f64; 23] = [eq124_e1590_d_n0, eq124_e1590_d_n1, eq124_e1590_d_n2, eq124_e1590_d_n3, eq124_e1590_d_n4, eq124_e1590_d_n5, eq124_e1590_d_n6, eq124_e1590_d_n7, eq124_e1590_d_n8, eq124_e1590_d_n9, 0.0, 0.0, eq124_e1590_d_n12, 0.0, eq124_e1590_d_n14, eq124_e1590_d_n15, eq124_e1590_d_n16, eq124_e1590_d_n17, eq124_e1590_d_n18, eq124_e1590_d_n19, eq124_e1590_d_n20, eq124_e1590_d_n21, eq124_e1590_d_n22];
        let eq124_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq124_reactive_node_derivatives,
            branches,
            &eq124_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq125_e1601, eq125_e1601_d_n0, eq125_e1601_d_n1, eq125_e1601_d_n2, eq125_e1601_d_n3, eq125_e1601_d_n4, eq125_e1601_d_n5, eq125_e1601_d_n6, eq125_e1601_d_n7, eq125_e1601_d_n8, eq125_e1601_d_n9, eq125_e1601_d_n12, eq125_e1601_d_n14, eq125_e1601_d_n15, eq125_e1601_d_n16, eq125_e1601_d_n17, eq125_e1601_d_n18, eq125_e1601_d_n19, eq125_e1601_d_n20, eq125_e1601_d_n21, eq125_e1601_d_n22, eq125_e1601_q,) = {
    if ((var_guard536 != 0.0) && (var_guard537 != 0.0)) {
        let eq125_e1597: f64 = (p.p251 * var_qg_fp1);
        let eq125_e1597_d_n0: f64 = (p.p251 * var_qg_fp1_dn0);
        let eq125_e1597_d_n1: f64 = (p.p251 * var_qg_fp1_dn1);
        let eq125_e1597_d_n2: f64 = (p.p251 * var_qg_fp1_dn2);
        let eq125_e1597_d_n3: f64 = (p.p251 * var_qg_fp1_dn3);
        let eq125_e1597_d_n4: f64 = (p.p251 * var_qg_fp1_dn4);
        let eq125_e1597_d_n5: f64 = (p.p251 * var_qg_fp1_dn5);
        let eq125_e1597_d_n6: f64 = (p.p251 * var_qg_fp1_dn6);
        let eq125_e1597_d_n7: f64 = (p.p251 * var_qg_fp1_dn7);
        let eq125_e1597_d_n8: f64 = (p.p251 * var_qg_fp1_dn8);
        let eq125_e1597_d_n9: f64 = (p.p251 * var_qg_fp1_dn9);
        let eq125_e1597_d_n12: f64 = (p.p251 * var_qg_fp1_dn12);
        let eq125_e1597_d_n14: f64 = (p.p251 * var_qg_fp1_dn14);
        let eq125_e1597_d_n15: f64 = (p.p251 * var_qg_fp1_dn15);
        let eq125_e1597_d_n16: f64 = (p.p251 * var_qg_fp1_dn16);
        let eq125_e1597_d_n17: f64 = (p.p251 * var_qg_fp1_dn17);
        let eq125_e1597_d_n18: f64 = (p.p251 * var_qg_fp1_dn18);
        let eq125_e1597_d_n19: f64 = (p.p251 * var_qg_fp1_dn19);
        let eq125_e1597_d_n20: f64 = (p.p251 * var_qg_fp1_dn20);
        let eq125_e1597_d_n21: f64 = (p.p251 * var_qg_fp1_dn21);
        let eq125_e1597_d_n22: f64 = (p.p251 * var_qg_fp1_dn22);
        let eq125_e1598_q: f64 = eq125_e1597;
        let eq125_e1599: f64 = (p.p7 * eq125_e1597);
        let eq125_e1599_d_n0: f64 = (p.p7 * eq125_e1597_d_n0);
        let eq125_e1599_d_n1: f64 = (p.p7 * eq125_e1597_d_n1);
        let eq125_e1599_d_n2: f64 = (p.p7 * eq125_e1597_d_n2);
        let eq125_e1599_d_n3: f64 = (p.p7 * eq125_e1597_d_n3);
        let eq125_e1599_d_n4: f64 = (p.p7 * eq125_e1597_d_n4);
        let eq125_e1599_d_n5: f64 = (p.p7 * eq125_e1597_d_n5);
        let eq125_e1599_d_n6: f64 = (p.p7 * eq125_e1597_d_n6);
        let eq125_e1599_d_n7: f64 = (p.p7 * eq125_e1597_d_n7);
        let eq125_e1599_d_n8: f64 = (p.p7 * eq125_e1597_d_n8);
        let eq125_e1599_d_n9: f64 = (p.p7 * eq125_e1597_d_n9);
        let eq125_e1599_d_n12: f64 = (p.p7 * eq125_e1597_d_n12);
        let eq125_e1599_d_n14: f64 = (p.p7 * eq125_e1597_d_n14);
        let eq125_e1599_d_n15: f64 = (p.p7 * eq125_e1597_d_n15);
        let eq125_e1599_d_n16: f64 = (p.p7 * eq125_e1597_d_n16);
        let eq125_e1599_d_n17: f64 = (p.p7 * eq125_e1597_d_n17);
        let eq125_e1599_d_n18: f64 = (p.p7 * eq125_e1597_d_n18);
        let eq125_e1599_d_n19: f64 = (p.p7 * eq125_e1597_d_n19);
        let eq125_e1599_d_n20: f64 = (p.p7 * eq125_e1597_d_n20);
        let eq125_e1599_d_n21: f64 = (p.p7 * eq125_e1597_d_n21);
        let eq125_e1599_d_n22: f64 = (p.p7 * eq125_e1597_d_n22);
        let eq125_e1599_q: f64 = (p.p7 * eq125_e1598_q);
        (eq125_e1599, eq125_e1599_d_n0, eq125_e1599_d_n1, eq125_e1599_d_n2, eq125_e1599_d_n3, eq125_e1599_d_n4, eq125_e1599_d_n5, eq125_e1599_d_n6, eq125_e1599_d_n7, eq125_e1599_d_n8, eq125_e1599_d_n9, eq125_e1599_d_n12, eq125_e1599_d_n14, eq125_e1599_d_n15, eq125_e1599_d_n16, eq125_e1599_d_n17, eq125_e1599_d_n18, eq125_e1599_d_n19, eq125_e1599_d_n20, eq125_e1599_d_n21, eq125_e1599_d_n22, eq125_e1599_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq125_reactive_node_derivatives: [f64; 23] = [eq125_e1601_d_n0, eq125_e1601_d_n1, eq125_e1601_d_n2, eq125_e1601_d_n3, eq125_e1601_d_n4, eq125_e1601_d_n5, eq125_e1601_d_n6, eq125_e1601_d_n7, eq125_e1601_d_n8, eq125_e1601_d_n9, 0.0, 0.0, eq125_e1601_d_n12, 0.0, eq125_e1601_d_n14, eq125_e1601_d_n15, eq125_e1601_d_n16, eq125_e1601_d_n17, eq125_e1601_d_n18, eq125_e1601_d_n19, eq125_e1601_d_n20, eq125_e1601_d_n21, eq125_e1601_d_n22];
        let eq125_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            nodes,
            &eq125_reactive_node_derivatives,
            branches,
            &eq125_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq126_e1611, eq126_e1611_d_n0, eq126_e1611_d_n1, eq126_e1611_d_n2, eq126_e1611_d_n3, eq126_e1611_d_n4, eq126_e1611_d_n5, eq126_e1611_d_n6, eq126_e1611_d_n7, eq126_e1611_d_n8, eq126_e1611_d_n9, eq126_e1611_d_n12, eq126_e1611_d_n14, eq126_e1611_d_n15, eq126_e1611_d_n16, eq126_e1611_d_n17, eq126_e1611_d_n18, eq126_e1611_d_n19, eq126_e1611_d_n20, eq126_e1611_d_n21, eq126_e1611_d_n22, eq126_e1611_q,) = {
    if ((var_guard536 == 0.0) && (var_guard539 != 0.0)) {
        let eq126_e1608_q: f64 = var_qd_fp1;
        let eq126_e1609: f64 = (p.p7 * var_qd_fp1);
        let eq126_e1609_d_n0: f64 = (p.p7 * var_qd_fp1_dn0);
        let eq126_e1609_d_n1: f64 = (p.p7 * var_qd_fp1_dn1);
        let eq126_e1609_d_n2: f64 = (p.p7 * var_qd_fp1_dn2);
        let eq126_e1609_d_n3: f64 = (p.p7 * var_qd_fp1_dn3);
        let eq126_e1609_d_n4: f64 = (p.p7 * var_qd_fp1_dn4);
        let eq126_e1609_d_n5: f64 = (p.p7 * var_qd_fp1_dn5);
        let eq126_e1609_d_n6: f64 = (p.p7 * var_qd_fp1_dn6);
        let eq126_e1609_d_n7: f64 = (p.p7 * var_qd_fp1_dn7);
        let eq126_e1609_d_n8: f64 = (p.p7 * var_qd_fp1_dn8);
        let eq126_e1609_d_n9: f64 = (p.p7 * var_qd_fp1_dn9);
        let eq126_e1609_d_n12: f64 = (p.p7 * var_qd_fp1_dn12);
        let eq126_e1609_d_n14: f64 = (p.p7 * var_qd_fp1_dn14);
        let eq126_e1609_d_n15: f64 = (p.p7 * var_qd_fp1_dn15);
        let eq126_e1609_d_n16: f64 = (p.p7 * var_qd_fp1_dn16);
        let eq126_e1609_d_n17: f64 = (p.p7 * var_qd_fp1_dn17);
        let eq126_e1609_d_n18: f64 = (p.p7 * var_qd_fp1_dn18);
        let eq126_e1609_d_n19: f64 = (p.p7 * var_qd_fp1_dn19);
        let eq126_e1609_d_n20: f64 = (p.p7 * var_qd_fp1_dn20);
        let eq126_e1609_d_n21: f64 = (p.p7 * var_qd_fp1_dn21);
        let eq126_e1609_d_n22: f64 = (p.p7 * var_qd_fp1_dn22);
        let eq126_e1609_q: f64 = (p.p7 * eq126_e1608_q);
        (eq126_e1609, eq126_e1609_d_n0, eq126_e1609_d_n1, eq126_e1609_d_n2, eq126_e1609_d_n3, eq126_e1609_d_n4, eq126_e1609_d_n5, eq126_e1609_d_n6, eq126_e1609_d_n7, eq126_e1609_d_n8, eq126_e1609_d_n9, eq126_e1609_d_n12, eq126_e1609_d_n14, eq126_e1609_d_n15, eq126_e1609_d_n16, eq126_e1609_d_n17, eq126_e1609_d_n18, eq126_e1609_d_n19, eq126_e1609_d_n20, eq126_e1609_d_n21, eq126_e1609_d_n22, eq126_e1609_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq126_reactive_node_derivatives: [f64; 23] = [eq126_e1611_d_n0, eq126_e1611_d_n1, eq126_e1611_d_n2, eq126_e1611_d_n3, eq126_e1611_d_n4, eq126_e1611_d_n5, eq126_e1611_d_n6, eq126_e1611_d_n7, eq126_e1611_d_n8, eq126_e1611_d_n9, 0.0, 0.0, eq126_e1611_d_n12, 0.0, eq126_e1611_d_n14, eq126_e1611_d_n15, eq126_e1611_d_n16, eq126_e1611_d_n17, eq126_e1611_d_n18, eq126_e1611_d_n19, eq126_e1611_d_n20, eq126_e1611_d_n21, eq126_e1611_d_n22];
        let eq126_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[7]),
            nodes,
            &eq126_reactive_node_derivatives,
            branches,
            &eq126_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq127_e1623, eq127_e1623_d_n0, eq127_e1623_d_n1, eq127_e1623_d_n2, eq127_e1623_d_n3, eq127_e1623_d_n4, eq127_e1623_d_n5, eq127_e1623_d_n6, eq127_e1623_d_n7, eq127_e1623_d_n8, eq127_e1623_d_n9, eq127_e1623_d_n12, eq127_e1623_d_n14, eq127_e1623_d_n15, eq127_e1623_d_n16, eq127_e1623_d_n17, eq127_e1623_d_n18, eq127_e1623_d_n19, eq127_e1623_d_n20, eq127_e1623_d_n21, eq127_e1623_d_n22, eq127_e1623_q,) = {
    if (((var_guard536 == 0.0) && (var_guard539 != 0.0)) && (var_guard540 != 0.0)) {
        let eq127_e1620_q: f64 = var_qg_fp1;
        let eq127_e1621: f64 = (p.p7 * var_qg_fp1);
        let eq127_e1621_d_n0: f64 = (p.p7 * var_qg_fp1_dn0);
        let eq127_e1621_d_n1: f64 = (p.p7 * var_qg_fp1_dn1);
        let eq127_e1621_d_n2: f64 = (p.p7 * var_qg_fp1_dn2);
        let eq127_e1621_d_n3: f64 = (p.p7 * var_qg_fp1_dn3);
        let eq127_e1621_d_n4: f64 = (p.p7 * var_qg_fp1_dn4);
        let eq127_e1621_d_n5: f64 = (p.p7 * var_qg_fp1_dn5);
        let eq127_e1621_d_n6: f64 = (p.p7 * var_qg_fp1_dn6);
        let eq127_e1621_d_n7: f64 = (p.p7 * var_qg_fp1_dn7);
        let eq127_e1621_d_n8: f64 = (p.p7 * var_qg_fp1_dn8);
        let eq127_e1621_d_n9: f64 = (p.p7 * var_qg_fp1_dn9);
        let eq127_e1621_d_n12: f64 = (p.p7 * var_qg_fp1_dn12);
        let eq127_e1621_d_n14: f64 = (p.p7 * var_qg_fp1_dn14);
        let eq127_e1621_d_n15: f64 = (p.p7 * var_qg_fp1_dn15);
        let eq127_e1621_d_n16: f64 = (p.p7 * var_qg_fp1_dn16);
        let eq127_e1621_d_n17: f64 = (p.p7 * var_qg_fp1_dn17);
        let eq127_e1621_d_n18: f64 = (p.p7 * var_qg_fp1_dn18);
        let eq127_e1621_d_n19: f64 = (p.p7 * var_qg_fp1_dn19);
        let eq127_e1621_d_n20: f64 = (p.p7 * var_qg_fp1_dn20);
        let eq127_e1621_d_n21: f64 = (p.p7 * var_qg_fp1_dn21);
        let eq127_e1621_d_n22: f64 = (p.p7 * var_qg_fp1_dn22);
        let eq127_e1621_q: f64 = (p.p7 * eq127_e1620_q);
        (eq127_e1621, eq127_e1621_d_n0, eq127_e1621_d_n1, eq127_e1621_d_n2, eq127_e1621_d_n3, eq127_e1621_d_n4, eq127_e1621_d_n5, eq127_e1621_d_n6, eq127_e1621_d_n7, eq127_e1621_d_n8, eq127_e1621_d_n9, eq127_e1621_d_n12, eq127_e1621_d_n14, eq127_e1621_d_n15, eq127_e1621_d_n16, eq127_e1621_d_n17, eq127_e1621_d_n18, eq127_e1621_d_n19, eq127_e1621_d_n20, eq127_e1621_d_n21, eq127_e1621_d_n22, eq127_e1621_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq127_reactive_node_derivatives: [f64; 23] = [eq127_e1623_d_n0, eq127_e1623_d_n1, eq127_e1623_d_n2, eq127_e1623_d_n3, eq127_e1623_d_n4, eq127_e1623_d_n5, eq127_e1623_d_n6, eq127_e1623_d_n7, eq127_e1623_d_n8, eq127_e1623_d_n9, 0.0, 0.0, eq127_e1623_d_n12, 0.0, eq127_e1623_d_n14, eq127_e1623_d_n15, eq127_e1623_d_n16, eq127_e1623_d_n17, eq127_e1623_d_n18, eq127_e1623_d_n19, eq127_e1623_d_n20, eq127_e1623_d_n21, eq127_e1623_d_n22];
        let eq127_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq127_reactive_node_derivatives,
            branches,
            &eq127_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq128_e1637, eq128_e1637_d_n0, eq128_e1637_d_n1, eq128_e1637_d_n2, eq128_e1637_d_n3, eq128_e1637_d_n4, eq128_e1637_d_n5, eq128_e1637_d_n6, eq128_e1637_d_n7, eq128_e1637_d_n8, eq128_e1637_d_n9, eq128_e1637_d_n12, eq128_e1637_d_n14, eq128_e1637_d_n15, eq128_e1637_d_n16, eq128_e1637_d_n17, eq128_e1637_d_n18, eq128_e1637_d_n19, eq128_e1637_d_n20, eq128_e1637_d_n21, eq128_e1637_d_n22, eq128_e1637_q,) = {
    if (((var_guard536 == 0.0) && (var_guard539 != 0.0)) && (var_guard540 != 0.0)) {
        let eq128_e1632_q: f64 = var_qg_fp1;
        let eq128_e1633: f64 = (p.p7 * var_qg_fp1);
        let eq128_e1633_d_n0: f64 = (p.p7 * var_qg_fp1_dn0);
        let eq128_e1633_d_n1: f64 = (p.p7 * var_qg_fp1_dn1);
        let eq128_e1633_d_n2: f64 = (p.p7 * var_qg_fp1_dn2);
        let eq128_e1633_d_n3: f64 = (p.p7 * var_qg_fp1_dn3);
        let eq128_e1633_d_n4: f64 = (p.p7 * var_qg_fp1_dn4);
        let eq128_e1633_d_n5: f64 = (p.p7 * var_qg_fp1_dn5);
        let eq128_e1633_d_n6: f64 = (p.p7 * var_qg_fp1_dn6);
        let eq128_e1633_d_n7: f64 = (p.p7 * var_qg_fp1_dn7);
        let eq128_e1633_d_n8: f64 = (p.p7 * var_qg_fp1_dn8);
        let eq128_e1633_d_n9: f64 = (p.p7 * var_qg_fp1_dn9);
        let eq128_e1633_d_n12: f64 = (p.p7 * var_qg_fp1_dn12);
        let eq128_e1633_d_n14: f64 = (p.p7 * var_qg_fp1_dn14);
        let eq128_e1633_d_n15: f64 = (p.p7 * var_qg_fp1_dn15);
        let eq128_e1633_d_n16: f64 = (p.p7 * var_qg_fp1_dn16);
        let eq128_e1633_d_n17: f64 = (p.p7 * var_qg_fp1_dn17);
        let eq128_e1633_d_n18: f64 = (p.p7 * var_qg_fp1_dn18);
        let eq128_e1633_d_n19: f64 = (p.p7 * var_qg_fp1_dn19);
        let eq128_e1633_d_n20: f64 = (p.p7 * var_qg_fp1_dn20);
        let eq128_e1633_d_n21: f64 = (p.p7 * var_qg_fp1_dn21);
        let eq128_e1633_d_n22: f64 = (p.p7 * var_qg_fp1_dn22);
        let eq128_e1633_q: f64 = (p.p7 * eq128_e1632_q);
        let eq128_e1635: f64 = (eq128_e1633 * p.p246);
        let eq128_e1635_d_n0: f64 = (eq128_e1633_d_n0 * p.p246);
        let eq128_e1635_d_n1: f64 = (eq128_e1633_d_n1 * p.p246);
        let eq128_e1635_d_n2: f64 = (eq128_e1633_d_n2 * p.p246);
        let eq128_e1635_d_n3: f64 = (eq128_e1633_d_n3 * p.p246);
        let eq128_e1635_d_n4: f64 = (eq128_e1633_d_n4 * p.p246);
        let eq128_e1635_d_n5: f64 = (eq128_e1633_d_n5 * p.p246);
        let eq128_e1635_d_n6: f64 = (eq128_e1633_d_n6 * p.p246);
        let eq128_e1635_d_n7: f64 = (eq128_e1633_d_n7 * p.p246);
        let eq128_e1635_d_n8: f64 = (eq128_e1633_d_n8 * p.p246);
        let eq128_e1635_d_n9: f64 = (eq128_e1633_d_n9 * p.p246);
        let eq128_e1635_d_n12: f64 = (eq128_e1633_d_n12 * p.p246);
        let eq128_e1635_d_n14: f64 = (eq128_e1633_d_n14 * p.p246);
        let eq128_e1635_d_n15: f64 = (eq128_e1633_d_n15 * p.p246);
        let eq128_e1635_d_n16: f64 = (eq128_e1633_d_n16 * p.p246);
        let eq128_e1635_d_n17: f64 = (eq128_e1633_d_n17 * p.p246);
        let eq128_e1635_d_n18: f64 = (eq128_e1633_d_n18 * p.p246);
        let eq128_e1635_d_n19: f64 = (eq128_e1633_d_n19 * p.p246);
        let eq128_e1635_d_n20: f64 = (eq128_e1633_d_n20 * p.p246);
        let eq128_e1635_d_n21: f64 = (eq128_e1633_d_n21 * p.p246);
        let eq128_e1635_d_n22: f64 = (eq128_e1633_d_n22 * p.p246);
        let eq128_e1635_q: f64 = (eq128_e1633_q * p.p246);
        (eq128_e1635, eq128_e1635_d_n0, eq128_e1635_d_n1, eq128_e1635_d_n2, eq128_e1635_d_n3, eq128_e1635_d_n4, eq128_e1635_d_n5, eq128_e1635_d_n6, eq128_e1635_d_n7, eq128_e1635_d_n8, eq128_e1635_d_n9, eq128_e1635_d_n12, eq128_e1635_d_n14, eq128_e1635_d_n15, eq128_e1635_d_n16, eq128_e1635_d_n17, eq128_e1635_d_n18, eq128_e1635_d_n19, eq128_e1635_d_n20, eq128_e1635_d_n21, eq128_e1635_d_n22, eq128_e1635_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq128_reactive_node_derivatives: [f64; 23] = [eq128_e1637_d_n0, eq128_e1637_d_n1, eq128_e1637_d_n2, eq128_e1637_d_n3, eq128_e1637_d_n4, eq128_e1637_d_n5, eq128_e1637_d_n6, eq128_e1637_d_n7, eq128_e1637_d_n8, eq128_e1637_d_n9, 0.0, 0.0, eq128_e1637_d_n12, 0.0, eq128_e1637_d_n14, eq128_e1637_d_n15, eq128_e1637_d_n16, eq128_e1637_d_n17, eq128_e1637_d_n18, eq128_e1637_d_n19, eq128_e1637_d_n20, eq128_e1637_d_n21, eq128_e1637_d_n22];
        let eq128_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq128_reactive_node_derivatives,
            branches,
            &eq128_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq129_e1650, eq129_e1650_d_n0, eq129_e1650_d_n1, eq129_e1650_d_n2, eq129_e1650_d_n3, eq129_e1650_d_n4, eq129_e1650_d_n5, eq129_e1650_d_n6, eq129_e1650_d_n7, eq129_e1650_d_n8, eq129_e1650_d_n9, eq129_e1650_d_n12, eq129_e1650_d_n14, eq129_e1650_d_n15, eq129_e1650_d_n16, eq129_e1650_d_n17, eq129_e1650_d_n18, eq129_e1650_d_n19, eq129_e1650_d_n20, eq129_e1650_d_n21, eq129_e1650_d_n22, eq129_e1650_q,) = {
    if (((var_guard536 == 0.0) && (var_guard539 != 0.0)) && (var_guard540 == 0.0)) {
        let eq129_e1647_q: f64 = var_qg_fp1;
        let eq129_e1648: f64 = (p.p7 * var_qg_fp1);
        let eq129_e1648_d_n0: f64 = (p.p7 * var_qg_fp1_dn0);
        let eq129_e1648_d_n1: f64 = (p.p7 * var_qg_fp1_dn1);
        let eq129_e1648_d_n2: f64 = (p.p7 * var_qg_fp1_dn2);
        let eq129_e1648_d_n3: f64 = (p.p7 * var_qg_fp1_dn3);
        let eq129_e1648_d_n4: f64 = (p.p7 * var_qg_fp1_dn4);
        let eq129_e1648_d_n5: f64 = (p.p7 * var_qg_fp1_dn5);
        let eq129_e1648_d_n6: f64 = (p.p7 * var_qg_fp1_dn6);
        let eq129_e1648_d_n7: f64 = (p.p7 * var_qg_fp1_dn7);
        let eq129_e1648_d_n8: f64 = (p.p7 * var_qg_fp1_dn8);
        let eq129_e1648_d_n9: f64 = (p.p7 * var_qg_fp1_dn9);
        let eq129_e1648_d_n12: f64 = (p.p7 * var_qg_fp1_dn12);
        let eq129_e1648_d_n14: f64 = (p.p7 * var_qg_fp1_dn14);
        let eq129_e1648_d_n15: f64 = (p.p7 * var_qg_fp1_dn15);
        let eq129_e1648_d_n16: f64 = (p.p7 * var_qg_fp1_dn16);
        let eq129_e1648_d_n17: f64 = (p.p7 * var_qg_fp1_dn17);
        let eq129_e1648_d_n18: f64 = (p.p7 * var_qg_fp1_dn18);
        let eq129_e1648_d_n19: f64 = (p.p7 * var_qg_fp1_dn19);
        let eq129_e1648_d_n20: f64 = (p.p7 * var_qg_fp1_dn20);
        let eq129_e1648_d_n21: f64 = (p.p7 * var_qg_fp1_dn21);
        let eq129_e1648_d_n22: f64 = (p.p7 * var_qg_fp1_dn22);
        let eq129_e1648_q: f64 = (p.p7 * eq129_e1647_q);
        (eq129_e1648, eq129_e1648_d_n0, eq129_e1648_d_n1, eq129_e1648_d_n2, eq129_e1648_d_n3, eq129_e1648_d_n4, eq129_e1648_d_n5, eq129_e1648_d_n6, eq129_e1648_d_n7, eq129_e1648_d_n8, eq129_e1648_d_n9, eq129_e1648_d_n12, eq129_e1648_d_n14, eq129_e1648_d_n15, eq129_e1648_d_n16, eq129_e1648_d_n17, eq129_e1648_d_n18, eq129_e1648_d_n19, eq129_e1648_d_n20, eq129_e1648_d_n21, eq129_e1648_d_n22, eq129_e1648_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq129_reactive_node_derivatives: [f64; 23] = [eq129_e1650_d_n0, eq129_e1650_d_n1, eq129_e1650_d_n2, eq129_e1650_d_n3, eq129_e1650_d_n4, eq129_e1650_d_n5, eq129_e1650_d_n6, eq129_e1650_d_n7, eq129_e1650_d_n8, eq129_e1650_d_n9, 0.0, 0.0, eq129_e1650_d_n12, 0.0, eq129_e1650_d_n14, eq129_e1650_d_n15, eq129_e1650_d_n16, eq129_e1650_d_n17, eq129_e1650_d_n18, eq129_e1650_d_n19, eq129_e1650_d_n20, eq129_e1650_d_n21, eq129_e1650_d_n22];
        let eq129_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq129_reactive_node_derivatives,
            branches,
            &eq129_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_2(
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_guard536: f64,
        var_guard539: f64,
        var_guard540: f64,
        var_guard541: f64,
        var_guard542: f64,
        var_guard543: f64,
        var_guard544: f64,
        var_qd_fp1s: f64,
        var_qd_fp1s_dn0: f64,
        var_qd_fp1s_dn1: f64,
        var_qd_fp1s_dn12: f64,
        var_qd_fp1s_dn14: f64,
        var_qd_fp1s_dn15: f64,
        var_qd_fp1s_dn16: f64,
        var_qd_fp1s_dn17: f64,
        var_qd_fp1s_dn18: f64,
        var_qd_fp1s_dn19: f64,
        var_qd_fp1s_dn2: f64,
        var_qd_fp1s_dn20: f64,
        var_qd_fp1s_dn21: f64,
        var_qd_fp1s_dn22: f64,
        var_qd_fp1s_dn3: f64,
        var_qd_fp1s_dn4: f64,
        var_qd_fp1s_dn5: f64,
        var_qd_fp1s_dn6: f64,
        var_qd_fp1s_dn7: f64,
        var_qd_fp1s_dn8: f64,
        var_qd_fp1s_dn9: f64,
        var_qg_fp1: f64,
        var_qg_fp1_dn0: f64,
        var_qg_fp1_dn1: f64,
        var_qg_fp1_dn12: f64,
        var_qg_fp1_dn14: f64,
        var_qg_fp1_dn15: f64,
        var_qg_fp1_dn16: f64,
        var_qg_fp1_dn17: f64,
        var_qg_fp1_dn18: f64,
        var_qg_fp1_dn19: f64,
        var_qg_fp1_dn2: f64,
        var_qg_fp1_dn20: f64,
        var_qg_fp1_dn21: f64,
        var_qg_fp1_dn22: f64,
        var_qg_fp1_dn3: f64,
        var_qg_fp1_dn4: f64,
        var_qg_fp1_dn5: f64,
        var_qg_fp1_dn6: f64,
        var_qg_fp1_dn7: f64,
        var_qg_fp1_dn8: f64,
        var_qg_fp1_dn9: f64,
        var_qg_fp1s: f64,
        var_qg_fp1s_dn0: f64,
        var_qg_fp1s_dn1: f64,
        var_qg_fp1s_dn12: f64,
        var_qg_fp1s_dn14: f64,
        var_qg_fp1s_dn15: f64,
        var_qg_fp1s_dn16: f64,
        var_qg_fp1s_dn17: f64,
        var_qg_fp1s_dn18: f64,
        var_qg_fp1s_dn19: f64,
        var_qg_fp1s_dn2: f64,
        var_qg_fp1s_dn20: f64,
        var_qg_fp1s_dn21: f64,
        var_qg_fp1s_dn22: f64,
        var_qg_fp1s_dn3: f64,
        var_qg_fp1s_dn4: f64,
        var_qg_fp1s_dn5: f64,
        var_qg_fp1s_dn6: f64,
        var_qg_fp1s_dn7: f64,
        var_qg_fp1s_dn8: f64,
        var_qg_fp1s_dn9: f64,
    ) {
        let (eq130_e1665, eq130_e1665_d_n0, eq130_e1665_d_n1, eq130_e1665_d_n2, eq130_e1665_d_n3, eq130_e1665_d_n4, eq130_e1665_d_n5, eq130_e1665_d_n6, eq130_e1665_d_n7, eq130_e1665_d_n8, eq130_e1665_d_n9, eq130_e1665_d_n12, eq130_e1665_d_n14, eq130_e1665_d_n15, eq130_e1665_d_n16, eq130_e1665_d_n17, eq130_e1665_d_n18, eq130_e1665_d_n19, eq130_e1665_d_n20, eq130_e1665_d_n21, eq130_e1665_d_n22, eq130_e1665_q,) = {
    if (((var_guard536 == 0.0) && (var_guard539 != 0.0)) && (var_guard540 == 0.0)) {
        let eq130_e1660_q: f64 = var_qg_fp1;
        let eq130_e1661: f64 = (p.p7 * var_qg_fp1);
        let eq130_e1661_d_n0: f64 = (p.p7 * var_qg_fp1_dn0);
        let eq130_e1661_d_n1: f64 = (p.p7 * var_qg_fp1_dn1);
        let eq130_e1661_d_n2: f64 = (p.p7 * var_qg_fp1_dn2);
        let eq130_e1661_d_n3: f64 = (p.p7 * var_qg_fp1_dn3);
        let eq130_e1661_d_n4: f64 = (p.p7 * var_qg_fp1_dn4);
        let eq130_e1661_d_n5: f64 = (p.p7 * var_qg_fp1_dn5);
        let eq130_e1661_d_n6: f64 = (p.p7 * var_qg_fp1_dn6);
        let eq130_e1661_d_n7: f64 = (p.p7 * var_qg_fp1_dn7);
        let eq130_e1661_d_n8: f64 = (p.p7 * var_qg_fp1_dn8);
        let eq130_e1661_d_n9: f64 = (p.p7 * var_qg_fp1_dn9);
        let eq130_e1661_d_n12: f64 = (p.p7 * var_qg_fp1_dn12);
        let eq130_e1661_d_n14: f64 = (p.p7 * var_qg_fp1_dn14);
        let eq130_e1661_d_n15: f64 = (p.p7 * var_qg_fp1_dn15);
        let eq130_e1661_d_n16: f64 = (p.p7 * var_qg_fp1_dn16);
        let eq130_e1661_d_n17: f64 = (p.p7 * var_qg_fp1_dn17);
        let eq130_e1661_d_n18: f64 = (p.p7 * var_qg_fp1_dn18);
        let eq130_e1661_d_n19: f64 = (p.p7 * var_qg_fp1_dn19);
        let eq130_e1661_d_n20: f64 = (p.p7 * var_qg_fp1_dn20);
        let eq130_e1661_d_n21: f64 = (p.p7 * var_qg_fp1_dn21);
        let eq130_e1661_d_n22: f64 = (p.p7 * var_qg_fp1_dn22);
        let eq130_e1661_q: f64 = (p.p7 * eq130_e1660_q);
        let eq130_e1663: f64 = (eq130_e1661 * p.p246);
        let eq130_e1663_d_n0: f64 = (eq130_e1661_d_n0 * p.p246);
        let eq130_e1663_d_n1: f64 = (eq130_e1661_d_n1 * p.p246);
        let eq130_e1663_d_n2: f64 = (eq130_e1661_d_n2 * p.p246);
        let eq130_e1663_d_n3: f64 = (eq130_e1661_d_n3 * p.p246);
        let eq130_e1663_d_n4: f64 = (eq130_e1661_d_n4 * p.p246);
        let eq130_e1663_d_n5: f64 = (eq130_e1661_d_n5 * p.p246);
        let eq130_e1663_d_n6: f64 = (eq130_e1661_d_n6 * p.p246);
        let eq130_e1663_d_n7: f64 = (eq130_e1661_d_n7 * p.p246);
        let eq130_e1663_d_n8: f64 = (eq130_e1661_d_n8 * p.p246);
        let eq130_e1663_d_n9: f64 = (eq130_e1661_d_n9 * p.p246);
        let eq130_e1663_d_n12: f64 = (eq130_e1661_d_n12 * p.p246);
        let eq130_e1663_d_n14: f64 = (eq130_e1661_d_n14 * p.p246);
        let eq130_e1663_d_n15: f64 = (eq130_e1661_d_n15 * p.p246);
        let eq130_e1663_d_n16: f64 = (eq130_e1661_d_n16 * p.p246);
        let eq130_e1663_d_n17: f64 = (eq130_e1661_d_n17 * p.p246);
        let eq130_e1663_d_n18: f64 = (eq130_e1661_d_n18 * p.p246);
        let eq130_e1663_d_n19: f64 = (eq130_e1661_d_n19 * p.p246);
        let eq130_e1663_d_n20: f64 = (eq130_e1661_d_n20 * p.p246);
        let eq130_e1663_d_n21: f64 = (eq130_e1661_d_n21 * p.p246);
        let eq130_e1663_d_n22: f64 = (eq130_e1661_d_n22 * p.p246);
        let eq130_e1663_q: f64 = (eq130_e1661_q * p.p246);
        (eq130_e1663, eq130_e1663_d_n0, eq130_e1663_d_n1, eq130_e1663_d_n2, eq130_e1663_d_n3, eq130_e1663_d_n4, eq130_e1663_d_n5, eq130_e1663_d_n6, eq130_e1663_d_n7, eq130_e1663_d_n8, eq130_e1663_d_n9, eq130_e1663_d_n12, eq130_e1663_d_n14, eq130_e1663_d_n15, eq130_e1663_d_n16, eq130_e1663_d_n17, eq130_e1663_d_n18, eq130_e1663_d_n19, eq130_e1663_d_n20, eq130_e1663_d_n21, eq130_e1663_d_n22, eq130_e1663_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq130_reactive_node_derivatives: [f64; 23] = [eq130_e1665_d_n0, eq130_e1665_d_n1, eq130_e1665_d_n2, eq130_e1665_d_n3, eq130_e1665_d_n4, eq130_e1665_d_n5, eq130_e1665_d_n6, eq130_e1665_d_n7, eq130_e1665_d_n8, eq130_e1665_d_n9, 0.0, 0.0, eq130_e1665_d_n12, 0.0, eq130_e1665_d_n14, eq130_e1665_d_n15, eq130_e1665_d_n16, eq130_e1665_d_n17, eq130_e1665_d_n18, eq130_e1665_d_n19, eq130_e1665_d_n20, eq130_e1665_d_n21, eq130_e1665_d_n22];
        let eq130_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq130_reactive_node_derivatives,
            branches,
            &eq130_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq131_e1677, eq131_e1677_d_n0, eq131_e1677_d_n1, eq131_e1677_d_n2, eq131_e1677_d_n3, eq131_e1677_d_n4, eq131_e1677_d_n5, eq131_e1677_d_n6, eq131_e1677_d_n7, eq131_e1677_d_n8, eq131_e1677_d_n9, eq131_e1677_d_n12, eq131_e1677_d_n14, eq131_e1677_d_n15, eq131_e1677_d_n16, eq131_e1677_d_n17, eq131_e1677_d_n18, eq131_e1677_d_n19, eq131_e1677_d_n20, eq131_e1677_d_n21, eq131_e1677_d_n22, eq131_e1677_q,) = {
    if ((var_guard536 == 0.0) && (var_guard539 != 0.0)) {
        let eq131_e1673: f64 = (p.p251 * var_qg_fp1);
        let eq131_e1673_d_n0: f64 = (p.p251 * var_qg_fp1_dn0);
        let eq131_e1673_d_n1: f64 = (p.p251 * var_qg_fp1_dn1);
        let eq131_e1673_d_n2: f64 = (p.p251 * var_qg_fp1_dn2);
        let eq131_e1673_d_n3: f64 = (p.p251 * var_qg_fp1_dn3);
        let eq131_e1673_d_n4: f64 = (p.p251 * var_qg_fp1_dn4);
        let eq131_e1673_d_n5: f64 = (p.p251 * var_qg_fp1_dn5);
        let eq131_e1673_d_n6: f64 = (p.p251 * var_qg_fp1_dn6);
        let eq131_e1673_d_n7: f64 = (p.p251 * var_qg_fp1_dn7);
        let eq131_e1673_d_n8: f64 = (p.p251 * var_qg_fp1_dn8);
        let eq131_e1673_d_n9: f64 = (p.p251 * var_qg_fp1_dn9);
        let eq131_e1673_d_n12: f64 = (p.p251 * var_qg_fp1_dn12);
        let eq131_e1673_d_n14: f64 = (p.p251 * var_qg_fp1_dn14);
        let eq131_e1673_d_n15: f64 = (p.p251 * var_qg_fp1_dn15);
        let eq131_e1673_d_n16: f64 = (p.p251 * var_qg_fp1_dn16);
        let eq131_e1673_d_n17: f64 = (p.p251 * var_qg_fp1_dn17);
        let eq131_e1673_d_n18: f64 = (p.p251 * var_qg_fp1_dn18);
        let eq131_e1673_d_n19: f64 = (p.p251 * var_qg_fp1_dn19);
        let eq131_e1673_d_n20: f64 = (p.p251 * var_qg_fp1_dn20);
        let eq131_e1673_d_n21: f64 = (p.p251 * var_qg_fp1_dn21);
        let eq131_e1673_d_n22: f64 = (p.p251 * var_qg_fp1_dn22);
        let eq131_e1674_q: f64 = eq131_e1673;
        let eq131_e1675: f64 = (p.p7 * eq131_e1673);
        let eq131_e1675_d_n0: f64 = (p.p7 * eq131_e1673_d_n0);
        let eq131_e1675_d_n1: f64 = (p.p7 * eq131_e1673_d_n1);
        let eq131_e1675_d_n2: f64 = (p.p7 * eq131_e1673_d_n2);
        let eq131_e1675_d_n3: f64 = (p.p7 * eq131_e1673_d_n3);
        let eq131_e1675_d_n4: f64 = (p.p7 * eq131_e1673_d_n4);
        let eq131_e1675_d_n5: f64 = (p.p7 * eq131_e1673_d_n5);
        let eq131_e1675_d_n6: f64 = (p.p7 * eq131_e1673_d_n6);
        let eq131_e1675_d_n7: f64 = (p.p7 * eq131_e1673_d_n7);
        let eq131_e1675_d_n8: f64 = (p.p7 * eq131_e1673_d_n8);
        let eq131_e1675_d_n9: f64 = (p.p7 * eq131_e1673_d_n9);
        let eq131_e1675_d_n12: f64 = (p.p7 * eq131_e1673_d_n12);
        let eq131_e1675_d_n14: f64 = (p.p7 * eq131_e1673_d_n14);
        let eq131_e1675_d_n15: f64 = (p.p7 * eq131_e1673_d_n15);
        let eq131_e1675_d_n16: f64 = (p.p7 * eq131_e1673_d_n16);
        let eq131_e1675_d_n17: f64 = (p.p7 * eq131_e1673_d_n17);
        let eq131_e1675_d_n18: f64 = (p.p7 * eq131_e1673_d_n18);
        let eq131_e1675_d_n19: f64 = (p.p7 * eq131_e1673_d_n19);
        let eq131_e1675_d_n20: f64 = (p.p7 * eq131_e1673_d_n20);
        let eq131_e1675_d_n21: f64 = (p.p7 * eq131_e1673_d_n21);
        let eq131_e1675_d_n22: f64 = (p.p7 * eq131_e1673_d_n22);
        let eq131_e1675_q: f64 = (p.p7 * eq131_e1674_q);
        (eq131_e1675, eq131_e1675_d_n0, eq131_e1675_d_n1, eq131_e1675_d_n2, eq131_e1675_d_n3, eq131_e1675_d_n4, eq131_e1675_d_n5, eq131_e1675_d_n6, eq131_e1675_d_n7, eq131_e1675_d_n8, eq131_e1675_d_n9, eq131_e1675_d_n12, eq131_e1675_d_n14, eq131_e1675_d_n15, eq131_e1675_d_n16, eq131_e1675_d_n17, eq131_e1675_d_n18, eq131_e1675_d_n19, eq131_e1675_d_n20, eq131_e1675_d_n21, eq131_e1675_d_n22, eq131_e1675_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq131_reactive_node_derivatives: [f64; 23] = [eq131_e1677_d_n0, eq131_e1677_d_n1, eq131_e1677_d_n2, eq131_e1677_d_n3, eq131_e1677_d_n4, eq131_e1677_d_n5, eq131_e1677_d_n6, eq131_e1677_d_n7, eq131_e1677_d_n8, eq131_e1677_d_n9, 0.0, 0.0, eq131_e1677_d_n12, 0.0, eq131_e1677_d_n14, eq131_e1677_d_n15, eq131_e1677_d_n16, eq131_e1677_d_n17, eq131_e1677_d_n18, eq131_e1677_d_n19, eq131_e1677_d_n20, eq131_e1677_d_n21, eq131_e1677_d_n22];
        let eq131_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            nodes,
            &eq131_reactive_node_derivatives,
            branches,
            &eq131_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq132_e1686, eq132_e1686_d_n0, eq132_e1686_d_n1, eq132_e1686_d_n2, eq132_e1686_d_n3, eq132_e1686_d_n4, eq132_e1686_d_n5, eq132_e1686_d_n6, eq132_e1686_d_n7, eq132_e1686_d_n8, eq132_e1686_d_n9, eq132_e1686_d_n12, eq132_e1686_d_n14, eq132_e1686_d_n15, eq132_e1686_d_n16, eq132_e1686_d_n17, eq132_e1686_d_n18, eq132_e1686_d_n19, eq132_e1686_d_n20, eq132_e1686_d_n21, eq132_e1686_d_n22, eq132_e1686_q,) = {
    if ((var_guard541 != 0.0) && (var_guard542 != 0.0)) {
        let eq132_e1683_q: f64 = var_qd_fp1s;
        let eq132_e1684: f64 = (p.p7 * var_qd_fp1s);
        let eq132_e1684_d_n0: f64 = (p.p7 * var_qd_fp1s_dn0);
        let eq132_e1684_d_n1: f64 = (p.p7 * var_qd_fp1s_dn1);
        let eq132_e1684_d_n2: f64 = (p.p7 * var_qd_fp1s_dn2);
        let eq132_e1684_d_n3: f64 = (p.p7 * var_qd_fp1s_dn3);
        let eq132_e1684_d_n4: f64 = (p.p7 * var_qd_fp1s_dn4);
        let eq132_e1684_d_n5: f64 = (p.p7 * var_qd_fp1s_dn5);
        let eq132_e1684_d_n6: f64 = (p.p7 * var_qd_fp1s_dn6);
        let eq132_e1684_d_n7: f64 = (p.p7 * var_qd_fp1s_dn7);
        let eq132_e1684_d_n8: f64 = (p.p7 * var_qd_fp1s_dn8);
        let eq132_e1684_d_n9: f64 = (p.p7 * var_qd_fp1s_dn9);
        let eq132_e1684_d_n12: f64 = (p.p7 * var_qd_fp1s_dn12);
        let eq132_e1684_d_n14: f64 = (p.p7 * var_qd_fp1s_dn14);
        let eq132_e1684_d_n15: f64 = (p.p7 * var_qd_fp1s_dn15);
        let eq132_e1684_d_n16: f64 = (p.p7 * var_qd_fp1s_dn16);
        let eq132_e1684_d_n17: f64 = (p.p7 * var_qd_fp1s_dn17);
        let eq132_e1684_d_n18: f64 = (p.p7 * var_qd_fp1s_dn18);
        let eq132_e1684_d_n19: f64 = (p.p7 * var_qd_fp1s_dn19);
        let eq132_e1684_d_n20: f64 = (p.p7 * var_qd_fp1s_dn20);
        let eq132_e1684_d_n21: f64 = (p.p7 * var_qd_fp1s_dn21);
        let eq132_e1684_d_n22: f64 = (p.p7 * var_qd_fp1s_dn22);
        let eq132_e1684_q: f64 = (p.p7 * eq132_e1683_q);
        (eq132_e1684, eq132_e1684_d_n0, eq132_e1684_d_n1, eq132_e1684_d_n2, eq132_e1684_d_n3, eq132_e1684_d_n4, eq132_e1684_d_n5, eq132_e1684_d_n6, eq132_e1684_d_n7, eq132_e1684_d_n8, eq132_e1684_d_n9, eq132_e1684_d_n12, eq132_e1684_d_n14, eq132_e1684_d_n15, eq132_e1684_d_n16, eq132_e1684_d_n17, eq132_e1684_d_n18, eq132_e1684_d_n19, eq132_e1684_d_n20, eq132_e1684_d_n21, eq132_e1684_d_n22, eq132_e1684_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq132_reactive_node_derivatives: [f64; 23] = [eq132_e1686_d_n0, eq132_e1686_d_n1, eq132_e1686_d_n2, eq132_e1686_d_n3, eq132_e1686_d_n4, eq132_e1686_d_n5, eq132_e1686_d_n6, eq132_e1686_d_n7, eq132_e1686_d_n8, eq132_e1686_d_n9, 0.0, 0.0, eq132_e1686_d_n12, 0.0, eq132_e1686_d_n14, eq132_e1686_d_n15, eq132_e1686_d_n16, eq132_e1686_d_n17, eq132_e1686_d_n18, eq132_e1686_d_n19, eq132_e1686_d_n20, eq132_e1686_d_n21, eq132_e1686_d_n22];
        let eq132_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[19]),
            nodes,
            &eq132_reactive_node_derivatives,
            branches,
            &eq132_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq133_e1697, eq133_e1697_d_n0, eq133_e1697_d_n1, eq133_e1697_d_n2, eq133_e1697_d_n3, eq133_e1697_d_n4, eq133_e1697_d_n5, eq133_e1697_d_n6, eq133_e1697_d_n7, eq133_e1697_d_n8, eq133_e1697_d_n9, eq133_e1697_d_n12, eq133_e1697_d_n14, eq133_e1697_d_n15, eq133_e1697_d_n16, eq133_e1697_d_n17, eq133_e1697_d_n18, eq133_e1697_d_n19, eq133_e1697_d_n20, eq133_e1697_d_n21, eq133_e1697_d_n22, eq133_e1697_q,) = {
    if (((var_guard541 != 0.0) && (var_guard542 != 0.0)) && (var_guard543 != 0.0)) {
        let eq133_e1694_q: f64 = var_qg_fp1s;
        let eq133_e1695: f64 = (p.p7 * var_qg_fp1s);
        let eq133_e1695_d_n0: f64 = (p.p7 * var_qg_fp1s_dn0);
        let eq133_e1695_d_n1: f64 = (p.p7 * var_qg_fp1s_dn1);
        let eq133_e1695_d_n2: f64 = (p.p7 * var_qg_fp1s_dn2);
        let eq133_e1695_d_n3: f64 = (p.p7 * var_qg_fp1s_dn3);
        let eq133_e1695_d_n4: f64 = (p.p7 * var_qg_fp1s_dn4);
        let eq133_e1695_d_n5: f64 = (p.p7 * var_qg_fp1s_dn5);
        let eq133_e1695_d_n6: f64 = (p.p7 * var_qg_fp1s_dn6);
        let eq133_e1695_d_n7: f64 = (p.p7 * var_qg_fp1s_dn7);
        let eq133_e1695_d_n8: f64 = (p.p7 * var_qg_fp1s_dn8);
        let eq133_e1695_d_n9: f64 = (p.p7 * var_qg_fp1s_dn9);
        let eq133_e1695_d_n12: f64 = (p.p7 * var_qg_fp1s_dn12);
        let eq133_e1695_d_n14: f64 = (p.p7 * var_qg_fp1s_dn14);
        let eq133_e1695_d_n15: f64 = (p.p7 * var_qg_fp1s_dn15);
        let eq133_e1695_d_n16: f64 = (p.p7 * var_qg_fp1s_dn16);
        let eq133_e1695_d_n17: f64 = (p.p7 * var_qg_fp1s_dn17);
        let eq133_e1695_d_n18: f64 = (p.p7 * var_qg_fp1s_dn18);
        let eq133_e1695_d_n19: f64 = (p.p7 * var_qg_fp1s_dn19);
        let eq133_e1695_d_n20: f64 = (p.p7 * var_qg_fp1s_dn20);
        let eq133_e1695_d_n21: f64 = (p.p7 * var_qg_fp1s_dn21);
        let eq133_e1695_d_n22: f64 = (p.p7 * var_qg_fp1s_dn22);
        let eq133_e1695_q: f64 = (p.p7 * eq133_e1694_q);
        (eq133_e1695, eq133_e1695_d_n0, eq133_e1695_d_n1, eq133_e1695_d_n2, eq133_e1695_d_n3, eq133_e1695_d_n4, eq133_e1695_d_n5, eq133_e1695_d_n6, eq133_e1695_d_n7, eq133_e1695_d_n8, eq133_e1695_d_n9, eq133_e1695_d_n12, eq133_e1695_d_n14, eq133_e1695_d_n15, eq133_e1695_d_n16, eq133_e1695_d_n17, eq133_e1695_d_n18, eq133_e1695_d_n19, eq133_e1695_d_n20, eq133_e1695_d_n21, eq133_e1695_d_n22, eq133_e1695_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq133_reactive_node_derivatives: [f64; 23] = [eq133_e1697_d_n0, eq133_e1697_d_n1, eq133_e1697_d_n2, eq133_e1697_d_n3, eq133_e1697_d_n4, eq133_e1697_d_n5, eq133_e1697_d_n6, eq133_e1697_d_n7, eq133_e1697_d_n8, eq133_e1697_d_n9, 0.0, 0.0, eq133_e1697_d_n12, 0.0, eq133_e1697_d_n14, eq133_e1697_d_n15, eq133_e1697_d_n16, eq133_e1697_d_n17, eq133_e1697_d_n18, eq133_e1697_d_n19, eq133_e1697_d_n20, eq133_e1697_d_n21, eq133_e1697_d_n22];
        let eq133_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[19]),
            nodes,
            &eq133_reactive_node_derivatives,
            branches,
            &eq133_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq134_e1710, eq134_e1710_d_n0, eq134_e1710_d_n1, eq134_e1710_d_n2, eq134_e1710_d_n3, eq134_e1710_d_n4, eq134_e1710_d_n5, eq134_e1710_d_n6, eq134_e1710_d_n7, eq134_e1710_d_n8, eq134_e1710_d_n9, eq134_e1710_d_n12, eq134_e1710_d_n14, eq134_e1710_d_n15, eq134_e1710_d_n16, eq134_e1710_d_n17, eq134_e1710_d_n18, eq134_e1710_d_n19, eq134_e1710_d_n20, eq134_e1710_d_n21, eq134_e1710_d_n22, eq134_e1710_q,) = {
    if (((var_guard541 != 0.0) && (var_guard542 != 0.0)) && (var_guard543 != 0.0)) {
        let eq134_e1705_q: f64 = var_qg_fp1s;
        let eq134_e1706: f64 = (p.p7 * var_qg_fp1s);
        let eq134_e1706_d_n0: f64 = (p.p7 * var_qg_fp1s_dn0);
        let eq134_e1706_d_n1: f64 = (p.p7 * var_qg_fp1s_dn1);
        let eq134_e1706_d_n2: f64 = (p.p7 * var_qg_fp1s_dn2);
        let eq134_e1706_d_n3: f64 = (p.p7 * var_qg_fp1s_dn3);
        let eq134_e1706_d_n4: f64 = (p.p7 * var_qg_fp1s_dn4);
        let eq134_e1706_d_n5: f64 = (p.p7 * var_qg_fp1s_dn5);
        let eq134_e1706_d_n6: f64 = (p.p7 * var_qg_fp1s_dn6);
        let eq134_e1706_d_n7: f64 = (p.p7 * var_qg_fp1s_dn7);
        let eq134_e1706_d_n8: f64 = (p.p7 * var_qg_fp1s_dn8);
        let eq134_e1706_d_n9: f64 = (p.p7 * var_qg_fp1s_dn9);
        let eq134_e1706_d_n12: f64 = (p.p7 * var_qg_fp1s_dn12);
        let eq134_e1706_d_n14: f64 = (p.p7 * var_qg_fp1s_dn14);
        let eq134_e1706_d_n15: f64 = (p.p7 * var_qg_fp1s_dn15);
        let eq134_e1706_d_n16: f64 = (p.p7 * var_qg_fp1s_dn16);
        let eq134_e1706_d_n17: f64 = (p.p7 * var_qg_fp1s_dn17);
        let eq134_e1706_d_n18: f64 = (p.p7 * var_qg_fp1s_dn18);
        let eq134_e1706_d_n19: f64 = (p.p7 * var_qg_fp1s_dn19);
        let eq134_e1706_d_n20: f64 = (p.p7 * var_qg_fp1s_dn20);
        let eq134_e1706_d_n21: f64 = (p.p7 * var_qg_fp1s_dn21);
        let eq134_e1706_d_n22: f64 = (p.p7 * var_qg_fp1s_dn22);
        let eq134_e1706_q: f64 = (p.p7 * eq134_e1705_q);
        let eq134_e1708: f64 = (eq134_e1706 * p.p246);
        let eq134_e1708_d_n0: f64 = (eq134_e1706_d_n0 * p.p246);
        let eq134_e1708_d_n1: f64 = (eq134_e1706_d_n1 * p.p246);
        let eq134_e1708_d_n2: f64 = (eq134_e1706_d_n2 * p.p246);
        let eq134_e1708_d_n3: f64 = (eq134_e1706_d_n3 * p.p246);
        let eq134_e1708_d_n4: f64 = (eq134_e1706_d_n4 * p.p246);
        let eq134_e1708_d_n5: f64 = (eq134_e1706_d_n5 * p.p246);
        let eq134_e1708_d_n6: f64 = (eq134_e1706_d_n6 * p.p246);
        let eq134_e1708_d_n7: f64 = (eq134_e1706_d_n7 * p.p246);
        let eq134_e1708_d_n8: f64 = (eq134_e1706_d_n8 * p.p246);
        let eq134_e1708_d_n9: f64 = (eq134_e1706_d_n9 * p.p246);
        let eq134_e1708_d_n12: f64 = (eq134_e1706_d_n12 * p.p246);
        let eq134_e1708_d_n14: f64 = (eq134_e1706_d_n14 * p.p246);
        let eq134_e1708_d_n15: f64 = (eq134_e1706_d_n15 * p.p246);
        let eq134_e1708_d_n16: f64 = (eq134_e1706_d_n16 * p.p246);
        let eq134_e1708_d_n17: f64 = (eq134_e1706_d_n17 * p.p246);
        let eq134_e1708_d_n18: f64 = (eq134_e1706_d_n18 * p.p246);
        let eq134_e1708_d_n19: f64 = (eq134_e1706_d_n19 * p.p246);
        let eq134_e1708_d_n20: f64 = (eq134_e1706_d_n20 * p.p246);
        let eq134_e1708_d_n21: f64 = (eq134_e1706_d_n21 * p.p246);
        let eq134_e1708_d_n22: f64 = (eq134_e1706_d_n22 * p.p246);
        let eq134_e1708_q: f64 = (eq134_e1706_q * p.p246);
        (eq134_e1708, eq134_e1708_d_n0, eq134_e1708_d_n1, eq134_e1708_d_n2, eq134_e1708_d_n3, eq134_e1708_d_n4, eq134_e1708_d_n5, eq134_e1708_d_n6, eq134_e1708_d_n7, eq134_e1708_d_n8, eq134_e1708_d_n9, eq134_e1708_d_n12, eq134_e1708_d_n14, eq134_e1708_d_n15, eq134_e1708_d_n16, eq134_e1708_d_n17, eq134_e1708_d_n18, eq134_e1708_d_n19, eq134_e1708_d_n20, eq134_e1708_d_n21, eq134_e1708_d_n22, eq134_e1708_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq134_reactive_node_derivatives: [f64; 23] = [eq134_e1710_d_n0, eq134_e1710_d_n1, eq134_e1710_d_n2, eq134_e1710_d_n3, eq134_e1710_d_n4, eq134_e1710_d_n5, eq134_e1710_d_n6, eq134_e1710_d_n7, eq134_e1710_d_n8, eq134_e1710_d_n9, 0.0, 0.0, eq134_e1710_d_n12, 0.0, eq134_e1710_d_n14, eq134_e1710_d_n15, eq134_e1710_d_n16, eq134_e1710_d_n17, eq134_e1710_d_n18, eq134_e1710_d_n19, eq134_e1710_d_n20, eq134_e1710_d_n21, eq134_e1710_d_n22];
        let eq134_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[19]),
            nodes,
            &eq134_reactive_node_derivatives,
            branches,
            &eq134_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq135_e1722, eq135_e1722_d_n0, eq135_e1722_d_n1, eq135_e1722_d_n2, eq135_e1722_d_n3, eq135_e1722_d_n4, eq135_e1722_d_n5, eq135_e1722_d_n6, eq135_e1722_d_n7, eq135_e1722_d_n8, eq135_e1722_d_n9, eq135_e1722_d_n12, eq135_e1722_d_n14, eq135_e1722_d_n15, eq135_e1722_d_n16, eq135_e1722_d_n17, eq135_e1722_d_n18, eq135_e1722_d_n19, eq135_e1722_d_n20, eq135_e1722_d_n21, eq135_e1722_d_n22, eq135_e1722_q,) = {
    if (((var_guard541 != 0.0) && (var_guard542 != 0.0)) && (var_guard543 == 0.0)) {
        let eq135_e1719_q: f64 = var_qg_fp1s;
        let eq135_e1720: f64 = (p.p7 * var_qg_fp1s);
        let eq135_e1720_d_n0: f64 = (p.p7 * var_qg_fp1s_dn0);
        let eq135_e1720_d_n1: f64 = (p.p7 * var_qg_fp1s_dn1);
        let eq135_e1720_d_n2: f64 = (p.p7 * var_qg_fp1s_dn2);
        let eq135_e1720_d_n3: f64 = (p.p7 * var_qg_fp1s_dn3);
        let eq135_e1720_d_n4: f64 = (p.p7 * var_qg_fp1s_dn4);
        let eq135_e1720_d_n5: f64 = (p.p7 * var_qg_fp1s_dn5);
        let eq135_e1720_d_n6: f64 = (p.p7 * var_qg_fp1s_dn6);
        let eq135_e1720_d_n7: f64 = (p.p7 * var_qg_fp1s_dn7);
        let eq135_e1720_d_n8: f64 = (p.p7 * var_qg_fp1s_dn8);
        let eq135_e1720_d_n9: f64 = (p.p7 * var_qg_fp1s_dn9);
        let eq135_e1720_d_n12: f64 = (p.p7 * var_qg_fp1s_dn12);
        let eq135_e1720_d_n14: f64 = (p.p7 * var_qg_fp1s_dn14);
        let eq135_e1720_d_n15: f64 = (p.p7 * var_qg_fp1s_dn15);
        let eq135_e1720_d_n16: f64 = (p.p7 * var_qg_fp1s_dn16);
        let eq135_e1720_d_n17: f64 = (p.p7 * var_qg_fp1s_dn17);
        let eq135_e1720_d_n18: f64 = (p.p7 * var_qg_fp1s_dn18);
        let eq135_e1720_d_n19: f64 = (p.p7 * var_qg_fp1s_dn19);
        let eq135_e1720_d_n20: f64 = (p.p7 * var_qg_fp1s_dn20);
        let eq135_e1720_d_n21: f64 = (p.p7 * var_qg_fp1s_dn21);
        let eq135_e1720_d_n22: f64 = (p.p7 * var_qg_fp1s_dn22);
        let eq135_e1720_q: f64 = (p.p7 * eq135_e1719_q);
        (eq135_e1720, eq135_e1720_d_n0, eq135_e1720_d_n1, eq135_e1720_d_n2, eq135_e1720_d_n3, eq135_e1720_d_n4, eq135_e1720_d_n5, eq135_e1720_d_n6, eq135_e1720_d_n7, eq135_e1720_d_n8, eq135_e1720_d_n9, eq135_e1720_d_n12, eq135_e1720_d_n14, eq135_e1720_d_n15, eq135_e1720_d_n16, eq135_e1720_d_n17, eq135_e1720_d_n18, eq135_e1720_d_n19, eq135_e1720_d_n20, eq135_e1720_d_n21, eq135_e1720_d_n22, eq135_e1720_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq135_reactive_node_derivatives: [f64; 23] = [eq135_e1722_d_n0, eq135_e1722_d_n1, eq135_e1722_d_n2, eq135_e1722_d_n3, eq135_e1722_d_n4, eq135_e1722_d_n5, eq135_e1722_d_n6, eq135_e1722_d_n7, eq135_e1722_d_n8, eq135_e1722_d_n9, 0.0, 0.0, eq135_e1722_d_n12, 0.0, eq135_e1722_d_n14, eq135_e1722_d_n15, eq135_e1722_d_n16, eq135_e1722_d_n17, eq135_e1722_d_n18, eq135_e1722_d_n19, eq135_e1722_d_n20, eq135_e1722_d_n21, eq135_e1722_d_n22];
        let eq135_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[19]),
            nodes,
            &eq135_reactive_node_derivatives,
            branches,
            &eq135_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq136_e1736, eq136_e1736_d_n0, eq136_e1736_d_n1, eq136_e1736_d_n2, eq136_e1736_d_n3, eq136_e1736_d_n4, eq136_e1736_d_n5, eq136_e1736_d_n6, eq136_e1736_d_n7, eq136_e1736_d_n8, eq136_e1736_d_n9, eq136_e1736_d_n12, eq136_e1736_d_n14, eq136_e1736_d_n15, eq136_e1736_d_n16, eq136_e1736_d_n17, eq136_e1736_d_n18, eq136_e1736_d_n19, eq136_e1736_d_n20, eq136_e1736_d_n21, eq136_e1736_d_n22, eq136_e1736_q,) = {
    if (((var_guard541 != 0.0) && (var_guard542 != 0.0)) && (var_guard543 == 0.0)) {
        let eq136_e1731_q: f64 = var_qg_fp1s;
        let eq136_e1732: f64 = (p.p7 * var_qg_fp1s);
        let eq136_e1732_d_n0: f64 = (p.p7 * var_qg_fp1s_dn0);
        let eq136_e1732_d_n1: f64 = (p.p7 * var_qg_fp1s_dn1);
        let eq136_e1732_d_n2: f64 = (p.p7 * var_qg_fp1s_dn2);
        let eq136_e1732_d_n3: f64 = (p.p7 * var_qg_fp1s_dn3);
        let eq136_e1732_d_n4: f64 = (p.p7 * var_qg_fp1s_dn4);
        let eq136_e1732_d_n5: f64 = (p.p7 * var_qg_fp1s_dn5);
        let eq136_e1732_d_n6: f64 = (p.p7 * var_qg_fp1s_dn6);
        let eq136_e1732_d_n7: f64 = (p.p7 * var_qg_fp1s_dn7);
        let eq136_e1732_d_n8: f64 = (p.p7 * var_qg_fp1s_dn8);
        let eq136_e1732_d_n9: f64 = (p.p7 * var_qg_fp1s_dn9);
        let eq136_e1732_d_n12: f64 = (p.p7 * var_qg_fp1s_dn12);
        let eq136_e1732_d_n14: f64 = (p.p7 * var_qg_fp1s_dn14);
        let eq136_e1732_d_n15: f64 = (p.p7 * var_qg_fp1s_dn15);
        let eq136_e1732_d_n16: f64 = (p.p7 * var_qg_fp1s_dn16);
        let eq136_e1732_d_n17: f64 = (p.p7 * var_qg_fp1s_dn17);
        let eq136_e1732_d_n18: f64 = (p.p7 * var_qg_fp1s_dn18);
        let eq136_e1732_d_n19: f64 = (p.p7 * var_qg_fp1s_dn19);
        let eq136_e1732_d_n20: f64 = (p.p7 * var_qg_fp1s_dn20);
        let eq136_e1732_d_n21: f64 = (p.p7 * var_qg_fp1s_dn21);
        let eq136_e1732_d_n22: f64 = (p.p7 * var_qg_fp1s_dn22);
        let eq136_e1732_q: f64 = (p.p7 * eq136_e1731_q);
        let eq136_e1734: f64 = (eq136_e1732 * p.p246);
        let eq136_e1734_d_n0: f64 = (eq136_e1732_d_n0 * p.p246);
        let eq136_e1734_d_n1: f64 = (eq136_e1732_d_n1 * p.p246);
        let eq136_e1734_d_n2: f64 = (eq136_e1732_d_n2 * p.p246);
        let eq136_e1734_d_n3: f64 = (eq136_e1732_d_n3 * p.p246);
        let eq136_e1734_d_n4: f64 = (eq136_e1732_d_n4 * p.p246);
        let eq136_e1734_d_n5: f64 = (eq136_e1732_d_n5 * p.p246);
        let eq136_e1734_d_n6: f64 = (eq136_e1732_d_n6 * p.p246);
        let eq136_e1734_d_n7: f64 = (eq136_e1732_d_n7 * p.p246);
        let eq136_e1734_d_n8: f64 = (eq136_e1732_d_n8 * p.p246);
        let eq136_e1734_d_n9: f64 = (eq136_e1732_d_n9 * p.p246);
        let eq136_e1734_d_n12: f64 = (eq136_e1732_d_n12 * p.p246);
        let eq136_e1734_d_n14: f64 = (eq136_e1732_d_n14 * p.p246);
        let eq136_e1734_d_n15: f64 = (eq136_e1732_d_n15 * p.p246);
        let eq136_e1734_d_n16: f64 = (eq136_e1732_d_n16 * p.p246);
        let eq136_e1734_d_n17: f64 = (eq136_e1732_d_n17 * p.p246);
        let eq136_e1734_d_n18: f64 = (eq136_e1732_d_n18 * p.p246);
        let eq136_e1734_d_n19: f64 = (eq136_e1732_d_n19 * p.p246);
        let eq136_e1734_d_n20: f64 = (eq136_e1732_d_n20 * p.p246);
        let eq136_e1734_d_n21: f64 = (eq136_e1732_d_n21 * p.p246);
        let eq136_e1734_d_n22: f64 = (eq136_e1732_d_n22 * p.p246);
        let eq136_e1734_q: f64 = (eq136_e1732_q * p.p246);
        (eq136_e1734, eq136_e1734_d_n0, eq136_e1734_d_n1, eq136_e1734_d_n2, eq136_e1734_d_n3, eq136_e1734_d_n4, eq136_e1734_d_n5, eq136_e1734_d_n6, eq136_e1734_d_n7, eq136_e1734_d_n8, eq136_e1734_d_n9, eq136_e1734_d_n12, eq136_e1734_d_n14, eq136_e1734_d_n15, eq136_e1734_d_n16, eq136_e1734_d_n17, eq136_e1734_d_n18, eq136_e1734_d_n19, eq136_e1734_d_n20, eq136_e1734_d_n21, eq136_e1734_d_n22, eq136_e1734_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq136_reactive_node_derivatives: [f64; 23] = [eq136_e1736_d_n0, eq136_e1736_d_n1, eq136_e1736_d_n2, eq136_e1736_d_n3, eq136_e1736_d_n4, eq136_e1736_d_n5, eq136_e1736_d_n6, eq136_e1736_d_n7, eq136_e1736_d_n8, eq136_e1736_d_n9, 0.0, 0.0, eq136_e1736_d_n12, 0.0, eq136_e1736_d_n14, eq136_e1736_d_n15, eq136_e1736_d_n16, eq136_e1736_d_n17, eq136_e1736_d_n18, eq136_e1736_d_n19, eq136_e1736_d_n20, eq136_e1736_d_n21, eq136_e1736_d_n22];
        let eq136_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[19]),
            nodes,
            &eq136_reactive_node_derivatives,
            branches,
            &eq136_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq137_e1747, eq137_e1747_d_n0, eq137_e1747_d_n1, eq137_e1747_d_n2, eq137_e1747_d_n3, eq137_e1747_d_n4, eq137_e1747_d_n5, eq137_e1747_d_n6, eq137_e1747_d_n7, eq137_e1747_d_n8, eq137_e1747_d_n9, eq137_e1747_d_n12, eq137_e1747_d_n14, eq137_e1747_d_n15, eq137_e1747_d_n16, eq137_e1747_d_n17, eq137_e1747_d_n18, eq137_e1747_d_n19, eq137_e1747_d_n20, eq137_e1747_d_n21, eq137_e1747_d_n22, eq137_e1747_q,) = {
    if ((var_guard541 != 0.0) && (var_guard542 != 0.0)) {
        let eq137_e1743: f64 = (p.p251 * var_qg_fp1s);
        let eq137_e1743_d_n0: f64 = (p.p251 * var_qg_fp1s_dn0);
        let eq137_e1743_d_n1: f64 = (p.p251 * var_qg_fp1s_dn1);
        let eq137_e1743_d_n2: f64 = (p.p251 * var_qg_fp1s_dn2);
        let eq137_e1743_d_n3: f64 = (p.p251 * var_qg_fp1s_dn3);
        let eq137_e1743_d_n4: f64 = (p.p251 * var_qg_fp1s_dn4);
        let eq137_e1743_d_n5: f64 = (p.p251 * var_qg_fp1s_dn5);
        let eq137_e1743_d_n6: f64 = (p.p251 * var_qg_fp1s_dn6);
        let eq137_e1743_d_n7: f64 = (p.p251 * var_qg_fp1s_dn7);
        let eq137_e1743_d_n8: f64 = (p.p251 * var_qg_fp1s_dn8);
        let eq137_e1743_d_n9: f64 = (p.p251 * var_qg_fp1s_dn9);
        let eq137_e1743_d_n12: f64 = (p.p251 * var_qg_fp1s_dn12);
        let eq137_e1743_d_n14: f64 = (p.p251 * var_qg_fp1s_dn14);
        let eq137_e1743_d_n15: f64 = (p.p251 * var_qg_fp1s_dn15);
        let eq137_e1743_d_n16: f64 = (p.p251 * var_qg_fp1s_dn16);
        let eq137_e1743_d_n17: f64 = (p.p251 * var_qg_fp1s_dn17);
        let eq137_e1743_d_n18: f64 = (p.p251 * var_qg_fp1s_dn18);
        let eq137_e1743_d_n19: f64 = (p.p251 * var_qg_fp1s_dn19);
        let eq137_e1743_d_n20: f64 = (p.p251 * var_qg_fp1s_dn20);
        let eq137_e1743_d_n21: f64 = (p.p251 * var_qg_fp1s_dn21);
        let eq137_e1743_d_n22: f64 = (p.p251 * var_qg_fp1s_dn22);
        let eq137_e1744_q: f64 = eq137_e1743;
        let eq137_e1745: f64 = (p.p7 * eq137_e1743);
        let eq137_e1745_d_n0: f64 = (p.p7 * eq137_e1743_d_n0);
        let eq137_e1745_d_n1: f64 = (p.p7 * eq137_e1743_d_n1);
        let eq137_e1745_d_n2: f64 = (p.p7 * eq137_e1743_d_n2);
        let eq137_e1745_d_n3: f64 = (p.p7 * eq137_e1743_d_n3);
        let eq137_e1745_d_n4: f64 = (p.p7 * eq137_e1743_d_n4);
        let eq137_e1745_d_n5: f64 = (p.p7 * eq137_e1743_d_n5);
        let eq137_e1745_d_n6: f64 = (p.p7 * eq137_e1743_d_n6);
        let eq137_e1745_d_n7: f64 = (p.p7 * eq137_e1743_d_n7);
        let eq137_e1745_d_n8: f64 = (p.p7 * eq137_e1743_d_n8);
        let eq137_e1745_d_n9: f64 = (p.p7 * eq137_e1743_d_n9);
        let eq137_e1745_d_n12: f64 = (p.p7 * eq137_e1743_d_n12);
        let eq137_e1745_d_n14: f64 = (p.p7 * eq137_e1743_d_n14);
        let eq137_e1745_d_n15: f64 = (p.p7 * eq137_e1743_d_n15);
        let eq137_e1745_d_n16: f64 = (p.p7 * eq137_e1743_d_n16);
        let eq137_e1745_d_n17: f64 = (p.p7 * eq137_e1743_d_n17);
        let eq137_e1745_d_n18: f64 = (p.p7 * eq137_e1743_d_n18);
        let eq137_e1745_d_n19: f64 = (p.p7 * eq137_e1743_d_n19);
        let eq137_e1745_d_n20: f64 = (p.p7 * eq137_e1743_d_n20);
        let eq137_e1745_d_n21: f64 = (p.p7 * eq137_e1743_d_n21);
        let eq137_e1745_d_n22: f64 = (p.p7 * eq137_e1743_d_n22);
        let eq137_e1745_q: f64 = (p.p7 * eq137_e1744_q);
        (eq137_e1745, eq137_e1745_d_n0, eq137_e1745_d_n1, eq137_e1745_d_n2, eq137_e1745_d_n3, eq137_e1745_d_n4, eq137_e1745_d_n5, eq137_e1745_d_n6, eq137_e1745_d_n7, eq137_e1745_d_n8, eq137_e1745_d_n9, eq137_e1745_d_n12, eq137_e1745_d_n14, eq137_e1745_d_n15, eq137_e1745_d_n16, eq137_e1745_d_n17, eq137_e1745_d_n18, eq137_e1745_d_n19, eq137_e1745_d_n20, eq137_e1745_d_n21, eq137_e1745_d_n22, eq137_e1745_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq137_reactive_node_derivatives: [f64; 23] = [eq137_e1747_d_n0, eq137_e1747_d_n1, eq137_e1747_d_n2, eq137_e1747_d_n3, eq137_e1747_d_n4, eq137_e1747_d_n5, eq137_e1747_d_n6, eq137_e1747_d_n7, eq137_e1747_d_n8, eq137_e1747_d_n9, 0.0, 0.0, eq137_e1747_d_n12, 0.0, eq137_e1747_d_n14, eq137_e1747_d_n15, eq137_e1747_d_n16, eq137_e1747_d_n17, eq137_e1747_d_n18, eq137_e1747_d_n19, eq137_e1747_d_n20, eq137_e1747_d_n21, eq137_e1747_d_n22];
        let eq137_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[19]),
            nodes,
            &eq137_reactive_node_derivatives,
            branches,
            &eq137_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq138_e1757, eq138_e1757_d_n0, eq138_e1757_d_n1, eq138_e1757_d_n2, eq138_e1757_d_n3, eq138_e1757_d_n4, eq138_e1757_d_n5, eq138_e1757_d_n6, eq138_e1757_d_n7, eq138_e1757_d_n8, eq138_e1757_d_n9, eq138_e1757_d_n12, eq138_e1757_d_n14, eq138_e1757_d_n15, eq138_e1757_d_n16, eq138_e1757_d_n17, eq138_e1757_d_n18, eq138_e1757_d_n19, eq138_e1757_d_n20, eq138_e1757_d_n21, eq138_e1757_d_n22, eq138_e1757_q,) = {
    if ((var_guard541 == 0.0) && (var_guard544 != 0.0)) {
        let eq138_e1754_q: f64 = var_qd_fp1s;
        let eq138_e1755: f64 = (p.p7 * var_qd_fp1s);
        let eq138_e1755_d_n0: f64 = (p.p7 * var_qd_fp1s_dn0);
        let eq138_e1755_d_n1: f64 = (p.p7 * var_qd_fp1s_dn1);
        let eq138_e1755_d_n2: f64 = (p.p7 * var_qd_fp1s_dn2);
        let eq138_e1755_d_n3: f64 = (p.p7 * var_qd_fp1s_dn3);
        let eq138_e1755_d_n4: f64 = (p.p7 * var_qd_fp1s_dn4);
        let eq138_e1755_d_n5: f64 = (p.p7 * var_qd_fp1s_dn5);
        let eq138_e1755_d_n6: f64 = (p.p7 * var_qd_fp1s_dn6);
        let eq138_e1755_d_n7: f64 = (p.p7 * var_qd_fp1s_dn7);
        let eq138_e1755_d_n8: f64 = (p.p7 * var_qd_fp1s_dn8);
        let eq138_e1755_d_n9: f64 = (p.p7 * var_qd_fp1s_dn9);
        let eq138_e1755_d_n12: f64 = (p.p7 * var_qd_fp1s_dn12);
        let eq138_e1755_d_n14: f64 = (p.p7 * var_qd_fp1s_dn14);
        let eq138_e1755_d_n15: f64 = (p.p7 * var_qd_fp1s_dn15);
        let eq138_e1755_d_n16: f64 = (p.p7 * var_qd_fp1s_dn16);
        let eq138_e1755_d_n17: f64 = (p.p7 * var_qd_fp1s_dn17);
        let eq138_e1755_d_n18: f64 = (p.p7 * var_qd_fp1s_dn18);
        let eq138_e1755_d_n19: f64 = (p.p7 * var_qd_fp1s_dn19);
        let eq138_e1755_d_n20: f64 = (p.p7 * var_qd_fp1s_dn20);
        let eq138_e1755_d_n21: f64 = (p.p7 * var_qd_fp1s_dn21);
        let eq138_e1755_d_n22: f64 = (p.p7 * var_qd_fp1s_dn22);
        let eq138_e1755_q: f64 = (p.p7 * eq138_e1754_q);
        (eq138_e1755, eq138_e1755_d_n0, eq138_e1755_d_n1, eq138_e1755_d_n2, eq138_e1755_d_n3, eq138_e1755_d_n4, eq138_e1755_d_n5, eq138_e1755_d_n6, eq138_e1755_d_n7, eq138_e1755_d_n8, eq138_e1755_d_n9, eq138_e1755_d_n12, eq138_e1755_d_n14, eq138_e1755_d_n15, eq138_e1755_d_n16, eq138_e1755_d_n17, eq138_e1755_d_n18, eq138_e1755_d_n19, eq138_e1755_d_n20, eq138_e1755_d_n21, eq138_e1755_d_n22, eq138_e1755_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq138_reactive_node_derivatives: [f64; 23] = [eq138_e1757_d_n0, eq138_e1757_d_n1, eq138_e1757_d_n2, eq138_e1757_d_n3, eq138_e1757_d_n4, eq138_e1757_d_n5, eq138_e1757_d_n6, eq138_e1757_d_n7, eq138_e1757_d_n8, eq138_e1757_d_n9, 0.0, 0.0, eq138_e1757_d_n12, 0.0, eq138_e1757_d_n14, eq138_e1757_d_n15, eq138_e1757_d_n16, eq138_e1757_d_n17, eq138_e1757_d_n18, eq138_e1757_d_n19, eq138_e1757_d_n20, eq138_e1757_d_n21, eq138_e1757_d_n22];
        let eq138_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            nodes,
            &eq138_reactive_node_derivatives,
            branches,
            &eq138_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_3(
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_guard541: f64,
        var_guard544: f64,
        var_guard545: f64,
        var_guard546: f64,
        var_guard547: f64,
        var_guard548: f64,
        var_qd_fp2: f64,
        var_qd_fp2_dn0: f64,
        var_qd_fp2_dn1: f64,
        var_qd_fp2_dn12: f64,
        var_qd_fp2_dn14: f64,
        var_qd_fp2_dn15: f64,
        var_qd_fp2_dn16: f64,
        var_qd_fp2_dn17: f64,
        var_qd_fp2_dn18: f64,
        var_qd_fp2_dn19: f64,
        var_qd_fp2_dn2: f64,
        var_qd_fp2_dn20: f64,
        var_qd_fp2_dn21: f64,
        var_qd_fp2_dn22: f64,
        var_qd_fp2_dn3: f64,
        var_qd_fp2_dn4: f64,
        var_qd_fp2_dn5: f64,
        var_qd_fp2_dn6: f64,
        var_qd_fp2_dn7: f64,
        var_qd_fp2_dn8: f64,
        var_qd_fp2_dn9: f64,
        var_qg_fp1s: f64,
        var_qg_fp1s_dn0: f64,
        var_qg_fp1s_dn1: f64,
        var_qg_fp1s_dn12: f64,
        var_qg_fp1s_dn14: f64,
        var_qg_fp1s_dn15: f64,
        var_qg_fp1s_dn16: f64,
        var_qg_fp1s_dn17: f64,
        var_qg_fp1s_dn18: f64,
        var_qg_fp1s_dn19: f64,
        var_qg_fp1s_dn2: f64,
        var_qg_fp1s_dn20: f64,
        var_qg_fp1s_dn21: f64,
        var_qg_fp1s_dn22: f64,
        var_qg_fp1s_dn3: f64,
        var_qg_fp1s_dn4: f64,
        var_qg_fp1s_dn5: f64,
        var_qg_fp1s_dn6: f64,
        var_qg_fp1s_dn7: f64,
        var_qg_fp1s_dn8: f64,
        var_qg_fp1s_dn9: f64,
        var_qg_fp2: f64,
        var_qg_fp2_dn0: f64,
        var_qg_fp2_dn1: f64,
        var_qg_fp2_dn12: f64,
        var_qg_fp2_dn14: f64,
        var_qg_fp2_dn15: f64,
        var_qg_fp2_dn16: f64,
        var_qg_fp2_dn17: f64,
        var_qg_fp2_dn18: f64,
        var_qg_fp2_dn19: f64,
        var_qg_fp2_dn2: f64,
        var_qg_fp2_dn20: f64,
        var_qg_fp2_dn21: f64,
        var_qg_fp2_dn22: f64,
        var_qg_fp2_dn3: f64,
        var_qg_fp2_dn4: f64,
        var_qg_fp2_dn5: f64,
        var_qg_fp2_dn6: f64,
        var_qg_fp2_dn7: f64,
        var_qg_fp2_dn8: f64,
        var_qg_fp2_dn9: f64,
    ) {
        let (eq139_e1769, eq139_e1769_d_n0, eq139_e1769_d_n1, eq139_e1769_d_n2, eq139_e1769_d_n3, eq139_e1769_d_n4, eq139_e1769_d_n5, eq139_e1769_d_n6, eq139_e1769_d_n7, eq139_e1769_d_n8, eq139_e1769_d_n9, eq139_e1769_d_n12, eq139_e1769_d_n14, eq139_e1769_d_n15, eq139_e1769_d_n16, eq139_e1769_d_n17, eq139_e1769_d_n18, eq139_e1769_d_n19, eq139_e1769_d_n20, eq139_e1769_d_n21, eq139_e1769_d_n22, eq139_e1769_q,) = {
    if (((var_guard541 == 0.0) && (var_guard544 != 0.0)) && (var_guard545 != 0.0)) {
        let eq139_e1766_q: f64 = var_qg_fp1s;
        let eq139_e1767: f64 = (p.p7 * var_qg_fp1s);
        let eq139_e1767_d_n0: f64 = (p.p7 * var_qg_fp1s_dn0);
        let eq139_e1767_d_n1: f64 = (p.p7 * var_qg_fp1s_dn1);
        let eq139_e1767_d_n2: f64 = (p.p7 * var_qg_fp1s_dn2);
        let eq139_e1767_d_n3: f64 = (p.p7 * var_qg_fp1s_dn3);
        let eq139_e1767_d_n4: f64 = (p.p7 * var_qg_fp1s_dn4);
        let eq139_e1767_d_n5: f64 = (p.p7 * var_qg_fp1s_dn5);
        let eq139_e1767_d_n6: f64 = (p.p7 * var_qg_fp1s_dn6);
        let eq139_e1767_d_n7: f64 = (p.p7 * var_qg_fp1s_dn7);
        let eq139_e1767_d_n8: f64 = (p.p7 * var_qg_fp1s_dn8);
        let eq139_e1767_d_n9: f64 = (p.p7 * var_qg_fp1s_dn9);
        let eq139_e1767_d_n12: f64 = (p.p7 * var_qg_fp1s_dn12);
        let eq139_e1767_d_n14: f64 = (p.p7 * var_qg_fp1s_dn14);
        let eq139_e1767_d_n15: f64 = (p.p7 * var_qg_fp1s_dn15);
        let eq139_e1767_d_n16: f64 = (p.p7 * var_qg_fp1s_dn16);
        let eq139_e1767_d_n17: f64 = (p.p7 * var_qg_fp1s_dn17);
        let eq139_e1767_d_n18: f64 = (p.p7 * var_qg_fp1s_dn18);
        let eq139_e1767_d_n19: f64 = (p.p7 * var_qg_fp1s_dn19);
        let eq139_e1767_d_n20: f64 = (p.p7 * var_qg_fp1s_dn20);
        let eq139_e1767_d_n21: f64 = (p.p7 * var_qg_fp1s_dn21);
        let eq139_e1767_d_n22: f64 = (p.p7 * var_qg_fp1s_dn22);
        let eq139_e1767_q: f64 = (p.p7 * eq139_e1766_q);
        (eq139_e1767, eq139_e1767_d_n0, eq139_e1767_d_n1, eq139_e1767_d_n2, eq139_e1767_d_n3, eq139_e1767_d_n4, eq139_e1767_d_n5, eq139_e1767_d_n6, eq139_e1767_d_n7, eq139_e1767_d_n8, eq139_e1767_d_n9, eq139_e1767_d_n12, eq139_e1767_d_n14, eq139_e1767_d_n15, eq139_e1767_d_n16, eq139_e1767_d_n17, eq139_e1767_d_n18, eq139_e1767_d_n19, eq139_e1767_d_n20, eq139_e1767_d_n21, eq139_e1767_d_n22, eq139_e1767_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq139_reactive_node_derivatives: [f64; 23] = [eq139_e1769_d_n0, eq139_e1769_d_n1, eq139_e1769_d_n2, eq139_e1769_d_n3, eq139_e1769_d_n4, eq139_e1769_d_n5, eq139_e1769_d_n6, eq139_e1769_d_n7, eq139_e1769_d_n8, eq139_e1769_d_n9, 0.0, 0.0, eq139_e1769_d_n12, 0.0, eq139_e1769_d_n14, eq139_e1769_d_n15, eq139_e1769_d_n16, eq139_e1769_d_n17, eq139_e1769_d_n18, eq139_e1769_d_n19, eq139_e1769_d_n20, eq139_e1769_d_n21, eq139_e1769_d_n22];
        let eq139_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq139_reactive_node_derivatives,
            branches,
            &eq139_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq140_e1783, eq140_e1783_d_n0, eq140_e1783_d_n1, eq140_e1783_d_n2, eq140_e1783_d_n3, eq140_e1783_d_n4, eq140_e1783_d_n5, eq140_e1783_d_n6, eq140_e1783_d_n7, eq140_e1783_d_n8, eq140_e1783_d_n9, eq140_e1783_d_n12, eq140_e1783_d_n14, eq140_e1783_d_n15, eq140_e1783_d_n16, eq140_e1783_d_n17, eq140_e1783_d_n18, eq140_e1783_d_n19, eq140_e1783_d_n20, eq140_e1783_d_n21, eq140_e1783_d_n22, eq140_e1783_q,) = {
    if (((var_guard541 == 0.0) && (var_guard544 != 0.0)) && (var_guard545 != 0.0)) {
        let eq140_e1778_q: f64 = var_qg_fp1s;
        let eq140_e1779: f64 = (p.p7 * var_qg_fp1s);
        let eq140_e1779_d_n0: f64 = (p.p7 * var_qg_fp1s_dn0);
        let eq140_e1779_d_n1: f64 = (p.p7 * var_qg_fp1s_dn1);
        let eq140_e1779_d_n2: f64 = (p.p7 * var_qg_fp1s_dn2);
        let eq140_e1779_d_n3: f64 = (p.p7 * var_qg_fp1s_dn3);
        let eq140_e1779_d_n4: f64 = (p.p7 * var_qg_fp1s_dn4);
        let eq140_e1779_d_n5: f64 = (p.p7 * var_qg_fp1s_dn5);
        let eq140_e1779_d_n6: f64 = (p.p7 * var_qg_fp1s_dn6);
        let eq140_e1779_d_n7: f64 = (p.p7 * var_qg_fp1s_dn7);
        let eq140_e1779_d_n8: f64 = (p.p7 * var_qg_fp1s_dn8);
        let eq140_e1779_d_n9: f64 = (p.p7 * var_qg_fp1s_dn9);
        let eq140_e1779_d_n12: f64 = (p.p7 * var_qg_fp1s_dn12);
        let eq140_e1779_d_n14: f64 = (p.p7 * var_qg_fp1s_dn14);
        let eq140_e1779_d_n15: f64 = (p.p7 * var_qg_fp1s_dn15);
        let eq140_e1779_d_n16: f64 = (p.p7 * var_qg_fp1s_dn16);
        let eq140_e1779_d_n17: f64 = (p.p7 * var_qg_fp1s_dn17);
        let eq140_e1779_d_n18: f64 = (p.p7 * var_qg_fp1s_dn18);
        let eq140_e1779_d_n19: f64 = (p.p7 * var_qg_fp1s_dn19);
        let eq140_e1779_d_n20: f64 = (p.p7 * var_qg_fp1s_dn20);
        let eq140_e1779_d_n21: f64 = (p.p7 * var_qg_fp1s_dn21);
        let eq140_e1779_d_n22: f64 = (p.p7 * var_qg_fp1s_dn22);
        let eq140_e1779_q: f64 = (p.p7 * eq140_e1778_q);
        let eq140_e1781: f64 = (eq140_e1779 * p.p246);
        let eq140_e1781_d_n0: f64 = (eq140_e1779_d_n0 * p.p246);
        let eq140_e1781_d_n1: f64 = (eq140_e1779_d_n1 * p.p246);
        let eq140_e1781_d_n2: f64 = (eq140_e1779_d_n2 * p.p246);
        let eq140_e1781_d_n3: f64 = (eq140_e1779_d_n3 * p.p246);
        let eq140_e1781_d_n4: f64 = (eq140_e1779_d_n4 * p.p246);
        let eq140_e1781_d_n5: f64 = (eq140_e1779_d_n5 * p.p246);
        let eq140_e1781_d_n6: f64 = (eq140_e1779_d_n6 * p.p246);
        let eq140_e1781_d_n7: f64 = (eq140_e1779_d_n7 * p.p246);
        let eq140_e1781_d_n8: f64 = (eq140_e1779_d_n8 * p.p246);
        let eq140_e1781_d_n9: f64 = (eq140_e1779_d_n9 * p.p246);
        let eq140_e1781_d_n12: f64 = (eq140_e1779_d_n12 * p.p246);
        let eq140_e1781_d_n14: f64 = (eq140_e1779_d_n14 * p.p246);
        let eq140_e1781_d_n15: f64 = (eq140_e1779_d_n15 * p.p246);
        let eq140_e1781_d_n16: f64 = (eq140_e1779_d_n16 * p.p246);
        let eq140_e1781_d_n17: f64 = (eq140_e1779_d_n17 * p.p246);
        let eq140_e1781_d_n18: f64 = (eq140_e1779_d_n18 * p.p246);
        let eq140_e1781_d_n19: f64 = (eq140_e1779_d_n19 * p.p246);
        let eq140_e1781_d_n20: f64 = (eq140_e1779_d_n20 * p.p246);
        let eq140_e1781_d_n21: f64 = (eq140_e1779_d_n21 * p.p246);
        let eq140_e1781_d_n22: f64 = (eq140_e1779_d_n22 * p.p246);
        let eq140_e1781_q: f64 = (eq140_e1779_q * p.p246);
        (eq140_e1781, eq140_e1781_d_n0, eq140_e1781_d_n1, eq140_e1781_d_n2, eq140_e1781_d_n3, eq140_e1781_d_n4, eq140_e1781_d_n5, eq140_e1781_d_n6, eq140_e1781_d_n7, eq140_e1781_d_n8, eq140_e1781_d_n9, eq140_e1781_d_n12, eq140_e1781_d_n14, eq140_e1781_d_n15, eq140_e1781_d_n16, eq140_e1781_d_n17, eq140_e1781_d_n18, eq140_e1781_d_n19, eq140_e1781_d_n20, eq140_e1781_d_n21, eq140_e1781_d_n22, eq140_e1781_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq140_reactive_node_derivatives: [f64; 23] = [eq140_e1783_d_n0, eq140_e1783_d_n1, eq140_e1783_d_n2, eq140_e1783_d_n3, eq140_e1783_d_n4, eq140_e1783_d_n5, eq140_e1783_d_n6, eq140_e1783_d_n7, eq140_e1783_d_n8, eq140_e1783_d_n9, 0.0, 0.0, eq140_e1783_d_n12, 0.0, eq140_e1783_d_n14, eq140_e1783_d_n15, eq140_e1783_d_n16, eq140_e1783_d_n17, eq140_e1783_d_n18, eq140_e1783_d_n19, eq140_e1783_d_n20, eq140_e1783_d_n21, eq140_e1783_d_n22];
        let eq140_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            nodes,
            &eq140_reactive_node_derivatives,
            branches,
            &eq140_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq141_e1796, eq141_e1796_d_n0, eq141_e1796_d_n1, eq141_e1796_d_n2, eq141_e1796_d_n3, eq141_e1796_d_n4, eq141_e1796_d_n5, eq141_e1796_d_n6, eq141_e1796_d_n7, eq141_e1796_d_n8, eq141_e1796_d_n9, eq141_e1796_d_n12, eq141_e1796_d_n14, eq141_e1796_d_n15, eq141_e1796_d_n16, eq141_e1796_d_n17, eq141_e1796_d_n18, eq141_e1796_d_n19, eq141_e1796_d_n20, eq141_e1796_d_n21, eq141_e1796_d_n22, eq141_e1796_q,) = {
    if (((var_guard541 == 0.0) && (var_guard544 != 0.0)) && (var_guard545 == 0.0)) {
        let eq141_e1793_q: f64 = var_qg_fp1s;
        let eq141_e1794: f64 = (p.p7 * var_qg_fp1s);
        let eq141_e1794_d_n0: f64 = (p.p7 * var_qg_fp1s_dn0);
        let eq141_e1794_d_n1: f64 = (p.p7 * var_qg_fp1s_dn1);
        let eq141_e1794_d_n2: f64 = (p.p7 * var_qg_fp1s_dn2);
        let eq141_e1794_d_n3: f64 = (p.p7 * var_qg_fp1s_dn3);
        let eq141_e1794_d_n4: f64 = (p.p7 * var_qg_fp1s_dn4);
        let eq141_e1794_d_n5: f64 = (p.p7 * var_qg_fp1s_dn5);
        let eq141_e1794_d_n6: f64 = (p.p7 * var_qg_fp1s_dn6);
        let eq141_e1794_d_n7: f64 = (p.p7 * var_qg_fp1s_dn7);
        let eq141_e1794_d_n8: f64 = (p.p7 * var_qg_fp1s_dn8);
        let eq141_e1794_d_n9: f64 = (p.p7 * var_qg_fp1s_dn9);
        let eq141_e1794_d_n12: f64 = (p.p7 * var_qg_fp1s_dn12);
        let eq141_e1794_d_n14: f64 = (p.p7 * var_qg_fp1s_dn14);
        let eq141_e1794_d_n15: f64 = (p.p7 * var_qg_fp1s_dn15);
        let eq141_e1794_d_n16: f64 = (p.p7 * var_qg_fp1s_dn16);
        let eq141_e1794_d_n17: f64 = (p.p7 * var_qg_fp1s_dn17);
        let eq141_e1794_d_n18: f64 = (p.p7 * var_qg_fp1s_dn18);
        let eq141_e1794_d_n19: f64 = (p.p7 * var_qg_fp1s_dn19);
        let eq141_e1794_d_n20: f64 = (p.p7 * var_qg_fp1s_dn20);
        let eq141_e1794_d_n21: f64 = (p.p7 * var_qg_fp1s_dn21);
        let eq141_e1794_d_n22: f64 = (p.p7 * var_qg_fp1s_dn22);
        let eq141_e1794_q: f64 = (p.p7 * eq141_e1793_q);
        (eq141_e1794, eq141_e1794_d_n0, eq141_e1794_d_n1, eq141_e1794_d_n2, eq141_e1794_d_n3, eq141_e1794_d_n4, eq141_e1794_d_n5, eq141_e1794_d_n6, eq141_e1794_d_n7, eq141_e1794_d_n8, eq141_e1794_d_n9, eq141_e1794_d_n12, eq141_e1794_d_n14, eq141_e1794_d_n15, eq141_e1794_d_n16, eq141_e1794_d_n17, eq141_e1794_d_n18, eq141_e1794_d_n19, eq141_e1794_d_n20, eq141_e1794_d_n21, eq141_e1794_d_n22, eq141_e1794_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq141_reactive_node_derivatives: [f64; 23] = [eq141_e1796_d_n0, eq141_e1796_d_n1, eq141_e1796_d_n2, eq141_e1796_d_n3, eq141_e1796_d_n4, eq141_e1796_d_n5, eq141_e1796_d_n6, eq141_e1796_d_n7, eq141_e1796_d_n8, eq141_e1796_d_n9, 0.0, 0.0, eq141_e1796_d_n12, 0.0, eq141_e1796_d_n14, eq141_e1796_d_n15, eq141_e1796_d_n16, eq141_e1796_d_n17, eq141_e1796_d_n18, eq141_e1796_d_n19, eq141_e1796_d_n20, eq141_e1796_d_n21, eq141_e1796_d_n22];
        let eq141_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            nodes,
            &eq141_reactive_node_derivatives,
            branches,
            &eq141_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq142_e1811, eq142_e1811_d_n0, eq142_e1811_d_n1, eq142_e1811_d_n2, eq142_e1811_d_n3, eq142_e1811_d_n4, eq142_e1811_d_n5, eq142_e1811_d_n6, eq142_e1811_d_n7, eq142_e1811_d_n8, eq142_e1811_d_n9, eq142_e1811_d_n12, eq142_e1811_d_n14, eq142_e1811_d_n15, eq142_e1811_d_n16, eq142_e1811_d_n17, eq142_e1811_d_n18, eq142_e1811_d_n19, eq142_e1811_d_n20, eq142_e1811_d_n21, eq142_e1811_d_n22, eq142_e1811_q,) = {
    if (((var_guard541 == 0.0) && (var_guard544 != 0.0)) && (var_guard545 == 0.0)) {
        let eq142_e1806_q: f64 = var_qg_fp1s;
        let eq142_e1807: f64 = (p.p7 * var_qg_fp1s);
        let eq142_e1807_d_n0: f64 = (p.p7 * var_qg_fp1s_dn0);
        let eq142_e1807_d_n1: f64 = (p.p7 * var_qg_fp1s_dn1);
        let eq142_e1807_d_n2: f64 = (p.p7 * var_qg_fp1s_dn2);
        let eq142_e1807_d_n3: f64 = (p.p7 * var_qg_fp1s_dn3);
        let eq142_e1807_d_n4: f64 = (p.p7 * var_qg_fp1s_dn4);
        let eq142_e1807_d_n5: f64 = (p.p7 * var_qg_fp1s_dn5);
        let eq142_e1807_d_n6: f64 = (p.p7 * var_qg_fp1s_dn6);
        let eq142_e1807_d_n7: f64 = (p.p7 * var_qg_fp1s_dn7);
        let eq142_e1807_d_n8: f64 = (p.p7 * var_qg_fp1s_dn8);
        let eq142_e1807_d_n9: f64 = (p.p7 * var_qg_fp1s_dn9);
        let eq142_e1807_d_n12: f64 = (p.p7 * var_qg_fp1s_dn12);
        let eq142_e1807_d_n14: f64 = (p.p7 * var_qg_fp1s_dn14);
        let eq142_e1807_d_n15: f64 = (p.p7 * var_qg_fp1s_dn15);
        let eq142_e1807_d_n16: f64 = (p.p7 * var_qg_fp1s_dn16);
        let eq142_e1807_d_n17: f64 = (p.p7 * var_qg_fp1s_dn17);
        let eq142_e1807_d_n18: f64 = (p.p7 * var_qg_fp1s_dn18);
        let eq142_e1807_d_n19: f64 = (p.p7 * var_qg_fp1s_dn19);
        let eq142_e1807_d_n20: f64 = (p.p7 * var_qg_fp1s_dn20);
        let eq142_e1807_d_n21: f64 = (p.p7 * var_qg_fp1s_dn21);
        let eq142_e1807_d_n22: f64 = (p.p7 * var_qg_fp1s_dn22);
        let eq142_e1807_q: f64 = (p.p7 * eq142_e1806_q);
        let eq142_e1809: f64 = (eq142_e1807 * p.p246);
        let eq142_e1809_d_n0: f64 = (eq142_e1807_d_n0 * p.p246);
        let eq142_e1809_d_n1: f64 = (eq142_e1807_d_n1 * p.p246);
        let eq142_e1809_d_n2: f64 = (eq142_e1807_d_n2 * p.p246);
        let eq142_e1809_d_n3: f64 = (eq142_e1807_d_n3 * p.p246);
        let eq142_e1809_d_n4: f64 = (eq142_e1807_d_n4 * p.p246);
        let eq142_e1809_d_n5: f64 = (eq142_e1807_d_n5 * p.p246);
        let eq142_e1809_d_n6: f64 = (eq142_e1807_d_n6 * p.p246);
        let eq142_e1809_d_n7: f64 = (eq142_e1807_d_n7 * p.p246);
        let eq142_e1809_d_n8: f64 = (eq142_e1807_d_n8 * p.p246);
        let eq142_e1809_d_n9: f64 = (eq142_e1807_d_n9 * p.p246);
        let eq142_e1809_d_n12: f64 = (eq142_e1807_d_n12 * p.p246);
        let eq142_e1809_d_n14: f64 = (eq142_e1807_d_n14 * p.p246);
        let eq142_e1809_d_n15: f64 = (eq142_e1807_d_n15 * p.p246);
        let eq142_e1809_d_n16: f64 = (eq142_e1807_d_n16 * p.p246);
        let eq142_e1809_d_n17: f64 = (eq142_e1807_d_n17 * p.p246);
        let eq142_e1809_d_n18: f64 = (eq142_e1807_d_n18 * p.p246);
        let eq142_e1809_d_n19: f64 = (eq142_e1807_d_n19 * p.p246);
        let eq142_e1809_d_n20: f64 = (eq142_e1807_d_n20 * p.p246);
        let eq142_e1809_d_n21: f64 = (eq142_e1807_d_n21 * p.p246);
        let eq142_e1809_d_n22: f64 = (eq142_e1807_d_n22 * p.p246);
        let eq142_e1809_q: f64 = (eq142_e1807_q * p.p246);
        (eq142_e1809, eq142_e1809_d_n0, eq142_e1809_d_n1, eq142_e1809_d_n2, eq142_e1809_d_n3, eq142_e1809_d_n4, eq142_e1809_d_n5, eq142_e1809_d_n6, eq142_e1809_d_n7, eq142_e1809_d_n8, eq142_e1809_d_n9, eq142_e1809_d_n12, eq142_e1809_d_n14, eq142_e1809_d_n15, eq142_e1809_d_n16, eq142_e1809_d_n17, eq142_e1809_d_n18, eq142_e1809_d_n19, eq142_e1809_d_n20, eq142_e1809_d_n21, eq142_e1809_d_n22, eq142_e1809_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq142_reactive_node_derivatives: [f64; 23] = [eq142_e1811_d_n0, eq142_e1811_d_n1, eq142_e1811_d_n2, eq142_e1811_d_n3, eq142_e1811_d_n4, eq142_e1811_d_n5, eq142_e1811_d_n6, eq142_e1811_d_n7, eq142_e1811_d_n8, eq142_e1811_d_n9, 0.0, 0.0, eq142_e1811_d_n12, 0.0, eq142_e1811_d_n14, eq142_e1811_d_n15, eq142_e1811_d_n16, eq142_e1811_d_n17, eq142_e1811_d_n18, eq142_e1811_d_n19, eq142_e1811_d_n20, eq142_e1811_d_n21, eq142_e1811_d_n22];
        let eq142_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq142_reactive_node_derivatives,
            branches,
            &eq142_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq143_e1823, eq143_e1823_d_n0, eq143_e1823_d_n1, eq143_e1823_d_n2, eq143_e1823_d_n3, eq143_e1823_d_n4, eq143_e1823_d_n5, eq143_e1823_d_n6, eq143_e1823_d_n7, eq143_e1823_d_n8, eq143_e1823_d_n9, eq143_e1823_d_n12, eq143_e1823_d_n14, eq143_e1823_d_n15, eq143_e1823_d_n16, eq143_e1823_d_n17, eq143_e1823_d_n18, eq143_e1823_d_n19, eq143_e1823_d_n20, eq143_e1823_d_n21, eq143_e1823_d_n22, eq143_e1823_q,) = {
    if ((var_guard541 == 0.0) && (var_guard544 != 0.0)) {
        let eq143_e1819: f64 = (p.p251 * var_qg_fp1s);
        let eq143_e1819_d_n0: f64 = (p.p251 * var_qg_fp1s_dn0);
        let eq143_e1819_d_n1: f64 = (p.p251 * var_qg_fp1s_dn1);
        let eq143_e1819_d_n2: f64 = (p.p251 * var_qg_fp1s_dn2);
        let eq143_e1819_d_n3: f64 = (p.p251 * var_qg_fp1s_dn3);
        let eq143_e1819_d_n4: f64 = (p.p251 * var_qg_fp1s_dn4);
        let eq143_e1819_d_n5: f64 = (p.p251 * var_qg_fp1s_dn5);
        let eq143_e1819_d_n6: f64 = (p.p251 * var_qg_fp1s_dn6);
        let eq143_e1819_d_n7: f64 = (p.p251 * var_qg_fp1s_dn7);
        let eq143_e1819_d_n8: f64 = (p.p251 * var_qg_fp1s_dn8);
        let eq143_e1819_d_n9: f64 = (p.p251 * var_qg_fp1s_dn9);
        let eq143_e1819_d_n12: f64 = (p.p251 * var_qg_fp1s_dn12);
        let eq143_e1819_d_n14: f64 = (p.p251 * var_qg_fp1s_dn14);
        let eq143_e1819_d_n15: f64 = (p.p251 * var_qg_fp1s_dn15);
        let eq143_e1819_d_n16: f64 = (p.p251 * var_qg_fp1s_dn16);
        let eq143_e1819_d_n17: f64 = (p.p251 * var_qg_fp1s_dn17);
        let eq143_e1819_d_n18: f64 = (p.p251 * var_qg_fp1s_dn18);
        let eq143_e1819_d_n19: f64 = (p.p251 * var_qg_fp1s_dn19);
        let eq143_e1819_d_n20: f64 = (p.p251 * var_qg_fp1s_dn20);
        let eq143_e1819_d_n21: f64 = (p.p251 * var_qg_fp1s_dn21);
        let eq143_e1819_d_n22: f64 = (p.p251 * var_qg_fp1s_dn22);
        let eq143_e1820_q: f64 = eq143_e1819;
        let eq143_e1821: f64 = (p.p7 * eq143_e1819);
        let eq143_e1821_d_n0: f64 = (p.p7 * eq143_e1819_d_n0);
        let eq143_e1821_d_n1: f64 = (p.p7 * eq143_e1819_d_n1);
        let eq143_e1821_d_n2: f64 = (p.p7 * eq143_e1819_d_n2);
        let eq143_e1821_d_n3: f64 = (p.p7 * eq143_e1819_d_n3);
        let eq143_e1821_d_n4: f64 = (p.p7 * eq143_e1819_d_n4);
        let eq143_e1821_d_n5: f64 = (p.p7 * eq143_e1819_d_n5);
        let eq143_e1821_d_n6: f64 = (p.p7 * eq143_e1819_d_n6);
        let eq143_e1821_d_n7: f64 = (p.p7 * eq143_e1819_d_n7);
        let eq143_e1821_d_n8: f64 = (p.p7 * eq143_e1819_d_n8);
        let eq143_e1821_d_n9: f64 = (p.p7 * eq143_e1819_d_n9);
        let eq143_e1821_d_n12: f64 = (p.p7 * eq143_e1819_d_n12);
        let eq143_e1821_d_n14: f64 = (p.p7 * eq143_e1819_d_n14);
        let eq143_e1821_d_n15: f64 = (p.p7 * eq143_e1819_d_n15);
        let eq143_e1821_d_n16: f64 = (p.p7 * eq143_e1819_d_n16);
        let eq143_e1821_d_n17: f64 = (p.p7 * eq143_e1819_d_n17);
        let eq143_e1821_d_n18: f64 = (p.p7 * eq143_e1819_d_n18);
        let eq143_e1821_d_n19: f64 = (p.p7 * eq143_e1819_d_n19);
        let eq143_e1821_d_n20: f64 = (p.p7 * eq143_e1819_d_n20);
        let eq143_e1821_d_n21: f64 = (p.p7 * eq143_e1819_d_n21);
        let eq143_e1821_d_n22: f64 = (p.p7 * eq143_e1819_d_n22);
        let eq143_e1821_q: f64 = (p.p7 * eq143_e1820_q);
        (eq143_e1821, eq143_e1821_d_n0, eq143_e1821_d_n1, eq143_e1821_d_n2, eq143_e1821_d_n3, eq143_e1821_d_n4, eq143_e1821_d_n5, eq143_e1821_d_n6, eq143_e1821_d_n7, eq143_e1821_d_n8, eq143_e1821_d_n9, eq143_e1821_d_n12, eq143_e1821_d_n14, eq143_e1821_d_n15, eq143_e1821_d_n16, eq143_e1821_d_n17, eq143_e1821_d_n18, eq143_e1821_d_n19, eq143_e1821_d_n20, eq143_e1821_d_n21, eq143_e1821_d_n22, eq143_e1821_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq143_reactive_node_derivatives: [f64; 23] = [eq143_e1823_d_n0, eq143_e1823_d_n1, eq143_e1823_d_n2, eq143_e1823_d_n3, eq143_e1823_d_n4, eq143_e1823_d_n5, eq143_e1823_d_n6, eq143_e1823_d_n7, eq143_e1823_d_n8, eq143_e1823_d_n9, 0.0, 0.0, eq143_e1823_d_n12, 0.0, eq143_e1823_d_n14, eq143_e1823_d_n15, eq143_e1823_d_n16, eq143_e1823_d_n17, eq143_e1823_d_n18, eq143_e1823_d_n19, eq143_e1823_d_n20, eq143_e1823_d_n21, eq143_e1823_d_n22];
        let eq143_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            nodes,
            &eq143_reactive_node_derivatives,
            branches,
            &eq143_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq144_e1832, eq144_e1832_d_n0, eq144_e1832_d_n1, eq144_e1832_d_n2, eq144_e1832_d_n3, eq144_e1832_d_n4, eq144_e1832_d_n5, eq144_e1832_d_n6, eq144_e1832_d_n7, eq144_e1832_d_n8, eq144_e1832_d_n9, eq144_e1832_d_n12, eq144_e1832_d_n14, eq144_e1832_d_n15, eq144_e1832_d_n16, eq144_e1832_d_n17, eq144_e1832_d_n18, eq144_e1832_d_n19, eq144_e1832_d_n20, eq144_e1832_d_n21, eq144_e1832_d_n22, eq144_e1832_q,) = {
    if ((var_guard546 != 0.0) && (var_guard547 != 0.0)) {
        let eq144_e1829_q: f64 = var_qd_fp2;
        let eq144_e1830: f64 = (p.p7 * var_qd_fp2);
        let eq144_e1830_d_n0: f64 = (p.p7 * var_qd_fp2_dn0);
        let eq144_e1830_d_n1: f64 = (p.p7 * var_qd_fp2_dn1);
        let eq144_e1830_d_n2: f64 = (p.p7 * var_qd_fp2_dn2);
        let eq144_e1830_d_n3: f64 = (p.p7 * var_qd_fp2_dn3);
        let eq144_e1830_d_n4: f64 = (p.p7 * var_qd_fp2_dn4);
        let eq144_e1830_d_n5: f64 = (p.p7 * var_qd_fp2_dn5);
        let eq144_e1830_d_n6: f64 = (p.p7 * var_qd_fp2_dn6);
        let eq144_e1830_d_n7: f64 = (p.p7 * var_qd_fp2_dn7);
        let eq144_e1830_d_n8: f64 = (p.p7 * var_qd_fp2_dn8);
        let eq144_e1830_d_n9: f64 = (p.p7 * var_qd_fp2_dn9);
        let eq144_e1830_d_n12: f64 = (p.p7 * var_qd_fp2_dn12);
        let eq144_e1830_d_n14: f64 = (p.p7 * var_qd_fp2_dn14);
        let eq144_e1830_d_n15: f64 = (p.p7 * var_qd_fp2_dn15);
        let eq144_e1830_d_n16: f64 = (p.p7 * var_qd_fp2_dn16);
        let eq144_e1830_d_n17: f64 = (p.p7 * var_qd_fp2_dn17);
        let eq144_e1830_d_n18: f64 = (p.p7 * var_qd_fp2_dn18);
        let eq144_e1830_d_n19: f64 = (p.p7 * var_qd_fp2_dn19);
        let eq144_e1830_d_n20: f64 = (p.p7 * var_qd_fp2_dn20);
        let eq144_e1830_d_n21: f64 = (p.p7 * var_qd_fp2_dn21);
        let eq144_e1830_d_n22: f64 = (p.p7 * var_qd_fp2_dn22);
        let eq144_e1830_q: f64 = (p.p7 * eq144_e1829_q);
        (eq144_e1830, eq144_e1830_d_n0, eq144_e1830_d_n1, eq144_e1830_d_n2, eq144_e1830_d_n3, eq144_e1830_d_n4, eq144_e1830_d_n5, eq144_e1830_d_n6, eq144_e1830_d_n7, eq144_e1830_d_n8, eq144_e1830_d_n9, eq144_e1830_d_n12, eq144_e1830_d_n14, eq144_e1830_d_n15, eq144_e1830_d_n16, eq144_e1830_d_n17, eq144_e1830_d_n18, eq144_e1830_d_n19, eq144_e1830_d_n20, eq144_e1830_d_n21, eq144_e1830_d_n22, eq144_e1830_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq144_reactive_node_derivatives: [f64; 23] = [eq144_e1832_d_n0, eq144_e1832_d_n1, eq144_e1832_d_n2, eq144_e1832_d_n3, eq144_e1832_d_n4, eq144_e1832_d_n5, eq144_e1832_d_n6, eq144_e1832_d_n7, eq144_e1832_d_n8, eq144_e1832_d_n9, 0.0, 0.0, eq144_e1832_d_n12, 0.0, eq144_e1832_d_n14, eq144_e1832_d_n15, eq144_e1832_d_n16, eq144_e1832_d_n17, eq144_e1832_d_n18, eq144_e1832_d_n19, eq144_e1832_d_n20, eq144_e1832_d_n21, eq144_e1832_d_n22];
        let eq144_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[16]),
            Some(nodes[15]),
            nodes,
            &eq144_reactive_node_derivatives,
            branches,
            &eq144_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq145_e1843, eq145_e1843_d_n0, eq145_e1843_d_n1, eq145_e1843_d_n2, eq145_e1843_d_n3, eq145_e1843_d_n4, eq145_e1843_d_n5, eq145_e1843_d_n6, eq145_e1843_d_n7, eq145_e1843_d_n8, eq145_e1843_d_n9, eq145_e1843_d_n12, eq145_e1843_d_n14, eq145_e1843_d_n15, eq145_e1843_d_n16, eq145_e1843_d_n17, eq145_e1843_d_n18, eq145_e1843_d_n19, eq145_e1843_d_n20, eq145_e1843_d_n21, eq145_e1843_d_n22, eq145_e1843_q,) = {
    if (((var_guard546 != 0.0) && (var_guard547 != 0.0)) && (var_guard548 != 0.0)) {
        let eq145_e1840_q: f64 = var_qg_fp2;
        let eq145_e1841: f64 = (p.p7 * var_qg_fp2);
        let eq145_e1841_d_n0: f64 = (p.p7 * var_qg_fp2_dn0);
        let eq145_e1841_d_n1: f64 = (p.p7 * var_qg_fp2_dn1);
        let eq145_e1841_d_n2: f64 = (p.p7 * var_qg_fp2_dn2);
        let eq145_e1841_d_n3: f64 = (p.p7 * var_qg_fp2_dn3);
        let eq145_e1841_d_n4: f64 = (p.p7 * var_qg_fp2_dn4);
        let eq145_e1841_d_n5: f64 = (p.p7 * var_qg_fp2_dn5);
        let eq145_e1841_d_n6: f64 = (p.p7 * var_qg_fp2_dn6);
        let eq145_e1841_d_n7: f64 = (p.p7 * var_qg_fp2_dn7);
        let eq145_e1841_d_n8: f64 = (p.p7 * var_qg_fp2_dn8);
        let eq145_e1841_d_n9: f64 = (p.p7 * var_qg_fp2_dn9);
        let eq145_e1841_d_n12: f64 = (p.p7 * var_qg_fp2_dn12);
        let eq145_e1841_d_n14: f64 = (p.p7 * var_qg_fp2_dn14);
        let eq145_e1841_d_n15: f64 = (p.p7 * var_qg_fp2_dn15);
        let eq145_e1841_d_n16: f64 = (p.p7 * var_qg_fp2_dn16);
        let eq145_e1841_d_n17: f64 = (p.p7 * var_qg_fp2_dn17);
        let eq145_e1841_d_n18: f64 = (p.p7 * var_qg_fp2_dn18);
        let eq145_e1841_d_n19: f64 = (p.p7 * var_qg_fp2_dn19);
        let eq145_e1841_d_n20: f64 = (p.p7 * var_qg_fp2_dn20);
        let eq145_e1841_d_n21: f64 = (p.p7 * var_qg_fp2_dn21);
        let eq145_e1841_d_n22: f64 = (p.p7 * var_qg_fp2_dn22);
        let eq145_e1841_q: f64 = (p.p7 * eq145_e1840_q);
        (eq145_e1841, eq145_e1841_d_n0, eq145_e1841_d_n1, eq145_e1841_d_n2, eq145_e1841_d_n3, eq145_e1841_d_n4, eq145_e1841_d_n5, eq145_e1841_d_n6, eq145_e1841_d_n7, eq145_e1841_d_n8, eq145_e1841_d_n9, eq145_e1841_d_n12, eq145_e1841_d_n14, eq145_e1841_d_n15, eq145_e1841_d_n16, eq145_e1841_d_n17, eq145_e1841_d_n18, eq145_e1841_d_n19, eq145_e1841_d_n20, eq145_e1841_d_n21, eq145_e1841_d_n22, eq145_e1841_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq145_reactive_node_derivatives: [f64; 23] = [eq145_e1843_d_n0, eq145_e1843_d_n1, eq145_e1843_d_n2, eq145_e1843_d_n3, eq145_e1843_d_n4, eq145_e1843_d_n5, eq145_e1843_d_n6, eq145_e1843_d_n7, eq145_e1843_d_n8, eq145_e1843_d_n9, 0.0, 0.0, eq145_e1843_d_n12, 0.0, eq145_e1843_d_n14, eq145_e1843_d_n15, eq145_e1843_d_n16, eq145_e1843_d_n17, eq145_e1843_d_n18, eq145_e1843_d_n19, eq145_e1843_d_n20, eq145_e1843_d_n21, eq145_e1843_d_n22];
        let eq145_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[15]),
            nodes,
            &eq145_reactive_node_derivatives,
            branches,
            &eq145_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq146_e1856, eq146_e1856_d_n0, eq146_e1856_d_n1, eq146_e1856_d_n2, eq146_e1856_d_n3, eq146_e1856_d_n4, eq146_e1856_d_n5, eq146_e1856_d_n6, eq146_e1856_d_n7, eq146_e1856_d_n8, eq146_e1856_d_n9, eq146_e1856_d_n12, eq146_e1856_d_n14, eq146_e1856_d_n15, eq146_e1856_d_n16, eq146_e1856_d_n17, eq146_e1856_d_n18, eq146_e1856_d_n19, eq146_e1856_d_n20, eq146_e1856_d_n21, eq146_e1856_d_n22, eq146_e1856_q,) = {
    if (((var_guard546 != 0.0) && (var_guard547 != 0.0)) && (var_guard548 != 0.0)) {
        let eq146_e1851: f64 = (p.p7 * p.p247);
        let eq146_e1853_q: f64 = var_qg_fp2;
        let eq146_e1854: f64 = (eq146_e1851 * var_qg_fp2);
        let eq146_e1854_d_n0: f64 = (eq146_e1851 * var_qg_fp2_dn0);
        let eq146_e1854_d_n1: f64 = (eq146_e1851 * var_qg_fp2_dn1);
        let eq146_e1854_d_n2: f64 = (eq146_e1851 * var_qg_fp2_dn2);
        let eq146_e1854_d_n3: f64 = (eq146_e1851 * var_qg_fp2_dn3);
        let eq146_e1854_d_n4: f64 = (eq146_e1851 * var_qg_fp2_dn4);
        let eq146_e1854_d_n5: f64 = (eq146_e1851 * var_qg_fp2_dn5);
        let eq146_e1854_d_n6: f64 = (eq146_e1851 * var_qg_fp2_dn6);
        let eq146_e1854_d_n7: f64 = (eq146_e1851 * var_qg_fp2_dn7);
        let eq146_e1854_d_n8: f64 = (eq146_e1851 * var_qg_fp2_dn8);
        let eq146_e1854_d_n9: f64 = (eq146_e1851 * var_qg_fp2_dn9);
        let eq146_e1854_d_n12: f64 = (eq146_e1851 * var_qg_fp2_dn12);
        let eq146_e1854_d_n14: f64 = (eq146_e1851 * var_qg_fp2_dn14);
        let eq146_e1854_d_n15: f64 = (eq146_e1851 * var_qg_fp2_dn15);
        let eq146_e1854_d_n16: f64 = (eq146_e1851 * var_qg_fp2_dn16);
        let eq146_e1854_d_n17: f64 = (eq146_e1851 * var_qg_fp2_dn17);
        let eq146_e1854_d_n18: f64 = (eq146_e1851 * var_qg_fp2_dn18);
        let eq146_e1854_d_n19: f64 = (eq146_e1851 * var_qg_fp2_dn19);
        let eq146_e1854_d_n20: f64 = (eq146_e1851 * var_qg_fp2_dn20);
        let eq146_e1854_d_n21: f64 = (eq146_e1851 * var_qg_fp2_dn21);
        let eq146_e1854_d_n22: f64 = (eq146_e1851 * var_qg_fp2_dn22);
        let eq146_e1854_q: f64 = (eq146_e1851 * eq146_e1853_q);
        (eq146_e1854, eq146_e1854_d_n0, eq146_e1854_d_n1, eq146_e1854_d_n2, eq146_e1854_d_n3, eq146_e1854_d_n4, eq146_e1854_d_n5, eq146_e1854_d_n6, eq146_e1854_d_n7, eq146_e1854_d_n8, eq146_e1854_d_n9, eq146_e1854_d_n12, eq146_e1854_d_n14, eq146_e1854_d_n15, eq146_e1854_d_n16, eq146_e1854_d_n17, eq146_e1854_d_n18, eq146_e1854_d_n19, eq146_e1854_d_n20, eq146_e1854_d_n21, eq146_e1854_d_n22, eq146_e1854_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq146_reactive_node_derivatives: [f64; 23] = [eq146_e1856_d_n0, eq146_e1856_d_n1, eq146_e1856_d_n2, eq146_e1856_d_n3, eq146_e1856_d_n4, eq146_e1856_d_n5, eq146_e1856_d_n6, eq146_e1856_d_n7, eq146_e1856_d_n8, eq146_e1856_d_n9, 0.0, 0.0, eq146_e1856_d_n12, 0.0, eq146_e1856_d_n14, eq146_e1856_d_n15, eq146_e1856_d_n16, eq146_e1856_d_n17, eq146_e1856_d_n18, eq146_e1856_d_n19, eq146_e1856_d_n20, eq146_e1856_d_n21, eq146_e1856_d_n22];
        let eq146_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            nodes,
            &eq146_reactive_node_derivatives,
            branches,
            &eq146_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq147_e1868, eq147_e1868_d_n0, eq147_e1868_d_n1, eq147_e1868_d_n2, eq147_e1868_d_n3, eq147_e1868_d_n4, eq147_e1868_d_n5, eq147_e1868_d_n6, eq147_e1868_d_n7, eq147_e1868_d_n8, eq147_e1868_d_n9, eq147_e1868_d_n12, eq147_e1868_d_n14, eq147_e1868_d_n15, eq147_e1868_d_n16, eq147_e1868_d_n17, eq147_e1868_d_n18, eq147_e1868_d_n19, eq147_e1868_d_n20, eq147_e1868_d_n21, eq147_e1868_d_n22, eq147_e1868_q,) = {
    if (((var_guard546 != 0.0) && (var_guard547 != 0.0)) && (var_guard548 == 0.0)) {
        let eq147_e1865_q: f64 = var_qg_fp2;
        let eq147_e1866: f64 = (p.p7 * var_qg_fp2);
        let eq147_e1866_d_n0: f64 = (p.p7 * var_qg_fp2_dn0);
        let eq147_e1866_d_n1: f64 = (p.p7 * var_qg_fp2_dn1);
        let eq147_e1866_d_n2: f64 = (p.p7 * var_qg_fp2_dn2);
        let eq147_e1866_d_n3: f64 = (p.p7 * var_qg_fp2_dn3);
        let eq147_e1866_d_n4: f64 = (p.p7 * var_qg_fp2_dn4);
        let eq147_e1866_d_n5: f64 = (p.p7 * var_qg_fp2_dn5);
        let eq147_e1866_d_n6: f64 = (p.p7 * var_qg_fp2_dn6);
        let eq147_e1866_d_n7: f64 = (p.p7 * var_qg_fp2_dn7);
        let eq147_e1866_d_n8: f64 = (p.p7 * var_qg_fp2_dn8);
        let eq147_e1866_d_n9: f64 = (p.p7 * var_qg_fp2_dn9);
        let eq147_e1866_d_n12: f64 = (p.p7 * var_qg_fp2_dn12);
        let eq147_e1866_d_n14: f64 = (p.p7 * var_qg_fp2_dn14);
        let eq147_e1866_d_n15: f64 = (p.p7 * var_qg_fp2_dn15);
        let eq147_e1866_d_n16: f64 = (p.p7 * var_qg_fp2_dn16);
        let eq147_e1866_d_n17: f64 = (p.p7 * var_qg_fp2_dn17);
        let eq147_e1866_d_n18: f64 = (p.p7 * var_qg_fp2_dn18);
        let eq147_e1866_d_n19: f64 = (p.p7 * var_qg_fp2_dn19);
        let eq147_e1866_d_n20: f64 = (p.p7 * var_qg_fp2_dn20);
        let eq147_e1866_d_n21: f64 = (p.p7 * var_qg_fp2_dn21);
        let eq147_e1866_d_n22: f64 = (p.p7 * var_qg_fp2_dn22);
        let eq147_e1866_q: f64 = (p.p7 * eq147_e1865_q);
        (eq147_e1866, eq147_e1866_d_n0, eq147_e1866_d_n1, eq147_e1866_d_n2, eq147_e1866_d_n3, eq147_e1866_d_n4, eq147_e1866_d_n5, eq147_e1866_d_n6, eq147_e1866_d_n7, eq147_e1866_d_n8, eq147_e1866_d_n9, eq147_e1866_d_n12, eq147_e1866_d_n14, eq147_e1866_d_n15, eq147_e1866_d_n16, eq147_e1866_d_n17, eq147_e1866_d_n18, eq147_e1866_d_n19, eq147_e1866_d_n20, eq147_e1866_d_n21, eq147_e1866_d_n22, eq147_e1866_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq147_reactive_node_derivatives: [f64; 23] = [eq147_e1868_d_n0, eq147_e1868_d_n1, eq147_e1868_d_n2, eq147_e1868_d_n3, eq147_e1868_d_n4, eq147_e1868_d_n5, eq147_e1868_d_n6, eq147_e1868_d_n7, eq147_e1868_d_n8, eq147_e1868_d_n9, 0.0, 0.0, eq147_e1868_d_n12, 0.0, eq147_e1868_d_n14, eq147_e1868_d_n15, eq147_e1868_d_n16, eq147_e1868_d_n17, eq147_e1868_d_n18, eq147_e1868_d_n19, eq147_e1868_d_n20, eq147_e1868_d_n21, eq147_e1868_d_n22];
        let eq147_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            nodes,
            &eq147_reactive_node_derivatives,
            branches,
            &eq147_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq148_e1882, eq148_e1882_d_n0, eq148_e1882_d_n1, eq148_e1882_d_n2, eq148_e1882_d_n3, eq148_e1882_d_n4, eq148_e1882_d_n5, eq148_e1882_d_n6, eq148_e1882_d_n7, eq148_e1882_d_n8, eq148_e1882_d_n9, eq148_e1882_d_n12, eq148_e1882_d_n14, eq148_e1882_d_n15, eq148_e1882_d_n16, eq148_e1882_d_n17, eq148_e1882_d_n18, eq148_e1882_d_n19, eq148_e1882_d_n20, eq148_e1882_d_n21, eq148_e1882_d_n22, eq148_e1882_q,) = {
    if (((var_guard546 != 0.0) && (var_guard547 != 0.0)) && (var_guard548 == 0.0)) {
        let eq148_e1877: f64 = (p.p7 * p.p247);
        let eq148_e1879_q: f64 = var_qg_fp2;
        let eq148_e1880: f64 = (eq148_e1877 * var_qg_fp2);
        let eq148_e1880_d_n0: f64 = (eq148_e1877 * var_qg_fp2_dn0);
        let eq148_e1880_d_n1: f64 = (eq148_e1877 * var_qg_fp2_dn1);
        let eq148_e1880_d_n2: f64 = (eq148_e1877 * var_qg_fp2_dn2);
        let eq148_e1880_d_n3: f64 = (eq148_e1877 * var_qg_fp2_dn3);
        let eq148_e1880_d_n4: f64 = (eq148_e1877 * var_qg_fp2_dn4);
        let eq148_e1880_d_n5: f64 = (eq148_e1877 * var_qg_fp2_dn5);
        let eq148_e1880_d_n6: f64 = (eq148_e1877 * var_qg_fp2_dn6);
        let eq148_e1880_d_n7: f64 = (eq148_e1877 * var_qg_fp2_dn7);
        let eq148_e1880_d_n8: f64 = (eq148_e1877 * var_qg_fp2_dn8);
        let eq148_e1880_d_n9: f64 = (eq148_e1877 * var_qg_fp2_dn9);
        let eq148_e1880_d_n12: f64 = (eq148_e1877 * var_qg_fp2_dn12);
        let eq148_e1880_d_n14: f64 = (eq148_e1877 * var_qg_fp2_dn14);
        let eq148_e1880_d_n15: f64 = (eq148_e1877 * var_qg_fp2_dn15);
        let eq148_e1880_d_n16: f64 = (eq148_e1877 * var_qg_fp2_dn16);
        let eq148_e1880_d_n17: f64 = (eq148_e1877 * var_qg_fp2_dn17);
        let eq148_e1880_d_n18: f64 = (eq148_e1877 * var_qg_fp2_dn18);
        let eq148_e1880_d_n19: f64 = (eq148_e1877 * var_qg_fp2_dn19);
        let eq148_e1880_d_n20: f64 = (eq148_e1877 * var_qg_fp2_dn20);
        let eq148_e1880_d_n21: f64 = (eq148_e1877 * var_qg_fp2_dn21);
        let eq148_e1880_d_n22: f64 = (eq148_e1877 * var_qg_fp2_dn22);
        let eq148_e1880_q: f64 = (eq148_e1877 * eq148_e1879_q);
        (eq148_e1880, eq148_e1880_d_n0, eq148_e1880_d_n1, eq148_e1880_d_n2, eq148_e1880_d_n3, eq148_e1880_d_n4, eq148_e1880_d_n5, eq148_e1880_d_n6, eq148_e1880_d_n7, eq148_e1880_d_n8, eq148_e1880_d_n9, eq148_e1880_d_n12, eq148_e1880_d_n14, eq148_e1880_d_n15, eq148_e1880_d_n16, eq148_e1880_d_n17, eq148_e1880_d_n18, eq148_e1880_d_n19, eq148_e1880_d_n20, eq148_e1880_d_n21, eq148_e1880_d_n22, eq148_e1880_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq148_reactive_node_derivatives: [f64; 23] = [eq148_e1882_d_n0, eq148_e1882_d_n1, eq148_e1882_d_n2, eq148_e1882_d_n3, eq148_e1882_d_n4, eq148_e1882_d_n5, eq148_e1882_d_n6, eq148_e1882_d_n7, eq148_e1882_d_n8, eq148_e1882_d_n9, 0.0, 0.0, eq148_e1882_d_n12, 0.0, eq148_e1882_d_n14, eq148_e1882_d_n15, eq148_e1882_d_n16, eq148_e1882_d_n17, eq148_e1882_d_n18, eq148_e1882_d_n19, eq148_e1882_d_n20, eq148_e1882_d_n21, eq148_e1882_d_n22];
        let eq148_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[15]),
            nodes,
            &eq148_reactive_node_derivatives,
            branches,
            &eq148_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_4(
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_guard546: f64,
        var_guard547: f64,
        var_guard549: f64,
        var_guard550: f64,
        var_guard551: f64,
        var_guard552: f64,
        var_guard553: f64,
        var_qd_fp2: f64,
        var_qd_fp2_dn0: f64,
        var_qd_fp2_dn1: f64,
        var_qd_fp2_dn12: f64,
        var_qd_fp2_dn14: f64,
        var_qd_fp2_dn15: f64,
        var_qd_fp2_dn16: f64,
        var_qd_fp2_dn17: f64,
        var_qd_fp2_dn18: f64,
        var_qd_fp2_dn19: f64,
        var_qd_fp2_dn2: f64,
        var_qd_fp2_dn20: f64,
        var_qd_fp2_dn21: f64,
        var_qd_fp2_dn22: f64,
        var_qd_fp2_dn3: f64,
        var_qd_fp2_dn4: f64,
        var_qd_fp2_dn5: f64,
        var_qd_fp2_dn6: f64,
        var_qd_fp2_dn7: f64,
        var_qd_fp2_dn8: f64,
        var_qd_fp2_dn9: f64,
        var_qd_fp2s: f64,
        var_qd_fp2s_dn0: f64,
        var_qd_fp2s_dn1: f64,
        var_qd_fp2s_dn12: f64,
        var_qd_fp2s_dn14: f64,
        var_qd_fp2s_dn15: f64,
        var_qd_fp2s_dn16: f64,
        var_qd_fp2s_dn17: f64,
        var_qd_fp2s_dn18: f64,
        var_qd_fp2s_dn19: f64,
        var_qd_fp2s_dn2: f64,
        var_qd_fp2s_dn20: f64,
        var_qd_fp2s_dn21: f64,
        var_qd_fp2s_dn22: f64,
        var_qd_fp2s_dn3: f64,
        var_qd_fp2s_dn4: f64,
        var_qd_fp2s_dn5: f64,
        var_qd_fp2s_dn6: f64,
        var_qd_fp2s_dn7: f64,
        var_qd_fp2s_dn8: f64,
        var_qd_fp2s_dn9: f64,
        var_qg_fp2: f64,
        var_qg_fp2_dn0: f64,
        var_qg_fp2_dn1: f64,
        var_qg_fp2_dn12: f64,
        var_qg_fp2_dn14: f64,
        var_qg_fp2_dn15: f64,
        var_qg_fp2_dn16: f64,
        var_qg_fp2_dn17: f64,
        var_qg_fp2_dn18: f64,
        var_qg_fp2_dn19: f64,
        var_qg_fp2_dn2: f64,
        var_qg_fp2_dn20: f64,
        var_qg_fp2_dn21: f64,
        var_qg_fp2_dn22: f64,
        var_qg_fp2_dn3: f64,
        var_qg_fp2_dn4: f64,
        var_qg_fp2_dn5: f64,
        var_qg_fp2_dn6: f64,
        var_qg_fp2_dn7: f64,
        var_qg_fp2_dn8: f64,
        var_qg_fp2_dn9: f64,
        var_qg_fp2s: f64,
        var_qg_fp2s_dn0: f64,
        var_qg_fp2s_dn1: f64,
        var_qg_fp2s_dn12: f64,
        var_qg_fp2s_dn14: f64,
        var_qg_fp2s_dn15: f64,
        var_qg_fp2s_dn16: f64,
        var_qg_fp2s_dn17: f64,
        var_qg_fp2s_dn18: f64,
        var_qg_fp2s_dn19: f64,
        var_qg_fp2s_dn2: f64,
        var_qg_fp2s_dn20: f64,
        var_qg_fp2s_dn21: f64,
        var_qg_fp2s_dn22: f64,
        var_qg_fp2s_dn3: f64,
        var_qg_fp2s_dn4: f64,
        var_qg_fp2s_dn5: f64,
        var_qg_fp2s_dn6: f64,
        var_qg_fp2s_dn7: f64,
        var_qg_fp2s_dn8: f64,
        var_qg_fp2s_dn9: f64,
    ) {
        let (eq149_e1893, eq149_e1893_d_n0, eq149_e1893_d_n1, eq149_e1893_d_n2, eq149_e1893_d_n3, eq149_e1893_d_n4, eq149_e1893_d_n5, eq149_e1893_d_n6, eq149_e1893_d_n7, eq149_e1893_d_n8, eq149_e1893_d_n9, eq149_e1893_d_n12, eq149_e1893_d_n14, eq149_e1893_d_n15, eq149_e1893_d_n16, eq149_e1893_d_n17, eq149_e1893_d_n18, eq149_e1893_d_n19, eq149_e1893_d_n20, eq149_e1893_d_n21, eq149_e1893_d_n22, eq149_e1893_q,) = {
    if ((var_guard546 != 0.0) && (var_guard547 != 0.0)) {
        let eq149_e1889: f64 = (p.p252 * var_qg_fp2);
        let eq149_e1889_d_n0: f64 = (p.p252 * var_qg_fp2_dn0);
        let eq149_e1889_d_n1: f64 = (p.p252 * var_qg_fp2_dn1);
        let eq149_e1889_d_n2: f64 = (p.p252 * var_qg_fp2_dn2);
        let eq149_e1889_d_n3: f64 = (p.p252 * var_qg_fp2_dn3);
        let eq149_e1889_d_n4: f64 = (p.p252 * var_qg_fp2_dn4);
        let eq149_e1889_d_n5: f64 = (p.p252 * var_qg_fp2_dn5);
        let eq149_e1889_d_n6: f64 = (p.p252 * var_qg_fp2_dn6);
        let eq149_e1889_d_n7: f64 = (p.p252 * var_qg_fp2_dn7);
        let eq149_e1889_d_n8: f64 = (p.p252 * var_qg_fp2_dn8);
        let eq149_e1889_d_n9: f64 = (p.p252 * var_qg_fp2_dn9);
        let eq149_e1889_d_n12: f64 = (p.p252 * var_qg_fp2_dn12);
        let eq149_e1889_d_n14: f64 = (p.p252 * var_qg_fp2_dn14);
        let eq149_e1889_d_n15: f64 = (p.p252 * var_qg_fp2_dn15);
        let eq149_e1889_d_n16: f64 = (p.p252 * var_qg_fp2_dn16);
        let eq149_e1889_d_n17: f64 = (p.p252 * var_qg_fp2_dn17);
        let eq149_e1889_d_n18: f64 = (p.p252 * var_qg_fp2_dn18);
        let eq149_e1889_d_n19: f64 = (p.p252 * var_qg_fp2_dn19);
        let eq149_e1889_d_n20: f64 = (p.p252 * var_qg_fp2_dn20);
        let eq149_e1889_d_n21: f64 = (p.p252 * var_qg_fp2_dn21);
        let eq149_e1889_d_n22: f64 = (p.p252 * var_qg_fp2_dn22);
        let eq149_e1890_q: f64 = eq149_e1889;
        let eq149_e1891: f64 = (p.p7 * eq149_e1889);
        let eq149_e1891_d_n0: f64 = (p.p7 * eq149_e1889_d_n0);
        let eq149_e1891_d_n1: f64 = (p.p7 * eq149_e1889_d_n1);
        let eq149_e1891_d_n2: f64 = (p.p7 * eq149_e1889_d_n2);
        let eq149_e1891_d_n3: f64 = (p.p7 * eq149_e1889_d_n3);
        let eq149_e1891_d_n4: f64 = (p.p7 * eq149_e1889_d_n4);
        let eq149_e1891_d_n5: f64 = (p.p7 * eq149_e1889_d_n5);
        let eq149_e1891_d_n6: f64 = (p.p7 * eq149_e1889_d_n6);
        let eq149_e1891_d_n7: f64 = (p.p7 * eq149_e1889_d_n7);
        let eq149_e1891_d_n8: f64 = (p.p7 * eq149_e1889_d_n8);
        let eq149_e1891_d_n9: f64 = (p.p7 * eq149_e1889_d_n9);
        let eq149_e1891_d_n12: f64 = (p.p7 * eq149_e1889_d_n12);
        let eq149_e1891_d_n14: f64 = (p.p7 * eq149_e1889_d_n14);
        let eq149_e1891_d_n15: f64 = (p.p7 * eq149_e1889_d_n15);
        let eq149_e1891_d_n16: f64 = (p.p7 * eq149_e1889_d_n16);
        let eq149_e1891_d_n17: f64 = (p.p7 * eq149_e1889_d_n17);
        let eq149_e1891_d_n18: f64 = (p.p7 * eq149_e1889_d_n18);
        let eq149_e1891_d_n19: f64 = (p.p7 * eq149_e1889_d_n19);
        let eq149_e1891_d_n20: f64 = (p.p7 * eq149_e1889_d_n20);
        let eq149_e1891_d_n21: f64 = (p.p7 * eq149_e1889_d_n21);
        let eq149_e1891_d_n22: f64 = (p.p7 * eq149_e1889_d_n22);
        let eq149_e1891_q: f64 = (p.p7 * eq149_e1890_q);
        (eq149_e1891, eq149_e1891_d_n0, eq149_e1891_d_n1, eq149_e1891_d_n2, eq149_e1891_d_n3, eq149_e1891_d_n4, eq149_e1891_d_n5, eq149_e1891_d_n6, eq149_e1891_d_n7, eq149_e1891_d_n8, eq149_e1891_d_n9, eq149_e1891_d_n12, eq149_e1891_d_n14, eq149_e1891_d_n15, eq149_e1891_d_n16, eq149_e1891_d_n17, eq149_e1891_d_n18, eq149_e1891_d_n19, eq149_e1891_d_n20, eq149_e1891_d_n21, eq149_e1891_d_n22, eq149_e1891_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq149_reactive_node_derivatives: [f64; 23] = [eq149_e1893_d_n0, eq149_e1893_d_n1, eq149_e1893_d_n2, eq149_e1893_d_n3, eq149_e1893_d_n4, eq149_e1893_d_n5, eq149_e1893_d_n6, eq149_e1893_d_n7, eq149_e1893_d_n8, eq149_e1893_d_n9, 0.0, 0.0, eq149_e1893_d_n12, 0.0, eq149_e1893_d_n14, eq149_e1893_d_n15, eq149_e1893_d_n16, eq149_e1893_d_n17, eq149_e1893_d_n18, eq149_e1893_d_n19, eq149_e1893_d_n20, eq149_e1893_d_n21, eq149_e1893_d_n22];
        let eq149_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[15]),
            nodes,
            &eq149_reactive_node_derivatives,
            branches,
            &eq149_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq150_e1903, eq150_e1903_d_n0, eq150_e1903_d_n1, eq150_e1903_d_n2, eq150_e1903_d_n3, eq150_e1903_d_n4, eq150_e1903_d_n5, eq150_e1903_d_n6, eq150_e1903_d_n7, eq150_e1903_d_n8, eq150_e1903_d_n9, eq150_e1903_d_n12, eq150_e1903_d_n14, eq150_e1903_d_n15, eq150_e1903_d_n16, eq150_e1903_d_n17, eq150_e1903_d_n18, eq150_e1903_d_n19, eq150_e1903_d_n20, eq150_e1903_d_n21, eq150_e1903_d_n22, eq150_e1903_q,) = {
    if ((var_guard546 == 0.0) && (var_guard549 != 0.0)) {
        let eq150_e1900_q: f64 = var_qd_fp2;
        let eq150_e1901: f64 = (p.p7 * var_qd_fp2);
        let eq150_e1901_d_n0: f64 = (p.p7 * var_qd_fp2_dn0);
        let eq150_e1901_d_n1: f64 = (p.p7 * var_qd_fp2_dn1);
        let eq150_e1901_d_n2: f64 = (p.p7 * var_qd_fp2_dn2);
        let eq150_e1901_d_n3: f64 = (p.p7 * var_qd_fp2_dn3);
        let eq150_e1901_d_n4: f64 = (p.p7 * var_qd_fp2_dn4);
        let eq150_e1901_d_n5: f64 = (p.p7 * var_qd_fp2_dn5);
        let eq150_e1901_d_n6: f64 = (p.p7 * var_qd_fp2_dn6);
        let eq150_e1901_d_n7: f64 = (p.p7 * var_qd_fp2_dn7);
        let eq150_e1901_d_n8: f64 = (p.p7 * var_qd_fp2_dn8);
        let eq150_e1901_d_n9: f64 = (p.p7 * var_qd_fp2_dn9);
        let eq150_e1901_d_n12: f64 = (p.p7 * var_qd_fp2_dn12);
        let eq150_e1901_d_n14: f64 = (p.p7 * var_qd_fp2_dn14);
        let eq150_e1901_d_n15: f64 = (p.p7 * var_qd_fp2_dn15);
        let eq150_e1901_d_n16: f64 = (p.p7 * var_qd_fp2_dn16);
        let eq150_e1901_d_n17: f64 = (p.p7 * var_qd_fp2_dn17);
        let eq150_e1901_d_n18: f64 = (p.p7 * var_qd_fp2_dn18);
        let eq150_e1901_d_n19: f64 = (p.p7 * var_qd_fp2_dn19);
        let eq150_e1901_d_n20: f64 = (p.p7 * var_qd_fp2_dn20);
        let eq150_e1901_d_n21: f64 = (p.p7 * var_qd_fp2_dn21);
        let eq150_e1901_d_n22: f64 = (p.p7 * var_qd_fp2_dn22);
        let eq150_e1901_q: f64 = (p.p7 * eq150_e1900_q);
        (eq150_e1901, eq150_e1901_d_n0, eq150_e1901_d_n1, eq150_e1901_d_n2, eq150_e1901_d_n3, eq150_e1901_d_n4, eq150_e1901_d_n5, eq150_e1901_d_n6, eq150_e1901_d_n7, eq150_e1901_d_n8, eq150_e1901_d_n9, eq150_e1901_d_n12, eq150_e1901_d_n14, eq150_e1901_d_n15, eq150_e1901_d_n16, eq150_e1901_d_n17, eq150_e1901_d_n18, eq150_e1901_d_n19, eq150_e1901_d_n20, eq150_e1901_d_n21, eq150_e1901_d_n22, eq150_e1901_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq150_reactive_node_derivatives: [f64; 23] = [eq150_e1903_d_n0, eq150_e1903_d_n1, eq150_e1903_d_n2, eq150_e1903_d_n3, eq150_e1903_d_n4, eq150_e1903_d_n5, eq150_e1903_d_n6, eq150_e1903_d_n7, eq150_e1903_d_n8, eq150_e1903_d_n9, 0.0, 0.0, eq150_e1903_d_n12, 0.0, eq150_e1903_d_n14, eq150_e1903_d_n15, eq150_e1903_d_n16, eq150_e1903_d_n17, eq150_e1903_d_n18, eq150_e1903_d_n19, eq150_e1903_d_n20, eq150_e1903_d_n21, eq150_e1903_d_n22];
        let eq150_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[7]),
            nodes,
            &eq150_reactive_node_derivatives,
            branches,
            &eq150_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq151_e1915, eq151_e1915_d_n0, eq151_e1915_d_n1, eq151_e1915_d_n2, eq151_e1915_d_n3, eq151_e1915_d_n4, eq151_e1915_d_n5, eq151_e1915_d_n6, eq151_e1915_d_n7, eq151_e1915_d_n8, eq151_e1915_d_n9, eq151_e1915_d_n12, eq151_e1915_d_n14, eq151_e1915_d_n15, eq151_e1915_d_n16, eq151_e1915_d_n17, eq151_e1915_d_n18, eq151_e1915_d_n19, eq151_e1915_d_n20, eq151_e1915_d_n21, eq151_e1915_d_n22, eq151_e1915_q,) = {
    if (((var_guard546 == 0.0) && (var_guard549 != 0.0)) && (var_guard550 != 0.0)) {
        let eq151_e1912_q: f64 = var_qg_fp2;
        let eq151_e1913: f64 = (p.p7 * var_qg_fp2);
        let eq151_e1913_d_n0: f64 = (p.p7 * var_qg_fp2_dn0);
        let eq151_e1913_d_n1: f64 = (p.p7 * var_qg_fp2_dn1);
        let eq151_e1913_d_n2: f64 = (p.p7 * var_qg_fp2_dn2);
        let eq151_e1913_d_n3: f64 = (p.p7 * var_qg_fp2_dn3);
        let eq151_e1913_d_n4: f64 = (p.p7 * var_qg_fp2_dn4);
        let eq151_e1913_d_n5: f64 = (p.p7 * var_qg_fp2_dn5);
        let eq151_e1913_d_n6: f64 = (p.p7 * var_qg_fp2_dn6);
        let eq151_e1913_d_n7: f64 = (p.p7 * var_qg_fp2_dn7);
        let eq151_e1913_d_n8: f64 = (p.p7 * var_qg_fp2_dn8);
        let eq151_e1913_d_n9: f64 = (p.p7 * var_qg_fp2_dn9);
        let eq151_e1913_d_n12: f64 = (p.p7 * var_qg_fp2_dn12);
        let eq151_e1913_d_n14: f64 = (p.p7 * var_qg_fp2_dn14);
        let eq151_e1913_d_n15: f64 = (p.p7 * var_qg_fp2_dn15);
        let eq151_e1913_d_n16: f64 = (p.p7 * var_qg_fp2_dn16);
        let eq151_e1913_d_n17: f64 = (p.p7 * var_qg_fp2_dn17);
        let eq151_e1913_d_n18: f64 = (p.p7 * var_qg_fp2_dn18);
        let eq151_e1913_d_n19: f64 = (p.p7 * var_qg_fp2_dn19);
        let eq151_e1913_d_n20: f64 = (p.p7 * var_qg_fp2_dn20);
        let eq151_e1913_d_n21: f64 = (p.p7 * var_qg_fp2_dn21);
        let eq151_e1913_d_n22: f64 = (p.p7 * var_qg_fp2_dn22);
        let eq151_e1913_q: f64 = (p.p7 * eq151_e1912_q);
        (eq151_e1913, eq151_e1913_d_n0, eq151_e1913_d_n1, eq151_e1913_d_n2, eq151_e1913_d_n3, eq151_e1913_d_n4, eq151_e1913_d_n5, eq151_e1913_d_n6, eq151_e1913_d_n7, eq151_e1913_d_n8, eq151_e1913_d_n9, eq151_e1913_d_n12, eq151_e1913_d_n14, eq151_e1913_d_n15, eq151_e1913_d_n16, eq151_e1913_d_n17, eq151_e1913_d_n18, eq151_e1913_d_n19, eq151_e1913_d_n20, eq151_e1913_d_n21, eq151_e1913_d_n22, eq151_e1913_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq151_reactive_node_derivatives: [f64; 23] = [eq151_e1915_d_n0, eq151_e1915_d_n1, eq151_e1915_d_n2, eq151_e1915_d_n3, eq151_e1915_d_n4, eq151_e1915_d_n5, eq151_e1915_d_n6, eq151_e1915_d_n7, eq151_e1915_d_n8, eq151_e1915_d_n9, 0.0, 0.0, eq151_e1915_d_n12, 0.0, eq151_e1915_d_n14, eq151_e1915_d_n15, eq151_e1915_d_n16, eq151_e1915_d_n17, eq151_e1915_d_n18, eq151_e1915_d_n19, eq151_e1915_d_n20, eq151_e1915_d_n21, eq151_e1915_d_n22];
        let eq151_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq151_reactive_node_derivatives,
            branches,
            &eq151_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq152_e1929, eq152_e1929_d_n0, eq152_e1929_d_n1, eq152_e1929_d_n2, eq152_e1929_d_n3, eq152_e1929_d_n4, eq152_e1929_d_n5, eq152_e1929_d_n6, eq152_e1929_d_n7, eq152_e1929_d_n8, eq152_e1929_d_n9, eq152_e1929_d_n12, eq152_e1929_d_n14, eq152_e1929_d_n15, eq152_e1929_d_n16, eq152_e1929_d_n17, eq152_e1929_d_n18, eq152_e1929_d_n19, eq152_e1929_d_n20, eq152_e1929_d_n21, eq152_e1929_d_n22, eq152_e1929_q,) = {
    if (((var_guard546 == 0.0) && (var_guard549 != 0.0)) && (var_guard550 != 0.0)) {
        let eq152_e1924: f64 = (p.p7 * p.p247);
        let eq152_e1926_q: f64 = var_qg_fp2;
        let eq152_e1927: f64 = (eq152_e1924 * var_qg_fp2);
        let eq152_e1927_d_n0: f64 = (eq152_e1924 * var_qg_fp2_dn0);
        let eq152_e1927_d_n1: f64 = (eq152_e1924 * var_qg_fp2_dn1);
        let eq152_e1927_d_n2: f64 = (eq152_e1924 * var_qg_fp2_dn2);
        let eq152_e1927_d_n3: f64 = (eq152_e1924 * var_qg_fp2_dn3);
        let eq152_e1927_d_n4: f64 = (eq152_e1924 * var_qg_fp2_dn4);
        let eq152_e1927_d_n5: f64 = (eq152_e1924 * var_qg_fp2_dn5);
        let eq152_e1927_d_n6: f64 = (eq152_e1924 * var_qg_fp2_dn6);
        let eq152_e1927_d_n7: f64 = (eq152_e1924 * var_qg_fp2_dn7);
        let eq152_e1927_d_n8: f64 = (eq152_e1924 * var_qg_fp2_dn8);
        let eq152_e1927_d_n9: f64 = (eq152_e1924 * var_qg_fp2_dn9);
        let eq152_e1927_d_n12: f64 = (eq152_e1924 * var_qg_fp2_dn12);
        let eq152_e1927_d_n14: f64 = (eq152_e1924 * var_qg_fp2_dn14);
        let eq152_e1927_d_n15: f64 = (eq152_e1924 * var_qg_fp2_dn15);
        let eq152_e1927_d_n16: f64 = (eq152_e1924 * var_qg_fp2_dn16);
        let eq152_e1927_d_n17: f64 = (eq152_e1924 * var_qg_fp2_dn17);
        let eq152_e1927_d_n18: f64 = (eq152_e1924 * var_qg_fp2_dn18);
        let eq152_e1927_d_n19: f64 = (eq152_e1924 * var_qg_fp2_dn19);
        let eq152_e1927_d_n20: f64 = (eq152_e1924 * var_qg_fp2_dn20);
        let eq152_e1927_d_n21: f64 = (eq152_e1924 * var_qg_fp2_dn21);
        let eq152_e1927_d_n22: f64 = (eq152_e1924 * var_qg_fp2_dn22);
        let eq152_e1927_q: f64 = (eq152_e1924 * eq152_e1926_q);
        (eq152_e1927, eq152_e1927_d_n0, eq152_e1927_d_n1, eq152_e1927_d_n2, eq152_e1927_d_n3, eq152_e1927_d_n4, eq152_e1927_d_n5, eq152_e1927_d_n6, eq152_e1927_d_n7, eq152_e1927_d_n8, eq152_e1927_d_n9, eq152_e1927_d_n12, eq152_e1927_d_n14, eq152_e1927_d_n15, eq152_e1927_d_n16, eq152_e1927_d_n17, eq152_e1927_d_n18, eq152_e1927_d_n19, eq152_e1927_d_n20, eq152_e1927_d_n21, eq152_e1927_d_n22, eq152_e1927_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq152_reactive_node_derivatives: [f64; 23] = [eq152_e1929_d_n0, eq152_e1929_d_n1, eq152_e1929_d_n2, eq152_e1929_d_n3, eq152_e1929_d_n4, eq152_e1929_d_n5, eq152_e1929_d_n6, eq152_e1929_d_n7, eq152_e1929_d_n8, eq152_e1929_d_n9, 0.0, 0.0, eq152_e1929_d_n12, 0.0, eq152_e1929_d_n14, eq152_e1929_d_n15, eq152_e1929_d_n16, eq152_e1929_d_n17, eq152_e1929_d_n18, eq152_e1929_d_n19, eq152_e1929_d_n20, eq152_e1929_d_n21, eq152_e1929_d_n22];
        let eq152_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq152_reactive_node_derivatives,
            branches,
            &eq152_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq153_e1942, eq153_e1942_d_n0, eq153_e1942_d_n1, eq153_e1942_d_n2, eq153_e1942_d_n3, eq153_e1942_d_n4, eq153_e1942_d_n5, eq153_e1942_d_n6, eq153_e1942_d_n7, eq153_e1942_d_n8, eq153_e1942_d_n9, eq153_e1942_d_n12, eq153_e1942_d_n14, eq153_e1942_d_n15, eq153_e1942_d_n16, eq153_e1942_d_n17, eq153_e1942_d_n18, eq153_e1942_d_n19, eq153_e1942_d_n20, eq153_e1942_d_n21, eq153_e1942_d_n22, eq153_e1942_q,) = {
    if (((var_guard546 == 0.0) && (var_guard549 != 0.0)) && (var_guard550 == 0.0)) {
        let eq153_e1939_q: f64 = var_qg_fp2;
        let eq153_e1940: f64 = (p.p7 * var_qg_fp2);
        let eq153_e1940_d_n0: f64 = (p.p7 * var_qg_fp2_dn0);
        let eq153_e1940_d_n1: f64 = (p.p7 * var_qg_fp2_dn1);
        let eq153_e1940_d_n2: f64 = (p.p7 * var_qg_fp2_dn2);
        let eq153_e1940_d_n3: f64 = (p.p7 * var_qg_fp2_dn3);
        let eq153_e1940_d_n4: f64 = (p.p7 * var_qg_fp2_dn4);
        let eq153_e1940_d_n5: f64 = (p.p7 * var_qg_fp2_dn5);
        let eq153_e1940_d_n6: f64 = (p.p7 * var_qg_fp2_dn6);
        let eq153_e1940_d_n7: f64 = (p.p7 * var_qg_fp2_dn7);
        let eq153_e1940_d_n8: f64 = (p.p7 * var_qg_fp2_dn8);
        let eq153_e1940_d_n9: f64 = (p.p7 * var_qg_fp2_dn9);
        let eq153_e1940_d_n12: f64 = (p.p7 * var_qg_fp2_dn12);
        let eq153_e1940_d_n14: f64 = (p.p7 * var_qg_fp2_dn14);
        let eq153_e1940_d_n15: f64 = (p.p7 * var_qg_fp2_dn15);
        let eq153_e1940_d_n16: f64 = (p.p7 * var_qg_fp2_dn16);
        let eq153_e1940_d_n17: f64 = (p.p7 * var_qg_fp2_dn17);
        let eq153_e1940_d_n18: f64 = (p.p7 * var_qg_fp2_dn18);
        let eq153_e1940_d_n19: f64 = (p.p7 * var_qg_fp2_dn19);
        let eq153_e1940_d_n20: f64 = (p.p7 * var_qg_fp2_dn20);
        let eq153_e1940_d_n21: f64 = (p.p7 * var_qg_fp2_dn21);
        let eq153_e1940_d_n22: f64 = (p.p7 * var_qg_fp2_dn22);
        let eq153_e1940_q: f64 = (p.p7 * eq153_e1939_q);
        (eq153_e1940, eq153_e1940_d_n0, eq153_e1940_d_n1, eq153_e1940_d_n2, eq153_e1940_d_n3, eq153_e1940_d_n4, eq153_e1940_d_n5, eq153_e1940_d_n6, eq153_e1940_d_n7, eq153_e1940_d_n8, eq153_e1940_d_n9, eq153_e1940_d_n12, eq153_e1940_d_n14, eq153_e1940_d_n15, eq153_e1940_d_n16, eq153_e1940_d_n17, eq153_e1940_d_n18, eq153_e1940_d_n19, eq153_e1940_d_n20, eq153_e1940_d_n21, eq153_e1940_d_n22, eq153_e1940_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq153_reactive_node_derivatives: [f64; 23] = [eq153_e1942_d_n0, eq153_e1942_d_n1, eq153_e1942_d_n2, eq153_e1942_d_n3, eq153_e1942_d_n4, eq153_e1942_d_n5, eq153_e1942_d_n6, eq153_e1942_d_n7, eq153_e1942_d_n8, eq153_e1942_d_n9, 0.0, 0.0, eq153_e1942_d_n12, 0.0, eq153_e1942_d_n14, eq153_e1942_d_n15, eq153_e1942_d_n16, eq153_e1942_d_n17, eq153_e1942_d_n18, eq153_e1942_d_n19, eq153_e1942_d_n20, eq153_e1942_d_n21, eq153_e1942_d_n22];
        let eq153_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq153_reactive_node_derivatives,
            branches,
            &eq153_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq154_e1957, eq154_e1957_d_n0, eq154_e1957_d_n1, eq154_e1957_d_n2, eq154_e1957_d_n3, eq154_e1957_d_n4, eq154_e1957_d_n5, eq154_e1957_d_n6, eq154_e1957_d_n7, eq154_e1957_d_n8, eq154_e1957_d_n9, eq154_e1957_d_n12, eq154_e1957_d_n14, eq154_e1957_d_n15, eq154_e1957_d_n16, eq154_e1957_d_n17, eq154_e1957_d_n18, eq154_e1957_d_n19, eq154_e1957_d_n20, eq154_e1957_d_n21, eq154_e1957_d_n22, eq154_e1957_q,) = {
    if (((var_guard546 == 0.0) && (var_guard549 != 0.0)) && (var_guard550 == 0.0)) {
        let eq154_e1952: f64 = (p.p7 * p.p247);
        let eq154_e1954_q: f64 = var_qg_fp2;
        let eq154_e1955: f64 = (eq154_e1952 * var_qg_fp2);
        let eq154_e1955_d_n0: f64 = (eq154_e1952 * var_qg_fp2_dn0);
        let eq154_e1955_d_n1: f64 = (eq154_e1952 * var_qg_fp2_dn1);
        let eq154_e1955_d_n2: f64 = (eq154_e1952 * var_qg_fp2_dn2);
        let eq154_e1955_d_n3: f64 = (eq154_e1952 * var_qg_fp2_dn3);
        let eq154_e1955_d_n4: f64 = (eq154_e1952 * var_qg_fp2_dn4);
        let eq154_e1955_d_n5: f64 = (eq154_e1952 * var_qg_fp2_dn5);
        let eq154_e1955_d_n6: f64 = (eq154_e1952 * var_qg_fp2_dn6);
        let eq154_e1955_d_n7: f64 = (eq154_e1952 * var_qg_fp2_dn7);
        let eq154_e1955_d_n8: f64 = (eq154_e1952 * var_qg_fp2_dn8);
        let eq154_e1955_d_n9: f64 = (eq154_e1952 * var_qg_fp2_dn9);
        let eq154_e1955_d_n12: f64 = (eq154_e1952 * var_qg_fp2_dn12);
        let eq154_e1955_d_n14: f64 = (eq154_e1952 * var_qg_fp2_dn14);
        let eq154_e1955_d_n15: f64 = (eq154_e1952 * var_qg_fp2_dn15);
        let eq154_e1955_d_n16: f64 = (eq154_e1952 * var_qg_fp2_dn16);
        let eq154_e1955_d_n17: f64 = (eq154_e1952 * var_qg_fp2_dn17);
        let eq154_e1955_d_n18: f64 = (eq154_e1952 * var_qg_fp2_dn18);
        let eq154_e1955_d_n19: f64 = (eq154_e1952 * var_qg_fp2_dn19);
        let eq154_e1955_d_n20: f64 = (eq154_e1952 * var_qg_fp2_dn20);
        let eq154_e1955_d_n21: f64 = (eq154_e1952 * var_qg_fp2_dn21);
        let eq154_e1955_d_n22: f64 = (eq154_e1952 * var_qg_fp2_dn22);
        let eq154_e1955_q: f64 = (eq154_e1952 * eq154_e1954_q);
        (eq154_e1955, eq154_e1955_d_n0, eq154_e1955_d_n1, eq154_e1955_d_n2, eq154_e1955_d_n3, eq154_e1955_d_n4, eq154_e1955_d_n5, eq154_e1955_d_n6, eq154_e1955_d_n7, eq154_e1955_d_n8, eq154_e1955_d_n9, eq154_e1955_d_n12, eq154_e1955_d_n14, eq154_e1955_d_n15, eq154_e1955_d_n16, eq154_e1955_d_n17, eq154_e1955_d_n18, eq154_e1955_d_n19, eq154_e1955_d_n20, eq154_e1955_d_n21, eq154_e1955_d_n22, eq154_e1955_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq154_reactive_node_derivatives: [f64; 23] = [eq154_e1957_d_n0, eq154_e1957_d_n1, eq154_e1957_d_n2, eq154_e1957_d_n3, eq154_e1957_d_n4, eq154_e1957_d_n5, eq154_e1957_d_n6, eq154_e1957_d_n7, eq154_e1957_d_n8, eq154_e1957_d_n9, 0.0, 0.0, eq154_e1957_d_n12, 0.0, eq154_e1957_d_n14, eq154_e1957_d_n15, eq154_e1957_d_n16, eq154_e1957_d_n17, eq154_e1957_d_n18, eq154_e1957_d_n19, eq154_e1957_d_n20, eq154_e1957_d_n21, eq154_e1957_d_n22];
        let eq154_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq154_reactive_node_derivatives,
            branches,
            &eq154_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq155_e1969, eq155_e1969_d_n0, eq155_e1969_d_n1, eq155_e1969_d_n2, eq155_e1969_d_n3, eq155_e1969_d_n4, eq155_e1969_d_n5, eq155_e1969_d_n6, eq155_e1969_d_n7, eq155_e1969_d_n8, eq155_e1969_d_n9, eq155_e1969_d_n12, eq155_e1969_d_n14, eq155_e1969_d_n15, eq155_e1969_d_n16, eq155_e1969_d_n17, eq155_e1969_d_n18, eq155_e1969_d_n19, eq155_e1969_d_n20, eq155_e1969_d_n21, eq155_e1969_d_n22, eq155_e1969_q,) = {
    if ((var_guard546 == 0.0) && (var_guard549 != 0.0)) {
        let eq155_e1965: f64 = (p.p252 * var_qg_fp2);
        let eq155_e1965_d_n0: f64 = (p.p252 * var_qg_fp2_dn0);
        let eq155_e1965_d_n1: f64 = (p.p252 * var_qg_fp2_dn1);
        let eq155_e1965_d_n2: f64 = (p.p252 * var_qg_fp2_dn2);
        let eq155_e1965_d_n3: f64 = (p.p252 * var_qg_fp2_dn3);
        let eq155_e1965_d_n4: f64 = (p.p252 * var_qg_fp2_dn4);
        let eq155_e1965_d_n5: f64 = (p.p252 * var_qg_fp2_dn5);
        let eq155_e1965_d_n6: f64 = (p.p252 * var_qg_fp2_dn6);
        let eq155_e1965_d_n7: f64 = (p.p252 * var_qg_fp2_dn7);
        let eq155_e1965_d_n8: f64 = (p.p252 * var_qg_fp2_dn8);
        let eq155_e1965_d_n9: f64 = (p.p252 * var_qg_fp2_dn9);
        let eq155_e1965_d_n12: f64 = (p.p252 * var_qg_fp2_dn12);
        let eq155_e1965_d_n14: f64 = (p.p252 * var_qg_fp2_dn14);
        let eq155_e1965_d_n15: f64 = (p.p252 * var_qg_fp2_dn15);
        let eq155_e1965_d_n16: f64 = (p.p252 * var_qg_fp2_dn16);
        let eq155_e1965_d_n17: f64 = (p.p252 * var_qg_fp2_dn17);
        let eq155_e1965_d_n18: f64 = (p.p252 * var_qg_fp2_dn18);
        let eq155_e1965_d_n19: f64 = (p.p252 * var_qg_fp2_dn19);
        let eq155_e1965_d_n20: f64 = (p.p252 * var_qg_fp2_dn20);
        let eq155_e1965_d_n21: f64 = (p.p252 * var_qg_fp2_dn21);
        let eq155_e1965_d_n22: f64 = (p.p252 * var_qg_fp2_dn22);
        let eq155_e1966_q: f64 = eq155_e1965;
        let eq155_e1967: f64 = (p.p7 * eq155_e1965);
        let eq155_e1967_d_n0: f64 = (p.p7 * eq155_e1965_d_n0);
        let eq155_e1967_d_n1: f64 = (p.p7 * eq155_e1965_d_n1);
        let eq155_e1967_d_n2: f64 = (p.p7 * eq155_e1965_d_n2);
        let eq155_e1967_d_n3: f64 = (p.p7 * eq155_e1965_d_n3);
        let eq155_e1967_d_n4: f64 = (p.p7 * eq155_e1965_d_n4);
        let eq155_e1967_d_n5: f64 = (p.p7 * eq155_e1965_d_n5);
        let eq155_e1967_d_n6: f64 = (p.p7 * eq155_e1965_d_n6);
        let eq155_e1967_d_n7: f64 = (p.p7 * eq155_e1965_d_n7);
        let eq155_e1967_d_n8: f64 = (p.p7 * eq155_e1965_d_n8);
        let eq155_e1967_d_n9: f64 = (p.p7 * eq155_e1965_d_n9);
        let eq155_e1967_d_n12: f64 = (p.p7 * eq155_e1965_d_n12);
        let eq155_e1967_d_n14: f64 = (p.p7 * eq155_e1965_d_n14);
        let eq155_e1967_d_n15: f64 = (p.p7 * eq155_e1965_d_n15);
        let eq155_e1967_d_n16: f64 = (p.p7 * eq155_e1965_d_n16);
        let eq155_e1967_d_n17: f64 = (p.p7 * eq155_e1965_d_n17);
        let eq155_e1967_d_n18: f64 = (p.p7 * eq155_e1965_d_n18);
        let eq155_e1967_d_n19: f64 = (p.p7 * eq155_e1965_d_n19);
        let eq155_e1967_d_n20: f64 = (p.p7 * eq155_e1965_d_n20);
        let eq155_e1967_d_n21: f64 = (p.p7 * eq155_e1965_d_n21);
        let eq155_e1967_d_n22: f64 = (p.p7 * eq155_e1965_d_n22);
        let eq155_e1967_q: f64 = (p.p7 * eq155_e1966_q);
        (eq155_e1967, eq155_e1967_d_n0, eq155_e1967_d_n1, eq155_e1967_d_n2, eq155_e1967_d_n3, eq155_e1967_d_n4, eq155_e1967_d_n5, eq155_e1967_d_n6, eq155_e1967_d_n7, eq155_e1967_d_n8, eq155_e1967_d_n9, eq155_e1967_d_n12, eq155_e1967_d_n14, eq155_e1967_d_n15, eq155_e1967_d_n16, eq155_e1967_d_n17, eq155_e1967_d_n18, eq155_e1967_d_n19, eq155_e1967_d_n20, eq155_e1967_d_n21, eq155_e1967_d_n22, eq155_e1967_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq155_reactive_node_derivatives: [f64; 23] = [eq155_e1969_d_n0, eq155_e1969_d_n1, eq155_e1969_d_n2, eq155_e1969_d_n3, eq155_e1969_d_n4, eq155_e1969_d_n5, eq155_e1969_d_n6, eq155_e1969_d_n7, eq155_e1969_d_n8, eq155_e1969_d_n9, 0.0, 0.0, eq155_e1969_d_n12, 0.0, eq155_e1969_d_n14, eq155_e1969_d_n15, eq155_e1969_d_n16, eq155_e1969_d_n17, eq155_e1969_d_n18, eq155_e1969_d_n19, eq155_e1969_d_n20, eq155_e1969_d_n21, eq155_e1969_d_n22];
        let eq155_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            nodes,
            &eq155_reactive_node_derivatives,
            branches,
            &eq155_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq156_e1978, eq156_e1978_d_n0, eq156_e1978_d_n1, eq156_e1978_d_n2, eq156_e1978_d_n3, eq156_e1978_d_n4, eq156_e1978_d_n5, eq156_e1978_d_n6, eq156_e1978_d_n7, eq156_e1978_d_n8, eq156_e1978_d_n9, eq156_e1978_d_n12, eq156_e1978_d_n14, eq156_e1978_d_n15, eq156_e1978_d_n16, eq156_e1978_d_n17, eq156_e1978_d_n18, eq156_e1978_d_n19, eq156_e1978_d_n20, eq156_e1978_d_n21, eq156_e1978_d_n22, eq156_e1978_q,) = {
    if ((var_guard551 != 0.0) && (var_guard552 != 0.0)) {
        let eq156_e1975_q: f64 = var_qd_fp2s;
        let eq156_e1976: f64 = (p.p7 * var_qd_fp2s);
        let eq156_e1976_d_n0: f64 = (p.p7 * var_qd_fp2s_dn0);
        let eq156_e1976_d_n1: f64 = (p.p7 * var_qd_fp2s_dn1);
        let eq156_e1976_d_n2: f64 = (p.p7 * var_qd_fp2s_dn2);
        let eq156_e1976_d_n3: f64 = (p.p7 * var_qd_fp2s_dn3);
        let eq156_e1976_d_n4: f64 = (p.p7 * var_qd_fp2s_dn4);
        let eq156_e1976_d_n5: f64 = (p.p7 * var_qd_fp2s_dn5);
        let eq156_e1976_d_n6: f64 = (p.p7 * var_qd_fp2s_dn6);
        let eq156_e1976_d_n7: f64 = (p.p7 * var_qd_fp2s_dn7);
        let eq156_e1976_d_n8: f64 = (p.p7 * var_qd_fp2s_dn8);
        let eq156_e1976_d_n9: f64 = (p.p7 * var_qd_fp2s_dn9);
        let eq156_e1976_d_n12: f64 = (p.p7 * var_qd_fp2s_dn12);
        let eq156_e1976_d_n14: f64 = (p.p7 * var_qd_fp2s_dn14);
        let eq156_e1976_d_n15: f64 = (p.p7 * var_qd_fp2s_dn15);
        let eq156_e1976_d_n16: f64 = (p.p7 * var_qd_fp2s_dn16);
        let eq156_e1976_d_n17: f64 = (p.p7 * var_qd_fp2s_dn17);
        let eq156_e1976_d_n18: f64 = (p.p7 * var_qd_fp2s_dn18);
        let eq156_e1976_d_n19: f64 = (p.p7 * var_qd_fp2s_dn19);
        let eq156_e1976_d_n20: f64 = (p.p7 * var_qd_fp2s_dn20);
        let eq156_e1976_d_n21: f64 = (p.p7 * var_qd_fp2s_dn21);
        let eq156_e1976_d_n22: f64 = (p.p7 * var_qd_fp2s_dn22);
        let eq156_e1976_q: f64 = (p.p7 * eq156_e1975_q);
        (eq156_e1976, eq156_e1976_d_n0, eq156_e1976_d_n1, eq156_e1976_d_n2, eq156_e1976_d_n3, eq156_e1976_d_n4, eq156_e1976_d_n5, eq156_e1976_d_n6, eq156_e1976_d_n7, eq156_e1976_d_n8, eq156_e1976_d_n9, eq156_e1976_d_n12, eq156_e1976_d_n14, eq156_e1976_d_n15, eq156_e1976_d_n16, eq156_e1976_d_n17, eq156_e1976_d_n18, eq156_e1976_d_n19, eq156_e1976_d_n20, eq156_e1976_d_n21, eq156_e1976_d_n22, eq156_e1976_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq156_reactive_node_derivatives: [f64; 23] = [eq156_e1978_d_n0, eq156_e1978_d_n1, eq156_e1978_d_n2, eq156_e1978_d_n3, eq156_e1978_d_n4, eq156_e1978_d_n5, eq156_e1978_d_n6, eq156_e1978_d_n7, eq156_e1978_d_n8, eq156_e1978_d_n9, 0.0, 0.0, eq156_e1978_d_n12, 0.0, eq156_e1978_d_n14, eq156_e1978_d_n15, eq156_e1978_d_n16, eq156_e1978_d_n17, eq156_e1978_d_n18, eq156_e1978_d_n19, eq156_e1978_d_n20, eq156_e1978_d_n21, eq156_e1978_d_n22];
        let eq156_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[20]),
            nodes,
            &eq156_reactive_node_derivatives,
            branches,
            &eq156_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq157_e1989, eq157_e1989_d_n0, eq157_e1989_d_n1, eq157_e1989_d_n2, eq157_e1989_d_n3, eq157_e1989_d_n4, eq157_e1989_d_n5, eq157_e1989_d_n6, eq157_e1989_d_n7, eq157_e1989_d_n8, eq157_e1989_d_n9, eq157_e1989_d_n12, eq157_e1989_d_n14, eq157_e1989_d_n15, eq157_e1989_d_n16, eq157_e1989_d_n17, eq157_e1989_d_n18, eq157_e1989_d_n19, eq157_e1989_d_n20, eq157_e1989_d_n21, eq157_e1989_d_n22, eq157_e1989_q,) = {
    if (((var_guard551 != 0.0) && (var_guard552 != 0.0)) && (var_guard553 != 0.0)) {
        let eq157_e1986_q: f64 = var_qg_fp2s;
        let eq157_e1987: f64 = (p.p7 * var_qg_fp2s);
        let eq157_e1987_d_n0: f64 = (p.p7 * var_qg_fp2s_dn0);
        let eq157_e1987_d_n1: f64 = (p.p7 * var_qg_fp2s_dn1);
        let eq157_e1987_d_n2: f64 = (p.p7 * var_qg_fp2s_dn2);
        let eq157_e1987_d_n3: f64 = (p.p7 * var_qg_fp2s_dn3);
        let eq157_e1987_d_n4: f64 = (p.p7 * var_qg_fp2s_dn4);
        let eq157_e1987_d_n5: f64 = (p.p7 * var_qg_fp2s_dn5);
        let eq157_e1987_d_n6: f64 = (p.p7 * var_qg_fp2s_dn6);
        let eq157_e1987_d_n7: f64 = (p.p7 * var_qg_fp2s_dn7);
        let eq157_e1987_d_n8: f64 = (p.p7 * var_qg_fp2s_dn8);
        let eq157_e1987_d_n9: f64 = (p.p7 * var_qg_fp2s_dn9);
        let eq157_e1987_d_n12: f64 = (p.p7 * var_qg_fp2s_dn12);
        let eq157_e1987_d_n14: f64 = (p.p7 * var_qg_fp2s_dn14);
        let eq157_e1987_d_n15: f64 = (p.p7 * var_qg_fp2s_dn15);
        let eq157_e1987_d_n16: f64 = (p.p7 * var_qg_fp2s_dn16);
        let eq157_e1987_d_n17: f64 = (p.p7 * var_qg_fp2s_dn17);
        let eq157_e1987_d_n18: f64 = (p.p7 * var_qg_fp2s_dn18);
        let eq157_e1987_d_n19: f64 = (p.p7 * var_qg_fp2s_dn19);
        let eq157_e1987_d_n20: f64 = (p.p7 * var_qg_fp2s_dn20);
        let eq157_e1987_d_n21: f64 = (p.p7 * var_qg_fp2s_dn21);
        let eq157_e1987_d_n22: f64 = (p.p7 * var_qg_fp2s_dn22);
        let eq157_e1987_q: f64 = (p.p7 * eq157_e1986_q);
        (eq157_e1987, eq157_e1987_d_n0, eq157_e1987_d_n1, eq157_e1987_d_n2, eq157_e1987_d_n3, eq157_e1987_d_n4, eq157_e1987_d_n5, eq157_e1987_d_n6, eq157_e1987_d_n7, eq157_e1987_d_n8, eq157_e1987_d_n9, eq157_e1987_d_n12, eq157_e1987_d_n14, eq157_e1987_d_n15, eq157_e1987_d_n16, eq157_e1987_d_n17, eq157_e1987_d_n18, eq157_e1987_d_n19, eq157_e1987_d_n20, eq157_e1987_d_n21, eq157_e1987_d_n22, eq157_e1987_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq157_reactive_node_derivatives: [f64; 23] = [eq157_e1989_d_n0, eq157_e1989_d_n1, eq157_e1989_d_n2, eq157_e1989_d_n3, eq157_e1989_d_n4, eq157_e1989_d_n5, eq157_e1989_d_n6, eq157_e1989_d_n7, eq157_e1989_d_n8, eq157_e1989_d_n9, 0.0, 0.0, eq157_e1989_d_n12, 0.0, eq157_e1989_d_n14, eq157_e1989_d_n15, eq157_e1989_d_n16, eq157_e1989_d_n17, eq157_e1989_d_n18, eq157_e1989_d_n19, eq157_e1989_d_n20, eq157_e1989_d_n21, eq157_e1989_d_n22];
        let eq157_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[20]),
            nodes,
            &eq157_reactive_node_derivatives,
            branches,
            &eq157_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq158_e2002, eq158_e2002_d_n0, eq158_e2002_d_n1, eq158_e2002_d_n2, eq158_e2002_d_n3, eq158_e2002_d_n4, eq158_e2002_d_n5, eq158_e2002_d_n6, eq158_e2002_d_n7, eq158_e2002_d_n8, eq158_e2002_d_n9, eq158_e2002_d_n12, eq158_e2002_d_n14, eq158_e2002_d_n15, eq158_e2002_d_n16, eq158_e2002_d_n17, eq158_e2002_d_n18, eq158_e2002_d_n19, eq158_e2002_d_n20, eq158_e2002_d_n21, eq158_e2002_d_n22, eq158_e2002_q,) = {
    if (((var_guard551 != 0.0) && (var_guard552 != 0.0)) && (var_guard553 != 0.0)) {
        let eq158_e1997_q: f64 = var_qg_fp2s;
        let eq158_e1998: f64 = (p.p7 * var_qg_fp2s);
        let eq158_e1998_d_n0: f64 = (p.p7 * var_qg_fp2s_dn0);
        let eq158_e1998_d_n1: f64 = (p.p7 * var_qg_fp2s_dn1);
        let eq158_e1998_d_n2: f64 = (p.p7 * var_qg_fp2s_dn2);
        let eq158_e1998_d_n3: f64 = (p.p7 * var_qg_fp2s_dn3);
        let eq158_e1998_d_n4: f64 = (p.p7 * var_qg_fp2s_dn4);
        let eq158_e1998_d_n5: f64 = (p.p7 * var_qg_fp2s_dn5);
        let eq158_e1998_d_n6: f64 = (p.p7 * var_qg_fp2s_dn6);
        let eq158_e1998_d_n7: f64 = (p.p7 * var_qg_fp2s_dn7);
        let eq158_e1998_d_n8: f64 = (p.p7 * var_qg_fp2s_dn8);
        let eq158_e1998_d_n9: f64 = (p.p7 * var_qg_fp2s_dn9);
        let eq158_e1998_d_n12: f64 = (p.p7 * var_qg_fp2s_dn12);
        let eq158_e1998_d_n14: f64 = (p.p7 * var_qg_fp2s_dn14);
        let eq158_e1998_d_n15: f64 = (p.p7 * var_qg_fp2s_dn15);
        let eq158_e1998_d_n16: f64 = (p.p7 * var_qg_fp2s_dn16);
        let eq158_e1998_d_n17: f64 = (p.p7 * var_qg_fp2s_dn17);
        let eq158_e1998_d_n18: f64 = (p.p7 * var_qg_fp2s_dn18);
        let eq158_e1998_d_n19: f64 = (p.p7 * var_qg_fp2s_dn19);
        let eq158_e1998_d_n20: f64 = (p.p7 * var_qg_fp2s_dn20);
        let eq158_e1998_d_n21: f64 = (p.p7 * var_qg_fp2s_dn21);
        let eq158_e1998_d_n22: f64 = (p.p7 * var_qg_fp2s_dn22);
        let eq158_e1998_q: f64 = (p.p7 * eq158_e1997_q);
        let eq158_e2000: f64 = (eq158_e1998 * p.p247);
        let eq158_e2000_d_n0: f64 = (eq158_e1998_d_n0 * p.p247);
        let eq158_e2000_d_n1: f64 = (eq158_e1998_d_n1 * p.p247);
        let eq158_e2000_d_n2: f64 = (eq158_e1998_d_n2 * p.p247);
        let eq158_e2000_d_n3: f64 = (eq158_e1998_d_n3 * p.p247);
        let eq158_e2000_d_n4: f64 = (eq158_e1998_d_n4 * p.p247);
        let eq158_e2000_d_n5: f64 = (eq158_e1998_d_n5 * p.p247);
        let eq158_e2000_d_n6: f64 = (eq158_e1998_d_n6 * p.p247);
        let eq158_e2000_d_n7: f64 = (eq158_e1998_d_n7 * p.p247);
        let eq158_e2000_d_n8: f64 = (eq158_e1998_d_n8 * p.p247);
        let eq158_e2000_d_n9: f64 = (eq158_e1998_d_n9 * p.p247);
        let eq158_e2000_d_n12: f64 = (eq158_e1998_d_n12 * p.p247);
        let eq158_e2000_d_n14: f64 = (eq158_e1998_d_n14 * p.p247);
        let eq158_e2000_d_n15: f64 = (eq158_e1998_d_n15 * p.p247);
        let eq158_e2000_d_n16: f64 = (eq158_e1998_d_n16 * p.p247);
        let eq158_e2000_d_n17: f64 = (eq158_e1998_d_n17 * p.p247);
        let eq158_e2000_d_n18: f64 = (eq158_e1998_d_n18 * p.p247);
        let eq158_e2000_d_n19: f64 = (eq158_e1998_d_n19 * p.p247);
        let eq158_e2000_d_n20: f64 = (eq158_e1998_d_n20 * p.p247);
        let eq158_e2000_d_n21: f64 = (eq158_e1998_d_n21 * p.p247);
        let eq158_e2000_d_n22: f64 = (eq158_e1998_d_n22 * p.p247);
        let eq158_e2000_q: f64 = (eq158_e1998_q * p.p247);
        (eq158_e2000, eq158_e2000_d_n0, eq158_e2000_d_n1, eq158_e2000_d_n2, eq158_e2000_d_n3, eq158_e2000_d_n4, eq158_e2000_d_n5, eq158_e2000_d_n6, eq158_e2000_d_n7, eq158_e2000_d_n8, eq158_e2000_d_n9, eq158_e2000_d_n12, eq158_e2000_d_n14, eq158_e2000_d_n15, eq158_e2000_d_n16, eq158_e2000_d_n17, eq158_e2000_d_n18, eq158_e2000_d_n19, eq158_e2000_d_n20, eq158_e2000_d_n21, eq158_e2000_d_n22, eq158_e2000_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq158_reactive_node_derivatives: [f64; 23] = [eq158_e2002_d_n0, eq158_e2002_d_n1, eq158_e2002_d_n2, eq158_e2002_d_n3, eq158_e2002_d_n4, eq158_e2002_d_n5, eq158_e2002_d_n6, eq158_e2002_d_n7, eq158_e2002_d_n8, eq158_e2002_d_n9, 0.0, 0.0, eq158_e2002_d_n12, 0.0, eq158_e2002_d_n14, eq158_e2002_d_n15, eq158_e2002_d_n16, eq158_e2002_d_n17, eq158_e2002_d_n18, eq158_e2002_d_n19, eq158_e2002_d_n20, eq158_e2002_d_n21, eq158_e2002_d_n22];
        let eq158_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[20]),
            nodes,
            &eq158_reactive_node_derivatives,
            branches,
            &eq158_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_5(
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_guard551: f64,
        var_guard552: f64,
        var_guard553: f64,
        var_guard554: f64,
        var_guard555: f64,
        var_qd_fp2s: f64,
        var_qd_fp2s_dn0: f64,
        var_qd_fp2s_dn1: f64,
        var_qd_fp2s_dn12: f64,
        var_qd_fp2s_dn14: f64,
        var_qd_fp2s_dn15: f64,
        var_qd_fp2s_dn16: f64,
        var_qd_fp2s_dn17: f64,
        var_qd_fp2s_dn18: f64,
        var_qd_fp2s_dn19: f64,
        var_qd_fp2s_dn2: f64,
        var_qd_fp2s_dn20: f64,
        var_qd_fp2s_dn21: f64,
        var_qd_fp2s_dn22: f64,
        var_qd_fp2s_dn3: f64,
        var_qd_fp2s_dn4: f64,
        var_qd_fp2s_dn5: f64,
        var_qd_fp2s_dn6: f64,
        var_qd_fp2s_dn7: f64,
        var_qd_fp2s_dn8: f64,
        var_qd_fp2s_dn9: f64,
        var_qg_fp2s: f64,
        var_qg_fp2s_dn0: f64,
        var_qg_fp2s_dn1: f64,
        var_qg_fp2s_dn12: f64,
        var_qg_fp2s_dn14: f64,
        var_qg_fp2s_dn15: f64,
        var_qg_fp2s_dn16: f64,
        var_qg_fp2s_dn17: f64,
        var_qg_fp2s_dn18: f64,
        var_qg_fp2s_dn19: f64,
        var_qg_fp2s_dn2: f64,
        var_qg_fp2s_dn20: f64,
        var_qg_fp2s_dn21: f64,
        var_qg_fp2s_dn22: f64,
        var_qg_fp2s_dn3: f64,
        var_qg_fp2s_dn4: f64,
        var_qg_fp2s_dn5: f64,
        var_qg_fp2s_dn6: f64,
        var_qg_fp2s_dn7: f64,
        var_qg_fp2s_dn8: f64,
        var_qg_fp2s_dn9: f64,
    ) {
        let (eq159_e2014, eq159_e2014_d_n0, eq159_e2014_d_n1, eq159_e2014_d_n2, eq159_e2014_d_n3, eq159_e2014_d_n4, eq159_e2014_d_n5, eq159_e2014_d_n6, eq159_e2014_d_n7, eq159_e2014_d_n8, eq159_e2014_d_n9, eq159_e2014_d_n12, eq159_e2014_d_n14, eq159_e2014_d_n15, eq159_e2014_d_n16, eq159_e2014_d_n17, eq159_e2014_d_n18, eq159_e2014_d_n19, eq159_e2014_d_n20, eq159_e2014_d_n21, eq159_e2014_d_n22, eq159_e2014_q,) = {
    if (((var_guard551 != 0.0) && (var_guard552 != 0.0)) && (var_guard553 == 0.0)) {
        let eq159_e2011_q: f64 = var_qg_fp2s;
        let eq159_e2012: f64 = (p.p7 * var_qg_fp2s);
        let eq159_e2012_d_n0: f64 = (p.p7 * var_qg_fp2s_dn0);
        let eq159_e2012_d_n1: f64 = (p.p7 * var_qg_fp2s_dn1);
        let eq159_e2012_d_n2: f64 = (p.p7 * var_qg_fp2s_dn2);
        let eq159_e2012_d_n3: f64 = (p.p7 * var_qg_fp2s_dn3);
        let eq159_e2012_d_n4: f64 = (p.p7 * var_qg_fp2s_dn4);
        let eq159_e2012_d_n5: f64 = (p.p7 * var_qg_fp2s_dn5);
        let eq159_e2012_d_n6: f64 = (p.p7 * var_qg_fp2s_dn6);
        let eq159_e2012_d_n7: f64 = (p.p7 * var_qg_fp2s_dn7);
        let eq159_e2012_d_n8: f64 = (p.p7 * var_qg_fp2s_dn8);
        let eq159_e2012_d_n9: f64 = (p.p7 * var_qg_fp2s_dn9);
        let eq159_e2012_d_n12: f64 = (p.p7 * var_qg_fp2s_dn12);
        let eq159_e2012_d_n14: f64 = (p.p7 * var_qg_fp2s_dn14);
        let eq159_e2012_d_n15: f64 = (p.p7 * var_qg_fp2s_dn15);
        let eq159_e2012_d_n16: f64 = (p.p7 * var_qg_fp2s_dn16);
        let eq159_e2012_d_n17: f64 = (p.p7 * var_qg_fp2s_dn17);
        let eq159_e2012_d_n18: f64 = (p.p7 * var_qg_fp2s_dn18);
        let eq159_e2012_d_n19: f64 = (p.p7 * var_qg_fp2s_dn19);
        let eq159_e2012_d_n20: f64 = (p.p7 * var_qg_fp2s_dn20);
        let eq159_e2012_d_n21: f64 = (p.p7 * var_qg_fp2s_dn21);
        let eq159_e2012_d_n22: f64 = (p.p7 * var_qg_fp2s_dn22);
        let eq159_e2012_q: f64 = (p.p7 * eq159_e2011_q);
        (eq159_e2012, eq159_e2012_d_n0, eq159_e2012_d_n1, eq159_e2012_d_n2, eq159_e2012_d_n3, eq159_e2012_d_n4, eq159_e2012_d_n5, eq159_e2012_d_n6, eq159_e2012_d_n7, eq159_e2012_d_n8, eq159_e2012_d_n9, eq159_e2012_d_n12, eq159_e2012_d_n14, eq159_e2012_d_n15, eq159_e2012_d_n16, eq159_e2012_d_n17, eq159_e2012_d_n18, eq159_e2012_d_n19, eq159_e2012_d_n20, eq159_e2012_d_n21, eq159_e2012_d_n22, eq159_e2012_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq159_reactive_node_derivatives: [f64; 23] = [eq159_e2014_d_n0, eq159_e2014_d_n1, eq159_e2014_d_n2, eq159_e2014_d_n3, eq159_e2014_d_n4, eq159_e2014_d_n5, eq159_e2014_d_n6, eq159_e2014_d_n7, eq159_e2014_d_n8, eq159_e2014_d_n9, 0.0, 0.0, eq159_e2014_d_n12, 0.0, eq159_e2014_d_n14, eq159_e2014_d_n15, eq159_e2014_d_n16, eq159_e2014_d_n17, eq159_e2014_d_n18, eq159_e2014_d_n19, eq159_e2014_d_n20, eq159_e2014_d_n21, eq159_e2014_d_n22];
        let eq159_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[20]),
            nodes,
            &eq159_reactive_node_derivatives,
            branches,
            &eq159_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq160_e2028, eq160_e2028_d_n0, eq160_e2028_d_n1, eq160_e2028_d_n2, eq160_e2028_d_n3, eq160_e2028_d_n4, eq160_e2028_d_n5, eq160_e2028_d_n6, eq160_e2028_d_n7, eq160_e2028_d_n8, eq160_e2028_d_n9, eq160_e2028_d_n12, eq160_e2028_d_n14, eq160_e2028_d_n15, eq160_e2028_d_n16, eq160_e2028_d_n17, eq160_e2028_d_n18, eq160_e2028_d_n19, eq160_e2028_d_n20, eq160_e2028_d_n21, eq160_e2028_d_n22, eq160_e2028_q,) = {
    if (((var_guard551 != 0.0) && (var_guard552 != 0.0)) && (var_guard553 == 0.0)) {
        let eq160_e2023_q: f64 = var_qg_fp2s;
        let eq160_e2024: f64 = (p.p7 * var_qg_fp2s);
        let eq160_e2024_d_n0: f64 = (p.p7 * var_qg_fp2s_dn0);
        let eq160_e2024_d_n1: f64 = (p.p7 * var_qg_fp2s_dn1);
        let eq160_e2024_d_n2: f64 = (p.p7 * var_qg_fp2s_dn2);
        let eq160_e2024_d_n3: f64 = (p.p7 * var_qg_fp2s_dn3);
        let eq160_e2024_d_n4: f64 = (p.p7 * var_qg_fp2s_dn4);
        let eq160_e2024_d_n5: f64 = (p.p7 * var_qg_fp2s_dn5);
        let eq160_e2024_d_n6: f64 = (p.p7 * var_qg_fp2s_dn6);
        let eq160_e2024_d_n7: f64 = (p.p7 * var_qg_fp2s_dn7);
        let eq160_e2024_d_n8: f64 = (p.p7 * var_qg_fp2s_dn8);
        let eq160_e2024_d_n9: f64 = (p.p7 * var_qg_fp2s_dn9);
        let eq160_e2024_d_n12: f64 = (p.p7 * var_qg_fp2s_dn12);
        let eq160_e2024_d_n14: f64 = (p.p7 * var_qg_fp2s_dn14);
        let eq160_e2024_d_n15: f64 = (p.p7 * var_qg_fp2s_dn15);
        let eq160_e2024_d_n16: f64 = (p.p7 * var_qg_fp2s_dn16);
        let eq160_e2024_d_n17: f64 = (p.p7 * var_qg_fp2s_dn17);
        let eq160_e2024_d_n18: f64 = (p.p7 * var_qg_fp2s_dn18);
        let eq160_e2024_d_n19: f64 = (p.p7 * var_qg_fp2s_dn19);
        let eq160_e2024_d_n20: f64 = (p.p7 * var_qg_fp2s_dn20);
        let eq160_e2024_d_n21: f64 = (p.p7 * var_qg_fp2s_dn21);
        let eq160_e2024_d_n22: f64 = (p.p7 * var_qg_fp2s_dn22);
        let eq160_e2024_q: f64 = (p.p7 * eq160_e2023_q);
        let eq160_e2026: f64 = (eq160_e2024 * p.p247);
        let eq160_e2026_d_n0: f64 = (eq160_e2024_d_n0 * p.p247);
        let eq160_e2026_d_n1: f64 = (eq160_e2024_d_n1 * p.p247);
        let eq160_e2026_d_n2: f64 = (eq160_e2024_d_n2 * p.p247);
        let eq160_e2026_d_n3: f64 = (eq160_e2024_d_n3 * p.p247);
        let eq160_e2026_d_n4: f64 = (eq160_e2024_d_n4 * p.p247);
        let eq160_e2026_d_n5: f64 = (eq160_e2024_d_n5 * p.p247);
        let eq160_e2026_d_n6: f64 = (eq160_e2024_d_n6 * p.p247);
        let eq160_e2026_d_n7: f64 = (eq160_e2024_d_n7 * p.p247);
        let eq160_e2026_d_n8: f64 = (eq160_e2024_d_n8 * p.p247);
        let eq160_e2026_d_n9: f64 = (eq160_e2024_d_n9 * p.p247);
        let eq160_e2026_d_n12: f64 = (eq160_e2024_d_n12 * p.p247);
        let eq160_e2026_d_n14: f64 = (eq160_e2024_d_n14 * p.p247);
        let eq160_e2026_d_n15: f64 = (eq160_e2024_d_n15 * p.p247);
        let eq160_e2026_d_n16: f64 = (eq160_e2024_d_n16 * p.p247);
        let eq160_e2026_d_n17: f64 = (eq160_e2024_d_n17 * p.p247);
        let eq160_e2026_d_n18: f64 = (eq160_e2024_d_n18 * p.p247);
        let eq160_e2026_d_n19: f64 = (eq160_e2024_d_n19 * p.p247);
        let eq160_e2026_d_n20: f64 = (eq160_e2024_d_n20 * p.p247);
        let eq160_e2026_d_n21: f64 = (eq160_e2024_d_n21 * p.p247);
        let eq160_e2026_d_n22: f64 = (eq160_e2024_d_n22 * p.p247);
        let eq160_e2026_q: f64 = (eq160_e2024_q * p.p247);
        (eq160_e2026, eq160_e2026_d_n0, eq160_e2026_d_n1, eq160_e2026_d_n2, eq160_e2026_d_n3, eq160_e2026_d_n4, eq160_e2026_d_n5, eq160_e2026_d_n6, eq160_e2026_d_n7, eq160_e2026_d_n8, eq160_e2026_d_n9, eq160_e2026_d_n12, eq160_e2026_d_n14, eq160_e2026_d_n15, eq160_e2026_d_n16, eq160_e2026_d_n17, eq160_e2026_d_n18, eq160_e2026_d_n19, eq160_e2026_d_n20, eq160_e2026_d_n21, eq160_e2026_d_n22, eq160_e2026_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq160_reactive_node_derivatives: [f64; 23] = [eq160_e2028_d_n0, eq160_e2028_d_n1, eq160_e2028_d_n2, eq160_e2028_d_n3, eq160_e2028_d_n4, eq160_e2028_d_n5, eq160_e2028_d_n6, eq160_e2028_d_n7, eq160_e2028_d_n8, eq160_e2028_d_n9, 0.0, 0.0, eq160_e2028_d_n12, 0.0, eq160_e2028_d_n14, eq160_e2028_d_n15, eq160_e2028_d_n16, eq160_e2028_d_n17, eq160_e2028_d_n18, eq160_e2028_d_n19, eq160_e2028_d_n20, eq160_e2028_d_n21, eq160_e2028_d_n22];
        let eq160_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[20]),
            nodes,
            &eq160_reactive_node_derivatives,
            branches,
            &eq160_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq161_e2039, eq161_e2039_d_n0, eq161_e2039_d_n1, eq161_e2039_d_n2, eq161_e2039_d_n3, eq161_e2039_d_n4, eq161_e2039_d_n5, eq161_e2039_d_n6, eq161_e2039_d_n7, eq161_e2039_d_n8, eq161_e2039_d_n9, eq161_e2039_d_n12, eq161_e2039_d_n14, eq161_e2039_d_n15, eq161_e2039_d_n16, eq161_e2039_d_n17, eq161_e2039_d_n18, eq161_e2039_d_n19, eq161_e2039_d_n20, eq161_e2039_d_n21, eq161_e2039_d_n22, eq161_e2039_q,) = {
    if ((var_guard551 != 0.0) && (var_guard552 != 0.0)) {
        let eq161_e2035: f64 = (p.p252 * var_qg_fp2s);
        let eq161_e2035_d_n0: f64 = (p.p252 * var_qg_fp2s_dn0);
        let eq161_e2035_d_n1: f64 = (p.p252 * var_qg_fp2s_dn1);
        let eq161_e2035_d_n2: f64 = (p.p252 * var_qg_fp2s_dn2);
        let eq161_e2035_d_n3: f64 = (p.p252 * var_qg_fp2s_dn3);
        let eq161_e2035_d_n4: f64 = (p.p252 * var_qg_fp2s_dn4);
        let eq161_e2035_d_n5: f64 = (p.p252 * var_qg_fp2s_dn5);
        let eq161_e2035_d_n6: f64 = (p.p252 * var_qg_fp2s_dn6);
        let eq161_e2035_d_n7: f64 = (p.p252 * var_qg_fp2s_dn7);
        let eq161_e2035_d_n8: f64 = (p.p252 * var_qg_fp2s_dn8);
        let eq161_e2035_d_n9: f64 = (p.p252 * var_qg_fp2s_dn9);
        let eq161_e2035_d_n12: f64 = (p.p252 * var_qg_fp2s_dn12);
        let eq161_e2035_d_n14: f64 = (p.p252 * var_qg_fp2s_dn14);
        let eq161_e2035_d_n15: f64 = (p.p252 * var_qg_fp2s_dn15);
        let eq161_e2035_d_n16: f64 = (p.p252 * var_qg_fp2s_dn16);
        let eq161_e2035_d_n17: f64 = (p.p252 * var_qg_fp2s_dn17);
        let eq161_e2035_d_n18: f64 = (p.p252 * var_qg_fp2s_dn18);
        let eq161_e2035_d_n19: f64 = (p.p252 * var_qg_fp2s_dn19);
        let eq161_e2035_d_n20: f64 = (p.p252 * var_qg_fp2s_dn20);
        let eq161_e2035_d_n21: f64 = (p.p252 * var_qg_fp2s_dn21);
        let eq161_e2035_d_n22: f64 = (p.p252 * var_qg_fp2s_dn22);
        let eq161_e2036_q: f64 = eq161_e2035;
        let eq161_e2037: f64 = (p.p7 * eq161_e2035);
        let eq161_e2037_d_n0: f64 = (p.p7 * eq161_e2035_d_n0);
        let eq161_e2037_d_n1: f64 = (p.p7 * eq161_e2035_d_n1);
        let eq161_e2037_d_n2: f64 = (p.p7 * eq161_e2035_d_n2);
        let eq161_e2037_d_n3: f64 = (p.p7 * eq161_e2035_d_n3);
        let eq161_e2037_d_n4: f64 = (p.p7 * eq161_e2035_d_n4);
        let eq161_e2037_d_n5: f64 = (p.p7 * eq161_e2035_d_n5);
        let eq161_e2037_d_n6: f64 = (p.p7 * eq161_e2035_d_n6);
        let eq161_e2037_d_n7: f64 = (p.p7 * eq161_e2035_d_n7);
        let eq161_e2037_d_n8: f64 = (p.p7 * eq161_e2035_d_n8);
        let eq161_e2037_d_n9: f64 = (p.p7 * eq161_e2035_d_n9);
        let eq161_e2037_d_n12: f64 = (p.p7 * eq161_e2035_d_n12);
        let eq161_e2037_d_n14: f64 = (p.p7 * eq161_e2035_d_n14);
        let eq161_e2037_d_n15: f64 = (p.p7 * eq161_e2035_d_n15);
        let eq161_e2037_d_n16: f64 = (p.p7 * eq161_e2035_d_n16);
        let eq161_e2037_d_n17: f64 = (p.p7 * eq161_e2035_d_n17);
        let eq161_e2037_d_n18: f64 = (p.p7 * eq161_e2035_d_n18);
        let eq161_e2037_d_n19: f64 = (p.p7 * eq161_e2035_d_n19);
        let eq161_e2037_d_n20: f64 = (p.p7 * eq161_e2035_d_n20);
        let eq161_e2037_d_n21: f64 = (p.p7 * eq161_e2035_d_n21);
        let eq161_e2037_d_n22: f64 = (p.p7 * eq161_e2035_d_n22);
        let eq161_e2037_q: f64 = (p.p7 * eq161_e2036_q);
        (eq161_e2037, eq161_e2037_d_n0, eq161_e2037_d_n1, eq161_e2037_d_n2, eq161_e2037_d_n3, eq161_e2037_d_n4, eq161_e2037_d_n5, eq161_e2037_d_n6, eq161_e2037_d_n7, eq161_e2037_d_n8, eq161_e2037_d_n9, eq161_e2037_d_n12, eq161_e2037_d_n14, eq161_e2037_d_n15, eq161_e2037_d_n16, eq161_e2037_d_n17, eq161_e2037_d_n18, eq161_e2037_d_n19, eq161_e2037_d_n20, eq161_e2037_d_n21, eq161_e2037_d_n22, eq161_e2037_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq161_reactive_node_derivatives: [f64; 23] = [eq161_e2039_d_n0, eq161_e2039_d_n1, eq161_e2039_d_n2, eq161_e2039_d_n3, eq161_e2039_d_n4, eq161_e2039_d_n5, eq161_e2039_d_n6, eq161_e2039_d_n7, eq161_e2039_d_n8, eq161_e2039_d_n9, 0.0, 0.0, eq161_e2039_d_n12, 0.0, eq161_e2039_d_n14, eq161_e2039_d_n15, eq161_e2039_d_n16, eq161_e2039_d_n17, eq161_e2039_d_n18, eq161_e2039_d_n19, eq161_e2039_d_n20, eq161_e2039_d_n21, eq161_e2039_d_n22];
        let eq161_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[20]),
            nodes,
            &eq161_reactive_node_derivatives,
            branches,
            &eq161_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq162_e2049, eq162_e2049_d_n0, eq162_e2049_d_n1, eq162_e2049_d_n2, eq162_e2049_d_n3, eq162_e2049_d_n4, eq162_e2049_d_n5, eq162_e2049_d_n6, eq162_e2049_d_n7, eq162_e2049_d_n8, eq162_e2049_d_n9, eq162_e2049_d_n12, eq162_e2049_d_n14, eq162_e2049_d_n15, eq162_e2049_d_n16, eq162_e2049_d_n17, eq162_e2049_d_n18, eq162_e2049_d_n19, eq162_e2049_d_n20, eq162_e2049_d_n21, eq162_e2049_d_n22, eq162_e2049_q,) = {
    if ((var_guard551 == 0.0) && (var_guard554 != 0.0)) {
        let eq162_e2046_q: f64 = var_qd_fp2s;
        let eq162_e2047: f64 = (p.p7 * var_qd_fp2s);
        let eq162_e2047_d_n0: f64 = (p.p7 * var_qd_fp2s_dn0);
        let eq162_e2047_d_n1: f64 = (p.p7 * var_qd_fp2s_dn1);
        let eq162_e2047_d_n2: f64 = (p.p7 * var_qd_fp2s_dn2);
        let eq162_e2047_d_n3: f64 = (p.p7 * var_qd_fp2s_dn3);
        let eq162_e2047_d_n4: f64 = (p.p7 * var_qd_fp2s_dn4);
        let eq162_e2047_d_n5: f64 = (p.p7 * var_qd_fp2s_dn5);
        let eq162_e2047_d_n6: f64 = (p.p7 * var_qd_fp2s_dn6);
        let eq162_e2047_d_n7: f64 = (p.p7 * var_qd_fp2s_dn7);
        let eq162_e2047_d_n8: f64 = (p.p7 * var_qd_fp2s_dn8);
        let eq162_e2047_d_n9: f64 = (p.p7 * var_qd_fp2s_dn9);
        let eq162_e2047_d_n12: f64 = (p.p7 * var_qd_fp2s_dn12);
        let eq162_e2047_d_n14: f64 = (p.p7 * var_qd_fp2s_dn14);
        let eq162_e2047_d_n15: f64 = (p.p7 * var_qd_fp2s_dn15);
        let eq162_e2047_d_n16: f64 = (p.p7 * var_qd_fp2s_dn16);
        let eq162_e2047_d_n17: f64 = (p.p7 * var_qd_fp2s_dn17);
        let eq162_e2047_d_n18: f64 = (p.p7 * var_qd_fp2s_dn18);
        let eq162_e2047_d_n19: f64 = (p.p7 * var_qd_fp2s_dn19);
        let eq162_e2047_d_n20: f64 = (p.p7 * var_qd_fp2s_dn20);
        let eq162_e2047_d_n21: f64 = (p.p7 * var_qd_fp2s_dn21);
        let eq162_e2047_d_n22: f64 = (p.p7 * var_qd_fp2s_dn22);
        let eq162_e2047_q: f64 = (p.p7 * eq162_e2046_q);
        (eq162_e2047, eq162_e2047_d_n0, eq162_e2047_d_n1, eq162_e2047_d_n2, eq162_e2047_d_n3, eq162_e2047_d_n4, eq162_e2047_d_n5, eq162_e2047_d_n6, eq162_e2047_d_n7, eq162_e2047_d_n8, eq162_e2047_d_n9, eq162_e2047_d_n12, eq162_e2047_d_n14, eq162_e2047_d_n15, eq162_e2047_d_n16, eq162_e2047_d_n17, eq162_e2047_d_n18, eq162_e2047_d_n19, eq162_e2047_d_n20, eq162_e2047_d_n21, eq162_e2047_d_n22, eq162_e2047_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq162_reactive_node_derivatives: [f64; 23] = [eq162_e2049_d_n0, eq162_e2049_d_n1, eq162_e2049_d_n2, eq162_e2049_d_n3, eq162_e2049_d_n4, eq162_e2049_d_n5, eq162_e2049_d_n6, eq162_e2049_d_n7, eq162_e2049_d_n8, eq162_e2049_d_n9, 0.0, 0.0, eq162_e2049_d_n12, 0.0, eq162_e2049_d_n14, eq162_e2049_d_n15, eq162_e2049_d_n16, eq162_e2049_d_n17, eq162_e2049_d_n18, eq162_e2049_d_n19, eq162_e2049_d_n20, eq162_e2049_d_n21, eq162_e2049_d_n22];
        let eq162_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            nodes,
            &eq162_reactive_node_derivatives,
            branches,
            &eq162_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq163_e2061, eq163_e2061_d_n0, eq163_e2061_d_n1, eq163_e2061_d_n2, eq163_e2061_d_n3, eq163_e2061_d_n4, eq163_e2061_d_n5, eq163_e2061_d_n6, eq163_e2061_d_n7, eq163_e2061_d_n8, eq163_e2061_d_n9, eq163_e2061_d_n12, eq163_e2061_d_n14, eq163_e2061_d_n15, eq163_e2061_d_n16, eq163_e2061_d_n17, eq163_e2061_d_n18, eq163_e2061_d_n19, eq163_e2061_d_n20, eq163_e2061_d_n21, eq163_e2061_d_n22, eq163_e2061_q,) = {
    if (((var_guard551 == 0.0) && (var_guard554 != 0.0)) && (var_guard555 != 0.0)) {
        let eq163_e2058_q: f64 = var_qg_fp2s;
        let eq163_e2059: f64 = (p.p7 * var_qg_fp2s);
        let eq163_e2059_d_n0: f64 = (p.p7 * var_qg_fp2s_dn0);
        let eq163_e2059_d_n1: f64 = (p.p7 * var_qg_fp2s_dn1);
        let eq163_e2059_d_n2: f64 = (p.p7 * var_qg_fp2s_dn2);
        let eq163_e2059_d_n3: f64 = (p.p7 * var_qg_fp2s_dn3);
        let eq163_e2059_d_n4: f64 = (p.p7 * var_qg_fp2s_dn4);
        let eq163_e2059_d_n5: f64 = (p.p7 * var_qg_fp2s_dn5);
        let eq163_e2059_d_n6: f64 = (p.p7 * var_qg_fp2s_dn6);
        let eq163_e2059_d_n7: f64 = (p.p7 * var_qg_fp2s_dn7);
        let eq163_e2059_d_n8: f64 = (p.p7 * var_qg_fp2s_dn8);
        let eq163_e2059_d_n9: f64 = (p.p7 * var_qg_fp2s_dn9);
        let eq163_e2059_d_n12: f64 = (p.p7 * var_qg_fp2s_dn12);
        let eq163_e2059_d_n14: f64 = (p.p7 * var_qg_fp2s_dn14);
        let eq163_e2059_d_n15: f64 = (p.p7 * var_qg_fp2s_dn15);
        let eq163_e2059_d_n16: f64 = (p.p7 * var_qg_fp2s_dn16);
        let eq163_e2059_d_n17: f64 = (p.p7 * var_qg_fp2s_dn17);
        let eq163_e2059_d_n18: f64 = (p.p7 * var_qg_fp2s_dn18);
        let eq163_e2059_d_n19: f64 = (p.p7 * var_qg_fp2s_dn19);
        let eq163_e2059_d_n20: f64 = (p.p7 * var_qg_fp2s_dn20);
        let eq163_e2059_d_n21: f64 = (p.p7 * var_qg_fp2s_dn21);
        let eq163_e2059_d_n22: f64 = (p.p7 * var_qg_fp2s_dn22);
        let eq163_e2059_q: f64 = (p.p7 * eq163_e2058_q);
        (eq163_e2059, eq163_e2059_d_n0, eq163_e2059_d_n1, eq163_e2059_d_n2, eq163_e2059_d_n3, eq163_e2059_d_n4, eq163_e2059_d_n5, eq163_e2059_d_n6, eq163_e2059_d_n7, eq163_e2059_d_n8, eq163_e2059_d_n9, eq163_e2059_d_n12, eq163_e2059_d_n14, eq163_e2059_d_n15, eq163_e2059_d_n16, eq163_e2059_d_n17, eq163_e2059_d_n18, eq163_e2059_d_n19, eq163_e2059_d_n20, eq163_e2059_d_n21, eq163_e2059_d_n22, eq163_e2059_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq163_reactive_node_derivatives: [f64; 23] = [eq163_e2061_d_n0, eq163_e2061_d_n1, eq163_e2061_d_n2, eq163_e2061_d_n3, eq163_e2061_d_n4, eq163_e2061_d_n5, eq163_e2061_d_n6, eq163_e2061_d_n7, eq163_e2061_d_n8, eq163_e2061_d_n9, 0.0, 0.0, eq163_e2061_d_n12, 0.0, eq163_e2061_d_n14, eq163_e2061_d_n15, eq163_e2061_d_n16, eq163_e2061_d_n17, eq163_e2061_d_n18, eq163_e2061_d_n19, eq163_e2061_d_n20, eq163_e2061_d_n21, eq163_e2061_d_n22];
        let eq163_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq163_reactive_node_derivatives,
            branches,
            &eq163_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq164_e2075, eq164_e2075_d_n0, eq164_e2075_d_n1, eq164_e2075_d_n2, eq164_e2075_d_n3, eq164_e2075_d_n4, eq164_e2075_d_n5, eq164_e2075_d_n6, eq164_e2075_d_n7, eq164_e2075_d_n8, eq164_e2075_d_n9, eq164_e2075_d_n12, eq164_e2075_d_n14, eq164_e2075_d_n15, eq164_e2075_d_n16, eq164_e2075_d_n17, eq164_e2075_d_n18, eq164_e2075_d_n19, eq164_e2075_d_n20, eq164_e2075_d_n21, eq164_e2075_d_n22, eq164_e2075_q,) = {
    if (((var_guard551 == 0.0) && (var_guard554 != 0.0)) && (var_guard555 != 0.0)) {
        let eq164_e2070_q: f64 = var_qg_fp2s;
        let eq164_e2071: f64 = (p.p7 * var_qg_fp2s);
        let eq164_e2071_d_n0: f64 = (p.p7 * var_qg_fp2s_dn0);
        let eq164_e2071_d_n1: f64 = (p.p7 * var_qg_fp2s_dn1);
        let eq164_e2071_d_n2: f64 = (p.p7 * var_qg_fp2s_dn2);
        let eq164_e2071_d_n3: f64 = (p.p7 * var_qg_fp2s_dn3);
        let eq164_e2071_d_n4: f64 = (p.p7 * var_qg_fp2s_dn4);
        let eq164_e2071_d_n5: f64 = (p.p7 * var_qg_fp2s_dn5);
        let eq164_e2071_d_n6: f64 = (p.p7 * var_qg_fp2s_dn6);
        let eq164_e2071_d_n7: f64 = (p.p7 * var_qg_fp2s_dn7);
        let eq164_e2071_d_n8: f64 = (p.p7 * var_qg_fp2s_dn8);
        let eq164_e2071_d_n9: f64 = (p.p7 * var_qg_fp2s_dn9);
        let eq164_e2071_d_n12: f64 = (p.p7 * var_qg_fp2s_dn12);
        let eq164_e2071_d_n14: f64 = (p.p7 * var_qg_fp2s_dn14);
        let eq164_e2071_d_n15: f64 = (p.p7 * var_qg_fp2s_dn15);
        let eq164_e2071_d_n16: f64 = (p.p7 * var_qg_fp2s_dn16);
        let eq164_e2071_d_n17: f64 = (p.p7 * var_qg_fp2s_dn17);
        let eq164_e2071_d_n18: f64 = (p.p7 * var_qg_fp2s_dn18);
        let eq164_e2071_d_n19: f64 = (p.p7 * var_qg_fp2s_dn19);
        let eq164_e2071_d_n20: f64 = (p.p7 * var_qg_fp2s_dn20);
        let eq164_e2071_d_n21: f64 = (p.p7 * var_qg_fp2s_dn21);
        let eq164_e2071_d_n22: f64 = (p.p7 * var_qg_fp2s_dn22);
        let eq164_e2071_q: f64 = (p.p7 * eq164_e2070_q);
        let eq164_e2073: f64 = (eq164_e2071 * p.p247);
        let eq164_e2073_d_n0: f64 = (eq164_e2071_d_n0 * p.p247);
        let eq164_e2073_d_n1: f64 = (eq164_e2071_d_n1 * p.p247);
        let eq164_e2073_d_n2: f64 = (eq164_e2071_d_n2 * p.p247);
        let eq164_e2073_d_n3: f64 = (eq164_e2071_d_n3 * p.p247);
        let eq164_e2073_d_n4: f64 = (eq164_e2071_d_n4 * p.p247);
        let eq164_e2073_d_n5: f64 = (eq164_e2071_d_n5 * p.p247);
        let eq164_e2073_d_n6: f64 = (eq164_e2071_d_n6 * p.p247);
        let eq164_e2073_d_n7: f64 = (eq164_e2071_d_n7 * p.p247);
        let eq164_e2073_d_n8: f64 = (eq164_e2071_d_n8 * p.p247);
        let eq164_e2073_d_n9: f64 = (eq164_e2071_d_n9 * p.p247);
        let eq164_e2073_d_n12: f64 = (eq164_e2071_d_n12 * p.p247);
        let eq164_e2073_d_n14: f64 = (eq164_e2071_d_n14 * p.p247);
        let eq164_e2073_d_n15: f64 = (eq164_e2071_d_n15 * p.p247);
        let eq164_e2073_d_n16: f64 = (eq164_e2071_d_n16 * p.p247);
        let eq164_e2073_d_n17: f64 = (eq164_e2071_d_n17 * p.p247);
        let eq164_e2073_d_n18: f64 = (eq164_e2071_d_n18 * p.p247);
        let eq164_e2073_d_n19: f64 = (eq164_e2071_d_n19 * p.p247);
        let eq164_e2073_d_n20: f64 = (eq164_e2071_d_n20 * p.p247);
        let eq164_e2073_d_n21: f64 = (eq164_e2071_d_n21 * p.p247);
        let eq164_e2073_d_n22: f64 = (eq164_e2071_d_n22 * p.p247);
        let eq164_e2073_q: f64 = (eq164_e2071_q * p.p247);
        (eq164_e2073, eq164_e2073_d_n0, eq164_e2073_d_n1, eq164_e2073_d_n2, eq164_e2073_d_n3, eq164_e2073_d_n4, eq164_e2073_d_n5, eq164_e2073_d_n6, eq164_e2073_d_n7, eq164_e2073_d_n8, eq164_e2073_d_n9, eq164_e2073_d_n12, eq164_e2073_d_n14, eq164_e2073_d_n15, eq164_e2073_d_n16, eq164_e2073_d_n17, eq164_e2073_d_n18, eq164_e2073_d_n19, eq164_e2073_d_n20, eq164_e2073_d_n21, eq164_e2073_d_n22, eq164_e2073_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq164_reactive_node_derivatives: [f64; 23] = [eq164_e2075_d_n0, eq164_e2075_d_n1, eq164_e2075_d_n2, eq164_e2075_d_n3, eq164_e2075_d_n4, eq164_e2075_d_n5, eq164_e2075_d_n6, eq164_e2075_d_n7, eq164_e2075_d_n8, eq164_e2075_d_n9, 0.0, 0.0, eq164_e2075_d_n12, 0.0, eq164_e2075_d_n14, eq164_e2075_d_n15, eq164_e2075_d_n16, eq164_e2075_d_n17, eq164_e2075_d_n18, eq164_e2075_d_n19, eq164_e2075_d_n20, eq164_e2075_d_n21, eq164_e2075_d_n22];
        let eq164_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            nodes,
            &eq164_reactive_node_derivatives,
            branches,
            &eq164_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq165_e2088, eq165_e2088_d_n0, eq165_e2088_d_n1, eq165_e2088_d_n2, eq165_e2088_d_n3, eq165_e2088_d_n4, eq165_e2088_d_n5, eq165_e2088_d_n6, eq165_e2088_d_n7, eq165_e2088_d_n8, eq165_e2088_d_n9, eq165_e2088_d_n12, eq165_e2088_d_n14, eq165_e2088_d_n15, eq165_e2088_d_n16, eq165_e2088_d_n17, eq165_e2088_d_n18, eq165_e2088_d_n19, eq165_e2088_d_n20, eq165_e2088_d_n21, eq165_e2088_d_n22, eq165_e2088_q,) = {
    if (((var_guard551 == 0.0) && (var_guard554 != 0.0)) && (var_guard555 == 0.0)) {
        let eq165_e2085_q: f64 = var_qg_fp2s;
        let eq165_e2086: f64 = (p.p7 * var_qg_fp2s);
        let eq165_e2086_d_n0: f64 = (p.p7 * var_qg_fp2s_dn0);
        let eq165_e2086_d_n1: f64 = (p.p7 * var_qg_fp2s_dn1);
        let eq165_e2086_d_n2: f64 = (p.p7 * var_qg_fp2s_dn2);
        let eq165_e2086_d_n3: f64 = (p.p7 * var_qg_fp2s_dn3);
        let eq165_e2086_d_n4: f64 = (p.p7 * var_qg_fp2s_dn4);
        let eq165_e2086_d_n5: f64 = (p.p7 * var_qg_fp2s_dn5);
        let eq165_e2086_d_n6: f64 = (p.p7 * var_qg_fp2s_dn6);
        let eq165_e2086_d_n7: f64 = (p.p7 * var_qg_fp2s_dn7);
        let eq165_e2086_d_n8: f64 = (p.p7 * var_qg_fp2s_dn8);
        let eq165_e2086_d_n9: f64 = (p.p7 * var_qg_fp2s_dn9);
        let eq165_e2086_d_n12: f64 = (p.p7 * var_qg_fp2s_dn12);
        let eq165_e2086_d_n14: f64 = (p.p7 * var_qg_fp2s_dn14);
        let eq165_e2086_d_n15: f64 = (p.p7 * var_qg_fp2s_dn15);
        let eq165_e2086_d_n16: f64 = (p.p7 * var_qg_fp2s_dn16);
        let eq165_e2086_d_n17: f64 = (p.p7 * var_qg_fp2s_dn17);
        let eq165_e2086_d_n18: f64 = (p.p7 * var_qg_fp2s_dn18);
        let eq165_e2086_d_n19: f64 = (p.p7 * var_qg_fp2s_dn19);
        let eq165_e2086_d_n20: f64 = (p.p7 * var_qg_fp2s_dn20);
        let eq165_e2086_d_n21: f64 = (p.p7 * var_qg_fp2s_dn21);
        let eq165_e2086_d_n22: f64 = (p.p7 * var_qg_fp2s_dn22);
        let eq165_e2086_q: f64 = (p.p7 * eq165_e2085_q);
        (eq165_e2086, eq165_e2086_d_n0, eq165_e2086_d_n1, eq165_e2086_d_n2, eq165_e2086_d_n3, eq165_e2086_d_n4, eq165_e2086_d_n5, eq165_e2086_d_n6, eq165_e2086_d_n7, eq165_e2086_d_n8, eq165_e2086_d_n9, eq165_e2086_d_n12, eq165_e2086_d_n14, eq165_e2086_d_n15, eq165_e2086_d_n16, eq165_e2086_d_n17, eq165_e2086_d_n18, eq165_e2086_d_n19, eq165_e2086_d_n20, eq165_e2086_d_n21, eq165_e2086_d_n22, eq165_e2086_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq165_reactive_node_derivatives: [f64; 23] = [eq165_e2088_d_n0, eq165_e2088_d_n1, eq165_e2088_d_n2, eq165_e2088_d_n3, eq165_e2088_d_n4, eq165_e2088_d_n5, eq165_e2088_d_n6, eq165_e2088_d_n7, eq165_e2088_d_n8, eq165_e2088_d_n9, 0.0, 0.0, eq165_e2088_d_n12, 0.0, eq165_e2088_d_n14, eq165_e2088_d_n15, eq165_e2088_d_n16, eq165_e2088_d_n17, eq165_e2088_d_n18, eq165_e2088_d_n19, eq165_e2088_d_n20, eq165_e2088_d_n21, eq165_e2088_d_n22];
        let eq165_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            nodes,
            &eq165_reactive_node_derivatives,
            branches,
            &eq165_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq166_e2103, eq166_e2103_d_n0, eq166_e2103_d_n1, eq166_e2103_d_n2, eq166_e2103_d_n3, eq166_e2103_d_n4, eq166_e2103_d_n5, eq166_e2103_d_n6, eq166_e2103_d_n7, eq166_e2103_d_n8, eq166_e2103_d_n9, eq166_e2103_d_n12, eq166_e2103_d_n14, eq166_e2103_d_n15, eq166_e2103_d_n16, eq166_e2103_d_n17, eq166_e2103_d_n18, eq166_e2103_d_n19, eq166_e2103_d_n20, eq166_e2103_d_n21, eq166_e2103_d_n22, eq166_e2103_q,) = {
    if (((var_guard551 == 0.0) && (var_guard554 != 0.0)) && (var_guard555 == 0.0)) {
        let eq166_e2098_q: f64 = var_qg_fp2s;
        let eq166_e2099: f64 = (p.p7 * var_qg_fp2s);
        let eq166_e2099_d_n0: f64 = (p.p7 * var_qg_fp2s_dn0);
        let eq166_e2099_d_n1: f64 = (p.p7 * var_qg_fp2s_dn1);
        let eq166_e2099_d_n2: f64 = (p.p7 * var_qg_fp2s_dn2);
        let eq166_e2099_d_n3: f64 = (p.p7 * var_qg_fp2s_dn3);
        let eq166_e2099_d_n4: f64 = (p.p7 * var_qg_fp2s_dn4);
        let eq166_e2099_d_n5: f64 = (p.p7 * var_qg_fp2s_dn5);
        let eq166_e2099_d_n6: f64 = (p.p7 * var_qg_fp2s_dn6);
        let eq166_e2099_d_n7: f64 = (p.p7 * var_qg_fp2s_dn7);
        let eq166_e2099_d_n8: f64 = (p.p7 * var_qg_fp2s_dn8);
        let eq166_e2099_d_n9: f64 = (p.p7 * var_qg_fp2s_dn9);
        let eq166_e2099_d_n12: f64 = (p.p7 * var_qg_fp2s_dn12);
        let eq166_e2099_d_n14: f64 = (p.p7 * var_qg_fp2s_dn14);
        let eq166_e2099_d_n15: f64 = (p.p7 * var_qg_fp2s_dn15);
        let eq166_e2099_d_n16: f64 = (p.p7 * var_qg_fp2s_dn16);
        let eq166_e2099_d_n17: f64 = (p.p7 * var_qg_fp2s_dn17);
        let eq166_e2099_d_n18: f64 = (p.p7 * var_qg_fp2s_dn18);
        let eq166_e2099_d_n19: f64 = (p.p7 * var_qg_fp2s_dn19);
        let eq166_e2099_d_n20: f64 = (p.p7 * var_qg_fp2s_dn20);
        let eq166_e2099_d_n21: f64 = (p.p7 * var_qg_fp2s_dn21);
        let eq166_e2099_d_n22: f64 = (p.p7 * var_qg_fp2s_dn22);
        let eq166_e2099_q: f64 = (p.p7 * eq166_e2098_q);
        let eq166_e2101: f64 = (eq166_e2099 * p.p247);
        let eq166_e2101_d_n0: f64 = (eq166_e2099_d_n0 * p.p247);
        let eq166_e2101_d_n1: f64 = (eq166_e2099_d_n1 * p.p247);
        let eq166_e2101_d_n2: f64 = (eq166_e2099_d_n2 * p.p247);
        let eq166_e2101_d_n3: f64 = (eq166_e2099_d_n3 * p.p247);
        let eq166_e2101_d_n4: f64 = (eq166_e2099_d_n4 * p.p247);
        let eq166_e2101_d_n5: f64 = (eq166_e2099_d_n5 * p.p247);
        let eq166_e2101_d_n6: f64 = (eq166_e2099_d_n6 * p.p247);
        let eq166_e2101_d_n7: f64 = (eq166_e2099_d_n7 * p.p247);
        let eq166_e2101_d_n8: f64 = (eq166_e2099_d_n8 * p.p247);
        let eq166_e2101_d_n9: f64 = (eq166_e2099_d_n9 * p.p247);
        let eq166_e2101_d_n12: f64 = (eq166_e2099_d_n12 * p.p247);
        let eq166_e2101_d_n14: f64 = (eq166_e2099_d_n14 * p.p247);
        let eq166_e2101_d_n15: f64 = (eq166_e2099_d_n15 * p.p247);
        let eq166_e2101_d_n16: f64 = (eq166_e2099_d_n16 * p.p247);
        let eq166_e2101_d_n17: f64 = (eq166_e2099_d_n17 * p.p247);
        let eq166_e2101_d_n18: f64 = (eq166_e2099_d_n18 * p.p247);
        let eq166_e2101_d_n19: f64 = (eq166_e2099_d_n19 * p.p247);
        let eq166_e2101_d_n20: f64 = (eq166_e2099_d_n20 * p.p247);
        let eq166_e2101_d_n21: f64 = (eq166_e2099_d_n21 * p.p247);
        let eq166_e2101_d_n22: f64 = (eq166_e2099_d_n22 * p.p247);
        let eq166_e2101_q: f64 = (eq166_e2099_q * p.p247);
        (eq166_e2101, eq166_e2101_d_n0, eq166_e2101_d_n1, eq166_e2101_d_n2, eq166_e2101_d_n3, eq166_e2101_d_n4, eq166_e2101_d_n5, eq166_e2101_d_n6, eq166_e2101_d_n7, eq166_e2101_d_n8, eq166_e2101_d_n9, eq166_e2101_d_n12, eq166_e2101_d_n14, eq166_e2101_d_n15, eq166_e2101_d_n16, eq166_e2101_d_n17, eq166_e2101_d_n18, eq166_e2101_d_n19, eq166_e2101_d_n20, eq166_e2101_d_n21, eq166_e2101_d_n22, eq166_e2101_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq166_reactive_node_derivatives: [f64; 23] = [eq166_e2103_d_n0, eq166_e2103_d_n1, eq166_e2103_d_n2, eq166_e2103_d_n3, eq166_e2103_d_n4, eq166_e2103_d_n5, eq166_e2103_d_n6, eq166_e2103_d_n7, eq166_e2103_d_n8, eq166_e2103_d_n9, 0.0, 0.0, eq166_e2103_d_n12, 0.0, eq166_e2103_d_n14, eq166_e2103_d_n15, eq166_e2103_d_n16, eq166_e2103_d_n17, eq166_e2103_d_n18, eq166_e2103_d_n19, eq166_e2103_d_n20, eq166_e2103_d_n21, eq166_e2103_d_n22];
        let eq166_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq166_reactive_node_derivatives,
            branches,
            &eq166_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq167_e2115, eq167_e2115_d_n0, eq167_e2115_d_n1, eq167_e2115_d_n2, eq167_e2115_d_n3, eq167_e2115_d_n4, eq167_e2115_d_n5, eq167_e2115_d_n6, eq167_e2115_d_n7, eq167_e2115_d_n8, eq167_e2115_d_n9, eq167_e2115_d_n12, eq167_e2115_d_n14, eq167_e2115_d_n15, eq167_e2115_d_n16, eq167_e2115_d_n17, eq167_e2115_d_n18, eq167_e2115_d_n19, eq167_e2115_d_n20, eq167_e2115_d_n21, eq167_e2115_d_n22, eq167_e2115_q,) = {
    if ((var_guard551 == 0.0) && (var_guard554 != 0.0)) {
        let eq167_e2111: f64 = (p.p252 * var_qg_fp2s);
        let eq167_e2111_d_n0: f64 = (p.p252 * var_qg_fp2s_dn0);
        let eq167_e2111_d_n1: f64 = (p.p252 * var_qg_fp2s_dn1);
        let eq167_e2111_d_n2: f64 = (p.p252 * var_qg_fp2s_dn2);
        let eq167_e2111_d_n3: f64 = (p.p252 * var_qg_fp2s_dn3);
        let eq167_e2111_d_n4: f64 = (p.p252 * var_qg_fp2s_dn4);
        let eq167_e2111_d_n5: f64 = (p.p252 * var_qg_fp2s_dn5);
        let eq167_e2111_d_n6: f64 = (p.p252 * var_qg_fp2s_dn6);
        let eq167_e2111_d_n7: f64 = (p.p252 * var_qg_fp2s_dn7);
        let eq167_e2111_d_n8: f64 = (p.p252 * var_qg_fp2s_dn8);
        let eq167_e2111_d_n9: f64 = (p.p252 * var_qg_fp2s_dn9);
        let eq167_e2111_d_n12: f64 = (p.p252 * var_qg_fp2s_dn12);
        let eq167_e2111_d_n14: f64 = (p.p252 * var_qg_fp2s_dn14);
        let eq167_e2111_d_n15: f64 = (p.p252 * var_qg_fp2s_dn15);
        let eq167_e2111_d_n16: f64 = (p.p252 * var_qg_fp2s_dn16);
        let eq167_e2111_d_n17: f64 = (p.p252 * var_qg_fp2s_dn17);
        let eq167_e2111_d_n18: f64 = (p.p252 * var_qg_fp2s_dn18);
        let eq167_e2111_d_n19: f64 = (p.p252 * var_qg_fp2s_dn19);
        let eq167_e2111_d_n20: f64 = (p.p252 * var_qg_fp2s_dn20);
        let eq167_e2111_d_n21: f64 = (p.p252 * var_qg_fp2s_dn21);
        let eq167_e2111_d_n22: f64 = (p.p252 * var_qg_fp2s_dn22);
        let eq167_e2112_q: f64 = eq167_e2111;
        let eq167_e2113: f64 = (p.p7 * eq167_e2111);
        let eq167_e2113_d_n0: f64 = (p.p7 * eq167_e2111_d_n0);
        let eq167_e2113_d_n1: f64 = (p.p7 * eq167_e2111_d_n1);
        let eq167_e2113_d_n2: f64 = (p.p7 * eq167_e2111_d_n2);
        let eq167_e2113_d_n3: f64 = (p.p7 * eq167_e2111_d_n3);
        let eq167_e2113_d_n4: f64 = (p.p7 * eq167_e2111_d_n4);
        let eq167_e2113_d_n5: f64 = (p.p7 * eq167_e2111_d_n5);
        let eq167_e2113_d_n6: f64 = (p.p7 * eq167_e2111_d_n6);
        let eq167_e2113_d_n7: f64 = (p.p7 * eq167_e2111_d_n7);
        let eq167_e2113_d_n8: f64 = (p.p7 * eq167_e2111_d_n8);
        let eq167_e2113_d_n9: f64 = (p.p7 * eq167_e2111_d_n9);
        let eq167_e2113_d_n12: f64 = (p.p7 * eq167_e2111_d_n12);
        let eq167_e2113_d_n14: f64 = (p.p7 * eq167_e2111_d_n14);
        let eq167_e2113_d_n15: f64 = (p.p7 * eq167_e2111_d_n15);
        let eq167_e2113_d_n16: f64 = (p.p7 * eq167_e2111_d_n16);
        let eq167_e2113_d_n17: f64 = (p.p7 * eq167_e2111_d_n17);
        let eq167_e2113_d_n18: f64 = (p.p7 * eq167_e2111_d_n18);
        let eq167_e2113_d_n19: f64 = (p.p7 * eq167_e2111_d_n19);
        let eq167_e2113_d_n20: f64 = (p.p7 * eq167_e2111_d_n20);
        let eq167_e2113_d_n21: f64 = (p.p7 * eq167_e2111_d_n21);
        let eq167_e2113_d_n22: f64 = (p.p7 * eq167_e2111_d_n22);
        let eq167_e2113_q: f64 = (p.p7 * eq167_e2112_q);
        (eq167_e2113, eq167_e2113_d_n0, eq167_e2113_d_n1, eq167_e2113_d_n2, eq167_e2113_d_n3, eq167_e2113_d_n4, eq167_e2113_d_n5, eq167_e2113_d_n6, eq167_e2113_d_n7, eq167_e2113_d_n8, eq167_e2113_d_n9, eq167_e2113_d_n12, eq167_e2113_d_n14, eq167_e2113_d_n15, eq167_e2113_d_n16, eq167_e2113_d_n17, eq167_e2113_d_n18, eq167_e2113_d_n19, eq167_e2113_d_n20, eq167_e2113_d_n21, eq167_e2113_d_n22, eq167_e2113_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq167_reactive_node_derivatives: [f64; 23] = [eq167_e2115_d_n0, eq167_e2115_d_n1, eq167_e2115_d_n2, eq167_e2115_d_n3, eq167_e2115_d_n4, eq167_e2115_d_n5, eq167_e2115_d_n6, eq167_e2115_d_n7, eq167_e2115_d_n8, eq167_e2115_d_n9, 0.0, 0.0, eq167_e2115_d_n12, 0.0, eq167_e2115_d_n14, eq167_e2115_d_n15, eq167_e2115_d_n16, eq167_e2115_d_n17, eq167_e2115_d_n18, eq167_e2115_d_n19, eq167_e2115_d_n20, eq167_e2115_d_n21, eq167_e2115_d_n22];
        let eq167_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            nodes,
            &eq167_reactive_node_derivatives,
            branches,
            &eq167_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_6(
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_guard556: f64,
        var_guard557: f64,
        var_guard558: f64,
        var_guard559: f64,
        var_guard560: f64,
        var_qd_fp3: f64,
        var_qd_fp3_dn0: f64,
        var_qd_fp3_dn1: f64,
        var_qd_fp3_dn12: f64,
        var_qd_fp3_dn14: f64,
        var_qd_fp3_dn15: f64,
        var_qd_fp3_dn16: f64,
        var_qd_fp3_dn17: f64,
        var_qd_fp3_dn18: f64,
        var_qd_fp3_dn19: f64,
        var_qd_fp3_dn2: f64,
        var_qd_fp3_dn20: f64,
        var_qd_fp3_dn21: f64,
        var_qd_fp3_dn22: f64,
        var_qd_fp3_dn3: f64,
        var_qd_fp3_dn4: f64,
        var_qd_fp3_dn5: f64,
        var_qd_fp3_dn6: f64,
        var_qd_fp3_dn7: f64,
        var_qd_fp3_dn8: f64,
        var_qd_fp3_dn9: f64,
        var_qg_fp3: f64,
        var_qg_fp3_dn0: f64,
        var_qg_fp3_dn1: f64,
        var_qg_fp3_dn12: f64,
        var_qg_fp3_dn14: f64,
        var_qg_fp3_dn15: f64,
        var_qg_fp3_dn16: f64,
        var_qg_fp3_dn17: f64,
        var_qg_fp3_dn18: f64,
        var_qg_fp3_dn19: f64,
        var_qg_fp3_dn2: f64,
        var_qg_fp3_dn20: f64,
        var_qg_fp3_dn21: f64,
        var_qg_fp3_dn22: f64,
        var_qg_fp3_dn3: f64,
        var_qg_fp3_dn4: f64,
        var_qg_fp3_dn5: f64,
        var_qg_fp3_dn6: f64,
        var_qg_fp3_dn7: f64,
        var_qg_fp3_dn8: f64,
        var_qg_fp3_dn9: f64,
    ) {
        let (eq168_e2124, eq168_e2124_d_n0, eq168_e2124_d_n1, eq168_e2124_d_n2, eq168_e2124_d_n3, eq168_e2124_d_n4, eq168_e2124_d_n5, eq168_e2124_d_n6, eq168_e2124_d_n7, eq168_e2124_d_n8, eq168_e2124_d_n9, eq168_e2124_d_n12, eq168_e2124_d_n14, eq168_e2124_d_n15, eq168_e2124_d_n16, eq168_e2124_d_n17, eq168_e2124_d_n18, eq168_e2124_d_n19, eq168_e2124_d_n20, eq168_e2124_d_n21, eq168_e2124_d_n22, eq168_e2124_q,) = {
    if ((var_guard556 != 0.0) && (var_guard557 != 0.0)) {
        let eq168_e2121_q: f64 = var_qd_fp3;
        let eq168_e2122: f64 = (p.p7 * var_qd_fp3);
        let eq168_e2122_d_n0: f64 = (p.p7 * var_qd_fp3_dn0);
        let eq168_e2122_d_n1: f64 = (p.p7 * var_qd_fp3_dn1);
        let eq168_e2122_d_n2: f64 = (p.p7 * var_qd_fp3_dn2);
        let eq168_e2122_d_n3: f64 = (p.p7 * var_qd_fp3_dn3);
        let eq168_e2122_d_n4: f64 = (p.p7 * var_qd_fp3_dn4);
        let eq168_e2122_d_n5: f64 = (p.p7 * var_qd_fp3_dn5);
        let eq168_e2122_d_n6: f64 = (p.p7 * var_qd_fp3_dn6);
        let eq168_e2122_d_n7: f64 = (p.p7 * var_qd_fp3_dn7);
        let eq168_e2122_d_n8: f64 = (p.p7 * var_qd_fp3_dn8);
        let eq168_e2122_d_n9: f64 = (p.p7 * var_qd_fp3_dn9);
        let eq168_e2122_d_n12: f64 = (p.p7 * var_qd_fp3_dn12);
        let eq168_e2122_d_n14: f64 = (p.p7 * var_qd_fp3_dn14);
        let eq168_e2122_d_n15: f64 = (p.p7 * var_qd_fp3_dn15);
        let eq168_e2122_d_n16: f64 = (p.p7 * var_qd_fp3_dn16);
        let eq168_e2122_d_n17: f64 = (p.p7 * var_qd_fp3_dn17);
        let eq168_e2122_d_n18: f64 = (p.p7 * var_qd_fp3_dn18);
        let eq168_e2122_d_n19: f64 = (p.p7 * var_qd_fp3_dn19);
        let eq168_e2122_d_n20: f64 = (p.p7 * var_qd_fp3_dn20);
        let eq168_e2122_d_n21: f64 = (p.p7 * var_qd_fp3_dn21);
        let eq168_e2122_d_n22: f64 = (p.p7 * var_qd_fp3_dn22);
        let eq168_e2122_q: f64 = (p.p7 * eq168_e2121_q);
        (eq168_e2122, eq168_e2122_d_n0, eq168_e2122_d_n1, eq168_e2122_d_n2, eq168_e2122_d_n3, eq168_e2122_d_n4, eq168_e2122_d_n5, eq168_e2122_d_n6, eq168_e2122_d_n7, eq168_e2122_d_n8, eq168_e2122_d_n9, eq168_e2122_d_n12, eq168_e2122_d_n14, eq168_e2122_d_n15, eq168_e2122_d_n16, eq168_e2122_d_n17, eq168_e2122_d_n18, eq168_e2122_d_n19, eq168_e2122_d_n20, eq168_e2122_d_n21, eq168_e2122_d_n22, eq168_e2122_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq168_reactive_node_derivatives: [f64; 23] = [eq168_e2124_d_n0, eq168_e2124_d_n1, eq168_e2124_d_n2, eq168_e2124_d_n3, eq168_e2124_d_n4, eq168_e2124_d_n5, eq168_e2124_d_n6, eq168_e2124_d_n7, eq168_e2124_d_n8, eq168_e2124_d_n9, 0.0, 0.0, eq168_e2124_d_n12, 0.0, eq168_e2124_d_n14, eq168_e2124_d_n15, eq168_e2124_d_n16, eq168_e2124_d_n17, eq168_e2124_d_n18, eq168_e2124_d_n19, eq168_e2124_d_n20, eq168_e2124_d_n21, eq168_e2124_d_n22];
        let eq168_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[17]),
            Some(nodes[16]),
            nodes,
            &eq168_reactive_node_derivatives,
            branches,
            &eq168_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq169_e2135, eq169_e2135_d_n0, eq169_e2135_d_n1, eq169_e2135_d_n2, eq169_e2135_d_n3, eq169_e2135_d_n4, eq169_e2135_d_n5, eq169_e2135_d_n6, eq169_e2135_d_n7, eq169_e2135_d_n8, eq169_e2135_d_n9, eq169_e2135_d_n12, eq169_e2135_d_n14, eq169_e2135_d_n15, eq169_e2135_d_n16, eq169_e2135_d_n17, eq169_e2135_d_n18, eq169_e2135_d_n19, eq169_e2135_d_n20, eq169_e2135_d_n21, eq169_e2135_d_n22, eq169_e2135_q,) = {
    if (((var_guard556 != 0.0) && (var_guard557 != 0.0)) && (var_guard558 != 0.0)) {
        let eq169_e2132_q: f64 = var_qg_fp3;
        let eq169_e2133: f64 = (p.p7 * var_qg_fp3);
        let eq169_e2133_d_n0: f64 = (p.p7 * var_qg_fp3_dn0);
        let eq169_e2133_d_n1: f64 = (p.p7 * var_qg_fp3_dn1);
        let eq169_e2133_d_n2: f64 = (p.p7 * var_qg_fp3_dn2);
        let eq169_e2133_d_n3: f64 = (p.p7 * var_qg_fp3_dn3);
        let eq169_e2133_d_n4: f64 = (p.p7 * var_qg_fp3_dn4);
        let eq169_e2133_d_n5: f64 = (p.p7 * var_qg_fp3_dn5);
        let eq169_e2133_d_n6: f64 = (p.p7 * var_qg_fp3_dn6);
        let eq169_e2133_d_n7: f64 = (p.p7 * var_qg_fp3_dn7);
        let eq169_e2133_d_n8: f64 = (p.p7 * var_qg_fp3_dn8);
        let eq169_e2133_d_n9: f64 = (p.p7 * var_qg_fp3_dn9);
        let eq169_e2133_d_n12: f64 = (p.p7 * var_qg_fp3_dn12);
        let eq169_e2133_d_n14: f64 = (p.p7 * var_qg_fp3_dn14);
        let eq169_e2133_d_n15: f64 = (p.p7 * var_qg_fp3_dn15);
        let eq169_e2133_d_n16: f64 = (p.p7 * var_qg_fp3_dn16);
        let eq169_e2133_d_n17: f64 = (p.p7 * var_qg_fp3_dn17);
        let eq169_e2133_d_n18: f64 = (p.p7 * var_qg_fp3_dn18);
        let eq169_e2133_d_n19: f64 = (p.p7 * var_qg_fp3_dn19);
        let eq169_e2133_d_n20: f64 = (p.p7 * var_qg_fp3_dn20);
        let eq169_e2133_d_n21: f64 = (p.p7 * var_qg_fp3_dn21);
        let eq169_e2133_d_n22: f64 = (p.p7 * var_qg_fp3_dn22);
        let eq169_e2133_q: f64 = (p.p7 * eq169_e2132_q);
        (eq169_e2133, eq169_e2133_d_n0, eq169_e2133_d_n1, eq169_e2133_d_n2, eq169_e2133_d_n3, eq169_e2133_d_n4, eq169_e2133_d_n5, eq169_e2133_d_n6, eq169_e2133_d_n7, eq169_e2133_d_n8, eq169_e2133_d_n9, eq169_e2133_d_n12, eq169_e2133_d_n14, eq169_e2133_d_n15, eq169_e2133_d_n16, eq169_e2133_d_n17, eq169_e2133_d_n18, eq169_e2133_d_n19, eq169_e2133_d_n20, eq169_e2133_d_n21, eq169_e2133_d_n22, eq169_e2133_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq169_reactive_node_derivatives: [f64; 23] = [eq169_e2135_d_n0, eq169_e2135_d_n1, eq169_e2135_d_n2, eq169_e2135_d_n3, eq169_e2135_d_n4, eq169_e2135_d_n5, eq169_e2135_d_n6, eq169_e2135_d_n7, eq169_e2135_d_n8, eq169_e2135_d_n9, 0.0, 0.0, eq169_e2135_d_n12, 0.0, eq169_e2135_d_n14, eq169_e2135_d_n15, eq169_e2135_d_n16, eq169_e2135_d_n17, eq169_e2135_d_n18, eq169_e2135_d_n19, eq169_e2135_d_n20, eq169_e2135_d_n21, eq169_e2135_d_n22];
        let eq169_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[16]),
            nodes,
            &eq169_reactive_node_derivatives,
            branches,
            &eq169_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq170_e2148, eq170_e2148_d_n0, eq170_e2148_d_n1, eq170_e2148_d_n2, eq170_e2148_d_n3, eq170_e2148_d_n4, eq170_e2148_d_n5, eq170_e2148_d_n6, eq170_e2148_d_n7, eq170_e2148_d_n8, eq170_e2148_d_n9, eq170_e2148_d_n12, eq170_e2148_d_n14, eq170_e2148_d_n15, eq170_e2148_d_n16, eq170_e2148_d_n17, eq170_e2148_d_n18, eq170_e2148_d_n19, eq170_e2148_d_n20, eq170_e2148_d_n21, eq170_e2148_d_n22, eq170_e2148_q,) = {
    if (((var_guard556 != 0.0) && (var_guard557 != 0.0)) && (var_guard558 != 0.0)) {
        let eq170_e2143_q: f64 = var_qg_fp3;
        let eq170_e2144: f64 = (p.p7 * var_qg_fp3);
        let eq170_e2144_d_n0: f64 = (p.p7 * var_qg_fp3_dn0);
        let eq170_e2144_d_n1: f64 = (p.p7 * var_qg_fp3_dn1);
        let eq170_e2144_d_n2: f64 = (p.p7 * var_qg_fp3_dn2);
        let eq170_e2144_d_n3: f64 = (p.p7 * var_qg_fp3_dn3);
        let eq170_e2144_d_n4: f64 = (p.p7 * var_qg_fp3_dn4);
        let eq170_e2144_d_n5: f64 = (p.p7 * var_qg_fp3_dn5);
        let eq170_e2144_d_n6: f64 = (p.p7 * var_qg_fp3_dn6);
        let eq170_e2144_d_n7: f64 = (p.p7 * var_qg_fp3_dn7);
        let eq170_e2144_d_n8: f64 = (p.p7 * var_qg_fp3_dn8);
        let eq170_e2144_d_n9: f64 = (p.p7 * var_qg_fp3_dn9);
        let eq170_e2144_d_n12: f64 = (p.p7 * var_qg_fp3_dn12);
        let eq170_e2144_d_n14: f64 = (p.p7 * var_qg_fp3_dn14);
        let eq170_e2144_d_n15: f64 = (p.p7 * var_qg_fp3_dn15);
        let eq170_e2144_d_n16: f64 = (p.p7 * var_qg_fp3_dn16);
        let eq170_e2144_d_n17: f64 = (p.p7 * var_qg_fp3_dn17);
        let eq170_e2144_d_n18: f64 = (p.p7 * var_qg_fp3_dn18);
        let eq170_e2144_d_n19: f64 = (p.p7 * var_qg_fp3_dn19);
        let eq170_e2144_d_n20: f64 = (p.p7 * var_qg_fp3_dn20);
        let eq170_e2144_d_n21: f64 = (p.p7 * var_qg_fp3_dn21);
        let eq170_e2144_d_n22: f64 = (p.p7 * var_qg_fp3_dn22);
        let eq170_e2144_q: f64 = (p.p7 * eq170_e2143_q);
        let eq170_e2146: f64 = (eq170_e2144 * p.p248);
        let eq170_e2146_d_n0: f64 = (eq170_e2144_d_n0 * p.p248);
        let eq170_e2146_d_n1: f64 = (eq170_e2144_d_n1 * p.p248);
        let eq170_e2146_d_n2: f64 = (eq170_e2144_d_n2 * p.p248);
        let eq170_e2146_d_n3: f64 = (eq170_e2144_d_n3 * p.p248);
        let eq170_e2146_d_n4: f64 = (eq170_e2144_d_n4 * p.p248);
        let eq170_e2146_d_n5: f64 = (eq170_e2144_d_n5 * p.p248);
        let eq170_e2146_d_n6: f64 = (eq170_e2144_d_n6 * p.p248);
        let eq170_e2146_d_n7: f64 = (eq170_e2144_d_n7 * p.p248);
        let eq170_e2146_d_n8: f64 = (eq170_e2144_d_n8 * p.p248);
        let eq170_e2146_d_n9: f64 = (eq170_e2144_d_n9 * p.p248);
        let eq170_e2146_d_n12: f64 = (eq170_e2144_d_n12 * p.p248);
        let eq170_e2146_d_n14: f64 = (eq170_e2144_d_n14 * p.p248);
        let eq170_e2146_d_n15: f64 = (eq170_e2144_d_n15 * p.p248);
        let eq170_e2146_d_n16: f64 = (eq170_e2144_d_n16 * p.p248);
        let eq170_e2146_d_n17: f64 = (eq170_e2144_d_n17 * p.p248);
        let eq170_e2146_d_n18: f64 = (eq170_e2144_d_n18 * p.p248);
        let eq170_e2146_d_n19: f64 = (eq170_e2144_d_n19 * p.p248);
        let eq170_e2146_d_n20: f64 = (eq170_e2144_d_n20 * p.p248);
        let eq170_e2146_d_n21: f64 = (eq170_e2144_d_n21 * p.p248);
        let eq170_e2146_d_n22: f64 = (eq170_e2144_d_n22 * p.p248);
        let eq170_e2146_q: f64 = (eq170_e2144_q * p.p248);
        (eq170_e2146, eq170_e2146_d_n0, eq170_e2146_d_n1, eq170_e2146_d_n2, eq170_e2146_d_n3, eq170_e2146_d_n4, eq170_e2146_d_n5, eq170_e2146_d_n6, eq170_e2146_d_n7, eq170_e2146_d_n8, eq170_e2146_d_n9, eq170_e2146_d_n12, eq170_e2146_d_n14, eq170_e2146_d_n15, eq170_e2146_d_n16, eq170_e2146_d_n17, eq170_e2146_d_n18, eq170_e2146_d_n19, eq170_e2146_d_n20, eq170_e2146_d_n21, eq170_e2146_d_n22, eq170_e2146_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq170_reactive_node_derivatives: [f64; 23] = [eq170_e2148_d_n0, eq170_e2148_d_n1, eq170_e2148_d_n2, eq170_e2148_d_n3, eq170_e2148_d_n4, eq170_e2148_d_n5, eq170_e2148_d_n6, eq170_e2148_d_n7, eq170_e2148_d_n8, eq170_e2148_d_n9, 0.0, 0.0, eq170_e2148_d_n12, 0.0, eq170_e2148_d_n14, eq170_e2148_d_n15, eq170_e2148_d_n16, eq170_e2148_d_n17, eq170_e2148_d_n18, eq170_e2148_d_n19, eq170_e2148_d_n20, eq170_e2148_d_n21, eq170_e2148_d_n22];
        let eq170_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[16]),
            nodes,
            &eq170_reactive_node_derivatives,
            branches,
            &eq170_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq171_e2160, eq171_e2160_d_n0, eq171_e2160_d_n1, eq171_e2160_d_n2, eq171_e2160_d_n3, eq171_e2160_d_n4, eq171_e2160_d_n5, eq171_e2160_d_n6, eq171_e2160_d_n7, eq171_e2160_d_n8, eq171_e2160_d_n9, eq171_e2160_d_n12, eq171_e2160_d_n14, eq171_e2160_d_n15, eq171_e2160_d_n16, eq171_e2160_d_n17, eq171_e2160_d_n18, eq171_e2160_d_n19, eq171_e2160_d_n20, eq171_e2160_d_n21, eq171_e2160_d_n22, eq171_e2160_q,) = {
    if (((var_guard556 != 0.0) && (var_guard557 != 0.0)) && (var_guard558 == 0.0)) {
        let eq171_e2157_q: f64 = var_qg_fp3;
        let eq171_e2158: f64 = (p.p7 * var_qg_fp3);
        let eq171_e2158_d_n0: f64 = (p.p7 * var_qg_fp3_dn0);
        let eq171_e2158_d_n1: f64 = (p.p7 * var_qg_fp3_dn1);
        let eq171_e2158_d_n2: f64 = (p.p7 * var_qg_fp3_dn2);
        let eq171_e2158_d_n3: f64 = (p.p7 * var_qg_fp3_dn3);
        let eq171_e2158_d_n4: f64 = (p.p7 * var_qg_fp3_dn4);
        let eq171_e2158_d_n5: f64 = (p.p7 * var_qg_fp3_dn5);
        let eq171_e2158_d_n6: f64 = (p.p7 * var_qg_fp3_dn6);
        let eq171_e2158_d_n7: f64 = (p.p7 * var_qg_fp3_dn7);
        let eq171_e2158_d_n8: f64 = (p.p7 * var_qg_fp3_dn8);
        let eq171_e2158_d_n9: f64 = (p.p7 * var_qg_fp3_dn9);
        let eq171_e2158_d_n12: f64 = (p.p7 * var_qg_fp3_dn12);
        let eq171_e2158_d_n14: f64 = (p.p7 * var_qg_fp3_dn14);
        let eq171_e2158_d_n15: f64 = (p.p7 * var_qg_fp3_dn15);
        let eq171_e2158_d_n16: f64 = (p.p7 * var_qg_fp3_dn16);
        let eq171_e2158_d_n17: f64 = (p.p7 * var_qg_fp3_dn17);
        let eq171_e2158_d_n18: f64 = (p.p7 * var_qg_fp3_dn18);
        let eq171_e2158_d_n19: f64 = (p.p7 * var_qg_fp3_dn19);
        let eq171_e2158_d_n20: f64 = (p.p7 * var_qg_fp3_dn20);
        let eq171_e2158_d_n21: f64 = (p.p7 * var_qg_fp3_dn21);
        let eq171_e2158_d_n22: f64 = (p.p7 * var_qg_fp3_dn22);
        let eq171_e2158_q: f64 = (p.p7 * eq171_e2157_q);
        (eq171_e2158, eq171_e2158_d_n0, eq171_e2158_d_n1, eq171_e2158_d_n2, eq171_e2158_d_n3, eq171_e2158_d_n4, eq171_e2158_d_n5, eq171_e2158_d_n6, eq171_e2158_d_n7, eq171_e2158_d_n8, eq171_e2158_d_n9, eq171_e2158_d_n12, eq171_e2158_d_n14, eq171_e2158_d_n15, eq171_e2158_d_n16, eq171_e2158_d_n17, eq171_e2158_d_n18, eq171_e2158_d_n19, eq171_e2158_d_n20, eq171_e2158_d_n21, eq171_e2158_d_n22, eq171_e2158_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq171_reactive_node_derivatives: [f64; 23] = [eq171_e2160_d_n0, eq171_e2160_d_n1, eq171_e2160_d_n2, eq171_e2160_d_n3, eq171_e2160_d_n4, eq171_e2160_d_n5, eq171_e2160_d_n6, eq171_e2160_d_n7, eq171_e2160_d_n8, eq171_e2160_d_n9, 0.0, 0.0, eq171_e2160_d_n12, 0.0, eq171_e2160_d_n14, eq171_e2160_d_n15, eq171_e2160_d_n16, eq171_e2160_d_n17, eq171_e2160_d_n18, eq171_e2160_d_n19, eq171_e2160_d_n20, eq171_e2160_d_n21, eq171_e2160_d_n22];
        let eq171_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[16]),
            nodes,
            &eq171_reactive_node_derivatives,
            branches,
            &eq171_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq172_e2174, eq172_e2174_d_n0, eq172_e2174_d_n1, eq172_e2174_d_n2, eq172_e2174_d_n3, eq172_e2174_d_n4, eq172_e2174_d_n5, eq172_e2174_d_n6, eq172_e2174_d_n7, eq172_e2174_d_n8, eq172_e2174_d_n9, eq172_e2174_d_n12, eq172_e2174_d_n14, eq172_e2174_d_n15, eq172_e2174_d_n16, eq172_e2174_d_n17, eq172_e2174_d_n18, eq172_e2174_d_n19, eq172_e2174_d_n20, eq172_e2174_d_n21, eq172_e2174_d_n22, eq172_e2174_q,) = {
    if (((var_guard556 != 0.0) && (var_guard557 != 0.0)) && (var_guard558 == 0.0)) {
        let eq172_e2169_q: f64 = var_qg_fp3;
        let eq172_e2170: f64 = (p.p7 * var_qg_fp3);
        let eq172_e2170_d_n0: f64 = (p.p7 * var_qg_fp3_dn0);
        let eq172_e2170_d_n1: f64 = (p.p7 * var_qg_fp3_dn1);
        let eq172_e2170_d_n2: f64 = (p.p7 * var_qg_fp3_dn2);
        let eq172_e2170_d_n3: f64 = (p.p7 * var_qg_fp3_dn3);
        let eq172_e2170_d_n4: f64 = (p.p7 * var_qg_fp3_dn4);
        let eq172_e2170_d_n5: f64 = (p.p7 * var_qg_fp3_dn5);
        let eq172_e2170_d_n6: f64 = (p.p7 * var_qg_fp3_dn6);
        let eq172_e2170_d_n7: f64 = (p.p7 * var_qg_fp3_dn7);
        let eq172_e2170_d_n8: f64 = (p.p7 * var_qg_fp3_dn8);
        let eq172_e2170_d_n9: f64 = (p.p7 * var_qg_fp3_dn9);
        let eq172_e2170_d_n12: f64 = (p.p7 * var_qg_fp3_dn12);
        let eq172_e2170_d_n14: f64 = (p.p7 * var_qg_fp3_dn14);
        let eq172_e2170_d_n15: f64 = (p.p7 * var_qg_fp3_dn15);
        let eq172_e2170_d_n16: f64 = (p.p7 * var_qg_fp3_dn16);
        let eq172_e2170_d_n17: f64 = (p.p7 * var_qg_fp3_dn17);
        let eq172_e2170_d_n18: f64 = (p.p7 * var_qg_fp3_dn18);
        let eq172_e2170_d_n19: f64 = (p.p7 * var_qg_fp3_dn19);
        let eq172_e2170_d_n20: f64 = (p.p7 * var_qg_fp3_dn20);
        let eq172_e2170_d_n21: f64 = (p.p7 * var_qg_fp3_dn21);
        let eq172_e2170_d_n22: f64 = (p.p7 * var_qg_fp3_dn22);
        let eq172_e2170_q: f64 = (p.p7 * eq172_e2169_q);
        let eq172_e2172: f64 = (eq172_e2170 * p.p248);
        let eq172_e2172_d_n0: f64 = (eq172_e2170_d_n0 * p.p248);
        let eq172_e2172_d_n1: f64 = (eq172_e2170_d_n1 * p.p248);
        let eq172_e2172_d_n2: f64 = (eq172_e2170_d_n2 * p.p248);
        let eq172_e2172_d_n3: f64 = (eq172_e2170_d_n3 * p.p248);
        let eq172_e2172_d_n4: f64 = (eq172_e2170_d_n4 * p.p248);
        let eq172_e2172_d_n5: f64 = (eq172_e2170_d_n5 * p.p248);
        let eq172_e2172_d_n6: f64 = (eq172_e2170_d_n6 * p.p248);
        let eq172_e2172_d_n7: f64 = (eq172_e2170_d_n7 * p.p248);
        let eq172_e2172_d_n8: f64 = (eq172_e2170_d_n8 * p.p248);
        let eq172_e2172_d_n9: f64 = (eq172_e2170_d_n9 * p.p248);
        let eq172_e2172_d_n12: f64 = (eq172_e2170_d_n12 * p.p248);
        let eq172_e2172_d_n14: f64 = (eq172_e2170_d_n14 * p.p248);
        let eq172_e2172_d_n15: f64 = (eq172_e2170_d_n15 * p.p248);
        let eq172_e2172_d_n16: f64 = (eq172_e2170_d_n16 * p.p248);
        let eq172_e2172_d_n17: f64 = (eq172_e2170_d_n17 * p.p248);
        let eq172_e2172_d_n18: f64 = (eq172_e2170_d_n18 * p.p248);
        let eq172_e2172_d_n19: f64 = (eq172_e2170_d_n19 * p.p248);
        let eq172_e2172_d_n20: f64 = (eq172_e2170_d_n20 * p.p248);
        let eq172_e2172_d_n21: f64 = (eq172_e2170_d_n21 * p.p248);
        let eq172_e2172_d_n22: f64 = (eq172_e2170_d_n22 * p.p248);
        let eq172_e2172_q: f64 = (eq172_e2170_q * p.p248);
        (eq172_e2172, eq172_e2172_d_n0, eq172_e2172_d_n1, eq172_e2172_d_n2, eq172_e2172_d_n3, eq172_e2172_d_n4, eq172_e2172_d_n5, eq172_e2172_d_n6, eq172_e2172_d_n7, eq172_e2172_d_n8, eq172_e2172_d_n9, eq172_e2172_d_n12, eq172_e2172_d_n14, eq172_e2172_d_n15, eq172_e2172_d_n16, eq172_e2172_d_n17, eq172_e2172_d_n18, eq172_e2172_d_n19, eq172_e2172_d_n20, eq172_e2172_d_n21, eq172_e2172_d_n22, eq172_e2172_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq172_reactive_node_derivatives: [f64; 23] = [eq172_e2174_d_n0, eq172_e2174_d_n1, eq172_e2174_d_n2, eq172_e2174_d_n3, eq172_e2174_d_n4, eq172_e2174_d_n5, eq172_e2174_d_n6, eq172_e2174_d_n7, eq172_e2174_d_n8, eq172_e2174_d_n9, 0.0, 0.0, eq172_e2174_d_n12, 0.0, eq172_e2174_d_n14, eq172_e2174_d_n15, eq172_e2174_d_n16, eq172_e2174_d_n17, eq172_e2174_d_n18, eq172_e2174_d_n19, eq172_e2174_d_n20, eq172_e2174_d_n21, eq172_e2174_d_n22];
        let eq172_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[16]),
            nodes,
            &eq172_reactive_node_derivatives,
            branches,
            &eq172_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq173_e2185, eq173_e2185_d_n0, eq173_e2185_d_n1, eq173_e2185_d_n2, eq173_e2185_d_n3, eq173_e2185_d_n4, eq173_e2185_d_n5, eq173_e2185_d_n6, eq173_e2185_d_n7, eq173_e2185_d_n8, eq173_e2185_d_n9, eq173_e2185_d_n12, eq173_e2185_d_n14, eq173_e2185_d_n15, eq173_e2185_d_n16, eq173_e2185_d_n17, eq173_e2185_d_n18, eq173_e2185_d_n19, eq173_e2185_d_n20, eq173_e2185_d_n21, eq173_e2185_d_n22, eq173_e2185_q,) = {
    if ((var_guard556 != 0.0) && (var_guard557 != 0.0)) {
        let eq173_e2181: f64 = (p.p253 * var_qg_fp3);
        let eq173_e2181_d_n0: f64 = (p.p253 * var_qg_fp3_dn0);
        let eq173_e2181_d_n1: f64 = (p.p253 * var_qg_fp3_dn1);
        let eq173_e2181_d_n2: f64 = (p.p253 * var_qg_fp3_dn2);
        let eq173_e2181_d_n3: f64 = (p.p253 * var_qg_fp3_dn3);
        let eq173_e2181_d_n4: f64 = (p.p253 * var_qg_fp3_dn4);
        let eq173_e2181_d_n5: f64 = (p.p253 * var_qg_fp3_dn5);
        let eq173_e2181_d_n6: f64 = (p.p253 * var_qg_fp3_dn6);
        let eq173_e2181_d_n7: f64 = (p.p253 * var_qg_fp3_dn7);
        let eq173_e2181_d_n8: f64 = (p.p253 * var_qg_fp3_dn8);
        let eq173_e2181_d_n9: f64 = (p.p253 * var_qg_fp3_dn9);
        let eq173_e2181_d_n12: f64 = (p.p253 * var_qg_fp3_dn12);
        let eq173_e2181_d_n14: f64 = (p.p253 * var_qg_fp3_dn14);
        let eq173_e2181_d_n15: f64 = (p.p253 * var_qg_fp3_dn15);
        let eq173_e2181_d_n16: f64 = (p.p253 * var_qg_fp3_dn16);
        let eq173_e2181_d_n17: f64 = (p.p253 * var_qg_fp3_dn17);
        let eq173_e2181_d_n18: f64 = (p.p253 * var_qg_fp3_dn18);
        let eq173_e2181_d_n19: f64 = (p.p253 * var_qg_fp3_dn19);
        let eq173_e2181_d_n20: f64 = (p.p253 * var_qg_fp3_dn20);
        let eq173_e2181_d_n21: f64 = (p.p253 * var_qg_fp3_dn21);
        let eq173_e2181_d_n22: f64 = (p.p253 * var_qg_fp3_dn22);
        let eq173_e2182_q: f64 = eq173_e2181;
        let eq173_e2183: f64 = (p.p7 * eq173_e2181);
        let eq173_e2183_d_n0: f64 = (p.p7 * eq173_e2181_d_n0);
        let eq173_e2183_d_n1: f64 = (p.p7 * eq173_e2181_d_n1);
        let eq173_e2183_d_n2: f64 = (p.p7 * eq173_e2181_d_n2);
        let eq173_e2183_d_n3: f64 = (p.p7 * eq173_e2181_d_n3);
        let eq173_e2183_d_n4: f64 = (p.p7 * eq173_e2181_d_n4);
        let eq173_e2183_d_n5: f64 = (p.p7 * eq173_e2181_d_n5);
        let eq173_e2183_d_n6: f64 = (p.p7 * eq173_e2181_d_n6);
        let eq173_e2183_d_n7: f64 = (p.p7 * eq173_e2181_d_n7);
        let eq173_e2183_d_n8: f64 = (p.p7 * eq173_e2181_d_n8);
        let eq173_e2183_d_n9: f64 = (p.p7 * eq173_e2181_d_n9);
        let eq173_e2183_d_n12: f64 = (p.p7 * eq173_e2181_d_n12);
        let eq173_e2183_d_n14: f64 = (p.p7 * eq173_e2181_d_n14);
        let eq173_e2183_d_n15: f64 = (p.p7 * eq173_e2181_d_n15);
        let eq173_e2183_d_n16: f64 = (p.p7 * eq173_e2181_d_n16);
        let eq173_e2183_d_n17: f64 = (p.p7 * eq173_e2181_d_n17);
        let eq173_e2183_d_n18: f64 = (p.p7 * eq173_e2181_d_n18);
        let eq173_e2183_d_n19: f64 = (p.p7 * eq173_e2181_d_n19);
        let eq173_e2183_d_n20: f64 = (p.p7 * eq173_e2181_d_n20);
        let eq173_e2183_d_n21: f64 = (p.p7 * eq173_e2181_d_n21);
        let eq173_e2183_d_n22: f64 = (p.p7 * eq173_e2181_d_n22);
        let eq173_e2183_q: f64 = (p.p7 * eq173_e2182_q);
        (eq173_e2183, eq173_e2183_d_n0, eq173_e2183_d_n1, eq173_e2183_d_n2, eq173_e2183_d_n3, eq173_e2183_d_n4, eq173_e2183_d_n5, eq173_e2183_d_n6, eq173_e2183_d_n7, eq173_e2183_d_n8, eq173_e2183_d_n9, eq173_e2183_d_n12, eq173_e2183_d_n14, eq173_e2183_d_n15, eq173_e2183_d_n16, eq173_e2183_d_n17, eq173_e2183_d_n18, eq173_e2183_d_n19, eq173_e2183_d_n20, eq173_e2183_d_n21, eq173_e2183_d_n22, eq173_e2183_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq173_reactive_node_derivatives: [f64; 23] = [eq173_e2185_d_n0, eq173_e2185_d_n1, eq173_e2185_d_n2, eq173_e2185_d_n3, eq173_e2185_d_n4, eq173_e2185_d_n5, eq173_e2185_d_n6, eq173_e2185_d_n7, eq173_e2185_d_n8, eq173_e2185_d_n9, 0.0, 0.0, eq173_e2185_d_n12, 0.0, eq173_e2185_d_n14, eq173_e2185_d_n15, eq173_e2185_d_n16, eq173_e2185_d_n17, eq173_e2185_d_n18, eq173_e2185_d_n19, eq173_e2185_d_n20, eq173_e2185_d_n21, eq173_e2185_d_n22];
        let eq173_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[16]),
            nodes,
            &eq173_reactive_node_derivatives,
            branches,
            &eq173_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq174_e2195, eq174_e2195_d_n0, eq174_e2195_d_n1, eq174_e2195_d_n2, eq174_e2195_d_n3, eq174_e2195_d_n4, eq174_e2195_d_n5, eq174_e2195_d_n6, eq174_e2195_d_n7, eq174_e2195_d_n8, eq174_e2195_d_n9, eq174_e2195_d_n12, eq174_e2195_d_n14, eq174_e2195_d_n15, eq174_e2195_d_n16, eq174_e2195_d_n17, eq174_e2195_d_n18, eq174_e2195_d_n19, eq174_e2195_d_n20, eq174_e2195_d_n21, eq174_e2195_d_n22, eq174_e2195_q,) = {
    if ((var_guard556 == 0.0) && (var_guard559 != 0.0)) {
        let eq174_e2192_q: f64 = var_qd_fp3;
        let eq174_e2193: f64 = (p.p7 * var_qd_fp3);
        let eq174_e2193_d_n0: f64 = (p.p7 * var_qd_fp3_dn0);
        let eq174_e2193_d_n1: f64 = (p.p7 * var_qd_fp3_dn1);
        let eq174_e2193_d_n2: f64 = (p.p7 * var_qd_fp3_dn2);
        let eq174_e2193_d_n3: f64 = (p.p7 * var_qd_fp3_dn3);
        let eq174_e2193_d_n4: f64 = (p.p7 * var_qd_fp3_dn4);
        let eq174_e2193_d_n5: f64 = (p.p7 * var_qd_fp3_dn5);
        let eq174_e2193_d_n6: f64 = (p.p7 * var_qd_fp3_dn6);
        let eq174_e2193_d_n7: f64 = (p.p7 * var_qd_fp3_dn7);
        let eq174_e2193_d_n8: f64 = (p.p7 * var_qd_fp3_dn8);
        let eq174_e2193_d_n9: f64 = (p.p7 * var_qd_fp3_dn9);
        let eq174_e2193_d_n12: f64 = (p.p7 * var_qd_fp3_dn12);
        let eq174_e2193_d_n14: f64 = (p.p7 * var_qd_fp3_dn14);
        let eq174_e2193_d_n15: f64 = (p.p7 * var_qd_fp3_dn15);
        let eq174_e2193_d_n16: f64 = (p.p7 * var_qd_fp3_dn16);
        let eq174_e2193_d_n17: f64 = (p.p7 * var_qd_fp3_dn17);
        let eq174_e2193_d_n18: f64 = (p.p7 * var_qd_fp3_dn18);
        let eq174_e2193_d_n19: f64 = (p.p7 * var_qd_fp3_dn19);
        let eq174_e2193_d_n20: f64 = (p.p7 * var_qd_fp3_dn20);
        let eq174_e2193_d_n21: f64 = (p.p7 * var_qd_fp3_dn21);
        let eq174_e2193_d_n22: f64 = (p.p7 * var_qd_fp3_dn22);
        let eq174_e2193_q: f64 = (p.p7 * eq174_e2192_q);
        (eq174_e2193, eq174_e2193_d_n0, eq174_e2193_d_n1, eq174_e2193_d_n2, eq174_e2193_d_n3, eq174_e2193_d_n4, eq174_e2193_d_n5, eq174_e2193_d_n6, eq174_e2193_d_n7, eq174_e2193_d_n8, eq174_e2193_d_n9, eq174_e2193_d_n12, eq174_e2193_d_n14, eq174_e2193_d_n15, eq174_e2193_d_n16, eq174_e2193_d_n17, eq174_e2193_d_n18, eq174_e2193_d_n19, eq174_e2193_d_n20, eq174_e2193_d_n21, eq174_e2193_d_n22, eq174_e2193_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq174_reactive_node_derivatives: [f64; 23] = [eq174_e2195_d_n0, eq174_e2195_d_n1, eq174_e2195_d_n2, eq174_e2195_d_n3, eq174_e2195_d_n4, eq174_e2195_d_n5, eq174_e2195_d_n6, eq174_e2195_d_n7, eq174_e2195_d_n8, eq174_e2195_d_n9, 0.0, 0.0, eq174_e2195_d_n12, 0.0, eq174_e2195_d_n14, eq174_e2195_d_n15, eq174_e2195_d_n16, eq174_e2195_d_n17, eq174_e2195_d_n18, eq174_e2195_d_n19, eq174_e2195_d_n20, eq174_e2195_d_n21, eq174_e2195_d_n22];
        let eq174_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[7]),
            nodes,
            &eq174_reactive_node_derivatives,
            branches,
            &eq174_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq175_e2207, eq175_e2207_d_n0, eq175_e2207_d_n1, eq175_e2207_d_n2, eq175_e2207_d_n3, eq175_e2207_d_n4, eq175_e2207_d_n5, eq175_e2207_d_n6, eq175_e2207_d_n7, eq175_e2207_d_n8, eq175_e2207_d_n9, eq175_e2207_d_n12, eq175_e2207_d_n14, eq175_e2207_d_n15, eq175_e2207_d_n16, eq175_e2207_d_n17, eq175_e2207_d_n18, eq175_e2207_d_n19, eq175_e2207_d_n20, eq175_e2207_d_n21, eq175_e2207_d_n22, eq175_e2207_q,) = {
    if (((var_guard556 == 0.0) && (var_guard559 != 0.0)) && (var_guard560 != 0.0)) {
        let eq175_e2204_q: f64 = var_qg_fp3;
        let eq175_e2205: f64 = (p.p7 * var_qg_fp3);
        let eq175_e2205_d_n0: f64 = (p.p7 * var_qg_fp3_dn0);
        let eq175_e2205_d_n1: f64 = (p.p7 * var_qg_fp3_dn1);
        let eq175_e2205_d_n2: f64 = (p.p7 * var_qg_fp3_dn2);
        let eq175_e2205_d_n3: f64 = (p.p7 * var_qg_fp3_dn3);
        let eq175_e2205_d_n4: f64 = (p.p7 * var_qg_fp3_dn4);
        let eq175_e2205_d_n5: f64 = (p.p7 * var_qg_fp3_dn5);
        let eq175_e2205_d_n6: f64 = (p.p7 * var_qg_fp3_dn6);
        let eq175_e2205_d_n7: f64 = (p.p7 * var_qg_fp3_dn7);
        let eq175_e2205_d_n8: f64 = (p.p7 * var_qg_fp3_dn8);
        let eq175_e2205_d_n9: f64 = (p.p7 * var_qg_fp3_dn9);
        let eq175_e2205_d_n12: f64 = (p.p7 * var_qg_fp3_dn12);
        let eq175_e2205_d_n14: f64 = (p.p7 * var_qg_fp3_dn14);
        let eq175_e2205_d_n15: f64 = (p.p7 * var_qg_fp3_dn15);
        let eq175_e2205_d_n16: f64 = (p.p7 * var_qg_fp3_dn16);
        let eq175_e2205_d_n17: f64 = (p.p7 * var_qg_fp3_dn17);
        let eq175_e2205_d_n18: f64 = (p.p7 * var_qg_fp3_dn18);
        let eq175_e2205_d_n19: f64 = (p.p7 * var_qg_fp3_dn19);
        let eq175_e2205_d_n20: f64 = (p.p7 * var_qg_fp3_dn20);
        let eq175_e2205_d_n21: f64 = (p.p7 * var_qg_fp3_dn21);
        let eq175_e2205_d_n22: f64 = (p.p7 * var_qg_fp3_dn22);
        let eq175_e2205_q: f64 = (p.p7 * eq175_e2204_q);
        (eq175_e2205, eq175_e2205_d_n0, eq175_e2205_d_n1, eq175_e2205_d_n2, eq175_e2205_d_n3, eq175_e2205_d_n4, eq175_e2205_d_n5, eq175_e2205_d_n6, eq175_e2205_d_n7, eq175_e2205_d_n8, eq175_e2205_d_n9, eq175_e2205_d_n12, eq175_e2205_d_n14, eq175_e2205_d_n15, eq175_e2205_d_n16, eq175_e2205_d_n17, eq175_e2205_d_n18, eq175_e2205_d_n19, eq175_e2205_d_n20, eq175_e2205_d_n21, eq175_e2205_d_n22, eq175_e2205_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq175_reactive_node_derivatives: [f64; 23] = [eq175_e2207_d_n0, eq175_e2207_d_n1, eq175_e2207_d_n2, eq175_e2207_d_n3, eq175_e2207_d_n4, eq175_e2207_d_n5, eq175_e2207_d_n6, eq175_e2207_d_n7, eq175_e2207_d_n8, eq175_e2207_d_n9, 0.0, 0.0, eq175_e2207_d_n12, 0.0, eq175_e2207_d_n14, eq175_e2207_d_n15, eq175_e2207_d_n16, eq175_e2207_d_n17, eq175_e2207_d_n18, eq175_e2207_d_n19, eq175_e2207_d_n20, eq175_e2207_d_n21, eq175_e2207_d_n22];
        let eq175_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq175_reactive_node_derivatives,
            branches,
            &eq175_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq176_e2221, eq176_e2221_d_n0, eq176_e2221_d_n1, eq176_e2221_d_n2, eq176_e2221_d_n3, eq176_e2221_d_n4, eq176_e2221_d_n5, eq176_e2221_d_n6, eq176_e2221_d_n7, eq176_e2221_d_n8, eq176_e2221_d_n9, eq176_e2221_d_n12, eq176_e2221_d_n14, eq176_e2221_d_n15, eq176_e2221_d_n16, eq176_e2221_d_n17, eq176_e2221_d_n18, eq176_e2221_d_n19, eq176_e2221_d_n20, eq176_e2221_d_n21, eq176_e2221_d_n22, eq176_e2221_q,) = {
    if (((var_guard556 == 0.0) && (var_guard559 != 0.0)) && (var_guard560 != 0.0)) {
        let eq176_e2216_q: f64 = var_qg_fp3;
        let eq176_e2217: f64 = (p.p7 * var_qg_fp3);
        let eq176_e2217_d_n0: f64 = (p.p7 * var_qg_fp3_dn0);
        let eq176_e2217_d_n1: f64 = (p.p7 * var_qg_fp3_dn1);
        let eq176_e2217_d_n2: f64 = (p.p7 * var_qg_fp3_dn2);
        let eq176_e2217_d_n3: f64 = (p.p7 * var_qg_fp3_dn3);
        let eq176_e2217_d_n4: f64 = (p.p7 * var_qg_fp3_dn4);
        let eq176_e2217_d_n5: f64 = (p.p7 * var_qg_fp3_dn5);
        let eq176_e2217_d_n6: f64 = (p.p7 * var_qg_fp3_dn6);
        let eq176_e2217_d_n7: f64 = (p.p7 * var_qg_fp3_dn7);
        let eq176_e2217_d_n8: f64 = (p.p7 * var_qg_fp3_dn8);
        let eq176_e2217_d_n9: f64 = (p.p7 * var_qg_fp3_dn9);
        let eq176_e2217_d_n12: f64 = (p.p7 * var_qg_fp3_dn12);
        let eq176_e2217_d_n14: f64 = (p.p7 * var_qg_fp3_dn14);
        let eq176_e2217_d_n15: f64 = (p.p7 * var_qg_fp3_dn15);
        let eq176_e2217_d_n16: f64 = (p.p7 * var_qg_fp3_dn16);
        let eq176_e2217_d_n17: f64 = (p.p7 * var_qg_fp3_dn17);
        let eq176_e2217_d_n18: f64 = (p.p7 * var_qg_fp3_dn18);
        let eq176_e2217_d_n19: f64 = (p.p7 * var_qg_fp3_dn19);
        let eq176_e2217_d_n20: f64 = (p.p7 * var_qg_fp3_dn20);
        let eq176_e2217_d_n21: f64 = (p.p7 * var_qg_fp3_dn21);
        let eq176_e2217_d_n22: f64 = (p.p7 * var_qg_fp3_dn22);
        let eq176_e2217_q: f64 = (p.p7 * eq176_e2216_q);
        let eq176_e2219: f64 = (eq176_e2217 * p.p248);
        let eq176_e2219_d_n0: f64 = (eq176_e2217_d_n0 * p.p248);
        let eq176_e2219_d_n1: f64 = (eq176_e2217_d_n1 * p.p248);
        let eq176_e2219_d_n2: f64 = (eq176_e2217_d_n2 * p.p248);
        let eq176_e2219_d_n3: f64 = (eq176_e2217_d_n3 * p.p248);
        let eq176_e2219_d_n4: f64 = (eq176_e2217_d_n4 * p.p248);
        let eq176_e2219_d_n5: f64 = (eq176_e2217_d_n5 * p.p248);
        let eq176_e2219_d_n6: f64 = (eq176_e2217_d_n6 * p.p248);
        let eq176_e2219_d_n7: f64 = (eq176_e2217_d_n7 * p.p248);
        let eq176_e2219_d_n8: f64 = (eq176_e2217_d_n8 * p.p248);
        let eq176_e2219_d_n9: f64 = (eq176_e2217_d_n9 * p.p248);
        let eq176_e2219_d_n12: f64 = (eq176_e2217_d_n12 * p.p248);
        let eq176_e2219_d_n14: f64 = (eq176_e2217_d_n14 * p.p248);
        let eq176_e2219_d_n15: f64 = (eq176_e2217_d_n15 * p.p248);
        let eq176_e2219_d_n16: f64 = (eq176_e2217_d_n16 * p.p248);
        let eq176_e2219_d_n17: f64 = (eq176_e2217_d_n17 * p.p248);
        let eq176_e2219_d_n18: f64 = (eq176_e2217_d_n18 * p.p248);
        let eq176_e2219_d_n19: f64 = (eq176_e2217_d_n19 * p.p248);
        let eq176_e2219_d_n20: f64 = (eq176_e2217_d_n20 * p.p248);
        let eq176_e2219_d_n21: f64 = (eq176_e2217_d_n21 * p.p248);
        let eq176_e2219_d_n22: f64 = (eq176_e2217_d_n22 * p.p248);
        let eq176_e2219_q: f64 = (eq176_e2217_q * p.p248);
        (eq176_e2219, eq176_e2219_d_n0, eq176_e2219_d_n1, eq176_e2219_d_n2, eq176_e2219_d_n3, eq176_e2219_d_n4, eq176_e2219_d_n5, eq176_e2219_d_n6, eq176_e2219_d_n7, eq176_e2219_d_n8, eq176_e2219_d_n9, eq176_e2219_d_n12, eq176_e2219_d_n14, eq176_e2219_d_n15, eq176_e2219_d_n16, eq176_e2219_d_n17, eq176_e2219_d_n18, eq176_e2219_d_n19, eq176_e2219_d_n20, eq176_e2219_d_n21, eq176_e2219_d_n22, eq176_e2219_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq176_reactive_node_derivatives: [f64; 23] = [eq176_e2221_d_n0, eq176_e2221_d_n1, eq176_e2221_d_n2, eq176_e2221_d_n3, eq176_e2221_d_n4, eq176_e2221_d_n5, eq176_e2221_d_n6, eq176_e2221_d_n7, eq176_e2221_d_n8, eq176_e2221_d_n9, 0.0, 0.0, eq176_e2221_d_n12, 0.0, eq176_e2221_d_n14, eq176_e2221_d_n15, eq176_e2221_d_n16, eq176_e2221_d_n17, eq176_e2221_d_n18, eq176_e2221_d_n19, eq176_e2221_d_n20, eq176_e2221_d_n21, eq176_e2221_d_n22];
        let eq176_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq176_reactive_node_derivatives,
            branches,
            &eq176_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq177_e2234, eq177_e2234_d_n0, eq177_e2234_d_n1, eq177_e2234_d_n2, eq177_e2234_d_n3, eq177_e2234_d_n4, eq177_e2234_d_n5, eq177_e2234_d_n6, eq177_e2234_d_n7, eq177_e2234_d_n8, eq177_e2234_d_n9, eq177_e2234_d_n12, eq177_e2234_d_n14, eq177_e2234_d_n15, eq177_e2234_d_n16, eq177_e2234_d_n17, eq177_e2234_d_n18, eq177_e2234_d_n19, eq177_e2234_d_n20, eq177_e2234_d_n21, eq177_e2234_d_n22, eq177_e2234_q,) = {
    if (((var_guard556 == 0.0) && (var_guard559 != 0.0)) && (var_guard560 == 0.0)) {
        let eq177_e2231_q: f64 = var_qg_fp3;
        let eq177_e2232: f64 = (p.p7 * var_qg_fp3);
        let eq177_e2232_d_n0: f64 = (p.p7 * var_qg_fp3_dn0);
        let eq177_e2232_d_n1: f64 = (p.p7 * var_qg_fp3_dn1);
        let eq177_e2232_d_n2: f64 = (p.p7 * var_qg_fp3_dn2);
        let eq177_e2232_d_n3: f64 = (p.p7 * var_qg_fp3_dn3);
        let eq177_e2232_d_n4: f64 = (p.p7 * var_qg_fp3_dn4);
        let eq177_e2232_d_n5: f64 = (p.p7 * var_qg_fp3_dn5);
        let eq177_e2232_d_n6: f64 = (p.p7 * var_qg_fp3_dn6);
        let eq177_e2232_d_n7: f64 = (p.p7 * var_qg_fp3_dn7);
        let eq177_e2232_d_n8: f64 = (p.p7 * var_qg_fp3_dn8);
        let eq177_e2232_d_n9: f64 = (p.p7 * var_qg_fp3_dn9);
        let eq177_e2232_d_n12: f64 = (p.p7 * var_qg_fp3_dn12);
        let eq177_e2232_d_n14: f64 = (p.p7 * var_qg_fp3_dn14);
        let eq177_e2232_d_n15: f64 = (p.p7 * var_qg_fp3_dn15);
        let eq177_e2232_d_n16: f64 = (p.p7 * var_qg_fp3_dn16);
        let eq177_e2232_d_n17: f64 = (p.p7 * var_qg_fp3_dn17);
        let eq177_e2232_d_n18: f64 = (p.p7 * var_qg_fp3_dn18);
        let eq177_e2232_d_n19: f64 = (p.p7 * var_qg_fp3_dn19);
        let eq177_e2232_d_n20: f64 = (p.p7 * var_qg_fp3_dn20);
        let eq177_e2232_d_n21: f64 = (p.p7 * var_qg_fp3_dn21);
        let eq177_e2232_d_n22: f64 = (p.p7 * var_qg_fp3_dn22);
        let eq177_e2232_q: f64 = (p.p7 * eq177_e2231_q);
        (eq177_e2232, eq177_e2232_d_n0, eq177_e2232_d_n1, eq177_e2232_d_n2, eq177_e2232_d_n3, eq177_e2232_d_n4, eq177_e2232_d_n5, eq177_e2232_d_n6, eq177_e2232_d_n7, eq177_e2232_d_n8, eq177_e2232_d_n9, eq177_e2232_d_n12, eq177_e2232_d_n14, eq177_e2232_d_n15, eq177_e2232_d_n16, eq177_e2232_d_n17, eq177_e2232_d_n18, eq177_e2232_d_n19, eq177_e2232_d_n20, eq177_e2232_d_n21, eq177_e2232_d_n22, eq177_e2232_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq177_reactive_node_derivatives: [f64; 23] = [eq177_e2234_d_n0, eq177_e2234_d_n1, eq177_e2234_d_n2, eq177_e2234_d_n3, eq177_e2234_d_n4, eq177_e2234_d_n5, eq177_e2234_d_n6, eq177_e2234_d_n7, eq177_e2234_d_n8, eq177_e2234_d_n9, 0.0, 0.0, eq177_e2234_d_n12, 0.0, eq177_e2234_d_n14, eq177_e2234_d_n15, eq177_e2234_d_n16, eq177_e2234_d_n17, eq177_e2234_d_n18, eq177_e2234_d_n19, eq177_e2234_d_n20, eq177_e2234_d_n21, eq177_e2234_d_n22];
        let eq177_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq177_reactive_node_derivatives,
            branches,
            &eq177_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_7(
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_guard556: f64,
        var_guard559: f64,
        var_guard560: f64,
        var_guard561: f64,
        var_guard562: f64,
        var_guard563: f64,
        var_guard564: f64,
        var_qd_fp3s: f64,
        var_qd_fp3s_dn0: f64,
        var_qd_fp3s_dn1: f64,
        var_qd_fp3s_dn12: f64,
        var_qd_fp3s_dn14: f64,
        var_qd_fp3s_dn15: f64,
        var_qd_fp3s_dn16: f64,
        var_qd_fp3s_dn17: f64,
        var_qd_fp3s_dn18: f64,
        var_qd_fp3s_dn19: f64,
        var_qd_fp3s_dn2: f64,
        var_qd_fp3s_dn20: f64,
        var_qd_fp3s_dn21: f64,
        var_qd_fp3s_dn22: f64,
        var_qd_fp3s_dn3: f64,
        var_qd_fp3s_dn4: f64,
        var_qd_fp3s_dn5: f64,
        var_qd_fp3s_dn6: f64,
        var_qd_fp3s_dn7: f64,
        var_qd_fp3s_dn8: f64,
        var_qd_fp3s_dn9: f64,
        var_qg_fp3: f64,
        var_qg_fp3_dn0: f64,
        var_qg_fp3_dn1: f64,
        var_qg_fp3_dn12: f64,
        var_qg_fp3_dn14: f64,
        var_qg_fp3_dn15: f64,
        var_qg_fp3_dn16: f64,
        var_qg_fp3_dn17: f64,
        var_qg_fp3_dn18: f64,
        var_qg_fp3_dn19: f64,
        var_qg_fp3_dn2: f64,
        var_qg_fp3_dn20: f64,
        var_qg_fp3_dn21: f64,
        var_qg_fp3_dn22: f64,
        var_qg_fp3_dn3: f64,
        var_qg_fp3_dn4: f64,
        var_qg_fp3_dn5: f64,
        var_qg_fp3_dn6: f64,
        var_qg_fp3_dn7: f64,
        var_qg_fp3_dn8: f64,
        var_qg_fp3_dn9: f64,
        var_qg_fp3s: f64,
        var_qg_fp3s_dn0: f64,
        var_qg_fp3s_dn1: f64,
        var_qg_fp3s_dn12: f64,
        var_qg_fp3s_dn14: f64,
        var_qg_fp3s_dn15: f64,
        var_qg_fp3s_dn16: f64,
        var_qg_fp3s_dn17: f64,
        var_qg_fp3s_dn18: f64,
        var_qg_fp3s_dn19: f64,
        var_qg_fp3s_dn2: f64,
        var_qg_fp3s_dn20: f64,
        var_qg_fp3s_dn21: f64,
        var_qg_fp3s_dn22: f64,
        var_qg_fp3s_dn3: f64,
        var_qg_fp3s_dn4: f64,
        var_qg_fp3s_dn5: f64,
        var_qg_fp3s_dn6: f64,
        var_qg_fp3s_dn7: f64,
        var_qg_fp3s_dn8: f64,
        var_qg_fp3s_dn9: f64,
    ) {
        let (eq178_e2249, eq178_e2249_d_n0, eq178_e2249_d_n1, eq178_e2249_d_n2, eq178_e2249_d_n3, eq178_e2249_d_n4, eq178_e2249_d_n5, eq178_e2249_d_n6, eq178_e2249_d_n7, eq178_e2249_d_n8, eq178_e2249_d_n9, eq178_e2249_d_n12, eq178_e2249_d_n14, eq178_e2249_d_n15, eq178_e2249_d_n16, eq178_e2249_d_n17, eq178_e2249_d_n18, eq178_e2249_d_n19, eq178_e2249_d_n20, eq178_e2249_d_n21, eq178_e2249_d_n22, eq178_e2249_q,) = {
    if (((var_guard556 == 0.0) && (var_guard559 != 0.0)) && (var_guard560 == 0.0)) {
        let eq178_e2244_q: f64 = var_qg_fp3;
        let eq178_e2245: f64 = (p.p7 * var_qg_fp3);
        let eq178_e2245_d_n0: f64 = (p.p7 * var_qg_fp3_dn0);
        let eq178_e2245_d_n1: f64 = (p.p7 * var_qg_fp3_dn1);
        let eq178_e2245_d_n2: f64 = (p.p7 * var_qg_fp3_dn2);
        let eq178_e2245_d_n3: f64 = (p.p7 * var_qg_fp3_dn3);
        let eq178_e2245_d_n4: f64 = (p.p7 * var_qg_fp3_dn4);
        let eq178_e2245_d_n5: f64 = (p.p7 * var_qg_fp3_dn5);
        let eq178_e2245_d_n6: f64 = (p.p7 * var_qg_fp3_dn6);
        let eq178_e2245_d_n7: f64 = (p.p7 * var_qg_fp3_dn7);
        let eq178_e2245_d_n8: f64 = (p.p7 * var_qg_fp3_dn8);
        let eq178_e2245_d_n9: f64 = (p.p7 * var_qg_fp3_dn9);
        let eq178_e2245_d_n12: f64 = (p.p7 * var_qg_fp3_dn12);
        let eq178_e2245_d_n14: f64 = (p.p7 * var_qg_fp3_dn14);
        let eq178_e2245_d_n15: f64 = (p.p7 * var_qg_fp3_dn15);
        let eq178_e2245_d_n16: f64 = (p.p7 * var_qg_fp3_dn16);
        let eq178_e2245_d_n17: f64 = (p.p7 * var_qg_fp3_dn17);
        let eq178_e2245_d_n18: f64 = (p.p7 * var_qg_fp3_dn18);
        let eq178_e2245_d_n19: f64 = (p.p7 * var_qg_fp3_dn19);
        let eq178_e2245_d_n20: f64 = (p.p7 * var_qg_fp3_dn20);
        let eq178_e2245_d_n21: f64 = (p.p7 * var_qg_fp3_dn21);
        let eq178_e2245_d_n22: f64 = (p.p7 * var_qg_fp3_dn22);
        let eq178_e2245_q: f64 = (p.p7 * eq178_e2244_q);
        let eq178_e2247: f64 = (eq178_e2245 * p.p248);
        let eq178_e2247_d_n0: f64 = (eq178_e2245_d_n0 * p.p248);
        let eq178_e2247_d_n1: f64 = (eq178_e2245_d_n1 * p.p248);
        let eq178_e2247_d_n2: f64 = (eq178_e2245_d_n2 * p.p248);
        let eq178_e2247_d_n3: f64 = (eq178_e2245_d_n3 * p.p248);
        let eq178_e2247_d_n4: f64 = (eq178_e2245_d_n4 * p.p248);
        let eq178_e2247_d_n5: f64 = (eq178_e2245_d_n5 * p.p248);
        let eq178_e2247_d_n6: f64 = (eq178_e2245_d_n6 * p.p248);
        let eq178_e2247_d_n7: f64 = (eq178_e2245_d_n7 * p.p248);
        let eq178_e2247_d_n8: f64 = (eq178_e2245_d_n8 * p.p248);
        let eq178_e2247_d_n9: f64 = (eq178_e2245_d_n9 * p.p248);
        let eq178_e2247_d_n12: f64 = (eq178_e2245_d_n12 * p.p248);
        let eq178_e2247_d_n14: f64 = (eq178_e2245_d_n14 * p.p248);
        let eq178_e2247_d_n15: f64 = (eq178_e2245_d_n15 * p.p248);
        let eq178_e2247_d_n16: f64 = (eq178_e2245_d_n16 * p.p248);
        let eq178_e2247_d_n17: f64 = (eq178_e2245_d_n17 * p.p248);
        let eq178_e2247_d_n18: f64 = (eq178_e2245_d_n18 * p.p248);
        let eq178_e2247_d_n19: f64 = (eq178_e2245_d_n19 * p.p248);
        let eq178_e2247_d_n20: f64 = (eq178_e2245_d_n20 * p.p248);
        let eq178_e2247_d_n21: f64 = (eq178_e2245_d_n21 * p.p248);
        let eq178_e2247_d_n22: f64 = (eq178_e2245_d_n22 * p.p248);
        let eq178_e2247_q: f64 = (eq178_e2245_q * p.p248);
        (eq178_e2247, eq178_e2247_d_n0, eq178_e2247_d_n1, eq178_e2247_d_n2, eq178_e2247_d_n3, eq178_e2247_d_n4, eq178_e2247_d_n5, eq178_e2247_d_n6, eq178_e2247_d_n7, eq178_e2247_d_n8, eq178_e2247_d_n9, eq178_e2247_d_n12, eq178_e2247_d_n14, eq178_e2247_d_n15, eq178_e2247_d_n16, eq178_e2247_d_n17, eq178_e2247_d_n18, eq178_e2247_d_n19, eq178_e2247_d_n20, eq178_e2247_d_n21, eq178_e2247_d_n22, eq178_e2247_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq178_reactive_node_derivatives: [f64; 23] = [eq178_e2249_d_n0, eq178_e2249_d_n1, eq178_e2249_d_n2, eq178_e2249_d_n3, eq178_e2249_d_n4, eq178_e2249_d_n5, eq178_e2249_d_n6, eq178_e2249_d_n7, eq178_e2249_d_n8, eq178_e2249_d_n9, 0.0, 0.0, eq178_e2249_d_n12, 0.0, eq178_e2249_d_n14, eq178_e2249_d_n15, eq178_e2249_d_n16, eq178_e2249_d_n17, eq178_e2249_d_n18, eq178_e2249_d_n19, eq178_e2249_d_n20, eq178_e2249_d_n21, eq178_e2249_d_n22];
        let eq178_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq178_reactive_node_derivatives,
            branches,
            &eq178_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq179_e2261, eq179_e2261_d_n0, eq179_e2261_d_n1, eq179_e2261_d_n2, eq179_e2261_d_n3, eq179_e2261_d_n4, eq179_e2261_d_n5, eq179_e2261_d_n6, eq179_e2261_d_n7, eq179_e2261_d_n8, eq179_e2261_d_n9, eq179_e2261_d_n12, eq179_e2261_d_n14, eq179_e2261_d_n15, eq179_e2261_d_n16, eq179_e2261_d_n17, eq179_e2261_d_n18, eq179_e2261_d_n19, eq179_e2261_d_n20, eq179_e2261_d_n21, eq179_e2261_d_n22, eq179_e2261_q,) = {
    if ((var_guard556 == 0.0) && (var_guard559 != 0.0)) {
        let eq179_e2257: f64 = (p.p253 * var_qg_fp3);
        let eq179_e2257_d_n0: f64 = (p.p253 * var_qg_fp3_dn0);
        let eq179_e2257_d_n1: f64 = (p.p253 * var_qg_fp3_dn1);
        let eq179_e2257_d_n2: f64 = (p.p253 * var_qg_fp3_dn2);
        let eq179_e2257_d_n3: f64 = (p.p253 * var_qg_fp3_dn3);
        let eq179_e2257_d_n4: f64 = (p.p253 * var_qg_fp3_dn4);
        let eq179_e2257_d_n5: f64 = (p.p253 * var_qg_fp3_dn5);
        let eq179_e2257_d_n6: f64 = (p.p253 * var_qg_fp3_dn6);
        let eq179_e2257_d_n7: f64 = (p.p253 * var_qg_fp3_dn7);
        let eq179_e2257_d_n8: f64 = (p.p253 * var_qg_fp3_dn8);
        let eq179_e2257_d_n9: f64 = (p.p253 * var_qg_fp3_dn9);
        let eq179_e2257_d_n12: f64 = (p.p253 * var_qg_fp3_dn12);
        let eq179_e2257_d_n14: f64 = (p.p253 * var_qg_fp3_dn14);
        let eq179_e2257_d_n15: f64 = (p.p253 * var_qg_fp3_dn15);
        let eq179_e2257_d_n16: f64 = (p.p253 * var_qg_fp3_dn16);
        let eq179_e2257_d_n17: f64 = (p.p253 * var_qg_fp3_dn17);
        let eq179_e2257_d_n18: f64 = (p.p253 * var_qg_fp3_dn18);
        let eq179_e2257_d_n19: f64 = (p.p253 * var_qg_fp3_dn19);
        let eq179_e2257_d_n20: f64 = (p.p253 * var_qg_fp3_dn20);
        let eq179_e2257_d_n21: f64 = (p.p253 * var_qg_fp3_dn21);
        let eq179_e2257_d_n22: f64 = (p.p253 * var_qg_fp3_dn22);
        let eq179_e2258_q: f64 = eq179_e2257;
        let eq179_e2259: f64 = (p.p7 * eq179_e2257);
        let eq179_e2259_d_n0: f64 = (p.p7 * eq179_e2257_d_n0);
        let eq179_e2259_d_n1: f64 = (p.p7 * eq179_e2257_d_n1);
        let eq179_e2259_d_n2: f64 = (p.p7 * eq179_e2257_d_n2);
        let eq179_e2259_d_n3: f64 = (p.p7 * eq179_e2257_d_n3);
        let eq179_e2259_d_n4: f64 = (p.p7 * eq179_e2257_d_n4);
        let eq179_e2259_d_n5: f64 = (p.p7 * eq179_e2257_d_n5);
        let eq179_e2259_d_n6: f64 = (p.p7 * eq179_e2257_d_n6);
        let eq179_e2259_d_n7: f64 = (p.p7 * eq179_e2257_d_n7);
        let eq179_e2259_d_n8: f64 = (p.p7 * eq179_e2257_d_n8);
        let eq179_e2259_d_n9: f64 = (p.p7 * eq179_e2257_d_n9);
        let eq179_e2259_d_n12: f64 = (p.p7 * eq179_e2257_d_n12);
        let eq179_e2259_d_n14: f64 = (p.p7 * eq179_e2257_d_n14);
        let eq179_e2259_d_n15: f64 = (p.p7 * eq179_e2257_d_n15);
        let eq179_e2259_d_n16: f64 = (p.p7 * eq179_e2257_d_n16);
        let eq179_e2259_d_n17: f64 = (p.p7 * eq179_e2257_d_n17);
        let eq179_e2259_d_n18: f64 = (p.p7 * eq179_e2257_d_n18);
        let eq179_e2259_d_n19: f64 = (p.p7 * eq179_e2257_d_n19);
        let eq179_e2259_d_n20: f64 = (p.p7 * eq179_e2257_d_n20);
        let eq179_e2259_d_n21: f64 = (p.p7 * eq179_e2257_d_n21);
        let eq179_e2259_d_n22: f64 = (p.p7 * eq179_e2257_d_n22);
        let eq179_e2259_q: f64 = (p.p7 * eq179_e2258_q);
        (eq179_e2259, eq179_e2259_d_n0, eq179_e2259_d_n1, eq179_e2259_d_n2, eq179_e2259_d_n3, eq179_e2259_d_n4, eq179_e2259_d_n5, eq179_e2259_d_n6, eq179_e2259_d_n7, eq179_e2259_d_n8, eq179_e2259_d_n9, eq179_e2259_d_n12, eq179_e2259_d_n14, eq179_e2259_d_n15, eq179_e2259_d_n16, eq179_e2259_d_n17, eq179_e2259_d_n18, eq179_e2259_d_n19, eq179_e2259_d_n20, eq179_e2259_d_n21, eq179_e2259_d_n22, eq179_e2259_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq179_reactive_node_derivatives: [f64; 23] = [eq179_e2261_d_n0, eq179_e2261_d_n1, eq179_e2261_d_n2, eq179_e2261_d_n3, eq179_e2261_d_n4, eq179_e2261_d_n5, eq179_e2261_d_n6, eq179_e2261_d_n7, eq179_e2261_d_n8, eq179_e2261_d_n9, 0.0, 0.0, eq179_e2261_d_n12, 0.0, eq179_e2261_d_n14, eq179_e2261_d_n15, eq179_e2261_d_n16, eq179_e2261_d_n17, eq179_e2261_d_n18, eq179_e2261_d_n19, eq179_e2261_d_n20, eq179_e2261_d_n21, eq179_e2261_d_n22];
        let eq179_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            nodes,
            &eq179_reactive_node_derivatives,
            branches,
            &eq179_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq180_e2270, eq180_e2270_d_n0, eq180_e2270_d_n1, eq180_e2270_d_n2, eq180_e2270_d_n3, eq180_e2270_d_n4, eq180_e2270_d_n5, eq180_e2270_d_n6, eq180_e2270_d_n7, eq180_e2270_d_n8, eq180_e2270_d_n9, eq180_e2270_d_n12, eq180_e2270_d_n14, eq180_e2270_d_n15, eq180_e2270_d_n16, eq180_e2270_d_n17, eq180_e2270_d_n18, eq180_e2270_d_n19, eq180_e2270_d_n20, eq180_e2270_d_n21, eq180_e2270_d_n22, eq180_e2270_q,) = {
    if ((var_guard561 != 0.0) && (var_guard562 != 0.0)) {
        let eq180_e2267_q: f64 = var_qd_fp3s;
        let eq180_e2268: f64 = (p.p7 * var_qd_fp3s);
        let eq180_e2268_d_n0: f64 = (p.p7 * var_qd_fp3s_dn0);
        let eq180_e2268_d_n1: f64 = (p.p7 * var_qd_fp3s_dn1);
        let eq180_e2268_d_n2: f64 = (p.p7 * var_qd_fp3s_dn2);
        let eq180_e2268_d_n3: f64 = (p.p7 * var_qd_fp3s_dn3);
        let eq180_e2268_d_n4: f64 = (p.p7 * var_qd_fp3s_dn4);
        let eq180_e2268_d_n5: f64 = (p.p7 * var_qd_fp3s_dn5);
        let eq180_e2268_d_n6: f64 = (p.p7 * var_qd_fp3s_dn6);
        let eq180_e2268_d_n7: f64 = (p.p7 * var_qd_fp3s_dn7);
        let eq180_e2268_d_n8: f64 = (p.p7 * var_qd_fp3s_dn8);
        let eq180_e2268_d_n9: f64 = (p.p7 * var_qd_fp3s_dn9);
        let eq180_e2268_d_n12: f64 = (p.p7 * var_qd_fp3s_dn12);
        let eq180_e2268_d_n14: f64 = (p.p7 * var_qd_fp3s_dn14);
        let eq180_e2268_d_n15: f64 = (p.p7 * var_qd_fp3s_dn15);
        let eq180_e2268_d_n16: f64 = (p.p7 * var_qd_fp3s_dn16);
        let eq180_e2268_d_n17: f64 = (p.p7 * var_qd_fp3s_dn17);
        let eq180_e2268_d_n18: f64 = (p.p7 * var_qd_fp3s_dn18);
        let eq180_e2268_d_n19: f64 = (p.p7 * var_qd_fp3s_dn19);
        let eq180_e2268_d_n20: f64 = (p.p7 * var_qd_fp3s_dn20);
        let eq180_e2268_d_n21: f64 = (p.p7 * var_qd_fp3s_dn21);
        let eq180_e2268_d_n22: f64 = (p.p7 * var_qd_fp3s_dn22);
        let eq180_e2268_q: f64 = (p.p7 * eq180_e2267_q);
        (eq180_e2268, eq180_e2268_d_n0, eq180_e2268_d_n1, eq180_e2268_d_n2, eq180_e2268_d_n3, eq180_e2268_d_n4, eq180_e2268_d_n5, eq180_e2268_d_n6, eq180_e2268_d_n7, eq180_e2268_d_n8, eq180_e2268_d_n9, eq180_e2268_d_n12, eq180_e2268_d_n14, eq180_e2268_d_n15, eq180_e2268_d_n16, eq180_e2268_d_n17, eq180_e2268_d_n18, eq180_e2268_d_n19, eq180_e2268_d_n20, eq180_e2268_d_n21, eq180_e2268_d_n22, eq180_e2268_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq180_reactive_node_derivatives: [f64; 23] = [eq180_e2270_d_n0, eq180_e2270_d_n1, eq180_e2270_d_n2, eq180_e2270_d_n3, eq180_e2270_d_n4, eq180_e2270_d_n5, eq180_e2270_d_n6, eq180_e2270_d_n7, eq180_e2270_d_n8, eq180_e2270_d_n9, 0.0, 0.0, eq180_e2270_d_n12, 0.0, eq180_e2270_d_n14, eq180_e2270_d_n15, eq180_e2270_d_n16, eq180_e2270_d_n17, eq180_e2270_d_n18, eq180_e2270_d_n19, eq180_e2270_d_n20, eq180_e2270_d_n21, eq180_e2270_d_n22];
        let eq180_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[21]),
            nodes,
            &eq180_reactive_node_derivatives,
            branches,
            &eq180_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq181_e2281, eq181_e2281_d_n0, eq181_e2281_d_n1, eq181_e2281_d_n2, eq181_e2281_d_n3, eq181_e2281_d_n4, eq181_e2281_d_n5, eq181_e2281_d_n6, eq181_e2281_d_n7, eq181_e2281_d_n8, eq181_e2281_d_n9, eq181_e2281_d_n12, eq181_e2281_d_n14, eq181_e2281_d_n15, eq181_e2281_d_n16, eq181_e2281_d_n17, eq181_e2281_d_n18, eq181_e2281_d_n19, eq181_e2281_d_n20, eq181_e2281_d_n21, eq181_e2281_d_n22, eq181_e2281_q,) = {
    if (((var_guard561 != 0.0) && (var_guard562 != 0.0)) && (var_guard563 != 0.0)) {
        let eq181_e2278_q: f64 = var_qg_fp3s;
        let eq181_e2279: f64 = (p.p7 * var_qg_fp3s);
        let eq181_e2279_d_n0: f64 = (p.p7 * var_qg_fp3s_dn0);
        let eq181_e2279_d_n1: f64 = (p.p7 * var_qg_fp3s_dn1);
        let eq181_e2279_d_n2: f64 = (p.p7 * var_qg_fp3s_dn2);
        let eq181_e2279_d_n3: f64 = (p.p7 * var_qg_fp3s_dn3);
        let eq181_e2279_d_n4: f64 = (p.p7 * var_qg_fp3s_dn4);
        let eq181_e2279_d_n5: f64 = (p.p7 * var_qg_fp3s_dn5);
        let eq181_e2279_d_n6: f64 = (p.p7 * var_qg_fp3s_dn6);
        let eq181_e2279_d_n7: f64 = (p.p7 * var_qg_fp3s_dn7);
        let eq181_e2279_d_n8: f64 = (p.p7 * var_qg_fp3s_dn8);
        let eq181_e2279_d_n9: f64 = (p.p7 * var_qg_fp3s_dn9);
        let eq181_e2279_d_n12: f64 = (p.p7 * var_qg_fp3s_dn12);
        let eq181_e2279_d_n14: f64 = (p.p7 * var_qg_fp3s_dn14);
        let eq181_e2279_d_n15: f64 = (p.p7 * var_qg_fp3s_dn15);
        let eq181_e2279_d_n16: f64 = (p.p7 * var_qg_fp3s_dn16);
        let eq181_e2279_d_n17: f64 = (p.p7 * var_qg_fp3s_dn17);
        let eq181_e2279_d_n18: f64 = (p.p7 * var_qg_fp3s_dn18);
        let eq181_e2279_d_n19: f64 = (p.p7 * var_qg_fp3s_dn19);
        let eq181_e2279_d_n20: f64 = (p.p7 * var_qg_fp3s_dn20);
        let eq181_e2279_d_n21: f64 = (p.p7 * var_qg_fp3s_dn21);
        let eq181_e2279_d_n22: f64 = (p.p7 * var_qg_fp3s_dn22);
        let eq181_e2279_q: f64 = (p.p7 * eq181_e2278_q);
        (eq181_e2279, eq181_e2279_d_n0, eq181_e2279_d_n1, eq181_e2279_d_n2, eq181_e2279_d_n3, eq181_e2279_d_n4, eq181_e2279_d_n5, eq181_e2279_d_n6, eq181_e2279_d_n7, eq181_e2279_d_n8, eq181_e2279_d_n9, eq181_e2279_d_n12, eq181_e2279_d_n14, eq181_e2279_d_n15, eq181_e2279_d_n16, eq181_e2279_d_n17, eq181_e2279_d_n18, eq181_e2279_d_n19, eq181_e2279_d_n20, eq181_e2279_d_n21, eq181_e2279_d_n22, eq181_e2279_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq181_reactive_node_derivatives: [f64; 23] = [eq181_e2281_d_n0, eq181_e2281_d_n1, eq181_e2281_d_n2, eq181_e2281_d_n3, eq181_e2281_d_n4, eq181_e2281_d_n5, eq181_e2281_d_n6, eq181_e2281_d_n7, eq181_e2281_d_n8, eq181_e2281_d_n9, 0.0, 0.0, eq181_e2281_d_n12, 0.0, eq181_e2281_d_n14, eq181_e2281_d_n15, eq181_e2281_d_n16, eq181_e2281_d_n17, eq181_e2281_d_n18, eq181_e2281_d_n19, eq181_e2281_d_n20, eq181_e2281_d_n21, eq181_e2281_d_n22];
        let eq181_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[21]),
            nodes,
            &eq181_reactive_node_derivatives,
            branches,
            &eq181_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq182_e2294, eq182_e2294_d_n0, eq182_e2294_d_n1, eq182_e2294_d_n2, eq182_e2294_d_n3, eq182_e2294_d_n4, eq182_e2294_d_n5, eq182_e2294_d_n6, eq182_e2294_d_n7, eq182_e2294_d_n8, eq182_e2294_d_n9, eq182_e2294_d_n12, eq182_e2294_d_n14, eq182_e2294_d_n15, eq182_e2294_d_n16, eq182_e2294_d_n17, eq182_e2294_d_n18, eq182_e2294_d_n19, eq182_e2294_d_n20, eq182_e2294_d_n21, eq182_e2294_d_n22, eq182_e2294_q,) = {
    if (((var_guard561 != 0.0) && (var_guard562 != 0.0)) && (var_guard563 != 0.0)) {
        let eq182_e2289_q: f64 = var_qg_fp3s;
        let eq182_e2290: f64 = (p.p7 * var_qg_fp3s);
        let eq182_e2290_d_n0: f64 = (p.p7 * var_qg_fp3s_dn0);
        let eq182_e2290_d_n1: f64 = (p.p7 * var_qg_fp3s_dn1);
        let eq182_e2290_d_n2: f64 = (p.p7 * var_qg_fp3s_dn2);
        let eq182_e2290_d_n3: f64 = (p.p7 * var_qg_fp3s_dn3);
        let eq182_e2290_d_n4: f64 = (p.p7 * var_qg_fp3s_dn4);
        let eq182_e2290_d_n5: f64 = (p.p7 * var_qg_fp3s_dn5);
        let eq182_e2290_d_n6: f64 = (p.p7 * var_qg_fp3s_dn6);
        let eq182_e2290_d_n7: f64 = (p.p7 * var_qg_fp3s_dn7);
        let eq182_e2290_d_n8: f64 = (p.p7 * var_qg_fp3s_dn8);
        let eq182_e2290_d_n9: f64 = (p.p7 * var_qg_fp3s_dn9);
        let eq182_e2290_d_n12: f64 = (p.p7 * var_qg_fp3s_dn12);
        let eq182_e2290_d_n14: f64 = (p.p7 * var_qg_fp3s_dn14);
        let eq182_e2290_d_n15: f64 = (p.p7 * var_qg_fp3s_dn15);
        let eq182_e2290_d_n16: f64 = (p.p7 * var_qg_fp3s_dn16);
        let eq182_e2290_d_n17: f64 = (p.p7 * var_qg_fp3s_dn17);
        let eq182_e2290_d_n18: f64 = (p.p7 * var_qg_fp3s_dn18);
        let eq182_e2290_d_n19: f64 = (p.p7 * var_qg_fp3s_dn19);
        let eq182_e2290_d_n20: f64 = (p.p7 * var_qg_fp3s_dn20);
        let eq182_e2290_d_n21: f64 = (p.p7 * var_qg_fp3s_dn21);
        let eq182_e2290_d_n22: f64 = (p.p7 * var_qg_fp3s_dn22);
        let eq182_e2290_q: f64 = (p.p7 * eq182_e2289_q);
        let eq182_e2292: f64 = (eq182_e2290 * p.p248);
        let eq182_e2292_d_n0: f64 = (eq182_e2290_d_n0 * p.p248);
        let eq182_e2292_d_n1: f64 = (eq182_e2290_d_n1 * p.p248);
        let eq182_e2292_d_n2: f64 = (eq182_e2290_d_n2 * p.p248);
        let eq182_e2292_d_n3: f64 = (eq182_e2290_d_n3 * p.p248);
        let eq182_e2292_d_n4: f64 = (eq182_e2290_d_n4 * p.p248);
        let eq182_e2292_d_n5: f64 = (eq182_e2290_d_n5 * p.p248);
        let eq182_e2292_d_n6: f64 = (eq182_e2290_d_n6 * p.p248);
        let eq182_e2292_d_n7: f64 = (eq182_e2290_d_n7 * p.p248);
        let eq182_e2292_d_n8: f64 = (eq182_e2290_d_n8 * p.p248);
        let eq182_e2292_d_n9: f64 = (eq182_e2290_d_n9 * p.p248);
        let eq182_e2292_d_n12: f64 = (eq182_e2290_d_n12 * p.p248);
        let eq182_e2292_d_n14: f64 = (eq182_e2290_d_n14 * p.p248);
        let eq182_e2292_d_n15: f64 = (eq182_e2290_d_n15 * p.p248);
        let eq182_e2292_d_n16: f64 = (eq182_e2290_d_n16 * p.p248);
        let eq182_e2292_d_n17: f64 = (eq182_e2290_d_n17 * p.p248);
        let eq182_e2292_d_n18: f64 = (eq182_e2290_d_n18 * p.p248);
        let eq182_e2292_d_n19: f64 = (eq182_e2290_d_n19 * p.p248);
        let eq182_e2292_d_n20: f64 = (eq182_e2290_d_n20 * p.p248);
        let eq182_e2292_d_n21: f64 = (eq182_e2290_d_n21 * p.p248);
        let eq182_e2292_d_n22: f64 = (eq182_e2290_d_n22 * p.p248);
        let eq182_e2292_q: f64 = (eq182_e2290_q * p.p248);
        (eq182_e2292, eq182_e2292_d_n0, eq182_e2292_d_n1, eq182_e2292_d_n2, eq182_e2292_d_n3, eq182_e2292_d_n4, eq182_e2292_d_n5, eq182_e2292_d_n6, eq182_e2292_d_n7, eq182_e2292_d_n8, eq182_e2292_d_n9, eq182_e2292_d_n12, eq182_e2292_d_n14, eq182_e2292_d_n15, eq182_e2292_d_n16, eq182_e2292_d_n17, eq182_e2292_d_n18, eq182_e2292_d_n19, eq182_e2292_d_n20, eq182_e2292_d_n21, eq182_e2292_d_n22, eq182_e2292_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq182_reactive_node_derivatives: [f64; 23] = [eq182_e2294_d_n0, eq182_e2294_d_n1, eq182_e2294_d_n2, eq182_e2294_d_n3, eq182_e2294_d_n4, eq182_e2294_d_n5, eq182_e2294_d_n6, eq182_e2294_d_n7, eq182_e2294_d_n8, eq182_e2294_d_n9, 0.0, 0.0, eq182_e2294_d_n12, 0.0, eq182_e2294_d_n14, eq182_e2294_d_n15, eq182_e2294_d_n16, eq182_e2294_d_n17, eq182_e2294_d_n18, eq182_e2294_d_n19, eq182_e2294_d_n20, eq182_e2294_d_n21, eq182_e2294_d_n22];
        let eq182_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[21]),
            nodes,
            &eq182_reactive_node_derivatives,
            branches,
            &eq182_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq183_e2306, eq183_e2306_d_n0, eq183_e2306_d_n1, eq183_e2306_d_n2, eq183_e2306_d_n3, eq183_e2306_d_n4, eq183_e2306_d_n5, eq183_e2306_d_n6, eq183_e2306_d_n7, eq183_e2306_d_n8, eq183_e2306_d_n9, eq183_e2306_d_n12, eq183_e2306_d_n14, eq183_e2306_d_n15, eq183_e2306_d_n16, eq183_e2306_d_n17, eq183_e2306_d_n18, eq183_e2306_d_n19, eq183_e2306_d_n20, eq183_e2306_d_n21, eq183_e2306_d_n22, eq183_e2306_q,) = {
    if (((var_guard561 != 0.0) && (var_guard562 != 0.0)) && (var_guard563 == 0.0)) {
        let eq183_e2303_q: f64 = var_qg_fp3s;
        let eq183_e2304: f64 = (p.p7 * var_qg_fp3s);
        let eq183_e2304_d_n0: f64 = (p.p7 * var_qg_fp3s_dn0);
        let eq183_e2304_d_n1: f64 = (p.p7 * var_qg_fp3s_dn1);
        let eq183_e2304_d_n2: f64 = (p.p7 * var_qg_fp3s_dn2);
        let eq183_e2304_d_n3: f64 = (p.p7 * var_qg_fp3s_dn3);
        let eq183_e2304_d_n4: f64 = (p.p7 * var_qg_fp3s_dn4);
        let eq183_e2304_d_n5: f64 = (p.p7 * var_qg_fp3s_dn5);
        let eq183_e2304_d_n6: f64 = (p.p7 * var_qg_fp3s_dn6);
        let eq183_e2304_d_n7: f64 = (p.p7 * var_qg_fp3s_dn7);
        let eq183_e2304_d_n8: f64 = (p.p7 * var_qg_fp3s_dn8);
        let eq183_e2304_d_n9: f64 = (p.p7 * var_qg_fp3s_dn9);
        let eq183_e2304_d_n12: f64 = (p.p7 * var_qg_fp3s_dn12);
        let eq183_e2304_d_n14: f64 = (p.p7 * var_qg_fp3s_dn14);
        let eq183_e2304_d_n15: f64 = (p.p7 * var_qg_fp3s_dn15);
        let eq183_e2304_d_n16: f64 = (p.p7 * var_qg_fp3s_dn16);
        let eq183_e2304_d_n17: f64 = (p.p7 * var_qg_fp3s_dn17);
        let eq183_e2304_d_n18: f64 = (p.p7 * var_qg_fp3s_dn18);
        let eq183_e2304_d_n19: f64 = (p.p7 * var_qg_fp3s_dn19);
        let eq183_e2304_d_n20: f64 = (p.p7 * var_qg_fp3s_dn20);
        let eq183_e2304_d_n21: f64 = (p.p7 * var_qg_fp3s_dn21);
        let eq183_e2304_d_n22: f64 = (p.p7 * var_qg_fp3s_dn22);
        let eq183_e2304_q: f64 = (p.p7 * eq183_e2303_q);
        (eq183_e2304, eq183_e2304_d_n0, eq183_e2304_d_n1, eq183_e2304_d_n2, eq183_e2304_d_n3, eq183_e2304_d_n4, eq183_e2304_d_n5, eq183_e2304_d_n6, eq183_e2304_d_n7, eq183_e2304_d_n8, eq183_e2304_d_n9, eq183_e2304_d_n12, eq183_e2304_d_n14, eq183_e2304_d_n15, eq183_e2304_d_n16, eq183_e2304_d_n17, eq183_e2304_d_n18, eq183_e2304_d_n19, eq183_e2304_d_n20, eq183_e2304_d_n21, eq183_e2304_d_n22, eq183_e2304_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq183_reactive_node_derivatives: [f64; 23] = [eq183_e2306_d_n0, eq183_e2306_d_n1, eq183_e2306_d_n2, eq183_e2306_d_n3, eq183_e2306_d_n4, eq183_e2306_d_n5, eq183_e2306_d_n6, eq183_e2306_d_n7, eq183_e2306_d_n8, eq183_e2306_d_n9, 0.0, 0.0, eq183_e2306_d_n12, 0.0, eq183_e2306_d_n14, eq183_e2306_d_n15, eq183_e2306_d_n16, eq183_e2306_d_n17, eq183_e2306_d_n18, eq183_e2306_d_n19, eq183_e2306_d_n20, eq183_e2306_d_n21, eq183_e2306_d_n22];
        let eq183_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[21]),
            nodes,
            &eq183_reactive_node_derivatives,
            branches,
            &eq183_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq184_e2320, eq184_e2320_d_n0, eq184_e2320_d_n1, eq184_e2320_d_n2, eq184_e2320_d_n3, eq184_e2320_d_n4, eq184_e2320_d_n5, eq184_e2320_d_n6, eq184_e2320_d_n7, eq184_e2320_d_n8, eq184_e2320_d_n9, eq184_e2320_d_n12, eq184_e2320_d_n14, eq184_e2320_d_n15, eq184_e2320_d_n16, eq184_e2320_d_n17, eq184_e2320_d_n18, eq184_e2320_d_n19, eq184_e2320_d_n20, eq184_e2320_d_n21, eq184_e2320_d_n22, eq184_e2320_q,) = {
    if (((var_guard561 != 0.0) && (var_guard562 != 0.0)) && (var_guard563 == 0.0)) {
        let eq184_e2315_q: f64 = var_qg_fp3s;
        let eq184_e2316: f64 = (p.p7 * var_qg_fp3s);
        let eq184_e2316_d_n0: f64 = (p.p7 * var_qg_fp3s_dn0);
        let eq184_e2316_d_n1: f64 = (p.p7 * var_qg_fp3s_dn1);
        let eq184_e2316_d_n2: f64 = (p.p7 * var_qg_fp3s_dn2);
        let eq184_e2316_d_n3: f64 = (p.p7 * var_qg_fp3s_dn3);
        let eq184_e2316_d_n4: f64 = (p.p7 * var_qg_fp3s_dn4);
        let eq184_e2316_d_n5: f64 = (p.p7 * var_qg_fp3s_dn5);
        let eq184_e2316_d_n6: f64 = (p.p7 * var_qg_fp3s_dn6);
        let eq184_e2316_d_n7: f64 = (p.p7 * var_qg_fp3s_dn7);
        let eq184_e2316_d_n8: f64 = (p.p7 * var_qg_fp3s_dn8);
        let eq184_e2316_d_n9: f64 = (p.p7 * var_qg_fp3s_dn9);
        let eq184_e2316_d_n12: f64 = (p.p7 * var_qg_fp3s_dn12);
        let eq184_e2316_d_n14: f64 = (p.p7 * var_qg_fp3s_dn14);
        let eq184_e2316_d_n15: f64 = (p.p7 * var_qg_fp3s_dn15);
        let eq184_e2316_d_n16: f64 = (p.p7 * var_qg_fp3s_dn16);
        let eq184_e2316_d_n17: f64 = (p.p7 * var_qg_fp3s_dn17);
        let eq184_e2316_d_n18: f64 = (p.p7 * var_qg_fp3s_dn18);
        let eq184_e2316_d_n19: f64 = (p.p7 * var_qg_fp3s_dn19);
        let eq184_e2316_d_n20: f64 = (p.p7 * var_qg_fp3s_dn20);
        let eq184_e2316_d_n21: f64 = (p.p7 * var_qg_fp3s_dn21);
        let eq184_e2316_d_n22: f64 = (p.p7 * var_qg_fp3s_dn22);
        let eq184_e2316_q: f64 = (p.p7 * eq184_e2315_q);
        let eq184_e2318: f64 = (eq184_e2316 * p.p248);
        let eq184_e2318_d_n0: f64 = (eq184_e2316_d_n0 * p.p248);
        let eq184_e2318_d_n1: f64 = (eq184_e2316_d_n1 * p.p248);
        let eq184_e2318_d_n2: f64 = (eq184_e2316_d_n2 * p.p248);
        let eq184_e2318_d_n3: f64 = (eq184_e2316_d_n3 * p.p248);
        let eq184_e2318_d_n4: f64 = (eq184_e2316_d_n4 * p.p248);
        let eq184_e2318_d_n5: f64 = (eq184_e2316_d_n5 * p.p248);
        let eq184_e2318_d_n6: f64 = (eq184_e2316_d_n6 * p.p248);
        let eq184_e2318_d_n7: f64 = (eq184_e2316_d_n7 * p.p248);
        let eq184_e2318_d_n8: f64 = (eq184_e2316_d_n8 * p.p248);
        let eq184_e2318_d_n9: f64 = (eq184_e2316_d_n9 * p.p248);
        let eq184_e2318_d_n12: f64 = (eq184_e2316_d_n12 * p.p248);
        let eq184_e2318_d_n14: f64 = (eq184_e2316_d_n14 * p.p248);
        let eq184_e2318_d_n15: f64 = (eq184_e2316_d_n15 * p.p248);
        let eq184_e2318_d_n16: f64 = (eq184_e2316_d_n16 * p.p248);
        let eq184_e2318_d_n17: f64 = (eq184_e2316_d_n17 * p.p248);
        let eq184_e2318_d_n18: f64 = (eq184_e2316_d_n18 * p.p248);
        let eq184_e2318_d_n19: f64 = (eq184_e2316_d_n19 * p.p248);
        let eq184_e2318_d_n20: f64 = (eq184_e2316_d_n20 * p.p248);
        let eq184_e2318_d_n21: f64 = (eq184_e2316_d_n21 * p.p248);
        let eq184_e2318_d_n22: f64 = (eq184_e2316_d_n22 * p.p248);
        let eq184_e2318_q: f64 = (eq184_e2316_q * p.p248);
        (eq184_e2318, eq184_e2318_d_n0, eq184_e2318_d_n1, eq184_e2318_d_n2, eq184_e2318_d_n3, eq184_e2318_d_n4, eq184_e2318_d_n5, eq184_e2318_d_n6, eq184_e2318_d_n7, eq184_e2318_d_n8, eq184_e2318_d_n9, eq184_e2318_d_n12, eq184_e2318_d_n14, eq184_e2318_d_n15, eq184_e2318_d_n16, eq184_e2318_d_n17, eq184_e2318_d_n18, eq184_e2318_d_n19, eq184_e2318_d_n20, eq184_e2318_d_n21, eq184_e2318_d_n22, eq184_e2318_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq184_reactive_node_derivatives: [f64; 23] = [eq184_e2320_d_n0, eq184_e2320_d_n1, eq184_e2320_d_n2, eq184_e2320_d_n3, eq184_e2320_d_n4, eq184_e2320_d_n5, eq184_e2320_d_n6, eq184_e2320_d_n7, eq184_e2320_d_n8, eq184_e2320_d_n9, 0.0, 0.0, eq184_e2320_d_n12, 0.0, eq184_e2320_d_n14, eq184_e2320_d_n15, eq184_e2320_d_n16, eq184_e2320_d_n17, eq184_e2320_d_n18, eq184_e2320_d_n19, eq184_e2320_d_n20, eq184_e2320_d_n21, eq184_e2320_d_n22];
        let eq184_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[21]),
            nodes,
            &eq184_reactive_node_derivatives,
            branches,
            &eq184_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq185_e2331, eq185_e2331_d_n0, eq185_e2331_d_n1, eq185_e2331_d_n2, eq185_e2331_d_n3, eq185_e2331_d_n4, eq185_e2331_d_n5, eq185_e2331_d_n6, eq185_e2331_d_n7, eq185_e2331_d_n8, eq185_e2331_d_n9, eq185_e2331_d_n12, eq185_e2331_d_n14, eq185_e2331_d_n15, eq185_e2331_d_n16, eq185_e2331_d_n17, eq185_e2331_d_n18, eq185_e2331_d_n19, eq185_e2331_d_n20, eq185_e2331_d_n21, eq185_e2331_d_n22, eq185_e2331_q,) = {
    if ((var_guard561 != 0.0) && (var_guard562 != 0.0)) {
        let eq185_e2327: f64 = (p.p253 * var_qg_fp3s);
        let eq185_e2327_d_n0: f64 = (p.p253 * var_qg_fp3s_dn0);
        let eq185_e2327_d_n1: f64 = (p.p253 * var_qg_fp3s_dn1);
        let eq185_e2327_d_n2: f64 = (p.p253 * var_qg_fp3s_dn2);
        let eq185_e2327_d_n3: f64 = (p.p253 * var_qg_fp3s_dn3);
        let eq185_e2327_d_n4: f64 = (p.p253 * var_qg_fp3s_dn4);
        let eq185_e2327_d_n5: f64 = (p.p253 * var_qg_fp3s_dn5);
        let eq185_e2327_d_n6: f64 = (p.p253 * var_qg_fp3s_dn6);
        let eq185_e2327_d_n7: f64 = (p.p253 * var_qg_fp3s_dn7);
        let eq185_e2327_d_n8: f64 = (p.p253 * var_qg_fp3s_dn8);
        let eq185_e2327_d_n9: f64 = (p.p253 * var_qg_fp3s_dn9);
        let eq185_e2327_d_n12: f64 = (p.p253 * var_qg_fp3s_dn12);
        let eq185_e2327_d_n14: f64 = (p.p253 * var_qg_fp3s_dn14);
        let eq185_e2327_d_n15: f64 = (p.p253 * var_qg_fp3s_dn15);
        let eq185_e2327_d_n16: f64 = (p.p253 * var_qg_fp3s_dn16);
        let eq185_e2327_d_n17: f64 = (p.p253 * var_qg_fp3s_dn17);
        let eq185_e2327_d_n18: f64 = (p.p253 * var_qg_fp3s_dn18);
        let eq185_e2327_d_n19: f64 = (p.p253 * var_qg_fp3s_dn19);
        let eq185_e2327_d_n20: f64 = (p.p253 * var_qg_fp3s_dn20);
        let eq185_e2327_d_n21: f64 = (p.p253 * var_qg_fp3s_dn21);
        let eq185_e2327_d_n22: f64 = (p.p253 * var_qg_fp3s_dn22);
        let eq185_e2328_q: f64 = eq185_e2327;
        let eq185_e2329: f64 = (p.p7 * eq185_e2327);
        let eq185_e2329_d_n0: f64 = (p.p7 * eq185_e2327_d_n0);
        let eq185_e2329_d_n1: f64 = (p.p7 * eq185_e2327_d_n1);
        let eq185_e2329_d_n2: f64 = (p.p7 * eq185_e2327_d_n2);
        let eq185_e2329_d_n3: f64 = (p.p7 * eq185_e2327_d_n3);
        let eq185_e2329_d_n4: f64 = (p.p7 * eq185_e2327_d_n4);
        let eq185_e2329_d_n5: f64 = (p.p7 * eq185_e2327_d_n5);
        let eq185_e2329_d_n6: f64 = (p.p7 * eq185_e2327_d_n6);
        let eq185_e2329_d_n7: f64 = (p.p7 * eq185_e2327_d_n7);
        let eq185_e2329_d_n8: f64 = (p.p7 * eq185_e2327_d_n8);
        let eq185_e2329_d_n9: f64 = (p.p7 * eq185_e2327_d_n9);
        let eq185_e2329_d_n12: f64 = (p.p7 * eq185_e2327_d_n12);
        let eq185_e2329_d_n14: f64 = (p.p7 * eq185_e2327_d_n14);
        let eq185_e2329_d_n15: f64 = (p.p7 * eq185_e2327_d_n15);
        let eq185_e2329_d_n16: f64 = (p.p7 * eq185_e2327_d_n16);
        let eq185_e2329_d_n17: f64 = (p.p7 * eq185_e2327_d_n17);
        let eq185_e2329_d_n18: f64 = (p.p7 * eq185_e2327_d_n18);
        let eq185_e2329_d_n19: f64 = (p.p7 * eq185_e2327_d_n19);
        let eq185_e2329_d_n20: f64 = (p.p7 * eq185_e2327_d_n20);
        let eq185_e2329_d_n21: f64 = (p.p7 * eq185_e2327_d_n21);
        let eq185_e2329_d_n22: f64 = (p.p7 * eq185_e2327_d_n22);
        let eq185_e2329_q: f64 = (p.p7 * eq185_e2328_q);
        (eq185_e2329, eq185_e2329_d_n0, eq185_e2329_d_n1, eq185_e2329_d_n2, eq185_e2329_d_n3, eq185_e2329_d_n4, eq185_e2329_d_n5, eq185_e2329_d_n6, eq185_e2329_d_n7, eq185_e2329_d_n8, eq185_e2329_d_n9, eq185_e2329_d_n12, eq185_e2329_d_n14, eq185_e2329_d_n15, eq185_e2329_d_n16, eq185_e2329_d_n17, eq185_e2329_d_n18, eq185_e2329_d_n19, eq185_e2329_d_n20, eq185_e2329_d_n21, eq185_e2329_d_n22, eq185_e2329_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq185_reactive_node_derivatives: [f64; 23] = [eq185_e2331_d_n0, eq185_e2331_d_n1, eq185_e2331_d_n2, eq185_e2331_d_n3, eq185_e2331_d_n4, eq185_e2331_d_n5, eq185_e2331_d_n6, eq185_e2331_d_n7, eq185_e2331_d_n8, eq185_e2331_d_n9, 0.0, 0.0, eq185_e2331_d_n12, 0.0, eq185_e2331_d_n14, eq185_e2331_d_n15, eq185_e2331_d_n16, eq185_e2331_d_n17, eq185_e2331_d_n18, eq185_e2331_d_n19, eq185_e2331_d_n20, eq185_e2331_d_n21, eq185_e2331_d_n22];
        let eq185_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[21]),
            nodes,
            &eq185_reactive_node_derivatives,
            branches,
            &eq185_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq186_e2341, eq186_e2341_d_n0, eq186_e2341_d_n1, eq186_e2341_d_n2, eq186_e2341_d_n3, eq186_e2341_d_n4, eq186_e2341_d_n5, eq186_e2341_d_n6, eq186_e2341_d_n7, eq186_e2341_d_n8, eq186_e2341_d_n9, eq186_e2341_d_n12, eq186_e2341_d_n14, eq186_e2341_d_n15, eq186_e2341_d_n16, eq186_e2341_d_n17, eq186_e2341_d_n18, eq186_e2341_d_n19, eq186_e2341_d_n20, eq186_e2341_d_n21, eq186_e2341_d_n22, eq186_e2341_q,) = {
    if ((var_guard561 == 0.0) && (var_guard564 != 0.0)) {
        let eq186_e2338_q: f64 = var_qd_fp3s;
        let eq186_e2339: f64 = (p.p7 * var_qd_fp3s);
        let eq186_e2339_d_n0: f64 = (p.p7 * var_qd_fp3s_dn0);
        let eq186_e2339_d_n1: f64 = (p.p7 * var_qd_fp3s_dn1);
        let eq186_e2339_d_n2: f64 = (p.p7 * var_qd_fp3s_dn2);
        let eq186_e2339_d_n3: f64 = (p.p7 * var_qd_fp3s_dn3);
        let eq186_e2339_d_n4: f64 = (p.p7 * var_qd_fp3s_dn4);
        let eq186_e2339_d_n5: f64 = (p.p7 * var_qd_fp3s_dn5);
        let eq186_e2339_d_n6: f64 = (p.p7 * var_qd_fp3s_dn6);
        let eq186_e2339_d_n7: f64 = (p.p7 * var_qd_fp3s_dn7);
        let eq186_e2339_d_n8: f64 = (p.p7 * var_qd_fp3s_dn8);
        let eq186_e2339_d_n9: f64 = (p.p7 * var_qd_fp3s_dn9);
        let eq186_e2339_d_n12: f64 = (p.p7 * var_qd_fp3s_dn12);
        let eq186_e2339_d_n14: f64 = (p.p7 * var_qd_fp3s_dn14);
        let eq186_e2339_d_n15: f64 = (p.p7 * var_qd_fp3s_dn15);
        let eq186_e2339_d_n16: f64 = (p.p7 * var_qd_fp3s_dn16);
        let eq186_e2339_d_n17: f64 = (p.p7 * var_qd_fp3s_dn17);
        let eq186_e2339_d_n18: f64 = (p.p7 * var_qd_fp3s_dn18);
        let eq186_e2339_d_n19: f64 = (p.p7 * var_qd_fp3s_dn19);
        let eq186_e2339_d_n20: f64 = (p.p7 * var_qd_fp3s_dn20);
        let eq186_e2339_d_n21: f64 = (p.p7 * var_qd_fp3s_dn21);
        let eq186_e2339_d_n22: f64 = (p.p7 * var_qd_fp3s_dn22);
        let eq186_e2339_q: f64 = (p.p7 * eq186_e2338_q);
        (eq186_e2339, eq186_e2339_d_n0, eq186_e2339_d_n1, eq186_e2339_d_n2, eq186_e2339_d_n3, eq186_e2339_d_n4, eq186_e2339_d_n5, eq186_e2339_d_n6, eq186_e2339_d_n7, eq186_e2339_d_n8, eq186_e2339_d_n9, eq186_e2339_d_n12, eq186_e2339_d_n14, eq186_e2339_d_n15, eq186_e2339_d_n16, eq186_e2339_d_n17, eq186_e2339_d_n18, eq186_e2339_d_n19, eq186_e2339_d_n20, eq186_e2339_d_n21, eq186_e2339_d_n22, eq186_e2339_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq186_reactive_node_derivatives: [f64; 23] = [eq186_e2341_d_n0, eq186_e2341_d_n1, eq186_e2341_d_n2, eq186_e2341_d_n3, eq186_e2341_d_n4, eq186_e2341_d_n5, eq186_e2341_d_n6, eq186_e2341_d_n7, eq186_e2341_d_n8, eq186_e2341_d_n9, 0.0, 0.0, eq186_e2341_d_n12, 0.0, eq186_e2341_d_n14, eq186_e2341_d_n15, eq186_e2341_d_n16, eq186_e2341_d_n17, eq186_e2341_d_n18, eq186_e2341_d_n19, eq186_e2341_d_n20, eq186_e2341_d_n21, eq186_e2341_d_n22];
        let eq186_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            nodes,
            &eq186_reactive_node_derivatives,
            branches,
            &eq186_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_8(
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_guard561: f64,
        var_guard564: f64,
        var_guard565: f64,
        var_guard566: f64,
        var_guard567: f64,
        var_guard568: f64,
        var_qd_fp4: f64,
        var_qd_fp4_dn0: f64,
        var_qd_fp4_dn1: f64,
        var_qd_fp4_dn12: f64,
        var_qd_fp4_dn14: f64,
        var_qd_fp4_dn15: f64,
        var_qd_fp4_dn16: f64,
        var_qd_fp4_dn17: f64,
        var_qd_fp4_dn18: f64,
        var_qd_fp4_dn19: f64,
        var_qd_fp4_dn2: f64,
        var_qd_fp4_dn20: f64,
        var_qd_fp4_dn21: f64,
        var_qd_fp4_dn22: f64,
        var_qd_fp4_dn3: f64,
        var_qd_fp4_dn4: f64,
        var_qd_fp4_dn5: f64,
        var_qd_fp4_dn6: f64,
        var_qd_fp4_dn7: f64,
        var_qd_fp4_dn8: f64,
        var_qd_fp4_dn9: f64,
        var_qg_fp3s: f64,
        var_qg_fp3s_dn0: f64,
        var_qg_fp3s_dn1: f64,
        var_qg_fp3s_dn12: f64,
        var_qg_fp3s_dn14: f64,
        var_qg_fp3s_dn15: f64,
        var_qg_fp3s_dn16: f64,
        var_qg_fp3s_dn17: f64,
        var_qg_fp3s_dn18: f64,
        var_qg_fp3s_dn19: f64,
        var_qg_fp3s_dn2: f64,
        var_qg_fp3s_dn20: f64,
        var_qg_fp3s_dn21: f64,
        var_qg_fp3s_dn22: f64,
        var_qg_fp3s_dn3: f64,
        var_qg_fp3s_dn4: f64,
        var_qg_fp3s_dn5: f64,
        var_qg_fp3s_dn6: f64,
        var_qg_fp3s_dn7: f64,
        var_qg_fp3s_dn8: f64,
        var_qg_fp3s_dn9: f64,
        var_qg_fp4: f64,
        var_qg_fp4_dn0: f64,
        var_qg_fp4_dn1: f64,
        var_qg_fp4_dn12: f64,
        var_qg_fp4_dn14: f64,
        var_qg_fp4_dn15: f64,
        var_qg_fp4_dn16: f64,
        var_qg_fp4_dn17: f64,
        var_qg_fp4_dn18: f64,
        var_qg_fp4_dn19: f64,
        var_qg_fp4_dn2: f64,
        var_qg_fp4_dn20: f64,
        var_qg_fp4_dn21: f64,
        var_qg_fp4_dn22: f64,
        var_qg_fp4_dn3: f64,
        var_qg_fp4_dn4: f64,
        var_qg_fp4_dn5: f64,
        var_qg_fp4_dn6: f64,
        var_qg_fp4_dn7: f64,
        var_qg_fp4_dn8: f64,
        var_qg_fp4_dn9: f64,
    ) {
        let (eq187_e2353, eq187_e2353_d_n0, eq187_e2353_d_n1, eq187_e2353_d_n2, eq187_e2353_d_n3, eq187_e2353_d_n4, eq187_e2353_d_n5, eq187_e2353_d_n6, eq187_e2353_d_n7, eq187_e2353_d_n8, eq187_e2353_d_n9, eq187_e2353_d_n12, eq187_e2353_d_n14, eq187_e2353_d_n15, eq187_e2353_d_n16, eq187_e2353_d_n17, eq187_e2353_d_n18, eq187_e2353_d_n19, eq187_e2353_d_n20, eq187_e2353_d_n21, eq187_e2353_d_n22, eq187_e2353_q,) = {
    if (((var_guard561 == 0.0) && (var_guard564 != 0.0)) && (var_guard565 != 0.0)) {
        let eq187_e2350_q: f64 = var_qg_fp3s;
        let eq187_e2351: f64 = (p.p7 * var_qg_fp3s);
        let eq187_e2351_d_n0: f64 = (p.p7 * var_qg_fp3s_dn0);
        let eq187_e2351_d_n1: f64 = (p.p7 * var_qg_fp3s_dn1);
        let eq187_e2351_d_n2: f64 = (p.p7 * var_qg_fp3s_dn2);
        let eq187_e2351_d_n3: f64 = (p.p7 * var_qg_fp3s_dn3);
        let eq187_e2351_d_n4: f64 = (p.p7 * var_qg_fp3s_dn4);
        let eq187_e2351_d_n5: f64 = (p.p7 * var_qg_fp3s_dn5);
        let eq187_e2351_d_n6: f64 = (p.p7 * var_qg_fp3s_dn6);
        let eq187_e2351_d_n7: f64 = (p.p7 * var_qg_fp3s_dn7);
        let eq187_e2351_d_n8: f64 = (p.p7 * var_qg_fp3s_dn8);
        let eq187_e2351_d_n9: f64 = (p.p7 * var_qg_fp3s_dn9);
        let eq187_e2351_d_n12: f64 = (p.p7 * var_qg_fp3s_dn12);
        let eq187_e2351_d_n14: f64 = (p.p7 * var_qg_fp3s_dn14);
        let eq187_e2351_d_n15: f64 = (p.p7 * var_qg_fp3s_dn15);
        let eq187_e2351_d_n16: f64 = (p.p7 * var_qg_fp3s_dn16);
        let eq187_e2351_d_n17: f64 = (p.p7 * var_qg_fp3s_dn17);
        let eq187_e2351_d_n18: f64 = (p.p7 * var_qg_fp3s_dn18);
        let eq187_e2351_d_n19: f64 = (p.p7 * var_qg_fp3s_dn19);
        let eq187_e2351_d_n20: f64 = (p.p7 * var_qg_fp3s_dn20);
        let eq187_e2351_d_n21: f64 = (p.p7 * var_qg_fp3s_dn21);
        let eq187_e2351_d_n22: f64 = (p.p7 * var_qg_fp3s_dn22);
        let eq187_e2351_q: f64 = (p.p7 * eq187_e2350_q);
        (eq187_e2351, eq187_e2351_d_n0, eq187_e2351_d_n1, eq187_e2351_d_n2, eq187_e2351_d_n3, eq187_e2351_d_n4, eq187_e2351_d_n5, eq187_e2351_d_n6, eq187_e2351_d_n7, eq187_e2351_d_n8, eq187_e2351_d_n9, eq187_e2351_d_n12, eq187_e2351_d_n14, eq187_e2351_d_n15, eq187_e2351_d_n16, eq187_e2351_d_n17, eq187_e2351_d_n18, eq187_e2351_d_n19, eq187_e2351_d_n20, eq187_e2351_d_n21, eq187_e2351_d_n22, eq187_e2351_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq187_reactive_node_derivatives: [f64; 23] = [eq187_e2353_d_n0, eq187_e2353_d_n1, eq187_e2353_d_n2, eq187_e2353_d_n3, eq187_e2353_d_n4, eq187_e2353_d_n5, eq187_e2353_d_n6, eq187_e2353_d_n7, eq187_e2353_d_n8, eq187_e2353_d_n9, 0.0, 0.0, eq187_e2353_d_n12, 0.0, eq187_e2353_d_n14, eq187_e2353_d_n15, eq187_e2353_d_n16, eq187_e2353_d_n17, eq187_e2353_d_n18, eq187_e2353_d_n19, eq187_e2353_d_n20, eq187_e2353_d_n21, eq187_e2353_d_n22];
        let eq187_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[2]),
            nodes,
            &eq187_reactive_node_derivatives,
            branches,
            &eq187_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq188_e2367, eq188_e2367_d_n0, eq188_e2367_d_n1, eq188_e2367_d_n2, eq188_e2367_d_n3, eq188_e2367_d_n4, eq188_e2367_d_n5, eq188_e2367_d_n6, eq188_e2367_d_n7, eq188_e2367_d_n8, eq188_e2367_d_n9, eq188_e2367_d_n12, eq188_e2367_d_n14, eq188_e2367_d_n15, eq188_e2367_d_n16, eq188_e2367_d_n17, eq188_e2367_d_n18, eq188_e2367_d_n19, eq188_e2367_d_n20, eq188_e2367_d_n21, eq188_e2367_d_n22, eq188_e2367_q,) = {
    if (((var_guard561 == 0.0) && (var_guard564 != 0.0)) && (var_guard565 != 0.0)) {
        let eq188_e2362_q: f64 = var_qg_fp3s;
        let eq188_e2363: f64 = (p.p7 * var_qg_fp3s);
        let eq188_e2363_d_n0: f64 = (p.p7 * var_qg_fp3s_dn0);
        let eq188_e2363_d_n1: f64 = (p.p7 * var_qg_fp3s_dn1);
        let eq188_e2363_d_n2: f64 = (p.p7 * var_qg_fp3s_dn2);
        let eq188_e2363_d_n3: f64 = (p.p7 * var_qg_fp3s_dn3);
        let eq188_e2363_d_n4: f64 = (p.p7 * var_qg_fp3s_dn4);
        let eq188_e2363_d_n5: f64 = (p.p7 * var_qg_fp3s_dn5);
        let eq188_e2363_d_n6: f64 = (p.p7 * var_qg_fp3s_dn6);
        let eq188_e2363_d_n7: f64 = (p.p7 * var_qg_fp3s_dn7);
        let eq188_e2363_d_n8: f64 = (p.p7 * var_qg_fp3s_dn8);
        let eq188_e2363_d_n9: f64 = (p.p7 * var_qg_fp3s_dn9);
        let eq188_e2363_d_n12: f64 = (p.p7 * var_qg_fp3s_dn12);
        let eq188_e2363_d_n14: f64 = (p.p7 * var_qg_fp3s_dn14);
        let eq188_e2363_d_n15: f64 = (p.p7 * var_qg_fp3s_dn15);
        let eq188_e2363_d_n16: f64 = (p.p7 * var_qg_fp3s_dn16);
        let eq188_e2363_d_n17: f64 = (p.p7 * var_qg_fp3s_dn17);
        let eq188_e2363_d_n18: f64 = (p.p7 * var_qg_fp3s_dn18);
        let eq188_e2363_d_n19: f64 = (p.p7 * var_qg_fp3s_dn19);
        let eq188_e2363_d_n20: f64 = (p.p7 * var_qg_fp3s_dn20);
        let eq188_e2363_d_n21: f64 = (p.p7 * var_qg_fp3s_dn21);
        let eq188_e2363_d_n22: f64 = (p.p7 * var_qg_fp3s_dn22);
        let eq188_e2363_q: f64 = (p.p7 * eq188_e2362_q);
        let eq188_e2365: f64 = (eq188_e2363 * p.p248);
        let eq188_e2365_d_n0: f64 = (eq188_e2363_d_n0 * p.p248);
        let eq188_e2365_d_n1: f64 = (eq188_e2363_d_n1 * p.p248);
        let eq188_e2365_d_n2: f64 = (eq188_e2363_d_n2 * p.p248);
        let eq188_e2365_d_n3: f64 = (eq188_e2363_d_n3 * p.p248);
        let eq188_e2365_d_n4: f64 = (eq188_e2363_d_n4 * p.p248);
        let eq188_e2365_d_n5: f64 = (eq188_e2363_d_n5 * p.p248);
        let eq188_e2365_d_n6: f64 = (eq188_e2363_d_n6 * p.p248);
        let eq188_e2365_d_n7: f64 = (eq188_e2363_d_n7 * p.p248);
        let eq188_e2365_d_n8: f64 = (eq188_e2363_d_n8 * p.p248);
        let eq188_e2365_d_n9: f64 = (eq188_e2363_d_n9 * p.p248);
        let eq188_e2365_d_n12: f64 = (eq188_e2363_d_n12 * p.p248);
        let eq188_e2365_d_n14: f64 = (eq188_e2363_d_n14 * p.p248);
        let eq188_e2365_d_n15: f64 = (eq188_e2363_d_n15 * p.p248);
        let eq188_e2365_d_n16: f64 = (eq188_e2363_d_n16 * p.p248);
        let eq188_e2365_d_n17: f64 = (eq188_e2363_d_n17 * p.p248);
        let eq188_e2365_d_n18: f64 = (eq188_e2363_d_n18 * p.p248);
        let eq188_e2365_d_n19: f64 = (eq188_e2363_d_n19 * p.p248);
        let eq188_e2365_d_n20: f64 = (eq188_e2363_d_n20 * p.p248);
        let eq188_e2365_d_n21: f64 = (eq188_e2363_d_n21 * p.p248);
        let eq188_e2365_d_n22: f64 = (eq188_e2363_d_n22 * p.p248);
        let eq188_e2365_q: f64 = (eq188_e2363_q * p.p248);
        (eq188_e2365, eq188_e2365_d_n0, eq188_e2365_d_n1, eq188_e2365_d_n2, eq188_e2365_d_n3, eq188_e2365_d_n4, eq188_e2365_d_n5, eq188_e2365_d_n6, eq188_e2365_d_n7, eq188_e2365_d_n8, eq188_e2365_d_n9, eq188_e2365_d_n12, eq188_e2365_d_n14, eq188_e2365_d_n15, eq188_e2365_d_n16, eq188_e2365_d_n17, eq188_e2365_d_n18, eq188_e2365_d_n19, eq188_e2365_d_n20, eq188_e2365_d_n21, eq188_e2365_d_n22, eq188_e2365_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq188_reactive_node_derivatives: [f64; 23] = [eq188_e2367_d_n0, eq188_e2367_d_n1, eq188_e2367_d_n2, eq188_e2367_d_n3, eq188_e2367_d_n4, eq188_e2367_d_n5, eq188_e2367_d_n6, eq188_e2367_d_n7, eq188_e2367_d_n8, eq188_e2367_d_n9, 0.0, 0.0, eq188_e2367_d_n12, 0.0, eq188_e2367_d_n14, eq188_e2367_d_n15, eq188_e2367_d_n16, eq188_e2367_d_n17, eq188_e2367_d_n18, eq188_e2367_d_n19, eq188_e2367_d_n20, eq188_e2367_d_n21, eq188_e2367_d_n22];
        let eq188_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            nodes,
            &eq188_reactive_node_derivatives,
            branches,
            &eq188_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq189_e2380, eq189_e2380_d_n0, eq189_e2380_d_n1, eq189_e2380_d_n2, eq189_e2380_d_n3, eq189_e2380_d_n4, eq189_e2380_d_n5, eq189_e2380_d_n6, eq189_e2380_d_n7, eq189_e2380_d_n8, eq189_e2380_d_n9, eq189_e2380_d_n12, eq189_e2380_d_n14, eq189_e2380_d_n15, eq189_e2380_d_n16, eq189_e2380_d_n17, eq189_e2380_d_n18, eq189_e2380_d_n19, eq189_e2380_d_n20, eq189_e2380_d_n21, eq189_e2380_d_n22, eq189_e2380_q,) = {
    if (((var_guard561 == 0.0) && (var_guard564 != 0.0)) && (var_guard565 == 0.0)) {
        let eq189_e2377_q: f64 = var_qg_fp3s;
        let eq189_e2378: f64 = (p.p7 * var_qg_fp3s);
        let eq189_e2378_d_n0: f64 = (p.p7 * var_qg_fp3s_dn0);
        let eq189_e2378_d_n1: f64 = (p.p7 * var_qg_fp3s_dn1);
        let eq189_e2378_d_n2: f64 = (p.p7 * var_qg_fp3s_dn2);
        let eq189_e2378_d_n3: f64 = (p.p7 * var_qg_fp3s_dn3);
        let eq189_e2378_d_n4: f64 = (p.p7 * var_qg_fp3s_dn4);
        let eq189_e2378_d_n5: f64 = (p.p7 * var_qg_fp3s_dn5);
        let eq189_e2378_d_n6: f64 = (p.p7 * var_qg_fp3s_dn6);
        let eq189_e2378_d_n7: f64 = (p.p7 * var_qg_fp3s_dn7);
        let eq189_e2378_d_n8: f64 = (p.p7 * var_qg_fp3s_dn8);
        let eq189_e2378_d_n9: f64 = (p.p7 * var_qg_fp3s_dn9);
        let eq189_e2378_d_n12: f64 = (p.p7 * var_qg_fp3s_dn12);
        let eq189_e2378_d_n14: f64 = (p.p7 * var_qg_fp3s_dn14);
        let eq189_e2378_d_n15: f64 = (p.p7 * var_qg_fp3s_dn15);
        let eq189_e2378_d_n16: f64 = (p.p7 * var_qg_fp3s_dn16);
        let eq189_e2378_d_n17: f64 = (p.p7 * var_qg_fp3s_dn17);
        let eq189_e2378_d_n18: f64 = (p.p7 * var_qg_fp3s_dn18);
        let eq189_e2378_d_n19: f64 = (p.p7 * var_qg_fp3s_dn19);
        let eq189_e2378_d_n20: f64 = (p.p7 * var_qg_fp3s_dn20);
        let eq189_e2378_d_n21: f64 = (p.p7 * var_qg_fp3s_dn21);
        let eq189_e2378_d_n22: f64 = (p.p7 * var_qg_fp3s_dn22);
        let eq189_e2378_q: f64 = (p.p7 * eq189_e2377_q);
        (eq189_e2378, eq189_e2378_d_n0, eq189_e2378_d_n1, eq189_e2378_d_n2, eq189_e2378_d_n3, eq189_e2378_d_n4, eq189_e2378_d_n5, eq189_e2378_d_n6, eq189_e2378_d_n7, eq189_e2378_d_n8, eq189_e2378_d_n9, eq189_e2378_d_n12, eq189_e2378_d_n14, eq189_e2378_d_n15, eq189_e2378_d_n16, eq189_e2378_d_n17, eq189_e2378_d_n18, eq189_e2378_d_n19, eq189_e2378_d_n20, eq189_e2378_d_n21, eq189_e2378_d_n22, eq189_e2378_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq189_reactive_node_derivatives: [f64; 23] = [eq189_e2380_d_n0, eq189_e2380_d_n1, eq189_e2380_d_n2, eq189_e2380_d_n3, eq189_e2380_d_n4, eq189_e2380_d_n5, eq189_e2380_d_n6, eq189_e2380_d_n7, eq189_e2380_d_n8, eq189_e2380_d_n9, 0.0, 0.0, eq189_e2380_d_n12, 0.0, eq189_e2380_d_n14, eq189_e2380_d_n15, eq189_e2380_d_n16, eq189_e2380_d_n17, eq189_e2380_d_n18, eq189_e2380_d_n19, eq189_e2380_d_n20, eq189_e2380_d_n21, eq189_e2380_d_n22];
        let eq189_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            nodes,
            &eq189_reactive_node_derivatives,
            branches,
            &eq189_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq190_e2395, eq190_e2395_d_n0, eq190_e2395_d_n1, eq190_e2395_d_n2, eq190_e2395_d_n3, eq190_e2395_d_n4, eq190_e2395_d_n5, eq190_e2395_d_n6, eq190_e2395_d_n7, eq190_e2395_d_n8, eq190_e2395_d_n9, eq190_e2395_d_n12, eq190_e2395_d_n14, eq190_e2395_d_n15, eq190_e2395_d_n16, eq190_e2395_d_n17, eq190_e2395_d_n18, eq190_e2395_d_n19, eq190_e2395_d_n20, eq190_e2395_d_n21, eq190_e2395_d_n22, eq190_e2395_q,) = {
    if (((var_guard561 == 0.0) && (var_guard564 != 0.0)) && (var_guard565 == 0.0)) {
        let eq190_e2390_q: f64 = var_qg_fp3s;
        let eq190_e2391: f64 = (p.p7 * var_qg_fp3s);
        let eq190_e2391_d_n0: f64 = (p.p7 * var_qg_fp3s_dn0);
        let eq190_e2391_d_n1: f64 = (p.p7 * var_qg_fp3s_dn1);
        let eq190_e2391_d_n2: f64 = (p.p7 * var_qg_fp3s_dn2);
        let eq190_e2391_d_n3: f64 = (p.p7 * var_qg_fp3s_dn3);
        let eq190_e2391_d_n4: f64 = (p.p7 * var_qg_fp3s_dn4);
        let eq190_e2391_d_n5: f64 = (p.p7 * var_qg_fp3s_dn5);
        let eq190_e2391_d_n6: f64 = (p.p7 * var_qg_fp3s_dn6);
        let eq190_e2391_d_n7: f64 = (p.p7 * var_qg_fp3s_dn7);
        let eq190_e2391_d_n8: f64 = (p.p7 * var_qg_fp3s_dn8);
        let eq190_e2391_d_n9: f64 = (p.p7 * var_qg_fp3s_dn9);
        let eq190_e2391_d_n12: f64 = (p.p7 * var_qg_fp3s_dn12);
        let eq190_e2391_d_n14: f64 = (p.p7 * var_qg_fp3s_dn14);
        let eq190_e2391_d_n15: f64 = (p.p7 * var_qg_fp3s_dn15);
        let eq190_e2391_d_n16: f64 = (p.p7 * var_qg_fp3s_dn16);
        let eq190_e2391_d_n17: f64 = (p.p7 * var_qg_fp3s_dn17);
        let eq190_e2391_d_n18: f64 = (p.p7 * var_qg_fp3s_dn18);
        let eq190_e2391_d_n19: f64 = (p.p7 * var_qg_fp3s_dn19);
        let eq190_e2391_d_n20: f64 = (p.p7 * var_qg_fp3s_dn20);
        let eq190_e2391_d_n21: f64 = (p.p7 * var_qg_fp3s_dn21);
        let eq190_e2391_d_n22: f64 = (p.p7 * var_qg_fp3s_dn22);
        let eq190_e2391_q: f64 = (p.p7 * eq190_e2390_q);
        let eq190_e2393: f64 = (eq190_e2391 * p.p248);
        let eq190_e2393_d_n0: f64 = (eq190_e2391_d_n0 * p.p248);
        let eq190_e2393_d_n1: f64 = (eq190_e2391_d_n1 * p.p248);
        let eq190_e2393_d_n2: f64 = (eq190_e2391_d_n2 * p.p248);
        let eq190_e2393_d_n3: f64 = (eq190_e2391_d_n3 * p.p248);
        let eq190_e2393_d_n4: f64 = (eq190_e2391_d_n4 * p.p248);
        let eq190_e2393_d_n5: f64 = (eq190_e2391_d_n5 * p.p248);
        let eq190_e2393_d_n6: f64 = (eq190_e2391_d_n6 * p.p248);
        let eq190_e2393_d_n7: f64 = (eq190_e2391_d_n7 * p.p248);
        let eq190_e2393_d_n8: f64 = (eq190_e2391_d_n8 * p.p248);
        let eq190_e2393_d_n9: f64 = (eq190_e2391_d_n9 * p.p248);
        let eq190_e2393_d_n12: f64 = (eq190_e2391_d_n12 * p.p248);
        let eq190_e2393_d_n14: f64 = (eq190_e2391_d_n14 * p.p248);
        let eq190_e2393_d_n15: f64 = (eq190_e2391_d_n15 * p.p248);
        let eq190_e2393_d_n16: f64 = (eq190_e2391_d_n16 * p.p248);
        let eq190_e2393_d_n17: f64 = (eq190_e2391_d_n17 * p.p248);
        let eq190_e2393_d_n18: f64 = (eq190_e2391_d_n18 * p.p248);
        let eq190_e2393_d_n19: f64 = (eq190_e2391_d_n19 * p.p248);
        let eq190_e2393_d_n20: f64 = (eq190_e2391_d_n20 * p.p248);
        let eq190_e2393_d_n21: f64 = (eq190_e2391_d_n21 * p.p248);
        let eq190_e2393_d_n22: f64 = (eq190_e2391_d_n22 * p.p248);
        let eq190_e2393_q: f64 = (eq190_e2391_q * p.p248);
        (eq190_e2393, eq190_e2393_d_n0, eq190_e2393_d_n1, eq190_e2393_d_n2, eq190_e2393_d_n3, eq190_e2393_d_n4, eq190_e2393_d_n5, eq190_e2393_d_n6, eq190_e2393_d_n7, eq190_e2393_d_n8, eq190_e2393_d_n9, eq190_e2393_d_n12, eq190_e2393_d_n14, eq190_e2393_d_n15, eq190_e2393_d_n16, eq190_e2393_d_n17, eq190_e2393_d_n18, eq190_e2393_d_n19, eq190_e2393_d_n20, eq190_e2393_d_n21, eq190_e2393_d_n22, eq190_e2393_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq190_reactive_node_derivatives: [f64; 23] = [eq190_e2395_d_n0, eq190_e2395_d_n1, eq190_e2395_d_n2, eq190_e2395_d_n3, eq190_e2395_d_n4, eq190_e2395_d_n5, eq190_e2395_d_n6, eq190_e2395_d_n7, eq190_e2395_d_n8, eq190_e2395_d_n9, 0.0, 0.0, eq190_e2395_d_n12, 0.0, eq190_e2395_d_n14, eq190_e2395_d_n15, eq190_e2395_d_n16, eq190_e2395_d_n17, eq190_e2395_d_n18, eq190_e2395_d_n19, eq190_e2395_d_n20, eq190_e2395_d_n21, eq190_e2395_d_n22];
        let eq190_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq190_reactive_node_derivatives,
            branches,
            &eq190_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq191_e2407, eq191_e2407_d_n0, eq191_e2407_d_n1, eq191_e2407_d_n2, eq191_e2407_d_n3, eq191_e2407_d_n4, eq191_e2407_d_n5, eq191_e2407_d_n6, eq191_e2407_d_n7, eq191_e2407_d_n8, eq191_e2407_d_n9, eq191_e2407_d_n12, eq191_e2407_d_n14, eq191_e2407_d_n15, eq191_e2407_d_n16, eq191_e2407_d_n17, eq191_e2407_d_n18, eq191_e2407_d_n19, eq191_e2407_d_n20, eq191_e2407_d_n21, eq191_e2407_d_n22, eq191_e2407_q,) = {
    if ((var_guard561 == 0.0) && (var_guard564 != 0.0)) {
        let eq191_e2403: f64 = (p.p253 * var_qg_fp3s);
        let eq191_e2403_d_n0: f64 = (p.p253 * var_qg_fp3s_dn0);
        let eq191_e2403_d_n1: f64 = (p.p253 * var_qg_fp3s_dn1);
        let eq191_e2403_d_n2: f64 = (p.p253 * var_qg_fp3s_dn2);
        let eq191_e2403_d_n3: f64 = (p.p253 * var_qg_fp3s_dn3);
        let eq191_e2403_d_n4: f64 = (p.p253 * var_qg_fp3s_dn4);
        let eq191_e2403_d_n5: f64 = (p.p253 * var_qg_fp3s_dn5);
        let eq191_e2403_d_n6: f64 = (p.p253 * var_qg_fp3s_dn6);
        let eq191_e2403_d_n7: f64 = (p.p253 * var_qg_fp3s_dn7);
        let eq191_e2403_d_n8: f64 = (p.p253 * var_qg_fp3s_dn8);
        let eq191_e2403_d_n9: f64 = (p.p253 * var_qg_fp3s_dn9);
        let eq191_e2403_d_n12: f64 = (p.p253 * var_qg_fp3s_dn12);
        let eq191_e2403_d_n14: f64 = (p.p253 * var_qg_fp3s_dn14);
        let eq191_e2403_d_n15: f64 = (p.p253 * var_qg_fp3s_dn15);
        let eq191_e2403_d_n16: f64 = (p.p253 * var_qg_fp3s_dn16);
        let eq191_e2403_d_n17: f64 = (p.p253 * var_qg_fp3s_dn17);
        let eq191_e2403_d_n18: f64 = (p.p253 * var_qg_fp3s_dn18);
        let eq191_e2403_d_n19: f64 = (p.p253 * var_qg_fp3s_dn19);
        let eq191_e2403_d_n20: f64 = (p.p253 * var_qg_fp3s_dn20);
        let eq191_e2403_d_n21: f64 = (p.p253 * var_qg_fp3s_dn21);
        let eq191_e2403_d_n22: f64 = (p.p253 * var_qg_fp3s_dn22);
        let eq191_e2404_q: f64 = eq191_e2403;
        let eq191_e2405: f64 = (p.p7 * eq191_e2403);
        let eq191_e2405_d_n0: f64 = (p.p7 * eq191_e2403_d_n0);
        let eq191_e2405_d_n1: f64 = (p.p7 * eq191_e2403_d_n1);
        let eq191_e2405_d_n2: f64 = (p.p7 * eq191_e2403_d_n2);
        let eq191_e2405_d_n3: f64 = (p.p7 * eq191_e2403_d_n3);
        let eq191_e2405_d_n4: f64 = (p.p7 * eq191_e2403_d_n4);
        let eq191_e2405_d_n5: f64 = (p.p7 * eq191_e2403_d_n5);
        let eq191_e2405_d_n6: f64 = (p.p7 * eq191_e2403_d_n6);
        let eq191_e2405_d_n7: f64 = (p.p7 * eq191_e2403_d_n7);
        let eq191_e2405_d_n8: f64 = (p.p7 * eq191_e2403_d_n8);
        let eq191_e2405_d_n9: f64 = (p.p7 * eq191_e2403_d_n9);
        let eq191_e2405_d_n12: f64 = (p.p7 * eq191_e2403_d_n12);
        let eq191_e2405_d_n14: f64 = (p.p7 * eq191_e2403_d_n14);
        let eq191_e2405_d_n15: f64 = (p.p7 * eq191_e2403_d_n15);
        let eq191_e2405_d_n16: f64 = (p.p7 * eq191_e2403_d_n16);
        let eq191_e2405_d_n17: f64 = (p.p7 * eq191_e2403_d_n17);
        let eq191_e2405_d_n18: f64 = (p.p7 * eq191_e2403_d_n18);
        let eq191_e2405_d_n19: f64 = (p.p7 * eq191_e2403_d_n19);
        let eq191_e2405_d_n20: f64 = (p.p7 * eq191_e2403_d_n20);
        let eq191_e2405_d_n21: f64 = (p.p7 * eq191_e2403_d_n21);
        let eq191_e2405_d_n22: f64 = (p.p7 * eq191_e2403_d_n22);
        let eq191_e2405_q: f64 = (p.p7 * eq191_e2404_q);
        (eq191_e2405, eq191_e2405_d_n0, eq191_e2405_d_n1, eq191_e2405_d_n2, eq191_e2405_d_n3, eq191_e2405_d_n4, eq191_e2405_d_n5, eq191_e2405_d_n6, eq191_e2405_d_n7, eq191_e2405_d_n8, eq191_e2405_d_n9, eq191_e2405_d_n12, eq191_e2405_d_n14, eq191_e2405_d_n15, eq191_e2405_d_n16, eq191_e2405_d_n17, eq191_e2405_d_n18, eq191_e2405_d_n19, eq191_e2405_d_n20, eq191_e2405_d_n21, eq191_e2405_d_n22, eq191_e2405_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq191_reactive_node_derivatives: [f64; 23] = [eq191_e2407_d_n0, eq191_e2407_d_n1, eq191_e2407_d_n2, eq191_e2407_d_n3, eq191_e2407_d_n4, eq191_e2407_d_n5, eq191_e2407_d_n6, eq191_e2407_d_n7, eq191_e2407_d_n8, eq191_e2407_d_n9, 0.0, 0.0, eq191_e2407_d_n12, 0.0, eq191_e2407_d_n14, eq191_e2407_d_n15, eq191_e2407_d_n16, eq191_e2407_d_n17, eq191_e2407_d_n18, eq191_e2407_d_n19, eq191_e2407_d_n20, eq191_e2407_d_n21, eq191_e2407_d_n22];
        let eq191_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            nodes,
            &eq191_reactive_node_derivatives,
            branches,
            &eq191_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq192_e2416, eq192_e2416_d_n0, eq192_e2416_d_n1, eq192_e2416_d_n2, eq192_e2416_d_n3, eq192_e2416_d_n4, eq192_e2416_d_n5, eq192_e2416_d_n6, eq192_e2416_d_n7, eq192_e2416_d_n8, eq192_e2416_d_n9, eq192_e2416_d_n12, eq192_e2416_d_n14, eq192_e2416_d_n15, eq192_e2416_d_n16, eq192_e2416_d_n17, eq192_e2416_d_n18, eq192_e2416_d_n19, eq192_e2416_d_n20, eq192_e2416_d_n21, eq192_e2416_d_n22, eq192_e2416_q,) = {
    if ((var_guard566 != 0.0) && (var_guard567 != 0.0)) {
        let eq192_e2413_q: f64 = var_qd_fp4;
        let eq192_e2414: f64 = (p.p7 * var_qd_fp4);
        let eq192_e2414_d_n0: f64 = (p.p7 * var_qd_fp4_dn0);
        let eq192_e2414_d_n1: f64 = (p.p7 * var_qd_fp4_dn1);
        let eq192_e2414_d_n2: f64 = (p.p7 * var_qd_fp4_dn2);
        let eq192_e2414_d_n3: f64 = (p.p7 * var_qd_fp4_dn3);
        let eq192_e2414_d_n4: f64 = (p.p7 * var_qd_fp4_dn4);
        let eq192_e2414_d_n5: f64 = (p.p7 * var_qd_fp4_dn5);
        let eq192_e2414_d_n6: f64 = (p.p7 * var_qd_fp4_dn6);
        let eq192_e2414_d_n7: f64 = (p.p7 * var_qd_fp4_dn7);
        let eq192_e2414_d_n8: f64 = (p.p7 * var_qd_fp4_dn8);
        let eq192_e2414_d_n9: f64 = (p.p7 * var_qd_fp4_dn9);
        let eq192_e2414_d_n12: f64 = (p.p7 * var_qd_fp4_dn12);
        let eq192_e2414_d_n14: f64 = (p.p7 * var_qd_fp4_dn14);
        let eq192_e2414_d_n15: f64 = (p.p7 * var_qd_fp4_dn15);
        let eq192_e2414_d_n16: f64 = (p.p7 * var_qd_fp4_dn16);
        let eq192_e2414_d_n17: f64 = (p.p7 * var_qd_fp4_dn17);
        let eq192_e2414_d_n18: f64 = (p.p7 * var_qd_fp4_dn18);
        let eq192_e2414_d_n19: f64 = (p.p7 * var_qd_fp4_dn19);
        let eq192_e2414_d_n20: f64 = (p.p7 * var_qd_fp4_dn20);
        let eq192_e2414_d_n21: f64 = (p.p7 * var_qd_fp4_dn21);
        let eq192_e2414_d_n22: f64 = (p.p7 * var_qd_fp4_dn22);
        let eq192_e2414_q: f64 = (p.p7 * eq192_e2413_q);
        (eq192_e2414, eq192_e2414_d_n0, eq192_e2414_d_n1, eq192_e2414_d_n2, eq192_e2414_d_n3, eq192_e2414_d_n4, eq192_e2414_d_n5, eq192_e2414_d_n6, eq192_e2414_d_n7, eq192_e2414_d_n8, eq192_e2414_d_n9, eq192_e2414_d_n12, eq192_e2414_d_n14, eq192_e2414_d_n15, eq192_e2414_d_n16, eq192_e2414_d_n17, eq192_e2414_d_n18, eq192_e2414_d_n19, eq192_e2414_d_n20, eq192_e2414_d_n21, eq192_e2414_d_n22, eq192_e2414_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq192_reactive_node_derivatives: [f64; 23] = [eq192_e2416_d_n0, eq192_e2416_d_n1, eq192_e2416_d_n2, eq192_e2416_d_n3, eq192_e2416_d_n4, eq192_e2416_d_n5, eq192_e2416_d_n6, eq192_e2416_d_n7, eq192_e2416_d_n8, eq192_e2416_d_n9, 0.0, 0.0, eq192_e2416_d_n12, 0.0, eq192_e2416_d_n14, eq192_e2416_d_n15, eq192_e2416_d_n16, eq192_e2416_d_n17, eq192_e2416_d_n18, eq192_e2416_d_n19, eq192_e2416_d_n20, eq192_e2416_d_n21, eq192_e2416_d_n22];
        let eq192_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[18]),
            Some(nodes[17]),
            nodes,
            &eq192_reactive_node_derivatives,
            branches,
            &eq192_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq193_e2427, eq193_e2427_d_n0, eq193_e2427_d_n1, eq193_e2427_d_n2, eq193_e2427_d_n3, eq193_e2427_d_n4, eq193_e2427_d_n5, eq193_e2427_d_n6, eq193_e2427_d_n7, eq193_e2427_d_n8, eq193_e2427_d_n9, eq193_e2427_d_n12, eq193_e2427_d_n14, eq193_e2427_d_n15, eq193_e2427_d_n16, eq193_e2427_d_n17, eq193_e2427_d_n18, eq193_e2427_d_n19, eq193_e2427_d_n20, eq193_e2427_d_n21, eq193_e2427_d_n22, eq193_e2427_q,) = {
    if (((var_guard566 != 0.0) && (var_guard567 != 0.0)) && (var_guard568 != 0.0)) {
        let eq193_e2424_q: f64 = var_qg_fp4;
        let eq193_e2425: f64 = (p.p7 * var_qg_fp4);
        let eq193_e2425_d_n0: f64 = (p.p7 * var_qg_fp4_dn0);
        let eq193_e2425_d_n1: f64 = (p.p7 * var_qg_fp4_dn1);
        let eq193_e2425_d_n2: f64 = (p.p7 * var_qg_fp4_dn2);
        let eq193_e2425_d_n3: f64 = (p.p7 * var_qg_fp4_dn3);
        let eq193_e2425_d_n4: f64 = (p.p7 * var_qg_fp4_dn4);
        let eq193_e2425_d_n5: f64 = (p.p7 * var_qg_fp4_dn5);
        let eq193_e2425_d_n6: f64 = (p.p7 * var_qg_fp4_dn6);
        let eq193_e2425_d_n7: f64 = (p.p7 * var_qg_fp4_dn7);
        let eq193_e2425_d_n8: f64 = (p.p7 * var_qg_fp4_dn8);
        let eq193_e2425_d_n9: f64 = (p.p7 * var_qg_fp4_dn9);
        let eq193_e2425_d_n12: f64 = (p.p7 * var_qg_fp4_dn12);
        let eq193_e2425_d_n14: f64 = (p.p7 * var_qg_fp4_dn14);
        let eq193_e2425_d_n15: f64 = (p.p7 * var_qg_fp4_dn15);
        let eq193_e2425_d_n16: f64 = (p.p7 * var_qg_fp4_dn16);
        let eq193_e2425_d_n17: f64 = (p.p7 * var_qg_fp4_dn17);
        let eq193_e2425_d_n18: f64 = (p.p7 * var_qg_fp4_dn18);
        let eq193_e2425_d_n19: f64 = (p.p7 * var_qg_fp4_dn19);
        let eq193_e2425_d_n20: f64 = (p.p7 * var_qg_fp4_dn20);
        let eq193_e2425_d_n21: f64 = (p.p7 * var_qg_fp4_dn21);
        let eq193_e2425_d_n22: f64 = (p.p7 * var_qg_fp4_dn22);
        let eq193_e2425_q: f64 = (p.p7 * eq193_e2424_q);
        (eq193_e2425, eq193_e2425_d_n0, eq193_e2425_d_n1, eq193_e2425_d_n2, eq193_e2425_d_n3, eq193_e2425_d_n4, eq193_e2425_d_n5, eq193_e2425_d_n6, eq193_e2425_d_n7, eq193_e2425_d_n8, eq193_e2425_d_n9, eq193_e2425_d_n12, eq193_e2425_d_n14, eq193_e2425_d_n15, eq193_e2425_d_n16, eq193_e2425_d_n17, eq193_e2425_d_n18, eq193_e2425_d_n19, eq193_e2425_d_n20, eq193_e2425_d_n21, eq193_e2425_d_n22, eq193_e2425_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq193_reactive_node_derivatives: [f64; 23] = [eq193_e2427_d_n0, eq193_e2427_d_n1, eq193_e2427_d_n2, eq193_e2427_d_n3, eq193_e2427_d_n4, eq193_e2427_d_n5, eq193_e2427_d_n6, eq193_e2427_d_n7, eq193_e2427_d_n8, eq193_e2427_d_n9, 0.0, 0.0, eq193_e2427_d_n12, 0.0, eq193_e2427_d_n14, eq193_e2427_d_n15, eq193_e2427_d_n16, eq193_e2427_d_n17, eq193_e2427_d_n18, eq193_e2427_d_n19, eq193_e2427_d_n20, eq193_e2427_d_n21, eq193_e2427_d_n22];
        let eq193_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[17]),
            nodes,
            &eq193_reactive_node_derivatives,
            branches,
            &eq193_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq194_e2440, eq194_e2440_d_n0, eq194_e2440_d_n1, eq194_e2440_d_n2, eq194_e2440_d_n3, eq194_e2440_d_n4, eq194_e2440_d_n5, eq194_e2440_d_n6, eq194_e2440_d_n7, eq194_e2440_d_n8, eq194_e2440_d_n9, eq194_e2440_d_n12, eq194_e2440_d_n14, eq194_e2440_d_n15, eq194_e2440_d_n16, eq194_e2440_d_n17, eq194_e2440_d_n18, eq194_e2440_d_n19, eq194_e2440_d_n20, eq194_e2440_d_n21, eq194_e2440_d_n22, eq194_e2440_q,) = {
    if (((var_guard566 != 0.0) && (var_guard567 != 0.0)) && (var_guard568 != 0.0)) {
        let eq194_e2435_q: f64 = var_qg_fp4;
        let eq194_e2436: f64 = (p.p7 * var_qg_fp4);
        let eq194_e2436_d_n0: f64 = (p.p7 * var_qg_fp4_dn0);
        let eq194_e2436_d_n1: f64 = (p.p7 * var_qg_fp4_dn1);
        let eq194_e2436_d_n2: f64 = (p.p7 * var_qg_fp4_dn2);
        let eq194_e2436_d_n3: f64 = (p.p7 * var_qg_fp4_dn3);
        let eq194_e2436_d_n4: f64 = (p.p7 * var_qg_fp4_dn4);
        let eq194_e2436_d_n5: f64 = (p.p7 * var_qg_fp4_dn5);
        let eq194_e2436_d_n6: f64 = (p.p7 * var_qg_fp4_dn6);
        let eq194_e2436_d_n7: f64 = (p.p7 * var_qg_fp4_dn7);
        let eq194_e2436_d_n8: f64 = (p.p7 * var_qg_fp4_dn8);
        let eq194_e2436_d_n9: f64 = (p.p7 * var_qg_fp4_dn9);
        let eq194_e2436_d_n12: f64 = (p.p7 * var_qg_fp4_dn12);
        let eq194_e2436_d_n14: f64 = (p.p7 * var_qg_fp4_dn14);
        let eq194_e2436_d_n15: f64 = (p.p7 * var_qg_fp4_dn15);
        let eq194_e2436_d_n16: f64 = (p.p7 * var_qg_fp4_dn16);
        let eq194_e2436_d_n17: f64 = (p.p7 * var_qg_fp4_dn17);
        let eq194_e2436_d_n18: f64 = (p.p7 * var_qg_fp4_dn18);
        let eq194_e2436_d_n19: f64 = (p.p7 * var_qg_fp4_dn19);
        let eq194_e2436_d_n20: f64 = (p.p7 * var_qg_fp4_dn20);
        let eq194_e2436_d_n21: f64 = (p.p7 * var_qg_fp4_dn21);
        let eq194_e2436_d_n22: f64 = (p.p7 * var_qg_fp4_dn22);
        let eq194_e2436_q: f64 = (p.p7 * eq194_e2435_q);
        let eq194_e2438: f64 = (eq194_e2436 * p.p249);
        let eq194_e2438_d_n0: f64 = (eq194_e2436_d_n0 * p.p249);
        let eq194_e2438_d_n1: f64 = (eq194_e2436_d_n1 * p.p249);
        let eq194_e2438_d_n2: f64 = (eq194_e2436_d_n2 * p.p249);
        let eq194_e2438_d_n3: f64 = (eq194_e2436_d_n3 * p.p249);
        let eq194_e2438_d_n4: f64 = (eq194_e2436_d_n4 * p.p249);
        let eq194_e2438_d_n5: f64 = (eq194_e2436_d_n5 * p.p249);
        let eq194_e2438_d_n6: f64 = (eq194_e2436_d_n6 * p.p249);
        let eq194_e2438_d_n7: f64 = (eq194_e2436_d_n7 * p.p249);
        let eq194_e2438_d_n8: f64 = (eq194_e2436_d_n8 * p.p249);
        let eq194_e2438_d_n9: f64 = (eq194_e2436_d_n9 * p.p249);
        let eq194_e2438_d_n12: f64 = (eq194_e2436_d_n12 * p.p249);
        let eq194_e2438_d_n14: f64 = (eq194_e2436_d_n14 * p.p249);
        let eq194_e2438_d_n15: f64 = (eq194_e2436_d_n15 * p.p249);
        let eq194_e2438_d_n16: f64 = (eq194_e2436_d_n16 * p.p249);
        let eq194_e2438_d_n17: f64 = (eq194_e2436_d_n17 * p.p249);
        let eq194_e2438_d_n18: f64 = (eq194_e2436_d_n18 * p.p249);
        let eq194_e2438_d_n19: f64 = (eq194_e2436_d_n19 * p.p249);
        let eq194_e2438_d_n20: f64 = (eq194_e2436_d_n20 * p.p249);
        let eq194_e2438_d_n21: f64 = (eq194_e2436_d_n21 * p.p249);
        let eq194_e2438_d_n22: f64 = (eq194_e2436_d_n22 * p.p249);
        let eq194_e2438_q: f64 = (eq194_e2436_q * p.p249);
        (eq194_e2438, eq194_e2438_d_n0, eq194_e2438_d_n1, eq194_e2438_d_n2, eq194_e2438_d_n3, eq194_e2438_d_n4, eq194_e2438_d_n5, eq194_e2438_d_n6, eq194_e2438_d_n7, eq194_e2438_d_n8, eq194_e2438_d_n9, eq194_e2438_d_n12, eq194_e2438_d_n14, eq194_e2438_d_n15, eq194_e2438_d_n16, eq194_e2438_d_n17, eq194_e2438_d_n18, eq194_e2438_d_n19, eq194_e2438_d_n20, eq194_e2438_d_n21, eq194_e2438_d_n22, eq194_e2438_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq194_reactive_node_derivatives: [f64; 23] = [eq194_e2440_d_n0, eq194_e2440_d_n1, eq194_e2440_d_n2, eq194_e2440_d_n3, eq194_e2440_d_n4, eq194_e2440_d_n5, eq194_e2440_d_n6, eq194_e2440_d_n7, eq194_e2440_d_n8, eq194_e2440_d_n9, 0.0, 0.0, eq194_e2440_d_n12, 0.0, eq194_e2440_d_n14, eq194_e2440_d_n15, eq194_e2440_d_n16, eq194_e2440_d_n17, eq194_e2440_d_n18, eq194_e2440_d_n19, eq194_e2440_d_n20, eq194_e2440_d_n21, eq194_e2440_d_n22];
        let eq194_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[17]),
            nodes,
            &eq194_reactive_node_derivatives,
            branches,
            &eq194_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq195_e2452, eq195_e2452_d_n0, eq195_e2452_d_n1, eq195_e2452_d_n2, eq195_e2452_d_n3, eq195_e2452_d_n4, eq195_e2452_d_n5, eq195_e2452_d_n6, eq195_e2452_d_n7, eq195_e2452_d_n8, eq195_e2452_d_n9, eq195_e2452_d_n12, eq195_e2452_d_n14, eq195_e2452_d_n15, eq195_e2452_d_n16, eq195_e2452_d_n17, eq195_e2452_d_n18, eq195_e2452_d_n19, eq195_e2452_d_n20, eq195_e2452_d_n21, eq195_e2452_d_n22, eq195_e2452_q,) = {
    if (((var_guard566 != 0.0) && (var_guard567 != 0.0)) && (var_guard568 == 0.0)) {
        let eq195_e2449_q: f64 = var_qg_fp4;
        let eq195_e2450: f64 = (p.p7 * var_qg_fp4);
        let eq195_e2450_d_n0: f64 = (p.p7 * var_qg_fp4_dn0);
        let eq195_e2450_d_n1: f64 = (p.p7 * var_qg_fp4_dn1);
        let eq195_e2450_d_n2: f64 = (p.p7 * var_qg_fp4_dn2);
        let eq195_e2450_d_n3: f64 = (p.p7 * var_qg_fp4_dn3);
        let eq195_e2450_d_n4: f64 = (p.p7 * var_qg_fp4_dn4);
        let eq195_e2450_d_n5: f64 = (p.p7 * var_qg_fp4_dn5);
        let eq195_e2450_d_n6: f64 = (p.p7 * var_qg_fp4_dn6);
        let eq195_e2450_d_n7: f64 = (p.p7 * var_qg_fp4_dn7);
        let eq195_e2450_d_n8: f64 = (p.p7 * var_qg_fp4_dn8);
        let eq195_e2450_d_n9: f64 = (p.p7 * var_qg_fp4_dn9);
        let eq195_e2450_d_n12: f64 = (p.p7 * var_qg_fp4_dn12);
        let eq195_e2450_d_n14: f64 = (p.p7 * var_qg_fp4_dn14);
        let eq195_e2450_d_n15: f64 = (p.p7 * var_qg_fp4_dn15);
        let eq195_e2450_d_n16: f64 = (p.p7 * var_qg_fp4_dn16);
        let eq195_e2450_d_n17: f64 = (p.p7 * var_qg_fp4_dn17);
        let eq195_e2450_d_n18: f64 = (p.p7 * var_qg_fp4_dn18);
        let eq195_e2450_d_n19: f64 = (p.p7 * var_qg_fp4_dn19);
        let eq195_e2450_d_n20: f64 = (p.p7 * var_qg_fp4_dn20);
        let eq195_e2450_d_n21: f64 = (p.p7 * var_qg_fp4_dn21);
        let eq195_e2450_d_n22: f64 = (p.p7 * var_qg_fp4_dn22);
        let eq195_e2450_q: f64 = (p.p7 * eq195_e2449_q);
        (eq195_e2450, eq195_e2450_d_n0, eq195_e2450_d_n1, eq195_e2450_d_n2, eq195_e2450_d_n3, eq195_e2450_d_n4, eq195_e2450_d_n5, eq195_e2450_d_n6, eq195_e2450_d_n7, eq195_e2450_d_n8, eq195_e2450_d_n9, eq195_e2450_d_n12, eq195_e2450_d_n14, eq195_e2450_d_n15, eq195_e2450_d_n16, eq195_e2450_d_n17, eq195_e2450_d_n18, eq195_e2450_d_n19, eq195_e2450_d_n20, eq195_e2450_d_n21, eq195_e2450_d_n22, eq195_e2450_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq195_reactive_node_derivatives: [f64; 23] = [eq195_e2452_d_n0, eq195_e2452_d_n1, eq195_e2452_d_n2, eq195_e2452_d_n3, eq195_e2452_d_n4, eq195_e2452_d_n5, eq195_e2452_d_n6, eq195_e2452_d_n7, eq195_e2452_d_n8, eq195_e2452_d_n9, 0.0, 0.0, eq195_e2452_d_n12, 0.0, eq195_e2452_d_n14, eq195_e2452_d_n15, eq195_e2452_d_n16, eq195_e2452_d_n17, eq195_e2452_d_n18, eq195_e2452_d_n19, eq195_e2452_d_n20, eq195_e2452_d_n21, eq195_e2452_d_n22];
        let eq195_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[17]),
            nodes,
            &eq195_reactive_node_derivatives,
            branches,
            &eq195_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_9(
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_guard566: f64,
        var_guard567: f64,
        var_guard568: f64,
        var_guard569: f64,
        var_guard570: f64,
        var_guard571: f64,
        var_guard572: f64,
        var_qd_fp4: f64,
        var_qd_fp4_dn0: f64,
        var_qd_fp4_dn1: f64,
        var_qd_fp4_dn12: f64,
        var_qd_fp4_dn14: f64,
        var_qd_fp4_dn15: f64,
        var_qd_fp4_dn16: f64,
        var_qd_fp4_dn17: f64,
        var_qd_fp4_dn18: f64,
        var_qd_fp4_dn19: f64,
        var_qd_fp4_dn2: f64,
        var_qd_fp4_dn20: f64,
        var_qd_fp4_dn21: f64,
        var_qd_fp4_dn22: f64,
        var_qd_fp4_dn3: f64,
        var_qd_fp4_dn4: f64,
        var_qd_fp4_dn5: f64,
        var_qd_fp4_dn6: f64,
        var_qd_fp4_dn7: f64,
        var_qd_fp4_dn8: f64,
        var_qd_fp4_dn9: f64,
        var_qd_fp4s: f64,
        var_qd_fp4s_dn0: f64,
        var_qd_fp4s_dn1: f64,
        var_qd_fp4s_dn12: f64,
        var_qd_fp4s_dn14: f64,
        var_qd_fp4s_dn15: f64,
        var_qd_fp4s_dn16: f64,
        var_qd_fp4s_dn17: f64,
        var_qd_fp4s_dn18: f64,
        var_qd_fp4s_dn19: f64,
        var_qd_fp4s_dn2: f64,
        var_qd_fp4s_dn20: f64,
        var_qd_fp4s_dn21: f64,
        var_qd_fp4s_dn22: f64,
        var_qd_fp4s_dn3: f64,
        var_qd_fp4s_dn4: f64,
        var_qd_fp4s_dn5: f64,
        var_qd_fp4s_dn6: f64,
        var_qd_fp4s_dn7: f64,
        var_qd_fp4s_dn8: f64,
        var_qd_fp4s_dn9: f64,
        var_qg_fp4: f64,
        var_qg_fp4_dn0: f64,
        var_qg_fp4_dn1: f64,
        var_qg_fp4_dn12: f64,
        var_qg_fp4_dn14: f64,
        var_qg_fp4_dn15: f64,
        var_qg_fp4_dn16: f64,
        var_qg_fp4_dn17: f64,
        var_qg_fp4_dn18: f64,
        var_qg_fp4_dn19: f64,
        var_qg_fp4_dn2: f64,
        var_qg_fp4_dn20: f64,
        var_qg_fp4_dn21: f64,
        var_qg_fp4_dn22: f64,
        var_qg_fp4_dn3: f64,
        var_qg_fp4_dn4: f64,
        var_qg_fp4_dn5: f64,
        var_qg_fp4_dn6: f64,
        var_qg_fp4_dn7: f64,
        var_qg_fp4_dn8: f64,
        var_qg_fp4_dn9: f64,
    ) {
        let (eq196_e2466, eq196_e2466_d_n0, eq196_e2466_d_n1, eq196_e2466_d_n2, eq196_e2466_d_n3, eq196_e2466_d_n4, eq196_e2466_d_n5, eq196_e2466_d_n6, eq196_e2466_d_n7, eq196_e2466_d_n8, eq196_e2466_d_n9, eq196_e2466_d_n12, eq196_e2466_d_n14, eq196_e2466_d_n15, eq196_e2466_d_n16, eq196_e2466_d_n17, eq196_e2466_d_n18, eq196_e2466_d_n19, eq196_e2466_d_n20, eq196_e2466_d_n21, eq196_e2466_d_n22, eq196_e2466_q,) = {
    if (((var_guard566 != 0.0) && (var_guard567 != 0.0)) && (var_guard568 == 0.0)) {
        let eq196_e2461_q: f64 = var_qg_fp4;
        let eq196_e2462: f64 = (p.p7 * var_qg_fp4);
        let eq196_e2462_d_n0: f64 = (p.p7 * var_qg_fp4_dn0);
        let eq196_e2462_d_n1: f64 = (p.p7 * var_qg_fp4_dn1);
        let eq196_e2462_d_n2: f64 = (p.p7 * var_qg_fp4_dn2);
        let eq196_e2462_d_n3: f64 = (p.p7 * var_qg_fp4_dn3);
        let eq196_e2462_d_n4: f64 = (p.p7 * var_qg_fp4_dn4);
        let eq196_e2462_d_n5: f64 = (p.p7 * var_qg_fp4_dn5);
        let eq196_e2462_d_n6: f64 = (p.p7 * var_qg_fp4_dn6);
        let eq196_e2462_d_n7: f64 = (p.p7 * var_qg_fp4_dn7);
        let eq196_e2462_d_n8: f64 = (p.p7 * var_qg_fp4_dn8);
        let eq196_e2462_d_n9: f64 = (p.p7 * var_qg_fp4_dn9);
        let eq196_e2462_d_n12: f64 = (p.p7 * var_qg_fp4_dn12);
        let eq196_e2462_d_n14: f64 = (p.p7 * var_qg_fp4_dn14);
        let eq196_e2462_d_n15: f64 = (p.p7 * var_qg_fp4_dn15);
        let eq196_e2462_d_n16: f64 = (p.p7 * var_qg_fp4_dn16);
        let eq196_e2462_d_n17: f64 = (p.p7 * var_qg_fp4_dn17);
        let eq196_e2462_d_n18: f64 = (p.p7 * var_qg_fp4_dn18);
        let eq196_e2462_d_n19: f64 = (p.p7 * var_qg_fp4_dn19);
        let eq196_e2462_d_n20: f64 = (p.p7 * var_qg_fp4_dn20);
        let eq196_e2462_d_n21: f64 = (p.p7 * var_qg_fp4_dn21);
        let eq196_e2462_d_n22: f64 = (p.p7 * var_qg_fp4_dn22);
        let eq196_e2462_q: f64 = (p.p7 * eq196_e2461_q);
        let eq196_e2464: f64 = (eq196_e2462 * p.p249);
        let eq196_e2464_d_n0: f64 = (eq196_e2462_d_n0 * p.p249);
        let eq196_e2464_d_n1: f64 = (eq196_e2462_d_n1 * p.p249);
        let eq196_e2464_d_n2: f64 = (eq196_e2462_d_n2 * p.p249);
        let eq196_e2464_d_n3: f64 = (eq196_e2462_d_n3 * p.p249);
        let eq196_e2464_d_n4: f64 = (eq196_e2462_d_n4 * p.p249);
        let eq196_e2464_d_n5: f64 = (eq196_e2462_d_n5 * p.p249);
        let eq196_e2464_d_n6: f64 = (eq196_e2462_d_n6 * p.p249);
        let eq196_e2464_d_n7: f64 = (eq196_e2462_d_n7 * p.p249);
        let eq196_e2464_d_n8: f64 = (eq196_e2462_d_n8 * p.p249);
        let eq196_e2464_d_n9: f64 = (eq196_e2462_d_n9 * p.p249);
        let eq196_e2464_d_n12: f64 = (eq196_e2462_d_n12 * p.p249);
        let eq196_e2464_d_n14: f64 = (eq196_e2462_d_n14 * p.p249);
        let eq196_e2464_d_n15: f64 = (eq196_e2462_d_n15 * p.p249);
        let eq196_e2464_d_n16: f64 = (eq196_e2462_d_n16 * p.p249);
        let eq196_e2464_d_n17: f64 = (eq196_e2462_d_n17 * p.p249);
        let eq196_e2464_d_n18: f64 = (eq196_e2462_d_n18 * p.p249);
        let eq196_e2464_d_n19: f64 = (eq196_e2462_d_n19 * p.p249);
        let eq196_e2464_d_n20: f64 = (eq196_e2462_d_n20 * p.p249);
        let eq196_e2464_d_n21: f64 = (eq196_e2462_d_n21 * p.p249);
        let eq196_e2464_d_n22: f64 = (eq196_e2462_d_n22 * p.p249);
        let eq196_e2464_q: f64 = (eq196_e2462_q * p.p249);
        (eq196_e2464, eq196_e2464_d_n0, eq196_e2464_d_n1, eq196_e2464_d_n2, eq196_e2464_d_n3, eq196_e2464_d_n4, eq196_e2464_d_n5, eq196_e2464_d_n6, eq196_e2464_d_n7, eq196_e2464_d_n8, eq196_e2464_d_n9, eq196_e2464_d_n12, eq196_e2464_d_n14, eq196_e2464_d_n15, eq196_e2464_d_n16, eq196_e2464_d_n17, eq196_e2464_d_n18, eq196_e2464_d_n19, eq196_e2464_d_n20, eq196_e2464_d_n21, eq196_e2464_d_n22, eq196_e2464_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq196_reactive_node_derivatives: [f64; 23] = [eq196_e2466_d_n0, eq196_e2466_d_n1, eq196_e2466_d_n2, eq196_e2466_d_n3, eq196_e2466_d_n4, eq196_e2466_d_n5, eq196_e2466_d_n6, eq196_e2466_d_n7, eq196_e2466_d_n8, eq196_e2466_d_n9, 0.0, 0.0, eq196_e2466_d_n12, 0.0, eq196_e2466_d_n14, eq196_e2466_d_n15, eq196_e2466_d_n16, eq196_e2466_d_n17, eq196_e2466_d_n18, eq196_e2466_d_n19, eq196_e2466_d_n20, eq196_e2466_d_n21, eq196_e2466_d_n22];
        let eq196_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[17]),
            nodes,
            &eq196_reactive_node_derivatives,
            branches,
            &eq196_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq197_e2477, eq197_e2477_d_n0, eq197_e2477_d_n1, eq197_e2477_d_n2, eq197_e2477_d_n3, eq197_e2477_d_n4, eq197_e2477_d_n5, eq197_e2477_d_n6, eq197_e2477_d_n7, eq197_e2477_d_n8, eq197_e2477_d_n9, eq197_e2477_d_n12, eq197_e2477_d_n14, eq197_e2477_d_n15, eq197_e2477_d_n16, eq197_e2477_d_n17, eq197_e2477_d_n18, eq197_e2477_d_n19, eq197_e2477_d_n20, eq197_e2477_d_n21, eq197_e2477_d_n22, eq197_e2477_q,) = {
    if ((var_guard566 != 0.0) && (var_guard567 != 0.0)) {
        let eq197_e2473: f64 = (p.p254 * var_qg_fp4);
        let eq197_e2473_d_n0: f64 = (p.p254 * var_qg_fp4_dn0);
        let eq197_e2473_d_n1: f64 = (p.p254 * var_qg_fp4_dn1);
        let eq197_e2473_d_n2: f64 = (p.p254 * var_qg_fp4_dn2);
        let eq197_e2473_d_n3: f64 = (p.p254 * var_qg_fp4_dn3);
        let eq197_e2473_d_n4: f64 = (p.p254 * var_qg_fp4_dn4);
        let eq197_e2473_d_n5: f64 = (p.p254 * var_qg_fp4_dn5);
        let eq197_e2473_d_n6: f64 = (p.p254 * var_qg_fp4_dn6);
        let eq197_e2473_d_n7: f64 = (p.p254 * var_qg_fp4_dn7);
        let eq197_e2473_d_n8: f64 = (p.p254 * var_qg_fp4_dn8);
        let eq197_e2473_d_n9: f64 = (p.p254 * var_qg_fp4_dn9);
        let eq197_e2473_d_n12: f64 = (p.p254 * var_qg_fp4_dn12);
        let eq197_e2473_d_n14: f64 = (p.p254 * var_qg_fp4_dn14);
        let eq197_e2473_d_n15: f64 = (p.p254 * var_qg_fp4_dn15);
        let eq197_e2473_d_n16: f64 = (p.p254 * var_qg_fp4_dn16);
        let eq197_e2473_d_n17: f64 = (p.p254 * var_qg_fp4_dn17);
        let eq197_e2473_d_n18: f64 = (p.p254 * var_qg_fp4_dn18);
        let eq197_e2473_d_n19: f64 = (p.p254 * var_qg_fp4_dn19);
        let eq197_e2473_d_n20: f64 = (p.p254 * var_qg_fp4_dn20);
        let eq197_e2473_d_n21: f64 = (p.p254 * var_qg_fp4_dn21);
        let eq197_e2473_d_n22: f64 = (p.p254 * var_qg_fp4_dn22);
        let eq197_e2474_q: f64 = eq197_e2473;
        let eq197_e2475: f64 = (p.p7 * eq197_e2473);
        let eq197_e2475_d_n0: f64 = (p.p7 * eq197_e2473_d_n0);
        let eq197_e2475_d_n1: f64 = (p.p7 * eq197_e2473_d_n1);
        let eq197_e2475_d_n2: f64 = (p.p7 * eq197_e2473_d_n2);
        let eq197_e2475_d_n3: f64 = (p.p7 * eq197_e2473_d_n3);
        let eq197_e2475_d_n4: f64 = (p.p7 * eq197_e2473_d_n4);
        let eq197_e2475_d_n5: f64 = (p.p7 * eq197_e2473_d_n5);
        let eq197_e2475_d_n6: f64 = (p.p7 * eq197_e2473_d_n6);
        let eq197_e2475_d_n7: f64 = (p.p7 * eq197_e2473_d_n7);
        let eq197_e2475_d_n8: f64 = (p.p7 * eq197_e2473_d_n8);
        let eq197_e2475_d_n9: f64 = (p.p7 * eq197_e2473_d_n9);
        let eq197_e2475_d_n12: f64 = (p.p7 * eq197_e2473_d_n12);
        let eq197_e2475_d_n14: f64 = (p.p7 * eq197_e2473_d_n14);
        let eq197_e2475_d_n15: f64 = (p.p7 * eq197_e2473_d_n15);
        let eq197_e2475_d_n16: f64 = (p.p7 * eq197_e2473_d_n16);
        let eq197_e2475_d_n17: f64 = (p.p7 * eq197_e2473_d_n17);
        let eq197_e2475_d_n18: f64 = (p.p7 * eq197_e2473_d_n18);
        let eq197_e2475_d_n19: f64 = (p.p7 * eq197_e2473_d_n19);
        let eq197_e2475_d_n20: f64 = (p.p7 * eq197_e2473_d_n20);
        let eq197_e2475_d_n21: f64 = (p.p7 * eq197_e2473_d_n21);
        let eq197_e2475_d_n22: f64 = (p.p7 * eq197_e2473_d_n22);
        let eq197_e2475_q: f64 = (p.p7 * eq197_e2474_q);
        (eq197_e2475, eq197_e2475_d_n0, eq197_e2475_d_n1, eq197_e2475_d_n2, eq197_e2475_d_n3, eq197_e2475_d_n4, eq197_e2475_d_n5, eq197_e2475_d_n6, eq197_e2475_d_n7, eq197_e2475_d_n8, eq197_e2475_d_n9, eq197_e2475_d_n12, eq197_e2475_d_n14, eq197_e2475_d_n15, eq197_e2475_d_n16, eq197_e2475_d_n17, eq197_e2475_d_n18, eq197_e2475_d_n19, eq197_e2475_d_n20, eq197_e2475_d_n21, eq197_e2475_d_n22, eq197_e2475_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq197_reactive_node_derivatives: [f64; 23] = [eq197_e2477_d_n0, eq197_e2477_d_n1, eq197_e2477_d_n2, eq197_e2477_d_n3, eq197_e2477_d_n4, eq197_e2477_d_n5, eq197_e2477_d_n6, eq197_e2477_d_n7, eq197_e2477_d_n8, eq197_e2477_d_n9, 0.0, 0.0, eq197_e2477_d_n12, 0.0, eq197_e2477_d_n14, eq197_e2477_d_n15, eq197_e2477_d_n16, eq197_e2477_d_n17, eq197_e2477_d_n18, eq197_e2477_d_n19, eq197_e2477_d_n20, eq197_e2477_d_n21, eq197_e2477_d_n22];
        let eq197_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[17]),
            nodes,
            &eq197_reactive_node_derivatives,
            branches,
            &eq197_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq198_e2487, eq198_e2487_d_n0, eq198_e2487_d_n1, eq198_e2487_d_n2, eq198_e2487_d_n3, eq198_e2487_d_n4, eq198_e2487_d_n5, eq198_e2487_d_n6, eq198_e2487_d_n7, eq198_e2487_d_n8, eq198_e2487_d_n9, eq198_e2487_d_n12, eq198_e2487_d_n14, eq198_e2487_d_n15, eq198_e2487_d_n16, eq198_e2487_d_n17, eq198_e2487_d_n18, eq198_e2487_d_n19, eq198_e2487_d_n20, eq198_e2487_d_n21, eq198_e2487_d_n22, eq198_e2487_q,) = {
    if ((var_guard566 == 0.0) && (var_guard569 != 0.0)) {
        let eq198_e2484_q: f64 = var_qd_fp4;
        let eq198_e2485: f64 = (p.p7 * var_qd_fp4);
        let eq198_e2485_d_n0: f64 = (p.p7 * var_qd_fp4_dn0);
        let eq198_e2485_d_n1: f64 = (p.p7 * var_qd_fp4_dn1);
        let eq198_e2485_d_n2: f64 = (p.p7 * var_qd_fp4_dn2);
        let eq198_e2485_d_n3: f64 = (p.p7 * var_qd_fp4_dn3);
        let eq198_e2485_d_n4: f64 = (p.p7 * var_qd_fp4_dn4);
        let eq198_e2485_d_n5: f64 = (p.p7 * var_qd_fp4_dn5);
        let eq198_e2485_d_n6: f64 = (p.p7 * var_qd_fp4_dn6);
        let eq198_e2485_d_n7: f64 = (p.p7 * var_qd_fp4_dn7);
        let eq198_e2485_d_n8: f64 = (p.p7 * var_qd_fp4_dn8);
        let eq198_e2485_d_n9: f64 = (p.p7 * var_qd_fp4_dn9);
        let eq198_e2485_d_n12: f64 = (p.p7 * var_qd_fp4_dn12);
        let eq198_e2485_d_n14: f64 = (p.p7 * var_qd_fp4_dn14);
        let eq198_e2485_d_n15: f64 = (p.p7 * var_qd_fp4_dn15);
        let eq198_e2485_d_n16: f64 = (p.p7 * var_qd_fp4_dn16);
        let eq198_e2485_d_n17: f64 = (p.p7 * var_qd_fp4_dn17);
        let eq198_e2485_d_n18: f64 = (p.p7 * var_qd_fp4_dn18);
        let eq198_e2485_d_n19: f64 = (p.p7 * var_qd_fp4_dn19);
        let eq198_e2485_d_n20: f64 = (p.p7 * var_qd_fp4_dn20);
        let eq198_e2485_d_n21: f64 = (p.p7 * var_qd_fp4_dn21);
        let eq198_e2485_d_n22: f64 = (p.p7 * var_qd_fp4_dn22);
        let eq198_e2485_q: f64 = (p.p7 * eq198_e2484_q);
        (eq198_e2485, eq198_e2485_d_n0, eq198_e2485_d_n1, eq198_e2485_d_n2, eq198_e2485_d_n3, eq198_e2485_d_n4, eq198_e2485_d_n5, eq198_e2485_d_n6, eq198_e2485_d_n7, eq198_e2485_d_n8, eq198_e2485_d_n9, eq198_e2485_d_n12, eq198_e2485_d_n14, eq198_e2485_d_n15, eq198_e2485_d_n16, eq198_e2485_d_n17, eq198_e2485_d_n18, eq198_e2485_d_n19, eq198_e2485_d_n20, eq198_e2485_d_n21, eq198_e2485_d_n22, eq198_e2485_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq198_reactive_node_derivatives: [f64; 23] = [eq198_e2487_d_n0, eq198_e2487_d_n1, eq198_e2487_d_n2, eq198_e2487_d_n3, eq198_e2487_d_n4, eq198_e2487_d_n5, eq198_e2487_d_n6, eq198_e2487_d_n7, eq198_e2487_d_n8, eq198_e2487_d_n9, 0.0, 0.0, eq198_e2487_d_n12, 0.0, eq198_e2487_d_n14, eq198_e2487_d_n15, eq198_e2487_d_n16, eq198_e2487_d_n17, eq198_e2487_d_n18, eq198_e2487_d_n19, eq198_e2487_d_n20, eq198_e2487_d_n21, eq198_e2487_d_n22];
        let eq198_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[7]),
            nodes,
            &eq198_reactive_node_derivatives,
            branches,
            &eq198_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq199_e2499, eq199_e2499_d_n0, eq199_e2499_d_n1, eq199_e2499_d_n2, eq199_e2499_d_n3, eq199_e2499_d_n4, eq199_e2499_d_n5, eq199_e2499_d_n6, eq199_e2499_d_n7, eq199_e2499_d_n8, eq199_e2499_d_n9, eq199_e2499_d_n12, eq199_e2499_d_n14, eq199_e2499_d_n15, eq199_e2499_d_n16, eq199_e2499_d_n17, eq199_e2499_d_n18, eq199_e2499_d_n19, eq199_e2499_d_n20, eq199_e2499_d_n21, eq199_e2499_d_n22, eq199_e2499_q,) = {
    if (((var_guard566 == 0.0) && (var_guard569 != 0.0)) && (var_guard570 != 0.0)) {
        let eq199_e2496_q: f64 = var_qg_fp4;
        let eq199_e2497: f64 = (p.p7 * var_qg_fp4);
        let eq199_e2497_d_n0: f64 = (p.p7 * var_qg_fp4_dn0);
        let eq199_e2497_d_n1: f64 = (p.p7 * var_qg_fp4_dn1);
        let eq199_e2497_d_n2: f64 = (p.p7 * var_qg_fp4_dn2);
        let eq199_e2497_d_n3: f64 = (p.p7 * var_qg_fp4_dn3);
        let eq199_e2497_d_n4: f64 = (p.p7 * var_qg_fp4_dn4);
        let eq199_e2497_d_n5: f64 = (p.p7 * var_qg_fp4_dn5);
        let eq199_e2497_d_n6: f64 = (p.p7 * var_qg_fp4_dn6);
        let eq199_e2497_d_n7: f64 = (p.p7 * var_qg_fp4_dn7);
        let eq199_e2497_d_n8: f64 = (p.p7 * var_qg_fp4_dn8);
        let eq199_e2497_d_n9: f64 = (p.p7 * var_qg_fp4_dn9);
        let eq199_e2497_d_n12: f64 = (p.p7 * var_qg_fp4_dn12);
        let eq199_e2497_d_n14: f64 = (p.p7 * var_qg_fp4_dn14);
        let eq199_e2497_d_n15: f64 = (p.p7 * var_qg_fp4_dn15);
        let eq199_e2497_d_n16: f64 = (p.p7 * var_qg_fp4_dn16);
        let eq199_e2497_d_n17: f64 = (p.p7 * var_qg_fp4_dn17);
        let eq199_e2497_d_n18: f64 = (p.p7 * var_qg_fp4_dn18);
        let eq199_e2497_d_n19: f64 = (p.p7 * var_qg_fp4_dn19);
        let eq199_e2497_d_n20: f64 = (p.p7 * var_qg_fp4_dn20);
        let eq199_e2497_d_n21: f64 = (p.p7 * var_qg_fp4_dn21);
        let eq199_e2497_d_n22: f64 = (p.p7 * var_qg_fp4_dn22);
        let eq199_e2497_q: f64 = (p.p7 * eq199_e2496_q);
        (eq199_e2497, eq199_e2497_d_n0, eq199_e2497_d_n1, eq199_e2497_d_n2, eq199_e2497_d_n3, eq199_e2497_d_n4, eq199_e2497_d_n5, eq199_e2497_d_n6, eq199_e2497_d_n7, eq199_e2497_d_n8, eq199_e2497_d_n9, eq199_e2497_d_n12, eq199_e2497_d_n14, eq199_e2497_d_n15, eq199_e2497_d_n16, eq199_e2497_d_n17, eq199_e2497_d_n18, eq199_e2497_d_n19, eq199_e2497_d_n20, eq199_e2497_d_n21, eq199_e2497_d_n22, eq199_e2497_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq199_reactive_node_derivatives: [f64; 23] = [eq199_e2499_d_n0, eq199_e2499_d_n1, eq199_e2499_d_n2, eq199_e2499_d_n3, eq199_e2499_d_n4, eq199_e2499_d_n5, eq199_e2499_d_n6, eq199_e2499_d_n7, eq199_e2499_d_n8, eq199_e2499_d_n9, 0.0, 0.0, eq199_e2499_d_n12, 0.0, eq199_e2499_d_n14, eq199_e2499_d_n15, eq199_e2499_d_n16, eq199_e2499_d_n17, eq199_e2499_d_n18, eq199_e2499_d_n19, eq199_e2499_d_n20, eq199_e2499_d_n21, eq199_e2499_d_n22];
        let eq199_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq199_reactive_node_derivatives,
            branches,
            &eq199_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq200_e2513, eq200_e2513_d_n0, eq200_e2513_d_n1, eq200_e2513_d_n2, eq200_e2513_d_n3, eq200_e2513_d_n4, eq200_e2513_d_n5, eq200_e2513_d_n6, eq200_e2513_d_n7, eq200_e2513_d_n8, eq200_e2513_d_n9, eq200_e2513_d_n12, eq200_e2513_d_n14, eq200_e2513_d_n15, eq200_e2513_d_n16, eq200_e2513_d_n17, eq200_e2513_d_n18, eq200_e2513_d_n19, eq200_e2513_d_n20, eq200_e2513_d_n21, eq200_e2513_d_n22, eq200_e2513_q,) = {
    if (((var_guard566 == 0.0) && (var_guard569 != 0.0)) && (var_guard570 != 0.0)) {
        let eq200_e2508_q: f64 = var_qg_fp4;
        let eq200_e2509: f64 = (p.p7 * var_qg_fp4);
        let eq200_e2509_d_n0: f64 = (p.p7 * var_qg_fp4_dn0);
        let eq200_e2509_d_n1: f64 = (p.p7 * var_qg_fp4_dn1);
        let eq200_e2509_d_n2: f64 = (p.p7 * var_qg_fp4_dn2);
        let eq200_e2509_d_n3: f64 = (p.p7 * var_qg_fp4_dn3);
        let eq200_e2509_d_n4: f64 = (p.p7 * var_qg_fp4_dn4);
        let eq200_e2509_d_n5: f64 = (p.p7 * var_qg_fp4_dn5);
        let eq200_e2509_d_n6: f64 = (p.p7 * var_qg_fp4_dn6);
        let eq200_e2509_d_n7: f64 = (p.p7 * var_qg_fp4_dn7);
        let eq200_e2509_d_n8: f64 = (p.p7 * var_qg_fp4_dn8);
        let eq200_e2509_d_n9: f64 = (p.p7 * var_qg_fp4_dn9);
        let eq200_e2509_d_n12: f64 = (p.p7 * var_qg_fp4_dn12);
        let eq200_e2509_d_n14: f64 = (p.p7 * var_qg_fp4_dn14);
        let eq200_e2509_d_n15: f64 = (p.p7 * var_qg_fp4_dn15);
        let eq200_e2509_d_n16: f64 = (p.p7 * var_qg_fp4_dn16);
        let eq200_e2509_d_n17: f64 = (p.p7 * var_qg_fp4_dn17);
        let eq200_e2509_d_n18: f64 = (p.p7 * var_qg_fp4_dn18);
        let eq200_e2509_d_n19: f64 = (p.p7 * var_qg_fp4_dn19);
        let eq200_e2509_d_n20: f64 = (p.p7 * var_qg_fp4_dn20);
        let eq200_e2509_d_n21: f64 = (p.p7 * var_qg_fp4_dn21);
        let eq200_e2509_d_n22: f64 = (p.p7 * var_qg_fp4_dn22);
        let eq200_e2509_q: f64 = (p.p7 * eq200_e2508_q);
        let eq200_e2511: f64 = (eq200_e2509 * p.p249);
        let eq200_e2511_d_n0: f64 = (eq200_e2509_d_n0 * p.p249);
        let eq200_e2511_d_n1: f64 = (eq200_e2509_d_n1 * p.p249);
        let eq200_e2511_d_n2: f64 = (eq200_e2509_d_n2 * p.p249);
        let eq200_e2511_d_n3: f64 = (eq200_e2509_d_n3 * p.p249);
        let eq200_e2511_d_n4: f64 = (eq200_e2509_d_n4 * p.p249);
        let eq200_e2511_d_n5: f64 = (eq200_e2509_d_n5 * p.p249);
        let eq200_e2511_d_n6: f64 = (eq200_e2509_d_n6 * p.p249);
        let eq200_e2511_d_n7: f64 = (eq200_e2509_d_n7 * p.p249);
        let eq200_e2511_d_n8: f64 = (eq200_e2509_d_n8 * p.p249);
        let eq200_e2511_d_n9: f64 = (eq200_e2509_d_n9 * p.p249);
        let eq200_e2511_d_n12: f64 = (eq200_e2509_d_n12 * p.p249);
        let eq200_e2511_d_n14: f64 = (eq200_e2509_d_n14 * p.p249);
        let eq200_e2511_d_n15: f64 = (eq200_e2509_d_n15 * p.p249);
        let eq200_e2511_d_n16: f64 = (eq200_e2509_d_n16 * p.p249);
        let eq200_e2511_d_n17: f64 = (eq200_e2509_d_n17 * p.p249);
        let eq200_e2511_d_n18: f64 = (eq200_e2509_d_n18 * p.p249);
        let eq200_e2511_d_n19: f64 = (eq200_e2509_d_n19 * p.p249);
        let eq200_e2511_d_n20: f64 = (eq200_e2509_d_n20 * p.p249);
        let eq200_e2511_d_n21: f64 = (eq200_e2509_d_n21 * p.p249);
        let eq200_e2511_d_n22: f64 = (eq200_e2509_d_n22 * p.p249);
        let eq200_e2511_q: f64 = (eq200_e2509_q * p.p249);
        (eq200_e2511, eq200_e2511_d_n0, eq200_e2511_d_n1, eq200_e2511_d_n2, eq200_e2511_d_n3, eq200_e2511_d_n4, eq200_e2511_d_n5, eq200_e2511_d_n6, eq200_e2511_d_n7, eq200_e2511_d_n8, eq200_e2511_d_n9, eq200_e2511_d_n12, eq200_e2511_d_n14, eq200_e2511_d_n15, eq200_e2511_d_n16, eq200_e2511_d_n17, eq200_e2511_d_n18, eq200_e2511_d_n19, eq200_e2511_d_n20, eq200_e2511_d_n21, eq200_e2511_d_n22, eq200_e2511_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq200_reactive_node_derivatives: [f64; 23] = [eq200_e2513_d_n0, eq200_e2513_d_n1, eq200_e2513_d_n2, eq200_e2513_d_n3, eq200_e2513_d_n4, eq200_e2513_d_n5, eq200_e2513_d_n6, eq200_e2513_d_n7, eq200_e2513_d_n8, eq200_e2513_d_n9, 0.0, 0.0, eq200_e2513_d_n12, 0.0, eq200_e2513_d_n14, eq200_e2513_d_n15, eq200_e2513_d_n16, eq200_e2513_d_n17, eq200_e2513_d_n18, eq200_e2513_d_n19, eq200_e2513_d_n20, eq200_e2513_d_n21, eq200_e2513_d_n22];
        let eq200_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq200_reactive_node_derivatives,
            branches,
            &eq200_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq201_e2526, eq201_e2526_d_n0, eq201_e2526_d_n1, eq201_e2526_d_n2, eq201_e2526_d_n3, eq201_e2526_d_n4, eq201_e2526_d_n5, eq201_e2526_d_n6, eq201_e2526_d_n7, eq201_e2526_d_n8, eq201_e2526_d_n9, eq201_e2526_d_n12, eq201_e2526_d_n14, eq201_e2526_d_n15, eq201_e2526_d_n16, eq201_e2526_d_n17, eq201_e2526_d_n18, eq201_e2526_d_n19, eq201_e2526_d_n20, eq201_e2526_d_n21, eq201_e2526_d_n22, eq201_e2526_q,) = {
    if (((var_guard566 == 0.0) && (var_guard569 != 0.0)) && (var_guard570 == 0.0)) {
        let eq201_e2523_q: f64 = var_qg_fp4;
        let eq201_e2524: f64 = (p.p7 * var_qg_fp4);
        let eq201_e2524_d_n0: f64 = (p.p7 * var_qg_fp4_dn0);
        let eq201_e2524_d_n1: f64 = (p.p7 * var_qg_fp4_dn1);
        let eq201_e2524_d_n2: f64 = (p.p7 * var_qg_fp4_dn2);
        let eq201_e2524_d_n3: f64 = (p.p7 * var_qg_fp4_dn3);
        let eq201_e2524_d_n4: f64 = (p.p7 * var_qg_fp4_dn4);
        let eq201_e2524_d_n5: f64 = (p.p7 * var_qg_fp4_dn5);
        let eq201_e2524_d_n6: f64 = (p.p7 * var_qg_fp4_dn6);
        let eq201_e2524_d_n7: f64 = (p.p7 * var_qg_fp4_dn7);
        let eq201_e2524_d_n8: f64 = (p.p7 * var_qg_fp4_dn8);
        let eq201_e2524_d_n9: f64 = (p.p7 * var_qg_fp4_dn9);
        let eq201_e2524_d_n12: f64 = (p.p7 * var_qg_fp4_dn12);
        let eq201_e2524_d_n14: f64 = (p.p7 * var_qg_fp4_dn14);
        let eq201_e2524_d_n15: f64 = (p.p7 * var_qg_fp4_dn15);
        let eq201_e2524_d_n16: f64 = (p.p7 * var_qg_fp4_dn16);
        let eq201_e2524_d_n17: f64 = (p.p7 * var_qg_fp4_dn17);
        let eq201_e2524_d_n18: f64 = (p.p7 * var_qg_fp4_dn18);
        let eq201_e2524_d_n19: f64 = (p.p7 * var_qg_fp4_dn19);
        let eq201_e2524_d_n20: f64 = (p.p7 * var_qg_fp4_dn20);
        let eq201_e2524_d_n21: f64 = (p.p7 * var_qg_fp4_dn21);
        let eq201_e2524_d_n22: f64 = (p.p7 * var_qg_fp4_dn22);
        let eq201_e2524_q: f64 = (p.p7 * eq201_e2523_q);
        (eq201_e2524, eq201_e2524_d_n0, eq201_e2524_d_n1, eq201_e2524_d_n2, eq201_e2524_d_n3, eq201_e2524_d_n4, eq201_e2524_d_n5, eq201_e2524_d_n6, eq201_e2524_d_n7, eq201_e2524_d_n8, eq201_e2524_d_n9, eq201_e2524_d_n12, eq201_e2524_d_n14, eq201_e2524_d_n15, eq201_e2524_d_n16, eq201_e2524_d_n17, eq201_e2524_d_n18, eq201_e2524_d_n19, eq201_e2524_d_n20, eq201_e2524_d_n21, eq201_e2524_d_n22, eq201_e2524_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq201_reactive_node_derivatives: [f64; 23] = [eq201_e2526_d_n0, eq201_e2526_d_n1, eq201_e2526_d_n2, eq201_e2526_d_n3, eq201_e2526_d_n4, eq201_e2526_d_n5, eq201_e2526_d_n6, eq201_e2526_d_n7, eq201_e2526_d_n8, eq201_e2526_d_n9, 0.0, 0.0, eq201_e2526_d_n12, 0.0, eq201_e2526_d_n14, eq201_e2526_d_n15, eq201_e2526_d_n16, eq201_e2526_d_n17, eq201_e2526_d_n18, eq201_e2526_d_n19, eq201_e2526_d_n20, eq201_e2526_d_n21, eq201_e2526_d_n22];
        let eq201_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq201_reactive_node_derivatives,
            branches,
            &eq201_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq202_e2541, eq202_e2541_d_n0, eq202_e2541_d_n1, eq202_e2541_d_n2, eq202_e2541_d_n3, eq202_e2541_d_n4, eq202_e2541_d_n5, eq202_e2541_d_n6, eq202_e2541_d_n7, eq202_e2541_d_n8, eq202_e2541_d_n9, eq202_e2541_d_n12, eq202_e2541_d_n14, eq202_e2541_d_n15, eq202_e2541_d_n16, eq202_e2541_d_n17, eq202_e2541_d_n18, eq202_e2541_d_n19, eq202_e2541_d_n20, eq202_e2541_d_n21, eq202_e2541_d_n22, eq202_e2541_q,) = {
    if (((var_guard566 == 0.0) && (var_guard569 != 0.0)) && (var_guard570 == 0.0)) {
        let eq202_e2536_q: f64 = var_qg_fp4;
        let eq202_e2537: f64 = (p.p7 * var_qg_fp4);
        let eq202_e2537_d_n0: f64 = (p.p7 * var_qg_fp4_dn0);
        let eq202_e2537_d_n1: f64 = (p.p7 * var_qg_fp4_dn1);
        let eq202_e2537_d_n2: f64 = (p.p7 * var_qg_fp4_dn2);
        let eq202_e2537_d_n3: f64 = (p.p7 * var_qg_fp4_dn3);
        let eq202_e2537_d_n4: f64 = (p.p7 * var_qg_fp4_dn4);
        let eq202_e2537_d_n5: f64 = (p.p7 * var_qg_fp4_dn5);
        let eq202_e2537_d_n6: f64 = (p.p7 * var_qg_fp4_dn6);
        let eq202_e2537_d_n7: f64 = (p.p7 * var_qg_fp4_dn7);
        let eq202_e2537_d_n8: f64 = (p.p7 * var_qg_fp4_dn8);
        let eq202_e2537_d_n9: f64 = (p.p7 * var_qg_fp4_dn9);
        let eq202_e2537_d_n12: f64 = (p.p7 * var_qg_fp4_dn12);
        let eq202_e2537_d_n14: f64 = (p.p7 * var_qg_fp4_dn14);
        let eq202_e2537_d_n15: f64 = (p.p7 * var_qg_fp4_dn15);
        let eq202_e2537_d_n16: f64 = (p.p7 * var_qg_fp4_dn16);
        let eq202_e2537_d_n17: f64 = (p.p7 * var_qg_fp4_dn17);
        let eq202_e2537_d_n18: f64 = (p.p7 * var_qg_fp4_dn18);
        let eq202_e2537_d_n19: f64 = (p.p7 * var_qg_fp4_dn19);
        let eq202_e2537_d_n20: f64 = (p.p7 * var_qg_fp4_dn20);
        let eq202_e2537_d_n21: f64 = (p.p7 * var_qg_fp4_dn21);
        let eq202_e2537_d_n22: f64 = (p.p7 * var_qg_fp4_dn22);
        let eq202_e2537_q: f64 = (p.p7 * eq202_e2536_q);
        let eq202_e2539: f64 = (eq202_e2537 * p.p249);
        let eq202_e2539_d_n0: f64 = (eq202_e2537_d_n0 * p.p249);
        let eq202_e2539_d_n1: f64 = (eq202_e2537_d_n1 * p.p249);
        let eq202_e2539_d_n2: f64 = (eq202_e2537_d_n2 * p.p249);
        let eq202_e2539_d_n3: f64 = (eq202_e2537_d_n3 * p.p249);
        let eq202_e2539_d_n4: f64 = (eq202_e2537_d_n4 * p.p249);
        let eq202_e2539_d_n5: f64 = (eq202_e2537_d_n5 * p.p249);
        let eq202_e2539_d_n6: f64 = (eq202_e2537_d_n6 * p.p249);
        let eq202_e2539_d_n7: f64 = (eq202_e2537_d_n7 * p.p249);
        let eq202_e2539_d_n8: f64 = (eq202_e2537_d_n8 * p.p249);
        let eq202_e2539_d_n9: f64 = (eq202_e2537_d_n9 * p.p249);
        let eq202_e2539_d_n12: f64 = (eq202_e2537_d_n12 * p.p249);
        let eq202_e2539_d_n14: f64 = (eq202_e2537_d_n14 * p.p249);
        let eq202_e2539_d_n15: f64 = (eq202_e2537_d_n15 * p.p249);
        let eq202_e2539_d_n16: f64 = (eq202_e2537_d_n16 * p.p249);
        let eq202_e2539_d_n17: f64 = (eq202_e2537_d_n17 * p.p249);
        let eq202_e2539_d_n18: f64 = (eq202_e2537_d_n18 * p.p249);
        let eq202_e2539_d_n19: f64 = (eq202_e2537_d_n19 * p.p249);
        let eq202_e2539_d_n20: f64 = (eq202_e2537_d_n20 * p.p249);
        let eq202_e2539_d_n21: f64 = (eq202_e2537_d_n21 * p.p249);
        let eq202_e2539_d_n22: f64 = (eq202_e2537_d_n22 * p.p249);
        let eq202_e2539_q: f64 = (eq202_e2537_q * p.p249);
        (eq202_e2539, eq202_e2539_d_n0, eq202_e2539_d_n1, eq202_e2539_d_n2, eq202_e2539_d_n3, eq202_e2539_d_n4, eq202_e2539_d_n5, eq202_e2539_d_n6, eq202_e2539_d_n7, eq202_e2539_d_n8, eq202_e2539_d_n9, eq202_e2539_d_n12, eq202_e2539_d_n14, eq202_e2539_d_n15, eq202_e2539_d_n16, eq202_e2539_d_n17, eq202_e2539_d_n18, eq202_e2539_d_n19, eq202_e2539_d_n20, eq202_e2539_d_n21, eq202_e2539_d_n22, eq202_e2539_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq202_reactive_node_derivatives: [f64; 23] = [eq202_e2541_d_n0, eq202_e2541_d_n1, eq202_e2541_d_n2, eq202_e2541_d_n3, eq202_e2541_d_n4, eq202_e2541_d_n5, eq202_e2541_d_n6, eq202_e2541_d_n7, eq202_e2541_d_n8, eq202_e2541_d_n9, 0.0, 0.0, eq202_e2541_d_n12, 0.0, eq202_e2541_d_n14, eq202_e2541_d_n15, eq202_e2541_d_n16, eq202_e2541_d_n17, eq202_e2541_d_n18, eq202_e2541_d_n19, eq202_e2541_d_n20, eq202_e2541_d_n21, eq202_e2541_d_n22];
        let eq202_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq202_reactive_node_derivatives,
            branches,
            &eq202_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq203_e2553, eq203_e2553_d_n0, eq203_e2553_d_n1, eq203_e2553_d_n2, eq203_e2553_d_n3, eq203_e2553_d_n4, eq203_e2553_d_n5, eq203_e2553_d_n6, eq203_e2553_d_n7, eq203_e2553_d_n8, eq203_e2553_d_n9, eq203_e2553_d_n12, eq203_e2553_d_n14, eq203_e2553_d_n15, eq203_e2553_d_n16, eq203_e2553_d_n17, eq203_e2553_d_n18, eq203_e2553_d_n19, eq203_e2553_d_n20, eq203_e2553_d_n21, eq203_e2553_d_n22, eq203_e2553_q,) = {
    if ((var_guard566 == 0.0) && (var_guard569 != 0.0)) {
        let eq203_e2549: f64 = (p.p254 * var_qg_fp4);
        let eq203_e2549_d_n0: f64 = (p.p254 * var_qg_fp4_dn0);
        let eq203_e2549_d_n1: f64 = (p.p254 * var_qg_fp4_dn1);
        let eq203_e2549_d_n2: f64 = (p.p254 * var_qg_fp4_dn2);
        let eq203_e2549_d_n3: f64 = (p.p254 * var_qg_fp4_dn3);
        let eq203_e2549_d_n4: f64 = (p.p254 * var_qg_fp4_dn4);
        let eq203_e2549_d_n5: f64 = (p.p254 * var_qg_fp4_dn5);
        let eq203_e2549_d_n6: f64 = (p.p254 * var_qg_fp4_dn6);
        let eq203_e2549_d_n7: f64 = (p.p254 * var_qg_fp4_dn7);
        let eq203_e2549_d_n8: f64 = (p.p254 * var_qg_fp4_dn8);
        let eq203_e2549_d_n9: f64 = (p.p254 * var_qg_fp4_dn9);
        let eq203_e2549_d_n12: f64 = (p.p254 * var_qg_fp4_dn12);
        let eq203_e2549_d_n14: f64 = (p.p254 * var_qg_fp4_dn14);
        let eq203_e2549_d_n15: f64 = (p.p254 * var_qg_fp4_dn15);
        let eq203_e2549_d_n16: f64 = (p.p254 * var_qg_fp4_dn16);
        let eq203_e2549_d_n17: f64 = (p.p254 * var_qg_fp4_dn17);
        let eq203_e2549_d_n18: f64 = (p.p254 * var_qg_fp4_dn18);
        let eq203_e2549_d_n19: f64 = (p.p254 * var_qg_fp4_dn19);
        let eq203_e2549_d_n20: f64 = (p.p254 * var_qg_fp4_dn20);
        let eq203_e2549_d_n21: f64 = (p.p254 * var_qg_fp4_dn21);
        let eq203_e2549_d_n22: f64 = (p.p254 * var_qg_fp4_dn22);
        let eq203_e2550_q: f64 = eq203_e2549;
        let eq203_e2551: f64 = (p.p7 * eq203_e2549);
        let eq203_e2551_d_n0: f64 = (p.p7 * eq203_e2549_d_n0);
        let eq203_e2551_d_n1: f64 = (p.p7 * eq203_e2549_d_n1);
        let eq203_e2551_d_n2: f64 = (p.p7 * eq203_e2549_d_n2);
        let eq203_e2551_d_n3: f64 = (p.p7 * eq203_e2549_d_n3);
        let eq203_e2551_d_n4: f64 = (p.p7 * eq203_e2549_d_n4);
        let eq203_e2551_d_n5: f64 = (p.p7 * eq203_e2549_d_n5);
        let eq203_e2551_d_n6: f64 = (p.p7 * eq203_e2549_d_n6);
        let eq203_e2551_d_n7: f64 = (p.p7 * eq203_e2549_d_n7);
        let eq203_e2551_d_n8: f64 = (p.p7 * eq203_e2549_d_n8);
        let eq203_e2551_d_n9: f64 = (p.p7 * eq203_e2549_d_n9);
        let eq203_e2551_d_n12: f64 = (p.p7 * eq203_e2549_d_n12);
        let eq203_e2551_d_n14: f64 = (p.p7 * eq203_e2549_d_n14);
        let eq203_e2551_d_n15: f64 = (p.p7 * eq203_e2549_d_n15);
        let eq203_e2551_d_n16: f64 = (p.p7 * eq203_e2549_d_n16);
        let eq203_e2551_d_n17: f64 = (p.p7 * eq203_e2549_d_n17);
        let eq203_e2551_d_n18: f64 = (p.p7 * eq203_e2549_d_n18);
        let eq203_e2551_d_n19: f64 = (p.p7 * eq203_e2549_d_n19);
        let eq203_e2551_d_n20: f64 = (p.p7 * eq203_e2549_d_n20);
        let eq203_e2551_d_n21: f64 = (p.p7 * eq203_e2549_d_n21);
        let eq203_e2551_d_n22: f64 = (p.p7 * eq203_e2549_d_n22);
        let eq203_e2551_q: f64 = (p.p7 * eq203_e2550_q);
        (eq203_e2551, eq203_e2551_d_n0, eq203_e2551_d_n1, eq203_e2551_d_n2, eq203_e2551_d_n3, eq203_e2551_d_n4, eq203_e2551_d_n5, eq203_e2551_d_n6, eq203_e2551_d_n7, eq203_e2551_d_n8, eq203_e2551_d_n9, eq203_e2551_d_n12, eq203_e2551_d_n14, eq203_e2551_d_n15, eq203_e2551_d_n16, eq203_e2551_d_n17, eq203_e2551_d_n18, eq203_e2551_d_n19, eq203_e2551_d_n20, eq203_e2551_d_n21, eq203_e2551_d_n22, eq203_e2551_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq203_reactive_node_derivatives: [f64; 23] = [eq203_e2553_d_n0, eq203_e2553_d_n1, eq203_e2553_d_n2, eq203_e2553_d_n3, eq203_e2553_d_n4, eq203_e2553_d_n5, eq203_e2553_d_n6, eq203_e2553_d_n7, eq203_e2553_d_n8, eq203_e2553_d_n9, 0.0, 0.0, eq203_e2553_d_n12, 0.0, eq203_e2553_d_n14, eq203_e2553_d_n15, eq203_e2553_d_n16, eq203_e2553_d_n17, eq203_e2553_d_n18, eq203_e2553_d_n19, eq203_e2553_d_n20, eq203_e2553_d_n21, eq203_e2553_d_n22];
        let eq203_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            nodes,
            &eq203_reactive_node_derivatives,
            branches,
            &eq203_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq204_e2562, eq204_e2562_d_n0, eq204_e2562_d_n1, eq204_e2562_d_n2, eq204_e2562_d_n3, eq204_e2562_d_n4, eq204_e2562_d_n5, eq204_e2562_d_n6, eq204_e2562_d_n7, eq204_e2562_d_n8, eq204_e2562_d_n9, eq204_e2562_d_n12, eq204_e2562_d_n14, eq204_e2562_d_n15, eq204_e2562_d_n16, eq204_e2562_d_n17, eq204_e2562_d_n18, eq204_e2562_d_n19, eq204_e2562_d_n20, eq204_e2562_d_n21, eq204_e2562_d_n22, eq204_e2562_q,) = {
    if ((var_guard571 != 0.0) && (var_guard572 != 0.0)) {
        let eq204_e2559_q: f64 = var_qd_fp4s;
        let eq204_e2560: f64 = (p.p7 * var_qd_fp4s);
        let eq204_e2560_d_n0: f64 = (p.p7 * var_qd_fp4s_dn0);
        let eq204_e2560_d_n1: f64 = (p.p7 * var_qd_fp4s_dn1);
        let eq204_e2560_d_n2: f64 = (p.p7 * var_qd_fp4s_dn2);
        let eq204_e2560_d_n3: f64 = (p.p7 * var_qd_fp4s_dn3);
        let eq204_e2560_d_n4: f64 = (p.p7 * var_qd_fp4s_dn4);
        let eq204_e2560_d_n5: f64 = (p.p7 * var_qd_fp4s_dn5);
        let eq204_e2560_d_n6: f64 = (p.p7 * var_qd_fp4s_dn6);
        let eq204_e2560_d_n7: f64 = (p.p7 * var_qd_fp4s_dn7);
        let eq204_e2560_d_n8: f64 = (p.p7 * var_qd_fp4s_dn8);
        let eq204_e2560_d_n9: f64 = (p.p7 * var_qd_fp4s_dn9);
        let eq204_e2560_d_n12: f64 = (p.p7 * var_qd_fp4s_dn12);
        let eq204_e2560_d_n14: f64 = (p.p7 * var_qd_fp4s_dn14);
        let eq204_e2560_d_n15: f64 = (p.p7 * var_qd_fp4s_dn15);
        let eq204_e2560_d_n16: f64 = (p.p7 * var_qd_fp4s_dn16);
        let eq204_e2560_d_n17: f64 = (p.p7 * var_qd_fp4s_dn17);
        let eq204_e2560_d_n18: f64 = (p.p7 * var_qd_fp4s_dn18);
        let eq204_e2560_d_n19: f64 = (p.p7 * var_qd_fp4s_dn19);
        let eq204_e2560_d_n20: f64 = (p.p7 * var_qd_fp4s_dn20);
        let eq204_e2560_d_n21: f64 = (p.p7 * var_qd_fp4s_dn21);
        let eq204_e2560_d_n22: f64 = (p.p7 * var_qd_fp4s_dn22);
        let eq204_e2560_q: f64 = (p.p7 * eq204_e2559_q);
        (eq204_e2560, eq204_e2560_d_n0, eq204_e2560_d_n1, eq204_e2560_d_n2, eq204_e2560_d_n3, eq204_e2560_d_n4, eq204_e2560_d_n5, eq204_e2560_d_n6, eq204_e2560_d_n7, eq204_e2560_d_n8, eq204_e2560_d_n9, eq204_e2560_d_n12, eq204_e2560_d_n14, eq204_e2560_d_n15, eq204_e2560_d_n16, eq204_e2560_d_n17, eq204_e2560_d_n18, eq204_e2560_d_n19, eq204_e2560_d_n20, eq204_e2560_d_n21, eq204_e2560_d_n22, eq204_e2560_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq204_reactive_node_derivatives: [f64; 23] = [eq204_e2562_d_n0, eq204_e2562_d_n1, eq204_e2562_d_n2, eq204_e2562_d_n3, eq204_e2562_d_n4, eq204_e2562_d_n5, eq204_e2562_d_n6, eq204_e2562_d_n7, eq204_e2562_d_n8, eq204_e2562_d_n9, 0.0, 0.0, eq204_e2562_d_n12, 0.0, eq204_e2562_d_n14, eq204_e2562_d_n15, eq204_e2562_d_n16, eq204_e2562_d_n17, eq204_e2562_d_n18, eq204_e2562_d_n19, eq204_e2562_d_n20, eq204_e2562_d_n21, eq204_e2562_d_n22];
        let eq204_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[22]),
            nodes,
            &eq204_reactive_node_derivatives,
            branches,
            &eq204_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_10(
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_guard571: f64,
        var_guard572: f64,
        var_guard573: f64,
        var_guard574: f64,
        var_guard575: f64,
        var_qd_fp4s: f64,
        var_qd_fp4s_dn0: f64,
        var_qd_fp4s_dn1: f64,
        var_qd_fp4s_dn12: f64,
        var_qd_fp4s_dn14: f64,
        var_qd_fp4s_dn15: f64,
        var_qd_fp4s_dn16: f64,
        var_qd_fp4s_dn17: f64,
        var_qd_fp4s_dn18: f64,
        var_qd_fp4s_dn19: f64,
        var_qd_fp4s_dn2: f64,
        var_qd_fp4s_dn20: f64,
        var_qd_fp4s_dn21: f64,
        var_qd_fp4s_dn22: f64,
        var_qd_fp4s_dn3: f64,
        var_qd_fp4s_dn4: f64,
        var_qd_fp4s_dn5: f64,
        var_qd_fp4s_dn6: f64,
        var_qd_fp4s_dn7: f64,
        var_qd_fp4s_dn8: f64,
        var_qd_fp4s_dn9: f64,
        var_qg_fp4s: f64,
        var_qg_fp4s_dn0: f64,
        var_qg_fp4s_dn1: f64,
        var_qg_fp4s_dn12: f64,
        var_qg_fp4s_dn14: f64,
        var_qg_fp4s_dn15: f64,
        var_qg_fp4s_dn16: f64,
        var_qg_fp4s_dn17: f64,
        var_qg_fp4s_dn18: f64,
        var_qg_fp4s_dn19: f64,
        var_qg_fp4s_dn2: f64,
        var_qg_fp4s_dn20: f64,
        var_qg_fp4s_dn21: f64,
        var_qg_fp4s_dn22: f64,
        var_qg_fp4s_dn3: f64,
        var_qg_fp4s_dn4: f64,
        var_qg_fp4s_dn5: f64,
        var_qg_fp4s_dn6: f64,
        var_qg_fp4s_dn7: f64,
        var_qg_fp4s_dn8: f64,
        var_qg_fp4s_dn9: f64,
    ) {
        let (eq205_e2573, eq205_e2573_d_n0, eq205_e2573_d_n1, eq205_e2573_d_n2, eq205_e2573_d_n3, eq205_e2573_d_n4, eq205_e2573_d_n5, eq205_e2573_d_n6, eq205_e2573_d_n7, eq205_e2573_d_n8, eq205_e2573_d_n9, eq205_e2573_d_n12, eq205_e2573_d_n14, eq205_e2573_d_n15, eq205_e2573_d_n16, eq205_e2573_d_n17, eq205_e2573_d_n18, eq205_e2573_d_n19, eq205_e2573_d_n20, eq205_e2573_d_n21, eq205_e2573_d_n22, eq205_e2573_q,) = {
    if (((var_guard571 != 0.0) && (var_guard572 != 0.0)) && (var_guard573 != 0.0)) {
        let eq205_e2570_q: f64 = var_qg_fp4s;
        let eq205_e2571: f64 = (p.p7 * var_qg_fp4s);
        let eq205_e2571_d_n0: f64 = (p.p7 * var_qg_fp4s_dn0);
        let eq205_e2571_d_n1: f64 = (p.p7 * var_qg_fp4s_dn1);
        let eq205_e2571_d_n2: f64 = (p.p7 * var_qg_fp4s_dn2);
        let eq205_e2571_d_n3: f64 = (p.p7 * var_qg_fp4s_dn3);
        let eq205_e2571_d_n4: f64 = (p.p7 * var_qg_fp4s_dn4);
        let eq205_e2571_d_n5: f64 = (p.p7 * var_qg_fp4s_dn5);
        let eq205_e2571_d_n6: f64 = (p.p7 * var_qg_fp4s_dn6);
        let eq205_e2571_d_n7: f64 = (p.p7 * var_qg_fp4s_dn7);
        let eq205_e2571_d_n8: f64 = (p.p7 * var_qg_fp4s_dn8);
        let eq205_e2571_d_n9: f64 = (p.p7 * var_qg_fp4s_dn9);
        let eq205_e2571_d_n12: f64 = (p.p7 * var_qg_fp4s_dn12);
        let eq205_e2571_d_n14: f64 = (p.p7 * var_qg_fp4s_dn14);
        let eq205_e2571_d_n15: f64 = (p.p7 * var_qg_fp4s_dn15);
        let eq205_e2571_d_n16: f64 = (p.p7 * var_qg_fp4s_dn16);
        let eq205_e2571_d_n17: f64 = (p.p7 * var_qg_fp4s_dn17);
        let eq205_e2571_d_n18: f64 = (p.p7 * var_qg_fp4s_dn18);
        let eq205_e2571_d_n19: f64 = (p.p7 * var_qg_fp4s_dn19);
        let eq205_e2571_d_n20: f64 = (p.p7 * var_qg_fp4s_dn20);
        let eq205_e2571_d_n21: f64 = (p.p7 * var_qg_fp4s_dn21);
        let eq205_e2571_d_n22: f64 = (p.p7 * var_qg_fp4s_dn22);
        let eq205_e2571_q: f64 = (p.p7 * eq205_e2570_q);
        (eq205_e2571, eq205_e2571_d_n0, eq205_e2571_d_n1, eq205_e2571_d_n2, eq205_e2571_d_n3, eq205_e2571_d_n4, eq205_e2571_d_n5, eq205_e2571_d_n6, eq205_e2571_d_n7, eq205_e2571_d_n8, eq205_e2571_d_n9, eq205_e2571_d_n12, eq205_e2571_d_n14, eq205_e2571_d_n15, eq205_e2571_d_n16, eq205_e2571_d_n17, eq205_e2571_d_n18, eq205_e2571_d_n19, eq205_e2571_d_n20, eq205_e2571_d_n21, eq205_e2571_d_n22, eq205_e2571_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq205_reactive_node_derivatives: [f64; 23] = [eq205_e2573_d_n0, eq205_e2573_d_n1, eq205_e2573_d_n2, eq205_e2573_d_n3, eq205_e2573_d_n4, eq205_e2573_d_n5, eq205_e2573_d_n6, eq205_e2573_d_n7, eq205_e2573_d_n8, eq205_e2573_d_n9, 0.0, 0.0, eq205_e2573_d_n12, 0.0, eq205_e2573_d_n14, eq205_e2573_d_n15, eq205_e2573_d_n16, eq205_e2573_d_n17, eq205_e2573_d_n18, eq205_e2573_d_n19, eq205_e2573_d_n20, eq205_e2573_d_n21, eq205_e2573_d_n22];
        let eq205_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[22]),
            nodes,
            &eq205_reactive_node_derivatives,
            branches,
            &eq205_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq206_e2586, eq206_e2586_d_n0, eq206_e2586_d_n1, eq206_e2586_d_n2, eq206_e2586_d_n3, eq206_e2586_d_n4, eq206_e2586_d_n5, eq206_e2586_d_n6, eq206_e2586_d_n7, eq206_e2586_d_n8, eq206_e2586_d_n9, eq206_e2586_d_n12, eq206_e2586_d_n14, eq206_e2586_d_n15, eq206_e2586_d_n16, eq206_e2586_d_n17, eq206_e2586_d_n18, eq206_e2586_d_n19, eq206_e2586_d_n20, eq206_e2586_d_n21, eq206_e2586_d_n22, eq206_e2586_q,) = {
    if (((var_guard571 != 0.0) && (var_guard572 != 0.0)) && (var_guard573 != 0.0)) {
        let eq206_e2581_q: f64 = var_qg_fp4s;
        let eq206_e2582: f64 = (p.p7 * var_qg_fp4s);
        let eq206_e2582_d_n0: f64 = (p.p7 * var_qg_fp4s_dn0);
        let eq206_e2582_d_n1: f64 = (p.p7 * var_qg_fp4s_dn1);
        let eq206_e2582_d_n2: f64 = (p.p7 * var_qg_fp4s_dn2);
        let eq206_e2582_d_n3: f64 = (p.p7 * var_qg_fp4s_dn3);
        let eq206_e2582_d_n4: f64 = (p.p7 * var_qg_fp4s_dn4);
        let eq206_e2582_d_n5: f64 = (p.p7 * var_qg_fp4s_dn5);
        let eq206_e2582_d_n6: f64 = (p.p7 * var_qg_fp4s_dn6);
        let eq206_e2582_d_n7: f64 = (p.p7 * var_qg_fp4s_dn7);
        let eq206_e2582_d_n8: f64 = (p.p7 * var_qg_fp4s_dn8);
        let eq206_e2582_d_n9: f64 = (p.p7 * var_qg_fp4s_dn9);
        let eq206_e2582_d_n12: f64 = (p.p7 * var_qg_fp4s_dn12);
        let eq206_e2582_d_n14: f64 = (p.p7 * var_qg_fp4s_dn14);
        let eq206_e2582_d_n15: f64 = (p.p7 * var_qg_fp4s_dn15);
        let eq206_e2582_d_n16: f64 = (p.p7 * var_qg_fp4s_dn16);
        let eq206_e2582_d_n17: f64 = (p.p7 * var_qg_fp4s_dn17);
        let eq206_e2582_d_n18: f64 = (p.p7 * var_qg_fp4s_dn18);
        let eq206_e2582_d_n19: f64 = (p.p7 * var_qg_fp4s_dn19);
        let eq206_e2582_d_n20: f64 = (p.p7 * var_qg_fp4s_dn20);
        let eq206_e2582_d_n21: f64 = (p.p7 * var_qg_fp4s_dn21);
        let eq206_e2582_d_n22: f64 = (p.p7 * var_qg_fp4s_dn22);
        let eq206_e2582_q: f64 = (p.p7 * eq206_e2581_q);
        let eq206_e2584: f64 = (eq206_e2582 * p.p249);
        let eq206_e2584_d_n0: f64 = (eq206_e2582_d_n0 * p.p249);
        let eq206_e2584_d_n1: f64 = (eq206_e2582_d_n1 * p.p249);
        let eq206_e2584_d_n2: f64 = (eq206_e2582_d_n2 * p.p249);
        let eq206_e2584_d_n3: f64 = (eq206_e2582_d_n3 * p.p249);
        let eq206_e2584_d_n4: f64 = (eq206_e2582_d_n4 * p.p249);
        let eq206_e2584_d_n5: f64 = (eq206_e2582_d_n5 * p.p249);
        let eq206_e2584_d_n6: f64 = (eq206_e2582_d_n6 * p.p249);
        let eq206_e2584_d_n7: f64 = (eq206_e2582_d_n7 * p.p249);
        let eq206_e2584_d_n8: f64 = (eq206_e2582_d_n8 * p.p249);
        let eq206_e2584_d_n9: f64 = (eq206_e2582_d_n9 * p.p249);
        let eq206_e2584_d_n12: f64 = (eq206_e2582_d_n12 * p.p249);
        let eq206_e2584_d_n14: f64 = (eq206_e2582_d_n14 * p.p249);
        let eq206_e2584_d_n15: f64 = (eq206_e2582_d_n15 * p.p249);
        let eq206_e2584_d_n16: f64 = (eq206_e2582_d_n16 * p.p249);
        let eq206_e2584_d_n17: f64 = (eq206_e2582_d_n17 * p.p249);
        let eq206_e2584_d_n18: f64 = (eq206_e2582_d_n18 * p.p249);
        let eq206_e2584_d_n19: f64 = (eq206_e2582_d_n19 * p.p249);
        let eq206_e2584_d_n20: f64 = (eq206_e2582_d_n20 * p.p249);
        let eq206_e2584_d_n21: f64 = (eq206_e2582_d_n21 * p.p249);
        let eq206_e2584_d_n22: f64 = (eq206_e2582_d_n22 * p.p249);
        let eq206_e2584_q: f64 = (eq206_e2582_q * p.p249);
        (eq206_e2584, eq206_e2584_d_n0, eq206_e2584_d_n1, eq206_e2584_d_n2, eq206_e2584_d_n3, eq206_e2584_d_n4, eq206_e2584_d_n5, eq206_e2584_d_n6, eq206_e2584_d_n7, eq206_e2584_d_n8, eq206_e2584_d_n9, eq206_e2584_d_n12, eq206_e2584_d_n14, eq206_e2584_d_n15, eq206_e2584_d_n16, eq206_e2584_d_n17, eq206_e2584_d_n18, eq206_e2584_d_n19, eq206_e2584_d_n20, eq206_e2584_d_n21, eq206_e2584_d_n22, eq206_e2584_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq206_reactive_node_derivatives: [f64; 23] = [eq206_e2586_d_n0, eq206_e2586_d_n1, eq206_e2586_d_n2, eq206_e2586_d_n3, eq206_e2586_d_n4, eq206_e2586_d_n5, eq206_e2586_d_n6, eq206_e2586_d_n7, eq206_e2586_d_n8, eq206_e2586_d_n9, 0.0, 0.0, eq206_e2586_d_n12, 0.0, eq206_e2586_d_n14, eq206_e2586_d_n15, eq206_e2586_d_n16, eq206_e2586_d_n17, eq206_e2586_d_n18, eq206_e2586_d_n19, eq206_e2586_d_n20, eq206_e2586_d_n21, eq206_e2586_d_n22];
        let eq206_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[22]),
            nodes,
            &eq206_reactive_node_derivatives,
            branches,
            &eq206_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq207_e2598, eq207_e2598_d_n0, eq207_e2598_d_n1, eq207_e2598_d_n2, eq207_e2598_d_n3, eq207_e2598_d_n4, eq207_e2598_d_n5, eq207_e2598_d_n6, eq207_e2598_d_n7, eq207_e2598_d_n8, eq207_e2598_d_n9, eq207_e2598_d_n12, eq207_e2598_d_n14, eq207_e2598_d_n15, eq207_e2598_d_n16, eq207_e2598_d_n17, eq207_e2598_d_n18, eq207_e2598_d_n19, eq207_e2598_d_n20, eq207_e2598_d_n21, eq207_e2598_d_n22, eq207_e2598_q,) = {
    if (((var_guard571 != 0.0) && (var_guard572 != 0.0)) && (var_guard573 == 0.0)) {
        let eq207_e2595_q: f64 = var_qg_fp4s;
        let eq207_e2596: f64 = (p.p7 * var_qg_fp4s);
        let eq207_e2596_d_n0: f64 = (p.p7 * var_qg_fp4s_dn0);
        let eq207_e2596_d_n1: f64 = (p.p7 * var_qg_fp4s_dn1);
        let eq207_e2596_d_n2: f64 = (p.p7 * var_qg_fp4s_dn2);
        let eq207_e2596_d_n3: f64 = (p.p7 * var_qg_fp4s_dn3);
        let eq207_e2596_d_n4: f64 = (p.p7 * var_qg_fp4s_dn4);
        let eq207_e2596_d_n5: f64 = (p.p7 * var_qg_fp4s_dn5);
        let eq207_e2596_d_n6: f64 = (p.p7 * var_qg_fp4s_dn6);
        let eq207_e2596_d_n7: f64 = (p.p7 * var_qg_fp4s_dn7);
        let eq207_e2596_d_n8: f64 = (p.p7 * var_qg_fp4s_dn8);
        let eq207_e2596_d_n9: f64 = (p.p7 * var_qg_fp4s_dn9);
        let eq207_e2596_d_n12: f64 = (p.p7 * var_qg_fp4s_dn12);
        let eq207_e2596_d_n14: f64 = (p.p7 * var_qg_fp4s_dn14);
        let eq207_e2596_d_n15: f64 = (p.p7 * var_qg_fp4s_dn15);
        let eq207_e2596_d_n16: f64 = (p.p7 * var_qg_fp4s_dn16);
        let eq207_e2596_d_n17: f64 = (p.p7 * var_qg_fp4s_dn17);
        let eq207_e2596_d_n18: f64 = (p.p7 * var_qg_fp4s_dn18);
        let eq207_e2596_d_n19: f64 = (p.p7 * var_qg_fp4s_dn19);
        let eq207_e2596_d_n20: f64 = (p.p7 * var_qg_fp4s_dn20);
        let eq207_e2596_d_n21: f64 = (p.p7 * var_qg_fp4s_dn21);
        let eq207_e2596_d_n22: f64 = (p.p7 * var_qg_fp4s_dn22);
        let eq207_e2596_q: f64 = (p.p7 * eq207_e2595_q);
        (eq207_e2596, eq207_e2596_d_n0, eq207_e2596_d_n1, eq207_e2596_d_n2, eq207_e2596_d_n3, eq207_e2596_d_n4, eq207_e2596_d_n5, eq207_e2596_d_n6, eq207_e2596_d_n7, eq207_e2596_d_n8, eq207_e2596_d_n9, eq207_e2596_d_n12, eq207_e2596_d_n14, eq207_e2596_d_n15, eq207_e2596_d_n16, eq207_e2596_d_n17, eq207_e2596_d_n18, eq207_e2596_d_n19, eq207_e2596_d_n20, eq207_e2596_d_n21, eq207_e2596_d_n22, eq207_e2596_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq207_reactive_node_derivatives: [f64; 23] = [eq207_e2598_d_n0, eq207_e2598_d_n1, eq207_e2598_d_n2, eq207_e2598_d_n3, eq207_e2598_d_n4, eq207_e2598_d_n5, eq207_e2598_d_n6, eq207_e2598_d_n7, eq207_e2598_d_n8, eq207_e2598_d_n9, 0.0, 0.0, eq207_e2598_d_n12, 0.0, eq207_e2598_d_n14, eq207_e2598_d_n15, eq207_e2598_d_n16, eq207_e2598_d_n17, eq207_e2598_d_n18, eq207_e2598_d_n19, eq207_e2598_d_n20, eq207_e2598_d_n21, eq207_e2598_d_n22];
        let eq207_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[22]),
            nodes,
            &eq207_reactive_node_derivatives,
            branches,
            &eq207_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq208_e2612, eq208_e2612_d_n0, eq208_e2612_d_n1, eq208_e2612_d_n2, eq208_e2612_d_n3, eq208_e2612_d_n4, eq208_e2612_d_n5, eq208_e2612_d_n6, eq208_e2612_d_n7, eq208_e2612_d_n8, eq208_e2612_d_n9, eq208_e2612_d_n12, eq208_e2612_d_n14, eq208_e2612_d_n15, eq208_e2612_d_n16, eq208_e2612_d_n17, eq208_e2612_d_n18, eq208_e2612_d_n19, eq208_e2612_d_n20, eq208_e2612_d_n21, eq208_e2612_d_n22, eq208_e2612_q,) = {
    if (((var_guard571 != 0.0) && (var_guard572 != 0.0)) && (var_guard573 == 0.0)) {
        let eq208_e2607_q: f64 = var_qg_fp4s;
        let eq208_e2608: f64 = (p.p7 * var_qg_fp4s);
        let eq208_e2608_d_n0: f64 = (p.p7 * var_qg_fp4s_dn0);
        let eq208_e2608_d_n1: f64 = (p.p7 * var_qg_fp4s_dn1);
        let eq208_e2608_d_n2: f64 = (p.p7 * var_qg_fp4s_dn2);
        let eq208_e2608_d_n3: f64 = (p.p7 * var_qg_fp4s_dn3);
        let eq208_e2608_d_n4: f64 = (p.p7 * var_qg_fp4s_dn4);
        let eq208_e2608_d_n5: f64 = (p.p7 * var_qg_fp4s_dn5);
        let eq208_e2608_d_n6: f64 = (p.p7 * var_qg_fp4s_dn6);
        let eq208_e2608_d_n7: f64 = (p.p7 * var_qg_fp4s_dn7);
        let eq208_e2608_d_n8: f64 = (p.p7 * var_qg_fp4s_dn8);
        let eq208_e2608_d_n9: f64 = (p.p7 * var_qg_fp4s_dn9);
        let eq208_e2608_d_n12: f64 = (p.p7 * var_qg_fp4s_dn12);
        let eq208_e2608_d_n14: f64 = (p.p7 * var_qg_fp4s_dn14);
        let eq208_e2608_d_n15: f64 = (p.p7 * var_qg_fp4s_dn15);
        let eq208_e2608_d_n16: f64 = (p.p7 * var_qg_fp4s_dn16);
        let eq208_e2608_d_n17: f64 = (p.p7 * var_qg_fp4s_dn17);
        let eq208_e2608_d_n18: f64 = (p.p7 * var_qg_fp4s_dn18);
        let eq208_e2608_d_n19: f64 = (p.p7 * var_qg_fp4s_dn19);
        let eq208_e2608_d_n20: f64 = (p.p7 * var_qg_fp4s_dn20);
        let eq208_e2608_d_n21: f64 = (p.p7 * var_qg_fp4s_dn21);
        let eq208_e2608_d_n22: f64 = (p.p7 * var_qg_fp4s_dn22);
        let eq208_e2608_q: f64 = (p.p7 * eq208_e2607_q);
        let eq208_e2610: f64 = (eq208_e2608 * p.p249);
        let eq208_e2610_d_n0: f64 = (eq208_e2608_d_n0 * p.p249);
        let eq208_e2610_d_n1: f64 = (eq208_e2608_d_n1 * p.p249);
        let eq208_e2610_d_n2: f64 = (eq208_e2608_d_n2 * p.p249);
        let eq208_e2610_d_n3: f64 = (eq208_e2608_d_n3 * p.p249);
        let eq208_e2610_d_n4: f64 = (eq208_e2608_d_n4 * p.p249);
        let eq208_e2610_d_n5: f64 = (eq208_e2608_d_n5 * p.p249);
        let eq208_e2610_d_n6: f64 = (eq208_e2608_d_n6 * p.p249);
        let eq208_e2610_d_n7: f64 = (eq208_e2608_d_n7 * p.p249);
        let eq208_e2610_d_n8: f64 = (eq208_e2608_d_n8 * p.p249);
        let eq208_e2610_d_n9: f64 = (eq208_e2608_d_n9 * p.p249);
        let eq208_e2610_d_n12: f64 = (eq208_e2608_d_n12 * p.p249);
        let eq208_e2610_d_n14: f64 = (eq208_e2608_d_n14 * p.p249);
        let eq208_e2610_d_n15: f64 = (eq208_e2608_d_n15 * p.p249);
        let eq208_e2610_d_n16: f64 = (eq208_e2608_d_n16 * p.p249);
        let eq208_e2610_d_n17: f64 = (eq208_e2608_d_n17 * p.p249);
        let eq208_e2610_d_n18: f64 = (eq208_e2608_d_n18 * p.p249);
        let eq208_e2610_d_n19: f64 = (eq208_e2608_d_n19 * p.p249);
        let eq208_e2610_d_n20: f64 = (eq208_e2608_d_n20 * p.p249);
        let eq208_e2610_d_n21: f64 = (eq208_e2608_d_n21 * p.p249);
        let eq208_e2610_d_n22: f64 = (eq208_e2608_d_n22 * p.p249);
        let eq208_e2610_q: f64 = (eq208_e2608_q * p.p249);
        (eq208_e2610, eq208_e2610_d_n0, eq208_e2610_d_n1, eq208_e2610_d_n2, eq208_e2610_d_n3, eq208_e2610_d_n4, eq208_e2610_d_n5, eq208_e2610_d_n6, eq208_e2610_d_n7, eq208_e2610_d_n8, eq208_e2610_d_n9, eq208_e2610_d_n12, eq208_e2610_d_n14, eq208_e2610_d_n15, eq208_e2610_d_n16, eq208_e2610_d_n17, eq208_e2610_d_n18, eq208_e2610_d_n19, eq208_e2610_d_n20, eq208_e2610_d_n21, eq208_e2610_d_n22, eq208_e2610_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq208_reactive_node_derivatives: [f64; 23] = [eq208_e2612_d_n0, eq208_e2612_d_n1, eq208_e2612_d_n2, eq208_e2612_d_n3, eq208_e2612_d_n4, eq208_e2612_d_n5, eq208_e2612_d_n6, eq208_e2612_d_n7, eq208_e2612_d_n8, eq208_e2612_d_n9, 0.0, 0.0, eq208_e2612_d_n12, 0.0, eq208_e2612_d_n14, eq208_e2612_d_n15, eq208_e2612_d_n16, eq208_e2612_d_n17, eq208_e2612_d_n18, eq208_e2612_d_n19, eq208_e2612_d_n20, eq208_e2612_d_n21, eq208_e2612_d_n22];
        let eq208_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[22]),
            nodes,
            &eq208_reactive_node_derivatives,
            branches,
            &eq208_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq209_e2623, eq209_e2623_d_n0, eq209_e2623_d_n1, eq209_e2623_d_n2, eq209_e2623_d_n3, eq209_e2623_d_n4, eq209_e2623_d_n5, eq209_e2623_d_n6, eq209_e2623_d_n7, eq209_e2623_d_n8, eq209_e2623_d_n9, eq209_e2623_d_n12, eq209_e2623_d_n14, eq209_e2623_d_n15, eq209_e2623_d_n16, eq209_e2623_d_n17, eq209_e2623_d_n18, eq209_e2623_d_n19, eq209_e2623_d_n20, eq209_e2623_d_n21, eq209_e2623_d_n22, eq209_e2623_q,) = {
    if ((var_guard571 != 0.0) && (var_guard572 != 0.0)) {
        let eq209_e2619: f64 = (p.p254 * var_qg_fp4s);
        let eq209_e2619_d_n0: f64 = (p.p254 * var_qg_fp4s_dn0);
        let eq209_e2619_d_n1: f64 = (p.p254 * var_qg_fp4s_dn1);
        let eq209_e2619_d_n2: f64 = (p.p254 * var_qg_fp4s_dn2);
        let eq209_e2619_d_n3: f64 = (p.p254 * var_qg_fp4s_dn3);
        let eq209_e2619_d_n4: f64 = (p.p254 * var_qg_fp4s_dn4);
        let eq209_e2619_d_n5: f64 = (p.p254 * var_qg_fp4s_dn5);
        let eq209_e2619_d_n6: f64 = (p.p254 * var_qg_fp4s_dn6);
        let eq209_e2619_d_n7: f64 = (p.p254 * var_qg_fp4s_dn7);
        let eq209_e2619_d_n8: f64 = (p.p254 * var_qg_fp4s_dn8);
        let eq209_e2619_d_n9: f64 = (p.p254 * var_qg_fp4s_dn9);
        let eq209_e2619_d_n12: f64 = (p.p254 * var_qg_fp4s_dn12);
        let eq209_e2619_d_n14: f64 = (p.p254 * var_qg_fp4s_dn14);
        let eq209_e2619_d_n15: f64 = (p.p254 * var_qg_fp4s_dn15);
        let eq209_e2619_d_n16: f64 = (p.p254 * var_qg_fp4s_dn16);
        let eq209_e2619_d_n17: f64 = (p.p254 * var_qg_fp4s_dn17);
        let eq209_e2619_d_n18: f64 = (p.p254 * var_qg_fp4s_dn18);
        let eq209_e2619_d_n19: f64 = (p.p254 * var_qg_fp4s_dn19);
        let eq209_e2619_d_n20: f64 = (p.p254 * var_qg_fp4s_dn20);
        let eq209_e2619_d_n21: f64 = (p.p254 * var_qg_fp4s_dn21);
        let eq209_e2619_d_n22: f64 = (p.p254 * var_qg_fp4s_dn22);
        let eq209_e2620_q: f64 = eq209_e2619;
        let eq209_e2621: f64 = (p.p7 * eq209_e2619);
        let eq209_e2621_d_n0: f64 = (p.p7 * eq209_e2619_d_n0);
        let eq209_e2621_d_n1: f64 = (p.p7 * eq209_e2619_d_n1);
        let eq209_e2621_d_n2: f64 = (p.p7 * eq209_e2619_d_n2);
        let eq209_e2621_d_n3: f64 = (p.p7 * eq209_e2619_d_n3);
        let eq209_e2621_d_n4: f64 = (p.p7 * eq209_e2619_d_n4);
        let eq209_e2621_d_n5: f64 = (p.p7 * eq209_e2619_d_n5);
        let eq209_e2621_d_n6: f64 = (p.p7 * eq209_e2619_d_n6);
        let eq209_e2621_d_n7: f64 = (p.p7 * eq209_e2619_d_n7);
        let eq209_e2621_d_n8: f64 = (p.p7 * eq209_e2619_d_n8);
        let eq209_e2621_d_n9: f64 = (p.p7 * eq209_e2619_d_n9);
        let eq209_e2621_d_n12: f64 = (p.p7 * eq209_e2619_d_n12);
        let eq209_e2621_d_n14: f64 = (p.p7 * eq209_e2619_d_n14);
        let eq209_e2621_d_n15: f64 = (p.p7 * eq209_e2619_d_n15);
        let eq209_e2621_d_n16: f64 = (p.p7 * eq209_e2619_d_n16);
        let eq209_e2621_d_n17: f64 = (p.p7 * eq209_e2619_d_n17);
        let eq209_e2621_d_n18: f64 = (p.p7 * eq209_e2619_d_n18);
        let eq209_e2621_d_n19: f64 = (p.p7 * eq209_e2619_d_n19);
        let eq209_e2621_d_n20: f64 = (p.p7 * eq209_e2619_d_n20);
        let eq209_e2621_d_n21: f64 = (p.p7 * eq209_e2619_d_n21);
        let eq209_e2621_d_n22: f64 = (p.p7 * eq209_e2619_d_n22);
        let eq209_e2621_q: f64 = (p.p7 * eq209_e2620_q);
        (eq209_e2621, eq209_e2621_d_n0, eq209_e2621_d_n1, eq209_e2621_d_n2, eq209_e2621_d_n3, eq209_e2621_d_n4, eq209_e2621_d_n5, eq209_e2621_d_n6, eq209_e2621_d_n7, eq209_e2621_d_n8, eq209_e2621_d_n9, eq209_e2621_d_n12, eq209_e2621_d_n14, eq209_e2621_d_n15, eq209_e2621_d_n16, eq209_e2621_d_n17, eq209_e2621_d_n18, eq209_e2621_d_n19, eq209_e2621_d_n20, eq209_e2621_d_n21, eq209_e2621_d_n22, eq209_e2621_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq209_reactive_node_derivatives: [f64; 23] = [eq209_e2623_d_n0, eq209_e2623_d_n1, eq209_e2623_d_n2, eq209_e2623_d_n3, eq209_e2623_d_n4, eq209_e2623_d_n5, eq209_e2623_d_n6, eq209_e2623_d_n7, eq209_e2623_d_n8, eq209_e2623_d_n9, 0.0, 0.0, eq209_e2623_d_n12, 0.0, eq209_e2623_d_n14, eq209_e2623_d_n15, eq209_e2623_d_n16, eq209_e2623_d_n17, eq209_e2623_d_n18, eq209_e2623_d_n19, eq209_e2623_d_n20, eq209_e2623_d_n21, eq209_e2623_d_n22];
        let eq209_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[22]),
            nodes,
            &eq209_reactive_node_derivatives,
            branches,
            &eq209_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq210_e2633, eq210_e2633_d_n0, eq210_e2633_d_n1, eq210_e2633_d_n2, eq210_e2633_d_n3, eq210_e2633_d_n4, eq210_e2633_d_n5, eq210_e2633_d_n6, eq210_e2633_d_n7, eq210_e2633_d_n8, eq210_e2633_d_n9, eq210_e2633_d_n12, eq210_e2633_d_n14, eq210_e2633_d_n15, eq210_e2633_d_n16, eq210_e2633_d_n17, eq210_e2633_d_n18, eq210_e2633_d_n19, eq210_e2633_d_n20, eq210_e2633_d_n21, eq210_e2633_d_n22, eq210_e2633_q,) = {
    if ((var_guard571 == 0.0) && (var_guard574 != 0.0)) {
        let eq210_e2630_q: f64 = var_qd_fp4s;
        let eq210_e2631: f64 = (p.p7 * var_qd_fp4s);
        let eq210_e2631_d_n0: f64 = (p.p7 * var_qd_fp4s_dn0);
        let eq210_e2631_d_n1: f64 = (p.p7 * var_qd_fp4s_dn1);
        let eq210_e2631_d_n2: f64 = (p.p7 * var_qd_fp4s_dn2);
        let eq210_e2631_d_n3: f64 = (p.p7 * var_qd_fp4s_dn3);
        let eq210_e2631_d_n4: f64 = (p.p7 * var_qd_fp4s_dn4);
        let eq210_e2631_d_n5: f64 = (p.p7 * var_qd_fp4s_dn5);
        let eq210_e2631_d_n6: f64 = (p.p7 * var_qd_fp4s_dn6);
        let eq210_e2631_d_n7: f64 = (p.p7 * var_qd_fp4s_dn7);
        let eq210_e2631_d_n8: f64 = (p.p7 * var_qd_fp4s_dn8);
        let eq210_e2631_d_n9: f64 = (p.p7 * var_qd_fp4s_dn9);
        let eq210_e2631_d_n12: f64 = (p.p7 * var_qd_fp4s_dn12);
        let eq210_e2631_d_n14: f64 = (p.p7 * var_qd_fp4s_dn14);
        let eq210_e2631_d_n15: f64 = (p.p7 * var_qd_fp4s_dn15);
        let eq210_e2631_d_n16: f64 = (p.p7 * var_qd_fp4s_dn16);
        let eq210_e2631_d_n17: f64 = (p.p7 * var_qd_fp4s_dn17);
        let eq210_e2631_d_n18: f64 = (p.p7 * var_qd_fp4s_dn18);
        let eq210_e2631_d_n19: f64 = (p.p7 * var_qd_fp4s_dn19);
        let eq210_e2631_d_n20: f64 = (p.p7 * var_qd_fp4s_dn20);
        let eq210_e2631_d_n21: f64 = (p.p7 * var_qd_fp4s_dn21);
        let eq210_e2631_d_n22: f64 = (p.p7 * var_qd_fp4s_dn22);
        let eq210_e2631_q: f64 = (p.p7 * eq210_e2630_q);
        (eq210_e2631, eq210_e2631_d_n0, eq210_e2631_d_n1, eq210_e2631_d_n2, eq210_e2631_d_n3, eq210_e2631_d_n4, eq210_e2631_d_n5, eq210_e2631_d_n6, eq210_e2631_d_n7, eq210_e2631_d_n8, eq210_e2631_d_n9, eq210_e2631_d_n12, eq210_e2631_d_n14, eq210_e2631_d_n15, eq210_e2631_d_n16, eq210_e2631_d_n17, eq210_e2631_d_n18, eq210_e2631_d_n19, eq210_e2631_d_n20, eq210_e2631_d_n21, eq210_e2631_d_n22, eq210_e2631_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq210_reactive_node_derivatives: [f64; 23] = [eq210_e2633_d_n0, eq210_e2633_d_n1, eq210_e2633_d_n2, eq210_e2633_d_n3, eq210_e2633_d_n4, eq210_e2633_d_n5, eq210_e2633_d_n6, eq210_e2633_d_n7, eq210_e2633_d_n8, eq210_e2633_d_n9, 0.0, 0.0, eq210_e2633_d_n12, 0.0, eq210_e2633_d_n14, eq210_e2633_d_n15, eq210_e2633_d_n16, eq210_e2633_d_n17, eq210_e2633_d_n18, eq210_e2633_d_n19, eq210_e2633_d_n20, eq210_e2633_d_n21, eq210_e2633_d_n22];
        let eq210_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            nodes,
            &eq210_reactive_node_derivatives,
            branches,
            &eq210_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq211_e2645, eq211_e2645_d_n0, eq211_e2645_d_n1, eq211_e2645_d_n2, eq211_e2645_d_n3, eq211_e2645_d_n4, eq211_e2645_d_n5, eq211_e2645_d_n6, eq211_e2645_d_n7, eq211_e2645_d_n8, eq211_e2645_d_n9, eq211_e2645_d_n12, eq211_e2645_d_n14, eq211_e2645_d_n15, eq211_e2645_d_n16, eq211_e2645_d_n17, eq211_e2645_d_n18, eq211_e2645_d_n19, eq211_e2645_d_n20, eq211_e2645_d_n21, eq211_e2645_d_n22, eq211_e2645_q,) = {
    if (((var_guard571 == 0.0) && (var_guard574 != 0.0)) && (var_guard575 != 0.0)) {
        let eq211_e2642_q: f64 = var_qg_fp4s;
        let eq211_e2643: f64 = (p.p7 * var_qg_fp4s);
        let eq211_e2643_d_n0: f64 = (p.p7 * var_qg_fp4s_dn0);
        let eq211_e2643_d_n1: f64 = (p.p7 * var_qg_fp4s_dn1);
        let eq211_e2643_d_n2: f64 = (p.p7 * var_qg_fp4s_dn2);
        let eq211_e2643_d_n3: f64 = (p.p7 * var_qg_fp4s_dn3);
        let eq211_e2643_d_n4: f64 = (p.p7 * var_qg_fp4s_dn4);
        let eq211_e2643_d_n5: f64 = (p.p7 * var_qg_fp4s_dn5);
        let eq211_e2643_d_n6: f64 = (p.p7 * var_qg_fp4s_dn6);
        let eq211_e2643_d_n7: f64 = (p.p7 * var_qg_fp4s_dn7);
        let eq211_e2643_d_n8: f64 = (p.p7 * var_qg_fp4s_dn8);
        let eq211_e2643_d_n9: f64 = (p.p7 * var_qg_fp4s_dn9);
        let eq211_e2643_d_n12: f64 = (p.p7 * var_qg_fp4s_dn12);
        let eq211_e2643_d_n14: f64 = (p.p7 * var_qg_fp4s_dn14);
        let eq211_e2643_d_n15: f64 = (p.p7 * var_qg_fp4s_dn15);
        let eq211_e2643_d_n16: f64 = (p.p7 * var_qg_fp4s_dn16);
        let eq211_e2643_d_n17: f64 = (p.p7 * var_qg_fp4s_dn17);
        let eq211_e2643_d_n18: f64 = (p.p7 * var_qg_fp4s_dn18);
        let eq211_e2643_d_n19: f64 = (p.p7 * var_qg_fp4s_dn19);
        let eq211_e2643_d_n20: f64 = (p.p7 * var_qg_fp4s_dn20);
        let eq211_e2643_d_n21: f64 = (p.p7 * var_qg_fp4s_dn21);
        let eq211_e2643_d_n22: f64 = (p.p7 * var_qg_fp4s_dn22);
        let eq211_e2643_q: f64 = (p.p7 * eq211_e2642_q);
        (eq211_e2643, eq211_e2643_d_n0, eq211_e2643_d_n1, eq211_e2643_d_n2, eq211_e2643_d_n3, eq211_e2643_d_n4, eq211_e2643_d_n5, eq211_e2643_d_n6, eq211_e2643_d_n7, eq211_e2643_d_n8, eq211_e2643_d_n9, eq211_e2643_d_n12, eq211_e2643_d_n14, eq211_e2643_d_n15, eq211_e2643_d_n16, eq211_e2643_d_n17, eq211_e2643_d_n18, eq211_e2643_d_n19, eq211_e2643_d_n20, eq211_e2643_d_n21, eq211_e2643_d_n22, eq211_e2643_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq211_reactive_node_derivatives: [f64; 23] = [eq211_e2645_d_n0, eq211_e2645_d_n1, eq211_e2645_d_n2, eq211_e2645_d_n3, eq211_e2645_d_n4, eq211_e2645_d_n5, eq211_e2645_d_n6, eq211_e2645_d_n7, eq211_e2645_d_n8, eq211_e2645_d_n9, 0.0, 0.0, eq211_e2645_d_n12, 0.0, eq211_e2645_d_n14, eq211_e2645_d_n15, eq211_e2645_d_n16, eq211_e2645_d_n17, eq211_e2645_d_n18, eq211_e2645_d_n19, eq211_e2645_d_n20, eq211_e2645_d_n21, eq211_e2645_d_n22];
        let eq211_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[2]),
            nodes,
            &eq211_reactive_node_derivatives,
            branches,
            &eq211_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq212_e2659, eq212_e2659_d_n0, eq212_e2659_d_n1, eq212_e2659_d_n2, eq212_e2659_d_n3, eq212_e2659_d_n4, eq212_e2659_d_n5, eq212_e2659_d_n6, eq212_e2659_d_n7, eq212_e2659_d_n8, eq212_e2659_d_n9, eq212_e2659_d_n12, eq212_e2659_d_n14, eq212_e2659_d_n15, eq212_e2659_d_n16, eq212_e2659_d_n17, eq212_e2659_d_n18, eq212_e2659_d_n19, eq212_e2659_d_n20, eq212_e2659_d_n21, eq212_e2659_d_n22, eq212_e2659_q,) = {
    if (((var_guard571 == 0.0) && (var_guard574 != 0.0)) && (var_guard575 != 0.0)) {
        let eq212_e2654_q: f64 = var_qg_fp4s;
        let eq212_e2655: f64 = (p.p7 * var_qg_fp4s);
        let eq212_e2655_d_n0: f64 = (p.p7 * var_qg_fp4s_dn0);
        let eq212_e2655_d_n1: f64 = (p.p7 * var_qg_fp4s_dn1);
        let eq212_e2655_d_n2: f64 = (p.p7 * var_qg_fp4s_dn2);
        let eq212_e2655_d_n3: f64 = (p.p7 * var_qg_fp4s_dn3);
        let eq212_e2655_d_n4: f64 = (p.p7 * var_qg_fp4s_dn4);
        let eq212_e2655_d_n5: f64 = (p.p7 * var_qg_fp4s_dn5);
        let eq212_e2655_d_n6: f64 = (p.p7 * var_qg_fp4s_dn6);
        let eq212_e2655_d_n7: f64 = (p.p7 * var_qg_fp4s_dn7);
        let eq212_e2655_d_n8: f64 = (p.p7 * var_qg_fp4s_dn8);
        let eq212_e2655_d_n9: f64 = (p.p7 * var_qg_fp4s_dn9);
        let eq212_e2655_d_n12: f64 = (p.p7 * var_qg_fp4s_dn12);
        let eq212_e2655_d_n14: f64 = (p.p7 * var_qg_fp4s_dn14);
        let eq212_e2655_d_n15: f64 = (p.p7 * var_qg_fp4s_dn15);
        let eq212_e2655_d_n16: f64 = (p.p7 * var_qg_fp4s_dn16);
        let eq212_e2655_d_n17: f64 = (p.p7 * var_qg_fp4s_dn17);
        let eq212_e2655_d_n18: f64 = (p.p7 * var_qg_fp4s_dn18);
        let eq212_e2655_d_n19: f64 = (p.p7 * var_qg_fp4s_dn19);
        let eq212_e2655_d_n20: f64 = (p.p7 * var_qg_fp4s_dn20);
        let eq212_e2655_d_n21: f64 = (p.p7 * var_qg_fp4s_dn21);
        let eq212_e2655_d_n22: f64 = (p.p7 * var_qg_fp4s_dn22);
        let eq212_e2655_q: f64 = (p.p7 * eq212_e2654_q);
        let eq212_e2657: f64 = (eq212_e2655 * p.p249);
        let eq212_e2657_d_n0: f64 = (eq212_e2655_d_n0 * p.p249);
        let eq212_e2657_d_n1: f64 = (eq212_e2655_d_n1 * p.p249);
        let eq212_e2657_d_n2: f64 = (eq212_e2655_d_n2 * p.p249);
        let eq212_e2657_d_n3: f64 = (eq212_e2655_d_n3 * p.p249);
        let eq212_e2657_d_n4: f64 = (eq212_e2655_d_n4 * p.p249);
        let eq212_e2657_d_n5: f64 = (eq212_e2655_d_n5 * p.p249);
        let eq212_e2657_d_n6: f64 = (eq212_e2655_d_n6 * p.p249);
        let eq212_e2657_d_n7: f64 = (eq212_e2655_d_n7 * p.p249);
        let eq212_e2657_d_n8: f64 = (eq212_e2655_d_n8 * p.p249);
        let eq212_e2657_d_n9: f64 = (eq212_e2655_d_n9 * p.p249);
        let eq212_e2657_d_n12: f64 = (eq212_e2655_d_n12 * p.p249);
        let eq212_e2657_d_n14: f64 = (eq212_e2655_d_n14 * p.p249);
        let eq212_e2657_d_n15: f64 = (eq212_e2655_d_n15 * p.p249);
        let eq212_e2657_d_n16: f64 = (eq212_e2655_d_n16 * p.p249);
        let eq212_e2657_d_n17: f64 = (eq212_e2655_d_n17 * p.p249);
        let eq212_e2657_d_n18: f64 = (eq212_e2655_d_n18 * p.p249);
        let eq212_e2657_d_n19: f64 = (eq212_e2655_d_n19 * p.p249);
        let eq212_e2657_d_n20: f64 = (eq212_e2655_d_n20 * p.p249);
        let eq212_e2657_d_n21: f64 = (eq212_e2655_d_n21 * p.p249);
        let eq212_e2657_d_n22: f64 = (eq212_e2655_d_n22 * p.p249);
        let eq212_e2657_q: f64 = (eq212_e2655_q * p.p249);
        (eq212_e2657, eq212_e2657_d_n0, eq212_e2657_d_n1, eq212_e2657_d_n2, eq212_e2657_d_n3, eq212_e2657_d_n4, eq212_e2657_d_n5, eq212_e2657_d_n6, eq212_e2657_d_n7, eq212_e2657_d_n8, eq212_e2657_d_n9, eq212_e2657_d_n12, eq212_e2657_d_n14, eq212_e2657_d_n15, eq212_e2657_d_n16, eq212_e2657_d_n17, eq212_e2657_d_n18, eq212_e2657_d_n19, eq212_e2657_d_n20, eq212_e2657_d_n21, eq212_e2657_d_n22, eq212_e2657_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq212_reactive_node_derivatives: [f64; 23] = [eq212_e2659_d_n0, eq212_e2659_d_n1, eq212_e2659_d_n2, eq212_e2659_d_n3, eq212_e2659_d_n4, eq212_e2659_d_n5, eq212_e2659_d_n6, eq212_e2659_d_n7, eq212_e2659_d_n8, eq212_e2659_d_n9, 0.0, 0.0, eq212_e2659_d_n12, 0.0, eq212_e2659_d_n14, eq212_e2659_d_n15, eq212_e2659_d_n16, eq212_e2659_d_n17, eq212_e2659_d_n18, eq212_e2659_d_n19, eq212_e2659_d_n20, eq212_e2659_d_n21, eq212_e2659_d_n22];
        let eq212_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            nodes,
            &eq212_reactive_node_derivatives,
            branches,
            &eq212_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq213_e2672, eq213_e2672_d_n0, eq213_e2672_d_n1, eq213_e2672_d_n2, eq213_e2672_d_n3, eq213_e2672_d_n4, eq213_e2672_d_n5, eq213_e2672_d_n6, eq213_e2672_d_n7, eq213_e2672_d_n8, eq213_e2672_d_n9, eq213_e2672_d_n12, eq213_e2672_d_n14, eq213_e2672_d_n15, eq213_e2672_d_n16, eq213_e2672_d_n17, eq213_e2672_d_n18, eq213_e2672_d_n19, eq213_e2672_d_n20, eq213_e2672_d_n21, eq213_e2672_d_n22, eq213_e2672_q,) = {
    if (((var_guard571 == 0.0) && (var_guard574 != 0.0)) && (var_guard575 == 0.0)) {
        let eq213_e2669_q: f64 = var_qg_fp4s;
        let eq213_e2670: f64 = (p.p7 * var_qg_fp4s);
        let eq213_e2670_d_n0: f64 = (p.p7 * var_qg_fp4s_dn0);
        let eq213_e2670_d_n1: f64 = (p.p7 * var_qg_fp4s_dn1);
        let eq213_e2670_d_n2: f64 = (p.p7 * var_qg_fp4s_dn2);
        let eq213_e2670_d_n3: f64 = (p.p7 * var_qg_fp4s_dn3);
        let eq213_e2670_d_n4: f64 = (p.p7 * var_qg_fp4s_dn4);
        let eq213_e2670_d_n5: f64 = (p.p7 * var_qg_fp4s_dn5);
        let eq213_e2670_d_n6: f64 = (p.p7 * var_qg_fp4s_dn6);
        let eq213_e2670_d_n7: f64 = (p.p7 * var_qg_fp4s_dn7);
        let eq213_e2670_d_n8: f64 = (p.p7 * var_qg_fp4s_dn8);
        let eq213_e2670_d_n9: f64 = (p.p7 * var_qg_fp4s_dn9);
        let eq213_e2670_d_n12: f64 = (p.p7 * var_qg_fp4s_dn12);
        let eq213_e2670_d_n14: f64 = (p.p7 * var_qg_fp4s_dn14);
        let eq213_e2670_d_n15: f64 = (p.p7 * var_qg_fp4s_dn15);
        let eq213_e2670_d_n16: f64 = (p.p7 * var_qg_fp4s_dn16);
        let eq213_e2670_d_n17: f64 = (p.p7 * var_qg_fp4s_dn17);
        let eq213_e2670_d_n18: f64 = (p.p7 * var_qg_fp4s_dn18);
        let eq213_e2670_d_n19: f64 = (p.p7 * var_qg_fp4s_dn19);
        let eq213_e2670_d_n20: f64 = (p.p7 * var_qg_fp4s_dn20);
        let eq213_e2670_d_n21: f64 = (p.p7 * var_qg_fp4s_dn21);
        let eq213_e2670_d_n22: f64 = (p.p7 * var_qg_fp4s_dn22);
        let eq213_e2670_q: f64 = (p.p7 * eq213_e2669_q);
        (eq213_e2670, eq213_e2670_d_n0, eq213_e2670_d_n1, eq213_e2670_d_n2, eq213_e2670_d_n3, eq213_e2670_d_n4, eq213_e2670_d_n5, eq213_e2670_d_n6, eq213_e2670_d_n7, eq213_e2670_d_n8, eq213_e2670_d_n9, eq213_e2670_d_n12, eq213_e2670_d_n14, eq213_e2670_d_n15, eq213_e2670_d_n16, eq213_e2670_d_n17, eq213_e2670_d_n18, eq213_e2670_d_n19, eq213_e2670_d_n20, eq213_e2670_d_n21, eq213_e2670_d_n22, eq213_e2670_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq213_reactive_node_derivatives: [f64; 23] = [eq213_e2672_d_n0, eq213_e2672_d_n1, eq213_e2672_d_n2, eq213_e2672_d_n3, eq213_e2672_d_n4, eq213_e2672_d_n5, eq213_e2672_d_n6, eq213_e2672_d_n7, eq213_e2672_d_n8, eq213_e2672_d_n9, 0.0, 0.0, eq213_e2672_d_n12, 0.0, eq213_e2672_d_n14, eq213_e2672_d_n15, eq213_e2672_d_n16, eq213_e2672_d_n17, eq213_e2672_d_n18, eq213_e2672_d_n19, eq213_e2672_d_n20, eq213_e2672_d_n21, eq213_e2672_d_n22];
        let eq213_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            nodes,
            &eq213_reactive_node_derivatives,
            branches,
            &eq213_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_11(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_guard571: f64,
        var_guard574: f64,
        var_guard575: f64,
        var_guard576: f64,
        var_qdep: f64,
        var_qdep_dn0: f64,
        var_qdep_dn1: f64,
        var_qdep_dn12: f64,
        var_qdep_dn14: f64,
        var_qdep_dn15: f64,
        var_qdep_dn16: f64,
        var_qdep_dn17: f64,
        var_qdep_dn18: f64,
        var_qdep_dn19: f64,
        var_qdep_dn2: f64,
        var_qdep_dn20: f64,
        var_qdep_dn21: f64,
        var_qdep_dn22: f64,
        var_qdep_dn3: f64,
        var_qdep_dn4: f64,
        var_qdep_dn5: f64,
        var_qdep_dn6: f64,
        var_qdep_dn7: f64,
        var_qdep_dn8: f64,
        var_qdep_dn9: f64,
        var_qfr: f64,
        var_qfr2: f64,
        var_qfr2_dn0: f64,
        var_qfr2_dn1: f64,
        var_qfr2_dn12: f64,
        var_qfr2_dn14: f64,
        var_qfr2_dn15: f64,
        var_qfr2_dn16: f64,
        var_qfr2_dn17: f64,
        var_qfr2_dn18: f64,
        var_qfr2_dn19: f64,
        var_qfr2_dn2: f64,
        var_qfr2_dn20: f64,
        var_qfr2_dn21: f64,
        var_qfr2_dn22: f64,
        var_qfr2_dn3: f64,
        var_qfr2_dn4: f64,
        var_qfr2_dn5: f64,
        var_qfr2_dn6: f64,
        var_qfr2_dn7: f64,
        var_qfr2_dn8: f64,
        var_qfr2_dn9: f64,
        var_qfr3: f64,
        var_qfr3_dn0: f64,
        var_qfr3_dn2: f64,
        var_qfr_dn0: f64,
        var_qfr_dn2: f64,
        var_qfr_dn4: f64,
        var_qg_fp4s: f64,
        var_qg_fp4s_dn0: f64,
        var_qg_fp4s_dn1: f64,
        var_qg_fp4s_dn12: f64,
        var_qg_fp4s_dn14: f64,
        var_qg_fp4s_dn15: f64,
        var_qg_fp4s_dn16: f64,
        var_qg_fp4s_dn17: f64,
        var_qg_fp4s_dn18: f64,
        var_qg_fp4s_dn19: f64,
        var_qg_fp4s_dn2: f64,
        var_qg_fp4s_dn20: f64,
        var_qg_fp4s_dn21: f64,
        var_qg_fp4s_dn22: f64,
        var_qg_fp4s_dn3: f64,
        var_qg_fp4s_dn4: f64,
        var_qg_fp4s_dn5: f64,
        var_qg_fp4s_dn6: f64,
        var_qg_fp4s_dn7: f64,
        var_qg_fp4s_dn8: f64,
        var_qg_fp4s_dn9: f64,
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq214_e2687, eq214_e2687_d_n0, eq214_e2687_d_n1, eq214_e2687_d_n2, eq214_e2687_d_n3, eq214_e2687_d_n4, eq214_e2687_d_n5, eq214_e2687_d_n6, eq214_e2687_d_n7, eq214_e2687_d_n8, eq214_e2687_d_n9, eq214_e2687_d_n12, eq214_e2687_d_n14, eq214_e2687_d_n15, eq214_e2687_d_n16, eq214_e2687_d_n17, eq214_e2687_d_n18, eq214_e2687_d_n19, eq214_e2687_d_n20, eq214_e2687_d_n21, eq214_e2687_d_n22, eq214_e2687_q,) = {
    if (((var_guard571 == 0.0) && (var_guard574 != 0.0)) && (var_guard575 == 0.0)) {
        let eq214_e2682_q: f64 = var_qg_fp4s;
        let eq214_e2683: f64 = (p.p7 * var_qg_fp4s);
        let eq214_e2683_d_n0: f64 = (p.p7 * var_qg_fp4s_dn0);
        let eq214_e2683_d_n1: f64 = (p.p7 * var_qg_fp4s_dn1);
        let eq214_e2683_d_n2: f64 = (p.p7 * var_qg_fp4s_dn2);
        let eq214_e2683_d_n3: f64 = (p.p7 * var_qg_fp4s_dn3);
        let eq214_e2683_d_n4: f64 = (p.p7 * var_qg_fp4s_dn4);
        let eq214_e2683_d_n5: f64 = (p.p7 * var_qg_fp4s_dn5);
        let eq214_e2683_d_n6: f64 = (p.p7 * var_qg_fp4s_dn6);
        let eq214_e2683_d_n7: f64 = (p.p7 * var_qg_fp4s_dn7);
        let eq214_e2683_d_n8: f64 = (p.p7 * var_qg_fp4s_dn8);
        let eq214_e2683_d_n9: f64 = (p.p7 * var_qg_fp4s_dn9);
        let eq214_e2683_d_n12: f64 = (p.p7 * var_qg_fp4s_dn12);
        let eq214_e2683_d_n14: f64 = (p.p7 * var_qg_fp4s_dn14);
        let eq214_e2683_d_n15: f64 = (p.p7 * var_qg_fp4s_dn15);
        let eq214_e2683_d_n16: f64 = (p.p7 * var_qg_fp4s_dn16);
        let eq214_e2683_d_n17: f64 = (p.p7 * var_qg_fp4s_dn17);
        let eq214_e2683_d_n18: f64 = (p.p7 * var_qg_fp4s_dn18);
        let eq214_e2683_d_n19: f64 = (p.p7 * var_qg_fp4s_dn19);
        let eq214_e2683_d_n20: f64 = (p.p7 * var_qg_fp4s_dn20);
        let eq214_e2683_d_n21: f64 = (p.p7 * var_qg_fp4s_dn21);
        let eq214_e2683_d_n22: f64 = (p.p7 * var_qg_fp4s_dn22);
        let eq214_e2683_q: f64 = (p.p7 * eq214_e2682_q);
        let eq214_e2685: f64 = (eq214_e2683 * p.p249);
        let eq214_e2685_d_n0: f64 = (eq214_e2683_d_n0 * p.p249);
        let eq214_e2685_d_n1: f64 = (eq214_e2683_d_n1 * p.p249);
        let eq214_e2685_d_n2: f64 = (eq214_e2683_d_n2 * p.p249);
        let eq214_e2685_d_n3: f64 = (eq214_e2683_d_n3 * p.p249);
        let eq214_e2685_d_n4: f64 = (eq214_e2683_d_n4 * p.p249);
        let eq214_e2685_d_n5: f64 = (eq214_e2683_d_n5 * p.p249);
        let eq214_e2685_d_n6: f64 = (eq214_e2683_d_n6 * p.p249);
        let eq214_e2685_d_n7: f64 = (eq214_e2683_d_n7 * p.p249);
        let eq214_e2685_d_n8: f64 = (eq214_e2683_d_n8 * p.p249);
        let eq214_e2685_d_n9: f64 = (eq214_e2683_d_n9 * p.p249);
        let eq214_e2685_d_n12: f64 = (eq214_e2683_d_n12 * p.p249);
        let eq214_e2685_d_n14: f64 = (eq214_e2683_d_n14 * p.p249);
        let eq214_e2685_d_n15: f64 = (eq214_e2683_d_n15 * p.p249);
        let eq214_e2685_d_n16: f64 = (eq214_e2683_d_n16 * p.p249);
        let eq214_e2685_d_n17: f64 = (eq214_e2683_d_n17 * p.p249);
        let eq214_e2685_d_n18: f64 = (eq214_e2683_d_n18 * p.p249);
        let eq214_e2685_d_n19: f64 = (eq214_e2683_d_n19 * p.p249);
        let eq214_e2685_d_n20: f64 = (eq214_e2683_d_n20 * p.p249);
        let eq214_e2685_d_n21: f64 = (eq214_e2683_d_n21 * p.p249);
        let eq214_e2685_d_n22: f64 = (eq214_e2683_d_n22 * p.p249);
        let eq214_e2685_q: f64 = (eq214_e2683_q * p.p249);
        (eq214_e2685, eq214_e2685_d_n0, eq214_e2685_d_n1, eq214_e2685_d_n2, eq214_e2685_d_n3, eq214_e2685_d_n4, eq214_e2685_d_n5, eq214_e2685_d_n6, eq214_e2685_d_n7, eq214_e2685_d_n8, eq214_e2685_d_n9, eq214_e2685_d_n12, eq214_e2685_d_n14, eq214_e2685_d_n15, eq214_e2685_d_n16, eq214_e2685_d_n17, eq214_e2685_d_n18, eq214_e2685_d_n19, eq214_e2685_d_n20, eq214_e2685_d_n21, eq214_e2685_d_n22, eq214_e2685_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq214_reactive_node_derivatives: [f64; 23] = [eq214_e2687_d_n0, eq214_e2687_d_n1, eq214_e2687_d_n2, eq214_e2687_d_n3, eq214_e2687_d_n4, eq214_e2687_d_n5, eq214_e2687_d_n6, eq214_e2687_d_n7, eq214_e2687_d_n8, eq214_e2687_d_n9, 0.0, 0.0, eq214_e2687_d_n12, 0.0, eq214_e2687_d_n14, eq214_e2687_d_n15, eq214_e2687_d_n16, eq214_e2687_d_n17, eq214_e2687_d_n18, eq214_e2687_d_n19, eq214_e2687_d_n20, eq214_e2687_d_n21, eq214_e2687_d_n22];
        let eq214_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq214_reactive_node_derivatives,
            branches,
            &eq214_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq215_e2699, eq215_e2699_d_n0, eq215_e2699_d_n1, eq215_e2699_d_n2, eq215_e2699_d_n3, eq215_e2699_d_n4, eq215_e2699_d_n5, eq215_e2699_d_n6, eq215_e2699_d_n7, eq215_e2699_d_n8, eq215_e2699_d_n9, eq215_e2699_d_n12, eq215_e2699_d_n14, eq215_e2699_d_n15, eq215_e2699_d_n16, eq215_e2699_d_n17, eq215_e2699_d_n18, eq215_e2699_d_n19, eq215_e2699_d_n20, eq215_e2699_d_n21, eq215_e2699_d_n22, eq215_e2699_q,) = {
    if ((var_guard571 == 0.0) && (var_guard574 != 0.0)) {
        let eq215_e2695: f64 = (p.p254 * var_qg_fp4s);
        let eq215_e2695_d_n0: f64 = (p.p254 * var_qg_fp4s_dn0);
        let eq215_e2695_d_n1: f64 = (p.p254 * var_qg_fp4s_dn1);
        let eq215_e2695_d_n2: f64 = (p.p254 * var_qg_fp4s_dn2);
        let eq215_e2695_d_n3: f64 = (p.p254 * var_qg_fp4s_dn3);
        let eq215_e2695_d_n4: f64 = (p.p254 * var_qg_fp4s_dn4);
        let eq215_e2695_d_n5: f64 = (p.p254 * var_qg_fp4s_dn5);
        let eq215_e2695_d_n6: f64 = (p.p254 * var_qg_fp4s_dn6);
        let eq215_e2695_d_n7: f64 = (p.p254 * var_qg_fp4s_dn7);
        let eq215_e2695_d_n8: f64 = (p.p254 * var_qg_fp4s_dn8);
        let eq215_e2695_d_n9: f64 = (p.p254 * var_qg_fp4s_dn9);
        let eq215_e2695_d_n12: f64 = (p.p254 * var_qg_fp4s_dn12);
        let eq215_e2695_d_n14: f64 = (p.p254 * var_qg_fp4s_dn14);
        let eq215_e2695_d_n15: f64 = (p.p254 * var_qg_fp4s_dn15);
        let eq215_e2695_d_n16: f64 = (p.p254 * var_qg_fp4s_dn16);
        let eq215_e2695_d_n17: f64 = (p.p254 * var_qg_fp4s_dn17);
        let eq215_e2695_d_n18: f64 = (p.p254 * var_qg_fp4s_dn18);
        let eq215_e2695_d_n19: f64 = (p.p254 * var_qg_fp4s_dn19);
        let eq215_e2695_d_n20: f64 = (p.p254 * var_qg_fp4s_dn20);
        let eq215_e2695_d_n21: f64 = (p.p254 * var_qg_fp4s_dn21);
        let eq215_e2695_d_n22: f64 = (p.p254 * var_qg_fp4s_dn22);
        let eq215_e2696_q: f64 = eq215_e2695;
        let eq215_e2697: f64 = (p.p7 * eq215_e2695);
        let eq215_e2697_d_n0: f64 = (p.p7 * eq215_e2695_d_n0);
        let eq215_e2697_d_n1: f64 = (p.p7 * eq215_e2695_d_n1);
        let eq215_e2697_d_n2: f64 = (p.p7 * eq215_e2695_d_n2);
        let eq215_e2697_d_n3: f64 = (p.p7 * eq215_e2695_d_n3);
        let eq215_e2697_d_n4: f64 = (p.p7 * eq215_e2695_d_n4);
        let eq215_e2697_d_n5: f64 = (p.p7 * eq215_e2695_d_n5);
        let eq215_e2697_d_n6: f64 = (p.p7 * eq215_e2695_d_n6);
        let eq215_e2697_d_n7: f64 = (p.p7 * eq215_e2695_d_n7);
        let eq215_e2697_d_n8: f64 = (p.p7 * eq215_e2695_d_n8);
        let eq215_e2697_d_n9: f64 = (p.p7 * eq215_e2695_d_n9);
        let eq215_e2697_d_n12: f64 = (p.p7 * eq215_e2695_d_n12);
        let eq215_e2697_d_n14: f64 = (p.p7 * eq215_e2695_d_n14);
        let eq215_e2697_d_n15: f64 = (p.p7 * eq215_e2695_d_n15);
        let eq215_e2697_d_n16: f64 = (p.p7 * eq215_e2695_d_n16);
        let eq215_e2697_d_n17: f64 = (p.p7 * eq215_e2695_d_n17);
        let eq215_e2697_d_n18: f64 = (p.p7 * eq215_e2695_d_n18);
        let eq215_e2697_d_n19: f64 = (p.p7 * eq215_e2695_d_n19);
        let eq215_e2697_d_n20: f64 = (p.p7 * eq215_e2695_d_n20);
        let eq215_e2697_d_n21: f64 = (p.p7 * eq215_e2695_d_n21);
        let eq215_e2697_d_n22: f64 = (p.p7 * eq215_e2695_d_n22);
        let eq215_e2697_q: f64 = (p.p7 * eq215_e2696_q);
        (eq215_e2697, eq215_e2697_d_n0, eq215_e2697_d_n1, eq215_e2697_d_n2, eq215_e2697_d_n3, eq215_e2697_d_n4, eq215_e2697_d_n5, eq215_e2697_d_n6, eq215_e2697_d_n7, eq215_e2697_d_n8, eq215_e2697_d_n9, eq215_e2697_d_n12, eq215_e2697_d_n14, eq215_e2697_d_n15, eq215_e2697_d_n16, eq215_e2697_d_n17, eq215_e2697_d_n18, eq215_e2697_d_n19, eq215_e2697_d_n20, eq215_e2697_d_n21, eq215_e2697_d_n22, eq215_e2697_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq215_reactive_node_derivatives: [f64; 23] = [eq215_e2699_d_n0, eq215_e2699_d_n1, eq215_e2699_d_n2, eq215_e2699_d_n3, eq215_e2699_d_n4, eq215_e2699_d_n5, eq215_e2699_d_n6, eq215_e2699_d_n7, eq215_e2699_d_n8, eq215_e2699_d_n9, 0.0, 0.0, eq215_e2699_d_n12, 0.0, eq215_e2699_d_n14, eq215_e2699_d_n15, eq215_e2699_d_n16, eq215_e2699_d_n17, eq215_e2699_d_n18, eq215_e2699_d_n19, eq215_e2699_d_n20, eq215_e2699_d_n21, eq215_e2699_d_n22];
        let eq215_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            nodes,
            &eq215_reactive_node_derivatives,
            branches,
            &eq215_reactive_branch_derivatives,
            multiplicity,
        );
        let eq216_e2702_q: f64 = var_qfr;
        let eq216_e2703: f64 = (p.p7 * var_qfr);
        let eq216_e2703_d_n0: f64 = (p.p7 * var_qfr_dn0);
        let eq216_e2703_d_n2: f64 = (p.p7 * var_qfr_dn2);
        let eq216_e2703_d_n4: f64 = (p.p7 * var_qfr_dn4);
        let eq216_e2703_q: f64 = (p.p7 * eq216_e2702_q);
        stamper.stamp_current_reactive_node3(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (eq216_e2703_d_n0),
            nodes[2],
            multiplicity * (eq216_e2703_d_n2),
            nodes[4],
            multiplicity * (eq216_e2703_d_n4),
        );
        let eq217_e2707: f64 = (p.p4 * p.p5);
        let eq217_e2709: f64 = (eq217_e2707 * p.p220);
        let eq217_e2711: f64 = (eq217_e2709 * (nv1 - nv2));
        let eq217_e2712_q: f64 = eq217_e2711;
        let eq217_e2713: f64 = (p.p7 * eq217_e2711);
        let eq217_e2713_d_n1: f64 = (p.p7 * eq217_e2709);
        let eq217_e2713_d_n2: f64 = (p.p7 * (-eq217_e2709));
        let eq217_e2713_q: f64 = (p.p7 * eq217_e2712_q);
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (eq217_e2713_d_n1),
            nodes[2],
            multiplicity * (eq217_e2713_d_n2),
        );
        let eq218_e2716_q: f64 = var_qfr2;
        let eq218_e2717: f64 = (p.p7 * var_qfr2);
        let eq218_e2717_d_n0: f64 = (p.p7 * var_qfr2_dn0);
        let eq218_e2717_d_n1: f64 = (p.p7 * var_qfr2_dn1);
        let eq218_e2717_d_n2: f64 = (p.p7 * var_qfr2_dn2);
        let eq218_e2717_d_n3: f64 = (p.p7 * var_qfr2_dn3);
        let eq218_e2717_d_n4: f64 = (p.p7 * var_qfr2_dn4);
        let eq218_e2717_d_n5: f64 = (p.p7 * var_qfr2_dn5);
        let eq218_e2717_d_n6: f64 = (p.p7 * var_qfr2_dn6);
        let eq218_e2717_d_n7: f64 = (p.p7 * var_qfr2_dn7);
        let eq218_e2717_d_n8: f64 = (p.p7 * var_qfr2_dn8);
        let eq218_e2717_d_n9: f64 = (p.p7 * var_qfr2_dn9);
        let eq218_e2717_d_n12: f64 = (p.p7 * var_qfr2_dn12);
        let eq218_e2717_d_n14: f64 = (p.p7 * var_qfr2_dn14);
        let eq218_e2717_d_n15: f64 = (p.p7 * var_qfr2_dn15);
        let eq218_e2717_d_n16: f64 = (p.p7 * var_qfr2_dn16);
        let eq218_e2717_d_n17: f64 = (p.p7 * var_qfr2_dn17);
        let eq218_e2717_d_n18: f64 = (p.p7 * var_qfr2_dn18);
        let eq218_e2717_d_n19: f64 = (p.p7 * var_qfr2_dn19);
        let eq218_e2717_d_n20: f64 = (p.p7 * var_qfr2_dn20);
        let eq218_e2717_d_n21: f64 = (p.p7 * var_qfr2_dn21);
        let eq218_e2717_d_n22: f64 = (p.p7 * var_qfr2_dn22);
        let eq218_e2717_q: f64 = (p.p7 * eq218_e2716_q);
        let eq218_reactive_node_derivatives: [f64; 23] = [eq218_e2717_d_n0, eq218_e2717_d_n1, eq218_e2717_d_n2, eq218_e2717_d_n3, eq218_e2717_d_n4, eq218_e2717_d_n5, eq218_e2717_d_n6, eq218_e2717_d_n7, eq218_e2717_d_n8, eq218_e2717_d_n9, 0.0, 0.0, eq218_e2717_d_n12, 0.0, eq218_e2717_d_n14, eq218_e2717_d_n15, eq218_e2717_d_n16, eq218_e2717_d_n17, eq218_e2717_d_n18, eq218_e2717_d_n19, eq218_e2717_d_n20, eq218_e2717_d_n21, eq218_e2717_d_n22];
        let eq218_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[2]),
            nodes,
            &eq218_reactive_node_derivatives,
            branches,
            &eq218_reactive_branch_derivatives,
            multiplicity,
        );
        let eq219_e2720_q: f64 = var_qfr3;
        let eq219_e2721: f64 = (p.p7 * var_qfr3);
        let eq219_e2721_d_n0: f64 = (p.p7 * var_qfr3_dn0);
        let eq219_e2721_d_n2: f64 = (p.p7 * var_qfr3_dn2);
        let eq219_e2721_q: f64 = (p.p7 * eq219_e2720_q);
        stamper.stamp_current_reactive_node2(
            Some(nodes[2]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (eq219_e2721_d_n0),
            nodes[2],
            multiplicity * (eq219_e2721_d_n2),
        );
        let eq220_e2724_q: f64 = var_qdep;
        let eq220_e2725: f64 = (p.p7 * var_qdep);
        let eq220_e2725_d_n0: f64 = (p.p7 * var_qdep_dn0);
        let eq220_e2725_d_n1: f64 = (p.p7 * var_qdep_dn1);
        let eq220_e2725_d_n2: f64 = (p.p7 * var_qdep_dn2);
        let eq220_e2725_d_n3: f64 = (p.p7 * var_qdep_dn3);
        let eq220_e2725_d_n4: f64 = (p.p7 * var_qdep_dn4);
        let eq220_e2725_d_n5: f64 = (p.p7 * var_qdep_dn5);
        let eq220_e2725_d_n6: f64 = (p.p7 * var_qdep_dn6);
        let eq220_e2725_d_n7: f64 = (p.p7 * var_qdep_dn7);
        let eq220_e2725_d_n8: f64 = (p.p7 * var_qdep_dn8);
        let eq220_e2725_d_n9: f64 = (p.p7 * var_qdep_dn9);
        let eq220_e2725_d_n12: f64 = (p.p7 * var_qdep_dn12);
        let eq220_e2725_d_n14: f64 = (p.p7 * var_qdep_dn14);
        let eq220_e2725_d_n15: f64 = (p.p7 * var_qdep_dn15);
        let eq220_e2725_d_n16: f64 = (p.p7 * var_qdep_dn16);
        let eq220_e2725_d_n17: f64 = (p.p7 * var_qdep_dn17);
        let eq220_e2725_d_n18: f64 = (p.p7 * var_qdep_dn18);
        let eq220_e2725_d_n19: f64 = (p.p7 * var_qdep_dn19);
        let eq220_e2725_d_n20: f64 = (p.p7 * var_qdep_dn20);
        let eq220_e2725_d_n21: f64 = (p.p7 * var_qdep_dn21);
        let eq220_e2725_d_n22: f64 = (p.p7 * var_qdep_dn22);
        let eq220_e2725_q: f64 = (p.p7 * eq220_e2724_q);
        let eq220_reactive_node_derivatives: [f64; 23] = [eq220_e2725_d_n0, eq220_e2725_d_n1, eq220_e2725_d_n2, eq220_e2725_d_n3, eq220_e2725_d_n4, eq220_e2725_d_n5, eq220_e2725_d_n6, eq220_e2725_d_n7, eq220_e2725_d_n8, eq220_e2725_d_n9, 0.0, 0.0, eq220_e2725_d_n12, 0.0, eq220_e2725_d_n14, eq220_e2725_d_n15, eq220_e2725_d_n16, eq220_e2725_d_n17, eq220_e2725_d_n18, eq220_e2725_d_n19, eq220_e2725_d_n20, eq220_e2725_d_n21, eq220_e2725_d_n22];
        let eq220_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[0]),
            nodes,
            &eq220_reactive_node_derivatives,
            branches,
            &eq220_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq223_e2771, eq223_e2771_d_n4, eq223_e2771_q,) = {
    if (var_guard576 != 0.0) {
        let eq223_e2768: f64 = ((nv4 - 0.0) * p.p33);
        let eq223_e2769_q: f64 = eq223_e2768;
        (eq223_e2768, p.p33, eq223_e2769_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (eq223_e2771_d_n4),
        );
    }
}
