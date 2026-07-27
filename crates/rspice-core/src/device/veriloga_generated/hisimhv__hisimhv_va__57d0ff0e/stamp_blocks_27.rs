#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_1(
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
        let (eq5_e1056, eq5_e1056_d_n0, eq5_e1056_d_n1, eq5_e1056_d_n2, eq5_e1056_d_n3, eq5_e1056_d_n4, eq5_e1056_d_n5, eq5_e1056_d_n6, eq5_e1056_d_n7, eq5_e1056_d_n8, eq5_e1056_d_n9, eq5_e1056_d_n10, eq5_e1056_d_n11, eq5_e1056_d_n12, eq5_e1056_d_n13, eq5_e1056_d_n14, eq5_e1056_d_n15, eq5_e1056_d_n16, eq5_e1056_d_n17, eq5_e1056_d_n18, eq5_e1056_d_b0, eq5_e1056_d_b1, eq5_e1056_d_b2, eq5_e1056_d_b3, eq5_e1056_d_b4, eq5_e1056_d_b5, eq5_e1056_d_b6, eq5_e1056_d_b7, eq5_e1056_d_b8, eq5_e1056_d_b9, eq5_e1056_d_b10, eq5_e1056_d_b11, eq5_e1056_d_b12,) = {
    if s.b[3310] {
        let eq5_e1053: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, s.v[931]);let eq5_e1054: f64 = (s.v[932] + eq5_e1053);let eq5_e1054_d_n0: f64 = (s.dn[932][0] + (s.dn[931][0] * ddt_scale));let eq5_e1054_d_n1: f64 = (s.dn[932][1] + (s.dn[931][1] * ddt_scale));let eq5_e1054_d_n2: f64 = (s.dn[932][2] + (s.dn[931][2] * ddt_scale));let eq5_e1054_d_n3: f64 = (s.dn[932][3] + (s.dn[931][3] * ddt_scale));let eq5_e1054_d_n4: f64 = (s.dn[932][4] + (s.dn[931][4] * ddt_scale));let eq5_e1054_d_n5: f64 = (s.dn[932][5] + (s.dn[931][5] * ddt_scale));let eq5_e1054_d_n6: f64 = (s.dn[932][6] + (s.dn[931][6] * ddt_scale));let eq5_e1054_d_n7: f64 = (s.dn[932][7] + (s.dn[931][7] * ddt_scale));let eq5_e1054_d_n8: f64 = (s.dn[932][8] + (s.dn[931][8] * ddt_scale));let eq5_e1054_d_n9: f64 = (s.dn[932][9] + (s.dn[931][9] * ddt_scale));let eq5_e1054_d_n10: f64 = (s.dn[932][10] + (s.dn[931][10] * ddt_scale));let eq5_e1054_d_n11: f64 = (s.dn[932][11] + (s.dn[931][11] * ddt_scale));let eq5_e1054_d_n12: f64 = (s.dn[932][12] + (s.dn[931][12] * ddt_scale));let eq5_e1054_d_n13: f64 = (s.dn[932][13] + (s.dn[931][13] * ddt_scale));let eq5_e1054_d_n14: f64 = (s.dn[932][14] + (s.dn[931][14] * ddt_scale));let eq5_e1054_d_n15: f64 = (s.dn[932][15] + (s.dn[931][15] * ddt_scale));let eq5_e1054_d_n16: f64 = (s.dn[932][16] + (s.dn[931][16] * ddt_scale));let eq5_e1054_d_n17: f64 = (s.dn[932][17] + (s.dn[931][17] * ddt_scale));let eq5_e1054_d_n18: f64 = (s.dn[932][18] + (s.dn[931][18] * ddt_scale));let eq5_e1054_d_b0: f64 = (s.db[932][0] + (s.db[931][0] * ddt_scale));let eq5_e1054_d_b1: f64 = (s.db[932][1] + (s.db[931][1] * ddt_scale));let eq5_e1054_d_b2: f64 = (s.db[932][2] + (s.db[931][2] * ddt_scale));let eq5_e1054_d_b3: f64 = (s.db[932][3] + (s.db[931][3] * ddt_scale));let eq5_e1054_d_b4: f64 = (s.db[932][4] + (s.db[931][4] * ddt_scale));let eq5_e1054_d_b5: f64 = (s.db[932][5] + (s.db[931][5] * ddt_scale));let eq5_e1054_d_b6: f64 = (s.db[932][6] + (s.db[931][6] * ddt_scale));let eq5_e1054_d_b7: f64 = (s.db[932][7] + (s.db[931][7] * ddt_scale));let eq5_e1054_d_b8: f64 = (s.db[932][8] + (s.db[931][8] * ddt_scale));let eq5_e1054_d_b9: f64 = (s.db[932][9] + (s.db[931][9] * ddt_scale));let eq5_e1054_d_b10: f64 = (s.db[932][10] + (s.db[931][10] * ddt_scale));let eq5_e1054_d_b11: f64 = (s.db[932][11] + (s.db[931][11] * ddt_scale));let eq5_e1054_d_b12: f64 = (s.db[932][12] + (s.db[931][12] * ddt_scale));
        (eq5_e1054, eq5_e1054_d_n0, eq5_e1054_d_n1, eq5_e1054_d_n2, eq5_e1054_d_n3, eq5_e1054_d_n4, eq5_e1054_d_n5, eq5_e1054_d_n6, eq5_e1054_d_n7, eq5_e1054_d_n8, eq5_e1054_d_n9, eq5_e1054_d_n10, eq5_e1054_d_n11, eq5_e1054_d_n12, eq5_e1054_d_n13, eq5_e1054_d_n14, eq5_e1054_d_n15, eq5_e1054_d_n16, eq5_e1054_d_n17, eq5_e1054_d_n18, eq5_e1054_d_b0, eq5_e1054_d_b1, eq5_e1054_d_b2, eq5_e1054_d_b3, eq5_e1054_d_b4, eq5_e1054_d_b5, eq5_e1054_d_b6, eq5_e1054_d_b7, eq5_e1054_d_b8, eq5_e1054_d_b9, eq5_e1054_d_b10, eq5_e1054_d_b11, eq5_e1054_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1056;let eq5_node_derivatives: [f64; 19] = [eq5_e1056_d_n0, eq5_e1056_d_n1, eq5_e1056_d_n2, eq5_e1056_d_n3, eq5_e1056_d_n4, eq5_e1056_d_n5, eq5_e1056_d_n6, eq5_e1056_d_n7, eq5_e1056_d_n8, eq5_e1056_d_n9, eq5_e1056_d_n10, eq5_e1056_d_n11, eq5_e1056_d_n12, eq5_e1056_d_n13, eq5_e1056_d_n14, eq5_e1056_d_n15, eq5_e1056_d_n16, eq5_e1056_d_n17, eq5_e1056_d_n18];let eq5_branch_derivatives: [f64; 13] = [eq5_e1056_d_b0, eq5_e1056_d_b1, eq5_e1056_d_b2, eq5_e1056_d_b3, eq5_e1056_d_b4, eq5_e1056_d_b5, eq5_e1056_d_b6, eq5_e1056_d_b7, eq5_e1056_d_b8, eq5_e1056_d_b9, eq5_e1056_d_b10, eq5_e1056_d_b11, eq5_e1056_d_b12];
        stamper.stamp_current_dense_local(
            Some(18),
            None,
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let (eq6_e1061,) = {
    if (!s.b[3310]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq6_value: f64 = eq6_e1061;
        stamper.stamp_potential_const_local(
            3,
            eq6_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_2(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq7_e1065: f64 = (s.v[134] + s.v[400]);let eq7_e1065_d_n0: f64 = (s.dn[134][0] + s.dn[400][0]);let eq7_e1065_d_n1: f64 = (s.dn[134][1] + s.dn[400][1]);let eq7_e1065_d_n2: f64 = (s.dn[134][2] + s.dn[400][2]);let eq7_e1065_d_n3: f64 = (s.dn[134][3] + s.dn[400][3]);let eq7_e1065_d_n4: f64 = (s.dn[134][4] + s.dn[400][4]);let eq7_e1065_d_n5: f64 = (s.dn[134][5] + s.dn[400][5]);let eq7_e1065_d_n6: f64 = (s.dn[134][6] + s.dn[400][6]);let eq7_e1065_d_n7: f64 = (s.dn[134][7] + s.dn[400][7]);let eq7_e1065_d_n8: f64 = (s.dn[134][8] + s.dn[400][8]);let eq7_e1065_d_n9: f64 = (s.dn[134][9] + s.dn[400][9]);let eq7_e1065_d_n10: f64 = (s.dn[134][10] + s.dn[400][10]);let eq7_e1065_d_n11: f64 = (s.dn[134][11] + s.dn[400][11]);let eq7_e1065_d_n12: f64 = (s.dn[134][12] + s.dn[400][12]);let eq7_e1065_d_n13: f64 = (s.dn[134][13] + s.dn[400][13]);let eq7_e1065_d_n14: f64 = (s.dn[134][14] + s.dn[400][14]);let eq7_e1065_d_n15: f64 = (s.dn[134][15] + s.dn[400][15]);let eq7_e1065_d_n16: f64 = (s.dn[134][16] + s.dn[400][16]);let eq7_e1065_d_n17: f64 = (s.dn[134][17] + s.dn[400][17]);let eq7_e1065_d_n18: f64 = (s.dn[134][18] + s.dn[400][18]);let eq7_e1065_d_b0: f64 = (s.db[134][0] + s.db[400][0]);let eq7_e1065_d_b1: f64 = (s.db[134][1] + s.db[400][1]);let eq7_e1065_d_b2: f64 = (s.db[134][2] + s.db[400][2]);let eq7_e1065_d_b3: f64 = (s.db[134][3] + s.db[400][3]);let eq7_e1065_d_b4: f64 = (s.db[134][4] + s.db[400][4]);let eq7_e1065_d_b5: f64 = (s.db[134][5] + s.db[400][5]);let eq7_e1065_d_b6: f64 = (s.db[134][6] + s.db[400][6]);let eq7_e1065_d_b7: f64 = (s.db[134][7] + s.db[400][7]);let eq7_e1065_d_b8: f64 = (s.db[134][8] + s.db[400][8]);let eq7_e1065_d_b9: f64 = (s.db[134][9] + s.db[400][9]);let eq7_e1065_d_b10: f64 = (s.db[134][10] + s.db[400][10]);let eq7_e1065_d_b11: f64 = (s.db[134][11] + s.db[400][11]);let eq7_e1065_d_b12: f64 = (s.db[134][12] + s.db[400][12]);let eq7_e1067: f64 = (eq7_e1065 - s.v[738]);let eq7_e1067_d_n0: f64 = (eq7_e1065_d_n0 - s.dn[738][0]);let eq7_e1067_d_n1: f64 = (eq7_e1065_d_n1 - s.dn[738][1]);let eq7_e1067_d_n2: f64 = (eq7_e1065_d_n2 - s.dn[738][2]);let eq7_e1067_d_n3: f64 = (eq7_e1065_d_n3 - s.dn[738][3]);let eq7_e1067_d_n4: f64 = (eq7_e1065_d_n4 - s.dn[738][4]);let eq7_e1067_d_n5: f64 = (eq7_e1065_d_n5 - s.dn[738][5]);let eq7_e1067_d_n6: f64 = (eq7_e1065_d_n6 - s.dn[738][6]);let eq7_e1067_d_n7: f64 = (eq7_e1065_d_n7 - s.dn[738][7]);let eq7_e1067_d_n8: f64 = (eq7_e1065_d_n8 - s.dn[738][8]);let eq7_e1067_d_n9: f64 = (eq7_e1065_d_n9 - s.dn[738][9]);let eq7_e1067_d_n10: f64 = (eq7_e1065_d_n10 - s.dn[738][10]);let eq7_e1067_d_n11: f64 = (eq7_e1065_d_n11 - s.dn[738][11]);let eq7_e1067_d_n12: f64 = (eq7_e1065_d_n12 - s.dn[738][12]);let eq7_e1067_d_n13: f64 = (eq7_e1065_d_n13 - s.dn[738][13]);let eq7_e1067_d_n14: f64 = (eq7_e1065_d_n14 - s.dn[738][14]);let eq7_e1067_d_n15: f64 = (eq7_e1065_d_n15 - s.dn[738][15]);let eq7_e1067_d_n16: f64 = (eq7_e1065_d_n16 - s.dn[738][16]);let eq7_e1067_d_n17: f64 = (eq7_e1065_d_n17 - s.dn[738][17]);let eq7_e1067_d_n18: f64 = (eq7_e1065_d_n18 - s.dn[738][18]);let eq7_e1067_d_b0: f64 = (eq7_e1065_d_b0 - s.db[738][0]);let eq7_e1067_d_b1: f64 = (eq7_e1065_d_b1 - s.db[738][1]);let eq7_e1067_d_b2: f64 = (eq7_e1065_d_b2 - s.db[738][2]);let eq7_e1067_d_b3: f64 = (eq7_e1065_d_b3 - s.db[738][3]);let eq7_e1067_d_b4: f64 = (eq7_e1065_d_b4 - s.db[738][4]);let eq7_e1067_d_b5: f64 = (eq7_e1065_d_b5 - s.db[738][5]);let eq7_e1067_d_b6: f64 = (eq7_e1065_d_b6 - s.db[738][6]);let eq7_e1067_d_b7: f64 = (eq7_e1065_d_b7 - s.db[738][7]);let eq7_e1067_d_b8: f64 = (eq7_e1065_d_b8 - s.db[738][8]);let eq7_e1067_d_b9: f64 = (eq7_e1065_d_b9 - s.db[738][9]);let eq7_e1067_d_b10: f64 = (eq7_e1065_d_b10 - s.db[738][10]);let eq7_e1067_d_b11: f64 = (eq7_e1065_d_b11 - s.db[738][11]);let eq7_e1067_d_b12: f64 = (eq7_e1065_d_b12 - s.db[738][12]);let eq7_e1068: f64 = (p[87] * eq7_e1067);let eq7_e1068_d_n0: f64 = (p[87] * eq7_e1067_d_n0);let eq7_e1068_d_n1: f64 = (p[87] * eq7_e1067_d_n1);let eq7_e1068_d_n2: f64 = (p[87] * eq7_e1067_d_n2);let eq7_e1068_d_n3: f64 = (p[87] * eq7_e1067_d_n3);
        let eq7_e1068_d_n4: f64 = (p[87] * eq7_e1067_d_n4);let eq7_e1068_d_n5: f64 = (p[87] * eq7_e1067_d_n5);let eq7_e1068_d_n6: f64 = (p[87] * eq7_e1067_d_n6);let eq7_e1068_d_n7: f64 = (p[87] * eq7_e1067_d_n7);let eq7_e1068_d_n8: f64 = (p[87] * eq7_e1067_d_n8);let eq7_e1068_d_n9: f64 = (p[87] * eq7_e1067_d_n9);let eq7_e1068_d_n10: f64 = (p[87] * eq7_e1067_d_n10);let eq7_e1068_d_n11: f64 = (p[87] * eq7_e1067_d_n11);let eq7_e1068_d_n12: f64 = (p[87] * eq7_e1067_d_n12);let eq7_e1068_d_n13: f64 = (p[87] * eq7_e1067_d_n13);let eq7_e1068_d_n14: f64 = (p[87] * eq7_e1067_d_n14);let eq7_e1068_d_n15: f64 = (p[87] * eq7_e1067_d_n15);let eq7_e1068_d_n16: f64 = (p[87] * eq7_e1067_d_n16);let eq7_e1068_d_n17: f64 = (p[87] * eq7_e1067_d_n17);let eq7_e1068_d_n18: f64 = (p[87] * eq7_e1067_d_n18);let eq7_e1068_d_b0: f64 = (p[87] * eq7_e1067_d_b0);let eq7_e1068_d_b1: f64 = (p[87] * eq7_e1067_d_b1);let eq7_e1068_d_b2: f64 = (p[87] * eq7_e1067_d_b2);let eq7_e1068_d_b3: f64 = (p[87] * eq7_e1067_d_b3);let eq7_e1068_d_b4: f64 = (p[87] * eq7_e1067_d_b4);let eq7_e1068_d_b5: f64 = (p[87] * eq7_e1067_d_b5);let eq7_e1068_d_b6: f64 = (p[87] * eq7_e1067_d_b6);let eq7_e1068_d_b7: f64 = (p[87] * eq7_e1067_d_b7);let eq7_e1068_d_b8: f64 = (p[87] * eq7_e1067_d_b8);let eq7_e1068_d_b9: f64 = (p[87] * eq7_e1067_d_b9);let eq7_e1068_d_b10: f64 = (p[87] * eq7_e1067_d_b10);let eq7_e1068_d_b11: f64 = (p[87] * eq7_e1067_d_b11);let eq7_e1068_d_b12: f64 = (p[87] * eq7_e1067_d_b12);let eq7_value: f64 = eq7_e1068;let eq7_node_derivatives: [f64; 19] = [eq7_e1068_d_n0, eq7_e1068_d_n1, eq7_e1068_d_n2, eq7_e1068_d_n3, eq7_e1068_d_n4, eq7_e1068_d_n5, eq7_e1068_d_n6, eq7_e1068_d_n7, eq7_e1068_d_n8, eq7_e1068_d_n9, eq7_e1068_d_n10, eq7_e1068_d_n11, eq7_e1068_d_n12, eq7_e1068_d_n13, eq7_e1068_d_n14, eq7_e1068_d_n15, eq7_e1068_d_n16, eq7_e1068_d_n17, eq7_e1068_d_n18];let eq7_branch_derivatives: [f64; 13] = [eq7_e1068_d_b0, eq7_e1068_d_b1, eq7_e1068_d_b2, eq7_e1068_d_b3, eq7_e1068_d_b4, eq7_e1068_d_b5, eq7_e1068_d_b6, eq7_e1068_d_b7, eq7_e1068_d_b8, eq7_e1068_d_b9, eq7_e1068_d_b10, eq7_e1068_d_b11, eq7_e1068_d_b12];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_3(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq8_e1072: f64 = (s.v[424] - s.v[425]);let eq8_e1072_d_n0: f64 = (s.dn[424][0] - s.dn[425][0]);let eq8_e1072_d_n1: f64 = (s.dn[424][1] - s.dn[425][1]);let eq8_e1072_d_n2: f64 = (s.dn[424][2] - s.dn[425][2]);let eq8_e1072_d_n3: f64 = (s.dn[424][3] - s.dn[425][3]);let eq8_e1072_d_n4: f64 = (s.dn[424][4] - s.dn[425][4]);let eq8_e1072_d_n5: f64 = (s.dn[424][5] - s.dn[425][5]);let eq8_e1072_d_n6: f64 = (s.dn[424][6] - s.dn[425][6]);let eq8_e1072_d_n7: f64 = (s.dn[424][7] - s.dn[425][7]);let eq8_e1072_d_n8: f64 = (s.dn[424][8] - s.dn[425][8]);let eq8_e1072_d_n9: f64 = (s.dn[424][9] - s.dn[425][9]);let eq8_e1072_d_n10: f64 = (s.dn[424][10] - s.dn[425][10]);let eq8_e1072_d_n11: f64 = (s.dn[424][11] - s.dn[425][11]);let eq8_e1072_d_n12: f64 = (s.dn[424][12] - s.dn[425][12]);let eq8_e1072_d_n13: f64 = (s.dn[424][13] - s.dn[425][13]);let eq8_e1072_d_n14: f64 = (s.dn[424][14] - s.dn[425][14]);let eq8_e1072_d_n15: f64 = (s.dn[424][15] - s.dn[425][15]);let eq8_e1072_d_n16: f64 = (s.dn[424][16] - s.dn[425][16]);let eq8_e1072_d_n17: f64 = (s.dn[424][17] - s.dn[425][17]);let eq8_e1072_d_n18: f64 = (s.dn[424][18] - s.dn[425][18]);let eq8_e1072_d_b0: f64 = (s.db[424][0] - s.db[425][0]);let eq8_e1072_d_b1: f64 = (s.db[424][1] - s.db[425][1]);let eq8_e1072_d_b2: f64 = (s.db[424][2] - s.db[425][2]);let eq8_e1072_d_b3: f64 = (s.db[424][3] - s.db[425][3]);let eq8_e1072_d_b4: f64 = (s.db[424][4] - s.db[425][4]);let eq8_e1072_d_b5: f64 = (s.db[424][5] - s.db[425][5]);let eq8_e1072_d_b6: f64 = (s.db[424][6] - s.db[425][6]);let eq8_e1072_d_b7: f64 = (s.db[424][7] - s.db[425][7]);let eq8_e1072_d_b8: f64 = (s.db[424][8] - s.db[425][8]);let eq8_e1072_d_b9: f64 = (s.db[424][9] - s.db[425][9]);let eq8_e1072_d_b10: f64 = (s.db[424][10] - s.db[425][10]);let eq8_e1072_d_b11: f64 = (s.db[424][11] - s.db[425][11]);let eq8_e1072_d_b12: f64 = (s.db[424][12] - s.db[425][12]);let eq8_e1073: f64 = (p[87] * eq8_e1072);let eq8_e1073_d_n0: f64 = (p[87] * eq8_e1072_d_n0);let eq8_e1073_d_n1: f64 = (p[87] * eq8_e1072_d_n1);let eq8_e1073_d_n2: f64 = (p[87] * eq8_e1072_d_n2);let eq8_e1073_d_n3: f64 = (p[87] * eq8_e1072_d_n3);let eq8_e1073_d_n4: f64 = (p[87] * eq8_e1072_d_n4);let eq8_e1073_d_n5: f64 = (p[87] * eq8_e1072_d_n5);let eq8_e1073_d_n6: f64 = (p[87] * eq8_e1072_d_n6);let eq8_e1073_d_n7: f64 = (p[87] * eq8_e1072_d_n7);let eq8_e1073_d_n8: f64 = (p[87] * eq8_e1072_d_n8);let eq8_e1073_d_n9: f64 = (p[87] * eq8_e1072_d_n9);let eq8_e1073_d_n10: f64 = (p[87] * eq8_e1072_d_n10);let eq8_e1073_d_n11: f64 = (p[87] * eq8_e1072_d_n11);let eq8_e1073_d_n12: f64 = (p[87] * eq8_e1072_d_n12);let eq8_e1073_d_n13: f64 = (p[87] * eq8_e1072_d_n13);let eq8_e1073_d_n14: f64 = (p[87] * eq8_e1072_d_n14);let eq8_e1073_d_n15: f64 = (p[87] * eq8_e1072_d_n15);let eq8_e1073_d_n16: f64 = (p[87] * eq8_e1072_d_n16);let eq8_e1073_d_n17: f64 = (p[87] * eq8_e1072_d_n17);let eq8_e1073_d_n18: f64 = (p[87] * eq8_e1072_d_n18);let eq8_e1073_d_b0: f64 = (p[87] * eq8_e1072_d_b0);let eq8_e1073_d_b1: f64 = (p[87] * eq8_e1072_d_b1);let eq8_e1073_d_b2: f64 = (p[87] * eq8_e1072_d_b2);let eq8_e1073_d_b3: f64 = (p[87] * eq8_e1072_d_b3);let eq8_e1073_d_b4: f64 = (p[87] * eq8_e1072_d_b4);let eq8_e1073_d_b5: f64 = (p[87] * eq8_e1072_d_b5);let eq8_e1073_d_b6: f64 = (p[87] * eq8_e1072_d_b6);let eq8_e1073_d_b7: f64 = (p[87] * eq8_e1072_d_b7);let eq8_e1073_d_b8: f64 = (p[87] * eq8_e1072_d_b8);let eq8_e1073_d_b9: f64 = (p[87] * eq8_e1072_d_b9);let eq8_e1073_d_b10: f64 = (p[87] * eq8_e1072_d_b10);let eq8_e1073_d_b11: f64 = (p[87] * eq8_e1072_d_b11);let eq8_e1073_d_b12: f64 = (p[87] * eq8_e1072_d_b12);let eq8_value: f64 = eq8_e1073;let eq8_node_derivatives: [f64; 19] = [eq8_e1073_d_n0, eq8_e1073_d_n1, eq8_e1073_d_n2, eq8_e1073_d_n3, eq8_e1073_d_n4, eq8_e1073_d_n5, eq8_e1073_d_n6, eq8_e1073_d_n7, eq8_e1073_d_n8, eq8_e1073_d_n9, eq8_e1073_d_n10, eq8_e1073_d_n11, eq8_e1073_d_n12, eq8_e1073_d_n13, eq8_e1073_d_n14, eq8_e1073_d_n15, eq8_e1073_d_n16, eq8_e1073_d_n17, eq8_e1073_d_n18];
        let eq8_branch_derivatives: [f64; 13] = [eq8_e1073_d_b0, eq8_e1073_d_b1, eq8_e1073_d_b2, eq8_e1073_d_b3, eq8_e1073_d_b4, eq8_e1073_d_b5, eq8_e1073_d_b6, eq8_e1073_d_b7, eq8_e1073_d_b8, eq8_e1073_d_b9, eq8_e1073_d_b10, eq8_e1073_d_b11, eq8_e1073_d_b12];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_4(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq9_e1077: f64 = (s.v[203] + s.v[280]);let eq9_e1077_d_n0: f64 = (s.dn[203][0] + s.dn[280][0]);let eq9_e1077_d_n1: f64 = (s.dn[203][1] + s.dn[280][1]);let eq9_e1077_d_n2: f64 = (s.dn[203][2] + s.dn[280][2]);let eq9_e1077_d_n3: f64 = (s.dn[203][3] + s.dn[280][3]);let eq9_e1077_d_n4: f64 = (s.dn[203][4] + s.dn[280][4]);let eq9_e1077_d_n5: f64 = (s.dn[203][5] + s.dn[280][5]);let eq9_e1077_d_n6: f64 = (s.dn[203][6] + s.dn[280][6]);let eq9_e1077_d_n7: f64 = (s.dn[203][7] + s.dn[280][7]);let eq9_e1077_d_n8: f64 = (s.dn[203][8] + s.dn[280][8]);let eq9_e1077_d_n9: f64 = (s.dn[203][9] + s.dn[280][9]);let eq9_e1077_d_n10: f64 = (s.dn[203][10] + s.dn[280][10]);let eq9_e1077_d_n11: f64 = (s.dn[203][11] + s.dn[280][11]);let eq9_e1077_d_n12: f64 = (s.dn[203][12] + s.dn[280][12]);let eq9_e1077_d_n13: f64 = (s.dn[203][13] + s.dn[280][13]);let eq9_e1077_d_n14: f64 = (s.dn[203][14] + s.dn[280][14]);let eq9_e1077_d_n15: f64 = (s.dn[203][15] + s.dn[280][15]);let eq9_e1077_d_n16: f64 = (s.dn[203][16] + s.dn[280][16]);let eq9_e1077_d_n17: f64 = (s.dn[203][17] + s.dn[280][17]);let eq9_e1077_d_n18: f64 = (s.dn[203][18] + s.dn[280][18]);let eq9_e1077_d_b0: f64 = (s.db[203][0] + s.db[280][0]);let eq9_e1077_d_b1: f64 = (s.db[203][1] + s.db[280][1]);let eq9_e1077_d_b2: f64 = (s.db[203][2] + s.db[280][2]);let eq9_e1077_d_b3: f64 = (s.db[203][3] + s.db[280][3]);let eq9_e1077_d_b4: f64 = (s.db[203][4] + s.db[280][4]);let eq9_e1077_d_b5: f64 = (s.db[203][5] + s.db[280][5]);let eq9_e1077_d_b6: f64 = (s.db[203][6] + s.db[280][6]);let eq9_e1077_d_b7: f64 = (s.db[203][7] + s.db[280][7]);let eq9_e1077_d_b8: f64 = (s.db[203][8] + s.db[280][8]);let eq9_e1077_d_b9: f64 = (s.db[203][9] + s.db[280][9]);let eq9_e1077_d_b10: f64 = (s.db[203][10] + s.db[280][10]);let eq9_e1077_d_b11: f64 = (s.db[203][11] + s.db[280][11]);let eq9_e1077_d_b12: f64 = (s.db[203][12] + s.db[280][12]);let eq9_e1079: f64 = (eq9_e1077 + s.v[431]);let eq9_e1079_d_n0: f64 = (eq9_e1077_d_n0 + s.dn[431][0]);let eq9_e1079_d_n1: f64 = (eq9_e1077_d_n1 + s.dn[431][1]);let eq9_e1079_d_n2: f64 = (eq9_e1077_d_n2 + s.dn[431][2]);let eq9_e1079_d_n3: f64 = (eq9_e1077_d_n3 + s.dn[431][3]);let eq9_e1079_d_n4: f64 = (eq9_e1077_d_n4 + s.dn[431][4]);let eq9_e1079_d_n5: f64 = (eq9_e1077_d_n5 + s.dn[431][5]);let eq9_e1079_d_n6: f64 = (eq9_e1077_d_n6 + s.dn[431][6]);let eq9_e1079_d_n7: f64 = (eq9_e1077_d_n7 + s.dn[431][7]);let eq9_e1079_d_n8: f64 = (eq9_e1077_d_n8 + s.dn[431][8]);let eq9_e1079_d_n9: f64 = (eq9_e1077_d_n9 + s.dn[431][9]);let eq9_e1079_d_n10: f64 = (eq9_e1077_d_n10 + s.dn[431][10]);let eq9_e1079_d_n11: f64 = (eq9_e1077_d_n11 + s.dn[431][11]);let eq9_e1079_d_n12: f64 = (eq9_e1077_d_n12 + s.dn[431][12]);let eq9_e1079_d_n13: f64 = (eq9_e1077_d_n13 + s.dn[431][13]);let eq9_e1079_d_n14: f64 = (eq9_e1077_d_n14 + s.dn[431][14]);let eq9_e1079_d_n15: f64 = (eq9_e1077_d_n15 + s.dn[431][15]);let eq9_e1079_d_n16: f64 = (eq9_e1077_d_n16 + s.dn[431][16]);let eq9_e1079_d_n17: f64 = (eq9_e1077_d_n17 + s.dn[431][17]);let eq9_e1079_d_n18: f64 = (eq9_e1077_d_n18 + s.dn[431][18]);let eq9_e1079_d_b0: f64 = (eq9_e1077_d_b0 + s.db[431][0]);let eq9_e1079_d_b1: f64 = (eq9_e1077_d_b1 + s.db[431][1]);let eq9_e1079_d_b2: f64 = (eq9_e1077_d_b2 + s.db[431][2]);let eq9_e1079_d_b3: f64 = (eq9_e1077_d_b3 + s.db[431][3]);let eq9_e1079_d_b4: f64 = (eq9_e1077_d_b4 + s.db[431][4]);let eq9_e1079_d_b5: f64 = (eq9_e1077_d_b5 + s.db[431][5]);let eq9_e1079_d_b6: f64 = (eq9_e1077_d_b6 + s.db[431][6]);let eq9_e1079_d_b7: f64 = (eq9_e1077_d_b7 + s.db[431][7]);let eq9_e1079_d_b8: f64 = (eq9_e1077_d_b8 + s.db[431][8]);let eq9_e1079_d_b9: f64 = (eq9_e1077_d_b9 + s.db[431][9]);let eq9_e1079_d_b10: f64 = (eq9_e1077_d_b10 + s.db[431][10]);let eq9_e1079_d_b11: f64 = (eq9_e1077_d_b11 + s.db[431][11]);let eq9_e1079_d_b12: f64 = (eq9_e1077_d_b12 + s.db[431][12]);let eq9_e1080: f64 = (p[87] * eq9_e1079);let eq9_e1080_d_n0: f64 = (p[87] * eq9_e1079_d_n0);let eq9_e1080_d_n1: f64 = (p[87] * eq9_e1079_d_n1);let eq9_e1080_d_n2: f64 = (p[87] * eq9_e1079_d_n2);let eq9_e1080_d_n3: f64 = (p[87] * eq9_e1079_d_n3);
        let eq9_e1080_d_n4: f64 = (p[87] * eq9_e1079_d_n4);let eq9_e1080_d_n5: f64 = (p[87] * eq9_e1079_d_n5);let eq9_e1080_d_n6: f64 = (p[87] * eq9_e1079_d_n6);let eq9_e1080_d_n7: f64 = (p[87] * eq9_e1079_d_n7);let eq9_e1080_d_n8: f64 = (p[87] * eq9_e1079_d_n8);let eq9_e1080_d_n9: f64 = (p[87] * eq9_e1079_d_n9);let eq9_e1080_d_n10: f64 = (p[87] * eq9_e1079_d_n10);let eq9_e1080_d_n11: f64 = (p[87] * eq9_e1079_d_n11);let eq9_e1080_d_n12: f64 = (p[87] * eq9_e1079_d_n12);let eq9_e1080_d_n13: f64 = (p[87] * eq9_e1079_d_n13);let eq9_e1080_d_n14: f64 = (p[87] * eq9_e1079_d_n14);let eq9_e1080_d_n15: f64 = (p[87] * eq9_e1079_d_n15);let eq9_e1080_d_n16: f64 = (p[87] * eq9_e1079_d_n16);let eq9_e1080_d_n17: f64 = (p[87] * eq9_e1079_d_n17);let eq9_e1080_d_n18: f64 = (p[87] * eq9_e1079_d_n18);let eq9_e1080_d_b0: f64 = (p[87] * eq9_e1079_d_b0);let eq9_e1080_d_b1: f64 = (p[87] * eq9_e1079_d_b1);let eq9_e1080_d_b2: f64 = (p[87] * eq9_e1079_d_b2);let eq9_e1080_d_b3: f64 = (p[87] * eq9_e1079_d_b3);let eq9_e1080_d_b4: f64 = (p[87] * eq9_e1079_d_b4);let eq9_e1080_d_b5: f64 = (p[87] * eq9_e1079_d_b5);let eq9_e1080_d_b6: f64 = (p[87] * eq9_e1079_d_b6);let eq9_e1080_d_b7: f64 = (p[87] * eq9_e1079_d_b7);let eq9_e1080_d_b8: f64 = (p[87] * eq9_e1079_d_b8);let eq9_e1080_d_b9: f64 = (p[87] * eq9_e1079_d_b9);let eq9_e1080_d_b10: f64 = (p[87] * eq9_e1079_d_b10);let eq9_e1080_d_b11: f64 = (p[87] * eq9_e1079_d_b11);let eq9_e1080_d_b12: f64 = (p[87] * eq9_e1079_d_b12);let eq9_value: f64 = eq9_e1080;let eq9_node_derivatives: [f64; 19] = [eq9_e1080_d_n0, eq9_e1080_d_n1, eq9_e1080_d_n2, eq9_e1080_d_n3, eq9_e1080_d_n4, eq9_e1080_d_n5, eq9_e1080_d_n6, eq9_e1080_d_n7, eq9_e1080_d_n8, eq9_e1080_d_n9, eq9_e1080_d_n10, eq9_e1080_d_n11, eq9_e1080_d_n12, eq9_e1080_d_n13, eq9_e1080_d_n14, eq9_e1080_d_n15, eq9_e1080_d_n16, eq9_e1080_d_n17, eq9_e1080_d_n18];let eq9_branch_derivatives: [f64; 13] = [eq9_e1080_d_b0, eq9_e1080_d_b1, eq9_e1080_d_b2, eq9_e1080_d_b3, eq9_e1080_d_b4, eq9_e1080_d_b5, eq9_e1080_d_b6, eq9_e1080_d_b7, eq9_e1080_d_b8, eq9_e1080_d_b9, eq9_e1080_d_b10, eq9_e1080_d_b11, eq9_e1080_d_b12];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(9),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_5(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq10_e1084: f64 = (s.v[204] + s.v[736]);let eq10_e1084_d_n0: f64 = (s.dn[204][0] + s.dn[736][0]);let eq10_e1084_d_n1: f64 = (s.dn[204][1] + s.dn[736][1]);let eq10_e1084_d_n2: f64 = (s.dn[204][2] + s.dn[736][2]);let eq10_e1084_d_n3: f64 = (s.dn[204][3] + s.dn[736][3]);let eq10_e1084_d_n4: f64 = (s.dn[204][4] + s.dn[736][4]);let eq10_e1084_d_n5: f64 = (s.dn[204][5] + s.dn[736][5]);let eq10_e1084_d_n6: f64 = (s.dn[204][6] + s.dn[736][6]);let eq10_e1084_d_n7: f64 = (s.dn[204][7] + s.dn[736][7]);let eq10_e1084_d_n8: f64 = (s.dn[204][8] + s.dn[736][8]);let eq10_e1084_d_n9: f64 = (s.dn[204][9] + s.dn[736][9]);let eq10_e1084_d_n10: f64 = (s.dn[204][10] + s.dn[736][10]);let eq10_e1084_d_n11: f64 = (s.dn[204][11] + s.dn[736][11]);let eq10_e1084_d_n12: f64 = (s.dn[204][12] + s.dn[736][12]);let eq10_e1084_d_n13: f64 = (s.dn[204][13] + s.dn[736][13]);let eq10_e1084_d_n14: f64 = (s.dn[204][14] + s.dn[736][14]);let eq10_e1084_d_n15: f64 = (s.dn[204][15] + s.dn[736][15]);let eq10_e1084_d_n16: f64 = (s.dn[204][16] + s.dn[736][16]);let eq10_e1084_d_n17: f64 = (s.dn[204][17] + s.dn[736][17]);let eq10_e1084_d_n18: f64 = (s.dn[204][18] + s.dn[736][18]);let eq10_e1084_d_b0: f64 = (s.db[204][0] + s.db[736][0]);let eq10_e1084_d_b1: f64 = (s.db[204][1] + s.db[736][1]);let eq10_e1084_d_b2: f64 = (s.db[204][2] + s.db[736][2]);let eq10_e1084_d_b3: f64 = (s.db[204][3] + s.db[736][3]);let eq10_e1084_d_b4: f64 = (s.db[204][4] + s.db[736][4]);let eq10_e1084_d_b5: f64 = (s.db[204][5] + s.db[736][5]);let eq10_e1084_d_b6: f64 = (s.db[204][6] + s.db[736][6]);let eq10_e1084_d_b7: f64 = (s.db[204][7] + s.db[736][7]);let eq10_e1084_d_b8: f64 = (s.db[204][8] + s.db[736][8]);let eq10_e1084_d_b9: f64 = (s.db[204][9] + s.db[736][9]);let eq10_e1084_d_b10: f64 = (s.db[204][10] + s.db[736][10]);let eq10_e1084_d_b11: f64 = (s.db[204][11] + s.db[736][11]);let eq10_e1084_d_b12: f64 = (s.db[204][12] + s.db[736][12]);let eq10_e1086: f64 = (eq10_e1084 + s.v[432]);let eq10_e1086_d_n0: f64 = (eq10_e1084_d_n0 + s.dn[432][0]);let eq10_e1086_d_n1: f64 = (eq10_e1084_d_n1 + s.dn[432][1]);let eq10_e1086_d_n2: f64 = (eq10_e1084_d_n2 + s.dn[432][2]);let eq10_e1086_d_n3: f64 = (eq10_e1084_d_n3 + s.dn[432][3]);let eq10_e1086_d_n4: f64 = (eq10_e1084_d_n4 + s.dn[432][4]);let eq10_e1086_d_n5: f64 = (eq10_e1084_d_n5 + s.dn[432][5]);let eq10_e1086_d_n6: f64 = (eq10_e1084_d_n6 + s.dn[432][6]);let eq10_e1086_d_n7: f64 = (eq10_e1084_d_n7 + s.dn[432][7]);let eq10_e1086_d_n8: f64 = (eq10_e1084_d_n8 + s.dn[432][8]);let eq10_e1086_d_n9: f64 = (eq10_e1084_d_n9 + s.dn[432][9]);let eq10_e1086_d_n10: f64 = (eq10_e1084_d_n10 + s.dn[432][10]);let eq10_e1086_d_n11: f64 = (eq10_e1084_d_n11 + s.dn[432][11]);let eq10_e1086_d_n12: f64 = (eq10_e1084_d_n12 + s.dn[432][12]);let eq10_e1086_d_n13: f64 = (eq10_e1084_d_n13 + s.dn[432][13]);let eq10_e1086_d_n14: f64 = (eq10_e1084_d_n14 + s.dn[432][14]);let eq10_e1086_d_n15: f64 = (eq10_e1084_d_n15 + s.dn[432][15]);let eq10_e1086_d_n16: f64 = (eq10_e1084_d_n16 + s.dn[432][16]);let eq10_e1086_d_n17: f64 = (eq10_e1084_d_n17 + s.dn[432][17]);let eq10_e1086_d_n18: f64 = (eq10_e1084_d_n18 + s.dn[432][18]);let eq10_e1086_d_b0: f64 = (eq10_e1084_d_b0 + s.db[432][0]);let eq10_e1086_d_b1: f64 = (eq10_e1084_d_b1 + s.db[432][1]);let eq10_e1086_d_b2: f64 = (eq10_e1084_d_b2 + s.db[432][2]);let eq10_e1086_d_b3: f64 = (eq10_e1084_d_b3 + s.db[432][3]);let eq10_e1086_d_b4: f64 = (eq10_e1084_d_b4 + s.db[432][4]);let eq10_e1086_d_b5: f64 = (eq10_e1084_d_b5 + s.db[432][5]);let eq10_e1086_d_b6: f64 = (eq10_e1084_d_b6 + s.db[432][6]);let eq10_e1086_d_b7: f64 = (eq10_e1084_d_b7 + s.db[432][7]);let eq10_e1086_d_b8: f64 = (eq10_e1084_d_b8 + s.db[432][8]);let eq10_e1086_d_b9: f64 = (eq10_e1084_d_b9 + s.db[432][9]);let eq10_e1086_d_b10: f64 = (eq10_e1084_d_b10 + s.db[432][10]);let eq10_e1086_d_b11: f64 = (eq10_e1084_d_b11 + s.db[432][11]);let eq10_e1086_d_b12: f64 = (eq10_e1084_d_b12 + s.db[432][12]);let eq10_e1087: f64 = (p[87] * eq10_e1086);let eq10_e1087_d_n0: f64 = (p[87] * eq10_e1086_d_n0);let eq10_e1087_d_n1: f64 = (p[87] * eq10_e1086_d_n1);
        let eq10_e1087_d_n2: f64 = (p[87] * eq10_e1086_d_n2);let eq10_e1087_d_n3: f64 = (p[87] * eq10_e1086_d_n3);let eq10_e1087_d_n4: f64 = (p[87] * eq10_e1086_d_n4);let eq10_e1087_d_n5: f64 = (p[87] * eq10_e1086_d_n5);let eq10_e1087_d_n6: f64 = (p[87] * eq10_e1086_d_n6);let eq10_e1087_d_n7: f64 = (p[87] * eq10_e1086_d_n7);let eq10_e1087_d_n8: f64 = (p[87] * eq10_e1086_d_n8);let eq10_e1087_d_n9: f64 = (p[87] * eq10_e1086_d_n9);let eq10_e1087_d_n10: f64 = (p[87] * eq10_e1086_d_n10);let eq10_e1087_d_n11: f64 = (p[87] * eq10_e1086_d_n11);let eq10_e1087_d_n12: f64 = (p[87] * eq10_e1086_d_n12);let eq10_e1087_d_n13: f64 = (p[87] * eq10_e1086_d_n13);let eq10_e1087_d_n14: f64 = (p[87] * eq10_e1086_d_n14);let eq10_e1087_d_n15: f64 = (p[87] * eq10_e1086_d_n15);let eq10_e1087_d_n16: f64 = (p[87] * eq10_e1086_d_n16);let eq10_e1087_d_n17: f64 = (p[87] * eq10_e1086_d_n17);let eq10_e1087_d_n18: f64 = (p[87] * eq10_e1086_d_n18);let eq10_e1087_d_b0: f64 = (p[87] * eq10_e1086_d_b0);let eq10_e1087_d_b1: f64 = (p[87] * eq10_e1086_d_b1);let eq10_e1087_d_b2: f64 = (p[87] * eq10_e1086_d_b2);let eq10_e1087_d_b3: f64 = (p[87] * eq10_e1086_d_b3);let eq10_e1087_d_b4: f64 = (p[87] * eq10_e1086_d_b4);let eq10_e1087_d_b5: f64 = (p[87] * eq10_e1086_d_b5);let eq10_e1087_d_b6: f64 = (p[87] * eq10_e1086_d_b6);let eq10_e1087_d_b7: f64 = (p[87] * eq10_e1086_d_b7);let eq10_e1087_d_b8: f64 = (p[87] * eq10_e1086_d_b8);let eq10_e1087_d_b9: f64 = (p[87] * eq10_e1086_d_b9);let eq10_e1087_d_b10: f64 = (p[87] * eq10_e1086_d_b10);let eq10_e1087_d_b11: f64 = (p[87] * eq10_e1086_d_b11);let eq10_e1087_d_b12: f64 = (p[87] * eq10_e1086_d_b12);let eq10_value: f64 = eq10_e1087;let eq10_node_derivatives: [f64; 19] = [eq10_e1087_d_n0, eq10_e1087_d_n1, eq10_e1087_d_n2, eq10_e1087_d_n3, eq10_e1087_d_n4, eq10_e1087_d_n5, eq10_e1087_d_n6, eq10_e1087_d_n7, eq10_e1087_d_n8, eq10_e1087_d_n9, eq10_e1087_d_n10, eq10_e1087_d_n11, eq10_e1087_d_n12, eq10_e1087_d_n13, eq10_e1087_d_n14, eq10_e1087_d_n15, eq10_e1087_d_n16, eq10_e1087_d_n17, eq10_e1087_d_n18];let eq10_branch_derivatives: [f64; 13] = [eq10_e1087_d_b0, eq10_e1087_d_b1, eq10_e1087_d_b2, eq10_e1087_d_b3, eq10_e1087_d_b4, eq10_e1087_d_b5, eq10_e1087_d_b6, eq10_e1087_d_b7, eq10_e1087_d_b8, eq10_e1087_d_b9, eq10_e1087_d_b10, eq10_e1087_d_b11, eq10_e1087_d_b12];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(9),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_6(
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
        let eq11_e1090: f64 = (p[87] * s.v[281]);let eq11_value: f64 = eq11_e1090;
        stamper.stamp_current_dense_local(
            Some(0),
            Some(9),
            multiplicity * (eq11_value),
            &s.dn[281],
            &s.db[281],
            (multiplicity) * (p[87]),
        );let eq12_e1093: f64 = (p[87] * s.v[737]);let eq12_value: f64 = eq12_e1093;
        stamper.stamp_current_dense_local(
            Some(2),
            Some(9),
            multiplicity * (eq12_value),
            &s.dn[737],
            &s.db[737],
            (multiplicity) * (p[87]),
        );let eq13_e1096: f64 = (p[87] * s.v[862]);let eq13_value: f64 = eq13_e1096;
        stamper.stamp_current_dense_local(
            Some(11),
            Some(2),
            multiplicity * (eq13_value),
            &s.dn[862],
            &s.db[862],
            (multiplicity) * (p[87]),
        );let eq14_e1099: f64 = (p[87] * s.v[861]);let eq14_value: f64 = eq14_e1099;
        stamper.stamp_current_dense_local(
            Some(10),
            Some(0),
            multiplicity * (eq14_value),
            &s.dn[861],
            &s.db[861],
            (multiplicity) * (p[87]),
        );let eq15_e1102: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, s.v[66]);let eq15_e1103: f64 = (p[87] * eq15_e1102);let eq15_e1103_d_n0: f64 = (p[87] * (s.dn[66][0] * ddt_scale));let eq15_e1103_d_n1: f64 = (p[87] * (s.dn[66][1] * ddt_scale));let eq15_e1103_d_n2: f64 = (p[87] * (s.dn[66][2] * ddt_scale));let eq15_e1103_d_n3: f64 = (p[87] * (s.dn[66][3] * ddt_scale));let eq15_e1103_d_n4: f64 = (p[87] * (s.dn[66][4] * ddt_scale));let eq15_e1103_d_n5: f64 = (p[87] * (s.dn[66][5] * ddt_scale));let eq15_e1103_d_n6: f64 = (p[87] * (s.dn[66][6] * ddt_scale));let eq15_e1103_d_n7: f64 = (p[87] * (s.dn[66][7] * ddt_scale));let eq15_e1103_d_n8: f64 = (p[87] * (s.dn[66][8] * ddt_scale));let eq15_e1103_d_n9: f64 = (p[87] * (s.dn[66][9] * ddt_scale));let eq15_e1103_d_n10: f64 = (p[87] * (s.dn[66][10] * ddt_scale));let eq15_e1103_d_n11: f64 = (p[87] * (s.dn[66][11] * ddt_scale));let eq15_e1103_d_n12: f64 = (p[87] * (s.dn[66][12] * ddt_scale));let eq15_e1103_d_n13: f64 = (p[87] * (s.dn[66][13] * ddt_scale));let eq15_e1103_d_n14: f64 = (p[87] * (s.dn[66][14] * ddt_scale));let eq15_e1103_d_n15: f64 = (p[87] * (s.dn[66][15] * ddt_scale));let eq15_e1103_d_n16: f64 = (p[87] * (s.dn[66][16] * ddt_scale));let eq15_e1103_d_n17: f64 = (p[87] * (s.dn[66][17] * ddt_scale));let eq15_e1103_d_n18: f64 = (p[87] * (s.dn[66][18] * ddt_scale));let eq15_e1103_d_b0: f64 = (p[87] * (s.db[66][0] * ddt_scale));let eq15_e1103_d_b1: f64 = (p[87] * (s.db[66][1] * ddt_scale));let eq15_e1103_d_b2: f64 = (p[87] * (s.db[66][2] * ddt_scale));let eq15_e1103_d_b3: f64 = (p[87] * (s.db[66][3] * ddt_scale));let eq15_e1103_d_b4: f64 = (p[87] * (s.db[66][4] * ddt_scale));let eq15_e1103_d_b5: f64 = (p[87] * (s.db[66][5] * ddt_scale));let eq15_e1103_d_b6: f64 = (p[87] * (s.db[66][6] * ddt_scale));let eq15_e1103_d_b7: f64 = (p[87] * (s.db[66][7] * ddt_scale));let eq15_e1103_d_b8: f64 = (p[87] * (s.db[66][8] * ddt_scale));let eq15_e1103_d_b9: f64 = (p[87] * (s.db[66][9] * ddt_scale));let eq15_e1103_d_b10: f64 = (p[87] * (s.db[66][10] * ddt_scale));let eq15_e1103_d_b11: f64 = (p[87] * (s.db[66][11] * ddt_scale));let eq15_e1103_d_b12: f64 = (p[87] * (s.db[66][12] * ddt_scale));let eq15_value: f64 = eq15_e1103;let eq15_node_derivatives: [f64; 19] = [eq15_e1103_d_n0, eq15_e1103_d_n1, eq15_e1103_d_n2, eq15_e1103_d_n3, eq15_e1103_d_n4, eq15_e1103_d_n5, eq15_e1103_d_n6, eq15_e1103_d_n7, eq15_e1103_d_n8, eq15_e1103_d_n9, eq15_e1103_d_n10, eq15_e1103_d_n11, eq15_e1103_d_n12, eq15_e1103_d_n13, eq15_e1103_d_n14, eq15_e1103_d_n15, eq15_e1103_d_n16, eq15_e1103_d_n17, eq15_e1103_d_n18];let eq15_branch_derivatives: [f64; 13] = [eq15_e1103_d_b0, eq15_e1103_d_b1, eq15_e1103_d_b2, eq15_e1103_d_b3, eq15_e1103_d_b4, eq15_e1103_d_b5, eq15_e1103_d_b6, eq15_e1103_d_b7, eq15_e1103_d_b8, eq15_e1103_d_b9, eq15_e1103_d_b10, eq15_e1103_d_b11, eq15_e1103_d_b12];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(2),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );let eq16_e1106: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, s.v[65]);let eq16_e1107: f64 = (p[87] * eq16_e1106);let eq16_e1107_d_n0: f64 = (p[87] * (s.dn[65][0] * ddt_scale));let eq16_e1107_d_n1: f64 = (p[87] * (s.dn[65][1] * ddt_scale));let eq16_e1107_d_n2: f64 = (p[87] * (s.dn[65][2] * ddt_scale));let eq16_e1107_d_n3: f64 = (p[87] * (s.dn[65][3] * ddt_scale));let eq16_e1107_d_n4: f64 = (p[87] * (s.dn[65][4] * ddt_scale));let eq16_e1107_d_n5: f64 = (p[87] * (s.dn[65][5] * ddt_scale));let eq16_e1107_d_n6: f64 = (p[87] * (s.dn[65][6] * ddt_scale));let eq16_e1107_d_n7: f64 = (p[87] * (s.dn[65][7] * ddt_scale));let eq16_e1107_d_n8: f64 = (p[87] * (s.dn[65][8] * ddt_scale));let eq16_e1107_d_n9: f64 = (p[87] * (s.dn[65][9] * ddt_scale));let eq16_e1107_d_n10: f64 = (p[87] * (s.dn[65][10] * ddt_scale));let eq16_e1107_d_n11: f64 = (p[87] * (s.dn[65][11] * ddt_scale));let eq16_e1107_d_n12: f64 = (p[87] * (s.dn[65][12] * ddt_scale));let eq16_e1107_d_n13: f64 = (p[87] * (s.dn[65][13] * ddt_scale));let eq16_e1107_d_n14: f64 = (p[87] * (s.dn[65][14] * ddt_scale));let eq16_e1107_d_n15: f64 = (p[87] * (s.dn[65][15] * ddt_scale));let eq16_e1107_d_n16: f64 = (p[87] * (s.dn[65][16] * ddt_scale));let eq16_e1107_d_n17: f64 = (p[87] * (s.dn[65][17] * ddt_scale));let eq16_e1107_d_n18: f64 = (p[87] * (s.dn[65][18] * ddt_scale));let eq16_e1107_d_b0: f64 = (p[87] * (s.db[65][0] * ddt_scale));let eq16_e1107_d_b1: f64 = (p[87] * (s.db[65][1] * ddt_scale));let eq16_e1107_d_b2: f64 = (p[87] * (s.db[65][2] * ddt_scale));let eq16_e1107_d_b3: f64 = (p[87] * (s.db[65][3] * ddt_scale));let eq16_e1107_d_b4: f64 = (p[87] * (s.db[65][4] * ddt_scale));let eq16_e1107_d_b5: f64 = (p[87] * (s.db[65][5] * ddt_scale));let eq16_e1107_d_b6: f64 = (p[87] * (s.db[65][6] * ddt_scale));let eq16_e1107_d_b7: f64 = (p[87] * (s.db[65][7] * ddt_scale));let eq16_e1107_d_b8: f64 = (p[87] * (s.db[65][8] * ddt_scale));let eq16_e1107_d_b9: f64 = (p[87] * (s.db[65][9] * ddt_scale));let eq16_e1107_d_b10: f64 = (p[87] * (s.db[65][10] * ddt_scale));let eq16_e1107_d_b11: f64 = (p[87] * (s.db[65][11] * ddt_scale));let eq16_e1107_d_b12: f64 = (p[87] * (s.db[65][12] * ddt_scale));let eq16_value: f64 = eq16_e1107;let eq16_node_derivatives: [f64; 19] = [eq16_e1107_d_n0, eq16_e1107_d_n1, eq16_e1107_d_n2, eq16_e1107_d_n3, eq16_e1107_d_n4, eq16_e1107_d_n5, eq16_e1107_d_n6, eq16_e1107_d_n7, eq16_e1107_d_n8, eq16_e1107_d_n9, eq16_e1107_d_n10, eq16_e1107_d_n11, eq16_e1107_d_n12, eq16_e1107_d_n13, eq16_e1107_d_n14, eq16_e1107_d_n15, eq16_e1107_d_n16, eq16_e1107_d_n17, eq16_e1107_d_n18];let eq16_branch_derivatives: [f64; 13] = [eq16_e1107_d_b0, eq16_e1107_d_b1, eq16_e1107_d_b2, eq16_e1107_d_b3, eq16_e1107_d_b4, eq16_e1107_d_b5, eq16_e1107_d_b6, eq16_e1107_d_b7, eq16_e1107_d_b8, eq16_e1107_d_b9, eq16_e1107_d_b10, eq16_e1107_d_b11, eq16_e1107_d_b12];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(0),
            multiplicity * (eq16_value),
            &eq16_node_derivatives,
            &eq16_branch_derivatives,
            multiplicity,
        );
    }
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
        let (eq17_e1113, eq17_e1113_d_n0, eq17_e1113_d_n1, eq17_e1113_d_n2, eq17_e1113_d_n3, eq17_e1113_d_n4, eq17_e1113_d_n5, eq17_e1113_d_n6, eq17_e1113_d_n7, eq17_e1113_d_n8, eq17_e1113_d_n9, eq17_e1113_d_n10, eq17_e1113_d_n11, eq17_e1113_d_n12, eq17_e1113_d_n13, eq17_e1113_d_n14, eq17_e1113_d_n15, eq17_e1113_d_n16, eq17_e1113_d_n17, eq17_e1113_d_n18, eq17_e1113_d_b0, eq17_e1113_d_b1, eq17_e1113_d_b2, eq17_e1113_d_b3, eq17_e1113_d_b4, eq17_e1113_d_b5, eq17_e1113_d_b6, eq17_e1113_d_b7, eq17_e1113_d_b8, eq17_e1113_d_b9, eq17_e1113_d_b10, eq17_e1113_d_b11, eq17_e1113_d_b12,) = {
    if s.b[3409] {
        let eq17_e1111: f64 = (p[87] * s.v[870]);
        (eq17_e1111, (p[87] * s.dn[870][0]), (p[87] * s.dn[870][1]), (p[87] * s.dn[870][2]), (p[87] * s.dn[870][3]), (p[87] * s.dn[870][4]), (p[87] * s.dn[870][5]), (p[87] * s.dn[870][6]), (p[87] * s.dn[870][7]), (p[87] * s.dn[870][8]), (p[87] * s.dn[870][9]), (p[87] * s.dn[870][10]), (p[87] * s.dn[870][11]), (p[87] * s.dn[870][12]), (p[87] * s.dn[870][13]), (p[87] * s.dn[870][14]), (p[87] * s.dn[870][15]), (p[87] * s.dn[870][16]), (p[87] * s.dn[870][17]), (p[87] * s.dn[870][18]), (p[87] * s.db[870][0]), (p[87] * s.db[870][1]), (p[87] * s.db[870][2]), (p[87] * s.db[870][3]), (p[87] * s.db[870][4]), (p[87] * s.db[870][5]), (p[87] * s.db[870][6]), (p[87] * s.db[870][7]), (p[87] * s.db[870][8]), (p[87] * s.db[870][9]), (p[87] * s.db[870][10]), (p[87] * s.db[870][11]), (p[87] * s.db[870][12]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e1113;let eq17_node_derivatives: [f64; 19] = [eq17_e1113_d_n0, eq17_e1113_d_n1, eq17_e1113_d_n2, eq17_e1113_d_n3, eq17_e1113_d_n4, eq17_e1113_d_n5, eq17_e1113_d_n6, eq17_e1113_d_n7, eq17_e1113_d_n8, eq17_e1113_d_n9, eq17_e1113_d_n10, eq17_e1113_d_n11, eq17_e1113_d_n12, eq17_e1113_d_n13, eq17_e1113_d_n14, eq17_e1113_d_n15, eq17_e1113_d_n16, eq17_e1113_d_n17, eq17_e1113_d_n18];let eq17_branch_derivatives: [f64; 13] = [eq17_e1113_d_b0, eq17_e1113_d_b1, eq17_e1113_d_b2, eq17_e1113_d_b3, eq17_e1113_d_b4, eq17_e1113_d_b5, eq17_e1113_d_b6, eq17_e1113_d_b7, eq17_e1113_d_b8, eq17_e1113_d_b9, eq17_e1113_d_b10, eq17_e1113_d_b11, eq17_e1113_d_b12];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let (eq18_e1119, eq18_e1119_d_n0, eq18_e1119_d_n1, eq18_e1119_d_n2, eq18_e1119_d_n3, eq18_e1119_d_n4, eq18_e1119_d_n5, eq18_e1119_d_n6, eq18_e1119_d_n7, eq18_e1119_d_n8, eq18_e1119_d_n9, eq18_e1119_d_n10, eq18_e1119_d_n11, eq18_e1119_d_n12, eq18_e1119_d_n13, eq18_e1119_d_n14, eq18_e1119_d_n15, eq18_e1119_d_n16, eq18_e1119_d_n17, eq18_e1119_d_n18, eq18_e1119_d_b0, eq18_e1119_d_b1, eq18_e1119_d_b2, eq18_e1119_d_b3, eq18_e1119_d_b4, eq18_e1119_d_b5, eq18_e1119_d_b6, eq18_e1119_d_b7, eq18_e1119_d_b8, eq18_e1119_d_b9, eq18_e1119_d_b10, eq18_e1119_d_b11, eq18_e1119_d_b12,) = {
    if s.b[3409] {
        let eq18_e1117: f64 = (p[87] * s.v[869]);
        (eq18_e1117, (p[87] * s.dn[869][0]), (p[87] * s.dn[869][1]), (p[87] * s.dn[869][2]), (p[87] * s.dn[869][3]), (p[87] * s.dn[869][4]), (p[87] * s.dn[869][5]), (p[87] * s.dn[869][6]), (p[87] * s.dn[869][7]), (p[87] * s.dn[869][8]), (p[87] * s.dn[869][9]), (p[87] * s.dn[869][10]), (p[87] * s.dn[869][11]), (p[87] * s.dn[869][12]), (p[87] * s.dn[869][13]), (p[87] * s.dn[869][14]), (p[87] * s.dn[869][15]), (p[87] * s.dn[869][16]), (p[87] * s.dn[869][17]), (p[87] * s.dn[869][18]), (p[87] * s.db[869][0]), (p[87] * s.db[869][1]), (p[87] * s.db[869][2]), (p[87] * s.db[869][3]), (p[87] * s.db[869][4]), (p[87] * s.db[869][5]), (p[87] * s.db[869][6]), (p[87] * s.db[869][7]), (p[87] * s.db[869][8]), (p[87] * s.db[869][9]), (p[87] * s.db[869][10]), (p[87] * s.db[869][11]), (p[87] * s.db[869][12]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1119;let eq18_node_derivatives: [f64; 19] = [eq18_e1119_d_n0, eq18_e1119_d_n1, eq18_e1119_d_n2, eq18_e1119_d_n3, eq18_e1119_d_n4, eq18_e1119_d_n5, eq18_e1119_d_n6, eq18_e1119_d_n7, eq18_e1119_d_n8, eq18_e1119_d_n9, eq18_e1119_d_n10, eq18_e1119_d_n11, eq18_e1119_d_n12, eq18_e1119_d_n13, eq18_e1119_d_n14, eq18_e1119_d_n15, eq18_e1119_d_n16, eq18_e1119_d_n17, eq18_e1119_d_n18];let eq18_branch_derivatives: [f64; 13] = [eq18_e1119_d_b0, eq18_e1119_d_b1, eq18_e1119_d_b2, eq18_e1119_d_b3, eq18_e1119_d_b4, eq18_e1119_d_b5, eq18_e1119_d_b6, eq18_e1119_d_b7, eq18_e1119_d_b8, eq18_e1119_d_b9, eq18_e1119_d_b10, eq18_e1119_d_b11, eq18_e1119_d_b12];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let (eq19_e1126, eq19_e1126_d_n0, eq19_e1126_d_n1, eq19_e1126_d_n2, eq19_e1126_d_n3, eq19_e1126_d_n4, eq19_e1126_d_n5, eq19_e1126_d_n6, eq19_e1126_d_n7, eq19_e1126_d_n8, eq19_e1126_d_n9, eq19_e1126_d_n10, eq19_e1126_d_n11, eq19_e1126_d_n12, eq19_e1126_d_n13, eq19_e1126_d_n14, eq19_e1126_d_n15, eq19_e1126_d_n16, eq19_e1126_d_n17, eq19_e1126_d_n18, eq19_e1126_d_b0, eq19_e1126_d_b1, eq19_e1126_d_b2, eq19_e1126_d_b3, eq19_e1126_d_b4, eq19_e1126_d_b5, eq19_e1126_d_b6, eq19_e1126_d_b7, eq19_e1126_d_b8, eq19_e1126_d_b9, eq19_e1126_d_b10, eq19_e1126_d_b11, eq19_e1126_d_b12,) = {
    if s.b[3409] {
        let eq19_e1123: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, s.v[68]);let eq19_e1124: f64 = (p[87] * eq19_e1123);let eq19_e1124_d_n0: f64 = (p[87] * (s.dn[68][0] * ddt_scale));let eq19_e1124_d_n1: f64 = (p[87] * (s.dn[68][1] * ddt_scale));let eq19_e1124_d_n2: f64 = (p[87] * (s.dn[68][2] * ddt_scale));let eq19_e1124_d_n3: f64 = (p[87] * (s.dn[68][3] * ddt_scale));let eq19_e1124_d_n4: f64 = (p[87] * (s.dn[68][4] * ddt_scale));let eq19_e1124_d_n5: f64 = (p[87] * (s.dn[68][5] * ddt_scale));let eq19_e1124_d_n6: f64 = (p[87] * (s.dn[68][6] * ddt_scale));let eq19_e1124_d_n7: f64 = (p[87] * (s.dn[68][7] * ddt_scale));let eq19_e1124_d_n8: f64 = (p[87] * (s.dn[68][8] * ddt_scale));let eq19_e1124_d_n9: f64 = (p[87] * (s.dn[68][9] * ddt_scale));let eq19_e1124_d_n10: f64 = (p[87] * (s.dn[68][10] * ddt_scale));let eq19_e1124_d_n11: f64 = (p[87] * (s.dn[68][11] * ddt_scale));let eq19_e1124_d_n12: f64 = (p[87] * (s.dn[68][12] * ddt_scale));let eq19_e1124_d_n13: f64 = (p[87] * (s.dn[68][13] * ddt_scale));let eq19_e1124_d_n14: f64 = (p[87] * (s.dn[68][14] * ddt_scale));let eq19_e1124_d_n15: f64 = (p[87] * (s.dn[68][15] * ddt_scale));let eq19_e1124_d_n16: f64 = (p[87] * (s.dn[68][16] * ddt_scale));let eq19_e1124_d_n17: f64 = (p[87] * (s.dn[68][17] * ddt_scale));let eq19_e1124_d_n18: f64 = (p[87] * (s.dn[68][18] * ddt_scale));let eq19_e1124_d_b0: f64 = (p[87] * (s.db[68][0] * ddt_scale));let eq19_e1124_d_b1: f64 = (p[87] * (s.db[68][1] * ddt_scale));let eq19_e1124_d_b2: f64 = (p[87] * (s.db[68][2] * ddt_scale));let eq19_e1124_d_b3: f64 = (p[87] * (s.db[68][3] * ddt_scale));let eq19_e1124_d_b4: f64 = (p[87] * (s.db[68][4] * ddt_scale));let eq19_e1124_d_b5: f64 = (p[87] * (s.db[68][5] * ddt_scale));let eq19_e1124_d_b6: f64 = (p[87] * (s.db[68][6] * ddt_scale));let eq19_e1124_d_b7: f64 = (p[87] * (s.db[68][7] * ddt_scale));let eq19_e1124_d_b8: f64 = (p[87] * (s.db[68][8] * ddt_scale));let eq19_e1124_d_b9: f64 = (p[87] * (s.db[68][9] * ddt_scale));let eq19_e1124_d_b10: f64 = (p[87] * (s.db[68][10] * ddt_scale));let eq19_e1124_d_b11: f64 = (p[87] * (s.db[68][11] * ddt_scale));let eq19_e1124_d_b12: f64 = (p[87] * (s.db[68][12] * ddt_scale));
        (eq19_e1124, eq19_e1124_d_n0, eq19_e1124_d_n1, eq19_e1124_d_n2, eq19_e1124_d_n3, eq19_e1124_d_n4, eq19_e1124_d_n5, eq19_e1124_d_n6, eq19_e1124_d_n7, eq19_e1124_d_n8, eq19_e1124_d_n9, eq19_e1124_d_n10, eq19_e1124_d_n11, eq19_e1124_d_n12, eq19_e1124_d_n13, eq19_e1124_d_n14, eq19_e1124_d_n15, eq19_e1124_d_n16, eq19_e1124_d_n17, eq19_e1124_d_n18, eq19_e1124_d_b0, eq19_e1124_d_b1, eq19_e1124_d_b2, eq19_e1124_d_b3, eq19_e1124_d_b4, eq19_e1124_d_b5, eq19_e1124_d_b6, eq19_e1124_d_b7, eq19_e1124_d_b8, eq19_e1124_d_b9, eq19_e1124_d_b10, eq19_e1124_d_b11, eq19_e1124_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_value: f64 = eq19_e1126;let eq19_node_derivatives: [f64; 19] = [eq19_e1126_d_n0, eq19_e1126_d_n1, eq19_e1126_d_n2, eq19_e1126_d_n3, eq19_e1126_d_n4, eq19_e1126_d_n5, eq19_e1126_d_n6, eq19_e1126_d_n7, eq19_e1126_d_n8, eq19_e1126_d_n9, eq19_e1126_d_n10, eq19_e1126_d_n11, eq19_e1126_d_n12, eq19_e1126_d_n13, eq19_e1126_d_n14, eq19_e1126_d_n15, eq19_e1126_d_n16, eq19_e1126_d_n17, eq19_e1126_d_n18];let eq19_branch_derivatives: [f64; 13] = [eq19_e1126_d_b0, eq19_e1126_d_b1, eq19_e1126_d_b2, eq19_e1126_d_b3, eq19_e1126_d_b4, eq19_e1126_d_b5, eq19_e1126_d_b6, eq19_e1126_d_b7, eq19_e1126_d_b8, eq19_e1126_d_b9, eq19_e1126_d_b10, eq19_e1126_d_b11, eq19_e1126_d_b12];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_8(
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
        let (eq20_e1133, eq20_e1133_d_n0, eq20_e1133_d_n1, eq20_e1133_d_n2, eq20_e1133_d_n3, eq20_e1133_d_n4, eq20_e1133_d_n5, eq20_e1133_d_n6, eq20_e1133_d_n7, eq20_e1133_d_n8, eq20_e1133_d_n9, eq20_e1133_d_n10, eq20_e1133_d_n11, eq20_e1133_d_n12, eq20_e1133_d_n13, eq20_e1133_d_n14, eq20_e1133_d_n15, eq20_e1133_d_n16, eq20_e1133_d_n17, eq20_e1133_d_n18, eq20_e1133_d_b0, eq20_e1133_d_b1, eq20_e1133_d_b2, eq20_e1133_d_b3, eq20_e1133_d_b4, eq20_e1133_d_b5, eq20_e1133_d_b6, eq20_e1133_d_b7, eq20_e1133_d_b8, eq20_e1133_d_b9, eq20_e1133_d_b10, eq20_e1133_d_b11, eq20_e1133_d_b12,) = {
    if s.b[3409] {
        let eq20_e1130: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, s.v[67]);let eq20_e1131: f64 = (p[87] * eq20_e1130);let eq20_e1131_d_n0: f64 = (p[87] * (s.dn[67][0] * ddt_scale));let eq20_e1131_d_n1: f64 = (p[87] * (s.dn[67][1] * ddt_scale));let eq20_e1131_d_n2: f64 = (p[87] * (s.dn[67][2] * ddt_scale));let eq20_e1131_d_n3: f64 = (p[87] * (s.dn[67][3] * ddt_scale));let eq20_e1131_d_n4: f64 = (p[87] * (s.dn[67][4] * ddt_scale));let eq20_e1131_d_n5: f64 = (p[87] * (s.dn[67][5] * ddt_scale));let eq20_e1131_d_n6: f64 = (p[87] * (s.dn[67][6] * ddt_scale));let eq20_e1131_d_n7: f64 = (p[87] * (s.dn[67][7] * ddt_scale));let eq20_e1131_d_n8: f64 = (p[87] * (s.dn[67][8] * ddt_scale));let eq20_e1131_d_n9: f64 = (p[87] * (s.dn[67][9] * ddt_scale));let eq20_e1131_d_n10: f64 = (p[87] * (s.dn[67][10] * ddt_scale));let eq20_e1131_d_n11: f64 = (p[87] * (s.dn[67][11] * ddt_scale));let eq20_e1131_d_n12: f64 = (p[87] * (s.dn[67][12] * ddt_scale));let eq20_e1131_d_n13: f64 = (p[87] * (s.dn[67][13] * ddt_scale));let eq20_e1131_d_n14: f64 = (p[87] * (s.dn[67][14] * ddt_scale));let eq20_e1131_d_n15: f64 = (p[87] * (s.dn[67][15] * ddt_scale));let eq20_e1131_d_n16: f64 = (p[87] * (s.dn[67][16] * ddt_scale));let eq20_e1131_d_n17: f64 = (p[87] * (s.dn[67][17] * ddt_scale));let eq20_e1131_d_n18: f64 = (p[87] * (s.dn[67][18] * ddt_scale));let eq20_e1131_d_b0: f64 = (p[87] * (s.db[67][0] * ddt_scale));let eq20_e1131_d_b1: f64 = (p[87] * (s.db[67][1] * ddt_scale));let eq20_e1131_d_b2: f64 = (p[87] * (s.db[67][2] * ddt_scale));let eq20_e1131_d_b3: f64 = (p[87] * (s.db[67][3] * ddt_scale));let eq20_e1131_d_b4: f64 = (p[87] * (s.db[67][4] * ddt_scale));let eq20_e1131_d_b5: f64 = (p[87] * (s.db[67][5] * ddt_scale));let eq20_e1131_d_b6: f64 = (p[87] * (s.db[67][6] * ddt_scale));let eq20_e1131_d_b7: f64 = (p[87] * (s.db[67][7] * ddt_scale));let eq20_e1131_d_b8: f64 = (p[87] * (s.db[67][8] * ddt_scale));let eq20_e1131_d_b9: f64 = (p[87] * (s.db[67][9] * ddt_scale));let eq20_e1131_d_b10: f64 = (p[87] * (s.db[67][10] * ddt_scale));let eq20_e1131_d_b11: f64 = (p[87] * (s.db[67][11] * ddt_scale));let eq20_e1131_d_b12: f64 = (p[87] * (s.db[67][12] * ddt_scale));
        (eq20_e1131, eq20_e1131_d_n0, eq20_e1131_d_n1, eq20_e1131_d_n2, eq20_e1131_d_n3, eq20_e1131_d_n4, eq20_e1131_d_n5, eq20_e1131_d_n6, eq20_e1131_d_n7, eq20_e1131_d_n8, eq20_e1131_d_n9, eq20_e1131_d_n10, eq20_e1131_d_n11, eq20_e1131_d_n12, eq20_e1131_d_n13, eq20_e1131_d_n14, eq20_e1131_d_n15, eq20_e1131_d_n16, eq20_e1131_d_n17, eq20_e1131_d_n18, eq20_e1131_d_b0, eq20_e1131_d_b1, eq20_e1131_d_b2, eq20_e1131_d_b3, eq20_e1131_d_b4, eq20_e1131_d_b5, eq20_e1131_d_b6, eq20_e1131_d_b7, eq20_e1131_d_b8, eq20_e1131_d_b9, eq20_e1131_d_b10, eq20_e1131_d_b11, eq20_e1131_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e1133;let eq20_node_derivatives: [f64; 19] = [eq20_e1133_d_n0, eq20_e1133_d_n1, eq20_e1133_d_n2, eq20_e1133_d_n3, eq20_e1133_d_n4, eq20_e1133_d_n5, eq20_e1133_d_n6, eq20_e1133_d_n7, eq20_e1133_d_n8, eq20_e1133_d_n9, eq20_e1133_d_n10, eq20_e1133_d_n11, eq20_e1133_d_n12, eq20_e1133_d_n13, eq20_e1133_d_n14, eq20_e1133_d_n15, eq20_e1133_d_n16, eq20_e1133_d_n17, eq20_e1133_d_n18];let eq20_branch_derivatives: [f64; 13] = [eq20_e1133_d_b0, eq20_e1133_d_b1, eq20_e1133_d_b2, eq20_e1133_d_b3, eq20_e1133_d_b4, eq20_e1133_d_b5, eq20_e1133_d_b6, eq20_e1133_d_b7, eq20_e1133_d_b8, eq20_e1133_d_b9, eq20_e1133_d_b10, eq20_e1133_d_b11, eq20_e1133_d_b12];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let (eq21_e1139, eq21_e1139_d_n0, eq21_e1139_d_n1, eq21_e1139_d_n2, eq21_e1139_d_n3, eq21_e1139_d_n4, eq21_e1139_d_n5, eq21_e1139_d_n6, eq21_e1139_d_n7, eq21_e1139_d_n8, eq21_e1139_d_n9, eq21_e1139_d_n10, eq21_e1139_d_n11, eq21_e1139_d_n12, eq21_e1139_d_n13, eq21_e1139_d_n14, eq21_e1139_d_n15, eq21_e1139_d_n16, eq21_e1139_d_n17, eq21_e1139_d_n18, eq21_e1139_d_b0, eq21_e1139_d_b1, eq21_e1139_d_b2, eq21_e1139_d_b3, eq21_e1139_d_b4, eq21_e1139_d_b5, eq21_e1139_d_b6, eq21_e1139_d_b7, eq21_e1139_d_b8, eq21_e1139_d_b9, eq21_e1139_d_b10, eq21_e1139_d_b11, eq21_e1139_d_b12,) = {
    if s.b[3410] {
        let eq21_e1137: f64 = (p[87] * s.v[200]);
        (eq21_e1137, (p[87] * s.dn[200][0]), (p[87] * s.dn[200][1]), (p[87] * s.dn[200][2]), (p[87] * s.dn[200][3]), (p[87] * s.dn[200][4]), (p[87] * s.dn[200][5]), (p[87] * s.dn[200][6]), (p[87] * s.dn[200][7]), (p[87] * s.dn[200][8]), (p[87] * s.dn[200][9]), (p[87] * s.dn[200][10]), (p[87] * s.dn[200][11]), (p[87] * s.dn[200][12]), (p[87] * s.dn[200][13]), (p[87] * s.dn[200][14]), (p[87] * s.dn[200][15]), (p[87] * s.dn[200][16]), (p[87] * s.dn[200][17]), (p[87] * s.dn[200][18]), (p[87] * s.db[200][0]), (p[87] * s.db[200][1]), (p[87] * s.db[200][2]), (p[87] * s.db[200][3]), (p[87] * s.db[200][4]), (p[87] * s.db[200][5]), (p[87] * s.db[200][6]), (p[87] * s.db[200][7]), (p[87] * s.db[200][8]), (p[87] * s.db[200][9]), (p[87] * s.db[200][10]), (p[87] * s.db[200][11]), (p[87] * s.db[200][12]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1139;let eq21_node_derivatives: [f64; 19] = [eq21_e1139_d_n0, eq21_e1139_d_n1, eq21_e1139_d_n2, eq21_e1139_d_n3, eq21_e1139_d_n4, eq21_e1139_d_n5, eq21_e1139_d_n6, eq21_e1139_d_n7, eq21_e1139_d_n8, eq21_e1139_d_n9, eq21_e1139_d_n10, eq21_e1139_d_n11, eq21_e1139_d_n12, eq21_e1139_d_n13, eq21_e1139_d_n14, eq21_e1139_d_n15, eq21_e1139_d_n16, eq21_e1139_d_n17, eq21_e1139_d_n18];let eq21_branch_derivatives: [f64; 13] = [eq21_e1139_d_b0, eq21_e1139_d_b1, eq21_e1139_d_b2, eq21_e1139_d_b3, eq21_e1139_d_b4, eq21_e1139_d_b5, eq21_e1139_d_b6, eq21_e1139_d_b7, eq21_e1139_d_b8, eq21_e1139_d_b9, eq21_e1139_d_b10, eq21_e1139_d_b11, eq21_e1139_d_b12];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let (eq22_e1145, eq22_e1145_d_n0, eq22_e1145_d_n1, eq22_e1145_d_n2, eq22_e1145_d_n3, eq22_e1145_d_n4, eq22_e1145_d_n5, eq22_e1145_d_n6, eq22_e1145_d_n7, eq22_e1145_d_n8, eq22_e1145_d_n9, eq22_e1145_d_n10, eq22_e1145_d_n11, eq22_e1145_d_n12, eq22_e1145_d_n13, eq22_e1145_d_n14, eq22_e1145_d_n15, eq22_e1145_d_n16, eq22_e1145_d_n17, eq22_e1145_d_n18, eq22_e1145_d_b0, eq22_e1145_d_b1, eq22_e1145_d_b2, eq22_e1145_d_b3, eq22_e1145_d_b4, eq22_e1145_d_b5, eq22_e1145_d_b6, eq22_e1145_d_b7, eq22_e1145_d_b8, eq22_e1145_d_b9, eq22_e1145_d_b10, eq22_e1145_d_b11, eq22_e1145_d_b12,) = {
    if s.b[3410] {
        let eq22_e1143: f64 = (p[87] * s.v[201]);
        (eq22_e1143, (p[87] * s.dn[201][0]), (p[87] * s.dn[201][1]), (p[87] * s.dn[201][2]), (p[87] * s.dn[201][3]), (p[87] * s.dn[201][4]), (p[87] * s.dn[201][5]), (p[87] * s.dn[201][6]), (p[87] * s.dn[201][7]), (p[87] * s.dn[201][8]), (p[87] * s.dn[201][9]), (p[87] * s.dn[201][10]), (p[87] * s.dn[201][11]), (p[87] * s.dn[201][12]), (p[87] * s.dn[201][13]), (p[87] * s.dn[201][14]), (p[87] * s.dn[201][15]), (p[87] * s.dn[201][16]), (p[87] * s.dn[201][17]), (p[87] * s.dn[201][18]), (p[87] * s.db[201][0]), (p[87] * s.db[201][1]), (p[87] * s.db[201][2]), (p[87] * s.db[201][3]), (p[87] * s.db[201][4]), (p[87] * s.db[201][5]), (p[87] * s.db[201][6]), (p[87] * s.db[201][7]), (p[87] * s.db[201][8]), (p[87] * s.db[201][9]), (p[87] * s.db[201][10]), (p[87] * s.db[201][11]), (p[87] * s.db[201][12]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq22_value: f64 = eq22_e1145;let eq22_node_derivatives: [f64; 19] = [eq22_e1145_d_n0, eq22_e1145_d_n1, eq22_e1145_d_n2, eq22_e1145_d_n3, eq22_e1145_d_n4, eq22_e1145_d_n5, eq22_e1145_d_n6, eq22_e1145_d_n7, eq22_e1145_d_n8, eq22_e1145_d_n9, eq22_e1145_d_n10, eq22_e1145_d_n11, eq22_e1145_d_n12, eq22_e1145_d_n13, eq22_e1145_d_n14, eq22_e1145_d_n15, eq22_e1145_d_n16, eq22_e1145_d_n17, eq22_e1145_d_n18];let eq22_branch_derivatives: [f64; 13] = [eq22_e1145_d_b0, eq22_e1145_d_b1, eq22_e1145_d_b2, eq22_e1145_d_b3, eq22_e1145_d_b4, eq22_e1145_d_b5, eq22_e1145_d_b6, eq22_e1145_d_b7, eq22_e1145_d_b8, eq22_e1145_d_b9, eq22_e1145_d_b10, eq22_e1145_d_b11, eq22_e1145_d_b12];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq22_value),
            &eq22_node_derivatives,
            &eq22_branch_derivatives,
            multiplicity,
        );
        let (eq23_e1151, eq23_e1151_d_n0, eq23_e1151_d_n1, eq23_e1151_d_n2, eq23_e1151_d_n3, eq23_e1151_d_n4, eq23_e1151_d_n5, eq23_e1151_d_n6, eq23_e1151_d_n7, eq23_e1151_d_n8, eq23_e1151_d_n9, eq23_e1151_d_n10, eq23_e1151_d_n11, eq23_e1151_d_n12, eq23_e1151_d_n13, eq23_e1151_d_n14, eq23_e1151_d_n15, eq23_e1151_d_n16, eq23_e1151_d_n17, eq23_e1151_d_n18, eq23_e1151_d_b0, eq23_e1151_d_b1, eq23_e1151_d_b2, eq23_e1151_d_b3, eq23_e1151_d_b4, eq23_e1151_d_b5, eq23_e1151_d_b6, eq23_e1151_d_b7, eq23_e1151_d_b8, eq23_e1151_d_b9, eq23_e1151_d_b10, eq23_e1151_d_b11, eq23_e1151_d_b12,) = {
    if s.b[3410] {
        let eq23_e1149: f64 = (p[87] * s.v[202]);
        (eq23_e1149, (p[87] * s.dn[202][0]), (p[87] * s.dn[202][1]), (p[87] * s.dn[202][2]), (p[87] * s.dn[202][3]), (p[87] * s.dn[202][4]), (p[87] * s.dn[202][5]), (p[87] * s.dn[202][6]), (p[87] * s.dn[202][7]), (p[87] * s.dn[202][8]), (p[87] * s.dn[202][9]), (p[87] * s.dn[202][10]), (p[87] * s.dn[202][11]), (p[87] * s.dn[202][12]), (p[87] * s.dn[202][13]), (p[87] * s.dn[202][14]), (p[87] * s.dn[202][15]), (p[87] * s.dn[202][16]), (p[87] * s.dn[202][17]), (p[87] * s.dn[202][18]), (p[87] * s.db[202][0]), (p[87] * s.db[202][1]), (p[87] * s.db[202][2]), (p[87] * s.db[202][3]), (p[87] * s.db[202][4]), (p[87] * s.db[202][5]), (p[87] * s.db[202][6]), (p[87] * s.db[202][7]), (p[87] * s.db[202][8]), (p[87] * s.db[202][9]), (p[87] * s.db[202][10]), (p[87] * s.db[202][11]), (p[87] * s.db[202][12]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e1151;let eq23_node_derivatives: [f64; 19] = [eq23_e1151_d_n0, eq23_e1151_d_n1, eq23_e1151_d_n2, eq23_e1151_d_n3, eq23_e1151_d_n4, eq23_e1151_d_n5, eq23_e1151_d_n6, eq23_e1151_d_n7, eq23_e1151_d_n8, eq23_e1151_d_n9, eq23_e1151_d_n10, eq23_e1151_d_n11, eq23_e1151_d_n12, eq23_e1151_d_n13, eq23_e1151_d_n14, eq23_e1151_d_n15, eq23_e1151_d_n16, eq23_e1151_d_n17, eq23_e1151_d_n18];let eq23_branch_derivatives: [f64; 13] = [eq23_e1151_d_b0, eq23_e1151_d_b1, eq23_e1151_d_b2, eq23_e1151_d_b3, eq23_e1151_d_b4, eq23_e1151_d_b5, eq23_e1151_d_b6, eq23_e1151_d_b7, eq23_e1151_d_b8, eq23_e1151_d_b9, eq23_e1151_d_b10, eq23_e1151_d_b11, eq23_e1151_d_b12];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_9(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);let nv6 = ctx.node_voltage(nodes[6]);let nv8 = ctx.node_voltage(nodes[8]);
        let (eq24_e1157, eq24_e1157_d_n0, eq24_e1157_d_n1, eq24_e1157_d_n2, eq24_e1157_d_n3, eq24_e1157_d_n4, eq24_e1157_d_n5, eq24_e1157_d_n6, eq24_e1157_d_n7, eq24_e1157_d_n8, eq24_e1157_d_n9, eq24_e1157_d_n10, eq24_e1157_d_n11, eq24_e1157_d_n12, eq24_e1157_d_n13, eq24_e1157_d_n14, eq24_e1157_d_n15, eq24_e1157_d_n16, eq24_e1157_d_n17, eq24_e1157_d_n18, eq24_e1157_d_b0, eq24_e1157_d_b1, eq24_e1157_d_b2, eq24_e1157_d_b3, eq24_e1157_d_b4, eq24_e1157_d_b5, eq24_e1157_d_b6, eq24_e1157_d_b7, eq24_e1157_d_b8, eq24_e1157_d_b9, eq24_e1157_d_b10, eq24_e1157_d_b11, eq24_e1157_d_b12,) = {
    if (s.v[75] != 0.0) {
        let eq24_e1155: f64 = ((nv0 - nv6) / s.v[4]);let eq24_e1155_d_n0: f64 = ((s.v[4] - ((nv0 - nv6) * s.dn[4][0])) / (s.v[4] * s.v[4]));let eq24_e1155_d_n1: f64 = (-(((nv0 - nv6) * s.dn[4][1]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n2: f64 = (-(((nv0 - nv6) * s.dn[4][2]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n3: f64 = (-(((nv0 - nv6) * s.dn[4][3]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n4: f64 = (-(((nv0 - nv6) * s.dn[4][4]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n5: f64 = (-(((nv0 - nv6) * s.dn[4][5]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n6: f64 = (((-s.v[4]) - ((nv0 - nv6) * s.dn[4][6])) / (s.v[4] * s.v[4]));let eq24_e1155_d_n7: f64 = (-(((nv0 - nv6) * s.dn[4][7]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n8: f64 = (-(((nv0 - nv6) * s.dn[4][8]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n9: f64 = (-(((nv0 - nv6) * s.dn[4][9]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n10: f64 = (-(((nv0 - nv6) * s.dn[4][10]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n11: f64 = (-(((nv0 - nv6) * s.dn[4][11]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n12: f64 = (-(((nv0 - nv6) * s.dn[4][12]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n13: f64 = (-(((nv0 - nv6) * s.dn[4][13]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n14: f64 = (-(((nv0 - nv6) * s.dn[4][14]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n15: f64 = (-(((nv0 - nv6) * s.dn[4][15]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n16: f64 = (-(((nv0 - nv6) * s.dn[4][16]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n17: f64 = (-(((nv0 - nv6) * s.dn[4][17]) / (s.v[4] * s.v[4])));let eq24_e1155_d_n18: f64 = (-(((nv0 - nv6) * s.dn[4][18]) / (s.v[4] * s.v[4])));let eq24_e1155_d_b0: f64 = (-(((nv0 - nv6) * s.db[4][0]) / (s.v[4] * s.v[4])));let eq24_e1155_d_b1: f64 = (-(((nv0 - nv6) * s.db[4][1]) / (s.v[4] * s.v[4])));let eq24_e1155_d_b2: f64 = (-(((nv0 - nv6) * s.db[4][2]) / (s.v[4] * s.v[4])));let eq24_e1155_d_b3: f64 = (-(((nv0 - nv6) * s.db[4][3]) / (s.v[4] * s.v[4])));let eq24_e1155_d_b4: f64 = (-(((nv0 - nv6) * s.db[4][4]) / (s.v[4] * s.v[4])));let eq24_e1155_d_b5: f64 = (-(((nv0 - nv6) * s.db[4][5]) / (s.v[4] * s.v[4])));let eq24_e1155_d_b6: f64 = (-(((nv0 - nv6) * s.db[4][6]) / (s.v[4] * s.v[4])));let eq24_e1155_d_b7: f64 = (-(((nv0 - nv6) * s.db[4][7]) / (s.v[4] * s.v[4])));let eq24_e1155_d_b8: f64 = (-(((nv0 - nv6) * s.db[4][8]) / (s.v[4] * s.v[4])));let eq24_e1155_d_b9: f64 = (-(((nv0 - nv6) * s.db[4][9]) / (s.v[4] * s.v[4])));let eq24_e1155_d_b10: f64 = (-(((nv0 - nv6) * s.db[4][10]) / (s.v[4] * s.v[4])));let eq24_e1155_d_b11: f64 = (-(((nv0 - nv6) * s.db[4][11]) / (s.v[4] * s.v[4])));let eq24_e1155_d_b12: f64 = (-(((nv0 - nv6) * s.db[4][12]) / (s.v[4] * s.v[4])));
        (eq24_e1155, eq24_e1155_d_n0, eq24_e1155_d_n1, eq24_e1155_d_n2, eq24_e1155_d_n3, eq24_e1155_d_n4, eq24_e1155_d_n5, eq24_e1155_d_n6, eq24_e1155_d_n7, eq24_e1155_d_n8, eq24_e1155_d_n9, eq24_e1155_d_n10, eq24_e1155_d_n11, eq24_e1155_d_n12, eq24_e1155_d_n13, eq24_e1155_d_n14, eq24_e1155_d_n15, eq24_e1155_d_n16, eq24_e1155_d_n17, eq24_e1155_d_n18, eq24_e1155_d_b0, eq24_e1155_d_b1, eq24_e1155_d_b2, eq24_e1155_d_b3, eq24_e1155_d_b4, eq24_e1155_d_b5, eq24_e1155_d_b6, eq24_e1155_d_b7, eq24_e1155_d_b8, eq24_e1155_d_b9, eq24_e1155_d_b10, eq24_e1155_d_b11, eq24_e1155_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1157;let eq24_node_derivatives: [f64; 19] = [eq24_e1157_d_n0, eq24_e1157_d_n1, eq24_e1157_d_n2, eq24_e1157_d_n3, eq24_e1157_d_n4, eq24_e1157_d_n5, eq24_e1157_d_n6, eq24_e1157_d_n7, eq24_e1157_d_n8, eq24_e1157_d_n9, eq24_e1157_d_n10, eq24_e1157_d_n11, eq24_e1157_d_n12, eq24_e1157_d_n13, eq24_e1157_d_n14, eq24_e1157_d_n15, eq24_e1157_d_n16, eq24_e1157_d_n17, eq24_e1157_d_n18];let eq24_branch_derivatives: [f64; 13] = [eq24_e1157_d_b0, eq24_e1157_d_b1, eq24_e1157_d_b2, eq24_e1157_d_b3, eq24_e1157_d_b4, eq24_e1157_d_b5, eq24_e1157_d_b6, eq24_e1157_d_b7, eq24_e1157_d_b8, eq24_e1157_d_b9, eq24_e1157_d_b10, eq24_e1157_d_b11, eq24_e1157_d_b12];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(6),
            multiplicity * (eq24_value),
            &eq24_node_derivatives,
            &eq24_branch_derivatives,
            multiplicity,
        );
        let (eq25_e1162,) = {
    if (s.v[75] == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq25_value: f64 = eq25_e1162;
        stamper.stamp_potential_const_local(
            4,
            eq25_value,
        );
        let (eq26_e1168, eq26_e1168_d_n0, eq26_e1168_d_n1, eq26_e1168_d_n2, eq26_e1168_d_n3, eq26_e1168_d_n4, eq26_e1168_d_n5, eq26_e1168_d_n6, eq26_e1168_d_n7, eq26_e1168_d_n8, eq26_e1168_d_n9, eq26_e1168_d_n10, eq26_e1168_d_n11, eq26_e1168_d_n12, eq26_e1168_d_n13, eq26_e1168_d_n14, eq26_e1168_d_n15, eq26_e1168_d_n16, eq26_e1168_d_n17, eq26_e1168_d_n18, eq26_e1168_d_b0, eq26_e1168_d_b1, eq26_e1168_d_b2, eq26_e1168_d_b3, eq26_e1168_d_b4, eq26_e1168_d_b5, eq26_e1168_d_b6, eq26_e1168_d_b7, eq26_e1168_d_b8, eq26_e1168_d_b9, eq26_e1168_d_b10, eq26_e1168_d_b11, eq26_e1168_d_b12,) = {
    if (s.v[76] != 0.0) {
        let eq26_e1166: f64 = ((nv8 - nv2) / s.v[5]);let eq26_e1166_d_n0: f64 = (-(((nv8 - nv2) * s.dn[5][0]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n1: f64 = (-(((nv8 - nv2) * s.dn[5][1]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n2: f64 = (((-s.v[5]) - ((nv8 - nv2) * s.dn[5][2])) / (s.v[5] * s.v[5]));let eq26_e1166_d_n3: f64 = (-(((nv8 - nv2) * s.dn[5][3]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n4: f64 = (-(((nv8 - nv2) * s.dn[5][4]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n5: f64 = (-(((nv8 - nv2) * s.dn[5][5]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n6: f64 = (-(((nv8 - nv2) * s.dn[5][6]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n7: f64 = (-(((nv8 - nv2) * s.dn[5][7]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n8: f64 = ((s.v[5] - ((nv8 - nv2) * s.dn[5][8])) / (s.v[5] * s.v[5]));let eq26_e1166_d_n9: f64 = (-(((nv8 - nv2) * s.dn[5][9]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n10: f64 = (-(((nv8 - nv2) * s.dn[5][10]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n11: f64 = (-(((nv8 - nv2) * s.dn[5][11]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n12: f64 = (-(((nv8 - nv2) * s.dn[5][12]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n13: f64 = (-(((nv8 - nv2) * s.dn[5][13]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n14: f64 = (-(((nv8 - nv2) * s.dn[5][14]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n15: f64 = (-(((nv8 - nv2) * s.dn[5][15]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n16: f64 = (-(((nv8 - nv2) * s.dn[5][16]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n17: f64 = (-(((nv8 - nv2) * s.dn[5][17]) / (s.v[5] * s.v[5])));let eq26_e1166_d_n18: f64 = (-(((nv8 - nv2) * s.dn[5][18]) / (s.v[5] * s.v[5])));let eq26_e1166_d_b0: f64 = (-(((nv8 - nv2) * s.db[5][0]) / (s.v[5] * s.v[5])));let eq26_e1166_d_b1: f64 = (-(((nv8 - nv2) * s.db[5][1]) / (s.v[5] * s.v[5])));let eq26_e1166_d_b2: f64 = (-(((nv8 - nv2) * s.db[5][2]) / (s.v[5] * s.v[5])));let eq26_e1166_d_b3: f64 = (-(((nv8 - nv2) * s.db[5][3]) / (s.v[5] * s.v[5])));let eq26_e1166_d_b4: f64 = (-(((nv8 - nv2) * s.db[5][4]) / (s.v[5] * s.v[5])));let eq26_e1166_d_b5: f64 = (-(((nv8 - nv2) * s.db[5][5]) / (s.v[5] * s.v[5])));let eq26_e1166_d_b6: f64 = (-(((nv8 - nv2) * s.db[5][6]) / (s.v[5] * s.v[5])));let eq26_e1166_d_b7: f64 = (-(((nv8 - nv2) * s.db[5][7]) / (s.v[5] * s.v[5])));let eq26_e1166_d_b8: f64 = (-(((nv8 - nv2) * s.db[5][8]) / (s.v[5] * s.v[5])));let eq26_e1166_d_b9: f64 = (-(((nv8 - nv2) * s.db[5][9]) / (s.v[5] * s.v[5])));let eq26_e1166_d_b10: f64 = (-(((nv8 - nv2) * s.db[5][10]) / (s.v[5] * s.v[5])));let eq26_e1166_d_b11: f64 = (-(((nv8 - nv2) * s.db[5][11]) / (s.v[5] * s.v[5])));let eq26_e1166_d_b12: f64 = (-(((nv8 - nv2) * s.db[5][12]) / (s.v[5] * s.v[5])));
        (eq26_e1166, eq26_e1166_d_n0, eq26_e1166_d_n1, eq26_e1166_d_n2, eq26_e1166_d_n3, eq26_e1166_d_n4, eq26_e1166_d_n5, eq26_e1166_d_n6, eq26_e1166_d_n7, eq26_e1166_d_n8, eq26_e1166_d_n9, eq26_e1166_d_n10, eq26_e1166_d_n11, eq26_e1166_d_n12, eq26_e1166_d_n13, eq26_e1166_d_n14, eq26_e1166_d_n15, eq26_e1166_d_n16, eq26_e1166_d_n17, eq26_e1166_d_n18, eq26_e1166_d_b0, eq26_e1166_d_b1, eq26_e1166_d_b2, eq26_e1166_d_b3, eq26_e1166_d_b4, eq26_e1166_d_b5, eq26_e1166_d_b6, eq26_e1166_d_b7, eq26_e1166_d_b8, eq26_e1166_d_b9, eq26_e1166_d_b10, eq26_e1166_d_b11, eq26_e1166_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e1168;let eq26_node_derivatives: [f64; 19] = [eq26_e1168_d_n0, eq26_e1168_d_n1, eq26_e1168_d_n2, eq26_e1168_d_n3, eq26_e1168_d_n4, eq26_e1168_d_n5, eq26_e1168_d_n6, eq26_e1168_d_n7, eq26_e1168_d_n8, eq26_e1168_d_n9, eq26_e1168_d_n10, eq26_e1168_d_n11, eq26_e1168_d_n12, eq26_e1168_d_n13, eq26_e1168_d_n14, eq26_e1168_d_n15, eq26_e1168_d_n16, eq26_e1168_d_n17, eq26_e1168_d_n18];let eq26_branch_derivatives: [f64; 13] = [eq26_e1168_d_b0, eq26_e1168_d_b1, eq26_e1168_d_b2, eq26_e1168_d_b3, eq26_e1168_d_b4, eq26_e1168_d_b5, eq26_e1168_d_b6, eq26_e1168_d_b7, eq26_e1168_d_b8, eq26_e1168_d_b9, eq26_e1168_d_b10, eq26_e1168_d_b11, eq26_e1168_d_b12];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(2),
            multiplicity * (eq26_value),
            &eq26_node_derivatives,
            &eq26_branch_derivatives,
            multiplicity,
        );
        let (eq27_e1173,) = {
    if (s.v[76] == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq27_value: f64 = eq27_e1173;
        stamper.stamp_potential_const_local(
            5,
            eq27_value,
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
        let eq28_e1177: f64 = (s.v[18] + s.v[753]);let eq28_e1177_d_n0: f64 = (s.dn[18][0] + s.dn[753][0]);let eq28_e1177_d_n1: f64 = (s.dn[18][1] + s.dn[753][1]);let eq28_e1177_d_n2: f64 = (s.dn[18][2] + s.dn[753][2]);let eq28_e1177_d_n3: f64 = (s.dn[18][3] + s.dn[753][3]);let eq28_e1177_d_n4: f64 = (s.dn[18][4] + s.dn[753][4]);let eq28_e1177_d_n5: f64 = (s.dn[18][5] + s.dn[753][5]);let eq28_e1177_d_n6: f64 = (s.dn[18][6] + s.dn[753][6]);let eq28_e1177_d_n7: f64 = (s.dn[18][7] + s.dn[753][7]);let eq28_e1177_d_n8: f64 = (s.dn[18][8] + s.dn[753][8]);let eq28_e1177_d_n9: f64 = (s.dn[18][9] + s.dn[753][9]);let eq28_e1177_d_n10: f64 = (s.dn[18][10] + s.dn[753][10]);let eq28_e1177_d_n11: f64 = (s.dn[18][11] + s.dn[753][11]);let eq28_e1177_d_n12: f64 = (s.dn[18][12] + s.dn[753][12]);let eq28_e1177_d_n13: f64 = (s.dn[18][13] + s.dn[753][13]);let eq28_e1177_d_n14: f64 = (s.dn[18][14] + s.dn[753][14]);let eq28_e1177_d_n15: f64 = (s.dn[18][15] + s.dn[753][15]);let eq28_e1177_d_n16: f64 = (s.dn[18][16] + s.dn[753][16]);let eq28_e1177_d_n17: f64 = (s.dn[18][17] + s.dn[753][17]);let eq28_e1177_d_n18: f64 = (s.dn[18][18] + s.dn[753][18]);let eq28_e1177_d_b0: f64 = (s.db[18][0] + s.db[753][0]);let eq28_e1177_d_b1: f64 = (s.db[18][1] + s.db[753][1]);let eq28_e1177_d_b2: f64 = (s.db[18][2] + s.db[753][2]);let eq28_e1177_d_b3: f64 = (s.db[18][3] + s.db[753][3]);let eq28_e1177_d_b4: f64 = (s.db[18][4] + s.db[753][4]);let eq28_e1177_d_b5: f64 = (s.db[18][5] + s.db[753][5]);let eq28_e1177_d_b6: f64 = (s.db[18][6] + s.db[753][6]);let eq28_e1177_d_b7: f64 = (s.db[18][7] + s.db[753][7]);let eq28_e1177_d_b8: f64 = (s.db[18][8] + s.db[753][8]);let eq28_e1177_d_b9: f64 = (s.db[18][9] + s.db[753][9]);let eq28_e1177_d_b10: f64 = (s.db[18][10] + s.db[753][10]);let eq28_e1177_d_b11: f64 = (s.db[18][11] + s.db[753][11]);let eq28_e1177_d_b12: f64 = (s.db[18][12] + s.db[753][12]);let eq28_e1178: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq28_e1177);let eq28_e1179: f64 = (p[87] * eq28_e1178);let eq28_e1179_d_n0: f64 = (p[87] * (eq28_e1177_d_n0 * ddt_scale));let eq28_e1179_d_n1: f64 = (p[87] * (eq28_e1177_d_n1 * ddt_scale));let eq28_e1179_d_n2: f64 = (p[87] * (eq28_e1177_d_n2 * ddt_scale));let eq28_e1179_d_n3: f64 = (p[87] * (eq28_e1177_d_n3 * ddt_scale));let eq28_e1179_d_n4: f64 = (p[87] * (eq28_e1177_d_n4 * ddt_scale));let eq28_e1179_d_n5: f64 = (p[87] * (eq28_e1177_d_n5 * ddt_scale));let eq28_e1179_d_n6: f64 = (p[87] * (eq28_e1177_d_n6 * ddt_scale));let eq28_e1179_d_n7: f64 = (p[87] * (eq28_e1177_d_n7 * ddt_scale));let eq28_e1179_d_n8: f64 = (p[87] * (eq28_e1177_d_n8 * ddt_scale));let eq28_e1179_d_n9: f64 = (p[87] * (eq28_e1177_d_n9 * ddt_scale));let eq28_e1179_d_n10: f64 = (p[87] * (eq28_e1177_d_n10 * ddt_scale));let eq28_e1179_d_n11: f64 = (p[87] * (eq28_e1177_d_n11 * ddt_scale));let eq28_e1179_d_n12: f64 = (p[87] * (eq28_e1177_d_n12 * ddt_scale));let eq28_e1179_d_n13: f64 = (p[87] * (eq28_e1177_d_n13 * ddt_scale));let eq28_e1179_d_n14: f64 = (p[87] * (eq28_e1177_d_n14 * ddt_scale));let eq28_e1179_d_n15: f64 = (p[87] * (eq28_e1177_d_n15 * ddt_scale));let eq28_e1179_d_n16: f64 = (p[87] * (eq28_e1177_d_n16 * ddt_scale));let eq28_e1179_d_n17: f64 = (p[87] * (eq28_e1177_d_n17 * ddt_scale));let eq28_e1179_d_n18: f64 = (p[87] * (eq28_e1177_d_n18 * ddt_scale));let eq28_e1179_d_b0: f64 = (p[87] * (eq28_e1177_d_b0 * ddt_scale));let eq28_e1179_d_b1: f64 = (p[87] * (eq28_e1177_d_b1 * ddt_scale));let eq28_e1179_d_b2: f64 = (p[87] * (eq28_e1177_d_b2 * ddt_scale));let eq28_e1179_d_b3: f64 = (p[87] * (eq28_e1177_d_b3 * ddt_scale));let eq28_e1179_d_b4: f64 = (p[87] * (eq28_e1177_d_b4 * ddt_scale));let eq28_e1179_d_b5: f64 = (p[87] * (eq28_e1177_d_b5 * ddt_scale));let eq28_e1179_d_b6: f64 = (p[87] * (eq28_e1177_d_b6 * ddt_scale));let eq28_e1179_d_b7: f64 = (p[87] * (eq28_e1177_d_b7 * ddt_scale));let eq28_e1179_d_b8: f64 = (p[87] * (eq28_e1177_d_b8 * ddt_scale));
        let eq28_e1179_d_b9: f64 = (p[87] * (eq28_e1177_d_b9 * ddt_scale));let eq28_e1179_d_b10: f64 = (p[87] * (eq28_e1177_d_b10 * ddt_scale));let eq28_e1179_d_b11: f64 = (p[87] * (eq28_e1177_d_b11 * ddt_scale));let eq28_e1179_d_b12: f64 = (p[87] * (eq28_e1177_d_b12 * ddt_scale));let eq28_value: f64 = eq28_e1179;let eq28_node_derivatives: [f64; 19] = [eq28_e1179_d_n0, eq28_e1179_d_n1, eq28_e1179_d_n2, eq28_e1179_d_n3, eq28_e1179_d_n4, eq28_e1179_d_n5, eq28_e1179_d_n6, eq28_e1179_d_n7, eq28_e1179_d_n8, eq28_e1179_d_n9, eq28_e1179_d_n10, eq28_e1179_d_n11, eq28_e1179_d_n12, eq28_e1179_d_n13, eq28_e1179_d_n14, eq28_e1179_d_n15, eq28_e1179_d_n16, eq28_e1179_d_n17, eq28_e1179_d_n18];let eq28_branch_derivatives: [f64; 13] = [eq28_e1179_d_b0, eq28_e1179_d_b1, eq28_e1179_d_b2, eq28_e1179_d_b3, eq28_e1179_d_b4, eq28_e1179_d_b5, eq28_e1179_d_b6, eq28_e1179_d_b7, eq28_e1179_d_b8, eq28_e1179_d_b9, eq28_e1179_d_b10, eq28_e1179_d_b11, eq28_e1179_d_b12];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
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
        let eq29_e1183: f64 = (s.v[19] + s.v[751]);let eq29_e1183_d_n0: f64 = (s.dn[19][0] + s.dn[751][0]);let eq29_e1183_d_n1: f64 = (s.dn[19][1] + s.dn[751][1]);let eq29_e1183_d_n2: f64 = (s.dn[19][2] + s.dn[751][2]);let eq29_e1183_d_n3: f64 = (s.dn[19][3] + s.dn[751][3]);let eq29_e1183_d_n4: f64 = (s.dn[19][4] + s.dn[751][4]);let eq29_e1183_d_n5: f64 = (s.dn[19][5] + s.dn[751][5]);let eq29_e1183_d_n6: f64 = (s.dn[19][6] + s.dn[751][6]);let eq29_e1183_d_n7: f64 = (s.dn[19][7] + s.dn[751][7]);let eq29_e1183_d_n8: f64 = (s.dn[19][8] + s.dn[751][8]);let eq29_e1183_d_n9: f64 = (s.dn[19][9] + s.dn[751][9]);let eq29_e1183_d_n10: f64 = (s.dn[19][10] + s.dn[751][10]);let eq29_e1183_d_n11: f64 = (s.dn[19][11] + s.dn[751][11]);let eq29_e1183_d_n12: f64 = (s.dn[19][12] + s.dn[751][12]);let eq29_e1183_d_n13: f64 = (s.dn[19][13] + s.dn[751][13]);let eq29_e1183_d_n14: f64 = (s.dn[19][14] + s.dn[751][14]);let eq29_e1183_d_n15: f64 = (s.dn[19][15] + s.dn[751][15]);let eq29_e1183_d_n16: f64 = (s.dn[19][16] + s.dn[751][16]);let eq29_e1183_d_n17: f64 = (s.dn[19][17] + s.dn[751][17]);let eq29_e1183_d_n18: f64 = (s.dn[19][18] + s.dn[751][18]);let eq29_e1183_d_b0: f64 = (s.db[19][0] + s.db[751][0]);let eq29_e1183_d_b1: f64 = (s.db[19][1] + s.db[751][1]);let eq29_e1183_d_b2: f64 = (s.db[19][2] + s.db[751][2]);let eq29_e1183_d_b3: f64 = (s.db[19][3] + s.db[751][3]);let eq29_e1183_d_b4: f64 = (s.db[19][4] + s.db[751][4]);let eq29_e1183_d_b5: f64 = (s.db[19][5] + s.db[751][5]);let eq29_e1183_d_b6: f64 = (s.db[19][6] + s.db[751][6]);let eq29_e1183_d_b7: f64 = (s.db[19][7] + s.db[751][7]);let eq29_e1183_d_b8: f64 = (s.db[19][8] + s.db[751][8]);let eq29_e1183_d_b9: f64 = (s.db[19][9] + s.db[751][9]);let eq29_e1183_d_b10: f64 = (s.db[19][10] + s.db[751][10]);let eq29_e1183_d_b11: f64 = (s.db[19][11] + s.db[751][11]);let eq29_e1183_d_b12: f64 = (s.db[19][12] + s.db[751][12]);let eq29_e1184: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq29_e1183);let eq29_e1185: f64 = (p[87] * eq29_e1184);let eq29_e1185_d_n0: f64 = (p[87] * (eq29_e1183_d_n0 * ddt_scale));let eq29_e1185_d_n1: f64 = (p[87] * (eq29_e1183_d_n1 * ddt_scale));let eq29_e1185_d_n2: f64 = (p[87] * (eq29_e1183_d_n2 * ddt_scale));let eq29_e1185_d_n3: f64 = (p[87] * (eq29_e1183_d_n3 * ddt_scale));let eq29_e1185_d_n4: f64 = (p[87] * (eq29_e1183_d_n4 * ddt_scale));let eq29_e1185_d_n5: f64 = (p[87] * (eq29_e1183_d_n5 * ddt_scale));let eq29_e1185_d_n6: f64 = (p[87] * (eq29_e1183_d_n6 * ddt_scale));let eq29_e1185_d_n7: f64 = (p[87] * (eq29_e1183_d_n7 * ddt_scale));let eq29_e1185_d_n8: f64 = (p[87] * (eq29_e1183_d_n8 * ddt_scale));let eq29_e1185_d_n9: f64 = (p[87] * (eq29_e1183_d_n9 * ddt_scale));let eq29_e1185_d_n10: f64 = (p[87] * (eq29_e1183_d_n10 * ddt_scale));let eq29_e1185_d_n11: f64 = (p[87] * (eq29_e1183_d_n11 * ddt_scale));let eq29_e1185_d_n12: f64 = (p[87] * (eq29_e1183_d_n12 * ddt_scale));let eq29_e1185_d_n13: f64 = (p[87] * (eq29_e1183_d_n13 * ddt_scale));let eq29_e1185_d_n14: f64 = (p[87] * (eq29_e1183_d_n14 * ddt_scale));let eq29_e1185_d_n15: f64 = (p[87] * (eq29_e1183_d_n15 * ddt_scale));let eq29_e1185_d_n16: f64 = (p[87] * (eq29_e1183_d_n16 * ddt_scale));let eq29_e1185_d_n17: f64 = (p[87] * (eq29_e1183_d_n17 * ddt_scale));let eq29_e1185_d_n18: f64 = (p[87] * (eq29_e1183_d_n18 * ddt_scale));let eq29_e1185_d_b0: f64 = (p[87] * (eq29_e1183_d_b0 * ddt_scale));let eq29_e1185_d_b1: f64 = (p[87] * (eq29_e1183_d_b1 * ddt_scale));let eq29_e1185_d_b2: f64 = (p[87] * (eq29_e1183_d_b2 * ddt_scale));let eq29_e1185_d_b3: f64 = (p[87] * (eq29_e1183_d_b3 * ddt_scale));let eq29_e1185_d_b4: f64 = (p[87] * (eq29_e1183_d_b4 * ddt_scale));let eq29_e1185_d_b5: f64 = (p[87] * (eq29_e1183_d_b5 * ddt_scale));let eq29_e1185_d_b6: f64 = (p[87] * (eq29_e1183_d_b6 * ddt_scale));let eq29_e1185_d_b7: f64 = (p[87] * (eq29_e1183_d_b7 * ddt_scale));let eq29_e1185_d_b8: f64 = (p[87] * (eq29_e1183_d_b8 * ddt_scale));
        let eq29_e1185_d_b9: f64 = (p[87] * (eq29_e1183_d_b9 * ddt_scale));let eq29_e1185_d_b10: f64 = (p[87] * (eq29_e1183_d_b10 * ddt_scale));let eq29_e1185_d_b11: f64 = (p[87] * (eq29_e1183_d_b11 * ddt_scale));let eq29_e1185_d_b12: f64 = (p[87] * (eq29_e1183_d_b12 * ddt_scale));let eq29_value: f64 = eq29_e1185;let eq29_node_derivatives: [f64; 19] = [eq29_e1185_d_n0, eq29_e1185_d_n1, eq29_e1185_d_n2, eq29_e1185_d_n3, eq29_e1185_d_n4, eq29_e1185_d_n5, eq29_e1185_d_n6, eq29_e1185_d_n7, eq29_e1185_d_n8, eq29_e1185_d_n9, eq29_e1185_d_n10, eq29_e1185_d_n11, eq29_e1185_d_n12, eq29_e1185_d_n13, eq29_e1185_d_n14, eq29_e1185_d_n15, eq29_e1185_d_n16, eq29_e1185_d_n17, eq29_e1185_d_n18];let eq29_branch_derivatives: [f64; 13] = [eq29_e1185_d_b0, eq29_e1185_d_b1, eq29_e1185_d_b2, eq29_e1185_d_b3, eq29_e1185_d_b4, eq29_e1185_d_b5, eq29_e1185_d_b6, eq29_e1185_d_b7, eq29_e1185_d_b8, eq29_e1185_d_b9, eq29_e1185_d_b10, eq29_e1185_d_b11, eq29_e1185_d_b12];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
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
        let eq30_e1190: f64 = (s.v[753] + s.v[751]);let eq30_e1190_d_n0: f64 = (s.dn[753][0] + s.dn[751][0]);let eq30_e1190_d_n1: f64 = (s.dn[753][1] + s.dn[751][1]);let eq30_e1190_d_n2: f64 = (s.dn[753][2] + s.dn[751][2]);let eq30_e1190_d_n3: f64 = (s.dn[753][3] + s.dn[751][3]);let eq30_e1190_d_n4: f64 = (s.dn[753][4] + s.dn[751][4]);let eq30_e1190_d_n5: f64 = (s.dn[753][5] + s.dn[751][5]);let eq30_e1190_d_n6: f64 = (s.dn[753][6] + s.dn[751][6]);let eq30_e1190_d_n7: f64 = (s.dn[753][7] + s.dn[751][7]);let eq30_e1190_d_n8: f64 = (s.dn[753][8] + s.dn[751][8]);let eq30_e1190_d_n9: f64 = (s.dn[753][9] + s.dn[751][9]);let eq30_e1190_d_n10: f64 = (s.dn[753][10] + s.dn[751][10]);let eq30_e1190_d_n11: f64 = (s.dn[753][11] + s.dn[751][11]);let eq30_e1190_d_n12: f64 = (s.dn[753][12] + s.dn[751][12]);let eq30_e1190_d_n13: f64 = (s.dn[753][13] + s.dn[751][13]);let eq30_e1190_d_n14: f64 = (s.dn[753][14] + s.dn[751][14]);let eq30_e1190_d_n15: f64 = (s.dn[753][15] + s.dn[751][15]);let eq30_e1190_d_n16: f64 = (s.dn[753][16] + s.dn[751][16]);let eq30_e1190_d_n17: f64 = (s.dn[753][17] + s.dn[751][17]);let eq30_e1190_d_n18: f64 = (s.dn[753][18] + s.dn[751][18]);let eq30_e1190_d_b0: f64 = (s.db[753][0] + s.db[751][0]);let eq30_e1190_d_b1: f64 = (s.db[753][1] + s.db[751][1]);let eq30_e1190_d_b2: f64 = (s.db[753][2] + s.db[751][2]);let eq30_e1190_d_b3: f64 = (s.db[753][3] + s.db[751][3]);let eq30_e1190_d_b4: f64 = (s.db[753][4] + s.db[751][4]);let eq30_e1190_d_b5: f64 = (s.db[753][5] + s.db[751][5]);let eq30_e1190_d_b6: f64 = (s.db[753][6] + s.db[751][6]);let eq30_e1190_d_b7: f64 = (s.db[753][7] + s.db[751][7]);let eq30_e1190_d_b8: f64 = (s.db[753][8] + s.db[751][8]);let eq30_e1190_d_b9: f64 = (s.db[753][9] + s.db[751][9]);let eq30_e1190_d_b10: f64 = (s.db[753][10] + s.db[751][10]);let eq30_e1190_d_b11: f64 = (s.db[753][11] + s.db[751][11]);let eq30_e1190_d_b12: f64 = (s.db[753][12] + s.db[751][12]);let eq30_e1192: f64 = (eq30_e1190 + s.v[752]);let eq30_e1192_d_n0: f64 = (eq30_e1190_d_n0 + s.dn[752][0]);let eq30_e1192_d_n1: f64 = (eq30_e1190_d_n1 + s.dn[752][1]);let eq30_e1192_d_n2: f64 = (eq30_e1190_d_n2 + s.dn[752][2]);let eq30_e1192_d_n3: f64 = (eq30_e1190_d_n3 + s.dn[752][3]);let eq30_e1192_d_n4: f64 = (eq30_e1190_d_n4 + s.dn[752][4]);let eq30_e1192_d_n5: f64 = (eq30_e1190_d_n5 + s.dn[752][5]);let eq30_e1192_d_n6: f64 = (eq30_e1190_d_n6 + s.dn[752][6]);let eq30_e1192_d_n7: f64 = (eq30_e1190_d_n7 + s.dn[752][7]);let eq30_e1192_d_n8: f64 = (eq30_e1190_d_n8 + s.dn[752][8]);let eq30_e1192_d_n9: f64 = (eq30_e1190_d_n9 + s.dn[752][9]);let eq30_e1192_d_n10: f64 = (eq30_e1190_d_n10 + s.dn[752][10]);let eq30_e1192_d_n11: f64 = (eq30_e1190_d_n11 + s.dn[752][11]);let eq30_e1192_d_n12: f64 = (eq30_e1190_d_n12 + s.dn[752][12]);let eq30_e1192_d_n13: f64 = (eq30_e1190_d_n13 + s.dn[752][13]);let eq30_e1192_d_n14: f64 = (eq30_e1190_d_n14 + s.dn[752][14]);let eq30_e1192_d_n15: f64 = (eq30_e1190_d_n15 + s.dn[752][15]);let eq30_e1192_d_n16: f64 = (eq30_e1190_d_n16 + s.dn[752][16]);let eq30_e1192_d_n17: f64 = (eq30_e1190_d_n17 + s.dn[752][17]);let eq30_e1192_d_n18: f64 = (eq30_e1190_d_n18 + s.dn[752][18]);let eq30_e1192_d_b0: f64 = (eq30_e1190_d_b0 + s.db[752][0]);let eq30_e1192_d_b1: f64 = (eq30_e1190_d_b1 + s.db[752][1]);let eq30_e1192_d_b2: f64 = (eq30_e1190_d_b2 + s.db[752][2]);let eq30_e1192_d_b3: f64 = (eq30_e1190_d_b3 + s.db[752][3]);let eq30_e1192_d_b4: f64 = (eq30_e1190_d_b4 + s.db[752][4]);let eq30_e1192_d_b5: f64 = (eq30_e1190_d_b5 + s.db[752][5]);let eq30_e1192_d_b6: f64 = (eq30_e1190_d_b6 + s.db[752][6]);let eq30_e1192_d_b7: f64 = (eq30_e1190_d_b7 + s.db[752][7]);let eq30_e1192_d_b8: f64 = (eq30_e1190_d_b8 + s.db[752][8]);let eq30_e1192_d_b9: f64 = (eq30_e1190_d_b9 + s.db[752][9]);let eq30_e1192_d_b10: f64 = (eq30_e1190_d_b10 + s.db[752][10]);let eq30_e1192_d_b11: f64 = (eq30_e1190_d_b11 + s.db[752][11]);let eq30_e1192_d_b12: f64 = (eq30_e1190_d_b12 + s.db[752][12]);let eq30_e1193: f64 = (s.v[20] - eq30_e1192);let eq30_e1193_d_n0: f64 = (s.dn[20][0] - eq30_e1192_d_n0);let eq30_e1193_d_n1: f64 = (s.dn[20][1] - eq30_e1192_d_n1);
        let eq30_e1193_d_n2: f64 = (s.dn[20][2] - eq30_e1192_d_n2);let eq30_e1193_d_n3: f64 = (s.dn[20][3] - eq30_e1192_d_n3);let eq30_e1193_d_n4: f64 = (s.dn[20][4] - eq30_e1192_d_n4);let eq30_e1193_d_n5: f64 = (s.dn[20][5] - eq30_e1192_d_n5);let eq30_e1193_d_n6: f64 = (s.dn[20][6] - eq30_e1192_d_n6);let eq30_e1193_d_n7: f64 = (s.dn[20][7] - eq30_e1192_d_n7);let eq30_e1193_d_n8: f64 = (s.dn[20][8] - eq30_e1192_d_n8);let eq30_e1193_d_n9: f64 = (s.dn[20][9] - eq30_e1192_d_n9);let eq30_e1193_d_n10: f64 = (s.dn[20][10] - eq30_e1192_d_n10);let eq30_e1193_d_n11: f64 = (s.dn[20][11] - eq30_e1192_d_n11);let eq30_e1193_d_n12: f64 = (s.dn[20][12] - eq30_e1192_d_n12);let eq30_e1193_d_n13: f64 = (s.dn[20][13] - eq30_e1192_d_n13);let eq30_e1193_d_n14: f64 = (s.dn[20][14] - eq30_e1192_d_n14);let eq30_e1193_d_n15: f64 = (s.dn[20][15] - eq30_e1192_d_n15);let eq30_e1193_d_n16: f64 = (s.dn[20][16] - eq30_e1192_d_n16);let eq30_e1193_d_n17: f64 = (s.dn[20][17] - eq30_e1192_d_n17);let eq30_e1193_d_n18: f64 = (s.dn[20][18] - eq30_e1192_d_n18);let eq30_e1193_d_b0: f64 = (s.db[20][0] - eq30_e1192_d_b0);let eq30_e1193_d_b1: f64 = (s.db[20][1] - eq30_e1192_d_b1);let eq30_e1193_d_b2: f64 = (s.db[20][2] - eq30_e1192_d_b2);let eq30_e1193_d_b3: f64 = (s.db[20][3] - eq30_e1192_d_b3);let eq30_e1193_d_b4: f64 = (s.db[20][4] - eq30_e1192_d_b4);let eq30_e1193_d_b5: f64 = (s.db[20][5] - eq30_e1192_d_b5);let eq30_e1193_d_b6: f64 = (s.db[20][6] - eq30_e1192_d_b6);let eq30_e1193_d_b7: f64 = (s.db[20][7] - eq30_e1192_d_b7);let eq30_e1193_d_b8: f64 = (s.db[20][8] - eq30_e1192_d_b8);let eq30_e1193_d_b9: f64 = (s.db[20][9] - eq30_e1192_d_b9);let eq30_e1193_d_b10: f64 = (s.db[20][10] - eq30_e1192_d_b10);let eq30_e1193_d_b11: f64 = (s.db[20][11] - eq30_e1192_d_b11);let eq30_e1193_d_b12: f64 = (s.db[20][12] - eq30_e1192_d_b12);let eq30_e1194: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq30_e1193);let eq30_e1195: f64 = (p[87] * eq30_e1194);let eq30_e1195_d_n0: f64 = (p[87] * (eq30_e1193_d_n0 * ddt_scale));let eq30_e1195_d_n1: f64 = (p[87] * (eq30_e1193_d_n1 * ddt_scale));let eq30_e1195_d_n2: f64 = (p[87] * (eq30_e1193_d_n2 * ddt_scale));let eq30_e1195_d_n3: f64 = (p[87] * (eq30_e1193_d_n3 * ddt_scale));let eq30_e1195_d_n4: f64 = (p[87] * (eq30_e1193_d_n4 * ddt_scale));let eq30_e1195_d_n5: f64 = (p[87] * (eq30_e1193_d_n5 * ddt_scale));let eq30_e1195_d_n6: f64 = (p[87] * (eq30_e1193_d_n6 * ddt_scale));let eq30_e1195_d_n7: f64 = (p[87] * (eq30_e1193_d_n7 * ddt_scale));let eq30_e1195_d_n8: f64 = (p[87] * (eq30_e1193_d_n8 * ddt_scale));let eq30_e1195_d_n9: f64 = (p[87] * (eq30_e1193_d_n9 * ddt_scale));let eq30_e1195_d_n10: f64 = (p[87] * (eq30_e1193_d_n10 * ddt_scale));let eq30_e1195_d_n11: f64 = (p[87] * (eq30_e1193_d_n11 * ddt_scale));let eq30_e1195_d_n12: f64 = (p[87] * (eq30_e1193_d_n12 * ddt_scale));let eq30_e1195_d_n13: f64 = (p[87] * (eq30_e1193_d_n13 * ddt_scale));let eq30_e1195_d_n14: f64 = (p[87] * (eq30_e1193_d_n14 * ddt_scale));let eq30_e1195_d_n15: f64 = (p[87] * (eq30_e1193_d_n15 * ddt_scale));let eq30_e1195_d_n16: f64 = (p[87] * (eq30_e1193_d_n16 * ddt_scale));let eq30_e1195_d_n17: f64 = (p[87] * (eq30_e1193_d_n17 * ddt_scale));let eq30_e1195_d_n18: f64 = (p[87] * (eq30_e1193_d_n18 * ddt_scale));let eq30_e1195_d_b0: f64 = (p[87] * (eq30_e1193_d_b0 * ddt_scale));let eq30_e1195_d_b1: f64 = (p[87] * (eq30_e1193_d_b1 * ddt_scale));let eq30_e1195_d_b2: f64 = (p[87] * (eq30_e1193_d_b2 * ddt_scale));let eq30_e1195_d_b3: f64 = (p[87] * (eq30_e1193_d_b3 * ddt_scale));let eq30_e1195_d_b4: f64 = (p[87] * (eq30_e1193_d_b4 * ddt_scale));let eq30_e1195_d_b5: f64 = (p[87] * (eq30_e1193_d_b5 * ddt_scale));let eq30_e1195_d_b6: f64 = (p[87] * (eq30_e1193_d_b6 * ddt_scale));let eq30_e1195_d_b7: f64 = (p[87] * (eq30_e1193_d_b7 * ddt_scale));let eq30_e1195_d_b8: f64 = (p[87] * (eq30_e1193_d_b8 * ddt_scale));let eq30_e1195_d_b9: f64 = (p[87] * (eq30_e1193_d_b9 * ddt_scale));
        let eq30_e1195_d_b10: f64 = (p[87] * (eq30_e1193_d_b10 * ddt_scale));let eq30_e1195_d_b11: f64 = (p[87] * (eq30_e1193_d_b11 * ddt_scale));let eq30_e1195_d_b12: f64 = (p[87] * (eq30_e1193_d_b12 * ddt_scale));let eq30_value: f64 = eq30_e1195;let eq30_node_derivatives: [f64; 19] = [eq30_e1195_d_n0, eq30_e1195_d_n1, eq30_e1195_d_n2, eq30_e1195_d_n3, eq30_e1195_d_n4, eq30_e1195_d_n5, eq30_e1195_d_n6, eq30_e1195_d_n7, eq30_e1195_d_n8, eq30_e1195_d_n9, eq30_e1195_d_n10, eq30_e1195_d_n11, eq30_e1195_d_n12, eq30_e1195_d_n13, eq30_e1195_d_n14, eq30_e1195_d_n15, eq30_e1195_d_n16, eq30_e1195_d_n17, eq30_e1195_d_n18];let eq30_branch_derivatives: [f64; 13] = [eq30_e1195_d_b0, eq30_e1195_d_b1, eq30_e1195_d_b2, eq30_e1195_d_b3, eq30_e1195_d_b4, eq30_e1195_d_b5, eq30_e1195_d_b6, eq30_e1195_d_b7, eq30_e1195_d_b8, eq30_e1195_d_b9, eq30_e1195_d_b10, eq30_e1195_d_b11, eq30_e1195_d_b12];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq30_value),
            &eq30_node_derivatives,
            &eq30_branch_derivatives,
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
        let eq31_e1198: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, s.v[743]);let eq31_e1199: f64 = (p[87] * eq31_e1198);let eq31_e1199_d_n0: f64 = (p[87] * (s.dn[743][0] * ddt_scale));let eq31_e1199_d_n1: f64 = (p[87] * (s.dn[743][1] * ddt_scale));let eq31_e1199_d_n2: f64 = (p[87] * (s.dn[743][2] * ddt_scale));let eq31_e1199_d_n3: f64 = (p[87] * (s.dn[743][3] * ddt_scale));let eq31_e1199_d_n4: f64 = (p[87] * (s.dn[743][4] * ddt_scale));let eq31_e1199_d_n5: f64 = (p[87] * (s.dn[743][5] * ddt_scale));let eq31_e1199_d_n6: f64 = (p[87] * (s.dn[743][6] * ddt_scale));let eq31_e1199_d_n7: f64 = (p[87] * (s.dn[743][7] * ddt_scale));let eq31_e1199_d_n8: f64 = (p[87] * (s.dn[743][8] * ddt_scale));let eq31_e1199_d_n9: f64 = (p[87] * (s.dn[743][9] * ddt_scale));let eq31_e1199_d_n10: f64 = (p[87] * (s.dn[743][10] * ddt_scale));let eq31_e1199_d_n11: f64 = (p[87] * (s.dn[743][11] * ddt_scale));let eq31_e1199_d_n12: f64 = (p[87] * (s.dn[743][12] * ddt_scale));let eq31_e1199_d_n13: f64 = (p[87] * (s.dn[743][13] * ddt_scale));let eq31_e1199_d_n14: f64 = (p[87] * (s.dn[743][14] * ddt_scale));let eq31_e1199_d_n15: f64 = (p[87] * (s.dn[743][15] * ddt_scale));let eq31_e1199_d_n16: f64 = (p[87] * (s.dn[743][16] * ddt_scale));let eq31_e1199_d_n17: f64 = (p[87] * (s.dn[743][17] * ddt_scale));let eq31_e1199_d_n18: f64 = (p[87] * (s.dn[743][18] * ddt_scale));let eq31_e1199_d_b0: f64 = (p[87] * (s.db[743][0] * ddt_scale));let eq31_e1199_d_b1: f64 = (p[87] * (s.db[743][1] * ddt_scale));let eq31_e1199_d_b2: f64 = (p[87] * (s.db[743][2] * ddt_scale));let eq31_e1199_d_b3: f64 = (p[87] * (s.db[743][3] * ddt_scale));let eq31_e1199_d_b4: f64 = (p[87] * (s.db[743][4] * ddt_scale));let eq31_e1199_d_b5: f64 = (p[87] * (s.db[743][5] * ddt_scale));let eq31_e1199_d_b6: f64 = (p[87] * (s.db[743][6] * ddt_scale));let eq31_e1199_d_b7: f64 = (p[87] * (s.db[743][7] * ddt_scale));let eq31_e1199_d_b8: f64 = (p[87] * (s.db[743][8] * ddt_scale));let eq31_e1199_d_b9: f64 = (p[87] * (s.db[743][9] * ddt_scale));let eq31_e1199_d_b10: f64 = (p[87] * (s.db[743][10] * ddt_scale));let eq31_e1199_d_b11: f64 = (p[87] * (s.db[743][11] * ddt_scale));let eq31_e1199_d_b12: f64 = (p[87] * (s.db[743][12] * ddt_scale));let eq31_value: f64 = eq31_e1199;let eq31_node_derivatives: [f64; 19] = [eq31_e1199_d_n0, eq31_e1199_d_n1, eq31_e1199_d_n2, eq31_e1199_d_n3, eq31_e1199_d_n4, eq31_e1199_d_n5, eq31_e1199_d_n6, eq31_e1199_d_n7, eq31_e1199_d_n8, eq31_e1199_d_n9, eq31_e1199_d_n10, eq31_e1199_d_n11, eq31_e1199_d_n12, eq31_e1199_d_n13, eq31_e1199_d_n14, eq31_e1199_d_n15, eq31_e1199_d_n16, eq31_e1199_d_n17, eq31_e1199_d_n18];let eq31_branch_derivatives: [f64; 13] = [eq31_e1199_d_b0, eq31_e1199_d_b1, eq31_e1199_d_b2, eq31_e1199_d_b3, eq31_e1199_d_b4, eq31_e1199_d_b5, eq31_e1199_d_b6, eq31_e1199_d_b7, eq31_e1199_d_b8, eq31_e1199_d_b9, eq31_e1199_d_b10, eq31_e1199_d_b11, eq31_e1199_d_b12];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(2),
            multiplicity * (eq31_value),
            &eq31_node_derivatives,
            &eq31_branch_derivatives,
            multiplicity,
        );let eq32_e1202: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, s.v[742]);let eq32_e1203: f64 = (p[87] * eq32_e1202);let eq32_e1203_d_n0: f64 = (p[87] * (s.dn[742][0] * ddt_scale));let eq32_e1203_d_n1: f64 = (p[87] * (s.dn[742][1] * ddt_scale));let eq32_e1203_d_n2: f64 = (p[87] * (s.dn[742][2] * ddt_scale));let eq32_e1203_d_n3: f64 = (p[87] * (s.dn[742][3] * ddt_scale));let eq32_e1203_d_n4: f64 = (p[87] * (s.dn[742][4] * ddt_scale));let eq32_e1203_d_n5: f64 = (p[87] * (s.dn[742][5] * ddt_scale));let eq32_e1203_d_n6: f64 = (p[87] * (s.dn[742][6] * ddt_scale));let eq32_e1203_d_n7: f64 = (p[87] * (s.dn[742][7] * ddt_scale));let eq32_e1203_d_n8: f64 = (p[87] * (s.dn[742][8] * ddt_scale));let eq32_e1203_d_n9: f64 = (p[87] * (s.dn[742][9] * ddt_scale));let eq32_e1203_d_n10: f64 = (p[87] * (s.dn[742][10] * ddt_scale));let eq32_e1203_d_n11: f64 = (p[87] * (s.dn[742][11] * ddt_scale));let eq32_e1203_d_n12: f64 = (p[87] * (s.dn[742][12] * ddt_scale));let eq32_e1203_d_n13: f64 = (p[87] * (s.dn[742][13] * ddt_scale));let eq32_e1203_d_n14: f64 = (p[87] * (s.dn[742][14] * ddt_scale));let eq32_e1203_d_n15: f64 = (p[87] * (s.dn[742][15] * ddt_scale));let eq32_e1203_d_n16: f64 = (p[87] * (s.dn[742][16] * ddt_scale));let eq32_e1203_d_n17: f64 = (p[87] * (s.dn[742][17] * ddt_scale));let eq32_e1203_d_n18: f64 = (p[87] * (s.dn[742][18] * ddt_scale));let eq32_e1203_d_b0: f64 = (p[87] * (s.db[742][0] * ddt_scale));let eq32_e1203_d_b1: f64 = (p[87] * (s.db[742][1] * ddt_scale));let eq32_e1203_d_b2: f64 = (p[87] * (s.db[742][2] * ddt_scale));let eq32_e1203_d_b3: f64 = (p[87] * (s.db[742][3] * ddt_scale));let eq32_e1203_d_b4: f64 = (p[87] * (s.db[742][4] * ddt_scale));let eq32_e1203_d_b5: f64 = (p[87] * (s.db[742][5] * ddt_scale));let eq32_e1203_d_b6: f64 = (p[87] * (s.db[742][6] * ddt_scale));let eq32_e1203_d_b7: f64 = (p[87] * (s.db[742][7] * ddt_scale));let eq32_e1203_d_b8: f64 = (p[87] * (s.db[742][8] * ddt_scale));let eq32_e1203_d_b9: f64 = (p[87] * (s.db[742][9] * ddt_scale));let eq32_e1203_d_b10: f64 = (p[87] * (s.db[742][10] * ddt_scale));let eq32_e1203_d_b11: f64 = (p[87] * (s.db[742][11] * ddt_scale));let eq32_e1203_d_b12: f64 = (p[87] * (s.db[742][12] * ddt_scale));let eq32_value: f64 = eq32_e1203;let eq32_node_derivatives: [f64; 19] = [eq32_e1203_d_n0, eq32_e1203_d_n1, eq32_e1203_d_n2, eq32_e1203_d_n3, eq32_e1203_d_n4, eq32_e1203_d_n5, eq32_e1203_d_n6, eq32_e1203_d_n7, eq32_e1203_d_n8, eq32_e1203_d_n9, eq32_e1203_d_n10, eq32_e1203_d_n11, eq32_e1203_d_n12, eq32_e1203_d_n13, eq32_e1203_d_n14, eq32_e1203_d_n15, eq32_e1203_d_n16, eq32_e1203_d_n17, eq32_e1203_d_n18];let eq32_branch_derivatives: [f64; 13] = [eq32_e1203_d_b0, eq32_e1203_d_b1, eq32_e1203_d_b2, eq32_e1203_d_b3, eq32_e1203_d_b4, eq32_e1203_d_b5, eq32_e1203_d_b6, eq32_e1203_d_b7, eq32_e1203_d_b8, eq32_e1203_d_b9, eq32_e1203_d_b10, eq32_e1203_d_b11, eq32_e1203_d_b12];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_14(
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
        let eq33_e1206: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, s.v[744]);let eq33_e1207: f64 = (p[87] * eq33_e1206);let eq33_e1207_d_n0: f64 = (p[87] * (s.dn[744][0] * ddt_scale));let eq33_e1207_d_n1: f64 = (p[87] * (s.dn[744][1] * ddt_scale));let eq33_e1207_d_n2: f64 = (p[87] * (s.dn[744][2] * ddt_scale));let eq33_e1207_d_n3: f64 = (p[87] * (s.dn[744][3] * ddt_scale));let eq33_e1207_d_n4: f64 = (p[87] * (s.dn[744][4] * ddt_scale));let eq33_e1207_d_n5: f64 = (p[87] * (s.dn[744][5] * ddt_scale));let eq33_e1207_d_n6: f64 = (p[87] * (s.dn[744][6] * ddt_scale));let eq33_e1207_d_n7: f64 = (p[87] * (s.dn[744][7] * ddt_scale));let eq33_e1207_d_n8: f64 = (p[87] * (s.dn[744][8] * ddt_scale));let eq33_e1207_d_n9: f64 = (p[87] * (s.dn[744][9] * ddt_scale));let eq33_e1207_d_n10: f64 = (p[87] * (s.dn[744][10] * ddt_scale));let eq33_e1207_d_n11: f64 = (p[87] * (s.dn[744][11] * ddt_scale));let eq33_e1207_d_n12: f64 = (p[87] * (s.dn[744][12] * ddt_scale));let eq33_e1207_d_n13: f64 = (p[87] * (s.dn[744][13] * ddt_scale));let eq33_e1207_d_n14: f64 = (p[87] * (s.dn[744][14] * ddt_scale));let eq33_e1207_d_n15: f64 = (p[87] * (s.dn[744][15] * ddt_scale));let eq33_e1207_d_n16: f64 = (p[87] * (s.dn[744][16] * ddt_scale));let eq33_e1207_d_n17: f64 = (p[87] * (s.dn[744][17] * ddt_scale));let eq33_e1207_d_n18: f64 = (p[87] * (s.dn[744][18] * ddt_scale));let eq33_e1207_d_b0: f64 = (p[87] * (s.db[744][0] * ddt_scale));let eq33_e1207_d_b1: f64 = (p[87] * (s.db[744][1] * ddt_scale));let eq33_e1207_d_b2: f64 = (p[87] * (s.db[744][2] * ddt_scale));let eq33_e1207_d_b3: f64 = (p[87] * (s.db[744][3] * ddt_scale));let eq33_e1207_d_b4: f64 = (p[87] * (s.db[744][4] * ddt_scale));let eq33_e1207_d_b5: f64 = (p[87] * (s.db[744][5] * ddt_scale));let eq33_e1207_d_b6: f64 = (p[87] * (s.db[744][6] * ddt_scale));let eq33_e1207_d_b7: f64 = (p[87] * (s.db[744][7] * ddt_scale));let eq33_e1207_d_b8: f64 = (p[87] * (s.db[744][8] * ddt_scale));let eq33_e1207_d_b9: f64 = (p[87] * (s.db[744][9] * ddt_scale));let eq33_e1207_d_b10: f64 = (p[87] * (s.db[744][10] * ddt_scale));let eq33_e1207_d_b11: f64 = (p[87] * (s.db[744][11] * ddt_scale));let eq33_e1207_d_b12: f64 = (p[87] * (s.db[744][12] * ddt_scale));let eq33_value: f64 = eq33_e1207;let eq33_node_derivatives: [f64; 19] = [eq33_e1207_d_n0, eq33_e1207_d_n1, eq33_e1207_d_n2, eq33_e1207_d_n3, eq33_e1207_d_n4, eq33_e1207_d_n5, eq33_e1207_d_n6, eq33_e1207_d_n7, eq33_e1207_d_n8, eq33_e1207_d_n9, eq33_e1207_d_n10, eq33_e1207_d_n11, eq33_e1207_d_n12, eq33_e1207_d_n13, eq33_e1207_d_n14, eq33_e1207_d_n15, eq33_e1207_d_n16, eq33_e1207_d_n17, eq33_e1207_d_n18];let eq33_branch_derivatives: [f64; 13] = [eq33_e1207_d_b0, eq33_e1207_d_b1, eq33_e1207_d_b2, eq33_e1207_d_b3, eq33_e1207_d_b4, eq33_e1207_d_b5, eq33_e1207_d_b6, eq33_e1207_d_b7, eq33_e1207_d_b8, eq33_e1207_d_b9, eq33_e1207_d_b10, eq33_e1207_d_b11, eq33_e1207_d_b12];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(2),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );let eq34_e1209: f64 = (-p[87]);let eq34_e1211: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, s.v[299]);let eq34_e1212: f64 = (eq34_e1209 * eq34_e1211);let eq34_e1212_d_n0: f64 = (eq34_e1209 * (s.dn[299][0] * ddt_scale));let eq34_e1212_d_n1: f64 = (eq34_e1209 * (s.dn[299][1] * ddt_scale));let eq34_e1212_d_n2: f64 = (eq34_e1209 * (s.dn[299][2] * ddt_scale));let eq34_e1212_d_n3: f64 = (eq34_e1209 * (s.dn[299][3] * ddt_scale));let eq34_e1212_d_n4: f64 = (eq34_e1209 * (s.dn[299][4] * ddt_scale));let eq34_e1212_d_n5: f64 = (eq34_e1209 * (s.dn[299][5] * ddt_scale));let eq34_e1212_d_n6: f64 = (eq34_e1209 * (s.dn[299][6] * ddt_scale));let eq34_e1212_d_n7: f64 = (eq34_e1209 * (s.dn[299][7] * ddt_scale));let eq34_e1212_d_n8: f64 = (eq34_e1209 * (s.dn[299][8] * ddt_scale));let eq34_e1212_d_n9: f64 = (eq34_e1209 * (s.dn[299][9] * ddt_scale));let eq34_e1212_d_n10: f64 = (eq34_e1209 * (s.dn[299][10] * ddt_scale));let eq34_e1212_d_n11: f64 = (eq34_e1209 * (s.dn[299][11] * ddt_scale));let eq34_e1212_d_n12: f64 = (eq34_e1209 * (s.dn[299][12] * ddt_scale));let eq34_e1212_d_n13: f64 = (eq34_e1209 * (s.dn[299][13] * ddt_scale));let eq34_e1212_d_n14: f64 = (eq34_e1209 * (s.dn[299][14] * ddt_scale));let eq34_e1212_d_n15: f64 = (eq34_e1209 * (s.dn[299][15] * ddt_scale));let eq34_e1212_d_n16: f64 = (eq34_e1209 * (s.dn[299][16] * ddt_scale));let eq34_e1212_d_n17: f64 = (eq34_e1209 * (s.dn[299][17] * ddt_scale));let eq34_e1212_d_n18: f64 = (eq34_e1209 * (s.dn[299][18] * ddt_scale));let eq34_e1212_d_b0: f64 = (eq34_e1209 * (s.db[299][0] * ddt_scale));let eq34_e1212_d_b1: f64 = (eq34_e1209 * (s.db[299][1] * ddt_scale));let eq34_e1212_d_b2: f64 = (eq34_e1209 * (s.db[299][2] * ddt_scale));let eq34_e1212_d_b3: f64 = (eq34_e1209 * (s.db[299][3] * ddt_scale));let eq34_e1212_d_b4: f64 = (eq34_e1209 * (s.db[299][4] * ddt_scale));let eq34_e1212_d_b5: f64 = (eq34_e1209 * (s.db[299][5] * ddt_scale));let eq34_e1212_d_b6: f64 = (eq34_e1209 * (s.db[299][6] * ddt_scale));let eq34_e1212_d_b7: f64 = (eq34_e1209 * (s.db[299][7] * ddt_scale));let eq34_e1212_d_b8: f64 = (eq34_e1209 * (s.db[299][8] * ddt_scale));let eq34_e1212_d_b9: f64 = (eq34_e1209 * (s.db[299][9] * ddt_scale));let eq34_e1212_d_b10: f64 = (eq34_e1209 * (s.db[299][10] * ddt_scale));let eq34_e1212_d_b11: f64 = (eq34_e1209 * (s.db[299][11] * ddt_scale));let eq34_e1212_d_b12: f64 = (eq34_e1209 * (s.db[299][12] * ddt_scale));let eq34_value: f64 = eq34_e1212;let eq34_node_derivatives: [f64; 19] = [eq34_e1212_d_n0, eq34_e1212_d_n1, eq34_e1212_d_n2, eq34_e1212_d_n3, eq34_e1212_d_n4, eq34_e1212_d_n5, eq34_e1212_d_n6, eq34_e1212_d_n7, eq34_e1212_d_n8, eq34_e1212_d_n9, eq34_e1212_d_n10, eq34_e1212_d_n11, eq34_e1212_d_n12, eq34_e1212_d_n13, eq34_e1212_d_n14, eq34_e1212_d_n15, eq34_e1212_d_n16, eq34_e1212_d_n17, eq34_e1212_d_n18];let eq34_branch_derivatives: [f64; 13] = [eq34_e1212_d_b0, eq34_e1212_d_b1, eq34_e1212_d_b2, eq34_e1212_d_b3, eq34_e1212_d_b4, eq34_e1212_d_b5, eq34_e1212_d_b6, eq34_e1212_d_b7, eq34_e1212_d_b8, eq34_e1212_d_b9, eq34_e1212_d_b10, eq34_e1212_d_b11, eq34_e1212_d_b12];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(0),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
            multiplicity,
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
        let nv15 = ctx.node_voltage(nodes[15]);let eq35_e1214: f64 = (-p[87]);let eq35_e1216: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, s.v[301]);let eq35_e1217: f64 = (eq35_e1214 * eq35_e1216);let eq35_e1217_d_n0: f64 = (eq35_e1214 * (s.dn[301][0] * ddt_scale));let eq35_e1217_d_n1: f64 = (eq35_e1214 * (s.dn[301][1] * ddt_scale));let eq35_e1217_d_n2: f64 = (eq35_e1214 * (s.dn[301][2] * ddt_scale));let eq35_e1217_d_n3: f64 = (eq35_e1214 * (s.dn[301][3] * ddt_scale));let eq35_e1217_d_n4: f64 = (eq35_e1214 * (s.dn[301][4] * ddt_scale));let eq35_e1217_d_n5: f64 = (eq35_e1214 * (s.dn[301][5] * ddt_scale));let eq35_e1217_d_n6: f64 = (eq35_e1214 * (s.dn[301][6] * ddt_scale));let eq35_e1217_d_n7: f64 = (eq35_e1214 * (s.dn[301][7] * ddt_scale));let eq35_e1217_d_n8: f64 = (eq35_e1214 * (s.dn[301][8] * ddt_scale));let eq35_e1217_d_n9: f64 = (eq35_e1214 * (s.dn[301][9] * ddt_scale));let eq35_e1217_d_n10: f64 = (eq35_e1214 * (s.dn[301][10] * ddt_scale));let eq35_e1217_d_n11: f64 = (eq35_e1214 * (s.dn[301][11] * ddt_scale));let eq35_e1217_d_n12: f64 = (eq35_e1214 * (s.dn[301][12] * ddt_scale));let eq35_e1217_d_n13: f64 = (eq35_e1214 * (s.dn[301][13] * ddt_scale));let eq35_e1217_d_n14: f64 = (eq35_e1214 * (s.dn[301][14] * ddt_scale));let eq35_e1217_d_n15: f64 = (eq35_e1214 * (s.dn[301][15] * ddt_scale));let eq35_e1217_d_n16: f64 = (eq35_e1214 * (s.dn[301][16] * ddt_scale));let eq35_e1217_d_n17: f64 = (eq35_e1214 * (s.dn[301][17] * ddt_scale));let eq35_e1217_d_n18: f64 = (eq35_e1214 * (s.dn[301][18] * ddt_scale));let eq35_e1217_d_b0: f64 = (eq35_e1214 * (s.db[301][0] * ddt_scale));let eq35_e1217_d_b1: f64 = (eq35_e1214 * (s.db[301][1] * ddt_scale));let eq35_e1217_d_b2: f64 = (eq35_e1214 * (s.db[301][2] * ddt_scale));let eq35_e1217_d_b3: f64 = (eq35_e1214 * (s.db[301][3] * ddt_scale));let eq35_e1217_d_b4: f64 = (eq35_e1214 * (s.db[301][4] * ddt_scale));let eq35_e1217_d_b5: f64 = (eq35_e1214 * (s.db[301][5] * ddt_scale));let eq35_e1217_d_b6: f64 = (eq35_e1214 * (s.db[301][6] * ddt_scale));let eq35_e1217_d_b7: f64 = (eq35_e1214 * (s.db[301][7] * ddt_scale));let eq35_e1217_d_b8: f64 = (eq35_e1214 * (s.db[301][8] * ddt_scale));let eq35_e1217_d_b9: f64 = (eq35_e1214 * (s.db[301][9] * ddt_scale));let eq35_e1217_d_b10: f64 = (eq35_e1214 * (s.db[301][10] * ddt_scale));let eq35_e1217_d_b11: f64 = (eq35_e1214 * (s.db[301][11] * ddt_scale));let eq35_e1217_d_b12: f64 = (eq35_e1214 * (s.db[301][12] * ddt_scale));let eq35_value: f64 = eq35_e1217;let eq35_node_derivatives: [f64; 19] = [eq35_e1217_d_n0, eq35_e1217_d_n1, eq35_e1217_d_n2, eq35_e1217_d_n3, eq35_e1217_d_n4, eq35_e1217_d_n5, eq35_e1217_d_n6, eq35_e1217_d_n7, eq35_e1217_d_n8, eq35_e1217_d_n9, eq35_e1217_d_n10, eq35_e1217_d_n11, eq35_e1217_d_n12, eq35_e1217_d_n13, eq35_e1217_d_n14, eq35_e1217_d_n15, eq35_e1217_d_n16, eq35_e1217_d_n17, eq35_e1217_d_n18];let eq35_branch_derivatives: [f64; 13] = [eq35_e1217_d_b0, eq35_e1217_d_b1, eq35_e1217_d_b2, eq35_e1217_d_b3, eq35_e1217_d_b4, eq35_e1217_d_b5, eq35_e1217_d_b6, eq35_e1217_d_b7, eq35_e1217_d_b8, eq35_e1217_d_b9, eq35_e1217_d_b10, eq35_e1217_d_b11, eq35_e1217_d_b12];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(2),
            multiplicity * (eq35_value),
            &eq35_node_derivatives,
            &eq35_branch_derivatives,
            multiplicity,
        );let eq37_e1228: f64 = (nv15 - 0.0);let eq37_value: f64 = eq37_e1228;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq37_value),
            15,
            multiplicity * (1.0),
        );let eq40_e1243: f64 = (s.v[951] * (nv15 - 0.0));let eq40_e1243_d_n0: f64 = (s.dn[951][0] * (nv15 - 0.0));let eq40_e1243_d_n1: f64 = (s.dn[951][1] * (nv15 - 0.0));let eq40_e1243_d_n2: f64 = (s.dn[951][2] * (nv15 - 0.0));let eq40_e1243_d_n3: f64 = (s.dn[951][3] * (nv15 - 0.0));let eq40_e1243_d_n4: f64 = (s.dn[951][4] * (nv15 - 0.0));let eq40_e1243_d_n5: f64 = (s.dn[951][5] * (nv15 - 0.0));let eq40_e1243_d_n6: f64 = (s.dn[951][6] * (nv15 - 0.0));let eq40_e1243_d_n7: f64 = (s.dn[951][7] * (nv15 - 0.0));let eq40_e1243_d_n8: f64 = (s.dn[951][8] * (nv15 - 0.0));let eq40_e1243_d_n9: f64 = (s.dn[951][9] * (nv15 - 0.0));let eq40_e1243_d_n10: f64 = (s.dn[951][10] * (nv15 - 0.0));let eq40_e1243_d_n11: f64 = (s.dn[951][11] * (nv15 - 0.0));let eq40_e1243_d_n12: f64 = (s.dn[951][12] * (nv15 - 0.0));let eq40_e1243_d_n13: f64 = (s.dn[951][13] * (nv15 - 0.0));let eq40_e1243_d_n14: f64 = (s.dn[951][14] * (nv15 - 0.0));let eq40_e1243_d_n15: f64 = ((s.dn[951][15] * (nv15 - 0.0)) + s.v[951]);let eq40_e1243_d_n16: f64 = (s.dn[951][16] * (nv15 - 0.0));let eq40_e1243_d_n17: f64 = (s.dn[951][17] * (nv15 - 0.0));let eq40_e1243_d_n18: f64 = (s.dn[951][18] * (nv15 - 0.0));let eq40_e1243_d_b0: f64 = (s.db[951][0] * (nv15 - 0.0));let eq40_e1243_d_b1: f64 = (s.db[951][1] * (nv15 - 0.0));let eq40_e1243_d_b2: f64 = (s.db[951][2] * (nv15 - 0.0));let eq40_e1243_d_b3: f64 = (s.db[951][3] * (nv15 - 0.0));let eq40_e1243_d_b4: f64 = (s.db[951][4] * (nv15 - 0.0));let eq40_e1243_d_b5: f64 = (s.db[951][5] * (nv15 - 0.0));let eq40_e1243_d_b6: f64 = (s.db[951][6] * (nv15 - 0.0));let eq40_e1243_d_b7: f64 = (s.db[951][7] * (nv15 - 0.0));let eq40_e1243_d_b8: f64 = (s.db[951][8] * (nv15 - 0.0));let eq40_e1243_d_b9: f64 = (s.db[951][9] * (nv15 - 0.0));let eq40_e1243_d_b10: f64 = (s.db[951][10] * (nv15 - 0.0));let eq40_e1243_d_b11: f64 = (s.db[951][11] * (nv15 - 0.0));let eq40_e1243_d_b12: f64 = (s.db[951][12] * (nv15 - 0.0));let eq40_value: f64 = eq40_e1243;let eq40_node_derivatives: [f64; 19] = [eq40_e1243_d_n0, eq40_e1243_d_n1, eq40_e1243_d_n2, eq40_e1243_d_n3, eq40_e1243_d_n4, eq40_e1243_d_n5, eq40_e1243_d_n6, eq40_e1243_d_n7, eq40_e1243_d_n8, eq40_e1243_d_n9, eq40_e1243_d_n10, eq40_e1243_d_n11, eq40_e1243_d_n12, eq40_e1243_d_n13, eq40_e1243_d_n14, eq40_e1243_d_n15, eq40_e1243_d_n16, eq40_e1243_d_n17, eq40_e1243_d_n18];let eq40_branch_derivatives: [f64; 13] = [eq40_e1243_d_b0, eq40_e1243_d_b1, eq40_e1243_d_b2, eq40_e1243_d_b3, eq40_e1243_d_b4, eq40_e1243_d_b5, eq40_e1243_d_b6, eq40_e1243_d_b7, eq40_e1243_d_b8, eq40_e1243_d_b9, eq40_e1243_d_b10, eq40_e1243_d_b11, eq40_e1243_d_b12];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq40_value),
            &eq40_node_derivatives,
            &eq40_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_16(
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
        let nv15 = ctx.node_voltage(nodes[15]);let eq41_e1246: f64 = ((nv15 - 0.0) * s.v[954]);let eq41_e1246_d_n0: f64 = ((nv15 - 0.0) * s.dn[954][0]);let eq41_e1246_d_n1: f64 = ((nv15 - 0.0) * s.dn[954][1]);let eq41_e1246_d_n2: f64 = ((nv15 - 0.0) * s.dn[954][2]);let eq41_e1246_d_n3: f64 = ((nv15 - 0.0) * s.dn[954][3]);let eq41_e1246_d_n4: f64 = ((nv15 - 0.0) * s.dn[954][4]);let eq41_e1246_d_n5: f64 = ((nv15 - 0.0) * s.dn[954][5]);let eq41_e1246_d_n6: f64 = ((nv15 - 0.0) * s.dn[954][6]);let eq41_e1246_d_n7: f64 = ((nv15 - 0.0) * s.dn[954][7]);let eq41_e1246_d_n8: f64 = ((nv15 - 0.0) * s.dn[954][8]);let eq41_e1246_d_n9: f64 = ((nv15 - 0.0) * s.dn[954][9]);let eq41_e1246_d_n10: f64 = ((nv15 - 0.0) * s.dn[954][10]);let eq41_e1246_d_n11: f64 = ((nv15 - 0.0) * s.dn[954][11]);let eq41_e1246_d_n12: f64 = ((nv15 - 0.0) * s.dn[954][12]);let eq41_e1246_d_n13: f64 = ((nv15 - 0.0) * s.dn[954][13]);let eq41_e1246_d_n14: f64 = ((nv15 - 0.0) * s.dn[954][14]);let eq41_e1246_d_n15: f64 = (s.v[954] + ((nv15 - 0.0) * s.dn[954][15]));let eq41_e1246_d_n16: f64 = ((nv15 - 0.0) * s.dn[954][16]);let eq41_e1246_d_n17: f64 = ((nv15 - 0.0) * s.dn[954][17]);let eq41_e1246_d_n18: f64 = ((nv15 - 0.0) * s.dn[954][18]);let eq41_e1246_d_b0: f64 = ((nv15 - 0.0) * s.db[954][0]);let eq41_e1246_d_b1: f64 = ((nv15 - 0.0) * s.db[954][1]);let eq41_e1246_d_b2: f64 = ((nv15 - 0.0) * s.db[954][2]);let eq41_e1246_d_b3: f64 = ((nv15 - 0.0) * s.db[954][3]);let eq41_e1246_d_b4: f64 = ((nv15 - 0.0) * s.db[954][4]);let eq41_e1246_d_b5: f64 = ((nv15 - 0.0) * s.db[954][5]);let eq41_e1246_d_b6: f64 = ((nv15 - 0.0) * s.db[954][6]);let eq41_e1246_d_b7: f64 = ((nv15 - 0.0) * s.db[954][7]);let eq41_e1246_d_b8: f64 = ((nv15 - 0.0) * s.db[954][8]);let eq41_e1246_d_b9: f64 = ((nv15 - 0.0) * s.db[954][9]);let eq41_e1246_d_b10: f64 = ((nv15 - 0.0) * s.db[954][10]);let eq41_e1246_d_b11: f64 = ((nv15 - 0.0) * s.db[954][11]);let eq41_e1246_d_b12: f64 = ((nv15 - 0.0) * s.db[954][12]);let eq41_e1247: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, eq41_e1246);let eq41_value: f64 = eq41_e1247;let eq41_node_derivatives: [f64; 19] = [(eq41_e1246_d_n0 * ddt_scale), (eq41_e1246_d_n1 * ddt_scale), (eq41_e1246_d_n2 * ddt_scale), (eq41_e1246_d_n3 * ddt_scale), (eq41_e1246_d_n4 * ddt_scale), (eq41_e1246_d_n5 * ddt_scale), (eq41_e1246_d_n6 * ddt_scale), (eq41_e1246_d_n7 * ddt_scale), (eq41_e1246_d_n8 * ddt_scale), (eq41_e1246_d_n9 * ddt_scale), (eq41_e1246_d_n10 * ddt_scale), (eq41_e1246_d_n11 * ddt_scale), (eq41_e1246_d_n12 * ddt_scale), (eq41_e1246_d_n13 * ddt_scale), (eq41_e1246_d_n14 * ddt_scale), (eq41_e1246_d_n15 * ddt_scale), (eq41_e1246_d_n16 * ddt_scale), (eq41_e1246_d_n17 * ddt_scale), (eq41_e1246_d_n18 * ddt_scale)];let eq41_branch_derivatives: [f64; 13] = [(eq41_e1246_d_b0 * ddt_scale), (eq41_e1246_d_b1 * ddt_scale), (eq41_e1246_d_b2 * ddt_scale), (eq41_e1246_d_b3 * ddt_scale), (eq41_e1246_d_b4 * ddt_scale), (eq41_e1246_d_b5 * ddt_scale), (eq41_e1246_d_b6 * ddt_scale), (eq41_e1246_d_b7 * ddt_scale), (eq41_e1246_d_b8 * ddt_scale), (eq41_e1246_d_b9 * ddt_scale), (eq41_e1246_d_b10 * ddt_scale), (eq41_e1246_d_b11 * ddt_scale), (eq41_e1246_d_b12 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq41_value),
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
            multiplicity,
        );let eq42_e1250: f64 = ((nv15 - 0.0) * s.v[955]);let eq42_e1250_d_n0: f64 = ((nv15 - 0.0) * s.dn[955][0]);let eq42_e1250_d_n1: f64 = ((nv15 - 0.0) * s.dn[955][1]);let eq42_e1250_d_n2: f64 = ((nv15 - 0.0) * s.dn[955][2]);let eq42_e1250_d_n3: f64 = ((nv15 - 0.0) * s.dn[955][3]);let eq42_e1250_d_n4: f64 = ((nv15 - 0.0) * s.dn[955][4]);let eq42_e1250_d_n5: f64 = ((nv15 - 0.0) * s.dn[955][5]);let eq42_e1250_d_n6: f64 = ((nv15 - 0.0) * s.dn[955][6]);let eq42_e1250_d_n7: f64 = ((nv15 - 0.0) * s.dn[955][7]);let eq42_e1250_d_n8: f64 = ((nv15 - 0.0) * s.dn[955][8]);let eq42_e1250_d_n9: f64 = ((nv15 - 0.0) * s.dn[955][9]);let eq42_e1250_d_n10: f64 = ((nv15 - 0.0) * s.dn[955][10]);let eq42_e1250_d_n11: f64 = ((nv15 - 0.0) * s.dn[955][11]);let eq42_e1250_d_n12: f64 = ((nv15 - 0.0) * s.dn[955][12]);let eq42_e1250_d_n13: f64 = ((nv15 - 0.0) * s.dn[955][13]);let eq42_e1250_d_n14: f64 = ((nv15 - 0.0) * s.dn[955][14]);let eq42_e1250_d_n15: f64 = (s.v[955] + ((nv15 - 0.0) * s.dn[955][15]));let eq42_e1250_d_n16: f64 = ((nv15 - 0.0) * s.dn[955][16]);let eq42_e1250_d_n17: f64 = ((nv15 - 0.0) * s.dn[955][17]);let eq42_e1250_d_n18: f64 = ((nv15 - 0.0) * s.dn[955][18]);let eq42_e1250_d_b0: f64 = ((nv15 - 0.0) * s.db[955][0]);let eq42_e1250_d_b1: f64 = ((nv15 - 0.0) * s.db[955][1]);let eq42_e1250_d_b2: f64 = ((nv15 - 0.0) * s.db[955][2]);let eq42_e1250_d_b3: f64 = ((nv15 - 0.0) * s.db[955][3]);let eq42_e1250_d_b4: f64 = ((nv15 - 0.0) * s.db[955][4]);let eq42_e1250_d_b5: f64 = ((nv15 - 0.0) * s.db[955][5]);let eq42_e1250_d_b6: f64 = ((nv15 - 0.0) * s.db[955][6]);let eq42_e1250_d_b7: f64 = ((nv15 - 0.0) * s.db[955][7]);let eq42_e1250_d_b8: f64 = ((nv15 - 0.0) * s.db[955][8]);let eq42_e1250_d_b9: f64 = ((nv15 - 0.0) * s.db[955][9]);let eq42_e1250_d_b10: f64 = ((nv15 - 0.0) * s.db[955][10]);let eq42_e1250_d_b11: f64 = ((nv15 - 0.0) * s.db[955][11]);let eq42_e1250_d_b12: f64 = ((nv15 - 0.0) * s.db[955][12]);let eq42_e1251: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, eq42_e1250);let eq42_value: f64 = eq42_e1251;let eq42_node_derivatives: [f64; 19] = [(eq42_e1250_d_n0 * ddt_scale), (eq42_e1250_d_n1 * ddt_scale), (eq42_e1250_d_n2 * ddt_scale), (eq42_e1250_d_n3 * ddt_scale), (eq42_e1250_d_n4 * ddt_scale), (eq42_e1250_d_n5 * ddt_scale), (eq42_e1250_d_n6 * ddt_scale), (eq42_e1250_d_n7 * ddt_scale), (eq42_e1250_d_n8 * ddt_scale), (eq42_e1250_d_n9 * ddt_scale), (eq42_e1250_d_n10 * ddt_scale), (eq42_e1250_d_n11 * ddt_scale), (eq42_e1250_d_n12 * ddt_scale), (eq42_e1250_d_n13 * ddt_scale), (eq42_e1250_d_n14 * ddt_scale), (eq42_e1250_d_n15 * ddt_scale), (eq42_e1250_d_n16 * ddt_scale), (eq42_e1250_d_n17 * ddt_scale), (eq42_e1250_d_n18 * ddt_scale)];let eq42_branch_derivatives: [f64; 13] = [(eq42_e1250_d_b0 * ddt_scale), (eq42_e1250_d_b1 * ddt_scale), (eq42_e1250_d_b2 * ddt_scale), (eq42_e1250_d_b3 * ddt_scale), (eq42_e1250_d_b4 * ddt_scale), (eq42_e1250_d_b5 * ddt_scale), (eq42_e1250_d_b6 * ddt_scale), (eq42_e1250_d_b7 * ddt_scale), (eq42_e1250_d_b8 * ddt_scale), (eq42_e1250_d_b9 * ddt_scale), (eq42_e1250_d_b10 * ddt_scale), (eq42_e1250_d_b11 * ddt_scale), (eq42_e1250_d_b12 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq42_value),
            &eq42_node_derivatives,
            &eq42_branch_derivatives,
            multiplicity,
        );
    }
}
