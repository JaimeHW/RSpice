#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_5(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
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
        let eq13_e238: f64 = (p.p148 * s.v[180]);let eq13_e239: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq13_e238);let eq13_e239_d_n0: f64 = ((p.p148 * s.dn[180][0]) * ddt_scale);let eq13_e239_d_n1: f64 = ((p.p148 * s.dn[180][1]) * ddt_scale);let eq13_e239_d_n2: f64 = ((p.p148 * s.dn[180][2]) * ddt_scale);let eq13_e239_d_n3: f64 = ((p.p148 * s.dn[180][3]) * ddt_scale);let eq13_e239_d_n4: f64 = ((p.p148 * s.dn[180][4]) * ddt_scale);let eq13_e239_d_n5: f64 = ((p.p148 * s.dn[180][5]) * ddt_scale);let eq13_e239_d_n6: f64 = ((p.p148 * s.dn[180][6]) * ddt_scale);let eq13_e239_d_n7: f64 = ((p.p148 * s.dn[180][7]) * ddt_scale);let eq13_e239_d_n8: f64 = ((p.p148 * s.dn[180][8]) * ddt_scale);let eq13_e239_d_n9: f64 = ((p.p148 * s.dn[180][9]) * ddt_scale);let eq13_e239_d_n10: f64 = ((p.p148 * s.dn[180][10]) * ddt_scale);let eq13_e239_d_n11: f64 = ((p.p148 * s.dn[180][11]) * ddt_scale);let eq13_e239_d_n12: f64 = ((p.p148 * s.dn[180][12]) * ddt_scale);let eq13_e239_d_n13: f64 = ((p.p148 * s.dn[180][13]) * ddt_scale);let eq13_e239_d_n14: f64 = ((p.p148 * s.dn[180][14]) * ddt_scale);let eq13_e239_d_b0: f64 = ((p.p148 * s.db[180][0]) * ddt_scale);let eq13_e239_d_b1: f64 = ((p.p148 * s.db[180][1]) * ddt_scale);let eq13_e239_d_b2: f64 = ((p.p148 * s.db[180][2]) * ddt_scale);let eq13_e239_d_b3: f64 = ((p.p148 * s.db[180][3]) * ddt_scale);let eq13_e239_d_b4: f64 = ((p.p148 * s.db[180][4]) * ddt_scale);let eq13_e239_d_b5: f64 = ((p.p148 * s.db[180][5]) * ddt_scale);let eq13_value: f64 = eq13_e239;let eq13_node_derivatives: [f64; 15] = [eq13_e239_d_n0, eq13_e239_d_n1, eq13_e239_d_n2, eq13_e239_d_n3, eq13_e239_d_n4, eq13_e239_d_n5, eq13_e239_d_n6, eq13_e239_d_n7, eq13_e239_d_n8, eq13_e239_d_n9, eq13_e239_d_n10, eq13_e239_d_n11, eq13_e239_d_n12, eq13_e239_d_n13, eq13_e239_d_n14];let eq13_branch_derivatives: [f64; 6] = [eq13_e239_d_b0, eq13_e239_d_b1, eq13_e239_d_b2, eq13_e239_d_b3, eq13_e239_d_b4, eq13_e239_d_b5];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );let eq14_e242: f64 = (p.p148 * s.v[194]);let eq14_value: f64 = eq14_e242;
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq14_value),
            &s.dn[194],
            &s.db[194],
            (multiplicity) * (p.p148),
        );let eq15_e246: f64 = (s.v[42] + s.v[199]);let eq15_e246_d_n0: f64 = (s.dn[42][0] + s.dn[199][0]);let eq15_e246_d_n1: f64 = (s.dn[42][1] + s.dn[199][1]);let eq15_e246_d_n2: f64 = (s.dn[42][2] + s.dn[199][2]);let eq15_e246_d_n3: f64 = (s.dn[42][3] + s.dn[199][3]);let eq15_e246_d_n4: f64 = (s.dn[42][4] + s.dn[199][4]);let eq15_e246_d_n5: f64 = (s.dn[42][5] + s.dn[199][5]);let eq15_e246_d_n6: f64 = (s.dn[42][6] + s.dn[199][6]);let eq15_e246_d_n7: f64 = (s.dn[42][7] + s.dn[199][7]);let eq15_e246_d_n8: f64 = (s.dn[42][8] + s.dn[199][8]);let eq15_e246_d_n9: f64 = (s.dn[42][9] + s.dn[199][9]);let eq15_e246_d_n10: f64 = (s.dn[42][10] + s.dn[199][10]);let eq15_e246_d_n11: f64 = (s.dn[42][11] + s.dn[199][11]);let eq15_e246_d_n12: f64 = (s.dn[42][12] + s.dn[199][12]);let eq15_e246_d_n13: f64 = (s.dn[42][13] + s.dn[199][13]);let eq15_e246_d_n14: f64 = (s.dn[42][14] + s.dn[199][14]);let eq15_e246_d_b0: f64 = (s.db[42][0] + s.db[199][0]);let eq15_e246_d_b1: f64 = (s.db[42][1] + s.db[199][1]);let eq15_e246_d_b2: f64 = (s.db[42][2] + s.db[199][2]);let eq15_e246_d_b3: f64 = (s.db[42][3] + s.db[199][3]);let eq15_e246_d_b4: f64 = (s.db[42][4] + s.db[199][4]);let eq15_e246_d_b5: f64 = (s.db[42][5] + s.db[199][5]);let eq15_e247: f64 = (p.p148 * eq15_e246);let eq15_e247_d_n0: f64 = (p.p148 * eq15_e246_d_n0);let eq15_e247_d_n1: f64 = (p.p148 * eq15_e246_d_n1);let eq15_e247_d_n2: f64 = (p.p148 * eq15_e246_d_n2);let eq15_e247_d_n3: f64 = (p.p148 * eq15_e246_d_n3);let eq15_e247_d_n4: f64 = (p.p148 * eq15_e246_d_n4);let eq15_e247_d_n5: f64 = (p.p148 * eq15_e246_d_n5);let eq15_e247_d_n6: f64 = (p.p148 * eq15_e246_d_n6);let eq15_e247_d_n7: f64 = (p.p148 * eq15_e246_d_n7);let eq15_e247_d_n8: f64 = (p.p148 * eq15_e246_d_n8);let eq15_e247_d_n9: f64 = (p.p148 * eq15_e246_d_n9);let eq15_e247_d_n10: f64 = (p.p148 * eq15_e246_d_n10);let eq15_e247_d_n11: f64 = (p.p148 * eq15_e246_d_n11);let eq15_e247_d_n12: f64 = (p.p148 * eq15_e246_d_n12);let eq15_e247_d_n13: f64 = (p.p148 * eq15_e246_d_n13);let eq15_e247_d_n14: f64 = (p.p148 * eq15_e246_d_n14);let eq15_e247_d_b0: f64 = (p.p148 * eq15_e246_d_b0);let eq15_e247_d_b1: f64 = (p.p148 * eq15_e246_d_b1);let eq15_e247_d_b2: f64 = (p.p148 * eq15_e246_d_b2);let eq15_e247_d_b3: f64 = (p.p148 * eq15_e246_d_b3);let eq15_e247_d_b4: f64 = (p.p148 * eq15_e246_d_b4);let eq15_e247_d_b5: f64 = (p.p148 * eq15_e246_d_b5);let eq15_e248: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq15_e247);let eq15_value: f64 = eq15_e248;let eq15_node_derivatives: [f64; 15] = [(eq15_e247_d_n0 * ddt_scale), (eq15_e247_d_n1 * ddt_scale), (eq15_e247_d_n2 * ddt_scale), (eq15_e247_d_n3 * ddt_scale), (eq15_e247_d_n4 * ddt_scale), (eq15_e247_d_n5 * ddt_scale), (eq15_e247_d_n6 * ddt_scale), (eq15_e247_d_n7 * ddt_scale), (eq15_e247_d_n8 * ddt_scale), (eq15_e247_d_n9 * ddt_scale), (eq15_e247_d_n10 * ddt_scale), (eq15_e247_d_n11 * ddt_scale), (eq15_e247_d_n12 * ddt_scale), (eq15_e247_d_n13 * ddt_scale), (eq15_e247_d_n14 * ddt_scale)];let eq15_branch_derivatives: [f64; 6] = [(eq15_e247_d_b0 * ddt_scale), (eq15_e247_d_b1 * ddt_scale), (eq15_e247_d_b2 * ddt_scale), (eq15_e247_d_b3 * ddt_scale), (eq15_e247_d_b4 * ddt_scale), (eq15_e247_d_b5 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_6(
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
        let nv1 = ctx.node_voltage(nodes[1]);let nv5 = ctx.node_voltage(nodes[5]);let nv7 = ctx.node_voltage(nodes[7]);let eq16_e251: f64 = (s.v[172] * (nv7 - nv5));let eq16_e251_d_n0: f64 = (s.dn[172][0] * (nv7 - nv5));let eq16_e251_d_n1: f64 = (s.dn[172][1] * (nv7 - nv5));let eq16_e251_d_n2: f64 = (s.dn[172][2] * (nv7 - nv5));let eq16_e251_d_n3: f64 = (s.dn[172][3] * (nv7 - nv5));let eq16_e251_d_n4: f64 = (s.dn[172][4] * (nv7 - nv5));let eq16_e251_d_n5: f64 = ((s.dn[172][5] * (nv7 - nv5)) + (-s.v[172]));let eq16_e251_d_n6: f64 = (s.dn[172][6] * (nv7 - nv5));let eq16_e251_d_n7: f64 = ((s.dn[172][7] * (nv7 - nv5)) + s.v[172]);let eq16_e251_d_n8: f64 = (s.dn[172][8] * (nv7 - nv5));let eq16_e251_d_n9: f64 = (s.dn[172][9] * (nv7 - nv5));let eq16_e251_d_n10: f64 = (s.dn[172][10] * (nv7 - nv5));let eq16_e251_d_n11: f64 = (s.dn[172][11] * (nv7 - nv5));let eq16_e251_d_n12: f64 = (s.dn[172][12] * (nv7 - nv5));let eq16_e251_d_n13: f64 = (s.dn[172][13] * (nv7 - nv5));let eq16_e251_d_n14: f64 = (s.dn[172][14] * (nv7 - nv5));let eq16_e251_d_b0: f64 = (s.db[172][0] * (nv7 - nv5));let eq16_e251_d_b1: f64 = (s.db[172][1] * (nv7 - nv5));let eq16_e251_d_b2: f64 = (s.db[172][2] * (nv7 - nv5));let eq16_e251_d_b3: f64 = (s.db[172][3] * (nv7 - nv5));let eq16_e251_d_b4: f64 = (s.db[172][4] * (nv7 - nv5));let eq16_e251_d_b5: f64 = (s.db[172][5] * (nv7 - nv5));let eq16_e252: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq16_e251);let eq16_value: f64 = eq16_e252;let eq16_node_derivatives: [f64; 15] = [(eq16_e251_d_n0 * ddt_scale), (eq16_e251_d_n1 * ddt_scale), (eq16_e251_d_n2 * ddt_scale), (eq16_e251_d_n3 * ddt_scale), (eq16_e251_d_n4 * ddt_scale), (eq16_e251_d_n5 * ddt_scale), (eq16_e251_d_n6 * ddt_scale), (eq16_e251_d_n7 * ddt_scale), (eq16_e251_d_n8 * ddt_scale), (eq16_e251_d_n9 * ddt_scale), (eq16_e251_d_n10 * ddt_scale), (eq16_e251_d_n11 * ddt_scale), (eq16_e251_d_n12 * ddt_scale), (eq16_e251_d_n13 * ddt_scale), (eq16_e251_d_n14 * ddt_scale)];let eq16_branch_derivatives: [f64; 6] = [(eq16_e251_d_b0 * ddt_scale), (eq16_e251_d_b1 * ddt_scale), (eq16_e251_d_b2 * ddt_scale), (eq16_e251_d_b3 * ddt_scale), (eq16_e251_d_b4 * ddt_scale), (eq16_e251_d_b5 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq16_value),
            &eq16_node_derivatives,
            &eq16_branch_derivatives,
            multiplicity,
        );let eq17_e255: f64 = (p.p148 * s.v[41]);let eq17_e256: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq17_e255);let eq17_e256_d_n0: f64 = ((p.p148 * s.dn[41][0]) * ddt_scale);let eq17_e256_d_n1: f64 = ((p.p148 * s.dn[41][1]) * ddt_scale);let eq17_e256_d_n2: f64 = ((p.p148 * s.dn[41][2]) * ddt_scale);let eq17_e256_d_n3: f64 = ((p.p148 * s.dn[41][3]) * ddt_scale);let eq17_e256_d_n4: f64 = ((p.p148 * s.dn[41][4]) * ddt_scale);let eq17_e256_d_n5: f64 = ((p.p148 * s.dn[41][5]) * ddt_scale);let eq17_e256_d_n6: f64 = ((p.p148 * s.dn[41][6]) * ddt_scale);let eq17_e256_d_n7: f64 = ((p.p148 * s.dn[41][7]) * ddt_scale);let eq17_e256_d_n8: f64 = ((p.p148 * s.dn[41][8]) * ddt_scale);let eq17_e256_d_n9: f64 = ((p.p148 * s.dn[41][9]) * ddt_scale);let eq17_e256_d_n10: f64 = ((p.p148 * s.dn[41][10]) * ddt_scale);let eq17_e256_d_n11: f64 = ((p.p148 * s.dn[41][11]) * ddt_scale);let eq17_e256_d_n12: f64 = ((p.p148 * s.dn[41][12]) * ddt_scale);let eq17_e256_d_n13: f64 = ((p.p148 * s.dn[41][13]) * ddt_scale);let eq17_e256_d_n14: f64 = ((p.p148 * s.dn[41][14]) * ddt_scale);let eq17_e256_d_b0: f64 = ((p.p148 * s.db[41][0]) * ddt_scale);let eq17_e256_d_b1: f64 = ((p.p148 * s.db[41][1]) * ddt_scale);let eq17_e256_d_b2: f64 = ((p.p148 * s.db[41][2]) * ddt_scale);let eq17_e256_d_b3: f64 = ((p.p148 * s.db[41][3]) * ddt_scale);let eq17_e256_d_b4: f64 = ((p.p148 * s.db[41][4]) * ddt_scale);let eq17_e256_d_b5: f64 = ((p.p148 * s.db[41][5]) * ddt_scale);let eq17_value: f64 = eq17_e256;let eq17_node_derivatives: [f64; 15] = [eq17_e256_d_n0, eq17_e256_d_n1, eq17_e256_d_n2, eq17_e256_d_n3, eq17_e256_d_n4, eq17_e256_d_n5, eq17_e256_d_n6, eq17_e256_d_n7, eq17_e256_d_n8, eq17_e256_d_n9, eq17_e256_d_n10, eq17_e256_d_n11, eq17_e256_d_n12, eq17_e256_d_n13, eq17_e256_d_n14];let eq17_branch_derivatives: [f64; 6] = [eq17_e256_d_b0, eq17_e256_d_b1, eq17_e256_d_b2, eq17_e256_d_b3, eq17_e256_d_b4, eq17_e256_d_b5];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(5),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );let eq18_e259: f64 = (s.v[171] * (nv1 - nv5));let eq18_e259_d_n0: f64 = (s.dn[171][0] * (nv1 - nv5));let eq18_e259_d_n1: f64 = ((s.dn[171][1] * (nv1 - nv5)) + s.v[171]);let eq18_e259_d_n2: f64 = (s.dn[171][2] * (nv1 - nv5));let eq18_e259_d_n3: f64 = (s.dn[171][3] * (nv1 - nv5));let eq18_e259_d_n4: f64 = (s.dn[171][4] * (nv1 - nv5));let eq18_e259_d_n5: f64 = ((s.dn[171][5] * (nv1 - nv5)) + (-s.v[171]));let eq18_e259_d_n6: f64 = (s.dn[171][6] * (nv1 - nv5));let eq18_e259_d_n7: f64 = (s.dn[171][7] * (nv1 - nv5));let eq18_e259_d_n8: f64 = (s.dn[171][8] * (nv1 - nv5));let eq18_e259_d_n9: f64 = (s.dn[171][9] * (nv1 - nv5));let eq18_e259_d_n10: f64 = (s.dn[171][10] * (nv1 - nv5));let eq18_e259_d_n11: f64 = (s.dn[171][11] * (nv1 - nv5));let eq18_e259_d_n12: f64 = (s.dn[171][12] * (nv1 - nv5));let eq18_e259_d_n13: f64 = (s.dn[171][13] * (nv1 - nv5));let eq18_e259_d_n14: f64 = (s.dn[171][14] * (nv1 - nv5));let eq18_e259_d_b0: f64 = (s.db[171][0] * (nv1 - nv5));let eq18_e259_d_b1: f64 = (s.db[171][1] * (nv1 - nv5));let eq18_e259_d_b2: f64 = (s.db[171][2] * (nv1 - nv5));let eq18_e259_d_b3: f64 = (s.db[171][3] * (nv1 - nv5));let eq18_e259_d_b4: f64 = (s.db[171][4] * (nv1 - nv5));let eq18_e259_d_b5: f64 = (s.db[171][5] * (nv1 - nv5));let eq18_e260: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq18_e259);let eq18_value: f64 = eq18_e260;let eq18_node_derivatives: [f64; 15] = [(eq18_e259_d_n0 * ddt_scale), (eq18_e259_d_n1 * ddt_scale), (eq18_e259_d_n2 * ddt_scale), (eq18_e259_d_n3 * ddt_scale), (eq18_e259_d_n4 * ddt_scale), (eq18_e259_d_n5 * ddt_scale), (eq18_e259_d_n6 * ddt_scale), (eq18_e259_d_n7 * ddt_scale), (eq18_e259_d_n8 * ddt_scale), (eq18_e259_d_n9 * ddt_scale), (eq18_e259_d_n10 * ddt_scale), (eq18_e259_d_n11 * ddt_scale), (eq18_e259_d_n12 * ddt_scale), (eq18_e259_d_n13 * ddt_scale), (eq18_e259_d_n14 * ddt_scale)];let eq18_branch_derivatives: [f64; 6] = [(eq18_e259_d_b0 * ddt_scale), (eq18_e259_d_b1 * ddt_scale), (eq18_e259_d_b2 * ddt_scale), (eq18_e259_d_b3 * ddt_scale), (eq18_e259_d_b4 * ddt_scale), (eq18_e259_d_b5 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(5),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_7(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv1 = ctx.node_voltage(nodes[1]);let nv2 = ctx.node_voltage(nodes[2]);let nv5 = ctx.node_voltage(nodes[5]);let nv6 = ctx.node_voltage(nodes[6]);let nv7 = ctx.node_voltage(nodes[7]);
        let (eq19_e266, eq19_e266_d_n0, eq19_e266_d_n1, eq19_e266_d_n2, eq19_e266_d_n3, eq19_e266_d_n4, eq19_e266_d_n5, eq19_e266_d_n6, eq19_e266_d_n7, eq19_e266_d_n8, eq19_e266_d_n9, eq19_e266_d_n10, eq19_e266_d_n11, eq19_e266_d_n12, eq19_e266_d_n13, eq19_e266_d_n14, eq19_e266_d_b0, eq19_e266_d_b1, eq19_e266_d_b2, eq19_e266_d_b3, eq19_e266_d_b4, eq19_e266_d_b5,) = {
    if s.b[511] {
        let eq19_e264: f64 = ((nv1 - nv7) / s.v[71]);let eq19_e264_d_n0: f64 = (-(((nv1 - nv7) * s.dn[71][0]) / (s.v[71] * s.v[71])));let eq19_e264_d_n1: f64 = ((s.v[71] - ((nv1 - nv7) * s.dn[71][1])) / (s.v[71] * s.v[71]));let eq19_e264_d_n2: f64 = (-(((nv1 - nv7) * s.dn[71][2]) / (s.v[71] * s.v[71])));let eq19_e264_d_n3: f64 = (-(((nv1 - nv7) * s.dn[71][3]) / (s.v[71] * s.v[71])));let eq19_e264_d_n4: f64 = (-(((nv1 - nv7) * s.dn[71][4]) / (s.v[71] * s.v[71])));let eq19_e264_d_n5: f64 = (-(((nv1 - nv7) * s.dn[71][5]) / (s.v[71] * s.v[71])));let eq19_e264_d_n6: f64 = (-(((nv1 - nv7) * s.dn[71][6]) / (s.v[71] * s.v[71])));let eq19_e264_d_n7: f64 = (((-s.v[71]) - ((nv1 - nv7) * s.dn[71][7])) / (s.v[71] * s.v[71]));let eq19_e264_d_n8: f64 = (-(((nv1 - nv7) * s.dn[71][8]) / (s.v[71] * s.v[71])));let eq19_e264_d_n9: f64 = (-(((nv1 - nv7) * s.dn[71][9]) / (s.v[71] * s.v[71])));let eq19_e264_d_n10: f64 = (-(((nv1 - nv7) * s.dn[71][10]) / (s.v[71] * s.v[71])));let eq19_e264_d_n11: f64 = (-(((nv1 - nv7) * s.dn[71][11]) / (s.v[71] * s.v[71])));let eq19_e264_d_n12: f64 = (-(((nv1 - nv7) * s.dn[71][12]) / (s.v[71] * s.v[71])));let eq19_e264_d_n13: f64 = (-(((nv1 - nv7) * s.dn[71][13]) / (s.v[71] * s.v[71])));let eq19_e264_d_n14: f64 = (-(((nv1 - nv7) * s.dn[71][14]) / (s.v[71] * s.v[71])));let eq19_e264_d_b0: f64 = (-(((nv1 - nv7) * s.db[71][0]) / (s.v[71] * s.v[71])));let eq19_e264_d_b1: f64 = (-(((nv1 - nv7) * s.db[71][1]) / (s.v[71] * s.v[71])));let eq19_e264_d_b2: f64 = (-(((nv1 - nv7) * s.db[71][2]) / (s.v[71] * s.v[71])));let eq19_e264_d_b3: f64 = (-(((nv1 - nv7) * s.db[71][3]) / (s.v[71] * s.v[71])));let eq19_e264_d_b4: f64 = (-(((nv1 - nv7) * s.db[71][4]) / (s.v[71] * s.v[71])));let eq19_e264_d_b5: f64 = (-(((nv1 - nv7) * s.db[71][5]) / (s.v[71] * s.v[71])));
        (eq19_e264, eq19_e264_d_n0, eq19_e264_d_n1, eq19_e264_d_n2, eq19_e264_d_n3, eq19_e264_d_n4, eq19_e264_d_n5, eq19_e264_d_n6, eq19_e264_d_n7, eq19_e264_d_n8, eq19_e264_d_n9, eq19_e264_d_n10, eq19_e264_d_n11, eq19_e264_d_n12, eq19_e264_d_n13, eq19_e264_d_n14, eq19_e264_d_b0, eq19_e264_d_b1, eq19_e264_d_b2, eq19_e264_d_b3, eq19_e264_d_b4, eq19_e264_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_value: f64 = eq19_e266;let eq19_node_derivatives: [f64; 15] = [eq19_e266_d_n0, eq19_e266_d_n1, eq19_e266_d_n2, eq19_e266_d_n3, eq19_e266_d_n4, eq19_e266_d_n5, eq19_e266_d_n6, eq19_e266_d_n7, eq19_e266_d_n8, eq19_e266_d_n9, eq19_e266_d_n10, eq19_e266_d_n11, eq19_e266_d_n12, eq19_e266_d_n13, eq19_e266_d_n14];let eq19_branch_derivatives: [f64; 6] = [eq19_e266_d_b0, eq19_e266_d_b1, eq19_e266_d_b2, eq19_e266_d_b3, eq19_e266_d_b4, eq19_e266_d_b5];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(7),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let (eq20_e271,) = {
    if (!s.b[511]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e271;
        stamper.stamp_potential_const_local(
            1,
            eq20_value,
        );
        let (eq21_e277, eq21_e277_d_n0, eq21_e277_d_n1, eq21_e277_d_n2, eq21_e277_d_n3, eq21_e277_d_n4, eq21_e277_d_n5, eq21_e277_d_n6, eq21_e277_d_n7, eq21_e277_d_n8, eq21_e277_d_n9, eq21_e277_d_n10, eq21_e277_d_n11, eq21_e277_d_n12, eq21_e277_d_n13, eq21_e277_d_n14, eq21_e277_d_b0, eq21_e277_d_b1, eq21_e277_d_b2, eq21_e277_d_b3, eq21_e277_d_b4, eq21_e277_d_b5,) = {
    if s.b[512] {
        let eq21_e275: f64 = ((nv6 - nv2) / s.v[73]);let eq21_e275_d_n0: f64 = (-(((nv6 - nv2) * s.dn[73][0]) / (s.v[73] * s.v[73])));let eq21_e275_d_n1: f64 = (-(((nv6 - nv2) * s.dn[73][1]) / (s.v[73] * s.v[73])));let eq21_e275_d_n2: f64 = (((-s.v[73]) - ((nv6 - nv2) * s.dn[73][2])) / (s.v[73] * s.v[73]));let eq21_e275_d_n3: f64 = (-(((nv6 - nv2) * s.dn[73][3]) / (s.v[73] * s.v[73])));let eq21_e275_d_n4: f64 = (-(((nv6 - nv2) * s.dn[73][4]) / (s.v[73] * s.v[73])));let eq21_e275_d_n5: f64 = (-(((nv6 - nv2) * s.dn[73][5]) / (s.v[73] * s.v[73])));let eq21_e275_d_n6: f64 = ((s.v[73] - ((nv6 - nv2) * s.dn[73][6])) / (s.v[73] * s.v[73]));let eq21_e275_d_n7: f64 = (-(((nv6 - nv2) * s.dn[73][7]) / (s.v[73] * s.v[73])));let eq21_e275_d_n8: f64 = (-(((nv6 - nv2) * s.dn[73][8]) / (s.v[73] * s.v[73])));let eq21_e275_d_n9: f64 = (-(((nv6 - nv2) * s.dn[73][9]) / (s.v[73] * s.v[73])));let eq21_e275_d_n10: f64 = (-(((nv6 - nv2) * s.dn[73][10]) / (s.v[73] * s.v[73])));let eq21_e275_d_n11: f64 = (-(((nv6 - nv2) * s.dn[73][11]) / (s.v[73] * s.v[73])));let eq21_e275_d_n12: f64 = (-(((nv6 - nv2) * s.dn[73][12]) / (s.v[73] * s.v[73])));let eq21_e275_d_n13: f64 = (-(((nv6 - nv2) * s.dn[73][13]) / (s.v[73] * s.v[73])));let eq21_e275_d_n14: f64 = (-(((nv6 - nv2) * s.dn[73][14]) / (s.v[73] * s.v[73])));let eq21_e275_d_b0: f64 = (-(((nv6 - nv2) * s.db[73][0]) / (s.v[73] * s.v[73])));let eq21_e275_d_b1: f64 = (-(((nv6 - nv2) * s.db[73][1]) / (s.v[73] * s.v[73])));let eq21_e275_d_b2: f64 = (-(((nv6 - nv2) * s.db[73][2]) / (s.v[73] * s.v[73])));let eq21_e275_d_b3: f64 = (-(((nv6 - nv2) * s.db[73][3]) / (s.v[73] * s.v[73])));let eq21_e275_d_b4: f64 = (-(((nv6 - nv2) * s.db[73][4]) / (s.v[73] * s.v[73])));let eq21_e275_d_b5: f64 = (-(((nv6 - nv2) * s.db[73][5]) / (s.v[73] * s.v[73])));
        (eq21_e275, eq21_e275_d_n0, eq21_e275_d_n1, eq21_e275_d_n2, eq21_e275_d_n3, eq21_e275_d_n4, eq21_e275_d_n5, eq21_e275_d_n6, eq21_e275_d_n7, eq21_e275_d_n8, eq21_e275_d_n9, eq21_e275_d_n10, eq21_e275_d_n11, eq21_e275_d_n12, eq21_e275_d_n13, eq21_e275_d_n14, eq21_e275_d_b0, eq21_e275_d_b1, eq21_e275_d_b2, eq21_e275_d_b3, eq21_e275_d_b4, eq21_e275_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e277;let eq21_node_derivatives: [f64; 15] = [eq21_e277_d_n0, eq21_e277_d_n1, eq21_e277_d_n2, eq21_e277_d_n3, eq21_e277_d_n4, eq21_e277_d_n5, eq21_e277_d_n6, eq21_e277_d_n7, eq21_e277_d_n8, eq21_e277_d_n9, eq21_e277_d_n10, eq21_e277_d_n11, eq21_e277_d_n12, eq21_e277_d_n13, eq21_e277_d_n14];let eq21_branch_derivatives: [f64; 6] = [eq21_e277_d_b0, eq21_e277_d_b1, eq21_e277_d_b2, eq21_e277_d_b3, eq21_e277_d_b4, eq21_e277_d_b5];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(2),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let (eq22_e282,) = {
    if (!s.b[512]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e282;
        stamper.stamp_potential_const_local(
            2,
            eq22_value,
        );
        let (eq23_e288, eq23_e288_d_n0, eq23_e288_d_n1, eq23_e288_d_n2, eq23_e288_d_n3, eq23_e288_d_n4, eq23_e288_d_n5, eq23_e288_d_n6, eq23_e288_d_n7, eq23_e288_d_n8, eq23_e288_d_n9, eq23_e288_d_n10, eq23_e288_d_n11, eq23_e288_d_n12, eq23_e288_d_n13, eq23_e288_d_n14, eq23_e288_d_b0, eq23_e288_d_b1, eq23_e288_d_b2, eq23_e288_d_b3, eq23_e288_d_b4, eq23_e288_d_b5,) = {
    if s.b[513] {
        let eq23_e286: f64 = ((nv5 - nv0) / s.v[72]);let eq23_e286_d_n0: f64 = (((-s.v[72]) - ((nv5 - nv0) * s.dn[72][0])) / (s.v[72] * s.v[72]));let eq23_e286_d_n1: f64 = (-(((nv5 - nv0) * s.dn[72][1]) / (s.v[72] * s.v[72])));let eq23_e286_d_n2: f64 = (-(((nv5 - nv0) * s.dn[72][2]) / (s.v[72] * s.v[72])));let eq23_e286_d_n3: f64 = (-(((nv5 - nv0) * s.dn[72][3]) / (s.v[72] * s.v[72])));let eq23_e286_d_n4: f64 = (-(((nv5 - nv0) * s.dn[72][4]) / (s.v[72] * s.v[72])));let eq23_e286_d_n5: f64 = ((s.v[72] - ((nv5 - nv0) * s.dn[72][5])) / (s.v[72] * s.v[72]));let eq23_e286_d_n6: f64 = (-(((nv5 - nv0) * s.dn[72][6]) / (s.v[72] * s.v[72])));let eq23_e286_d_n7: f64 = (-(((nv5 - nv0) * s.dn[72][7]) / (s.v[72] * s.v[72])));let eq23_e286_d_n8: f64 = (-(((nv5 - nv0) * s.dn[72][8]) / (s.v[72] * s.v[72])));let eq23_e286_d_n9: f64 = (-(((nv5 - nv0) * s.dn[72][9]) / (s.v[72] * s.v[72])));let eq23_e286_d_n10: f64 = (-(((nv5 - nv0) * s.dn[72][10]) / (s.v[72] * s.v[72])));let eq23_e286_d_n11: f64 = (-(((nv5 - nv0) * s.dn[72][11]) / (s.v[72] * s.v[72])));let eq23_e286_d_n12: f64 = (-(((nv5 - nv0) * s.dn[72][12]) / (s.v[72] * s.v[72])));let eq23_e286_d_n13: f64 = (-(((nv5 - nv0) * s.dn[72][13]) / (s.v[72] * s.v[72])));let eq23_e286_d_n14: f64 = (-(((nv5 - nv0) * s.dn[72][14]) / (s.v[72] * s.v[72])));let eq23_e286_d_b0: f64 = (-(((nv5 - nv0) * s.db[72][0]) / (s.v[72] * s.v[72])));let eq23_e286_d_b1: f64 = (-(((nv5 - nv0) * s.db[72][1]) / (s.v[72] * s.v[72])));let eq23_e286_d_b2: f64 = (-(((nv5 - nv0) * s.db[72][2]) / (s.v[72] * s.v[72])));let eq23_e286_d_b3: f64 = (-(((nv5 - nv0) * s.db[72][3]) / (s.v[72] * s.v[72])));let eq23_e286_d_b4: f64 = (-(((nv5 - nv0) * s.db[72][4]) / (s.v[72] * s.v[72])));let eq23_e286_d_b5: f64 = (-(((nv5 - nv0) * s.db[72][5]) / (s.v[72] * s.v[72])));
        (eq23_e286, eq23_e286_d_n0, eq23_e286_d_n1, eq23_e286_d_n2, eq23_e286_d_n3, eq23_e286_d_n4, eq23_e286_d_n5, eq23_e286_d_n6, eq23_e286_d_n7, eq23_e286_d_n8, eq23_e286_d_n9, eq23_e286_d_n10, eq23_e286_d_n11, eq23_e286_d_n12, eq23_e286_d_n13, eq23_e286_d_n14, eq23_e286_d_b0, eq23_e286_d_b1, eq23_e286_d_b2, eq23_e286_d_b3, eq23_e286_d_b4, eq23_e286_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e288;let eq23_node_derivatives: [f64; 15] = [eq23_e288_d_n0, eq23_e288_d_n1, eq23_e288_d_n2, eq23_e288_d_n3, eq23_e288_d_n4, eq23_e288_d_n5, eq23_e288_d_n6, eq23_e288_d_n7, eq23_e288_d_n8, eq23_e288_d_n9, eq23_e288_d_n10, eq23_e288_d_n11, eq23_e288_d_n12, eq23_e288_d_n13, eq23_e288_d_n14];let eq23_branch_derivatives: [f64; 6] = [eq23_e288_d_b0, eq23_e288_d_b1, eq23_e288_d_b2, eq23_e288_d_b3, eq23_e288_d_b4, eq23_e288_d_b5];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(0),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );
        let (eq24_e293,) = {
    if (!s.b[513]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq24_value: f64 = eq24_e293;
        stamper.stamp_potential_const_local(
            3,
            eq24_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_8(
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
        let nv0 = ctx.node_voltage(nodes[0]);let nv1 = ctx.node_voltage(nodes[1]);let nv2 = ctx.node_voltage(nodes[2]);let nv5 = ctx.node_voltage(nodes[5]);let nv7 = ctx.node_voltage(nodes[7]);let nv9 = ctx.node_voltage(nodes[9]);let eq25_e296: f64 = (s.v[174] * (nv7 - nv2));let eq25_e297: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq25_e296);let eq25_value: f64 = eq25_e297;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(2),
            multiplicity * (eq25_value),
            2,
            multiplicity * (((-s.v[174]) * ddt_scale)),
            7,
            multiplicity * ((s.v[174] * ddt_scale)),
        );let eq26_e300: f64 = (s.v[173] * (nv1 - nv2));let eq26_e301: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq26_e300);let eq26_value: f64 = eq26_e301;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (eq26_value),
            1,
            multiplicity * ((s.v[173] * ddt_scale)),
            2,
            multiplicity * (((-s.v[173]) * ddt_scale)),
        );let eq27_e304: f64 = (p.p108 * (nv0 - nv2));let eq27_e305: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq27_e304);let eq27_value: f64 = eq27_e305;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(2),
            multiplicity * (eq27_value),
            0,
            multiplicity * ((p.p108 * ddt_scale)),
            2,
            multiplicity * (((-p.p108) * ddt_scale)),
        );let eq28_e308: f64 = (p.p148 * s.v[198]);let eq28_value: f64 = eq28_e308;
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq28_value),
            &s.dn[198],
            &s.db[198],
            (multiplicity) * (p.p148),
        );
        let (eq29_e316, eq29_e316_d_n0, eq29_e316_d_n1, eq29_e316_d_n2, eq29_e316_d_n3, eq29_e316_d_n4, eq29_e316_d_n5, eq29_e316_d_n6, eq29_e316_d_n7, eq29_e316_d_n8, eq29_e316_d_n9, eq29_e316_d_n10, eq29_e316_d_n11, eq29_e316_d_n12, eq29_e316_d_n13, eq29_e316_d_n14, eq29_e316_d_b0, eq29_e316_d_b1, eq29_e316_d_b2, eq29_e316_d_b3, eq29_e316_d_b4, eq29_e316_d_b5,) = {
    if (s.b[514] && s.b[515]) {
        let eq29_e314: f64 = (p.p148 * s.v[195]);
        (eq29_e314, (p.p148 * s.dn[195][0]), (p.p148 * s.dn[195][1]), (p.p148 * s.dn[195][2]), (p.p148 * s.dn[195][3]), (p.p148 * s.dn[195][4]), (p.p148 * s.dn[195][5]), (p.p148 * s.dn[195][6]), (p.p148 * s.dn[195][7]), (p.p148 * s.dn[195][8]), (p.p148 * s.dn[195][9]), (p.p148 * s.dn[195][10]), (p.p148 * s.dn[195][11]), (p.p148 * s.dn[195][12]), (p.p148 * s.dn[195][13]), (p.p148 * s.dn[195][14]), (p.p148 * s.db[195][0]), (p.p148 * s.db[195][1]), (p.p148 * s.db[195][2]), (p.p148 * s.db[195][3]), (p.p148 * s.db[195][4]), (p.p148 * s.db[195][5]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e316;let eq29_node_derivatives: [f64; 15] = [eq29_e316_d_n0, eq29_e316_d_n1, eq29_e316_d_n2, eq29_e316_d_n3, eq29_e316_d_n4, eq29_e316_d_n5, eq29_e316_d_n6, eq29_e316_d_n7, eq29_e316_d_n8, eq29_e316_d_n9, eq29_e316_d_n10, eq29_e316_d_n11, eq29_e316_d_n12, eq29_e316_d_n13, eq29_e316_d_n14];let eq29_branch_derivatives: [f64; 6] = [eq29_e316_d_b0, eq29_e316_d_b1, eq29_e316_d_b2, eq29_e316_d_b3, eq29_e316_d_b4, eq29_e316_d_b5];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq29_value),
            &eq29_node_derivatives,
            &eq29_branch_derivatives,
            multiplicity,
        );
        let (eq30_e324, eq30_e324_d_n5, eq30_e324_d_n9,) = {
    if (s.b[514] && s.b[515]) {
        let eq30_e322: f64 = (s.v[233] * (nv9 - nv5));
        (eq30_e322, (-s.v[233]), s.v[233],)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e324;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(5),
            multiplicity * (eq30_value),
            5,
            multiplicity * (eq30_e324_d_n5),
            9,
            multiplicity * (eq30_e324_d_n9),
        );
        let (eq31_e331, eq31_e331_d_n0, eq31_e331_d_n1, eq31_e331_d_n2, eq31_e331_d_n3, eq31_e331_d_n4, eq31_e331_d_n5, eq31_e331_d_n6, eq31_e331_d_n7, eq31_e331_d_n8, eq31_e331_d_n9, eq31_e331_d_n10, eq31_e331_d_n11, eq31_e331_d_n12, eq31_e331_d_n13, eq31_e331_d_n14, eq31_e331_d_b0, eq31_e331_d_b1, eq31_e331_d_b2, eq31_e331_d_b3, eq31_e331_d_b4, eq31_e331_d_b5,) = {
    if (!s.b[514]) {
        let eq31_e329: f64 = (p.p148 * s.v[195]);
        (eq31_e329, (p.p148 * s.dn[195][0]), (p.p148 * s.dn[195][1]), (p.p148 * s.dn[195][2]), (p.p148 * s.dn[195][3]), (p.p148 * s.dn[195][4]), (p.p148 * s.dn[195][5]), (p.p148 * s.dn[195][6]), (p.p148 * s.dn[195][7]), (p.p148 * s.dn[195][8]), (p.p148 * s.dn[195][9]), (p.p148 * s.dn[195][10]), (p.p148 * s.dn[195][11]), (p.p148 * s.dn[195][12]), (p.p148 * s.dn[195][13]), (p.p148 * s.dn[195][14]), (p.p148 * s.db[195][0]), (p.p148 * s.db[195][1]), (p.p148 * s.db[195][2]), (p.p148 * s.db[195][3]), (p.p148 * s.db[195][4]), (p.p148 * s.db[195][5]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e331;let eq31_node_derivatives: [f64; 15] = [eq31_e331_d_n0, eq31_e331_d_n1, eq31_e331_d_n2, eq31_e331_d_n3, eq31_e331_d_n4, eq31_e331_d_n5, eq31_e331_d_n6, eq31_e331_d_n7, eq31_e331_d_n8, eq31_e331_d_n9, eq31_e331_d_n10, eq31_e331_d_n11, eq31_e331_d_n12, eq31_e331_d_n13, eq31_e331_d_n14];let eq31_branch_derivatives: [f64; 6] = [eq31_e331_d_b0, eq31_e331_d_b1, eq31_e331_d_b2, eq31_e331_d_b3, eq31_e331_d_b4, eq31_e331_d_b5];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq31_value),
            &eq31_node_derivatives,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let (eq32_e340, eq32_e340_d_n5, eq32_e340_d_n9,) = {
    if ((!s.b[514]) && s.b[516]) {
        let eq32_e338: f64 = (s.v[233] * (nv9 - nv5));
        (eq32_e338, (-s.v[233]), s.v[233],)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e340;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(5),
            multiplicity * (eq32_value),
            5,
            multiplicity * (eq32_e340_d_n5),
            9,
            multiplicity * (eq32_e340_d_n9),
        );let eq33_e343: f64 = (p.p148 * s.v[196]);let eq33_e344: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq33_e343);let eq33_e344_d_n0: f64 = ((p.p148 * s.dn[196][0]) * ddt_scale);let eq33_e344_d_n1: f64 = ((p.p148 * s.dn[196][1]) * ddt_scale);let eq33_e344_d_n2: f64 = ((p.p148 * s.dn[196][2]) * ddt_scale);let eq33_e344_d_n3: f64 = ((p.p148 * s.dn[196][3]) * ddt_scale);let eq33_e344_d_n4: f64 = ((p.p148 * s.dn[196][4]) * ddt_scale);let eq33_e344_d_n5: f64 = ((p.p148 * s.dn[196][5]) * ddt_scale);let eq33_e344_d_n6: f64 = ((p.p148 * s.dn[196][6]) * ddt_scale);let eq33_e344_d_n7: f64 = ((p.p148 * s.dn[196][7]) * ddt_scale);let eq33_e344_d_n8: f64 = ((p.p148 * s.dn[196][8]) * ddt_scale);let eq33_e344_d_n9: f64 = ((p.p148 * s.dn[196][9]) * ddt_scale);let eq33_e344_d_n10: f64 = ((p.p148 * s.dn[196][10]) * ddt_scale);let eq33_e344_d_n11: f64 = ((p.p148 * s.dn[196][11]) * ddt_scale);let eq33_e344_d_n12: f64 = ((p.p148 * s.dn[196][12]) * ddt_scale);let eq33_e344_d_n13: f64 = ((p.p148 * s.dn[196][13]) * ddt_scale);let eq33_e344_d_n14: f64 = ((p.p148 * s.dn[196][14]) * ddt_scale);let eq33_e344_d_b0: f64 = ((p.p148 * s.db[196][0]) * ddt_scale);let eq33_e344_d_b1: f64 = ((p.p148 * s.db[196][1]) * ddt_scale);let eq33_e344_d_b2: f64 = ((p.p148 * s.db[196][2]) * ddt_scale);let eq33_e344_d_b3: f64 = ((p.p148 * s.db[196][3]) * ddt_scale);let eq33_e344_d_b4: f64 = ((p.p148 * s.db[196][4]) * ddt_scale);let eq33_e344_d_b5: f64 = ((p.p148 * s.db[196][5]) * ddt_scale);let eq33_value: f64 = eq33_e344;let eq33_node_derivatives: [f64; 15] = [eq33_e344_d_n0, eq33_e344_d_n1, eq33_e344_d_n2, eq33_e344_d_n3, eq33_e344_d_n4, eq33_e344_d_n5, eq33_e344_d_n6, eq33_e344_d_n7, eq33_e344_d_n8, eq33_e344_d_n9, eq33_e344_d_n10, eq33_e344_d_n11, eq33_e344_d_n12, eq33_e344_d_n13, eq33_e344_d_n14];let eq33_branch_derivatives: [f64; 6] = [eq33_e344_d_b0, eq33_e344_d_b1, eq33_e344_d_b2, eq33_e344_d_b3, eq33_e344_d_b4, eq33_e344_d_b5];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_9(
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
        let nv3 = ctx.node_voltage(nodes[3]);let nv4 = ctx.node_voltage(nodes[4]);let nv9 = ctx.node_voltage(nodes[9]);let eq34_e347: f64 = (p.p148 * s.v[197]);let eq34_e348: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq34_e347);let eq34_e348_d_n0: f64 = ((p.p148 * s.dn[197][0]) * ddt_scale);let eq34_e348_d_n1: f64 = ((p.p148 * s.dn[197][1]) * ddt_scale);let eq34_e348_d_n2: f64 = ((p.p148 * s.dn[197][2]) * ddt_scale);let eq34_e348_d_n3: f64 = ((p.p148 * s.dn[197][3]) * ddt_scale);let eq34_e348_d_n4: f64 = ((p.p148 * s.dn[197][4]) * ddt_scale);let eq34_e348_d_n5: f64 = ((p.p148 * s.dn[197][5]) * ddt_scale);let eq34_e348_d_n6: f64 = ((p.p148 * s.dn[197][6]) * ddt_scale);let eq34_e348_d_n7: f64 = ((p.p148 * s.dn[197][7]) * ddt_scale);let eq34_e348_d_n8: f64 = ((p.p148 * s.dn[197][8]) * ddt_scale);let eq34_e348_d_n9: f64 = ((p.p148 * s.dn[197][9]) * ddt_scale);let eq34_e348_d_n10: f64 = ((p.p148 * s.dn[197][10]) * ddt_scale);let eq34_e348_d_n11: f64 = ((p.p148 * s.dn[197][11]) * ddt_scale);let eq34_e348_d_n12: f64 = ((p.p148 * s.dn[197][12]) * ddt_scale);let eq34_e348_d_n13: f64 = ((p.p148 * s.dn[197][13]) * ddt_scale);let eq34_e348_d_n14: f64 = ((p.p148 * s.dn[197][14]) * ddt_scale);let eq34_e348_d_b0: f64 = ((p.p148 * s.db[197][0]) * ddt_scale);let eq34_e348_d_b1: f64 = ((p.p148 * s.db[197][1]) * ddt_scale);let eq34_e348_d_b2: f64 = ((p.p148 * s.db[197][2]) * ddt_scale);let eq34_e348_d_b3: f64 = ((p.p148 * s.db[197][3]) * ddt_scale);let eq34_e348_d_b4: f64 = ((p.p148 * s.db[197][4]) * ddt_scale);let eq34_e348_d_b5: f64 = ((p.p148 * s.db[197][5]) * ddt_scale);let eq34_value: f64 = eq34_e348;let eq34_node_derivatives: [f64; 15] = [eq34_e348_d_n0, eq34_e348_d_n1, eq34_e348_d_n2, eq34_e348_d_n3, eq34_e348_d_n4, eq34_e348_d_n5, eq34_e348_d_n6, eq34_e348_d_n7, eq34_e348_d_n8, eq34_e348_d_n9, eq34_e348_d_n10, eq34_e348_d_n11, eq34_e348_d_n12, eq34_e348_d_n13, eq34_e348_d_n14];let eq34_branch_derivatives: [f64; 6] = [eq34_e348_d_b0, eq34_e348_d_b1, eq34_e348_d_b2, eq34_e348_d_b3, eq34_e348_d_b4, eq34_e348_d_b5];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(0),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
            multiplicity,
        );
        let (eq35_e354, eq35_e354_d_n3, eq35_e354_d_n9,) = {
    if s.b[517] {
        let __rspice_inv_cse_0: f64 = 1.0 / p.p102;let eq35_e352: f64 = ((nv9 - nv3) * __rspice_inv_cse_0);let eq35_e352_d_n3: f64 = ((-1.0) * __rspice_inv_cse_0);let eq35_e352_d_n9: f64 = (1.0 * __rspice_inv_cse_0);
        (eq35_e352, eq35_e352_d_n3, eq35_e352_d_n9,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e354;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(3),
            multiplicity * (eq35_value),
            3,
            multiplicity * (eq35_e354_d_n3),
            9,
            multiplicity * (eq35_e354_d_n9),
        );
        let (eq36_e363, eq36_e363_d_n3, eq36_e363_d_n9,) = {
    if (s.b[517] && s.b[518]) {
        let eq36_e360: f64 = (p.p103 * (nv9 - nv3));let eq36_e361: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq36_e360);
        (eq36_e361, ((-p.p103) * ddt_scale), (p.p103 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e363;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(3),
            multiplicity * (eq36_value),
            3,
            multiplicity * (eq36_e363_d_n3),
            9,
            multiplicity * (eq36_e363_d_n9),
        );
        let (eq37_e368,) = {
    if (!s.b[517]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq37_value: f64 = eq37_e368;
        stamper.stamp_potential_const_local(
            4,
            eq37_value,
        );
        let (eq38_e376, eq38_e376_d_n0, eq38_e376_d_n1, eq38_e376_d_n2, eq38_e376_d_n3, eq38_e376_d_n4, eq38_e376_d_n5, eq38_e376_d_n6, eq38_e376_d_n7, eq38_e376_d_n8, eq38_e376_d_n9, eq38_e376_d_n10, eq38_e376_d_n11, eq38_e376_d_n12, eq38_e376_d_n13, eq38_e376_d_n14, eq38_e376_d_b0, eq38_e376_d_b1, eq38_e376_d_b2, eq38_e376_d_b3, eq38_e376_d_b4, eq38_e376_d_b5,) = {
    if s.b[519] {
        let eq38_e372: f64 = ((nv4 - 0.0) / s.v[201]);let eq38_e372_d_n0: f64 = (-(((nv4 - 0.0) * s.dn[201][0]) / (s.v[201] * s.v[201])));let eq38_e372_d_n1: f64 = (-(((nv4 - 0.0) * s.dn[201][1]) / (s.v[201] * s.v[201])));let eq38_e372_d_n2: f64 = (-(((nv4 - 0.0) * s.dn[201][2]) / (s.v[201] * s.v[201])));let eq38_e372_d_n3: f64 = (-(((nv4 - 0.0) * s.dn[201][3]) / (s.v[201] * s.v[201])));let eq38_e372_d_n4: f64 = ((s.v[201] - ((nv4 - 0.0) * s.dn[201][4])) / (s.v[201] * s.v[201]));let eq38_e372_d_n5: f64 = (-(((nv4 - 0.0) * s.dn[201][5]) / (s.v[201] * s.v[201])));let eq38_e372_d_n6: f64 = (-(((nv4 - 0.0) * s.dn[201][6]) / (s.v[201] * s.v[201])));let eq38_e372_d_n7: f64 = (-(((nv4 - 0.0) * s.dn[201][7]) / (s.v[201] * s.v[201])));let eq38_e372_d_n8: f64 = (-(((nv4 - 0.0) * s.dn[201][8]) / (s.v[201] * s.v[201])));let eq38_e372_d_n9: f64 = (-(((nv4 - 0.0) * s.dn[201][9]) / (s.v[201] * s.v[201])));let eq38_e372_d_n10: f64 = (-(((nv4 - 0.0) * s.dn[201][10]) / (s.v[201] * s.v[201])));let eq38_e372_d_n11: f64 = (-(((nv4 - 0.0) * s.dn[201][11]) / (s.v[201] * s.v[201])));let eq38_e372_d_n12: f64 = (-(((nv4 - 0.0) * s.dn[201][12]) / (s.v[201] * s.v[201])));let eq38_e372_d_n13: f64 = (-(((nv4 - 0.0) * s.dn[201][13]) / (s.v[201] * s.v[201])));let eq38_e372_d_n14: f64 = (-(((nv4 - 0.0) * s.dn[201][14]) / (s.v[201] * s.v[201])));let eq38_e372_d_b0: f64 = (-(((nv4 - 0.0) * s.db[201][0]) / (s.v[201] * s.v[201])));let eq38_e372_d_b1: f64 = (-(((nv4 - 0.0) * s.db[201][1]) / (s.v[201] * s.v[201])));let eq38_e372_d_b2: f64 = (-(((nv4 - 0.0) * s.db[201][2]) / (s.v[201] * s.v[201])));let eq38_e372_d_b3: f64 = (-(((nv4 - 0.0) * s.db[201][3]) / (s.v[201] * s.v[201])));let eq38_e372_d_b4: f64 = (-(((nv4 - 0.0) * s.db[201][4]) / (s.v[201] * s.v[201])));let eq38_e372_d_b5: f64 = (-(((nv4 - 0.0) * s.db[201][5]) / (s.v[201] * s.v[201])));let eq38_e374: f64 = (eq38_e372 - s.v[200]);let eq38_e374_d_n0: f64 = (eq38_e372_d_n0 - s.dn[200][0]);let eq38_e374_d_n1: f64 = (eq38_e372_d_n1 - s.dn[200][1]);let eq38_e374_d_n2: f64 = (eq38_e372_d_n2 - s.dn[200][2]);let eq38_e374_d_n3: f64 = (eq38_e372_d_n3 - s.dn[200][3]);let eq38_e374_d_n4: f64 = (eq38_e372_d_n4 - s.dn[200][4]);let eq38_e374_d_n5: f64 = (eq38_e372_d_n5 - s.dn[200][5]);let eq38_e374_d_n6: f64 = (eq38_e372_d_n6 - s.dn[200][6]);let eq38_e374_d_n7: f64 = (eq38_e372_d_n7 - s.dn[200][7]);let eq38_e374_d_n8: f64 = (eq38_e372_d_n8 - s.dn[200][8]);let eq38_e374_d_n9: f64 = (eq38_e372_d_n9 - s.dn[200][9]);let eq38_e374_d_n10: f64 = (eq38_e372_d_n10 - s.dn[200][10]);let eq38_e374_d_n11: f64 = (eq38_e372_d_n11 - s.dn[200][11]);let eq38_e374_d_n12: f64 = (eq38_e372_d_n12 - s.dn[200][12]);let eq38_e374_d_n13: f64 = (eq38_e372_d_n13 - s.dn[200][13]);let eq38_e374_d_n14: f64 = (eq38_e372_d_n14 - s.dn[200][14]);let eq38_e374_d_b0: f64 = (eq38_e372_d_b0 - s.db[200][0]);let eq38_e374_d_b1: f64 = (eq38_e372_d_b1 - s.db[200][1]);let eq38_e374_d_b2: f64 = (eq38_e372_d_b2 - s.db[200][2]);let eq38_e374_d_b3: f64 = (eq38_e372_d_b3 - s.db[200][3]);let eq38_e374_d_b4: f64 = (eq38_e372_d_b4 - s.db[200][4]);let eq38_e374_d_b5: f64 = (eq38_e372_d_b5 - s.db[200][5]);
        (eq38_e374, eq38_e374_d_n0, eq38_e374_d_n1, eq38_e374_d_n2, eq38_e374_d_n3, eq38_e374_d_n4, eq38_e374_d_n5, eq38_e374_d_n6, eq38_e374_d_n7, eq38_e374_d_n8, eq38_e374_d_n9, eq38_e374_d_n10, eq38_e374_d_n11, eq38_e374_d_n12, eq38_e374_d_n13, eq38_e374_d_n14, eq38_e374_d_b0, eq38_e374_d_b1, eq38_e374_d_b2, eq38_e374_d_b3, eq38_e374_d_b4, eq38_e374_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e376;let eq38_node_derivatives: [f64; 15] = [eq38_e376_d_n0, eq38_e376_d_n1, eq38_e376_d_n2, eq38_e376_d_n3, eq38_e376_d_n4, eq38_e376_d_n5, eq38_e376_d_n6, eq38_e376_d_n7, eq38_e376_d_n8, eq38_e376_d_n9, eq38_e376_d_n10, eq38_e376_d_n11, eq38_e376_d_n12, eq38_e376_d_n13, eq38_e376_d_n14];let eq38_branch_derivatives: [f64; 6] = [eq38_e376_d_b0, eq38_e376_d_b1, eq38_e376_d_b2, eq38_e376_d_b3, eq38_e376_d_b4, eq38_e376_d_b5];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq38_value),
            &eq38_node_derivatives,
            &eq38_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_10(
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
        let nv4 = ctx.node_voltage(nodes[4]);let nv13 = ctx.node_voltage(nodes[13]);
        let (eq39_e385, eq39_e385_d_n4,) = {
    if (s.b[519] && s.b[520]) {
        let eq39_e382: f64 = (p.p145 * (nv4 - 0.0));let eq39_e383: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq39_e382);
        (eq39_e383, (p.p145 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e385;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq39_value),
            4,
            multiplicity * (eq39_e385_d_n4),
        );
        let (eq40_e390,) = {
    if (!s.b[519]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq40_value: f64 = eq40_e390;
        stamper.stamp_potential_const_local(
            5,
            eq40_value,
        );let eq41_value: f64 = s.v[237];
        stamper.stamp_current_dense_local(
            Some(10),
            None,
            multiplicity * (eq41_value),
            &s.dn[237],
            &s.db[237],
            multiplicity,
        );let eq42_e393: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, s.v[239]);let eq42_value: f64 = eq42_e393;
        stamper.stamp_current_dense_local(
            Some(10),
            None,
            multiplicity * (eq42_value),
            &s.dn[239],
            &s.db[239],
            (multiplicity) * (ddt_scale),
        );let eq43_value: f64 = s.v[238];
        stamper.stamp_current_dense_local(
            Some(11),
            None,
            multiplicity * (eq43_value),
            &s.dn[238],
            &s.db[238],
            multiplicity,
        );let eq44_e396: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, s.v[240]);let eq44_value: f64 = eq44_e396;
        stamper.stamp_current_dense_local(
            Some(11),
            None,
            multiplicity * (eq44_value),
            &s.dn[240],
            &s.db[240],
            (multiplicity) * (ddt_scale),
        );let eq45_value: f64 = s.v[235];
        stamper.stamp_current_dense_local(
            Some(12),
            None,
            multiplicity * (eq45_value),
            &s.dn[235],
            &s.db[235],
            multiplicity,
        );let eq46_e399: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, s.v[236]);let eq46_value: f64 = eq46_e399;
        stamper.stamp_current_dense_local(
            Some(12),
            None,
            multiplicity * (eq46_value),
            &s.dn[236],
            &s.db[236],
            (multiplicity) * (ddt_scale),
        );
        let (eq63_e519, eq63_e519_d_n13,) = {
    if s.b[533] {
        let eq63_e517: f64 = (-(nv13 - 0.0));
        (eq63_e517, (-1.0),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e519;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq63_value),
            13,
            multiplicity * (eq63_e519_d_n13),
        );
        let (eq64_e523, eq64_e523_d_n13,) = {
    if s.b[533] {
        ((nv13 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq64_value: f64 = eq64_e523;
        stamper.stamp_current_node1_local(
            Some(8),
            Some(6),
            multiplicity * (eq64_value),
            13,
            multiplicity * (eq64_e523_d_n13),
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_11(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq65_e534, eq65_e534_d_n0, eq65_e534_d_n1, eq65_e534_d_n2, eq65_e534_d_n3, eq65_e534_d_n4, eq65_e534_d_n5, eq65_e534_d_n6, eq65_e534_d_n7, eq65_e534_d_n8, eq65_e534_d_n9, eq65_e534_d_n10, eq65_e534_d_n11, eq65_e534_d_n12, eq65_e534_d_n13, eq65_e534_d_n14, eq65_e534_d_b0, eq65_e534_d_b1, eq65_e534_d_b2, eq65_e534_d_b3, eq65_e534_d_b4, eq65_e534_d_b5,) = {
    if s.b[533] {
        let eq65_e527: f64 = (s.v[537] / s.v[535]);let __rspice_inv_cse_0: f64 = 1.0 / (s.v[535] * s.v[535]);let eq65_e527_d_n0: f64 = (((s.dn[537][0] * s.v[535]) - (s.v[537] * s.dn[535][0])) * __rspice_inv_cse_0);let eq65_e527_d_n1: f64 = (((s.dn[537][1] * s.v[535]) - (s.v[537] * s.dn[535][1])) * __rspice_inv_cse_0);let eq65_e527_d_n2: f64 = (((s.dn[537][2] * s.v[535]) - (s.v[537] * s.dn[535][2])) * __rspice_inv_cse_0);let eq65_e527_d_n3: f64 = (((s.dn[537][3] * s.v[535]) - (s.v[537] * s.dn[535][3])) * __rspice_inv_cse_0);let eq65_e527_d_n4: f64 = (((s.dn[537][4] * s.v[535]) - (s.v[537] * s.dn[535][4])) * __rspice_inv_cse_0);let eq65_e527_d_n5: f64 = (((s.dn[537][5] * s.v[535]) - (s.v[537] * s.dn[535][5])) * __rspice_inv_cse_0);let eq65_e527_d_n6: f64 = (((s.dn[537][6] * s.v[535]) - (s.v[537] * s.dn[535][6])) * __rspice_inv_cse_0);let eq65_e527_d_n7: f64 = (((s.dn[537][7] * s.v[535]) - (s.v[537] * s.dn[535][7])) * __rspice_inv_cse_0);let eq65_e527_d_n8: f64 = (((s.dn[537][8] * s.v[535]) - (s.v[537] * s.dn[535][8])) * __rspice_inv_cse_0);let eq65_e527_d_n9: f64 = (((s.dn[537][9] * s.v[535]) - (s.v[537] * s.dn[535][9])) * __rspice_inv_cse_0);let eq65_e527_d_n10: f64 = (((s.dn[537][10] * s.v[535]) - (s.v[537] * s.dn[535][10])) * __rspice_inv_cse_0);let eq65_e527_d_n11: f64 = (((s.dn[537][11] * s.v[535]) - (s.v[537] * s.dn[535][11])) * __rspice_inv_cse_0);let eq65_e527_d_n12: f64 = (((s.dn[537][12] * s.v[535]) - (s.v[537] * s.dn[535][12])) * __rspice_inv_cse_0);let eq65_e527_d_n13: f64 = (((s.dn[537][13] * s.v[535]) - (s.v[537] * s.dn[535][13])) * __rspice_inv_cse_0);let eq65_e527_d_n14: f64 = (((s.dn[537][14] * s.v[535]) - (s.v[537] * s.dn[535][14])) * __rspice_inv_cse_0);let eq65_e527_d_b0: f64 = (((s.db[537][0] * s.v[535]) - (s.v[537] * s.db[535][0])) * __rspice_inv_cse_0);let eq65_e527_d_b1: f64 = (((s.db[537][1] * s.v[535]) - (s.v[537] * s.db[535][1])) * __rspice_inv_cse_0);let eq65_e527_d_b2: f64 = (((s.db[537][2] * s.v[535]) - (s.v[537] * s.db[535][2])) * __rspice_inv_cse_0);let eq65_e527_d_b3: f64 = (((s.db[537][3] * s.v[535]) - (s.v[537] * s.db[535][3])) * __rspice_inv_cse_0);let eq65_e527_d_b4: f64 = (((s.db[537][4] * s.v[535]) - (s.v[537] * s.db[535][4])) * __rspice_inv_cse_0);let eq65_e527_d_b5: f64 = (((s.db[537][5] * s.v[535]) - (s.v[537] * s.db[535][5])) * __rspice_inv_cse_0);let eq65_e530: f64 = (s.v[535] * (nv13 - 0.0));let eq65_e530_d_n0: f64 = (s.dn[535][0] * (nv13 - 0.0));let eq65_e530_d_n1: f64 = (s.dn[535][1] * (nv13 - 0.0));let eq65_e530_d_n2: f64 = (s.dn[535][2] * (nv13 - 0.0));let eq65_e530_d_n3: f64 = (s.dn[535][3] * (nv13 - 0.0));let eq65_e530_d_n4: f64 = (s.dn[535][4] * (nv13 - 0.0));let eq65_e530_d_n5: f64 = (s.dn[535][5] * (nv13 - 0.0));let eq65_e530_d_n6: f64 = (s.dn[535][6] * (nv13 - 0.0));let eq65_e530_d_n7: f64 = (s.dn[535][7] * (nv13 - 0.0));let eq65_e530_d_n8: f64 = (s.dn[535][8] * (nv13 - 0.0));let eq65_e530_d_n9: f64 = (s.dn[535][9] * (nv13 - 0.0));let eq65_e530_d_n10: f64 = (s.dn[535][10] * (nv13 - 0.0));let eq65_e530_d_n11: f64 = (s.dn[535][11] * (nv13 - 0.0));let eq65_e530_d_n12: f64 = (s.dn[535][12] * (nv13 - 0.0));let eq65_e530_d_n13: f64 = ((s.dn[535][13] * (nv13 - 0.0)) + s.v[535]);let eq65_e530_d_n14: f64 = (s.dn[535][14] * (nv13 - 0.0));let eq65_e530_d_b0: f64 = (s.db[535][0] * (nv13 - 0.0));let eq65_e530_d_b1: f64 = (s.db[535][1] * (nv13 - 0.0));let eq65_e530_d_b2: f64 = (s.db[535][2] * (nv13 - 0.0));let eq65_e530_d_b3: f64 = (s.db[535][3] * (nv13 - 0.0));let eq65_e530_d_b4: f64 = (s.db[535][4] * (nv13 - 0.0));let eq65_e530_d_b5: f64 = (s.db[535][5] * (nv13 - 0.0));let eq65_e531: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, eq65_e530);let eq65_e532: f64 = (eq65_e527 * eq65_e531);let eq65_e532_d_n0: f64 = ((eq65_e527_d_n0 * eq65_e531) + (eq65_e527 * (eq65_e530_d_n0 * ddt_scale)));
        let eq65_e532_d_n1: f64 = ((eq65_e527_d_n1 * eq65_e531) + (eq65_e527 * (eq65_e530_d_n1 * ddt_scale)));let eq65_e532_d_n2: f64 = ((eq65_e527_d_n2 * eq65_e531) + (eq65_e527 * (eq65_e530_d_n2 * ddt_scale)));let eq65_e532_d_n3: f64 = ((eq65_e527_d_n3 * eq65_e531) + (eq65_e527 * (eq65_e530_d_n3 * ddt_scale)));let eq65_e532_d_n4: f64 = ((eq65_e527_d_n4 * eq65_e531) + (eq65_e527 * (eq65_e530_d_n4 * ddt_scale)));let eq65_e532_d_n5: f64 = ((eq65_e527_d_n5 * eq65_e531) + (eq65_e527 * (eq65_e530_d_n5 * ddt_scale)));let eq65_e532_d_n6: f64 = ((eq65_e527_d_n6 * eq65_e531) + (eq65_e527 * (eq65_e530_d_n6 * ddt_scale)));let eq65_e532_d_n7: f64 = ((eq65_e527_d_n7 * eq65_e531) + (eq65_e527 * (eq65_e530_d_n7 * ddt_scale)));let eq65_e532_d_n8: f64 = ((eq65_e527_d_n8 * eq65_e531) + (eq65_e527 * (eq65_e530_d_n8 * ddt_scale)));let eq65_e532_d_n9: f64 = ((eq65_e527_d_n9 * eq65_e531) + (eq65_e527 * (eq65_e530_d_n9 * ddt_scale)));let eq65_e532_d_n10: f64 = ((eq65_e527_d_n10 * eq65_e531) + (eq65_e527 * (eq65_e530_d_n10 * ddt_scale)));let eq65_e532_d_n11: f64 = ((eq65_e527_d_n11 * eq65_e531) + (eq65_e527 * (eq65_e530_d_n11 * ddt_scale)));let eq65_e532_d_n12: f64 = ((eq65_e527_d_n12 * eq65_e531) + (eq65_e527 * (eq65_e530_d_n12 * ddt_scale)));let eq65_e532_d_n13: f64 = ((eq65_e527_d_n13 * eq65_e531) + (eq65_e527 * (eq65_e530_d_n13 * ddt_scale)));let eq65_e532_d_n14: f64 = ((eq65_e527_d_n14 * eq65_e531) + (eq65_e527 * (eq65_e530_d_n14 * ddt_scale)));let eq65_e532_d_b0: f64 = ((eq65_e527_d_b0 * eq65_e531) + (eq65_e527 * (eq65_e530_d_b0 * ddt_scale)));let eq65_e532_d_b1: f64 = ((eq65_e527_d_b1 * eq65_e531) + (eq65_e527 * (eq65_e530_d_b1 * ddt_scale)));let eq65_e532_d_b2: f64 = ((eq65_e527_d_b2 * eq65_e531) + (eq65_e527 * (eq65_e530_d_b2 * ddt_scale)));let eq65_e532_d_b3: f64 = ((eq65_e527_d_b3 * eq65_e531) + (eq65_e527 * (eq65_e530_d_b3 * ddt_scale)));let eq65_e532_d_b4: f64 = ((eq65_e527_d_b4 * eq65_e531) + (eq65_e527 * (eq65_e530_d_b4 * ddt_scale)));let eq65_e532_d_b5: f64 = ((eq65_e527_d_b5 * eq65_e531) + (eq65_e527 * (eq65_e530_d_b5 * ddt_scale)));
        (eq65_e532, eq65_e532_d_n0, eq65_e532_d_n1, eq65_e532_d_n2, eq65_e532_d_n3, eq65_e532_d_n4, eq65_e532_d_n5, eq65_e532_d_n6, eq65_e532_d_n7, eq65_e532_d_n8, eq65_e532_d_n9, eq65_e532_d_n10, eq65_e532_d_n11, eq65_e532_d_n12, eq65_e532_d_n13, eq65_e532_d_n14, eq65_e532_d_b0, eq65_e532_d_b1, eq65_e532_d_b2, eq65_e532_d_b3, eq65_e532_d_b4, eq65_e532_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e534;let eq65_node_derivatives: [f64; 15] = [eq65_e534_d_n0, eq65_e534_d_n1, eq65_e534_d_n2, eq65_e534_d_n3, eq65_e534_d_n4, eq65_e534_d_n5, eq65_e534_d_n6, eq65_e534_d_n7, eq65_e534_d_n8, eq65_e534_d_n9, eq65_e534_d_n10, eq65_e534_d_n11, eq65_e534_d_n12, eq65_e534_d_n13, eq65_e534_d_n14];let eq65_branch_derivatives: [f64; 6] = [eq65_e534_d_b0, eq65_e534_d_b1, eq65_e534_d_b2, eq65_e534_d_b3, eq65_e534_d_b4, eq65_e534_d_b5];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq65_value),
            &eq65_node_derivatives,
            &eq65_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_12(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
        let nv13 = ctx.node_voltage(nodes[13]);let nv14 = ctx.node_voltage(nodes[14]);
        let (eq66_e545, eq66_e545_d_n0, eq66_e545_d_n1, eq66_e545_d_n2, eq66_e545_d_n3, eq66_e545_d_n4, eq66_e545_d_n5, eq66_e545_d_n6, eq66_e545_d_n7, eq66_e545_d_n8, eq66_e545_d_n9, eq66_e545_d_n10, eq66_e545_d_n11, eq66_e545_d_n12, eq66_e545_d_n13, eq66_e545_d_n14, eq66_e545_d_b0, eq66_e545_d_b1, eq66_e545_d_b2, eq66_e545_d_b3, eq66_e545_d_b4, eq66_e545_d_b5,) = {
    if s.b[533] {
        let eq66_e538: f64 = (s.v[536] / s.v[535]);let __rspice_inv_cse_0: f64 = 1.0 / (s.v[535] * s.v[535]);let eq66_e538_d_n0: f64 = (((s.dn[536][0] * s.v[535]) - (s.v[536] * s.dn[535][0])) * __rspice_inv_cse_0);let eq66_e538_d_n1: f64 = (((s.dn[536][1] * s.v[535]) - (s.v[536] * s.dn[535][1])) * __rspice_inv_cse_0);let eq66_e538_d_n2: f64 = (((s.dn[536][2] * s.v[535]) - (s.v[536] * s.dn[535][2])) * __rspice_inv_cse_0);let eq66_e538_d_n3: f64 = (((s.dn[536][3] * s.v[535]) - (s.v[536] * s.dn[535][3])) * __rspice_inv_cse_0);let eq66_e538_d_n4: f64 = (((s.dn[536][4] * s.v[535]) - (s.v[536] * s.dn[535][4])) * __rspice_inv_cse_0);let eq66_e538_d_n5: f64 = (((s.dn[536][5] * s.v[535]) - (s.v[536] * s.dn[535][5])) * __rspice_inv_cse_0);let eq66_e538_d_n6: f64 = (((s.dn[536][6] * s.v[535]) - (s.v[536] * s.dn[535][6])) * __rspice_inv_cse_0);let eq66_e538_d_n7: f64 = (((s.dn[536][7] * s.v[535]) - (s.v[536] * s.dn[535][7])) * __rspice_inv_cse_0);let eq66_e538_d_n8: f64 = (((s.dn[536][8] * s.v[535]) - (s.v[536] * s.dn[535][8])) * __rspice_inv_cse_0);let eq66_e538_d_n9: f64 = (((s.dn[536][9] * s.v[535]) - (s.v[536] * s.dn[535][9])) * __rspice_inv_cse_0);let eq66_e538_d_n10: f64 = (((s.dn[536][10] * s.v[535]) - (s.v[536] * s.dn[535][10])) * __rspice_inv_cse_0);let eq66_e538_d_n11: f64 = (((s.dn[536][11] * s.v[535]) - (s.v[536] * s.dn[535][11])) * __rspice_inv_cse_0);let eq66_e538_d_n12: f64 = (((s.dn[536][12] * s.v[535]) - (s.v[536] * s.dn[535][12])) * __rspice_inv_cse_0);let eq66_e538_d_n13: f64 = (((s.dn[536][13] * s.v[535]) - (s.v[536] * s.dn[535][13])) * __rspice_inv_cse_0);let eq66_e538_d_n14: f64 = (((s.dn[536][14] * s.v[535]) - (s.v[536] * s.dn[535][14])) * __rspice_inv_cse_0);let eq66_e538_d_b0: f64 = (((s.db[536][0] * s.v[535]) - (s.v[536] * s.db[535][0])) * __rspice_inv_cse_0);let eq66_e538_d_b1: f64 = (((s.db[536][1] * s.v[535]) - (s.v[536] * s.db[535][1])) * __rspice_inv_cse_0);let eq66_e538_d_b2: f64 = (((s.db[536][2] * s.v[535]) - (s.v[536] * s.db[535][2])) * __rspice_inv_cse_0);let eq66_e538_d_b3: f64 = (((s.db[536][3] * s.v[535]) - (s.v[536] * s.db[535][3])) * __rspice_inv_cse_0);let eq66_e538_d_b4: f64 = (((s.db[536][4] * s.v[535]) - (s.v[536] * s.db[535][4])) * __rspice_inv_cse_0);let eq66_e538_d_b5: f64 = (((s.db[536][5] * s.v[535]) - (s.v[536] * s.db[535][5])) * __rspice_inv_cse_0);let eq66_e541: f64 = (s.v[535] * (nv14 - 0.0));let eq66_e541_d_n0: f64 = (s.dn[535][0] * (nv14 - 0.0));let eq66_e541_d_n1: f64 = (s.dn[535][1] * (nv14 - 0.0));let eq66_e541_d_n2: f64 = (s.dn[535][2] * (nv14 - 0.0));let eq66_e541_d_n3: f64 = (s.dn[535][3] * (nv14 - 0.0));let eq66_e541_d_n4: f64 = (s.dn[535][4] * (nv14 - 0.0));let eq66_e541_d_n5: f64 = (s.dn[535][5] * (nv14 - 0.0));let eq66_e541_d_n6: f64 = (s.dn[535][6] * (nv14 - 0.0));let eq66_e541_d_n7: f64 = (s.dn[535][7] * (nv14 - 0.0));let eq66_e541_d_n8: f64 = (s.dn[535][8] * (nv14 - 0.0));let eq66_e541_d_n9: f64 = (s.dn[535][9] * (nv14 - 0.0));let eq66_e541_d_n10: f64 = (s.dn[535][10] * (nv14 - 0.0));let eq66_e541_d_n11: f64 = (s.dn[535][11] * (nv14 - 0.0));let eq66_e541_d_n12: f64 = (s.dn[535][12] * (nv14 - 0.0));let eq66_e541_d_n13: f64 = (s.dn[535][13] * (nv14 - 0.0));let eq66_e541_d_n14: f64 = ((s.dn[535][14] * (nv14 - 0.0)) + s.v[535]);let eq66_e541_d_b0: f64 = (s.db[535][0] * (nv14 - 0.0));let eq66_e541_d_b1: f64 = (s.db[535][1] * (nv14 - 0.0));let eq66_e541_d_b2: f64 = (s.db[535][2] * (nv14 - 0.0));let eq66_e541_d_b3: f64 = (s.db[535][3] * (nv14 - 0.0));let eq66_e541_d_b4: f64 = (s.db[535][4] * (nv14 - 0.0));let eq66_e541_d_b5: f64 = (s.db[535][5] * (nv14 - 0.0));let eq66_e542: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, eq66_e541);let eq66_e543: f64 = (eq66_e538 * eq66_e542);let eq66_e543_d_n0: f64 = ((eq66_e538_d_n0 * eq66_e542) + (eq66_e538 * (eq66_e541_d_n0 * ddt_scale)));
        let eq66_e543_d_n1: f64 = ((eq66_e538_d_n1 * eq66_e542) + (eq66_e538 * (eq66_e541_d_n1 * ddt_scale)));let eq66_e543_d_n2: f64 = ((eq66_e538_d_n2 * eq66_e542) + (eq66_e538 * (eq66_e541_d_n2 * ddt_scale)));let eq66_e543_d_n3: f64 = ((eq66_e538_d_n3 * eq66_e542) + (eq66_e538 * (eq66_e541_d_n3 * ddt_scale)));let eq66_e543_d_n4: f64 = ((eq66_e538_d_n4 * eq66_e542) + (eq66_e538 * (eq66_e541_d_n4 * ddt_scale)));let eq66_e543_d_n5: f64 = ((eq66_e538_d_n5 * eq66_e542) + (eq66_e538 * (eq66_e541_d_n5 * ddt_scale)));let eq66_e543_d_n6: f64 = ((eq66_e538_d_n6 * eq66_e542) + (eq66_e538 * (eq66_e541_d_n6 * ddt_scale)));let eq66_e543_d_n7: f64 = ((eq66_e538_d_n7 * eq66_e542) + (eq66_e538 * (eq66_e541_d_n7 * ddt_scale)));let eq66_e543_d_n8: f64 = ((eq66_e538_d_n8 * eq66_e542) + (eq66_e538 * (eq66_e541_d_n8 * ddt_scale)));let eq66_e543_d_n9: f64 = ((eq66_e538_d_n9 * eq66_e542) + (eq66_e538 * (eq66_e541_d_n9 * ddt_scale)));let eq66_e543_d_n10: f64 = ((eq66_e538_d_n10 * eq66_e542) + (eq66_e538 * (eq66_e541_d_n10 * ddt_scale)));let eq66_e543_d_n11: f64 = ((eq66_e538_d_n11 * eq66_e542) + (eq66_e538 * (eq66_e541_d_n11 * ddt_scale)));let eq66_e543_d_n12: f64 = ((eq66_e538_d_n12 * eq66_e542) + (eq66_e538 * (eq66_e541_d_n12 * ddt_scale)));let eq66_e543_d_n13: f64 = ((eq66_e538_d_n13 * eq66_e542) + (eq66_e538 * (eq66_e541_d_n13 * ddt_scale)));let eq66_e543_d_n14: f64 = ((eq66_e538_d_n14 * eq66_e542) + (eq66_e538 * (eq66_e541_d_n14 * ddt_scale)));let eq66_e543_d_b0: f64 = ((eq66_e538_d_b0 * eq66_e542) + (eq66_e538 * (eq66_e541_d_b0 * ddt_scale)));let eq66_e543_d_b1: f64 = ((eq66_e538_d_b1 * eq66_e542) + (eq66_e538 * (eq66_e541_d_b1 * ddt_scale)));let eq66_e543_d_b2: f64 = ((eq66_e538_d_b2 * eq66_e542) + (eq66_e538 * (eq66_e541_d_b2 * ddt_scale)));let eq66_e543_d_b3: f64 = ((eq66_e538_d_b3 * eq66_e542) + (eq66_e538 * (eq66_e541_d_b3 * ddt_scale)));let eq66_e543_d_b4: f64 = ((eq66_e538_d_b4 * eq66_e542) + (eq66_e538 * (eq66_e541_d_b4 * ddt_scale)));let eq66_e543_d_b5: f64 = ((eq66_e538_d_b5 * eq66_e542) + (eq66_e538 * (eq66_e541_d_b5 * ddt_scale)));
        (eq66_e543, eq66_e543_d_n0, eq66_e543_d_n1, eq66_e543_d_n2, eq66_e543_d_n3, eq66_e543_d_n4, eq66_e543_d_n5, eq66_e543_d_n6, eq66_e543_d_n7, eq66_e543_d_n8, eq66_e543_d_n9, eq66_e543_d_n10, eq66_e543_d_n11, eq66_e543_d_n12, eq66_e543_d_n13, eq66_e543_d_n14, eq66_e543_d_b0, eq66_e543_d_b1, eq66_e543_d_b2, eq66_e543_d_b3, eq66_e543_d_b4, eq66_e543_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq66_value: f64 = eq66_e545;let eq66_node_derivatives: [f64; 15] = [eq66_e545_d_n0, eq66_e545_d_n1, eq66_e545_d_n2, eq66_e545_d_n3, eq66_e545_d_n4, eq66_e545_d_n5, eq66_e545_d_n6, eq66_e545_d_n7, eq66_e545_d_n8, eq66_e545_d_n9, eq66_e545_d_n10, eq66_e545_d_n11, eq66_e545_d_n12, eq66_e545_d_n13, eq66_e545_d_n14];let eq66_branch_derivatives: [f64; 6] = [eq66_e545_d_b0, eq66_e545_d_b1, eq66_e545_d_b2, eq66_e545_d_b3, eq66_e545_d_b4, eq66_e545_d_b5];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq66_value),
            &eq66_node_derivatives,
            &eq66_branch_derivatives,
            multiplicity,
        );
        let (eq68_e559, eq68_e559_d_n14,) = {
    if s.b[533] {
        let eq68_e557: f64 = (-(nv14 - 0.0));
        (eq68_e557, (-1.0),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq68_value: f64 = eq68_e559;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (eq68_value),
            14,
            multiplicity * (eq68_e559_d_n14),
        );
        let (eq69_e563, eq69_e563_d_n14,) = {
    if s.b[533] {
        ((nv14 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e563;
        stamper.stamp_current_node1_local(
            Some(5),
            Some(6),
            multiplicity * (eq69_value),
            14,
            multiplicity * (eq69_e563_d_n14),
        );
        let (eq72_e588, eq72_e588_d_n13,) = {
    if (!s.b[533]) {
        ((nv13 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq72_value: f64 = eq72_e588;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq72_value),
            13,
            multiplicity * (eq72_e588_d_n13),
        );
        let (eq73_e593, eq73_e593_d_n14,) = {
    if (!s.b[533]) {
        ((nv14 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq73_value: f64 = eq73_e593;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (eq73_value),
            14,
            multiplicity * (eq73_e593_d_n14),
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq1_e170: f64 = (s.v[242] + s.v[179]);let eq1_e170_d_n0: f64 = (s.dn[242][0] + s.dn[179][0]);let eq1_e170_d_n1: f64 = (s.dn[242][1] + s.dn[179][1]);let eq1_e170_d_n2: f64 = (s.dn[242][2] + s.dn[179][2]);let eq1_e170_d_n3: f64 = (s.dn[242][3] + s.dn[179][3]);let eq1_e170_d_n4: f64 = (s.dn[242][4] + s.dn[179][4]);let eq1_e170_d_n5: f64 = (s.dn[242][5] + s.dn[179][5]);let eq1_e170_d_n6: f64 = (s.dn[242][6] + s.dn[179][6]);let eq1_e170_d_n7: f64 = (s.dn[242][7] + s.dn[179][7]);let eq1_e170_d_n8: f64 = (s.dn[242][8] + s.dn[179][8]);let eq1_e170_d_n9: f64 = (s.dn[242][9] + s.dn[179][9]);let eq1_e170_d_n10: f64 = (s.dn[242][10] + s.dn[179][10]);let eq1_e170_d_n11: f64 = (s.dn[242][11] + s.dn[179][11]);let eq1_e170_d_n12: f64 = (s.dn[242][12] + s.dn[179][12]);let eq1_e170_d_n13: f64 = (s.dn[242][13] + s.dn[179][13]);let eq1_e170_d_n14: f64 = (s.dn[242][14] + s.dn[179][14]);let eq1_e170_d_b0: f64 = (s.db[242][0] + s.db[179][0]);let eq1_e170_d_b1: f64 = (s.db[242][1] + s.db[179][1]);let eq1_e170_d_b2: f64 = (s.db[242][2] + s.db[179][2]);let eq1_e170_d_b3: f64 = (s.db[242][3] + s.db[179][3]);let eq1_e170_d_b4: f64 = (s.db[242][4] + s.db[179][4]);let eq1_e170_d_b5: f64 = (s.db[242][5] + s.db[179][5]);let eq1_e171: f64 = (p.p148 * eq1_e170);let eq1_e171_d_n0: f64 = (p.p148 * eq1_e170_d_n0);let eq1_e171_d_n1: f64 = (p.p148 * eq1_e170_d_n1);let eq1_e171_d_n2: f64 = (p.p148 * eq1_e170_d_n2);let eq1_e171_d_n3: f64 = (p.p148 * eq1_e170_d_n3);let eq1_e171_d_n4: f64 = (p.p148 * eq1_e170_d_n4);let eq1_e171_d_n5: f64 = (p.p148 * eq1_e170_d_n5);let eq1_e171_d_n6: f64 = (p.p148 * eq1_e170_d_n6);let eq1_e171_d_n7: f64 = (p.p148 * eq1_e170_d_n7);let eq1_e171_d_n8: f64 = (p.p148 * eq1_e170_d_n8);let eq1_e171_d_n9: f64 = (p.p148 * eq1_e170_d_n9);let eq1_e171_d_n10: f64 = (p.p148 * eq1_e170_d_n10);let eq1_e171_d_n11: f64 = (p.p148 * eq1_e170_d_n11);let eq1_e171_d_n12: f64 = (p.p148 * eq1_e170_d_n12);let eq1_e171_d_n13: f64 = (p.p148 * eq1_e170_d_n13);let eq1_e171_d_n14: f64 = (p.p148 * eq1_e170_d_n14);let eq1_e171_d_b0: f64 = (p.p148 * eq1_e170_d_b0);let eq1_e171_d_b1: f64 = (p.p148 * eq1_e170_d_b1);let eq1_e171_d_b2: f64 = (p.p148 * eq1_e170_d_b2);let eq1_e171_d_b3: f64 = (p.p148 * eq1_e170_d_b3);let eq1_e171_d_b4: f64 = (p.p148 * eq1_e170_d_b4);let eq1_e171_d_b5: f64 = (p.p148 * eq1_e170_d_b5);let eq1_e172_q: f64 = eq1_e171;let eq1_reactive_node_derivatives: [f64; 15] = [eq1_e171_d_n0, eq1_e171_d_n1, eq1_e171_d_n2, eq1_e171_d_n3, eq1_e171_d_n4, eq1_e171_d_n5, eq1_e171_d_n6, eq1_e171_d_n7, eq1_e171_d_n8, eq1_e171_d_n9, eq1_e171_d_n10, eq1_e171_d_n11, eq1_e171_d_n12, eq1_e171_d_n13, eq1_e171_d_n14];let eq1_reactive_branch_derivatives: [f64; 6] = [eq1_e171_d_b0, eq1_e171_d_b1, eq1_e171_d_b2, eq1_e171_d_b3, eq1_e171_d_b4, eq1_e171_d_b5];
        stamper.stamp_current_reactive_dense_local(
            Some(8),
            Some(6),
            &eq1_reactive_node_derivatives,
            &eq1_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_1(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq3_e185: f64 = (s.v[182] + s.v[178]);let eq3_e185_d_n0: f64 = (s.dn[182][0] + s.dn[178][0]);let eq3_e185_d_n1: f64 = (s.dn[182][1] + s.dn[178][1]);let eq3_e185_d_n2: f64 = (s.dn[182][2] + s.dn[178][2]);let eq3_e185_d_n3: f64 = (s.dn[182][3] + s.dn[178][3]);let eq3_e185_d_n4: f64 = (s.dn[182][4] + s.dn[178][4]);let eq3_e185_d_n5: f64 = (s.dn[182][5] + s.dn[178][5]);let eq3_e185_d_n6: f64 = (s.dn[182][6] + s.dn[178][6]);let eq3_e185_d_n7: f64 = (s.dn[182][7] + s.dn[178][7]);let eq3_e185_d_n8: f64 = (s.dn[182][8] + s.dn[178][8]);let eq3_e185_d_n9: f64 = (s.dn[182][9] + s.dn[178][9]);let eq3_e185_d_n10: f64 = (s.dn[182][10] + s.dn[178][10]);let eq3_e185_d_n11: f64 = (s.dn[182][11] + s.dn[178][11]);let eq3_e185_d_n12: f64 = (s.dn[182][12] + s.dn[178][12]);let eq3_e185_d_n13: f64 = (s.dn[182][13] + s.dn[178][13]);let eq3_e185_d_n14: f64 = (s.dn[182][14] + s.dn[178][14]);let eq3_e185_d_b0: f64 = (s.db[182][0] + s.db[178][0]);let eq3_e185_d_b1: f64 = (s.db[182][1] + s.db[178][1]);let eq3_e185_d_b2: f64 = (s.db[182][2] + s.db[178][2]);let eq3_e185_d_b3: f64 = (s.db[182][3] + s.db[178][3]);let eq3_e185_d_b4: f64 = (s.db[182][4] + s.db[178][4]);let eq3_e185_d_b5: f64 = (s.db[182][5] + s.db[178][5]);let eq3_e186: f64 = (p.p148 * eq3_e185);let eq3_e186_d_n0: f64 = (p.p148 * eq3_e185_d_n0);let eq3_e186_d_n1: f64 = (p.p148 * eq3_e185_d_n1);let eq3_e186_d_n2: f64 = (p.p148 * eq3_e185_d_n2);let eq3_e186_d_n3: f64 = (p.p148 * eq3_e185_d_n3);let eq3_e186_d_n4: f64 = (p.p148 * eq3_e185_d_n4);let eq3_e186_d_n5: f64 = (p.p148 * eq3_e185_d_n5);let eq3_e186_d_n6: f64 = (p.p148 * eq3_e185_d_n6);let eq3_e186_d_n7: f64 = (p.p148 * eq3_e185_d_n7);let eq3_e186_d_n8: f64 = (p.p148 * eq3_e185_d_n8);let eq3_e186_d_n9: f64 = (p.p148 * eq3_e185_d_n9);let eq3_e186_d_n10: f64 = (p.p148 * eq3_e185_d_n10);let eq3_e186_d_n11: f64 = (p.p148 * eq3_e185_d_n11);let eq3_e186_d_n12: f64 = (p.p148 * eq3_e185_d_n12);let eq3_e186_d_n13: f64 = (p.p148 * eq3_e185_d_n13);let eq3_e186_d_n14: f64 = (p.p148 * eq3_e185_d_n14);let eq3_e186_d_b0: f64 = (p.p148 * eq3_e185_d_b0);let eq3_e186_d_b1: f64 = (p.p148 * eq3_e185_d_b1);let eq3_e186_d_b2: f64 = (p.p148 * eq3_e185_d_b2);let eq3_e186_d_b3: f64 = (p.p148 * eq3_e185_d_b3);let eq3_e186_d_b4: f64 = (p.p148 * eq3_e185_d_b4);let eq3_e186_d_b5: f64 = (p.p148 * eq3_e185_d_b5);let eq3_e187_q: f64 = eq3_e186;let eq3_reactive_node_derivatives: [f64; 15] = [eq3_e186_d_n0, eq3_e186_d_n1, eq3_e186_d_n2, eq3_e186_d_n3, eq3_e186_d_n4, eq3_e186_d_n5, eq3_e186_d_n6, eq3_e186_d_n7, eq3_e186_d_n8, eq3_e186_d_n9, eq3_e186_d_n10, eq3_e186_d_n11, eq3_e186_d_n12, eq3_e186_d_n13, eq3_e186_d_n14];let eq3_reactive_branch_derivatives: [f64; 6] = [eq3_e186_d_b0, eq3_e186_d_b1, eq3_e186_d_b2, eq3_e186_d_b3, eq3_e186_d_b4, eq3_e186_d_b5];
        stamper.stamp_current_reactive_dense_local(
            Some(8),
            Some(5),
            &eq3_reactive_node_derivatives,
            &eq3_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq7_e206, eq7_e206_d_n0, eq7_e206_d_n1, eq7_e206_d_n2, eq7_e206_d_n3, eq7_e206_d_n4, eq7_e206_d_n5, eq7_e206_d_n6, eq7_e206_d_n7, eq7_e206_d_n8, eq7_e206_d_n9, eq7_e206_d_n10, eq7_e206_d_n11, eq7_e206_d_n12, eq7_e206_d_n13, eq7_e206_d_n14, eq7_e206_d_b0, eq7_e206_d_b1, eq7_e206_d_b2, eq7_e206_d_b3, eq7_e206_d_b4, eq7_e206_d_b5, eq7_e206_q,) = {
    if (s.b[508] && s.b[509]) {
        let eq7_e204_q: f64 = s.v[183];
        (s.v[183], s.dn[183][0], s.dn[183][1], s.dn[183][2], s.dn[183][3], s.dn[183][4], s.dn[183][5], s.dn[183][6], s.dn[183][7], s.dn[183][8], s.dn[183][9], s.dn[183][10], s.dn[183][11], s.dn[183][12], s.dn[183][13], s.dn[183][14], s.db[183][0], s.db[183][1], s.db[183][2], s.db[183][3], s.db[183][4], s.db[183][5], eq7_e204_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_reactive_node_derivatives: [f64; 15] = [eq7_e206_d_n0, eq7_e206_d_n1, eq7_e206_d_n2, eq7_e206_d_n3, eq7_e206_d_n4, eq7_e206_d_n5, eq7_e206_d_n6, eq7_e206_d_n7, eq7_e206_d_n8, eq7_e206_d_n9, eq7_e206_d_n10, eq7_e206_d_n11, eq7_e206_d_n12, eq7_e206_d_n13, eq7_e206_d_n14];let eq7_reactive_branch_derivatives: [f64; 6] = [eq7_e206_d_b0, eq7_e206_d_b1, eq7_e206_d_b2, eq7_e206_d_b3, eq7_e206_d_b4, eq7_e206_d_b5];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(8),
            &eq7_reactive_node_derivatives,
            &eq7_reactive_branch_derivatives,
            multiplicity,
        );let eq13_e238: f64 = (p.p148 * s.v[180]);let eq13_e239_q: f64 = eq13_e238;
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(6),
            &s.dn[180],
            &s.db[180],
            (multiplicity) * (p.p148),
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);let nv7 = ctx.node_voltage(nodes[7]);let eq15_e246: f64 = (s.v[42] + s.v[199]);let eq15_e246_d_n0: f64 = (s.dn[42][0] + s.dn[199][0]);let eq15_e246_d_n1: f64 = (s.dn[42][1] + s.dn[199][1]);let eq15_e246_d_n2: f64 = (s.dn[42][2] + s.dn[199][2]);let eq15_e246_d_n3: f64 = (s.dn[42][3] + s.dn[199][3]);let eq15_e246_d_n4: f64 = (s.dn[42][4] + s.dn[199][4]);let eq15_e246_d_n5: f64 = (s.dn[42][5] + s.dn[199][5]);let eq15_e246_d_n6: f64 = (s.dn[42][6] + s.dn[199][6]);let eq15_e246_d_n7: f64 = (s.dn[42][7] + s.dn[199][7]);let eq15_e246_d_n8: f64 = (s.dn[42][8] + s.dn[199][8]);let eq15_e246_d_n9: f64 = (s.dn[42][9] + s.dn[199][9]);let eq15_e246_d_n10: f64 = (s.dn[42][10] + s.dn[199][10]);let eq15_e246_d_n11: f64 = (s.dn[42][11] + s.dn[199][11]);let eq15_e246_d_n12: f64 = (s.dn[42][12] + s.dn[199][12]);let eq15_e246_d_n13: f64 = (s.dn[42][13] + s.dn[199][13]);let eq15_e246_d_n14: f64 = (s.dn[42][14] + s.dn[199][14]);let eq15_e246_d_b0: f64 = (s.db[42][0] + s.db[199][0]);let eq15_e246_d_b1: f64 = (s.db[42][1] + s.db[199][1]);let eq15_e246_d_b2: f64 = (s.db[42][2] + s.db[199][2]);let eq15_e246_d_b3: f64 = (s.db[42][3] + s.db[199][3]);let eq15_e246_d_b4: f64 = (s.db[42][4] + s.db[199][4]);let eq15_e246_d_b5: f64 = (s.db[42][5] + s.db[199][5]);let eq15_e247: f64 = (p.p148 * eq15_e246);let eq15_e247_d_n0: f64 = (p.p148 * eq15_e246_d_n0);let eq15_e247_d_n1: f64 = (p.p148 * eq15_e246_d_n1);let eq15_e247_d_n2: f64 = (p.p148 * eq15_e246_d_n2);let eq15_e247_d_n3: f64 = (p.p148 * eq15_e246_d_n3);let eq15_e247_d_n4: f64 = (p.p148 * eq15_e246_d_n4);let eq15_e247_d_n5: f64 = (p.p148 * eq15_e246_d_n5);let eq15_e247_d_n6: f64 = (p.p148 * eq15_e246_d_n6);let eq15_e247_d_n7: f64 = (p.p148 * eq15_e246_d_n7);let eq15_e247_d_n8: f64 = (p.p148 * eq15_e246_d_n8);let eq15_e247_d_n9: f64 = (p.p148 * eq15_e246_d_n9);let eq15_e247_d_n10: f64 = (p.p148 * eq15_e246_d_n10);let eq15_e247_d_n11: f64 = (p.p148 * eq15_e246_d_n11);let eq15_e247_d_n12: f64 = (p.p148 * eq15_e246_d_n12);let eq15_e247_d_n13: f64 = (p.p148 * eq15_e246_d_n13);let eq15_e247_d_n14: f64 = (p.p148 * eq15_e246_d_n14);let eq15_e247_d_b0: f64 = (p.p148 * eq15_e246_d_b0);let eq15_e247_d_b1: f64 = (p.p148 * eq15_e246_d_b1);let eq15_e247_d_b2: f64 = (p.p148 * eq15_e246_d_b2);let eq15_e247_d_b3: f64 = (p.p148 * eq15_e246_d_b3);let eq15_e247_d_b4: f64 = (p.p148 * eq15_e246_d_b4);let eq15_e247_d_b5: f64 = (p.p148 * eq15_e246_d_b5);let eq15_e248_q: f64 = eq15_e247;let eq15_reactive_node_derivatives: [f64; 15] = [eq15_e247_d_n0, eq15_e247_d_n1, eq15_e247_d_n2, eq15_e247_d_n3, eq15_e247_d_n4, eq15_e247_d_n5, eq15_e247_d_n6, eq15_e247_d_n7, eq15_e247_d_n8, eq15_e247_d_n9, eq15_e247_d_n10, eq15_e247_d_n11, eq15_e247_d_n12, eq15_e247_d_n13, eq15_e247_d_n14];let eq15_reactive_branch_derivatives: [f64; 6] = [eq15_e247_d_b0, eq15_e247_d_b1, eq15_e247_d_b2, eq15_e247_d_b3, eq15_e247_d_b4, eq15_e247_d_b5];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(5),
            &eq15_reactive_node_derivatives,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );let eq16_e251: f64 = (s.v[172] * (nv7 - nv5));let eq16_e251_d_n0: f64 = (s.dn[172][0] * (nv7 - nv5));let eq16_e251_d_n1: f64 = (s.dn[172][1] * (nv7 - nv5));let eq16_e251_d_n2: f64 = (s.dn[172][2] * (nv7 - nv5));let eq16_e251_d_n3: f64 = (s.dn[172][3] * (nv7 - nv5));let eq16_e251_d_n4: f64 = (s.dn[172][4] * (nv7 - nv5));let eq16_e251_d_n5: f64 = ((s.dn[172][5] * (nv7 - nv5)) + (-s.v[172]));let eq16_e251_d_n6: f64 = (s.dn[172][6] * (nv7 - nv5));let eq16_e251_d_n7: f64 = ((s.dn[172][7] * (nv7 - nv5)) + s.v[172]);let eq16_e251_d_n8: f64 = (s.dn[172][8] * (nv7 - nv5));let eq16_e251_d_n9: f64 = (s.dn[172][9] * (nv7 - nv5));let eq16_e251_d_n10: f64 = (s.dn[172][10] * (nv7 - nv5));let eq16_e251_d_n11: f64 = (s.dn[172][11] * (nv7 - nv5));let eq16_e251_d_n12: f64 = (s.dn[172][12] * (nv7 - nv5));let eq16_e251_d_n13: f64 = (s.dn[172][13] * (nv7 - nv5));let eq16_e251_d_n14: f64 = (s.dn[172][14] * (nv7 - nv5));let eq16_e251_d_b0: f64 = (s.db[172][0] * (nv7 - nv5));let eq16_e251_d_b1: f64 = (s.db[172][1] * (nv7 - nv5));let eq16_e251_d_b2: f64 = (s.db[172][2] * (nv7 - nv5));let eq16_e251_d_b3: f64 = (s.db[172][3] * (nv7 - nv5));let eq16_e251_d_b4: f64 = (s.db[172][4] * (nv7 - nv5));let eq16_e251_d_b5: f64 = (s.db[172][5] * (nv7 - nv5));let eq16_e252_q: f64 = eq16_e251;let eq16_reactive_node_derivatives: [f64; 15] = [eq16_e251_d_n0, eq16_e251_d_n1, eq16_e251_d_n2, eq16_e251_d_n3, eq16_e251_d_n4, eq16_e251_d_n5, eq16_e251_d_n6, eq16_e251_d_n7, eq16_e251_d_n8, eq16_e251_d_n9, eq16_e251_d_n10, eq16_e251_d_n11, eq16_e251_d_n12, eq16_e251_d_n13, eq16_e251_d_n14];let eq16_reactive_branch_derivatives: [f64; 6] = [eq16_e251_d_b0, eq16_e251_d_b1, eq16_e251_d_b2, eq16_e251_d_b3, eq16_e251_d_b4, eq16_e251_d_b5];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(5),
            &eq16_reactive_node_derivatives,
            &eq16_reactive_branch_derivatives,
            multiplicity,
        );let eq17_e255: f64 = (p.p148 * s.v[41]);let eq17_e256_q: f64 = eq17_e255;
        stamper.stamp_current_reactive_dense_local(
            Some(1),
            Some(5),
            &s.dn[41],
            &s.db[41],
            (multiplicity) * (p.p148),
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
        let nv0 = ctx.node_voltage(nodes[0]);let nv1 = ctx.node_voltage(nodes[1]);let nv2 = ctx.node_voltage(nodes[2]);let nv3 = ctx.node_voltage(nodes[3]);let nv4 = ctx.node_voltage(nodes[4]);let nv5 = ctx.node_voltage(nodes[5]);let nv7 = ctx.node_voltage(nodes[7]);let nv9 = ctx.node_voltage(nodes[9]);let eq18_e259: f64 = (s.v[171] * (nv1 - nv5));let eq18_e259_d_n0: f64 = (s.dn[171][0] * (nv1 - nv5));let eq18_e259_d_n1: f64 = ((s.dn[171][1] * (nv1 - nv5)) + s.v[171]);let eq18_e259_d_n2: f64 = (s.dn[171][2] * (nv1 - nv5));let eq18_e259_d_n3: f64 = (s.dn[171][3] * (nv1 - nv5));let eq18_e259_d_n4: f64 = (s.dn[171][4] * (nv1 - nv5));let eq18_e259_d_n5: f64 = ((s.dn[171][5] * (nv1 - nv5)) + (-s.v[171]));let eq18_e259_d_n6: f64 = (s.dn[171][6] * (nv1 - nv5));let eq18_e259_d_n7: f64 = (s.dn[171][7] * (nv1 - nv5));let eq18_e259_d_n8: f64 = (s.dn[171][8] * (nv1 - nv5));let eq18_e259_d_n9: f64 = (s.dn[171][9] * (nv1 - nv5));let eq18_e259_d_n10: f64 = (s.dn[171][10] * (nv1 - nv5));let eq18_e259_d_n11: f64 = (s.dn[171][11] * (nv1 - nv5));let eq18_e259_d_n12: f64 = (s.dn[171][12] * (nv1 - nv5));let eq18_e259_d_n13: f64 = (s.dn[171][13] * (nv1 - nv5));let eq18_e259_d_n14: f64 = (s.dn[171][14] * (nv1 - nv5));let eq18_e259_d_b0: f64 = (s.db[171][0] * (nv1 - nv5));let eq18_e259_d_b1: f64 = (s.db[171][1] * (nv1 - nv5));let eq18_e259_d_b2: f64 = (s.db[171][2] * (nv1 - nv5));let eq18_e259_d_b3: f64 = (s.db[171][3] * (nv1 - nv5));let eq18_e259_d_b4: f64 = (s.db[171][4] * (nv1 - nv5));let eq18_e259_d_b5: f64 = (s.db[171][5] * (nv1 - nv5));let eq18_e260_q: f64 = eq18_e259;let eq18_reactive_node_derivatives: [f64; 15] = [eq18_e259_d_n0, eq18_e259_d_n1, eq18_e259_d_n2, eq18_e259_d_n3, eq18_e259_d_n4, eq18_e259_d_n5, eq18_e259_d_n6, eq18_e259_d_n7, eq18_e259_d_n8, eq18_e259_d_n9, eq18_e259_d_n10, eq18_e259_d_n11, eq18_e259_d_n12, eq18_e259_d_n13, eq18_e259_d_n14];let eq18_reactive_branch_derivatives: [f64; 6] = [eq18_e259_d_b0, eq18_e259_d_b1, eq18_e259_d_b2, eq18_e259_d_b3, eq18_e259_d_b4, eq18_e259_d_b5];
        stamper.stamp_current_reactive_dense_local(
            Some(1),
            Some(5),
            &eq18_reactive_node_derivatives,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );let eq25_e296: f64 = (s.v[174] * (nv7 - nv2));let eq25_e297_q: f64 = eq25_e296;
        stamper.stamp_current_reactive_node2_local(
            Some(7),
            Some(2),
            2,
            multiplicity * ((-s.v[174])),
            7,
            multiplicity * (s.v[174]),
        );let eq26_e300: f64 = (s.v[173] * (nv1 - nv2));let eq26_e301_q: f64 = eq26_e300;
        stamper.stamp_current_reactive_node2_local(
            Some(1),
            Some(2),
            1,
            multiplicity * (s.v[173]),
            2,
            multiplicity * ((-s.v[173])),
        );let eq27_e304: f64 = (p.p108 * (nv0 - nv2));let eq27_e305_q: f64 = eq27_e304;
        stamper.stamp_current_reactive_node2_local(
            Some(0),
            Some(2),
            0,
            multiplicity * (p.p108),
            2,
            multiplicity * ((-p.p108)),
        );let eq33_e343: f64 = (p.p148 * s.v[196]);let eq33_e344_q: f64 = eq33_e343;
        stamper.stamp_current_reactive_dense_local(
            Some(9),
            Some(5),
            &s.dn[196],
            &s.db[196],
            (multiplicity) * (p.p148),
        );let eq34_e347: f64 = (p.p148 * s.v[197]);let eq34_e348_q: f64 = eq34_e347;
        stamper.stamp_current_reactive_dense_local(
            Some(3),
            Some(0),
            &s.dn[197],
            &s.db[197],
            (multiplicity) * (p.p148),
        );
        let (eq36_e363, eq36_e363_d_n3, eq36_e363_d_n9, eq36_e363_q,) = {
    if (s.b[517] && s.b[518]) {
        let eq36_e360: f64 = (p.p103 * (nv9 - nv3));let eq36_e361_q: f64 = eq36_e360;
        (eq36_e360, (-p.p103), p.p103, eq36_e361_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node2_local(
            Some(9),
            Some(3),
            3,
            multiplicity * (eq36_e363_d_n3),
            9,
            multiplicity * (eq36_e363_d_n9),
        );
        let (eq39_e385, eq39_e385_d_n4, eq39_e385_q,) = {
    if (s.b[519] && s.b[520]) {
        let eq39_e382: f64 = (p.p145 * (nv4 - 0.0));let eq39_e383_q: f64 = eq39_e382;
        (eq39_e382, p.p145, eq39_e383_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(4),
            None,
            4,
            multiplicity * (eq39_e385_d_n4),
        );let eq42_e393_q: f64 = s.v[239];
        stamper.stamp_current_reactive_dense_local(
            Some(10),
            None,
            &s.dn[239],
            &s.db[239],
            multiplicity,
        );let eq44_e396_q: f64 = s.v[240];
        stamper.stamp_current_reactive_dense_local(
            Some(11),
            None,
            &s.dn[240],
            &s.db[240],
            multiplicity,
        );let eq46_e399_q: f64 = s.v[236];
        stamper.stamp_current_reactive_dense_local(
            Some(12),
            None,
            &s.dn[236],
            &s.db[236],
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq65_e534, eq65_e534_d_n0, eq65_e534_d_n1, eq65_e534_d_n2, eq65_e534_d_n3, eq65_e534_d_n4, eq65_e534_d_n5, eq65_e534_d_n6, eq65_e534_d_n7, eq65_e534_d_n8, eq65_e534_d_n9, eq65_e534_d_n10, eq65_e534_d_n11, eq65_e534_d_n12, eq65_e534_d_n13, eq65_e534_d_n14, eq65_e534_d_b0, eq65_e534_d_b1, eq65_e534_d_b2, eq65_e534_d_b3, eq65_e534_d_b4, eq65_e534_d_b5, eq65_e534_q, eq65_e534_q_d_n0, eq65_e534_q_d_n1, eq65_e534_q_d_n2, eq65_e534_q_d_n3, eq65_e534_q_d_n4, eq65_e534_q_d_n5, eq65_e534_q_d_n6, eq65_e534_q_d_n7, eq65_e534_q_d_n8, eq65_e534_q_d_n9, eq65_e534_q_d_n10, eq65_e534_q_d_n11, eq65_e534_q_d_n12, eq65_e534_q_d_n13, eq65_e534_q_d_n14, eq65_e534_q_d_b0, eq65_e534_q_d_b1, eq65_e534_q_d_b2, eq65_e534_q_d_b3, eq65_e534_q_d_b4, eq65_e534_q_d_b5,) = {
    if s.b[533] {
        let eq65_e527: f64 = (s.v[537] / s.v[535]);let __rspice_inv_cse_0: f64 = 1.0 / (s.v[535] * s.v[535]);let eq65_e527_d_n0: f64 = (((s.dn[537][0] * s.v[535]) - (s.v[537] * s.dn[535][0])) * __rspice_inv_cse_0);let eq65_e527_d_n1: f64 = (((s.dn[537][1] * s.v[535]) - (s.v[537] * s.dn[535][1])) * __rspice_inv_cse_0);let eq65_e527_d_n2: f64 = (((s.dn[537][2] * s.v[535]) - (s.v[537] * s.dn[535][2])) * __rspice_inv_cse_0);let eq65_e527_d_n3: f64 = (((s.dn[537][3] * s.v[535]) - (s.v[537] * s.dn[535][3])) * __rspice_inv_cse_0);let eq65_e527_d_n4: f64 = (((s.dn[537][4] * s.v[535]) - (s.v[537] * s.dn[535][4])) * __rspice_inv_cse_0);let eq65_e527_d_n5: f64 = (((s.dn[537][5] * s.v[535]) - (s.v[537] * s.dn[535][5])) * __rspice_inv_cse_0);let eq65_e527_d_n6: f64 = (((s.dn[537][6] * s.v[535]) - (s.v[537] * s.dn[535][6])) * __rspice_inv_cse_0);let eq65_e527_d_n7: f64 = (((s.dn[537][7] * s.v[535]) - (s.v[537] * s.dn[535][7])) * __rspice_inv_cse_0);let eq65_e527_d_n8: f64 = (((s.dn[537][8] * s.v[535]) - (s.v[537] * s.dn[535][8])) * __rspice_inv_cse_0);let eq65_e527_d_n9: f64 = (((s.dn[537][9] * s.v[535]) - (s.v[537] * s.dn[535][9])) * __rspice_inv_cse_0);let eq65_e527_d_n10: f64 = (((s.dn[537][10] * s.v[535]) - (s.v[537] * s.dn[535][10])) * __rspice_inv_cse_0);let eq65_e527_d_n11: f64 = (((s.dn[537][11] * s.v[535]) - (s.v[537] * s.dn[535][11])) * __rspice_inv_cse_0);let eq65_e527_d_n12: f64 = (((s.dn[537][12] * s.v[535]) - (s.v[537] * s.dn[535][12])) * __rspice_inv_cse_0);let eq65_e527_d_n13: f64 = (((s.dn[537][13] * s.v[535]) - (s.v[537] * s.dn[535][13])) * __rspice_inv_cse_0);let eq65_e527_d_n14: f64 = (((s.dn[537][14] * s.v[535]) - (s.v[537] * s.dn[535][14])) * __rspice_inv_cse_0);let eq65_e527_d_b0: f64 = (((s.db[537][0] * s.v[535]) - (s.v[537] * s.db[535][0])) * __rspice_inv_cse_0);let eq65_e527_d_b1: f64 = (((s.db[537][1] * s.v[535]) - (s.v[537] * s.db[535][1])) * __rspice_inv_cse_0);let eq65_e527_d_b2: f64 = (((s.db[537][2] * s.v[535]) - (s.v[537] * s.db[535][2])) * __rspice_inv_cse_0);let eq65_e527_d_b3: f64 = (((s.db[537][3] * s.v[535]) - (s.v[537] * s.db[535][3])) * __rspice_inv_cse_0);let eq65_e527_d_b4: f64 = (((s.db[537][4] * s.v[535]) - (s.v[537] * s.db[535][4])) * __rspice_inv_cse_0);let eq65_e527_d_b5: f64 = (((s.db[537][5] * s.v[535]) - (s.v[537] * s.db[535][5])) * __rspice_inv_cse_0);let eq65_e530: f64 = (s.v[535] * (nv13 - 0.0));let eq65_e530_d_n0: f64 = (s.dn[535][0] * (nv13 - 0.0));let eq65_e530_d_n1: f64 = (s.dn[535][1] * (nv13 - 0.0));let eq65_e530_d_n2: f64 = (s.dn[535][2] * (nv13 - 0.0));let eq65_e530_d_n3: f64 = (s.dn[535][3] * (nv13 - 0.0));let eq65_e530_d_n4: f64 = (s.dn[535][4] * (nv13 - 0.0));let eq65_e530_d_n5: f64 = (s.dn[535][5] * (nv13 - 0.0));let eq65_e530_d_n6: f64 = (s.dn[535][6] * (nv13 - 0.0));let eq65_e530_d_n7: f64 = (s.dn[535][7] * (nv13 - 0.0));let eq65_e530_d_n8: f64 = (s.dn[535][8] * (nv13 - 0.0));let eq65_e530_d_n9: f64 = (s.dn[535][9] * (nv13 - 0.0));let eq65_e530_d_n10: f64 = (s.dn[535][10] * (nv13 - 0.0));let eq65_e530_d_n11: f64 = (s.dn[535][11] * (nv13 - 0.0));let eq65_e530_d_n12: f64 = (s.dn[535][12] * (nv13 - 0.0));let eq65_e530_d_n13: f64 = ((s.dn[535][13] * (nv13 - 0.0)) + s.v[535]);let eq65_e530_d_n14: f64 = (s.dn[535][14] * (nv13 - 0.0));let eq65_e530_d_b0: f64 = (s.db[535][0] * (nv13 - 0.0));let eq65_e530_d_b1: f64 = (s.db[535][1] * (nv13 - 0.0));let eq65_e530_d_b2: f64 = (s.db[535][2] * (nv13 - 0.0));let eq65_e530_d_b3: f64 = (s.db[535][3] * (nv13 - 0.0));let eq65_e530_d_b4: f64 = (s.db[535][4] * (nv13 - 0.0));let eq65_e530_d_b5: f64 = (s.db[535][5] * (nv13 - 0.0));let eq65_e531_q: f64 = eq65_e530;let eq65_e532: f64 = (eq65_e527 * eq65_e530);let eq65_e532_d_n0: f64 = ((eq65_e527_d_n0 * eq65_e530) + (eq65_e527 * eq65_e530_d_n0));let eq65_e532_d_n1: f64 = ((eq65_e527_d_n1 * eq65_e530) + (eq65_e527 * eq65_e530_d_n1));let eq65_e532_d_n2: f64 = ((eq65_e527_d_n2 * eq65_e530) + (eq65_e527 * eq65_e530_d_n2));let eq65_e532_d_n3: f64 = ((eq65_e527_d_n3 * eq65_e530) + (eq65_e527 * eq65_e530_d_n3));let eq65_e532_d_n4: f64 = ((eq65_e527_d_n4 * eq65_e530) + (eq65_e527 * eq65_e530_d_n4));
        let eq65_e532_d_n5: f64 = ((eq65_e527_d_n5 * eq65_e530) + (eq65_e527 * eq65_e530_d_n5));let eq65_e532_d_n6: f64 = ((eq65_e527_d_n6 * eq65_e530) + (eq65_e527 * eq65_e530_d_n6));let eq65_e532_d_n7: f64 = ((eq65_e527_d_n7 * eq65_e530) + (eq65_e527 * eq65_e530_d_n7));let eq65_e532_d_n8: f64 = ((eq65_e527_d_n8 * eq65_e530) + (eq65_e527 * eq65_e530_d_n8));let eq65_e532_d_n9: f64 = ((eq65_e527_d_n9 * eq65_e530) + (eq65_e527 * eq65_e530_d_n9));let eq65_e532_d_n10: f64 = ((eq65_e527_d_n10 * eq65_e530) + (eq65_e527 * eq65_e530_d_n10));let eq65_e532_d_n11: f64 = ((eq65_e527_d_n11 * eq65_e530) + (eq65_e527 * eq65_e530_d_n11));let eq65_e532_d_n12: f64 = ((eq65_e527_d_n12 * eq65_e530) + (eq65_e527 * eq65_e530_d_n12));let eq65_e532_d_n13: f64 = ((eq65_e527_d_n13 * eq65_e530) + (eq65_e527 * eq65_e530_d_n13));let eq65_e532_d_n14: f64 = ((eq65_e527_d_n14 * eq65_e530) + (eq65_e527 * eq65_e530_d_n14));let eq65_e532_d_b0: f64 = ((eq65_e527_d_b0 * eq65_e530) + (eq65_e527 * eq65_e530_d_b0));let eq65_e532_d_b1: f64 = ((eq65_e527_d_b1 * eq65_e530) + (eq65_e527 * eq65_e530_d_b1));let eq65_e532_d_b2: f64 = ((eq65_e527_d_b2 * eq65_e530) + (eq65_e527 * eq65_e530_d_b2));let eq65_e532_d_b3: f64 = ((eq65_e527_d_b3 * eq65_e530) + (eq65_e527 * eq65_e530_d_b3));let eq65_e532_d_b4: f64 = ((eq65_e527_d_b4 * eq65_e530) + (eq65_e527 * eq65_e530_d_b4));let eq65_e532_d_b5: f64 = ((eq65_e527_d_b5 * eq65_e530) + (eq65_e527 * eq65_e530_d_b5));let eq65_e532_q: f64 = (eq65_e527 * eq65_e531_q);let eq65_e532_q_d_n0: f64 = ((eq65_e527_d_n0 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n0));let eq65_e532_q_d_n1: f64 = ((eq65_e527_d_n1 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n1));let eq65_e532_q_d_n2: f64 = ((eq65_e527_d_n2 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n2));let eq65_e532_q_d_n3: f64 = ((eq65_e527_d_n3 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n3));let eq65_e532_q_d_n4: f64 = ((eq65_e527_d_n4 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n4));let eq65_e532_q_d_n5: f64 = ((eq65_e527_d_n5 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n5));let eq65_e532_q_d_n6: f64 = ((eq65_e527_d_n6 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n6));let eq65_e532_q_d_n7: f64 = ((eq65_e527_d_n7 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n7));let eq65_e532_q_d_n8: f64 = ((eq65_e527_d_n8 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n8));let eq65_e532_q_d_n9: f64 = ((eq65_e527_d_n9 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n9));let eq65_e532_q_d_n10: f64 = ((eq65_e527_d_n10 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n10));let eq65_e532_q_d_n11: f64 = ((eq65_e527_d_n11 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n11));let eq65_e532_q_d_n12: f64 = ((eq65_e527_d_n12 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n12));let eq65_e532_q_d_n13: f64 = ((eq65_e527_d_n13 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n13));let eq65_e532_q_d_n14: f64 = ((eq65_e527_d_n14 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n14));let eq65_e532_q_d_b0: f64 = ((eq65_e527_d_b0 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_b0));let eq65_e532_q_d_b1: f64 = ((eq65_e527_d_b1 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_b1));let eq65_e532_q_d_b2: f64 = ((eq65_e527_d_b2 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_b2));let eq65_e532_q_d_b3: f64 = ((eq65_e527_d_b3 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_b3));let eq65_e532_q_d_b4: f64 = ((eq65_e527_d_b4 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_b4));let eq65_e532_q_d_b5: f64 = ((eq65_e527_d_b5 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_b5));
        (eq65_e532, eq65_e532_d_n0, eq65_e532_d_n1, eq65_e532_d_n2, eq65_e532_d_n3, eq65_e532_d_n4, eq65_e532_d_n5, eq65_e532_d_n6, eq65_e532_d_n7, eq65_e532_d_n8, eq65_e532_d_n9, eq65_e532_d_n10, eq65_e532_d_n11, eq65_e532_d_n12, eq65_e532_d_n13, eq65_e532_d_n14, eq65_e532_d_b0, eq65_e532_d_b1, eq65_e532_d_b2, eq65_e532_d_b3, eq65_e532_d_b4, eq65_e532_d_b5, eq65_e532_q, eq65_e532_q_d_n0, eq65_e532_q_d_n1, eq65_e532_q_d_n2, eq65_e532_q_d_n3, eq65_e532_q_d_n4, eq65_e532_q_d_n5, eq65_e532_q_d_n6, eq65_e532_q_d_n7, eq65_e532_q_d_n8, eq65_e532_q_d_n9, eq65_e532_q_d_n10, eq65_e532_q_d_n11, eq65_e532_q_d_n12, eq65_e532_q_d_n13, eq65_e532_q_d_n14, eq65_e532_q_d_b0, eq65_e532_q_d_b1, eq65_e532_q_d_b2, eq65_e532_q_d_b3, eq65_e532_q_d_b4, eq65_e532_q_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_reactive_node_derivatives: [f64; 15] = [eq65_e534_q_d_n0, eq65_e534_q_d_n1, eq65_e534_q_d_n2, eq65_e534_q_d_n3, eq65_e534_q_d_n4, eq65_e534_q_d_n5, eq65_e534_q_d_n6, eq65_e534_q_d_n7, eq65_e534_q_d_n8, eq65_e534_q_d_n9, eq65_e534_q_d_n10, eq65_e534_q_d_n11, eq65_e534_q_d_n12, eq65_e534_q_d_n13, eq65_e534_q_d_n14];let eq65_reactive_branch_derivatives: [f64; 6] = [eq65_e534_q_d_b0, eq65_e534_q_d_b1, eq65_e534_q_d_b2, eq65_e534_q_d_b3, eq65_e534_q_d_b4, eq65_e534_q_d_b5];
        stamper.stamp_current_reactive_dense_local(
            Some(8),
            Some(6),
            &eq65_reactive_node_derivatives,
            &eq65_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_5(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq66_e545, eq66_e545_d_n0, eq66_e545_d_n1, eq66_e545_d_n2, eq66_e545_d_n3, eq66_e545_d_n4, eq66_e545_d_n5, eq66_e545_d_n6, eq66_e545_d_n7, eq66_e545_d_n8, eq66_e545_d_n9, eq66_e545_d_n10, eq66_e545_d_n11, eq66_e545_d_n12, eq66_e545_d_n13, eq66_e545_d_n14, eq66_e545_d_b0, eq66_e545_d_b1, eq66_e545_d_b2, eq66_e545_d_b3, eq66_e545_d_b4, eq66_e545_d_b5, eq66_e545_q, eq66_e545_q_d_n0, eq66_e545_q_d_n1, eq66_e545_q_d_n2, eq66_e545_q_d_n3, eq66_e545_q_d_n4, eq66_e545_q_d_n5, eq66_e545_q_d_n6, eq66_e545_q_d_n7, eq66_e545_q_d_n8, eq66_e545_q_d_n9, eq66_e545_q_d_n10, eq66_e545_q_d_n11, eq66_e545_q_d_n12, eq66_e545_q_d_n13, eq66_e545_q_d_n14, eq66_e545_q_d_b0, eq66_e545_q_d_b1, eq66_e545_q_d_b2, eq66_e545_q_d_b3, eq66_e545_q_d_b4, eq66_e545_q_d_b5,) = {
    if s.b[533] {
        let eq66_e538: f64 = (s.v[536] / s.v[535]);let __rspice_inv_cse_0: f64 = 1.0 / (s.v[535] * s.v[535]);let eq66_e538_d_n0: f64 = (((s.dn[536][0] * s.v[535]) - (s.v[536] * s.dn[535][0])) * __rspice_inv_cse_0);let eq66_e538_d_n1: f64 = (((s.dn[536][1] * s.v[535]) - (s.v[536] * s.dn[535][1])) * __rspice_inv_cse_0);let eq66_e538_d_n2: f64 = (((s.dn[536][2] * s.v[535]) - (s.v[536] * s.dn[535][2])) * __rspice_inv_cse_0);let eq66_e538_d_n3: f64 = (((s.dn[536][3] * s.v[535]) - (s.v[536] * s.dn[535][3])) * __rspice_inv_cse_0);let eq66_e538_d_n4: f64 = (((s.dn[536][4] * s.v[535]) - (s.v[536] * s.dn[535][4])) * __rspice_inv_cse_0);let eq66_e538_d_n5: f64 = (((s.dn[536][5] * s.v[535]) - (s.v[536] * s.dn[535][5])) * __rspice_inv_cse_0);let eq66_e538_d_n6: f64 = (((s.dn[536][6] * s.v[535]) - (s.v[536] * s.dn[535][6])) * __rspice_inv_cse_0);let eq66_e538_d_n7: f64 = (((s.dn[536][7] * s.v[535]) - (s.v[536] * s.dn[535][7])) * __rspice_inv_cse_0);let eq66_e538_d_n8: f64 = (((s.dn[536][8] * s.v[535]) - (s.v[536] * s.dn[535][8])) * __rspice_inv_cse_0);let eq66_e538_d_n9: f64 = (((s.dn[536][9] * s.v[535]) - (s.v[536] * s.dn[535][9])) * __rspice_inv_cse_0);let eq66_e538_d_n10: f64 = (((s.dn[536][10] * s.v[535]) - (s.v[536] * s.dn[535][10])) * __rspice_inv_cse_0);let eq66_e538_d_n11: f64 = (((s.dn[536][11] * s.v[535]) - (s.v[536] * s.dn[535][11])) * __rspice_inv_cse_0);let eq66_e538_d_n12: f64 = (((s.dn[536][12] * s.v[535]) - (s.v[536] * s.dn[535][12])) * __rspice_inv_cse_0);let eq66_e538_d_n13: f64 = (((s.dn[536][13] * s.v[535]) - (s.v[536] * s.dn[535][13])) * __rspice_inv_cse_0);let eq66_e538_d_n14: f64 = (((s.dn[536][14] * s.v[535]) - (s.v[536] * s.dn[535][14])) * __rspice_inv_cse_0);let eq66_e538_d_b0: f64 = (((s.db[536][0] * s.v[535]) - (s.v[536] * s.db[535][0])) * __rspice_inv_cse_0);let eq66_e538_d_b1: f64 = (((s.db[536][1] * s.v[535]) - (s.v[536] * s.db[535][1])) * __rspice_inv_cse_0);let eq66_e538_d_b2: f64 = (((s.db[536][2] * s.v[535]) - (s.v[536] * s.db[535][2])) * __rspice_inv_cse_0);let eq66_e538_d_b3: f64 = (((s.db[536][3] * s.v[535]) - (s.v[536] * s.db[535][3])) * __rspice_inv_cse_0);let eq66_e538_d_b4: f64 = (((s.db[536][4] * s.v[535]) - (s.v[536] * s.db[535][4])) * __rspice_inv_cse_0);let eq66_e538_d_b5: f64 = (((s.db[536][5] * s.v[535]) - (s.v[536] * s.db[535][5])) * __rspice_inv_cse_0);let eq66_e541: f64 = (s.v[535] * (nv14 - 0.0));let eq66_e541_d_n0: f64 = (s.dn[535][0] * (nv14 - 0.0));let eq66_e541_d_n1: f64 = (s.dn[535][1] * (nv14 - 0.0));let eq66_e541_d_n2: f64 = (s.dn[535][2] * (nv14 - 0.0));let eq66_e541_d_n3: f64 = (s.dn[535][3] * (nv14 - 0.0));let eq66_e541_d_n4: f64 = (s.dn[535][4] * (nv14 - 0.0));let eq66_e541_d_n5: f64 = (s.dn[535][5] * (nv14 - 0.0));let eq66_e541_d_n6: f64 = (s.dn[535][6] * (nv14 - 0.0));let eq66_e541_d_n7: f64 = (s.dn[535][7] * (nv14 - 0.0));let eq66_e541_d_n8: f64 = (s.dn[535][8] * (nv14 - 0.0));let eq66_e541_d_n9: f64 = (s.dn[535][9] * (nv14 - 0.0));let eq66_e541_d_n10: f64 = (s.dn[535][10] * (nv14 - 0.0));let eq66_e541_d_n11: f64 = (s.dn[535][11] * (nv14 - 0.0));let eq66_e541_d_n12: f64 = (s.dn[535][12] * (nv14 - 0.0));let eq66_e541_d_n13: f64 = (s.dn[535][13] * (nv14 - 0.0));let eq66_e541_d_n14: f64 = ((s.dn[535][14] * (nv14 - 0.0)) + s.v[535]);let eq66_e541_d_b0: f64 = (s.db[535][0] * (nv14 - 0.0));let eq66_e541_d_b1: f64 = (s.db[535][1] * (nv14 - 0.0));let eq66_e541_d_b2: f64 = (s.db[535][2] * (nv14 - 0.0));let eq66_e541_d_b3: f64 = (s.db[535][3] * (nv14 - 0.0));let eq66_e541_d_b4: f64 = (s.db[535][4] * (nv14 - 0.0));let eq66_e541_d_b5: f64 = (s.db[535][5] * (nv14 - 0.0));let eq66_e542_q: f64 = eq66_e541;let eq66_e543: f64 = (eq66_e538 * eq66_e541);let eq66_e543_d_n0: f64 = ((eq66_e538_d_n0 * eq66_e541) + (eq66_e538 * eq66_e541_d_n0));let eq66_e543_d_n1: f64 = ((eq66_e538_d_n1 * eq66_e541) + (eq66_e538 * eq66_e541_d_n1));let eq66_e543_d_n2: f64 = ((eq66_e538_d_n2 * eq66_e541) + (eq66_e538 * eq66_e541_d_n2));let eq66_e543_d_n3: f64 = ((eq66_e538_d_n3 * eq66_e541) + (eq66_e538 * eq66_e541_d_n3));let eq66_e543_d_n4: f64 = ((eq66_e538_d_n4 * eq66_e541) + (eq66_e538 * eq66_e541_d_n4));
        let eq66_e543_d_n5: f64 = ((eq66_e538_d_n5 * eq66_e541) + (eq66_e538 * eq66_e541_d_n5));let eq66_e543_d_n6: f64 = ((eq66_e538_d_n6 * eq66_e541) + (eq66_e538 * eq66_e541_d_n6));let eq66_e543_d_n7: f64 = ((eq66_e538_d_n7 * eq66_e541) + (eq66_e538 * eq66_e541_d_n7));let eq66_e543_d_n8: f64 = ((eq66_e538_d_n8 * eq66_e541) + (eq66_e538 * eq66_e541_d_n8));let eq66_e543_d_n9: f64 = ((eq66_e538_d_n9 * eq66_e541) + (eq66_e538 * eq66_e541_d_n9));let eq66_e543_d_n10: f64 = ((eq66_e538_d_n10 * eq66_e541) + (eq66_e538 * eq66_e541_d_n10));let eq66_e543_d_n11: f64 = ((eq66_e538_d_n11 * eq66_e541) + (eq66_e538 * eq66_e541_d_n11));let eq66_e543_d_n12: f64 = ((eq66_e538_d_n12 * eq66_e541) + (eq66_e538 * eq66_e541_d_n12));let eq66_e543_d_n13: f64 = ((eq66_e538_d_n13 * eq66_e541) + (eq66_e538 * eq66_e541_d_n13));let eq66_e543_d_n14: f64 = ((eq66_e538_d_n14 * eq66_e541) + (eq66_e538 * eq66_e541_d_n14));let eq66_e543_d_b0: f64 = ((eq66_e538_d_b0 * eq66_e541) + (eq66_e538 * eq66_e541_d_b0));let eq66_e543_d_b1: f64 = ((eq66_e538_d_b1 * eq66_e541) + (eq66_e538 * eq66_e541_d_b1));let eq66_e543_d_b2: f64 = ((eq66_e538_d_b2 * eq66_e541) + (eq66_e538 * eq66_e541_d_b2));let eq66_e543_d_b3: f64 = ((eq66_e538_d_b3 * eq66_e541) + (eq66_e538 * eq66_e541_d_b3));let eq66_e543_d_b4: f64 = ((eq66_e538_d_b4 * eq66_e541) + (eq66_e538 * eq66_e541_d_b4));let eq66_e543_d_b5: f64 = ((eq66_e538_d_b5 * eq66_e541) + (eq66_e538 * eq66_e541_d_b5));let eq66_e543_q: f64 = (eq66_e538 * eq66_e542_q);let eq66_e543_q_d_n0: f64 = ((eq66_e538_d_n0 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n0));let eq66_e543_q_d_n1: f64 = ((eq66_e538_d_n1 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n1));let eq66_e543_q_d_n2: f64 = ((eq66_e538_d_n2 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n2));let eq66_e543_q_d_n3: f64 = ((eq66_e538_d_n3 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n3));let eq66_e543_q_d_n4: f64 = ((eq66_e538_d_n4 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n4));let eq66_e543_q_d_n5: f64 = ((eq66_e538_d_n5 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n5));let eq66_e543_q_d_n6: f64 = ((eq66_e538_d_n6 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n6));let eq66_e543_q_d_n7: f64 = ((eq66_e538_d_n7 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n7));let eq66_e543_q_d_n8: f64 = ((eq66_e538_d_n8 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n8));let eq66_e543_q_d_n9: f64 = ((eq66_e538_d_n9 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n9));let eq66_e543_q_d_n10: f64 = ((eq66_e538_d_n10 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n10));let eq66_e543_q_d_n11: f64 = ((eq66_e538_d_n11 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n11));let eq66_e543_q_d_n12: f64 = ((eq66_e538_d_n12 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n12));let eq66_e543_q_d_n13: f64 = ((eq66_e538_d_n13 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n13));let eq66_e543_q_d_n14: f64 = ((eq66_e538_d_n14 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n14));let eq66_e543_q_d_b0: f64 = ((eq66_e538_d_b0 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_b0));let eq66_e543_q_d_b1: f64 = ((eq66_e538_d_b1 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_b1));let eq66_e543_q_d_b2: f64 = ((eq66_e538_d_b2 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_b2));let eq66_e543_q_d_b3: f64 = ((eq66_e538_d_b3 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_b3));let eq66_e543_q_d_b4: f64 = ((eq66_e538_d_b4 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_b4));let eq66_e543_q_d_b5: f64 = ((eq66_e538_d_b5 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_b5));
        (eq66_e543, eq66_e543_d_n0, eq66_e543_d_n1, eq66_e543_d_n2, eq66_e543_d_n3, eq66_e543_d_n4, eq66_e543_d_n5, eq66_e543_d_n6, eq66_e543_d_n7, eq66_e543_d_n8, eq66_e543_d_n9, eq66_e543_d_n10, eq66_e543_d_n11, eq66_e543_d_n12, eq66_e543_d_n13, eq66_e543_d_n14, eq66_e543_d_b0, eq66_e543_d_b1, eq66_e543_d_b2, eq66_e543_d_b3, eq66_e543_d_b4, eq66_e543_d_b5, eq66_e543_q, eq66_e543_q_d_n0, eq66_e543_q_d_n1, eq66_e543_q_d_n2, eq66_e543_q_d_n3, eq66_e543_q_d_n4, eq66_e543_q_d_n5, eq66_e543_q_d_n6, eq66_e543_q_d_n7, eq66_e543_q_d_n8, eq66_e543_q_d_n9, eq66_e543_q_d_n10, eq66_e543_q_d_n11, eq66_e543_q_d_n12, eq66_e543_q_d_n13, eq66_e543_q_d_n14, eq66_e543_q_d_b0, eq66_e543_q_d_b1, eq66_e543_q_d_b2, eq66_e543_q_d_b3, eq66_e543_q_d_b4, eq66_e543_q_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq66_reactive_node_derivatives: [f64; 15] = [eq66_e545_q_d_n0, eq66_e545_q_d_n1, eq66_e545_q_d_n2, eq66_e545_q_d_n3, eq66_e545_q_d_n4, eq66_e545_q_d_n5, eq66_e545_q_d_n6, eq66_e545_q_d_n7, eq66_e545_q_d_n8, eq66_e545_q_d_n9, eq66_e545_q_d_n10, eq66_e545_q_d_n11, eq66_e545_q_d_n12, eq66_e545_q_d_n13, eq66_e545_q_d_n14];let eq66_reactive_branch_derivatives: [f64; 6] = [eq66_e545_q_d_b0, eq66_e545_q_d_b1, eq66_e545_q_d_b2, eq66_e545_q_d_b3, eq66_e545_q_d_b4, eq66_e545_q_d_b5];
        stamper.stamp_current_reactive_dense_local(
            Some(8),
            Some(6),
            &eq66_reactive_node_derivatives,
            &eq66_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
