#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_7(
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
        let (eq17_e1115, eq17_e1115_d_n0, eq17_e1115_d_n1, eq17_e1115_d_n2, eq17_e1115_d_n3, eq17_e1115_d_n4, eq17_e1115_d_n5, eq17_e1115_d_n6, eq17_e1115_d_n7, eq17_e1115_d_n8, eq17_e1115_d_n9, eq17_e1115_d_n10, eq17_e1115_d_n11, eq17_e1115_d_n12, eq17_e1115_d_n13, eq17_e1115_d_n14, eq17_e1115_d_n15, eq17_e1115_d_n16, eq17_e1115_d_n17, eq17_e1115_d_b0, eq17_e1115_d_b1, eq17_e1115_d_b2, eq17_e1115_d_b3, eq17_e1115_d_b4, eq17_e1115_d_b5, eq17_e1115_d_b6, eq17_e1115_d_b7, eq17_e1115_d_b8, eq17_e1115_d_b9, eq17_e1115_d_b10, eq17_e1115_d_b11,) = {
    if s.b[3405] {
        let eq17_e1113: f64 = (p.p87 * s.v[869]);
        (eq17_e1113, (p.p87 * s.dn[869][0]), (p.p87 * s.dn[869][1]), (p.p87 * s.dn[869][2]), (p.p87 * s.dn[869][3]), (p.p87 * s.dn[869][4]), (p.p87 * s.dn[869][5]), (p.p87 * s.dn[869][6]), (p.p87 * s.dn[869][7]), (p.p87 * s.dn[869][8]), (p.p87 * s.dn[869][9]), (p.p87 * s.dn[869][10]), (p.p87 * s.dn[869][11]), (p.p87 * s.dn[869][12]), (p.p87 * s.dn[869][13]), (p.p87 * s.dn[869][14]), (p.p87 * s.dn[869][15]), (p.p87 * s.dn[869][16]), (p.p87 * s.dn[869][17]), (p.p87 * s.db[869][0]), (p.p87 * s.db[869][1]), (p.p87 * s.db[869][2]), (p.p87 * s.db[869][3]), (p.p87 * s.db[869][4]), (p.p87 * s.db[869][5]), (p.p87 * s.db[869][6]), (p.p87 * s.db[869][7]), (p.p87 * s.db[869][8]), (p.p87 * s.db[869][9]), (p.p87 * s.db[869][10]), (p.p87 * s.db[869][11]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e1115;let eq17_node_derivatives: [f64; 18] = [eq17_e1115_d_n0, eq17_e1115_d_n1, eq17_e1115_d_n2, eq17_e1115_d_n3, eq17_e1115_d_n4, eq17_e1115_d_n5, eq17_e1115_d_n6, eq17_e1115_d_n7, eq17_e1115_d_n8, eq17_e1115_d_n9, eq17_e1115_d_n10, eq17_e1115_d_n11, eq17_e1115_d_n12, eq17_e1115_d_n13, eq17_e1115_d_n14, eq17_e1115_d_n15, eq17_e1115_d_n16, eq17_e1115_d_n17];let eq17_branch_derivatives: [f64; 12] = [eq17_e1115_d_b0, eq17_e1115_d_b1, eq17_e1115_d_b2, eq17_e1115_d_b3, eq17_e1115_d_b4, eq17_e1115_d_b5, eq17_e1115_d_b6, eq17_e1115_d_b7, eq17_e1115_d_b8, eq17_e1115_d_b9, eq17_e1115_d_b10, eq17_e1115_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let (eq18_e1122, eq18_e1122_d_n0, eq18_e1122_d_n1, eq18_e1122_d_n2, eq18_e1122_d_n3, eq18_e1122_d_n4, eq18_e1122_d_n5, eq18_e1122_d_n6, eq18_e1122_d_n7, eq18_e1122_d_n8, eq18_e1122_d_n9, eq18_e1122_d_n10, eq18_e1122_d_n11, eq18_e1122_d_n12, eq18_e1122_d_n13, eq18_e1122_d_n14, eq18_e1122_d_n15, eq18_e1122_d_n16, eq18_e1122_d_n17, eq18_e1122_d_b0, eq18_e1122_d_b1, eq18_e1122_d_b2, eq18_e1122_d_b3, eq18_e1122_d_b4, eq18_e1122_d_b5, eq18_e1122_d_b6, eq18_e1122_d_b7, eq18_e1122_d_b8, eq18_e1122_d_b9, eq18_e1122_d_b10, eq18_e1122_d_b11,) = {
    if s.b[3405] {
        let eq18_e1119: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, s.v[68]);let eq18_e1120: f64 = (p.p87 * eq18_e1119);let eq18_e1120_d_n0: f64 = (p.p87 * (s.dn[68][0] * ddt_scale));let eq18_e1120_d_n1: f64 = (p.p87 * (s.dn[68][1] * ddt_scale));let eq18_e1120_d_n2: f64 = (p.p87 * (s.dn[68][2] * ddt_scale));let eq18_e1120_d_n3: f64 = (p.p87 * (s.dn[68][3] * ddt_scale));let eq18_e1120_d_n4: f64 = (p.p87 * (s.dn[68][4] * ddt_scale));let eq18_e1120_d_n5: f64 = (p.p87 * (s.dn[68][5] * ddt_scale));let eq18_e1120_d_n6: f64 = (p.p87 * (s.dn[68][6] * ddt_scale));let eq18_e1120_d_n7: f64 = (p.p87 * (s.dn[68][7] * ddt_scale));let eq18_e1120_d_n8: f64 = (p.p87 * (s.dn[68][8] * ddt_scale));let eq18_e1120_d_n9: f64 = (p.p87 * (s.dn[68][9] * ddt_scale));let eq18_e1120_d_n10: f64 = (p.p87 * (s.dn[68][10] * ddt_scale));let eq18_e1120_d_n11: f64 = (p.p87 * (s.dn[68][11] * ddt_scale));let eq18_e1120_d_n12: f64 = (p.p87 * (s.dn[68][12] * ddt_scale));let eq18_e1120_d_n13: f64 = (p.p87 * (s.dn[68][13] * ddt_scale));let eq18_e1120_d_n14: f64 = (p.p87 * (s.dn[68][14] * ddt_scale));let eq18_e1120_d_n15: f64 = (p.p87 * (s.dn[68][15] * ddt_scale));let eq18_e1120_d_n16: f64 = (p.p87 * (s.dn[68][16] * ddt_scale));let eq18_e1120_d_n17: f64 = (p.p87 * (s.dn[68][17] * ddt_scale));let eq18_e1120_d_b0: f64 = (p.p87 * (s.db[68][0] * ddt_scale));let eq18_e1120_d_b1: f64 = (p.p87 * (s.db[68][1] * ddt_scale));let eq18_e1120_d_b2: f64 = (p.p87 * (s.db[68][2] * ddt_scale));let eq18_e1120_d_b3: f64 = (p.p87 * (s.db[68][3] * ddt_scale));let eq18_e1120_d_b4: f64 = (p.p87 * (s.db[68][4] * ddt_scale));let eq18_e1120_d_b5: f64 = (p.p87 * (s.db[68][5] * ddt_scale));let eq18_e1120_d_b6: f64 = (p.p87 * (s.db[68][6] * ddt_scale));let eq18_e1120_d_b7: f64 = (p.p87 * (s.db[68][7] * ddt_scale));let eq18_e1120_d_b8: f64 = (p.p87 * (s.db[68][8] * ddt_scale));let eq18_e1120_d_b9: f64 = (p.p87 * (s.db[68][9] * ddt_scale));let eq18_e1120_d_b10: f64 = (p.p87 * (s.db[68][10] * ddt_scale));let eq18_e1120_d_b11: f64 = (p.p87 * (s.db[68][11] * ddt_scale));
        (eq18_e1120, eq18_e1120_d_n0, eq18_e1120_d_n1, eq18_e1120_d_n2, eq18_e1120_d_n3, eq18_e1120_d_n4, eq18_e1120_d_n5, eq18_e1120_d_n6, eq18_e1120_d_n7, eq18_e1120_d_n8, eq18_e1120_d_n9, eq18_e1120_d_n10, eq18_e1120_d_n11, eq18_e1120_d_n12, eq18_e1120_d_n13, eq18_e1120_d_n14, eq18_e1120_d_n15, eq18_e1120_d_n16, eq18_e1120_d_n17, eq18_e1120_d_b0, eq18_e1120_d_b1, eq18_e1120_d_b2, eq18_e1120_d_b3, eq18_e1120_d_b4, eq18_e1120_d_b5, eq18_e1120_d_b6, eq18_e1120_d_b7, eq18_e1120_d_b8, eq18_e1120_d_b9, eq18_e1120_d_b10, eq18_e1120_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1122;let eq18_node_derivatives: [f64; 18] = [eq18_e1122_d_n0, eq18_e1122_d_n1, eq18_e1122_d_n2, eq18_e1122_d_n3, eq18_e1122_d_n4, eq18_e1122_d_n5, eq18_e1122_d_n6, eq18_e1122_d_n7, eq18_e1122_d_n8, eq18_e1122_d_n9, eq18_e1122_d_n10, eq18_e1122_d_n11, eq18_e1122_d_n12, eq18_e1122_d_n13, eq18_e1122_d_n14, eq18_e1122_d_n15, eq18_e1122_d_n16, eq18_e1122_d_n17];let eq18_branch_derivatives: [f64; 12] = [eq18_e1122_d_b0, eq18_e1122_d_b1, eq18_e1122_d_b2, eq18_e1122_d_b3, eq18_e1122_d_b4, eq18_e1122_d_b5, eq18_e1122_d_b6, eq18_e1122_d_b7, eq18_e1122_d_b8, eq18_e1122_d_b9, eq18_e1122_d_b10, eq18_e1122_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let (eq19_e1129, eq19_e1129_d_n0, eq19_e1129_d_n1, eq19_e1129_d_n2, eq19_e1129_d_n3, eq19_e1129_d_n4, eq19_e1129_d_n5, eq19_e1129_d_n6, eq19_e1129_d_n7, eq19_e1129_d_n8, eq19_e1129_d_n9, eq19_e1129_d_n10, eq19_e1129_d_n11, eq19_e1129_d_n12, eq19_e1129_d_n13, eq19_e1129_d_n14, eq19_e1129_d_n15, eq19_e1129_d_n16, eq19_e1129_d_n17, eq19_e1129_d_b0, eq19_e1129_d_b1, eq19_e1129_d_b2, eq19_e1129_d_b3, eq19_e1129_d_b4, eq19_e1129_d_b5, eq19_e1129_d_b6, eq19_e1129_d_b7, eq19_e1129_d_b8, eq19_e1129_d_b9, eq19_e1129_d_b10, eq19_e1129_d_b11,) = {
    if s.b[3405] {
        let eq19_e1126: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, s.v[67]);let eq19_e1127: f64 = (p.p87 * eq19_e1126);let eq19_e1127_d_n0: f64 = (p.p87 * (s.dn[67][0] * ddt_scale));let eq19_e1127_d_n1: f64 = (p.p87 * (s.dn[67][1] * ddt_scale));let eq19_e1127_d_n2: f64 = (p.p87 * (s.dn[67][2] * ddt_scale));let eq19_e1127_d_n3: f64 = (p.p87 * (s.dn[67][3] * ddt_scale));let eq19_e1127_d_n4: f64 = (p.p87 * (s.dn[67][4] * ddt_scale));let eq19_e1127_d_n5: f64 = (p.p87 * (s.dn[67][5] * ddt_scale));let eq19_e1127_d_n6: f64 = (p.p87 * (s.dn[67][6] * ddt_scale));let eq19_e1127_d_n7: f64 = (p.p87 * (s.dn[67][7] * ddt_scale));let eq19_e1127_d_n8: f64 = (p.p87 * (s.dn[67][8] * ddt_scale));let eq19_e1127_d_n9: f64 = (p.p87 * (s.dn[67][9] * ddt_scale));let eq19_e1127_d_n10: f64 = (p.p87 * (s.dn[67][10] * ddt_scale));let eq19_e1127_d_n11: f64 = (p.p87 * (s.dn[67][11] * ddt_scale));let eq19_e1127_d_n12: f64 = (p.p87 * (s.dn[67][12] * ddt_scale));let eq19_e1127_d_n13: f64 = (p.p87 * (s.dn[67][13] * ddt_scale));let eq19_e1127_d_n14: f64 = (p.p87 * (s.dn[67][14] * ddt_scale));let eq19_e1127_d_n15: f64 = (p.p87 * (s.dn[67][15] * ddt_scale));let eq19_e1127_d_n16: f64 = (p.p87 * (s.dn[67][16] * ddt_scale));let eq19_e1127_d_n17: f64 = (p.p87 * (s.dn[67][17] * ddt_scale));let eq19_e1127_d_b0: f64 = (p.p87 * (s.db[67][0] * ddt_scale));let eq19_e1127_d_b1: f64 = (p.p87 * (s.db[67][1] * ddt_scale));let eq19_e1127_d_b2: f64 = (p.p87 * (s.db[67][2] * ddt_scale));let eq19_e1127_d_b3: f64 = (p.p87 * (s.db[67][3] * ddt_scale));let eq19_e1127_d_b4: f64 = (p.p87 * (s.db[67][4] * ddt_scale));let eq19_e1127_d_b5: f64 = (p.p87 * (s.db[67][5] * ddt_scale));let eq19_e1127_d_b6: f64 = (p.p87 * (s.db[67][6] * ddt_scale));let eq19_e1127_d_b7: f64 = (p.p87 * (s.db[67][7] * ddt_scale));let eq19_e1127_d_b8: f64 = (p.p87 * (s.db[67][8] * ddt_scale));let eq19_e1127_d_b9: f64 = (p.p87 * (s.db[67][9] * ddt_scale));let eq19_e1127_d_b10: f64 = (p.p87 * (s.db[67][10] * ddt_scale));let eq19_e1127_d_b11: f64 = (p.p87 * (s.db[67][11] * ddt_scale));
        (eq19_e1127, eq19_e1127_d_n0, eq19_e1127_d_n1, eq19_e1127_d_n2, eq19_e1127_d_n3, eq19_e1127_d_n4, eq19_e1127_d_n5, eq19_e1127_d_n6, eq19_e1127_d_n7, eq19_e1127_d_n8, eq19_e1127_d_n9, eq19_e1127_d_n10, eq19_e1127_d_n11, eq19_e1127_d_n12, eq19_e1127_d_n13, eq19_e1127_d_n14, eq19_e1127_d_n15, eq19_e1127_d_n16, eq19_e1127_d_n17, eq19_e1127_d_b0, eq19_e1127_d_b1, eq19_e1127_d_b2, eq19_e1127_d_b3, eq19_e1127_d_b4, eq19_e1127_d_b5, eq19_e1127_d_b6, eq19_e1127_d_b7, eq19_e1127_d_b8, eq19_e1127_d_b9, eq19_e1127_d_b10, eq19_e1127_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_value: f64 = eq19_e1129;let eq19_node_derivatives: [f64; 18] = [eq19_e1129_d_n0, eq19_e1129_d_n1, eq19_e1129_d_n2, eq19_e1129_d_n3, eq19_e1129_d_n4, eq19_e1129_d_n5, eq19_e1129_d_n6, eq19_e1129_d_n7, eq19_e1129_d_n8, eq19_e1129_d_n9, eq19_e1129_d_n10, eq19_e1129_d_n11, eq19_e1129_d_n12, eq19_e1129_d_n13, eq19_e1129_d_n14, eq19_e1129_d_n15, eq19_e1129_d_n16, eq19_e1129_d_n17];let eq19_branch_derivatives: [f64; 12] = [eq19_e1129_d_b0, eq19_e1129_d_b1, eq19_e1129_d_b2, eq19_e1129_d_b3, eq19_e1129_d_b4, eq19_e1129_d_b5, eq19_e1129_d_b6, eq19_e1129_d_b7, eq19_e1129_d_b8, eq19_e1129_d_b9, eq19_e1129_d_b10, eq19_e1129_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let (eq20_e1135, eq20_e1135_d_n0, eq20_e1135_d_n1, eq20_e1135_d_n2, eq20_e1135_d_n3, eq20_e1135_d_n4, eq20_e1135_d_n5, eq20_e1135_d_n6, eq20_e1135_d_n7, eq20_e1135_d_n8, eq20_e1135_d_n9, eq20_e1135_d_n10, eq20_e1135_d_n11, eq20_e1135_d_n12, eq20_e1135_d_n13, eq20_e1135_d_n14, eq20_e1135_d_n15, eq20_e1135_d_n16, eq20_e1135_d_n17, eq20_e1135_d_b0, eq20_e1135_d_b1, eq20_e1135_d_b2, eq20_e1135_d_b3, eq20_e1135_d_b4, eq20_e1135_d_b5, eq20_e1135_d_b6, eq20_e1135_d_b7, eq20_e1135_d_b8, eq20_e1135_d_b9, eq20_e1135_d_b10, eq20_e1135_d_b11,) = {
    if s.b[3406] {
        let eq20_e1133: f64 = (p.p87 * s.v[200]);
        (eq20_e1133, (p.p87 * s.dn[200][0]), (p.p87 * s.dn[200][1]), (p.p87 * s.dn[200][2]), (p.p87 * s.dn[200][3]), (p.p87 * s.dn[200][4]), (p.p87 * s.dn[200][5]), (p.p87 * s.dn[200][6]), (p.p87 * s.dn[200][7]), (p.p87 * s.dn[200][8]), (p.p87 * s.dn[200][9]), (p.p87 * s.dn[200][10]), (p.p87 * s.dn[200][11]), (p.p87 * s.dn[200][12]), (p.p87 * s.dn[200][13]), (p.p87 * s.dn[200][14]), (p.p87 * s.dn[200][15]), (p.p87 * s.dn[200][16]), (p.p87 * s.dn[200][17]), (p.p87 * s.db[200][0]), (p.p87 * s.db[200][1]), (p.p87 * s.db[200][2]), (p.p87 * s.db[200][3]), (p.p87 * s.db[200][4]), (p.p87 * s.db[200][5]), (p.p87 * s.db[200][6]), (p.p87 * s.db[200][7]), (p.p87 * s.db[200][8]), (p.p87 * s.db[200][9]), (p.p87 * s.db[200][10]), (p.p87 * s.db[200][11]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e1135;let eq20_node_derivatives: [f64; 18] = [eq20_e1135_d_n0, eq20_e1135_d_n1, eq20_e1135_d_n2, eq20_e1135_d_n3, eq20_e1135_d_n4, eq20_e1135_d_n5, eq20_e1135_d_n6, eq20_e1135_d_n7, eq20_e1135_d_n8, eq20_e1135_d_n9, eq20_e1135_d_n10, eq20_e1135_d_n11, eq20_e1135_d_n12, eq20_e1135_d_n13, eq20_e1135_d_n14, eq20_e1135_d_n15, eq20_e1135_d_n16, eq20_e1135_d_n17];let eq20_branch_derivatives: [f64; 12] = [eq20_e1135_d_b0, eq20_e1135_d_b1, eq20_e1135_d_b2, eq20_e1135_d_b3, eq20_e1135_d_b4, eq20_e1135_d_b5, eq20_e1135_d_b6, eq20_e1135_d_b7, eq20_e1135_d_b8, eq20_e1135_d_b9, eq20_e1135_d_b10, eq20_e1135_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &eq20_branch_derivatives,
            multiplicity,
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
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);let nv5 = ctx.node_voltage(nodes[5]);let nv7 = ctx.node_voltage(nodes[7]);
        let (eq21_e1141, eq21_e1141_d_n0, eq21_e1141_d_n1, eq21_e1141_d_n2, eq21_e1141_d_n3, eq21_e1141_d_n4, eq21_e1141_d_n5, eq21_e1141_d_n6, eq21_e1141_d_n7, eq21_e1141_d_n8, eq21_e1141_d_n9, eq21_e1141_d_n10, eq21_e1141_d_n11, eq21_e1141_d_n12, eq21_e1141_d_n13, eq21_e1141_d_n14, eq21_e1141_d_n15, eq21_e1141_d_n16, eq21_e1141_d_n17, eq21_e1141_d_b0, eq21_e1141_d_b1, eq21_e1141_d_b2, eq21_e1141_d_b3, eq21_e1141_d_b4, eq21_e1141_d_b5, eq21_e1141_d_b6, eq21_e1141_d_b7, eq21_e1141_d_b8, eq21_e1141_d_b9, eq21_e1141_d_b10, eq21_e1141_d_b11,) = {
    if s.b[3406] {
        let eq21_e1139: f64 = (p.p87 * s.v[201]);
        (eq21_e1139, (p.p87 * s.dn[201][0]), (p.p87 * s.dn[201][1]), (p.p87 * s.dn[201][2]), (p.p87 * s.dn[201][3]), (p.p87 * s.dn[201][4]), (p.p87 * s.dn[201][5]), (p.p87 * s.dn[201][6]), (p.p87 * s.dn[201][7]), (p.p87 * s.dn[201][8]), (p.p87 * s.dn[201][9]), (p.p87 * s.dn[201][10]), (p.p87 * s.dn[201][11]), (p.p87 * s.dn[201][12]), (p.p87 * s.dn[201][13]), (p.p87 * s.dn[201][14]), (p.p87 * s.dn[201][15]), (p.p87 * s.dn[201][16]), (p.p87 * s.dn[201][17]), (p.p87 * s.db[201][0]), (p.p87 * s.db[201][1]), (p.p87 * s.db[201][2]), (p.p87 * s.db[201][3]), (p.p87 * s.db[201][4]), (p.p87 * s.db[201][5]), (p.p87 * s.db[201][6]), (p.p87 * s.db[201][7]), (p.p87 * s.db[201][8]), (p.p87 * s.db[201][9]), (p.p87 * s.db[201][10]), (p.p87 * s.db[201][11]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1141;let eq21_node_derivatives: [f64; 18] = [eq21_e1141_d_n0, eq21_e1141_d_n1, eq21_e1141_d_n2, eq21_e1141_d_n3, eq21_e1141_d_n4, eq21_e1141_d_n5, eq21_e1141_d_n6, eq21_e1141_d_n7, eq21_e1141_d_n8, eq21_e1141_d_n9, eq21_e1141_d_n10, eq21_e1141_d_n11, eq21_e1141_d_n12, eq21_e1141_d_n13, eq21_e1141_d_n14, eq21_e1141_d_n15, eq21_e1141_d_n16, eq21_e1141_d_n17];let eq21_branch_derivatives: [f64; 12] = [eq21_e1141_d_b0, eq21_e1141_d_b1, eq21_e1141_d_b2, eq21_e1141_d_b3, eq21_e1141_d_b4, eq21_e1141_d_b5, eq21_e1141_d_b6, eq21_e1141_d_b7, eq21_e1141_d_b8, eq21_e1141_d_b9, eq21_e1141_d_b10, eq21_e1141_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let (eq22_e1147, eq22_e1147_d_n0, eq22_e1147_d_n1, eq22_e1147_d_n2, eq22_e1147_d_n3, eq22_e1147_d_n4, eq22_e1147_d_n5, eq22_e1147_d_n6, eq22_e1147_d_n7, eq22_e1147_d_n8, eq22_e1147_d_n9, eq22_e1147_d_n10, eq22_e1147_d_n11, eq22_e1147_d_n12, eq22_e1147_d_n13, eq22_e1147_d_n14, eq22_e1147_d_n15, eq22_e1147_d_n16, eq22_e1147_d_n17, eq22_e1147_d_b0, eq22_e1147_d_b1, eq22_e1147_d_b2, eq22_e1147_d_b3, eq22_e1147_d_b4, eq22_e1147_d_b5, eq22_e1147_d_b6, eq22_e1147_d_b7, eq22_e1147_d_b8, eq22_e1147_d_b9, eq22_e1147_d_b10, eq22_e1147_d_b11,) = {
    if s.b[3406] {
        let eq22_e1145: f64 = (p.p87 * s.v[202]);
        (eq22_e1145, (p.p87 * s.dn[202][0]), (p.p87 * s.dn[202][1]), (p.p87 * s.dn[202][2]), (p.p87 * s.dn[202][3]), (p.p87 * s.dn[202][4]), (p.p87 * s.dn[202][5]), (p.p87 * s.dn[202][6]), (p.p87 * s.dn[202][7]), (p.p87 * s.dn[202][8]), (p.p87 * s.dn[202][9]), (p.p87 * s.dn[202][10]), (p.p87 * s.dn[202][11]), (p.p87 * s.dn[202][12]), (p.p87 * s.dn[202][13]), (p.p87 * s.dn[202][14]), (p.p87 * s.dn[202][15]), (p.p87 * s.dn[202][16]), (p.p87 * s.dn[202][17]), (p.p87 * s.db[202][0]), (p.p87 * s.db[202][1]), (p.p87 * s.db[202][2]), (p.p87 * s.db[202][3]), (p.p87 * s.db[202][4]), (p.p87 * s.db[202][5]), (p.p87 * s.db[202][6]), (p.p87 * s.db[202][7]), (p.p87 * s.db[202][8]), (p.p87 * s.db[202][9]), (p.p87 * s.db[202][10]), (p.p87 * s.db[202][11]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq22_value: f64 = eq22_e1147;let eq22_node_derivatives: [f64; 18] = [eq22_e1147_d_n0, eq22_e1147_d_n1, eq22_e1147_d_n2, eq22_e1147_d_n3, eq22_e1147_d_n4, eq22_e1147_d_n5, eq22_e1147_d_n6, eq22_e1147_d_n7, eq22_e1147_d_n8, eq22_e1147_d_n9, eq22_e1147_d_n10, eq22_e1147_d_n11, eq22_e1147_d_n12, eq22_e1147_d_n13, eq22_e1147_d_n14, eq22_e1147_d_n15, eq22_e1147_d_n16, eq22_e1147_d_n17];let eq22_branch_derivatives: [f64; 12] = [eq22_e1147_d_b0, eq22_e1147_d_b1, eq22_e1147_d_b2, eq22_e1147_d_b3, eq22_e1147_d_b4, eq22_e1147_d_b5, eq22_e1147_d_b6, eq22_e1147_d_b7, eq22_e1147_d_b8, eq22_e1147_d_b9, eq22_e1147_d_b10, eq22_e1147_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq22_value),
            &eq22_node_derivatives,
            &eq22_branch_derivatives,
            multiplicity,
        );
        let (eq23_e1153, eq23_e1153_d_n0, eq23_e1153_d_n1, eq23_e1153_d_n2, eq23_e1153_d_n3, eq23_e1153_d_n4, eq23_e1153_d_n5, eq23_e1153_d_n6, eq23_e1153_d_n7, eq23_e1153_d_n8, eq23_e1153_d_n9, eq23_e1153_d_n10, eq23_e1153_d_n11, eq23_e1153_d_n12, eq23_e1153_d_n13, eq23_e1153_d_n14, eq23_e1153_d_n15, eq23_e1153_d_n16, eq23_e1153_d_n17, eq23_e1153_d_b0, eq23_e1153_d_b1, eq23_e1153_d_b2, eq23_e1153_d_b3, eq23_e1153_d_b4, eq23_e1153_d_b5, eq23_e1153_d_b6, eq23_e1153_d_b7, eq23_e1153_d_b8, eq23_e1153_d_b9, eq23_e1153_d_b10, eq23_e1153_d_b11,) = {
    if (s.v[75] != 0.0) {
        let eq23_e1151: f64 = ((nv0 - nv5) / s.v[4]);let eq23_e1151_d_n0: f64 = ((s.v[4] - ((nv0 - nv5) * s.dn[4][0])) / (s.v[4] * s.v[4]));let eq23_e1151_d_n1: f64 = (-(((nv0 - nv5) * s.dn[4][1]) / (s.v[4] * s.v[4])));let eq23_e1151_d_n2: f64 = (-(((nv0 - nv5) * s.dn[4][2]) / (s.v[4] * s.v[4])));let eq23_e1151_d_n3: f64 = (-(((nv0 - nv5) * s.dn[4][3]) / (s.v[4] * s.v[4])));let eq23_e1151_d_n4: f64 = (-(((nv0 - nv5) * s.dn[4][4]) / (s.v[4] * s.v[4])));let eq23_e1151_d_n5: f64 = (((-s.v[4]) - ((nv0 - nv5) * s.dn[4][5])) / (s.v[4] * s.v[4]));let eq23_e1151_d_n6: f64 = (-(((nv0 - nv5) * s.dn[4][6]) / (s.v[4] * s.v[4])));let eq23_e1151_d_n7: f64 = (-(((nv0 - nv5) * s.dn[4][7]) / (s.v[4] * s.v[4])));let eq23_e1151_d_n8: f64 = (-(((nv0 - nv5) * s.dn[4][8]) / (s.v[4] * s.v[4])));let eq23_e1151_d_n9: f64 = (-(((nv0 - nv5) * s.dn[4][9]) / (s.v[4] * s.v[4])));let eq23_e1151_d_n10: f64 = (-(((nv0 - nv5) * s.dn[4][10]) / (s.v[4] * s.v[4])));let eq23_e1151_d_n11: f64 = (-(((nv0 - nv5) * s.dn[4][11]) / (s.v[4] * s.v[4])));let eq23_e1151_d_n12: f64 = (-(((nv0 - nv5) * s.dn[4][12]) / (s.v[4] * s.v[4])));let eq23_e1151_d_n13: f64 = (-(((nv0 - nv5) * s.dn[4][13]) / (s.v[4] * s.v[4])));let eq23_e1151_d_n14: f64 = (-(((nv0 - nv5) * s.dn[4][14]) / (s.v[4] * s.v[4])));let eq23_e1151_d_n15: f64 = (-(((nv0 - nv5) * s.dn[4][15]) / (s.v[4] * s.v[4])));let eq23_e1151_d_n16: f64 = (-(((nv0 - nv5) * s.dn[4][16]) / (s.v[4] * s.v[4])));let eq23_e1151_d_n17: f64 = (-(((nv0 - nv5) * s.dn[4][17]) / (s.v[4] * s.v[4])));let eq23_e1151_d_b0: f64 = (-(((nv0 - nv5) * s.db[4][0]) / (s.v[4] * s.v[4])));let eq23_e1151_d_b1: f64 = (-(((nv0 - nv5) * s.db[4][1]) / (s.v[4] * s.v[4])));let eq23_e1151_d_b2: f64 = (-(((nv0 - nv5) * s.db[4][2]) / (s.v[4] * s.v[4])));let eq23_e1151_d_b3: f64 = (-(((nv0 - nv5) * s.db[4][3]) / (s.v[4] * s.v[4])));let eq23_e1151_d_b4: f64 = (-(((nv0 - nv5) * s.db[4][4]) / (s.v[4] * s.v[4])));let eq23_e1151_d_b5: f64 = (-(((nv0 - nv5) * s.db[4][5]) / (s.v[4] * s.v[4])));let eq23_e1151_d_b6: f64 = (-(((nv0 - nv5) * s.db[4][6]) / (s.v[4] * s.v[4])));let eq23_e1151_d_b7: f64 = (-(((nv0 - nv5) * s.db[4][7]) / (s.v[4] * s.v[4])));let eq23_e1151_d_b8: f64 = (-(((nv0 - nv5) * s.db[4][8]) / (s.v[4] * s.v[4])));let eq23_e1151_d_b9: f64 = (-(((nv0 - nv5) * s.db[4][9]) / (s.v[4] * s.v[4])));let eq23_e1151_d_b10: f64 = (-(((nv0 - nv5) * s.db[4][10]) / (s.v[4] * s.v[4])));let eq23_e1151_d_b11: f64 = (-(((nv0 - nv5) * s.db[4][11]) / (s.v[4] * s.v[4])));
        (eq23_e1151, eq23_e1151_d_n0, eq23_e1151_d_n1, eq23_e1151_d_n2, eq23_e1151_d_n3, eq23_e1151_d_n4, eq23_e1151_d_n5, eq23_e1151_d_n6, eq23_e1151_d_n7, eq23_e1151_d_n8, eq23_e1151_d_n9, eq23_e1151_d_n10, eq23_e1151_d_n11, eq23_e1151_d_n12, eq23_e1151_d_n13, eq23_e1151_d_n14, eq23_e1151_d_n15, eq23_e1151_d_n16, eq23_e1151_d_n17, eq23_e1151_d_b0, eq23_e1151_d_b1, eq23_e1151_d_b2, eq23_e1151_d_b3, eq23_e1151_d_b4, eq23_e1151_d_b5, eq23_e1151_d_b6, eq23_e1151_d_b7, eq23_e1151_d_b8, eq23_e1151_d_b9, eq23_e1151_d_b10, eq23_e1151_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e1153;let eq23_node_derivatives: [f64; 18] = [eq23_e1153_d_n0, eq23_e1153_d_n1, eq23_e1153_d_n2, eq23_e1153_d_n3, eq23_e1153_d_n4, eq23_e1153_d_n5, eq23_e1153_d_n6, eq23_e1153_d_n7, eq23_e1153_d_n8, eq23_e1153_d_n9, eq23_e1153_d_n10, eq23_e1153_d_n11, eq23_e1153_d_n12, eq23_e1153_d_n13, eq23_e1153_d_n14, eq23_e1153_d_n15, eq23_e1153_d_n16, eq23_e1153_d_n17];let eq23_branch_derivatives: [f64; 12] = [eq23_e1153_d_b0, eq23_e1153_d_b1, eq23_e1153_d_b2, eq23_e1153_d_b3, eq23_e1153_d_b4, eq23_e1153_d_b5, eq23_e1153_d_b6, eq23_e1153_d_b7, eq23_e1153_d_b8, eq23_e1153_d_b9, eq23_e1153_d_b10, eq23_e1153_d_b11];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(5),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );
        let (eq24_e1158,) = {
    if (s.v[75] == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq24_value: f64 = eq24_e1158;
        stamper.stamp_potential_const_local(
            3,
            eq24_value,
        );
        let (eq25_e1164, eq25_e1164_d_n0, eq25_e1164_d_n1, eq25_e1164_d_n2, eq25_e1164_d_n3, eq25_e1164_d_n4, eq25_e1164_d_n5, eq25_e1164_d_n6, eq25_e1164_d_n7, eq25_e1164_d_n8, eq25_e1164_d_n9, eq25_e1164_d_n10, eq25_e1164_d_n11, eq25_e1164_d_n12, eq25_e1164_d_n13, eq25_e1164_d_n14, eq25_e1164_d_n15, eq25_e1164_d_n16, eq25_e1164_d_n17, eq25_e1164_d_b0, eq25_e1164_d_b1, eq25_e1164_d_b2, eq25_e1164_d_b3, eq25_e1164_d_b4, eq25_e1164_d_b5, eq25_e1164_d_b6, eq25_e1164_d_b7, eq25_e1164_d_b8, eq25_e1164_d_b9, eq25_e1164_d_b10, eq25_e1164_d_b11,) = {
    if (s.v[76] != 0.0) {
        let eq25_e1162: f64 = ((nv7 - nv2) / s.v[5]);let eq25_e1162_d_n0: f64 = (-(((nv7 - nv2) * s.dn[5][0]) / (s.v[5] * s.v[5])));let eq25_e1162_d_n1: f64 = (-(((nv7 - nv2) * s.dn[5][1]) / (s.v[5] * s.v[5])));let eq25_e1162_d_n2: f64 = (((-s.v[5]) - ((nv7 - nv2) * s.dn[5][2])) / (s.v[5] * s.v[5]));let eq25_e1162_d_n3: f64 = (-(((nv7 - nv2) * s.dn[5][3]) / (s.v[5] * s.v[5])));let eq25_e1162_d_n4: f64 = (-(((nv7 - nv2) * s.dn[5][4]) / (s.v[5] * s.v[5])));let eq25_e1162_d_n5: f64 = (-(((nv7 - nv2) * s.dn[5][5]) / (s.v[5] * s.v[5])));let eq25_e1162_d_n6: f64 = (-(((nv7 - nv2) * s.dn[5][6]) / (s.v[5] * s.v[5])));let eq25_e1162_d_n7: f64 = ((s.v[5] - ((nv7 - nv2) * s.dn[5][7])) / (s.v[5] * s.v[5]));let eq25_e1162_d_n8: f64 = (-(((nv7 - nv2) * s.dn[5][8]) / (s.v[5] * s.v[5])));let eq25_e1162_d_n9: f64 = (-(((nv7 - nv2) * s.dn[5][9]) / (s.v[5] * s.v[5])));let eq25_e1162_d_n10: f64 = (-(((nv7 - nv2) * s.dn[5][10]) / (s.v[5] * s.v[5])));let eq25_e1162_d_n11: f64 = (-(((nv7 - nv2) * s.dn[5][11]) / (s.v[5] * s.v[5])));let eq25_e1162_d_n12: f64 = (-(((nv7 - nv2) * s.dn[5][12]) / (s.v[5] * s.v[5])));let eq25_e1162_d_n13: f64 = (-(((nv7 - nv2) * s.dn[5][13]) / (s.v[5] * s.v[5])));let eq25_e1162_d_n14: f64 = (-(((nv7 - nv2) * s.dn[5][14]) / (s.v[5] * s.v[5])));let eq25_e1162_d_n15: f64 = (-(((nv7 - nv2) * s.dn[5][15]) / (s.v[5] * s.v[5])));let eq25_e1162_d_n16: f64 = (-(((nv7 - nv2) * s.dn[5][16]) / (s.v[5] * s.v[5])));let eq25_e1162_d_n17: f64 = (-(((nv7 - nv2) * s.dn[5][17]) / (s.v[5] * s.v[5])));let eq25_e1162_d_b0: f64 = (-(((nv7 - nv2) * s.db[5][0]) / (s.v[5] * s.v[5])));let eq25_e1162_d_b1: f64 = (-(((nv7 - nv2) * s.db[5][1]) / (s.v[5] * s.v[5])));let eq25_e1162_d_b2: f64 = (-(((nv7 - nv2) * s.db[5][2]) / (s.v[5] * s.v[5])));let eq25_e1162_d_b3: f64 = (-(((nv7 - nv2) * s.db[5][3]) / (s.v[5] * s.v[5])));let eq25_e1162_d_b4: f64 = (-(((nv7 - nv2) * s.db[5][4]) / (s.v[5] * s.v[5])));let eq25_e1162_d_b5: f64 = (-(((nv7 - nv2) * s.db[5][5]) / (s.v[5] * s.v[5])));let eq25_e1162_d_b6: f64 = (-(((nv7 - nv2) * s.db[5][6]) / (s.v[5] * s.v[5])));let eq25_e1162_d_b7: f64 = (-(((nv7 - nv2) * s.db[5][7]) / (s.v[5] * s.v[5])));let eq25_e1162_d_b8: f64 = (-(((nv7 - nv2) * s.db[5][8]) / (s.v[5] * s.v[5])));let eq25_e1162_d_b9: f64 = (-(((nv7 - nv2) * s.db[5][9]) / (s.v[5] * s.v[5])));let eq25_e1162_d_b10: f64 = (-(((nv7 - nv2) * s.db[5][10]) / (s.v[5] * s.v[5])));let eq25_e1162_d_b11: f64 = (-(((nv7 - nv2) * s.db[5][11]) / (s.v[5] * s.v[5])));
        (eq25_e1162, eq25_e1162_d_n0, eq25_e1162_d_n1, eq25_e1162_d_n2, eq25_e1162_d_n3, eq25_e1162_d_n4, eq25_e1162_d_n5, eq25_e1162_d_n6, eq25_e1162_d_n7, eq25_e1162_d_n8, eq25_e1162_d_n9, eq25_e1162_d_n10, eq25_e1162_d_n11, eq25_e1162_d_n12, eq25_e1162_d_n13, eq25_e1162_d_n14, eq25_e1162_d_n15, eq25_e1162_d_n16, eq25_e1162_d_n17, eq25_e1162_d_b0, eq25_e1162_d_b1, eq25_e1162_d_b2, eq25_e1162_d_b3, eq25_e1162_d_b4, eq25_e1162_d_b5, eq25_e1162_d_b6, eq25_e1162_d_b7, eq25_e1162_d_b8, eq25_e1162_d_b9, eq25_e1162_d_b10, eq25_e1162_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e1164;let eq25_node_derivatives: [f64; 18] = [eq25_e1164_d_n0, eq25_e1164_d_n1, eq25_e1164_d_n2, eq25_e1164_d_n3, eq25_e1164_d_n4, eq25_e1164_d_n5, eq25_e1164_d_n6, eq25_e1164_d_n7, eq25_e1164_d_n8, eq25_e1164_d_n9, eq25_e1164_d_n10, eq25_e1164_d_n11, eq25_e1164_d_n12, eq25_e1164_d_n13, eq25_e1164_d_n14, eq25_e1164_d_n15, eq25_e1164_d_n16, eq25_e1164_d_n17];let eq25_branch_derivatives: [f64; 12] = [eq25_e1164_d_b0, eq25_e1164_d_b1, eq25_e1164_d_b2, eq25_e1164_d_b3, eq25_e1164_d_b4, eq25_e1164_d_b5, eq25_e1164_d_b6, eq25_e1164_d_b7, eq25_e1164_d_b8, eq25_e1164_d_b9, eq25_e1164_d_b10, eq25_e1164_d_b11];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(2),
            multiplicity * (eq25_value),
            &eq25_node_derivatives,
            &eq25_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_9(
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
        let (eq26_e1169,) = {
    if (s.v[76] == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e1169;
        stamper.stamp_potential_const_local(
            4,
            eq26_value,
        );let eq27_e1173: f64 = (s.v[18] + s.v[753]);let eq27_e1173_d_n0: f64 = (s.dn[18][0] + s.dn[753][0]);let eq27_e1173_d_n1: f64 = (s.dn[18][1] + s.dn[753][1]);let eq27_e1173_d_n2: f64 = (s.dn[18][2] + s.dn[753][2]);let eq27_e1173_d_n3: f64 = (s.dn[18][3] + s.dn[753][3]);let eq27_e1173_d_n4: f64 = (s.dn[18][4] + s.dn[753][4]);let eq27_e1173_d_n5: f64 = (s.dn[18][5] + s.dn[753][5]);let eq27_e1173_d_n6: f64 = (s.dn[18][6] + s.dn[753][6]);let eq27_e1173_d_n7: f64 = (s.dn[18][7] + s.dn[753][7]);let eq27_e1173_d_n8: f64 = (s.dn[18][8] + s.dn[753][8]);let eq27_e1173_d_n9: f64 = (s.dn[18][9] + s.dn[753][9]);let eq27_e1173_d_n10: f64 = (s.dn[18][10] + s.dn[753][10]);let eq27_e1173_d_n11: f64 = (s.dn[18][11] + s.dn[753][11]);let eq27_e1173_d_n12: f64 = (s.dn[18][12] + s.dn[753][12]);let eq27_e1173_d_n13: f64 = (s.dn[18][13] + s.dn[753][13]);let eq27_e1173_d_n14: f64 = (s.dn[18][14] + s.dn[753][14]);let eq27_e1173_d_n15: f64 = (s.dn[18][15] + s.dn[753][15]);let eq27_e1173_d_n16: f64 = (s.dn[18][16] + s.dn[753][16]);let eq27_e1173_d_n17: f64 = (s.dn[18][17] + s.dn[753][17]);let eq27_e1173_d_b0: f64 = (s.db[18][0] + s.db[753][0]);let eq27_e1173_d_b1: f64 = (s.db[18][1] + s.db[753][1]);let eq27_e1173_d_b2: f64 = (s.db[18][2] + s.db[753][2]);let eq27_e1173_d_b3: f64 = (s.db[18][3] + s.db[753][3]);let eq27_e1173_d_b4: f64 = (s.db[18][4] + s.db[753][4]);let eq27_e1173_d_b5: f64 = (s.db[18][5] + s.db[753][5]);let eq27_e1173_d_b6: f64 = (s.db[18][6] + s.db[753][6]);let eq27_e1173_d_b7: f64 = (s.db[18][7] + s.db[753][7]);let eq27_e1173_d_b8: f64 = (s.db[18][8] + s.db[753][8]);let eq27_e1173_d_b9: f64 = (s.db[18][9] + s.db[753][9]);let eq27_e1173_d_b10: f64 = (s.db[18][10] + s.db[753][10]);let eq27_e1173_d_b11: f64 = (s.db[18][11] + s.db[753][11]);let eq27_e1174: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq27_e1173);let eq27_e1175: f64 = (p.p87 * eq27_e1174);let eq27_e1175_d_n0: f64 = (p.p87 * (eq27_e1173_d_n0 * ddt_scale));let eq27_e1175_d_n1: f64 = (p.p87 * (eq27_e1173_d_n1 * ddt_scale));let eq27_e1175_d_n2: f64 = (p.p87 * (eq27_e1173_d_n2 * ddt_scale));let eq27_e1175_d_n3: f64 = (p.p87 * (eq27_e1173_d_n3 * ddt_scale));let eq27_e1175_d_n4: f64 = (p.p87 * (eq27_e1173_d_n4 * ddt_scale));let eq27_e1175_d_n5: f64 = (p.p87 * (eq27_e1173_d_n5 * ddt_scale));let eq27_e1175_d_n6: f64 = (p.p87 * (eq27_e1173_d_n6 * ddt_scale));let eq27_e1175_d_n7: f64 = (p.p87 * (eq27_e1173_d_n7 * ddt_scale));let eq27_e1175_d_n8: f64 = (p.p87 * (eq27_e1173_d_n8 * ddt_scale));let eq27_e1175_d_n9: f64 = (p.p87 * (eq27_e1173_d_n9 * ddt_scale));let eq27_e1175_d_n10: f64 = (p.p87 * (eq27_e1173_d_n10 * ddt_scale));let eq27_e1175_d_n11: f64 = (p.p87 * (eq27_e1173_d_n11 * ddt_scale));let eq27_e1175_d_n12: f64 = (p.p87 * (eq27_e1173_d_n12 * ddt_scale));let eq27_e1175_d_n13: f64 = (p.p87 * (eq27_e1173_d_n13 * ddt_scale));let eq27_e1175_d_n14: f64 = (p.p87 * (eq27_e1173_d_n14 * ddt_scale));let eq27_e1175_d_n15: f64 = (p.p87 * (eq27_e1173_d_n15 * ddt_scale));let eq27_e1175_d_n16: f64 = (p.p87 * (eq27_e1173_d_n16 * ddt_scale));let eq27_e1175_d_n17: f64 = (p.p87 * (eq27_e1173_d_n17 * ddt_scale));let eq27_e1175_d_b0: f64 = (p.p87 * (eq27_e1173_d_b0 * ddt_scale));let eq27_e1175_d_b1: f64 = (p.p87 * (eq27_e1173_d_b1 * ddt_scale));let eq27_e1175_d_b2: f64 = (p.p87 * (eq27_e1173_d_b2 * ddt_scale));let eq27_e1175_d_b3: f64 = (p.p87 * (eq27_e1173_d_b3 * ddt_scale));let eq27_e1175_d_b4: f64 = (p.p87 * (eq27_e1173_d_b4 * ddt_scale));let eq27_e1175_d_b5: f64 = (p.p87 * (eq27_e1173_d_b5 * ddt_scale));let eq27_e1175_d_b6: f64 = (p.p87 * (eq27_e1173_d_b6 * ddt_scale));let eq27_e1175_d_b7: f64 = (p.p87 * (eq27_e1173_d_b7 * ddt_scale));let eq27_e1175_d_b8: f64 = (p.p87 * (eq27_e1173_d_b8 * ddt_scale));let eq27_e1175_d_b9: f64 = (p.p87 * (eq27_e1173_d_b9 * ddt_scale));let eq27_e1175_d_b10: f64 = (p.p87 * (eq27_e1173_d_b10 * ddt_scale));
        let eq27_e1175_d_b11: f64 = (p.p87 * (eq27_e1173_d_b11 * ddt_scale));let eq27_value: f64 = eq27_e1175;let eq27_node_derivatives: [f64; 18] = [eq27_e1175_d_n0, eq27_e1175_d_n1, eq27_e1175_d_n2, eq27_e1175_d_n3, eq27_e1175_d_n4, eq27_e1175_d_n5, eq27_e1175_d_n6, eq27_e1175_d_n7, eq27_e1175_d_n8, eq27_e1175_d_n9, eq27_e1175_d_n10, eq27_e1175_d_n11, eq27_e1175_d_n12, eq27_e1175_d_n13, eq27_e1175_d_n14, eq27_e1175_d_n15, eq27_e1175_d_n16, eq27_e1175_d_n17];let eq27_branch_derivatives: [f64; 12] = [eq27_e1175_d_b0, eq27_e1175_d_b1, eq27_e1175_d_b2, eq27_e1175_d_b3, eq27_e1175_d_b4, eq27_e1175_d_b5, eq27_e1175_d_b6, eq27_e1175_d_b7, eq27_e1175_d_b8, eq27_e1175_d_b9, eq27_e1175_d_b10, eq27_e1175_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &eq27_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_10(
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
        let eq28_e1179: f64 = (s.v[19] + s.v[751]);let eq28_e1179_d_n0: f64 = (s.dn[19][0] + s.dn[751][0]);let eq28_e1179_d_n1: f64 = (s.dn[19][1] + s.dn[751][1]);let eq28_e1179_d_n2: f64 = (s.dn[19][2] + s.dn[751][2]);let eq28_e1179_d_n3: f64 = (s.dn[19][3] + s.dn[751][3]);let eq28_e1179_d_n4: f64 = (s.dn[19][4] + s.dn[751][4]);let eq28_e1179_d_n5: f64 = (s.dn[19][5] + s.dn[751][5]);let eq28_e1179_d_n6: f64 = (s.dn[19][6] + s.dn[751][6]);let eq28_e1179_d_n7: f64 = (s.dn[19][7] + s.dn[751][7]);let eq28_e1179_d_n8: f64 = (s.dn[19][8] + s.dn[751][8]);let eq28_e1179_d_n9: f64 = (s.dn[19][9] + s.dn[751][9]);let eq28_e1179_d_n10: f64 = (s.dn[19][10] + s.dn[751][10]);let eq28_e1179_d_n11: f64 = (s.dn[19][11] + s.dn[751][11]);let eq28_e1179_d_n12: f64 = (s.dn[19][12] + s.dn[751][12]);let eq28_e1179_d_n13: f64 = (s.dn[19][13] + s.dn[751][13]);let eq28_e1179_d_n14: f64 = (s.dn[19][14] + s.dn[751][14]);let eq28_e1179_d_n15: f64 = (s.dn[19][15] + s.dn[751][15]);let eq28_e1179_d_n16: f64 = (s.dn[19][16] + s.dn[751][16]);let eq28_e1179_d_n17: f64 = (s.dn[19][17] + s.dn[751][17]);let eq28_e1179_d_b0: f64 = (s.db[19][0] + s.db[751][0]);let eq28_e1179_d_b1: f64 = (s.db[19][1] + s.db[751][1]);let eq28_e1179_d_b2: f64 = (s.db[19][2] + s.db[751][2]);let eq28_e1179_d_b3: f64 = (s.db[19][3] + s.db[751][3]);let eq28_e1179_d_b4: f64 = (s.db[19][4] + s.db[751][4]);let eq28_e1179_d_b5: f64 = (s.db[19][5] + s.db[751][5]);let eq28_e1179_d_b6: f64 = (s.db[19][6] + s.db[751][6]);let eq28_e1179_d_b7: f64 = (s.db[19][7] + s.db[751][7]);let eq28_e1179_d_b8: f64 = (s.db[19][8] + s.db[751][8]);let eq28_e1179_d_b9: f64 = (s.db[19][9] + s.db[751][9]);let eq28_e1179_d_b10: f64 = (s.db[19][10] + s.db[751][10]);let eq28_e1179_d_b11: f64 = (s.db[19][11] + s.db[751][11]);let eq28_e1180: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq28_e1179);let eq28_e1181: f64 = (p.p87 * eq28_e1180);let eq28_e1181_d_n0: f64 = (p.p87 * (eq28_e1179_d_n0 * ddt_scale));let eq28_e1181_d_n1: f64 = (p.p87 * (eq28_e1179_d_n1 * ddt_scale));let eq28_e1181_d_n2: f64 = (p.p87 * (eq28_e1179_d_n2 * ddt_scale));let eq28_e1181_d_n3: f64 = (p.p87 * (eq28_e1179_d_n3 * ddt_scale));let eq28_e1181_d_n4: f64 = (p.p87 * (eq28_e1179_d_n4 * ddt_scale));let eq28_e1181_d_n5: f64 = (p.p87 * (eq28_e1179_d_n5 * ddt_scale));let eq28_e1181_d_n6: f64 = (p.p87 * (eq28_e1179_d_n6 * ddt_scale));let eq28_e1181_d_n7: f64 = (p.p87 * (eq28_e1179_d_n7 * ddt_scale));let eq28_e1181_d_n8: f64 = (p.p87 * (eq28_e1179_d_n8 * ddt_scale));let eq28_e1181_d_n9: f64 = (p.p87 * (eq28_e1179_d_n9 * ddt_scale));let eq28_e1181_d_n10: f64 = (p.p87 * (eq28_e1179_d_n10 * ddt_scale));let eq28_e1181_d_n11: f64 = (p.p87 * (eq28_e1179_d_n11 * ddt_scale));let eq28_e1181_d_n12: f64 = (p.p87 * (eq28_e1179_d_n12 * ddt_scale));let eq28_e1181_d_n13: f64 = (p.p87 * (eq28_e1179_d_n13 * ddt_scale));let eq28_e1181_d_n14: f64 = (p.p87 * (eq28_e1179_d_n14 * ddt_scale));let eq28_e1181_d_n15: f64 = (p.p87 * (eq28_e1179_d_n15 * ddt_scale));let eq28_e1181_d_n16: f64 = (p.p87 * (eq28_e1179_d_n16 * ddt_scale));let eq28_e1181_d_n17: f64 = (p.p87 * (eq28_e1179_d_n17 * ddt_scale));let eq28_e1181_d_b0: f64 = (p.p87 * (eq28_e1179_d_b0 * ddt_scale));let eq28_e1181_d_b1: f64 = (p.p87 * (eq28_e1179_d_b1 * ddt_scale));let eq28_e1181_d_b2: f64 = (p.p87 * (eq28_e1179_d_b2 * ddt_scale));let eq28_e1181_d_b3: f64 = (p.p87 * (eq28_e1179_d_b3 * ddt_scale));let eq28_e1181_d_b4: f64 = (p.p87 * (eq28_e1179_d_b4 * ddt_scale));let eq28_e1181_d_b5: f64 = (p.p87 * (eq28_e1179_d_b5 * ddt_scale));let eq28_e1181_d_b6: f64 = (p.p87 * (eq28_e1179_d_b6 * ddt_scale));let eq28_e1181_d_b7: f64 = (p.p87 * (eq28_e1179_d_b7 * ddt_scale));let eq28_e1181_d_b8: f64 = (p.p87 * (eq28_e1179_d_b8 * ddt_scale));let eq28_e1181_d_b9: f64 = (p.p87 * (eq28_e1179_d_b9 * ddt_scale));let eq28_e1181_d_b10: f64 = (p.p87 * (eq28_e1179_d_b10 * ddt_scale));
        let eq28_e1181_d_b11: f64 = (p.p87 * (eq28_e1179_d_b11 * ddt_scale));let eq28_value: f64 = eq28_e1181;let eq28_node_derivatives: [f64; 18] = [eq28_e1181_d_n0, eq28_e1181_d_n1, eq28_e1181_d_n2, eq28_e1181_d_n3, eq28_e1181_d_n4, eq28_e1181_d_n5, eq28_e1181_d_n6, eq28_e1181_d_n7, eq28_e1181_d_n8, eq28_e1181_d_n9, eq28_e1181_d_n10, eq28_e1181_d_n11, eq28_e1181_d_n12, eq28_e1181_d_n13, eq28_e1181_d_n14, eq28_e1181_d_n15, eq28_e1181_d_n16, eq28_e1181_d_n17];let eq28_branch_derivatives: [f64; 12] = [eq28_e1181_d_b0, eq28_e1181_d_b1, eq28_e1181_d_b2, eq28_e1181_d_b3, eq28_e1181_d_b4, eq28_e1181_d_b5, eq28_e1181_d_b6, eq28_e1181_d_b7, eq28_e1181_d_b8, eq28_e1181_d_b9, eq28_e1181_d_b10, eq28_e1181_d_b11];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq28_value),
            &eq28_node_derivatives,
            &eq28_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_11(
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
        let eq29_e1186: f64 = (s.v[753] + s.v[751]);let eq29_e1186_d_n0: f64 = (s.dn[753][0] + s.dn[751][0]);let eq29_e1186_d_n1: f64 = (s.dn[753][1] + s.dn[751][1]);let eq29_e1186_d_n2: f64 = (s.dn[753][2] + s.dn[751][2]);let eq29_e1186_d_n3: f64 = (s.dn[753][3] + s.dn[751][3]);let eq29_e1186_d_n4: f64 = (s.dn[753][4] + s.dn[751][4]);let eq29_e1186_d_n5: f64 = (s.dn[753][5] + s.dn[751][5]);let eq29_e1186_d_n6: f64 = (s.dn[753][6] + s.dn[751][6]);let eq29_e1186_d_n7: f64 = (s.dn[753][7] + s.dn[751][7]);let eq29_e1186_d_n8: f64 = (s.dn[753][8] + s.dn[751][8]);let eq29_e1186_d_n9: f64 = (s.dn[753][9] + s.dn[751][9]);let eq29_e1186_d_n10: f64 = (s.dn[753][10] + s.dn[751][10]);let eq29_e1186_d_n11: f64 = (s.dn[753][11] + s.dn[751][11]);let eq29_e1186_d_n12: f64 = (s.dn[753][12] + s.dn[751][12]);let eq29_e1186_d_n13: f64 = (s.dn[753][13] + s.dn[751][13]);let eq29_e1186_d_n14: f64 = (s.dn[753][14] + s.dn[751][14]);let eq29_e1186_d_n15: f64 = (s.dn[753][15] + s.dn[751][15]);let eq29_e1186_d_n16: f64 = (s.dn[753][16] + s.dn[751][16]);let eq29_e1186_d_n17: f64 = (s.dn[753][17] + s.dn[751][17]);let eq29_e1186_d_b0: f64 = (s.db[753][0] + s.db[751][0]);let eq29_e1186_d_b1: f64 = (s.db[753][1] + s.db[751][1]);let eq29_e1186_d_b2: f64 = (s.db[753][2] + s.db[751][2]);let eq29_e1186_d_b3: f64 = (s.db[753][3] + s.db[751][3]);let eq29_e1186_d_b4: f64 = (s.db[753][4] + s.db[751][4]);let eq29_e1186_d_b5: f64 = (s.db[753][5] + s.db[751][5]);let eq29_e1186_d_b6: f64 = (s.db[753][6] + s.db[751][6]);let eq29_e1186_d_b7: f64 = (s.db[753][7] + s.db[751][7]);let eq29_e1186_d_b8: f64 = (s.db[753][8] + s.db[751][8]);let eq29_e1186_d_b9: f64 = (s.db[753][9] + s.db[751][9]);let eq29_e1186_d_b10: f64 = (s.db[753][10] + s.db[751][10]);let eq29_e1186_d_b11: f64 = (s.db[753][11] + s.db[751][11]);let eq29_e1188: f64 = (eq29_e1186 + s.v[752]);let eq29_e1188_d_n0: f64 = (eq29_e1186_d_n0 + s.dn[752][0]);let eq29_e1188_d_n1: f64 = (eq29_e1186_d_n1 + s.dn[752][1]);let eq29_e1188_d_n2: f64 = (eq29_e1186_d_n2 + s.dn[752][2]);let eq29_e1188_d_n3: f64 = (eq29_e1186_d_n3 + s.dn[752][3]);let eq29_e1188_d_n4: f64 = (eq29_e1186_d_n4 + s.dn[752][4]);let eq29_e1188_d_n5: f64 = (eq29_e1186_d_n5 + s.dn[752][5]);let eq29_e1188_d_n6: f64 = (eq29_e1186_d_n6 + s.dn[752][6]);let eq29_e1188_d_n7: f64 = (eq29_e1186_d_n7 + s.dn[752][7]);let eq29_e1188_d_n8: f64 = (eq29_e1186_d_n8 + s.dn[752][8]);let eq29_e1188_d_n9: f64 = (eq29_e1186_d_n9 + s.dn[752][9]);let eq29_e1188_d_n10: f64 = (eq29_e1186_d_n10 + s.dn[752][10]);let eq29_e1188_d_n11: f64 = (eq29_e1186_d_n11 + s.dn[752][11]);let eq29_e1188_d_n12: f64 = (eq29_e1186_d_n12 + s.dn[752][12]);let eq29_e1188_d_n13: f64 = (eq29_e1186_d_n13 + s.dn[752][13]);let eq29_e1188_d_n14: f64 = (eq29_e1186_d_n14 + s.dn[752][14]);let eq29_e1188_d_n15: f64 = (eq29_e1186_d_n15 + s.dn[752][15]);let eq29_e1188_d_n16: f64 = (eq29_e1186_d_n16 + s.dn[752][16]);let eq29_e1188_d_n17: f64 = (eq29_e1186_d_n17 + s.dn[752][17]);let eq29_e1188_d_b0: f64 = (eq29_e1186_d_b0 + s.db[752][0]);let eq29_e1188_d_b1: f64 = (eq29_e1186_d_b1 + s.db[752][1]);let eq29_e1188_d_b2: f64 = (eq29_e1186_d_b2 + s.db[752][2]);let eq29_e1188_d_b3: f64 = (eq29_e1186_d_b3 + s.db[752][3]);let eq29_e1188_d_b4: f64 = (eq29_e1186_d_b4 + s.db[752][4]);let eq29_e1188_d_b5: f64 = (eq29_e1186_d_b5 + s.db[752][5]);let eq29_e1188_d_b6: f64 = (eq29_e1186_d_b6 + s.db[752][6]);let eq29_e1188_d_b7: f64 = (eq29_e1186_d_b7 + s.db[752][7]);let eq29_e1188_d_b8: f64 = (eq29_e1186_d_b8 + s.db[752][8]);let eq29_e1188_d_b9: f64 = (eq29_e1186_d_b9 + s.db[752][9]);let eq29_e1188_d_b10: f64 = (eq29_e1186_d_b10 + s.db[752][10]);let eq29_e1188_d_b11: f64 = (eq29_e1186_d_b11 + s.db[752][11]);let eq29_e1189: f64 = (s.v[20] - eq29_e1188);let eq29_e1189_d_n0: f64 = (s.dn[20][0] - eq29_e1188_d_n0);let eq29_e1189_d_n1: f64 = (s.dn[20][1] - eq29_e1188_d_n1);let eq29_e1189_d_n2: f64 = (s.dn[20][2] - eq29_e1188_d_n2);let eq29_e1189_d_n3: f64 = (s.dn[20][3] - eq29_e1188_d_n3);let eq29_e1189_d_n4: f64 = (s.dn[20][4] - eq29_e1188_d_n4);let eq29_e1189_d_n5: f64 = (s.dn[20][5] - eq29_e1188_d_n5);
        let eq29_e1189_d_n6: f64 = (s.dn[20][6] - eq29_e1188_d_n6);let eq29_e1189_d_n7: f64 = (s.dn[20][7] - eq29_e1188_d_n7);let eq29_e1189_d_n8: f64 = (s.dn[20][8] - eq29_e1188_d_n8);let eq29_e1189_d_n9: f64 = (s.dn[20][9] - eq29_e1188_d_n9);let eq29_e1189_d_n10: f64 = (s.dn[20][10] - eq29_e1188_d_n10);let eq29_e1189_d_n11: f64 = (s.dn[20][11] - eq29_e1188_d_n11);let eq29_e1189_d_n12: f64 = (s.dn[20][12] - eq29_e1188_d_n12);let eq29_e1189_d_n13: f64 = (s.dn[20][13] - eq29_e1188_d_n13);let eq29_e1189_d_n14: f64 = (s.dn[20][14] - eq29_e1188_d_n14);let eq29_e1189_d_n15: f64 = (s.dn[20][15] - eq29_e1188_d_n15);let eq29_e1189_d_n16: f64 = (s.dn[20][16] - eq29_e1188_d_n16);let eq29_e1189_d_n17: f64 = (s.dn[20][17] - eq29_e1188_d_n17);let eq29_e1189_d_b0: f64 = (s.db[20][0] - eq29_e1188_d_b0);let eq29_e1189_d_b1: f64 = (s.db[20][1] - eq29_e1188_d_b1);let eq29_e1189_d_b2: f64 = (s.db[20][2] - eq29_e1188_d_b2);let eq29_e1189_d_b3: f64 = (s.db[20][3] - eq29_e1188_d_b3);let eq29_e1189_d_b4: f64 = (s.db[20][4] - eq29_e1188_d_b4);let eq29_e1189_d_b5: f64 = (s.db[20][5] - eq29_e1188_d_b5);let eq29_e1189_d_b6: f64 = (s.db[20][6] - eq29_e1188_d_b6);let eq29_e1189_d_b7: f64 = (s.db[20][7] - eq29_e1188_d_b7);let eq29_e1189_d_b8: f64 = (s.db[20][8] - eq29_e1188_d_b8);let eq29_e1189_d_b9: f64 = (s.db[20][9] - eq29_e1188_d_b9);let eq29_e1189_d_b10: f64 = (s.db[20][10] - eq29_e1188_d_b10);let eq29_e1189_d_b11: f64 = (s.db[20][11] - eq29_e1188_d_b11);let eq29_e1190: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq29_e1189);let eq29_e1191: f64 = (p.p87 * eq29_e1190);let eq29_e1191_d_n0: f64 = (p.p87 * (eq29_e1189_d_n0 * ddt_scale));let eq29_e1191_d_n1: f64 = (p.p87 * (eq29_e1189_d_n1 * ddt_scale));let eq29_e1191_d_n2: f64 = (p.p87 * (eq29_e1189_d_n2 * ddt_scale));let eq29_e1191_d_n3: f64 = (p.p87 * (eq29_e1189_d_n3 * ddt_scale));let eq29_e1191_d_n4: f64 = (p.p87 * (eq29_e1189_d_n4 * ddt_scale));let eq29_e1191_d_n5: f64 = (p.p87 * (eq29_e1189_d_n5 * ddt_scale));let eq29_e1191_d_n6: f64 = (p.p87 * (eq29_e1189_d_n6 * ddt_scale));let eq29_e1191_d_n7: f64 = (p.p87 * (eq29_e1189_d_n7 * ddt_scale));let eq29_e1191_d_n8: f64 = (p.p87 * (eq29_e1189_d_n8 * ddt_scale));let eq29_e1191_d_n9: f64 = (p.p87 * (eq29_e1189_d_n9 * ddt_scale));let eq29_e1191_d_n10: f64 = (p.p87 * (eq29_e1189_d_n10 * ddt_scale));let eq29_e1191_d_n11: f64 = (p.p87 * (eq29_e1189_d_n11 * ddt_scale));let eq29_e1191_d_n12: f64 = (p.p87 * (eq29_e1189_d_n12 * ddt_scale));let eq29_e1191_d_n13: f64 = (p.p87 * (eq29_e1189_d_n13 * ddt_scale));let eq29_e1191_d_n14: f64 = (p.p87 * (eq29_e1189_d_n14 * ddt_scale));let eq29_e1191_d_n15: f64 = (p.p87 * (eq29_e1189_d_n15 * ddt_scale));let eq29_e1191_d_n16: f64 = (p.p87 * (eq29_e1189_d_n16 * ddt_scale));let eq29_e1191_d_n17: f64 = (p.p87 * (eq29_e1189_d_n17 * ddt_scale));let eq29_e1191_d_b0: f64 = (p.p87 * (eq29_e1189_d_b0 * ddt_scale));let eq29_e1191_d_b1: f64 = (p.p87 * (eq29_e1189_d_b1 * ddt_scale));let eq29_e1191_d_b2: f64 = (p.p87 * (eq29_e1189_d_b2 * ddt_scale));let eq29_e1191_d_b3: f64 = (p.p87 * (eq29_e1189_d_b3 * ddt_scale));let eq29_e1191_d_b4: f64 = (p.p87 * (eq29_e1189_d_b4 * ddt_scale));let eq29_e1191_d_b5: f64 = (p.p87 * (eq29_e1189_d_b5 * ddt_scale));let eq29_e1191_d_b6: f64 = (p.p87 * (eq29_e1189_d_b6 * ddt_scale));let eq29_e1191_d_b7: f64 = (p.p87 * (eq29_e1189_d_b7 * ddt_scale));let eq29_e1191_d_b8: f64 = (p.p87 * (eq29_e1189_d_b8 * ddt_scale));let eq29_e1191_d_b9: f64 = (p.p87 * (eq29_e1189_d_b9 * ddt_scale));let eq29_e1191_d_b10: f64 = (p.p87 * (eq29_e1189_d_b10 * ddt_scale));let eq29_e1191_d_b11: f64 = (p.p87 * (eq29_e1189_d_b11 * ddt_scale));let eq29_value: f64 = eq29_e1191;
        let eq29_node_derivatives: [f64; 18] = [eq29_e1191_d_n0, eq29_e1191_d_n1, eq29_e1191_d_n2, eq29_e1191_d_n3, eq29_e1191_d_n4, eq29_e1191_d_n5, eq29_e1191_d_n6, eq29_e1191_d_n7, eq29_e1191_d_n8, eq29_e1191_d_n9, eq29_e1191_d_n10, eq29_e1191_d_n11, eq29_e1191_d_n12, eq29_e1191_d_n13, eq29_e1191_d_n14, eq29_e1191_d_n15, eq29_e1191_d_n16, eq29_e1191_d_n17];let eq29_branch_derivatives: [f64; 12] = [eq29_e1191_d_b0, eq29_e1191_d_b1, eq29_e1191_d_b2, eq29_e1191_d_b3, eq29_e1191_d_b4, eq29_e1191_d_b5, eq29_e1191_d_b6, eq29_e1191_d_b7, eq29_e1191_d_b8, eq29_e1191_d_b9, eq29_e1191_d_b10, eq29_e1191_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq29_value),
            &eq29_node_derivatives,
            &eq29_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_12(
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
        let eq30_e1194: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, s.v[743]);let eq30_e1195: f64 = (p.p87 * eq30_e1194);let eq30_e1195_d_n0: f64 = (p.p87 * (s.dn[743][0] * ddt_scale));let eq30_e1195_d_n1: f64 = (p.p87 * (s.dn[743][1] * ddt_scale));let eq30_e1195_d_n2: f64 = (p.p87 * (s.dn[743][2] * ddt_scale));let eq30_e1195_d_n3: f64 = (p.p87 * (s.dn[743][3] * ddt_scale));let eq30_e1195_d_n4: f64 = (p.p87 * (s.dn[743][4] * ddt_scale));let eq30_e1195_d_n5: f64 = (p.p87 * (s.dn[743][5] * ddt_scale));let eq30_e1195_d_n6: f64 = (p.p87 * (s.dn[743][6] * ddt_scale));let eq30_e1195_d_n7: f64 = (p.p87 * (s.dn[743][7] * ddt_scale));let eq30_e1195_d_n8: f64 = (p.p87 * (s.dn[743][8] * ddt_scale));let eq30_e1195_d_n9: f64 = (p.p87 * (s.dn[743][9] * ddt_scale));let eq30_e1195_d_n10: f64 = (p.p87 * (s.dn[743][10] * ddt_scale));let eq30_e1195_d_n11: f64 = (p.p87 * (s.dn[743][11] * ddt_scale));let eq30_e1195_d_n12: f64 = (p.p87 * (s.dn[743][12] * ddt_scale));let eq30_e1195_d_n13: f64 = (p.p87 * (s.dn[743][13] * ddt_scale));let eq30_e1195_d_n14: f64 = (p.p87 * (s.dn[743][14] * ddt_scale));let eq30_e1195_d_n15: f64 = (p.p87 * (s.dn[743][15] * ddt_scale));let eq30_e1195_d_n16: f64 = (p.p87 * (s.dn[743][16] * ddt_scale));let eq30_e1195_d_n17: f64 = (p.p87 * (s.dn[743][17] * ddt_scale));let eq30_e1195_d_b0: f64 = (p.p87 * (s.db[743][0] * ddt_scale));let eq30_e1195_d_b1: f64 = (p.p87 * (s.db[743][1] * ddt_scale));let eq30_e1195_d_b2: f64 = (p.p87 * (s.db[743][2] * ddt_scale));let eq30_e1195_d_b3: f64 = (p.p87 * (s.db[743][3] * ddt_scale));let eq30_e1195_d_b4: f64 = (p.p87 * (s.db[743][4] * ddt_scale));let eq30_e1195_d_b5: f64 = (p.p87 * (s.db[743][5] * ddt_scale));let eq30_e1195_d_b6: f64 = (p.p87 * (s.db[743][6] * ddt_scale));let eq30_e1195_d_b7: f64 = (p.p87 * (s.db[743][7] * ddt_scale));let eq30_e1195_d_b8: f64 = (p.p87 * (s.db[743][8] * ddt_scale));let eq30_e1195_d_b9: f64 = (p.p87 * (s.db[743][9] * ddt_scale));let eq30_e1195_d_b10: f64 = (p.p87 * (s.db[743][10] * ddt_scale));let eq30_e1195_d_b11: f64 = (p.p87 * (s.db[743][11] * ddt_scale));let eq30_value: f64 = eq30_e1195;let eq30_node_derivatives: [f64; 18] = [eq30_e1195_d_n0, eq30_e1195_d_n1, eq30_e1195_d_n2, eq30_e1195_d_n3, eq30_e1195_d_n4, eq30_e1195_d_n5, eq30_e1195_d_n6, eq30_e1195_d_n7, eq30_e1195_d_n8, eq30_e1195_d_n9, eq30_e1195_d_n10, eq30_e1195_d_n11, eq30_e1195_d_n12, eq30_e1195_d_n13, eq30_e1195_d_n14, eq30_e1195_d_n15, eq30_e1195_d_n16, eq30_e1195_d_n17];let eq30_branch_derivatives: [f64; 12] = [eq30_e1195_d_b0, eq30_e1195_d_b1, eq30_e1195_d_b2, eq30_e1195_d_b3, eq30_e1195_d_b4, eq30_e1195_d_b5, eq30_e1195_d_b6, eq30_e1195_d_b7, eq30_e1195_d_b8, eq30_e1195_d_b9, eq30_e1195_d_b10, eq30_e1195_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(2),
            multiplicity * (eq30_value),
            &eq30_node_derivatives,
            &eq30_branch_derivatives,
            multiplicity,
        );let eq31_e1198: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, s.v[742]);let eq31_e1199: f64 = (p.p87 * eq31_e1198);let eq31_e1199_d_n0: f64 = (p.p87 * (s.dn[742][0] * ddt_scale));let eq31_e1199_d_n1: f64 = (p.p87 * (s.dn[742][1] * ddt_scale));let eq31_e1199_d_n2: f64 = (p.p87 * (s.dn[742][2] * ddt_scale));let eq31_e1199_d_n3: f64 = (p.p87 * (s.dn[742][3] * ddt_scale));let eq31_e1199_d_n4: f64 = (p.p87 * (s.dn[742][4] * ddt_scale));let eq31_e1199_d_n5: f64 = (p.p87 * (s.dn[742][5] * ddt_scale));let eq31_e1199_d_n6: f64 = (p.p87 * (s.dn[742][6] * ddt_scale));let eq31_e1199_d_n7: f64 = (p.p87 * (s.dn[742][7] * ddt_scale));let eq31_e1199_d_n8: f64 = (p.p87 * (s.dn[742][8] * ddt_scale));let eq31_e1199_d_n9: f64 = (p.p87 * (s.dn[742][9] * ddt_scale));let eq31_e1199_d_n10: f64 = (p.p87 * (s.dn[742][10] * ddt_scale));let eq31_e1199_d_n11: f64 = (p.p87 * (s.dn[742][11] * ddt_scale));let eq31_e1199_d_n12: f64 = (p.p87 * (s.dn[742][12] * ddt_scale));let eq31_e1199_d_n13: f64 = (p.p87 * (s.dn[742][13] * ddt_scale));let eq31_e1199_d_n14: f64 = (p.p87 * (s.dn[742][14] * ddt_scale));let eq31_e1199_d_n15: f64 = (p.p87 * (s.dn[742][15] * ddt_scale));let eq31_e1199_d_n16: f64 = (p.p87 * (s.dn[742][16] * ddt_scale));let eq31_e1199_d_n17: f64 = (p.p87 * (s.dn[742][17] * ddt_scale));let eq31_e1199_d_b0: f64 = (p.p87 * (s.db[742][0] * ddt_scale));let eq31_e1199_d_b1: f64 = (p.p87 * (s.db[742][1] * ddt_scale));let eq31_e1199_d_b2: f64 = (p.p87 * (s.db[742][2] * ddt_scale));let eq31_e1199_d_b3: f64 = (p.p87 * (s.db[742][3] * ddt_scale));let eq31_e1199_d_b4: f64 = (p.p87 * (s.db[742][4] * ddt_scale));let eq31_e1199_d_b5: f64 = (p.p87 * (s.db[742][5] * ddt_scale));let eq31_e1199_d_b6: f64 = (p.p87 * (s.db[742][6] * ddt_scale));let eq31_e1199_d_b7: f64 = (p.p87 * (s.db[742][7] * ddt_scale));let eq31_e1199_d_b8: f64 = (p.p87 * (s.db[742][8] * ddt_scale));let eq31_e1199_d_b9: f64 = (p.p87 * (s.db[742][9] * ddt_scale));let eq31_e1199_d_b10: f64 = (p.p87 * (s.db[742][10] * ddt_scale));let eq31_e1199_d_b11: f64 = (p.p87 * (s.db[742][11] * ddt_scale));let eq31_value: f64 = eq31_e1199;let eq31_node_derivatives: [f64; 18] = [eq31_e1199_d_n0, eq31_e1199_d_n1, eq31_e1199_d_n2, eq31_e1199_d_n3, eq31_e1199_d_n4, eq31_e1199_d_n5, eq31_e1199_d_n6, eq31_e1199_d_n7, eq31_e1199_d_n8, eq31_e1199_d_n9, eq31_e1199_d_n10, eq31_e1199_d_n11, eq31_e1199_d_n12, eq31_e1199_d_n13, eq31_e1199_d_n14, eq31_e1199_d_n15, eq31_e1199_d_n16, eq31_e1199_d_n17];let eq31_branch_derivatives: [f64; 12] = [eq31_e1199_d_b0, eq31_e1199_d_b1, eq31_e1199_d_b2, eq31_e1199_d_b3, eq31_e1199_d_b4, eq31_e1199_d_b5, eq31_e1199_d_b6, eq31_e1199_d_b7, eq31_e1199_d_b8, eq31_e1199_d_b9, eq31_e1199_d_b10, eq31_e1199_d_b11];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq31_value),
            &eq31_node_derivatives,
            &eq31_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_13(
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
        let eq32_e1202: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, s.v[744]);let eq32_e1203: f64 = (p.p87 * eq32_e1202);let eq32_e1203_d_n0: f64 = (p.p87 * (s.dn[744][0] * ddt_scale));let eq32_e1203_d_n1: f64 = (p.p87 * (s.dn[744][1] * ddt_scale));let eq32_e1203_d_n2: f64 = (p.p87 * (s.dn[744][2] * ddt_scale));let eq32_e1203_d_n3: f64 = (p.p87 * (s.dn[744][3] * ddt_scale));let eq32_e1203_d_n4: f64 = (p.p87 * (s.dn[744][4] * ddt_scale));let eq32_e1203_d_n5: f64 = (p.p87 * (s.dn[744][5] * ddt_scale));let eq32_e1203_d_n6: f64 = (p.p87 * (s.dn[744][6] * ddt_scale));let eq32_e1203_d_n7: f64 = (p.p87 * (s.dn[744][7] * ddt_scale));let eq32_e1203_d_n8: f64 = (p.p87 * (s.dn[744][8] * ddt_scale));let eq32_e1203_d_n9: f64 = (p.p87 * (s.dn[744][9] * ddt_scale));let eq32_e1203_d_n10: f64 = (p.p87 * (s.dn[744][10] * ddt_scale));let eq32_e1203_d_n11: f64 = (p.p87 * (s.dn[744][11] * ddt_scale));let eq32_e1203_d_n12: f64 = (p.p87 * (s.dn[744][12] * ddt_scale));let eq32_e1203_d_n13: f64 = (p.p87 * (s.dn[744][13] * ddt_scale));let eq32_e1203_d_n14: f64 = (p.p87 * (s.dn[744][14] * ddt_scale));let eq32_e1203_d_n15: f64 = (p.p87 * (s.dn[744][15] * ddt_scale));let eq32_e1203_d_n16: f64 = (p.p87 * (s.dn[744][16] * ddt_scale));let eq32_e1203_d_n17: f64 = (p.p87 * (s.dn[744][17] * ddt_scale));let eq32_e1203_d_b0: f64 = (p.p87 * (s.db[744][0] * ddt_scale));let eq32_e1203_d_b1: f64 = (p.p87 * (s.db[744][1] * ddt_scale));let eq32_e1203_d_b2: f64 = (p.p87 * (s.db[744][2] * ddt_scale));let eq32_e1203_d_b3: f64 = (p.p87 * (s.db[744][3] * ddt_scale));let eq32_e1203_d_b4: f64 = (p.p87 * (s.db[744][4] * ddt_scale));let eq32_e1203_d_b5: f64 = (p.p87 * (s.db[744][5] * ddt_scale));let eq32_e1203_d_b6: f64 = (p.p87 * (s.db[744][6] * ddt_scale));let eq32_e1203_d_b7: f64 = (p.p87 * (s.db[744][7] * ddt_scale));let eq32_e1203_d_b8: f64 = (p.p87 * (s.db[744][8] * ddt_scale));let eq32_e1203_d_b9: f64 = (p.p87 * (s.db[744][9] * ddt_scale));let eq32_e1203_d_b10: f64 = (p.p87 * (s.db[744][10] * ddt_scale));let eq32_e1203_d_b11: f64 = (p.p87 * (s.db[744][11] * ddt_scale));let eq32_value: f64 = eq32_e1203;let eq32_node_derivatives: [f64; 18] = [eq32_e1203_d_n0, eq32_e1203_d_n1, eq32_e1203_d_n2, eq32_e1203_d_n3, eq32_e1203_d_n4, eq32_e1203_d_n5, eq32_e1203_d_n6, eq32_e1203_d_n7, eq32_e1203_d_n8, eq32_e1203_d_n9, eq32_e1203_d_n10, eq32_e1203_d_n11, eq32_e1203_d_n12, eq32_e1203_d_n13, eq32_e1203_d_n14, eq32_e1203_d_n15, eq32_e1203_d_n16, eq32_e1203_d_n17];let eq32_branch_derivatives: [f64; 12] = [eq32_e1203_d_b0, eq32_e1203_d_b1, eq32_e1203_d_b2, eq32_e1203_d_b3, eq32_e1203_d_b4, eq32_e1203_d_b5, eq32_e1203_d_b6, eq32_e1203_d_b7, eq32_e1203_d_b8, eq32_e1203_d_b9, eq32_e1203_d_b10, eq32_e1203_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(2),
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );let eq33_e1205: f64 = (-p.p87);let eq33_e1207: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, s.v[299]);let eq33_e1208: f64 = (eq33_e1205 * eq33_e1207);let eq33_e1208_d_n0: f64 = (eq33_e1205 * (s.dn[299][0] * ddt_scale));let eq33_e1208_d_n1: f64 = (eq33_e1205 * (s.dn[299][1] * ddt_scale));let eq33_e1208_d_n2: f64 = (eq33_e1205 * (s.dn[299][2] * ddt_scale));let eq33_e1208_d_n3: f64 = (eq33_e1205 * (s.dn[299][3] * ddt_scale));let eq33_e1208_d_n4: f64 = (eq33_e1205 * (s.dn[299][4] * ddt_scale));let eq33_e1208_d_n5: f64 = (eq33_e1205 * (s.dn[299][5] * ddt_scale));let eq33_e1208_d_n6: f64 = (eq33_e1205 * (s.dn[299][6] * ddt_scale));let eq33_e1208_d_n7: f64 = (eq33_e1205 * (s.dn[299][7] * ddt_scale));let eq33_e1208_d_n8: f64 = (eq33_e1205 * (s.dn[299][8] * ddt_scale));let eq33_e1208_d_n9: f64 = (eq33_e1205 * (s.dn[299][9] * ddt_scale));let eq33_e1208_d_n10: f64 = (eq33_e1205 * (s.dn[299][10] * ddt_scale));let eq33_e1208_d_n11: f64 = (eq33_e1205 * (s.dn[299][11] * ddt_scale));let eq33_e1208_d_n12: f64 = (eq33_e1205 * (s.dn[299][12] * ddt_scale));let eq33_e1208_d_n13: f64 = (eq33_e1205 * (s.dn[299][13] * ddt_scale));let eq33_e1208_d_n14: f64 = (eq33_e1205 * (s.dn[299][14] * ddt_scale));let eq33_e1208_d_n15: f64 = (eq33_e1205 * (s.dn[299][15] * ddt_scale));let eq33_e1208_d_n16: f64 = (eq33_e1205 * (s.dn[299][16] * ddt_scale));let eq33_e1208_d_n17: f64 = (eq33_e1205 * (s.dn[299][17] * ddt_scale));let eq33_e1208_d_b0: f64 = (eq33_e1205 * (s.db[299][0] * ddt_scale));let eq33_e1208_d_b1: f64 = (eq33_e1205 * (s.db[299][1] * ddt_scale));let eq33_e1208_d_b2: f64 = (eq33_e1205 * (s.db[299][2] * ddt_scale));let eq33_e1208_d_b3: f64 = (eq33_e1205 * (s.db[299][3] * ddt_scale));let eq33_e1208_d_b4: f64 = (eq33_e1205 * (s.db[299][4] * ddt_scale));let eq33_e1208_d_b5: f64 = (eq33_e1205 * (s.db[299][5] * ddt_scale));let eq33_e1208_d_b6: f64 = (eq33_e1205 * (s.db[299][6] * ddt_scale));let eq33_e1208_d_b7: f64 = (eq33_e1205 * (s.db[299][7] * ddt_scale));let eq33_e1208_d_b8: f64 = (eq33_e1205 * (s.db[299][8] * ddt_scale));let eq33_e1208_d_b9: f64 = (eq33_e1205 * (s.db[299][9] * ddt_scale));let eq33_e1208_d_b10: f64 = (eq33_e1205 * (s.db[299][10] * ddt_scale));let eq33_e1208_d_b11: f64 = (eq33_e1205 * (s.db[299][11] * ddt_scale));let eq33_value: f64 = eq33_e1208;let eq33_node_derivatives: [f64; 18] = [eq33_e1208_d_n0, eq33_e1208_d_n1, eq33_e1208_d_n2, eq33_e1208_d_n3, eq33_e1208_d_n4, eq33_e1208_d_n5, eq33_e1208_d_n6, eq33_e1208_d_n7, eq33_e1208_d_n8, eq33_e1208_d_n9, eq33_e1208_d_n10, eq33_e1208_d_n11, eq33_e1208_d_n12, eq33_e1208_d_n13, eq33_e1208_d_n14, eq33_e1208_d_n15, eq33_e1208_d_n16, eq33_e1208_d_n17];let eq33_branch_derivatives: [f64; 12] = [eq33_e1208_d_b0, eq33_e1208_d_b1, eq33_e1208_d_b2, eq33_e1208_d_b3, eq33_e1208_d_b4, eq33_e1208_d_b5, eq33_e1208_d_b6, eq33_e1208_d_b7, eq33_e1208_d_b8, eq33_e1208_d_b9, eq33_e1208_d_b10, eq33_e1208_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(0),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
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
        let nv14 = ctx.node_voltage(nodes[14]);let eq34_e1210: f64 = (-p.p87);let eq34_e1212: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, s.v[301]);let eq34_e1213: f64 = (eq34_e1210 * eq34_e1212);let eq34_e1213_d_n0: f64 = (eq34_e1210 * (s.dn[301][0] * ddt_scale));let eq34_e1213_d_n1: f64 = (eq34_e1210 * (s.dn[301][1] * ddt_scale));let eq34_e1213_d_n2: f64 = (eq34_e1210 * (s.dn[301][2] * ddt_scale));let eq34_e1213_d_n3: f64 = (eq34_e1210 * (s.dn[301][3] * ddt_scale));let eq34_e1213_d_n4: f64 = (eq34_e1210 * (s.dn[301][4] * ddt_scale));let eq34_e1213_d_n5: f64 = (eq34_e1210 * (s.dn[301][5] * ddt_scale));let eq34_e1213_d_n6: f64 = (eq34_e1210 * (s.dn[301][6] * ddt_scale));let eq34_e1213_d_n7: f64 = (eq34_e1210 * (s.dn[301][7] * ddt_scale));let eq34_e1213_d_n8: f64 = (eq34_e1210 * (s.dn[301][8] * ddt_scale));let eq34_e1213_d_n9: f64 = (eq34_e1210 * (s.dn[301][9] * ddt_scale));let eq34_e1213_d_n10: f64 = (eq34_e1210 * (s.dn[301][10] * ddt_scale));let eq34_e1213_d_n11: f64 = (eq34_e1210 * (s.dn[301][11] * ddt_scale));let eq34_e1213_d_n12: f64 = (eq34_e1210 * (s.dn[301][12] * ddt_scale));let eq34_e1213_d_n13: f64 = (eq34_e1210 * (s.dn[301][13] * ddt_scale));let eq34_e1213_d_n14: f64 = (eq34_e1210 * (s.dn[301][14] * ddt_scale));let eq34_e1213_d_n15: f64 = (eq34_e1210 * (s.dn[301][15] * ddt_scale));let eq34_e1213_d_n16: f64 = (eq34_e1210 * (s.dn[301][16] * ddt_scale));let eq34_e1213_d_n17: f64 = (eq34_e1210 * (s.dn[301][17] * ddt_scale));let eq34_e1213_d_b0: f64 = (eq34_e1210 * (s.db[301][0] * ddt_scale));let eq34_e1213_d_b1: f64 = (eq34_e1210 * (s.db[301][1] * ddt_scale));let eq34_e1213_d_b2: f64 = (eq34_e1210 * (s.db[301][2] * ddt_scale));let eq34_e1213_d_b3: f64 = (eq34_e1210 * (s.db[301][3] * ddt_scale));let eq34_e1213_d_b4: f64 = (eq34_e1210 * (s.db[301][4] * ddt_scale));let eq34_e1213_d_b5: f64 = (eq34_e1210 * (s.db[301][5] * ddt_scale));let eq34_e1213_d_b6: f64 = (eq34_e1210 * (s.db[301][6] * ddt_scale));let eq34_e1213_d_b7: f64 = (eq34_e1210 * (s.db[301][7] * ddt_scale));let eq34_e1213_d_b8: f64 = (eq34_e1210 * (s.db[301][8] * ddt_scale));let eq34_e1213_d_b9: f64 = (eq34_e1210 * (s.db[301][9] * ddt_scale));let eq34_e1213_d_b10: f64 = (eq34_e1210 * (s.db[301][10] * ddt_scale));let eq34_e1213_d_b11: f64 = (eq34_e1210 * (s.db[301][11] * ddt_scale));let eq34_value: f64 = eq34_e1213;let eq34_node_derivatives: [f64; 18] = [eq34_e1213_d_n0, eq34_e1213_d_n1, eq34_e1213_d_n2, eq34_e1213_d_n3, eq34_e1213_d_n4, eq34_e1213_d_n5, eq34_e1213_d_n6, eq34_e1213_d_n7, eq34_e1213_d_n8, eq34_e1213_d_n9, eq34_e1213_d_n10, eq34_e1213_d_n11, eq34_e1213_d_n12, eq34_e1213_d_n13, eq34_e1213_d_n14, eq34_e1213_d_n15, eq34_e1213_d_n16, eq34_e1213_d_n17];let eq34_branch_derivatives: [f64; 12] = [eq34_e1213_d_b0, eq34_e1213_d_b1, eq34_e1213_d_b2, eq34_e1213_d_b3, eq34_e1213_d_b4, eq34_e1213_d_b5, eq34_e1213_d_b6, eq34_e1213_d_b7, eq34_e1213_d_b8, eq34_e1213_d_b9, eq34_e1213_d_b10, eq34_e1213_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(2),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
            multiplicity,
        );let eq36_e1224: f64 = (nv14 - 0.0);let eq36_value: f64 = eq36_e1224;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (eq36_value),
            14,
            multiplicity * (1.0),
        );let eq39_e1239: f64 = (s.v[951] * (nv14 - 0.0));let eq39_e1239_d_n0: f64 = (s.dn[951][0] * (nv14 - 0.0));let eq39_e1239_d_n1: f64 = (s.dn[951][1] * (nv14 - 0.0));let eq39_e1239_d_n2: f64 = (s.dn[951][2] * (nv14 - 0.0));let eq39_e1239_d_n3: f64 = (s.dn[951][3] * (nv14 - 0.0));let eq39_e1239_d_n4: f64 = (s.dn[951][4] * (nv14 - 0.0));let eq39_e1239_d_n5: f64 = (s.dn[951][5] * (nv14 - 0.0));let eq39_e1239_d_n6: f64 = (s.dn[951][6] * (nv14 - 0.0));let eq39_e1239_d_n7: f64 = (s.dn[951][7] * (nv14 - 0.0));let eq39_e1239_d_n8: f64 = (s.dn[951][8] * (nv14 - 0.0));let eq39_e1239_d_n9: f64 = (s.dn[951][9] * (nv14 - 0.0));let eq39_e1239_d_n10: f64 = (s.dn[951][10] * (nv14 - 0.0));let eq39_e1239_d_n11: f64 = (s.dn[951][11] * (nv14 - 0.0));let eq39_e1239_d_n12: f64 = (s.dn[951][12] * (nv14 - 0.0));let eq39_e1239_d_n13: f64 = (s.dn[951][13] * (nv14 - 0.0));let eq39_e1239_d_n14: f64 = ((s.dn[951][14] * (nv14 - 0.0)) + s.v[951]);let eq39_e1239_d_n15: f64 = (s.dn[951][15] * (nv14 - 0.0));let eq39_e1239_d_n16: f64 = (s.dn[951][16] * (nv14 - 0.0));let eq39_e1239_d_n17: f64 = (s.dn[951][17] * (nv14 - 0.0));let eq39_e1239_d_b0: f64 = (s.db[951][0] * (nv14 - 0.0));let eq39_e1239_d_b1: f64 = (s.db[951][1] * (nv14 - 0.0));let eq39_e1239_d_b2: f64 = (s.db[951][2] * (nv14 - 0.0));let eq39_e1239_d_b3: f64 = (s.db[951][3] * (nv14 - 0.0));let eq39_e1239_d_b4: f64 = (s.db[951][4] * (nv14 - 0.0));let eq39_e1239_d_b5: f64 = (s.db[951][5] * (nv14 - 0.0));let eq39_e1239_d_b6: f64 = (s.db[951][6] * (nv14 - 0.0));let eq39_e1239_d_b7: f64 = (s.db[951][7] * (nv14 - 0.0));let eq39_e1239_d_b8: f64 = (s.db[951][8] * (nv14 - 0.0));let eq39_e1239_d_b9: f64 = (s.db[951][9] * (nv14 - 0.0));let eq39_e1239_d_b10: f64 = (s.db[951][10] * (nv14 - 0.0));let eq39_e1239_d_b11: f64 = (s.db[951][11] * (nv14 - 0.0));let eq39_value: f64 = eq39_e1239;let eq39_node_derivatives: [f64; 18] = [eq39_e1239_d_n0, eq39_e1239_d_n1, eq39_e1239_d_n2, eq39_e1239_d_n3, eq39_e1239_d_n4, eq39_e1239_d_n5, eq39_e1239_d_n6, eq39_e1239_d_n7, eq39_e1239_d_n8, eq39_e1239_d_n9, eq39_e1239_d_n10, eq39_e1239_d_n11, eq39_e1239_d_n12, eq39_e1239_d_n13, eq39_e1239_d_n14, eq39_e1239_d_n15, eq39_e1239_d_n16, eq39_e1239_d_n17];let eq39_branch_derivatives: [f64; 12] = [eq39_e1239_d_b0, eq39_e1239_d_b1, eq39_e1239_d_b2, eq39_e1239_d_b3, eq39_e1239_d_b4, eq39_e1239_d_b5, eq39_e1239_d_b6, eq39_e1239_d_b7, eq39_e1239_d_b8, eq39_e1239_d_b9, eq39_e1239_d_b10, eq39_e1239_d_b11];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq39_value),
            &eq39_node_derivatives,
            &eq39_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_15(
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
        let nv14 = ctx.node_voltage(nodes[14]);let eq40_e1242: f64 = ((nv14 - 0.0) * s.v[954]);let eq40_e1242_d_n0: f64 = ((nv14 - 0.0) * s.dn[954][0]);let eq40_e1242_d_n1: f64 = ((nv14 - 0.0) * s.dn[954][1]);let eq40_e1242_d_n2: f64 = ((nv14 - 0.0) * s.dn[954][2]);let eq40_e1242_d_n3: f64 = ((nv14 - 0.0) * s.dn[954][3]);let eq40_e1242_d_n4: f64 = ((nv14 - 0.0) * s.dn[954][4]);let eq40_e1242_d_n5: f64 = ((nv14 - 0.0) * s.dn[954][5]);let eq40_e1242_d_n6: f64 = ((nv14 - 0.0) * s.dn[954][6]);let eq40_e1242_d_n7: f64 = ((nv14 - 0.0) * s.dn[954][7]);let eq40_e1242_d_n8: f64 = ((nv14 - 0.0) * s.dn[954][8]);let eq40_e1242_d_n9: f64 = ((nv14 - 0.0) * s.dn[954][9]);let eq40_e1242_d_n10: f64 = ((nv14 - 0.0) * s.dn[954][10]);let eq40_e1242_d_n11: f64 = ((nv14 - 0.0) * s.dn[954][11]);let eq40_e1242_d_n12: f64 = ((nv14 - 0.0) * s.dn[954][12]);let eq40_e1242_d_n13: f64 = ((nv14 - 0.0) * s.dn[954][13]);let eq40_e1242_d_n14: f64 = (s.v[954] + ((nv14 - 0.0) * s.dn[954][14]));let eq40_e1242_d_n15: f64 = ((nv14 - 0.0) * s.dn[954][15]);let eq40_e1242_d_n16: f64 = ((nv14 - 0.0) * s.dn[954][16]);let eq40_e1242_d_n17: f64 = ((nv14 - 0.0) * s.dn[954][17]);let eq40_e1242_d_b0: f64 = ((nv14 - 0.0) * s.db[954][0]);let eq40_e1242_d_b1: f64 = ((nv14 - 0.0) * s.db[954][1]);let eq40_e1242_d_b2: f64 = ((nv14 - 0.0) * s.db[954][2]);let eq40_e1242_d_b3: f64 = ((nv14 - 0.0) * s.db[954][3]);let eq40_e1242_d_b4: f64 = ((nv14 - 0.0) * s.db[954][4]);let eq40_e1242_d_b5: f64 = ((nv14 - 0.0) * s.db[954][5]);let eq40_e1242_d_b6: f64 = ((nv14 - 0.0) * s.db[954][6]);let eq40_e1242_d_b7: f64 = ((nv14 - 0.0) * s.db[954][7]);let eq40_e1242_d_b8: f64 = ((nv14 - 0.0) * s.db[954][8]);let eq40_e1242_d_b9: f64 = ((nv14 - 0.0) * s.db[954][9]);let eq40_e1242_d_b10: f64 = ((nv14 - 0.0) * s.db[954][10]);let eq40_e1242_d_b11: f64 = ((nv14 - 0.0) * s.db[954][11]);let eq40_e1243: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, eq40_e1242);let eq40_value: f64 = eq40_e1243;let eq40_node_derivatives: [f64; 18] = [(eq40_e1242_d_n0 * ddt_scale), (eq40_e1242_d_n1 * ddt_scale), (eq40_e1242_d_n2 * ddt_scale), (eq40_e1242_d_n3 * ddt_scale), (eq40_e1242_d_n4 * ddt_scale), (eq40_e1242_d_n5 * ddt_scale), (eq40_e1242_d_n6 * ddt_scale), (eq40_e1242_d_n7 * ddt_scale), (eq40_e1242_d_n8 * ddt_scale), (eq40_e1242_d_n9 * ddt_scale), (eq40_e1242_d_n10 * ddt_scale), (eq40_e1242_d_n11 * ddt_scale), (eq40_e1242_d_n12 * ddt_scale), (eq40_e1242_d_n13 * ddt_scale), (eq40_e1242_d_n14 * ddt_scale), (eq40_e1242_d_n15 * ddt_scale), (eq40_e1242_d_n16 * ddt_scale), (eq40_e1242_d_n17 * ddt_scale)];let eq40_branch_derivatives: [f64; 12] = [(eq40_e1242_d_b0 * ddt_scale), (eq40_e1242_d_b1 * ddt_scale), (eq40_e1242_d_b2 * ddt_scale), (eq40_e1242_d_b3 * ddt_scale), (eq40_e1242_d_b4 * ddt_scale), (eq40_e1242_d_b5 * ddt_scale), (eq40_e1242_d_b6 * ddt_scale), (eq40_e1242_d_b7 * ddt_scale), (eq40_e1242_d_b8 * ddt_scale), (eq40_e1242_d_b9 * ddt_scale), (eq40_e1242_d_b10 * ddt_scale), (eq40_e1242_d_b11 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq40_value),
            &eq40_node_derivatives,
            &eq40_branch_derivatives,
            multiplicity,
        );let eq41_e1246: f64 = ((nv14 - 0.0) * s.v[955]);let eq41_e1246_d_n0: f64 = ((nv14 - 0.0) * s.dn[955][0]);let eq41_e1246_d_n1: f64 = ((nv14 - 0.0) * s.dn[955][1]);let eq41_e1246_d_n2: f64 = ((nv14 - 0.0) * s.dn[955][2]);let eq41_e1246_d_n3: f64 = ((nv14 - 0.0) * s.dn[955][3]);let eq41_e1246_d_n4: f64 = ((nv14 - 0.0) * s.dn[955][4]);let eq41_e1246_d_n5: f64 = ((nv14 - 0.0) * s.dn[955][5]);let eq41_e1246_d_n6: f64 = ((nv14 - 0.0) * s.dn[955][6]);let eq41_e1246_d_n7: f64 = ((nv14 - 0.0) * s.dn[955][7]);let eq41_e1246_d_n8: f64 = ((nv14 - 0.0) * s.dn[955][8]);let eq41_e1246_d_n9: f64 = ((nv14 - 0.0) * s.dn[955][9]);let eq41_e1246_d_n10: f64 = ((nv14 - 0.0) * s.dn[955][10]);let eq41_e1246_d_n11: f64 = ((nv14 - 0.0) * s.dn[955][11]);let eq41_e1246_d_n12: f64 = ((nv14 - 0.0) * s.dn[955][12]);let eq41_e1246_d_n13: f64 = ((nv14 - 0.0) * s.dn[955][13]);let eq41_e1246_d_n14: f64 = (s.v[955] + ((nv14 - 0.0) * s.dn[955][14]));let eq41_e1246_d_n15: f64 = ((nv14 - 0.0) * s.dn[955][15]);let eq41_e1246_d_n16: f64 = ((nv14 - 0.0) * s.dn[955][16]);let eq41_e1246_d_n17: f64 = ((nv14 - 0.0) * s.dn[955][17]);let eq41_e1246_d_b0: f64 = ((nv14 - 0.0) * s.db[955][0]);let eq41_e1246_d_b1: f64 = ((nv14 - 0.0) * s.db[955][1]);let eq41_e1246_d_b2: f64 = ((nv14 - 0.0) * s.db[955][2]);let eq41_e1246_d_b3: f64 = ((nv14 - 0.0) * s.db[955][3]);let eq41_e1246_d_b4: f64 = ((nv14 - 0.0) * s.db[955][4]);let eq41_e1246_d_b5: f64 = ((nv14 - 0.0) * s.db[955][5]);let eq41_e1246_d_b6: f64 = ((nv14 - 0.0) * s.db[955][6]);let eq41_e1246_d_b7: f64 = ((nv14 - 0.0) * s.db[955][7]);let eq41_e1246_d_b8: f64 = ((nv14 - 0.0) * s.db[955][8]);let eq41_e1246_d_b9: f64 = ((nv14 - 0.0) * s.db[955][9]);let eq41_e1246_d_b10: f64 = ((nv14 - 0.0) * s.db[955][10]);let eq41_e1246_d_b11: f64 = ((nv14 - 0.0) * s.db[955][11]);let eq41_e1247: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, eq41_e1246);let eq41_value: f64 = eq41_e1247;let eq41_node_derivatives: [f64; 18] = [(eq41_e1246_d_n0 * ddt_scale), (eq41_e1246_d_n1 * ddt_scale), (eq41_e1246_d_n2 * ddt_scale), (eq41_e1246_d_n3 * ddt_scale), (eq41_e1246_d_n4 * ddt_scale), (eq41_e1246_d_n5 * ddt_scale), (eq41_e1246_d_n6 * ddt_scale), (eq41_e1246_d_n7 * ddt_scale), (eq41_e1246_d_n8 * ddt_scale), (eq41_e1246_d_n9 * ddt_scale), (eq41_e1246_d_n10 * ddt_scale), (eq41_e1246_d_n11 * ddt_scale), (eq41_e1246_d_n12 * ddt_scale), (eq41_e1246_d_n13 * ddt_scale), (eq41_e1246_d_n14 * ddt_scale), (eq41_e1246_d_n15 * ddt_scale), (eq41_e1246_d_n16 * ddt_scale), (eq41_e1246_d_n17 * ddt_scale)];let eq41_branch_derivatives: [f64; 12] = [(eq41_e1246_d_b0 * ddt_scale), (eq41_e1246_d_b1 * ddt_scale), (eq41_e1246_d_b2 * ddt_scale), (eq41_e1246_d_b3 * ddt_scale), (eq41_e1246_d_b4 * ddt_scale), (eq41_e1246_d_b5 * ddt_scale), (eq41_e1246_d_b6 * ddt_scale), (eq41_e1246_d_b7 * ddt_scale), (eq41_e1246_d_b8 * ddt_scale), (eq41_e1246_d_b9 * ddt_scale), (eq41_e1246_d_b10 * ddt_scale), (eq41_e1246_d_b11 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq41_value),
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_16(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);let nv6 = ctx.node_voltage(nodes[6]);let nv8 = ctx.node_voltage(nodes[8]);let nv10 = ctx.node_voltage(nodes[10]);
        let (eq47_e1293, eq47_e1293_d_n0, eq47_e1293_d_n1, eq47_e1293_d_n2, eq47_e1293_d_n3, eq47_e1293_d_n4, eq47_e1293_d_n5, eq47_e1293_d_n6, eq47_e1293_d_n7, eq47_e1293_d_n8, eq47_e1293_d_n9, eq47_e1293_d_n10, eq47_e1293_d_n11, eq47_e1293_d_n12, eq47_e1293_d_n13, eq47_e1293_d_n14, eq47_e1293_d_n15, eq47_e1293_d_n16, eq47_e1293_d_n17, eq47_e1293_d_b0, eq47_e1293_d_b1, eq47_e1293_d_b2, eq47_e1293_d_b3, eq47_e1293_d_b4, eq47_e1293_d_b5, eq47_e1293_d_b6, eq47_e1293_d_b7, eq47_e1293_d_b8, eq47_e1293_d_b9, eq47_e1293_d_b10, eq47_e1293_d_b11,) = {
    if s.b[3408] {
        let eq47_e1291: f64 = (s.v[643] * (nv1 - nv6));let eq47_e1291_d_n0: f64 = (s.dn[643][0] * (nv1 - nv6));let eq47_e1291_d_n1: f64 = ((s.dn[643][1] * (nv1 - nv6)) + s.v[643]);let eq47_e1291_d_n2: f64 = (s.dn[643][2] * (nv1 - nv6));let eq47_e1291_d_n3: f64 = (s.dn[643][3] * (nv1 - nv6));let eq47_e1291_d_n4: f64 = (s.dn[643][4] * (nv1 - nv6));let eq47_e1291_d_n5: f64 = (s.dn[643][5] * (nv1 - nv6));let eq47_e1291_d_n6: f64 = ((s.dn[643][6] * (nv1 - nv6)) + (-s.v[643]));let eq47_e1291_d_n7: f64 = (s.dn[643][7] * (nv1 - nv6));let eq47_e1291_d_n8: f64 = (s.dn[643][8] * (nv1 - nv6));let eq47_e1291_d_n9: f64 = (s.dn[643][9] * (nv1 - nv6));let eq47_e1291_d_n10: f64 = (s.dn[643][10] * (nv1 - nv6));let eq47_e1291_d_n11: f64 = (s.dn[643][11] * (nv1 - nv6));let eq47_e1291_d_n12: f64 = (s.dn[643][12] * (nv1 - nv6));let eq47_e1291_d_n13: f64 = (s.dn[643][13] * (nv1 - nv6));let eq47_e1291_d_n14: f64 = (s.dn[643][14] * (nv1 - nv6));let eq47_e1291_d_n15: f64 = (s.dn[643][15] * (nv1 - nv6));let eq47_e1291_d_n16: f64 = (s.dn[643][16] * (nv1 - nv6));let eq47_e1291_d_n17: f64 = (s.dn[643][17] * (nv1 - nv6));let eq47_e1291_d_b0: f64 = (s.db[643][0] * (nv1 - nv6));let eq47_e1291_d_b1: f64 = (s.db[643][1] * (nv1 - nv6));let eq47_e1291_d_b2: f64 = (s.db[643][2] * (nv1 - nv6));let eq47_e1291_d_b3: f64 = (s.db[643][3] * (nv1 - nv6));let eq47_e1291_d_b4: f64 = (s.db[643][4] * (nv1 - nv6));let eq47_e1291_d_b5: f64 = (s.db[643][5] * (nv1 - nv6));let eq47_e1291_d_b6: f64 = (s.db[643][6] * (nv1 - nv6));let eq47_e1291_d_b7: f64 = (s.db[643][7] * (nv1 - nv6));let eq47_e1291_d_b8: f64 = (s.db[643][8] * (nv1 - nv6));let eq47_e1291_d_b9: f64 = (s.db[643][9] * (nv1 - nv6));let eq47_e1291_d_b10: f64 = (s.db[643][10] * (nv1 - nv6));let eq47_e1291_d_b11: f64 = (s.db[643][11] * (nv1 - nv6));
        (eq47_e1291, eq47_e1291_d_n0, eq47_e1291_d_n1, eq47_e1291_d_n2, eq47_e1291_d_n3, eq47_e1291_d_n4, eq47_e1291_d_n5, eq47_e1291_d_n6, eq47_e1291_d_n7, eq47_e1291_d_n8, eq47_e1291_d_n9, eq47_e1291_d_n10, eq47_e1291_d_n11, eq47_e1291_d_n12, eq47_e1291_d_n13, eq47_e1291_d_n14, eq47_e1291_d_n15, eq47_e1291_d_n16, eq47_e1291_d_n17, eq47_e1291_d_b0, eq47_e1291_d_b1, eq47_e1291_d_b2, eq47_e1291_d_b3, eq47_e1291_d_b4, eq47_e1291_d_b5, eq47_e1291_d_b6, eq47_e1291_d_b7, eq47_e1291_d_b8, eq47_e1291_d_b9, eq47_e1291_d_b10, eq47_e1291_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e1293;let eq47_node_derivatives: [f64; 18] = [eq47_e1293_d_n0, eq47_e1293_d_n1, eq47_e1293_d_n2, eq47_e1293_d_n3, eq47_e1293_d_n4, eq47_e1293_d_n5, eq47_e1293_d_n6, eq47_e1293_d_n7, eq47_e1293_d_n8, eq47_e1293_d_n9, eq47_e1293_d_n10, eq47_e1293_d_n11, eq47_e1293_d_n12, eq47_e1293_d_n13, eq47_e1293_d_n14, eq47_e1293_d_n15, eq47_e1293_d_n16, eq47_e1293_d_n17];let eq47_branch_derivatives: [f64; 12] = [eq47_e1293_d_b0, eq47_e1293_d_b1, eq47_e1293_d_b2, eq47_e1293_d_b3, eq47_e1293_d_b4, eq47_e1293_d_b5, eq47_e1293_d_b6, eq47_e1293_d_b7, eq47_e1293_d_b8, eq47_e1293_d_b9, eq47_e1293_d_b10, eq47_e1293_d_b11];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(6),
            multiplicity * (eq47_value),
            &eq47_node_derivatives,
            &eq47_branch_derivatives,
            multiplicity,
        );
        let (eq48_e1298,) = {
    if (!s.b[3408]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq48_value: f64 = eq48_e1298;
        stamper.stamp_potential_const_local(
            5,
            eq48_value,
        );
        let (eq49_e1304, eq49_e1304_d_n0, eq49_e1304_d_n1, eq49_e1304_d_n2, eq49_e1304_d_n3, eq49_e1304_d_n4, eq49_e1304_d_n5, eq49_e1304_d_n6, eq49_e1304_d_n7, eq49_e1304_d_n8, eq49_e1304_d_n9, eq49_e1304_d_n10, eq49_e1304_d_n11, eq49_e1304_d_n12, eq49_e1304_d_n13, eq49_e1304_d_n14, eq49_e1304_d_n15, eq49_e1304_d_n16, eq49_e1304_d_n17, eq49_e1304_d_b0, eq49_e1304_d_b1, eq49_e1304_d_b2, eq49_e1304_d_b3, eq49_e1304_d_b4, eq49_e1304_d_b5, eq49_e1304_d_b6, eq49_e1304_d_b7, eq49_e1304_d_b8, eq49_e1304_d_b9, eq49_e1304_d_b10, eq49_e1304_d_b11,) = {
    if (p.p52 != 0.0) {
        let eq49_e1302: f64 = (s.v[656] * (nv10 - nv8));let eq49_e1302_d_n0: f64 = (s.dn[656][0] * (nv10 - nv8));let eq49_e1302_d_n1: f64 = (s.dn[656][1] * (nv10 - nv8));let eq49_e1302_d_n2: f64 = (s.dn[656][2] * (nv10 - nv8));let eq49_e1302_d_n3: f64 = (s.dn[656][3] * (nv10 - nv8));let eq49_e1302_d_n4: f64 = (s.dn[656][4] * (nv10 - nv8));let eq49_e1302_d_n5: f64 = (s.dn[656][5] * (nv10 - nv8));let eq49_e1302_d_n6: f64 = (s.dn[656][6] * (nv10 - nv8));let eq49_e1302_d_n7: f64 = (s.dn[656][7] * (nv10 - nv8));let eq49_e1302_d_n8: f64 = ((s.dn[656][8] * (nv10 - nv8)) + (-s.v[656]));let eq49_e1302_d_n9: f64 = (s.dn[656][9] * (nv10 - nv8));let eq49_e1302_d_n10: f64 = ((s.dn[656][10] * (nv10 - nv8)) + s.v[656]);let eq49_e1302_d_n11: f64 = (s.dn[656][11] * (nv10 - nv8));let eq49_e1302_d_n12: f64 = (s.dn[656][12] * (nv10 - nv8));let eq49_e1302_d_n13: f64 = (s.dn[656][13] * (nv10 - nv8));let eq49_e1302_d_n14: f64 = (s.dn[656][14] * (nv10 - nv8));let eq49_e1302_d_n15: f64 = (s.dn[656][15] * (nv10 - nv8));let eq49_e1302_d_n16: f64 = (s.dn[656][16] * (nv10 - nv8));let eq49_e1302_d_n17: f64 = (s.dn[656][17] * (nv10 - nv8));let eq49_e1302_d_b0: f64 = (s.db[656][0] * (nv10 - nv8));let eq49_e1302_d_b1: f64 = (s.db[656][1] * (nv10 - nv8));let eq49_e1302_d_b2: f64 = (s.db[656][2] * (nv10 - nv8));let eq49_e1302_d_b3: f64 = (s.db[656][3] * (nv10 - nv8));let eq49_e1302_d_b4: f64 = (s.db[656][4] * (nv10 - nv8));let eq49_e1302_d_b5: f64 = (s.db[656][5] * (nv10 - nv8));let eq49_e1302_d_b6: f64 = (s.db[656][6] * (nv10 - nv8));let eq49_e1302_d_b7: f64 = (s.db[656][7] * (nv10 - nv8));let eq49_e1302_d_b8: f64 = (s.db[656][8] * (nv10 - nv8));let eq49_e1302_d_b9: f64 = (s.db[656][9] * (nv10 - nv8));let eq49_e1302_d_b10: f64 = (s.db[656][10] * (nv10 - nv8));let eq49_e1302_d_b11: f64 = (s.db[656][11] * (nv10 - nv8));
        (eq49_e1302, eq49_e1302_d_n0, eq49_e1302_d_n1, eq49_e1302_d_n2, eq49_e1302_d_n3, eq49_e1302_d_n4, eq49_e1302_d_n5, eq49_e1302_d_n6, eq49_e1302_d_n7, eq49_e1302_d_n8, eq49_e1302_d_n9, eq49_e1302_d_n10, eq49_e1302_d_n11, eq49_e1302_d_n12, eq49_e1302_d_n13, eq49_e1302_d_n14, eq49_e1302_d_n15, eq49_e1302_d_n16, eq49_e1302_d_n17, eq49_e1302_d_b0, eq49_e1302_d_b1, eq49_e1302_d_b2, eq49_e1302_d_b3, eq49_e1302_d_b4, eq49_e1302_d_b5, eq49_e1302_d_b6, eq49_e1302_d_b7, eq49_e1302_d_b8, eq49_e1302_d_b9, eq49_e1302_d_b10, eq49_e1302_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq49_value: f64 = eq49_e1304;let eq49_node_derivatives: [f64; 18] = [eq49_e1304_d_n0, eq49_e1304_d_n1, eq49_e1304_d_n2, eq49_e1304_d_n3, eq49_e1304_d_n4, eq49_e1304_d_n5, eq49_e1304_d_n6, eq49_e1304_d_n7, eq49_e1304_d_n8, eq49_e1304_d_n9, eq49_e1304_d_n10, eq49_e1304_d_n11, eq49_e1304_d_n12, eq49_e1304_d_n13, eq49_e1304_d_n14, eq49_e1304_d_n15, eq49_e1304_d_n16, eq49_e1304_d_n17];let eq49_branch_derivatives: [f64; 12] = [eq49_e1304_d_b0, eq49_e1304_d_b1, eq49_e1304_d_b2, eq49_e1304_d_b3, eq49_e1304_d_b4, eq49_e1304_d_b5, eq49_e1304_d_b6, eq49_e1304_d_b7, eq49_e1304_d_b8, eq49_e1304_d_b9, eq49_e1304_d_b10, eq49_e1304_d_b11];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(8),
            multiplicity * (eq49_value),
            &eq49_node_derivatives,
            &eq49_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_17(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);let nv8 = ctx.node_voltage(nodes[8]);let nv9 = ctx.node_voltage(nodes[9]);
        let (eq50_e1310, eq50_e1310_d_n0, eq50_e1310_d_n1, eq50_e1310_d_n2, eq50_e1310_d_n3, eq50_e1310_d_n4, eq50_e1310_d_n5, eq50_e1310_d_n6, eq50_e1310_d_n7, eq50_e1310_d_n8, eq50_e1310_d_n9, eq50_e1310_d_n10, eq50_e1310_d_n11, eq50_e1310_d_n12, eq50_e1310_d_n13, eq50_e1310_d_n14, eq50_e1310_d_n15, eq50_e1310_d_n16, eq50_e1310_d_n17, eq50_e1310_d_b0, eq50_e1310_d_b1, eq50_e1310_d_b2, eq50_e1310_d_b3, eq50_e1310_d_b4, eq50_e1310_d_b5, eq50_e1310_d_b6, eq50_e1310_d_b7, eq50_e1310_d_b8, eq50_e1310_d_b9, eq50_e1310_d_b10, eq50_e1310_d_b11,) = {
    if (p.p52 != 0.0) {
        let eq50_e1308: f64 = (s.v[657] * (nv9 - nv8));let eq50_e1308_d_n0: f64 = (s.dn[657][0] * (nv9 - nv8));let eq50_e1308_d_n1: f64 = (s.dn[657][1] * (nv9 - nv8));let eq50_e1308_d_n2: f64 = (s.dn[657][2] * (nv9 - nv8));let eq50_e1308_d_n3: f64 = (s.dn[657][3] * (nv9 - nv8));let eq50_e1308_d_n4: f64 = (s.dn[657][4] * (nv9 - nv8));let eq50_e1308_d_n5: f64 = (s.dn[657][5] * (nv9 - nv8));let eq50_e1308_d_n6: f64 = (s.dn[657][6] * (nv9 - nv8));let eq50_e1308_d_n7: f64 = (s.dn[657][7] * (nv9 - nv8));let eq50_e1308_d_n8: f64 = ((s.dn[657][8] * (nv9 - nv8)) + (-s.v[657]));let eq50_e1308_d_n9: f64 = ((s.dn[657][9] * (nv9 - nv8)) + s.v[657]);let eq50_e1308_d_n10: f64 = (s.dn[657][10] * (nv9 - nv8));let eq50_e1308_d_n11: f64 = (s.dn[657][11] * (nv9 - nv8));let eq50_e1308_d_n12: f64 = (s.dn[657][12] * (nv9 - nv8));let eq50_e1308_d_n13: f64 = (s.dn[657][13] * (nv9 - nv8));let eq50_e1308_d_n14: f64 = (s.dn[657][14] * (nv9 - nv8));let eq50_e1308_d_n15: f64 = (s.dn[657][15] * (nv9 - nv8));let eq50_e1308_d_n16: f64 = (s.dn[657][16] * (nv9 - nv8));let eq50_e1308_d_n17: f64 = (s.dn[657][17] * (nv9 - nv8));let eq50_e1308_d_b0: f64 = (s.db[657][0] * (nv9 - nv8));let eq50_e1308_d_b1: f64 = (s.db[657][1] * (nv9 - nv8));let eq50_e1308_d_b2: f64 = (s.db[657][2] * (nv9 - nv8));let eq50_e1308_d_b3: f64 = (s.db[657][3] * (nv9 - nv8));let eq50_e1308_d_b4: f64 = (s.db[657][4] * (nv9 - nv8));let eq50_e1308_d_b5: f64 = (s.db[657][5] * (nv9 - nv8));let eq50_e1308_d_b6: f64 = (s.db[657][6] * (nv9 - nv8));let eq50_e1308_d_b7: f64 = (s.db[657][7] * (nv9 - nv8));let eq50_e1308_d_b8: f64 = (s.db[657][8] * (nv9 - nv8));let eq50_e1308_d_b9: f64 = (s.db[657][9] * (nv9 - nv8));let eq50_e1308_d_b10: f64 = (s.db[657][10] * (nv9 - nv8));let eq50_e1308_d_b11: f64 = (s.db[657][11] * (nv9 - nv8));
        (eq50_e1308, eq50_e1308_d_n0, eq50_e1308_d_n1, eq50_e1308_d_n2, eq50_e1308_d_n3, eq50_e1308_d_n4, eq50_e1308_d_n5, eq50_e1308_d_n6, eq50_e1308_d_n7, eq50_e1308_d_n8, eq50_e1308_d_n9, eq50_e1308_d_n10, eq50_e1308_d_n11, eq50_e1308_d_n12, eq50_e1308_d_n13, eq50_e1308_d_n14, eq50_e1308_d_n15, eq50_e1308_d_n16, eq50_e1308_d_n17, eq50_e1308_d_b0, eq50_e1308_d_b1, eq50_e1308_d_b2, eq50_e1308_d_b3, eq50_e1308_d_b4, eq50_e1308_d_b5, eq50_e1308_d_b6, eq50_e1308_d_b7, eq50_e1308_d_b8, eq50_e1308_d_b9, eq50_e1308_d_b10, eq50_e1308_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e1310;let eq50_node_derivatives: [f64; 18] = [eq50_e1310_d_n0, eq50_e1310_d_n1, eq50_e1310_d_n2, eq50_e1310_d_n3, eq50_e1310_d_n4, eq50_e1310_d_n5, eq50_e1310_d_n6, eq50_e1310_d_n7, eq50_e1310_d_n8, eq50_e1310_d_n9, eq50_e1310_d_n10, eq50_e1310_d_n11, eq50_e1310_d_n12, eq50_e1310_d_n13, eq50_e1310_d_n14, eq50_e1310_d_n15, eq50_e1310_d_n16, eq50_e1310_d_n17];let eq50_branch_derivatives: [f64; 12] = [eq50_e1310_d_b0, eq50_e1310_d_b1, eq50_e1310_d_b2, eq50_e1310_d_b3, eq50_e1310_d_b4, eq50_e1310_d_b5, eq50_e1310_d_b6, eq50_e1310_d_b7, eq50_e1310_d_b8, eq50_e1310_d_b9, eq50_e1310_d_b10, eq50_e1310_d_b11];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq50_value),
            &eq50_node_derivatives,
            &eq50_branch_derivatives,
            multiplicity,
        );
        let (eq51_e1316, eq51_e1316_d_n0, eq51_e1316_d_n1, eq51_e1316_d_n2, eq51_e1316_d_n3, eq51_e1316_d_n4, eq51_e1316_d_n5, eq51_e1316_d_n6, eq51_e1316_d_n7, eq51_e1316_d_n8, eq51_e1316_d_n9, eq51_e1316_d_n10, eq51_e1316_d_n11, eq51_e1316_d_n12, eq51_e1316_d_n13, eq51_e1316_d_n14, eq51_e1316_d_n15, eq51_e1316_d_n16, eq51_e1316_d_n17, eq51_e1316_d_b0, eq51_e1316_d_b1, eq51_e1316_d_b2, eq51_e1316_d_b3, eq51_e1316_d_b4, eq51_e1316_d_b5, eq51_e1316_d_b6, eq51_e1316_d_b7, eq51_e1316_d_b8, eq51_e1316_d_b9, eq51_e1316_d_b10, eq51_e1316_d_b11,) = {
    if (p.p52 != 0.0) {
        let eq51_e1314: f64 = (s.v[655] * (nv3 - nv8));let eq51_e1314_d_n0: f64 = (s.dn[655][0] * (nv3 - nv8));let eq51_e1314_d_n1: f64 = (s.dn[655][1] * (nv3 - nv8));let eq51_e1314_d_n2: f64 = (s.dn[655][2] * (nv3 - nv8));let eq51_e1314_d_n3: f64 = ((s.dn[655][3] * (nv3 - nv8)) + s.v[655]);let eq51_e1314_d_n4: f64 = (s.dn[655][4] * (nv3 - nv8));let eq51_e1314_d_n5: f64 = (s.dn[655][5] * (nv3 - nv8));let eq51_e1314_d_n6: f64 = (s.dn[655][6] * (nv3 - nv8));let eq51_e1314_d_n7: f64 = (s.dn[655][7] * (nv3 - nv8));let eq51_e1314_d_n8: f64 = ((s.dn[655][8] * (nv3 - nv8)) + (-s.v[655]));let eq51_e1314_d_n9: f64 = (s.dn[655][9] * (nv3 - nv8));let eq51_e1314_d_n10: f64 = (s.dn[655][10] * (nv3 - nv8));let eq51_e1314_d_n11: f64 = (s.dn[655][11] * (nv3 - nv8));let eq51_e1314_d_n12: f64 = (s.dn[655][12] * (nv3 - nv8));let eq51_e1314_d_n13: f64 = (s.dn[655][13] * (nv3 - nv8));let eq51_e1314_d_n14: f64 = (s.dn[655][14] * (nv3 - nv8));let eq51_e1314_d_n15: f64 = (s.dn[655][15] * (nv3 - nv8));let eq51_e1314_d_n16: f64 = (s.dn[655][16] * (nv3 - nv8));let eq51_e1314_d_n17: f64 = (s.dn[655][17] * (nv3 - nv8));let eq51_e1314_d_b0: f64 = (s.db[655][0] * (nv3 - nv8));let eq51_e1314_d_b1: f64 = (s.db[655][1] * (nv3 - nv8));let eq51_e1314_d_b2: f64 = (s.db[655][2] * (nv3 - nv8));let eq51_e1314_d_b3: f64 = (s.db[655][3] * (nv3 - nv8));let eq51_e1314_d_b4: f64 = (s.db[655][4] * (nv3 - nv8));let eq51_e1314_d_b5: f64 = (s.db[655][5] * (nv3 - nv8));let eq51_e1314_d_b6: f64 = (s.db[655][6] * (nv3 - nv8));let eq51_e1314_d_b7: f64 = (s.db[655][7] * (nv3 - nv8));let eq51_e1314_d_b8: f64 = (s.db[655][8] * (nv3 - nv8));let eq51_e1314_d_b9: f64 = (s.db[655][9] * (nv3 - nv8));let eq51_e1314_d_b10: f64 = (s.db[655][10] * (nv3 - nv8));let eq51_e1314_d_b11: f64 = (s.db[655][11] * (nv3 - nv8));
        (eq51_e1314, eq51_e1314_d_n0, eq51_e1314_d_n1, eq51_e1314_d_n2, eq51_e1314_d_n3, eq51_e1314_d_n4, eq51_e1314_d_n5, eq51_e1314_d_n6, eq51_e1314_d_n7, eq51_e1314_d_n8, eq51_e1314_d_n9, eq51_e1314_d_n10, eq51_e1314_d_n11, eq51_e1314_d_n12, eq51_e1314_d_n13, eq51_e1314_d_n14, eq51_e1314_d_n15, eq51_e1314_d_n16, eq51_e1314_d_n17, eq51_e1314_d_b0, eq51_e1314_d_b1, eq51_e1314_d_b2, eq51_e1314_d_b3, eq51_e1314_d_b4, eq51_e1314_d_b5, eq51_e1314_d_b6, eq51_e1314_d_b7, eq51_e1314_d_b8, eq51_e1314_d_b9, eq51_e1314_d_b10, eq51_e1314_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e1316;let eq51_node_derivatives: [f64; 18] = [eq51_e1316_d_n0, eq51_e1316_d_n1, eq51_e1316_d_n2, eq51_e1316_d_n3, eq51_e1316_d_n4, eq51_e1316_d_n5, eq51_e1316_d_n6, eq51_e1316_d_n7, eq51_e1316_d_n8, eq51_e1316_d_n9, eq51_e1316_d_n10, eq51_e1316_d_n11, eq51_e1316_d_n12, eq51_e1316_d_n13, eq51_e1316_d_n14, eq51_e1316_d_n15, eq51_e1316_d_n16, eq51_e1316_d_n17];let eq51_branch_derivatives: [f64; 12] = [eq51_e1316_d_b0, eq51_e1316_d_b1, eq51_e1316_d_b2, eq51_e1316_d_b3, eq51_e1316_d_b4, eq51_e1316_d_b5, eq51_e1316_d_b6, eq51_e1316_d_b7, eq51_e1316_d_b8, eq51_e1316_d_b9, eq51_e1316_d_b10, eq51_e1316_d_b11];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(8),
            multiplicity * (eq51_value),
            &eq51_node_derivatives,
            &eq51_branch_derivatives,
            multiplicity,
        );
        let (eq52_e1321,) = {
    if (p.p52 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq52_value: f64 = eq52_e1321;
        stamper.stamp_potential_const_local(
            6,
            eq52_value,
        );
        let (eq53_e1326,) = {
    if (p.p52 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq53_value: f64 = eq53_e1326;
        stamper.stamp_potential_const_local(
            7,
            eq53_value,
        );
        let (eq54_e1331,) = {
    if (p.p52 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq54_value: f64 = eq54_e1331;
        stamper.stamp_potential_const_local(
            8,
            eq54_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_18(
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq55_e1337, eq55_e1337_d_n0, eq55_e1337_d_n1, eq55_e1337_d_n2, eq55_e1337_d_n3, eq55_e1337_d_n4, eq55_e1337_d_n5, eq55_e1337_d_n6, eq55_e1337_d_n7, eq55_e1337_d_n8, eq55_e1337_d_n9, eq55_e1337_d_n10, eq55_e1337_d_n11, eq55_e1337_d_n12, eq55_e1337_d_n13, eq55_e1337_d_n14, eq55_e1337_d_n15, eq55_e1337_d_n16, eq55_e1337_d_n17, eq55_e1337_d_b0, eq55_e1337_d_b1, eq55_e1337_d_b2, eq55_e1337_d_b3, eq55_e1337_d_b4, eq55_e1337_d_b5, eq55_e1337_d_b6, eq55_e1337_d_b7, eq55_e1337_d_b8, eq55_e1337_d_b9, eq55_e1337_d_b10, eq55_e1337_d_b11,) = {
    if s.b[3409] {
        let eq55_e1335: f64 = ((nv4 - 0.0) * s.v[740]);let eq55_e1335_d_n0: f64 = ((nv4 - 0.0) * s.dn[740][0]);let eq55_e1335_d_n1: f64 = ((nv4 - 0.0) * s.dn[740][1]);let eq55_e1335_d_n2: f64 = ((nv4 - 0.0) * s.dn[740][2]);let eq55_e1335_d_n3: f64 = ((nv4 - 0.0) * s.dn[740][3]);let eq55_e1335_d_n4: f64 = (s.v[740] + ((nv4 - 0.0) * s.dn[740][4]));let eq55_e1335_d_n5: f64 = ((nv4 - 0.0) * s.dn[740][5]);let eq55_e1335_d_n6: f64 = ((nv4 - 0.0) * s.dn[740][6]);let eq55_e1335_d_n7: f64 = ((nv4 - 0.0) * s.dn[740][7]);let eq55_e1335_d_n8: f64 = ((nv4 - 0.0) * s.dn[740][8]);let eq55_e1335_d_n9: f64 = ((nv4 - 0.0) * s.dn[740][9]);let eq55_e1335_d_n10: f64 = ((nv4 - 0.0) * s.dn[740][10]);let eq55_e1335_d_n11: f64 = ((nv4 - 0.0) * s.dn[740][11]);let eq55_e1335_d_n12: f64 = ((nv4 - 0.0) * s.dn[740][12]);let eq55_e1335_d_n13: f64 = ((nv4 - 0.0) * s.dn[740][13]);let eq55_e1335_d_n14: f64 = ((nv4 - 0.0) * s.dn[740][14]);let eq55_e1335_d_n15: f64 = ((nv4 - 0.0) * s.dn[740][15]);let eq55_e1335_d_n16: f64 = ((nv4 - 0.0) * s.dn[740][16]);let eq55_e1335_d_n17: f64 = ((nv4 - 0.0) * s.dn[740][17]);let eq55_e1335_d_b0: f64 = ((nv4 - 0.0) * s.db[740][0]);let eq55_e1335_d_b1: f64 = ((nv4 - 0.0) * s.db[740][1]);let eq55_e1335_d_b2: f64 = ((nv4 - 0.0) * s.db[740][2]);let eq55_e1335_d_b3: f64 = ((nv4 - 0.0) * s.db[740][3]);let eq55_e1335_d_b4: f64 = ((nv4 - 0.0) * s.db[740][4]);let eq55_e1335_d_b5: f64 = ((nv4 - 0.0) * s.db[740][5]);let eq55_e1335_d_b6: f64 = ((nv4 - 0.0) * s.db[740][6]);let eq55_e1335_d_b7: f64 = ((nv4 - 0.0) * s.db[740][7]);let eq55_e1335_d_b8: f64 = ((nv4 - 0.0) * s.db[740][8]);let eq55_e1335_d_b9: f64 = ((nv4 - 0.0) * s.db[740][9]);let eq55_e1335_d_b10: f64 = ((nv4 - 0.0) * s.db[740][10]);let eq55_e1335_d_b11: f64 = ((nv4 - 0.0) * s.db[740][11]);
        (eq55_e1335, eq55_e1335_d_n0, eq55_e1335_d_n1, eq55_e1335_d_n2, eq55_e1335_d_n3, eq55_e1335_d_n4, eq55_e1335_d_n5, eq55_e1335_d_n6, eq55_e1335_d_n7, eq55_e1335_d_n8, eq55_e1335_d_n9, eq55_e1335_d_n10, eq55_e1335_d_n11, eq55_e1335_d_n12, eq55_e1335_d_n13, eq55_e1335_d_n14, eq55_e1335_d_n15, eq55_e1335_d_n16, eq55_e1335_d_n17, eq55_e1335_d_b0, eq55_e1335_d_b1, eq55_e1335_d_b2, eq55_e1335_d_b3, eq55_e1335_d_b4, eq55_e1335_d_b5, eq55_e1335_d_b6, eq55_e1335_d_b7, eq55_e1335_d_b8, eq55_e1335_d_b9, eq55_e1335_d_b10, eq55_e1335_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e1337;let eq55_node_derivatives: [f64; 18] = [eq55_e1337_d_n0, eq55_e1337_d_n1, eq55_e1337_d_n2, eq55_e1337_d_n3, eq55_e1337_d_n4, eq55_e1337_d_n5, eq55_e1337_d_n6, eq55_e1337_d_n7, eq55_e1337_d_n8, eq55_e1337_d_n9, eq55_e1337_d_n10, eq55_e1337_d_n11, eq55_e1337_d_n12, eq55_e1337_d_n13, eq55_e1337_d_n14, eq55_e1337_d_n15, eq55_e1337_d_n16, eq55_e1337_d_n17];let eq55_branch_derivatives: [f64; 12] = [eq55_e1337_d_b0, eq55_e1337_d_b1, eq55_e1337_d_b2, eq55_e1337_d_b3, eq55_e1337_d_b4, eq55_e1337_d_b5, eq55_e1337_d_b6, eq55_e1337_d_b7, eq55_e1337_d_b8, eq55_e1337_d_b9, eq55_e1337_d_b10, eq55_e1337_d_b11];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq55_value),
            &eq55_node_derivatives,
            &eq55_branch_derivatives,
            multiplicity,
        );
        let (eq56_e1342, eq56_e1342_d_n0, eq56_e1342_d_n1, eq56_e1342_d_n2, eq56_e1342_d_n3, eq56_e1342_d_n4, eq56_e1342_d_n5, eq56_e1342_d_n6, eq56_e1342_d_n7, eq56_e1342_d_n8, eq56_e1342_d_n9, eq56_e1342_d_n10, eq56_e1342_d_n11, eq56_e1342_d_n12, eq56_e1342_d_n13, eq56_e1342_d_n14, eq56_e1342_d_n15, eq56_e1342_d_n16, eq56_e1342_d_n17, eq56_e1342_d_b0, eq56_e1342_d_b1, eq56_e1342_d_b2, eq56_e1342_d_b3, eq56_e1342_d_b4, eq56_e1342_d_b5, eq56_e1342_d_b6, eq56_e1342_d_b7, eq56_e1342_d_b8, eq56_e1342_d_b9, eq56_e1342_d_b10, eq56_e1342_d_b11,) = {
    if s.b[3409] {
        let eq56_e1340: f64 = (-s.v[802]);
        (eq56_e1340, (-s.dn[802][0]), (-s.dn[802][1]), (-s.dn[802][2]), (-s.dn[802][3]), (-s.dn[802][4]), (-s.dn[802][5]), (-s.dn[802][6]), (-s.dn[802][7]), (-s.dn[802][8]), (-s.dn[802][9]), (-s.dn[802][10]), (-s.dn[802][11]), (-s.dn[802][12]), (-s.dn[802][13]), (-s.dn[802][14]), (-s.dn[802][15]), (-s.dn[802][16]), (-s.dn[802][17]), (-s.db[802][0]), (-s.db[802][1]), (-s.db[802][2]), (-s.db[802][3]), (-s.db[802][4]), (-s.db[802][5]), (-s.db[802][6]), (-s.db[802][7]), (-s.db[802][8]), (-s.db[802][9]), (-s.db[802][10]), (-s.db[802][11]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_value: f64 = eq56_e1342;let eq56_node_derivatives: [f64; 18] = [eq56_e1342_d_n0, eq56_e1342_d_n1, eq56_e1342_d_n2, eq56_e1342_d_n3, eq56_e1342_d_n4, eq56_e1342_d_n5, eq56_e1342_d_n6, eq56_e1342_d_n7, eq56_e1342_d_n8, eq56_e1342_d_n9, eq56_e1342_d_n10, eq56_e1342_d_n11, eq56_e1342_d_n12, eq56_e1342_d_n13, eq56_e1342_d_n14, eq56_e1342_d_n15, eq56_e1342_d_n16, eq56_e1342_d_n17];let eq56_branch_derivatives: [f64; 12] = [eq56_e1342_d_b0, eq56_e1342_d_b1, eq56_e1342_d_b2, eq56_e1342_d_b3, eq56_e1342_d_b4, eq56_e1342_d_b5, eq56_e1342_d_b6, eq56_e1342_d_b7, eq56_e1342_d_b8, eq56_e1342_d_b9, eq56_e1342_d_b10, eq56_e1342_d_b11];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq56_value),
            &eq56_node_derivatives,
            &eq56_branch_derivatives,
            multiplicity,
        );
        let (eq57_e1349, eq57_e1349_d_n4,) = {
    if (!s.b[3409]) {
        let eq57_e1347: f64 = ((nv4 - 0.0) * 10000.0);
        (eq57_e1347, 10000.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e1349;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq57_value),
            4,
            multiplicity * (eq57_e1349_d_n4),
        );let eq58_e1352: f64 = (s.v[767] * (nv4 - 0.0));let eq58_e1352_d_n0: f64 = (s.dn[767][0] * (nv4 - 0.0));let eq58_e1352_d_n1: f64 = (s.dn[767][1] * (nv4 - 0.0));let eq58_e1352_d_n2: f64 = (s.dn[767][2] * (nv4 - 0.0));let eq58_e1352_d_n3: f64 = (s.dn[767][3] * (nv4 - 0.0));let eq58_e1352_d_n4: f64 = ((s.dn[767][4] * (nv4 - 0.0)) + s.v[767]);let eq58_e1352_d_n5: f64 = (s.dn[767][5] * (nv4 - 0.0));let eq58_e1352_d_n6: f64 = (s.dn[767][6] * (nv4 - 0.0));let eq58_e1352_d_n7: f64 = (s.dn[767][7] * (nv4 - 0.0));let eq58_e1352_d_n8: f64 = (s.dn[767][8] * (nv4 - 0.0));let eq58_e1352_d_n9: f64 = (s.dn[767][9] * (nv4 - 0.0));let eq58_e1352_d_n10: f64 = (s.dn[767][10] * (nv4 - 0.0));let eq58_e1352_d_n11: f64 = (s.dn[767][11] * (nv4 - 0.0));let eq58_e1352_d_n12: f64 = (s.dn[767][12] * (nv4 - 0.0));let eq58_e1352_d_n13: f64 = (s.dn[767][13] * (nv4 - 0.0));let eq58_e1352_d_n14: f64 = (s.dn[767][14] * (nv4 - 0.0));let eq58_e1352_d_n15: f64 = (s.dn[767][15] * (nv4 - 0.0));let eq58_e1352_d_n16: f64 = (s.dn[767][16] * (nv4 - 0.0));let eq58_e1352_d_n17: f64 = (s.dn[767][17] * (nv4 - 0.0));let eq58_e1352_d_b0: f64 = (s.db[767][0] * (nv4 - 0.0));let eq58_e1352_d_b1: f64 = (s.db[767][1] * (nv4 - 0.0));let eq58_e1352_d_b2: f64 = (s.db[767][2] * (nv4 - 0.0));let eq58_e1352_d_b3: f64 = (s.db[767][3] * (nv4 - 0.0));let eq58_e1352_d_b4: f64 = (s.db[767][4] * (nv4 - 0.0));let eq58_e1352_d_b5: f64 = (s.db[767][5] * (nv4 - 0.0));let eq58_e1352_d_b6: f64 = (s.db[767][6] * (nv4 - 0.0));let eq58_e1352_d_b7: f64 = (s.db[767][7] * (nv4 - 0.0));let eq58_e1352_d_b8: f64 = (s.db[767][8] * (nv4 - 0.0));let eq58_e1352_d_b9: f64 = (s.db[767][9] * (nv4 - 0.0));let eq58_e1352_d_b10: f64 = (s.db[767][10] * (nv4 - 0.0));let eq58_e1352_d_b11: f64 = (s.db[767][11] * (nv4 - 0.0));let eq58_e1353: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, eq58_e1352);let eq58_value: f64 = eq58_e1353;let eq58_node_derivatives: [f64; 18] = [(eq58_e1352_d_n0 * ddt_scale), (eq58_e1352_d_n1 * ddt_scale), (eq58_e1352_d_n2 * ddt_scale), (eq58_e1352_d_n3 * ddt_scale), (eq58_e1352_d_n4 * ddt_scale), (eq58_e1352_d_n5 * ddt_scale), (eq58_e1352_d_n6 * ddt_scale), (eq58_e1352_d_n7 * ddt_scale), (eq58_e1352_d_n8 * ddt_scale), (eq58_e1352_d_n9 * ddt_scale), (eq58_e1352_d_n10 * ddt_scale), (eq58_e1352_d_n11 * ddt_scale), (eq58_e1352_d_n12 * ddt_scale), (eq58_e1352_d_n13 * ddt_scale), (eq58_e1352_d_n14 * ddt_scale), (eq58_e1352_d_n15 * ddt_scale), (eq58_e1352_d_n16 * ddt_scale), (eq58_e1352_d_n17 * ddt_scale)];let eq58_branch_derivatives: [f64; 12] = [(eq58_e1352_d_b0 * ddt_scale), (eq58_e1352_d_b1 * ddt_scale), (eq58_e1352_d_b2 * ddt_scale), (eq58_e1352_d_b3 * ddt_scale), (eq58_e1352_d_b4 * ddt_scale), (eq58_e1352_d_b5 * ddt_scale), (eq58_e1352_d_b6 * ddt_scale), (eq58_e1352_d_b7 * ddt_scale), (eq58_e1352_d_b8 * ddt_scale), (eq58_e1352_d_b9 * ddt_scale), (eq58_e1352_d_b10 * ddt_scale), (eq58_e1352_d_b11 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq58_value),
            &eq58_node_derivatives,
            &eq58_branch_derivatives,
            multiplicity,
        );
        let (eq59_e1357, eq59_e1357_d_n0, eq59_e1357_d_n1, eq59_e1357_d_n2, eq59_e1357_d_n3, eq59_e1357_d_n4, eq59_e1357_d_n5, eq59_e1357_d_n6, eq59_e1357_d_n7, eq59_e1357_d_n8, eq59_e1357_d_n9, eq59_e1357_d_n10, eq59_e1357_d_n11, eq59_e1357_d_n12, eq59_e1357_d_n13, eq59_e1357_d_n14, eq59_e1357_d_n15, eq59_e1357_d_n16, eq59_e1357_d_n17, eq59_e1357_d_b0, eq59_e1357_d_b1, eq59_e1357_d_b2, eq59_e1357_d_b3, eq59_e1357_d_b4, eq59_e1357_d_b5, eq59_e1357_d_b6, eq59_e1357_d_b7, eq59_e1357_d_b8, eq59_e1357_d_b9, eq59_e1357_d_b10, eq59_e1357_d_b11,) = {
    if (p.p28 != 0.0) {
        (s.v[749], s.dn[749][0], s.dn[749][1], s.dn[749][2], s.dn[749][3], s.dn[749][4], s.dn[749][5], s.dn[749][6], s.dn[749][7], s.dn[749][8], s.dn[749][9], s.dn[749][10], s.dn[749][11], s.dn[749][12], s.dn[749][13], s.dn[749][14], s.dn[749][15], s.dn[749][16], s.dn[749][17], s.db[749][0], s.db[749][1], s.db[749][2], s.db[749][3], s.db[749][4], s.db[749][5], s.db[749][6], s.db[749][7], s.db[749][8], s.db[749][9], s.db[749][10], s.db[749][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e1357;let eq59_node_derivatives: [f64; 18] = [eq59_e1357_d_n0, eq59_e1357_d_n1, eq59_e1357_d_n2, eq59_e1357_d_n3, eq59_e1357_d_n4, eq59_e1357_d_n5, eq59_e1357_d_n6, eq59_e1357_d_n7, eq59_e1357_d_n8, eq59_e1357_d_n9, eq59_e1357_d_n10, eq59_e1357_d_n11, eq59_e1357_d_n12, eq59_e1357_d_n13, eq59_e1357_d_n14, eq59_e1357_d_n15, eq59_e1357_d_n16, eq59_e1357_d_n17];let eq59_branch_derivatives: [f64; 12] = [eq59_e1357_d_b0, eq59_e1357_d_b1, eq59_e1357_d_b2, eq59_e1357_d_b3, eq59_e1357_d_b4, eq59_e1357_d_b5, eq59_e1357_d_b6, eq59_e1357_d_b7, eq59_e1357_d_b8, eq59_e1357_d_b9, eq59_e1357_d_b10, eq59_e1357_d_b11];
        stamper.stamp_current_dense_local(
            Some(11),
            None,
            multiplicity * (eq59_value),
            &eq59_node_derivatives,
            &eq59_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_19(
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
        let nv11 = ctx.node_voltage(nodes[11]);let nv12 = ctx.node_voltage(nodes[12]);
        let (eq60_e1361, eq60_e1361_d_n0, eq60_e1361_d_n1, eq60_e1361_d_n2, eq60_e1361_d_n3, eq60_e1361_d_n4, eq60_e1361_d_n5, eq60_e1361_d_n6, eq60_e1361_d_n7, eq60_e1361_d_n8, eq60_e1361_d_n9, eq60_e1361_d_n10, eq60_e1361_d_n11, eq60_e1361_d_n12, eq60_e1361_d_n13, eq60_e1361_d_n14, eq60_e1361_d_n15, eq60_e1361_d_n16, eq60_e1361_d_n17, eq60_e1361_d_b0, eq60_e1361_d_b1, eq60_e1361_d_b2, eq60_e1361_d_b3, eq60_e1361_d_b4, eq60_e1361_d_b5, eq60_e1361_d_b6, eq60_e1361_d_b7, eq60_e1361_d_b8, eq60_e1361_d_b9, eq60_e1361_d_b10, eq60_e1361_d_b11,) = {
    if (p.p28 != 0.0) {
        (s.v[750], s.dn[750][0], s.dn[750][1], s.dn[750][2], s.dn[750][3], s.dn[750][4], s.dn[750][5], s.dn[750][6], s.dn[750][7], s.dn[750][8], s.dn[750][9], s.dn[750][10], s.dn[750][11], s.dn[750][12], s.dn[750][13], s.dn[750][14], s.dn[750][15], s.dn[750][16], s.dn[750][17], s.db[750][0], s.db[750][1], s.db[750][2], s.db[750][3], s.db[750][4], s.db[750][5], s.db[750][6], s.db[750][7], s.db[750][8], s.db[750][9], s.db[750][10], s.db[750][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e1361;let eq60_node_derivatives: [f64; 18] = [eq60_e1361_d_n0, eq60_e1361_d_n1, eq60_e1361_d_n2, eq60_e1361_d_n3, eq60_e1361_d_n4, eq60_e1361_d_n5, eq60_e1361_d_n6, eq60_e1361_d_n7, eq60_e1361_d_n8, eq60_e1361_d_n9, eq60_e1361_d_n10, eq60_e1361_d_n11, eq60_e1361_d_n12, eq60_e1361_d_n13, eq60_e1361_d_n14, eq60_e1361_d_n15, eq60_e1361_d_n16, eq60_e1361_d_n17];let eq60_branch_derivatives: [f64; 12] = [eq60_e1361_d_b0, eq60_e1361_d_b1, eq60_e1361_d_b2, eq60_e1361_d_b3, eq60_e1361_d_b4, eq60_e1361_d_b5, eq60_e1361_d_b6, eq60_e1361_d_b7, eq60_e1361_d_b8, eq60_e1361_d_b9, eq60_e1361_d_b10, eq60_e1361_d_b11];
        stamper.stamp_current_dense_local(
            Some(12),
            None,
            multiplicity * (eq60_value),
            &eq60_node_derivatives,
            &eq60_branch_derivatives,
            multiplicity,
        );
        let (eq61_e1368, eq61_e1368_d_n0, eq61_e1368_d_n1, eq61_e1368_d_n2, eq61_e1368_d_n3, eq61_e1368_d_n4, eq61_e1368_d_n5, eq61_e1368_d_n6, eq61_e1368_d_n7, eq61_e1368_d_n8, eq61_e1368_d_n9, eq61_e1368_d_n10, eq61_e1368_d_n11, eq61_e1368_d_n12, eq61_e1368_d_n13, eq61_e1368_d_n14, eq61_e1368_d_n15, eq61_e1368_d_n16, eq61_e1368_d_n17, eq61_e1368_d_b0, eq61_e1368_d_b1, eq61_e1368_d_b2, eq61_e1368_d_b3, eq61_e1368_d_b4, eq61_e1368_d_b5, eq61_e1368_d_b6, eq61_e1368_d_b7, eq61_e1368_d_b8, eq61_e1368_d_b9, eq61_e1368_d_b10, eq61_e1368_d_b11,) = {
    if (p.p28 != 0.0) {
        let eq61_e1365: f64 = (s.v[800] * (nv11 - 0.0));let eq61_e1365_d_n0: f64 = (s.dn[800][0] * (nv11 - 0.0));let eq61_e1365_d_n1: f64 = (s.dn[800][1] * (nv11 - 0.0));let eq61_e1365_d_n2: f64 = (s.dn[800][2] * (nv11 - 0.0));let eq61_e1365_d_n3: f64 = (s.dn[800][3] * (nv11 - 0.0));let eq61_e1365_d_n4: f64 = (s.dn[800][4] * (nv11 - 0.0));let eq61_e1365_d_n5: f64 = (s.dn[800][5] * (nv11 - 0.0));let eq61_e1365_d_n6: f64 = (s.dn[800][6] * (nv11 - 0.0));let eq61_e1365_d_n7: f64 = (s.dn[800][7] * (nv11 - 0.0));let eq61_e1365_d_n8: f64 = (s.dn[800][8] * (nv11 - 0.0));let eq61_e1365_d_n9: f64 = (s.dn[800][9] * (nv11 - 0.0));let eq61_e1365_d_n10: f64 = (s.dn[800][10] * (nv11 - 0.0));let eq61_e1365_d_n11: f64 = ((s.dn[800][11] * (nv11 - 0.0)) + s.v[800]);let eq61_e1365_d_n12: f64 = (s.dn[800][12] * (nv11 - 0.0));let eq61_e1365_d_n13: f64 = (s.dn[800][13] * (nv11 - 0.0));let eq61_e1365_d_n14: f64 = (s.dn[800][14] * (nv11 - 0.0));let eq61_e1365_d_n15: f64 = (s.dn[800][15] * (nv11 - 0.0));let eq61_e1365_d_n16: f64 = (s.dn[800][16] * (nv11 - 0.0));let eq61_e1365_d_n17: f64 = (s.dn[800][17] * (nv11 - 0.0));let eq61_e1365_d_b0: f64 = (s.db[800][0] * (nv11 - 0.0));let eq61_e1365_d_b1: f64 = (s.db[800][1] * (nv11 - 0.0));let eq61_e1365_d_b2: f64 = (s.db[800][2] * (nv11 - 0.0));let eq61_e1365_d_b3: f64 = (s.db[800][3] * (nv11 - 0.0));let eq61_e1365_d_b4: f64 = (s.db[800][4] * (nv11 - 0.0));let eq61_e1365_d_b5: f64 = (s.db[800][5] * (nv11 - 0.0));let eq61_e1365_d_b6: f64 = (s.db[800][6] * (nv11 - 0.0));let eq61_e1365_d_b7: f64 = (s.db[800][7] * (nv11 - 0.0));let eq61_e1365_d_b8: f64 = (s.db[800][8] * (nv11 - 0.0));let eq61_e1365_d_b9: f64 = (s.db[800][9] * (nv11 - 0.0));let eq61_e1365_d_b10: f64 = (s.db[800][10] * (nv11 - 0.0));let eq61_e1365_d_b11: f64 = (s.db[800][11] * (nv11 - 0.0));let eq61_e1366: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, eq61_e1365);
        (eq61_e1366, (eq61_e1365_d_n0 * ddt_scale), (eq61_e1365_d_n1 * ddt_scale), (eq61_e1365_d_n2 * ddt_scale), (eq61_e1365_d_n3 * ddt_scale), (eq61_e1365_d_n4 * ddt_scale), (eq61_e1365_d_n5 * ddt_scale), (eq61_e1365_d_n6 * ddt_scale), (eq61_e1365_d_n7 * ddt_scale), (eq61_e1365_d_n8 * ddt_scale), (eq61_e1365_d_n9 * ddt_scale), (eq61_e1365_d_n10 * ddt_scale), (eq61_e1365_d_n11 * ddt_scale), (eq61_e1365_d_n12 * ddt_scale), (eq61_e1365_d_n13 * ddt_scale), (eq61_e1365_d_n14 * ddt_scale), (eq61_e1365_d_n15 * ddt_scale), (eq61_e1365_d_n16 * ddt_scale), (eq61_e1365_d_n17 * ddt_scale), (eq61_e1365_d_b0 * ddt_scale), (eq61_e1365_d_b1 * ddt_scale), (eq61_e1365_d_b2 * ddt_scale), (eq61_e1365_d_b3 * ddt_scale), (eq61_e1365_d_b4 * ddt_scale), (eq61_e1365_d_b5 * ddt_scale), (eq61_e1365_d_b6 * ddt_scale), (eq61_e1365_d_b7 * ddt_scale), (eq61_e1365_d_b8 * ddt_scale), (eq61_e1365_d_b9 * ddt_scale), (eq61_e1365_d_b10 * ddt_scale), (eq61_e1365_d_b11 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_value: f64 = eq61_e1368;let eq61_node_derivatives: [f64; 18] = [eq61_e1368_d_n0, eq61_e1368_d_n1, eq61_e1368_d_n2, eq61_e1368_d_n3, eq61_e1368_d_n4, eq61_e1368_d_n5, eq61_e1368_d_n6, eq61_e1368_d_n7, eq61_e1368_d_n8, eq61_e1368_d_n9, eq61_e1368_d_n10, eq61_e1368_d_n11, eq61_e1368_d_n12, eq61_e1368_d_n13, eq61_e1368_d_n14, eq61_e1368_d_n15, eq61_e1368_d_n16, eq61_e1368_d_n17];let eq61_branch_derivatives: [f64; 12] = [eq61_e1368_d_b0, eq61_e1368_d_b1, eq61_e1368_d_b2, eq61_e1368_d_b3, eq61_e1368_d_b4, eq61_e1368_d_b5, eq61_e1368_d_b6, eq61_e1368_d_b7, eq61_e1368_d_b8, eq61_e1368_d_b9, eq61_e1368_d_b10, eq61_e1368_d_b11];
        stamper.stamp_current_dense_local(
            Some(11),
            None,
            multiplicity * (eq61_value),
            &eq61_node_derivatives,
            &eq61_branch_derivatives,
            multiplicity,
        );
        let (eq62_e1375, eq62_e1375_d_n0, eq62_e1375_d_n1, eq62_e1375_d_n2, eq62_e1375_d_n3, eq62_e1375_d_n4, eq62_e1375_d_n5, eq62_e1375_d_n6, eq62_e1375_d_n7, eq62_e1375_d_n8, eq62_e1375_d_n9, eq62_e1375_d_n10, eq62_e1375_d_n11, eq62_e1375_d_n12, eq62_e1375_d_n13, eq62_e1375_d_n14, eq62_e1375_d_n15, eq62_e1375_d_n16, eq62_e1375_d_n17, eq62_e1375_d_b0, eq62_e1375_d_b1, eq62_e1375_d_b2, eq62_e1375_d_b3, eq62_e1375_d_b4, eq62_e1375_d_b5, eq62_e1375_d_b6, eq62_e1375_d_b7, eq62_e1375_d_b8, eq62_e1375_d_b9, eq62_e1375_d_b10, eq62_e1375_d_b11,) = {
    if (p.p28 != 0.0) {
        let eq62_e1372: f64 = (s.v[801] * (nv12 - 0.0));let eq62_e1372_d_n0: f64 = (s.dn[801][0] * (nv12 - 0.0));let eq62_e1372_d_n1: f64 = (s.dn[801][1] * (nv12 - 0.0));let eq62_e1372_d_n2: f64 = (s.dn[801][2] * (nv12 - 0.0));let eq62_e1372_d_n3: f64 = (s.dn[801][3] * (nv12 - 0.0));let eq62_e1372_d_n4: f64 = (s.dn[801][4] * (nv12 - 0.0));let eq62_e1372_d_n5: f64 = (s.dn[801][5] * (nv12 - 0.0));let eq62_e1372_d_n6: f64 = (s.dn[801][6] * (nv12 - 0.0));let eq62_e1372_d_n7: f64 = (s.dn[801][7] * (nv12 - 0.0));let eq62_e1372_d_n8: f64 = (s.dn[801][8] * (nv12 - 0.0));let eq62_e1372_d_n9: f64 = (s.dn[801][9] * (nv12 - 0.0));let eq62_e1372_d_n10: f64 = (s.dn[801][10] * (nv12 - 0.0));let eq62_e1372_d_n11: f64 = (s.dn[801][11] * (nv12 - 0.0));let eq62_e1372_d_n12: f64 = ((s.dn[801][12] * (nv12 - 0.0)) + s.v[801]);let eq62_e1372_d_n13: f64 = (s.dn[801][13] * (nv12 - 0.0));let eq62_e1372_d_n14: f64 = (s.dn[801][14] * (nv12 - 0.0));let eq62_e1372_d_n15: f64 = (s.dn[801][15] * (nv12 - 0.0));let eq62_e1372_d_n16: f64 = (s.dn[801][16] * (nv12 - 0.0));let eq62_e1372_d_n17: f64 = (s.dn[801][17] * (nv12 - 0.0));let eq62_e1372_d_b0: f64 = (s.db[801][0] * (nv12 - 0.0));let eq62_e1372_d_b1: f64 = (s.db[801][1] * (nv12 - 0.0));let eq62_e1372_d_b2: f64 = (s.db[801][2] * (nv12 - 0.0));let eq62_e1372_d_b3: f64 = (s.db[801][3] * (nv12 - 0.0));let eq62_e1372_d_b4: f64 = (s.db[801][4] * (nv12 - 0.0));let eq62_e1372_d_b5: f64 = (s.db[801][5] * (nv12 - 0.0));let eq62_e1372_d_b6: f64 = (s.db[801][6] * (nv12 - 0.0));let eq62_e1372_d_b7: f64 = (s.db[801][7] * (nv12 - 0.0));let eq62_e1372_d_b8: f64 = (s.db[801][8] * (nv12 - 0.0));let eq62_e1372_d_b9: f64 = (s.db[801][9] * (nv12 - 0.0));let eq62_e1372_d_b10: f64 = (s.db[801][10] * (nv12 - 0.0));let eq62_e1372_d_b11: f64 = (s.db[801][11] * (nv12 - 0.0));let eq62_e1373: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, eq62_e1372);
        (eq62_e1373, (eq62_e1372_d_n0 * ddt_scale), (eq62_e1372_d_n1 * ddt_scale), (eq62_e1372_d_n2 * ddt_scale), (eq62_e1372_d_n3 * ddt_scale), (eq62_e1372_d_n4 * ddt_scale), (eq62_e1372_d_n5 * ddt_scale), (eq62_e1372_d_n6 * ddt_scale), (eq62_e1372_d_n7 * ddt_scale), (eq62_e1372_d_n8 * ddt_scale), (eq62_e1372_d_n9 * ddt_scale), (eq62_e1372_d_n10 * ddt_scale), (eq62_e1372_d_n11 * ddt_scale), (eq62_e1372_d_n12 * ddt_scale), (eq62_e1372_d_n13 * ddt_scale), (eq62_e1372_d_n14 * ddt_scale), (eq62_e1372_d_n15 * ddt_scale), (eq62_e1372_d_n16 * ddt_scale), (eq62_e1372_d_n17 * ddt_scale), (eq62_e1372_d_b0 * ddt_scale), (eq62_e1372_d_b1 * ddt_scale), (eq62_e1372_d_b2 * ddt_scale), (eq62_e1372_d_b3 * ddt_scale), (eq62_e1372_d_b4 * ddt_scale), (eq62_e1372_d_b5 * ddt_scale), (eq62_e1372_d_b6 * ddt_scale), (eq62_e1372_d_b7 * ddt_scale), (eq62_e1372_d_b8 * ddt_scale), (eq62_e1372_d_b9 * ddt_scale), (eq62_e1372_d_b10 * ddt_scale), (eq62_e1372_d_b11 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e1375;let eq62_node_derivatives: [f64; 18] = [eq62_e1375_d_n0, eq62_e1375_d_n1, eq62_e1375_d_n2, eq62_e1375_d_n3, eq62_e1375_d_n4, eq62_e1375_d_n5, eq62_e1375_d_n6, eq62_e1375_d_n7, eq62_e1375_d_n8, eq62_e1375_d_n9, eq62_e1375_d_n10, eq62_e1375_d_n11, eq62_e1375_d_n12, eq62_e1375_d_n13, eq62_e1375_d_n14, eq62_e1375_d_n15, eq62_e1375_d_n16, eq62_e1375_d_n17];let eq62_branch_derivatives: [f64; 12] = [eq62_e1375_d_b0, eq62_e1375_d_b1, eq62_e1375_d_b2, eq62_e1375_d_b3, eq62_e1375_d_b4, eq62_e1375_d_b5, eq62_e1375_d_b6, eq62_e1375_d_b7, eq62_e1375_d_b8, eq62_e1375_d_b9, eq62_e1375_d_b10, eq62_e1375_d_b11];
        stamper.stamp_current_dense_local(
            Some(12),
            None,
            multiplicity * (eq62_value),
            &eq62_node_derivatives,
            &eq62_branch_derivatives,
            multiplicity,
        );
        let (eq63_e1380,) = {
    if (p.p28 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq63_value: f64 = eq63_e1380;
        stamper.stamp_potential_const_local(
            9,
            eq63_value,
        );
        let (eq64_e1385,) = {
    if (p.p28 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq64_value: f64 = eq64_e1385;
        stamper.stamp_potential_const_local(
            10,
            eq64_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_20(
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
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq65_e1389, eq65_e1389_d_n0, eq65_e1389_d_n1, eq65_e1389_d_n2, eq65_e1389_d_n3, eq65_e1389_d_n4, eq65_e1389_d_n5, eq65_e1389_d_n6, eq65_e1389_d_n7, eq65_e1389_d_n8, eq65_e1389_d_n9, eq65_e1389_d_n10, eq65_e1389_d_n11, eq65_e1389_d_n12, eq65_e1389_d_n13, eq65_e1389_d_n14, eq65_e1389_d_n15, eq65_e1389_d_n16, eq65_e1389_d_n17, eq65_e1389_d_b0, eq65_e1389_d_b1, eq65_e1389_d_b2, eq65_e1389_d_b3, eq65_e1389_d_b4, eq65_e1389_d_b5, eq65_e1389_d_b6, eq65_e1389_d_b7, eq65_e1389_d_b8, eq65_e1389_d_b9, eq65_e1389_d_b10, eq65_e1389_d_b11,) = {
    if (p.p29 != 0.0) {
        (s.v[815], s.dn[815][0], s.dn[815][1], s.dn[815][2], s.dn[815][3], s.dn[815][4], s.dn[815][5], s.dn[815][6], s.dn[815][7], s.dn[815][8], s.dn[815][9], s.dn[815][10], s.dn[815][11], s.dn[815][12], s.dn[815][13], s.dn[815][14], s.dn[815][15], s.dn[815][16], s.dn[815][17], s.db[815][0], s.db[815][1], s.db[815][2], s.db[815][3], s.db[815][4], s.db[815][5], s.db[815][6], s.db[815][7], s.db[815][8], s.db[815][9], s.db[815][10], s.db[815][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e1389;let eq65_node_derivatives: [f64; 18] = [eq65_e1389_d_n0, eq65_e1389_d_n1, eq65_e1389_d_n2, eq65_e1389_d_n3, eq65_e1389_d_n4, eq65_e1389_d_n5, eq65_e1389_d_n6, eq65_e1389_d_n7, eq65_e1389_d_n8, eq65_e1389_d_n9, eq65_e1389_d_n10, eq65_e1389_d_n11, eq65_e1389_d_n12, eq65_e1389_d_n13, eq65_e1389_d_n14, eq65_e1389_d_n15, eq65_e1389_d_n16, eq65_e1389_d_n17];let eq65_branch_derivatives: [f64; 12] = [eq65_e1389_d_b0, eq65_e1389_d_b1, eq65_e1389_d_b2, eq65_e1389_d_b3, eq65_e1389_d_b4, eq65_e1389_d_b5, eq65_e1389_d_b6, eq65_e1389_d_b7, eq65_e1389_d_b8, eq65_e1389_d_b9, eq65_e1389_d_b10, eq65_e1389_d_b11];
        stamper.stamp_current_dense_local(
            Some(13),
            None,
            multiplicity * (eq65_value),
            &eq65_node_derivatives,
            &eq65_branch_derivatives,
            multiplicity,
        );
        let (eq66_e1394, eq66_e1394_d_n13,) = {
    if (p.p29 != 0.0) {
        let eq66_e1392: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 20, (nv13 - 0.0));
        (eq66_e1392, ddt_scale,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq66_value: f64 = eq66_e1394;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq66_value),
            13,
            multiplicity * (eq66_e1394_d_n13),
        );
        let (eq67_e1399,) = {
    if (p.p29 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq67_value: f64 = eq67_e1399;
        stamper.stamp_potential_const_local(
            11,
            eq67_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        multiplicity: f64,
    ) {
        let (eq0_e1028, eq0_e1028_d_n0, eq0_e1028_d_n1, eq0_e1028_d_n2, eq0_e1028_d_n3, eq0_e1028_d_n4, eq0_e1028_d_n5, eq0_e1028_d_n6, eq0_e1028_d_n7, eq0_e1028_d_n8, eq0_e1028_d_n9, eq0_e1028_d_n10, eq0_e1028_d_n11, eq0_e1028_d_n12, eq0_e1028_d_n13, eq0_e1028_d_n14, eq0_e1028_d_n15, eq0_e1028_d_n16, eq0_e1028_d_n17, eq0_e1028_d_b0, eq0_e1028_d_b1, eq0_e1028_d_b2, eq0_e1028_d_b3, eq0_e1028_d_b4, eq0_e1028_d_b5, eq0_e1028_d_b6, eq0_e1028_d_b7, eq0_e1028_d_b8, eq0_e1028_d_b9, eq0_e1028_d_b10, eq0_e1028_d_b11, eq0_e1028_q, eq0_e1028_q_d_n0, eq0_e1028_q_d_n1, eq0_e1028_q_d_n2, eq0_e1028_q_d_n3, eq0_e1028_q_d_n4, eq0_e1028_q_d_n5, eq0_e1028_q_d_n6, eq0_e1028_q_d_n7, eq0_e1028_q_d_n8, eq0_e1028_q_d_n9, eq0_e1028_q_d_n10, eq0_e1028_q_d_n11, eq0_e1028_q_d_n12, eq0_e1028_q_d_n13, eq0_e1028_q_d_n14, eq0_e1028_q_d_n15, eq0_e1028_q_d_n16, eq0_e1028_q_d_n17, eq0_e1028_q_d_b0, eq0_e1028_q_d_b1, eq0_e1028_q_d_b2, eq0_e1028_q_d_b3, eq0_e1028_q_d_b4, eq0_e1028_q_d_b5, eq0_e1028_q_d_b6, eq0_e1028_q_d_b7, eq0_e1028_q_d_b8, eq0_e1028_q_d_b9, eq0_e1028_q_d_b10, eq0_e1028_q_d_b11,) = {
    if s.b[3305] {
        let eq0_e1025_q: f64 = s.v[924];let eq0_e1026: f64 = (s.v[926] + s.v[924]);let eq0_e1026_d_n0: f64 = (s.dn[926][0] + s.dn[924][0]);let eq0_e1026_d_n1: f64 = (s.dn[926][1] + s.dn[924][1]);let eq0_e1026_d_n2: f64 = (s.dn[926][2] + s.dn[924][2]);let eq0_e1026_d_n3: f64 = (s.dn[926][3] + s.dn[924][3]);let eq0_e1026_d_n4: f64 = (s.dn[926][4] + s.dn[924][4]);let eq0_e1026_d_n5: f64 = (s.dn[926][5] + s.dn[924][5]);let eq0_e1026_d_n6: f64 = (s.dn[926][6] + s.dn[924][6]);let eq0_e1026_d_n7: f64 = (s.dn[926][7] + s.dn[924][7]);let eq0_e1026_d_n8: f64 = (s.dn[926][8] + s.dn[924][8]);let eq0_e1026_d_n9: f64 = (s.dn[926][9] + s.dn[924][9]);let eq0_e1026_d_n10: f64 = (s.dn[926][10] + s.dn[924][10]);let eq0_e1026_d_n11: f64 = (s.dn[926][11] + s.dn[924][11]);let eq0_e1026_d_n12: f64 = (s.dn[926][12] + s.dn[924][12]);let eq0_e1026_d_n13: f64 = (s.dn[926][13] + s.dn[924][13]);let eq0_e1026_d_n14: f64 = (s.dn[926][14] + s.dn[924][14]);let eq0_e1026_d_n15: f64 = (s.dn[926][15] + s.dn[924][15]);let eq0_e1026_d_n16: f64 = (s.dn[926][16] + s.dn[924][16]);let eq0_e1026_d_n17: f64 = (s.dn[926][17] + s.dn[924][17]);let eq0_e1026_d_b0: f64 = (s.db[926][0] + s.db[924][0]);let eq0_e1026_d_b1: f64 = (s.db[926][1] + s.db[924][1]);let eq0_e1026_d_b2: f64 = (s.db[926][2] + s.db[924][2]);let eq0_e1026_d_b3: f64 = (s.db[926][3] + s.db[924][3]);let eq0_e1026_d_b4: f64 = (s.db[926][4] + s.db[924][4]);let eq0_e1026_d_b5: f64 = (s.db[926][5] + s.db[924][5]);let eq0_e1026_d_b6: f64 = (s.db[926][6] + s.db[924][6]);let eq0_e1026_d_b7: f64 = (s.db[926][7] + s.db[924][7]);let eq0_e1026_d_b8: f64 = (s.db[926][8] + s.db[924][8]);let eq0_e1026_d_b9: f64 = (s.db[926][9] + s.db[924][9]);let eq0_e1026_d_b10: f64 = (s.db[926][10] + s.db[924][10]);let eq0_e1026_d_b11: f64 = (s.db[926][11] + s.db[924][11]);let eq0_e1026_q: f64 = eq0_e1025_q;
        (eq0_e1026, eq0_e1026_d_n0, eq0_e1026_d_n1, eq0_e1026_d_n2, eq0_e1026_d_n3, eq0_e1026_d_n4, eq0_e1026_d_n5, eq0_e1026_d_n6, eq0_e1026_d_n7, eq0_e1026_d_n8, eq0_e1026_d_n9, eq0_e1026_d_n10, eq0_e1026_d_n11, eq0_e1026_d_n12, eq0_e1026_d_n13, eq0_e1026_d_n14, eq0_e1026_d_n15, eq0_e1026_d_n16, eq0_e1026_d_n17, eq0_e1026_d_b0, eq0_e1026_d_b1, eq0_e1026_d_b2, eq0_e1026_d_b3, eq0_e1026_d_b4, eq0_e1026_d_b5, eq0_e1026_d_b6, eq0_e1026_d_b7, eq0_e1026_d_b8, eq0_e1026_d_b9, eq0_e1026_d_b10, eq0_e1026_d_b11, eq0_e1026_q, s.dn[924][0], s.dn[924][1], s.dn[924][2], s.dn[924][3], s.dn[924][4], s.dn[924][5], s.dn[924][6], s.dn[924][7], s.dn[924][8], s.dn[924][9], s.dn[924][10], s.dn[924][11], s.dn[924][12], s.dn[924][13], s.dn[924][14], s.dn[924][15], s.dn[924][16], s.dn[924][17], s.db[924][0], s.db[924][1], s.db[924][2], s.db[924][3], s.db[924][4], s.db[924][5], s.db[924][6], s.db[924][7], s.db[924][8], s.db[924][9], s.db[924][10], s.db[924][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_reactive_node_derivatives: [f64; 18] = [eq0_e1028_q_d_n0, eq0_e1028_q_d_n1, eq0_e1028_q_d_n2, eq0_e1028_q_d_n3, eq0_e1028_q_d_n4, eq0_e1028_q_d_n5, eq0_e1028_q_d_n6, eq0_e1028_q_d_n7, eq0_e1028_q_d_n8, eq0_e1028_q_d_n9, eq0_e1028_q_d_n10, eq0_e1028_q_d_n11, eq0_e1028_q_d_n12, eq0_e1028_q_d_n13, eq0_e1028_q_d_n14, eq0_e1028_q_d_n15, eq0_e1028_q_d_n16, eq0_e1028_q_d_n17];let eq0_reactive_branch_derivatives: [f64; 12] = [eq0_e1028_q_d_b0, eq0_e1028_q_d_b1, eq0_e1028_q_d_b2, eq0_e1028_q_d_b3, eq0_e1028_q_d_b4, eq0_e1028_q_d_b5, eq0_e1028_q_d_b6, eq0_e1028_q_d_b7, eq0_e1028_q_d_b8, eq0_e1028_q_d_b9, eq0_e1028_q_d_b10, eq0_e1028_q_d_b11];
        stamper.stamp_current_reactive_dense_local(
            Some(15),
            None,
            &eq0_reactive_node_derivatives,
            &eq0_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq1_e1035, eq1_e1035_d_n0, eq1_e1035_d_n1, eq1_e1035_d_n2, eq1_e1035_d_n3, eq1_e1035_d_n4, eq1_e1035_d_n5, eq1_e1035_d_n6, eq1_e1035_d_n7, eq1_e1035_d_n8, eq1_e1035_d_n9, eq1_e1035_d_n10, eq1_e1035_d_n11, eq1_e1035_d_n12, eq1_e1035_d_n13, eq1_e1035_d_n14, eq1_e1035_d_n15, eq1_e1035_d_n16, eq1_e1035_d_n17, eq1_e1035_d_b0, eq1_e1035_d_b1, eq1_e1035_d_b2, eq1_e1035_d_b3, eq1_e1035_d_b4, eq1_e1035_d_b5, eq1_e1035_d_b6, eq1_e1035_d_b7, eq1_e1035_d_b8, eq1_e1035_d_b9, eq1_e1035_d_b10, eq1_e1035_d_b11, eq1_e1035_q, eq1_e1035_q_d_n0, eq1_e1035_q_d_n1, eq1_e1035_q_d_n2, eq1_e1035_q_d_n3, eq1_e1035_q_d_n4, eq1_e1035_q_d_n5, eq1_e1035_q_d_n6, eq1_e1035_q_d_n7, eq1_e1035_q_d_n8, eq1_e1035_q_d_n9, eq1_e1035_q_d_n10, eq1_e1035_q_d_n11, eq1_e1035_q_d_n12, eq1_e1035_q_d_n13, eq1_e1035_q_d_n14, eq1_e1035_q_d_n15, eq1_e1035_q_d_n16, eq1_e1035_q_d_n17, eq1_e1035_q_d_b0, eq1_e1035_q_d_b1, eq1_e1035_q_d_b2, eq1_e1035_q_d_b3, eq1_e1035_q_d_b4, eq1_e1035_q_d_b5, eq1_e1035_q_d_b6, eq1_e1035_q_d_b7, eq1_e1035_q_d_b8, eq1_e1035_q_d_b9, eq1_e1035_q_d_b10, eq1_e1035_q_d_b11,) = {
    if s.b[3305] {
        let eq1_e1032_q: f64 = s.v[925];let eq1_e1033: f64 = (s.v[927] + s.v[925]);let eq1_e1033_d_n0: f64 = (s.dn[927][0] + s.dn[925][0]);let eq1_e1033_d_n1: f64 = (s.dn[927][1] + s.dn[925][1]);let eq1_e1033_d_n2: f64 = (s.dn[927][2] + s.dn[925][2]);let eq1_e1033_d_n3: f64 = (s.dn[927][3] + s.dn[925][3]);let eq1_e1033_d_n4: f64 = (s.dn[927][4] + s.dn[925][4]);let eq1_e1033_d_n5: f64 = (s.dn[927][5] + s.dn[925][5]);let eq1_e1033_d_n6: f64 = (s.dn[927][6] + s.dn[925][6]);let eq1_e1033_d_n7: f64 = (s.dn[927][7] + s.dn[925][7]);let eq1_e1033_d_n8: f64 = (s.dn[927][8] + s.dn[925][8]);let eq1_e1033_d_n9: f64 = (s.dn[927][9] + s.dn[925][9]);let eq1_e1033_d_n10: f64 = (s.dn[927][10] + s.dn[925][10]);let eq1_e1033_d_n11: f64 = (s.dn[927][11] + s.dn[925][11]);let eq1_e1033_d_n12: f64 = (s.dn[927][12] + s.dn[925][12]);let eq1_e1033_d_n13: f64 = (s.dn[927][13] + s.dn[925][13]);let eq1_e1033_d_n14: f64 = (s.dn[927][14] + s.dn[925][14]);let eq1_e1033_d_n15: f64 = (s.dn[927][15] + s.dn[925][15]);let eq1_e1033_d_n16: f64 = (s.dn[927][16] + s.dn[925][16]);let eq1_e1033_d_n17: f64 = (s.dn[927][17] + s.dn[925][17]);let eq1_e1033_d_b0: f64 = (s.db[927][0] + s.db[925][0]);let eq1_e1033_d_b1: f64 = (s.db[927][1] + s.db[925][1]);let eq1_e1033_d_b2: f64 = (s.db[927][2] + s.db[925][2]);let eq1_e1033_d_b3: f64 = (s.db[927][3] + s.db[925][3]);let eq1_e1033_d_b4: f64 = (s.db[927][4] + s.db[925][4]);let eq1_e1033_d_b5: f64 = (s.db[927][5] + s.db[925][5]);let eq1_e1033_d_b6: f64 = (s.db[927][6] + s.db[925][6]);let eq1_e1033_d_b7: f64 = (s.db[927][7] + s.db[925][7]);let eq1_e1033_d_b8: f64 = (s.db[927][8] + s.db[925][8]);let eq1_e1033_d_b9: f64 = (s.db[927][9] + s.db[925][9]);let eq1_e1033_d_b10: f64 = (s.db[927][10] + s.db[925][10]);let eq1_e1033_d_b11: f64 = (s.db[927][11] + s.db[925][11]);let eq1_e1033_q: f64 = eq1_e1032_q;
        (eq1_e1033, eq1_e1033_d_n0, eq1_e1033_d_n1, eq1_e1033_d_n2, eq1_e1033_d_n3, eq1_e1033_d_n4, eq1_e1033_d_n5, eq1_e1033_d_n6, eq1_e1033_d_n7, eq1_e1033_d_n8, eq1_e1033_d_n9, eq1_e1033_d_n10, eq1_e1033_d_n11, eq1_e1033_d_n12, eq1_e1033_d_n13, eq1_e1033_d_n14, eq1_e1033_d_n15, eq1_e1033_d_n16, eq1_e1033_d_n17, eq1_e1033_d_b0, eq1_e1033_d_b1, eq1_e1033_d_b2, eq1_e1033_d_b3, eq1_e1033_d_b4, eq1_e1033_d_b5, eq1_e1033_d_b6, eq1_e1033_d_b7, eq1_e1033_d_b8, eq1_e1033_d_b9, eq1_e1033_d_b10, eq1_e1033_d_b11, eq1_e1033_q, s.dn[925][0], s.dn[925][1], s.dn[925][2], s.dn[925][3], s.dn[925][4], s.dn[925][5], s.dn[925][6], s.dn[925][7], s.dn[925][8], s.dn[925][9], s.dn[925][10], s.dn[925][11], s.dn[925][12], s.dn[925][13], s.dn[925][14], s.dn[925][15], s.dn[925][16], s.dn[925][17], s.db[925][0], s.db[925][1], s.db[925][2], s.db[925][3], s.db[925][4], s.db[925][5], s.db[925][6], s.db[925][7], s.db[925][8], s.db[925][9], s.db[925][10], s.db[925][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_reactive_node_derivatives: [f64; 18] = [eq1_e1035_q_d_n0, eq1_e1035_q_d_n1, eq1_e1035_q_d_n2, eq1_e1035_q_d_n3, eq1_e1035_q_d_n4, eq1_e1035_q_d_n5, eq1_e1035_q_d_n6, eq1_e1035_q_d_n7, eq1_e1035_q_d_n8, eq1_e1035_q_d_n9, eq1_e1035_q_d_n10, eq1_e1035_q_d_n11, eq1_e1035_q_d_n12, eq1_e1035_q_d_n13, eq1_e1035_q_d_n14, eq1_e1035_q_d_n15, eq1_e1035_q_d_n16, eq1_e1035_q_d_n17];let eq1_reactive_branch_derivatives: [f64; 12] = [eq1_e1035_q_d_b0, eq1_e1035_q_d_b1, eq1_e1035_q_d_b2, eq1_e1035_q_d_b3, eq1_e1035_q_d_b4, eq1_e1035_q_d_b5, eq1_e1035_q_d_b6, eq1_e1035_q_d_b7, eq1_e1035_q_d_b8, eq1_e1035_q_d_b9, eq1_e1035_q_d_b10, eq1_e1035_q_d_b11];
        stamper.stamp_current_reactive_dense_local(
            Some(16),
            None,
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
        let (eq4_e1052, eq4_e1052_d_n0, eq4_e1052_d_n1, eq4_e1052_d_n2, eq4_e1052_d_n3, eq4_e1052_d_n4, eq4_e1052_d_n5, eq4_e1052_d_n6, eq4_e1052_d_n7, eq4_e1052_d_n8, eq4_e1052_d_n9, eq4_e1052_d_n10, eq4_e1052_d_n11, eq4_e1052_d_n12, eq4_e1052_d_n13, eq4_e1052_d_n14, eq4_e1052_d_n15, eq4_e1052_d_n16, eq4_e1052_d_n17, eq4_e1052_d_b0, eq4_e1052_d_b1, eq4_e1052_d_b2, eq4_e1052_d_b3, eq4_e1052_d_b4, eq4_e1052_d_b5, eq4_e1052_d_b6, eq4_e1052_d_b7, eq4_e1052_d_b8, eq4_e1052_d_b9, eq4_e1052_d_b10, eq4_e1052_d_b11, eq4_e1052_q, eq4_e1052_q_d_n0, eq4_e1052_q_d_n1, eq4_e1052_q_d_n2, eq4_e1052_q_d_n3, eq4_e1052_q_d_n4, eq4_e1052_q_d_n5, eq4_e1052_q_d_n6, eq4_e1052_q_d_n7, eq4_e1052_q_d_n8, eq4_e1052_q_d_n9, eq4_e1052_q_d_n10, eq4_e1052_q_d_n11, eq4_e1052_q_d_n12, eq4_e1052_q_d_n13, eq4_e1052_q_d_n14, eq4_e1052_q_d_n15, eq4_e1052_q_d_n16, eq4_e1052_q_d_n17, eq4_e1052_q_d_b0, eq4_e1052_q_d_b1, eq4_e1052_q_d_b2, eq4_e1052_q_d_b3, eq4_e1052_q_d_b4, eq4_e1052_q_d_b5, eq4_e1052_q_d_b6, eq4_e1052_q_d_b7, eq4_e1052_q_d_b8, eq4_e1052_q_d_b9, eq4_e1052_q_d_b10, eq4_e1052_q_d_b11,) = {
    if s.b[3306] {
        let eq4_e1049_q: f64 = s.v[931];let eq4_e1050: f64 = (s.v[932] + s.v[931]);let eq4_e1050_d_n0: f64 = (s.dn[932][0] + s.dn[931][0]);let eq4_e1050_d_n1: f64 = (s.dn[932][1] + s.dn[931][1]);let eq4_e1050_d_n2: f64 = (s.dn[932][2] + s.dn[931][2]);let eq4_e1050_d_n3: f64 = (s.dn[932][3] + s.dn[931][3]);let eq4_e1050_d_n4: f64 = (s.dn[932][4] + s.dn[931][4]);let eq4_e1050_d_n5: f64 = (s.dn[932][5] + s.dn[931][5]);let eq4_e1050_d_n6: f64 = (s.dn[932][6] + s.dn[931][6]);let eq4_e1050_d_n7: f64 = (s.dn[932][7] + s.dn[931][7]);let eq4_e1050_d_n8: f64 = (s.dn[932][8] + s.dn[931][8]);let eq4_e1050_d_n9: f64 = (s.dn[932][9] + s.dn[931][9]);let eq4_e1050_d_n10: f64 = (s.dn[932][10] + s.dn[931][10]);let eq4_e1050_d_n11: f64 = (s.dn[932][11] + s.dn[931][11]);let eq4_e1050_d_n12: f64 = (s.dn[932][12] + s.dn[931][12]);let eq4_e1050_d_n13: f64 = (s.dn[932][13] + s.dn[931][13]);let eq4_e1050_d_n14: f64 = (s.dn[932][14] + s.dn[931][14]);let eq4_e1050_d_n15: f64 = (s.dn[932][15] + s.dn[931][15]);let eq4_e1050_d_n16: f64 = (s.dn[932][16] + s.dn[931][16]);let eq4_e1050_d_n17: f64 = (s.dn[932][17] + s.dn[931][17]);let eq4_e1050_d_b0: f64 = (s.db[932][0] + s.db[931][0]);let eq4_e1050_d_b1: f64 = (s.db[932][1] + s.db[931][1]);let eq4_e1050_d_b2: f64 = (s.db[932][2] + s.db[931][2]);let eq4_e1050_d_b3: f64 = (s.db[932][3] + s.db[931][3]);let eq4_e1050_d_b4: f64 = (s.db[932][4] + s.db[931][4]);let eq4_e1050_d_b5: f64 = (s.db[932][5] + s.db[931][5]);let eq4_e1050_d_b6: f64 = (s.db[932][6] + s.db[931][6]);let eq4_e1050_d_b7: f64 = (s.db[932][7] + s.db[931][7]);let eq4_e1050_d_b8: f64 = (s.db[932][8] + s.db[931][8]);let eq4_e1050_d_b9: f64 = (s.db[932][9] + s.db[931][9]);let eq4_e1050_d_b10: f64 = (s.db[932][10] + s.db[931][10]);let eq4_e1050_d_b11: f64 = (s.db[932][11] + s.db[931][11]);let eq4_e1050_q: f64 = eq4_e1049_q;
        (eq4_e1050, eq4_e1050_d_n0, eq4_e1050_d_n1, eq4_e1050_d_n2, eq4_e1050_d_n3, eq4_e1050_d_n4, eq4_e1050_d_n5, eq4_e1050_d_n6, eq4_e1050_d_n7, eq4_e1050_d_n8, eq4_e1050_d_n9, eq4_e1050_d_n10, eq4_e1050_d_n11, eq4_e1050_d_n12, eq4_e1050_d_n13, eq4_e1050_d_n14, eq4_e1050_d_n15, eq4_e1050_d_n16, eq4_e1050_d_n17, eq4_e1050_d_b0, eq4_e1050_d_b1, eq4_e1050_d_b2, eq4_e1050_d_b3, eq4_e1050_d_b4, eq4_e1050_d_b5, eq4_e1050_d_b6, eq4_e1050_d_b7, eq4_e1050_d_b8, eq4_e1050_d_b9, eq4_e1050_d_b10, eq4_e1050_d_b11, eq4_e1050_q, s.dn[931][0], s.dn[931][1], s.dn[931][2], s.dn[931][3], s.dn[931][4], s.dn[931][5], s.dn[931][6], s.dn[931][7], s.dn[931][8], s.dn[931][9], s.dn[931][10], s.dn[931][11], s.dn[931][12], s.dn[931][13], s.dn[931][14], s.dn[931][15], s.dn[931][16], s.dn[931][17], s.db[931][0], s.db[931][1], s.db[931][2], s.db[931][3], s.db[931][4], s.db[931][5], s.db[931][6], s.db[931][7], s.db[931][8], s.db[931][9], s.db[931][10], s.db[931][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_reactive_node_derivatives: [f64; 18] = [eq4_e1052_q_d_n0, eq4_e1052_q_d_n1, eq4_e1052_q_d_n2, eq4_e1052_q_d_n3, eq4_e1052_q_d_n4, eq4_e1052_q_d_n5, eq4_e1052_q_d_n6, eq4_e1052_q_d_n7, eq4_e1052_q_d_n8, eq4_e1052_q_d_n9, eq4_e1052_q_d_n10, eq4_e1052_q_d_n11, eq4_e1052_q_d_n12, eq4_e1052_q_d_n13, eq4_e1052_q_d_n14, eq4_e1052_q_d_n15, eq4_e1052_q_d_n16, eq4_e1052_q_d_n17];let eq4_reactive_branch_derivatives: [f64; 12] = [eq4_e1052_q_d_b0, eq4_e1052_q_d_b1, eq4_e1052_q_d_b2, eq4_e1052_q_d_b3, eq4_e1052_q_d_b4, eq4_e1052_q_d_b5, eq4_e1052_q_d_b6, eq4_e1052_q_d_b7, eq4_e1052_q_d_b8, eq4_e1052_q_d_b9, eq4_e1052_q_d_b10, eq4_e1052_q_d_b11];
        stamper.stamp_current_reactive_dense_local(
            Some(17),
            None,
            &eq4_reactive_node_derivatives,
            &eq4_reactive_branch_derivatives,
            multiplicity,
        );let eq14_e1098_q: f64 = s.v[66];let eq14_e1099: f64 = (p.p87 * s.v[66]);let eq14_e1099_q: f64 = (p.p87 * eq14_e1098_q);
        stamper.stamp_current_reactive_dense_local(
            Some(10),
            Some(2),
            &s.dn[66],
            &s.db[66],
            (multiplicity) * (p.p87),
        );let eq15_e1102_q: f64 = s.v[65];let eq15_e1103: f64 = (p.p87 * s.v[65]);let eq15_e1103_q: f64 = (p.p87 * eq15_e1102_q);
        stamper.stamp_current_reactive_dense_local(
            Some(9),
            Some(0),
            &s.dn[65],
            &s.db[65],
            (multiplicity) * (p.p87),
        );
        let (eq18_e1122, eq18_e1122_d_n0, eq18_e1122_d_n1, eq18_e1122_d_n2, eq18_e1122_d_n3, eq18_e1122_d_n4, eq18_e1122_d_n5, eq18_e1122_d_n6, eq18_e1122_d_n7, eq18_e1122_d_n8, eq18_e1122_d_n9, eq18_e1122_d_n10, eq18_e1122_d_n11, eq18_e1122_d_n12, eq18_e1122_d_n13, eq18_e1122_d_n14, eq18_e1122_d_n15, eq18_e1122_d_n16, eq18_e1122_d_n17, eq18_e1122_d_b0, eq18_e1122_d_b1, eq18_e1122_d_b2, eq18_e1122_d_b3, eq18_e1122_d_b4, eq18_e1122_d_b5, eq18_e1122_d_b6, eq18_e1122_d_b7, eq18_e1122_d_b8, eq18_e1122_d_b9, eq18_e1122_d_b10, eq18_e1122_d_b11, eq18_e1122_q,) = {
    if s.b[3405] {
        let eq18_e1119_q: f64 = s.v[68];let eq18_e1120: f64 = (p.p87 * s.v[68]);let eq18_e1120_q: f64 = (p.p87 * eq18_e1119_q);
        (eq18_e1120, (p.p87 * s.dn[68][0]), (p.p87 * s.dn[68][1]), (p.p87 * s.dn[68][2]), (p.p87 * s.dn[68][3]), (p.p87 * s.dn[68][4]), (p.p87 * s.dn[68][5]), (p.p87 * s.dn[68][6]), (p.p87 * s.dn[68][7]), (p.p87 * s.dn[68][8]), (p.p87 * s.dn[68][9]), (p.p87 * s.dn[68][10]), (p.p87 * s.dn[68][11]), (p.p87 * s.dn[68][12]), (p.p87 * s.dn[68][13]), (p.p87 * s.dn[68][14]), (p.p87 * s.dn[68][15]), (p.p87 * s.dn[68][16]), (p.p87 * s.dn[68][17]), (p.p87 * s.db[68][0]), (p.p87 * s.db[68][1]), (p.p87 * s.db[68][2]), (p.p87 * s.db[68][3]), (p.p87 * s.db[68][4]), (p.p87 * s.db[68][5]), (p.p87 * s.db[68][6]), (p.p87 * s.db[68][7]), (p.p87 * s.db[68][8]), (p.p87 * s.db[68][9]), (p.p87 * s.db[68][10]), (p.p87 * s.db[68][11]), eq18_e1120_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_reactive_node_derivatives: [f64; 18] = [eq18_e1122_d_n0, eq18_e1122_d_n1, eq18_e1122_d_n2, eq18_e1122_d_n3, eq18_e1122_d_n4, eq18_e1122_d_n5, eq18_e1122_d_n6, eq18_e1122_d_n7, eq18_e1122_d_n8, eq18_e1122_d_n9, eq18_e1122_d_n10, eq18_e1122_d_n11, eq18_e1122_d_n12, eq18_e1122_d_n13, eq18_e1122_d_n14, eq18_e1122_d_n15, eq18_e1122_d_n16, eq18_e1122_d_n17];let eq18_reactive_branch_derivatives: [f64; 12] = [eq18_e1122_d_b0, eq18_e1122_d_b1, eq18_e1122_d_b2, eq18_e1122_d_b3, eq18_e1122_d_b4, eq18_e1122_d_b5, eq18_e1122_d_b6, eq18_e1122_d_b7, eq18_e1122_d_b8, eq18_e1122_d_b9, eq18_e1122_d_b10, eq18_e1122_d_b11];
        stamper.stamp_current_reactive_dense_local(
            Some(8),
            Some(7),
            &eq18_reactive_node_derivatives,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq19_e1129, eq19_e1129_d_n0, eq19_e1129_d_n1, eq19_e1129_d_n2, eq19_e1129_d_n3, eq19_e1129_d_n4, eq19_e1129_d_n5, eq19_e1129_d_n6, eq19_e1129_d_n7, eq19_e1129_d_n8, eq19_e1129_d_n9, eq19_e1129_d_n10, eq19_e1129_d_n11, eq19_e1129_d_n12, eq19_e1129_d_n13, eq19_e1129_d_n14, eq19_e1129_d_n15, eq19_e1129_d_n16, eq19_e1129_d_n17, eq19_e1129_d_b0, eq19_e1129_d_b1, eq19_e1129_d_b2, eq19_e1129_d_b3, eq19_e1129_d_b4, eq19_e1129_d_b5, eq19_e1129_d_b6, eq19_e1129_d_b7, eq19_e1129_d_b8, eq19_e1129_d_b9, eq19_e1129_d_b10, eq19_e1129_d_b11, eq19_e1129_q,) = {
    if s.b[3405] {
        let eq19_e1126_q: f64 = s.v[67];let eq19_e1127: f64 = (p.p87 * s.v[67]);let eq19_e1127_q: f64 = (p.p87 * eq19_e1126_q);
        (eq19_e1127, (p.p87 * s.dn[67][0]), (p.p87 * s.dn[67][1]), (p.p87 * s.dn[67][2]), (p.p87 * s.dn[67][3]), (p.p87 * s.dn[67][4]), (p.p87 * s.dn[67][5]), (p.p87 * s.dn[67][6]), (p.p87 * s.dn[67][7]), (p.p87 * s.dn[67][8]), (p.p87 * s.dn[67][9]), (p.p87 * s.dn[67][10]), (p.p87 * s.dn[67][11]), (p.p87 * s.dn[67][12]), (p.p87 * s.dn[67][13]), (p.p87 * s.dn[67][14]), (p.p87 * s.dn[67][15]), (p.p87 * s.dn[67][16]), (p.p87 * s.dn[67][17]), (p.p87 * s.db[67][0]), (p.p87 * s.db[67][1]), (p.p87 * s.db[67][2]), (p.p87 * s.db[67][3]), (p.p87 * s.db[67][4]), (p.p87 * s.db[67][5]), (p.p87 * s.db[67][6]), (p.p87 * s.db[67][7]), (p.p87 * s.db[67][8]), (p.p87 * s.db[67][9]), (p.p87 * s.db[67][10]), (p.p87 * s.db[67][11]), eq19_e1127_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_reactive_node_derivatives: [f64; 18] = [eq19_e1129_d_n0, eq19_e1129_d_n1, eq19_e1129_d_n2, eq19_e1129_d_n3, eq19_e1129_d_n4, eq19_e1129_d_n5, eq19_e1129_d_n6, eq19_e1129_d_n7, eq19_e1129_d_n8, eq19_e1129_d_n9, eq19_e1129_d_n10, eq19_e1129_d_n11, eq19_e1129_d_n12, eq19_e1129_d_n13, eq19_e1129_d_n14, eq19_e1129_d_n15, eq19_e1129_d_n16, eq19_e1129_d_n17];let eq19_reactive_branch_derivatives: [f64; 12] = [eq19_e1129_d_b0, eq19_e1129_d_b1, eq19_e1129_d_b2, eq19_e1129_d_b3, eq19_e1129_d_b4, eq19_e1129_d_b5, eq19_e1129_d_b6, eq19_e1129_d_b7, eq19_e1129_d_b8, eq19_e1129_d_b9, eq19_e1129_d_b10, eq19_e1129_d_b11];
        stamper.stamp_current_reactive_dense_local(
            Some(8),
            Some(5),
            &eq19_reactive_node_derivatives,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
