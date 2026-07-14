#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_6(
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
        let nv3 = ctx.node_voltage(nodes[3]);let nv10 = ctx.node_voltage(nodes[10]);
        let (eq38_e1338, eq38_e1338_d_n0, eq38_e1338_d_n1, eq38_e1338_d_n2, eq38_e1338_d_n3, eq38_e1338_d_n4, eq38_e1338_d_n5, eq38_e1338_d_n6, eq38_e1338_d_n7, eq38_e1338_d_n8, eq38_e1338_d_n9, eq38_e1338_d_n10, eq38_e1338_d_n11, eq38_e1338_d_n12, eq38_e1338_d_b0, eq38_e1338_d_b1, eq38_e1338_d_b2, eq38_e1338_d_b3, eq38_e1338_d_b4, eq38_e1338_d_b5, eq38_e1338_d_b6, eq38_e1338_d_b7, eq38_e1338_d_b8,) = {
    if s.b[1863] {
        let eq38_e1335: f64 = ((nv10 - nv3) * s.v[697]);let eq38_e1335_d_n0: f64 = ((nv10 - nv3) * s.dn[697][0]);let eq38_e1335_d_n1: f64 = ((nv10 - nv3) * s.dn[697][1]);let eq38_e1335_d_n2: f64 = ((nv10 - nv3) * s.dn[697][2]);let eq38_e1335_d_n3: f64 = ((-s.v[697]) + ((nv10 - nv3) * s.dn[697][3]));let eq38_e1335_d_n4: f64 = ((nv10 - nv3) * s.dn[697][4]);let eq38_e1335_d_n5: f64 = ((nv10 - nv3) * s.dn[697][5]);let eq38_e1335_d_n6: f64 = ((nv10 - nv3) * s.dn[697][6]);let eq38_e1335_d_n7: f64 = ((nv10 - nv3) * s.dn[697][7]);let eq38_e1335_d_n8: f64 = ((nv10 - nv3) * s.dn[697][8]);let eq38_e1335_d_n9: f64 = ((nv10 - nv3) * s.dn[697][9]);let eq38_e1335_d_n10: f64 = (s.v[697] + ((nv10 - nv3) * s.dn[697][10]));let eq38_e1335_d_n11: f64 = ((nv10 - nv3) * s.dn[697][11]);let eq38_e1335_d_n12: f64 = ((nv10 - nv3) * s.dn[697][12]);let eq38_e1335_d_b0: f64 = ((nv10 - nv3) * s.db[697][0]);let eq38_e1335_d_b1: f64 = ((nv10 - nv3) * s.db[697][1]);let eq38_e1335_d_b2: f64 = ((nv10 - nv3) * s.db[697][2]);let eq38_e1335_d_b3: f64 = ((nv10 - nv3) * s.db[697][3]);let eq38_e1335_d_b4: f64 = ((nv10 - nv3) * s.db[697][4]);let eq38_e1335_d_b5: f64 = ((nv10 - nv3) * s.db[697][5]);let eq38_e1335_d_b6: f64 = ((nv10 - nv3) * s.db[697][6]);let eq38_e1335_d_b7: f64 = ((nv10 - nv3) * s.db[697][7]);let eq38_e1335_d_b8: f64 = ((nv10 - nv3) * s.db[697][8]);let eq38_e1336: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq38_e1335);
        (eq38_e1336, (eq38_e1335_d_n0 * ddt_scale), (eq38_e1335_d_n1 * ddt_scale), (eq38_e1335_d_n2 * ddt_scale), (eq38_e1335_d_n3 * ddt_scale), (eq38_e1335_d_n4 * ddt_scale), (eq38_e1335_d_n5 * ddt_scale), (eq38_e1335_d_n6 * ddt_scale), (eq38_e1335_d_n7 * ddt_scale), (eq38_e1335_d_n8 * ddt_scale), (eq38_e1335_d_n9 * ddt_scale), (eq38_e1335_d_n10 * ddt_scale), (eq38_e1335_d_n11 * ddt_scale), (eq38_e1335_d_n12 * ddt_scale), (eq38_e1335_d_b0 * ddt_scale), (eq38_e1335_d_b1 * ddt_scale), (eq38_e1335_d_b2 * ddt_scale), (eq38_e1335_d_b3 * ddt_scale), (eq38_e1335_d_b4 * ddt_scale), (eq38_e1335_d_b5 * ddt_scale), (eq38_e1335_d_b6 * ddt_scale), (eq38_e1335_d_b7 * ddt_scale), (eq38_e1335_d_b8 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e1338;let eq38_node_derivatives: [f64; 13] = [eq38_e1338_d_n0, eq38_e1338_d_n1, eq38_e1338_d_n2, eq38_e1338_d_n3, eq38_e1338_d_n4, eq38_e1338_d_n5, eq38_e1338_d_n6, eq38_e1338_d_n7, eq38_e1338_d_n8, eq38_e1338_d_n9, eq38_e1338_d_n10, eq38_e1338_d_n11, eq38_e1338_d_n12];let eq38_branch_derivatives: [f64; 9] = [eq38_e1338_d_b0, eq38_e1338_d_b1, eq38_e1338_d_b2, eq38_e1338_d_b3, eq38_e1338_d_b4, eq38_e1338_d_b5, eq38_e1338_d_b6, eq38_e1338_d_b7, eq38_e1338_d_b8];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(3),
            multiplicity * (eq38_value),
            &eq38_node_derivatives,
            &eq38_branch_derivatives,
            multiplicity,
        );
        let (eq39_e1346, eq39_e1346_d_n0, eq39_e1346_d_n1, eq39_e1346_d_n2, eq39_e1346_d_n3, eq39_e1346_d_n4, eq39_e1346_d_n5, eq39_e1346_d_n6, eq39_e1346_d_n7, eq39_e1346_d_n8, eq39_e1346_d_n9, eq39_e1346_d_n10, eq39_e1346_d_n11, eq39_e1346_d_n12, eq39_e1346_d_b0, eq39_e1346_d_b1, eq39_e1346_d_b2, eq39_e1346_d_b3, eq39_e1346_d_b4, eq39_e1346_d_b5, eq39_e1346_d_b6, eq39_e1346_d_b7, eq39_e1346_d_b8,) = {
    if (!s.b[1863]) {
        let eq39_e1343: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, s.v[1230]);let eq39_e1344: f64 = (s.v[36] * eq39_e1343);let eq39_e1344_d_n0: f64 = (s.v[36] * (s.dn[1230][0] * ddt_scale));let eq39_e1344_d_n1: f64 = (s.v[36] * (s.dn[1230][1] * ddt_scale));let eq39_e1344_d_n2: f64 = (s.v[36] * (s.dn[1230][2] * ddt_scale));let eq39_e1344_d_n3: f64 = (s.v[36] * (s.dn[1230][3] * ddt_scale));let eq39_e1344_d_n4: f64 = (s.v[36] * (s.dn[1230][4] * ddt_scale));let eq39_e1344_d_n5: f64 = (s.v[36] * (s.dn[1230][5] * ddt_scale));let eq39_e1344_d_n6: f64 = (s.v[36] * (s.dn[1230][6] * ddt_scale));let eq39_e1344_d_n7: f64 = (s.v[36] * (s.dn[1230][7] * ddt_scale));let eq39_e1344_d_n8: f64 = (s.v[36] * (s.dn[1230][8] * ddt_scale));let eq39_e1344_d_n9: f64 = (s.v[36] * (s.dn[1230][9] * ddt_scale));let eq39_e1344_d_n10: f64 = (s.v[36] * (s.dn[1230][10] * ddt_scale));let eq39_e1344_d_n11: f64 = (s.v[36] * (s.dn[1230][11] * ddt_scale));let eq39_e1344_d_n12: f64 = (s.v[36] * (s.dn[1230][12] * ddt_scale));let eq39_e1344_d_b0: f64 = (s.v[36] * (s.db[1230][0] * ddt_scale));let eq39_e1344_d_b1: f64 = (s.v[36] * (s.db[1230][1] * ddt_scale));let eq39_e1344_d_b2: f64 = (s.v[36] * (s.db[1230][2] * ddt_scale));let eq39_e1344_d_b3: f64 = (s.v[36] * (s.db[1230][3] * ddt_scale));let eq39_e1344_d_b4: f64 = (s.v[36] * (s.db[1230][4] * ddt_scale));let eq39_e1344_d_b5: f64 = (s.v[36] * (s.db[1230][5] * ddt_scale));let eq39_e1344_d_b6: f64 = (s.v[36] * (s.db[1230][6] * ddt_scale));let eq39_e1344_d_b7: f64 = (s.v[36] * (s.db[1230][7] * ddt_scale));let eq39_e1344_d_b8: f64 = (s.v[36] * (s.db[1230][8] * ddt_scale));
        (eq39_e1344, eq39_e1344_d_n0, eq39_e1344_d_n1, eq39_e1344_d_n2, eq39_e1344_d_n3, eq39_e1344_d_n4, eq39_e1344_d_n5, eq39_e1344_d_n6, eq39_e1344_d_n7, eq39_e1344_d_n8, eq39_e1344_d_n9, eq39_e1344_d_n10, eq39_e1344_d_n11, eq39_e1344_d_n12, eq39_e1344_d_b0, eq39_e1344_d_b1, eq39_e1344_d_b2, eq39_e1344_d_b3, eq39_e1344_d_b4, eq39_e1344_d_b5, eq39_e1344_d_b6, eq39_e1344_d_b7, eq39_e1344_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e1346;let eq39_node_derivatives: [f64; 13] = [eq39_e1346_d_n0, eq39_e1346_d_n1, eq39_e1346_d_n2, eq39_e1346_d_n3, eq39_e1346_d_n4, eq39_e1346_d_n5, eq39_e1346_d_n6, eq39_e1346_d_n7, eq39_e1346_d_n8, eq39_e1346_d_n9, eq39_e1346_d_n10, eq39_e1346_d_n11, eq39_e1346_d_n12];let eq39_branch_derivatives: [f64; 9] = [eq39_e1346_d_b0, eq39_e1346_d_b1, eq39_e1346_d_b2, eq39_e1346_d_b3, eq39_e1346_d_b4, eq39_e1346_d_b5, eq39_e1346_d_b6, eq39_e1346_d_b7, eq39_e1346_d_b8];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq39_value),
            &eq39_node_derivatives,
            &eq39_branch_derivatives,
            multiplicity,
        );
        let (eq40_e1354, eq40_e1354_d_n0, eq40_e1354_d_n1, eq40_e1354_d_n2, eq40_e1354_d_n3, eq40_e1354_d_n4, eq40_e1354_d_n5, eq40_e1354_d_n6, eq40_e1354_d_n7, eq40_e1354_d_n8, eq40_e1354_d_n9, eq40_e1354_d_n10, eq40_e1354_d_n11, eq40_e1354_d_n12, eq40_e1354_d_b0, eq40_e1354_d_b1, eq40_e1354_d_b2, eq40_e1354_d_b3, eq40_e1354_d_b4, eq40_e1354_d_b5, eq40_e1354_d_b6, eq40_e1354_d_b7, eq40_e1354_d_b8,) = {
    if (!s.b[1863]) {
        let eq40_e1351: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, s.v[1231]);let eq40_e1352: f64 = (s.v[36] * eq40_e1351);let eq40_e1352_d_n0: f64 = (s.v[36] * (s.dn[1231][0] * ddt_scale));let eq40_e1352_d_n1: f64 = (s.v[36] * (s.dn[1231][1] * ddt_scale));let eq40_e1352_d_n2: f64 = (s.v[36] * (s.dn[1231][2] * ddt_scale));let eq40_e1352_d_n3: f64 = (s.v[36] * (s.dn[1231][3] * ddt_scale));let eq40_e1352_d_n4: f64 = (s.v[36] * (s.dn[1231][4] * ddt_scale));let eq40_e1352_d_n5: f64 = (s.v[36] * (s.dn[1231][5] * ddt_scale));let eq40_e1352_d_n6: f64 = (s.v[36] * (s.dn[1231][6] * ddt_scale));let eq40_e1352_d_n7: f64 = (s.v[36] * (s.dn[1231][7] * ddt_scale));let eq40_e1352_d_n8: f64 = (s.v[36] * (s.dn[1231][8] * ddt_scale));let eq40_e1352_d_n9: f64 = (s.v[36] * (s.dn[1231][9] * ddt_scale));let eq40_e1352_d_n10: f64 = (s.v[36] * (s.dn[1231][10] * ddt_scale));let eq40_e1352_d_n11: f64 = (s.v[36] * (s.dn[1231][11] * ddt_scale));let eq40_e1352_d_n12: f64 = (s.v[36] * (s.dn[1231][12] * ddt_scale));let eq40_e1352_d_b0: f64 = (s.v[36] * (s.db[1231][0] * ddt_scale));let eq40_e1352_d_b1: f64 = (s.v[36] * (s.db[1231][1] * ddt_scale));let eq40_e1352_d_b2: f64 = (s.v[36] * (s.db[1231][2] * ddt_scale));let eq40_e1352_d_b3: f64 = (s.v[36] * (s.db[1231][3] * ddt_scale));let eq40_e1352_d_b4: f64 = (s.v[36] * (s.db[1231][4] * ddt_scale));let eq40_e1352_d_b5: f64 = (s.v[36] * (s.db[1231][5] * ddt_scale));let eq40_e1352_d_b6: f64 = (s.v[36] * (s.db[1231][6] * ddt_scale));let eq40_e1352_d_b7: f64 = (s.v[36] * (s.db[1231][7] * ddt_scale));let eq40_e1352_d_b8: f64 = (s.v[36] * (s.db[1231][8] * ddt_scale));
        (eq40_e1352, eq40_e1352_d_n0, eq40_e1352_d_n1, eq40_e1352_d_n2, eq40_e1352_d_n3, eq40_e1352_d_n4, eq40_e1352_d_n5, eq40_e1352_d_n6, eq40_e1352_d_n7, eq40_e1352_d_n8, eq40_e1352_d_n9, eq40_e1352_d_n10, eq40_e1352_d_n11, eq40_e1352_d_n12, eq40_e1352_d_b0, eq40_e1352_d_b1, eq40_e1352_d_b2, eq40_e1352_d_b3, eq40_e1352_d_b4, eq40_e1352_d_b5, eq40_e1352_d_b6, eq40_e1352_d_b7, eq40_e1352_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_value: f64 = eq40_e1354;let eq40_node_derivatives: [f64; 13] = [eq40_e1354_d_n0, eq40_e1354_d_n1, eq40_e1354_d_n2, eq40_e1354_d_n3, eq40_e1354_d_n4, eq40_e1354_d_n5, eq40_e1354_d_n6, eq40_e1354_d_n7, eq40_e1354_d_n8, eq40_e1354_d_n9, eq40_e1354_d_n10, eq40_e1354_d_n11, eq40_e1354_d_n12];let eq40_branch_derivatives: [f64; 9] = [eq40_e1354_d_b0, eq40_e1354_d_b1, eq40_e1354_d_b2, eq40_e1354_d_b3, eq40_e1354_d_b4, eq40_e1354_d_b5, eq40_e1354_d_b6, eq40_e1354_d_b7, eq40_e1354_d_b8];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq40_value),
            &eq40_node_derivatives,
            &eq40_branch_derivatives,
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
        let nv1 = ctx.node_voltage(nodes[1]);let nv3 = ctx.node_voltage(nodes[3]);let nv9 = ctx.node_voltage(nodes[9]);let nv10 = ctx.node_voltage(nodes[10]);
        let (eq41_e1362, eq41_e1362_d_n0, eq41_e1362_d_n1, eq41_e1362_d_n2, eq41_e1362_d_n3, eq41_e1362_d_n4, eq41_e1362_d_n5, eq41_e1362_d_n6, eq41_e1362_d_n7, eq41_e1362_d_n8, eq41_e1362_d_n9, eq41_e1362_d_n10, eq41_e1362_d_n11, eq41_e1362_d_n12, eq41_e1362_d_b0, eq41_e1362_d_b1, eq41_e1362_d_b2, eq41_e1362_d_b3, eq41_e1362_d_b4, eq41_e1362_d_b5, eq41_e1362_d_b6, eq41_e1362_d_b7, eq41_e1362_d_b8,) = {
    if (!s.b[1863]) {
        let eq41_e1359: f64 = ((nv9 - nv3) * s.v[697]);let eq41_e1359_d_n0: f64 = ((nv9 - nv3) * s.dn[697][0]);let eq41_e1359_d_n1: f64 = ((nv9 - nv3) * s.dn[697][1]);let eq41_e1359_d_n2: f64 = ((nv9 - nv3) * s.dn[697][2]);let eq41_e1359_d_n3: f64 = ((-s.v[697]) + ((nv9 - nv3) * s.dn[697][3]));let eq41_e1359_d_n4: f64 = ((nv9 - nv3) * s.dn[697][4]);let eq41_e1359_d_n5: f64 = ((nv9 - nv3) * s.dn[697][5]);let eq41_e1359_d_n6: f64 = ((nv9 - nv3) * s.dn[697][6]);let eq41_e1359_d_n7: f64 = ((nv9 - nv3) * s.dn[697][7]);let eq41_e1359_d_n8: f64 = ((nv9 - nv3) * s.dn[697][8]);let eq41_e1359_d_n9: f64 = (s.v[697] + ((nv9 - nv3) * s.dn[697][9]));let eq41_e1359_d_n10: f64 = ((nv9 - nv3) * s.dn[697][10]);let eq41_e1359_d_n11: f64 = ((nv9 - nv3) * s.dn[697][11]);let eq41_e1359_d_n12: f64 = ((nv9 - nv3) * s.dn[697][12]);let eq41_e1359_d_b0: f64 = ((nv9 - nv3) * s.db[697][0]);let eq41_e1359_d_b1: f64 = ((nv9 - nv3) * s.db[697][1]);let eq41_e1359_d_b2: f64 = ((nv9 - nv3) * s.db[697][2]);let eq41_e1359_d_b3: f64 = ((nv9 - nv3) * s.db[697][3]);let eq41_e1359_d_b4: f64 = ((nv9 - nv3) * s.db[697][4]);let eq41_e1359_d_b5: f64 = ((nv9 - nv3) * s.db[697][5]);let eq41_e1359_d_b6: f64 = ((nv9 - nv3) * s.db[697][6]);let eq41_e1359_d_b7: f64 = ((nv9 - nv3) * s.db[697][7]);let eq41_e1359_d_b8: f64 = ((nv9 - nv3) * s.db[697][8]);let eq41_e1360: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq41_e1359);
        (eq41_e1360, (eq41_e1359_d_n0 * ddt_scale), (eq41_e1359_d_n1 * ddt_scale), (eq41_e1359_d_n2 * ddt_scale), (eq41_e1359_d_n3 * ddt_scale), (eq41_e1359_d_n4 * ddt_scale), (eq41_e1359_d_n5 * ddt_scale), (eq41_e1359_d_n6 * ddt_scale), (eq41_e1359_d_n7 * ddt_scale), (eq41_e1359_d_n8 * ddt_scale), (eq41_e1359_d_n9 * ddt_scale), (eq41_e1359_d_n10 * ddt_scale), (eq41_e1359_d_n11 * ddt_scale), (eq41_e1359_d_n12 * ddt_scale), (eq41_e1359_d_b0 * ddt_scale), (eq41_e1359_d_b1 * ddt_scale), (eq41_e1359_d_b2 * ddt_scale), (eq41_e1359_d_b3 * ddt_scale), (eq41_e1359_d_b4 * ddt_scale), (eq41_e1359_d_b5 * ddt_scale), (eq41_e1359_d_b6 * ddt_scale), (eq41_e1359_d_b7 * ddt_scale), (eq41_e1359_d_b8 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_value: f64 = eq41_e1362;let eq41_node_derivatives: [f64; 13] = [eq41_e1362_d_n0, eq41_e1362_d_n1, eq41_e1362_d_n2, eq41_e1362_d_n3, eq41_e1362_d_n4, eq41_e1362_d_n5, eq41_e1362_d_n6, eq41_e1362_d_n7, eq41_e1362_d_n8, eq41_e1362_d_n9, eq41_e1362_d_n10, eq41_e1362_d_n11, eq41_e1362_d_n12];let eq41_branch_derivatives: [f64; 9] = [eq41_e1362_d_b0, eq41_e1362_d_b1, eq41_e1362_d_b2, eq41_e1362_d_b3, eq41_e1362_d_b4, eq41_e1362_d_b5, eq41_e1362_d_b6, eq41_e1362_d_b7, eq41_e1362_d_b8];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(3),
            multiplicity * (eq41_value),
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
            multiplicity,
        );let eq42_e1364: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, s.v[449]);let eq42_value: f64 = eq42_e1364;
        stamper.stamp_current_dense_local(
            Some(7),
            Some(3),
            multiplicity * (eq42_value),
            &s.dn[449],
            &s.db[449],
            (multiplicity) * (ddt_scale),
        );let eq43_e1366: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, s.v[448]);let eq43_value: f64 = eq43_e1366;
        stamper.stamp_current_dense_local(
            Some(8),
            Some(3),
            multiplicity * (eq43_value),
            &s.dn[448],
            &s.db[448],
            (multiplicity) * (ddt_scale),
        );
        let (eq44_e1370,) = {
    if s.b[1864] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq44_value: f64 = eq44_e1370;
        stamper.stamp_potential_const_local(
            3,
            eq44_value,
        );
        let (eq45_e1377, eq45_e1377_d_n0, eq45_e1377_d_n1, eq45_e1377_d_n2, eq45_e1377_d_n3, eq45_e1377_d_n4, eq45_e1377_d_n5, eq45_e1377_d_n6, eq45_e1377_d_n7, eq45_e1377_d_n8, eq45_e1377_d_n9, eq45_e1377_d_n10, eq45_e1377_d_n11, eq45_e1377_d_n12, eq45_e1377_d_b0, eq45_e1377_d_b1, eq45_e1377_d_b2, eq45_e1377_d_b3, eq45_e1377_d_b4, eq45_e1377_d_b5, eq45_e1377_d_b6, eq45_e1377_d_b7, eq45_e1377_d_b8,) = {
    if (!s.b[1864]) {
        let eq45_e1375: f64 = ((nv1 - nv10) * s.v[421]);let eq45_e1375_d_n0: f64 = ((nv1 - nv10) * s.dn[421][0]);let eq45_e1375_d_n1: f64 = (s.v[421] + ((nv1 - nv10) * s.dn[421][1]));let eq45_e1375_d_n2: f64 = ((nv1 - nv10) * s.dn[421][2]);let eq45_e1375_d_n3: f64 = ((nv1 - nv10) * s.dn[421][3]);let eq45_e1375_d_n4: f64 = ((nv1 - nv10) * s.dn[421][4]);let eq45_e1375_d_n5: f64 = ((nv1 - nv10) * s.dn[421][5]);let eq45_e1375_d_n6: f64 = ((nv1 - nv10) * s.dn[421][6]);let eq45_e1375_d_n7: f64 = ((nv1 - nv10) * s.dn[421][7]);let eq45_e1375_d_n8: f64 = ((nv1 - nv10) * s.dn[421][8]);let eq45_e1375_d_n9: f64 = ((nv1 - nv10) * s.dn[421][9]);let eq45_e1375_d_n10: f64 = ((-s.v[421]) + ((nv1 - nv10) * s.dn[421][10]));let eq45_e1375_d_n11: f64 = ((nv1 - nv10) * s.dn[421][11]);let eq45_e1375_d_n12: f64 = ((nv1 - nv10) * s.dn[421][12]);let eq45_e1375_d_b0: f64 = ((nv1 - nv10) * s.db[421][0]);let eq45_e1375_d_b1: f64 = ((nv1 - nv10) * s.db[421][1]);let eq45_e1375_d_b2: f64 = ((nv1 - nv10) * s.db[421][2]);let eq45_e1375_d_b3: f64 = ((nv1 - nv10) * s.db[421][3]);let eq45_e1375_d_b4: f64 = ((nv1 - nv10) * s.db[421][4]);let eq45_e1375_d_b5: f64 = ((nv1 - nv10) * s.db[421][5]);let eq45_e1375_d_b6: f64 = ((nv1 - nv10) * s.db[421][6]);let eq45_e1375_d_b7: f64 = ((nv1 - nv10) * s.db[421][7]);let eq45_e1375_d_b8: f64 = ((nv1 - nv10) * s.db[421][8]);
        (eq45_e1375, eq45_e1375_d_n0, eq45_e1375_d_n1, eq45_e1375_d_n2, eq45_e1375_d_n3, eq45_e1375_d_n4, eq45_e1375_d_n5, eq45_e1375_d_n6, eq45_e1375_d_n7, eq45_e1375_d_n8, eq45_e1375_d_n9, eq45_e1375_d_n10, eq45_e1375_d_n11, eq45_e1375_d_n12, eq45_e1375_d_b0, eq45_e1375_d_b1, eq45_e1375_d_b2, eq45_e1375_d_b3, eq45_e1375_d_b4, eq45_e1375_d_b5, eq45_e1375_d_b6, eq45_e1375_d_b7, eq45_e1375_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq45_value: f64 = eq45_e1377;let eq45_node_derivatives: [f64; 13] = [eq45_e1377_d_n0, eq45_e1377_d_n1, eq45_e1377_d_n2, eq45_e1377_d_n3, eq45_e1377_d_n4, eq45_e1377_d_n5, eq45_e1377_d_n6, eq45_e1377_d_n7, eq45_e1377_d_n8, eq45_e1377_d_n9, eq45_e1377_d_n10, eq45_e1377_d_n11, eq45_e1377_d_n12];let eq45_branch_derivatives: [f64; 9] = [eq45_e1377_d_b0, eq45_e1377_d_b1, eq45_e1377_d_b2, eq45_e1377_d_b3, eq45_e1377_d_b4, eq45_e1377_d_b5, eq45_e1377_d_b6, eq45_e1377_d_b7, eq45_e1377_d_b8];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(10),
            multiplicity * (eq45_value),
            &eq45_node_derivatives,
            &eq45_branch_derivatives,
            multiplicity,
        );
        let (eq47_e1391,) = {
    if s.b[1865] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq47_value: f64 = eq47_e1391;
        stamper.stamp_potential_const_local(
            4,
            eq47_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_8(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);let nv9 = ctx.node_voltage(nodes[9]);let nv10 = ctx.node_voltage(nodes[10]);let nv11 = ctx.node_voltage(nodes[11]);let nv12 = ctx.node_voltage(nodes[12]);
        let (eq48_e1398, eq48_e1398_d_n0, eq48_e1398_d_n1, eq48_e1398_d_n2, eq48_e1398_d_n3, eq48_e1398_d_n4, eq48_e1398_d_n5, eq48_e1398_d_n6, eq48_e1398_d_n7, eq48_e1398_d_n8, eq48_e1398_d_n9, eq48_e1398_d_n10, eq48_e1398_d_n11, eq48_e1398_d_n12, eq48_e1398_d_b0, eq48_e1398_d_b1, eq48_e1398_d_b2, eq48_e1398_d_b3, eq48_e1398_d_b4, eq48_e1398_d_b5, eq48_e1398_d_b6, eq48_e1398_d_b7, eq48_e1398_d_b8,) = {
    if (!s.b[1865]) {
        let eq48_e1396: f64 = ((nv10 - nv9) * s.v[413]);let eq48_e1396_d_n0: f64 = ((nv10 - nv9) * s.dn[413][0]);let eq48_e1396_d_n1: f64 = ((nv10 - nv9) * s.dn[413][1]);let eq48_e1396_d_n2: f64 = ((nv10 - nv9) * s.dn[413][2]);let eq48_e1396_d_n3: f64 = ((nv10 - nv9) * s.dn[413][3]);let eq48_e1396_d_n4: f64 = ((nv10 - nv9) * s.dn[413][4]);let eq48_e1396_d_n5: f64 = ((nv10 - nv9) * s.dn[413][5]);let eq48_e1396_d_n6: f64 = ((nv10 - nv9) * s.dn[413][6]);let eq48_e1396_d_n7: f64 = ((nv10 - nv9) * s.dn[413][7]);let eq48_e1396_d_n8: f64 = ((nv10 - nv9) * s.dn[413][8]);let eq48_e1396_d_n9: f64 = ((-s.v[413]) + ((nv10 - nv9) * s.dn[413][9]));let eq48_e1396_d_n10: f64 = (s.v[413] + ((nv10 - nv9) * s.dn[413][10]));let eq48_e1396_d_n11: f64 = ((nv10 - nv9) * s.dn[413][11]);let eq48_e1396_d_n12: f64 = ((nv10 - nv9) * s.dn[413][12]);let eq48_e1396_d_b0: f64 = ((nv10 - nv9) * s.db[413][0]);let eq48_e1396_d_b1: f64 = ((nv10 - nv9) * s.db[413][1]);let eq48_e1396_d_b2: f64 = ((nv10 - nv9) * s.db[413][2]);let eq48_e1396_d_b3: f64 = ((nv10 - nv9) * s.db[413][3]);let eq48_e1396_d_b4: f64 = ((nv10 - nv9) * s.db[413][4]);let eq48_e1396_d_b5: f64 = ((nv10 - nv9) * s.db[413][5]);let eq48_e1396_d_b6: f64 = ((nv10 - nv9) * s.db[413][6]);let eq48_e1396_d_b7: f64 = ((nv10 - nv9) * s.db[413][7]);let eq48_e1396_d_b8: f64 = ((nv10 - nv9) * s.db[413][8]);
        (eq48_e1396, eq48_e1396_d_n0, eq48_e1396_d_n1, eq48_e1396_d_n2, eq48_e1396_d_n3, eq48_e1396_d_n4, eq48_e1396_d_n5, eq48_e1396_d_n6, eq48_e1396_d_n7, eq48_e1396_d_n8, eq48_e1396_d_n9, eq48_e1396_d_n10, eq48_e1396_d_n11, eq48_e1396_d_n12, eq48_e1396_d_b0, eq48_e1396_d_b1, eq48_e1396_d_b2, eq48_e1396_d_b3, eq48_e1396_d_b4, eq48_e1396_d_b5, eq48_e1396_d_b6, eq48_e1396_d_b7, eq48_e1396_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_value: f64 = eq48_e1398;let eq48_node_derivatives: [f64; 13] = [eq48_e1398_d_n0, eq48_e1398_d_n1, eq48_e1398_d_n2, eq48_e1398_d_n3, eq48_e1398_d_n4, eq48_e1398_d_n5, eq48_e1398_d_n6, eq48_e1398_d_n7, eq48_e1398_d_n8, eq48_e1398_d_n9, eq48_e1398_d_n10, eq48_e1398_d_n11, eq48_e1398_d_n12];let eq48_branch_derivatives: [f64; 9] = [eq48_e1398_d_b0, eq48_e1398_d_b1, eq48_e1398_d_b2, eq48_e1398_d_b3, eq48_e1398_d_b4, eq48_e1398_d_b5, eq48_e1398_d_b6, eq48_e1398_d_b7, eq48_e1398_d_b8];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(9),
            multiplicity * (eq48_value),
            &eq48_node_derivatives,
            &eq48_branch_derivatives,
            multiplicity,
        );
        let (eq50_e1420, eq50_e1420_d_n0, eq50_e1420_d_n1, eq50_e1420_d_n2, eq50_e1420_d_n3, eq50_e1420_d_n4, eq50_e1420_d_n5, eq50_e1420_d_n6, eq50_e1420_d_n7, eq50_e1420_d_n8, eq50_e1420_d_n9, eq50_e1420_d_n10, eq50_e1420_d_n11, eq50_e1420_d_n12, eq50_e1420_d_b0, eq50_e1420_d_b1, eq50_e1420_d_b2, eq50_e1420_d_b3, eq50_e1420_d_b4, eq50_e1420_d_b5, eq50_e1420_d_b6, eq50_e1420_d_b7, eq50_e1420_d_b8,) = {
    if (s.v[67] != 0.0) {
        let eq50_e1418: f64 = ((nv5 - nv12) * s.v[416]);let eq50_e1418_d_n0: f64 = ((nv5 - nv12) * s.dn[416][0]);let eq50_e1418_d_n1: f64 = ((nv5 - nv12) * s.dn[416][1]);let eq50_e1418_d_n2: f64 = ((nv5 - nv12) * s.dn[416][2]);let eq50_e1418_d_n3: f64 = ((nv5 - nv12) * s.dn[416][3]);let eq50_e1418_d_n4: f64 = ((nv5 - nv12) * s.dn[416][4]);let eq50_e1418_d_n5: f64 = (s.v[416] + ((nv5 - nv12) * s.dn[416][5]));let eq50_e1418_d_n6: f64 = ((nv5 - nv12) * s.dn[416][6]);let eq50_e1418_d_n7: f64 = ((nv5 - nv12) * s.dn[416][7]);let eq50_e1418_d_n8: f64 = ((nv5 - nv12) * s.dn[416][8]);let eq50_e1418_d_n9: f64 = ((nv5 - nv12) * s.dn[416][9]);let eq50_e1418_d_n10: f64 = ((nv5 - nv12) * s.dn[416][10]);let eq50_e1418_d_n11: f64 = ((nv5 - nv12) * s.dn[416][11]);let eq50_e1418_d_n12: f64 = ((-s.v[416]) + ((nv5 - nv12) * s.dn[416][12]));let eq50_e1418_d_b0: f64 = ((nv5 - nv12) * s.db[416][0]);let eq50_e1418_d_b1: f64 = ((nv5 - nv12) * s.db[416][1]);let eq50_e1418_d_b2: f64 = ((nv5 - nv12) * s.db[416][2]);let eq50_e1418_d_b3: f64 = ((nv5 - nv12) * s.db[416][3]);let eq50_e1418_d_b4: f64 = ((nv5 - nv12) * s.db[416][4]);let eq50_e1418_d_b5: f64 = ((nv5 - nv12) * s.db[416][5]);let eq50_e1418_d_b6: f64 = ((nv5 - nv12) * s.db[416][6]);let eq50_e1418_d_b7: f64 = ((nv5 - nv12) * s.db[416][7]);let eq50_e1418_d_b8: f64 = ((nv5 - nv12) * s.db[416][8]);
        (eq50_e1418, eq50_e1418_d_n0, eq50_e1418_d_n1, eq50_e1418_d_n2, eq50_e1418_d_n3, eq50_e1418_d_n4, eq50_e1418_d_n5, eq50_e1418_d_n6, eq50_e1418_d_n7, eq50_e1418_d_n8, eq50_e1418_d_n9, eq50_e1418_d_n10, eq50_e1418_d_n11, eq50_e1418_d_n12, eq50_e1418_d_b0, eq50_e1418_d_b1, eq50_e1418_d_b2, eq50_e1418_d_b3, eq50_e1418_d_b4, eq50_e1418_d_b5, eq50_e1418_d_b6, eq50_e1418_d_b7, eq50_e1418_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e1420;let eq50_node_derivatives: [f64; 13] = [eq50_e1420_d_n0, eq50_e1420_d_n1, eq50_e1420_d_n2, eq50_e1420_d_n3, eq50_e1420_d_n4, eq50_e1420_d_n5, eq50_e1420_d_n6, eq50_e1420_d_n7, eq50_e1420_d_n8, eq50_e1420_d_n9, eq50_e1420_d_n10, eq50_e1420_d_n11, eq50_e1420_d_n12];let eq50_branch_derivatives: [f64; 9] = [eq50_e1420_d_b0, eq50_e1420_d_b1, eq50_e1420_d_b2, eq50_e1420_d_b3, eq50_e1420_d_b4, eq50_e1420_d_b5, eq50_e1420_d_b6, eq50_e1420_d_b7, eq50_e1420_d_b8];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(12),
            multiplicity * (eq50_value),
            &eq50_node_derivatives,
            &eq50_branch_derivatives,
            multiplicity,
        );
        let (eq51_e1426, eq51_e1426_d_n0, eq51_e1426_d_n1, eq51_e1426_d_n2, eq51_e1426_d_n3, eq51_e1426_d_n4, eq51_e1426_d_n5, eq51_e1426_d_n6, eq51_e1426_d_n7, eq51_e1426_d_n8, eq51_e1426_d_n9, eq51_e1426_d_n10, eq51_e1426_d_n11, eq51_e1426_d_n12, eq51_e1426_d_b0, eq51_e1426_d_b1, eq51_e1426_d_b2, eq51_e1426_d_b3, eq51_e1426_d_b4, eq51_e1426_d_b5, eq51_e1426_d_b6, eq51_e1426_d_b7, eq51_e1426_d_b8,) = {
    if (s.v[67] != 0.0) {
        let eq51_e1424: f64 = ((nv5 - nv11) * s.v[415]);let eq51_e1424_d_n0: f64 = ((nv5 - nv11) * s.dn[415][0]);let eq51_e1424_d_n1: f64 = ((nv5 - nv11) * s.dn[415][1]);let eq51_e1424_d_n2: f64 = ((nv5 - nv11) * s.dn[415][2]);let eq51_e1424_d_n3: f64 = ((nv5 - nv11) * s.dn[415][3]);let eq51_e1424_d_n4: f64 = ((nv5 - nv11) * s.dn[415][4]);let eq51_e1424_d_n5: f64 = (s.v[415] + ((nv5 - nv11) * s.dn[415][5]));let eq51_e1424_d_n6: f64 = ((nv5 - nv11) * s.dn[415][6]);let eq51_e1424_d_n7: f64 = ((nv5 - nv11) * s.dn[415][7]);let eq51_e1424_d_n8: f64 = ((nv5 - nv11) * s.dn[415][8]);let eq51_e1424_d_n9: f64 = ((nv5 - nv11) * s.dn[415][9]);let eq51_e1424_d_n10: f64 = ((nv5 - nv11) * s.dn[415][10]);let eq51_e1424_d_n11: f64 = ((-s.v[415]) + ((nv5 - nv11) * s.dn[415][11]));let eq51_e1424_d_n12: f64 = ((nv5 - nv11) * s.dn[415][12]);let eq51_e1424_d_b0: f64 = ((nv5 - nv11) * s.db[415][0]);let eq51_e1424_d_b1: f64 = ((nv5 - nv11) * s.db[415][1]);let eq51_e1424_d_b2: f64 = ((nv5 - nv11) * s.db[415][2]);let eq51_e1424_d_b3: f64 = ((nv5 - nv11) * s.db[415][3]);let eq51_e1424_d_b4: f64 = ((nv5 - nv11) * s.db[415][4]);let eq51_e1424_d_b5: f64 = ((nv5 - nv11) * s.db[415][5]);let eq51_e1424_d_b6: f64 = ((nv5 - nv11) * s.db[415][6]);let eq51_e1424_d_b7: f64 = ((nv5 - nv11) * s.db[415][7]);let eq51_e1424_d_b8: f64 = ((nv5 - nv11) * s.db[415][8]);
        (eq51_e1424, eq51_e1424_d_n0, eq51_e1424_d_n1, eq51_e1424_d_n2, eq51_e1424_d_n3, eq51_e1424_d_n4, eq51_e1424_d_n5, eq51_e1424_d_n6, eq51_e1424_d_n7, eq51_e1424_d_n8, eq51_e1424_d_n9, eq51_e1424_d_n10, eq51_e1424_d_n11, eq51_e1424_d_n12, eq51_e1424_d_b0, eq51_e1424_d_b1, eq51_e1424_d_b2, eq51_e1424_d_b3, eq51_e1424_d_b4, eq51_e1424_d_b5, eq51_e1424_d_b6, eq51_e1424_d_b7, eq51_e1424_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e1426;let eq51_node_derivatives: [f64; 13] = [eq51_e1426_d_n0, eq51_e1426_d_n1, eq51_e1426_d_n2, eq51_e1426_d_n3, eq51_e1426_d_n4, eq51_e1426_d_n5, eq51_e1426_d_n6, eq51_e1426_d_n7, eq51_e1426_d_n8, eq51_e1426_d_n9, eq51_e1426_d_n10, eq51_e1426_d_n11, eq51_e1426_d_n12];let eq51_branch_derivatives: [f64; 9] = [eq51_e1426_d_b0, eq51_e1426_d_b1, eq51_e1426_d_b2, eq51_e1426_d_b3, eq51_e1426_d_b4, eq51_e1426_d_b5, eq51_e1426_d_b6, eq51_e1426_d_b7, eq51_e1426_d_b8];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(11),
            multiplicity * (eq51_value),
            &eq51_node_derivatives,
            &eq51_branch_derivatives,
            multiplicity,
        );
        let (eq54_e1449,) = {
    if (s.v[67] == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq54_value: f64 = eq54_e1449;
        stamper.stamp_potential_const_local(
            5,
            eq54_value,
        );
        let (eq55_e1454,) = {
    if (s.v[67] == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq55_value: f64 = eq55_e1454;
        stamper.stamp_potential_const_local(
            6,
            eq55_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_9(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
    ) {
        let (eq56_e1458,) = {
    if s.b[1868] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq56_value: f64 = eq56_e1458;
        stamper.stamp_potential_const_local(
            7,
            eq56_value,
        );
        let (eq57_e1469, eq57_e1469_d_n0, eq57_e1469_d_n1, eq57_e1469_d_n2, eq57_e1469_d_n3, eq57_e1469_d_n4, eq57_e1469_d_n5, eq57_e1469_d_n6, eq57_e1469_d_n7, eq57_e1469_d_n8, eq57_e1469_d_n9, eq57_e1469_d_n10, eq57_e1469_d_n11, eq57_e1469_d_n12, eq57_e1469_d_b0, eq57_e1469_d_b1, eq57_e1469_d_b2, eq57_e1469_d_b3, eq57_e1469_d_b4, eq57_e1469_d_b5, eq57_e1469_d_b6, eq57_e1469_d_b7, eq57_e1469_d_b8,) = {
    if s.b[1869] {
        let eq57_e1461: f64 = (-s.v[1220]);let eq57_e1463: f64 = (eq57_e1461 * s.v[1158]);let eq57_e1463_d_n0: f64 = (((-s.dn[1220][0]) * s.v[1158]) + (eq57_e1461 * s.dn[1158][0]));let eq57_e1463_d_n1: f64 = (((-s.dn[1220][1]) * s.v[1158]) + (eq57_e1461 * s.dn[1158][1]));let eq57_e1463_d_n2: f64 = (((-s.dn[1220][2]) * s.v[1158]) + (eq57_e1461 * s.dn[1158][2]));let eq57_e1463_d_n3: f64 = (((-s.dn[1220][3]) * s.v[1158]) + (eq57_e1461 * s.dn[1158][3]));let eq57_e1463_d_n4: f64 = (((-s.dn[1220][4]) * s.v[1158]) + (eq57_e1461 * s.dn[1158][4]));let eq57_e1463_d_n5: f64 = (((-s.dn[1220][5]) * s.v[1158]) + (eq57_e1461 * s.dn[1158][5]));let eq57_e1463_d_n6: f64 = (((-s.dn[1220][6]) * s.v[1158]) + (eq57_e1461 * s.dn[1158][6]));let eq57_e1463_d_n7: f64 = (((-s.dn[1220][7]) * s.v[1158]) + (eq57_e1461 * s.dn[1158][7]));let eq57_e1463_d_n8: f64 = (((-s.dn[1220][8]) * s.v[1158]) + (eq57_e1461 * s.dn[1158][8]));let eq57_e1463_d_n9: f64 = (((-s.dn[1220][9]) * s.v[1158]) + (eq57_e1461 * s.dn[1158][9]));let eq57_e1463_d_n10: f64 = (((-s.dn[1220][10]) * s.v[1158]) + (eq57_e1461 * s.dn[1158][10]));let eq57_e1463_d_n11: f64 = (((-s.dn[1220][11]) * s.v[1158]) + (eq57_e1461 * s.dn[1158][11]));let eq57_e1463_d_n12: f64 = (((-s.dn[1220][12]) * s.v[1158]) + (eq57_e1461 * s.dn[1158][12]));let eq57_e1463_d_b0: f64 = (((-s.db[1220][0]) * s.v[1158]) + (eq57_e1461 * s.db[1158][0]));let eq57_e1463_d_b1: f64 = (((-s.db[1220][1]) * s.v[1158]) + (eq57_e1461 * s.db[1158][1]));let eq57_e1463_d_b2: f64 = (((-s.db[1220][2]) * s.v[1158]) + (eq57_e1461 * s.db[1158][2]));let eq57_e1463_d_b3: f64 = (((-s.db[1220][3]) * s.v[1158]) + (eq57_e1461 * s.db[1158][3]));let eq57_e1463_d_b4: f64 = (((-s.db[1220][4]) * s.v[1158]) + (eq57_e1461 * s.db[1158][4]));let eq57_e1463_d_b5: f64 = (((-s.db[1220][5]) * s.v[1158]) + (eq57_e1461 * s.db[1158][5]));let eq57_e1463_d_b6: f64 = (((-s.db[1220][6]) * s.v[1158]) + (eq57_e1461 * s.db[1158][6]));let eq57_e1463_d_b7: f64 = (((-s.db[1220][7]) * s.v[1158]) + (eq57_e1461 * s.db[1158][7]));let eq57_e1463_d_b8: f64 = (((-s.db[1220][8]) * s.v[1158]) + (eq57_e1461 * s.db[1158][8]));let __rspice_inv_cse_0: f64 = 1.0 / s.v[527];let eq57_e1466: f64 = (s.v[770] * __rspice_inv_cse_0);let eq57_e1466_d_n0: f64 = (s.dn[770][0] * __rspice_inv_cse_0);let eq57_e1466_d_n1: f64 = (s.dn[770][1] * __rspice_inv_cse_0);let eq57_e1466_d_n2: f64 = (s.dn[770][2] * __rspice_inv_cse_0);let eq57_e1466_d_n3: f64 = (s.dn[770][3] * __rspice_inv_cse_0);let eq57_e1466_d_n4: f64 = (s.dn[770][4] * __rspice_inv_cse_0);let eq57_e1466_d_n5: f64 = (s.dn[770][5] * __rspice_inv_cse_0);let eq57_e1466_d_n6: f64 = (s.dn[770][6] * __rspice_inv_cse_0);let eq57_e1466_d_n7: f64 = (s.dn[770][7] * __rspice_inv_cse_0);let eq57_e1466_d_n8: f64 = (s.dn[770][8] * __rspice_inv_cse_0);let eq57_e1466_d_n9: f64 = (s.dn[770][9] * __rspice_inv_cse_0);let eq57_e1466_d_n10: f64 = (s.dn[770][10] * __rspice_inv_cse_0);let eq57_e1466_d_n11: f64 = (s.dn[770][11] * __rspice_inv_cse_0);let eq57_e1466_d_n12: f64 = (s.dn[770][12] * __rspice_inv_cse_0);let eq57_e1466_d_b0: f64 = (s.db[770][0] * __rspice_inv_cse_0);let eq57_e1466_d_b1: f64 = (s.db[770][1] * __rspice_inv_cse_0);let eq57_e1466_d_b2: f64 = (s.db[770][2] * __rspice_inv_cse_0);let eq57_e1466_d_b3: f64 = (s.db[770][3] * __rspice_inv_cse_0);let eq57_e1466_d_b4: f64 = (s.db[770][4] * __rspice_inv_cse_0);let eq57_e1466_d_b5: f64 = (s.db[770][5] * __rspice_inv_cse_0);let eq57_e1466_d_b6: f64 = (s.db[770][6] * __rspice_inv_cse_0);let eq57_e1466_d_b7: f64 = (s.db[770][7] * __rspice_inv_cse_0);let eq57_e1466_d_b8: f64 = (s.db[770][8] * __rspice_inv_cse_0);let eq57_e1467: f64 = (eq57_e1463 + eq57_e1466);let eq57_e1467_d_n0: f64 = (eq57_e1463_d_n0 + eq57_e1466_d_n0);let eq57_e1467_d_n1: f64 = (eq57_e1463_d_n1 + eq57_e1466_d_n1);let eq57_e1467_d_n2: f64 = (eq57_e1463_d_n2 + eq57_e1466_d_n2);let eq57_e1467_d_n3: f64 = (eq57_e1463_d_n3 + eq57_e1466_d_n3);let eq57_e1467_d_n4: f64 = (eq57_e1463_d_n4 + eq57_e1466_d_n4);let eq57_e1467_d_n5: f64 = (eq57_e1463_d_n5 + eq57_e1466_d_n5);let eq57_e1467_d_n6: f64 = (eq57_e1463_d_n6 + eq57_e1466_d_n6);
        let eq57_e1467_d_n7: f64 = (eq57_e1463_d_n7 + eq57_e1466_d_n7);let eq57_e1467_d_n8: f64 = (eq57_e1463_d_n8 + eq57_e1466_d_n8);let eq57_e1467_d_n9: f64 = (eq57_e1463_d_n9 + eq57_e1466_d_n9);let eq57_e1467_d_n10: f64 = (eq57_e1463_d_n10 + eq57_e1466_d_n10);let eq57_e1467_d_n11: f64 = (eq57_e1463_d_n11 + eq57_e1466_d_n11);let eq57_e1467_d_n12: f64 = (eq57_e1463_d_n12 + eq57_e1466_d_n12);let eq57_e1467_d_b0: f64 = (eq57_e1463_d_b0 + eq57_e1466_d_b0);let eq57_e1467_d_b1: f64 = (eq57_e1463_d_b1 + eq57_e1466_d_b1);let eq57_e1467_d_b2: f64 = (eq57_e1463_d_b2 + eq57_e1466_d_b2);let eq57_e1467_d_b3: f64 = (eq57_e1463_d_b3 + eq57_e1466_d_b3);let eq57_e1467_d_b4: f64 = (eq57_e1463_d_b4 + eq57_e1466_d_b4);let eq57_e1467_d_b5: f64 = (eq57_e1463_d_b5 + eq57_e1466_d_b5);let eq57_e1467_d_b6: f64 = (eq57_e1463_d_b6 + eq57_e1466_d_b6);let eq57_e1467_d_b7: f64 = (eq57_e1463_d_b7 + eq57_e1466_d_b7);let eq57_e1467_d_b8: f64 = (eq57_e1463_d_b8 + eq57_e1466_d_b8);
        (eq57_e1467, eq57_e1467_d_n0, eq57_e1467_d_n1, eq57_e1467_d_n2, eq57_e1467_d_n3, eq57_e1467_d_n4, eq57_e1467_d_n5, eq57_e1467_d_n6, eq57_e1467_d_n7, eq57_e1467_d_n8, eq57_e1467_d_n9, eq57_e1467_d_n10, eq57_e1467_d_n11, eq57_e1467_d_n12, eq57_e1467_d_b0, eq57_e1467_d_b1, eq57_e1467_d_b2, eq57_e1467_d_b3, eq57_e1467_d_b4, eq57_e1467_d_b5, eq57_e1467_d_b6, eq57_e1467_d_b7, eq57_e1467_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e1469;let eq57_node_derivatives: [f64; 13] = [eq57_e1469_d_n0, eq57_e1469_d_n1, eq57_e1469_d_n2, eq57_e1469_d_n3, eq57_e1469_d_n4, eq57_e1469_d_n5, eq57_e1469_d_n6, eq57_e1469_d_n7, eq57_e1469_d_n8, eq57_e1469_d_n9, eq57_e1469_d_n10, eq57_e1469_d_n11, eq57_e1469_d_n12];let eq57_branch_derivatives: [f64; 9] = [eq57_e1469_d_b0, eq57_e1469_d_b1, eq57_e1469_d_b2, eq57_e1469_d_b3, eq57_e1469_d_b4, eq57_e1469_d_b5, eq57_e1469_d_b6, eq57_e1469_d_b7, eq57_e1469_d_b8];
        stamper.stamp_current_dense_local(
            Some(6),
            None,
            multiplicity * (eq57_value),
            &eq57_node_derivatives,
            &eq57_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_10(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
        let (eq58_e1476, eq58_e1476_d_n0, eq58_e1476_d_n1, eq58_e1476_d_n2, eq58_e1476_d_n3, eq58_e1476_d_n4, eq58_e1476_d_n5, eq58_e1476_d_n6, eq58_e1476_d_n7, eq58_e1476_d_n8, eq58_e1476_d_n9, eq58_e1476_d_n10, eq58_e1476_d_n11, eq58_e1476_d_n12, eq58_e1476_d_b0, eq58_e1476_d_b1, eq58_e1476_d_b2, eq58_e1476_d_b3, eq58_e1476_d_b4, eq58_e1476_d_b5, eq58_e1476_d_b6, eq58_e1476_d_b7, eq58_e1476_d_b8,) = {
    if s.b[1869] {
        let eq58_e1473: f64 = (s.v[770] * s.v[528]);let eq58_e1474: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq58_e1473);let eq58_e1474_d_n0: f64 = ((s.dn[770][0] * s.v[528]) * ddt_scale);let eq58_e1474_d_n1: f64 = ((s.dn[770][1] * s.v[528]) * ddt_scale);let eq58_e1474_d_n2: f64 = ((s.dn[770][2] * s.v[528]) * ddt_scale);let eq58_e1474_d_n3: f64 = ((s.dn[770][3] * s.v[528]) * ddt_scale);let eq58_e1474_d_n4: f64 = ((s.dn[770][4] * s.v[528]) * ddt_scale);let eq58_e1474_d_n5: f64 = ((s.dn[770][5] * s.v[528]) * ddt_scale);let eq58_e1474_d_n6: f64 = ((s.dn[770][6] * s.v[528]) * ddt_scale);let eq58_e1474_d_n7: f64 = ((s.dn[770][7] * s.v[528]) * ddt_scale);let eq58_e1474_d_n8: f64 = ((s.dn[770][8] * s.v[528]) * ddt_scale);let eq58_e1474_d_n9: f64 = ((s.dn[770][9] * s.v[528]) * ddt_scale);let eq58_e1474_d_n10: f64 = ((s.dn[770][10] * s.v[528]) * ddt_scale);let eq58_e1474_d_n11: f64 = ((s.dn[770][11] * s.v[528]) * ddt_scale);let eq58_e1474_d_n12: f64 = ((s.dn[770][12] * s.v[528]) * ddt_scale);let eq58_e1474_d_b0: f64 = ((s.db[770][0] * s.v[528]) * ddt_scale);let eq58_e1474_d_b1: f64 = ((s.db[770][1] * s.v[528]) * ddt_scale);let eq58_e1474_d_b2: f64 = ((s.db[770][2] * s.v[528]) * ddt_scale);let eq58_e1474_d_b3: f64 = ((s.db[770][3] * s.v[528]) * ddt_scale);let eq58_e1474_d_b4: f64 = ((s.db[770][4] * s.v[528]) * ddt_scale);let eq58_e1474_d_b5: f64 = ((s.db[770][5] * s.v[528]) * ddt_scale);let eq58_e1474_d_b6: f64 = ((s.db[770][6] * s.v[528]) * ddt_scale);let eq58_e1474_d_b7: f64 = ((s.db[770][7] * s.v[528]) * ddt_scale);let eq58_e1474_d_b8: f64 = ((s.db[770][8] * s.v[528]) * ddt_scale);
        (eq58_e1474, eq58_e1474_d_n0, eq58_e1474_d_n1, eq58_e1474_d_n2, eq58_e1474_d_n3, eq58_e1474_d_n4, eq58_e1474_d_n5, eq58_e1474_d_n6, eq58_e1474_d_n7, eq58_e1474_d_n8, eq58_e1474_d_n9, eq58_e1474_d_n10, eq58_e1474_d_n11, eq58_e1474_d_n12, eq58_e1474_d_b0, eq58_e1474_d_b1, eq58_e1474_d_b2, eq58_e1474_d_b3, eq58_e1474_d_b4, eq58_e1474_d_b5, eq58_e1474_d_b6, eq58_e1474_d_b7, eq58_e1474_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e1476;let eq58_node_derivatives: [f64; 13] = [eq58_e1476_d_n0, eq58_e1476_d_n1, eq58_e1476_d_n2, eq58_e1476_d_n3, eq58_e1476_d_n4, eq58_e1476_d_n5, eq58_e1476_d_n6, eq58_e1476_d_n7, eq58_e1476_d_n8, eq58_e1476_d_n9, eq58_e1476_d_n10, eq58_e1476_d_n11, eq58_e1476_d_n12];let eq58_branch_derivatives: [f64; 9] = [eq58_e1476_d_b0, eq58_e1476_d_b1, eq58_e1476_d_b2, eq58_e1476_d_b3, eq58_e1476_d_b4, eq58_e1476_d_b5, eq58_e1476_d_b6, eq58_e1476_d_b7, eq58_e1476_d_b8];
        stamper.stamp_current_dense_local(
            Some(6),
            None,
            multiplicity * (eq58_value),
            &eq58_node_derivatives,
            &eq58_branch_derivatives,
            multiplicity,
        );
        let (eq59_e1481,) = {
    if (!s.b[1869]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq59_value: f64 = eq59_e1481;
        stamper.stamp_potential_const_local(
            8,
            eq59_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);let nv10 = ctx.node_voltage(nodes[10]);let eq30_e1299_q: f64 = s.v[446];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(5),
            &s.dn[446],
            &s.db[446],
            multiplicity,
        );let eq31_e1301_q: f64 = s.v[447];
        stamper.stamp_current_reactive_dense_local(
            Some(8),
            Some(5),
            &s.dn[447],
            &s.db[447],
            multiplicity,
        );let eq32_e1304_q: f64 = s.v[1251];let eq32_e1305: f64 = (s.v[36] * s.v[1251]);let eq32_e1305_q: f64 = (s.v[36] * eq32_e1304_q);
        stamper.stamp_current_reactive_dense_local(
            Some(9),
            Some(5),
            &s.dn[1251],
            &s.db[1251],
            (multiplicity) * (s.v[36]),
        );let eq33_e1308_q: f64 = s.v[1255];let eq33_e1309: f64 = (s.v[36] * s.v[1255]);let eq33_e1309_q: f64 = (s.v[36] * eq33_e1308_q);
        stamper.stamp_current_reactive_dense_local(
            Some(3),
            Some(5),
            &s.dn[1255],
            &s.db[1255],
            (multiplicity) * (s.v[36]),
        );let eq34_e1312_q: f64 = s.v[1244];let eq34_e1313: f64 = (s.v[36] * s.v[1244]);let eq34_e1313_q: f64 = (s.v[36] * eq34_e1312_q);
        stamper.stamp_current_reactive_dense_local(
            Some(12),
            Some(7),
            &s.dn[1244],
            &s.db[1244],
            (multiplicity) * (s.v[36]),
        );let eq35_e1316_q: f64 = s.v[1245];let eq35_e1317: f64 = (s.v[36] * s.v[1245]);let eq35_e1317_q: f64 = (s.v[36] * eq35_e1316_q);
        stamper.stamp_current_reactive_dense_local(
            Some(11),
            Some(8),
            &s.dn[1245],
            &s.db[1245],
            (multiplicity) * (s.v[36]),
        );
        let (eq36_e1324, eq36_e1324_d_n0, eq36_e1324_d_n1, eq36_e1324_d_n2, eq36_e1324_d_n3, eq36_e1324_d_n4, eq36_e1324_d_n5, eq36_e1324_d_n6, eq36_e1324_d_n7, eq36_e1324_d_n8, eq36_e1324_d_n9, eq36_e1324_d_n10, eq36_e1324_d_n11, eq36_e1324_d_n12, eq36_e1324_d_b0, eq36_e1324_d_b1, eq36_e1324_d_b2, eq36_e1324_d_b3, eq36_e1324_d_b4, eq36_e1324_d_b5, eq36_e1324_d_b6, eq36_e1324_d_b7, eq36_e1324_d_b8, eq36_e1324_q,) = {
    if s.b[1863] {
        let eq36_e1321_q: f64 = s.v[1230];let eq36_e1322: f64 = (s.v[36] * s.v[1230]);let eq36_e1322_q: f64 = (s.v[36] * eq36_e1321_q);
        (eq36_e1322, (s.v[36] * s.dn[1230][0]), (s.v[36] * s.dn[1230][1]), (s.v[36] * s.dn[1230][2]), (s.v[36] * s.dn[1230][3]), (s.v[36] * s.dn[1230][4]), (s.v[36] * s.dn[1230][5]), (s.v[36] * s.dn[1230][6]), (s.v[36] * s.dn[1230][7]), (s.v[36] * s.dn[1230][8]), (s.v[36] * s.dn[1230][9]), (s.v[36] * s.dn[1230][10]), (s.v[36] * s.dn[1230][11]), (s.v[36] * s.dn[1230][12]), (s.v[36] * s.db[1230][0]), (s.v[36] * s.db[1230][1]), (s.v[36] * s.db[1230][2]), (s.v[36] * s.db[1230][3]), (s.v[36] * s.db[1230][4]), (s.v[36] * s.db[1230][5]), (s.v[36] * s.db[1230][6]), (s.v[36] * s.db[1230][7]), (s.v[36] * s.db[1230][8]), eq36_e1322_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_reactive_node_derivatives: [f64; 13] = [eq36_e1324_d_n0, eq36_e1324_d_n1, eq36_e1324_d_n2, eq36_e1324_d_n3, eq36_e1324_d_n4, eq36_e1324_d_n5, eq36_e1324_d_n6, eq36_e1324_d_n7, eq36_e1324_d_n8, eq36_e1324_d_n9, eq36_e1324_d_n10, eq36_e1324_d_n11, eq36_e1324_d_n12];let eq36_reactive_branch_derivatives: [f64; 9] = [eq36_e1324_d_b0, eq36_e1324_d_b1, eq36_e1324_d_b2, eq36_e1324_d_b3, eq36_e1324_d_b4, eq36_e1324_d_b5, eq36_e1324_d_b6, eq36_e1324_d_b7, eq36_e1324_d_b8];
        stamper.stamp_current_reactive_dense_local(
            Some(10),
            Some(7),
            &eq36_reactive_node_derivatives,
            &eq36_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq37_e1331, eq37_e1331_d_n0, eq37_e1331_d_n1, eq37_e1331_d_n2, eq37_e1331_d_n3, eq37_e1331_d_n4, eq37_e1331_d_n5, eq37_e1331_d_n6, eq37_e1331_d_n7, eq37_e1331_d_n8, eq37_e1331_d_n9, eq37_e1331_d_n10, eq37_e1331_d_n11, eq37_e1331_d_n12, eq37_e1331_d_b0, eq37_e1331_d_b1, eq37_e1331_d_b2, eq37_e1331_d_b3, eq37_e1331_d_b4, eq37_e1331_d_b5, eq37_e1331_d_b6, eq37_e1331_d_b7, eq37_e1331_d_b8, eq37_e1331_q,) = {
    if s.b[1863] {
        let eq37_e1328_q: f64 = s.v[1231];let eq37_e1329: f64 = (s.v[36] * s.v[1231]);let eq37_e1329_q: f64 = (s.v[36] * eq37_e1328_q);
        (eq37_e1329, (s.v[36] * s.dn[1231][0]), (s.v[36] * s.dn[1231][1]), (s.v[36] * s.dn[1231][2]), (s.v[36] * s.dn[1231][3]), (s.v[36] * s.dn[1231][4]), (s.v[36] * s.dn[1231][5]), (s.v[36] * s.dn[1231][6]), (s.v[36] * s.dn[1231][7]), (s.v[36] * s.dn[1231][8]), (s.v[36] * s.dn[1231][9]), (s.v[36] * s.dn[1231][10]), (s.v[36] * s.dn[1231][11]), (s.v[36] * s.dn[1231][12]), (s.v[36] * s.db[1231][0]), (s.v[36] * s.db[1231][1]), (s.v[36] * s.db[1231][2]), (s.v[36] * s.db[1231][3]), (s.v[36] * s.db[1231][4]), (s.v[36] * s.db[1231][5]), (s.v[36] * s.db[1231][6]), (s.v[36] * s.db[1231][7]), (s.v[36] * s.db[1231][8]), eq37_e1329_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_reactive_node_derivatives: [f64; 13] = [eq37_e1331_d_n0, eq37_e1331_d_n1, eq37_e1331_d_n2, eq37_e1331_d_n3, eq37_e1331_d_n4, eq37_e1331_d_n5, eq37_e1331_d_n6, eq37_e1331_d_n7, eq37_e1331_d_n8, eq37_e1331_d_n9, eq37_e1331_d_n10, eq37_e1331_d_n11, eq37_e1331_d_n12];let eq37_reactive_branch_derivatives: [f64; 9] = [eq37_e1331_d_b0, eq37_e1331_d_b1, eq37_e1331_d_b2, eq37_e1331_d_b3, eq37_e1331_d_b4, eq37_e1331_d_b5, eq37_e1331_d_b6, eq37_e1331_d_b7, eq37_e1331_d_b8];
        stamper.stamp_current_reactive_dense_local(
            Some(10),
            Some(8),
            &eq37_reactive_node_derivatives,
            &eq37_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq38_e1338, eq38_e1338_d_n0, eq38_e1338_d_n1, eq38_e1338_d_n2, eq38_e1338_d_n3, eq38_e1338_d_n4, eq38_e1338_d_n5, eq38_e1338_d_n6, eq38_e1338_d_n7, eq38_e1338_d_n8, eq38_e1338_d_n9, eq38_e1338_d_n10, eq38_e1338_d_n11, eq38_e1338_d_n12, eq38_e1338_d_b0, eq38_e1338_d_b1, eq38_e1338_d_b2, eq38_e1338_d_b3, eq38_e1338_d_b4, eq38_e1338_d_b5, eq38_e1338_d_b6, eq38_e1338_d_b7, eq38_e1338_d_b8, eq38_e1338_q,) = {
    if s.b[1863] {
        let eq38_e1335: f64 = ((nv10 - nv3) * s.v[697]);let eq38_e1335_d_n0: f64 = ((nv10 - nv3) * s.dn[697][0]);let eq38_e1335_d_n1: f64 = ((nv10 - nv3) * s.dn[697][1]);let eq38_e1335_d_n2: f64 = ((nv10 - nv3) * s.dn[697][2]);let eq38_e1335_d_n3: f64 = ((-s.v[697]) + ((nv10 - nv3) * s.dn[697][3]));let eq38_e1335_d_n4: f64 = ((nv10 - nv3) * s.dn[697][4]);let eq38_e1335_d_n5: f64 = ((nv10 - nv3) * s.dn[697][5]);let eq38_e1335_d_n6: f64 = ((nv10 - nv3) * s.dn[697][6]);let eq38_e1335_d_n7: f64 = ((nv10 - nv3) * s.dn[697][7]);let eq38_e1335_d_n8: f64 = ((nv10 - nv3) * s.dn[697][8]);let eq38_e1335_d_n9: f64 = ((nv10 - nv3) * s.dn[697][9]);let eq38_e1335_d_n10: f64 = (s.v[697] + ((nv10 - nv3) * s.dn[697][10]));let eq38_e1335_d_n11: f64 = ((nv10 - nv3) * s.dn[697][11]);let eq38_e1335_d_n12: f64 = ((nv10 - nv3) * s.dn[697][12]);let eq38_e1335_d_b0: f64 = ((nv10 - nv3) * s.db[697][0]);let eq38_e1335_d_b1: f64 = ((nv10 - nv3) * s.db[697][1]);let eq38_e1335_d_b2: f64 = ((nv10 - nv3) * s.db[697][2]);let eq38_e1335_d_b3: f64 = ((nv10 - nv3) * s.db[697][3]);let eq38_e1335_d_b4: f64 = ((nv10 - nv3) * s.db[697][4]);let eq38_e1335_d_b5: f64 = ((nv10 - nv3) * s.db[697][5]);let eq38_e1335_d_b6: f64 = ((nv10 - nv3) * s.db[697][6]);let eq38_e1335_d_b7: f64 = ((nv10 - nv3) * s.db[697][7]);let eq38_e1335_d_b8: f64 = ((nv10 - nv3) * s.db[697][8]);let eq38_e1336_q: f64 = eq38_e1335;
        (eq38_e1335, eq38_e1335_d_n0, eq38_e1335_d_n1, eq38_e1335_d_n2, eq38_e1335_d_n3, eq38_e1335_d_n4, eq38_e1335_d_n5, eq38_e1335_d_n6, eq38_e1335_d_n7, eq38_e1335_d_n8, eq38_e1335_d_n9, eq38_e1335_d_n10, eq38_e1335_d_n11, eq38_e1335_d_n12, eq38_e1335_d_b0, eq38_e1335_d_b1, eq38_e1335_d_b2, eq38_e1335_d_b3, eq38_e1335_d_b4, eq38_e1335_d_b5, eq38_e1335_d_b6, eq38_e1335_d_b7, eq38_e1335_d_b8, eq38_e1336_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_reactive_node_derivatives: [f64; 13] = [eq38_e1338_d_n0, eq38_e1338_d_n1, eq38_e1338_d_n2, eq38_e1338_d_n3, eq38_e1338_d_n4, eq38_e1338_d_n5, eq38_e1338_d_n6, eq38_e1338_d_n7, eq38_e1338_d_n8, eq38_e1338_d_n9, eq38_e1338_d_n10, eq38_e1338_d_n11, eq38_e1338_d_n12];let eq38_reactive_branch_derivatives: [f64; 9] = [eq38_e1338_d_b0, eq38_e1338_d_b1, eq38_e1338_d_b2, eq38_e1338_d_b3, eq38_e1338_d_b4, eq38_e1338_d_b5, eq38_e1338_d_b6, eq38_e1338_d_b7, eq38_e1338_d_b8];
        stamper.stamp_current_reactive_dense_local(
            Some(10),
            Some(3),
            &eq38_reactive_node_derivatives,
            &eq38_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq39_e1346, eq39_e1346_d_n0, eq39_e1346_d_n1, eq39_e1346_d_n2, eq39_e1346_d_n3, eq39_e1346_d_n4, eq39_e1346_d_n5, eq39_e1346_d_n6, eq39_e1346_d_n7, eq39_e1346_d_n8, eq39_e1346_d_n9, eq39_e1346_d_n10, eq39_e1346_d_n11, eq39_e1346_d_n12, eq39_e1346_d_b0, eq39_e1346_d_b1, eq39_e1346_d_b2, eq39_e1346_d_b3, eq39_e1346_d_b4, eq39_e1346_d_b5, eq39_e1346_d_b6, eq39_e1346_d_b7, eq39_e1346_d_b8, eq39_e1346_q,) = {
    if (!s.b[1863]) {
        let eq39_e1343_q: f64 = s.v[1230];let eq39_e1344: f64 = (s.v[36] * s.v[1230]);let eq39_e1344_q: f64 = (s.v[36] * eq39_e1343_q);
        (eq39_e1344, (s.v[36] * s.dn[1230][0]), (s.v[36] * s.dn[1230][1]), (s.v[36] * s.dn[1230][2]), (s.v[36] * s.dn[1230][3]), (s.v[36] * s.dn[1230][4]), (s.v[36] * s.dn[1230][5]), (s.v[36] * s.dn[1230][6]), (s.v[36] * s.dn[1230][7]), (s.v[36] * s.dn[1230][8]), (s.v[36] * s.dn[1230][9]), (s.v[36] * s.dn[1230][10]), (s.v[36] * s.dn[1230][11]), (s.v[36] * s.dn[1230][12]), (s.v[36] * s.db[1230][0]), (s.v[36] * s.db[1230][1]), (s.v[36] * s.db[1230][2]), (s.v[36] * s.db[1230][3]), (s.v[36] * s.db[1230][4]), (s.v[36] * s.db[1230][5]), (s.v[36] * s.db[1230][6]), (s.v[36] * s.db[1230][7]), (s.v[36] * s.db[1230][8]), eq39_e1344_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_reactive_node_derivatives: [f64; 13] = [eq39_e1346_d_n0, eq39_e1346_d_n1, eq39_e1346_d_n2, eq39_e1346_d_n3, eq39_e1346_d_n4, eq39_e1346_d_n5, eq39_e1346_d_n6, eq39_e1346_d_n7, eq39_e1346_d_n8, eq39_e1346_d_n9, eq39_e1346_d_n10, eq39_e1346_d_n11, eq39_e1346_d_n12];let eq39_reactive_branch_derivatives: [f64; 9] = [eq39_e1346_d_b0, eq39_e1346_d_b1, eq39_e1346_d_b2, eq39_e1346_d_b3, eq39_e1346_d_b4, eq39_e1346_d_b5, eq39_e1346_d_b6, eq39_e1346_d_b7, eq39_e1346_d_b8];
        stamper.stamp_current_reactive_dense_local(
            Some(9),
            Some(7),
            &eq39_reactive_node_derivatives,
            &eq39_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq40_e1354, eq40_e1354_d_n0, eq40_e1354_d_n1, eq40_e1354_d_n2, eq40_e1354_d_n3, eq40_e1354_d_n4, eq40_e1354_d_n5, eq40_e1354_d_n6, eq40_e1354_d_n7, eq40_e1354_d_n8, eq40_e1354_d_n9, eq40_e1354_d_n10, eq40_e1354_d_n11, eq40_e1354_d_n12, eq40_e1354_d_b0, eq40_e1354_d_b1, eq40_e1354_d_b2, eq40_e1354_d_b3, eq40_e1354_d_b4, eq40_e1354_d_b5, eq40_e1354_d_b6, eq40_e1354_d_b7, eq40_e1354_d_b8, eq40_e1354_q,) = {
    if (!s.b[1863]) {
        let eq40_e1351_q: f64 = s.v[1231];let eq40_e1352: f64 = (s.v[36] * s.v[1231]);let eq40_e1352_q: f64 = (s.v[36] * eq40_e1351_q);
        (eq40_e1352, (s.v[36] * s.dn[1231][0]), (s.v[36] * s.dn[1231][1]), (s.v[36] * s.dn[1231][2]), (s.v[36] * s.dn[1231][3]), (s.v[36] * s.dn[1231][4]), (s.v[36] * s.dn[1231][5]), (s.v[36] * s.dn[1231][6]), (s.v[36] * s.dn[1231][7]), (s.v[36] * s.dn[1231][8]), (s.v[36] * s.dn[1231][9]), (s.v[36] * s.dn[1231][10]), (s.v[36] * s.dn[1231][11]), (s.v[36] * s.dn[1231][12]), (s.v[36] * s.db[1231][0]), (s.v[36] * s.db[1231][1]), (s.v[36] * s.db[1231][2]), (s.v[36] * s.db[1231][3]), (s.v[36] * s.db[1231][4]), (s.v[36] * s.db[1231][5]), (s.v[36] * s.db[1231][6]), (s.v[36] * s.db[1231][7]), (s.v[36] * s.db[1231][8]), eq40_e1352_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_reactive_node_derivatives: [f64; 13] = [eq40_e1354_d_n0, eq40_e1354_d_n1, eq40_e1354_d_n2, eq40_e1354_d_n3, eq40_e1354_d_n4, eq40_e1354_d_n5, eq40_e1354_d_n6, eq40_e1354_d_n7, eq40_e1354_d_n8, eq40_e1354_d_n9, eq40_e1354_d_n10, eq40_e1354_d_n11, eq40_e1354_d_n12];let eq40_reactive_branch_derivatives: [f64; 9] = [eq40_e1354_d_b0, eq40_e1354_d_b1, eq40_e1354_d_b2, eq40_e1354_d_b3, eq40_e1354_d_b4, eq40_e1354_d_b5, eq40_e1354_d_b6, eq40_e1354_d_b7, eq40_e1354_d_b8];
        stamper.stamp_current_reactive_dense_local(
            Some(9),
            Some(8),
            &eq40_reactive_node_derivatives,
            &eq40_reactive_branch_derivatives,
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
        let nv3 = ctx.node_voltage(nodes[3]);let nv9 = ctx.node_voltage(nodes[9]);
        let (eq41_e1362, eq41_e1362_d_n0, eq41_e1362_d_n1, eq41_e1362_d_n2, eq41_e1362_d_n3, eq41_e1362_d_n4, eq41_e1362_d_n5, eq41_e1362_d_n6, eq41_e1362_d_n7, eq41_e1362_d_n8, eq41_e1362_d_n9, eq41_e1362_d_n10, eq41_e1362_d_n11, eq41_e1362_d_n12, eq41_e1362_d_b0, eq41_e1362_d_b1, eq41_e1362_d_b2, eq41_e1362_d_b3, eq41_e1362_d_b4, eq41_e1362_d_b5, eq41_e1362_d_b6, eq41_e1362_d_b7, eq41_e1362_d_b8, eq41_e1362_q,) = {
    if (!s.b[1863]) {
        let eq41_e1359: f64 = ((nv9 - nv3) * s.v[697]);let eq41_e1359_d_n0: f64 = ((nv9 - nv3) * s.dn[697][0]);let eq41_e1359_d_n1: f64 = ((nv9 - nv3) * s.dn[697][1]);let eq41_e1359_d_n2: f64 = ((nv9 - nv3) * s.dn[697][2]);let eq41_e1359_d_n3: f64 = ((-s.v[697]) + ((nv9 - nv3) * s.dn[697][3]));let eq41_e1359_d_n4: f64 = ((nv9 - nv3) * s.dn[697][4]);let eq41_e1359_d_n5: f64 = ((nv9 - nv3) * s.dn[697][5]);let eq41_e1359_d_n6: f64 = ((nv9 - nv3) * s.dn[697][6]);let eq41_e1359_d_n7: f64 = ((nv9 - nv3) * s.dn[697][7]);let eq41_e1359_d_n8: f64 = ((nv9 - nv3) * s.dn[697][8]);let eq41_e1359_d_n9: f64 = (s.v[697] + ((nv9 - nv3) * s.dn[697][9]));let eq41_e1359_d_n10: f64 = ((nv9 - nv3) * s.dn[697][10]);let eq41_e1359_d_n11: f64 = ((nv9 - nv3) * s.dn[697][11]);let eq41_e1359_d_n12: f64 = ((nv9 - nv3) * s.dn[697][12]);let eq41_e1359_d_b0: f64 = ((nv9 - nv3) * s.db[697][0]);let eq41_e1359_d_b1: f64 = ((nv9 - nv3) * s.db[697][1]);let eq41_e1359_d_b2: f64 = ((nv9 - nv3) * s.db[697][2]);let eq41_e1359_d_b3: f64 = ((nv9 - nv3) * s.db[697][3]);let eq41_e1359_d_b4: f64 = ((nv9 - nv3) * s.db[697][4]);let eq41_e1359_d_b5: f64 = ((nv9 - nv3) * s.db[697][5]);let eq41_e1359_d_b6: f64 = ((nv9 - nv3) * s.db[697][6]);let eq41_e1359_d_b7: f64 = ((nv9 - nv3) * s.db[697][7]);let eq41_e1359_d_b8: f64 = ((nv9 - nv3) * s.db[697][8]);let eq41_e1360_q: f64 = eq41_e1359;
        (eq41_e1359, eq41_e1359_d_n0, eq41_e1359_d_n1, eq41_e1359_d_n2, eq41_e1359_d_n3, eq41_e1359_d_n4, eq41_e1359_d_n5, eq41_e1359_d_n6, eq41_e1359_d_n7, eq41_e1359_d_n8, eq41_e1359_d_n9, eq41_e1359_d_n10, eq41_e1359_d_n11, eq41_e1359_d_n12, eq41_e1359_d_b0, eq41_e1359_d_b1, eq41_e1359_d_b2, eq41_e1359_d_b3, eq41_e1359_d_b4, eq41_e1359_d_b5, eq41_e1359_d_b6, eq41_e1359_d_b7, eq41_e1359_d_b8, eq41_e1360_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_reactive_node_derivatives: [f64; 13] = [eq41_e1362_d_n0, eq41_e1362_d_n1, eq41_e1362_d_n2, eq41_e1362_d_n3, eq41_e1362_d_n4, eq41_e1362_d_n5, eq41_e1362_d_n6, eq41_e1362_d_n7, eq41_e1362_d_n8, eq41_e1362_d_n9, eq41_e1362_d_n10, eq41_e1362_d_n11, eq41_e1362_d_n12];let eq41_reactive_branch_derivatives: [f64; 9] = [eq41_e1362_d_b0, eq41_e1362_d_b1, eq41_e1362_d_b2, eq41_e1362_d_b3, eq41_e1362_d_b4, eq41_e1362_d_b5, eq41_e1362_d_b6, eq41_e1362_d_b7, eq41_e1362_d_b8];
        stamper.stamp_current_reactive_dense_local(
            Some(9),
            Some(3),
            &eq41_reactive_node_derivatives,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );let eq42_e1364_q: f64 = s.v[449];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(3),
            &s.dn[449],
            &s.db[449],
            multiplicity,
        );let eq43_e1366_q: f64 = s.v[448];
        stamper.stamp_current_reactive_dense_local(
            Some(8),
            Some(3),
            &s.dn[448],
            &s.db[448],
            multiplicity,
        );
        let (eq58_e1476, eq58_e1476_d_n0, eq58_e1476_d_n1, eq58_e1476_d_n2, eq58_e1476_d_n3, eq58_e1476_d_n4, eq58_e1476_d_n5, eq58_e1476_d_n6, eq58_e1476_d_n7, eq58_e1476_d_n8, eq58_e1476_d_n9, eq58_e1476_d_n10, eq58_e1476_d_n11, eq58_e1476_d_n12, eq58_e1476_d_b0, eq58_e1476_d_b1, eq58_e1476_d_b2, eq58_e1476_d_b3, eq58_e1476_d_b4, eq58_e1476_d_b5, eq58_e1476_d_b6, eq58_e1476_d_b7, eq58_e1476_d_b8, eq58_e1476_q,) = {
    if s.b[1869] {
        let eq58_e1473: f64 = (s.v[770] * s.v[528]);let eq58_e1474_q: f64 = eq58_e1473;
        (eq58_e1473, (s.dn[770][0] * s.v[528]), (s.dn[770][1] * s.v[528]), (s.dn[770][2] * s.v[528]), (s.dn[770][3] * s.v[528]), (s.dn[770][4] * s.v[528]), (s.dn[770][5] * s.v[528]), (s.dn[770][6] * s.v[528]), (s.dn[770][7] * s.v[528]), (s.dn[770][8] * s.v[528]), (s.dn[770][9] * s.v[528]), (s.dn[770][10] * s.v[528]), (s.dn[770][11] * s.v[528]), (s.dn[770][12] * s.v[528]), (s.db[770][0] * s.v[528]), (s.db[770][1] * s.v[528]), (s.db[770][2] * s.v[528]), (s.db[770][3] * s.v[528]), (s.db[770][4] * s.v[528]), (s.db[770][5] * s.v[528]), (s.db[770][6] * s.v[528]), (s.db[770][7] * s.v[528]), (s.db[770][8] * s.v[528]), eq58_e1474_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq58_reactive_node_derivatives: [f64; 13] = [eq58_e1476_d_n0, eq58_e1476_d_n1, eq58_e1476_d_n2, eq58_e1476_d_n3, eq58_e1476_d_n4, eq58_e1476_d_n5, eq58_e1476_d_n6, eq58_e1476_d_n7, eq58_e1476_d_n8, eq58_e1476_d_n9, eq58_e1476_d_n10, eq58_e1476_d_n11, eq58_e1476_d_n12];let eq58_reactive_branch_derivatives: [f64; 9] = [eq58_e1476_d_b0, eq58_e1476_d_b1, eq58_e1476_d_b2, eq58_e1476_d_b3, eq58_e1476_d_b4, eq58_e1476_d_b5, eq58_e1476_d_b6, eq58_e1476_d_b7, eq58_e1476_d_b8];
        stamper.stamp_current_reactive_dense_local(
            Some(6),
            None,
            &eq58_reactive_node_derivatives,
            &eq58_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
